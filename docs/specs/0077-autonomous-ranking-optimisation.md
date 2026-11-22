# 0077: Autonomous ranking optimisation and champion control

## Purpose and ownership

This specification owns autonomous ranking optimisation, the eight-head
neural prediction contract, Optuna study boundaries, ranking workflows,
immutable candidate artefacts, champion/challenger transitions and
verified rollback.

Spec 0024 owns model artefact admission and serving. Spec 0026 owns text
and spaCy feature production. Spec 0027 owns candidate eligibility,
ranking and the 3–6 internal-destination gate. Spec 0053 owns feedback
and training-data acquisition. Spec 0057 owns labels, metric definitions,
evaluation datasets and release assessment. Spec 0019 owns platform-wide
PostgreSQL queue, transactional outbox and Rust scheduler. Spec 0110
owns their relay and scheduling primitives. This specification
registers ranking work with those contracts.

No outcome of this specification permits an unauthorised or stale source,
partial reading represented as complete evidence, weakened permission or
source-integrity control, changed evaluation label or release threshold,
or unmoderated public write. Model scores are ranking features, never
factual evidence.

Example (illustrative; the numbers come from the test cases below): the
champion's weight for one ranking channel is 100. A nightly study proposes
a challenger at 102. After normalisation this is inside the 2% trust region,
so the challenger may go forward. A second trial resolves to 103. It is
marked diagnostic only (`ERR-0077-NUDGE-BOUND`) and cannot be promoted. At
the weekly evaluation the frozen challenger scores 0.949 nDCG@10. That is
below the 0.95 floor, so the controller gives `KEEP_CHAMPION`, nothing is
released and the floor failure stays visible.

## Words used here

- **Champion**: the complete set of artefacts now serving ranking.
- **Challenger**: a frozen candidate set that asks to replace the champion.
- **Head**: one of eight five-logit outputs of the neural ranker, each for
  one kind of human-adjudicated target.
- **Registry**: the versioned list of parameters Optuna may suggest, with
  bounds.
- **Nudge**: a production-eligible change that stays inside the 2% trust
  region around the champion.
- **Manifest**: the content-addressed record of every artefact and setting
  in a candidate.
- **Controller**: the independent component that gives exactly one outcome:
  `PROMOTE`, `KEEP_CHAMPION`, `ROLL_BACK` or `BLOCK_FOR_DATA_QUALITY`.
- **Tier**: an evaluation size from local diagnostic to monthly integrity.
- **Generation**: the monotone number of the active-champion pointer.

## What this spec designs

| Part | What the owner or program sees |
|---|---|
| Architecture and trust boundaries | Rust owns decisions; Python packages return bounded results |
| Immutable artefact and dependency lock | A candidate is a hash-addressed set, never a model name |
| spaCy feature contract | Typed features with one producer per value |
| Eight neural prediction heads | Eight masked ordinal heads that never become labels |
| Typed Optuna registry and nudges | Bounded tuning with a 2% trust region |
| Dataset separation and evaluation tiers | Fixed tier sizes; weekly set held out from Optuna |
| Registered ranking workflows | Six typed job kinds under spec 0019 |
| Job and event recovery | At-least-once delivery with one effect per key |
| Candidate and controller state machine | Eleven candidate states, four outcomes |
| Promotion comparison | Paired evidence and a bootstrap lower bound |
| Atomic activation and verified rollback | One compare-and-set; rollback restores a whole set |
| Resource admission | Combined measured envelope before training or promotion |

## Architecture and trust boundaries

The route is:

1. approved source features and labels are frozen into immutable,
   versioned datasets by Rust product services;
2. the Rust scheduler admits typed ranking jobs under specs 0019
   and 0110, with bounded claims and retries;
3. Rust-owned integration invokes pinned Optuna, training, BGE and spaCy
   packages through an admitted embedding bridge, packaged executable or
   packaged service interface;
4. Rust owns the objective, workflow decisions, job state and all
   production ranking integration. The bounded spec 0104 sidecar
   invokes admitted packages without evaluating owner policy;
5. Rust product services consume only admitted, hash-addressed artefacts
   through typed interfaces;
6. the independent promotion controller evaluates protected evidence and
   emits exactly one controller outcome; and
7. the Rust release controller performs an atomic champion transition or
   verified rollback.

The Optuna bridge exposes only registry-bound study, ask, tell, prune and
result operations. Objective evaluation is a Rust-owned typed function
whose inputs and outputs cross the admitted bridge; no Python objective
source is written. Training uses a packaged training application with
declarative configuration. If the selected package route cannot express
the eight-head contract, implementation is blocked until another approved
package route can; custom Python is not a fallback.

Python packages cannot modify Cedar policy, RLS, signing keys,
source-integrity rules, evaluation labels, release thresholds, protected
Optuna bounds, the promotion controller or release gates. A worker
can submit only bounded results. It cannot approve its own candidate.
Job and event delivery is at least once under spec 0019.

## Immutable artefact and dependency lock

Every candidate is a content-addressed set, not a mutable model name. Its
manifest contains:

- candidate, parent champion, study, trial, source revision and run IDs;
- source, dependency-lock, container-image and toolchain digests;
- dataset, label-policy, metric-policy and BM25-baseline digests;
- every model, tokenizer, spaCy pipeline, knowledge-base and approved
  packaged-component artefact digest;
- neural architecture, eight-head definitions, training seed and complete
  resolved Optuna parameters;
- nudge-base champion digest and each final resolved parameter delta;
- retrieval, fusion, reranking, editorial, diversity, calibration,
  batching and inference configuration;
- module-stage admissions and their diagnostic evidence references;
- supported platforms, languages and variants;
- resource profile and hardware identity;
- code, review, test and provenance evidence references; and
- the canonical-manifest encoding version and SHA-256 digest.

The target local model identities and the paid exception are fixed by
ADR-0020:

| Role | Publisher artefact and revision | Licence | Admission rule |
|---|---|---|---|
| Default dense and mandatory sparse and multi-vector retrieval | `BAAI/bge-m3`, `5617a9f61b028005a4858fdac845db406aefb181` | MIT | Declare and execute all three local channels; sparse and multi-vector remain in every production champion |
| Sole paid dense exception | Exact AWS Bedrock identity admitted under spec 0025 | Recorded provider and model terms | May replace only BGE-M3 dense computation after full evaluation and `PROMOTE`; never replaces sparse, multi-vector or reranking |
| Cross-encoder reranking | `BAAI/bge-reranker-v2-m3`, `953dc6f6f85a1b2dbfca4c34a2796e7dde08d41e` | Apache-2.0 | Use the publisher's data-only `model.safetensors` route where it satisfies the admitted runtime |

Every production champion includes BGE-M3 sparse and multi-vector
retrieval channels and exactly one admitted dense channel. The sole
production exception to BGE-M3 dense is an exact paid Bedrock model
admitted under spec 0025 and promoted through this specification. A paid
champion retains local BGE-M3 for sparse and multi-vector retrieval and a
complete verified local champion and compatible dense index for rollback
and spending-cap restoration. Paid and BGE-M3 dense vectors never share a
dense-index generation.

Optuna cannot select, propose or change a paid provider or model identity.
That identity enters only through spec 0025's owner-authorised admission
and this specification's full weekly promotion route, never through a
nudge. Each active dense, sparse and multi-vector channel is present, has
a positive normalised retrieval and fusion weight, and has its own
admitted candidate count. An omitted or disabled required channel, zero
production channel weight, mixed dense generation, unadmitted paid
identity or missing local rollback set cannot become an eligible
challenger, receive `PROMOTE`, or enter the active champion. A diagnostic
ablation has no promotion authority.

The dependency lock records mutually compatible exact versions and
artefact hashes for Python, Optuna, the packaged training application,
the admitted Rust bridge and sidecar, spaCy and each trained pipeline,
the admitted inference runtime, Rust/Tokio integration and PostgreSQL.
A version enters the normative lock only with publisher evidence and
an integration result covering the complete set. Missing hashes,
incompatible pins, an unproved bridge or an unlisted transitive artefact
refuse admission. For a 0053 feedback input, verify the manifest's
compatibility keys against the exact plan's admitted input profile.
Apply the 0053 compatibility check on freeze admission, before
0104 sidecar hand-off and at checkpoint resume. Keep the committed
input and effect identity during recovery. A currently admitted
worker with different pins cannot take over that retry.

