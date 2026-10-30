# SPEC-011: One progress system

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in the
[specification decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Requirements: R110 and [ADR-0004](../../adr/0004-progress.md) in full;
also R001, R008, R015, R020, R022, R023, R025, R026, R027, R043, R044,
R047, R048, R049, R050 (as amended), R051, R054, R058, R065, R067,
R072, R077, R087, R089, R090, R092 (as replaced for metrics and
tracing by R127), R107, R119, R121, R123, R127, R128 and R129;
[ADR-0006](../../adr/0006-observability.md) and
[ADR-0007](../../adr/0007-service-levels.md).
Serves features: F003 (events, batch work and durable recovery), F018
(GUI, `sutra` command line and MCP), F022 (diagnostics), F007
(notebooks), F021 (installation, upgrade and removal), F023 (quality
judging) and F025 (developer checks).

Parts:

1. This part: scope, terms, the handle, the state record, estimates,
   the update limit, saving and resuming, history and cancelling.
2. [Part 2](progress-2.md): the live stream route, the live channel
   into XenForo pages and every view of the state.
3. [Part 3](progress-3.md): the operations that must use it, who sees
   what, enforcement, failure behaviour, privacy and security, and
   admission records.
4. [Part 4](progress-4.md): memory and storage, testing and acceptance
   cases.
5. [Part 5](progress-5.md): settings, the owner's decisions,
   proposals and recorder's notes.

The owner's requirement is in the
[platform update](../../requirements/updates-2026-10-30/platform.md);
the owner's exact words are in the
[platform requests](../../requirements/sources/platform-requests-2026-10-30.md).

## In plain words

Anything Sutradhar does that can take longer than two seconds tells you
how far it has got, in the same way everywhere: a bar on the admin,
review and models pages, a bar or a plain line in the command line, a
line in the logs, a number for monitoring tools and a message when it
ends. All of these read one shared record per task, so they always
agree. The record is saved with the task's restart point, so a task
that stops and restarts carries on from where it was. Its estimate of
the time left is honest: it says "estimating" or "unknown" rather than
guessing.

Terms used in this specification:

- Handle: the small object a task holds to report its progress.
- ETA: estimated time remaining.
- Throughput: items, bytes or steps finished per second.
- Checkpoint: a saved restart point of a job (F003).
- Indeterminate: the total amount of work is not known yet.
- Server-sent events: one open connection over which the server keeps
  sending updates to a web page. Polling means the page asks again
  every few seconds instead.
- Terminal (TTY): an interactive command window, as opposed to a log.
- Prometheus: a monitoring tool that reads numbers from the service.
  Sutradhar bundles a Prometheus server, on by default (R127).
- OpenTelemetry: a standard for traces, the timed steps of one piece of
  work. A bundled collector receives them (R127).

## Scope

In scope:

- the core module `sutradhar.karya.progress` (karya: "work", the
  scheduled and batch jobs part): the `Progress` handle, its state
  record, estimates, update limit, saving with checkpoints and the run
  history;
- every view listed in ADR-0004: command line; the admin, review and
  models pages; the jobs section of the health page; two API routes;
  log lines; Prometheus gauges, on by default (R127); the completion
  notification; cancel buttons;
- the product's single live stream route and the "live channel" that
  brings it into XenForo pages; SPEC-012, and SPEC-009's live
  suggestions and queue figures (R123), add topics to it and have no
  channel of their own;
- a small PHP reporter for long steps that run inside XenForo;
- the marimo progress bar for notebooks, fed from the same handle;
- use in every long operation listed in ADR-0004, plus the per-post
  analysis row that SPEC-009 reports ("12 of 50 reranked");
- the enforcement tests and pytest's progress display.

Out of scope:

- The operations themselves. Reading and indexing are SPEC-008;
  matching and anchors SPEC-009; model files, community models,
  trials, retraining and removal SPEC-010; AutoIssues and `just qa`
  SPEC-005. Features without specifications yet (F003 checkpoints,
  F009 bulk actions, F021, F022, F023) adopt this system when they are
  specified. This specification fixes only how they report progress.
- The checkpoint mechanism itself (F003). This specification requires
  only that progress is saved inside each checkpoint.
- The memory table for the service's cap (6 GB, or 8 GB once Dell
  measurements confirm the estimate, R121) and the uninstall
  classification table, both owned by SPEC-010; part 4 gives this
  system's rows by reference.
