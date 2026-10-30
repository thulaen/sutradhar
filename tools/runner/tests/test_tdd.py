"""Test-first evidence rules (SPEC-007 part 2: AT-T01, AT-T02, AT-G19)."""
import copy
import os
import unittest

from tools.runner import policy, tdd
from tools.runner.tests.test_policy import GOOD

OLD = """import unittest
from pkg import code
class T(unittest.TestCase):
    def setUp(self):
        self.n = 1
    def test_old(self):
        self.assertEqual(code.f(), self.n)  # first
"""
SAME = OLD.replace("# first", "# comment only").replace(
    "\n    def", "\n\n    def")
M = "p.test_a"


def keys(before, after):
    return tdd.changed_keys(before, after)


class DeclarationTest(unittest.TestCase):
    def test_each_declaration_is_read(self):
        for line in ("test-first", "retrofit", "not-applicable (documentation)",
                     "not-applicable (refactor)", "not-applicable (dependency)",
                     "not-applicable (secret)",
                     "not-applicable (configuration)"):
            message = f"feat: x\r\n\r\nBody.\r\n\r\nTDD: {line}  \r\n".encode()
            self.assertEqual(tdd.declaration(message), line)

    def test_missing_repeated_or_unknown_declarations_are_none(self):
        for message in (None, b"", b"feat: x\n", b"TDD: maybe\n",
                        b"TDD: not-applicable\n", b"TDD: test-first\n" * 2,
                        b"TDD: not-applicable (style)\n",
                        b"TDD: test-first\nTDD: retrofit\n",
                        b"TDD:test-first\n", b"tdd: test-first\n",
                        b"  TDD: test-first\n", b"TDD: test-first \xff\n",
                        "TDD: test-first\u2028more\n".encode()):
            self.assertIsNone(tdd.declaration(message), message)

    def test_the_reason_of_a_declaration(self):
        self.assertEqual([tdd.reason(found) for found in (
            "test-first", "retrofit", "not-applicable (refactor)")],
            [None, None, "refactor"])


class ChangedKeysTest(unittest.TestCase):
    """A test is new or changed when its own code, its class's fixtures
    or its module's set-up changed; layout and comments do not count."""

    def test_a_new_file_brings_every_key(self):
        found = keys({}, {"p/test_a.py": OLD.encode(), "p/tests/x.txt": b"x"})
        self.assertEqual(found, ([M, f"{M}.T", f"{M}.T.test_old"], [M]))

    def test_only_what_changed_is_named(self):
        before = {"p/test_a.py": OLD.encode()}
        self.assertEqual(keys(before, {"p/test_a.py": SAME.encode()}),
                         ([], []))
        for old, new, changed in (
                ("code.f(), self.n", "code.f(), 2", [f"{M}.T.test_old"]),
                ("self.n = 1", "self.n = 2", [f"{M}.T"]),
                ("class T(", "@unittest.skip('x')\nclass T(", [f"{M}.T"]),
                ("from pkg import code", "from pkg import other as code",
                 [M])):
            after = {"p/test_a.py": OLD.replace(old, new).encode()}
            self.assertEqual(keys(before, after), (changed, [M]), new)

    def test_unparsable_or_deep_files_never_break_the_run(self):
        deep = "x = " + "+".join(["1"] * 3000) + "\n"
        broken = {"p/test_a.py": b"def (:\n", "p/test_b.py": b"x = 1\0\n",
                  "p/test_c.py": b"#" * (tdd.LIMIT + 1)}
        self.assertEqual(keys({}, broken), ([], []))
        found = keys({}, {"p/test_d.py": (OLD + deep).encode()})
        self.assertIn("p.test_d", found[0])

    def test_mixins_and_nested_classes_are_named_by_qualified_name(self):
        text = ("class Cases:\n    def test_x(self):\n        pass\n"
                "class Outer:\n    class Inner:\n"
                "        def test_y(self):\n            pass\n")
        found = keys({}, {"p/test_a.py": text.encode()})[0]
        self.assertIn(f"{M}.Cases.test_x", found)
        self.assertIn(f"{M}.Outer.Inner.test_y", found)


