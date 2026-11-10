# 0014: Settings, profiles and history

Settings, profiles and history is vinyas ("arrangement"), the part of
Sutradhar Boot that holds every setting, checks each change, applies it
safely and remembers every change for good. It also sets the rule that
nothing in Sutradhar is hardcoded. It is roadmap Phase 3 item 1.

## What it does

### In plain words

- Every saved change makes a new numbered settings version. Nothing is
  overwritten: going back makes a new version that copies an old one.
- A change is checked before it is saved. Every wrong value is named at
  once, in plain English, with what is allowed.
- A change takes effect between jobs, never in the middle of one. A job
  that started on version 41 finishes on 41 even if 42 is saved, and its
  record says "used version 41".
- A few settings take effect only after a restart; the screen says
  "restart needed".
- Each version records who asked for it, why, what was expected to get
  better or worse, and later the measured before-and-after results.
- Agents may change settings only with the owner's live confirmation of
  that one plan (spec 0020, rule 5), and the version records it.

Example: an administrator lowers "links per thread" from 5 to 3 with
the reason "fewer links on short threads". The screen shows the one
change, says it applies to new work within seconds, and saves version
42. A backlog batch already running keeps 5 until it ends. A week later
the version shows the accept rate before and after.

### Words used here

- Setting: one named value, such as `linking.thread.max_links = 5`. Keys
  are English, lower case, with dots; the first part is the module's
  section, which may be a glossary name.
- Bootstrap setting: needed before the service reaches its database,
  such as the database address. It comes from files and environment
  variables and always needs a restart.
- Runtime setting: every other setting. It lives in settings versions in
  Sutradhar's database and changes only through the management API,
  used by the desktop app, `sutra` and the MCP server.
- Governed setting: one whose activation needs an approved
  specification or policy change and a qualified release.
- Profile: `dev`, `test` or `prod`. It picks bootstrap files and safety
  rules, never feature behaviour.
- Preset: a shipped starting set of runtime values, `balanced` (the
  default, with everything on) or `aggressive`. Choosing one makes a new
  version.
- Settings version: an unchangeable, numbered record of every runtime
  value, with the changes from its parent and their reasons.
- Active version: the one new jobs use. One pointer row names it.
- Snapshot: the checked, typed, read-only copy of one version that
  running code reads. Reading it never touches the database.
- Apply class: when a change takes effect: `between_jobs` (default),
  `live` (at once, for a few safe keys) or `restart`.
- Bound: the allowed range of a value, such as 1 to 10.

### Scope

- In scope: the `vinyas` crate in `crates/vinyas/`: the registry,
  bootstrap loading with profiles, the runtime store and versions,
  checks, snapshots and their delivery, history, reverts, presets,
  secret handling, its API routes, `sutra vinyas` commands and its
  tables.
- Out of scope: which keys exist and their defaults (each feature's
  specification); the tuner and before-and-after maths (the tuner
  feature and ganit, which never write this store); who may change which
  section (the Cedar permission rules); module switches and modes (the
  modules specification); how jobs take a snapshot (the jobs
  specification).

### Crate layout

The crate depends only on the shared base types crate. Every other core
and feature crate may depend on it; core never imports feature modules.
`unsafe` is forbidden. Modules, each with one job:

- `registry`: each key's `SettingSpec`: type, default, bound, apply
  class, tunable or not, secret or not, help text and manual page.
- `section`: the `settings_section!` macro (`macro_rules!`). One
  declaration gives both the typed struct and its `SettingSpec` list.
- `bootstrap`: profile choice and layers through figment, keeping each
  value's source.
- `check`: collects every problem before reporting; cross-key rules;
  change rules for agents and the tuner.
- `store`: the `SettingsStore` trait and its PostgreSQL version.
- `snapshot`: `Snapshot` and `SnapshotHandle`.
- `watch`: notices a new active version.
- `history`: diffs, reverts and linked groups.
- `secret`: the `SecretSource` trait and its file version.
- `error`: `SettingsError`, one thiserror enum with SUT codes.
- `api` and `cli`: thin adapters; no rules live in them.

