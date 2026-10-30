"""Pinned unit-test harness: runs a candidate's tests and reports counts.

Started inside the sandbox as `python3 -I -S -B harness.py ROOT`, with the
run's nonce as the only line of standard input, so it never appears on a
command line. The candidate folder is added after the standard library,
so candidate files cannot shadow standard modules. Tests are discovered
from each top-level package except `docs` (historical records are never
run); a test file that yields no test is reported as uncollected. Test
output goes to standard error, and standard output is set aside first, so
the JSON summary written to it is the runner's alone (tools.runner
reports). An early exit, zero tests, skips or uncollected files never
pass. Code under test runs in the same process and could still forge a
summary on purpose; that is a matter for review, not something this
harness can prevent. Standard library only.
"""
import json
import os
import sys
import unittest

FORMAT = "sutradhar-unittest/1"
EXCLUDED = ("docs",)
USAGE = "usage: harness.py ROOT < NONCE"


def _skipped(root, folder, name):
    return name.startswith(".") or (folder == root and name in EXCLUDED)


def test_files(root):
    """Every test*.py or *_test.py file under root, outside hidden and
    excluded folders; files that yield no test are reported, never run."""
    found = []
    for folder, dirs, files in os.walk(root):
        dirs[:] = sorted(name for name in dirs
                         if not _skipped(root, folder, name))
        found += [os.path.relpath(os.path.join(folder, name), root)
                  .replace(os.sep, "/") for name in files
                  if name.endswith(".py") and (name.startswith("test")
                                               or name.endswith("_test.py"))]
    return sorted(found)


def _modules(suite):
    for item in suite:
        if isinstance(item, unittest.TestSuite):
            yield from _modules(item)
        else:
            yield type(item).__module__


def _discover(loader, root):
    suite = unittest.TestSuite()
    for name in sorted(os.listdir(root)):
        package = os.path.join(root, name)
        wanted = (not _skipped(root, root, name)
                  and os.path.isfile(os.path.join(package, "__init__.py")))
        if wanted:
            suite.addTests(loader.discover(package, top_level_dir=root))
    return suite


def run(root, nonce, stream=None):
    """Discover and run root's tests; return the summary."""
    if root not in sys.path:
        sys.path.append(root)
    suite = _discover(unittest.TestLoader(), root)
    seen = {name.replace(".", "/") + ".py" for name in _modules(suite)}
    result = unittest.TextTestRunner(stream=stream).run(suite)
    return {"harness": FORMAT, "nonce": nonce,
            "collected": suite.countTestCases(), "run": result.testsRun,
            "failures": len(result.failures)
            + len(result.unexpectedSuccesses),
            "errors": len(result.errors),
            "skipped": len(result.skipped) + len(result.expectedFailures),
            "uncollected": [path for path in test_files(root)
                            if path not in seen]}


def clean(summary):
    """Whether every collected test ran with nothing failed or skipped."""
    return (summary["run"] == summary["collected"] > 0
            and not summary["uncollected"]
            and not any(summary[key]
                        for key in ("failures", "errors", "skipped")))


def isolate_stdout():
    """Set the real standard output aside; send fd 1 to standard error."""
    sys.stdout.flush()
    report = os.fdopen(os.dup(1), "w")
    os.dup2(2, 1)
    return report


def main(argv, stdin, isolate=isolate_stdout):
    """Entry point; exit 0 only for a complete, clean run."""
    if len(argv) != 1:
        print(USAGE)
        return 2
    nonce = stdin.readline().strip()
    report = isolate()
    summary = run(os.path.abspath(argv[0]), nonce)
    report.write(json.dumps(summary, sort_keys=True) + "\n")
    report.flush()
    return 0 if clean(summary) else 1


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:], sys.stdin))
