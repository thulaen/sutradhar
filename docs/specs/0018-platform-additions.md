# 0018: Platform additions

This part gathers the owner's small platform answers. Each one changes
the part that owns it; none is a new system. It covers extensions,
saved screen layouts, records kept as versions, the "removed member"
label, the feature owner map, filing found faults, held app faults,
encrypted backups, upgrade from a package file and the language order.
It is roadmap Phase 3 item 1 and Phase 5 item 2.

Example: an admin saves a layout with the review list sorted by date.
A colleague sees their own layout, not hers. The numbers on both
screens match. Removing a member leaves "removed member" in the
history, not a name.

## What it does

Every idea here goes through GanitEvolve research before it is built
(see "Research questions" below).

### A. Extensions: eleven rules

Extensions run in sandboxes built from Linux's own controls (spec
0037), and reach only the extension broker. Of the
eleven extension rules, nine apply now; the two on extensions depending
on each other stay on file, since extensions must never reach each
other. Already in the extensions part: a developer kit with the
`namaste` sample and the broker's API description; a manifest with id,
publisher, version, licence and image digest; one digest per extension;
fixed grants checked by the broker; rate limits and the load governor;
add, review, start, disable and remove with export; list, detail,
metrics and audited approval. The changes:

1. **Contract range.** The manifest gains `contract`: the broker
   versions it works with (today `/ext/v1/`). Add refuses a manifest
   whose range excludes the running broker, with error SUT-810 naming
   the problem.
2. **Upgrade hold.** The upgrade check lists enabled extensions whose
   range excludes the new broker contract. The upgrade is held, not
   forced, and the screen names each one. The ways out: install a
   compatible version, or the owner disables it by hand. Nothing is
   disabled for the owner. `extensions.contract_hold` is always on.
3. **Approval per site.** An approval names one site. The same
   extension on two sites is two approvals, two sandboxes, two keys and
   two own stores, and its allowance counts once per site against the
   pool. One site's approval never covers another. Installing a package
   grants nothing. Approval is on or off per site, never a percentage.
4. **Withdrawal.** Withdrawing a grant makes the broker refuse the next
   call (SUT-814). Extension keys and tokens never appear in logs, the
   manifest view or the support bundle (secret types and the bundle
   scrubber).
5. **Interrupted steps.** Add, start, update and remove each run as one
   durable job with checkpoints. A kill leaves one clear state: nothing
   half-made (sandbox, own store, key), and a repeated request does
   nothing twice.
6. **Own store before update.** Before an update that may change the
   extension's own store, the broker takes an export and keeps it until
   the new version is healthy. A failed update restores it. Removal
   keeps data unless deletion is chosen.
7. **Reasons.** Detail shows grants per site and who approved them. If
   clean-up cannot finish, the screen says why, with a new error code
   (SUT-810 to SUT-819 are all used; a number is still to be given).
8. **Check command.** `sutra extension check FILE` runs Add's own
   manifest checks offline, from the same code, with no copy of the
   rules. Broker contract tests ship with `namaste`. Its sample reads
   link records through `read.link_graph`.
9. **Not included.** A publisher list (approval is per package);
   extension-to-extension dependencies and cycle checks; screens added
   by extensions (extensions only propose, in the normal review queues).
   An approved package that is not reviewed may run, since the isolation
   tests are the proof.

### B. Saved layouts, and "the app may preview, never decide"

The desktop app holds no truth and writes only through the management
API. Plan and apply already exist (a preview with an id; SUT-803 if the
thing changed), with one list of actions for every way in. The changes:

1. **Previews come from the service.** Every preview (what-if, plan
   preview, link or text preview, estimate) is made by the service, so
   the logic exists once, and carries `preview: true`. The screen shows
   "Preview, not a decision" in words and an icon, never colour alone,
   and any export of it says so. An unlabelled preview is a defect.
