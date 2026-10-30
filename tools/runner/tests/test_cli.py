"""The runner command: export, sandboxed checks, signed bundle, verify."""
import copy
import datetime
import io
import json
import os
import subprocess
import tempfile
import unittest

from tools.runner import candidate, cli, orchestrate, signing
from tools.runner.tests.test_candidate import Repo
from tools.runner.tests.test_policy import GOOD
from tools.runner.tests.test_reports import REPORT
from tools.runner.tests.test_signing import keypair

NOW = datetime.datetime(2026, 10, 30, 12, 0, tzinfo=datetime.timezone.utc)


class FakeSandbox:
    """Stands in for systemd-run; each check's own status is chosen.

    Statuses 0 and 1 come back through the wrapper as 10 and 11; raw
    names a code systemd-run itself returns instead. The unittest check
    prints a harness summary carrying the nonce it reads from standard
    input unless it is silent; during(check) runs while a check "runs".
    """

    def __init__(self, codes=None, exc=None, raw=None, silent=(),
                 during=None):
        self.codes, self.exc, self.calls = codes or {}, exc, []
        self.raw, self.silent, self.during = raw or {}, silent, during
        self.kwargs = []

    def __call__(self, argv, **kwargs):
        self.calls.append(argv)
        self.kwargs.append(kwargs)
        if self.exc:
            raise self.exc
        unit = next(a for a in argv if a.startswith("--unit="))
        check = unit[len("--unit=sutradhar-"):].rsplit("-", 1)[0]
        status = self.codes.get(check, 0)
        out = b"out " + check.encode()
        if check == "unit" and check not in self.silent:
            nonce = kwargs.get("input", b"").decode().strip()
            report = dict(REPORT, nonce=nonce, failures=status)
            out += b"\n" + json.dumps(report).encode() + b"\n"
        kwargs["stdout"].write(out)
        if self.during:
            self.during(check)
        code = {0: 10, 1: 11}.get(status, 12)
        return subprocess.CompletedProcess(argv, self.raw.get(check, code))


class CliCase(unittest.TestCase):
    def setUp(self):
        tmp = tempfile.TemporaryDirectory()
        self.addCleanup(tmp.cleanup)
        self.dir = tmp.name
        self.repo = Repo(self.dir)
        self.repo.write("a.md", b"x\n")
        self.base = self.repo.commit()
        self.repo.write("b.md", b"y\n")
        self.tree = self.repo.commit()
        self.tool = self.path("tool-limits")
        os.mkdir(self.tool)
        self.write("tool-limits/classes.txt", b"text *.md\n")
        self.harness = self.write("harness.py", b"# harness\n")
        self.policy_doc = copy.deepcopy(GOOD)
        self.policy_doc["runner"] = orchestrate.runner_digest()
        self.policy_doc["tools"] = {
            "limits": candidate.path_digest(self.tool),
            "harness": candidate.path_digest(self.harness)}
        self.write_policy()
        self.write_json("host.json", {"prefix": [], "tools": {
            "limits": self.tool, "harness": self.harness}})
        self.keep_evidence = False
        self.evidence = {"interim": {"test-first": [self.path("red.log")]},
                         "reviews": [self.path("review.md")]}
        self.refresh_evidence()
        self.write_json("evidence.json", self.evidence)
        self.key, public = keypair(self.dir, "runner")
        with open(self.path("signers"), "w", encoding="ascii") as handle:
            handle.write(signing.signer_line("sutradhar-runner", *public))

    def path(self, name):
        return os.path.join(self.dir, name)

    def write(self, name, data):
        with open(self.path(name), "wb") as handle:
            handle.write(data)
        return self.path(name)

    def refresh_evidence(self, **changes):
        """Evidence records naming the tree that HEAD holds now."""
        tree = self.repo.git("rev-parse", "HEAD^{tree}")
        record = {"format": "sutradhar-interim/1", "tree": tree,
                  "check": "test-first", "outcome": "PASSED",
                  "by": "red and green runs"}
        record.update(changes)
        self.write("red.log", json.dumps(record).encode())
        self.write("review.md", json.dumps({
            "format": "sutradhar-review/1", "tree": tree,
            "reviewer": "independent", "verdict": "accepted"}).encode())

    def write_json(self, name, doc):
        with open(self.path(name), "w", encoding="utf-8") as handle:
            json.dump(doc, handle)

    def write_policy(self):
        self.write_json("policy.json", self.policy_doc)

    def main(self, *args, sandbox=None):
        out = io.StringIO()
        code = cli.main(list(args), out, run=sandbox or FakeSandbox(),
                        now=lambda: NOW, token=lambda: "0123abcd")
        return code, json.loads(out.getvalue())

    def run_profile(self, profile="commit", sandbox=None, base=None,
                    evidence=True, message=None):
        args = ["run", "--repo", self.repo.path, "--tree", "HEAD",
                "--profile", profile, "--policy", self.path("policy.json"),
                "--host", self.path("host.json"), "--key", self.key,
                "--out", self.path("out")]
        if base:
            args += ["--base", base]
        if message:
            args += ["--message", message]
        if evidence:
            args += ["--evidence", self.path("evidence.json")]
            if not self.keep_evidence:
                self.refresh_evidence()
        return self.main(*args, sandbox=sandbox)

    def verify(self, tree=None, profile="commit", base=None, message="none"):
        args = ["verify", "--bundle", self.path("out/bundle.json"),
                "--signature", self.path("out/bundle.json.sig"),
                "--signers", self.path("signers"),
                "--identity", "sutradhar-runner",
                "--policy", self.path("policy.json"),
                "--tree", tree or self.tree, "--profile", profile,
                "--base", base or "none", "--message", message]
        return self.main(*args)


