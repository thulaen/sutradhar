"""Retrofit (mutation closure): runner command details mutants exposed."""
import contextlib
import datetime
import io
import json
import os
import runpy
import stat
import sys
import unittest
from unittest import mock

from tools.runner import candidate, cli, orchestrate
from tools.runner.tests.test_cli import CliCase


class FactsTest(CliCase):
    def test_bundle_records_every_fact(self):
        code, doc = self.run_profile()
        self.assertEqual(code, 0)
        self.assertEqual(doc["started"], "2026-10-30T12:00:00Z")
        self.assertEqual(doc["policy_version"], "t1")
        self.assertEqual(doc["profile"], "commit")
        self.assertEqual(doc["tools"], self.policy_doc["tools"])
        self.assertEqual(doc["tools"]["limits"],
                         candidate.path_digest(self.tool))
        self.assertEqual(sorted(doc["environment"]),
                         ["binaries", "machine", "python", "system"])
        self.assertEqual(doc["runner_sha256"], orchestrate.runner_digest())

    def test_output_folders_are_private(self):
        self.run_profile()
        modes = [stat.S_IMODE(os.stat(self.path(name)).st_mode)
                 for name in ("out", "out/private")]
        self.assertEqual(modes, [0o750, 0o700])


class OptionsTest(CliCase):
    def verify_args(self):
        return ["verify", "--bundle", "b", "--signature", "s",
                "--signers", "n", "--identity", "i", "--policy", "p",
                "--tree", "t", "--profile", "commit"]

    def test_options_need_two_dashes(self):
        code, doc = self.main("run", "--repo", "r", "xxtree", "t")
        self.assertEqual((code, doc["blocked"]),
                         (2, ["usage: unknown or repeated option xxtree"]))

    def test_repeated_options_are_refused(self):
        code, doc = self.main(*self.verify_args(), "--bundle", "c")
        self.assertEqual((code, doc["blocked"]),
                         (2, ["usage: unknown or repeated option --bundle"]))

    def test_missing_options_are_named(self):
        code, doc = self.main("run")
        self.assertEqual(doc["blocked"], [
            "usage: run needs --repo --tree --profile --policy --host "
            "--key --out"])

    def test_report_is_indented_and_sorted(self):
        out = io.StringIO()
        cli.main([], out)
        self.assertEqual(out.getvalue(), '{\n "blocked": [\n  "usage: '
                         'run|verify --option value ..."\n ],\n "status": '
                         '"BLOCKED"\n}')


class MoreVerifyAndUsageTest(CliCase):
    def test_verify_option_errors_block(self):
        """Retrofit (coverage closure)."""
        self.run_profile()
        base = ["verify", "--bundle", self.path("out/bundle.json"),
                "--signature", self.path("out/bundle.json.sig"),
                "--signers", self.path("signers"),
                "--identity", "sutradhar-runner",
                "--policy", self.path("policy.json"), "--tree", self.tree,
                "--message", "none", "--base", "none"]
        code, doc = self.main(*base, "--profile", "release")
        self.assertEqual((code, doc["blocked"]),
                         (2, ["unknown profile: release"]))
        code, doc = self.main(*base[:-2], "--profile", "commit",
                              "--base", self.base)
        self.assertEqual((code, doc["reasons"]),
                         (2, ["base differs from the expected value"]))

    def test_missing_tool_path_blocks(self):
        """Retrofit (coverage closure)."""
        self.write_json("host.json", {"prefix": [], "tools": {
            "limits": self.path("missing"), "harness": self.harness}})
        code, doc = self.run_profile()
        self.assertEqual((code, doc["blocked"]),
                         (2, ["tool not installed: limits"]))

    def test_defaults_and_module_entry(self):
        """Retrofit (coverage closure)."""
        self.assertEqual(cli._utc_now().utcoffset(),
                         datetime.timedelta(0))
        self.assertRegex(cli._token(), "^[0-9a-f]{16}$")
        captured = io.StringIO()
        with mock.patch.object(sys, "argv", ["cli"]), \
                contextlib.redirect_stdout(captured), \
                self.assertRaises(SystemExit) as stop:
            runpy.run_module("tools.runner.cli", run_name="__main__")
        self.assertEqual(stop.exception.code, 2)
        self.assertEqual(json.loads(captured.getvalue())["status"],
                         "BLOCKED")

    def test_usage_errors_block(self):
        for args in ([], ["run"], ["frob"], ["run", "--repo"],
                     ["verify", "--bundle", "x", "--bundle", "y"],
                     ["run", "--colour", "x"]):
            code, doc = self.main(*args)
            self.assertEqual((code, doc["status"]), (2, "BLOCKED"), args)
            self.assertTrue(doc["blocked"][0].startswith("usage"), args)


if __name__ == "__main__":
    unittest.main()
