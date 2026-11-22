# 0126: Writing sessions and learning

A writer can quiet the editor, set a timer, record their progress
and make a private note about a session. They can set a challenge,
work through an imported lesson and return to parts of a draft
that need more work.

Spec 0121 owns personal writing workspaces and protected records.
Spec 0125 owns story planning and story-specific prompt content.
This spec owns the local timer, writing progress, draft markers,
learning materials and challenge workflow. Live group and expert
services keep their own route decisions under DR-114.

Example: Nia presses “Focus” to hide the checks and starts a
25 minute timer titled “Chapter two” with a goal of 500 words.
She hides the window and the timer keeps its own clock. The laptop
suspends before the deadline. On return the panel says “Time was
interrupted” and offers resume, finish or cancel. She finishes,
presses “Save session” and corrects the word count in her diary.
She adds a “Mark for later” marker on a weak sentence and writes a
private reflection. When she shares her progress table, the
reflection is not in it.

## Words used here

- **Session**: one timed or diary-recorded period of writing,
  planning or revision.
- **Diary record**: the saved, correctable duration and words of a
  session. Measured intervals are kept apart from it.
- **Checkpoint**: the last proven point of a running timer.
- **Marker**: an inline note anchored to an exact range or point.
- **Reflection**: a private author-only note after a session.
- **Lesson**: imported, authorised material the writer works through.
- **Challenge**: a dated local sprint with own goals and milestones.
- **Streak**: consecutive days or weeks with eligible saved
  writing.
- **Activity export**: an authorised summary of recorded sessions.

## What this spec designs

| Feature | What the writer sees |
|---|---|
| H01 Focus | Panels hidden and focus restored |
| H02 Timer | A timer that survives a hidden window |
| H03 Session history | Saved, correctable session rows |
| H04 Goals and progress | Targets, trends and streaks with table views |
| H05 Draft marker | Inline “Mark for later” markers |
| H06 Private reflection | Author-only notes |
| H07 Guided pause | A short local exercise |
| H08 Practice and lesson | Exercises and imported lessons |
| H09 Sprint and challenge | Local milestones and rewards |
| H10 Manual event preparation | A local schedule and checklist |
| H11 Progress record | An authorised activity export |

## Behaviours

The [timer guide][timer], [session guide][sessions],
[placeholder guide][marks], [reflection guide][reflect],
[guided break guide][break], [challenge guide][challenge],
[streak guide][streak] and [writing suite features][features]
describe the source outcomes observed on 20 November 2026. Keep
the exact source surface, plan and observed conditions in the
coverage register. Use original wording, materials and exercises.

| ID and outcome | Local behaviour | Acceptance |
|---|---|---|
| H01 Focus | Hide checks, outline and progress panels without discarding any draft or changing model settings. Restore keyboard focus when exiting | Toggle in the middle of an edit. Resume at the same selection, with saved checks and marks unchanged |
| H02 Timer | Start from a title, duration, mode and optional goal. Extend, pause, continue or finish early. A hidden timer keeps its own clock under the desktop service | Hide and reopen. Pause, extend and finish before expiry. Each operation updates the exact timer once |
| H03 Session history | Save duration, words, or both. View today's totals and historical rows. Correct a title, date, diary duration or words, remove a row or save and start another session | Replay a save-and-start action and correct each allowed field. Exactly one new session starts. Preserve measured time and keep removed rows out of restore |
| H04 Goals and progress | Choose a project, starting word count, target, dates and time zone. View trends with table alternatives, current and best matches and local awards | Distinguish zero, absent and corrected counts. Cross local day and weekly barriers. Keep logged writing streaks and any separately admitted weekly use outcomes distinct |
| H05 Draft marker | Add or remove an inline marker with a deliberate editor action. Search, jump to and resolve an exact range or point. Ordinary brackets in text stay literal | Mark two occurrences of the same text, move one through an edit and re-open. Do not pick a new matching passage when the anchor is lost |
| H06 Private reflection | Optionally record mood, confidence and notes after any writing, planning or revision session. Compare past reflections privately | Check a note remains private when a progress view is shared. Erase that note and its derived indicators |
| H07 Guided pause | Start a short original, local exercise with text and optional media. Stop any time, repeat it or return to the same draft | Test independently paced cues, progress and an accessible silent mode. Stop, resume and repeat with keyboard only. No health claim, mandatory pause or lost work |
| H08 Practice and eligible lesson | Use independent short writing exercises or import authorised materials with instructions and optional captions. Save progress and replay the exercise | Open one approved material revision. Change bytes, use the wrong right or malformed media and refuse before preview. Saved progress is bound to admitted bytes. The lesson cannot open a URL, run code or write a draft |
| H09 Local sprint and challenge | Choose a timed writing or planning sprint, session date, milestones and goals. View active and archived challenges and local rewards earned from saved records | Compare daily total, cumulative total, own target and average within each timeline. Try a session outside entry dates and a later allowed correction. Withdraw a milestone when eligible evidence falls below it. Unstarted sprints cannot count |
| H10 Manual event preparation | Keep a schedule and private preparation checklist for a user-supplied workshop, contest, sprint or peer practice group. Prepare a local copy for deliberate handoff | Import a synthetic schedule. Preview participants and materials only to an authorised viewer. No RSVP, live room, entry, payment or invitation happens |
| H11 Progress record | Export an authorised summary of recorded sessions with units, time frame and event origins | Separate writer-entered counts, corrected entries and measured duration. Withhold unapproved text, reflections and draft content. Keep the record labelled as activity evidence |

