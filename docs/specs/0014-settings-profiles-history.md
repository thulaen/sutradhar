# 0014: Settings, profiles and history

Settings, profiles and history is vinyas ("arrangement"), the part of
Behemoth Boot that holds every setting, checks each change, applies it
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
  specification); optimisation and before-and-after maths (spec 0077 and
  ganit); who may change which
  section (the Cedar permission rules); module switches and modes (the
  modules specification); how jobs take a snapshot (the jobs
  specification).

### Crate layout

The crate depends only on the shared base types crate. Every other core
and feature crate may depend on it; core never imports feature modules.
`unsafe` is forbidden. Modules, each with one job:

- `registry`: each key's `SettingSpec`: type, default, bound, apply
  class, secret or not, help text and manual page. Spec 0077 owns the
  separate Optuna parameter registry; a setting is not tunable by declaration
  here.
- `section`: the `settings_section!` macro (`macro_rules!`). One
  declaration gives both the typed struct and its `SettingSpec` list.
- `bootstrap`: profile choice and layers through figment, keeping each
  value's source.
- `check`: collects every problem before reporting; cross-key rules;
  change rules for agents and the spec 0077 release controller.
- `store`: the `SettingsStore` trait and its PostgreSQL version.
- `snapshot`: `Snapshot` and `SnapshotHandle`.
- `watch`: notices a new active version.
- `history`: diffs, reverts and linked groups.
- `secret`: the `SecretSource` trait and its file version.
- `error`: `SettingsError`, one thiserror enum with SUT codes.
- `api` and `cli`: thin adapters; no rules live in them.

Swappable parts are traits. A fake used by one crate lives in that
crate's tests; a shared fake lives in `sutradhar-test-support`, a
development dependency only. The production evaluation crate pariksha
(spec 0056) holds no platform fakes:

| Trait | Production | Test fake |
| --- | --- | --- |
| `SettingsStore` | PostgreSQL through sqlx | in memory, failures injected |
| `SecretSource` | files only the service account can read | a map |
| `Clock` (shared) | system clock | a hand-moved clock |
| `ActivationNotifier` | in-process wake after the durable settings inbox commit | a channel |

The composition root builds each once and passes it as an
`Arc<dyn Trait>` field of the application state.

### Declaring settings

