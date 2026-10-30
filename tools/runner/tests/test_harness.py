"""The pinned unit-test harness (SPEC-001: no partial or skipped pass)."""
import contextlib
import io
import json
import os
import runpy
import sys
import tempfile
import textwrap
import unittest
from unittest import mock

from tools.runner import harness

PASSING = """
import unittest
class T(unittest.TestCase):
    def test_a(self):
        self.assertTrue(True)
    def test_b(self):
        self.assertTrue(True)
"""


class HarnessCase(unittest.TestCase):
    counter = 0

    def setUp(self):
        tmp = tempfile.TemporaryDirectory()
        self.addCleanup(tmp.cleanup)
        self.root = tmp.name
        HarnessCase.counter += 1
        self.package = f"harnesspkg{HarnessCase.counter}"
        self.write(f"{self.package}/__init__.py", "")
        self.addCleanup(self.forget)

    def forget(self):
        for name in [n for n in sys.modules if n.startswith(self.package)]:
            del sys.modules[name]
        if self.root in sys.path:
            sys.path.remove(self.root)

    def write(self, rel, text):
        path = os.path.join(self.root, rel)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w", encoding="utf-8") as handle:
            handle.write(textwrap.dedent(text))

    def summary(self, nonce="n1"):
        self.stream = io.StringIO()
        return harness.run(self.root, nonce, self.stream)


class RunTest(HarnessCase):
    def test_passing_tests_are_counted(self):
        self.write(f"{self.package}/test_a.py", PASSING)
        self.assertEqual(self.summary(), {
            "harness": "sutradhar-unittest/1", "nonce": "n1",
            "collected": 2, "run": 2, "failures": 0, "errors": 0,
            "skipped": 0, "uncollected": []})
        self.assertIn("Ran 2 tests", self.stream.getvalue())

    def test_failures_errors_and_skips_are_counted(self):
        self.write(f"{self.package}/test_a.py", """
            import unittest
            class T(unittest.TestCase):
                def test_fail(self):
                    self.assertEqual(1, 2)
                def test_error(self):
                    raise RuntimeError("x")
                @unittest.skip("later")
                def test_skip(self):
                    pass
                @unittest.expectedFailure
                def test_expected(self):
                    self.assertEqual(1, 2)
                @unittest.expectedFailure
                def test_unexpected(self):
                    pass
            """)
        got = self.summary()
        self.assertEqual((got["collected"], got["run"], got["failures"],
                          got["errors"], got["skipped"]), (5, 5, 2, 1, 2))

    def test_files_outside_packages_and_broken_modules_are_reported(self):
        self.write("loose/test_b.py", PASSING)
        self.write(f"{self.package}/test_c.py", "import missing_module\n")
        got = self.summary()
        self.assertEqual(got["uncollected"], [f"{self.package}/test_c.py",
                                              "loose/test_b.py"])
        self.assertGreaterEqual(got["errors"], 1)

    def test_other_test_file_names_are_reported(self):
        # Review 3 finding 6: tests that would never run are flagged.
        self.write(f"{self.package}/test_a.py", PASSING)
        self.write(f"{self.package}/feature_test.py", PASSING)
        self.assertEqual(self.summary()["uncollected"],
                         [f"{self.package}/feature_test.py"])

    def test_hidden_folders_are_not_searched(self):
        self.write(".git/test_x.py", "raise SystemExit(9)\n")
        self.write(f"{self.package}/test_a.py", PASSING)
        self.assertEqual(self.summary()["uncollected"], [])

    def main(self, nonce):
        report = io.StringIO()
        with contextlib.redirect_stderr(io.StringIO()):
            code = harness.main([self.root], io.StringIO(nonce + "\n"),
                                lambda: report)
        return code, report.getvalue()

    def test_main_takes_the_nonce_from_input_and_reports_privately(self):
        # Review 2 finding C: the nonce is never on the command line, and
        # the summary goes to the runner's own copy of standard output.
        self.write(f"{self.package}/test_a.py", PASSING)
        code, text = self.main("n2")
        summary = json.loads(text)
        self.assertEqual((code, summary["nonce"]), (0, "n2"))
        self.assertEqual(text, json.dumps(summary, sort_keys=True) + "\n")
        self.write(f"{self.package}/test_b.py", "import missing_module\n")
        self.assertEqual(self.main("n3")[0], 1)

    def test_clean_needs_every_test_run_and_nothing_else(self):
        good = {"collected": 1, "run": 1, "failures": 0, "errors": 0,
                "skipped": 0, "uncollected": []}
        self.assertTrue(harness.clean(good))
        for change in ({"run": 0}, {"collected": 2}, {"collected": 0, "run": 0},
                       {"failures": 1}, {"errors": 1}, {"skipped": 1},
                       {"uncollected": ["a/test_x.py"]}):
            self.assertFalse(harness.clean(dict(good, **change)), change)

    def test_docs_are_never_run(self):
        # Historical source records are data; SPEC-001 forbids running them.
        self.write("docs/__init__.py", "")
        self.write("docs/old/__init__.py", "")
        self.write("docs/old/test_quote.py", "raise SystemExit(5)\n")
        self.write(f"{self.package}/test_a.py", PASSING)
        got = self.summary()
        self.assertEqual((got["run"], got["errors"], got["uncollected"]),
                         (2, 0, []))

    def test_isolating_standard_output(self):
        saved = os.dup(1)
        try:
            report = harness.isolate_stdout()
            self.assertEqual(os.fstat(1).st_ino, os.fstat(2).st_ino)
            self.assertNotEqual(report.fileno(), 1)
            self.assertEqual(os.fstat(report.fileno()).st_ino,
                             os.fstat(saved).st_ino)
            report.write("")
            report.flush()
            report.close()
        finally:
            os.dup2(saved, 1)
            os.close(saved)

    def test_usage(self):
        out = io.StringIO()
        with contextlib.redirect_stdout(out):
            for argv in (["a", "b"], ["--detail"], ["--other", "a"],
                         ["--detail", "a", "b"], ["--other"]):
                self.assertEqual(harness.main(argv, io.StringIO(""),
                                              lambda: out), 2, argv)
        self.assertEqual(out.getvalue(),
                         "usage: harness.py [--detail] ROOT < NONCE\n" * 5)
        with mock.patch.object(sys, "argv", ["harness"]), \
                contextlib.redirect_stdout(io.StringIO()), \
                self.assertRaises(SystemExit) as stop:
            runpy.run_module("tools.runner.harness", run_name="__main__")
        self.assertEqual(stop.exception.code, 2)


