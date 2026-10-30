"""Exact candidate export and manifest (SPEC-001 stale evidence)."""
import hashlib
import os
import stat
import subprocess
import tempfile
import unittest
import zlib

from tools.runner import candidate

GIT_ENV = {"PATH": os.environ.get("PATH", "/usr/bin:/bin"),
           "GIT_CONFIG_NOSYSTEM": "1", "GIT_CONFIG_GLOBAL": os.devnull,
           "GIT_AUTHOR_NAME": "t", "GIT_AUTHOR_EMAIL": "t@example.invalid",
           "GIT_COMMITTER_NAME": "t",
           "GIT_COMMITTER_EMAIL": "t@example.invalid"}


class Repo:
    def __init__(self, folder, name="repo", *options):
        self.path = os.path.join(folder, name)
        self.git("init", "-q", "-b", "main", *options, self.path, cwd=folder)

    def git(self, *args, cwd=None, data=None):
        done = subprocess.run(["git", *args], cwd=cwd or self.path,
                              env=GIT_ENV, input=data, check=True,
                              capture_output=True)
        return done.stdout.decode().strip()

    def write(self, rel, data, executable=False):
        target = os.path.join(self.path, rel)
        os.makedirs(os.path.dirname(target), exist_ok=True)
        with open(target, "wb") as handle:
            handle.write(data)
        self.git("add", "--", rel)
        if executable:
            self.git("update-index", "--chmod=+x", rel)

    def commit(self):
        self.git("commit", "-q", "-m", "c")
        return self.git("rev-parse", "HEAD^{tree}")


class CandidateCase(unittest.TestCase):
    def setUp(self):
        tmp = tempfile.TemporaryDirectory()
        self.addCleanup(tmp.cleanup)
        self.folder = tmp.name
        self.repo = Repo(self.folder)

    def dest(self, name="out"):
        return os.path.join(self.folder, name)

    def export(self, tree, name="out"):
        return candidate.export(self.repo.path, tree, self.dest(name))


class ExportTest(CandidateCase):
    def test_exact_files_modes_and_manifest(self):
        self.repo.write("a.md", b"x\n")
        self.repo.write("tools/t.py", b"print()\n", executable=True)
        manifest = self.export(self.repo.commit())
        self.assertEqual(manifest, [
            {"path": "a.md", "executable": False,
             "sha256": hashlib.sha256(b"x\n").hexdigest()},
            {"path": "tools/t.py", "executable": True,
             "sha256": hashlib.sha256(b"print()\n").hexdigest()}])
        with open(os.path.join(self.dest(), "a.md"), "rb") as handle:
            self.assertEqual(handle.read(), b"x\n")
        modes = {rel: stat.S_IMODE(os.stat(os.path.join(self.dest(), rel))
                                   .st_mode)
                 for rel in ("", "a.md", "tools", "tools/t.py")}
        self.assertEqual(modes, {"": 0o755, "a.md": 0o644,
                                 "tools": 0o755, "tools/t.py": 0o755})

    def test_attributes_cannot_hide_or_rewrite_content(self):
        self.repo.write(".gitattributes",
                        b"secret.txt export-ignore\na.txt export-subst\n")
        self.repo.write("secret.txt", b"s\n")
        self.repo.write("a.txt", b"$Format:%H$\n")
        manifest = self.export(self.repo.commit())
        self.assertEqual([m["path"] for m in manifest],
                         [".gitattributes", "a.txt", "secret.txt"])
        with open(os.path.join(self.dest(), "a.txt"), "rb") as handle:
            self.assertEqual(handle.read(), b"$Format:%H$\n")

    def test_replace_references_are_ignored(self):
        self.repo.write("a.txt", b"real\n")
        tree = self.repo.commit()
        real = self.repo.git("rev-parse", "HEAD:a.txt")
        fake = self.repo.git("hash-object", "-w", "--stdin", data=b"fake\n")
        self.repo.git("replace", real, fake)
        self.export(tree)
        with open(os.path.join(self.dest(), "a.txt"), "rb") as handle:
            self.assertEqual(handle.read(), b"real\n")

    def test_symbolic_links_are_refused(self):
        self.repo.write("a.txt", b"x\n")
        os.symlink("a.txt", os.path.join(self.repo.path, "l"))
        self.repo.git("add", "l")
        with self.assertRaisesRegex(candidate.CandidateError,
                                    "unsupported entry: l"):
            self.export(self.repo.commit())

    def test_existing_destination_is_refused(self):
        self.repo.write("a.txt", b"x\n")
        tree = self.repo.commit()
        os.mkdir(self.dest())
        with self.assertRaisesRegex(candidate.CandidateError,
                                    "^cannot create the export: "):
            self.export(tree)

    def test_unknown_tree_is_refused(self):
        with self.assertRaisesRegex(candidate.CandidateError,
                                    "^git ls-tree failed$"):
            self.export("0" * 40)


