# Owner decisions on SPEC-008 to SPEC-012, 30 October 2026, part 2

Status: historical source record, continued from
[part 1](specification-decisions-2026-10-30.md), in the same form.

## Round 7: SPEC-009 hardware, challengers and names

### SPEC-009 Q-MA02: hardware for the 30-second target

Asked: on what hardware the 30-second first suggestion is promised.
Offered: "8 cores, NVMe, 6 GB (Recommended)", "4 cores, SATA SSD"
(described as matching smaller hosts, with search alone possibly about
12 seconds, so the target unlikely), "Dell as it is".

```text
4 cores, SATA SSD
```

### SPEC-009 Q-MA03: when a challenger clearly wins

Asked: what makes a challenger clearly win. Offered: "At least +2
points (Recommended)", "Any clear improvement", "At least +5 points".

```text
At least +2 points (Recommended)
```

### SPEC-009 Q-MA04: testing the earlier choices

Asked: whether arctic-embed-s, jina-reranker-v1-tiny-en and
potion-base-8M are also tested. Offered: "Yes, those that load safely
(Recommended)", "No, named challengers only".

```text
Yes, those that load safely (Recommended)
```

### SPEC-009 Q-MA05: people's names as link text

Asked: which safeguards apply to people's names as anchors. Offered:
"Strict safeguards (Recommended)", "Any accepted destination", "Off by
default".

```text
Strict safeguards (Recommended)
```

## Round 8: SPEC-009 held suggestions, existing posts and proposals

### SPEC-009 Q-MA06: suggestions made before a model mismatch

Asked: the owner had chosen to block suggestions until the index is
rebuilt when it was built with another model revision; can suggestions
made before the mismatch still be approved meanwhile? Offered: "No,
hold them (Recommended)", "Yes, if consistent". The owner chose
"Other":

```text
Other. Never hold suggestions. Each suggestion records the model version and index version it was made with, and it is approvable as long as those two matched when it was made, which is always true by construction; approving a link does not depend on the current model. On a model change, keep serving and approving with the old model and index until the new index is complete, then switch atomically (blue/green), so there is no mismatch period at all.

Suggestions are produced and shown in real time, per post, never per batch. Processing order: build the thread-level index first (about 50k items, minutes), start generating suggestions as soon as it is complete, and build the per-post index in the background. Two queues: a live queue for new and edited posts, processed within seconds and always first; and a backfill queue for the historical catalogue, worked in the background ordered by thread views and recency, with each post's suggestions appearing the moment that post completes. Suggestions made before the index was complete record the coverage at the time and are re-scored once it is complete, replacing the pending suggestion if a clearly better target is found. The review page receives new suggestions live over server-sent events. Show queue depth, processing rate, and index coverage on the review page and the health page.
```

### SPEC-009 Q-MA07: which existing posts are scored

Asked: scoring every existing post would take years; which are
scored? Offered: "Linkable posts, best first (Recommended)", "Same,
recent years only", "Every eligible post".

```text
Linkable posts, best first (Recommended)
```

### SPEC-009 proposals P-MA01 to P-MA12

Asked: whether to accept SPEC-009's 12 technical proposals as a group.
Offered: "Accept all 12 (Recommended)", "Ask me some separately".

```text
Accept all 12 (Recommended)
```

### SPEC-011 D-PG01: the live route into XenForo's admin pages

Asked: which route live progress bars use. Offered: "Web server
passthrough (Recommended)", "PHP relays updates".

```text
Web server passthrough (Recommended)
```

## Round 9: reconciling the rebuild answers; SPEC-011 continues

### Rebuilds: MD-04 against Q-MA06

Asked: the MD-04 answer paused suggestions during a rebuild; the
Q-MA06 answer kept serving and approving with the old model and index
until the new index was complete, then switched in one step
(blue/green). Which should the specifications follow? Offered: "Keep
serving, blue/green (Recommended)", "Pause during rebuild".

```text
Pause during rebuild
```

### The thread-level index and R098

Asked: to keep R098's rule that bge-m3 reads both sides before a
suggestion reaches moderators, the thread-level index would be built
by bge-m3 from each thread's title and summary, by estimate about a
day or two for 50,000 threads on an ordinary processor rather than
minutes, with its suggestions re-scored once the per-post index is
complete; is that right? Offered: "Yes, bge-m3, approvable
(Recommended)", "Yes, but not yet approvable", "Use a faster model
first".