Keep active online workshops and co-writing presence, hosted
critique, courses with live experts, networking, sponsor offers
and contest registration distinct from their local counterparts.
An entered plan, saved schedule, reward or rehearsal cannot stand
for a service effect. Take uncovered outcomes to the owner under
DR-114. QR-119 holds the mapping of remaining gaps and the evidence
needed. This spec grants no provider access.

## Data

| Record | Fields |
|---|---|
| Session | Workspace, person, session ID, mode, state, title, goal, planned duration, process and boot clock identity, bounded measured intervals and checkpoint, recorded start and finish, separately labelled diary duration and words, diary date, zone and version |
| Timer operation | Session ID, expected version, actor, operation, requested value, idempotency key and recorded effect |
| Goal | Workspace and person, project, interval and zone revision, starting count, target count or time, policy version, optional milestones |
| Draft marker | Scope, actor, document/revision/block IDs, exact 0121 byte range or point with before/after insertion affinity, edit map chain and context digest, note and tag, state open/resolved/anchor lost and version |
| Reflection | Private author, optional session reference, mood/confidence selections and text, creation time, encrypted purpose scope, revision and erasure identity |
| Local lesson | Identity, signer or supplied origin, content and media hashes, rights affirmation, preserved source and locale, lesson revision and admitted bounds |
| Challenge | Owner, event and entry dates, correction cutoff and zone, independent exercise ID, allowed session modes, own goal and milestone rules, associated session references and policy version |
| Activity export | Exact recipient, selected row IDs and versions, units, reason, time range and payload digest, current grant and expiry |

Reflection has a separate purpose and encryption key from shared
activity. Only the author can see, search, change or erase it.
Refuse access for all other actors including workspace administrators
and local tools. Shared, scheduled and app-tool outputs exclude
its existence, text and derived patterns. Keep reflection and
private marker indices scoped to current grants. Erasure removes
dependent records and cached indicators and applies after restore.
For lessons, require allowed rights, exact content and media
identity and bounded admitted decoding before preview. Treat active
URLs or script as inert or refuse the input. Bind progress to the
approved immutable lesson revision. Use the same file ledger and
erasure rules as 0121.

## Timing and progress

1. Acquire one active timer per actor and workspace atomically.
   Operations require its exact version and idempotency key. A
   replay returns the committed effect. Changed bytes with the
   same key are refused. A monotonic clock that only advances
   measures time within the admitted boot and service epoch.
   Close each interval once, at or before its deadline.
2. At the deadline, expire before accepting another operation.
   Pause, extension, early finish and expiry compete under the
   version fence. A commit closes or modifies one interval and
   persists before the view changes. Keep a bounded checkpoint.
   A hidden normal window leaves its service running. Portable
   closure and Quit follow spec 0030. On suspend outside the
   admitted clock profile, power loss or service restart, retain
   the last proven checkpoint. Say “Time was interrupted” and
   offer resume, finish or cancel. Never fill gaps from civil
   clock time. An expired session counts only after save.
