"""The exact candidate: a Git tree exported blob by blob.

Content is read object by object rather than through `git archive`, so
attributes (export-ignore, export-subst), replace references and hooks
cannot change what is tested, and each blob's identifier is recomputed
from its bytes. Symbolic links, submodules, unsafe paths, names that look
like options and exports over the size or file-count limits are refused.
Git runs only on a repository the runner owns, with a time limit, never
fetching missing objects or using any transport.
The manifest (path, SHA-256, executable bit) and its digest bind evidence
to content (SPEC-001). An installed tool folder has a digest of the same
form, so pinned tools and exports compare directly.
"""
import hashlib
import operator
import os
import re
import subprocess

from tools.runner import bundle

GIT_SECONDS = 300
MAX_FILES = 100_000
MAX_EXPORT_BYTES = 512 * 1024 * 1024
_GIT = ("git", "--no-replace-objects", "-c", "core.fsmonitor=false",
        "-c", "core.hooksPath=/dev/null", "-c", "protocol.allow=never")
_ENV = {"PATH": "/usr/bin:/bin", "LC_ALL": "C", "GIT_CONFIG_NOSYSTEM": "1",
        "GIT_CONFIG_GLOBAL": "/dev/null", "GIT_NO_REPLACE_OBJECTS": "1",
        "GIT_NO_LAZY_FETCH": "1"}
_OID = re.compile(rb"[0-9a-f]{40}")
_MODES = {"100644": False, "100755": True}
_BAD_PARTS = ("", ".", "..", ".git")
_NUL = bytes(1)
_BY_PATH = operator.itemgetter("path")


class CandidateError(Exception):
    """The candidate cannot be identified or exported exactly."""


def git(repo, args, data=None, run=subprocess.run):
    """Standard output of one git command, or CandidateError."""
    if os.stat(repo).st_uid != os.getuid():
        raise CandidateError("repository not owned by the runner")
    try:
        done = run([*_GIT, "-C", repo, *args], input=data,
                   capture_output=True, env=_ENV, timeout=GIT_SECONDS)
    except subprocess.TimeoutExpired:
        raise CandidateError(f"git {args[0]} timed out") from None
    if done.returncode:
        raise CandidateError(f"git {args[0]} failed")
    return done.stdout


def resolve_tree(repo, rev):
    """The tree identifier that rev names (never read as an option)."""
    if rev.startswith("-"):
        raise CandidateError(f"not a tree: {rev}")
    tree = git(repo, ["rev-parse", "--verify", rev + "^{tree}"]).strip()
    if not _OID.fullmatch(tree):
        raise CandidateError(f"not a tree: {rev}")
    return tree.decode()


def _safe_path(raw):
    try:
        path = raw.decode()
    except UnicodeDecodeError:
        raise CandidateError("path is not UTF-8") from None
    if path.startswith("-") or any(part in _BAD_PARTS
                                   for part in path.split("/")):
        raise CandidateError(f"unsafe path: {path}")
    return path


def _entries(repo, tree):
    if not _OID.fullmatch(tree.encode()):
        raise CandidateError(f"not a tree: {tree}")
    found = []
    for record in git(repo, ["ls-tree", "-r", "-z", "--long", "--full-tree",
                             tree]).split(_NUL):
        if record:
            meta, raw = record.split(b"\t", 1)
            mode, kind, oid, size = meta.decode().split()
            path = _safe_path(raw)
            if mode not in _MODES or kind != "blob":
                raise CandidateError(f"unsupported entry: {path}")
            if not size.isdigit():  # Git prints "BAD" for a missing object
                raise CandidateError(f"object mismatch: {path}")
            found.append((path, oid, _MODES[mode], int(size)))
    return found


def executables(repo, tree):
    """The paths in the tree whose executable bit is set."""
    return {path for path, _, executable, _ in _entries(repo, tree)
            if executable}


def _blob_id(body):
    header = b"blob " + str(len(body)).encode() + _NUL
    return hashlib.sha1(header + body).hexdigest()


def _blobs(repo, found):
    """(path, executable, bytes) per entry, checked against its id."""
    request = "".join(f"{entry[1]}\n" for entry in found).encode()
    data, pos = git(repo, ["cat-file", "--batch"], request), 0
    for path, oid, executable, _ in found:
        end = data.index(b"\n", pos)
        header = data[pos:end].split()
        if header[:2] != [oid.encode(), b"blob"]:
            raise CandidateError(f"object mismatch: {path}")
        size = int(header[2])
        body, pos = data[end + 1:end + 1 + size], end + 2 + size
        if _blob_id(body) != oid:
            raise CandidateError(f"object mismatch: {path}")
        yield path, executable, body


def _write(dest, path, body, executable):
    *folders, name = path.split("/")
    folder = dest
    for part in folders:
        folder = os.path.join(folder, part)
        if not os.path.isdir(folder):
            os.mkdir(folder)
            os.chmod(folder, 0o755)
    target = os.path.join(folder, name)
    # O_EXCL refuses any existing entry, a symbolic link included.
    flags = os.O_WRONLY | os.O_CREAT | os.O_EXCL
    with os.fdopen(os.open(target, flags), "wb") as handle:
        handle.write(body)
    os.chmod(target, 0o755 if executable else 0o644)


def export(repo, tree, dest):
    """Write the tree's files under dest, which must not exist yet."""
    found = _entries(repo, tree)
    if len(found) > MAX_FILES:
        raise CandidateError(f"candidate has too many files: {len(found)}")
    total = sum(entry[3] for entry in found)
    if total > MAX_EXPORT_BYTES:
        raise CandidateError(f"candidate too large: {total} bytes")
    try:
        os.mkdir(dest)
    except OSError as error:
        raise CandidateError(f"cannot create the export: {error}") from None
    os.chmod(dest, 0o755)
    manifest = []
    for path, executable, body in _blobs(repo, found):
        _write(dest, path, body, executable)
        manifest.append({"path": path, "executable": executable,
                         "sha256": hashlib.sha256(body).hexdigest()})
    return sorted(manifest, key=_BY_PATH)


def changed(repo, base, tree):
    """Sorted paths that differ from base to tree (all when no base)."""
    if base is None:
        args = ["ls-tree", "-r", "-z", "--name-only", "--full-tree", tree]
    else:
        args = ["diff-tree", "-r", "-z", "--no-renames", "--name-only",
                base, tree]
    return sorted(_safe_path(p) for p in git(repo, args).split(_NUL) if p)


def digest(manifest):
    """SHA-256 of the canonical manifest."""
    return hashlib.sha256(bundle.canonical(manifest)).hexdigest()


def file_sha256(path):
    """SHA-256 of a file's bytes, read in blocks."""
    with open(path, "rb") as handle:
        return hashlib.file_digest(handle, hashlib.sha256).hexdigest()


def _walk(root):
    manifest = []
    for folder, dirs, files in os.walk(root):
        for name in dirs + files:
            if os.path.islink(os.path.join(folder, name)):
                raise CandidateError(f"symbolic link in {root}")
        for name in files:
            full = os.path.join(folder, name)
            manifest.append({
                "path": os.path.relpath(full, root).replace(os.sep, "/"),
                "executable": bool(os.stat(full).st_mode & 0o100),
                "sha256": file_sha256(full)})
    return sorted(manifest, key=_BY_PATH)


def path_digest(path):
    """SHA-256 of an installed file, or the manifest digest of a folder."""
    if os.path.islink(path) or not os.path.exists(path):
        raise CandidateError(f"missing or linked: {path}")
    if os.path.isfile(path):
        return file_sha256(path)
    return digest(_walk(path))
