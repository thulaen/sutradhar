"""The test-first check run by the runner (SPEC-007 part 2, AT-T01/T02)."""
import hashlib
import json
import os
import subprocess
import unittest

from tools.runner import candidate, harness
from tools.runner.tests.test_cli import CliCase, FakeSandbox

NEW, OLD = "pkg.tests.test_code.T.test_new", "pkg.tests.test_code.T.test_old"
TESTS = """import unittest
from pkg import code
class T(unittest.TestCase):
    def test_old(self):
        self.assertTrue(code.f())
"""
ADDED = TESTS + """    def test_new(self):
        self.assertEqual(code.f(), 2)
"""


def detail(ids, failed=(), errored=()):
    return {"harness": "sutradhar-unittest/1", "collected": len(ids),
            "run": len(ids), "failures": len(failed),
            "errors": len(errored), "skipped": 0, "uncollected": [],
            "ids": sorted(ids), "failed_ids": sorted(failed),
            "errored_ids": sorted(errored),
            "keys": {name: [name.rsplit(".", 2)[0], name.rsplit(".", 1)[0],
                            name] for name in ids}}


class FakeHarness(FakeSandbox):
    """FakeSandbox whose harness runs report chosen test ids."""

    def __init__(self, red=None, green=None, nonce=None, disagree=(),
                 **kwargs):
        super().__init__(**kwargs)
        self.details = {"test-first-red": red or detail([OLD, NEW], [NEW]),
                        "test-first-green": green or detail([OLD, NEW])}
        self.nonce, self.disagree, self.bound = nonce, disagree, {}
        self.inputs = {}

    def __call__(self, argv, **kwargs):
        unit = next(a for a in argv if a.startswith("--unit="))
        check = unit[len("--unit=sutradhar-"):].rsplit("-", 1)[0]
        if check not in self.details:
            return super().__call__(argv, **kwargs)
        self.calls.append(argv)
        bind = next(a for a in argv if a.endswith(":/opt/scan/candidate"))
        self.bound[check] = bind.split("=", 2)[2].rsplit(":", 1)[0]
        lines = kwargs["input"].decode().split("\n")
        self.inputs[check] = lines[1:]
        summary = dict(self.details[check], nonce=self.nonce or lines[0])
        if check not in self.silent:
            kwargs["stdout"].write(json.dumps(summary).encode() + b"\n")
        status = int(not harness.clean(summary)) ^ (check in self.disagree)
        return subprocess.CompletedProcess(argv,
                                           self.raw.get(check, 10 + status))


class RedGreenCase(CliCase):
    def setUp(self):
        super().setUp()
        self.policy_doc["checks"]["test-first"] = {
            "cwd": "@candidate", "report": "test-first",
            "argv": ["python3", "-I", "@tool:harness", "--detail",
                     "@candidate"]}
        self.policy_doc["profiles"]["docs"].update(not_yet_active={},
                                                   interim=[])
        self.policy_doc["profiles"]["tdd"] = {
            "documentation_only": False, "run": ["test-first", "unit"],
            "not_applicable": {}, "not_yet_active": {}, "interim": []}
        self.write_policy()
        for rel, data in (("pkg/__init__.py", ""),
                          ("pkg/tests/__init__.py", ""),
                          ("pkg/code.py", "def f():\n    return 1\n"),
                          ("pkg/tests/test_code.py", TESTS)):
            self.repo.write(rel, data.encode())
        self.parent = self.repo.commit()
        self.repo.write("pkg/code.py", b"def f():\n    return 2\n")
        self.repo.write("pkg/tests/test_code.py", ADDED.encode())
        self.tree = self.repo.commit()

    def tdd(self, message=b"feat: f\n\nTDD: test-first\n", sandbox=None,
            base=True):
        if message is not None:
            self.write("message.txt", message)
        self.sandbox = sandbox or FakeHarness()
        code, doc = self.run_profile(
            "tdd", sandbox=self.sandbox, base=self.parent if base else None,
            message=None if message is None else self.path("message.txt"))
        return code, doc, doc["checks"][0] if "checks" in doc else None

    def runs(self, check):
        return [run["id"] for run in check.get("runs", [])]

    def read(self, rel):
        with open(self.path(rel), "rb") as handle:
            return handle.read()


