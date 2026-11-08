# 0017: Visual layer on real data

The visual layer adds charts, maps and live drawings that show what
Sutradhar is really doing. Every one draws only genuine system records:
nothing is made up to look busy. Each has a plain text or table form,
respects people who ask for less motion, and builds its heavy drawing
only when needed. Text written by members appears only on staff
screens. It has 58 items, built on SDS components in the Dioxus app,
with a few in prepared form on the site's pages. It is roadmap Phase 3
items 1 and 3: the shared foundation is part of Sutradhar Boot and its
drawing parts are SDS components; each item is built when its data
exists.

## What it does

### Words used here

- GPU drawing: drawing with the graphics hardware, for very large
  pictures such as the link graph.
- Text equivalent: a sentence or table with the same information as a
  picture.
- Reduced motion: an operating-system setting asking screens not to
  animate (`prefers-reduced-motion`).
- Grouped (deduplicated): repeats shown once with a count.
- Treemap: rectangles sized by a number, nested like folders.
- Force-directed layout: points pushed apart and pulled together by
  their links until the picture settles.
- Each item below gives where it appears, its data, what it shows, its
  reduced-motion form ("Still"), its text equivalent ("Text") and its
  test.

### Where visuals appear

- Desktop app, Live screen: items 1 to 6, 8, 9 and 58.
- Review screen in the app (staff only): items 10 to 15, 18 to 21 and
  38, with item 16 in its header. The site's admin review page shows
  the same items in prepared form (see "Site pages").
- Post and thread detail screens: items 21, 37 and 38 (post); item 17
  (thread).
- Dashboard: items 16 and 22 to 33.
- Explore screen: items 34 to 36, 39 and 41.
- Operations screen: items 42 to 48, 50 and 57; item 49 in the install
  wizard; item 40 on the models screen.
- Every Sutradhar screen: items 7, 51, 52 and 53.
- Ops wall (item 55): a full-screen window.
- Outgoing: item 54. Public thread pages: item 56, only once reviewed.
- Live suggestions and the queue and coverage figures (specs 0022,
  0023) arrive on the stream of spec 0016 as its `suggestions` and
  `queues` topics; the visuals beside them read the same records, so
  figures always agree.

### Architecture

1. Data sources: each item has a registered source,
   `visuals.<number>-<slug>`, declaring the records it reads, the
   permission it needs, how it refreshes (live, when opened, or by a
   scheduled job), its payload limit, its text-equivalent builder and
   its empty-state wording. The registry sits with health and metrics
   in drishti ("sight"), as `drishti::visuals`. Registering a visual
   without a source fails.
2. Routes on the management API: `GET /api/v1/visuals/{source}` returns
   a snapshot; large GPU datasets come in binary chunks
   (`.../chunks/{n}`). Live updates are `visual.<source>` topics on the
   one stream of spec 0016, with its polling fallback. All are
   read-only and checked with Cedar.
3. Screens show the text equivalent first, from the snapshot, then
   build the picture when it comes into view or is opened. If anything
   fails, the text stays.
4. The service decides; screens only lay out what it sends, and every
   preview comes from the service (spec 0018).
5. Actions started from a visual (accept, reject, replay dead letters,
   cancel) go through the action layer, are checked with Cedar and are
   audited. Visuals themselves only read.

### Real data only

Real data means events, metrics, traces, the link graph, the index, the
audit log, progress records, the error log, the job registry, health
checks and settings. Metrics and tracing are always on: "the metric
store" below is the bundled metrics store, kept 15 days, and traces are
the sampled traces stored in Sutradhar's database for 7 days.

- Every dataset carries `source`, `as_of` and `item`; screens show the
  "as of" time.
- Product code makes no random, sample or demo values. Layouts that
  need randomness use a fixed seed from the data, so the same data
  always gives the same picture.
- Motion shows only real change or a measured rate. With no change and
  a zero rate, nothing moves.
- An empty system shows empty states that say what will appear and
  when; never shapes that look like data.
- A missing source (feature off or not built, crate not admitted,
  consent absent) shows "Not available" with the reason and a manual
  link; never zero.
- Tests and manual screenshots use the synthetic fixture site,
  processed by the real system.

### Live updates and grouping

- Live visuals use the stream of spec 0016: at most four updates a
  second per visual; a pause control on every screen with live content;
  hidden windows poll instead.
- Error grouping (items 52, 57 and any error list): fingerprint = error
  code + message template (numbers, ids and times removed) + component
  + raising function. One row per fingerprint with count, first and
  last seen, latest trace id, affected jobs and the code's manual link.
- Feed grouping (items 52, 55 and 58): entries with the same event
  type, module, site and outcome within a rolling window (default 60
  seconds) form one row with a count and the latest time; expanding it
  lists the entries still held.
- One shared Rust function implements each rule, so every list groups
  the same way.

### Drawing and loading

- SDS components draw charts, graphs, maps and lists. Plain layout is
  used where nothing needs drawing.
- A screen builds only its always-visible items at first. A heavy
  drawing is built the first time it comes into view or is opened.
- GPU views (items 6, 9, 24, 34 and 35) open only when the person opens
  them and only if GPU drawing works; otherwise the text stays.
