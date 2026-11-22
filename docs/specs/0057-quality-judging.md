# 0057: Quality judging and release assessment

## Purpose and ownership

This specification owns relevance and usefulness labels, evaluation
protocols, ranking and suitability metrics, statistical evidence and the
release assessment. Spec 0077 owns optimisation and the four controller
outcomes. Spec 0076 owns feature-design readiness. Spec 0064 owns complete
Version 1 scope qualification. Spec 0019 owns the product queue
and transactional outbox.

nDCG@10 at least 0.95 is a mandatory release floor. It is not an
aspirational target, a measured result or proof that 95% of proposed links
are suitable. One is the theoretical maximum. The independent BM25 gate,
link suitability, severe failures, safety, calibration and resource gates
remain mandatory.

## Evaluation artefacts and trust

Every evaluation run freezes and hashes:

- candidate and champion manifests;
- source-content snapshots and complete-reading receipts;
- source permissions and visibility at evaluation time;
- query, candidate-pool and destination identities;
- label rubric, labels, adjudications and label-policy version;
- training, development, calibration, nightly-search and held-out split
  membership;
- BM25 implementation, tokenisation, fields, parameters and output rules;
- metric protocol, thresholds, strata, seeds and statistical method;
- runtime, models, spaCy pipelines, dependency locks and hardware profile;
- every result, exclusion, error and controller or release receipt.

The evaluation-label and threshold store is strict-core under ADR-0020.
Only authorised, independent label reviewers and adjudicators can write
it. Packaged Python workers and Optuna receive typed read-only views
through the admitted Rust bridge and cannot change labels, thresholds,
split membership, source permissions or the release policy. No first-party
Python production adapter is permitted. A changed protocol creates a new
evaluation identity; results cannot be moved across identities.

Model scores are not labels or factual evidence. Agent judges may assist
with usefulness labels but do not replace the release adjudication
process or judge their own candidate model.

## Relevance and usefulness rubric

Each query names a source passage or thread task, an anchor where
applicable, an eligible judged destination pool and the intended reader
task. Every returned destination receives an integer grade from 0 to 4:

| Grade | Meaning |
|---:|---|
| 0 | Irrelevant, misleading, contradictory, stale for the task or otherwise unsuitable |
| 1 | Weak topical overlap but no useful reason to follow the link |
| 2 | Relevant and potentially useful; counts as relevant for MRR and Recall@40 |
| 3 | Relevant, contextually appropriate and useful; counts as a good link for Precision@6 |
| 4 | Direct, highly useful and evidence-rich destination with excellent contextual and entity/version fit |

Labels consider topical relevance, anchor fit, answer usefulness, intent,
evidence quality, entity/version fit, information gain and editorial
suitability. These correspond to the eight prediction heads in spec 0077,
but head outputs never supply the labels.

Two independent qualified reviewers label each promotion item without
seeing model scores or each other's answers. Disagreement of more than one
grade, any disagreement across the grade-2 or grade-3 boundary, and every
severe-failure flag require a third senior adjudicator. The adjudication
record names the rubric criterion and evidence. An unresolved item blocks
data quality; it is not assigned zero to make evaluation convenient.

Usefulness is also assessed independently by agent judges as a 0–4 score
using the same evidence pack. Agent-judged usefulness is reported
separately from human-adjudicated relevance. Judge agreement, invalid
answers, systematic bias and drift are measured. Any disagreement across
the good-link boundary requires human adjudication before release use.
Agent judges are version-pinned, blind to candidate identity and never the
embedding, reranker or neural ranker being evaluated.

Outside AI routes for product data remain off unless the owner enables
them. Their packs
are bounded and redacted, site text is untrusted data, no tools are
available and credentials are isolated. Local judging does not require an
outside route.

Project push and release use ADR-0021's synthetic fleet. Freeze
three XenForo and three WordPress sites before judging. The fleet
covers tech and PC repair, personal finance and legal basics,
health and fitness, and hobbies. Each site and declared stratum
contributes eligible judgments.

Use three independent pinned AI makers. Apply two blind judgments
per selected item. The third maker resolves ties. Assign real work
across all three. Bind actual maker, model version and rubric
receipts to the frozen allocation and candidate. Judges see only
made-up packs and have no tools or credentials. Missing evidence or
unresolved grades block. Human adjudication and the relevance
rubric still apply. This does not change product-data consent.

## Dataset splits and held-out governance

Thread and time separation prevents a source thread, its later posts or
near-duplicate destination versions from crossing training, development,
calibration and held-out splits. Split assignment uses immutable content
and thread identities, not outcome-dependent selection.

Optuna, early stopping and calibration never read weekly promotion labels
or per-query hidden answers. Weekly evaluation returns only authorised
aggregate receipts and the controller outcome. A new held-out rotation is
created under the label policy before repeated candidate selection could
adapt to the previous set. Rotation never removes a difficult forum or
stratum from the eligible population.

