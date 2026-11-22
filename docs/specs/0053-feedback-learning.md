# 0053: Feedback learning and label governance

This specification owns moderator-feedback semantics, label integrity,
training-data export, feedback-derived module-stage evidence and feedback
observability. It is
roadmap Phase 6. It does not own a second settings tuner, model-promotion
path or rollback controller.

Spec 0077 owns Optuna studies, neural and retrieval training, challenger
evaluation, champion activation and rollback. Spec 0057 owns evaluation
labels, metrics and protected release gates. Spec 0019 owns jobs and
the transactional outbox. Spec 0014 owns settings storage. Spec 0020 owns
human-requested setting changes and confirmation.

## Outcomes and boundaries

1. Moderator decisions become versioned, permission-checked learning
   records without exposing post text or moderator identity to training
   workers.
2. Feedback can improve ranking only through the single typed optimisation
   and champion/challenger route in spec 0077.
3. No weight moves through an independent autotuner or outside spec 0077's
   2% production-eligible trust region, no setting is activated merely
   because a training metric improved, and no feedback process enables
   automatic public linking.
4. Rust retains product policy, permissions, source integrity, moderator
   approval, data preparation and release authority. Approved Python
   packages perform model training and Optuna functions only through the
   admitted Rust-owned interfaces. No first-party Python production code
   or adapter is permitted.
5. A rejected, withdrawn, stale or invisible source cannot remain as
   positive evidence. Erasure and permission changes propagate to labels,
   datasets, candidate evaluations and derived artefacts.

The old independent bounded-autotuning algorithm, its one-setting-at-a-time
watch and its direct settings writer are superseded by ADR-0020 and spec
0077. The module-stage evidence below informs that single route; it does
not create a separate visibility, promotion or demotion controller.

## Terms

- **Decision**: approve, reject or remove (undo), with an optional reason
  code from spec 0028.
- **Feedback label**: the typed learning interpretation of a decision.
- **Evaluation label**: a governed relevance or usefulness grade under
  spec 0057. A production decision is not silently promoted to an
  evaluation label.
- **Training example**: a feedback label joined to immutable source and
  destination content versions, model keys and permitted features.
- **Module mode**: `shadow` or `review`. There is no automatic public-write
  mode in Version 1.
- **Segment**: a declared site partition, such as a forum section, used for
  fairness and regression analysis.

## Decision and reason contract

Reasons are optional bounded codes, never free text.

| Reason | Display | Feedback interpretation |
|---|---|---|
| `not_relevant` | Not relevant | Negative source/destination pair |
| `wrong_anchor` | Wrong words linked | Negative anchor only; no destination negative |
| `better_target` | A better thread exists | Preference for the selected destination over the rejected one |
| `weak_target` | Target not good enough | Negative source/destination pair |
| `too_many_links` | Enough links here | Link-set feedback only; no destination label |
| `already_covered` | Already covered | Redundancy feedback only; no relevance negative |
| `other` | Other | No training label |

Decision mapping is:

| Decision | Feedback label | Constraints |
|---|---|---|
| Approve | Positive pair; preference over shown alternatives | Only when the exact destination and content versions are recorded |
| Reject without a reason | Weak negative with weight `0.5` | Never interpret as an anchor defect |
| Remove after insertion | Negative pair | Stronger than rejection only in the declared training weighting scheme |
| Withdraw rejection | Tombstone the earlier label | Later training exports exclude the withdrawn label |
| Recalculate, park, retire or stale | No new label | Preserve audit history only |

A placed link removed during a later edit supplies link-survival feedback,
not factual evidence about the destination. The weighting function and its
bounds are versioned in the training-data specification and cannot be
changed by Optuna.

## Identity, permission and freshness

Each `FeedbackLabel` contains:

```text
label_id, tenant_id, decision_id, suggestion_id, reason_code,
label_kind, source_content_id, source_version, source_digest,
destination_content_id, destination_version, destination_digest,
anchor_span?, preferred_destination_id?, weight, model_keys,
moderator_pseudonym, occurred_at, permission_epoch, state, schema_version
```

The following rules are mandatory:

1. The decision service derives the tenant and moderator pseudonym from the
   authenticated context. Supplied tenant or identity fields are ignored.
2. Spec 0028 must have accepted the decision under the moderator's review
   permission. Training cannot repair or reauthorise a rejected decision.
