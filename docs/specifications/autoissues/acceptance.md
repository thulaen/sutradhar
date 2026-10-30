# SPEC-005 acceptance cases, part 1

Status: APPROVED. Each case becomes at least one automated test, written
before the behaviour (R087). Continued in [part 2](acceptance-2.md).
Every case runs on Dell or an isolated synthetic-data worker.
Approved by the owner on 30 October 2026 (see the approval record in
`docs/specifications/approvals-2026-10-30.md`); it supersedes any
"awaiting approval" wording below.

## Packaging

- AT-A01: Build every product artifact; any `tools/` path, QA compose file,
  dashboard or AutoIssues module inside it fails the build.
- AT-A02: The product's resolved dependency set contains no AutoIssues-only
  or QA-only package.

## Storage

- AT-S01: The configured live path outside the authorised live-data area,
  or inside a preserved version, is refused at start; tests never touch
  the live file.
- AT-S02: A database with pending migrations is backed up, migrated in
  order and integrity-checked; a database newer than the code is refused.
- AT-S03: A migration that fails mid-way rolls back, keeps the backup and
  leaves the service stopped with a plain-English error.
- AT-S04: A second service instance cannot acquire the writer lock.
- AT-S05: Two concurrent ingest runs are serialised; readers continue
  during writes; no write is lost under a crash injected between them.
- AT-S06: Identifiers are sequential `AI-<n>` and never reused after
  deletion attempts or restore.
- AT-S07: Invalid type, status, priority, severity or resolution values are
  refused by the schema.
- AT-S08: A second open issue with an existing open fingerprint is refused;
  the same fingerprint may exist again after the first is closed.
- AT-S09: Evidence with an unknown kind, or a file reference whose SHA-256
  does not match, is refused.
- AT-S10: Links are written with their inverse; self-links and parent or
  duplicate cycles are refused.
- AT-S11: Component default hotlists are applied on triage;
  `needs-owner-decision` exists in a fresh database.
- AT-S12: Every field change produces a history row in the same
  transaction; UPDATE or DELETE on history is refused.
- AT-S13: Comments are append-only.
- AT-S14: A run killed mid-ingest stays interrupted and cannot verify.

## Lifecycle

- AT-L01: Glob triage picks the most specific component; ambiguous
  configuration is refused at load; unmatched paths go to `unassigned`.
- AT-L02: Priority, severity and SLA due dates follow `triage.yaml`.
- AT-L03: Claim gives a 24-hour lease; a second claimant is refused.
- AT-L04: An expired lease returns the issue to triaged and fences the old
  holder's later writes.
- AT-L05: A repeated fingerprint updates occurrences instead of filing.
- AT-L06: `Fixes: AI-<n>` on a published `main` commit moves an issue with
  a valid fix check to fixed; an unpublished commit does not.
- AT-L07: Three consecutive complete clean runs on `main` (or two
  nightlies) verify; interrupted runs neither count nor reset; a run with
  the fingerprint resets the count.
- AT-L08: Seven days after verification the issue closes.
- AT-L09: Resolution reasons other than intended, infeasible or obsolete
  are refused.
- AT-L10: A reappearing fingerprint on a closed issue reopens it as a
  regression one priority higher (P0 stays P0) and revokes its credit.
- AT-L11: `wontfix` without a published suppression commit, justification
  and reason is refused.
- AT-L12: A suppression on a mandatory test, safety rule, hard-rule check
  or quota record is refused.
- AT-L13: A suppressed signal seen after the suppression is published
  reopens the issue.
- AT-L14: A source's first run baselines P3/P4 findings into grouped issues
  per rule; P0–P2 stay individual; later findings are individual.
- AT-L15: The fifty-first new finding in one run files one adapter-flood
  issue and stops; withheld findings are filed on later runs.
- AT-L16: Every transition outside the fixed table is refused and recorded.
- AT-L17: A performance fix without same-benchmark before and after numbers
  is refused at the commit gate.
- AT-L18: `verify-run` from an agent account is refused.
- AT-L19: The weekly sweep lists P3/P4 issues oldest first, grouped by
  rule, without closing any.
- AT-L20: `dupe` on a P0–P2 issue without an adapter fingerprint match or an
  owner decision record is refused; a valid duplicate keeps the higher
  priority on the survivor.
- AT-L21: An agent's request to add a quota-removing hotlist has no effect
  until the gate or an owner decision record applies it.
- AT-L22: Each issue type's proof of fix is checked: a coverage issue fixed
  by a test that passes on the parent is accepted on coverage evidence; a
  dependency advisory is accepted when the scanner no longer matches.
- AT-L23: An open issue absent for the required clean runs without a fix
  becomes not_reproducible and earns no credit.
- AT-L24: A worker's claim is honoured when the coordinator commits the
  worker's fix; a commit naming an issue leased to another worker is
  refused.

## Interfaces

- AT-I01: `python -m autoissues` and the `autoissues` command both work.
- AT-I02: Every command with `--json` emits one schema-valid document and
  the documented exit code.
- AT-I03: `next` ordering and filters match a fixture with ties on
  priority and SLA; leased and blocked issues are skipped.
- AT-I04: Invalid arguments fail before contacting the service.
- AT-I05: The API refuses to listen outside the QA namespace without the
  approved setting.
- AT-I06: Webhooks with a wrong secret, wrong schema or oversized body are
  refused and counted; valid ones enqueue one ingest run.
- AT-I07: Dashboard views match counts computed directly from a fixture.
- AT-I08: The dashboard renders headless to HTML and starts interactively
  on localhost.
- AT-I09: Two exports of the same state are byte-identical.
- AT-I12: A snapshot contains exactly `open.json`, `summary.md` and
  `bottlenecks.md`, with every open issue, its scrubbed evidence summary,
  the counts, SLO breaches and needs-owner-decision hotlist (AS-41).
- AT-I10: Snapshot publication is refused unless it is part of a milestone
  candidate that passed the privacy gate.
- AT-I11: Every bottleneck row over its threshold names an issue; missing
  sources show "no data yet".
