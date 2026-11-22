# 0032: Audit, erasure, diagnostics and safe exports

This is Sutradhar's permanent record of every action, the erasure of a
person's data and every copy made from it, the plain-English error and
alert system, the self-check, the scrubbed support report, and the one
safe path for every export. It is roadmap Phase 5 items 1 and 3
(install, repair and removal from the app; retention). It is feature
F022 in the feature list.

In plain words: Sutradhar keeps a permanent list of every action taken
on it (what, when, to which item) but no names or post text. Names,
addresses and before-and-after text sit beside it in a separate store
that can be deleted. When a member must be forgotten, that store is
deleted and every copy of their words is removed or rebuilt, while the
permanent list stays whole. A receipt says what was erased, place by
place.

Example: a member asks to be erased. XenForo deletes the account.
Sutradhar learns of it within a minute, removes the member's chunks from
the index and search, deletes cached text, retires suggestions on their
posts that wait for review, empties their items from the trash at once,
deletes their names from the audit details and records a receipt:
"Index: 412 chunks removed. Search: 97 posts removed. Backups: will be
cleaned on restore; oldest expires 14 January". The audit list still
shows "event 88,213: suggestion approved", with the name now "removed
member".

## What it does

### Terms

- Audit event: one permanent record of one action, with only
  identifiers and outcomes.
- Audit details: the erasable record beside an event: the name,
  address and before-and-after text.
- Subject: a person whose data Sutradhar holds: a forum member or an
  analytics visitor identifier.
- Erasure: deleting a subject's personal data and every derived copy.
- Derived copy: anything made from a subject's content, such as a
  vector or a search entry.
- Support bundle: one compressed file for a helper to diagnose a fault.
- Scrubbing: removing secrets and private content before export.

### Scope

In scope: the audit log and details; the erasure job, its target
registry and receipts; the central error-code registry; grouped alerts
in the app and as XenForo staff alerts; `sutra doctor` and its check
registry; the support bundle with preview; the shared export pipeline
every feature's exports use. Out of scope: the health, metrics, trace
and log screens; deleting posts, which the site does; what each module
stores (each registers its own erasure target here).

### Modules and traits

- lekha (audit): `AuditSink`, the log, the details store and
  `lekha::erasure`.
- drishti (health): the error-code registry, alerts, `drishti::doctor`,
  `drishti::bundle` and `drishti::scrub`.
- bhandar: `bhandar::export`, the export pipeline.

```rust
pub trait ErasureTarget: Send + Sync {
    fn name(&self) -> &'static str;
    async fn find(&self, s: &Subject) -> Result<Found>;
    async fn erase(&self, s: &Subject, ck: &mut Checkpoint)
        -> Result<Erased>;
}
pub trait DoctorCheck: Send + Sync {
    fn id(&self) -> &'static str;
    async fn run(&self, cx: &DoctorCx) -> CheckResult;
}
pub trait Scrubber: Send + Sync {
    fn scrub(&self, item: Item) -> Scrubbed;
}
```

1. Every module that holds subject data registers an `ErasureTarget`;
   every module with something to check registers a `DoctorCheck`.
   Fakes for each trait live in the shared test-support crate.
2. lekha does not know what modules store; each module erases its own
   data. One scrubber serves support bundles, exports and public
   aggregate figures. One error registry feeds the screens,
   `sutra error`, the MCP server and the manual.
3. Details are deleted, not encrypted and shredded.

### The audit log

1. Each event holds a sequence number, time, action ID, door (the entry
   point the action came through), actor reference, object identifiers,
   outcome code and a blake3 hash of the previous event plus this one,
   so a changed or missing event is found. No names, addresses or post
   text.
2. Actor references are opaque ("a-7f3c..."). The `subjects` table maps
   them to forum accounts or visitor identifiers. One lekha lookup
   returns the name while the mapping exists and the label "removed
   member" after.
3. Details hold, per event, the person's name at the time, the address
   where recorded, and the before-and-after values. Settings diffs hold
   no personal data and stay with the event.