class IdentityTest(CandidateCase):
    def test_resolve_tree(self):
        self.repo.write("a.txt", b"x\n")
        tree = self.repo.commit()
        self.assertEqual(candidate.resolve_tree(self.repo.path, "HEAD"), tree)
        self.assertEqual(candidate.resolve_tree(self.repo.path, tree), tree)
        for bad, message in (("nope", "^git rev-parse failed$"),
                             ("--output=x", "^not a tree: --output=x$"),
                             ("HEAD~5", "^git rev-parse failed$")):
            with self.assertRaisesRegex(candidate.CandidateError, message):
                candidate.resolve_tree(self.repo.path, bad)

    def test_changed_paths(self):
        self.repo.write("a.txt", b"1\n")
        self.repo.write("b.txt", b"1\n")
        base = self.repo.commit()
        self.repo.write("a.txt", b"2\n")
        self.repo.git("rm", "-q", "b.txt")
        self.repo.write("d/c.py", b"\n")
        tree = self.repo.commit()
        changed = candidate.changed(self.repo.path, base, tree)
        self.assertEqual(changed, ["a.txt", "b.txt", "d/c.py"])
        self.assertEqual(candidate.changed(self.repo.path, None, tree),
                         ["a.txt", "d/c.py"])

    def test_digest_tracks_content_and_mode(self):
        self.repo.write("a.txt", b"1\n")
        first = candidate.digest(self.export(self.repo.commit(), "o1"))
        self.repo.write("a.txt", b"2\n")
        second = candidate.digest(self.export(self.repo.commit(), "o2"))
        self.repo.git("update-index", "--chmod=+x", "a.txt")
        third = candidate.digest(self.export(self.repo.commit(), "o3"))
        self.assertEqual(len({first, second, third}), 3)
        self.assertRegex(first, "^[0-9a-f]{64}$")

    def test_path_digest_matches_export_and_files(self):
        self.repo.write("a.txt", b"1\n")
        self.repo.write("d/e.sh", b"\n", executable=True)
        manifest = self.export(self.repo.commit())
        self.assertEqual(candidate.path_digest(self.dest()),
                         candidate.digest(manifest))
        self.assertEqual(
            candidate.path_digest(os.path.join(self.dest(), "a.txt")),
            hashlib.sha256(b"1\n").hexdigest())

    def test_path_digest_refuses_links_and_missing(self):
        os.mkdir(self.dest())
        os.symlink("/etc", os.path.join(self.dest(), "l"))
        with self.assertRaisesRegex(candidate.CandidateError,
                                    "^symbolic link in "):
            candidate.path_digest(self.dest())
        with self.assertRaisesRegex(candidate.CandidateError,
                                    "^missing or linked: "):
            candidate.path_digest(os.path.join(self.dest(), "missing"))


class HostileTreeTest(CandidateCase):
    """Retrofit (coverage closure): trees Git itself would never create."""

    def mktree(self, line, *options):
        return self.repo.git("mktree", *options, data=line.encode())

    def test_git_folder_entry_is_refused(self):
        blob = self.repo.git("hash-object", "-w", "--stdin", data=b"x\n")
        tree = self.mktree(f"100644 blob {blob}\t.git\n")
        with self.assertRaisesRegex(candidate.CandidateError,
                                    r"^unsafe path: \.git$"):
            self.export(tree)

    def test_missing_object_is_refused(self):
        tree = self.mktree(f"100644 blob {'1' * 40}\ta.txt\n", "--missing")
        with self.assertRaisesRegex(candidate.CandidateError,
                                    "^object mismatch: a.txt$"):
            self.export(tree)
        # An object that vanishes after listing is still caught.
        with self.assertRaisesRegex(candidate.CandidateError,
                                    "^object mismatch: a.txt$"):
            list(candidate._blobs(self.repo.path,
                                  [("a.txt", "1" * 40, False, 3)]))

    def test_corrupt_object_is_refused(self):
        header = b"blob 5" + bytes(1)
        good = hashlib.sha1(header + b"good\n").hexdigest()
        folder = os.path.join(self.repo.path, ".git", "objects", good[:2])
        os.makedirs(folder, exist_ok=True)
        with open(os.path.join(folder, good[2:]), "wb") as handle:
            handle.write(zlib.compress(header + b"evil\n"))
        tree = self.mktree(f"100644 blob {good}\ta.txt\n")
        with self.assertRaisesRegex(candidate.CandidateError,
                                    "^object mismatch: a.txt$"):
            self.export(tree)
        self.assertFalse(os.path.exists(os.path.join(self.dest(), "a.txt")))

    def test_non_utf8_path_is_refused(self):
        name = os.fsdecode(bytes([0xFF]) + b".txt")
        self.repo.write(name, b"x\n")
        tree = self.repo.commit()
        with self.assertRaisesRegex(candidate.CandidateError,
                                    "^path is not UTF-8$"):
            self.export(tree)

    def test_names_that_are_not_tree_identifiers(self):
        self.repo.write("a.txt", b"x\n")
        self.repo.commit()
        with self.assertRaisesRegex(candidate.CandidateError,
                                    "not a tree: HEAD"):
            self.export("HEAD")
        other = Repo(self.folder, "sha256", "--object-format=sha256")
        other.write("a.txt", b"x\n")
        other.commit()
        with self.assertRaisesRegex(candidate.CandidateError, "not a tree"):
            candidate.resolve_tree(other.path, "HEAD")

    def test_files_share_folders(self):
        self.repo.write("d/a.txt", b"1\n")
        self.repo.write("d/b.txt", b"2\n")
        manifest = self.export(self.repo.commit())
        self.assertEqual([m["path"] for m in manifest],
                         ["d/a.txt", "d/b.txt"])


if __name__ == "__main__":
    unittest.main()
