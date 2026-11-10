# 0030: Point-and-click install, upgrade and removal

This part installs, upgrades, backs up, restores and removes Sutradhar by
clicking through screens in the desktop app, with no terminal for ordinary
work. It is roadmap Phase 5 item 1.

Installing checks the computer first, then installs Sutradhar's signed
parts, starts them inside the memory limits and checks they work. An
upgrade is never automatic: the owner is told what will change and
approves; the affected features pause at once; a backup is checked; the new
version starts; and if its checks fail it rolls back by itself. Removal
offers an export, lets the owner choose what to keep, always deletes the
embeddings, search index and model files, and leaves every approved link in
place.

Example: the owner sees "Version 1.4 is available: 3 capabilities added, a
database change of about 2 minutes, suggestions paused for about 6
minutes". The owner approves; suggestions pause; last night's backup is
verified; the new version starts; the smoke test passes; suggestions
resume. Had the smoke test failed, 1.3 would run again within 3 minutes and
the screen would say why.

## What it does

### Words used here

- Host helper: `sutradhar-host`, a small Rust program outside the service
  that may act only on Sutradhar's own systemd units, slice, data folder
  and package.
- Preflight: the checks run before anything is installed.
- Smoke test: a short end-to-end check that the new version works.
- Migration: a numbered SQL change to Sutradhar's own tables, run once.

### Parts and boundaries

- `sutradhar-host` has a fixed set of verbs: `preflight`, `fetch`,
  `install`, `stop`, `start`, `status`, `logs` (bounded), `migrate`,
  `backup`, `restore`, `switch`, `rollback`, `extension-start`,
  `extension-stop` and `uninstall`. It accepts only signed requests from
  the service, on a local socket. It refuses any unit or package that is
  not Sutradhar's, any package whose hash and signature do not match, and
  any setting outside the shared allow list. It runs without root rights
  beyond what those verbs need.
- The service's setup role writes the unit settings from their template,
  runs the smoke test, and owns the install, upgrade and uninstall state
  machines, step by step, with a checkpoint after each step and progress
  shown in the app.
- The Dioxus app has the Installation, Upgrade, Backups, Restore and
  Remove screens, built from SDS components.
- The site add-ons are installed through each platform's own add-on
  system, from the signed zips the app hands the owner. Their thin setup
  pages handle the site link and the site's own disable and uninstall.
