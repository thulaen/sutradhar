# 0028: Staff suggestions, insertion and undo

This part lets staff review the links Sutradhar suggests, places a link
only when a person approves it, and takes out only Sutradhar's own link on
undo. It is roadmap Phase 4 item 3 (accept and undo on XenForo); the
WordPress plugin follows the same rules under Phase 4 item 4. The Rust
module is sujhav ("suggestion").

Sutradhar never puts a link into a post on its own. A moderator reads each
suggestion and presses Approve. Only then are words already in the post
wrapped in a link, through the site's own editor, as a quiet edit that does
not bump the thread or alert anyone. Undo takes out only that link; any
later change by anyone else stays.

## What it does

### Words used here

- Suggestion: one source post, one anchor phrase and up to five ranked
  destinations.
- Content version: the SHA-256 digest of a post's stored message, kept
  with every suggestion.
- Connection: a link from one source thread to one destination thread,
  judged on the canonical parent threads.
- Suppressed connection: one a person removed. It is never suggested again
  until staff reset it.
- Thread allowance: the most content links one thread (with its replies)
  or resource (with its updates) may hold; 5 by default, configurable.
  Quotes, navigation and mentions do not count.
- Silent edit: an edit by the site's system user with formatting, the
  "last edited" note and alerts turned off.

### States of a suggestion

The states set by matching (analysing, complete, replaced, shadow) stay.
This part adds the review states.

| State | Meaning | Next |
| --- | --- | --- |
| analysing | methods still running | complete, retired |
| complete | can be approved | approving, rejected, retired, stale, parked |
| approving | apply job queued or running | applied, apply_failed, stale |
| applied | the link is in the post | undoing, gone |
| apply_failed | refused at apply, with its code | complete, retired |
| rejected | refused by a person; in the trash bin | complete (restored) |
| parked | 3 recalculations, no decision | approving, rejected, retired |
| retired | stale or below threshold; in the trash bin | complete (restored) |
| stale | the post changed; analysed again | none (a new one follows) |
| undoing | undo job queued or running | removed, undo_conflict, gone |
| removed | our link taken out; connection suppressed | none |
| undo_conflict | needs a person; nothing changed | removed, applied |
| gone | our link vanished through another edit | none |
| shadow | stored, never shown | none |
| replaced | superseded by a re-score | none |

- One state machine table drives every move. Services only call
  `transition(from, to, guard)`.
- Every move is one database transaction guarded by the suggestion's
  `version` number, so two moves can never both happen.

### The review page

1. The list shows suggestions waiting for a decision, in review-queue
   order (most accepted links per moderator minute, from ganit). It can be
   filtered by forum section, confidence, age, "person" mark and playbook,
   and is paged so tens of thousands stay quick. Queue figures and review
   workload sit above it.
2. Views: the main queue; the parked list, with the same filters; and the
   trash bin, with the days left before the 90-day purge and a Restore
   button.
3. Each card shows:
   - the source sentence with the anchor highlighted, fetched with the
     viewer's permission and the content's guest visibility checked again,
     and censored words masked;
   - up to five destinations, each with title, confidence and a "why"
     link;
   - an exact preview of the post as it will read, from shodhan's output;
   - a target preview with its mini neighbourhood graph;
   - a "worth a careful look" note when ganit marks the case contested;
   - a "few links in" tag, and "asked for while drafting" or "held for
     test" where those apply (spec 0029).
4. While analysing, the card says "analysis continuing" and what it waits
   for. Approve stays disabled until analysis has ended and every
   hard rule passes.
5. Buttons: Approve (per destination), Reject (with optional reasons) and,
   for applied links, Undo. Each has a keyboard shortcut.
6. Presence shows "Asha is reviewing this" on a card another moderator has
   open. It is advice only; the version check decides who wins.
7. New suggestions and other moderators' decisions appear without a
   reload. If live updates fail, the page polls.
8. Accessibility is WCAG 2.2 AA, checked on every state.

### Approval

The action `setu.suggestion.approve` takes the suggestion id, the chosen
destination, the `version` the moderator saw and an idempotency key. In
one transaction the service:

1. Checks the caller holds the site permission "Sutradhar: review
   suggestions" and is a person. An AI agent may recommend but never
   approve.
2. Locks the suggestion row, then the source thread's allowance row
   (`SELECT ... FOR UPDATE`), always in that order, so two approvals in one
   thread queue rather than deadlock.
