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
   3. Backups: an erasure list is kept and applied to any restore before
      the restored data is used. Backups are never rewritten; they
      expire by their retention.
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
3. Alert thresholds start from fixed defaults and are later tuned from
   history by ganit (job 246), shadow-tested before use. Every
   activation and rollback of a ganit output is an audit event.
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
   (identifiers and numbers only).
2. Scrub: secrets never serialise (secrecy types); addresses, emails,
   member names and the forum's domain and paths become placeholders;
   post text is never collected.
3. Preview: the owner sees every line that will be in the file and can
   remove any item.
4. Export: one `bundle.json.zst` file with its blake3 hash, downloaded
   by the owner. Sutradhar never sends it anywhere itself.

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

The range SUT-840 to SUT-849 is checked as free in the central list.

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

Rust coverage is 100% of lines, branches and MC/DC, with every mutant
killed. Rule-critical code (the event shape, the hash chain, the
erasure job's "each item once" and the scrubber) also has property
tests and Kani proofs where tractable, such as "an event built from any
input holds no field of a personal kind" and "the chain check fails for
any single changed event". Screens have Dioxus component tests.

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
- AU-09, grouped alerts: the same fault 1,000 times in a minute shows
  one alert with a count of 1,000 in the app and as one XenForo staff
  alert. Drishti unit test and Dioxus component test.
- AU-10, every error explained: a gate test reads the registry; each
  code has a plain message, a fix and a manual page. An error shown to a
  person without a code or manual page fails the gate.
- AU-11, doctor names the fault: with each dependency broken in turn
  (database, forum connection, disk, model file), that check fails with
  its fix and the others pass. Unit and chaos test.
- AU-12, scrubbed bundle: planted secrets, emails, addresses, member
  names, the forum's domain and post text never appear in a bundle.
  Property test.
- AU-13, preview is the file: after the owner removes an item and
  exports, the file holds exactly the remaining preview.
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

Test-first order: event shape and chain (AU-01, AU-03), grants and RLS
(AU-02, AU-19) and their proofs (AU-21); subjects and details (AU-04);
the target registry and gate test (AU-06), with each module's target
arriving with its module; the erasure job (AU-05, AU-07), restore
(AU-08) and ganit's target (AU-20); the code registry (AU-10) and
alerts (AU-09); doctor checks (AU-11); scrubber and bundle (AU-12,
AU-13); exports (AU-14, AU-15) and audit failure (AU-16); screens, load
and chaos (AU-17, AU-18).

Facts to check while building: how a XenForo 2.3 member deletion or
anonymisation shows to the add-on, on Dell's test forum; the XenForo
staff-alert route open to the add-on and its limits; which spreadsheet
programs still run formulas from CSV, for the export guard's test list.

## Changes from the source

- The live screens become the Dioxus app built from SDS, tested with
  Dioxus component tests.
- Numeric work and its snapshots move to ganit, the Rust numerical
  engine.
- Member deletions reach Sutradhar through the add-on over the signed
  HTTPS contract, not a read-only database link to the site.
- Speed targets use the 5% median rule, replacing the 5% ratchet.
- Memory follows the limits in AGENTS.md section 5.6, replacing
  "all containers".
- Coverage adds MC/DC and 100% killed mutants (AGENTS.md 6.4).
- Mutation runs use `cargo mutants`.
- "Dashboard" becomes the desktop app; bundles stay on the owner's PC.

Source: `docs/specifications/operations/audit.md` and
`docs/specifications/operations/audit-2.md` in the archive
`archive-2026-11-08-old-repo`.