- Traits, each with a fake: `HostClient` (the helper's verbs),
  `ReleaseFeed` (where release manifests come from), `BackupStore` (a
  local folder or the owner's cloud bucket) and `PreflightCheck` (one per
  check, so a check is added without touching the others).
- Install, upgrade and rollback share the same steps: fetch, migrate,
  start in health order, smoke test. Systemd only; no orchestrator.

### Preflight

Each check passes, warns or fails, in plain English with its code.

- Ubuntu 26.04 on x86-64; cgroup v2 with the memory controller; the slice
  limits work (a test unit is started with a limit and checked).
- Free disk for the program, the data estimate for each site's post count,
  and backups.
- Fewer than 4 cores or no solid-state disk: a warning that the 30-second
  promise for new posts does not apply.
- Memory: the PC has at least 8 GB (16 GB recommended); the installer
  reads the PC's size and sets the service slice and window caps of
  AGENTS.md section 5.6 for it, with no swap. Under 8 GB the install
  fails with the reason. No check of memory left for other programs.
- The clock is in sync.
- For each connected site: XenForo 2.3 or a supported WordPress, PHP 8.5,
  and an add-on contract version that matches.

### Installation

1. Preflight passes. Warnings are shown; failures stop.
2. The owner chooses the data folder, optional modules (off by default)
   and monitoring (on by default).
3. The unit settings are written from the template with limits, and the
   memory total is checked again.
4. The signed package's parts are verified by hash and signature.
5. Secrets are created and held by tijori.
6. PostgreSQL starts. Migrations run as the schema-owner role, creating
   the tables, the app roles (none with `BYPASSRLS`) and the RLS
   policies. The step fails unless every table has RLS enabled and forced.
7. Parts start in health order and each must pass its health check. ganit
   must also report "warm".
8. Models are downloaded and verified.
9. The smoke test runs on synthetic data inside the new install, including
   one ganit call and, with ganit stopped, the degraded mode (top-k with
   the hard rules).
10. The app asks the owner to approve the first backlog run, and it
    starts in the background only once approved; the app then moves to
    its normal screens. No setup test writes real content without
    approval (archive `installation-platforms/lifecycle.md`: "Do not
    write real content as an unapproved setup test or start a backlog
    run before its approval").

Every step can resume: a failure at step 7 restarts at step 7.

### Licence and source offer

- The first setup screen says in plain words that Sutradhar is free
  software under AGPL-3.0-or-later, links the licence, and offers the exact
  source of the version being installed (the release's source archive for
  its tag) with the third-party notices.
- The app's footer and `sutra source` keep that offer.
- If parts are installed that are not the project's published builds,
  setup asks for the address where the changed source is offered, and
  explains why: anyone running a modified version for others must offer
  its source.
- The manual's installation and licence pages say the same, and the offer
  covers the add-ons too.

### Upgrade

1. Notice: a signed release manifest lists the new parts' hashes, the
   capabilities added, changed and retired, migrations with their measured
   time, the expected pause and any memory change. It is shown in the app
   and as a staff alert on each site. Never automatic.
2. Approval by the owner, or an administrator with the upgrade
   permission, audited.
3. Pause at once: suggestions, backfill and jobs reach their safe points
   and stop; the review page says "Paused for upgrade".
4. Backup check: the latest backup exists, is whole, is encrypted and
   checksummed, and has passed a test restore. If not, a backup runs. If
   that is not possible, only the owner may override, with a typed reason,
   recorded. No code gate can be overridden.
5. Fetch and verify the new package.
6. Migrations are written so the previous version still runs on the new
   tables (add first, remove a release later), so rollback needs no
   database restore.
7. Start: the large parts (inference, jobs, scheduler) stop before their
   new versions start, so the slice total never passes its cap. Only small
   parts may run old and new side by side, and only when both fit.
8. Smoke test. On success the new version takes over and work resumes; on
   failure, automatic rollback.

Also:

- Rollback is one button or `sutra rollback`. The previous version's
  package is kept until the next successful upgrade and starts the same
  way.
- PostgreSQL minor versions change with the package. A major version
  changes only inside a Sutradhar release, by dump and restore after the
  backup check, named in the notice with its time.
- Ganit output contracts carry versions. Parameters from the previous
  version are used only if the new contract accepts them; otherwise users
  keep their safe defaults until the next nightly run, shadow-tested as
  always. Rollback keeps the parameter registry, which only grows.
- After the upgrade, ganit job 307 compares live figures before and after,
  and the Upgrade screen shows any regression with the rollback button
  beside it.
- An upgrade holds, and is not forced, when an enabled extension's contract
  range excludes the new version.
- An upgrade can come from a downloaded package, for a computer with no
  internet.
- An upgrade notice the add-on cannot use yet says "upgrade the add-on on
  the site first".
- Every release is battle tested before it is offered, its upgrade path
  and the add-ons included (tests 21 to 26 below).

### Backup and restore

- Nightly and before each upgrade: a consistent PostgreSQL dump (custom
  format, with roles and RLS policies), which also holds ganit's parameter
  registry, plus the data folder's owned files. Compressed with zstd and
  checksummed with BLAKE3.
- Every backup is encrypted on the PC before it goes anywhere. The owner
  holds the key; with no key there is no off-PC backup. Targets: a local
  folder or the owner's own cloud bucket.
- Left out: secrets (listed for re-entry), caches and telemetry, ganit's
  working snapshots (rebuilt from the tables) and model files
  (downloaded and verified again). The vector index is included when disk
  allows.
- A test restore into a temporary install runs weekly in CI on synthetic
  data, and monthly on the owner's PC when the load governor allows. Its
  result feeds the upgrade's backup check.
- A restore passes only if every table again has RLS enabled and forced
  with its policies, and no role has `BYPASSRLS`.
- Backups and exports hold member data. Only the service account can read
  them, they are never published, and erased data is removed from them as
  the retention rules set.

### Disable, enable and uninstall

Disable and enable follow the models design: every part stops within 90
seconds and resumes where it was. Uninstall, from the app or from a site's
add-on page for that site:

1. A list of everything Sutradhar holds, locally and in the cloud, with
   sizes, in three groups:
   - always deleted: embeddings, index, model files and the trash bin;
   - deleted without export: caches and telemetry;
   - the owner's choice: settings, suggestion history, audit, analytics,
     extension stores and the rest, each with export and keep or delete.
2. Export runs with progress, checksums and a download link.
3. Parts stop and are removed. The PostgreSQL data is deleted unless some
   data was kept, in which case only the kept tables remain, with their RLS
   policies. Chosen files are deleted; cloud objects only if chosen.
4. Approved links stay in the sites' posts.
5. The host helper removes itself last and leaves a short receipt.

- Uninstall while a backup runs cancels the backup at its safe point
  first.

### Data

In PostgreSQL with RLS enabled and forced; only the setup role writes
these tables.

- `install_state`: version, package hashes, step, checkpoint, started, by.
- `releases_seen`: version, manifest hash, notice time, decision.
- `upgrade_runs`: from, to, approved by, override reason (or none), steps
  with times, result, rolled back.
- `backups`: id, kind, size, BLAKE3 hash, location, verified restore at.
- `uninstall_receipt`: categories with keep or delete, export hashes.

### Actions and events

- Actions: `sutradhar.preflight`, `.install`, `.upgrade.notice`,
  `.upgrade.approve`, `.rollback`, `.disable`, `.enable`, `.backup.run`,
  `.backup.restore`, `.uninstall.plan`, `.uninstall.apply`. Each is a
  management API route checked by Cedar and a `sutra` command.
- Events: `install.step`, `upgrade.available`, `upgrade.paused`,
  `upgrade.finished`, `upgrade.rolled_back`, `backup.verified`,
  `uninstall.finished`.
- Capabilities: setup and `sutra install` (CAP-001); upgrade notice and
  approval (CAP-002); rollback (CAP-003); disable and enable (CAP-004);
  removal (CAP-005); restore (CAP-031); preflight ("Check whether a
  computer is ready for Sutradhar before installing") and backups ("Back
  up Sutradhar's data on a schedule or now, and see it verified"), each a
  new entry.

### Failure codes

SUT-830 preflight failed (each check named); SUT-831 host helper
unreachable; SUT-832 package hash or signature mismatch; SUT-833 migration
failed, nothing switched; SUT-834 health check failed at start; SUT-835
smoke test failed, rolled back; SUT-836 no verified backup (owner override
possible); SUT-837 limits add up to more than the cap; SUT-838 add-on and
service versions do not match, and the screen says which side to upgrade;
SUT-839 export failed, nothing deleted. Any failure leaves the previous
working state or the paused state, never a half-switched one.

### Security and privacy

- The helper acts only on Sutradhar's own units, from signed requests,
  with fixed verbs.
- Release manifests are signed with tijori's signing core; the helper
  refuses an unsigned or wrongly signed one.
- The migrate role alone owns the schema and is never used at run time.
- No step sends site content off the PC unless the owner chose a cloud
  bucket, and then only encrypted.
- The override needs the owner role.

### Screens

WCAG 2.2 AA. Each step is a numbered list item with its state in words
("Running", "Failed: SUT-834"); progress is a bar with text; the uninstall
choices are grouped radio buttons with a fieldset and legend per group,
set to "Keep" so nothing is deleted by an unread default; the final delete
button repeats what it deletes; the override reason is a labelled text area
with its purpose stated. The add-ons' setup pages use the platform's own
admin styles and meet the same rules.

### Observability

- Metrics: `sutradhar_install_step{step,status}`,
  `sutradhar_upgrade_duration_seconds{phase}`,
  `sutradhar_upgrade_total{outcome}`,
  `sutradhar_backup_last_success_timestamp`,
  `sutradhar_backup_restore_verified_timestamp`,
  `sutradhar_memory_limit_bytes` (the total).
- The host helper writes structured logs to the journal and its own
  metrics file, read by the service.
- Alerts: backup older than 36 hours; test restore older than 35 days; an
  upgrade paused more than 15 minutes; a part restarting again and again.

### Edge cases

- Power loss during a migration: the migration's journal shows the step,
  and running it again completes or reverses it.
- Disk full while fetching: stopped before anything is switched.
- A site restored from its own backup to an older state: Sutradhar sees
  missing posts and reads them again.

## Limits

- Memory follows the limits in AGENTS.md section 5.6. The service,
  engine, models, the MCP server and PostgreSQL share the service
  slice; the window has its own cap. The installer and the gate refuse
  limits that add up to more, and the total is never passed while parts
  start or switch. Every budget fits a PC under 16 GB; larger settings
  apply only on a PC with 16 GB or more.
- Starting estimates for each size of PC are in spec 0024's memory
  table, to be measured on Dell. This part adds migrations (256 MiB,
  run alone in the heavy job slot) and the host helper (about 16 MiB).
- ganit's nightly batch runs only in the nightly window, in the heavy
  job slot of spec 0024, while backfill and rebuilds pause and the
  reranker is unloaded. Whether new posts are still served within 30
  seconds then is measured on Dell, on both sizes of PC.
- x86-64 only in version 1.
- Speed targets, on the reference machine (4 cores, SATA solid-state disk),
  without downloads and the backlog run, each held by the 5% median rule:
  - preflight under 60 s;
  - installation after downloads under 10 minutes;
  - upgrade pause, without migrations, under 5 minutes, with each
    migration's measured time on reference data in the notice;
  - automatic rollback under 3 minutes;
  - disable: every part stopped within 90 s;
  - backup at least 50 MB/s; restore at least 30 MB/s;
  - uninstall, without the export, under 5 minutes.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Rust tests unless stated; add-on tests are PHP tests with PHPStan at its
strictest level.

1. Preflight on fake hosts each missing one need fails with SUT-830 naming
   it (`preflight.rs`).
2. Memory is the only memory check: with other programs using memory, it
   passes when the limits fit (`preflight.rs`).
3. Limits over the cap are refused with SUT-837 by install and the gate
   (`memory_total.rs`, unit and property).
4. The helper's fence refuses another program's unit, an unsigned or
   mismatched package, a folder outside its own and a missing signature
   (`sutradhar-host/tests/verbs.rs`, and a property test).
5. An install killed at each step in turn finishes from that step with
   no step repeated (chaos).
6. A new release gives only a notice and a staff alert; nothing is fetched
   (`upgrade.rs`).
7. On approval, suggestions and backfill stop at safe points within 60 s
   and the review page says so (Dell).
8. With no verified backup the upgrade stops with SUT-836; an
   administrator's override is refused; the owner's override with a reason
   continues and is audited.
9. A failed smoke test brings the previous version back within 3 minutes
   with no lost or repeated work (chaos).
10. With random part sizes, the running limits never pass the cap at any
    moment of the start step (property).
11. The previous version's tests pass against the migrated tables.
12. A backup of the synthetic site restored into a fresh install gives the
    same records and checksums (weekly in CI).
13. "Keep settings, delete analytics" does exactly that; embeddings, index
    and model files are always deleted; approved links stay in every post
    (Dell test site).
14. The Remove screen submitted without changes deletes nothing optional
    (Dioxus screen test).
15. An add-on one contract version behind is refused with SUT-838, and the
    screen says which side to upgrade.
16. Every screen and state passes the SDS accessibility checks and matches
    its screenshot.
17. After a fresh install and after a restore, the RLS check finds every
    table enabled and forced with policies and no `BYPASSRLS` role (SQL
    test against the test PostgreSQL).
18. The first setup screen offers the exact source and notices; an install
    of unpublished builds asks for the source address (Rust and PHP tests).
19. ganit answers warm in the smoke test; with ganit stopped, suggestions
    still come from the degraded mode (chaos).
20. Rule-critical code (the memory total check, the helper's fence, "never
    over the cap while starting" and "delete only what was chosen"): 100%
    lines, branches and MC/DC, every mutant killed, property tests, and
    Kani proofs that no accepted settings add up to more than the cap and
    that the uninstall plan never deletes a group marked Keep.

Battle tests on every release candidate:

21. Upgrade under the full load: the pause stays in its target, no work is
    lost or repeated, the cap is never passed.
22. Faults during upgrade, in turn: release source unreachable, disk full,
    PostgreSQL killed mid-migration, the host helper restarted, a bad
    signature. Each stops safely or rolls back with its code (chaos).
23. Memory read every second through the upgrade and the nightly ganit
    window never passes the cap, and no part is killed for memory.
24. The add-on upgraded before and after the service: site pages work
    throughout, mismatches show SUT-838, and the add-on stays within 8 MB
    a request (PHP test, Dell test site).
25. Under full load a backup is compressed, its ratio no more than 5% worse
    than the last release, and its test restore passes with RLS intact.
26. Rollback under full load serves again within 3 minutes and the service
    levels recover.

## Changes from the source

- Sutradhar installs from a signed `.deb` on the owner's Ubuntu 26.04 PC
  and runs under systemd, not as containers on the site's server.
- The host helper acts on Sutradhar's systemd units and package, not a
  container project, and is reached only on a local socket.
- Package hashes and signatures replace image digests; manifest signing
  uses tijori's signing core.
- The read-only forum database check is dropped, since the service reaches
  each site only through the add-on over HTTPS.
- The screens are in the Dioxus app; the old web admin app and its live
  layer are gone, and so is their memory share.
- Metrics and traces come from Sutradhar Boot, not separate monitoring
  services with their own memory shares.
- Memory follows the limits in AGENTS.md section 5.6, in place of the
  old 5.6 GB cap and 4.76 GB peak.
- Speed targets follow the 5% median rule.
- The add-ons are GPL-2.0-or-later, as AGENTS.md section 5.7 sets.

Source: `docs/specifications/operations/install.md`,
`operations/install-2.md` and `operations/install-3.md` in the archive
`archive-2026-11-08-old-repo`.