Swappable parts are traits, each with a test fake in the testing kit
crate pariksha ("test"):

| Trait | Production | Test fake |
| --- | --- | --- |
| `SettingsStore` | PostgreSQL through sqlx | in memory, failures injected |
| `SecretSource` | files only the service account can read | a map |
| `Clock` (shared) | system clock | a hand-moved clock |
| `ActivationNotifier` | an outbox event | a channel |

The composition root builds each once and passes it as an
`Arc<dyn Trait>` field of the application state.

### Declaring settings

```rust
settings_section! {
    /// Linking limits (setu, "bridge").
    pub struct LinkLimits in "linking.thread" {
        max_links: u32 = 5, bound 1..=20, apply between_jobs,
            help "Most content links per thread or resource";
        weight_title: f64 = 2.0, bound 0.1..=10.0, tunable step 5%,
            positive, help "Weight of title matches";
    }
}
```

1. The macro expands to a serde and garde struct and a
   `const SPECS: &[SettingSpec]`. Two sections with one name, or a key
   declared twice, stop start-up naming both owners.
2. A declaration may carry: type (bool, integer, float, text from a
   fixed list, duration, byte size, or a list of these); bound;
   `positive` (never zero or below: such a weight can never be turned
   off); `tunable` with its step rule; apply class; `secret`;
   `owner_only` (a setting in the never-changeable class of spec 0020,
   rule 5); `route_out` (the setting can let site text leave the PC,
   and so must also be `owner_only`); and
   cross-key rules as small named functions, such as "chunk overlap is
   below chunk size" and "chunk size is at least 1,024".

### Bootstrap settings and profiles

1. The profile comes from `SUTRADHAR_PROFILE`; the installed service
   sets `prod`. Layers, lowest first, each value keeping its source:
   1. defaults compiled from the declarations;
   2. `/etc/sutradhar/sutradhar.toml`, if present;
   3. `/etc/sutradhar/sutradhar.<profile>.toml`, which must exist for
      any profile but `prod`;
   4. environment variables `SUTRADHAR_<SECTION>__<KEY>`, for bootstrap
      keys only; they never override a runtime setting;
   5. secrets, read only from the files their declarations name.
2. Under `test`, the database must be the declared test database host,
   with a name starting `test_`. Anything else stops start-up before
   any connection.
3. Under `prod`, debugging switches such as the Tokio task inspector are
   refused; it exists in development builds only.
4. Bootstrap files are not reloaded while running; a changed file needs
   a restart.

### Runtime settings and versions

1. A version's resolved document is: compiled defaults, then the
   preset, then the site's own overrides. A version stores the preset
   name, the overrides, the full resolved document and its BLAKE3 hash,
   so a job can be reproduced after an upgrade changes a default.
2. A change has three steps, all through the management API:
   1. Draft: the caller sends the keys, a reason, and optionally
      benefits, drawbacks, evidence links and a linked group. The
      checker returns every problem at once, or the diff: each key's old
      and new value, apply class and whether a restart is needed.
      Drafts are not stored, and a draft never activates on its own.
   2. Activate: one transaction inserts the version, its change rows and
      the `settings.version_activated` outbox event, and moves the
      pointer only if it still names the draft's parent. If another
      version won, the caller sees the new diff.
   3. Deliver: each running process's watcher learns of the new version
      from the event, or by reading the pointer every 2 seconds. It
      loads the version, checks it again against its own registry,
      builds a snapshot and swaps it in.