4. The audit tables are append-only in PostgreSQL: forced RLS with
   policies that allow only insert and select, and no update, delete or
   truncate grant for any run-time role. A separate erasure role has
   only a delete policy on `audit_details` and `subjects`.
5. Details are kept until the person is erased or the installation is
   removed.
6. A restore records an event marking the gap, so the chain stays
   checkable.

### Erasure

1. Triggers: a member deleted or anonymised in XenForo, reported by the
   add-on; a visitor's request about their analytics identifier; an
   administrator's erasure action. Removal of content (a post hidden or
   deleted) uses the same targets for that post's derived copies,
   without touching audit details.
2. The job is a durable job with progress and checkpoints:
   1. Find: each registered target lists what it holds for the subject.
   2. Erase: each target removes or rebuilds its copies: vectors and
      chunks; search entries, built-in and connected; caches;
      suggestions waiting for review (recalculated at 30 days and
      retired with the reason in the audit log); the person's trash
      items (emptied at once, not after 90 days); snapshots; lab data
      on Dell; analytics archives; paid-embedding records; extension
      stores (asked through the broker); cached judge verdicts; ganit's
      inputs and outputs keyed by the subject (exported snapshots and
      per-person results such as jobs 154 and 155 on moderators,
      removed and recomputed at the next run); and audit details. ganit
      never receives post text.
   3. Backups: keep the current erasure history outside restored
      data under 0030. Apply it to any restore before the restored
      data is used. Backups expire by their retention and remain
      unchanged. A previous payload returning through 0031 must
      enforce the same current erasure history before reading or
      delivering affected data. Hold the fallback when its current
      history format or enforcement proof is unproved. Keep the
      erasure history and its existing target receipts.
   4. Receipt: per target, one of erased, nothing found, waiting (with
      the reason) or failed. No personal data in the receipt.
3. Approved links in posts stay; they are the forum's content.
4. Erasure cannot be undone. The request shows its scope first.
5. Edge cases:
   - A member erased while a post of theirs is in a suggestion awaiting
     review: the suggestion is recalculated at 30 days and retired with
     the reason "content no longer available" in the audit log.
   - A member who registers again is a new subject; nothing joins the
     two.
   - A visitor identifier that never existed: SUT-847, with a receipt
     saying nothing was found.
   - An outside party (a paid provider, an extension) that cannot
     confirm: the receipt says "waiting", with what was sent and its
     stated retention.
   - Quoted text: a quote of an erased member inside another member's
     post is that member's content. Sutradhar does not rewrite posts,
     and the receipt says so. The member-text option (a moderator
     approves each change) is the one case where Sutradhar edits post
     text.

### Errors, alerts and the doctor

1. Every error is a `SutCode` with a plain-English message, cause, fix
   and manual page, defined once with thiserror.
2. Alerts are grouped by code and cause: one alert with a count and
   first and last times, not one per occurrence. They show in the app
   and as XenForo staff alerts, sent through the add-on so XenForo
   writes them. Snooze and resolve are actions.
3. Alert thresholds start from fixed defaults. Ganit (job 246) may propose
   tuned candidates from history, but spec 0077 alone shadow-evaluates,
   promotes, activates and rolls them back. Every such transition is an
   audit event; a rejected or failed candidate leaves the champion unchanged.
4. A faulty method is isolated and reported, not the whole engine; its
   results are withheld and the screen says why.
5. `sutra doctor` and the Diagnostics screen run every registered check
   with progress: forum connection, database, queues, models, disk,
   memory against budgets, search, storage, extensions, the warm ganit
   server, every overdue ganit job, clock and versions. Each shows pass,
   warn or fail, with the fix.

### Support bundles