3. Source and destination versions must have been guest-visible and fully
   read. Partial content is not complete evidence.
4. A content edit, visibility withdrawal, permission-epoch change or source
   deletion marks affected labels ineligible for future exports.
5. Duplicate delivery of one `decision_id` produces one label. Conflicting
   payloads for that id raise `ERR-0053-DUPLICATE-CONFLICT` and block the
   affected dataset.
6. Label correction appends a tombstone or superseding record; it does not
   rewrite signed history.
7. Labels from different tenants never share a training partition or
   evaluation aggregate.

## Dataset preparation and splitting

The Rust job `feedback.dataset.prepare` (schema 1, spec 0019 maintenance
lane) prepares the export and split for its one site. It reads through the
authorised feedback export interface under RLS, without arbitrary table
access. That export feeds spec 0077's dataset freeze, which owns admission
to ranking studies. The worker's key names the site, input digest, export
policy, permission revision and erasure generation. At the 0019
publication fence, recheck the current lease and authority. For each
selected label, check current 0053 state and the exact committed 0028
decision and correction through the owning typed interfaces. An
earlier eligibility read, outbox acknowledgement, inbox receipt or
staged copy cannot order withdrawal against publication. Require
proof that the eligibility check and publication are ordered with
the source correction. Use the existing owner interfaces and fence.
If a correction committed first, or the fence cannot prove the
order, keep the staged export held under SUT-609. A correction can
commit without changing the permission or erasure version. A held
export publishes no ready event. A new authorised preparation uses its
own exact eligible input. Packaged Python libraries receive only
bounded typed inputs through spec 0104.

Each `TrainingDatasetManifest` records:

- schema and specification versions;
- tenant and declared segment coverage;
- immutable label and content-digest commitments;
- inclusion, exclusion, withdrawal and erasure counts;
- model, tokenizer, feature and spaCy pipeline compatibility keys;
- thread-disjoint training, validation and test partitions;
- time cut-offs and prevention of future-information leakage;
- weighting scheme, reason-code mapping and pseudonymisation salt version;
- source permission epochs and reconciliation evidence;
- dependency, code and configuration digests; and
- SHA-256 of the canonical manifest.

The Rust owner resolves the manifest's exact model, tokenizer,
feature and spaCy pipeline keys against the input profile of the
selected 0077 freeze or training plan. Resolve worker dependency,
code and configuration pins to the plan's admitted lock, bridge
and platform profile. Check this binding before freeze admission,
before handing bounded inputs to the 0104 sidecar, and when resuming
a checkpoint. Compare the expected input keys. The model being
trained can have a separate approved output identity. Identical
schemas, dimensions or reported package versions give no proof of
compatibility without the matching keys and artefact hashes.

Hold an input with a missing or mismatched binding before freeze
publication or sidecar consumption. Use BLOCK_FOR_DATA_QUALITY
under 0077. Show “This dataset needs its original worker profile.”
Retain the original immutable input and effect identity. A retry
may resume with the original
matching profile while it remains admitted, after current authority
and effect reconciliation. A different profile requires a new
authorised preparation and plan. Do not rewrite an old manifest,
translate its features in place or retry an unresolved effect with
a substituted profile.

The default temporal hold-out is the newest 20% of eligible decisions, with
all decisions from one thread confined to one partition. If temporal and
thread-disjoint constraints conflict, thread disjointness wins and the
manifest reports the actual time coverage. A partition with no eligible
examples is reported, not filled by duplication.

Feedback datasets do not replace the independent human-adjudicated
relevance datasets in spec 0057. A candidate needs the protected weekly
promotion evidence and every mandatory gate, even when feedback validation
improves.

## Single optimisation and activation route

1. Stage and verify the immutable export. Commit its durable reference,
   receipt and `feedback.dataset_ready` outbox row in one transaction.
   Carry an opaque reference, source digest, canonical export manifest
   digest and schema. There is no caller-set
   network location or training data in the event.
2. The spec 0077 consumer checks the site, policy, source digests and
   current permission and erasure evidence. Verify canonical manifest
   bytes against the named digest and schema. Commit the freeze job
   and inbox receipt together. Use the source and policy digests
   of spec 0077's freeze key. Check current eligibility through the
   0053 source on claim and at the 0019 freeze publication fence.
   Apply the same commit order rule to freeze. An eligible export
   or freeze that committed before withdrawal keeps its original
   receipt in history, within current read scope and retention.
   Recheck current source eligibility before serving the artefact,
   freezing an export or using a frozen input for a study. A later
   correction holds affected input even when its receipt survives.
   This event never starts a study.
