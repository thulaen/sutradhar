# Constraint register

A constraint is a rule the design must obey. MUST is mandatory. Each row
names its source so a reader can check it. Dates are 2026.

## Languages, tools and build

| ID | Constraint | Source |
|---|---|---|
| CR-001 | First-party production code is Rust. | ADR-0021; owner, 19 Nov |
| CR-002 | The front end is Angular 22 with Angular Material in strict TypeScript, with Angular templates and CSS. | ADR-0021 |
| CR-003 | First-party Python exists only for the sidecar and user-script host, is small, pinned, tested and boxed in. | ADR-0021 |
| CR-004 | First-party Go MUST NOT exist. | ADR-0020, ADR-0021 |
| CR-005 | Hand-written shell is limited to install, packaging and operations glue and passes ShellCheck and shfmt. | ADR-0021 |
| CR-006 | Quint, Node.js, a Java runtime, Lean 4 and Alloy are development and release tools only, pinned and never shipped. | ADR-0021 |
| CR-007 | Bazel builds and tests everything and replaces Cargo. | ADR-0021 |
| CR-008 | We MUST NOT compile C or C++. Third-party native libraries are official prebuilt binaries, pinned by hash and scanned. | ADR-0021 |
| CR-009 | A source file has at most 300 lines, a function 60 and a line 80 characters; every module has runnable doc-tests. | AGENTS.md 6; owner, 19 Nov |
| CR-010 | A commit changes at most 300 lines and about 10 files; a push holds at most 5 commits. | AGENTS.md 8.5; owner, 19 Nov |
| CR-011 | Tests come before code; every behaviour and failure has a test; no error is swallowed. | AGENTS.md 6 |
| CR-012 | Before writing a component, record a build-or-reuse decision; do not reinvent the wheel. | AGENTS.md 6 rule 14 |
| CR-013 | Code follows DO-178C objectives, translated MISRA rules and NASA's coding rules. | AGENTS.md 6; owner |
| CR-014 | At most ten agents work at once. | AGENTS.md 8.2; owner, 19 Nov |

## Architecture and data

| ID | Constraint | Source |
|---|---|---|
| CR-020 | The product is a modular monolith on Behemoth Boot, with module walls, no cycles and public APIs. | AGENTS.md 5.1; ADR-0022 |
| CR-021 | PostgreSQL is the system of record, with row-level security forced on every table. | AGENTS.md 5.2 |
| CR-022 | Search runs on PostgreSQL full-text search and embedded Tantivy; vectors use an embedded Rust index, not pgvector. Analytics runs on embedded DuckDB. There is no Kafka, OpenSearch, Elasticsearch or Airflow. | ADR-0021 |
| CR-023 | Events use an outbox in the same transaction; delivery is at least once and every consumer is idempotent. | ADR-0020 |
| CR-024 | Rust makes every live ranking decision. Python and local language models never take part. | ADR-0021 |
| CR-025 | A person accepts every site edit. Scripts, workflows and agents only create proposals. | ADR-0021; owner, 11 Nov |
| CR-026 | Site messages are signed (Ed25519) with time, single-use number and request ID. | AGENTS.md 5.4 |
| CR-027 | Local access is a user-owned socket or named pipe, never a network port. | ADR-0021 |
| CR-028 | MCP is local only and never reaches tokens and keys, permission rules, the confirm step, audit settings or switches that send site text off the PC. | ADR-0021 |

## Platforms, deployment and resources

| ID | Constraint | Source |
|---|---|---|
| CR-030 | Site platforms are XenForo 2.3 and the two newest WordPress majors only. | ADR-0021 |
| CR-031 | Linux is Ubuntu 26.04 and Debian 13 as a signed .deb; Windows 11 has an unsigned installer; Windows 10 and macOS are out. | ADR-0021; DR-055 |
| CR-032 | The engine runs as app-managed processes in one home folder; no systemd, Docker or registry use. | ADR-0021 |
| CR-033 | Memory budget is automatic, 6 GB by default, with a user slider and room left for the operating system. | ADR-0021; spec 0078 |
| CR-034 | When memory is short, crawls pause first, then training, then the rest by fairness and size. | ADR-0021; spec 0078 |
| CR-035 | At most one heavy model loads at a time on a tight budget; every model load goes through the scheduler. | ADR-0021 |
| CR-036 | One install manages many sites, each with separate data, keys, models and settings. | ADR-0021 |
| CR-037 | Updates are signed and come from GitHub Releases; the project hosts no server. | ADR-0021 |
| CR-038 | Windows builds and tests run on GitHub Actions Windows runners. | ADR-0021 |
| CR-039 | Every task is point and click with zero terminal use. | AGENTS.md 5.3; owner, 19 Nov |

## Data permissions, services and offline use

| ID | Constraint | Source |
|---|---|---|
| CR-040 | The core works fully offline. Outside services are optional, off by default and use the user's own accounts. | ADR-0021 |
| CR-041 | Site text leaves the PC only through a route the owner turned on. | AGENTS.md 5.5 |
| CR-042 | Raw visitor events are kept for the period the owner sets, 13 months by default, then only aggregates. | ADR-0021 |
| CR-043 | Model downloads are hash-checked and use safe formats only (ONNX, safetensors, GGUF). | ADR-0021 |
| CR-044 | Private material (keys, tokens, forum data) never enters the repository. | AGENTS.md 11 |

## Licensing

| ID | Constraint | Source |
|---|---|---|
| CR-050 | The app and shared libraries are Apache-2.0; the add-ons are GPL-2.0-or-later. | ADR-0021 |
| CR-051 | Only permissive dependencies ship in the app: MIT, Apache-2.0, BSD, ISC, Zlib, Unicode, MPL-2.0, PostgreSQL, PSF-2.0. | AGENTS.md 5.7 |
| CR-052 | Contributors sign off commits (DCO); there is no contributor licence agreement. | ADR-0021 |

## Quality and release

| ID | Constraint | Source |
|---|---|---|
| CR-060 | nDCG@10 is at least 95% and challenger Precision@6 is at least 1.2 times BM25 on every eligible test forum. | ADR-0020; AGENTS.md 3 |
| CR-061 | At least 360 distinct ranking methods ship, in 30 families of at least 12; a setting change never counts. | ADR-0021; spec 0079 |
| CR-062 | Every release candidate beats or matches the verified champion; rollback is automatic when protected metrics fail. | ADR-0020 |
| CR-063 | Quality is judged on six synthetic test sites, three XenForo and three WordPress, by three AI makers. | ADR-0021 |
| CR-064 | The never-off guardrails stay on: human accept, signed messages, secret masking, memory and disk caps, undo snapshots. | ADR-0021 |
| CR-065 | Behemoth Boot and Leviathan each stay within a tracked target of four million meaningful lines, with anti-padding rules. | ADR-0022 |
| CR-066 | Leviathan obeys robots.txt and every opt-out, never bypasses locks, and never pretends to be another company's bot. | ADR-0022 |

## Prohibited approaches

| ID | Constraint | Source |
|---|---|---|
| CR-070 | No trading-indicator content in the product or docs. | Owner, 17 and 19 Nov |
| CR-071 | No count quotas for methods, capabilities or variants. | ADR-0021 |
| CR-072 | No hand-written JavaScript or HTML outside the Angular app. | AGENTS.md 4 |
| CR-073 | No `--no-verify`, skipped check or faked success. | AGENTS.md 6 |
| CR-074 | No clean-room claim without a record of what was read and by whom. | ADR-0020 |
