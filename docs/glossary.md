# Glossary

Plain meanings of the names and terms used in Sutradhar.

## Module names

| Name | Meaning | What it is |
|---|---|---|
| Sutradhar | Hindi: "the one who holds the strings" | The whole product |
| Behemoth Boot | English name | Our Rust application framework |
| SDS | Sutradhar Design System | Our screen component library |
| shodhan | Hindi: "refining" | The text engine that reads posts and finds links |
| ganit | Hindi: "mathematics" | Shared maths and statistics |
| Ganit Vikas | ganit "maths" and Hindi vikas "growth"; formerly GanitEvolve | The research and optimisation engine; its speed search writes a receipt for each source file (spec 0012) |
| Nyaya | Hindi: "justice"; formerly King Solomon | The research cards, evaluator checks and proof standard in spec 0013 |
| Smriti | Hindi: "memory"; formerly King David | The research memory and concept map in spec 0013, not a second store |
| mandali | Hindi: "assembly" | The module registry and lifecycle manager |
| sanyojan | Hindi: "coordination" | The composition root that wires the modules together |
| vinyas | Hindi: "configuration" | Typed settings, profiles, validation and change history |
| sanchar | Hindi: "communication" | Typed, versioned events through the PostgreSQL outbox under spec 0019; not the XenForo site change feed; `sutra sanchar` keeps its name for compatibility |
| karya | Hindi: "work" | Durable jobs on our own Axum and Tokio PostgreSQL scheduler, with recovery and shared progress, under spec 0019; `sutra karya` keeps its name for compatibility |
| bhandar | Hindi: "store" | Search, object storage, archives and safe exports |
| abhilekh | Hindi: "record" | Audit events, private details and erasure; replaces the earlier name lekha, which older specs still use |
| drishti | Hindi: "view" | Health, metrics, diagnostics and visual data |
| dwarpal | Hindi: "gatekeeper" | Authorisation and, separately, the signed-message guard; tijori holds the keys |
| sampark | Hindi: "contact" | The signed site client and guest-visible readers |
| pariksha | Hindi: "test" | The production evaluation library in spec 0056; not the shared fake toolkit |
| test-support | English name | Shared test fakes in a development-only crate, separate from pariksha |
| agvani | Hindi: "reception, welcome" | The example module and extension used by the developer guides; replaces the earlier name namaste, which older specs still use |
| sujhav | Hindi: "suggestion" | Moderator review, approved link insertion and undo |
| kavach | Hindi: "armour" | The module that supplies spam evidence through `SpamSignal` |
| Buddhi | Hindi: "intellect"; formerly RankMedulla | The ranking model owned by `shodhan.rank`, not a second ranking path |
| Rakshak | Hindi: "protector"; formerly SpamGuard | kavach's spam model, used as a ranking signal (spec 0027) |
| Tathya | Hindi: "fact"; formerly Factledger | The contradiction tip in spec 0042; no list of facts is stored |
| Sanchay | Hindi: "collection"; formerly Shraddar and Shraddha | The research memory store in spec 0013 |
| Pagdandi | Hindi: "footpath"; formerly Pixie | A graph-walk method candidate the lab may admit (specs 0049 and 0056) |
| NetworkX | Outside library name | A public reference for graph operations; no code is copied |
| sanket | Hindi: "signal" | SEO signals, the shared crawler and health reports |
| Leviathan | Crawler name, kept by the owner (19 November 2026) | The shared crawler, its own browser stack and block-page handling in specs 0044 and 0045 |
| Prayogshala | Hindi: "laboratory"; formerly MachineMirror | The `mirror` reader test room in spec 0039 |
| Subramanian | Link-study module name | The `subramanian` reports in spec 0046; it does no crawling |
| Alekh | Hindi: "graph, plot"; formerly QuarryView | The `quarry` charts, comparisons, alerts and filters in spec 0043 |
| Narada | Editorial check module name | The passage checks used by the Editorial Coach in spec 0042 |
| graph | English module name | Stores the knowledge graph and applies ganit's graph results |
| work_graph | Internal module name | The site's Fix list, with claims, rechecks and recurrence |
| shikhar | Hindi: "peak" | Votes, usefulness formulas, sorting and flags |
| pratishtha | Hindi: "reputation" | Member reputation, overall and per forum; replaces the earlier name karma, which older specs still use |
| panchayat | Hindi: "village council" | Moderation hints and their review queue |
| uttar | Hindi: "answer" | Optional expert routing |
| content | English module name | Hubs, summaries and content drafts |
| tuning | English module name | Bounded feedback learning, activation and rollback |
| Chayan | Ganit engine name | Selection and optimisation, such as review order and anchor variety |
| Mapan | Ganit engine name | Measurement and calibration, such as uncertainty and relevance thresholds |
| Shiksha | Ganit engine name | Learning, such as supervised models and feature importance |
| Anuman | Ganit engine name | Inference, such as priors and exploration estimates |
| Samanjan | Ganit engine name | Parameter adjustment, such as retrieval tuning |
| Mulyankan | Ganit engine name | Evaluation statistics and comparisons |
| Vikalp | Ganit engine name | Alternative outcomes, such as offline policy evaluation |
| Bahav | Ganit engine name | Distribution drift and recalibration signals |
| Gati | Ganit engine name | Dynamics, such as attention decay and freshness |
| Jaal | Ganit engine name | Graph analysis: graph algorithms and graph-based signals |
| Varnakram | Ganit engine name | Spectral methods and linear algebra, such as graph embeddings and embedding whitening |
| Sannidhya | Ganit engine name | Embedding geometry, such as near-duplicate clusters |
| Padbandh | Ganit engine name | Phrase statistics, such as generic-anchor detection |
| Satya | Ganit engine name | Label truth and agreement between judges, moderators and checks |
| Parivartan | Ganit engine name | Changepoints: when a series of numbers changes its level or trend |
| Vichalan | Ganit engine name | Anomalies: values far from what is expected |
| Bhavishya | Ganit engine name | Forecasts with ranges |
| Anukaran | Ganit engine name | Replay and simulation of past or projected work |
| Niyantran | Ganit engine name | Feedback controllers that keep a value near its target |
| Kshamata | Ganit engine name | Cost, memory and capacity models |
| Chitran | Ganit engine name | Graph layouts for pictures of the link map |
| Karan | Ganit engine name | Causal effects: whether a change caused a result |
| Ayu | Ganit engine name | Survival analysis: how long something lasts |
| darshak | Hindi: "viewer" | Visitor statistics that resist bots |
| tijori | Hindi: "strongbox" | Keys, signing and the release key guard |
| sutradhar-tijori | sutradhar + tijori | tijori's check library: the signed-record layout and signature checks, with no private key |
| hisaab | Hindi: "accounts" | Books and reports |
| setu | Hindi: "bridge" | The link between the service and the site add-ons |
| sutra | Hindi: "thread" | The command-line tool |
| chaukidar | Hindi: "watchman" | Spots any change to what guards the project, and raises the alarm |
| pankti | Hindi: "a line" | The heavy-job queue on Dell: at most two heavy jobs at once |
| aegis | Greek: "shield" | The gate: every check |
| aegis-door | aegis + English "door" | The gate on Forgejo's side: checks every push before it lands |
| nirikshak | Hindi: "inspector" | The code analyser used by the gate |
| prahari | Hindi: "guard" | The outside tester that checks the gate itself |
| autoissues | English name | Keeps the list of faults found, apart from Forgejo tasks; a fault is linked to a task only when an agent takes it on |
| forge-sync | English name | Copies Forgejo to GitHub and brings GitHub issues back |

