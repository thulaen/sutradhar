# 0015: Module registry and lifecycle

Sutradhar is built from modules: internal linking (setu), SEO signals
(sanket), karma and so on, each its own Rust crate on a small core. The
module registry, mandali ("troupe"), finds the modules, starts them in
the right order, keeps one broken module from breaking the others, and
lets the owner see and switch them. The composition root that wires them
is sanyojan ("coordination"). It is part of Sutradhar Boot, roadmap
Phase 3 item 1.

## What it does

### Words used here

- Module: one feature part with a glossary name (such as `setu`), its
  own crate, settings section, jobs, routes and tables.
- Core: the crates every module may use and that cannot be switched
  off: settings (vinyas), events (sanchar), jobs (karya), storage
  (bhandar), audit (lekha), health and metrics (drishti), signing
  (dwarpal), the site link (sampark) and this registry (mandali).
- Descriptor: a module's fixed facts.
- Registrar: the object a module uses at start-up to declare its
  settings, jobs, event handlers, routes and commands.
- Mode: shadow (works and logs, shows nothing to anyone) or review
  (moderators see and approve each suggestion). There is no automatic
  mode in version 1.
- Faulty: failed to start, panicked, or kept failing its health check.
  Waiting: cannot run yet because something it needs is off, faulty or
  missing.

### Crates and the direction of use

1. The binary `sutradhar` holds the composition root. It uses feature
   crates (`sutradhar-setu`, `-sanket`, `-karma`, `-namaste` and so
   on), which use core crates (`-mandali`, `-karya`, `-sanchar`,
   `-vinyas`, `-sampark`, `-dwarpal`, `-bhandar`, `-lekha`,
   `-drishti`), which use `sutradhar-base`.
2. `sutradhar-base` (an English placeholder name) holds the shared base
   types: the `Clock` trait, the SUT error code type, typed ids.
3. Feature crates may use core crates and the feature crates they
   declare. Core crates never use feature crates. cargo-deny's `bans`
   wrappers rule enforces this: a feature crate may be used only by the
   binary and the features that declare it.
4. `unsafe_code` is forbidden in every crate here.

### The `Module` trait

```rust
#[async_trait]
pub trait Module: Send + Sync + 'static {
    fn descriptor(&self) -> &'static Descriptor;
    fn register(&self, reg: &mut Registrar<'_>) -> Result<(), ModuleError>;
    async fn start(&self, cx: StartContext) -> Result<(), ModuleError>;
    async fn health(&self) -> Health;
    async fn stop(&self) -> Result<(), ModuleError>;
}
```

- `async-trait` keeps the trait usable as `Box<dyn Module>`.
- `register` is quick and does no input or output: it only declares.
- `start` gets a `StartContext`: the shared services it asked for, the
  settings handle, a child cancellation token and a child task tracker.
  Every task the module spawns goes through its tracker, so the
  registry can wait for them and see their panics. A workspace lint
  test refuses `tokio::spawn` in feature crates.
- `health` answers within 2 seconds or counts as failed.
- `stop` stops taking work, lets current work reach a safe point and
  returns within 10 seconds; the registry then cancels its token.
- `Descriptor` fields: `name`, `meaning` (shown first in help),
  `version`, `contract` (the core contract version it was built
  against), `requires` and `optional` (other modules), `core` (cannot
  be switched off), `settings_section`, `capabilities` (capability ids)
  and `tables` (the tables it owns).

### Registration and wiring

1. Modules are compiled in. The binary holds one list,
   `modules::all()`. Nothing is loaded at run time. A test reads the
   workspace metadata and fails if a crate marked
   `[package.metadata.sutradhar] module = true` is missing from the
   list or listed twice.
2. The registrar checks each declaration and stops start-up, naming
   the module, on: a name already taken; a settings section not the
   module's own; a route outside `/api/v1/modules/<name>/` (so no
   module can shadow health or core routes); a job or event handler
   not prefixed with the module's name; a surface without a capability
   id.
