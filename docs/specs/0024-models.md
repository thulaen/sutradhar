# 0024: Models and NLP artefact admission

> The Model Hub (finding, downloading, swapping and removing models) is
> owned by spec 0080. This spec keeps model admission, quarantine and file
> integrity. Wording here that names candle, Qwen3-Embedding or potion is
> superseded by ADR-0020 and ADR-0021.

## Purpose and ownership

This specification owns model and trained-pipeline identity, dependency
locking, download and quarantine, conversion to data-only artefacts,
installation, verification, local serving, compatibility keys, health and
removal. It supplies the artefact and index mechanics used by champion
transitions, but does not own a promotion or rollback decision. It covers
BGE-M3, the BGE cross-encoder, spaCy pipelines, custom spaCy models, the
eight-head neural ranker, the optional paid dense route in spec 0025 and
any separately admitted judge model. It is roadmap Phase 4 item 3.

Spec 0022 owns reading and index generations. Spec 0026 owns text and
spaCy feature semantics. Spec 0027 owns candidate ranking. Spec 0057 owns
quality evaluation. Spec 0077 owns every optimisation, promotion,
activation and rollback decision and its complete champion manifests.
Spec 0019 owns the PostgreSQL jobs and transactional outbox.

## Target model and pipeline roles

Exactly one complete compatible champion artefact set is active per site,
with one admitted identity for each required role within that manifest:

| Role | Target artefact | Purpose |
|---|---|---|
| `embedder` | `BAAI/bge-m3`, revision `5617a9f61b028005a4858fdac845db406aefb181`, MIT | Default dense representation and mandatory sparse and multi-vector representations |
| `dense_embedder` | Optional exact paid Bedrock identity admitted under spec 0025 | Sole paid production exception for the dense channel only; never sparse, multi-vector or reranking output |
| `reranker` | `BAAI/bge-reranker-v2-m3`, revision `953dc6f6f85a1b2dbfca4c34a2796e7dde08d41e`, Apache-2.0 | One multilingual cross-encoder score per source/destination pair |
| `nlp_pipeline` | A spaCy pipeline per declared language and capability in spec 0026 | Typed linguistic and entity features; not a competing retriever or reranker |
| `neural_ranker` | Candidate-specific eight-head network from spec 0077 | Eight ordinal prediction heads over admitted features |
| `judge` | Separately admitted evaluation model where spec 0057 uses one | Usefulness assistance only; never a production retriever or reranker |
| `lab_only` | Hash-locked comparison artefact with `ship = false` | Research baseline only, never production activation |

Qwen3-Embedding-0.6B, potion-retrieval-32M and
`gte-reranker-modernbert-base` are not target production roles. A lab
comparison must not create an alternate production stack. BGE-M3 weights
and the BGE cross-encoder are not fine-tuned by Optuna; optimisation covers
the declared retrieval/fusion configuration and eight-head ranker. Custom
spaCy training follows spec 0026's data and evaluation contract and spec
0077's promotion route. Optuna cannot select, propose or change a paid
provider or model identity.

## Immutable model lock

`models.lock` is the canonical lock for shipped base artefacts. Site
overlays may select only an already admitted language/capability variant
or a spec 0025 paid dense candidate promoted through spec 0077. They
cannot introduce another reranker, replace BGE-M3 sparse or multi-vector,
or select a paid dense identity outside that exception. The lock is
written by the canonical pinning tool and checked byte for byte. A
base-model change needs owner approval and spec 0057/0077 promotion
evidence. A candidate configuration change follows spec 0077 and cannot
change this base lock.

Each entry contains:

- stable id, role, purpose and supported capabilities;
- official source repository and full immutable revision;
- licence, licence-text digest, notices and data terms;
- runtime and exact compatible dependency/container locks;
- source and production formats, architecture and operator allowlists;
- every source and exported file's byte size and SHA-256;
- tokenizer/configuration file list and combined tokenizer digest;
- pooling, normalisation, instruction/prefix and output-shape contract;
- dense dimension, sparse and multi-vector output contracts where enabled;
- maximum input tokens and the publisher evidence for that limit;
- index-compatibility version and feature-schema version;
- fixed self-test inputs, expected outputs, order and numerical tolerance;
- measured resource profile and admission record;
- previous admitted set for rollback; and
- `ship = false` for lab-only entries.

Branch names, moving tags and unversioned “latest” references are invalid.
Missing file hashes, transitive dependency hashes, tokenizer limits or
self-test values refuse implementation and activation. A generated lock
is not hand-edited to bypass the source-binding check.

