# Owner requests on models, progress and visuals, 30 October 2026

Status: historical source record. The owner gave these requests as free
text inside answers to the model follow-up questions (see the
[model decisions](model-decisions-2026-10-30.md)). They are kept exactly
inside the code blocks. The current requirements are in the
[platform update](../updates-2026-10-30/platform.md).

## Community models from Hugging Face

```text
Extend the model system with a point-and-click Hugging Face browser, keeping the pinned models in models.lock as the Verified tier.

Models page → Add model: search the Hugging Face Hub API (huggingface_hub HfApi) with filters for task (embedding, reranker), language, maximum size, and licence, sorted by downloads. Each result shows name, author, downloads, likes, size on disk, vector size, max input length, languages, licence, last updated, and a tier badge: Verified (in models.lock), Compatible (passes the checks), or Not supported (with the reason).

Compatibility check, run before any download, using model_info with file metadata and the config files: refuse models without safetensors weights; refuse models whose config requires custom code (auto_map or custom pipeline), since trust_remote_code is never enabled; require an architecture supported by sentence-transformers or fastembed; estimate memory from parameter count and refuse if it exceeds this server's model budget, and show an estimated CPU latency; compare vector size with the active index and show the full-rebuild time estimated from the forum's post count; prompt for a Hugging Face token for gated models; show the licence with an explicit accept step.

Download: resolve the repository's current commit at selection time, record repo id, commit, and per-file SHA-256 into the forum's local model lock (a per-site overlay on models.lock), verify every file against the Hub's LFS metadata during download, store under the data directory with restricted permissions, resumable with progress. From then on the model is pinned exactly like the defaults and never auto-updated.

After download: run the self-test; then offer a trial that runs link reconstruction on a sample of the forum's existing internal links with the candidate model against the active one, showing recall at 1, 3, and 5, anchor agreement, latency p95, and estimated full reindex time side by side. Activate builds the new index in the background while the old one serves, switches on completion, and keeps the previous model and index for one-click rollback. Every step writes to the audit log.

Health page: community models carry a "community, untested by Sutradhar" badge and the same status, latency, error, and index-match reporting as verified models. Settings: models.community.enabled (default true), models.community.allowed_authors (empty means any), models.community.max_size, and the memory budget. CLI equivalents: `sutra models search`, `sutra models check <repo>`, `sutra models add <repo>`, `sutra models trial <repo>`, `sutra models activate <repo>`, `sutra models rollback`. Test the compatibility checker against recorded Hub API fixtures covering: safetensors missing, custom code required, unsupported architecture, oversize, dimension mismatch, gated. The nightly verifies only the Verified tier. Record this as an ADR, including why trust_remote_code is never enabled.
```

## One progress system

