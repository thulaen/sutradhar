# Acceptance ledger

Status: PLANNED. No row below is a product-completion claim.

This ledger identifies the evidence needed for the full objective. The current
requirements are R001–R082 in [current decisions](current-decisions.md); exact
source details remain in the [interview](sources/README.md) and
[source-item inventory](source-items/index.yaml). Feature specifications
must decompose compound source lines into testable cases before
implementation.

## Evidence contracts

Every acceptance packet identifies the exact source tree, dependency/model/data
versions, configuration, environment, commands, results, review authorship and
limitations. It records failed and skipped checks as well as passes. Public
packets contain only privacy-reviewed summaries. Sensitive evidence stays on the
authorised validation host.

An evidence link must prove the stated scope. A small example does not prove a
million-post workload; synthetic tests do not prove real-data quality; source
review does not prove installation; a legal source citation does not prove
rights clearance; an existing configuration does not prove runtime enforcement.

## Required evidence by requirement

- **R001**. Evidence that must exist: Qualified PHP 8.5/XenForo 2.3 installation
  and supported-boundary integration tests on Linux. Current state: UNKNOWN.
- **R002**. Evidence that must exist: Qualified large-forum workloads and a
  beginner completing guided setup without command-line work. Current state:
  PLANNED.
- **R003**. Evidence that must exist: Relevance assessments and separately
  reported usefulness/search outcomes; playbook comparisons. Current state:
  PLANNED.
- **R004–R006**. Evidence that must exist: At least 6,000 distinct qualified
  method packets, complete scope matrix and more than 10,000 meaningful passing
  playbook cases. Current state: PLANNED.
- **R007**. Evidence that must exist: Owner approval of each concrete feature
  specification; behaviour examples, test-first record and linked docs. Current
  state: PLANNED.
- **R008**. Evidence that must exist: Initial backlog progress, checkpoints and
  new/edit event tests, including interruption and resume. Current state:
  PLANNED.
- **R009**. Evidence that must exist: Permission and publication tests proving
  no v1 author-editor/automatic insertion path. Current state: PLANNED.
- **R010–R014**. Evidence that must exist: Property and integration tests for
  exact-text preservation, caps, duplicate targets, BBCode, guest visibility and
  optional destinations. Current state: PLANNED.
- **R015**. Evidence that must exist: Qualified 30-second first-result
  measurements and proof that approval is blocked until required applicable work
  completes. Current state: PLANNED.
- **R016–R018**. Evidence that must exist: Stale-content, expiry, atomic
  insertion, independent-edit-preserving undo, removal suppression, rejection
  and feedback tests. Current state: PLANNED.
- **R019**. Evidence that must exist: Scheduler traces showing every applicable
  method; explicit missing-input/fault states and blocked dependent outputs.
  Current state: PLANNED.
- **R020**. Evidence that must exist: Power/process interruption tests with
  durable completed work and bounded repeated unfinished work. Current state:
  PLANNED.
- **R021–R023**. Evidence that must exist: Combined process/native/cache
  resource accounting, PHP differential peaks, refusal/degradation and
  hard-limit crash recovery. Current state: UNKNOWN.
- **R024–R025**. Evidence that must exist: Controlled tuning experiments,
  bounded steps, positive weights, atomic settings versions and targeted
  rollback. Current state: PLANNED.
- **R026**. Evidence that must exist: Notebook-to-production import checks,
  headless notebook results and a ban on production importing lab code. Current
  state: PLANNED.
- **R027–R028**. Evidence that must exist: Independently reviewed package/model
  admission records, lazy-loading measurements and rejection of executable model
  formats. Current state: UNKNOWN.
- **R029**. Evidence that must exist: Current source-backed Adobe capability
  matrix with implementation and acceptance proof for every parity claim.
  Current state: PLANNED.
- **R030–R032**. Evidence that must exist: Consent/withdrawal, deduplication,
  retention, verified upload, outage queue and storage migration tests. Current
  state: PLANNED.
- **R033–R034**. Evidence that must exist: Connector provenance/coverage tests,
  quota scheduling, crawler limits and proof that prohibited SEO writes cannot
  occur. Current state: PLANNED.
- **R035**. Evidence that must exist: Rights/source review and knowledge/tag
  ambiguity, freshness, length, originality and batch-approval tests. Current
  state: UNKNOWN.
- **R036–R038**. Evidence that must exist: Ranking fixtures and perturbation
  tests, public karma recalculation, guest/reaction policy and moderator-only
  abuse actions. Current state: PLANNED.
