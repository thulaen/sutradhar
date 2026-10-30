# Telemetry practices

Owner: the Git coordinator. Requirement: R128; decisions:
[ADR-0006](../adr/0006-observability.md) and
[ADR-0007](../adr/0007-service-levels.md). Status: standing rules
adopted by the owner on 30 October 2026. The owner's exact words, from
which each practice below is copied word for word and only rewrapped,
are in [part 4](../requirements/sources/specification-decisions-2026-10-30-4.md)
of the specification decisions (round 15).

Practices marked "Gate check" are the testable ones the owner asked to
enforce in the gate. None is enforced yet. Recorder's note: each check
is put to the owner as a question before it is switched on (R116). The
owner also asked for recording rules, multi-window burn-rate alerts on
the service levels, the collector's redaction and memory-limiter
processors, tail sampling that keeps all errors and slow traces, last-
success gauges for every job, queue age metrics and exemplars where the
stack supports them (R128).

## OpenTelemetry: what to trace

- TP-01: One trace per unit of work: an HTTP request, a consumed event, a job
  run, a CLI command. Background jobs get their own root span, not an orphaned
  one.
- TP-02: Auto-instrument the frameworks (HTTP server and client, database
  driver, Redis, message queue) so every external call is a span without
  hand-written code.
- TP-03: Add manual spans around business steps that matter (“extract text”,
  “find candidates”, “rerank”), so a trace reads like the pipeline, not like a
  stack of library calls.
- TP-04: Record errors properly: set span status to error, attach the
  exception as a span event with type, message, and stack, and never swallow
  it silently.
- TP-05: Follow the semantic conventions for attribute names
  (http.request.method, db.system, messaging.operation), so any backend
  understands your spans.
- TP-06: Put business identifiers on spans as attributes: site, module, model
  version, job name, post and thread IDs, never free text or personal data.
- TP-07: Propagate context across process boundaries with W3C traceparent,
  including from PHP into Python and into queued events, so one trace spans
  the whole system.
- TP-08: Use span links when work is triggered by many sources (a batch job
  processing 500 queued events links each one) rather than forcing a false
  parent-child relationship.
- TP-09: Use baggage sparingly for values every downstream span needs, such as
  the site ID, and never for secrets.
- TP-10: Add span events for milestones inside long spans (“checkpoint
  written”, “index swapped”) instead of creating hundreds of tiny spans.
- TP-11: Trace the schedulers and queues themselves: time waiting in queue is
  a span, so “slow” can be split into “slow to start” and “slow to run”.
- TP-12: Keep a span-count budget per trace; a loop that creates 10,000 spans
  is a bug.

## OpenTelemetry: how to run it

- TP-13: Configure the SDK only through the standard OTEL_* environment
  variables, so operators never learn a custom config format.
- TP-14: Use the batch span processor with explicit queue and batch sizes, and
  flush on shutdown, so the last spans before a crash or exit aren’t lost.
- TP-15: Sample deliberately: head sampling for volume, tail sampling to keep
  every error and every slow trace, and always 100% in development.
- TP-16: Set the service’s resource attributes: service name, version,
  environment, instance ID, and host, so traces from two installs never blur
  together.
- TP-17: Run an OpenTelemetry Collector in front of backends when there’s more
  than one consumer: receive once, process, export to many.
- TP-18: Put a redaction processor in the Collector that strips or hashes
  anything that looks like an email, phone number, token, or free-text body
  before export.
- TP-19: Put a memory limiter in the Collector so a traffic burst degrades
  telemetry, not the host.
- TP-20: Bridge logs to traces: every log line carries the trace and span ID,
  and the log backend links straight to the trace.
- TP-21: Use the OTel logs pipeline (or the bridge) so logs, traces, and
  metrics share one resource identity and one export path.
- TP-22: Export metrics with exemplars where supported, so a latency histogram
  bucket links to an example trace from that bucket.
- TP-23: Measure instrumentation overhead in CI (CPU and latency with
  telemetry on and off) and keep it under a stated budget, say 3%.
  Gate check: the overhead budget (ADR-0007: 3%).
- TP-24: Test the instrumentation: assert that a request produces the expected
  spans with the expected attributes, using the in-memory exporter.
  Gate check: span and attribute assertions.
