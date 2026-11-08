# 0008: Trust boundaries

This part sets who may run what on Dell, so that code being judged never
acts with the power of the checks judging it. It is shared by the door
(0001), the check runner (0003), the sync (0006) and the gate checks
(0007). It is roadmap Phase 1 item 9 (the sync) and Phase 2 item 1
(bringing the gate code over).

## What it does

### The principle

Code being judged never runs with the authority of the judge. Every
decision that Forgejo's door, the sync or autoissues relies on is made
from signed reports, by an account that never runs candidate code.

### Accounts on Dell

1. **Candidate box.** A fresh throwaway container per run, with no
   network, a read-only view of the candidate and its admitted
   dependencies, a private writable scratch area, no access to any key,
   socket, store, journal or other account's files, and limits on
   memory, processes and time inside the heavy-job budget. Every test,
   benchmark, coverage run, mutant run, analyser, red check and adapter
   re-run that runs candidate code runs here.
2. **Runner.** Starts the boxes, collects their reports and signs an
   evidence bundle bound to the exact tree, profile, policy version,
   environment and dependency lock. Only this account can read its
   signing key. It never runs candidate code under its own identity.
3. **Gate.** Owns the autoissues store and journal. Reads only signed
   bundles. Decides queue and quota results and signs its verdicts.
4. **Door.** Runs as its own account, as 0001 sets out. Only the door's
   account and Forgejo's may reach its socket.
5. **Sync.** Holds only the sync's GitHub token, in its own account.
6. **Release step.** Holds the encrypted release key. Every use is
   logged and copied to GitHub (AGENTS.md section 8.1).
7. **Test services.** Run in containers on a private network reachable
   only by the test-services and gate accounts.
8. **Reviewer.** Holds only the AI reviewer's sign-in. The runner starts
   each review in a reviewer box: a throwaway identity that can read only
   the public candidate and reach only the AI provider's network
   addresses, with no other key, socket or store. The runner signs the
   review result.
9. **Coding agent.** Edits code and pushes through the door. It holds no
   key of the accounts above. Separate agent accounts are preferred.
   While agents share one, independence checks rely on runner signals
   and runner-made reviews, never on agent names.

### Each checker checks for itself

1. The door checks the runner's and the gate's signatures for every
   commit in a push.
2. The gate works out the required results again from the signed
   bundles and the protected policy taken from the base commit. It never
   trusts another account's summary.
3. Protected changes are classified from that same base policy, never
   from the candidate.

### Protected policy and installed tools

1. The protected policy is always read from the base commit or the
   installed copy, never from the candidate.
2. Protected paths are the full dependency closure of the runner, gate,
   door, sync and release step: their source, the modules they use,
   their settings, the benchmark map, lock files and workflows, plus the
   rules, roadmap and specifications, and the files AGENTS.md section
   7.2 names. A change to any of them needs the owner's signature.
3. These tools are installed only by their own service account, only
   from an accepted `main` tree whose identifier is in a publication
   receipt. The installed bytes are hashed again before each use. A
   mismatch means the tool is not used and the tier is blocked.

### Administrator access on Dell

AGENTS.md section 11 gives agents sudo. So the account walls above
protect against mistakes and ordinary faults, not against a hostile
agent with administrator rights. What stays unforgeable is the owner's
signature: the owner's signing key never lives on Dell
([AGENTS.md section 7.2](../../AGENTS.md#72-waivers)). The checksum
guard also watches Forgejo's hooks and settings, the protected gate
files and the GitHub CI files, and raises an urgent task and a GitHub
alert issue when they change any other way.

### Owner approval

1. Approval is a signature made with the owner's key over a short
   statement naming the candidate tree, the commit identifiers and the
   SHA-256 of each protected file. The gate checks it against
   `gate/owner-key.pub`. No agent can make that signature.
2. The owner reads the statement on the owner's own PC before signing.
3. Owner decisions that the quota and the task lifecycle need
   (deferrals, recovery records, charge cancellations, duplicate and hot
   list decisions) use the same route, naming the record they approve.
4. A Forgejo task may tell the owner that a signature is wanted. The task
   itself grants nothing.

### Test services and signals

1. Webhook secrets and service credentials live in settings only the
   gate can read.
2. Webhooks use HMAC signing where the tool supports it.
3. A webhook alone never proves that a signal is absent. To confirm, the
   gate account asks the source service directly.

## Limits

- Candidate boxes run inside the Dell heavy-job budget: at most two at
  once, each held to 4 GB of memory and 8 CPU threads (AGENTS.md
  section 11).
- Each box has limits on memory, processes and time, and a box failure
  is BLOCKED (0003).

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Rust tests in the runner, gate and door crates, with planted faults:

- A hostile test in the box tries to read the sync token, the runner
  key, the store, the journal and the gate socket; every attempt fails
  and is reported.
- A bundle with a forged or missing runner signature is refused by the
  door and the gate.
- The gate works out a failing result from the bundle even when the push
  claims success.
- A candidate that edits the protected-path list is classified from the
  base policy and needs the owner's signature.
- A change to a module used by the gate is protected.
- An installed gate whose bytes differ from the receipt is not used and
  blocks the tier.
- A signature from another key, over a different tree or over different
  protected-file hashes, fails; owner records follow the same rule.
- A monitoring alert's "resolved" message without confirmation from the
  source does not count as absence.

## Changes from the source

- Throwaway systemd identities for the box are replaced by throwaway
  containers, as the runner (0003) uses.
- The publisher, attester and approval verifier accounts with their
  GitHub Apps are replaced by the door, the sync and the release step.
- The rule that the agent account has no sudo is dropped, since
  AGENTS.md section 11 gives agents sudo; the trade-off the owner chose
  earlier (account walls guard mistakes, not hostility) now always holds.
- Approval by GitHub comment and the hardware-key choice are replaced
  by the owner's signing key of AGENTS.md section 7.2.
- Owner notices go through Forgejo tasks, not GitHub issues.
- The task runner file is no longer a protected path; Cargo files and
  `gate/` files are.

Source: `docs/specifications/trust-boundaries.md` in the archive
`archive-2026-11-08-old-repo`.
