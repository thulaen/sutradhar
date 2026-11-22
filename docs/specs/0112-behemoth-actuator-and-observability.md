# 0112: Behemoth Boot actuator and observability

This is group PB-H of Behemoth Boot (spec 0081). It owns how a Behemoth Boot
program shows what it is doing: the management endpoints (the "actuator"),
health probes, metrics, tracing, structured logs, log rotation, live log levels,
the start-up flight recorder, plain-English failure analysers and the report of
which parts switched on or off. Spec 0081 owns the ledger and clean-room rules,
spec 0106 the core (the conditions report and the start timeline originate
there), 0107 the web layer, 0109 security, and 0110 the scheduler and resilience
policies whose metrics appear here.

Sutradhar's product-level diagnostics are owned by spec 0032: the audit log,
alerts, `sutra doctor`, the Diagnostics screen and the `sutradhar_*` metric
names. This spec provides the framework pieces underneath: the health and
metric registries, the log pipeline, the analyser list. The doctor's checks use
health indicators from here; the product metrics of spec 0032 are recorded with
the registry from here.

ADR-0021 says Sutradhar has no telemetry by default. Nothing here sends data off
the PC unless the owner switches on a destination, with the owner's own account
or server. Management endpoints are reached only over the local doors (the
owner-only socket or pipe, or local MCP), never a network port.

Spring facts here come from the owner's pillar list (19 November 2026) and are
"unverified" until the scout confirms each against official notes (spec 0081).
The behaviour description, not the Spring wording, is the contract. Terms from
the supervisor and the lifecycle (state file, heartbeat, local doors, live
confirm, chain) are defined in specs 0030, 0106, 0020 and 0109. Mature crates
sit behind our starters for metrics, tracing and log writing (ADR-0022,
AGENTS.md section 6 rule 14).

Example: Mei's app will not start. Instead of a stack trace it prints: "The
app could not start. Another copy of Sutradhar is already using its socket.
Close that copy, or restart the PC. (SUT-1211)". Once running, she opens the
Engine Room and asks why the cache part is off; the conditions report says
"Off: the setting `cache.local.max_entries` is 0." She turns the `sync` part
to debug for 30 minutes; a note says it will return to normal on its own, and
the change is in the audit log. A readiness check answers "ready" only after
the start finished and every critical indicator (by default the database, once
it has been used or its first check passed) is up.

Values marked "proposed" are design choices to be checked by measurement.

## What it does

### Words used here

- **Actuator**: the set of management endpoints that report on a running
  program.
- **Liveness**: "is the process alive and not stuck?"
- **Readiness**: "can it take work right now?"
- **Health indicator**: one check that reports up, down, out of service or
  unknown.
- **Meter**: one measurement: a counter, a gauge, a timer or a histogram.
- **Series**: one meter with one set of tags.
- **Cardinality**: how many different tag values a meter has.
- **Trace**: the path of one request through the program.
- **Structured log**: a log line as data fields, not free text.
- **Analyser**: code that recognises a known error and explains it.
- **Starter, composition root**: defined in specs 0081 and 0106.

### What this spec designs

| Feature | What the user or program sees |
|---|---|
| N-1849 Complete Actuator management suite (names unverified) | Standard management endpoints, local only, with access gates |
| N-1786 Default liveness and readiness health probes (default unverified) | Separate "alive" and "ready" answers out of the box |
| N-1787 Clean-room Micrometer metrics engine | Counters, gauges, timers and histograms with tags, and exporters |
| N-1788 Native OpenTelemetry OTLP starter | Traces, metrics and logs sent to an OpenTelemetry collector the owner chooses |
| N-1789 Out-of-the-box structured JSON logging | Log formats chosen by one setting |
| N-1790 Flexible log file rotation engine (four strategies, unverified) | Rotation by size, time, both or a cron schedule, with compression |
| N-1791 Zero-restart runtime log-level tuner | Per-module levels changed live |
| N-1792 Startup timeline flight recorder | Milliseconds spent by every start-up step, as a flame graph |
| N-1793 Plain-English failure analysers | A calm explanation and the exact fix for common errors |
| N-1794 Live auto-configuration decision report | Every part on or off, with the reason |

### Management endpoints