class TestFirstTest(RedGreenCase):
    def test_a_behavioural_red_on_the_parent_passes(self):
        code, doc, check = self.tdd()
        self.assertEqual((code, check["outcome"], check["reason"],
                          check["declaration"], check["qualifying"]),
                         (0, "PASSED", "", "test-first", [NEW]))
        self.assertEqual(self.runs(check),
                         ["test-first-green", "test-first-red"])
        self.assertEqual(self.sandbox.bound, {
            "test-first-green": self.path("out/candidate"),
            "test-first-red": self.path("out/red")})
        self.assertEqual(self.read("out/red/pkg/code.py"),
                         b"def f():\n    return 1\n")
        self.assertEqual(self.read("out/red/pkg/tests/test_code.py"),
                         ADDED.encode())
        self.assertEqual(doc["message_sha256"], hashlib.sha256(
            b"feat: f\n\nTDD: test-first\n").hexdigest())
        chosen = self.sandbox.inputs["test-first-red"]
        self.assertEqual((check["modules"], chosen),
                         (["pkg.tests.test_code"],
                          ["pkg.tests.test_code", ""]))
        self.assertEqual(check["red_manifest_sha256"],
                         candidate.path_digest(self.path("out/red")))

    def test_a_new_test_passing_on_the_parent_is_not_evidence(self):
        code, doc, check = self.tdd(sandbox=FakeHarness(
            red=detail([OLD, NEW])))
        self.assertEqual((code, check["outcome"], check["reason"]), (
            1, "FAILED", "no new or changed test fails on the parent by "
            "assertion"))

    def test_an_import_error_counts_only_with_a_reviewed_note(self):
        red = detail([OLD, NEW], errored=[NEW])
        code, doc, check = self.tdd(sandbox=FakeHarness(red=red))
        self.assertEqual((code, check["outcome"]), (1, "FAILED"))
        note = {"format": "sutradhar-new-unit/1", "tree": self.tree,
                "tests": [NEW], "note": "pkg.code did not exist",
                "reviewer": "independent"}
        self.write_json("unit.json", note)
        self.keep_evidence = True
        self.write_json("evidence.json",
                        {"new_units": [self.path("unit.json")]})
        os.rename(self.path("out"), self.path("out-1"))
        code, doc, check = self.tdd(sandbox=FakeHarness(red=red))
        self.assertEqual((code, check["qualifying"]), (0, [NEW]))
        self.assertEqual(
            {key: doc["new_units"][0][key]
             for key in ("tests", "reviewer", "note")},
            {"tests": [NEW], "reviewer": "independent",
             "note": "pkg.code did not exist"})
        self.write_json("unit.json", dict(note, note=" "))
        code, doc, check = self.tdd(sandbox=FakeHarness(red=red))
        self.assertEqual(doc["blocked"], ["bad evidence record: unit.json"])

    def test_an_evidence_file_may_name_every_kind_of_record(self):
        # Retrofit (mutation closure): all three lists together are valid.
        note = {"format": "sutradhar-new-unit/1", "tree": self.tree,
                "tests": [NEW], "note": "new", "reviewer": "independent"}
        self.write_json("unit.json", note)
        self.write_json("evidence.json", dict(
            self.evidence, new_units=[self.path("unit.json")]))
        code, doc, check = self.tdd()
        self.assertEqual((code, len(doc["new_units"]), len(doc["reviews"])),
                         (0, 1, 1))

    def test_a_missing_or_repeated_declaration_fails_not_blocks(self):
        # AT-G19: omitting the declaration is a quality failure.
        code, doc, check = self.tdd(message=None)
        self.assertEqual((code, check["outcome"], check["reason"]),
                         (1, "FAILED", "no valid TDD declaration"))
        self.assertEqual((doc["message_sha256"], self.runs(check)),
                         (None, []))
        os.rename(self.path("out"), self.path("out-1"))
        code, doc, check = self.tdd(b"TDD: test-first\nTDD: retrofit\n")
        self.assertEqual((code, check["declaration"]), (1, None))

    def test_only_new_or_changed_tests_count(self):
        code, doc, check = self.tdd(sandbox=FakeHarness(
            red=detail([OLD, NEW], [OLD])))
        self.assertEqual((code, check["outcome"]), (1, "FAILED"))

    def test_a_green_failure_fails_before_the_red_run(self):
        code, doc, check = self.tdd(sandbox=FakeHarness(
            green=detail([OLD, NEW], [NEW])))
        self.assertEqual((code, check["reason"], self.runs(check)),
                         (1, "tests fail on the candidate",
                          ["test-first-green"]))

    def test_runs_that_decide_nothing_block(self):
        for number, (sandbox, reason) in enumerate((
                (FakeHarness(silent=("test-first-red",)),
                 "red run: no test report"),
                (FakeHarness(nonce="other"),
                 "green run: test report not for this run"),
                (FakeHarness(disagree=("test-first-red",)),
                 "red run: exit status and test report disagree"),
                (FakeHarness(raw={"test-first-green": 1}),
                 "green run: sandbox exit 1"),
                (FakeHarness(green=detail([])),
                 "green run: no tests ran"))):
            code, doc, check = self.tdd(sandbox=sandbox)
            self.assertEqual((code, check["outcome"], check["reason"]),
                             (2, "BLOCKED", reason))
            os.rename(self.path("out"), self.path(f"out-{number}"))

    def test_a_root_commit_red_tree_holds_only_tests(self):
        code, doc, check = self.tdd(base=False)
        self.assertEqual(code, 0)
        self.assertFalse(os.path.exists(self.path("out/red/pkg/code.py")))
        self.assertTrue(os.path.exists(self.path("out/red/pkg/tests")))

    def test_an_unparsable_test_file_holds_no_test(self):
        self.repo.write("pkg/tests/test_code.py", b"def (:\n")
        self.tree = self.repo.commit()
        code, doc, check = self.tdd()
        self.assertEqual((code, check["reason"]),
                         (1, "test-first: no new or changed test"))

    def test_verify_binds_the_message(self):
        self.tdd()
        message = self.path("message.txt")
        good = self.verify(profile="tdd", base=self.parent, message=message)
        self.assertEqual(good, (0, {"status": "PASSED", "reasons": []}))
        for other in ("none", self.write("other.txt", b"TDD: retrofit\n")):
            code, doc = self.verify(profile="tdd", base=self.parent,
                                    message=other)
            self.assertEqual(doc["reasons"], [
                "message_sha256 differs from the expected value"])