3. Optuna may tune only the typed, bounded registry in spec 0077. Feedback
   cannot alter source integrity, permissions, evaluation labels,
   thresholds, moderator gates, protected resource limits or release
   policy.
4. The pinned packaged training application may update the eight-head
   neural ranker, calibration candidate and declared retrieval/fusion
   parameters under its declarative configuration. Rust owns the input
   and output contracts. BGE-M3 and the BGE cross-encoder base weights
   are not fine-tuned by this route.
5. A candidate receives an immutable manifest and passes identical,
   versioned evaluation inputs against the champion.
6. Only the Rust release-control component can request activation after
   the signed protected evidence authorises it. The scheduler,
   notification and sidecar have no activation authority.
7. `PROMOTE`, `KEEP_CHAMPION`, `ROLL_BACK` and
   `BLOCK_FOR_DATA_QUALITY` have exactly the meanings in spec 0077. None
   can waive nDCG@10, the per-forum BM25 improvement gate or protected
   safety and suitability checks.
8. A verified critical regression restores the complete previous artefact
   set. It does not revert one weight while retaining incompatible
   tokenizers, indexes or calibration artefacts.

## Scoring and calibration ownership

The eight prediction heads and their five-logit ordinal outputs are owned
by spec 0077. Feedback examples can train those heads only where a valid
mapping to the head's label semantics exists. A moderator approval is not
automatically a grade for all eight heads.

Calibration changes the meaning of confidence, not relevance labels or
source evidence. It is versioned and evaluated with the candidate.
Confidence intervals and abstention thresholds use spec 0057's locked
evaluation contract. A confidence score is never displayed as the
probability that statements in a destination are true.

Feature importance or an attention value is not a causal explanation.
Moderator explanations cite the actual source/destination features and
model manifest under spec 0027.

## Module-stage evidence

Spec 0053 supplies feedback-derived module-stage evidence. Spec 0015 applies
the stage admitted by the current spec 0077 champion; this specification
cannot transition a module itself.

- Every suggestion-producing module declares its required observation
  window and evidence fields in its feature specification. These describe
  evidence completeness, not an independent promotion threshold.
- Accept rates, decision counts, intervals, protected relevance results and
  severe-failure results are diagnostic inputs to spec 0077. A score never
  authorises review or shadow directly.
- `shadow` produces internal suggestions for evaluation but shows none to
  staff. `review` shows suggestions for moderator decision. Neither stage
  publishes automatically.
- Spec 0077 evaluates module-stage evidence with the complete candidate and
  every protected gate. Spec 0015 applies only the resulting champion's
  module-stage map, subject to the module's ceiling.
- Missing, stale or contradictory evidence leaves new and unadmitted modules
  in shadow. It cannot trigger a local demotion or re-promotion.
- Spec 0015 owns the audited stage receipts shown on the health page.

## Local review evidence for release

Supply spec 0057 with a fixed, read-only projection over one admitted
site, cohort and native review profile. Read through the owning 0028
and 0032 read contracts with current permissions. The projection
leaves operational labels, diagnostic accept rates and module stages
under their existing rules. It grants no native action or broad audit
access and creates no external data path.

Each eligible row carries the local decision ID and version, approved
decision kind, qualified audit sequence, source proof reference and
scope. Get approval and rejection order from their same-commit
`review_audit_ref`. Get successful human review Undo order from the
worker-owned terminal effect and its exact audited commit, joined to
the original 0028 proof of human review and the signed native receipt.
The intent sequence only proves origin. Select one row per committed
choice even after delivery replay, a repeated native effect result or
a batch. Count approvals as approvals. Keep completed eligible undo
and reject outcomes separate. Leave historical approval intact after
undo. Learning labels and aggregate dashboard figures cannot supply
release rows.

Pin the exact cohort, policy, source profile and verified audit cut
before selecting either window. Under 0057, the first window selects
the first 200 eligible rows using the admitted ascending commit order.
The current window selects the newest 200 using the same order in
reverse, then verifies every row follows the locked reference end.
A window returns at most 200 eligible rows. Page the local proof by
the last inspected audit sequence and pin direction, cut, site,
policy, profile, permission revision and erasure generation. Inspect
at most 200 audit entries per page and process one page per site on
each scheduler turn. The qualified profile also fixes the total
inspected rows, proof bytes and elapsed work before starting.
Prove coverage of the
enclosing range and the eligibility or exclusion of its contributors
under the same consistent cut. The output limit alone cannot prove
coverage or bound the work. A window read cannot hold the audit
append lock. Do not skip a possibly eligible row whose origin, exact
join or commit order is unknown.

