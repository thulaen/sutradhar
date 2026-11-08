# 0051: Moderation hints

Two helpers for moderators. First, account hints: when a new account
looks like a banned one (for example it logs in from the same address and
writes the same way), Sutradhar shows a hint with its evidence in a
moderator queue. It never bans, warns or restricts anyone itself, and one
signal alone is never enough. Second, expert routing, optional and off by
default: when a question has had no reply for a while, Sutradhar suggests
up to three members who answered similar questions well. A moderator
approves each invitation; only members who said yes to invitations get
them, and never too often. The modules are `panchayat` ("village
council": moderation, module `hints`) and `uttar` ("answer", module
`experts`). It is part of roadmap Phase 6.

Example: a question about a router has no reply after 12 hours.
Sutradhar suggests two members whose accepted answers cover the same
router series. The moderator approves one; that member gets a XenForo
alert: "A member asked about routers you know well. Can you help?"

## What it does

### Scope

In scope: hint signals, scoring, the review queue and decisions;
coordinated-voting hints from spec 0050; expert candidates, member
preferences, invitations and limits. Out of scope: any automatic action;
spam detection (kavach); automatic moderation rules (other `panchayat`
work, with its own specification).

### Design

1. Traits with fakes: `MemberReader` (read-only XenForo data through
   sampark: accounts, bans, IP records, posts; fake `FixtureForum`),
   `StyleModel` (writing-style similarity; fake with fixed values),
   `SimilarThreads` (from the reading index, spec 0022, and the
   knowledge graph, spec 0049), `Notifier` (XenForo alerts through the
   add-on; a recording fake) and `Clock`.
2. One evidence model serves every hint kind. One rate-limiter type
   (governor) serves invitations.
3. Hints are recomputed for new accounts only (first 30 days) and on a
   ban, not for every member nightly.

### Hint signals (independent families)

1. Network: the new account shares an IP address with a banned account
   within 30 days. XenForo's own IP records are compared in memory;
   addresses are never copied into Sutradhar's tables. Shared addresses
   (universities, mobile carriers, VPNs) weaken this family.
2. Writing: character n-gram style similarity between the accounts'
   guest-visible posts above a threshold, with at least 5 posts each.
3. Names: username similarity (strsim) above a threshold.
4. Timing: active hours and posting rhythm closely matching.
5. Behaviour: reacting to or replying to the same members, or to each
   other (co-voting from spec 0050, and rings of accounts reacting to
   each other from ganit job 263).

A hint is raised only when at least two different families agree. It
shows each family's evidence and strength. Email addresses are never
used, and no hint input type has an email field.

### The review queue

1. Moderators with the "View account hints" permission see each hint:
   the accounts, the families that matched, and the evidence. IP
   addresses show only to those XenForo already lets view IPs.
2. Three choices: dismiss, keep watching, or open the accounts in
   XenForo to act there. Sutradhar records the decision and never acts
   itself. No hint path calls any XenForo write.
3. A dismissed hint moves to the trash bin 30 days after dismissal,
   where staff can still restore it, and is removed for ever 90 days
   later. An erasure request removes a member's hints from the trash bin
   at once.
4. A hint closes when its account is deleted, or when the banned
   account is later unbanned.

### Turning hints on

Hints are off until the administrator switches them on. The page then
shows: "Hints use members' IP addresses and writing. Check that your
privacy policy covers this." and records the acknowledgement. Switching
on without it is refused.

### Expert routing (optional, off by default)

1. A thread qualifies when it is guest-visible, is a question (ganit job
   67's question detector, or a chosen forum), is not locked, and has no
   reply after a set time (default 12 hours) or a high chance of staying
   unanswered (job 317).
2. Candidates: members with skill in the thread's topic (job 102, from
   accepted answers and well-voted replies in similar threads), active in
   the last 90 days, not the author, not banned, and opted in to
   invitations. Job 318's response times break ties.
3. Up to three candidates go to the moderator, chosen by job 25's plan,
   which spreads invitations so no expert is overloaded. A member the
   plan names who has opted out is dropped. If no plan is active, the
   three highest skill scores within the limits are offered. The
   moderator approves, edits or declines. No alert is sent without
   approval.
4. Limits, all settings: at most 2 invitations per member a week, 3 per
   thread, and 20 per forum a day.
5. The invitation is a XenForo alert sent through the add-on. Members can
   say "not interested in this topic" (then they are never suggested for
   it) or turn invitations off on their preferences page.
6. A thread answered while an invitation waits has the invitation
   withdrawn.

### ganit jobs used

