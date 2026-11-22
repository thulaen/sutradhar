# Research dossier

This dossier checks the consequential technical choices of ADR-0021 and
ADR-0022 against official sources. It records what was found, what was not
found, and what must still be tested. It does not change a decision: where
a finding conflicts with an owner decision, it raises a question row in
[the question register](../registers/questions.md).

## Method and labels

- Retrieval date: 20 November 2026 for every source below.
- **Verified**: an official page was fetched and the claim appears in it.
  The fetch tool returns a summary of the page, so "verified" means the
  claim was seen in that summary, not read line by line.
- **Search only**: seen only in a search summary; unverified.
- **Unverified**: from the owner's lists or general knowledge; no source.
- A design decision may propose a limit. It is never a measurement.
- Where a decision needs a prototype, the precise experiment is written
  down and the decision stays blocked. No prototype was built.

## Index

| ID | Question | Result |
|---|---|---|
| RQ-001 | Desktop shell: Tauri 2 | Confirmed, with Windows notes |
| RQ-002 | Front end: Angular 22 | Confirmed |
| RQ-003 | Build system: Bazel with rules_rust | Owner chose Bazel on both systems (DR-052); EX-03 sizes the work |
| RQ-004 | PostgreSQL 18 on Windows and Linux | Confirmed |
| RQ-005 | Vector search: pgvector | Owner chose a Rust vector index (DR-053); EX-04 picks the crate |
| RQ-006 | Text search: Tantivy | Confirmed with limits |
| RQ-007 | Analytics: embedded DuckDB | Owner chose prebuilt library or child process (DR-054); EX-05 decides |
| RQ-008 | Model runtime: ort and tract | Experiment needed (EX-01) |
| RQ-009 | Local language models: llama.cpp | Confirmed as prebuilt child process |
| RQ-010 | Formal checks: Quint and Quint Connect | Partly confirmed; gaps listed |
| RQ-011 | Leviathan's rendering and JavaScript engine | Blocked on experiment (EX-02) |
| RQ-012 | Supported operating systems | Owner confirmed (DR-055) |
| RQ-013 | Windows build and test runners | Confirmed |
| RQ-014 | Memory limit enforcement without systemd | Mechanisms confirmed; fallback needed |
| RQ-015 | Spring Boot facts for the parity ledger | Partly confirmed; 4.2.0 is released |
| RQ-016 | XenForo licence for test installs | Blocked: source not reachable |

Not yet researched, each a later slice: the Python runtime and its link to
Rust, spaCy and Optuna versions, Web Bot Auth, Common Crawl link maps, the
WordPress.org plugin licence rule, and the Windows start-up folder rules.

## RQ-001 Desktop shell

- **Decision:** Tauri 2 as the window on Linux and Windows (DR-005).
- **Requirements affected:** CR-031, CR-032; spec 0030.
- **Alternatives:** Dioxus (dropped by the owner), Electron, a native toolkit.
- **Criteria:** Rust core, small memory, Linux and Windows, installers.
- **Findings:** Verified (SR-001, SR-012). Linux needs webkit2gtk 4.1 and a
  list of development packages. Windows uses Microsoft Edge WebView2,
  which the Tauri page says is already installed on Windows 10 (1803 and
  later) and later versions, with an Evergreen Bootstrapper otherwise. A
  Windows installer can be cross-built from Linux with NSIS only as a last
  resort; MSI needs Windows.
- **Risks:** cross-building is not well tested; signing needs a separate
  tool (we ship unsigned).
- **Selected:** Tauri 2. Build Windows installers on GitHub Actions
  Windows runners (RQ-013), not from Linux.
- **Rejected:** Electron (heavier, not Rust); Dioxus (owner decision).
- **Reconsider if:** WebKitGTK 4.1 is missing on a target Linux release.
- **Confidence:** high. The installed WebKitGTK version on Ubuntu 26.04
  and Debian 13 is not checked here.

## RQ-002 Front end

- **Decision:** Angular 22 with Angular Material in strict TypeScript.
- **Findings:** Verified (SR-013). Angular 22 is listed as actively
  supported. It needs Node.js `^22.22.3`, `^24.15.0` or `^26.0.0` and
  TypeScript `>=6.0.0 <6.1.0`. Release and end-of-support dates were not on
  the page.
