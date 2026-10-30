"""Findings of the second review pass, 30 October 2026, written first."""
import io
import json
import os
import tempfile
import unittest

from tools.limits import cli, counting, functions, policy

SRC = "docs/requirements/sources/interview-q001-q009.md"


class InvisibleCharacterTest(unittest.TestCase):
    CODES = (0x034F, 0x115F, 0x1160, 0x17B4, 0x180B, 0x180F, 0x3164,
             0xFE00, 0xFE0F, 0xFFA0, 0x1D173, 0xE0100, 0xE01EF)

    def test_default_ignorable_characters_are_refused(self):
        for code in self.CODES:
            with self.subTest(code=hex(code)):
                found = counting.check_text("a.md", "a" + chr(code) + "\n")
                self.assertEqual([v.kind for v in found],
                                 ["forbidden_character"])

    def test_visible_neighbours_are_allowed(self):
        for code in (0x034E, 0x1161, 0xFE10, 0xE01F0 - 0x10000):
            with self.subTest(code=hex(code)):
                self.assertEqual(
                    counting.check_text("a.md", "a" + chr(code) + "\n"), [])


class QuotedLookupTest(unittest.TestCase):
    def test_many_blocks_are_looked_up_correctly(self):
        blocks = "".join("```\n" + "q" * 90 + "\n```\n" + "p" * 81 + "\n"
                         for _ in range(40))
        found = policy.check_file(SRC, blocks)
        self.assertEqual([v.line for v in found if v.kind == "line_length"],
                         [4 + 4 * i for i in range(40)])


class RepoCase(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = self.tmp.name
        self.put("tools/limits/classes.txt", b"text *.md\ntext *.py\n")

    def tearDown(self):
        self.tmp.cleanup()

    def put(self, rel, data):
        path = os.path.join(self.root, rel)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "wb") as handle:
            handle.write(data)

    def run_cli(self, *args):
        out = io.StringIO()
        code = cli.main([self.root, *args], out)
        return code, json.loads(out.getvalue())


class SizeCapTest(RepoCase):
    def test_huge_file_is_refused_by_size_before_reading(self):
        limit = 4 * counting.MAX_FILE_CHARS
        self.put("a.md", b"x" * (limit + 1))
        _, rep = self.run_cli("a.md")
        self.assertEqual(rep["violations"],
                         [{"path": "a.md", "line": None,
                           "kind": "file_size", "value": limit + 1}])

    def test_oversized_python_is_not_parsed(self):
        self.put("a.py", b"x = 1\n" * 200_000)
        original = functions.check_source
        functions.check_source = lambda path, text: 1 / 0
        try:
            _, rep = self.run_cli("a.py")
        finally:
            functions.check_source = original
        self.assertEqual([v["kind"] for v in rep["violations"]],
                         ["file_size"])


class CombinedCapTest(RepoCase):
    def test_text_and_function_violations_share_one_cap(self):
        long_fn = "def f{}():\n" + "    x = 1\n" * 61
        src = "".join(long_fn.format(i) for i in range(30))
        src += "".join("y = '" + "z" * 80 + "'\n" for _ in range(30))
        self.put("a.py", src.encode())
        _, rep = self.run_cli("a.py")
        kinds = [v["kind"] for v in rep["violations"]]
        self.assertEqual(len(kinds), counting.MAX_REPORTED + 1)
        self.assertEqual(kinds[-1], "more_violations")


class DecisiveRuleTest(unittest.TestCase):
    def test_format_characters_outside_the_table_are_refused(self):
        for code in (0x0600, 0xFFF9, 0x110BD):
            with self.subTest(code=hex(code)):
                found = counting.check_text("a.md", "a" + chr(code) + "\n")
                self.assertEqual([v.kind for v in found],
                                 ["forbidden_character"])

    def test_first_table_entry_is_refused(self):
        found = counting.check_text("a.md", "a" + chr(0x034F) + "\n")
        self.assertEqual([v.value for v in found], ["U+034F"])


class ByteBoundaryTest(RepoCase):
    def test_exactly_the_byte_limit_is_read_and_measured(self):
        wide = chr(0x1F600)  # four bytes in UTF-8, an ordinary emoji
        self.put("a.md", (wide * counting.MAX_FILE_CHARS).encode())
        _, rep = self.run_cli("a.md")
        self.assertEqual([v["kind"] for v in rep["violations"]],
                         ["line_length"])

    def test_capped_summary_names_the_file(self):
        src = "".join("y = '" + "z" * 80 + "'\n" for _ in range(60))
        self.put("a.py", src.encode())
        _, rep = self.run_cli("a.py")
        self.assertEqual(rep["violations"][-1]["path"], "a.py")


if __name__ == "__main__":
    unittest.main()