- TP-25: Version your span and attribute names like an API: deprecate, don’t
  rename silently, because dashboards and alerts depend on them.

## Prometheus: designing metrics

- TP-26: Pick the right type: counters for things that only go up, gauges for
  current values, histograms for distributions. Avoid summaries, which can’t
  be aggregated.
- TP-27: Name metrics by convention: base unit in the name (_seconds, _bytes),
  _total on counters, one namespace prefix per service.
  Gate check: metric naming and unit conventions.
- TP-28: Write a HELP string for every metric, in plain words, because that
  text is the documentation.
  Gate check: HELP strings on every metric.
- TP-29: Keep label cardinality bounded: labels for things with a handful of
  values (status, module, job), never for IDs, user names, URLs with
  parameters, or free text.
- TP-30: Set a cardinality budget per metric and a test that fails when a
  label set grows past it.
  Gate check: cardinality budgets per metric.
- TP-31: Choose histogram buckets for the question being asked: latency
  buckets around the SLO, size buckets on powers of two; use native histograms
  where the stack supports them.
- TP-32: Cover the four golden signals for every service: latency, traffic,
  errors, saturation.
- TP-33: Apply the RED method to request-handling code (rate, errors,
  duration) and the USE method to resources (utilisation, saturation, errors).
- TP-34: Expose queue depth, oldest-item age, and processing rate for every
  queue; depth alone hides a stuck consumer.
- TP-35: Record the timestamp of the last success for every periodic job as a
  gauge; “time since last success” is the most useful job alert there is.
- TP-36: Export build and config info as a gauge set to 1 with version labels,
  so dashboards can show what’s running and when it changed.
- TP-37: Expose process and runtime metrics: CPU, memory, open file
  descriptors, garbage collection, thread count, and event-loop lag for async
  services.
- TP-38: Instrument caches with hits, misses, evictions, and size, not just a
  hit ratio, which can’t be aggregated.
- TP-39: Count every external dependency call by outcome and status code, per
  dependency, so a failing third party is visible immediately.
- TP-40: Measure work units, not only requests: items processed, bytes
  embedded, suggestions produced, so throughput is in business terms.
- TP-41: Handle multi-process servers correctly (multiprocess mode or a shared
  registry); otherwise each worker reports a different partial truth.
- TP-42: Push metrics from short-lived batch jobs through a Pushgateway or a
  textfile collector, since there’s nothing left to scrape when they finish.

## Prometheus: running it

- TP-43: Keep /metrics cheap and side-effect free; it will be scraped every 15
  seconds forever, and it must never touch the database.
  Gate check: side-effect-free /metrics check.
- TP-44: Protect the metrics endpoint: bind it to an internal interface or
  require auth, because metrics leak structure and volume.
- TP-45: Scrape at a fixed interval (15 or 30 seconds) and set retention
  deliberately, with remote write to long-term storage if you need more than
  weeks.
- TP-46: Alert on up == 0 and on scrape failures before anything else; if
  scraping is broken, every other alert is lying by silence.
- TP-47: Use recording rules for anything a dashboard computes repeatedly
  (per-module error ratios, p95s), so dashboards load fast and alerts share
  one definition.
- TP-48: Define SLOs and alert on burn rate with multiple windows (fast burn
  pages, slow burn tickets), not on raw thresholds that flap.
- TP-49: Make every alert actionable: a runbook link, a severity, and an owner
  in the annotations; an alert nobody acts on gets deleted.
  Gate check: runbook link, severity and owner on every alert rule.
- TP-50: Use inhibition and grouping so one root cause produces one
  notification, not forty.
- TP-51: Keep dashboards as code in the repository, generated or JSON,
  reviewed like code, with the metric names they depend on tested.
  Gate check: dashboard metric-name tests.
- TP-52: Annotate charts with deployments and config changes, so every
  regression has a candidate cause next to it.
- TP-53: Add blackbox or synthetic probes that exercise the system from
  outside, catching failures the inside view can’t see.
- TP-54: Watch Prometheus itself: its memory, series count, scrape duration,
  and rule evaluation time, because telemetry has capacity limits too.
- TP-55: Load-test the telemetry path: a traffic spike must not take down the
  service through its own metrics or spans.
- TP-56: Review the whole set quarterly: delete metrics nobody queries, split
  labels that grew, retire alerts that never fired or always fire.