- **Risks:** a very new TypeScript major; a pinned Node is needed for the
  build and is development-only (CR-006).
- **Selected:** Angular 22, with Node and TypeScript pinned by hash in the
  build. **Reconsider if:** the Material release for 22 lags.
- **Confidence:** high for existence and support; Material 22 not checked.

## RQ-003 Build system

- **Decision:** Bazel replaces Cargo completely (DR-013).
- **Alternatives:** Cargo with a task runner; Bazel for the product only.
- **Findings:** Verified (SR-014). The rules_rust documentation says: "We
  aim to support Linux and macOS" and "We do not have sufficient
  maintainer expertise to support Windows." It is Apache-2.0, tests against
  the latest rolling Bazel and Bazel 7.4.1 as the oldest on `main`, and is
  set up with `MODULE.bazel`. How it handles `Cargo.lock` was not on the
  page.
- **Conflict:** Windows is a version 1 requirement (DR-005) and Bazel is the
  only build system (DR-013).
- **Owner decision (DR-052, 20 November 2026):** option (A) below.
- **Options that were put to the owner:** (A) keep Bazel and own the work
  of making rules_rust build on Windows, with GitHub Actions runners as the test
  bed; (B) Bazel on Linux, Cargo-driven builds for the Windows leg, with
  one source of truth for dependencies; (C) Cargo plus Bazel only for
  orchestration. The earlier recommendation considered (B) as a
  fallback. DR-052 selected (A). EX-03 failure holds the Windows
  requirement. It cannot re-admit a split build system.
- **Experiment EX-03:** build one small Rust crate, one Tauri shell and
  one Angular target with Bazel on the admitted hosted Windows and
  Linux profiles. Pin source, tools, runner, dependencies and outcome
  receipt. Hold an unadmitted compiler or hidden download.
- **Confidence:** high that Windows support is not promised by the rules.

## RQ-004 PostgreSQL

- **Decision:** PostgreSQL 18 with row-level security, embedded in the home
  folder (AGENTS.md 5.2).
- **Findings:** Verified (SR-015). PostgreSQL's Windows page offers an
  EDB zip archive of the binaries "intended for users who wish to include
  Postgres as part of another application installer". Its table lists
  versions 13 to 18 as tested on 64-bit Windows; it also announces
  PostgreSQL 19 but lists no Windows platforms for it. Debian and Ubuntu
  packages come from the PostgreSQL apt repository (SR-016).
- **Selected:** PostgreSQL 18, from the EDB zip on Windows and the official
  packages' binaries on Linux, pinned by hash. **Reconsider if:** 19 gains
  Windows listings and the owner wants it.
- **Confidence:** medium-high; the zip's licence terms and contents were
  not read.

## RQ-005 Vector search

- **Decision:** pgvector for vectors (DR-016).
- **Findings:** Verified (SR-016). pgvector v0.8.7 supports PostgreSQL 13
  and later, including 18. Debian and Ubuntu packages are available from
  the PostgreSQL apt repository. For Windows it offers no prebuilt
  binaries: you build from source with Visual Studio (`nmake`), or use
  Docker or conda-forge. The licence name was not on the page.
- **Conflict:** building from source breaks CR-008 (we compile no C or
  C++), and Docker is ruled out (CR-032).
- **Alternatives:** (1) an embedded Rust nearest-neighbour index in the
  engine, rebuilt from PostgreSQL at need; (2) pgvector on Linux and the
  Rust index on Windows, with one interface; (3) conda-forge binaries.
- **Recommended:** (1) one Rust index on both systems behind a vector
  interface, with pgvector optional later. **Why:** one behaviour on both
  systems, nothing compiled by us, and an index that can always be
  rebuilt.
- **Experiment EX-04:** measure recall, build time and memory of a Rust
  HNSW crate on 100,000 and 1,000,000 synthetic vectors on an 8 GiB
  machine. Not run. The crate is not chosen here.
- **Confidence:** high on the Windows finding; medium on the alternative.

## RQ-006 Text search

