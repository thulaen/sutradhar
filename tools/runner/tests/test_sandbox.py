"""Sandbox command construction (trust boundaries, AT-TB01)."""
import subprocess
import unittest

from tools.runner import sandbox

LIMITS = {"memory": "1G", "tasks": 64, "seconds": 300}


class CommandTest(unittest.TestCase):
    def build(self, **extra):
        args = {"prefix": ["sudo", "-n"], "unit": "sutradhar-check-1",
                "limits": LIMITS, "binds": [("/w/c", "/candidate")],
                "cwd": "/candidate", "env": {"LANG": "C.UTF-8"},
                "argv": ["python3", "-c", "pass"]}
        args.update(extra)
        return sandbox.command(**args)

    def props(self, argv):
        flag = "--property="
        return [a[len(flag):] for a in argv if a.startswith(flag)]

    def test_shape(self):
        argv = self.build()
        self.assertEqual(argv[:3], ["sudo", "-n", "systemd-run"])
        self.assertEqual(argv[-8:], ["--", "/bin/sh", "-c", sandbox.WRAPPER,
                                     "sutradhar-check", "python3", "-c",
                                     "pass"])
        for flag in ("--wait", "--pipe", "--collect", "--quiet",
                     "--expand-environment=no", "--service-type=exec",
                     "--unit=sutradhar-check-1", "--setenv=LANG=C.UTF-8"):
            self.assertIn(flag, argv[:-8])

    def test_wrapper_reports_the_checks_own_status(self):
        # systemd-run exits 1 when it cannot start a unit; the wrapper's
        # codes keep that from ever looking like a failing check.
        for script, code in (("exit 0", 10), ("exit 1", 11), ("exit 2", 12),
                             ("kill -9 $$", 12)):
            done = subprocess.run(["/bin/sh", "-c", sandbox.WRAPPER,
                                   "sutradhar-check", "/bin/sh", "-c",
                                   script])
            self.assertEqual(done.returncode, code, script)
        done = subprocess.run(["/bin/sh", "-c", sandbox.WRAPPER,
                               "sutradhar-check", "/nonexistent/tool"],
                              stderr=subprocess.DEVNULL)
        self.assertEqual(done.returncode, 12)

    def test_status_decoding(self):
        self.assertEqual([sandbox.check_status(code)
                          for code in (10, 11, 12, 0, 1, None)],
                         [0, 1, None, None, None, None])

    def test_no_prefix(self):
        self.assertEqual(self.build(prefix=[])[0], "systemd-run")

    def test_isolation_properties(self):
        props = self.props(self.build())
        for prop in ("DynamicUser=yes", "PrivateNetwork=yes",
                     "PrivateDevices=yes", "PrivateUsers=yes",
                     "ProtectSystem=strict", "ProtectHome=yes",
                     "NoNewPrivileges=yes", "CapabilityBoundingSet=",
                     "RestrictAddressFamilies=AF_UNIX",
                     "ProtectProc=invisible", "ProcSubset=pid", "UMask=0077",
                     "SystemCallArchitectures=native",
                     "SystemCallFilter=@system-service"):
            self.assertIn(prop, props)
        # Review 2 finding G: /tmp and /var/tmp in memory, never on disk.
        self.assertIn("PrivateTmp=disconnected", props)
        self.assertNotIn("PrivateTmp=yes", props)

    def test_host_folders_are_hidden(self):
        # Review finding 7: only the user database socket stays visible.
        props = self.props(self.build())
        for folder in ("/opt", "/run", "/var", "/srv", "/mnt", "/media"):
            self.assertIn(f"TemporaryFileSystem={folder}", props)
        self.assertIn("BindReadOnlyPaths=-/run/systemd/userdb", props)

    def test_limits_and_view(self):
        props = self.props(self.build())
        for prop in ("MemoryMax=1G", "MemorySwapMax=0", "TasksMax=64",
                     "RuntimeMaxSec=300", "WorkingDirectory=/candidate",
                     "BindReadOnlyPaths=/w/c:/candidate",
                     f"LimitFSIZE={sandbox.FILE_LIMIT}"):
            self.assertIn(prop, props)
        self.assertEqual(sandbox.FILE_LIMIT, 64 * 1024 * 1024)

    def test_candidate_sits_alone_inside_the_scan_folder(self):
        self.assertEqual((sandbox.SCAN, sandbox.CANDIDATE),
                         ("/opt/scan", "/opt/scan/candidate"))

    def test_safe_path_is_shared(self):
        self.assertEqual(sandbox.safe_path("/srv/x-1/y"), "/srv/x-1/y")
        for bad in ("relative", "/a b", "/a/../b", "/a:b", "//x/out"):
            with self.assertRaises(ValueError, msg=bad):
                sandbox.safe_path(bad)

    def test_several_binds_in_order(self):
        props = self.props(self.build(binds=[("/w/c", "/candidate"),
                                             ("/t/l", "/opt/sutradhar/l")]))
        binds = [p for p in props if p.startswith("BindReadOnlyPaths=")]
        self.assertEqual(binds, ["BindReadOnlyPaths=-/run/systemd/userdb",
                                 "BindReadOnlyPaths=/w/c:/candidate",
                                 "BindReadOnlyPaths=/t/l:/opt/sutradhar/l"])

    def test_unsafe_values_are_refused(self):
        bad = [({"binds": [("/w/a b", "/candidate")]}, "unsafe path: "),
               ({"binds": [("/w:x", "/candidate")]}, "unsafe path: "),
               ({"binds": [("relative", "/candidate")]}, "unsafe path: "),
               ({"binds": [("/w/../etc", "/candidate")]}, "unsafe path: "),
               ({"binds": [("/w/c", "/candidate/")]}, "unsafe path: "),
               ({"cwd": "candidate"}, "unsafe path: "),
               ({"env": {"A": "x\ny"}}, "unsafe environment entry: "),
               ({"env": {"A=B": "x"}}, "unsafe environment entry: "),
               ({"env": {"a": "x"}}, "unsafe environment entry: "),
               ({"unit": "bad unit"}, "unsafe unit name: "),
               ({"unit": ""}, "unsafe unit name: "),
               ({"limits": dict(LIMITS, memory="1G\nX=y")},
                "bad memory limit: "),
               ({"limits": dict(LIMITS, tasks=0)}, "bad limit: 0"),
               ({"limits": dict(LIMITS, seconds="300")}, "bad limit: '300'"),
               ({"argv": []}, "empty command")]
        for extra, message in bad:
            with self.assertRaises(ValueError) as caught:
                self.build(**extra)
            self.assertTrue(str(caught.exception).startswith(message), extra)
        self.assertEqual(str(caught.exception), "empty command")

    def test_smallest_limits_are_accepted(self):
        """Retrofit (mutation closure)."""
        props = self.props(self.build(limits={"memory": "1", "tasks": 1,
                                              "seconds": 1}))
        for prop in ("MemoryMax=1", "TasksMax=1", "RuntimeMaxSec=1"):
            self.assertIn(prop, props)


if __name__ == "__main__":
    unittest.main()
