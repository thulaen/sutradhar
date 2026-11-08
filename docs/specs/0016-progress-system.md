# 0016: One progress system

Anything Sutradhar does that can take longer than two seconds tells you
how far it has got, the same way everywhere: a bar in the desktop app, a
bar or plain line in the `sutra` command, a log line, a metric and a
message when it ends. Every view reads one shared record per task, so
they always agree. The record is saved with the task's restart point, so
a restarted task carries on where it was, and its estimate of time left
says "estimating" or "unknown" rather than guessing. It lives in karya
(the jobs part) as `karya::progress`, and is part of Sutradhar Boot,
roadmap Phase 3 item 1.

## What it does

### Words used here

- Handle: the small object a task holds to report progress.
- ETA: estimated time left. Throughput: items, bytes or steps per
  second.
- Checkpoint: a saved restart point of a job (spec 0019).
- Indeterminate: the total amount of work is not known yet.
- Stream: one open connection over which the service keeps sending
  updates. Polling: asking again every few seconds instead.

### The handle

A job registered with karya receives its handle as `ctx.progress`,
already restored from its checkpoint when it resumes. Other code
creates one with `Progress::track(key, name)`, which returns a guard.

- `start(total, unit, noun)`: `unit` is items, bytes or steps; a total
  of `None` means indeterminate; `noun` is for display ("posts",
  "pairs").
- `advance(n)`: `n` units handled; never negative.
- `set_total(n)`: sets or revises the total.
- `stage(name, index, count)`: begins a stage and returns a child
  handle for its own counts; the parent works out the overall figure.
  Stages may nest, and several child handles may advance at once (the
  model trial runs one per model).
- `fail_item(error)`: one item failed. It counts as handled and adds
  one to `failed`. `error` is a central error code with a short
  plain-English message.
