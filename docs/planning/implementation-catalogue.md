# Implementation catalogue

The [roadmap](../roadmap.md) owns the phase order and phase exit checks.
The [feature register](../specs/feature-register.md) owns the scope and
feature owners. [Spec 0076](../specs/0076-specification-completeness.md)
owns the requirements for admitting a slice to implementation.

This catalogue breaks work into pieces that can be reviewed and tested
on their own. A task identity is a stable label for one outcome. It is
separate from the Forgejo issue number. Forgejo records the active
issue, task owner, dependencies, reviews and evidence.

## Milestones and exits

| Milestone | Starts after | Observable exit |
|---|---|---|
| Phase 0, save and settle | Existing inputs can be read on Dell | Recoverable private archive, reviewed decisions, roadmap and rule book, with owner bootstrap proof |
| Phase 1, Forgejo home | Phase 0 | A passing push reaches main, a failing one is refused, protected rules are checked, and the approved mirror agrees |
| Phase 2, the gate | Accepted Phase 1 exit. Start with the 0012 pre-numbered build steps | Signed full gate run with clean and planted cases for the required checks, languages and tiers. Required Windows evidence is verified on Dell |
| Phase 3A, full refactor | Phase 2 | Exact foundation inventory, accepted refactor, compatibility and recovery proof under 0139 |
| Phase 3B, complete Behemoth Boot | Signed 3A exit | Every applicable parity row and supported profile qualifies under 0081, 0114 and 0139 |
| Phase 3C, first product slice | Signed 3B exit | Desktop, local API, service, store and scheduler work through one recorded scenario using the qualified framework |
| Phase 4, site slices | Phase 3C and Phase 3 closure | XenForo then WordPress each reads, proposes, accepts and undoes a fixture link within its footprint |
| Phase 5, core | Phase 4 | Installation, recovery, storage, controls and manuals cover each admitted platform and workload with measured evidence |
| Phase 6, features | Phase 5 and the exact feature readiness record | Supported features and variants have their own evidence in the roadmap wave order. Deferred and excluded routes stay unavailable |
| Phase 7, version 1 | Phase 6 | Signed qualification, artefacts, manuals and reviews satisfy the release bar |
| Later | Owner-approved placement | No version 1 admission is implied |

## Task records

Each row names an owning design, a dependency or prerequisite, the checks to
design first, the result to accept, and the specific evidence needed to
finish. The [test and traceability catalogue](test-and-traceability.md)
explains how to bind the test cases to the exact requirement. Every
implementation issue also needs the common completion evidence below.

Use the [documentation completeness audit](documentation-completeness-audit.md)
to verify the named tasks and their dependent evidence.

Historical Forgejo issue numbers are matching hints. Pin the catalogue
revision and digest, the exact source repository and the identity of an
authorised private metadata snapshot. Read issue numbers in the hint
cell only. Expand explicit numeric ranges inclusively, preserve the
original qualifiers and compare exact repository and issue identities
in both directions. Keep milestone numbers in their own namespace.

Record identity resolution separately from the relationship. A hint
can be absent, unobserved in the permitted snapshot, or resolved.
Its relationship can be current, parent, dependency, evidence,
historical, superseded or indeterminate. A number supplied by several
rows needs a relationship check for each row. Before confirming a
current binding, compare exact scope, design, variants, authority,
dependencies and required evidence with the authorised issue. Flag a
possible duplicate for review only when two distinct issues claim the
same stable task or scope. The reviewer resolves parent, replacement
and evidence relationships before recording a duplicate.

Keep current binds, work status and exact duplicate decisions in
Forgejo. A read-only comparison changes no issue or milestone.
Historical archive contents stay in their approved custody.

For each task, the definition of done includes the exact baseline and
candidate identities, the named tests with their observed results, the
applicable gate and review receipts, and any affected owner guide and
trace links. Where test-first applies, record the expected failure and
later pass. Owner actions need the owner's recorded confirmation and
a safe receipt. Follow the governing spec for the exact proof and
place execution evidence in Forgejo. A task can be prepared as a
design even when evidence or a design decision is still required.

At most 10 tasks carry `status/ready` at once (AGENTS.md section 10).
This catalogue lists more tasks than that on purpose. Mark a task ready
only from the current phase, in roadmap order, and update the task with
proof or a blocked reason before the session ends.

The earlier estimates of about 3,500 product slices and about 8,000
Behemoth Boot and Leviathan slices help plan review capacity. The
catalogue derives its entries from actual behaviours and dependencies.
Estimates cannot stand in for an entry or a release requirement.

## Phase 0 and 1 foundation slices

