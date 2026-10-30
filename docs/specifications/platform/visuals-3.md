# SPEC-012 part 3: items 1 to 14, Live and the start of Review

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in the
[specification decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Part of [SPEC-012](visuals.md). Numbers and names follow the
[visual layer register](../../dev/visuals.md). Each item lists where it
appears and its drawing tool, its data, what it shows, its
reduced-motion form ("Still"), its text equivalent ("Text") and its
acceptance case, numbered AT-VL1 followed by the register number.

## Live

### 1. Pipeline flow diagram (first ten)

- Live page and ops wall; D3. Data: stage events (event, extract,
  candidates, rank, suggest) with timings, and each stage's throughput
  and p95 latency over five minutes from the service's metric store.
  SPEC-008 and SPEC-009 emit the stage events.
- Shows: dots move between stages at the real event rate, one per
  event up to 20 a second, then one dot per stated number of events
  ("1 dot = 5 events"). The stage with the highest median time per
  item over five minutes glows amber and is labelled "slowest".
- Still: no dots; rates change as numbers. Text: a table of stages with
  events a minute and p95 latency, naming the slowest stage.
- AT-VL101: Replaying 600 recorded fixture events over one minute draws
  600 dots (±2%); stage figures equal the metric store's; the amber
  stage has the highest recorded median; a minute with no events moves
  no dot and the text says "no events in the last minute".

### 2. Live activity stream (first ten)

- Live page; plain HTML. Data: the event stream (post saved,
  suggestions created, time taken) with forum and module.
- Shows: entries such as "Post 48,112 saved → 3 suggestions in 212 ms"
  appear at the top as they happen; the page keeps the newest 500;
  filters by forum and module apply at once; while paused, new entries
  are counted ("12 new") instead of inserted. Post numbers only, no
  post text. Its grouped view is item 58 (D-VL01).
- Still: entries appear without sliding. Text: the list itself, plus
  one polite summary a minute ("38 posts processed in the last
  minute").
- AT-VL102: Each fixture event appears once, in order, with its real
  post number, suggestion count and time; a forum filter shows only
  that forum; while paused nothing is inserted until resumed.

### 3. Heartbeat

- Live page and ops wall; D3. Data: events per minute from the add-on's
  event inbox, including a liveness event the add-on sends once a
  minute, so an idle forum can be told apart from a silent add-on.
- The liveness event needs XenForo's server-based job runner, a
  scheduled server task that runs XenForo's jobs every minute, because
  XenForo's default jobs run only when someone visits and would stop
  on an idle forum. The installer checks for it; without it the
  heartbeat says "liveness unknown: XenForo jobs run only on visits"
  instead of going flat.
- Shows: a pulse whose rate grows with events per minute on a bounded,
  logarithmic scale, from one beat every five seconds up to 2.5 beats a
  second, never three or more; the figure is always written beside it.
  Without a liveness event for three minutes the line goes flat:
  "add-on not sending since 14:02".
- Still: a static line and the figure. Text: "120 events a minute;
  add-on last heard 20 seconds ago".
- AT-VL103: Doubling the fixture event rate never lowers the beat rate,
  which stays below three a second even at 10,000 events a minute;
  stopping liveness events flattens the line within three minutes of
  fake-clock time and shows the time; with the visitor-triggered job
  runner the text says liveness is unknown.

### 4. Now-processing card

- Live page; plain HTML. Data: the current item from progress records
  (SPEC-011 `current`) and anchor candidates from extraction as they
  are found (SPEC-009). Shows member text: staff pages only.
- Shows: the post being processed, with candidate phrases highlighting
  as they are found. Candidate events are produced only while someone
  has the card open.
- Still: highlights appear without fading. Text: the found phrases and
  scores listed under the post.
- AT-VL104: With the card open during a fixture job, the post equals the
  progress record's current item and each highlight matches a recorded
  candidate at its exact position; with the card closed, no candidate
  events are produced.

### 5. Forum treemap with blips

- Live page; ECharts treemap. Data: the XenForo node tree with
  guest-visible thread counts, supplied by PHP (R043), and processing
  events with their forum.
- Shows: rectangles sized by thread count and nested by node; a blip
  lights the forum where a post is processed and fades within two
  seconds, at most one a second per forum.
- Still: a per-forum counter instead of blips. Text: a table of forums
  with thread counts and posts processed in the last five minutes.
- AT-VL105: Areas are proportional to the supplied counts (±1%); a
  fixture event lights only its forum; staff-only forums never appear.

### 6. Link galaxy (first ten)

- Live page; sigma.js and graphology (WebGL), loaded when opened.
  Data: the link graph (threads and inbound links, SPEC-009), positions
  from a nightly server layout job (P-VL07), and accepted-link events.
- Shows: every thread as its own star, sized by inbound links; hubs
  bright, orphans dim but still visible; each accepted link animates
  into place when its event arrives, its new thread placed near its
  neighbours until the next layout run. Titles on hover or focus. On
  very large forums every thread is still drawn, loaded in pieces, with
  a notice that drawing may be slow on weaker computers (D-VL07).
- Still: new links appear without animation. Text: top hubs, the orphan
  count, links added in the last hour and a search that lists a
  thread's links.
- AT-VL106: There is one star per guest-visible thread; star sizes
  follow inbound counts; an accepted-link fixture event adds exactly
  that link; with WebGL off the text form shows and sigma.js is never
  requested; nothing is requested before the galaxy is opened.

### 7. Header sparkline

- The header of Sutradhar's own admin pages (P-VL03); a few lines of
  SVG (P-VL04). Data: p95 latency of suggestion requests per minute
  over the last hour, from the metric store.
- Shows: 60 points and the current value, refreshed each minute.
- Still: no transition. Text: its accessible name, "p95 latency 180 ms
  now; 120 to 240 ms over the last hour".
- AT-VL107: The 60 values equal the store's per-minute p95; the item
  adds at most 3 KB to the page; an hour without requests reads "no
  requests in the last hour".

### 8. Fluid queue gauges

- Live page; D3. Data: event inbox and dead-letter queue depths and
  their capacity limits (F003).
- Shows: two liquid levels rising and draining with the real depths,
  with the number and limit written; a surface ripple only while the
  depth changes.
- Still: plain bars. Text: "inbox 1,204 of 10,000; dead letters 3".
- AT-VL108: Each level equals depth ÷ capacity; at constant depth
  nothing moves; a dead-letter change moves only that gauge.

### 9. Fresh-edge glow

- Inside the link galaxy (6); sigma.js. Data: link-graph edges with
  their creation times.
- Shows: links created in the last hour glow; the glow fades with age
  over 24 hours; older links do not glow. The link itself always keeps
  at least 3:1 contrast; only the glow fades.
- Still: fixed colour bands for "last hour" and "last day", each also
  named in the legend. Text: links added in the last day, with times.
- AT-VL109: On a fake clock a 30-minute-old link is in the last-hour
  band and a 25-hour-old link does not glow; the list matches the
  audit log.

## Review page

### 10. Before/after morph (first ten)

- Review page; HTML and CSS. Data: the suggestion record (source text,
  anchor positions, target) and the exact preview of the change the
  apply step would write (F009).
- Shows: hovering or focusing Accept turns the chosen words into a
  link. The "after" view is that real preview, so it shows exactly
  what will change; no word changes (R010).
- Still: before and after side by side. Text: "will link 'torque
  wrench' to the thread 'Choosing a torque wrench'".
- AT-VL110: The after view equals the apply step's preview exactly; the
  words are identical before and after apart from the link markup;
  keyboard focus on Accept shows the same preview as hover.

### 11. Target preview on hover

- Review page; HTML. Data: the target's title and first-post excerpt
  from the guest-visible index (SPEC-008), reactions and age supplied
  by PHP, and its inbound-link count from the link graph.
- Shows: a card on hover or focus, closed with Escape (WCAG 1.4.13);
  the excerpt is cut at 300 characters.
- Still and Text: the card is text.
- AT-VL111: The card opens on focus and hover and closes on Escape;
  figures equal their sources; a target no longer public shows "not
  available: target no longer public" and no excerpt.

### 12. Score breakdown (first ten)

- Review page; ECharts. Data now: the three scores SPEC-009 stores with
  each suggestion (retrieval, anchor and reranker). Data later: the
  owner's named parts (same forum, popularity, freshness and past
  accepts) come from ranking (F008), which has no specification yet;
  until it exists they read "not available yet" and the item is marked
  partly blocked.
- Shows: a stacked bar, with a radar view on request (P-VL02), and a
  drill-down to single methods. Missing evidence is drawn as
  "waiting", never as zero (R015, R019).
- Still: no transition. Text: "reranker 0.71, retrieval 0.64, anchor
  0.58; popularity and freshness not available yet".
- AT-VL112: Today's bar shows exactly SPEC-009's three stored scores;
  the F008 parts read "not available yet", never zero; a waiting
  method is shown as waiting; the table lists every stored score.

### 13. Confidence halo

- Review page; CSS. Data: the suggestion's score as the register says:
  the reranker score SPEC-009 stores, measured against its minimum
  score. It is not a calibrated confidence.
- Shows: glow strength in steps that follow the score; within a low
  band just above the minimum (a setting) the card is dashed and
  labelled "tentative", so colour is not the only signal.
- Still: the glow is static. Text: "score 0.81, well above the
  minimum".
- AT-VL113: The step follows the stored score; tentative cards carry
  the label; contrast passes in both themes.

### 14. Anchor heat map

- Review page; HTML. Data: anchor candidates with scores, and rejection
  reasons, which SPEC-009 keeps for 30 days (a setting).
- Shows: every candidate phrase in the post shaded by score, rejected
  ones hatched, with the reason on hover and focus. After 30 days a
  rejected phrase says "reason no longer kept (older than 30 days)".
- Still and Text: a table of phrase, score, outcome and reason.
- AT-VL114: Every recorded candidate is shaded at its exact position,
  rejected ones included; each rejected phrase gives its reason on
  keyboard focus, or the "no longer kept" text after 30 days; nothing
  is shaded where no candidate exists.
