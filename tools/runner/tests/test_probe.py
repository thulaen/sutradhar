"""Sandbox refusal probe (AT-TB01); real refusals are qualified on Dell."""
import contextlib
import errno
import io
import json
import os
import runpy
import socket
import subprocess
import sys
import tempfile
import unittest
from unittest import mock

from tools.runner import probe

USAGE = ("usage: [--read FILE]... [--write FOLDER]... "
         "[--connect SOCKET]...\n")


def allowed():
    return None


def refused():
    raise PermissionError(errno.EACCES, "no")


def raiser(error):
    def act():
        raise error
    return act


def broken():
    raise NameError("bug in the probe")


def run_main(argv, made):
    out = io.StringIO()
    code = probe.main(argv, out, make=lambda *lists: made)
    return code, [json.loads(line) for line in out.getvalue().splitlines()]


class ProbeTest(unittest.TestCase):
    def test_every_refusal_exits_zero_and_is_reported(self):
        code, lines = run_main([], [("a", refused), ("b", refused)])
        self.assertEqual(code, 0)
        self.assertEqual(lines, [
            {"attempt": "a", "refused": True, "error": "EACCES"},
            {"attempt": "b", "refused": True, "error": "EACCES"}])

    def test_one_success_fails(self):
        code, lines = run_main([], [("a", refused), ("b", allowed)])
        self.assertEqual(code, 1)
        self.assertEqual(lines[1], {"attempt": "b", "refused": False,
                                    "error": None})

    def test_no_attempts_is_not_a_pass(self):
        self.assertEqual(run_main([], [])[0], 1)

    def test_probe_bugs_are_not_refusals(self):
        with self.assertRaises(NameError):
            probe.attempt([("a", broken)])

    def test_only_operating_system_refusals_count(self):
        # Review finding 6: "already exists", "connection refused", a
        # time-out or an error without a code is not a refusal.
        cases = [(PermissionError(errno.EACCES, "x"), True, "EACCES"),
                 (PermissionError(errno.EPERM, "x"), True, "EPERM"),
                 (OSError(errno.EROFS, "x"), True, "EROFS"),
                 (OSError(errno.EAFNOSUPPORT, "x"), True, "EAFNOSUPPORT"),
                 (FileNotFoundError(errno.ENOENT, "x"), True, "ENOENT"),
                 (FileExistsError(errno.EEXIST, "x"), False, "EEXIST"),
                 (ConnectionRefusedError(errno.ECONNREFUSED, "x"), False,
                  "ECONNREFUSED"),
                 (PermissionError("no code"), False, "PermissionError"),
                 (subprocess.TimeoutExpired(["sudo"], 10), False,
                  "TimeoutExpired")]
        for error, refusal, name in cases:
            got = probe.attempt([("t", raiser(error))])[0]
            self.assertEqual((got["refused"], got["error"]),
                             (refusal, name), error)

    def test_trials_cover_every_kind_of_attempt(self):
        names = [name for name, _ in probe.trials(["/k", "/s"], ["/w"],
                                                   ["/c"])]
        self.assertEqual(names, ["read /k", "read /s", "write /w",
                                 "connect /c", "network ipv4",
                                 "network ipv6", "privileges"])

    def test_options_reach_the_trials(self):
        seen = []

        def make(*lists):
            seen.append(lists)
            return [("a", refused)]
        code = probe.main(["--read", "a", "--write", "b", "--read", "c",
                           "--connect", "d"], io.StringIO(), make=make)
        self.assertEqual((code, seen), (0, [(["a", "c"], ["b"], ["d"])]))

    def test_bad_options_are_usage_errors(self):
        for argv in (["--read"], ["--colour", "x"], ["a", "b"],
                     ["--read", "a", "--write"]):
            out = io.StringIO()
            self.assertEqual(probe.main(argv, out), 2, argv)
            self.assertEqual(out.getvalue(), USAGE)

    def test_report_lines_are_sorted_json(self):
        out = io.StringIO()
        probe.main([], out, make=lambda *lists: [("a", refused)])
        self.assertEqual(out.getvalue(), '{"attempt": "a", "error": '
                         '"EACCES", "refused": true}\n')