Recheck the pinned permission revision and erasure generation before
each continuation and before sealing the local proof. If they change
or cannot be proved, invalidate the partial selection and hold. No
continuation, stale proof or older substitute may fill that window.
Below 200, overlap, conflicting identities, an unproved cut or
exceeding the admitted read bounds also holds the evidence for 0057.
Keep both window memberships and their evidence on the owner's PC.
Only the separately admitted 0057 aggregate hand-off may leave that
boundary.

## Storage and interfaces

PostgreSQL stores authoritative feedback and accept-rate evidence with RLS
forced. Queue, outbox and inbox records use spec 0019 and carry
only the refs needed for the authorised work. The original signed
moderation decisions retain their own source record.

Tables:

- `feedback_label`: the fields in `FeedbackLabel`, plus tombstone and
  supersession references;
- `feedback_dataset`: manifest digest, state, schema, storage reference,
  permission reconciliation and creation metadata;
- `accept_rate`: segment, observation window, decisions, accepts and
  interval, with no raw moderator identity.

Product actions:

| Action | Behaviour | Capability |
|---|---|---|
| `setu.tuning.status` | Read optimisation eligibility and recent controller outcomes | CAP-077 |
| `setu.tuning.enable` | Enable or pause new studies; never force promotion | CAP-077 |
| `setu.tuning.history` | Read proposals, manifests, evidence and outcomes | CAP-077 |
| `setu.tuning.revert` | Request the verified rollback route in spec 0077 | CAP-077 |
| `setu.module.stage_evidence` | Read feedback-derived module-stage evidence; never request a transition | CAP-045 |
| `setu.accept_rate` | Read accept rates and drift by segment | CAP-047 |

The CLI and MCP expose the same typed actions under spec 0020. Enabling
studies does not confirm a public write or protected setting change.

Events use the envelopes in spec 0019: `feedback.label_changed`,
`feedback.dataset_ready`, `feedback.dataset_blocked` and
`accept_rate.drift`. Module-stage receipts belong to spec 0015;
optimisation and controller events are owned only by spec 0077.

## Failure contract

| Code | Condition | Required result |
|---|---|---|
| `SUT-600` | Too few eligible decisions for the declared study | No study starts; record the shortfall |
| `SUT-601` | Candidate fails protected evaluation | `KEEP_CHAMPION`; retain evidence |
| `SUT-602` | Inputs or settings changed during comparison | Withdraw candidate and rebuild manifest |
| `SUT-603` | Verified critical regression | `ROLL_BACK` the complete admitted set |
| `SUT-604` | Required model or evaluation output is overdue | Keep champion; block new candidate admission |
| `SUT-605` | Product policy refuses a requested transition | No activation; record policy defect |
| `SUT-606` | Module-stage evidence is incomplete, stale or contradictory | Make no stage transition; keep new and unadmitted modules in shadow and report the evidence defect to spec 0077 |
| `SUT-607` | Job exceeds its admitted window | Checkpoint or stop safely; resume by idempotent key |
| `SUT-608` | Parameter is outside its typed registry or non-finite | Reject before training or activation |
| `SUT-609` | Permission, erasure or freshness reconciliation fails | `BLOCK_FOR_DATA_QUALITY` |
| `ERR-0053-DUPLICATE-CONFLICT` | Duplicate id has conflicting content | Quarantine id and block affected dataset |

Failures never weaken moderator review or silently emit a public link.

## Security, privacy and erasure

1. Training exports contain ids, content hashes, bounded features and reason
   codes. They contain no raw post text unless a separately approved custom
   training contract explicitly requires it and enforces the same tenant,
   permission and erasure controls.
2. Moderator identities are pseudonymous and cannot be ranked or shown to
   other users. Re-identification material stays in the private product
   boundary.
