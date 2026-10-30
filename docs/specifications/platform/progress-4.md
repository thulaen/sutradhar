# SPEC-011 part 4: resources, testing and acceptance

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in the
[specification decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Part of [SPEC-011](progress.md). Requirements: R110 and ADR-0004; R021
and R050 (as amended), R022, R048, R054, R087, R090, R121, R127, R128
and R129; ADR-0006 and ADR-0007.

## Memory, storage and removal

SPEC-010 owns the single table that accounts for the service's cap
(6 GB, or 8 GB once Dell measurements confirm the estimate, R121) and
the table that classifies what uninstall removes. This specification
asks for the following rows in them; where SPEC-010's figures differ,
SPEC-010's tables are the authority. The bundled Prometheus server and
OpenTelemetry collector are outside the service's cap, with their own
declared allowance (R121, ADR-0006); this system's gauges are among the
series that allowance covers.

- Memory row "progress system and live channel": about 40 MB, covering
  records and estimate windows for 100 operations at once (each under
  64 KB, with at most 256 samples and 100 kept failures) and buffers
  for 50 streams (each under 64 KB).
- Storage row "progress history": one row per run, kept 30 days and at
  least 200 runs per job, with per-post analysis summarised per day;
  expected well under 100 MB.
- Uninstall: progress history is "other owned data" under R050 as
  amended, so uninstall offers an export and asks whether to keep or
  delete it. The live state, one row per operation in the service's
  state store (part 1), is deleted without export under SPEC-010's
  MP-17, which the owner accepted: it describes only operations in
  progress, and each finished run is already in the history.

## Time and page budgets

- `advance()` between updates does no input or output; its median cost
  is measured on Dell and recorded as a benchmark.
- Store writes: at most four a second plus at most 100 per run for each
  operation, plus checkpoint writes. Reads: one query every 250 ms per
  service process, whatever the number of viewers.
- An update reaches the browser within one second at the 95th
  percentile (p95) on Dell with 50 open streams.
- Page weight: the bar component is at most 4 KB of compressed
  JavaScript and 1 KB of CSS, served from the add-on's own files and
  loaded only on pages that show bars; no chart library, no WebGL.
- PHP (R022): issuing or renewing a token and each fallback poll each
  add under 1 MB of peak memory, measured with the add-on enabled
  against disabled, well inside 8 MB. No PHP request stays open for a
  stream. Polling is at most one request every 2 seconds per visible
  tab.
- Command line: `rich` redraws at most ten times a second; plain mode
  prints at most 21 percentage lines per determinate run, plus stage
  and status lines.
- Telemetry: the gauges, span events and log fields above fall within
  the overhead budget of [ADR-0007](../../adr/0007-service-levels.md),
  under 3% of CPU and latency, measured with telemetry on and off.
- 20× gate ([SPEC-007](../performance-gate.md)): every changed
  executable unit (Python, PHP and JavaScript) is mapped to a scenario.
  New code is measured against the simplest correct reference, for
  example publishing on every advance, or one database query per
  viewer. The update limit and the shared reader may show large
  speed-ups against those references; input- and output-bound units
  will probably need reviewed limit reports. Nothing is claimed until
  measured.

## Testing

All tests are written first and seen to fail for the intended reason
before the code exists (R087). They run on Dell, or in cloud workers on
synthetic fixtures only; never on Windows (R072, R119). R054's coverage
targets apply to Python, PHP and JavaScript; part 5's notes record the
JavaScript gap for one of them.

- Unit: the handle and strict mode; status changes; estimates with a
  fake clock (the four ADR-0004 cases and the others in part 1); the
  update limit; stage weights and side-by-side child handles; skipped
  counts; snapshot and restore; history and its pruning; gauge values,
  names and cardinality budget (R128); span events with the in-memory
  exporter; the log line format; the plain and `rich` renderers (using
  `rich`'s
  recording console); token signing and checking with shared test
  vectors in Python and PHP; the bar component's text, ARIA values and
  reduced-motion behaviour.
- Integration on Dell: the service, store, stream route and polling
  with the add-on in a XenForo test instance on the synthetic fixture
  forum; a forwarding proxy over HTTP/2 and over HTTP/1.1; a proxy that
  holds streams back; killing a worker at the operating-system level
  and resuming (R023); 50 open streams; the default compose project
  (R129) with the bundled Prometheus server scraping the gauges.
- Enforcement: the fixture run of every registered job, with recorded
  fakes for network jobs, and the command timing guard (part 3).
- Accessibility: automated checks with axe-core and keyboard tests in
  Playwright (a browser automation tool) against WCAG 2.2 AA (R048);
  reduced-motion emulation; a recorded manual screen-reader check.
  Following the owner's "skip it" (D-PG07), no Safari support is
  claimed and no real-Safari check is made; R048 needs real Safari only
  where support is claimed. Automated tests may still use Playwright's
  WebKit engine on Dell; they make no Safari claim.
- Privacy: scrubber tests on records, logs, span events and
  notifications with hostile fixture text.
- Performance: the scenarios above under SPEC-007.

## Acceptance cases

- AT-PG01: With a total of 100,000 and one item advanced every
  millisecond for ten seconds on a fake clock, about 40 updates are
  published; with a total of 1,000 at that pace, one per 10 items. No
  operation exceeds four updates a second plus 100 per run. Start,
  stage and end updates are never delayed.
- AT-PG02: An indeterminate operation has `total` and `percent` null,
  no ETA, and shows elapsed time and throughput; its bar has no
  `aria-valuenow`.
- AT-PG03: At a steady 100 items a second, after 30 seconds the ETA
  equals the remaining items ÷ 100 within 1%; `done` and the percentage
  never decrease while the total is unchanged.
- AT-PG04: With a previous run at 50 a second and live work at 100 a
  second, the ETA basis is "history" at the first advance, "blended"
  at 15 seconds (75 a second) and "live" at 30 seconds.
- AT-PG05: With no history and under 5 seconds of data, the ETA is
  null and views say "estimating".
- AT-PG06: A job checkpointed at 41,200, published at 42,000 and then
  killed resumes at 41,200 with the same operation and run ids,
  `resumed_from` 41,200, and an ETA seeded from the saved throughput.
- AT-PG07: Paused time is excluded from throughput; while paused, no
  ETA is shown and the reason is.
- AT-PG08: In 100 items, three `fail_item` and two `skip_item("not
  english")` calls end with `done` 100, `failed` 3, `skipped` 2 and
  `skipped_by_reason` {"not_english": 2}.
- AT-PG09: An exception inside the context manager records failed with
  its error code, is raised again and writes a history row.
- AT-PG10: Strict mode raises on each listed misuse, including
  `finish()` at 90 of 100 without a revised total; in production the
  same calls log one error code, do not raise and mark the total
  revised.
- AT-PG11: Raising the total mid-run sets `total_revised` and the
  views say so.
- AT-PG12: Staged weights follow the previous run's stage durations;
  the overall ETA is "unknown" when a later stage has no history; two
  child handles advancing at once both show their own progress.
- AT-PG13: In a terminal, `rich` shows nested bars with throughput, ETA
  and failed count; with output piped, exactly one plain line per 5%
  plus start, stage and end lines, with no control codes.
- AT-PG14: An indeterminate run in plain mode prints one line every 60
  seconds of fake-clock time.
- AT-PG15: Two workers and three viewers produce exactly one log line
  per 5% boundary, with every listed field.
- AT-PG16: Both JSON routes return records valid against the schema,
  including `run_url` and the skipped counts; `running` includes paused
  and cancelling records; unknown and forbidden ids both return 404.
- AT-PG17: One stream carries `progress`, `visual`, `suggestions` and
  `queues` topics together; with 50 streams open on Dell, updates
  arrive within one second at p95; keep-alives arrive every 15 seconds;
  a reconnect with `Last-Event-ID` receives each topic's current state;
  a viewer without review-page permission receives no `suggestions`.
- AT-PG18: Behind a proxy that holds streams back, the page polls within
  20 seconds; a hidden tab closes its stream and polls every 30
  seconds; showing it reopens the stream; over HTTP/1.1 pages poll.
- AT-PG19: Each fallback poll and token renewal adds under 1 MB of PHP
  peak memory, measured enabled against disabled.
- AT-PG20: Expired, altered, wrongly scoped and missing tokens are
  refused; a signed PHP request is not accepted as a viewer token, nor
  the reverse; no token appears in an address or a log; a page idle
  for 30 minutes stops renewing.
- AT-PG21: Through the live path, routes outside the allow-list and
  non-GET methods are refused; a browser-sent signature header is
  removed by the proxy; an unsigned request is refused before any
  database query; every response has `Cache-Control: no-store` and
  `Vary: Accept, Cookie`.
- AT-PG22: Bars have the progressbar role with correct values; the
  visible text equals `aria-valuetext`; axe-core finds no violations;
  controls are at least 24 by 24 pixels; announcements come at most
  once a minute apart from status changes; "pause live updates" works;
  reduced motion removes transitions.
- AT-PG23: The health page lists running operations and the last seven
  days of runs with durations and links, and flags a stall after five
  minutes of fake-clock time without change.
- AT-PG24: With the default settings, `/actuator/metrics` serves each
  running operation's gauges, removed one minute after it ends, and the
  bundled Prometheus server scrapes them; the route answers only on the
  private address and not through the live path; a scrape makes no
  database query; a backfill of 1,000 fixture posts adds no per-post
  series; every metric name passes R128's naming check.
- AT-PG25: A run that meets D-PG04's rule produces one staff alert with
  name, status, duration, done and total and failed count; a run that
  does not produces none; repeats are grouped; no alert holds post
  text.
- AT-PG26: Cancelling a cancellable job shows "cancelling, finishing
  current batch", commits the current batch, ends cancelled and keeps
  the checkpoint; a migration shows no cancel button and a request is
  refused with an explanation; every request is audited.
- AT-PG27: A moderator sees system-started per-post analysis rows on
  the review page but not index rebuilds; a person with "View models"
  sees model downloads; someone without permission cannot cancel; no
  live channel route accepts a cancel.
- AT-PG28: The fixture run passes for every registered job, with
  network jobs on recorded fakes and none skipped; a test job without a
  handle makes it fail.
- AT-PG29: A command declared short that runs longer than two seconds,
  and a long command without a handle, both fail the timing guard.
- AT-PG30: The marimo bar mirrors the handle's updates; the production
  package imports neither marimo nor lab code.
- AT-PG31: pytest-sugar output appears only in interactive Dell
  terminal runs; CI and gate logs show plain output.
- AT-PG32: Each operation in part 3's table, once built, reports in its
  stated unit with a determinate total where knowable; the judge job
  shows paused with its resume time at a simulated usage limit; a
  per-post analysis row reads "k of 50 reranked".
- AT-PG33: With the store unavailable, a job still finishes, the latest
  update is delivered once the store returns, and warnings appear at
  most once a minute.
- AT-PG34: After a worker is killed, its record becomes paused or
  failed with a reason when the service starts; none stays running.
- AT-PG35: 100 concurrent operations and 50 streams stay within the
  memory row given to SPEC-010.
- AT-PG36: Records, logs, gauges, span events and alerts contain no
  post text, titles or member names when the fixture holds hostile
  text.
- AT-PG37: During installation without a running service, steps show
  in XenForo's setup display and reach the store once it is running.
- AT-PG38: History keeps 30 days and at least 200 runs per job, removes
  nothing newer, and at uninstall is offered for export with a keep or
  delete choice; the live state is deleted without export (MP-17).