3. Checks the state is `complete`, the version matches (otherwise it shows
   "Already approved by Asha at 10:02", or the current state), every
   applicable method has finished and every hard rule passed.
4. Re-checks against the site's current data:
   - the stored message still has the suggestion's content version; if
     not, the suggestion becomes `stale` and is analysed again (it is not
     re-located);
   - source and destination are guest-visible now, and not deleted,
     soft-deleted or awaiting moderation;
   - the destination still resolves to the same canonical thread,
     following merges and redirects;
   - no link from the source thread to that destination thread exists or
     is being applied, counting bare addresses and links inside quotes,
     and the two are not the same thread;
   - the connection is not suppressed, and not rejected without a
     meaningful change since;
   - the thread allowance has room: existing content links, plus links
     being applied, plus this one, are at most the limit;
   - every link-equity rule still holds, through the same `equity::check`
     the ranker uses, with day and week caps, anchor variety, section
     spread and reciprocity counted again now; owner reject rules
     (spec 0029) are checked here too.
5. Records the decision, moves the suggestion to `approving`, and writes
   the apply job to the outbox in the same transaction, so a crash can
   never lose an approval or apply one twice.

A failed re-check refuses the approval with its plain-English reason and
code and moves the suggestion to the matching state. Nothing is written to
the site. An equity refusal leaves the suggestion waiting, so it can be
approved once the cap frees up, or be replaced or retired at its next
recalculation.

The relevance gate is not run again at approval, because approval never
depends on the current models. A suggestion made under older models is
judged on content, visibility and the rules alone.

### Batch approval

- A moderator may tick several suggestions, each with its chosen
  destination visible, and approve them together.
- Each goes through the same action and checks, one at a time, in the
  order ticked, so caps count the earlier ones. The page reports each
  outcome separately.
- At most 50 per batch. That is a top that research may lower (spec 0029);
  raising it needs the owner.

### Placing the link: the apply job

A durable job takes the outbox entry:

1. Read the post's current stored message and check the content version
   again.
2. Call shodhan's `insert_link` with the recorded anchor offsets, the
   message hash and the destination's short address. A refusal ends the
   attempt with its code.
3. Send the add-on a signed request: post id, old hash, new message, new
   hash, idempotency key.
4. The add-on, in one database transaction, compares the stored message's
   hash with the old hash and stops if they differ; saves through the
   site's post editor as a silent edit by the system user (formatting off,
   no "last edited" note, no thread bump, no alerts, automated mode); then
   compares the saved message's hash with the new hash and rolls back on
   any difference (SUT-416).
5. Edit history is kept, so the site's history shows the version before
   our link.
6. On success the service stores the insertion record, moves the
   suggestion to `applied` and sends `suggestion.applied`.

- A link placed here never carries a `nofollow`, `sponsored` or `ugc`
  mark, and no setting changes that.
- A post the site's save would rewrite (an unclosed or tangled tag) is
  never edited. The card says "Skipped: the site would rewrite this post"
  (SUT-424).
- If the destination was merged after approval but before apply, the job
  links the canonical thread's short address only if it is still a
  different thread.
- An approval that arrives before a re-score's replacement wins.

### WordPress

- The WordPress plugin has its own apply and undo contract. It uses
  WordPress's own capabilities, an HTML and block checker and an exact
  change journal, with no whole-revision restore and no shortcut through
  the XenForo text checks.
- On both platforms, current permissions, visibility, content version and
  any lasting pause are checked again just before apply and undo, and the
  save uses the platform's own save path.

### Recovery

- Site unreachable or slow: the job retries with backoff and is never
  dead-lettered for an outage. The card says "waiting for the site".
- Crash after the add-on saved but before the service recorded it: on
  restart the job reads the post. If its hash equals the new hash, it
  records `applied`. If it equals the old hash, it tries again. Otherwise
  it records `apply_failed` with "the post changed during the edit" and
  sends the case to review.
- The add-on keeps each idempotency key with its result for 7 days, so a
  repeated request returns the first answer and never edits twice.
- Disabling stops waiting apply jobs at a safe point, visibly, and they
  resume when enabled. No half-applied edit is possible.

### Recalculation, parking and retirement

No suggestion is ever just deleted for age.

- A suggestion with no decision (complete or parked) is recalculated when
  the interval passes: search, reranking and ranking run again with the
  current models, settings, ganit outputs and the latest text of the post
  and destination, at backfill priority.