1. Collect: versions and digests, settings without secrets, error codes
   with counts, check results, recent warnings and errors, resource use,
   recent job outcomes, and the versions of ganit's active parameters
   (identifiers and numbers only). Include bounded recent log records
   from one admitted 0030 manifest generation. Read sealed entries only.
   Use the same verified file beneath the owner-only root. Verify path,
   identity, compressed length and sealed hash before using decoded
   output. Refuse linked, replaced, open or unlisted input and unsafe
   prune or rotation races.

   For setting values, use the current typed Settings projection in
   spec 0014 under the requester's site scope and read permission.
   Collect only fields already admitted for support bundles and
   visible in that projection. If the projection or permission
   cannot be verified, omit the setting values. Do not read backup
   files, isolated restore data, settings history, raw configuration
   or secret storage to populate the bundle.
2. Decode allowed typed fields within time, count, compressed and
   expanded bounds. Refuse the unchecked log set on failure, corruption
   or excess. Apply spec 0112 masking and the same export scrubber.
   Post text is never collected. No raw log is attached. Summarise
   omitted items without including their values.
3. Freeze the canonical scrubbed JSON for preview and item removal.
   Approval binds its content hash, owner, site scope and expiry.
   Recheck grant and erasure immediately before the common export
   route delivers it. For frozen setting fields, recheck the
   requester's read scope and any restore fence that affects those
   fields. A change requires a fresh preview. A rotation cannot add
   records to a frozen candidate. Give the final compressed
   file its own byte hash.
4. Export: one `bundle.json.zst` file with its blake3 hash, downloaded
   by the owner. Sutradhar never sends it anywhere itself.
   Register any managed prepared bundle file in the 0030 ledger under
   its exact export and scoped identity. Keep the approval bound to
   the frozen preview. On an erasure that affects the bundle, block
   further delivery of those bytes through the existing download
   action, including a link already issued. The link's expiry or
   invalidation supplies no deletion proof.

   Apply the erasure target to the exact affected managed file and
   its approved storage identity. When removal is interrupted or its
   reply is lost, reconcile that recorded effect before retry. Only
   verified removal of the selected managed copy closes its receipt.
   An unreachable or unverified copy stays waiting. Report a proved
   deletion failure through the existing failed outcome. A file
   already delivered to the owner's custody follows the recipient
   boundary below. This action has no authority over that copy.

### Safe exports

1. All exports (reports, suggestions, audit, the uninstall export) go
   through `bhandar::export`.
2. Only what the requester may see is exported.
3. CSV cells starting with `=`, `+`, `-`, `@`, a tab or a carriage
   return get a leading apostrophe. JSON has a schema.
4. Every export is audited. Download links expire after 24 hours.
   Exports that hold personal data are marked "contains personal data".
5. Erased subjects are left out of later exports. Earlier downloads
   cannot be recalled, and the screen says so.

### Data

All in PostgreSQL, every table with forced RLS:

- `audit_events` (append only), `audit_details`, `subjects`.
- `erasure_requests`: id, subject reference, trigger, state.
- `erasure_receipts`: request id, target, result, counts, time.
- `alerts`: code, cause key, count, first, last, state.
- `doctor_runs`; `exports`: id, kind, requester, expires, hash.

### Actions and events

- Actions: `lekha.audit.browse`, `lekha.audit.verify_chain`,
  `lekha.erasure.request`, `lekha.erasure.receipt`,
  `drishti.alert.snooze`, `drishti.alert.resolve`, `drishti.doctor.run`,
  `drishti.bundle.preview`, `drishti.bundle.export`,
  `bhandar.export.create`. Each is checked against the Cedar rules.
- Events: `erasure.started`, `erasure.finished`, `alert.raised`,
  `alert.grouped`, `doctor.finished`.
- Commands: `sutra audit browse|verify`, `sutra erase`, `sutra doctor`,
  `sutra error`, `sutra bundle`.

### Error codes

Spec 0032 owns SUT-840 to SUT-849 in the central catalogue.

| Code | Meaning |
| --- | --- |
| SUT-840 | Audit chain broken (a top-priority alert) |
| SUT-841 | Erasure target failed; retried |
| SUT-842 | Erasure waiting on an outside party |
| SUT-843 | Export refused: not permitted |
| SUT-844 | Bundle preview expired |
| SUT-845 | XenForo alert route down; app only |
| SUT-846 | Doctor check timed out |
| SUT-847 | Subject not found |
| SUT-848 | Audit write failed, so the action is refused |
| SUT-849 | Scrubber met an unknown field and left it out |