1. The endpoints are: health, info, metrics, environment and settings (`env`,
   `configprops`), parts (`beans`), conditions, routes (`mappings`), scheduled
   tasks, caches, loggers and shutdown. Names follow the official reference
   (unverified). Heap dumps and thread dumps are not applicable to a Rust
   program and are marked so in the ledger with the owner's approved reason.
2. Reach: only the owner-only socket, the named pipe and local MCP (spec
   0020). An HTTP exposure on a network listener exists only in test builds,
   behind a Cargo feature that the Sutradhar build leaves out; a gate test
   checks that the product binary has none.
3. Exposure is opt-in per endpoint (`actuator.endpoints`). The default exposes
   health and info only. Each exposed endpoint passes the security chain and
   method rules of spec 0109.
4. Values in `env` and `configprops` are masked by default. A value marked
   secret is never shown, and no setting reveals it. Turning
   `actuator.mask_all` off only reveals non-secret values; secrets stay masked
   in every case.
5. Shutdown is off. When switched on it is a "needs confirm" action: it asks
   for the live confirm of spec 0020 before it stops anything. A local MCP
   caller cannot give the confirm, so with no person present the call is
   refused with SUT-1210.
6. Audit: state-changing calls and refused calls are audited (spec 0032)
   without secrets. Read-only calls are counted in a metric and not written
   one by one, so a poller cannot flood the audit log. Calls are rate limited
   per caller (`actuator.rate_per_min`).
7. Responses are size limited (`actuator.max_response_kib`). Large lists page.
   Health detail needs the permission `actuator.health.detail` of spec 0109.
8. The `conditions` endpoint and the Engine Room view show the report whose
   content and format spec 0106 owns (BB-06). This spec owns only the endpoint
   and the flight-record extension.

### Health

1. Two groups exist by default: liveness and readiness. Others may be added by
   name. The default is the owner's wording and unverified.
2. Liveness answers whether the process is running and its main task loop still
   moves. The app writes a monotonic heartbeat into the state file of spec 0106
   (once per `health.heartbeat_s`). The engine supervisor (spec 0030) reads the
   exit code and this state file, so a stale heartbeat is how a stuck program is
   found for a verified instance. The `health` endpoint shows the same data.
   Liveness never depends on a database or an outside service. An outage
   cannot restart a healthy program. Spec 0030 owns the restart checks.
3. Readiness is derived from the lifecycle states of spec 0106: `ready` only
   when start finished and not `draining`, and when every indicator named in
   `health.readiness_critical` (default: the database) is up.
4. An indicator has a timeout (`health.timeout_ms`) and returns up, down, out of
   service or unknown. A slow or failing indicator reports down or unknown; it
   never blocks the probe past its timeout. Indicators run in parallel, off
   the request path, and can be cancelled at the timeout. Their
   results are cached for `health.cache_ms`.
5. A group's status is the worst of its members in this order, worst first:
   down, out of service, unknown, up. This is our contract and is tested on a
   mix of unknown and up; Spring's own order is unverified and may differ.
   Details are shown only to callers with the permission above and never include
   secrets.
6. An unused lazy database pool (spec 0108) reports "not yet used". For
   readiness it counts as up until its first use. Reading an indicator
   does not open an unused pool. After a first-use failure for that
   pool is recorded, do not reuse its earlier unused result as current
   readiness, regardless of the cached result age. Apply the normal
   critical-indicator rule and keep liveness separate. See N-1756.
7. The doctor of spec 0032 uses these indicators for its checks.

### Metrics

1. A registry holds counters, gauges, timers and histograms with tags. Hot paths
   record with atomic operations and make no allocation; an allocation-counting
   test checks this (BH-20).
2. Tags use a typed API with a closed set of allowed values per tag (an enum
   or a fixed list), so a value from untrusted input (a URL, a user name)
   cannot be a tag. Cardinality is bounded by `metrics.max_series`. A new
   series over the limit is dropped and counted; the SUT-1204 note is logged
   once per meter per minute, not once per drop. The registry never grows
   without bound (spec 0078).
3. Histograms use fixed buckets (`metrics.histogram_buckets_ms`, default 1, 5,
   10, 50, 100, 500, 1,000, 5,000, 10,000); a percentile is the upper bound of
   its bucket, so its error is at most one bucket width. Timers record in
   nanoseconds and expose seconds.
