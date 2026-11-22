# ADR-0020: Version 1 documentation and architecture baseline

Superseded in part by [ADR-0021](0021-version-1-product-stack-and-scope.md)
(background processing is PostgreSQL outbox and our own Axum/Tokio
scheduler, not Kafka or Airflow).

Decided by the owner on 18 November 2026 after the senior engineering
review of the product vision, software delivery lifecycle, feature
register, specifications and Forgejo decision records. Revised on
19 November 2026 to require packaged Python libraries without first-party
Python production code. This record is the decision ledger entry for that
review. It supplements ADR-0010 and replaces the conflicting decisions
listed below.

This is a documentation and architecture decision. It does not authorise
application code, executable tests, build files, deployment
configuration or infrastructure changes.

## Decision hierarchy

1. An owner decision takes effect in the repository through a dated ADR.
   A later ADR must name every earlier decision it replaces.
2. AGENTS.md holds delivery rules. Numbered specifications hold normative
   product behaviour, interfaces, verification and operating procedures.
   The roadmap holds phase order and completion outcomes. None may
   contradict another; a contradiction is a blocking documentation
   defect, not a choice for an implementer.
3. Forgejo milestones and tasks are derived execution and evidence
   records. They have no independent requirements authority. A Forgejo
   comment that records an owner answer remains historical evidence until
   it is carried into an ADR.
4. Chat is not an enduring project record. A later owner answer enters an
   ADR first, then propagates to rules, specifications, roadmap and
   derived tasks before implementation relies on it.

A reference to a superseded decision is valid only as dated history and
must name its successor. Generated indexes and traceability reports must
resolve to the successor.

## Version 1 outcome floor

Version 1 retains every approved feature in `docs/specs/features.md` and
the expanded scope in spec 0064. The standalone numerical method,
capability and variant quotas are removed. The release floor is instead:

- every approved feature has one complete owning specification and one
  derived build task;
- every declared supported platform, language and input variant is
  implemented and accepted with feature-specific evidence;
- every required check and required review passes;
- ranking methods ship only when held-out product evidence justifies
  them;
- the mandatory ranking, link-suitability, safety, calibration,
  performance, signing and owner-approval gates pass.

No method, wrapper, configuration value or alias earns release credit
merely by increasing a count. Spec 0076 owns design completeness and
traceability. Spec 0064 owns release-scope qualification.

## Languages and execution

Rust remains the default for product services and substantive tools.
First-party Go is banned. A pinned third-party program written in Go is
permitted when the normal dependency, licence, hash and security checks
pass. There is no assumed Go-to-Rust migration: an inventory must prove
that a first-party Go component exists before a port is specified.

Bounded operational shell is permitted. It must use strict failure
options, fixed argument arrays and explicit environment and working
directories. It must not use `eval`, construct commands from untrusted
text, contain business logic or parse structured data. ShellCheck,
shfmt, success-path and failure-path tests are mandatory. Product
operations exposed to sites or ordinary administrators remain typed
operations, not shell command entry points.

The product must use approved Python packages and libraries for BGE,
spaCy, Optuna and Airflow rather than reimplementing their algorithms in
Rust. The owner's 19 November 2026 decision prohibits all first-party
Python production code, including hand-written or generated DAGs,
operators, hooks, sensors, callables, Optuna objectives, training or
serving adapters, evaluation adapters and custom spaCy components.

Rust owns every first-party production integration and invokes pinned
packages through an admitted embedding bridge, supported packaged
executable or packaged service interface. Airflow DAGs are declarative
data consumed by a pinned DAG-factory package and packaged operators.
First-party Python is limited to explicitly permitted non-production
tests and development checks. Python packages may propose typed values
but may not autonomously change permissions, source integrity,
evaluation labels, release thresholds, protected configuration or
release control. A capability that needs custom Python is blocked until
a supported packaged or Rust-hosted route is admitted.

## Delivery and assurance

Each agent uses an isolated disposable clone with local `main`, no shared
unfinished work, rebase before integration and one serial integration
queue. At most ten agents run at once. Force push is forbidden. Changes
are coherent and reviewable; there is no universal 300-line commit cap.
Generated artefacts remain bound to their sources by hash.

Every required check must pass. Regression-introducing changes are fixed
or reverted before unrelated work. Other defects are recorded, owned and
prioritised openly rather than hidden or mechanically expanding the
current change.

