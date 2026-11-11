# Specifications

Each specification describes one part of Sutradhar: what it does, its
limits, its footprint budget where it touches public pages, and how it
is tested. How specifications are named and approved is in
[AGENTS.md section 12](../../AGENTS.md#12-docs).

The feature list, features 1 to 1357, is in [features.md](features.md).
Its section "Which spec designs each feature" names the one home of
each feature.
Specifications 0003 to 0057, 0061 and 0063 are rewritten from the
private archive `archive-2026-11-08-old-repo` on Dell; the table below
names each one's source files. Specifications 0001, 0002, 0058 to
0060, 0062, 0064, 0065, 0067, 0068, 0069, 0070 and 0071 were written
fresh.

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
| [0012](0012-ganit-evolve.md) | GanitEvolve | Phase 2 item 6 | `ganit-evolve/README.md ganit-evolve/ganit-evolve-2.md` to `-5.md` |
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
| [0030](0030-install-upgrade-removal.md) | Point-and-click install, upgrade and removal | Phase 5 item 1 | `operations/install*.md` |
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
| [0053](0053-feedback-learning.md) | Feedback learning and bounded autotuning | Phase 6 | `linking/feedback.md linking/feedback-2.md linking/feedback-3.md` |
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
| [0067](0067-method-chains.md) | Method chains | Phase 7 item 1 | written fresh |
| [0068](0068-chain-shapes-ranking-graph.md) | Chain shapes for link ranking and graph relevance | Phase 7 item 1 | written fresh |
| [0069](0069-wide-link-ranking.md) | Wide link ranking shape | Phase 7 item 1 | written fresh |
| [0070](0070-capability-workflows.md) | Capability workflows | Phase 7 item 1 | written fresh |
| [0071](0071-checked-variants.md) | Checked variants | Phase 7 item 1 | written fresh |
