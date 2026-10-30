# SPEC-006: AutoIssues queue and quota gate

Status: APPROVED, revision 2 after independent review. A protected-gate
design that needs the owner's approval. Part of the
[AutoIssues design](README.md); bootstrap, activation, gate repair and
store recovery are in [quota recovery](quota-recovery.md). Accounts and
signatures: [trust boundaries](../trust-boundaries.md).
Approved by the owner on 30 October 2026 (see the approval record in
`docs/specifications/approvals-2026-10-30.md`); it supersedes any
"awaiting approval" wording below.

Requirements: R093, R094, R086 and R069 as amended. Signal Controller's
quota documents were read as behavioural reference only.

## Where the gate runs

The gate account evaluates every commit-profile, push-profile and
publisher attempt from signed runner bundles, using the installed gate
version and the base commit's policy. Local hooks are a convenience; the
publisher's final check (SPEC-004 step 7) cannot be skipped.

## Outcomes and classification

Every attempt records the candidate tree, stage and one outcome:

- PASSED;
- QUALITY_FAILED: a check ran and found a defect, including candidate
  crashes and timeouts not explained by host pressure. Charges debt;
- INFRA_UNAVAILABLE: no debt, action refused;
- BLOCKED: required evidence missing, stale or untrusted, or a policy
  refusal. No debt, action refused.

Rules that stop the classes being swapped:

- All runnable checks run even when another is BLOCKED. If any check ran
  and failed, the outcome is QUALITY_FAILED (AT-G19).
- Infrastructure probes come only from the installed policy and target
  services the candidate does not control. They run before and after each
  check. INFRA_UNAVAILABLE needs the after-check probe to fail too, and is
  refused when the failing service's configuration is changed by the
  candidate (AT-G20).
- A timeout is a quality failure unless the installed host-pressure probe
  shows contention from outside Sutradhar's lab slice during the run.
- Every BLOCKED and FAILED reason in SPEC-001 and SPEC-007 maps to one of
  these classes in the protected policy; an unmapped reason is BLOCKED.

## Charging

Base amount Q per charged failure: proposed 10 (owner decision). Each
charged failure while debt is unpaid adds Q: 10, 20, 30. A tree is charged
at most once; later attempts on the same tree, with the same or different
failed checks, are recorded as occurrences (AT-G05).

Every charge also files a gate-failure issue naming the failed checks, so
the defect that was charged is itself tracked and can be fixed.

A tree that failed and later passes on retry is admitted only after the
gate files a flaky_test issue for each check that changed outcome.

The round starts at the journal position of the first unpaid charge. That
position is the cutoff; clock changes cannot move it (AT-G06).

## Credit

A fix earns credit when all of these hold:

1. The issue was filed by an adapter, or is an agent finding confirmed by
   an adapter signal or a reproducer run in the sandbox. Self-confirmation
   under another label never counts.
2. It was first seen before the round's cutoff, or is that round's own
   gate-failure issue. Issues first seen after the cutoff cannot pay the
   round (stops planting defects to fix).
3. It is not a duplicate, not suppressed and not a rank-relative signal
   (for example a function entering a top-ten list because another left).
4. Its type-specific proof of fix passed in the sandbox (see
   [lifecycle](lifecycle.md#proof-of-fix)).
5. A lesson records cause, fix and prevention, and a reviewer other than
   the fixer confirmed it; the runner-produced AI review counts as such a
   reviewer.
6. The same rule or fingerprint family does not reappear in the
   candidate's changed files (stops moving code to change a fingerprint).

Credit counts per root cause: issues the review groups under one cause
earn one credit together. Credit counts when the publisher confirms the
fix on GitHub `main`. If the issue later reopens, its credit is revoked and
one unit of debt is added to the current round, so a shallow fix gains
nothing (AT-G07, AT-G08).

## Requirement and availability

Let D be the debt, C the active credits since the cutoff, and O the open
creditable issues now. O excludes an issue only when its duplicate,
suppression or deferral status was set by the gate account, an adapter
fingerprint match or an owner decision record; anything an agent set
during the round still counts. The requirement is `min(D, C + O)`. When
fewer real issues exist than the debt, fixing all of them satisfies the
round; nothing is invented (AT-G09). Meeting it records a clearance.

## Zero debt

Zero debt passes only when the gate verifies: the store opens and passes
integrity checks; the external journal is intact and extends the last
anchored head; no charge is unpaid; no closure lacks its lesson or valid
suppression commit. The result lists any owner-approved deferrals; with
none, it is labelled verified zero. A missing, unreadable, older or
unverifiable store is BLOCKED (AT-G10, AT-G11).

## Queue policy

- An open P0 blocks every commit, push and publication outside the repair
  lane. The block lifts when the fix is published (status fixed);
  reopening restores it (AT-G02).
- An open P1 or P2 in a component blocks feature changes touching that
  component (AT-G04).
- A P0 that needs the owner's decision is shown at the top of every report;
  an owner decision record can defer it for a stated period, after which it
  blocks again.
- Agents read `autoissues next --json` before a task (AT-G01).

## Repair lane

A candidate is in the repair lane when every changed hunk is attributed by
`Fixes:` or `Refs:` trailers to open issues, including reproducer, harness,
suppression and revert work, and a reviewer other than the author
confirms the attribution (the runner-produced AI review counts). Mixed
candidates are features. A repair-lane candidate is admitted when every
other gate passes, even while debt is unpaid or a P0 is open, and its own
fixes count as pending credit (AT-G03, AT-G12, AT-G21). Feature candidates
need the quota satisfied.

## No bypass

No variable, flag, override file or command skips the gate or resets
debt. The only corrections are the signed records in
[quota recovery](quota-recovery.md). Charge, credit, revocation and
clearance records are append-only (AT-G17, AT-G25).

## Messages

Refusals give the outcome, numbers and next step, for example: "Commit
refused: 20 fixes owed after two failed attempts since journal position
4180. 7 credited; 9 real issues are open, so 9 more fixes are needed. Run
`autoissues next`." No credentials, raw logs or private values (AT-G18).

## Decisions requested

- Q: proposed 10; alternatives 5, 25 or 100.
- Debt grows additively and credits are not reset by a later failure.
- The bootstrap scope and activation order in quota recovery.
