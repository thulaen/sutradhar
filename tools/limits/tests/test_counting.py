"""Physical-line counting and per-line checks (SPEC-007, AT-T04 to T07)."""
import unittest

from tools.limits import counting


class SplitLinesTest(unittest.TestCase):
    def test_empty_text_has_no_lines(self):
        self.assertEqual(counting.split_lines(""), [])

    def test_final_newline_does_not_add_a_line(self):
        self.assertEqual(counting.split_lines("a\nb\n"), ["a", "b"])

    def test_missing_final_newline_still_counts_last_line(self):
        self.assertEqual(counting.split_lines("a\nb"), ["a", "b"])

    def test_crlf_counts_once(self):
        self.assertEqual(counting.split_lines("a\r\nb\r\n"), ["a", "b"])

    def test_lone_cr_is_a_line_break(self):
        self.assertEqual(counting.split_lines("a\rb\rc"), ["a", "b", "c"])

    def test_blank_lines_count(self):
        self.assertEqual(counting.split_lines("\n\n"), ["", ""])


class DecodeTest(unittest.TestCase):
    def test_utf8_is_decoded(self):
        self.assertEqual(counting.decode("é".encode()), "é")

    def test_invalid_utf8_is_rejected(self):
        with self.assertRaises(ValueError):
            counting.decode(b"\xff\xfe")


class FileLimitTest(unittest.TestCase):
    def kinds(self, text, path="docs/x.md"):
        return [v.kind for v in counting.check_text(path, text)]

    def test_300_lines_without_final_newline_passes(self):
        self.assertEqual(self.kinds("x\n" * 299 + "x"), [])

    def test_301_lines_fail(self):
        self.assertEqual(self.kinds("x\n" * 301), ["file_lines"])

    def test_trailing_blank_line_counts(self):
        self.assertEqual(self.kinds("x\n" * 300 + "\n"), ["file_lines"])

    def test_81_code_points_fail(self):
        self.assertEqual(self.kinds("a" * 81 + "\n"), ["line_length"])

    def test_80_wide_characters_pass(self):
        self.assertEqual(self.kinds("界" * 80 + "\n"), [])

    def test_tab_is_refused(self):
        self.assertEqual(self.kinds("a\tb\n"), ["forbidden_character"])

    def test_bidi_control_is_refused(self):
        self.assertEqual(self.kinds("a\u202eb\n"), ["forbidden_character"])

    def test_zero_width_space_is_refused(self):
        self.assertEqual(self.kinds("a\u200bb\n"), ["forbidden_character"])

    def test_cr_only_breaks_cannot_hide_lines(self):
        self.assertEqual(self.kinds("x\r" * 301), ["file_lines"])

    def test_violation_reports_line_and_value(self):
        v = counting.check_text("p.md", "ok\n" + "b" * 90 + "\n")[0]
        self.assertEqual((v.path, v.line, v.kind, v.value),
                         ("p.md", 2, "line_length", 90))


if __name__ == "__main__":
    unittest.main()