3. Change rules, applied before any write:
   - Every key is declared, of the right type and inside its bound.
   - A `positive` key is never zero or below.
   - Actor `agent`: refused unless the `ConfirmationCheck` trait
     confirms that the draft's `instruction_ref` names the owner's live
     confirmation of this plan, unexpired and not used before. vinyas
     declares the trait, with a fake in pariksha for tests; the actions
     module implements it (spec 0020, rule 5), so vinyas depends on no
     other module. The confirmation is used up in the same transaction
     as the write, so a second draft naming it is refused and a refused
     draft leaves it unspent. lekha, the audit record, only records
     it.
   - Actor `agent`: refused with SUT-120 when the resolved diff, after
     reverts and presets are expanded, touches any `owner_only` key,
     even with a live confirmation.
   - Actor `tuner`: only `tunable` keys, inside their bounds; continuous
     values move by at most 5% of the current value a version, discrete
     values by their declared step. The tuner never turns a module or
     method off.
   - Actor `system`: only an upgrade's default changes and reverts the
     owner's rules ask for, each labelled so.
   - Secret keys never appear in a draft.
   - A governed setting needs its approval; without it, the screen
     names the requirement and evidence still needed and does not
     activate. An administrator's settings permission does not cover a
     protected requirement or another site's limits.
   - Unsafe combined resource totals are refused.
4. A refusal keeps the last valid active version. Values are never
   silently clamped or guessed.

### Snapshots and delivery

1. `Snapshot` holds the version ID, its hash and each section as its
   typed struct. `snapshot.get::<T>()` returns a reference with no lock
   and no allocation.
2. `SnapshotHandle` wraps `tokio::sync::watch` holding `Arc<Snapshot>`.
   Readers clone the `Arc`; the watcher replaces it; the old snapshot is
   freed when its last reader lets go.
3. A job takes its snapshot when claimed and keeps it to the end; the
   job row records the version ID. A job may run for days on an old
   version; its snapshot is one of those held at once (see Limits), and
   the settings screen shows which jobs hold which snapshot.
4. `live` keys, such as the backlog pause switch or the log level, are
   read through the handle each time. A key may be `live` only if its
   declaration says why that is safe mid-job.
5. `restart` keys are saved and shown at once but take effect only on
   the next start. The service reports the version it runs and the keys
   waiting for a restart. A "Restart now" button makes it finish its
   current batch and exit cleanly, and systemd starts it again. Nothing
   restarts without a person.
6. If the service cannot use a new version (for example, an older build
   meets an unknown key), it keeps its snapshot, reports "not applied:
   unknown key" with SUT-118, and readiness warns. A downgrade never
   silently reinterprets a key: an older build reads a key only as the
   version that wrote it means it, or refuses it.
7. Hot paths read typed snapshots only: no database read or settings
   lookup per token, word or maths step.

### History, reverts and presets

1. History is append-only: no version, change row or result is edited
   or deleted while Sutradhar is installed. History is never trimmed.
2. Revert to version N makes a new version whose overrides equal N's,
   with reason "revert to N". Reverting a linked group restores only
   that group's keys. A revert to an older schema's version is checked
   against today's declarations first.
3. Results: ganit jobs compute before-and-after figures, and Rust code
   records them on a version: measure, window, value before and after,
   and its confidence interval. Until then the version shows "results
   not yet measured".
4. Choosing a preset makes a version that sets the preset name and keeps
   the site's overrides, after showing which overrides now differ.
5. On an upgrade, a default the site never changed takes the new value
   in a `system` version labelled "default changed by upgrade", which
   one revert undoes. An overridden key keeps the site's value. A preset
   value outside a bound tightened by an upgrade is clamped by that
   `system` version, which says so.

### Data

All tables are in Sutradhar's own PostgreSQL, schema `vinyas`, with
English names, times as `timestamptz` in UTC and JSON as `jsonb`. Every
table carries the site ID.

`settings_version`, one row per version, never updated:

| Column | Type | Meaning |
| --- | --- | --- |
| `id` | bigint, identity | the version number |
| `parent_id` | bigint, null | the version it was made from |
| `schema_version` | integer | the registry version that wrote it |
| `code_version` | text | the release that wrote it |
| `preset` | text, checked | `balanced` or `aggressive` |
| `overrides` | jsonb | the site's own values |
| `resolved` | jsonb | every runtime value, fully resolved |
| `resolved_hash` | bytea (32) | BLAKE3 of the resolved document |
| `actor_kind` | text, checked | person, agent, tuner, system |
| `actor_ref` | uuid | a pseudonymous ID; the person is in lekha |
| `instruction_ref` | uuid, null | the owner's confirmation an agent acted on |
| `group_id` | uuid, null | linked changes that revert together |
| `reason` | text | why, in the actor's words |
| `benefits`, `drawbacks` | text, null | what was expected |
| `evidence` | jsonb, null | links to reports |
| `created_at` | timestamptz | when it was saved |

- `settings_change`: `version_id`, `key`, `old_value`, `new_value`
  (jsonb) and `apply_class`; primary key (`version_id`, `key`).
- `settings_pointer`: one row per site with `version_id`,
  `activated_at` and `activated_by`. Activation updates it with
  `WHERE version_id = <parent>`; zero rows changed means another
  activation won.
- `settings_role_state`: per running process, `role`, `instance`,
  `running_version`, `pending_restart` (jsonb list of keys) and
  `seen_at`, written by its watcher every 30 seconds.
- `settings_result`: `version_id`, `measure` (such as `accept_rate`),
  `window_start`, `window_end`, `before_value`, `after_value`, `ci_low`,
  `ci_high`, `method`, `output_version` (the ganit output, if any) and
  `recorded_at`.
- Secrets are never stored in any table. PostgreSQL compresses large
  values itself.

Row-level security is on and forced for every table. The app role may
read and insert versions, changes and results, and may update the
pointer and its own role-state rows. No role has an update or delete
policy on versions, changes or results. With no site set, every policy
returns no rows.

Migrations are numbered SQL files in `migrations/`, applied by sqlx at
start-up under a PostgreSQL advisory lock held for the whole run, so a
second migrator waits and then finds nothing to do. Each migration that
creates a table turns on and forces its row-level security and adds its
policies. The first migration makes version 1, by actor `system`, from
the defaults and the `balanced` preset.

### API, command line and events

Routes on the management API, described in OpenAPI through utoipa. Each
call is checked against the Cedar permission rules and each write is
audited in lekha.

- `GET /api/v1/settings`: each runtime key's value, source (default,
  preset or override), apply class, bound and help. Secrets show only
  "set" or "not set".
- `GET /api/v1/settings/schema`: every declaration, for forms.
- `POST /api/v1/settings/drafts`: check a change; writes nothing.
- `POST /api/v1/settings/versions`: activate a draft with its parent;
  409 if the parent is no longer active.
- `GET /api/v1/settings/versions?after=<id>&limit=<n>`: history by ID
  cursor, newest first.
- `GET /api/v1/settings/versions/{id}` and
  `GET /api/v1/settings/versions/{id}/diff?against=<id>`.
- `POST /api/v1/settings/versions/{id}/revert`, optionally with a group.
- `GET /api/v1/settings/roles`: each process's running version and keys
  waiting for a restart.
- `POST /api/v1/settings/versions/{id}/results`: the tuner and the
  results feature only, by their service identity.

`sutra vinyas`, a client of the same API: `check [--profile <name>]`
(checks bootstrap files and the active version without starting
anything); `show [<key>]` with `--sources`; `set <key>=<value>...
--reason "<text>"` (shows a draft, then asks to activate); `history`;
`diff <a> <b>`; `revert <version> [--group <id>]`; and `roles`.

Outbox events, with IDs and counts only:
`settings.version_activated` (version, parent, keys changed, restart
needed), `settings.version_rejected_by_role` (role, version, SUT code)
and `settings.restart_pending` (role, keys waiting).

### Errors

Each has a SUT code, a plain-English message and a manual page:

