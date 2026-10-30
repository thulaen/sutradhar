"""Test-first rules: second review pass and mutation closure.

Tests marked "Review 2" were written before their fix; those marked
"Retrofit" close mutation or coverage gaps after the code existed.
"""
import ast
import copy
import json
import unittest

from tools.runner import policy, tdd
from tools.runner.tests.test_policy import GOOD
from tools.runner.tests.test_tdd import OLD, keys

USES = """from pkg import code
class T:
    def test_x(self):
        self.assertTrue(code.f())
"""


class ReviewTwoTest(unittest.TestCase):
    def test_a_huge_hex_number_is_fingerprinted(self):
        # Review 2: repr() refuses integers over 4,300 digits.
        text = OLD + "N = 0x" + "f" * 3600 + "\n"
        self.assertIn("p.test_a", keys({}, {"p/test_a.py": text.encode()})[0])

    def test_an_import_swap_across_files_is_an_edit(self):
        # Review 2: each part carries the imports it uses.
        fake = USES.replace("from pkg import", "from pkg.fakes import")
        real, fakes = b"from pkg import code\n", b"from pkg.fakes import code\n"
        before = {"p/test_a.py": USES.encode(), "p/test_b.py": real,
                  "p/test_c.py": fakes}
        after = {"p/test_a.py": fake.encode(), "p/test_b.py": real + b"#\n",
                 "p/test_c.py": fakes + b"#\n"}
        self.assertTrue(tdd.tests_edited(before, after))

    def test_fixtures_belong_to_their_class(self):
        # Review 2: swapping set-up between classes is an edit, while a
        # test method may move to another class.
        text = ("class A:\n    def setUp(self):\n        self.n = 1\n"
                "    def test_x(self):\n        pass\n"
                "class B:\n    def setUp(self):\n        self.n = 2\n")
        swapped = text.replace("n = 1", "n = 9").replace(
            "n = 2", "n = 1").replace("n = 9", "n = 2")
        moved = text.replace("    def test_x(self):\n        pass\n", "") \
            + "    def test_x(self):\n        pass\n"
        before = {"p/test_a.py": text.encode()}
        self.assertTrue(tdd.tests_edited(
            before, {"p/test_a.py": swapped.encode()}))
        self.assertFalse(tdd.tests_edited(
            before, {"p/test_a.py": moved.encode()}))

    def test_a_changed_helper_runs_every_module(self):
        # Review 2: tests that inherit a changed helper class live in
        # other modules, so the runs cover every module.
        found = keys({}, {"p/tests/cases.py": USES.encode(),
                          "p/tests/test_b.py": USES.encode()})
        self.assertEqual(found[1], [])
        self.assertIn("p.tests.cases.T.test_x", found[0])

    def test_a_profile_running_test_first_also_runs_unit_tests(self):
        # Review 2: refactor and retrofit rely on the unit-test check.
        doc = copy.deepcopy(GOOD)
        doc["checks"]["tf"] = {"cwd": "@candidate", "report": "test-first",
                               "argv": ["python3", "@tool:harness"]}
        doc["profiles"]["commit"]["run"] = ["limits", "tf"]
        with self.assertRaises(policy.PolicyError) as caught:
            policy.load(json.dumps(doc).encode())
        self.assertEqual(str(caught.exception),
                         "profiles.commit: test-first needs a unit-test check")


class QualificationTest(unittest.TestCase):
    def test_modules_that_use_a_changed_test_module_also_run(self):
        # Benchmark finding: tests elsewhere inherit a changed test
        # module's classes, so modules naming it run too.
        user = b"from p.tests.test_a import T\nclass U(T):\n    pass\n"
        other = b"import unittest\n"
        before = {"p/tests/test_a.py": OLD.encode(),
                  "p/tests/test_b.py": user, "p/tests/test_c.py": other,
                  "p/tests/test_d.py": b"from p.tests import test_a\n"}
        after = dict(before, **{"p/tests/test_a.py": OLD.replace(
            "self.n = 1", "self.n = 2").encode()})
        self.assertEqual(keys(before, after)[1], [
            "p.tests.test_a", "p.tests.test_b", "p.tests.test_d"])

    def test_every_changed_module_is_looked_for(self):
        # Retrofit (mutation closure): two changed modules, each named by
        # a different user.
        before = {"p/tests/test_a.py": OLD.encode(),
                  "p/tests/test_e.py": OLD.encode(),
                  "p/tests/test_f.py": b"from p.tests import test_e\n"}
        after = {path: data.replace(b"self.n = 1", b"self.n = 2")
                 for path, data in before.items()}
        self.assertEqual(keys(before, after)[1], [
            "p.tests.test_a", "p.tests.test_e", "p.tests.test_f"])


class ClosureTest(unittest.TestCase):
    """Retrofit (mutation closure)."""

    def test_a_deleted_test_file_changes_no_test(self):
        self.assertEqual(keys({"p/test_a.py": OLD.encode()}, {}), ([], []))

    def test_a_file_at_the_size_limit_is_still_read(self):
        pad = tdd.LIMIT - len(OLD.encode())
        exact = (OLD + "#" * (pad - 1) + "\n").encode()
        self.assertEqual(len(exact), tdd.LIMIT)
        self.assertTrue(keys({}, {"p/test_a.py": exact})[0])
        self.assertFalse(keys({}, {"p/test_a.py": exact + b"#"})[0])

    def test_test_module_names_choose_the_modules_to_run(self):
        found = keys({}, {"test_top.py": OLD.encode(),
                          "p/thing_test.py": OLD.encode()})
        self.assertEqual(found[1], ["p.thing_test", "test_top"])

    def test_counts_and_bytes_matter_in_a_refactor(self):
        before = {"p/test_a.py": (OLD + "X = 1\n").encode(),
                  "p/tests/data.txt": b"x = 1"}
        for path, new in (("p/test_a.py", OLD + "X = 1\nX = 1\n"),
                          ("p/tests/data.txt", "x =  1")):
            after = dict(before, **{path: new.encode()})
            self.assertTrue(tdd.tests_edited(before, after), new)

    def test_the_fingerprint_format_is_fixed(self):
        # Pinned on Dell's Python; a new Python whose syntax trees differ
        # changes it, which must be reviewed rather than missed.
        tree = ast.parse("x = a + 0x10  # c")
        self.assertEqual(tdd._digest(tree), "900503197d861a94209b13"
                         "7813a5d3e3f6931433504b06d18e7056a33b76fefe")

    def test_dotted_imports_bind_their_first_name(self):
        tree = ast.parse("import os.path\nfrom a import b as c\n")
        self.assertEqual(sorted(tdd._bindings(tree)), ["c", "os"])

    def test_a_new_import_alone_is_an_edit(self):
        # An import can change behaviour by being run, unused or not.
        before = {"p/test_a.py": OLD.encode()}
        after = {"p/test_a.py": ("import patcher\n" + OLD).encode()}
        self.assertTrue(tdd.tests_edited(before, after))

    def test_a_candidate_test_without_keys_never_counts(self):
        red = {"ids": ["m.T.test_x"], "failed_ids": ["m.T.test_x"],
               "errored_ids": []}
        green = {"ids": ["m.T.test_x"], "keys": {}}
        self.assertEqual(tdd.qualifying(["m.T.test_x"], red, green),
                         [])


if __name__ == "__main__":
    unittest.main()
