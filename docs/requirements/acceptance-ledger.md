# Acceptance ledger

Status: PLANNED. No row below is a product-completion claim.

This ledger identifies the evidence needed for the full objective. The current
requirements are R001–R082 in [current decisions](current-decisions.md); exact
source details remain in the [interview](sources/README.md) and
[source-item inventory](source-items.csv). Feature specifications must decompose
compound source lines into testable cases before implementation.

## Evidence contracts

Every acceptance packet identifies the exact source tree, dependency/model/data
versions, configuration, environment, commands, results, review authorship and
limitations. It records failed and skipped checks as well as passes. Public
packets contain only privacy-reviewed summaries. Sensitive evidence stays on the
authorised validation host.

An evidence link must prove the stated scope. A small example does not prove a
million-post workload; synthetic tests do not prove real-data quality; source
review does not prove installation; a legal source citation does not prove rights
clearance; an existing configuration does not prove runtime enforcement.

## Required evidence by requirement

| Requirement | Evidence that must exist | Current state |
| --- | --- | --- |
| R001 | Qualified PHP 8.5/XenForo 2.3 installation and supported-boundary integration tests on Linux. | UNKNOWN |
| R002 | Qualified large-forum workloads and a beginner completing guided setup without command-line work. | PLANNED |
| R003 | Relevance assessments and separately reported usefulness/search outcomes; playbook comparisons. | PLANNED |
| R004–R006 | At least 6,000 distinct qualified method packets, complete scope matrix and more than 10,000 meaningful passing playbook cases. | PLANNED |
| R007 | Owner approval of each concrete feature specification; behaviour examples, test-first record and linked docs. | PLANNED |
| R008 | Initial backlog progress, checkpoints and new/edit event tests, including interruption and resume. | PLANNED |
| R009 | Permission and publication tests proving no v1 author-editor/automatic insertion path. | PLANNED |
| R010–R014 | Property and integration tests for exact-text preservation, caps, duplicate targets, BBCode, guest visibility and optional destinations. | PLANNED |
| R015 | Qualified 30-second first-result measurements and proof that approval is blocked until required applicable work completes. | PLANNED |
| R016–R018 | Stale-content, expiry, atomic insertion, independent-edit-preserving undo, removal suppression, rejection and feedback tests. | PLANNED |
| R019 | Scheduler traces showing every applicable method; explicit missing-input/fault states and blocked dependent outputs. | PLANNED |
| R020 | Power/process interruption tests with durable completed work and bounded repeated unfinished work. | PLANNED |
| R021–R023 | Combined process/native/cache resource accounting, PHP differential peaks, refusal/degradation and hard-limit crash recovery. | UNKNOWN |
| R024–R025 | Controlled tuning experiments, bounded steps, positive weights, atomic settings versions and targeted rollback. | PLANNED |
| R026 | Notebook-to-production import checks, headless notebook results and a ban on production importing lab code. | PLANNED |
| R027–R028 | Independently reviewed package/model admission records, lazy-loading measurements and rejection of executable model formats. | UNKNOWN |
| R029 | Current source-backed Adobe capability matrix with implementation and acceptance proof for every parity claim. | PLANNED |
| R030–R032 | Consent/withdrawal, deduplication, retention, verified upload, outage queue and storage migration tests. | PLANNED |
| R033–R034 | Connector provenance/coverage tests, quota scheduling, crawler limits and proof that prohibited SEO writes cannot occur. | PLANNED |
| R035 | Rights/source review and knowledge/tag ambiguity, freshness, length, originality and batch-approval tests. | UNKNOWN |
| R036–R038 | Ranking fixtures and perturbation tests, public karma recalculation, guest/reaction policy and moderator-only abuse actions. | PLANNED |
| R039–R040 | Hub/summary/draft review flows, safe authorship, publication permission, stale-summary and novelty evidence. | PLANNED |
| R041–R044 | Glossary, core/module boundary and action-interface parity tests; schema validation and permission-denial evidence. | PLANNED |
| R045–R046 | Qualified agent adapters, simultaneous-writer refusal, stale-lease fencing, preserved handover and private SQLite recovery. | UNKNOWN |
| R047–R048 | Rendered screenshots, interaction/browser tests, contrast and accessibility evidence for all owned screens. | PLANNED |
| R049–R050 | Clean installation/upgrade/rollback/uninstall using the manual, including optional services and explicit backup override limits. | PLANNED |
| R051–R052 | Fault-code catalogue coverage, scrubbed support exports, alert grouping and privacy deletion through every retained data path. | PLANNED |
| R053 | A mapped case for every applicable Q148 edge case, with starred-case release blocking demonstrated. | PLANNED |
| R054 | Full required coverage and gate reports, including real decision/MC/DC instrumentation rather than inferred percentages. | UNKNOWN |
| R055–R057 | Approved standards mapping, meaningful mutation/performance evidence and current code/performance/security/AI reviews. | PLANNED |
| R058 | Synthetic fixture provenance, private dataset access boundaries, stratification records and a public-output privacy scan. | PLANNED |
| R059–R060 | Frozen fixed/fresh sampling protocol, at least 500 suggestions in each, confidence intervals/subgroups, independent opportunity coverage, no regression in reconstruction recall or anchor agreement, and the post-launch latest-200-decision accept-rate measure under its approved decision rule. | PLANNED |
| R061–R062 | Verified account/terms controls, redaction, no-tools/read-only isolation and private structured judge-output tests. | UNKNOWN |
| R063–R065 | Consensus, invalid repair, reliability, blind agreement, cache identity, drift and resumable candidate-run tests. | PLANNED |
| R066 | Non-judge nightly schedule, budgeted optional warnings and uninterrupted 72-hour supervised soak evidence. | PLANNED |
| R067 | Source provenance and rights admission for every contribution/dependency/model/dataset; unresolved items blocked. | UNKNOWN |
| R068–R071 | Live GitHub settings plus attempted forbidden-action tests, trusted identity evidence, stale-check refusal and qualified release publication. | PLANNED |
| R072–R074 | Host execution boundaries, verified authorised storage, offline pause/resume and private corpus reconstruction evidence. | PLANNED |
| R075 | Review and scans of current files, Git history, artifacts, previews and logs for identifying or member data. | PLANNED |
| R076–R077 | Qualified pinned tools; actual just commands, import/schema checks and required gates exercised on known failures. | PLANNED |
| R078–R080 | Built/deployed manual, generated references, full version retention, preview lifecycle, fresh synthetic screenshots, proved build-input isolation from private data/raw lab results, and literal clean-install trial. | PLANNED |
| R081 | A reviewed docs-only change classification and concrete docs/secrets/rights/consistency results, with truthful code-only N/A decisions. | PLANNED |
| R082 | Catalogue-first milestone evidence, without claiming its research entries are already implemented or release-qualified. | PLANNED |

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