3. Workers receive only their authorised tenant and dataset manifest.
4. Erasure removes or tombstones affected labels and prevents new use of
   derived datasets. Existing candidates using them are blocked until
   re-evaluated under spec 0077.
   For an isolated backup restore, reconcile the affected labels and
   their derived manifests, checkpoints and candidate references
   against the current 0032 erasure history outside the backup.
   Apply the 0030 restore fence before use and retain the current
   erasure receipts. An affected pre-erasure manifest, job receipt
   or signed promotion record keeps only its permitted history.
   Hold the affected trial or candidate from fresh use with SUT-609.
5. Logs, metrics and events carry ids, digests, counts and bounded figures,
   never credentials, post text or moderator names.

## Resource and operational contract

Every workflow and worker has a measured resource-admission record under
AGENTS.md sections 5.6 and 11. The record states exact artefacts, label and
pair counts, batch sizes, CPU, memory, temporary storage, concurrency,
p95 duration and checkpoint overhead. No unmeasured fixed memory or nightly
runtime claim is accepted.

Dataset preparation streams bounded chunks. Training and evaluation cannot
run beside an incompatible model conversion, index backfill or second
challenger when their combined profile exceeds admission. Live review and
rollback have priority over studies. Handle an interrupted or
unacknowledged training result through the 0077 recovery fence.
A prior result receipt preserves only the original effect. Before
using the result again, recheck the current dataset eligibility
under this specification. Any separately verified fault in the
active set or its retained rollback set follows 0077 protective
control.

## Requirement, design and task traceability

These identities group the normative contracts above. They do not show
execution state or admit an implementation task. Derived tasks still
follow spec 0076's readiness gate.

| Requirement | Normative obligation | Design and derived task | Unit evidence | BDD evidence |
|---|---|---|---|---|
| `REQ-0053-LABEL-INTEGRITY` | Versioned decision and reason mappings, authenticated tenant and permission context, complete visible evidence, idempotent tombstones and freshness reconciliation preserve eligible feedback without crossing tenants | `DES-0053-FEEDBACK-PLATFORM`; `TASK-0053-FEEDBACK-PLATFORM` | `UT-0053-REASONS`, `UT-0053-WITHDRAW`, `UT-0053-TENANT`, `UT-0053-PERMISSION`, `UT-0053-PARTIAL` | `BDD-0053-INELIGIBLE-EVIDENCE` |
| `REQ-0053-DATASET-GOVERNANCE` | Immutable authorised manifests, thread-disjoint temporal partitions, bounded exports and permission or erasure reconciliation prevent ineligible or incompatible derived data from entering a study | `DES-0053-FEEDBACK-PLATFORM`; `TASK-0053-FEEDBACK-PLATFORM` | `UT-0053-SPLIT`, `UT-0053-ERASURE`, `UT-0053-EXPORT` | `BDD-0053-ERASURE-BLOCK` |
| `REQ-0053-CONTROLLED-LEARNING` | Feedback reaches training and module-stage evidence only through spec 0077's protected route, with Rust alone controlling activation, complete rollback and shadow or review authority | `DES-0053-FEEDBACK-PLATFORM`; `TASK-0053-FEEDBACK-PLATFORM` | `UT-0053-CONTROLLER-AUTHORITY` | `BDD-0053-PROMOTION`, `BDD-0053-REGRESSION`, `BDD-0053-MODE` |

| Design | Owning contract | Requirements and derived task |
|---|---|---|
| `DES-0053-FEEDBACK-PLATFORM` | The outcomes and boundaries through the resource and operational contracts above | `REQ-0053-LABEL-INTEGRITY`, `REQ-0053-DATASET-GOVERNANCE`, `REQ-0053-CONTROLLED-LEARNING`; `TASK-0053-FEEDBACK-PLATFORM` |

| Derived task | Implementation boundary | Upstream identities |
|---|---|---|
| `TASK-0053-FEEDBACK-PLATFORM` | Implement the Rust-owned feedback labels, manifests, package interfaces and spec 0077 control boundary without first-party Python production code or a second promotion path | `DES-0053-FEEDBACK-PLATFORM`; `REQ-0053-LABEL-INTEGRITY`, `REQ-0053-DATASET-GOVERNANCE`, `REQ-0053-CONTROLLED-LEARNING` |

## Test and acceptance design

Tests use synthetic tenants, content versions and signed fixtures. Rust
tests cover product policy, storage and package integration; permitted
non-production Python checks may exercise packaged library behaviour but
are never shipped. Integration tests cover queue and outbox recovery
and check that only the admitted host and helpers remain in the
spec 0104 sidecar. Job, study policy and activation logic stay in Rust.

