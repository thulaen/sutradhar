"""The candidate sandbox: one systemd transient service per check.

Each check runs under a throwaway dynamic user with no network, no
devices, no capabilities, private /tmp and /var/tmp held in memory (so
counted against the memory limit), a read-only system, no home
directories, hidden host data folders, a file-size limit and memory,
process and time limits (trust boundaries). The candidate and each
installed tool are bound read-only under a private /opt, so nothing is
created on the host; the candidate sits alone in a scan folder so that
scanning the folder never reads settings the candidate supplies.
Environment expansion is off, so the command runs exactly as listed.
Values are checked before use: a space, colon or line break could
otherwise change a property's meaning.
"""
import posixpath
import re

SCAN = "/opt/scan"
CANDIDATE = f"{SCAN}/candidate"
TOOLS = "/opt/sutradhar"
# systemd-run exits 1 itself when it cannot start a unit, so inside the
# sandbox a shell reports the check's own status as 10 (0) or 11 (1) and
# anything else as 12. The shell also finds tools by their sandbox paths.
WRAPPER = '"$@"; case $? in 0) exit 10;; 1) exit 11;; esac; exit 12'
_STATUS = {10: 0, 11: 1}
FILE_LIMIT = 64 * 1024 * 1024  # largest file or output a check may write
# Host folders are replaced by empty private ones; only the user database
# socket stays, so the dynamic user's name can still be looked up.
_HIDDEN = ("/opt", "/run", "/var", "/srv", "/mnt", "/media")
_ISOLATION = (
    "DynamicUser=yes", "PrivateNetwork=yes", "PrivateDevices=yes",
    "PrivateTmp=disconnected",
    "PrivateIPC=yes", "PrivateUsers=yes", "ProtectSystem=strict",
    "ProtectHome=yes", "ProtectKernelTunables=yes",
    "ProtectKernelModules=yes", "ProtectKernelLogs=yes",
    "ProtectControlGroups=yes", "ProtectClock=yes", "ProtectHostname=yes",
    "ProtectProc=invisible", "ProcSubset=pid", "NoNewPrivileges=yes",
    "RestrictSUIDSGID=yes", "RestrictNamespaces=yes",
    "RestrictRealtime=yes", "LockPersonality=yes",
    "RestrictAddressFamilies=AF_UNIX", "CapabilityBoundingSet=",
    "SystemCallArchitectures=native", "SystemCallFilter=@system-service",
    "UMask=0077", "LimitCORE=0", f"LimitFSIZE={FILE_LIMIT}",
    "MemorySwapMax=0", *[f"TemporaryFileSystem={f}" for f in _HIDDEN],
    "BindReadOnlyPaths=-/run/systemd/userdb")
_PATH = re.compile(r"/[A-Za-z0-9._/+-]*")
_UNIT = re.compile(r"[a-z0-9-]{1,64}")
_NAME = re.compile(r"[A-Z_][A-Z0-9_]*")
_CONTROL = re.compile(r"[\x00-\x1f\x7f]")
_MEMORY = re.compile(r"[1-9][0-9]*[KMG]?")


def safe_path(value):
    """value, if it is a plain absolute path safe inside a property.

    A leading "//" is refused: it survives normalisation, and walking its
    parent folders would never reach "/".
    """
    unsafe = (not _PATH.fullmatch(value) or value.startswith("//")
              or posixpath.normpath(value) != value)
    if unsafe:
        raise ValueError(f"unsafe path: {value!r}")
    return value


def _count(value):
    if type(value) is not int or value < 1:
        raise ValueError(f"bad limit: {value!r}")
    return value


def _limits(limits):
    if not _MEMORY.fullmatch(limits["memory"]):
        raise ValueError(f"bad memory limit: {limits['memory']!r}")
    return [f"MemoryMax={limits['memory']}",
            f"TasksMax={_count(limits['tasks'])}",
            f"RuntimeMaxSec={_count(limits['seconds'])}"]


def _environment(env):
    for name, value in env.items():
        if not _NAME.fullmatch(name) or _CONTROL.search(value):
            raise ValueError(f"unsafe environment entry: {name!r}")
        yield f"--setenv={name}={value}"


def command(prefix, unit, limits, binds, cwd, env, argv):
    """The full command line that runs argv in a fresh sandbox."""
    if not _UNIT.fullmatch(unit):
        raise ValueError(f"unsafe unit name: {unit!r}")
    if not argv:
        raise ValueError("empty command")
    props = [*_ISOLATION, *_limits(limits),
             f"WorkingDirectory={safe_path(cwd)}"]
    props += [f"BindReadOnlyPaths={safe_path(src)}:{safe_path(dst)}"
              for src, dst in binds]
    return [*prefix, "systemd-run", "--quiet", "--wait", "--pipe",
            "--collect", "--service-type=exec", "--expand-environment=no",
            f"--unit={unit}", *[f"--property={p}" for p in props],
            *_environment(env), "--", "/bin/sh", "-c", WRAPPER,
            "sutradhar-check", *argv]


def check_status(code):
    """The check's own exit status (0 or 1), or None if it decided nothing."""
    return _STATUS.get(code)
