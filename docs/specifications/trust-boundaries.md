# Trust boundaries for gates, publication and AutoIssues

Status: APPROVED. Shared by SPEC-004, SPEC-005, SPEC-006 and SPEC-007;
added after an independent review found that the first drafts let
candidate code act with the authority of the checks judging it.
Awaiting the owner's approval with those designs.
Approved by the owner on 30 October 2026 (see the approval record in
`docs/specifications/approvals-2026-10-30.md`); it supersedes any
"awaiting approval" wording below.

## Principle

Code being judged never runs with the authority of the judge. Every
decision that GitHub or the quota relies on is made from signed reports by
an account that never executes candidate code.

## Accounts on Dell

- Candidate sandbox: a fresh throwaway identity per run (a systemd
  transient service with a dynamic user), with no network, a read-only
  view of the candidate and its admitted dependencies, a private writable
  scratch area, no access to any key, socket, store, journal or other
  account's files, and memory, process and time limits inside the lab
  budget. All tests, benchmarks, coverage, mutation, analysers, red checks
  and adapter re-runs that execute candidate code run here.
- Runner (`sutradhar-runner`): launches sandboxes, collects their reports
  and signs an evidence bundle bound to the exact tree, profile, policy
  version, environment and dependency lock. Its signing key is readable
  only by this account. It never executes candidate code in its own
  identity.
- Gate (`sutradhar-gate`): owns the AutoIssues store and journal; reads only
  signed bundles; decides queue and quota results; signs its verdicts.
- Publisher, Attester and approval verifier: one account each, each holding
  only its own GitHub App key (SPEC-004).
- QA (`sutradhar-qa`): runs QA services with rootless Podman in a private
  network namespace reachable only by the QA and gate accounts.
- Reviewer (`sutradhar-reviewer`): holds only the AI reviewer's sign-in.
  The runner starts the AI review in a reviewer sandbox: a throwaway
  identity that can read only the public candidate and reach only the AI
  provider's network endpoints, with no other key, socket or store. The
  runner signs the review result.
- Coding agent (`sutradhar-agent`): edits and submits requests. No
  administrator rights, no docker group, no route to any account above.
  Separate agent accounts are preferred; while agents share one,
  independence checks rely on adapter signals and the runner-produced AI
  review, never on agent names.

Each verifier checks signatures itself: the Publisher checks the runner's
and gate's signatures for every commit in a request; the Attester
independently recomputes the required-check results from the signed
bundles and the protected policy taken from the base commit; the approval
verifier classifies the change from that same base policy. None trusts
another's summary (AT-TB01 to AT-TB04).

## Protected policy and installed tools

The protected policy is always read from the base commit or the installed
copy, never from the candidate. Protected paths are the full dependency
closure of the runner, gate, publisher, Attester and verifier: their source,
imported modules, configuration, `justfile`, `benchmarks/map.yaml`,
lockfiles and workflows, plus rules, requirements and specifications.
A change to any of them needs owner approval (AT-TB05).

These tools are installed only by their own service account, only from a
published `main` tree whose identifier is in a publication receipt, and the
installed bytes are re-hashed before each use (AT-TB06).

## Host precondition

Account separation means nothing if an agent can become root or another
service account. Before activation, the owner confirms that the coding
agent account has no `sudo`, no docker group membership and no other route
to root or the service accounts, with refusal tests (AT-TB07).

The Dell host is shared with another project whose operator account has
passwordless administrator access. Whoever controls that account controls
every key on the host. The owner must choose:

- (a) remove passwordless administrator access from any account an agent
  session can use, on this host, performing administrator steps personally
  (recommended);
- (b) keep it, accepting that the separations above protect against
  mistakes and ordinary faults but not against a deliberately hostile
  agent with administrator access. Only the hardware-signed owner approval
  below remains unforgeable then.

## Owner approval

Owner decision, 30 October 2026: the owner chose the GitHub-comment
alternative described at the end of this section, and kept passwordless
administrator access (host option b). The approval record states the
consequences. The hardware-key design below is retained as the stronger
option the owner may adopt later.

Approval is a signature made with the owner's hardware security key
(a FIDO2 key that needs a physical touch) over a short statement naming the
candidate tree, the commit identifiers and the SHA-256 of each protected
file. The owner double-clicks a provided approval file on the editing
computer, reads the statement and touches the key. The approval verifier
checks the signature against the owner's public key recorded in the
protected policy. No agent can produce that signature (AT-TB08).

Owner decisions that the quota and lifecycle need (deferrals, recovery
records, charge annulments, duplicate and hotlist decisions) use the same
route as approvals, naming the record they approve.

The approval verifier opens a GitHub issue to notify the owner (it gets
Issues write for this); the issue itself grants nothing. Alternative, if the
owner has no security key: a comment from the owner's account, accepted
only if the editing computer's GitHub credential is reduced to read-only
and agents have no browser session signed into the owner's account. That
alternative is weaker and not recommended.

## QA services and signals

Webhook secrets and service credentials live in gate-only configuration.
Webhooks use HMAC signing where the tool supports it. A webhook never
proves a signal is absent by itself; verification queries the source
service directly through the gate account (AT-TB09).

## Acceptance cases

- AT-TB01: a hostile test in the sandbox tries to read an App key, the
  runner key, the store, the journal and the gate socket; every attempt
  fails and the attempt is reported.
- AT-TB02: a bundle with a forged or missing runner signature is refused
  by the Publisher and the Attester.
- AT-TB03: the Attester recomputes a failing result from the bundle even
  when the Publisher's request claims success.
- AT-TB04: a candidate that edits the protected-path list is classified
  from the base policy and needs owner approval.
- AT-TB05: a change to a module imported by the gate is protected.
- AT-TB06: an installed gate whose bytes differ from the receipt is not
  used and blocks the stage.
- AT-TB07: the agent account's `sudo`, docker and account-switch attempts
  are refused.
- AT-TB08: an approval comment from another account, edited or deleted,
  or over a different tree, fails; owner records follow the same rule.
- AT-TB09: Alertmanager "resolved" payloads without source confirmation do
  not count as absence.