```text
Yes, bge-m3, approvable (Recommended)
```

### The order of the catch-up queue

Asked: the Q-MA06 answer orders the catch-up queue by thread views and
recency, where the drafts used nearest unlinked neighbours (SPEC-008)
and strongest matches (Q-MA07); which order applies? Offered: "Views
and recency (Recommended)", "Views and recency, then matches",
"Strongest matches first".

```text
Views and recency (Recommended)
```

### SPEC-011 D-PG02: "the sooner of 250 ms or 1%"

Asked: which reading of the update rate is right. Offered: "Whichever
comes first (Recommended)", "Only when both happen", "Every 250 ms
only".

```text
Whichever comes first (Recommended)
```

## Round 10: SPEC-011 monitoring, alerts, states and libraries

### SPEC-011 D-PG03: Prometheus gauges and monitoring off by default

Asked: the owner asked for Prometheus gauges, but R092 keeps product
instrumentation off by default; which wins? Offered: "Built, off by
default (Recommended)", "On, local only". The owner chose "Other":

```text
ship prometheus too, prometheus should also be used for progress and otel shipped too for traces and showing error logs on the gui
```

### SPEC-011 D-PG04: which finished tasks alert staff

Asked: which finished tasks send staff a XenForo alert. Offered: "Long
or failed (Recommended)", "Only failed or cancelled", "Every task".

```text
Long or failed (Recommended)
```

### SPEC-011 D-PG05: a "paused" state

Asked: whether "paused" is a task state of its own. Offered: "Yes, a
seventh state (Recommended)", "No, keep six".

```text
Yes, a seventh state (Recommended)
```

### SPEC-011 D-PG06: a library that fails its review

Asked: what happens if an outside library fails its licence and safety
review. Offered: "Block and ask me (Recommended)", "Agents pick a
replacement".

```text
Block and ask me (Recommended)
```

## Round 11: monitoring follow-ups, Safari and default settings

### Follow-up to D-PG03: on by default?

Asked: R092 keeps product instrumentation off by default; should the
shipped Prometheus and OpenTelemetry be switched on by default on
forums? Offered: "On by default, local only (Recommended)", "Shipped,
off by default". The owner chose "Other":