3. The composition root builds each shared service once, as
   `Arc<dyn Trait>`, in a plain `Services` struct. A module that offers
   something to others (for example shodhan's text checks) registers
   it as a provided service. A module gets it from its `StartContext`
   only if its descriptor `requires` the provider. Asking for anything
   undeclared is refused at start-up (SUT-158). Nothing is built
   automatically, there are no per-request scopes and nothing is found
   by reflection: it is a checked list, not a container.
4. Each request's data (the viewer, the trace) travels in the request,
   through Axum's extractors, never in shared state.

### The dependency order

1. The registry builds the graph of `requires` among switched-on
   modules and orders it with Kahn's algorithm, breaking ties by name
   so the order is always the same. It is about 40 lines of our own
   code, with no graph crate.
2. A loop stops start-up with its full path: "setu needs sanket needs
   setu (SUT-152)". A module that requires itself is a loop of one.
3. Start follows the order; modules with no path between them start
   together. Stop goes in reverse order.
4. `optional` needs never block. A missing optional module shows as
   "running without SEO signals"; one not built shows as absent.
5. The contract version check matters only for third-party extensions
   (spec 0037), since a mismatched built-in module cannot compile. It
   is kept and tested.

### The lifecycle

States: `disabled`; `waiting` (reason names what it waits for; its
settings and weights stay as they are); `starting` (at most 30
seconds); `running`; `degraded` (running with a problem it can live
with, such as a slow dependency); `stopping`; `stopped`; `faulty`
(reason carries a SUT code and a manual link).

Moves: disabled to waiting or starting when switched on; waiting to
starting when its needs are met; starting to running or faulty; running
to degraded and back, to faulty, or to stopping; stopping to stopped;
faulty to starting on a retry; any state to disabled when switched off.
Every move is one history row and one event.

The registry is one Tokio task. It owns the states; nothing else
changes them. Modules report through their health check and tracker;
the registry decides.

### Faults and isolation

1. A panic in any module task is caught by its tracker: the module
   becomes faulty (SUT-161), its token is cancelled and its other tasks
   get 10 seconds to stop.
2. While a module is faulty or waiting: karya claims none of its jobs
   (it asks the registry first); its routes answer 503 with the plain
   reason; its event handlers' work stays queued, not lost; every
   module that requires it becomes waiting, so no output that depends
   on it is made or shown.
3. A faulty module is retried after 1, 5 and 15 minutes, then stays
   faulty until the owner presses "Retry" (`sutra mandali retry
   <name>`). After a service restart it starts normally and the retry
   count starts from zero.
4. A fault never changes the module's settings, mode or weights.
5. Work under way at the fault resumes from its last checkpoint
   (spec 0019); nothing is skipped silently.
6. A health check that panics counts as failed, not as a crash.
7. A panic in the core is a defect: the service exits, systemd restarts
   it, and an AutoIssue (an automatic problem report) is filed.

### Modes and promotion

1. Version 1 has `shadow` and `review`. A mode of `auto` is refused by
   the settings check (SUT-166).
2. `modules.<name>.mode` is a ceiling. The effective mode is the lower
   of the ceiling and the promotion stage. The default for every
   module is review; shadow stays for testing new modules. A lowered
   ceiling wins at the next read.
3. Promotion from shadow to review is a karya job. It reads measures a
   module supplies through a `PromotionMeasure` trait, compares them
   with the module's `promotion.*` settings, and records the decision
   in lekha and on the health screen. It never goes past the ceiling
   and never reaches auto.
4. The measure is the module's link-reconstruction score on the site's
   own links, from a ganit evaluation job, against a threshold the
   owner approves after the lab measures it. With no approved
   threshold the job reports "threshold not approved" and promotes
   nothing.
5. Shadow work is logged as what would have happened and is never shown
   to members or moderators.

### Health and readiness

1. Live: the process is up. It never checks modules, so a slow module
   cannot get a healthy service restarted.
2. Ready: every core module running, the settings version applied and
   the database reachable. A faulty feature module makes the service
   "ready, with problems", not unready. A faulty core module makes it
   unready.
3. Health checks run in parallel, each with a 2-second limit. The
   combined answer is kept for 5 seconds, so frequent callers cannot
   overload modules.
4. If ganit's serving part is down, the service is "ready, with
   problems" and live features use the degraded mode (top-k with the
   hard rules); it never makes the service unready.
