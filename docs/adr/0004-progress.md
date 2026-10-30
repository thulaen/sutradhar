# ADR-0004: One progress system for everything over two seconds

Status: accepted by the owner, 30 October 2026. Requirement: R110. The
owner's exact words are in the
[platform requests](../requirements/sources/platform-requests-2026-10-30.md).

## Context

Many operations take minutes or days: downloads, index builds, the first
backlog run, evaluations and the nightly run. Each would otherwise report
progress its own way, or not at all.

## Decision

### Core

- Module `sutradhar.karya.progress`. A `Progress` handle offers
  `start(total=None, unit="items"|"bytes"|"steps")`, `advance(n)`,
  `set_total(n)`, `stage(name, index, count)` for nested stages,
  `fail_item(error)`, `finish()`, `cancel()` and a context-manager form.
- State per operation: id, name, stage, done, total, failed, unit,
  started, updated, ETA, throughput, status (running, finishing,
  cancelling, done, failed, cancelled) and a link to the job run.
- ETA uses a moving average of the last 30 seconds of throughput, seeded
  from the previous completed run of the same job. When the total is
  unknown, progress is reported as indeterminate with elapsed time and
  throughput, never a made-up percentage.
- Updates are rate-limited to the sooner of 250 ms or 1%.
- Progress is saved with the job checkpoint, so a resumed job continues
  from its previous position. A history of completed runs with durations
  is kept.

### Views, all reading the same state

- Command line through `rich` (nested bars, throughput, ETA, failed count),
  falling back to one plain line every 5% when standard output is not a
  terminal, so CI logs stay readable.
- Admin, review and models pages with live bars updated by server-sent
  events (`sse-starlette`) with a polling fallback, accessible with ARIA
  progressbar roles and a text equivalent.
- The jobs section of the health page: every running operation with its
  bar and every recent run with its duration.
- API: `/actuator/jobs/{id}/progress` and `/actuator/jobs/running`.
- A log line at every 5% with done, total, failed and ETA; a Prometheus
  gauge per operation; and the completion notification with duration and
  failed count.
- Cancel buttons where cancellation is safe, with the bar showing
  "cancelling, finishing current batch".

### Used everywhere

Determinate totals wherever a count or byte size is knowable: model
downloads (bytes), model verification, self-test, trials, index builds and
rebuilds (posts), incremental embedding, the initial import, every batch
job, link-graph rebuilds, database migrations (per migration), backups and
restores (bytes), export and uninstall, evaluation runs (suggestions
judged, batches done), the judge job (batches, with a paused state when a
usage limit is hit), AutoIssues ingestion (per adapter), snapshot export,
bulk actions on the review page, `sutra doctor` (checks), and the whole
nightly run and `just qa` as staged operations with an overall bar and a
per-stage bar. Notebooks use marimo's progress bar fed from the same
handle.

### Enforcement

- Every registered karya job and every command-line command that can run
  longer than two seconds creates a `Progress` handle.
- A test runs each registered job on the fixture data and asserts it
  emitted start, at least one advance and finish events, with done equal
  to total at the end.
- ETA behaviour is tested with a fake clock: monotonic progress, the
  indeterminate fallback, seeding from history and resuming from a
  checkpoint.
- pytest shows progress with pytest-sugar locally and plain output in CI.

## Consequences

- One state model serves every view (bars, logs, metrics and
  notifications read the same state).
- Recorder's notes for the feature specification: `rich`, `sse-starlette`,
  the Prometheus client, marimo and pytest-sugar each need an admission
  record; the Dell gates run tests on Dell, so "locally" means the
  developer's Dell runs, never the Windows editing computer.
