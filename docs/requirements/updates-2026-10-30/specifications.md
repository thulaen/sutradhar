# Requirements update: decisions on SPEC-008 to SPEC-012

Owner: project owner. Recorded by the Claude coordinator from the
owner's answers of 30 October 2026, given through the question form in
a session on Dell. The owner's exact words are in the
[specification decisions](../sources/specification-decisions-2026-10-30.md)
(parts 1 to 4). Status: owner requirement decisions. Answers that only
choose between a specification's own options are recorded in that
specification; this page records the answers that change or add a
requirement. Recorder's notes, marked as such, are interpretations.

## R120: model file formats

- bge-m3 is used through its publishers' official ONNX export at full
  precision, run by ONNX Runtime, not through its pickle weights
  (SPEC-010 MD-01). R106's rule that pickle is never loaded stays word
  for word; this export joins the formats R106 lists.
- spaCy's `en_core_web_lg` replaces `en_core_web_trf`, whose
  transformer weights are an embedded pickle (SPEC-010 MD-02). It runs
  without PyTorch.

Recorder's note: ONNX Runtime and `en_core_web_lg` still need admission
records (R027). SPEC-009's comparison and entity fixtures use the new
forms.

## R121: memory

- The Python service's cap is raised from 6 GB to 8 GB if Dell
  measurements confirm the estimate that everyday use does not fit 6 GB
  (SPEC-010 MD-03). Until they do, 6 GB stands.
- The bundled monitoring services of R127 have their own capped,
  declared allowance, published in the resource figures, on top of the
  service's cap; by estimate about 512 MB for Prometheus and 128 MB for
  the OpenTelemetry collector, to be measured on Dell.

## R122: model updates and suggestion records

- Every suggestion records the model version and the index version it
  was made with. Suggestions are never held: a suggestion is
  approvable when those two matched when it was made, which is true by
  construction, and approving a link does not depend on the current
  model.
- When the embedding model changes, new suggestions pause while the
  new model rebuilds the index on its own; the switch to the new index
  is made in one step when it is complete, so there is no mismatch
  period (SPEC-010 MD-04, as reconciled with SPEC-009 Q-MA06).

Recorder's note: the owner first chose "take turns, keep suggesting"
for MD-04, then "pause suggestions during rebuild", then, in Q-MA06,
keeping the old model serving until the switch. Asked which wins, the
owner chose the pause. The rest of the Q-MA06 answer stands:
suggestions already made stay approvable during the rebuild.

## R123: suggestions in real time

- Suggestions are produced and shown in real time, per post, never per
  batch.
- The thread-level index is built first, by bge-m3 from each thread's
  title and summary, so that R098's rule that bge-m3 reads both sides
  before a suggestion reaches moderators holds. Suggestions start as
  soon as it is complete; the per-post index is built in the
  background.
- Two queues: a live queue for new and edited posts, processed within
  seconds and always first; and a backfill queue for the historical
  catalogue, ordered by thread views and recency, with each post's
  suggestions appearing the moment that post completes.
- Suggestions made before the per-post index is complete are
  approvable, record the index coverage at the time, and are re-scored
  once it is complete, replacing the pending suggestion if a clearly
  better target is found.
- The review page receives new suggestions live over server-sent
  events. Queue depth, processing rate and index coverage are shown on
  the review page and the health page.

Recorder's notes: the owner estimated the thread-level index at "about
50k items, minutes"; with bge-m3 on an ordinary processor the estimate
is about a day or two, which the owner accepted. SPEC-008's reading of
"the embedder has read the destination" as "all of it" is replaced by
this rule.

## R124: the 30-second target

- The 30-second first useful suggestion (R015) is promised on a stated
  minimum of 4 processor cores and a SATA solid-state disk (SPEC-009
  Q-MA02).
- Its service level is 99% of new posts within 30 seconds, from the
  post being saved to the first useful suggestion appearing
  ([ADR-0007](../../adr/0007-service-levels.md)).

Recorder's note: the owner chose this hardware knowing search alone
may take about 12 seconds there, so the target is at risk; SPEC-009
must show how it is met or report honestly that it is not.

## R125: community models outside the defaults