- The interval is `suggestions.recalculate_after`: 30 days by default,
  allowed 7 to 90 days. Each suggestion counts its recalculations.
- Outcomes, each recorded with its reason in the audit log:
  - still the best link: it stays with its new score and keeps its
    original date for queue order;
  - a better destination now exists: it is replaced in one step, and the
    successor keeps the original date;
  - the destination was deleted, made private or merged, the score fell
    below the display threshold, or the acceptance probability or reranker
    relevance fell below the site's relevance gate (SUT-550): it is
    retired into the trash bin.
- After three recalculations with no decision, the suggestion moves to the
  parked list. Moderators can still act on it, and it is still
  recalculated each interval.
- An edit to the source post makes the suggestion `stale` and the post is
  analysed again at once. When the source or destination stops being
  guest-visible, or is deleted, its suggestions are hidden and retired at
  once, into the trash bin.
- Change events from sampark act at once. A daily sweep, paged by id and
  never by offset, finds suggestions due for recalculation. The approval
  re-check has the final word.
- How acceptance fades with age (ganit) orders the queue and shows
  freshness only. It never retires a suggestion.
- When a thread's allowance fills, its other suggestions show "allowance
  used" and wait. They can be approved again if a link is removed.

### The trash bin

- It holds rejected and retired suggestions, links a moderator undid
  (their insertion records), suppressions or rejection blocks lifted by a
  reset, and dismissed "needs a person" cases.
- Restore (`setu.suggestion.restore`), within 90 days:
  - a restored suggestion is recalculated first, so the relevance gate
    and equity rules apply, and then returns to the queue or is retired
    again with its reason;
  - restoring a rejection withdraws it, lifts its connection block, and
    withdraws its learning label (spec 0053);
  - restoring an undone link returns it as a suggestion waiting for a new
    approval, never as a placed link, and lifts its suppression;
  - restoring a reset puts the suppression or block back.
- A daily purge deletes for ever every item 90 days in the bin: the
  record, scores, anchor text references and decisions' private details
  and reasons. Only the minimal audit events without identifying details
  remain, with id-only labels of decisions that stand. The 90 days is the
  owner's figure, not a setting.
- Erasure does not wait: when a person's data must be erased, every bin
  item holding their content or identity is purged at once.
- Nothing in the bin shows in the main queue or counts towards an
  allowance.

### Undo

The action `setu.suggestion.undo` on an applied link:

1. Read the current message and call shodhan's `plan_undo` with the
   insertion record.
2. `remove`: the add-on deletes our two tags and nothing else, with the
   same hash checks as insertion. Later edits around the link stay byte
   for byte. State `removed`; the connection is suppressed.
3. `nothing to undo`: someone already took the link out. No edit; state
   `gone` (SUT-418, shown as information).
4. `conflict`: nothing changes; state `undo_conflict`. The case joins the
   "needs a person" list with the current text beside the original, for a
   moderator to resolve by hand or to keep the link (SUT-417). It is never
   resolved automatically.

### Links removed outside Sutradhar

- A nightly check, and any edit event on a post holding one of our links,
  compares the post with its insertion records. A link of ours that has
  gone becomes `gone`, including one removed by an edit-history rollback.
- A moderator's own edit that removed it counts as a deliberate removal:
  the connection is suppressed and learning uses it. An author's edit does
  not.
- A forum section that becomes private after a link was placed is flagged
  on the health page, never edited automatically.

### Reject and reset

- Reject (`setu.suggestion.reject`) has optional reason buttons (spec
  0053). The connection is then blocked until a meaningful change: the
  source sentence holding the anchor changed, or the destination's title
  or first post changed.
- Reset (`setu.connection.reset`) lifts a suppression or rejection block,
  with a reason, recorded in the audit log. The next analysis may suggest
  the pair again.
- Approve and reject racing: the first commit wins; the other sees
  SUT-570.

### Shadow mode, uninstall and review-only

- Shadow suggestions are made but never shown, for testing new methods.
  They can never be approved.
- Uninstall leaves published links in posts. Insertion records, decisions
  and suppressions follow the export offer and keep-or-delete choice; once
  deleted, undo is no longer possible, and the screen says so first.
  Uninstall always empties the trash bin, whatever the choice.
- The review page is the only way a link is placed. No setting, playbook
  or ganit output can change that in version 1.

### Data

