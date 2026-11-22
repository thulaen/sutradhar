# 0106: Behemoth Boot core: container, configuration and lifecycle

This is group PB-B of Behemoth Boot (spec 0081). It owns the heart of the
framework: how parts are wired together, how conditions switch parts on and
off, how settings are loaded and checked, and how an application starts, runs
and stops. Spec 0081 owns the rules that keep the framework in step with
Spring Boot (the ledger, the scout, the clean-room procedure and the release
block); this spec says what the core behaves like. Every other group builds on
it. It adds nothing that Spring Boot does not have, and drops nothing that
Spring Boot has, except where the idiom map of spec 0081 says a Java idea has
no sensible Rust shape.

Spring facts in this spec come from the owner's pillar list (19 November
2026). The scout confirms each against official release notes and the
reference guide before its row leaves `scheduled` (spec 0081). Until then a
claim is "unverified" and the behaviour description is the contract, not the
Spring wording.

Claims taken from the owner's list stay "unverified" until the scout confirms
them; their ledger rows start in the `unverified` state and stay out of the
ledger counts (spec 0081). They are: relaxed binding of names; the order of
settings layers; profile groups and config trees; the properties migrator
switch; background initialisation; the ahead-of-time optimiser; the
null-safety baseline; and the refusal of circular needs by default. Related
records: DR-040 (Behemoth Boot mirrors Spring Boot) and RQ-015 (the upstream
version in force).

Example: Nia's app still has a setting called `server.port.old` that a newer
release renamed. She switches on the strict migration check. The next start
stops before anything opens, and prints: "2 settings are out of date.
`server.port.old` is now `server.port`. `cache.ttl` was removed; use
`cache.ttl_seconds`." She edits the file and the app starts. With the check
off, the app starts and the same two lines appear in the log as warnings.

Values marked "proposed" are design choices to be checked by measurement.

## What it does

### Words used here

- **Part**: one unit of the application with one purpose and a public API
  (AGENTS.md section 5), also called a component.
- **Composition root**: the one place where every part is created and joined.
- **Scope**: how long a part lives: singleton (one for the whole run),
  request (one per request), lazy (made on first use) or named (one of
  several implementations picked by name).
- **Starter**: one crate that wires one capability (spec 0081).
- **Condition**: a rule that switches a starter or part on or off.
- **Setting**: a typed, named value with a default.
- **Profile**: a named set of settings for one environment.
- **Config tree**: a folder where each file is one setting, such as a mounted
  secrets folder.
- **Readiness**: the state in which the app accepts work.
- **Conditions report**: the plain list of which parts are on or off and why.
- **Wiring graph**: the map of which part needs which.
- **Start hook**: code a part runs once at start, before the app is ready.
- **Safe point**: a place where work can be stopped without leaving it half
  done, such as between two items.
- **Serialisation plan**: the precomputed steps for turning data into bytes.

### What this spec designs

| Feature | What the user or program sees |
|---|---|
| N-1880 Clean-room dependency container | Parts declared with their needs; the container builds them in the right order with the right lifetime |
| N-1736 Modular auto-configuration architecture | One small crate per capability; an app compiles only what it wires |
| N-1737 Rich conditional activation engine | Starters switch on or off from the presence of a type, a setting value, a missing custom part or the operating system |
| N-1738 Strict circular-reference blocker | A loop of needs is refused at start with a plain diagram |
| N-1739 Layered configuration and relaxed binding | Settings load from layers in a fixed order and match names written in several styles |
| N-1740 Strong profile and config-tree system | Built-in profiles, admitted groups and an approved read-only configuration tree |
| N-1735 Strict startup migration blocker | A switch that refuses to start while out-of-date setting names remain |
| N-1741 Background component initialisation | Independent parts start in parallel. Readiness waits for all required parts. |
| N-1742 Ahead-of-time compile-time optimiser | Wiring graphs and tables are worked out when the program is built |
| N-1743 Non-null safety contracts by default | A missing value is a compile-time or start-time error, never a surprise later |
| N-1744 Unified lifecycle and graceful shutdown engine | Ordered start events, readiness, and a clean drain on shutdown |

### The container

1. Parts are declared in Rust with their needs as typed arguments. The
   composition root lists every part explicitly. There is no run-time
   discovery of parts, no crate-wide auto-collection, and no lookup by string,
   `TypeId` or `Any` outside the root (AGENTS.md 5.1). A build step compares
   the explicit list with the declared parts and fails on any difference; it
   never adds a part to the list.
