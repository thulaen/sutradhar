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

[Spec 0120](0120-opt-in-referral-backlink-discovery.md) owns feature
106. A moderation hint cannot read its leads or referral paths.

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

The queue also carries Rakshak's content hints. Spec 0072 sets their
evidence, permission and closing rule; items 1 to 4 below are for
account hints and item 5 is for content hints.

1. Moderators with the "View account hints" permission see each
   account hint: the accounts, the families that matched, and the
   evidence. IP addresses show only to those XenForo already lets view
   IPs.
2. Three choices: dismiss, keep watching, or open the accounts in
   XenForo to act there. Sutradhar records the decision and never acts
   itself. No hint path calls any XenForo write.
3. A dismissed hint moves to the trash bin 30 days after dismissal,
   where staff can still restore it, and is removed for ever 90 days
   later. An erasure request removes a member's hints from the trash bin
   at once.
4. An account hint closes when its account is deleted, or when the
   banned account is later unbanned.
5. Holders of the "View content hints" permission see each content
   hint: the content, its revision, the families that matched and the
   evidence, never accounts. Three choices: dismiss, keep watching, or
   open the content in XenForo to act there. Sutradhar records the
   decision and never acts itself. Dismissed content hints follow item
   3's trash times; its erasure clause is for account hints only, since
   content hints hold no accounts. A content hint closes by spec 0072's
   rule, including on kavach's `spam.content_hint_withdrawn`.

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

#### Wave 1 expert invitation binding

F-0026 · REQ-0051-W1-EXPERT-ROUTING · DES-0051-W1-EXPERT-APPROVAL.
Use this section's optional routing, current topic skill and opt-in,
pending question, and member, thread and forum caps. Recheck these and the
moderator's approval before the native XenForo invitation. WordPress
requires an independently admitted native forum notification adapter.

FX-0051-W1-EXPERT-ROUTING · IT-0051-W1-EXPERT-ROUTING. Reuse the routing,
eligibility, moderator-approval, job 25 and cap cases below. Give two
eligible opted-in candidates on separate waiting threads, an over-limit
expert and an unrelated member. Approve one, and answer the other thread
before delivery. Revoke opt-in on an otherwise eligible pending attempt.
Require one native alert for the still-approved, eligible XenForo case.
Require no second alert on receipt replay, and no invitation for answered,
opted-out, over-limit or unrelated cases. With no admitted WordPress
adapter, require no delivery or email fallback.

### ganit jobs used

ganit receives member, thread and topic ids with counts and times, never
post text, names or IP addresses. Writing style is compared in the service
only. Every result is inert candidate evidence; spec 0077 alone evaluates,
promotes, activates and rolls it back. A late or failed candidate leaves the
verified champion unchanged.

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
- `content_hint`: id, content id, revision, finding id, families,
  scores, state, decided by, decided at, expires at; no accounts.
- `content_hint_evidence`: content hint id, family, reason codes,
  strength, and a summary built from those codes; no IP addresses,
  anchors or post text.
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
  `expert.invited`. `hint.raised` and `hint.decided` carry a hint kind,
  account or content. The hints module consumes kavach's
  `spam.content_hint_raised` and `spam.content_hint_withdrawn` events
  (spec 0072).
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

### Build-blocking platform evidence

The moderation integration record must establish XenForo 2.3's IP record table
and retention setting, the add-on alert handler for custom alerts, and the
permission that lets a moderator view IPs. Each dependent integration remains
blocked until official-version evidence, privacy review and isolated fixtures
are admitted; missing evidence never permits a schema or permission guess.

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
- Depth follows ADR-0020. Only admitted strict-core components require
  100% line, branch and MC/DC coverage and zero surviving killable mutants,
  with equivalent mutants proved and independently reviewed; other code
  follows the owning risk matrix. The two-family rule, no-action guard,
  IP-view permission check, retention deletion, opt-in and mute checks and
  invitation limits retain their assigned mutation gates and property
  tests. Kani proves a hint is never raised with fewer than two families,
  and that no sequence of requests sends a member more than the weekly
  limit.

## Wave 1 member and staff evidence bindings

Keep the exact site, actor, view permission, current source and
native profile. Separate browser-local advice, staff review and a
native notification. Feature 106 remains under 0120, including its
separate privacy review, owner grant and visitor consent. These
features have no access to its leads or referring paths.

| Feature and owning design | Exact planned fixture and boundary |
|---|---|
| F-0101 · REQ-0051-W1-DUPLICATE-TITLE · DES-0051-W1-LOCAL-EDITOR | FX-0051-W1-DUPLICATE-TITLE, BDD-0051-W1-DUPLICATE-TITLE. Compare a current admitted map with an answered public match, withdrawn visibility, a stale map and paste. Match only current eligible public material in the approved local 0029 editor profile. A stale or unproved map reveals no old title. Reuse the zero-typing-request proof; typing or paste makes no request, captures no draft and creates no merge or publication. |
| F-0102 · REQ-0051-W1-TITLE-ADVICE · DES-0051-W1-LOCAL-EDITOR | FX-0051-W1-TITLE-ADVICE, BDD-0051-W1-TITLE-ADVICE. Compare vague and clear local title inputs, disabled settings and an absent map. Require the exact admitted local wording and member privacy profile. Show gentle advice only for the qualifying case. No service call, capture or automatic title replacement. The separate 0117 desktop fan-out route supplies no member-editor authority. |
| F-0103 · REQ-0051-W1-SOLVED-REMINDER · DES-0051-W1-EXACT-NATIVE-NOTICE | FX-0051-W1-SOLVED-REMINDER, IT-0051-W1-SOLVED-REMINDER. Use an answered thread with a current pending solved mark, the eligible asker and explicit current approval. Replay an eligible effect; in separate cases mark the thread solved, revoke approval or withhold the WordPress adapter. Only an admitted native platform route may deliver one reminder. Recheck before delivery and reconcile an uncertain receipt without a second send. The expert invitation route and other channels grant no fallback or actor substitution. |
| F-0104 · REQ-0051-W1-SECTION-SCORECARD · DES-0051-W1-STAFF-EVIDENCE | FX-0051-W1-SECTION-SCORECARD, IT-0051-W1-SECTION-SCORECARD. Compare permitted and forbidden sections, a justified fix and missing evidence or an unresolved tie. Use current native section authority, the 0048 priority rule and separately admitted protected aggregates if used. Return only eligible section evidence and one supported fix. Apply the existing disclosure guard to counts and complements. No hidden section exposure or effect. |
| F-0105 · REQ-0051-W1-STAFF-LESSON · DES-0051-W1-STAFF-EVIDENCE | FX-0051-W1-STAFF-LESSON, BDD-0051-W1-STAFF-LESSON. Open each eligible suggestion type with approved 0041 evidence, a missing lesson and a changed rule version. Show concise dated source-linked guidance under its actual action and current grant. Missing guidance is unavailable. No automatic approval, apply, actor impersonation or new inference route. |

The existing editor, analytics and native notification checks keep
their owners. QR-108 retains exact profile and fixture admission.
The cases describe planned evidence only.

## Changes from the source

- The maths jobs run in ganit, the Rust numerical engine.
- Moderator pages are desktop app screens built from SDS components.
- Speed follows the 5% median rule; the 20x goal is kept (spec 0007).

Source: `docs/specifications/signals/moderation.md` in the archive
`archive-2026-11-08-old-repo`.
