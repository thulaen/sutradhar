"""Runner policy: strict loading, profile choice, argument expansion."""
import copy
import json
import os
import unittest

from tools.runner import policy

HASH = "a" * 64
GOOD = {
    "format": "sutradhar-runner-policy/1", "version": "t1", "runner": HASH,
    "bootstrap_scope": ["docs/*", "tools/*", "*.md"], "max_age": 3600,
    "limits": {"memory": "1G", "tasks": 64, "seconds": 300},
    "documentation": ["docs/*", "*.md"],
    "test_first": {"tests": ["test_*.py", "*_test.py", "tests/*",
                             "*/tests/*"],
                   "paths": {"configuration": ["*.cfg", "*.ini", "*.toml"],
                             "dependency": ["requirements*.txt", "*.lock"],
                             "secret": []}},
    "tools": {"limits": HASH, "harness": HASH},
    "checks": {
        "limits": {"cwd": "@tool:limits", "report": "exit",
                   "argv": ["python3", "-m", "tools.limits.cli",
                            "@candidate", "--classes",
                            "@tool:limits/classes.txt", "@paths"]},
        "unit": {"cwd": "@candidate", "report": "unittest",
                 "argv": ["python3", "-I", "@tool:harness", "@candidate"]},
    },
    "profiles": {
        "docs": {"documentation_only": True, "run": ["limits"],
                 "not_applicable": {"unit": "documentation only"},
                 "not_yet_active": {"test-first": "activation step 3"},
                 "interim": ["test-first"]},
        "commit": {"documentation_only": False, "run": ["limits", "unit"],
                   "not_applicable": {}, "not_yet_active": {},
                   "interim": []},
    },
}


def load(doc):
    return policy.load(json.dumps(doc).encode())


def changed(doc, path, value):
    doc = copy.deepcopy(doc)
    *parents, last = path
    target = doc
    for key in parents:
        target = target[key]
    if value is KeyError:
        del target[last]
    else:
        target[last] = value
    return doc


_VERSION = "policy: unsupported format or empty version"
_CWD = "checks.unit: cwd must be @candidate or a tool"
_MIXED = "profiles.docs: empty or overlapping check lists"
_REASON = "profiles.docs: an exemption needs a reason"
_KEYS = "profiles.docs: expected keys "
BAD_CASES = [
    (("format",), "other/1", _VERSION), (("version",), "", _VERSION),
    (("runner",), "abc", "runner: must be a SHA-256 digest"),
    (("bootstrap_scope",), [3],
     "bootstrap_scope: patterns must be text"),
    (("max_age",), 0, "max_age: must be a positive whole number"),
    (("max_age",), True, "policy.max_age: expected int"),
    (("profiles", "docs", "interim"), ["unit"],
     "profiles.docs: interim checks must be not yet active"),
    (("profiles", "docs", "interim"), "test-first",
     "profiles.docs.interim: expected list"),
    (("checks", "unit", "report"), "junit",
     "checks.unit: report must be exit, unittest or test-first"),
    (("test_first",), KeyError, "policy: expected keys "),
    (("test_first", "other"), 1, "test_first: expected keys "),
    (("test_first", "tests"), "*.py", "test_first.tests: expected list"),
    (("test_first", "tests"), [3],
     "test_first.tests: patterns must be text"),
    (("test_first", "paths", "other"), [], "test_first.paths: expected keys "),
    (("test_first", "paths", "secret"), "x",
     "test_first.paths.secret: expected list"),
    (("test_first", "paths", "dependency"), [1],
     "test_first.paths.dependency: patterns must be text"),
    (("extra",), 1, "policy: expected keys "),
    (("limits", "tasks"), "64", "limits.tasks: expected int"),
    (("limits", "memory"), 1, "limits.memory: expected str"),
    (("limits", "other"), 1, "limits: expected keys "),
    (("documentation",), "*.md",
     "policy.documentation: expected list"),
    (("documentation",), [3], "documentation: patterns must be text"),
    (("profiles", "docs"), {
        "documentation_only": True, "run": ["limits"],
        "not_applicable": {}, "not_yet_active": {"unit": "r"},
        "interim": []},
     "profiles.docs: an active check cannot be not yet active: "
     "unit"),
    (("tools", "limits"), "abc",
     "tools: each pin must be a SHA-256 digest"),
    (("tools", "Bad"), HASH, "tools: bad identifier 'Bad'"),
    (("checks", "unit", "argv"), [], "checks.unit: empty argv"),
    (("checks", "unit", "argv"), [3],
     "checks.unit: arguments must be text"),
    (("checks", "unit", "cwd"), "@tool:limits/x", _CWD),
    (("checks", "unit", "cwd"), "@paths", _CWD),
    (("checks", "unit", "cwd"), "/tmp", _CWD),
    (("checks", "unit", "extra"), 1, "checks.unit: expected keys "),
    (("checks", "bad id"), {"cwd": "@candidate", "argv": ["x"]},
     "checks: bad identifier 'bad id'"),
    (("profiles", "docs", "run"), [], _MIXED),
    (("profiles", "docs", "run"), ["nope"],
     "profiles.docs: unknown check"),
    (("profiles", "docs", "run"), ["limits", "limits"], _MIXED),
    (("profiles", "docs", "not_applicable"), {"nope": "r"},
     "profiles.docs: unknown check"),
    (("profiles", "docs", "not_applicable"), {"unit": ""}, _REASON),
    (("profiles", "docs", "not_applicable"), {"unit": " "}, _REASON),
    (("profiles", "docs", "not_applicable"), {"limits": "r"}, _MIXED),
    (("profiles", "docs", "not_yet_active"), {"unit": "r"}, _MIXED),
    (("profiles", "docs", "not_yet_active"), {"Bad Id": "r"},
     "profiles.docs: bad identifier 'Bad Id'"),
    (("profiles", "docs", "documentation_only"), 1,
     "profiles.docs.documentation_only: expected bool"),
    (("profiles", "docs", "extra"), 1, _KEYS),
    (("profiles", "docs", "run"), KeyError, _KEYS),
    (("profiles",), {}, "profiles: none defined"),
    (("profiles", "Bad"), GOOD["profiles"]["docs"],
     "profiles: bad identifier 'Bad'"),
]
for value in ("@unknown", "@tool:missing", "@tool:limits/../x",
              "@tool:limits/./x"):
    BAD_CASES.append((("checks", "unit", "argv"), [value],
                  f"checks.unit: unknown placeholder {value}"))