Eight independent reviews and 100% line, branch, MC/DC and
killable-mutant coverage apply only to this strict core:

1. signing and message verification;
2. authentication and authorisation;
3. tenant isolation and row-level security;
4. secret storage and key handling;
5. moderator approval, public writes and undo;
6. destructive backup, restore and delete operations;
7. immutable model/configuration artefact identity and atomic champion
   transitions;
8. release-gate enforcement;
9. evaluation-label and threshold integrity; and
10. verified rollback.

An equivalent mutant requires a written proof and independent review and
is excluded from the killable-mutant denominator. Ranking algorithms,
Optuna, model training, spaCy pipelines, ordinary ingestion, analytics,
read-only reporting and presentation are outside the strict core unless
the component also performs one of the included operations. Ordinary
work uses risk-appropriate tests, independent review, relevant mutation
analysis and scoped formal properties. Formal tools are selected per
property, with their assumptions and trusted computing base recorded.

## Background processing

> Superseded in part by [ADR-0021](0021-version-1-product-stack-and-scope.md)
> (background processing is PostgreSQL outbox and our own Axum/Tokio
> scheduler, not Kafka or Airflow).

Airflow replaces Apalis. Kafka and Airflow own background event delivery
and orchestration across the platform. Rust/Tokio provides all first-party
product-facing integration and workers. Approved Python packages provide
their own library execution through the admitted interfaces; the project
writes no first-party Python production jobs or adapters.

Each job class has one accountable orchestrator and one durable execution
path. The PostgreSQL durable-job and product-outbox path is retired only
through a migration with inventory, compatibility, drain, replay,
reconciliation, cutover and rollback evidence. PostgreSQL remains
authoritative business storage and may hold approved component metadata.
The XenForo add-on's change-feed table is a site integration contract,
not a background-job executor, and is governed separately by spec 0021.

Delivery is at least once unless a stronger property is proved for the
whole route. Consumers are idempotent. Versioned event and DAG contracts
define keys, ordering, retries, dead letters, backpressure, retention,
replay, recovery, reconciliation and audit. No component may claim
end-to-end exactly-once delivery from a broker feature alone.

## Retrieval, language and optimisation

The target retrieval stack is:

- `BAAI/bge-m3`, revision
  `5617a9f61b028005a4858fdac845db406aefb181`, MIT, for approved dense,
  sparse and multi-vector retrieval modes;
- `BAAI/bge-reranker-v2-m3`, revision
  `953dc6f6f85a1b2dbfca4c34a2796e7dde08d41e`, Apache-2.0, for
  cross-encoder reranking.

Qwen3-Embedding-0.6B, potion-retrieval-32M and
`gte-reranker-modernbert-base` are removed from the target production
direction. Historical records may retain them as superseded choices.
Mandating this architecture does not waive any release gate.

On 19 November 2026 the owner approved a paid production exception to
BGE-M3 dense retrieval. An exact AWS Bedrock embedding model admitted
under spec 0025 and promoted under spec 0077 may supply the complete dense
channel in one site's production champion. This is the sole production
exception to BGE-M3 dense; it does not replace BM25, BGE-M3 sparse or
multi-vector retrieval, the BGE cross-encoder, spaCy or any protected
gate. The first production champion remains the complete local stack. A
complete verified local champion and compatible dense index remain
available for rollback and spending-cap restoration.

Paid provider and model identities are not Optuna parameters or nudges.
Activation requires owner authorisation, recorded model admission, the
diagnostic trial and full weekly promotion evidence. An unadmitted
identity, missing local rollback set or failed protected gate refuses
activation. Paid and BGE-M3 dense vectors never share a dense-index
generation, while BGE-M3 sparse and multi-vector generations remain
present and unchanged.

spaCy is the broad NLP platform for tokenisation, sentence segmentation,
lemmatisation, part-of-speech and dependency features, named-entity
recognition, entity linking, classification, rule-based matching, approved
packaged components and declaratively custom-trained models. Each output
has one authoritative producer and a typed, versioned contract. spaCy does
not become a second
embedding-retrieval or cross-encoder stack and does not control final
ranking, evaluation labels, thresholds or release decisions. Language
support is declared per feature, component and trained pipeline; broad
tokeniser support is not evidence of uniform trained-model support.

Spec 0077 owns autonomous ranking optimisation, including typed Optuna
parameters, eight neural prediction heads, champion/challenger control,
immutable artefacts, Airflow DAGs, Kafka recovery and verified rollback.