The official BGE-M3 repository includes pickle-capable PyTorch artefacts
as well as other formats. Therefore:

1. download only publisher artefacts named by the lock and verify their
   hashes before opening;
2. perform conversion in a quarantined, network-disabled, read-only-input
   job with no secrets and no `trust_remote_code`;
3. where a PyTorch source must be read, use a pinned runtime and restricted
   `weights_only=True` loading; never treat that switch as permission to
   load untrusted data;
4. export approved weights to a reviewed data-only format, validate tensor
   names, shapes, dtypes and numerical fixtures, and hash the export;
5. production loads only admitted data-only artefacts and never loads
   pickle-capable files or remote model code; and
6. retain source and export digests, conversion receipt and parity
   evidence in the candidate manifest.

BGE-M3's published long-context capability does not authorise unlimited
input. Spec 0026 defines tokenisation and chunking, spec 0024 defines
serving limits, and every document beyond the admitted limit follows the
specified complete-reading path or is marked incomplete and excluded
from eligible ranking evidence.

## spaCy feature contract

spaCy may provide tokenisation, sentence segmentation, lemmatisation,
part-of-speech and dependency features, named entities, entity links,
text classifications, rule-based matches and approved packaged components.
Custom-trained models use spaCy's declarative training configuration and
packaged training command. No first-party Python custom component,
factory, callback or training loop is permitted. spaCy does not provide a
competing embedding retriever or cross-encoder reranker and does not make
release decisions.

Each `SpacyFeatureSet` version declares:

| Field | Contract |
|---|---|
| Input | Immutable source-content digest, language identifier, platform and complete-reading receipt |
| Pipeline | Package/model identity, revision, artefact digest, licence and component order |
| Capabilities | Exact enabled packaged components and their published signatures |
| Language matrix | Per-language tokeniser, trained pipeline, entity linker, knowledge base, classifier and fallback behaviour |
| Output schema | Typed field names, offsets, labels, confidence, provenance and schema version |
| Training provenance | Dataset and label digests, licences, seeds, metrics and declarative-training receipt |
| Quality | Per-language and per-component acceptance set, expected labels and resource budget |
| Failure | Unsupported language, missing component, conflicting offsets and model-load error outcomes |

Offsets refer to the immutable source bytes and Unicode contract in spec
0026. A pipeline that changes offsets or normalisation creates a new
feature-set version and downstream revalidation. Missing trained support
produces the declared unsupported-language outcome; it is not silently
replaced by a tokeniser-only result presented as equivalent.

Entity, classification and normalisation outputs have one producer.
Downstream ranking reads the typed feature set and cannot invoke a second
pipeline to obtain a competing value. spaCy transformer similarity, if
admitted as an ordinary feature, is not used as dense retrieval or
cross-encoder reranking.

## Eight neural prediction heads

The neural ranker consumes the admitted active-dense representation,
BGE-M3 sparse and multi-vector representations, retrieval scores, spaCy
features and declared editorial signals. It has eight separate five-logit
ordinal heads corresponding to relevance grades 0 through 4. Each head
uses masked softmax cross-entropy; missing labels are masked, never
imputed from another head or from model output.

| Head | Meaning of the human-adjudicated target | Required label evidence |
|---|---|---|
| `topical_relevance` | Destination addresses the source passage's subject | Source and destination complete-reading receipts and topical label |
| `contextual_anchor_fit` | The proposed anchor and surrounding sentence support a natural link to the destination | Exact source span, anchor, destination and contextual-fit label |
| `answer_usefulness` | The destination materially helps answer the reader's question or task | Reader-task label and answer-usefulness grade |
| `intent_alignment` | Destination matches informational, navigational, troubleshooting, comparison or transactional intent | Intent taxonomy version and adjudicated intent label |
| `evidence_quality` | Destination provides direct, current and attributable support for the linked claim | Evidence-passage references, freshness and quality grade |
| `entity_version_fit` | Entities, products, versions, dates and constraints agree between source and destination | Entity/version annotations and mismatch adjudication |
| `novelty_information_gain` | Destination adds information not already present in the source passage | Source-covered claims and destination-added-information label |
| `editorial_suitability` | Placing the link is editorially appropriate, non-deceptive and useful after other constraints | Editorial label and severe-failure flag where applicable |

A head score is not the release relevance grade. Spec 0057 derives its
metrics from adjudicated dataset labels, not from neural predictions.
Head outputs may be calibrated separately, but calibration cannot turn a
score into evidence that a statement is true.

The initial network is bounded by the parameter registry below. The eight
task weights are positive, normalised to sum to one and recorded in the
manifest. A head with no labels in a batch contributes zero to that
batch's loss and its absence is reported; it is not assigned a synthetic
target.

## Typed Optuna registry

Optuna uses define-by-run suggestions only from this versioned registry.
Every suggestion has a name, type, lower and upper bound, allowed values,
log/linear scale, conditional dependencies, unit and manifest path. An
unknown name, out-of-range value, NaN, infinity or changed bound refuses
the study. Protected source, permission, label, metric, release and
moderation parameters are not suggestible.

The approved search bounds are:

| Area | Parameter | Type and bound |
|---|---|---|
| Network | Hidden layers | Integer 1–4 |
| Network | Width per layer | Choice 64, 128, 256, 512 |
| Network | Activation | Choice GELU, ReLU |
| Network | Dropout | Float 0.0–0.5 |
| Training | Optimiser | Choice AdamW, SGD with momentum 0.9 |
| Training | Learning rate | Log float 0.00001–0.01 |
| Training | Batch size | Choice 16, 32, 64, 128, admitted also by memory |
| Training | Weight decay | Float 0.0–0.1 |
| Training | Maximum epochs | Integer 1–30, subject to pruning |
| Multitask | Each of eight head weights | Log float 0.001–100, then normalised to a simplex |
| BM25 | `k1` | Float 0.5–2.5 |
| BM25 | `b` | Float 0.0–1.0 |
| BM25 | Title, body, heading and phrase field weights | Independent floats 0.0–8.0, normalised to sum to one; all-zero is invalid |
| Retrieval | Active dense, BGE-M3 sparse and BGE-M3 multi-vector retrieval weights | Independent floats 0.0–8.0 for each active production channel, normalised; every production channel weight is positive, while a zero is diagnostic only |
| Retrieval | Candidate count for each active retrieval channel | Integer 20–500 for each of dense, sparse and multi-vector retrieval, deduplicated before fusion; every production count is present |
| Hybrid fusion | RRF rank constant | Integer 1–120 |
| Hybrid fusion | BM25, active dense and both BGE-M3 non-dense channel weights | Independent floats 0.0–8.0, normalised; every active production retrieval-channel weight is positive and a zero weight is diagnostic only |
| Cross-encoder | Input candidate count | Integer 10–100 and no greater than unique fused candidates |
| Cross-encoder | Source aggregation | Choice maximum, mean of per-source best, reciprocal-source fusion |
| Editorial | Contextual relevance, usefulness, overlap and diversity weights | Independent floats 0.0–8.0, normalised; all-zero is invalid |
| Diversity | Relevance/redundancy balance | Relevance float 0.75–0.95; redundancy is one minus relevance |
| Diversity | MMR starting balance | 0.85 relevance and 0.15 redundancy, a tunable starting value, not a measured optimum |
| Performance | Inference batch size | Choice 1, 2, 4, 8, 16, 32, admitted by memory |
| Performance | Worker concurrency | Integer 1–8, admitted by CPU, memory and downstream rate limits |
| Calibration | Temperature | Log float 0.05–20.0, fitted on calibration data only |
| Calibration | Abstention operating threshold | Float 0.50–0.99; this is not an evaluation or release threshold |

The initial cross-encoder input candidate count is 50. The initial production
candidate counts for dense, sparse and multi-vector are not admitted. Each
must be selected explicitly within its registered bound and supported by
development and resource evidence before implementation or activation. No
agent may infer a starting count from the cross-encoder count or from another
channel.

### Bounded autotuning and small nudges