```text
Add a single progress system and use it for everything that can take longer than two seconds.

Core API in sutradhar.karya.progress: a Progress handle with start(total=None, unit="items"|"bytes"|"steps"), advance(n), set_total(n), stage(name, index, count) for nested stages, fail_item(error), finish(), cancel(), and a context-manager form. State per operation: id, name, stage, done, total, failed, unit, started, updated, ETA, throughput, status (running, finishing, cancelling, done, failed, cancelled), and a link to the job run. ETA uses a moving average of the last 30 seconds of throughput, seeded from the previous completed run of the same job; when total is unknown, report indeterminate with elapsed time and throughput, never a made-up percentage. Updates are rate-limited to the sooner of 250 ms or 1%. Progress is persisted with the job checkpoint, so a resumed job continues from its previous position. Keep a history of completed runs with durations.

Views, all reading the same state: CLI through rich (nested bars, throughput, ETA, failed count), falling back to one plain line every 5% when stdout is not a TTY, so CI logs stay readable; the admin, review, and models pages with live bars updated by server-sent events (sse-starlette) with polling fallback, accessible with ARIA progressbar roles and a text equivalent; the jobs section of the health page listing every running operation with its bar and every recent run with its duration; `/actuator/jobs/{id}/progress` and `/actuator/jobs/running` in the API; a log line at every 5% with done, total, failed, and ETA; a Prometheus gauge per operation; and the completion notification including duration and failed count. Cancel buttons where cancellation is safe, with the bar showing "cancelling, finishing current batch".

Use it everywhere, with determinate totals wherever a count or byte size is knowable: model downloads (bytes), model verification, self-test, trials, index builds and rebuilds (posts), incremental embedding, the initial import, every batch job, link-graph rebuilds, database migrations (per migration), backups and restores (bytes), export and uninstall, eval runs (suggestions judged, batches done), the judge job (batches, with a paused state when a usage limit is hit), autoissues ingestion (per adapter), snapshot export, bulk actions on the review page, `sutra doctor` (checks), and the whole nightly and `just qa` as staged operations with an overall bar and a per-stage bar. Notebooks use marimo's progress bar fed from the same handle.

Enforcement: every registered karya job and every CLI command that can run longer than two seconds must create a Progress handle; add a test that runs each registered job on the fixture data and asserts it emitted start, at least one advance, and finish events, with done equal to total at the end. Test ETA behaviour with a fake clock: monotonic progress, indeterminate fallback, seeding from history, and resume from checkpoint. Make pytest itself show progress with pytest-sugar locally and plain output in CI. Record this as an ADR.
```

## Visual layer

