"""Evidence bundles: canonical form, recomputation and verification."""
import datetime
import json
import unittest

from tools.runner import bundle
from tools.runner import outcomes as o

NOW = datetime.datetime(2026, 10, 30, 12, 0, tzinfo=datetime.timezone.utc)
FACTS = {"tree": "t" * 40, "base": None, "manifest_sha256": "m" * 64,
         "profile": "commit", "changed": ["a.md"], "changed_executable": [],
         "policy_version": "p1", "policy_sha256": "p" * 64,
         "runner_sha256": "r" * 64, "environment": {"python": "3.14.4"},
         "tools": {"limits": "l" * 64}, "reviews": [],
         "started": "2026-10-30T11:59:00Z",
         "finished": "2026-10-30T11:59:30Z"}
PLAN = {"run": ["a", "b"], "not_applicable": [], "not_yet_active": [],
        "interim": []}


def result(check, outcome, reason=""):
    return {"id": check, "outcome": outcome, "reason": reason}


def make(*outcomes, plan=PLAN, bootstrap=False, **changes):
    results = [result(c, out, "r") for c, out in zip("ab", outcomes)]
    doc = bundle.build(FACTS, plan, results, bootstrap)
    doc.update(changes)
    return doc


def trusting(data, signature):
    return signature == b"good"


class BuildTest(unittest.TestCase):
    def test_build_records_facts_and_recomputed_status(self):
        doc = make(o.PASSED, o.PASSED)
        self.assertEqual(doc["format"], bundle.FORMAT)
        self.assertEqual((doc["status"], doc["reasons"]), (o.PASSED, []))
        self.assertEqual(doc["plan"], PLAN)
        self.assertFalse(doc["bootstrap"])
        for key, value in FACTS.items():
            self.assertEqual(doc[key], value)

    def test_build_reports_missing_results(self):
        doc = bundle.build(FACTS, PLAN, [result("a", o.PASSED)], True)
        self.assertEqual((doc["status"], doc["reasons"]),
                         (o.BLOCKED, ["b: missing result"]))

    def test_canonical_is_compact_sorted_ascii(self):
        self.assertEqual(bundle.canonical({"b": 1, "a": chr(0xE9)}),
                         b'{"a":"\\u00e9","b":1}')
        with self.assertRaises(ValueError):
            bundle.canonical({"a": float("nan")})


class VerifyTest(unittest.TestCase):
    def verify(self, doc, signature=b"good", expect=None, data=None,
               extra=None):
        data = bundle.canonical(doc) if data is None else data
        expect = {"tree": FACTS["tree"]} if expect is None else expect
        if extra is None:
            return bundle.verify(data, signature, trusting, expect, NOW,
                                 3600)
        return bundle.verify(data, signature, trusting, expect, NOW, 3600,
                             extra)

    def test_good_bundle(self):
        self.assertEqual(self.verify(make(o.PASSED, o.PASSED)),
                         (o.PASSED, []))

    def test_failing_bundle_stays_failed(self):
        self.assertEqual(self.verify(make(o.PASSED, o.FAILED)),
                         (o.FAILED, []))

    def test_bad_signature(self):
        self.assertEqual(self.verify(make(o.PASSED, o.PASSED), b"bad"),
                         (o.BLOCKED, ["signature not valid"]))

    def test_non_canonical_or_non_json(self):
        doc = make(o.PASSED, o.PASSED)
        for data in (json.dumps(doc, indent=1).encode(), b"{", b"[]",
                     b'{"a":NaN}'):
            self.assertEqual(self.verify(doc, data=data),
                             (o.BLOCKED, ["not canonical JSON"]), data)

    def test_expectations(self):
        doc = make(o.PASSED, o.PASSED)
        got = self.verify(doc, expect={"tree": "x" * 40, "profile": "docs",
                                       "plan": PLAN, "base": "b" * 40})
        self.assertEqual(got, (o.BLOCKED, [
            "tree differs from the expected value",
            "profile differs from the expected value",
            "base differs from the expected value"]))

    def test_extra_checks_see_the_verified_document(self):
        seen = []

        def extra(doc):
            seen.append(doc["changed"])
            return ["documentation-only profile refused: no change"]
        got = self.verify(make(o.PASSED, o.PASSED), extra=extra)
        self.assertEqual(got, (o.BLOCKED, [
            "documentation-only profile refused: no change"]))
        self.assertEqual(seen, [["a.md"]])
        self.assertEqual(self.verify(make(o.PASSED, o.PASSED),
                                     extra=lambda doc: []), (o.PASSED, []))

    def test_false_status_claim_is_caught(self):
        doc = make(o.PASSED, o.FAILED, status=o.PASSED)
        self.assertEqual(self.verify(doc), (o.BLOCKED, [
            "status claims PASSED; checks give FAILED"]))

    def test_bootstrap_exemptions_are_recomputed_with_the_bundle_flag(self):
        plan = dict(PLAN, run=["a"], not_yet_active=["b"])
        results = [result("a", o.PASSED, "r"),
                   result("b", o.NOT_YET_ACTIVE, "activation step 3")]
        doc = bundle.build(FACTS, plan, results, True)
        self.assertEqual(self.verify(doc), (o.PASSED, []))
        doc = bundle.build(FACTS, plan, results, False)
        self.assertEqual(self.verify(doc), (o.BLOCKED, [
            "b: NOT_YET_ACTIVE outside bootstrap"]))

    def test_unsupported_format(self):
        doc = make(o.PASSED, o.PASSED, format="sutradhar-evidence/1")
        self.assertEqual(self.verify(doc),
                         (o.BLOCKED, ["unsupported format"]))

    def test_age(self):
        old = make(o.PASSED, o.PASSED, finished="2026-10-30T10:59:59Z")
        self.assertEqual(self.verify(old), (o.BLOCKED, ["evidence expired"]))
        edge = make(o.PASSED, o.PASSED, finished="2026-10-30T11:00:00Z")
        self.assertEqual(self.verify(edge), (o.PASSED, []))
        future = make(o.PASSED, o.PASSED, finished="2026-10-30T12:05:01Z")
        self.assertEqual(self.verify(future),
                         (o.BLOCKED, ["finished in the future"]))
        near = make(o.PASSED, o.PASSED, finished="2026-10-30T12:05:00Z")
        self.assertEqual(self.verify(near), (o.PASSED, []))

    def test_malformed(self):
        cases = [{"finished": "yesterday"}, {"finished": 5},
                 {"checks": {"a": 1}}, {"plan": ["a"]},
                 {"plan": dict(PLAN, run="ab")}, {"plan": {"run": []}},
                 {"bootstrap": "no"}, {"status": 1}, {"changed": "a.md"},
                 {"changed_executable": None}]
        for change in cases:
            doc = make(o.PASSED, o.PASSED)
            doc.update(change)
            self.assertEqual(self.verify(doc),
                             (o.BLOCKED, ["malformed bundle"]), change)
        doc = make(o.PASSED, o.PASSED)
        del doc["finished"]
        self.assertEqual(self.verify(doc), (o.BLOCKED, ["malformed bundle"]))


if __name__ == "__main__":
    unittest.main()