The autotuner is spec 0077's bounded candidate-generation and evaluation route,
not a second controller. It covers only parameters expressly admitted to the
registry above. Adding a Sutradhar behaviour or parameter requires its complete
owning specification and a registered bounded contract; an agent cannot infer
tunability from the existence of a setting.

Production-eligible candidates use a 2% production-eligible trust region around
the current champion. The 2% is an owner-approved design bound, not a measured
optimum. The rules are:

1. For an eligible continuous parameter with a non-zero champion value, the
   final resolved and normalised values satisfy
   `abs(challenger - champion) <= 0.02 × abs(champion)`. The candidate also
   remains inside its registered bounds. Check this after normalisation so
   rescaling cannot create a larger hidden nudge in another weight.
2. Zero-valued champion parameters need an explicit absolute nudge bound before
   production-eligible tuning. Without one, that parameter's nudge route is
   blocked; no agent invents an epsilon or uses a percentage of zero.
3. An ordered integer or categorical parameter moves by one adjacent ordered
   choice under its declared registry ordering. An unordered categorical change
   is not a nudge and remains exploratory unless its own ADR admits another
   route.
4. Optimisation cannot set a production-eligible weight to zero, disable a
   required active retrieval channel or turn off a feature. Positive weights
   remain positive after normalisation. It cannot replace the dense model
   identity; only the spec 0025 paid exception and full promotion route can
   do that. This no-off rule does not weaken mandatory source, permission,
   moderator or safety controls.
5. Safety, permission, source-integrity, moderation, legal and rollback controls
   can disable or quarantine immediately. These are protective operations, not
   Optuna suggestions, and no nudge or champion can suppress them.
6. A study may explore the wider approved registry for diagnostics. No
   broad-exploration trial is directly eligible for promotion. A production
   challenger must be frozen and evaluated as an exact immutable candidate
   satisfying the trust region against the current champion.
7. The initial champion has no nudge base. Its initial configuration remains
   blocked until the applicable initial values and complete release evidence
   are admitted; the trust region is not a way to invent them.

A nudge never waives nDCG@10, the per-forum BM25 margin, link suitability,
severe-failure, calibration, resource or any other protected gate. Controller
outcomes and atomic activation remain unchanged.

MMR, xQuAD, deterministic source limits and novelty-aware reranking are
candidate methods, not mandatory shipment by assertion. A candidate must
preserve the relevance gate and demonstrate benefit on development
evidence before promotion evaluation. Diversity never forces an
irrelevant destination into the result.

The study uses a seeded multi-objective sampler admitted by the locked
Optuna version. Its sampler name, seed and complete configuration are in
the manifest. Objectives maximise nDCG@10, MRR, Recall@40, Precision@6
and adjudicated usefulness, and minimise expected calibration error, p95
latency, peak memory and severe-error count. Hard constraints reject any
trial that violates source integrity, permissions, candidate eligibility,
resource admission or protected limits.

Pruning uses intermediate development-set objective values only after
the minimum epoch and evaluation-step counts declared in the study
manifest. A pruned or failed trial cannot become a challenger. Resource
admission precedes every larger evaluation tier. Study storage records
the complete suggested and resolved parameters, pruning reason, failures,
artefact digests and random seeds.

## Dataset separation and evaluation tiers

Optuna sees only approved training, development and nightly-search
splits. The weekly promotion set is held out from study suggestions,
early stopping, calibration and feature selection. The controller returns
only the decision and permitted aggregate evidence to the study record;
it does not expose hidden labels or per-query hidden answers to Optuna.

Spec 0057 owns split rotation, adjudication and integrity. The fixed
sizes are:

| Tier | Threads | Queries | Authority |
|---|---:|---:|---|
| Local diagnostic | 16 | 32 | Debugging only |
| Push diagnostic | 48 | 96 | Required synthetic judge proof under spec 0057. Scores give no promotion authority |
| Extended integration | 256 | 512 | Challenger screening |
| Nightly Optuna | 512 | 1,024 | Search and pruning evidence |
| Weekly promotion | 2,000 | 4,000 | Required promotion evidence |
| Monthly integrity | All eligible threads | Separate reconciliation | Dataset, permission, source and result integrity |

Every sample is stratified across forums, languages, source types,
publication periods and thread lengths. A tier that cannot populate a
required stratum reports a data-quality block rather than silently
rebalancing after seeing results. Small tiers cannot establish the 0.95
floor, the 20% per-forum BM25 margin or a 0.005 difference.

## Registered ranking workflows

Each row is one typed Rust workflow under spec 0019. Use its maintenance
lane, with priority for protective rollback over tuning. A schedule may
enqueue work but cannot change policy or authorise a transition.

| Job kind (schema 1) | Trigger | Ordered work | Success evidence |
|---|---|---|---|
| `rank.dataset.freeze` | Approved dataset policy | Validate sources, permissions, complete reading, labels and strata; freeze manifests | Signed dataset and label-policy digests and reconciliation |
| `rank.optuna.nightly` | Nightly or authorised manual request | Admit resources, resume study, propose, train, prune and screen | Trial records and candidate manifests bound to exact input |
| `rank.challenger.weekly` | Weekly promotion window | Select eligible challengers, evaluate frozen pairs, compare champion | Signed pair and controller outcome |
| `rank.champion.activate` | Signed `PROMOTE` authorisation | Reverify set and policy, compare expected champion, install and check readiness | Signed receipt binding both manifests |
| `rank.champion.rollback` | Verified critical regression or failed activation check | Fence admission, restore and verify the complete prior set, then reconcile | Signed verified-rollback receipt |
| `rank.integrity.monthly` | Monthly | Reconcile eligible threads, permissions, labels, sources, artefacts and decisions | Signed integrity report and precise blockers |

Follow spec 0019's class registry for immutable bounded payloads, deadlines,
retries, concurrency, resource claims, checkpoint cursors, cancellation,
failure and audit retention. A scheduled fire and its job commit together.
A manual recovery keeps the immutable inputs and effect identity. An
authorised new run has a new identity and cannot bypass an unresolved
effect. Each row needs admitted limits and fixtures under QR-094.
Dataset freeze is keyed by source and policy digests. For a 0053
feedback export, use its existing typed source to check current
label eligibility before freezing. At publication apply 0053's
source correction order under the 0019 fence. The exact committed
source decides eligibility. Hold a preceding withdrawal or an
unproved order without publishing a freeze or admitting study
use. An eligible freeze committed first retains its receipt under
current read rights. Later artefact reads and study admission must
check current source eligibility. Studies key by site, dataset,
study and trial number. Weekly evaluation keys by
protected allocation, both manifests, policy and tier. Integrity
keys by site and scheduled fire or authorised request. Only the
registered schedule or authenticated action can create those keys.
A protected allocation and its use receipt are fixed before work.
A retry resumes its prior position and cannot draw a new allocation.

## Job and event recovery

1. Use the spec 0019 envelope for events and the same queue and outbox
   for ranked work. Typed fields bind run, candidate and manifest
   identities when present and the immutable input and policy digests.
   Dataset jobs carry dataset and label-policy digests. Control jobs
   carry the expected champion digest and signed controller reference.
   A queue record or event can never stand in for signed evidence.
2. Scope site jobs to their one site and the corresponding RLS context.
   Candidate work orders by site and candidate. Control orders by site
   and champion stream. Preserve per-key order and permit independent
   keys to proceed separately. Protected release evaluation keeps its
   isolated spec 0057 evidence route, with no cross-site production
   query. Store opaque references, digests and safe status in the
   shared runtime. The protected evaluator alone reads the labels.
   It passes the controller only signed aggregates and outcomes that
   spec 0057 permits. Queue rows, receipts, logs, checkpoints and
   dead letters expose no hidden labels, inputs or per-query results.
   Weekly scheduling has no release authority.
3. Commit each job's durable effect, fence, checkpoint and receipt
   together where they share a database. Publish a needed event in the
   same transaction as the change. Stage and verify file artefacts
   before the fenced database reference commits. Only the current
   lease and generation can publish. Same key and digest reuse the
   receipt. A conflict is quarantined. The activation pointer still
   requires the independent controller compare-and-set below.