> Superseded in part by [ADR-0021](0021-version-1-product-stack-and-scope.md)
> (background processing is PostgreSQL outbox and our own Axum/Tokio
> scheduler, not Kafka or Airflow).

On 19 November 2026 the owner approved bounded autotuning with a 2%
production-eligible trust region around the current champion for eligible
continuous parameters and one adjacent ordered choice for discrete
parameters. This is a design bound, not a measured optimum. Broad Optuna
exploration remains diagnostic and cannot be promoted directly. Optimisation
cannot turn off features, required channels or positive weights. Immediate
safety, permission, source-integrity, moderation, legal and rollback disable
or quarantine actions remain mandatory and are not optimisation decisions.
The former independent tuner and direct settings writer remain superseded.

## Mandatory ranking and suitability evidence

nDCG@10 at least 0.95 is a mandatory release gate, not an aspirational
target or a measured result. One is the theoretical maximum under the
specified metric. `KEEP_CHAMPION` can preserve the operating champion but
cannot release a candidate or champion below this floor.

The earlier BM25 decision remains an independent gate: on every eligible
test forum, challenger Precision@6 must be at least 1.20 times the locked
BM25 Precision@6, using identical judged links and output rules. Neither
gate waives the other. Link suitability and severe relevance failures
are measured separately from ranking order.

The evaluation tiers are:

| Tier | Threads | Queries |
|---|---:|---:|
| Local diagnostic | 16 | 32 |
| Push diagnostic | 48 | 96 |
| Extended integration | 256 | 512 |
| Nightly Optuna | 512 | 1,024 |
| Weekly promotion | 2,000 | 4,000 |
| Monthly integrity | All eligible threads | Separate integrity reconciliation |

Samples are stratified across forums, languages, source types,
publication periods and thread lengths. Local and push runs are
diagnostic; they cannot establish promotion evidence. The initial push
nDCG@10, MRR and Recall@40 regression tolerance is 0.005, subject to
measured baseline variability, and cannot waive the absolute floor or
substitute for promotion-tier evidence.

Spec 0057 owns metric definitions, label governance and release
assessment. Spec 0077 owns optimisation and controller execution.

## Provenance and clean-room boundaries

Every consequential choice records its sources, versions, licences,
alternatives and rationale. Each method is classified as original design,
public standard or published algorithm, licensed dependency, permitted
interoperability observation, or strict clean-room reimplementation.

Researcher/implementer separation, access records and a controlled
specification transfer are required only for strict clean-room
reimplementation. Licensed reuse and implementation from public
standards must not be described as strict clean-room work. Patent
searches, freedom-to-operate analysis and qualified legal review are
separate gates. No engineering document may assert that they are
complete without their actual evidence.

## Explicit supersession map