All in Sutradhar's PostgreSQL with RLS forced. The API role reads
suggestions and writes decisions; the worker role moves apply and undo
states and writes insertions; only erasure and audit export see
`review_decision_private`. A policy hides other sites' rows even when a
query forgets its filter.

- Suggestion records gain `version`, `review_state`, `chosen_destination`,
  `applied_insertion_id`, `original_made_at`, `recalculations`,
  `next_recalculation_at`, `parked_at`, `retired_reason`, `trashed_at` and
  `purge_after` (90 days after `trashed_at`).
- `review_decision`: suggestion, action, destination, reason codes, time,
  pseudonymous actor ref, settings version, outcome code.
- `review_decision_private`: actor ref to site user id; erasable.
- `insertion`: the insertion record, post id, old and new hash, applied
  time, the add-on's result id.
- `connection_block`: canonical source and destination threads, kind
  (suppressed or rejected), fingerprints of both sides when made, made at,
  reset at and by.
- `apply_outbox`: job id, suggestion, kind (apply or undo), idempotency
  key, attempts, next try, lease with heartbeat.
- `needs_person`: case id, kind (undo conflict, or apply failed after the
  save), opened, resolved.
- The add-on keeps `xf_sutradhar_apply_log` (idempotency key, post id, old
  hash, new hash, outcome, time; 7 days). It holds hashes and ids, never
  message text.
- The audit trail (lekha) keeps one minimal, append-only event per action,
  refusals and failures included: suggestion id, action, outcome code,
  time and a pseudonymous actor ref. Identities live apart and can be
  erased without rewriting the sequence.

### Actions, events and endpoints

| Action | Capability |
| --- | --- |
| `setu.suggestion.approve` | CAP-035 |
| `setu.suggestion.reject` | CAP-036 |
| `setu.suggestion.approve_batch` | CAP-037 |
| `setu.suggestion.undo` | CAP-038 |
| `setu.connection.reset` | CAP-039 |
| `setu.suggestion.restore` | new entry: restore within 90 days |
| `setu.suggestion.list`, `.show`, `.preview` | CAP-042, CAP-043, CAP-046 |

- Each action is also a management API route and a `sutra setu suggestion
  <verb>` command, through one permission check.
- Events carry ids, states and codes only: `suggestion.approved`,
  `.applied`, `.apply_failed`, `.rejected`, `.recalculated`, `.parked`,
  `.retired`, `.trashed`, `.restored`, `.purged`, `.stale`, `.removed`,
  `.gone`, `.undo_conflict`, and `connection.reset`.
- The add-on's `apply` and `undo` endpoints are called only by the
  service. They take post id, old hash, new message, new hash and
  idempotency key, and answer with the outcome and the saved hash.

### Failure codes

The shodhan codes SUT-416 to SUT-418 and SUT-424 are reused, and the
equity codes SUT-550 to SUT-558. New codes:

- SUT-570: already decided by someone else (shows who and when).
- SUT-571: the post changed; it is being analysed again.
- SUT-572: source or destination no longer visible to guests.
- SUT-573: the thread already links to that destination.
- SUT-574: the thread's link allowance is used.
- SUT-575: the connection is suppressed or was rejected; a reset or a
  meaningful change is needed.
- SUT-576: the site is unreachable; the link is placed when it answers.
- SUT-577: the add-on refused the request's signature or time.
- SUT-578: not allowed: missing permission, or an agent tried to approve.
- SUT-579: analysis not finished; approval waits.
- SUT-580: restore refused: purged after 90 days, or the destination no
  longer exists; the message says which.
- SUT-581: retired at recalculation; the reason (deleted, private, merged,
  below the threshold) is in the audit log.

No failure leaves a post half edited.

### Security and privacy

- Only staff with the review permission see the page. Every text fetch
  re-checks permission and guest visibility.
- Text on the page is escaped; previews come from shodhan's output, never
  raw HTML.
- The add-on only wraps existing words in a link tag. It never inserts
  supplied text.
- Signing keys are secrets that are never logged. Logs, metrics and events
  carry ids and codes, never post text, anchors or names.

### Observability

- Metrics: decisions by action and outcome; click-to-live time; apply
  retries; outbox depth and oldest age; recalculations by outcome; parked
  and trash-bin sizes; purges; retirements by reason; undo outcomes; links
  gone; review workload (queue age, time to review).
- One trace runs from the click through the service, the job and the
  add-on.