3. Diary correction can change date, title, recorded duration
   and words. Preserve and label original measured intervals.
   Save and start again commits the diary record and exactly one
   separate new timer, or neither. A manual count names its
   source. For automatic net change use eligible 0123 evidence
   and assign each exact covered revision interval to at most
   one session, with atomic ownership. Keep counts for words
   written, net revision change and recorded goals distinct.
4. Past diary dates retain the stored time zone and local day.
   Daily writing streaks count consecutive days with eligible
   positive saved activity. Report current and best periods and
   their supporting dates. A weekly writing period runs Monday
   through Sunday and uses the same saved events. Open periods
   do not close a streak prematurely. The reference also has a
   Monday-to-Sunday UTC product-use streak, even without writing.
   Keep that separate. It needs a privacy-reviewed explicit local
   activity alternative or a gap decision under DR-114. Starting
   the app silently records no qualifying event.
5. Apply only saved, in-date and admitted sessions to a challenge.
   Calculate daily and cumulative eligible totals and average
   per declared elapsed day using the same recorded time zone.
   State the denominator. A correction within the declared
   cutoff or deletion recomputes the milestone. Deletion still
   takes effect after the cutoff. A badge records the rule
   and evidence versions and cannot revive an erased note or
   session. Source sponsor, RSVP and live participation effects
   retain independent gap identities.
6. Saving a draft marker uses a 0121 revision fence. Map it
   through exact admitted edit lineage or mark it “Find this
   marker again”. Missing or ambiguous lineage cannot choose
   the same words at a new position. Resolve or delete a marker
   separately from deleting source text.

## Layout and failure behaviour

The editor offers “Focus”, “Start timer” and “Mark for later”.
The session panel shows running, paused, expired or interrupted
state, and a separate “Save session” action. “Progress” opens an
accessible table of sessions, goals and challenge milestones.
Reflections open from an author-only tab. Shared previews and
exports contain eligible session data and exclude reflections
and their presence. Show any enabled media controls.

Protect exact report files under specs 0100 and 0118. Preview
what would leave the workspace and require current authority
for the recipient, scope, exact bytes and route. Offer manual
handoff only for a supported route. Exports never purport to be
proof of identity. Keep an earlier authorised result until a
new valid save or delete. Unknown time, word count or source
provenance has a plain label and no made-up estimate.

## Settings

| Setting | Type | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `timer.active_per_actor` | whole number per workspace | 1 | fixed at 1 | not changeable |
| `timer.max_minutes` | minutes | 480 (proposed) | 1 to 1,440 (proposed) | next timer |
| `goal.week_start` | day | Monday | fixed; weeks run Monday through Sunday | not changeable |
| `challenge.correction_cutoff` | date and zone, set per challenge | none until declared | after the entry dates and at most 90 days after the last entry (proposed) | next correction |
| `reflection.enabled` | on or off | off (proposed) | two values | at once |
| `lesson.import_enabled` | on or off | off (proposed) | two values; each lesson needs rights and exact bytes | next import |
| `activity.export_expiry_days` | days | 7 (proposed) | 1 to 90 (proposed) | next export |

No setting records a qualifying event when the app merely starts,
shares a reflection, or grants a provider access.

## Failure codes

Decision DR-130 records SUT-5500 to SUT-5599 for this spec. It was confirmed
by the owner on 21 November 2026. Only the first seven, SUT-5500 to SUT-5506,
are in use.

| Code | Cause | Result |
|---|---|---|
| SUT-5500 | Suspend, power loss or restart | “Time was interrupted”. Resume, finish or cancel |
| SUT-5501 | Wrong version, or key reused with changed bytes | Operation refused |
| SUT-5502 | Marker lineage missing or ambiguous | “Find this marker again” |
| SUT-5503 | Lesson rights, bytes or media not admitted | Refused before preview |
| SUT-5504 | Export grant, recipient or bytes changed | Handoff refused |
| SUT-5505 | Outcome needs a live or hosted service | Not covered. Refer to the owner under DR-114 |
| SUT-5506 | Revision interval already owned by a session | Count refused |

## Security and privacy

- Reflections have their own purpose and key. Only the author can
  see, search, change or erase them. Administrators and local
  tools are refused.
