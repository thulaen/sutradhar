# 0076: Specification completeness and implementation readiness

## Purpose and authority

This specification owns the meaning of a complete, implementation-ready
design for every Version 1 feature, method, supported variant and
operational procedure. It applies before any implementation resumes and
to every later change.

ADR-0020 owns the decision hierarchy. This specification owns requirement,
design, task and test traceability. The feature register owns feature
identities and aliases. Each numbered product specification owns its
behaviour. Forgejo owns execution state and evidence, not requirements.

Documentation completion does not authorise coding. A complete design is
necessary for implementation but does not bypass the roadmap, release
gates, security controls or owner approval.

Example: a writer retires feature `F-1000` in favour of `F-0933`. The
register keeps `F-1000` as an alias that resolves only to `F-0933`, so a
count of features does not include both (`FX-0076-ALIAS-RETENTION`). Later
a feature is added without a readiness record. Its implementation task asks
for admission and is refused, with the missing owner and record named
(`FX-0076-UNMAPPED-FEATURE`). The owner adds the record. The checker runs
again at a fixed revision and the feature becomes eligible for task
derivation. That is all it becomes: no code, release or legal approval
follows from the check.

## Words used here

- **Readiness record**: the set of sections that makes one feature's design
  complete enough to derive tasks from.
- **Owning specification**: the one spec that defines a behaviour; every
  other spec refers to it by identity.
- **Identity**: a permanent, unique identifier such as `REQ-0076-SSS`.
- **Alias**: a retired identifier kept so that old references still resolve.
- **Traceability**: the two-way links from requirement to design, task and
  test, and back.
- **Fixture**: a documentation-validation input marked `FX-`. It is not
  product evidence.
- **Strict core**: the operations that ADR-0020 puts under eight independent
  reviews and full coverage.
- **Provenance record**: the evidence that says where a method, dependency
  or threshold came from.
- **Placeholder**: wording such as `TBD` that leaves a design to later.

## What this spec designs

| Part | What the reader or checker sees |
|---|---|
| Stable identities | One permanent, unique identifier for each kind of thing |
| The feature readiness record | The sections every non-alias feature must carry |
| Verification design | Test-first traceability, unit, BDD, integration and edge cases |
| Risk-based assurance | Which components are strict core and what that demands |
| DRY, KISS and SOLID | Reviewable checks in place of decorative claims |
| Research provenance | The evidence class for each method, dependency and threshold |
| Conflict, duplication and overlap control | The audit that finds known contradictions |
| Derived task contract | What a Forgejo task must contain before it exists |
| Readiness and prohibited placeholders | What the readiness checker rejects |
| Specification-owned verification | The `FX-` fixtures that test this spec |

## Stable identities

The following identifiers are permanent and unique within their kind:

| Kind | Form | Owner |
|---|---|---|
| Feature | `F-` plus the zero-padded feature number, for example `F-0002` | [Feature register](feature-register.md) |
| Requirement | `REQ-NNNN-SSS`, where `NNNN` is the owning specification | Owning specification |
| Design contract | `DES-NNNN-SSS` | Owning specification |
| Supported variant | `VAR-NNNN-SSS` | Owning specification |
| Interface | `API-NNNN-SSS`, `EVT-NNNN-SSS`, `DB-NNNN-SSS` or `UI-NNNN-SSS` | Owning specification |
| Error | `ERR-NNNN-SSS` | Owning specification |
| Derived task | `TASK-NNNN-SSS` | Owning specification for the stable identity, Forgejo for the issued task number |
| Unit test | `UT-NNNN-SSS` | Owning specification |
| BDD scenario | `BDD-NNNN-SSS` | Owning specification |
| Integration test | `IT-NNNN-SSS` | Owning specification |
| End-to-end test | `E2E-NNNN-SSS` | Owning specification |
| Property or formal obligation | `PRP-NNNN-SSS` | Owning specification |
| Research source | `SRC-NNNN-SSS` | Owning specification |
| Legal review obligation | `LEG-NNNN-SSS` | Owning specification |

`SSS` is a stable, upper-case mnemonic beginning with a letter and
containing only letters, digits and hyphens. Numbers are never recycled
to change meaning. A retired identifier remains as an alias with its
successor and reason. A fixture prefix `FX-` marks non-product evidence;
fixture identifiers cannot qualify a release.

A feature may have many requirements, designs, variants, tasks and tests.
Each has exactly one owning specification. Shared behaviour is defined
once and referenced by stable identity; it is not paraphrased into a
second normative contract.

## The feature readiness record

Every non-alias feature must have a readiness record in its owning
specification. A title, one-paragraph idea, generic checklist or link to
a future range task is not a readiness record. The record contains all
of the following sections, with stable identifiers and concrete content.

### Purpose, actors and boundaries