| Test id | Requirement | Given | When | Then |
|---|---|---|---|---|
| `UT-0053-REASONS` | `REQ-0053-LABEL-INTEGRITY` | Every decision and reason combination | Labels are derived | Exactly the declared label kind and weight result |
| `UT-0053-WITHDRAW` | `REQ-0053-LABEL-INTEGRITY` | A rejected suggestion later restored | Withdrawal arrives twice | One tombstone exists and future exports exclude it |
| `UT-0053-TENANT` | `REQ-0053-LABEL-INTEGRITY` | Two tenants use the same external ids | Labels and datasets are built | No row, partition, metric or artefact crosses tenants |
| `UT-0053-PERMISSION` | `REQ-0053-LABEL-INTEGRITY` | A moderator lacks review permission | A decision event is offered | It is rejected and produces no label |
| `UT-0053-PARTIAL` | `REQ-0053-LABEL-INTEGRITY` | Only part of a source was read | A label is prepared | Dataset admission blocks incomplete evidence |
| `UT-0053-SPLIT` | `REQ-0053-DATASET-GOVERNANCE` | Decisions share threads and cross the time cut-off | Partitions are built | Threads never leak and actual temporal coverage is reported |
| `UT-0053-ERASURE` | `REQ-0053-DATASET-GOVERNANCE` | A source is erased after dataset creation | A study is requested | The dataset is blocked and affected candidates require re-evaluation |
| `UT-0053-EXPORT` | `REQ-0053-DATASET-GOVERNANCE` | An authorised export is produced | Fields are inspected | No undeclared text, identity or credential field exists |
| `UT-0053-CONTROLLER-AUTHORITY` | `REQ-0053-CONTROLLED-LEARNING` | Packaged training output improves a metric but lacks signed protected evidence | Activation is requested | Rust refuses; the champion and protected thresholds remain unchanged |
| `IT-0053-DUPLICATE` | `REQ-0053-LABEL-INTEGRITY` | One decision id arrives with identical and then conflicting bodies | Events are consumed | Identical delivery is idempotent; conflict returns `ERR-0053-DUPLICATE-CONFLICT` |
| `IT-0053-RELEASE-REVIEW-PROJECTION` | `REQ-0057-PROTECTED-RELEASE-GATES` | Use synthetic complete, 199-row and overlapping cuts. Delay a valid undo result across the cut and replay it. Plant a mismatched audit join, unrelated audit events, a false origin, an unqualified profile and an unresolved sequence gap. Exceed the admitted read bound with excluded events. In separate cases erase a contributor between pages or revoke the grant before seal | Freeze against the exact local cohort and select the two windows | Select at most 200 per window in proved commit order. Undo appears once at its terminal sequence. Known ineligible events stay excluded. A limit overrun or changed generation invalidates the affected selection. An incomplete, conflicting, overlapping or unproved selection holds with no aggregate output, older substitute or reusable continuation. Ordinary feedback and native review continue through their owning actions |
| `IT-0053-QUEUE` | `REQ-0053-DATASET-GOVERNANCE` | Worker dies after staging and the relay restarts after delivery | Queue recovery and consumer replay run | One authoritatively published manifest, one freeze job, and no duplicate study. Withdrawal after inbox commit but before claim and a wrong site both refuse |
| `IT-0053-AUTHORITY` | `REQ-0053-CONTROLLED-LEARNING` | The sidecar or scheduler requests direct activation | The request reaches Rust policy | It is refused regardless of metric results |
| `IT-0053-NO-PYTHON-PRODUCTION` | `REQ-0053-CONTROLLED-LEARNING` | Check source and shipped artefacts around the admitted sidecar | Language and artefact checks run | Only approved host and helpers can run. A planted Python objective, scheduler decision or activation path is refused |
| `BDD-0053-INELIGIBLE-EVIDENCE` | `REQ-0053-LABEL-INTEGRITY` | A source was visible at decision time but loses guest visibility before export | Permission reconciliation runs | The affected label is ineligible and the manifest records the exclusion without rewriting history |
| `BDD-0053-ERASURE-BLOCK` | `REQ-0053-DATASET-GOVERNANCE` | An existing dataset and candidate contain an erased source | A new study or promotion is requested | Both block pending authorised reconstruction and complete re-evaluation under spec 0077 |
| `BDD-0053-PROMOTION` | `REQ-0053-CONTROLLED-LEARNING` | A challenger improves training loss only | Weekly evidence is absent | Champion remains active and no setting or model changes |
| `BDD-0053-REGRESSION` | `REQ-0053-CONTROLLED-LEARNING` | An activated challenger has a verified critical regression | Controller evaluates evidence | Complete previous set is restored under spec 0077 |
| `BDD-0053-MODE` | `REQ-0053-CONTROLLED-LEARNING` | A module lacks complete stage evidence but has a high accept rate | Its candidate is evaluated | No local transition occurs; without spec 0077 admission it remains in shadow |
| `E2E-0053-FEEDBACK` | `REQ-0053-LABEL-INTEGRITY` | A moderator approves and later removes a link | Full learning route runs | Labels reflect both events; no automatic public write occurs |
| `NEG-0053-THRESHOLD` | `REQ-0053-CONTROLLED-LEARNING` | Optuna suggests a changed release threshold | Registry validation runs | Value is refused before a trial starts |
| `OPS-0053-RESOURCE` | `REQ-0053-DATASET-GOVERNANCE` | A study exceeds its admitted resource profile | Scheduler observes usage | Study waits or stops; live review and rollback remain available |