The tiers are:

| Tier | Threads | Queries | Purpose |
|---|---:|---:|---|
| Local | 16 | 32 | Development diagnosis only |
| Push | 48 | 96 | Required six-site and three-maker proof. Diagnostic scores give no promotion authority |
| Extended integration | 256 | 512 | Challenger screening |
| Nightly Optuna | 512 | 1,024 | Search and pruning |
| Weekly promotion | 2,000 | 4,000 | Mandatory promotion evidence |
| Monthly integrity | All eligible threads | Separate reconciliation | Source, permission, label and result integrity |

Samples are stratified across forums, languages, source types,
publication periods and thread lengths. The protocol declares strata and
allocation before seeing candidate scores. A missing required stratum is
`BLOCK_FOR_DATA_QUALITY`; no post-hoc reweighting or exclusion is allowed.

## Ranking metric definitions

### Common rules

Metrics use the adjudicated labels from the frozen judged pool. Query
macro averages give each eligible query equal weight. Forum results give
each eligible query in that forum equal weight. Per-forum and stratum
results accompany the overall result.

Duplicate destination identities occupy no additional credit. The first
returned occurrence keeps its rank; later occurrences have zero gain and
are recorded as output defects. A returned item with no eligible label,
an unauthorised or stale source, a changed source version or an
incomplete-reading receipt causes `BLOCK_FOR_DATA_QUALITY`; it is never
silently treated as grade zero.

An unfilled rank has zero gain and zero good-link credit. Abstention does
not reduce a metric denominator. Coverage and abstention are reported
separately, and the product's 3–6 suitable-destination contract still
applies.

### nDCG@10

For relevance grade `rel_i` at one-based rank `i`:

```text
DCG@10 = sum over i = 1..10 of (2^rel_i - 1) / log2(i + 1)
IDCG@10 = DCG@10 for grades sorted descending from the eligible judged pool
nDCG@10 = DCG@10 / IDCG@10
```

Only the first ten eligible judged-pool destinations contribute to the
ideal ranking. Missing returned ranks have grade zero for DCG. A query
whose judged pool has no destination with grade above zero has IDCG zero:
report it separately and exclude it from the nDCG macro average. If no
eligible query has positive IDCG, the evaluation is
`BLOCK_FOR_DATA_QUALITY`.

The release and promotion floor is:

```text
macro nDCG@10 >= 0.95
```

The point estimate must pass. A confidence interval cannot waive a failed
point estimate.

### MRR

A destination is relevant for MRR when its grade is at least 2. For each
query, reciprocal rank is `1/r` for the first relevant destination in the
returned ranking, or zero when none is returned. MRR is the macro average.
An unjudged returned destination blocks the run rather than being skipped
while searching for a later relevant item.

### Recall@40

For each query:

```text
Recall@40 = unique returned destinations in the top 40 with grade >= 2
            / all unique eligible judged-pool destinations with grade >= 2
```

A query with no grade-2-or-higher destination is reported and excluded
from the Recall@40 macro average. No eligible recall queries blocks data
quality. Duplicate destinations count once. Unjudged returned candidates
block the run.

### Precision@6 and link suitability

A destination is a good link when its grade is at least 3. For each query:

```text
Precision@6 = good-link destinations in the top 6 / 6
```

The denominator remains six when fewer than six are returned. Empty slots
and duplicate later occurrences earn no credit. Link suitability is also
reported as the share of all proposed links judged grade 3 or 4, with its
own numerator, denominator, severe-failure count and abstention coverage;
it is not inferred from nDCG@10.

### Agent-judged usefulness

Normalise each adjudicated usefulness score to `score / 4` and macro
average over returned proposals. Report agreement, invalid rate and
coverage. The metric cannot substitute for human-adjudicated relevance or
Precision@6. A missing usefulness judgement on a returned proposal blocks
the affected assessment.

### Calibration and abstention

Calibration uses a separate frozen split and the probability that a
proposal is grade 3 or 4. Expected calibration error uses ten equal-width
confidence bins `[0, 0.1), …, [0.9, 1.0]`, weighting each non-empty bin's
absolute confidence/accuracy difference by its item count. The final bin
includes confidence exactly 1. Temperature fitting never reads promotion
labels.

Abstention threshold is an operating parameter, not an evaluation
threshold. The evaluation reports coverage, suitability among returned
items, severe failures and all fixed-denominator metrics. A candidate
cannot improve its release result by silently reducing the required
number of destinations.

### Severe relevance failures

A severe relevance failure is a top-ranked grade-0 destination, a
deceptive or contextually harmful link, or an entity/version mismatch
that could mislead a reader. A permission, privacy or source-integrity
violation is additionally a safety failure and is never tolerated.