| Code | Problem |
|---|---|
| SUT-101 | Key not declared, with a "did you mean" suggestion (strsim) |
| SUT-102 | Wrong type, such as the text "false" for a boolean |
| SUT-103 | Value outside its bound |
| SUT-104 | A cross-key rule failed; both keys named |
| SUT-105 | Profile file missing |
| SUT-106 | A secret in a file, variable or draft; key or file named, never the value |
| SUT-107 | A test profile points at a database that is not the test one |
| SUT-108 | Agent change with no live owner confirmation of this plan |
| SUT-109 | Tuner change outside its bound or bigger than its step |
| SUT-110 | A `positive` key would be zero or below |
| SUT-111 | The active version changed since the draft |
| SUT-112 | Settings database unreachable at start-up |
| SUT-113 | Stored resolved document does not match its hash |
| SUT-114 | A needed secret file is missing or unreadable |
| SUT-115 | Two modules declare the same section or key |
| SUT-116 | A duration or byte size has no unit, such as `30` or `5.6` |
| SUT-117 | A debugging switch set under `prod` |
| SUT-118 | This process cannot use the active version; it keeps its own |
| SUT-119 | An environment variable names a runtime setting |
| SUT-120 | Agent change touches an `owner_only` key |

- Start-up collects every problem from every layer before stopping.
- Database unreachable at start-up: retry with backoff (backon) up to
  the start timeout, then exit so systemd tries again; never run on
  guessed settings.
- Database lost while running: keep the current snapshot and keep
  working; activations fail with a plain message; the watcher retries.
- A version failing its hash check (SUT-113) is refused; the snapshot
  is kept, a health alert is raised and readiness reports not ready for
  changes.
- A crash between activation and delivery loses nothing: the pointer is
  the truth and every watcher reads it on start.
- A database restored from a backup older than the running version is
  never taken silently; the administrator confirms, recorded as a
  `system` version.
- Edge cases: `0`, `yes` or "false" for a boolean are refused; `5.0` as
  an integer or `5` as a float is accepted only where lossless; a key
  removed in a newer release stays in history and is ignored on load
  with a note; the profile file beats the base file and `--sources`
  shows both; an empty profile file is allowed; versions are ordered by
  ID, never by time; a mode setting is a ceiling, and version 1 has no
  automatic mode.

### Security and privacy

- Secrets (database passwords, signing keys, provider keys) are declared
  `secret`, read only from files, held as `secrecy::SecretString`, and
  never logged or put in a version, diff, export or diagnostic bundle.
- A pasted secret is refused wherever it appears, by key name and by
  shape: private-key headers, known provider key prefixes and long
  high-randomness strings.
- `actor_ref` is pseudonymous; erasing a person leaves the history whole
  and anonymous.
- `reason`, `benefits` and `drawbacks` are shown only in the app, never
  sent to telemetry.
- Removal offers an export of the settings history and asks whether to
  keep or delete it.

### Nothing hardcoded

1. Every product, operational and policy choice is a declared setting:
   endpoints, paths, ports, schedules, timeouts, retries, limits,
   thresholds, weights, algorithm and model choices, feature modes,
   resource budgets, retention and presentation choices. This covers
   every language, interface, adapter, tool, gate rule and deployment
   part. No second settings store is allowed.
2. Each key, default, range and fixed decision is declared once and
   linked from vinyas. Generated code may embed a default but never
   create its own value or block a permitted change.
3. Each declaration records: key, owning module, type, units, default,
   allowed range or values, dependencies and the reason for any fixed
   bound; meaning, effects, risks, resource cost, one example and the
   manual page; site or install scope, who may read, propose and
   activate, and whether owner approval or a release is needed; source,
   version, activation timing, restart need, deprecation and safe
   failure; and secret handling.
4. Privacy, permissions, content safety, trust, required checks, proof
   bounds, quality thresholds and release protections keep their own
   approval rules. A configurable gate cannot approve its own loosening.
5. Maths definitions, protocol syntax, schema versions and test
   expectations have named, versioned contract owners and are never
   runtime switches. A test's expected answer never follows the current
   setting in a way that stops the test failing.
