# Source register

Sources the project has actually used. "Read" means the page itself was
read. "Fetched" means a fetch tool returned a summary of the page, so a claim
is verified only as far as that summary shows it. "Search only" means only a
search summary was seen; such a claim
stays unverified until the research dossier reads the page. Retrieval
dates are 2026.

| ID | Source | Used for | Retrieved | Evidence |
|---|---|---|---|---|
| SR-001 | Tauri 2 documentation, Windows installer page (v2.tauri.app/distribute/windows-installer) | Windows installers can be cross-built from Linux with NSIS only as a last resort; MSI needs Windows; signing needs an external tool | 19 Nov | Fetched |
| SR-002 | Apache Airflow installation prerequisites (airflow.apache.org) | Native Windows is not supported; WSL2 or containers; Linux for production | 19 Nov | Search only |
| SR-003 | Elastic licensing FAQ and 2024 announcement | AGPL added as an option from Elasticsearch 8.16 source; binaries unchanged | 19 Nov | Search only |
| SR-004 | Kafka on Windows guides (Conduktor, Confluent) | Windows is not a first-class Kafka platform | 19 Nov | Search only |
| SR-005 | OpenSearch installation and OS compatibility pages | Windows zip with its own JDK; tested list shows Windows Server 2019 | 19 Nov | Search only |
| SR-006 | XenForo license agreement (xenforo.com/license-agreement) | Test installs allowed if password-protected and limited to owner and staff | 19 Nov | Search only |
| SR-007 | Apache-2.0, AGPL-3.0 and GPL texts | Licence trade-off advice | 19 Nov | Not read; general knowledge, to verify |
| SR-008 | Angular release listings (several blogs and release pages) | Angular 22 stable on or about 3 June 2026; superseded by SR-013 | 19 Nov | Search only |
| SR-009 | The owner's pasted lists of 19 November 2026 | Features for the Jobs screen, Python scripting, Behemoth Boot and Leviathan | 19 Nov | Owner input, unverified for outside facts |
| SR-010 | Owner answers through the question tool, 19 November 2026 | Every decision in decisions.md | 19 Nov | Primary |
| SR-011 | Earlier owner messages in prior agent sessions | The history in the decision ledger | 19 Nov | Primary, mined by script |
| SR-012 | Tauri 2 prerequisites page (v2.tauri.app/start/prerequisites) | webkit2gtk 4.1 on Linux; WebView2 on Windows | 20 Nov | Fetched |
| SR-013 | Angular versions page (angular.dev/reference/versions) | Angular 22 supported; Node and TypeScript ranges | 20 Nov | Fetched |
| SR-014 | rules_rust README and documentation (github.com/bazelbuild/rules_rust, bazelbuild.github.io/rules_rust) | Apache-2.0; Linux and macOS aimed at; no Windows maintainers | 20 Nov | Fetched |
| SR-015 | PostgreSQL Windows download page (postgresql.org/download/windows) | EDB zip of binaries for embedding; versions 13 to 18 tested on Windows | 20 Nov | Fetched |
| SR-016 | pgvector README (github.com/pgvector/pgvector) | v0.8.7; PostgreSQL 13 and later; no Windows prebuilts; apt packages | 20 Nov | Fetched |
| SR-017 | Tantivy README (github.com/quickwit-oss/tantivy) | MIT; Linux, macOS and Windows; BM25; immutable data | 20 Nov | Fetched |
| SR-018 | DuckDB installation page (duckdb.org/docs/2.0/installation.html) | Versions 2.0.0 LTS, 1.5.6, 1.4.5 LTS; Rust crate with bundled feature | 20 Nov | Fetched |
| SR-019 | `ort` README (github.com/pykeio/ort) | Apache-2.0 or MIT; ONNX Runtime 1.31.0 badge; 2.0.0-rc.15 docs | 20 Nov | Fetched |
| SR-020 | `tract` README (github.com/sonos/tract) | Apache-2.0 or MIT; ONNX loading; Windows with no CI badge | 20 Nov | Fetched |
| SR-021 | llama.cpp README (github.com/ggml-org/llama.cpp) | MIT; prebuilt binaries on the releases page | 20 Nov | Fetched |
| SR-022 | llama.cpp releases page, build b12556 | Prebuilt Windows and Ubuntu x64 Vulkan assets | 20 Nov | Fetched |
| SR-023 | Quint README (github.com/informalsystems/quint) | Apache-2.0; npm install; points to Quint Connect | 20 Nov | Fetched |
| SR-024 | Quint getting-started page (quint.sh/docs/getting-started) | JDK 17 or newer for Apalache; `quint run` and `verify` | 20 Nov | Fetched |
| SR-025 | Quint Connect README (github.com/quint-co/quint-connect) | Exists; Apache-2.0; Rust 1.70 or newer | 20 Nov | Fetched |
| SR-026 | Servo home page (servo.org) | Embeddable Rust rendering engine; WebView API | 20 Nov | Fetched |
| SR-027 | Boa README (github.com/boa-dev/boa) | Experimental Rust JavaScript engine; MIT or Unlicense; 0.21.0 | 20 Nov | Fetched |
| SR-028 | Ubuntu release cycle (ubuntu.com/about/release-cycle) | 26.04 LTS released April 2026; support to May 2031 | 20 Nov | Fetched |
| SR-029 | Debian releases page (debian.org/releases) | Debian 13 trixie stable; support dates | 20 Nov | Fetched |
| SR-030 | Windows 10 lifecycle page (learn.microsoft.com) | End of support 14 October 2025 | 20 Nov | Fetched |
| SR-031 | GitHub-hosted runners reference (docs.github.com) | 4 vCPU, 16 GB, 14 GB; labels; free on public repositories | 20 Nov | Fetched |
| SR-032 | Windows job objects (learn.microsoft.com) | Limits and termination of process groups | 20 Nov | Read in full |
| SR-033 | Linux kernel cgroup v2 documentation (docs.kernel.org) | Delegation to an unprivileged user | 20 Nov | Fetched, first 100,000 characters |
| SR-034 | Spring Boot project page (spring.io/projects/spring-boot) | Shows 4.1.2 (stale cache) | 20 Nov | Fetched |
| SR-035 | Spring Boot releases on GitHub | 4.2.0, 4.1.3, 4.0.10 on 19 Nov; RC2 on 5 Nov; 4.1.2 on 22 Oct | 20 Nov | Fetched, first 100,000 characters |
| SR-036 | Spring Boot 4.1 release notes (GitHub wiki) | gRPC, InetAddressFilter, lazy DataSource, async context, log rotation | 20 Nov | Fetched, first 100,000 characters |
| SR-037 | Spring Boot 4.2.0 release notes (GitHub wiki) | AMQP 1.0, OpenTelemetry, LDAP SSL bundle, deprecations | 20 Nov | Fetched, first 100,000 characters |
| SR-038 | XenForo license agreement (xenforo.com) | Not read: HTTP 403 to the fetch tool | 20 Nov | Failed |
| SR-039 | [Spring Boot 4.0.0 GA announcement](https://spring.io/blog/2025/11/20/spring-boot-4-0-0-available-now/) | Dates Spring Boot 4.0.0 GA to 20 November 2025 | 21 Nov | Fetched, dated announcement |
| SR-040 | [Spring Boot 4.0 release notes](https://github.com/spring-projects/spring-boot/wiki/Spring-Boot-4.0-Release-Notes#api-versioning) | API versioning auto-configuration in Spring MVC and WebFlux | 21 Nov | Fetched, API Versioning section |
| SR-041 | [Spring Boot servlet reference](https://docs.spring.io/spring-boot/reference/web/servlet.html) | Rolling reference corroborates MVC API versioning | 21 Nov | Fetched, API Versioning section. Rolling reference only |
| SR-042 | [Spring Boot reactive reference](https://docs.spring.io/spring-boot/reference/web/reactive.html) | Rolling reference corroborates WebFlux API versioning | 21 Nov | Fetched, API Versioning section. Rolling reference only |
| SR-043 | [Spring Boot 4.2 release notes](https://github.com/spring-projects/spring-boot/wiki/Spring-Boot-4.2-Release-Notes) | Lists the Boot 4.2.0 Spring dependency upgrades and records Apache Pulsar support as deprecated | 21 Nov | Fetched, Dependency Upgrades and Deprecated Apache Pulsar Support sections |
| SR-044 | [Spring Security InetAddressMatcher API](https://docs.spring.io/spring-security/reference/api/java/org/springframework/security/util/matcher/InetAddressMatcher.html) | Identifies the interface and states “Since 7.1” | 21 Nov | Fetched, API reference and Since field |
| SR-045 | [Spring Security June 2026 release announcement](https://spring.io/blog/2026/06/09/spring-security-releases-2026-06/) | Dates Spring Security 7.1.0 availability to 9 June 2026 | 21 Nov | Fetched, dated announcement |
| SR-046 | [Spring AI 2.1.0 GA announcement](https://spring.io/blog/2026/11/20/spring-ai-2-1-0-GA-available-now/) | On 20 November 2026 the Spring AI team announced 2.1.0 GA and stated its relationship to Spring Boot 4.2 | 21 Nov | Fetched, dated announcement and upgrade notes |
| SR-047 | [Spring Boot Managed Dependency Coordinates](https://docs.spring.io/spring-boot/appendix/dependency-versions/coordinates.html) | Served Boot 4.2.0 coordinates with `org.springframework.grpc:spring-grpc-core` 1.1.2 | 21 Nov | Fetched, published coordinate rows. Rolling URL |
| SR-048 | [Spring gRPC v1.1.2 release](https://github.com/spring-projects/spring-grpc/releases/tag/v1.1.2) | Published release tag corroborates the gRPC version | 21 Nov | Fetched, tagged release |
| SR-049 | [Spring Modulith 2.2 GA announcement](https://spring.io/blog/2026/11/19/spring-modulith-2-2-ga-2-1-3-and-2-0-10-released/) and [2.2.0 release](https://github.com/spring-projects/spring-modulith/releases/tag/2.2.0) | 19 November 2026 announcement records builds against Boot 4.2 and Framework 7.1 | 21 Nov | Fetched, Platform and dependency upgrades and exact release |
| SR-050 | [Spring Cloud 2026.0.0 GA announcement](https://spring.io/blog/2026/11/20/spring-cloud-2026-0-0-has-been-released/) | 20 November 2026 announcement explicitly identifies Boot 4.2.0 as the basis of Paddington GA | 21 Nov | Fetched, dated announcement and Notable Changes section |
| SR-051 | [Spring Cloud v2026.0.0 release](https://github.com/spring-cloud/spring-cloud-release/releases/tag/v2026.0.0#whats-included) | Exact train tag and included module versions | 21 Nov | Fetched, What's Included. Membership and applicability review open |
| SR-052 | [Spring Cloud project page](https://spring.io/projects/spring-cloud/) | Rolling table has 2025.1.x for Boot 4.0.x and 4.1.x | 21 Nov | Fetched, release-train table |
| SR-053 | [Spring Cloud support policy](https://github.com/spring-cloud/spring-cloud-release/wiki/Supported-Versions#spring-cloud-support-policy) | Describes project-based support and directs readers to the official generation and support views | 21 Nov | Fetched, support policy and Releases sections |
| SR-054 | [Spring Support generation mappings](https://spring.io/projects/generations/) | Fetch exposed Boot 4.2.x as selected and returned no generation mappings | 21 Nov | Fetched, rendered text only. No negative compatibility inference |
| SR-055 | [Spring Cloud 2026.0 train reference](https://docs.spring.io/spring-cloud-release/reference/2026.0/index.html) | Returned Cloud 2026.0.0-RC2 with Boot 4.2.0-RC2 | 21 Nov | Fetched, release-train heading and compatible Boot field. Pre-release view |
| SR-056 | [Spring Cloud Stream project support](https://spring.io/projects/spring-cloud-stream/#support) | Returned the rolling 5.0.4 project overview without 5.1.x support dates | 21 Nov | Fetched, rendered text only. Exact support dates open |
| SR-057 | [Spring Cloud tagged 2026.0.0 dependency manifest](https://github.com/spring-cloud/spring-cloud-release/blob/v2026.0.0/spring-cloud-dependencies/pom.xml) | Source parent 5.1.0 and ordered thirteen direct component imports at 5.1.0. Git blob 5eacb3767c54d4091b8641f597d233a87829cc3d. GA link binds 936a18de72a3112b14b786b3b11026c459ae17d8 | 21 Nov | Tagged source, fetched and checked through GitHub project content. Effective published artefact needs separate proof |
| SR-058 | [Spring Cloud Stream tagged 5.1.0 dependency manifest](https://github.com/spring-cloud/spring-cloud-stream/blob/v5.1.0/bom/spring-cloud-stream-dependencies/pom.xml#L101-L121) | Four exact direct managed Bus coordinates at the project version. Git blob 2ec4cc559ede25d3374995fefa308c37b2ec21db | 21 Nov | Tagged source via GitHub project content |
| SR-059 | [Spring Cloud Stream tagged 5.1.0 Bus reactor](https://github.com/spring-cloud/spring-cloud-stream/blob/v5.1.0/bus/pom.xml#L7-L27) | Bus parent and six reactor modules. Git blob afd7a300d785ae4ab2aa7c9b1b20602f6c0b2e63 | 21 Nov | Tagged source via GitHub project content |
| SR-060 | [Spring Cloud Stream 5.1.0 release](https://github.com/spring-cloud/spring-cloud-stream/releases/tag/v5.1.0) and [Bus integration record](https://github.com/spring-cloud/spring-cloud-stream/pull/3279) | Release includes the Bus repository integration, with Bus under Stream | 21 Nov | Fetched tagged release and upstream merge record |
| SR-061 | [Spring project support policy](https://spring.io/support-policy/) | Explains Boot and portfolio support policy and the project-generation mapping | 21 Nov | Fetched policy. Exact current Cloud and Stream dates unverified |
| SR-062 | [Spring Enterprise release support](https://enterprise.spring.io/lts-releases/) | Gives portfolio support rules with separate Spring Cloud treatment | 21 Nov | Fetched rolling list. No exact date for the Cloud train or Stream generation in the inspected rows |
| SR-063 | [Google Search Console retention](https://support.google.com/analytics/answer/10737381?hl=en) and [Search Analytics query guide](https://developers.google.com/webmaster-tools/v1/how-tos/search_analytics) | Published 16-month retention and Search Analytics access to available Performance report data. Actual coverage depends on the property and request. | 21 Nov | Read provider help and API guide |
| SR-064 | [Search Analytics data limits](https://developers.google.com/webmaster-tools/v1/how-tos/all-your-data), [provider explanation of row limits](https://developers.google.com/search/blog/2022/10/performance-data-deep-dive) and [query reference](https://developers.google.com/webmaster-tools/v1/searchanalytics/query) | Maximum 25,000 per page and up to 50,000 per property, day and search type. A short result can still exclude data. | 21 Nov | Read provider guides and method contract |
| SR-065 | [Liben-Nowell and Kleinberg, The Link-Prediction Problem for Social Networks, section 3](https://www.cs.carleton.edu/faculty/dln/papers/link-prediction/link.pdf) | Pairwise Katz proximity weights walks by length. Used as a mathematical reference for the synthetic 0066 known-answer case. It establishes no Sutradhar graph profile or measured resource limits. | 21 Nov | Authors’ publication, pairwise Katz definition and matrix form |