Promotion permits no new severe irrelevant top-ranked result and no
safety failure. Release reports absolute counts by forum and stratum and
passes only its predefined severe-failure and safety gates. A high average
score cannot hide one of these failures.

## Locked BM25 baseline and independent relative gate

The baseline is BM25 over the locked title and post-body fields using the
frozen tokenisation, parameters and top-six output contract. Its manifest
binds implementation, dependency, tokenizer, field definitions, `k1`, `b`,
field weights, judged dataset and result digests. Optuna cannot change the
locked baseline. A separately tuned challenger BM25 configuration is not
the baseline.

On every eligible test forum:

```text
challenger macro Precision@6 >= 1.20 * locked BM25 macro Precision@6
```

Both use identical judged destinations and output rules. Comparisons use
exact rational counts or cross-products, not rounded displayed
percentages. A zero BM25 denominator on an eligible forum makes the
relative gate unevaluable and produces `BLOCK_FOR_DATA_QUALITY`, not an
automatic pass.

This gate and nDCG@10 at least 0.95 are independent. Neither waives the
other. No forum may be excluded after seeing its result.

## Statistical comparison and power

Champion and challenger are evaluated on identical frozen queries,
sources, labels, hardware profile and request order. The primary paired
comparison uses 10,000 query-level bootstrap resamples with replacement,
seeded from the evaluation manifest. The same resampled query identities
are used for both systems. The report retains the point difference, 95%
paired interval and per-forum results.

Challenger improvement requires positive nDCG@10 point improvement and a
one-sided 95% paired-bootstrap lower bound above zero. Absolute floors
still use their point estimates. MRR and Recall@40 may not regress by more
than 0.005 absolute at push diagnostic level; at promotion they obey the
protected comparison in spec 0077. No tolerance waives nDCG, BM25, safety,
suitability or severe-failure gates.

Before promotion evaluation, the protocol estimates per-forum power from
locked baseline variance and the paired design. Each forum must have at
least 80% power at one-sided 5% significance to detect the required 20%
Precision@6 relative effect. If the fixed weekly allocation cannot supply
that evidence, the outcome is `BLOCK_FOR_DATA_QUALITY`; the release cannot
claim the gate from an underpowered sample. The power method, inputs and
calculation receipt are frozen before challenger scores are unblinded.

Local and push tiers cannot establish promotion. A small sample cannot
prove tiny differences or the absolute release floor.

## Controller and release decisions

Spec 0077 maps evaluation to exactly `PROMOTE`, `KEEP_CHAMPION`,
`ROLL_BACK` and `BLOCK_FOR_DATA_QUALITY`. Data-quality failure takes
precedence. `KEEP_CHAMPION` preserves operations only; it is not a release
waiver when the champion is below 0.95 or fails another release gate.

A release requires all of the following for the exact candidate:

1. Every feature and declared supported variant has complete spec 0076
   readiness and acceptance evidence, and spec 0064 scope qualification
   passes.
2. Weekly promotion-tier nDCG@10 is at least 0.95.
3. Every eligible forum passes the 1.20-times-BM25 Precision@6 gate.
4. MRR, Recall@40, link suitability, agent-judged usefulness,
   calibration, diversity and severe-failure gates pass with no protected
   regression.
5. Every source is authorised, current and completely read; monthly
   integrity reconciliation passes.
6. The 3–6 suitable internal-destination gate and all moderator approval,
   public-write and undo controls pass.
7. Every required check passes, strict-core assurance passes and all
   required independent reviews pass.
8. The 72-hour supervised production-profile soak, full load, overload,
   memory, recovery and chaos tests pass on the declared target profiles.
9. No open critical or high-severity shipped-component defect or security
   finding remains, and every release fault has a disposition under the
   tracker rules.
10. Dependency, licence, notice, provenance and applicable legal gates
    pass. Engineering research is not represented as patent clearance.
11. The exact candidate meets spec 0079's per-family ranking-method
    floor, and spec 0114's long lane passes.
12. Obtain the spec 0005 approval after the preceding checks. The
    protected release step signs the exact artefacts.

A missing, stale, contradictory or unverifiable artefact fails its
criterion. The assessment screen and `sutra release assess` name each
criterion, result, evidence identity and plain-English failure reason.

IT-0057-SYNTHETIC-JUDGES checks all six site identities and all three
maker identities on push and release. Try a missing site, shared
maker, withheld tie decision and production text. Each fails the
required judge proof. A valid push cannot promote or release a model.

### Post-launch review acceptance

DR-152 defines the field criterion required by 0005. Launch is the
first verified final public product release under 0005's release
record. Candidate tags, previews, staging and synthetic runs do not
establish launch. Only a verified pre-launch state makes this
criterion inapplicable. Unknown launch state or a missing required
field receipt holds the release. The synthetic, safety and resource
criteria remain independently required.