- `skip_item(reason)`: one item deliberately left out (such as "not
  English"). It counts as handled, adds one to `skipped` and to that
  reason's count.
- `finish()`: the work has ended well. For an indeterminate task the
  total becomes the final handled count, so handled equals total.
- `cancel()`: records that the task stopped on request.
- `pause(reason)` and `resume()`: for usage limits, the backlog run
  pausing under site load, and interrupted runs.
- `current(kind, item_id)`: the item being worked on now, as an
  identifier only.
- `cancel_requested()`: true once a permitted cancel request arrived.
- Guard form: dropping the guard normally calls `finish()`; an error
  records failed with its code and is passed on; a cancellation records
  cancelled.

Misuse fails tests ("strict mode"): advancing before `start`, a
negative `n`, handled beyond total, a total below handled, use after
the end, and `finish()` below a known total unless `set_total` revised
it. In production misuse never breaks the job: it logs one error code
per task and keeps the record honest, for example by setting the total
to handled and marking it "total revised".

The handle takes an injected clock. Durations and estimates use a
monotonic clock; shown times use the wall clock in UTC.

### The state record

One record per task, read by every view:

- `id`, `job` (registered job key), `name` (plain English), `run_id`,
  `run_link` (the screen that shows the run), `parent_id` (for
  stages), `audience` (who may see it);
- `stage`: name, index and count;
- `unit`, `noun`, `handled`, `total` (or null), `failed`, `skipped`,
  `skipped_by_reason` (count per reason code), `percent` (null when
  indeterminate);
- `status`: running, paused, finishing, cancelling, finished, failed or
  cancelled; `reason` in plain English with an error code for paused,
  failed and cancelled; a pause also gives its resume time where known;
- `started`, `updated` (UTC) and `elapsed_seconds`;
- `eta_seconds` (or null) and `eta_basis`: live, history, blended or
  none;
- `throughput`; `cancellable`, `total_revised`, `resumed_from` (or
  null) and `current` (or null);
- `failures`: the first 100 failed items of the run, each with code,
  item identifier and message; `failed` counts them all.

Status moves: running and paused alternate; running moves to finishing,
then finished; running or paused moves to cancelling, then cancelled;
any unfinished status may move to failed. Finished, failed and
cancelled are final. The record's shape is a Rust type with a shape
check in the contracts folder.

### Honest estimates

1. Throughput is work handled per second over the last 30 seconds of
   running time. The window always spans at least the two latest
   advances, so bursty jobs still get a figure. Paused time is left
   out.
2. Seed: the average throughput of the previous ended run of the same
   job and unit. Until the window holds 30 seconds, the figure blends
   seed and live by how full the window is (at 15 seconds, half each).
3. With no seed and under 5 seconds of live data, the ETA is null and
   views say "estimating".
4. ETA = (total - handled) / throughput, shown rounded ("about 12
   minutes").
5. With an unknown total there is no percentage and no ETA: views show
   elapsed time and throughput only.
6. Stages: the overall figure adds finished stages' weights and the
   current stage's fraction. Weights come from the previous run's stage
   durations, or are equal without history. The overall ETA is the
   current stage's ETA plus the previous durations of later stages; if
   a later stage has no history, the overall ETA is "unknown" and only
   the stage ETA shows.
7. A resumed run starts an empty window, seeded with the throughput
   saved in its checkpoint, or else with history.
8. When nothing has advanced for the stall time (default five minutes),
   views say "no progress for 5 minutes" instead of an ETA.
9. A revised total can lower the percentage; views say "total revised".
   The count handled never goes down.

### The update limit

An update is published when 250 ms have passed since the last one, or
the work has grown by 1% of the total, whichever comes first: at most
four a second plus at most 100 per run for each task. Store writes have
the same bound. With an unknown total only the 250 ms rule applies.
Between updates `advance()` only changes counters in memory. Start,
stage changes, status changes, a revised total, the end and a failure
are published at once. The 250 ms, 1%, 30-second and 5% figures are
not settings.

Updates go to listeners in the same process (the terminal view) and to
the service's state store, one row per task. One publisher writes the
log lines, metrics and notices, so each is made once however many
people watch.

### Saving, resuming and history

1. Each checkpoint karya commits holds a progress snapshot in the same
   transaction: handled, total, failed, skipped, stage, unit, noun,
   throughput and elapsed time.
2. A resumed run is restored from that snapshot, not from the last
   published update, which may be ahead of committed work. It keeps its
   id and run id. The record shows "resumed from checkpoint at 41,200
   of 120,000" and how many items will be repeated (at most one minute
   of work per worker).
3. When a task reaches a final status, a history row is written: job,
   run id, name, unit, noun, start, end, duration, handled, total,
   failed, skipped, final status, average throughput and each stage's
   duration. History seeds estimates, fills the health screen's recent
   runs and feeds the replays of spec 0017.
4. History keeps the last 30 days and at least the latest 200 runs of
   each job. Per-post analysis is summarised per day, so history does
   not grow with site size.

### Cancelling safely

1. A job may offer cancelling only if it declares safe points: it
   commits in batches and stopping between batches leaves no
   half-applied change.
2. Requests come only through the action layer as `karya.cancel`,
   checked by Cedar: the app's Cancel button, `sutra karya cancel
   <id>` or MCP. Each request is audited in lekha.
3. On a request the status becomes cancelling and views say
   "cancelling, finishing current batch". At its next safe point the
   job commits or rolls back the batch and calls `cancel()`. Its
   checkpoint is kept.
4. Tasks that cannot stop safely show no Cancel button; a request is
   refused with a plain reason.
5. A cancel not ended after ten minutes (a setting) shows "still
   finishing current batch" and a health warning. Nothing kills the
   worker in a way that skips recovery.

### Tasks that must use it

| Task | Unit and total | Cancel | Spec |
|---|---|---|---|
| Model downloads | bytes, from the lock files | yes | 0024 |
| Model verification | bytes hashed | yes | 0024 |
| Self-test | steps, one per model | yes | 0024 |
| Model trials | items: sample links; a stage per model | yes | 0024 |
| Thread-level index | items: threads | yes | 0022 |
| Index builds and rebuilds | items: posts | yes | 0022 |
| Incremental embedding | items: queued posts | yes | 0022 |
| Initial import | items: eligible posts; pauses | yes | 0022 |
| Per-post analysis | items: pairs reranked, of 50 | no | 0023 |
| Link-graph rebuilds | items: threads | yes | 0023 |
| Other batch jobs | their natural count | declared | own |
| Database migrations | steps, one per migration | no | 0030 |
| Backups | bytes | yes | 0030 |
| Restores | bytes | no | 0030 |
| Export | items or bytes | yes | 0032 |
| Removal | steps | no | 0030 |
| Evaluation runs | items judged; batch stages | yes | 0057 |
| Judge job | steps: batches; pauses at limits | yes | 0057 |
| AutoIssues intake | steps, one per adapter | yes | AutoIssues |
| Snapshot export | items | yes | AutoIssues |
| Review bulk actions | items: suggestions | yes | 0028 |
| `sutra doctor` | steps: checks | yes | 0032 |
| Nightly run | staged | per stage | 0019 |
| The gate's full run | staged | per stage | 0007 |

- Totals are known wherever a count or size is knowable. Posts that
  arrive during incremental embedding raise the total and mark it
  revised. Skipped posts are counted by reason.
- The thread-level index (from each thread's title and summary) is
  built first; the initial import then fills the per-post index in the
  background, by thread views and recency.
- A model trial runs each sample link through both models at once: two
  child handles advance side by side.
- Per-post analysis shows "k of 50 reranked" and the methods still to
  run. It stops only with its post's work.
- Partial downloads are kept for resuming; removal deletes them with
  the model files. A cancelled backup is marked incomplete and never
  offered for restore.
- The judge job pauses with its resume time at a usage limit, then
  continues.
- "Nightly" covers the product's nightly run and Dell's nightly checks.
- The gate's full run, Dell's nightly and AutoIssues use the installed,
  published copy of this module, never the candidate's. They have
  terminal and log views only, with history in the tools' own store on
  Dell.

### Who sees what

Each job declares its audience, stored in the record. The owner sees
every task in the desktop app. On the site's admin pages, which the
add-on draws from what the service sends:

- staff with Sutradhar's progress permission (`progress.view`) see
  every task the service sends there;
- staff with "View models" see model tasks (downloads, verification,
  self-test, trials, activation and rollback);
- staff who may use the review page see per-post analysis rows,
  including those the system started, and review bulk actions;
- the person who started a task always sees it;
- item identifiers in failure lists are shown to administrators only.

### The stream in the desktop app

1. The management API offers one stream route on the local socket,
   `GET /live/stream?topics=...`, carrying every topic the screen asks
   for: `progress` (every unfinished task); `progress.<id>`;
   `visual.<source>` (spec 0017); `suggestions` (new and changed
   suggestions as each post's analysis ends, spec 0023); `queues` (live
   and backfill queue depth, processing rate and index coverage, the
   same figures as the metrics, spec 0022 and 0023). There is no second
   channel: other parts add topics.
2. Each event names its topic and has an id. A reconnect with its last
   event id gets each topic's current state, not missed history. A
   keep-alive is sent every 15 seconds.
3. A screen opens at most one stream. One reader per service checks the
   store for changed records every 250 ms and fans them out, so database
   load does not grow with viewers. At most 50 streams per service;
   beyond that screens poll.
4. The app polls after three failed connections within 30 seconds, or
   when nothing arrives within 20 seconds of connecting, and retries the
   stream every five minutes. A hidden window polls every 30 seconds
   and reopens the stream when shown.
5. Snapshot routes: `GET /api/v1/jobs/{id}/progress` (one record) and
   `GET /api/v1/jobs/running` (every unfinished record the caller may
   see, stages, paused and cancelling ones included). An unknown id and
   a forbidden id both return 404.

### Progress on the site's admin pages

1. The site never calls the owner's PC. The service sends snapshots of
   the tasks site staff may see to the add-on in signed messages, at
   most once every 2 seconds per task, plus start, status changes and
   the end.
2. The add-on stores the latest snapshot; the page polls the add-on
   every 2 seconds while visible and every 30 when hidden. Each poll is
   one short request. No request stays open for a stream.
3. Cancel on the site (review bulk actions) is a site form with its
   CSRF protection. The add-on records the request and the service
   picks it up on its next call, then applies it through the action
   layer.
4. When the add-on runs its own long steps (install, upgrade and
   removal on the site), it records them in its own table and shows
   them in the site's setup progress display; the service collects them
   on its next call. Steps after the service has gone stay in the
   site's log only.

### Views

- Terminal (`sutra`): when output is a terminal, nested bars for stages
  with throughput, ETA and failed and skipped counts, redrawn at most
  ten times a second. Otherwise one plain line at each 5% and at start,
  stage changes and the end, such as
  `progress index.rebuild 45% 54,000/120,000 posts failed=3 eta=12m40s`.
  With an unknown total, one line every 60 seconds with handled,
  elapsed and throughput. `--progress auto|bars|plain|off` overrides;
  `NO_COLOR` is respected; plain mode writes no control codes. A command
  that starts work in the service follows it through the action layer,
  by the stream or the snapshot route; it computes nothing itself.
- Desktop app and site pages: an SDS progress bar with an overall bar
  and a stage bar for staged work; per-post rows such as "analysing:
  12 of 50 reranked".
  - It has the progressbar role, a label with the task's name, minimum
    0, maximum 100, the whole percentage as its value, and a full
    sentence as its value text: "45%, 54,000 of 120,000 posts, about 12
    minutes left, 3 failed". An indeterminate bar has no value, and its
    text gives elapsed time and throughput.
  - The same sentence is visible under the bar.
  - Screen readers hear status changes and at most one progress
    announcement a minute. A "Pause live updates" control stops
    updates. Controls are at least 24 by 24 pixels. With reduced motion
    there are no stripes or transitions.
  - Cancel buttons appear only for cancellable tasks.
  - If updates stop, the bar says "live updates unavailable, last
    update 14:02:10" and is marked stale.
  - The review screen receives suggestions on the `suggestions` topic
    and shows queue figures from `queues`, on the same stream.
- Health screen, jobs section: every running, paused, finishing or
  cancelling task with bars, status and start time; every run of the
  last seven days, up to 100, with duration, final status, handled and
  total, failed and skipped; stalled tasks flagged; each row links to
  its run. Beside them, the same queue figures as the review screen.
- Logs: one structured line at each 5% and at start, stage changes,
  pause, resume and the end, with `event`, `op_id`, `job`, `run_id`,
  `trace_id`, `stage`, `handled`, `total`, `failed`, `skipped`, `unit`,
  `eta_seconds`, `throughput` and `status`. Unknown total: one line a
  minute. Per-post analysis logs at debug level.
- Metrics, one set per task, labelled with job key, task id and unit:
  `sutradhar_progress_ratio` (absent when the total is unknown),
  `_handled`, `_expected` (the total; `_total` is kept for counters),
  `_failed`, `_skipped` and `_eta_seconds`. Metrics are always on,
  served on the local socket only, scraped every 15 seconds and kept 15
  days. A scrape reads only memory, never the database. Series are
  bounded by tasks running at once (100) and removed a minute after a
  task ends. Per-post analysis sends per-job totals, not a set per
  post. Charts across runs read the 15-day series; live bars read the
  state record.
- Traces: each job run is one trace; stage and status changes are span
  events, not spans.
- Notice when a task ends, for tasks that ran a minute or more and
  every failed task, except per-post analyses: an app notice for the
  owner, and a site staff alert, sent through the add-on, to whoever
  started it and staff who follow job alerts. It gives name, final
  status, duration, handled and total, failed count and a link to the
  run. Repeats from recurring jobs are grouped.
- Lab runs show the same handle's progress in their recorded output.
  Product code never uses lab code; a boundary test enforces it.

### Failure behaviour

- Reporting never breaks a job. Outside strict tests an error is logged
  once per task with a code, and the job carries on.
- Store down: the latest update is kept in memory and retried with
  growing pauses; a warning at most once a minute. Progress in a
  checkpoint shares its transaction, so the two never disagree.
- Worker killed or service restarted: at start-up, records whose worker
  is gone become paused "interrupted, resumes from checkpoint" if the
  job can resume, or failed with a reason if not. None stays running.
- Work refused before it starts (for example under memory pressure):
  a failed record is still made, with the plain reason.
- Stall: "no progress for 5 minutes" and a health warning; the job is
  not killed.
- Stream down: polling; if that fails too, the bar shows its last
  update time and is marked stale.
- Notice not delivered: retried with growing pauses for up to 24 hours
  and listed on the health screen.
- Metrics store or trace collector stopped or at its memory cap: jobs
  and views carry on, scrapes are missed, the bounded span queue drops
  spans, and the health screen says monitoring is unavailable.

### Privacy and security

- Records, logs, metrics, spans and notices hold job names, counts,
  durations, codes and item identifiers; never post text, titles or
  member names. `fail_item` messages pass through the log scrubber.
- Audiences are enforced by the service, not by the screen.
- Metric labels carry only job keys, task ids and units; every metric
  is on the "what we collect" page.

### Settings

Section `karya.progress`, applied between jobs:
`stall_after_seconds` 300; `indeterminate_line_seconds` 60;
`history_days` 30; `history_min_runs` 200; `failures_kept` 100;
`cancel_warn_seconds` 600; `notify_min_seconds` 60;
`live.poll_seconds` 2; `live.hidden_poll_seconds` 30;
`live.max_streams` 50.

## Limits

- Memory: about 40 MB inside the service slice, for records and
  estimate windows of 100 tasks at once (each under 64 KB, at most 256
  samples and 100 kept failures) and buffers for 50 streams (each under
  64 KB).
- Storage: history, one row per run, 30 days and at least 200 runs per
  job, per-post analysis summarised per day: well under 100 MB.
- Removal: history is owned data, so removal offers an export and asks
  keep or delete. Live state is deleted without export.
- Store writes: at most four a second plus 100 per run per task, plus
  checkpoint writes. Reads: one query every 250 ms per service.
- An update reaches the app within one second (p95) with 50 streams.
- `advance()` between updates does no input or output; its median cost
  is a benchmark.
- Plain mode: at most 21 percentage lines per determinate run, plus
  stage and status lines.
- Telemetry overhead under 3% of processor time and latency, measured
  on and off.

## Footprint budget

This part adds nothing to public pages. On the site's admin pages each
poll and each snapshot write adds under 1 MB of PHP peak memory,
measured with the add-on on against off; polling is at most one request
every 2 seconds per visible tab; the bar adds at most 5 KB compressed.

## How it is tested

Tests are written first and seen to fail. Rust tests use a fake clock
and a recording listener; PHP tests cover the add-on's side.

- AT-PG01: total 100,000, one item a millisecond for ten seconds: about
  40 updates; total 1,000 at that pace: one per 10 items; never more
  than four a second plus 100 per run; start, stage and end never
  delayed.
- AT-PG02: indeterminate: total and percent null, no ETA, elapsed and
  throughput shown, bar has no value.
- AT-PG03: steady 100 a second: after 30 seconds the ETA is remaining
  / 100 within 1%; handled and percentage never drop while the total is
  unchanged.
- AT-PG04: history 50 a second, live 100: basis "history" at the first
  advance, "blended" at 15 s (75 a second), "live" at 30 s.
- AT-PG05: no history and under 5 s: ETA null, "estimating".
- AT-PG06: checkpointed at 41,200, published at 42,000, killed: resumes
  at 41,200 with the same ids, `resumed_from` 41,200 and a seeded ETA.
- AT-PG07: paused time is left out of throughput; while paused no ETA,
  the reason shows.
- AT-PG08: 100 items with three `fail_item` and two `skip_item("not
  english")`: handled 100, failed 3, skipped 2, by reason 2.
- AT-PG09: an error in the guard form records failed, passes the error
  on and writes history.
- AT-PG10: strict mode fails on each misuse, including `finish()` at 90
  of 100; production logs one code, does not fail and marks the total
  revised.
- AT-PG11: raising the total sets `total_revised` and views say so.
- AT-PG12: stage weights follow the previous run; overall ETA
  "unknown" when a later stage has no history; two child handles
  advancing at once show their own figures.
- AT-PG13: in a terminal, nested bars; piped, one line per 5% plus
  start, stage and end, no control codes.
- AT-PG14: indeterminate plain mode prints a line every 60 fake-clock
  seconds.
- AT-PG15: two workers and three viewers give one log line per 5% step,
  with every field.
- AT-PG16: both snapshot routes match the shape check, including
  `run_link` and skipped counts; `running` includes paused and
  cancelling; unknown and forbidden ids return 404.
- AT-PG17: one stream carries `progress`, `visual`, `suggestions` and
  `queues`; with 50 streams updates arrive within one second (p95);
  keep-alives every 15 s; reconnect gets current state; a viewer
  without review permission gets no `suggestions`.
- AT-PG18: with a stream held back, the app polls within 20 s; a hidden
  window polls every 30 s and reopens the stream when shown.
- AT-PG19 (PHP): each poll and snapshot write adds under 1 MB of peak
  memory, on against off.
- AT-PG20: site snapshots carry only what each staff audience may see;
  a forged or unsigned snapshot is refused by the add-on.
- AT-PG21: the stream and snapshot routes answer only on the local
  socket, with Cedar checks; a caller without permission gets nothing.
- AT-PG22: SDS bars have the right role and values; visible text
  equals the value text; accessibility checks find nothing; targets at
  least 24 by 24; announcements at most once a minute apart from status
  changes; "Pause live updates" works; reduced motion removes
  transitions.
- AT-PG23: the health screen lists running tasks and seven days of runs
  with links, and flags a stall after five fake-clock minutes.
- AT-PG24: metrics serve each running task's set, removed a minute
  after it ends, scraped by the metrics store; served only on the local
  socket; a scrape makes no database query; a backfill of 1,000 posts
  adds no per-post series; every name passes the naming check.
- AT-PG25: a run that meets the notice rule gives one notice and one
  staff alert with the listed fields; one that does not gives none;
  repeats grouped; no post text.
- AT-PG26: cancelling shows "cancelling, finishing current batch",
  commits the batch, ends cancelled and keeps the checkpoint; a
  migration has no Cancel button and a request is refused with a
  reason; every request is audited.
- AT-PG27: a moderator sees system-started per-post rows but not index
  rebuilds; "View models" staff see downloads; someone without
  permission cannot cancel.
- AT-PG28: every registered job, run on the synthetic fixture site
  (about 300 threads) with a recording listener, shows a start, at
  least one advance, an end, and handled equal to total; network jobs
  (model downloads, the Hugging Face Hub, AI judge routes, AutoIssues
  adapters) use recorded fakes; none is skipped; a job with no work on
  the fixture fails as a fixture gap; a job without a handle fails.
- AT-PG29: every `sutra` command declares whether it can take over two
  seconds (its work grows with site size, it uses the network, loads a
  model or waits on another service, or it measured over two seconds on
  the 50,000-thread tier). A timing guard fails a short command that
  ran over two seconds and a long one that made no handle.
- AT-PG30: lab runs record the handle's updates; product crates do not
  use lab crates.
- AT-PG31: test output in the gate and CI is plain.
- AT-PG32: each task in the table reports in its unit with a known
  total where knowable; the judge job shows paused with its resume time
  at a simulated limit; a per-post row reads "k of 50 reranked".
- AT-PG33: with the store down a job still ends, the latest update
  lands when the store returns, warnings at most once a minute.
- AT-PG34: after a killed worker, its record becomes paused or failed
  at start-up; none stays running.
- AT-PG35: 100 tasks and 50 streams stay within 40 MB.
- AT-PG36: with hostile fixture text, records, logs, metrics, span
  events and notices hold no post text, titles or member names.
- AT-PG37 (PHP): add-on steps run while the service is away show in the
  site's setup display and reach the store on the service's next call.
- AT-PG38: history keeps 30 days and 200 runs per job, removes nothing
  newer, and is offered for export at removal; live state is deleted
  without export.
- Unit tests also cover status moves, history pruning, span events with
  an in-memory exporter, the log format and both terminal renderers.
- Performance: benchmarks for `advance()`, the update limit and the
  shared reader, under the 5% median rule.

## Changes from the source

- Rewritten as Rust: the handle is a Rust type with a drop guard.
- The live stream into the site's admin pages through a web server
  passthrough, with viewer tokens, signature-header stripping and an
  HTTP/2 preflight, is dropped: the site never calls the owner's PC.
  The stream serves the Dioxus app on the local socket, and site pages
  poll the add-on for snapshots the service sends.
- The PHP reporter records steps in the add-on and the service collects
  them, instead of the add-on sending them.
- Site-side cancel requests are collected by the service on its next
  call.
- Health and metrics routes move to the management API on the local
  socket.
- The bundled metrics store and trace collector run inside the service slice,
  not from a compose file.
- Terminal bars and test progress use Rust crates; lab notebooks become
  Rust lab runs with recorded inputs and outputs.
- Outside libraries are Rust crates under cargo-deny, cargo-audit and
  cargo-vet; one that fails review holds back its feature and a
  replacement is put to the owner.
- The screen-side bar is an SDS component, not hand-written page code.
- The count of handled items and the ending-well status got new names,
  to keep status words out of docs.
- The per-person stream limit (5) is dropped, since the app has one
  user.
- The 20-times speed goal is replaced by the 5% median rule.

Source: `docs/specifications/platform/progress.md`,
`docs/specifications/platform/progress-2.md`,
`docs/specifications/platform/progress-3.md`,
`docs/specifications/platform/progress-4.md` and
`docs/specifications/platform/progress-5.md` in the archive
`archive-2026-11-08-old-repo`.
