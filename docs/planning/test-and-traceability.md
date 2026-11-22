# Test and traceability catalogue

[Spec 0076](../specs/0076-specification-completeness.md) owns what each
test record must prove. This catalogue links the owning design and task
to named checks. Test plans describe actions and expected results.
Execution evidence uses Forgejo and the signed evidence store.

## Trace format

| Link | What identifies it |
|---|---|
| Behaviour | Feature or roadmap item, owning spec and exact requirement |
| Applicability | Actor, site and supported variant or explicit refusal |
| Implementation slice | Stable task key and the reconciled Forgejo issue |
| Verification | Existing test identity or a proposed fixture with input and observable expected result |
| Evidence | Named producer, scope, tool or reviewer and expected receipt |
| Closure | Check under spec 0076 that each requirement has a test and each test has an owning requirement |

Rows below describe concrete fixtures. A proposed fixture gets its
stable test ID in the owning spec before the implementation task can
be admitted. Reuse existing IDs. A shared test needs a trace entry
for each task and requirement it covers. One passing example does
not qualify a different platform, variant or error case.

Start the expected requirement set with the
[owning specification audit][owning-spec-audit]. The feature-wave
keys below do not enumerate the requirements of shared contracts,
operational procedures or other owning specifications.

[owning-spec-audit]:
  documentation-completeness-audit.md#audit-each-owning-specification

## Trace admission after the five feature waves

The [feature register](../specs/feature-register.md) owns source IDs
and their owners. [Spec 0076](../specs/0076-specification-completeness.md)
owns readiness and trace identities. The
[implementation catalogue](implementation-catalogue.md) owns task
derivation within the roadmap. Forgejo records task state and run
evidence. Apply this procedure to a fixed candidate revision.

### Reconcile the source inventory

| Wave | Source range | Planning reference |
|---|---|---|
| 1 | 1–112 | [Wave 1](feature-wave-1.md) |
| 2 | 113–712 | [Wave 2](feature-wave-2.md) |
| 3 | 713–1312 | [Wave 3](feature-wave-3.md) |
| 4 | 1313–1357 | [Wave 4](feature-wave-4.md) |
| 5 | N-1358–N-1880 | [Wave 5](feature-wave-5.md) |

1. Build independent source sets from features.md and the protected
   owner paste item keys. Expect 112, 600, 600, 45 and 523 items,
   respectively. Compare both ways with the register and actual
   acceptance rows. Counts locate gaps and do not prove readiness.
   Keep the original numeric or `N-` spelling and title. Resolve
   the spec 0076 feature identity through the register or an explicit
   recorded binding. Apply the [Wave 5 canonical bindings][w5-ids]
   to registered pasted keys. Keep the original source key and any
   documented alias distinct.
   Never join rows by title similarity, wave order or shared section.
2. For each row record the candidate revision and register digest,
   exact source locator and row key, canonical feature ID, one owner,
   classification and controlling scope decision if applicable. For
   Wave 5 keep its exact source group and paste item reference. Keep
   private source references in the existing protected record.
3. Join the source index and acceptance designs by exact registered
   key. Expand a range or grouped reference only where the individual
   feature and expected result are identifiable. Reconcile both ways
   and name every missing, duplicate or unregistered row. Record the
   deterministic inventory digest with the audit. Use UTF-8 with LF.
   Use the canonical JSON profile of spec 0003. Encode each row as
   array fields in this order: candidate tree,
   register digest, original key, source digest and stable locator,
   canonical ID, owner spec, classification, controlling decision
   or null, and acceptance digest or null. Follow each row with LF.
   Order by numeric ID, then original namespace and exact key.
   Edge arrays hold kind, exact source and target identity, scope
   digest and evidence digest or null in that order. Order by kind,
   source, target and scope digest.
   Compute SHA-256 on the ordered bytes. Keep locators and copies of
   protected source hashes only in the scoped evidence.
4. Run the 0076 semantic audit independently of key reconciliation.
   Compare neighbouring boundaries, defaults, errors, effects and
   transitions. Report contradictions even when both IDs resolve.
   Recheck affected results when a binding or behaviour changes.

[w5-ids]:
  ../specs/feature-register.md#canonical-bindings-for-pasted-features

### Bind each selected behaviour in both directions

| Edge | Minimum binding |
|---|---|
| Source row → owner and scope | Exact canonical feature and one spec, with applicable decision and entry point. Retain exclusions and applicability. |
| Feature → requirement → design | Owning `REQ` and `DES` IDs and exact sections. Include observable limits and effects and link typed interface IDs and inherited contracts to their original owners. Complete the spec 0076 readiness record at the owner. |
| Design → source and legal review | Bind the applicable SRC and LEG identities, exact scope, date, evidence or qualified review and expiry. Missing research or legal proof holds unresolved admission. |
| Design → variant and error | Owning `VAR` and applicable `ERR` IDs with the selection predicate, limits, effect and refusal. An excluded platform has its exact reason and presentation. Changes in a setting do not create a distinct variant. |
| Requirement and variant → test design | Exact spec 0076 test IDs, individually linked to requirements, applicable variants and errors. Bind the named fixture and input, expected output and allowed or refused effects, layer, expected evidence producer and required profile. Include each applicable run, refusal, failure and recovery obligation. |
| Test design → executable source | Exact revision with path, test symbol or scenario parameter, fixture identity and how the case enters its required suite. Record each part covered by a shared fixture. A prose case and a test name alone do not establish this edge. |
| Test source → execution receipt | Expected receipt type and producer, then the actual signed receipt reference and verified scope when evidence exists. Bind the exact candidate tree, test and fixture inputs, admitted tool and profile, required tier, and recorded result. Where test-first proof is required, bind the exact unmodified baseline failure and reason separately from candidate success. |
| Owner design → implementation task | Verify the complete upstream design and applicable derivation prerequisites under spec 0076 and the roadmap. The design can name the stable `TASK` before Forgejo issues its number. After derivation, bind the exact Forgejo repository and number to that identity and map each upstream requirement, design, variant, case and fixture, baseline proof where required and exact candidate binding. Verify reciprocal task links before `status/ready` or claim. Link signed independent review and required gate evidence when applicable. A source row does not itself create a task. |
| Review obligation → exact reviewed scope | Bind the existing required discipline and risk decision, owner design, affected feature or variant and operation, and the fingerprint of the exact content. At the applicable gate check the current trusted review, required discipline, author and reviewer principals and outcome. Finer scope needs an admitted protected subject binding. Without it, preserve any proved whole-change review and hold the finer obligation. A plan or a sibling pass cannot close a review obligation. |

Reverse-check each design for its requirements, each obligation for a
live owning requirement, and each task and receipt for the exact tests
and variants it claims. Reconcile all expected cases with actual
receipt entries. An extra or orphan receipt cannot satisfy a missing
one. Match each receipt to the exact test case and its input, profile
and tier, even when several cases use the same executable symbol.
A shared test links back to each case it actually exercises,
including the variant and effect assertion for each caller. A family
check cannot close an uncovered feature-specific obligation.
Use the owner's requirement table as the authority. Existing short
check labels may locate an obligation. Bind them to the stable 0076
identities in the owning record before treating the trace as closed.

Use 0076 FX-0076-REVIEW-SCOPE to check sibling scope and discipline
substitution separately. The verifier sees the same candidate and
case results. Without an admitted finer scope, the current review
proves only its own obligation. In the conditional scoped branch
reject an unsigned mapping to a sibling or another discipline.
Report only the covered scope and leave the other review open.
Keep exact review evidence separate from case passes and counts.
For the Phase 6 closure in QR-243, apply the same scope check to
its required phase reviews. The synthetic trace supplies no
admitted closure reader or record.

Join [0064 member review][member-review-trace]
with 0076 `FX-0076-REVIEW-SCOPE`, `UT-0064-REVIEWER-AUTHOR`
and `BDD-0064-TRUSTED-REPLAY`. A valid runner review result supplies
no member review. Where the protected reader can resolve exact scope,
check the independent reviews for A and B separately. Keep the original
whole-change or A credit and hold B when its required scope is
unproved. Complete test results or counts do not fill that omission.

### Interpret scope before assessing evidence

[member-review-trace]:
  ../specs/0064-v1-qualification.md#feature-evidence-records

| Trace finding | Required treatment |
|---|---|
| Shared dependency | Keep its original owner and point to the exact shared contract. Retain the feature's own behaviour and checks for its use of that contract. |
| Retired alias | Resolve the named canonical successor and retain the source row and reason. No independent task, capability or test credit. Missing successor or cycle holds admission. |
| Part of another behaviour | Link the owning contract and any unique assertion about the feature's use of it. Link the existing task where assigned. The scope map can precede task creation. Count the underlying behaviour once. |
| Deferred or excluded | Record the exact scope decision and current refusal or unavailable presentation where applicable. Derive no version 1 build task and count no delivered behaviour for this row. |
| Scope held | Record the unproved grant, rights, owner decision, unsupported route or conflicting limit and the exact evidence needed. Scope held confers no alternate route or permission. |
| Source or binding unresolved | Name the missing, unmapped, ambiguous or contradictory edge and its owning document. Route the gap through Forgejo under 0076. Never guess the owner, variant or test from proximity. |

Keep scope, design and source inspection separate from the execution
field per required tier. Source can be verified while the case has no
established run, or a failed result. Never overwrite a failure from an
unapproved retry. Keep current gaps and results in Forgejo. The
documentation explains the method and scopes immutable references.

| Evidence state | Exact meaning |
|---|---|
| Design absent | A concrete verification design or a required binding is missing. |
| Designed | The concrete case and expected receipt are documented. Executable source for this exact case is absent or has not been verified. |
| Source verified | The exact test case and fixture are located. The source gives no run verdict. |
| Run not established | No exact current run is recorded or visible. An explicitly skipped case records the reason it did not run. |
| Receipt unavailable | A run or receipt reference exists, but the signed result cannot be retrieved and verified. |
| Receipt mismatched | The supplied receipt is invalid, revoked or belongs to a different candidate, case, profile or tier. It cannot supply the current outcome. |
| Signed failure | An admitted current signed result reports failure. Retain its exact failed assertion and scope. |
| Signed pass | The admitted current signature and evidence match the exact candidate, case, profile and tier and report success. If other required observations or receipts are absent, those cases stay open. |

An AGENTS.md 7.3 pre-signing record applies only to its roadmap phase
and then-admitted proof. It cannot supply a signed release verdict.
A prior failure, historical pass, skipped case, inapplicability
classification or scope hold supplies no current passing result. An
exact signed result may prove the owning guard's specified refusal
within its approved scope. A synthetic `FX-` trace exercise proves
documentation behaviour only. Assess trace closure, build admission
and release qualification separately. For a complete candidate
inventory every
source row must resolve to its owner or documented disposition. Each
in-scope obligation must close both trace directions. Any relevant
scope hold or missing binding prevents admission of that slice.
Only an actual verified result can close an execution obligation.

### Illustrative trace checks

These examples use registered feature rows and existing case designs.
The example outcomes below exercise the trace procedure. They assign
no new product IDs and record no test execution.

| Documentation fixture | Source and planned assertion | Trace check |
|---|---|---|
| `FX-0076-W4-AUTOSAVE-TRACE` | Wave 4, feature 1357 → 0091 local autosave, with 0042 and 0101 shared contracts. Lose the local save reply, race a newer revision and try an erased revision after restore. Expect one current local result or a scoped conflict, with no native write. | With only the wave row and supporting prose, retain the design locator and list any unbound stable requirement, design, variant and test IDs. If the owner binds these and supplies an exact source case, that source proves no signed outcome. A receipt for a prior draft revision fails the example's exact input check. |
| `FX-0076-W5-WORKFLOW-TRACE` | Wave 5, N-1358 → 0099 workflow view, JW-02 and JW-03. Supply recorded movement, hidden activity and a stale snapshot, then switch to the reduced-motion view. Expect the same permitted state with equivalent access and no hidden task. | A test symbol covering only animation leaves visibility, stale state and equivalent access open. A matching synthetic passed trace check does not close any of those product obligations. Require reverse links from each actual case to its exact owner bindings. |
| `FX-0076-W3-ALIAS-TRACE` | Wave 3 feature 1284 resolves to 1283. | Keep the reason and successor. Refuse a second runtime capability, test credit or independent task. |
| `FX-0076-WAVE-SCOPE-TRACE` | Wave 2, feature 280 → 0046, deferred outreach under DR-116. Attempt to reuse an ordinary alert recipient and old approval as an outreach grant. | Record the deferred scope and link the refusal obligation. A signed refusal test can prove the guard; it cannot qualify the deferred feature or admit a send route. |

Record verification gaps with owner, candidate, case and precise
missing evidence. Document structural or reviewed results at their
actual scope. The roadmap, signed evidence and owner decisions retain
their own authority.

## Initial foundation trace examples

These examples cover selected foundation tasks. The implementation
catalogue names the other tasks. The owning links point to the
design sections. The last column names the stable bindings each
owner must supply for admission. For the reverse check, start at
that requirement in its owner and find this same task, variant,
test and receipt identity. A missing binding prevents admission.