class DeclarationKindTest(RedGreenCase):
    def commit(self, rel, data):
        self.repo.write(rel, data)
        self.parent, self.tree = self.tree, self.repo.commit()

    def test_a_refactor_may_not_edit_tests(self):
        # AT-T02: a refactor commit that edits test assertions is refused.
        code, doc, check = self.tdd(b"TDD: not-applicable (refactor)\n")
        self.assertEqual((code, check["reason"], self.runs(check)),
                         (1, "refactor edits tests", []))
        os.rename(self.path("out"), self.path("out-1"))
        self.commit("pkg/code.py", b"def f():\n    return 1 + 1\n")
        code, doc, check = self.tdd(b"TDD: not-applicable (refactor)\n")
        self.assertEqual((code, check["reason"], self.runs(check)),
                         (0, "", []))

    def test_a_retrofit_adds_tests_and_changes_no_code(self):
        # Review: a retrofit that changes code would skip the red check.
        for number, (rel, code, reason, runs) in enumerate((
                ("pkg/code.py", 1, "retrofit changes code: pkg/code.py", []),
                ("b3.md", 1, "retrofit: no new or changed test", []),
                ("pkg/tests/test_more.py", 0, "", []))):
            self.commit(rel, ADDED.encode())
            got = self.tdd(b"TDD: retrofit\n")
            self.assertEqual((got[0], got[2]["reason"], self.runs(got[2])),
                             (code, reason, runs), rel)
            os.rename(self.path("out"), self.path(f"out-{number}"))

    def test_other_reasons_are_judged_by_the_changed_paths(self):
        for number, (rel, line, code, reason) in enumerate((
                ("b2.md", "documentation", 0, ""),
                ("requirements.txt", "dependency", 0, ""),
                ("tools/x.py", "documentation", 1,
                 "not a documentation change: tools/x.py"),
                ("key.py", "secret", 1, "not a secret change: key.py"))):
            self.commit(rel, b"x\n")
            got = self.tdd(f"TDD: not-applicable ({line})\n".encode())
            self.assertEqual((got[0], got[2]["reason"], self.runs(got[2])),
                             (code, reason, []), rel)
            os.rename(self.path("out"), self.path(f"out-{number}"))


if __name__ == "__main__":
    unittest.main()
