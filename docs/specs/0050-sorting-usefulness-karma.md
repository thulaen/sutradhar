# 0050: Sorting, usefulness and karma

This part lets readers sort thread lists and replies by Hot (popular
now), Rising (gaining fast), Top (highest score in a period), Best (most
reliably liked) and Controversial (many votes both ways). Replies still
open in date order unless the reader picks another sort. Votes come from
XenForo reactions and from a new "Was this useful?" pair of buttons, open
to members and guests. Each member gets karma, overall and per forum,
from the reactions their public posts receive. The modules are `shikhar`
("peak": votes, formulas and flags) and `karma` ("deeds"). It is part of
roadmap Phase 6. This is not link ranking (spec 0027).

Example: reply A has 9 up and 1 down; reply B has 50 up and 40 down.
Best puts A first (confidence score 0.72 against 0.49), because A is
liked by nearly everyone who voted.

## What it does

### Terms

- Up and down: a vote's direction. Score: ups minus downs.
- Confidence score: the lower edge of the likely true share of up votes
  given the votes so far, so small samples are judged cautiously.
- Window: the period for Top (day, week, month, year, all time).
- Karma: a member's total from reactions received on public posts.

### Scope

In scope: vote mapping; the five sort formulas; recalculation; the
usefulness buttons and their display; guest vote checks; suspicious
voting flags; karma; what the add-on shows; signals to learning. Out of
scope: graph relevance (spec 0049), link ranking (spec 0027), linked
account hints (spec 0051) and spam detection itself (kavach).

### Votes

1. Each reaction type's direction follows the administrator's XenForo
   score (positive up, negative down, zero neutral), except Sad and
   Angry, which are always down, whatever the administrator set.
2. Every reaction type, neutral ones included, is also passed to
   learning with its own identity.
3. Usefulness votes: "Yes" is up, "No" is down. One vote per member or
   guest per item, changeable and removable.
4. Votes on non-public content never count. A member's own votes on
   their own posts are excluded.
5. One vote mapping serves sorting, karma and learning.

### The formulas

Specified independently, inspired by the published formulas of Reddit's
former open-source code, with no claim that they match any site's
current ranking. Times are seconds since 1 January 2020.

1. Hot: `s = ups - downs`; `order = log10(max(|s|, 1))`; `sign` is 1, 0
   or -1 by `s`; `hot = sign * order + created / D`, where `D` is each
   forum's decay from ganit job 121 (default 45,000 seconds, so every
   ten times more score is worth 12.5 hours of newness).
2. Best (replies): with `n = ups + downs`, `p = ups / n` and
   `z = 1.281551565545` (80% confidence), the Wilson lower bound
   `(p + z²/2n - z * sqrt(p(1-p)/n + z²/4n²)) / (1 + z²/n)`; 0 when
   `n = 0`. Worked: 9 and 1 gives 0.7176; 50 and 40 gives 0.4880. Best
   stays plain Wilson unless a shadow trial proves the ganit-weighted
   version (job 105 priors, job 104 reliability) agrees better with
   usefulness votes.
3. Top: `s` counting only votes cast inside the chosen window, for items
   created inside it.
4. Controversial: 0 if ups or downs is 0; otherwise
   `(ups + downs) ^ (min(ups, downs) / max(ups, downs))`.
5. Rising: ganit job 113 gives each young item a surprise score (how far
   its recent votes exceed what its forum and age predict) and job 44
   the per-forum threshold for "rising"; the service applies them as
   votes arrive. Until those results exist, or if they are late, the
   fallback is: for items under 24 hours old, the score gained in the
   last 3 hours divided by `(age in hours + 2) ^ 1.5`.
6. Ties break by newer first, then by id, so order is always stable.
7. Every constant is a setting with these defaults. ganit's versioned
   values replace them only after a shadow test and can be rolled back.

### Modules and boundaries

1. `karma` depends on `shikhar`'s vote mapping, never the reverse.
2. Each formula is a pure function behind the trait `SortFormula`
   (`score` from counts and times), so a new sort is a new type and
   nothing else changes.
