# 0081: Behemoth Boot parity and lockstep

Behemoth Boot is our Rust application framework, the new name of Sutradhar
Boot. It follows Spring Boot: every part of the shipped product is built on
it, and it keeps pace with Spring Boot and the Spring projects in its release
train. This specification owns the rules that keep it aligned: the version
lockstep, the monthly scout, the parity ledger, the clean-room procedure and
the release block. It does not describe each framework feature. Each feature
group gets its own specification in later slices; this one says how every
such feature is found, specified, built, proved and tracked (ADR-0022).

Example: on the first of the month the scout reads the official Spring Boot
release notes and sees that a new release adds a setting that makes the
application fail at start-up on outdated keys. It adds one ledger row, with
the note's address and date, and writes a plain behaviour description: "With
the setting on, start-up stops with a list of every outdated key". A separate
implementer builds the Rust behaviour from that description alone. The row
moves to "matched" only when the contract test for that description passes.

## What it does

### Words used here

- **Train**: the Spring projects that release together with Spring Boot, as
  the owner lists them: Framework, Security, Data, Batch, Integration, GraphQL,
  Kafka, AMQP, Session, LDAP, Web Services, gRPC, AI, Modulith and Cloud.
- **Upstream release**: one official release of a project in the train.
- **Parity item**: one observable behaviour, setting, default, deprecation or
  removal found in an upstream release.
- **Parity ledger**: the list of every parity item and its state.
- **Scout**: the monthly agent that reads public release material and writes
  parity items and behaviour descriptions.
- **Behaviour description**: a plain statement of what a user or program can
  observe, with examples and edge cases, and no upstream source text.
- **Implementer**: the agent that builds the Rust behaviour from a behaviour
  description.
- **Starter**: a Behemoth Boot crate that wires one capability.
- **Not applicable**: a parity item that cannot apply to a Rust application,
  with an approved reason.

### Version lockstep

1. A Behemoth Boot version has the number of the Spring Boot release it
   matches. Behemoth Boot 4.2.0 matches Spring Boot 4.2.0.
