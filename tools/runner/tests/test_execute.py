"""Running one sandboxed check and keeping its private output."""
import hashlib
import os
import subprocess
import tempfile
import unittest
from unittest import mock

from tools.runner import execute, sandbox
from tools.runner import outcomes as o


class FakeRun:
    def __init__(self, code=0, out=b"", err=b"", exc=None):
        self.code, self.out, self.err, self.exc = code, out, err, exc
        self.calls = []

    def __call__(self, argv, **kwargs):
        self.calls.append((argv, kwargs))
        kwargs["stdout"].write(self.out)
        kwargs["stderr"].write(self.err)
        if self.exc:
            raise self.exc
        return subprocess.CompletedProcess(argv, self.code)


def clock(*values):
    return iter(values).__next__


class RunCheckTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)

    def check(self, fake, seconds=60, times=(10.0, 12.5)):
        return execute.run_check("unit-tests", ["x", "y"], self.tmp.name,
                                 seconds, run=fake, clock=clock(*times))

    def read(self, name):
        with open(os.path.join(self.tmp.name, name), "rb") as handle:
            return handle.read()

    def test_pass_records_hashes_timing_and_output(self):
        got = self.check(FakeRun(0, b"hello", b"warn"))
        self.assertEqual(got, {
            "id": "unit-tests", "outcome": o.PASSED, "reason": "",
            "exit": 0, "seconds": 2.5,
            "stdout_sha256": hashlib.sha256(b"hello").hexdigest(),
            "stderr_sha256": hashlib.sha256(b"warn").hexdigest()})
        self.assertEqual(self.read("unit-tests.stdout"), b"hello")
        self.assertEqual(self.read("unit-tests.stderr"), b"warn")

    def test_call_has_no_shell_no_input_and_a_backstop(self):
        fake = FakeRun()
        self.check(fake, seconds=60)
        argv, kwargs = fake.calls[0]
        self.assertEqual(argv, ["x", "y"])
        self.assertEqual(kwargs["timeout"], 60 + execute.MARGIN)
        self.assertIs(kwargs["stdin"], subprocess.DEVNULL)
        self.assertFalse(kwargs.get("shell", False))
        self.assertFalse(kwargs.get("check", False))

    def test_failure_and_other_exits(self):
        got = self.check(FakeRun(1))
        self.assertEqual((got["outcome"], got["reason"]), (o.FAILED, "exit 1"))
        got = self.check(FakeRun(2))
        self.assertEqual((got["outcome"], got["reason"]),
                         (o.BLOCKED, "exit 2"))

    def test_backstop_timeout_blocks_and_keeps_output(self):
        fake = FakeRun(out=b"partial",
                       exc=subprocess.TimeoutExpired(["x"], 90))
        got = self.check(fake)
        self.assertEqual((got["outcome"], got["reason"], got["exit"]),
                         (o.BLOCKED, "time limit", None))
        self.assertEqual(self.read("unit-tests.stdout"), b"partial")

    def test_reaching_the_limit_blocks_even_with_exit_zero(self):
        got = self.check(FakeRun(0), seconds=60, times=(0.0, 60.0))
        self.assertEqual((got["outcome"], got["reason"]),
                         (o.BLOCKED, "time limit"))
        got = self.check(FakeRun(0), seconds=60, times=(0.0, 59.9))
        self.assertEqual(got["outcome"], o.PASSED)

    def test_start_failure_blocks(self):
        got = self.check(FakeRun(exc=FileNotFoundError("systemd-run")))
        self.assertEqual((got["outcome"], got["reason"], got["exit"]),
                         (o.BLOCKED, "cannot start: FileNotFoundError", None))

    def test_output_limit_matches_the_sandbox_file_limit(self):
        self.assertEqual(execute.MAX_OUTPUT, sandbox.FILE_LIMIT)

    def test_oversized_output_blocks(self):
        with mock.patch.object(execute, "MAX_OUTPUT", 4):
            got = self.check(FakeRun(0, b"12345"))
            self.assertEqual((got["outcome"], got["reason"]),
                             (o.BLOCKED, "output too large"))
            got = self.check(FakeRun(0, b"1234", b"1234"))
            self.assertEqual(got["outcome"], o.PASSED)

    def test_output_over_the_limit_on_stderr_blocks(self):
        with mock.patch.object(execute, "MAX_OUTPUT", 4):
            got = self.check(FakeRun(0, b"", b"12345"))
        self.assertEqual(got["reason"], "output too large")

    def test_decoded_status_decides_and_raw_code_is_kept(self):
        def decode(code):
            return {10: 0, 11: 1}.get(code)
        for raw, outcome, reason in ((10, o.PASSED, ""),
                                     (11, o.FAILED, "exit 1"),
                                     (1, o.BLOCKED, "sandbox exit 1"),
                                     (12, o.BLOCKED, "sandbox exit 12")):
            got = execute.run_check("unit-tests", ["x"], self.tmp.name, 60,
                                    run=FakeRun(raw), clock=clock(0.0, 1.0),
                                    decode=decode)
            self.assertEqual((got["outcome"], got["reason"], got["exit"]),
                             (outcome, reason, raw))

    def test_secret_input_goes_to_standard_input(self):
        # Review 2 finding C: the run's nonce is not on the command line.
        fake = FakeRun(0)
        execute.run_check("unit-tests", ["x"], self.tmp.name, 60, run=fake,
                          clock=clock(0.0, 1.0), stdin_data=b"n1\n")
        kwargs = fake.calls[0][1]
        self.assertEqual(kwargs["input"], b"n1\n")
        self.assertNotIn("stdin", kwargs)

    def test_backstop_stops_the_sandbox_only_on_timeout(self):
        # Review finding 14: the unit itself is stopped, not just waited on.
        stopped = []
        for fake in (FakeRun(exc=subprocess.TimeoutExpired(["x"], 90)),
                     FakeRun(0)):
            execute.run_check("unit-tests", ["x"], self.tmp.name, 60,
                              run=fake, clock=clock(0.0, 1.0),
                              stop=lambda: stopped.append("stop"))
        self.assertEqual(stopped, ["stop"])

    def test_judge_has_the_last_word(self):
        def judge(path, outcome, reason):
            with open(path, "rb") as handle:
                return o.FAILED, f"{outcome} {reason} {handle.read()!r}"
        got = execute.run_check("unit-tests", ["x"], self.tmp.name, 60,
                                run=FakeRun(0, b"out"),
                                clock=clock(0.0, 1.0), judge=judge)
        self.assertEqual((got["outcome"], got["reason"]),
                         (o.FAILED, "PASSED  b'out'"))

    def test_unsafe_identifier_refused(self):
        for bad in ("../x", "A", "", "a b", "a/b"):
            with self.assertRaisesRegex(ValueError,
                                        "^unsafe check identifier: ",
                                        msg=bad):
                execute.run_check(bad, ["x"], self.tmp.name, 5,
                                  run=FakeRun(), clock=clock(0.0, 1.0))

    def test_timing_is_kept_to_the_millisecond(self):
        """Retrofit (mutation closure)."""
        got = self.check(FakeRun(0), times=(10.0, 12.12345))
        self.assertEqual(got["seconds"], 2.123)


if __name__ == "__main__":
    unittest.main()