4. Built-in meters cover the framework; each producing spec owns its meters
   and this spec owns the registry: web requests (spec 0107), the database
   pool (0108), caches, the outbox, scheduler and resilience policies (0110),
   batch jobs (0111), the runtime and memory. Names follow the convention of
   spec 0032 (`sutradhar_` prefix, snake case, units in the name).
5. The `metrics` endpoint returns the registry in Prometheus text form to a
   caller on a local door; it can be read by hand or through MCP. There is no
   pull listener in the product, because nothing local scrapes a socket.
6. Push exporters (InfluxDB, Graphite, Datadog) are starters, all off by
   default. Each goes only to a destination the owner set
   (`export.influx.url`, `export.graphite.addr`, `export.datadog.site`),
   validated by the rule below, and uses the outbound protection of spec 0107.
   Metric names, tags and values never carry site text or personal data.
7. Destination rule for every exporter and for OTLP: an address must use TLS
   (https, or the exporter's secure mode) unless it is loopback; a private-
   range address needs the owner's allowed-range entry (spec 0107). Otherwise
   SUT-1205.

### OpenTelemetry export

1. A starter sends traces, metrics and logs over OTLP to a collector address the
   owner sets (`otlp.endpoint`), for the signals listed in `otlp.signals`. It is
   off by default; with no address nothing is sent (ADR-0021).
2. Semantic conventions follow the published OpenTelemetry names (version
   unverified until the scout confirms it). `otlp.compression` (gzip or none),
   `otlp.headers` (secret values) and `otlp.tls_bundle` (spec 0109) are
   settings.
3. Sampling defaults to none (`otlp.sample_ratio` 0). When on, it is parent-
   based: a child span follows its parent's decision, and a root span is
   sampled by the ratio. Spans carry the trace context of spec 0110.
   Attributes are scrubbed: no site text, no personal data, no secret, no full
   URL with a query.
4. Export is buffered with a bound for each signal (`otlp.buffer_mib`). When the
   collector is down the oldest data of that signal is dropped, never blocking
   work or growing memory (spec 0078), and the drops are counted.

### Structured logging

1. Formats: plain text (default for the console), and JSON in the Elastic
   Common Schema, GELF and Logstash layouts, chosen by `log.format`. The
   layout names are unverified until the scout confirms them. A format crate
   does the writing.
2. Every line has a time, level, module, message, trace id and the SUT code if
   any. Fields are data, not joined text, and the logging API takes no
   pre-formatted string built from untrusted input.
3. Masking happens in the logging layer and reduces risk; it does not
   guarantee safety, so tests measure its misses. The primary defence is the
   secret type (spec 0065), whose value is replaced wherever it is logged. A
   configurable minimum list of key names (`log.mask_keys`, matched in any
   case and as a substring: password, passwd, token, secret, api_key,
   authorization, cookie, set-cookie, session) is masked wherever it appears.
   A value scrubber also masks URL user-info, bearer tokens and long
   hexadecimal or base64 strings. The fuzz test (FZ-BH-01) runs a fixed corpus
   and requires zero misses on it.
4. Injection: every value is encoded. In JSON formats this is JSON string
   encoding; in text, line breaks and control characters are escaped, so a value
   can never start a fake log line.
5. Logs hold no subject identifiers (a member id, a name or a snippet of site
   text), so they need no erasure target; the scrubber runs on log fields and a
   test checks it. Support bundles (spec 0032) include logs through the same
   masking layer.
6. Writing never blocks work. A bounded queue drops the lowest levels first when
   full and counts the drops (SUT-1206, noted once per minute).

### Log rotation

1. Four strategies, set by `log.rotate.strategy`: by size
   (`log.rotate.size_mib`), by time interval (`log.rotate.interval_h`), by
   size and time together, and by a cron schedule (`log.rotate.cron`, in the
   zone `log.rotate.tz`, with the cron rules of spec 0110). Intervals use
   monotonic time, so a clock change cannot cause a missed or repeated
   rotation. Old files are compressed with `log.rotate.compress` (gzip by
   default; the crate is a build-or-reuse item).
2. Rotation closes the current file and opens a new one through a single
   writer task, so it works on Windows, where an open file cannot be renamed,
   and no line in flight is lost: lines wait in the queue during the switch.
3. Old files are kept up to `log.rotate.keep` files and `log.total_cap_mib`
   total; the oldest are removed first. The cap is part of the disk cap of spec
   0078. The file ledger of spec 0030 records the log folder and a manifest, not
   every rotated file, so rotation cannot flood it.
4. Files are created owner-only: a permission mode on Linux and an access list
   on Windows. Both are tested.
5. When the disk is full, file writing stops, the console keeps working, an
   alert is raised (spec 0032) and the dropped lines are counted (SUT-1208).

### Runtime log levels

1. A module's level (error, warn, info, debug, trace) can be changed live
   through the loggers endpoint or local MCP, with no restart and no config
   edit. The configured level is `log.level` with optional per-module entries.
2. A change to debug or trace returns to the configured level after
   `log.level_revert_min` minutes, so a forgotten debug cannot fill the disk.
   The response says when. The timer does not survive a restart; a restart
   returns every module to its configured level.
3. Every change is audited with who and when (spec 0032) and needs the
   permission of spec 0109 (`actuator.loggers.write`). A refused change is
   SUT-1207. Levels never expose secrets: masking still applies.

### Start-up flight recorder

1. The timeline of spec 0106 is extended with the duration of every condition
   check, pool start and listener run, in milliseconds. The producer is our own
   start-up timeline code, which records spans; it is not a sampling profiler.
2. The record is a span tree folded by duration into the folded-stack text
   format and drawn as a flame graph by a mature crate, shown in the Engine
   Room. It is bounded in size (`flight.max_kib`, SUT-1209) and stored with
   the latest run only.
3. It stays on the PC and holds no secret or site text.

### Failure analysers

1. An analyser is a small typed unit: it recognises one error type and returns a
   plain explanation and the exact fix (a setting name, a button). It runs at
   start-up failure and on request.
2. The first set, each with its own code: socket or port in use (SUT-1211);
   missing or wrong setting (SUT-1212, from spec 0106's SUT-930); circular
   need (SUT-1213, from SUT-931); unreadable or expired certificate
   (SUT-1214); database not reachable (SUT-1215, from SUT-970); missing role
   or permission (SUT-1216); disk full (SUT-1217); corrupted state file
   (SUT-1218). The analyser code explains the underlying SUT code and does not
   replace it. Each has a manual page (spec 0032).
3. Messages are calm, in British English, say what happened and what to do, and
   never show a secret or a stack trace by default; the stack goes to the log.
   They live in the translation files (AGENTS.md section 8) and are checked
   against a short reading checklist (one idea per sentence, no jargon without a
   gloss, always a next step).
4. An error no analyser knows prints a generic message with its SUT code
   (SUT-1203) and says where the detail is.
5. Analysers are listed at the composition root; there is no scanning.

### Auto-configuration decision report

Spec 0106 owns the content and format of the conditions report. This spec owns
the `conditions` endpoint and the Engine Room view that show it, human
readable and as data.

### Reuse

Our own code is the health groups and aggregation, the metric registry's limits
and naming, the masking layer, the rotation caps, the log-level revert, the
analyser list and the endpoint gates. Mature crates sit behind them: a metrics
facade and exporters, an OpenTelemetry client, a log writer and format
encoders, compression, and a flame-graph renderer. Each choice is a
build-or-reuse record in the research dossier.

#### Local health, metrics and protected log files (N-1786–N-1790)

Use the current local owner route and protected observations.
Bind process or producing module, site where applicable, exact
profile, policy, time and viewer grant before exposing a result.
A metric, span, log line or framework exporter setting gives no
connection, recovery, report or release authority. The product
has no management network listener and no telemetry by default.
Use safe synthetic fixtures and the owning 0032 disclosure,
retention and erasure rules.

| Feature | Exact framework behaviour and product boundary | Existing requirements |
|---|---|---|
| N-1786 Default liveness and readiness health probes | Read 0106's exact instance lifecycle and advancing 0112 heartbeat. Keep readiness dependent on admitted critical indicators without making them liveness proof. Order mixed group severity down, out of service, unknown then up. Show details only with actuator.health.detail. Timeout stops the indicator and records SUT-1202. Only 0030 owns restart authority under QR-086. A readiness outage cannot cause restart. | BH-01, BH-02, BH-05, BH-06, BH-21, BH-34 · [FX-0112-W5-SEPARATE-LIVENESS-READINESS](../planning/feature-wave-5.md#acceptance-design-for-n-1767-to-n-1790) |
| N-1787 Clean-room Micrometer metrics engine | Use typed bounded counters, gauges, timers and histograms with defined units, bucket version and closed tag values. Keep recording within the admitted resource profile. Count refused excess series without allocating another series. Return only the permitted local projection. A format or exporter starter cannot open scrape or send access. | BH-01, BH-03, BH-07 to BH-09, BH-20 · [FX-0112-W5-BOUNDED-LOCAL-METRIC-SERIES](../planning/feature-wave-5.md#acceptance-design-for-n-1767-to-n-1790) |
| N-1788 Native OpenTelemetry OTLP starter | Keep off by default. A separately admitted owner destination and exact signal profile must pass protected transport, current scope, safe attributes and bounded batching. Require TLS except on the admitted loopback profile. Private range needs exact owner allow entry or SUT-1205. On saturation drop oldest only in the affected signal and count the drop. Jobs continue. A copied setting, parent trace or destination shape grants no send, remote management or disclosure of site text and secrets. An unadmitted export route stays unavailable. | BH-03, BH-09, BH-10, BH-25 · [FX-0112-W5-ADMITTED-OTLP-SIGNAL-SCOPE](../planning/feature-wave-5.md#acceptance-design-for-n-1767-to-n-1790) |
| N-1789 Out-of-the-box structured JSON logging | Encode typed fields for the admitted output profile with safe escaping and scrub before queue, disk or export. Keep secrets, site text, private identifiers and unapproved context out of the final bytes. Bounded writes never block work and drops are counted. A format label adds no exporter and a fixed corpus cannot prove universal masking. | BH-03, BH-11, BH-22, BH-24, BH-31 · [FX-0112-W5-SCRUBBED-BOUNDED-JSON-LOG](../planning/feature-wave-5.md#acceptance-design-for-n-1767-to-n-1790) |
| N-1790 Flexible log file rotation engine | Use one bounded writer. Admit size, interval, combined and cron profiles under current clock and respective file and total caps. Require Linux owner-only mode or equivalent Windows ACL. Prune old eligible sealed segments under both caps. Publish the 0030 owner-only folder manifest only for verified sealed segments. Reconcile interrupted rotation before exposure and apply current retention before pruning. No per-rotation ledger row. Preserve in-flight data within admitted queue limits, count explicit overflow and stop file writes only on full disk. Keep console, drop count and the safe alert. Only 0032 owns support bundle release. | BH-12, BH-23, BH-24, BH-31; 0030/0032 · [FX-0112-W5-LOG-ROTATION-VERIFIED-SEALED-GROUP](../planning/feature-wave-5.md#acceptance-design-for-n-1767-to-n-1790) |

QR-215 qualifies the exact default names and profile claims under
the existing documentation-only parity process. None of these
forms adds a native role, service, cluster, spend path or public
output.

### Scoped runtime observation and diagnosis (N-1791 to N-1794)

QR-216 qualifies these local observability forms independently. Bind
each to the admitted profile, site and explicit Cedar decision. Treat
condition reasons and startup records as bounded, translated output
from typed sources. Free-text errors, secrets and protected settings
values have no route into these views or their receipts.

| Feature | Owning behaviour and evidence |
|---|---|
| N-1791 | BH-13, BH-28 and BH-33. An actor with `actuator.loggers.write` may change one exposed target in the applicable site without restarting the service. Bind the timer to exact logger target, scope and override generation. Old expiry cannot clear a newer override. Debug and trace return only the current expiring override to its configured value. A site grant cannot change an install logger. A restart discards temporary overrides. Apply the exact spec 0020 setting and confirmation boundary where relevant. [FX-0112-W5-SCOPED-LOG-OVERRIDE-EXACT-GENERATION](../planning/feature-wave-5.md#acceptance-design-for-n-1791-to-n-1814). |
| N-1792 | BH-32 and UT-BH-13. Read the latest admitted local startup trace as ordered stages with parent spans and monotonic elapsed times from spec 0106, shown as a flame graph in the Engine Room. Enforce `flight.max_kib` and report SUT-1209 when the bound is hit. [FX-0112-W5-STARTUP-SPANS-MONOTONIC-BOUNDED](../planning/feature-wave-5.md#acceptance-design-for-n-1791-to-n-1814). |
| N-1793 | BH-15, BH-16 and BH-29. A typed, registered analyser maps a fault to a translated explanation and an allowed manual next step. Preserve the distinct underlying fault code. Give the generic safe message for an unknown fault. [FX-0112-W5-TYPED-FAILURE-SAFE-MANUAL-STEP](../planning/feature-wave-5.md#acceptance-design-for-n-1791-to-n-1814). |
| N-1794 | BH-17. The local Engine Room conditions view reports the recorded spec 0106 decision and its safe reason for the current admitted profile. A missing entry remains unknown. Reading the report leaves module state and settings intact. [FX-0112-W5-CONDITION-VIEW-RECORDED-SCOPE](../planning/feature-wave-5.md#acceptance-design-for-n-1791-to-n-1814). |

#### N-1849 Complete Actuator management suite

Use the existing per-endpoint opt-in and local owner-authenticated
management routes. Health and info keep their default exposure.
Mask protected fields, rate-limit and bound responses and require
current endpoint and method scope. Full management display grants
no refresh, export, logger change or shutdown by itself. Shutdown
retains 0020's live-confirmation policy and the existing MCP refusal.
Do not add a port, sink, dump or new remote control.
[N-1849](../planning/feature-wave-5.md#acceptance-design-for-n-1839-to-n-1862)
uses fixture `FX-0112-W5-MANAGEMENT-LOCAL-BOUNDED-READ-ONLY`.

### Data

- Metrics live in memory; nothing is stored unless an exporter is on.
- Log files: the home folder's log area, recorded in the file ledger as a folder
  with a manifest (spec 0030).
- The flight record and the conditions report: files with the latest run.
- Ledger and behaviour descriptions: repository files (spec 0081).

### Settings

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `actuator.endpoints` | list | `health`, `info` | the endpoint names | next start |
| `actuator.mask_all` | on or off | on | two values | next start |
| `actuator.rate_per_min` | whole number | 60 | 1 to 6,000 | next start |
| `actuator.max_response_kib` | KiB | 1,024 | 1 to 65,536 | next start |
| `actuator.shutdown_enabled` | on or off | off | two values | next start |
| `health.timeout_ms` | milliseconds | 1,000 | 50 to 10,000 (proposed) | next start |
| `health.cache_ms` | milliseconds | 1,000 | 0 to 60,000 | next start |
| `health.heartbeat_s` | seconds | 2 | 1 to 60 | next start |
| `health.readiness_critical` | list of indicators | `database` | any indicator names | next start |
| `metrics.max_series` | whole number | 10,000 | 100 to 1,000,000 | next start |
| `metrics.histogram_buckets_ms` | list | see Metrics | ascending numbers | next start |
| `export.influx.url`, `export.graphite.addr`, `export.datadog.site` | address | none | a valid address | next start |
| `otlp.endpoint` | address | none | a valid address | next start |
| `otlp.signals` | list | `traces`, `metrics`, `logs` | any subset | next start |
| `otlp.compression` | gzip or none | gzip | two values | next start |
| `otlp.headers` | secret reference | none | a secret id | next start |
| `otlp.tls_bundle` | bundle name | none | a bundle of spec 0109 | next start |
| `otlp.sample_ratio` | ratio | 0 | 0 to 1 | next start |
| `otlp.buffer_mib` | MiB | 8 | 1 to 256 | next start |
| `log.format` | text, json, ecs, gelf, logstash | text | five values | next start |
| `log.dir` | folder | log area of the home folder | a writable folder | next start |
| `log.level` | error, warn, info, debug, trace | info | five values | next start |
| `log.mask_keys` | list | the minimum list above | any superset | next start |
| `log.rotate.strategy` | size, time, size_time, cron | size | four values | next start |
| `log.rotate.interval_h` | hours | 24 | 1 to 8,760 | next start |
| `log.rotate.cron` | cron expression | none | a valid expression | next start |
| `log.rotate.tz` | time zone | local | a zone name | next start |
| `log.rotate.compress` | gzip, zstd, none | gzip | three values | next start |
| `log.rotate.size_mib` | MiB | 10 | 1 to 1,024 | next start |
| `log.rotate.keep` | whole number | 10 | 1 to 1,000 | next start |
| `log.total_cap_mib` | MiB | 200 | 10 to 100,000 | next start |
| `log.level_revert_min` | minutes | 30 | 1 to 1,440 | next change |
| `flight.max_kib` | KiB | 512 | 16 to 65,536 | next start |

There is no setting that turns off secret masking in logs, metrics and exports,
the cardinality limit, or the audit of endpoint calls.

### Failure codes

SUT-1200 endpoint refused by the chain; SUT-1201 endpoint not exposed;
SUT-1202 health indicator timed out; SUT-1203 start failed, analyser message
given; SUT-1204 metric series limit reached; SUT-1205 exporter destination not
allowed; SUT-1206 log queue dropped lines; SUT-1207 log level change refused;
SUT-1208 log cap reached, oldest files removed; SUT-1209 flight record over
its size limit; SUT-1210 shutdown needs the live confirm; SUT-1211 to SUT-1218
failure analyser codes (see Failure analysers). Codes SUT-1200 to SUT-1299 are
reserved for this spec.

### Security and privacy

- No secret, token, personal data or site text appears in a log, metric,
  trace, health detail, endpoint answer or export.
- Management endpoints are reachable only on the local doors and only through
  the security chain; nothing is exposed on a network port.
- No data leaves the PC without the owner's switch for that destination, using
  the owner's own account or server.
- Analyser messages never reveal more than the user needs to fix the problem.

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| BH-01 | Management endpoints MUST be reachable only over the local doors, each exposed endpoint MUST pass the security chain, and the product binary MUST contain no network listener for them. | IT-BH-01, PL-BH-01, UT-BH-16 |
| BH-02 | Endpoint exposure MUST be opt-in per endpoint, with health and info the only defaults. | UT-BH-01 |
| BH-03 | A secret value MUST never appear in `env`, `configprops`, logs, metrics, traces or exports. | UT-BH-02, FZ-BH-01 |
| BH-04 | Shutdown MUST be off by default and, when on, MUST need the live confirm (SUT-1210). | UT-BH-03, BD-BH-01 |
| BH-05 | Liveness MUST NOT depend on any outside service, MUST be shown by a heartbeat in the state file, and a stale heartbeat MUST be detectable; readiness MUST follow the lifecycle states of spec 0106 and the critical indicators. | UT-BH-04, IT-BH-02, IT-BH-05 |
| BH-06 | A slow or failing indicator MUST NOT block a probe past its timeout, and group status MUST be the worst member. | UT-BH-05, FI-BH-01 |
| BH-07 | Meters MUST respect the series limit, drop new series over it and count the drops. | UT-BH-06 |
| BH-08 | Tag values from untrusted input MUST be refused. | UT-BH-07 |
| BH-09 | Exporters MUST be off by default and MUST NOT send data without the owner's destination switch and the outbound protection of spec 0107. | UT-BH-08, IT-BH-03 |
| BH-10 | OTLP export MUST be buffered within its bound, MUST NOT block work, and spans and attributes MUST be scrubbed. | FI-BH-02, UT-BH-09 |
| BH-11 | Log lines MUST be masked in the logging layer and writing MUST NOT block work. | UT-BH-10, FI-BH-03 |
| BH-12 | Rotation MUST follow the four strategies, honour the total cap and lose no line in flight. | UT-BH-11, FI-BH-04 |
| BH-13 | A debug or trace level change MUST revert after its time limit and MUST be audited. | UT-BH-12 |
| BH-14 | Withdrawn: no requirement. The number is kept so later references stay stable. | none |
| BH-15 | Each analyser MUST give a plain explanation and the exact fix, and MUST NOT show a secret or a stack trace by default. | UT-BH-14, PL-BH-02 |
| BH-16 | An unknown error MUST print a generic message with its SUT code. | UT-BH-15 |
| BH-17 | The `conditions` endpoint MUST return the report of spec 0106 and the Engine Room MUST show it. | BD-BH-02 |
| BH-18 | Every endpoint call MUST be audited without secrets, rate limited and size limited. | IT-BH-04 |
| BH-19 | Each feature MUST have parity rows, behaviour descriptions and independent contract tests. | CT-BH-01 |
| BH-20 | Hot-path metric recording MUST make no allocation, and the series limit MUST log SUT-1204 at most once per meter per minute. | UT-BH-17 |
| BH-21 | Group health MUST be the worst member in the stated order, tested on an unknown and up mix. | UT-BH-05 |
| BH-22 | Log values MUST be encoded so no value can start a fake line, and the masking corpus MUST show zero misses. | UT-BH-10, FZ-BH-01 |
| BH-23 | Rotation MUST work on Windows by closing and reopening through one writer, use monotonic time, and create owner-only files on both systems. | UT-BH-11, IT-BH-06 |
| BH-24 | A full disk MUST stop file writing only, raise an alert and count dropped lines (SUT-1208). | FI-BH-04 |
| BH-25 | An exporter or OTLP destination MUST use TLS unless loopback, and a private address MUST need an allowed-range entry (SUT-1205). | UT-BH-18 |
| BH-26 | Read-only endpoint calls MUST be counted and not audited one by one, while state-changing and refused calls MUST be audited. | IT-BH-04 |
| BH-27 | Endpoint calls MUST obey the rate and response size settings. | UT-BH-19 |
| BH-28 | Log level changes MUST need `actuator.loggers.write`, MUST be audited, and a revert timer MUST NOT survive a restart. | UT-BH-12, IT-BH-07 |
| BH-29 | Each failure analyser MUST have its own code, an explanation and an exact fix, with the message text coming from the translation files. | UT-BH-14, UT-BH-20 |
| BH-30 | A shutdown call from MCP MUST be refused with SUT-1210. | UT-BH-03, BD-BH-01 |
| BH-31 | Log queue overflow MUST drop the lowest levels first, count them and note SUT-1206 at most once a minute. | FI-BH-03 |
| BH-32 | The flight recorder MUST be built from our start-up spans, MUST stay within `flight.max_kib` (SUT-1209) and MUST hold no secret. | UT-BH-13 |
| BH-33 | A refused log level change MUST return SUT-1207, and an unexposed endpoint MUST return SUT-1201. | UT-BH-21 |
| BH-34 | A health indicator timeout MUST be reported as SUT-1202. | UT-BH-05 |

## How it is tested

Designed, not run.

- **Unit (UT-BH):** endpoint exposure and masking tables; health aggregation
  order; series limits and tag refusal; log masking with a table of hostile
  keys and values; rotation arithmetic with one value just inside and one just
  outside each limit; level revert timing; analyser outputs for each fixture
  error.
- **Planted faults (PL-BH):** an endpoint exposed on a network listener in the
  product build; an analyser that prints a secret; each must be refused with its
  named message while an honest case passes.
- **Integration (IT-BH):** a real start with a deliberately occupied port;
  readiness through start, drain and stop; an exporter pointed at a blocked
  address.
- **Fault injection (FI-BH):** a hanging health indicator, an unreachable
  collector, a full disk during rotation, a log flood.
- **Fuzz (FZ-BH):** hostile strings in logs, tags and settings never leak a
  secret or break the format.
- **Behaviour (BD-BH):** the Mei scenario above, step by step.
- **Contract (CT-BH):** one test per parity item, written from the behaviour
  description by someone other than the implementer.
- **Measurement (MS-BH-01):** metric record cost per call, log lines per
  second and memory of the registry at the series limit on the 8 GiB
  reference.

## Changes from the source

The pillar list asked for a clean-room metrics engine with exporters for four
vendors, and OTLP as a first-class export. Behemoth Boot wraps mature crates
and keeps the limits, masking, analysers and gates as our own code (AGENTS.md
section 6 rule 14), and every export is off unless the owner switches it on
(ADR-0021). Heap and thread dumps are not applicable. The product-level audit,
alerts and doctor remain with spec 0032.