The compatibility baseline for the data platform is recorded in spec 0077.
Exact spaCy packages, language pipelines, knowledge bases, custom
components and the Optuna, training and runtime artefacts enter
the same lock graph with hashes and integration evidence.

## Safe acquisition and conversion

The fetcher and converter are separate from production serving. Only the
fetcher has network access, only for an owner-authorised install
or other admitted spec 0019 job. It contacts the official publisher
endpoint
over HTTPS with certificate validation. Repository code is never fetched
for execution and `trust_remote_code` is always false.

1. Read the locked revision's file list and admit only declared files.
2. Stream into a service-owned staging directory with byte limits and
   resumable ranges. Compute SHA-256 while writing.
3. Refuse absolute paths, traversal, links, duplicate archive names,
   undeclared files, executable/library content and nested pickle or
   `data.pkl` containers.
4. Compare every size and digest with the lock. Quarantine mismatch;
   never load or rename it into an admitted directory.
5. Scan configuration for `auto_map`, custom pipelines, unknown
   architectures, custom operators and external paths.
6. Convert only in a network-disabled quarantine with no credentials,
   read-only inputs and a pinned converter runtime.
7. Where a pickle-capable PyTorch source is unavoidable, use restricted
   `weights_only=True` loading only after publisher and hash verification.
   This is not a claim that untrusted pickle is safe.
8. Export reviewed data-only artefacts, normally safetensors or an
   approved ONNX graph with standard operators and local external weights.
9. Validate tensor names, shapes, dtypes, finiteness, vector dimensions,
   tokenizer behaviour and fixed output fixtures against the source.
10. Hash the export and record source, export, converter and input
    digests. Rename into the immutable store and make it durable.
    Only a fenced registry transaction can publish the reference
    and its receipt. Directory presence grants no admission.
    Recovery verifies digest, authority and effect key. Clean
    unreferenced staging through the ledger, never a live reference.

Production never loads `.bin`, `.pt`, `.pth`, `.ckpt`, `.pkl`, `.pickle`,
`.joblib` or arbitrary archives, and never executes publisher code. A
data-only extension does not override content inspection.

The official BGE-M3 repository's presence of `pytorch_model.bin`,
`sparse_linear.pt` and `colbert_linear.pt` means a blanket “all official
artefacts are data-only” claim is forbidden. The cross-encoder's locked
`model.safetensors` route is preferred when it passes parity.

## Installation and verification

Install and update use `model.artefact.install` (schema 1, spec
0019 maintenance lane), with byte and stage progress. Key the
effect by authorised install scope, exact lock digest, artefact
digest and operation revision. Stage and verify bytes before the current fenced
job records the admitted artefact. This install step cannot change
the champion pointer. Preflight verifies service-owned directories, free disk
for source/export/staging and two index generations, resource admission,
licence acceptance and compatible locks. A missing required artefact stops
installation without changing active models.

Offline bundles are streamed, uncompressed archives with a manifest.
Every member must be a regular declared file, within its byte limit and
matched independently against `models.lock`. The manifest alone is never
trusted. Partial bundles remain in quarantine and resume by digest.

Verification occurs after installation, at every new serving-process
start, daily and on demand. It checks the exact file set, sizes, hashes,
permissions and absence of unlisted files. Same-process reload may use
identity, size and modification evidence only after a successful full
hash; a new process always rehashes. Failure makes the artefact
unavailable and preserves the prior active set.

Access tokens, where an official gated source requires one, enter through
a masked prompt or standard input, never a command argument, environment,
URL, log, audit value, progress message or diagnostics bundle. They live
only for the fetch job and are discarded afterwards.

## Local serving and typed interfaces

The model host has no outbound network and loads only local admitted
paths. Approved Python inference and spaCy packages run with pinned
dependencies and resource limits through the admitted Rust embedding
bridge or a packaged local service. All first-party integration is Rust;
no first-party Python production worker or adapter is permitted. Rust
product services use owner-only local IPC or the admitted in-process
bridge with versioned, bounded messages. No model endpoint listens on a
network port.

The shared host loads one admitted copy per active artefact and resource
profile. Workers do not load private copies to evade memory accounting.
Turn-taking or separate non-overlapping profiles are required when two
sets cannot fit simultaneously.

Product-facing contracts are:

```text
Embed(request_id, tenant_id, model_key, mode, texts)
  -> vectors or sparse/multi-vector outputs, feature_schema, model_key

Rerank(request_id, tenant_id, model_key, source_destination_pairs)
  -> one finite cross-encoder score per pair, model_key

Parse(request_id, tenant_id, pipeline_key, language, source_digest, text)
  -> typed spaCy feature set with offsets and provenance

RankHeads(request_id, tenant_id, candidate_manifest, feature_rows)
  -> eight five-logit vectors per row and calibration version
```

Requests carry maximum counts and bytes, deadline, trace and idempotency
identity. Responses contain no source text and are bound to the exact
model, tokenizer, pipeline and feature-schema keys. Unknown modes,
non-finite outputs, mismatched counts, deadline excess and incompatible
keys are typed errors; no silent truncation or zero-filled output is
allowed.

A production champion includes BGE-M3 sparse and multi-vector modes and
exactly one admitted dense mode. The sole production exception to BGE-M3
dense is an exact paid Bedrock model admitted under spec 0025 and promoted
through spec 0077. The paid candidate retains BGE-M3 sparse and
multi-vector artefacts and a complete verified local rollback set.
Diagnostic manifests may disable a channel for ablation, but spec 0077
cannot promote a configuration missing a required active channel.
Tokenisation, chunking and complete-reading rules come from spec 0026.
Published support for 8,192 tokens does not permit truncating a longer
source and representing it as complete. Cross-encoder candidate counts and
batching follow spec 0077 and the measured profile.

## Compatibility keys and index generations

An embedder key includes model id, revision, exported-artefact digest,
tokenizer digest, enabled modes, pooling, normalisation, prefixes, input
window, precision and index-compatibility version. A paid dense key also
includes the exact provider identity, API contract and self-test
fingerprint under spec 0025. A spaCy key includes pipeline, component
order, knowledge base, feature schema, offsets and language. A neural key
includes its complete candidate manifest.

Every vector, feature row and suggestion records the keys that produced
it. Stored output is valid only for its exact compatible key. A mismatch
is missing evidence, never a zero score. New suggestions block while an
active representation and its index generation differ; existing
suggestions remain reviewable with their original keys.

An embedder change requires a complete new generation for every
representation whose compatibility key changes. The complete hybrid
candidate also verifies every unchanged BM25, sparse, multi-vector,
reranker and spaCy generation it references. Spec 0077 switches the
manifest and all referenced generation identities atomically after
verification. A reranker, spaCy pipeline or neural-head change requires
the downstream invalidation and revalidation named in spec 0026 or 0077.
No request may combine old weights with a new tokenizer, feature schema
or calibration.

## Champion transition mechanics

Base-model updates require owner approval and are never downloaded merely
because a new upstream revision exists. Every base-model, paid-model or
candidate-configuration activation follows spec 0077's protected champion
controller. None activates from owner approval, a training metric or an
Optuna trial record alone.

Spec 0077's activation preflight verifies the complete set, expected
active digests, index generations, resource profile and release receipts.
Its single-writer compare-and-set exposes either the whole old set or the
whole new set. In-flight work finishes on its starting set or reconciles
and retries under its idempotency contract.

The previous verified set and compatible indexes are retained for spec
0077 rollback. That rollback restores model weights, tokenizer, spaCy
pipelines, knowledge bases, neural heads, feature schemas, calibration
and retrieval/ranking configuration together. It succeeds only after
digest, readiness and representative output checks pass and in-flight
work is reconciled.

A failed update or rollback leaves the previous set active where it
remains verifiable. If no set is verifiable, new suggestions stop and the
health screen names the failed artefacts; existing site pages are not
changed.

## Health and readiness

Each model row shows role, immutable revision, enabled capabilities,
last verification, load time, measured resident/peak memory, device,
inference p50/p95, error count and last typed error. It never shows post
text, credentials or access tokens.

Readiness states are `not_installed`, `downloading`, `verifying`,
`loaded`, `degraded`, `failed`, `update_available`, `index_mismatch` and
`quarantined`. A required missing/failed model or index mismatch is not
ready for new suggestions. A measured latency or error-rate breach marks
degraded and follows the performance policy; it is never hidden by
changing the baseline.

Metrics cover inference duration, errors, resident bytes, peak bytes,
queue delay, verification time and active status by registered role and
model key. High-cardinality text, tenant names and raw content are not
metric labels.

## Resource admission

All serving and training memory counts within the applicable Sutradhar
slice. CPU, GPU, VRAM, unreclaimable memory, queue delay and p95 latency
are measured for the complete profile. Disk cache is reported separately
and is not treated as unreclaimable model memory.

