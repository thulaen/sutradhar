# Risk register

Each row names what could go wrong, why we think so, its effect and the
control. Ratings are the register keeper's judgement, not measurements.
Evidence uses source IDs from [sources.md](sources.md).

| ID | Risk | Evidence | Effect | Control | Role |
|---|---|---|---|---|---|
| RK-001 | One-click install of any PyPI package brings unvetted code and licences into a user's scripts. | Owner decision DR-019 | Malicious or copyleft code runs in the sidecar | Fences on folders, network and memory; licence and hash shown; Advanced unlock; never in the app SBOM | Specification owner |
| RK-002 | The unsigned Windows installer triggers SmartScreen warnings. | Owner decision DR-005 | Users abandon install | Plain-English warning page; revisit signing | Owner |
| RK-003 | Three-maker judging at every push uses a large share of the owner's plan. | DR-024 | Work stalls when limits hit | Hand-off prompt; slices sized for one session | Owner |
| RK-004 | No official prebuilt exists for pgvector on Windows, and DuckDB's Rust crate builds C++ when bundled. | CR-008; SR-016, SR-018 | A component waits or is replaced | Research dossier checks each target before specs depend on it | Research agent |
| RK-005 | Quint needs Node.js and Java; Windows support and the quint-connect crate are unverified. | ADR-0021 | Gate cannot run on Windows | Verify in dossier; sealed pinned toolchain | Research agent |
| RK-006 | Spring Boot versions and features in the owner's lists may be wrong or out of date. | ADR-0022 | Parity ledger tracks features that do not exist | Verify each against official release notes before a ledger row | Scout |
| RK-007 | Line-count targets invite padding. | ADR-0013; DR-042 | Bloated, duplicated code | Meaningful-line gate; copy, dead-code, coverage and mutation checks | Reviewer |
| RK-008 | The 6 GB default may not fit PostgreSQL, Tantivy, DuckDB, models and the Python sidecar together. | DR-021 | Stalls or crashes on small PCs | Memory budget spec; measured admission; turn-taking | Specification owner |
| RK-009 | Windows has no Unix sockets for the owner-only link. | DR-007 | Weaker local security | Named pipe locked to the user; tested | Specification owner |
| RK-010 | Updates depend on GitHub availability and terms. | DR-009 | Users cannot update | Manual download path; signed files | Release approver |
| RK-011 | Tauri 2 and Angular 22 version and Windows support are verified only from search results and one official page. | SR-001, SR-008 | Wrong stack assumptions | Dossier verifies from official sources | Research agent |
| RK-012 | Pattern Clues names third-party sites. | DR-041 | Defamation or trust harm if shared | Opt-in, local only, evidence, disclaimer, no named export without confirm; legal review | Owner |
| RK-013 | Crawling any public site raises terms-of-service and legal questions. | DR-041 | Complaints or blocks | Legal wall, honest identity, rate limits; legal review | Owner |
| RK-014 | A local language model plus embeddings and rerankers may exceed memory. | DR-020 | Out-of-memory | One heavy model at a time on tight budgets; fit badges; scheduler | Specification owner |
| RK-015 | No contributor licence agreement means the licence cannot change later without every contributor. | DR-037 | Locked licence | Decide now; record the trade-off | Owner |
| RK-016 | Apache-2.0 lets anyone fork and close the code. | DR-037 | Competing closed copies | Trademark policy; quality edge | Owner |
| RK-017 | XenForo's licence may restrict six test installs or add-on licences. | SR-006 | Test fleet or add-on distribution blocked | Confirm against the live agreement | Owner |
| RK-018 | The scale of two crown jewels is far beyond one plan. | DR-043 | 1.0.0 never arrives | Slices, ledgers and the hand-off prompt; planning figure of about 8,000 slices (estimate) | Owner |
| RK-019 | Own-logic layers may drift from Spring or from crawler standards. | DR-040 | Parity gap | Monthly scout, parity ledger and release block | Scout |
| RK-020 | Three divergent copies of AGENTS.md exist (main, clone, in-flight edits). | Register keeper | Conflicting rules | One merge in the #640 slice before any push | Specification owner |
| RK-021 | The JavaScript and rendering engine for Leviathan is not chosen. | ADR-0022 | Delays Leviathan | Build-or-reuse study in the dossier | Research agent |
| RK-022 | Model licences on open Hugging Face search may be non-commercial. | DR-020 | Users break licences | Licence badge and extra confirm | Specification owner |
| RK-023 | rules_rust does not promise Windows support, but Windows is a version 1 requirement and Bazel is the only build system; the owner chose to own the fix (DR-052). | SR-014 | Windows builds fail or need heavy custom work | Experiment EX-03 sizes the work early | Owner |
| RK-024 | pgvector has no Windows prebuilt; building it breaks the no-C++ rule. | SR-016 | Vector search differs by system | Resolved by DR-053: a Rust vector index on both systems; EX-04 picks the crate | Specification owner |
| RK-025 | The `ort` crate is a release candidate and tract's coverage of the BGE graphs is unknown. | SR-019, SR-020 | No working model runtime | Experiment EX-01 | Research agent |
| RK-026 | Windows 10 is out of support. | SR-030 | Users on Windows 10 cannot be supported | Windows 11 only (DR-055) | Owner |
| RK-027 | Quint's documented commands do not include `typecheck` or `test`, and Windows is not mentioned. | SR-024 | The gate design assumes commands that may not exist | Experiment EX-06 | Research agent |
| RK-028 | llama.cpp releases are frequent and marked pre-release. | SR-022 | Breaking changes between builds | Pin an exact build by hash; run as a child process | Release approver |
| RK-029 | The GitHub runner disk is 14 GB. | SR-031 | Builds and tests run out of disk | Remote caches; synthetic data; the repository stays public | Release approver |
| RK-030 | A scrubbed 160-character sentence in a receipt may raise copyright or privacy questions. | Spec 0082; DR-059 | Complaints or legal exposure | Short cap, scrubber, local only, legal review (QR-010) | Owner |
| RK-031 | The meaningful-line target blocks releases below four million lines each; it may push work toward volume. | Spec 0083 | Padding or delay | Anti-padding checks, ledger requirement, owner waiver route | Reviewer |
| RK-032 | The Receipts mode memory fence and filter sizes are the owner's figures, not measurements. | Spec 0082 | Crawler too heavy or too weak | Measurement on 8 GiB and 16 GiB machines | Specification owner |
