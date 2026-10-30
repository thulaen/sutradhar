# SPEC-012 part 4: items 15 to 30, Review page and Dashboard

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in the
[specification decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Part of [SPEC-012](visuals.md). The item format is explained in
[part 3](visuals-3.md).

## Review page, continued

### 15. Accept/reject micro-animations

- Review page; CSS and the browser's animation interface. Data: review
  actions as recorded in the audit log.
- Shows: keyboard-first review (A accept, R reject, J and K to move;
  the keys can be switched off or changed). The green ripple and link
  slide play only after the server confirms the action was recorded;
  reject fades the card after confirmation; a failure shows an error
  and no animation.
- Still: an instant change with a confirmation. Text: a polite status
  message such as "accepted; link queued for insertion".
- AT-VL115: No animation starts before the audit record exists; a
  forced server failure plays none and shows the error; every action
  works by keyboard alone.

### 16. Impact counters

- Review page header and Dashboard; HTML. Data: approved links from the
  audit log; measured clicks on them and measured page views that came
  through them, from analytics (F011, F012), aggregates only. No
  estimate of extra views is made (D-VL05).
- Shows: three counters that count up as real increments arrive: links
  approved, clicks on them and page views through them, the last two
  labelled "measured". They appear only once F011 and F012 are approved
  and collecting under their consent rules (R029–R033). A counter whose
  source is not collecting reads "not available: analytics off", never
  zero; coverage gaps are shown (R033).
- Still: numbers change without counting up. Text: labelled counters,
  with the period the measurements cover.
- AT-VL116: The approved count equals the audit records; with analytics
  off the other two read "not available"; with fixture analytics on,
  clicks and views equal the stored aggregates and are labelled
  measured; no estimated figure appears.

### 17. Impact replay

- Thread detail page; ECharts. Data: the thread's inbound-link history
  (audit log) and measured daily traffic aggregates (F011, F012),
  never estimates (D-VL05), within their retention (R031).
- Shows: a time-lapse of the thread gaining links and traffic, with
  play, pause and step controls and a marker for each link. Days
  without collection are gaps. Because pseudonymous data is personal
  data (R030), a day with fewer than 10 views or clicks shows "fewer
  than 10" rather than the exact count.
- Still: a slider replaces playback. Text: a table of dates, inbound
  links and daily views.
- AT-VL117: Markers match the audit log dates; traffic equals the
  stored aggregates; uncollected days are gaps, not zeros; a day with 4
  fixture views shows "fewer than 10" in both chart and table.

### 18. Shared-term highlighting

- Review page; HTML. Data: term and concept matches between source and
  target from extraction and reranking inputs (SPEC-009).
- Shows: hovering or focusing a matched word lights its matches in the
  target excerpt, and the other way round.
- Still and Text: a list of shared terms and concepts.
- AT-VL118: Every lit pair is a stored match; keyboard focus lights the
  same pairs as hover; unmatched words light nothing.

### 19. Bulk cards

- Review page; HTML. Data: pending suggestions grouped by anchor phrase
  and target.
- Shows: suggestions with the same anchor and target in one card with a
  scrolling gallery of every context, all visible before any action.
  Accepting the card records an individual approval of each listed
  suggestion, each checked again when applied (R009, R013, R016), with
  progress through SPEC-011 and failures listed per item.
- Still and Text: the card lists the count and each context.
- AT-VL119: Twenty fixture suggestions with one anchor and target form
  one card of twenty contexts; accepting creates twenty audit records;
  an item made private before applying is skipped and reported.

### 20. Neighbourhood mini-graph

- Each review card; D3 SVG (not WebGL: a page holds many cards). Data:
  the link graph one step around source and target, at most 20 nodes.
- Shows: source, target and their existing links; the proposed link is
  dashed and labelled.
- Still: static. Text: "source links to 4 threads; target has 7 inbound
  links; 1 shared neighbour".
- AT-VL120: Nodes and links equal the graph's one-step neighbourhood,
  capped at 20 with "and 12 more"; the proposed link is dashed.

### 21. "Why no suggestions?"

- Post detail and the review page's search; HTML. Data: candidate and
  rejection records per post (SPEC-009) and the post's reading state
  (SPEC-008: not public, other language, not yet read).
- Shows: for any post, the candidates considered and the rule that
  rejected each, or why the post was not considered. SPEC-009 keeps
  rejection reasons for 30 days (a setting); after that the view shows
  the post's current state and outcome and says "detailed reasons are
  kept for 30 days; this post's have been removed".
- Still and Text: it is a table.
- AT-VL121: A fixture post with three rejected candidates lists all
  three with their recorded rules; after 31 days on a fake clock it
  shows only state and outcome with the removal sentence; a post in
  another language says "skipped: not English" (R098); a non-public
  post says "not eligible" and shows no content.

## Dashboard

### 22. Coverage treemap (first ten)

- Dashboard; ECharts treemap. Data: inbound-link counts per thread from
  the link graph, aggregated per forum: the share of guest-visible
  threads with at least one inbound link.
- Shows: forums as rectangles sized by thread count and coloured by
  that share, with the percentage written where it fits and a numbered
  colour key; colours shift when real updates change coverage.
- Still: colours change at once. Text: a table of forums with threads,
  covered threads and share.
- AT-VL122: Each share equals covered ÷ guest-visible threads; the key
  carries numbers; with no update nothing moves.

### 23. Orphan radar

- Dashboard; ECharts polar chart. Data: orphaned threads (no inbound
  links) per forum, from weekly snapshots taken by a scheduled job.
- Shows: one spoke per forum (the twelve largest, the rest grouped) and
  one ring per week, shrinking as orphans are linked.
- Still: static. Text: a table per forum per week.
- AT-VL123: Values equal the snapshots; a week without a snapshot is
  shown as missing, not zero.

### 24. Time-travel slider

- Dashboard, opening a galaxy view; sigma.js (WebGL, loaded when
  opened). Data: the link graph's history rebuilt from the audit log,
  drawn at the latest layout's positions.
- Shows: a slider, buttons and keys move through dates; the graph shows
  the links that existed on that date; a play button grows it.
- Still: stepping without growth animation. Text: per-date counts of
  threads, links and orphans.
- AT-VL124: On a date the graph holds exactly the links added before it
  and not removed by then; the slider works by keys and buttons without
  dragging.

### 25. Promotion rings

- Dashboard; ECharts. Data: each module's accept rate over its latest
  200 decisions (R060) and its promotion threshold once one is approved
  (D-VL06).
- Shows: a ring filled to the accept rate, with the threshold marker
  only when an approved threshold exists. Version 1 has no automatic
  mode (R009): as the owner decided (D-VL06), "auto" is drawn locked
  and labelled "not available in version 1".
- Still: no fill animation. Text: "module X: 74% accepted over the last
  200 decisions; no approved threshold; auto not available in version
  1".
- AT-VL125: The rate equals accepted ÷ decided over the latest 200;
  without an approved threshold no marker is drawn; "auto" is shown
  locked with "not available in version 1", and no control can start
  automatic insertion.

### 26. Health constellation

- Dashboard and Operations page; D3. Data: the health checks in the
  health registry (drishti): the service's database, models, the
  XenForo API, the index, the bundled Prometheus server and
  OpenTelemetry collector (R127), and any other admitted component.
  Redis appears only if admitted and installed (P-VL08).
- Shows: components as connected nodes; a failing check turns its node
  red and one red ripple runs along its dependency links, marking
  dependants "affected".
- Still: no ripple; affected links drawn red and labelled. Text:
  components with status and last check time.
- AT-VL126: Nodes equal the registered checks; a forced failure marks
  the node and its dependants and names them in text; no Redis node
  exists unless a Redis check is registered.

### 27. Model cards

- Dashboard and models page; ECharts. Data: model health rows and
  latency figures (ADR-0002) for Verified and community models
  (SPEC-010), shown to people with SPEC-010's "View models".
- Shows: one card per model with status, a p95 latency sparkline for
  the last hour, a gauge of p95 against its budget and the community
  badge where it applies.
- Still: no transitions. Text: the health row's values.
- AT-VL127: Each card's status and p95 equal the health row; the gauge
  marks the budget; a degraded model says "degraded" in words.

### 28. Index freshness clock

- Dashboard; ECharts gauge. Data: index metadata (SPEC-008): when the
  last embedding happened. The target is a setting,
  `visuals.freshness_target_minutes`, default 10.
- Shows: a dial of index age against the target and "last embedded 3
  minutes ago".
- Still and Text: the sentence and the target.
- AT-VL128: On a fake clock the age equals now minus the recorded time;
  past the target the text says "behind target"; changing the setting
  moves the target mark.

### 29. Daily mission report

- Dashboard; ECharts sparklines. Data: daily aggregates of links added,
  accept rate and the threads that gained the most inbound links,
  made by a scheduled job.
- Shows: a summary card for the day, exportable by an administrator as
  an image under the outgoing rule (D-VL04): totals, plus the titles
  of the guest-visible threads that gained the most links; never post
  text or member names.
- Still and Text: the card is a text summary with figures.
- AT-VL129: Figures equal the day's aggregates; the data behind an
  exported image holds no member names or post text, and any title in
  it belongs to a thread that is guest-visible when the image is made;
  a quiet day says so.

### 30. Forecast bands

- Dashboard; ECharts. Data: coverage history and a forecast from the
  method in P-VL06.
- Shows: projected coverage with a confidence band and a sentence such
  as "at this rate, 80% by March"; only with at least four weeks of
  history. When the band is wider than 20 percentage points at the
  target date, the sentence says the date cannot be estimated yet.
- Still: static. Text: the sentence and a table of projected values
  with their ranges.
- AT-VL130: With three weeks of history no forecast appears; with the
  fixture history the band equals the method's output and the date in
  the sentence lies inside the band.