The receipt measures the deployed review workflow, not the new
candidate's future acceptance. Use one fixed, owner-approved production
review cohort from one site, declared before observing its reference
decisions. Lock the site's local identity, native platform and profile,
review-page routes, workflow and eligibility policy. Preserve those
values and required rights for both windows. Record the exact deployed
producer, release and review or champion profile for each. Different
producer builds must have verified equivalent decision and display
semantics under the locked cohort policy. Hold a changed or unproved
cohort. Scope or policy replacement uses the existing protected owner
route and carries no automatic or weaker reference.

Read only locally committed human review decisions from 0028 through
the qualified 0053 projection. Order only by a verified 0032 audit
sequence bound to the committed 0028 review record. An asynchronous
job or audit-delivery order alone cannot prove that ordering. Count
each distinct committed approve and reject once, regardless of the
reject reason or later link application.
Count a human review-page undo once only when the exact removal has a
verified `removed` terminal effect. A stale, failed, `gone` or
`undo_conflict` outcome cannot qualify. Approval counts positive. Reject and
eligible undo count only in the denominator. An undo does not rewrite
the recorded approval. Failed actions, batch wrappers, repeat delivery,
automatic work, shadow or test records and native edits outside the
review page contribute nothing. A pending undo has no terminal choice
in the cohort until its effect is verified at the fixed cut. Its
terminal commit sequence fixes its order. The intent and later audit
delivery do not set that position.

Before the first field assessment, freeze the first 200 eligible
post-launch decisions of the declared cohort as its reference.
Independently attest the whole selection from the first allowed
sequence, including the filtered evidence and the ending checkpoint.
Hold if older actions could be eligible but lack the required proof.
A zero-approval reference cannot admit release. Do not skip past it to
find a better sample. Later assessments freeze their current audit
checkpoint and take the latest 200 eligible decisions of the same
cohort. All 200 must follow the reference end. In both windows,
verify the enclosing audit range and the join to committed decisions,
including decisions filtered out. Inclusion alone cannot prove
completeness. Missing contributors, conflicting duplicates, withdrawn
rights or unverifiable sequence coverage hold the window. Do not
backfill an older decision to replace a selected one that loses proof.

With exactly 200 decisions in each window, compare integer approval
counts. Pass when the current count meets or exceeds the reference
count. Compare before percentage rounding. Equality passes the field
criterion only. A smaller verified count fails release under
`ERR-0057-RELEASE-GATE`. A missing, incomplete, stale or ineligible
window returns SUT-855. Report the criterion through the assessment
screen and `sutra release assess` with its permitted evidence reference.
Say “Review acceptance was below the reference” or “The latest review
decisions could not be verified”, as applicable. If launch proof is
unknown, say “The public release history could not be verified”.
This comparison reports
observed health. It grants no candidate deployment or ranking authority.

#### Field evidence and custody

On the owner's PC, keep decision IDs, audit sequences, exclusion
details, actor and site metadata, and raw records in their owning
protected stores. The gate receives only an admitted offline aggregate
for its exact release attempt. Use digests and counts, with no site
address, site ID, actor, content or raw action event.

| Evidence field | Exact meaning |
| --- | --- |
| `schema_version` | Integer 1 for this projection |
| `policy_digest`, `cohort_commitment` | SHA-256 of the admitted rule and a protected opaque cohort identity |
| `launch_record_digest`, `candidate_digest`, `release_run_id` | Verified first-final record, exact proposed candidate and protected release run identity |
| `reference`, `current` | Each has an integer `decisions` equal to 200 and integer `approvals` from 0 to 200. Reference approvals must be positive. Each includes its exact deployed release and profile digests, verified window commitment, decision-selection proof digest and current authority proof digest |
| `reference_receipt_digest`, `recipient_key_id` | Exact frozen signed reference and independently admitted Dell verifier recipient |
| `issued_at`, `expires_at` | Unsigned UTC seconds for exact source issue and expiry. The deadline cannot exceed either the release run or the owner export approval expiry |

The receipt and its two window commitments must bind this exact schema,
cohort and decision rule. A copy of 200 signed leaves, a diagnostic
dashboard count or an owner-entered value cannot supply selection proof.
Use the 0005 release hand-off after its own recipient, producer,
completeness and privacy qualification. Each transfer needs the owner's
approval of the exact safe projection and recipient. Verify current
rights and erasure state at local freeze, transfer and release approval.
Recheck before signing and again before publication if separate.
Revalidation binds the same frozen reference identity. A lost proof
or unadmitted hand-off holds the criterion. An exact current receipt
may reconcile to the same result once. A new candidate, changed evidence,
revocation or recipient change needs fresh qualified evidence.

## Scheduled execution and evidence recovery

Spec 0077 owns the ranked workflows under spec 0019. Its shared job
records carry opaque evaluation references. Labels, work positions
and per-query results stay inside the evaluator's protected scope.
Only signed permitted aggregates and the controller outcome leave
that scope. Sealed release checks return only the evidence allowed
by AGENTS.md. The signed task grants access to the release
evaluation. Publication needs spec 0005's approval receipt.