2. Scopes: singleton (one for the run), request (built per request and passed
   as an explicit argument, with no ambient lookup), lazy (made on first use,
   initialised once) and named (one of several implementations picked by a
   typed key declared at the root). An ordered list of implementations of one
   trait is a first-class need (for example "all health checks, in order").
   Order is given by an explicit number or by "before" and "after" relations.
   Ties are broken by declaration order in the root. Conflicting relations
   fail the start (SUT-938).
3. The wiring graph is checked at build time, by a procedural macro, wherever
   the types allow it. What cannot be checked then is checked at start, before
   the app accepts any work. A failure lists all problems, not just the first.
4. A part that needs a thing nobody provides, or that two providers could
   satisfy (an ambiguous need), stops the build or the start. The message names
   the part, the need and the candidates.
5. A circular need is refused. The message draws the loop in plain text as
   `A needs B needs C needs A` and suggests the usual ways to break it (an
   event, a shared third part, a lazy need). No setting allows a loop.
6. Every need is a non-null type. An absent value is a typed option that the
   compiler forces the caller to handle. No public API uses a sentinel value
   in place of an option; a lint in the gate checks this.
7. Parts are dropped in the reverse order of creation at shutdown.

### Auto-configuration and conditions

1. A starter crate declares what it provides, what it needs, and the
   conditions under which it is on.
2. Conditions: a type or a feature is present, a setting has a value, a custom
   part of the same kind is absent, the operating system is Linux or Windows,
   a profile is active, and combinations of these. Other systems are out of
   scope (ADR-0021) and recorded as not applicable with the owner's approval.
3. A condition is either build-fixed (it depends only on Cargo features and
   the target, and is decided by a feature or `cfg` fact) or start-evaluated
   (it depends on settings or profiles). The conditions report at start lists
   build-fixed conditions as constants beside the start-evaluated ones. The
   same inputs always give the same report line.
4. A user part always wins over a starter's default part of the same kind. The
   report says so.
5. The conditions report lists every starter and part, on or off, with the
   exact reason in plain words. It is shown by the Engine Room and available
   over local MCP (spec 0020).
6. An app compiles only the starters it names; unused starters are not built.

### Settings, layers and relaxed names

1. Settings are typed structs with defaults. A missing required setting, a
   wrong type or a value outside its range stops the start with a message that
   names the setting and where it came from. For a normal setting the message
   also shows the allowed values. For a secret setting it names only the key
   and the file, never the value or the allowed list. An unknown key is an
   error that names the key and the nearest known key (SUT-940). Nothing is
   silently ignored (AGENTS.md section 5).
2. Layers, strongest first (owner's wording, unverified; the scout copies the
   exact order from the official reference): command-line flags, environment
   variables, profile-specific files, the main files, and defaults. File
   formats: YAML, TOML and JSON. `boot.profile` and `boot.strict_migration`
   are read from flags and environment only, before files are read.
3. Relaxed names (unverified): a name is split into words at `-`, `_`, `.`
   and camel-case boundaries and compared in lower case, so `my-key`, `myKey`,
   `my_key` and `MY_KEY` are one setting. The environment form is the upper-case
   name with `_` in place of `.` and `-`. Where an environment name could map
   to two known settings, it fails with both candidates named. Two spellings of
   one setting in the same layer fail the start (SUT-939). Typed parsing uses
   a mature crate; the name rules are our own code, tested with a table.
4. The app can print, for any setting, its final value and which layer won.
5. Every setting has metadata (name, type, default, range, description, secret
   or not) built with the program. The metadata feeds settings screens, the
   Engine Room and the configuration changelog diff of spec 0081.
6. A secret value never appears in a log, a report, an error or a screen
   (AGENTS.md section 5).

### Refreshable settings

Each setting declaration has a boolean `refreshable` flag, default false.
Only an ordinary `live` key under spec 0014 may opt in. Its declaration
names the safe read boundary, eligible sources, bound and reason. A
restart key, wiring or profile selection, credential, secret, protected
permission or owner-only setting cannot enter a remote refresh. A source
cannot set the flag or grant itself authority.