1. The user problem, actor, preconditions, trigger and observable result.
2. The owning module and public entry point.
3. Explicit exclusions and neighbouring behaviours that must not be
   reimplemented.
4. XenForo, WordPress and applicability behaviour. “Not applicable”
   requires a machine-readable reason and the exact user presentation.
5. Data classes read and written, their authoritative stores, tenant key,
   retention class and deletion behaviour.
6. Whether the feature can affect a public page. If so, its query, time
   and byte footprint budgets and the measurements that enforce them.
7. Any signal supplied to ranking, its relevance hypothesis, provenance,
   freshness, confidence and opt-out. A feature does not automatically
   become a ranking signal.

### Functional contract

For every operation, state the inputs, outputs, preconditions,
postconditions, invariants and side effects. Include:

- exact field names, types, units, ranges, defaults and nullability;
- validation order and canonicalisation;
- state transitions and legal initial and terminal states;
- permission and moderator-approval requirements;
- ordering, concurrency, idempotency and transaction boundaries;
- external calls, timeouts, retries, rate limits and cancellation;
- deterministic behaviour and the source of every random seed;
- audit events, metrics, logs and traces, excluding sensitive content;
- recovery after a crash at every externally visible side-effect
  boundary.

A default is normative only when its value and reason are stated. A
fallback names its trigger, substitute behaviour, operator-visible
signal and recovery path. “Best effort”, “appropriate”, “as needed” and
similar open-ended instructions are not contracts.

### Interface and error contracts

Every public interface includes a versioned schema and examples for a
success and each error class. Interfaces between modules define the
calling module, callee, visibility and compatibility policy. Database
contracts include ownership, constraints, indexes, transaction isolation,
RLS predicate and migration behaviour where applicable.

Every error record defines:

| Field | Required content |
|---|---|
| Identity | Stable `ERR-NNNN-SSS` and machine-readable code |
| Trigger | Exact predicate, including boundary values |
| Detection | The component that detects it |
| Result | Returned error shape and HTTP, IPC, job or event outcome as applicable |
| Side effects | What is committed, rolled back, retried or parked |
| Security | Whether details are redacted and what the caller may learn |
| Recovery | Actor, retry safety and reconciliation procedure |
| Tests | Unit and BDD identifiers that exercise it |

An implementation may not invent an undocumented fallback for an
unrecognised error. Unknown versions, malformed data and partial failure
fail closed where security, integrity or public writes are involved.

### Supported variants

A supported variant is a declared platform, language, model, input shape
or operating mode with distinct observable behaviour or a distinct code
path. Each `VAR` record defines:

- its owner and distinguishing predicate;
- accepted and rejected inputs, with examples;
- dependencies, model or pipeline artefact and licence;
- exact outputs and downstream consumers;
- resource and quality budgets;
- fallback behaviour, if any;
- feature-specific unit, BDD and integration evidence.

A value sweep does not create variants merely to increase a count.
Conversely, a statement such as “multilingual”, “both platforms” or
“large documents” is incomplete until every supported class and its
limits are enumerated. Unsupported cases require explicit refusal or
applicability behaviour; silent degradation is forbidden.

### Operational procedure

Every installation, migration, scheduled job, backup, restore, replay,
cutover, rollback, recovery and destructive operation has a procedure
with:

1. purpose, trigger, actor and required permission;
2. inputs, immutable artefact identities and preflight checks;
3. ordered steps and the observable result of each;
4. concurrency exclusion and resource admission;
5. idempotency key and safe retry point;
6. failure handling, dead-letter or quarantine behaviour;
7. reconciliation and proof of completion;
8. rollback or forward recovery, including its own verification;
9. audit and retained evidence; and
10. BDD and failure-injection tests for interruption at each side-effect
    boundary.

A procedure never instructs an operator or agent to improvise commands,
select an unversioned artefact or treat a log line as proof of a side
effect.

For each migration, cutover or rollback that crosses versions, state
which combinations of caller or binary, interface and stored format
the owning specifications support before, during and after the change.
List the target platform, version or schema identifiers, allowed
operations, and the expected result of each applicable combination.
For a refused combination, name the error, preserved state and
admitted recovery route. Give a reason when a combination cannot occur.
Link each applicable combination to its owning contract and acceptance
case. Use existing compatibility decisions to fill this matrix. A
missing decision holds the affected transition. Listing a combination
grants no additional support.

For each recovery time limit, name the start event, the elapsed-time
source and the observable finish. For any recovery preservation
obligation, name the affected data or effects that must survive.
The acceptance cases measure the time limit and check the required
preserved state.

For example, [0030][upgrade] requires the previous binary to work with
the migrated tables. Its acceptance checks verify that pairing and
measure rollback following a failed self-test, with no lost or repeated
work. An empty-store test of the previous binary exercises neither
requirement.