- **R039–R040**. Evidence that must exist: Hub/summary/draft review flows, safe
  authorship, publication permission, stale-summary and novelty evidence.
  Current state: PLANNED.
- **R041–R044**. Evidence that must exist: Glossary, core/module boundary and
  action-interface parity tests; schema validation and permission-denial
  evidence. Current state: PLANNED.
- **R045–R046**. Evidence that must exist: Qualified agent adapters,
  simultaneous-writer refusal, stale-lease fencing, preserved handover and
  private SQLite recovery. Current state: UNKNOWN.
- **R047–R048**. Evidence that must exist: Rendered screenshots,
  interaction/browser tests, contrast and accessibility evidence for all owned
  screens. Current state: PLANNED.
- **R049–R050**. Evidence that must exist: Clean
  installation/upgrade/rollback/uninstall using the manual, including optional
  services and explicit backup override limits. Current state: PLANNED.
- **R051–R052**. Evidence that must exist: Fault-code catalogue coverage,
  scrubbed support exports, alert grouping and privacy deletion through every
  retained data path. Current state: PLANNED.
- **R053**. Evidence that must exist: A mapped case for every applicable Q148
  edge case, with starred-case release blocking demonstrated. Current state:
  PLANNED.
- **R054**. Evidence that must exist: Full required coverage and gate reports,
  including real decision/MC/DC instrumentation rather than inferred
  percentages. Current state: UNKNOWN.
- **R055–R057**. Evidence that must exist: Approved standards mapping,
  meaningful mutation/performance evidence and current
  code/performance/security/AI reviews. Current state: PLANNED.
- **R058**. Evidence that must exist: Synthetic fixture provenance, private
  dataset access boundaries, stratification records and a public-output privacy
  scan. Current state: PLANNED.
- **R059–R060**. Evidence that must exist: Frozen fixed/fresh sampling protocol,
  at least 500 suggestions in each, confidence intervals/subgroups, independent
  opportunity coverage, no regression in reconstruction recall or anchor
  agreement, and the post-launch latest-200-decision accept-rate measure under
  its approved decision rule. Current state: PLANNED.
- **R061–R062**. Evidence that must exist: Verified account/terms controls,
  redaction, no-tools/read-only isolation and private structured judge-output
  tests. Current state: UNKNOWN.
- **R063–R065**. Evidence that must exist: Consensus, invalid repair,
  reliability, blind agreement, cache identity, drift and resumable
  candidate-run tests. Current state: PLANNED.
- **R066**. Evidence that must exist: Non-judge nightly schedule, budgeted
  optional warnings and uninterrupted 72-hour supervised soak evidence. Current
  state: PLANNED.
- **R067**. Evidence that must exist: Source provenance and rights admission for
  every contribution/dependency/model/dataset; unresolved items blocked. Current
  state: UNKNOWN.
- **R068–R071**. Evidence that must exist: Live GitHub settings plus attempted
  forbidden-action tests, trusted identity evidence, stale-check refusal and
  qualified release publication. Current state: PLANNED.
- **R072–R074**. Evidence that must exist: Host execution boundaries, verified
  authorised storage, offline pause/resume and private corpus reconstruction
  evidence. Current state: PLANNED.
- **R075**. Evidence that must exist: Review and scans of current files, Git
  history, artifacts, previews and logs for identifying or member data. Current
  state: PLANNED.
- **R076–R077**. Evidence that must exist: Qualified pinned tools; actual just
  commands, import/schema checks and required gates exercised on known failures.
  Current state: PLANNED.
- **R078–R080**. Evidence that must exist: Built/deployed manual, generated
  references, full version retention, preview lifecycle, fresh synthetic
  screenshots, proved build-input isolation from private data/raw lab results,
  and literal clean-install trial. Current state: PLANNED.
- **R081**. Evidence that must exist: A reviewed docs-only change classification
  and concrete docs/secrets/rights/consistency results, with truthful code-only
  N/A decisions. Current state: PLANNED.
- **R082**. Evidence that must exist: Catalogue-first milestone evidence,
  without claiming its research entries are already implemented or
  release-qualified. Current state: PLANNED.

## Source inventory treatment

The source-item CSV currently marks every indexed answer line
REVIEW_WITH_OVERRIDES and its implementation evidence UNKNOWN. This prevents an
old title-changing instruction, author-editor flow or obsolete privacy rule from
being treated as an active task merely because it appears in a quotation.

For each feature, record ACTIVE, SUPERSEDED, CONTEXT or CANDIDATE for its source
items, link the applicable current requirement and give an evidence contract.
Never delete source items to improve apparent completion. Package proposals and
research candidates require separate admission decisions; they do not become
active dependencies automatically.