```text
Live: “it’s alive”
★ Pipeline flow diagram: event → extract → candidates → rank → suggest, with particles flowing between stages at the real event rate, each stage showing live throughput and latency, and the slowest stage glowing amber.
★ Live activity stream: “Post 48,112 saved → 3 suggestions in 212 ms” entries sliding in as they happen, filterable by forum and module.
Heartbeat: a pulse animation whose rate is events per minute, flatlining visibly if the add-on stops sending.
Now-processing card: the post being processed right now, with anchor phrases highlighting in real time as they’re found.
Forum treemap with blips: the node tree sized by thread count, lighting up where posts are being processed.
★ Link galaxy: the link graph as a force-directed graph, threads as stars sized by inbound links, hubs bright, orphans dim, and each accepted link animating into place.
Header sparkline: p95 latency for the last hour, always visible.
Fluid queue gauges: event inbox and dead letters as liquid levels that rise and drain.
Fresh-edge glow: links added in the last hour glow in the galaxy, fading over the day.
Review page
★ Before/after morph: the post text with the chosen words morphing into a link when you hover Accept, so you see exactly what will change.
Target preview on hover: title, first-post excerpt, reactions, age, and inbound-link count of the target thread.
★ Score breakdown: a radar or stacked bar per suggestion (semantic, keyword, same forum, popularity, freshness, past accepts), so “why this one” is visible at a glance.
Confidence halo: glow intensity proportional to score; low-confidence suggestions look tentative.
Anchor heat map: the post with every candidate phrase shaded by score, including rejected ones, with the rejection reason on hover.
Accept/reject micro-animations: keyboard-first review where accept slides the link into the text with a green ripple and reject fades the card.
Impact counters: links approved, measured clicks on them, and estimated extra page views, counting up live.
Impact replay: a time-lapse of a thread gaining inbound links and traffic after linking, from its own history.
Shared-term highlighting: hover a word in the post and the matching concepts light up in the target excerpt, and vice versa.
Bulk cards: twenty suggestions of the same anchor → target collapsed into one card with a scrolling gallery of contexts.
Neighbourhood mini-graph: current thread, target, and their existing links as a tiny graph on each card.
“Why no suggestions?”: for any post, the candidates considered and the rule that rejected each, so silence is explained too.
Dashboard
★ Coverage treemap: forums coloured by the share of threads with at least one inbound link, animating as coverage improves.
Orphan radar: orphaned threads per forum on a radial chart that shrinks week by week.
Time-travel slider: scrub the link graph through its history and watch it grow.
Promotion rings: each module’s accept rate as a ring filling toward the promotion threshold, with the marker visible.
Health constellation: database, Redis, models, XenForo API, and index as connected nodes; a failure ripples red along its edges.
Model cards: live inference-latency sparklines and a gauge of p95 against budget.
Index freshness clock: a radial showing index age against target, “last embedded 3 minutes ago.”
Daily mission report: an auto-generated summary card with sparklines (links added, accept rate, top boosted threads), exportable as an image.
Forecast bands: projected coverage with a confidence band: “at this rate, 80% by March.”
Signal wall: the SEO signals as a grid of tiles coloured by status, pulsing when one changes.
Ghost lines: last week’s line drawn faintly behind every chart.
Heat calendar: links added per day, GitHub-contribution style.
Exploration and AI visuals
★ Embedding explorer: thread embeddings projected to 2D in WebGL, coloured by forum; click a thread to see its neighbours, and watch new posts drop in.
Semantic playground: type a sentence, and the nearest threads light up in the map while a ranked list animates in.
Topic bubbles: auto-labelled clusters sized by thread count, drillable to threads.
Post X-ray: toggleable layers over one post: raw BBCode, extracted text, excluded zones (quotes, code), candidates, final links.
Reranker reorder: candidates reshuffling with animated transitions as reranking scores arrive.
Reconstruction quiz: a real post with a member’s link removed, the system’s guesses, then the reveal; the best demo of the system there is.
★ Model trial arena: two models racing through the same sample with live recall and latency bars.
Simulation sliders: drag the score threshold and watch suggestions per day and precision update instantly from cached results.
Request waterfall: the trace of one suggestion request as a timing waterfall, straight from OpenTelemetry.
Operations
★ Nightly timeline: a Gantt of every stage with live progress, and a replay of last night’s run.
Arc gauges: CPU, memory, and disk with budget markers and the memory ceiling drawn in.
Job graph: jobs and their dependencies, each with a progress ring.
Retry wave: replaying dead letters shows them flowing back into the pipeline.
Audit timeline: every change with who (system or moderator), when, and a diff preview.
Promotion subway map: each module’s journey shadow → review → auto as a line with the current station lit.
Setup systems check: during install, each check lights up in sequence as it passes, cinematic but real.
Doctor circuit board: components wired together; a failing check visibly breaks the circuit at the right spot.
Command palette (Ctrl+K): jump to any thread, suggestion, job, or setting, with instant results.
Mission-control console: live structured logs with colour, filters, and trace-ID links, docked at the bottom of every admin page.
Ambient and notifications
Favicon badge: pending count and a status dot in the browser tab.
Rich push notifications: ntfy or email with a chart image attached (“nightly done: 1,204 new suggestions, coverage 61% → 63%”).
Ops-wall mode: a full-screen page of big numbers, the heartbeat, and the pipeline flow for a spare monitor.
Forum-facing and trust
Transparency chips: a subtle marker on inserted links (“suggested by Sutradhar, approved by a moderator”) and a “connected to 12 discussions” chip on threads with a mini-graph popover, so members see the system working too.

Add a visual layer to the admin surfaces, implemented only on top of real data from events, metrics, traces, the link graph, the index, and the audit log; no decorative animation without a data source. Respect prefers-reduced-motion, give every visual a text equivalent, lazy-load WebGL views, and use free libraries (Apache ECharts for charts, sigma.js for graphs, a WebGL scatter for the embedding explorer, D3 for the rest). Member text appears only on admin pages. Build these first: the pipeline flow diagram, the live activity stream, the link galaxy, the before/after link morph, the score breakdown, the coverage treemap, the embedding explorer, the model trial arena, the nightly timeline, and the command palette. Put the full list of 56 in docs/dev/visuals.md with a data source and a status for each, and work through the rest in that order after the first ten.
```