2. **The app may lay out, never decide.** It may sort, group and hide
   columns of rows already returned. It never works out readiness, a
   permission, an order of suggestions, or whether an action is
   allowed. When the service's typed answer is missing, the screen shows
   the loud error state (as SUT-800), not a stand-in worked out from raw
   numbers. Hiding a control is the service's answer, not a rule the
   screen evaluates.
3. **Deep links carry places, not verdicts.** A link holds page, entity,
   filter and time. A forged score, verdict or permission in it is
   ignored; the service answers and checks the permission again.
4. **Context stays in one site.** Moving between screens carries the
   subject (site, thing, time window), read again from the service. It
   never crosses sites.
5. **Saved layouts, per admin and per site.** A layout holds column
   order, panel arrangement, saved filters, pinned views and the landing
   screen. The service stores it in `admin_layouts`; the app holds
   nothing. It is working state (see C): updated in place, with one step
   back that shows the old value. It is private to that admin, never
   shared by site and never copied across sites.
6. **A layout never changes a result.** Totals, counts, exports, plans
   and audit read no layout. A saved filter shows "Showing N of M". A
   bulk action's plan states the exact count it will touch, worked out
   by the service, so a hidden row is never acted on uncounted.
7. **Limits and clean-up.** `layouts.max_per_admin` and
   `layouts.max_bytes` have safe bottoms and tops. Over a limit is a
   plain refusal, never a cut. A layout is deleted when the admin's
   account is removed or loses staff rights, and is an erasure target.
   "Reset to default" is one click.
8. New capability: "Save my screen layout".

### C. Stored records: new versions only

Already there: append-only audit tables with no update or delete grant;
the change journal with old and new values, undo for 180 days and trash
for 90; settings history; retention sweeps by the scheduler; low-disk
notices. The changes to the database rules:

1. **Two kinds of table.** A *record* says what was decided, said or
   set, and when: setting values, suggestion decisions, approvals,
   applied thresholds, lab results. *Working state* is job rows, leases,
   queues, counters, caches, sessions and layouts. The schema registry
   declares each table one or the other. Working state is updated in
   place.
2. **Records get new versions.** A change inserts a version with
   `record_id`, `version`, `supersedes`, `recorded_at`, `changed_by`,
   `reason` and the value, plus `valid_from` where a value starts
   later. No runtime role may update or delete a record table. A view
   gives the current value: the latest version. Old versions stay
   readable for history and undo.
3. **Limits.** `records.max_versions` per record and
   `records.max_bytes` per version, with safe bottoms and tops. A
   version over its byte limit is a typed error, never a cut. At the
   version limit the oldest superseded versions go to clean-up; the
   current one never does.
4. **Clean-up.** The retention job removes only superseded versions
   older than `records.keep_days`, which is never below `undo.keep_days`
   (180). It never removes one that a waiting plan or a held or
   dead-letter job still points to. Clean-up runs in the maintenance
   lane, so it is shed first. After three failures in a row it raises
   the one "clean-up is failing" notice. Version growth counts in the
   low-disk notice. Evidence and kept foundation copies are outside
   this job.
5. **Erasure is the one exception.** The erasure job may blank personal
   fields inside record versions. The version stays; its person goes.
6. **Checks.** A schema test fails a record table with an update or
   delete grant, and a table declared as neither kind.
7. **Not included.** Keeping everything for ever; more than two
   timestamps per fact; chunk sharing, snapshots, branching and extra
   cache tiers.

### D. "Removed member" on erasure

Erasure already deletes the name mapping and details; events stay,
pointing to nothing; restore applies the erasure list; quotes in other
people's posts stand. The changes:

1. One label, "removed member", is used in every screen, export,
   journal, trash list, approval list and version history.
2. One lookup in lekha (the audit module) returns the name while the
   mapping exists and the label after. Screens never join names
   themselves.
3. Every "who" column in every record or working table holds an opaque
   subject reference, never a name. The audit gate walk fails a name
   there.
4. The label is the same for every removed member and shows no initial,
   count or joining date.

To research: whether a kept hidden reference is still personal data
where law applies (not legal advice).

### E. Feature owner map, checked by the gate

