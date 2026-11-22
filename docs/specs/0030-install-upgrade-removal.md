# 0030: Point-and-click install, update, repair, backup and removal

This part installs, updates, repairs, backs up, restores, moves and
removes Sutradhar by clicking through screens, with no terminal for
ordinary work. It owns the installers, the home folder, the file ledger
and the update route. It follows ADR-0021: one signed `.deb` for Ubuntu
and Debian, an unsigned installer for Windows, an engine the app starts and
watches inside one home folder, and updates that come from GitHub
Releases. It is roadmap Phase 5 item 1.

Installing checks the computer, places signed program files, creates the
home folder and its file ledger, starts the engine inside the memory
budget of spec 0078 and checks that it works. An update is found by a
check, explained in plain English and applied with one click; the affected
features pause, a backup is checked and the new version starts. If its
checks fail, the previous version returns automatically only with proven
compatibility and current trust. Otherwise the app keeps work paused and
shows the recovery step. Removal reads the ledger,
shows every file group with Keep or Delete choices, deletes what is chosen
and leaves a receipt that proves nothing else remains.

Example: Priya installs Sutradhar on her Windows laptop. The installer
puts the program in her own program folder and makes a `Sutradhar` folder in
her user folder. Next day the app says "Version 0.9 is available: 4
changes, a database step of about 2 minutes, suggestions paused for about
6 minutes". She clicks Update. Last night's backup is checked, the new
version starts and the self-test passes. Months later she clicks Remove,
keeps her scripts and deletes the rest, and gets a receipt saying no files,
tasks or start-up entries are left.

Values marked "proposed" are design choices to be checked by measurement.

## What it does

### Words used here

- **Program files**: the signed, read-only parts of Sutradhar that the
  installer places (the app, the engine supervisor and bundled runtimes).
- **Home folder**: the one folder that holds everything Sutradhar creates
  while it runs: data, models, scripts, backups, logs and the ledger.
- **File ledger**: an append-only, hash-chained record of every file,
  folder, shortcut, start-up entry and background task Sutradhar creates,
  with its path, group, size, who made it, when and why.
- **Engine supervisor**: `sutradhar-engine`, a small Rust program that
  starts, watches and stops the engine parts. It runs as the user, not as
  an administrator.
- **Preflight**: the checks run before anything is installed.
- **Self-test**: a short end-to-end check that a version works.
- **Migration**: a numbered SQL change to Sutradhar's own tables, run once.
- **Portable mode**: a copy of everything in one folder that runs from any
  location, including an external drive.
- **Update channel**: the stream of signed releases the app follows.

### Parts and boundaries

- The Tauri app (Angular screens built from SDS) shows the Installation,
  Update, Backups, Restore, Repair, Files and Remove screens.
- The engine supervisor owns the install, update and uninstall state
  machines step by step, with a checkpoint after each step, and starts the
  parts in health order. It accepts only signed requests from the app, on
  the owner-only local link (socket on Linux, named pipe on Windows).
- Installers only place program files and create the home folder. The Linux
  `.deb` installs program files in the system's program area and registers
  nothing else. The Windows installer places program files in the user's own
  program folder and needs no administrator rights. One standard
  uninstall entry needs QR-015 owner approval and installer admission.
  No other registry settings are allowed.
- Traits, each with a fake: `ReleaseFeed` (where release manifests come
  from), `PayloadStore` (verified downloads), `BackupStore` (a local
  folder, an external drive or an S3-compatible bucket), `PreflightCheck`
  (one per check, so a check is added without touching the others) and
  `LedgerWriter` (the only way to create a file under the home folder).
- Install, update and rollback share the same steps: fetch, verify,
  migrate, start in health order, self-test.
- Storage moves to an external drive or a bucket, and the storage wizard,
  belong to spec 0038. This part requires only that every move updates the
  ledger and that removal follows it.
- The site add-ons are installed through each platform's own add-on
  system from the signed zips the app hands the user (spec 0031).

### The home folder

The layout is the same on Linux and Windows. The default place is a folder
named `Sutradhar` in the user's own folder; the user may choose another,
including an external drive.

| Folder | Holds | Group in the ledger |
|---|---|---|
| `engine/` | Runtimes the engine needs (PostgreSQL, the Python runtime, native libraries) | Engine |
| `data/` | PostgreSQL data and one folder per site | User data |
| `models/` | Downloaded models and their hashes | Models |
| `scripts/` | The user's Python scripts and their environments | Scripts |
| `backups/` | Local backups | Backups |
| `exports/` | Exports the user asked for | User data |
| `cache/` and `tmp/` | Rebuildable files and scratch space | Caches |
| `logs/` | Owner-only log folder and rotation manifest under spec 0112 | Logs |
| `ledger/` | The file ledger, its checkpoints and the site register | Ledger |

Rules: nothing is written outside the home folder except program files, the
start-up entry and the uninstall entry, and each of those is in the ledger.
Settings are readable files in the home folder, never registry values. The
program files are never edited by the app after install. `tmp/` empties
itself when Sutradhar stops cleanly.

### The site register

1. The service reads `ledger/site-register.toml` before opening a site
   transaction. It holds the installation id, format version,
   generation and the site ids, names, platform, canonical site links,
   settings profile references and lifecycle states. It holds no
   secret or page data. Pause and weight use their existing settings
   owners. The register is accessible only to the service and owner.
   For confirmed first setup, DR-151 generates an installation ID
   from 16 bytes of operating system randomness. The register
   stores `installation_id` as a TOML string of exactly 32 lower-case
   hexadecimal characters. Persist the identity in the original
   first-setup intent and commit the register before opening a
   scoped PostgreSQL transaction. Reuse that identity on retry.
   An older register needs its exact admitted decoder. Unsupported
   format or identity holds. Do not infer a format from the string,
   rewrite the ID or generate a replacement on restore.
2. The service is the single writer under the home-folder lock.
   Check ids, names and canonical links against the whole register.
   LedgerWriter first persists an intent with the old and new file
   checksums, generations and ledger entry. Flush the candidate
   before atomic replacement and make that replacement durable.
   Append and flush the ledger commit under the same lock before
   any reader sees the new generation.
   On restart, finish a prepared replacement if the live file
   matches the new checksum. If it matches the old checksum, abort
   the intent. Any other checksum blocks recovery. The commit step
   updates the file ledger once, even when recovery runs twice.
   Refuse a damaged, unsupported or missing register. Create an
   empty one only on confirmed first setup. Files screen repair
   shows the fault instead of guessing from database rows.
3. Record a pending site before pairing or provisioning. Publish it
   to readers only after both succeed. For removal, durably mark the
   site unavailable before stopping its work and running spec 0102's
   removal list. Keep the pending removal until cleanup succeeds.
   Resume either pending operation on restart by site id and
   generation. A stale worker cannot publish a tile or reopen a
   retired site. Keep a durable retirement record with site id and
   generation, without the old name or link. Record it in the
   install's protected restore history before publishing removal.
   Keep that history while a retained backup could restore the data.
4. Capture the register under a lifecycle barrier while the
   database backup snapshot starts. Retry when setup or removal is
   pending. Bind the file checksum and generation with the database
   snapshot and ledger checkpoint in the manifest. Leave out the
   portfolio cache. Validate site ids and manifest before restore.
5. Restore preserves the current install's retirement and spec
   0032 erasure history outside the files it replaces. A portable
   move carries the latest authenticated history checkpoint and
   chain from the source install. Verify continuity and apply newer
   fences before publishing restored records. If current history
   cannot be verified, withhold the affected sites and data. A site
   with missing keys stays disconnected until the owner pairs it
   again. Registry recovery makes no cross-site data query.
   Bind the prepared restore to the exact retirement and erasure
   history checkpoints. After isolated verification, compare those
   checkpoints with current history at the guarded live publication.
   Use the current lifecycle guard to order that decision against
   admitted retirement or erasure commits. For a portable handover,
   require the authoritative source checkpoints and verified handover
   fence. Hold if currentness cannot be proved. If history advanced,
   show “Restore paused. Removal or erasure records changed.”
   Hold the affected staged generation before looking up any site,
   job or data in it. Keep the current
   live state and original backup. Recheck and apply the current
   fences before another publication under the exact eligible plan.
   If its approved scope changed, require a fresh owner preview and
   confirmation. A valid history advance is a restore hold. An audit
   write failure keeps its separate 0032 SUT-848 refusal.

For spec 0014, keep the settings epoch and restore grant commitment
per site and domain in the current protected restore history.
Reserve and persist a new epoch before publishing restored settings.
After a crash, use that epoch only for the same prepared restore.
Recheck the exact approved plan and current authority of the owner
who approved it before publishing. Match the verified installation,
site, settings domain, backup manifest and isolated target to the
reserved epoch and its grant commitment. Changed scope or lost or
unproved authority holds the affected settings before activation.
Another restore or move needs a different activation epoch.
Portability carries the latest authenticated history. A stale or
missing history cannot reinstate settings or approvals. Test crash,
repeat and move in AT-CF-DELIVERY of spec 0014.

See IT-SS-07 in spec 0102 for restart, removal and restore fixtures.

### The file ledger

1. Every creation goes through `LedgerWriter`. A direct write elsewhere in
   product code is a build-time ban (spec 0033).
2. Each entry has: path (relative to the home folder where possible), kind
   (file, folder, shortcut, start-up entry, task), group, size, content
   hash for files that never change, creator (installer, app or a named
   job), time and plain-English reason.
3. Entries are appended with a chained hash, so a missing or edited entry
   is found. A mirror table `ledger_entries` lets the Files screen search it.
4. Deleting or moving a file updates the ledger in the same step.
5. The Check files action compares the ledger with the disk. Missing
   program files can be restored from the signed package; missing user
   files are reported, never invented. A file on disk that the ledger does
   not know is listed as unknown and left alone until the user decides.

6. Record one managed log folder and versioned manifest. Verify
   an owner-only root and relative entries. Atomically publish each
   manifest generation after the log writer durably seals a file or
   prunes it. Record time ranges, sizes and sealed hashes. Show open
   files as unsealed. Reconcile interrupted rotation or prune before
   use on restart. A replaced or unknown entry is a discrepancy.
   Rotation never adds a ledger row per file.

### Preflight

Each check passes, warns or fails, in plain English with its code.

- Operating system: Ubuntu 26.04 LTS or Debian 13 on x86-64, or 64-bit
  Windows 11. Windows 10 reached end of support on 14 October 2025 and
  is not supported (RQ-012, DR-055). macOS is not supported.
- Memory: at least 8 GiB on the supported baseline. The installer
  shows the budget spec 0078 will set. QR-013 owns the exact
  below-minimum installer behaviour. Qualify smaller hosts only
  with that decision and within the verified ceiling.
- Disk: free space for the program, the data estimate for each site's post
  count, models and backups, within the disk cap.
- Window support: the desktop's web view is present (webkit2gtk 4.1 on
  Linux; WebView2 on Windows, which the installer's bootstrapper adds if it
  is missing; RQ-001).
- Fewer than 4 cores or no solid-state disk: a warning that the 30-second
  promise for new posts does not apply.
- The clock is in sync, because signed messages carry time.
- A Vulkan graphics card is detected for information only.
- For each connected site: XenForo 2.3 or a supported WordPress, PHP 8.5
  and a matching add-on contract version.

### Lifecycle disk admission

