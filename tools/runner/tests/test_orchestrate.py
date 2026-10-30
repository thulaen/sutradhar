"""Review findings on the run itself (independent review, 30 Oct 2026)."""
import json
import os
import subprocess
import unittest
from unittest import mock

from tools.runner import bundle, candidate, orchestrate, signing
from tools.runner.tests.test_cli import CliCase, FakeSandbox


class ScopeTest(CliCase):
    """Finding 1: NOT_YET_ACTIVE only inside the bootstrap scope."""

    def test_change_outside_the_scope_blocks_exemptions(self):
        self.policy_doc["profiles"]["commit"]["not_yet_active"] = {
            "test-first": "activation step 3"}
        self.write_policy()
        self.repo.write("notes.txt", b"n\n")
        self.repo.commit()
        code, doc = self.run_profile("commit", base=self.tree)
        self.assertEqual((code, doc["bootstrap"]), (2, False))
        self.assertEqual(doc["reasons"],
                         ["test-first: NOT_YET_ACTIVE outside bootstrap"])

    def test_interim_evidence_is_required(self):
        code, doc = self.run_profile("docs", base=self.base, evidence=False)
        self.assertEqual((code, doc["reasons"]), (2, [
            "test-first: NOT_YET_ACTIVE without interim evidence"]))

    def test_bad_evidence_files_block(self):
        for evidence in ({"interim": {"test-first": ["relative"]}},
                         {"interim": {"test-first": "x"}}, {"reviews": [3]},
                         {"interim": {}, "reviews": [], "other": 1}, []):
            self.write_json("evidence.json", evidence)
            code, doc = self.run_profile("docs", base=self.base)
            self.assertEqual((code, doc["blocked"]),
                             (2, ["bad evidence file"]), evidence)
        self.write_json("evidence.json", {"interim": {"test-first": [
            self.path("missing.log")]}})
        code, doc = self.run_profile("docs", base=self.base)
        self.assertEqual(code, 2)


class BaseTest(CliCase):
    """Finding 3: the base is recorded and an empty change is refused."""

    def test_documentation_profile_needs_a_real_change(self):
        code, doc = self.run_profile("docs", base=self.tree)
        self.assertEqual((code, doc["blocked"]), (2, [
            "documentation-only profile refused: no change"]))

    def test_base_is_recorded_and_checked(self):
        code, doc = self.run_profile("docs", base=self.base)
        self.assertEqual((code, doc["base"]), (0, self.base))
        self.assertEqual(self.verify(profile="docs", base=self.base),
                         (0, {"status": "PASSED", "reasons": []}))
        code, doc = self.verify(profile="docs", base=self.tree)
        self.assertEqual(doc["reasons"],
                         ["base differs from the expected value"])

    def test_executable_bit_refuses_documentation_profile(self):
        self.repo.write("run.md", b"#!/bin/sh\n", executable=True)
        self.repo.commit()
        code, doc = self.run_profile("docs", base=self.tree)
        self.assertEqual((code, doc["blocked"]), (2, [
            "documentation-only profile refused: not a documentation "
            "change: run.md"]))

    def test_verify_recomputes_the_profile_choice(self):
        self.run_profile("docs", base=self.base)
        with open(self.path("out/bundle.json"), "rb") as handle:
            doc = json.loads(handle.read())
        doc["changed"] = ["b.md", "tools/x.py"]
        data = bundle.canonical(doc)
        with open(self.path("out/bundle.json"), "wb") as handle:
            handle.write(data)
        with open(self.path("out/bundle.json.sig"), "wb") as handle:
            handle.write(signing.sign(self.key, data))
        code, doc = self.verify(profile="docs", base=self.base)
        self.assertEqual((code, doc["reasons"]), (2, [
            "documentation-only profile refused: not a documentation "
            "change: tools/x.py"]))


class SwapTest(CliCase):
    """Finding 9: tools and the export cannot change while checks run."""

    def test_tools_are_copied_and_bound_from_the_copy(self):
        self.run_profile()
        self.assertEqual(candidate.path_digest(self.path("out/tools/limits")),
                         self.policy_doc["tools"]["limits"])

    def changing(self, rel):
        def during(check):
            path = self.path(rel)
            os.chmod(path, 0o644)
            with open(path, "ab") as handle:
                handle.write(b"changed\n")
        return FakeSandbox(during=during)

    def test_changed_copy_or_export_blocks(self):
        for rel, message in (
                ("out/tools/limits/classes.txt",
                 "tool changed during checks: limits"),
                ("out/candidate/a.md", "candidate changed during checks")):
            code, doc = self.run_profile(sandbox=self.changing(rel))
            self.assertEqual((code, doc["blocked"]), (2, [message]), rel)
            os.rename(self.path("out"), self.path(f"old-{len(rel)}"))

    def test_output_parent_must_be_private(self):
        os.chmod(self.dir, 0o777)
        self.addCleanup(os.chmod, self.dir, 0o700)
        code, doc = self.run_profile()
        self.assertEqual((code, doc["blocked"]),
                         (2, ["unsafe output folder"]))
        os.chmod(self.dir, 0o700)
        owner = os.stat(self.dir).st_uid
        with mock.patch("os.getuid", return_value=owner + 1):
            code, doc = self.run_profile()
        self.assertIn("unsafe output folder", doc["blocked"][0])


