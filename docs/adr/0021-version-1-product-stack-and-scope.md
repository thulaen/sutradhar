# ADR-0021: Version 1 product, stack and scope

Decided by the owner on 19 November 2026, answer by answer, through the
question tool. This record supplements ADR-0010 and ADR-0020 and
replaces the earlier decisions named in the supersession map below. It
is a documentation and architecture decision. It does not authorise
application code, executable tests, build files, deployment
configuration or infrastructure changes.

## Product and audience

Sutradhar is free open-source software (Apache-2.0 for the app) for
other people to install, not only for its maker. It has no licence
keys and no telemetry by default. Every task is point and click: no
screen, message or manual step may tell a person to open a terminal,
type a command or edit a file.

Version 1 has five products on one shared platform:

1. internal linking for XenForo and WordPress;
2. the Editor Coach (Rank Math-style checks and score, Grammarly-style
   writing help, Conductor-style content intelligence, and link and
   structure coaching);
3. analytics that rival Adobe Analytics: real-time views, segments,
   cohorts, funnels, paths, attribution, anomaly alerts and forecasts;
4. Ganit Vikas, the research and optimisation engine, with a screen for
   users; and
5. the MCP server and the `sutra` command.

All features stay in version 1, including the 1,357 in the feature list
and about 470 from the owner's two lists of 19 November 2026 (Jobs
screen, hardware and scheduling controls, self-contained install, file
ledger, storage backends, MCP set-up and repair, Python scripting). Each
feature has one owning specification. The work is built in small slices
with signed 0.x releases; 1.0.0 is tagged only when every gate passes.

## Platforms and packaging

- Site platforms: XenForo 2.3 and the two newest WordPress major
  versions only. Shopify and headless CMSs are out of scope.
- Desktop: Tauri for Linux and Windows. Linux is Ubuntu 26.04 and Debian 13 as a
  signed `.deb`; no other Linux format is in version 1. Windows is a
  version 1 requirement built after the Linux engine works, with an
  unsigned installer and a plain-English warning page. macOS is out.
- The app starts and supervises its own engine as child processes inside
  one home folder on both systems. There is no systemd unit, no Docker
  and no registry use. The folder holds a file ledger; the uninstaller
  reads it and removes every file, with a checklist for user data. A
  portable single-folder mode exists.
- Local access: a Unix socket only the owner's account can open on
  Linux, and a user-locked named pipe on Windows. Never a network port.
- One install manages many sites. Each site keeps its own data, keys,
  models and settings, and memory is shared fairly between them.
- Updates are signed and come from GitHub Releases; the project hosts no
  server. Until 1.0.0 the 0.x releases are the only channel. From 1.0.0
  a pre-release channel is optional. Each update is explained in plain
  English and can roll back.
- Windows builds and tests run on GitHub Actions Windows runners.
  Forgejo stays the source of truth.

## Licences

The app and shared libraries are Apache-2.0. The WordPress plugin is
GPL-2.0-or-later, because WordPress.org expects GPL-compatible plugins;
the XenForo add-on is GPL-2.0-or-later too unless XenForo's Resource
Manager rules require otherwise (to verify). Contributors sign off each
commit (DCO); there is no contributor licence agreement, so the licence
cannot later be changed without every contributor's consent.

The owner chose Apache-2.0 so that companies, agencies and individuals
can contribute without licence worries. The accepted trade-off is that
anyone may fork and close the code or sell a hosted copy. The protections
are a trademark policy for the name Sutradhar and its logo, and the
quality of the models, test sets and release pipeline.

Only permissive code ships in the app: MIT, Apache-2.0, BSD-2-Clause,
BSD-3-Clause, ISC, Zlib, Unicode, MPL-2.0, the PostgreSQL licence and
PSF-2.0 for the bundled Python. No GPL, LGPL or AGPL code goes into the
app. The SBOM records every licence and a check refuses the rest. A
package that a user installs into a script's own environment is the
user's choice and not part of the app; the Hub and the installer show its
licence. The LICENSE file, `LICENSES/`, file headers, `Cargo.toml` and
`deny.toml` licence fields, the dependency allow-list and spec 0020's
source-offer footer change in later slices.