```rust
settings_section! {
    /// Linking limits (setu, "bridge").
    pub struct LinkLimits in "linking.thread" {
        max_links: u32 = 5, bound 1..=20, apply between_jobs,
            help "Most content links per thread or resource";
        weight_title: f64 = 2.0, bound 0.1..=10.0,
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
   off); apply class; `secret`;
   `owner_only` (a setting in the never-changeable class of spec 0020,
   rule 5); `route_out` (the setting can let site text leave the PC,
   and so must also be `owner_only`); and
   cross-key rules as small named functions, such as "chunk overlap is
   below chunk size" and "chunk size is at least 1,024".
3. The shared base type `CrawlerDisplayName` is admitted only for
   the 0152 crawler name key. Its JSON value is a string. The type
   uses the validation and canonical form in [0152][crawler-name].
   Reject a different JSON type with SUT-102 and an invalid name
   with SUT-103. The error names the key and rule without the
   rejected text. This type adds no unrestricted text declaration.

[crawler-name]: 0152-live-backlink-crawler-profile.md

Discovery: the registry provides a safe description of every
declared setting. It gives the stable ID, purpose, type, allowed
values, scope, owner and change route through the same schema in
the app, CLI and MCP. For an `owner_only` setting it describes the
owner's route or the rule that fixes it. A description carries no
site's current value, secret presence, private path or active
control. Reading a description gives no permission to read or
change a site's settings. Agent change requests follow 0020 rule 5.

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
   2. Activate: the settings service checks the current actor and
      policy in the one site transaction. Bind plan digest, site,
      domain, epoch and parent. Recheck scope, expiry and use of
      any required live confirmation at commit. An actor whose
      grant changed must pass current policy. General system
      principals have no right to advance settings. Insert the
      version and change rows, move the pointer and write
      `settings.version_activated` to the product outbox (spec 0019).
      It also consumes any required confirmation. All effects commit
      together. Losing the pointer comparison or failing an insert
      leaves them all unchanged. The caller can get the current diff.
      A settings event has no model or champion authority.
   3. Deliver: the settings consumer records the latest validated
      activation position and its inbox receipt in one transaction.
      A watcher wakes after that commit. It reconciles against the
      current pointer on start and every 2 seconds. A wake only asks
      the watcher to check; it cannot set values or replace the
      pointer. The rules below govern each process's snapshot.
3. Change rules, applied before any write:
   - Every key is declared, of the right type and inside its bound.
   - A `positive` key is never zero or below.
   - Actor `agent`: refused unless the `ConfirmationCheck` trait
     confirms that the draft's `instruction_ref` names the owner's live
     confirmation of this plan, unexpired and not used before. vinyas
     declares the trait, with a fake in `sutradhar-test-support` for
     tests; the actions
     module implements it (spec 0020, rule 5), so vinyas depends on no
     other module. The confirmation is used up in the same transaction
     as the write, so a second draft naming it is refused and a refused
     draft leaves it unspent. lekha, the audit record, only records
     it.
   - Actor `agent`: refused with SUT-120 when the resolved diff, after
     reverts and presets are expanded, touches any `owner_only` key,
     even with a live confirmation.
   - Actor `release`: only the exact resolved configuration named by the
     signed spec 0077 candidate manifest, as part of that specification's
     atomic activation. Expected-champion and policy checks pass before
     the write, and activation cannot report success until settings and
     every other artefact match the new champion. The resolved settings,
     candidate and expected champion digests must match. This actor cannot
     choose a value, round a parameter, turn a module or method off, or
     activate a diagnostic ablation.
   - Actor `system`: the initial settings of a site under the
     authorised 0030 provisioning step, an upgrade's default changes
     reverts the owner's rules ask for, and the authorised restore below.
     Label each accordingly.
   - Secret keys never appear in a draft.
   - A governed setting needs its approval; without it, the screen
     names the requirement and evidence still needed and does not
     activate. An administrator's settings permission does not cover a
     protected requirement or another site's limits.
   - Unsafe combined resource totals are refused.
4. A refusal keeps the last valid active version. Values are never
   silently clamped or guessed.

### Inherit a site setting from a reset plan

The settings service derives the internal operation
`InheritSiteOverride { key: SettingKey }` from an owner-confirmed
Reset plan in spec 0030. Its only field is the key's declared stable
registry ID. The plan names one site and one settings domain. Require
the owner's current Cedar authority for that site and live
confirmation on an already admitted human route under specs 0030 and
0020. An installation scope alone supplies no site authority.
The service constructs the activation operation after admission.
The general settings API, CLI and MCP cannot submit the raw type.
An authorised interactive human Reset uses the same owner and plan checks. Agent
and script inputs gain no reset or settings authority.

Select only declared ordinary runtime keys with a present override
and a current owning change route. A bootstrap, secret, protected,
governed, owner-only or outbound-route key is ineligible. The preview
shows each selected key's current and inherited effective values,
their sources and the ordinary apply class. Bind the exact key set,
current registry and preset, site, domain, active parent and epoch to
the existing confirmed plan. The owner confirms those changes and
the exact eligible cache identities, if any. Recheck the plan,
authority and settings preconditions at activation.

Remove the selected keys from the new version's `overrides` map.
Preserve the preset and every other override. Resolve the new
document under the usual order. A key inherits its current preset
value if that preset supplies one, or the compiled declaration's
default otherwise. Setting a key to a null or to the computed
effective value never expresses this operation. It must be absent
from the new override map so later preset and declaration changes
can resolve normally.

The reset accepts only inheritance operations for its settings
changes. Reject the whole plan when it mixes inheritance with a
value assignment, includes a value or duplicate key, contains any
ineligible selection or lacks current scope, proof or confirmation.
Validate the whole reset plan before any cache or settings effect.
Reject a changed active parent under SUT-111. Use the existing
installation lease in spec 0031 and the activation transaction above.
In a combined reset, begin cache cleanup only after the exact
settings activation is verified. Specs 0030 and 0031 own
authorisation and reconciliation of each cache effect.

Record the activation as a new `person` version. Append one
`settings_change` row per removed override even when the resolved
value stays equal. Its old and new values are the typed effective
values. History and diff derive override presence and sources from
the immutable parent and new version, and show the change from
override to preset or default. Equal resolved hashes still get a
new version and activation sequence. Prior versions and results remain.
Consume the bound confirmation, publish the pointer and record the
outbox event in the existing transaction. An exact replay
reconciles the recorded activation once. An unproved result holds
the remaining cleanup for the original reset plan.

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
   waiting for a restart. "Restart now" is an owner action naming
   the affected role, process and desired position. The supervisor
   of spec 0030 drains and restarts it under its lifecycle checks.
   A changed position or replaced process needs new confirmation.
6. If the service cannot use a new version (for example, an older build
   meets an unknown key), it keeps its snapshot, reports "not applied:
   unknown key" with SUT-118, and readiness warns. A downgrade never
   silently reinterprets a key: an older build reads a key only as the
   version that wrote it means it, or refuses it.
7. Hot paths read typed snapshots only: no database read or settings
   lookup per token, word or maths step.
8. Events use the strict site and settings-domain ordering key.
   Each immutable version and the pointer carry a settings epoch
   and activation sequence. Assign the next sequence in the
   activation transaction. A repeat
   or older position cannot replace a newer desired version. The
   consumer verifies the site, version and committed position
   before advancing. A gap or mismatched digest requests bounded
   pointer reconciliation and never applies the event body.
9. A watcher verifies its committed version, hash and role registry.
   Build the effective snapshot with new live values and the
   process's current restart values. Validate the actual combination,
   including cross-key invariants and resource bounds. If invalid,
   hold the last complete effective snapshot and show refusal or
   restart needed. Do not publish an unsafe mix.
10. Serialise the swap per site, domain and role instance against
    activation and process replacement. Recheck the desired position
    under the settings activation lock. Compare the local effective
    generation when swapping the complete snapshot. A late build
    cannot replace a newer snapshot. Prepare outside the lock.
11. Record role outcomes after the swap with conditional instance,
    desired position and attempt checks. A stale or replaced caller
    cannot overwrite a newer outcome. Report desired and effective
    positions separately. A database acknowledgement failure does
    not undo a completed swap. Reconcile from the actual process
    and current pointer. An inbox receipt proves delivery only.
    An old snapshot gives no authority to use withdrawn permissions.

### History, reverts and presets

1. Settings history is append-only during ordinary operation for an
   enrolled site. Routine settings changes do not edit, delete or trim
   existing versions, change rows or results. Apply current 0032 erasure
   before using restored data. Specs 0030 and 0102 govern removal of
   settings history in the exact approved site or installation scope.
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
| `actor_kind` | text, checked | person, agent, release, system |
| `actor_ref` | uuid | a pseudonymous ID; the person is in lekha |
| `instruction_ref` | uuid, null | the owner's confirmation an agent acted on |
| `group_id` | uuid, null | linked changes that revert together |
| `reason` | text | why, in the actor's words |
| `benefits`, `drawbacks` | text, null | what was expected |
| `evidence` | jsonb, null | links to reports |
| `created_at` | timestamptz | when it was saved |

- `settings_change`: `version_id`, `key`, `old_value`, `new_value`
  (jsonb) and `apply_class`; primary key (`version_id`, `key`).
- `settings_pointer`: one row per site with version, epoch,
  activation sequence, activation time and actor. The parent and
  epoch must match for activation.
- `settings_delivery`: one desired position and version per site.
  The owning durable consumer updates it with its inbox receipt.
- `settings_role_state`: per running process, site, role, instance,
  desired position, effective snapshot identity, running and
  pending restart versions, attempt and outcome sequence, safe
  outcome code and last seen time. Update after each outcome and
  every 30 seconds. Repeated heartbeats create no new outcome.
- `settings_result`: `version_id`, `measure` (such as `accept_rate`),
  `window_start`, `window_end`, `before_value`, `after_value`, `ci_low`,
  `ci_high`, `method`, `output_version` (the ganit output, if any) and
  `recorded_at`.
- Secrets are never stored in any table. PostgreSQL compresses large
  values itself.

Row-level security is on and forced for every table. The app role may
read and insert versions, changes and results, and may update the
pointer and its own role-state rows. Only the authorised first site
provisioning operation may insert that site's first pointer. No role
has an update or delete policy on versions, changes or results.
Without a valid site context, or with installation scope present,
vinyas permits no protected rows or writes under 0108.

Migrations are numbered SQL files in `migrations/`. Apply them through
the 0108 SQLx profile and its dedicated advisory lock. The migrator
creates the vinyas schema and forced site policies. Installation-only
setup and migration history under DR-150 cannot create site settings.

Create the initial settings version during 0030's authorised
provisioning of the exact pending site in its verified native register
generation. Bind that site through 0108 with the restricted app role.
The system actor sets the registered defaults and the `balanced`
preset. Commit the first version and its unique site pointer together
under the existing activation and event rules. The site remains
unavailable to readers until 0030 completes pairing and provisioning.
On retry verify the original site and provisioning intent. Reconcile
the matching initial version and pointer without writing a second
initial version or replacing a later pointer. A missing pointer on an
established site requires recovery. Its absence alone cannot authorise
the first version.

### API, command line and events

Routes on the management API, described in OpenAPI through utoipa. Each
call is checked against the Cedar permission rules and each write is
audited in lekha.

- `GET /api/v1/settings`: each authorised runtime key's value, source
  (default, preset or override), apply class, bound and help. An
  authorised secret status shows only "set" or "not set".
- `GET /api/v1/settings/schema`: the safe registry descriptions for
  forms and discovery. It reads no runtime snapshot.
- `POST /api/v1/settings/drafts`: check a change; writes nothing.
- `POST /api/v1/settings/versions`: activate a draft with its parent;
  409 if the parent is no longer active.
- `GET /api/v1/settings/versions?after=<id>&limit=<n>`: history by ID
  cursor, newest first.
- `GET /api/v1/settings/versions/{id}` and
  `GET /api/v1/settings/versions/{id}/diff?against=<id>`.
- `POST /api/v1/settings/versions/{id}/revert`, optionally with a group.
- `GET /api/v1/settings/roles`: use the current verified process
  instance for each authorised role in the selected site and settings
  domain. Show the committed desired position, verified effective
  snapshot and permitted keys waiting for a restart separately. The
  effective snapshot can retain values from before the desired
  version. If the current effective snapshot cannot be verified, show
  "Effective settings not yet verified". A delivery receipt or a
  replaced instance's outcome cannot prove it. This read does not
  register a role, update its state or request a restart.
- `POST /api/v1/settings/versions/{id}/results`: the spec 0077
  evaluation-evidence writer and the results feature only, by service identity.

For runtime values, sources, history, diffs and role status, derive
the actor and site from the authenticated context. Apply current
Cedar action permission and each declaration's read permission.
Read site settings through the 0108 site scope. Installation
settings use their own admitted route. DR-150 cannot read
vinyas site rows. Check each version, key and role result before
returning it. Project both
versions of a diff under current rights before comparison. A
history cursor must pass the same checks before another page. Keep
existing restrictions on audit and app-only fields. A refused
runtime read gives no value, presence, source or role detail.

An N-1463 reset or separately admitted 0044 settings-file import
can change the desired settings only through its owning activation
route. Read the authorised
committed version and keep desired values, pending restart and
effective values distinct. A prior preview, cached snapshot or
cursor confers no current read authority or evidence of activation.
Recheck permissions before delivery. A changed site or lost grant
cannot release a previous result.

`sutra vinyas`, a client of the same API: `check [--profile <name>]`
(checks bootstrap files and the active version without starting
anything); `show [<key>]` with `--sources`; `set <key>=<value>...
--reason "<text>"` (shows a draft, then asks to activate); `history`;
`diff <a> <b>`; `revert <version> [--group <id>]`; and `roles`.

Use spec 0019 versioned outbox events. `settings.version_activated`
carries version, parent, epoch, sequence, digest and change count.
`settings.version_rejected_by_role` carries role, instance, position
and safe error code. `settings.restart_pending` carries role,
instance, position and count. Role events use the strict site,
domain, role and process-instance key. Assign a monotone outcome
sequence with each new role result. Commit status and any event
together. Dedupe by key, outcome sequence and event type. Changed
bytes for that identity quarantine the event. Role events cannot
change the pointer or request a restart. No values, secrets or
key lists travel in an event.

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
  the start timeout, then report failure to the supervisor. Use the
  bounded restart policy of spec 0030. Never use guessed settings.
- Database lost while running: keep the current snapshot and keep
  working; activations fail with a plain message; the watcher retries.
- A version failing its hash check (SUT-113) is refused; the snapshot
  is kept, a health alert is raised and readiness reports not ready for
  changes.
- A crash between activation and delivery loses nothing: the pointer is
  the truth and every watcher reads it on start.
- A restore from an older backup needs recorded owner approval.
  Under spec 0030 recovery, advance the settings epoch against the
  protected current restore history before publishing an authorised
  `system` version. If that history cannot be verified, withhold the
  affected settings. Reject old-epoch confirmations, events and role
  receipts. Start roles from the restored pointer only after the
  queue and outbox reconcile as in spec 0019. Recheck restored
  protected settings against current authority before use.
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
- `actor_ref` is pseudonymous. Spec 0032 erasure removes the actor
  mapping and applicable personal details. Keep the non-personal
  settings history. A permitted name lookup for that erased actor
  uses the existing "removed member" label.
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
   Spec 0152 owns all eight `seo.backlink` keys, including the five
   numeric limits. Use its types, bounds, `owner_only` declarations,
   `live` apply class and per-run limit fence. The registered
   descriptions use the local crawler controls and the
   [manual's settings scope guide][settings-scope]. The actual site
   values, history and diffs use the same protected site store and
   current Cedar checks as other owner-only settings. Discovery
   gives only the safe descriptions. Track the desired settings
   separately from the verified public card and crawl admission.

The sole [0135 AI fetch weight](0135-ai-fetches-referrals-and-ranking.md) is
   `ranking.ai_fetch_boost_bps`. The 0027 ranking module consumes this site-
   scoped non-secret integer in basis points. Its inclusive range is 0–500,
   and its default is 0 (DR-160). Zero applies no boost. For example, 200
   requests a 2% adjustment of an eligible order key. This key is
   `owner_only`. Show a typed preview and use the current owner's exact 0020
   confirmation before committing the settings version and its old and new
   values. An agent cannot activate it. Native and MCP descriptions grant no
   change authority. Current Cedar read rights govern disclosure. Follow the
   manual’s site administration page for the native AI traffic view.

   Use the existing `between_jobs` apply class. At claim, pin the
   committed settings version and weight for the ranking batch under
   Snapshots and delivery. Record that version with the actual applied
   weight. The pin gives no authority over current eligibility, fact
   revocation or the final-release fence. A batch that encounters an
   invalid or unadmitted weight uses the 0135 base-order outcome.
   The declaration grants no collection, fetch proof or signal
   activation. The 0135 and
   0077 admission evidence still governs production use.
8. Nirikshak rules 4, 5 and 10 (spec 0010) link declarations, reads and
   environment inputs to this inventory; reviews judge choices a parser
   cannot. Missing or unsure analysis is never a pass.

[settings-scope]: ../../manual/configuration.md#check-the-scope-of-a-setting

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
seen to fail for the right reason. Coverage and mutation obligations follow
ADR-0020: its admitted strict core requires 100% line, branch and MC/DC
coverage and every killable mutant killed, with equivalent mutants proved
and independently reviewed. Other code keeps the risk-appropriate coverage,
property tests and proofs required by its owning specification; the strict
core targets do not apply universally.

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
- AT-CF-OUTBOX: fail after each activation write, including outbox
  insert and confirmation use. Rollback exposes no version,
  pointer move, consumed confirmation or event. Repeat a committed
  event and check one desired position.
- AT-CF-DELIVERY: deliver repeats, later generations, a dead letter,
  an older process acknowledgement and an old-epoch replay after
  restore. Race a slow build with a newer activation and a live
  key with a coupled restart key. Fail after swap before role
  status. A late acknowledgement cannot become a new outcome.
  Reconcile the current site and role once. Check that refusal,
  pending restart and applied state remain distinct and no event
  exposes a value or gives permission to restart. Revoke a grant
  after draft and try reusing confirmation on a different parent
  or epoch. Check no committed change. In a separate restore case,
  stop after reserving the new epoch and before publishing the
  settings pointer. On restart, resume the same prepared restore
  with current approval and publish once under that epoch. Repeat
  from the interruption with the original owner's authority
  withdrawn, with unproved current authority, and with the same
  backup offered to a different site or settings domain. Each held
  case leaves the current pointer intact and starts no role from
  the staged settings. The uncommitted restore creates no success
  receipt or settings event. In a separate restore case, use a synthetic
  backup whose `coach.ai.order` preferred the owner's same-site MCP
  wording route. Revoke that assistant after the backup under 0020
  F-1302. Keep the current minimum revocation epoch and the owner's
  separate approval for the exact 0030 restore. Restore the eligible
  order under a new settings epoch and start the role after
  reconciliation. Check that the authorised settings read shows the
  committed desired order separately from the current verified role
  snapshot. Neither proves that the assistant can accept sampling.
  Use the restored wording preference for an eligible local draft.
  The revoked assistant remains unavailable. Its old token gets no
  settings detail and the Coach sends it no sampling request or text.
  A restored approval, prior session or old receipt cannot revive
  that grant. Apply 0042 EB01 to EB05. With an already admitted local
  route next in the list, show the move and use that route only. With
  the local route unavailable and the paid route lacking a key or cap,
  keep the evidence and original passage and produce no wording.
  In a separate status read, advance a desired version containing a
  live key and a restart key. Check the
  desired position, mixed effective snapshot and pending restart key
  separately. Replace the process while the old acknowledgement is
  delayed. Hold the effective result until the current process is
  verified, then show its verified snapshot. The stale acknowledgement
  and an inbox receipt never certify the new process. The reads leave
  the registry and role state unchanged and request no restart.
  Join the desktop role-status read to 0020 AT-LOCAL-TRANSPORT and
  0118 IT-0118-LOCAL. Stage `GET /api/v1/settings/roles` with a committed
  desired position and a mixed effective snapshot. Lose the reply and
  event stream. Replace the role process before reconnect. With current
  read permission intact, a fresh read shows the desired position and
  "Effective settings not yet verified" until the new process is
  verified. The event replay and the old response do not verify it.
  In the separate revocation branch, keep the enrolled local peer and
  transport trust eligible. Revoke the Cedar read grant after the
  server prepares the response and before release. Return the safe
  denial with no settings fields. The next authoritative refusal
  clears the denied fields from the desktop view. Neither branch
  activates a version, acknowledges a role or requests a restart.
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
- AT-CF18 (`tests/prop_release.rs`, property): every accepted release
  change equals the signed spec 0077 champion's resolved configuration and
  stays in bound; a missing, altered or diagnostic manifest gets SUT-109.
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
- AT-CF29 (`src/check/proofs.rs`, Kani): for any bounded positive key,
  an accepted release change is in bound, above zero and exactly equal to
  the signed champion value; no differing value is accepted.
- AT-CF30 (`tests/rls.rs`, SQL against test PostgreSQL): the app role
  cannot update or delete a version, change or result. Verify forced
  row-level security on every table. First setup with an empty native
  site register creates the schema and no site settings or pointers.
  Try the authorised site operation with no site binding, inside
  installation-only scope and with both scopes bound. Confirm all
  three allow zero protected reads or writes. The mixed binding gives 0108's
  SUT-972 before protected SQL.
- AT-CF-INITIAL-SITE: provision each of two synthetic sites through
  0030. Interrupt before the first commit, then in a separate trace
  after commit before a reply. Resume the exact original site and
  intent. Each site has one first settings version and pointer from
  the defaults and the `balanced` preset. Pairing and provisioning
  must succeed before the site is visible. Try a different site, an
  old pending generation and a missing pointer on an established
  site. None can create a replacement, overwrite a later version
  or publish a site.
- AT-CF31 (`tests/bootstrap.rs`): a saved `max_links = 3` and
  `SUTRADHAR_LINKING__THREAD__MAX_LINKS=7` give SUT-119; the service
  runs on 3 and `--sources` names the saved version.
- AT-CF32 (`tests/restart.rs`): "Restart now" drains the admitted
  process and starts with the desired restart keys. A role event
  cannot request a restart. Check newer versions, stale process
  identity, Quit and removal each refuse the pending request.
  Check bounded drain, readiness on the new instance and refusal
  evidence after failure on Linux and Windows.
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
  it; no agent, variable or unadmitted candidate bypasses safety, trust,
  privacy,
  permissions or the publishing gate.
- AT-CF-RESET-INHERIT: for one synthetic site and one eligible key,
  remove its override through the confirmed human Repair plan.
  Use a current preset that differs from the compiled default. In
  separate cases use no preset entry and an override equal to the
  inherited value. Check absent override membership, effective
  values, sources, append-only history and one activation on exact
  replay. Change the preset after reset and verify the key follows
  the usual layers. Preserve the other site's override.
  Mix an ordinary inheritance with an ordinary value assignment,
  a protected key or a null in separate plans. Try duplicate keys,
  a changed parent and withdrawn authority. Try submitting the raw
  operation through the API, CLI, MCP or an agent request under a
  valid-looking approval. Separately exercise the admitted human
  Reset routes. Each invalid attempt leaves the caches, active pointer
  and previous settings versions unchanged. In the valid combined
  case, interrupt after settings activation and before cache cleanup.
  Reconcile the original activation before retrying the cleanup;
  current permissions and cache identities still apply.
- AT-CF38: wrong units, bounds, dependencies, concurrent changes,
  mismatched processes and excessive resource totals all refuse,
  keeping the active version and job snapshots.
  In the 0078 N-1500 branch, use a synthetic Custom profile with
  16 GiB of machine memory, 4 GiB of effective headroom and
  a 12 GiB budget. Save an authorised plan for 2 GiB of
  `budget.headroom_gb` and 13 GiB of `budget.custom_gb`.
  The desired combination fits a 14 GiB ceiling.
  At the next job boundary leave the headroom restart-pending.
  The effective ceiling remains 12 GiB. Hold the last complete
  effective snapshot, show the pending restart and admit no
  work using the unproved 13 GiB budget. Existing job snapshots
  and promises stay intact. After a separately authorised
  verified restart, check the actual current resource bounds
  before applying the desired values for new work.
- AT-CF39: history, job-to-version binding, secret exclusion, and help
  reachable by keyboard, touch and without hovering, restart keys
  included. Pair with 0032 AU-04 and AU-08. Back up an enrolled
  synthetic site with an ordinary non-personal settings change and
  its recorded actor. Erase that actor under 0032. With the existing
  current owner approval and restore history, restore the eligible
  value under the 0030 settings epoch and reconciliation checks.
  Read the restored value and its eligible original history only
  through the current authorised site projection. Keep the original
  non-personal version and diff. Show "removed member" on a permitted
  name lookup. The erased actor mapping and personal details stay
  absent before publication and on later replay.
  In a separate 0102 IT-SS-07 branch, remove the exact site through
  its approved removal plan while a second site remains enrolled.
  Offer the older backup with current retirement proof. The retired
  site gains no settings pointer, readable history or active role
  snapshot. The enrolled site's authorised history remains usable.
- AT-CF40 (`tests/rules.rs`): a confirmed agent draft that sets
  `coach.ai.order`, a Bedrock spending cap, the MCP grading switch, a
  backup route or `actions.confirm_wait`, directly, by a revert or by a
  preset, gets SUT-120; the owner's own draft of the same keys passes.
  A test fails when any `route_out` key lacks `owner_only`, and
  compares the `route_out` keys and the `owner_only` keys each both
  ways with a reviewed list kept in the test. For the synthetic 0135
  weight, try confirmed owner values of 0 and 500, then -1, 501 and a
  fractional value. The invalid values leave the pointer and outbox
  unchanged. A confirmed agent request for an otherwise valid value
  gives SUT-120. In a running batch pinned at 500, commit the owner's
  change to 200. The running batch stays on its recorded snapshot and
  the next eligible batch can use 200. Independent fact and release
  fences still apply. Reuse IT-0135-INSPECT for the current setting and
  the older result's separate used version and applied weight.
- AT-CF41 (`tests/discovery.rs`): CLI and MCP discovery give every
  declared safe description, including the owner route for an
  `owner_only` key, without a runtime value or secret status.
  In two synthetic sites, give the same key different values and
  deny another key to the test role. Read values, sources, a
  history page, a diff and role status on the admitted routes.
  Supply the other site's IDs for those reads. Separately revoke
  the read grant after a history page and replay its cursor.
  No denied value, source, presence or role result appears.
  Reuse IT-0152-NAME for the crawler name, public label and
  enable controls and IT-0152-LIMIT-SETTINGS for the numeric limits.
  Give sites A and B distinct labels. Check the
  protected same-site owner view, a foreign site, staff, and an
  MCP settings token. Discovery shows only safe descriptions. A
  denied read, change or replay gives no value, card proof or
  current enable state and writes no version.
  Compare a confirmed N-1463 reset and an admitted 0044 settings-file
  import before and after activation. Show only the authorised
  committed desired or effective values at their actual stage.
  A draft import or restart-pending value cannot appear applied.
  Static descriptions and denied change routes stay distinct.
- Benchmarks `bench_bootstrap_5000`, `bench_snapshot_get`,
  `bench_draft_check_50` and `bench_history_page`, and the load test
  `load_activation`, hold the speed targets under the 5% median rule.
- Checks of library behaviour: figment keeps each value's source;
  bytesize reads `GB` as 10^9 and `GiB` as 2^30 and refuses a bare
  number; humantime-serde accepts `30s`, `5m` and `1h30m` and refuses a
  bare number; garde field paths map to dotted keys; `jsonb` and the
  toml crate keep `5` and `5.0` apart; sqlx's migrator holds its lock
  for the whole run. Kani runs AT-CF29 within the gate's time.

## Changes from the source

- Profiles are `dev`, `test` and `prod`; the old `bench` and `lab`
  profiles are dropped.
- "Restart now" uses the platform supervisor in spec 0030.
  Migrations run at start-up.
- The old settings pages are desktop app screens built from SDS, using
  the management API on the local socket.
- Permissions are the Cedar rules, not the old action layer.
- Changes and notifications use spec 0019's transactional outbox.
  Spec 0110 supplies relay and inbox delivery.
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
- Strict-core MC/DC and mutation requirements follow ADR-0020. Equivalent
  mutants need proof and independent review; other settings code follows
  its risk-appropriate coverage and mutation contract.
- Nightly chaos tests run in the release tier.

Source: `docs/specifications/platform/settings.md`,
`platform/settings-2.md`, `platform/settings-3.md`,
`platform/settings-4.md` and `platform/settings-5.md` in the archive
`archive-2026-11-08-old-repo`.