class RunnerIdentityTest(CliCase):
    """Findings 10 and 11: the runner and environment are evidence."""

    def test_runner_must_match_its_pin(self):
        self.policy_doc["runner"] = "b" * 64
        self.write_policy()
        code, doc = self.run_profile()
        self.assertEqual((code, doc["blocked"]),
                         (2, ["runner differs from its pin"]))

    def test_runner_digest_covers_only_its_source_files(self):
        here = os.path.dirname(os.path.abspath(orchestrate.__file__))
        names = [name for name, _ in orchestrate.runner_files()]
        self.assertIn("tools/__init__.py", names)
        self.assertIn("tools/runner/orchestrate.py", names)
        self.assertFalse([n for n in names if "tests" in n
                          or "__pycache__" in n or not n.endswith(".py")])
        self.assertTrue(os.path.isfile(os.path.join(here, "harness.py")))

    def test_environment_and_reviews_are_recorded(self):
        code, doc = self.run_profile()
        self.assertEqual(code, 0)
        self.assertEqual(doc["runner_sha256"], orchestrate.runner_digest())
        self.assertEqual(sorted(doc["environment"]["binaries"]),
                         ["git", "python3", "sh", "ssh-keygen"])
        self.assertEqual(doc["reviews"], [{
            "file": "review.md",
            "sha256": candidate.path_digest(self.path("review.md"))}])


class ClosureTest(CliCase):
    """Retrofit (coverage closure) for rarely reached refusals."""

    def resign(self, change):
        with open(self.path("out/bundle.json"), "rb") as handle:
            doc = json.loads(handle.read())
        change(doc)
        data = bundle.canonical(doc)
        with open(self.path("out/bundle.json"), "wb") as handle:
            handle.write(data)
        with open(self.path("out/bundle.json.sig"), "wb") as handle:
            handle.write(signing.sign(self.key, data))

    def test_verify_recomputes_the_bootstrap_scope(self):
        self.run_profile()
        self.resign(lambda doc: doc.update(bootstrap=False))
        code, doc = self.verify()
        self.assertEqual((code, doc["reasons"]), (2, [
            "bootstrap differs from the policy's scope"]))

    def test_unreadable_evidence_file_blocks(self):
        for content in (None, b"{"):
            if content is not None:
                self.write("evidence.json", content)
            else:
                os.remove(self.path("evidence.json"))
            code, doc = self.run_profile("docs", base=self.base)
            self.assertEqual((code, doc["blocked"]),
                             (2, ["bad evidence file"]))

    def test_tool_changed_between_check_and_copy_blocks(self):
        with open(os.path.join(self.tool, "classes.txt"), "ab") as handle:
            handle.write(b"text *.txt\n")
        with mock.patch.object(orchestrate, "_check_tools"):
            code, doc = self.run_profile()
        self.assertEqual((code, doc["blocked"]),
                         (2, ["tool changed: limits"]))

    def test_backstop_stops_the_unit(self):
        sandbox = FakeSandbox(exc=subprocess.TimeoutExpired(["x"], 1))
        code, doc = self.run_profile(sandbox=sandbox)
        self.assertEqual(sandbox.calls[1], ["systemctl", "stop",
                                            "sutradhar-limits-0123abcd"])
        self.assertEqual((doc["checks"][0]["outcome"],
                          doc["checks"][0]["reason"]),
                         ("BLOCKED", "time limit"))


class ReportTest(CliCase):
    """Finding 4: the unit tests count only with the harness report."""

    def test_missing_report_blocks(self):
        code, doc = self.run_profile(sandbox=FakeSandbox(silent=("unit",)))
        unit = doc["checks"][1]
        self.assertEqual((code, unit["outcome"], unit["reason"]),
                         (2, "BLOCKED", "no test report"))


if __name__ == "__main__":
    unittest.main()