The old Qwen/potion/gte memory table and rebuild-time estimates are
superseded and are not evidence for BGE or spaCy. Admission measures:

- base services, PostgreSQL, queue, scheduler and outbox relay;
- BGE-M3 and cross-encoder load and peak batch memory;
- every active spaCy pipeline and knowledge base;
- neural ranker and evaluation workers;
- index generation and rollback headroom; and
- non-overlapping training, evaluation and production-serving profiles.

If the complete profile cannot meet the owner-PC service and desktop
ceilings in AGENTS.md section 5.6 or the applicable Dell admission limits
in section 11, the affected activation or release is blocked. No numerical
memory or latency result is claimed without a measurement receipt naming
hardware, versions, batch, input sizes and load.

## Failure contract

Existing model error codes retain their general meanings:

| Code | Outcome |
|---|---|
| `MF-E01` | Official-source network failure; retry only the fetch under its bounded policy and retain safe staging |
| `MF-E02` | Locked revision/file unavailable; stop and offer the verified offline bundle, never substitute a revision |
| `MF-E03` | Size/digest mismatch; quarantine and alert, never load |
| `MF-E04` | Insufficient disk; refuse before or stop safely during staging |
| `MF-E05` | Forbidden, executable, pickle-capable production or unsafe archive content; refuse the whole artefact |
| `MF-E06` | Custom code, unknown architecture/operator or external path requested; refuse before loading |
| `MF-E07` | Resource envelope exceeded; refuse load or stop the admitted trial and preserve the active set |
| `MF-E08` | Self-test/parity failure; quarantine and keep suggestions not ready if no prior set is valid |
| `MF-E09` | Index/model key mismatch; block new suggestions and retain old suggestions for review |
| `MF-E10` | Installed revision absent from the admitted lock; never load |
| `MF-E11` | Site overlay or lock digest mismatch; refuse the affected variant |
| `MF-E12` | Installed file fails verification; mark failed and offer verified reinstall |
| `MF-E13` | Publisher metadata/search failure; no unrecorded substitution |
| `MF-E14` | Access token refused; discard it and request authorised action |
| `MF-E15` | Rebuild/activation failure; retain or restore the verified previous set |
| `MF-E16` | Rollback cannot be verified; keep activation blocked and quarantine the failed candidate |
| `MF-E17` | Forced stop after the stop timeout; recover from checkpoints |
| `MF-E18` | Interrupted removal; resume from the deletion receipt |
| `MF-E21` | Production receives an unsafe model format or remote-code request; refuse before deserialisation |
| `MF-E22` | spaCy language/capability unsupported; return the declared unsupported outcome, never a fabricated annotation |
| `MF-E23` | Model response has wrong count, shape, non-finite value or incompatible key; reject the whole response |

Model failures never change public site pages. Repeated failures follow
spec 0019's retry/dead-letter and fault-tracking rules.

## Stores and removal

Removing a site or uninstalling Sutradhar deletes embeddings, sparse and
multi-vector indexes, model files, previous revisions, spaCy pipelines,
custom components, neural weights, calibration, staging, trial vectors
and verification markers without an export or keep option. It never
follows symbolic links or deletes outside the recorded data directories.

Reading records, suggestion and decision history, licence acceptances,
activation/rollback history, settings and audit evidence follow their
feature's export and retention contract. Published links and the site's
own admin log remain. Lab training data follows its separate erasure and
retention rules; an erasure marks every derived training set and model
for the required retraining or quarantine procedure.

Removal is resumable and records counts, bytes, classes, time and any
remainder. Destructive removal and restore are strict-core operations.

## Requirement, design and task traceability

These identities group the normative contracts above. They do not show
execution state or admit an implementation task. Derived tasks still
follow spec 0076's readiness gate.

