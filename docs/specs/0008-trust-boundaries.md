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
2. **Runner** (the runner supervisor of specs 0064 and 0065). Starts the
   boxes, collects their reports and signs an evidence bundle bound to
   the exact tree, profile, policy version, environment and dependency
   lock. It signs the bundle as an `evidence-bundle` through tijori
   (spec 0065), which alone holds the runner's key; only this account
   may ask tijori for an `evidence-bundle` signature. It never runs
   candidate code under its own identity.
3. **Gate.** Owns the autoissues store and journal. Reads only signed
   bundles. Decides queue and quota results and signs its verdicts.
4. **Door.** Runs as its own account, as 0001 sets out. Only the door's
   account and Forgejo's may reach its socket.
5. **Sync.** Holds only the sync's GitHub token, in its own account.
6. **Release step.** The only account that may ask tijori (spec 0065)
   to sign with the release key, which tijori alone holds. Every use is
   logged and copied to GitHub (AGENTS.md section 8.1).
7. **Test services.** Run in containers on a private network reachable
   only by the test-services and gate accounts.
8. **Reviewer.** Holds only the AI reviewer's sign-in. The runner starts
   each review in a reviewer box: a throwaway identity that can read
   only the public candidate and reach only the AI provider's network
   addresses, with no other key, socket or store. The runner signs the
   review result as a `review-result` through tijori (spec 0065), naming
   the reviewing product in a field. It takes that product, the change's
   fingerprint and the review's name from its own record of the job and
   the reviewer sign-in it started, never from the box's output; the
   gate refuses one whose product is not an agent product, a principal
   tijori's policy allows `review-record` (spec 0065), and compares that
   product, never the runner, with the change's authors. The signed
   bytes follow spec 0064's layout: the purpose, record type and format
   version fields first, then the reviewed change's fingerprint, the
   required review it is for (AGENTS.md section 7.1), the reviewing
   product and the verdict, each a length-prefixed field. This spec owns
   that record type. The gate refuses a `review-result` whose
   fingerprint is not the change being checked, or whose review is not
   the one being counted.
9. **Coding agent.** Edits code and pushes through the door. Managed
   agents use the locked account and harness under AGENTS.md section 11.
   Service-account keys follow the custody rules above. For shared
   sessions permitted by section 11, establish product independence
   solely from runner signals and runner-made reviews.

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
   door, sync, release step, the `tijori` program and `tijori-verify`
   (spec 0065): their source, the modules they use, their settings, the
   benchmark map, lock files and workflows, plus the rules, roadmap and
   specifications, and the files AGENTS.md section 7.2 names.
   Classify the change using the protected base policy. Apply
   [AGENTS.md section 7.2](../../AGENTS.md#72-waivers) to
   waivers and the protected changes it names. The separate
   owner-signature requirements for policy and trust changes
   follow [AGENTS.md section 8.1](../../AGENTS.md#81-releases).
   The owner-signed policy and its activation and verification
   follow [0065](0065-tijori.md#the-signer-registry). A runner
   or gate cutover follows the signed record under
   [0003](0003-toolchain-and-check-runner.md#cutover-and-dependency-evidence).
3. These tools are installed only by their own service account, only
   from an accepted `main` tree whose identifier is in a publication
   receipt. The installed bytes are hashed again before each use. A
   mismatch means the tool is not used and the tier is blocked.

### Administrator access on Dell

[AGENTS.md section 11](../../AGENTS.md#11-the-shared-computer-dell)
defines managed accounts and the permitted shared sessions. A managed
agent uses a locked account without sudo, Docker or LXD group membership
or a token. It reaches Forgejo, the mailbox and git through the harness.
The owner keeps root. The shared sudo account serves the owner's work
and recovery. Until locked accounts are ready, the owner-started
sessions and helpers that section 11 permits use that account as
unmanaged sessions.

The account walls protect against mistakes and ordinary faults. Dell
administrator access sits outside that protection. The owner's signing
key stays off Dell under
[AGENTS.md section 7.2](../../AGENTS.md#72-waivers). The checksum guard
also watches Forgejo's hooks and settings, the protected gate files
and the GitHub CI files, and raises an urgent task and a GitHub alert
issue when they change outside the required route.

### Owner approval

1. A required owner approval for a protected source change is
   a signature made with the owner's key over the candidate
   tree, the commit identifiers and the SHA-256 of each
   protected file. The gate checks it against `gate/owner-key.pub`.
   No agent can make that signature. Owner-signed policy and
   other records use their owning encodings.
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
  key, the store, the journal and the gate socket, and to reach tijori's
  socket; every attempt fails and is reported.
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
- Managed coding accounts and permitted unmanaged shared sessions
  follow AGENTS.md section 11. Dell administrator access remains
  outside the account protection.
- Approval by GitHub comment and the hardware-key choice are replaced
  by the owner's signing key of AGENTS.md section 7.2.
- Owner notices go through Forgejo tasks, not GitHub issues.
- The task runner file is no longer a protected path; Cargo files and
  `gate/` files are.

Source: `docs/specifications/trust-boundaries.md` in the archive
`archive-2026-11-08-old-repo`.