4. Unauthorised, wrong-site, oversize, unsupported or changed inputs
   cause no work. Reconcile leased attempts, artefact digests, control
   receipts and outcome authority before a retry or restore. Rebuild
   first in isolation from the approved checkpoint. Never rerun a held
   external effect or let a stale worker publish success. The scheduler
   cannot lower a gate, supply held-out labels or prefer a candidate.

### Interrupted training result

For an interrupted 0104 training attempt, Rust reconciles the exact
site, logical job and trial, candidate, immutable input and attempt
fence under spec 0019 before publishing or resuming. A sidecar
reply or a verified staged file identifies an attempt's output.
Publication requires the committed result reference, checkpoint and
receipt. Cancellation that commits first or replacement of the
attempt fence refuses its delayed result, even if the staged bytes
match. Leave any replacement attempt unchanged. Retain only the
evidence permitted by the existing cleanup and retention
rules. If reconciliation cannot establish a committed result or
prove that publication did not occur, hold the effect.

After a committed result whose reply was lost, replay only its
original authorised receipt. An eligible replay can reuse the exact
published result. Check current source and label eligibility and
the exact output proof before returning an artefact or admitting
screening, weekly evaluation or another use. A later 0053
withdrawal holds the affected result from those uses while
preserving its permitted history. When reconciliation proves that
publication did not occur, the registered retry may resume an
eligible checkpoint only with current authority, the original
immutable input and the current fence.

A failed, held or recovered trial cannot change the active pointer,
its generation or the retained rollback record. A separately
verified fault affecting either verified set still follows the
existing protective checks and controller. Recovery of training
does not reverify either set or grant control authority.

## Candidate and controller state machine

Candidate lifecycle states are `FROZEN`, `ADMITTED`, `TRAINING`,
`SCREENING`, `PROMOTION_EVALUATION`, `ELIGIBLE`, `ACTIVE`,
`RETAINED_CHALLENGER`, `REJECTED`, `ROLLED_BACK` and `QUARANTINED`.
Dataset freeze records `FROZEN`. Admission and study work can enter
`ADMITTED`, `TRAINING` and `SCREENING`. The weekly evaluator owns
`PROMOTION_EVALUATION` and the controller decides eligibility
or rejection. Only the activation workflow with a signed controller
decision can enter `ACTIVE` and retain the previous challenger.
Only verified rollback can record `ROLLED_BACK`. The integrity
workflow reports issues and can request protective control. It
cannot approve a transition. The controller owns quarantine.

The controller has exactly four externally meaningful outcomes:

| Outcome | Required predicate | Effect |
|---|---|---|
| `PROMOTE` | Data quality passes; promotion-tier evidence is complete; nDCG@10 is at least 0.95; every per-forum BM25 Precision@6 gate passes; all quality, safety, calibration, diversity, link-suitability and resource gates pass; challenger improvement is established; no protected regression exists | Request an atomic transition from the exact expected champion to the exact challenger manifest |
| `KEEP_CHAMPION` | Evidence is valid but the challenger is not eligible for promotion, or no challenger improves without protected regression | Leave champion unchanged; never issue a release below a mandatory floor |
| `ROLL_BACK` | A verified post-transition critical regression or failed activation check affects the new champion | Stop new admission and restore the complete prior verified set under the rollback contract |
| `BLOCK_FOR_DATA_QUALITY` | Labels, source permissions, completeness, judged candidates, baseline validity, strata, digests, evaluation power or integrity are insufficient or contradictory | Prevent promotion and release; preserve evidence and name the failed integrity checks |

Data-quality failure takes precedence over promotion. A below-floor
champion can remain operational only under the existing operating policy;
`KEEP_CHAMPION` is never a release waiver. A failed mandatory gate cannot
be offset by improvement in another metric.

## Promotion comparison and protected regression

Champion and challenger run on identical immutable sources, judged pools,
hardware profile, warm-up, batching opportunities and request order. The
controller uses paired query results and paired resource measurements.
The point-estimate floors in spec 0057 are decisive; a confidence
interval cannot waive a failed floor.

Challenger improvement requires a positive paired nDCG@10 difference and
a one-sided 95% paired-bootstrap lower bound above zero, using 10,000
resamples seeded from the evaluation manifest. MRR and Recall@40 may not
fall by more than 0.005 absolute. Precision@6, adjudicated usefulness,
calibration, link suitability and severe-error measures obey their
independent spec 0057 gates and may not materially regress. No new severe
irrelevant top-ranked result is permitted.

p95 latency and peak memory must remain within their existing absolute
budgets and must not exceed the champion's paired point estimates.
Measurement repeats continue under the predefined maximum only to resolve
measurement instability; an underpowered or unstable comparison blocks
promotion rather than passing. Resource failure is not relabelled as a
data-quality outcome.

Promotion requires the weekly tier. Local, push, extended and nightly
results select and diagnose candidates but cannot authorise promotion.
Training-set improvement alone has no promotion authority.

## Atomic activation and verified rollback

A signed control decision binds its site or protected instance,
stream, action, expected current generation, both manifest digests,
policy and evidence receipt. Its effect key is the signed decision
identity. Persist the intent before staging or changing a pointer.
Activation and rollback use one compare-and-set on the digest and
monotone generation. Commit the pointer and effect receipt together.
A duplicate returns that receipt. A late decision cannot apply to
the same digest at a later generation. Reverify the signer, policy,
evidence, current lease and complete artefact set before the change.
After a crash, resolve the recorded intent before retrying. Before
restoring a prior ranking set for serving or changing the active
pointer, verify the current host, runtime lock, model and index
combination against its exact admission and measured resource
profile under specs 0024 and 0078. Repeat that check on resume.
An earlier profile's measurement has only its original scope.
For missing or mismatched current proof, keep rollback unverified
under ERR-0077-ROLLBACK-UNVERIFIED and new admission paused.
Preserve the existing custody and retention rules for the prior
set. Apply the existing failed-rollback contract.

The active set includes model weights, tokenizer, spaCy pipelines,
feature schemas, neural heads, calibration, retrieval/fusion/reranking
configuration, module-stage admissions and runtime compatibility metadata.
Readers see either the
complete old set or the complete new set. In-flight requests finish on
their starting manifest or are safely restarted under its idempotency
contract; they never mix sets.

The champion's module-stage receipt supplies the admitted stage.
Spec 0015 applies the current settings ceiling when the moderator
reads suggestions. A valid receipt for the unchanged champion gives
no exception to the lower ceiling or spec 0028's shadow restriction.

Rollback restores one complete prior verified manifest, not merely the
previous model file. It succeeds only when:

1. new candidate admission is paused;
2. every artefact digest and signer is reverified;
3. the atomic pointer names the intended prior set;
4. readiness, permission, source-integrity and representative ranking
   checks pass on that set. For a set derived from spec 0053 feedback,
   verify present eligibility under the current 0032 erasure history.
   A valid older manifest or receipt cannot establish this check after
   an erasure. A set held by 0053 cannot satisfy rollback verification;
5. in-flight and dead-letter work is reconciled without duplicate public
   effects; and
6. a signed receipt binds the failed set, restored set, reason, checks
   and operator-visible state.

Reconcile rollback success against the recorded signed rollback
decision, its effect key and the expected champion generation.
Verify that the restoration and checks belong to that decision
and its restored set. A signed receipt from an earlier rollback
can remain as permitted history. Equal failed and restored
manifest digests give it no authority over another effect key
or generation. When current verification is missing, use
`ERR-0077-ROLLBACK-UNVERIFIED`. Keep candidate admission paused
and give no `ROLLED_BACK` outcome for the current rollback.

A failed rollback quarantines the candidate and keeps release and new
activation blocked. It never reports success from a process exit or log
message alone.

## Resource admission

Use Dell's admitted queue and host limits under AGENTS.md section 11
and the owner's memory profiles under spec 0078. Each profile needs
combined resource measurements.
BGE inference, spaCy pipelines, PostgreSQL, the scheduler, bounded
sidecar workers and product services are measured together under
each declared profile.

Training and promotion evaluation do not overlap production serving or
one another unless a measured admission record proves the combined
envelope. The record must cover the full set of workloads that
would run together, their exact model and index generations,
batch sizes and concurrency on the current host. Include resident
and warm processes covered by the profile. Use the measured
combined peak and serving latency. Even when the sum fits and
each pair has valid measurements, hold an unmeasured larger
combination. Use the existing turn-taking rules and resource
promises. An admitted smaller combination may start after the
outgoing work reaches its safe point and its conflicting promise
is reconciled. Recheck admission at that boundary.