1. `features.toml` has one row per feature (F001 to F025, and each
   specification without a feature number): id, module, owner role and
   contract (a specification and, where one exists, its API tag or
   trait). About 35 rows.
2. The gate fails a feature with no row or two rows, a missing contract
   path, a module folder without an owners file, and a capability whose
   module no feature owns.
3. A commit that changes a row's module or owner is refused without the
   owner's signed record and a re-run of that feature's acceptance
   tests on the exact tree.
4. A missing map blocks; it never passes.
5. A test deletes a row and expects the gate to fail.

### F. A found fault is filed before a check passes

1. A pass needs every finding of the run (check output, adapter report,
   review comment, "Defects:" entry) to have an autoissues record with
   its fingerprint. One without blocks the run as "unfiled finding",
   with no debt, and is named. "Filed later" is never a pass.
2. Filing runs first. If it cannot, the result is "infrastructure
   unavailable" or blocked, never a pass.
3. A found fault is fixed in the same change (`Fixes:` trailer) or
   filed, never only mentioned.
4. A candidate change cannot weaken a check, ban, threshold, priority or
   suppression to pass. Policy comes from the base commit.

### G. App faults, scrubbed and held

1. **A fault record.** A coded fault (an SUT code at fault level, a job
   gone to the dead-letter list, a breaker opening, a caught panic)
   writes one record: code, component, cause key, count, first and last
   time, version digests. It passes the bundle scrubber. It holds no
   member name, address, domain, path or post text.
2. **A bounded holding file.** Records go to one holding file in the
   service's data folder, one entry per fingerprint with a count, so a
   storm raises a count and adds no lines. It has a size bound
   (`faults.hold_max_bytes`), an entry bound (`faults.max_entries`) and
   an age bound (`faults.hold_keep_days`). At a bound the oldest entry's
   detail goes, its code and count stay, and one notice is raised.
   Writing uses a small bounded queue that never blocks or slows the
   service.
3. **Held means held.** Nothing leaves by itself. The owner previews
   every line and exports the file through the support bundle route.
   Taking it to autoissues is by the owner's hand. For Dell's own test
   forum only, the gate account reads the file inside the test network
   through a new autoissues adapter: the fingerprint is code, component
   and cause key; a cut-off file makes the run incomplete. Faults are
   never copied to the Forgejo task list by themselves.
4. **Notices.** A fault notice reaches the owner during quiet hours.
   The file's growth feeds the low-disk notice, with no daily repeats.
5. **Erasure.** The file holds no personal data by design. It registers
   an erasure target that answers "nothing held".

### H. Backups encrypted before they leave the computer

1. Every backup is encrypted by the service before it is written
   anywhere off the computer: bucket, NAS or download. The local copy is
   the same encrypted file, so there is one format.
2. The owner holds the key. A public-key kind is preferred (to
   research), so the service keeps only the half that encrypts and
   cannot decrypt. The other half is shown once for the owner to save.
   It is never kept on the computer and never enters a support bundle.
3. No key, no off-computer backup, and the screen says why. There is no
   fallback to plain text. `backup.encrypt_off_server` is always on.
4. The encryption library must pass the dependency licence and
   admission checks. Which one is to research.
5. Restore needs the owner's key, goes to an isolated target, and
   applies the erasure list after decrypting and before use.
6. The Backups screen says in plain words: lose the key and the backups
   cannot be read. It names the recovery material to keep.
7. The upgrade backup check accepts a backup that is encrypted, whose
   checksum holds and whose last owner-run restore drill passed within
   a researched time. Restores are tested by these drills plus checksum
   checks.

### I. Upgrade from a package file, with no internet

1. A second way in: the owner loads one package file on the Upgrade
   screen or with a `sutra` command (name to be given). It holds the
   signed release manifest, every image by digest, notices and any
   listed model.
2. Everything after is the same as an online upgrade: approval, pause
   at once, backup check, check of every digest, migrations, smoke
   test, rollback. An unsigned package is refused, as online.
