# ADR-0007: Service levels and the telemetry overhead budget

Status: accepted by the owner, 30 October 2026. Requirements: R124 and
R128. The owner's exact words are in
[part 4](../requirements/sources/specification-decisions-2026-10-30-4.md)
of the specification decisions (rounds 15 and 16).

## Context

The owner asked for multi-window burn-rate alerts on stated service
levels and a measured overhead budget for telemetry, recorded in an
ADR. The owner's earlier rule (R015) promises a first useful
suggestion within 30 seconds; R124 sets the hardware it is promised on.

## Decision

Service levels for each forum's Python service, measured by its own
Prometheus metrics:

- Availability: 99.9% a month, about 43 minutes of unavailability
  allowed. Unavailable means the service's health check fails or it
  cannot accept new posts.
- Suggestion speed: 99% of new posts get their first useful suggestion
  within 30 seconds of being saved, on the stated minimum of 4
  processor cores and a SATA solid-state disk (R124). Edited posts take
  the same live queue; their times are measured and shown beside this
  level but do not count towards it.

Overhead budget:

- Telemetry (metrics and tracing) costs under 3% of CPU and under 3% of
  latency, measured in CI by running the same workload with telemetry
  on and off. A larger difference fails the check.

Alerting:

- Each service level has burn-rate alerts over two windows: a fast
  burn notifies at once; a slow burn becomes an issue to fix.
- Every alert rule carries a runbook link, a severity and an owner, and
  alerts are grouped so one cause gives one notification.

## Consequences

- The latency histogram's buckets are set around 30 seconds.
- A community model the administrator activates against a slower trial
  (R125) puts that forum outside these levels; its health page says so
  and its alerts use its own trial figures.
- Paid embeddings (R126) add an outside dependency; its calls are
  counted by outcome, and its outages count against availability only
  when the service cannot accept new posts.
- Posts saved while new suggestions pause for a model rebuild (R122)
  are reported separately and do not count towards the speed level,
  since the owner chose the pause knowing they get no suggestions.
- The service levels are promises to be met by measurement on Dell
  before release, not figures already measured.

## Recorder's notes

- "Unavailable", the separate reporting of edited posts and of posts
  saved during a rebuild pause, and the treatment of community models
  and paid embeddings above are the
  recorder's reading, for the owner to correct. The owner's answer
  named new posts only.
- The owner's example, "within 12 seconds on average, 99% within 40",
  was offered as an alternative and not chosen.