The resource record includes RSS and peak memory, CPU, GPU and
VRAM where applicable, model-loading and database worker memory,
p50/p95/p99 latency, queue delay and recovery headroom.

A candidate exceeding a budget is not promoted. An environment unable to
run the required evaluation honestly produces a resource block in the
run record and `KEEP_CHAMPION` where a controller outcome is required;
it does not lower sample sizes or waive release evidence.

Protective rollback has priority among eligible work under spec
0019. A rollback that needs a new model load still needs the
current combined resource promise. Keep an interrupted training
attempt's unresolved promise charged once under spec 0033 until
the exact authorised release is proved. An expired lease, current
memory reading or another attempt's receipt gives no capacity
from that charge. If the restored set fits only after its release,
hold the original rollback at admission with the spec 0078
unresolved-promise reason. A waiting rollback uses no load attempt
or pointer change and cannot supply rollback success. Preserve
the controller's existing protective hold. After reconciliation,
repeat current resource and controller checks for that same intent
before granting its load. A resource release gives no authority
to restore a ranking set.

## Error contract

| Error | Trigger | Result and recovery |
|---|---|---|
| `ERR-0077-ARTEFACT-DIGEST` | Fetched or exported bytes differ from the locked digest | Quarantine artefact, refuse execution, preserve receipt and require a newly admitted manifest |
| `ERR-0077-UNSAFE-MODEL-FORMAT` | Production is offered pickle-capable weights or remote model code | Refuse load; admit only through quarantined conversion and parity evidence |
| `ERR-0077-PARAMETER-BOUND` | A suggestion is unknown, protected, out of range or non-finite | Fail the trial before training; retain champion and audit the registry mismatch |
| `ERR-0077-NUDGE-BOUND` | A production candidate exceeds the 2% resolved-value trust region, jumps more than one ordered choice or is a broad-exploration trial | Mark diagnostic only; refuse promotion and activation |
| `ERR-0077-NUDGE-ZERO-BOUND` | A production nudge starts from a zero champion value without an explicit absolute bound | Block that parameter's nudge route; never invent an epsilon |
| `ERR-0077-BGE-CHANNEL-MISSING` | A proposed production champion omits or disables the active dense, BGE-M3 sparse or BGE-M3 multi-vector channel, gives one a zero weight, or lacks its candidate count | Mark the configuration diagnostic only; refuse promotion and activation and retain the complete champion |
| `ERR-0077-PAID-DENSE-ADMISSION` | A paid dense candidate lacks the exact spec 0025 identity, owner authorisation, diagnostic trial, local rollback set or full weekly `PROMOTE` evidence, or Optuna attempts to select its provider or model | Refuse promotion and activation; preserve the complete current champion and record the missing admission evidence |
| `ERR-0077-LANGUAGE-UNSUPPORTED` | spaCy matrix lacks the required trained capability | Emit the declared unsupported-language result; exclude from eligible evidence unless spec 0057 requires the stratum, which then blocks data quality |
| `ERR-0077-INCOMPLETE-SOURCE` | Complete-reading receipt is missing or source version changes | Exclude from candidate evidence and block affected evaluation |
| `ERR-0077-DUPLICATE-EXECUTION` | Idempotency key already has a result | Return the original result and do not repeat side effects; conflicting payload for the same key is quarantined |
| `ERR-0077-STALE-CHAMPION` | Expected champion digest differs at compare-and-set | Refuse activation and reevaluate against the actual champion; never overwrite it |
| `ERR-0077-EVALUATION-UNJUDGED` | A returned candidate lacks an eligible label | `BLOCK_FOR_DATA_QUALITY`; do not silently treat it as grade zero |
| `ERR-0077-BM25-ZERO` | Locked forum BM25 Precision@6 is zero | `BLOCK_FOR_DATA_QUALITY`; the relative gate is unevaluable, not passed |
| `ERR-0077-RESOURCE-REFUSED` | Predicted or measured envelope exceeds its admission budget | Stop before side effects or stop the trial safely; retain champion and block release evidence |
| `ERR-0077-ROLLBACK-UNVERIFIED` | Any rollback identity, readiness or reconciliation check fails | Quarantine, keep activation and release blocked, and alert for authorised recovery |

## Research and implementation provenance

The normative research baseline includes:

- Järvelin and Kekäläinen, “Cumulated gain-based evaluation of IR
  techniques”, DOI
  [10.1145/582415.582418][doi-cg], for graded cumulative-gain evaluation;