Each item and result has an idempotency key and exact input digest.
Replay checks the current authorised allocation and its use record.
Repeated delivery of one completed judgement returns its receipt
without charging again or creating another label.

A crashed run resumes from immutable packs and signed intermediate
receipts. A provider usage limit pauses only the affected external judge;
it does not redraw the sample. Protocol or model changes invalidate the
cache and require a fresh assessment. Dead-letter, quarantine and replay
are permission-checked and audited.

In IT-0057-REPLAY-IDEMPOTENT, start weekly scoring, then activate a
separately admitted synthetic protocol revision before the evaluator
accepts completion. Keep model bytes, query IDs, grades, thresholds
and measured latency equal. Stop the old evaluation with SUT-854.
Its current-format checkpoint or a matching score cannot resume work
under the new identity. A fresh evaluation uses its own authorised
allocation under 0077. In the separate BDD-0057-RELEASE-EVIDENCE case,
offer an authentic completed pass from the former protocol for the
current criterion. Keep the current candidate and other qualifying
proofs valid. Return SUT-855 for the stale criterion evidence and
hold release. The old proof keeps its original scope. A fresh passing
result for the exact current identity clears only that criterion.
Use synthetic allocations and leave thresholds and signing unchanged.

Labels, verdicts and results live in the RLS-protected evaluation schema.
Packs contain only the required source and destination evidence. Exports
contain identifiers, grades, reason codes, rounds and aggregates, never
credentials or unrestricted post text.

## Battle-testing evidence

Every release candidate, add-ons included, retains evidence for:

1. **Fault injection:** PostgreSQL, queue claims, relay, protected
   evaluator, site APIs, storage, artefacts and judge routes fail
   at declared boundaries. Inspect queue records after replay for
   protected data and refuse a weekly job offered as a release. Every
   case recovers or returns its typed error without duplicate public
   effects.
2. **Performance:** hot-path and end-to-end budgets pass with measured
   baselines and load.
3. **Defect checking:** every required test, static analysis, fuzz,
   property, mutation and scoped formal obligation passes for the
   candidate's risk class.
4. **Security:** dependency and image audits, secret scanning, signature
   and replay tests, tenant isolation, permission and API fuzzing pass.
5. **Resource management:** CPU, memory, processes, connections, file
   handles, queues and tasks remain within measured admission budgets.
6. **Storage:** compression, integrity and restore checks pass; disk use
   per 1,000 posts and compression ratio do not regress beyond the
   approved performance contract.
7. **Memory management:** soak, load and chaos high-water marks pass with
   no out-of-memory kill or unexplained growth.
8. **Profiling:** CPU, heap and admitted model-runtime profiles identify
   regressions; a regression blocks until corrected or formally
   dispositioned under the performance policy.
9. **Chaos recovery:** each component killed mid-work resumes within its
   recovery objective without lost or duplicated externally visible work.
10. **Quality assurance:** the complete acceptance suite, accessibility,
    visual regression and add-on install/use/upgrade/removal procedures
    pass on both platforms.
11. **Load and overload:** expected peak meets service objectives; twice
    expected peak sheds background work and refuses excess safely without
    affecting public pages.

A missing check fails release. Evidence binds the candidate, hardware,
resource profile and immutable workloads.

## Error contract

| Code | Meaning and outcome |
|---|---|
| `SUT-850` | Judge unavailable; pause the affected job and retain frozen inputs |
| `SUT-851` | Invalid verdict after the permitted repair; block the affected assessment |
| `SUT-852` | Invalid verdict rate above the frozen rubric limit; block data quality |
| `SUT-853` | Reviewer or judge agreement below the rubric requirement; adjudicate or block, never average away |
| `SUT-854` | Protocol, label, threshold or split changed after scoring began; stop and create a new authorised evaluation identity |
| `SUT-855` | Required evidence missing, stale, contradictory or bound to another candidate; fail the criterion |
| `SUT-856` | Soak or required resource run interrupted; restart the affected clock and preserve failure evidence |
| `SUT-857` | Outside judge terms, privacy or credential checks fail; refuse that route |
| `SUT-858` | Candidate is not the exact immutable release candidate; refuse assessment |
| `SUT-859` | Credential or unauthorised content exposed to a judge; stop, quarantine and raise a security defect |
| `ERR-0057-UNJUDGED-CANDIDATE` | Returned candidate is unjudged, unauthorised, stale or incompletely read; `BLOCK_FOR_DATA_QUALITY` |
| `ERR-0057-EVALUATION-POPULATION` | No eligible nDCG or Recall queries, a required stratum is absent, or BM25 baseline is zero; `BLOCK_FOR_DATA_QUALITY` |
| `ERR-0057-INSUFFICIENT-POWER` | Weekly allocation lacks the required statistical power or paired measurement is unstable; block promotion |
| `ERR-0057-RELEASE-GATE` | nDCG, BM25, suitability, calibration, severe-failure or resource gate failed; no promotion or release |
| `ERR-0057-MALFORMED-RANKING` | Duplicate destination, malformed ranking or denominator contract violated; record output defect and block affected evaluation |

