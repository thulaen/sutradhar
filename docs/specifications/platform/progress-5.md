# SPEC-011 part 5: settings, decisions, proposals and notes

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in the
[specification decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Part of [SPEC-011](progress.md).

## Settings and defaults

Section `karya.progress`; keys inside it are English. Changes apply
between jobs (R025). The owner approved these defaults as listed
(D-PG08); R127 later removed `metrics_enabled`.

- `stall_after_seconds`: 300.
- `indeterminate_line_seconds`: 60 (P-PG03).
- `history_days`: 30; `history_min_runs`: 200 per job.
- `failures_kept`: 100 per run.
- `cancel_warn_seconds`: 600.
- `notify_min_seconds`: 60 (D-PG04).
- `metrics_enabled`: removed. It was listed as false (D-PG03); metrics
  are now always on (R127, ADR-0006), so there is no switch.
- `live.poll_seconds`: 2; `live.hidden_poll_seconds`: 30.
- `live.max_streams`: 50 per service; `live.max_streams_per_user`: 5.
- `live.token_seconds`: 300; `live.idle_minutes`: 30.

## Decisions (answered by the owner)

These were the choices that needed the owner's judgement; the first
answer was the draft's recommendation. The owner answered them on 30
October 2026. Each question and its options are kept as history, with
the owner's answer under it. D-PG06 to D-PG08 also cover SPEC-012.

- D-PG01: Live progress bars need a way into XenForo's admin pages.
  Which route should they use?
  - (a) The forum's web server passes one extra web address through to
    a few read-only parts of the Python service. Those parts become
    reachable from the internet through the forum's address, protected
    by a five-minute sign-in token that XenForo issues; everything else
    stays private. Where that address is not set up, pages ask PHP for
    updates every two seconds instead (recommended).
  - (b) PHP passes the live updates on itself. Nothing new is reachable
    from the internet, but every open admin page keeps one of the
    forum's PHP workers busy, which can slow the forum.

  Owner's answer: (a), the web server passthrough.
- D-PG02: The owner asked for updates "rate-limited to the sooner of
  250 ms or 1%". Which reading is right?
  - (a) Update when a quarter of a second has passed or 1% more work is
    done, whichever comes first: at most four updates a second plus at
    most 100 per task (recommended).
  - (b) Update only when both have happened: fewer updates, but a slow
    task's bar may stand still for minutes at a time.
  - (c) Update at most every quarter of a second; 1% is not a trigger.

  Owner's answer: (a), whichever comes first.
- D-PG03: The owner asked for monitoring numbers (Prometheus gauges),
  but another rule (R092) keeps the product's monitoring switched off
  by default. Which wins?
  - (a) The gauges are built but switched off by default; Dell's test
    profile and an administrator can switch them on (recommended).
  - (b) Switched on by default, readable only from the forum host.

  Owner's answer: neither. In the owner's words, "ship prometheus too,
  prometheus should also be used for progress and otel shipped too for
  traces and showing error logs on the gui", and, asked again, "shipped
  on by default. let them work." Recorded as R127 and ADR-0006:
  Prometheus metrics and OpenTelemetry tracing always on; a bundled
  Prometheus server with 15-day retention and a memory cap; a bundled
  OpenTelemetry collector that feeds sampled traces into Sutradhar's
  own database for 7 days, with a built-in trace viewer;
  `/actuator/metrics`, an optional OTLP export setting and a one-click
  Grafana export; all run from the default docker compose file (R129).
  The owner's full words are in parts 2 and 4 of the specification
  decisions (rounds 10, 11 and 15). Part 2 applies it to progress.
- D-PG04: Which finished tasks should send staff a XenForo alert?
  - (a) Tasks that ran a minute or more, and every failed task, except
    the many small per-post analyses; repeats from recurring tasks are
    grouped (recommended).
  - (b) Only failed and cancelled tasks.
  - (c) Every task of those kinds, however short.

  Owner's answer: (a), long or failed.
- D-PG05: The owner's decision record lists six states for a task but
  also asks for a "paused" state when the judge hits a usage limit.
  Should "paused" be a state of its own?
  - (a) Yes: a seventh state, always with a reason and, where known,
    the time it will carry on (recommended).
  - (b) No: keep six states and show pauses as "running" with a note.

  Owner's answer: (a), a seventh state.
- D-PG06: Both specifications need outside libraries: `rich`,
  `sse-starlette`, the Prometheus client, marimo and pytest-sugar here;
  Apache ECharts, sigma.js with graphology, D3, a WebGL scatter
  library, OpenTelemetry, a Python plotting library and the projection
  tools in SPEC-012. Each gets a licence and safety review first. What
  happens if one fails it?
  - (a) The feature that needs it is marked blocked, and a replacement
    is put to the owner (recommended).
  - (b) Agents may choose another free library of the same kind and
    report the change afterwards.

  Owner's answer: (a), block it and ask the owner.
- D-PG07: Before Sutradhar may say it works in Safari on Macs, iPhones
  and iPads, it must be checked in real Safari (R048). Dell cannot run
  Safari. How should that be done?
  - (a) The owner signs up for a cloud testing service with real Apple
    devices, used only with the made-up test forum after its terms and
    privacy are checked (recommended).
  - (b) The owner lends an agent access to a Mac and an iPhone or iPad.
  - (c) Make no Safari claim for now; automated tests use Safari's
    engine (WebKit) on Dell, and the question returns before release.

  Owner's answer: "skip it". No real-Safari check is made and
  Sutradhar makes no Safari support claim; R048 needs real Safari only
  where support is claimed. Automated WebKit tests on Dell may still
  run.
- D-PG08: Are the default settings in both specifications approved as
  a group (this part's list and SPEC-012 part 7's)?
  - (a) Yes, as listed (recommended).
  - (b) Yes, with the changes the owner names.

  Owner's answer: (a), yes, as listed. R127 later removed the metrics
  and tracing switches in both lists.

## Proposals

Technical choices, accepted as a group by the owner on 30 October
2026. Each says what it changes; P-PG05 narrows one ADR-0004 view for
one kind of task. Notes marked "Since R127" say how the later
monitoring decision applies.

- P-PG01: Prometheus gauges are kept per task, as ADR-0004 says ("a
  gauge per operation"), labelled with the job and task id. Their
  number is bounded by how many tasks run at once, and each set is
  removed a minute after its task ends. Since R127: the gauges are
  always on and kept 15 days by the bundled Prometheus server; that
  bound is their cardinality budget (R128); per-post analysis sends
  per-job totals instead of a gauge set per post (part 2).
- P-PG02: When a task's total grows, the bar may move back, labelled
  "total revised"; the work counted never decreases.
- P-PG03: When the total is unknown, plain and log lines appear every
  60 seconds, since "every 5%" cannot apply.
- P-PG04: While the Python service is not running (during installation
  and late in uninstallation), the PHP reporter shows steps in
  XenForo's own setup display and forwards them later.
- P-PG05: Per-post analysis, of which there is one per post, writes its
  5% log lines at debug level, which the default log level does not
  keep, and is summarised per day in the history instead of one row per
  post. Its live row and gauges are unchanged. If this group is not
  accepted, those lines are written at the normal level. Since R127:
  its live row is unchanged, and its gauges follow P-PG01's note.
- P-PG06: First-party browser code is plain JavaScript modules with no
  build step. Outside libraries are used as their published, unchanged
  prebuilt files (for ECharts, its full prebuilt file rather than a
  custom build). Tests use Node's built-in test runner and coverage,
  Playwright and axe-core, on Dell only.
- P-PG07: A command "can take longer than two seconds" when its work
  grows with forum size, uses the network, loads a model or waits on
  another service, or when it measured over two seconds on the
  50,000-thread test tier.
- P-PG08: "pytest-sugar locally" means interactive terminal sessions on
  Dell, never the Windows computer, which only runs the app (R119); the
  gate, CI and sandboxes use plain output.
- P-PG09: Without HTTP/2 on the forwarded address, pages poll instead
  of streaming, because older connections would run out.

## Recorder's notes

These are the drafter's readings, not owner decisions.

- `noun`, `skip_item`, `current`, `pause` and `resume` are additions
  to the ADR's method list; `skipped` serves SPEC-008's per-reason
  counts and `run_url` is the ADR's "link to the job run".
- `fail_item` and `skip_item` count as handled and `finish()` fixes an
  unknown total to `done`, so the enforcement test's "done equal to
  total" holds for every job, including indeterminate ones.
- "Nested stages" is read as stages returning child handles, which may
  advance together, as SPEC-010's trial needs.
- The single stream route is owned here so that SPEC-012 and future
  features add topics instead of channels. The ADR's two routes stay
  as JSON snapshots.
- R001: XenForo's rules govern the add-on inside XenForo. The forwarded
  address in D-PG01 (a) changes no XenForo code, route or table, and
  who may see what is still decided by XenForo's permissions, carried
  in the token XenForo issues. Setting it up is server preparation,
  which R049 gives to an agent or the host administrator; the polling
  fallback keeps R002's point-and-click path independent of it.
- R092 kept product instrumentation off by default while the ADR asked
  for a gauge per operation (D-PG03). R127 replaces R092 for metrics
  and tracing: both are always on. The owner's "prometheus should also
  be used for progress" is read as the gauges being scraped and kept by
  the bundled Prometheus server for charts over time, while live views
  keep reading the state record, which updates far more often than
  Prometheus scrapes.
- R128's practices say metric labels never carry IDs. P-PG01's
  operation id label, which the owner accepted, is kept because its
  series are bounded by the operations running at once and removed a
  minute after each ends. Per-post analysis would add one gauge set per
  post during the backfill, so it sends per-job totals instead. Both
  are readings to confirm when R128's cardinality check is put to the
  owner (R116).
- The total's gauge is named `sutradhar_progress_expected`, because
  R128's naming rule keeps the `_total` suffix for counters.
- Saving progress inside the checkpoint depends on F003's checkpoint
  design, which is not yet specified.
- "Nightly" is read as both the product's scheduled nightly run and the
  Dell nightly checks (R066).
- Dell tools use the installed, published copy of this module, so
  candidate code never runs with the gate's authority.
- Memory and storage figures are requests for rows in SPEC-010's
  tables; history is "other owned data" at uninstall (R050 as
  amended), and the live state is deleted without export (MP-17).
- R054 asks for MC/DC coverage (each condition in a decision shown to
  matter on its own). No JavaScript tool for this is known to the
  drafter; Node's coverage measures lines, functions and branches. The
  gap is recorded: until a tool is admitted, first-party JavaScript
  keeps each decision to a single condition where it can, so branch
  coverage also shows each condition, and any remaining compound
  condition is listed for review. JavaScript cannot be called fully
  compliant with R054 until the gap closes.
- First-party JavaScript and PHP fall under the source limits (R089),
  coverage (R054) and the 20× gate (R090) like Python. A JavaScript
  benchmark harness needs admission before the gate can measure it;
  until then SPEC-007's activation rules apply.
