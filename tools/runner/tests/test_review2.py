"""Second review and mutation findings on the run (30 October 2026)."""
import json
import os
import shutil
import stat
import subprocess
import tempfile
import unittest
from unittest import mock

from tools.runner import bundle, candidate, orchestrate, policy, signing
from tools.runner.tests.test_cli import CliCase, FakeSandbox
from tools.runner.tests.test_policy import GOOD


class EvidenceTest(CliCase):
    def test_evidence_must_name_this_tree(self):
        # Finding D: an old report from another tree is refused.
        self.keep_evidence = True
        self.refresh_evidence(tree="0" * 40)
        code, doc = self.run_profile("docs", base=self.base)
        self.assertEqual((code, doc["blocked"]),
                         (2, ["evidence not for this tree: red.log"]))

    def test_evidence_must_be_a_passing_record_for_its_check(self):
        # Review 3 finding 2: a structured record, not any text.
        self.keep_evidence = True
        for change, message in (
                ({"check": "coverage"}, "evidence for another check"),
                ({"outcome": "FAILED"}, "evidence does not show a pass"),
                ({"by": ""}, "bad evidence record"),
                ({"format": "other/1"}, "bad evidence record")):
            self.refresh_evidence(**change)
            code, doc = self.run_profile("docs", base=self.base)
            self.assertEqual((code, doc["blocked"]),
                             (2, [f"{message}: red.log"]), change)
        self.write("red.log", b"not a record")
        code, doc = self.run_profile("docs", base=self.base)
        self.assertEqual(doc["blocked"], ["bad evidence record: red.log"])

    def test_review_records_are_checked_too(self):
        self.keep_evidence = True
        self.refresh_evidence()
        self.write("review.md", b"{}")
        code, doc = self.run_profile("docs", base=self.base)
        self.assertEqual((code, doc["blocked"]),
                         (2, ["bad evidence record: review.md"]))

    def test_links_in_a_tool_are_refused_not_followed(self):
        # Review 3 finding 7: a link is copied as a link, then refused.
        os.symlink(self.path("review.md"), os.path.join(self.tool, "link"))
        with mock.patch.object(orchestrate, "_check_tools"):
            code, doc = self.run_profile()
        self.assertEqual(code, 2)
        self.assertTrue(doc["blocked"][0].startswith("symbolic link in "))

    def test_reviews_may_be_left_out(self):
        self.write_json("evidence.json",
                        {"interim": self.evidence["interim"]})
        code, doc = self.run_profile("docs", base=self.base)
        self.assertEqual((code, doc["reviews"]), (0, []))


class FolderTest(CliCase):
    """Finding E and N: every ancestor of the output folder is safe."""

    def test_writable_ancestor_is_refused(self):
        shared = self.path("shared")
        os.mkdir(shared)
        os.chmod(shared, 0o777)
        os.mkdir(os.path.join(shared, "inner"), 0o700)
        args = ["run", "--repo", self.repo.path, "--tree", "HEAD",
                "--profile", "commit", "--policy", self.path("policy.json"),
                "--host", self.path("host.json"), "--key", self.key,
                "--out", os.path.join(shared, "inner", "out")]
        code, doc = self.main(*args)
        self.assertEqual((code, doc["blocked"]),
                         (2, ["unsafe output folder"]))
        os.chmod(shared, 0o720)
        self.assertEqual(self.main(*args)[1]["blocked"],
                         ["unsafe output folder"])

    def test_linked_ancestor_and_relative_paths_are_refused(self):
        os.mkdir(self.path("real"), 0o700)
        os.symlink(self.path("real"), self.path("link"))
        for out in (os.path.join(self.path("link"), "out"), "relative/out",
                    self.path("with space")):
            args = ["run", "--repo", self.repo.path, "--tree", "HEAD",
                    "--profile", "commit", "--policy",
                    self.path("policy.json"), "--host",
                    self.path("host.json"), "--key", self.key, "--out", out]
            code, doc = self.main(*args)
            self.assertEqual((code, doc["blocked"]),
                             (2, ["unsafe output folder"]), out)


