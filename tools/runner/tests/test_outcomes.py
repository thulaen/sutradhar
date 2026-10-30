"""Outcomes and how they combine (SPEC-001; SPEC-006 activation)."""
import unittest

from tools.runner import outcomes as o

PLAN = {"run": ["a"], "not_applicable": ["b"], "not_yet_active": ["c"],
        "interim": ["c"]}
EVIDENCE = [{"file": "red-green.log", "sha256": "e" * 64}]


def result(check, outcome, reason="", evidence=None):
    found = {"id": check, "outcome": outcome, "reason": reason}
    if evidence is not None:
        found["evidence"] = evidence
    return found


def good():
    return [result("a", o.PASSED), result("b", o.NOT_APPLICABLE, "docs"),
            result("c", o.NOT_YET_ACTIVE, "step 3", EVIDENCE)]


class FromExitTest(unittest.TestCase):
    def test_zero_passes_and_one_fails(self):
        self.assertEqual(o.from_exit(0, False), o.PASSED)
        self.assertEqual(o.from_exit(1, False), o.FAILED)

    def test_other_codes_signals_and_timeouts_block(self):
        for code in (2, 5, 124, -9, None):
            self.assertEqual(o.from_exit(code, False), o.BLOCKED, code)
        self.assertEqual(o.from_exit(0, True), o.BLOCKED)
        self.assertEqual(o.from_exit(1, True), o.BLOCKED)


class OverallTest(unittest.TestCase):
    def overall(self, results, plan=None, bootstrap=True):
        return o.overall(plan or PLAN, results, bootstrap)

    def test_checks_lists_every_planned_check_in_order(self):
        self.assertEqual(o.checks(PLAN), ["a", "b", "c"])

    def test_all_good(self):
        self.assertEqual(self.overall(good()), (o.PASSED, []))

    def test_failure_wins_over_block(self):
        plan = dict(PLAN, run=["a", "d"])
        results = good() + [result("d", o.FAILED, "exit 1")]
        results[0] = result("a", o.BLOCKED, "time limit")
        self.assertEqual(self.overall(results, plan), (o.FAILED, []))

    def test_block_alone_blocks(self):
        results = good()
        results[0] = result("a", o.BLOCKED, "sandbox exit 1")
        self.assertEqual(self.overall(results), (o.BLOCKED, []))

    def test_a_check_that_must_run_cannot_be_exempted(self):
        for outcome in (o.NOT_APPLICABLE, o.NOT_YET_ACTIVE):
            results = good()
            results[0] = result("a", outcome, "why", EVIDENCE)
            self.assertEqual(self.overall(results), (o.BLOCKED, [
                f"a: must run, reported {outcome}"]))

    def test_exemptions_must_be_the_planned_kind(self):
        results = good()
        results[1] = result("b", o.PASSED)
        results[2] = result("c", o.NOT_APPLICABLE, "why")
        self.assertEqual(self.overall(results), (o.BLOCKED, [
            "b: expected NOT_APPLICABLE, reported PASSED",
            "c: expected NOT_YET_ACTIVE, reported NOT_APPLICABLE"]))

    def test_unexplained_exemptions_block(self):
        results = good()
        results[1] = result("b", o.NOT_APPLICABLE)
        results[2] = result("c", o.NOT_YET_ACTIVE, "", EVIDENCE)
        self.assertEqual(self.overall(results), (o.BLOCKED, [
            "b: NOT_APPLICABLE without a reason",
            "c: NOT_YET_ACTIVE without a reason"]))

    def test_interim_evidence_is_required_where_planned(self):
        results = good()
        results[2] = result("c", o.NOT_YET_ACTIVE, "step 3")
        self.assertEqual(self.overall(results), (o.BLOCKED, [
            "c: NOT_YET_ACTIVE without interim evidence"]))
        results[2] = result("c", o.NOT_YET_ACTIVE, "step 3", [])
        self.assertEqual(self.overall(results)[0], o.BLOCKED)
        no_interim = dict(PLAN, interim=[])
        results[2] = result("c", o.NOT_YET_ACTIVE, "step 3")
        self.assertEqual(self.overall(results, no_interim), (o.PASSED, []))

    def test_not_yet_active_only_inside_bootstrap(self):
        self.assertEqual(self.overall(good(), bootstrap=False), (o.BLOCKED, [
            "c: NOT_YET_ACTIVE outside bootstrap"]))

    def test_missing_result_blocks_even_with_a_failure(self):
        results = [result("a", o.FAILED, "exit 1"), good()[1]]
        self.assertEqual(self.overall(results),
                         (o.BLOCKED, ["c: missing result"]))

    def test_unknown_duplicate_and_bad_outcomes_block(self):
        results = good() + [result("a", o.PASSED), result("z", o.PASSED)]
        results[1] = result("b", "GREEN", "x")
        self.assertEqual(self.overall(results), (o.BLOCKED, [
            "b: unknown outcome GREEN", "a: duplicate result",
            "z: not in the profile"]))

    def test_malformed_results_block(self):
        for bad in ("a", None, {"outcome": o.PASSED}, {"id": 3}):
            results = [bad] + good()
            self.assertEqual(self.overall(results),
                             (o.BLOCKED, ["malformed result"]), bad)

    def test_empty_profile_blocks(self):
        empty = {"run": [], "not_applicable": [], "not_yet_active": [],
                 "interim": []}
        self.assertEqual(self.overall([], empty),
                         (o.BLOCKED, ["profile has no checks"]))


if __name__ == "__main__":
    unittest.main()
