"""Command line: python -m tools.limits.cli ROOT [--classes FILE] PATH...

Checks the listed repository paths against the source limits and prints
one JSON report. Exit codes: 0 PASSED, 1 FAILED, 2 BLOCKED or usage
error. Missing inputs, unsafe paths, unsupported languages, parser
resource limits and internal errors block; they never pass. The report
records the SHA-256 of the class list used, so a trusted runner can
require the protected copy from the base revision via --classes.
"""
import hashlib
import json
import os
import sys
import traceback

from tools.limits import counting, functions, policy

RULE = "SPEC-007 source limits, 2026-10-30"
CLASSES_FILE = "tools/limits/classes.txt"
_BLOCKING = ("unsupported_language", "parse_limit")
# UTF-8 needs at most four bytes per character.
_MAX_FILE_BYTES = 4 * counting.MAX_FILE_CHARS


def _unsafe(root, rel):
    """Why rel may not be read under root, or None when it is safe."""
    parts = rel.split("/")
    # An absolute path starts with an empty segment, so it is refused too.
    if "\\" in rel or any(part in ("", ".", "..") for part in parts):
        return "unsafe path"
    real_root = os.path.realpath(root)
    current = real_root
    for part in parts:
        current = os.path.join(current, part)
        if os.path.islink(current):
            return "symbolic link"
    if not os.path.isfile(current):
        return "missing"
    return None


def _read(path):
    with open(path, "rb") as handle:
        return handle.read()


def _check_one(root, rel, classes, report):
    cls = policy.classify(rel, classes)
    if cls is None:
        return [counting.Violation(rel, None, "unclassified", None)]
    if not policy.measured(cls):
        report["inventory"].setdefault(cls, []).append(rel)
        return []
    report["checked"] += 1
    path = os.path.join(root, rel)
    size = os.stat(path).st_size
    if size > _MAX_FILE_BYTES:  # refused before reading anything
        return [counting.Violation(rel, None, "file_size", size)]
    try:
        text = counting.decode(_read(path))
    except ValueError as error:
        return [counting.Violation(rel, None, "not_utf8", str(error))]
    found = policy.check_file(rel, text)
    if not any(v.kind == "file_size" for v in found):
        found += functions.check_source(rel, text)
    return counting.cap(rel, found)


def _load_classes(root, classes_path, report):
    path = classes_path or os.path.join(root, CLASSES_FILE)
    try:
        data = _read(path)
        report["classes_sha256"] = hashlib.sha256(data).hexdigest()
        return policy.load_classes(counting.decode(data))
    except (OSError, ValueError) as error:
        report["blocked"].append(f"{classes_path or CLASSES_FILE}: {error}")
        return None


def _status(report):
    if report["blocked"]:
        return "BLOCKED"
    return "FAILED" if report["violations"] else "PASSED"


def run(root, paths, classes_path=None):
    """Build the report dictionary for the listed paths."""
    report = {"rule": RULE, "checked": 0, "violations": [],
              "blocked": [], "inventory": {}, "classes_sha256": None}
    if not paths:
        report["blocked"].append("no paths given")
    classes = _load_classes(root, classes_path, report)
    for rel in paths if classes is not None else []:
        reason = _unsafe(root, rel)
        if reason:
            report["blocked"].append(f"{rel}: {reason}")
            continue
        for v in _check_one(root, rel, classes, report):
            if v.kind in _BLOCKING:
                report["blocked"].append(f"{rel}: {v.value}")
            report["violations"].append(v._asdict())
    report["status"] = _status(report)
    return report


def _parse(argv):
    root, rest = argv[0], argv[1:]
    if rest[:1] == ["--classes"] and len(rest) >= 2:
        return root, rest[2:], rest[1]
    return root, rest, None


def main(argv, out=sys.stdout):
    """Entry point; returns the process exit code."""
    if not argv:
        out.write(json.dumps({"status": "BLOCKED",
                              "blocked": ["usage: ROOT PATH..."]}))
        return 2
    try:
        report = run(*_parse(argv))
    except Exception as error:  # report, never crash into a pass
        traceback.print_exc()  # goes to standard error
        report = {"rule": RULE, "status": "BLOCKED",
                  "blocked": [f"internal error: {type(error).__name__}"]}
    out.write(json.dumps(report, indent=1, sort_keys=True))
    return {"PASSED": 0, "FAILED": 1}.get(report["status"], 2)


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