class RunTest(CliCase):
    def test_passing_run_is_signed_and_bound_to_content(self):
        sandbox = FakeSandbox()
        code, doc = self.run_profile(sandbox=sandbox)
        self.assertEqual((code, doc["status"]), (0, "PASSED"))
        self.assertEqual(doc["tree"], self.tree)
        self.assertEqual(doc["changed"], ["a.md", "b.md"])
        exported = candidate.path_digest(self.path("out/candidate"))
        self.assertEqual(doc["manifest_sha256"], exported)
        self.assertEqual([c["id"] for c in doc["checks"]], ["limits", "unit"])
        self.assertEqual(doc["finished"], "2026-10-30T12:00:00Z")
        with open(self.path("out/bundle.json"), "rb") as handle:
            data = handle.read()
        with open(self.path("out/bundle.json.sig"), "rb") as handle:
            signature = handle.read()
        self.assertTrue(signing.verify(self.path("signers"),
                                       "sutradhar-runner", data, signature))
        with open(self.path("out/private/limits.stdout"), "rb") as handle:
            self.assertEqual(handle.read(), b"out limits")

    def test_sandbox_sees_exact_candidate_tools_and_paths(self):
        sandbox = FakeSandbox()
        self.run_profile(sandbox=sandbox)
        argv = sandbox.calls[0]
        self.assertIn("--unit=sutradhar-limits-0123abcd", argv)
        self.assertIn("--property=BindReadOnlyPaths="
                      f"{self.path('out/candidate')}:/opt/scan/candidate", argv)
        self.assertIn("--property=BindReadOnlyPaths="
                      f"{self.path('out/tools/limits')}:"
                      "/opt/sutradhar/limits", argv)
        self.assertIn("--property=WorkingDirectory=/opt/sutradhar/limits",
                      argv)
        self.assertEqual(argv[-2:], ["a.md", "b.md"])

    def test_failing_check_fails_and_is_still_signed(self):
        code, doc = self.run_profile(sandbox=FakeSandbox({"unit": 1}))
        self.assertEqual((code, doc["status"]), (1, "FAILED"))
        self.assertTrue(os.path.exists(self.path("out/bundle.json.sig")))

    def test_sandbox_start_failure_blocks_never_fails(self):
        code, doc = self.run_profile(sandbox=FakeSandbox(raw={"unit": 1}))
        self.assertEqual((code, doc["status"]), (2, "BLOCKED"))
        unit = doc["checks"][1]
        self.assertEqual((unit["outcome"], unit["reason"]),
                         ("BLOCKED", "sandbox exit 1"))

    def test_docs_profile_records_exemptions(self):
        code, doc = self.run_profile("docs", base=self.base)
        self.assertEqual((code, doc["status"]), (0, "PASSED"))
        self.assertEqual(doc["changed"], ["b.md"])
        self.assertEqual([(c["id"], c["outcome"]) for c in doc["checks"]], [
            ("limits", "PASSED"), ("unit", "NOT_APPLICABLE"),
            ("test-first", "NOT_YET_ACTIVE")])
        self.assertEqual(doc["checks"][2]["evidence"], [
            {"file": "red.log",
             "sha256": candidate.path_digest(self.path("red.log"))}])

    def test_docs_profile_refused_for_executable_change(self):
        self.repo.write("tools/x.py", b"\n")
        self.repo.commit()
        code, doc = self.run_profile("docs", base=self.tree)
        self.assertEqual((code, doc["status"]), (2, "BLOCKED"))
        self.assertIn("not a documentation change: tools/x.py",
                      doc["blocked"][0])
        self.assertFalse(os.path.exists(self.path("out")))

    def test_changed_or_missing_tool_blocks(self):
        with open(os.path.join(self.tool, "classes.txt"), "ab") as handle:
            handle.write(b"text *.txt\n")
        code, doc = self.run_profile()
        self.assertEqual((code, doc["blocked"]), (2, ["tool changed: limits"]))
        self.write_json("host.json", {"prefix": [], "tools": {
            "harness": self.harness}})
        code, doc = self.run_profile()
        self.assertEqual((code, doc["blocked"]),
                         (2, ["tool not installed: limits"]))

    def test_bad_host_configuration_blocks(self):
        for host in ({"prefix": "sudo", "tools": {}},
                     {"prefix": [], "tools": {"limits": "relative"}},
                     {"tools": {}}, []):
            self.write_json("host.json", host)
            code, doc = self.run_profile()
            self.assertEqual((code, doc["blocked"]),
                             (2, ["bad host configuration"]), host)

    def test_unexpected_error_blocks(self):
        code, doc = self.run_profile(sandbox=FakeSandbox(
            exc=RuntimeError("boom")))
        self.assertEqual((code, doc["blocked"]),
                         (2, ["internal error: RuntimeError"]))


