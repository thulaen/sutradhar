# ADR-0006: Metrics and tracing on by default

Status: accepted by the owner, 30 October 2026. Requirements: R127,
R128, R121 and R129. The owner's exact words are in parts
[2](../requirements/sources/specification-decisions-2026-10-30-2.md)
(round 11) and
[4](../requirements/sources/specification-decisions-2026-10-30-4.md)
(round 15) of the specification decisions.

## Context

R092 kept product instrumentation off by default. SPEC-011 asked for
Prometheus gauges for progress and SPEC-012 for traces behind the slow
request view (item 42), both off by default. Asked which should win,
the owner chose to ship metrics and tracing on by default, with the
servers that collect them, and an owner-facing layer that explains in
plain words what the service is doing and why it is slow.

## Decision

- Prometheus metrics (through `prometheus-client`) and OpenTelemetry
  tracing are always on in the Python service. Prometheus is also used
  for progress (SPEC-011).
- The default docker compose file (R129) includes a Prometheus server
  with 15-day retention and a memory cap, and an OpenTelemetry
  collector with redaction and memory-limiter processors.
- The collector feeds sampled traces into Sutradhar's own database,
  kept for 7 days, with a built-in trace viewer, so Grafana and Tempo
  are not needed. Tail sampling keeps every error and every slow trace.
- `/actuator/metrics` stays; an optional OTLP export setting sends
  copies to an owner's own tools; a one-click export writes Grafana
  dashboard JSON and data-source settings.
- Error logs are shown on the admin pages, through the collector, as
  the owner asked ("showing error logs on the gui"; SPEC-012 item 57).
- Metrics and spans carry only IDs, counts and durations, enforced by a
  test. A "what we collect" page lists every metric and span attribute.
- The owner-facing layer is built on top, in the owner's order; the 36
  features, the signal each uses and its status are in
  [observability features](../dev/observability-features.md). The 56
  practices that govern the telemetry are in
  [telemetry practices](../dev/telemetry-practices.md).
- Service levels and the overhead budget are in
  [ADR-0007](0007-service-levels.md).

## Memory cost

By estimate, to be measured on Dell before release:

- Prometheus server: capped at 512 MB, for 15 days of the service's
  bounded label sets, scraped every 15 seconds.
- OpenTelemetry collector: capped at 128 MB; its memory limiter drops
  telemetry before the cap is reached, so a burst degrades telemetry,
  not the host.
- In the Python service: the metrics registry and the batch span queue,
  estimated at under 64 MB, inside the service's own cap (6 GB, or 8 GB
  once confirmed, R121).
- The trace store: 7 days of sampled spans in Sutradhar's database, on
  disk rather than in memory; its size is a storage row in SPEC-010.

The two bundled servers, about 640 MB together, have their own capped,
declared allowance on top of the service's cap, published in the
resource figures (R121). Each container's cap is set in the compose
file.

## Consequences

- R092's "product instrumentation stays off by default" no longer
  applies to metrics and tracing. SPEC-012's P-VL09 (tracing off by
  default) is replaced.
- Every metric name, label set, span name and attribute becomes part of
  the product's interface: renamed only by deprecation (practice 25).
- The gate gains telemetry checks (R128). Recorder's note: each is put
  to the owner as a question before it is switched on (R116).
- Forums with very little memory must still fit the declared allowance;
  the installer's preflight checks it.

## Recorder's notes

- The owner first chose "Bundle Prometheus and OTel servers", then,
  asked how the collector and the database store fit, answered
  "collector feeds our database". Both are recorded here.
- The figures above are the recorder's estimates, not measurements; the
  owner accepted measuring them on Dell.