- Heavy layout work (the galaxy layout and the explorer's projection)
  runs in scheduled ganit jobs, so screens only draw.

### Accessibility (WCAG 2.2 AA)

- Every visual has a text equivalent shown first: a summary sentence
  and a "Show as table" control with the same numbers.
- Every interactive visual works by keyboard, with visible focus;
  dragging always has buttons or keys too; targets are at least 24 by
  24 pixels; single-key shortcuts can be switched off or changed.
- Colour is never the only signal. Contrast is at least 4.5:1 for text
  and 3:1 for every drawn part needed for meaning, in light and dark
  themes. "Faint" or "dim" drawings use dashes, thinner strokes or
  smaller marks, never contrast below 3:1.
- Reduced motion follows the system setting and a per-person setting
  (system, reduced, full). Reduced means no movement: changes appear at
  once.
- Anything that moves or updates for more than five seconds has a pause
  control. Nothing flashes or pulses three times a second or more.
- Screen-reader announcements are polite and at most once a minute,
  except alerts. Docked items never hide the focused element.

### Who sees member text

Member text means post text, excerpts, thread titles and member names.
It appears only on staff screens: the owner's app and the site's admin
pages behind the site's permissions, including the review page. The ops
wall shows none. Public pages show only item 56's allowed content.
Notifications carry totals only; pictures an administrator exports may
also carry titles of guest-visible threads; neither ever carries post
text or member names.

### Site pages

The site never calls the owner's PC. For the site's admin review page
and item 56, the service prepares the content (markup, CSS and drawn
SVG) and sends it to the add-on in signed messages. The add-on stores
it and shows it. Site pages run no hand-written page scripts: hover and
focus effects use CSS, pop-ups use disclosure elements, and drawings
are SVG made by the service.

### Build order

The shared foundation comes first, then the first ten in the owner's
order: 1, 2, 6, 10, 12, 22, 34, 40, 43 and 51. Then the rest in
register order: 3 to 5, 7 to 9, 11, 13 to 21, 23 to 33, 35 to 39, 41,
42, 44 to 50, and 52 to 58. If an item's data source does not exist
yet, work moves to the next item and comes back once the source exists.

### Live items

**1. Pipeline flow** (first ten). Live screen and ops wall. Data: stage
events (event, extract, candidates, rank, suggest) with timings, and
each stage's throughput and p95 over five minutes from the metric
store. Shows: dots move between stages at the real rate, one per event
up to 20 a second, then one dot per stated number ("1 dot = 5
events"). The stage with the highest median time per item over five
minutes is amber and labelled "slowest". Still: no dots; rates change
as numbers. Text: stages with events a minute and p95, naming the
slowest. Test AT-VL101: 600 replayed events over a minute draw 600 dots
(within 2%); figures equal the store's; amber is the highest median; a
minute with no events moves nothing and says "no events in the last
minute".

**2. Live activity stream** (first ten). Live screen. Data: the event
stream (post saved, suggestions made, time taken) with site and module.
Shows: entries such as "Post 48,112 saved, 3 suggestions in 212 ms"
appear at the top; the newest 500 are kept; site and module filters
apply at once; while paused, new entries are counted ("12 new"). Post
numbers only, no post text. Its grouped view is item 58. Still: no
sliding. Text: the list, plus one polite summary a minute ("38 posts
processed in the last minute"). Test AT-VL102: each event appears once,
in order, with its real figures; a site filter shows only that site;
nothing is inserted while paused.

**3. Heartbeat.** Live screen and ops wall. Data: events per minute from
the add-on's event inbox, including a liveness mark the add-on records
once a minute, so an idle site differs from a silent add-on. The mark
needs the site's server-based job runner (a scheduled server task that
runs the site's jobs every minute), since visit-driven jobs stop on an
idle site; the installer checks for it, and without it the heartbeat
says "liveness unknown: site jobs run only on visits". Shows: a pulse
whose rate grows with events on a bounded logarithmic scale, from one
beat every five seconds to 2.5 a second, never three or more, with the
figure beside it. With no liveness mark for three minutes the line goes
flat: "add-on not sending since 14:02". Still: a static line and the
figure. Text: "120 events a minute; add-on last heard 20 seconds ago".
Test AT-VL103: doubling the rate never lowers the beat, which stays
below three a second at 10,000 events a minute; stopping marks flattens
the line within three fake-clock minutes; with visit-driven jobs the
text says liveness is unknown.

**4. Now-processing card.** Live screen; member text. Data: the current
item from progress records (spec 0016 `current`) and anchor candidates
as found (spec 0023). Shows: the post with candidate phrases lighting
up as found. Candidate events are made only while the card is open.
Still: no fading. Text: phrases and scores under the post. Test
AT-VL104: the post equals the record's current item; each highlight
matches a recorded candidate at its exact place; with the card closed
no candidate events are made.

**5. Site treemap with blips.** Live screen. Data: the site's forum tree
with guest-visible thread counts, supplied by the add-on, and processing
events with their forum. Shows: rectangles sized by thread count and
nested as the tree; a blip lights a forum where a post is processed and
fades within two seconds, at most one a second per forum. Still: a
counter per forum. Text: forums with thread counts and posts processed
in five minutes. Test AT-VL105: areas within 1% of the counts; an event
lights only its forum; staff-only forums never appear.

**6. Link galaxy** (first ten). Live screen; GPU, opened on request.
Data: the link graph (spec 0023), positions from a nightly server
layout, and accepted-link events. Shows: every thread as its own star,
sized by inbound links; hubs bright; orphans dim but visible; an
accepted link animates in, its new thread placed near its neighbours
until the next layout. Titles on hover or focus. Very large sites still
draw every thread, loaded in pieces, with a notice that drawing may be
slow on weaker computers. Still: new links appear at once. Text: top
hubs, the orphan count, links added in the last hour, and a search
listing a thread's links. Test AT-VL106: one star per guest-visible
thread; sizes follow inbound counts; an event adds exactly that link;
without GPU drawing the text shows; nothing is built before opening.

**7. Header sparkline.** Every screen's header; a small SVG. Data: p95
time of suggestion requests per minute over the last hour. Shows: 60
points and the current value, refreshed each minute. Still: no
transition. Text: "p95 latency 180 ms now; 120 to 240 ms over the last
hour". Test AT-VL107: the 60 values equal the store's; an hour without
requests reads "no requests in the last hour".

**8. Queue gauges.** Live screen. Data: event inbox and dead-letter
depths and their limits (spec 0019). Shows: two liquid levels with the
number and limit written; a ripple only while the depth changes.
Still: plain bars. Text: "inbox 1,204 of 10,000; dead letters 3". Test
AT-VL108: each level is depth / capacity; at constant depth nothing
moves; a dead-letter change moves only that gauge.

**9. Fresh-link glow.** Inside the galaxy. Data: links with creation
times. Shows: links from the last hour glow, fading over 24 hours; the
link itself always keeps 3:1 contrast, only the glow fades. Still:
fixed bands "last hour" and "last day", named in the legend. Text:
links added in the last day, with times. Test AT-VL109: on a fake clock
a 30-minute link is in the last-hour band and a 25-hour link does not
glow; the list matches the audit log.

### Review items

**10. Before and after** (first ten). Data: the suggestion (source
text, anchor positions, target) and the exact preview of the change the
apply step would write (spec 0028). Shows: hovering or focusing Accept
turns the chosen words into a link; the after view is that real
preview; no word changes. Still: side by side. Text: "will link 'torque
wrench' to the thread 'Choosing a torque wrench'". Test AT-VL110: the
after view equals the apply preview exactly; words are identical apart
from link markup; focus shows the same as hover.

**11. Target preview.** Data: the target's title and first-post excerpt
from the guest-visible index (spec 0022), reactions and age from the
add-on, inbound count from the link graph. Shows: a card on hover or
focus, closed with Escape; excerpt cut at 300 characters. Still and
Text: the card is text. Test AT-VL111: opens on focus and hover, closes
on Escape; figures equal their sources; a target no longer public says
"not available: target no longer public" with no excerpt.

**12. Score breakdown** (first ten). Data: the three scores stored with
each suggestion (retrieval, anchor and reranker, spec 0023); later the
owner's named parts (same forum, popularity, freshness, past accepts)
from ranking (spec 0027), which read "not available yet" until ranking
exists. Shows: a stacked bar, a radar view on request, and a drill-down
to single methods. Missing evidence is drawn as "waiting", never zero.
Still: no transition. Text: "reranker 0.71, retrieval 0.64, anchor
0.58; popularity and freshness not available yet". Test AT-VL112: the
bar shows exactly the three stored scores; the later parts read "not
available yet"; a waiting method shows as waiting; the table lists
every stored score.

**13. Score halo.** Data: the stored reranker score against its minimum
(not a calibrated confidence). Shows: glow in steps following the
score; within a band just above the minimum (a setting) the card is
dashed and labelled "tentative". Still: static glow. Text: "score 0.81,
well above the minimum". Test AT-VL113: the step follows the score;
tentative cards carry the label; contrast passes in both themes.

**14. Anchor heat map.** Data: anchor candidates with scores and
rejection reasons, kept 30 days (a setting, spec 0023). Shows: each
candidate phrase shaded by score, rejected ones hatched, the reason on
hover and focus; after 30 days "reason no longer kept (older than 30
days)". Still and Text: a table of phrase, score, outcome and reason.
Test AT-VL114: every candidate is shaded at its exact place; each
rejected phrase gives its reason on focus or the "no longer kept" text;
nothing is shaded where no candidate exists.

**15. Accept and reject feedback.** App review screen. Data: review
actions in the audit log. Shows: keyboard-first review (one key each to
accept, reject and move up or down; keys can be switched off or changed). The
accept ripple plays only after the service confirms the action was
recorded; reject fades the card after confirmation; a failure shows an
error and no animation. Still: an instant change with a confirmation.
Text: a polite message such as "accepted; link queued for insertion".
Test AT-VL115: no animation before the audit record exists; a forced
failure plays none and shows the error; every action works by keyboard.

**16. Impact counters.** Review header and Dashboard. Data: approved
links from the audit log; measured clicks on them and measured page
views through them from analytics (spec 0034), totals only; no estimate
of extra views. Shows: three counters that count up as real increments
arrive, the last two labelled "measured" and shown only once analytics
is collecting under its consent rules. A source not collecting reads
"not available: analytics off", never zero; coverage gaps are shown.
Still: no counting up. Text: labelled counters with the period covered.
Test AT-VL116: the approved count equals the audit records; with
analytics off the others read "not available"; with fixture analytics
on, figures equal the stored totals and are labelled measured.

**17. Impact replay.** Thread detail. Data: the thread's inbound-link
history (audit log) and measured daily traffic totals (spec 0034)
within their retention. Shows: a time-lapse with play, pause and step
controls and a marker per link; uncollected days are gaps; a day with
fewer than 10 views or clicks shows "fewer than 10". Still: a slider.
Text: dates, inbound links and daily views. Test AT-VL117: markers match
audit dates; traffic equals stored totals; gaps are not zeros; a day
with 4 views shows "fewer than 10" in chart and table.

**18. Shared terms.** Data: term and concept matches between source and
target (spec 0023). Shows: hovering or focusing a matched word lights
its matches in the target excerpt, and back. Still and Text: a list of
shared terms. Test AT-VL118: every lit pair is a stored match; focus
lights the same as hover; unmatched words light nothing.

**19. Bulk cards.** Data: suggestions waiting for review, grouped by
anchor phrase and target. Shows: one card with every context visible
before any action. Accepting records an individual approval of each
listed suggestion, each checked again when applied, with progress
through spec 0016 and failures listed per item. Still and Text: the
count and each context. Test AT-VL119: twenty suggestions with one
anchor and target form one card of twenty; accepting makes twenty audit
records; an item made private before applying is skipped and reported.

**20. Neighbourhood mini-graph.** Each review card; SVG, not GPU. Data:
the link graph one step around source and target, at most 20 nodes.
Shows: source, target and existing links; the proposed link dashed and
labelled. Still: static. Text: "source links to 4 threads; target has 7
inbound links; 1 shared neighbour". Test AT-VL120: nodes and links equal
the one-step neighbourhood, capped at 20 with "and 12 more"; the
proposed link is dashed.

**21. Why no suggestions?** Post detail and review search. Data:
candidate and rejection records per post (spec 0023) and the post's
reading state (spec 0022: not public, other language, not yet read).
Shows: the candidates and the rule that rejected each, or why the post
was not considered; after the 30-day keep, its current state and
outcome with "detailed reasons are kept for 30 days; this post's have
been removed". Still and Text: a table. Test AT-VL121: three rejected
candidates are listed with their rules; after 31 fake-clock days only
state, outcome and the sentence; another language says "skipped: not
English"; a non-public post says "not eligible" with no content.

### Dashboard items

**22. Coverage treemap** (first ten). Data: inbound counts per thread,
per forum: the share of guest-visible threads with at least one inbound
link. Shows: forums sized by thread count and coloured by share, the
percentage written where it fits, a numbered key. Still: colours change
at once. Text: forums with threads, covered threads and share. Test
AT-VL122: each share is covered / guest-visible; the key has numbers;
with no update nothing moves.

**23. Orphan radar.** Data: orphaned threads per forum from weekly
snapshots. Shows: a spoke per forum (the twelve largest, the rest
grouped) and a ring per week. Still: static. Text: a table per forum
per week. Test AT-VL123: values equal the snapshots; a missing week
shows as missing, not zero.

**24. Time-travel slider.** Opens a galaxy view; GPU. Data: link-graph
history rebuilt from the audit log, at the latest layout's positions.
Shows: a slider, buttons and keys move through dates; a play button
grows the graph. Still: stepping without growth. Text: per-date threads,
links and orphans. Test AT-VL124: a date holds exactly the links added
before it and not removed by then; it works by keys and buttons.

**25. Promotion rings.** Data: each module's accept rate over its latest
200 decisions and its approved promotion threshold (spec 0015). Shows: a
ring filled to the rate, a threshold mark only when one is approved;
"auto" drawn locked and labelled "not available in version 1". Still:
no fill animation. Text: "module X: 74% accepted over the last 200
decisions; no approved threshold; auto not available in version 1".
Test AT-VL125: rate is accepted / decided over 200; no mark without a
threshold; auto shown locked; no control can start automatic insertion.

**26. Health constellation.** Dashboard and Operations. Data: the
registered health checks: database, models, the site link, the index,
the metrics store, the trace collector and any other admitted part.
Shows: parts as connected points; a failing check turns its point red
and one ripple runs along its dependency links, marking dependants
"affected". Still: no ripple; affected links red and labelled. Text:
parts with status and last check time. Test AT-VL126: points equal the
checks; a forced failure marks the point and its dependants and names
them; no point exists without a registered check.

**27. Model cards.** Dashboard and models screen, for "View models".
Data: model health rows and latency (spec 0024). Shows: a card per
model with status, a one-hour p95 sparkline, a gauge of p95 against its
budget and the community badge where it applies. Still: no
transitions. Text: the row's values. Test AT-VL127: status and p95
equal the row; the gauge marks the budget; degraded says "degraded".

**28. Index freshness clock.** Data: when the last embedding happened
(spec 0022) and the target `visuals.freshness_target_minutes` (10).
Shows: a dial of index age against the target and "last embedded 3
minutes ago". Still and Text: the sentence and target. Test AT-VL128:
age equals now minus the recorded time; past target says "behind
target"; changing the setting moves the mark.

**29. Daily report.** Data: daily totals of links added, accept rate and
threads that gained most inbound links, by a scheduled job. Shows: a
summary card, exportable as an image: totals plus titles of the
guest-visible threads that gained most links; never post text or member
names. Still and Text: a text summary. Test AT-VL129: figures equal the
day's totals; the exported image holds no member names or post text,
and any title is of a thread guest-visible when made; a quiet day says
so.

**30. Forecast bands.** Data: coverage history and a forecast. Shows:
projected coverage with a range and "at this rate, 80% by March", only
with at least four weeks of history; when the range is wider than 20
points at the target date, it says the date cannot be estimated yet.
Still: static. Text: the sentence and projected values with ranges.
Test AT-VL130: with three weeks no forecast; with the fixture history
the band equals the method's output and the date lies inside it.

**31. Signal wall.** Data: SEO signal statuses (sanket, spec 0044).
Shows: a tile per signal, coloured and worded by status; a changed tile
pulses twice, slower than three a second, then rests. Still: no pulse;
"changed at 14:02" on the tile. Text: the tiles. Test AT-VL131: tiles
equal the records; before SEO signals exist the wall says "not
available"; a change pulses only that tile, twice.

**32. Ghost lines.** Every time chart. Data: the same metric seven days
earlier. Shows: last week's line behind, dashed and thinner, still 3:1,
named in the legend and hideable. Text: tables gain a "same time last
week" column. Test AT-VL132: ghost values equal the history; missing
history leaves gaps; contrast at least 3:1 in both themes.

**33. Heat calendar.** Data: links added per day (audit log). Shows: a
square per day for a year, coloured in steps with a numbered key.
Text: a table by month and day and the busiest day. Test AT-VL133:
counts equal the audit records; focus on a square reads date and count.

### Exploration items

**34. Embedding explorer** (first ten). Explore screen; GPU. Data:
thread summary vectors, one point per thread, projected to two
dimensions by a scheduled job and coloured by forum. The job pauses
during the backlog run and model rebuilds. Shows: every thread as a
point; choosing one lists its nearest neighbours from the stored
vectors, not map distance; a changed summary vector moves its point;
new threads drop in. Very large sites still draw every thread, in
pieces, with the slow-drawing notice. Still: points jump. Text: a
searchable thread list with forum and neighbours, and a cluster
summary. Test AT-VL134: one point per guest-visible thread with a
vector; neighbours equal an exact search; one changed vector moves only
its point; without GPU the list shows; the job pauses when a rebuild
starts.

**35. Semantic playground.** With the explorer's map. Data: the typed
sentence embedded by the production model (specs 0022, 0024), searched
against thread summary vectors only. Shows: the nearest threads light
up and a ranked list appears. Searches run below the live lane (spec
0023), so they never delay new posts; at most one every two seconds per
person; the typed text is not stored or logged; refused with a reason
when memory is short. Text: the ranked list with scores. Test
AT-VL135: results equal an exact search; the text is in no log, store
or metric; a second search within two seconds is delayed, then run;
a search during live-lane work waits.

**36. Topic bubbles.** Data: clusters of thread vectors with labels,
from a scheduled job. Shows: bubbles sized by thread count and
labelled; click or Enter opens a cluster's threads. Still: no zoom.
Text: clusters with label, size and top threads. Test AT-VL136: sizes
equal cluster sizes; opening lists exactly its threads; labels equal
the stored output.

**37. Post X-ray.** Post detail. Data: one post's BBCode, extracted text,
excluded zones (quotes, code, spoilers, signatures), candidates and
final links (specs 0022, 0023). Shows: layers switched by buttons,
aligned; guest-visible posts only. Text: each layer as text. Test
AT-VL137: zones match the extractor's spans; buttons report pressed
state; a non-public post says "not eligible".

**38. Reranker replay.** Post detail and review. Data: the reranker
scores stored for a post, in computed order. Shows: "replay of the
analysis at 14:02", candidates moving as each score arrives, ending at
the stored ranking. Live reranking shows only as the progress row.
Still: instant reorder with arrows. Text: first-pass and reranked
positions. Test AT-VL138: only stored scores in stored order; labelled
a replay; the final order equals the stored ranking.

**39. Reconstruction quiz.** Data: link-reconstruction evaluation
records (specs 0057, 0024): a guest-visible post with a member link
removed, the guesses and the true target. Shows: the post without its
link, then the guesses, then the answer on request; never changes the
site. Test AT-VL139: post, guesses and answer equal one record; only
guest-visible posts; no site change.

**40. Model trial race** (first ten). Models screen, "View models". Data:
trial results and progress: the trial runs each sample link through both
models, two child handles advancing at once. Shows: both bars advance
through the same sample at their own pace, with recall at 1, 3 and 5
and p95 after each batch; then final figures side by side with the
estimated reindex time. Still: no transitions. Text: both models'
figures and progress. Test AT-VL140: both handles advance in the same
interval; figures equal the trial records after each batch; both use
the same sample ids.

**41. Simulation sliders.** Data: cached scored candidates and judged
results (spec 0057). Shows: moving the score threshold updates
suggestions per day and precision (with its range) at once, from the
cache only. It changes no live setting; the real threshold stays on the
settings screen. Test AT-VL141: values equal a recount over the cache;
the slider works by keyboard; no settings change is recorded.

### Operations items

**42. Request waterfall.** Operations, also opened from a suggestion and
from trace ids in items 52, 57 and 58. It is the built-in trace viewer.
Data: traces of suggestion requests, from the add-on to the scoring
step. The service and the add-on's spans (carried back in signed
replies) go to the bundled trace collector, which keeps every error and
slow trace and stores them in Sutradhar's database for 7 days. Nothing
leaves the PC unless the owner sets the optional trace export. Shows: a
trace as timed bars, one per step, errors named. Text: steps with
start, duration and parent. Test AT-VL142: a failing fixture request is
always kept and its bars equal the recorded steps from both sides; an
unsampled one says "not sampled"; after 7 fake-clock days "trace no
longer kept"; without the export no trace leaves the PC.

**43. Nightly timeline** (first ten). Data: the nightly run's staged
progress and last run's history (spec 0016). Shows: a bar per stage on
a time axis, filling live, and a replay of last night at a chosen
speed. Still: static; the replay steps. Text: stage, start, end,
duration, status, handled and total. Test AT-VL143: stage times equal
history; live bars equal the records; the replay uses recorded times.

**44. Resource gauges.** Data: processor and memory use of the systemd
slice and of the app window, each against its cap from AGENTS.md
section 5.6 for this PC's size, and disk use and free
space of the data folder's file system, each with its budget. Shows:
arcs with budget marks; the metrics store and trace collector are shown
within the slice. Still: no needle movement. Text, on a PC under 16 GB:
"memory 2.1 GB of the 4.5 GB slice; window 300 MB of 0.5 GB; disk
40 GB free". Test AT-VL144:
memory within 1% of the control group's figure; disk equals the file
system's; marks sit at the configured caps.

**45. Job graph.** Data: the job registry's dependencies and progress
records. Shows: jobs in dependency layers, each with a progress ring.
Text: jobs, what each depends on, and progress. Test AT-VL145: links
equal the registry; rings equal the records; an indeterminate job's
ring has no percentage.

**46. Retry wave.** Operations and over item 1. Data: dead-letter replay
events. Shows: replayed items flow back at the real rate, with
successes and repeat failures counted. Still: counters. Text:
"replaying 40 items: 32 succeeded, 2 failed again". Test AT-VL146: each
dot is a replay event; counts equal outcomes; no replay, no motion.

**47. Audit timeline.** Data: the audit log with diffs (spec 0032).
Shows: every change with who (system or moderator), when and a diff
preview; an erased record keeps its minimal event and says "details
erased". Test AT-VL147: entries equal the log; an erased record shows
"details erased" and no text.

**48. Promotion map.** Data: module state history (spec 0015). Shows: a
line per module through shadow, review and auto, the current station
lit; auto drawn locked, "not available in version 1". Text: each
module's state and change dates. Test AT-VL148: stations and dates
equal history; no module shows at auto.

**49. Setup systems check.** The install wizard. Data: install checks
(spec 0030), reported as steps through spec 0016. Shows: each check
lights when it really ends, passed or failed, in order; a failure shows
its reason and manual link. Checks include item 3's server-based job
runner and room in the systemd slice. Still: words change, no light.
Text: a checklist. Test AT-VL149: each light matches a recorded result;
none shows passed before its result; a failing check shows its reason.

**50. Doctor circuit.** Data: `sutra doctor` results (spec 0032) and the
component map. Shows: parts wired together; a failing check breaks the
wire at its part. Text: checks with part, status, explanation and code.
Test AT-VL150: the break sits at the failing part; all passing gives an
unbroken circuit; the text equals the report.

**51. Command palette** (first ten). Every screen. Data: search over
guest-visible thread titles, suggestions, jobs and settings, through
the action layer with the person's permissions. Shows: Ctrl+K opens it;
typing searches; arrows and Enter choose; only what the person may see.
Text: an accessible combobox with the count announced. Test AT-VL151:
it works by keyboard alone; a person without settings permission sees
no settings; p95 within 200 ms.

**52. Console.** Docked at the bottom of every screen. Data: structured
service logs with trace ids, scrubbed of post text. Shows: folded by
default; opened, live logs worded by level, filters for level, module
and trace, and a grouped mode using the feed rule. Each entry's trace
id opens item 42, or says "not sampled" or "no longer kept". It never
hides the focused element. Test AT-VL152: entries equal the log; while
folded no log data is fetched; focus under the dock scrolls into view.

### Ambient and notifications

**53. Window badge.** Data: the count of suggestions waiting for review
and readiness. Shows: the count and a status dot on the app's window
icon; the window title says the same in words, such as "(12) Review".
Test AT-VL153: badge and title equal the count; a readiness change
changes the dot and the title.

**54. Push notifications.** By ntfy (a push-message service) or email,
off until an address is set. Content: totals only, no thread titles.
Data: job end events (spec 0016) with a chart image drawn by the
service from totals. The service hands text and image to the add-on,
which sends them through the site's mail system or to the ntfy server
and token the owner enters. Shows: "nightly run finished: 1,204 new
suggestions, coverage 61% to 63%", the image and a link to the admin
page, which needs signing in. In email the image has a text
description. Test AT-VL154: only allow-listed fields and no titles,
even with hostile titles; nothing sent by default; a failed send is
retried and listed on the health screen.

**55. Ops wall.** A full-screen, read-only window using items 1, 3 and
58: headline figures, the heartbeat, the flow and the grouped feed. It
shows no member text, since others may see the screen. It opens with
its own read-only permission, created for one screen and revocable at
any time. Still: items 1 and 3's still forms. Test AT-VL155: no titles,
post text or names even when they exist; figures equal sources; a
revoked wall permission stops updates within a minute and opens nothing
else.

### Site-facing

**56. Transparency chips.** Public thread pages; off until reviewed.
Before it can be switched on, an agent that did not build it checks it
against a written privacy checklist and decides. Data: link provenance
(suggested and approved) and per-thread counts and lists of
guest-visible connections. The service sends each thread's figures to
the add-on, which stores them in its own table; public pages read only
that table with one indexed read and never wait on the service. Shows:
a small marker beside links Sutradhar inserted ("suggested by
Sutradhar, approved by a moderator"), drawn when the page is shown and
never stored in the post, so its words are untouched; and a "connected
to 12 discussions" chip whose pop-up lists the connected threads as
ordinary links, with a service-drawn mini-graph. Never post text,
scores, moderator names or visitor figures. Only guest-visible
connections count; one whose target stops being public leaves within
60 seconds of the change reaching the service. Text: the marker and chip
have text labels; the pop-up is a list. Test AT-VL156: stored post text
is unchanged; counts hold only guest-visible links and drop within 60
seconds; the page makes no call to the service and stays in budget; a
guest sees nothing beyond the allowed content; with chips off there is
no chip markup.

### Grouped lists

**57. Grouped error log.** Operations, errors section. Data: errors from
the service and the add-on's own errors (with codes, carried back in
signed replies), through the trace collector, stored in Sutradhar's
database. The site's general error log is not read. Shows: one row per
fingerprint with code, plain explanation, count, first and last seen,
affected jobs, latest trace id and manual link; a repeat raises the
count in place. The trace id opens item 42 for 7 days, then "trace no
longer kept", while the row stays for `errors.keep_days`. Test
AT-VL157: 1,000 errors with three fingerprints give three rows with
right counts and times; no post text or secret; a new fingerprint adds
a row; eight fake-clock days later the trace says "trace no longer
kept".

**58. Grouped live feed.** Live screen (item 2's grouped view, with a
switch between them), ops wall and console. Shows: repeats within 60
seconds as one row with count and latest time ("post saved, no
suggestions x14 in the last minute"); expanding lists the entries held,
each trace id opening item 42 when sampled. Test AT-VL158: fourteen
identical events in 60 seconds give one row marked x14; one outside the
window starts a new row; expanding lists fourteen.

### Methods for layouts and forecasts

All run in ganit, each after method admission (specs 0055, 0056).

- Explorer map (item 34): incremental PCA, fitted in batches on a
  sample; new threads placed with the fitted transform; refitted only
  after a model change and turned to match the previous map so points
  move little. UMAP was not chosen: it cannot be fitted in batches
  within the memory limit, and a nightly refit would move every point.
- Topic bubbles (item 36): mini-batch k-means on the reduced vectors,
  labelled with each cluster's distinctive title words.
- Forecast (item 30): a linear trend over the last eight weeks with a
  bootstrap range.
- Galaxy layout (item 6): our own build of the published ForceAtlas2
  method, run nightly from the previous positions so the picture stays
  stable.

### Failure behaviour

- Source down: the visual keeps its last data greyed, "as of 14:02;
  source unavailable", with the reason; old data is never shown as
  current.
- Live updates down: polling; if that fails too, a banner gives the
  last update time.
- A drawing fails to build: the text stays, a code is logged, and it is
  retried once.
- GPU drawing missing or lost: the text form, with one attempt to
  restore.
- Data over a visual's limit: it says so and offers the text form.
  Large sites are not grouped: the galaxy and explorer draw one per
  thread, in pieces, with the slow-drawing notice.
- A route slower than ten seconds: "still loading", then an error with
  a Retry button.
- Service short of memory: heavy requests, the playground first, are
  refused with a plain reason and code.
- A scheduled dataset whose job failed: the old dataset with its "as
  of" time and a link to the failed run.
- Item 56: on any failure the chip is left out with no message to
  members; the post shows normally; staff see the error in the log.
  Site pages always come first.
- Item 54: failed sends are retried with growing pauses and listed on
  the health screen.
- Times come from the service, so a wrong local clock does not mislead.

### Privacy and security

- Only guest-visible content enters Sutradhar. Every route also filters
  by current visibility when it serves, so a thread made private leaves
  every dataset as soon as the change reaches the service. Deletion and
  erasure purge or rebuild cached datasets, layouts and projections.
- Member text needs the matching permission: `visuals.view` for
  figures and `visuals.member_text` for text, checked with Cedar in the
  app and by the site's permissions on site pages. The ops wall never
  shows member text.
- Outgoing content (item 54 and item 29's image) is built through an
  allow-list of fields, so a new field cannot leak by accident.
  Notifications carry totals only; exported pictures may add titles of
  guest-visible threads; neither carries post text, member names or
  numbers, IP addresses or error details beyond codes.
- Analytics (items 16 and 17): only measured totals from spec 0034,
  following its consent, retention (detailed events 90 days, totals two
  years) and source rules. No visitor identifier reaches any visual.
  Daily counts below 10 show as "fewer than 10". Gaps are shown, never
  filled.
- The playground's typed text is used for one search and not stored,
  logged or counted.
- Logs pass the scrubber before storage. Spans carry only ids, counts
  and durations; error records add only codes and message templates;
  the collector's redaction is a second guard.
- Member text is always inserted as text, never as markup, including
  labels and tooltips, so hostile titles cannot run code.
- Screens and site pages make no request to any outside server.
- Manual screenshots come only from the synthetic fixture site.

### Settings

Section `visuals`: `enabled` true; `motion` (per person) system;
`live_updates` true; `feed.group_window_seconds` 60;
`feed.page_entries` 500; `feed.server_entries` 10,000;
`errors.keep_days` 90 after last seen; `galaxy.fresh_hours` 1;
`galaxy.fade_hours` 24; `playground.min_interval_seconds` 2;
`freshness_target_minutes` 10; `halo.tentative_band` 0.05 above the
minimum score; `analytics.min_count` 10; `chips.enabled` false;
`notify.channel` none (none, email or ntfy); `opswall.enabled` false;
`palette.shortcut` Ctrl+K. Trace sampling, the 7-day keep and the
optional export are monitoring settings.

## Limits

All inside the service slice and, for drawing, the window's cap, within
the memory limits in AGENTS.md section 5.6.

- Memory: visual caches about 32 MB on a PC under 16 GB and 128 MB on a
  larger one; one data request under 64 MB of working memory; at most
  one heavy dataset build at a time under 16 GB, two otherwise.
- Scheduled visual jobs (projection, galaxy layout, clusters, weekly and daily
  figures): at most 512 MB, one at a time, at low priority, in spec 0024's heavy
  job slot. They pause while the backlog run or a model rebuild needs memory,
  and read vectors in batches. Measured on the 50,000-thread tier and confirmed
  at a million posts before release.
- Storage: projection coordinates, layout positions, clusters and
  labels, weekly snapshots, daily reports, the error log and the feed
  buffer.
- Removal: visual datasets, clusters and labels are owned data: removal
  offers an export and asks keep or delete. Caches that only speed
  screens up are deleted without export: item 41's cache (rebuilt from
  suggestion and evaluation records) and item 56's prepared figures in
  the add-on's table (removed with the add-on's other tables).
- Data routes under 500 ms at p95 on the 50,000-thread tier; the
  palette under 200 ms.
- Data: at most 1 MB per first snapshot; GPU data in chunks of at most
  2 MB.
- Drawing: during live updates no screen task over 50 ms on Dell with
  the processor slowed to a quarter; animation stops while hidden.
- The playground: one search every two seconds per person.

## Footprint budget

- Item 56 on public thread pages: one indexed read of the add-on's own
  table, at most 5 ms added at p95 to page generation, at most 1 MB of
  added PHP peak memory (on against off); no call to the service; no
  page scripts; the pop-up's content at most 15 KB.
- With chips off, public pages get no chip markup and nothing added.
- The site's admin review page with visuals adds at most 2 MB of PHP
  peak memory, on against off.

## How it is tested

Each item starts from its failing test, named with the item above, run
on the synthetic fixture site. Rust tests cover data builders, SDS
components and the service; PHP tests cover the add-on's side; a
headless browser checks the site pages.

- Unit: each data builder, with property tests for invariants (shares
  between 0 and 1; treemap areas that sum to the whole); the grouping
  functions; the outgoing allow-list; the code that turns data into
  drawings and text equivalents.
- Integration on Dell: the service, a test site, the app; site pages in
  a headless browser on three engines; runs without GPU drawing;
  reduced motion; light and dark themes. No Safari support is claimed
  and no real-Safari check is made.
- Accessibility: automated checks on every screen and page; a keyboard
  script per interactive item; contrast and target-size checks; a
  recorded manual screen-reader pass.
- Privacy: a hostile fixture (markup and scripts in titles and posts);
  a crawler signed in as a guest and as a member; allow-list tests.
- Shared cases:
  - AT-VL01: all 58 items have a source, permission, text equivalent
    and empty state; a visual with no source fails to register.
  - AT-VL02: on an empty system every visual shows its empty state; a
    frame counter counts nothing for a minute; no animation runs; no
    figure but a true count appears.
  - AT-VL03: every dataset carries `source`, `as_of` and `item`, and
    the screen shows "as of".
  - AT-VL04: a static check finds no random numbers for data in visual
    code; the same data gives the same layout twice.
  - AT-VL05: with reduced motion (system or personal), the frame
    counter stays at zero and every state still shows.
  - AT-VL06: each table holds the same values as its dataset.
  - AT-VL07: no WCAG 2.2 AA failures in either theme; keyboard scripts
    pass; targets at least 24 by 24; nothing repeats three times a
    second; drawn parts meet 3:1.
  - AT-VL08: a screen builds only its always-visible items at first;
    heavy drawings are built only when they come into view or open.
  - AT-VL09: without GPU drawing, GPU views show text and raise no
    error dialog.
  - AT-VL10: no screen or site page makes a request to another server.
  - AT-VL11: as a guest or member, no route or page returns member text
    from this layer, except item 56's allowed fields when on.
  - AT-VL12: hostile titles and posts appear as plain text in every
    label, tooltip and table.
  - AT-VL13: a thread made private leaves every dataset within 60
    seconds of the change reaching the service, older scheduled ones
    included.
  - AT-VL14: with analytics or SEO signals absent, visuals say "Not
    available" with the reason, never zero; with the metrics store or
    collector stopped, their items keep last data greyed with "as of".
  - AT-VL15: live topics arrive on the one stream; pause stops them;
    when streaming fails, polling goes on and the banner shows the last
    update.
  - AT-VL16: data-size budgets are met.
  - AT-VL17: with fixture events replayed at 1,200 a minute (where item
    1 starts grouping dots), no screen task exceeds 50 ms on the slowed
    profile.
  - AT-VL18: routes meet their time budgets on the 50,000-thread tier
    and stay within memory; under memory pressure the playground is
    refused first.
  - AT-VL19 (PHP): added peak memory per admin page and per public
    thread page stays within budget, on against off.
  - AT-VL21: every error or feed list uses the shared grouping
    functions and groups identically.
  - AT-VL22: outgoing messages and exported images hold only
    allow-listed fields; notifications hold no titles; images hold no
    post text, names or titles of threads not guest-visible.
- Performance: route benchmarks and data builders under the 5% median
  rule.

## Changes from the source

- Admin-page visuals become Dioxus app screens built from SDS
  components; outside drawing libraries and their page-weight budgets
  are dropped.
- Site pages (the admin review page and item 56) get service-prepared
  markup, CSS and SVG with no hand-written page scripts; keyboard
  shortcuts and animated feedback (item 15) are in the app.
- The live channel into site pages is dropped; the app uses the local
  socket stream of spec 0016.
- Item 3's liveness event is recorded by the add-on and read by the
  service, since the site never calls the PC.
- The add-on's spans and errors (items 42 and 57) come back in signed
  replies instead of being sent to the PC.
- Item 44 reads the systemd slice and the window cap, not a container's
  control group; the metrics store and collector count inside the
  slice.
- Item 49 drops the passthrough, HTTP/2 and compose checks.
- Item 53 badges the app window, not a browser tab.
- Item 55 uses its own revocable read-only permission in the app.
- Layout, projection, clustering and forecast work run in ganit;
  item 54's chart image is drawn by the service.
- Outside crates pass cargo-deny, cargo-audit and cargo-vet; one that
  fails review holds back its item and a replacement goes to the owner.
- The register's five progress states (old AT-VL20) are dropped: status
  lives in Forgejo.
- The 20-times speed goal is replaced by the 5% median rule.

Source: `docs/specifications/platform/visuals.md`,
`docs/specifications/platform/visuals-2.md`,
`docs/specifications/platform/visuals-3.md`,
`docs/specifications/platform/visuals-4.md`,
`docs/specifications/platform/visuals-5.md`,
`docs/specifications/platform/visuals-6.md` and
`docs/specifications/platform/visuals-7.md` in the archive
`archive-2026-11-08-old-repo`.