- **Decision:** embedded Tantivy for text search.
- **Findings:** Verified (SR-017). MIT licence; "supports Linux, macOS, and
  Windows"; Lucene-style BM25; stemming for 17 Latin-script languages, and
  third-party tokenizers for Chinese, Japanese and Korean; data is
  immutable, so an edit deletes and reindexes; mmap directory. The page
  gave no version and no memory figures.
- **Selected:** Tantivy. **Limits to carry into specs:** language coverage
  (version 1 is British English only), no in-place update, and memory under
  the search-cache share of spec 0078 to be measured.
- **Confidence:** high.

## RQ-007 Analytics store

- **Decision:** embedded DuckDB for analytics (DR-016).
- **Findings:** Verified (SR-018). DuckDB lists 2.0.0 (LTS), 1.5.6 and
  1.4.5 (LTS) for Linux, macOS and Windows; Windows needs the Visual C++
  Redistributable. The Rust client is `cargo add duckdb --features
  bundled`. The page did not state a licence.
- **Conflict:** the `bundled` feature builds DuckDB's C++ source, which
  breaks CR-008. Linking a prebuilt library is a different feature whose
  support was not checked here.
- **Options:** (A) allow a pinned C++ toolchain inside the sealed build for
  DuckDB only; (B) link an official prebuilt `libduckdb` if the Rust crate
  supports it; (C) run the official prebuilt DuckDB command-line program as
  a supervised child process; (D) use PostgreSQL and Tantivy for analytics.
- **Recommended:** (B) if confirmed, else (C). **Experiment EX-05:** read
  the `duckdb` crate documentation for prebuilt linking and test a prebuilt
  library on both systems. Not run.
- **Confidence:** high on the finding.

## RQ-008 Model runtime

- **Decision:** ONNX models run in Rust with `tract` or `ort` (DR-020).
- **Findings:** Verified (SR-019, SR-020). `ort` is Apache-2.0 or MIT; its
  badge shows ONNX Runtime v1.31.0, but its API reference points to
  2.0.0-rc.15, a release candidate; Windows, Linux, download behaviour and
  accelerator support were not on the page. `tract` is Apache-2.0 or MIT,
  loads ONNX but cannot save it, runs on Linux, macOS and Windows, shows no
  Windows CI badge, and lists no operator coverage; it has transformer
  examples.
- **Risks:** `ort` is a release candidate; `tract`'s coverage of the BGE-M3
  and reranker graphs is unknown; ONNX Runtime itself is a prebuilt native
  library (CR-008).
- **Experiment EX-01:** export BGE-M3 and the BGE reranker to ONNX, run each
  through `tract` and through `ort` on both systems, compare outputs against
  the original on a fixed test set, and record memory and speed. The
  runtime is not chosen by this observation. Use the EX-01 evidence
  and format boundary at the end of this dossier.
- **Confidence:** medium.

## RQ-009 Local language models

- **Decision:** GGUF language models with a Vulkan or CPU route (DR-020).
- **Findings:** Verified (SR-021, SR-022). llama.cpp is MIT. Its latest
  release (b12556, shown as a pre-release, 19 November) lists prebuilt
  assets for Windows x64 with Vulkan (`win-vulkan-x64.zip`), Windows CPU,
  Ubuntu x64 with Vulkan (`ubuntu-vulkan-x64.tar.gz`) and Ubuntu CPU, plus
  other back ends. The README page did not mention Vulkan or Rust bindings.
- **Selected approach:** run the pinned prebuilt `llama-server` as a child
  process of the engine supervisor over a local pipe or socket. This
  needs no C++ compile and keeps language models out of the ranking path.
  **Risk:** releases are frequent and marked pre-release; pin an exact
  build by hash. **Reconsider if:** a maintained Rust binding with prebuilt
  libraries appears.
- **Confidence:** high for existence; medium for stability.

## RQ-010 Formal checks

- **Decision:** Quint models, run fast at commit and push and deeply at
  release (ADR-0021).
- **Findings:** Verified (SR-023, SR-024, SR-025). Quint is Apache-2.0 and
  installs with npm; the model checker (Apalache) needs a JDK 17 or newer,
  with Temurin or Zulu 17 suggested; it is also offered through Homebrew,
  Nix and prebuilt binaries. The getting-started page documents `quint run`
  and `quint verify` and does not mention Windows. Quint Connect exists
  (Apache-2.0, Rust 1.70 or newer, needs Quint on the path); its release
  and maintenance status was not visible.