class DocstringTest(unittest.TestCase):
    def test_module_and_class_docstrings_are_not_code(self):
        # Retrofit (coverage closure).
        before = {"p/test_a.py": OLD.encode()}
        text = '"""Module."""\n' + OLD.replace(
            "TestCase):\n", 'TestCase):\n    """Class."""\n')
        after = {"p/test_a.py": text.encode()}
        self.assertEqual(keys(before, after), ([], []))
        self.assertFalse(tdd.tests_edited(before, after))


class EditedTest(unittest.TestCase):
    """AT-T02: a refactor may move tests but never edit them."""

    def test_moves_and_repeated_imports_are_not_edits(self):
        before = {"p/test_a.py": OLD.encode(), "p/test_c.py": b"import os\n",
                  "p/tests/data.txt": b"x", "p/tests/bad.py": b"def (:\n"}
        moved = {"p/test_b.py": SAME.encode(), "p/test_c.py": b"import os\n",
                 "p/tests/data.txt": b"x", "p/tests/bad.py": b"def (:\n"}
        self.assertFalse(tdd.tests_edited(before, moved))

    def test_anything_that_changes_behaviour_is_an_edit(self):
        base = OLD + "CASES = [1]\n"
        before = {"p/test_a.py": base.encode(), "p/tests/data.txt": b"x",
                  "p/tests/__init__.py": b"", "p/tests/bad.py": b"def (:\n"}
        for path, new in (
                ("p/test_a.py", base.replace("f(), self.n", "f(), 2")),
                ("p/test_a.py", base.replace("[1]", "[2]")),
                ("p/test_a.py", base + "setattr(T, 'test_old', None)\n"),
                ("p/test_a.py", base + "del T.test_old\n"),
                ("p/test_a.py", base.replace("class T(", "@d\nclass T(")),
                ("p/test_a.py", base.replace("(unittest.TestCase)", "(X)")),
                ("p/test_a.py", base + "if True:\n    CASES = []\n"),
                ("p/test_a.py", base.replace("import code",
                                             "import g as code")),
                ("p/tests/__init__.py", "import mock\nmock.patch('x')\n"),
                ("p/tests/data.txt", "y"), ("p/tests/bad.py", "def (:\n#")):
            after = dict(before, **{path: new.encode()})
            self.assertTrue(tdd.tests_edited(before, after), new)


DETAIL = {"ids": ["m.T.test_new", "m.T.test_noted", "m.T.test_old",
                  "m.T.test_passes"],
          "failed_ids": ["m.T.test_new", "m.T.test_old", "m.T.test_ghost"],
          "errored_ids": ["m.T.test_noted"]}
GREEN = {"ids": DETAIL["ids"] + ["m.U.test_absent", "m.T.test_ghost"],
         "keys": {name: ["m", name.rsplit(".", 1)[0], name]
                  for name in DETAIL["ids"] + ["m.U.test_absent",
                                               "m.T.test_ghost"]}}


class QualifyingTest(unittest.TestCase):
    """AT-T01: only a behavioural red on a new or changed test counts."""

    def test_failures_on_the_parent_that_pass_now_qualify(self):
        changed = ["m.T.test_new", "m.T.test_noted", "m.T.test_passes",
                   "m.U.test_absent", "m.T.test_ghost"]
        self.assertEqual(tdd.qualifying(changed, DETAIL, GREEN),
                         ["m.T.test_new"])

    def test_a_changed_fixture_makes_its_tests_changed(self):
        self.assertEqual(tdd.qualifying(["m.T"], DETAIL, GREEN),
                         ["m.T.test_new", "m.T.test_old"])

    def test_errors_and_absence_need_a_reviewed_note(self):
        changed = ["m.T.test_noted", "m.T.test_passes", "m.U.test_absent"]
        self.assertEqual(tdd.qualifying(changed, DETAIL, GREEN, changed),
                         ["m.T.test_noted", "m.U.test_absent"])
        self.assertEqual(tdd.qualifying(changed, DETAIL, GREEN), [])

    def test_a_test_missing_from_the_candidate_run_never_counts(self):
        self.assertEqual(tdd.qualifying(
            ["m.T.test_new"], DETAIL, {"ids": [], "keys": {}},
            ["m.T.test_new"]), [])