class IdentityTest(CliCase):
    def test_runner_digest_covers_exactly_the_source_files(self):
        entries = [{"path": name, "executable": False,
                    "sha256": candidate.file_sha256(path)}
                   for name, path in orchestrate.runner_files()]
        self.assertEqual(orchestrate.runner_digest(),
                         candidate.digest(entries))

    def test_missing_programs_are_recorded_as_absent(self):
        with mock.patch.object(orchestrate, "_BINARIES",
                               ("sh", "no-such-program-sutradhar")):
            found = orchestrate.environment()["binaries"]
        sh = shutil.which("sh", path="/usr/bin:/bin")
        self.assertEqual(found, {
            "sh": candidate.file_sha256(os.path.realpath(sh)),
            "no-such-program-sutradhar": None})

    def test_stopping_a_unit_is_quiet_and_bounded(self):
        sandbox = FakeSandbox(exc=subprocess.TimeoutExpired(["x"], 1))
        self.run_profile(sandbox=sandbox)
        self.assertEqual(sandbox.kwargs[1], {"capture_output": True,
                                             "timeout": 60})

    def test_nonce_reaches_only_the_harness_check(self):
        sandbox = FakeSandbox()
        self.run_profile(sandbox=sandbox)
        self.assertNotIn("input", sandbox.kwargs[0])
        self.assertEqual(sandbox.kwargs[1]["input"], b"0123abcd\n")


class SafeFolderTest(unittest.TestCase):
    """Mutation closure: every combination of owner, sticky bit and mode."""

    def judged(self, uid, mode, link=False):
        kind = stat.S_IFLNK if link else stat.S_IFDIR
        info = os.stat_result((kind | mode, 0, 0, 0, uid, 0, 0, 0, 0, 0))
        with mock.patch("os.lstat", return_value=info):
            return orchestrate._safe_folder("/x")

    def test_combinations(self):
        me = os.getuid()
        self.assertTrue(self.judged(0, 0o755))
        self.assertTrue(self.judged(me, 0o700))
        self.assertTrue(self.judged(0, 0o1777))
        self.assertFalse(self.judged(0, 0o777))
        self.assertFalse(self.judged(me + 1, 0o755))
        if me != 0:
            self.assertFalse(self.judged(me, 0o1777))
        self.assertFalse(self.judged(0, 0o755, link=True))


class SearchPathTest(unittest.TestCase):
    def test_programs_are_found_only_on_the_fixed_path(self):
        with tempfile.TemporaryDirectory() as folder:
            fake = os.path.join(folder, "sh")
            with open(fake, "w", encoding="ascii") as handle:
                handle.write("#!/bin/sh\n")
            os.chmod(fake, 0o755)
            with mock.patch.dict(os.environ, {"PATH": folder}), \
                    mock.patch.object(orchestrate, "_BINARIES", ("sh",)):
                found = orchestrate.environment()["binaries"]["sh"]
        real = shutil.which("sh", path="/usr/bin:/bin")
        self.assertEqual(found,
                         candidate.file_sha256(os.path.realpath(real)))


class PolicyClosureTest(unittest.TestCase):
    def test_every_active_check_is_named(self):
        doc = json.loads(json.dumps(GOOD))
        doc["checks"]["extra"] = {"cwd": "@candidate", "report": "exit",
                                  "argv": ["x"]}
        doc["profiles"]["docs"].update(
            not_applicable={}, interim=[],
            not_yet_active={"unit": "r", "extra": "r"})
        with self.assertRaises(policy.PolicyError) as caught:
            policy.load(json.dumps(doc).encode())
        self.assertEqual(str(caught.exception), "profiles.docs: an active "
                         "check cannot be not yet active: extra, unit")

    def test_patterns_without_a_slash_match_the_file_name(self):
        doc = json.loads(json.dumps(GOOD))
        doc["documentation"] = ["NOTES.md"]
        loaded = policy.load(json.dumps(doc).encode())
        self.assertIs(policy.select(loaded, "docs", ["sub/NOTES.md"]),
                      loaded["profiles"]["docs"])


class VerifyTest(CliCase):
    def test_verify_recomputes_executable_bits(self):
        self.run_profile("docs", base=self.base)
        with open(self.path("out/bundle.json"), "rb") as handle:
            doc = json.loads(handle.read())
        doc["changed_executable"] = ["b.md"]
        data = bundle.canonical(doc)
        with open(self.path("out/bundle.json"), "wb") as handle:
            handle.write(data)
        with open(self.path("out/bundle.json.sig"), "wb") as handle:
            handle.write(signing.sign(self.key, data))
        code, doc = self.verify(profile="docs", base=self.base)
        self.assertEqual((code, doc["reasons"]), (2, [
            "documentation-only profile refused: not a documentation "
            "change: b.md"]))

    def test_removing_an_executable_bit_is_not_documentation(self):
        self.repo.write("tool.md", b"#!/bin/sh\n", executable=True)
        base = self.repo.commit()
        self.repo.git("update-index", "--chmod=-x", "tool.md")
        self.repo.commit()
        code, doc = self.run_profile("docs", base=base)
        self.assertEqual((code, doc["blocked"]), (2, [
            "documentation-only profile refused: not a documentation "
            "change: tool.md"]))


if __name__ == "__main__":
    unittest.main()