- **Not found:** `typecheck` and `test` commands on that page, a Rust
  simulator back end, Windows support, and the "quint-lang" and
  "quint-modeling" agent skills.
- **Selected:** keep Quint, with the commands and back ends in the gate
  design limited to what the documentation confirms. **Experiment EX-06:**
  install Quint and Apalache on both systems, run a small model through
  every command the gate would use, and record time. Not run.
- **Confidence:** medium.

## RQ-011 Leviathan rendering and JavaScript engine

- **Decision:** which existing engine renders pages and runs scripts in
  Audit mode (ADR-0022). Receipts mode runs no scripts.
- **Alternatives:** (1) Servo's embeddable WebView; (2) Boa, a Rust
  JavaScript engine; (3) a prebuilt V8 through a Rust binding; (4) a
  managed headless browser driven over its debugging protocol.
- **Findings:** Servo (SR-026) describes itself as an embeddable web
  rendering engine written in Rust with a WebView API for Windows, macOS,
  Linux and others; its licence, release version and API maturity were not
  on the page. Boa (SR-027) is described as experimental, MIT or
  Unlicense, version 0.21.0, over 90% of the ECMAScript specification, and
  not a browser (no page tree or layout). V8 and headless browsers were not
  researched.
- **Criteria:** memory in the 6 GiB budget, Windows and Linux builds without
  our compiling C++, honest identity, robots and quota limits, fidelity.
- **Experiment EX-02:** use a set of 200 representative synthetic, own-site
  or expressly authorised fixture pages. Record memory, time, fidelity
  against the approved reference and build effort for each candidate in
  each platform cage. Repeat checks stay within the test fixture set.
  The choice requires those results.
- **Confidence:** low; this is open.

## RQ-012 Supported operating systems

- **Decision:** which Linux and Windows versions version 1 supports.
- **Findings:** Verified (SR-028 to SR-030). Ubuntu 26.04 LTS was released
  in April 2026 with standard security maintenance to May 2031. Debian 13
  (trixie) is the current stable, released 9 August 2025, with full support
  to 9 August 2028 and LTS to 30 June 2030. Windows 10 reached end of
  support on 14 October 2025.
- **Resolved:** version 1 supports Ubuntu 26.04 LTS and Debian 13 on
  x86-64, and Windows 11 on x86-64. Windows 10 is not supported. Spec 0030
  is updated. Ubuntu 24.04 LTS is not listed and is out of scope unless the
  owner adds it.
- **Confidence:** high.

## RQ-013 Windows build and test runners

- **Decision:** GitHub Actions Windows runners (DR-010).
- **Findings:** Verified (SR-031). Standard runners for public repositories
  have 4 vCPUs, 16 GB of memory and 14 GB of storage on Linux and Windows,
  with labels `windows-latest` and `windows-2025`, and `ubuntu-latest`,
  `ubuntu-24.04` and `ubuntu-26.04`. Use of standard runners on public
  repositories is "free and unlimited".
- **Limit:** 14 GB of storage is small for Bazel caches and models; use
  remote caches and synthetic data. The repository must stay public.
- **Confidence:** high.

## RQ-014 Memory limit enforcement

- **Decision:** how to enforce hard memory limits on app-managed processes
  without systemd units (MB-15, QR-014).
- **Findings:** Windows (SR-032, verified): job objects limit a process
  group, can enforce working-set and other limits, and can terminate all
  processes in the job; the memory-limit fields are in
  `JOBOBJECT_EXTENDED_LIMIT_INFORMATION`. Linux (SR-033, verified from the
  kernel documentation): cgroup v2 can be delegated to an unprivileged user
  by giving write access to a subtree; controllers can be enabled only after
  processes are moved into child groups. The page does not say whether
  systemd is required.
- **Selected:** Windows job objects. Linux: use a delegated cgroup v2
  subtree when the user's session provides one, else a monitor that reads
  resident memory every second and stops a process at the safe point or at
  125% of its promise (spec 0078). **Experiment EX-07:** check on Ubuntu
  26.04 and Debian 13 desktop sessions whether a user subtree is delegated.