An action whose audit write fails does not happen (SUT-848).

### Screens

The Audit, Erasure, Alerts, Diagnostics, Support bundle and Exports
screens are SDS components and meet WCAG 2.2 AA:

- The audit list is a real table with column headers, sortable by
  keyboard, paged, with the row count in words.
- Diffs mark removed and added text with words ("Removed", "Added")
  and symbols, never colour alone.
- The erasure request repeats its scope and needs a typed confirmation.
- The bundle preview is a list, each item with a labelled remove button.
- New alerts are announced politely and never take focus.

### Observability

- Metrics: `sutradhar_audit_events_total{action,outcome}`,
  `sutradhar_audit_chain_verified_timestamp`,
  `sutradhar_erasure_requests_total{trigger,outcome}`,
  `sutradhar_erasure_duration_seconds`,
  `sutradhar_alerts_open{severity}`, `sutradhar_doctor_checks{result}`,
  `sutradhar_exports_total{kind}`.
- Last-success gauges for the nightly chain check and the erasure
  watcher.
- Alerts: a broken chain (top priority); an erasure waiting more than
  72 hours; the XenForo alert route down for an hour.
- A checkpoint-age gauge, a late-checkpoint alert and the "Prove this
  event" button are in the audit proof specification.

### Capabilities

- CAP-048: audit screen and `sutra audit`: "Browse every change with
  who made it, when, and a diff".
- CAP-049: erasure screen, receipt and `sutra erase`: "Erase a person's
  private data, including derived copies".
- CAP-059: alerts in the app and in XenForo, with snooze and resolve.
- CAP-060: Diagnostics screen and `sutra doctor`.
- CAP-009: error panel and `sutra error`.
- CAP-065: de-duplicated error log.
- CAP-069: support bundle preview and export, `sutra bundle`.
- Exports count under each feature's own report capability (for
  example "Export the suggestions report"); the pipeline is machinery.

### Security and privacy

- No personal data in events, metrics, spans or receipts.
- Bundles and exports never leave the PC unless a person downloads
  them. Nothing public holds the forum's name or data.

## Limits

Speed targets, on Dell under fixed conditions:

| Measure | Target |
| --- | --- |
| Writing an audit event, added to an action | p95 under 2 ms |
| Browsing or filtering 10 million events | p95 under 300 ms a page |
| Checking the whole audit chain | at least 1 million events a minute |
| Erasing a member with 10,000 posts, all local targets | under 15 minutes |
| Member deleted in XenForo to erasure start | under 2 minutes |
| A fault to its grouped alert in the app | under 30 s |
| `sutra doctor` | under 30 s |
| Support bundle ready to preview | under 60 s |

- Erasure yields to live work.
- Memory: everything here runs inside the service slice, within the
  memory limits in AGENTS.md section 5.6, inside spec 0024's row for
  other parts' standing budgets until measured. Estimates, measured on
  Dell:
  - Audit writer and alert grouping: 16 MiB, inside the server's share.
  - Erasure job: at most 64 MiB, in batches of 1,000 items, inside the
    worker's share.
  - Doctor and bundle builder: at most 32 MiB, streaming the bundle to
    disk.
  - Exports stream rows; none holds more than 16 MiB at once.
- Download links: 24 hours. Suggestions on erased posts: recalculated
  at 30 days.

## Footprint budget

This part adds nothing to public pages. XenForo staff alerts appear
only in the site's admin pages.

## How it is tested

Coverage and mutation obligations follow ADR-0020. Only components
explicitly admitted to its strict core require 100% line, branch and MC/DC
coverage and every killable mutant killed; equivalent mutants require
proof and independent review. Other code follows the owning risk matrix.
Rule-critical code (the event shape, the hash chain, the erasure job's
"each item once" and the scrubber) also has property tests and Kani proofs
where tractable, such as "an event built from any input holds no field of
a personal kind" and "the chain check fails for any single changed event".
Screens have Angular component tests.