- Alerts: outbox older than 10 minutes; apply failures above 1% in an
  hour; any "needs a person" case older than 7 days.

### Ganit outputs used

All nightly; the service shadow-tests and activates each, and keeps the
last good one when a job is late. Review-queue order (4); "my share", a
fair split across moderators that never hides suggestions except the
opt-in hold-back test (26); contested cases (31); review time per
suggestion (50); often-rejected targets, which rank lower and appear on an
admin list for one-click suppression, never suppressed automatically (97);
changepoints in daily decisions for a workload alert (168); queue size
forecast (206); moderator availability by hour (217); mini neighbourhood
graphs (271); how acceptance fades with age (309); time to decision per
queue (321).

## Limits

- Batch approval: at most 50 per batch, or the lower researched top.
- Recalculation interval: 7 to 90 days. Trash bin: 90 days, fixed.
- Memory, inside the service slice, within the memory limits in AGENTS.md
  section 5.6 (no models): sujhav adds at most 32 MB to the API; the jobs
  (apply, undo, sweeps) use at most 64 MB, holding one post's message at a time,
  with at most 4 jobs at once. Estimates to be measured.
- The add-on adds at most 8 MB of peak memory per request.
- Signed messages older than 5 minutes, dated in the future or with a
  single-use number seen in the last 5 minutes are refused.
- Speed targets on Dell:
  - click to link live in the post: at most 3 s at the 95th percentile and
    10 s at the 99th, with the site answering normally; undo the same;
  - the review page's first content within 1.5 s; a page of 50 cards from
    100,000 waiting within 300 ms at the API;
  - another moderator's approval appears within 1 s at the 95th
    percentile;
  - the approval check (locks, re-checks, outbox) at most 150 ms at the
    95th percentile;
  - the daily sweep and the trash purge, each over 1,000,000 suggestions,
    within 60 s, yielding to live work; one recalculation costs one
    backfill match;
  - the nightly link check covers 100,000 placed links within 30 minutes;
  - the add-on endpoint, without the site's own save, at most 500 ms for a
    5 MB post.
- Every speed figure is held by the 5% median rule.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Rust tests live in `crates/sujhav/tests/` unless stated. All use synthetic
posts, fakes (`SuggestionRepo`, `ForumReader`, `VisibilityOracle`,
`ForumWriter`, `Clock`, `EventSink`, `AuditLog`) and the real shodhan.

1. State machine: every allowed and refused move, and a property test over
   random move sequences.
2. Approval races: two approvals within 10 ms record exactly one; the other
   sees SUT-570 (`approve_race.rs`, and a stress test on the test
   PostgreSQL). One approval sent twice with one key gives one decision and
   one job (`approve_idempotent.rs`).
3. Allowance: eight approvals racing for a thread's last place under loom
   give exactly one success, SUT-574 for the rest and no deadlock
   (`loom_allowance.rs`). A thread with four content links, one quote link
   and two mentions, limit five: the first approval succeeds, the second
   sees SUT-574, counted across the whole thread (`allowance.rs`).
4. Re-checks (`recheck.rs`): edited post gives SUT-571 and stale; hidden
   destination gives SUT-572 and the trash bin; existing link by bare
   address or in a quote gives SUT-573; approval during analysis gives
   SUT-579 and the button is disabled (`approve_wait.rs`).
5. Apply: the saved message equals shodhan's verified text byte for byte,
   no bump, no alert, author unchanged, edit history keeps the old text
   (`acceptance/cap_035_approve.rs` on the Dell test site, and a PHP test
   of the add-on). Worker killed after the save: `applied` once, one link
   (`chaos_apply.rs`, nightly). Site down 10 minutes: every job finishes,
   none dead-lettered, cards said "waiting for the site"
   (`apply_outage.rs`). A post the save would rewrite: nothing saved,
   SUT-424 (`apply_skip.rs`).
6. Undo: a link later wrapped in bold loses only our tags and the bold
   stays (`acceptance/cap_038_undo.rs`, plus a property test over
   generated later edits); a changed link text opens a SUT-417 case
   (`undo_conflict.rs`); a removed pair is not suggested until a reset
   (`acceptance/cap_039_reset.rs`).
7. Reject: the pair never returns unless the anchor sentence, the
   destination's title or first post changes (`reject_return.rs`).