3. Refused, naming the missing or bad part: an incomplete package; a
   version not newer than the one installed (going back is rollback
   only); an extraction path, link, size or decompression ratio over
   its bound. `upgrade.package_max_bytes` bounds the file.
4. With no internet the release notice cannot arrive. The screen says
   so and offers "check for an upgrade by loading a package". Disk
   space is checked before unpacking.
5. The extension hold (A2) applies to a package upgrade too.
6. Self-updating by pulling from anywhere is not included.

### J. Languages after English

1. After English, in the owner's order: French, German, Spanish,
   Chinese, Japanese, Korean, Italian, Shona, Zulu, Ndebele, Swahili,
   Portuguese, Russian, Hindi, Hebrew, Arabic, Dutch, Thai, Telugu.
   Chinese means both Simplified and Traditional writing. Ndebele means
   both the Zimbabwean and South African varieties.
2. `text.languages.order` holds the list. Each language is "not yet
   supported" or "supported". English is supported first. A language
   becomes supported only when its checks pass, one at a time in this
   order. The order says what is researched next, not a date.
3. A post in a language not yet supported is "skipped: language not yet
   supported", a reason beside "language unknown". It is never taken as
   English, and no link goes in it.
4. The order covers checking forum posts only. There is no speech
   transcription.
5. Research must answer, per language: does the identifier tell it
   apart, with accuracy measured on a test set; how words and sentences
   split (Chinese, Japanese and Thai have no spaces; Arabic and Hebrew
   run right to left); do Shona and Ndebele have a usable model and
   licence; and what the extra tables cost in memory.

### Settings

Every setting ships on, with a researched value, a safe bottom and a
safe top.

| Setting | Default | Range |
| --- | --- | --- |
| `extensions.contract_hold` | on, no off switch | fixed |
| `layouts.max_per_admin`, `max_bytes` | to research | safe bottom, top |
| `records.max_versions`, `max_bytes` | to research | safe bottom, top |
| `records.keep_days` | to research | not below `undo.keep_days` |
| `faults.hold_max_bytes`, `max_entries` | to research | safe bottom, top |
| `faults.hold_keep_days` | to research | safe bottom, top |
| `backup.encrypt_off_server` | on, no off switch | fixed |
| `upgrade.package_max_bytes` | to research | safe bottom, top |
| `text.languages.order` | owner's list | supported flag per language |

### Research questions

How many versions per record and how many days keep history without
crowding the memory limits on a large forum; the safe size of a layout
and the number per admin; how big a fault file stays useful in a storm; which
encryption library and key handling suit a non-programmer and pass
admission; the largest sensible upgrade package; per-language accuracy
and memory; and the cost of one sandbox per site for an extension.

## Limits

- Everything fits the memory limits in AGENTS.md section 5.6, on a PC
  under 16 GB as well as a larger one. The old 5.6 GB extension figure
  is replaced by these; role budgets and the extension pool stay as set
  until measured.
- One sandbox per site per extension is the biggest memory risk.
- Layouts, record versions, the fault file and upgrade packages are
  each bounded by the settings above. Over a bound is a refusal or a
  shed of old detail, never a silent cut.
- Record clean-up never goes below 180 days of history.
- The fault queue is bounded and never blocks the service.
- Extension calls stay under the broker's rate limits and load
  governor.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Rust tests unless marked; SQL tests for database rules; screen tests
on the Dioxus app with an accessibility check.

- PA01: a manifest whose `contract` excludes the broker is refused,
  problem named (unit, extension manifest tests).
- PA02: an upgrade with an incompatible enabled extension is held;
  nothing is disabled; installing a compatible version lets it run.
- PA03: site A's approval does not let the extension read site B; two
  sites mean two sandboxes (unit and failure injection).
- PA04: after withdrawal the next call gets SUT-814; no key appears in
  any log or bundle (unit and property).
- PA05: add, start, update and remove are killed at random points;
  one state, no leftovers (failure injection).