| Task and source | Layer and fixture | Given and when | Expected result | Evidence producer | Owner bindings to establish |
|---|---|---|---|---|---|
| TASK-0001-DOOR-GATE, [0001](../specs/0001-door.md), roadmap 1.4 | Integration, disposable Forgejo repository | Given the admitted profile and its evidence for tree A, attempt to submit B. Then submit A with its exact receipt | B is refused and its ref never lands. A advances main once | Dell door gate and ref read-back | Gate and exact-tree acceptance requirement, bootstrap and final variants, integration IDs |
| TASK-0002-SYNC, [0002](../specs/0002-forge-sync.md), roadmap 1.9 | BDD, fake mirror with a lost response | Given main advanced and the mirror applied the update, lose its reply and retry. Offer private issue text in a second input | The ref is verified once. No private payload leaves Dell | Sync receipt and sink history | Mirror and privacy requirements, lost-result and protected-input variants, BDD IDs |
| TASK-0001-HEAVY-QUEUE, [0059](../specs/0059-pankti.md#limits), [0010](../specs/0010-coding-harness.md#heavy-job-control-on-dell) and roadmap 1.7 | Integration design, synthetic sealed Dell queue evidence | Fill both places and put a third job in line. In a separate case lose the queue holder while its exact child remains live. Offer the freed lock to a waiting job for that place and to a timed benchmark. Separately miss the current scope observation or substitute one from another scope. Then prove the child and its owned work have stopped using the exact current scope | The third waits as required. A freed lock, missing observation or wrong scope cannot supply protected admission or a timing result. A current empty-scope proof permits only the remaining eligible admission, with both places clear for the timed case. No stale result can finish another job | Bind the exact queue, holder, scope, observed child disposition and applicable normal or timed profile to admission and its receipt. Use Phase 1 evidence under the existing bootstrap rules. The Phase 2 sealed runner consumes only the matching current route proof | No live process control, queue change, or 0019 product lease is part of the fixture. |
| TASK-0003-SEALED-RUN, [0003](../specs/0003-toolchain-and-check-runner.md), refusal probe | Failure injection, exported scratch tree | Ask a test worker to open a protected file. Alter the candidate after the test returns | OS fence refuses the read. The changed tree cannot reuse the result | Trusted gate refusal record | Isolation and exact-candidate requirements, Dell refusal and wrong-tree variants, fault IDs |
| TASK-0012-SOURCE-RECEIPT, [0012](../specs/0012-ganit-evolve.md), roadmap 2.6 | Integration, accepted card and one altered source file | Offer the clean receipt, then alter only the bound source | Exact input verifies. Stale receipt gives no pass | Receipt generator and gate verifier | Receipt identity requirement, exact and altered input variants, integration IDs |
| TASK-0007-MEASURE-SPEED, [0007](../specs/0007-performance-and-engineering-gates.md), roadmap 2.7 | Regression, one sealed hot-path fixture | Compare an admitted baseline and candidate with planted loop allocation or throughput loss | The correct check refuses the planted difference and preserves both profiles | Dell speed runner and independent review | Tier and speed requirements, loop-allocation and time variants, regression IDs |
| TASK-0065-REVIEW-WAIVER-SIGN, [0065](../specs/0065-tijori.md), roadmap 2.8 | Unit and integration, purpose-specific signed fixtures | Offer a review signature as an owner waiver, then an expired waiver | Both are refused without weakening any check | Trusted signature verifier | Purpose and expiry requirements, review and waiver variants, unit and integration IDs |
| TASK-0003-WINDOWS-GATE, [0003](../specs/0003-toolchain-and-check-runner.md#the-locked-down-box), Phase 2 items 3 and 11 | Integration, current protected Windows profile and hosted result | Hold the candidate and policy fixed. Separately offer a result for a different tree, workflow or profile, withhold isolation proof, and omit one required check. Offer an otherwise valid test-signed result for the replacement Bazel graph under its own synthetic profile. Use the exact protected-route result as the control | Dell accepts only the matching authenticated current-route evidence for its required cell. Missing or substituted evidence gives no Phase 2 exit proof. The replacement result proves nothing for the current route. The control gives no EX-03/QR-099 cutover, accepted 3B evidence or live gate authority | Dell verifier and exact hosted tree, workflow, check, profile, policy and refusal-fence bindings | Current-route requirement and negative hosted variants. Use the separate 0003 cutover proof for the replacement profile and complete graph. |
| TASK-0003-MIRROR-CI, [0003](../specs/0003-toolchain-and-check-runner.md), roadmap 2.11 | End to end, one admitted sync event and one pull request | Mirror a candidate and compare the expected CI and Dell check scopes. Try running the same event from a pull request | The sync route records its required checks and Dell retains timing and reference results. The pull request starts no check | Mirror run identity and Dell scope comparison | Mirror scope requirement, sync and pull request variants, end-to-end IDs |
| TASK-0003-PHASE-PROOF, [0003](../specs/0003-toolchain-and-check-runner.md), [0010](../specs/0010-coding-harness.md), roadmap Phase 2 exit, and [0134](../specs/0134-ci-latency-load-gate.md) REQ-0134-TRUST | End-to-end clean and planted candidates, including the existing LOAD-0134-DOOR receipt fixture | Run the expected tier inventory, then remove a required receipt. In a separate case take two required Phase 2 release-tier planted faults for the same admitted language and profile. Exchange their correctly test-signed refusal receipts between the two expected check and fixture roles. Keep the protected `required-check-inventory` and `check_catalogue_digest` unchanged and preserve canonical role order and counts. In the 0134 bootstrap use the normal door check and matching synthetic signed envelope, then substitute the candidate, tier or profile | Only complete evidence for the required clean and planted checks supplies the phase result. Refuse each swapped receipt against its expected check and fixture. Hold Phase 2 acceptance even though both signatures verify. The bootstrap proves only exact receipt verification. Later product workloads still require their admitted entry points. The independent 0007 gates still apply | Signed gate evidence and phase review, with a separately bound bootstrap fixture and outcome | Phase exit requirement, complete, missing and swapped receipt variants, and the existing E2E-0134-TRUST receipt checks |

For TASK-0003-PHASE-PROOF, keep the exact 0010 protected
`required-check-inventory` and withhold one required language-and-tier
result. Offer an exit-zero `aegis catalogue` report with the gap.
Separately offer a valid test-signed 0003 structure-check result
for that candidate and profile. It verifies only for its own check.
In a separate exact synthetic candidate and profile, change `partly`
to `items`. Bind that candidate and its matching catalogue digest.
Keep the required role and fixture identities fixed with no observed
proof for the affected check.
Each case holds Phase 2 with the missing role named and the accepted
pointer unchanged. Supply the matching observed result signed by
the admitted synthetic runner as the control. The control passes
only its own cell, with no live phase or signing effect.

The
[0059 nightly check](../specs/0059-pankti.md#the-nightly-noisy-neighbour-test)
owns `IT-0059-NOISY-PRESSURE`. Bind each probe's monotonic interval to
continuous pressure in its exact current boot, queue place, run and
owned scope. Twenty in-time rounds qualify only with full coverage.
Try a legacy-length pressure interval and near-deadline probes.
Probe completion after pressure ends, an early marker, partial
pressure or a different scope fails even with fast replies and exit
zero. The nightly availability result supplies neither scope
admission nor benchmark timing proof.

Per-feature readiness also needs its exact requirements, variants,
boundary neighbours, failures and operational evidence under spec
0076. A family-level example is a shared fixture, with no effect on
the feature's individual proof.

Keep the [0059 disk and legacy sweep proof][legacy-sweep-trace]
under QR-246 separate from the heavy-job queue result. Compare the
current rule-book scope with the exact planned cleanup candidate
and synthetic refusal. A recent or in-use cache, protected material,
work copy or unknown use supplies no cleanup credit. Bind identity,
write and use evidence before the eligibility decision. Report the
unproved cleanup scope as its own finding. Exercise the expected and
effective unit files and ordered overrides with unchanged executable
bytes, then test mode drift and unproved owned scope after recovery.
The read-only identity result proves no cleanup effect or permission.
The fixture starts no timer or service, removes no files and changes
no installed state.

[legacy-sweep-trace]:
  ../specs/0059-pankti.md#legacy-nightly-sweep-and-current-permission

### Phase 0 and 1 archive, bootstrap and runner trace designs

Follow the order in the implementation catalogue
[foundation slices](implementation-catalogue.md#phase-0-and-1-foundation-slices)
and [roadmap](../roadmap.md). These five missing trace rows use
synthetic `FX-` fixtures. Each owner binds the requirement, design,
variants and executable case under
[0076](../specs/0076-specification-completeness.md).

#### TASK-0009-PRIVATE-ARCHIVE

**Owner and anchors.** Roadmap 0.1 and
[ADR-0011](../adr/0011-fresh-start.md#decision) own private read-only
retention on Dell outside the repository. The catalogue names
[0009](../specs/0009-source-class-inputs.md#reading) for source classes.
Its bounded class-list reader governs that input when used. It
supplies no archive or restore procedure.

**Fixture `FX-0009-PRIVATE-ARCHIVE-MANIFEST`.** Use an invented
read-only inventory of three files representing an earlier design,
a task list and a source input. Verify their expected names and
digests, restore the three to isolated scratch and check them
byte for byte. Separately omit the task list and alter one design
byte without updating the manifest. This is a three-family recovery sample.
Compare the complete Phase 0
inventory independently and require the canonical private manifest,
allowed redacted summary and owning restore role before any full
preservation claim. An omission or alteration blocks its claim. The source
remains unchanged and the restore stays private.

**Expected proof.** Private manifest, fixture and scratch result
digests, per-entry observations and approving review. The owner
needs the stable archive and restore design binding for this task.

#### TASK-0076-BASELINE-DESIGN

**Owner and anchors.** Roadmap 0.2. See
[spec 0076](../specs/0076-specification-completeness.md), section
“Conflict, duplication and overlap control”, and
[ADR-0020](../adr/0020-version-1-documentation-and-architecture-baseline.md),
section “Decision hierarchy”.
Reuse FX-0076-INVENTORY-DIGEST, FX-0076-CONFLICT-LIMIT and the
existing reverse-trace checks.

**Fixture `FX-0076-BASELINE-OWNERSHIP`.** Give an invented baseline
one dated current decision, one explicitly superseded decision,
one behaviour owner and exact references from the roadmap, rules
and specification. Compare it with two separate mutations. One
substitutes an obsolete threshold as current. The other assigns
a second owner. The audit names the conflicting references or
duplicate owners. Only the exact reviewed baseline has the required
inventory and reverse links. The audit does not choose a successor.

**Expected proof.** Candidate and inventory digests, complete
decision-to-design crosswalk, planted discrepancy results and exact
review record. A pre-signing review has the committed bootstrap
evidence defined by AGENTS.md section 7.3.

#### TASK-0062-OWNER-BOOTSTRAP

**Owner and anchors.** Roadmap 0.3–0.4,
[0062](../specs/0062-forgejo-server.md#settings), AGENTS.md sections
4 and 11, and the `gh` pin in `gate/tools.toml`. The initial
owner account and developer client are distinct from the later
repository-scoped mirror credential.

**Fixture `FX-0062-BOOTSTRAP-IDENTITY`.** Keep roadmap 0.3 GitHub
client sign-in separate from 0.4 Forgejo owner setup. Pin each
fixture endpoint, account and authority. Test matched and wrong
gh client pin and sign-in on the first, and owner identity and
protected password custody on the second. Prove separate read-backs. Plant a
synthetic secret
in the service's internal failure and inspect the safe result. The
exact approved client and identity can verify their own step. Each
negative case stops its affected step before any protected change.
No token, password or private-note contents enter the receipt.

**Expected proof.** Client pin identity, redacted owner confirmation,
separate read-back of each admitted identity and evidence for each
refusal. A simulated owner confirmation proves no real enrolment.

#### TASK-0062-HOME

**Owner and anchors.** Roadmap 1.1,
[0062](../specs/0062-forgejo-server.md#settings), Forgejo scope in
AGENTS.md sections 2 and 9, and
[0006](../specs/0006-main-only-publication.md#the-early-commits)
for the bootstrap record boundary.

**Fixture `FX-0062-FIRST-MAIN-READBACK`.** In a disposable repository
model, start with an authorised bootstrap actor and the expected
private repository identity, absent initial ref and approved root
commit and tree. Confirm the exact initial `main` by read-back.
Repeat the original action and test a different repository, wrong
actor and a conflicting existing ref. Exact replay observes the
same initial ref. The other cases perform no write or overwrite.
No mirror action is part of this fixture.

**Expected proof.** Exact repo and actor scope, before and after ref,
commit and tree read-back, step record and applicable bootstrap
review proof. This fixture does not exercise the later door gate.

#### TASK-0060-DELL-RUNNER

**Owner and anchors.** Roadmap 1.2,
[0060](../specs/0060-runner.md#what-it-does), Workflows, Install and
Tests, plus [0059](../specs/0059-pankti.md) for the job
place. Use the Forgejo runner version and hash and Ubuntu 26.04 image
digest in `gate/tools.toml`.

**Fixture `FX-0060-ADMITTED-RUNNER-JOB`.** Use a stand-in registration
for one fixture repository and the `dell` label. Supply the exact
pinned runner and image identities for one no-op job, then test a
changed runner hash, wrong image, label or registration scope.
Only the admitted profile can start the fixture job. Record that its
heavy-job place is held through completion. Pin protected workflow
identity and exact queue lease at launch. The required platform
proof also observes the isolated job network, no host mounts or
container-service socket, and the configured resource caps. A simulated
admission gives no observed closed-network or cap proof.

**Expected proof.** Exact repository, runner and image identities,
job and queue receipts, protected scope observations and refusal
results, with any live runner proof named separately. No sign-up
secret enters a result or fixture record.

Record the source, profile and expected baseline failure before
implementation. Bind observed baseline failure and candidate outcome
separately using the admitted phase evidence. Documentation fixtures
carry no release signature or phase completion.

### Remaining Phase 1 foundation trace designs

These rows complement the implementation catalogue
[foundation slices](implementation-catalogue.md#phase-0-and-1-foundation-slices)
and the earlier foundation trace designs. The `FX-` case labels are
proposed synthetic fixtures. Bind each to the exact owning requirement,
design, variants and test source under
[0076](../specs/0076-specification-completeness.md).
Use disposable or modelled state and invented identities throughout.

#### TASK-0006-REF-GUARD

**Owner and anchors.** Roadmap 1.3,
[0006](../specs/0006-main-only-publication.md#on-forgejo) and
[0001](../specs/0001-door.md#what-it-does), items 2–4.

**Case `FX-0006-REF-GUARD-BOUNDARY`.** Given accepted `main` at A,
submit an admitted fast-forward to B in a disposable repository.
In separate one-ref resets offer a branch, divergent tip, deletion
and a release tag outside previously accepted `main`. Only the
valid update advances `main`. Every rejected single-ref case leaves both refs
at baseline.
Mixed-ref effects have their own 0001 contract.
Use a valid new release tag in a separate positive case only with
the authorised release actor and a commit on both pre-push refs.

**Expected proof.** Admitted profile and actor, candidate and
old/new ref read-backs, request outcomes and matching door record.
Bind the tag and main cases separately. This case verifies reference
policy. It does not repeat the gate's check-inventory proof.

#### TASK-0062-PROTECTED-CHECKSUM

**Owner and anchors.** Roadmap 1.5,
[0062](../specs/0062-forgejo-server.md#settings) for settings,
[0058 baseline acceptance][chaukidar-baseline] for the watch
observation, and [0008 installed tools][protected-install] for any
protected-tool installation. AGENTS.md sections 7.2 and 11 govern
protected gate inputs, hooks, Forgejo settings and GitHub CI custody.

[protected-install]:
  ../specs/0008-trust-boundaries.md#protected-policy-and-installed-tools

[chaukidar-baseline]:
  ../specs/0058-chaukidar.md#accepting-a-replacement-baseline

**Case `FX-0062-PROTECTED-DRIFT`.** Given an approved synthetic
fingerprint inventory, alter one covered input outside its reviewed
route. Read again under the same drift episode and protected inventory.
Lose a mock reply on each sink separately and verify its exact
receipt. Unknown is unresolved, and no live alert is sent. Prove that the
trusted fingerprint does not move.
The independent reports identify one drift incident, with one urgent
Forgejo outcome and one GitHub alert outcome that cannot return
through sync. A separately reviewed authorised replacement records
the exact new fingerprint. A stale approval cannot accept either
version by replay.

Use the exact current reviewed installation and expected watch
definition as the control. Keep the accepted installation current
while presenting a restored older installed snapshot and its valid
expected fingerprint proof. In another case, keep the current
installation and change a watched target after review while
preserving its label and content fingerprint. Each case withholds
replacement and preserves the prior baseline and alarm evidence.

Give one watched source an accepted fingerprint A, then observe
drift to B. Model the legacy bootstrap recording B locally and
clearing its local alarm state. A later local comparison is quiet
because the source still gives B. The trusted fingerprint stays A
and the drift stays unresolved. Separately give a reported resolution
with source read-back unavailable. Independently confirm that the
same current source again matches A to clear its finding.

For the install race, hold an earlier installation's clean result
until after the reviewed replacement takes effect. Repeat with an
earlier changed result. Neither can update the
current alarm state or its sink receipts. The earlier evidence
remains attached to its original installation. Check public outputs
for labels and safe findings only. No fixture opens external alerts,
updates an installed watch list or runs a service.

**Expected proof.** Exact watch definitions in the private inventory,
their prior and proposed fingerprints, the current accepted
installation and policy, approval, independent read-back and
de-duplicated alert evidence from both destinations. Use the actual
signed fingerprint proof when admitted. If it cannot establish the
exact current scope, hold acceptance. A committed bootstrap review
under AGENTS.md section 7.3 proves only the review. The owner must bind
exact fingerprint scope and alert reconciliation before
implementation.

#### TASK-0062-WORK-LIST

**Owner and anchors.** Roadmap 1.6,
[0062](../specs/0062-forgejo-server.md#settings) for Forgejo task scope,
and AGENTS.md sections 7.3, 8 and 10 for bootstrap phase records,
milestone identity and labels.

**Case `FX-0062-WORK-LIST-REPLAY`.** Given an empty fixture work list
and a committed synthetic Phase 0 closure record, initialise each
roadmap phase, the Later milestone and `status/ready`,
`status/working`, `status/blocked` and `status/urgent`. Suppress
the reply after the fixture creates part of the set. Repeat the same intent
and read back existing IDs. Phase 0
retains its already closed state because of its exact bootstrap
proof with the exact assigned milestone. Missing or ambiguous
closure leaves that milestone open. Each phase, Later and each
label has one stable identity.
An ambiguous existing title holds reconciliation without creating
a duplicate or changing any unrelated item.

**Expected proof.** Exact roadmap-to-milestone and label mapping,
closure evidence reference, before and after IDs, and read-back after
replay. Record owner-resolved mapping changes separately.

#### TASK-0076-CARRY-OVER

**Owner and anchors.** Roadmap 1.8,
[ADR-0011](../adr/0011-fresh-start.md#decision), AGENTS.md section 12
and [spec 0076](../specs/0076-specification-completeness.md), section
“Conflict, duplication and overlap control”.
Reuse FX-0076-ALIAS-RETENTION and FX-0076-INVENTORY-DIGEST.

**Case `FX-0076-ARCHIVE-CROSSWALK`.** Given invented earlier source
rows, map an eligible task to its named item in an unopened phase,
one task fitting only an already started phase to Later, and one
recorded alias to its successor. Reconcile forward and backward.
Then omit a source row, supply competing owners and remove the
alias reason. Each mutation blocks the crosswalk and names the
exact row and discrepancy. Source retention is unaffected.

**Expected proof.** Private source and crosswalk digests, exact
destination and owner references, aliases with reasons and complete
two-way coverage. Keep the carry-over closure separate from any
archive-retention decision. No fixture grants a deletion action.

#### TASK-0002-MIRROR-REPORTING

**Owner and anchors.** Roadmap 1.10 owns the owner setup action.
[0002](../specs/0002-forge-sync.md#what-it-does) owns the sync boundary
for public issue fields and disclosure checks, alongside AGENTS.md
section 9.1. This operating route needs its own owner proof.

**Case `FX-0002-PRIVATE-REPORT-SCOPE`.** On an in-memory mirror
model, turn the authorised private-report capability on and read
back its exact repository scope. Give it an invented private
report marker in a restricted record. Verify absence from public
issue results and the ordinary sync field projection. Repeat with
an unauthorised actor, unavailable setting and different repository.
Each of those lacks proof of a usable private reporting route.
A planted leak fails the case without copying the marker.

**Expected proof.** Redacted owner-authorised setting read-back and
independent public projection observation for the same repository,
with a missing or unverified current setting left open. The simulation
proves neither owner access nor current public mirror settings.
The case holds no real report or vulnerability material.

#### TASK-0006-MIRROR-RULES

**Owner and anchors.** Roadmap 1.3 and 1.9,
[0006](../specs/0006-main-only-publication.md#rulesets) and its
publication step 6, AGENTS.md section 9 and [0002](../specs/0002-forge-sync.md).

**Case `FX-0006-MIRROR-RECHECK`.** In a fake mirror, compare the
approved two main rulesets, the non-main branch and tag rules and
repository settings against live-shaped observations. Exercise
one exact authorised fast-forward. Refuse an ordinary actor, forced
sync update and forbidden ref. Change a rule after the initial read
and lose a reply in a separate run. Pin preflight base, door-accepted source,
policy and target
generations. Recheck within ten minutes before each pending write.
The recovery check refuses stale authority.
If the exact authorised update already landed, read it back as one
effect without repeating the write.

**Expected proof.** Owner-approved rules and bypass scope, dated
rules reads and actor identity, exact source/target ref comparison
and safe refusal or one authorised read-back. A modelled result
cannot qualify current owner settings or public mirror enforcement.

Record expected baseline failure and fixture identity before a
test is written. Keep future observed baseline failure, source
inspection and candidate outcome separate. An exact synthetic
result grants no live access, signature, publication or phase exit.

### Protected foundation task trace expansion

These five existing [catalogue tasks](implementation-catalogue.md)
use the phase order in the [roadmap](../roadmap.md). The proposed
case IDs identify designs only. The owning specs supply the
requirements and interfaces. Use the named sections where a stable
`REQ`, `DES` or `VAR` binding is still needed under
[spec 0076](../specs/0076-specification-completeness.md).

| Existing task and owning anchors | Proposed case and bounded input | Expected result | Expected evidence | Remaining bindings |
| --- | --- | --- | --- | --- |
| `TASK-0003-CHECK-MAP`, [0003](../specs/0003-toolchain-and-check-runner.md), Entry points and profiles 1–3, Cutover and dependency evidence 1–2, Results 1–2. Roadmap Phase 2 item 2. | `IT-0003-CHECK-MAP-COVERAGE`. In a sealed synthetic candidate's exact admitted tier profile, compare a complete check map with one missing check, a duplicate check and a required check marked inapplicable without scope evidence. | Only the complete eligible map supports a verified gate result. The planted variants cannot pass and name the discrepant check. An omitted check never becomes inapplicable. | Trusted gate result, exact catalogue and profile digests, expected check identities and dispositions, linked planted inputs and output identities. | Owner's stable requirement, design and variant IDs for the exact tier/language map and inapplicable evidence. Bind the generated case enumeration, baseline failure reason and exact suite entry to the accepted policy. Each admitted language and required tier has its exact protected check list and planted case. Distinguish evidenced N/A from missing, duplicate or incomplete worker evidence. |
| `TASK-0003-PINNED-INPUTS`, [0003](../specs/0003-toolchain-and-check-runner.md), Cutover and dependency evidence 3, The locked-down box 3–4, Tools 5. AGENTS.md sections 4 and 5.7. Roadmap Phase 2 item 3. | `IT-0003-PINNED-INPUT-MISMATCH`. In separate sealed synthetic runs substitute one pinned tool's bytes and then omit one transitive dependency from the matching inventory. Leave the same labels and version claims in place. | The altered executable cannot start. The missing dependency cannot qualify the inventory or run. The clean matching fixture can establish only its scoped result. | Trusted pin and inventory verifier output, exact candidate, tool, graph or lock, licence and SBOM bindings, and scrubbed refusal reasons. | Owner's stable inventory and admission requirement/design IDs and exact tool, dependency and platform variant IDs. Bind the protected catalogue entry, allowed SBOM scope and fixture digest to the current profile. Future Bazel proof needs its own exact mapping. Separately remove a required licence, change or omit image digest and try a writable staged input. Check before staging, after sealing and after the run with current licence, vulnerability and vetting inputs. |
| `TASK-0065-RUN-SITE-SIGN`, [0065](../specs/0065-tijori.md), Signing purposes 1–5, Callers and their keys 1–2 and Checking signatures 4–6. [0003](../specs/0003-toolchain-and-check-runner.md), Results 4, and [0021](../specs/0021-forum-contracts-ingestion.md), signed request and replay contract. Use [0008](../specs/0008-trust-boundaries.md#each-checker-checks-for-itself) for the ordinary verdict and [0010](../specs/0010-coding-harness.md#phase-acceptance-gate-run) for phase acceptance. Roadmap Phase 2 item 8. | `IT-0065-RUN-SITE-PURPOSE`. Use synthetic runner, gate and site identities. Verify one exact `evidence-bundle` and one exact `site-message`, then offer each to the other verifier and vary the tree, site or request identity independently. Replay one admitted site attempt. Verify the gate account's ordinary `gate-run` verdict for the exact candidate and protected profile. Refuse a runner account request for that purpose. Reuse 0010's same-purpose wrong-record-type case: offer the correctly signed ordinary verdict as a phase receipt. Use a separate `phase-gate-run` control that matches the protected phase, candidate, policy and expected role set. | Signatures remain purpose-scoped. The owning caller refuses mismatched identities and the site route refuses replay. The ordinary verdict verifies only in its own route. The phase verifier refuses the ordinary record despite the matching signature purpose. Only the exact phase control passes its own fixture check. No second site effect, live gate pass, phase transition or extra signature authority follows. | Purpose-specific signing and check-library fixture evidence, canonical bytes and policy digest, plus separate candidate verdict, site refusal and phase record-type verifier outcomes. | Owner's stable signing and interface IDs and distinct Dell runner, gate and paired-site variants. Bind caller verification cases to the exact canonical encodings and platform fixtures. The signer alone establishes no accepted tree, site or request claim. Verify exact base, profile, plan, pins, maximum age and 0118 current paired frame where relevant. Keep logical replay and signing purpose distinct. AGENTS 7.3 committed review proof grants no signed evidence-bundle, gate-run or site replay proof. Keep the ordinary verdict and protected phase receipt as distinct record types. |
| `TASK-0065-OWNER-KEY`, [0065](../specs/0065-tijori.md), Instances and crates 2 and Checking signatures 1–3. AGENTS.md section 7.2. Roadmap Phase 2 item 12. | `BDD-0065-OWNER-KEY-BOOTSTRAP`. Generate a synthetic test key in the owner-side fixture. Approve an otherwise eligible test waiver in the owner view and export only the public bytes. With no pin on protected main, offer even the matching pin in the candidate and require refusal. After the exact public pin is accepted on protected main in the fixture, verify the same waiver. Keep that pin fixed and refuse an otherwise matching waiver signed by a different test key. Separately offer a candidate-supplied alternate pin and an owner socket request without live approval. | Dell verifies only the exact protected public key and record. The alternate pin and unconfirmed signing request confer no waiver or policy authority. Private key bytes stay in the owner-side fixture. | Candidate-specific public fingerprint and trusted verifier refusal, owner-view fixture confirmation and safe evidence that the exported artefact contains only the public key. The real owner action has its own recorded confirmation. | Owner's stable bootstrap and trust-source IDs, pinned-public-key and missing or altered-key variants, and exact confirmation receipt binding. A synthetic pass cannot establish the real owner key or satisfy the owner action. Require exact protected-main gate/owner-key.pub, refusing missing and candidate alternate pin. Verify owner display and decision over the exact bytes and purpose. Synthetic key proof grants no owner authority. |
| `TASK-0065-BOOT-SIGNER`, [0065](../specs/0065-tijori.md), Instances and crates 3–4, Signing purposes 2–5 and Interfaces 4–6. Shared [0106](../specs/0106-behemoth-core-container-and-config.md) BB-01 and BB-03. Roadmap Phase 3 item 8. | `IT-0065-BOOT-SIGNER-CONFORMANCE`. Compare one exact signed-site fixture through the original signer route and the explicitly wired product route. Use the same policy and test keys. Then request the wrong purpose and check the gate and sync dependency graph. | Both product signer paths give the same scoped decision on equal inputs and refuse the substituted purpose. The product graph uses typed wiring. The operations tools do not need a Boot service, and test evidence grants no live cutover. | Baseline and candidate identities, exact fixture and policy digests, signer verdicts and independently checked dependency-graph evidence for the product and operations tools. | Owner's stable wiring and purpose bindings, service and operations variants, and exact before/after fixture entry and inject-at-start failure. Link the distinct 0106 requirements to their original owner and supply the phase's recorded evidence separately. Exercise distinct missing and ambiguous BB-03 providers and expected SUT-932/SUT-937 before any signing. Gate and forge-sync remain off Boot. Separate source locator, baseline failure and signed candidate evidence. |

In IT-0065-BOOT-SIGNER-CONFORMANCE, start the product through
its exact admitted Boot cell and product signing fixture. Admit
one synthetic site-message signing request with the correct
policy and test key. Commit its signature and indexes, then
hold the local reply. Explicit Quit closes new product calls.
Reach the admitted drain bound and stop the old caller.
Deliver the late reply after that caller is fenced. It changes
no lifecycle state, job or site effect. Start the eligible
product fixture again, using the same principal, purpose and
bytes with current authority and no revoked key. Return the
same durable signature and counter. Verify one signature entry
for that request. Bind FI-BB-02 and IT-BB-01 to the product
lifecycle. Bind the signature and log proof to 0065. Keep
independent gate and forge-sync checks separate. The fixture
uses no real key or site send.

Reverse-check each proposed case through the owning requirement or
section back to the exact task. Before implementation, record the
fixture bytes, suite entry, admitted profile, expected baseline
failure and observable effects. Record actual failure and later
candidate outcome separately with the required review and verified
receipts. A planted refusal tests the guard. It performs no phase
transition, waiver, key enrolment, signing or publication.

### Phase 2 research, assurance and key trace expansion

These rows extend the existing
[implementation catalogue](implementation-catalogue.md) with one
proposed case per task. Use the [Phase 2 roadmap](../roadmap.md)
and [spec 0076](../specs/0076-specification-completeness.md) for the
owning bindings. Each case uses synthetic inputs and test keys where
needed. The expected evidence below describes future verification.

| Existing task and owning anchors | Proposed case | Given | When | Then | Expected signed evidence and remaining binding |
| --- | --- | --- | --- | --- | --- |
| `TASK-0013-RESEARCH-RECORD`. [0013](../specs/0013-research-rules.md), Research first 2–3, Independent route and reviewer, The receipt 1–3. [0012](../specs/0012-ganit-evolve.md), The effect contract, owns the additional Ganit acceptance fields. | `IT-0013-CARD-SCOPE`, synthetic accepted card and exact-path change. | One fixture card names one exact path, a falsifiable test and the evidence required to support it. An eligible independent product signs the matching acceptance. | Check the exact card and scoped change. Separately substitute the card version, remove the signature, change a bound fixture or name a path outside the card. | Only exact current acceptance can establish the stated scope. Changed, unsigned or out-of-scope inputs hold. A card with an unproved value gives no implementation permission. | Exact card, independent acceptance and tested-input digests, named verifier outcomes and receipt identity. Bind the stable card and scope requirement/design/variant IDs, the applicable acceptance and proof profiles, and the deliberate baseline failure. No private research content is copied into shared evidence. |
| `TASK-0012-SEALED-TRIAL`. [0012](../specs/0012-ganit-evolve.md), Evaluation steps, The effect contract and Build order 1–2. [0003](../specs/0003-toolchain-and-check-runner.md), The locked-down box, owns the runner boundary. | `IT-0012-SEALED-TRIAL-INTERRUPTED`, fixed synthetic trial. | A scoped accepted card, pinned fixture, input tree, tool and sandbox identity, with an eligible admitted baseline and deterministic oracle. | Interrupt one worker before a trusted result and retry through the existing sealed route using the same inputs. Offer output from the stopped worker and from a simulated unsealed attempt. | Neither interrupted nor unsealed output earns a successful receipt. Only exact fresh sealed work can verify against the oracle. A repeat cannot turn stale bytes into a second accepted result. | Trusted runner and trial receipts bind card, candidate, fixture, oracle and tool digests, sandbox refusal, attempt identity and measured result. AGENTS 7.3 committed records apply only to review and phase proof before signing. The 0012 unchanged adapter admission is separate from exact sealed runtime results. Bind the owning problem and effect profile, exact fixture and crash point. Trial evidence cannot substitute for the source-change receipt or phase exit. |
| `TASK-0061-COVERAGE-PROOF`. [0061](../specs/0061-nirikshak.md), How it works and Rules M08, own analyser facts and reachability. [ADR-0020](../adr/0020-version-1-documentation-and-architecture-baseline.md) and [0076](../specs/0076-specification-completeness.md), Risk-based assurance, own the strict-core denominator and equivalent-mutant proof. | `IT-0061-EQUIVALENT-PROOF-BOUNDARY`, hand-worked strict-core decision and mutation fixture. | One exact strict-core operation, an independently computed decision table, one killed mutant and one claimed equivalent. | Compare the expected MC/DC independence pairs and mutation counts. Separately omit an independence pair, leave a killable mutant surviving and remove the independent review of the equivalent-mutant proof. | Missing independence evidence or a surviving killable mutant refuses. Exclude an equivalent mutant only with its exact proof and independent review. A name-based reachability result cannot establish MC/DC. | Signed gate result with strict-core scope, pinned coverage and mutation checks, oracle and mutant identities, denominators and reviewed equivalence proof. Bind the actual MC/DC and mutation runner, catalogue and case IDs to the owning assurance policy. The analyser's M08 evidence alone cannot close these bindings. |
| `TASK-0065-RELEASE-KEY`. [0065](../specs/0065-tijori.md), Instances and crates 1, Signing 2–3, Release guards and Interfaces 2. [0005](../specs/0005-release-coordination.md) owns exact candidate, approval and final release. | `IT-0065-RELEASE-KEY-CUSTODY`, using the custody and refusal cases in Phase 2 | Use the admitted synthetic custody profile and scoped key identity. The dependent Phase 5 positive fixture uses a separately admitted test release key and exact eligible manifest, fixture ledger and confirmed head | In Phase 2 verify the protected key identity. Probe private-key export, generic signing, wrong purpose and unauthorised caller. In Phase 5 use the dedicated interface with the eligible test manifest and reconcile a lost reply. Vary the declared digest | The Phase 2 probes refuse without a release signature or publication. The eligible Phase 5 test signature binds only the fixture manifest and reconciles to its original result. A changed digest refuses | Phase 2 custody and exact refusal evidence bind key identity, caller, purpose and policy, with gate and independent review evidence. Keep the Phase 5 test signature and log position in the dependent integration record. Phase 7 requires its own exact ledger, head and live release authority |
| `TASK-0001-COMMIT-HOOK`. [0001](../specs/0001-door.md), What it does and How it is tested, owns the independent push door. [0003](../specs/0003-toolchain-and-check-runner.md), Evidence and timing 1–2, and [0010](../specs/0010-coding-harness.md), commit-hook staged-content rule, give the shared exact-input boundary. Roadmap Phase 2 item 10 owns the Rust hook step. | `IT-0001-COMMIT-EXACT-INDEX`, local scratch repository only. | A staged candidate with the exact eligible commit-tier profile and a different unstaged working-tree change. | Attempt the fixture commit with the current staged receipt. Separately remove a required tool, substitute another tree's receipt and alter the index after checks. | Only the exact checked staged bytes can be committed by the eligible route. Required-input failures leave the scratch ref unchanged, and working-tree bytes do not substitute for the staged bytes. The local receipt grants no push authority. | Commit-tier signed runner evidence, and separate AGENTS 7.3 committed review or phase proof where allowed, with staged tree, base, profile and check pins and scratch ref before and after. Bind the Rust hook executable, stable local requirement/design and profile variants, and the exact commit/ref refusal fixture. Keep the independent push-door case under its own task. |

Under [0065 signature-log recovery][tijori-index-recovery],
`FI-0065-INDEX-RECOVERY` exercises the Phase 2 item 8 signing core
with synthetic keys. Bind the same stopped instance, original head
counter and hash, owner approval, both index counters and running
hashes from the earlier signature entry, and the recovery entry
whose predecessor is the original head. With only staged rebuilt
indexes or only the recovery entry, restart keeps the existing
signing stop.
With both parts durable and verified, reconcile the original record
after a lost reply and refuse to reapply it. A replacement head
refuses an uncommitted step. This proof uses the existing recovery
purpose and carries no live key, signer or release approval.

[tijori-index-recovery]: ../specs/0065-tijori.md#the-signature-log

For imported watch outcomes, reuse FX-0062-PROTECTED-DRIFT. In a
separate synthetic branch deliver a current alert to its first
authorised sink and retain the verified receipt. Advance the
installation before the send to the second sink. The old attempt
cannot send there or update the new alarm state. No external call
or operator action is part of this fixture.

For each row reverse-link the owning requirement, design, applicable
variant and proposed case to the exact task. A named case needs its
fixture digest, suite entry, baseline failure reason and later exact
candidate outcome before it supplies execution evidence. Preserve
current protected policy, scope and independent review records.

### Phase 2 early gate handoff

Prepare exact tasks under [0076](../specs/0076-specification-completeness.md)
after the accepted Phase 1 exit. Use
[0012's owning trace](../specs/0012-ganit-evolve.md#early-gate-handoff)
and its Build order. The first cases exercise the checker and safety
paths before the numbered work. TASK-0012-SEALED-TRIAL keeps its
separate sealed attempt and retry case.

| Owning requirement, variant and case | Synthetic fixture and expected outcome |
|---|---|
| REQ-0012-CHECK-BUILD · VAR-0012-BOOTSTRAP-CHECK · UT-0012-CHECK-EXACT-SOURCE | FX-0012-RECEIPT-CANDIDATE gives one changed Rust source and a synthetic matching receipt for hash A. Test the actual checker on the matching capture, then on source hash B under the same receipt and on hash A without a receipt. The checker accepts the exact match and refuses both mutations. A separate documentation-only capture creates no source receipt requirement. The pre-change case must show the expected missing check or incorrect result before implementation. |
| REQ-0012-RUST-SAFETY · VAR-0012-RUST-NUMERICAL and VAR-0012-RUST-OPERATIONS · IT-0012-BOOTSTRAP-RUST-GRANTS | FX-0012-RUST-FIXTURE-GRANT gives one whole-file Rust fixture that reads the one-byte synthetic fixture through an independently admitted unchanged adapter under the existing operations contract. The numerical counterpart refuses the effect. The operations fixture can take only the declared action within its pinned bound. Use separate pinned fixture profiles before and after Build order step 2. Before it, refuse the same grant-bearing problem before launch and produce no receipt. After it, permit the bounded action only through the exact independently admitted unchanged adapter. Separately, a pinned fixture-call tool uses its exact entry with an explicit true flag. Aegis treats a missing flag as false, and a tool marked false cannot be called. Changed fixture-adapter bytes lose their original admission. In a separate admission branch keep the accepted Phase 1 proof and other scope checks fixed. Offer the valid step 1 checker result for a checker absent from the proposed step 2 base, with the same grants fixture outputs. Hold ready, claim and candidate acceptance. A later accepted base with that exact checker and still-applicable evidence satisfies only this dependency. Every refusal has no real external effect. |
| REQ-0012-RECEIPT-ENABLE · VAR-0012-RECEIPT-ENFORCEMENT · IT-0012-BOOTSTRAP-ENABLE | FX-0012-RUST-ENABLE uses two protected policy generations. Give the first a passing checker fixture with no Rust receipt capability. Offer a synthetic capability and eligible card fixture to exercise the governing activation predicate. Under the enabled fixture policy, require matching evidence for changed Rust source and refuse stale evidence as in TASK-0012-SOURCE-RECEIPT. Repeat with a missing capability or an unready card path. Neither incomplete prerequisite admits activation. |

Bind each result to the owning design, exact fixture, base and
candidate, admitted check profile and tool pins. Record the run
outcome and independent reviews under the existing authority.
0076 supplies the exact task derivation fields. The proposed
fixture authorities supply no signed trial, candidate gate pass
or accepted phase exit.

### Remaining Phase 2 trace designs

These tasks come from the Phase 2 and Phase 4 sections of the
[implementation catalogue](implementation-catalogue.md). The proposed
case IDs identify synthetic designs. Apply
[spec 0076](../specs/0076-specification-completeness.md) for the owning
requirements, variants, exact evidence and reverse trace.

| Existing task and owning anchor | Proposed case and Given | When | Then | Expected proof and remaining binding |
| --- | --- | --- | --- | --- |
| `TASK-0011-RULE-MATRIX`, [0011](../specs/0011-engineering-standard.md), Rules that hold throughout 1–5, GG-04 finding shape and Build order. AGENTS 6.3 owns languages and risk matrix. [0003](../specs/0003-toolchain-and-check-runner.md) owns profiles and outcomes. Roadmap Phase 2 item 4. | `IT-0011-RULE-MATRIX-PLANTED`. Given one declared, supported language and rule row per applicable tier, with an independent clean and deliberately failing fixture. | Run the exact profile against each fixture. Then omit a required cell, duplicate its identifier and submit an unexplained inapplicable cell. | The clean cell can pass its rule and the planted case names its precise finding. A separately evidenced applicable N/A is distinct from omission. A missing, ambiguous or unexplained cell cannot qualify the matrix or remove a check. | Expected signed rule-to-check and fixture evidence, reported finding, policy identity and independent matrix review. Bind each rule's exact admitted language and tier, expected baseline failure and stable owning requirement/design and variants. This case does not complete the separate strict-core coverage work. |
| `TASK-0003-PROJECT-GUARDS`, [0003](../specs/0003-toolchain-and-check-runner.md), Entry points and profiles 1–4, Results 2–4, Who may decide and [Documentation profile evidence](../specs/0003-toolchain-and-check-runner.md#documentation-profile-evidence). `REQ-0003-DOC-PROFILE`, `DES-0003-DOC-MODES`, `VAR-0003-DOC-MODE-CHANGE`. AGENTS.md sections 2, 4 and 7 and roadmap Phase 2 item 9 own the norms. | `IT-0003-PROJECT-GUARDS-REFUSAL`. Given a scratch candidate, protected runner policy, exact staged tree and synthetic review and waiver records. | In separate variants plant a disallowed type, a status statement in docs, an invalid review or required source receipt and a wrong-purpose or unapproved waiver. Offer the candidate's own permissive rule copy and an executable change labelled docs-only. In a separate fixture leave Markdown bytes and path identical and change only its executable bit. Reverse which side has that bit in another fixture. For a documentation-only candidate, present a push bundle signed with test keys for the exact protected request. Keep the push request and its commit and tree identities fixed. Substitute the correctly signed commit-tier bundle from its own test plan for the same base, candidate tree and policy. Separately use a scratch chain A, B and C. An inert Markdown path is executable at A, non-executable at B and edited without changing that mode at C. Use the existing runner role and a test key to sign the documentation-only push bundle for B to C. Keep C's exact commit and tree, push tier and policy fixed and offer that bundle against the protected push request from A to C. In a separate documentation-only fixture, a fixed synthetic protected rule requires Route A and refuses Route B for one Platform A operation. Candidate Markdown requires Route B and refuses Route A for the same platform, actor, operation and scope. In a separate post-cutover staged fixture, fix an inert documentation-only candidate, the protected commit profile, policy and exact accepted base. Offer an otherwise valid test-signed bundle bound to a different base commit with identical tree, mode and profile. In a separate branch request a bundle with the exact eligible raw check evidence and a claimed supervisor name from a different authenticated socket peer. Then offer the passing unsigned diagnostic from the retired bootstrap path for those same bytes. | Each applicable guard produces its safe scoped refusal. The trusted profile still runs required documentation guards and selects the code profile if the changed path was executable on either side. A prior evidence bundle for the path's earlier mode cannot verify the changed tree. The signature check accepts the valid signature. The 0003 candidate verifier refuses the substituted tier against the protected push plan even if the supplied checks pass. For A to C the trusted verifier requires the code profile from A's executable mode and refuses the valid B to C bundle for its mismatched base and profile. The door retains its own checks. For the synthetic platform case, the consistency guard identifies both source clauses and the shared operation and refuses the documentation candidate. Candidate policy cannot supply authority or waive a guard. Refuse the valid signature bound to the other base. The other peer gets no evidence-bundle signature. After the accepted cutover, the retired diagnostic grants no gate pass and cannot advance the candidate or change the current policy. | Signed runner result with each named guard, plant digest, selected profile, protected policy and verified review/waiver outcome. Bind each guard to its exact catalogue ID and fixture, including the owning signer/admission boundaries. For the contradiction fixture bind both source references and their shared scope. For the staged boundary use 0008 protected policy and 0065 caller and purpose rules. Bind both base identities, the authenticated test peer, raw-result and candidate digests, exact active policy generation and its synthetic cutover proof. Record separate refusal reasons. The matching synthetic control gives only fixture evidence. Rules or variants without a concrete proof stay open. |

### WordPress Plugin Check trace

For `TASK-0003-CHECK-MAP` and `TASK-0003-PINNED-INPUTS`, use
[IT-0003-WP-PLUGIN-CHECK][wp-check-cases]. Qualify clean and planted
static and runtime outcomes, no-pass failure cases and every tier
binding under QR-247. The synthetic Phase 2 proof has no product
scope. `TASK-0031-SITE-FIXTURES` supplies the exact clean profiles
for both WordPress majors before the first WordPress plugin candidate.
`TASK-0005-FULL-RELEASE-RUN` consumes the fresh result on the exact
ZIP under 0005. Keep check completion, native hand-off and release
approval receipts distinct. The protected observer must account for
every finding in the report as well as every completed check.
Exercise the exact-target and independent-finding refusals in the
same acceptance suite, including when the plugin bytes stay equal.
In the interrupted release retry, keep the WordPress ZIP and its
approved inputs identical. Bind the supervisor's first and second
profile results and teardown to the current protected attempt.
Block an earlier first-profile result combined with a current
second-profile pass. A fresh complete control still needs the
separate native release evidence and approval.

[wp-check-cases]:
  ../specs/0003-toolchain-and-check-runner.md#wordpress-plugin-check-acceptance

### Door and sync bootstrap BDD pilot

Start from the foundation rows for `TASK-0001-DOOR-GATE` and
`TASK-0002-SYNC`. The fixture labels below index these scenarios.
Use the owning specs to bind each scenario to its stable requirement,
design, variant, error where applicable and test identity under spec
0076. The original foundation rows carry the task links. Forgejo
owns the live task and its execution records.

#### Source evidence for the pilot

| Task and owning behaviour | Executable source present in the checkout | What the source covers |
|---|---|---|
| `TASK-0001-DOOR-GATE`, [0001](../specs/0001-door.md), What it does, items 1–4 | [Door hook test](../../crates/aegis-door/tests/hook.rs), `the_hook_lets_good_main_in_and_keeps_bad_pushes_out`; [door rules tests](../../crates/aegis-door/tests/rules.rs), `a_good_push_passes_and_is_remembered` and `a_bad_message_is_refused_and_not_remembered` | The bare-repository hook checks exit status for the valid and invalid pushes. The independent scratch rule tests inspect their local accepted-ref record. A Forgejo adapter and exact paired ref reads are separate designed evidence. |
| `TASK-0002-SYNC`, [0002](../specs/0002-forge-sync.md), What it does, items 1–4 and the copy-identity rule | [Sync runs tests](../../crates/forge-sync/tests/runs.rs), `copies_everything_github_lacks` and `the_record_stops_a_second_copy`; [sync failures tests](../../crates/forge-sync/tests/failures.rs), `private_text_stops_everything` and `failed_writes_are_problems`; [copy-marker predicate test](../../crates/forge-sync/src/plan_tests.rs), `bad_markers_are_not_copies` | Stand-in commands observe the planned successful copy and the suppression of a second issue creation after a recorded success. The failures cover private-text refusal and commands reporting write failure. The marker predicate tests no actual sink write. |

The source locators establish fixture coverage by inspection.
The signed evidence expected below is separate from the source
locators. The fixtures use only disposable repositories and sinks.

#### Scenario designs

Here, `M0` is the last accepted commit and `M1` is an eligible
descendant. Define a separate candidate `M2` under the same old
reference with a changed tree. Before each scenario capture the
test profile, allowed refs and destinations, baseline refs and the
fixture digest. The door fixture submits only one ref, `main`, to
avoid mixing the door's per-ref decision with Forgejo's decision
over a push containing several refs.

| Pilot fixture and forward trace | Given | When | Then |
|---|---|---|---|
| `BDD-0001-DOOR-CLEAN`. `TASK-0001-DOOR-GATE` → 0001, What it does, items 1–4 → hook and good-rule source cases | An authorised fixture pusher, Forgejo and the door agree on `M0`. `M1` passes every applicable check in the admitted bootstrap profile. | The pusher submits the one-ref update to `M1` through the pre-receive boundary, with local hooks skipped. | The door's accepted reference and Forgejo's `main` each read back as `M1`. The clean case binds both reads to the same old and new commit pair. |
| `BDD-0001-DOOR-REFUSE`. `TASK-0001-DOOR-GATE` → 0001, What it does, items 2–4 → hook and bad-message source cases | Both baseline refs are `M0`. The otherwise eligible test push contains an invalid commit subject. | The pusher submits it through the same boundary. | The observed check reports `commit.message`. Both refs stay at `M0`. A later clean push follows the original baseline without a rejected receipt becoming an accepted one. |
| `BDD-0001-DOOR-WRONG-TREE`. `TASK-0001-DOOR-GATE` → 0001, What it does, items 2–4 and 0003, exact candidate and profile binding → designed only | The fixture uses an admitted evidence-verification profile. The trusted result for `M1` is valid under that profile but `M2` has a different tree digest. | The fixture presents `M1`'s result to the verifier for `M2`. Repeat with an omitted required result in the same profile. | Both attempts hold at the verifier. Neither ref advances. The verdict records wrong-candidate or missing-result evidence without treating the earlier pass as current. |
| `BDD-0001-DOOR-WRONG-COMMIT`. `TASK-0001-DOOR-GATE` → [0001 exact push evidence](../specs/0001-door.md#exact-push-evidence) → `REQ-0001-PUSH-IDENTITY`, `DES-0001-PUSH-IDENTITY`, `VAR-0001-PUSH-SAME-TREE` | Use two otherwise eligible commits with the same checked tree, common base `M0` and distinct commit metadata. The first has complete evidence under the admitted one-ref push profile. | Offer that evidence for the second commit. Then submit the first commit with matching evidence and the unchanged base. | The mismatched candidate refuses and both refs stay at `M0`. The eligible exact commit advances both once. Bind the candidate and tree of each attempt, base, profile and exact ref reads to the expected result. Fixture digest, suite entry and execution receipt remain required. |
| `BDD-0002-SYNC-CLEAN`. `TASK-0002-SYNC` → 0002, What it does, items 1–4 → copy and recorded replay source cases | Forgejo accepted `M1`, the disposable mirror is at `M0`, and the eligible fixture task has one allowlisted title and body. Any applicable campaign permits this sync. Privacy checks have the required inputs. | The configured fixture sync runs, reads back the sink and runs the same eligible update again. | The mirror `main` matches `M1`. Its task has the exact allowed projection and one copy under the sync account. Replay after the saved success creates no second copy. |
| `BDD-0002-SYNC-PRIVATE-TEXT`. `TASK-0002-SYNC` → 0002, What it does, items 2–3 → private-text source case | The mirror is at `M0`. A would-be outbound title contains a harmless planted token on the fixture's private-word list. | The sync preflights the full outgoing plan. | It refuses publishing the planned mirror updates. The sink reference and task count stay unchanged. The fault receipt records the privacy check outcome without the planted value. |
| `BDD-0002-SYNC-LOST-REF-REPLY`. `TASK-0002-SYNC` and `TASK-0002-SYNC-AUTHORITY` → [0002 exact mirror publication](../specs/0002-forge-sync.md#exact-mirror-publication), `REQ-0002-SYNC-IDENTITY`, `DES-0002-SYNC-IDENTITY`, `VAR-0002-SYNC-LOST-REF-REPLY` and 0006, steps 1–8 and Recovery → designed only | Pin the accepted door lineage, exact source commit and tree, configured sink, base M0, expected M1, campaign, allowlist and privacy profile. Use a controllable disposable sink. | Commit M1 there and withhold its acknowledgement. Retry the original operation. Separately try M0 still present, a different commit with the M1 tree, exact M1 in an unconfigured repository, stalled campaign and unavailable final checks. | Exact M1 and tree in the configured repository reconcile one effect. M0 permits resumption only after fresh rule and ref checks under step 6. A wrong tip or repository, blocked campaign or missing check holds. Refuse outbound writes and keep one scrubbed local urgent record where required. Fixture digest, suite entry and exact admitted evidence remain required. |
| `BDD-0002-SYNC-LOST-ISSUE-REPLY`. `TASK-0002-SYNC` → 0002 copy identity and 9.1 venue authority → designed only | Use exact Forgejo issue marker, authenticated configured sync account and sink repository, and approved field projection. Capture a zero-copy baseline in a disposable sink. | Create the intended single issue and suppress acknowledgement. Retry with local receipt absent. Also present multiple matches, wrong author and conflicting text. | Verify one copy after commit and after retry using author, marker, repo and exact eligible projection. Ambiguity holds without another POST and with safe local evidence. A marker predicate alone proves no remote effect. |

The scratch hook and fake command tests cover parts of the clean
and planted-refusal scenarios. Exact paired ref reads and a clean
push after refusal need the new fixture. In particular, recorded replay
starts with a saved result. A command reporting failure does not
establish that a remote write committed before its reply was lost.
Wrong-tree receipt verification and that lost-reply boundary need
their own exact fixtures.

#### Expected receipts and reverse trace

The trusted door runner's evidence binds the repository and ref,
old and new commit and tree, admitted check inventory and profile,
fixture and input digests, tier, tool identity, outcomes and both
independent ref read-backs. The sync runner's evidence binds the
accepted source commit, configured destination identity, allowlist
and privacy-check revision, fixture digest, copy or attempt identity,
safe check results and sink read-back. The lost-reply evidence
includes the effect before the suppressed acknowledgement and the
observed count after retry. Keep private text and credentials outside
these records.

Where the signing and custody profiles are admitted, use the existing
purpose-bound verifier and bind the signed results to their precise
candidate and trusted producer. A changed candidate, fixture, scope,
purpose or destination cannot reuse a result.

Under AGENTS.md 7.3, a pre-signing phase record names reviewers and
results and is copied to the Forgejo milestone as prescribed. Keep
7.1 reviews, 7.2 protected changes and the applicable then-admitted
tier distinct. A bootstrap record carries no signed release verdict. Do not
substitute those records for the full
phase proof in `TASK-0003-PHASE-PROOF`. No signing purpose is allocated
by this pilot.

For the reverse check, start in 0001 at the acceptance or refusal
rule, or in 0002 at the allowlist, privacy or copy-identity rule.
Resolve the owning requirement and applicable variant to the same
task, scenario, source case or proposed fixture and expected receipt.
The exact push identity links are in
[spec 0001](../specs/0001-door.md#exact-push-evidence).
Other paths still need their complete REQ, DES, variant and applicable
error bindings under 0076. Use the BDD identifiers above as design
locators. Bind the exact fixture and admitted evidence producer before
counting a run. Source inspection records no execution verdict.

## Phase 3 foundation handoff checks

Follow [the roadmap](../roadmap.md) and
[spec 0139](../specs/0139-full-foundation-refactor.md) for stage
order. Reuse REQ-0139-ORDER and BDD-0139-ORDER. These proposed
fixtures extend that case without creating implementation tasks.
The catalogue's existing 87 task IDs keep their original owners.
Derive refactor tasks from the reviewed inventory. Derive framework
tasks from applicable source-confirmed 0081 rows with complete
designs under spec 0076. Stage exit requires verified parity.
Allocate task IDs only for exact derived and admitted scopes.

| Incoming work class | Evidence before it can start |
| --- | --- |
| 3A foundation refactor | Exact accepted Phase 2 exit and a derived in-scope refactor task under 0139. |
| 3B framework implementation and qualification | Exact current signed 3A exit and a derived framework task in an applicable 0081 row. Framework fixtures and their contained host applications stay in this work class. |
| 3C first product integration and later product phases | Current signed 3A and 3B exits, the exact qualified Boot artefact set consumed, and the task's own phase prerequisites. Apply this to every existing downstream row, even one whose dependency cell names a narrower prerequisite. |
| Documentation and isolated trace exercises | Review under the existing documentation scope. A design note or a fixture marked FX cannot claim a task lease, implement a later-stage product route or close a phase. |

Identify a task's class from its reviewed ownership, paths, effects
and consumers. Labels, milestone placement, a split issue or a
caller-supplied class cannot supply this proof. Check the same
predecessor evidence when admitting implementation work, moving a
task to build-ready, assigning or claiming it, renewing or resuming
it, accepting the first or a retried candidate, and recording a
stage exit. A refusal does not prevent reading history, stopping
work or following the approved reconciliation route. No refactor,
framework or product effect may follow a denied transition.

| Synthetic fixture | Given, when and expected result |
| --- | --- |
| IT-0139-INVENTORY | For a 3A task, bind its exact frozen item, owner, disposition and reviewed inventory record. Use [0139 acceptance](../specs/0139-full-foundation-refactor.md#requirements-and-acceptance-cases) including its generated and installed callers and [exact disposition evidence](../specs/0139-full-foundation-refactor.md#exact-3a-disposition-evidence). Omit a caller, duplicate ownership, claim unproved archive-only coverage and omit a disposition in separate cases. Hold each affected task or exit with the precise missing or conflicting identity. Keep every inventory item in scope. Replace a retained component’s conformance proof with its design-only receipt, or omit the proof. Both hold 3A closure and leave the pointer unchanged. An independently proved archive design closes its own disposition only. Use FX-0076-PHASE-WORK-SCOPE for the binding check. A matching Phase 2 exit cannot fill an inventory gap. |
| FX-0139-ENTRY-3A | Give a complete proposed refactor task and a Phase 2 summary lacking one required signed receipt. Try a manual and automated ready move, claim and first submission. Hold them all. A genuine matching Phase 2 exit in the fixture only passes the predecessor check. |
| FX-0139-INITIAL-PHASE-2 | Give the protected gate a verified initial pointer state, a complete signed Phase 2 result with zero predecessor references and its approved evidence. Race two acceptances and lose the reply to the first commit. Exactly one initial transition commits. A retry reads that same result. Restore with a missing pointer after acceptance and prove that Phase 2 cannot initialise again. |
| FX-0139-ENTRY-3B | Give one owned framework row with an old signed 3A record for another inventory, then a current matching record. Try start, assignment, a restored claim and a new candidate. The stale record confers no lease or accepted candidate. In a separate branch keep the current eligible signed 3A receipt, protected pointer, policy, cited 0081 row and profile fixed. Substitute only the accepted refactor digest that the 3B task would consume. Existing 0076 admission holds ready, assignment, claim and candidate acceptance on the dependency mismatch. In the phase-only 0010 verifier case, use an otherwise matching test-signed 3B record whose `adopted-refactor` digest differs from 3A's `accepted-refactor` digest. Refuse the record and keep the protected pointer unchanged. The matching digest passes only this proof check. The exact current record admits only the scoped framework transition. With a matching 3A exit, try one applicable source-confirmed `specified` 0081 row with a complete 0076 design and its exact dependency receipt. Substitute a receipt for the same dependency under another profile. Hold ready, claim and acceptance until the matching receipt is restored. The matching dependency allows only that scoped task under the other admission checks. A separate unresolved applicable row outside its dependencies still holds the 3B exit. |
| FX-0139-ENTRY-PRODUCT | Try the first product wiring and the separate SDS row with Phase 2 accepted and partial 3B. Offer a starter demo, an omitted platform, an otherwise valid unrelated waiver and a Boot artefact swap. Hold all ready, claim and candidate transitions. In separate synthetic variants fix the current eligible signed 3B exit, its exact `boot-artefact-inventory` digest, the Boot artefact bytes and the product task's consuming platform and profile. Substitute only the selected cell and its evidence. Use a different qualified profile on the same platform, then a qualified cell on a different admitted platform, with identical synthetic artefact bytes in each case. Both substitutions hold ready, claim and candidate acceptance. For TASK-0106-BOOT-WIRING, fix a complete eligible signed 3B exit, the correct selected cell and its applicable 0113 BI-13 and 0114 BJ-21 proof. Replace only the consumed starter artefact with different bytes carrying the same name and version. The existing dependency check holds ready, claim, build, start and candidate acceptance before product work. The original matching bytes pass this dependency check only. Separately, refuse an otherwise matching test-signed 3C record with the wrong cell in `adopted-boot-artefacts` under the existing 0010 phase verifier and keep the protected pointer unchanged. The matching consumer cell passes only this dependency check. With matching complete 3A/3B evidence, apply the task's own phase and scope checks as well. |
| FX-0139-STALE-WORK | Admit an exact scoped fixture claim, then supersede its proof or change the adopted framework artefact. Race lease renewal, retry, resume, a copied task and final candidate acceptance. In a separate branch start with a verified row and fail its contract test against changed upstream input under 0081 UT-PB-03 and 0114 BJ-22. Offer the earlier test-signed 3B result with unchanged bytes as proof for the changed input. The affected row is `in_progress` and holds current qualification for that input. The earlier signature remains history for its original evidence. The row transition does not advance the protected phase pointer. No stale claim can start or complete protected work. Reconcile any recorded effect through its existing owner. |
| FX-0139-ENTRY-PHASE-4 | Give TASK-0031-SITE-FIXTURES pinned licensed inputs and accepted 3B without 3C closure. Hold ready, claim and candidate acceptance. Then give it matching 3C closure and exercise its other prerequisites. In a separate synthetic branch keep the exact current signed 3C closure, official fixture version and input digest fixed. Withhold only the corresponding licence evidence for the pinned XenForo fixture. The existing 0031 CO-13 prerequisite holds the affected fixture before it runs. Matching licence proof passes only that input check. Accept fresh 3B through the existing phase check while a 3C acceptance and Phase 4 attempt wait. After replacement the old closure and claims are historical. Refuse ready, claim, renew, resume and acceptance using them, even after restore. A fresh matching 3C closure can pass the predecessor check; it gives no wider authority. |
| FX-0139-ISOLATION | Compare an isolated synthetic Boot host to a task that wires a real downstream action and is labelled a framework fixture. Refuse the disguised product task before 3B exit. Try a locally signed FX receipt, a copied milestone comment and a historical gate result as live phase proof. None can close the stage. |

For BDD-0139-ORDER, keep the current signed 3B receipt
and the correct selected Boot cell. Withhold the admitted
`first-product-slice` owner format or its protected reader
in separate 3C runs. Give a candidate summary naming the
correct product and cells. Bind the summary's digest in a
synthetic 3C record and give the candidate a passing ordinary
gate result. Hold 3C acceptance and keep
the protected phase pointer unchanged. A candidate digest and
an ordinary result provide no product subject proof. In the
conditional control use the admitted protected reader and
exact observed 0106 result and review references. Only the
matching product subject check passes. Use the existing 0010
phase envelope and test keys. This adds no encoding or new
authority. QR-243 cannot supply this 3C proof.

The PB-B start-up scope uses the 0106 migration strict and warning
variants with their existing Wave 5 fixture. Reuse IT-0139-BOOT-COVERAGE
under [0139 start-up cells][boot-startup-cells] with BD-BB-03 and FI-BB-03.
A complete `specified` task cites the reviewed source, planned inputs,
planned cases and current predecessor. At qualification, join the reviewed
inputs, generated table digest and observed result for the exact row,
variant, candidate and target profile. A wrong source row holds task
entry. Both variants refuse a missing or altered table before any start
effect, even under the same release label. A mismatched or missing result
holds 3B acceptance and leaves the protected pointer unchanged.

[boot-startup-cells]: ../specs/0139-full-foundation-refactor.md

Join 0003's cutover refusal probes to IT-0139-BOOT-COVERAGE under
[0139 exact protected build cutover][boot-cutover]. Use the existing
0010 proof roster. Match the committed current route and selected target
to the exact 3B build evidence. Hold a wrong-route proof, an uncommitted
switch or mixed authority, and keep the phase pointer unchanged. Check
whether the policy floor moved before reusing any earlier gate route.

[boot-cutover]:
  ../specs/0139-full-foundation-refactor.md#exact-protected-build-cutover

Use synthetic site and framework fixture data in trusted runs as
the owning specifications require. An actual trusted-run result
on that data can support a live stage decision only when its
candidate, fixture, inventory or parity matrix, adopted artefacts,
tool, profile, tier, signer and current policy all match. A
synthetic authority record, fixture key or dry-run result has no
live authority. Keep the observations, signed predecessor result
and verified task transition as separate evidence.

The existing 0065 signing and 0010 gate contracts own the phase
evidence and verification. Bind the exact phase record subtype,
current stage and candidate identity, policy and accepted artefacts
to the authoritative transition reader before enabling progression.
No new purpose, caller key, waiver or unsigned substitute is
introduced by these case designs.

## Phase 3 to 5 exact and proposed trace links

Where an owning requirement uses a numbered contract instead of a
formal ID, use that section as the source and supply its stable
requirement and variant IDs under spec 0076 before admission.
The cases below extend existing fixtures. They do not replace the
other fixtures or distinct supported variants.

| Task and owning requirement | Layer and case | Given and when | Expected result | Evidence producer and remaining binding |
|---|---|---|---|---|
| TASK-0106-BOOT-WIRING, BB-01, BB-03, BB-04 | UT-BB-01, UT-BB-03, UT-BB-04 and PL-BB-01 | Start one explicit graph. Add a missing provider, ambiguous provider and cycle in separate runs | The declared graph starts. Each planted fault names its part and refuses startup | Boot startup report. Record exact profile and variants |
| TASK-0014-PROFILE-ACTIVATION, 0014 activation and delivery | AT-CF-OUTBOX and AT-CF-DELIVERY | Lose acknowledgement after swap. Deliver an older result after restart and attempt old-epoch replay | Current process and pointer reconcile once. Earlier evidence cannot acknowledge or activate a value | Settings service and role receipts. Bind activation and restart variant IDs |
| TASK-0107-FIRST-ERROR-SHAPE, 0107 problem details | Local API conformance and denied action | Submit bad input and a wrong-scope request, then fail the service dependency | Responses keep one declared shape and safe codes without private values | Local API and action fixtures. Bind stable requirement and test IDs |
| TASK-0112-FIRST-LOG-METRICS, BH-11 and BH-20 | UT-BH-10, FI-BH-03 and UT-BH-17 | Plant a private value in a logged error. Fill the log and metric bounds | The value is masked, no work waits for the log, and metric recording holds its bound | Log and measurement receipts |
| TASK-0032-FIRST-AUDIT, 0032 event shape and grants | AU-01, AU-02 and AU-03 | Record an admitted action with private input, then attempt to mutate the audit row and verify a tampered fixture | History appends safe metadata only. The app role cannot edit it and tampering is detected | Signed audit and role receipts. Bind exact action and failure IDs |
| TASK-0108-RLS-STARTER, [0108 scope](../specs/0108-behemoth-data-access.md#site-context-and-row-level-security-sutradhar-rule), BD-07 to BD-09 and BD-20 to BD-22 | IT-BD-01, IT-BD-02, IT-BD-03, IT-BD-06, UT-BD-14, UT-BD-15, PL-BD-03, PL-BD-04 and PL-BD-05 | Write site A, then check absent scope at commit or rollback before reset. Use the same physical connection to check absence after reset and then read with site B. Plant the wrong binding lifetime, run a query before the binding, and return a connection with an open transaction. Start as a role that can bypass RLS. Separately pass startup, then change only the same named synthetic app role’s bypass eligibility before the first ordinary checkout. Run the eligible local action on its exact site. In a separate isolated pre-site fixture compare setup and migrator access to their exact admitted records. Try site data with installation scope and migration history with setup authority. | The setter uses a bound value and transaction lifetime. No A rows cross the boundary. Refuse early access and the bypass role. At first borrow use SUT-971 and require no site binding, application SQL or protected result. The current restricted-role control returns only the bound site’s rows. Close the open transaction's connection. No site data leaves its site and only the authorised migrator reaches migration history. | Bind database and checkout identity, startup and current role checks, and the scoped 0020 result. Keep the 0108 receipt tied to the qualified Boot profile consumed in 3C and its platform applicability. Product installation and recovery keep their separate 0030/0031 cases. The BD-15 UT-BD-12 and IT-BD-06 numbered migration cases prove that schema and successful SQLx history cannot commit before policy validation.  When version and checksum match but legacy history ownership cannot be proved, refuse before history row access with SUT-971. Keep committed schema and history unchanged. See the installer SUT-833 branch in IT-0030-PLATFORM-INSTALL. |
| TASK-0110-FIRST-OUTBOX, BF-01 and BF-02 | UT-BF-01, FI-BF-01, IT-BF-01 and FI-BF-02 | Roll back a write, then deliver a committed row twice after crashing the first relay | Rollback publishes nothing. The committed change produces one logical consumer effect | Transaction, relay and inbox receipts |
| TASK-0019-DURABLE-PLATFORM, REQ-0019-DURABLE-EXECUTION and DES-0019-DURABLE-PLATFORM | IT-0019-SCHEDULER-RESTART, IT-0019-DELIVERY-DUPLICATE and 0110 UT-BF-23 | Stop the scheduler after it commits a fire. Reclaim the job and deliver its event again. In a separate run withhold database time across restart and cross due and expiry only on the workstation clock. Recover with verified database time before and at the old expiry. | One fire and effect remain. An old worker cannot commit. Preserve the original fire, attempt and deadline while time is unknown. Hold reclaim below expiry. At equality allow only one eligible fenced replacement after reconciliation. | Scheduler, job and execution receipts. Bind the first registered class fixture |
| TASK-0028-XF-REVIEW-EDIT, 0028 apply and undo and 0021 link receipts | IT-REVIEW-EFFECT | Lose the signed result after one native save. Replay approve and undo; then repeat after expiry | Exact effect reconciles once. Expired or ambiguous proof needs review without a blind retry | XenForo log and service receipt. Bind apply, undo and expiry identifiers |
| TASK-0031-WP-REVIEW-EDIT, 0031 selective undo and native save conformance | IT-REVIEW-EFFECT replay, WX-08 to WX-11, CO-12 and CO-13 with Wordfence and the admitted cache fixture | A human edits the anchor before apply. In a second case the reply is lost after a save | Conflict holds. The signed receipt proves only the exact saved effect and preserves later edits | WordPress journal, platform compatibility and footprint checks |
| TASK-0030-BACKUP-RESTORE, 0030 site register and protected history, 0030 Backup and restore, and 0031 Backups and restore | 0031 IP-30, 0102 IT-SS-07 and 0014 AT-CF-DELIVERY | Restore an older file and database pair after a site's removal and a settings epoch change. Run the separate IP-30 refusal cases below for the matching restore variant. | A retired site stays unavailable and a site missing its pairing key stays disconnected. An old settings approval or event cannot reactivate settings. Failed recovery or forced RLS proof holds isolated restore without a live switch. | Bind exact platform, restore scope, archive, reader, storage route and target. Keep the manifest, protected history, recovery and restored policy results and settings outcome distinct. |
| TASK-0032-RETENTION-ERASURE, 0032 erasure and restore | AU-08 and export race fixture | Erase a record after backup, then restore while a support file waits for delivery | The affected record is removed before restored data can be used. Stale prepared output is withheld | Restorer and export fence evidence. Bind the exact export case For transformed forecast views, the 0151 conservative-display fixture checks that exact source erasure locates and fences managed derivatives across cache, prepared export and restore. |
| TASK-0038-OWNED-STORAGE, 0038 verified upload and Creating resources, N-1476 and QR-202 | SC-05/07/08 for move, SC-08/09/11 and [FX-0038-W5-PRIVATE-BUCKET-EXACT-CLAIM](feature-wave-5.md#acceptance-design-for-n-1454-to-n-1477) for private creation | In a synthetic creation fixture give the exact approved quote and one bucket in the quoted account and region with the approved privacy, encryption and lifecycle controls. Separately race marker claims, lose the create reply and observe a same-name bucket with unknown creation, and vary the owner or current privacy proof. After verified activation, make the bucket public in a separate case. Separately exercise the owner-approved existing-bucket connection for its exact site, region, prefix and rights, and offer that approval to Create. In the move cases retain corrupt upload and revoked scope. | For Create, only the exact approved private creation and current sole claim can activate. A lost reply or missing proof keeps the bucket unusable. Reconcile only the original intent without recreating or deleting by name. A proved public bucket stops uploads with SUT-829. Existing-bucket connection grants no creation permission. Keep verified local data on a failed move and refuse unapproved cleanup. | Bind the exact applicable provider and route, quote, approval, account, bucket, region, current safeguards and original create and claim proofs. Record the connection scope or encrypted move receipt and checkpoint separately. Synthetic outcomes give no live provider or release admission. |
| TASK-0078-CORE-RESOURCE-PROOF, MB-02, MB-04 and MB-08 | UT-MB-03, PT-MB-01, QM-MB-01 and QM-MB-03 with measured profile. Join 0033 SR03, 0078 FI-MB and the 0019 expiry fence under QR-081. | Occupy the admission pool while two sites wait and a never-wait action starts | The ceiling holds, the protected action runs, and both eligible sites receive their admitted share. In the separate crash case, neither an expired lease nor another attempt receipt releases the original charge. Restore keeps it charged once, SUT-4610 explains the wait and the blocked candidate counts no attempt. | Resource measurements and scheduler model. Bind each supported profile |

For the same erasure boundary, join BDD-0053-ERASURE-BLOCK to
0032 AU-08 and 0077 BDD-0077-ROLLBACK-FAILURE. Supply authentic
pre-erasure dataset, trial and prior candidate records. Keep current
erasure history outside the restored backup. Hold affected artefact
release and checkpoint reuse even where the old records verify.
Current rollback verification also needs proof that its exact prior
set is eligible after applying erasure. Keep the original history
and derive no successful result or rollback from those old receipts.

Join 0030 test 25 with AU-08 and IT-SS-07 for the restore publication
race. Advance verified retirement or erasure history after isolated
verification and before the live claim, in separate cases. Repeat
with an earlier pending audit and newer erasure history. Recheck the
0030 history fence at publication. Old staged state stays unavailable
after restart. Check the original
archive and current live state, the owner restore hold and the
fresh-scope approval boundary. AU-12 checks that support uses only
the current eligible settings projection while the restore is held.
AU-16 retains the separate SUT-848 audit refusal.

Use [0028 CAP-039][exact-block-reset] and `acceptance/cap_039_reset.rs`
for local connection reset. Bind the original 0020 plan and idempotency
key to a site, canonical pair and exact active block occurrence B1.
Revoke permission before an unperformed reset and check that B1 stays
active. Separately commit the reset, lose the reply and create a fresh
review rejection B2 with the same site, pair, kind and fingerprints.
After checking current action and result rights, the original retry
returns only the permitted B1 history, with B2 still active. When those
rights are revoked, the replay discloses no receipt or reason. Give an
unperformed B1 plan to the action after a separate authorised reset
and new rejection make B2 active. SUT-803 asks for a fresh preview
and leaves B2 in place. Count one committed reset and
one reset event in the committed branch. Assert zero native sends or
writes in the reset and replay cases. An independently authorised
history read keeps its own rights and never applies either reset.

For the separate 0028 trash Restore in `acceptance/cap_trash_restore.rs`,
commit the B1 restore and lose its reply. Let recalculation return
the card, then reject again with the same pair and fingerprints.
Bind B2's different decision version, block occurrence and 0053
label. Replay B1's exact plan and key. Only the authorised earlier
result can return, with neither B2 record withdrawn nor a second
recalculation, withdrawal or native edit. A stale unperformed plan
holds under SUT-803. Retention and erasure use their existing rules.

[exact-block-reset]: ../specs/0028-staff-suggestions.md#reject-and-reset

For TASK-0030-BACKUP-RESTORE, bind 0031 IP-30 to each applicable
admitted restore variant. Keep the selected backup's verified bytes,
authority and live state fixed. In separate branches, remove its
backup recovery material or supply a recovery key below 0118's
signed minimum epoch. Name the failed recovery check and keep the
backup sealed. In a third branch, use eligible recovery material
and let decryption complete in the isolated target. Withhold its
forced row-level security proof. Name the failed database check.
In all three, retain the live pointer and later work. Issue no
restore success, switch or rollback receipt. Compare an eligible
isolated restore with current keys, history, restored roles and
forced row-level security. Its result proves only that restore.

For IP-30's current history format case, join 0030 site-register
item 5 and 0032 AU-08. Keep archive, key epoch and package release
eligibility current. Supply the older readable checkpoint after
a verified advance to an unsupported current history format.
Hold before staged data lookup or switch. Compare the exact
admitted reader control applying current fences in isolation.
Keep the outside target's waiting result independent of restore.
In a committed-fence control, also reconcile one still-eligible
native receipt for a different effect on the same site under
IP-31 and 0021's exact read. The receipt adds no native write.
The restored view still withholds erased data and the outside
target remains waiting. Qualify the earlier pending-request
branch under [QR-250][restore-erasure]. Bind its authority and exact
ordering against the 0030 publication claim. Apply the existing
hold for unproved current history and the committed fence check.

[restore-erasure]: ../registers/questions.md

For the IP-30 pre-switch refusal, use an already admitted synthetic
switch fixture. Keep the verified isolated result, its separate
switch approval and exact current lease, and all applicable [protected
history][restore-audit-history] and [minimum key epoch][restore-audit-key-floor]
proof fixed. Fail only the [required audit write][restore-audit-rule]
before activation. Restore reports SUT-848. Keep the reader, manifest,
authority and applicable storage proof current. The live pointer,
current records and later work stay in place. Existing holds still
apply. Keep the isolated result separate and its target inactive. No
live restore success, switch or rollback receipt follows. Record the
exact plan and lease, isolated result, current policy proof, audit
failure and before and after pointer and availability. Check the
[owner recovery wording][restore-audit-owner-help]. It identifies
the failed audit even with a passed isolated test, keeps the current
work and holds, and directs the owner to the original restore result.
Reconcile the original effect after interruption. A new attempt
requires current authority and lease. The fixture confers no
production admission.

[restore-audit-history]:
  ../specs/0030-install-upgrade-removal.md#the-site-register
[restore-audit-key-floor]:
  ../specs/0118-end-to-end-encryption-gate.md
[restore-audit-rule]:
  ../specs/0031-shared-platform-installation.md#diagnostics-and-audit
[restore-audit-owner-help]: ../../manual/operations.md#restore-a-backup

For IP-30's portable activation refusal, use 0030 test 22 and the
exact admitted QR-202 mode and native profile. Offer a normal
install's otherwise valid proof for matching release and archive
bytes while the portable proof is absent. Hold the affected switch,
writes and launch. Keep normal start-up effects unavailable and
preserve earlier verified results. A portable result needs its own
current release, restore, volume and at-use proof.

For the separate Windows IP-30 branch, bind the exact admitted
native profile, volume and ledger to the isolated result. Use
[0030 backup and restore][restore-native-owner] for publication
and 0038 SC-05 and N-1499 for the storage identity boundary. After
isolated verification and the pending switch audit, substitute
another writable volume at the same drive letter and label before
publication. Hold the switch with the destination verification
reason. Keep later work, the live pointer and the isolated result
for the original volume. Assert no write, relink, cleanup or
activation on the substitute and no success or rollback receipt.
With the original eligible volume and all other publication checks
current, the synthetic control records one switch to that volume.
In the existing IP-30 after-check variant, first prove the exact
isolated backing root A and pending audit in the expected volume.
Keep A's check result. Present a different root B on the same
volume with the same path and visible ledger markers at the effect
hand-off. Hold the switch and the live pointer. Keep A's isolated
result and the pending audit separate from success. Verify no new
receipt, write, relink or cleanup on B. The control keeps the exact
checked root through the effect. Bind owner-approved plan, current
lease, exact expected and observed at-use root, native volume,
ledger and live pointer disposition. A move test alone gives no
restore proof.

Exercise the IP-30 portable lost-reply branch after a verified
switch to A. Keep the pointer and exact effect receipt. Offer B
with matching path, label and bytes at restart. Record the original
receipt separately from current destination eligibility. Bind the
current volume and root under the admitted profile. Hold restored
data access on B without further write, relink, cleanup, launch or
switch. When exact A returns, current checks permit only its own
eligible access. Both branches keep the total switch count at one.

[restore-native-owner]:
  ../specs/0030-install-upgrade-removal.md#backup-and-restore

For TASK-0030-BACKUP-RESTORE, join [0031 IP-31][restore-purpose] to
IT-SS-07, [0118's current pairing and key boundary][restore-pair-proof]
and the existing BDD-0021-PAIRED-SITE control. After the isolated site
restore on a synthetic new machine, retain the historical fingerprint
but leave the hardware-bound key unavailable. Give a genuine current
release-root recovery receipt to the site recovery check. The site
remains disconnected. Replay of a pending effect sends no protected
request, changes no native content and reactivates no old key or grant.
The release proof remains usable only for its owning package checks.
Separately make a fresh pairing for the exact restored site using its
current approved site and installation identities, both approvals and
fresh possession of the signing and encryption keys. Verify that link
under its own current scope. Withhold only current release-root proof
and offer the authentic pairing receipt to the package check. Refuse
before launch or native install effect. Record source purpose and scope,
trust generation, possession result and scoped refusal at each verifier.
Run no live recovery and admit no new trust route.

For IP-31's same-installation branch, join 0021 signing tests 3
and 10 and the exact 0118 rotation evidence. Reject a retained
reply under the revoked key. A fresh `ReadLinkEffect` under the
current pair binds the read message and the original effect
separately. Reconcile only the verified native receipt. Missing
native proof leaves the restored action unknown, with no replay,
new native receipt or current-link-status credit.

[restore-purpose]: ../specs/0031-shared-platform-installation.md
[restore-pair-proof]: ../specs/0118-end-to-end-encryption-gate.md

Bind desktop and manual evidence to the exact supported profile.
AT-LOCAL-TRANSPORT and IT-0118-LOCAL cover Linux and Windows identity
and replay. AT-VL144 covers gauge limits and "not measured" on an
admitted machine. IP-06 holds an unproved platform entirely.
MAN-PLATFORM binds each native site route, IP-15, Windows first
trust and Ubuntu and Debian IT-0030-VERIFIED-STAGE handoff refusals.
MAN-OFFLINE tests packaged help and recovery without the app.

## Phase 3 service and desktop scenario designs

Use synthetic fixture data and identities on exact QR-100 admitted
x86-64 Ubuntu 26.04 LTS, Debian 13 and Windows 11 profiles. Hold
missing platform or client profiles. Task IDs and dependencies
come from the [implementation catalogue](implementation-catalogue.md).

### TASK-0112-SERVICE-HEALTH

Proposed case `BDD-0112-SERVICE-HEALTH-BOUNDARY`.

**Owner and checks.** Health under
[0112](../specs/0112-behemoth-actuator-and-observability.md), BH-03 and
BH-05/06/21/34, with UT-BH-02, UT-BH-04 and IT-BH-02.
[0106](../specs/0106-behemoth-core-container-and-config.md) Lifecycle owns
the state file and instance identity.
[0030](../specs/0030-install-upgrade-removal.md) Engine supervisor and
QR-086 own any recovery admission.

- **Given** one verified ready instance, its advancing monotonic
  heartbeat, a healthy critical database indicator and a protected
  fixture value in indicator detail.
- **When** the database becomes unavailable while the instance keeps
  advancing, then present a stale sample for that same instance and,
  separately, missing, corrupt and superseded instance samples.
  Try draining, mixed indicator ordering and timeout SUT-1202.
  Read detail with and without actuator.health.detail.
- **Then** the outage changes readiness and leaves liveness and the
  instance intact. A current stale sample is distinguishable from
  unavailable evidence. Missing, corrupt and superseded evidence
  cannot prove a stuck current instance or request a restart. No
  displayed public detail contains the protected value. Draining
  stays live and unready.

**Receipt and remaining binding.** Pin the observed instance and
generation, samples, runner clock, critical indicator profile and
redacted outcomes. The 0112/0030 owners bind the joined stale and
unavailable cases to stable variant and test IDs, with QR-086 timing
held until qualified.

### TASK-0017-FIRST-COMPONENTS

Proposed case `BDD-0017-ACCESSIBLE-FIRST-COMPONENTS`.

**Owner and checks.** [0017](../specs/0017-visual-layer.md) Accessibility
and How it is tested, including AT-VL07 for shared keyboard and
accessibility checks. The [0020](../specs/0020-screens-actions-sutra-mcp.md)
Accessibility contract supplies owner-app focus and error behaviour.

- **Given** the first Angular catalogue fixture with a labelled input,
  invalid entry, progress text and action. Run the supported window and
  both themes and exact admitted profile. Verify keyboard-only focus,
  contrast and at least 24 × 24 target size, plus screen-reader passes.
- **When** the owner navigates by keyboard, submits invalid input,
  follows the error link and completes the eligible action. Repeat with
  a control missing an accessible label and with a missing translation.
- **Then** visible focus and navigation reach each control. The error
  names the field, links back to it and progress has a text state.
  Control name, error and state are announced without relying on colour.
  The accessibility and distinct 0017/0020 translation checks refuse
  the respective planted cases.

**Receipt and remaining binding.** Record the actual Angular catalogue
build, component and translation fixture digests, OS, width and theme,
automated outcomes and the independent keyboard and screen-reader
record. The owner must bind stable requirement, component test and
variant IDs for this first catalogue slice. AT-VL07 alone gives no
catalogue approval.

### TASK-0030-FIRST-WINDOW

Proposed case `BDD-0030-VERIFIED-FIRST-WINDOW`.

**Owner and checks.** [0030](../specs/0030-install-upgrade-removal.md)
Preflight, Installation, Screens, Windows first trust check and tests
1 and 4. [0020](../specs/0020-screens-actions-sutra-mcp.md) Desktop app's
shell and AT-LOCAL-TRANSPORT own the local client boundary.
[0118](../specs/0118-end-to-end-encryption-gate.md) IT-0118-LOCAL covers
protected local handshake refusals.

- **Given** a clean owner profile, verified fixture artefacts for its
  operating system and an admitted local service fixture.
- **When** the owner opens the first window. Repeat the preflight
  with changed installer digest or verifier pin on Windows, separately
  with modified bundled payload, native component or WebView, then
  repeat eligible Tauri/Angular launch without local transport.
- **Then** the verified launch reaches safe setup. Modified bytes are
  refused before starting the affected part. Transport failure shows a
  safe unavailable state. SUT-830 names the failed preflight and
  SUT-832 the payload/signature mismatch. Independent QR-202 first
  trust applies before Windows installer launch.

**Receipt and remaining binding.** Record the manifest and payload
digests, verification result and launch or refusal observation. Pin
the host, UI build, native runtime and local door profile. Bind stable setup
and unavailable-transport IDs in
0030. QR-202 holds unproved Windows first trust.

### TASK-0020-OWNER-DOOR

Proposed case `BDD-0020-OWNER-TRANSPORT-BOUNDARY`.

**Owner and checks.** [0020](../specs/0020-screens-actions-sutra-mcp.md)
The runner and Desktop app's shell, AT-LOCAL-TRANSPORT.
[0118](../specs/0118-end-to-end-encryption-gate.md) IT-0118-LOCAL,
[0109](../specs/0109-behemoth-security-and-identity.md) BE-22 and
[0112](../specs/0112-behemoth-actuator-and-observability.md) BH-01
supply protected peer and management boundary checks.

- **Given** an enrolled owner client and one typed current-site
  permission read under REQ-0118-LOCAL and IT-0118-LOCAL. Prepare an
  owner-only Unix socket on each Linux profile
  and a protected named pipe on Windows.
- **When** the client calls the action for its current site, then try
  wrong OS account, installation or site, unenrolled same-owner
  peer, wrong origin/window/launch, revoked token and replay.
  Check required peer mutual TLS and deny plaintext protected MCP. Inventory
  the candidate's management listeners.
- **Then** the admitted request returns its typed scoped result. The
  other requests have safe refusals and no protected read or mutation.
  The product exposes no management network listener.

**Receipt and remaining binding.** Bind expected and observed call
effects, action schema and policy digest, redacted peer and site scope,
transport and listener inventory. Link
REQ-0020-FIRST-ACTION-SCOPE and DES-0020-FIRST-ACTION-BINDING
to the exact registered ActionSpec, Cedar action/resource and
site projection. On each admitted OS profile keep the peer
and local TLS checks eligible. Give an explicit Cedar denial
alongside an otherwise applicable allow, under BE-13. Compare
SUT-801 and zero protected reads with the current allow
control's typed result. Separately disable the action in the
admitted action profile and verify the existing safe refusal.
Transport success cannot substitute for either result.
Require the owning product action's exact identity and
evidence before using this test for the first product slice.

**Joined role-status read.** Use 0014 AT-CF-DELIVERY,
AT-LOCAL-TRANSPORT and IT-0118-LOCAL for
`GET /api/v1/settings/roles` on each applicable admitted desktop profile.
Lose its reply and event stream with a different desired position and
an effective snapshot from the old process. Reconnect with current
rights before the replacement process proves its effective snapshot.
Return the desired position separately and show the effective state as
unverified until a fresh eligible read verifies it. In a separate run,
keep peer trust fixed and revoke the action grant after the reply is
prepared and before release. Prove the 0107 safe refusal shape and no
role, key, pending restart or value fields. On the next authoritative
denial clear the denied fields and never restore them from the old
request or replay. Bind the role process, request, site and redacted
Cedar decision. No case changes settings or restarts a role.

### TASK-0030-SUPERVISOR

Proposed case `BDD-0030-SUPERVISOR-RECOVERY-FENCE`.

**Owner and checks.** [0030](../specs/0030-install-upgrade-removal.md)
Engine supervisor and its lifecycle fault fixtures. Lifecycle and
BB-27/IT-BB-01 in [0106](../specs/0106-behemoth-core-container-and-config.md)
own draining. BH-05 owns heartbeat evidence.
[0019](../specs/0019-events-jobs-recovery.md) REQ-0019-DURABLE-EXECUTION,
DES-0019-DURABLE-PLATFORM and IT-0019-SCHEDULER-RESTART own job replay.

- **Given** one verified normal installation with a running service
  and a synthetic durable job holding a committed checkpoint.
- **When** the owner closes the window, the service later exits, and
  Quit races a pending recovery. Check exact process handle, principal,
  generation and fresh monotonic baseline at stop and start. Try
  reused ID or exhausted budget, and portable close separately.
- **Then** ordinary window close keeps the verified service running.
  Quit wins over recovery and stops the owned parts. A reused ID
  confers no control. The portable service stops on close. In the
  separately admitted recovery case only, one current successor can
  reconcile the checkpoint. UT-0019-FENCE-COMMIT expects SUT-233 for
  stale commit. Unrelated processes stay untouched. Portable mode
  leaves no startup registration.

**Receipt and remaining binding.** Record the ordered instance, exit,
drain, stop and checkpoint observations, protected job identity and
effects. Bind normal, portable, crash and Quit-race variants to stable
0030 IDs. QR-086 still owns stop budgets and restart admission.

Source inspection gives a locator. Expected baseline failure has a
reason. Record the actual baseline refusal and independently verified
candidate result separately. Absent, unrun or blocked cases give no pass.

Each future receipt binds exact tested tree and artefact digests,
profile, pins, fixture digest, test and task IDs, runner, observed
result and independent review. Apply 0003 signed evidence and the
0076 reverse trace through the reconciled Forgejo task and variant.

### TASK-0106-FIRST-SLICE-DEMO

**Scenario.** Proposed BDD-0106-FIRST-SLICE-DEMO, owner window to
recovered job. Use 0106 BB-03, BB-07 and BB-14–BB-16, and the
existing FI-BB-01, FI-BB-03 and N-1741 startup fixture. Use
[0019](../specs/0019-events-jobs-recovery.md) REQ-0019-DURABLE-EXECUTION,
DES-0019-DURABLE-PLATFORM and IT-0019-SCHEDULER-RESTART.

- **Given** a clean QR-100 exact admitted x86-64 OS profile, Tauri
  and Angular owner window, protected local door and forced database
  site isolation.
- **When** the owner starts the first service and runs a registered
  synthetic job. Crash a worker after a checkpoint and resume. Try
  a missing provider and bad settings in separate starts. Close the
  normal window while healthy work is pending.
- **Required startup part.** In FI-BB-03, fail a required
  synthetic start hook after an admitted local listener and
  provisional file exist, before readiness. Name SUT-935 as the
  safe startup cause and exit non-zero. The first owner window
  sees no ready state and no job is admitted. Verify that the
  listener and provisional file are gone. Keep unrelated
  resources untouched. Reuse N-1741 to stop a separate start
  with required work pending. Deliver late success for that
  stopped instance after a replacement. It supplies no readiness
  or result. In the FI-BB-01 control an optional background part
  fails after readiness, degrades only its health and leaves the
  verified service running.
- **Stored readiness sample.** Reuse FI-BB-03 and BH-05 with
  `BDD-0030-SUPERVISOR-RECOVERY-FENCE` on the same admitted native
  controller. Give replacement B its own advancing heartbeat and
  one required hook with a provisional listener, file and idle
  native fixture child. Present A's genuine earlier ready sample
  while B is still starting. Do not deliver a callback. One eligible
  ordinary job stays queued with no lease, worker start or effect.
  Then fail B's hook. Verify SUT-935, non-zero exit and teardown of
  only B's exact owned child, listener and file. A similar unrelated
  process stays running. In a separate current-ready control apply
  the ordinary 0019 checks and observe one logical job admission.
- **Then** the healthy journey records one logical job result. Each
  invalid start gives its safe reason, no readiness or surviving
  listener, file or process under BB-16. Stale job lease cannot
  publish. Ordinary window close keeps the service. The separate
  explicit Quit path drains and stops it.

**Evidence.** Bind the owner journey, protected transport, failed
start, checkpoint, recovery and window observations to the declared
platform and first class. Record the required-part failure before
readiness, listener and file cleanup, the stopped generation's
late callback refusal and the post-ready health-only outcome.
Record stored and current instance identities, heartbeat, native
controller and child exit, and first job claim and effect counts
separately. A prior ready sample supplies no current journey proof.
Link every Phase 3 dependency's own receipt
and required independent reviews. REQ-0106-FIRST-SLICE and
DES-0106-FIRST-SLICE own this product result. Exercise
VAR-0106-FIRST-SLICE-UBUNTU, VAR-0106-FIRST-SLICE-DEBIAN and
VAR-0106-FIRST-SLICE-WINDOWS on their required protected cells
under QR-100. The 0010 profile resolves its exact result
through `first-product-slice`.

In the 0139 refusal branch, keep the current signed 3B exit and
the correct adopted Boot cell. Substitute the genuine product
journey of another Linux profile, even with equal application
bytes and the same socket kind. It cannot fill the expected
product result cell. Holds leave the phase pointer unchanged.
An exact result passes only its own proof role.

In the same-cell BDD-0139-ORDER branch keep current 3A and 3B,
exact adopted Boot evidence, the candidate and applicable 3C
fixture profile. Supply an eligible 0014 AT-CF-DELIVERY result
and the required independent review. Offer the settings result
for the missing selected action observation under
REQ-0020-FIRST-ACTION-SCOPE and DES-0020-FIRST-ACTION-BINDING.
Preserve settings and review credit, identify the unproved action
and hold the product subject and accepted pointer. The control
adds the matching observation from the scoped registered action
and its current Cedar result. The proof read gives no runtime
permission and executes no action. Exercise the reader only within
its QR-251 qualification and exact authorised synthetic scope.

## Paired change-feed recovery

Use [0021 outbox tests 23, 24 and 30][saved-outbox-format] and
[0031 Events and pause][saved-outbox-platform] on both admitted
synthetic adapters. Commit an unsupported saved event 501 followed
by an otherwise eligible 502. Keep the last proved acknowledgement
at 500 and authenticate the current 2.0 reply. Its body fails the
admitted typed schema. Report SUT-308 with a safe reason and incomplete
coverage. No event from that reply is published. The acknowledged
cursor and native prune boundary stay at 500 after the missing-ID
timeout, restart and pruning. Retain both original records. Their
presence supplies no permission to read an unsupported body.
Verify the safe unsupported-format reason and incomplete coverage
in the authorised owner’s recovery view. An eligible connection
result alone keeps the change-feed hold in place. Verify the
[manual recovery steps][saved-change-help]. The existing late or
absent ID case is separate. On an admitted compatible reader,
reconcile 501 and 502 once with current guest
and deletion checks. Commit the
accepted events and then advance the acknowledgement. Bind native
row IDs, envelope and body disposition, persisted cursor values,
event receipts and retained counts. Use no legacy transport.

[saved-outbox-format]:
  ../specs/0021-forum-contracts-ingestion.md#how-it-is-tested
[saved-outbox-platform]: ../specs/0031-shared-platform-installation.md
[saved-change-help]:
  ../../manual/troubleshooting.md#recover-a-saved-site-change

## Phase 4 signed site contract and native edit pilot

Follow XenForo then WordPress under the [Phase 4
roadmap](../roadmap.md#phase-4-xenforo-then-wordpress).
The scenario headings are proposed BDD labels for these designs,
pending owning-spec binding under spec 0076.

### Tasks and current owner anchors

| Task | Owning contract and existing test anchors |
|---|---|
| `TASK-0021-SIGNED-SITE-CONTRACT` | 0021 **Signing a message**, **Messages**, **Link effect receipts** and signing tests 1–10. 0118 `REQ-0118-LINK` / `IT-0118-LINK`, `REQ-0118-HOST` / `IT-0118-HOST` and `REQ-0118-KEY` / `FI-0118-KEY`. 0031 IP-12, IP-13 and CO-06. |
| `TASK-0028-XF-REVIEW-EDIT` | 0028 `CAP-035`, `CAP-038` and `IT-REVIEW-EFFECT`. 0021 native edit, receipt and footprint contracts. 0031 IP-15, IP-16 and IP-17. 0026 SH52–SH54 for selective undo. |
| `TASK-0031-WP-REVIEW-EDIT` | 0031 **Conditional save**, **Selective undo**, IP-15–IP-16, IP-18–IP-21, WX-07–WX-13 and CO-05/06/12/13. 0028 `IT-REVIEW-EFFECT` for shared service effects. Use each current native permission and protected grant. |

Record missing stable owner and executable case bindings under 0076.

### Fixture

Use disposable Dell site fixtures and synthetic records only. Use
XenForo 2.3 and each exact admitted WordPress major, PHP and host
profile. The WordPress branch includes the admitted Wordfence version
and an admitted cache profile with its real runtime. Record the
image, configuration, plug-in and theme versions, licence and tool
inputs for each tuple. Hold an unavailable or unqualified tuple.
Use two fixture sites with equal local IDs and distinct keys, each
with a public source `S`, a public target `T`, one
proposed anchor and a different neighbouring link. Keep exact source
and target revisions and hashes, an invented moderator, an
unprivileged actor and a controllable time and reply boundary.

Use synthetic credentials and sealed 0118 fixture envelopes at the
exact fixed POST with empty query in contract 2.0. Reject downgrade
and header or sealed-projection disagreement. The older 0021 signing
vectors cover primitives and canonicalisation only. Prove possession
of both signing and encryption keys with fresh challenges. Capture safe native
before and
after state, receipts, read-backs and service execution and outbox rows.

### Signed site contract

#### BDD-0021-PAIRED-SITE

**Given** one eligible site, a trusted endpoint and fresh pairing
challenges with current admin and local owner approval on both ends.
**When** both sides confirm the same installation, site, protocol,
generation and permissions, compare signing and encryption fingerprints
outside the network transcript. Lose the pairing reply and reconcile
the original approval before one signed fixture read.
**Then** verify one credential and the same pairing receipt. Both
ends verify the protected request and its bound reply. A rebound,
expired or second approval creates no extra pairing or grant.

#### BDD-0021-REFUSED-ENVELOPE

**Given** a current paired scope and one accepted fixture read.
**When** the fixture varies one binding at a time: other site, wrong
purpose or key, revoked generation, used nonce across restart,
299-second, 300-second and 301-second ages, future time, altered
signed bytes, or reply belonging to a different attempt.
**Then** admit only the unchanged eligible read and fresh messages
within the allowed age, including exactly 300 seconds. On each receiver
use 0021 signing tests 3–4 to check the distinct already-seen nonce
at equality after restart and normal pruning. Keep the nonce through
that second and refuse the repeat before opening the sealed body. At
301 seconds the age check refuses even after purge. Record no second
operation. For a bad request the add-on replies with fixed empty
HTTP 400. For a bad reply the PC refuses locally and sends nothing.
Other refused inputs cross no protected boundary. Keep exact scrubbed
local evidence. For a request before authentication use the
fixed empty HTTP 400. After authentication require a sealed HTTP
200 outcome. A fresh signed transport attempt can read the original
logical effect without writing. Reused nonce or changed intent
under the held logical effect refuses. Replies bind the attempt.

For the [0021 signing test 3][paired-replay-restore] restore case,
keep the exact clock, paired authority and eligible signed synthetic
fixture. Restore a replay store without current continuity proof.
The add-on refuses before opening with fixed empty HTTP 400. If the PC
is the receiver, it refuses locally before opening and emits no reply.
Check unchanged cursor and effect counts on both adapters. A verified
current store preserves the rejection of the old accepted message and
allows a separate otherwise eligible fresh message. Independently
exercise the approved fresh pairing under 0118 with its own fresh
eligible message. The old pairing stays ineligible on that route.

[paired-replay-restore]:
  ../specs/0021-forum-contracts-ingestion.md#how-it-is-tested

In the [0021 test 32][typed-ack-compatibility] branch of IT-0118-LINK,
keep the two admitted adapter fixtures on the exact 2.0 route. Give
each a current eligible cursor advance for committed events. In a
fresh otherwise valid `AcknowledgeEvents` request add a field allowed
only as ignored input in the separate local 1.3 note reader. The live
typed request returns the existing sealed HTTP 200 refusal. Check
transport reply binding, no event acknowledgement, unchanged
cursor and retention of records above it when the normal prune
check runs after those records reach its age limit. A separate fresh
request without that field uses the normal acknowledgement checks.

[typed-ack-compatibility]:
  ../specs/0021-forum-contracts-ingestion.md#how-it-is-tested

### XenForo attachment-only visibility

Join [0021 guest conformance test 14][xf-attachment-visibility] to
[0022 extras test 9][xf-attachment-extras]. On the admitted XenForo
fixture, read one synthetic attachment name and buffer a repeated
read. Revoke only the native attachment permission while the parent
post stays eligible with the same body hash and change counter. Check
no new name read under denial. Withhold the late result and block use
of the cached name, indexed extra and derived combination. Preserve
the eligible body and unrelated eligible results. Observe the existing
cleanup and any rebuild based only on allowed extras. Use a fresh
parent and attachment permission check for the matching control.
No attachment bytes or new native probe is used.

[xf-attachment-visibility]:
  ../specs/0021-forum-contracts-ingestion.md#checking-against-xenforo-itself
[xf-attachment-extras]: ../specs/0022-reading-indexing.md#extras

### XenForo native review, apply and undo

#### BDD-0028-XF-ONE-EDIT

**Given** a current moderator, one approved same-site suggestion,
current source and destination revisions, exact canonical target,
guest visibility, language authority and recorded source post hash.
**When** duplicate approval arrives and the worker loses the reply
after one valid native save. Replay the event and reconcile the
original effect through `ReadLinkEffect`.
**Then** observe one decision, job, native edit and committed receipt.
The editor changes only the approved anchor, with no bump or alert and
with the old version in edit history. The verified service state,
execution receipt and outbox event commit together. Compare a save
that changes the approved bytes: SUT-416 rolls back without success.

#### BDD-0028-XF-CURRENT-AUTHORITY

**Given** a valid approval and delayed work. **When** role, pause,
source or target privacy changes before native save, or a committed
receipt becomes expired or ambiguous. **Then** current checks refuse
a new effect. Unknown history holds. If a valid receipt survives
role loss or a human edit after save, the current service proves
history only. Distinguish proved pre-save refusal from unknown
commit. Changed canonical target needs new approval. No alternate
effect ID or blind retry edits the post.

#### BDD-0028-XF-SELECTIVE-UNDO

**Given** the recorded applied link and a later moderator edit around
it. **When** an authorised undo loses the reply after exact removal
and the original request is reconciled. **Then** only the inserted tags
are removed. Later bytes and the neighbouring link survive, with one
terminal result and suppression. An already absent link gives SUT-418.
An edited anchor or ambiguous match gives SUT-417 and no write.
Leave quoted copies untouched under SH54. Try duplicate or changed
target and SUT-424 save rewrite refusal. Keep absence, conflict
and verified removal and suppression distinct.

### Approval and re-score replacement

Use [0028 approval race and recalculation tests][rescore-approval-race]
with [0020's stale-preview response][rescore-stale-preview] and the
existing [signed native intent checks][rescore-native-intent]. Keep
the original suggestion revision, source text and both eligible
destinations fixed while positioning
the pending approval and replacement at the shared state transition.
Commit approval first in one run. Keep one decision, approval event
and apply job for the exact approved destination. The losing re-score
cannot retarget it. Restart with a changed scoring epoch and the
same current native and approval proof. Keep the original apply job.
The ordinary native route checks any later send.
Commit replacement first in the other run. Refuse the earlier plan
with the current suggestion state and SUT-803 on the 0020 action.
The proposed replacement needs its own approval. The losing plan
produces no approval event, job or native send. Replay preserves the
same winner. Compare exact suggestion versions, the decision and
outbox records, job target and unchanged native intent scope.

[rescore-approval-race]: ../specs/0028-staff-suggestions.md#how-it-is-tested
[rescore-stale-preview]: ../specs/0020-screens-actions-sutra-mcp.md#failures
[rescore-native-intent]:
  ../specs/0021-forum-contracts-ingestion.md#link-effect-receipts

### WordPress native review, apply and undo

#### BDD-0031-WP-CONDITIONAL-APPLY

**Given** a qualifying native save protocol, current WordPress object
permission, public password-free source and target, and matching
approved plan and Prahari verdict. **When** one case saves unchanged,
and separate cases race a human save, lose permission, hide the
target, or transform the proposed bytes at the save edge.
**Then** only the eligible case records one exact edit and native
receipt together. Other cases preserve the human result. A missing
conditional protocol keeps writes off. `wp_update_post` with only a
plug-in lock is insufficient. Check native CSRF and grant separately
from stale or missing Prahari proof. Refuse unknown neighbouring
blocks. Reopen and save in every admitted editor tuple. Native
hooks and revisions stay enabled.

#### BDD-0031-WP-LOST-REPLY-AND-UNDO

**Given** that native protocol and a scoped journal. **When** apply
commits but its reply is lost, a human makes a later unrelated edit,
then an authorised selective undo also loses its reply. **Then** the
exact receipts reconcile one apply and one inverse edit. The later
human text and unrelated link remain. Repeat with a stale role or
pause, edited anchor, overlap, lost journal and expired receipt.
Record proved pre-save refusal separately from unknown receipt. Test
pause across restore and restart. Use the WordPress own retention,
expiry and no-op results. No whole-revision restore.

For the [0028 IT-REVIEW-EFFECT][undo-reinserted-check] reinstated-link
branch, use the [0021 original effect receipt][undo-original-receipt]
and the admitted 0031 WordPress selective-undo contract where
applicable. Confirm the original undo on each native fixture. A
later moderator save puts the exact same anchor markup and target
back at the same location. The pre-undo whole-body hash matches and
the new native revision differs. Restart and replay the original
logical effect with its original scope. Keep the existing history
and leave the later markup intact. Check no extra native save, job,
suppression or terminal record. Only a separate eligible current read
may report presence. Otherwise withhold current status. Record the
original effect ID, receipt, before and after revision and hash and
native effect count. This is receipt recovery, with no new undo scope.

In the separate open-view branch of IT-REVIEW-EFFECT and 0031
IP-15, keep the same exact site, pairing and original effect
receipt. Withhold a result prepared under an earlier eligible
grant when access is lost before server release. Separately
release a result before revocation and delay its delivery until
the current refusal arrives through the existing poll. Require
the open view to clear details and actions on that refusal.
Deliver the earlier result and restore the old view in separate
cases. Both remain withheld. A separate current audit grant
exposes only its existing audit projection. Bind the site,
session and grant authority version, release and response order,
view state and unchanged effect count.
No native write or new notification route is involved.

[undo-reinserted-check]:
  ../specs/0028-staff-suggestions.md#how-it-is-tested
[undo-original-receipt]:
  ../specs/0021-forum-contracts-ingestion.md#link-effect-receipts

### WordPress ranged reply identity

Use [0021 `ReadRange` and test 42][wp-range-identity] on the admitted
WordPress fixture. Request the second of two equal slices in the same
part. Keep the attempt, site, post, version, part and all checksums
correct. Report the first slice's start in the authenticated reply.
SUT-308 holds assembly at its admitted position. Verify zero bytes
accepted from that reply and no part, post or index update from it.
Separately return a valid shorter slice whose start matches and end
is a UTF-8 boundary. Advance by the reported length and serve only the
verified complete post. Keep the separate SUT-347 and SUT-338 cases.

[wp-range-identity]:
  ../specs/0021-forum-contracts-ingestion.md#wordpress

### Footprint and expected evidence

Measure queries, time, bytes, writes and memory on and off under
the same load even with the service slow or absent. Separate adapter
admission and per-feature footprint research. XenForo 0021 allows one added
outbox insert per change,
at most 1 ms extra post-save time (p99), 64 KB extra bytes per save
and 8 MB extra memory per request. Public views gain no queries,
time or bytes in that slice. 0028 adds no public footprint.
For WordPress, enforce 0031's 8 MiB extra memory and no service
request during rendering. Require an approved bound and measured
proof for public query, time and byte budgets before admission.
Bind Wordfence and the cache tuple to CO-12 and CO-13.

Reverse-link every check result to the task, owner anchor, applicable
platform and case. Expected receipts bind the candidate, fixture
hashes, exact profile, actor and policy generation, effect ID and
intent digest, before and after revision and hash, refusal reason,
observed effect count and trusted producer. Use the sealed native receipt for
original logical effect and approved
bytes, the fenced service receipt for its resulting state, and the
independent runner and reviewer signature for exact built artefacts,
policy, tools, PHP, host and cache tuple. Every layer must match.
Keep only opaque receipt references in shared records. No keys,
challenge values or post text. Source inspection is separate from
a signed outcome.

### XenForo contact and heartbeat status

Use [0021 test 29][heartbeat-stale-contact] with 0031's existing site
last-contact display. Take a genuinely new signed reply to each current
poll while its saved heartbeat H remains the same. Verify the same
five-minute result under restart and uninterrupted operation at equal
elapsed times from H's first verified observation. Last contact is
current, the heartbeat warning remains after the boundary, and replay
starts no recovery. Use H2 to observe the existing reconciliation
once. H2 alone gives no verified coverage. Check the site view and
[owner recovery steps][heartbeat-owner-help] in both states: fresh
contact with H still shows the warning, and H2 ends the warning
while affected coverage stays incomplete until its own recovery
proof. The heartbeat warning gives no local supervisor restart
signal under 0112 BH-05. This fixture concerns the existing
XenForo producer.

[heartbeat-stale-contact]: ../specs/0021-forum-contracts-ingestion.md
[heartbeat-owner-help]:
  ../../manual/site-administration.md#check-a-xenforo-add-on-reporting-warning

### Remaining Phase 4 trace designs

Use synthetic content and the exact admitted platform profiles. The
[roadmap](../roadmap.md) puts the XenForo slice before WordPress.
Keep the separate signed-site contract and platform edit cases linked
from the selected Phase 4 pilot. These rows cover distinct tasks.

| Existing task and owning anchor | Proposed case and Given | When | Then | Expected proof and remaining binding |
| --- | --- | --- | --- | --- |
| `TASK-0031-SITE-FIXTURES`, [0031](../specs/0031-shared-platform-installation.md), Footprint budget, IP-06/IP-08 and CO-01. Roadmap Phase 4 item 2. | `IT-0031-SITE-FIXTURE-FOOTPRINT`. Given disposable licensed XenForo and supported WordPress fixtures on Dell with exact pinned versions, synthetic content, and matched add-on-off and add-on-on profiles. | Measure the same admitted requests and load on each baseline and enabled fixture. Plant an extra request cost and remove an input pin or required measurement. | Exact query, time, byte and memory results remain separate by platform. Page rendering makes no service call. Excess or unproved cost blocks the candidate profile. | Expected signed Dell queue, fixture and measurement receipts, exact site/package and cache identities, deltas and independent review. Bind admitted statistic, limit, load and noise for each unresolved footprint measure before applying a pass threshold. Vendor claims or a partial fixture cannot close the route. Use 0059 sealed Dell queue and slow or unavailable service. Preserve exact 0021 save limits and zero view overhead, separate from 0031 unadmitted broader statistics. |
| `TASK-0031-SITE-FIXTURES`, [0031 exact native ZIP acceptance](../specs/0031-shared-platform-installation.md#exact-native-zip-acceptance), IP-02/IP-06/IP-08 and [0005 platform acceptance](../specs/0005-release-coordination.md#platform-acceptance-design). Roadmap Phase 4 item 2. | `IT-0031-ZIP-EXACT-UPLOAD`. Given a clean, pinned XenForo 2.3 fixture and separate fixtures for both supported WordPress majors on their declared hosting profiles, exact unchanged synthetic ZIPs, detached manifests and scoped native administrators. | Verify release bytes at the owner PC, independently approve the native operation, then verify the exact native input before extraction. Separately substitute the upload, switch the site or actor, revoke native approval, withhold host proof, offer an outside recipient for the package or proof, or use a synthetic archive with two members at the same decoded path and valid signature, length and digest. Run a separate WordPress update with a verified ZIP and no current compatible proof for a changed native database. | The exact immutable stream or independently matched private stage produces one native receipt for the approved action. Each refusal leaves files, scripts, registration and activation untouched. In the extra recipient case no ZIP bytes or proof leave the approved owner-to-host boundary. A stock upload or code first run after extraction cannot fill the missing proof. For the WordPress update hold before native writes or unpack and keep the compatibility reason separate from ZIP verification. Follow the [WordPress update guide](../../manual/wordpress-safety.md#if-a-plugin-update-stays-held). | Bind the candidate, exact host tuple, manifest and consumed ZIP length and digest, native actor, action, verified handoff route, extraction or no-effect observation and native receipt. Reconcile a lost reply to that receipt before retry. Keep package proof separate from footprint, application signing, pairing and the existing whole-lifecycle qualification. |
| `TASK-0021-XF-GUEST-READ`, [0021](../specs/0021-forum-contracts-ingestion.md), Reading content, The guest rules, Checking against XenForo itself and Noticing changes. [0031](../specs/0031-shared-platform-installation.md), IP-09/IP-17, owns the shared platform checks. Roadmap Phase 4 item 3. | `IT-0021-XF-GUEST-VISIBILITY`. Given one paired synthetic XenForo site, a separate site with a colliding native ID, and a pinned native guest oracle with visible, forbidden and redirect cases. | Read an eligible revision. Revoke inherited guest rights and remove another post while work is queued. Replay the earlier signed observation and query with wrong or missing site context. | Only current native guest facts for the exact site enter serving. Denied, missing or stale posts remain unavailable and departed content is reconciled. Collision and unscoped reads expose no rows. | Expected signed add-on read and rejection, native oracle result, versioned visibility map and site-bound service/SQL receipts. Bind exact version and node policy, redirect target decision, permission generation, post deletion, job fence and recorded removal proof. Use current 0118 v2 attempt binding. Redirect target needs independent current guest oracle. Keep service and SQL cross-site negatives separate from native adapter evidence. |
| `TASK-0022-SITE-INDEX`, [0022](../specs/0022-reading-indexing.md), Pieces, Very long posts, Index generations and the revision check, and How it is tested. Roadmap Phase 4 item 3. | `IT-0022-INDEX-COMPLETE-REVISION`. Given a long eligible synthetic post with skipped zones, exact native and index generations, known text coverage, and admitted dense, sparse and multi-vector fixtures. | Stage chunks and representations, then withhold one required output. Repeat with unproved language, later erasure and replay of a staged older revision. | No skipped text becomes eligible. The complete eligible revision alone can use its admitted generation and required representations. No partial, unproved or erased candidate serves. | Expected signed source, tokenizer and coverage digests, representation completeness, generation and query/refusal evidence with site context. Bind exact per-stage variant and erasure fixtures, relevant 0024 compatibility keys and old/new reader gates. Do not count thread-level readiness as proof of complete post indexing. Pin BM25 apart from dense/sparse/multi-vector. On embedder or tokenizer mismatch hold new suggestions and recheck already eligible ones. A reading-rules-only rebuild keeps serving the current generation until exact cutover. |
| `TASK-0027-SITE-PROPOSAL`, [0027](../specs/0027-candidates-ranking.md), The relevance gate, The link-equity rules and The 3–6 suitable-destination gate. Roadmap Phase 4 item 3. | `IT-0027-RELEVANT-SITE-PROPOSAL`. Given an eligible indexed source, relevant candidates, an unrelated high-traffic page, and a private or foreign-site candidate under one exact champion manifest. | Ask for a proposal, then reduce the qualifying set below three and replace a candidate's eligibility generation before delivery. | Only candidates with both required relevance results and current scope can appear. Caps and equity hold even on shortfall. Traffic cannot restore a refused candidate. The owner gets an explainable review proposal with no native write. | Expected signed candidate/gate fixture evidence with site, source, model and champion digests, exact thresholds, ordered refusals, shortfall and no-write proof. Bind admitted scope and qualification profile, the measured fixtures for exact thresholds and any distinct variant. A proposal receipt does not prove recommendation quality or an approved edit. Evaluate both relevance checks independently, and revoke before delivery. Label zero to two eligible results as unmet 3–6 with no padding or cross-site handoff. |
| `TASK-0031-WP-GUEST-READ`, [0031](../specs/0031-shared-platform-installation.md), WordPress public eligibility, IP-09/IP-18, WX-04/WX-05 and Coexistence CO-03/06. Roadmap Phase 4 item 4. | `IT-0031-WP-GUEST-READ-SCOPE`. Given a paired synthetic WordPress site with an admitted security and cache profile, public and password-protected items, private and draft items, and a same-ID object on another site. | Compare current native public decisions and the exact signed namespace response. Change one item's visibility while a cached observation exists. Plant a blocked host or missing native proof and query an unscoped pooled connection. | Only current, published, password-free and otherwise eligible content can leave the exact site. Wrong-site and stale evidence, unknown types and blocked hosting refuse. Missing site context returns no rows and no cache restores access. | Expected signed add-on facts and refusals, native visibility oracle, site-filter and cache evidence, and held SQL result. Bind exact WordPress major, PHP, hosting and plugin rows, actor/target fixtures and the relevant guest visibility generation. The read result gives no native write authority. Use current 0118 v2 attempt binding and native anonymous eligibility, supported types and firewall refusal with zero private bytes. Keep pooled SQL probe separate from site adapter. |
| `TASK-0031-SITE-PHASE-DEMO`, [0031](../specs/0031-shared-platform-installation.md), IP-08/IP-15/IP-17/IP-18 and Coexistence CO-04/05/06. Shared [0028](../specs/0028-staff-suggestions.md) owns the native apply and undo effect. Roadmap Phase 4 exit. | `E2E-0031-SITE-SLICE-DEMO`. Given the two disposable admitted fixtures, paired test credentials, independently measured footprints, exact scoped native moderator accounts and `IT-0031-ZIP-EXACT-UPLOAD` proof for the demonstrated host tuples. | Demonstrate signed read, proposal, native review, save and selective undo on XenForo then WordPress. Repeat with revoked native rights and a later human edit. | Both positive journeys stay within admitted footprints, use native permission and preserve unrelated changes. A revoked, stale or unproved action holds with no effect. A missing platform journey or review cannot close Phase 4. | Expected signed site acceptance, edit and inverse receipts, exact link into the separate pilot cases, per-platform footprint and scope, and independent phase review. Bind exact platform tuple, native ZIP acceptance reference, recorded journey and authoritative pass entry. A previously installed test add-on supplies no first-install proof. This aggregate demonstration cannot replace any constituent test or release approval. Separate exact later-edit undo from edited-anchor conflict and role loss. Bind original logical effects, native receipts and lease. Check CO-12/13 Wordfence/cache and hold any unqualified tuple. |

Expected proof must resolve back to exact cases, inputs, candidate
bytes, admitted profile and owning task. Keep the reason and scope
of a verified baseline failure separate from later candidate proof.
Record run outcomes and review decisions through Forgejo and the
current protected evidence route.

## Batch approval recovery

Join [0028 CAP-037][ordered-batch-approval] and the existing
`acceptance/cap_037_batch.rs` to [0020's replay rule][batch-action-replay],
0019 decision and apply-job receipts, and the existing native effect
receipt under 0021 or the 0031 WordPress contract. Use three
eligible same-site suggestions in the order selected. Commit
one native effect for the first. Interrupt before the second approval
and lose the batch reply. Revoke approval and result rights. Preserve
the first decision, job and exact native receipt. Keep both later items
unperformed. They create no approval, job or native effect. Release
no protected result under revoked scope. Check the
[moderator recovery guidance][batch-result-owner-help]. It directs
the moderator to the original ordered group and preserves the
access hold instead of offering a repeat of the whole approval.

In the separately authorised control, use the original batch plan and
exact ordered item IDs, versions and destinations. Preserve the first
outcome once with the original receipt and no second native write.
Recheck an unperformed item's current plan and rights through its
ordinary action. Plant a stale selected version and a loss of rights
in separate runs. The batch result cannot override either refusal.
Refuse a reordered or substituted batch plan. Record ordered item
outcomes and before and after counts for approvals, jobs and native
sends. Keep the original approval and native-effect authorities.

[ordered-batch-approval]: ../specs/0028-staff-suggestions.md#batch-approval
[batch-action-replay]: ../specs/0020-screens-actions-sutra-mcp.md#failures
[batch-result-owner-help]:
  ../../manual/site-administration.md#recover-a-group-of-approvals

## Phase 5 numerical work, reports and key custody

Use synthetic fixture data and test keys. Task IDs come from the
[implementation catalogue](implementation-catalogue.md). Reuse the
owning checks and bind each proposed joined case before admission.
The results below are expected observations.

### TASK-0066-GANIT-PLATFORM

Proposed case `IT-0066-JAAL-ONE-LOGICAL-RESULT`.

**Anchors.** [0066](../specs/0066-ganit.md) Where it runs and tests
6, 8–10 and 12. [0019](../specs/0019-events-jobs-recovery.md)
REQ-0019-DURABLE-EXECUTION, DES-0019-DURABLE-PLATFORM,
UT-0019-REGISTRY-UNIQUE, UT-0019-FENCE-COMMIT and
IT-0019-DELIVERY-DUPLICATE supply the shared scheduler proof.

- **Given** one registered batch fixture for Jaal job 251 on an
  invented graph, with a fixed snapshot, settings, seed, version,
  expected output and current same-site maintenance grant and lease.
  Register ganit.251 in its nightly window with one-at-a-time
  identity and exact measured time blocker from 0066.
- **When** the worker stops after a checkpoint and karya resumes the
  same logical job. Repeat with a wrong job owner, unknown schema or
  entry point, a text field, wrong site, cancelled or stale lease
  and an invalid number.
- **Then** the admitted restart gives the same stamped result as the
  uninterrupted case, with one logical result. Unknown or invalid
  input never reaches execution, and invalid numerical output or an
  old fence cannot commit. The result is inert challenger evidence.

**Receipt.** Bind the registry and entry point versions, safe snapshot,
settings and seed identities, expected oracle, checkpoint, job and
effect IDs, current lease and output digest. The owners need stable
job-251 variant and joined-case IDs. This fixture closes no other
engine or job contract.

### Jaal link prediction in the expanded scope

The [0064 graph scope][jaal-prediction-scope]
links to the registered Jaal job 256 in
[0066](../specs/0066-ganit.md#jaal-graph-analysis). This is the producer of
Adamic–Adar, resource allocation and Katz scores and ordered
candidate identities. Wave 2 F-0256 owns Editor accessibility in 0041.
Keep that feature identity separate from this graph job.

Use 0066 tests 2, 4, 5, 7 and 8 for the job's acceptance design.
The small graph in test 2 has hand-worked Adamic–Adar and resource
allocation answers. Check each against its own admitted input and
output contract. The Katz profile needs its own hand-worked case,
method admission and resource bounds under 0066. A result for either
other score, job 251 or the shared scheduler supplies no Katz proof.

Use the [0049 relevance and result contracts][jaal-prediction-consumer]
for the versioned candidate. The [0027 candidate route][jaal-prediction-ranking]
consumes only admitted identities from job 256 through the existing
0023 retrieval filters and 0077 champion authority. Exercise missing
or invalid graph evidence and keep the required BM25 and BGE routes.
A job result alone authorises no active ranking change or link write.

Bind the qualified job, variant, graph snapshot, score definition,
expected result, version and evidence recipient in the separate
derived record under 0076. Maintain the exact canonical scope binding
under 0064 before using its signed qualification. This source trace
creates no implementation task or qualifying result.

[jaal-prediction-scope]:
  ../specs/0064-v1-qualification.md#what-the-product-covers
[jaal-prediction-consumer]: ../specs/0049-knowledge-graph.md#relevance
[jaal-prediction-ranking]:
  ../specs/0027-candidates-ranking.md#candidate-generation-ownership

### TASK-0027-GANIT-JOB-ROUTE

Proposed case `IT-0027-JOB-FALLBACK-BOUNDARY`.

**Anchors.** [0027](../specs/0027-candidates-ranking.md) Job 1 and
degraded mode, Ganit jobs used, Champion outputs and the link-set
proof under How it is tested. Use [0066](../specs/0066-ganit.md) job
147 and [0077](../specs/0077-autonomous-ranking-optimisation.md) for
the separately scheduled maintenance candidate.

- **Given** a fixed verified champion, an invented public source and
  six eligible targets, an unrelated target with a higher raw
  ordering score, and one offline ranking-policy candidate.
- **When** job 1 returns a valid link set, then separately fails or
  exceeds 300 ms and sends a late result. In other cases return
  malformed or filtered output per 0027 tests 5 and 6. Offer stale or
  incompatible
  job-147 evidence to the challenger path during the live requests.
- **Then** job 1 output is checked against relevance, hard exclusions,
  anchor and destination caps and equity. The declared timeout or
  qualifying failure uses the labelled SUT-549 fallback, with no more
  than five per anchor and the same 3–6 eligibility rule. A shortfall
  remains visible and unpadded. Discard late results and pin one
  champion per post. Hold job 147 as a separate candidate until 0077
  admission. It cannot supply live output.

**Receipt.** Bind the exact consumer and job-1 contracts, fixture,
serving manifest and output digests, timer and reason, before and
after champion identity and maintenance result or hold. Bind separate
stable case and variant IDs for the typed routes and the slow and
incompatible results. Production timing and promotion need their
own proof.

### TASK-0034-VISIT-COUNTING

Proposed case `IT-0034-COUNTING-SOURCE-WITHDRAWAL`.

**Anchors.** [0034](../specs/0034-analytics.md) Counting modes and
consent, The freeze rule, AT-AN38 and AT-AN31/32/35. QR-088 owns
admission of the exact log source and its evidence.

- **Given** synthetic eligible beacon, authorised log and separate
  opted-out cases, under anonymous and consent-required modes. Pin
  the exact site and source grant, open day, counting mode, consent
  epochs, ingestion identity and previous clean rate.
- **When** an admitted line arrives 40 seconds after its beacon, a
  separate line arrives without a beacon, and the committed cursor is
  replayed. Vary withdrawal, missing proof and a closed date. Stop
  the source, replace it and shorten retention as independent cases.
  Exercise a distinct traffic-anomaly freeze.
- **Then** the admitted join marks the existing visit without adding
  a count. A separately eligible log-only visit counts once. Refused cases add
  neither a mark nor a
  count. Each change immediately fences stale output and verifies removal.
  A rebuild preserves
  eligible beacon-only visits. The freeze retains the last clean,
  admitted per-thread human rates, or leaves them unavailable. Verified
  crawler-only observations stay separate, while
  current non-traffic inputs and labelled report counts remain.

**Receipt.** Bind mode, scope and source generations, window and cursor,
labels, counts, coverage and protected retention and freeze outcomes.
Keep raw log and visitor identifiers out of shared evidence. Bind the
exact production source variant and separate freeze case to the
counting task. Collection and ranking approvals remain independent.

**Phase 5 consumer proof.** Reuse 0034 AT-AN47 with the exact 0066
job 325 registration, the 0019 claim and the 0077 verified serving
manifest. Give the darshak consumer a finite, in-range old result
whose snapshot was superseded and whose claim is stale. Its existing
SUT-614 outcome creates no new serving rate or series value. Verify
the unchanged active manifest and read only otherwise eligible
serving inputs. Check that a current candidate gives no activation
without the 0077 decision. Keep the infinite-value case separate.
Bind receipt IDs, old and current snapshot and claim identities, and
the before and after active manifest. Keep visitor data out of the
receipt. Jaal job 251 and counting, freeze and scientific-source
proofs retain their own scopes.

### TASK-0035-OWNER-BOOKS

Proposed case `IT-0035-BATCH-RECOVERY-BOUNDARIES`.

**Anchors.** [0035](../specs/0035-hisaab.md) The night's run, Balancing
rules and signatures, AT-HS02/07/10/12, and restart equivalence.

- **Given** one synthetic batch ledger for two sites and two
  currencies, with a negative refund, an item in suspense, a prior
  verified statement and independently calculated expected totals.
- **When** the batch stops after a chunk and resumes on the same
  inputs. Replay after a successful ingest. Separately try a
  duplicate source ID and bad balance. Change input hash for SUT-860
  refusal, present unchanged input and incompatible checkpoint for
  SUT-868 safe restart, and change sealed output for SUT-862 before
  parsing.
- **Then** the statement keeps exact signed amounts and separate
  site, ledger, currency and day totals. Each scoped reader sees
  only its own RLS-bound report. Resume at most one repeated chunk
  with one accepted report. Debits equal credits and
  journal totals equal posted plus suspense. Replay adds no second
  entry. Invalid or incompatible evidence never replaces the last
  verified statement.

**Receipt.** Record the file and run-manifest digests, contract and
rule-table versions, typed balancing result and site-scoped consumer
receipt. Verify the batch HMAC through its owning route. This does not
supply the signed candidate result. The owner must bind stable statement
and compatible restore cases and explicit currency variants.

### TASK-0065-VAULT-LIFECYCLE

Proposed case `IT-0065-SITE-KEY-EPOCH-FENCE`.

**Anchors.** [0065](../specs/0065-tijori.md) The signer registry,
Keys at rest and the site-key, wrong-purpose and restart tests.
[0118](../specs/0118-end-to-end-encryption-gate.md) REQ-0118-KEY and
FI-0118-KEY cover revocation and restart for protected data keys.

- **Given** one scoped synthetic service key and separately purposed
  test protection keys, with a current key record and queued use.
- **When** the owner-approved fixture rotates the site-link key,
  distinguishes planned site-link overlap from immediate revocation.
  Offer prior queued request, stale restore, invalid same-generation
  replacement, wrong purpose or request to export private bytes.
  Keep separate 0118 protection-key epoch under its own lifecycle.
- **Then** only the entitled current key and purpose succeed. An
  admitted site-link changeover never revives a revoked key. A
  stale or missing current record gives no authority. No key bytes
  appear in logs, errors, reports or exports.

**Receipt.** Bind public key fingerprints, purpose and current policy,
last authorised site-key change, predecessor, version and issuer
scope, trusted minimum epoch outside restored state and refusals.
Bind signing and encryption variants to their own checks. This slice
uses private test keys and does not qualify the release-key task.

Bind each expected signed runner receipt to the exact candidate tree,
built artefacts, platform and tool pins, fixture, task and check IDs,
observed results and independent review. Apply the 0003 evidence path
and 0076 reverse trace through the reconciled Forgejo task and variants.

### Phase 5 installation and recovery scenarios

### Native release consumer trace

Use the owning tables for [0005 delivery][native-delivery-ids],
[0030 PC package use][pc-handoff-ids] and
[0031 native upload][zip-handoff-ids]. These are planned acceptance
links. Bind the exact variant, case and observed outcome through
0076 at acceptance.

[native-delivery-ids]:
  ../specs/0005-release-coordination.md#native-delivery-boundary-identities
[pc-handoff-ids]:
  ../specs/0030-install-upgrade-removal.md#native-package-handoff-trace
[zip-handoff-ids]:
  ../specs/0031-shared-platform-installation.md#native-zip-handoff-trace

| Existing task slice | Exact owning links | Case boundaries |
| --- | --- | --- |
| `TASK-0030-PLATFORM-INSTALL` | 0030 `DES-0030-LINUX-PACKAGE-HANDOFF`, `DES-0030-LINUX-NATIVE-RESOLUTION`, `DES-0030-WINDOWS-FIRST-TRUST` and `DES-0030-WINDOWS-LIFECYCLE-HANDOFF`, and 0005 `REQ-0005-WINDOWS-TRUST-SEPARATION` | Bind `IT-0030-VERIFIED-STAGE` to each admitted Ubuntu or Debian initial install. Bind `IT-0030-WINDOWS-FIRST-TRUST` to Windows initial and portable first launch including applicable component branches and the separate QR-099 substitution. |
| `TASK-0030-UPDATE-REPAIR` | The same 0030 owning designs for the applicable update and repair variants, plus 0005 `REQ-0005-WINDOWS-TRUST-SEPARATION` for Windows | Bind exact owner update approval or repair intent. Exercise `IT-0030-VERIFIED-STAGE` including native preflight and current resolution where applicable, and the exact Windows lifecycle branches of `IT-0030-WINDOWS-FIRST-TRUST`. A no-resolver repair has its own scope. |
| `TASK-0031-SITE-FIXTURES` | `DES-0031-NATIVE-ZIP-HANDOFF`, its four native requirements and `REQ-0005-ZIP-RECIPIENT` | Bind `IT-0031-ZIP-EXACT-UPLOAD` for XenForo and both WordPress majors to the exact adopted stream or private-stage path, site, actor, no-extra-recipient proof and one native effect receipt. |

Use 0005's detached exact, mix-up and trust cases for the upstream
profile and 0118's separate release protection proof. Follow each
requirement back to its one owner. An observed package result in
another variant gives no evidence for the selected native step.
For the separate lost-response case under IT-0031-ZIP-EXACT-UPLOAD,
remove current native receipt access after the exact install
commits. Keep its original receipt and package proof, withhold the
protected result from the former administrator and run no second
installation. Public help is independent. The authorised control
reads only the original same-site native effect.

For TASK-0031-SITE-FIXTURES, join IP-08 and
IT-0031-ZIP-EXACT-UPLOAD to 0031's WordPress edge and compatibility
register. Matching ZIP, core and PHP bytes plus a genuine
single-site lifecycle result do not qualify an unproved multisite
target. Hold the native effects before extraction and preserve the
original proof. Keep 0021's encrypted pairing check and 0139's
same-profile refactor comparison separate from topology admission.
For the separate native update case, keep the verified WordPress
ZIP, immutable handoff, approval and current site fixed. Substitute
only the current compatibility result with an authentic XenForo one
for the same release and action class. Hold before native extraction,
migration or activation. Preserve the previous files, state and
receipts. Keep any shared-app rollback decision under IP-26's
independent checks for each registered site.

#### TASK-0030-PLATFORM-INSTALL

**Scenario.** Proposed IT-0030-PLATFORM-INSTALL, pre-launch and first
setup. Use [0030](../specs/0030-install-upgrade-removal.md) Preflight,
Installation, Windows first trust check, tests 1–5/17/20/22/23,
0031 IP-07, and QR-100 and QR-202. Use 0078 for the memory envelope
and 0108 for the installation scope and exact migration receipt.
Use [0118](../specs/0118-end-to-end-encryption-gate.md) PL-0118 for
exact package protection.

- **Given** clean Ubuntu 26.04, Debian 13 and Windows 11 fixtures
  with the declared package and portable variants. Resolve supported
  release targets from protected policy. Evidence from an isolated
  candidate-only target supplies no ordinary installation authority.
- **When** each fixture checks the release manifest and starts setup.
  Use 0005 IT-0030-WINDOWS-FIRST-TRUST for Windows wrapper digest and
  verifier key refusal, and for payload, native and WebView validation.
  With the same signed manifest and original verification receipt,
  replace the verified inert wrapper before launch. In a separate
  case keep the wrapper unchanged and replace a verified required
  component before its first load. Independently keep the exact plan
  fixed on release B and offer complete genuine A proof and artefacts
  before verification. Then in a separate case verify B and supply
  the genuine A wrapper and its proof at graphical handoff. SUT-832
  refuses both wrong-release cases before any wrapper starts. Neither
  case changes the B plan. Compare unchanged verified B bytes at
  each handoff. SUT-832 also refuses the changed-byte cases. A changed
  wrapper starts no installer. A changed member is never loaded and
  records no later effect or setup success from the held step. Keep
  already verified effects for reconciliation under the existing
  recovery path. Reuse [0030's Windows handoff cases][windows-handoff-cases]
  for the explicit first-install control and separate declared
  bundled and downloaded component branches. On the interrupted
  initial-install branch, keep the exact release and member, earlier
  verified checkpoint and setup data. Offer only the former policy's
  first-load binding after restart. Expect SUT-832 again while the
  proof stays stale, with no repeat earlier effect, new load or setup
  success. Check recovery with current binding on the same exact
  step under 0005 and QR-202. An offered genuine QR-099 result with
  QR-202 first-trust proof withheld keeps the
  affected wrapper or component unloaded. Preserve SUT-832 and
  the existing unsigned-wrapper warning. On Ubuntu and Debian, use the detached
  Linux delivery route and spec 0005 IT-0030-VERIFIED-STAGE. Check
  that the native installer consumes the exact verified package
  through its privilege boundary. Replace the staged copy after
  verification and separately withhold the native handoff proof.
  Each failed proof produces the owning 0030 SUT-832 and no native
  install effect. Use [the separate Linux package cases][linux-package-cases]
  for the admitted initial-install dependency profile and independent
  native refusal. Keep first trust and install authority distinct from
  update and repair approval. Reuse the MAN-PLATFORM recovery guidance
  for the verified retry. Keep add-on authority separate. Pull a portable
  drive unexpectedly.
- **First creation.** On each applicable route start with a missing
  home, no ledger and no enrolled site. Withhold the native intent,
  volume reservation, lock or required audit evidence in separate
  branches. Race two installers and replace the approved target
  after intent. Interrupt after home creation but before its receipt,
  then separately before the first allowed SQL mirror. Replay with
  the same verified intent and with a substituted or unproved one.
  On recovery remove the original durable audit proof or offer proof
  from another intent. The affected attempt holds with no new effect,
  new success audit entry or SQL mirror advance. Matching proof keeps
  the original audited effect and receipt identity across restart.
- **First setup memory.** Pass preflight, then separately remove
  current memory proof or leave insufficient available memory before
  PostgreSQL launch and before migration SQL. SUT-830 names the
  held check. There is no new process at the first hold, no changed
  schema or migration success at the second, and no new site or
  scheduler scope. On a resumed setup with earlier verified DDL,
  the pre-session hold preserves that DDL and adds no new effect.
  Test proved budget excess separately as SUT-837.
  Retain the verified running instance and launch receipt. Restore
  current capacity proof and continue the same intent. Reconcile
  a lost reply to the exact committed migration and setup receipt.
  Count one launch and one migration effect.
- **Resumed legacy history.** Reuse the 0108 IT-BD-06 history frame.
  Give the exact authorised setup intent and a prior committed
  migration with matching version and checksum but no verified
  history ownership. 0108 holds before row access with SUT-971.
  The installer shows SUT-833 and names the failed history check.
  Keep prior native receipts and committed data. No new DDL, SQLx
  success row, setup mirror, rollback or timing success. Do not
  treat the existing installation as empty.
- **Then** verification precedes extraction or execution. Windows
  uses independent pre-launch trust for the unsigned wrapper.
  Only an eligible native attempt makes its owned files. Missing
  proof or an unproved creation blocks adoption and a second effect.
  Reconcile the original receipt once and mirror only the committed
  installation records allowed by 0108 under forced row-level
  security. No site, connector, unscoped audit write or extra native
  authority follows. Portable mode adds no start-up entry. Unexpected
  unplug fences launch until the exact volume and ledger reconcile.

**Evidence.** Bind the exact package, payload, native handle, intent,
lock, disk reservation, durable audit or refusal evidence, file
ledger and scoped setup result. For the Windows first-trust cases,
record the selected plan and release, the offered release and its
proof disposition, the verified wrapper or member digest, actual
handoff identity or refusal, and observed wrapper launch or member
load count. Bind the existing QR-202 profile and independent verifier
separately from QR-099. Bind the first-setup memory checks
to the current 0078 envelope, platform process limit proof, the
0030 migration reservation and each held or reconciled step.
QR-202 first trust and native file
effects are separate from QR-099 protected build cutover and QR-100
platform qualification. QR-015 retains the separate Windows
uninstaller registration approval.

#### TASK-0030-UPDATE-REPAIR

**Scenario.** Proposed IT-0030-UPDATE-REPAIR, interrupted update
and exact repair. Use 0030 Update, Repair and reset, tests
7–11/23/24/27, 0031 IP-23 and IP-26, and QR-204. PL-0118 and
E2E-0118-MOVE own the protection floor through update and rollback. If recovery
uses a separately confirmed restore, bind 0031 IP-30 for that exact
admitted restore variant through TASK-0030-BACKUP-RESTORE.

- **Given** a verified installation with current user data, settings,
  scripts, an eligible backup and the approved replacement payload.
- **When** offer the signed update plan to a non-owner who held the
  old app update permission. Separately change the plan after the
  installation owner previews it. With a fresh exact owner approval,
  interrupt at each declared switch boundary and fail the new version's
  self-test in a separate run. Repair one known damaged program file
  while leaving an unknown file. Try SUT-836 with an ordinary
  administrator and with the exact owner-only backup override.
- **Then** SUT-801 and SUT-803 leave the running version and write state
  in place. With a matching approval, recovery exposes one complete
  verified version. The prior version remains usable only with
  verified compatible tables and current trust and erasure floor.
  The separate IP-30 outcome proves recovery material and isolated
  restored RLS for the selected restore. It supplies no live
  restart, rollback or switch receipt. IT-SS-07 supplies registry
  retirement and missing-pairing-key results. AT-CF-DELIVERY
  supplies settings epoch and activation evidence. Preserve
  current user work without repeating it. Repair the known file
  from exact signed bytes and leave the unknown file untouched.

**Current release trust before fallback.** Join 0030 test 9 and 0031
IP-26 under QR-202. Keep the exact previous bytes, original receipt,
owner plan, lease and compatibility proof. On a resumed fallback,
apply the current verified 0005 acceptance floor for the admitted
native profile. The excluded previous version stays stopped with
no resumed writes, live rollback receipt or timing pass. Keep the
initial installer public-proof and 0118 key-floor cases separate.

**Windows native handoff during lifecycle work.** Reuse
IT-0030-WINDOWS-FIRST-TRUST and the
[0030 Windows handoff cases][windows-handoff-cases].
Bind the current exact owner plan or repair intent, candidate,
verified stage, and first-load or launch result. Compare the
unchanged control, changed native input, and matching downloaded
bytes with current handoff proof withheld. Record no affected
component load, later effect or success on SUT-832. Keep verified
prior effects for the existing recovery route. For portable first
launch use the independent applicable profile and current manifest,
with no native installer effect. No first-install receipt qualifies
one of these lifecycle routes by itself.

[windows-handoff-cases]:
  ../specs/0030-install-upgrade-removal.md#windows-first-trust-check

**Authority after migration.** Join IP-23 and IP-25 for the verified
offline update. Commit its forward migration, then withdraw only the
owner's live update authority before activation. Restart with the
same verified package and exact migration receipt. The owner sees
the recorded approval hold. No repeat migration, new payload start,
switch or resumed writes follows. Reconcile the completed checkpoint under
current read authority. Only exact current owner confirmation through
the existing route can admit the otherwise eligible remaining work.
Follow the original update in the owner guide and preserve its hold.

**Native grant after service update.** Join 0030 test 31,
IT-0031-ZIP-EXACT-UPLOAD and the 0021/0118 conversion boundary.
Commit the owner's app update, then remove only the actor's
native site update capability before using the verified ZIP.
Keep the app result and site files. Assert no native extraction,
activation or protected exchange. Current exact native approval
and independent endpoint proof admit their own steps only.

**Current database starter before rollback promotion.** Use 0030
test 11 and 0031 IP-26 with 0108's role and forced-RLS guard on
each admitted Linux and Windows profile. Keep the actual fallback
payload and migration version and checksums fixed. After preflight,
lose forced RLS on the history table in the synthetic starter
view. SUT-971 holds readiness, active pointer and writes. Preserve
the earlier SQLx outcome and prove no successful rollback or
timing result. Neither a migration receipt nor an isolated
restore proves this fallback's current database check.

**Current encryption floor before fallback.** Join 0118 E2E-0118-MOVE
with 0031 IP-26 and 0030 tests 9 and 11. After the failed update
self-test, interrupt at an otherwise eligible fallback before the
previous process starts. In the control use the current external key
floor and exact store and reader proof. Verify one protected return.
Advance the floor after preflight and before the guarded launch.
Replay the earlier authentic proof at restart and without restart.
Stale or missing proof gives SUT-3700. In the separate case with
a verified current floor but no compatible protected key path,
show SUT-3701. Keep writes paused and data and journal bytes sealed.
No prior-payload start, protected hand-off, completed rollback,
SUT-835 or timing pass follows a refusal.

**Current erasure history before fallback.** Join the existing
0031 IP-26 and 0118 E2E-0118-MOVE fixtures. Use the 0030 protected
history boundary and 0032 erasure evidence from AU-08. Keep the
current key path, site scope, owner plan and lease valid. Offer
the exact current history with a committed erasure in a format
unsupported by the previous payload. In a separate run also
offer an earlier readable checkpoint that predates the erasure.
Both hold launch and preserve the validated current history,
audit receipts and live data. No affected read, prepared export
delivery, write resume or rollback result follows. The control
enforces the current history through one verified protected return
and keeps the erased data unavailable. AU-08 still owns the
separate restored-generation test. Bind the exact previous
payload, reader, history format and checkpoints, scope and
observed outcome before admitting the fixture as proof.

**Exact repair destination.** Join 0030 test 24 and N-1870, 0031
IP-02 and IP-27, and 0005 IT-0030-VERIFIED-STAGE for the source
handoff. After approval to restore a missing managed program file,
put an unowned object at the target after the final check. Give it
the same bytes. Keep the approved package, ledger and root fixed.
SUT-4100 leaves the object and
ledger untouched and offers Check files. In a separate run keep
the target eligible and substitute the verified stage. SUT-832
refuses package use. Neither refusal starts the affected file or
records completion. In the eligible run lose the reply after the
verified repair commits. Read back the exact original receipt
under current authority without a second file write. Match the
ledger intent and owned effect proof independently of byte equality.
Bind the synthetic installation, plan, target, stage, trust and
native receipts under QR-204 on each applicable admitted profile.

**Dependency proof at the Linux native handoff.** Reuse 0005
IT-0030-VERIFIED-STAGE and [its separate cases][linux-package-cases]
under 0030 Linux package trust and handoff. Bind initial install to
its own first-trust and setup authority. Bind an update or a repair
using the native resolver to its own current approval and lease.
Keep each admitted Ubuntu and Debian route and exact verified .deb
separate. A managed-file repair that never calls the native
resolver supplies no evidence for the resolver case.

For each relevant route compare an otherwise valid out-of-profile
resolution, an unchanged visible plan with no independent current
proof and a verified resolution replaced before native use. Use
SUT-832 and verify unchanged native effect counts for each
refusal. Separately present a direct package open or a repository
signature as the only proof, and verify no managed handoff. Then
keep the exact package, current resolution and handoff proof and
fail only the admitted native package preflight. Retain its own
reason. No unpack, dependency install, maintainer script, native
write or activation follows. The exact eligible control enters
its other owning checks. Keep any verified checkpoint and original
receipt under the existing reconciliation rules.

For the separate late-permission branch, keep the native preflight
pass, exact approved update and lease current when the native role
loses its permission for the exact update. The admitted Linux
handoff holds without a privileged package effect or native success
receipt. A passed isolated data restore under IP-30 supplies no
permission for this package action.

Bind the action, host and dependency profile, expected and observed
resolution, verified stage, native preflight disposition, current
native permission result where tested and safe reason. Use a
synthetic resolver and installer with no live calls. Follow the
[owner update guide][linux-update-dependency-help] in the unchanged
package case with current dependency proof withheld. Check that it
points to the recorded SUT-832 reason and requires current resolver
proof. A changed plan goes through fresh owner confirmation.

[linux-update-dependency-help]:
  ../../manual/operations.md#update-or-roll-back

[linux-package-cases]:
  ../specs/0005-release-coordination.md#linux-native-package-checks

**Space at the final switch.** Join 0030 test 29 and 0031 IP-24 and
IP-25. Give a passing self-test, verified forward migration and the
current plan and lease. Reduce space on the admitted volume before the
switch. Keep writes paused and preserve the previous payload, pointer
and migration checkpoint. Show the disk hold. No switch, resumed writes,
update completion, fallback launch or rollback receipt follows. Restore
proof and recheck the original operation. Separately commit an eligible
switch, lose its reply and reduce space. Read only the verified original result
under current authority. Hold pending effects until their own checks
pass. Compare effect counts and the exact pointer and journal, with one
migration and at most one switch in each operation.

**Lifecycle holder after staging.** Use 0031 IP-28 on each applicable
admitted Linux and Windows profile, separately from app rollback.
Commit one verified staged-file effect for the new lifecycle part
and lose the reply. Retain both verified payloads. On restart,
offer only the earlier holder proof. Read the exact historical
file result under current authority and hold all new lifecycle
effects. Assert no new holder, repeated file effect, fallback
launch, resumed writes or rollback receipt. The exact one-holder
control needs current hand-over proof through the admitted native
profile and lease. Record stage result and holder disposition
separately. Keep the recovery route available.

**Root at the final switch.** Use the separate 0030 test 29 and
0031 IP-24 branch on the admitted synthetic Linux profile. After
verified migration and self-test, change the backing directory at
the displayed approved path. Keep the native volume, adequate
capacity, verified package, current plan and lease. The exact root
check holds publication and writes, preserves the earlier pointer,
payload and migration receipt, and gives no fallback or success.
Verify no write or cleanup on the substitute. In the original-root
control recheck the current reservation, journal, plan and lease.
Reconcile remaining effects without repeating migration.
In the existing test 29 after-check variant, let the final check
succeed against backing root A. At use, present B at the same path
and native volume with identical staged bytes. Hold before switch
and write resume. Preserve the prior pointer, migration receipt,
journal and original owner approval. Bind the current lease and
match the expected, checked and at-use native root separately from
the unchanged volume and ledger reservation. In the control prove
the exact root through the committed effect and its read-back.
The fixture changes no live mount or installation.

**Local effect journal.** Use 0031 IP-26 and 0021 Link effect
receipts. On the disposable paired fixture, keep the exact committed
native apply receipt and the later human revision. After a failed
candidate self-test, compare rollback using a supported local effect
journal format and a well-formed newer local effect-journal format
unsupported by the exact previous payload. Keep native evidence,
permissions and all other fallback proof fixed. The supported reader
reconciles the original terminal effect once. Verify that replay
preserves every later byte and issues no second native write. The
unsupported reader holds before prior-app launch and preserves the
local journal, native receipt and post. No retry, absent-effect
result, automatic restore, rollback success or timing pass follows.
Record the exact journal format and reader identities, the original
effect and receipt, the before and after post hashes, and launch and
native-write counts. Use no real site queries.

In the IP-26 branch fail the new version's self-test and checkpoint
with compatible proof for two synthetic paired sites. Before the
previous app starts, change one site's native add-on version to an
incompatible version. In a separate run keep the version fixed and
revoke the native pairing used by that site's proof. Then join 0021
signing test 3 in an independent case. Keep the installed version,
current keys and pairing generation fixed and present a restored
receiver replay store for that site without current continuity proof.
Resume from the same checkpoint with the earlier proof present. The
other site's proof and all other fallback inputs stay fixed. Require
the current proof recheck under the original operation lease. The
previous app stays stopped and writes stay paused. No protected send,
cursor advance or native effect follows. Retain existing data and
receipts and report the paused outcome. No verified rollback receipt,
SUT-835 or timing pass follows. The control verifies both sites'
current proofs and the exact replay-store continuity. On the admitted
XenForo and WordPress fixtures, the receiver separately proves the
accepted-message repeat refusal in 0021. Use no real site queries.

**Evidence.** Compare before and after data and job identities,
migration journal, pointer, backup, self-test and repair digests.
Bind the native version, pairing and replay-store dispositions
separately to the owning checkpoint. Verify no prior-app launch,
cursor advance or native send on each pre-launch refusal. For IP-26's
post-start database branch, keep the one verified prior-app start.
Present the new incompatible schema revision before protected work
or a verified return. Preserve committed data and receipts, keep
writes paused and record no second start, rollback or timing pass.
The control uses the exact compatible current revision and must
still prove return under the original lease. Bind the schema
revision at start and at the held completion separately.
Bind separate interruption and self-test cases in 0030. A measured
rollback needs the owning platform and load evidence.

#### TASK-0030-REMOVE

**Scenario.** Proposed IT-0030-REMOVE, confirmed scope and retry.
Use 0030 Disable, enable and remove, N-1452/N-1453, tests
13/14/20/21/26/27 and QR-201.

- **Given** two sites with equal local IDs, an exact ledger preview,
  optional groups on Keep, one optional Delete choice and an
  eligible export awaiting verification.
- **When** confirm, race a pending job, vary the ledger after preview
  and interrupt a valid remove after one scoped deletion. Reconcile
  the same plan. Run site-only and whole-install cases separately.
- **Changed baseline.** In separate cases, complete an authorised
  app upgrade or retire one of the sites after previewing whole
  removal. Offer that earlier plan and its confirmation. Keep
  current records intact while the stale plan holds. A fresh plan
  binds the current verified release, native register generation,
  checked ledger and confirmation from each affected site under
  0031. No retired site returns to the portfolio. Use UT-SS-16 for
  the separate installation-binding refusal.
- **Wrong site and target.** With matching synthetic local IDs,
  try a fixed-site command spanning sites and a desktop call whose
  explicit site differs from the current window. Check SUT-1901
  and SUT-1904 at the owning scope checks. Separately use an
  approved path replaced with a different file identity. Check
  SUT-4100 on the ledger-to-disk mismatch. All three cases leave
  the unapproved target untouched.
- **Removal help.** Check the Remove refusal and the manual's
  “If removal is held” steps. An unused stale confirmation gives
  SUT-803 and fresh review before supervisor pause, deletion or
  an `uninstall.finished` event.
  A site mismatch directs the owner to the intended authorised
  site without disclosing another site's details. A disk mismatch
  offers Check files and no deletion of the replaced item.
  Interrupt after an already verified deletion. Keep that result
  visible to an authorised reader and reconcile an uncertain later
  effect before retry. In the separate authority branch, close the
  window and withdraw current permission for the remaining native
  site cleanup. Keep current read access and the exact
  plan and journal. Follow the manual: preserve the verified earlier
  result and hold new cleanup with no completion receipt.
- **Then** stale scope or incomplete required export holds. The
  supervisor stops owned work before deletion. Replay removes only
  remaining approved identities in exact rechecked root and path.
  SUT-4100 protects mismatched identities. Optional Keep and retained
  RLS stay in force. Delete cloud data only under the exact separate
  choice. Keep the final receipt only by request. Unknown or
  unreachable groups give no completion proof.
- **Offline Delete result.** Join 0030 test 21, N-1464 and 0032
  AU-13. Verify one local deletion, lose the result for an approved
  managed file on another volume, detach it, and restart. An empty
  directory at its old path does not settle that exact deletion.
  Preserve the verified local result, fence the issued file link
  and keep the plan and lifecycle journal resumable with no final
  receipt requested. Withhold lifecycle teardown and
  `uninstall.finished` while the copy is unproved. With the exact
  original volume and current authority, reconcile the original
  deletion once before terminal completion. The fixture changes
  no real files or mounts.
- **WordPress disconnect and native removal.** Under 0031 IP-32 to
  IP-34, distinguish a completed site disconnect with the plugin
  installed from a selected native cleanup with its result lost.
  Verify that local revocation stays in effect while native cleanup
  remains unverified. Offer a pre-removal native backup with the old
  site key and prove it cannot restore access. Check the
  [owner removal guidance][wp-native-removal-help] and preserve links
  and the other site. Record the cleanup outcome only from the
  original authorised effect.
- **Whole-install Keep case.** Under test 21, keep an exact verified
  encrypted backup, choose Delete for eligible ordinary audit and
  decline the optional final receipt. Interrupt before the lifecycle
  part leaves. Reuse IP-33 and IP-34 for exact removal and grant
  revocation. Verify the retained backup and its required retirement,
  erasure and minimum key epoch evidence before claiming completion.
  Keep the epoch proof outside the restored bytes. List the kept
  backup and protected evidence with their reasons. Leave no optional
  receipt and no live grant or site key authority. With missing proof
  withhold completion even if the backup bytes are intact.

**Evidence.** Bind plan, current release, native register generation,
current ledger, export digest, stop checks, second-pass process and
storage checks and per-group dispositions. Reuse 0031 IP-27 and
IP-32 to IP-34, 0102 IT-SS-07 and the owning scope refusals.
Bind the kept backup digest, checked protection history, key and
grant refusal, and receipt absence separately. The owner must bind
both scopes and partial outcomes to stable IDs.

[wp-native-removal-help]:
  ../../manual/operations.md#remove-a-site-or-the-installation

#### TASK-0033-SAFE-RECOVERY

**Scenario.** Proposed IT-0033-SAFE-RECOVERY, unsafe disk and
unconfirmed notice. Use [0033](../specs/0033-safe-running.md)
Low-disk notices and SR09–SR11, plus 0019
REQ-0019-COORDINATED-RECOVERY for shared reconciliation.

- **Given** a synthetic profile with fixture disk bounds, an eligible
  queued backfill and one recipient of a notice.
- **When** a reading makes the disk unsafe and notice confirmation
  is lost. Repeat after restart, present a late or tied reading, and
  later present a clearly safe reading for the owning hold interval.
- **Then** no new promises are accepted and backfill pauses first.
  No automatic deletion occurs. An unknown notice stays unresolved
  until reconciliation. There is one unsafe and one recovery notice
  per recipient and spell. Bind transition, spell, recipient and
  outbox atomically. Missing or corrupt notice journal keeps unsafe
  scope visible and holds new promises. Unsafe warning and quiet
  recovery hours have distinct outcomes. A late or tied sample
  cannot recover.

**Separate SR11 retention case.** Use the fixed 0018 record-retention
notice with synthetic run outcomes. Fail twice with no notice, fail
three times in a row with one notice, then record a fourth failure
with no further notice. A successful run resets the streak without changing
its prior alert, acknowledgement or audit revision. The
third failure of a new streak gives one new notice. An unadmitted
optional threshold or another cleanup route cannot change those
outcomes. The result supplies no permission or proof for the 0059
Dell legacy sweep, and no test starts a timer or removes anything.

**Evidence.** Bind readings, spell, recipient scope, outbox and
receipt identities. Bind 0018 run outcomes and notice counts for the
separate SR11 case. Production disk floor and safe hold interval
remain with the disk-safety research record. The fixture values
confer no production recovery or resource admission.

**Unresolved worker promise.** Use
[0033's shared variant][promise-owner-trace] for all three
owners. SR03 binds REQ-0033-PROMISE-ACCOUNTING and
DES-0033-PROMISE-CHECK to TASK-0033-SAFE-RECOVERY. Join
IT-0019-SCHEDULER-RESTART under REQ-0019-RESOURCE-ADMISSION,
REQ-0019-COORDINATED-RECOVERY and DES-0019-DURABLE-PLATFORM
for the execution update, scoped replay and restore hold.
TASK-0019-DURABLE-PLATFORM owns that effect. Join FI-MB under
MB-09, MB-14 and DES-0078-PROMISE-CONSUMER for the waiting
admission and its Jobs reason. TASK-0078-CORE-RESOURCE-PROOF
uses the exact applicable profiles from TASK-0078-PINNED-PROFILES.
Exercise wrong-attempt proof, partial stop of the correct
scope, a crash before the release commit, a lost reply after its
commit and unavailable current disposition proof on restore. Bind each
observed state and charge count to the original record.
QR-029 qualifies process-scope proof where applicable. QR-081
still requires the actual recovery action, current Cedar
scope and audited evidence. The fixture grants no clearance.

[promise-owner-trace]: ../specs/0033-safe-running.md#promise-accounting-trace

**Cancellation after a native commit.** Join UT-0019-SITE-UNKNOWN
and the existing 0021 ReadLinkEffect result to SR08 dependency
hold and MB-06/MB-08 turn-taking. Use each applicable synthetic
paired native profile. Commit one write and lose the response.
Commit cancellation and expire the sending worker. Reconcile
through the current authorised read. Prove one original effect,
a still-cancelled job and a held dependent. When the receipt
cannot be verified, keep the effect unknown. Exercise a fitting
independent job after its own resource checks. A late response,
resource release, or aging supplies no second send or automatic
undo.

### Phase 5 owner tools and supported profiles

These five catalogue tasks use local synthetic fixtures. Case labels
are proposed names pending owning-spec binding under
[0076](../specs/0076-specification-completeness.md). Future receipts
bind the exact candidate tree and built artefacts, policy, platform,
fixture and tool identities, test and task IDs, observed outcome and
runner identity. Use the existing signed evidence and independent
review route under spec 0003.

#### TASK-0020-SUTRA

**Proposed IT-0020-SUTRA-SCOPED-PLAN.** Use
[0020](../specs/0020-screens-actions-sutra-mcp.md), The runner, Discovery
and help, tests 1/4/8/10 and AT-LOCAL-TRANSPORT.

- **Given** an enrolled owner client on the protected local door,
  one site, a typed read action and an eligible settings plan.
- **When** comparing API and CLI results, request a plan with
  `--plan`. Offer an expired or wrong-site plan, and try to give
  an agent settings confirmation through `--yes` or `--json`.
- **Then** the read and refusal shapes match. Planning produces
  no settings or downstream effect. Expired and mismatched scope
  refuse. A flag cannot replace the owner's live confirmation of
  the exact eligible agent plan in the app or interactive prompt.
  The confirmation is one-use and binds the current owner, token,
  site, policy, plan and expiry. Noninteractive input refuses.
  The never-changeable class refuses even with a confirmation.

**Evidence.** Bind action and schema versions, door, policy and
approved scope, redacted plan, expiry, audit and effect counts. Use
the existing typed door and confirmation checks. Bind the exact
first read and settings fixtures and their transport variants.

#### TASK-0020-LOCAL-MCP

**Proposed IT-0020-MCP-GRANT-REPLAY.** Use 0020 MCP and tests 3/4/6/7,
AT-LOCAL-TRANSPORT, and
[0118](../specs/0118-end-to-end-encryption-gate.md) IT-0118-LOCAL.

- **Given** an owner-started enrolled client with an expiring
  one-site token, two synthetic sites and a permitted draft.
- **When** read or draft within the site, ask for another site's
  records or credentials, keys and audit logs, and try a moderation
  action or the confirm step. Revoke the token before reconnect and
  replay. Try an unenrolled peer, wrong WebView origin and a
  protected request over plain standard input. Recheck the grant
  before retrieving a prior result.
- **Then** only current, scoped reads and drafts succeed. The
  enrolled client cannot approve or confirm an effect. An eligible
  settings change needs the existing exact live owner confirmation.
  Plain standard input and output carry only public help or opaque
  status. The revoked token supplies no authority or replay data.

**Evidence.** Bind installation, peer, site, action and grant versions,
reconnect result, safe refusal codes and absence of a management
network listener. Keep tokens and protected fixture data out of the
review record. Bind Unix socket and Windows pipe separately.

#### TASK-0037-WORKER-FENCE

**Proposed PL-0037-WORKER-STOP.** Use
[0037](../specs/0037-extension-isolation.md) The sandbox, Lifecycle,
EX-04/05/07/13/18, and [0104](../specs/0104-python-scripting-and-sidecar.md)
PY-03/12/17 and PL-PY-01 to 07 for the separate sidecar profile.
QR-029 owns the platform fence qualification.

- **Given** reviewed synthetic packages, scoped grants, resource
  commitments and independently admitted OS fixture profiles.
- **When** exercise denied file, process and network requests against
  the fixture boundary, meet and exceed a measured allowance, and
  withdraw authority or stop while a child operation is pending.
- **Then** the OS and broker enforce their respective limits. The
  denied cases cause no protected effect. Stop closes the owned
  process tree, releases its resources and prevents stale results
  from committing. A platform with missing or failed fence proof
  keeps the affected feature unavailable.

**Evidence.** Bind package, OS fence and broker policy digests, grant
and lease generations, stop and process-tree check, leftover resources
and effect count. Prove Linux and Windows independently for each
offered worker type. Bind Ubuntu 26.04 and Debian 13 separately
for the Linux extension. The admitted Windows 11 x86-64 Python
or render fence has separate evidence. The fixture checks absence
of unauthorised host helpers and ambient authority.

Join the process-tree stop proof to 0033 SR03 and 0078 FI-MB.
Keep the original attempt, fence and resource scope fixed. A
genuine parent exit with an owned fixture member left behind,
or with missing closure proof, leaves the original charge held.
The Jobs reason reflects unresolved release and no dependent
attempt is counted. Verify the entire originally charged scope
under the exact platform fence. QR-029 owns fence proof and
QR-081 owns the current authority to release the charge. Exit
evidence and a synthetic fixture confer no recovery authority.

#### TASK-0004-IN-APP-GUIDE

**Proposed BDD-0004-CORE-HELP-OFFLINE.** Use
[0004](../specs/0004-manual-publishing.md) Content, The manual in the
app and MAN-PLATFORM/MAN-OFFLINE. QR-100 owns the platform and help
proof. Preserve [0030](../specs/0030-install-upgrade-removal.md)
first trust on verified offline media.

- **Given** an exact candidate's packaged help and inventory of
  Phase 5 screens, outcomes and error links. Use made-up content
  and take the fixture offline.
- **When** find the beginner steps for install, restore, removal,
  support, pairing and a provider refusal. Open same-release recovery help
  with the app and engine absent. Offer mismatched help and a
  missing or keyboard-inaccessible page in separate variants.
- **Then** each admitted journey reaches the matching offline
  help for the exact release. Missing or foreign content blocks
  the walkthrough. Recovery help opens without starting the
  installer or engine. Unavailable actions are clearly labelled.
- **Paused update.** Join MAN-OFFLINE with 0030 test 16 and 0031
  IP-40. With the app and engine stopped, use verified guides for
  the previous release and the recorded update. Bind the applicable
  guide to the exact held step and its recorded release. A genuine
  previous-release guide alone or an unproved original record keeps
  the update paused, with no Resume, Rollback, launch or data change.
  The matching guide gives the same safe next step without performing
  it. An isolated restore, if separately offered under IP-30, retains
  its own plan and approval.

- **Recorded guide with a newer bundle available.** Join MAN-OFFLINE
  and 0030 test 16. Keep the paused update on B and take the fixture
  offline. Offer the eligible B guide and verified newer C bundle
  and guide. The local error link opens B's exact held step. Without
  B, C leaves Update paused with no Resume or Rollback. Both branches
  launch nothing and keep the recorded operation and data intact.

- **Offline verifier scope.** Join MAN-OFFLINE and
  IT-0005-DETACHED-TRUST under the applicable QR-100 help and QR-202
  native profile. Offer the exact independently verified current
  guide and package with valid M and R, and the earlier manifest-only
  test verifier. The guide opens as help. Hold current native use
  because the required R reader is unproved. Keep the Linux native
  installer and Windows unsigned wrapper stopped. Only independently
  admitted current verification clears that check.

- **Same-release help for another step.** Join MAN-OFFLINE, 0030
  test 16 and 0031 IP-26. Keep original data and a verified previous
  app that cannot read its format. Offer a genuine page from the
  recorded update's guide for a different recovery step. Refuse
  that recovery match. The exact page explains the format hold.
  Keep Resume, Rollback, previous-app launch and success unavailable.

**Evidence.** Bind package and help digests, versioned links, screen
and action inventory, keyboard and screen-reader results and offline
reachability. Capture each MAN-PLATFORM journey independently,
including a refusal with its precise code and recovery steps.
Follow Getting started's offline bundle and verification steps
with valid M and R but absent or stale public release proof.
Use IT-0005-DETACHED-TRUST for the admitted install or repair
route. The guide alone cannot admit package use. No private Dell
log or proof is needed in the eligible control.
For Ubuntu and Debian, match the guide's failed package handoff
message and verify-again steps to the SUT-832 result from the
same-route IT-0030-VERIFIED-STAGE fixture. Use the exact verified
staged copy for the matching control. Bind each Phase 5 outcome
and profile to actual package evidence.
For Windows, join 0030 test 16 and MAN-OFFLINE. The package and guide
verify while the desktop WebView2 prerequisite is absent. Match
SUT-4103 to the local guidance and show it without the desktop.
Assert no installer launch from help, substitute component load,
network request or startup success. The admitted component control
clears only the prerequisite. SUT-832 still owns failed file proof.
This fixture gives no Pages or release authority.

Join IT-0019-SCHEDULER-RESTART, REQ-0019-RESOURCE-ADMISSION and
REQ-0019-COORDINATED-RECOVERY to 0033 SR03 and 0078 FI-MB for
the release commit boundary. An exact worker-stop proof with
synthetic recovery authority and no durable release still takes
one charge. A committed release followed by a lost reply gives
no repeat credit after a later attempt claims the same amount.
During isolated restore, missing current disposition proof
holds dependent admissions. QR-081 still qualifies the owning
recovery action and exact receipt. Live effects keep their
separate fences.

Join IT-0019-SCHEDULER-RESTART to 0099 N-1378 and 0078 MB-06
and MB-08. Race current pressure recovery and permitted manual
Resume for one committed backfill pause, then replay against
a second pause with the same checkpoint.
A committed old Resume only supplies its authorised history.
An unperformed stale request or cancelled run starts no attempt.
Verify one queue identity and one fair turn at most. Bind both
ordered pause records, the existing scoped resume action, result
and worker fence. QR-081 still qualifies per-class recovery.

#### TASK-0078-PINNED-PROFILES

**Proposed IT-0078-PROFILE-IDENTITY.** Use
[0078](../specs/0078-memory-budget-and-turn-taking.md) Reading the
machine, The budget formula, MB-01/02 and Measurement protocol, with
AGENTS.md section 5.6. Core workload measurements remain under
TASK-0078-CORE-RESOURCE-PROOF.

- **Given** distinct declared 8 GiB and 16 GiB host fixtures and
  each supported OS and matching package profile. Bind workload,
  model and index generations and concurrency assumptions.
- **When** verify the signed profile inventory. Omit hardware or OS,
  change a package or workload identity, and substitute a memory
  figure from the other machine. Separately fail a resource reading.
- **Then** both complete profiles retain distinct identities.
  Missing or different identities cannot reuse signed results.
  A failed reading keeps the last good figure with an unavailable
  state and does not create fresh profile evidence. A stale reading
  gives no new capacity. Below 8 GiB stays subject to QR-013.
- **Replay in FI-MB-02.** Retain the same host and profile. Cross the
  admitted stale boundary and deliver the exact earlier observation
  again. Keep the original observation time and hold. A new successful
  observation with equal values re-enters the ordinary admission
  checks. Record the source and age of both observations and the
  held then eligible control outcomes. Observations release no
  promise and grant no measurement or profile qualification.

**Evidence.** Bind hardware and OS, package build, exact models and
indices, workload, batch sizes, expected budget and headroom,
source reading age and verification result. Bind every supported
tuple and its measurement plan. A fixture profile and a budget
calculation do not replace real combined-peak and latency evidence.

For the 0078 Measurement protocol and 0031 Limits, keep one
complete observed run on the correct pinned host, artefact and
workload, with the window running. Offer a valid native observation
for only the service and workers. Even a low observed peak proves
nothing about the omitted window. Hold the combined-peak result
and keep the observation scoped. A matching full-scope result in
the control is subject to the existing budget and other checks.
Record what the native observer covered, and keep this separate
from run completeness and wrong-profile refusals.

Join BDD-0077-RESOURCE-NONOVERLAP to the matching 0078 host profile
and full combined workload evidence. Keep the same target, model
bytes and budget while offering all pair measurements for an
unmeasured three-way overlap. Refuse its admission before new
work or evidence. Start the existing measured smaller combination
only after checked turn-taking and release of the outgoing
promise. The synthetic case clears no measurement requirement.

For TASK-0078-PINNED-PROFILES and TASK-0078-CORE-RESOURCE-PROOF,
bind the 0078 unresolved resource boundary fixtures to QR-013,
QR-081 and QR-248. A 3 GiB fixture cannot admit a 2 GiB budget
against its 1 GiB ceiling. Two waiting 5 GiB jobs in an 8 GiB
pool must meet the admitted share policy. No passing starvation
claim follows until PT-MB-02 and QM-MB-02 prove their exact
eligibility and ordering. With 16 GiB free, the proposed 4 GiB
disk default fails the proposed 5 GiB minimum. With 4 GiB free,
check an empty interval. Neither case creates a disk promise
without an admitted setting and the 0033 floor. Synthetic
numbers give no host or release admission.

Bind the N-1500 mixed settings case to AT-CF38, MB-02, MB-03
and MB-13 under QR-203. For 16 GiB RAM the effective 4 GiB
headroom leaves a 12 GiB ceiling. The desired 2 GiB headroom
and 13 GiB Custom budget can fit together after restart.
Before the verified restart, no claim may use the desired
larger ceiling. Bind explicitly sized input bytes, the owner
plan, desired and effective versions and the new process's
outcome. Show the pending value as pending. Use existing
admission, lifecycle and protected checks throughout.

Join 0078 MB-15 and FI-MB-01 to 0033 SR04/SR05 under QR-249.
Bind the original promise and current job and process scope.
Use 1,024 MiB with readings one byte below 1,280 MiB, at equality
and one byte above. All use the 0033 over-promise path. Only
the admitted comparison and current scope can give a forced
stop result. A combined reading without individual proof holds
that result. Use QR-081 for measured admission and QR-029 where
platform containment proof applies. Record ordinary stop,
qualified forced stop and hung-worker recovery separately.

For TASK-0078-CORE-RESOURCE-PROOF, join MB-07 and MB-09 to
REQ-0019-RESOURCE-ADMISSION, UT-0019-RESOURCE-BUDGET and
BDD-0019-RESOURCE-REFUSAL, with 0033 SR03 for a temporary wait.
On the pinned host, compare a required measured workload
above the ceiling with an admitted workload that temporarily
lacks room. Check SUT-237 before execution in the first case.
Keep the fitting job waiting with its current reason and
no started attempt in the second. After verified release,
the fitting job still needs the current resource checks
and a fair turn. Bind the scope, measurements, current promises
and attempt counts, with mandatory evaluation and serving
safety preserved. QR-081 owns admission of the measured profile.

**Whole-run evidence.** For item 9 apply the 0078 Measurement
protocol, 0019 full-backfill measurement contract and 0033 SR02
where the admitted recipe requires them. Hold build, exact machine,
OS, profile and declared recipe fixed. Offer valid earlier windows
from incomplete run A and the remaining required windows from
incomplete run B. All expected step IDs appear across the pair.
Require the complete observed scope of one eligible run per
required test. The joined partials retain only their own evidence.
The affected item 9 obligation remains unproved under QR-241.
A complete matching control clears only this measurement obligation.
In an already authorised resume require current proof of the same
original run and each required window. Retain the original test
identity and record the missing or substitution reason. This
fixture supplies no recipe, execution or closure authority.

## Supply-chain evidence for gate transition and release

Bind every row to an exact candidate, protected policy generation,
verification tier and platform. Use the
[toolchain contract](../specs/0003-toolchain-and-check-runner.md),
[framework build rules](../specs/0113-behemoth-developer-tools.md),
[verification rules](../specs/0114-behemoth-verification.md),
[encryption gate](../specs/0118-end-to-end-encryption-gate.md) and
[release sequence](../specs/0005-release-coordination.md).
Keep raw evidence in its approved custody. Forgejo records the exact
task, checks, runs and decisions.

### Inventory comparison

At inspected revision `c21fcb9f52ec`, the tracked inputs are
`Cargo.lock`, `gate/tools.toml`, `gate/catalogue.toml` and
`gate/sbom.cdx.json`. The resolved Bazel, Composer, Angular and
Python lock manifests for the proposed build were absent from the
inspected tracked inputs. Inspect candidate changes separately. To review any
candidate, match both by stable component
identity, version, source and hash or image digest. Include the lock
that supplied each dependency. For example, the inspected inventory
has Cargo references, such as `cargo:anyhow@1.0.104`, and tool
references, such as `tool:rust@1.99.1`. The Forgejo and Ubuntu
container records use the corresponding pinned image digests.
Treat these as static examples of the current inventory shape.
Qualification needs resolved, observed and independently checked
evidence for the exact candidate.

| Evidence boundary | Current inventory and authority | Required proof for the replacement graph or release |
| --- | --- | --- |
| Entry points and policy | Use the current protected Dell runner, check catalogue and tool pins. | Under EX-03 and QR-099, map every current check to its proposed entry point, profile, tier and independently reviewed outcome. Cover Rust, Angular, Python, formal checks, PHP adapters and packaging. Prove reverse-dependency closure for generated inputs, policy, migrations, tools and dependency changes. Missing, reduced or unmapped checks block under 0003 and the waiver rules. |
| Executables, tools and images | `gate/tools.toml` records exact tool versions and SHA-256 hashes or image digests. The checked-in SBOM carries corresponding component rows. | Bind provenance from the official source, licence, exact executable or image bytes, dependency and database inputs where relevant, target platform, and observed use. Cover the resolver, compiler and build rules, actions, scanners, language and proof runtimes, builders and verification executables. Verify before admission and after sealed use. An unpinned tool or host fallback blocks. |
| Full resolution | Start from the current protected locks and tool inventory. | Reconcile all direct, transitive, generated, prebuilt, system and tool inputs in both directions against the pinned graph and lock digests. Record target and host OS, architecture, ABI where relevant, and build, test or runtime scope. Include the adapter and front-end locks, Python environment, package inputs and platform-specific alternatives where applicable. Missing lock entries or private inputs without an admitted route block. Keep Cargo locks and cargo-deny, cargo-audit and cargo-vet until equivalent protected replacements pass. After cutover Cargo locks have no build authority. |
| Licence, vulnerability and SBOM | Apply the AGENTS.md licence policy and existing dependency and image checks. An inventory row is the record to verify. | Bind the scan, accepted vulnerability data, provenance and licence decision to each exact component. Require the SBOM to describe the full resolved set with stable identities, hashes, scope and platform, then reconcile it with observed inputs, build outputs and shipped bytes. Include tools, images and permitted prebuilt artefacts and regenerate on any covered input change. Use the pinned push-tier Grype policy. High or critical fixed image holes block unless the exact #130 historical entry applies. No new entries, remove fixed entries and honour the 9 December 2026 expiry. For 0113 packages, verify the CycloneDX slice, resource hash manifest and each required `unsafe_count`. |
| Packaged scanner result join | [0114 BJ-28](../specs/0114-behemoth-verification.md#requirements) owns the scanner result. Use [0113 BI-20 and BI-25](../specs/0113-behemoth-developer-tools.md#requirements), UT-BI-15/19 and N-1801 for package evidence. | Join 0003's exact resolved component and packaged bytes to IT-0005-RELEASE-APPROVAL. Keep the correct fixture package hashes and substitute validly structured scanner evidence for another revision. Refuse release proof. A current matched observation, including an observed zero, passes only its own count check. |
| Hermetic verification | The sealed Dell route uses read-only pinned inputs and bounded scratch. | Outside the seal, acquire only approved official inputs and verify them for staging. Inside the seal, prove no network, extra devices or hidden dependency fetches. Verify cold or missing cache, tampered input and attempted network or compiler use with the owning refusal evidence. The sole C compiler exception is the sealed Forgejo build on Dell using the exact Alpine package verified against Alpine's signed list. No C or C++ compilation in the product, dependencies or general targets. Admit only separately verified official prebuilt bytes by exact target, licence, scan, hash and SBOM. |
| Hosts, targets and packages | Dell owns private and licensed data, the trusted door and final measurements. Sutradhar ships its native packages and add-ons under their owning contracts. | Qualify each required OS, architecture, runner and package profile. For hosted Windows use only admitted public candidate inputs and a pinned workflow, actions, permissions and image. Bind run, freshness, tool and input digests via the trusted collector outside candidate control. Dell verifies the result and equivalent refusal fence. Keep private fixtures, secrets and release keys on their admitted routes. Verify each supported PHP and hosting profile and the separate Windows first-trust route. |
| Cache and reproducibility | A recorded cache hit or component build has only its exact scope. | Reuse results only for identical admitted inputs, graph, target, policy and environment. Compare package payloads under 0116 with two clean builds on the first pinned runner image and one on a second. EX-08 qualifies declared-input closure and authenticated cold and warm cache equivalence separately. Only authorised CI writes shared cache entries. A matching payload in the required independent runs is reproducibility evidence. A platform exception or missing lane follows the owning gate and gives no inferred pass. |
| Waiver and authority | The current protected gate decides accept or refuse. AGENTS.md section 7.2 owns waivers. | Verify a current owner-signed waiver against the pinned owner key, exact named check, scope and expiry where required. Keep the historical image-hole exception limited to its recorded entries and rules. A platform difference, incomplete inventory, failed result or documentation review conveys no waiver or cutover authority. |

### Binding order

1. Record the current and proposed runner, entry points, check map,
   policy generations, candidate tree, graph, tools, lock digests,
   profiles, component inventory and SBOM. Independently verify actual
   execution scope, including planted refusals. A proposed graph
   can produce comparison evidence while the current gate remains
   authoritative. A missing or partial check gives no pass.
2. The AGENTS.md 7.3 pre-signing phase proof records exact committed
   reviewer names, results and phase scope. It cannot qualify a
   signed release or protected cutover. Submit cutover only through
   the accepted Dell gate under 0003.
   Its signed record binds both policy generations and exact evidence
   after the required independent reviews and owner waiver checks.
   Switch authority once on that record. A partial or inconsistent
   switch keeps the last verified generation or blocks. Remove legacy
   inputs only after their accepted replacement covers them.
3. For release, verify the applicable checks, independent reviews and
   0114 long lane and 0079 floor on the exact candidate tag created
   before the 0005 run. Require the 0118 signed security review. Match the
   SBOM, manifests
   and package observations to the unsigned payload. Verify the 0118
   protection inventory both ways and verify the signed gate result
   for those still-unsigned bytes. This signature attests proof.
   It cannot sign the payload, approve release or create the final tag.
4. Accept only the 0005 scoped release-approval receipt. Verify its
   purpose, principal, lease, run, candidate tag and exact evidence,
   policy and unsigned manifest digests. The authorised release step
   then signs the exact artefacts and verifies signed package contents
   against the approved payload before tagging or publishing. Recheck
   package contents at publication and install. The Windows wrapper
   needs 0030/QR-202 independent graphical digest and key proof
   before launch, then manifest and payload verification at install.
   QR-099 build evidence alone cannot establish first trust. A changed payload
   needs fresh evidence. A lost reply is
   reconciled by its original receipt and any unperformed action still
   needs current authority.

Record missing or mismatched matrix evidence with the exact candidate,
scope, owning check and required remedy. Do not substitute source
inspection, a proposed lock, a package list or a synthetic trace
fixture for the corresponding admitted run and verified receipt.

## Joined security and privacy evidence

Use invented sites, users, content and protected values. Use local
in-process fixtures for outside services and a sealed test runner for
supported operating systems and add-on profiles. The designs below
reuse the owning checks. They assign no test pass, launch permission,
external account access or release authority.

### Authority and the evidence record

Bind each record to the exact candidate and built artefact, platform,
adapter and host profile, fixture and policy digests, site and actor
scope, owning requirement and test IDs, and runner identity. Include
the expected decision, observed boundary, safe result code, and
signed receipt or explicit evidence gap. Use opaque fixture IDs and
object counts outside the protected reviewer bundle. Keep keys, raw
addresses, payloads and personal details out of shared evidence.
Apply [0076](../specs/0076-specification-completeness.md) in both
directions. Every joined result has a trace back to each requirement,
and each requirement names the result and supported variant it needs.

| Boundary | Owning checks and fixture | Required observation |
|---|---|---|
| Collection purpose and current authority | [0021](../specs/0021-forum-contracts-ingestion.md) guest and site-rights cases, and [0120](../specs/0120-opt-in-referral-backlink-discovery.md) IT-0120-GRANT and IT-0120-LEAD. Use two sites, native guest access changes and independent privacy and consent records. Vary current purpose, origin and paths, opt-in, withdrawal and browser privacy choice. | Each route checks guest visibility and its own documented authority. The 0120 lane also requires its independent signed review, exact owner approval and current purpose-specific consent. Public origin alone grants no power to collect, retain or reuse it. An expired or unknown permit stops the generation before payload creation. |
| Local identity, actions and secrets | [0020](../specs/0020-screens-actions-sutra-mcp.md) owns the local door and exact plan confirmation. [0109](../specs/0109-behemoth-security-and-identity.md) BE-13, BE-22, BE-23 and BE-17 and [0118](../specs/0118-end-to-end-encryption-gate.md) IT-0118-LOCAL cover wrong peer, stale authority and missing confirmation. | Unix socket and Windows named pipe use current OS peer and authorisation. An enrolled tool sees only its scoped data. Use one-use exact live owner confirmation for eligible settings in app or interactive prompt. The never-changeable class cannot be changed through the tool, even with a confirmation. Recheck current peer and token at read-back. Plain stdio may expose only permitted public help and status. No credentials, protected key material or private action detail in tool output and public refusal. |
| Site storage and transaction scope | [0108](../specs/0108-behemoth-data-access.md) site context and RLS, BD-08, BD-15, BD-20/21, SUT-971 and SUT-972. Use app and migrator roles, missing context, two site contexts on a reused connection and a restored store. Include migration metadata, current transaction binding, reset and interrupted return. | Database starter verifies forced row-level security on every table and the restricted app role. App cannot migrate, own tables, act as superuser or bypass RLS. Use the DR-148 setter with a separately bound site before any application statement or savepoint. Refuse unbound or failed site setting before application work. Show commit and rollback clearing, reset before pool reuse and close-on-open-transaction. No cross-site read or write. A restored worker proves the same role and policy before serving. |
| Native crash-capture profile at restart | Join 0118 FI-0118-STORE to the same admitted 0030 start profile. Use an earlier sealed absence receipt for a disabled crash sink. In a separate synthetic restart, keep the package and store fixed and vary only the fixture evidence for native capture state or destination. | Refuse the affected start before opening the protected store or exposing its data to that process. The current exact sink proof clears only this check. An earlier valid receipt cannot prove the changed profile. |
| Paired site exchange, local transport and storage | [0021](../specs/0021-forum-contracts-ingestion.md) signed exchange and effect receipt rules. [0118](../specs/0118-end-to-end-encryption-gate.md) IT-0118-LINK, IT-0118-LOCAL, IT-0118-HOST and FI-0118-STORE. Match the inventory to the expected endpoint, durable sinks and allowed metadata. | Service initiates paired traffic. Paired routes use verified TLS 1.3 and current sealed and signed 2.0 contract. Hold an unproved browser-to-origin TLS path behind an intermediary. Local protected channels require owner-only mutual TLS and the bundled DB authenticated loopback. Plaintext stays within admitted endpoint processing. Structured stores may use the approved encrypted volume or filesystem profile. Protected add-on rows seal before storage. Separate key custody or refuse the write. Coverage includes WAL, buffers, queues, archives, exports, temporary bytes and crash material. |
| Active protected transport trust | Join 0109 BE-16, IT-BE-02 and FI-BE-02 with 0118 FI-0118-KEY and N-1768 on each applicable admitted profile. Keep one synthetic connection open. Rotate valid trust as a control. Separately revoke trust before a new protected request and before releasing a staged reply. | Valid rotation preserves eligible in-flight work. Verified revocation denies further protected effects or body delivery on the old connection. Reconcile an already committed effect only through an eligible connection with current authority. No new profile or alternate transport is admitted. |
| Independent publication and ranking gates | [0120](../specs/0120-opt-in-referral-backlink-discovery.md) BDD-0120-CHECK and PL-0120-PRIVACY, plus [0135](../specs/0135-ai-fetches-referrals-and-ranking.md) IT-0135-FETCH, IT-0135-REFERRAL, E2E-0135-RANK and E2E-0135-NATIVE. Give the fixture separate lead, human referral, verified fetch fact and native view records. | A referral lead remains private even after a public link is confirmed. Native results contain only admitted scoped cells and coverage, with current permission checked on render. Human referral data keeps an eligible host only. Suppress small or overlapping cells under the owning policy. Only an eligible verified fetch fact for the exact current public revision may enter the existing ordering gate after relevance and safety. Test once-only weighting at 0 and the 500 basis point ceiling, the 28-day evidence window and the final epoch check. Referral counts, leads, paid research and raw source data confer no rank effect. Withdrawal, expiry or revision mismatch restores the base order. Cross a UTC boundary with one older write pending. Require proved full-day scope before measured zero. Revoke one disclosure generation independently of role and visibility and keep it fenced on replay and restore. |
| Export and egress | 0032 AU-14/15 and 0100/DR-079. Try scope change, erasure or expiry at local CSV/JSON hand-off as well as a provider or workflow reuse. | Deliver only an approved safe projection to the exact current eligible owner recipient. Prior shared copies cannot be recalled. Headline excludes page addresses. Standard and Full are owner-only never-changeable classes, and Full needs the site-text switch. A saved file grants no outside dispatch. |
| Frozen report cursor and prepared response | F-0598, SD-12 and SD-16, FX-0100-W2-AUTHORISED-FROZEN-REPORT and UT-SD-14. Use the app and local MCP boundaries in [0100][report-session-fence] and the [0020 cursor binding][report-cursor-binding]. Hold the master, projection and source proof fixed. Prepare page two under G1, revoke G1 and admit a new session and G2 with the same permitted scope. | Withhold the old page, counts and next cursor at release. Under G2 an authentic G1 cursor gives the existing safe restart notice. A fresh eligible request starts under G2 without inheriting the cursor position. Keep the ordinary source-refresh control and the separate source-withdrawal and erasure holds. Evidence binds the exact session, grant, frozen master, projection and response disposition. |
| Paid provider separation | [0132](../specs/0132-dataforseo-keyword-research.md) IT-0132-REQUEST, IT-0132-SPEND and E2E-0132-ISOLATION plus 0133/DR-118 IT-0133-DISPATCH and E2E-0133-SAFETY. Exercise separate keyword and Coach endpoint intents under one account spend fence. Use fake provider and vault. Vary exact approved payload, destination, current owner confirmation, site, accounting scope and pause. | Only the declared data class reaches the intended recipient after confirmation and reservation. Race simultaneous reservations. Hold each unproved operation with no fallback. Keep results local. No site secrets, raw site text or live ranking write. A lost settlement keeps the opaque spend fence. Erasure removes protected research data while the still-usable account retains only the permitted charge state until settlement or decommission. |
| Direct keyword evidence dates | [0132](../specs/0132-dataforseo-keyword-research.md) UT-0132-EVIDENCE and BDD-0132-COACH. Reuse the same admitted synthetic item in the Coach, local MCP results and prepared brief. Vary fetch, volume and intent dates and include undated difficulty, a monthly zero, omitted data and a same-phrase Search Console series. | Preserve source, date, units, separate freshness warnings and coverage. A numeric difficulty can have an unknown date. A stale snapshot gets its own warning. No metric substitution, transport or spend. |
| Provider error registry and recovery | [0032](../specs/0032-audit-erasure-diagnostics.md) AU-10. Reuse [0133](../specs/0133-dataforseo-live-coach-evidence.md) IT-0133-DISPATCH for SUT-6200 through SUT-6203, BDD-0133-EVIDENCE for SUT-6204 and E2E-0133-SAFETY for SUT-6205 and SUT-6206. Reuse [0136](../specs/0136-matomo-timeline-connector.md) IT-0136-READ for SUT-6300 and SUT-6301, IT-0136-ARCHIVE for SUT-6302 and SUT-6303, IT-0136-SCOPE and IT-0136-LIFE for SUT-6304, and IT-0136-LIFE for SUT-6305 and SUT-6306. | Pair each emitted code's exact synthetic refusal or hold with its own registered owner, safe message and reachable next step in the same release manual. Fail a missing or conflicting mapping. The 0133 uncertain-billing recovery keeps the original reservation and starts no second paid request. The 0136 guidance preserves the exact read-only scope, current grants and bounded retry. Unproved days stay unknown and only an eligible observed day may show as stale. The registry and documentation check supplies no live source, request or approval proof. |

For F-0516, join VAR-0133-W2-ONPAGE-PARSE and
FX-0133-W2-EXACT-ONPAGE-PASSAGE to IT-0133-DISPATCH,
BDD-0133-EVIDENCE and E2E-0133-SAFETY. Keep the 0082 supplier
capability and exact observed target, 0132 spend and 0117
passage authority separate. Only the synthetic matching
fixture yields a scrubbed attributed local passage. An echo-only
reply yields no passage, second attempt or fetch. QR-144 holds
supplier admission.

[report-session-fence]: ../specs/0100-outside-services-and-data-platform.md
[report-cursor-binding]: ../specs/0020-screens-actions-sutra-mcp.md

### Saved keyword evidence in a local brief

Join 0132 VAR-0132-SAVED-BRIEF-EVIDENCE and
FX-0132-SAVED-BRIEF-BINDING to BDD-0132-COACH,
0041 F-0036 BDD-0041-W1-BLOG-BRIEF and 0084 F-0755
UT-0084-PACK. Match the current actor and receiving scope,
original snapshot and row, market and source revision.
Retain source dates, units and missing values. An identical
term cannot substitute a newer or wrong-market row. Withdraw
the keyword grant before result delivery and on replay. The
chosen overlay refuses. An independently eligible request with
an empty selection keeps its existing brief grant and source
checks. The F-0036 brief still needs its admitted own-site source
minimum. No refresh, purchase, second cache or broader evidence
grant.

### Independent stop, restore and erasure check

For F-0106, join 0120 BDD-0120-CHECK and FI-0120-ERASE to the existing
0021 XenForo guest permission proof and 0031 WordPress IP-18. Keep the
landing content revision, consent and approval fixed while guest access
is lost. Before transport, refuse the claim without consuming the
one-off approval or sending a request. After verification, withhold
the referring path on review, display and restore. Use the existing
0120 page-withdrawal cleanup and keep its receipt pending while
proof is incomplete. Prove both native cases in their existing scope.

1. Start with a permitted synthetic generation. Record which site and
   PC stores hold data. Place copies in a pending queue, one claimed
   item, a sealed export awaiting delivery, an archive and a pre-stop
   backup. Record the opaque logical effect, current grant and epoch
   identities. The inventory names every replica and its key purpose.
2. Race withdrawal with queue claim and commit, then restart.
   Check that the local fence takes effect immediately. For 0120,
   require a durable site floor receipt or proof that every issued
   permit has expired before claiming site stop. For 0135, distinguish
   native withdrawal from a service-only withdrawal and prove the
   paired exchange or offline native view expiry. For 0132, place
   pause before hand-off and reconcile any already handed-off charge
   through the owning settlement rule. A late reply cannot reopen a
   retired grant, complete an erased result or supply fresh evidence.
3. Independently restore the old PC and add-on state, one side at a
   time and then together. Repeat the cross-boundary check after an
   upgrade and an admitted rollback. Verify current authenticated
   floors, tombstones, role, scope and available keys before enabling
   any read, route, worker, view or rank result. Missing current proof
   holds availability. Replay cannot resurrect native views, evidence
   or pending proposals. An erasure list applies before backup
   contents are usable. Retained backups follow their owning retention
   and protected restore rules.
4. Reconcile the original inventory against independently observed
   database and file targets, queues, derived data, managed archive
   versions and keys. A [0032](../specs/0032-audit-erasure-diagnostics.md)
   receipt has one allowed disposition for each target with an opaque
   count or safe reason. Use erased, nothing found, waiting with a
   safe reason or failed. Unreachable and unconfirmed stay waiting
   or failed as the owner contract requires. Pre-erasure backups
   expire unchanged and apply erasure before restore exposure.
   Distinguish prior taken files and outside copies from managed
   targets. The named account spend fence keeps its own retention. Keep site-
   authored content
   under its existing moderator authority. Final success needs each
   required removal or rebuild to be proved under its owning contract.
5. Have an independent reviewer verify the signed scope and receipt
   against fixture input, repeats, required stores and allowed
   retention. Repeat on Ubuntu 26.04, Debian 13 and Windows 11,
   XenForo 2.3 and both supported WordPress major versions, for each
   admitted installation and hosting shape. Record refusal for
   unsupported shapes. Reuse AU-04–AU-08, FI-0120-ERASE, E2E-0132-ISOLATION,
   E2E-0135-NATIVE, FI-0118-KEY, E2E-0118-MOVE and PL-0118.
   Missing proof or a missing variant remains an evidence gap.

## Phase 7 protection and evidence

| Task and owning requirement | Layer and case | Given and when | Expected result | Evidence producer and remaining binding |
|---|---|---|---|---|
| TASK-0064-QUALIFICATION-PLATFORM, REQ-0064-FEATURE-INVENTORY-AND-PROVENANCE, REQ-0064-VARIANT-APPLICABILITY | UT-0064-FEATURE-MISSING, UT-0064-DUPLICATE-OWNER, BDD-0064-VARIANT-MATRIX | Remove one approved feature, duplicate its owner and remove one required WordPress variant in separate fixtures | Every affected feature or variant is named and qualification refuses | Scope verifier using the exact candidate feature and variant inventory |
| TASK-0064-QUALIFICATION-PLATFORM, REQ-0064-TRUSTED-QUALIFICATION-EVIDENCE | IT-0064-REPLAY, IT-0064-REPLAY-CHANGE, IT-0064-PILOT-READ, UT-0064-WRONG-PURPOSE, UT-0064-FAILURE-HISTORY and UT-0064-INFRA-RETRY | Replay exact evidence, then change policy, supply a pilot result or use the wrong signing purpose. Use two required cases with a shared check and target to substitute a sibling pass or retry authority. Offer a signed aggregate with no exact case mapping, then a separately authorised batch with both exact results. | Exact replay retains the outcome and new sequence. Each changed or pilot case refuses. Only exact cell outcomes and ordered retry evidence contribute to the aggregate. A sibling pass or permission cannot cover the failed cell. | Trusted verifier and signature log |
| TASK-0057-EVALUATION-PLATFORM, REQ-0057-METRIC-ARITHMETIC and REQ-0057-PROTECTED-RELEASE-GATES | UT-0057-NDCG-EMPTY-RANKS, UT-0057-BM25-BOUNDARY, UT-0057-GATES-INDEPENDENT, BDD-0057-KEEP-NOT-RELEASE | Leave six ranks empty. Try the per-forum threshold and its lower neighbour. Offer a keep-champion decision with a failed gate | Denominators stay fixed and the independent release gate still blocks | Protected evaluator and signed outcome. Bind the reconciled release fixtures under QR-101 |
| TASK-0057-EVALUATION-PLATFORM and TASK-0005-FULL-RELEASE-RUN, DR-152 | IT-0028-FIELD-EVIDENCE, IT-REVIEW-EFFECT, IT-0053-RELEASE-REVIEW-PROJECTION, UT-0057-FIELD-ACCEPTANCE, BDD-0057-FIELD-ACCEPTANCE, IT-0057-FIELD-CUSTODY and IT-0005-RELEASE-APPROVAL | Join committed 0028 records and successful terminal removals through 0053 under concurrent review load. Use complete disjoint 200-decision windows. Bound the range proof across local pages and erase or revoke an eligible source before sealing. Substitute an otherwise valid receiver, run or source signer, or revoke rights after approval. | Preserve review speed and scope limits. Compare only exact integer counts after current bounded proof. Hold wrong, incomplete or revoked evidence even with each synthetic bar passing. An erased local selection produces no aggregate. Lost proof blocks signing and publication. | Qualified 0028 producer and 0053 local window receipts, versioned 0057 aggregate and exact 0005 protected criterion result. Source proof stays local and the independent Dell verifier checks the admitted hand-off. |
| TASK-0077-VERIFIED-CHAMPION, 0077 atomic activation and verified rollback | UT-0077-CONTROL-CAS, IT-0077-QUEUE-RECOVERY, E2E-0077-CHAMPION-CHALLENGER, BDD-0077-ROLLBACK-FAILURE | Verify a promotion and crash after committing its receipt. Retry after restoring the earlier champion. Give a new rollback a genuine earlier success and checks for the same restored and failed manifest digests at a different generation | One transition occurs. A stale decision cannot reuse a later generation. Prior success cannot settle the current rollback or resume admission | Controller, execution store and rollback receipts |
| TASK-0114-LONG-LANE, BJ-21, BJ-30, BJ-31 and BJ-32 | AU-BJ-03, IT-BJ-02, UT-BJ-24 and UT-BJ-25. Matrix execution is under spec 0116 | Change a pinned profile or stop before the required soak finishes | The full candidate cannot pass on the incomplete or changed run. Benchmark evidence stays private | Release runner, compatibility matrix and long-lane manifest |
| TASK-0116-REPRO-MATRIX, LS-27 to LS-29 and TASK-0005-RELEASE-PUBLISH, 0005 proof before approval | IT-LS-05, IT-LS-06 and IT-0005-RELEASE-APPROVAL. Use N-1726 and the exact admitted target matrix | Compare both systems and three builds per target across two pinned images. Hold byte drift, missing targets, changed report, valid proof for another inventory or target and a `release-manifest` used as runner evidence | Changes without an approved difference fail the matrix. An unrun required Windows check gives no pass. Missing or mismatched proof blocks approval, signing, final tag and publication. A matching result clears only the reproducibility prerequisite. Keep the Windows wrapper unsigned | Verified 0003 supervisor bundle, distinct 0005 approval and delivery proof, and exact `H(C)`, `H(I)`, profile and target bindings |
| TASK-0005-FULL-RELEASE-RUN, 0005 release steps 3 to 5 and [0134 warning review](../specs/0134-ci-latency-load-gate.md#verdict-and-summary) | Integration, one exact candidate and tier plan. E2E-0134-TIERS for the separate staging and release warning reviews and E2E-0134-TRUST for 0001 evidence admission | Resume an interrupted release run against its original tag and attempt a shortened soak in a second run. In the load-gate variant, give one valid p95-passing pair two p99 warnings and withhold one review or substitute a review from another tier or run. In one long-tier run let the first p95 breach settle with two later passes. Require both exact p99 reviews for the same scenario in pairs one and three, even when the warning values match. In a separate verifier case keep the same enclosing run and warning key and offer the former receipt’s review after an already admitted replacement with equal values. | Only the exact run can resume. Short or incompatible evidence gives no signature or final tag. Incomplete review holds progression and preserves the numeric p95 pass. The former receipt’s review leaves the replacement warning held and the numeric pass unchanged. Complete accepted warning coverage clears only that tier's review hold. Hold missing or unadmitted load storage and 0007 receipt substitution. A historical baseline for changed driver or runtime calibration gives no load verdict, even with matching labels and in-band p95. The 0007 checks still apply | Gate runner and soak receipts. Bind the run and fault IDs. Bind each warning and accepted independent review to the same candidate, tier, run, scenario and valid pair |

Join BDD-0057-KEEP-NOT-RELEASE to 0099 N-1430 and
FX-0099-W5-COMMITTED-OUTCOME-AND-COVERAGE. Reopen Jobs after restart
with an eligible run read and a withheld current assessment grant.
Reconcile only the authorised history of the exact KEEP_CHAMPION
result. Withhold the now-unavailable assessment fields and evidence
references even if the saved view once showed them. A separately
admitted assessment read shows its own failed criterion with
existing redaction. Keep release held and make no ranking change,
retry, release effect or new permission from either read.

Join 0020 SUT-800 and 0099 AC-JW-02 to the 0077 rollback result.
Lose the ranking service while a current 0030 application rollback
receipt is available. Keep any permitted cached KEEP_CHAMPION shown
as dated history. Separately show that the current ranking result
is unavailable. The application result gives no current ranking
status. On reconnect resolve the original run through its current
scope and 0077 result. Keep inaccessible details withheld and use
[the matching recovery explanation][ranking-offline-help]. None of
these reads starts a rollback or resumes work.

[ranking-offline-help]:
  ../../manual/troubleshooting.md#check-ranking-when-the-engine-is-unavailable

For TASK-0077-VERIFIED-CHAMPION, join IT-0077-QUEUE-RECOVERY and
BDD-0077-QUEUE-REPLAY to 0053 dataset eligibility and 0019
reconciliation. Refuse a delayed 0104 result after committed
cancellation or fence replacement even with valid bytes. Hold
unproved publication. A lost reply after durable publication
returns only the authorised original effect, with current
eligibility needed for artefact use after a correction. Compare
the active digest and generation and retained rollback proof
before and after each recovery. Training emits no control decision.

For REQ-0077-RESOURCE and REQ-0077-CONTROLLER join
BDD-0077-PROMOTE-VERIFIED to the matching 0078 measured profile.
Keep the profile, model and index fixed. Give the eligible weekly
repeat complete paired queries and a passing p95 result. Replace
only its missing challenger peak observation with the genuine
staged measurement from the timed-out attempt. Keep that
observation on its original attempt and hold resource evidence
with KEEP_CHAMPION. The current paired p95 result stays in its
own scope. The champion and rollback pointers do not move. The
matching repeat measurement clears only the resource obligation
when it meets the existing comparison and absolute limits.

Use spec 0015 AT-MR19 at the 0014 applied settings change, alongside
0077's champion-stage map and the 0028 shadow rule. Replay a valid
review-stage receipt after lowering the ceiling while an old job
finishes. Check the next admitted moderator read and distinguish an
earlier shadow result from a new eligible review result when the
ceiling is restored. Use 0014 AT-CF-DELIVERY while the restored review
value is saved and the verified running ceiling is still shadow.
Check the desktop shows saved and effective modes separately and
keeps the 0028 result hidden until current conditions allow review.
Bind the explanation to the [Configuration guide][rank-mode-help].
BDD-0064-TRUSTED-REPLAY rejects a result for the static fixture in
place of this changed-settings case. Neither fixture supplies new
activation or qualification authority.

[rank-mode-help]: ../../manual/configuration.md#check-the-scope-of-a-setting

Join IT-0057-REPLAY-IDEMPOTENT and BDD-0057-RELEASE-EVIDENCE to
0077 weekly allocation and 0064 IT-0064-REPLAY-CHANGE. Fix candidate,
model bytes, exact grades, query identities and limits. After scoring
starts, change only the admitted synthetic evaluation protocol revision
before completion is accepted. Stop and hold the old evaluation under
SUT-854. In a separate release assessment use a valid old-policy pass
alongside the current scoped qualification proof. The named criterion
fails under SUT-855. A matching signed result for the fresh authorised
evaluation clears only its own requirement. Neither path grants
promotion, releases an artefact or draws an extra allocation on retry.

In a distinct BDD-0024-INDEX-MISMATCH branch verify the exact model
file through the existing MF-E12 reinstall and leave the required
index incompatible. Withhold every complete alternate set. Show
file verification and MF-E09 separately in Model Hub. Hold new
suggestions, keep prior eligible review on its original keys and
leave the active pointer untouched. Follow the owner's
[model repair guide][model-file-index-help]. Use the current complete
compatible set as the separate positive admission control.

[model-file-index-help]:
  ../../manual/troubleshooting.md#if-a-model-repair-leaves-suggestions-on-hold

Join IT-0024-ROLLBACK-SET and REQ-0077-ROLLBACK to 0078 profile
identity proof. Resume a rollback on a different host profile with
the same OS, unchanged set and index and a valid measurement for
the old profile only. Hold before serving the restored set or
moving the active pointer. Retain the original measurement in its
own scope. The exact eligible profile control completes only
after current checks and reconciliation. QR-094 still requires
actual rollback workload admission and recovery evidence.

In the exact processor-profile control of IT-0024-ROLLBACK-SET,
keep the restored model, index and 0078 measurement valid with
current graphics offload off. Withhold the current rollback
lease before serving. A currently authorised status read reports
verified inputs separately from the observed serving device
and the unverified rollback. It starts no work. Bind the reason
and observed result to the [owner's processor guide][cpu-restore-help].
Reconcile only the original intent under 0077. The matching
current lease clears only this authority check.

[cpu-restore-help]:
  ../../manual/modules.md#if-a-processor-rollback-is-held

Join E2E-0077-CHAMPION-CHALLENGER and IT-0057-REPLAY-IDEMPOTENT
for a committed weekly evaluation whose reply is lost before
activation. Withdraw guest visibility of one evaluation-only
source while the expected champion generation, manifests and
protocol remain current. Match the original protected allocation
and signed decision. Read-back returns only the permitted history
or safe hold. Offering the authentic decision on its existing route
still fails current source eligibility before the pointer change. Prove
no hidden row, new judgement, allocation or activation receipt.
Keep the eligible serving and rollback sets fixed. A separate
eligible reconciliation reaches the regular activation checks
with the original outcome and effect key.

For the 0077 rollback-priority branch of IT-0077-QUEUE-RECOVERY,
join 0078 FI-MB to 0033 SR03. Hold a rollback which fits only by
refunding an interrupted training worker's unresolved promise.
Use the same eligible set and host and a valid receipt for another
attempt. Retain the charge and the existing protective hold with
no model load or pointer effect. Bind actual release
authority to QR-081 and the exact workload under QR-094. A
matching release clears one promise and supplies no rollback proof.

For REQ-0077-SAFETY-DISABLE join BDD-0077-SAFETY-DISABLE to
`rank.integrity.monthly` and the existing controller checks.
Delay an authentic signed G1 permission-fault report until
G2 serves the same admitted manifest with independently verified
current source eligibility. Keep the report on its original
scope. It cannot disable or quarantine G2 or queue a rollback.
Give the current-generation fault its own evidence in the
control and observe the immediate protective effect under
existing authority. Bind the report's original scope, current
generation and effect or refusal separately. Preserve the
serving source guards. A report supplies no controller approval.

**Current protection proof before the next release effect.** Join
PL-0118 and 0005 IT-0005-RELEASE-APPROVAL with the separate
`TASK-0065-RELEASE-BINDING` receipt. Keep the exact approved payload
and recorded signature. Before the final tag, withhold the protection
result selected by the approved full-gate record, or offer a valid
result with matching candidate and payload hashes but a different
protection inventory. Repeat the substitution after a final tag and
before publication. Hold the next effect in each case and reconcile
only exact earlier receipts. Confirm no repeat signature or tag and
no new publication on refusal. Bind the required and offered inventory
and result identities and the unchanged earlier receipt. Use test
records only and preserve the existing signing and publication roles.

**Unavailable release prerequisites.** Use
IT-0005-RELEASE-APPROVAL under TASK-0005-RELEASE-PUBLISH. Hold
approval if canonical qualification evidence can be parsed and
its fixture signature checked while the admitted 0064 reader is
unavailable. Hold the signing effect if an otherwise eligible
WordPress ZIP is offered for an unapproved byte rewrite. Check
that its frozen bytes stay intact. Keep the approved-record reader
available in a separate fixture and withhold the protected
inventory part-count limit. An equal candidate-supplied number
provides no authority for covered allocation or recursion.
Confirm no final tag or publication in any refusal. Match each
control to its exact admitted reader, detached profile or
protected limit and credit only the corresponding prerequisite.

**Fresh release check recovery.** Use 0010's results test with
IT-0005-RELEASE-APPROVAL under TASK-0005-FULL-RELEASE-RUN.
Withhold a fresh release check and verify a test-signed `cache-entry`
for the same comparable inputs in its own cache scope. Keep the
release cell missing and hold approval before any release-key call.
In the control recover the exact already performed fresh result
from the original unchanged authorised run, without repeating the
check. Match the exact original durable receipt, ordered attempt
and required candidate, check, target and profile under the current
policy and trusted reader. A missing receipt or a valid fresh result
for a separate run with identical inputs keeps the cell unproved.
The fixture makes no release-key request, tag or publication.

| Task and owning requirement | Layer and case | Given and when | Expected result | Evidence producer and remaining binding |
|---|---|---|---|---|
| TASK-0055-METHOD-ADMISSION-PROOF, 0079 count contract | CT-04 and IT-0064-METHOD-FLOOR | Keep total above the floor with one family at 11. In separate cases offer a setting-only duplicate, an unused method, another candidate's evidence and a test-signed historical `MC-` result for the exact candidate and an already qualified computation in the short family. Separately offer a historical result for a synthetic distinct computation with no current 0079 admission. | Family floor, duplicates, unused methods and wrong-candidate evidence independently refuse. The historical result adds no method and the same family still blocks qualification. | Bound method-family proof in the qualified ledger |
| TASK-0005-RELEASE-PUBLISH, 0005 approval receipt | IT-0005-RELEASE-APPROVAL | Pass every gate, then vary the lease, candidate, unsigned payload manifest and authority. In separate cases leave the release long lane incomplete and substitute a valid test-signed Phase 2 `phase-gate-run` with its tier proofs or a passing test-signed nightly campaign on the candidate commit. | Only the exact approved candidate can be signed and published. Both substitutes leave approval, artefact signing and the final tag blocked. Keep each proof valid in its owning phase or campaign. | Protected approval, artefact and publication receipts. Verify full-gate evidence against the protected release task, run, candidate and profile. Supersede Phase 3B before signing, tagging and publication in separate tests. Check the current Phase 3 head at each new effect and reconcile any exact prior effect. |
| TASK-0005-RELEASE-PUBLISH, 0005 and 0076 Phase 6 closure prerequisite, QR-243 | IT-0005-RELEASE-APPROVAL | Exercise the Phase 6 prerequisite independently with the other checks held eligible in the synthetic verifier fixture. Withhold the protected closure reader or proof. Separately offer exact Phase 3 closure plus release qualification. Present a prospective Phase 6 evidence set that omits one applicable variant demonstration while keeping the row count by including a demonstration for another variant. | Each refusal holds approval, artefact signing, final tagging and publication. Existing receipts keep their owning scope. The equal count cannot fill the omitted variant proof. In the conditional admitted-profile branch, approve with current closure proof. Let the preliminary check pass. Withdraw or replace only that proof before the same protected step authorises dispatch. Use a same-inventory replacement and preserve the candidate, release tier and Phase 3 evidence. Hold the new effect, make zero release-key calls and refuse replacement under the old full-gate digest. Reconcile only an exact prior effect. | Exact candidate, current authority, and the missing, superseded or wrong prerequisite. This case supplies no positive Phase 6 acceptance format or proof. |
| TASK-0057-EVALUATION-PLATFORM, 0057 judge policy | IT-0057-SYNTHETIC-JUDGES | Remove a fixture site, maker or required judgement and offer a push receipt for promotion | The affected check refuses. Push gives no promotion authority | Signed synthetic site, protocol and maker inventory |
| TASK-0004-PUBLIC-MANUAL, 0004 publication and QR-245 | End to end, mirror and Pages fixture | Finish the synced main run before the frozen asset arrives. Mirror the exact asset without another main update. Try a changed asset, a candidate preview, a stale attempt and a missing admitted trigger. In the conditional admitted branch, lose the exact deployment reply. Separately prove that the old attempt performed no deploy and fence it before one current retry. Between plan and dispatch advance dev, advance latest to an eligible 1.0.1, or omit a published edition in separate cases. Require a newly verified complete plan. | Hold without an admitted trigger or valid inputs and preserve the published site and latest. An otherwise valid older bundle cannot regress any current edition, dev or latest. After a lost reply, keep the last verified receipt and show the current outcome as unverified. Reconcile only the original exact deployment from its owning read-back. A missing listing or elapsed deadline does not authorise retry. The matching deployment supplies the exact version URL and qualified latest where applicable. | Bind the synced commit, release, frozen manual digest and deploy identity, the verified mirror and Pages read-back. No repeat release, signature or tag follows. This fixture supplies no protected trigger admission. |
| TASK-0005-RELEASE-PUBLISH, 0005 release steps 7 to 10 | End-to-end publication and recovery fixture | Interrupt after the checked manifest is signed and after one publish reply is lost | Resume the same approved release from its evidence. A different candidate cannot move the tag or manual version | Forgejo and mirror read-back. Bind release receipt and state transition IDs |

**Current field rights before dispatch.** Join IT-0057-FIELD-CUSTODY
and IT-0005-RELEASE-APPROVAL. Keep approval, sealed source and
frozen reference fixed. Present an authentic current rights and
erasure result for another window with the same counts. Expect
SUT-855 and no publication. Match both exact original windows
under the existing 0057 custody proof in the control. Keep the
recorded signatures and tag.

**Release venue and target.** Use the publication branch of
IT-0005-RELEASE-APPROVAL under `TASK-0005-RELEASE-PUBLISH`.
Keep the verified candidate, signed bytes, final tag and approval.
Offer a different synthetic target with the same visible repository
and release version, or withhold approved venue proof. Neither
permits dispatch. An earlier signature or tag keeps its own receipt.
In the exact control lose the publication reply. A verified read-back
from the other target leaves the original effect unresolved. The
exact original destination and effect read-back reconciles once.
In the same-run branch keep approval current and lose the reply
after package dispatch. Offer only the verified final-tag receipt
for that exact candidate, run and repository. The tag stays verified
and the package effect stays pending. Record no verified package
publication, launch proof or second dispatch. Only the exact
terminal package read-back closes the original pending effect.
Bind the expected and observed target identities, approved venue
proof, original effect and separate read-back. Count outbound
dispatches in the synthetic fixture. Keep mirror and Pages proof
under their own 0002 and 0004 routes.

**Exact package at publication.** In the same release case, keep
the current protected pending effect and approved venue and target.
Refuse a different same-length package at dispatch after otherwise
valid approval and verification. After an exact dispatch with a
lost reply, hold reconciliation if the correct destination returns
matching metadata but different asset bytes. Match actual length
and digest to the frozen manifest row for the original effect. A
verified tag or mirror metadata still supplies only its own proof.
Use an admitted asset read-back for this case. No new mirror asset
route or Pages authority follows from the fixture.

**Phase 7 closure.** Use 0005's post-publication check under QR-244.
Give the verifier a signed and mirrored `v1.0.0` release. In separate
cases withhold the protected closure profile or current Phase 6 proof,
offer the signed release record alone, substitute a valid Pages
read-back for another frozen manual, leave a milestone task open or
omit a required phase review. Keep the verified publication receipts
and hold closure with no second sign, tag, publish or deploy effect.
Within a separately admitted profile, match the exact current evidence.
Reconcile a lost closure or publication reply only to its own verified
outcome. This fixture supplies no protected closure format or acceptance.

### Candidate identity, frozen manual and package binding

These joined checks cover the exact candidate, frozen manual and
signing boundary. Use disposable
repositories, invented data and test signing identities. Bind proposed
case and variant IDs in the owning specs under 0076 before admission.

#### TASK-0005-RELEASE-CANDIDATE

**Proposed IT-0005-CANDIDATE-RECOVERY.**

**Owner.** [0005](../specs/0005-release-coordination.md) steps 1–4 and
8–10, protected roles and its scratch-repository tests for version
commits, tag authority, exact candidate and interrupted recovery.
Reuse the existing TASK-0005-FULL-RELEASE-RUN trace for the full tier.

- **Given** an eligible clean copy of main with passing scope preflight,
  one authorised release action, proposed version and changelog,
  and the declared commit and push checks for that exact version commit.
- **When** the protected step admits the version commit and creates
  its candidate tag and run. Interrupt before and after the pair is
  durably registered. Lose the reply after registration.
  Reconcile that tag and run, then offer a coding identity's tag push,
  a different commit and the earlier run's evidence after a change.
- **Then** the exact registered attempt resumes with the same run and
  tag. Wrong authority, changed commit or inconsistent read-back
  cannot create or reuse a candidate. A missing half, mismatched pair
  or inconclusive result is held. Confirmed retry creates no duplicate.
  The candidate has no final tag, release asset publication or
  permission to change the manual's latest pointer.

**Durable freeze before accepting evidence.** Keep this registered
candidate tag, run, expected I and C and matching actual unsigned
bytes fixed. Interrupt immediately before the exact freeze is
durable. Reopen once with no authoritative freeze read-back and,
in a separate branch, with only I present in that read-back. Offer
otherwise matching synthetic qualification and encryption evidence
in both branches. Hold before qualification credit or recording
an encryption receipt. Preserve the exact attempt with no signing,
final tag or publication. In the control read back the complete
durable freeze for that same attempt, including I and C, and pass
only this prerequisite under current authority.

**Frozen input before qualification.** Reuse this candidate recovery
fixture and IT-0064-METHOD-FLOOR's exact candidate check. Freeze the
required parts, notice and manual in I and C. Hold those records and
an otherwise valid fixture qualification ledger for that candidate
fixed. Change an actual staged required part before accepting
qualification. A stored I with the expected digest cannot prove the
changed bytes. Fail this check before crediting qualification or
recording a new encryption receipt for the old digest. In a separate
branch omit the required frozen manual and recompute I and C. Refuse
the incomplete inventory even though both new digests match its
bytes. Pass only this input check in the original exact control.
Create no new signature, final tag or publication in either refusal.

Keep the exact profile and all four required parts. Set the
fixture count cap to five. Add the standalone part key
`(manual, FX-Z, z.zip)` with bytes `z` and LF and their digest.
Recompute I and C. Refuse the extra key because the profile does
not admit it. An extra archive member is a separate case.

**Selected packaging profile.** Prepare two independently admitted
fixture profiles with different protected digests and the same
visible target, required part keys, lengths and payload bytes.
Check each profile under its own protected fixture context. Keep
the current attempt's selected profile and authority fixed. Offer
a re-encoded C naming the other profile and recompute its digest.
Compare against the profile selected by the protected release
context. Refuse at that binding before qualification credit,
recording encryption proof or any new release effect. The exact
selected profile and matching C pass only this prerequisite.

In the canonical inventory fixture, the standalone manual is
seven bytes including LF. Keep its actual seven bytes and digest.
Change just its declared part length to six, then separately to
eight. Recompute I and C within the same admitted fixture profile
and current context. Refuse both length mismatches at the part check. With seven
declared bytes the unchanged input passes that check. The outer
inventory digest alone cannot prove the actual part length.

Use the exact admitted protected context and C. In a separate
case make the protected signer registry digest unavailable. Keep
the correct previous registry digest in C and the candidate hint.
Withhold qualification because its protected source is missing.
Restore that exact current protected source in the control.

Link to 0064 IT-0064-REPLAY-CHANGE for the exact ledger ordering.
Keep I and C fixed while the same candidate gains a newer refused
attempt outside the authorised replay. The old signed ledger
cannot qualify that candidate under the newer-attempt freshness
rule. Keep the canonical C bytes and H(C) unchanged in both this
refusal and the exact eligible-ledger control. A ledger supplies
evidence about the frozen candidate after it exists. It contributes
no caller bytes to C.

Count qualification credit and new encryption or release effects
in each refusal. All remain zero. Earlier exact receipts keep
their original scope.

**Owning inventory requirements.** Use the [0005 inventory map][inv-id].
IT-0005-CANDIDATE-RECOVERY binds REQ-0005-CANDIDATE-ATTEMPT,
REQ-0005-CANDIDATE-FREEZE, REQ-0005-PACKAGING-PROFILE-MEMBERSHIP,
REQ-0005-CANDIDATE-TRUSTED-INPUTS and REQ-0005-FROZEN-NOTICE-AND-MANUAL.
Join REQ-0064-TRUSTED-QUALIFICATION-EVIDENCE to
REQ-0005-QUALIFICATION-INPUT-BINDING at the frozen H(C). The declared
part length and qualification-credit cases retain their own results.

The encoding fixtures here and in TASK-0065-RELEASE-BINDING cover
REQ-0005-RELEASE-RECORD-FORMAT, REQ-0005-RELEASE-DECODE-BOUNDS,
REQ-0005-RELEASE-TEXT-IDENTITY, REQ-0005-INVENTORY-LOCATION,
REQ-0005-INVENTORY-CANONICAL-KEYS and REQ-0005-CANDIDATE-PAYLOAD-DIGESTS.
Bind VAR-0005-PART-STANDALONE and VAR-0005-PART-MEMBER to their own
forms and positive and refusal cases. IT-0005-RELEASE-APPROVAL
separately exercises the missing protected bound.

BDD-0005-DETACHED-EXACT and IT-0005-DETACHED-MIXUP verify
REQ-0005-INVENTORY-PART-BYTES and REQ-0005-RELEASE-PATH-SAFETY.
Bind each native path result to its applicable exact QR-202 proof
under TASK-0065-RELEASE-BINDING. Keep 0030 and 0031 native receipts
bound to their own host and action.

[inv-id]:
  ../specs/0005-release-coordination.md#frozen-inventory-requirement-identities

**Receipt.** Bind repository, task and lease IDs, main base and version
commit, version and changelog digest, candidate tag object and resolved
commit, run ID and signed commit and push evidence. Verify the durable
run and tag together. A real release needs that run's full independent
checks, including its supervised 72-hour soak. A simulated interruption
shows recovery behaviour only. Keep failed candidates and their safe
reason. Changes to the tested candidate go through the owning recheck.

#### TASK-0004-RELEASE-MANUAL-ASSET

**Proposed IT-0004-FROZEN-MANUAL-ASSET.**

**Owner.** [0004](../specs/0004-manual-publishing.md) Builds and
previews, Publication, Checks and Keeping private data out. Reuse
MAN-PLATFORM and MAN-OFFLINE for each applicable release profile.

- **Given** one exact candidate tree, both theme captures from the
  admitted profiles, synthetic content and the verified same-release
  help package. Retain the previous frozen manual, if any. Bind the
  isolation proof and exact profile of each Tauri capture.
- **When** the pinned isolated build produces the candidate's manual
  bytes and proposed frozen version asset. Offer a missing or foreign
  capture, an unsafe input, invalid links, a failed keyboard or
  accessibility check and a different version or asset digest.
- **Then** only the verified exact bytes may be recorded as the
  candidate's manual asset. Each bad case holds the asset. A retry
  reads the recorded artefact or leaves it held on mismatch. It cannot
  overwrite a frozen manual, publish a preview or move latest.

**Receipt.** Bind tree and version, pinned build, content and profile
inventory, capture and check results, built bytes and hash, and the
same-release packaged help result. Independently verify install and
recovery help. Dell records the exact published-asset hash. The
qualified release step later attaches that asset. Publishing or
restoring Pages state uses TASK-0004-PUBLIC-MANUAL and the existing
read-back. Before first release the stable version remains unavailable.

**Frozen version read-back.** Use [0004 Publication item
5](../specs/0004-manual-publishing.md#publication) in the existing
TASK-0004-PUBLIC-MANUAL case. After an eligible 1.0.1 deployment points
`latest` at 1.0.1, verify the exact 1.0.0 published URL and original
frozen asset binding in the current deployment manifest. Verify
the served subtree separately from the archive digest. Keep the
old publication receipt. Check `latest` separately against the current
switcher. Offer missing, preview and wrong-version read-backs for
1.0.0. Separately keep the exact public files readable and the
observed deployment fixed. Give only the historical receipt after
it falls outside the admitted current proof. Preserve history,
withhold fresh verification and keep any dependent QR-244 closure
held. Exact current deployment proof is the verification control.
The public URL needs no extra grant. Read
only. Check zero pointer, deploy, sign, tag or publish effects.

**Expected Pages deployment.** Under TASK-0004-PUBLIC-MANUAL and a
separately admitted QR-245 profile, advance A to B after the late
bundle was prepared. Keep the served files, version map, verified
release and mirrored frozen asset identical. The old expected
deployment A cannot use B's current authority. Hold before upload
or deploy with the verified site and receipts intact. A fresh exact
plan binds B and preserves all existing editions and current links.

#### TASK-0065-RELEASE-BINDING

**Proposed IT-0065-EXACT-RELEASE-BINDING.**

**Owner.** [0065](../specs/0065-tijori.md) release signing guards and
purpose tests, [0064](../specs/0064-v1-qualification.md) trusted ledger,
[0005](../specs/0005-release-coordination.md) steps 6–10 and
IT-0005-RELEASE-APPROVAL, and
[0118](../specs/0118-end-to-end-encryption-gate.md) PL-0118.

- **Given** the passing exact candidate and unsigned payload manifest,
  all required platform packages and the frozen manual, a complete
  signed gate bundle, the full 72-hour soak and independent reviews
  for that candidate. Verify the signed PL-0118 result against the
  exact still-unsigned payload inventory before approval. Include
  the Windows wrapper separately. Check the owner-signed lease
  policy and candidate principal, independent review, signing and
  publication roles, and the owner's confirmed signing-log head.
- **When** the test signer records the matching release-manifest
  before release-record and loses one reply. Verify the signed
  package contents against the approved bytes. Repeat with a
  substituted payload, foreign ledger, wrong purpose, revoked key
  or expired lease, proof bound to another candidate, wrong owner
  head or a substituted pending-state or mirror counter.
  In a separate case before the first signature, keep the current
  attempt's qualified ledger, unsigned payload and manifest request.
  Substitute a valid, unexpired test-signed approval from a separately
  qualified attempt with the same commit, version and unsigned
  inventory. Give that attempt a different protected run and
  candidate tag. The `H(I)` values match and the `H(C)` values differ.
  Verify each approval in its own protected context.
- **Then** in the substituted-approval case, the 0005 release step
  refuses the mismatched run and candidate tag before submitting
  a 0065 release-key request. The other approval stays valid in its
  own attempt. Refuse manifest and record signatures and the final
  tag on the substituted approval.
  Isolate the nested C and outer M version mismatch with every
  canonical digest correct and typed fields matching M. Use
  exact test-purpose ledger proof for C. Separately lose the
  committed M reply and offer a later test-purpose ledger for
  the same candidate with identical manifest bytes. The original
  signature remains bound to its original ledger. Neither fault
  yields a new signature, release count, final tag or publication.
  Exact replay reconciles the existing signature and log
  entry without signing or counting twice. A changed or unperformed
  action needs current authority and evidence. A revoked key,
  mismatched package or missing proof blocks the final tag. Treat
  read-back of an existing result and a pending record signature as
  separate steps. An expired or revoked release cannot mint a new
  signature. Count the completed manifest and record once.
  Join the 0064 qualification check to the 0065 log-check branch
  on the exact ledger signature. Keep canonical ledger bytes,
  authenticated purpose and candidate correct. Withhold its selected
  core's log entry, then separately offer an authentic owner head
  covering the pending entry but ending before the exact logged
  ledger signature. Neither produces a new signature or count.
  Hold final tag and publication. Confirm the exact ledger counter
  to clear only this prerequisite.
  The final release task alone may create a final tag, on the same
  commit as the verified candidate. A failed candidate cannot
  displace the previous release. Installed-package rollback still
  meets the protection floor in 0118.

**Exact principal and current approval receipt.** Join
IT-0005-RELEASE-APPROVAL and IT-0065-EXACT-RELEASE-BINDING. A valid
approval-purpose signature made by another enrolled principal
cannot bind the named approver's original lease. With the original
lease still current, check receipt expiry strictly before the
decision time, before first signing and after M commits but
before R. Preserve exact earlier read-back and hold all new
signatures and release effects. Require the separate QR-101 time
and boundary qualification for the real release path.
For the separate publication interleaving keep the exact release
lease, role and bytes. Compare expiry before dispatch with expiry
after an authorised dispatch whose reply was lost. The first
reconciles the existing tag only. The second can reconcile its
exact publisher read-back under current read authority. Count
one historical publication there and no new dispatch in either
case. Keep later mirror and deployment authority separate.

For the separate principal-revocation case under
IT-0005-RELEASE-APPROVAL keep the receipt unexpired. Revoke the named
approval role in the current owner-signed policy after M, R and
the final tag, before publication. Preserve the previously verified
receipts and hold publication. The historical approval verifies in
its original policy scope only. Keep the release key and the field
source eligible.

**Release authority owners.** Use [0005 prerequisite
identities][release-approval-ids] and [0065 signing guard
identities][signing-guard-ids]. IT-0005-RELEASE-APPROVAL binds
protected evidence, current approval, phase and action claims,
proof rechecks and exact earlier-effect reconciliation.
IT-0065-RELEASE-KEY-CUSTODY binds REQ-0065-RELEASE-KEY-ROUTE.
IT-0065-EXACT-RELEASE-BINDING covers the exact pair, qualification
custody, signature-once and owner-head requirements. Use the 0065
release retry and window checks for REQ-0065-RELEASE-PENDING-WINDOW.
For REQ-0065-RELEASE-WAIT-RECHECK, independently withhold current
ledger eligibility while the release waits. The exact qualified
ledger, current owner head and active release key are separate
checks. An unchanged key and head do not carry a stale verdict.

[release-approval-ids]:
  ../specs/0005-release-coordination.md#release-authority-identities
[signing-guard-ids]: ../specs/0065-tijori.md#release-signing-guard-identities

**Owner mirror and policy scope.** Use IT-0065-EXACT-RELEASE-BINDING
under 0065 The relay items 5 and 6. Hold the required mirror counter
for identical bytes from the relay account or a different mirror
location. Advance it only on the independent policy-bound fetch
and exact owner-log comparison. A current policy for another
instance cannot clear Dell's waiting policy or give a new Dell
head confirmation. Require the exact accepted Dell log evidence.

**Receipt.** Join the current policy and approval, candidate and
catalogue digests, unsigned payload and final manifest digests,
platform package identities, frozen manual hash, qualification
ledger and confirmed log head. Verify the separate purpose-signed
manifest and record against their common candidate and ledger.
Keep exact signed runner and independent review identities bound to
the task, candidate, profiles and check outcomes. A role cannot sign
outside its purpose.


**Installer public proof.** Bind IT-0005-DETACHED-TRUST under the
applicable admitted QR-202 consumer profile. M, R and package bytes
remain exact. Withhold the current public proof, then offer a
test-authenticated private Dell ledger for the same release.
Hold both before private access, extraction or native effects.
Only the exact eligible public proof clears that check. Dell's
publication proof keeps its separate source and authority.

**Native path refusal before extraction.** For IT-0005-DETACHED-MIXUP
match the exact admitted parser, target and native path rules.
Use a separately bound signed synthetic container for each
escape or collision. In the collision fixture keep the raw
member names distinct and observe the exact profile's path
refusal before any extraction or native effect. An admitted
early spelling refusal counts only for that profile and case.
Bind QR-202 evidence to the exact packaging profile digest, stage
root, parser and native consumer identities. Require the admitted
relation, escape refusal and either a collision refusal or the
profile's earlier spelling refusal. Include a contained control
with distinct destinations. Hold when that evidence is absent.

**Required archive member identity.** Under IT-0005-DETACHED-MIXUP
verify the test-signed manifest and complete outer ZIP first.
Its inventory has one row for the required member path. The
archive has two members at that path, only one matching both
its length and digest. Run both archive orders and require
refusal before extraction to disk or native handoff. The single
matching member is the control. Join IT-0031-ZIP-EXACT-UPLOAD
only for its separate native host, actor, upload and effect proof.

Spec 0005 owns exact candidate and inventory bytes. Verify the 0065
manifest against that candidate and each artefact it commits to. Use
the [synthetic encoding fixtures](release-encoding-fixtures.md)
for byte and digest checks with test keys. Include the two-row
sort discriminator in TASK-0005-RELEASE-CANDIDATE. An encoder
preserves the expected inventory hash under either input order.
The reader refuses order by length-prefixed target fields while
the individual rows and their byte hashes stay valid.
For a real release,
require the exact signed qualification, evidence, owner approval,
manifest and record before any publication.

For 0005's inventory and scalar decoder cases in the same
fixtures, isolate the exact and just-over count limit, truncated
text that fits its declared bound, an unknown inventory version,
and malformed UTF-8. Assert refusal before covered allocation or
recursion. Treat the two canonically equivalent UTF-8 sequences
as distinct wire text. Refuse a case-changed fixture ID. The 0064
UT-0064-ENCODING result stays scoped to its qualification record.

For the same 0005 inventory fixture, distinguish a valid part key
that resolves to a member from an eligible standalone parent.
The reader must also refuse an absent exact member path even when
an alternate returns identical bytes. Check duplicate part keys
and independently repeated parent and member-path references
from distinct keys. These codec and abstract lookup cases supply
no archive parser qualification or native upload evidence.

## Fan-out history and encrypted compatibility

Bind 0117 REQ-0117-LEDGER and FI-0117-LEDGER to the owner app
approval record, the permitted history projection and the 0030
and 0032 ledger and erasure receipts. Start with storage off and
an eligible active preview. Use the exact grant to write twice.
Change site, fields or retention and require new approval. Race
revocation with a write and an old restore. Confirm the transient
view and the derived history remain distinct. Off-device movement
requires the separate 0038 grant and matching absence proof.

For 0117 REQ-0117-BUDGET and REQ-0117-STATE, join E2E-0117-BUDGET,
QM-0117 and RP-0117 to 0033's promise check and 0078's resource
accounting. Interrupt unload before proof, restart and attempt a
second heavy lease. Hold the affected editor score and keep the first
charge. Use existing human reconciliation and exact teardown proof
before releasing the original charge. The new stage needs current
snapshot, authority and resource proof. Late output gives no result.

For 0117 REQ-0117-EDITOR and BDD-0117-EDITOR, use the 0041
display and copy boundary. Hold an authorised card's Copy at
clipboard dispatch, then switch sites or change the draft revision.
The 0020 scope denial or same-site SUT-3604 leaves the clipboard
unchanged. A fresh eligible card copies only its own current link.
Copy starts no outside request, draft edit or Coach history write.

Bind 0117 REQ-0117-ROUTE and IT-0117-ROUTE to a reviewed cluster
suggestion with complete own-site absence evidence. Before the
candidate Save can reach a destination writer, replace the index
generation or erase its named evidence. Observe the evidence
refusal and remove the stale route. The stand-in writer receives
no call and no plan, draft, job or native effect follows. A new
complete check can offer fresh review. For a changed draft with
the same selected text, use SUT-3604. Hold the positive plan write
and task derivation under QR-240 until the exact private
destination and owner action are bound.

Bind spec 0021 test 32 to IT-0118-LINK. Exercise an admitted sealed
2.0 exchange and refuse 2.1 and 1.x before opening or acknowledging
them. Independently present a valid 1.3 note to the authorised
local upgrade reader and keep its unsupported-major refusal.
An offline upgrade result gives no authority to negotiate a live
envelope. Record the exact operation schema and proof for each
admitted platform and keep caller and owner effects separate.

## Update Tracker and advisory Lab

Start from 0137 REQ-0137-FEED, REQ-0137-TIME, REQ-0137-BOARD and
REQ-0137-SAFE. Bind a current exact JSON incident revision, native
revision evidence, 0137 settling deadline and independent comparable
source coverage before giving the protected 0138 reader authority.
The tracker keeps separate surface and provider panes. A query
projection or synthetic coverage receipt confers no live source
scope. Pair exact 0138 IT-0138-SEALED-SOURCE and UT-0138-WINDOWS with
the tracker acceptance cases. For the scope correction case in
UT-0137-BOARD, bind the incident scope record and matching coverage
to the current revision. A narrower correction holds the comparison
and protected reader when only the broader pane is eligible. Verify
both producers and their fence on site withdrawal, source correction
and old restore. For REQ-0138-COACH-GRADE and
BDD-0138-GRADE-FIRST-INCIDENT, use IT-0137-FEED’s incident
confirmation scope with an already recorded first applicable event.
Its absence holds that grade even when a later event is settled.
Revalidate only the original incident through 0137 and show the exact
waiting or ungradable reason under the current read grant.

Connect REQ-0138-SNAPSHOT, REQ-0138-MATCH and REQ-0138-VALIDATE
to UT-0138-FEATURES, UT-0138-MATCH, UT-0138-CONTRAST and
PRP-0138-STATE. Freeze source, revision, registry and sample before
opening outcomes. Test owner approval and local MCP separately.
Bind the local-only result, grade and coach rule to their exact
verification cases. A Lab receipt is separate from ranking and
release qualification. Route any protected archive through its
explicit storage and erasure contract.

## Scientific experiment source admission

[QR-231](../registers/questions.md) tracks exact own-site profile
qualification under ADR-0026 and DR-144. Use the
[0150 source cases](../specs/0150-experiment-source-and-sensor-gates.md)
for each candidate. Apply the owning
[0040 connector](../specs/0040-search-connectors.md) and
[0034 collector](../specs/0034-analytics.md) boundaries.
Synthetic witness cases provide verification evidence
for the refusal and receipt checks only.

For 0150 REQ-0150-GSC, pair FX-0150-GSC-PAGE-BASIS and
IT-0150-GSC-COVERAGE with 0040 AT-CN24 and AT-CN25. Use the
limit recorded under SR-064 in the [source register](../registers/sources.md).
An authorised report keeps its observed row and marks the missing
page unknown. For the same source, hold the scientific cell under
ERR-0150-CELL-PROOF and the verdict checkpoint under ERR-0148-SOURCE.
The final look and efficacy data stay inaccessible. A local
completeness mark or successful pagination gives no new authority.

For the exact primary measure, join 0150 BDD-0150-THREE-SOURCE
with 0149 IT-0149-FINAL and FX-0149-SOLE-FINAL-LOOK. Use isolated
synthetic source-native cases with identical primary matrices
and different qualified corroborating series. Each case requires
all three current proofs from local synthetic producers. No real
producer or provider request occurs. Compare the primary effect, interval
and registered decision class only. Require each claim and bundle
to bind its own exact source and custody proofs. An unchanged
primary matrix cannot overrule a failed required guardrail. A
same-number corroborating value cannot replace a primary cell
under the original claimed manifest. Refuse before another
efficacy read, and keep the consumed look unchanged.

For the native experiment guardrail stop, use 0094 F-0328
and IT-0094-GUARD under QR-133. Keep the original stop unresolved
through a PC restart after its acknowledgement is lost. The
visible baseline and valid receipts for a different effect or
lease leave its reservation and no-renewal fence in force. A
current scoped read of the exact original receipt and native
version can settle only that stop. Prove no live effect remains
or verify a completed rollback before releasing its footprint. Preserve
later staff edits. This proof gives no 0148 verdict authority.

### Scientific experiment work packages

Bind these verification rows to the matching package in the
[implementation catalogue][science-packages]. Keep synthetic
case evidence, the exact candidate and the producer qualification
separate. Every real final claim requires three current independent
source proofs and the admitted method for that frozen plan. No
row authorises imported or missing data.

| Work package | Existing requirements, variants and checks | Exact result or hold |
|---|---|---|
| TASK-0148-FINAL-LOOK | REQ-0148-GATE and DES-0148-FINAL-LOOK. VAR-0148-RANDOMISED uses VAR-0149-BLOCKED-ITT-DID. VAR-0148-OBSERVATIONAL uses separately qualified VAR-0149-MATCHED-DID or VAR-0149-SYNTHETIC-CONTROL. FX-0148-ONE-LOOK, IT-0148-RELEASE, IT-0148-CORRECT, IT-0149-FINAL, IT-0149-PLACEBOS, BDD-0150-THREE-SOURCE and AT-CN25. Bind PRP-0148-ONE-LOOK and PRP-0148-ADVISORY-ONLY. | Stay under the original deadline, scope and all three source fences. At the deadline the exact eligible fixture claims once. One microsecond later it is Inconclusive. A verified false alert can clear its exact hold only before a claim. Before-claim source replacement consumes no look. After-claim replacement withholds release and any further read without opening a new look. An eligible result has no native or ranking effect. In IT-0148-RELEASE keep source proofs current and withdraw the study executor after staging, before seal. An authorised read sees Aborted. Keep its consumed look, persist the stop fence and refuse result and advisory release after restart or regrant. The former executor gets no protected payload. No source call or further efficacy read follows. |
| TASK-0148-LOCAL-ENTRY | REQ-0148-PREPARE and REQ-0148-LOCAL. DES-0148-ENTRY and DES-0148-MANAGEMENT. VAR-0148-DESKTOP-LINUX, VAR-0148-DESKTOP-WINDOWS and VAR-0148-LOCAL-MCP. FX-0148-START-ENTRYPOINTS, FX-0148-LOCAL-AUTHORITY, BDD-0148-START, IT-0148-LOCAL and E2E-0148-DESKTOP. Apply UT-0148-ERROR-BOUNDARY and BDD-0148-ERROR-RECOVERY to ERR-0148-AUTH and the stale local request. | Each exact, eligible Start entry prepares one draft and its replay returns that draft. An ordinary card or Coach action prepares no experiment. Safe same-site reads and prepares supply no registration or lock. Stale, foreign or revoked scope and privileged MCP actions expose no protected object or effect. |
| TASK-0148-SEAL-FOOTPRINT | REQ-0148-FREEZE and REQ-0148-SCOPE. DES-0148-SEAL and DES-0148-FOOTPRINT. VAR-0148-RANDOMISED and VAR-0148-OBSERVATIONAL follow their own pre-outcome allocation or donor pinning. FX-0148-PRE-EXPOSURE-SEAL, FX-0148-FOOTPRINT-RACE, IT-0148-SEAL, IT-0148-LOCK and FI-0148-CKPT. Bind PRP-0148-FROZEN-PLAN and the footprint part of PRP-0148-SCOPE-FENCE. | Freeze exact revision, assignment and dates before exposure. A real activation without all three current qualified receipts holds before reservation. A competing intersecting reservation returns ERR-0148-PLAN. Changed state or a different request under an existing key cannot repeat assignment, and a legacy or wrong native effect receipt holds before reservation or dispatch. Preserve the stop fence during uncertain reconciliation. |
| TASK-0148-WORK-RECOVERY | REQ-0148-RECOVER and REQ-0148-RESOURCE. DES-0148-CUSTODY and DES-0148-RESOURCE. Exercise each applicable 0148 host and lifecycle variant in IT-0148-RESOURCE, FI-0148-CKPT and E2E-0148-DESKTOP with FX-0148-RECOVERY-FENCES and FX-0148-WORKER-BOUNDS. Bind the recovery part of PRP-0148-SCOPE-FENCE. Replay the exact model transitions only through the admitted Quint Connect path. | An exact retry reconciles only the original effect or returns a bounded failure. A stale or cancelled worker cannot publish, even after budget shrink or a late reply. Hold an unqualified profile or an over-limit plan with the named error. Erasure removes access before stale restore and cleanup follows the original file-ledger scope. Join API-0148-ABORT and API-0148-FINAL-WORK in both commit orders on one eligible PlaceboCheck fixture per run. Abort first persists its original receipt and fence through restart and yields no verdict or advisory receipt. Seal first leaves the original scoped verdict and refuses stale Abort with SUT-803. No new look, native effect or source request follows either race. |
| TASK-0149-BLOCKED-ITT | REQ-0149-ITT, DES-0149-ALLOCATION-CONTRAST, VAR-0149-BLOCKED-ITT-DID, UT-0149-ITT and FX-0149-BLOCK-WEIGHTS | Two unequal synthetic strata give 1/3 in the original arms. An exposure switch cannot reassign membership. A removed or unsupported frozen unit holds the computation. |
| TASK-0149-DECISION-REPLAY | REQ-0149-PLACEBO, REQ-0149-SENSITIVITY, REQ-0149-FINAL and REQ-0149-REPLAY. VAR-0149-BLOCKED-ITT-DID, VAR-0149-MATCHED-DID and VAR-0149-SYNTHETIC-CONTROL. FX-0149-PLACEBO-MARGIN, FX-0149-ROUNDED-MARGIN, FX-0149-SOLE-FINAL-LOOK and FX-0149-EXACT-PAYLOAD-REPLAY. IT-0149-PLACEBOS, UT-0149-SENSITIVITY, IT-0149-FINAL, IT-0149-REPLAY and IT-0148-RELEASE | A failed placebo or insufficient bound cannot yield a qualified class. A stale claim, changed composite or wrong input holds. Exact eligible replay compares canonical numeric bytes without reopening the look or approving a release. |
| TASK-0149-MATCHED-DID | REQ-0149-MATCH, DES-0149-PRE-ONLY-MATCH, VAR-0149-MATCHED-DID, UT-0149-MATCH and FX-0149-TIED-PAIR-ORDER | Select the pinned pairs from tied pre-only inputs. Foreign donors, unsupported trends and unmatched frozen units produce no conditional estimate for release. |
| TASK-0149-SYNTHETIC-CONTROL | REQ-0149-SYNTH, DES-0149-PRE-ONLY-DONOR-GRID, VAR-0149-SYNTHETIC-CONTROL, UT-0149-SYNTH and FX-0149-TIED-DONOR-WEIGHTS | The tied grid chooses the exact canonical weight vector. The boundary admits equality and holds one over. A timeout or a post-outcome input yields no fitted output. |
| `TASK-0034-SCIENCE-DARSHAK` · 0034 producer evidence | REQ-0150-DAR, DES-0150-DAR-CELLS, VAR-0150-DAR-READING-BAND and FX-0150-DAR-HUMAN-BAND. Pair IT-0150-DAR-READER with 0034 AT-AN24 and AT-AN37 to AT-AN40 and FX-0034-W2-ATTENTION-BANDS. | A completed eligible after-midnight visit keeps its original site day. Require a closed day, current classification watermark and the exact current measured-human denominator. An equal-valued denominator from a superseded release holds the cell under ERR-0150-CELL-PROOF. The route exposes no visitor records. |
| `TASK-0040-SCIENCE-GA4` · 0040 producer evidence | REQ-0150-GA4, DES-0150-GA4-CELLS, VAR-0150-GA4-KEY-EVENT and FX-0150-GA4-EVENT-BASIS. Use IT-0150-GA4-KEY-EVENT with 0040 AT-CN29, AT-CN33 and AT-CN35 for connector checks. | With an exact mature event scope and compatible denominator, accept a proved explicit zero. Omit the event row or independently set subjectToThresholding or dataLossFromOtherRow. Each affected cell withholds its value and supplies no phase proof under ERR-0150-CELL-PROOF. The separately authorised partial report keeps its scope. |
| `TASK-0040-SCIENCE-GSC` · 0040 producer evidence | REQ-0150-GSC and REQ-0150-SURFACES, DES-0150-GSC-CELLS and DES-0150-GSC-DIMENSIONS, VAR-0150-GSC-EXACT-PAGE, VAR-0150-GSC-GROUPED-PAGE, VAR-0150-GSC-BRAND and VAR-0150-GSC-MULTIMODAL. Pair FX-0150-GSC-PAGE-BASIS and FX-0150-GSC-SURFACES with IT-0150-GSC-COVERAGE, UT-0150-SURFACE-BRAND and 0040 AT-CN24 and AT-CN25. | Require exact byPage basis and independently proved page-day coverage. Keep the observed report row when another page is omitted. Missing cells stay unknown. A page cursor, final day or a different surface or source generation gives no scientific proof. Apply ERR-0150-CELL-PROOF and the owning ERR-0148-SOURCE hold. |
| `TASK-0150-SCIENCE-GATE` · 0150 admission with 0148 handoff | REQ-0150-API, REQ-0150-GATE, REQ-0150-WINDOW and REQ-0150-CUSTODY, VAR-0150-THREE-COMPARABLE and VAR-0150-THREE-NATIVE. Bind the exact 0150 producer read, phase proof and sealed hand-off interfaces. Use IT-0150-API-SCOPE, BDD-0150-THREE-SOURCE, BDD-0150-WINDOW-TIME, FI-0150-EXACT-REPLAY and PRP-0150-EXACT-PHASE, PRP-0150-NO-IMPUTATION, PRP-0150-ONE-CLAIM and PRP-0150-NO-LATE-RELEASE. Bind 0148 REQ-0148-GATE and FX-0148-ONE-LOOK to IT-0148-RELEASE and IT-0148-CORRECT. | Require one current ordered proof from each producer in the composite. Refuse a missing, duplicated, unqualified or wrong-phase proof. Exercise both claim and source supersession orders under AT-CN25. A pre-claim hold consumes no look. A post-claim supersession closes efficacy access and release while preserving the one consumed look. Healthy FC-32 supplies no phase proof. |
| TASK-0151-CALIBRATION | REQ-0151-CALIBRATION and REQ-0151-PROTECTED-BOUNDARY. VAR-0151-BINARY-PROBABILITY, VAR-0151-CLOSED-INTERVAL, VAR-0151-NUMERIC-POINT and VAR-0151-PROGRAMME. IT-0151-CALIBRATION and IT-0151-PROTECTED-BOUNDARY FX-0151-CALIBRATION and FX-0151-PROTECTED-BOUNDARY. | Hold on missing or ineligible outcomes and retain the original issue. An unresolved outcome cannot become a negative. Apply scope, suppression and cursor checks before releasing comparable aggregates. |
| TASK-0151-DISPLAY | REQ-0151-CONSERVATIVE-DISPLAY, VAR-0151-PROBABILITY-DISPLAY, VAR-0151-INTERVAL-DISPLAY and IT-0151-CONSERVATIVE-DISPLAY FX-0151-CONSERVATIVE-DISPLAY. | Both validation blocks and current approval must pass for the exact future stream. A failed check or withdrawal withholds the transform. The ordinary prediction remains eligible only under its own current rights. |
| TASK-0151-LAWBOOK | REQ-0151-SCOPED-GUIDANCE, REQ-0151-REPEAT-EVIDENCE, REQ-0151-INFORMATION and REQ-0151-PROTECTED-BOUNDARY. BDD-0151-GUIDANCE-REVIEW, UT-0151-REPEAT-EVIDENCE, UT-0151-INFORMATION, IT-0151-PROTECTED-BOUNDARY and IT-0148-LOCAL FX-0151-GUIDANCE-REVIEW, FX-0151-REPEAT-EVIDENCE, FX-0151-INFORMATION and FX-0151-PROTECTED-BOUNDARY. | Foreign, stale or unqualified evidence holds. A current owner may confirm the exact eligible guidance revision once. The scoped shelf and information estimate leave task order, mandatory audits and live ranking unchanged. |
| TASK-0151-REPLAY-CUSTODY | REQ-0151-REPLAY-CUSTODY and REQ-0151-QUIT-ERASURE. VAR-0151-SEALED-RECEIPT and VAR-0151-OFFLINE-NUMERIC-REPLAY. IT-0151-REPLAY-CUSTODY, IT-0151-QUIT-ERASURE, IT-0149-REPLAY and PRP-0151-GENERATION-FENCES FX-0151-REPLAY-CUSTODY and FX-0151-QUIT-ERASURE. | A sealed display emits no recomputation receipt. A changed ordered source, composite, look, authority or custody epoch holds. Keep reads fenced while exact managed deletion is unproved and report any owner download separately. |

Shared error, security, recovery and formal cases keep their owners.
Use the exact platform, profile and case applicability in each spec.
Where a task supplies only an internal record or calculation, the
consumer owns the user-visible journey. Run transport and
isolation checks for every applicable local boundary. A receipt
for synthetic fixtures cannot grant production use.

[science-packages]: implementation-catalogue.md

### Specs 0134, 0135, 0146 and 0147 work packages

Bind these verification rows to the matching package in the
[implementation catalogue][design-pkgs]. Keep synthetic case evidence
and the exact candidate separate. No row authorises imported or missing
data.

| Work package | Existing requirements, variants and checks | Exact result or hold |
|---|---|---|
| TASK-0001-LOAD-NAMESPACE | REQ-0134-TRUST with the namespace declaration in spec 0001. FX receipts for a signed workload, baseline and run. | Stay unverified until spec 0001 declares the namespace and profile. A candidate cannot write it. |
| TASK-0134-LOAD-VERDICT | REQ-0134-P95 and REQ-0134-NOISE. UT-0134-P95 and IT-0134-NOISE. | Only p95 drives the numeric verdict. A runner fault is unverified, never a regression. |
| TASK-0134-LOAD-TIERS | REQ-0134-TRUST and REQ-0134-TIERS. E2E-0134-TRUST and E2E-0134-TIERS, with the 0114 BJ-29 waiver rule (SUT-1416). | A short receipt cannot clear a release. No waiver relaxes a limit at staging or release. |
| TASK-0135-SPOOL-COLLECT | REQ-0135-FETCH. IT-0135-SPOOL and IT-0135-FETCH. | A failed or slow write serves the page and counts a gap. Absent proof is unknown, never zero. |
| TASK-0135-REFERRAL-VIEWS | REQ-0135-REFERRAL, REQ-0135-NATIVE and REQ-0135-INSPECT. IT-0135-REFERRAL, E2E-0135-NATIVE and IT-0135-INSPECT. | Suppressed cells expose no complement. A missing descriptor fails the release check. |
| TASK-0135-RANK-FACT | REQ-0135-RANK. E2E-0135-RANK. | Missing, expired or farmed evidence keeps the base order. Referrals never enter the signal. |
| TASK-0048-ACTION-ADDITIONS | REQ-0147-CATALOG and the amended 0048 tests. UT-0147-CATALOG. | An unnamed signal fails the registry gate. Only a recheck sets Verified. |
| TASK-0146-FORECAST-STORE | REQ-0146-GATE, REQ-0146-SOURCES, REQ-0146-EVENTS and REQ-0146-SCALE. UT-0146-GATE, UT-0146-SOURCES, IT-0146-EVENTS and PF-0146-SCALE. | Broken sanity withholds the grade. A missing source is never zero. |
| TASK-0146-FORECAST-MATHS | REQ-0146-MATHS, REQ-0146-STATS, REQ-0146-LEDGER and REQ-0146-EXAMPLE. UT-0146-MATHS, UT-0146-MATHS2, IT-0146-LEDGER and FX-0146-MEERA. | Every flagging capability meets the planted-null bound. Verdict words say observed, never caused. |
| TASK-0146-BLOCK-SIGNALS | REQ-0146-BLOCKS, REQ-0146-FUNNEL, REQ-0146-POISON and REQ-0146-OFFSTATE. IT-0146-BLOCKS, IT-0146-FUNNEL, IT-0146-POISON and UT-0146-OFFSTATE. | Off until QR-221 is answered. Counts under 5 are never stored. |
| TASK-0146-FORECAST-SAFE | REQ-0146-SAFE, REQ-0146-PY, REQ-0146-OFFLOAD and REQ-0146-REPLAY. IT-0146-SAFE, IT-0146-PY, IT-0146-OFFLOAD, QM-0146-GATE, QM-0146-ERASE, QM-0146-BUDGET and RP-0146. | Offload, Python hooks and block signals are owner-only and off by default. |
| TASK-0147-COMPILER-CORE | REQ-0147-ROLLUP, REQ-0147-EVENTS and REQ-0147-SAFE. UT-0147-ROLLUP, IT-0147-ANGLES, IT-0147-EVENTS and IT-0147-SAFE. | A bad event is rejected whole. No member is dropped. Only registered emitters are accepted. |
| TASK-0147-PROOF-GATES | REQ-0147-EVIDENCE and REQ-0147-GATES. UT-0147-EVIDENCE and IT-0147-GATES with FX-0147-SANITY and FX-0147-ROLLOUT. | No Fund on a broken source except the tracking card. Nothing lifts a hold. |
| TASK-0147-LIFECYCLE-OUTCOME | REQ-0147-LIFECYCLE and REQ-0147-OUTCOME. IT-0147-LIFECYCLE and IT-0147-OUTCOME with FX-0147-COOLDOWN. | A return reopens a card even in cooldown. Outcomes say observed and never zero a missing source. |
| AGENTS.md 7.2 waiver gate (task to be filed at admission) | BJ-29 of spec 0114, DR-159 and SUT-1416. UT-BJ-23 | A waiver expires within 30 days of signing on the pinned signed time, is renewed at most once per check id, and is refused on a future signing time, a backwards clock, two times more than 5 minutes apart, a second renewal, a rename, split or merge with no owner signature, or an id with no signed history |
| TASK-0147-BOARD-EXPORT | REQ-0147-ORDER, REQ-0147-EXPORT, REQ-0147-MCP, REQ-0147-PY, REQ-0147-WALLS, REQ-0147-LIMITS and REQ-0147-REPLAY. UT-0147-ORDER, UT-0147-EXPORT, IT-0147-MCP, IT-0147-PY, UT-0147-GATEWALK, UT-0147-NORANK, UT-0147-CODES, UT-0147-LIMITS and RP-0147. | No tool sets a recheck-only state. No Notion template. Python hooks off by default. |

[design-pkgs]:
  implementation-catalogue.md#design-packages-for-specs-0134-0135-0146-and-0147

### Crawler interruption and durable intent

| Authoritative design | Work and tests | Required result |
|---|---|---|
| [0116 LS-09 and LS-31](../specs/0116-leviathan-scale-storage-and-verification.md#journal-and-recovery), [0082 N-1641](../specs/0082-leviathan-crawler.md#legal-wall-privacy-identity-and-polite-fetching-n-1632-to-n-1646) and [0044 F-0114](../specs/0044-seo-signals-crawler.md) | FI-LS-02, IT-LS-02, FI-LS-03, IT-LS-07, UT-LS-07, UT-0019-FENCE-COMMIT and IT-0044-W2-OWN-CRAWL-SCHEDULE. Use [Wave 5](feature-wave-5.md#acceptance-design-for-n-1695-to-n-1718) FX-0116-W5-RECOVERY-UNCERTAIN-UNFLUSHED-HOLD and the linked N-1641 intent fixture. Use the [Wave 2](feature-wave-2.md) scheduled crawl fixture for trigger equivalence. | Distinguish exact proved-unsent ordinary work, unknown handoff and committed result. Verify the current durable head before dispatch or recovering a result after merge or restore. Count sends separately from local progress. An unknown effect retains its reservation and gap with no resend. A stale or unproved head, failed qualification or revoked grant cannot admit dependent work. Preserve the 0120 and 0131 one-off limits. |

### Audit grid manual CSV recovery

Use [0143 AT-AG-04][audit-csv-design] with 0100 SD-32, SD-33 and the
0118 protection checks for the exact approved local destination.
Lose the reply after a committed synthetic CSV export. Restart,
prove the original scoped ledger operation and destination, and
return the same receipt without a second file. With no proof,
disconnect the destination or replace it with a same-name file on
another drive. The original operation stays unconfirmed, and all
existing bytes stay unchanged. Check current rights and erasure
before exposing any receipt detail or file evidence. Erase an
affected managed CSV while its approved drive is detached.
Withhold the bytes and keep the 0032 target receipt waiting. Check
the exact target and report the proved removal or absence once
that evidence is available. An earlier owner copy remains outside the proof.
Exercise the [owner help][audit-csv-help] on the case. These are outcomes
of the admitted CSV export. ZIP and PDF retain separate admission.

[audit-csv-design]:
  ../specs/0143-audit-grid-findings-and-local-preview.md#ui-and-exports
[audit-csv-help]:
  ../../manual/troubleshooting.md#check-an-unconfirmed-audit-export

### Local Slides report hand-off

| Source and owning design | Work and tests | Required result |
|---|---|---|
| DR-087, [0094 Slides candidate][trace-slides], REQ-0094-EXPORT, DES-0094-FILES and VAR-0094-SLIDES-PPTX | TASK-0094-LOCAL-SLIDES, FX-0094-SLIDES-MANUAL-IMPORT and E2E-0094-EXPORT. Use 0100 local file custody and 0032 and 0118 for scope and protection. | Bind the same frozen master, approved projection and package payload digest through the package inspection and independently observed synthetic manual import. Exercise expiry, erasure, wrong profile, active parts and over-limit refusal. Hold the variant and task derivation until QR-067, QR-132 and 0076 proof is bound. The product and automated fixture send nothing to the target. |

[trace-slides]:
  ../specs/0094-reporting-forecasts-proof-and-experiments.md#local-slides-report

### Live and public-web backlink evidence

| Authoritative design | Work and tests | Required result |
|---|---|---|
| [0152 live crawl](../specs/0152-live-backlink-crawler-profile.md) | REQ-0152-IDENTITY, LIMITS, SCOPE, APPROVAL and PRIVACY. UT-0152-DISPLAY-NAME, IT-0152-NAME, IT-0152-LIMITS, IT-0152-LIMIT-CHANGE, IT-0152-SHARED-WAIT, IT-0152-URL-SCOPE, BDD-0152-PUBLIC-LABEL, E2E-0152-APPROVAL, SEC-0152-RECORD and Quint replay. Share 0014 AT-CF41 and 0116 LS-38. | QR-237 requires exact signed legal and resource evidence and the 0116 matrix on 0030's Ubuntu 26.04 LTS, Debian 13 and Windows 11 profiles. A chosen label cannot change wire identity, opt-outs or enable outside dispatch. |
| F-0273 ordinary named-lead observations | [0046][link-change-owner] remains the change-notice owner. Reuse FX-0046-W2-LINK-STATE-NOTICES and IT-0046-LINK-READOUTS, with 0045 BL06 and the 0152 named-lead qualification under QR-237. | These checks bind dated link-state evidence and separate live admission. File-only evidence and any separately admitted 0154 slice cannot initiate a live check. |
| [0153 public-web index](../specs/0153-public-web-backlink-index.md) | Synthetic capped crawl, IT-0153-FROZEN-CENSUS, IT-0153-DISCOVERY-PROGRESS, IT-0153-REFRESH-OCCURRENCE, IT-0153-COLD-QUERY, IT-0153-WITHDRAW-RESTORE and Quint replay | Correct edges and coverage within one published generation. Match frozen source membership, complete eligible reads and the 0154 coverage witness. New operator topology and discovery scope require the separate QR-238 adoption. |
| [0154 site slices](../specs/0154-backlink-index-site-slices.md) | IT-0154-SLICE-IMPORT, IT-0154-SLICE-IDENTITY, IT-0154-SLICE-PAGE, IT-0154-CENSUS-ZERO, IT-0154-OFFLINE-WITHDRAW, E2E-0154-SLICE-READ | QR-239 proves wire and storage bounds, disclosure, signed offline expiry and withdrawal. Bind the fixed 0065 framing and distinct publisher, custody and verifier signing keys. An otherwise valid signature under another test purpose changes no pointer, floor or result. Lookup and refresh perform no outbound read. Replaying an exact expired package holds its data, keeps the original expiry and preserves an independently eligible successor even with equal payload bytes. |

[link-change-owner]: ../specs/0046-web-link-study.md#link-states