ganit receives member, thread and topic ids with counts and times, never
post text, names or IP addresses. Writing style is compared in the
service only. Results are shadow-tested and the last good version stays
if a job is late.

- Job 67 question detector (from ids and the service's features, not
  text).
- Job 102 expert skill per topic.
- Job 25 expert routing with workload caps.
- Job 317 time to first reply and the chance a thread stays unanswered;
  job 318 expert response times.
- Job 263 rings of accounts reacting to each other, as the behaviour
  family's evidence.

### Data

- `hint`: id, accounts, families, scores, state, decided by, decided at,
  expires at.
- `hint_evidence`: hint id, family, summary, strength; no IP addresses.
- `expert_pref`: member id, opted in, muted topics.
- `expert_invite`: thread id, member id, reason, state, moderator, sent
  at.
- All in PostgreSQL with forced row-level security. Hint tables are
  granted only to the role behind the moderator screens and the hint
  job.

### API, events and commands

- Management API: `GET /api/hints`, `POST /api/hints/{id}/decide`,
  `GET /api/experts/candidates/{thread}`, `POST /api/experts/invite`,
  `PUT /api/experts/prefs/{member}`.
- Events: `hint.raised`, `hint.decided`, `expert.suggested`,
  `expert.invited`.
- Commands: `sutra panchayat hints`, `sutra uttar experts <thread>`.

### Failures

- SUT-690: too little text for writing style. The family is skipped, and
  the hint says so.
- SUT-691: an invitation over a limit. Refused, with the reason shown.
- SUT-692: alert delivery failed. Retried, then shown as not sent.
- SUT-693: a hint's account was deleted. The hint closes.

### Security and privacy

1. Data is kept to the minimum: evidence summaries, no copies of IP
   addresses or post text.
2. No automatic bans, warnings or restrictions.
3. Erasing a member deletes their hints, evidence, preferences and
   invitations.
4. Metrics carry no member labels.

### Edge cases to cover

Shared addresses; households with two honest members; banned accounts
later unbanned; members writing in two languages; a candidate expert who
is staff; threads answered while an invitation waits.

### Facts to check when built

XenForo 2.3's IP record table and its retention setting; the alert
handler an add-on uses for custom alerts; which permission lets a
moderator view IPs.

## Limits

- Runs in the worker, at most 64 MiB above idle, inside the service slice.
- A hint for a new account appears within 10 minutes of its first post.
- Expert candidates for a thread appear within 3 seconds.
- A style comparison takes at most 20 ms.
- Invitations: at most 2 per member a week, 3 per thread, 20 per forum a
  day.
- Dismissed hints: trash bin after 30 days, purged 90 days later.

## Footprint budget

This part adds nothing to public pages. Invitations reach members as
XenForo alerts through the add-on, and the member preferences page gains
an invitations setting.

## How it is tested

Rust tests in `panchayat` and `uttar`:

- Only a shared IP gives no hint.
- A shared IP and similar writing give a hint with both families'
  evidence.
- For any hint and decision, no XenForo write is made.
- A moderator without XenForo's IP permission sees no address.
- Retention: a hint dismissed 31 days ago is in the trash bin and can be
  restored; at 121 days it is gone for ever; an erasure request removes
  it from the trash bin at once.
- Switching hints on without the privacy acknowledgement is refused.
- 4 posts skip the writing family with SUT-690.
- With routing off, no candidates or invitations exist.
- Candidates exclude the author, banned and opted-out members.
- With no moderator approval, no alert is sent.
- Property test: for any request sequence, no member gets more than 2
  invitations in 7 days.
- A member who mutes a topic is never suggested for it.
- Erasing a member removes their hints, evidence and invitations.
- Contract: job 25's plan naming an opted-out member has that member
  dropped and the limits still hold; with no active plan, the three
  highest skill scores within the limits are offered.
- Two accounts sharing only an email address or domain raise no hint.
- Benchmark: a style comparison within 20 ms, under the 5% median rule.
- Depth: 100% line and branch coverage. The two-family rule, the
  no-action guard, the IP-view permission check, retention deletion,
  opt-in and mute checks and the invitation limits have no surviving
  mutants and have property tests. Kani proves a hint is never raised
  with fewer than two families, and that no sequence of requests sends
  a member more than the weekly limit.

## Changes from the source

- The maths jobs run in ganit, the Rust numerical engine.
- Moderator pages are Dioxus app screens built from SDS components.
- The 20x speed goal is replaced by the 5% median rule.

Source: `docs/specifications/signals/moderation.md` in the archive
`archive-2026-11-08-old-repo`.
