"""Exact report contents (added after mutation testing showed gaps).

Retrofit: these tests pin paths, line numbers, values and messages that
the first tests left unchecked, so a fault in any of them is detected.
"""
import io
import json
import os
import tempfile
import unittest

from tools.limits import cli, counting, functions, policy

V = counting.Violation


class CountingContractTest(unittest.TestCase):
    def test_forbidden_character_violation_is_complete(self):
        self.assertEqual(counting.check_text("a.md", "x\ta\n"),
                         [V("a.md", 1, "forbidden_character", "U+0009")])

    def test_file_lines_violation_is_complete(self):
        self.assertEqual(counting.check_text("a.md", "x\n" * 301),
                         [V("a.md", None, "file_lines", 301)])

    def test_decode_error_names_the_byte(self):
        with self.assertRaisesRegex(ValueError, r"^not UTF-8 at byte 1$"):
            counting.decode(b"a\xff")


class PolicyContractTest(unittest.TestCase):
    def test_exactly_300_lines_pass_and_301_report_count(self):
        self.assertEqual(policy.check_file("a.md", "x\n" * 300), [])
        self.assertEqual(policy.check_file("a.md", "x\n" * 301),
                         [V("a.md", None, "file_lines", 301)])

    def test_line_numbers_start_at_one(self):
        found = policy.check_file("a.md", "ok\n" + "y" * 81 + "\n")
        self.assertEqual(found, [V("a.md", 2, "line_length", 81)])

    def test_class_errors_name_line_and_class(self):
        with self.assertRaisesRegex(ValueError,
                                    r"^line 2: unknown class magic$"):
            policy.load_classes("text *.md\nmagic *.x\n")
        with self.assertRaisesRegex(ValueError,
                                    r"^line 1: expected 'class pattern'$"):
            policy.load_classes("text\n")

    def test_recursive_globs_cross_several_directories(self):
        classes = policy.load_classes("text tools/**/*.py\n")
        self.assertEqual(policy.classify("tools/a/b/c/d.py", classes),
                         "text")

    def test_hidden_directories_are_matched(self):
        classes = policy.load_classes("text docs/**\n")
        self.assertEqual(policy.classify("docs/.private/a.md", classes),
                         "text")


class FunctionContractTest(unittest.TestCase):
    def test_lambda_inside_function_is_qualified(self):
        src = "def f():\n    g = lambda: 1\n    return g\n"
        self.assertIn(("f.<lambda>", 2, 2), functions.python_spans(src))

    def test_lambda_inside_lambda_is_listed(self):
        src = "def f():\n    return lambda: (lambda: 1)\n"
        self.assertEqual(functions.python_spans(src),
                         [("f", 1, 2), ("f.<lambda>", 2, 2),
                          ("f.<lambda>", 2, 2)])

    def test_parse_error_names_path_and_line(self):
        found = functions.check_python("a.py", "x = 1\ndef (:\n")
        self.assertEqual([(f.path, f.line, f.kind) for f in found],
                         [("a.py", 2, "parse_error")])
        self.assertTrue(found[0].value)

    def test_function_violation_is_complete(self):
        src = "def f():\n" + "    x = 1\n" * 61
        self.assertEqual(functions.check_source("t.py", src),
                         [V("t.py", 1, "function_lines", 62)])

    def test_php_violation_is_complete(self):
        self.assertEqual(
            functions.check_source("a.php", "<?php\n"),
            [V("a.php", None, "unsupported_language",
               "no admitted PHP parser")])


class CliContractTest(unittest.TestCase):
    CLASSES = "text *.md\ntext *.py\ntext *.php\n"

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = self.tmp.name
        self.put("tools/limits/classes.txt", self.CLASSES.encode())

    def tearDown(self):
        self.tmp.cleanup()

    def put(self, rel, data):
        path = os.path.join(self.root, rel)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "wb") as handle:
            handle.write(data)

    def run_cli(self, *paths):
        out = io.StringIO()
        code = cli.main([self.root, *paths], out)
        return code, out.getvalue()

    def test_output_is_sorted_json_with_one_space_indent(self):
        self.put("a.md", b"ok\n")
        _, text = self.run_cli("a.md")
        self.assertEqual(text, json.dumps(json.loads(text), indent=1,
                                          sort_keys=True))
        self.assertIn('\n "checked": 1,', text)

    def test_every_file_is_counted_and_reported_by_path(self):
        self.put("a.md", b"ok\n")
        self.put("x.exe", b"\x00")
        self.put("b.md", b"\xff")
        self.put("t.py", ("def f():\n" + "    x = 1\n" * 61).encode())
        _, text = self.run_cli("a.md", "x.exe", "b.md", "t.py")
        rep = json.loads(text)
        self.assertEqual(rep["checked"], 3)
        self.assertEqual([(v["path"], v["kind"]) for v in rep["violations"]],
                         [("x.exe", "unclassified"), ("b.md", "not_utf8"),
                          ("t.py", "function_lines")])
        self.assertEqual(rep["violations"][1]["value"],
                         "not UTF-8 at byte 0")

    def test_missing_file_does_not_stop_later_checks(self):
        self.put("a.md", b"ok\n")
        _, text = self.run_cli("absent.md", "a.md")
        rep = json.loads(text)
        self.assertEqual((rep["blocked"], rep["checked"]),
                         (["absent.md: missing"], 1))

    def test_blocked_messages_are_specific(self):
        self.put("a.php", b"<?php\n")
        _, text = self.run_cli("a.php")
        self.assertEqual(json.loads(text)["blocked"],
                         ["a.php: no admitted PHP parser"])
        os.remove(os.path.join(self.root, "tools/limits/classes.txt"))
        _, text = self.run_cli("a.php")
        self.assertTrue(json.loads(text)["blocked"][0].startswith(
            "tools/limits/classes.txt: "))

    def test_usage_report_is_exact(self):
        out = io.StringIO()
        cli.main([], out)
        self.assertEqual(json.loads(out.getvalue()),
                         {"status": "BLOCKED",
                          "blocked": ["usage: ROOT PATH..."]})


if __name__ == "__main__":
    unittest.main()
