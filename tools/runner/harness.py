"""Pinned unit-test harness: runs a candidate's tests and reports counts.

Started inside the sandbox as `python3 -I -S -B harness.py [--detail]
ROOT`, with the run's nonce as the first line of standard input, so it
never appears on a command line; --detail adds each test's outcome and
keys for the test-first check, and reads the test modules to run from the
following lines (all when none). The candidate folder is added after the
standard library, so candidate files cannot shadow standard modules.
Test ids are built from each test's class and method. Tests are discovered
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
import types
import unittest

FORMAT = "sutradhar-unittest/1"
EXCLUDED = ("docs",)
USAGE = "usage: harness.py [--detail] ROOT < NONCE"


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


def _tests(suite):
    for item in suite:
        if isinstance(item, unittest.TestSuite):
            yield from _tests(item)
        else:
            yield item


def _name(test):
    """A test's id from its class and method, so an overridden id() or a
    stray attribute cannot change it; a subtest counts as its test."""
    if isinstance(test, unittest.case._SubTest):
        test = test.test_case
    if isinstance(test, unittest.TestCase):
        kind = type(test)
        return f"{kind.__module__}.{kind.__qualname__}.{test._testMethodName}"
    return str(test.id())  # a class or module set-up failure


def _names(pairs):
    return sorted({_name(test) for test, _ in pairs})


def _keys(test):
    """The code a test comes from: its module, its own and inherited
    classes and the function defining it (standard library left out)."""
    kind = type(test)
    found = {kind.__module__} | {f"{c.__module__}.{c.__qualname__}"
                                 for c in kind.__mro__}
    function = getattr(kind, test._testMethodName, None)
    if isinstance(function, types.FunctionType):
        found.add(f"{function.__module__}.{function.__qualname__}")
    return sorted(key for key in found
                  if not key.startswith(("unittest.", "builtins.")))


def _module(test):
    if isinstance(test, unittest.loader._FailedTest):
        return test._testMethodName  # the module that failed to load
    return type(test).__module__


class _Result(unittest.TextTestResult):
    """Remembers each test as it starts: when a class or module set-up
    fails, its collected tests never start."""

    started = ()

    def startTest(self, test):
        self.started = (*self.started, test)
        super().startTest(test)


def _discover(loader, root):
    suite = unittest.TestSuite()
    for name in sorted(os.listdir(root)):
        package = os.path.join(root, name)
        wanted = (not _skipped(root, root, name)
                  and os.path.isfile(os.path.join(package, "__init__.py")))
        if wanted:
            suite.addTests(loader.discover(package, top_level_dir=root))
    return suite


def run(root, nonce, stream=None, detail=False, modules=()):
    """Discover and run root's tests; return the summary.

    modules, when given, limits the run to those test modules (a module
    that failed to load counts by its own name). With detail, the summary
    also lists the ids of tests that started, the ids that failed
    (assertions) and that raised errors, and each started test's keys.
    """
    if root not in sys.path:
        sys.path.append(root)
    suite = _discover(unittest.TestLoader(), root)
    if modules:
        wanted = set(modules)
        suite = unittest.TestSuite(test for test in _tests(suite)
                                   if _module(test) in wanted)
    found = {_module(test) for test in _tests(suite)}
    seen = {type(test).__module__.replace(".", "/") + ".py"
            for test in _tests(suite)}
    result = unittest.TextTestRunner(stream=stream, verbosity=2,
                                     resultclass=_Result).run(suite)
    summary = {"harness": FORMAT, "nonce": nonce,
               "collected": suite.countTestCases(), "run": result.testsRun,
               "failures": len(result.failures)
               + len(result.unexpectedSuccesses),
               "errors": len(result.errors),
               "skipped": len(result.skipped) + len(result.expectedFailures),
               "uncollected": sorted(set(modules) - found) if modules else [
                   path for path in test_files(root) if path not in seen]}
    if detail:
        summary.update(
            ids=sorted({_name(test) for test in result.started}),
            failed_ids=_names(result.failures),
            errored_ids=_names(result.errors),
            keys={_name(test): _keys(test) for test in result.started})
    return summary


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
    detail = argv[:1] == ["--detail"]
    roots = argv[1:] if detail else argv
    if len(roots) != 1 or roots[0].startswith("-"):
        print(USAGE)
        return 2
    nonce = stdin.readline().strip()
    modules = [line.strip() for line in stdin if line.strip()] if detail \
        else []
    report = isolate()
    summary = run(os.path.abspath(roots[0]), nonce, detail=detail,
                  modules=modules)
    report.write(json.dumps(summary, sort_keys=True) + "\n")
    report.flush()
    return 0 if clean(summary) else 1


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:], sys.stdin))
