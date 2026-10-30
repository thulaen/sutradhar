"""Evidence signatures with the system's OpenSSH (ssh-keygen -Y).

Signatures are made in OpenSSH's signature format under a fixed
namespace, so a signature made for another purpose (a Git commit, a
file) never verifies as evidence. The private key stays with the runner
account; verifiers need only an allowed-signers file naming the runner.
No Python cryptography package is needed.
"""
import re
import subprocess
import tempfile

NAMESPACE = "sutradhar-evidence"
_IDENTITY = re.compile(r"[a-z0-9][a-z0-9-]{0,63}")
_KEY = re.compile(r"[A-Za-z0-9+/]+={0,2}")
_KEY_TYPES = ("ssh-ed25519",)


class SigningError(Exception):
    """The signature could not be made."""


def sign(key, data, run=subprocess.run):
    """An armoured signature over data, made with the private key file."""
    done = run(["ssh-keygen", "-Y", "sign", "-f", key, "-n", NAMESPACE],
               input=data, capture_output=True)
    if done.returncode or not done.stdout:
        raise SigningError("ssh-keygen could not sign")
    return done.stdout


def verify(signers, identity, data, signature, run=subprocess.run):
    """Whether signature is identity's evidence signature over data."""
    with tempfile.NamedTemporaryFile() as handle:
        handle.write(signature)
        handle.flush()
        done = run(["ssh-keygen", "-Y", "verify", "-f", signers,
                    "-I", identity, "-n", NAMESPACE, "-s", handle.name],
                   input=data, capture_output=True)
    return done.returncode == 0


def signer_line(identity, key_type, key):
    """One allowed-signers line limited to the evidence namespace."""
    if not (_IDENTITY.fullmatch(identity) and key_type in _KEY_TYPES
            and _KEY.fullmatch(key)):
        raise ValueError("unsafe allowed-signers entry")
    return f'{identity} namespaces="{NAMESPACE}" {key_type} {key}\n'