- **Confidence:** medium.

## RQ-015 Spring Boot facts

- **Decision:** what the parity ledger tracks (ADR-0022).
- **Findings:** Verified (SR-034 to SR-037). The Spring Boot releases page
  lists v4.2.0 (19 November), v4.1.3 and v4.0.10 (19 November), v4.2.0-RC2
  (5 November), v4.2.0-RC1 (22 October) and v4.1.2 (22 October, then
  "Latest"). So the owner's 4.1.2 and 4.2.0-RC2 were right when written and
  4.2.0 is now released. The 4.1 notes confirm gRPC support for servers and
  clients, an `InetAddressFilter` for HTTP clients (SSRF protection), lazy
  DataSource connections, `@Async` context propagation, and log rotation
  strategies (the notes say for Log4j) and Docker Compose changes. The 4.2.0
  notes confirm generic AMQP 1.0 and RabbitMQ AMQP 1.0 starters,
  OpenTelemetry convention and OTLP settings, LDAP over SSL bundles,
  GraphQL method settings, Buildpacks build caches, and a properties
  migrator option that can fail the application on errors or warnings. They
  also list deprecations: `RestTemplate`, Apache Pulsar support and
  `server.forward-headers-strategy`.
- **Source follow-up, 21 November 2026 (SR-039 to SR-046).**
  - Spring Boot 4.0.0 GA was announced on 20 November 2025. Its release
    notes identify API versioning auto-configuration for Spring MVC
    and WebFlux. The rolling servlet and reactive references corroborate
    both behaviours. They are rolling references, and the separate
    version-pinned 4.2.0 page was unavailable to fetch. Exact 4.2.0
    contract evidence and local parity remain open.
  - The Spring Security documentation dates `InetAddressMatcher` to
    Security 7.1. The dated Spring announcement records availability of
    7.1.0 on 9 June 2026. Spring Boot's 4.2 release notes list Spring
    Security 7.2.0 as a dependency. The Security matcher interface and
    the Boot HTTP-client `InetAddressFilter` are separate items. These
    sources do not fix the local matcher variant or its parity scope.
  - Spring AI announced 2.1.0 GA on 20 November 2026 and states that
    2.1 is built on Spring Boot 4.2. This confirms a documented version
    relationship. Exact train membership and the applicable Spring AI
    capabilities still need separate evidence.
- **Version baseline, 21 November 2026 (SR-043 and SR-046 to SR-054).**
  - Boot 4.2.0 release notes, Dependency Upgrades, list Framework
    7.1.0, Security 7.2.0, Data 2026.1.0, Batch 6.1.0, Integration
    7.2.0, GraphQL 2.1.0, Kafka 4.2.0, AMQP 4.2.0, Session 4.2.0,
    LDAP 4.2.0 and Spring Web Services 5.1.0. They also list
    Spring HATEOAS 3.2.0 and Spring REST Docs 4.1.0.
  - The published Boot managed coordinates served as 4.2.0 on
    the review date list Spring gRPC core 1.1.2. The Spring gRPC
    release tag independently confirms that version exists.
    These sources give dependency versions. Per-project compatibility
    and per-behaviour evidence come from their owning releases.
  - The 19 November Spring Modulith 2.2 GA announcement and
    2.2.0 release identify builds against Boot 4.2 and Framework
    7.1. The 20 November Spring AI 2.1.0 GA announcement identifies
    Boot 4.2 as its build baseline. Both are separately
    released projects with evidenced Boot version relationships.
  - The 20 November Spring Cloud 2026.0.0 (Paddington) GA
    announcement explicitly identifies Boot 4.2.0. The matching
    tag lists included module versions, a source lead for exact
    membership review. The rolling Cloud project table still lists
    2025.1.x for Boot 4.0.x and 4.1.x, and the Spring Support
    generation view returned no mappings for Boot 4.2.x in this
    fetch. Neither rolling view resolves 2026.0.0 membership or
    per-module support. The Cloud support policy ties support
    to included projects.