## Requirement, design and task traceability

These identities group the normative contracts above. They do not show
execution state or admit an implementation task. Derived tasks still
follow spec 0076's readiness gate.

| Requirement | Normative obligation | Design and derived task | Unit evidence | BDD evidence |
|---|---|---|---|---|
| `REQ-0057-METRIC-ARITHMETIC` | Adjudicated labels, exact ranking formulas, fixed denominators, unique-destination credit and explicit abstention coverage produce reproducible metrics; malformed or unjudged rankings block rather than being silently repaired | `DES-0057-EVALUATION-PLATFORM`; `TASK-0057-EVALUATION-PLATFORM` | `UT-0057-NDCG-FORMULA`, `UT-0057-NDCG-EMPTY-RANKS`, `UT-0057-ZERO-IDCG`, `UT-0057-UNJUDGED-NOT-ZERO`, `UT-0057-DUPLICATE-DESTINATION`, `UT-0057-MRR-THRESHOLD`, `UT-0057-RECALL-DENOMINATOR`, `UT-0057-PRECISION-SIX`, `UT-0057-ABSTENTION` | `BDD-0057-MALFORMED-RANKING` |
| `REQ-0057-PROTECTED-RELEASE-GATES` | The nDCG and per-forum BM25 floors, severe-failure and complete release-evidence gates apply independently; `KEEP_CHAMPION` cannot waive release failure and no result-dependent exclusion is allowed. Apply the separate DR-152 post-launch criterion under its exact field evidence | `DES-0057-EVALUATION-PLATFORM`; `TASK-0057-EVALUATION-PLATFORM` | `UT-0057-BM25-BOUNDARY`, `UT-0057-BM25-ZERO`, `UT-0057-GATES-INDEPENDENT`, `UT-0057-SEVERE-FAILURE`, `UT-0057-FIELD-ACCEPTANCE` | `BDD-0057-KEEP-NOT-RELEASE`, `BDD-0057-NO-POSTHOC-EXCLUSION`, `BDD-0057-RELEASE-EVIDENCE`, `BDD-0057-FIELD-ACCEPTANCE` |
| `REQ-0057-STATISTICAL-EVIDENCE` | Frozen paired queries, seeded bootstrap, preregistered power and governed held-out rotations establish comparable promotion evidence without adaptive exclusion | `DES-0057-EVALUATION-PLATFORM`; `TASK-0057-EVALUATION-PLATFORM` | `UT-0057-POWER-INSUFFICIENT`, `UT-0057-BOOTSTRAP-PAIRED` | `BDD-0057-HELDOUT-ROTATION` |
| `REQ-0057-EVIDENCE-JUDGE-GOVERNANCE` | Read-only label and threshold views, frozen calibration, disabled-by-default outside judges and idempotent replay protect judgement provenance and prevent duplicate outcomes | `DES-0057-EVALUATION-PLATFORM`; `TASK-0057-EVALUATION-PLATFORM` | `UT-0057-CALIBRATION-BINS` | `BDD-0057-OUTSIDE-JUDGE-OFF` |

| Design | Owning contract | Requirements and derived task |
|---|---|---|
| `DES-0057-EVALUATION-PLATFORM` | The evaluation artefacts, rubric, metric, statistical, controller, scheduler, recovery and release contracts above | `REQ-0057-METRIC-ARITHMETIC`, `REQ-0057-PROTECTED-RELEASE-GATES`, `REQ-0057-STATISTICAL-EVIDENCE`, `REQ-0057-EVIDENCE-JUDGE-GOVERNANCE`; `TASK-0057-EVALUATION-PLATFORM` |

| Derived task | Implementation boundary | Upstream identities |
|---|---|---|
| `TASK-0057-EVALUATION-PLATFORM` | Implement the evaluation, immutable evidence and admitted package boundaries without first-party Python production code or a second metric or release-control path | `DES-0057-EVALUATION-PLATFORM`; `REQ-0057-METRIC-ARITHMETIC`, `REQ-0057-PROTECTED-RELEASE-GATES`, `REQ-0057-STATISTICAL-EVIDENCE`, `REQ-0057-EVIDENCE-JUDGE-GOVERNANCE` |

## Verification design

The following are required test designs, not claims that executable tests
have run.

