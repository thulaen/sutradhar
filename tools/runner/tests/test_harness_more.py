"""Harness detail: second review pass and mutation closure.

Tests marked "Review 2" were written before their fix; those marked
"Retrofit" close mutation gaps after the code existed.
"""
import contextlib
import io
import json
import unittest

from tools.runner import harness
from tools.runner.tests.test_harness import PASSING, HarnessCase


class SelectionTest(HarnessCase):
    def test_a_requested_module_that_is_not_found_is_reported(self):
        # Review 2: it fails the run instead of leaving nothing to judge.
        self.write(f"{self.package}/test_a.py", PASSING)
        missing = f"{self.package}.test_gone"
        got = harness.run(self.root, "n", io.StringIO(), detail=True,
                          modules=[f"{self.package}.test_a", missing])
        self.assertEqual((got["collected"], got["uncollected"]),
                         (2, [missing]))
        self.assertFalse(harness.clean(got))

    def test_module_lines_are_read_only_with_detail(self):
        # Retrofit (mutation closure).
        self.write(f"{self.package}/test_a.py", PASSING)
        self.write(f"{self.package}/test_b.py", PASSING)
        report = io.StringIO()
        with contextlib.redirect_stderr(io.StringIO()):
            harness.main([self.root],
                         io.StringIO(f"n\n{self.package}.test_b\n"),
                         lambda: report)
        self.assertEqual(json.loads(report.getvalue())["collected"], 4)


class KeyTest(HarnessCase):
    def test_a_callable_object_test_has_no_function_key(self):
        # Retrofit (mutation closure): only real functions give a key.
        self.write(f"{self.package}/test_o.py", """
            import unittest
            class Check:
                def __call__(self):
                    pass
            class T(unittest.TestCase):
                test_obj = Check()
            """)
        got = harness.run(self.root, "n", io.StringIO(), detail=True)
        name = f"{self.package}.test_o.T.test_obj"
        self.assertEqual(got["keys"][name], [f"{self.package}.test_o",
                                             f"{self.package}.test_o.T"])

    def test_each_test_is_named_in_the_log(self):
        # Mutation closure, written before the change: the log names each
        # test, so a wrong test reaching the result shows.
        self.write(f"{self.package}/test_a.py", PASSING)
        stream = io.StringIO()
        harness.run(self.root, "n", stream)
        self.assertIn("test_a (", stream.getvalue())


if __name__ == "__main__":
    unittest.main()