- **Still open (QR-004).** Freeze the exact official train, including
  Cloud membership and support, and each source version. Record each
  row's applicability, clean-room behaviour
  and owning profile. Distinguish a deprecation from a dated removal.
  The 4.2 notes corroborate the Pulsar deprecation and the existing
  Pulsar disposition remains. Disposition of other deprecated or
  removed items needs its exact source, compatibility and authority.
  These are source findings from official publications under DR-070.
  They confer no implementation, execution or policy authority.
- **Rule for the ledger:** a Behemoth Boot row exists only with an official
  release-note source and date. The Pulsar adapter in the owner's list is
  deprecated upstream and is not a parity target.
- **Selected:** lockstep target Spring Boot 4.2.0; the monthly scout starts
  from 4.2.0. **Confidence:** high for versions; medium for features.

### Spring Cloud 2026.0.0 GA component list

The 20 November 2026 GA release tag's *What's Included* section
lists the following 14 Spring Cloud components (SR-051).

| Component | Version |
|---|---|
| Spring Cloud Build | 5.1.0 |
| Spring Cloud Function | 5.1.0 |
| Spring Cloud Stream | 5.1.0 |
| Spring Cloud Commons | 5.1.0 |
| Spring Cloud Task | 5.1.0 |
| Spring Cloud Config | 5.1.0 |
| Spring Cloud Netflix | 5.1.0 |
| Spring Cloud OpenFeign | 5.1.0 |
| Spring Cloud Consul | 5.1.0 |
| Spring Cloud Circuitbreaker | 5.1.0 |
| Spring Cloud Gateway | 5.1.0 |
| Spring Cloud Zookeeper | 5.1.0 |
| Spring Cloud Kubernetes | 5.1.0 |
| Spring Cloud Vault | 5.1.0 |

The GA announcement calls out Starter Build at the train version
2026.0.0 in its update summary (SR-050). The tag separately lists
Spring Cloud Build 5.1.0. The table records the exact inclusions
published on the tag. It does not enumerate all managed artefacts.

The same GA announcement places Spring Cloud Bus under Spring Cloud
Stream, with the existing Bus artefact coordinates kept. The tag
therefore has a Stream row and no separate Bus row. Keep Bus behaviour
as a source investigation under Stream where applicable. The ownership
move is no proof of a behaviour removal, compatibility in every
profile or a local implementation decision.

The official train reference served RC2 and identified Boot 4.2.0-RC2
on this review date (SR-055). The Stream project Support link returned
its rolling overview without a 5.1.x support timetable (SR-056).
Use the dated GA tag and announcement for the version list and the
project-based support policy in SR-053. Keep exact OSS and enterprise
support dates, managed-artefact reconciliation, local applicability
and per-behaviour parity open under QR-004.

### Tagged dependency manifests for Spring Cloud 2026.0.0

SR-057 records the upstream dependency manifest at the GA tag and
release commit 936a18de72a3112b14b786b3b11026c459ae17d8. The
top-level bill of materials, or BOM, is the Maven configuration
that supplies managed versions. It is
org.springframework.cloud:spring-cloud-dependencies:2026.0.0.
Its declared parent is
org.springframework.cloud:spring-cloud-dependencies-parent:5.1.0.
The source sets Boot 4.2.0 as the documentation-build property.
The dated GA announcement in SR-050 supplies compatibility evidence.

The manifest directly imports these thirteen BOMs in this order.
Each has group org.springframework.cloud and version 5.1.0.

| Order | Imported artefact |
|---|---|
| 1 | spring-cloud-commons-dependencies |
| 2 | spring-cloud-netflix-dependencies |
| 3 | spring-cloud-stream-dependencies |
| 4 | spring-cloud-task-dependencies |
| 5 | spring-cloud-circuitbreaker-dependencies |
| 6 | spring-cloud-config-dependencies |
| 7 | spring-cloud-function-dependencies |
| 8 | spring-cloud-gateway-dependencies |
| 9 | spring-cloud-consul-dependencies |
| 10 | spring-cloud-vault-dependencies |
| 11 | spring-cloud-zookeeper-dependencies |
| 12 | spring-cloud-openfeign-dependencies |
| 13 | spring-cloud-kubernetes-dependencies |