1. The local product settings path remains under specs 0014 and
   0020, including current approvals for tool requests. For the
   framework, authenticate app, profile, scope and source. Bind
   layer placement and label selection to trusted bootstrap
   metadata. Apply a pre-merge allow-list. Refuse unknown, masked
   or unchanged forbidden keys and duplicate relaxed names.
   Check the resolved whole candidate, including changed or
   removed values, limits, cross-key rules and permissions.
2. Bind replay to authenticated source, scope, version and digest.
   Changed bytes at the same identity fail. An identical repeat
   returns its original result only within current authority.
   Atomically compare base generation, grant and policy revision
   at swap. Expired or revoked authority and stale proposals fail.
   Each read uses one complete snapshot. Ordinary settings stay
   pinned. Declared live boundaries and permission checks use
   current state under spec 0014.
3. Bound held and staged generations under spec 0014. Failed
   validation or activation keeps the current snapshot. After a
   proved health failure, try the last good compatible snapshot.
   Atomically verify the failing generation is still current,
   source scope and authority remain valid, and protected values
   stay intact. Otherwise hold affected work. A stale rollback
   cannot overwrite a newer swap or revocation.
4. Audit source, scope, generation, affected key names and reason. Keep
   values out of errors and history for protected keys. Versioned
   source and authority checks block restore or replay of revoked
   configuration. Spec 0111 owns remote transport, not this contract.

### Profiles and config trees

1. The built-in profiles are `dev`, `test` and `prod`. `prod` is the default
   for an installed app. Custom profile names are allowed, and a profile may
   include others through a profile group (unverified).
2. A profile can change settings and switch starters on or off, nothing else.
3. A config tree is read from a folder where each file name is a setting name
   and the content is its value. It works for mounted secrets. Read access to
   the folder is checked at start. "Too open" means readable by other users: on
   Linux, group or world read bits; on Windows, an access list that grants
   read to anyone other than the owner, the system and administrators. A folder
   that is too open gives a warning; the Engine Room shows it (a line is to be
   added to spec 0020).
4. Settings that differ by profile are listed in the metadata so a person can
   see them side by side.

### The strict migration check

1. A table of renamed and removed settings travels with each release of
   Behemoth Boot. Generate it from the exact reviewed, source-confirmed
   0081 parity rows and the matching local settings schema. It covers
   the keys Behemoth Boot defines, not other programs' keys. Keep the
   reviewed inputs and the generated output distinct in the 0139 build
   and artefact evidence. Verify that the generated resource is present
   in the package under 0113 BI-12. In sealed 3B qualification, take
   the expected digest from the harness-verified exact candidate build
   inventory, fixed before faults are planted. This check precedes
   3B acceptance. At a protected product start, take the digest
   from the admitted artefact cell for the exact build and profile.
   Match the table to that digest and parse the same verified bytes.
   If the table or its binding is missing or mismatched, stop before
   start hooks, listeners, jobs
   or file changes. Use SUT-934 with “Cannot verify the settings
   migration table.” Use the [same-release recovery help][boot-help].
   A version label or restored copy cannot supply the binding.
2. With `boot.strict_migration` on, any setting found in a file that appears in
   the table stops the start, with one plain line per setting and its new name
   or the note that it is removed.
3. With it off, the same lines are logged as warnings once per start.
4. The check never changes a file. A fix is something the user does, or a
   proposal the app shows (AGENTS.md section 5).

N-1735 uses the two start-up variants below under BB-12 and BB-16. Both use
this spec's release migration table, admitted flag and environment sources,
typed settings, masking and resource limits. Config files cannot select the
mode. Their keys stay under normal typed validation. Record each applicable
variant and target profile on its 0081 parity row. Use
`FX-0106-W5-STRICT-MIGRATION-BEFORE-START-EFFECTS` under BD-BB-03 and FI-BB-03
for the planned cases. The fixture covers shadowed outdated keys, unknown
and secret-marked keys, and an attempt to select the mode from a file.
With the same admitted build and settings, replace just the table with
changed bytes under the same resource name and release label. Then omit
the table. Both modes refuse before any start hook, listener, job or
file change and exit non-zero under BB-16. Re-run with the exact table.
A listed outdated key then triggers the strict or warning behaviour below.

- `VAR-0106-MIGRATION-STRICT` applies when the admitted setting is on. A
  listed outdated key stops start-up with SUT-933 and leaves no open port,
  work or changed file. Emit the safe key diagnostic. Without an outdated
  key, proceed only through the existing typed and lifecycle checks.
