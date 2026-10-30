"""Judging a check by its structured report (SPEC-001 partial reports)."""
import json
import os
import tempfile
import unittest

from tools.runner import reports
from tools.runner import outcomes as o

REPORT = {"harness": "sutradhar-unittest/1", "nonce": "n", "collected": 3,
          "run": 3, "failures": 0, "errors": 0, "skipped": 0,
          "uncollected": []}


class JudgeTest(unittest.TestCase):
    def setUp(self):
        tmp = tempfile.TemporaryDirectory()
        self.addCleanup(tmp.cleanup)
        self.path = os.path.join(tmp.name, "out")

    def judge(self, text, outcome=o.PASSED, reason=""):
        with open(self.path, "w", encoding="utf-8") as handle:
            handle.write(text)
        return reports.unittest_judge("n")(self.path, outcome, reason)

    def report(self, **changes):
        return "test output\n" + json.dumps(dict(REPORT, **changes)) + "\n"

    def test_good_report_and_exit_pass(self):
        self.assertEqual(self.judge(self.report()), (o.PASSED, ""))
        self.assertEqual(self.judge(self.report() + "\n\n"), (o.PASSED, ""))

    def test_blocked_checks_stay_blocked(self):
        self.assertEqual(self.judge("", o.BLOCKED, "time limit"),
                         (o.BLOCKED, "time limit"))

    def test_exit_failure_with_a_clean_report_disagrees(self):
        self.assertEqual(self.judge(self.report(), o.FAILED, "exit 1"), (
            o.BLOCKED, "exit status and test report disagree"))

    def test_missing_or_malformed_reports_block(self):
        for text in ("", "Ran 3 tests\nOK\n", "{\n",
                     self.report(harness="other/1"),
                     self.report(collected="3"),
                     self.report(uncollected="x"),
                     json.dumps({"harness": "sutradhar-unittest/1"}) + "\n"):
            self.assertEqual(self.judge(text), (o.BLOCKED, "no test report"),
                             text)

    def test_report_must_be_for_this_run(self):
        self.assertEqual(self.judge(self.report(nonce="m")),
                         (o.BLOCKED, "test report not for this run"))

    def test_counts_must_show_every_test_ran(self):
        self.assertEqual(self.judge(self.report(collected=0, run=0)),
                         (o.BLOCKED, "no tests ran"))
        self.assertEqual(self.judge(self.report(run=2)),
                         (o.BLOCKED, "2 of 3 tests ran"))

    def test_failures_skips_and_uncollected_files_fail(self):
        cases = [({"uncollected": ["a/test_x.py", "b/test_y.py"]},
                  "test files not collected: a/test_x.py, b/test_y.py"),
                 ({"failures": 1, "errors": 2}, "1 failed, 2 errors"),
                 ({"errors": 1}, "0 failed, 1 errors"),
                 ({"skipped": 2}, "2 skipped")]
        for changes, reason in cases:
            self.assertEqual(self.judge(self.report(**changes)),
                             (o.FAILED, reason), changes)

    def test_undecodable_output_and_blank_lines_are_tolerated(self):
        with open(self.path, "wb") as handle:
            handle.write(b"\xff\xfe junk\n" + self.report().encode()
                         + b"   \n")
        self.assertEqual(reports.unittest_judge("n")(self.path, o.PASSED, ""),
                         (o.PASSED, ""))
        self.assertEqual(self.judge("  \n\n"), (o.BLOCKED, "no test report"))

    def test_only_the_tail_of_large_output_is_read(self):
        text = "x" * (reports.TAIL + 10) + "\n" + self.report()
        self.assertEqual(self.judge(text), (o.PASSED, ""))


class DetailTest(unittest.TestCase):
    """The test-first check reads each test's outcome (whole output)."""

    def setUp(self):
        tmp = tempfile.TemporaryDirectory()
        self.addCleanup(tmp.cleanup)
        self.path = os.path.join(tmp.name, "out")
        self.good = dict(REPORT, ids=["a.T.test_x"], failed_ids=[],
                         errored_ids=["a.T.test_x"],
                         keys={"a.T.test_x": ["a", "a.T"]})

    def detail(self, text):
        with open(self.path, "w", encoding="utf-8") as handle:
            handle.write(text)
        return reports.detail(self.path)

    def test_detail_needs_the_three_id_lists(self):
        self.assertEqual(self.detail("x\n" + json.dumps(self.good) + "\n"),
                         self.good)
        missing = dict(self.good)
        del missing["ids"]
        for doc in (missing, dict(self.good, ids="a"),
                    dict(self.good, failed_ids=[1]),
                    dict(self.good, errored_ids=None),
                    dict(self.good, harness="other/1"),
                    dict(self.good, keys=[]),
                    {k: v for k, v in self.good.items() if k != "keys"},
                    dict(self.good, keys={"a.T.test_x": "a"}),
                    dict(self.good, keys={"a.T.test_x": [1]})):
            self.assertIsNone(self.detail(json.dumps(doc) + "\n"), doc)
        self.assertIsNone(self.detail(""))
        self.assertIsNone(self.detail("[" * 100_000 + "\n"))

    def test_a_long_summary_is_read_whole(self):
        ids = [f"m.T.test_{n:06d}" for n in range(8000)]
        doc = dict(self.good, ids=ids)
        self.assertGreater(len(json.dumps(doc)), reports.TAIL)
        self.assertEqual(self.detail(json.dumps(doc) + "\n")["ids"], ids)


class ExitJudgeTest(unittest.TestCase):
    def test_exit_judge_keeps_the_exit_outcome(self):
        self.assertEqual(reports.exit_judge("p", o.FAILED, "exit 1"),
                         (o.FAILED, "exit 1"))


if __name__ == "__main__":
    unittest.main()