The release's Build 5.1.0 list entry and the manifest's declared
parent are separate evidence from the direct import list. The
manifest has no separate Bus import. Stream's 5.1.0 BOM in SR-058
directly manages the following Bus coordinates. Each has group
org.springframework.cloud and uses the Stream project version,
which is 5.1.0 at this tag.

| Managed Bus artefact |
|---|
| spring-cloud-bus |
| spring-cloud-starter-bus-amqp |
| spring-cloud-starter-bus-kafka |
| spring-cloud-starter-bus-stream |

The tagged Bus reactor in SR-059 also lists spring-cloud-bus-tests
and spring-cloud-starter-bus-jgroups. A reactor lists the modules
built together. These two modules are absent from the direct
Stream BOM entries. Their publication and effective management
need separate evidence. SR-060 records the Stream release and
the repository integration that places Bus under Stream.

Spring project and Enterprise support policies are in SR-061 and
SR-062. The public tables inspected on 21 November 2026 did
not provide exact open-source or enterprise end dates for
Cloud 2026.0 or Stream 5.1. Universal support periods cannot
supply those exact dates.

To complete QR-004, reconcile the published top-level artefact,
its effective parent and each imported BOM with the tagged
source. Keep the upstream coordinate, owning manifest, version
and import path when a coordinate is repeated. Use the effective
version only after the published precedence and profile are
proved. Resolve support dates from exact project generations
and record local behaviour rows and approved dispositions
separately. This source inventory confers no runtime dependency,
application authority or qualification proof.

## RQ-016 XenForo licence

- **Decision:** whether six private XenForo test installs and an add-on
  under GPL are allowed (QR-001).
- **Findings:** the XenForo licence agreement returned HTTP 403 to the
  fetch tool, so it was not read. An earlier search summary (SR-006) says
  extra test installs are allowed if password-protected and limited to the
  owner and staff; that is unverified.
- **Status:** blocked. **What closes it:** the owner reads the live
  agreement or asks XenForo Limited; this register records the answer.
- **Confidence:** low.

## RQ-017 Search Console history and row limits

SR-063 and SR-064 record the official provider material used for
QR-226. Search Console documents a 16-month history. Search
Analytics has a maximum page size of 25,000 and exposes up to
50,000 rows for a site, day and search type. A short page or an
omitted row cannot prove complete coverage. Keep the provider's
retrieval horizon separate from Sutradhar's authorised retention.

Spec 0040 owns the exact request, paging and disclosure rules.
Spec 0146 uses the eligible history that has been saved. Spec
0150 still requires independent proof of each experimental cell.
The documents establish published limits. Property-specific
availability, profile admission and measured coverage need
their own evidence.

## Experiments, not yet run

| ID | Purpose | Closes |
|---|---|---|
| EX-01 | BGE-M3 and reranker ONNX candidates on `tract` and `ort` by supported OS | RQ-008 |
| EX-02 | Render 200 pages with each engine candidate | RQ-011 |
| EX-03 | Bazel with rules_rust on a Windows runner | RQ-003 |
| EX-04 | Rust vector index recall, time and memory | RQ-005 |
| EX-05 | DuckDB with a prebuilt library, both systems | RQ-007 |
| EX-06 | Quint and Apalache on both systems | RQ-010 |
| EX-07 | cgroup v2 delegation in desktop sessions | RQ-014 |
| EX-08 | Verify hermetic Bazel build targets, affected-target selection and cache identity | N-1805 and BJ-09/10/21/26. Supports QR-099. |

### EX-08 Build graph slicing

Use a synthetic module and starter graph with a declared shared input,
a dependent and an unrelated target. Pin the candidate, graph and input
digests, tools and sandbox profiles under spec 0003. Cover supported x86-64
Ubuntu 26.04 LTS, Debian 13 and Windows 11
proof routes, with the EX-03 tooling and QR-099's
hosted evidence identity on Windows.

- Run the valid graph clean. Edit the starter and then a declared
  shared input. Include a generated input edge. Record the affected
  closure and why each target ran, reused results or was unaffected.
  Exercise the undeclared-input denial SUT-1414 and the declared target
  size limit SUT-1406.