- AU-01, no personal data in events: for every action with random
  inputs, the event holds no name, address, email or post text. Lekha
  property test.
- AU-02, append only: the service's database account cannot update or
  delete an audit event. SQL test on test PostgreSQL.
- AU-03, tampering found: an event changed or removed in the database
  makes the chain check fail with SUT-840 at that event. Unit and
  property test.
- AU-04, erasure keeps history: after erasing a member with audited
  actions, every event remains, the chain verifies and the name reads
  "removed member". CAP-049 acceptance test.
- AU-05, every derived copy: with a member's posts indexed, searched,
  cached, suggested and in lab snapshots, after erasure no target finds
  anything and the receipt lists each target. CAP-049 acceptance test
  on the Dell test forum.
- AU-06, every table covered: a gate test walks the schema; every table
  with a subject column has a registered erasure target, and no "who"
  column holds a name, only an opaque subject reference.
- AU-07, resumable erasure: killed at random points and resumed, the
  job ends with each item erased once. Chaos test.
- AU-08, restore applies erasures: a backup made before an erasure is
  restored with the erased data removed before the service uses it.
  In the joined 0030 test 25, advance current erasure history after
  an isolated restore passes and before live publication. The old
  stage stays held. Restore uses the newer verified fences before
  publishing an eligible stage. This hold preserves the lawful
  erasure and uses the restore reason. AU-16 separately checks a
  failed audit write.
  Use the IP-30 history-format branch for a newer authenticated
  checkpoint the selected restore reader cannot enforce. Keep its
  committed fence and existing waiting outside-target receipt.
  The held restore or a later isolated success gives no completion
  proof for that target.
  [QR-250](../registers/questions.md) defines the decision and proof
  for a durable authorised request whose protected restore fence
  is unconfirmed. Exercise that state separately from preview,
  uncertain commit and a committed fence with a waiting target.
- AU-09, grouped alerts: the same fault 1,000 times in a minute shows
  one alert with a count of 1,000 in the app and as one XenForo staff
  alert. Drishti unit test and Angular component test.
- AU-10, every error explained: a gate test reads the registry; each
  code has a plain message, a fix and a manual page. An error shown to a
  person without a code or manual page fails the gate. For the
  [0133][coach-errors] and [0136][matomo-errors] error tables, match
  each emitted code to its owner, the outcome in the existing
  synthetic acceptance check and a reachable next step in the
  same release's manual. A reserved but unused slot supplies
  no error outcome. Hold missing, conflicting or unsafe mappings.
  The message and help must preserve the owning route's scope,
  authority, retry and recovery fences. The registry check uses
  the existing fixtures and changes no provider access.
- AU-11, doctor names the fault: with each dependency broken in turn
  (database, forum connection, disk, model file), that check fails with
  its fix and the others pass. Unit and chaos test.
- AU-12L, log bundle: simulate rotation, a manifest mismatch, an
  replacement, linked or open entry, malformed fields and expansion
  above the cap. Refuse unsafe prune. Race expiry, revoke and erasure
  with delivery. Require a new preview and never include raw logs or
  records from a later rotation.
- AU-12, scrubbed bundle: planted secrets, emails, addresses, member
  names, the forum's domain and post text never appear in a bundle.
  Property test. Separately, stage an isolated restore and plant
  distinct synthetic non-secret values in the current Settings
  projection, the staging store and another site. Check the collector
  reads only the requester's admitted projection. Read no raw backup
  or staged value, even if the current Settings projection is
  unavailable. Repeat with the joined AU-08 restore held because its
  erasure history advanced after verification. The support collector
  may use the current authorised projection only. It cannot replace
  withheld settings with values from the staged restore. Repeat
  after verified activation. Only allowlisted and currently visible
  values are eligible.