- Cormack, Clarke and Büttcher, reciprocal rank fusion, DOI
  [10.1145/1571941.1572114](https://doi.org/10.1145/1571941.1572114);
- Carbonell and Goldstein, maximal marginal relevance, DOI
  [10.1145/290941.291025](https://doi.org/10.1145/290941.291025);
- Santos, Macdonald and Ounis, xQuAD, DOI
  [10.1007/978-3-642-12275-0_11](https://doi.org/10.1007/978-3-642-12275-0_11);
- Chen et al., “BGE M3-Embedding”,
  [arXiv:2402.03216][bge], and the
  publisher model cards named above;
- Akiba et al., Optuna, DOI
  [10.1145/3292500.3330701][doi-optuna], plus the locked
  Optuna documentation;
- specs 0019 and 0110 for the project's queue, outbox and scheduler;
  and
- the locked spaCy API, pipeline and language documentation for each
  admitted component.

Implementations use original project code or expressly licensed
dependencies. Public formulas and protocols receive independent
conformance fixtures. No proprietary model weights, vendor labels,
private code, screens or unpublished thresholds are copied. Any strict
clean-room requirement follows spec 0076 with separate researcher and
implementer evidence.

These citations establish algorithmic provenance, not patent clearance or
freedom to operate. `LEG-0077-RANKING-PATENTS` requires a separately
scoped professional patent/legal review before release where flagged.
Optuna, BGE, spaCy and their transitive artefacts retain their
licence and notice obligations.

## Verification design

### Unit and property tests

| Test | Case | Required result |
|---|---|---|
| `UT-0077-NDCG-FLOOR-BOUNDARY` | Evaluation exactly 0.950000 and one representable step below with every other gate passing | Exactly 0.95 remains eligible; below fails promotion and release |
| `UT-0077-BM25-INDEPENDENT` | nDCG@10 0.99 but one forum achieves only 1.19 times BM25 Precision@6 | `KEEP_CHAMPION`; high nDCG cannot compensate |
| `UT-0077-ZERO-BASELINE` | Locked BM25 Precision@6 is zero on an eligible forum | `BLOCK_FOR_DATA_QUALITY`, not an infinite or automatic relative pass |
| `UT-0077-EIGHT-HEAD-MASK` | One batch has labels for seven heads and none for novelty | Compute seven masked losses, report the missing head and emit no synthetic novelty target |
| `UT-0077-HEAD-NOT-LABEL` | A neural head predicts grade 4 for an unjudged destination | The prediction cannot create an evaluation label or factual evidence |
| `UT-0077-WEIGHT-SIMPLEX` | Eight positive raw head weights and an all-zero BM25 field-weight vector | Normalise head weights to sum one within tolerance; reject the all-zero vector |
| `UT-0077-PARAMETER-PROTECTION` | Optuna attempts to suggest `ndcg_floor`, a Cedar rule or a label threshold | Reject before worker execution and record the protected name |
| `UT-0077-NUDGE-BOUNDARY` | A non-zero champion value is 100; candidates resolve to 102, the next representable value above 102 and 98 | Exactly 102 and 98 satisfy the relative bound; above 102 fails with `ERR-0077-NUDGE-BOUND` |
| `UT-0077-NUDGE-NORMALISED` | Raw weights appear within 2% but normalisation moves one final weight beyond it | Reject based on final resolved and normalised values, not raw suggestions |
| `UT-0077-NUDGE-ZERO` | The champion value is zero and no absolute nudge bound exists | `ERR-0077-NUDGE-ZERO-BOUND`; no epsilon is guessed |
| `UT-0077-NUDGE-DISCRETE` | Ordered counts move one and two adjacent choices; an unordered activation changes | One ordered choice is nudge-eligible; two choices and the unordered change are diagnostic only |
| `UT-0077-NO-PYTHON-PRODUCTION` | Check the exact admitted spec 0104 host and helpers. Plant Python job policy, a live ranker, an embedded objective or a custom spaCy factory outside that contract | The admitted host can run. Every planted path is refused before work |
| `UT-0077-RRF-DEDUP` | The same destination occurs in BM25, dense and sparse lists at different ranks | Fuse one destination identity using each eligible rank; never emit duplicate destinations |
| `UT-0077-BGE-CHANNELS-REQUIRED` | Production candidates omit the active dense, BGE-M3 sparse or BGE-M3 multi-vector channel in turn, set its weight to zero or omit its candidate count; a diagnostic ablation does the same | Every production variant fails with `ERR-0077-BGE-CHANNEL-MISSING`; the ablation may produce diagnostic evidence but cannot become champion |
| `UT-0077-PAID-DENSE-EXCEPTION` | Submit an admitted paid dense candidate with local sparse, multi-vector and rollback evidence; then remove each safeguard and ask Optuna to change the provider identity | The complete candidate remains eligible for weekly evaluation; every omission fails with `ERR-0077-PAID-DENSE-ADMISSION`, and the identity suggestion is refused |
| `UT-0077-DIVERSITY-IRRELEVANT` | A diverse but grade-0 destination would displace a relevant result | Relevance gate refuses it regardless of MMR or xQuAD score |
| `UT-0077-ABSTENTION-DENOMINATOR` | Only four results pass abstention at Precision@6 | Evaluate four returned plus two empty slots under spec 0057; do not change denominator to four |
| `UT-0077-DUPLICATE-IDEMPOTENT` | Identical trial request delivered twice | One training side effect and the same result receipt for both deliveries |
| `UT-0077-DUPLICATE-CONFLICT` | Same idempotency key with a different manifest digest | Quarantine and block; never reuse the first result |
| `UT-0077-CONTROL-CAS` | Two activations name one generation. Replay after an A to B to A cycle and deliver a late rollback | At most one compare-and-set succeeds. A duplicate returns its original receipt. An old unused decision fails even when the digest matches |
| `UT-0077-ARTEFACT-ATOMICITY` | New weights with old calibration in one attempted reader snapshot | Reader sees no mixed set; activation fails completeness verification |
| `UT-0077-ROLLBACK-COMPLETE` | Rollback restores model weights but omits spaCy pipeline and calibration | Rollback is unverified and cannot report success |
| `UT-0077-MODEL-PICKLE-REFUSAL` | Production loader receives `pytorch_model.bin` or a remote-code request | Refuse before deserialisation or network access |
| `PRP-0077-CONTROLLER-SAFETY` | Generate arbitrary metric vectors, labels and artefact states | No reachable state emits `PROMOTE` while any mandatory predicate fails |
| `PRP-0077-NUDGE-SAFETY` | Generate non-zero, zero, negative, normalised, ordered and unordered parameter pairs | No production-eligible result exceeds its admitted nudge contract, crosses to zero or disables a required component |
| `PRP-0077-IDEMPOTENCY` | Generate duplicates, reordering across keys and crashes around result persistence | No duplicate externally visible effect; per-key order and digest binding remain intact |

### BDD and integration scenarios

| Scenario | Given/When/Then |
|---|---|
| `BDD-0077-PROMOTE-VERIFIED` | Given a frozen champion and a weekly-tier challenger passing every gate, when signed evaluation authorises promotion, then exactly the challenger manifest becomes active and the receipt binds both digests |
| `BDD-0077-KEEP-BELOW-FLOOR` | Given a champion below 0.95 and a better challenger at 0.949, when weekly evaluation completes, then the outcome is `KEEP_CHAMPION`, neither is released and the floor failure remains visible |
| `BDD-0077-TRAINING-NO-PROMOTION` | Given training nDCG improves to 1.0 but held-out evidence is absent, when a challenger requests activation, then promotion is refused |
| `BDD-0077-EXPLORATION-NO-PROMOTION` | Given a broad-exploration trial outside the 2% trust region with excellent development metrics, when it requests promotion, then it is diagnostic only and receives no activation |
| `BDD-0077-SAFETY-DISABLE` | Given an active candidate with a newly verified permission or source-integrity failure, when protective control runs, then the affected capability is disabled or quarantined immediately despite the optimisation no-off rule. In a separate case retain a valid signed monthly integrity report for a permission fault in serving generation G1. Delay its delivery until the same manifest is active at G2 after ordinary admission and independent verification of current source eligibility. Treat G1 as permitted history. Its report cannot disable or quarantine G2, pause its eligible work or authorise rollback. Preserve the current protections and require the current scoped fault check. In the control a newly verified fault on G2 invokes the existing immediate protection. The integrity report can request control and does not approve it. |
| `BDD-0077-DATA-QUALITY-PRECEDENCE` | Given otherwise passing metrics and an unjudged returned candidate, when control runs, then the outcome is `BLOCK_FOR_DATA_QUALITY` and no activation request is emitted |
| `BDD-0077-HELDOUT-NO-LEAK` | Given weekly hidden labels, when the Optuna study resumes, then its records contain no hidden per-query labels and no suggestion used them |
| `BDD-0077-QUEUE-REPLAY` | Given a crash after a worker side effect but before its receipt, when an approved replay occurs, then reconciliation returns the original authorised effect only when its exact commitment is proved. An unresolved outcome holds without repeating it. |
| `BDD-0077-DEAD-LETTER` | Given an unknown event version or bad payload digest, when a consumer reads it, then it is not executed and enters a diagnosable dead-letter record |
| `BDD-0077-RESOURCE-NONOVERLAP` | Given production serving near its admitted envelope, when nightly training requests admission, then training waits or is refused without degrading serving or reducing promotion evidence. In a separate branch, admit serving with training and provide valid synthetic measurements for every pair among serving, training and promotion evaluation on the same host. Start the serving and training pair, then request promotion evaluation for a different otherwise eligible candidate. Keep the current budget and all other checks valid. Hold the three-way overlap before a claim, load or result. When training reaches its safe point and its promise is released, the measured serving and evaluation pair may proceed under current admission. |
| `BDD-0077-AUTOMATIC-ROLLBACK` | Given a verified critical regression after activation, when rollback runs, then the complete prior set is restored, verified and reconciled before new admission resumes |
| `BDD-0077-ROLLBACK-FAILURE` | Given one restored digest does not match, when rollback verification runs, then the candidate is quarantined and release and activation remain blocked. In a separate run roll back from synthetic set B to A, then legitimately activate B again. Start a new authorised rollback from the later B generation to the same A. Substitute the first rollback's authentic success receipt and checks, keeping both manifest digests identical. The current effect remains unverified, with no current `ROLLED_BACK` receipt or resumed admission. An independent control run with checks for the exact current decision succeeds once. A duplicate of the earlier decision itself can return only its authorised original receipt. |
| `BDD-0077-SPACY-LANGUAGE` | Given a language with tokenisation but no admitted NER pipeline, when NER features are requested, then the declared unsupported outcome occurs and no tokeniser-only result is labelled as NER support |
| `BDD-0077-SOURCE-GATE` | Given a high-scoring destination whose source is stale, unauthorised or partially read, when ranking runs, then it is ineligible regardless of every model score |
| `BDD-0077-DESTINATION-COUNT` | Given abundant eligible destinations and an aggressive diversity candidate, when final selection runs, then spec 0027 still returns only 3–6 suitable internal destinations |
| `BDD-0077-ABLATION-NO-PROMOTION` | Given a diagnostic trial with sparse retrieval disabled and better development metrics, when weekly control evaluates it, then it cannot receive `PROMOTE` and the complete three-channel champion remains active |
| `BDD-0077-PAID-DENSE-PROMOTION` | Given an owner-authorised exact Bedrock candidate with spec 0025 admission, local sparse and multi-vector channels, a verified local rollback set and passing weekly evidence, when the controller emits `PROMOTE`, then only the dense generation and dense identity change; given any missing safeguard, then activation is refused with `ERR-0077-PAID-DENSE-ADMISSION` |
| `IT-0077-QUEUE-RECOVERY` | Stop the scheduler, relay, database or sidecar at claim, checkpoint, staged artefact, result and activation boundaries. Recover from immutable inputs with one effect, current receipts and no hidden labels in the study or outbox. |
| `IT-0077-MODEL-CONVERSION-PARITY` | Convert the hash-verified source in quarantine and compare fixed publisher/derived vectors and reranker scores within spec 0024 tolerances; production accepts only the verified data-only export |
| `E2E-0077-CHAMPION-CHALLENGER` | Freeze data, train and screen a candidate, evaluate weekly, atomically promote, detect a planted critical regression and verify automatic restoration of the complete prior set |

In BDD-0077-PROMOTE-VERIFIED, let an evaluation attempt time out
with an authentic in-budget challenger peak measurement staged.
Use an eligible repeat on the same admitted profile with identical
model and index digests. The repeat has complete paired queries
and a passing paired p95 comparison. Keep the other promotion
predicates satisfied. Withhold only its challenger peak measurement
and offer the timed-out attempt's observation.
The original observation retains its own scope. Hold promotion with
an incomplete resource record and KEEP_CHAMPION. Keep the paired
p95 result separate from the resource hold. Leave the active pointer
and retained rollback set unchanged. In a separate control supply
the repeat's own verified paired peak measurements within the
existing limit and enforce the other promotion checks as written.
This fixture uses the existing repeat cap and attempt fences.

In IT-0077-QUEUE-RECOVERY, stage a valid synthetic training result.
Deliver it after cancellation commits, then in a separate run
after the original lease loses its fence to a reconciled retry.
Use the same staged bytes in both runs. Neither publishes that
late result or starts screening. The eligible replacement can
commit its own verified result once in the second run. Where
staging is visible but publication cannot be reconciled, hold the
effect and start no
replacement. In a separate run, commit the result and lose the
reply. Reconcile and return the same receipt once per request
without rerunning training. Reuse the result only while its source
is eligible. Repeat with a 0028 correction committed before replay
and the 0053 label ineligible on current authority. Keep only the
permitted history and refuse further artefact use. In every
branch, assert that the unchanged eligible champion digest and
generation, rollback set and verification records are identical
before and after recovery. No training path emits a new control
decision or treats an old rollback receipt as fresh verification.

In E2E-0077-CHAMPION-CHALLENGER, commit a valid weekly evaluation
and its signed PROMOTE decision, then lose the reply before the
activation claim. Reconcile the original evaluation under 0057.
First withdraw guest visibility from one synthetic source used only
in that evaluation. Keep the frozen source bytes, model digests,
protocol, expected champion generation and caller's control grant
fixed. Recheck current source eligibility on read-back and before
activation. Give only the permitted historical or held outcome;
withhold protected evidence from reuse. Offer the authentic earlier
PROMOTE decision under its unchanged effect key. Refuse it before
the activation compare-and-set and leave the still-eligible champion
and rollback set unchanged. The historical decision keeps its own
scope. Do not rerun judging, draw another allocation or disclose
hidden rows. The separate current-rights control recovers the
original result and reaches only the normal activation checks.

### Evaluation edge cases

The shared metric fixtures in spec 0057 include all-equal relevance, tied
scores, fewer than ten results, zero ideal gain, unjudged candidates,
duplicate destinations, missing forum strata, zero BM25 baseline,
abstention and repeated runs with measurement noise. This specification
adds crashes, stale champion pointers, duplicate messages, forbidden
parameters, missing head labels, unsupported languages and incomplete
artefact sets. Every case asserts both the visible outcome and the absence
of forbidden side effects.

Promotion and rollback evidence feeds spec 0057 and spec 0064. A
protected push judgement checks the synthetic fleet but authorises
no promotion. Spec 0005 alone validates release approval. A passing
promotion cannot replace the ranking-method floor or long lane.

## Release evidence

A release candidate retains the dependency lock, manifests, study
history, dataset and label-policy versions, paired champion/challenger
results, metric and resource reports, controller receipt, activation or
rollback receipt, provenance and required legal review. Missing evidence
blocks release. No document or screen may describe 0.95 as achieved until
the applicable signed evaluation actually measures it.

## Data

- Candidate manifests, with the fields listed under "Immutable artefact and
  dependency lock", held as hash-addressed sets with a SHA-256 digest.
- Study storage: suggested and resolved parameters, pruning reasons,
  failures, artefact digests and random seeds. It holds no hidden per-query
  labels.
- Dataset and label-policy digests, frozen by `rank.dataset.freeze`.
- Controller receipts and activation or rollback receipts, signed, binding
  both manifest digests.
- Job, outbox and dead-letter records in the shared PostgreSQL runtime of
  spec 0019, scoped to one site by its row-level security context. They hold
  opaque references, digests and safe status only.
- Resource admission records and release evidence, retained per the
  "Release evidence" section.
- Labels and metric definitions belong to spec 0057, not to this spec.

## Settings

The spec names no runtime setting key. The values below are its registered
and protected bounds. The text does not say when a change takes effect, so
"next candidate manifest" is (proposed). Protected values are not
suggestible by Optuna.

| Setting | Type | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| Nudge trust region | relative fraction | 2% | fixed by owner; zero champion needs an absolute bound | next candidate manifest (proposed) |
| nDCG@10 promotion floor (protected) | ratio | 0.95 | fixed | next candidate manifest (proposed) |
| Per-forum BM25 Precision@6 margin (protected) | ratio | 20% | fixed | next candidate manifest (proposed) |
| MRR and Recall@40 largest fall | absolute | 0.005 | fixed | next candidate manifest (proposed) |
| Paired-bootstrap resamples | whole number | 10,000 | fixed; one-sided 95% | next candidate manifest (proposed) |
| Hidden layers, width, dropout | integer, choice, float | none admitted | 1 to 4; 64, 128, 256 or 512; 0.0 to 0.5 | next candidate manifest (proposed) |
| Learning rate, weight decay, epochs | float, float, integer | none admitted | 0.00001 to 0.01 (log); 0.0 to 0.1; 1 to 30 | next candidate manifest (proposed) |
| Head weight, each of eight | float | none admitted | 0.001 to 100 (log), then normalised | next candidate manifest (proposed) |
| BM25 `k1`, `b` | float | none admitted | 0.5 to 2.5; 0.0 to 1.0 | next candidate manifest (proposed) |
| Candidate count per retrieval channel | integer | none admitted; must be chosen explicitly | 20 to 500 | next candidate manifest (proposed) |
| Cross-encoder input candidate count | integer | 50 (initial) | 10 to 100, within unique fused candidates | next candidate manifest (proposed) |
| RRF rank constant | integer | none admitted | 1 to 120 | next candidate manifest (proposed) |
| Diversity relevance share | float | 0.85 (starting value, not a measured optimum) | 0.75 to 0.95 | next candidate manifest (proposed) |
| Calibration temperature | float | none admitted | 0.05 to 20.0 (log) | next candidate manifest (proposed) |
| Abstention operating threshold | float | none admitted | 0.50 to 0.99 | next candidate manifest (proposed) |
| Worker concurrency | integer | none admitted | 1 to 8, within admission | next candidate manifest (proposed) |

The weight, channel and field-weight rules (normalised, all-zero invalid,
positive in production) are in the registry table above. Evaluation tier
sizes are in the table under "Dataset separation and evaluation tiers".
No setting lets a worker approve its own candidate, lower a gate, or select
a paid dense provider or model.

## Failure codes

This spec cites these codes, each defined under "Error contract":
`ERR-0077-ARTEFACT-DIGEST`, `ERR-0077-UNSAFE-MODEL-FORMAT`,
`ERR-0077-PARAMETER-BOUND`, `ERR-0077-NUDGE-BOUND`,
`ERR-0077-NUDGE-ZERO-BOUND`, `ERR-0077-BGE-CHANNEL-MISSING`,
`ERR-0077-PAID-DENSE-ADMISSION`, `ERR-0077-LANGUAGE-UNSUPPORTED`,
`ERR-0077-INCOMPLETE-SOURCE`, `ERR-0077-DUPLICATE-EXECUTION`,
`ERR-0077-STALE-CHAMPION`, `ERR-0077-EVALUATION-UNJUDGED`,
`ERR-0077-BM25-ZERO`, `ERR-0077-RESOURCE-REFUSED` and
`ERR-0077-ROLLBACK-UNVERIFIED`. Names in the `ERR-0077-` form are owned by
this spec under spec 0076. The spec has no numeric `SUT-` range. Numeric
codes are reserved under a later decision (see QR-220).

## Security and privacy

- Python packages cannot change Cedar policy, row-level security, signing
  keys, source-integrity rules, evaluation labels, release thresholds,
  protected Optuna bounds, the controller or release gates. A worker
  returns bounded results and cannot approve its own candidate.
- Production never loads pickle-capable files or remote model code.
  Conversion runs in a quarantined, network-disabled, read-only-input job
  with no secrets and no `trust_remote_code`.
- Artefact digests are checked before opening. A mismatch quarantines the
  artefact (`ERR-0077-ARTEFACT-DIGEST`).
- Hidden weekly labels never reach Optuna, queue rows, receipts, logs,
  checkpoints or dead letters. Only the protected evaluator reads them.
- Site jobs run under their own site's row-level security context, with no
  cross-site production query.
- Neural scores are ranking features, never factual evidence, and cannot
  create labels.
- A paid dense model enters only through spec 0025's owner-authorised
  admission and a full weekly `PROMOTE`. A local rollback set stays.
- Controller decisions and receipts are signed. A queue record or event
  never stands in for signed evidence.
- Patent clearance is separate (`LEG-0077-RANKING-PATENTS`).

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| REQ-0077-NO-PYTHON-PRODUCTION | Production ranking MUST NOT run Python job policy, a live ranker, an embedded objective or a custom spaCy factory outside the admitted spec 0104 host. | UT-0077-NO-PYTHON-PRODUCTION |
| REQ-0077-MODEL-FORMAT | Production MUST refuse pickle-capable weights and remote model code and load only verified data-only exports. | UT-0077-MODEL-PICKLE-REFUSAL, IT-0077-MODEL-CONVERSION-PARITY |
| REQ-0077-HEADS | The eight heads MUST use masked losses, with normalised positive weights, and a head output MUST NOT create a label or evidence. | UT-0077-EIGHT-HEAD-MASK, UT-0077-HEAD-NOT-LABEL, UT-0077-WEIGHT-SIMPLEX |
| REQ-0077-REGISTRY | Optuna MUST suggest only registered, in-range, finite, unprotected parameters. | UT-0077-PARAMETER-PROTECTION |
| REQ-0077-NUDGE | A production candidate MUST stay inside the 2% trust region after normalisation, move one ordered choice at most, and have an explicit absolute bound if the champion value is zero. | UT-0077-NUDGE-BOUNDARY, UT-0077-NUDGE-NORMALISED, UT-0077-NUDGE-ZERO, UT-0077-NUDGE-DISCRETE, PRP-0077-NUDGE-SAFETY |
| REQ-0077-BGE-CHANNELS | A production champion MUST carry active dense, sparse and multi-vector channels with positive weights and counts, and an ablation MUST NOT be promoted. | UT-0077-BGE-CHANNELS-REQUIRED, BDD-0077-ABLATION-NO-PROMOTION |
| REQ-0077-PAID-DENSE | A paid dense candidate MUST have spec 0025 admission, owner authorisation, a local rollback set and weekly `PROMOTE`, and Optuna MUST NOT choose it. | UT-0077-PAID-DENSE-EXCEPTION, BDD-0077-PAID-DENSE-PROMOTION |
| REQ-0077-SOURCE-GATE | A stale, unauthorised or partly read source MUST be ineligible whatever its score. | BDD-0077-SOURCE-GATE |
| REQ-0077-RELEVANCE-FIRST | Diversity methods MUST NOT admit an irrelevant destination or change the 3 to 6 destination rule. | UT-0077-DIVERSITY-IRRELEVANT, BDD-0077-DESTINATION-COUNT |
| REQ-0077-FLOORS | A challenger MUST meet the 0.95 nDCG floor and the per-forum BM25 margin independently, and a zero BM25 baseline MUST block. | UT-0077-NDCG-FLOOR-BOUNDARY, UT-0077-BM25-INDEPENDENT, UT-0077-ZERO-BASELINE, BDD-0077-KEEP-BELOW-FLOOR |
| REQ-0077-WEEKLY-ONLY | Only weekly-tier evidence MAY authorise promotion, and training or exploration results MUST NOT. | BDD-0077-TRAINING-NO-PROMOTION, BDD-0077-EXPLORATION-NO-PROMOTION |
| REQ-0077-CONTROLLER | The controller MUST give no `PROMOTE` while a mandatory predicate fails, and data-quality failure MUST take precedence. | PRP-0077-CONTROLLER-SAFETY, BDD-0077-DATA-QUALITY-PRECEDENCE, UT-0077-ABSTENTION-DENOMINATOR |
| REQ-0077-HELDOUT | Hidden labels MUST NOT enter Optuna studies or suggestions. | BDD-0077-HELDOUT-NO-LEAK |
| REQ-0077-IDEMPOTENT | A repeated key MUST return the original result, and a conflicting payload for it MUST be quarantined. | UT-0077-DUPLICATE-IDEMPOTENT, UT-0077-DUPLICATE-CONFLICT, PRP-0077-IDEMPOTENCY, BDD-0077-QUEUE-REPLAY |
| REQ-0077-RECOVERY | Each stop at a claim, checkpoint, staging, result or activation boundary MUST recover with one effect, and bad events MUST go to a dead letter. | IT-0077-QUEUE-RECOVERY, BDD-0077-DEAD-LETTER |
| REQ-0077-ATOMIC-ACTIVATION | Activation MUST be one compare-and-set on digest and generation, readers MUST NOT see a mixed set, and a late decision MUST fail. | UT-0077-CONTROL-CAS, UT-0077-ARTEFACT-ATOMICITY, BDD-0077-PROMOTE-VERIFIED |
| REQ-0077-ROLLBACK | Rollback MUST restore one complete verified prior set and MUST NOT report success unless every check passes. | UT-0077-ROLLBACK-COMPLETE, BDD-0077-AUTOMATIC-ROLLBACK, BDD-0077-ROLLBACK-FAILURE, E2E-0077-CHAMPION-CHALLENGER |
| REQ-0077-SAFETY-DISABLE | A verified permission or source-integrity failure MUST disable or quarantine at once despite the no-off rule. | BDD-0077-SAFETY-DISABLE |
| REQ-0077-RESOURCE | A candidate over budget MUST NOT be promoted, and training MUST NOT overlap serving without a measured record. | BDD-0077-RESOURCE-NONOVERLAP |
| REQ-0077-SPACY | A language without the needed trained pipeline MUST give the declared unsupported outcome. | BDD-0077-SPACY-LANGUAGE |
| REQ-0077-FUSION | Fusion MUST give one entry per destination identity. | UT-0077-RRF-DEDUP |

## How it is tested

Designed, not run.

The tests are those named under "Verification design": unit and property
tests (`UT-0077`, `PRP-0077`), BDD and integration scenarios (`BDD-0077`,
`IT-0077`), one end-to-end journey (`E2E-0077-CHAMPION-CHALLENGER`) and the
evaluation edge cases shared with spec 0057. Each requirement above names
its tests. A passing result needs the signed evidence listed under
"Release evidence". No document may claim 0.95 as achieved until a signed
evaluation measures it.

## Changes from the source

The decision DR-023 and the question QR-007 record that the eight heads of
this spec are still to be reconciled with the cross-encoder plus
learning-to-rank design. QR-084 and QR-094 record the open alignment of
worker and runtime dependencies with ADR-0021 and spec 0019, and the limits
and fixtures for the job kinds. The text itself says Rust owns the objective
and decisions, custom Python is not a fallback, and the 2% trust region is an
owner-approved design bound, not a measured optimum. It records no other
change from an earlier source.

[doi-cg]: https://doi.org/10.1145/582415.582418
[bge]: https://arxiv.org/abs/2402.03216
[doi-optuna]: https://doi.org/10.1145/3292500.3330701