- Compare cold and warm output digests and check outcomes. Change an
  input and then a tool identity and require cache invalidation. Refuse
  stale, invalid or wrong-profile entries and local writes. An
  authenticated CI writer binds entry and result to candidate,
  platform, tool and dependency locks and protected policy and
  profile. Compare only the exact chosen outputs. Use independent
  0116 reproducible build and full phase proof separately.
- Record per-target build and test times, cache decisions and failures.
  The under-one-minute warm Linux target in spec 0114 is proposed.
  Record the Windows measurements separately.

EX-03 is the cross-platform toolchain probe using a small Rust crate,
Tauri shell and Angular target. EX-08 supplies graph and cache evidence.
Both support QR-099. Under 0003 verify the exact independently
reviewed candidate, old and new policy generations, complete check
map, locks, SBOM and platform fences. The current Dell entry points
keep their authority until a signed current authorisation and the
atomic recorded switch complete. Experiment or signature alone
cannot switch the gate.

### Proof required by the other experiments

These references describe planned proof. A result needs a record of
the exact tested inputs, platform and outcomes.

| ID | Owning proof | Evidence still required |
|---|---|---|
| EX-01 | [0024](../specs/0024-models.md) owns the model lock, UT-0024-EXPORT-PARITY, IT-0024-OFFLINE-LOAD and resource admission. [0077](../specs/0077-autonomous-ranking-optimisation.md) owns champion activation. QR-005 covers native prebuilt compatibility. | For candidate ONNX, run exact locked dense, sparse, multi-vector and cross-encoder fixtures on Ubuntu 26.04 LTS, Debian 13 and Windows 11 x86-64. Apply 0024 safe conversion. Admission of any selected production format also needs the authority below. Record parity, licence, platform compatibility, memory and time. The runtime and any ONNX native prebuilt path require separate admission. |
| EX-04 | DR-053 owns the choice of an embedded Rust index. [0022](../specs/0022-reading-indexing.md) owns representation identity, generation hand-off and resource proof. [0023](../specs/0023-matching-anchors.md) owns retrieval and candidate evidence. | Compare a pinned Rust index candidate with exact neighbour results at both specified synthetic vector volumes. Record recall, build time and memory on the 8 GiB profile. Verify supported platform behaviour before choosing a crate. |
| EX-05 | DR-054 and QR-005 own the prebuilt route decision and compatibility question. [0108](../specs/0108-behemoth-data-access.md) owns the SQL starter. [0034](../specs/0034-analytics.md) owns analytics. [0100](../specs/0100-outside-services-and-data-platform.md) owns read-only console checks SD-28 and SD-29. | Verify official binary, licence, version, digest and a supported link route on each intended platform. If linking does not qualify, prove the official supervised child-process route and its limits. |
| EX-06 | QR-006 and DR-012 own tool qualification and scope. [0003](../specs/0003-toolchain-and-check-runner.md) owns pins. [0114](../specs/0114-behemoth-verification.md), BJ-04, BJ-27 and UT-BJ-02, owns bounded properties and matching Rust tests. | Record a pinned Quint and Apalache command and runtime matrix with timings on Linux and Windows. Keep model-check results and matched Rust tests as distinct evidence. |
| EX-07 | QR-014 owns the enforcement question. [0078](../specs/0078-memory-budget-and-turn-taking.md), MB-15 and FI-MB-01, owns promise overrun. [0037](../specs/0037-extension-isolation.md) owns the separate extension cage. | Verify delegated cgroup v2 in ordinary supported Ubuntu and Debian desktop sessions and measure fallback when delegation is absent. Windows job-object proof is separate. A sampled memory monitor can substantiate its measured recovery bound. A hard cap and a cage each need their own proof. |

EX-01 is an ONNX candidate experiment. It cannot select production
model format. ADR-0021 describes ONNX embeddings and rerankers.
Spec 0024 prefers an independently admitted safetensors cross-encoder
if parity qualifies it. Before selecting that route, reconcile the
format scope with the existing owner authority. Qualify the ONNX
graph, operators, local external weights and exact runtime/native
ABI independently of direct safetensors serving. The latter needs
the exact Rust graph implementation, model and tokenizer and its own
equivalent output, resource and security proof. A safetensors source
for conversion gives no direct serving approval. Neither result
changes the 0077 activation path or the BGE and Python boundaries.