3. Other traits with fakes: `VoteReader` (the add-on's table and XenForo
   reactions, read-only through sampark; fake `FixtureVotes`),
   `ScoreWriter` (the add-on's batch endpoint; a recording fake),
   `ScoreStore` (PostgreSQL; in-memory fake) and `Clock`.
4. One incremental engine for all five sorts, with integer counters per
   item and window. Scores are derived, never edited by hand.

### Sorting and link ranking

1. The five sorts are what readers choose to see. They are not
   link-ranking inputs.
2. The link scorer may use only two rates from `shikhar`: Best's
   confidence score and the share of usefulness votes that are positive.
   These only reorder candidates that passed the relevance gate (spec
   0027).
3. Members' reactions and votes come from signed-in accounts that
   XenForo already checks, so they carry no visit label. Guests' votes
   count only when the visit passed darshak's `human` checks.
4. Hot, Top, Controversial, Rising, vote totals and karma totals never
   enter link ranking.

### Where and how sorting shows

1. Thread lists in forums, tag pages and "What's new", and replies in a
   thread. Replies default to date order; the reader's choice is
   remembered per session.
2. A vote or reaction event updates only that item's score and, for Top,
   its window counters. A full rebuild exists for formula changes, and
   incremental updates queue behind it.
3. Scores reach XenForo through the add-on's own endpoint in batches
   every few seconds, stored in the add-on's table, so XenForo sorts with
   one indexed join and never waits on Sutradhar.
4. If Sutradhar is down, the sort options use the last stored scores,
   marked with their age. Date order always works.

### Usefulness buttons

1. Under each guest-visible thread and resource: "Was this useful?" with
   Yes and No.
2. After voting the reader sees their own vote (highlighted, changeable),
   the positive and negative counts, and a summary such as "82% found
   this useful (45 votes)". Under 5 votes the summary says "Not enough
   votes yet".
3. The add-on stores votes in its own table through XenForo. Sutradhar
   reads them read-only and receives an event per change.
4. Accessible: real buttons, keyboard use and announced results, to WCAG
   2.2 AA.

### Guest vote checks

A guest vote is accepted, and then counts exactly like a member's, only
when all of these pass:

1. A guest vote token: a random value in a first-party cookie, set only
   when the guest presses a button (strictly needed to remember the
   vote). A guest who only reads gets no cookie.
2. One vote per token per item; at most 30 votes per token a day and 60
   per address hash an hour (the address held in memory only).
3. The visit carrying the vote passes darshak's `human` checks (spec
   0034: 5 visible seconds, an interaction, a valid page token, not a
   crawler or data centre). A visit labelled `suspected_automation` is
   refused and the reason recorded. For a guest whose browser sends
   Global Privacy Control or Do Not Track, or on a site with "require
   consent" on and no consent given, the checks run in memory only and
   no visit is recorded; the guest can still vote.
4. kavach's spam check, where present, does not refuse it.

A refused vote is not counted and the guest sees "Your vote could not be
recorded". Nothing is silently counted then dropped.

### Suspicious voting (flag, never change)

1. Patterns flagged for moderators: many votes on one member's posts
   from new accounts; groups of accounts that vote together far more
   than chance (co-voting overlap); bursts from one address hash; one
   account's votes all against one member; ganit job 178 (reactions
   piling onto one thread) and job 263 (rings of accounts reacting to
   each other).
2. A flag shows the evidence and the affected scores and karma.
3. Votes stay counted until a moderator approves excluding them.
   Exclusion is reversible and logged. XenForo's reactions are never
   changed.
4. Hints about linked accounts go to moderation hints (spec 0051).

### Karma

1. For each member, overall and per forum, the service keeps tallies of
   reactions received on their guest-visible posts, using the vote
   mapping (Sad and Angry down). Usefulness votes do not count towards
   karma.
2. The karma shown is a Bayesian average (ganit job 106): each member's
   tally is pulled towards their forum's typical value until they have
   enough reactions to stand on their own, so three likes on a first
   post do not outrank years of steady help. ganit returns the forum's
   prior value and strength. Reactions are weighted by each reactor's
   reliability (job 104: members who react to everything count less).
   Job 200 watches for karma inflation and returns a normalisation
   factor. The service computes each member's karma from these as
   reactions arrive, so updates stay instant.
