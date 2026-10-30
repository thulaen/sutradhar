# Owner decisions on SPEC-008 to SPEC-012, 30 October 2026, part 4

Status: historical source record, continued from
[part 3](specification-decisions-2026-10-30-3.md), in the same form.

## Round 15: monitoring memory, installation and traces

### The monitoring services' memory

Asked: the bundled Prometheus server and OTel collector need about
512 MB and 128 MB by estimate, to be measured on Dell; how does that
count against the memory limits? Offered: "Separate declared allowance
(Recommended)", "Inside the service limit".

```text
Separate declared allowance (Recommended)
```

### How the monitoring services are installed

Asked: the Python service was planned as an ordinary system service,
not Docker; how are Prometheus and the OTel collector installed?
Offered: "Compose for monitoring only (Recommended)", "Compose for
everything", "System services only". The chosen option was described
as: "The whole of Sutradhar, including the Python service, runs from
the default docker compose file."

```text
Compose for everything
```

### How the collector and the trace store fit

Asked: the owner chose a bundled OTel server and also traces stored,
sampled, in Sutradhar's own database for 7 days with a built-in
viewer; how do they fit? Offered: "Collector feeds our database
(Recommended)", "Service writes to database itself". The owner chose
"Other":

```text
collector feeds our database:

OpenTelemetry: what to trace
One trace per unit of work: an HTTP request, a consumed event, a job run, a CLI command. Background jobs get their own root span, not an orphaned one.
Auto-instrument the frameworks (HTTP server and client, database driver, Redis, message queue) so every external call is a span without hand-written code.
Add manual spans around business steps that matter (“extract text”, “find candidates”, “rerank”), so a trace reads like the pipeline, not like a stack of library calls.
Record errors properly: set span status to error, attach the exception as a span event with type, message, and stack, and never swallow it silently.
Follow the semantic conventions for attribute names (http.request.method, db.system, messaging.operation), so any backend understands your spans.
Put business identifiers on spans as attributes: site, module, model version, job name, post and thread IDs, never free text or personal data.
Propagate context across process boundaries with W3C traceparent, including from PHP into Python and into queued events, so one trace spans the whole system.
Use span links when work is triggered by many sources (a batch job processing 500 queued events links each one) rather than forcing a false parent-child relationship.
Use baggage sparingly for values every downstream span needs, such as the site ID, and never for secrets.
Add span events for milestones inside long spans (“checkpoint written”, “index swapped”) instead of creating hundreds of tiny spans.
Trace the schedulers and queues themselves: time waiting in queue is a span, so “slow” can be split into “slow to start” and “slow to run”.
Keep a span-count budget per trace; a loop that creates 10,000 spans is a bug.
OpenTelemetry: how to run it
Configure the SDK only through the standard OTEL_* environment variables, so operators never learn a custom config format.
Use the batch span processor with explicit queue and batch sizes, and flush on shutdown, so the last spans before a crash or exit aren’t lost.
Sample deliberately: head sampling for volume, tail sampling to keep every error and every slow trace, and always 100% in development.
Set the service’s resource attributes: service name, version, environment, instance ID, and host, so traces from two installs never blur together.
Run an OpenTelemetry Collector in front of backends when there’s more than one consumer: receive once, process, export to many.
Put a redaction processor in the Collector that strips or hashes anything that looks like an email, phone number, token, or free-text body before export.
Put a memory limiter in the Collector so a traffic burst degrades telemetry, not the host.
Bridge logs to traces: every log line carries the trace and span ID, and the log backend links straight to the trace.
Use the OTel logs pipeline (or the bridge) so logs, traces, and metrics share one resource identity and one export path.
Export metrics with exemplars where supported, so a latency histogram bucket links to an example trace from that bucket.
Measure instrumentation overhead in CI (CPU and latency with telemetry on and off) and keep it under a stated budget, say 3%.
Test the instrumentation: assert that a request produces the expected spans with the expected attributes, using the in-memory exporter.
Version your span and attribute names like an API: deprecate, don’t rename silently, because dashboards and alerts depend on them.
Prometheus: designing metrics
Pick the right type: counters for things that only go up, gauges for current values, histograms for distributions. Avoid summaries, which can’t be aggregated.
Name metrics by convention: base unit in the name (_seconds, _bytes), _total on counters, one namespace prefix per service.
Write a HELP string for every metric, in plain words, because that text is the documentation.
Keep label cardinality bounded: labels for things with a handful of values (status, module, job), never for IDs, user names, URLs with parameters, or free text.
Set a cardinality budget per metric and a test that fails when a label set grows past it.
Choose histogram buckets for the question being asked: latency buckets around the SLO, size buckets on powers of two; use native histograms where the stack supports them.
Cover the four golden signals for every service: latency, traffic, errors, saturation.
Apply the RED method to request-handling code (rate, errors, duration) and the USE method to resources (utilisation, saturation, errors).
Expose queue depth, oldest-item age, and processing rate for every queue; depth alone hides a stuck consumer.
Record the timestamp of the last success for every periodic job as a gauge; “time since last success” is the most useful job alert there is.
Export build and config info as a gauge set to 1 with version labels, so dashboards can show what’s running and when it changed.
Expose process and runtime metrics: CPU, memory, open file descriptors, garbage collection, thread count, and event-loop lag for async services.
Instrument caches with hits, misses, evictions, and size, not just a hit ratio, which can’t be aggregated.
Count every external dependency call by outcome and status code, per dependency, so a failing third party is visible immediately.
Measure work units, not only requests: items processed, bytes embedded, suggestions produced, so throughput is in business terms.
Handle multi-process servers correctly (multiprocess mode or a shared registry); otherwise each worker reports a different partial truth.
Push metrics from short-lived batch jobs through a Pushgateway or a textfile collector, since there’s nothing left to scrape when they finish.
Prometheus: running it
Keep /metrics cheap and side-effect free; it will be scraped every 15 seconds forever, and it must never touch the database.
Protect the metrics endpoint: bind it to an internal interface or require auth, because metrics leak structure and volume.
Scrape at a fixed interval (15 or 30 seconds) and set retention deliberately, with remote write to long-term storage if you need more than weeks.
Alert on up == 0 and on scrape failures before anything else; if scraping is broken, every other alert is lying by silence.
Use recording rules for anything a dashboard computes repeatedly (per-module error ratios, p95s), so dashboards load fast and alerts share one definition.
Define SLOs and alert on burn rate with multiple windows (fast burn pages, slow burn tickets), not on raw thresholds that flap.
Make every alert actionable: a runbook link, a severity, and an owner in the annotations; an alert nobody acts on gets deleted.
Use inhibition and grouping so one root cause produces one notification, not forty.
Keep dashboards as code in the repository, generated or JSON, reviewed like code, with the metric names they depend on tested.
Annotate charts with deployments and config changes, so every regression has a candidate cause next to it.
Add blackbox or synthetic probes that exercise the system from outside, catching failures the inside view can’t see.
Watch Prometheus itself: its memory, series count, scrape duration, and rule evaluation time, because telemetry has capacity limits too.
Load-test the telemetry path: a traffic spike must not take down the service through its own metrics or spans.
Review the whole set quarterly: delete metrics nobody queries, split labels that grew, retire alerts that never fired or always fire.

Adopt the 56 OpenTelemetry and Prometheus practices in docs/dev/telemetry-practices.md (I will paste the list) as standing rules. Enforce the testable ones in the gate: span and attribute assertions with the in-memory exporter, metric naming and unit conventions, HELP strings on every metric, cardinality budgets per metric, a side-effect-free /metrics check, the overhead budget, dashboard-metric-name tests, and a check that every alert rule has a runbook link, severity, and owner. Add recording rules, multi-window burn-rate alerts for the stated SLOs, the Collector redaction and memory-limiter processors, tail sampling that keeps all errors and slow traces, last-success gauges for every job, queue age metrics, and exemplars where the stack supports them. Record the SLOs and the overhead budget in an ADR.
```

