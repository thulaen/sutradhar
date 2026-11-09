# 0010: Coding harness and checker

The coding harness is aegis, the gate that runs every check, and
nirikshak, the code analyser it uses. It judges what code actually does,
not what a tool name or a passing fixture claims. It is roadmap Phase 2
items 1 and 2. The gate lives in `gate/` and `crates/aegis`, and its rules
are in [AGENTS.md sections 6 and 7](../../AGENTS.md#7-the-gate).

## What it does

### Goals and bounds

1. A row in the catalogue, a tool's name or a passing fixture does not
   prove that a check works or that a product is qualified. Analysis a
   tool cannot do is recorded as unsupported, never as a pass.
2. Every allowed language that is switched on is covered. Every check
   runs in pinned containers. Heavy work on Dell goes through the
   heavy-job queue. Other projects' services, containers, storage and
   credentials are never used.
3. Every behaviour gets its failing test before it is built.

### Catalogue, results and trust

1. `gate/catalogue.toml` records, for every check: its ID, title, why it
   exists, the gate items it covers, its languages and tools, tier,
   threshold, owner, and its passing and failing fixtures.
2. A meta-test refuses a gate item with no check, a check with no gate
   item, and a switched-on language with no proven mapping. No check may
   only warn: every check blocks.
3. Results use versioned contracts. Each names the candidate, its
   parent, source inputs, tools, rules, image, environment, findings,
   evidence and whether the result is complete.
4. Reports are parsed with size bounds and strict shapes. A zero exit
   code alone never stands in for a complete result. A check that is
   missing, cancelled, skipped, stale or timed out blocks.
5. Caches are signed by the gate only (with the tijori signing core) and
   reused only when every input identity is the same: dependencies,
   fixtures, environment, secret-input pins and advisory data where
   relevant. A command, result or cache supplied by an agent is never
   trusted. The release tier always runs fresh.
6. Reports come in plain English, JSON, SARIF, generated HTML and
   Prometheus form, with steps to reproduce and links to the autoissues
   tasks.
7. The `aegis` command offers: run a tier, check the catalogue, verify a
   result, show where a run stands, and reproduce a finding.
8. The git hooks only hand over the staged content. The door picks and
   checks the exact pushed commit itself. The gate creates no worktrees
   or branches.

### What each tier runs

- Commit: every check on the changed files and everything that depends
  on them, plus secrets, forbidden files and the commit message form. A
  local pass never stands in for the door's pass.
- Push: every check on the whole project, including:
  - all unit, property, snapshot, contract, integration and edge-case
    tests; each new test run twenty times; tests with no assertion
    refused;
  - full-project nirikshak, short mutation and fuzz runs on changed code;
  - Kani proofs, model checks of the flows a change touches, and checks
    that code built two ways (generated browser code and native code)
    gives the same results;
  - coverage, advisories, security, secrets in output, duplication,
    complexity, architecture, dead code, compatibility, row-level
    security, idempotency, benchmarks, query plans and budgets, memory
    smoke tests, environment shape, observability, docs, the manual,
    links, capabilities and handoff;
  - browser, accessibility and state tests for every changed screen.
- Release: every check fresh on the exact commit, plus full mutation,
  long fuzzing, soak, memory leaks, chaos, the browser set, a dynamic
  web security scan, container scans, planted-defect canaries, restore
  drill, upgrade and rollback rehearsal and the evaluation set. Every
  required test must pass.
- The required reviews of AGENTS.md section 7.1 run at commit and release and
  block like any check.

### Nirikshak analysis

1. Nirikshak reads facts from Rust, TypeScript, PHP, SQL, TOML, YAML,
   compose files and Markdown with pinned grammars, and from every other
   allowed language once switched on. Fact contracts are versioned.
   Facts are cached by content, grammar and extractor identity.
2. Facts gathered: functions, calls, imports, routes and their callers,
   typed request and response fields, setting and environment
   declarations and uses, metrics, logs, alerts, literals, sensitive
   flows, assertions, capabilities, glossary names, ownership, generated
   and vendored markers, and handoff references.
3. Nirikshak runs Datalog rules (simple logic rules over the facts) and
   gives locations and plain-English findings in SARIF. Style and types
   stay with each language's own tools.
4. The first 22 rules, in this order:
   1. The Rust validators of the platform always run. Only an
      independently written second safety checker may add a veto. The
      PHP add-ons check both current approvals and the result hash
      before saving to the site.
   2. API calls match the OpenAPI operations, fields and types.
   3. Arrow and JSON data contracts agree on both sides.
   4. Setting reads and declarations agree; no unused keys; every choice
      is mapped under spec 0014's no-hardcoding rule.
   5. Environment uses, docs and compose declarations agree.
   6. Jobs and endpoints have metrics, logs, alerts and runbooks;
      dashboards name real metrics.
   7. Sensitive content never reaches logs, errors, ganit or bundles.
   8. Outbound calls use approved clients with timeouts.
   9. No swallowed errors, discarded results or silent fallbacks.
   10. No hardcoded choices outside declared settings, including URLs,
       hosts, ports, limits and thresholds.
   11. No business logic repeated across languages, except the second
       safety checker of rule 1, whose verdict must agree with Rust.
   12. Database, ganit and module boundaries hold; no cycles.
   13. Capabilities link routes, commands, jobs and screens to the
       catalogue, admin paths, runbooks and support entries.
   14. Tests hold meaningful assertions that can fail.
   15. No hand edits to generated or vendored code.
   16. Text a person sees is translatable.
   17. Glossary names match their meaning and spelling.
   18. No unbounded queues or channels.
   19. No SQL built by joining strings.
   20. Docs name only commands, keys and symbols that exist.
   21. Every push leaves a clear handoff, and the commits and tasks it
       names exist.
   22. Exported functions and endpoints used across languages have real
       callers.
5. Security rules are added without repeating what clippy and the PHP
   taint checker already do. Each rule has a triggering and a clean
   fixture in every language it applies to.
6. An unknown fact or a parse failure never becomes "no finding" or a
   safe choice of what to check. Limits are recorded.
7. Before a rule may block, nirikshak itself needs 100% line and branch
   coverage, property tests and a measured false-alarm rate of at most
   2%. A missing measurement is not a rate of zero.
8. Site-platform facts are added to the same rules, keeping their order:
   platform and site, validator, site write, object capability, content
   version, pause, journal, handoff purpose, recipient, expiry, epoch and
   revocation, each with where it came from. An unknown hook or fact
   never proves a safe selection or that nothing is bypassed.
9. A plugin matrix in containers runs install, pair, suggest, approve,
   update and remove on every release, checking signed instructions,
   blocked responses, uncached tokens, no writes to other plugins'
   settings, refusals, and retries that add no page visits. Missing
   licensed inputs block. Full web-firewall proof needs real isolated
   evidence, not replayed fixtures.

### Tools and coverage

1. Rust uses the pinned test, coverage, fuzz, mutation, concurrency,
   compiler, lint, docs and dependency tools. TypeScript uses the pinned
   native strict compiler and a headless browser. PHP uses PHPUnit, its
   coverage, Infection, strict PHPStan, Psalm taint checks and Composer
   audit. Each version is pinned in `gate/tools.toml` before use.
2. Line or expression coverage is never labelled branch coverage or
   MC/DC. MC/DC is measured by our own tool (Phase 2 item 5).
3. A coverage tool is proven on fixtures: positive and negative decision
   cases, incomplete branches and conditions that must fail, panics and
   unreachable paths, and reported locations matched to the source.
   Short-circuiting, compiler lowering, optimisation and left-out
   regions are checked. A compiler accepting a flag proves nothing.
4. A substitute check keeps the original gate item ID, its inventory,
   generated and explicit cases, every planted fault and outcome, seed,
   scope and gaps. Property tests are not reported as coverage or fuzz
   tools.
5. A mapping to a tool does not prove the tool covers every construct,
   dialect or report field.

### Link safety rules and proofs

1. The Rust validator enforces the hard rules for forum posts: keep
   every word; wrap only existing words; skip protected BBCode areas;
   never nest links; keep BBCode balanced; the target must be visible to
   guests now; no duplicate targets and no link where one exists; lose
   no content. Adding link markup is not rewriting words.
2. Kani proves the real validator within stated bounds, backed by
   proptest, fuzzing and zero surviving mutants on the hard rules.
3. WordPress has its own HTML and block rules: a whole item is refused
   if any of its blocks is not qualified, and the editor must round-trip
   it. Proofs cover Rust code only, not the editor or other plugins.
4. stateright models cover: publishing, gating, forward and revert;
   queues with leases and retries; idempotent delivery; blue/green
   indexes and model switching; the 30-day recalculation; and module
   promotion. Each states safety and liveness properties, bounds and
   assumptions, and links its traces to real code. Models may change
   with the code; protected properties may not.
5. The protected checks also include agent rules in Rego, app
   permissions in Cedar, trust-model checks, the release-key guard and
   the architecture checker.

### Integrity and operations

1. Code being judged is kept apart from changes to its rules, Kani
   harnesses and thresholds. Old and new rules run side by side.
2. A waiver, including an inline suppression, is an owner-signed record
   in `gate/waivers/` with a reason and an expiry of at most 30 days.
   No test is turned off or set aside, and no hook is skipped.
3. Canaries plant one defect per gate-item family in a scratch copy. A
   missed canary raises an urgent task against the gate.
4. autoissues keeps its data in PostgreSQL with forced row-level
   security. Migrations only move forward: add, keep both working, then
   remove later.
5. A migration commit is never reverted by a tool. Another commit is
   reverted only after it is shown, repeatably, to cause the failure and
   the revert passes the gate. Otherwise publishing stops, evidence is
   kept and the owner is told.
6. GitHub CI runs the public source on made-up data with the same
   catalogue and pinned images. Private inputs and final performance
   runs stay on Dell. CI failures become autoissues tasks with stable
   fingerprints; retries are merged; full evidence is kept; outages and
   recovery are tested.
7. Urgent notices go to Forgejo as `status/urgent` tasks, shown first.
   Delivery cursors are durable. Failed notices are retried and a
   notifier's own failures are shown.

### Source identity

1. A runner target is one program the door runs. A tool it launches is
   not part of that target.
2. A source manifest lists every file needed to build the runner
   targets, sorted by exact path bytes. Each record holds the relative
   path, executable bit and SHA-256 of the content. The source identity
   is the manifest's digest plus a source kind and the target list. It
   is one input to acceptance, not a signature.
3. Building the manifest:
   1. Read the protected target list from the gate's policy: package,
      target, features and conditional compilation. Missing, repeated,
      unclear or unsupported entries block.
   2. Follow each target's first-party Cargo path dependencies, with
      their manifests.
   3. Follow modules, `#[path]`, `include!`, `include_str!` and
      `include_bytes!`. Dynamic inclusion, build scripts or unresolved
      conditional compilation block; never treat them as "no source".
   4. Add the workspace manifest, `Cargo.lock`, toolchain file,
      `gate/catalogue.toml` and every other asset the targets need.
   5. Leave out tests and fixture programs the targets do not need,
      keeping them in a separate test-input identity.
   6. Accept only ordinary files under the source root. Refuse links,
      special files, absolute or escaping paths, repeats, unreadable
      files and exceeded file, depth, byte or work limits.
   7. Encode with the one bounded canonical JSON and SHA-256 code.
   8. Record the source kind, targets, features, digest and limits.
   9. Check the exported files again just before running. Any missing,
      changed or extra required file blocks.
4. The target list, features, roots and bounds come from settings, never
   from the candidate. A new layout gets a new source kind. Old reports
   keep their kind and digest and lend no acceptance to a new one.

### Heavy-job control on Dell

These rules apply to every route that runs work in containers: the
container controller, builder jobs, the bootstrap compiler route and
dependency intake.

1. The controller is a small safe Rust program. It takes only check IDs
   and bounded data from the gate's policy, never a program, command or
   shell text. It runs pinned images with exact arguments.
2. Mounts are limited to read-only candidate exports and fresh evidence
   folders. Links, escapes, private inputs in public jobs, keys and
   other projects' storage are refused. No container gets the Docker or
   service-manager socket, signing material or private data.
3. Each job has a fresh full identity bound to the boot, account, unit
   and control group, source, policy, tools, images and limits. Stale,
   reused, aliased, malformed or neighbouring identities are refused.
   Nothing is ever found by a similar name.
4. A builder job gets its own rootless Docker engine. Its daemon,
   helpers and every worker sit under one job-owned control group;
   monitoring and recovery sit outside it. The only effects allowed are:
   create its transient units and engine; read its state; write the
   fixed freeze or thaw value to its own `cgroup.freeze`; kill its group
   through `cgroup.kill`; and confirm termination before the lease
   closes. No parent or sibling group can be touched.
5. No persistent unit, restart policy, account, network, firewall,
   kernel or security setting is changed. Missing authority blocks and
   needs a separate owner step.
6. The first run uses an isolated network namespace with no network
   driver, port, registry credential or download, and loads only a
   hash-checked archive of an admitted image. Image builds stay blocked
   until whole-job control is proven, and then need a pinned recipe,
   full dependency closure and no host network, secret forwarding or
   candidate overrides.
7. Pausing is proven only when the kernel confirms it and a counter job,
   watched from outside, stops advancing and stops using CPU. A frozen
   log file proves nothing. Resuming needs fresh queue checks and seen
   progress. Clock changes cannot fake a pass.
8. Caller loss, cancel, deadline, long pressure or supervisor failure
   kills every descendant. Recovery runs outside the frozen group and
   never starts a replacement engine. The lease is held until no live
   work is proven; a missing group or a failed signal is not proof.
9. A fresh container is used for each job, never `exec` into an old one.
   Its work waits behind a one-use start barrier until an outside
   watchdog is ready and the job is written to the job journal. A
   cancelled job's barrier can never be released by a replay, a late
   start or a new channel. Missing pressure readings block; they are
   never read as zero.
10. No automatic retry creates another job. Every source, policy, image,
    log and failed attempt is kept. Running out of quota or disk blocks
    instead of dropping evidence. Nothing is pruned.
11. Results from these routes are never gate passes, owner waivers or
    signed caches.

### Dependency intake

1. A download runs as one bounded intake job in an isolated container:
   non-root, read-only root, no capabilities, no host network, no
   inbound ports, no sockets, no checkout, keys or Cargo settings.
2. It may reach only the official Cargo index and archive, with checked
   TLS, no credentials and no host proxy. No downloaded build script,
   macro, test or program runs. Compiling and testing happen offline
   later.
3. The first intake was Tokio 1.54.1 with default features off and the
   `net` and `rt` features on, and its resolved closure. No other version
   is implied.
4. Licences, provenance, registry checksums, upstream identity and
   advisories are checked before the source is compiled.
5. The intake worker starts in a waiting state. It may not resolve
   names, open connections or start fetch helpers until the supervisor
   releases it once, over a fresh per-job Unix socket in protected
   storage. Both sides prove each other's identity from kernel
   credentials or a proven exclusive listener; a path match alone never
   passes.
6. The release frame is a four-byte unsigned big-endian length and one
   UTF-8 JSON object with exactly `version`, `job`, `container`, `image`,
   `worker`, `policy`, `network`, `clock` and `deadline`. The reply has
   exactly `version`, `job`, `worker` and `result` (`accepted`).
   Unknown, missing or repeated fields, unknown versions, oversized
   lengths (refused before allocation), partial frames, trailing bytes
   and extra frames are refused. No secrets are sent.
7. The consumed release is written to the job journal and synced before
   sending. An unclear write counts as consumed. A failed send, lost
   reply or lost connection ends the job; it is never resent.
8. Every helper-launch route needs the same job authority, which cannot
   be copied or forged and dies with the channel or the deadline. No
   reconnect, retry or new session resets the job's time or budget.
9. Before the first start, only a never-started container may show an
   empty network attachment ID; once running, the exact full network ID
   is required. Extra networks or published ports are refused.

### Host set-up and storage on Dell

1. Every Sutradhar payload stays on the project SSD: sources, settings
   contents, tools, images, temporary files, dependencies, output, logs,
   diagnostics, evidence and kept originals.
2. Linux account, service, login and audit records, and time-limited
   administrator set-up state, may stay on the OS drive under the host's
   own retention. Crash memory is payload, not a control record. A zero
   core-size limit does not stop a piped crash handler.
3. Bind mounts may show SSD content at the usual paths; a mount is not
   proof that writes go to the SSD. Open files keep their old targets,
   so a writer hand-over must lose nothing. Without the SSD, Sutradhar
   does not start.
4. A read-only host inventory runs for at most 90 seconds with at most
   1 MiB of output, no network and no reading of logs or settings
   contents. Unknowns fail it.
5. Other projects on Dell, such as Signal Controller, and shared host
   records are never inspected, moved, paused or changed. Shared
   retention is never changed.
6. A filesystem trial uses only a new bounded image file on the SSD and
   made-up files. It proves that a content seal is refused while a
   writer or shared writable mapping exists, allowed once it closes,
   and then blocks writes and truncation while keeping bytes and inode
   across a rename. Teardown is never forced.

## Limits

- Nirikshak: at most 30 seconds on changed files and 5 minutes on the
  whole project.
- A tier taking more than 30 minutes adds a speed-up task to "Later"; it
  is never cut short.
- Heavy jobs follow AGENTS.md section 11: at most 2 at once, each held to
  4 GB of memory and 8 CPU threads, with no swap; a benchmark runs alone.
  Nested workers count once within their parent.
- Per-route caps, all with zero swap:

| Route | Memory | CPU | Tasks | Time |
|---|---|---|---|---|
| Builder engine and workers | 768 MiB | 0.5 | 256 | declared per job |
| Builder outside supervisor | 256 MiB | 0.5 | 64 | |
| Builder transport helpers | 128 MiB | 0.25 | 64 | |
| Builder outer caller | 64 MiB | 0.25 | 32 | |
| Counter job (nested) | 64 MiB | 0.25 | 32 | 90 s |
| Bootstrap compile or check | 2 GiB | 1 | 128 | 10 min |
| Dependency intake, whole job | 256 MiB | 0.25 | 32 | 120 s |
| Filesystem trial work | 256 MiB | 0.25 | 32 | 90 s |
| Filesystem trial stop | 32 MiB | 0.25 | 16 | |

- Intake bytes: one archive at most 16 MiB, one index reply at most
  4 MiB, 64 MiB received in all, at most 20 packages.
- Filesystem trial: at most 1 MiB output a trial, 32 trials and 4 GiB of
  new kept storage a campaign, with a five-second stop reserve.
- The bootstrap compiler route runs one job at a time and ends at the
  earlier of the native queue being admitted or 24 hours after its first
  trial, bound to the boot and a monotonic clock.
- The gate runs on Dell; the memory limits in AGENTS.md section 5.6
  apply to the product on the owner's PC, not to the gate.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Tests live in `crates/aegis/tests/` unless named otherwise.

- `catalogue.rs`: the meta-test refuses gate items with no check, checks
  with no item and languages with no mapping; a warn-only check fails.
- `results.rs`: forged, stale and incomplete results, altered cache
  inputs, oversized and malformed reports, a zero exit with no result,
  and missing, cancelled, skipped and timed-out checks all block.
- `planted.rs`: every check refuses its planted fault in every allowed
  language switched on, and passes clean code. Also: missing tools,
  malformed facts, weak coverage, surviving mutants, leaks and loosened
  policy are refused.
- `nirikshak/`: each of the 22 rules and each security rule has a
  triggering and a clean fixture per language; unknown facts and parse
  failures never yield "no finding"; the false-alarm rate is measured.
  Site-platform cases: wrong validators, unknown blocks, cross-site
  handoffs, stale approvals, later edits and missing journals.
- `qualify.rs`: parent and fix replay, each new test run twenty times,
  seeded order runs, all old crash inputs, planted surviving mutants,
  malformed reports, missing tools and timeouts.
- `coverage.rs`: decision fixtures prove a coverage tool reports
  incomplete branches and conditions as failures at the right places.
- `source_identity.rs`:
  1. two packages with a path dependency give the hand-worked ordered
     manifest and digest, both manifests included;
  2. `#[path]` modules and all three include macros appear once, and
     any changed byte changes the digest;
  3. an unrelated test changes only the test-input identity;
  4. a fixture program is left out unless named as a target, then
     included with its support files;
  5. changed workspace manifest, lock file, toolchain, catalogue,
     features or targets change the right identity;
  6. dynamic includes, unknown targets, build scripts, unclear
     conditional compilation, cycles and missing Cargo metadata refuse
     with no partial digest;
  7. links, special files, escapes, repeats, non-UTF-8 paths and
     exceeded bounds refuse without reading outside the root;
  8. a report of another source kind fails acceptance;
  9. a file changed between export and run revokes acceptance, while an
     unrelated extra file stays visible but does not change the runner
     identity;
  10. old and new reports keep their own identities.
- `heavy/builder.rs`, run on Dell through the queue (BC-01 to BC-15):
  bindings agree; forged, stale, reused, aliased and neighbouring
  identities refuse; the new engine cannot control shared or foreign
  engines; every daemon, helper and worker stays in the owned subtree;
  real limits and totals meet the caps; the counter advances, stops on
  freeze and resumes; cancel and pressure kill every descendant; caller,
  supervisor or daemon loss leaks no work and releases no fence; changed
  kernel controls, permissions or limits block; forks and an escape
  fixture cannot evade refusal; private files, keys, sockets and other
  jobs stay safe; missing isolation, tools, image or logs block;
  deadline, clock, outage and failed recovery keep evidence; old and new
  resource kinds stay explicit; only then an offline image build.
- `heavy/compiler.rs` (CB-01 to CB-12): pinned inputs agree; swapped
  inputs and overrides refuse; caps hold; source is read-only and every
  write is on the SSD; the counter freezes with its descendants and
  resumes; a frozen logger cannot hide progress; caller, watchdog or
  daemon loss stops the exact work; expiry and failed pressure reads
  refuse; late starts cannot release cancelled work; partial cleanup
  keeps the fence across restart; old containers and other engines stay
  safe; storage failure keeps failed attempts and blocks.
- `heavy/intake.rs`: wrong versions, endpoints, redirects, credentials,
  users, limits and oversized or partial downloads refuse; the job ends
  at its deadline keeping all it received. Release frames: wrong or
  replaced peers, two peers, replay, malformed, partial, oversized and
  valid fragmented frames, expiry, suspend and clock mismatch, lost
  replies, two supervisors and restart. Every helper-launch route
  refuses before release and after expiry; no second fetch follows an
  unclear send; failure at every phase and around the journal write
  keeps the fence and evidence. Real traffic records show no worker
  network activity before release, told apart from Docker's own set-up.
- `heavy/controller.rs`: fault injection on freeze, resume, inspect,
  stop and kill; forged identities, mutable images, path escapes and
  other containers refused; the lock held until work is proven stopped.
- `host/fs_trial.rs` (on Dell, through the queue): the ten trial cases,
  from input checks and seal refusal with a writer or mapping, to
  rename ordering, failed writes, stopping after caller loss and a
  normal unmount.
- Canary runs plant one defect per gate-item family; each must be
  caught.
- Kani harnesses prove the link validator; stateright models check the
  listed flows; mutation testing leaves no survivor on the hard rules.

## Changes from the source

- The harness, nirikshak and every helper are rewritten as Rust, built
  with Cargo. The bootstrap route's temporary supervisor is a small
  shell script, which AGENTS.md section 4 allows.
- Languages not allowed by AGENTS.md section 4 are dropped from the
  checked set, with their tools, models and native solver checks.
- The second safety checker is written independently in an allowed
  language.
- Old formal models become stateright model checks and Rust trust-model
  checks.
- The tiers are the three of AGENTS.md section 7; the old nightly
  campaign moves to the release tier, and its pause after ten commits
  is dropped.
- The old AI review becomes the required reviews of AGENTS.md section 7.1.
- The old 45-minute push budget is replaced by the 30-minute speed-task
  rule.
- The fixed count of 47 checks is replaced by `gate/catalogue.toml` and
  the gate item list.
- The old `aegis.toml` registry is `gate/catalogue.toml`; the old crate
  names become `crates/aegis` and `crates/aegis-door`.
- Waivers and inline suppressions need the owner's signed record in
  `gate/waivers/`.
- The old 20x speed goal is replaced by the 5% median rule of AGENTS.md
  section 6.8.
- The handoff state file is replaced by Forgejo task proofs.
- Old Dell job limits (one job, 3.5 GB, or three jobs in 12 GB) are
  replaced by AGENTS.md section 11.
- The phone notice service is replaced by `status/urgent` tasks.
- The old commit freeze, baseline exception and status reports at task
  start are dropped as process notes.
- Sanitiser checks of native libraries are dropped: there is no unsafe
  or native code.

Source: `docs/specifications/coding-harness/README.md`,
`coding-harness/bootstrap-storage.md`,
`coding-harness/builder-control.md`,
`coding-harness/compiler-bootstrap.md`,
`coding-harness/dependency-intake.md`,
`coding-harness/deterministic-references.md`,
`coding-harness/docker-controller.md`,
`coding-harness/host-filesystem-trial.md`,
`coding-harness/intake-release-transport.md`,
`coding-harness/intake-startup.md`,
`coding-harness/source-identity.md` and
`coding-harness/ssd-host-baseline.md` in the archive
`archive-2026-11-08-old-repo`.