| Test | Requirement | Case | Required result |
|---|---|---|---|
| `UT-0057-NDCG-FORMULA` | `REQ-0057-METRIC-ARITHMETIC` | Grades `[4,2,1,0]` and an ideal pool `[4,3,2,1]` | Compute gain as `2^rel-1`, logarithmic discount and ideal from the judged pool; compare exact rational result within the declared floating tolerance |
| `UT-0057-NDCG-EMPTY-RANKS` | `REQ-0057-METRIC-ARITHMETIC` | Four returned results and six missing ranks | Missing ranks contribute zero and denominator remains IDCG@10 |
| `UT-0057-ZERO-IDCG` | `REQ-0057-METRIC-ARITHMETIC` | One all-zero query, one positive-IDCG query, then only all-zero queries | First run reports and excludes only the zero-IDCG query; second returns `BLOCK_FOR_DATA_QUALITY` |
| `UT-0057-UNJUDGED-NOT-ZERO` | `REQ-0057-METRIC-ARITHMETIC` | Grade-4 rank 1 and unjudged rank 2 | Block data quality; do not score rank 2 as zero or skip to rank 3 |
| `UT-0057-DUPLICATE-DESTINATION` | `REQ-0057-METRIC-ARITHMETIC` | Same destination at ranks 2 and 5 | First keeps its rank; later receives no gain and emits the duplicate-output defect |
| `UT-0057-MRR-THRESHOLD` | `REQ-0057-METRIC-ARITHMETIC` | Returned grades `[1,2,3]`, then `[1,1,0]` | Reciprocal rank is `1/2` in the first and zero in the second |
| `UT-0057-RECALL-DENOMINATOR` | `REQ-0057-METRIC-ARITHMETIC` | Judged pool has four grade-2+ destinations and top 40 contains two unique relevant plus one duplicate | Recall@40 is `2/4`; duplicate adds nothing |
| `UT-0057-PRECISION-SIX` | `REQ-0057-METRIC-ARITHMETIC` | Four returned grades `[4,3,2,1]` | Precision@6 is `2/6`, not `2/4`; grade 2 is relevant but not a good link |
| `UT-0057-BM25-BOUNDARY` | `REQ-0057-PROTECTED-RELEASE-GATES` | Locked baseline exactly `0.25`, challenger exactly `0.30`, then `0.299` | First passes the 1.20 gate; second fails without display rounding |
| `UT-0057-BM25-ZERO` | `REQ-0057-PROTECTED-RELEASE-GATES` | Baseline zero and challenger one | `BLOCK_FOR_DATA_QUALITY`, not an automatic relative pass |
| `UT-0057-GATES-INDEPENDENT` | `REQ-0057-PROTECTED-RELEASE-GATES` | nDCG `0.99` with one forum below the BM25 margin, then BM25 passing with nDCG `0.949` | Both fail release for different named gates |
| `UT-0057-ABSTENTION` | `REQ-0057-METRIC-ARITHMETIC` | Two good links returned and four abstentions | Precision@6 is `2/6`; coverage is reported separately |
| `UT-0057-CALIBRATION-BINS` | `REQ-0057-EVIDENCE-JUDGE-GOVERNANCE` | Confidence 0.0, 0.1, 0.999 and 1.0 with known outcomes | Assign bins exactly at boundaries, ignore empty bins and include confidence 1.0 in the final bin |
| `UT-0057-SEVERE-FAILURE` | `REQ-0057-PROTECTED-RELEASE-GATES` | High macro nDCG with a new top-ranked grade-0 link | Fail the severe-failure gate despite the average |
| `UT-0057-POWER-INSUFFICIENT` | `REQ-0057-STATISTICAL-EVIDENCE` | Weekly forum sample below the frozen 80%-power calculation | Block promotion before unblinding challenger scores |
| `UT-0057-BOOTSTRAP-PAIRED` | `REQ-0057-STATISTICAL-EVIDENCE` | Fixed queries, seed and unequal champion/challenger query sets | Paired run uses identical resampled identities; unequal inputs are refused |
| `IT-0057-LABEL-IMMUTABILITY` | `REQ-0057-EVIDENCE-JUDGE-GOVERNANCE` | Optuna or a packaged Python runtime attempts a label/threshold write | RLS and release policy refuse it and emit the integrity audit event |
| `IT-0057-REPLAY-IDEMPOTENT` | `REQ-0057-EVIDENCE-JUDGE-GOVERNANCE` | Duplicate evaluation request and crash after judgement before receipt | Original verdict is returned once, with no duplicate charge or label |
| `BDD-0057-MALFORMED-RANKING` | `REQ-0057-METRIC-ARITHMETIC` | One returned ranking repeats a destination and another returns an unjudged destination | Each affected evaluation blocks under `ERR-0057-MALFORMED-RANKING` or `ERR-0057-UNJUDGED-CANDIDATE`; the later duplicate earns no credit, and no unjudged item is skipped or silently graded zero |
| `BDD-0057-KEEP-NOT-RELEASE` | `REQ-0057-PROTECTED-RELEASE-GATES` | Champion and challenger are both below 0.95 and no challenger passes. In a separate synthetic branch retain the committed decision, restart the app and reopen its 0099 N-1430 outcome card. Keep the current Jobs read eligible. Withdraw the authority needed for the protected assessment detail previously seen. Offer the prior view and receipt on reconnect | Outcome is `KEEP_CHAMPION`; release remains refused. The eligible Jobs projection may show the historical decision of that exact run. Hold protected assessment fields and evidence references at the current read check. A saved view unlocks no details, retry or activation. The existing authorised assessment control still shows the failed criterion only within 0057 limits. No sealed rows or labels are exposed |
| `BDD-0057-HELDOUT-ROTATION` | `REQ-0057-STATISTICAL-EVIDENCE` | Repeated weekly candidates have exhausted the permitted hidden-set use | A new governed rotation is required; difficult forums remain eligible |
| `BDD-0057-NO-POSTHOC-EXCLUSION` | `REQ-0057-PROTECTED-RELEASE-GATES` | One difficult forum fails after scores are visible | It cannot be removed; release fails and records the forum |
| `BDD-0057-OUTSIDE-JUDGE-OFF` | `REQ-0057-EVIDENCE-JUDGE-GOVERNANCE` | Outside judge route is disabled | No pack or credential leaves local processing and no outside judge is constructed |
| `BDD-0057-RELEASE-EVIDENCE` | `REQ-0057-PROTECTED-RELEASE-GATES` | Any one release criterion is missing or stale. For the family-count cases, bind the synthetic protected policy profile that uses the 0079 floor. Keep quality and long-lane checks passing in three separate cases. An authorised reader opens the assessment screen and runs `sutra release assess` with a signed release-tier 0064 diagnostic ledger for the exact candidate and one 0079 family below its floor, then with a correctly signed release-tier 0064 ledger for another candidate. In the third case provide only 0055 catalogue-stage counts for the exact candidate, with no signed release ledger. | Release stays refused and names the affected criterion and the permitted evidence reference where available. Both surfaces report the ranking-method floor separately from the passing checks. Name the deficient family only for the verified shortfall. For the substituted ledger, give the candidate mismatch as the reason and expose no family outcome from that ledger. In the catalogue-only case, report missing signed evidence. Display no family result or qualified count from the catalogue figures. Use the existing access checks and redaction for all displayed evidence. |
| `UT-0057-FIELD-ACCEPTANCE` | `REQ-0057-PROTECTED-RELEASE-GATES` | Reference has 150 approvals. Compare complete scopes with 149, 150 and 151 approvals, then a zero reference. Feed a duplicate committed approval, a rejected action with no reason, a pending undo that completes after the frozen cut, and a separate gone or conflict result. | The first three field results are fail, pass and pass. Zero cannot admit release. Count a committed choice once, exclude the pending undo at the original cut and count its later terminal removal once at the next eligible cut. Gone and conflict never count. |
| `BDD-0057-FIELD-ACCEPTANCE` | `REQ-0057-PROTECTED-RELEASE-GATES` | Start with a staged result, then a verified final release. On a later candidate try 199 eligible decisions, an overlapping window, 200 valid but cherry-picked events, a changed site or workflow, a revoked right and an exact complete later receipt in separate scenarios. | Only verified final publication makes later field proof required. Each inconsistent or incomplete path returns SUT-855 with no pass. The exact later receipt verifies only this criterion. A failed synthetic check still refuses the release. |
| `IT-0057-FIELD-CUSTODY` | `REQ-0057-PROTECTED-RELEASE-GATES` | Give the protected assessor an exact qualified offline aggregate, the same successful delivery again, and wrong-run, wrong-recipient, individual-event or no-longer-authorised variants. Revoke rights before signing and, in a separate case, after signing but before publication. | Match current proof and one result only. Refuse invalid evidence without production detail. Stop signing on earlier revocation and hold publication in both revocation cases. |

Property tests cover ties, all-equal grades, empty pools, maximum pools,
duplicate identities, grade boundaries, exact 0.95 and 1.20 thresholds,
and every arithmetic edge. Formal or model-checked evidence covers the
release-decision invariant that release occurs only when every required
criterion has current passing evidence.

## Research provenance

The graded cumulative-gain metric follows Järvelin and Kekäläinen, DOI
[10.1145/582415.582418](https://doi.org/10.1145/582415.582418). BM25,
RRF and diversity candidates retain their citations in specs 0027 and
0077. The Clopper–Pearson method, where used for a reported binomial
interval, follows its original publication and is not substituted for the
paired promotion comparison. Every statistical implementation has golden
vectors and independent numerical review.

These sources provide methods, not product thresholds or legal clearance.
The 0.95 and 1.20 floors are owner decisions. Dataset and judge licences,
consent, access and retention are recorded under spec 0076.