class PathsTest(unittest.TestCase):
    def setUp(self):
        self.loaded = copy.deepcopy(GOOD)

    def test_documentation_needs_documentation_files_only(self):
        problem = tdd.paths_problem
        self.assertIsNone(problem("documentation", ["docs/a.md"], [],
                                  self.loaded))
        self.assertEqual(
            problem("documentation", ["a.md", "tools/x.py"], [], self.loaded),
            "not a documentation change: tools/x.py")
        self.assertEqual(
            problem("documentation", ["run.md"], ["run.md"], self.loaded),
            "not a documentation change: run.md")
        self.assertEqual(problem("documentation", [], [], self.loaded),
                         "no change")

    def test_other_reasons_use_the_policy_paths(self):
        problem = tdd.paths_problem
        self.assertIsNone(problem("dependency", ["requirements.txt"],
                                  ["requirements.txt"], self.loaded))
        self.assertIsNone(problem("configuration", ["setup.cfg"], [],
                                  self.loaded))
        self.assertEqual(
            problem("configuration", ["setup.cfg", "x.py"], [], self.loaded),
            "not a configuration change: x.py")
        self.assertEqual(problem("secret", ["x.py"], [], self.loaded),
                         "not a secret change: x.py")

    def test_a_retrofit_changes_only_tests_and_documentation(self):
        problem = tdd.retrofit_problem
        self.assertIsNone(problem(["a/tests/test_x.py", "docs/a.md"], [],
                                  self.loaded))
        self.assertEqual(problem(["a/tests/test_x.py", "a/code.py"], [],
                                 self.loaded),
                         "retrofit changes code: a/code.py")
        self.assertEqual(problem(["run.md"], ["run.md"], self.loaded),
                         "retrofit changes code: run.md")


class PolicyTest(unittest.TestCase):
    def test_only_test_folders_hold_test_like_file_names(self):
        # Qualification finding: the harness treats every test*.py and
        # *_test.py file as a test file, so a module named that way fails
        # the unit-test check.
        root = os.path.dirname(os.path.dirname(policy.__file__))
        for folder, _, files in os.walk(root):
            for name in files:
                looks = name.endswith(".py") and (
                    name.startswith("test") or name.endswith("_test.py"))
                if looks:
                    self.assertEqual(os.path.basename(folder), "tests",
                                     os.path.join(folder, name))

    def test_patterns_match_names_or_whole_paths(self):
        self.assertTrue(policy.matches(["*.md"], "docs/a.md"))
        self.assertTrue(policy.matches(["*/tests/*"], "a/tests/b/c.py"))
        self.assertFalse(policy.matches(["docs/*"], "x/docs/a.md"))

    def test_repository_policy_runs_the_test_first_gate(self):
        path = os.path.join(os.path.dirname(policy.__file__), "policy.json")
        with open(path, "rb") as handle:
            loaded = policy.load(handle.read())
        self.assertEqual(loaded["checks"]["test-first"], {
            "cwd": "@candidate", "report": "test-first",
            "argv": ["python3", "-I", "-S", "-B", "@tool:harness",
                     "--detail", "@candidate"]})
        for name in ("docs", "commit", "push"):
            self.assertIn("test-first", loaded["profiles"][name]["run"])
        tests = loaded["test_first"]["tests"]
        for name in ("tools/runner/tests/test_cli.py", "tests/x.py",
                     "a/b_test.py", "a/tests/data.txt"):
            self.assertTrue(policy.matches(tests, name), name)
        self.assertFalse(policy.matches(tests, "tools/runner/cli.py"))
        self.assertEqual(loaded["test_first"]["paths"]["secret"], [])


if __name__ == "__main__":
    unittest.main()