2. A release candidate or milestone upstream may have a matching Behemoth Boot
   pre-release. A final release follows the verified parity and waiver rules
   under [Release block](#release-block).
3. Patch releases follow: a Spring Boot 4.2.1 means a Behemoth Boot 4.2.1 that
   covers the new items.
4. Behemoth Boot's own fixes that have no upstream counterpart use a fourth
   number, such as 4.2.0.1, and never change a matched behaviour silently.
5. Proposed windows (design choices, to be revised when the first cycles are
   measured): a patch release within 14 days of its upstream release, a minor
   release within 60 days. A release past its window blocks further Behemoth
   Boot releases until it is matched or the owner signs a waiver.
6. The research dossier records the upstream versions in force: Spring Boot
   4.2.0 was released on 19 November 2026 (RQ-015). That is the first target.

### The monthly scout

1. On the first of each month an agent runs the scout procedure. A run that
   cannot start is reported to the owner in plain English, never skipped.
2. Inputs, all public: the official release notes and migration guides of
   every project in the train, the release lists with milestones and release
   candidates, configuration changelogs and the security advisories.
3. Output for every upstream release since the last run: one parity item per
   addition, change, deprecation and removal, each with an official address,
   a retrieval date and a plain behaviour description. A change with no
   behaviour a user can see is recorded as "internal, not observable" and
   needs no work.
4. An item the scout cannot confirm from an official source is marked
   "unverified" and stays out of the ledger's counts until confirmed. This
   keeps wrong claims, such as a feature that does not exist, out of the
   ledger.
5. The scout lists every item whether or not Sutradhar uses it. The owner's
   decision is that all of them are built, so there is no "unused" state.
6. The scout links one task draft to each source-confirmed applicable
   item or authorised small group under N-1729. Tasks are cut to one
   slice (about 150k tokens). Derive a build task only after 0076
   verifies the exact `specified` row, complete design, dependencies
   and current signed predecessor. Use the existing work-list
   authority for submission. Scouting and drafting confer no readiness
   or implementation authority.

### The clean-room procedure

1. The scout and the implementer are different agents and never share a
   session. The scout reads only official documentation, release notes,
   reference guides and observable behaviour. Nobody in the chain reads
   upstream source code.
2. A behaviour description holds no source text and no copied wording beyond
   short names of settings and keys, which are interface facts.
3. The implementer receives the behaviour description, the Rust idiom map and
   the existing code, and nothing from upstream.
4. A written record keeps, for each item: who scouted it, what they read, the
   description's hash, who implemented it, and what they received. The record
   is kept for the life of the project.
5. A claim of independent implementation is made only with that record.
   Spring being open source does not change the procedure; the owner chose it.
6. If separation cannot be shown for an item, the item is marked "not
   independent" and no clean-room claim is made for it.

### The Rust idiom map

Parity means equal observable behaviour, not equal syntax. The map says how
Java ideas become Rust, so two implementers choose the same shape.

| Spring idea | Behemoth Boot shape |
|---|---|
| Annotated components found by scanning | Explicit registration in the composition root, with typed dependency injection and no hidden reflection (AGENTS.md 5.1) |
| Conditions on beans and settings | Compile-time features and boot-time checks that give the same on or off decision and the same plain-English report |
| Auto-configuration classes | One starter crate per capability, wired by the composition root |
| Property binding with relaxed names | Typed settings structs with serde, the same name forms and the same precedence |
| Profiles | The `dev`, `test` and `prod` profiles and profile groups |
| Actuator endpoints | The same endpoints and their output shapes, served on the local link or local MCP |
| Starters and BOM | A workspace of starter crates with one list of pinned versions |
| Declarative HTTP clients | Attribute macros generating typed clients |
| Repositories from method names | A derive macro building typed queries at compile time |

An item that has no sensible Rust shape is "not applicable" and needs the
owner's recorded approval of its reason.

### Reuse inside Behemoth Boot

Our own code is the framework layer. HTTP servers, SQL and NoSQL drivers,
serialisation, messaging clients and TLS are mature crates behind starters
(ADR-0022, AGENTS.md section 6 rule 14). A starter wraps a crate in the same
behaviour the parity item describes. Starters are final, not throwaway.

### The parity ledger

A row has these fields: identifier (`PB-` plus release, project and a number),
upstream project and release, item kind (addition, change, deprecation,
removal, security fix), title, official address and retrieval date, behaviour
description (hash and place), Behemoth Boot module, mature crate used (if
any), owning specification, contract test identifiers, state, and the upstream
release it was last checked against.

For each applicable profile, link the row to its owning requirement,
variant and planned contract-test fixture. Record approved reasons for
non-applicability under the existing process. At `specified`, the exact
source, design and fixture are task inputs under spec 0076. A passing
result for the task's own implementation comes later. At `matched` and
`verified`, bind the applicable case and result to the same row, variant,
candidate and target profile under the existing promotion checks. A result
for another cell supplies no proof, even when its test identifier matches.

The normal promotion order is `scheduled`, `specified`, `in_progress`,
`matched`, `verified`. Side states: `unverified`, `internal`,
`not_applicable` (with approved reason), `deprecated_upstream` (tracked
until upstream removes it, then `removed`). The ledger tool checks and
records each transition. Passing code and contract tests are required for
`matched`. Independent review and the applicable benchmark and
compatibility checks are required for `verified`.

Apply the existing failure transitions below through the same ledger
tool. Return an ambiguous description to `specified`. Further work
still needs a complete owning design under 0076. A failed contract
check after an upstream upgrade returns a verified row to
`in_progress` under 0114 BJ-22. Keep the earlier signed proof with its
original inputs. It cannot qualify the affected row against the changed
upstream input. Requalification uses fresh applicable checks and
review. Any protected phase replacement follows spec 0010.

Rules:

1. Deprecations and removals follow upstream's schedule. When upstream removes
   a feature, Behemoth Boot removes it in the matching release.
2. A security advisory creates an item at once, handled outside the monthly
   cycle, with a matching fix checked in the Rust equivalent.
3. A configuration changelog diff compares every key and default of a new
   release with Behemoth Boot's metadata and creates items for any missing,
   renamed or changed one.
4. The ledger is text in the repository, one file per upstream release, with a
   generated summary. The summary shows counts by state and project. It never
   states how long anything will take.
5. Deleting a row is not allowed; a wrong row is marked `unverified` or
   `internal` with the reason.

### Release block

A Behemoth Boot release is refused when: any item of its upstream release is
neither verified nor not applicable with an approved reason; the release is
past its lockstep window; a row lacks an official source and date; a clean-room
record is missing for an item claimed as independent; or the meaningful-line
gate (spec 0083) fails. Only the owner's signed waiver can allow an exception,
and a waiver names the rows and expires in 30 days or less (BJ-29 of
spec 0114; AGENTS.md section 7.2).

The Phase 3B handoff to Sutradhar requires complete verified
framework coverage under spec 0139 and ADR-0025. Product use starts
after the signed handoff for the exact qualified framework. A
release waiver cannot fill a missing applicable phase row or
platform proof.

### Framework groups and their specifications

These stable group codes map PB-A to this specification and
PB-B through PB-J in order to specs 0106 through 0114. Each owner
keeps its scope. The mapping gives no qualification credit for
an upstream train, an applicable row or a runtime profile.

| Group | Scope |
|---|---|
| PB-A | Lockstep, scout, ledger and migration tools (this spec) |
| PB-B | Container, wiring, auto-configuration, conditions, settings, profiles, lifecycle, start-up |
| PB-C | Web servers, routing, API versioning, clients, gRPC, WebSockets and streaming, GraphQL, hypermedia, serialisation, SOAP |
| PB-D | Data access: repositories, mapping, drivers and pools, transactions, migrations, caches |
| PB-E | Security and identity: filter chain, OAuth 2.0, OpenID Connect, SAML, LDAP, passkeys, method security, SSL bundles, sessions |
| PB-F | Messaging, events, integration, outbox, scheduling, resilience, state machines, locks |
| PB-G | Batch, cloud patterns, AI and MCP, mail and templates |
| PB-H | Actuator, health, metrics, tracing, structured logs, failure analysers |
| PB-I | Developer tools: reload, composition tools, test slices, container images, metadata, upgrade doctor |
| PB-J | Verification: parity contract tests, Quint models, fuzzing, matrices, benchmarks, chaos |

For PB-D, use the [0108 scope contract][pb-d-scope] and its
BD-20 and BD-21 cases for the separated setup and migration contexts.
IT-BD-06 and UT-BD-14 check the migrator history boundary. The
transaction and pool cases remain under BD-07, BD-09, BD-21 and
BD-22. A synthetic host can exercise the Sutradhar profile while
qualification evidence remains limited to its exact fixture and
admitted profile. Product integration follows the signed 3B
handoff and the owning 3C task under specs 0076 and 0139.

[pb-d-scope]: 0108-behemoth-data-access.md

Items the dossier found deprecated upstream, such as Apache Pulsar support in
4.2.0, are tracked as `deprecated_upstream` and are not built.

#### Upstream scouting and parity evidence (N-1727–N-1734)

Use the existing monthly scout, clean-room procedure, parity ledger
and release block. Treat upstream text as inert research material.
Retain source identity, retrieval date and exact mapped release.
Names and versions in the owner's catalogue are research leads for
the current owning process. A report, diff or unresolved finding
cannot create a live runtime change, deployment or release.

| Owner catalogue entry | Exact parity scope |
| --- | --- |
| N-1727 Monthly upstream scout agent | Use the existing first-of-month run and next-day failure route with one durable occurrence and per-source cursor. Read only admitted public official materials for each pinned project and release train with dated official provenance. Record additions, changes, deprecations, removals and internal or unverified observations by release. Missing evidence is reported and holds the atomic per-source cursor until complete admitted coverage. [FX-0081-W5-UPSTREAM-SCOUT-EXACT-OCCURRENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1719-to-n-1742). |
| N-1728 Portfolio-wide train tracker | Project the exact verified upstream release-train manifest, including separately scoped projects and version relationships. Keep candidate, final, withdrawn, unverified and previous observations distinct. Do not infer compatibility, raise dependency versions or grant a new upstream source from a name in the catalogue. [FX-0081-W5-UPSTREAM-TRAIN-VERIFIED-RELATIONSHIPS](../planning/feature-wave-5.md#acceptance-design-for-n-1719-to-n-1742). |
| N-1729 Automatic work-ticket scheduling | Use the existing parity item-to-task route for a source-confirmed applicable item or authorised small group. Bind release, behaviour hash, owner spec and independent acceptance intent. Generate one idempotent task draft. Use 0076's exact row, design, dependency and signed predecessor checks before deriving a build task. Check separate work-list authority before submission. An acceptance fixture uses a fake sink and receipt only. A lost reply uses the same identity for reconciliation. A ticket gives no implementation, release or new scope. [FX-0081-W5-PARITY-TASK-FAKE-IDENTITY-RECONCILIATION](../planning/feature-wave-5.md#acceptance-design-for-n-1719-to-n-1742). |
| N-1730 Zero-drift compliance scoreboard | Derive the existing local parity summary from exact release rows and actual evidence, with counts for each state and explicit incomplete or unverified cohorts excluded from qualified counts. Release decisions use the existing gate and signed exception path. No public publish, invented parity, unlocked tier or status authority is created by the view. [FX-0081-W5-ACTUAL-PARITY-SCOREBOARD-EVIDENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1719-to-n-1742). |
| N-1731 Clean-room wall enforcement | Keep scout, implementer and independent reviewer distinct under the existing role rules. Hand over only the sanitised behaviour record, typed interface facts, exact provenance and local allowed references. Official docs supply the evidence. No upstream code, program execution, copied text or prompt control passes the wall. Missing or crossed separation holds an independence claim. [FX-0081-W5-CLEAN-ROOM-EXACT-ROLE-SEPARATION](../planning/feature-wave-5.md#acceptance-design-for-n-1719-to-n-1742). |
| N-1732 Synchronised deprecations and removals | Record the verified upstream milestone, affected local setting, endpoint or module, matching release and migration implications in parity evidence. Bind the existing reviewed candidate and compatibility tests before any removal. Do not rewrite live settings, remove running code or treat an unverified date as a release decision. [FX-0081-W5-MATCHING-RELEASE-REMOVAL-EVIDENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1719-to-n-1742). |
| N-1733 Upstream security advisory mirror | On receipt of an admitted verified official advisory, create the parity item through the existing immediate route outside the monthly cycle. Record the safe affected-range and local applicability decision, and use bounded defensive regression and release review. No offensive tests, new background service, or automatic patch or publication follows from the advisory. [FX-0081-W5-VERIFIED-ADVISORY-RECORD-REPLAY](../planning/feature-wave-5.md#acceptance-design-for-n-1719-to-n-1742). |
| N-1734 Automated configuration changelog diff | Compare a verified upstream release's configuration metadata with the exact owning local schema. Report typed additions, removals, renames and changed defaults only with evidence. Unknown or ambiguous mapping stays unverified and secret values stay masked. An accepted diff can feed the reviewed 0106 migration table and existing task route. It cannot alter settings or enable migration. [FX-0081-W5-CONFIG-DIFF-TYPED-MASKED-EVIDENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1719-to-n-1742). |

QR-213 qualifies the source and release identities, clean-room role
separation, deduplicated task drafts, applicability evidence and refusal
of unreviewed or unauthorised effects.

#### N-1839 Mirror-image version numbering

Bind the exact Behemoth version claim to its verified upstream
release and scoped parity ledger. Keep candidate and final labels
distinct and retain the existing fourth-segment rule for local-only
fixes. Missing evidence, withdrawn upstream or an unmet release
rule holds the claim. A version label neither states parity by
itself nor releases or deploys a build. Keep the phase and signed
release authority in the roadmap and owning gate.
[N-1839](../planning/feature-wave-5.md#acceptance-design-for-n-1839-to-n-1862)
uses fixture `FX-0081-W5-VERSION-CLAIM-EXACT-UPSTREAM-EVIDENCE`.

### Data

The ledger and clean-room records are repository files, not database tables.
The generated summary is a text file with counts by project, release and
state. The monthly scout writes a dated run note: what it read, how many items
it added and which it could not confirm.

### Settings

The release windows are design choices to be revised when the first cycles
are measured. The names and ranges are proposed. All live in the pipeline
settings file with those of spec 0114.

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `parity.patch_window_days` | days | 14 | 1 to 90 (proposed) | next check |
| `parity.minor_window_days` | days | 60 | 7 to 180 (proposed) | next check |
| `parity.scout_day` | day of month | 1 | 1 to 28 (proposed) | next month |
| `parity.slice_tokens_k` | k tokens | 150 | 50 to 300 (proposed) | next task cut |

There is no setting that turns the release block off, lets one agent do both
scout and implementer roles, reads upstream source code, or deletes a row. A
waiver is a signed owner record naming the rows, expiring in 30 days or
less (BJ-29 of spec 0114; AGENTS.md section 7.2), not a setting.

### Failure behaviour

| Failure | Effect | Recovery |
|---|---|---|
| The scout cannot reach an official page | The run note names the page; items from it stay unverified | Retry the next day; the owner is told |
| Upstream withdraws or renames a release | Rows are re-pointed or marked `internal` with a note | Scout re-checks |
| A behaviour description is ambiguous | The implementer asks; the item returns to `specified` | Scout rewrites it |
| A contract test fails after a Spring upgrade | The row drops from `verified` to `in_progress` | Fix or revise the description |
| The window passes with open items | Releases are blocked | Match the items or the owner signs a waiver |

### Failure codes

The codes below are proposed. Codes SUT-4800 to SUT-4899 are proposed as the
range reserved for this spec; a decision record must confirm it first.

| Code | Meaning |
|---|---|
| SUT-4800 | Scout run could not start or reach an official page |
| SUT-4801 | Row refused: no official source or retrieval date |
| SUT-4802 | State move refused: evidence missing (for example no tests) |
| SUT-4803 | Row deletion refused |
| SUT-4804 | Not-applicable row refused: no recorded owner reason |
| SUT-4805 | Clean-room record missing; item marked "not independent" |
| SUT-4806 | Release refused: an item is neither verified nor not applicable |
| SUT-4807 | Release refused: past its lockstep window |
| SUT-4808 | Version number does not equal the matched Spring Boot release |
| SUT-4809 | Contract test failed; row dropped from `verified` |

### Security and privacy

- The scout reads only public official material. Nobody in the chain reads
  upstream source code, and a behaviour description holds no source text.
- The scout and the implementer never share a session; the clean-room record
  is kept for the life of the project.
- A security advisory creates an item at once, outside the monthly cycle.
- Ledger files and run notes hold no secret, credential or personal data.
- Only the owner's signed waiver, naming the rows and expiring in 30 days
  or less (BJ-29 of spec 0114; AGENTS.md section 7.2), allows a blocked
  release.

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| PB-R01 | A Behemoth Boot version number MUST equal the Spring Boot release it matches. | UT-PB-01 |
| PB-R02 | The scout MUST run monthly and report a failed run to the owner. | BD-PB-01 |
| PB-R03 | Every ledger row MUST have an official source and a retrieval date, or be marked unverified. | UT-PB-02 |
| PB-R04 | A row MUST pass its contract tests to reach `matched`. A failed contract test after an upstream upgrade returns a verified row to `in_progress` under 0114 BJ-22. | UT-PB-03, 0114 UT-BJ-18 |
| PB-R05 | The scout and implementer MUST NOT share a session, and no source code MUST be read. | AU-PB-01 |
| PB-R06 | A clean-room claim MUST have the written record. | AU-PB-02 |
| PB-R07 | A release MUST be blocked under the rules in "Release block". | UT-PB-04, BD-PB-02 |
| PB-R08 | A removal upstream MUST become a removal in the matching release. | BD-PB-03 |
| PB-R09 | A security advisory MUST create an item at once. | BD-PB-04 |
| PB-R10 | A deleted row MUST be refused. | UT-PB-05 |
| PB-R11 | A not-applicable row MUST carry the owner's recorded reason. | UT-PB-06 |

## How it is tested

Designed, not run.

- **Unit (UT-PB):** use the ledger tool to check promotion and the
  existing failure transitions. Check absent source, row deletion,
  version number and just-inside and just-outside window values.
  UT-PB-03 first rejects promotion without passing contract tests.
  Then give a verified row a failed contract check after an upstream
  upgrade. Only the affected row returns to `in_progress`. Keep the
  original signed references. Hold a direct or replayed claim of
  `verified` for the changed input until fresh proof passes.
- **Behaviour (BD-PB):** the monthly scenario above; a missed run; a removal
  followed through; an advisory handled outside the cycle; a release blocked
  and then released after a waiver.
- **Audit (AU-PB):** a sample of items is traced from official source to
  description to code to test, and the record shows the two roles were
  separate. The auditor is a reviewer who wrote neither.
- **Property:** the summary counts always add up to the number of rows.
- **Contract tests:** each parity item brings its own, written from its
  description by someone other than the implementer, so the oracle is
  independent of the code.

## Changes from the source

ADR-0002's small framework becomes a full Spring-parity framework (ADR-0022).
The old name Sutradhar Boot is retired. Wording about specific Spring features
in the owner's lists is tracked as unverified until the scout confirms it.
