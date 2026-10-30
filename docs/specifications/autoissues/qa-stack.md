# SPEC-005 QA stack

Status: APPROVED. Part of [SPEC-005](README.md).
Approved by the owner on 30 October 2026 (see the approval record in
`docs/specifications/approvals-2026-10-30.md`); it supersedes any
"awaiting approval" wording below.

## Host facts and limits

Measured on 30 October 2026: the Dell host has about 16 GB of memory and
no GPU, as the original request stated. It already runs other projects'
long-lived services. Sutradhar never uses, changes or shares them.

Its QA stack runs under the dedicated QA account with rootless Podman,
reading the same Compose file, because membership of the rootful Docker
group would give root-equivalent access to every key on the host
([trust boundaries](../trust-boundaries.md)). Agents never start
containers themselves; `just qa` and `just observe` submit a request that
the QA account carries out. Services listen only inside a private network
namespace owned by the QA account, and their volumes live in the
authorised Sutradhar live-data area. A read-only query proxy, run by the QA
account and requiring a credential, exposes only query endpoints (Grafana
views, PromQL, TraceQL, LogQL and Pyroscope queries). The gate and agents
use that proxy with separate credentials; nobody else can write to or
administer the services. Product code run during the soak writes telemetry
only through the collector, as it would in production.

The approved lab budget is 8 GB combined (R021, R092). Every QA process,
container and sandbox runs inside one systemd slice with `MemoryMax=8G`
(rootless Podman under the QA user's delegated slice; sandboxes through
`systemd-run --slice`). Each stage also has its own lower ceiling.
Qualification must prove the slice really contains container memory on this
host before the stack is relied on (AT-Q04). The budget is not raised
without the owner's approval.

## Profiles

`tools/qa/docker-compose.qa.yaml` keeps the requested name and defines
three profiles (AT-Q01); rootless Podman's Compose support must be
qualified for each profile before use:

- observe: `grafana/otel-lgtm` (Grafana, Tempo, Loki, Prometheus and the
  collector) plus a Grafana Pyroscope server;
- errors: GlitchTip web and worker with Postgres and Valkey;
- sonar: SonarQube Community Build with its database.

Images are pinned by digest after admission. The tool support record lists
which versions were verified.

## Sequence

`just qa` runs these stages strictly one after another, with ceilings
(proposed, to be confirmed by measurement):

| Stage | Work | Ceiling |
| --- | --- | --- |
| 1 | Tests, benchmarks, coverage, mutation; ingest | 4 GB |
| 2 | observe + errors profiles; soak; ingest; stop | 7 GB |
| 3 | sonar profile alone; analyse PHP and Python; stop | 6 GB |
| 4 | Security scanners; ingest | 3 GB |
| 5 | Evaluation adapters | 3 GB |
| 6 | Export snapshot to a new Dell directory | 1 GB |

Stage 2's soak runs locust against `/suggest` plus one full nightly job
with all instrumentation flowing, then ingests from GlitchTip, Prometheus,
Tempo, Pyroscope and Loki. Each stage waits for the previous stage's
containers to stop and for their memory to be released before starting.
Two heavy stages never overlap (AT-Q02, AT-Q03).

A stage failure stops the sequence, records an interrupted run and leaves
evidence on Dell. A stage whose target does not exist yet (no product
service for the soak, no evaluation lab) reports NO_TARGET_YET with the
inventory proof; it becomes mandatory when the target exists (AT-Q10).
NO_TARGET_YET is never a pass for a stage whose target exists.

## Instrumentation

No product code exists yet, so nothing is "already installed". When the
Python service is built, OpenTelemetry, prometheus-client, sentry-sdk and
pyroscope-io are each admitted first. They are off by default. The `qa`
settings profile turns on OTLP export to the collector, Prometheus scraping
of `/actuator/metrics`, sentry-sdk pointed at the local GlitchTip with
`traces_sample_rate` 1.0, continuous profiling and JSON logs shipped to
Loki (AT-Q05). A test proves that the default and production profiles send
nothing and open no exporter connections (AT-Q06). PHP-side instrumentation
needs its own admission and the 8 MB request budget measurement.

## Dashboards

Prebuilt Grafana dashboards live in `tools/qa/grafana` and alert rules in
`tools/qa/prometheus`. A provisioning test starts the observe profile and
confirms every dashboard and rule loads without errors (AT-Q07).

## Interactive

`just observe` starts only the observe profile and prints the read-only
proxy address for Grafana (AT-Q08). It uses the same slice and ceilings.

## Observability guide

`docs/dev/observability.md` documents every local endpoint with example
PromQL, TraceQL, LogQL and Pyroscope queries so agents can query them
directly. A documentation test runs each example against the fixture stack
(AT-Q09).

## Privacy

QA runs that use private Dell datasets keep traces, logs and profiles in
Sutradhar's Dell storage. Span attributes and log fields must not contain
post text or member data; a scrubber test covers the exporters. Nothing
from these services is published without the privacy gate.
