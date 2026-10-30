"""Evidence signatures with the system's OpenSSH (ssh-keygen -Y)."""
import os
import subprocess
import tempfile
import unittest

from tools.runner import signing


def keypair(folder, name):
    key = os.path.join(folder, name)
    subprocess.run(["ssh-keygen", "-q", "-t", "ed25519", "-N", "",
                    "-C", name, "-f", key],
                   check=True, stdin=subprocess.DEVNULL)
    with open(key + ".pub", encoding="ascii") as handle:
        return key, handle.read().split()[:2]


class SigningTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tmp = tempfile.TemporaryDirectory()
        folder = cls.tmp.name
        cls.key, public = keypair(folder, "runner")
        cls.other, _ = keypair(folder, "other")
        cls.signers = os.path.join(folder, "allowed")
        with open(cls.signers, "w", encoding="ascii") as handle:
            handle.write(signing.signer_line("sutradhar-runner", *public))

    @classmethod
    def tearDownClass(cls):
        cls.tmp.cleanup()

    def verify(self, data, signature, identity="sutradhar-runner"):
        return signing.verify(self.signers, identity, data, signature)

    def test_round_trip(self):
        signature = signing.sign(self.key, b"evidence")
        self.assertTrue(signature.startswith(b"-----BEGIN SSH SIGNATURE"))
        self.assertTrue(self.verify(b"evidence", signature))

    def test_changed_data_fails(self):
        signature = signing.sign(self.key, b"evidence")
        self.assertFalse(self.verify(b"evidencf", signature))

    def test_other_key_fails(self):
        self.assertFalse(self.verify(b"x", signing.sign(self.other, b"x")))

    def test_wrong_identity_fails(self):
        signature = signing.sign(self.key, b"x")
        self.assertFalse(self.verify(b"x", signature, "sutradhar-gate"))

    def test_other_namespace_fails(self):
        done = subprocess.run(["ssh-keygen", "-q", "-Y", "sign", "-f",
                               self.key, "-n", "file"], input=b"x",
                              capture_output=True, check=True)
        self.assertFalse(self.verify(b"x", done.stdout))

    def test_garbage_and_empty_signatures_fail(self):
        self.assertFalse(self.verify(b"x", b"not a signature"))
        self.assertFalse(self.verify(b"x", b""))

    def test_sign_failure_raises(self):
        missing = os.path.join(self.tmp.name, "missing")
        with self.assertRaisesRegex(signing.SigningError,
                                    "^ssh-keygen could not sign$"):
            signing.sign(missing, b"x")

    def test_exit_status_and_output_must_both_be_good(self):
        """Retrofit (mutation closure)."""
        for code, out in ((1, b"-----BEGIN SSH SIGNATURE-----"), (0, b"")):
            fake = Recorder(code, out)
            with self.assertRaises(signing.SigningError, msg=code):
                signing.sign("k", b"x", run=fake)

    def test_commands_are_exact_and_quiet(self):
        """Retrofit (mutation closure)."""
        fake = Recorder(0, b"sig")
        self.assertEqual(signing.sign("k", b"x", run=fake), b"sig")
        self.assertTrue(signing.verify("s", "who", b"x", b"sig", run=fake))
        (sign_argv, sign_kw), (verify_argv, verify_kw) = fake.calls
        self.assertEqual(sign_argv, ["ssh-keygen", "-Y", "sign", "-f", "k",
                                     "-n", "sutradhar-evidence"])
        self.assertEqual(verify_argv[:-1], [
            "ssh-keygen", "-Y", "verify", "-f", "s", "-I", "who",
            "-n", "sutradhar-evidence", "-s"])
        self.assertEqual(fake.signatures, [b"sig"])
        for kwargs in (sign_kw, verify_kw):
            self.assertEqual(kwargs, {"input": b"x", "capture_output": True})
        self.assertFalse(signing.verify("s", "who", b"x", b"sig",
                                        run=Recorder(1, b"")))

    def test_signer_line(self):
        line = signing.signer_line("sutradhar-runner", "ssh-ed25519",
                                   "AAAAC3Nz+/=")
        self.assertEqual(line, 'sutradhar-runner namespaces="'
                         'sutradhar-evidence" ssh-ed25519 AAAAC3Nz+/=\n')

    def test_signer_line_refuses_unsafe_fields(self):
        for args in (("a b", "ssh-ed25519", "AAAA"),
                     ("x", "ssh-rsa", "AAAA"),
                     ("x", "ssh-ed25519", "AA AA"),
                     ("x", "ssh-ed25519", "")):
            with self.assertRaisesRegex(ValueError,
                                        "^unsafe allowed-signers entry$",
                                        msg=args):
                signing.signer_line(*args)


class Recorder:
    """A stand-in for subprocess.run that records each call."""

    def __init__(self, code, out):
        self.code, self.out, self.calls, self.signatures = code, out, [], []

    def __call__(self, argv, **kwargs):
        self.calls.append((argv, kwargs))
        if "verify" in argv:
            with open(argv[-1], "rb") as handle:
                self.signatures.append(handle.read())
        return subprocess.CompletedProcess(argv, self.code, self.out, b"")


if __name__ == "__main__":
    unittest.main()