Use spec 0033's protected floor and outstanding promises and spec
0031's lifecycle lease. Bind the approved roots, exact volumes and
qualified workspace profile to each install or update plan. Reserve
the measured remaining peak on each required volume before creating
lifecycle files or pausing writes for update. Include staged release
bytes, migration workspace and the required recovery journal and
fallback workspace. Count an existing retained payload in actual
occupied bytes. Reserve any further space its fallback needs.
Free bytes on another volume cannot cover a shortfall. The backup
override leaves the disk and cap checks in force.

Before a space-consuming stage or fallback begins, recheck current
approved root identity, volume, remaining reservation, free capacity,
protected floor and owner cap. Treat a substituted, unavailable or
stale root or volume observation as unproved. A displayed path and
matching volume can refer to a changed backing directory. Use SUT-837
for a proved budget excess.
A shortage before write pause leaves the verified running version
and its writes in place. If the disk changes or a write runs out of
space after pause, stop at the owning safe boundary. Keep current
data, the verified previous payload and the recovery checkpoint.
Hold a further write or switch until current space and exact identity
are proved. Recheck the exact approved root and volume, remaining
reservation and durable journal at the final switch and write resume,
even after a passing self-test. If only root identity is unproved,
show that the approved update location could not be verified and keep
writes paused. Do not use or clean up the substitute. Keep the
verified backing root and native volume bound through the switch
and each unperformed write-resume effect. At use, the profile must
prove that the effective object is the one checked for the exact
owner-approved plan and current lease, with the matching managed
ledger and reservation. A path re-open needs that same proof. Hold
an unproved hand-off under QR-202. Retain the existing journal and
migration receipt. Only the verified commit and current read-back
of its exact effect can supply the switch receipt. If a current
reading proves insufficient space, show
“Update paused. The approved drive no longer has enough space” with
SUT-837. An unavailable journal cannot supply a successful checkpoint,
resumed write state or rollback receipt.

If a switch reply is lost, use current read authority to reconcile the
original operation and its verified committed result. If the result is
unproved, hold rather than replay the switch. A verified result keeps
its exact effect identity. Any remaining effect still needs current
space, authority and lease. Retain 0033's existing eligible space-freeing
steps and forbid automatic relocation or deletion of protected
recovery files.

Admit the peak workspace, disk floor and recovery hold through the
existing disk-safety record before use. Qualify the intended managed
volume and release route, including staging, journal and fallback
peaks. An unproved reservation cannot enable cloud or removable
storage beyond its existing route.

### Installation

1. Preflight passes. Warnings are shown; failures stop.
2. The user chooses the home folder, optional modules (off by default) and
   monitoring (on by default). A portable copy is offered here.
3. Recheck disk admission after choosing the home and before its
   first file effect. Verify the native setup authority and bootstrap
   trust under spec 0031. For a missing home, require a qualified
   native intent, lock, disk reservation and recovery anchor under
   QR-202 before creation. The home lock and LedgerWriter protect
   the owner-private home and ledger. Hold an unproved profile or
   ambiguous earlier creation.
4. Verify each part by hash and release signature before
   extraction or execution (spec 0065). On Windows the independent
   first trust check below precedes starting the unsigned installer.
5. Secrets are created and held by tijori.
6. Before starting PostgreSQL from `engine/`, the verified native
   supervisor checks current available memory and the admitted process
   limits under 0031 and 0078. Enforce those limits before launch. Just
   before opening the migration session, recheck capacity and reserve
   the migration memory in Limits inside that same envelope. Keep the heavy slot
   exclusive. First setup uses the verified native setup intent for
   both checks. It starts no ordinary scheduler job, and grants no site
   authority. A shortage or unproved current reading or process limit
   holds that step with SUT-830 naming the failed check. Show
   “Not enough free memory for this step” for a known shortage
   or “Sutradhar could not verify memory for this step” if unproved.
   A proved budget above the ceiling uses SUT-837. Keep prior
   verified receipts and any already committed database state.

   The supervisor verifies the exact installation scope described
   below. Migrations run as the schema-owner role under spec 0108.
   The step fails unless every table, including migration history,
   has RLS enabled and forced. Settle the permitted installation
   checkpoint before closing that context. A site-dependent check
   uses its own admitted site scope, including a synthetic fixture
   scope where the existing setup test calls for one.
7. Parts start in health order inside the budget of spec 0078; each must
   pass its health check. ganit must report "warm".
8. Starter models arrive from the offline bundle or from the Model Hub,
   hash-checked. The installer stays small; models are never in it.
9. The self-test runs on synthetic data inside the new install, including
   one ganit call and, with ganit stopped, the degraded mode (top-k with
   the hard rules).
10. The first-run wizard starts. The app asks the user to approve the first
    backlog run, and it starts only once approved. No setup test writes
    real content, and none starts a backlog run before its approval.

Every step can resume from its verified checkpoint. A failure at step 7
restarts at step 7 when the current release, recorded step and authority
still agree.

#### Installation database scope

Use the owner-confirmed managed home, installation identity in the
verified site register and durable LedgerWriter checkpoint before
relying on PostgreSQL history. Confirm the empty register only in first setup.
Bind the installation identity, register generation, owned database root,
exact verified release and migration profile, and current step intent.
Apply the home lock and spec 0108's migration lock where that step
requires them. A missing or damaged register, unknown checkpoint,
different root or changed approval holds before effect. The SQL database
or a pending site cannot supply replacement identity or authority.

DR-150 admits the existing lifecycle setup role to the installation
transaction of spec 0108 for the permitted setup or migration step.
After site enrolment, require an owner-approved exact update under
DR-149 before using the same scope for its migration and checkpoints.
Before SQL is ready, keep the verified checkpoint in the file ledger.
Preserve the original native disk reservation and effect identities
across restart and the database handoff. After the database policies
are verified, write only the authorised installation checkpoint and
mirror rows below, using committed native evidence. A missing or
unmatched mirror cannot replace the native first-setup proof.
The migrator owns schema
work and migration history under 0108. The setup role owns the lifecycle
checkpoints, including its record of a migration outcome. A normal app,
site job or connector receives no installation context.

On interruption, compare the original verified intent, current release
trust and authority, live files and committed checkpoint before
resuming. Continue the same
step if its next effect is proved safe. Reconcile a proved completed
step without repeating it. A ledger mismatch remains SUT-4100. An
unproved effect keeps the operation paused without moving its checkpoint
or recreating a resource. Close installation context before ordinary
site work. Site registration, pairing and provisioning keep their own
authority and exact site context.

### The engine supervisor

- On a normal install it starts at login through one start-up entry in the
  ledger (an autostart file on Linux; a shortcut in the user's Startup
  folder on Windows) and keeps the engine running when the window is closed.
- A tray icon shows its state; "Quit Sutradhar completely" stops all parts
  within 90 seconds (proposed) and leaves them stopped.
- In portable mode there is no start-up entry. The engine stops when the
  window closes and when the drive is unplugged (spec 0038 shows the
  safe-unplug light).
- Track the verified launched instance and exit code. Read atomic
  state samples under specs 0106 and 0112. Reset the monotonic watch
  baseline for each instance. A missing, corrupt or wrong-instance
  sample shows unavailable after bounded checks. It triggers no restart.
- An admitted stale heartbeat can identify a stuck verified instance.
  Recheck exact process handle, instance and permission to run at the
  stop and start boundary. Quit, draining, update, rollback, removal
  and portable disconnect win over a pending restart. A readiness
  outage cannot restart a healthy instance. QR-086 admits timing and
  bounded stop and restart policies, including exhaustion and restart
  accounting. Until then the automatic recovery route is unavailable.

### Licence and notices

- The first setup screen says in plain words that Sutradhar is free
  software under the Apache License 2.0, links the licence and shows the
  notices page: every third-party licence, from the SBOM.
- The add-ons are GPL-2.0-or-later and their zips contain their source.
- There is no source-offer duty for the app; spec 0020's source-offer footer
  is retired (QR-008).

### Update

1. Check: the app checks the update channel on a schedule (default daily,
   a setting) over GitHub Releases. GitHub is only a transport; nothing is
   trusted until verified.
2. Notice: a signed release manifest lists each part's hash, the changes,
   migrations with their measured time, the expected pause and any memory
   change. It is shown in the app and as a staff alert on each site.
3. The installation owner approves the exact current update plan under
   DR-149 and spec 0031. Use the same check for a downloaded bundle.
   Refuse an administrator who is not the installation owner with
   SUT-801. If the plan changed after preview, return SUT-803 and ask
   for a new preview and owner approval. Audit each outcome under
   spec 0020. A refused approval starts no update or write pause.
4. Pause: suggestions, backfill and jobs reach their safe points and stop;
   the review page says "Paused for update".
5. Backup check: the latest backup exists, is whole, encrypted and
   checksummed, and has passed a test restore. If not, a backup runs. If
   that is not possible, only the owner may override, with a typed reason,
   recorded. No code gate can be overridden.
6. Fetch and verify the payload by hash and signature; a failure stops
   before anything is switched.
7. Apply forward-only schema expansion. Keep the previous version working
   on the expanded tables within spec 0031's compatibility window.
   Contraction needs its own compatibility evidence. Table compatibility
   alone cannot admit rollback.
8. Start: the large parts (models, sidecar, jobs) stop before their new
   versions start so the budget is never passed; only small parts may run
   old and new together, and only when both fit.
9. Self-test. A pass allows the final switch and write resume only after
   the current lifecycle disk and authority checks pass. A failed
   self-test uses 0031's conditional fallback under the update lease.
   Before starting the previous payload, verify current release trust,
   exact schema, persisted state and format versions, recovery journal
   and every registered site's compatibility. Include the site register
   and owned files the previous version must read. After encryption cutover
   the build and store must still meet spec 0118. If any proof fails or
   is unavailable, keep writes paused and show the failed or unproved
   check. Keep the current data and recovery evidence. Emit
   `update.paused` and record the paused outcome. Record rollback as
   complete and emit `update.rolled_back` only on verified return. The
   three-minute target applies to compatible automatic rollback.

Before a remaining start, switch, write resume or fallback, recheck
current installation-owner authority, the exact plan and its operation
lease. If authority is lost after a migration commits, keep the committed
schema, exact migration receipt and update journal. Show “Update paused”
and “Current owner approval is needed for the remaining steps.” A verified
package, completed migration or earlier approval cannot authorise the
next effect. Follow the original recorded operation and reconcile only
under current read authority. Continue only the remaining authorised work
when the existing route offers an exact current owner confirmation for
that plan and checkpoint. Recheck all its other gates. Do not rerun the
committed migration or fall back under the withdrawn authority.

Also:

- The rollback button checks the same compatibility and trust conditions.
  The previous payload is kept until the next successful update. An old
  backup is never restored automatically and committed migrations never
  run backwards.
- PostgreSQL minor versions change with the payload. A major version changes
  only inside a release, by dump and restore after the backup check, named
  in the notice with its time.
- ganit output contracts carry versions. Parameters from the previous
  version are used only if the new contract accepts them; otherwise users
  keep safe defaults until the next nightly run. Rollback keeps the
  parameter registry, which only grows.
- After the update, ganit job 307 compares live figures before and after,
  and the screen shows any regression with the rollback button beside it.
- An update holds, and is not forced, when an enabled extension's contract
  range excludes the new version.
- An update can come from a downloaded package, for a computer with no
  internet.
- The local notice shows which verified component needs updating under the
  exact site compatibility plan. The legacy conversion in specs 0021 and
  0118 updates the service first and holds protected integration traffic
  until both ends prove the encrypted contract.
- Until 1.0.0 the 0.x releases are the only channel; from 1.0.0 the user
  may opt into a pre-release channel.

### Repair and reset