5. systemd's readiness and start order follow "ready".

### Switching modules on and off

1. The switch is the setting `modules.<name>.enabled`, changed through
   the settings system (spec 0014), so it has one write path, a history
   and a reason. Core modules have no switch.
2. Switching on a module whose required module is off is refused
   (SUT-164: "turn on SEO signals first"). Switching off a module that
   others require is refused, naming them (SUT-165).
3. A switch takes effect between jobs: switching off lets current jobs
   reach their next safe point, then stops the module.

### Data and migrations

Schema `mandali` in PostgreSQL:

- `module_state`: `module`, `state`, `mode`, `reason_code`, `since`,
  `module_version`; key `module`. Written on every move and refreshed
  every 30 seconds; rows unseen for 5 minutes show "not reporting".
- `module_transition` (append-only): `id`, `module`, `from_state`,
  `to_state`, `reason_code`, `actor_ref` (null for automatic moves),
  `at`.
- `module_promotion`: `id`, `module`, `measure`, `value`, `threshold`,
  `output_version` (the ganit output used, if any), `decision`
  (promote, hold, demote), `at`.

Migrations live in one folder, `migrations/`, applied in order by
sqlx. Each file is named `<sequence>_<module>_<what>.sql`. A gate check
refuses a file that creates or changes a table not in that module's
descriptor (SUT-160). A switched-off module's tables are still created,
so switching it on needs no migration. Each table is declared either a
record (new versions only, never updated or deleted at run time) or
working state, as spec 0018 sets out.

### Row-level security rules

These rules are written once here; each specification lists only its
own tables' policies.

1. Only the migration role owns tables, and it runs only migrations.
   The service connects as the app role, which owns no tables, is not a
   superuser and cannot bypass row-level security.
2. Two narrow roles, each for one job only: an erasure role whose only
   policies are delete on spec 0032's `audit_details` and `subjects`
   (audit events stay append-only while details can be erased); and a
   setup role, the only one that may write spec 0030's installation
   tables.
3. A table whose rows can belong to more than one site carries
   `site_id`. Each transaction sets its site with `SET LOCAL`, and
   every policy requires the row's site to match, so a query that
   forgets its filter still sees only its own site. With no site set,
   every policy returns no rows.
4. Each module's tables live in its own schema. Every table has
   row-level security enabled and forced, and explicit policies naming
   the role and the command. Grants follow the policies; nothing is
   granted to `PUBLIC`.
5. Append-only tables have no update or delete policy for any role.
6. Policies use simple comparisons on indexed columns, and every speed
   target is measured with row-level security on.
7. mandali's own policies: the app role reads all three tables,
   inserts and updates `module_state`, and inserts `module_transition`
   and `module_promotion` rows.

### Management API, command and events

Routes on the management API, through the action layer and Cedar:

- `GET /api/v1/modules`: every module with its state, mode and ceiling,
  what it requires and what requires it, its capability ids and its
  manual page.
- `GET /api/v1/modules/{name}`: the same for one, with its last 50
  moves and its promotion record.
- `POST /api/v1/modules/{name}/retry`: retry a faulty module.
- Switching uses the settings routes (spec 0014), not a second route.

`sutra mandali` (help says "mandali" means troupe): `list`, `show
<name>`, `graph` (the order as text), `retry <name>` and `check`
(resolve the graph and declarations without starting anything).

Events through sanchar: `module.state_changed` (module, from, to,
code) and `module.promotion_decided` (module, decision).

### Error codes

Placeholders in the block SUT-150 to SUT-199:

| Code | Meaning |
|---|---|
| SUT-151 | Two modules claim the same name; both are named |
| SUT-152 | A dependency loop, with its full path |
| SUT-153 | A required module is not in the binary |
| SUT-154 | A route outside the module's own prefix |
| SUT-155 | A surface without a capability id |
| SUT-156 | A job or handler name without the module's prefix |
| SUT-157 | A module's `start` took longer than 30 seconds |
| SUT-158 | A module asked for a service it did not declare |
| SUT-159 | An external module's contract version does not match |
| SUT-160 | A migration touches a table its module does not own |
| SUT-161 | A module task panicked |
| SUT-162 | A health check failed three times or timed out |
| SUT-163 | A module's `start` returned an error (with its own code) |
| SUT-164 | Switching on refused: a required module is off |
| SUT-165 | Switching off refused: other modules require it |
| SUT-166 | The mode `auto` is not available in version 1 |
| SUT-167 | A module's `stop` took longer than 10 seconds |
| SUT-168 | A table has no row-level security, or no policy |

