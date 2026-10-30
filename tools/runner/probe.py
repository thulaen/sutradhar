"""Sandbox refusal probe (trust boundaries, AT-TB01).

Run as a check inside the sandbox, it tries what candidate code must never
manage: reading protected files, writing outside its private scratch
area, connecting to host sockets, opening an internet socket and gaining
privileges. It prints one JSON line per attempt and exits 0 only when
every attempt was refused. Only operating-system refusals count: denied,
not permitted, read-only, unsupported address family, or not found (the
path is hidden; the qualification checks beforehand that it exists on
the host). "Already exists", "connection refused", time-outs and bugs in
the probe itself are never mistaken for refusals.

    python3 -m tools.runner.probe [--read FILE]... [--write FOLDER]...
        [--connect SOCKET]...
"""
import errno
import json
import os
import secrets
import socket
import subprocess
import sys

_REFUSED = {errno.EACCES, errno.EPERM, errno.EROFS, errno.EAFNOSUPPORT,
            errno.ENOENT}
_SECONDS = 10
USAGE = ("usage: [--read FILE]... [--write FOLDER]... "
         "[--connect SOCKET]...\n")


def read(path):
    """Opening the file for reading is the attempt."""
    os.close(os.open(path, os.O_RDONLY))


def write(folder):
    """Creating a new file in the folder is the attempt; it is removed."""
    target = os.path.join(folder, f"sutradhar-probe-{secrets.token_hex()}")
    os.close(os.open(target, os.O_WRONLY | os.O_CREAT | os.O_EXCL))
    os.unlink(target)


def connect(path):
    """Connecting to a Unix socket is the attempt."""
    with socket.socket(socket.AF_UNIX) as handle:
        handle.connect(path)


def network(family):
    """Opening an internet socket at all is the attempt.

    Reaching an unrouted address can fail even where networking works, so
    the sandbox's address-family restriction itself is what is tested.
    """
    socket.socket(family, socket.SOCK_STREAM).close()


def privileges(run=subprocess.run):
    done = run(["sudo", "-n", "true"], capture_output=True,
               timeout=_SECONDS, stdin=subprocess.DEVNULL)
    if done.returncode:
        raise PermissionError(errno.EPERM, "sudo refused")


def trials(reads, writes, connects):
    """(name, action) for each attempt the sandbox must refuse."""
    found = [(f"read {path}", lambda p=path: read(p)) for path in reads]
    found += [(f"write {path}", lambda p=path: write(p)) for path in writes]
    found += [(f"connect {path}", lambda p=path: connect(p))
              for path in connects]
    return found + [("network ipv4", lambda: network(socket.AF_INET)),
                    ("network ipv6", lambda: network(socket.AF_INET6)),
                    ("privileges", privileges)]


def _judged(error):
    code = getattr(error, "errno", None)
    name = errno.errorcode.get(code, type(error).__name__)
    return code in _REFUSED, name


def attempt(trial_list):
    """One report entry per trial: refused or not, and how."""
    report = []
    for name, action in trial_list:
        try:
            action()
        except (OSError, subprocess.SubprocessError) as error:
            refused, how = _judged(error)
            report.append({"attempt": name, "refused": refused,
                           "error": how})
        else:
            report.append({"attempt": name, "refused": False,
                           "error": None})
    return report


def _arguments(argv):
    """(reads, writes, connects), or None for a usage error."""
    found = {"--read": [], "--write": [], "--connect": []}
    if len(argv) % 2:
        return None
    for flag, value in zip(argv[::2], argv[1::2]):
        if flag not in found:
            return None
        found[flag].append(value)
    return found["--read"], found["--write"], found["--connect"]


def main(argv, out=sys.stdout, make=trials):
    """Entry point; exit 0 only when every attempt was refused."""
    parsed = _arguments(argv)
    if parsed is None:
        out.write(USAGE)
        return 2
    report = attempt(make(*parsed))
    for entry in report:
        out.write(json.dumps(entry, sort_keys=True) + "\n")
    refused = bool(report) and all(entry["refused"] for entry in report)
    return 0 if refused else 1


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