- Check files (see the ledger) and Restore program files. For a missing
  managed program file, bind the plan to the exact ledger entry, verified
  root, inspected absence and exact signed replacement bytes. Under the
  existing lifecycle lease, recheck that target at the write boundary.
  Use a qualified no-replace operation bound to the verified root and
  exact target. If the native profile cannot prove this, hold the
  repair. A replaced or newly occupied target gives SUT-4100.
  Keep it and report the discrepancy through Check files. A mismatched
  package or staged payload gives SUT-832 under the 0005 handoff.
  Record the verified owned effect before reporting success. On a lost
  reply, reconcile that exact intent and result. Matching current bytes
  alone cannot prove that the earlier repair ran.
- Clean up: removes orphan scratch files from interrupted runs and old
  updates, shown as a list first.
- Factory reset that keeps data: preview the exact eligible caches
  and ordinary settings for the confirmed scope under N-1463. Use
  spec 0014's typed inheritance for selected site overrides, through
  live confirmation of the exact owner-approved Repair plan.
  Validate the full plan before any cache or settings effect. An
  installation cache reset cannot change site settings. Keep scripts,
  site data, protected settings, settings history, trust, erasure and
  recovery records under their existing rules.
- Relink: if an external drive gets a new letter or path, the app finds the
  ledger entries by their group and offers to relink them.
- Move: an export of the home folder (settings, scripts, layout, optionally
  data and models) into one encrypted archive that imports on Linux or
  Windows, translating paths.

If the selected settings activation commits and cache cleanup stops,
the Repair journal keeps the exact original reset plan and references
to its verified 0014 activation and 0030 cache ledger receipts. The
activation identifies the site, settings domain, version, epoch and
sequence. The ledger proves each exact cache effect. The journal
keeps those references and safe outcomes within their existing
protected scope.

After checking current permission to read this plan, show “Settings
saved. Cache cleanup paused.” Give the safe reason and point to the
original reset result. Use 0014's role status for any setting waiting
to take effect. Only verified cleanup for the whole selected scope
completes the reset. An uncertain cleanup result stays under review
in the original plan.

Before any remaining effect, recheck the frozen plan, current owner
authority, lease, ledger and the exact cache identity. Reconcile an
uncertain cache effect before attempting the next one. Continue only
the remaining authorised cleanup. Keep the committed settings
version. A past activation does not authorise a fresh activation,
revert, cache copy or cleanup outside the selected scope.

#### Repair explanation, rehearsal and migration (N-1503–N-1513)

Use the existing verified installation, file ledger, repair and
move contracts. Bind the owner, installation, site, current
generation and exact scope before reading or proposing a change.
Use specs 0014, 0020, 0032 and 0118 for settings and protected
diagnostics. An owner-started local MCP session may produce an
eligible proposal and receives only its authorised projection.
It cannot change a fence, accept its own plan, write product
source, launch a repair or bypass the signed package and move
contracts. Plans and synthetic evidence confer no effect authority.

Use [BDD-0030-VERIFIED-FIRST-WINDOW][first-window-setup-case]
for the trusted first-window inputs in N-1507. Its local checklist
and draft still need the separate feature fixture. Use
[IT-0030-UPDATE-REPAIR][product-repair-case] for the signed product
repair path in N-1508. The extractor fixture separately tests
synthetic replay and the user-script proposal under spec 0104.

[first-window-setup-case]:
  ../planning/test-and-traceability.md#task-0030-first-window
[product-repair-case]:
  ../planning/test-and-traceability.md#task-0030-update-repair