[upgrade]: 0030-install-upgrade-removal.md#update

## Verification design

### Test-first traceability

Each requirement maps to at least one concrete verification obligation
before implementation. Each test maps back to at least one requirement
and, where relevant, a variant and error. The traceability record includes
the test layer, fixture, exact input, expected observable result and
evidence producer.

The future TDD cycle is:

1. write the named test from the populated requirement;
2. observe its failure for the stated reason on the unmodified baseline;
3. implement the smallest coherent change;
4. run the focused and dependent tests;
5. run required reviews, integration and regression checks; and
6. bind the signed result to the exact source and input digests.

Writing a test plan does not claim that TDD has occurred. Only execution
records can establish a prior failure or later pass.

### Unit tests

A unit test specifies one observable behaviour at a public contract or
focused component boundary. It names the setup, input, expected output,
state change and side effects. Boundary tests include the value exactly
at, immediately below and immediately above each limit where the type
permits.

Mocks are limited to external boundaries whose contract is independently
tested. A test may not assert the implementation's private calls when an
observable public result expresses the requirement.

### BDD scenarios

BDD scenarios use Given/When/Then with concrete actors and data. Every
feature has, as applicable:

- the primary successful user journey;
- each declared supported variant and explicit non-applicability;
- permission denial and tenant isolation;
- moderator approval, rejection and undo for site changes;
- invalid, empty, maximum-size, duplicate and out-of-order input;
- timeout, retry, duplicate delivery and dependency failure;
- interruption and recovery around every durable side effect;
- privacy, redaction and data-deletion behaviour;
- public-page footprint and accessibility where applicable; and
- an end-to-end journey on each applicable platform.

These categories are a completeness check, not substitute text. Each
scenario must contain feature-specific data and expected results.

### Integration, end-to-end and non-functional tests

Integration tests exercise real contracts between owning modules and
real storage or broker test doubles only where the contract permits.
End-to-end tests begin with the actor's real trigger and finish at the
observable product outcome, including undo where applicable.

Performance evidence states the workload, hardware, concurrency, data
size, warm-up, repetitions, p50/p95/p99 measure and memory high-water
mark. Security evidence names the threat and protection. Formal evidence
names the property, tool, bounds, assumptions and trusted computing base.
No universal claim follows from a scoped proof.

### Edge-case catalogue

The owning specification enumerates edge cases derived from its data and
state machine. At minimum it considers:

- empty, singleton, maximum and oversize collections;
- zero, negative, overflow, underflow, NaN and infinity for numeric
  domains that can represent them;
- Unicode normalisation, right-to-left text, combining marks, invalid
  UTF-8 at external boundaries and very long tokens;
- duplicate, missing, stale, future-dated and contradictory records;
- cross-tenant identifiers and permission changes during execution;
- repeated delivery, reordering, concurrent writers and stale artefacts;
- cancellation, crash, clock movement and partial external success;
- no eligible result, all-equal scores, ties and ideal-versus-actual
  ordering where ranking is involved.

An inapplicable category requires a reason tied to the contract. It is
not recorded with an unexplained `N/A`.

## Risk-based assurance

