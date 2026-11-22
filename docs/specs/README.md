# Specifications

Each specification describes one part of Sutradhar: what it does, its
limits, its footprint budget where it touches public pages, and how it
is tested. How specifications are named and approved is in
[AGENTS.md section 12](../../AGENTS.md#12-docs).

Use [features.md](features.md) for the original feature titles.
Resolve the owning specification through the feature register's
[original feature rows][original-features] or its
[pasted feature rows][pasted-features]. The
[canonical bindings][feature-bindings] connect each pasted source
key with its spec 0076 feature ID. Follow the Owner column for
the contract and use any Shares reference for its supporting part.

[original-features]: feature-register.md#old-features-1-to-1357
[pasted-features]: feature-register.md#pasted-features-new-ids
[feature-bindings]:
  feature-register.md#canonical-bindings-for-pasted-features
Specifications 0003 to 0057, 0061, 0063 and 0066 are rewritten from the
private archive `archive-2026-11-08-old-repo` on Dell; the table below
names each one's source files. Specifications 0001, 0002, 0058 to
0060, 0062, 0064, 0065, 0067, 0068, 0069, 0070, 0071 and 0072
were written fresh. Specifications 0078 to 0083 were written fresh from
ADR-0021, ADR-0022 and the owner's answers of 19 and 20 November 2026.
Specifications 0076 and 0077 were written fresh from ADR-0020 and the
owner's documentation, architecture and ranking protocol. Where an archived
source conflicts with ADR-0020, the revised
specification and its explicit supersession record govern; the archive is
provenance, not current authority.

| No. | Title | Roadmap item | Archive source (under `docs/specifications/`) |
|---|---|---|---|
| [0001](0001-door.md) | The door | Phase 1 items 3 and 4 | written fresh |
| [0002](0002-forge-sync.md) | forge-sync | Phase 1 item 9 | written fresh |
| [0003](0003-toolchain-and-check-runner.md) | Toolchain and check runner | Phase 2 items 1 and 3 | `bootstrap-toolchain.md bootstrap-toolchain-rust.md` |
| [0004](0004-manual-publishing.md) | Manual publishing | Phase 5 item 7; Phase 7 item 2 | `manual-publishing.md manual-publishing-main-only.md` |
| [0005](0005-release-coordination.md) | Agent and release coordination | Phase 7 item 3 | `agent-release-coordination.md release-main-only.md` |
| [0006](0006-main-only-publication.md) | Main-only publication and GitHub settings | Phase 1 items 3, 4 and 9 | `main-only-publication.md main-only-github-settings.md` |
| [0007](0007-performance-and-engineering-gates.md) | Performance gate and engineering gates | Phase 2 items 7 and 9 | `performance-gate.md performance-gate-inputs.md performance-gate-rust.md engineering-gates.md` |
| [0008](0008-trust-boundaries.md) | Trust boundaries | Phase 1 item 9; Phase 2 item 1 | `trust-boundaries.md` |
| [0009](0009-source-class-inputs.md) | Source class inputs | Phase 2 item 1 | `source-class-inputs.md` |
| [0010](0010-coding-harness.md) | Coding harness and checker | Phase 2 items 1 and 2 | `coding-harness/README.md` and the other `coding-harness/*.md` files except `google-grade*.md` |
| [0011](0011-engineering-standard.md) | Engineering standard for the gate | Phase 2 items 2 and 4 | `coding-harness/google-grade.md coding-harness/google-grade-2.md` |
| [0012](0012-ganit-evolve.md) | Ganit Vikas | Phase 2 item 6 | `ganit-evolve/README.md ganit-evolve/ganit-evolve-2.md` to `-5.md` |
| [0013](0013-research-rules.md) | Research rules | Phase 2 item 6 | `ganit-evolve/research-rules*.md` |
| [0014](0014-settings-profiles-history.md) | Settings, profiles and history | Phase 3 item 1 | `platform/settings*.md` |
| [0015](0015-module-registry.md) | Module registry and lifecycle | Phase 3 item 1 | `platform/modules*.md` |
| [0016](0016-progress-system.md) | One progress system | Phase 3 item 1 | `platform/progress*.md` |
| [0017](0017-visual-layer.md) | Visual layer on real data | Phase 3 items 1 and 3 | `platform/visuals*.md` |
| [0018](0018-platform-additions.md) | Platform additions | Phase 3 item 1; Phase 5 item 2 | `platform/platform-additions*.md` |
| [0019](0019-events-jobs-recovery.md) | Events, jobs and durable recovery | Phase 3 item 6 | `platform/jobs*.md` |
| [0020](0020-screens-actions-sutra-mcp.md) | Screens, actions, sutra and MCP | Phase 5 item 5; Phase 3 item 5; Phase 6 item 3 | `operations/actions*.md` |
| [0021](0021-forum-contracts-ingestion.md) | Signed contracts and guest-visible ingestion | Phase 4 item 1 | `platform/forum*.md` |
| [0022](0022-reading-indexing.md) | Reading and indexing | Phase 4 item 3 | `linking/reading*.md` |
| [0023](0023-matching-anchors.md) | Matching and anchors | Phase 4 item 3; Phase 6 item 1 | `linking/matching*.md` |
| [0024](0024-models.md) | Models | Phase 4 item 3 | `linking/models*.md` |
| [0025](0025-paid-embeddings.md) | Paid embeddings | Phase 4 item 3 | `linking/paid-embeddings*.md` |
| [0026](0026-text-engine.md) | shodhan, the text engine | Phase 4 item 3 | `text/*.md` |
| [0027](0027-candidates-ranking.md) | Candidates, relevance and ranking | Phase 4 item 3; Phase 6 item 1 | `linking/ranking*.md` |
| [0028](0028-staff-suggestions.md) | Staff suggestions, insertion and undo | Phase 4 item 3 | `linking/suggestions*.md` |
| [0029](0029-link-additions.md) | Link additions | Phase 4 item 3; Phase 6 items 1 and 2 | `linking/link-additions*.md` |
| [0030](0030-install-upgrade-removal.md) | Point-and-click install, update, repair, backup and removal | Phase 5 item 1 | `operations/install*.md` |
| [0031](0031-shared-platform-installation.md) | Shared-platform installation | Phase 5 item 1; Phase 4 items 3 and 4 | `installation-platforms/*.md` |
| [0032](0032-audit-erasure-diagnostics.md) | Audit, erasure, diagnostics and safe exports | Phase 5 items 1 and 3 | `operations/audit.md operations/audit-2.md` |
| [0033](0033-safe-running.md) | Safe running | Phase 5 item 2 | `platform/safe-running*.md` |
| [0034](0034-analytics.md) | darshak, first-party analytics | Phase 5 item 4; Phase 6 items 1 and 2 | `signals/analytics*.md` |
| [0035](0035-hisaab.md) | hisaab, books and reports | Phase 5 item 4 | `hisaab/*.md` |
| [0036](0036-audit-proof.md) | Audit proof | Phase 5 item 4 | `operations/audit-proof.md` |
| [0037](0037-extension-isolation.md) | Third-party extension isolation | Phase 5 item 5 | `operations/extensions*.md` |
| [0038](0038-search-services-cloud-storage.md) | Search services, cloud storage and provisioning | Phase 5 item 6 | `operations/search-storage*.md` |
| [0039](0039-test-room.md) | Test room | Phase 6 | `signals/machinemirror*.md` |
| [0040](0040-search-connectors.md) | Search Console, Bing and GA4 connectors | Phase 6 items 1 and 2 | `signals/connectors*.md` |
| [0041](0041-editorial-coach.md) | Editorial Coach | Phase 6 items 1 and 2 | `signals/editorial-coach.md` |
| [0042](0042-coach-additions.md) | Coach additions | Phase 6 items 2 and 3 | `signals/coach-additions*.md` |
| [0043](0043-search-charts.md) | Search charts | Phase 6 items 1 and 2 | `signals/quarryview*.md` |
| [0044](0044-seo-signals-crawler.md) | SEO signals, crawler and health reports | Phase 6 items 1, 2 and 4 | `signals/seo.md signals/seo-2.md signals/seo-crawler-amendment.md` |
| [0045](0045-backlink-crawl.md) | Backlink crawl | Phase 6 item 2 | `signals/backlink-crawl.md` |
| [0046](0046-web-link-study.md) | Web link study | Phase 6 items 1 and 2 | `signals/link-study.md` |
| [0047](0047-site-structure-checks.md) | Site structure checks | Phase 6 items 1 and 2 | `signals/site-structure-checks*.md` |
| [0048](0048-fix-list.md) | Fix list | Phase 6 items 1 and 2 | `signals/work-graph*.md` |
| [0049](0049-knowledge-graph.md) | Knowledge graph | Phase 6 items 1 and 3 | `signals/graph.md` |
| [0050](0050-sorting-usefulness-karma.md) | Sorting, usefulness and karma | Phase 6 | `signals/ranking*.md` |
| [0051](0051-moderation-hints.md) | Moderation hints | Phase 6 item 1 | `signals/moderation.md` |
| [0052](0052-hubs-summaries-drafts.md) | Hubs, summaries and content drafts | Phase 6 items 1 and 3 | `signals/content*.md` |
| [0053](0053-feedback-learning.md) | Feedback learning and label governance | Phase 6 | `linking/feedback.md linking/feedback-2.md linking/feedback-3.md` |
| [0054](0054-feedback-harvest.md) | Feedback harvest | Phase 6 | `linking/feedback-harvest*.md` |
| [0055](0055-method-research-catalogue.md) | Method research and admission catalogue | Phase 7 item 1 | `linking/methods*.md` |
| [0056](0056-method-lab.md) | Method lab | Phase 7 item 1 | `linking/lab*.md` |
| [0057](0057-quality-judging.md) | Quality judging and release assessment | Phase 7 item 1 | `operations/judging*.md` |
| [0058](0058-chaukidar.md) | chaukidar, the watchman | Phase 1 item 5 | written fresh |
| [0059](0059-pankti.md) | pankti, the heavy-job queue | Phase 1 item 7 | written fresh |
| [0060](0060-runner.md) | Forgejo's runner on Dell | Phase 1 item 2 | written fresh |
| [0061](0061-nirikshak.md) | nirikshak, the code analyser | Phase 2 item 2 | `findings/nirikshak-2026-11-07.md`, rewritten for Rust and Rego |
| [0062](0062-forgejo-server.md) | Forgejo on Dell and its settings | Phase 1 item 1 | written fresh |
| [0063](0063-autoissues.md) | autoissues, the fault tracker | Later (task #23) | `autoissues/*.md` |
| [0064](0064-v1-qualification.md) | Version 1 scope and qualification | Phase 6; Phase 7 item 1 | written fresh |
| [0065](0065-tijori.md) | tijori's signing core | Phase 2 item 8; Phase 3 item 8; Phase 5 items 1 and 5; Phase 7 items 1 and 3 | written fresh, using the archive's ADR-0050 |
| [0066](0066-ganit.md) | ganit, the shared maths engine | Phase 5 item 4 | the archive's `docs/dev/ganit.md` to `ganit-5.md` |
| [0067](0067-method-chains.md) | Method chains (superseded by ADR-0021) | Phase 7 item 1 | written fresh |
| [0068](0068-chain-shapes-ranking-graph.md) | Chain shapes for link ranking and graph relevance (superseded by ADR-0021) | Phase 7 item 1 | written fresh |
| [0069](0069-wide-link-ranking.md) | Wide link ranking shape (superseded by ADR-0021) | Phase 7 item 1 | written fresh |
| [0070](0070-capability-workflows.md) | Capability workflows (superseded by ADR-0021) | Phase 7 item 1 | written fresh |
| [0071](0071-checked-variants.md) | Checked variants (superseded by ADR-0021) | Phase 7 item 1 | written fresh |
| [0072](0072-ranking-and-spam.md) | Buddhi and Rakshak | Phase 6 | written fresh, using the archive's ADR-0063 item 3 and interview Q049 |
| [0076](0076-specification-completeness.md) | Specification completeness and implementation readiness | Every phase before implementation | written fresh, from ADR-0020 |
| [0077](0077-autonomous-ranking-optimisation.md) | Autonomous ranking optimisation and champion control | Phase 7 item 1 | written fresh, from ADR-0020 and the owner's ranking protocol |
| [0078](0078-memory-budget-and-turn-taking.md) | Memory budget and turn-taking | Phase 3 and Phase 5 item 2 | written fresh |
| [0079](0079-ranking-method-catalogue.md) | Ranking method catalogue | Phase 6 and Phase 7 item 1 | written fresh |
| [0080](0080-model-hub.md) | Model Hub | Phase 5 | written fresh |
| [0081](0081-behemoth-boot-parity.md) | Behemoth Boot parity and lockstep | Phase 3B | written fresh |
| [0082](0082-leviathan-crawler.md) | Leviathan, the crawler | Phase 6 items 1 and 2 | written fresh |
| [0083](0083-meaningful-lines-and-anti-padding.md) | Meaningful lines and anti-padding | Phase 2 and Phase 7 item 1 | written fresh |
| [0084](0084-research-sources-planning-and-outlining.md) | Research, sources, planning and outlining | Phase 6 | written fresh |
| [0085](0085-article-types-and-forum-native-content.md) | Article types and forum-native content | Phase 6 | written fresh |
| [0086](0086-learning-reference-and-community-pages.md) | Learning, reference and community pages | Phase 6 | written fresh |
| [0087](0087-visual-audio-video-and-podcast-content.md) | Visual, audio, video and podcast content | Phase 6 | written fresh |
| [0088](0088-email-commerce-sponsor-and-site-copy.md) | Email, commerce, sponsor and site copy | Phase 6 | written fresh |
| [0089](0089-editing-operations-and-quality-gates.md) | Editing operations and quality gates | Phase 6 | written fresh |
| [0090](0090-variants-collaboration-and-publishing.md) | Variants, collaboration and publishing | Phase 6 | written fresh |
| [0091](0091-measurement-safety-and-policy-guardrails.md) | Measurement, safety and policy guardrails | Phase 6 | written fresh |
| [0092](0092-keyword-topic-and-competitive-research.md) | Keyword, topic and competitive research | Phase 6 | written fresh |
| [0093](0093-ai-search-visibility.md) | AI search visibility | Phase 6 | written fresh |
| [0094](0094-reporting-forecasts-proof-and-experiments.md) | Reporting, forecasts, proof and experiments | Phase 6 | written fresh |
| [0095](0095-local-presence-listings-and-reviews.md) | Local presence, listings and reviews | Phase 6 | written fresh |
| [0096](0096-social-publishing-and-distribution.md) | Social publishing and distribution | Phase 6 | written fresh |
| [0097](0097-brand-monitoring-assets-and-surveys.md) | Brand monitoring, assets and surveys | Phase 6 | written fresh |
| [0098](0098-sponsors-crm-and-community-growth.md) | Sponsors, CRM and community growth | Phase 6 | written fresh |
| [0099](0099-jobs-and-workflow-screen.md) | Jobs and workflow screen | Phase 3 | written fresh |
| [0100](0100-outside-services-and-data-platform.md) | Outside services and the data platform | Phase 6 | written fresh |
| [0101](0101-site-accounts-writers-and-governance.md) | Site accounts, writers, governance and compliance helpers | Phase 5 | written fresh |
| [0102](0102-several-sites-and-portfolio.md) | Several sites and the portfolio view | Phase 5 | written fresh |
| [0103](0103-shop-seo-for-woocommerce.md) | Shop SEO for WooCommerce | Phase 6 | written fresh |
| [0104](0104-python-scripting-and-sidecar.md) | Python scripting and the sidecar | Phase 5 | written fresh |
| [0105](0105-leviathan-link-evidence-and-clues.md) | Leviathan link evidence and pattern clues | Phase 6 | written fresh |
| [0106](0106-behemoth-core-container-and-config.md) | Behemoth Boot core: container, configuration, lifecycle (PB-B) | Phase 3B | written fresh |
| [0107](0107-behemoth-web-clients-and-serialisation.md) | Behemoth Boot web, clients and serialisation (PB-C) | Phase 3B | written fresh |
| [0108](0108-behemoth-data-access.md) | Behemoth Boot data access (PB-D) | Phase 3B | written fresh |
| [0109](0109-behemoth-security-and-identity.md) | Behemoth Boot security and identity (PB-E) | Phase 3B | written fresh |
| [0110](0110-behemoth-messaging-events-and-resilience.md) | Behemoth Boot messaging, events and resilience (PB-F) | Phase 3B | written fresh |
| [0111](0111-behemoth-batch-cloud-ai-and-mail.md) | Behemoth Boot batch, cloud patterns, AI, MCP and mail (PB-G) | Phase 3B | written fresh |
| [0112](0112-behemoth-actuator-and-observability.md) | Behemoth Boot actuator and observability (PB-H) | Phase 3B | written fresh |
| [0113](0113-behemoth-developer-tools.md) | Behemoth Boot developer tools (PB-I) | Phase 3B | written fresh |
| [0114](0114-behemoth-verification.md) | Behemoth Boot verification (PB-J) | Phase 3B | written fresh |
| [0115](0115-leviathan-rendering-and-content-engine.md) | Leviathan rendering and content engine | Phase 6 | written fresh |
| [0116](0116-leviathan-scale-storage-and-verification.md) | Leviathan scale, storage and verification | Phase 6 | written fresh |
| [0117](0117-editor-coach-query-fan-out.md) | Editorial Coach Query Fan-Out Simulator | Phase 6 | written fresh |
| [0118](0118-end-to-end-encryption-gate.md) | End-to-end encryption gate | Phase 2 | written fresh |
| [0119](0119-cross-site-link-suggestions.md) | Cross-site link suggestions | Phase 6 | written fresh |
| [0120](0120-opt-in-referral-backlink-discovery.md) | Opt-in referral backlink discovery | Phase 6 | written fresh |
| [0121](0121-clean-room-writing-suite.md) | Clean-room writing suite | Phase 6 | written fresh |
| [0122](0122-multilingual-writing-checks.md) | Multilingual writing checks | Phase 6 | written fresh |
| [0123](0123-writing-analysis-reports.md) | Writing analysis reports | Phase 6 | written fresh |
| [0124](0124-writing-revision-and-translation.md) | Writing revision and translation | Phase 6 | written fresh |
| [0125](0125-story-planning-and-critique.md) | Story planning and critique | Phase 6 | written fresh |
| [0126](0126-writing-sessions-and-learning.md) | Writing sessions and learning | Phase 6 | written fresh |
| [0127](0127-writing-sources-and-origin-records.md) | Writing sources and origin records | Phase 6 | written fresh |
| [0128](0128-writing-policies-and-team-controls.md) | Writing policies and team controls | Phase 6 | written fresh |
| [0129](0129-writing-files-and-review-exchange.md) | Writing files and review exchange | Phase 6 | written fresh |
| [0130](0130-writing-surfaces-and-local-assistants.md) | Writing surfaces and local assistants | Phase 6 | written fresh |
| [0131](0131-one-off-rival-page-comparison.md) | One-off rival page comparison | Phase 6 | written fresh |
| [0132](0132-dataforseo-keyword-research.md) | DataForSEO keyword research | Phase 6 | written fresh |
| [0133](0133-dataforseo-live-coach-evidence.md) | Live search evidence in the Editorial Coach | Phase 6 | written fresh |
| [0134](0134-ci-latency-load-gate.md) | CI latency load gate | Phase 2 | written fresh |
| [0135](0135-ai-fetches-referrals-and-ranking.md) | AI fetches, referrals and ranking evidence | Phase 6 | written fresh |
| [0136](0136-matomo-timeline-connector.md) | Read-only Matomo timeline connector | Phase 6 | written fresh |
| [0137](0137-google-search-update-tracker.md) | Google Search Update Tracker and content timeline | Phase 6 | written fresh |
| [0138](0138-empirical-update-lab.md) | Empirical Update Lab and post-update Editorial Coach guidance | Phase 6 | original design |
| [0139](0139-full-foundation-refactor.md) | Foundation refactor and framework qualification handoff | Phase 3A and 3B | original design |
| [0140](0140-unified-site-audit-suite.md) | Unified site audit suite | Phase 6 | original design |
| [0141](0141-audit-crawl-logs-and-technical-toolbox.md) | Crawl, logs and Technical Toolbox | Phase 6 | original design |
| [0142](0142-local-performance-lab.md) | Local Performance Lab | Phase 6 | original design |
| [0143](0143-audit-grid-findings-and-local-preview.md) | Site Health, audit views and local search preview | Phase 6 | original design |
| [0144](0144-site-corpus-and-text-mining.md) | Site corpus and text mining | Phase 6 | original design |
| [0145](0145-audit-rule-and-field-catalogue.md) | Audit check and field catalogue | Phase 6 | original design |
| [0146](0146-analytics-forecasting-and-text-behaviour-engine.md) | Analytics forecasting and text-behaviour engine | Phase 6 | original design |
| [0147](0147-action-compiler-and-evidence-backed-tasks.md) | Action Compiler and evidence-backed tasks | Phase 6 | original design, extends 0048 |
| [0148](0148-scientific-experimentation-engine.md) | Scientific Experimentation Engine | Phase 6 | original design |
| [0149](0149-causal-estimators-and-placebos.md) | Causal estimators and placebos | Phase 6 | original design |
| [0150](0150-experiment-source-and-sensor-gates.md) | Experiment source and sensor gates | Phase 6 | original design |
| [0151](0151-site-lawbook-calibration-and-replay.md) | Site Lawbook, calibration and replay | Phase 6 | original design |
| [0152](0152-live-backlink-crawler-profile.md) | Live backlink crawl profiles and crawler names | Phase 6 | 0045 and 0082 profile |
| [0153](0153-public-web-backlink-index.md) | Full public-web backlink index | Separate scope decision | DR-154, QR-238 |
| [0154](0154-backlink-index-site-slices.md) | Backlink index site slices and local lookup | Separate scope decision | 0153 contract, QR-239 |
| [0155](0155-agent-harness.md) | The agent harness | Phase 2, task #166 | DR-161, QR-255 to QR-257 |