- `VAR-0106-MIGRATION-WARNING` applies when the admitted setting is off.
  The same outdated-key diagnostic appears once as a warning. Continue
  typed validation without changing files or masking another start failure.
  Failure of those checks keeps the owning error and no start effects.

### Start-up, background initialisation and the optimiser

1. Start-up is deterministic. The order is the same on every run: read
   settings, check them, build the wiring, create singletons in dependency
   order, run start hooks, open ports, then report ready.
2. A part may be marked "background": it starts on a separate task after the
   rest and the app reports ready only when every part that readiness needs is
   done. A background part that fails after ready makes its own health state
   bad and never stops the rest.
3. At build time, a step precomputes the wiring order so that start-up does
   the least possible work. Later groups may add their own build-time plans
   under their own specs. If the step is skipped the app still starts and
   gives an identical result, only slower; tests run both ways.
4. Proposed target (measured, no promise on another machine): a small app (20
   parts, no starters) on the Linux reference machine starts in under 200 ms
   and uses under 50 MiB of resident memory at the ready event, over 30 cold
   runs, at the 95th percentile. The full Sutradhar app gets its target after
   the first measurement; none is promised now.
5. The start log has a timeline: how long each part took, so slow parts are
   obvious. The detailed flight recorder is group PB-H.

### Lifecycle and graceful shutdown

1. The app has named states: `starting`, `ready`, `draining`, `stopped` and
   `failed`. Group PB-H reads these states to build health detail and does
   not add states.
2. Lifecycle events are plain in-process callbacks in this spec, in this
   order: settings loaded, wiring built, started, ready, shutdown requested,
   drained, stopped. Group PB-F may later route them through the event bus
   without changing their order.
3. On a stop signal the app moves to `draining`: it stops accepting new work,
   lets work in flight finish up to `boot.shutdown_timeout_s`, then stops
   parts in reverse order. Work that does not finish in time is cancelled at
   its next safe point and logged by name.
4. A second stop signal during draining stops sooner, still in order.

   In the 3C product signing fixture, apply this boundary to the
   current product `KeySource` caller. Spec 0065 owns the durable
   signing result. A stopped product caller cannot use a late
   reply to resume a lifecycle state or a site send. Reuse
   IT-0065-BOOT-SIGNER-CONFORMANCE with FI-BB-02 and IT-BB-01
   for the exact product drain and signed-result observations.
5. A failure in a start hook leaves no half-open port and no stray file, and
   exits non-zero.
6. The engine supervisor (spec 0030) is a separate program that is not built
   on this core. It reads exit code and an atomic state file. Each sample
   binds lifecycle state and spec 0112's monotonic heartbeat to the launched
   instance identity, not a process ID alone. Write on state change and
   heartbeat. The total of the drain timeout and the stop of
   parts must stay under the Quit limit of spec 0030 (90 seconds, proposed);
   the shorter of the two wins. Memory and job admission stay with spec 0078.

### Reuse

Our own code is the composition root, the container checks, the conditions
and report, the relaxed-name rules, the migration table, the lifecycle and the
shutdown order. Mature crates stay behind them: serialisation and typed
settings parsing, file-format parsers, the async runtime, structured logging,
and signal handling. The choice of each crate is a build-or-reuse record in
the research dossier (AGENTS.md section 6 rule 14).

### Parity rows and tests

Each feature above becomes parity items in the ledger of spec 0081 under the
`PB-B` group. For every item the scout writes a behaviour description, and
another agent writes a contract test from it before the implementer starts.
The tests are the oracle for `matched`.

#### Exact start-up and configuration contracts (N-1735–N-1742)

Use the explicit typed composition root, 0014 settings metadata
and the existing startup and lifecycle order. Bind configuration,
candidate and parity evidence to exact versions. Emit safe reasons
before readiness or another protected effect. A report, condition,
profile or build plan cannot add ambient authority or authorise a
different runtime. Preserve the existing refresh contract and
never-off rules.

