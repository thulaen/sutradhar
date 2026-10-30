"""Whole-text scanning edge cases (added before the performance rewrite)."""
import unittest

from tools.limits import counting, policy

V = counting.Violation
SRC = "docs/requirements/sources/interview-q001-q009.md"


def quoted(norm):
    return set().union(*policy.quoted_ranges(norm))


class ScanOrderTest(unittest.TestCase):
    def test_one_forbidden_violation_per_line(self):
        self.assertEqual(counting.check_text("a.md", "\t\t\u200b\n"),
                         [V("a.md", 1, "forbidden_character", "U+0009")])

    def test_forbidden_comes_before_length_on_the_same_line(self):
        self.assertEqual(
            counting.check_text("a.md", "\t" + "x" * 85 + "\n"),
            [V("a.md", 1, "forbidden_character", "U+0009"),
             V("a.md", 1, "line_length", 86)])

    def test_violations_are_in_line_order(self):
        text = "y" * 90 + "\nok\n\tz\n"
        self.assertEqual([v.line for v in counting.check_text("a.md", text)],
                         [1, 3])

    def test_crlf_and_cr_lines_are_numbered_correctly(self):
        text = "a\r\nb\rc\n" + "d" * 81
        self.assertEqual(counting.check_text("a.md", text),
                         [V("a.md", 4, "line_length", 81)])

    def test_many_long_lines_are_each_reported(self):
        text = ("z" * 81 + "\n") * 3
        self.assertEqual([v.line for v in counting.check_text("a.md", text)],
                         [1, 2, 3])


class HelperTest(unittest.TestCase):
    def test_several_forbidden_kinds_do_not_match_ordinary_text(self):
        text = "aX\nb\t\u200bc\n"
        self.assertEqual(counting.check_text("a.md", text),
                         [V("a.md", 2, "forbidden_character", "U+0009")])


    def test_normalise_writes_every_break_as_lf(self):
        self.assertEqual(counting.normalise("a\r\nb\rc\nd"), "a\nb\nc\nd")

    def test_line_count(self):
        self.assertEqual([counting.line_count(t) for t in
                          ("", "a", "a\n", "a\nb", "\n\n")], [0, 1, 1, 2, 2])

    def test_numbering_counts_a_leading_blank_line(self):
        self.assertEqual(counting.check_text("a.md", "\n" + "x" * 81),
                         [V("a.md", 2, "line_length", 81)])

    def test_long_line_found_at_end_of_text(self):
        self.assertEqual(counting.check_text("a.md", "a\n" + "x" * 81),
                         [V("a.md", 2, "line_length", 81)])

    def test_quoted_lines_are_exactly_the_inner_lines(self):
        norm = "a\n```\nb\nc\n```\nd\n"
        self.assertEqual(quoted(norm), {3, 4})

    def test_unclosed_quotation_runs_to_the_last_line(self):
        self.assertEqual(quoted("x\n```\nb\nc"), {3, 4})

    def test_no_fences_means_no_quoted_lines(self):
        self.assertEqual(quoted("a\nb\n"), set())

    def test_fence_on_the_first_line(self):
        self.assertEqual(quoted("```\nb\n```\n"), {2})

    def test_up_to_three_leading_spaces_make_a_fence(self):
        self.assertEqual(quoted("   ```\nb\n   ```\n"), {2})

    def test_four_leading_spaces_are_not_a_fence(self):
        self.assertEqual(quoted("    ```\nb\n    ```\n"),
                         set())

    def test_two_blocks_are_numbered_independently(self):
        norm = "```\na\n```\nx\n~~~\nb\nc\n~~~\n"
        self.assertEqual(quoted(norm), {2, 6, 7})

    def test_forbidden_characters_on_later_lines_are_numbered(self):
        text = "a\n\nb\tc\nd\n\te\n"
        self.assertEqual([v.line for v in counting.check_text("a.md", text)],
                         [3, 5])

    def test_long_lines_report_exact_text_lengths(self):
        text = "s\n" + "x" * 81 + "\nshort\n" + "y" * 95 + "\n"
        self.assertEqual(list(counting.long_lines(counting.normalise(text))),
                         [(2, "x" * 81), (4, "y" * 95)])


class QuotedRegionTest(unittest.TestCase):
    def test_unclosed_fence_exempts_to_end_of_file(self):
        text = "```\n" + "q" * 90 + "\n" + "r" * 90 + "\n"
        self.assertEqual(policy.check_file(SRC, text), [])

    def test_second_block_after_prose_is_tracked(self):
        text = ("```\nx\n```\n" + "p" * 81 + "\n````text\n" + "q" * 90 +
                "\n````\n")
        self.assertEqual(policy.check_file(SRC, text),
                         [V(SRC, 4, "line_length", 81)])

    def test_inner_shorter_fence_does_not_close(self):
        text = "`````\n```\n" + "q" * 90 + "\n`````\n" + "p" * 81 + "\n"
        self.assertEqual(policy.check_file(SRC, text),
                         [V(SRC, 5, "line_length", 81)])

    def test_tilde_fences_are_tracked(self):
        text = "~~~\n" + "q" * 90 + "\n~~~\n"
        self.assertEqual(policy.check_file(SRC, text), [])


if __name__ == "__main__":
    unittest.main()
