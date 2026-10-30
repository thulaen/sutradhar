"""Owner-approved exceptions and path classification (SPEC-007)."""
import unittest

from tools.limits import counting, policy

LONG_URL = "https://example.org/" + "a" * 250


class LinkOnlyTest(unittest.TestCase):
    def test_whole_link_on_its_own_line(self):
        self.assertTrue(policy.is_link_only(f"[text]({LONG_URL})"))

    def test_link_with_trailing_full_stop(self):
        self.assertTrue(policy.is_link_only(f"[text]({LONG_URL})."))

    def test_continued_link_text(self):
        self.assertTrue(policy.is_link_only(f"settings]({LONG_URL})"))

    def test_bare_url(self):
        self.assertTrue(policy.is_link_only(LONG_URL))

    def test_prose_before_link_is_not_link_only(self):
        self.assertFalse(policy.is_link_only(f"See the [text]({LONG_URL})"))

    def test_prose_after_link_is_not_link_only(self):
        self.assertFalse(policy.is_link_only(f"[t]({LONG_URL}) and more"))


class ExceptionTest(unittest.TestCase):
    def kinds(self, path, text):
        return [v.kind for v in policy.check_file(path, text)]

    def test_link_line_up_to_200_passes(self):
        line = f"[t]({LONG_URL[:195]})"
        self.assertEqual(len(line), 200)
        self.assertEqual(self.kinds("docs/a.md", line + "\n"), [])

    def test_link_line_over_200_fails(self):
        line = f"[t]({LONG_URL[:196]})"
        self.assertEqual(self.kinds("docs/a.md", line + "\n"),
                         ["line_length"])

    def test_quotation_block_in_sources_keeps_long_lines(self):
        text = "`````text\n" + "q" * 300 + "\n`````\n"
        path = "docs/requirements/sources/interview-q001-q009.md"
        self.assertEqual(self.kinds(path, text), [])

    def test_quotation_block_elsewhere_is_not_exempt(self):
        text = "`````text\n" + "q" * 300 + "\n`````\n"
        self.assertEqual(self.kinds("docs/dev/a.md", text), ["line_length"])

    def test_prose_in_sources_is_not_exempt(self):
        path = "docs/requirements/sources/README.md"
        self.assertEqual(self.kinds(path, "p" * 81 + "\n"), ["line_length"])

    def test_tab_inside_quotation_is_still_refused(self):
        text = "```\na\tb\n```\n"
        path = "docs/requirements/sources/interview-q001-q009.md"
        self.assertEqual(self.kinds(path, text), ["forbidden_character"])

    def test_file_length_limit_has_no_exception(self):
        path = "docs/requirements/sources/interview-q001-q009.md"
        text = "```\n" + "x\n" * 300 + "```\n"
        self.assertIn("file_lines", self.kinds(path, text))


class ClassificationTest(unittest.TestCase):
    RULES = "text *.md\ntext tools/**/*.py\nlockfile uv.lock\n" \
            "binary *.png\nthird_party vendor/**\n"

    def setUp(self):
        self.classes = policy.load_classes(self.RULES)

    def test_first_matching_rule_wins(self):
        self.assertEqual(policy.classify("docs/a.md", self.classes), "text")

    def test_nested_glob(self):
        self.assertEqual(
            policy.classify("tools/limits/cli.py", self.classes), "text")

    def test_unclassified_path(self):
        self.assertIsNone(policy.classify("x.exe", self.classes))

    def test_unknown_class_is_rejected(self):
        with self.assertRaises(ValueError):
            policy.load_classes("magic *.md\n")

    def test_malformed_rule_is_rejected(self):
        with self.assertRaises(ValueError):
            policy.load_classes("text\n")

    def test_comments_and_blank_lines_are_ignored(self):
        self.assertEqual(policy.load_classes("# c\n\ntext *.md\n"),
                         [("text", "*.md")])

    def test_lockfile_is_not_line_checked(self):
        self.assertEqual(policy.measured("lockfile"), False)
        self.assertEqual(policy.measured("text"), True)


class ViolationTypeTest(unittest.TestCase):
    def test_policy_reuses_counting_violation(self):
        v = policy.check_file("a.md", "x" * 81 + "\n")[0]
        self.assertIsInstance(v, counting.Violation)


if __name__ == "__main__":
    unittest.main()
