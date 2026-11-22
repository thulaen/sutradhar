# 0114: Behemoth Boot verification

This is group PB-J of Behemoth Boot (spec 0081). It owns how we prove that
Behemoth Boot does what its behaviour descriptions say and stays safe: the
parity contract suite, formal models in Quint, the fuzzing farm, the build
graph slicing that keeps checking fast, the head-to-head benchmark, the rule
against unsafe code, the chaos lab, the compatibility matrix against real
servers, and the generated plain-English guide exposed over MCP. Spec 0081
owns the ledger, clean-room rules and release block; specs 0106 to 0113 own
the behaviour being checked; spec 0003 owns the check runner and toolchain;
spec 0007 owns the performance gates; spec 0083 owns the line counts and says
test code never counts toward a target.

Two features of the pillar list are shared with Leviathan and are owned by
spec 0116. Its compatibility matrix (N-1725) and reproducible
source-to-binary proof (N-1726 and LS-30) apply to Behemoth Boot.
BJ-21 uses the exact admitted targets and reproducibility results.
The 3B exit needs those results for the unsigned Boot outputs and
the applicable BJ-30 long lane on isolated framework fixtures.
Later Leviathan or product acceptance and Phase 7 release signing
are separate. A named profile or fixture gives no observed result.

Checks are hard-blocking at the three tiers of the gate (AGENTS.md section 7
and spec 0083): commit, push and release. Commit and push run every check;
Bazel's cache means a check whose inputs are unchanged is not run again, but
its result is valid and the check is never skipped. A missing tool fails the
check. The tier time rule of spec 0007 and AGENTS.md applies (a tier over its
budget raises a speed task and is never cut short). The very long checks (full
fuzz budgets, the soak, the full matrix and the benchmark wall) form a
separate long lane that is part of the release tier; the owner decided on 20
November 2026 (DR-069) that it is exempt from the 30-minute rule. It runs for
every release candidate, and an agent may approve a release only after it
passes. Nothing here passes by a count alone: completion of a framework group
is judged by the parity ledger, the contract tests and the proofs (spec 0083).
The pillar list's "16 million lines" is not a requirement: the line target is
the one of ADR-0022 and spec 0083, and tests are counted separately.

Example: a pull request changes the transaction manager. At commit the runner
runs every check; Bazel re-executes only the targets whose inputs changed, and
the unsafe-code audit and the `data_test` slice run in a minute or two. At
push, every check runs on both operating systems, including the whole parity
suite and a short fuzz run of every changed target. At release, the long lane
adds the Quint model checking, the compatibility matrix against real
servers, the benchmark wall, the full fuzz budgets and the soak. One
benchmark is 8 percent slower than its stored baseline; the release is
blocked until it is fixed, the baseline is changed by a reviewed change
that explains why, or the owner signs a waiver naming it.

Values marked "proposed" are design choices to be checked by measurement.

## What it does

### Words used here

- **Contract test**: a test written from a behaviour description (spec 0081)
  that states what an observer sees.
- **Parity suite**: all contract tests of all ledger items.
- **Oracle**: the source of an expected value. For a contract test it is the
  behaviour description, and sometimes an attributed figure from official
  documentation. No Spring program is run (DR-070).
- **Model**: a small formal description of a state machine, checked by a tool.
- **Fuzzing**: feeding a program many random or mangled inputs to find crashes.
- **Corpus**: the saved set of interesting fuzz inputs.
- **Baseline**: the stored result a new measurement is compared with.
- **Chaos**: deliberately injected faults such as a dropped connection.
- **Soak**: a long run to find slow leaks.
- **Hermetic**: reading only declared inputs, so a build is repeatable.
- **Sanitiser**: a tool that makes a program report memory errors as it runs.
- **Minimised input**: the smallest input that still causes a failure.
- **Invariant test**: a test of a rule that must always hold.
- **Starter, slice**: defined in specs 0081, 0106 and 0113.
- **Long lane**: the slow checks of the release tier (see above).

### What this spec designs

