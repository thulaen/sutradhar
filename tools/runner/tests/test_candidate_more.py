"""Retrofit (mutation closure): export edge cases found by mutation tests.

Each test pins behaviour that a surviving mutant showed was unchecked.
"""
import hashlib
import os
import subprocess
import tempfile
import unittest
from unittest import mock

from tools.runner import candidate
from tools.runner.tests.test_candidate import CandidateCase


class EnvironmentTest(CandidateCase):
    def test_callers_git_environment_cannot_redirect_the_export(self):
        self.repo.write("a.txt", b"x\n")
        tree = self.repo.commit()
        hostile = {"GIT_DIR": os.path.join(self.folder, "elsewhere"),
                   "GIT_INDEX_FILE": os.path.join(self.folder, "index")}
        with mock.patch.dict(os.environ, hostile):
            self.assertEqual(candidate.resolve_tree(self.repo.path, "HEAD"),
                             tree)
            self.assertEqual(len(self.export(tree)), 1)


class NameTest(CandidateCase):
    def test_nested_git_folder_is_refused(self):
        blob = self.repo.git("hash-object", "-w", "--stdin", data=b"x\n")
        inner = self.repo.git("mktree", data=f"100644 blob {blob}\t.git\n"
                              .encode())
        outer = self.repo.git("mktree", data=f"040000 tree {inner}\ta\n"
                              .encode())
        with self.assertRaisesRegex(candidate.CandidateError,
                                    r"^unsafe path: a/\.git$"):
            self.export(outer)

    def test_tab_in_a_name_is_exported_exactly(self):
        name = "a" + chr(9) + "b.txt"
        self.repo.write(name, b"t\n")
        manifest = self.export(self.repo.commit())
        self.assertEqual([m["path"] for m in manifest], [name])
        with open(os.path.join(self.dest(), name), "rb") as handle:
            self.assertEqual(handle.read(), b"t\n")


class LimitTest(CandidateCase):
    """Review finding 8: an export cannot exhaust memory or disk."""

    def test_size_and_count_limits(self):
        self.repo.write("a.txt", b"12345\n")
        self.repo.write("b.txt", b"1\n")
        tree = self.repo.commit()
        with mock.patch.object(candidate, "MAX_EXPORT_BYTES", 7):
            with self.assertRaisesRegex(candidate.CandidateError,
                                        "^candidate too large: 8 bytes$"):
                self.export(tree, "o1")
        with mock.patch.object(candidate, "MAX_FILES", 1):
            with self.assertRaisesRegex(candidate.CandidateError,
                                        "^candidate has too many files: 2$"):
                self.export(tree, "o2")
        self.assertFalse(os.path.exists(self.dest("o1")))
        with mock.patch.object(candidate, "MAX_EXPORT_BYTES", 8), \
                mock.patch.object(candidate, "MAX_FILES", 2):
            self.assertEqual(len(self.export(tree, "o3")), 2)

    def test_names_that_look_like_options_are_refused(self):
        self.repo.write("-rf", b"x\n")
        with self.assertRaisesRegex(candidate.CandidateError,
                                    "^unsafe path: -rf$"):
            self.export(self.repo.commit())


class GitTest(CandidateCase):
    """Review finding 12: Git runs only on the runner's own repository."""

    def test_repository_must_belong_to_the_runner(self):
        self.repo.write("a.txt", b"x\n")
        self.repo.commit()
        owner = os.stat(self.repo.path).st_uid
        with mock.patch("os.getuid", return_value=owner + 1):
            with self.assertRaisesRegex(candidate.CandidateError,
                                        "^repository not owned by the "
                                        "runner$"):
                candidate.resolve_tree(self.repo.path, "HEAD")

    def test_git_never_fetches_and_has_a_time_limit(self):
        calls = []

        def fake(argv, **kwargs):
            calls.append((argv, kwargs))
            raise subprocess.TimeoutExpired(argv, kwargs["timeout"])
        with self.assertRaisesRegex(candidate.CandidateError,
                                    "^git version timed out$"):
            candidate.git(self.repo.path, ["version"], run=fake)
        argv, kwargs = calls[0]
        self.assertIn("protocol.allow=never", argv)
        self.assertEqual(kwargs["env"]["GIT_NO_LAZY_FETCH"], "1")
        self.assertEqual(kwargs["timeout"], candidate.GIT_SECONDS)


class WriteTest(unittest.TestCase):
    def test_existing_entries_and_links_are_never_overwritten(self):
        with tempfile.TemporaryDirectory() as folder:
            candidate._write(folder, "x", b"1", False)
            with self.assertRaises(FileExistsError):
                candidate._write(folder, "x", b"2", False)
            target = os.path.join(folder, "target")
            os.symlink(target, os.path.join(folder, "y"))
            with self.assertRaises(FileExistsError):
                candidate._write(folder, "y", b"2", False)
            self.assertFalse(os.path.exists(target))

    def test_only_the_owner_execute_bit_marks_executables(self):
        with tempfile.TemporaryDirectory() as folder:
            path = os.path.join(folder, "f")
            with open(path, "wb") as handle:
                handle.write(b"1")
            os.chmod(path, 0o645)
            entry = {"path": "f", "executable": False,
                     "sha256": hashlib.sha256(b"1").hexdigest()}
            self.assertEqual(candidate.path_digest(folder),
                             candidate.digest([entry]))


if __name__ == "__main__":
    unittest.main()
