# Visual layer register

Owner: project owner. Requirement: R111 in the
[platform update](../requirements/updates-2026-10-30/platform.md); the
owner's exact list is in the
[platform requests](../requirements/sources/platform-requests-2026-10-30.md).

## Rules

- Every visual is built only on real system data: events, metrics,
  traces, the link graph, the index, the audit log or any other genuine
  system record (owner's follow-up answer); no decorative animation
  without a data source.
- Respect `prefers-reduced-motion`, give every visual a text equivalent
  and lazy-load WebGL views.
- Free libraries: Apache ECharts for charts, sigma.js for graphs, a WebGL
  scatter for the embedding explorer, D3 for the rest.
- Member text appears only on admin pages.
- Order of work: the first ten (marked "first ten" below, in the owner's
  order: pipeline flow diagram, live activity stream, link galaxy,
  before/after link morph, score breakdown, coverage treemap, embedding
  explorer, model trial arena, nightly timeline, command palette), then
  the rest in the order listed.

No product code exists yet; each visual needs the approved feature
specification and its data source before work starts.

## Live

1. Pipeline flow diagram (first ten). Data: event stream stage timings,
   per-stage throughput and latency metrics.
   Status: not started.
2. Live activity stream (first ten). Data: event stream (post saved,
   suggestions created, timing), filterable by forum and module.
   Status: not started.
3. Heartbeat. Data: events per minute from the add-on's event inbox.
   Status: not started.
4. Now-processing card. Data: the current job item from progress state
   and anchor candidates from extraction.
   Status: not started.
5. Forum treemap with blips. Data: XenForo node tree with thread counts;
   processing events.
   Status: not started.
6. Link galaxy (first ten). Data: the link graph (threads, inbound links)
   and accepted-link events.
   Status: not started.
7. Header sparkline. Data: p95 latency metric over the last hour.
   Status: not started.
8. Fluid queue gauges. Data: event inbox and dead-letter queue depths.
   Status: not started.
9. Fresh-edge glow. Data: link graph edges with creation times.
   Status: not started.

## Review page

10. Before/after morph (first ten). Data: the suggestion record (source
    text, anchor offsets, target).
    Status: not started.
11. Target preview on hover. Data: target thread title, first-post
    excerpt, reactions, age and inbound-link count.
    Status: not started.
12. Score breakdown (first ten). Data: per-method scores stored with each
    suggestion.
    Status: not started.
13. Confidence halo. Data: the suggestion's score.
    Status: not started.
14. Anchor heat map. Data: anchor candidates with scores and rejection
    reasons.
    Status: not started.
15. Accept/reject micro-animations. Data: review actions (audit log).
    Status: not started.
16. Impact counters. Data: approved links, measured clicks and page-view
    estimates.
    Status: not started.
17. Impact replay. Data: a thread's history of inbound links and traffic.
    Status: not started.
18. Shared-term highlighting. Data: term and concept matches between
    source and target from extraction and reranking inputs.
    Status: not started.
19. Bulk cards. Data: suggestions grouped by anchor and target.
    Status: not started.
20. Neighbourhood mini-graph. Data: link graph around source and target.
    Status: not started.
21. "Why no suggestions?". Data: candidate and rejection records per post.
    Status: not started.

## Dashboard

22. Coverage treemap (first ten). Data: inbound-link counts per thread,
    aggregated per forum.
    Status: not started.
23. Orphan radar. Data: orphaned threads per forum, weekly snapshots.
    Status: not started.
24. Time-travel slider. Data: link-graph history from the audit log.
    Status: not started.
25. Promotion rings. Data: module accept rates and promotion thresholds.
    Status: not started.
26. Health constellation. Data: health checks for the database, Redis,
    models, XenForo API and index.
    Status: not started.
27. Model cards. Data: model health rows and latency metrics (ADR-0002).
    Status: not started.
28. Index freshness clock. Data: index metadata (last embedding time
    against target).
    Status: not started.
29. Daily mission report. Data: daily metric aggregates.
    Status: not started.
30. Forecast bands. Data: coverage history and its forecast.
    Status: not started.
31. Signal wall. Data: SEO signal statuses.
    Status: not started.
32. Ghost lines. Data: the previous week's metric history.
    Status: not started.
33. Heat calendar. Data: links added per day from the audit log.
    Status: not started.

## Exploration and AI

34. Embedding explorer (first ten). Data: thread summary vectors (R102)
    projected to two dimensions, one point per thread (owner's follow-up
    answer). Recorder's note: new posts then move their thread's point.
    Status: not started.
35. Semantic playground. Data: a live query embedding against the index.
    Status: not started.
36. Topic bubbles. Data: clusters of thread vectors with labels.
    Status: not started.
37. Post X-ray. Data: one post's BBCode, extracted text, excluded zones,
    candidates and final links.
    Status: not started.
38. Reranker reorder. Data: reranking scores as they arrive.
    Status: not started.
39. Reconstruction quiz. Data: link-reconstruction evaluation records.
    Status: not started.
40. Model trial arena (first ten). Data: model trial results and progress
    (ADR-0003, ADR-0004).
    Status: not started.
41. Simulation sliders. Data: cached scored candidates and judged results.
    Status: not started.
42. Request waterfall. Data: OpenTelemetry traces.
    Status: not started.

## Operations

43. Nightly timeline (first ten). Data: staged progress and the previous
    run's history (ADR-0004).
    Status: not started.
44. Arc gauges. Data: CPU, memory and disk metrics with budgets.
    Status: not started.
45. Job graph. Data: job registry dependencies and progress.
    Status: not started.
46. Retry wave. Data: dead-letter replay events.
    Status: not started.
47. Audit timeline. Data: the audit log with diffs.
    Status: not started.
48. Promotion subway map. Data: module state history.
    Status: not started.
49. Setup systems check. Data: installer preflight check results.
    Status: not started.
50. Doctor circuit board. Data: `sutra doctor` results and component map.
    Status: not started.
51. Command palette (first ten). Data: search over threads, suggestions,
    jobs and settings.
    Status: not started.
52. Mission-control console. Data: structured logs with trace IDs.
    Status: not started.

## Ambient and notifications

53. Favicon badge. Data: pending suggestion count and readiness status.
    Status: not started.
54. Rich push notifications. Data: job completion events with chart
    images, through ntfy or email.
    Status: not started.
55. Ops-wall mode. Data: big-number metrics, the heartbeat and the
    pipeline flow.
    Status: not started.

## Forum-facing and trust

56. Transparency chips. Data: link provenance (suggested and approved) and
    per-thread link counts.
    Status: not started.

## Added by the owner's follow-up

Recorder's interpretation, for the owner to confirm: the answer "plus an
error log that is deduped, plus a live feed that is deduped" is recorded
as two new visuals.

57. Deduplicated error log. Data: the error log, with repeated errors
    shown once with their count.
    Status: not started.
58. Deduplicated live feed. Data: the live event feed, with repeated
    entries shown once with their count.
    Status: not started.

## Recorder's notes for the feature specification

- Items 25 and 48 show promotion toward automatic mode; v1 has no
  automatic insertion (R009), so the specification decides what they show.
- Item 58 may be the same as item 2 (the live activity stream) with
  deduplication; the specification settles this.
- Item 56 shows markers to forum members; it must keep R010 (no inserted
  prose) and R013, and needs a privacy review.
- Item 26 names Redis, which is not yet an admitted component.
- Item 54 sends to ntfy or email, external services needing approval.
- ECharts, sigma.js, D3, a WebGL scatter library, OpenTelemetry and the
  Prometheus client each need an admission record.