- AU-13, preview is the file: after the owner removes an item and
  exports, the file holds exactly the remaining preview. Revoke
  settings read permission or invalidate the restore fence after
  preview and before delivery in separate cases. Hold the frozen
  bundle and require a fresh eligible preview. No file is delivered.
  In the existing export race, stage an approved bundle for a
  synthetic subject and lose the managed-file deletion reply after
  erasure. Exercise its already-issued download link. Block delivery
  of the old bytes and leave the affected file's erasure receipt
  unverified. Recover with exact removal proof and record that
  outcome once. In a separate run replace its managed file identity
  before replay. Leave the substitute untouched and withhold
  verified erasure. Join 0030 test 21 for a lost deletion result
  with the selected managed volume detached. An empty view of its
  old path cannot close the erasure receipt. The prior link stays
  fenced while current storage proof is missing. An earlier owner
  download is outside this proof.
- AU-14, safe CSV: cells starting with `=`, `+`, `-` or `@` open as text
  in a spreadsheet. Unit and property test.
- AU-15, exports respect permission: a moderator's audit export leaves
  out details beyond their permission (Cedar rule test as well).
- AU-16, a failed audit stops the action: with a failing audit store an
  action is refused with SUT-848 and nothing changes.
- AU-17, speed: with 10 million made-up events, browsing and the chain
  check meet the targets. A Rust load benchmark on Dell; a slowdown
  blocks under the 5% median rule.
- AU-18, screens: every screen and state passes automated WCAG 2.2 AA
  checks and screenshot comparison.
- AU-19, the database keeps the log: each run-time role trying to
  update, delete or truncate an audit event, even with no filter, is
  refused by policies and grants; the erasure role can delete only
  details and subject mappings. SQL test on test PostgreSQL.
- AU-20, ganit forgets too: a moderator in an exported snapshot and in
  job 154's output is removed from both, the next run recomputes
  without them, and the receipt lists ganit.
- AU-21, proofs and mutants: Kani proofs for the event shape, chain,
  erasure job and scrubber hold, and `cargo mutants` leaves no survivor.

[coach-errors]: 0133-dataforseo-live-coach-evidence.md#failure-codes
[matomo-errors]: 0136-matomo-timeline-connector.md#failure-codes

Test-first order: event shape and chain (AU-01, AU-03), grants and RLS
(AU-02, AU-19) and their proofs (AU-21); subjects and details (AU-04);
the target registry and gate test (AU-06), with each module's target
arriving with its module; the erasure job (AU-05, AU-07), restore
(AU-08) and ganit's target (AU-20); the code registry (AU-10) and
alerts (AU-09); doctor checks (AU-11); scrubber and bundle (AU-12,
AU-13); exports (AU-14, AU-15) and audit failure (AU-16); screens, load
and chaos (AU-17, AU-18).

Build-blocking evidence: the audit integration record must establish how
XenForo 2.3 member deletion or anonymisation reaches the add-on on Dell's test
forum, the available staff-alert route and its limits, and which spreadsheet
programs execute formulas from CSV. Dependent erasure, alert and export-guard
implementations remain blocked until official-version evidence and isolated
fixtures are admitted.

## Changes from the source

- The live screens become the desktop app built from SDS, tested with
  Angular component tests.
- Numeric work and its snapshots move to ganit, the Rust numerical
  engine.
- Member deletions reach Sutradhar through the add-on over the signed
  HTTPS contract, not a read-only database link to the site.
- Speed targets use the 5% median rule, replacing the 5% ratchet.
- Memory follows the limits in AGENTS.md section 5.6, replacing
  "all containers".
- Coverage and mutation follow ADR-0020; its MC/DC and zero-surviving-
  killable-mutant requirements apply only to admitted strict-core members.
- Mutation runs use `cargo mutants`.
- "Dashboard" becomes the desktop app; bundles stay on the owner's PC.

Source: `docs/specifications/operations/audit.md` and
`docs/specifications/operations/audit-2.md` in the archive
`archive-2026-11-08-old-repo`.
