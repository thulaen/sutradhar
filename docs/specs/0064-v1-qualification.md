# 0064: Version 1 scope and qualification

Version 1 includes every approved non-alias feature in the
[feature register](feature-register.md), every declared supported
variant and the expanded scope below. Phase 6 builds the features.
Phase 7 qualifies the exact release candidate against AGENTS.md
section 3 and this specification.

The former numerical method, capability and variant quotas are removed by
ADR-0020. Qualification proves complete feature outcomes and protected
release properties. It does not award release credit for increasing a
count, adding wrappers or renaming methods.

Spec 0076 owns implementation readiness and stable identities. Spec 0057
owns ranking metrics and release assessment. Spec 0077 owns optimisation
and champion control. Spec 0005 owns release coordination and signing.
Spec 0019 owns durable execution and reconciliation.

## Qualification authority and verifier

The `qualify` crate in `tools/qualify` is the only Version 1 scope
verifier. It runs only on Dell, is never shipped and has no production
dependencies. It reads the feature register, owning specifications,
protected release policy and signed run evidence. It cannot modify source,
labels, thresholds, policy, tasks or release state.

The verifier emits a signed `FeatureQualificationLedger`. A command,
screen, registered job or release controller reads the ledger only through
the shared validator. No caller may recount evidence for itself or select a
subset that hides a failed feature, variant, forum or target.

The ledger proves only the qualification evidence. Spec 0005 uses it
before release approval. The ledger cannot approve, sign or tag.

## What the product covers