8. Recalculation (`recalculate.rs`): at 29 and 31 days only the older is
   recalculated and nothing is deleted; a hard-deleted destination retires
   its suggestions at once with the reason logged; still-best keeps score,
   count 1 and original date, and sorts before a newer one of equal
   priority (plus a queue-order property test); a better destination
   replaces in one step; private, merged or below threshold retires with
   SUT-581. Three recalculations park (`parking.rs`). Interval 14 is
   accepted; 5 and 120 are refused (`recalculate_setting.rs`).
9. Trash bin: restore after 40 days withdraws the rejection, lifts the
   block and the label (`acceptance/cap_trash_restore.rs`). At 89 and 90
   days only the 90-day item is purged and a restore answers SUT-580
   (`purge.rs`, plus a property test over ages and clocks). A purge killed
   half-way purges every due item once and nothing else (`chaos_purge.rs`).
   Undone links and resets land in the bin and restore as stated
   (`trash_kinds.rs`). Erasure purges all four items of a member and a
   moderator at once (`trash_erasure.rs`). Either uninstall choice empties
   the bin and leaves links in posts (`acceptance/uninstall_trash.rs`).
10. Equity at approval (`equity_approve.rs`): anchor words moved means
    nothing written; anchor share at the limit gives SUT-551; a third link
    in a 300-word post gives SUT-552; a fourth link to one destination in a
    day gives SUT-553 and can be approved the next day (moving clock);
    noindex or hidden gives SUT-555 or SUT-572, a redirect links the
    canonical thread; anchor now in a quote or signature gives SUT-556.
    Five in one batch over the section share give SUT-554
    (`equity_batch.rs`). A→B and B→A at once: one succeeds, the other
    gets SUT-557; D→A closing a loop gets SUT-558 (`equity_race.rs`,
    `loom_equity.rs`).
11. Gate: a suggestion whose acceptance probability fell below the
    threshold is retired with SUT-550 at recalculation; approval does not
    re-run the gate (`gate_recalc.rs`). No setting, playbook or ganit
    output places a link without a person's approval
    (`props_review_only.rs`).
12. Permissions and audit: an agent's token gets SUT-578 and the refusal is
    audited (`permission.rs`); every action has one append-only event with
    no post text, and erasing a moderator keeps the sequence (`audit.rs`).
13. Signing: a reused single-use number, an old time or a changed body is
    refused with SUT-577 and nothing is saved (`sign.rs` and a PHP test).
14. Batch: 20 ticked, three stale, gives 17 applied and three reported
    (`acceptance/cap_037_batch.rs`).
15. Load and speed: another's approval shows within 1 s; 100,000 waiting,
    paged for 2 minutes, answer within 300 ms at the 95th percentile
    (`load_review.rs`); click to live at most 3 s at the 95th percentile on
    Dell, held by the 5% median rule (`benches/approve.rs`).
16. Privacy: logs, metrics, traces and events of a fixture run hold no
    post text, anchors or names (`privacy_scan.rs`). RLS returns no other
    site's or private rows when a query omits its filter (`rls.rs`, SQL
    test against the test PostgreSQL).
17. Rule-critical code (re-checks, allowance, connection rules, state
    machine, hash checks, purge): 100% lines, branches and MC/DC, every
    mutant killed, property tests, and Kani proofs: `applied` is never
    reached without `approving`; the allowance never passes its limit; an
    approved link is never nested and never repeats a connection; the purge
    never deletes an item less than 90 days in the bin.
18. Each fixed bug gets a regression test that fails on the parent commit.

## Changes from the source

- The review page is in the site's admin pages, drawn by the add-on, not
  a separate web admin app.
- The service reads the site through the add-on over HTTPS, since the site
  never calls the owner's PC; re-checks no longer read the forum database
  directly.
- Requests are signed with Ed25519 with time, single-use number and request
  ID, not a shared-secret hash; keys rotate from the app.
- Presence and live updates are part of the review page, not a separate
  live layer.
- Memory now fits the limits in AGENTS.md section 5.6.
- Speed goals follow the 5% median rule; the old speed-multiple goals are
  dropped.
- Ganit is the Rust numerical engine inside the service.
- Coverage adds MC/DC and kills every mutant, as AGENTS.md section 6 sets.

Source: `docs/specifications/linking/suggestions.md`,
`linking/suggestions-2.md`, `linking/suggestions-3.md`,
`linking/suggestions-4.md` and `linking/suggestions-5.md` in the archive
`archive-2026-11-08-old-repo`.