| Task | Outcome and owning design | Depends on | Checks and acceptance | Specific done evidence | Forgejo export hints |
|---|---|---|---|---|---|
| TASK-0009-PRIVATE-ARCHIVE | Preserve known source families in the private archive, [0009](../specs/0009-source-class-inputs.md), roadmap 0.1 | Read access to the earlier work | Verify the manifest and restore several allowed input kinds to scratch. An omitted or altered file fails verification | Private manifest, restore report and approving review | Roadmap 0.1 |
| TASK-0076-BASELINE-DESIGN | Seal the reviewed decision, roadmap and rule set under [0076](../specs/0076-specification-completeness.md), roadmap 0.2 | Private archive and recorded decisions | Compare each active baseline decision to the design and find planted stale terms and duplicate owners | Exact reviewed baseline and decision-to-design inventory | #640 |
| TASK-0062-OWNER-BOOTSTRAP | Initialise the Forgejo owner and the allowed developer client under [0062](../specs/0062-forgejo-server.md), roadmap 0.3–0.4 | Owner access and pinned client | Owner confirms authorised access. Missing credentials block and no log or evidence file contains a password or token | Owner-safe receipt, exact client pin and redacted refusal proof | Roadmap 0.3–0.4 |
| TASK-0062-HOME | Create the Forgejo repository and publish its initial main through the admitted bootstrap route, [0062](../specs/0062-forgejo-server.md), roadmap 1.1 | Owner bootstrap | Read back the exact expected main. Refuse an unauthorised repository or actor | Initial ref, repository identity and write receipt | #10 |
| TASK-0060-DELL-RUNNER | Admit the pinned Forgejo Actions runner, [0060](../specs/0060-runner.md), roadmap 1.2 | Forgejo home | An approved runner starts the fixture job. Refuse a wrong pin or unapproved runner | Runner identity, refusal and job receipt | #1 |
| TASK-0006-REF-GUARD | Enforce the approved reference policy, [0006](../specs/0006-main-only-publication.md), roadmap 1.3 | Forgejo home | Allow the admitted main update. Refuse forbidden branch, non-fast-forward and deletion requests on a fixture repository | Ref decisions and unchanged rejected refs | #2 |
| TASK-0001-DOOR-GATE | Require push evidence at Forgejo's door, [0001](../specs/0001-door.md), roadmap 1.4 | Reference guard, pinned runner and currently admitted check profile | Exercise the Phase 1 bootstrap route of AGENTS.md section 7.3 with every then-required check. Refuse failed or wrong-tree evidence. One passing candidate advances main once | Bound bootstrap ref receipts. TASK-0003-PHASE-PROOF establishes the later full gate | #3 |
| TASK-0062-PROTECTED-CHECKSUM | Detect drift in protected Forgejo and gate inputs, [0062](../specs/0062-forgejo-server.md), roadmap 1.5 | Forgejo home | A planted out-of-band change opens one visible fault. A reviewed update retains the new trusted fingerprint | Signed fingerprint record, drift and replay evidence | #4 |
| TASK-0062-WORK-LIST | Set up milestone and label identities, [0062](../specs/0062-forgejo-server.md), roadmap 1.6 | Forgejo home | Reconcile each roadmap phase and label. A retry leaves the same identities and does not duplicate items | Exact mapping and read-back after replay | #11 |
| TASK-0001-HEAVY-QUEUE | Enforce Dell heavy-job turn-taking, [0059](../specs/0059-pankti.md) and [0010](../specs/0010-coding-harness.md#heavy-job-control-on-dell), roadmap 1.7 | Pinned Dell worker and exact queue and scope observations | Two jobs occupy places, a third waits, and a timed run holds both places. Plant a lost queue holder with a live child in the evidence fixture. A freed lock gives no protected admission proof. Recheck both places before admitting timed work | Exact queue, holder, owned unit, verified stop or absence of its work or hold, and timing receipts on Dell | #5 |
| TASK-0076-CARRY-OVER | Classify the archived tasks and features once, [0076](../specs/0076-specification-completeness.md), roadmap 1.8 | Work-list identities and source archive | Compare source identities with the crosswalk. A repeated owner, omitted item or alias without a reason fails | Reconciled carry-over register and source coverage evidence | #6, #640 |
| TASK-0002-MIRROR-REPORTING | Set up GitHub private vulnerability reporting, [0002](../specs/0002-forge-sync.md) and roadmap 1.10 | Owner access to the mirror | Authorised private reporting is available. A private sample does not appear on the public issue list | Redacted read-back of the approved setting | #8 |
| TASK-0002-SYNC-AUTHORITY | Verify the repository-only fine-grained token for the mirror, [0002](../specs/0002-forge-sync.md) and AGENTS.md section 9.1, roadmap 1.11 | Owner action | A scoped credential can reach only the configured repository from Dell. An overbroad credential or direct job use is refused | Redacted owner and permission receipt | #9 covers the earlier App proposal |
| TASK-0006-MIRROR-RULES | Reconcile the mirror's protected refs and settings, [0006](../specs/0006-main-only-publication.md), roadmap 1.3 and 1.9 | Owner settings authority | Refuse independent writes outside the approved sync route. Read back the exact rules | Mirror rule and refusal receipt | #76 |
| TASK-0002-SYNC | Mirror the allowed Forgejo changes to GitHub, [0002](../specs/0002-forge-sync.md), roadmap 1.9 | Sync authority and door gate | Replay an update after a lost response. One mirror change results. A private field or stalled campaign does not leave Dell | Bounded receipt, field allow-list evidence and ref comparison | #7 |

## Phase 2 gate slices

Prepare the test-first checker and the Rust safety and fixture work
in [0012's Build order](../specs/0012-ganit-evolve.md#build-order)
before the numbered work. Use the [early gate cases][early-gate-cases]
to derive each exact slice under 0076. The research, sealed trial
and source receipt rows below keep their own inputs and evidence.
Enable receipt enforcement only at AGENTS.md section 7.3's
threshold. The signed full gate run belongs to the Phase 2 exit.

[early-gate-cases]: test-and-traceability.md#phase-2-early-gate-handoff

| Task | Outcome and owning design | Depends on | Checks and acceptance | Specific done evidence | Forgejo export hints |
|---|---|---|---|---|---|
| TASK-0013-RESEARCH-RECORD | Admit exact research cards before measured source work, [0013](../specs/0013-research-rules.md) | Pinned research sources | Refuse wrong version, out-of-scope file and unsigned evidence. An accepted card binds one exact scope | Card and refusal receipts with no private source data | #596, #157 |
| TASK-0012-SEALED-TRIAL | Run the sealed research and speed trial route, [0012](../specs/0012-ganit-evolve.md), item 6 | Research record and heavy queue | Seal trial inputs, interrupt a worker, and verify exact repeat behaviour. An unsealed run gives no receipt | Bound input, worker and result digests | #69, #594 |
| TASK-0012-SOURCE-RECEIPT | Bind each covered source change to its card and speed evidence, [0012](../specs/0012-ganit-evolve.md), item 6 | Sealed trial and signing admission where required | Swap candidate or research card and refuse the receipt. Accept the exact bound one | Gate receipts for admitted and planted cases | #157, #595 |
| TASK-0003-SEALED-RUN | Carry over the trusted sealed runner, [0003](../specs/0003-toolchain-and-check-runner.md), item 1 | Applicable TASK-0001-HEAVY-QUEUE proof for the existing Dell route and current pins | Deny network and protected inputs. Refuse changed candidate bytes, unknown checks and a partial result | Exact candidate, refusal probe and verified result | #39, #101 |
| TASK-0003-CHECK-MAP | Bind every required check to tier, language and planted case, [0003](../specs/0003-toolchain-and-check-runner.md), item 2 | Sealed runner | Omit or duplicate one expected check and refuse the record. Verify an inapplicable check carries evidence of its scope | Full requirement-to-check map and exact refusal receipts. Include 0003 WordPress Plugin Check at commit, push and release, with synthetic proof before product input | #17, #19, #129, #160 |
| TASK-0003-PINNED-INPUTS | Verify tool and dependency bytes in the build and SBOM, [0003](../specs/0003-toolchain-and-check-runner.md), item 3 | Sealed runner | Replace a tool or omit a transitive dependency, licence or image digest. Each gate attempt fails | Hashes, licence, vulnerability and vetting evidence with a matching inventory | #14, #21, #130 |
| TASK-0011-RULE-MATRIX | Apply style and engineering rules to supported languages, [0011](../specs/0011-engineering-standard.md), item 4 | Check map | Plant one example for each admitted rule and verify the exact refusal. The clean case passes | Rule-to-check evidence and independently reviewed matrix | #50, #67 |
| TASK-0061-COVERAGE-PROOF | Check strict-core decision and mutation evidence, [0061](../specs/0061-nirikshak.md), item 5 | Rule matrix, exact strict-core scope | Hand-work conditional cases, leave one killable mutant alive and offer weak equivalent-mutant proof. Refuse the defects | Reviewed denominators and measured clean and planted reports | #68, #135 |
| TASK-0007-MEASURE-SPEED | Measure the required hot paths and reference baselines, [0007](../specs/0007-performance-and-engineering-gates.md), item 7 | Pinned input and sealed queue | Exercise a planted slowdown, changed baseline, hot-loop allocation and a missing reference. Match the owning tier rules | Before and after profiles, receipts and comparison evidence | #139, #158 |
| TASK-0065-REVIEW-WAIVER-SIGN | Sign and verify reviews and owner waivers by purpose, [0065](../specs/0065-tijori.md), item 8 | Purpose schema and fixture keys for the build. TASK-0065-OWNER-KEY for live waiver acceptance | Reject wrong signer, purpose, expired waiver and changed tree. Verify the exact allowed record | Purpose-specific clean and planted results. Live authority also needs its separately verified owner key | #167, #446 |
| TASK-0065-RUN-SITE-SIGN | Sign gate evidence and signed-site contract records, [0065](../specs/0065-tijori.md), item 8 | Purpose-specific signing | Refuse a replay or mismatched tree, site or request identity. Verify an allowed receipt against the correct purpose | Signature verification and replay records | #167, #447 |
| TASK-0065-RELEASE-KEY | Verify release-key custody and purpose refusals, [0065](../specs/0065-tijori.md) | Signing core and applicable custody policy and test-key profile | Verify key identity and refuse private-key export, generic signing and wrong purpose or caller. Positive test-key manifest signing belongs to Phase 5. Real release signing requires Phase 7 authority | Key identity, custody record and exact refusal evidence. No accepted release signature is required for Phase 2 | #448 |
| TASK-0003-PROJECT-GUARDS | Enforce the project rules and reject stale or unsafe documentation, [0003](../specs/0003-toolchain-and-check-runner.md), item 9 | Check map and review signing | Plant illegal file types, status text, bad review evidence, forbidden receipts and unsafe waivers. Required guards refuse them | Guard-to-plant matrix and signed full candidate result | #15, #18, #46, #59 |
| TASK-0001-COMMIT-HOOK | Run the Rust commit entry and refuse missing gate inputs, [0001](../specs/0001-door.md), item 10 | Trusted sealed run | A test commit with a missing tool or wrong receipt fails. A clean fixture succeeds with the same checked tree | Local hook receipt and unchanged rejected refs | #188 and #191 are historical |
| TASK-0003-WINDOWS-GATE | Verify the hosted Windows result for the current Phase 2 gate on Dell, [0003](../specs/0003-toolchain-and-check-runner.md#the-locked-down-box), items 3 and 11 | Current protected gate, pinned inputs and equivalent hosted refusal fence | Refuse wrong tree, workflow, profile or policy, missing checks, unsigned evidence and a result from the replacement graph. An exact eligible current-route result satisfies only its Phase 2 cell. Full EX-03/QR-099 cutover proof belongs to 3B | Exact current runner, workflow, check and isolation evidence bound to the Forgejo tree and verified on Dell | #168 CI parent. Keep QR-099 cutover proof separate. |
| TASK-0003-MIRROR-CI | Re-run the required mirror checks, [0003](../specs/0003-toolchain-and-check-runner.md) and AGENTS.md section 9.3, item 11 | Approved sync, check map and current Phase 2 hosted Windows scope under TASK-0003-WINDOWS-GATE | A mirrored candidate runs the current protected CI profile, including instruction and allocation counts. Dell supplies timings and its reference race. A pull request cannot start this route | Exact mirror run and Dell-only evidence comparison | #168 |
| TASK-0065-OWNER-KEY | Establish the owner waiver key without moving its private bytes, [0065](../specs/0065-tijori.md), item 12 | Owner action | Verify the recorded public key on Dell using a signed fixture. An alternate key fails | Owner confirmation and public fingerprint only | #169 |
| TASK-0003-PHASE-PROOF | Reconcile the complete signed Phase 2 evidence, [0003](../specs/0003-toolchain-and-check-runner.md), [0010](../specs/0010-coding-harness.md) and the [0134](../specs/0134-ci-latency-load-gate.md) bootstrap under DR-121 | Every applicable Phase 2 slice | Run the named clean and planted tiers. Exercise LOAD-0134-DOOR with a synthetic signed envelope and reject a wrong candidate, tier or profile. The 0134 product workloads wait for their owning entry points. Missing, late, wrong-tree and partial evidence block | Exact signed full run, bootstrap receipt and independent phase review | Roadmap Phase 2 exit |

Each larger milestone uses the same task shape. The phase exits above
do not replace a task or test record for any behaviour.

## Phase 3 foundation tasks

Spec 0139 owns the 3A inventory and refactor proof. Derive each work
item from the reviewed inventory. Bind each derived task to its exact
frozen item, disposition, reviewed inventory and 0139's
IT-0139-INVENTORY case. Use the case as planned proof before build
and bind its exact result at acceptance. Verify retained running
components against their owning contract and keep archive design
proof limited to its own disposition. Use FX-0139-ENTRY-3A for
the separate predecessor check. Stage 3B takes its work items from
all applicable rows in the 0081 parity ledger and groups PB-A to PB-J.
Verify the signed Phase 2 exit before admitting any 3A task. Verify
the signed 3A exit and its accepted refactor identity before any
3B task. Use the 0139 handoffs and 0076 task admission contract.

### Phase 3B framework work packages

Use the group map below to route every applicable frozen 0081 row to
its owning design. Allocate implementation tasks from exact rows.
Derive a 0076 task only
when the exact release and ledger row, owner, independent behaviour
record, dependencies, covered variant and acceptance case are known.
Bind the signed 3A result and its adopted refactor as the prerequisite.
One task may cover an approved small group of compatible rows if its
requirements, fixtures and proofs trace back to each row and variant.
An admitted `specified` row can enter work. `matched` and `verified`
remain outputs of the 0081 process.

| Group and owner | Scope to account for | Example acceptance anchor. Use the row's exact contract tests |
|---|---|---|
| PB-A [0081](../specs/0081-behemoth-boot-parity.md) | Official source scout and release train, ledger state, clean-room handoff, lockstep and compatibility migration | PB-R02 to PB-R06, BD-PB-01, UT-PB-02, UT-PB-03 and AU-PB-01, AU-PB-02 |
| PB-B [0106](../specs/0106-behemoth-core-container-and-config.md) | Typed container and wiring, starters and auto-configuration, settings, conditions, profiles and lifecycle | BB-01, BB-03, BB-04, UT-BB-01, UT-BB-03, UT-BB-04 and PL-BB-01. For the strict and warning migration cells use BB-12, BB-16, BD-BB-03, FI-BB-03 and the existing 0139 start-up qualification joins |
| PB-C [0107](../specs/0107-behemoth-web-clients-and-serialisation.md) | Servers, routing and versioning, clients, gRPC and streaming, WebSockets, GraphQL, hypermedia, serialisation and SOAP | BC-01, BC-02, IT-BC-01 and UT-BC-01 |
| PB-D [0108](../specs/0108-behemoth-data-access.md) | Repositories and mapping, drivers and pools, scope and transactions, migrations and caches | BD-07 to BD-09 and BD-20 to BD-22, IT-BD-01, IT-BD-02, IT-BD-03, IT-BD-06, UT-BD-14 and PL-BD-03 |
| PB-E [0109](../specs/0109-behemoth-security-and-identity.md) | Filters and access, OAuth, OpenID Connect, SAML, LDAP, passkeys, method security, TLS bundles and sessions | BE-01, BE-02, BE-03, UT-BE-01, UT-BE-02, UT-BE-03 and PL-BE-01 |
| PB-F [0110](../specs/0110-behemoth-messaging-events-and-resilience.md) | Messaging and integration, event ordering and outbox, scheduling, resilience, state machines and locks | BF-01, BF-02, BF-03, UT-BF-01, UT-BF-02, IT-BF-01 and FI-BF-01, FI-BF-02 |
| PB-G [0111](../specs/0111-behemoth-batch-cloud-ai-and-mail.md) | Batch and checkpointing, cloud patterns, AI and MCP, mail and templates | BG-01, BG-02, BG-03, IT-BG-01, FI-BG-01, FI-BG-02 and MS-BG-01 |
| PB-H [0112](../specs/0112-behemoth-actuator-and-observability.md) | Actuator, health, metrics, tracing, structured logs and failure analysis | BH-01, BH-03, BH-05, IT-BH-01, IT-BH-02, IT-BH-05, UT-BH-02 and FZ-BH-01 |
| PB-I [0113](../specs/0113-behemoth-developer-tools.md) | Reload, composition tools, test slices, images, metadata and upgrade diagnosis | BI-01, BI-02, BI-03, UT-BI-01, UT-BI-02, PL-BI-01 and FI-BI-01 |
| PB-J [0114](../specs/0114-behemoth-verification.md) | Independent parity tests, Quint, fuzzing, matrix, benchmarks and chaos | BJ-01, BJ-02, BJ-03, BJ-21, UT-BJ-01, PL-BJ-01 and AU-BJ-01, AU-BJ-02, AU-BJ-03 |

Before sending work downstream, reconcile the following prerequisites
under the existing owners. A Phase 3C integration exercise or Phase 7
release task does not supply these Phase 3B results by itself.

- **Source and ledger.** Close 0081's source and applicability
  checks for the frozen official train. QR-004 and RQ-015 hold full
  coverage where an upstream item or release remains unresolved.
  Preserve the approved disposition for each inapplicable row and keep
  unmatched or unverified applicable items out of completed coverage.
- **Protected build.** Carry the 0003 cutover result under EX-03 and
  QR-099. Verify committed activation and match the active gate to the
  exact Boot build inputs and target closure under 0139 and 0010. Use
  IT-0139-BOOT-COVERAGE to refuse mismatches and interrupted activation.
- **Boot qualification.** Use BJ-21 and 0116 N-1725/N-1726 and
  LS-27, LS-28 and LS-30 for the exact Boot artefacts and profiles
  under 0139.
  Keep platform parity, reproducibility, independent test review and
  applicable long-lane results bound to the same candidate. A skipped
  or unadmitted Windows result gives no pass. This is framework
  evidence under the existing contract. Phase 7 still needs its own
  exact release candidate and its applicable release proofs.
- **Signed handoff.** The existing 0010 phase result under 0139 binds
  the complete covered ledger, graph, profile matrix and artefacts
  after 3A. Use synthetic host fixtures for 3B. Downstream product
  integration and live effects remain subject to their own phase and
  owner contracts.

## Phase 3C first-slice tasks

Every row below inherits the 0076 check of the exact signed 3B exit
and its Boot artefacts, parity ledger and consuming profile. Carry
this prerequisite in each Forgejo task. Verify it at all 0076
admission and acceptance points even when a row lists only its
local dependencies. The same dependency holds for later product
work, alongside signed Phase 3 closure and intervening exits.
Use ADR-0021's desktop and local transport boundaries. QR-100 is a
prerequisite for admitting SDS, the desktop and local action route,
the WordPress interface and packaged manual.

| Task | Outcome and owning design | Depends on | Checks and acceptance | Specific done evidence | Forgejo export hints |
|---|---|---|---|---|---|
| TASK-0106-BOOT-WIRING | Wire the first service through the composition root, [0106](../specs/0106-behemoth-core-container-and-config.md), item 1 | Signed 3B exit and exact starter artefacts for the consuming platform and profile | BB-01, BB-03 and BB-04 refuse hidden lookup, missing or ambiguous providers and a cycle. An explicit graph starts. Use [FX-0139-ENTRY-PRODUCT](test-and-traceability.md#phase-3-foundation-handoff-checks) to refuse a same-version starter replacement before product build or start. | Startup report and planted wiring failures, exact consumed artefact digests and substitution refusal | #26 |
| TASK-0014-PROFILE-ACTIVATION | Load typed startup settings and apply a complete version, [0014](../specs/0014-settings-profiles-history.md), item 1 | Composition root and database starter | AT-CF-OUTBOX and AT-CF-DELIVERY prove coupled commit, ordered recovery and complete snapshots. Bad values or expired authority are refused | Exact role versions, rollback proof and pending-restart evidence | #26 |
| TASK-0112-SERVICE-HEALTH | Expose redacted health and process heartbeat for the first service, [0112](../specs/0112-behemoth-actuator-and-observability.md), item 1 | Composition root | BH-03 and BH-05 check no secret leaves and outside outage affects readiness without replacing liveness | Health and heartbeat receipts for current and stale instances | #34 |
| TASK-0107-FIRST-ERROR-SHAPE | Return the common problem-details response for the first service action, [0107](../specs/0107-behemoth-web-clients-and-serialisation.md), item 1 | Composition root and first typed action | Wrong input, missing permission and failed dependency use their declared codes and one redacted shape | Contract response fixtures and consumer parity | #26 parent |
| TASK-0112-FIRST-LOG-METRICS | Emit bounded masked logs and local metrics for the first service, [0112](../specs/0112-behemoth-actuator-and-observability.md), item 1 | Service health | BH-11 and BH-20 cover non-blocking masked logs and bounded hot-path metric recording. Keep telemetry off by default | Local output and planted secret and bound checks | #34 parent |
| TASK-0032-FIRST-AUDIT | Audit one authorised action and refuse a tampered history, [0032](../specs/0032-audit-erasure-diagnostics.md), item 1 | Database starter and local action | AU-01 to AU-03 keep private values out, prevent edits by the app role and detect a changed chain | Exact append and tamper receipts with redacted outputs | Roadmap 3C item 1 |
| TASK-0108-RLS-STARTER | Start PostgreSQL with checked migrations and site isolation, [0108](../specs/0108-behemoth-data-access.md), item 2 | Composition root and pinned PostgreSQL | Use the qualified PB-D scope under BD-07 to BD-09 and BD-20 to BD-22. Check unscoped reads, distinct setup and migrator authority, bypass roles and pooled context leakage. Applied migration drift blocks startup. IT-BD-02 and IT-BD-03 refuse the same named app role that becomes ineligible before first borrow despite successful startup | Exact consumed Boot profile, two-tenant results, bootstrap scope refusals, and schema and role identities | #170 |
| TASK-0017-FIRST-COMPONENTS | Create the first SDS components and catalogue on the approved Angular route, [0017](../specs/0017-visual-layer.md), item 3 | Signed 3B exit and Phase 2 front-end check admission | A labelled input, error, progress state and action work by keyboard with text and focus. Plant a missing label and untranslated text | Component evidence and accessibility results for the declared profiles | #35, #104 |
| TASK-0030-FIRST-WINDOW | Open the first Tauri and Angular desktop slice, [0030](../specs/0030-install-upgrade-removal.md), item 4 | SDS and admitted package inputs | A clean profile opens to safe setup. Refuse unverified payload and unavailable local transport with plain reasons | Window and setup demonstration, package identity and refusal receipts | #171 is the superseded desktop issue |
| TASK-0020-OWNER-DOOR | Connect the local Axum action route over the owner transport, [0020](../specs/0020-screens-actions-sutra-mcp.md), item 5 and REQ-0020-FIRST-ACTION-SCOPE / DES-0020-FIRST-ACTION-BINDING | Window, database starter and Cedar admission | BDD-0020-OWNER-TRANSPORT-BOUNDARY exercises exact socket and pipe profiles. Refuse a different user, wrong site, explicitly denied action and disabled action. Keep an eligible authenticated peer fixed in the action cases. Check no network listener | Observed first registered ActionSpec and policy, scoped action and adapter receipts, and BH-01 management boundary cross-check | Roadmap 3C item 5 |
| TASK-0110-FIRST-OUTBOX | Publish one typed event with its business change, [0110](../specs/0110-behemoth-messaging-events-and-resilience.md), item 6 | Database starter | BF-01 and BF-02 cover rollback and crash after delivery. One inbox effect results on replay | Transaction and relay evidence on the same site | #172 parent |
| TASK-0019-DURABLE-PLATFORM | Execute one recoverable background job through the registered PostgreSQL route, [0019](../specs/0019-events-jobs-recovery.md), item 6 | Database starter and first outbox | IT-0019-SCHEDULER-RESTART and IT-0019-DELIVERY-DUPLICATE preserve one fire, claim and logical effect. A stale attempt cannot commit. Use VAR-0033-INTERRUPTED-PROMISE for the verified release commit and replay branch of IT-0019-SCHEDULER-RESTART under QR-081 | Queue, lease, resource and recovery receipts | #172 parent |
| TASK-0030-SUPERVISOR | Start, drain and recover the first service under the app supervisor, [0030](../specs/0030-install-upgrade-removal.md), item 6 | Service heartbeat, local door and durable job | Close the window during ordinary work. Crash the service, then race Quit with restart. Record exactly the allowed instance and keep job state | Lifecycle timeline, drained stop and job reconciliation on the admitted profile | #172, #173 |
| TASK-0065-BOOT-SIGNER | Move the product signing core onto Boot with exact purpose and key checks, [0065](../specs/0065-tijori.md), item 8 | Product composition root and signing fixtures | Matching signed fixtures produce the same decisions through Boot. A wrong purpose fails and the gate and sync need no Boot service. Join IT-0065-BOOT-SIGNER-CONFORMANCE, FI-BB-02 and IT-BB-01 for drain after a durable site-message signature | Before and after conformance, exact product late-reply refusal and signature replay proof, and operations dependency-wall evidence | #174 contains earlier scope |
| TASK-0106-FIRST-SLICE-DEMO | Demonstrate the first service from the owner window, [0106 first product slice](../specs/0106-behemoth-core-container-and-config.md#first-product-slice-demonstration), REQ-0106-FIRST-SLICE and DES-0106-FIRST-SLICE | Current signed 3A and 3B exits, exact adopted Boot consumer cell and all other 3C tasks | BDD-0106-FIRST-SLICE-DEMO exercises the healthy journey, startup failures, worker recovery and ordinary window close on each required consuming profile. Bind the exact 0106 OS variant. A different profile’s demo cannot supply the required cell | Exact first-product-slice evidence, one recorded journey for each protected required cell, gate and review records under 0010. Proof of the adopted Boot cell is separate | #173 |

## Phase 4 site-slice tasks

Every row below requires verified Phase 3 closure under 0076
before it may be derived, claimed or resumed. Include the exact
closure receipt in each task. Follow the roadmap platform order.
Reuse the shared signing, indexing and review contracts in both adapters.

| Task | Outcome and owning design | Depends on | Checks and acceptance | Specific done evidence | Forgejo export hints |
|---|---|---|---|---|---|
| TASK-0021-SIGNED-SITE-CONTRACT | Pair one test site and exchange signed messages, [0021](../specs/0021-forum-contracts-ingestion.md), item 1 | Boot signer and owner door | Test wrong site and key, stale time, used nonce and altered bytes. Only the matching paired service succeeds | Both sides' verified conformance and signed refusal records | #27 |
| TASK-0031-SITE-FIXTURES | Run both disposable site fixtures in Dell containers through the heavy-job queue and measure footprint, [0031](../specs/0031-shared-platform-installation.md), item 2 | Signed Phase 3 closure and pinned and licensed fixture inputs | Run each version with synthetic data. [IT-0031-ZIP-EXACT-UPLOAD](../specs/0031-shared-platform-installation.md#exact-native-zip-acceptance) uses a clean XenForo 2.3 fixture and each supported WordPress major on the exact host profile. A measured added query, byte or delay beyond its design budget fails | Reproducible fixture identity, site matrix, separate native handoff and refusal evidence, and measured baselines. Supply the admitted WordPress Plugin Check profiles for both WordPress majors | #175 |
| TASK-0021-XF-GUEST-READ | Ingest one XenForo public thread using current visibility, [0021](../specs/0021-forum-contracts-ingestion.md), item 3 | XenForo fixture and signed contract | Compare current native guest decision, permission changes and deleted posts. Private content and stale revisions never enter the eligible set | Signed input and visibility reconciliation evidence | #28, #43 |
| TASK-0022-SITE-INDEX | Index one complete eligible source and its revision, [0022](../specs/0022-reading-indexing.md), item 3 | XenForo read and admitted text and model inputs | A long post stays whole across chunks. Unknown language or torn representation is refused. A replay after erasure cannot restore the index row | Chunk and source coverage, generation and erasure proof | #25, #28 |
| TASK-0027-SITE-PROPOSAL | Produce one relevant reviewable candidate set, [0027](../specs/0027-candidates-ranking.md), item 3 | Exact index generation and eligibility | Exercise on-topic, ineligible and high-traffic unrelated targets. The relevance gate refuses unrelated candidates and the result is reviewable | Candidate and explanation fixture with exact model evidence | #33 |
| TASK-0028-XF-REVIEW-EDIT | Review, apply and undo one XenForo suggestion, [0028](../specs/0028-staff-suggestions.md), item 3 | Proposal and native edit conformance | IT-REVIEW-EFFECT covers a lost reply, stale destination, expired receipt, duplicate undo and revoked authority. Unrelated text stays exact | Signed edit and undo history, refusal and footprint proof | #33 |
| TASK-0031-WP-GUEST-READ | Ingest only WordPress public material with native permission checks, [0031](../specs/0031-shared-platform-installation.md), item 4 | XenForo end-to-end proof and WordPress fixture | Password, private, draft and public variants use current native facts. Cache or security plugin does not widen access | Visibility and compatibility evidence across the declared matrix | #176 parent |
| TASK-0031-WP-REVIEW-EDIT | Propose, review and selectively undo through the WordPress native route, [0031](../specs/0031-shared-platform-installation.md), item 4 | WordPress read, proposal and approved native save protocol | Race a human edit with apply. Wordfence and admitted cache tests preserve exact scope. Lost reply reconciles without overwriting a later edit | WordPress receipt and replay, footprint and compatibility matrix | #176 |
| TASK-0031-SITE-PHASE-DEMO | Demonstrate both site routes and associated refusal cases, [0031](../specs/0031-shared-platform-installation.md), phase exit | Both platform slices, exact ZIP handoff fixture evidence and measurements | Recorded journeys show signed read, suggestion, review, save and selective undo. All footprint limits hold | Two recorded site journeys, exact native ZIP proof references for the demonstrated host profiles and independent phase evidence | Roadmap Phase 4 exit |

### Phase 4 exit evidence

The four items in the [Phase 4 roadmap][phase-four-exit] have separate
evidence. Under QR-241 bind the exact accepted 3C lineage, protected
candidate, platform and host profiles, independent reviews and the
authoritative milestone and task checks. The roster gives the
protected profile no signed record or milestone authority.

| Roadmap item | Required evidence sources |
| --- | --- |
| 1. Signed site contract | TASK-0021-SIGNED-SITE-CONTRACT. Bind the exact signed 0021 and 0118 protocol and adapter profiles. Show one valid paired exchange and the named errors or safe refusals on each relevant receiver. |
| 2. Host fixtures and footprint | TASK-0031-SITE-FIXTURES. Bind pinned, licensed synthetic fixtures and baseline-plus-enabled query, time and byte results on Dell. Keep the exact native ZIP handoff, input licence and each footprint measurement as separate proofs. |
| 3. XenForo route | TASK-0021-XF-GUEST-READ, TASK-0022-SITE-INDEX, TASK-0027-SITE-PROPOSAL and TASK-0028-XF-REVIEW-EDIT. Bind guest visibility, index generation, exact proposal, native moderator review and original apply and selective undo receipts. |
| 4. WordPress route | TASK-0031-WP-GUEST-READ and TASK-0031-WP-REVIEW-EDIT, with the same index and proposal contracts for the demonstrated site. Bind the admitted Wordfence and cache combination, supported host proof and independent native review, apply and selective undo receipts. |

TASK-0031-SITE-PHASE-DEMO ties the two recorded journeys to their
exact constituent evidence. A successful demonstration cannot
supply a missing clean-install proof or a footprint measurement.
A WordPress recording made without the required Wordfence and cache
profile cannot qualify that profile. Retain the exact predecessor
at the exit check. Phase 5 needs the approved current Phase 4 proof
under 0076 and QR-241.

[phase-four-exit]: ../roadmap.md#phase-4-xenforo-then-wordpress

## Phase 5 core slices

| Task | Outcome and owning design | Depends on | Checks and acceptance | Specific done evidence | Forgejo export hints |
|---|---|---|---|---|---|
| TASK-0030-PLATFORM-INSTALL | Install the approved packages and portable setup on each admitted platform, [0030](../specs/0030-install-upgrade-removal.md), item 1 | Admitted package and first-trust route, exact consumed `TASK-0108-RLS-STARTER` installation profile, 0033 disk admission and QR-202 native first-creation authority | [IT-0030-PLATFORM-INSTALL](test-and-traceability.md#task-0030-platform-install) verifies the exact first-run bytes and warns about the unsigned Windows wrapper. Hold an unproved root, reservation, audit record or first effect. Reconcile the original native receipt and exercise only the authorised installation database mirror. | Installer identity, native intent, lock, reservation, audit and ledger receipts, exact scoped database evidence and tested first-run paths on each admitted target | #40 |
| TASK-0030-UPDATE-REPAIR | Update and repair through the app without losing user work, [0030](../specs/0030-install-upgrade-removal.md), item 1 | Install and backup proof | Use 0031 IP-23 for exact owner approval and changed-plan refusal. Cut power at each switch boundary. A corrupted part is identified and repaired from exact signed bytes | Interrupted update, rollback and repair receipts | #40 |
| TASK-0030-BACKUP-RESTORE | Restore consistent data, settings and site registry, [0030](../specs/0030-install-upgrade-removal.md), item 1 | Site ledger and protected restore history | Use 0031 IP-30 for backup recovery material, protection epoch and isolated restored roles and forced RLS on each applicable admitted restore variant. On the admitted Windows variant, IP-30 also refuses a substituted native volume with the same drive letter and label before publication. Use IT-SS-07 for registry retirement and missing pairing keys, and AT-CF-DELIVERY for settings epoch and activation replay. | Exact variant, verified manifest, recovery and restored policy results, and separate retirement, erasure and settings evidence | #40 |
| TASK-0030-REMOVE | Remove one site or the app from the ledger, [0030](../specs/0030-install-upgrade-removal.md), item 1 | Install, backup and declared keep/delete choices | Refuse stale confirmation, race a queued job, and retry a partial removal. Respect the kept evidence and user scope | Exact preview, removal receipts and retained-scope proof | #40 |
| TASK-0033-SAFE-RECOVERY | Report and recover an admitted local fault while holding ambiguous effects, [0033](../specs/0033-safe-running.md), item 2 | Supervisor and durable job recovery | Stop on unsafe disk state. A missing or uncertain effect is reconciled and no unauthorised retry runs. Check the separate fixed 0018 retention notice and reset under SR11. Join SR03 to REQ-0033-PROMISE-ACCOUNTING, DES-0033-PROMISE-CHECK and VAR-0033-INTERRUPTED-PROMISE. The original charge remains held without qualified proof under QR-081 | Failure, reconciliation and resumed-work evidence | #51 |
| TASK-0032-RETENTION-ERASURE | Apply declared retention and erasure through active copies and restore, [0032](../specs/0032-audit-erasure-diagnostics.md), item 3 | Backups and site ownership | Request erasure while an export or restore is prepared. Revoked output cannot leave. AU-08 verifies the restored set | Deletion and refusal receipts with no private details | #45 |
| TASK-0066-GANIT-PLATFORM | Define registered typed job entry, receipt and recovery for ganit, [0066](../specs/0066-ganit.md), item 4 | Durable scheduler, resource admission and an exact engine registration | Refuse wrong schema, job identity and undeclared owner. Replay one compatible result without a second effect | Versioned job registry and boundary receipts | #30 |
| TASK-0027-GANIT-JOB-ROUTE | Connect a named live numerical job and one bounded maintenance candidate, [0027](../specs/0027-candidates-ranking.md), item 4 | Site proposal and resource admission | Refuse partial, stale or slow candidate output. The verified champion remains current | Versioned results and degraded-route demonstration | #30–#32 are broader parents |
| TASK-0034-VISIT-COUNTING | Count admitted human visits and show coverage on the owner's site, [0034](../specs/0034-analytics.md), item 4 | Site link and consent decision | AT-AN38 separates a valid joined line, rejected opt-out and replay. A freeze retains only the previous eligible rates. For the darshak job 325 consumer, AT-AN47 refuses a late finite result under SUT-614 and keeps the verified champion | Exact visit labels, no-count and retention evidence, and the separate 0066/0077 consumer disposition | #37 |
| TASK-0035-OWNER-BOOKS | Record scoped monetary events and show one reconciled report, [0035](../specs/0035-hisaab.md), item 4 | Site-scoped records and admitted currency rules | Refuse duplicate and unbalanced events. Preserve exact separate currencies on replay and restore | Balanced statements, evidence and fault cases | #58 |
| TASK-0065-VAULT-LIFECYCLE | Store, rotate and revoke the application's admitted keys, [0065](../specs/0065-tijori.md), item 4 | Boot signer and local owner authority | Check that old authority fails after rotation and no state, export or log exposes a key | Key lifecycle and refusal evidence | #448 is release custody only |
| TASK-0020-SUTRA | Expose the same scoped actions and error shape through the local CLI, [0020](../specs/0020-screens-actions-sutra-mcp.md), item 5 | Local typed action API | Dry-run is read-only. A change attempt with expired or wrong-scope authority refuses and JSON cannot give live confirmation | API and CLI parity and redacted receipts | #41, #44 |
| TASK-0020-LOCAL-MCP | Let the owner connect an agent using local single-site authority, [0020](../specs/0020-screens-actions-sutra-mcp.md), item 5 | Typed action API and grant registry | Read only within the granted site, refuse credentials and protected keys, and require the existing live-confirmation route | MCP scope and revocation evidence on both transports | #41 parent |
| TASK-0037-WORKER-FENCE | Admit the approved extension and sidecar worker bounds, [0037](../specs/0037-extension-isolation.md), item 5 | Resource profile, QR-029 and approved package route | Plant disallowed file, process and network access. Stop closes the complete worker tree and leaves no permitted effect | Linux and Windows fence and cleanup evidence | #207 |
| TASK-0038-OWNED-STORAGE | Connect an approved existing bucket or create one approved private bucket, and verify a buffered object move, [0038](../specs/0038-search-services-cloud-storage.md), item 6 | Local storage, exact owner approval for each action, and the applicable QR-202 storage profile | Use SC-08, SC-09, SC-11 and [FX-0038-W5-PRIVATE-BUCKET-EXACT-CLAIM](feature-wave-5.md#acceptance-design-for-n-1454-to-n-1477) for exact creation, privacy verification and claim recovery. A missing create result keeps the bucket staged. Confirm an existing bucket through its own scoped connection without create permission. Keep SC-05, SC-07 and SC-08 for buffered move, verified switch and separately approved cleanup. | Bind the exact provider and route, approval, private-state proof and original create and claim receipts. Keep existing-bucket connection and encrypted-object move evidence distinct. Hold an unproved profile. | #49 |
| TASK-0004-IN-APP-GUIDE | Package the beginner manual for offline use, [0004](../specs/0004-manual-publishing.md), item 7 | Core action states and QR-100 | Every Phase 5 item has a beginner guide or explanation tied to a matching screen and outcome. Walk install, restore, removal, support and provider-refusal cases without a network | Exact packaged manual and accessible, offline walkthrough evidence | #42 |
| TASK-0078-PINNED-PROFILES | Identify each supported owner-machine test profile, [0078](../specs/0078-memory-budget-and-turn-taking.md), item 8 | Admitted supported OS and packages | Refuse a result if hardware, OS, workload or model identity is missing. Distinguish low-memory and larger profiles | Exact signed profile inventory and preflight receipts | #177 |
| TASK-0078-CORE-RESOURCE-PROOF | Measure the core workloads on each admitted profile, [0078](../specs/0078-memory-budget-and-turn-taking.md), item 9 | Pinned profiles and Phase 5 workloads | MB-02, MB-04 and MB-08 prove ceiling, never-wait work and fair shares. Load and soak preserve recovery headroom. FI-MB uses MB-09, MB-14 and DES-0078-PROMISE-CONSUMER for the VAR-0033-INTERRUPTED-PROMISE admission and Jobs reason on each applicable profile. Join 0019 UT-0019-RESOURCE-BUDGET and BDD-0019-RESOURCE-REFUSAL, 0033 SR03 and 0078 MB-07/MB-09 for fixed over-ceiling refusal and temporary capacity wait | Exact-profile peak memory, tail latency, load and soak receipts. Verify SUT-237 separately from a temporary wait reason and attempt counts | #178 |

TASK-0027-GANIT-JOB-ROUTE is one integration slice. Phase 5 item 4
also derives per-engine and per-job tasks from the spec 0066 registry
and each named consumer. The platform, sample and archive parent
issues do not supply evidence for those other contracts.

### Phase 5 exit evidence

Use the nine items and exit rule in the [Phase 5
roadmap](../roadmap.md#phase-5-the-rest-of-the-core). This roster
points to the existing evidence owners for the protected closure
profile in QR-241. The catalogue supplies no signed receipt or
signing authority. Preserve the current signed 3C lineage and
require the exact accepted Phase 4 exit. Match the authorised
candidate, profiles, tests and independent reviews. Before closing
check the authoritative Forgejo milestone and its open tasks under
AGENTS.md section 10.5. Missing source proof holds the affected exit.

| Roadmap item | Required evidence sources |
| --- | --- |
| 1. Installation lifecycle | TASK-0030-PLATFORM-INSTALL, TASK-0030-UPDATE-REPAIR, TASK-0030-BACKUP-RESTORE and TASK-0030-REMOVE. Bind the exact 0030/0031 platform, source and native target, protection, state-reader and recovery proofs. Use test release keys for the Phase 5 package fixtures under 0065. |
| 2. Recovery | TASK-0033-SAFE-RECOVERY and the owned fault cases for the selected routes. Require the original effect or refusal receipt and the measured recovery result where applicable. |
| 3. Retention | TASK-0032-RETENTION-ERASURE, with the selected archives, active copies and restored data checked under the owning retention and erasure rules. |
| 4. Core services | Exact 0066 engine and job evidence and its consuming tasks, TASK-0034-VISIT-COUNTING, TASK-0035-OWNER-BOOKS and TASK-0065-VAULT-LIFECYCLE. Include the separate AT-AN47 darshak consumer proof for job 325 under 0066 and 0077. Match the owning scope and reviews for each part. |
| 5. Local tools | TASK-0020-SUTRA and TASK-0020-LOCAL-MCP across their admitted transports and authority limits, with the local key tools under 0065. |
| 6. Cloud helpers | Separate protected evidence for [0031's owner-account helper](../specs/0031-shared-platform-installation.md#shape-of-an-installation) under QR-242. TASK-0038-OWNED-STORAGE gives proof only for its exact store operations. A bucket connection or verified move supplies no helper evidence. |
| 7. Manual | TASK-0004-IN-APP-GUIDE. Verify the packaged beginner guide for each required Phase 5 item against its admitted route and results. |
| 8. Pinned machines | TASK-0078-PINNED-PROFILES. Bind the exact hardware, OS and resource identity of every declared supported owner-PC profile. |
| 9. Workload measurements | TASK-0078-CORE-RESOURCE-PROOF. Use release-tier peak memory, p95 latency, load and soak evidence on the matching pinned owner-PC profiles under 0078. Require each test's complete observed run under its exact admitted workload recipe. Separate incomplete runs supply only partial observations, even if together they name every required window. |

Item 6 keeps the owner-only settings checks in 0014/0020, the
registered job and recovery rules in 0019, the 0078 resource fence
and the 0118 approved recipient and encryption proof. A helper
build task needs a complete owning design and evidence under 0076.
The unresolved qualification question supplies no execution, cost
or provider grant. Existing local operation remains available.

## Phase 6 feature waves

[Wave 1](feature-wave-1.md) routes features 1 to 112.
[Wave 2](feature-wave-2.md) names features 113 to 712 and their sections.
[Wave 3](feature-wave-3.md) names features 713 to 1312 and their sections.
[Wave 4](feature-wave-4.md) names features 1313 to 1357 and their sections.
[Wave 5](feature-wave-5.md) traces 523 pasted features and their owners.

For [N-1871 storage setup](../specs/0038-search-services-cloud-storage.md),
use the existing Phase 5 item 6 storage actions under
TASK-0038-OWNED-STORAGE as the dependency for the applicable Amazon S3
or verified move path. Bind the
[wizard fixture](feature-wave-5.md#acceptance-design-for-n-1863-to-n-1880)
to the wizard's own 0076 readiness record. Reuse a storage receipt only for
the exact approved action, site, destination and source revision.
Activation requires verified read-back and its protected receipt.
Removing the local copy needs separate owner approval.

Use the [trace admission procedure](test-and-traceability.md) to
reconcile source, owning contracts, scope, scenarios and evidence
under spec 0076. Derive tasks in roadmap order.

### Local Slides report design package

Use DR-087 and the [0094 candidate][catalogue-slides] for this
report variant. Derive an issue only after the exact
prerequisites of spec 0076 are bound.

| Task | Outcome and owning design | Depends on | Checks and acceptance | Specific done evidence | Forgejo export hints |
|---|---|---|---|---|---|
| TASK-0094-LOCAL-SLIDES | Seal one eligible Slides-compatible local report under DES-0094-FILES and VAR-0094-SLIDES-PPTX. Source DR-087 and REQ-0094-EXPORT. | Frozen 0094 master, 0100 exact local file effect, 0032 and 0118 custody. QR-067 and QR-132 qualify the published sources, package, renderer, checker, rights, bounds and exact manual target. | Join FX-0094-SLIDES-MANUAL-IMPORT and E2E-0094-EXPORT, with current scope and erasure refusal. | Exact profile and package checks, separately authorised synthetic manual import witness, typed layout, fidelity and editable-text evidence, one scoped receipt, and no product or test-runner requests to the destination. | 0076 derivation only |

[catalogue-slides]:
  ../specs/0094-reporting-forecasts-proof-and-experiments.md#local-slides-report

### Scientific experimentation design packages

Use the contracts in specs 0148–0151 under ADR-0026. The producer
work builds on the existing 0040 connectors and the 0034 aggregate
API. It follows each existing owner and consumer boundary.
These records define exact planned work. Apply 0076 to the
owning design, permitted variants and proof before deriving an issue.
Synthetic fixtures supply no real source, platform or phase admission.

| Task | Outcome and owning design | Depends on | Checks and acceptance | Specific done evidence | Forgejo export hints |
|---|---|---|---|---|---|
| TASK-0148-FINAL-LOOK | Own the sole final claim, alert correction and advisory verdict release under [0148](../specs/0148-scientific-experimentation-engine.md), DES-0148-FINAL-LOOK. Keep estimators, source evidence and guidance under their existing owners. | Exact sealed plan and scope fences, and the 0149 method and 0150 proof interface definitions for synthetic cases. An actual final claim needs the qualified 0149 method and placebo profile, all three current 0150 source proofs and the original authority and evidence fences. | Run IT-0148-RELEASE and IT-0148-CORRECT with IT-0149-FINAL and IT-0149-PLACEBOS. Join BDD-0150-THREE-SOURCE and AT-CN25 for both source-generation commit orders. Exercise the exact deadline, one microsecond later, and correction before and after claim. | The pinned claim, source proof references, correction receipt, sealed verdict or specific hold and no-second-look evidence. Synthetic proof does not qualify a method, a real source, a live effect or a release. | 0076 derivation only |
| TASK-0148-LOCAL-ENTRY | Own the dedicated experiment draft and local management boundary under [0148](../specs/0148-scientific-experimentation-engine.md), DES-0148-ENTRY and DES-0148-MANAGEMENT. Use the typed prepare and read interfaces and the current desktop confirmation route. | 0020 scoped local actions, 0118 protected transport, and the exact eligible Coach and Action Compiler source revisions in SRC-0148-ENTRY. Use owning 0148 action interfaces for effects. | Use BDD-0148-START, IT-0148-LOCAL and E2E-0148-DESKTOP. Keep ordinary work free of experiment side effects. Refuse cross-site, expired or changed authority and every privileged MCP action. | Exact deduplicated draft and protected projection receipts. Bind Ubuntu 26.04 LTS, Debian 13, Windows 11 and the eligible local MCP route to their own tested scope and authority. | 0076 derivation only |
| TASK-0148-SEAL-FOOTPRINT | Register the immutable plan, assignment and exact footprint guard under [0148](../specs/0148-scientific-experimentation-engine.md), DES-0148-SEAL and DES-0148-FOOTPRINT. Preserve the owning exposure and stop routes. | Exact draft and owner confirmation, 0094 assignment and guard, 0048 revision verification and the 0150 activation interface for synthetic checks. A real reservation needs all three current qualified activation receipts. Any separately authorised native effect needs matching current 0021 sealed 2.0 and the owning moderation proof. | Run IT-0148-SEAL, IT-0148-LOCK and FI-0148-CKPT. Pin membership, metric and deadlines before exposure. Race intersecting footprints. Refuse guarded Sutradhar writes to controls and separately hold on proved outside edits. Refuse stale or mismatched scope and legacy native proof. | One sealed registration and assignment per exact receipt. One eligible intersecting reservation and a verified stop or hold with original effect identity. Preserve 0094 exposure and the existing native apply and undo authorities. | 0076 derivation only |
| TASK-0148-WORK-RECOVERY | Own experiment job bounds, exact effect reconciliation, stale-result withholding and tracked erasure under [0148](../specs/0148-scientific-experimentation-engine.md), DES-0148-CUSTODY and DES-0148-RESOURCE. | 0019 job, outbox and effect identities, 0078 host admission, 0104 worker teardown, 0030 file ledger, 0032 erasure and 0118 custody. Synthetic cases use the defined plan, claim and result contracts. | Run FI-0148-CKPT, IT-0148-RESOURCE and E2E-0148-DESKTOP. Reuse UT-0019-FENCE-COMMIT and UT-0019-SITE-UNKNOWN for their exact worker and native-effect boundaries. Exercise budget shrink, pause, lost reply and stale restore. | One exact job or recovery receipt, measured bounds under each required host profile, complete tracked cleanup, and no duplicate effect, late publication or stale restore access. A fixture gives no executor or native authority. | 0076 derivation only |
| TASK-0149-BLOCKED-ITT | [0149](../specs/0149-causal-estimators-and-placebos.md#readiness-trace) blocked intention-to-treat computation preserves original assignment and frozen group weights | 0094 assignment and exposure, 0148 registered analysis plan and 0150 checked matrix under DR-143/144 | UT-0149-ITT and FX-0149-BLOCK-WEIGHTS reproduce the exact 1/3 effect and retain original arms after an exposure switch. Missing frozen support holds | Exact allocation and matrix identities, checked rational contrast and its arithmetic profile | 0076 derivation |
| TASK-0149-DECISION-REPLAY | [0149](../specs/0149-causal-estimators-and-placebos.md#readiness-trace) binds placebo diagnostics, outward decision bounds and canonical numeric output to the exact one-look analysis | Exact registered estimator variant, 0148 sole final-claim and release contract, 0150 composite and three ordered producer proofs and independent method and arithmetic admission | IT-0149-PLACEBOS, UT-0149-SENSITIVITY, IT-0149-FINAL and IT-0149-REPLAY with IT-0148-RELEASE check refusal, one-look recovery and exact numeric replay | Variant-specific diagnostic and interval receipts, canonical numeric bytes and digest bound to the final look and exact platform profile | 0076 derivation |
| TASK-0149-MATCHED-DID | [0149](../specs/0149-causal-estimators-and-placebos.md#readiness-trace) exact pre-period matching and difference-in-differences over frozen own-site pairs | 0148 observational plan, qualifying 0150 pre-period evidence and independent same-site support, parallel trend and bias admission under DR-143/144 | UT-0149-MATCH and FX-0149-TIED-PAIR-ORDER select the canonical tied pairs. Foreign, unmatched, changed or failed pre-validation cases hold | Frozen pair roster and fit and validation references, exact tie order and independent support receipt for the named profile | 0076 derivation |
| TASK-0149-SYNTHETIC-CONTROL | [0149](../specs/0149-causal-estimators-and-placebos.md#readiness-trace) bounds the pre-only donor search and rechecks the selected weight vector in Rust | 0148 observational plan, 0150 admitted pre-period inputs, independent donor and sensitivity admission, 0019 and 0078 budgets and the applicable 0037 worker fence | UT-0149-SYNTH and FX-0149-TIED-DONOR-WEIGHTS prove canonical tied weights and exact cap behaviour. A timeout or post-outcome input cannot alter the fit | Exact donor roster, fit-window and grid identities, checked weights and refusal evidence for the admitted worker profile | 0076 derivation |
| `TASK-0034-SCIENCE-DARSHAK` | Qualify producer-owned human reading-band evidence through the existing [0034 collector](../specs/0034-analytics.md#beacons-log-lines-and-the-visit-id). Spec 0150 owns scientific admission. | Exact TASK-0034-VISIT-COUNTING proof, current scope and disclosure grant, and independent QR-231 receipt for the exact collector release | Pair IT-0150-DAR-READER with AT-AN24 and AT-AN37 to AT-AN40 and FX-0034-W2-ATTENTION-BANDS in the [scientific source trace](test-and-traceability.md#scientific-experiment-source-admission). | Pin the exact native day, measurement mode, content revision, classification watermark, completed human band and current measured-human denominator. Withhold an unproved release. | Derive under [0076](../specs/0076-specification-completeness.md#derived-task-contract). |
| `TASK-0040-SCIENCE-GA4` | Qualify the producer-owned GA4 key-event evidence on the same read-only [0040 connector](../specs/0040-search-connectors.md#experiment-source-profiles). Spec 0150 owns scientific admission. | Current verified own property, eligible host mapping and event definition, compatible denominator and independent QR-231 maturity and population evidence | Pair IT-0150-GA4-KEY-EVENT with AT-CN29, AT-CN33 and AT-CN35 for the separately owned quota, parser and site fences. Use the [scientific source trace](test-and-traceability.md#scientific-experiment-source-admission). | Pin the approved selected event, native day and zone, filters, attribution, denominator, metadata and current report generation. Record exact complete-zero evidence or a typed hold. | Derive under [0076](../specs/0076-specification-completeness.md#derived-task-contract). |
| `TASK-0040-SCIENCE-GSC` | Qualify the producer-owned Search Analytics page-day evidence under [0040](../specs/0040-search-connectors.md#experiment-source-profiles). Spec 0150 owns scientific admission. | Current read-only own-property grant, exact native page and surface profile, and independent QR-231 coverage and zero evidence | Pair AT-CN24 and AT-CN25 with IT-0150-GSC-COVERAGE and UT-0150-SURFACE-BRAND in the [scientific source trace](test-and-traceability.md#scientific-experiment-source-admission). | Pin the approved site, property, native day, aggregation, filters, manifest and current source generation. Preserve observed report rows and unknown omitted cells. | Derive under [0076](../specs/0076-specification-completeness.md#derived-task-contract). |
| `TASK-0150-SCIENCE-GATE` | Bind exact same-site producer proofs at each phase using [0150](../specs/0150-experiment-source-and-sensor-gates.md#source-gate-trace-identities). [0148](../specs/0148-scientific-experimentation-engine.md) owns the final claim and release. | TASK-0040-SCIENCE-GSC, TASK-0040-SCIENCE-GA4 and TASK-0034-SCIENCE-DARSHAK with exact candidate match, plus current frozen plan, incident, authority and custody proof | Use IT-0150-API-SCOPE, BDD-0150-THREE-SOURCE, BDD-0150-WINDOW-TIME and FI-0150-EXACT-REPLAY with IT-0148-RELEASE and IT-0148-CORRECT. Use the [scientific source trace](test-and-traceability.md#scientific-experiment-source-admission). | One bound composite per checkpoint revision, with ordered GSC, GA4 and darshak proof references and exact final claim binding. Missing or superseded evidence holds the affected boundary. | Derive under [0076](../specs/0076-specification-completeness.md#derived-task-contract). |
| TASK-0151-CALIBRATION | [0151](../specs/0151-site-lawbook-calibration-and-replay.md#requirements-and-planned-proof) scores immutable prediction cohorts and projects only eligible aggregate calibration | Exact producer issue, independent outcome and frozen roster, 0034 disclosure and 0020 scope. An 0146 forecast handoff needs separate QR-067 admission through 0040 and 0150 | IT-0151-CALIBRATION and IT-0151-PROTECTED-BOUNDARY check Brier, reliability, interval, point and programme evidence with unresolved and suppressed inputs | Frozen issue roster and one-look receipt, independently settled comparable evidence and currently eligible aggregate projection | 0076 derivation |
| TASK-0151-DISPLAY | [0151](../specs/0151-site-lawbook-calibration-and-replay.md#requirements-and-planned-proof) qualifies a separate conservative display for future issues of one exact registered stream | Exact eligible TASK-0151-CALIBRATION evidence, one frozen fit block and two independent validation blocks, current approvals and protected source and erasure generation | IT-0151-CONSERVATIVE-DISPLAY checks both transforms, validates both held-out blocks and fences a changed approval, source or erased display input | Pinned transform and independent validation evidence, exact owner confirmation and current approved display projection separate from the frozen prediction | 0076 derivation |
| TASK-0151-LAWBOOK | [0151](../specs/0151-site-lawbook-calibration-and-replay.md#requirements-and-planned-proof) projects the site shelves and exact reviewable guidance while keeping information estimates advisory | 0149 signed decision classes, 0148 current study and generation, 0020 exact owner confirmation, 0034 disclosure and DR-146 | BDD-0151-GUIDANCE-REVIEW, UT-0151-REPEAT-EVIDENCE, UT-0151-INFORMATION and IT-0151-PROTECTED-BOUNDARY distinguish qualified evidence, held proposals and scoped local reads | Frozen repeat-evidence roster, scoped read projection and one receipt for an exact owner-confirmed guidance revision | 0076 derivation |
| TASK-0151-REPLAY-CUSTODY | [0151](../specs/0151-site-lawbook-calibration-and-replay.md#requirements-and-planned-proof) verifies sealed receipt display and separately admitted offline recomputation under managed custody | TASK-0149-DECISION-REPLAY numeric bytes, exact 0148 consumed look, 0150 composite and three ordered producer proofs, 0032 ledger and 0118 custody, QR-006 replay profile | IT-0151-REPLAY-CUSTODY, IT-0151-QUIT-ERASURE and IT-0149-REPLAY prove matching results and independent read, replay, restore and cleanup fences | Exact bundle and consumed-look identities, canonical-byte comparison where admitted, and separate managed-removal and scoped display receipts | 0076 derivation |

Use the [scientific work trace][science-work] for the matching
requirement and test route. A prospective package supplies no
qualification to another candidate, source or platform.

[science-work]: test-and-traceability.md#scientific-experiment-work-packages

### Design packages for specs 0134, 0135, 0146 and 0147

These records define planned work for four specs. Apply 0076 to the
owning design, variants and proof before deriving an issue. Specs 0136 to
0145 are added by their owner. Synthetic fixtures supply no real source,
platform or phase admission.

| Task | Outcome and owning design | Depends on | Checks and acceptance | Specific done evidence | Forgejo export hints |
|---|---|---|---|---|---|
| TASK-0001-LOAD-NAMESPACE | Declare the load-gate namespace and its evidence profile in the 0001 store so 0134 receipts have a home, [0001](../specs/0001-door.md) amended for [0134](../specs/0134-ci-latency-load-gate.md) | Owner-approved amendment to spec 0001, accepted Phase 1 door | Store a signed workload, baseline and run receipt. Refuse a candidate writer. Refuse a raw sample vector with no admitted profile (protected store, readers, retention, cleanup) | The namespace declaration, profile fields and signed fixture receipts. Until it exists every 0134 receipt is unverified | 0076 derivation only |
| TASK-0134-LOAD-VERDICT | Compute the nearest-rank p95 verdict, pair votes and invalid-pair rules, [0134](../specs/0134-ci-latency-load-gate.md) | TASK-0001-LOAD-NAMESPACE, sealed runner (TASK-0003-SEALED-RUN) | UT-0134-P95 and IT-0134-NOISE: rank boundaries, equality, one breach then a later pair, two breaches confirm, runner faults give unverified, global time and attempt caps | A signed run receipt with every pair, vote and invalidity reason. A confirmed failure keeps its verdict | 0076 derivation only |
| TASK-0134-LOAD-TIERS | Admit the commit, push, staging and release workloads and receipts, [0134](../specs/0134-ci-latency-load-gate.md) | TASK-0134-LOAD-VERDICT, signed workload manifests, baseline signing in the long lane | E2E-0134-TRUST and E2E-0134-TIERS: missing and tampered receipts, wrong tier or baseline generation, waiver cases (31 days refused, none at staging or release), p99 review binding | Per-tier results, the baseline receipt and the review reference. A short receipt cannot clear a release | 0076 derivation only |
| TASK-0135-SPOOL-COLLECT | Record verified AI fetches after the response into the bounded spool and verify the agent, [0135](../specs/0135-ai-fetches-referrals-and-ranking.md) | 0021 paired transport, 0034 daily verifier refresh, 0118 listing of the four paired operations | IT-0135-SPOOL and IT-0135-FETCH: symbolic link, permissions, forged and oversized lines, concurrent appends at the cap, slow or refused write still serves the page, prefix cap, verifier expiry and revocation | Per-host timing against the proposed 1 ms p95 and 5 ms p99, coverage gaps, and the verifier receipt | 0076 derivation only |
| TASK-0135-REFERRAL-VIEWS | Count AI-assistant referrals and show the native AI traffic page, [0135](../specs/0135-ai-fetches-referrals-and-ranking.md) | TASK-0135-SPOOL-COLLECT, 0034 collector, 0093 host list, QR-065 for the visit floor | IT-0135-REFERRAL, E2E-0135-NATIVE and IT-0135-INSPECT: hostile referrers, suppression, partial 90-day view, role loss, offline expiry, keyboard and screen-reader parity | Sealed view receipts, coverage dates and the descriptor check for every shipped setting | 0076 derivation only |
| TASK-0135-RANK-FACT | Publish one verified-fetch fact and the bounded ordering signal, [0135](../specs/0135-ai-fetches-referrals-and-ranking.md) | TASK-0135-SPOOL-COLLECT, a 0014 registry entry for `ranking.ai_fetch_boost_bps`, a 0055 fact entry for `rank.ai_verified_fetch`, the 0027 evidence door | E2E-0135-RANK: default 0, an owner-set 5% and over-cap, 28-day expiry never renewed, search roles only, no stacked boost, 10% cap, tie, overflow, farming cases | The applied-weight trace for an exact result and the unchanged base order when the fact is missing | 0076 derivation only |
| TASK-0048-ACTION-ADDITIONS | Amend the Fix list for the four additions and ask 0044 for the `card`, `reactive` and `cooldown_class` signal fields, [0048](../specs/0048-fix-list.md) and [0147](../specs/0147-action-compiler-and-evidence-backed-tasks.md) | Owner confirmation of DR-140 | Check the amended state and freeze tables, the Fund exemption for the tracking card, snooze date and the registry gate that refuses a signal with no card rule | The amended 0048 and 0044 text and a registry check that fails an unnamed signal | 0076 derivation only |
| TASK-0146-FORECAST-STORE | Ingest stored Search Console, GA4 and darshak copies into per-site tables, build rollups, the confidence object and the sanity gate, [0146](../specs/0146-analytics-forecasting-and-text-behaviour-engine.md) | 0040 stored copies, 0034 aggregates, 0066 job numbers for the new fits, 0078 admission | UT-0146-GATE, UT-0146-SOURCES, IT-0146-EVENTS: all seven source combinations, missing source never zero, broken status withholds, hostile query text redacted | Per-source badges, confidence objects and a rebuild within the proposed budget on 100,000 and 1,000,000 rows | 0076 derivation only |
| TASK-0146-FORECAST-MATHS | Implement capabilities FC-01 to FC-32 with their minimums and false-alarm control, [0146](../specs/0146-analytics-forecasting-and-text-behaviour-engine.md) | TASK-0146-FORECAST-STORE | UT-0146-MATHS, UT-0146-MATHS2 and FX-0146-MEERA: hand-computed values, threshold equalities, the planted-null set, DST days, identical bytes on Linux and Windows | Each capability's result with its interval and sample label on the synthetic multi-year fixture | 0076 derivation only |
| TASK-0146-BLOCK-SIGNALS | Qualify darshak block reach, copy counts and link clicks, [0146](../specs/0146-analytics-forecasting-and-text-behaviour-engine.md) and [0034](../specs/0034-analytics.md) | Owner answer to QR-221 and an approved 0034 beacon change | IT-0146-BLOCKS and IT-0146-POISON: moved and orphaned blocks, counts of 4 and 5, complementary suppression, hostile beacons, the unmatched-hash bucket | The approved beacon footprint within 4 KiB and 32 entries, and the off state showing SUT-6707 | Blocked until QR-221 |
| TASK-0146-FORECAST-SAFE | Local MCP reads, fenced Python hooks, offload and erasure for the forecast data, [0146](../specs/0146-analytics-forecasting-and-text-behaviour-engine.md) | TASK-0146-FORECAST-STORE, 0104 fence, 0020 tools, 0030 ledger | IT-0146-SAFE, IT-0146-PY, IT-0146-OFFLOAD, QM-0146-GATE, QM-0146-ERASE, QM-0146-BUDGET and RP-0146 | Erasure proof for every file kind, owner-only switches off by default, and replayed Quint traces | 0076 derivation only |
| TASK-0019-OBSERVATION-EVENTS | Register the `ObservationEvent` contract (carried by the `ObservationEmitted` event), the other 0147 events (`CardOpened`, `CardUpdated`, `HoldChanged`, `VerificationFinished`, `OutcomeReady`) and the three 0146 events with spec 0019 and name them in each emitting spec, [0147](../specs/0147-action-compiler-and-evidence-backed-tasks.md) and [0146](../specs/0146-analytics-forecasting-and-text-behaviour-engine.md) | 0019 outbox, owners of the emitters in the 0147 ten-angle table | Send each event twice and expect one record. Refuse an unregistered event or a version mismatch | The registered event list and one emitter proof per angle | 0076 derivation only |
| TASK-0147-COMPILER-CORE | Compile typed observation events into root-cause cards, [0147](../specs/0147-action-compiler-and-evidence-backed-tasks.md) | TASK-0048-ACTION-ADDITIONS, 0019 outbox, emitters named in the ten-angle table | UT-0147-CATALOG, UT-0147-ROLLUP, IT-0147-ANGLES, IT-0147-EVENTS: one event per angle, 4, 5 and 400 pages on one cause, chunked events, rejected bad events, no dropped member | Cards and clusters whose member counts always match, within the proposed budget | 0076 derivation only |
| TASK-0147-PROOF-GATES | Attach three-source proof receipts and apply the data and rollout holds, [0147](../specs/0147-action-compiler-and-evidence-backed-tasks.md) | TASK-0147-COMPILER-CORE, TASK-0146-FORECAST-STORE, 0137 phases | UT-0147-EVIDENCE and IT-0147-GATES with FX-0147-SANITY and FX-0147-ROLLOUT: broken, caution and unchecked status, tracking card, reactive and non-reactive cards in a rollout, stale 0137 data | Receipts with per-source badges and holds that no waiver, sign-off, tool or script can lift | 0076 derivation only |
| TASK-0147-LIFECYCLE-OUTCOME | Verify fixes, apply cooldown and write the 28-day outcome receipt, [0147](../specs/0147-action-compiler-and-evidence-backed-tasks.md) | TASK-0147-COMPILER-CORE, 0044 check my fix, 0146 FC-20 | IT-0147-LIFECYCLE and IT-0147-OUTCOME with FX-0147-COOLDOWN: one active run, retries as clicks, a return during cooldown, frozen look-alike pages, confounded cases, identical bytes on Linux and Windows | Verification runs, cooldown dates and outcome receipts worded as observed change | 0076 derivation only |
| TASK-0147-BOARD-EXPORT | Focus 3, the task board, exports, local MCP tools and Python hooks, [0147](../specs/0147-action-compiler-and-evidence-backed-tasks.md) | TASK-0147-PROOF-GATES, TASK-0147-LIFECYCLE-OUTCOME, 0020 tools, 0104 fence | UT-0147-ORDER, UT-0147-EXPORT, IT-0147-MCP, IT-0147-PY, IT-0147-SAFE, QM-0147-LIFECYCLE, QM-0147-GATES, QM-0147-ERASE and RP-0147 | The board columns, three exports and three templates, owner-only hooks off by default, and replayed Quint traces | 0076 derivation only |

Use the [work-package trace][design-packages] for the matching
requirement and test route.

[design-packages]:
  test-and-traceability.md#specs-0134-0135-0146-and-0147-work-packages

### Known gaps before Phase 6 admission

These gaps in the staged documentation are recorded here so none is only
implied. Each names an owner and what closes it. They block admission of the
named specs to implementation under spec 0076, not this documentation commit.

| Gap | Owner | Closes when |
|---|---|---|
| Feature register has no rows or "adds no feature" statement for specs 0134 to 0154. Specs 0153 and 0154 need the label "separate scope decision, no version 1 owner" (DR-154, QR-238, QR-239) | Codex accountable (0136 to 0145, 0148 to 0154); Claude contributes 0134, 0135, 0146, 0147 | Each spec has a register row or statement |
| Implementation catalogue and test-and-traceability have no rows for 0136, 0138, 0140 to 0145 and 0152 to 0154 (0153 and 0154 may say "no version 1 task") | Codex | The rows land |
| Spec 0136 has no limits and footprint section; spec 0153 names its operating limits without values | Codex | Each states proposed values and cites spec 0078 |
| Spec 0148 PreRegistered and Balanced plans have no expiry for an unactivated plan | Codex | A row moves a stale plan to Aborted |
| Specs 0153 and 0154 name no owning module, store or event path; 0149 and 0151 ERR ids have no SUT mapping or "no new public code" line | Codex | One line each |
| FX-0139-ENTRY-3A, ENTRY-3B, ENTRY-PHASE-4, ENTRY-PRODUCT, INITIAL-PHASE-2, ISOLATION and STALE-WORK are defined only in test-and-traceability | Codex | Spec 0139 lists them as proposed |
| TASK-0048-ACTION-ADDITIONS and the 0044 signal fields (`card`, `reactive`, `cooldown_class`) | Claude drafts; the owner confirms DR-140 | Specs 0048 and 0044 carry the text |
| Specs 0148 and 0151 publish lifecycle and receipt events through spec 0019 that are not registered there; spec 0148 assigns Alekh (0043) a projection that 0043 does not accept | Codex | 0019 lists the events and 0043 accepts the projection |
| Spec 0135 changes the darshak collector (first-party referrer input, 2,048-byte rejection) but spec 0034 carries no such text | Codex | 0034 declares the input and its limit |
| Modules and crates for 0146 and 0148 to 0151 have no module or glossary names (spec 0015) | Claude (0146), Codex (0148 to 0151) | Each names its module and crate |
| 0149 bisection doubling has no explicit iteration cap; 0136 and 0152 hold dispatch on Retry-After with no maximum (0137 now clamps it) | Codex | Each states a cap |
| The AGENTS.md 7.2 waiver gate has no task: the gate must count waivers per check id, carry the count through an owner-signed catalogue change and apply the signed-time rule (BJ-29, UT-BJ-23, DR-163) | Claude | A task is filed at admission and carries DR-163 |
| The shared `sutradhar-agent` Forgejo account has no task. A "Later" Forgejo task covers: the account, scopes, keyring helper and `main` protection of ADR-0008, then proof that a push failing the push tier, a force-push, a settings change, a push to another branch, a commit or branch made through the Forgejo API, a pushed `v*` tag that is not new or names a commit not on `main`, and a `v*` tag pushed with the commit it names are each refused, and that a valid release tag is accepted (DR-162). Spec 0008 (trust boundaries) and AGENTS.md section 11 also need a line naming the shared account (DR-161, QR-254) | Owner makes the account; Claude records the proof | The owner confirms QR-254 and the proof is posted in Forgejo |
| A `v*` release tag or release made or changed through the Forgejo API with the agent token does not pass through the door. The watchman flags the change within five minutes, but the sync may copy it to GitHub first (DR-162) | Claude | The sync holds back a tag or release the door and release step did not make, and proof covers an API tag on a commit off `main`, an API delete then re-create, and an API change to a release's files |
| The clean-room Notion-equivalent workspace is inside version 1 (DR-165) but has no scope statement, spec series, phase, owning module or feature-register rows, and ADR-0021's feature count does not yet include it | Owner scopes it; Claude drafts | A scope statement, a spec series and a phase are written and ADR-0021 and the feature register carry it |
| The agent harness (spec 0155) has its request parser, its push checks and its task and mailbox rules on plain data in the crate `agent-harness`; it still needs catalogue and test rows for every later part (the git fetch and push, the log and the limits, and the service), and the locked agent accounts of AGENTS.md section 11 (task #166) | Claude drafts; the owner makes the accounts | Spec 0155 is admitted with catalogue and test rows for each part, the parts are built, and the accounts exist |
| The release signing key (DR-166) has no release step that uses it, no use log and no reconciliation with spec 0065's release-key authority | Claude drafts; Codex builds the release step | The release step signs with it, logs each use and 0065 and this key agree on one authority |
| `.claude/skills/architecture/SKILL.md` still says "Sutradhar Boot" | Rule maintainer | Renamed to Behemoth Boot |

## Phase 7 release slices

The release task uses one exact candidate from Phase 6. DR-025,
DR-056 and DR-069 bind approval, the method floor and the long lane.
QR-101 tracks proof of the exact release and synthetic judge scope.
Owner-signed waivers have separate authority.

| Task | Outcome and owning design | Depends on | Checks and acceptance | Specific done evidence | Forgejo export hints |
|---|---|---|---|---|---|
| TASK-0064-QUALIFICATION-PLATFORM | Verify all feature, variant and protected evidence under one exact candidate, [0064](../specs/0064-v1-qualification.md), item 1 | Phase 6 and signed evidence store | Prove the exact inventory, ownership, provenance and declared variant set. Missing, duplicate, stale, wrong-purpose and out-of-budget evidence refuse | Canonical signed ledger and E2E-0064-RELEASE acceptance with IT-0064-REPLAY evidence | #64 and #189 are earlier qualification work |
| TASK-0055-METHOD-ADMISSION-PROOF | Verify distinctness and entitled use of ranking methods, [0055](../specs/0055-method-research-catalogue.md), item 1 | Exact candidate, method lab and QR-101 | Refuse a parameter-only duplicate or missing provenance. Require 12 qualified methods in each of 30 families and at least 360 in total | Exact catalogue and independent distinctness, legal and performance receipts | #36 method admission. #185 is history only |
| TASK-0057-EVALUATION-PLATFORM | Produce the protected ranking evaluation, [0057](../specs/0057-quality-judging.md), item 1 | Synthetic site inventory, frozen protocol and QR-101 | Exercise metric boundaries and omitted labels. A failed independent floor or insufficient data refuses release | Paired results, baseline identity and signed protected outcomes | #205 |
| TASK-0077-VERIFIED-CHAMPION | Demonstrate protected promotion and exact rollback, [0077](../specs/0077-autonomous-ranking-optimisation.md), item 1 | Ranking evaluation and admitted schedule classes | E2E-0077-CHAMPION-CHALLENGER covers hold, promotion and rollback. A stale decision cannot activate an old generation | Signed transitions and complete rollback manifest | Phase 7 item 1 |
| TASK-0114-LONG-LANE | Verify the candidate under full fuzz, load, soak, chaos and private benchmark scope, [0114](../specs/0114-behemoth-verification.md), item 1 | Exact candidate and admitted target profiles | Preserve required run length and fault cases. Stop or inconclusive evidence gives no pass | Named environment, complete long-lane and measurement receipts | #178 is a profile evidence input |
| TASK-0116-REPRO-MATRIX | Verify shared matrix and reproducible artefacts for both crown jewels, [0116](../specs/0116-leviathan-scale-storage-and-verification.md), item 1 | Trusted pinned build and compatibility inputs | Run LS-27 and LS-28 with IT-LS-05 and IT-LS-06. Build twice on the same pinned image and once on a second pinned image. Refuse mixed input, target gaps or differing bytes. Join candidate proof to IT-0005-RELEASE-APPROVAL | Target matrix, byte and dependency comparison and exact report digest in the 0003 release evidence bundle | Phase 7 item 1 |
| TASK-0005-RELEASE-CANDIDATE | Create the checked version commit and bind a candidate tag to one durable run, [0005](../specs/0005-release-coordination.md), item 1 and [0005 frozen inventory identities](../specs/0005-release-coordination.md#frozen-inventory-requirement-identities) | Scope preflight on main, commit and push tiers, authorised release action | Refuse an arbitrary tag, a wrong commit or an attempt to reuse evidence after change. Resume the exact recorded run under the same identity | Exact version change, commit and push receipts, tag and run identity | #38 addresses the earlier release process |
| TASK-0005-FULL-RELEASE-RUN | Execute the fresh release tier for that exact candidate, [0005](../specs/0005-release-coordination.md), item 1 | Candidate tag and admitted evaluation and resource fixtures | Require full fresh quality and resource results and the supervised 72-hour soak. A crash resumes only under the authorised run record. Missing or changed evidence refuses | Complete signed release run and individual gate outcomes | #38 process, #178 profile evidence |
| TASK-0004-RELEASE-MANUAL-ASSET | Freeze one exact version of the public manual, [0004](../specs/0004-manual-publishing.md), item 2 | In-app content and exact candidate record | Check links, accessibility, private data and version binding. A prepared asset or candidate preview cannot move latest | Built bytes and hash for the exact candidate | #38 |
| TASK-0004-PUBLIC-MANUAL | Deploy the qualified frozen asset to GitHub Pages and read it back, [0004](../specs/0004-manual-publishing.md), item 2 | Published qualified release, mirrored exact asset and authorised Pages route | Run the sync workflow before the asset exists and verify latest stays put. Refuse wrong bytes. After the verified mirror, the version URL and latest resolve to the exact asset | Versioned URL, latest read-back and verified deploy identity | #38 |
| TASK-0065-RELEASE-BINDING | Check signed manifest and all packaged artefacts for the candidate, [0065](../specs/0065-tijori.md), item 3 and [0005 frozen inventory identities](../specs/0005-release-coordination.md#frozen-inventory-requirement-identities) | Passing candidate and signing custody | Substitute one payload or signing purpose. Verification fails before promotion. Every required platform payload is covered | Manifest, package hashes and signed verification receipts | #179 |
| TASK-0005-RELEASE-PUBLISH | Publish the exact approved release and mirror its tag and notes once, [0005](../specs/0005-release-coordination.md), phase exit | All independent gates, QR-101 and current approval | Lose the reply after publishing a draft artefact. Reconcile the exact result. A failed or changed candidate cannot publish or move latest | Final Forgejo and mirror references plus the signed approval for the exact candidate and manifest | #38, #179 |
