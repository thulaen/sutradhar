# Observability features

Owner: the Git coordinator. Requirement: R127; decision:
[ADR-0006](../adr/0006-observability.md). The owner's exact words are in
[part 2](../requirements/sources/specification-decisions-2026-10-30-2.md)
of the specification decisions, round 11. Status of every feature: not
started. Nothing here is built.

Each entry gives the feature, what it shows, the signal it uses and,
where the owner set one, its place in the build order. Features without
a place are built after the ordered ones. Every figure in quotation
marks is the owner's example, not a measurement.

## Build order set by the owner

1. Traffic-light status (OF-01).
2. What it is doing right now (OF-02).
3. Keeping-up lag (OF-03).
4. Where the time went (OF-07).
5. Self-diagnosis playbooks (OF-14).
6. Alerts that explain themselves (OF-20).
7. Weekly digest (OF-06).
8. Capacity forecast (OF-21) and right-sizing (OF-22).
9. Pre-upgrade baselines with regression flags (OF-27).
10. Metric-driven rollback (OF-28) and backpressure (OF-29).
11. Accept-rate drift (OF-31).
12. Impact numbers (OF-32).
13. Canary request (OF-12).
14. `sutra why slow` (OF-13).
15. Support bundle with context (OF-34).

## Plain-English status

- OF-01 Traffic-light status with one sentence ("Working normally",
  "Working but slow", "Not working"). Signal: latency histograms and
  health checks. Order 1.
- OF-02 What it is doing right now: events, suggestions per minute and
  queue depth, live. Signal: counters and gauges. Order 2.
- OF-03 Is it keeping up: lag from a post being saved to its
  suggestions, with time to catch up. Signal: histogram of event age
  at processing. Order 3.
- OF-04 A promise you can see: latency percentiles as a sentence.
  Signal: latency histograms against ADR-0007's levels.
- OF-05 Uptime in minutes: downtime this month against the allowance.
  Signal: availability error budget (ADR-0007).
- OF-06 Weekly digest notification: uptime, suggestions made and
  accepted, slowest thing, one recommended action. Signal: weekly
  roll-up of the metrics above. Order 7.

## Slowness, explained

- OF-07 Where the time went, per request type. Signal: spans. Order 4.
- OF-08 Top five slow queries, with the SQL and advice. Signal:
  database spans.
- OF-09 Model performance card: latency, throughput and memory per
  model. Signal: spans around inference and process metrics.
- OF-10 Cost of a suggestion in CPU-seconds, by stage, over time.
  Signal: spans and CPU counters.
- OF-11 Built-in trace viewer: a slow suggestion's timing waterfall
  from the review page. Signal: sampled spans in Sutradhar's database.
- OF-12 Canary request: a fixed suggestion request every minute,
  graphed. Signal: synthetic probe. Order 13.
- OF-13 `sutra why slow`: a text analysis of the last hour's traces to
  paste into Claude or Codex. Signal: spans. Order 14.

## Diagnosis and fixes

- OF-14 Self-diagnosis playbooks: metric patterns mapped to causes and
  fixes, with manual links. Signal: metrics and alert rules. Order 5.
- OF-15 Crash-loop detection with the last exit reason. Signal: restart
  counter and last exit reason.
- OF-16 Forum API health by status code, with advice. Signal:
  error-rate counters by status code.
- OF-17 Redis and database health in words: hit rate, evictions, pool
  saturation. Signal: cache and pool metrics.
- OF-18 Disk growth explained. Signal: per-directory size gauges.
- OF-19 Security signals: bad signatures, replay attempts, rate-limit
  hits. Signal: security counters.
- OF-20 Alerts that explain themselves: chart, likely cause, fix and a
  snooze button, only for things the owner can act on. Signal: alert
  rules with annotations. Order 6.

## Planning ahead

- OF-21 Capacity forecast. Signal: trend on size gauges. Order 8.
- OF-22 Right-sizing advice. Signal: measured peaks against budgets.
  Order 8.
- OF-23 Busy-hours heatmap and a suggested nightly window. Signal:
  request-rate history.
- OF-24 Nightly job report card against previous nights. Signal: job
  duration metrics.
- OF-25 Moderator workload: pending queue age, review time, time to
  clear. Signal: review-queue gauges and counters.

## Changes and safety

- OF-26 Did my change help: releases and setting changes marked on
  every chart. Signal: build-info gauge and change annotations.
- OF-27 Pre-upgrade baseline with regressions flagged after. Signal:
  metrics captured before each upgrade. Order 9.
- OF-28 Metric-driven rollback of a model activation that worsens
  latency or errors past a threshold. Signal: latency and error
  metrics. Order 10.
- OF-29 Backpressure: memory pressure shrinks batches and pauses
  backfill. Signal: memory gauges driving behaviour. Order 10.
- OF-30 Queue fairness: backfill pauses when live latency rises.
  Signal: live-queue latency.

## Quality and impact

- OF-31 Accept-rate drift alert per forum. Signal: per-forum counters.
  Order 11.
- OF-32 Impact numbers: clicks on links Sutradhar added. Signal: click
  counters from the add-on. Order 12.
- OF-33 Coverage progress: share of threads with inbound links, with a
  forecast. Signal: coverage gauges.

## For support and power users

- OF-34 Support bundle with context: the last hour of metrics and
  traces around an error, attached to `sutra doctor --bundle`. Signal:
  metrics and spans. Order 15.
- OF-35 One-click Grafana export, plus the OTLP setting. Signal: the
  metric and span catalogue.
- OF-36 "What we collect" page, with the test that proves no member
  data is in them. Signal: the metric and span catalogue.

## Notes

- Click counters (OF-32) depend on analytics features that need their
  own approval and consent rules (R029–R033) before they collect
  anything. Accept rates (OF-31) come from the review page's own
  decisions (R060) and do not.