6. Every interface calls the same permission-checked Rust code: no
   copied rules, hidden environment overrides or direct database
   writes. Help sits beside each control; tooltips work with keyboard
   and touch, and essential help never needs hovering.
7. Known keys: `trash.keep_days`, fixed at 90, and the `agency.*` keys.
   Further keys join as each feature's specification declares them.
8. Nirikshak rules 4, 5 and 10 (spec 0010) link declarations, reads and
   environment inputs to this inventory; reviews judge choices a parser
   cannot. Missing or unsure analysis is never a pass.

## Limits

- Memory per process for this part, within the service slice of
  AGENTS.md section 5.6:

| Item | Bound |
| --- | --- |
| Registry of declarations (5,000 keys with help) | 1 MiB |
| One snapshot (typed sections and resolved document) | 2 MiB |
| Snapshots held at once | at most 10 |
| Watcher and API buffers (one version in flight) | 2 MiB |
| Total | at most 25 MiB |

- An eleventh snapshot is not built until one is freed; the service
  reports "settings change waiting for long jobs". History lives in the
  database and pages stream by ID cursor.
- Storage: about 2 KiB per change row and at most 2 MiB per resolved
  document; 10,000 versions stay under 2 GB. History is shown on the
  storage panel.
- Speed targets:
  - load and check 5,000 keys in at most 300 ms (p99), so a restarted
    service is ready within its 60-second recovery;
  - `snapshot.get` at most 50 ns median, with no allocation;
  - a draft of 50 changes checked in at most 200 ms (p99);
  - activation to every process's new snapshot within 5 s (p99),
    including the 2-second poll; normally under 1 s with the event;
  - the first page of 10,000 versions in at most 150 ms (p99).
- Changing a resource setting never escapes the memory, concurrency,
  deadline or no-swap limits.
- Telemetry: metric names start `sutradhar_`, carry their unit, counters
  end `_total`, and labels have few values, never IDs, values or people.
  Metrics, spans and logs carry only version IDs, key names, counts,
  durations and codes, never values.
  - Metrics: `sutradhar_settings_active_version`,
    `sutradhar_settings_activations_total{outcome}` (activated,
    conflict, refused), `sutradhar_settings_check_failures_total{code}`,
    `sutradhar_settings_propagation_seconds`,
    `sutradhar_settings_pending_restart_keys`,
    `sutradhar_settings_snapshots_held`, and `sutradhar_build_info`
    and `sutradhar_settings_info{preset}` set to 1.
  - Spans: `vinyas.bootstrap`, `vinyas.check`, `vinyas.activate` and
    `vinyas.deliver`, with `settings.version_id`,
    `settings.keys_changed` and `error.code`.
  - Logs (JSON): `settings_loaded`, `settings_activated`,
    `settings_refused` and `settings_restart_pending`.
  - Alerts: a process that has not applied the active version within 5
    minutes; a version refused by any process; keys waiting for a
    restart for more than a day.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Tests are in `crates/vinyas/`, on made-up data only, written first and
seen to fail for the right reason. Coverage is 100% for lines, branches
and MC/DC, and every mutant is killed.

- AT-CF01 (`tests/check.rs`): a profile file with an unknown key, a
  wrong type and an over-bound value gives SUT-101, 102 and 103, each
  with file and line, and nothing starts.
- AT-CF02 (`tests/check.rs`): `linking.thred.max_links` gets SUT-101
  suggesting `linking.thread.max_links`.
- AT-CF03 (`tests/bootstrap.rs`): `SUTRADHAR_DRISHTI__METRICS_ENABLED=False`
  gives SUT-102 naming the variable and saying only `true` or `false`.
- AT-CF04 (`tests/bootstrap.rs`): `dev` with no `sutradhar.dev.toml`
  gives SUT-105; `prod` with no profile file starts on defaults and base.
- AT-CF05 (`tests/secrets.rs`): a password in `sutradhar.toml` gives
  SUT-106 naming key and file; the value is in no output; start-up stops.
