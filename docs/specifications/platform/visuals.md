# SPEC-012: Visual layer on real data

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in the
[specification decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Requirements: R111 and the
[visual layer register](../../dev/visuals.md) (58 items); also R001,
R009, R010, R013, R015, R016, R019, R022, R023, R024, R025, R027,
R029–R033, R043, R044, R047, R048, R050 (as amended), R051, R052, R054,
R060, R067, R075, R078, R079, R087, R090, R092 (as replaced for
metrics and tracing by R127), R097, R098, R102, R110, R121, R123, R127,
R128 and R129; [ADR-0006](../../adr/0006-observability.md).
Serves features: F018 (GUI and action layer), F003 (events), F009
(staff suggestions and review), F008 (ranking), F022 (audit and
diagnostics), F011 and F012 (analytics, items 16 and 17), F013 (SEO
signals, item 31), F021 (installation, item 49), F023 (quality
judging, items 39 and 41) and F002 (module lifecycle, items 25 and 48).

Parts:

1. This part: scope, terms and the shared design.
2. [Part 2](visuals-2.md): failure behaviour, privacy and security,
   admission records, resources, testing and shared acceptance cases.
3. [Part 3](visuals-3.md): items 1 to 14 (Live; Review page begins).
4. [Part 4](visuals-4.md): items 15 to 30 (Review page; Dashboard).
5. [Part 5](visuals-5.md): items 31 to 45 (Dashboard; Exploration and
   AI; Operations begins).
6. [Part 6](visuals-6.md): items 46 to 58 (Operations; Ambient and
   notifications; Forum-facing; follow-up items).
7. [Part 7](visuals-7.md): settings, the owner's decisions,
   proposals and recorder's notes.

The owner's exact list is in the
[platform requests](../../requirements/sources/platform-requests-2026-10-30.md).

## In plain words

The admin pages gain charts, maps and live animations that show what
Sutradhar is really doing. Every one of them draws only genuine system
records: nothing is invented to look busy. Each has a plain text or
table version, respects people who ask their computer for less motion,
and downloads its drawing code only when it is needed. Text written by
members appears only on staff pages. Only public thread titles reach
beyond them, in two places and under stricter rules set out below: the
label on public thread pages (item 56) and pictures an administrator
exports (item 29). Notifications carry totals only.

Terms:

- WebGL: drawing with the computer's graphics hardware in the browser,
  used for very large pictures such as the link graph.
- Lazy loading: downloading a piece of code only when it is needed.
- Text equivalent: a sentence or table giving the same information as
  a picture, for screen readers and anyone who prefers text.
- Reduced motion: an operating-system setting that asks pages not to
  animate; pages read it as `prefers-reduced-motion`.
- Deduplicated: repeats shown once with a count.
- Treemap: rectangles sized by a number, nested like folders.
- Force-directed layout: points pushed apart and pulled together by
  their links until the picture settles.

## Scope

In scope:

- all 58 register items, each with its page, data source, behaviour,
  reduced-motion form, text equivalent and acceptance case (parts 3 to
  6);
- the shared foundation: data source registry, snapshot routes, loader,
  live topics on SPEC-011's single stream route, accessibility helpers,
  light and dark themes;
- the rules of R111: real data only, reduced motion, text equivalents,
  lazy-loaded WebGL, free libraries, member text only on admin pages;
- the build order: the first ten, then the rest in register order;
- what member-facing (item 56) and outgoing (item 54, item 29's image)
  content may contain;
- admission records for Apache ECharts, sigma.js, D3, a WebGL scatter
  library and OpenTelemetry, and the other libraries this design needs.

Out of scope:

- Producing the data. Events and stage timings come from F003,
  SPEC-008 and SPEC-009; the link graph and suggestion scores from
  SPEC-009; ranking signals from F008; the index and summary vectors
  from SPEC-008; models and trials from SPEC-010; progress from
  SPEC-011; analytics from F011 and F012; SEO signals from F013; audit
  and diagnostics from F022; installation from F021; evaluation from
  F023; module lifecycle from F002.
- The live stream itself (SPEC-011), and the memory table for the
  service's cap (6 GB, or 8 GB once Dell measurements confirm the
  estimate, R121) and uninstall classification table (SPEC-010); part
  2 gives this layer's rows by reference.
- The review page's live suggestions and the queue and coverage figures
  (R123; SPEC-008, SPEC-009 and SPEC-011).
- The bundled Prometheus server and OpenTelemetry collector, the trace
  store and the owner-facing monitoring features: ADR-0006, with the 36
  features in [observability features](../../dev/observability-features.md)
  and the 56 practices in
  [telemetry practices](../../dev/telemetry-practices.md). This layer
  draws on their data.
- Changing any rule: R009, R010, R013 and R029–R033 apply unchanged.
- XenForo pages other than those named here.

## Design: where visuals appear

- Live page: items 1 to 6, 8, 9 and 58.
- Review page (restricted to staff, R009): items 10 to 15, 18 to 21
  and 38; item 16 in its header.
- Post and thread detail pages for administrators: items 21, 37 and 38
  on post detail; item 17 on thread detail.
- Dashboard: items 16 and 22 to 33.
- Explore page: items 34 to 36, 39 and 41.
- Operations page: items 42 to 48, 50 and 57; item 49 in the install
  wizard; item 40 on the models page.
- Sutradhar's own admin and review pages (P-VL03): items 7, 51, 52
  and 53.
- Ops wall (item 55): a full-screen view.
- Outgoing: item 54. Public thread pages: item 56, off by default.

The review page also receives new suggestions live, and the review and
health pages show queue depth, processing rate and index coverage
(R123). Those belong to SPEC-008 and SPEC-009 and arrive on SPEC-011's
stream as its `suggestions` and `queues` topics; the visuals there sit
beside them, share the same stream and read the same records, so the
figures always agree.

## Design: architecture

1. Data sources, in the Python service: each register item has a
   registered source named `visuals.<number>-<slug>`, declaring the
   records it reads, the permission scope it needs, how it refreshes
   (live, when opened, or by a scheduled job), its payload limit, its
   text-equivalent builder and its empty-state wording. The registry
   lives with health and metrics (drishti, "sight") in a module with
   the English placeholder name `sutradhar.drishti.visuals`.
2. Routes: `GET /visuals/{source}` returns a snapshot; large WebGL
   datasets come in binary chunks (`GET /visuals/{source}/chunks/n`).
   Live updates are `visual.<source>` topics on SPEC-011's single
   stream route, so each page holds one stream for all its live
   content, progress included. All routes are read-only, on the live
   channel's allow-list, behind its viewer token, and have its polling
   fallback (SPEC-011 part 2).
3. Browser: a small first-party loader finds visual placeholders on a
   page, renders each text equivalent first from the snapshot, then
   loads the needed library when the visual comes into view or is
   opened, and turns the text into the picture. If anything fails the
   text stays.

Actions started from a visual (accept, reject, replay dead letters,
cancel) go through XenForo forms and the action layer (R044) and are
audited. Visuals themselves only read.

## Design: real data only

The owner's rule: "any real system data, plus an error log that is
deduped, plus a live feed that is deduped". Real system data means
events, metrics, traces, the link graph, the index, the audit log,
progress records, the error log, the job registry, health checks and
settings. Metrics and tracing are on by default (R127, ADR-0006): "the
metric store" in the items below is the bundled Prometheus server,
which keeps 15 days, and traces are the sampled traces that the
bundled OpenTelemetry collector stores in Sutradhar's own database for
7 days.

- Every dataset carries `source`, `as_of` and `item`; pages show the
  "as of" time.
- Product code generates no random, sample or demo values. Layouts
  that need randomness use a fixed seed derived from the data, so the
  same data always gives the same picture.
- Motion only ever shows real data: a change, or a measured rate such
  as events a minute. With no change and a zero rate, nothing moves.
- An empty system shows empty states that say what will appear and
  when; never placeholder shapes that look like data.
- A missing source (feature off or not built, library not admitted,
  consent absent) shows "Not available" with the reason and a manual
  link (R079); never zero.
- Tests and manual screenshots use the synthetic fixture forum,
  processed by the real system (R058, R078).

## Design: live updates and deduplication

- Live visuals use SPEC-011's stream route and its polling fallback:
  at most four updates a second per visual; a pause control on every
  page with live content; hidden tabs poll instead of streaming.
- Error deduplication (items 57, 52 and any error list): fingerprint =
  error code + message template (numbers, identifiers and times
  removed) + component + raising function. One row per fingerprint
  with count, first seen, last seen, latest trace id, affected jobs and
  the manual link for the code (R051).
- Feed deduplication (items 58, 55 and 52's grouped mode): entries with
  the same event type, module, forum and outcome within a rolling
  window (default 60 seconds) form one row with a count and the latest
  time; expanding it lists the entries still held.
- One shared function implements each rule, so every list groups the
  same way.

## Design: libraries and lazy loading

- Apache ECharts for charts; sigma.js, with the graph library
  graphology it is built on, for large graphs (WebGL); a WebGL scatter
  library for the embedding explorer and semantic playground (P-VL05);
  D3 modules for other drawings; plain HTML and CSS where nothing
  needs drawing.
- Each is used only after its admission record is approved (SPEC-011
  D-PG06), served from the add-on's own files as its published,
  unchanged prebuilt file at a pinned version with an integrity hash,
  never from an outside server (SPEC-011 P-PG06).
- Each page loads only the loader and the always-visible items. A
  library is downloaded the first time a visual needing it comes into
  view or is opened, then cached by the browser.
- WebGL views (items 6, 9, 24, 34 and 35) load only when the person
  opens them and only if WebGL works; otherwise the text stays.
- Heavy layout work (the galaxy's layout and the explorer's
  projection) runs on the server in scheduled jobs (P-VL06, P-VL07), so
  browsers only draw.

## Design: accessibility

R048 requires WCAG 2.2 AA.

- Text equivalent for every visual, shown first: a summary sentence
  and a "show as table" control whose table holds the same numbers.
- Every interactive visual works by keyboard, with visible focus;
  dragging always has buttons or keys as an alternative (2.5.7);
  controls and clickable targets are at least 24 by 24 pixels (2.5.8);
  single-key shortcuts can be switched off or changed (2.1.4).
- Colour is never the only signal: labels, patterns or icons too.
  Contrast is at least 4.5:1 for text and 3:1 for every drawn part
  needed to understand a picture, in both light and dark themes
  (R047); "faint" or "dim" drawings use dashes, thinner strokes or
  smaller marks, never contrast below 3:1.
- Reduced motion follows `prefers-reduced-motion` and a per-person
  setting (system, reduced, full). Reduced means no movement: changes
  appear at once.
- Anything that moves or updates for more than five seconds has a
  pause control (2.2.2). Nothing flashes or pulses three times a second
  or more (2.3.1).
- Screen-reader announcements are polite and at most once a minute,
  except alerts. Docked items never hide the focused element (2.4.11).

## Design: who sees member text

Member text means post text, excerpts, thread titles and member names.
It appears only in visuals on staff pages protected by XenForo
permissions, which include the moderators' review page. The ops wall
shows none (D-VL08). Public pages show only item 56's allowed content
(D-VL02). Under the outgoing rule (D-VL04), notifications carry totals
only, while pictures an administrator exports may also carry public
thread titles; neither ever carries post text or member names.

## Design: build order and status

The shared foundation comes first, then the first ten in the owner's
order: 1, 2, 6, 10, 12, 22, 34, 40, 43 and 51. Then the rest in
register order: 3 to 5, 7 to 9, 11, 13 to 21, 23 to 33, 35 to 39, 41,
42, 44 to 50, and 52 to 58. An item whose data source does not exist
yet is marked blocked by an external dependency, work moves to the
next, and the blocked item is finished once its source exists (P-VL01,
accepted by the owner).

The register records each item's status in R097's five ledger states:
not started; awaiting design approval; blocked by an external
dependency (naming it); implemented awaiting checks; finished and
verified.
