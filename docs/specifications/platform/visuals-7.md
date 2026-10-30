# SPEC-012 part 7: settings, decisions, proposals and notes

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in the
[specification decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Part of [SPEC-012](visuals.md).

## Settings and defaults

Section `visuals`, an English placeholder because the glossary has no
name for the visual layer. The owner approved these as listed, together
with SPEC-011's settings (SPEC-011 D-PG08); R127 later removed
`waterfall.enabled`.

- `enabled`: true. `motion` (per person): system. `live_updates`: true.
- `feed.group_window_seconds`: 60; `feed.page_entries`: 500;
  `feed.server_entries`: 10,000.
- `errors.keep_days`: 90 after last seen.
- `galaxy.fresh_hours`: 1; `galaxy.fade_hours`: 24.
- `playground.min_interval_seconds`: 2.
- `freshness_target_minutes`: 10 (item 28).
- `halo.tentative_band`: 0.05 above the minimum score (item 13).
- `analytics.min_count`: 10; smaller daily counts show as "fewer than
  10" (items 16 and 17).
- `chips.enabled`: false (item 56).
- `notify.channel`: none (none, email or ntfy; item 54).
- `opswall.enabled`: false (item 55).
- `waterfall.enabled`: removed. It was listed as false (item 42,
  P-VL09); tracing is now always on (R127), and its sampling, 7-day
  retention and optional OTLP export are monitoring settings under
  ADR-0006, not this section's.
- `palette.shortcut`: Ctrl+K (Cmd+K on Apple computers).

## Decisions (answered by the owner)

These were the choices that needed the owner's judgement; the first
answer was the draft's recommendation. The owner answered them on 30
October 2026. Each question and its options are kept as history, with
the owner's answer under it. Shared questions about libraries, Safari
testing and default settings are SPEC-011's D-PG06, D-PG07 and D-PG08.

- D-VL01: In the follow-up answer the owner asked for "an error log
  that is deduped, plus a live feed that is deduped". The register
  records these as two new visuals, items 57 and 58. Is that right?
  - (a) Yes. Item 57 is an error list that shows each repeated error
    once with a count; item 58 is the same kind of grouped view of the
    live activity stream (item 2), with a switch between the two views.
    Every other list of errors or events groups repeats the same way
    (recommended).
  - (b) No new visuals: grouping repeats is simply a rule for every
    error list and feed, and items 57 and 58 become notes.
  - (c) Yes, two separate visuals with their own panels, without a
    general rule.

  Owner's answer: (a), two items plus a rule.
- D-VL02: Item 56 shows members and guests that Sutradhar is at work.
  What may they see, and when does it start?
  - (a) A small marker beside links Sutradhar inserted ("suggested by
    Sutradhar, approved by a moderator"), drawn when the page is shown
    and never saved into the post; and a "connected to 12 discussions"
    label whose pop-up lists those public threads' titles as ordinary
    links. Never post text, scores, moderator names or visitor figures.
    Public thread titles here are a narrow exception to "member text
    only on admin pages". It stays off until the privacy review passes
    and an administrator switches it on (recommended).
  - (b) The same, but counts only: no thread titles.
  - (c) The thread label only, with no marker beside links.

  Owner's answer: (a), the marker and thread label.
- D-VL03: Who carries out item 56's privacy review before it can be
  switched on? (Every other item has the ordinary reviews of R057.)
  - (a) An agent that did not build it checks it against a written
    privacy checklist, and the owner gives the final yes or no by
    answering one question (recommended).
  - (b) The owner reviews it personally.
  - (c) An outside privacy professional whom the owner engages.

  Owner's answer: "you check it". Recorder's interpretation: an
  independent Claude agent that did not build item 56 carries out the
  review against the written privacy checklist and decides; no
  separate owner question is asked.
- D-VL04: What may leave the admin pages in notifications (item 54)
  and exported pictures (item 29)?
  - (a) Only totals: counts, percentages, durations, task names,
    statuses and charts drawn from them, plus a link to the admin page,
    which needs signing in. No post text, thread titles, member names
    or numbers, IP addresses, or error details beyond codes
    (recommended).
  - (b) As (a), but public thread titles may appear in pictures an
    administrator exports, though never in notifications.

  Owner's answer: (b), totals, with titles in exports. Items 29 and 54
  and part 2's outgoing rule apply it.
- D-VL05: Items 16 and 17 show clicks and visits. How should they
  handle visitor analytics?
  - (a) Approved links are shown now. Clicks and visits appear only
    once the analytics features (F011, F012) are approved and
    collecting under their consent rules (R029–R033), and only as
    totals. "Estimated extra page views" uses a comparison method fixed
    before measuring, shown with its range, and says "not available"
    until that method is approved (recommended).
  - (b) As (a), but instead of an estimate of extra views, show the
    page views that came through the approved links, measured and
    labelled as such.

  Owner's answer: (b), measured views, not estimates. Items 16 and 17
  apply it.
- D-VL06: Items 25 and 48 show modules moving towards "auto", but
  version 1 has no automatic linking (R009). What should they show?
  - (a) The shadow and review stages; "auto" drawn locked and labelled
    "not available in version 1"; threshold marks only once a threshold
    is approved (R060) (recommended).
  - (b) Leave the auto stage out entirely.
  - (c) Build items 25 and 48 only after a specification for moving
    modules between stages is approved.

  Owner's answer: (a), auto shown locked.
- D-VL07: The owner asked for one star per thread in the link galaxy
  (item 6) and one point per thread in the embedding explorer (item
  34). On a very large forum, a weaker computer may draw these slowly.
  What then?
  - (a) Always one per thread, loaded in pieces, with a notice that
    drawing may be slow on weaker computers (recommended).
  - (b) Above a size the administrator sets, start grouped by forum and
    open single threads on request.
  - (c) Above that size, draw only the busiest threads and list the
    rest.

  Owner's answer: (a), always one per thread.
- D-VL08: The ops wall (item 55) is meant for a spare screen that
  others can see. How does it sign in, and what may it show?
  - (a) Its own read-only token, created by an administrator for that
    screen and revocable at any time, showing figures only and never
    member text (recommended).
  - (b) An ordinary staff sign-in left open, showing the same as other
    admin pages.

  Owner's answer: (a), its own read-only token.

## Proposals

Technical choices, accepted as a group by the owner on 30 October 2026,
except P-VL09, which R127 replaced. P-VL01 and P-VL03, first put to the
owner in wrong summaries, were asked again in their real wording and
accepted.

- P-VL01: If the next item's data source does not exist yet, the item
  is marked "blocked by an external dependency", work moves to the next
  item, and the blocked item is finished as soon as its source exists.
  This reads the owner's order as the order of work, not a reason to
  stop.
- P-VL02: The score breakdown (item 12) is a stacked bar, with a radar
  view on request.
- P-VL03: The always-visible items (7, 51, 52 and 53) appear on
  Sutradhar's own admin and review pages, not on every XenForo admin
  page.
- P-VL04: The small always-visible pictures (items 7 and 53) are drawn
  with a few lines of SVG or small D3 modules, so ECharts is not
  loaded on every page.
- P-VL05: The WebGL scatter library is regl-scatterplot, a small
  library made for large scatter plots; alternatives are deck.gl's
  scatter layer or ECharts GL.
- P-VL06: The explorer's map (item 34) uses incremental PCA, a method
  that fits in batches and so stays within its memory row. It is
  fitted on a sample, new threads are placed with the fitted
  transform, and it is refitted only after a model change, turned to
  match the previous map so points move as little as possible. Topic
  bubbles (item 36) use mini-batch k-means clusters on the reduced
  vectors, labelled with each cluster's distinctive title words. The
  forecast (item 30) is a linear trend over the last eight weeks with a
  bootstrap range. Each needs method admission (F006). UMAP was
  considered: it cannot be fitted from batches within the memory row,
  and a nightly refit would move every point; using it would also need
  umap-learn, numba, pynndescent and HDBSCAN admitted.
- P-VL07: The galaxy's layout (item 6) is computed on the server each
  night by an independent implementation of a published
  force-directed method (ForceAtlas2), starting from the previous
  positions so the picture stays stable. Browsers only draw.
- P-VL08: The health constellation (item 26) draws the registered
  health checks; Redis appears only once admitted and installed.
- P-VL09: Tracing for item 42 is off by default. When an administrator
  switches it on, PHP and the service send traces only to a small
  receiver inside the service on the same host, kept in memory;
  nothing leaves the host. Not accepted: replaced by R127 and ADR-0006.
  Tracing is on by default; PHP and the service send spans to the
  bundled OpenTelemetry collector, which feeds sampled traces into
  Sutradhar's own database for 7 days, and item 42 is the built-in
  viewer over them.
- P-VL10: Item 54's messages are sent by PHP, through XenForo's mail
  system or its web client to an ntfy server and token the
  administrator enters. The service supplies the text and a PNG chart
  drawn by an admitted Python plotting library, such as matplotlib.
- P-VL11: Item 56 reads prepared figures from the add-on's own XenForo
  table, written through PHP, so public pages never call the service.

## Recorder's notes

These are the drafter's readings, not owner decisions.

- "Admin pages" in "member text appears only on admin pages" is read as
  staff pages protected by XenForo permissions, including the
  moderators' review page, which must show posts (R009).
- "Any real system data" is read to include progress records, the
  error log, the job registry, health checks and settings, alongside
  the owner's original list.
- SPEC-011 owns the single live stream; this layer adds `visual.`
  topics and holds no channel of its own.
- Item 3 adds a once-a-minute liveness event, and needs XenForo's
  server-based job runner, so silence can be told apart from an idle
  forum. Its bounded beat scale keeps below WCAG's three-a-second
  limit while still rising with the rate.
- Item 4 relies on SPEC-011's `current` field.
- Item 7's "p95 latency" is read as suggestion requests.
- Item 11's 300-character excerpt is a proposal.
- Item 12: SPEC-009 stores three scores today; the owner's other parts
  depend on F008, so the item is partly blocked until F008 exists.
- Item 13 follows the register ("the suggestion's score"); SPEC-009
  stores no calibrated confidence, so the halo uses the reranker score.
- Item 15 animates after the server confirms an action, not on the
  click, so it never shows something that did not happen.
- Item 19's accept is read as an individual approval of each shown
  suggestion, keeping R009.
- Items 14 and 21 depend on SPEC-009's 30-day retention of rejection
  reasons.
- Item 29's "top boosted threads" is read as the threads that gained
  the most inbound links that day.
- Item 34's neighbours come from the stored vectors; positions on the
  flat map are only approximate.
- Item 38 is a replay of stored scores; live reranking shows as
  SPEC-011's per-post analysis row.
- Item 40 depends on SPEC-010's trial running both models side by side.
- Item 54's image needs a plotting library the owner did not name.
- Item 57 includes the add-on's own errors sent by PHP; XenForo's
  general error log is not read (R043).
- R092 kept product instrumentation off by default; R127 replaces it
  for metrics and tracing, which are now on by default. "The metric
  store" is read as the bundled Prometheus server, and the owner's
  "showing error logs on the gui" as item 57 fed through the
  collector, with items 42, 52, 57 and 58 linked by trace IDs.
- Item 41's cache and item 56's prepared figures are read as caches
  under SPEC-010's MP-17, and cluster records and labels as visual
  datasets (part 2).
- "D3 for the rest" is read as drawing work that ECharts, sigma.js and
  the scatter library do not cover; lists and cards are plain HTML.
- The owner's "drawn faintly" (item 32) and "orphans dim" (item 6) are
  met with dashes and smaller marks, because WCAG 1.4.11 needs 3:1
  contrast for drawn parts that carry meaning.
- Library files are third-party material under R089's classification:
  never edited and not counted as first-party; classifying them is a
  protected change.
- Many items depend on features without approved specifications (F002,
  F005, F008, F009, F011 to F013, F021 to F023) and stay blocked until
  those exist; the first ten also need SPEC-008 to SPEC-011 built.
- Memory and storage figures are requests for rows in SPEC-010's
  tables; visual datasets are "other owned data" at uninstall (R050 as
  amended).
