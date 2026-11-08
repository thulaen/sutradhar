# Glossary

Plain meanings of the names and terms used in Sutradhar.

## Module names

| Name | Meaning | What it is |
|---|---|---|
| Sutradhar | Hindi: "the one who holds the strings" | The whole product |
| Sutradhar Boot | English name | Our Rust application framework |
| SDS | Sutradhar Design System | Our screen component library |
| shodhan | Hindi: "refining" | The text engine that reads posts and finds links |
| ganit | Hindi: "mathematics" | Shared maths and statistics |
| GanitEvolve | ganit + English "evolve" | The speed search that writes a receipt for each source file |
| darshak | Hindi: "viewer" | Visitor statistics that resist bots |
| tijori | Hindi: "strongbox" | Keys, signing and the release key guard |
| hisaab | Hindi: "accounts" | Books and reports |
| setu | Hindi: "bridge" | The link between the service and the site add-ons |
| sutra | Hindi: "thread" | The command-line tool |
| chaukidar | Hindi: "watchman" | Spots any change to what guards the project, and raises the alarm |
| aegis | Greek: "shield" | The gate: every check |
| nirikshak | Hindi: "inspector" | The code analyser used by the gate |
| prahari | Hindi: "guard" | The outside tester that checks the gate itself |
| autoissues | English name | Turns findings into Forgejo tasks |
| forge-sync | English name | Copies Forgejo to GitHub and brings GitHub issues back |

## Terms

| Term | Meaning |
|---|---|
| Alert fatigue | So many alerts that people stop noticing them |
| API | The set routes one program offers for others to call |
| ARIA | Labels that tell screen readers what a part of a page is |
| Artefact | A file a build produces, such as a `.deb` or a zip |
| At-least-once delivery | Each event is sent until it is confirmed, so it may arrive twice |
| Axum | A Rust library for serving web requests |
| Baseline | A saved measurement that later runs are compared with |
| BBC GEL | The BBC's public design guide for clear, accessible screens |
| Benchmark | A timed run used to measure speed |
| Branch | A separate line of work in git; Sutradhar uses only `main` |
| Buffer overflow | Writing past the end of a block of memory |
| Cache invalidation | Throwing away saved copies once the real data changes |
| candle | A pure Rust library for running AI models |
| Capability | One thing the shipped product can do, counted with evidence for the release bar |
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
| Digest | A fingerprint that names one exact container image |
| Dioxus | The Rust library used to build the desktop app's screens |
| DO-178C | The standard for software in aircraft; Level A is the strictest |
| Door gate | Forgejo's check that runs before it accepts a push, and refuses a push that fails |
| DRY | "Don't repeat yourself": each fact or piece of logic lives in one place |
| DST | Daylight saving time |
| Ed25519 | A modern, fast method for signing messages |
| Editorial Coach | The feature that suggests the smallest useful edit to a page |
| Embeddings | Numbers that stand for the meaning of a piece of text |
| End-to-end test | A test that runs a whole journey, from start to finish |
| Environment variable | A named setting passed to a program when it starts |
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
| JPL | NASA's Jet Propulsion Laboratory, which publishes strict coding rules |
| Kani | A Rust tool that checks every possible input for small pieces of code |
| Killed mutant | A changed copy of the code that the tests caught by failing |
| KISS | "Keep it simple": choose the simplest design that works |
| Label | A tag on a Forgejo task, such as `status/ready` |
| Lifecycle hooks | Code that runs at set moments, such as start-up and shutdown |
| Lock file | A file that records the exact version of every library used |
| Mailbox | The shared service where agents send messages and take file locks |
| MC/DC | Modified condition/decision coverage: tests show each condition in a decision really changes the result |
| MCP | Model Context Protocol: a standard way for AI tools to plug into a program |
| Method | One distinct, independently qualified way of doing a job, counted for the release bar |
| Migration | A numbered change to the database's layout |
| Milestone | A group of Forgejo tasks: one for each roadmap phase, plus "Later" |
| MISRA | Coding rules from the car industry for safe, predictable code |
| Modular monolith | One program made of walled-off modules that talk only through set routes |
| Monkey patch | Changing other code while it runs |
| Mutant | A slightly changed copy of the code, made by a mutation test |
| Mutation test | A tool changes the code slightly; a good test suite must then fail |
| N+1 query | A slow pattern that makes one database query per item instead of one for all |
| Observability | Being able to see what a running program is doing, through logs, metrics and traces |
| Outbox | A table where events are saved before they are sent, so none is lost |
| Pagination | Showing long lists one page at a time |
| Patch | A file that holds one change, so it can be applied later |
| Planted fault | A deliberate mistake used to prove a check catches it |
| Policy | A rule the database checks for each row, such as "only this site's rows" |
| Power of 10 | NASA's ten rules for safe code |
| Profile | A named set of settings: `dev`, `test` or `prod` |
| Property test | A test that checks a rule holds for many generated inputs |
| Push | Sending saved commits from one copy of the project to another, such as Forgejo |
| Question tool | The agent's way of asking the owner a question with set answers to pick from |
| Race condition | A bug that depends on which of two things happens first |
| Readiness check | A check that says whether a part is ready to take work |
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
| Spring Boot | A well-known framework whose ideas Sutradhar Boot follows |
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
| TDD | Test-driven development: write the failing test first |
| Telemetry | Measurements a program records about itself |
| Tier | When checks run: commit, push or release |
| Tokio | The Rust library that runs many tasks at once |
| Tower | A Rust library of reusable parts for web services |
| Tracing | The Rust library for structured logs |
| tract | A pure Rust library for running AI models |
| Transaction | A group of database changes that all happen, or none do |
| Trunk-based development | Everyone works on one branch, `main`, in small safe steps |
| Unix socket | A private connection between programs on the same computer |
| Variant | One distinct checked case, counted for the release bar |
| Vendored code | Outside code copied into the repository |
| Verus | A Rust tool for writing proofs that code is correct |
| Waiver | A signed owner record that loosens a check; only the owner may make one |
| WCAG | The web accessibility standard; AA is its middle level |
| Web view | The built-in browser part the desktop window uses to draw screens |
| Webhook | A web call one service makes to tell another that something happened |
| WebKitGTK | The Ubuntu library that provides the desktop window's web view |
| Wordfence | A common WordPress security plugin |
| Workflow | A list of steps CI runs |
| XSS | A trick that runs an attacker's script in another user's page |
| `.deb` | An Ubuntu install package |
| `unsafe` | Rust code that turns off its safety checks |
