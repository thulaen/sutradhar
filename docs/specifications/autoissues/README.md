# SPEC-005: AutoIssues defect tracker and QA integrations

Status: APPROVED. Awaiting the owner's design approval. No implementation
may start before approval.
Approved by the owner on 30 October 2026 (see the approval record in
`docs/specifications/approvals-2026-10-30.md`); it supersedes any
"awaiting approval" wording below.

Audience: owner, maintainers and implementation agents.
Outcome: every quality signal from tests, analysers, monitors and agents
becomes one stable, deduplicated issue that the system, not the agent who
fixed it, verifies and closes.

Requirements: R083–R097 (see the
[30 October update](../../requirements/updates-2026-10-30/README.md)),
R045–R046, R054, R058, R072–R075. Source items AS-01 to AS-81 are mapped
in the
[source register](../../requirements/updates-2026-10-30/autoissues-source.md).

## Documents

| Part | Contents |
| --- | --- |
| [Storage](storage.md) | SQLite store, schema, migrations, writer |
| [Lifecycle](lifecycle.md) | Triage, claims, fixes, verification |
| [Interfaces](interfaces.md) | CLI, API, dashboard, snapshots |
| [Adapters](adapters.md) | Ingest contract and every adapter |
| [QA stack](qa-stack.md) | Dell services, sequence and budgets |
| [Quota (SPEC-006)](quota.md) | Commit and push queue and quota gate |
| [Acceptance](acceptance.md) | Acceptance cases, part 1 |
| [Acceptance 2](acceptance-2.md) | Acceptance cases, part 2 |

SPEC-006 is a separate approval item because it changes protected gates.

## Boundaries

AutoIssues is development tooling. It never reads private forum data
directly. Adapters read reports produced by checks that already ran on
Dell; private evidence (logs, traces, profiles) stays on Dell and the store
keeps references and scrubbed summaries only. Public snapshots pass the
privacy gate before publication (R075).

It is an independent implementation. Buganizer and Signal Controller are
behavioural inspiration. No code, schema, database, configuration,
credential, approval or branding is copied from either (R067).

## Packaging

All AutoIssues code lives in `tools/autoissues`; the QA stack lives in
`tools/qa`. Product packages (Python wheel, container image, XenForo add-on
archive and release bundle) are built from explicit include lists that do
not contain `tools/`. A packaging test inspects every built artifact and
fails if any `tools/` path, QA dependency or QA configuration appears
(AT-A01). The product's own dependency set must not require any
AutoIssues-only dependency (AT-A02).

## Runtime shape

One long-running AutoIssues service runs on Dell under the gate account
(see [trust boundaries](../trust-boundaries.md)). It is the only process
that opens the database for writing, and it never executes candidate
code: adapter re-runs and proofs of fix run in the runner's sandboxes and
reach the service only as signed bundles. The CLI, API and dashboard are
clients:

- the CLI talks to the service over a local Unix socket; the service reads
  the caller's operating-system identity from the socket (`SO_PEERCRED`),
  which is meaningful only under the host precondition that agents cannot
  switch accounts;
- the HTTP API listens only inside the QA network namespace, for webhooks
  and dashboard reads;
- gate and verification operations accept only the gate account's own
  identity, so an agent cannot call `verify-run` or write quota records.

When the service is not running, read-only commands may open a read-only
connection; every write command fails with a plain-English error. This
keeps "single writer" true without relying on agents' good behaviour.

## Language and dependencies

Python 3.14 for the tracker, adapters and dashboard, matching the Dell
runtime. PHP signals arrive as reports from PHP tools (PHPUnit JUnit XML,
the admitted PHP mutation and coverage tools, SonarQube PHP analysis,
`composer audit`). Each dependency (FastAPI, marimo, YAML parser, HTTP
client and each QA service image) needs an admission record with version,
licence, Python 3.14 or PHP 8.5 support evidence and rights review before
it is added. A tool whose support cannot be confirmed blocks its adapter;
it is not silently replaced. The current support findings are listed in
the [tool support record](tool-support.md).

## Engineering rules

The implementation obeys R087–R090: test-first evidence, the 300/60/80
source limits, the 20× performance gate, 100% line, branch, decision and
MC/DC coverage, mutation testing and the review set. Fixture-based tests run
on Dell or isolated cloud workers with synthetic data, never on Windows.

## Delivery milestones

1. Storage, migrations, single writer and data model, with tests.
2. Lifecycle: triage, claims, fixes, verification, reopening, suppression,
   flood control.
3. CLI and API with JSON output.
4. Test, benchmark, coverage, mutation and documentation adapters.
5. Quota and queue gate in the commit, push and publisher profiles.
6. Security, fuzzing and agent adapters.
7. QA stack profiles and the GlitchTip, Prometheus, Tempo, Pyroscope, Loki
   and SonarQube adapters.
8. Dashboard, bottlenecks, snapshots and the seeding run.

Each is a separate published milestone with its own evidence (R086).
Milestone 5 activates the quota gate; see the
[bootstrap rule](quota-recovery.md#bootstrap-and-activation-order).