| Feature | Exact local action or held route |
|---|---|
| N-1503 Setting conflict resolver | Identify incompatible typed settings and the exact affected revisions using current registry, ownership and dependency evidence. Propose permitted changes and their impact. If an owner-only or never-changeable value is involved, show a safe hold or its existing owner screen. Apply only through the action and live confirmation that own each permitted setting. [FX-0030-W5-SETTING-CONFLICT-REVIEW-ONLY](../planning/feature-wave-5.md#acceptance-design-for-n-1502-to-n-1525). |
| N-1504 Plain-English mystery solver | Explain the selected metric or stalled job from exact authorised diagnostics, audit-safe reason, current versions and known gaps. Inputs are inert and protected detail stays behind its own grant. Unknown evidence remains unknown. No open internal read, probing, private logs or change action. [FX-0030-W5-DIAGNOSTIC-EXACT-SITE-AND-PROOF](../planning/feature-wave-5.md#acceptance-design-for-n-1502-to-n-1525). |
| N-1505 Automatic pre-repair snapshot | After owner approval for an existing repair requiring a restore point, bind exact approved repair intent, settings and script preimage revisions, ledger, site, completed snapshot digest and recovery profile in a durable pre-effect receipt. Retry reuses that exact verified snapshot only. Changed state or revoked grant holds before effect. Use existing encryption, custody and retention. No raw keys or transfer. Hold a required snapshot whose durability or recoverability cannot be verified. The snapshot itself grants no restore or write. [FX-0030-W5-REPAIR-EXACT-VERIFIED-PREIMAGE](../planning/feature-wave-5.md#acceptance-design-for-n-1502-to-n-1525). |
| N-1506 Weekly automated checkup | Use an admitted owner-configured local schedule for bounded read-only checks of recorded disk, ledger, storage and script health. Use the scheduler's site, identity and occurrence fences. Report stale or missing facts. Opening or repeating a report cannot start an unattended agent, outside probe, package repair, send or settings change. [FX-0030-W5-READ-ONLY-CHECKUP-ONE-OCCURRENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1502-to-n-1525). |
| N-1507 Hands-free setup co-pilot | Offer a local manual checklist or a draft plan using the exact admitted wizard choices and trusted first-run facts. Verify the current device, selected destination and site through their owning actions. Review each eligible proposed effect through the existing owner flow. The assistant cannot choose credentials, pair a site, start an audio route or apply a setup plan. [FX-0030-W5-SETUP-CURRENT-LOCAL-DRAFT](../planning/feature-wave-5.md#acceptance-design-for-n-1502-to-n-1525). |
| N-1508 Broken-extractor rescue | Use eligible protected failure and already-held eligible same-site page evidence. Verify proposed changes in a compatible synthetic replay to propose a fix in the owning route. A user-script draft follows spec 0104's isolated checks and admission. Product extractor code follows the normal reviewed build and signed repair route. No arbitrary remote sample, live source rewrite, automatic activation or job resume. [FX-0030-W5-EXTRACTOR-SYNTHETIC-OWNING-REPAIR](../planning/feature-wave-5.md#acceptance-design-for-n-1502-to-n-1525). |
| N-1509 Self-contained library healer | Diagnose the exact missing package or version in a user-script environment. A targeted replacement proposal names the already admitted package identity, hash, licence and environment. Existing 0104 owner approval and package admission govern installation. A product dependency is restored from signed package bytes. An assistant cannot download, install or resolve new dependencies. [FX-0030-W5-ADMITTED-PACKAGE-REPAIR-PROPOSAL](../planning/feature-wave-5.md#acceptance-design-for-n-1502-to-n-1525). |
| N-1510 Live repair subtitles | Render short progress from current protected repair receipts and authorised phases. Distinguish proposed, queued, verified, held and uncertain. Use accessible text without internal addresses, raw refusal text or secret values. A subtitle cannot act, claim success before read-back or keep an obsolete run visible after a grant change. [FX-0030-W5-REPAIR-SUBTITLES-VERIFIED-RECEIPT](../planning/feature-wave-5.md#acceptance-design-for-n-1502-to-n-1525). |
| N-1511 Safe clone rehearsal | Model the selected settings, script interfaces and repair plan in a bounded synthetic isolated fixture. Use a new scratch scope and symbolic local references. Omit real data, credentials, live grants, schedules and callbacks. Discard scratch after completion or failure. A pass proves only the recorded fixture and gives no production repair or move authority. [FX-0030-W5-REHEARSAL-SYNTHETIC-FENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1502-to-n-1525). |
| N-1512 Linux-to-Windows migration assistant | Preview a typed path and storage translation for the exact eligible 0030 encrypted home export and current destination profile. Detect collisions, escaping paths, forbidden values, missing destination capabilities, mismatched ledger and retained-data or erasure fences. Exclude install and site keys, tokens and active grants. Re-enter eligible credentials through existing owner custody on a verified destination after independent first trust. The owner confirms the existing import after offline compatibility proof. Translation only proposes values. Never revive session tokens, old startup entries, cloud routes or prior install identity. [FX-0030-W5-MIGRATION-PREVIEW-NO-RESTORED-AUTHORITY](../planning/feature-wave-5.md#acceptance-design-for-n-1502-to-n-1525). |
| N-1513 Emergency rescue backdoor | No backdoor, extra listener, remote bridge, alternate credential or grant bypass. Offer the existing signed repair or local owner-only recovery information if available. An unreachable screen or local door holds any remote or MCP request. Recovery requires its own trusted installation and authorised path. [FX-0030-W5-SIGNED-RECOVERY-NO-ALTERNATE-DOOR](../planning/feature-wave-5.md#acceptance-design-for-n-1502-to-n-1525). |

Use 0030's current signed repair, update, confirmation, move and
recovery verification. QR-204 binds synthetic interruption,
erasure, stale manifest and no-effect fixtures. Live scope or
authority outside these paths requires a separate decision.

### Backup and restore

- Nightly and before each update: a consistent PostgreSQL dump (custom
  format, with roles and RLS policies), which also holds ganit's parameter
  registry, plus the home folder's owned files. Compressed with zstd and
  checksummed with BLAKE3.
- Every backup is encrypted on the PC before it goes anywhere. The owner
  holds the key; with no key there is no off-PC backup. Targets: a local
  folder, an external drive or the user's own S3-compatible bucket
  (spec 0038).
- Left out: secrets (listed for re-entry), caches and telemetry, ganit's
  working snapshots and model files (downloaded and verified again). The
  vector index is included when disk allows.
- A test restore into a temporary install runs weekly in CI on synthetic
  data, and monthly on the user's PC when the scheduler allows. Its result
  feeds the update's backup check.
- A restore passes only if every table again has RLS enabled and forced
  with its policies, and no role has `BYPASSRLS`.
- Backups and exports hold member data. Only the user's account can read
  them, they are never published, and erased data leaves them as the
  retention rules set.

### Disable, enable and remove

Disable and enable follow the models design: every part stops within 90
seconds and resumes where it was.

Remove, from the app or from a site's add-on page for that site:

1. The Remove screen reads the ledger and shows every file group with
   its size, wherever it lives (the home folder, an external drive or a
   bucket):
   - always deleted: engine runtimes, embeddings, indexes, model files,
     caches, logs, the start-up entry and shortcuts;
   - the user's choice, each with Export and Keep or Delete: settings,
     scripts, suggestion history, audit, analytics, backups and user data.
2. Export runs with progress, checksums and a link.
3. The supervisor stops all parts and tasks, and a running backup is
   cancelled at its safe point first.
4. Everything marked Delete is removed in ledger order; cloud objects only
   if chosen. PostgreSQL data goes unless some data was kept, in which case
   only the kept tables remain, with their RLS policies.
5. A second pass compares the ledger with the disk and the process list.
   Verify the exact approved storage and affected managed copy when
   reconciling any lost deletion result. An empty path after a volume
   disconnect supplies no absence proof for that volume. Keep the
   group unresolved and preserve the previous verified results.
   Retain the exact plan and lifecycle journal through the hold,
   including when the optional final receipt was declined. Reconcile
   the original effect under current authority before any retry.
   Withhold completion and `uninstall.finished` until the required
   managed Delete scope and existing stop checks are verified.
6. The receipt counts removed files, folders, tasks and start-up
   entries in the exact plan. List kept, still present or unverified
   items and their reasons. Say “None remain” only for a reconciled
   empty managed Delete class. Previously downloaded copies outside
   the plan stay with their recipients. The receipt itself is the
   last file, kept only if the user asks.
7. Approved links stay in the sites' posts.

Removing the `.deb` or the Windows program through the system's own tool
removes program files only and leaves the home folder; the Remove screen is
the complete route. When a desktop session is available the package's
removal step opens that screen first (proposed).

### Files screen

- X-ray storage map: a picture of every folder, its size and a note on what
  it does.
- File inspector: click a database, cache or settings file to see a plain
  summary of what is inside.
- Disk activity: an opt-in live list of what Sutradhar reads, writes and
  deletes.
- Disk cap: a simple limit such as "never more than 50 GB"; jobs that would
  pass it wait (spec 0078).
- Permission prompts: before Sutradhar touches any folder outside its home,
  the screen shows the exact path and the reason.

#### Project location, file lock and cleanup (N-1479, N-1481 and N-1501)

Bind the exact installation ledger revision, active registered site,
owner grant and selected data class. The home-folder and site register
remain authoritative. Recheck protection, current owner approval and
erasure before displaying or using any prepared action.

| Feature | Exact storage or cleanup scope |
|---|---|
| N-1479 Per-project home picker | Show a site's approved local placement and eligible separate encrypted archive or backup destination. Keep the installation's single home and separate site register, not a live cloud store. A new managed location goes through the existing owner-selected 0038 verified move and 0030 ledger. Check exact project, data class, bounds and disconnected drive state. Picking a destination stages a plan only. [FX-0030-W5-LOCAL-SITE-HOME-PROPOSAL](../planning/feature-wave-5.md#acceptance-design-for-n-1478-to-n-1501) |
| N-1481 One-switch file lock | Show mandatory protection status and the existing authorised storage or export custody options. Protected data stays sealed before leaving its permitted process, storage or transport boundary. Owner credential or key choices use 0065 and 0118. A lock setting cannot weaken mandatory protection, authorise plaintext, change egress, expose a key or perform a migration. Show unavailable if the required custody cannot be proved. [FX-0030-W5-MANDATORY-CUSTODY-FILE-LOCK](../planning/feature-wave-5.md#acceptance-design-for-n-1478-to-n-1501) |
| N-1501 Storage cleanup assistant | Read an eligible, bounded ledger projection. Suggest disposable or permitted stale artefacts with exact identities, class, size, retention and reasons. Check current references, legal or audit retention, pending work, managed scope and Keep decisions. Unknown or replaced paths and protected records are never deletion targets. Any eligible cleanup uses the existing owner-confirmed exact deletion plan and receipt. An assistant cannot confirm or delete. [FX-0030-W5-CLEANUP-EXACT-OWNER-LEDGER-SCOPE](../planning/feature-wave-5.md#acceptance-design-for-n-1478-to-n-1501). |

QR-203 qualifies target and key identities, protected relocation and
cleanup scope, interruption, replacement, erasure and replay. These
views introduce no new storage provider or deletion route.

#### Managed file origin (N-1870)

Reissue the current permitted file-ledger projection as the file's
plain-English record of origin. Bind the install, managed group,
root and verified identity, site where relevant, producer and
safe purpose. Create a durable write intent before the owned
effect and record the resulting verified object and outcome
before exposing it as committed. Mark pending, unknown, replaced
and missing entries distinctly. Reconcile on restart and restore
against the same authority and receipt.

The ledger covers managed objects, shortcuts and entries only.
It gives no system scan, right to browse file contents or
permission to delete. Logs follow the owner-only folder and
sealed manifest contract, with no ledger row per rotation.
Show only current allowed fields and omit sensitive locations
and provenance on an ineligible view. Use the ledger and
removal tests 20 and 21, and QR-219.
See [FX-0030-W5-VERIFIED-MANAGED-FILE-ORIGIN][w5-file-origin-case].

[w5-file-origin-case]:
  ../planning/feature-wave-5.md#acceptance-design-for-n-1863-to-n-1880

### Data

In PostgreSQL with RLS enabled and forced; only the setup role writes
these tables.

- `install_state`: version, payload hashes, step, checkpoint, started, by.
- `releases_seen`: version, manifest hash, notice time, decision.
- `update_runs`: from, to, approved by, override reason (or none), steps
  with times, result, rolled back.
- `backups`: id, kind, size, BLAKE3 hash, location, verified restore at.
- `ledger_entries`: the mirror of the file ledger.
- `uninstall_receipt`: groups with keep or delete, export hashes, counts.

The six names above are the closed setup metadata catalogue under
spec 0108. The installation scope covers only the operations below.

| Installation operation | Exact eligible database records |
| --- | --- |
| First setup | The current installation's `install_state` checkpoint, the `releases_seen` row for the exact verified release, and the `ledger_entries` mirror of committed installation-owned setup entries |
| Approved pre-site migration | The scoped first-setup records for the exact migration and its installation-owned `update_runs` checkpoint where it uses one |
| Owner-approved app update after enrolment | The installation-owned `update_runs` checkpoint for the exact approved release and step |
| Schema and migration history | The migrator's distinct scope under spec 0108. This route does not write setup lifecycle rows. |

Each admitted operation exposes only the named records for its
exact current installation. Backup and removal receipt routes keep
their existing authority outside those steps. Site-owned rows in any
catalogue table require exact site scope. An installation context gives
no site reads, credential lookup, connector call, backlog job, site
write or native effect. Missing, mixed or wrong scope gives no protected
rows or change under spec 0108.

### Actions and events

- Actions: `sutradhar.preflight`, `.install`, `.update.notice`,
  `.update.approve`, `.rollback`, `.disable`, `.enable`, `.backup.run`,
  `.backup.restore`, `.repair.check`, `.repair.reset`, `.move.export`,
  `.move.import`, `.uninstall.plan`, `.uninstall.apply`. Each is a
  management API route checked by Cedar, a screen action and a `sutra`
  command. For `sutradhar.update.approve`, CAP-002 checks the current
  installation owner and exact plan under DR-149. Apply SUT-801 and
  SUT-803 as in Update step 3.
- Events: `install.step`, `update.available`, `update.paused`,
  `update.finished`, `update.rolled_back`, `backup.verified`,
  `ledger.mismatch`, `uninstall.finished`.
- Capabilities: setup (CAP-001), update notice and approval (CAP-002),
  rollback (CAP-003), disable and enable (CAP-004), removal (CAP-005),
  restore (CAP-031); preflight, backups, repair, move and the Files screen
  each a new entry.

### Failure codes

SUT-830 preflight failed (each check named); SUT-831 engine supervisor
unreachable; SUT-832 payload hash or signature mismatch; SUT-833 migration
failed, nothing switched; SUT-834 health check failed at start; SUT-835
self-test failed, verified rollback completed; SUT-836 no verified backup
(owner override possible); SUT-837 budget would be passed; SUT-838 add-on
and service versions do not match, and the screen says which side to
update; SUT-839 export failed, nothing deleted; SUT-4100 ledger does
not match the disk;
SUT-4101 home folder missing or not writable (for example an unplugged
drive); SUT-4102 update source unreachable; SUT-4103 web view missing. Any
failure leaves the previous working state or the paused state, never a
half-switched one.

### Security and privacy

- The supervisor acts only on Sutradhar's own parts, from signed requests,
  with fixed verbs, as the user.
- Verify the tijori-signed release manifest and exact package and
  required member bytes under spec 0005. On Windows, independently
  verify the unsigned wrapper against the signed manifest before
  launch. Verify the payload and required components before their
  first load or execution. Keep the unsigned wrapper warning.
- The migrate role alone owns the schema and is never used at run time.
- No step sends site content off the PC unless the user chose a bucket, and
  then only encrypted. Update checks send no personal data.
- The override needs the owner role. Logs mask secrets.

### Linux package trust and handoff

Use the
[release profile](0005-release-coordination.md#detached-delivery-profile).
Before initial install, update or repair, verify the signed manifest,
exact complete .deb and required members in an independently trusted
graphical route. Use the same immutable staged package through the
native installer and its privilege boundary.

For an update, bind the current owner-approved plan's release,
target and artefacts to the admitted native dependency profile.
Before unpacking, installing dependencies or running scripts,
verify the native resolution against that profile. At native use,
prove that the installer consumes that exact verified resolution
alongside the same immutable staged package. If resolution runs
again, verify its current result under the same plan and at the
same boundary before any of those effects. Matching displayed
package names and versions or an earlier plan do not prove the
current native resolution. A verified resolver result remains
evidence for its own exact native input until an effect is proved.
If the approved plan itself changes,
use the existing 0031 approval route.

At native delegation, recheck the existing native permission for
the exact action and target under the 0005 handoff procedure.
Earlier native preflight and current owner approval do not prove
that permission. If it is denied or cannot be proved, retain the
reason and hold before privileged package effects. Keep any
verified checkpoint under the existing reconciliation rules.

A substitute, unchecked handoff or unadmitted profile gives
SUT-832 and no install effect. For the two site ZIPs, use the
verified native handoff in spec 0031.

### Windows first trust check

The first install uses a platform-admitted graphical verification
route whose release key is pinned independently of the unsigned
installer. Check the installer digest in the signed manifest before
launch. The warning explains the unsigned installer and what was
verified. A key or verifier supplied only by the installer cannot
establish trust.

Check the signed manifest and exact payload bytes before running
bundled or fetched parts, including the required native and WebView
components. Bind verification to the exact release and artefacts in
the current plan for the applicable route. Proof for another release
gives no permission to use its bytes under this plan. Each admitted
handoff must bind the bytes used for launch or first load to the
exact independently verified wrapper or verified member. A prior
verification receipt alone supplies no proof of the bytes at use.
If the match is unproved or the bytes differ, hold the affected
launch or load with SUT-832. Keep the plan on its chosen release.
A refused wrapper starts no installer. A refused member runs no
code from that member and admits
no later effect or setup success from its held step. Keep any earlier
verified effects for the existing recovery path.

Updates, repair and portable first launch follow the same rule.
QR-202 qualifies this independent first-run path. QR-099 qualifies
the protected build cutover separately. Without first-run evidence the
Windows package cannot pass the gate. Test a modified installer,
changed payload, substituted key, unverified download and stale
portable manifest. In IT-0030-WINDOWS-FIRST-TRUST, keep the verified
manifest and earlier receipt, then replace an inert verified wrapper
before launch. Separately replace a required member after verification
and before its first load, with the original wrapper unchanged.
Keep the plan fixed on a synthetic release B. Give the verifier an
otherwise eligible release A with its own complete genuine proof.
In a separate case, verify B and offer the genuine A wrapper and its
proof at the graphical handoff. Reject both with SUT-832 under the
unchanged B plan, before launching any wrapper. Refuse the changed
bytes cases before their affected execution. The unchanged B control
can proceed with the exact verified bytes through the other checks.

In the same test, exercise each declared bundled and downloaded
component route separately. Keep the exact release, wrapper where
used, owner action and verified stage. After a downloaded required
native or WebView component is verified, substitute its bytes before
first load. In a separate case keep those bytes and withhold current
use-time proof. The existing SUT-832 refusal prevents that component
loading or running and holds later effects from its step. Compare a
matching verified control. A test of a bundled member alone supplies
no downloaded-member evidence. Keep the existing missing WebView
behaviour for a genuinely missing prerequisite.

Bind the fixture separately to initial installation, the exact
owner-approved update plan, the existing approved repair intent,
and portable first launch with its current trusted manifest. Use
only components and steps declared for that exact lifecycle route.
Check the required wrapper or component substitution and an otherwise
eligible exact control at its own launch or first-load boundary.
A prior verified effect stays in its original recovery record. The
refusal records no success for the affected step. Portable first
launch creates no installer effect through this proof.

Separately interrupt initial setup after the unsigned wrapper's
verified launch and an earlier setup checkpoint, but before the
first load of a declared required component. Keep the selected
release, exact staged bytes, verified checkpoint and target fixed.
The current synthetic policy still admits those artefacts, but the
resumed handoff offers only a first-load verification from the
previous policy epoch. Under 0005 re-establish the current file and
policy binding. Until proved, report SUT-832 on that same step
after each restart. Keep the prior verified effect and data with
their original recovery record. Do not repeat a verified setup
effect, load the component or record setup success.
The exact current binding permits only the still-unperformed step
under the remaining setup checks.

Independently offer a genuine synthetic QR-099 protected build
result for the same candidate and declared Windows target under its
own admitted build profile. Withhold QR-202 independent native
first-trust proof. Keep SUT-832 and hold before
launch or first load. The build proof remains usable only within
its own scope. An exact independently verified QR-202 control can
continue under the other applicable checks. The wrapper stays
unsigned and its existing warning remains on the applicable route.
Use no live download, installer or protected release key.

### Native package handoff trace

These identities bind the existing
[Linux handoff](#linux-package-trust-and-handoff) and
[Windows first trust](#windows-first-trust-check) to their cases.

| Owning requirement | Owning design and exact clause | Existing planned case |
| --- | --- | --- |
| `REQ-0030-LINUX-PACKAGE-TRUST` | `DES-0030-LINUX-PACKAGE-HANDOFF` uses independent graphical verification for the selected Linux package, architecture, release role and required members. | `BDD-0005-DETACHED-EXACT`, `IT-0005-DETACHED-MIXUP`, `IT-0005-DETACHED-TRUST` |
| `REQ-0030-LINUX-IMMUTABLE-USE` | `DES-0030-LINUX-PACKAGE-HANDOFF` preserves the verified immutable stage through privilege change and native consumption. | `IT-0030-VERIFIED-STAGE` source or stage substitution, privilege boundary and exact-effect reconciliation |
| `REQ-0030-LINUX-RESOLUTION-PROOF` | `DES-0030-LINUX-NATIVE-RESOLUTION` verifies exact current dependency resolution and binds the actual use before effects under the approved action. | `IT-0030-VERIFIED-STAGE` out-of-profile result, identical visible plan without proof and post-check substitution |
| `REQ-0030-LINUX-HANDOFF-PROOF` | `DES-0030-LINUX-PACKAGE-HANDOFF` holds a managed handoff backed only by direct package open or repository signing proof. | `IT-0030-VERIFIED-STAGE` each insufficient source alone, with no managed native effect |
| `REQ-0030-LINUX-NATIVE-PREFLIGHT` | `DES-0030-LINUX-NATIVE-RESOLUTION` keeps the native package preflight and its refusal independently binding with valid package and resolver results. | `IT-0030-VERIFIED-STAGE` native refusal before unpack, scripts or activation |
| `REQ-0030-WINDOWS-INDEPENDENT-TRUST` | `DES-0030-WINDOWS-FIRST-TRUST` requires the independent graphical verifier and trust root before wrapper execution. | `IT-0030-WINDOWS-FIRST-TRUST` wrapper-supplied verifier or key refusal |
| `REQ-0030-WINDOWS-EXACT-WRAPPER` | `DES-0030-WINDOWS-FIRST-TRUST` binds the selected release, exact unsigned wrapper and its native launch. | `IT-0030-WINDOWS-FIRST-TRUST` swapped wrapper, release A offered under plan B and exact B control |
| `REQ-0030-WINDOWS-UNSIGNED-WARNING` | `DES-0030-WINDOWS-FIRST-TRUST` presents the unsigned-wrapper warning and what the independent check verified. | `IT-0030-WINDOWS-FIRST-TRUST` warning on the applicable wrapper route and 0004 `MAN-PLATFORM` |
| `REQ-0030-WINDOWS-COMPONENT-USE` | `DES-0030-WINDOWS-LIFECYCLE-HANDOFF` binds the actual bundled or declared downloaded native and WebView input before its first use. | `IT-0030-WINDOWS-FIRST-TRUST` altered required member, unchanged downloaded bytes without current proof and exact control |
| `REQ-0030-WINDOWS-LIFECYCLE-AUTHORITY` | `DES-0030-WINDOWS-LIFECYCLE-HANDOFF` checks native use against the current exact installation, update, repair or portable first launch route. | `IT-0030-WINDOWS-FIRST-TRUST` independent lifecycle branches and existing protected action checks |

0005 defines the shared stage and release proof cases. The two 0030
native tests above bind their expected effects to those cases without
moving or replacing a shared check. `TASK-0030-PLATFORM-INSTALL` owns
initial and portable first-use evidence. `TASK-0030-UPDATE-REPAIR`
owns evidence for the exact authorised update and repair actions.
Resolve the separate 0005 `REQ-0005-WINDOWS-TRUST-SEPARATION` at the
Windows first-trust boundary. The current 0031 approval still applies.

| Variant identity | Applicability and exact evidence |
| --- | --- |
| `VAR-0030-LINUX-INITIAL-INSTALL` | First use under the admitted independent installer and setup authority. Bind exact handoff to `TASK-0030-PLATFORM-INSTALL`. |
| `VAR-0030-LINUX-UPDATE` | Current installation, exact approved update plan and lifecycle lease. Bind exact handoff to `TASK-0030-UPDATE-REPAIR`. |
| `VAR-0030-LINUX-REPAIR` | Exact approved repair intent on the admitted route. Run the dependency cases only where that route resolves dependencies. |
| `VAR-0030-WINDOWS-INITIAL-INSTALL` | Independently verified wrapper and admitted Windows target. Bind launch proof to `TASK-0030-PLATFORM-INSTALL`. |
| `VAR-0030-WINDOWS-UPDATE` | Exact owner-approved update plan and applicable verified wrapper or required component. |
| `VAR-0030-WINDOWS-REPAIR` | Exact repair intent and required verified native input for its affected step. |
| `VAR-0030-WINDOWS-PORTABLE-FIRST-LAUNCH` | Current independently trusted portable manifest and eligible load or launch. No installer effect follows from this proof. |
| `VAR-0030-WINDOWS-BUNDLED-COMPONENT` | Required member supplied in the admitted bundle. Verify its retained use-time identity. |
| `VAR-0030-WINDOWS-DOWNLOADED-COMPONENT` | Required native or WebView member declared as downloaded by the admitted route. Verify its use-time proof separately. |

Use only applicable host, lifecycle and component combinations.
Ubuntu 26.04 and Debian 13 each need a distinct host fixture cell.
They use the same Linux variant when the native path and observable
contract agree. QR-202 owns exact native proof for each. QR-204
retains exact repair admission. The IDs identify existing duties
and planned tests. Runtime and signed acceptance evidence remain
separate.

### Screens

WCAG 2.2 AA. Each step is a numbered list item with its state in words
("Running", "Failed: SUT-834"); progress is a bar with text; the Remove
choices are grouped radio buttons with a fieldset and legend per group,
set to "Keep" so nothing is deleted by an unread default; the final delete
button repeats what it deletes; the override reason is a labelled text area
with its purpose stated. The same screens look the same on Linux and
Windows. The add-ons' setup pages use the platform's own admin styles and
meet the same rules.

Show the current recorded update outcome in the owner view. If fallback
fails its compatibility checks or remains unproved under spec 0031,
show "Update paused" and "Sutradhar cannot safely start the previous
version." Name the failed or unproved check in plain English. For
example, say "The previous version cannot read the current data format"
when that incompatibility is proved. Show which work is paused and that
the current data and recorded work are kept. Keep protections in force.
Keep Resume and Rollback unavailable while the failure or missing proof
keeps the update paused.
Offer the verified recovery guide matching the release and held step
in the original recorded update. With the engine or window down, use
that guide on verified offline media without launching the installer.
A verified guide for the previous payload supplies no proof for the
held step. If the exact match cannot be verified, keep the update
paused and leave Resume and Rollback unavailable.
In Update paused recovery help, offer the existing Restore action
after reconciling the held update and only if the verified profile
offers an isolated test under 0031. Show its target and ask the owner to
confirm that test plan. Report the test result separately in Restore.
The live update stays paused, Resume and Rollback stay unavailable,
and the previous version stays stopped on the live installation.

Show "Previous version restored" only after verified return. Use
SUT-835 when a failed update self-test led to that verified rollback.
Show elapsed time only with admitted recovery timing evidence. If the
measurement is unavailable, say so. Require a verified runtime result
even when the previous package and a timing reading are available.
Show the step result and reason in accessible text as well as any
progress indicator.

#### Removal held

Show the failed check in text and offer only an authorised current
action. Use these messages with the existing refusal codes:

| Condition | Message and next step |
| --- | --- |
| An unstarted plan no longer matches the current release, site list or ledger revision (0020, SUT-803) | “Things changed since you previewed this. Preview again.” Open Remove for a fresh plan and current confirmation. |
| The fixed-site request crosses sites (0102, SUT-1901) | “This request is limited to one site. Open the intended site in the desktop app to review removal.” |
| The named site differs from the current desktop window (0102, SUT-1904) | “The site in this request does not match this window. Open the intended site and review removal again.” |
| An item fails the ledger-to-disk check (SUT-4100) | “Removal paused. The item on disk does not match its removal record. Use Check files.” Offer verified Relink only if the exact drive is recognised under its existing check. |

A rejected request removes nothing further. If earlier removals are
already verified, keep those results visible alongside the hold.
Resolve an uncertain previous effect against the original plan
before offering any retry or a new confirmation. Otherwise keep
removal paused. Do not show an unrelated site's details or a
private path in the refusal text.

#### Cross-platform views, local health and removal (N-1445 to N-1453)

Use the admitted Windows and Linux profiles under ADR-0021 and the
existing supervisor, ledger and Remove plan. Pin current installation
and site identity. Display safe facts and current evidence only.

| Feature | Exact view, observation or removal contract | Owning contract |
|---|---|---|
| N-1445 Identical look on every computer | Use the same admitted SDS Jobs view and status semantics, keyboard flow and accessible choices on Ubuntu, Debian and Windows. Use the owner-only Unix socket or protected Windows named pipe per 0020/0118. No fallback listener. Reject stale site or path generations after migration. Present platform-specific capabilities accurately. Layout parity cannot enable an unsupported operation or require identical system chrome. | 0030 Screens, test 16 and 0099 JW-03 · [FX-0030-W5-ACCESSIBLE-PLATFORM-VIEW-PARITY](../planning/feature-wave-5.md#acceptance-design-for-n-1430-to-n-1453) |
| N-1448 Collapsible colour folders | Use local saved display groups of currently eligible jobs. Pair colour with text, counts and keyboard controls, and keep an accessible focus order. Recheck viewer scope before counts and saved selections. Folding groups or restoring display preferences cannot start, stop or move jobs, rearrange priority or reveal a hidden group. | 0030 Screens, test 16 and 0099 JW-03/JW-16 · [FX-0030-W5-SCOPED-ACCESSIBLE-GROUP-FOLD](../planning/feature-wave-5.md#acceptance-design-for-n-1430-to-n-1453) |
| N-1451 Three-second system doctor | Read bounded local health evidence for the exact supervisor instance, scheduler, available memory, disk and eligible sidecar state through 0106, 0112 and 0032. Show the checked scope, age and time budget. Three seconds is a qualification target. A late, absent or stale check is unavailable. Inspection starts no new worker, remote connection, repair or restart. | 0030 Preflight and Engine supervisor, QR-086 · [FX-0030-W5-CURRENT-BOUNDED-LOCAL-DOCTOR](../planning/feature-wave-5.md#acceptance-design-for-n-1430-to-n-1453) |
| N-1452 Zero-trace uninstall sweep | Bind the durable owner-confirmed plan and receipt to exact install, site or whole-install target, ledger revision, ordered identities, group choices and approved export hash and custody. Stop backup at a safe boundary. Reach the supervisor safe stop first. Delete only the ledger identities marked Delete, and at deletion recheck canonical owner root and file identity. Refuse substituted, symlink or reparse paths. Keep unknown or replaced entries out of the sweep and preserve Keep groups. Keep the original plan until interruptions reconcile. Recheck ledger paths, owned process, startup or task entries on the second pass. Preserve kept PostgreSQL groups with their RLS and approved live links. Cloud deletion needs an explicit choice via the owning route. Keep the final receipt only if requested. Summarise any remaining items. The result covers managed artefacts only and does not prove physical overwriting. | 0030 file ledger and Disable, enable and remove, tests 13/14/20/21/26/27 · [FX-0030-W5-EXACT-RESUMABLE-LEDGER-SWEEP](../planning/feature-wave-5.md#acceptance-design-for-n-1430-to-n-1453) |
| N-1453 Interactive uninstall checklist | Show eligible current ledger groups by location, size and purpose, with always-deleted groups distinguished from optional Export and Keep or Delete groups. Optional groups default to Keep. Preview the exact target and consequences before the existing owner confirmation. A changed group, ledger revision, site, running effect or missing required export holds the plan. Do not use package-manager removal as the complete route or touch another site's data. | 0030 Disable, enable and remove and Screens, tests 13/14/21/26/27 · [FX-0030-W5-CONFIRMED-REMOVE-KEEP-DEFAULT](../planning/feature-wave-5.md#acceptance-design-for-n-1430-to-n-1453) |

After an update or a site retirement, use the verified current
release, native register generation and file ledger revision for
Remove. Before removal begins, a change to the preview baseline
makes `sutradhar.uninstall.apply` refuse the stale plan
with 0020's SUT-803 before pausing work or deleting anything.
Require a fresh preview and current confirmation.
A removal that has started reconciles its own verified steps
against the original plan. Those steps cannot add scope. Use
0031's installation authority and separate confirmation for each
affected site on the whole-install route. Each PostgreSQL site-data
step uses the exact site scope under 0108. DR-150's setup and
migration context carries no removal authority. A retired site's
old scope cannot revive its access or publish its tile.

QR-201 qualifies the platform parity, health timing and signed
confirmation, path replacement, interrupted removal, Keep and
export fixtures. The diagnostics add no public-page or telemetry work.

### Managed installation, files and portability (N-1454–N-1468)

Use the existing 0030 home, file ledger and supervisor. Bind each
action to the exact installation, owner, managed root, admitted
platform, expected ledger generation, intent and safe receipt.
Register durable intent before a new write and prove the resulting
identity and receipt before release. Resolve
only admitted roots and handles, and compare identity and authority
immediately before effect. Hold if a link, mount, permission,
owner, process, root or generation changes. The supervisor,
settings registry and owning recovery actions keep their existing
authority. A preview, file description, portable layout or restored
record grants no install, launch, deletion, access or migration
right. Use 0032/0118 for protected records, erasure and hand-off.

| Owner catalogue entry | Exact scope |
| --- | --- |
| N-1454 No scattered Windows registry clutter | Store product settings under the managed home through the registered settings path. Keep the single standard uninstaller registration unavailable unless the owner approves QR-015 and the exact installer profile and ledger are admitted. Portable installs create no uninstaller registration. No extra settings, autorun, system policy or other registry entries. The registered Startup shortcut and portable mode follow their own existing rules. [FX-0030-W5-APPROVED-STANDARD-UNINSTALL-ENTRY](../planning/feature-wave-5.md#acceptance-design-for-n-1454-to-n-1477). |
| N-1455 Single-folder portable mode | Use the existing self-contained portable installation on an admitted volume and platform. Verify the release manifest, payload and independent first trust where required before launch. Bind one owner and process instance to the home. No startup registration. Drain owned work on graceful close or requested eject. Fence work when the mount disappears unexpectedly and hold relaunch until exact volume and ledger recovery. Hold unsupported permissions, media or executable profile. [FX-0030-W5-PORTABLE-VERIFIED-VOLUME-AND-EJECT](../planning/feature-wave-5.md#acceptance-design-for-n-1454-to-n-1477). |
| N-1456 X-ray storage map | Project ledger-owned groups and the admitted current size view, with descriptions, units, time and source. Distinguish measured, reserved, missing, kept and unknown items. No recursive inspection of an unapproved folder. Provide the same permitted information in the accessible list. [FX-0030-W5-STORAGE-MAP-CURRENT-OWNED-GROUPS](../planning/feature-wave-5.md#acceptance-design-for-n-1454-to-n-1477). |
| N-1457 Live disk-activity window | Present opt-in bounded local observations from the existing managed file and worker events. Show eligible operation, managed group or safe relative label, scope, time and gap state. Disclose coverage; this is not a claim about every OS file access. No global file watcher, protected contents, another site's labels or off-PC log. [FX-0030-W5-DISK-EVENTS-CURRENT-OPT-IN-SCOPE](../planning/feature-wave-5.md#acceptance-design-for-n-1454-to-n-1477). |
| N-1458 Zero-surprise folder permissions | Show the exact destination and purpose in the owner view before any new out-of-root access, with operations, current scope and expiry. Bind the approval to the resolved managed volume and root. A prompt cannot change OS permissions, elevate the agent or follow an unapproved link. Denial or changed path, identity or rights holds the operation. [FX-0030-W5-DESTINATION-APPROVAL-EXACT-ROOT](../planning/feature-wave-5.md#acceptance-design-for-n-1454-to-n-1477). |
| N-1459 Everything bundled inside | Ship each required runtime, library and helper only through the exact approved package and platform inventory. Verify manifest and payload hashes, licence and SBOM, compatible native and WebView prerequisites and the admitted 0104 sidecar and its pinned bundled Python. The Windows first-trust check uses independent admission under QR-202 and the 0030 first-run contract. QR-099's build proof alone cannot admit first launch. Missing or unverified dependencies hold. No host-wide setup, arbitrary download or helper execution outside its admitted profile. [FX-0030-W5-OFFLINE-PACKAGE-INDEPENDENT-TRUST](../planning/feature-wave-5.md#acceptance-design-for-n-1454-to-n-1477). |
| N-1460 Orphan file sweeper | Offer the existing cleanup preview for proven ledger-owned orphan scratch or eligible update leftovers only. Name the group, owner, age and safe reason. Recheck scratch identity, closed or stale owner generation, file handles, lease, manifest, references and unresolved effects immediately before unlink through the verified managed root. Refuse links, reparse points or replacement. Unknown, active or protected state holds. Apply the exact current approved plan, with a ledger receipt and replay reconciliation. [FX-0030-W5-ORPHAN-SWEEP-VERIFIED-UNLINK](../planning/feature-wave-5.md#acceptance-design-for-n-1454-to-n-1477). |
| N-1461 Matching folder layouts on Windows and Linux | Use one logical, relative managed home layout on Linux and Windows. Validate per-platform roots, path and case rules, file ownership and compatible versions. Same group names do not make binaries, keys or scripts interchangeable. Preserve package and migration gates. No alternate namespace or implicit file relink. [FX-0030-W5-PORTABLE-LAYOUT-COMPATIBLE-PATHS](../planning/feature-wave-5.md#acceptance-design-for-n-1454-to-n-1477). |
| N-1462 Plain-English file inspector | Show allowed facts derived from the verified ledger and admitted summary fields for the exact managed database, settings or cache entry. Explain purpose, scope and freshness without opening a raw data browser, executing code, showing secrets or reading a replaced or unknown file. [FX-0030-W5-INSPECTOR-VERIFIED-LEDGER-SUMMARY](../planning/feature-wave-5.md#acceptance-design-for-n-1454-to-n-1477). |
| N-1463 One-click factory reset that keeps data | Preview exact classes and effects. Use existing reset and settings authority for eligible caches and ordinary settings. Preserve project data, scripts, approved backup evidence, audit, erasure and trust history. Fence affected work and old settings and outputs before publishing the exact eligible configuration generation with current erasure state. Do not erase a protected setting, reinstall a trust root or restore previous access by resetting defaults. [FX-0030-W5-RESET-APPROVED-CLASS-AND-FENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1454-to-n-1477). |
| N-1464 Uninstall completion receipt | Project the existing N-1452 and N-1453 completion only after verifying stop and ledger reconciliation. Use the same Remove plan and export or Keep choices. Report exact removed, kept, unreachable and uncertain groups separately for managed files, processes and entries. State that none remain only for a reconciled empty class. Retain a durable final local receipt only on owner request. Report it separately as a retained receipt. No wider OS scan, native undo or cloud deletion from the receipt. [FX-0030-W5-REMOVAL-RECONCILED-CLASS-RECEIPT](../planning/feature-wave-5.md#acceptance-design-for-n-1454-to-n-1477). |
| N-1465 Self-emptying temp locker | Put eligible temporary parts under the registered scratch root and track their owning instance and lease. Clean up only after the exact owner generation, handles and leases have stopped under the same identity and unlink fence as N-1460. On interrupted shutdown, reconcile identity and any pending receipt before sweeping. Unknown files, pinned recovery and files outside scratch stay untouched. [FX-0030-W5-CLOSED-OWNED-SCRATCH-ONLY](../planning/feature-wave-5.md#acceptance-design-for-n-1454-to-n-1477). |
| N-1466 Disk-space safety cap | Use the existing owner-controlled cap and 0033/0078 storage reservations for the exact managed volume. Show actual bytes separately from promised work and protected floor. A lower cap stops new unsafe claims and waits for the existing safe path. Missing readings or floor proof hold new work. The owner may choose 50 GB as an example, subject to the same floor checks. No automatic deletion or relocation. [FX-0030-W5-DISK-CAP-EXACT-RESERVATIONS](../planning/feature-wave-5.md#acceptance-design-for-n-1454-to-n-1477). |
| N-1467 Locked core shield | Keep verified compiled program and runtime bytes separate from writable user data and scripts under the signed install manifest and admitted OS rights. Changes, signature mismatch or unknown permissions hold startup or repair in the current trust flow. Users and scripts cannot restore a core file by replacing it. No wider OS lock change. [FX-0030-W5-UNVERIFIED-CORE-START-REFUSAL](../planning/feature-wave-5.md#acceptance-design-for-n-1454-to-n-1477). |
| N-1468 Whole-app moving box | Use the existing encrypted archive export and import and exact owner recipient approval. Pin the scoped manifest, layout, allowed settings and scripts, and individually selected eligible data or artefacts. Fence existing work. Check format, collision, paths, compatibility, receipts and current erasure, then publish the new configuration generation atomically. New install trust comes from signed packages. Exclude raw secrets and active trust grants. Restore no executable authority, old access, cloud route or automatic launch. [FX-0030-W5-ENCRYPTED-MOVE-CURRENT-RECIPIENT](../planning/feature-wave-5.md#acceptance-design-for-n-1454-to-n-1477). |

QR-202 qualifies current platform and volume identity, the signed
package and runtime boundary, safe file effects, queue fences and
cleanup on retry. Recheck exact owner approval or the existing
scoped lifecycle cleanup policy, with current ledger authority. Preserve
0078/0019 stops and 0104 worker fences.

### Observability

- Metrics: `sutradhar_install_step{step,status}`,
  `sutradhar_update_duration_seconds{phase}`,
  `sutradhar_update_total{outcome}`,
  `sutradhar_backup_last_success_timestamp`,
  `sutradhar_backup_restore_verified_timestamp`,
  `sutradhar_ledger_mismatch_total`, `sutradhar_home_bytes{group}`.
- The supervisor writes structured logs and its own metrics file, read by
  the service.
- Alerts: backup older than 36 hours; test restore older than 35 days; an
  update paused more than 15 minutes; a part restarting again and again; a
  ledger mismatch.

### Edge cases

- After power loss during migration, reconcile the recorded checkpoint and
  committed state before continuing. Abort uncommitted work or resume a
  proved forward step. An unproved checkpoint holds. Recovery never
  reverses a committed migration or restores an older backup by itself.
- Disk full while fetching: stopped before anything is switched.
- The user deletes or edits files by hand: the next check shows a ledger
  mismatch; nothing is deleted automatically.
- An antivirus quarantines a program file on Windows: Check files names it
  and offers restore from the signed package.
- An external drive is missing at start-up: only that drive's sites pause,
  with a friendly reminder (spec 0038).
- A site restored from its own backup to an older state: Sutradhar sees
  missing posts and reads them again.
- The home folder is on a drive with a different letter after a restart:
  Relink finds it.

## Limits

- Memory follows spec 0078. Migrations reserve 256 MiB and run alone
  in the heavy slot. In first setup the native supervisor verifies
  that reservation under Installation step 6 before SQL. The
  supervisor's own footprint is a design target of about 16 MiB.
  Both need measurement before admission.
- The installer, without models, targets under 700 MiB of program files
  (proposed, to be measured).
- ganit's nightly batch runs only in the nightly window while backfill and
  rebuilds pause and heavy models unload. Whether new posts are still served
  within 30 seconds on both PC sizes needs measurement on 8 GiB and 16 GiB
  machines.
- x86-64 only in version 1.
- Speed targets, on the reference machine (4 cores, SATA solid-state disk),
  without downloads and the backlog run, each held by the 5% median rule:
  - preflight under 60 s;
  - installation after downloads under 10 minutes;
  - update pause, without migrations, under 5 minutes, with each
    migration's measured time on reference data in the notice;
  - compatible automatic rollback under 3 minutes;
  - disable: every part stopped within 90 s;
  - backup at least 50 MB/s; restore at least 30 MB/s;
  - uninstall, without the export, under 5 minutes;
  - Check files of a typical home folder under 60 s.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Rust tests unless stated; Angular screen tests use the SDS checks; add-on
tests are PHP tests with PHPStan at its strictest level. Status is
"designed", never "passed".

1. Preflight on fake hosts, each missing one need, fails with SUT-830 naming
   it, for Linux and Windows fixtures.
2. Give nominal budgets within the 0078 ceiling. After passing
   initial preflight, separately lose current capacity evidence or
   make available memory insufficient before PostgreSQL launch and
   before the migration reservation. Each hold gives SUT-830 and
   names the failed check. The first starts no PostgreSQL process.
   The second opens no migration session and adds no schema,
   migration history or success checkpoint changes. Both keep
   verified receipts and leave other applications running. A
   proved budget above the ceiling stays the separate SUT-837 case.
3. A budget over the ceiling is refused with SUT-837 by install and the
   gate (unit and property).
4. The supervisor's fence refuses another program, an unsigned or
   mismatched payload, a folder outside the home folder and a missing
   signature (and a property test).
- Supervisor fixtures distinguish a live, starting, stopped or stale
  verified instance, reused PID, changed generation, corrupt file and
  readiness outage. Race Quit and rollback against restart, and refuse
  an exhausted recovery budget. Reconcile a sealed rotation, prune and
  unexpected or replaced entry without an unlogged file action.

5. An install killed at each step in turn finishes from that step with no
   effect repeated (chaos). Exercise a verified early setup with no
   registered site and an approved pre-site migration. Only the exact
   permitted installation checkpoints and scoped migration history
   change. Try a missing or replaced register, foreign installation,
   substituted database root, stale step intent and attempted site
   access or connector dispatch. Refuse without new rows or effects.
   Interrupt before and after the ledger or migration commit. Resume
   the original verified step once when its receipt is known. Keep a
   different or unproved checkpoint paused without rebuilding its data.
   With a verified launch receipt and the original PostgreSQL
   instance still running within its limits, hold before migration
   on a memory shortage. Restore current proof and continue the
   same intent. Reuse the launch receipt with no second start.
   Lose the reply to the committed migration. Reconcile the exact
   0108 history and setup receipt once without another migration.
   An old memory sample grants no new reservation on either retry.
   In a separate resume, lose memory proof after the initial forced
   RLS history metadata commits but before its setup receipt. Hold
   further database effects. Preserve the existing committed shape
   and original intent. After current reservation passes, reconcile
   the exact committed metadata before any next migration. Publish
   the one matching receipt. Reaching a hold never undoes committed
   metadata or writes a numbered migration success row.
   In the first-start capacity case, choose the home after generic
   preflight. Replace its volume and remove its reservation proof in
   separate cases before creation. Each withholds the first file
   effect and SQL. On an admitted route, interrupt the first
   LedgerWriter creation before and after its native receipt.
   Reconcile only the exact original intent. Interrupt again before
   the first scoped SQL mirror. The original reservation and native
   receipts remain the proof and the mirror writes only the allowed
   installation rows.
   For old history with correct checksums and no verified installation
   ownership, show SUT-833 and the failed history check. Keep current
   data, offer the verified recovery guide and record no rollback or
   timing success. After enrolment, apply 0108 IT-BD-06 and 0031
   IP-23 to the matching and changed-plan update cases.
6. A new release gives only a notice and a staff alert; nothing is fetched
   or installed.
7. Check the owner, non-owner and changed-plan branches in spec 0031
   IP-23 before update starts. On matching owner approval, suggestions
   and backfill stop at safe points within 60 s and the review page
   says so.
8. With no verified backup the update stops with SUT-836; an administrator's
   override is refused; the owner's override with a reason continues and is
   audited.
9. With compatible current schema, state formats, journal, sites and trust,
   a failed self-test brings the previous version back within 3 minutes.
   Require one verified rollback result and no lost or repeated work
   (chaos). For measured release runs use the fixture timing contract below.
   IP-26 in spec 0031 covers the paused outcome.
   Use IP-26's release-trust branch to refuse the exact previous
   payload after a verified change in its current release eligibility.
10. With random part sizes, running limits never pass the budget at any
    moment of the start step (property).
11. The previous version's tests pass against the expanded tables and
    admit only compatible state, journal and site versions. With the
    same tables, offer an unreadable site register format and an
    incompatible site in separate runs. Both hold fallback before the
    old process starts. Preserve current data and receipts. No verified
    rollback event or SUT-835 on a held fallback.
   For a failed current database starter check during fallback, use
   0031 IP-26 and 0108's existing role and forced-RLS guard.
   Use IP-26's separate post-start branch for a database revision that
   fails compatibility before writes resume or return is verified.
12. A backup of the synthetic site restored into a fresh install gives the
    same records and checksums (weekly in CI).
13. "Keep settings, delete analytics" does exactly that; embeddings, indexes
    and model files are always deleted; approved links stay in every post.
14. The Remove screen submitted without changes deletes nothing optional.
15. An incompatible installed add-on gives the local SUT-838 mismatch and
    exact update guidance. An unsupported live outer envelope gets the
    fixed spec 0118 refusal before opening its body. For authorised
    legacy upgrade input, accept admitted 1.x note fields under
    spec 0021 and leave an unsupported major note in place with SUT-307.
    No live legacy send or acknowledgement.
16. Every screen and state passes the SDS accessibility checks and matches
    its screenshot on both systems. For IP-26, use an unreadable current
    data format and an unproved compatibility check in separate cases.
    Show the held step, safe reason and matching release recovery help.
    Resume and Rollback stay unavailable, current work is kept, and no
    restored message or SUT-835 appears. Compare a verified compatible
    return with missing timing evidence. Show the verified outcome and
    the unavailable measurement separately. With the window and engine
    down, the matching offline guide gives the same next step.
    Use 0004 MAN-OFFLINE with independently verified guides for the
    previous release and the recorded update. Only the matching
    held-step guide gives the applicable help. Give the prior guide
    alone or remove the recorded update in separate cases. Both hold
    with no launch, Resume, Rollback or change to current data.
    For MAN-OFFLINE's older-verifier case, keep the exact current
    guide independently verified. The guide remains read-only help
    when only the offered verifier lacks the required 0005 record
    reader. Keep the affected package use and recovery held.
    In the isolated test case under 0031 IP-30, offer Restore only
    after the held update is reconciled and the exact test route is
    eligible. Show the isolated target before owner confirmation.
    A passed test shows its own result. Keep the live paused result
    and unavailable Resume and Rollback, with no old live start.
    In the offline Windows desktop start case, keep exact package
    verification and the matching local guide. On the admitted
    profile, make its desktop WebView2 prerequisite genuinely absent
    with no eligible offline component available. Show SUT-4103
    with no desktop startup success. Open the verified guide through
    its independent offline help route. It identifies the required
    component and cannot start an installer, load a substitute or
    make a network request. Keep the original setup record. In the
    control supply the exact eligible desktop component and run the
    remaining startup checks. Keep the altered component's SUT-832
    case separate under Windows first trust.
17. After a fresh install and after a restore, the RLS check finds every
    table enabled and forced with policies and no `BYPASSRLS` role (SQL
    test against the test PostgreSQL).
18. The first setup screen shows the licence and the notices page matches
    the SBOM.
19. ganit answers warm in the self-test; with ganit stopped, suggestions
    still come from the degraded mode (chaos).
20. Ledger: every file the installer and a full run create appears in the
    ledger; a direct write outside `LedgerWriter` fails the build; an edited
    or removed entry breaks the hash chain and is reported (unit, property
    and gate fixtures).
21. Zero trace: after install, a full run and uninstall with everything
    set to Delete, the second pass finds no file, folder, task or start-up
    entry from the ledger on Linux and Windows fixtures. With Keep chosen,
    check the exact kept groups and any required protection evidence.

    For the Delete case, stage one managed synthetic support file
    and download a separate copy outside the managed plan. Verify the
    managed file is gone before reporting its class empty. After
    revoking the old grant and before removing the lifecycle part,
    try the old download link. It releases no bytes. Independently
    verify the delivered copy is unchanged. The receipt limits its
    result to the verified managed class and makes no claim about
    deleting the delivered copy.

    Join 0032 AU-13 to the N-1464 unreachable-volume case. Verify a
    local deletion, then lose the result for an approved managed
    support file on an eligible removable volume. In the interrupted
    case detach that volume. Restart and present an empty directory
    at its old mount path or drive letter. The second pass keeps the
    local deletion verified and the detached copy unverified. It
    cannot report an empty Delete class from the substitute view.
    The issued link stays fenced. Decline the optional final receipt
    and keep the lifecycle part, exact plan and journal through the
    hold. Record no `uninstall.finished` or final success. In the
    returning-volume control, verify the exact original storage and
    managed deletion result under current authority. Close that
    result once without another delete. Finish only when the
    remaining original checks pass. The optional final file is
    absent. Use synthetic storage and effect evidence only.

    In the whole-install Keep case, keep a verified encrypted backup,
    select Delete for the eligible ordinary audit group and leave the
    final receipt unselected. Reconcile an interruption before the
    lifecycle part is removed. Verify the kept backup bytes and the
    retirement and erasure history required by that backup, with the
    current signed minimum key epoch proof outside the archive. List
    the retained backup and protected evidence with their reasons.
    Report an empty reconciled Delete scope separately from what
    remains. Check the eligible Delete scope is gone, old live grants and
    site keys are revoked, and no optional final receipt remains.
    The kept backup does not retain active authority. In a separate
    case, make the required history proof unverified before the second
    pass. Withhold verified completion and preserve the kept backup.
    Use spec 0031 IP-33 and IP-34, spec 0032 AU-08 for the retained
    erasure obligation and the current key-epoch rules in spec 0118.

    For the late IP-34 restart, interrupt after one affected site's
    removal and retirement are recorded and before the lifecycle part
    leaves. Resume the same plan from the verified journal and existing
    ledger. Reconcile that site's completed step without repeating it.
    Check that no fresh ledger, install identity, live site row or
    tile appears. The removed site's old credentials still fail the
    site-scope check. Finish only the remaining authorised cleanup.
    Replay gives one logical completion. Separately remove
    the expected ledger before restart. Keep removal paused and withhold
    completion. Do not rebuild the ledger from database rows or the
    retained backup, and leave the backup and its protection history
    available under the original Keep choice.
22. Portable mode: the folder runs from a different path and drive, writes
    nothing outside it and adds no start-up entry; unplugging stops the
    engine safely.
    In the admitted IP-30 restore fixture, keep portable launch
    eligibility and normal-install proof separate.
23. Update trust: a payload with a wrong hash, a wrong signature or a stale
    manifest is refused (SUT-832); an update source that is down gives
    SUT-4102. When the notice check fails before an update, nothing
    pauses or changes. GitHub is untrusted transport.
    On an exact owner-approved update, compare an unavailable selected
    online source with an offline file whose valid signed metadata
    has expired. The first shows SUT-4102 and a route-specific retry.
    The second shows SUT-832, explains the expired proof and asks for a
    current verified bundle. Neither runs the payload, starts migration
    or switches versions. If update work has already paused, show that
    recorded state and keep the safe next action. As a separate control,
    select a complete offline bundle with current independently verified
    proof and exact owner approval while the online source remains
    unreachable. The offline trust check can pass and the other update
    checks still run.
    Changing route uses the current exact plan and its own approval.
24. Repair: a deleted program file is restored from the signed package; an
    unknown file is listed and left alone; Relink finds a drive whose path
    changed; the factory reset keeps data and scripts.
    Use the exact approved plan for the missing managed file. Keep the
    verified release, ledger and approved root fixed. After approval,
    put an unowned object at the planned destination after the final
    check but before publishing. Match the approved replacement bytes.
    Refuse with SUT-4100. Preserve the object and ledger. Record no
    completed repair or start from the held file. In a separate case
    keep the destination eligible and substitute the staged payload.
    Use the 0005 handoff refusal, SUT-832, with no installed effect.
    In the eligible control commit the repair and lose its reply.
    Reconcile the original verified result once. A file with matching
    bytes but no verified effect receipt cannot prove completion.
25. Move: an exported home folder imports on the other system with paths
    translated and every ledger entry valid. Use a compatible two-site
    archive made before one site retired and one subject was erased
    on the other site. The verified current source history carries both
    fences under the site-register rules. Before publishing the restored
    generation, prove the retired site stays unavailable and the erased
    subject's data is removed from the eligible site's restored records.
    Check the other eligible records are intact. Keep the archive bytes
    unchanged. In a separate run, offer the same valid archive with a
    missing link in the current history. Withhold the affected restore
    before publishing a tile or serving data. Reuse IT-SS-07, AU-08 and
    E2E-0118-MOVE for site visibility, erasure and lifecycle proof.
    Stage the two-site archive and pass isolated verification. After
    isolation and before the live switch claim, commit retirement of
    one site in one branch and erasure of a subject in another branch.
    In the recovery branch, replay a pending audit for the old stage
    while the authoritative history now includes the erasure. Replay
    the old pre-publication proof and staged state after restart. Keep
    both old stages held with the changed-history reason. Publish no
    staged tile, job or subject data. Keep the current live state and
    archive bytes. With current proof verify a newly prepared eligible
    stage, applying the new fences before publication. A changed scope
    requires a fresh confirmed plan. Separately fail the audit write
    and verify SUT-848 instead of the changed-history hold.
26. Rule-critical code (the budget check, the supervisor's fence, "never
    over the budget while starting", "delete only what was chosen" and the
    ledger chain) follows ADR-0020's strict core with 100% line, branch and
    MC/DC coverage and every killable mutant killed, with equivalent
    mutants proved and independently reviewed. All retain property tests and
    Kani proofs that no accepted settings add up to more than the budget and
    that the uninstall plan never deletes a group marked Keep.
27. Quint models of the update state machine, rollback and the uninstall
    plan prove that no step can leave a half-switched state and that Keep is
    never deleted (spec 0078 lists the other models).

Battle tests on every release candidate:

28. Update under the full load: the pause stays in its target, no work is
    lost or repeated, the budget is never passed.
29. Faults during an update, in turn: source unreachable, disk full,
    PostgreSQL killed mid-migration, the supervisor restarted, a bad
    signature. Each stops safely or rolls back with its code (chaos).
    In the disk cases, leave sufficient free space on another volume
    while the approved staging volume has insufficient space before
    pause. Separately exhaust the approved workspace after pause and
    fail the candidate self-test with unproved fallback workspace.
    Try a stale or replaced volume sample and an unavailable journal.
    Check the old service remains active in the pre-pause case. The
    later failures keep the verified checkpoint and previous payload
    and hold further writes or switch. There is no rollback receipt
    from an unproved fallback and no automatic deletion or move.
    A matching current reservation permits only the exact safe
    continuation. Exercise the backup override with an unsafe volume
    and confirm the disk check still holds.
    In a separate branch pass the candidate self-test after a committed
    forward migration. Keep the same volume, plan and lease. Before the
    final switch drop space below the admitted workspace and floor.
    Keep writes paused, the previous pointer and payload intact, and
    the exact migration receipt and recovery journal. No switch, write
    resume, completion or fallback launch follows. Restore current
    space proof and reconcile the original operation before continuing.
    With the admitted Linux fixture, pass self-test after one
    committed forward migration. Keep the path label, native volume,
    adequate space, verified package, current approval and lease.
    Before the final switch give the approved path a different backing
    directory on that same volume. Hold on the unverified root. Keep
    writes paused, the prior pointer and payload, and the exact
    migration and recovery records. No write or cleanup on the
    substitute, fallback launch, repeated migration, switch or success
    receipt follows. In the control prove the original root with its
    current reservation, journal, plan and lease, and continue only
    the unperformed part of the original operation. In a separate
    run, pass the final check against backing root A and retain that
    proof. At the switch effect hand-off, supply backing root B on the
    same volume under the same path. Hold even when B has matching
    staged bytes. Preserve the prior pointer, paused writes, payload
    and exact migration and recovery records. Refuse a switch or
    success receipt, fallback launch, repeat migration, write or
    cleanup on B. A control that keeps the exact checked root through
    use records one verified switch with the original migration
    receipt. Use synthetic native identity evidence with no live
    mounts or switch.
    Separately commit an eligible switch and lose its reply, then make
    space unsafe. Read back only the original verified committed result
    under current read authority. Hold any remaining effect until its
    current checks pass. Record one switch and no repeated migration.
30. Memory read every second through the update and the nightly ganit window
    never passes the budget, and no part is killed for memory.
31. Exercise both mixed-version states of the add-on and service. Ordinary
    site pages still work, incompatible integration is held, and the
    local notice gives SUT-838 where applicable. The authorised legacy
    conversion updates the service first. The add-on stays within 8 MB
    a request.
    In the service-first branch, complete the authorised app update
    and preserve its exact receipt. Before native use of the verified
    add-on ZIP, revoke the same actor's site update capability while
    the actor remains the installation owner. Apply 0031's current
    native action check. The site step holds before extraction,
    registration or activation. Preserve existing site files, ordinary
    pages and the committed app update. The incompatible protected
    site exchange stays held under 0118. The control uses current
    native approval for the exact ZIP on the existing host route.
    Admit protected exchange only after both ends prove the contract.
32. Under full load a backup is compressed, its ratio no more than 5% worse
    than the last release, and its test restore passes with RLS intact.
33. Compatible rollback under full load serves again within 3 minutes and
    service levels recover. Incompatible or unproved fallback keeps writes
    paused and preserves recovery evidence with no automatic restore.

### Rollback timing in the release fixture

Use the existing sealed local release harness, outside the product
process tree. Keep the test observer on the same admitted fixture
clock across product process replacement. The observer times the
fixture and has no installation, recovery or signing authority.
The fixture uses the existing synthetic input, fault controls and
local product entry point. It opens no new product listener.

Start at the first recorded terminal failure of the candidate
self-test on the exact update run. The observer records the live
monotone reading at that boundary before fallback work starts.
Finish at the first observed verified return to serving on the exact
previous payload. Require the durable rollback result, successful
checks on the normal local entry point and evidence that current
work was kept without repeats. A process launch, cached screen,
fault-injection timestamp or changed lease cannot supply either
boundary by itself.

Use two live `std::time::Instant` readings in the same test observer.
Keep the original start while product processes or update lease
holders change. Record the exact candidate, fixture, platform and
runner, run and checkpoint identities, observer and clock session,
event identities and elapsed duration. Use the observer's checked
duration in integer nanoseconds before rounding for display. Refuse
missing, reversed, substituted or incomparable readings. The release
runner records the measured evidence under its existing trust route.

Tests 9 and 33 pass their existing time bound at equality. The
reference speed target still requires a time strictly below its own
bound under the existing speed rules. Timing evidence changes
neither the product recovery outcome nor its permissions. Spec 0134
owns its separate request load gate.

Check just below, at and just above the bound in a synthetic timing
fixture. In the measured release fixture, restart a product process
and verify that timing keeps the original start. If a real
interruption ends the observer or breaks clock continuity, report
timing unavailable for that run. Keep any separately verified
functional recovery evidence. The incomplete timing record cannot
pass the required timing case. A reset clock, wall time or a newly
started observer cannot fill the gap. Time a subject host restart
only under a separately qualified fixture that proves the original
observer clock remained continuous. Keep measured host qualification
separate from a simulated interruption.

## Changes from the source

- The app and engine install from a signed `.deb` (Ubuntu and Debian) or an
  unsigned Windows installer; there is no systemd, Docker or host helper.
  An engine supervisor, run as the user, replaces the host helper.
- One home folder, a file ledger, zero-trace removal, portable mode, repair
  and move are new.
- Updates come from GitHub Releases with one click, not from a vendor
  server; the previous rule that an upgrade is never automatic stays in
  force as "never installed without the user's click".
- The screens are Angular and SDS on Tauri, not Dioxus.
- Memory follows spec 0078, not the fixed ceilings of AGENTS.md section 5.6.
- The AGPL source offer is replaced by the licence and notices page.
- The read-only forum database check stays dropped: the service reaches
  each site only through the add-on over HTTPS.
- The add-ons are GPL-2.0-or-later.

Source: the first version of this specification and its sources in the
archive named by the specifications index; the earlier text is kept in
the repository history.