| Earlier decision | Replaced or narrowed rule | Successor |
|---|---|---|
| Forgejo #357 V2 and V5; Forgejo #185 numerical goals | Fixed 60,000/6,000 and related method/capability/variant quantity goals; this also supersedes the older 32,000/3,200 repository quotas | Outcome floor in this ADR and spec 0064 |
| #357 V40, only where it supports a numerical capability quota | Shipped abilities remain the unit of capability evidence, but no standalone total is required | Spec 0076 and spec 0064 |
| #357 V10 and V19; ADR-0017 and ADR-0019 Rust-only/no-shell operational rules | Rust-only operations and mandatory shell removal are replaced by the Rust, bounded-shell and packaged-Python policy above | This ADR; revised ADR-0019 |
| The 18 November 2026 Python rule in this ADR, ADR-0001 and ADR-0019 | First-party Python production DAGs, objectives, adapters and custom components are prohibited; maintained Python packages remain mandatory for their specialist roles | Packaged-Python rule above; revised ADR-0001 and ADR-0019 |
| #357 V14 and the 300-line rule in #357's job card | Generated files still require source binding, but commits have no universal line cap | Coherent-change rule in this ADR and AGENTS.md |
| #357 V17 and V68 | Retained, not replaced: isolated clones, local `main`, serial integration and at most ten agents | AGENTS.md section 8.2 |
| #357 V18 | Fast, pre-integration, nightly and release evidence is retained; a detected regression blocks release, while unrelated defects receive visible prioritised ownership rather than automatic inclusion in the current change | AGENTS.md sections 6 to 8 |
| #357 V29 and V70, and the eight-review clause in #357's job card | Equivalent-mutant proof and the eight review disciplines are retained for the strict core; universal eight reviews and universal MC/DC/mutant coverage are narrowed | Assurance policy above and spec 0076 |
| #357 V49 and V64 | No first-party Go is permitted; no port is invented where no first-party Go artefact exists | Language policy above |
| #357 V52 | Any older wording that made quantity the principal release quality bar is replaced by outcome, ranking, safety and performance evidence | This ADR; specs 0057, 0064 and 0076 |
| #357 V53 and V54 | Retained and made precise: BM25 is the locked baseline and Precision@6 must improve by at least 20% on every eligible test forum | Spec 0057 |
| #357 V55 | A fixed agent-judged set is insufficient alone; versioned human-adjudicated labels, held-out evaluation, stratification, power and integrity controls are mandatory | Spec 0057 |
| #357 V57 | Specifications do not win by silence or precedence alone; all conflicting artefacts must be reconciled through the decision hierarchy and traceability checks | Spec 0076 |
| #357 V58 | Model selection is no longer left to the lab alone; the BGE stack and broad spaCy direction are mandated, subject to release evidence | This ADR; specs 0024, 0026 and 0077 |
| #357 V38 and V38a | The 12 GB Sutradhar allocation remains an admission constraint, not proof that the enlarged stack fits; measured resource admission and non-overlapping profiles are required | Spec 0077 and resource specifications |
| ADR-0010's count-based trade-off and check | Version 1 scope remains full; its release check changes from numerical counts to complete feature and outcome evidence | This ADR; spec 0064 |
| ADR-0012 model rows; specs 0013, 0022, 0023, 0024, 0025, 0026, 0027, 0039, 0041, 0042, 0052, 0053, 0056 and 0057 | Pure-Rust-only model execution, the previous Qwen/potion/gte model stack and Rust-only training are superseded where they conflict with the approved packaged-Python/BGE/spaCy direction; no first-party Python production code is allowed | This ADR and the named model, text, ranking and optimisation specifications |
| ADR-0002, ADR-0005, spec 0019 and other product job/outbox rules | PostgreSQL durable jobs and the product outbox are superseded as the target execution path, subject to the controlled migration above; spec 0021's site change feed is not removed | This ADR and spec 0019 |
| Apalis references and bounded autotuning in specs 0053 and 0057 | Apalis is replaced by Airflow; autonomous optimisation uses spec 0077's typed parameters, 2% production-eligible trust region and protected release control; no independent tuner or direct settings writer remains | Spec 0077 |
| The earlier all-production-BGE-M3-dense wording in this ADR and specs 0022, 0023, 0024, 0027 and 0077 | BGE-M3 remains the default dense route and sparse and multi-vector remain universally mandatory; one exact admitted Bedrock model may instead supply a site's dense channel under the paid production exception above | This ADR; specs 0022, 0023, 0024, 0025, 0027 and 0077 |

> Superseded in part by [ADR-0021](0021-version-1-product-stack-and-scope.md)
> (background processing is PostgreSQL outbox and our own Axum/Tokio
> scheduler, not Kafka or Airflow).

Any older statement covered by this map has no normative force. A
statement outside the map remains unless another ADR expressly replaces
it.

## Consequences

Implementation tasks must trace to populated specifications and cannot
infer missing behaviour from a feature title. Research and legal evidence
cannot be replaced by plausible prose. The enlarged data platform needs
measured admission on Dell and the owner's target computers; an
unproven memory or latency envelope blocks release rather than being
explained away.

## How this decision is checked

- The documentation checker rejects active numerical quota language,
  removed model names outside historical ADR context, Apalis as a target
  component, first-party Go requirements, unbounded shell rules,
  first-party Python production code or adapters, and deferred
  feature-design instructions.
- Spec 0076's traceability checks cover every feature, decision,
  requirement, task and test identifier.
- Spec 0077's nudge fixtures cover the exact 2% boundary, normalised
  values, zero-valued champions, ordered discrete choices, broad exploration
  and the safety-disable exception.
- Spec 0057's metric fixtures cover the absolute and relative ranking
  gates and their edge cases.
- Spec 0077's controller fixtures prove that no outcome can bypass a
  mandatory gate and that rollback restores a complete verified artefact
  set.
- Forgejo tasks link to the ADR and specifications from which they are
  derived. Dynamic execution status remains in Forgejo, not in these
  documents.