3. Worked example, prior 0.6 and strength 20: 18 up and 2 down gives
   (0.6 × 20 + 18) / (20 + 20) = 0.75, shown on a 0 to 100 scale as 75,
   with the number of reactions beside it.
4. Overall weighted tallies are the sum of the per-forum tallies.
   Overall karma is the Bayesian average of those tallies with the
   overall prior, not the sum of the forums' karma.
5. Until job 106's first result is active, karma shows the plain tally,
   marked "provisional".
6. It updates incrementally when a reaction is added, removed or
   changed; a post is edited, moved, hidden, deleted or restored; a
   thread moves to a private node; or an administrator changes a
   reaction's score (a full recompute for that reaction type).
7. Karma from content later deleted as spam is taken back at once.
8. Visible to members and visitors on profiles and member cards, through
   the add-on, labelled "Karma".

### ganit jobs used

ganit receives vote and reaction counts by item, member and forum id,
never text or names. Every result is shadow-tested, activated, can be
rolled back, and the last good version stays if a job is late. A result
that is negative where it must not be, or NaN, is refused and the active
value stays.

- Job 104 reactor reliability (karma and Best); job 105 Best reply
  priors per forum; job 106 karma as a Bayesian average; job 113 Rising
  surprise scores.
- Job 44 Rising and trending thresholds per forum.
- Job 121 Hot's decay per forum, replayed against engagement.
- Job 178 reactions piling onto one thread, raised as a flag.
- Job 200 karma inflation and its normalisation factor.
- Job 263 rings of accounts reacting to each other, raised as a flag and
  passed to moderation hints.

### Data

- `shikhar_item_score`: item kind, id, ups, downs, hot, best,
  controversial, rising, updated at.
- `shikhar_window`: item, window, ups, downs.
- `karma_total`: member id, forum id (0 for overall), weighted ups and
  downs, karma, parameter version, updated at.
- `ganit_param`: job, version, values, state (shadow, active or rolled
  back); ganit's shared parameter registry.
- `vote_flag`: id, pattern, accounts, items, evidence, state, moderator,
  decided at.
- `vote_exclusion`: flag id, vote ids, active.
- Usefulness votes live in the add-on's table.
- Every service table has forced row-level security, per site.

### API, events and commands

- Management API: `GET /api/shikhar/scores?items`,
  `POST /api/shikhar/rebuild`, `GET /api/karma/{member}`,
  `GET /api/votes/flags`, `POST /api/votes/flags/{id}/decide` (exclude
  or dismiss). The add-on's batch endpoint receives scores and karma.
- Events: `vote.changed`, `score.updated`, `karma.updated`,
  `vote.flagged`, `vote.exclusion_changed`.
- Commands: `sutra shikhar rebuild|explain <item>`,
  `sutra karma show <member>`.

### Failures

- SUT-680: a score batch is not accepted by the add-on. Retried with
  backoff; stored scores keep serving.
- SUT-681: a rebuild is running. Incremental updates queue behind it.
- SUT-682: a vote is refused by the checks. The reader is told.
- SUT-683: karma recompute more than an hour behind.

### Security and privacy

1. Votes and karma use only public content.
2. Guest tokens hold no identity. Address hashes are kept in memory for
   at most an hour.
3. Flags are visible only to moderators with the permission.
4. Erasing a member removes their votes and recomputes affected scores
   and karma.

### Edge cases to cover

Reactions removed or changed; brigading with negative reactions; sock
puppets reacting to each other; very old threads gaining votes; equal
scores; items with millions of votes; a member's own votes on their
posts (excluded); merged or split threads moving votes; deleted members
whose votes remain (kept, anonymous); clock changes.

### Facts to check when built

The Hot, Best and Controversial formulas against Reddit's archived
public source, with its licence and date; XenForo 2.3's reaction score
fields and events.

## Limits

- Scoring runs in the worker on streamed events, at most 64 MiB above
  idle, inside the service slice. A full rebuild streams from
  PostgreSQL in pages.
