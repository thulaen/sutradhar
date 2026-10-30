"""Command line for the check runner (SPEC-001; trust boundaries).

run: check one exact candidate tree under a profile and write a signed
bundle (see tools.runner.orchestrate). verify: check a bundle against a
protected policy, an expected tree, profile and base (always given, "none"
for a root commit), and the policy's maximum age; the status, the profile
choice and the bootstrap scope are recomputed, never trusted. Exit codes:
0 PASSED, 1 FAILED, 2 BLOCKED or usage error.

The host file (private, never in Git) names the command prefix used to
start sandboxes and where each pinned tool is installed. The evidence
file names interim evidence for checks not yet active, and review records.
"""
import datetime
import hashlib
import json
import secrets
import subprocess
import sys
import traceback

from tools.runner import (bundle, candidate, orchestrate, outcomes, policy,
                          signing)

USAGE = {"run": (("repo", "tree", "profile", "policy", "host", "key", "out"),
                 ("base", "evidence")),
         "verify": (("bundle", "signature", "signers", "identity", "policy",
                     "tree", "profile", "base"), ())}
_EXITS = {outcomes.PASSED: 0, outcomes.FAILED: 1}
Blocked = orchestrate.Blocked


def _options(args):
    if not args or args[0] not in USAGE:
        raise Blocked("usage: run|verify --option value ...")
    needed, optional = USAGE[args[0]]
    pairs, found = args[1:], {}
    for key, value in zip(pairs[::2], pairs[1::2]):
        name = key[2:]
        allowed = key.startswith("--") and name in needed + optional
        if not allowed or name in found:
            raise Blocked(f"usage: unknown or repeated option {key}")
        found[name] = value
    if len(pairs) % 2 or not set(needed) <= set(found):
        raise Blocked(f"usage: {args[0]} needs --" + " --".join(needed))
    return args[0], found


def _recheck(loaded, name):
    """Recompute, from the verifier's policy, what the runner decided."""
    def extra(doc):
        try:
            policy.select(loaded, name, doc["changed"],
                          doc["changed_executable"])
        except policy.PolicyError as error:
            return [str(error)]
        if policy.in_scope(loaded, doc["changed"]) != doc["bootstrap"]:
            return ["bootstrap differs from the policy's scope"]
        return []
    return extra


def _verify(opts, now):
    raw = orchestrate.read(opts["policy"])
    loaded = policy.load(raw)
    name = opts["profile"]
    if name not in loaded["profiles"]:
        raise Blocked(f"unknown profile: {name}")
    # The base is always required: the change, and so the profile choice
    # and the bootstrap scope, depend on it. "none" means a root commit.
    expect = {"tree": opts["tree"], "profile": name,
              "plan": policy.plan(loaded, name),
              "policy_sha256": hashlib.sha256(raw).hexdigest(),
              "runner_sha256": loaded["runner"],
              "base": None if opts["base"] == "none" else opts["base"]}

    def trust(data, signature):
        return signing.verify(opts["signers"], opts["identity"], data,
                              signature)
    status, reasons = bundle.verify(
        orchestrate.read(opts["bundle"]), orchestrate.read(opts["signature"]),
        trust, expect, now(), loaded["max_age"], _recheck(loaded, name))
    return {"status": status, "reasons": reasons}


def _utc_now():
    return datetime.datetime.now(datetime.timezone.utc)


def _token():
    return secrets.token_hex(8)


def main(argv, out=sys.stdout, run=subprocess.run, now=_utc_now,
         token=_token):
    """Entry point; returns the process exit code."""
    try:
        command, opts = _options(argv)
        report = (orchestrate.run_candidate(opts, run, now, token)
                  if command == "run" else _verify(opts, now))
    except (Blocked, policy.PolicyError, candidate.CandidateError,
            signing.SigningError, OSError) as error:
        report = {"status": outcomes.BLOCKED, "blocked": [str(error)]}
    except Exception as error:  # report, never crash into a pass
        traceback.print_exc()  # goes to standard error
        report = {"status": outcomes.BLOCKED,
                  "blocked": [f"internal error: {type(error).__name__}"]}
    out.write(json.dumps(report, indent=1, sort_keys=True))
    return _EXITS.get(report["status"], 2)


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
