"""Exact boundaries and messages for the hardened checker (mutation gaps).

Retrofit: written after mutation testing of the hardening changes showed
unpinned boundaries, messages and report keys.
"""
import io
import json
import os
import tempfile
import unittest

from tools.limits import cli, counting, functions, policy

V = counting.Violation


class BoundaryTest(unittest.TestCase):
    def test_bare_address_line_gets_the_link_allowance(self):
        line = "https://example.org/" + "c" * 130
        self.assertEqual(policy.check_file("a.md", line + "\n"), [])

    def test_text_beside_address_of_exactly_80_is_allowed(self):
        url = "https://example.org/" + "d" * 40
        line = "[" + "t" * 76 + "](" + url + ")"
        self.assertEqual(len(line) - len(url), 80)
        self.assertEqual(policy.check_file("a.md", line + "\n"), [])

    def test_c1_boundary_and_no_break_space(self):
        self.assertEqual([v.value for v in counting.check_text(
            "a.md", "a" + chr(0x9F) + "\n")], ["U+009F"])
        self.assertEqual(counting.check_text("a.md", "a" + chr(0xA0)), [])

    def test_exactly_the_cap_reports_no_summary(self):
        found = counting.check_text("a.md", "\t\n" * counting.MAX_REPORTED)
        self.assertEqual(len(found), counting.MAX_REPORTED)
        self.assertNotIn("more_violations", [v.kind for v in found])

    def test_file_size_boundary_and_violation(self):
        limit = counting.MAX_FILE_CHARS
        self.assertNotIn("file_size", [v.kind for v in counting.check_text(
            "a.md", ("x" * 79 + "\n") * (limit // 80))])
        self.assertEqual(counting.check_text("a.md", "x" * (limit + 1)),
                         [V("a.md", None, "file_size", limit + 1)])

    def test_null_byte_in_python_is_reported_not_raised(self):
        found = functions.check_python("a.py", "x = 1\0\n")
        self.assertEqual([(f.path, f.kind) for f in found],
                         [("a.py", "parse_error")])

    def test_non_ascii_text_with_forbidden_ascii(self):
        self.assertEqual([v.value for v in counting.check_text(
            "a.md", "é\x01\n")], ["U+0001"])


class MessageTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = self.tmp.name
        self.put("tools/limits/classes.txt", "text *.md\n")

    def tearDown(self):
        self.tmp.cleanup()

    def put(self, rel, text):
        path = os.path.join(self.root, rel)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w", encoding="utf-8") as handle:
            handle.write(text)

    def run_cli(self, *args):
        out = io.StringIO()
        code = cli.main([self.root, *args], out)
        return code, json.loads(out.getvalue())

    def test_unsafe_path_reasons_are_exact(self):
        self.put("a.md", "ok\n")
        cases = {"docs\\a.md": "unsafe path", "a//a.md": "unsafe path",
                 "../a.md": "unsafe path", "./a.md": "unsafe path",
                 "/a.md": "unsafe path", "absent.md": "missing"}
        for rel, reason in cases.items():
            with self.subTest(rel=rel):
                _, rep = self.run_cli(rel)
                self.assertEqual(rep["blocked"], [f"{rel}: {reason}"])

    @unittest.skipUnless(hasattr(os, "symlink"), "needs symlinks")
    def test_symlink_reason_is_exact(self):
        self.put("real.md", "ok\n")
        os.symlink(os.path.join(self.root, "real.md"),
                   os.path.join(self.root, "link.md"))
        _, rep = self.run_cli("link.md")
        self.assertEqual(rep["blocked"], ["link.md: symbolic link"])

    def test_no_paths_message_and_class_option_without_paths(self):
        protected = os.path.join(self.root, "protected.txt")
        with open(protected, "w", encoding="utf-8") as handle:
            handle.write("text *.md\n")
        _, rep = self.run_cli("--classes", protected)
        self.assertEqual(rep["blocked"], ["no paths given"])
        self.assertRegex(rep["classes_sha256"], r"^[0-9a-f]{64}$")

    def test_missing_class_list_reports_no_hash(self):
        os.remove(os.path.join(self.root, "tools/limits/classes.txt"))
        _, rep = self.run_cli("a.md")
        self.assertIsNone(rep["classes_sha256"])

    def test_internal_error_report_is_exact(self):
        original = policy.check_file
        policy.check_file = lambda path, text: 1 / 0
        self.put("a.md", "ok\n")
        try:
            _, rep = self.run_cli("a.md")
        finally:
            policy.check_file = original
        self.assertEqual(rep, {"rule": cli.RULE, "status": "BLOCKED",
                               "blocked": ["internal error: "
                                           "ZeroDivisionError"]})


class ScopeWalkTest(unittest.TestCase):
    def test_sibling_functions_and_methods_are_named_separately(self):
        src = ("def f():\n    pass\n\n\ndef g():\n    pass\n\n\n"
               "class C:\n    def a(self):\n        pass\n\n"
               "    def b(self):\n        pass\n")
        self.assertEqual([s[0] for s in functions.python_spans(src)],
                         ["f", "g", "C.a", "C.b"])

    def test_lambda_free_files_never_walk_expressions(self):
        import ast
        original = ast.walk
        ast.walk = lambda node: (_ for _ in ()).throw(AssertionError())
        try:
            spans = functions.python_spans("def f():\n    return 1\n")
        finally:
            ast.walk = original
        self.assertEqual(spans, [("f", 1, 2)])

    def test_files_with_lambdas_use_the_full_walk(self):
        src = "x = [1]\ny = sorted(x, key=lambda v: v)\n"
        self.assertEqual(functions.python_spans(src),
                         [("<lambda>", 2, 2)])


if __name__ == "__main__":
    unittest.main()