| Feature | What the project gets |
|---|---|
| N-1851 Executable Spring Boot behavioural parity suite | One independent contract test per ledger item, run on every change that touches it |
| N-1804 Quint formal state-machine verification | Models of start-up order, shutdown drain, transaction commit and rollback, filter ordering and batch restart, checked by Quint |
| N-1852 24/7 protocol and parser fuzzing farm | Continuous fuzzing of every parser, with saved corpora and automatic issue filing |
| N-1805 Granular Bazel build graph slicing | Small hermetic Bazel targets and caching, so a change rebuilds and retests only what it affects |
| N-1806 Automated head-to-head benchmark wall | Cold start, memory, tail latency and throughput against our own same-machine baseline. Official Spring figures appear only as attributed document evidence (DR-070); no Spring program is run |
| N-1807 Zero-unsafe core audit gate | No unsafe code in core crates, and audited wrappers for the few unavoidable calls |
| N-1808 Fault-injection and chaos test lab | Automated runs that break connections, packets, certificates and memory and check recovery |
| N-1809 Automated compatibility matrix across external servers | Drivers and clients tested against live versions of real servers |
| N-1810 Self-updating plain-English encyclopaedia over MCP | Generated guides, setting dictionary, migration notes and runnable recipes served read-only over local MCP |

### Parity contract suite

1. Every ledger row (spec 0081) has at least one contract test, written from its
   behaviour description by someone other than the implementer. The test is the
   oracle for the row moving to `matched`.
2. Tests state the observable behaviour only: routes, binding, headers, status
   codes, transactions, errors, settings and defaults. They never inspect
   internal structure.
3. Oracles come only from the behaviour description and the official
   documentation examples. No Spring program is ever run, by anyone, for any
   purpose (owner decision of 20 November 2026, DR-070). This keeps the clean-
   room claim of spec 0081 simple: the only inputs are official documentation,
   release notes and reference guides.
4. Counts are not a goal. The pillar list's "hundreds of thousands" of tests
   is an aim; the requirement is one independent test per row and per
   documented edge case, and the suite's size is reported as a number, never
   as a gate.
5. A test that cannot fail (no assertion, or no mutant can fail it) is refused
   (spec 0083, AP-07).
6. Tier use: commit and push run the whole suite (cached results count when
   inputs are unchanged); release runs it on both systems.
7. A failing test after an upstream upgrade drops its row from `verified` to
   `in_progress` (spec 0081, BJ-22).
8. A flaky test (one that fails and then passes with no change) counts as
   failing. It is fixed or removed within seven days; a rerun never turns a
   functional test into a pass. Only benchmarks are rerun for noise (below).

### Formal models in Quint

1. Quint models (ADR-0021: Quint, with its pinned Node and Java runtime, is a
   development-only tool) describe: dependency start-up order (spec 0106),
   graceful shutdown draining (0106), transaction commit and rollback (0108),
   security filter ordering (0109) and batch job restart safety (0111). Models
   also cover the outbox, lock and fencing protocol (0110) and the state-
   machine transition rules (0110), with the property "a transition applies
   only from the stored state, and a transition with a failing guard does not
   apply".
2. What Quint gives is a check, not a proof of the Rust code. Its simulator
   explores random runs and is used at commit and push; its model checker
   (which needs the pinned Java runtime) explores all runs within stated
   bounds and is used in the long lane. Documents say "model-checked within
   bounds" and name the bounds. Models are abstractions: they leave out data
   races, input and output, and timing, and each model says so.
3. Link between model and code: every property has an id `QP-nnn`, written in
   the model and in the name of the Rust test (an invariant test or a property
   test) that checks the same rule. A gate check fails if an id exists on one
   side only (SUT-1403).
4. Models and results are files in the repository. A change to behaviour that
   has a model requires the model to change in the same commit, and the check
   to pass.
5. Rust-level proofs of small core pieces use Kani, Verus or Creusot where
   they fit (ADR-0021, development only). Each runs on Linux only, with its
   toolchain version pinned in `gate/tools.toml`; the list per module says
   what is proved and its limits. A proof that cannot run on a system is
   listed as "not run there".

### Fuzzing farm

1. Every code path that reads untrusted bytes has a fuzz target: HTTP/1,
   HTTP/2 and HTTP/3 framing, TLS records, JSON, YAML, XML, CBOR, SQL wire
   protocols, JWT, SAML, LDAP, Kafka, AMQP and Protobuf (the groups of specs
   0107 to 0111). Most of these are parsers in mature crates. We do not fuzz
   those crates in isolation; we fuzz our own wrappers and glue and the whole
   path from bytes to our public interface, which exercises the dependency as
   it is used.
2. The fuzz engine is not yet chosen (QR-042). Candidates are LibAFL,
   honggfuzz-rs and a `cargo-fuzz` style engine. libFuzzer is C++ and is
   excluded unless it is a pinned prebuilt binary. Sanitiser support is
   limited on Windows, so a sanitiser finding may be Linux only, and the farm
   says which system found it.
