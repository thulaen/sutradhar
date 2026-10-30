# SPEC-011 part 2: the live stream, the live channel and the views

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in the
[specification decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Part of [SPEC-011](progress.md). Requirements: R110 and ADR-0004;
R001, R022, R043, R044, R047, R048, R049, R051, R092 as replaced by
R127, R123, R128 and R129; ADR-0006.

Every view below reads the state record of part 1; none computes its
own figures.

## The live stream route

This specification owns the product's single live stream route.
SPEC-012 and SPEC-009 add topics to it; there is no second channel.

- `GET /live/stream?topics=...` opens one server-sent events stream,
  served with `sse-starlette`, carrying every topic the page asks for:
  - `progress`: every unfinished operation the viewer may see;
  - `progress.<id>`: one operation;
  - `visual.<source>`: one of SPEC-012's visual data sources;
  - `suggestions`: new and changed suggestions for the review page, as
    each post's analysis completes (R123); SPEC-009 defines their
    content, and only people who may use the review page receive it;
  - `queues`: the live and backfill queues' depth and processing rate
    and the index coverage (R123), for the review and health pages;
    SPEC-008 and SPEC-009 define the figures, which are the same ones
    exported to Prometheus (R127), so pages and charts agree.
  Topic names are not secret; what a viewer may see comes from the
  viewer token, never from the address.
- Each event names its topic and carries an id. A reconnect with
  `Last-Event-ID` receives the current state of each topic, not the
  missed history. A keep-alive comment is sent every 15 seconds.
- A page opens at most one stream. One reader per service process
  checks the store for changed records every 250 ms and fans them out,
  so database load does not grow with the number of viewers.
- Up to 50 streams per service and 5 per person; beyond that the page
  is told to poll.
- The browser switches to polling after three failed connections within
  30 seconds, or when nothing, not even a keep-alive, arrives within 20
  seconds of connecting (which detects proxies that hold streams back).
  It retries streaming every five minutes.
- When a tab is hidden its stream closes and it polls every 30 seconds;
  when it is shown again the stream reopens.

## The live channel into XenForo pages

ADR-0004 asks for server-sent events from the Python service, with a
polling fallback, inside XenForo admin pages. R001 makes XenForo's
rules win at the forum boundary, R022 keeps PHP thin and R043 keeps
Python out of the XenForo database. The owner chose the web server
passthrough (D-PG01):

1. The Python service, run from the default docker compose file
   (R129), listens only on a private address of the forum host.
2. The forum's web server forwards one path on the forum's own address
   (for example `/sutradhar-live/`) to an allow-list: the stream route,
   the two progress routes below and SPEC-012's snapshot routes, for
   GET requests only. These routes therefore become reachable from the
   internet through the forum's address, protected by the viewer
   token; every other service route stays private. An agent or the
   host administrator sets this up during installation (R049).
3. The forwarding rule removes any signature headers a browser sends
   (the headers PHP uses for its signed requests), so a browser cannot
   pose as PHP.
4. When a page with live content is shown, PHP issues a viewer token:
   the XenForo user id, the permission scopes XenForo granted and an
   expiry five minutes ahead. It is signed with a key derived from the
   PHP–Python signing key (dwarpal, "gatekeeper") for this one purpose,
   so a viewer token can never pass as a signed PHP request, nor the
   other way round. It is set as a cookie limited to the live path
   (HttpOnly, Secure, SameSite=Strict) and never placed in an address.
5. The page renews the token through a small XenForo action every four
   minutes, but only while the tab is visible and in use (a key press,
   click or scroll in the last 30 minutes). An unattended page stops
   renewing, and its live updates end with "paused: page idle".
6. The service checks the token's signature, expiry and scopes before
   doing any other work, and refuses unsigned requests at once. It
   returns only what the scopes allow. Every response carries
   `Cache-Control: no-store` and `Vary: Accept, Cookie`, so no proxy or
   browser cache keeps it.
7. The installer's preflight checks that the forwarded path is served
   over HTTP/2, which lets many streams share one connection. Browsers
   allow only six older-style (HTTP/1.1) connections per site, so
   without HTTP/2 pages poll instead of streaming (P-PG09), and the
   installer says so.
8. Fallback when the forwarded path is not set up: the page polls a
   XenForo controller every 2 seconds while visible (every 30 when
   hidden); PHP fetches the snapshot with a signed request and passes
   it on through a small fixed buffer. Each poll is one short request.
9. PHP never holds a long streaming request open. Actions such as
   cancelling never use the live channel; they go through XenForo
   forms and the action layer.

## API routes

- `GET /actuator/jobs/{id}/progress`: one state record as JSON.
- `GET /actuator/jobs/running`: every unfinished record the caller may
  see, stages included.
- Both are snapshots; live updates come only from the stream route.
- Callers: signed requests from PHP and the action layer, or a viewer
  token through the live channel. Part 3 sets who sees what.
- An unknown id and an id the caller may not see both return 404, so
  existence is not revealed.

Example record:

```json
{"id": "op-7f3a", "job": "index.rebuild",
 "name": "Rebuild the search index", "run_id": "run-1182",
 "run_url": "/admin/sutradhar/jobs/run-1182", "audience": "admin",
 "status": "running", "stage": {"name": "embed", "index": 2,
 "count": 3}, "unit": "items", "noun": "posts", "done": 54000,
 "total": 120000, "failed": 3, "skipped": 41,
 "skipped_by_reason": {"not_english": 41}, "percent": 45.0,
 "eta_seconds": 760, "eta_basis": "live", "throughput": 87.4,
 "cancellable": true, "updated": "2026-11-02T10:42:07Z"}
```

## Command line

- `sutra` commands use `rich` (a library that draws terminal bars) when
  standard output is an interactive terminal: nested bars for stages,
  throughput, ETA and the failed and skipped counts.
- When standard output is not a terminal (CI logs, piped output, the
  gate), one plain line is printed at every 5% and at start, stage
  changes and the end, for example:
  `progress index.rebuild 45% 54,000/120,000 posts failed=3 eta=12m40s`
  With an unknown total there is one line every 60 seconds with done,
  elapsed time and throughput (P-PG03).
- `--progress auto|rich|plain|off` overrides the choice; `NO_COLOR` is
  respected; plain mode writes no terminal control codes.
- A command that starts work in the service follows that operation
  through the action layer (F018), using the stream route or, failing
  that, the JSON route; it does not compute figures itself.

## Admin, review and models pages

- A small first-party bar component (JavaScript and CSS, no library)
  shows each operation, with an overall bar and a stage bar for staged
  work. The review page shows per-post analysis rows, such as
  "analysing: 12 of 50 reranked".
- Markup: `role="progressbar"`, an `aria-label` with the operation's
  name, `aria-valuemin` 0, `aria-valuemax` 100, `aria-valuenow` the
  whole percentage and `aria-valuetext` a full sentence: "45%, 54,000
  of 120,000 posts, about 12 minutes left, 3 failed". An indeterminate
  bar has no `aria-valuenow`, as ARIA requires for unknown values, and
  its text gives elapsed time and throughput.
- The same sentence is shown as visible text under the bar: the text
  equivalent.
- Screen readers hear status changes and at most one progress
  announcement a minute. A "pause live updates" control stops updates
  (WCAG 2.2.2). Controls are at least 24 by 24 pixels (WCAG 2.5.8).
  With reduced motion the bar has no stripes or transitions.
- Cancel buttons appear only for cancellable operations. They submit a
  XenForo form, with its CSRF protection (protection against forged
  requests), to PHP, which calls the action layer.
- If live updates stop, the bar says "live updates unavailable, last
  update 14:02:10" rather than freezing silently.
- The review page receives new suggestions live on the `suggestions`
  topic and shows queue depth, processing rate and index coverage from
  the `queues` topic (R123), over the same stream as its bars and with
  the same polling fallback.

## Jobs section of the health page

Every running, paused, finishing or cancelling operation with its bar,
stage bar, status and start time; every run of the last seven days, up
to 100, with its duration, final status, done and total, and failed and
skipped counts; stalled operations flagged. Each row links to its run
through `run_url`. Beside the jobs, the health page shows the same
queue depth, processing rate and index coverage as the review page
(R123).

## Log lines, gauges and the completion notification

- Log: one structured line at every 5% and at start, stage changes,
  pause, resume and the end, with English fields: `event`, `op_id`,
  `job`, `run_id`, `trace_id`, `stage`, `done`, `total`, `failed`,
  `skipped`, `unit`, `eta_seconds`, `throughput` and `status`. With an
  unknown total, one line every 60 seconds. Per-post analysis writes
  its lines at debug level (P-PG05).
- Gauges through the Prometheus client, one set per operation, as
  ADR-0004 says (P-PG01): `sutradhar_progress_ratio`, `_done`,
  `_expected` (the total), `_failed`, `_skipped` and `_eta_seconds`,
  labelled with the job key, operation id and unit. The ratio is absent
  when the total is unknown. Metrics are always on (R127,
  [ADR-0006](../../adr/0006-observability.md)), which replaces the
  draft's "off by default" for D-PG03: the gauges are exposed at
  `/actuator/metrics` on the service's private address, never through
  the forwarded live path, and the bundled Prometheus server scrapes
  them every 15 seconds and keeps them 15 days. A scrape reads only the
  in-memory registry, never the database.
- The series are bounded by the operations running at once, sized at
  100 in part 4, which serves as these gauges' cardinality budget under
  R128; each set is removed one minute after its operation ends. Per-post
  analysis has no gauge set per post: Prometheus receives per-job
  totals for it instead, so the backfill does not create a series for
  every post (part 5's notes).
- Prometheus is also used for progress over time (R127): charts of
  throughput and duration across runs read its 15-day series. Live
  bars, the stream and the JSON routes still read the state record,
  because a scrape every 15 seconds cannot carry quarter-second
  updates.
- Traces: each job run is one trace (R127). Stage and status changes
  are added to it as span events, not as spans of their own, so a long
  job keeps within its span budget
  ([telemetry practices](../../dev/telemetry-practices.md)).
- Completion notification (D-PG04): one XenForo staff alert (R051),
  sent through PHP to the person who started the operation and to
  administrators who follow job alerts. It gives the name, final
  status, duration, done and total, failed count and a link to the
  run. Repeated alerts from recurring jobs are grouped (R051). ntfy
  and email are SPEC-012 item 54.

## Notebooks

Notebooks use marimo's progress bar, fed from the same handle by a
small adapter in the lab support code. Production code never imports
marimo or lab code (R026); an import-boundary test enforces this.

## PHP reporter

A small PHP class in the XenForo integration (sampark, "connection")
reports long steps that run inside XenForo, such as install, upgrade
and uninstall steps. It sends signed, batched updates at most once a
second to the same store. While the service is not running (before
installation finishes, or after uninstall has stopped it), the steps
appear in XenForo's own setup progress display and are sent once the
service is reachable; steps after the service has gone are recorded in
XenForo's log only (P-PG04).