## Round 16: service levels and the overhead budget

### Availability target

Asked: what availability target the Python service has. Offered:
"99.9% a month (Recommended)", "99.5% a month", "99% a month".

```text
99.9% a month (Recommended)
```

### Suggestion speed target

Asked: what target new posts' suggestions have, given the 30-second
rule on 4 cores with a SATA SSD and the example "within 12 seconds on
average, 99% within 40". Offered: "99% within 30 seconds
(Recommended)", "12 s average, 99% within 40 s", "95% within 30
seconds".

```text
99% within 30 seconds (Recommended)
```

### Monitoring overhead budget

Asked: what overhead budget monitoring has, measured in CI with
telemetry on and off. Offered: "3% CPU and latency (Recommended)",
"5%", "1%".

```text
3% CPU and latency (Recommended)
```

## Round 17: approval of the specifications

### Approval of SPEC-008 to SPEC-012

Asked, after the five specifications were revised to every answer and
independently reviewed: the owner had answered their decisions and
accepted their proposals but had not been asked to approve the
specifications themselves; approve all five for building? Offered:
"Approve all five (Recommended)" (described as approval for their
bounded implementation and tests, with the owner's answers; paid
embeddings still needing their own specification before any post
leaves the forum), "Approve some", "Not yet, I'll read first".

```text
Approve all five (Recommended)
```

### The recorder's readings

Asked: whether to accept five readings that fill gaps in the answers,
each marked as a reading in the specifications: (1) a rebuild for new
reading rules alone does not pause suggestions; (2) edited posts' times
are shown beside the 99% target but do not count towards it; (3) "you
check it" means an independent Claude agent carries out item 56's
privacy review and decides; (4) per-post analysis reports one running
total per job rather than a gauge per post, because the telemetry
practices forbid IDs in labels; (5) the catch-up queue alternates
reading and matching so the per-post index finishes in weeks, not
years. Offered: "Accept all five (Recommended)", "Change some".

```text
Accept all five (Recommended)
```