class VerifyAndUsageTest(CliCase):
    def test_verify_good_wrong_tree_and_tampered(self):
        self.run_profile()
        self.assertEqual(self.verify(), (0, {"status": "PASSED",
                                              "reasons": []}))
        code, doc = self.verify(tree=self.base)
        self.assertEqual((code, doc["reasons"]),
                         (2, ["tree differs from the expected value"]))
        code, doc = self.verify(profile="docs", base=self.base)
        self.assertEqual((code, doc["reasons"]), (2, [
            "profile differs from the expected value",
            "plan differs from the expected value",
            "base differs from the expected value"]))
        code, doc = self.main(
            "verify", "--bundle", "b", "--signature", "s", "--signers", "n",
            "--identity", "i", "--policy", "p", "--tree", "t",
            "--profile", "commit")
        self.assertEqual((code, doc["blocked"]), (2, [
            "usage: verify needs --bundle --signature --signers --identity "
            "--policy --tree --profile --base --message"]))
        with open(self.path("out/bundle.json"), "ab") as handle:
            handle.write(b" ")
        self.assertEqual(self.verify(), (2, {
            "status": "BLOCKED", "reasons": ["signature not valid"]}))

    def test_verify_reports_failure(self):
        self.run_profile(sandbox=FakeSandbox({"unit": 1}))
        self.assertEqual(self.verify(), (1, {"status": "FAILED",
                                              "reasons": []}))



if __name__ == "__main__":
    unittest.main()