- Shared, scheduled and app-tool outputs exclude reflections, their
  existence and derived patterns.
- Lessons are data. Active URLs and script are inert or refused.
  They cannot open a URL, run code or write a draft.
- Exports preview what would leave, need current authority for
  recipient, scope, exact bytes and route, and never prove identity.
- Workshop, contest and event preparation makes no RSVP, entry,
  payment or invitation. No provider access is granted.
- Files follow the same ledger and erasure rules as spec 0121.

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| REQ-0126-COVER | Every outcome MUST have source evidence. An absent live outcome MUST NOT count as covered by a local timer or schedule. | IT-0126-COVER |
| REQ-0126-TIME | One timer per actor and workspace MUST hold. Operations MUST be version-fenced and idempotent, and gaps MUST never be filled from civil time. | FI-0126-TIME |
| REQ-0126-GOAL | Counts, streaks and milestones MUST use the stored zone. A revision interval MUST NOT count twice. | UT-0126-GOAL |
| REQ-0126-MARK | A marker MUST follow its exact lineage or be marked for finding again, and MUST NOT take an unrelated match. | BDD-0126-MARK |
| REQ-0126-PRIVATE | Reflections MUST stay author-only and out of shared output, and a lesson MUST NOT leak or call out. | E2E-0126-PRIVATE |
| REQ-0126-LOCAL | Every local behaviour MUST work with the network blocked and MUST refuse any unapproved effect. | IT-0126-LOCAL |
| REQ-0126-BOUND | Each stated limit MUST work on each version 1 platform, and a failed timer or job MUST leave the draft responsive. | E2E-0126-BOUND |

## How it is tested

Designed, not run.

- **IT-0126-COVER:** check every outcome and distinct variant against source
  records. An absent live outcome or missing approved gap cannot count as
  covered by a local timer or schedule
- **FI-0126-TIME:** race expiry, pause, early finish and extension. Try reused
  keys with changed bytes, close normal and portable windows, Quit, restart and
  lose power. Preserve known elapsed intervals and show interrupted gaps.
  Save-and-restart creates one record and one new timer
- **UT-0126-GOAL:** replay synthetic dates and weekly boundaries, add a
  starting count and correct measured time in the diary without changing the
  original. Refuse counting one revision interval twice. Compare daily writing
  with weekly open and closed periods. Recompute each affected eligible
  milestone after edit or deletion
- **BDD-0126-MARK:** add, find, relocate, resolve and delete one marker at a
  selection and one at a point. Test literal brackets, exact inline actions,
  paste, branch, undo and restore with identical passages. An unrelated match or
  stale span cannot take the marker
- **E2E-0126-PRIVATE:** switch users, cancel authorisation between preview and
  handoff, deny even an administrator's reflection search. Try a queued export
  with private fields, an active lesson, erased indicator and old restored
  state. Observe no leak, undisclosed export or network call
- **IT-0126-LOCAL:** with network blocked, prepare a workshop and contest
  schedule. Refuse any unapproved effect. Keep local goals, date and excerpt
  review usable
- **E2E-0126-BOUND:** on each version 1 platform test empty, one, admitted
  maximum and over-limit history, markers, timers and lesson files. A failed
  timer or job leaves the draft responsive

QR-119 records the exact workload, time and storage bounds and
remaining source distinctions. Give each local activity its
own rights and test evidence.

## Changes from the source

The sources are public guides observed on 20 November 2026. This
spec keeps their outcomes as local behaviour with original
wording and exercises. Live workshops, co-writing, hosted
critique, expert courses, networking, sponsor offers and contest
registration are not built here. They go to the owner under
DR-114 and QR-119. The reference's UTC product-use streak is kept
apart from the writing streak.

[timer]: https://help.prowritingaid.com/article/417-how-to-use-write-mode
[sessions]: https://prowritingaid.com/features/writing-sessions
[marks]: https://help.prowritingaid.com/article/436-how-to-use-placeholders
[reflect]: https://prowritingaid.com/features/reflections
[break]: https://help.prowritingaid.com/article/438-what-is-the-resets-feature
[challenge]: https://help.prowritingaid.com/article/380-how-to-use-the-novel-november-dashboard
[streak]: https://help.prowritingaid.com/article/443-what-are-streaks
[features]: https://prowritingaid.com/features