- The bundled Prometheus server and OpenTelemetry collector, their
  place in the default docker compose file (R129), the trace store and
  the owner-facing monitoring layer: ADR-0006, with the 36 features in
  [observability features](../../dev/observability-features.md) and the
  56 practices in [telemetry practices](../../dev/telemetry-practices.md).
  This specification only feeds them progress.
- Visuals built on progress (register items 4, 40, 43, 45, 49 and 50)
  and notifications by ntfy or email (item 54): SPEC-012.

## Design: the handle

A job registered with karya receives its handle from karya as
`ctx.progress`, already restored from its checkpoint when the job
resumes. A command or other code creates one with `Progress.track`:

```python
from sutradhar.karya.progress import Progress

with Progress.track("index.rebuild", "Rebuild the search index") as p:
    p.start(total=post_count, unit="items", noun="posts")
    for batch in batches:
        embed(batch)
        p.advance(len(batch))
```

Methods are those of ADR-0004; those marked "added" were this draft's
additions, accepted by the owner with its proposals:

- `start(total=None, unit="items")`: `unit` is "items", "bytes" or
  "steps"; `total=None` means indeterminate. Added: an optional `noun`
  for display ("posts", "pairs"); the unit stays one of the three.
- `advance(n=1)`: `n` items, bytes or steps finished; never negative.
- `set_total(n)`: sets or revises the total.
- `stage(name, index, count)`: begins stage `index` of `count` and
  returns a child handle for that stage's own counts; the parent works
  out the overall figure. Stages may nest, and several child handles
  may advance at the same time (SPEC-010's trial runs one per model).
- `fail_item(error)`: one item failed. It counts as handled, adding one
  to both `done` and `failed`. `error` is a central error code (R051)
  with a short plain-English message.
- Added: `skip_item(reason)`: one item deliberately not processed, such
  as a post skipped as "not English" (SPEC-008). It counts as handled,
  adding one to `done`, to `skipped` and to that reason's count.
- `finish()`: the work is complete. For an indeterminate operation the
  total becomes the final `done`, so `done` equals `total` at the end.
- `cancel()`: the job records that it stopped on request.
- Added: `pause(reason)` and `resume()`, for the judge job's usage
  limit, the backlog run pausing under forum load (R107) and
  interrupted runs (decision D-PG05).
- Added: `current(kind, item_id)`: the item being worked on now, as an
  identifier only; SPEC-012 register item 4 shows it.
- `cancel_requested`: true once a permitted cancel request arrived.
- Context-manager form: leaving the block normally calls `finish()`;
  an exception records `failed` with its error code and is raised
  again; a cancellation records `cancelled`.

Misuse raises an error in tests ("strict mode"): advancing before
`start`, a negative `n`, `done` beyond `total`, a total below `done`,
use after the end, and `finish()` with `done` below a known total
unless `set_total` revised the total first. In production misuse never
breaks the job: it logs one error code per operation and keeps the
record honest, for example by setting the total to `done` and marking
it "total revised".

The handle accepts an injected clock for tests. Durations and estimates
use a monotonic clock (one that never jumps when the system time is
changed); displayed times use the wall clock in UTC.

## Design: the state record

One record per operation, read by every view:

- `id` (the operation), `job` (registered job key), `name` (plain
  English), `run_id` and `run_url` (the job run and the page that
  shows it), `parent_id` (for stages), `audience` (who may see it,
  part 3);
- `stage`: name, index and count;
- `unit`, `noun`, `done`, `total` (or null), `failed`, `skipped`,
  `skipped_by_reason` (a count per reason code), `percent` (null when
  indeterminate);
- `status`: running, paused, finishing, cancelling, done, failed or
  cancelled; `reason` in plain English with an error code for paused,
  failed and cancelled;
- `started` and `updated` (UTC) and `elapsed_seconds`;
- `eta_seconds` (or null) and `eta_basis`: live, history, blended or
  none;
- `throughput` (units per second);
- `cancellable`, `total_revised`, `resumed_from` (or null) and
  `current` (or null);
- `failures`: the first 100 failed items of the run, each with error
  code, item identifier and message; the `failed` count covers all.

Status changes: running and paused alternate; running moves to
finishing, then done; running or paused moves to cancelling, then
cancelled; any unfinished status may move to failed. Done, failed and
cancelled are final. A JSON Schema for the record lives in the
contracts directory (R077).

## Design: honest estimates

1. Throughput is the work finished per second over the last 30 seconds
   of running time. The window always spans at least the two most
   recent advances, so a job that reports in large bursts still gets a
   figure. Paused time is left out.
2. Seed: the average throughput of the previous completed run of the
   same job with the same unit. Until the live window holds 30 seconds
   the figure blends the two, weighted by how full the window is: after
   15 seconds, half live and half seed.
3. With no seed and under 5 seconds of live data the ETA is null and
   views say "estimating". Nothing is guessed.
4. ETA = (total − done) ÷ throughput, shown rounded ("about 12
   minutes").
5. With an unknown total there is no percentage and no ETA: views show
   elapsed time and throughput only, never a made-up percentage.
6. Staged operations: the overall figure adds the finished stages'
   weights and the current stage's fraction. Weights come from the
   previous run's stage durations, or are equal without history. The
   overall ETA is the current stage's ETA plus the previous durations
   of later stages; if a later stage has no history, the overall ETA
   is "unknown" and only the stage ETA is shown.
7. A resumed run starts an empty window, seeded with the throughput the
   interrupted run saved in its checkpoint, or else with history.
8. When nothing has advanced for the stall threshold (default five
   minutes), views say "no progress for 5 minutes" instead of an ETA.
9. A revised total can lower the percentage; views then say "total
   revised" rather than hiding the change (proposal P-PG02).

## Design: the update limit

ADR-0004 fixes the figures 250 ms and 1%. They are read as: publish an
update when 250 ms have passed since the last one, or the work has
grown by 1% of the total since then, whichever comes first (the owner
confirmed this reading, D-PG02). That allows at most four updates a
second plus at most 100 per run for each operation, and the same bound
applies to the store writes that carry them. With an unknown total only
the 250 ms rule applies. Between updates `advance()` only changes
counters in memory. Start, stage changes, status changes, a revised
total, the end and a job failure are published at once. The 250 ms,
1%, 30 second and 5% figures are not settings.

Updates go to listeners in the same process (the command-line view and
the notebook bar) and to the service's own state store as one row per
operation. One publisher in the service writes the log lines, gauges
and notifications, so each is produced once however many people watch.

## Design: saving, resuming and history

Each checkpoint that karya commits (F003) holds a progress snapshot in
the same transaction: done, total, failed, skipped, stage, unit, noun,
measured throughput and elapsed time. A resumed run is restored from
that snapshot, not from the last published update, because a published
update can be ahead of the last committed work. The operation keeps its
id and run id. The record shows "resumed from checkpoint at 41,200 of
120,000" and how many items will be repeated, which R020 bounds to one
minute of work per worker.

When an operation reaches done, failed or cancelled, a history row is
written: job, run id, name, unit, noun, start, end, duration, done,
total, failed, skipped, final status, average throughput and each
stage's duration. History seeds estimates, fills the health page's
recent runs and feeds SPEC-012's replays. Runs from the last 30 days
are kept, and at least the latest 200 runs of each job (settings).
Per-post analysis rows are summarised per day instead of kept one by
one (P-PG05), so history does not grow with the forum's size.

## Design: cancelling safely

- A job may offer cancelling only if its registration declares safe
  points: it commits work in batches, and stopping between batches
  leaves no half-applied change. Part 3 lists each operation.
- Requests come only through the permission-checked action layer
  (R044) as the action `karya.cancel`: a cancel button,
  `sutra karya cancel <id>` or MCP. Each request is audited (lekha).
- On a request the status becomes cancelling and views show
  "cancelling, finishing current batch". At its next safe point the job
  sees `cancel_requested`, commits or rolls back the current batch and
  calls `cancel()`. Its checkpoint is kept, so it can be resumed or
  started again as the job defines.
- Operations that cannot stop safely show no cancel button; a request
  is refused with a plain explanation.
- If a cancel has not finished within ten minutes (setting), views say
  "still finishing current batch" and the health page warns. Nothing
  kills the worker in a way that skips recovery.