## Named deliverables and commands

These explicitly requested names remain acceptance obligations. A name in this
table is not a claim that the file, command, branch or service currently works.
If a later approved design changes a name, record that decision rather than
silently abandoning the requested interface.

- **AGENTS.md and CONTRIBUTING.md**. Acceptance evidence needed: Current
  reviewed rules, plain-English communication, contributor workflow and
  enforcement cross-references.
- **justfile; just check, test, lint, typecheck, eval, run and db**.
  Acceptance evidence needed: Actual commands with the applicable
  full/diagnostic semantics, failure-path tests and qualified host
  execution.
- **.devcontainer**. Acceptance evidence needed: A pinned CI-matching
  environment that cannot become an editing-computer execution loophole.
- **contracts/ and generated Pydantic models**. Acceptance evidence needed:
  Admitted schemas, generation checks, malformed-input tests and matching
  runtime consumers.
- **docs/operations.md and docs/dev/repository-setup.md**.
  Acceptance evidence needed: Accurate operating/setup instructions, actual
  settings evidence and exact owner-only setup steps where needed.
- **docs/decisions.md**. Acceptance evidence needed: A discoverable
  pointer to the current decisions, explicitly covering the redacted AI-pack
  exception and account-control qualification.
- **docs/edge-cases.md**. Acceptance evidence needed: A discoverable
  current-case index linking the full Q148 request, its overrides and
  feature-specific acceptance cases.
- **lab/eval/pack.py and lab/eval/score.py**. Acceptance evidence needed:
  Approved evaluation specification, bounded private packs, strict schema,
  consensus/invalid-output handling and correct scoring evidence.
- **lab/results/verdicts/<judge>/<sha>.jsonl**. Acceptance evidence needed:
  Private validated verdict artifacts with complete identities; never raw
  prompts or public member data.
- **Live Marimo method notebooks**. Acceptance evidence needed:
  Production-function imports, explanation, mathematical review,
  benchmarks/regression cases and headless verification.
- **Private SQLite run ledger**. Acceptance evidence needed: Durable
  ownership/evidence/handover records, access isolation, stale-writer
  refusal and recovery tests.
- **manual/ and the public manual site**. Acceptance evidence needed:
  Complete current-state user pages, all requested navigation/modules,
  accessibility and actual deployment evidence.
- **manual/stylesheets/tokens.css and manual/stylesheets/manual.css**.
  Acceptance evidence needed: Sourced palette tokens, permitted styling,
  contrast checks and rendered light/dark evidence.
- **docs/dev/manual-design.md**. Acceptance evidence needed: Design
  rationale and palette-source records without copied protected assets or
  unsupported qualification claims.
- **docs/dev/manual-gardener.md**. Acceptance evidence needed: A usable
  weekly agent task, reviewed changes, accurate per-page review metadata and
  the expected PR workflow.
- **docs/dev/releasing.md and CHANGELOG.md**. Acceptance evidence needed:
  Accurate candidate-to-final process, qualified release notes and
  artifact/manual version mapping.
- **gh-pages; dev; preview/<branch-name>/; full release versions; latest**.
  Acceptance evidence needed: Isolated paths, safe publication identity,
  preview creation/removal, frozen history and correct post-gate latest
  promotion.
- **sutra doctor and sutra doctor --bundle**. Acceptance evidence needed:
  Plain-English diagnosis, exhaustive user-facing error catalogue links and
  scrubbed preview-before-export behaviour.
- **SUT-NNN user-facing error catalogue**. Acceptance evidence needed: Every
  raised user-facing error has a catalogue entry, explanation, remedy and
  stable manual URL.
- **Settings, CLI, errors and release-note generated references**.
  Acceptance evidence needed: Consistency with the admitted source
  definitions and no unsafe imports or private-data inputs during generation.
- **XenForo agent guidance in the repository**. Acceptance evidence needed:
  Original, source-linked guidance from verified official material; no
  invented claim of an official packaged skill.
- **Feature-request records and milestone tracking**.
  Acceptance evidence needed: Every new request has origin, overlap check,
  decision, implementation status and acceptance evidence without duplicate
  inflation.

## Release audit

Before marking a release complete, inspect every active requirement and every
applicable named source item, module, command, gate, invariant and deliverable.
Classify its evidence as proven, contradicted, incomplete, indirect or missing.
Only proven items count as achieved. An unapproved specification, unavailable
account control, unqualified library or absent dataset cannot be hidden by a
green subset of tests. A release remains incomplete until every required row and
its detailed feature cases have appropriate current evidence.