| Requested artifact or interface | Acceptance evidence needed |
| --- | --- |
| AGENTS.md and CONTRIBUTING.md | Current reviewed rules, plain-English communication, contributor workflow and enforcement cross-references. |
| justfile; just check, test, lint, typecheck, eval, run and db | Actual commands with the applicable full/diagnostic semantics, failure-path tests and qualified host execution. |
| .devcontainer | A pinned CI-matching environment that cannot become an editing-computer execution loophole. |
| contracts/ and generated Pydantic models | Admitted schemas, generation checks, malformed-input tests and matching runtime consumers. |
| docs/operations.md and docs/dev/repository-setup.md | Accurate operating/setup instructions, actual settings evidence and exact owner-only setup steps where needed. |
| docs/decisions.md | A discoverable pointer to the current decisions, explicitly covering the redacted AI-pack exception and account-control qualification. |
| docs/edge-cases.md | A discoverable current-case index linking the full Q148 request, its overrides and feature-specific acceptance cases. |
| lab/eval/pack.py and lab/eval/score.py | Approved evaluation specification, bounded private packs, strict schema, consensus/invalid-output handling and correct scoring evidence. |
| lab/results/verdicts/<judge>/<sha>.jsonl | Private validated verdict artifacts with complete identities; never raw prompts or public member data. |
| Live Marimo method notebooks | Production-function imports, explanation, mathematical review, benchmarks/regression cases and headless verification. |
| Private SQLite run ledger | Durable ownership/evidence/handover records, access isolation, stale-writer refusal and recovery tests. |
| manual/ and the public manual site | Complete current-state user pages, all requested navigation/modules, accessibility and actual deployment evidence. |
| manual/stylesheets/tokens.css and manual/stylesheets/manual.css | Sourced palette tokens, permitted styling, contrast checks and rendered light/dark evidence. |
| docs/dev/manual-design.md | Design rationale and palette-source records without copied protected assets or unsupported qualification claims. |
| docs/dev/manual-gardener.md | A usable weekly agent task, reviewed changes, accurate per-page review metadata and the expected PR workflow. |
| docs/dev/releasing.md and CHANGELOG.md | Accurate candidate-to-final process, qualified release notes and artifact/manual version mapping. |
| gh-pages; dev; preview/<branch-name>/; full release versions; latest | Isolated paths, safe publication identity, preview creation/removal, frozen history and correct post-gate latest promotion. |
| sutra doctor and sutra doctor --bundle | Plain-English diagnosis, exhaustive user-facing error catalogue links and scrubbed preview-before-export behaviour. |
| SUT-NNN user-facing error catalogue | Every raised user-facing error has a catalogue entry, explanation, remedy and stable manual URL. |
| Settings, CLI, errors and release-note generated references | Consistency with the admitted source definitions and no unsafe imports or private-data inputs during generation. |
| XenForo agent guidance in the repository | Original, source-linked guidance from verified official material; no invented claim of an official packaged skill. |
| Feature-request records and milestone tracking | Every new request has origin, overlap check, decision, implementation status and acceptance evidence without duplicate inflation. |

## Release audit

Before marking a release complete, inspect every active requirement and every
applicable named source item, module, command, gate, invariant and deliverable.
Classify its evidence as proven, contradicted, incomplete, indirect or missing.
Only proven items count as achieved. An unapproved specification, unavailable
account control, unqualified library or absent dataset cannot be hidden by a
green subset of tests. A release remains incomplete until every required row and
its detailed feature cases have appropriate current evidence.