| Requirement | Normative obligation | Design and derived task | Unit evidence | BDD evidence |
|---|---|---|---|---|
| `REQ-0024-ARTEFACT-INTEGRITY` | Canonical immutable locks, official-source binding, quarantined data-only conversion, exact export parity, secret redaction and offline verification prevent unadmitted or unsafe artefacts from entering production | `DES-0024-MODEL-PLATFORM`; `TASK-0024-MODEL-PLATFORM` | `UT-0024-LOCK-CANONICAL`, `UT-0024-UNSAFE-FORMAT`, `UT-0024-ARCHIVE-SAFETY`, `UT-0024-EXPORT-PARITY` | `BDD-0024-ADMISSION-QUARANTINE` |
| `REQ-0024-COMPATIBLE-OPERATION` | Typed local interfaces, exact compatibility keys, complete resource-admitted champion sets and atomic activation or rollback prevent mixed or insufficient representations | `DES-0024-MODEL-PLATFORM`; `TASK-0024-MODEL-PLATFORM` | `UT-0024-TOKENIZER-KEY`, `UT-0024-RESPONSE-SHAPE`, `UT-0024-SPACY-UNSUPPORTED`, `UT-0024-RESOURCE-REFUSAL` | `BDD-0024-INDEX-MISMATCH` |
| `REQ-0024-DERIVED-REMOVAL` | Erasure and uninstall propagate to derived models and remove only declared data through resumable bounded receipts without following links or deleting external evidence | `DES-0024-MODEL-PLATFORM`; `TASK-0024-MODEL-PLATFORM` | `UT-0024-REMOVAL-SAFETY` | `BDD-0024-ERASURE-DERIVED`, `BDD-0024-REMOVAL-RESUME` |

| Design | Owning contract | Requirements and derived task |
|---|---|---|
| `DES-0024-MODEL-PLATFORM` | The immutable model lock through the stores and removal contracts above | `REQ-0024-ARTEFACT-INTEGRITY`, `REQ-0024-COMPATIBLE-OPERATION`, `REQ-0024-DERIVED-REMOVAL`; `TASK-0024-MODEL-PLATFORM` |

| Derived task | Implementation boundary | Upstream identities |
|---|---|---|
| `TASK-0024-MODEL-PLATFORM` | Implement the Rust-owned lock, admission, conversion, packaged-model interface, compatibility, transition and removal boundaries without first-party Python production code | `DES-0024-MODEL-PLATFORM`; `REQ-0024-ARTEFACT-INTEGRITY`, `REQ-0024-COMPATIBLE-OPERATION`, `REQ-0024-DERIVED-REMOVAL` |

## Verification design

These are test designs, not claims that tests have run.