- AT-CF06 (`tests/prop_secrets.rs`, property): secrets planted in random
  string values are all refused, and none of their bytes appear in
  errors.
- AT-CF07 (`tests/profiles.rs`): `test` with a non-test database host
  gives SUT-107 before any connection.
- AT-CF08 (`tests/check.rs`): `30` and `5.6` give SUT-116; `30s` reads
  as 30 seconds, `5734 MiB` as 6,012,534,784 bytes, and `5.6 GB` as
  5,600,000,000 bytes (`GB` is always decimal).
- AT-CF09 (`tests/activate.rs`): from version 41, one change makes 42
  with its reason and parent, and the pointer names 42.
- AT-CF10 (`tests/loom_activate.rs`, loom): two activations from one
  parent; exactly one wins, the other gets SUT-111 and the new diff.
- AT-CF11 (`tests/stress_activate.rs`): eight clients for 30 seconds on
  test PostgreSQL leave one unbroken parent chain, no gaps, no twins.
- AT-CF12 (`tests/snapshot.rs`): a job on 41 reads 41 to its end while
  new jobs read 42; the job's record says 41.
- AT-CF13 (`tests/loom_snapshot.rs`, loom): readers see a whole old or
  whole new snapshot, never a mix, and the old one is freed once.
- AT-CF14 (`tests/snapshot.rs`): with ten old snapshots held, an
  eleventh version waits with its message and applies when one frees.
- AT-CF15 (`tests/apply_class.rs`): a `restart` key is saved, the old
  value is kept and reported as waiting, and the new one is used after
  restart.
- AT-CF16 (`tests/apply_class.rs`): a `live` key is seen at the next
  read, within the propagation target.
- AT-CF17 (`tests/rules.rs`): an agent draft with no owner
  confirmation, or with one that expired, was used before or names
  another plan, gets SUT-108; with a live one, it activates and shows
  the confirmation.
- AT-CF18 (`tests/prop_tuner.rs`, property): every accepted tuner change
  stays in bound and within 5%; every other gets SUT-109.
- AT-CF19 (`tests/prop_positive.rs`, property): no accepted version holds
  zero or less for a `positive` key.
- AT-CF20 (`tests/history.rs`): reverting to 41 from 45 makes 46 with
  41's overrides and "revert to 41"; 40 to 45 are unchanged byte for
  byte.
- AT-CF21 (`tests/history.rs`): reverting group G restores only its
  three keys.
- AT-CF22 (`tests/telemetry.rs`): for every key type, metrics, spans and
  logs hold no value.
- AT-CF23 (`tests/upgrade.rs`): from every earlier released schema, the
  active hash is unchanged and new keys take defaults in a `system`
  version.
- AT-CF24 (`tests/upgrade.rs`): an older build meeting an unknown key
  keeps its snapshot, reports SUT-118 and readiness warns.
- AT-CF25 (`tests/chaos_settings.rs`, release tier): the database
  stopped for 60 seconds; the service keeps working, activations fail
  plainly, nothing else fails.
- AT-CF26 (`tests/chaos_settings.rs`, release tier): the service killed
  just after the pointer moves runs the new version within the target.
- AT-CF27 (`tests/snap_check.rs`): `sutra vinyas check` output on the
  wrong-file set matches the stored insta snapshot.
- AT-CF28 (`tests/load_activation.rs`, at most 2 minutes): 200
  activations back to back reach four processes within 5 s at p99,
  none missed.
- AT-CF29 (`src/check/proofs.rs`, Kani): for any current and proposed
  value of a bounded, tunable, positive key, an accepted change is in
  bound, above zero and within 5%.
- AT-CF30 (`tests/rls.rs`, SQL against test PostgreSQL): the app role
  cannot update or delete a version, change or result; with no site
  set it sees no rows; every table has forced row-level security.