## Languages, tools and build

Rust owns every first-party production component, the Tauri shell, the
scheduler, the engine and ranking. The front end is Angular 22 with
Angular Material in strict TypeScript, with Angular templates and CSS.
Allowed development-only tools are Quint, Node.js and a Java runtime
(for Quint's checkers), Lean 4 and Alloy, each pinned by version and
hash and sealed in the build; none ships to users. Bazel builds and
tests everything (Rust, Angular and Python targets) and replaces Cargo
completely on both systems; the project owns the work of making the Rust
rules build on Windows, because rules_rust does not promise it (owner,
20 November 2026). `Cargo.toml` and `Cargo.lock` are not kept as build inputs.
Bazel's own files are Starlark.

First-party Go is banned. Hand-written shell is allowed only for
install, packaging and operations glue, under the ShellCheck, shfmt and
failure-path rules of ADR-0020; it never runs in the product's live
code and never builds a command from untrusted text.

We compile no C or C++. Third-party native libraries (PostgreSQL,
DuckDB, the GGUF runtime, ONNX Runtime, Quint's checkers) are
official prebuilt binaries, pinned by hash, scanned and listed in the
SBOM. Where no official prebuilt exists for a target system, that
component waits or is replaced.

Every source file has at most 300 lines, every function at most 60
lines and every line at most 80 characters. Every module has runnable
documentation tests. Standards: DO-178C objectives, translated MISRA
rules and NASA's coding rules, as ADR-0007 sets them out.

## Data and processing

The platform stack is PostgreSQL (full-text search), an embedded Rust
vector index (not pgvector, which has no Windows build; owner, 19
November 2026),
Tantivy (embedded search), DuckDB (embedded analytics) and our own
scheduler in Rust on Axum and Tokio. There is no Kafka, no
OpenSearch or Elasticsearch and no Airflow. PostgreSQL stays the system
of record; Tantivy indexes and DuckDB tables can be rebuilt from it.

Jobs and events use a PostgreSQL queue with an outbox in the same
transaction as the change. Delivery is at least once and every consumer
is idempotent, as ADR-0020 requires. The scheduler owns turn-taking: no
job starves, and memory waits are shown in plain English. When memory
runs short, site crawls pause first, then model training, then the rest
by fairness and size. Live review pages, Rust ranking, the window and a
running backup never pause. A Jobs screen shows queue, running and
waiting jobs with a visual workflow graph, retries and a manual or
automatic switch for the Python sidecar.

## Python

A Python sidecar runs in the same home folder. It hosts Optuna and
neural-network training, spaCy, and the user's scripts in fenced
workers with time, memory, folder and network limits. We may write
first-party Python for the sidecar; it is small, tested, pinned and
follows the same limits as other code. It is never in the live ranking
path and never builds a reply to a site.

Rust makes every live ranking decision. Python writes signals, tags,
scores and reports as plain data that Rust reads. Local language models
are also kept out of live ranking.

User customisation is Python (Rhai is not used). Alekh views, custom
buttons, extractors, hooks and link scorers are Python scripts. A script
or an AI agent may only create proposals on the review page; a person
accepts every site edit. A user may install any PyPI package with one
click into a script's own environment; the app shows its licence and
hash and applies the folder and network fences. The owner accepts the
supply-chain risk, which the risk register records.

## Models and memory

Models are exported to ONNX and run in Rust (tract or ort). The target
set stays the BGE stack of ADR-0020. The Model Hub inside the app works
like LM Studio: a curated shelf with hash-checked one-click downloads,
open search by name, ID or URL on Hugging Face, local file import,
hardware-fit badges, pause and resume, load, unload and delete, and
safe formats only (ONNX, safetensors, GGUF). An embedding swap keeps
each model's index separate and asks before rebuilding. The shelf holds
permissive licences only; open search shows a licence badge and needs an
extra confirm for non-commercial or unclear licences. The installer is
small, with a separate offline starter bundle.

Version 1 runs ONNX embeddings, rerankers and any compatible GGUF local
language model. The memory budget is automatic. The app detects real
RAM and graphics memory at start-up and defaults to 6 GB on standard
machines, scaling up on larger machines and always leaving room for the
operating system. A slider offers Safe, Balanced and Custom. On tight
budgets at most one heavy model loads at a time. A "too big for this PC"
badge has a power-user override, with graphics offload and context
controls. Vulkan on Linux and Windows is the graphics route, with a CPU
fallback; CUDA and Metal are out. Every model load goes through the
scheduler.

## Ranking and release quality

The release floor is quality, not a count. The bars of AGENTS.md section
3 item 6 stay: nDCG@10 at least 95%, challenger Precision@6 at least 1.2
times BM25 on every eligible test forum, no regression against the
verified champion and automatic rollback. At least 360 distinct ranking
methods ship, as 30 families of at least 12 each, and a method counts
only with a different signal or mechanism from every other method. A
changed setting never counts.

The families are: exact and phrase matching; probabilistic text models;
semantic embeddings; cross-encoder reranking; link-graph authority; hub
and community detection; entity and knowledge graph; reader behaviour,
including query-to-page click satisfaction (long clicks against quick
bounces); freshness and time; page structure and link position; quality
and trust; spam resistance (Rakshak); learning to rank; diversity
re-ranking; calibration and uncertainty; rank fusion and ensembles;
intent and topic models; version and compatibility; passage and section
ranking; anchor text and link context; deduplication and canonical
selection; originality and information gain; query rewriting and
expansion; site-wide and section-level signals; page experience and
usability; transparent rule-based adjustments; author and source
provenance; multimodal and rich-media relevance; language, location and
device context; and crawl, discovery and index-tier signals.

Ranking diversity has 12 types: semantic, source, reader intent,
solution, subtopic, evidence, entity and version, information gain,
freshness and time, authority, audience level and format.

The neural network is a fine-tuned BGE-style cross-encoder with a small
learning-to-rank network over the methods' signals. It is trained per
site on that PC from moderator decisions and judged sets, exported to
ONNX and served by Rust. No site data leaves the PC. Optuna tunes it
under spec 0077's 2% trust region. Spec 0077's eight prediction heads
must be reconciled with this design.

Quality is judged on six synthetic test sites: three XenForo and three
WordPress, with made-up posts and no member data, spread over tech and
PC repair, personal finance and legal basics, health and fitness, and
hobbies. Three AI makers judge as AGENTS.md section 3 item 6 sets out,
including at every push. Agents may approve a release when every check,
review and bar passes at 100%. Waivers stay with the owner's signed key.

## Safety and guardrails

Guardrails ship on. Most can be switched off in the Engine Room view.
These never go off: a person accepts every site edit; signed messages
between the app and sites; secret masking in logs; hard memory and disk
caps; undo snapshots before changes.

MCP reaches everything a person can click except the never-changeable
class: tokens and keys, permission rules, the confirm step, audit and
history settings, and switches that send site text off the PC. Risky
changes need the owner's live confirm. MCP is local only. There is no
remote bridge and no live progress link; progress is shared by an
exported snapshot. Outside services are optional, use the user's own
accounts and are off by default. Alerts, tickets and reports may go to
Slack, Teams, email, Jira, Linear, Asana and local files for manual
upload to Slides; site edits go to XenForo and WordPress only, through
the review page.

Raw visitor events are kept for a period the owner sets, 13 months by
default, then only aggregates. Counting is cookie-free by default and
erase-visitor and erase-all buttons exist.

## Interface

The Tauri app uses the Sutradhar Design System (SDS) as a themed layer
on Angular Material: our tokens, accessible patterns in the spirit of
GOV.UK and BBC GEL (no copied code or marks) and a palette inspired by
Kraken's. WCAG 2.2 AA is the floor. Version 1 ships in British English
with all text in translation files. Settings have three views: Basic,
Advanced and Engine Room.

## Delivery

At most ten agents run at once, counting every parent session and
helper across all agent products. This replaces the 56 of 12 November
2026 and keeps ADR-0020's ten. A commit changes at most 300 lines and
touches about 10 files, a push holds at most 5 commits, and the size
check refuses more unless the owner signs a waiver. Tool-made files such
as lock files and the SBOM do not count but must match their sources.
This restores the 17 November 2026 rule and replaces ADR-0020's "no
universal 300-line commit cap". Files stay at 300 lines, functions at 60
and lines at 80 characters. Each agent works one slice of about 150k
tokens, including its commit and push, and hands over with a record of
progress, decisions, changed files, evidence, blockers and next steps.

## Assurance

ADR-0020's strict core stays and gains the scheduler's fairness and
memory caps (item 11). Kani, Verus and Creusot prove the Rust core;
Lean 4 and Alloy check designs; Quint models the scheduler, the file
ledger and uninstaller, storage moves, the fenced Python sidecar and
hot-reload rollback, MCP permission locks and the human-review link
lifecycle. Quint runs fast and fail-closed at commit and push, on changed
models at commit and all models at push, and deeply at release on Linux
and Windows. SPARK, Haskell, OCaml, R, Julia and Fortran are not used.

## Names

QuarryView is Alekh; MachineMirror is Prayogshala; Factledger is Tathya;
King Solomon is Nyaya; King David is Smriti; Shraddar is Sanchay; Pixie
is Pagdandi; RankMedulla is Buddhi; SpamGuard is Rakshak; GanitEvolve is
Ganit Vikas; Sutradhar Boot is Behemoth Boot. Leviathan keeps its name
(owner, 19 November 2026). The first mapping offered Bhandar, Drishti
and Jaal, but those already name the storage module, the health module
and ganit's graph engine, so Sanchay and Alekh replace them. Old names
stay as "formerly" aliases for one release. Code identifiers such as the
`quarry` module and its settings keep their names until the module-rename
slice.

## Ownership

Analytics: darshak owns collection, labels, privacy, real-time views,
segments, cohorts, funnels, paths, attribution, anomalies and forecasts;
the connectors spec only imports data; hisaab owns reports and exports;
Alekh (formerly QuarryView) owns charts, tables and Python views; the
crawler is not analytics. Ranking: spec 0027 owns the pipeline, 0072 the
Buddhi and Rakshak models, 0053 feedback data, 0077 optimisation, spec 0055
admission, spec 0079 the 360-method list and spec 0056 the lab; spec 0050
leaves the family. Coach: spec 0041 owns the Coach and absorbs 0042 as its
Narada checks; spec 0052 keeps drafts and hubs. Each new feature goes to the
spec that owns its area. Two new specs are needed: the Jobs and workflow
screen, and Python scripting and the sidecar. The Model Hub is spec 0080;
spec 0024 keeps model admission, quarantine and integrity.

AI visibility uses supplied answer files (DR-086, spec 0093).
Spec 0042 keeps the archive and brand mention rate. Live answer checks,
visibility spending and automatic monitoring are after version 1.

Report files for Google Slides, Google Sheets and data warehouses are
created on the owner's PC for manual upload (DR-087, spec 0094).
Those destinations have no direct connection in version 1.

Specs 0095 to 0098 may use local checks, supplied files and drafts for
manual use (DR-088). Excluded services have no direct connection or
automatic posting, monitoring or sending in version 1.

## Explicit supersession map

| Earlier decision | Replaced or narrowed rule | Successor |
|---|---|---|
| ADR-0003 Dioxus desktop app and SDS in Dioxus | Tauri with Angular 22 and Material; SDS stays as the themed layer | Interface section |
| AGENTS.md 5.3 owner-only Ubuntu screen and local socket | Linux and Windows, socket or named pipe | Platforms section |
| ADR-0020 Kafka and Airflow background processing | PostgreSQL queue, outbox and our own scheduler | Data and processing section |
| ADR-0020 Python rule: no first-party Python | Small boxed first-party Python for the sidecar; never in ranking | Python section |
| ADR-0020 bounded shell anywhere | Shell for install, packaging and operations glue only | Languages section |
| ADR-0002, ADR-0005, spec 0019 on PostgreSQL jobs | Retained as the queue; the Kafka and Airflow migration is cancelled | Data section |
| AGENTS.md 5.6 and 11; ADR-0020 12 GB admission rule | Adaptive budget, 6 GB default, slider | Models and memory |
| AGENTS.md 5.7 signed .deb, systemd and Docker-free rules | One home folder, no systemd; .deb on Linux and an unsigned Windows installer | Platforms section |
| AGENTS.md 5.5 pure-Rust models only | ONNX and GGUF in Rust runtimes; Model Hub | Models section |
| AGENTS.md 5.5 outside-service wording; spec 0038 Elasticsearch | Optional, user-owned; no OpenSearch or Elasticsearch | Safety and data sections |
| AGENTS.md 3 items 1 to 5, ADR-0010, ADR-0013 counts; specs 0067 to 0071 and 0074 | No counts except 360 methods in 30 families; counting specs retire | Ranking section |
| AGENTS.md 3 item 9 owner approves the release | Agents approve when all checks pass; waivers stay with the owner | Ranking section |
| Spec 0075 (indicator steering) and the matching chart-signal wording in spec 0043 | Removed (owner, 17 and 19 November 2026) | This ADR |
| Rhai user scripts (QuarryView answer) | Python scripts | Python section |
| Settings views Basic, Advanced, Expert (13 November) and Calm and Engine Room | Basic, Advanced, Engine Room | Interface section |
| ADR-0012 app under AGPL; AGENTS.md 5.7 and README licence lines | App and libraries Apache-2.0; add-ons stay GPL-2.0-or-later; permissive-only dependencies | Licences section |
| ADR-0002 and AGENTS.md 5.1 "Sutradhar Boot" | Behemoth Boot | Names section |
| "Any Linux machine" install | Ubuntu and Debian .deb only in version 1 | Platforms section |
| AGENTS.md 8.2 at most 56 agents (12 November 2026) | At most ten agents at once | Delivery section |
| ADR-0020 "no universal 300-line commit cap" and AGENTS.md 8.5 "Coherent changes" | 300-line commits, about 10 files, 5 commits per push | Delivery section; AGENTS.md 8.5 |

Any older statement covered by this map has no normative force.

## Open items and risks

- Verify before specs rely on them: Tauri 2 packaging, Angular 22,
  Tantivy, DuckDB, the vector index crate, tract and ort, the GGUF runtime with
  Vulkan, and official prebuilt binaries on Windows and Debian.
  Quint's `quint-connect` crate and agent skills are unverified.
- XenForo's licence appears to allow password-protected test installs;
  confirm against the live agreement. Also confirm XenForo's rules for
  add-on licences, and write the trademark policy.
- Any-PyPI installs, an unsigned Windows installer and three-maker
  judging at every push are accepted risks and costs.

## Consequences

Specs 0014, 0016, 0017, 0019, 0020, 0024, 0030, 0031, 0033, 0034 to
0044, 0050 to 0057, 0064, 0072 and 0077, the roadmap and AGENTS.md must
change to agree, and the Forgejo backlog is re-cut into slices. Until
they change, this ADR wins.

## How this decision is checked

- The documentation checker rejects Kafka, OpenSearch, Elasticsearch,
  Airflow, Dioxus, Rhai, the removed chart-signal terms, numerical quota
  language and
  first-party Go as target components.
- Spec 0076's traceability covers every decision in this ADR.
- Memory, scheduler, Quint and ranking fixtures are named in their owning
  specs. Dynamic status stays in Forgejo, not in these documents.