| Test | Requirement | Case | Required result |
|---|---|---|---|
| `UT-0024-LOCK-CANONICAL` | `REQ-0024-ARTEFACT-INTEGRITY` | Missing hash, moving tag, unknown field, duplicate role and non-canonical lock | Refuse each and preserve the previous lock |
| `UT-0024-UNSAFE-FORMAT` | `REQ-0024-ARTEFACT-INTEGRITY` | Pickle renamed `.safetensors`, nested `data.pkl`, executable header, `.pt` production input and remote-code config | Refuse before production load; only quarantined verified conversion can admit source weights |
| `UT-0024-ARCHIVE-SAFETY` | `REQ-0024-ARTEFACT-INTEGRITY` | Absolute path, `..`, link, duplicate, oversize and undeclared member | Refuse the whole bundle without changing installed files |
| `UT-0024-EXPORT-PARITY` | `REQ-0024-ARTEFACT-INTEGRITY` | Fixed BGE dense/sparse/multi-vector and reranker fixtures | Exported values and ordering match the approved source within locked tolerances; wrong shape or NaN fails |
| `UT-0024-TOKENIZER-KEY` | `REQ-0024-COMPATIBLE-OPERATION` | Same weights with another tokenizer or prefix | New compatibility key and mandatory index rebuild; no silent reuse |
| `UT-0024-RESPONSE-SHAPE` | `REQ-0024-COMPATIBLE-OPERATION` | Reranker returns NaN, too few scores or an old model key | Reject the complete response and emit `MF-E23` |
| `UT-0024-SPACY-UNSUPPORTED` | `REQ-0024-COMPATIBLE-OPERATION` | Tokeniser exists but requested trained NER model is absent | `MF-E22`; no tokeniser-only output masquerades as NER |
| `UT-0024-RESOURCE-REFUSAL` | `REQ-0024-COMPATIBLE-OPERATION` | Combined active profile exceeds its budget | Refuse activation before replacing the previous set |
| `UT-0024-REMOVAL-SAFETY` | `REQ-0024-DERIVED-REMOVAL` | Recorded symlink, outside path and interrupted deletion receipt | Refuse traversal and outside deletion; resume only the declared remainder with exact counts and bytes |
| `IT-0024-FENCED-INSTALL` | `REQ-0024-ARTEFACT-INTEGRITY` | Crash after rename, lose the lease before commit, or submit different bytes under one effect key | A stale or conflicting attempt cannot publish. Valid retry reconciles the immutable bytes and receipt, with no champion pointer write |
| `IT-0024-OFFLINE-LOAD` | `REQ-0024-ARTEFACT-INTEGRITY` | Start serving with network disabled and a missing locked file | Valid files load; missing file fails without attempted download |
| `IT-0024-ATOMIC-UPDATE` | `REQ-0024-COMPATIBLE-OPERATION` | Fault at download, conversion, index build, compare-and-set and readiness | Every case exposes one complete old or new set, never a mixture |
| `IT-0024-ROLLBACK-SET` | `REQ-0024-COMPATIBLE-OPERATION` | Restore previous model but mismatched spaCy/calibration | Rollback remains unverified until the complete compatible set is restored |
| `IT-0024-TOKEN-REDACTION` | `REQ-0024-ARTEFACT-INTEGRITY` | Exercise successful and failed gated downloads | Token appears in no argument, environment, file, log, audit, metric or diagnostics bundle |
| `BDD-0024-ADMISSION-QUARANTINE` | `REQ-0024-ARTEFACT-INTEGRITY` | A digest mismatch appears while a verified previous set is active | Quarantine the candidate, preserve the previous set and never rename or load the mismatched bytes |
| `BDD-0024-INDEX-MISMATCH` | `REQ-0024-COMPATIBLE-OPERATION` | New embedder activated before its generation completes. In a separate case verify the exact locked model file after the existing MF-E12 reinstall. Keep the required index incompatible and no alternate complete set verifiable. | New suggestions block and existing suggestions remain reviewable with original keys under current rights. In the reinstall case show file verification separately from `index_mismatch` and MF-E09 for the serving set. No activation or new suggestions follow. The same-profile control with a completely verified compatible set can serve through its existing admission checks. |
| `BDD-0024-ERASURE-DERIVED` | `REQ-0024-DERIVED-REMOVAL` | Source used in training is erased | Derived datasets/models enter the specified purge/retrain/quarantine workflow |
| `BDD-0024-REMOVAL-RESUME` | `REQ-0024-DERIVED-REMOVAL` | Removal is interrupted after some files are deleted | Resume deletes only declared paths and reports exact remaining classes |
| `PRP-0024-NO-MIXED-SET` | `REQ-0024-COMPATIBLE-OPERATION` | Generate failures and concurrent readers during every transition | No reader observes weights, tokenizer, features and calibration from different manifests |

In IT-0024-ROLLBACK-SET, give the current rollback an exact signed
decision, the complete prior set and its matching index. Pause after
the profile check and before the active pointer changes. Resume
with a different synthetic host profile on the same supported OS.
Keep the model bytes, dependency lock and stated memory budget
fixed. Offer only the original profile's valid measured resource
record. It cannot admit
the current profile. Refuse the rollback before loading that set
for serving or changing the pointer. Give
ERR-0077-ROLLBACK-UNVERIFIED and preserve the earlier proof in its
original scope. In an independent control on the original profile,
reconcile the interruption and verify the same set and current
lease. It can complete one rollback when every check passes.

In a separate IT-0024-ROLLBACK-SET case use the 0078 processor
route with graphics offload effectively off. Give the original
rollback the correct model and index and the current admitted
processor measurement. Withhold only its current execution lease
before any serving load or pointer change. Keep the rollback
unverified. With current read permission, show verification of the
files and processor profile separately from the serving device and
rollback outcome. A verified processor profile is not an observed
load or successful rollback. Follow the existing fenced recovery
for the original intent. The exact current lease control can serve
only when the rest of the 0077 rollback checks pass. Viewing either
status does not start or retry work.

Real pinned-model tests run offline from the verified cache. They record
load time, memory, latency and output parity on each admitted hardware
profile. Static checks refuse production code-loading paths and network
access from the model host. Every long operation emits progress and every
mutation emits one audit event.

## Research provenance

BGE-M3's architecture and capabilities follow
[arXiv:2402.03216](https://arxiv.org/abs/2402.03216) and the exact
publisher model card/revision in the lock. The reranker follows its exact
publisher card/revision. spaCy component behaviour follows the locked
official documentation and model cards. safetensors and ONNX admissions
follow their respective public format specifications and runtime operator
allowlists.

These sources establish technical provenance, not output quality on
Sutradhar's data, resource fit or legal clearance. Those require separate
evaluation, measurement and legal evidence under specs 0057 and 0076.