class ActionTest(unittest.TestCase):
    def test_read_and_write_leave_nothing_behind(self):
        with tempfile.TemporaryDirectory() as folder:
            probe.write(folder)
            probe.write(folder)
            self.assertEqual(os.listdir(folder), [])
            path = os.path.join(folder, "f")
            with open(path, "wb") as handle:
                handle.write(b"x")
            probe.read(path)
            with self.assertRaises(FileNotFoundError):
                probe.read(os.path.join(folder, "missing"))

    def test_write_happens_inside_the_folder(self):
        with tempfile.TemporaryDirectory() as folder, \
                mock.patch("os.unlink", wraps=os.unlink) as unlink:
            probe.write(folder)
        self.assertEqual(os.path.dirname(unlink.call_args[0][0]), folder)

    def test_read_and_write_trials_use_their_paths(self):
        with tempfile.TemporaryDirectory() as folder:
            trial = dict(probe.trials([os.path.join(folder, "missing")],
                                      [folder], []))
            with mock.patch("os.unlink", wraps=os.unlink) as unlink:
                report = probe.attempt([(name, trial[name]) for name in trial
                                        if name.split()[0] in ("read",
                                                               "write")])
            self.assertEqual(unlink.call_count, 1)
        self.assertEqual([(e["refused"], e["error"]) for e in report],
                         [(True, "ENOENT"), (False, None)])

    def test_connect_tells_a_listener_from_a_refusal(self):
        with tempfile.TemporaryDirectory() as folder:
            path = os.path.join(folder, "s")
            server = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
            self.addCleanup(server.close)
            server.bind(path)
            server.listen(1)
            trial = dict(probe.trials([], [], [path, path + "x"]))
            report = probe.attempt([(f"connect {p}", trial[f"connect {p}"])
                                    for p in (path, path + "x")])
        self.assertEqual([(e["refused"], e["error"]) for e in report],
                         [(False, None), (True, "ENOENT")])

    def test_opening_any_internet_socket_is_the_attempt(self):
        opened = []

        class Socket:
            def __init__(self, family, kind):
                opened.append((family, kind))

            def close(self):
                opened.append("closed")
        trial = dict(probe.trials([], [], []))
        with mock.patch("socket.socket", Socket):
            trial["network ipv4"]()
            trial["network ipv6"]()
        self.assertEqual(opened, [
            (socket.AF_INET, socket.SOCK_STREAM), "closed",
            (socket.AF_INET6, socket.SOCK_STREAM), "closed"])

    def test_privileges(self):
        calls = []

        def fake(code):
            def run(argv, **kwargs):
                calls.append((argv, kwargs))
                return subprocess.CompletedProcess(argv, code)
            return run
        self.assertIsNone(probe.privileges(run=fake(0)))
        with self.assertRaises(PermissionError) as caught:
            probe.privileges(run=fake(1))
        self.assertEqual((caught.exception.errno, caught.exception.strerror),
                         (errno.EPERM, "sudo refused"))
        self.assertEqual(calls[0], (["sudo", "-n", "true"], {
            "capture_output": True, "timeout": 10,
            "stdin": subprocess.DEVNULL}))


class EntryTest(unittest.TestCase):
    def test_module_entry(self):
        captured = io.StringIO()
        with mock.patch.object(sys, "argv", ["probe", "--read"]), \
                contextlib.redirect_stdout(captured), \
                self.assertRaises(SystemExit) as stop:
            runpy.run_module("tools.runner.probe", run_name="__main__")
        self.assertEqual(stop.exception.code, 2)
        self.assertEqual(captured.getvalue(), USAGE)


if __name__ == "__main__":
    unittest.main()
