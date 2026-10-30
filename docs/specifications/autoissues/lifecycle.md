# SPEC-005 lifecycle

Status: APPROVED, revision 2 after independent review. Part of
[SPEC-005](README.md).
Approved by the owner on 30 October 2026 (see the approval record in
`docs/specifications/approvals-2026-10-30.md`); it supersedes any
"awaiting approval" wording below.

## States

Statuses (AS-12): new, triaged, assigned, in_progress, fixed, verified,
closed, wont_fix, not_reproducible, duplicate. Open means new, triaged,
assigned, in_progress or fixed. A checked candidate fix is a `fix_checks`
record (candidate tree, proof, runner signature), not a status.

## Transition table

Only these transitions exist; anything else is refused and recorded
(AT-L16). "Agent" means the worker identity in the coordinator's
authenticated session record; "gate" means the gate account.

| From | To | Who | Condition |
| --- | --- | --- | --- |
| new | triaged | gate | automatic triage |
| triaged | assigned | agent | `claim`; no live lease held |
| assigned | in_progress | lease holder | `start` |
| assigned, in_progress | triaged | holder or gate | `release` or expiry |
| triaged, assigned, in_progress | fixed | gate | published checked fix |
| fixed | verified | gate | clean-run rule below |
| verified | closed | gate | seven days later |
| fixed | triaged | gate | signal returns |
| verified, closed | triaged | gate | regression reopen |
| open | wont_fix | gate | published suppression |
| open | not_reproducible | gate | absence rule below |
| open | duplicate | gate | see duplicates rule below |
| wont_fix | triaged | gate | suppressed signal returns |

## Triage

`tools/autoissues/components.yaml` defines the component tree with path
globs, owners as labels, default hotlists and SLOs per priority.
`tools/autoissues/triage.yaml` maps each source's severity to priority and
severity. Triage runs in the filing transaction: most specific glob (ties
refused at load), priority and severity from the mapping, `sla_due` from
the SLO. An unmatched finding goes to `unassigned` with the
`needs-owner-decision` hotlist (AT-L01, AT-L02). Both files are protected.

## Claims

`claim` gives the worker a 24-hour lease. The lease belongs to the worker
identity, not to whoever commits: the coordinator commits the worker's fix
and the gate checks the worker's lease on the issue (AT-L03). Expiry
returns the issue to triaged and fences the old holder's later writes
(AT-L04). A fix checked while the worker's lease was live keeps that claim:
the `fix_checks` record stores it, so the fix can be published and moved
to fixed even after the lease expires. Each agent should have its own
operating-system account; while agents share one, cross-identity checks
rely only on adapter signals and the runner-produced AI review.

## Duplicates and deferral

A repeated fingerprint updates occurrences instead of filing (AT-L05).
`dupe` on a P0–P2 or creditable issue needs a fingerprint match recorded
by an adapter or an owner record; for other issues the gate applies an
agent's request after checking both issues exist and are open; the
surviving issue takes the higher priority (AT-L20). Hotlists that remove
an issue from the quota's open count (`needs-owner-decision`,
`blocked-external`) can be set by an agent only as a request; they take
effect from the gate or an owner decision record (AT-L21).

## Proof of fix

A commit fixes an issue with `Fixes: AI-<n>` trailers, a cited evidence
reference and a lesson. Each type has its own proof, run by the runner in
a sandbox on the candidate (AT-L06, AT-L22):

- bug, regression, flaky_test, fuzz_crash: a test or reproducer that
  fails on the parent for the intended reason and passes on the candidate;
- coverage: the coverage report shows the named lines and branches covered;
- mutation_survivor: the named mutants are killed;
- performance, hotspot, slow_span: same-benchmark before and after numbers
  under SPEC-007 (AT-L17);
- security dependency findings: the scanner no longer matches the locked
  versions;
- code_quality, security code findings: the analyser no longer reports the
  finding and does not report the same rule elsewhere in changed files;
- docs: the documentation check passes;
- alert and other runtime signals: a recorded reproducer replayed in the
  sandbox, or, when none exists, "fixed, awaiting nightly", which earns no
  credit until verified.

Dependency, secret and configuration fixes may declare
`TDD: not-applicable (dependency|secret|configuration)` with the proof
above instead of a failing test.

## Verification

Only the gate sets verified. Three consecutive complete runs of the
issue's source on published `main` without the fingerprint, or two
consecutive complete nightly runs for nightly-only sources, verify a fixed
issue. Incomplete runs neither count nor reset; a run showing the
fingerprint resets (AT-L07). Seven days later the gate closes it (AT-L08).
An open issue whose signal is absent for the same number of complete runs
without any fix becomes not_reproducible, with no credit (AT-L23). An agent
calling `verify-run` is refused (AT-L18).

## Reopening

A returning fingerprint on a verified or closed issue reopens it to
triaged as type regression, one priority higher (P0 stays P0), with a new
SLA and a link to the earlier fix. On a fixed issue it returns to triaged.
Either way the credit is revoked under SPEC-006 (AT-L10).

## Suppression

`wontfix` needs a resolution (intended, infeasible or obsolete), a written
justification and a published commit that adds the matching suppression
(Sonar exclusion, `nosemgrep` with reason, `baseline.yaml` entry or
threshold change). The CLI refuses it otherwise, and the gate refuses a
suppression commit without the issue reference (AT-L09, AT-L11, AT-L12).
Suppressions are protected changes and can never silence a mandatory test,
safety rule, hard-rule check or quota record. A suppressed signal that
reappears reopens the issue (AT-L13).

## Flood control

On a source's first complete run, P3 and P4 findings go to
`tools/autoissues/baseline.yaml` and one grouped issue per rule with the
file list; P0–P2 are always individual; later findings are individual
(AT-L14). Each adapter has a per-run cap, default 50 new issues. At the
cap it files one adapter-flood issue (P2), stops filing and records the
run as flooded; withheld findings appear on later runs (AT-L15). Baseline
entries still count as owed work in their grouped issues.

## Sweeps

A weekly job lists open P3 and P4 issues oldest first, grouped by rule, in
the `weekly-sweep` hotlist (AT-L19). It closes nothing.