class DetailTest(HarnessCase):
    """The test-first check needs each test's own outcome (AT-T01)."""

    def test_detail_lists_each_test_by_outcome(self):
        self.write(f"{self.package}/test_a.py", """
            import unittest
            class T(unittest.TestCase):
                def test_pass(self):
                    pass
                def test_fail(self):
                    self.assertEqual(1, 2)
                def test_error(self):
                    raise RuntimeError("x")
                def test_sub(self):
                    for i in (1, 2, 3):
                        with self.subTest(i=i):
                            self.assertEqual(i, 1)
            """)
        self.write(f"{self.package}/test_b.py", "import missing_module\n")
        got = harness.run(self.root, "n", io.StringIO(), detail=True)
        name = f"{self.package}.test_a.T.test_"
        broken = f"unittest.loader._FailedTest.{self.package}.test_b"
        self.assertEqual(got["ids"], [name + "error", name + "fail",
                                      name + "pass", name + "sub", broken])
        self.assertEqual(got["failed_ids"], [name + "fail", name + "sub"])
        self.assertEqual(got["errored_ids"], [name + "error", broken])
        self.assertEqual(got["keys"][name + "pass"], [
            f"{self.package}.test_a", f"{self.package}.test_a.T",
            name + "pass"])
        self.assertEqual(got["keys"][broken], [])

    def test_names_and_keys_come_from_the_defining_code(self):
        # Review: ids cannot be spoofed by id() or a "test_case" method,
        # and a test inherited from a mixin is keyed by the mixin's code.
        self.write(f"{self.package}/test_m.py", """
            import unittest
            class Cases:
                def test_x(self):
                    self.assertTrue(True)
            class T(Cases, unittest.TestCase):
                pass
            class U(T):
                def id(self):
                    return "spoofed"
                def test_case(self):
                    pass
            """)
        got = harness.run(self.root, "n", io.StringIO(), detail=True)
        m = f"{self.package}.test_m"
        self.assertEqual(got["ids"], [f"{m}.T.test_x", f"{m}.U.test_case",
                                      f"{m}.U.test_x"])
        self.assertEqual(got["keys"][f"{m}.U.test_x"], [
            m, f"{m}.Cases", f"{m}.Cases.test_x", f"{m}.T", f"{m}.U"])

    def test_tests_that_never_started_are_not_listed(self):
        self.write(f"{self.package}/test_c.py", """
            import unittest
            class Broken(unittest.TestCase):
                @classmethod
                def setUpClass(cls):
                    raise RuntimeError("x")
                def test_never(self):
                    pass
            """)
        got = harness.run(self.root, "n", io.StringIO(), detail=True)
        self.assertEqual(got["ids"], [])
        self.assertEqual(got["errored_ids"],
                         [f"setUpClass ({self.package}.test_c.Broken)"])

    def test_only_the_named_modules_run(self):
        self.write(f"{self.package}/test_a.py", PASSING)
        self.write(f"{self.package}/test_b.py", PASSING)
        self.write(f"{self.package}/test_c.py", "import missing_module\n")
        chosen = [f"{self.package}.test_b", f"{self.package}.test_c"]
        got = harness.run(self.root, "n", io.StringIO(), detail=True,
                          modules=chosen)
        broken = f"unittest.loader._FailedTest.{self.package}.test_c"
        self.assertEqual((got["collected"], got["uncollected"], got["ids"]),
                         (3, [], [f"{self.package}.test_b.T.test_a",
                                  f"{self.package}.test_b.T.test_b",
                                  broken]))

    def test_main_reports_detail_on_request(self):
        self.write(f"{self.package}/test_a.py", PASSING)
        self.write(f"{self.package}/test_b.py", PASSING)
        report = io.StringIO()
        with contextlib.redirect_stderr(io.StringIO()):
            code = harness.main(["--detail", self.root],
                                io.StringIO(f"n\n{self.package}.test_b\n\n"),
                                lambda: report)
        summary = json.loads(report.getvalue())
        self.assertEqual((code, summary["failed_ids"], len(summary["ids"])),
                         (0, [], 2))


class IsolationTest(HarnessCase):
    def test_candidate_files_cannot_shadow_the_standard_library(self):
        self.write("colorsys.py", "raise SystemExit(3)\n")
        self.write(f"{self.package}/test_a.py", """
            import colorsys
            import unittest
            class T(unittest.TestCase):
                def test_standard_module(self):
                    self.assertTrue(hasattr(colorsys, "rgb_to_hsv"))
            """)
        sys.modules.pop("colorsys", None)
        got = self.summary()
        self.assertEqual((got["run"], got["failures"], got["errors"]),
                         (1, 0, 0))


if __name__ == "__main__":
    unittest.main()