| Area | Work and owning designs |
|---|---|
| On-page and technical search | Specs 0044, 0045, 0047 and 0048 own audit and backlink uses, speed diagnosis, remedies and their evidence. Spec 0082 owns the shared Leviathan crawler, including fetch admission and block-page handling. Specs 0115 and 0116 own rendering, audit custody and crawler verification under their own admission rules. The feature catalogue owns enterprise search workflows across content, visibility, crawl, links, competitors, experiments, local presence and reporting. |
| Editorial coaching | Specs 0041 and 0042 own the SEO Editorial Coach and Narada checks. Tathya compares passages under 0042 and stores no fact list. Advice binds its exact passage, page or draft revision and evidence under the owning route. An edit or publication uses that route’s separate staff or moderator authority. Shared text checks keep their owning computation and identity. |
| Text and meaning | Specs 0024 and 0026 own text mining, linguistic analysis, BGE/spaCy artefact admission and feature quality. The pipeline covers segmentation, morphology, parts of speech, syntax, entities, entity linking, coreference where an admitted packaged component supports it, sentiment, relations and declared custom-trained packaged pipelines. Every public operation and downstream annotation has language-specific quality evidence. |
| Knowledge and graph analysis | Spec 0049 owns entity and site graphs. [Spec 0066's Jaal](0066-ganit.md#jaal-graph-analysis) owns the Rust centrality, community, structure and link-prediction algorithms. The [job 256 scope trace](../planning/test-and-traceability.md#jaal-link-prediction-in-the-expanded-scope) binds the link-prediction work to its own evidence. NetworkX public operations are research references, not a claim of strict clean-room work. Pagdandi-style bounded random walks enter only through method-lab admission in spec 0056 and held-out comparison. |
| Link retrieval, ranking and protection | Specs 0022, 0023, 0024, 0026, 0027, 0053, 0055, 0056 and 0077 own full-body reading, BGE retrieval, cross-encoder reranking, spaCy features, learning, calibration and champion control. Spec 0027 owns the single recommendation path and the 3–6 suitable-destination gate. Buddhi and Rakshak supply evidence only through their declared owners. |
| First-party and enterprise analytics | Specs 0034 and 0040 own events, attribution, segmentation, funnels, cohorts, journeys, retention and source connectors. Analysis joins to the page or content version it describes. Provenance, sample size and uncertainty travel with every ranking signal. |
| Research and bounded optimisation | Specs 0012, 0013, 0055 and 0056 own research cards, evaluator qualification, research memory and method admission. Spec 0077 is the sole Optuna and promotion path. Spec 0053 supplies governed feedback. No separate settings tuner or activation route exists. |
| Agent operations | Spec 0014 owns the settings registry and discovery. Spec 0020 owns scoped MCP, `sutra`, change requests, permission checks and live confirmation. Every action and setting has an owner, reference documentation and machine-readable discovery. |
| Events and background work | Spec 0019 owns the product transactional outbox, PostgreSQL queue, Rust scheduler, recovery and migration. Spec 0110 owns the shared framework primitives. The XenForo change feed remains a separate site-side contract in 0021 and does not supply a second product job queue. |

Older and working names point to one owning design. A name never creates
a second home.

| Name | Owning design |
|---|---|
| Leviathan | Shared crawler and block-page handling, spec 0082 |
| Narada | Coach editorial checks, spec 0042 |
| Tathya | Contradiction tip, spec 0042; no stored fact list |
| Nyaya | Research cards and independent checking, spec 0013 |
| Smriti | Research memory and concept map, spec 0013 |
| Sanchay | Research memory store, spec 0013 |
| Prayogshala | Test room, spec 0039 |
| Subramanian | Web link study, spec 0046 |
| Alekh | Search charts, spec 0043 |
| Buddhi | Ranking model served by `shodhan.rank`, spec 0027 |
| Rakshak | kavach spam model used as a ranking signal, spec 0027 |
| Pagdandi | Graph-walk candidate for lab admission, specs 0049 and 0056 |
| NetworkX | Public graph-operation reference; licensed use and strict clean room are not conflated |

## One home for each behaviour

Every non-alias feature has exactly one owning specification and one
derived build task. A feature may reference shared requirements and test
contracts, but its own readiness record must contain its observable
behaviour, applicability, variants, errors, operations and acceptance
evidence. A title or generic range checklist is insufficient.

An alias points to its successor and receives no independent
implementation or qualification row. Desktop, CLI, MCP and platform
wrappers for the same behaviour share its identity. A shared computation
is defined once and referenced by stable requirement and design IDs.

Two entries conflict when they prescribe incompatible results for the same
input and context. They overlap when both own the same decision or side
effect. They duplicate when they define the same contract independently.
Shared references are the required DRY mechanism; copied normative text is
not.

Zero conflicts means zero known unresolved contradictions in the complete
audited inventory at the recorded revision. It is not a claim that no
undiscovered defect exists.

## Required qualification outcomes

A release candidate passes scope qualification only when all of these
hold simultaneously:

1. Every approved non-alias feature resolves to a populated spec 0076
   readiness record and one derived build task.
2. Every requirement and design contract traces to at least one
   feature-specific unit test and BDD scenario, plus applicable
   integration, end-to-end, property, operational and failure evidence.
3. Every declared supported platform, language and input variant passes on
   every required target. Unsupported or inapplicable cases follow their
   explicit contract and machine-readable reason.
4. No orphan, duplicate owner, conflicting contract, undocumented fallback,
   placeholder or untraceable task or test remains in the frozen inventory.
5. Source integrity, guest visibility, full reading, tenant isolation and
   moderator approval pass before ranking or public-write evidence is
   accepted.
6. nDCG@10 is at least 0.95 and every eligible forum independently meets
   the 1.20 times locked-BM25 Precision@6 gate under spec 0057.
7. Link suitability, severe failures, calibration, diversity, privacy,
   resource admission, p95 latency, recovery and site-footprint gates pass.
8. The 3–6 suitable-internal-destination rule is preserved. Ranking or
   diversity cannot force an unsuitable destination into the set.
9. Every required review, strict-core coverage rule and qualification
   signature binds the exact candidate and evidence set.
10. All approved features and expanded scope are present. Spec
    0079's evidence binds the exact candidate and each distinct
    qualified method used by its shipped pipeline to one family.
    The verifier refuses duplicates, omissions and a family below
    its floor even if the total passes. Quality also passes.
11. Give the bound qualification evidence to the spec 0005 release
    coordinator. It checks the long lane, obtains exact approval and
    verifies signed artefacts before publication.

## Freeze the run

For a release run, use the 0005 scope preflight and exact-candidate
rechecks for the Phase 7 entry prerequisites under 0076. The 0010
phase verifier and 0139 remain the authority for Phase 3 closure
and the adopted Boot qualification. Check their applicability to
the release dependencies through the existing protected context.
A scope result or pilot cannot substitute for missing or
superseded phase proof.

The protected release policy issues a unique run id before execution. The
run kind is `release`, `pilot` or `fixture`; its purpose is one of
`qualification`, `gate`, `nightly`, `soak` or `replay`. The policy, not a
submitted label, determines both.

The run manifest freezes:

- exact candidate commit, source-tree digest and build artefacts;
- feature-register, specification, requirement, design and task digests;
- supported-variant and applicability register;
- dependency, model, tokenizer, spaCy pipeline and configuration locks;
- metric protocol, relevance labels and BM25 baseline;
- execution targets, site versions, languages, hardware and resource
  profiles;
- required check and review matrix;
- toolchain, verifier, runner supervisor and trusted harness digests;
- authorised signer registry and protected policy digest; and
- canonical encoding version and manifest SHA-256.

The protected policy and signer registry are loaded outside the candidate.
Submitted evidence cannot replace them, select an older policy, grant a
scope or choose a different trust root. Unknown, revoked or out-of-scope
keys and policy rollback are refused.

The candidate runs in a sealed job without signing keys. The trusted
supervisor observes the approved invocation, exit and evidence. Only its
authenticated result can become a signed check receipt. A pass printed by
candidate code is not evidence.

A pilot has a separate signing purpose and cannot qualify a release. A
fixture uses only `FX-` identities and cannot reference release evidence.

## Feature evidence records

Each feature evidence record contains:

| Field | Required content |
|---|---|
| Identity | Canonical `F-` id, feature version and owning specification |
| Trace | Requirement, design, variant, interface, error and derived-task IDs |
| Applicability | XenForo, WordPress, language, input and operating-mode declarations |
| Implementation | Exact artefact and source digests that realise the contracts |
| Verification | Runner-signed check, target, outcome and evidence digest for every required cell |
| Operations | Install, migration, recovery, rollback, retention and footprint evidence where applicable |
| Provenance | Source classifications, licences, research and legal-review references |
| Review | Independent reviewer identities, disciplines, outcomes and exact content fingerprints |
| Side effects | Last artefact reference, output digest and idempotency evidence for writes |

Verify the required review identity, discipline and expected scope
from the frozen review matrix. A member review needs that member's
own authenticated review record under 0065. Check its exact content
fingerprint, current signer, independence and verdict. For a required
feature, variant or operation scope, resolve that subject through the
existing protected evidence references under [0076][scoped-review].
A whole-change review retains its own credit. A valid sibling review,
unsigned scope label or unresolved subject cannot satisfy a finer
obligation. Hold the affected qualification if its admitted reader
cannot prove that binding. The existing signed encodings and review
authority continue to apply.

[scoped-review]: 0076-specification-completeness.md#risk-based-assurance

A variant record names its `VAR-` id, owner, admitted input predicate and
required targets. It passes only when its owning feature also passes.
Several checks for the same feature and variant do not create another
feature outcome. A wrapper, preset, template, seed or configuration value
does not create one either.

An inapplicable variant requires its approved reason, observable behaviour
and test evidence. It is not silently marked unsupported after a failure.

A ranking-signal feature additionally records its relevance hypothesis,
provenance, freshness, confidence, evaluation and opt-out where required.
Its evidence must show that it cannot promote an off-topic or otherwise
ineligible destination.

## Evidence groups and disjointness

Evidence is organised into signed groups. Each group manifest binds the
run, exact feature and variant commitments, evidence commitment, encoding
and verifier version. Every feature's results for one target matrix reside
in one group. A feature may not be split to hide a failed cell.

Groups are disjoint by canonical feature identity. Shared implementation
evidence is referenced by digest from each relevant feature, while the
feature-specific contract and distinguishing test remain separate. The
verifier rejects:

- the same feature independently qualified in two groups;
- an alias or second surface presented as another feature;
- two owners for one behaviour or side effect;
- copied contracts with different IDs but no distinguishing behaviour;
- an admitted method or signal that no shipped pipeline invokes;
- a result for another candidate, target, specification or policy digest;
- a group total, unchecked range or sample offered in place of individual
  feature results; and
- a location-based evidence reference that is not resolved by digest from
  the protected evidence store.

A digest collision with different bytes is a refusal, never a match.

## Attempt history and freshness

The protected Dell run ledger records every attempt and final verdict.
For each feature, check and target, derive the final verdict from every
required cell of the frozen matrix. Within each cell, check its exact
variant, test, fixture, input, profile and tier under the
[test and traceability plan](../planning/test-and-traceability.md).
Resolve its outcome and ordered attempts from the authenticated runner
evidence and its existing digest references in the protected evidence
store. Bind retry authority to the exact affected cells under the
frozen policy. An interruption in one cell gives no retry authority
for a sibling case. One supervised invocation can supply multiple
separately verifiable case results. An aggregate pass cannot replace
those results. If the evidence cannot identify a unique cell and its
ordered attempts, hold qualification even when another case shares
its check and target. This check uses the existing signed records.

Only the trusted supervisor can classify an infrastructure interruption.
A candidate-caused exit, timeout or resource-limit failure is a test
failure. A test failure on the same candidate and evidence cell blocks
that cell; a later pass cannot erase it. A new corrected candidate needs
a new identity and complete qualification binding.

The scope ledger refuses when:

- its run kind is not `release` when read by the release gate;
- its candidate, catalogue, specification or policy digest differs from
  the candidate being judged;
- a newer refused or interrupted attempt exists for the same release
  candidate;
- a newer relevant test failure exists;
- a newer valid release ledger supersedes it; or
- the run ledger or signer registry cannot be read.

Passing gate, nightly or soak attempts do not by themselves stale a
release ledger. Their failures and all refused or interrupted runs remain
part of history. Submitted data cannot choose exclusions.

## Canonical ledger and signature contract

The ledger contains:

```text
format_version, record_type, signing_purpose, run_id, run_kind,
run_purpose, run_manifest_digest, candidate_digest, feature_register_digest,
specification_set_digest, protected_policy_digest, previous_run_ledger_head,
current_run_ledger_head, sequence, feature_evidence_commitment,
variant_evidence_commitment, method_evidence_commitment,
outcome, refusal_reasons, evidence_digest
```

`outcome` is `PASS` or `REFUSE`. A `REFUSE` ledger is diagnostic and
cannot authorise release. `sequence` increases monotonically for one
candidate and run kind. No caller-supplied count exists. The policy
requires the spec 0079 evidence digest in the release ledger format.
Verify that evidence against the candidate and rederive all counts.

Records use a canonical binary encoding: unsigned 64-bit big-endian byte
lengths prefix variable-length fields; UTF-8 is mandatory for text;
digests are 32 raw bytes; sequences carry their exact element count and
ordered records. The format version fixes field order. Unknown versions,
ambiguous values, trailing bytes, invalid UTF-8 and oversized lengths are
refused before allocation.

The signing service signs the canonical bytes under the scope-ledger
purpose. Feature results, reviews and group manifests use distinct
purposes. A valid signature under the wrong purpose is refused.

IT-0064-METHOD-FLOOR holds the total above the floor with one
family at 11. Try a setting-only duplicate, a method unused by the
shipped pipeline and another candidate's digest. Each refuses.
In a separate case, offer a historical spec 0067 `MC-` result
as the missing member. Keep its historical record type. Sign
its test bytes for the exact candidate with the fixture key and
bind the same computation as a current qualified method in the
short family. Separately offer a synthetic distinct computation
with no current 0079 method admission. In both cases refuse
the historical record as method evidence. It adds no method
and the short family still blocks qualification.

In the Family 29 case, give each of the other 29 families 13
synthetic qualified methods and give Family 29 seven. The total
is 384, above the floor. Offer the six deferred RM-29 entries
with otherwise valid test-signed evidence in their separate
scope. The version 1 count stays at seven for Family 29 and
384 overall. The response names the short family and blocks
qualification. In a separate case add five independently
eligible and qualified Family 29 methods in the synthetic
fixture, each called by the shipped fixture pipeline. The
12-method family check passes. Apply the other checks as usual.

## Input and resource limits

The protected run policy fixes maximums for groups, manifests, encoded and
decoded bytes, feature and result rows, referenced objects, bytes fetched,
scratch space, execution time and runner memory. An absent or unbounded
limit refuses the run. Evidence cannot raise a limit.

The verifier bounds reads before authentication, then streams groups under
those limits. It checks every length against remaining bytes before
allocation or expansion, charges retries and scratch writes to the run and
stops at the first excess or incomplete field. A refused run leaves the
last verified ledger intact and removes partial output.

Resource admission covers the verifier, evidence store, Rust services and
packaged Python runtimes together. No first-party Python production
verifier or adapter is permitted.

## Replay

Replay starts from a trusted policy-issued run id and reopens the frozen
inputs with a fresh verifier. It validates the previous ledger under the
original trusted run context, then repeats membership, applicability,
signature, freshness, attempt-history and conflict checks.

Replay preserves the original run kind, purpose and signing purpose. It
cannot turn pilot or fixture evidence into release evidence. The trusted
supervisor supplies the current replay invocation and ordered attempt ids;
only those attempts are excluded from newer-attempt freshness. Their own
failures are never excluded.

Replay refuses if the protected policy, frozen inputs, feature set,
required targets or evidence bytes changed, if any new disqualifying
attempt appears, or if any replay attempt fails. Success returns the same
per-feature outcomes in a new ledger bound to the exact new run-ledger
head and next sequence. The previous file remains immutable.

## Provenance and legal gates

The enterprise research inventory names Adobe Analytics, Mixpanel,
Semrush Enterprise, Conductor, Piano, Optimizely, Botify, Ahrefs, Lumar and
BrightLocal. Research records link public descriptions to the exact agreed
workflow and owning design. They do not establish licence, patent
clearance or freedom to operate.

Public references include:

- [graph walks](https://arxiv.org/abs/1711.07601);
- [NetworkX graph operations][networkx];
- [CoreNLP linguistic analysis](https://stanfordnlp.github.io/CoreNLP/);
- the BGE papers and publisher model cards in spec 0077; and
- the locked spaCy and Optuna documentation and spec 0019's
  queue and recovery contract.

Each implementation is classified as original design, public standard or
published algorithm, licensed dependency, permitted interoperability
observation or strict clean-room reimplementation. Researcher/implementer
separation applies only to the final class. Licensed package use is not
called strict clean room.

`LEG-0064-V1-LEGAL` requires the scoped patent search,
freedom-to-operate analysis and qualified legal review where flagged.
Engineering evidence cannot mark that obligation complete.

## Fixture and pilot design

A fixture uses a separate fixture catalogue and only `FX-` identities.
It includes:

1. one feature passing on two declared targets;
2. one feature with an inapplicable variant and its approved reason;
3. one feature with a failed distinguishing test;
4. one alias pointing to the first feature;
5. one shared implementation referenced by two feature-specific contracts;
6. one ranking-signal feature with an off-topic destination; and
7. one feature whose evidence is bound to the previous candidate.

The independently expected result is refusal until every required
non-alias feature and variant passes. The alias adds no row, shared code
does not erase feature-specific tests and the off-topic signal cannot
pass. Correcting evidence without changing the candidate cannot erase a
recorded failure; a new candidate identity is required.

The pilot uses real representative features from every scope area and all
supported target classes. It exercises signing, sealing, replay, failure
history, independent review and resource measurement. It produces no
release credit.

## Requirement, design and task traceability

These identities group the normative contracts above. They do not show
execution state or admit an implementation task. Derived tasks still
follow spec 0076's readiness gate.

| Requirement | Normative obligation | Design and derived task | Unit evidence | BDD evidence |
|---|---|---|---|---|
| `REQ-0064-FEATURE-INVENTORY-AND-PROVENANCE` | Every approved non-alias feature has one owner and derived task, aliases add no outcome, shared computations retain feature-specific evidence and provenance remains truthful; no count substitutes for completeness | `DES-0064-QUALIFICATION-PLATFORM`; `TASK-0064-QUALIFICATION-PLATFORM` | `UT-0064-FEATURE-MISSING`, `UT-0064-ALIAS`, `UT-0064-DUPLICATE-OWNER`, `UT-0064-SHARED-CONTRACT` | `BDD-0064-ALL-FEATURES`, `BDD-0064-NO-COUNT-SUBSTITUTE`, `BDD-0064-PROVENANCE` |
| `REQ-0064-VARIANT-APPLICABILITY` | Every declared supported variant passes its required targets, and only predeclared inapplicable cases with approved reasons and behaviour evidence are accepted | `DES-0064-QUALIFICATION-PLATFORM`; `TASK-0064-QUALIFICATION-PLATFORM` | `UT-0064-VARIANT-MISSING`, `UT-0064-INAPPLICABLE`, `UT-0064-POSTHOC-UNSUPPORTED` | `BDD-0064-VARIANT-MATRIX` |
| `REQ-0064-PROTECTED-RELEASE-GATES` | Scope evidence cannot waive the nDCG and per-forum BM25 floors, the 3–6 suitable-destination rule or the relevance gate. The exact candidate must also meet the separate 0079 method-family floor through current admission evidence | `DES-0064-QUALIFICATION-PLATFORM`; `TASK-0064-QUALIFICATION-PLATFORM` | `UT-0064-RANKING-FLOOR`, `UT-0064-BM25-FLOOR`, `UT-0064-DESTINATION-GATE`, `UT-0064-RELEVANCE-SIGNAL` | `BDD-0064-PROTECTED-GATES` |
| `REQ-0064-TRUSTED-QUALIFICATION-EVIDENCE` | Trusted run context, candidate binding, failure history, authorised retries, signing purposes, independent reviewers, canonical decoding and bounded resources govern qualification and replay | `DES-0064-QUALIFICATION-PLATFORM`; `TASK-0064-QUALIFICATION-PLATFORM` | `UT-0064-STALE-CANDIDATE`, `UT-0064-FAILURE-HISTORY`, `UT-0064-INFRA-RETRY`, `UT-0064-WRONG-PURPOSE`, `UT-0064-REVIEWER-AUTHOR`, `UT-0064-TRUST-ROOT`, `UT-0064-ENCODING`, `UT-0064-RESOURCE` | `BDD-0064-TRUSTED-REPLAY` |

| Design | Owning contract | Requirements and derived task |
|---|---|---|
| `DES-0064-QUALIFICATION-PLATFORM` | The qualification authority through the reporting, agent-configuration and footprint contracts above | `REQ-0064-FEATURE-INVENTORY-AND-PROVENANCE`, `REQ-0064-VARIANT-APPLICABILITY`, `REQ-0064-PROTECTED-RELEASE-GATES`, `REQ-0064-TRUSTED-QUALIFICATION-EVIDENCE`; `TASK-0064-QUALIFICATION-PLATFORM` |

| Derived task | Implementation boundary | Upstream identities |
|---|---|---|
| `TASK-0064-QUALIFICATION-PLATFORM` | Implement the Rust-owned scope verifier, protected evidence validation and replay boundary without a second qualification path or first-party Python production code | `DES-0064-QUALIFICATION-PLATFORM`; `REQ-0064-FEATURE-INVENTORY-AND-PROVENANCE`, `REQ-0064-VARIANT-APPLICABILITY`, `REQ-0064-PROTECTED-RELEASE-GATES`, `REQ-0064-TRUSTED-QUALIFICATION-EVIDENCE` |

## Verification cases

The following are required test designs, not claims that executable tests
have run.

| Test id | Requirement | Given | When | Then |
|---|---|---|---|---|
| `UT-0064-FEATURE-MISSING` | `REQ-0064-FEATURE-INVENTORY-AND-PROVENANCE` | One approved non-alias feature lacks a readiness row | Qualification runs | Refuse and name the feature |
| `UT-0064-ALIAS` | `REQ-0064-FEATURE-INVENTORY-AND-PROVENANCE` | An alias has separate evidence | Groups are checked | Alias does not create another feature outcome |
| `UT-0064-DUPLICATE-OWNER` | `REQ-0064-FEATURE-INVENTORY-AND-PROVENANCE` | Two specifications claim the same public write | Traceability runs | Refuse before execution evidence is accepted |
| `UT-0064-SHARED-CONTRACT` | `REQ-0064-FEATURE-INVENTORY-AND-PROVENANCE` | Two features reference one shared computation but have different observable contracts | Evidence is checked | Shared implementation is allowed; both feature-specific tests remain required |
| `UT-0064-VARIANT-MISSING` | `REQ-0064-VARIANT-APPLICABILITY` | A feature passes XenForo but lacks its declared WordPress target. Separately, keep the same feature, check and target for two required cases. Supply exact signed evidence that the refusal case passed and omit the recovery case's evidence. | Qualification checks the frozen matrix and resolves the evidence references | Refuse without dropping the target. In the second case keep the verified refusal result in its own case and name the missing recovery case. |
| `UT-0064-INAPPLICABLE` | `REQ-0064-VARIANT-APPLICABILITY` | A declared inapplicable variant has an approved reason and passing behaviour test | Evidence is checked | Accept only that declared path |
| `UT-0064-POSTHOC-UNSUPPORTED` | `REQ-0064-VARIANT-APPLICABILITY` | A failed variant is reclassified after results | Manifest is checked | Refuse the changed register and post-hoc exclusion |
| `UT-0064-RANKING-FLOOR` | `REQ-0064-PROTECTED-RELEASE-GATES` | Every feature passes except nDCG@10 is below 0.95 | Release assessment joins | Scope evidence cannot compensate; release refuses |
| `UT-0064-BM25-FLOOR` | `REQ-0064-PROTECTED-RELEASE-GATES` | nDCG passes but one forum misses the BM25 margin | Release assessment joins | Refuse and name the forum |
| `UT-0064-DESTINATION-GATE` | `REQ-0064-PROTECTED-RELEASE-GATES` | A candidate returns seven destinations or an unsuitable sixth | Qualification runs | Refuse under spec 0027 |
| `UT-0064-RELEVANCE-SIGNAL` | `REQ-0064-PROTECTED-RELEASE-GATES` | A popular but off-topic destination receives a high analytics score | Ranking evidence is checked | Relevance gate rejects it |
| `UT-0064-STALE-CANDIDATE` | `REQ-0064-TRUSTED-QUALIFICATION-EVIDENCE` | Correctly signed evidence names the previous commit | Candidate binding runs | Refuse |
| `UT-0064-FAILURE-HISTORY` | `REQ-0064-TRUSTED-QUALIFICATION-EVIDENCE` | A failed check is followed by a passing retry on the same candidate. In another branch, two required cases share a feature, check and target. The first case fails and a correctly signed result for the second passes. Offer the second result for both, then give a signed aggregate pass with no distinct case evidence. | Check ordered attempts and exact resolved case outcomes | The original failure remains blocking. The second case keeps its own pass and the aggregate pass supplies no missing case or authority. |
| `UT-0064-INFRA-RETRY` | `REQ-0064-TRUSTED-QUALIFICATION-EVIDENCE` | The trusted supervisor approves an interruption and retry for one cell. Try attaching that authority to a sibling with the same feature, check and target. In a separate authorised invocation for both cells, verify signed evidence that identifies each case result. | Apply the frozen retry policy to the affected cells | Reject the sibling retry. Shared invocation can supply each independently identified result. All required cells still govern the aggregate. |
| `UT-0064-WRONG-PURPOSE` | `REQ-0064-TRUSTED-QUALIFICATION-EVIDENCE` | A runner signature is reused as a review or ledger signature | Purpose is checked | Refuse |
| `UT-0064-REVIEWER-AUTHOR` | `REQ-0064-TRUSTED-QUALIFICATION-EVIDENCE` | Reviewer and author are the same principal under different keys. In a separate branch, give the correct independent member and exact subject. Offer the runner's valid `review-result`, then the member's own valid `review-record`. | Check reviewer identity and the owning record purpose | Refuse the self-review and the runner substitute for a member review. The exact member record clears only the identity and purpose checks. |
| `UT-0064-TRUST-ROOT` | `REQ-0064-TRUSTED-QUALIFICATION-EVIDENCE` | Submitted manifest supplies a replacement signer registry | Verification starts | Refuse before opening feature evidence |
| `UT-0064-ENCODING` | `REQ-0064-TRUSTED-QUALIFICATION-EVIDENCE` | Inputs contain truncated fields, trailing bytes, invalid UTF-8 and a declared length of 2^64 minus one | Decoder runs | Refuse before allocation and preserve prior ledger |
| `UT-0064-RESOURCE` | `REQ-0064-TRUSTED-QUALIFICATION-EVIDENCE` | Individually valid groups exhaust the whole-run fetch budget | Verification runs | Refuse and retain no partial ledger |
| `IT-0064-REPLAY` | `REQ-0064-TRUSTED-QUALIFICATION-EVIDENCE` | A valid run is replayed with identical trusted inputs | Fresh verifier runs | Same feature outcomes and new bound sequence result |
| `IT-0064-REPLAY-CHANGE` | `REQ-0064-TRUSTED-QUALIFICATION-EVIDENCE` | Policy, feature register or one evidence byte changes before replay. In a separate case keep the frozen release inputs and the authorised replay invocation and ordered attempt IDs fixed. | Replay runs. In the separate case, after the verifier observes the initial run-ledger head and before it binds the new head, append an independently recorded refused attempt for the same release candidate outside the replay invocation. | Refuse. The separate case yields no new passing ledger. Preserve the previous file bytes and refuse its release use under the newer-attempt freshness rule. |
| `IT-0064-PILOT-READ` | `REQ-0064-TRUSTED-QUALIFICATION-EVIDENCE` | Correctly signed pilot or fixture ledger is offered to release | Reader validates kind | Refuse release use |
| `IT-0064-METHOD-FLOOR` | `REQ-0064-PROTECTED-RELEASE-GATES` | Use the short-family, duplicate, unused-method, wrong-candidate and historical record cases under [Canonical ledger and signature contract](#canonical-ledger-and-signature-contract) | Validate the current spec 0079 evidence for the exact candidate and rederive the family counts | Refuse the affected release qualification. Historical or otherwise ineligible evidence supplies no qualifying method. A passing total or ranking result gives no pass to a short family |
| `BDD-0064-ALL-FEATURES` | `REQ-0064-FEATURE-INVENTORY-AND-PROVENANCE` | Every approved feature and variant has passing signed evidence | All independent release gates pass | Scope qualification emits `PASS`; exact candidate and policy remain bound |
| `BDD-0064-NO-COUNT-SUBSTITUTE` | `REQ-0064-FEATURE-INVENTORY-AND-PROVENANCE` | Many admitted methods exist but one approved feature is missing | Qualification runs | The result is `REFUSE`; no count or percentage can waive the omission |
| `BDD-0064-PROVENANCE` | `REQ-0064-FEATURE-INVENTORY-AND-PROVENANCE` | A licensed spaCy package is described as strict clean room | Provenance is checked | The false classification blocks admission; package licence and actual provenance remain visible |
| `BDD-0064-VARIANT-MATRIX` | `REQ-0064-VARIANT-APPLICABILITY` | Supported variants lack one required target and an inapplicable variant has its approved evidence | Qualification evaluates the frozen variant register | Missing target evidence refuses; only the declared inapplicable path is accepted and no post-hoc reclassification occurs |
| `BDD-0064-PROTECTED-GATES` | `REQ-0064-PROTECTED-RELEASE-GATES` | Feature evidence is complete but a ranking, destination-count or relevance protection fails | Release assessment joins scope qualification | Refuse and name each failed protection; feature completeness cannot compensate |
| `BDD-0064-TRUSTED-REPLAY` | `REQ-0064-TRUSTED-QUALIFICATION-EVIDENCE` | A signed candidate has attempt history and a replay request. Use 0076 `FX-0076-REVIEW-SCOPE` in a separate branch with two required scopes, complete test evidence and a valid whole-change review. Where a synthetic admitted reader can resolve scoped member reviews, bind A and B independently. | Replay the trusted context. In the scope branch, omit B's subject binding, add an unsigned mapping, or offer A's valid review for B. Compare with the exact B review under its admitted scope. | Only authorised retries apply, recorded failures remain blocking and changed inputs, purposes or trust roots refuse without replacing the prior ledger. A valid review keeps its exact credit. An unproved B obligation holds qualification and produces no new passing ledger. The exact B review clears only that obligation. The fixture provides no signed qualification. |
| `E2E-0064-RELEASE` | `REQ-0064-TRUSTED-QUALIFICATION-EVIDENCE` | Freeze, execute, sign, verify and replay a representative candidate | End-to-end qualification runs | Every accepted row resolves to real execution evidence and every planted omission refuses |
| `OPS-0064-STORE-DOWN` | `REQ-0064-TRUSTED-QUALIFICATION-EVIDENCE` | Evidence store or run ledger is unavailable | Verification is requested | Fail closed without using cached unverified counts or outcomes |

In BDD-0064-TRUSTED-REPLAY, include the spec 0015 AT-MR19
ceiling change as a required synthetic fixture in the frozen matrix.
Offer an authentic result for its static shadow branch in place of
the changed-settings branch with the same feature, check and target.
The result keeps its original case credit. The changed-settings
branch requires its own resolved input and ordered attempt evidence.
Hold the missing branch under the existing cell rules.

Property tests generate feature graphs, aliases, shared contracts,
variant matrices and attempt histories. They prove that no input produces
`PASS` with an orphan, duplicate owner, missing required cell or failed
mandatory gate. Mutation tests cover every verifier comparison and signing
purpose under the strict-core assurance policy.

## Reporting

The report lists each feature and variant outcome, refusal reason,
owning design, evidence digest and target. It reports elapsed time by
queue, execution, review, signing, transfer, verification and retry, plus
peak memory, storage and replay time on named hardware.

Totals may describe coverage of the audited inventory, but they are not a
release quota. A high percentage with one missing required feature remains
a refusal. Pilot and fixture reports are labelled separately and cannot be
presented as release evidence.

## Agent configuration

Settings discovery is owned by spec 0014. Scoped change requests,
permission checks and live confirmation are owned by spec 0020. Their
features qualify under the same feature evidence contract; they receive no
separate numerical capability credit.

## Footprint budget

This specification adds no work to public pages. Every owning feature
sets and measures its own footprint budget under AGENTS.md section 5.4.

[networkx]: https://networkx.org/documentation/stable/reference/