- An administrator may activate a community model whose trial
  projects a first suggestion slower than 30 seconds, after a warning
  and confirmation recorded in the audit log; that forum then runs
  outside the qualified target and the health page says so. This
  waives R015 for that forum's choice only (SPEC-010 MD-05).
- A community model that states no licence may be used after a
  stronger warning that the administrator accepts (SPEC-010 MD-10).

## R126: paid embeddings

Paid embedding models from Google and OpenAI are allowed as an
optional choice, as the option the owner chose described it: off by
default; an administrator may choose one
instead of bge-m3 for the whole index, entering their own key, with a
spending cap; only guest-visible text is sent; switching back to
bge-m3 means a rebuild (SPEC-008 Q-RD01 and its follow-ups). A separate
specification, drafted next, covers providers, consent, privacy, costs
and keys, for the owner's approval.

Recorder's note: vectors from different models cannot share an index,
so a paid model cannot shorten bge-m3's first run; it replaces bge-m3
for that forum.

## R127: monitoring on by default

Metrics and tracing ship on by default, and error logs are shown on
the admin pages, as recorded in
[ADR-0006](../../adr/0006-observability.md): Prometheus metrics and
OpenTelemetry tracing always on in the service; a bundled Prometheus
server with 15-day retention and a memory cap; a bundled OpenTelemetry
collector that feeds sampled traces into Sutradhar's own database for
7 days, with a built-in trace viewer, so no Grafana or Tempo is
needed; `/actuator/metrics`, an optional OTLP export setting and a
one-click Grafana dashboard export. Metrics and spans carry only IDs,
counts and durations, enforced by a test, and a "what we collect" page
lists every metric and attribute. The owner-facing layer is built in
the owner's order; the 36 features are listed in
[observability features](../../dev/observability-features.md).

## R128: telemetry practices and service levels

The owner's 56 OpenTelemetry and Prometheus practices are standing
rules, listed in [telemetry practices](../../dev/telemetry-practices.md).
The testable ones are enforced in the gate. Recording rules,
multi-window burn-rate alerts on the service levels, the collector's
redaction and memory-limiter processors, tail sampling that keeps all
errors and slow traces, last-success gauges for every job, queue age
metrics and exemplars where the stack supports them are added. The
service levels and the overhead budget are in
[ADR-0007](../../adr/0007-service-levels.md): 99.9% availability a
month, 99% of new posts' first suggestions within 30 seconds, and
telemetry overhead under 3% of CPU and latency.

Recorder's note: each new gate check is still put to the owner as a
question before it is switched on (R116).

## R129: installation through docker compose

The whole of Sutradhar, including the Python service and the bundled
monitoring services, runs from a default docker compose file, each
container with its memory cap.

Recorder's notes: the option the owner chose read "The whole of
Sutradhar, including the Python service, runs from the default docker
compose file"; the XenForo add-on itself still runs inside the forum's
own XenForo. R049's point-and-click setup is unchanged; the compose
file is what it installs. SPEC-010's host helper (MP-16) and
"service definition" are read as the compose project. Dell's test
sandbox is a separate matter and is unchanged.

## Replaced or changed

- R098: the runtime for bge-m3 becomes ONNX Runtime and the spaCy
  model becomes `en_core_web_lg` (R120); the rule that bge-m3 reads
  both sides is kept, with the destination read through its title and
  summary first (R123); the first pass no longer decides what bge-m3
  reads first, which now follows views and recency (R123), but still
  gives the early shortlist.
- R106: the official ONNX export of bge-m3 joins the listed formats
  (R120).
- ADR-0002's list of loaded formats (safetensors and spaCy pipeline
  files, "which allows `en_core_web_trf`"): bge-m3's official ONNX
  export is also loaded and `en_core_web_trf` is no longer used (R120).
- R107's "may take days" for the first run: the owner accepted one to
  several weeks per million posts (SPEC-008 Q-RD01).
- R021 and R092 as amended (6 GB): 8 GB if confirmed (R121).
- R092's "product instrumentation stays off by default": replaced for
  metrics and tracing by R127.
- R015: waived only for an administrator's confirmed community model
  choice (R125); its hardware and service level are set by R124.
- SPEC-012's P-VL09 (tracing off by default): replaced by R127.