3. Each target has a seed corpus of valid examples, a dictionary of tokens,
   and a saved corpus of found inputs. Corpora live in a corpus area outside
   the source tree, with an integrity hash per file, a size cap
   (`verify.corpus_max_gib`) and pruning by coverage. A scan checks that no
   corpus input contains a secret.
4. Tier budgets: commit and push fuzz only the targets whose inputs changed
   (the rest are cached), each for at most `verify.fuzz_push_min` minutes,
   inside a total budget per tier: `verify.fuzz_commit_total_s` and
   `verify.fuzz_push_total_s`. The push total stays inside the 5-minute push
   suite; a target not reached is reported as not fuzzed this run, never as
   passed. The long lane runs every target for `verify.fuzz_release_min`
   minutes. Between releases the farm runs on
   machines the owner approves. "Billions of inputs" is an aim; the
   requirement is the budgets above and zero open findings at release.
5. A finding (crash, hang, out of memory, sanitiser report) files an issue
   (spec 0063) with the minimised input and its crash signature. Issues are
   de-duplicated by signature and rate limited (`verify.fuzz_issues_per_day`)
   so a farm cannot flood the queue. The input becomes a permanent regression
   test. A release with an unfixed finding is blocked. Agents triage in
   slices within the AGENTS.md limit on concurrent agents.
6. In memory-safe Rust the findings are mostly panics, hangs and limit failures
   (an expectation, not a measured fact); those count.
7. Machine rules for the farm: the owner approves each machine; it runs inside
   the heavy-job slot and the memory and disk caps of spec 0078, with a CPU
   and heat limit; it stops when the machine is busy. GitHub runners have
   about 14 GB of disk (RK-029), so their corpus cap is lower than an owner
   machine's (`verify.corpus_max_gib_runner`). It uses no outside service and
   sends no data off the machine. The farm's own tools are pinned by hash in
   `gate/tools.toml`.

### Build graph slicing

1. The code is organised into small, hermetic Bazel targets, one per module
   and per starter, with declared dependencies only (spec 0003, AGENTS.md
   section 5 module walls). A sandbox check runs every target with only its
   declared inputs visible; this must work on Linux and on Windows, where it
   depends on the rules_rust work of ADR-0021 (RK-023). An experiment for it
   is added to the research dossier (EX-08).
2. Caching is on for local and CI runs. A cache entry is valid only for
   identical inputs and tool versions. Only CI may write the shared cache,
   entries are hashed, and agents' local runs read it but cannot write to it,
   so a poisoned entry cannot be created locally. A test builds with and
   without the cache and requires identical results (BJ-10).
3. A change triggers only the affected targets and their dependents to run
   again; the check reports which targets it ran, and which it took from the
   cache and why that is valid.
4. A target over its size limit fails (SUT-1406); a target with an undeclared
   dependency fails (SUT-1414). The file and function limits are those of
   AGENTS.md.
5. Build time per target is recorded, and a target that grows slow is
   reported. Proposed aim: one starter's change rebuilds and tests in under
   one minute with a warm cache on the Linux reference machine; Windows is
   measured and may differ.

### Benchmark wall

1. A fixed set of benchmark applications exists for each group: a web route, a
   data query, a security chain, a message relay, a batch job, a template
   render. They run on Behemoth Boot only. No Spring program is run (DR-070),
   so there is no head-to-head comparison against Spring; the pillar list's
   "head-to-head" wall compares each release with our own stored baselines and
   with the numbers that official documentation and release notes state, where
   they state any.
2. Definitions: cold start is the time from process start to the first
   successful answer of the readiness check (spec 0112). Memory is resident
   memory at ready. Tail latency is the 95th and 99th percentile under an
   open-loop load (a fixed arrival rate, corrected for coordinated omission)
   from a pinned load tool. Throughput is the highest rate with the 99th
   percentile under a stated limit. Warm-up runs are discarded for steady-
   state numbers and cold start is reported separately.
3. Each result is the median of `verify.bench_runs` runs with a confidence
   interval, on the same machine with the same inputs, and records machine and
   tool versions (spec 0007). A result whose interval is too wide is "noisy":
   it is rerun up to three times as spec 0007 says, and if it stays noisy it
   is blocked (not a pass).