- A reader's vote shows in the counts at once, and in sort order within
  5 seconds, for 95% of votes.
- A full rebuild over 10 million reactions finishes within 10 minutes.
- One score update takes at most 50 microseconds.
- Guest votes: at most 30 per token a day and 60 per address hash an
  hour.

## Footprint budget

- Sorting a list adds one indexed join and at most 5 ms to XenForo's page
  time, measured on the test forum.
- The add-on adds no measurable memory per request, well under its
  8 MB per request limit.
- The usefulness buttons and karma display add a small block of markup;
  their byte budget is set when the feature is built.
- No cookie is set for guests who only read.

## How it is tested

Rust tests in `shikhar` and `karma`, PHP tests in the add-on, and one
headless-browser test:

- Best gives 0.7176 for 9 up 1 down and 0.4880 for 50 up 40 down, and
  orders the first higher.
- Property test: for any counts Best is between 0 and 1 and rises with
  an extra up vote.
- Hot: equal scores 12.5 hours apart differ by exactly 1, newer higher.
- Controversial: 10 up 0 down gives 0; 50 and 50 gives 100.
- Top: only votes inside the week count.
- Rising: an item 25 hours old is not in Rising.
- Mapping: Sad set to neutral in XenForo counts as down, and learning
  receives "Sad".
- Property test: for any event sequence, incremental scores equal a
  full rebuild.
- A thread opened with no choice shows replies in date order.
- Votes on a private node's thread change no score or karma.
- A guest vote passing all checks counts the same as a member's.
- A 31st vote from one token in a day is refused with SUT-682 and the
  guest is told.
- A guest vote from a visit labelled `suspected_automation` is refused
  with SUT-682 and the reason recorded.
- Co-voting accounts raise a flag and no vote or reaction changes.
- A moderator excluding then restoring changes scores and changes them
  back, logged.
- A post with reactions deleted as spam drops its author's karma at
  once.
- An administrator changing Like's score triggers a karma recompute.
- Reactions in two forums: overall tallies equal the sum of the
  per-forum tallies, and overall karma is the Bayesian average with the
  overall prior.
- With job 106's prior 0.6 and strength 20, 18 up and 2 down gives
  karma 75; with no active result the plain tally shows "provisional".
- Usefulness votes on a member's threads, with no reactions, leave
  their karma unchanged.
- Contract: a job 121 decay that is negative or NaN is refused and the
  forum keeps its active decay; a valid one is used only after its
  shadow test, and one action rolls it back.
- Gate: the link scorer's feature registry refuses Hot, Top,
  Controversial, Rising, vote totals and karma totals, and allows only
  Best's confidence score and the usefulness share.
- Failure injection: with Sutradhar stopped, the site sorts by stored
  scores and date order still works.
- PHP and Rust display test: after a vote the viewer's vote, counts and
  summary show, and "Not enough votes yet" under 5.
- Headless-browser and PHP cookie test: a guest who only reads gets no
  cookie; pressing Yes sets only the vote-token cookie; with Global
  Privacy Control the vote is accepted and no darshak visit is recorded.
- Footprint test: sorting adds one indexed join and at most 5 ms per
  page.
- Benchmark: one score update within 50 microseconds, under the 5%
  median rule.
- Depth: 100% line and branch coverage. The vote mapping, the
  non-public rule, the flags-never-change rule, karma clawback, the guest
  checks and incremental-equals-rebuild have no surviving mutants and
  have property tests. Kani proves the mapping returns down for Sad and
  Angry whatever the administrator's score, and that integer counters
  never go below zero after any sequence of add and remove events.

## Changes from the source

- The maths jobs run in ganit, the Rust numerical engine.
- Moderator pages are Dioxus app screens built from SDS components.
- Speed follows the 5% median rule; the 20x goal is kept (spec 0007).
- Browser-side tests are Rust tests driving a headless browser.

Source: `docs/specifications/signals/ranking.md` and
`docs/specifications/signals/ranking-2.md` in the archive
`archive-2026-11-08-old-repo`.
