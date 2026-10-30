# SPEC-012 part 5: items 31 to 45, Dashboard, Exploration, Operations

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in the
[specification decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Part of [SPEC-012](visuals.md). The item format is explained in
[part 3](visuals-3.md).

## Dashboard, continued

### 31. Signal wall

- Dashboard; HTML grid. Data: SEO signal statuses (F013, sanket,
  "signal").
- Shows: one tile per signal, coloured and worded by status. When a
  tile's value changes it pulses twice, slower than three times a
  second, then stays still with its new status.
- Still: the new status appears without pulsing, with "changed at
  14:02" written on the tile. Text: the tiles are text.
- AT-VL131: Tiles equal the signal records; before F013 exists the wall
  says "not available"; a change pulses only that tile, twice.

### 32. Ghost lines

- Every time-based chart; ECharts. Data: the same metric's history from
  seven days earlier.
- Shows: last week's line behind the current one, made to look faint by
  a dashed, thinner stroke rather than by low contrast, so it still
  meets 3:1; named in the legend and hideable.
- Still: static. Text: tables gain a "same time last week" column.
- AT-VL132: Ghost values equal the stored history seven days earlier;
  missing history leaves gaps; the legend names the line; its contrast
  is at least 3:1 in both themes.

### 33. Heat calendar

- Dashboard; ECharts calendar. Data: links added per day from the audit
  log.
- Shows: one square per day for the last year, coloured in steps with a
  numbered key.
- Still: static. Text: a table by month and day and the busiest day.
- AT-VL133: Daily counts equal the audit records; keyboard focus on a
  square reads its date and count.

## Exploration and AI

### 34. Embedding explorer (first ten)

- Explore page; the WebGL scatter library (P-VL05), loaded when opened.
  Data: thread summary vectors (R102), one point per thread (owner's
  follow-up answer), projected to two dimensions by a scheduled job
  (P-VL06) and coloured by forum. The job pauses during the backlog run
  and during model rebuilds.
- Shows: every thread as its own point. Choosing one lists its nearest
  neighbours, found from the stored vectors rather than from distances
  on the flat map. When new posts change a thread's summary vector its
  point moves; new threads drop in. On very large forums every thread
  is still drawn, loaded in pieces, with a notice that drawing may be
  slow on weaker computers (D-VL07).
- Still: points jump to new places. Text: a searchable thread list with
  forum and neighbours, and a cluster summary.
- AT-VL134: There is exactly one point per guest-visible thread with a
  summary vector; listed neighbours equal an exact search over the
  stored vectors; a fixture post that changes one summary vector moves
  only that point; without WebGL the list shows and the library is not
  downloaded; the job pauses when a model rebuild starts.

### 35. Semantic playground

- Explore page, with the explorer's map; the same WebGL library. Data:
  a live embedding of the typed sentence by the production embedding
  model (SPEC-008, SPEC-010), searched against the thread summary
  vectors only, not every piece.
- Shows: the nearest threads light up on the map and a ranked list
  appears. Searches run below the live lane (SPEC-009), so they never
  delay new posts' suggestions; at most one search every two seconds
  per person; the typed text is not stored or logged; refused with a
  reason when the service is short of memory (R023).
- Still: the list appears without animation. Text: the ranked list
  with scores.
- AT-VL135: Results equal an exact search over summary vectors for the
  query vector; the text appears in no log, store or metric; a second
  search within two seconds is delayed and then run, not silently
  dropped; a search during live-lane work waits for it.

### 36. Topic bubbles

- Explore page; D3 circle packing. Data: clusters of thread vectors
  with labels, made by a scheduled job (P-VL06).
- Shows: bubbles sized by thread count and labelled; clicking or Enter
  opens a cluster's threads.
- Still: no zoom animation. Text: a table of clusters with label, size
  and top threads.
- AT-VL136: Sizes equal cluster sizes; opening a cluster lists exactly
  its threads; labels equal the stored labelling output.

### 37. Post X-ray

- Post detail; layered HTML. Data: one post's BBCode, extracted text,
  excluded zones (quotes, code, spoilers, signatures), candidates and
  final links (F005, SPEC-008, SPEC-009).
- Shows: layers switched on and off by buttons, lined up at the same
  positions; only for guest-visible posts.
- Still: layers switch without fading. Text: each layer is text.
- AT-VL137: Excluded zones match the extractor's recorded spans; layer
  buttons report their pressed state; a non-public post shows "not
  eligible" and no content.

### 38. Reranker reorder

- Post detail and review page; D3. Data: the reranker scores SPEC-009
  stored for a chosen post, in the order they were computed.
- Shows: a replay, labelled "replay of the analysis at 14:02", in which
  candidates move into their new order as each stored score arrives.
  The final order equals the stored ranking. Live reranking of a post
  appears only as its progress row (SPEC-011).
- Still: instant reordering with arrows showing each change. Text: a
  table of first-pass and reranked positions.
- AT-VL138: The replay uses only stored scores in their stored order;
  it is labelled as a replay; after the last score the order equals
  the stored ranking.

### 39. Reconstruction quiz

- Explore page; HTML. Data: link-reconstruction evaluation records
  (F023, and trials in SPEC-010): a real guest-visible post with an
  existing member link removed, the system's guesses and the true
  target.
- Shows: the post without its link, then the guesses, then the reveal
  on request. It never changes the forum.
- Still and Text: the quiz is text.
- AT-VL139: Post, guesses and answer equal one evaluation record; only
  guest-visible posts are used; the page makes no forum change.

### 40. Model trial arena (first ten)

- Models page, for people with SPEC-010's "View models"; ECharts bars.
  Data: trial results and progress (ADR-0003, ADR-0004). SPEC-010's
  trial runs each sample link through both models, so its SPEC-011
  record has two child handles, one per model, advancing at the same
  time.
- Shows: a race: both models' bars advance together through the same
  sample, each with its own pace, recall at 1, 3 and 5 and p95
  latency, updated after each batch; then the final figures side by
  side with the estimated reindex time.
- Still: bars change without transitions. Text: a table of both models'
  current figures and progress.
- AT-VL140: Both child handles advance during the same interval and the
  bars show each one's own progress; figures equal the trial records
  after each batch; both runs use the same sample identifiers.

### 41. Simulation sliders

- Explore page; ECharts. Data: cached scored candidates and judged
  results (F023).
- Shows: moving the score threshold updates suggestions per day and
  precision at once, from the cache only, precision with its range.
  Nothing here changes a live setting; changing the real threshold
  stays on the settings page under R024 and R025.
- Still: no transitions. Text: a table for the chosen threshold.
- AT-VL141: Values equal a recomputation over the cache; the slider
  works by keyboard; no settings change is recorded.

### 42. Request waterfall

- Operations page, also opened from a suggestion on the review page and
  from trace IDs in items 52, 57 and 58; ECharts. This item is the
  built-in trace viewer of R127, so no Grafana or Tempo is needed.
- Data: OpenTelemetry traces of suggestion requests, from PHP to the
  Python scoring step. Tracing is on by default (R127, ADR-0006), which
  replaces P-VL09: PHP and the service send spans to the bundled
  OpenTelemetry collector on the same host, which samples them,
  keeping every error and every slow trace, and stores them in
  Sutradhar's own database for 7 days. Nothing leaves the host unless
  an administrator sets the optional OTLP export.
- Shows: a chosen trace as timed bars, one per step, errors named in
  words.
- Still: static. Text: a table of steps with start, duration and
  parent.
- AT-VL142: With the default settings, a fixture suggestion request
  that fails is always kept, and its bars equal the recorded steps from
  both PHP and Python; a request not sampled says "not sampled"; a
  trace older than 7 days on a fake clock says "trace no longer kept";
  without the OTLP export no trace connection leaves the host.

## Operations

### 43. Nightly timeline (first ten)

- Operations page; ECharts. Data: the nightly run's staged progress and
  the previous run's history (SPEC-011).
- Shows: one bar per stage on a time axis, filling with live progress,
  and a replay of last night's run from its recorded stage times at a
  chosen speed.
- Still: a static chart; the replay steps from stage to stage. Text: a
  table of stage, start, end, duration, status, done and total.
- AT-VL143: Stage times equal the history; live bars equal the SPEC-011
  records; the replay uses only recorded times.

### 44. Arc gauges

- Operations page; ECharts gauges. Data: processor and memory use of
  the service and all its workers, read from the operating system's
  control group for the service's container, whose memory cap the
  default docker compose file sets (R129; 6 GB, or 8 GB once Dell
  measurements confirm the estimate, R021 as amended and R121), and
  disk use and free space of the file system holding the service's
  data directory, each with its budget.
- Shows: three arcs with budget markers; the memory arc shows all
  workers together against the service's cap. The bundled Prometheus
  server and OpenTelemetry collector are shown beneath it against their
  own declared allowance, never inside the service's cap (R121).
- Still: no needle movement. Text: "memory 2.1 GB of the 6 GB cap;
  monitoring 410 MB of 640 MB; disk 40 GB free".
- AT-VL144: Memory equals the control group's figure within 1%; disk
  equals the data directory's file-system figures; the ceiling marker
  sits at the cap configured in the compose file; the monitoring
  containers' use is shown apart from the service's.

### 45. Job graph

- Operations page; D3 with a layered layout. Data: the job registry's
  dependencies and progress records (SPEC-011).
- Shows: jobs as nodes in dependency layers, each with a progress ring.
- Still: rings change without transitions. Text: a table of jobs, what
  each depends on, and progress.
- AT-VL145: Links equal the registry's dependencies; rings equal the
  progress records; an indeterminate job shows a ring without a
  percentage.