- AT-CF31 (`tests/bootstrap.rs`): a saved `max_links = 3` and
  `SUTRADHAR_LINKING__THREAD__MAX_LINKS=7` give SUT-119; the service
  runs on 3 and `--sources` names the saved version.
- AT-CF32 (`tests/restart.rs`): "Restart now" finishes the current
  chunk, exits with code 0 and restarts on the new value; nothing
  restarts before it is pressed.
- AT-CF33 (`tests/upgrade.rs`): an untouched key takes the new default
  in a "default changed by upgrade" version; an overridden key keeps
  its value; one revert restores the old default.
- AT-CF34 (`tests/prop_history.rs`, property): after random activations,
  reverts, upgrades, retention sweeps and restarts, every version is
  present and unchanged.
- AT-CF35: an inventory of every configurable choice across languages,
  scripts, images and interfaces; a planted undeclared choice is
  refused; every declaration has owner, help, scope, source and class.
- AT-CF36: non-default values through each interface change behaviour,
  and every interface shows the same version; generated defaults never
  override an active value.
- AT-CF37: a governed change previews; missing or stale approval blocks
  it; no agent, variable or tuner bypasses safety, trust, privacy,
  permissions or the publishing gate.
- AT-CF38: wrong units, bounds, dependencies, concurrent changes,
  mismatched processes and excessive resource totals all refuse,
  keeping the active version and job snapshots.
- AT-CF39: history, job-to-version binding, secret exclusion, and help
  reachable by keyboard, touch and without hovering, restart keys
  included.
- AT-CF40 (`tests/rules.rs`): a confirmed agent draft that sets
  `coach.ai.order`, a Bedrock spending cap, the MCP grading switch, a
  backup route or `actions.confirm_wait`, directly, by a revert or by a
  preset, gets SUT-120; the owner's own draft of the same keys passes.
  A test fails when any `route_out` key lacks `owner_only`, and
  compares the `route_out` keys and the `owner_only` keys each both
  ways with a reviewed list kept in the test.
- Benchmarks `bench_bootstrap_5000`, `bench_snapshot_get`,
  `bench_draft_check_50` and `bench_history_page`, and the load test
  `load_activation`, hold the speed targets under the 5% median rule.
- Checks of library behaviour: figment keeps each value's source;
  bytesize reads `GB` as 10^9 and `GiB` as 2^30 and refuses a bare
  number; humantime-serde accepts `30s`, `5m` and `1h30m` and refuses a
  bare number; garde field paths map to dotted keys; `jsonb` and the
  toml crate keep `5` and `5.0` apart; sqlx's migrator holds its lock
  for the whole run; systemd restarts the service after a clean exit;
  Kani runs AT-CF29 within the gate's time.

## Changes from the source

- Profiles are `dev`, `test` and `prod`; the old `bench` and `lab`
  profiles are dropped.
- The service is one systemd service, not containers; "Restart now"
  relies on systemd, and migrations run at start-up, not in a separate
  container.
- The old settings pages are Dioxus app screens built from SDS, using
  the management API on the local socket.
- Permissions are the Cedar rules, not the old action layer.
- Events are outbox events of Sutradhar Boot.
- Tables carry the site ID and use one app role, per AGENTS.md
  section 5.2, in place of one login per container.
- The rule on the forum's own database is dropped: the service never
  reaches it.
- Secrets come from files only the service account can read.
- A failed hash check raises a health alert, not a gate task.
- The three-run median ratchet becomes the 5% median rule of AGENTS.md
  section 6 rule 8; the 20x goal is kept (spec 0007, "The 20x goal").
- The old 5.6 GB and 85% memory figures are replaced by AGENTS.md
  section 5.6.
- MC/DC is measured by our own tool, and every mutant is killed.
- Nightly chaos tests run in the release tier.

Source: `docs/specifications/platform/settings.md`,
`platform/settings-2.md`, `platform/settings-3.md`,
`platform/settings-4.md` and `platform/settings-5.md` in the archive
`archive-2026-11-08-old-repo`.