4. The comparison is against our own stored baseline in `gate/bench-
   baselines.toml`, protected like the speed targets of spec 0007. A
   regression beyond `verify.bench_regress_pct` blocks a release. Baselines
   change only through a reviewed change that explains the reason.
5. Results stay private. Nothing is published in a public repository or
   elsewhere until the owner decides and the legal review concludes (DR-071).
6. A benchmark on one machine is never compared with a run on another.

### Unsafe-code audit

1. The core crates are listed in `gate/unsafe-core.toml` (parsers, routing,
   serialisation, security and the container, by rule: any crate that reads
   untrusted bytes or makes a security decision). Each carries
   `forbid(unsafe_code)`, and a gate check fails if the attribute is missing or
   weakened to `allow`.
2. The few unavoidable low-level calls (operating-system calls, for example
   for permissions and peer credentials) live in separate wrapper crates. Each
   wrapper is small, documented, tested, and either proved with Kani, Verus or
   Creusot or reviewed independently under the review rules of AGENTS.md, with
   the record kept.
3. An unsafe-usage scanner, pinned in `gate/tools.toml`, lists every unsafe
   block in first-party code and in dependencies, and also lists build scripts
   and procedural macros, which run on the build machine. A new entry fails
   the gate at commit and push until it has a review record (SUT-1410); it is
   not left to the release.
4. Dependencies with unsafe code are allowed because we cannot rewrite them.
   Their counts are part of the supply-chain record: spec 0113's bill of
   materials carries an `unsafe_count` field per component (QR-046).
   Resolve each count from the pinned scanner's observed result for the
   exact resolved component revision in the admitted scan profile. A
   result for a different revision or scope cannot supply the count,
   even when it has the same value. Reuse an earlier result only with
   exact matching inputs, scanner pin, scope and current trust.

### Chaos lab

1. Faults are injected in process through test fakes and through a small
   first-party network proxy that sits between test client and server on
   loopback, with no Docker and no kernel driver (a driver would break
   pinning). It drops packets, cuts connections mid-byte, delays, and returns
   error storms. Other faults: killed worker processes, full disks and
   exhausted memory.
2. Memory limits use Job Objects on Windows and control groups on Linux, with an
   allocator cap as the fallback where permission is missing. A full-disk test
   uses a small size-limited temporary volume or an allocator-level stand-in,
   never the real system disk of an owner machine.
3. Expired certificates are generated already expired, and a clock hook is used
   where a TLS library accepts one. Paths that read the real clock and cannot be
   hooked are listed as not covered.
4. Each fault has a named scenario with the expected recovery: no lost committed
   data, no duplicate effect beyond what at-least-once allows, no crash, bounded
   memory and a clear error. Each scenario has a time limit and a resource cap
   (`verify.chaos_timeout_s`), which may not exceed one hour (proposed),
   the per-scenario ceiling of the long lane (DR-069).
5. Scenarios are deterministic given a seed; a failing seed is stored so it
   can be replayed.
6. Commit and push run a short set of scenarios and the long lane runs the
   full set, plus a soak of `verify.soak_hours` for any release (the condition
   is not a judgement: the soak runs for every release candidate).
7. The lab proves nothing it did not inject; its report lists what was and was
   not covered.

### Compatibility matrix

1. Drivers and clients are tested against live servers: PostgreSQL (18 first,
   then the other supported versions), MySQL, MariaDB, Redis-compatible
   stores, Kafka, RabbitMQ, MongoDB and OpenTelemetry collectors (the families
   of specs 0108 to 0112). The versions are listed in `gate/verify.toml`
   (`verify.matrix_versions`).
2. Servers run as plain processes from pinned prebuilt binaries, with no Docker
   (ADR-0021). Binaries are pinned by hash in `gate/tools.toml`, scanned by the
   gate's supply-chain checks, and their licences are checked before use. Some
   servers need their own runtime (Kafka needs a Java runtime, RabbitMQ needs
   Erlang); those runtimes are development-only tools pinned the same way.
3. Only open-source servers are in the matrix (owner decision of 20 November
   2026, DR-071): Valkey stands in for Redis, and MongoDB is not used until a
   legal review of its licence (SSPL) concludes. The MongoDB cell is listed as
   blocked, not faked. Licences are checked per server before it enters the
   matrix.
4. The matrix runs on the two operating systems where the server exists for
   both; a server with no build for one system is tested on the other only.
   Each cell is pass, fail or not available, with the version tested. A cell
   that is not available blocks a release unless the matrix exceptions file
   lists it with a reason that the owner has approved. A family with no usable
   driver is listed as blocked (specs 0108 and 0110).