ADR-0020 defines strict-core membership. Apply the independent reviews
in [AGENTS.md section 7.1](../../AGENTS.md#71-the-required-reviews) and
the separate line, branch, MC/DC and killable-mutant coverage required by
[AGENTS.md section 6](../../AGENTS.md#6-code-standards) rule 4 to each
strict-core operation and its reachable control flow. Check the evidence
for each dimension under its owning rule. Use `TASK-0061-COVERAGE-PROOF`
for measured evidence and reviewed denominators. Apply AGENTS.md rule 4's
proof and independent review requirement before excluding an equivalent
mutant from its denominator.

Other components require all specified unit, BDD, integration and
end-to-end checks to pass, plus risk-appropriate independent review,
property tests, fuzzing, failure injection and mutation analysis.
Security, privacy, financial or destructive impact raises assurance even
when the component is outside the strict core. Required checks have a
100% pass rate; this does not imply universal statement or mutation
coverage for every ordinary component.

Reviewers must be independent of the change author and possess the
needed security, performance, data or legal competence. Multiple reviews
written by the same context do not establish independence.

Bind each applicable review obligation to its owning operation, risk
classification and required discipline. At the stage that requires a
completed review, resolve its authorised record under AGENTS.md 7.1.
Verify the current signed fingerprint, required review identity,
verdict and signer. Compare reviewer and author principals under
the existing independence and freshness rules. For 0064 member
evidence require the member’s own 0065 review record. Resolve any
finer feature, variant or operation scope through the admitted
protected subject and evidence references. If the current record
and reader cannot distinguish the scope, hold that finer review
obligation. A whole-change review keeps its own credit. A summary,
filename or unsigned mapping cannot extend it.
Keep a missing, failed, unresolved, stale or wrong-scope review
open for its own obligation even when test evidence and counts pass.
A review plan before derivation describes what is required. It does
not stand in for a completed review at a later gate.

## DRY, KISS and SOLID as reviewable contracts

These principles produce concrete checks rather than decorative claims:

- **DRY:** each rule, computation, schema and default has one normative
  owner. References preserve identity. A duplicate must be removed or
  proved to be independent data.
- **KISS:** the design uses the smallest contract and state machine that
  meets the requirement. Every fallback, abstraction and configuration
  option has a triggering requirement and test.
- **Single responsibility:** each module and specification has one
  behavioural reason to change; cross-cutting rules are referenced.
- **Open/closed:** extension points identify the permitted variant and
  compatibility rule; speculative plugin interfaces are rejected.
- **Liskov substitution:** a variant documents and tests every inherited
  precondition, postcondition, error and side-effect guarantee.
- **Interface segregation:** callers see only operations they use;
  permissions are scoped to those operations.
- **Dependency inversion:** product policy depends on typed interfaces;
  infrastructure adapters implement them. Dependency direction and
  forbidden cycles are checked.

The architecture review records the actual contract inspected and any
finding. Naming these principles in a task is not evidence of compliance.

## Research provenance and clean-room evidence

Every consequential algorithm, protocol, dependency, dataset, threshold
and security control has a provenance record containing:

1. stable requirement and design identifiers;
2. source title, authors or publisher, publication and persistent URL or
   identifier;
3. exact version, revision and artefact digest where applicable;
4. licence and notices;
5. the fact or method taken from the source;
6. alternatives considered and the product reason for selection;
7. independent validation and known limitations; and
8. the provenance class below.

| Class | Required evidence |
|---|---|
| Original design | Requirements, design rationale and independent review; no unsupported claim of novelty or patent safety |
| Public standard or published algorithm | Normative citation, formula/protocol interpretation, conformance vectors and divergence list |
| Licensed dependency | Publisher, exact artefact, digest, licence, notices, security and compatibility evidence |
| Permitted interoperability observation | Public observation source, lawful-access basis, clean functional description and no copied expression |
| Strict clean-room reimplementation | Separate researcher and implementer identities, researcher access log, controlled functional specification, implementer attestation of no access to restricted material, independent comparison and retained transfer evidence |

Strict clean room is used only when the provenance risk requires it.
Ordinary standards implementation and licensed reuse must not be
mislabelled as clean room. Publicly available code is not automatically
lawful to copy.

A patent or freedom-to-operate obligation has its own `LEG` identity,
jurisdiction, claims examined, reviewer, date and conclusion. Engineering
research is not legal clearance. An unresolved legal obligation blocks
the affected release and cannot be closed by an agent's assertion.

## Conflict, duplication and overlap control

“Zero conflicts” means zero known unresolved contradictions in the
complete audited inventory at a recorded repository revision. It does not
prove that no unknown semantic conflict exists. The audit record states
the revision, inventory digest, checks performed and residual risk.

Two requirements conflict when their permitted outputs, side effects,
permissions, thresholds or resource limits cannot all hold for one legal
input. Two behaviours duplicate when they implement the same contract
under different identities. They overlap when callers cannot determine
the authoritative producer or when both can write the same state without
one coordination contract.

The audit must:

1. enumerate every feature and alias, not a sample;
2. resolve each behaviour to one owner and each shared contract to one
   definition;
3. compare errors, defaults, thresholds and state transitions across
   owners;
4. check bidirectional requirement/design/task/test links;
5. search for duplicate and look-alike identifiers;
6. review semantic neighbours, not only lexical duplicates; and
7. record every unresolved item as a blocker with an owner and precise
   question.

An implementer may not choose between contradictory rules silently. The
question goes to the owner through the decision process, and the answer
enters the ADR ledger before propagation.

## Derived task contract

A Forgejo implementation task is derived only after all its upstream
requirements and designs are complete. The owning design can name
the stable `TASK` identity before a Forgejo task number exists.
The derived task contains:

- exact feature or foundation source identities, as defined here;
- requirement, design and variant identifiers;
- owning specification and exact repository revision;
- permitted files and module boundary;
- named unit, BDD, integration, end-to-end and non-functional tests;
- exact fixtures and expected failure before implementation;
- security, privacy, performance and accessibility obligations;
- research and legal gates;
- the required phase entry and exact signed predecessor proof;
- migration, recovery and rollback procedures where applicable;
- required independent reviewers; and
- definition of success expressed as observable evidence.

After derivation, bind the exact Forgejo repository and task
number to the stable `TASK` identity. Verify the reciprocal links
to its owning design and required upstream records before moving
it to `status/ready` or claiming it. Forgejo records the task state.

Under DR-131 and [0139](0139-full-foundation-refactor.md), verify
the predecessor before deriving a build task, moving it to ready,
assigning or claiming it, renewing or resuming it, or accepting its
candidate or exit evidence:

- 3A requires the accepted Phase 2 exit.
- 3B requires the accepted 3A exit and its frozen refactor identity.
- 3C requires the accepted 3B exit for the exact Boot artefacts,
  parity ledger and consuming platform and profile.
- Phase 4 and later work retain signed Phase 3 closure and their
  own exact intervening prerequisites.

The protected 0010 phase check verifies the gate account's
version 1 `phase-gate-run` record under the 0065 `gate-run`
signing purpose. Match its verified stage, accepted candidate,
policy and predecessor evidence to the cited
0139 handoff. Use the record digest in the task admission evidence.
Missing, stale, wrong-stage, wrong-signer or mismatched proof
holds the task. Match a changed dependency to fresh applicable
proof before reuse. The version 1 record covers the existing
Phase 2, 3A, 3B and 3C exits. For Phases 4 and 5, require the exact
protected closure profile under QR-241 before accepting their
exit evidence or using it as a predecessor. A local task
dependency, fixture, review,
ordinary passing gate result or release signature supplies no
phase acceptance on its own. Hold implementation while the 0010
phase verifier lacks admission proof, even for a complete design.
Reading history, stopping work and authorised reconciliation
continue under their own rules.

Require the separately admitted Phase 6 closure profile under
[QR-243](../registers/questions.md) before accepting that phase's
exit or using it for Phase 7 entry. Match the accepted Phase 5
lineage, the exact approved feature and expanded-scope inventories,
applicable supported variants, and bound evidence for both test sites,
recorded demonstrations and independent reviews. Use each declared
exclusion under its owning authority. Apply the check only at closure
and the downstream hand-off. Phase 6 work keeps its existing entry and
per-task prerequisites.

### Foundation and framework task scope

For 3A and 3B, the governing inventory or parity row supplies the
source identity in place of a numbered product feature. Keep the
same requirement, design, variant, test and review obligations.
Classify each task by its owning operation and reachable effects.
Record the exact work class, revision and source records, owning
contracts, affected callers and consumers, allowed boundaries,
and the scoped dependencies and verification cases. Cite the
current phase prerequisite separately from task-level inputs.

For 3A, cite the reviewed 0139 baseline and each inventory item
with its disposition, owner and planned evidence. At acceptance bind
the result to that exact disposition under 0139. A design receipt
cannot supply conformance proof for a retained running component.
For 3B, cite an exact applicable
0081 row with an admitted upstream source and a complete design
in its owning group. The row can enter implementation at
`specified`. Its later `matched` and `verified` states are
outputs of implementation and qualification. A train label,
group title or adjacent row supplies no substitute design.

Each dependency names its owning record, exact identity and
observable result required before the consumer starts. Reference
shared proof only for the profile and scope it actually covers.
Hold ready, claim and acceptance when a required dependency
or work-class boundary is missing or unproved. A fixture may
exercise the framework under 3B. A real product consumer uses
the 3C or later route even when bundled with framework work.
Split the scopes or hold the combined task.

A task may not introduce a new requirement, choose a model, set a
threshold, resolve a legal question or design the feature it implements.
Missing information returns the task to documentation readiness; it does
not become an implementation-time research prompt.

## Readiness and prohibited placeholders

A feature is implementation-ready only when its complete readiness
record, traceability links, provenance, operational procedures and test
design are present and every applicable legal and safety gate has
closed. The readiness checker rejects:

- `TBD`, `TODO`, `to research` and `to decide` as design
  placeholders, and any required design field left empty,
  unspecified or marked `unknown` or unexplained `N/A`;
- placeholders such as `chosen later`, `lab picks`, `sensible default`,
  `appropriate fallback` or equivalent wording;
- an unmapped feature, alias without successor or requirement without a
  verification obligation;
- a task without a populated upstream design;
- an unbounded parameter, missing error outcome or unspecified side
  effect;
- a claimed measurement, review, patent clearance or clean-room process
  without evidence; and
- contradictory current instructions, including text that tells a later
  range task to supply the missing design.

A defined runtime result such as `unknown` needs its exact trigger,
observable result, effects and applicable refusal. Verify that
recovery and tests resolve under the owning contract. [0150][sensor]
gives an unproved cell a typed state, a safe reason and no value.
Check the complete behaviour and treat an unspecified part of its
design as a placeholder.

A known missing artefact is recorded in Forgejo as a documentation
blocker with its exact required evidence. It is not hidden behind a
placeholder and cannot be marked implementation-ready.

[sensor]: 0150-experiment-source-and-sensor-gates.md#evidence-records

## Specification-owned verification

The following fixtures are documentation-validation fixtures, not
product capabilities and not evidence that executable product tests have
run. They use `FX-` identities under "Stable identities". The retired
product-style aliases preserve earlier references only; they do not
restore product-test identities or qualify a release.

| Fixture | Retired alias | Concrete input | Required result |
|---|---|---|---|
| `FX-0076-ID-CANONICAL` | `UT-0076-ID-CANONICAL` | Accept `REQ-0076-ID-CANONICAL`; submit lower case, a look-alike Unicode hyphen and a duplicate | Accept only the canonical identifier; reject each invalid or duplicate form with its location |
| `FX-0076-ALIAS-RETENTION` | `UT-0076-ALIAS-RETENTION` | Retire `F-1000` to `F-0933` and then query both | Preserve the alias and resolve only to the successor; do not count two features |
| `FX-0076-TRACE-CYCLE` | `UT-0076-TRACE-CYCLE` | Link requirement R1 to design D1, task T1 and tests U1/B1, then remove B1's reverse link | Fail bidirectional traceability and name R1 and B1 |
| `FX-0076-ERROR-COMPLETE` | `UT-0076-ERROR-COMPLETE` | An error with code and trigger but no retry or side-effect contract | Reject readiness and identify the two missing fields |
| `FX-0076-VARIANT-DISTINCT` | `UT-0076-VARIANT-DISTINCT` | Two variants differing only by a numeric tuning value, then two with different validation and outputs | Reject artificial variant counting in the first case; require separate evidence in the second |
| `FX-0076-PLACEHOLDER-SCAN` | `UT-0076-PLACEHOLDER-SCAN` | Use `TBD`, `to research`, `chosen later` and `appropriate fallback` in normative fields, the same words in a marked rejected example, an unspecified maximum labelled `unknown`, and a defined missing-cell result of `unknown` with its trigger, safe reason, value suppression and linked refusal and recovery cases. Remove the trigger in a separate case. | Reject unspecified design and the incomplete result contract. The complete result contract and marked negative example pass the placeholder check only. |
| `FX-0076-CONFLICT-LIMIT` | `UT-0076-CONFLICT-LIMIT` | One contract permits six results and another requires at most five for the same operation and input | Report one conflict naming both identifiers; no precedence-based silent resolution |
| `FX-0076-CLEANROOM-SEPARATION` | `UT-0076-CLEANROOM-SEPARATION` | Strict clean-room record with the same principal as researcher and implementer | Reject the evidence; licensed public-standard records remain distinguishable |
| `FX-0076-MUTANT-DENOMINATOR` | `UT-0076-MUTANT-DENOMINATOR` | One equivalent mutant with proof and independent review, one without proof, and one killed mutant | Exclude only the proved equivalent mutant; block on the unproved survivor |
| `FX-0076-INVENTORY-DIGEST` | `IT-0076-INVENTORY-DIGEST` | Add a feature without updating the readiness index, then restore the index without changing feature bytes | First run fails inventory coverage and digest reconciliation; second passes without inventing design content |
| `FX-0076-RELEASE-TRANSITION-COVERAGE` | None | A synthetic migration permits rollback with binary A on schema B. Its test covers A on schema A only. In separate cases, add the A-on-B expected result and its matching test, present a refused pairing with no preserved-state rule, and report successful rollback without its start and finish times. | Reject the missing A-on-B case. Accept the corrected pairing and matching case as design evidence only. Name the missing preserved-state rule for the refusal, and reject the unmeasured recovery claim. |

| Fixture | Retired alias | Given | When | Then |
|---|---|---|---|---|
| `FX-0076-UNMAPPED-FEATURE` | `BDD-0076-UNMAPPED-FEATURE` | A non-alias feature has a title but no readiness record | Its implementation task requests admission | Admission is refused and the missing feature owner and record are named |
| `FX-0076-COMPLETE-FEATURE` | `BDD-0076-COMPLETE-FEATURE` | Every required section, concrete test, provenance record and pre-derivation prerequisite validates at a fixed revision. The design names a stable `TASK` before Forgejo issues a number. | Check derivation, then assess a derived fixture task for ready and claim. In separate cases provide an absent number, a wrong number, the exact number with a missing reciprocal link, and the exact number with its link. | The complete design can be used to derive the task. Refuse ready and claim for an absent or wrong binding. The exact binding passes the structural check only. It grants no coding or release authority. |
| `FX-0076-CONTRADICTORY-THRESHOLD` | `BDD-0076-CONTRADICTORY-THRESHOLD` | Two current specifications set different values for one protected threshold | An agent asks which to implement | The checker blocks both and records the conflict; the agent must use the owner decision route |
| `FX-0076-LATE-DESIGN` | `BDD-0076-LATE-DESIGN` | A range task contains only a feature list and says design will happen during implementation | The task is checked | It is rejected as a derived implementation task and no feature-specific tests are fabricated |
| `FX-0076-STRICT-BOUNDARY` | `BDD-0076-STRICT-BOUNDARY` | A ranking module also performs atomic champion transitions. A separate synthetic assurance record for that transition reports line coverage at its required threshold and supplies no branch, MC/DC or killable-mutant evidence. | Classify both operations and evaluate the claim of complete strict-core assurance | Ranking calculations use risk-based assurance. The transition operation and reachable control flow use strict-core assurance. Reject the incomplete assurance claim and name the missing branch, MC/DC and killable-mutant evidence. The synthetic fixture gives no coverage proof |
| `FX-0076-REVIEW-SCOPE` | None | Use two approved features on one candidate, then two approved variants of one feature. First give them the same valid whole-change review and no finer protected subject proof. In another synthetic admitted-reader branch use independently bound subjects A and B. Scopes A and B each need the same named discipline. Keep scope, tests and counts fixed. All other required reviews are eligible | Supply a valid current independent pass for A only and attach it to B as well. Add an unsigned mapping claiming the same reviewed bytes cover B. Then supply an independent pass with B's exact scope but another discipline. Use the exact required review for each in the control | The whole-change review satisfies only its original duty. In the admitted-reader branch accept only the proved A obligation in each refusal case and identify B's unproved scope or discipline. In the control accept both only within their proved scope. The synthetic result supplies no executed review, signer admission or release proof |
| `FX-0076-LEGAL-OPEN` | `BDD-0076-LEGAL-OPEN` | A method has papers and clean-room evidence but an unresolved patent review. In separate cases supply a qualified review scoped only to another `LEG` obligation or only to another jurisdiction. Use a matching synthetic signed document check in each case. | Evaluate evidence for the original release obligation | Refuse to close the original obligation, name its missing in-scope review and keep the affected release blocked. The signed document check proves structure only. Emit no legal conclusion. |
| `FX-0076-REBASE-AUDIT` | `BDD-0076-REBASE-AUDIT` | The inventory changes after a readiness report | The report is reused for a new revision | Stale evidence is rejected until the complete affected audit is rerun and the new digest is recorded |

`FX-0076-PHASE-WORK-SCOPE` checks three records under matching
synthetic phase prerequisites. The first claims implemented 3A
coverage for an archive-only design. The second names an applicable
0081 row at `specified` with exact owner, design, dependencies
and failing-first cases. The third bundles a real product consumer
into that 3B task. Refuse the first and third claims. The second
passes the document scope check without claiming a matched or
verified row. Remove one dependency or substitute a sibling row
and require a refusal for that exact missing or mismatched field.
For the 3A disposition binding, reuse IT-0139-INVENTORY. Keep the frozen
item, baseline and `retain` disposition fixed and cite a design receipt
for that same item in place of its conformance result. Then omit the
result. Both fail the acceptance evidence check. The matching receipt
passes the binding check only. The fixture relies on no live phase proof.

The documentation gate runs link, identifier, schema, ownership,
placeholder, provenance and bidirectional traceability checks on the
whole inventory. A passing structural check establishes document
consistency only; it does not establish product quality, implementation
coverage, legal clearance or release approval.

## Data

This spec owns no database table and no product setting. It governs
records held elsewhere:

- feature identities and aliases, held by
  the [feature register](feature-register.md);
  other identities and aliases are held by the owning specification;
- readiness records, requirements, designs, variants, interfaces, errors
  and tests, held by the owning specification;
- derived tasks, held by Forgejo with their task numbers, together with the
  execution state and evidence (Forgejo owns these, not requirements);
- provenance and legal-review records (`SRC` and `LEG` identities), held by
  the owning specification;
- the conflict audit record: repository revision, inventory digest, checks
  performed and residual risk; and
- the `FX-` fixtures, held in this specification and in the repository.

## Settings

This spec has no setting that a user or operator can change. Strict-core
coverage follows the applicable rules in "Risk-based assurance". The values
below are fixed by the text or by ADR-0020. The text does not say when a
change takes effect, so each "takes effect" entry is (proposed).

| Setting | Type | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| Independent reviews of a strict-core operation | whole number | 8 | fixed by ADR-0020 | next readiness run (proposed) |
| Pass rate of required checks | percent | 100 | fixed | next readiness run (proposed) |
| Known unresolved contradictions in the audited inventory | whole number | 0 | fixed | next audit (proposed) |

## Failure codes

This spec defines no failure code of its own. Its readiness checker rejects
the cases listed under "Readiness and prohibited placeholders" and in the
fixtures, and reports the location or identifiers named there. The error
form `ERR-NNNN-SSS` belongs to each owning specification. Codes are
reserved under a later decision (see QR-220).

## Security and privacy

- The fixtures use `FX-` identities, hold no product data and cannot
  qualify a release.
- Provenance records name sources and licences. They do not copy
  expression, and publicly available code is not treated as lawful to copy.
- A strict clean-room record needs separate researcher and implementer
  identities and a researcher access log. The same principal in both roles
  is rejected (`FX-0076-CLEANROOM-SEPARATION`).
- An unresolved legal obligation blocks the affected release. An agent's
  assertion cannot close it.
- Error records state what the caller may learn and whether details are
  redacted. Audit events and logs exclude sensitive content.
- Reviewers must be independent of the change author. Several reviews
  written by one context do not count as independent.
- Evidence from an old revision is rejected for a new one
  (`FX-0076-REBASE-AUDIT`).

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| REQ-0076-ID-CANONICAL | An identity MUST be canonical, unique within its kind and never recycled, and a retired identity MUST stay as an alias with its successor and reason. | FX-0076-ID-CANONICAL, FX-0076-ALIAS-RETENTION |
| REQ-0076-READINESS-RECORD | Every non-alias feature MUST have a readiness record in its owning specification, and a feature without one MUST NOT be admitted for a task. | FX-0076-UNMAPPED-FEATURE, FX-0076-COMPLETE-FEATURE, FX-0076-RELEASE-TRANSITION-COVERAGE |
| REQ-0076-NO-CODING-AUTHORITY | A complete design MUST NOT by itself authorise coding, release, or legal clearance. | FX-0076-COMPLETE-FEATURE, FX-0076-LEGAL-OPEN |
| REQ-0076-PHASE-WORK-SCOPE | A 3A or 3B task MUST map its exact inventory or parity source, owning design, callers, dependencies and tests. A framework task MUST NOT confer a product prerequisite or claim an unverified exit. | FX-0076-PHASE-WORK-SCOPE |
| REQ-0076-TRACE | Requirement, design, task and test links MUST hold in both directions and match the inventory digest. Verify the issued Forgejo task binding before ready or claim. | FX-0076-TRACE-CYCLE, FX-0076-INVENTORY-DIGEST, FX-0076-COMPLETE-FEATURE, FX-0076-REVIEW-SCOPE |
| REQ-0076-ERROR-COMPLETE | An error record MUST state its trigger, detection, result, side effects, security, recovery and tests, and one that lacks a field MUST be rejected. | FX-0076-ERROR-COMPLETE |
| REQ-0076-VARIANT-DISTINCT | A variant MUST differ in observable behaviour or code path, and a numeric tuning difference MUST NOT create one. | FX-0076-VARIANT-DISTINCT |
| REQ-0076-PLACEHOLDER | The checker MUST reject normative placeholders and a task that leaves design to implementation, and MUST distinguish a specified runtime result or marked negative example from a missing design value. | FX-0076-PLACEHOLDER-SCAN, FX-0076-LATE-DESIGN |
| REQ-0076-CONFLICT | Two requirements that cannot both hold for one legal input MUST be reported as a conflict naming both, with no silent precedence. | FX-0076-CONFLICT-LIMIT, FX-0076-CONTRADICTORY-THRESHOLD |
| REQ-0076-CLEANROOM | Strict clean-room evidence MUST show separate researcher and implementer, and ordinary standards or licensed reuse MUST NOT be labelled clean room. | FX-0076-CLEANROOM-SEPARATION |
| REQ-0076-ASSURANCE | Strict-core operations MUST get strict-core assurance and other work risk-based assurance, and only a proved, independently reviewed equivalent mutant MAY leave the denominator. | FX-0076-MUTANT-DENOMINATOR, FX-0076-STRICT-BOUNDARY, FX-0076-REVIEW-SCOPE |
| REQ-0076-REBASE | A readiness report MUST NOT be reused for a changed inventory until the affected audit is rerun and its digest recorded. | FX-0076-REBASE-AUDIT |

## How it is tested

The fixtures under "Specification-owned verification" are the tests of this
spec. They test documentation checks only. A passing run shows that the
documents are consistent. It does not show product quality, coverage, legal
clearance or release approval. Each requirement above names its fixtures.
The retired `UT-0076`, `IT-0076` and `BDD-0076` aliases are listed beside the
fixtures and do not qualify a release.

## Changes from the source

The text records two changes. The earlier product-style test identities
(`UT-0076-*`, `IT-0076-*`, `BDD-0076-*`) are now `FX-0076-*` documentation
fixtures, with the old names kept as retired aliases. The checker now
rejects placeholder wording and a range task that leaves design for
implementation. This spec does not record the original source text, so no
other change is listed here.
