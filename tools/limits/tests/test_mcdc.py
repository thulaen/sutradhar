"""Truth-table tests for compound decisions (rulebook E20, MC/DC).

Retrofit: written after the implementation to show that each condition
independently changes the decision. Each pair below differs in exactly
one condition and gives a different outcome.
"""
import unittest

from tools.limits import policy

SRC = "docs/requirements/sources/interview-q001-q009.md"
LONG = "q" * 90


class FenceClosingTest(unittest.TestCase):
    """closes = same mark AND long enough AND no info string;
    a backtick opener's info may not contain a backtick."""

    def after(self, marks, info, opener):
        return policy.fence_after(marks, info, opener)

    def test_all_conditions_true_closes(self):
        self.assertIsNone(self.after("`````", "", "`````"))

    def test_different_mark_keeps_open(self):
        self.assertEqual(self.after("~~~~~", "", "`````"), "`````")

    def test_shorter_fence_keeps_open(self):
        self.assertEqual(self.after("```", "", "`````"), "`````")

    def test_info_string_keeps_open(self):
        self.assertEqual(self.after("`````", "text", "`````"), "`````")

    def test_longer_fence_closes(self):
        self.assertIsNone(self.after("``````", "", "`````"))

    def test_backtick_opener_with_backtick_info_does_not_open(self):
        self.assertIsNone(self.after("```", "x`", None))

    def test_tilde_opener_with_backtick_info_opens(self):
        self.assertEqual(self.after("~~~", "x`", None), "~~~")

    def test_plain_opener_opens(self):
        self.assertEqual(self.after("```", "text", None), "```")


class QuotedTest(unittest.TestCase):
    """quoted = in sources AND was open AND still open."""

    def kinds(self, path, text):
        return [v.kind for v in policy.check_file(path, text)]

    def test_all_true_exempts_inner_line(self):
        self.assertEqual(self.kinds(SRC, f"```\n{LONG}\n```\n"), [])

    def test_outside_sources_measures(self):
        self.assertEqual(self.kinds("docs/a.md", f"```\n{LONG}\n```\n"),
                         ["line_length"])

    def test_opening_line_is_measured(self):
        self.assertEqual(self.kinds(SRC, f"```{LONG}\nx\n```\n"),
                         ["line_length"])

    def test_closing_line_is_measured(self):
        self.assertEqual(self.kinds(SRC, f"```\nx\n```{'`' * 85}\n"),
                         ["line_length"])


class LimitChoiceTest(unittest.TestCase):
    """limit: quoted -> none; else link-only -> 200; else 80."""

    def test_link_only_line_gets_200(self):
        line = "[t](https://example.org/" + "a" * 150 + ")"
        self.assertEqual(policy.line_limit(line, False), 200)

    def test_prose_line_gets_80(self):
        self.assertEqual(policy.line_limit("prose", False), 80)

    def test_quoted_line_has_no_length_limit(self):
        self.assertEqual(policy.line_limit("prose", True), float("inf"))


if __name__ == "__main__":
    unittest.main()