5. Size and memory: push runs PostgreSQL 18 only. The long lane runs the full
   matrix; its memory and disk use are measured on the GitHub runners (14 GB
   disk) and the 8 GiB reference before the matrix is fixed.

### Plain-English encyclopaedia over MCP

1. At each release, the pipeline generates from the verified code and the
   ledger: a guide for each starter, the settings dictionary (from the
   catalogue of spec 0113), migration notes (from the doctor), and runnable
   recipes.
2. Every recipe is run as a test; a recipe that does not run is not published.
3. The result is exposed read-only over local MCP (spec 0020) as resources of
   Behemoth Boot's catalogue, so a beginner or an agent can look things up. No
   write action, no secret and no private data is included. Text is British
   English and follows the plain-English rules of AGENTS.md.
4. "Self-updating" means regenerated by the release pipeline. Nothing is
   rewritten by an agent at run time.
5. The existing manual pages and action catalogue of spec 0020 stay the
   Sutradhar resources; this adds the framework's.

### Data

Verification keeps files, not database tables. It adds no table to the
product database.

- In the repository: the Quint models and their results, the contract tests,
  `gate/tools.toml` (pinned tools), `gate/verify.toml` (settings and matrix
  versions), `gate/bench-baselines.toml` (stored baselines),
  `gate/unsafe-core.toml` (core crates), the matrix exceptions file and the
  signed waivers in `gate/waivers/`.
- Outside the source tree: the fuzz corpora, with an integrity hash per file
  and a size cap, and the failing seeds stored for replay.
- Reports: the check results, the chaos lab's covered and not-covered list,
  and the generated plain-English guide.

### Pipeline settings

These live in the gate's own configuration (`gate/verify.toml`), not in the
app's settings. They are registered with the gate's other configuration (spec
0003, QR-046).

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `verify.fuzz_push_min` | minutes per target | 5 (proposed) | 1 to 5 (proposed) | next run |
| `verify.fuzz_commit_total_s` | seconds per commit | 60 (proposed) | 10 to 300 | next run |
| `verify.fuzz_push_total_s` | seconds per push | 120 (proposed) | 10 to 300 | next run |
| `verify.fuzz_release_min` | minutes per target | 60 | 5 to 1,440 | next run |
| `verify.fuzz_issues_per_day` | whole number | 20 | 1 to 1,000 | next run |
| `verify.corpus_max_gib` | GiB (owner machine) | 20 | 1 to 500 | next run |
| `verify.corpus_max_gib_runner` | GiB (CI runner) | 5 | 1 to 10 | next run |
| `verify.bench_runs` | whole number | 30 | 10 to 1,000 | next run |
| `verify.bench_warmup_runs` | whole number | 5 | 0 to 100 | next run |
| `verify.bench_regress_pct` | percent | 5 | 1 to 50 | next run |
| `verify.soak_hours` | hours | 72 | 1 to 720 | next run |
| `verify.soak_mem_mib` | MiB cap | 2,048 | 256 to 65,536 | next run |
| `verify.chaos_seeds` | whole number per scenario | 50 | 1 to 10,000 | next run |
| `verify.chaos_timeout_s` | seconds per scenario | 300 | 10 to 3,600 (proposed) | next run |
| `verify.matrix_versions` | list per server | see the file | supported versions | next run |
| `verify.farm_cpu_pct` | percent of CPU | 50 | 5 to 100 | next run |
| `verify.farm_machines` | list | none | machines the owner approved | next run |

There is no setting that skips a check at its tier. Only the owner's signed
waiver (AGENTS.md section 7.2, kept in `gate/waivers/`) passes a failure; it
names the check and expires within 30 days.

### Failure codes

SUT-1400 contract test missing for a ledger row; SUT-1401 contract test cannot
fail; SUT-1402 model check failed; SUT-1403 model without a linked test;
SUT-1404 fuzz finding open; SUT-1405 corpus over its cap; SUT-1406 target over
its size limits; SUT-1407 benchmark regression; SUT-1408 noisy benchmark;
SUT-1409 unsafe block in a core crate; SUT-1410 unsafe entry without a review
record; SUT-1411 chaos scenario failed; SUT-1412 compatibility cell failed;
SUT-1413 recipe does not run; SUT-1414 undeclared dependency in a build
target; SUT-1415 flaky test; SUT-1416 waiver missing, expired, over-renewed or
with an invalid time; SUT-1417 farm machine not approved; SUT-1418 unsafe-core
list missing a crate; SUT-1419 secret found in a corpus or recipe. Codes
SUT-1400 to SUT-1499 are reserved for this spec.