For the restore variant in BDD-0053-ERASURE-BLOCK, keep an authentic
backup containing one trial checkpoint, its dataset and a previously
verified candidate A. Erase a source used in A after the backup.
Apply the current 0032 history before releasing restored data.
Replay that checkpoint with the old signed success evidence.
The affected label stays unavailable. No affected artefact is
served, reused for training or offered for the controller decision.
Supply the same complete A artefacts to the existing 0077 rollback
verification with an otherwise valid current rollback decision.
Current source proof fails with ERR-0077-ROLLBACK-UNVERIFIED.
Report no verified rollback and keep release and new activation
blocked. An independent synthetic control with a prior set free
of the erased input and proof of every other current check can
complete its own exact rollback. It supplies no substitute for A.

Extend IT-0053-QUEUE under REQ-0053-DATASET-GOVERNANCE. With two
eligible labels, let the worker read eligibility and stage the
export. Commit the exact 0028 rejection withdrawal for one label
before 0053 publication, leaving permission, erasure generation
and lease unchanged. Test with the correction event still delayed
and after the tombstone arrives. The held export gives SUT-609 and
publishes no ready event. Also hold when the current source proof
cannot order the commits. In separate runs, commit the eligible
export first. Place the withdrawal after the 0077 eligibility read
and before freeze publication, then reverse that final order in
the control run. The first freeze holds, even with delayed delivery.
The second preserves the prior freeze and export receipts. After
withdrawal, replay exposes only the currently authorised historical
result, and affected artefact reads and later study use hold. A
fresh authorised preparation excludes the withdrawn label and keeps
the other eligible label. Preserve the original moderation receipt.
Use the existing source, outbox and freeze identities and synthetic
evidence only.

Property tests cover duplicate delivery, thread-disjoint splitting,
monotonic permission withdrawal and canonical manifest serialisation.
Failure injection covers outbox replay, job restart, stale model keys,
erasure during evaluation and partial checkpoints. In IT-0053-QUEUE,
keep site authority and all labels current. Use a valid manifest and
checkpoint from admitted profile A. Separately substitute another
admitted profile B with the same supported schema, feature dimensions
and displayed package version but a different pinned spaCy pipeline
artefact and key. Test at freeze admission and at resume before
sidecar hand-off. Repeat with the required profile proof missing.
Both cases hold before a new freeze, payload hand-off or training
effect. A then-current exact A resumes the original effect only
after reconciliation. A committed prior effect returns its authorised
receipt without rerunning. Feature-specific coverage and mutation
requirements follow ADR-0020.

## Footprint budget

This specification adds nothing to public pages. Suggestion rendering and
insertion retain the budgets in specs 0027 and 0028.

## Provenance

The decision mappings and interfaces are original Sutradhar design derived
from the archived feedback specifications and revised under ADR-0020.
Optuna and spaCy use admitted dependency licences and provenance.
An algorithm reimplementation records its own provenance and legal
evidence under spec 0076.

Source: `docs/specifications/linking/feedback.md`,
`linking/feedback-2.md` and `linking/feedback-3.md` in the archive
`archive-2026-11-08-old-repo`.
