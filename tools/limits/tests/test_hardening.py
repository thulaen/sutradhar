"""Findings of the independent review of 30 October 2026, written first.

Each test captures one review finding and failed before the fix.
"""
import io
import json
import os
import tempfile
import unittest

from tools.limits import cli, counting, functions, policy

V = counting.Violation
SRC = "docs/requirements/sources/interview-q001-q009.md"


def kinds(path, text):
    return [v.kind for v in policy.check_file(path, text)]


class LinkAllowanceTest(unittest.TestCase):
    def test_prose_ending_in_a_link_tail_gets_no_allowance(self):
        self.assertEqual(kinds("a.md", "word " * 38 + "](u)\n"),
                         ["line_length"])

    def test_code_comment_tail_gets_no_allowance(self):
        line = "total = alpha + beta + gamma + delta  # " + "x" * 50 + "](u)"
        self.assertEqual(kinds("a.py", line + "\n"), ["line_length"])

    def test_link_text_beside_the_address_is_limited_to_80(self):
        url = "https://example.org/" + "b" * 60
        self.assertEqual(kinds("a.md", f"[{'t' * 79}]({url})\n"),
                         ["line_length"])
        self.assertEqual(kinds("a.md", f"[{'t' * 70}]({url})\n"), [])


class FenceRuleTest(unittest.TestCase):
    def test_backtick_info_with_backtick_does_not_open(self):
        text = "```x```\n" + "q" * 90 + "\n"
        self.assertEqual(kinds(SRC, text), ["line_length"])

    def test_tilde_info_may_contain_backticks(self):
        self.assertEqual(kinds(SRC, "~~~ a`b\n" + "q" * 90 + "\n~~~\n"), [])

    def test_shorter_or_other_mark_does_not_close(self):
        text = "`````\n```\n~~~~~\n" + "q" * 90 + "\n`````\n" + "p" * 81
        self.assertEqual([(v.line, v.kind) for v in
                          policy.check_file(SRC, text)],
                         [(6, "line_length")])

    def test_info_string_on_closing_fence_does_not_close(self):
        text = "```\n```text\n" + "q" * 90 + "\n```\n"
        self.assertEqual(kinds(SRC, text), [])


class ForbiddenSetTest(unittest.TestCase):
    CODES = (0x00, 0x08, 0x0B, 0x7F, 0x85, 0x9B, 0xAD, 0x061C, 0x200E,
             0x200F, 0x2028, 0x2029, 0xE0001)

    def test_controls_and_format_characters_are_refused(self):
        for code in self.CODES:
            with self.subTest(code=hex(code)):
                self.assertEqual(kinds("a.md", "a" + chr(code) + "b\n"),
                                 ["forbidden_character"])

    def test_ordinary_text_including_cr_and_lf_is_allowed(self):
        self.assertEqual(kinds("a.md", "café – naïve “quotes”\r\nok\n"), [])


class ResourceTest(unittest.TestCase):
    def test_oversized_file_is_refused_before_scanning(self):
        found = policy.check_file("a.md", "x\n" * 600_000)
        self.assertEqual(found[0].kind, "file_size")

    def test_violations_per_file_are_capped_with_a_total(self):
        found = counting.check_text("a.md", "\t\n" * 120)
        self.assertEqual(len(found), counting.MAX_REPORTED + 1)
        self.assertEqual(found[-1], V("a.md", None, "more_violations", 70))

    def test_deep_nesting_never_raises(self):
        src = "x = (" + " + ".join(["1"] * 20000) + ")\n"
        found = functions.check_python("a.py", src)
        # Python 3.14 parses this; the iterative walk then finds nothing.
        self.assertEqual(found, [])

    def test_parser_limits_are_reported(self):
        import ast
        original = ast.parse
        ast.parse = lambda source: (_ for _ in ()).throw(RecursionError())
        try:
            found = functions.check_python("a.py", "x = 1\n")
        finally:
            ast.parse = original
        self.assertEqual(found, [V("a.py", None, "parse_limit",
                                   "parser resource limit")])


class RepoCase(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = self.tmp.name
        self.put("tools/limits/classes.txt",
                 "third_party vendor/**\ntext *.md\ndata *.yaml\n")

    def tearDown(self):
        self.tmp.cleanup()

    def put(self, rel, text):
        path = os.path.join(self.root, rel)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w", encoding="utf-8", newline="") as handle:
            handle.write(text)

    def run_cli(self, *args):
        out = io.StringIO()
        code = cli.main([self.root, *args], out)
        return code, json.loads(out.getvalue())


class CliHardeningTest(RepoCase):
    def test_no_paths_is_blocked_not_passed(self):
        code, rep = self.run_cli()
        self.assertEqual((code, rep["status"]), (2, "BLOCKED"))

    def test_report_records_the_class_list_hash(self):
        self.put("a.md", "ok\n")
        _, rep = self.run_cli("a.md")
        self.assertRegex(rep["classes_sha256"], r"^[0-9a-f]{64}$")

    def test_class_list_can_come_from_a_protected_copy(self):
        self.put("a.md", "x" * 81 + "\n")
        self.put("tools/limits/classes.txt", "binary *.md\n")
        protected = os.path.join(self.root, "protected.txt")
        with open(protected, "w", encoding="utf-8") as handle:
            handle.write("text *.md\n")
        code, rep = self.run_cli("--classes", protected, "a.md")
        self.assertEqual((code, rep["status"]), (1, "FAILED"))

    def test_paths_escaping_the_root_are_blocked(self):
        self.put("a.md", "ok\n")
        for bad in ("/etc/passwd", "../a.md", "docs/../a.md", "./a.md",
                    "docs\\a.md"):
            with self.subTest(path=bad):
                code, rep = self.run_cli(bad)
                self.assertEqual((code, rep["status"]), (2, "BLOCKED"))

    @unittest.skipUnless(hasattr(os, "symlink"), "needs symlinks")
    def test_symlinks_are_blocked(self):
        self.put("real.md", "ok\n")
        os.symlink(os.path.join(self.root, "real.md"),
                   os.path.join(self.root, "link.md"))
        code, rep = self.run_cli("link.md")
        self.assertEqual((code, rep["status"]), (2, "BLOCKED"))

    def test_yaml_is_measured(self):
        self.put("d.yaml", "k: " + "v" * 90 + "\n")
        code, rep = self.run_cli("d.yaml")
        self.assertEqual(rep["violations"][0]["kind"], "line_length")

    def test_third_party_is_inventoried_separately(self):
        self.put("vendor/lib/x.md", "y" * 500 + "\n")
        code, rep = self.run_cli("vendor/lib/x.md")
        self.assertEqual((code, rep["inventory"]),
                         (0, {"third_party": ["vendor/lib/x.md"]}))

    def test_unexpected_errors_block_with_a_report(self):
        self.put("a.md", "ok\n")
        original = policy.check_file
        policy.check_file = lambda path, text: 1 / 0
        try:
            code, rep = self.run_cli("a.md")
        finally:
            policy.check_file = original
        self.assertEqual((code, rep["status"]), (2, "BLOCKED"))


if __name__ == "__main__":
    unittest.main()