```text
shipped on by default. let them work.

----------

Plain-English status
Traffic-light status with one sentence: “Working normally”, “Working but slow: the database is taking three times longer than usual”, “Not working: can’t reach your forum’s API”. Built from latency histograms plus health checks.
“What it’s doing right now”: events per minute, suggestions per minute, queue depth, live. Counters and gauges.
“Is it keeping up?”: lag between a post being saved and its suggestions appearing, with time-to-catch-up when behind. Histogram of event age at processing.
A promise you can see: “New posts get suggestions within 12 seconds on average, 99% within 40”. Latency percentiles as a plain sentence.
Uptime in minutes: “Unavailable 7 minutes this month, of 43 allowed”. An error budget without the jargon.
Weekly digest notification: uptime, suggestions made and accepted, slowest thing, one recommended action.
Slowness, explained
Where the time went: when something is slow, the trace breakdown says “72% waiting for the database, 18% reranking, 10% the forum API”, per request type. Spans.
Top five slow queries, with the SQL and advice (“add an index on xf_post.last_edit_date”). Database spans.
Model performance card: latency, throughput, and memory per model. Spans around inference.
Cost of a suggestion: “0.8 CPU-seconds each; the reranker is 70% of it”, trending over time.
Built-in trace viewer: click a slow suggestion on the review page and see its timing waterfall. No Tempo needed; sampled spans in the database.
Canary request: a fixed suggestion request every minute, graphed, so a slowdown is caught even when the forum is quiet.
sutra why slow: a text analysis of the last hour’s traces, written so an owner can paste it into Claude or Codex and get a fix.
Diagnosis and fixes
Self-diagnosis playbooks: metric patterns mapped to causes and fixes with manual links (“Redis at 95% memory → raise maxmemory or lower cache lifetime”).
Crash-loop detection: “Restarted five times in ten minutes; last crash was out of memory; here’s the fix”. Restart counter plus the last exit reason.
Forum API health: “Your forum returned 429 (rate limited) 34 times today → raise the API limit or lower polling”. Error-rate counters by status code.
Redis and database health in words: hit rate, evictions, pool saturation, each with what to change.
Disk growth explained: “Logs grew 2 GB this week because the level is DEBUG; set it to INFO”. Per-directory size gauges.
Security signals: “300 requests with a bad signature from one IP today”, replay attempts, rate-limit hits.
Alerts that explain themselves: each carries the chart, the likely cause, the fix, and a snooze button, and alerts fire only for things the owner can act on.
Planning ahead
Capacity forecast: “At current growth, disk runs out in 9 weeks”. Trend on size gauges.
Right-sizing advice: “Peak memory was 1.1 GB; you can lower the limit to 1.5 GB”, or the reverse. Measured peaks against budgets.
Busy-hours heatmap and a suggested nightly window from request-rate history.
Nightly job report card: durations against previous nights, “embedding is getting 8% slower each week”.
Moderator workload: pending queue age, review time per suggestion, “the queue clears in about 3 hours at this pace”.
Changes and safety
“Did my change help?”: releases and config changes marked on every chart with before-and-after comparisons.
Pre-upgrade baseline: metrics captured automatically before an upgrade; regressions flagged after (“suggestion latency doubled since 1.4.0”).
Metric-driven rollback: a model activation that worsens latency or error rate past a threshold rolls back by itself.
Backpressure: memory pressure shrinks batch sizes and pauses backfill before anything crashes. Gauges driving behaviour, not only charts.
Queue fairness: backfill pauses automatically when live suggestion latency rises.
Quality and impact
Accept-rate drift alert: “Accept rate in Forum X fell from 92% to 71% this week”. Per-forum counters.
Impact numbers: “Links added by Sutradhar were clicked 1,204 times this week”. Click counters from the add-on.
Coverage progress: share of threads with inbound links, trending, with the forecast date for a target.
For support and power users
Support bundle with context: the last hour of metrics and traces around an error attached to sutra doctor --bundle, so a fix doesn’t need ten rounds of questions.
One-click Grafana: an export that generates dashboard JSON and data-source config for owners who have Grafana, plus the OTLP setting for sending traces elsewhere.
“What we collect” page: every metric and span attribute listed, with the test that proves no member data is in them.

Change the observability decision: metrics and tracing ship on by default. prometheus-client metrics and OpenTelemetry tracing are always on in the service; the default docker compose includes a Prometheus container with 15-day retention and a memory cap, and traces are stored sampled in Sutradhar's own database for 7 days with a built-in trace viewer, so no Grafana or Tempo is required. Keep /actuator/metrics and an optional OTLP export setting for owners with their own tools, plus a one-click Grafana dashboard export. Metrics and spans carry only IDs, counts, and durations, enforced by a test, and a "what we collect" page lists every metric and attribute. Build the owner-facing layer on top, in this order: the traffic-light status sentence, "what it's doing right now", keeping-up lag, where-the-time-went breakdown, self-diagnosis playbooks with manual links, alerts that explain themselves, the weekly digest, capacity forecast and right-sizing, pre-upgrade baselines with regression flags, metric-driven rollback and backpressure, accept-rate drift, impact numbers, the canary request, `sutra why slow`, and the support bundle with context. Record the full list of 36 in docs/dev/observability-features.md with the signal each one uses and a status, and record the decision change as an ADR, including the memory cost.
```

### Follow-up to D-PG03: what ships

Asked: what "ship Prometheus and OTel" means in practice. Offered:
"Built-in, no extra servers (Recommended)", "Bundle Prometheus and OTel
servers".

```text
Bundle Prometheus and OTel servers
```

### SPEC-011 D-PG07: checking real Safari

Asked: how Safari on Macs, iPhones and iPads is checked, since Dell
cannot run Safari. Offered: "Cloud Apple devices (Recommended)", "Lend
a Mac and iPhone", "No Safari claim yet". The owner chose "Other":

```text
skip it
```

### SPEC-011 D-PG08: default settings as a group

Asked: whether the default settings in SPEC-011 part 5 and SPEC-012
part 7 are approved as a group. Offered: "Yes, as listed
(Recommended)", "Yes, with changes".

```text
Yes, as listed (Recommended)
```