- PA06: a failed update restores the saved store.
- PA07: `sutra extension check` and Add give the same verdict on
  generated manifests (property).
- PA08: every preview route returns `preview: true`; a screen test fails
  if preview data shows without the label.
- PA09: a build check lists app-side interop code and fails on anything
  not on the allowed list.
- PA10: a link with a forged verdict or permission shows the service's
  answer or SUT-801.
- PA11: two layouts over the same data give the same counts, exports
  and plan counts (property).
- PA12: a layout is invisible to other admins and sites, deleted with
  the account, and removed by the erasure job.
- PA13: an oversize or extra layout is refused in plain words.
- PA14: the database refuses an update or delete on a record table
  (SQL).
- PA15: a change makes a version and the view shows it (SQL).
- PA16: the byte and version limits hold.
- PA17: clean-up spares the current version, anything inside 180 days
  and anything a plan points to.
- PA18: erasure blanks a version and keeps the row.
- PA19: an erased member shows "removed member" in every listed place.
- PA20: a "who" column holding a name fails the gate.
- PA21: two removed members look the same on screen.
- PA22: a feature with no row fails the gate.
- PA23: a changed owner without a signed record fails.
- PA24: a record without a re-run of acceptance tests fails.
- PA25: a missing map blocks.
- PA26: deleting a row in a test fails the gate.
- PA27: an unfiled finding blocks.
- PA28: an outage of the issue store never gives a pass.
- PA29: a "Defects:" entry with no issue id fails.
- PA30: a candidate that relaxes a rule is judged by the base policy.
- PA31: seeded names, addresses, domains and post text never appear in
  the fault file (property).
- PA32: a flood at ten times the bound keeps the file inside it, raises
  counts and keeps service speed within its band (load).
- PA33: with outbound traffic blocked, nothing is sent and the owner's
  export works.
- PA34: the adapter gives the same fingerprint over two runs and marks
  a cut-off file incomplete.
- PA35: a fault during quiet hours still raises its notice.
- PA36: a scan of outbound bytes finds no readable marker text in any
  backup route.
- PA37: no decrypting key is on the computer or in a bundle.
- PA38: restore with the right key works; the wrong key fails cleanly.
- PA39: an erasure made after the backup is applied again after
  decrypting.
- PA40: with no key, the off-computer copy is refused and nothing is
  sent.
- PA41: a package with one wrong digest or a missing part is refused,
  naming it.
- PA42: an equal or older version is refused.
- PA43: a decompression bomb or a path-escaping archive is stopped at
  its bound (fuzzing).
- PA44: a full upgrade runs with the network off, through the same
  steps, and rollback works.
- PA45: an unsigned package is refused.
- PA46: the order setting decides which language is next.
- PA47: an unsupported language is skipped with the named reason, never
  as English.
- PA48: no link is placed in unsupported text.
- PA49: a language cannot be marked supported without its accuracy run
  on record.

## Changes from the source

- Decision logic (contract range, upgrade hold, record checks, owner
  map, unfiled finding, fault bound, language state) is Rust, proved
  with Kani, Verus or Creusot.
- Researched limits come from ganit and GanitEvolve, in Rust.
- Layouts, labels, previews and the extension, backup and upgrade
  screens are Dioxus app screens built from SDS components.
- "The browser may preview, never decide" becomes "the app may preview,
  never decide"; previews come from the service over the local socket.
- Extensions add no screens of any kind; the old desktop-shell rule is
  not brought.
- The service runs on the owner's PC, so "the server" in the backup and
  upgrade items means that computer.
- Gate checks are named by the gate's catalogue, not the old gate item
  names.
- The old 5.6 GB memory figure is replaced by the limits in AGENTS.md
  section 5.6.
- Extensions run in Linux sandboxes, not containers; the owner's PC
  never needs Docker.

Source: `docs/specifications/platform/platform-additions.md`,
`platform/platform-additions-2.md`, `platform/platform-additions-3.md`
and `platform/platform-additions-4.md` in the archive
`archive-2026-11-08-old-repo`.