class LoadTest(unittest.TestCase):
    def test_good_policy_loads(self):
        self.assertEqual(load(GOOD), GOOD)

    def test_repository_policy_loads(self):
        path = os.path.join(os.path.dirname(policy.__file__), "policy.json")
        with open(path, "rb") as handle:
            loaded = policy.load(handle.read())
        self.assertIn("commit", loaded["profiles"])

    def test_bad_documents_are_refused_in_plain_words(self):
        for path, value, message in BAD_CASES:
            with self.assertRaises(policy.PolicyError) as caught:
                load(changed(GOOD, path, value))
            self.assertTrue(str(caught.exception).startswith(message),
                            (path, value, str(caught.exception)))
            if not message.endswith(" "):
                self.assertEqual(str(caught.exception), message)

    def test_non_json_and_non_objects_are_refused(self):
        for data, message in ((b"{", "not JSON: "), (b"\xff", "not JSON: "),
                              (b'{"a": NaN}',
                               "not JSON: NaN is not allowed"),
                              (b"[]", "policy: expected keys ")):
            with self.assertRaises(policy.PolicyError) as caught:
                policy.load(data)
            self.assertTrue(str(caught.exception).startswith(message), data)


class SelectTest(unittest.TestCase):
    def setUp(self):
        self.policy = load(GOOD)

    def test_documentation_change_may_use_docs_profile(self):
        got = policy.select(self.policy, "docs", ["docs/a.md", "README.md"])
        self.assertIs(got, self.policy["profiles"]["docs"])

    def test_only_documentation_paths_may_use_docs_profile(self):
        # Review 2 finding F: an allowlist, so tool settings such as
        # tools/runner/policy.json never pass as documentation.
        for path in ("tools/x.py", "justfile", ".github/workflows/a.yml",
                     "tools/runner/policy.json", "tools/limits/classes.txt",
                     "LICENSE"):
            with self.assertRaises(policy.PolicyError) as caught:
                policy.select(self.policy, "docs", ["a.md", path])
            self.assertEqual(str(caught.exception),
                             "documentation-only profile refused: "
                             f"not a documentation change: {path}")

    def test_executable_bit_refuses_docs_profile(self):
        with self.assertRaises(policy.PolicyError) as caught:
            policy.select(self.policy, "docs", ["a.md", "run.md"],
                          ["run.md"])
        self.assertEqual(str(caught.exception), "documentation-only "
                         "profile refused: not a documentation change: "
                         "run.md")

    def test_smallest_maximum_age_is_one_second(self):
        self.assertEqual(load(changed(GOOD, ("max_age",), 1))["max_age"], 1)

    def test_empty_change_refuses_docs_profile(self):
        with self.assertRaises(policy.PolicyError) as caught:
            policy.select(self.policy, "docs", [])
        self.assertEqual(str(caught.exception),
                         "documentation-only profile refused: no change")

    def test_code_profile_accepts_anything(self):
        got = policy.select(self.policy, "commit", ["tools/x.py"], ["x"])
        self.assertIs(got, self.policy["profiles"]["commit"])

    def test_unknown_profile(self):
        with self.assertRaisesRegex(policy.PolicyError,
                                    "^unknown profile: release$"):
            policy.select(self.policy, "release", ["a.md"])

    def test_plan_names_each_kind_of_check(self):
        self.assertEqual(policy.plan(self.policy, "docs"), {
            "run": ["limits"], "not_applicable": ["unit"],
            "not_yet_active": ["test-first"], "interim": ["test-first"]})
        self.assertEqual(policy.plan(self.policy, "commit"), {
            "run": ["limits", "unit"], "not_applicable": [],
            "not_yet_active": [], "interim": []})

    def test_bootstrap_scope_covers_every_changed_path(self):
        self.assertTrue(policy.in_scope(self.policy,
                                        ["docs/a.md", "tools/x.py", "R.md"]))
        self.assertFalse(policy.in_scope(self.policy,
                                         ["docs/a.md", "src/feature.py"]))
        self.assertFalse(policy.in_scope(self.policy, []))

    def test_repository_documentation_is_judged_by_file_type(self):
        # Review 3 finding 5: code inside docs/ is not documentation.
        path = os.path.join(os.path.dirname(policy.__file__), "policy.json")
        with open(path, "rb") as handle:
            loaded = policy.load(handle.read())
        for name in ("docs/conf.py", "docs/build.sh", "catalogue/x.py"):
            with self.assertRaises(policy.PolicyError, msg=name):
                policy.select(loaded, "docs", [name])
        policy.select(loaded, "docs", ["docs/a.md", "docs/b/c.yaml",
                                       "catalogue/x.yaml", "README.md"])

    def test_repository_policy_treats_build_files_as_code(self):
        path = os.path.join(os.path.dirname(policy.__file__), "policy.json")
        with open(path, "rb") as handle:
            loaded = policy.load(handle.read())
        for name in ("Justfile", ".justfile", "pyproject.toml", "setup.cfg",
                     "a/b.sh", "tox.ini", ".github/workflows/x.yml"):
            with self.assertRaises(policy.PolicyError, msg=name):
                policy.select(loaded, "docs", [name])

    def test_repository_policy_keeps_its_safety_settings(self):
        # Review 2 finding I: these settings are what make the checks
        # trustworthy, so changing them must fail a test.
        path = os.path.join(os.path.dirname(policy.__file__), "policy.json")
        with open(path, "rb") as handle:
            checks = policy.load(handle.read())["checks"]
        # Review 3 findings 1 and 4: the whole command is pinned.
        self.assertEqual(checks["secrets"], {
            "cwd": "@candidate", "report": "exit",
            "argv": ["@tool:gitleaks", "dir", "--no-banner", "--redact",
                     "--log-level", "warn", "--exit-code", "1",
                     "--ignore-gitleaks-allow", "--config",
                     "@tool:gitleaks-config", "--gitleaks-ignore-path",
                     "/opt/sutradhar", "@scan"]})
        self.assertEqual(checks["source-limits"]["argv"], [
            "python3", "-E", "-s", "-B", "-m", "tools.limits.cli",
            "@candidate", "--classes", "@tool:limits/tools/limits/classes.txt",
            "@paths"])
        unit = checks["unit-tests"]
        self.assertEqual(unit["report"], "unittest")
        self.assertEqual(unit["argv"], ["python3", "-I", "-S", "-B",
                                        "@tool:harness", "@candidate"])
        self.assertEqual(checks["source-limits"]["cwd"], "@tool:limits")


class ExpandTest(unittest.TestCase):
    def test_tokens(self):
        loaded = load(GOOD)
        self.assertEqual(policy.argv(loaded, "limits", ["a.md", "b.py"]), [
            "python3", "-m", "tools.limits.cli", "/opt/scan/candidate",
            "--classes", "/opt/sutradhar/limits/classes.txt", "a.md", "b.py"])
        self.assertEqual(policy.argv(loaded, "unit", ["a.md"]), [
            "python3", "-I", "/opt/sutradhar/harness", "/opt/scan/candidate"])
        self.assertEqual(policy.cwd(loaded, "limits"),
                         "/opt/sutradhar/limits")
        self.assertEqual(policy.cwd(loaded, "unit"), "/opt/scan/candidate")

    def test_scan_folder_holds_only_the_candidate(self):
        # Review 2 finding B: scanning the folder above the candidate means
        # the candidate's own scanner settings and ignore files are unread.
        loaded = load(changed(GOOD, ("checks", "unit", "argv"),
                              ["x", "@scan"]))
        self.assertEqual(policy.argv(loaded, "unit", []), ["x", "/opt/scan"])


if __name__ == "__main__":
    unittest.main()