### Security and privacy

- Fuzz corpora, observations of Spring, benchmark data and recipes hold no
  secret or personal data.
- The farm and the chaos lab send nothing off the machine.
- No Spring program is run. The clean-room record (spec 0081) shows that the
  only inputs are official documentation, release notes and reference guides.

### Verification scope and read-only evidence (N-1804 to N-1810)

QR-216 qualifies each result for its exact verification target and
profile. Scope the receipts to the tools, pins, targets and fixtures
used. Required check tiers keep their designated tests and evidence.

| Feature | Owning behaviour and evidence |
|---|---|
| N-1804 | BJ-04, BJ-05 and BJ-27. Bind each pinned Quint proof to its QP model, property, bounds and repository test. Record a counterexample or execution failure against that exact check. Runtime tests retain their separate evidence. [FX-0114-W5-QUINT-EXACT-PROPERTY-AND-TEST-BINDING](../planning/feature-wave-5.md#acceptance-design-for-n-1791-to-n-1814). |
| N-1805 | BJ-09, BJ-10 and BJ-26. Use declared Bazel dependencies and their reverse dependencies to select work. Reuse only compatible cache entries with exact pins. Local clients read and authorised CI writes. Retain EX-08 and QR-099 for host and cutover evidence. [FX-0114-W5-SLICED-BUILD-EXACT-CACHE-PROFILE](../planning/feature-wave-5.md#acceptance-design-for-n-1791-to-n-1814). |
| N-1806 | BJ-11, BJ-12 and BJ-32. Compare private Behemoth measurements against its own same-machine baseline with identical profiles and pinned settings. Apply the written noise rule. Keep any official Spring figures as attributed document evidence under DR-070. The existing engineering speed gates keep their own scope. [FX-0114-W5-PRIVATE-SAME-MACHINE-BENCHMARK](../planning/feature-wave-5.md#acceptance-design-for-n-1791-to-n-1814). |
| N-1807 | BJ-13 and BJ-28. Enforce `forbid(unsafe_code)` for core code and report the declared dependency and macro audit separately with the `unsafe_count` receipt. The existing benchmark exception applies only where AGENTS.md gives it. [FX-0114-W5-CORE-UNSAFE-INVENTORY-AND-AUDIT](../planning/feature-wave-5.md#acceptance-design-for-n-1791-to-n-1814). |
| N-1808 | BJ-14, BJ-15, BJ-30 and BJ-31. Use sealed synthetic fault fixtures with bounded seeds and durations. Inspect recovery and preserved committed state with no live credentials or routes. Resolve uncertain outcomes according to the existing job and effect rules before another attempt. [FX-0114-W5-SEALED-SYNTHETIC-FAULT-RECOVERY](../planning/feature-wave-5.md#acceptance-design-for-n-1791-to-n-1814). |
| N-1809 | BJ-16, BJ-17, BJ-21 and BJ-32. Run the admitted pinned server fixtures as local processes. Record each exact matrix cell. Keep the PostgreSQL 18 push lane and the complete required release lane. Use Valkey in the admitted cache profile and hold MongoDB without the required decision. A test profile grants no product connection. [FX-0114-W5-PINNED-LOCAL-SERVER-MATRIX-LANES](../planning/feature-wave-5.md#acceptance-design-for-n-1791-to-n-1814). |
| N-1810 | BJ-18 and BJ-19. Generate only through the existing release workflow from exact verified candidate, released schema and permitted sources. Pin tested recipe and admitted resource. MCP reads only checked versions. A stale copy cannot supplant the current release or author a replacement. Verify declared recipes in fixtures. The scoped local MCP reader shows the bound approved text and provenance. It cannot edit or run a recipe. [FX-0114-W5-RELEASE-BOUND-READ-ONLY-RECIPES](../planning/feature-wave-5.md#acceptance-design-for-n-1791-to-n-1814). |

#### Independent parity and parser proof (N-1851–N-1852)

| Owner catalogue entry | Exact test and authority contract |
| --- | --- |
| N-1851 Executable Spring Boot behavioural parity suite | Run independently authored Behemoth contract tests tied to pinned 0081 descriptions, exact candidate inputs and official documentation. DR-070 forbids running Spring. Preserve per-edge coverage, cached evidence identity, flaky failure and the existing commit, push and release gates. Neither test inventory nor pass report grants phase, release or deployment authority. [FX-0114-W5-INDEPENDENT-PINNED-PARITY-PROOF](../planning/feature-wave-5.md#acceptance-design-for-n-1839-to-n-1862) |
| N-1852 24/7 protocol and parser fuzzing farm | Use the existing admitted sealed machines, local synthetic corpus, parser boundaries and measured tier limits. Guard machine load, logs and corpus privacy. Reproduce and deduplicate safe local findings in the approved tracking path, with the existing open-finding gate. The scheduling label gives no always-on entitlement, external target, autonomous vulnerability workflow or exploit reproduction. [FX-0114-W5-SEALED-BOUNDED-PARSER-PROOF](../planning/feature-wave-5.md#acceptance-design-for-n-1839-to-n-1862) |

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| BJ-01 | Every ledger row MUST have an independent contract test before it can be `matched` (SUT-1400). | UT-BJ-01, AU-BJ-01 |
| BJ-02 | A contract test that cannot fail MUST be refused (SUT-1401). | PL-BJ-01 |
| BJ-03 | Expected values MUST come only from behaviour descriptions and official documentation, and no Spring program MUST be run. | AU-BJ-02 |
| BJ-04 | Each Quint model MUST name its properties and bounds, and each property MUST have a linked test on the real code (SUT-1402, SUT-1403). | UT-BJ-02 |
| BJ-05 | A behaviour change with a model MUST change the model in the same commit. | UT-BJ-03 |
| BJ-06 | Every untrusted-byte parser MUST have a fuzz target with a seed corpus and a dictionary. | UT-BJ-04 |
| BJ-07 | A release MUST be blocked by any open fuzz finding (SUT-1404), and each finding MUST become a regression test. | UT-BJ-05, IT-BJ-01 |
| BJ-08 | The corpus MUST stay under its cap (SUT-1405). | UT-BJ-06 |
| BJ-09 | A target over its limits (SUT-1406) or with an undeclared dependency (SUT-1414) MUST fail, and the sandbox check MUST run on both systems. | UT-BJ-07, PL-BJ-02 |
| BJ-10 | A cache hit MUST be valid only for identical inputs and tool versions. | UT-BJ-08 |
| BJ-11 | Benchmarks MUST run on Behemoth Boot only, on the same machine with the same inputs, and report machine and tool versions. | UT-BJ-09 |
| BJ-12 | A regression over `verify.bench_regress_pct` against the baseline MUST block a release (SUT-1407). A noisy result MUST be rerun up to three times and, if still noisy, MUST be blocked and not count as a pass (SUT-1408). | UT-BJ-10 |
| BJ-13 | A core crate MUST forbid unsafe code (SUT-1409), and every unsafe entry MUST have a review record (SUT-1410). | UT-BJ-11, PL-BJ-03 |
| BJ-14 | Chaos scenarios MUST be deterministic by seed, replayable, and MUST list what they did not cover. | UT-BJ-12 |
| BJ-15 | A chaos scenario MUST end with no lost committed data, bounded memory and a clear error (SUT-1411). | FI-BJ-01 |
| BJ-16 | A compatibility cell MUST show the tested version and result, and an unavailable cell MUST be shown, not hidden (SUT-1412). | UT-BJ-13 |
| BJ-17 | Servers in the matrix MUST be pinned prebuilt binaries run without Docker. | UT-BJ-14 |
| BJ-18 | Every published recipe MUST run as a test (SUT-1413). | UT-BJ-15 |
| BJ-19 | The encyclopaedia MUST be read-only, include no secret, and be generated by the release pipeline. | UT-BJ-16, PL-BJ-04 |
| BJ-20 | A check whose tool is missing MUST fail, and no tier MUST skip a check. | UT-BJ-17 |
| BJ-21 | Behemoth Boot targets MUST be included in the matrix and the reproducible-build verifier of spec 0116. | AU-BJ-03 |
| BJ-22 | A row whose test fails after an upstream upgrade MUST drop from `verified` to `in_progress`. | UT-BJ-18 |
| BJ-23 | A flaky test MUST count as failing (SUT-1415) and be fixed or removed within seven days. | UT-BJ-19 |
| BJ-24 | Fuzz issues MUST be de-duplicated by crash signature and rate limited, and a corpus or recipe holding a secret MUST be refused (SUT-1419). | UT-BJ-20, PL-BJ-04 |
| BJ-25 | A farm machine MUST be owner-approved (SUT-1417) and run within the caps of spec 0078 and `verify.farm_cpu_pct`. | UT-BJ-21 |
| BJ-26 | The shared build cache MUST be written only by CI and an entry MUST be hashed; local runs MUST be read-only. | UT-BJ-22 |
| BJ-27 | Every Quint property id `QP-nnn` MUST appear in both the model and a Rust test (SUT-1403). | UT-BJ-02 |
| BJ-28 | Every core crate MUST be listed in the unsafe-core file and forbid unsafe code, and a new unsafe entry MUST fail at commit and push without a review record (SUT-1409, SUT-1410, SUT-1418). | UT-BJ-11, PL-BJ-03 |
| BJ-29 | A waiver MUST be signed by the owner, name its check and expire within 30 days (SUT-1416). The gate counts waivers per check id, whatever their scope, base revision or candidate. The first waiver and one renewal are allowed. A further waiver for the same check id MUST carry a new owner signature naming the reason, and the gate shows how many waivers that check has had. Removing, renaming, splitting or merging a check id in `gate/catalogue.toml` already needs the owner's signature (AGENTS.md 7.2, a change that removes a check); that signature names the new ids and the count carried to them, and the gate refuses a waiver for an id with no signed history. Time: the gate reads time only from the pinned signed time of the door or CI, never the agent host. It uses the later of the two times for expiry and the earlier for the future-signing check, and refuses when the two differ by more than 5 minutes. It refuses a waiver whose expiry is over 30 days after signing, whose signing time is in the future, or when the signed time goes backwards. | UT-BJ-23 (clock rollback; signing time in the future; 30 days plus one second; the two times differ by more than 5 minutes; a second renewal refused; a waiver for the same check id with a narrower, wider or different scope counts the same; a rename, split or merge with no owner signature refused; a waiver for an id with no signed history refused) |
| BJ-30 | The soak MUST run for every release candidate and the long lane MUST pass before an agent approves a release. | IT-BJ-02 |
| BJ-31 | The pipeline settings MUST be enforced as stated, including `verify.fuzz_push_min`, `verify.fuzz_release_min`, `verify.soak_hours` and `verify.chaos_seeds`. | UT-BJ-24 |
| BJ-32 | Benchmark results MUST stay private and MUST NOT be published until the owner decides after legal review, and the matrix MUST contain only open-source servers. | UT-BJ-25 |

## How it is tested

Designed, not run.

- **Unit (UT-BJ):** the audit and gate checks on small fixture crates, with
  one honest and one failing case each; cache validity; corpus size; benchmark
  arithmetic with one value just inside and one just outside the regression
  limit.
- **Planted faults (PL-BJ):** a test with no assertion; a target with an
  undeclared dependency; an unsafe block in a core crate; a recipe that leaks
  a secret; each must be refused with its named message while an honest case
  passes.
- **Integration (IT-BJ):** a fuzz finding becomes an issue and then a regression
  test; one matrix run against PostgreSQL 18 on both systems.
- **Fault injection (FI-BJ):** the chaos scenarios themselves, run against the
  real starters.
- **Audit (AU-BJ):** a sample of ledger rows traced from description to test to
  observation, with the clean-room record; the shared items of spec 0116.
- **Measurement (MS-BJ-01):** time of commit, push and the long lane on the 8
  GiB reference and on the GitHub runners, fuzz throughput per target, and
  build time of one starter change with a warm cache. The targets are set
  after the first run; until then the only target is the proposed one minute
  for one starter.

## Changes from the source

The owner decided on 20 November 2026 that the long lane is exempt from the
30-minute rule (DR-069), that no Spring program is ever run (DR-070), and that
benchmark numbers stay private and the matrix uses open-source servers only
(DR-071). The pillar list's head-to-head benchmark against Spring is therefore
replaced by comparison with our own baselines. The pillar list said Quint
would "prove" the state machines mathematically. Quint checks models within
bounds, so documents say "model-checked". It also asked for billions of fuzz
inputs, hundreds of thousands of tests and 16 million lines; those are
reported measures, not requirements, and the line target stays with spec
0083. "Self-updating" means regenerated at release. The matrix and the build
      verifier shared with Leviathan are owned by spec 0116. Docker is not
      used; servers run as pinned prebuilt processes (ADR-0021).