SUT-151 to SUT-160 stop start-up and are listed together. SUT-161 to
SUT-167 affect only the module concerned.

### Security and privacy

- Modules are compiled in and checked by the full gate. Third-party
  code runs only in the restricted workers of spec 0037.
- A module gets only the services it declared, so a module that never
  needed the site link cannot read the site.
- Module routes sit behind the same action layer and Cedar checks as
  core routes.
- States, moves and promotion records hold module names, codes and
  pseudonymous actors only; the person behind an `actor_ref` is in
  lekha's private part.

### Observability

- Metrics, labels bounded to `module` (about 25 values), `state` (8)
  and `code`: `sutradhar_module_state{module,state}` (gauge),
  `sutradhar_module_transitions_total{module,to}`,
  `sutradhar_module_faults_total{module,code}`,
  `sutradhar_module_start_seconds{module}` and
  `sutradhar_module_health_check_seconds{module}` (histograms).
- Traces `mandali.resolve`, `mandali.start` and `mandali.health`, with
  `module.name`, `module.state` and `error.code`.
- Logs `module_started`, `module_waiting`, `module_faulty`,
  `module_stopped` and `module_promotion_decided`, with module, code
  and reason.
- Alerts: a module faulty for more than 15 minutes; a core module not
  running; a module changing state more than 6 times an hour.

### The example module

`namaste` ("hello") shows how to build a module. It registers a
setting, a job, an event handler and a route with capability ids, and
the manual's "Build a module" page is generated from it.

## Limits

- Memory: registry, descriptors and graph for 60 modules at most 2 MiB;
  states and the 5-second health store at most 1 MiB; at most 4 MiB in
  all, inside the service slice. History goes to the database;
  only the last 50 moves per module are held for the screen. A module's
  own memory is in its own specification.
- Time: register and order 60 modules, without their own start work,
  in at most 50 ms (p99), well inside a 60-second restart.
- Ready answers in at most 2.1 s (p99) with one module hanging, and in
  at most 20 ms (p99) from the 5-second store.
- With one module panicking every second, other modules' p99 request
  time stays within 5% of a run with no fault.
