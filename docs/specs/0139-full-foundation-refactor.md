# 0139: Full foundation refactor

Complete the full refactor before building Behemoth Boot. Build and
qualify the entire framework before starting downstream Sutradhar
implementation. [ADR-0025](../adr/0025-refactor-and-boot-first.md) records
the order. The roadmap owns the phase exits.
[Phase 3 roadmap](../roadmap.md#phase-3-foundation-and-first-product-slice)

## Scope and ownership

Review the whole existing first-party foundation on an exact accepted
revision. Cover product code, platform and developer tools, tests,
interfaces, migrations, build and packaging paths, automation and
operating procedures. Inventory archived components before proposing
carry-over. Record third-party and generated inputs at their ownership,
build and licence boundaries. Keep each component's owning spec, and
follow spec 0076 for acceptance and task derivation.

The refactor prepares existing code and contracts. Phase 3B delivers
the framework and its qualification fixtures. Phase 3C delivers its
first product integration. Phases 0 to 2 supply the repository, checks
and the Tijori signing bootstrap that those checks depend on. Aegis
and forge-sync use their ADR-0019 operations paths.

## Baseline and disposition

1. Freeze the accepted source tree and the verified archive reference.
   Record exact toolchain, dependency and target identities. Keep a
   recoverable copy using the existing custody rules. Note the known
   limitations of the baseline with their owners and evidence.
2. Build an inventory from the full tracked tree, build and package
   graphs, action and job registration, migration history and protected
   deployment inventory. Compare it both ways against the owned
   interfaces and intended delivery. Record shared dependencies,
   transitively affected callers and components that exist in the
   archive only. A name in a plan does not prove code exists.
3. Give each item one disposition: retain, refactor, adapt, replace,
   retire or carry forward as a design. Record the reason, compatibility
   contract, affected callers and tests. Refactoring the whole foundation
   requires a reviewed disposition for every item. Retention requires
   positive evidence that the item fits its owning design.
4. Derive small task slices from the graph and the owning specs. Fix
   dependency cycles and mixed authority at their boundaries before
   moving their callers. Each task names the code and effects it
   reaches, the invariants it preserves and the proof it must produce.
   A proposed behaviour or policy change needs separate owner approval.

## Refactor contracts

| Boundary | Required result |
|---|---|
| Modules and entry points | Give each module one owner and an explicit typed boundary. Check reverse dependencies, generated callers and declared entry points before changing or retiring a path. Existing operations remain usable throughout the migration. |
| Code, dependencies and build | Check all first-party code against the supported stack and rules. Record the build-or-reuse decision, pinned identities, allowed licences and build graph. Remove duplicate paths only after all callers and check coverage have moved. |
| Data and settings | Check schema, migration checksums, actor and site scope, permissions, site isolation, encryption and value defaults. Add versioned migrations under the owning spec. Keep a compatibility plan and prove backup, restore and verifiable recovery before a change that can strand durable state. |
| Jobs and external effects | Preserve event order, approvals, replay keys and effect ownership. Fence old and new writers so only one generation can commit an effect. Reconcile in-flight work before moving its ownership. |
| Security and trust | Preserve signing purposes, authority, key custody, audit evidence, data boundaries and the release floor. Route proposed trust or permission changes through their existing protected approval path. Keep secrets and real site data out of fixtures. |
| Operator behaviour and performance | Compare inputs, outputs, refusals, health, failure recovery and observable budgets on the affected profiles. Use the existing instruction, load, memory and footprint gates. Record any discrepancy at its exact owner and source revision. |

### Per-slice migration and recovery

Record expected behaviour in baseline fixtures before changing a
component. Add a failing test for any approved defect correction.
Run before-and-after checks against the same declared fixtures and
profiles. Where an old path cannot run, document the missing evidence
and use the owning approved contract and independent acceptance proof.

An affected interface, schema or durable job needs an exact transition
plan. It names the old and new readers and writers, version order,
drain and verification points, durable commit point and rollback
boundary. Use the existing owner for activation. On uncertainty, hold
the effect and reconcile the original identity before retrying.
If a rollback would read incompatible state, use the owning verified
restore or forward-recovery route. Prove the route on an interrupted
synthetic transition. Retire a replaced path only after proving all
callers, old data, job claims, documents and recovery references have
been reconciled. Apply the existing archive and deletion authority.

## Phase task coverage

Use the 0076 foundation and framework task scope contract. Check
both directions at exit. In 3A, reconcile each frozen inventory
item and disposition to its exact evidence. A `retain` row needs
positive conformance evidence for that item and its owning design,
even when its bytes did not change. A `carry-forward-as-design`
row needs reviewed evidence for that design disposition. Bind each
result to the frozen inventory, item, owner and disposition. Keep
every item in the inventory check. Archive-only code or design does
not count as accepted running coverage. A referenced task must map
back to every claimed item and affected boundary.

In 3B, derive from applicable source-confirmed, specified rows.
At exit require verified behaviour for each applicable row on its
required profiles, or an approved inapplicability reason under
0081. Reconcile admitted cases, artefacts and reviews back to the
exact row and profile. Group-level fixtures and shared tests cover
only the obligations they prove. An unresolved upstream row,
unowned dependency, missing test or missing platform stays held.
The 3B exit still requires complete scope under QR-004 and RQ-015.

## Evidence records

Record these logical fields under the existing task and signed evidence
contracts. Use owning formats and limits for actual files and receipts.

| Record | Required fields |
|---|---|
| Baseline | Accepted commit and tree, archive reference if used, graph and inventory digests, tool/dependency lock, declared profiles and known limitations |
| Inventory item | Stable component key, exact source or archive locator, owner spec, disposition, dependencies, callers and reachable effects, relevant variants and protected boundaries |
| Phase task | Stage, 0076 task and owner, exact inventory or parity row identities and applicable profile, permitted paths and effects, callers and dependencies, owning requirements and variant IDs with their planned cases, required proof and reviewers. A refactor task also binds its baseline, compatibility and migration plan and rollback boundary |
| Acceptance | Baseline and candidate identity, exact variants, fixture and test identities, observed results, signed gate and review references, and evidence for changed callers, interrupted work and recovery |
| Phase handoff | The 0010 phase gate-run binds stage, exact accepted candidate and predecessor, inventory or parity ledger, complete task and proof references and adopted artefact digests. |

The evidence verifier checks completeness and current identities in
both directions. A duplicate or missing owner, unexplained removal,
invalid signature, missing test, open blocker or stale receipt holds
the affected exit. Record status and results in Forgejo. Reconcile
retries to the original stage and artefact identities.

Use the existing 0010 phase check under Tijori's `gate-run`
purpose in 0065. Bind each handoff to the exact preceding
accepted stage receipt and candidate under current authority.
Refactor admission checks the Phase 2 exit. Boot admission checks
the signed refactor exit. Product admission checks the signed
Boot exit, exact artefacts and consuming platform and profile.
The Phase 7 release key supplies no authority for these steps.

The 0106 first-product-slice demonstration owns the product
journey and its exact scope. The 0010 protected phase profile
must name the canonical owner record and reader for its
`first-product-slice` subject. Admit that proof under current
protected policy with the gate and integration owners and
independent reviews. The observed product result must bind the
actual consuming candidate and profile and the authoritative
result and review references under their own owners. This is
3C exit evidence. Reuse the existing 0076 predecessor and
scope checks for entry and execution.

A 3B task requires the matching signed 3A predecessor and the exact
source-confirmed `specified` row, design, planned variants, fixtures and
applicable dependency receipts under 0076. Its own passing implementation
results belong to parity qualification. A missing variant design or
applicable dependency still holds task entry. Full 3B acceptance requires
the observed result for every required row, variant and profile under 0081.

## Phase exits

### Refactor accepted

Every inventory item has one reviewed disposition. Every in-scope
refactor, migration and retirement is accepted with exact regression,
security, recovery and performance evidence as applicable. Code and
specs agree on active boundaries. No Boot implementation task may
start until this exit passes. This exit does not grant a release or
change the qualification requirements in the other specs.

### Behemoth Boot qualified

Freeze the complete official train and parity inventory under
0081. Resolve QR-004 and RQ-015 before calling coverage complete.
A verified subset gives no proof for unresolved applicable entries.
Implement and test groups PB-A through PB-J, including approved
capabilities Sutradhar does not use. Every applicable row needs
matching behaviour and evidence. Every inapplicable row needs its
approved reason. Verify all supported targets and the applicable
clean-room, dependency, reproducibility, security, failure, resource,
benchmark and long-lane requirements under specs 0081 and 0114 and
the shared gates. Require the 0003 protected Bazel cutover and
0116 platform and reproducibility proofs. Use synthetic host
applications and service fixtures
for end-to-end proof. A missing row, unverified feature, stale upstream
match or partial matrix cannot prove a full framework.

Bind exact Boot artefact digests, graph, pins and verified parity
and gate evidence in the 0010 phase gate-run and its owning proofs.
A waiver for another purpose gives no qualifying proof for a missing
applicable Boot feature.
The first product integration consumes the exact admitted Boot
artefacts. A changed framework candidate needs the applicable refreshed
proof before it can supply the product's prerequisite.

### First product slice accepted

Use [0106's first-slice design][product-demo] and its exact
recorded journey for the existing 0010 `first-product-slice`
subject. The current 3B artefact selection stays a separate
proof. Require the product result for the specified consuming
platform and profile in the protected 3C roster.

In BDD-0139-ORDER, keep the current 3B receipt, correct
`adopted-boot-artefacts` cell, exact candidate and 3C profile.
For a required Debian cell, offer an otherwise valid Ubuntu
product journey with the same application bytes and Unix
transport. Refuse the substituted product evidence and keep
the protected pointer unchanged. The exact matching journey
clears only its evidence requirement. Apply the remaining 3C
gate and review checks. Use synthetic evidence and the existing
0010 record shape and signer.

In a separate BDD-0139-ORDER branch keep the exact current candidate,
3B receipt, adopted Boot cell and applicable 3C test profile. Give
it the required independent review and a valid observed 0014 settings
result from that same profile. Withhold only the observed result of
the selected registered 0020 action and offer the settings result in
its place. The settings and review retain their own evidence credit.
The product subject stays held and the protected pointer is unchanged.
The corresponding observed action with its current scoped Cedar
result can fill only its own proof role. Use the protected reader only
as admitted under QR-251. The evidence evaluation leaves runtime
scope unchanged and executes no action.

[product-demo]:
  0106-behemoth-core-container-and-config.md#first-product-slice-demonstration

## Limits, footprint and failures

Use the existing protected chunk, resource, evidence and release
bounds. This specification introduces no run-time option or public
route. Any refactored public path must meet its existing owning budget.
A missing record, mismatch or failed check blocks the relevant task
or exit and refers to the owning tool's error and recovery contract.

## Requirements and acceptance cases

| Requirement | Planned verification |
|---|---|
| REQ-0139-INVENTORY | IT-0139-INVENTORY proves a full bidirectional inventory on a frozen fixture. Plant a hidden caller, second owner, generated dependency, an unproved archive-only implementation claim and missing disposition. Each creates an exact hold. |
| REQ-0139-COMPATIBILITY | IT-0139-COMPARE runs identical accepted and candidate fixtures, including same-site reads, wrong scope, expired authority, unchanged signing decisions and resource limits. A planted regression fails its owning gate. |
| REQ-0139-RECOVERY | E2E-0139-RECOVER interrupts before and after the durable transition, then retries and restores. Verify the exact fence, one logical effect and reconciliation of pending work. A stale rollback target cannot take over. |
| REQ-0139-ORDER | BDD-0139-ORDER tries Boot work before refactor acceptance and product work during partial Boot qualification. Both are held. Only each exact signed predecessor exit admits the next stage under its own checks. |
| REQ-0139-BOOT-CLOSURE | IT-0139-BOOT-COVERAGE plants an unmatched applicable row, unproved inapplicability, an omitted platform, a stale artefact and a mismatched clean-room record. Each holds product entry. Compare the protected `official-train-inventory` with the `boot-parity-ledger` in an otherwise valid test-signed 3B proposal. Omit an applicable train member and its rows while every remaining row verifies. In a separate case offer candidate release evidence for a required final release. Both hold 3B acceptance and product entry and leave the phase pointer unchanged. Complete matching evidence passes only this coverage check. |

### Exact 3A disposition evidence

Extend IT-0139-INVENTORY with two entries in a frozen inventory. Retain
an unchanged running component and carry forward an archive-only
design. Keep the reviewed inventory, exact item identities, dispositions
and Phase 2 proof fixed. For the retained item, present an otherwise
valid test-signed design-disposition receipt naming that same item and
baseline in place of its conformance result. Separately omit its result.
Refuse both 3A exit proposals and leave the protected phase pointer
unchanged. Preserve the independently supported archive disposition.
With each exact result restored, both entries pass their own inventory
checks. Only the retained component has running coverage. The complete
inventory and all other 3A checks still apply before an exit can pass.

### Exact start-up qualification cells

Extend IT-0139-BOOT-COVERAGE with one source-confirmed PB-B start-up row
that binds BB-12, BB-16, both 0106 migration variants and the owning
`FX-0106-W5-STRICT-MIGRATION-BEFORE-START-EFFECTS` fixture. Use an exact
current test 3A predecessor. At 0076 task entry bind the applicable
0081 source and planned schema and generation inputs. Exercise entry
with all planned cases and no result or generated table from this task's
own implementation. In separate branches omit one variant design or
substitute the source row with one for another release. Each holds entry.

For the 3B coverage check, give the same candidate all required test proof
except one exact start-up result. Substitute in separate runs a valid test
receipt for the same fixture from another candidate, target profile or
migration variant. For the same candidate, also substitute a receipt
whose table digest differs from the selected artefact cell, and omit the
generated resource binding in a separate case. Resolve the reviewed
source rows and local schema through `boot-build-inputs`. Resolve the
generated table digest through `boot-artefact-inventory` for the selected
target and profile. Each mismatch or omission holds acceptance and 3C
entry and leaves the protected pointer unchanged. The matching cell
passes only this check. Use the existing 0010 subjects and proof roster.

### Exact protected build cutover

Extend IT-0139-BOOT-COVERAGE with a current eligible 3A predecessor and
complete 3B proof apart from the 0003 cutover branch. Keep the exact Boot
candidate and `boot-build-inputs` fixed. Offer otherwise valid test
cutover proof for a different graph, build route, target profile or
installed runner generation in separate cases. Then offer the right switch
record without committed activation. Exercise a switch interrupted before
the 0065 policy floor moved and one interrupted after it advanced but
before the rest of the activation could be verified. The earlier route
can retain authority only while its policy stays current. The latter
case holds the affected gate. In all fault cases the 3B exit and 3C
entry hold and the protected phase pointer stays unchanged. The exact
committed control can clear only the cutover prerequisite. Check the 3A
predecessor under its own recorded identity and current trust. At 3B
task entry, require committed activation only if the task depends on
the replacement as its authoritative route. The current gate can still
authorise comparison work on a proposed route under 0003.

### Generated and installed callers

Extend IT-0139-INVENTORY with two synthetic deployment graphs.
For the sync route under 0002 and 0006, bind the generator in
`deploy/forge-sync/install.sh`, timer, service, one-shot command
and durable record owner. Omit the timer-to-service edge while
keeping every tracked path. The missing caller holds the item.

For the door under 0001 and 0062, bind the generated service,
installed pre-receive hook, effective socket mount and listener
binary. Keep the tracked and installed hook hashes identical
while the fixture mount refers to an older service generation.
Hold the incompatible installed edge and name its opaque
identities. Use stand-ins for every runtime effect. The
verified Phase 2 baseline and protected installed identities
are still required for live inventory acceptance.

For that same Door graph, trace the installed listener to the
protected policy and pinned runner used for the exact
[0001 accepted base](0001-door.md#exact-push-evidence). Use the existing
[0003 runner authority](0003-toolchain-and-check-runner.md#who-may-decide)
and [0008](0008-trust-boundaries.md#protected-policy-and-installed-tools).
In a separate fixture keep the hook, mount and listener current.
Keep the accepted-base record and the applicable sealed queue evidence.
Omit just the binding from the used runner to the exact protected
base and selected policy. Hold the unresolved inventory dependency
and name its opaque identities. The queue evidence retains its
execution purpose and supplies no missing trust binding.

## Changes from the source

ADR-0025 changes delivery order. Specs 0076, 0081, 0106 through 0114
and the owning migration and gate specs supply the underlying contracts.