| Owner catalogue entry | Exact framework contract |
| --- | --- |
| N-1735 Strict startup migration blocker | Read boot.strict_migration only from admitted flags and environment before opening config files. Compare every inspected file against the exact release migration table, including lower-layer and shadowed old keys. Strict mode holds before opening a port or starting work. Warning mode follows the existing typed validation and never rewrites config. Report aliases, removed keys and unknowns without revealing values. [FX-0106-W5-STRICT-MIGRATION-BEFORE-START-EFFECTS](../planning/feature-wave-5.md#acceptance-design-for-n-1719-to-n-1742). |
| N-1736 Modular auto-configuration architecture | Compile named starter crates through the explicit composition root with typed offers, needs and declared ordering. Preserve the registered override rule. Unnamed or unadmitted parts are absent. No discovery, plugin loading, ambient lookups or runtime activation from a dependency being present. [FX-0106-W5-EXPLICIT-STARTER-COMPOSITION](../planning/feature-wave-5.md#acceptance-design-for-n-1719-to-n-1742). |
| N-1737 Rich conditional activation engine | Use admitted explicit predicates for target, type or feature and registered settings or profile. Distinguish build-fixed and start-evaluated inputs and report deterministic masked reasons. Unknown or ambiguous conditions hold under the existing validation contract. A disabled safety rule, invalid user override or dynamic loading cannot follow from a predicate. [FX-0106-W5-DETERMINISTIC-CONDITIONS-NO-OVERRIDE](../planning/feature-wave-5.md#acceptance-design-for-n-1719-to-n-1742). |
| N-1738 Strict circular-reference blocker | Check typed dependencies and explicit order constraints at build time where known and before effects otherwise. Show the exact loop with safe part names and clear reason. An optional or lazy declaration is valid only if the effective graph is acyclic. No setting, profile or optimiser may admit a circular graph. [FX-0106-W5-EFFECTIVE-DEPENDENCY-CYCLE-REFUSAL](../planning/feature-wave-5.md#acceptance-design-for-n-1719-to-n-1742). |
| N-1739 Layered configuration and relaxed binding | Use versioned exact layer precedence and canonical key rules qualified by the existing parity tests. Retain provenance and reject duplicate normal forms in a layer and ambiguous aliases. The winner in a higher layer cannot bypass validation or ownership; lower-layer values stay masked where protected. No assumption of upstream behaviour without a verified row. [FX-0106-W5-CONFIG-LAYER-PROVENANCE-AND-OWNERSHIP](../planning/feature-wave-5.md#acceptance-design-for-n-1719-to-n-1742). |
| N-1740 Strong profile and config-tree system | Keep built-in dev, test and prod. Admit profile groups only within the existing validated profile contract. A read-only tree binds the owner-approved stable root and file identities and effective reader. Check containment, link or mount replacement, inherited grants, native ACL and 0065/0118 custody before reading secret data. Framework BB-11 may show a safe warning without values or identifying paths. Unsafe product secret use holds under custody rules. A profile changes only eligible settings and starters. No new mount, secret exposure or permission escalation. [FX-0106-W5-PROFILE-TREE-CURRENT-CUSTODY](../planning/feature-wave-5.md#acceptance-design-for-n-1719-to-n-1742). |
| N-1741 Background component initialisation | Bind startup and readiness to exact build, config, graph and instance generation. Start independent declared parts within the existing bounded lifecycle. Critical readiness waits for every required part. Safe optional failure degrades its own health; a required failure closes provisional listeners and removes provisional files. Fence late callbacks. Reconcile stop, timeout and late completion before accepting readiness or results. No component may open an unowned listener or change dependency order. [FX-0106-W5-STARTUP-REQUIRED-READINESS-AND-STOP](../planning/feature-wave-5.md#acceptance-design-for-n-1719-to-n-1742). |
| N-1742 Ahead-of-time compile-time optimiser | Use the existing precomputed wiring order bound to exact candidate, target and graph. Plans for routes or serialisation belong to their owning specs and require parity proof. The runtime fallback must have valid phase and exact checked success and failure parity. A retained but phase-incompatible plan is unavailable. Use existing measured startup and memory targets for admitted profiles, with no unmeasured latency promise or live replacement. [FX-0106-W5-EXACT-PRECOMPUTED-WIRING-PARITY](../planning/feature-wave-5.md#acceptance-design-for-n-1719-to-n-1742) |

QR-213 qualifies deterministic graph and conditions evidence, migration
and config custody, readiness races, and measured parity of the
precomputed and ordinary start paths.

#### Typed values and shutdown (N-1743–N-1744)

| Owner catalogue entry | Exact framework contract |
| --- | --- |
| N-1743 Non-null safety contracts by default | Require typed values for required inputs. Express an admitted absence with an explicit option at the owning boundary. Validate decoded or configured values before forming typed state. Missing required input, invalid type and absent optional input remain distinct. No public sentinel, silent default or inferred permission. [FX-0106-W5-EXPLICIT-TYPED-ABSENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1743-to-n-1766) |
| N-1744 Unified lifecycle and graceful shutdown engine | Use 0106 ordered in-process lifecycle callbacks and safe observations for exact installation, instance and generation. 0030 and 0112 keep their ownership. No new durable event path. Mark ready only after the declared readiness dependencies pass. Stop new admissions before draining eligible work under existing timeout and Quit bounds. Stop at the earliest admitted bound, fence pending activation and late readiness, close provisional listeners, stop in reverse dependency order, preserve receipts for unknown work and fence late events. A second stop follows the existing escalation without granting a new retry, remote control or release action. [FX-0106-W5-ORDERED-STOP-BOUNDED-DRAIN](../planning/feature-wave-5.md#acceptance-design-for-n-1743-to-n-1766) |

#### Explicit dependency container (N-1880)

Use the one typed composition root and declared lifetimes,
named keys and ordered lists under the existing container
contract. Qualify framework lifetimes under spec 0081 separately
from the admitted Sutradhar profile. A run-scoped instance
owned and passed explicitly by the root creates no global
singleton, shared mutable service locator or cross-site grant.
Request-scoped construction inherits only the current admitted
request context. Lazy construction cannot bypass the effective
acyclic graph or start work during discovery.

Refuse missing or ambiguous providers, conflicting order and
cycles before work. Pin the exact build-fixed and runtime
inputs and drop parts in the existing reverse order. No
reflection, ambient lookup, package load or runtime activation
from an available provider. Use the existing 0106 container
requirements and QR-219 for typed order, lifetime and phase
boundary fixtures. DR-070 allows no Spring execution.

Use [FX-0106-W5-TYPED-CONTAINER-SCOPE-ORDER-AND-CYCLE-REFUSAL](
  ../planning/feature-wave-5.md#acceptance-design-for-n-1863-to-n-1880).

[boot-help]: ../../manual/troubleshooting.md#recover-a-start-up-check

### Data

- Ledger and behaviour descriptions: repository files (spec 0081).
- The migration table and the setting metadata: generated files shipped with
  each release and checked in tests.
- The conditions report and the start timeline: generated at each start, kept
  with the latest run, never in the database.

### Settings

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `boot.strict_migration` | on or off | off | two values | next start |
| `boot.shutdown_timeout_s` | seconds | 30 | 1 to 80 | next stop |
| `boot.background_init` | on or off | on | two values | next start |
| `boot.conditions_report` | on or off | on | two values | next start |
| `boot.profile` | `dev`, `test`, `prod` | `prod` | three values | next start |
| `boot.start_timeline` | on or off | on | two values | next start |

There is no setting that allows a circular need or that skips setting checks.

### Failure codes

SUT-930 setting missing, wrong type or out of range; SUT-931 circular need;
SUT-932 need nobody provides; SUT-933 outdated setting name (strict mode);
SUT-934 config tree unreadable or required release settings migration
table unreadable or unverifiable; SUT-935 start hook failed; SUT-936 shutdown
timeout, work cancelled; SUT-937 two parts claim one name; SUT-938 conflicting
order relations; SUT-939 two spellings of one setting in a layer; SUT-940
unknown setting. Every start failure lists all problems found and exits non-
zero.

### Security and privacy

- Settings with a secret mark are masked everywhere.
- A config tree is read-only to the app.
- The conditions report and the setting listing show names and sources, never
  secret values.
- A profile cannot widen permissions or switch off a never-off safety rule.

## First product slice demonstration

`REQ-0106-FIRST-SLICE` belongs to the [roadmap's 3C exit][first-slice].
`DES-0106-FIRST-SLICE` binds the integrated product journey for
`TASK-0106-FIRST-SLICE-DEMO`. Use
[`BDD-0106-FIRST-SLICE-DEMO`][first-slice-case] on the exact product
candidate and each required consuming cell of the protected 3C
profile. Before the demo, resolve the current signed 3A and 3B
chain and the exact Boot artefact cell actually used through
0076 and 0139. A qualified framework host alone supplies no
product journey.

| Existing platform variant design | Product transport and evidence |
|---|---|
| `VAR-0106-FIRST-SLICE-UBUNTU` | Ubuntu 26.04 x86-64 with the admitted 0030, 0020 and 0118 owner-only Unix socket profile. |
| `VAR-0106-FIRST-SLICE-DEBIAN` | Debian 13 x86-64 with its own admitted 0030, 0020 and 0118 owner-only Unix socket profile. |
| `VAR-0106-FIRST-SLICE-WINDOWS` | Windows 11 under its admitted 0030, 0020 and 0118 owner-locked named-pipe profile. |

QR-100 qualifies each applicable product host and local door.
Use 0020 AT-LOCAL-TRANSPORT, 0118 IT-0118-LOCAL and 0112 BH-01
for the exact door. Use 0030's normal-install lifecycle and
the existing supervisor case for window close and explicit
Quit. Bind the durable job and recovery through 0019
IT-0019-SCHEDULER-RESTART. Use BB-03, BB-07 and BB-14 to BB-16,
FI-BB-01 and FI-BB-03 for the startup and readiness results.
Keep the owning audit, database, settings and signing tests
as separate evidence in the 3C profile. The exact observed
`TASK-0020-OWNER-DOOR` result binds the selected action's scope
and its current Cedar outcome. Prove the settings and independent
review obligations separately. Use 0139 BDD-0139-ORDER to test
substitution between those evidence roles.

Bind the action and class selected under [QR-252](../registers/questions.md)
to the exact action and job receipts in the journey. Use the owning
0020 and 0019 fixtures for both the admitted behaviour and its refusals.
A fixture from another action, class or schema version leaves the required
journey evidence unproved. Where 0110 supplies the job trigger, verify
the one committed event and its job effect in the same journey.

The 0010 verifier resolves this evidence through the existing
`first-product-slice` subject and required proof roster. Bind
the observed result to the product candidate, OS, profile,
exact adopted Boot cell, job and fixture instance. A valid
journey from another cell cannot stand in. Use the current
protected format and signer. An undefined format or missing
result holds that proof. This trace adds no record format or
phase authority.

[first-slice]: ../roadmap.md#3c-first-product-slice
[first-slice-case]:
  ../planning/test-and-traceability.md#task-0106-first-slice-demo

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| BB-01 | Parts MUST be wired only from the explicit composition root, with no run-time discovery, crate-wide auto-collection or lookup by string, `TypeId` or `Any` outside the root. | UT-BB-01, PL-BB-01 |
| BB-02 | The container MUST support singleton, request, lazy and named scopes and ordered lists. | UT-BB-02 |
| BB-03 | A missing or ambiguous need (two providers) MUST stop the build or start and name the part, the need and the candidates (SUT-932, SUT-937). | UT-BB-03 |
| BB-04 | A circular need MUST be refused with a plain loop diagram and no setting MUST allow it. | UT-BB-04 |
| BB-05 | Build-fixed conditions MUST appear as constants in the start report, and the same inputs MUST give the same report line. | UT-BB-05, PR-BB-01 |
| BB-06 | The conditions report MUST list every starter and part with its reason. | BD-BB-01 |
| BB-07 | A missing, wrong or out-of-range setting MUST stop the start and list all problems. | UT-BB-06, BD-BB-02 |
| BB-08 | The layer order and relaxed-name rules MUST match the parity ledger. | UT-BB-07, CT-BB-01 |
| BB-09 | A secret MUST never appear in a log, report or screen. | UT-BB-08, FZ-BB-01 |
| BB-10 | Profiles MUST change only settings and starters. | UT-BB-09 |
| BB-11 | A config tree MUST be read as one setting per file, with a warning for open permissions. | UT-BB-10 |
| BB-12 | The strict migration check MUST stop the start on an outdated name and MUST NOT edit files. | BD-BB-03 |
| BB-13 | Start-up order MUST be the same on every run. | PR-BB-02 |
| BB-14 | A background part that fails after readiness MUST turn its own health bad and MUST NOT stop the app. A required part that fails during startup follows BB-16. | FI-BB-01, FI-BB-03 |
| BB-15 | Shutdown MUST drain in-flight work up to the timeout and stop parts in reverse order. | BD-BB-04, FI-BB-02 |
| BB-16 | A start failure MUST leave no open port or stray file and exit non-zero. | FI-BB-03 |
| BB-17 | Each feature MUST have parity rows, behaviour descriptions and independent contract tests. | AU-BB-01 |
| BB-18 | A small app MUST meet the stated start-time and memory targets on the reference machine, measured over 30 cold runs at the 95th percentile. | MS-BB-01 |
| BB-19 | States and lifecycle events MUST follow the stated order, and a second stop signal MUST stop sooner in order. | UT-BB-11, FI-BB-02 |
| BB-20 | An unknown setting MUST fail with the nearest known key, and two spellings of one setting in one layer MUST fail. | UT-BB-12 |
| BB-21 | An app built with and without the build-time step MUST give identical results. | PR-BB-03 |
| BB-22 | No public API MUST use a sentinel value in place of an option. | UT-BB-13 |
| BB-23 | A user part MUST win over a starter's part of the same kind and the report MUST say so. | UT-BB-14 |
| BB-24 | The final value and winning layer of any setting, and the start timeline, MUST be printable. | BD-BB-05 |
| BB-25 | A secret setting's error message MUST NOT show its value or allowed values. | UT-BB-15, FZ-BB-01 |
| BB-26 | Conflicting order relations MUST fail the start (SUT-938). | UT-BB-16 |
| BB-27 | The drain timeout plus the stop of parts MUST stay under the Quit limit of spec 0030. | IT-BB-01 |
| BB-28 | A live refresh MUST validate a full admitted candidate and MUST swap one current generation or leave it unchanged. | IT-BB-02, FI-BB-04 |
| BB-29 | Refresh and rollback MUST NOT widen authority, change protected keys or displace a newer generation. | UT-BB-17, FI-BB-04 |

## How it is tested

Designed, not run.

- **Unit (UT-BB):** every container case with one passing and one failing
  example; a loop of two, three and ten parts; condition tables; the relaxed
  name table with one value just inside and one just outside; layer order with
  each pair of layers in conflict; masking.
- **Planted faults (PL-BB):** a part found by run-time discovery, a part added
  by crate-wide auto-collection, and a lookup by `TypeId` outside the root; each
  must be refused with a named message, while an honest list that matches its
  declarations passes.
- **Property (PR-BB):** random wiring graphs give the same build and start
  decisions; start order is stable under reordering of declarations.
- **Behaviour (BD-BB):** the Nia scenario with the switch on and off; a
  conditions report on a sample app; a shutdown with three tasks in flight.
- **Fault injection (FI-BB):** a failing background part; a stop signal during
  start; a hook that panics.
  In the 3C FI-BB-03 product fixture, retain a previously ready
  instance's genuine atomic state sample. Start a replacement with
  its own advancing heartbeat, a provisional listener and file, and
  one idle synthetic native child of a required hook. Leave the hook
  unready. Offer the old ready sample while one eligible ordinary
  job awaits its first claim. Use 0030's exact launched instance and
  0112 BH-05. An old sample and a current heartbeat supply no current
  readiness or permission to claim the queued job. Observe no job
  lease, worker launch or business effect. Fail the hook with
  SUT-935. Verify non-zero exit, listener and file cleanup, and exact
  native child exit through the admitted process controller. Leave
  an unrelated process untouched. In the separate control a current
  instance reaches ready and the ordinary 0019 checks admit the
  eligible job. This case reads a stored sample without a callback.
- **Fuzz (FZ-BB):** random setting files and names never crash the loader or
  leak a secret.
- **Contract (CT-BB):** one test per parity item, written from the behaviour
  description by someone other than the implementer.
- **Audit (AU-BB):** a sample traced from official source to description to
  test.
- **Refresh (UT-BB-17, IT-BB-02, FI-BB-04):** deny missing flag,
  restart, owner-only, shadowed forbidden, duplicate and unknown
  keys. Check metadata default, wrong scope and protected fallback
  on removal. Refuse a bad coupled key and an exhausted snapshot
  cap. Race proposals and authority expiry at swap and rollback.
  Test same and changed replay digests, pinned readers and a newer
  winner. Keep protected values out of history and errors.
- **Measurement (MS-BB-01):** start time and resident memory at ready for a
  small app (30 cold runs, 95th percentile) and, for information only, for the
  full Sutradhar app on the 8 GiB reference.

## Changes from the source

AGENTS.md section 5.1 says Boot is a small framework with the stated
principles and spec 0081 owns the group codes. ADR-0021 selects the
PostgreSQL queue and outbox. Spec 0014's committed settings event
wakes the watcher. It does not bypass this spec's refresh admission
and atomic snapshot checks.
