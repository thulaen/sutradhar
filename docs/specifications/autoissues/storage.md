# SPEC-005 storage

Status: APPROVED. Part of [SPEC-005](README.md).
Approved by the owner on 30 October 2026 (see the approval record in
`docs/specifications/approvals-2026-10-30.md`); it supersedes any
"awaiting approval" wording below.

## Location

The live database is one SQLite file in a new live-data area inside the
authorised Dell project storage. That area must first be added to the
[storage preservation](../../dev/storage-preservation.md) design by an
approved revision (R096), because the project root is append-only and
preserved versions are immutable. The live file is mutable; it is never
placed inside a preserved version, and no preserved file is unlocked to make
it work.

The repository path `tools/autoissues/data/autoissues.db` is Git-ignored.
Tests create disposable databases in per-test temporary directories on the
test host. The Windows checkout never holds a database (AT-S01).

## Durability and snapshots

The database runs in WAL mode with `synchronous=FULL` and foreign keys on.
A nightly job writes a consistent copy with SQLite's online backup API,
verifies it with `PRAGMA integrity_check`, records its SHA-256 and moves it
into a new immutable snapshot directory. Snapshots are never overwritten or
pruned without the owner's explicit instruction. Recovery restores the
newest verified snapshot and replays the external journal written since
(see "Journal" and [quota recovery](quota-recovery.md#store-recovery)).

## Migrations

`schema_version` holds one row. Migrations are numbered, forward-only SQL
files with a SHA-256 manifest. On start the service:

1. refuses a database newer than the code (no downgrade);
2. takes an online backup before the first pending migration;
3. applies each pending migration in its own transaction;
4. verifies the result with `integrity_check` and the expected schema.

A failed migration rolls back and leaves the backup; the service stays
stopped with a plain-English error (AT-S02, AT-S03).

## Single writer

The service holds an exclusive advisory lock file and opens the only
read-write connection. All writes, including every ingest run, pass through
one in-process queue, so ingest runs are serialised. A second service
instance fails to start. Readers use separate read-only connections, which
WAL allows to proceed during writes (AT-S04, AT-S05).

## Issues

Columns:

- `id` integer key; displayed as `AI-<id>`; never reused (AT-S06).
- `title`, `type`, `component`, `priority` (P0–P4), `severity` (S0–S4),
  `status`, `resolution`, `assignee`, `lease_expires`, `reporter`.
- `fingerprint` text; a partial unique index enforces one open issue per
  fingerprint, where open means any status except verified, closed,
  wont_fix, not_reproducible and duplicate (AT-S08).
- `occurrence_count`, `first_seen`, `last_seen`, `last_run_id`.
- `sla_due`, `created`, `updated` as UTC ISO-8601 text.
- `lesson` structured text: cause, fix and prevention (see
  [quota](quota.md#credit)).

`type`, `status`, `priority`, `severity` and `resolution` are enforced by
`CHECK` constraints (AT-S07).

## Evidence

`evidence(issue_id, kind, ref, summary, sha256, created)` where kind is one
of log, trace_id, flamegraph, sonar_rule, glitchtip_event,
benchmark_before_after, sarif, reproducer, url. `ref` is a Dell path or URL;
`sha256` binds a file reference to its content. Summaries are scrubbed text;
raw private output is never copied into the store (AT-S09).

## Links

`links(from_id, kind, to_id)` with kind blocks, blocked_by, duplicate_of,
parent or child. Inverse pairs are written together in one transaction;
self-links and cycles in parent and duplicate chains are refused (AT-S10).

## Hotlists

`hotlists(id, name, description)` and `issue_hotlists(issue_id,
hotlist_id)`. Components supply default hotlists; `needs-owner-decision` is
built in (AT-S11).

## History

Every field change writes `history(issue_id, field, old, new, actor,
run_id, at)` in the same transaction as the change. History rows are never
updated or deleted; database triggers refuse `UPDATE` and `DELETE` on the
table (AT-S12).

## Comments

`comments(id, issue_id, actor, body, at)`; append-only like history
(AT-S13).

## Runs

`runs(id, source, candidate, started, finished, outcome, filed, updated,
verified, suppressed, reopened, flood)`. `candidate` is the Git commit or
tree the reports came from. A run that crashes is left with no `finished`
value and an `outcome` of interrupted; it never counts toward verification
(AT-S14).

## Fix checks

`fix_checks(issue_id, candidate_tree, proof_kind, bundle_sha256,
runner_signature, published_commit, at)` records each type-specific proof
of fix (see [lifecycle](lifecycle.md#proof-of-fix)). `published_commit` is
filled only after the publisher confirms the commit on GitHub `main`.

## Quota records

Written only by the gate account; database triggers refuse `UPDATE` and
`DELETE` on all of them:

- `gate_attempts(id, tree, stage, outcome, failed_checks, bundle_sha256,
  journal_seq, at)`;
- `charges(id, attempt_id, tree, amount, gate_failure_issue, journal_seq)`,
  unique per tree;
- `credits(id, root_cause, issue_ids, fix_check_id, reviewer,
  journal_seq)`;
- `revocations(id, credit_id, reason, added_debt, journal_seq)`;
- `clearances(id, round_start_seq, debt, credits, open_count,
  journal_seq)`;
- `owner_records(id, kind, payload, approval_ref, journal_seq)` for
  deferrals, recovery records, charge annulments and duplicate or hotlist
  decisions, each bound to the owner's verified approval comment;
- `activations(id, gate, policy_commit, genesis_sha256, journal_seq)`.

## Journal

Inside the database, `journal(seq, at, actor, operation, payload_sha256,
prev_sha256, sha256)` is an append-only, hash-chained index of every write.
The full payloads are also appended, before acknowledgement, to an
external journal file owned by the gate account and synced to disk, and
each new head is anchored with the publisher (see
[quota recovery](quota-recovery.md#external-journal-and-anchors)). Replay
uses the external file; the in-database table is for fast verification.

## Actors

Every write records an actor: an adapter, the gate, the verification job or
an agent's worker identity from the coordinator's authenticated session
record. Agent names alone are labels, not authentication; authority comes
from the operating-system identity the service observes, and quota records
accept only the gate account (see
[trust boundaries](../trust-boundaries.md)).
