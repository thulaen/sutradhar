# SPEC-006 part 2: bootstrap, activation, repair and recovery

Status: APPROVED, revision 2. Part of [SPEC-006](quota.md). Every case
here exists so that no gate can deadlock the project and no failure can
silently erase debt (R094, R093).
Approved by the owner on 30 October 2026 (see the approval record in
`docs/specifications/approvals-2026-10-30.md`); it supersedes any
"awaiting approval" wording below.

## Bootstrap and activation order

Before a gate's checker is qualified, it reports NOT_YET_ACTIVE. That
outcome is added to SPEC-001's set and accepted at commit, push and
publish only for candidates inside the bootstrap scope, with the
interim result the activation table names. Outside that scope it is
BLOCKED (AT-G15).

Bootstrap scope: the foundation, requirement and specification records,
documentation compliance, the source-limit checker, the SPEC-001 runner,
the test-first gate, the SPEC-007 performance harness, the sandbox and
signing tools, the SPEC-004 publisher and AutoIssues milestones 1 to 5.
Product features wait until the end of the sequence.

Activation happens in this order. Each step is a local milestone commit,
judged by the gates already active plus the interim results named here.
Publication begins when the publisher activates (step 5): all earlier
milestones are then published in one fast-forward under SPEC-004's
bootstrap publication rule, which re-runs and signs their checks.

1. Source-limit checker. Interim before it: a Dell measurement by the
   coordinator with the approved counting rule and the owner's exceptions,
   re-run and signed by the runner once the runner activates.
2. SPEC-001 runner, sandbox and signing. Interim: the existing
   documentation profile plus independent review.
3. Test-first gate. Interim: the red and green runs recorded by the runner.
4. SPEC-007 performance harness. Interim: a reviewed limit report per
   changed unit, based on recorded Dell measurements, as SPEC-007 result B
   requires; re-measured by the harness once it is active.
5. SPEC-004 publisher, after qualification on the test repository.
6. AutoIssues milestones 1 to 5; milestone 5 activates the queue and quota
   gate.

Each activation writes a record into the publisher's protected policy and
derives from what is on `main`, so recovery cannot forget it. Once a
gate is active, NOT_YET_ACTIVE for it is refused permanently (AT-G16).
The AutoIssues activation record stores the store's genesis journal hash
and first anchored head, so "verified zero" has something to extend.

## External journal and anchors

Every write's full payload is appended to an external journal file owned
by the gate account, outside the database and synced to disk before the
write is acknowledged. The file is append-only at the filesystem level.
After every charge, credit, revocation and clearance, at every stage, the
gate sends the new head (sequence number and hash) to the publisher's
protected anchor log; publication also records it (AT-G17, AT-G22).

## Store recovery

A damaged database is rebuilt by restoring the newest verified snapshot
and replaying the external journal from it. The rebuilt store must reach
the last anchored head exactly. If the journal itself is damaged, the gate
stays BLOCKED and asks for an owner-approved recovery record. That record may
only keep debt conservatively: it carries forward the highest debt and
the lowest credit recorded in any surviving anchor, never less debt
(AT-G23).

## Gate repair

A candidate that changes only the gate and its dependencies uses the gate
repair lane:

- it is judged by the previous qualified gate version, kept immutably on
  Dell;
- if that version cannot run, the candidate gate runs in a sandbox
  against a replayed copy of the journal, and must reproduce every
  earlier decision except the ones its fix changes, which are listed for
  review;
- migrations must remain readable by the previous version, so falling
  back stays possible;
- at the very first activation there is no previous version, so the
  milestone 5 commit is judged under the bootstrap interim results.

If the installed gate is shown to have charged wrongly, the owner may
approve a "charge annulled: gate defect" record naming the charges and the
evidence. It is journalled, never deletes the original charge, and is the
only way to correct a charge. It is not available for quality failures
(AT-G13, AT-G14, AT-G24).

## Acceptance cases

- AT-G15: NOT_YET_ACTIVE is accepted only for bootstrap-scope candidates,
  at commit, push and publish.
- AT-G16: after a gate's activation, NOT_YET_ACTIVE for it is refused,
  including after publisher recovery.
- AT-G17: restoring an older snapshot that drops anchored debt is refused.
- AT-G22: a charge recorded at the commit stage and never published is
  still anchored and survives a database restore.
- AT-G23: with the journal damaged, nothing passes until an owner-approved
  recovery record exists, and that record cannot lower debt.
- AT-G24: a charge annulment without the owner's approval comment, or for a
  quality failure, is refused.
