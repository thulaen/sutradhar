"""Command-line entry point and repository walk (SPEC-007, AT-T08)."""
import io
import json
import os
import tempfile
import unittest

from tools.limits import cli

CLASSES = "text *.md\ntext *.py\nbinary *.png\nlockfile uv.lock\n"


class RepoCase(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = self.tmp.name
        self.write("tools/limits/classes.txt", CLASSES)

    def tearDown(self):
        self.tmp.cleanup()

    def write(self, rel, data):
        path = os.path.join(self.root, rel)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        mode = "wb" if isinstance(data, bytes) else "w"
        with open(path, mode, **({} if mode == "wb" else
                                 {"encoding": "utf-8", "newline": ""})) as f:
            f.write(data)

    def run_cli(self, paths):
        out = io.StringIO()
        code = cli.main([self.root, *paths], out)
        return code, json.loads(out.getvalue())


class CliTest(RepoCase):
    def test_clean_files_pass(self):
        self.write("a.md", "ok\n")
        code, rep = self.run_cli(["a.md"])
        self.assertEqual((code, rep["status"]), (0, "PASSED"))

    def test_violation_fails_with_details(self):
        self.write("a.md", "x" * 81 + "\n")
        code, rep = self.run_cli(["a.md"])
        self.assertEqual((code, rep["status"]), (1, "FAILED"))
        self.assertEqual(rep["violations"][0]["kind"], "line_length")

    def test_unclassified_path_fails(self):
        self.write("x.exe", b"\x00")
        code, rep = self.run_cli(["x.exe"])
        self.assertEqual(rep["violations"][0]["kind"], "unclassified")
        self.assertEqual(code, 1)

    def test_binary_and_lockfile_are_inventoried_not_measured(self):
        self.write("i.png", b"\x89PNG" + b"x" * 500)
        self.write("uv.lock", "y" * 500 + "\n")
        code, rep = self.run_cli(["i.png", "uv.lock"])
        self.assertEqual(code, 0)
        self.assertEqual(rep["inventory"],
                         {"binary": ["i.png"], "lockfile": ["uv.lock"]})

    def test_undecodable_text_fails(self):
        self.write("a.md", b"\xff\n")
        code, rep = self.run_cli(["a.md"])
        self.assertEqual(rep["violations"][0]["kind"], "not_utf8")

    def test_php_source_blocks(self):
        self.write("tools/limits/classes.txt", CLASSES + "text *.php\n")
        self.write("a.php", "<?php\n")
        code, rep = self.run_cli(["a.php"])
        self.assertEqual((code, rep["status"]), (2, "BLOCKED"))

    def test_missing_classes_file_blocks(self):
        os.remove(os.path.join(self.root, "tools/limits/classes.txt"))
        self.write("a.md", "ok\n")
        code, rep = self.run_cli(["a.md"])
        self.assertEqual((code, rep["status"]), (2, "BLOCKED"))

    def test_missing_listed_file_blocks(self):
        code, rep = self.run_cli(["absent.md"])
        self.assertEqual((code, rep["status"]), (2, "BLOCKED"))

    def test_report_names_rule_version(self):
        self.write("a.md", "ok\n")
        _, rep = self.run_cli(["a.md"])
        self.assertEqual(rep["rule"], "SPEC-007 source limits, 2026-10-30")
        self.assertEqual(rep["checked"], 1)


class UsageTest(unittest.TestCase):
    def test_no_arguments_is_a_usage_error(self):
        out = io.StringIO()
        self.assertEqual(cli.main([], out), 2)

    def test_module_entry_point_exits_with_main_status(self):
        # Retrofit: written after the entry point to close a coverage gap.
        import contextlib
        import runpy
        import sys
        argv = sys.argv
        sys.argv = ["cli"]
        try:
            with contextlib.redirect_stdout(io.StringIO()), \
                    self.assertRaises(SystemExit) as stop:
                runpy.run_module("tools.limits.cli", run_name="__main__")
        finally:
            sys.argv = argv
        self.assertEqual(stop.exception.code, 2)


if __name__ == "__main__":
    unittest.main()