## Terms

| Term | Meaning |
|---|---|
| `.deb` | An install package for Ubuntu or Debian |
| `unsafe` | Rust code that turns off its safety checks |
| Alert fatigue | So many alerts that people stop noticing them |
| Angular | The web framework used to build the desktop app's screens, inside Tauri |
| API | The set routes one program offers for others to call |
| ARIA | Labels that tell screen readers what a part of a page is |
| Artefact | A file a build produces, such as a `.deb` or a zip; in a capability workflow, see "Artefact kind" |
| Artefact kind | In a capability workflow, the kind of thing an action reads or writes, such as a crawl or a draft (spec 0070) |
| At-least-once delivery | Each event is sent until it is confirmed, so it may arrive twice |
| Axum | A Rust library for serving web requests |
| Backlink | A link from another site to a page on this site (specs 0045, 0152 and 0153) |
| Baseline | A saved measurement that later runs are compared with |
| BBC GEL | The BBC's public design guide for clear, accessible screens |
| Benchmark | A timed run used to measure speed |
| Branch | A separate line of work in git; Sutradhar uses only `main` |
| Buffer overflow | Writing past the end of a block of memory |
| Cache invalidation | Throwing away saved copies once the real data changes |
| candle | A pure Rust library for running AI models |
| Capability | One thing the shipped product can do, counted with evidence for the release bar |
| Capability workflow | One choice of shipped action for each slot of a workflow shape, run in order and admitted with one identity when it behaves differently from every other admitted capability; it supports feature qualification, not a numerical release quota (spec 0070) |
| Checksum | A short fingerprint of a file that changes if the file changes |
| CI | Continuous integration: checks that run by themselves on every push |
| Circular import | Two modules that each depend on the other |
| Cloud helpers | Optional jobs the owner can run on their own cloud accounts |
| Code smell | A sign in code that something is badly designed |
| Command injection | A trick that makes a program run a command the attacker chose |
| Commit | One saved change to the project, with a message saying what and why |
| Composition root | The one place where all parts are wired together |
| Consumer | The code that receives and acts on an event |
| Container | A sealed box that runs a program with only what it needs |
| Contract test | A test that checks both sides of a link agree on the same messages |
| Conventional Commits | A fixed form for commit messages, such as `feat(scope): what changed` |
| CORS | Rules for which websites may call a web service from a browser |
| Coverage | How much of the code the tests run: every line, every path through each decision, and every condition |
| Crate | A Rust package: one library or program |
| Creusot | A Rust tool that proves code meets written rules |
| Cruft | Old, unused or messy leftovers in a project |
| CSRF | A trick that makes a signed-in user's browser send an unwanted request |
| DCO | Developer Certificate of Origin: a contributor's promise that they may give the code |
| Dead-letter list | Where a job goes after it has failed too many times, for a person to look at |
| Dell | The project's build computer, where Forgejo and the agents run |
| Dependency injection | Each part is handed what it needs, instead of fetching it itself |
| Design tokens | Named values for colours, sizes and spacing used by every screen |
| DiD | Difference-in-differences: the change in a group minus the change in a comparison group (spec 0149) |
| Digest | A fingerprint that names one exact container image |
| DO-178C | The standard for software in aircraft; Level A is the strictest |
| Donor page | An unchanged page used to build the comparison for a changed page (spec 0149) |
| Door gate | Forgejo's check that runs before it accepts a push, and refuses a push that fails |
| DRY | "Don't repeat yourself": each fact or piece of logic lives in one place |
| DST | Daylight saving time |
| Ed25519 | A modern, fast method for signing messages |
| Editor Coach | The owner's shorter name for the Editorial Coach, kept in quoted decisions and ADRs |
| Editorial Coach | The feature that suggests the smallest useful edit to a page |
| Embeddings | Numbers that stand for the meaning of a piece of text |
| End-to-end test | A test that runs a whole journey, from start to finish |
| Environment variable | A named setting passed to a program when it starts |
| Estimator | The calculation that turns before-and-after page data into a measured effect (specs 0148 and 0149) |
| Failure injection | Breaking things on purpose in tests to prove the system recovers |
| Footprint | Extra work an add-on does on a site's public pages |
| Forgejo | The private code and task website on Dell |
| Fork | A copy of a repository made by someone else on GitHub |
| Front-end bundle | The packed files a screen loads |
| Fuzzing | Feeding a program huge numbers of random inputs to find crashes |
| Gate | The set of checks every change must pass |
| GOV.UK Design System | The UK government's public guide to clear, accessible service screens |
| Graceful shutdown | Stopping a program so it finishes or safely saves its work first |
| Hash | A short fingerprint of a file; if one byte changes, the hash changes |
| Hook | A small program git runs by itself at a set moment, such as before a commit |
| HTTPS | The secure way programs talk over the web |
| Idempotency | Doing the same request twice has the same effect as once |
| Index | A database aid that makes finding rows fast |
| Integration test | A test that checks several parts working together |
| Interop module | A small piece of code that lets two languages talk to each other |
| ITT | Intention to treat: judging each page by its original assignment, whatever happened later (spec 0149) |
| JPL | NASA's Jet Propulsion Laboratory, which publishes strict coding rules |
| Kani | A Rust tool that checks every possible input for small pieces of code |
| Killed mutant | A changed copy of the code that the tests caught by failing |
| KISS | "Keep it simple": choose the simplest design that works |
| Label | A tag on a Forgejo task, such as `status/ready` |
| Lawbook | The site's own reviewable record of what its past experiments showed (spec 0151) |
| Lifecycle hooks | Code that runs at set moments, such as start-up and shutdown |
| Lock file | A file that records the exact version of every library used |
| Mailbox | The shared service where agents send messages and take file locks |
| MC/DC | Modified condition/decision coverage: tests show each condition in a decision really changes the result |
| MCP | Model Context Protocol: a standard way for AI tools to plug into a program |
| Method | One distinct, independently qualified way of doing a job, counted for the release bar |
| Migration | A numbered change to the database's layout |
| Milestone | A group of Forgejo tasks: one for each roadmap phase, plus "Later" |
| MISRA | Coding rules from the car industry for safe, predictable code |
| Model Hub | The screen in Sutradhar where a user finds, downloads, checks and removes AI models (spec 0080) |
| Modular monolith | One program made of walled-off modules that talk only through set routes |
| Monkey patch | Changing other code while it runs |
| Mutant | A slightly changed copy of the code, made by a mutation test |
| Mutation test | A tool changes the code slightly; a good test suite must then fail |
| N+1 query | A slow pattern that makes one database query per item instead of one for all |
| Observability | Being able to see what a running program is doing, through logs, metrics and traces |
| Outbox | A table where events are saved before they are sent, so none is lost |
| Pagination | Showing long lists one page at a time |
| Patch | A file that holds one change, so it can be applied later |
| Placebo check | A test on a fake change that should find nothing, to catch false findings (spec 0149) |
| Planted fault | A deliberate mistake used to prove a check catches it |
| Policy | A rule the database checks for each row, such as "only this site's rows" |
| Power of 10 | NASA's ten rules for safe code |
| Profile | A named set of settings: `dev`, `test` or `prod` |
| Property test | A test that checks a rule holds for many generated inputs |
| Push | Sending saved commits from one copy of the project to another, such as Forgejo |
| Python sidecar | The helper process that hosts Python scripts, Optuna and spaCy in fenced workers (ADR-0021, spec 0104) |
| Question tool | The agent's way of asking the owner a question with set answers to pick from |
| Race condition | A bug that depends on which of two things happens first |
| Readiness check | A check that says whether a part is ready to take work |
| Recipe | A capability workflow that counted in the qualified run, offered for people and agents to run by its `CW-` id (spec 0070) |
| Reflection | Code that inspects or changes itself while running |
| Replay | Sending a copy of an old message again to trick the receiver |
| RLS | Row-level security: the database itself hides rows a user may not see |
| Role | A database account with set permissions |
| Runner | The program that runs Forgejo's checks |
| SemVer | Version numbers like 1.2.3: major, minor and patch (here "patch" means a small fix) |
| Serde | A Rust library for reading and writing data formats |
| Service locator | A shared lookup where code fetches what it needs |
| Shared mutable state | Data that many parts can change |
| Signed run | A full run of the gate whose results are signed, so they cannot be changed later |
| Singleton | A single shared object the whole program uses |
| SLO | Service level objective: a promised measure, such as "answers within 1 second" |
| Soak test | Running the system under load for a long time to find slow leaks |
| Spaghetti code | Tangled code that is hard to follow |
| Spring Boot | A well-known framework whose ideas Behemoth Boot follows |
| SQL injection | A trick that sneaks commands into a database query |
| sqlx | A Rust library for talking to PostgreSQL |
| SSRF | A trick that makes a server fetch an address the attacker chose |
| stateright | A Rust library for checking designs by exploring every state |
| Stop check | A check that refuses to let an agent end its session until its task is updated |
| Subagent | A helper agent started by an agent for one job |
| Subagent review | A fresh helper agent that checks a change it did not write |
| sudo | Running a command with full system rights |
| superuser | A database account that can do anything |
| systemd | The Ubuntu part that starts and watches background services |
| Systemd slice | A group of programs that systemd limits together, such as one memory cap for all |
| Tag | A named point in git history, used for each release |
| Tailwind | A CSS tool that builds styles from short class names |
| Task starvation | Some jobs never getting a turn |
| Tauri | The toolkit that wraps the Angular screens in the desktop app's window (ADR-0021) |
| TDD | Test-driven development: write the failing test first |
| Telemetry | Measurements a program records about itself |
| Tier | When checks run: commit, push or release |
| Tokio | The Rust library that runs many tasks at once |
| Tower | A Rust library of reusable parts for web services |
| Tracing | The Rust library for structured logs |
| tract | A pure Rust library for running AI models |
| Transaction | A group of database changes that all happen, or none do |
| Trunk-based development | Everyone works on one branch, `main`, in small safe steps |
| Unix socket | A private connection between programs on the same computer; on Windows a named pipe does the same job |
| Variant | One distinct checked case, counted for the release bar |
| Vendored code | Outside code copied into the repository |
| Verus | A Rust tool for writing proofs that code is correct |
| Waiver | A signed owner record that loosens a check; only the owner may make one |
| Waiver count | How many waivers the gate has counted for one check id; one renewal is allowed (AGENTS.md 7.2, spec 0114 BJ-29) |
| WCAG | The web accessibility standard; AA is its middle level |
| Web view | The built-in browser part the desktop window uses to draw screens |
| Webhook | A web call one service makes to tell another that something happened |
| WebKitGTK | The Ubuntu library that provides the desktop window's web view |
| Wordfence | A common WordPress security plugin |
| Workflow | A list of steps CI runs; in the product, see "Capability workflow" |
| XSS | A trick that runs an attacker's script in another user's page |
