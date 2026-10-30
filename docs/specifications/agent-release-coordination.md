# SPEC-003: agent, repository and release coordination

Status: APPROVED for bounded implementation. See [approval
record](approvals.md).

Audience: owner, maintainers and implementation agents. Outcome: agents can
collaborate unattended inside approved specifications while the owner's
protected decisions, private data and release authority stay enforced.

Requirements: R045–R046, R057–R066 and R068–R075.

## Trust boundaries

Automation uses dedicated GitHub App identities rather than the owner's personal
credentials. Separate proposal writing, trusted status/approval reporting and
final release publication. Private keys and installation tokens stay with a
protected publisher on the validation host, outside candidate checkouts.
Ordinary coding workers receive neither those credentials nor ambient owner
credentials.

Workers submit bounded patch/branch/PR requests to that publisher. The publisher
validates the worker identity, reservation, approved scope, content manifest and
required evidence before a write. It exposes no generic GitHub API proxy, shell
execution or arbitrary ref-update operation. Proposal access must not permit tag
creation, main writes, workflow weakening or publication through an existing
tag. Contents-write permission alone is not a sufficiently narrow worker
boundary.

The check publisher accepts results only from qualified runners and current
policy. The owner-approval publisher verifies a real owner decision against the
specific specification/rule change and its version. A worker cannot impersonate
that decision with a label, self-written file, forged report or PR comment.

## Joining, ownership and records

Agents join explicitly through qualified adapters. A2A messages are transport,
not permission to edit another agent's files or approve its changes. Record
identity, assigned scope, specification approval, current lease and activity in
a private SQLite run ledger on the validation host.

Allow one active writer per file. Overlapping requests are rejected or queued.
To hand over, preserve the draft and checkpoints, record the reason, revoke the
old lease and issue a new one. A stale worker is fenced from subsequent writes,
even if it resumes after its replacement starts. Do not discard another worker's
staging or uncommitted work to make a task easier.

The ledger records runs, attempts, ownership, evidence references, publication
state and handovers. It contains no copied provider credentials. Publish only
scrubbed summaries. Use durable transactions and a recovery check before
resuming a partially completed write; retries must not create duplicate commits,
PRs, tags, releases or manual deployments.

## Repository admission

Main accepts changes through PRs only, with no direct/force pushes or deletion.
Use squash merges and up-to-date branches. Require the agreed contexts: just
check; tests; edge-case suite; import-linter; security scans; manual checks;
AI code review; owner approval; Dell availability.

An ordinary trusted implementation PR inside an approved specification may
auto-merge after all applicable requirements pass. The owner-approval context
may truthfully record that no new protected decision is involved, but cannot
pretend an unapproved specification is already approved. Protected rules,
privacy, checks, release policy, trust admission and feature specifications need
the owner's explicit decision. No bypass identity may waive those checks.

Forks use isolated, read-only public/synthetic checks without private
credentials or private corpus access. The owner approves the exact outside
candidate before its code executes on the validation host and makes its merge
decision, unless the contributor was explicitly admitted to the protected
trusted list. A read-only fork token alone does not enforce the merge policy.

Recognised Dependabot and Release Please proposal branches have only their
documented pre-commit timing exception. They still receive the full applicable
checks before merging. Every ordinary agent commit needs the full pre-commit
profile. If main becomes broken, open a revert PR immediately rather than using
an unrelated fix-forward change as the first response.

## Availability and reviews

Prove current validation-host availability before starting or admitting work;
pause when it cannot be established. An old green status does not expire by
itself and therefore is not sufficient proof. The implementation must qualify
the availability monitor, lease expiry, job pause and external-bot scheduling
behaviour before claiming the offline rule is enforced.

A control-plane health check may detect restoration and report state; it must
not continue ordinary project work during the pause. Retain queued work and
durable progress. Required-service outages pause the manual deadlines and leave
release gates incomplete.

GitHub AI review must identify the current candidate and complete successfully.
The integration must be verified against actual review output and converted into
a trusted blocking result where necessary. A reaction, silence, stale review,
incomplete scope or conditionally skipped job is not success. Keep the separate
engineering code/performance/security review evidence and label self-review.

## Release sequence

Release Please prepares the weekly release proposal. Incomplete first-release
scope, including the 6,000 qualified method requirement, blocks admission even
when other checks pass. The trusted release service, not an ordinary agent,
creates a candidate tag for the exact prepared commit and starts its durable
release run. A final public release remains a draft until every required gate
has passed.

The release run performs the full fixed/fresh quality assessment, required
resource/environment checks and 72-hour supervised soak. It resumes across
provider usage windows and records exact content/model/rubric/data identities.
It must not rerun or publish a changed candidate using old evidence.

The release checks also require no regression in reconstruction recall or anchor
agreement, alongside hard-rule and opportunity-coverage requirements. After
launch, include the review-page acceptance rate over the latest 200 decisions.
Its metric definition and decision threshold must be approved in the evaluation
specification; do not invent a threshold or call this check passed while that
contract is missing. Deterministic nightly runs retain reconstruction recall,
anchor agreement, hard-rule and coverage reporting without invoking AI judges.

After success, the separate final publisher creates the final tag/release for
the same qualified code, publishes verified artifacts and advances the matching
manual version/latest alias. Any packaging/version metadata change that affects
the tested artifact needs matching qualification; it cannot be hidden under the
phrase same code. Keep failed candidates and their scrubbed failure summaries.

## Acceptance examples

- Two agents request the same file: only one receives a valid write lease.
- A stopped agent returns after handover: its stale lease cannot publish
  changes.
- A worker requests a main update, tag, release or arbitrary GitHub endpoint:
  the publisher refuses it without exposing credentials.
- A worker fabricates a passing check or owner approval: trusted publication
  rejects the result and records the attempted action privately.
- An ordinary trusted PR passes approved checks: it can auto-merge; a protected
  specification change remains blocked until the actual owner approves it.
- An untrusted fork cannot run private-host code, obtain secrets or auto-merge
  merely by changing workflow files or actor labels.
- A cached availability result is stale or the validation host disappears:
  new work/admission pauses and the implementation reports the true state.
- A release candidate lacks any required scope or fails judging/soak: no final
  tag, public product release or latest-manual promotion occurs.
- A publisher crashes after creating an artifact: recovery verifies the existing
  state and completes once rather than duplicating the release.

## Approval and rollout

Approval covers the coordination/broker implementation and its enforcement
tests. It does not establish that the currently configured repository settings
already provide these runtime guarantees. Qualify the identities and refusal
paths before handing any write capability to coding workers. Enable unattended
admission only after the end-to-end rejection and recovery examples pass.
