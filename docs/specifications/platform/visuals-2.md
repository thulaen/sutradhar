# SPEC-012 part 2: failures, privacy, resources, tests and acceptance

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in the
[specification decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Part of [SPEC-012](visuals.md). Requirements: R111; R013, R021 and
R050 (as amended), R022, R023, R029–R033, R048, R052, R054, R075, R078,
R087, R090, R097, R119, R121, R127 and R129; ADR-0006.

## Failure behaviour

- Source unavailable: the visual keeps its last data greyed, with "as
  of 14:02; source unavailable" and the reason. Old data is never
  presented as current.
- Live updates down: polling takes over (SPEC-011); if that fails too,
  a banner gives the time of the last update.
- A library fails to load (blocked, damaged, wrong integrity hash):
  the text equivalent stays, an error code is logged and the load is
  retried once, not repeatedly.
- WebGL missing, or its drawing context lost: the text form is shown,
  with one attempt to restore the context.
- Data larger than a visual's limit: the visual says so in a sentence
  and offers its text form. Very large forums are not grouped: the
  galaxy and the explorer still draw one per thread, loaded in pieces,
  with a notice that drawing may be slow on weaker computers (D-VL07).
- A route slower than ten seconds: "still loading", then an error with
  a retry button.
- Feature off, not built, library not admitted or consent absent:
  "Not available" with the reason and a manual link; never zero.
- Service short of memory (R023): heavy requests, the semantic
  playground first, are refused with a plain reason and error code.
- A scheduled dataset (projection, galaxy layout, clusters, weekly or
  daily figures) whose job failed: the old dataset with its "as of"
  time and a link to the failed run (SPEC-011).
- Public pages (item 56): on any failure the chip is left out without a
  message to members, the post shows normally, and staff see the error
  in the error log. Forum pages always come first.
- Notifications (item 54): failures are retried with growing pauses
  and listed on the health page.
- Times come from the server, so a wrong browser clock does not mislead.

## Privacy and security

- Only guest-visible content enters Sutradhar (R013). Every route also
  filters by current visibility when it serves, so a thread made
  private disappears as soon as the change reaches the service, even
  from datasets made earlier. Deletion and erasure purge or regenerate
  cached datasets, layouts and projections (R052).
- Member text appears only on staff pages with the matching XenForo
  permission, checked through the viewer token's scopes: `visuals.view`
  for figures and `visuals.member_text` for text. The ops wall never
  shows member text (D-VL08).
- Public pages: item 56 only, under D-VL02 and D-VL03, off by default.
  Before it can be switched on, a Claude agent that did not build it
  checks it against a written privacy checklist and decides (D-VL03,
  as the recorder reads the owner's "you check it").
- Outgoing content (item 54 and item 29's image): built through an
  allow-list of fields (D-VL04), so a new field cannot leak by
  accident. Notifications carry totals only; exported pictures may
  also carry titles of guest-visible threads; neither carries post
  text, member names or numbers, IP addresses or error details beyond
  codes.
- Analytics (items 16 and 17): only aggregate figures from F011 and
  F012, measured, never estimated (D-VL05), following their consent
  rules (R030), retention (R031: detailed events 90 days, aggregates
  two years) and source preference (R033).
  No visitor identifier reaches any visual. Because pseudonymous data
  is still personal data (R030), per-thread daily counts below 10 are
  shown as "fewer than 10". Gaps in coverage are shown, never filled.
- The semantic playground's typed text is used for one search and is
  not stored, logged or counted.
- Logs shown in the console carry no post text: they pass the log
  scrubber before storage. Spans carry only IDs, counts and durations,
  enforced by ADR-0006's test; error records add only codes and
  message templates (part 1); the collector's redaction processor is a
  second guard.
- Member text is always inserted as text, never as markup, including
  chart labels and tooltips, so hostile titles cannot run code.
- Pages make no request to any outside server; libraries are local
  with integrity hashes; no code uses `eval`.
- Manual screenshots come only from the synthetic fixture forum
  (R075, R078).

## Admission records needed

Each library is used only after its admission record is approved
(R027, R067); one that fails its review blocks the item that needs it
and a replacement is put to the owner (SPEC-011 D-PG06). A record
names the pinned version, the licence
read from the project's own files, the rights and patent review, where
it runs and its size or memory cost. Licences below are expectations to
confirm.

- Apache ECharts (expected Apache-2.0): charts in admin pages.
- sigma.js and graphology (expected MIT): large graphs in admin pages.
- D3 modules (expected ISC): other drawings, including item 56's
  public mini-graph.
- The WebGL scatter library chosen in P-VL05 (expected MIT).
- OpenTelemetry for Python and PHP (expected Apache-2.0): tracing, on
  by default (R127, replacing P-VL09), used by items 42, 52 and 57;
  PHP use also needs R022's memory measurement.
- The OpenTelemetry collector (expected Apache-2.0), bundled as a
  pinned container image in the default docker compose file (R129)
  with its own memory cap (R121): receives traces and error records
  and feeds them into Sutradhar's database (ADR-0006).
- A Python plotting library for item 54's image (P-VL10), for example
  matplotlib, whose own licence is checked.
- Projection and clustering (P-VL06): numpy, which the model runtime
  already needs, or scikit-learn after admission. If the owner prefers
  UMAP instead, umap-learn, numba, pynndescent and HDBSCAN each need a
  record too.
- Browser test tools are SPEC-011's (P-PG06), including axe-core as a
  test-only tool.

## Memory, storage and removal

SPEC-010 owns the table that accounts for the service's cap (6 GB, or
8 GB once Dell measurements confirm the estimate, R121) and the table
that classifies what uninstall removes. This specification asks for
these rows; where SPEC-010's figures differ, its tables win. The
bundled Prometheus server and OpenTelemetry collector that feed items
1, 7, 42 and 57 sit outside the service's cap, with their own declared
allowance (R121, ADR-0006).

- Memory row "visual layer caches": about 256 MB; a single data request
  uses under 64 MB of working memory, and at most four heavy dataset
  builds run at once.
- Memory row "scheduled visual jobs" (projection, galaxy layout,
  clusters, weekly and daily figures): at most 512 MB, one job at a
  time, at low priority. They pause while the backlog run or a model
  rebuild needs the memory, and read vectors in batches rather than
  holding them all. Figures are measured on the 50,000-thread tier and
  confirmed at million-post scale before release.
- Storage row "visual datasets": projection coordinates, layout
  positions, cluster records and their labels (item 36), weekly
  snapshots, daily reports, the error log and the feed buffer.
- Uninstall: visual datasets, cluster records and labels included, are
  "other owned data" under R050 as amended: uninstall offers an export
  and asks whether to keep or delete them. The caches that only speed
  pages up are deleted without export under SPEC-010's MP-17, which the
  owner accepted, because they are rebuilt from the datasets and from
  records that keep the export offer. Two of them are named here:
  - item 41's cache of scored candidates and judged results, rebuilt
    from SPEC-009's suggestion records and F023's evaluation records;
  - item 56's prepared figures, which PHP keeps in the add-on's own
    XenForo table, rebuilt from the link graph and link provenance;
    the table is removed with the add-on's other tables.

## Time and page budgets

Sizes are compressed, to be measured on Dell and confirmed at
admission.

- Data routes answer in under 500 ms at p95 on the 50,000-thread tier;
  the command palette in under 200 ms.
- Every Sutradhar admin page: loader at most 10 KB; always-visible
  items (7, 51, 52 folded, 53) at most 15 KB more. No chart library is
  loaded until a visual needs it.
- Libraries, loaded on demand and cached: ECharts' full prebuilt file
  at most 400 KB; sigma.js with graphology at most 200 KB; the WebGL
  scatter library at most 200 KB; D3 modules at most 80 KB; first-party
  visual code at most 30 KB per page. If ECharts' prebuilt file exceeds
  its budget, a pinned build step for outside libraries is put forward
  separately.
- Data: at most 1 MB per initial snapshot; WebGL data in binary chunks
  of at most 2 MB.
- Drawing: during live updates no browser task over 50 ms on Dell's
  reference browser profile slowed to a quarter of normal speed;
  animations stop while the tab is hidden.
- PHP (R022): an admin page with visuals adds at most 2 MB of peak
  memory and a public thread page with item 56 at most 1 MB, measured
  with the add-on enabled against disabled.
- 20× gate ([SPEC-007](../performance-gate.md)): each changed
  executable unit (Python data builders, PHP templates and controllers,
  JavaScript) is mapped to a scenario against the simplest correct
  reference, such as recounting all records instead of reading kept
  totals. Expect both measured speed-ups and reviewed limit reports;
  nothing is claimed until measured.

## Testing

Each item starts from its failing acceptance test on fixture data
(R087). Tests run on Dell, or in cloud workers on synthetic fixtures
only; never on Windows (R072, R119). R054's coverage targets apply to
Python, PHP and JavaScript, with SPEC-011's recorded JavaScript gap.

- Unit: each data builder against the synthetic fixture forum (R058),
  with property tests (generated inputs) for invariants such as shares
  between 0 and 1 and treemap areas that sum to the whole; the shared
  deduplication functions; the outgoing allow-list; JavaScript
  functions that turn data into chart settings and text equivalents.
- Integration on Dell: the service, a XenForo test instance and the
  live channel; every page in Playwright with Chromium, Firefox and
  WebKit; network logs for lazy loading; runs with WebGL switched off;
  reduced-motion emulation; light and dark themes.
- Accessibility: axe-core on every page; a keyboard script for every
  interactive item; contrast and target-size checks; a recorded manual
  screen-reader pass. No Safari support is claimed and no real-Safari
  check is made (SPEC-011 D-PG07, the owner's "skip it"; R048 needs
  real Safari only where claimed); automated WebKit tests on Dell
  still run.
- Privacy and security: a hostile-content fixture (titles and posts
  with markup and scripts); a crawler signed in as a guest and as a
  member; allow-list tests for outgoing content; token tests shared
  with SPEC-011.
- Performance: page weight measured at build; browser traces for long
  tasks; route benchmarks; the 20× gate.

## Shared acceptance cases

- AT-VL01: All 58 items are registered with a data source, scope, text
  equivalent and empty state; registering a visual without a data
  source fails.
- AT-VL02: On an empty fixture system every visual shows its empty
  state; a test hook that counts animation-frame callbacks counts none
  over a minute, no CSS animation or transition runs, and no figure
  other than a true count appears.
- AT-VL03: Every dataset carries `source`, `as_of` and `item`, and the
  page shows the "as of" time.
- AT-VL04: A static check finds no random-number use for data in
  product visual code; the same data gives the same layout twice.
- AT-VL05: With reduced motion (system or personal setting), the frame
  callback counter stays at zero and no CSS animation or transition
  runs on any page, while every state is still shown.
- AT-VL06: Each visual's table holds the same values as its dataset.
- AT-VL07: axe-core reports no WCAG 2.2 AA violations on any page in
  either theme; every item's keyboard script completes; every target
  is at least 24 by 24 pixels; no pulse or glow repeats three times a
  second or more; every drawn part needed for meaning meets 3:1.
- AT-VL08: On page load only the loader and always-visible items are
  fetched; ECharts, sigma.js, D3 and the scatter library are requested
  only when a visual needing them comes into view or is opened.
- AT-VL09: With WebGL unavailable, WebGL views show their text form,
  raise no error dialog and do not download WebGL libraries.
- AT-VL10: No page makes a request to another server; every library's
  hash matches its pinned version.
- AT-VL11: Signed in as a guest or an ordinary member, no route or
  page returns member text from the visual layer, except item 56's
  allowed fields when enabled; visual routes refuse their tokens.
- AT-VL12: Hostile fixture titles and posts appear as plain text in
  every label, tooltip and table.
- AT-VL13: A thread made private disappears from every visual dataset
  within 60 seconds of the change reaching the service, including
  scheduled datasets made before.
- AT-VL14: With analytics or SEO signals absent, the affected visuals
  say "Not available" with the reason and never show zero; with the
  bundled Prometheus server or collector stopped, items that read them
  keep their last data greyed with its "as of" time.
- AT-VL15: Live topics arrive on SPEC-011's single stream; the pause
  control stops them; when streaming fails, polling continues and the
  banner shows the last update time.
- AT-VL16: Page-weight and data-size budgets above are met in the
  measured build.
- AT-VL17: At fixture peak traffic, meaning the synthetic fixture's
  recorded events replayed at 1,200 a minute (20 a second, where item 1
  starts grouping dots), no browser task exceeds 50 ms on the slowed
  reference profile.
- AT-VL18: Data routes meet their time budgets on the 50,000-thread
  tier and stay within the memory rows given to SPEC-010; under
  simulated memory pressure the playground is refused first.
- AT-VL19: PHP peak memory added per admin page and per public thread
  page stays within its budget, enabled against disabled.
- AT-VL20: The register shows each item in one of R097's five states;
  a blocked item names its missing dependency.
- AT-VL21: Every list of errors or feed entries uses the shared
  deduplication functions and groups identically.
- AT-VL22: Outgoing messages and exported images contain only
  allow-listed fields; notifications hold no thread titles; exported
  images hold no post text, member names or titles of threads that are
  not guest-visible.