- A state move is visible through the API within 2 s (p99).
- `is_runnable` (may this module's job be claimed?) at most 100 ns
  median.
- `start` 30 s, `health` 2 s, `stop` 10 s; retries at 1, 5 and 15
  minutes.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Rust tests in `crates/mandali/` unless named otherwise, using small
test modules and `namaste`, a fake clock, and test PostgreSQL where
tables are involved.

- AT-MR01: a fixture workspace where `sutradhar-karya` uses
  `sutradhar-setu` fails the cargo-deny bans check, naming both.
- AT-MR02: a crate marked as a module but missing from
  `modules::all()` fails a test naming it.
- AT-MR03: two modules named `sanket` give SUT-151 naming both, and
  nothing starts.
- AT-MR04: C requires B requires A: start A, B, C; stop C, B, A.
- AT-MR05 (property test): generated acyclic graphs of up to 60
  modules always start each module after all it requires, and the same
  input gives the same order.
- AT-MR06: setu and sanket requiring each other give SUT-152 with
  "setu needs sanket needs setu".
- AT-MR07: a module registering a health route gets SUT-154.
- AT-MR08: a job with no capability id gets SUT-155.
- AT-MR09: a module asking for the site link without declaring it gets
  SUT-158 and is faulty.
- AT-MR10: B requires A and A's `start` fails: A faulty with SUT-163, B
  waiting "for A", unrelated C runs.
- AT-MR11: a task panic makes the module faulty with SUT-161, cancels
  its token, stops its jobs being claimed, makes its routes answer 503
  with the reason, and other modules keep serving.
- AT-MR12: a faulty module is retried at 1, 5 and 15 fake-clock
  minutes, then stays faulty until `retry`.
- AT-MR13: settings, mode and weights are unchanged after a fault and
  recovery.
- AT-MR14: a health check that never returns: ready answers within
  2.1 s; after three such checks the module is faulty with SUT-162.
- AT-MR15: a faulty feature module gives "ready, with problems"; a
  faulty core module gives not ready.
- AT-MR16: of three modules, the middle `stop` errors and the last
  hangs: all are asked in reverse order, the error is recorded, the
  hang is cut at 10 s with SUT-167, and stopping ends within the grace
  period.
- AT-MR17 (in vinyas): switching on a module whose need is off gives
  SUT-164 "turn on SEO signals first"; switching off a needed module
  gives SUT-165 naming the dependant.
- AT-MR18: a settings draft with mode `auto` gets SUT-166.
- AT-MR19: ceiling shadow and promotion stage review give shadow.
- AT-MR20 (loom): a panic while the registry is stopping and a retry
  timer fires ends in exactly one state, one history row per move.
- AT-MR21 (stress): 200 concurrent requests from different viewers to
  a module route each get only their own viewer's data.
- AT-MR22 (chaos, nightly): a module panicking in a loop for 10
  minutes leaves suggestions and other modules running, the module
  faulty, and one AutoIssue filed, not one per panic.
- AT-MR23 (load, at most 2 minutes in the gate): with one module
  panicking every second, others' p99 stays within 5% of a clean run.
- AT-MR24 (insta snapshot): `sutra mandali graph` output matches the
  stored snapshot.
- AT-MR25: a migration named for sanket that alters a setu table gets
  SUT-160.
- AT-MR26: `namaste` builds, registers its setting, job, handler and
  route with capability ids, and generates the manual page.
- AT-MR27 (SQL catalogue test on a fresh migrated database): every
  table in Sutradhar's schemas has row-level security enabled and
  forced and at least one policy; a fixture migration without them
  fails with SUT-168. The check reads PostgreSQL's catalogue, so a
  comment cannot satisfy it.
- AT-MR28 (Kani proof): for any state and input (switch, fault, health
  result, retry, stop) the transition function gives only an allowed
  move, and a faulty or waiting module is never runnable.
- AT-MR29: shadow module, review ceiling, a very high score and no
  approved threshold: nothing promoted, record says "threshold not
  approved"; once a threshold is approved and met it moves to review,
  never past the ceiling.
- Benchmarks under the 5% median rule: `bench_register_60`,
  `bench_ready_cached`, `bench_is_runnable`; load tests `load_health`,
  `load_fault_isolation`, `load_state_visible`.
- Test-first order: ordering and loops; registration rules; lifecycle
  and faults; health and stopping; switches, modes and promotion;
  concurrency; workspace checks, `namaste` and the Kani proof; load,
  chaos and the snapshot.
- The state machine and switch rules are rule-critical: 100% coverage
  of lines, branches and MC/DC, every mutant killed, a property test
  and a Kani proof.
- Facts checked by tests: sqlx applies one folder in order under its
  advisory lock; cargo-deny's wrappers rule works for workspace crates;
  a panic in a `TaskTracker` task comes back as a `JoinError` with
  `panic = "unwind"`; boxed futures add no measurable cost to `health`;
  Kani proves the transition function within the gate's time; forced
  row-level security binds the table owner and shows in the catalogue.

## Changes from the source

- Several container roles became the one background service under
  systemd; per-role states, logins and edge cases fold into one.
- One login per container role became the single app role of AGENTS.md
  5.2, plus the migration owner and the two narrow job roles.
- Per-forum rows set once per connection became per-site rows set with
  `SET LOCAL` in each transaction; no site set means no rows.
- Compose health checks and restart policy became systemd's.
- ganit's separate containers became a ganit module in the service.
- Health routes and module routes are served on the local management
  socket, checked with Cedar.
- The 20-times speed goal is replaced by the 5% median rule.

Source: `docs/specifications/platform/modules.md`,
`docs/specifications/platform/modules-2.md` and
`docs/specifications/platform/modules-3.md` in the archive
`archive-2026-11-08-old-repo`.
