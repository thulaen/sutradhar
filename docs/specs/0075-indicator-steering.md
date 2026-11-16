# 0075: Indicator steering

Traders use indicators to read a noisy price: a smoothed line, a band
around it, a count of losses in a row. Sutradhar has noisy series of its
own, such as accept rates, reject reasons and click rates. This spec
takes ten groups of those indicators and says where each one may act.
An indicator never changes a setting by itself. It acts only through a
door that already exists: a ranking signal (spec 0027), a step of the
tuner (spec 0053), a report (spec 0053's history), a Coach card (spec
0041) or a kavach caution (spec 0072). Groups 1 to 7 can only make the
system steadier or more careful, or reorder links that are already on
the same subject. Group 8 only
writes reports. Group 9 raises advice that an editor may ignore. Group
10 follows spec 0072's own contract.

This is Later work (task #218). Its method candidates would count under
roadmap Phase 7 item 1 once the owner places the task in a phase. The
longer list of 400 ideas it was picked from is in task #218.

Example: moderators reject the thread "Choosing a spare power supply"
as a destination nine times in a row, each time as "Not relevant". The
exhaustion rule (group 3) rests it for 30 days: it moves below every
other on-topic destination, but it still passes the gate and stays
eligible. A moderator who approves it ends the rest at once.

## What it does

### Words used here

- Gate: spec 0027's relevance gate. Only destinations that pass it are
  ordered; nothing here changes which ones pass.
- Series: numbers in time order about one thing, such as one
  destination's accept rate.
- Decision bar: one point of a decision series, closed after a set
  number of decisions rather than after a set time (group 2).
- Daily series: a series with one point per day. Group 3's shown share,
  group 6's site rate, group 9's page series and group 10's author
  series are daily.
- Exponential moving average (EMA): a running average where each new
  point counts for a fixed share `a` and older points fade:
  `e = e + a × (x − e)`, with `a = 2 / (n + 1)` for a length of `n`
  points.
- Efficiency ratio: how straight a series has moved over its last `n`
  steps, which needs its last `n + 1` points. It is the size of the net
  change divided by the sum of the sizes of every step. It is 1 for a
  straight line and near 0 for noise. When nothing moved, it is 1: a
  series that holds still is steady, not noisy.
- Adaptive average: an EMA whose share follows the efficiency ratio
  (group 1).
- Noise width: the mean size of the step between points over the last
  `n` steps, never below its floor `w_min` (a setting per series,
  above 0). It is close to, but not the same as, what traders call the
  average true range.
- Band: a line with a width above and below it, such as the adaptive
  average plus and minus two noise widths.
- Relative strength index (RSI): over the last `n` steps, the upward
  movement as a share of all movement, on a scale of 0 to 100. When
  nothing moved, it is 50.
- Label: a word an indicator gives a thing, such as "resting". "Normal"
  means it has no label.
- Warm-up: the points an indicator needs before it may act (group 2).
- Door: the existing place where an indicator's output may act (table
  below).

### Doors and what each may do

| Door | Owner | What an indicator may do there |
|---|---|---|
| Trust door of the order stage | spec 0027 | Lower a destination's place among destinations that passed the gate. Never raise one. |
| Tuner, steps 2, 5 and 6 | spec 0053 | Skip a night, drop a proposal, break a tie, make a step smaller, hold a promotion, or lengthen a watch by at most 28 days. Never a step above 5%, never outside its bound, never past the pass rule. |
| Reports | spec 0053's tuning history; spec 0056's lab | Write figures and states. Nothing that a report holds changes what moderators see. |
| Coach cards | spec 0041 | Raise and order a card, and word the answer to "did the change help". |
| Kavach cautions | spec 0072 | Add link-family evidence to `SpamSignal`, under spec 0072's rules. |

These rules hold in groups 1 to 9:

1. An indicator's output is a rate, a share, a fact or a label (spec
   0027, signal rule 2), and it never reads a popularity count (signal
   rule 4).
2. It never feeds the scoring model, the calibrated probability or the
   gate. RS00 holds for every indicator that is a signal.
3. A window length, a threshold or a margin is a setting with its bound
   in the settings store (spec 0014). Changing one never makes a new
   method (spec 0055, identity rule 4). Every bar size and window
   length is at least 2; every margin, floor and step factor is above
   0. RSI and EMA lengths default to 14 points where this spec gives
   none. The day counts (rests, watch, brake) and the brake's
   percentage points are settings too, each with an owner-set bound;
   the watch extension's bound is never above 28 days.
4. No setting of this spec is `tunable` in version 1. The tuner never
   tunes the indicators that guard it.
5. No indicator reads `SpamSignal`, the spam risk column or anything
   computed from them (spec 0072, test SG04). The tuner's replay of
   stored scores and job 45's weight training, which SG04 admits, stay
   admitted when groups 5 to 8 use them.
6. Every action an indicator triggers is written to the audit log
   (spec 0032) with the indicator's id, the point it acted on (the
   series' own internal id and the bar number or date) and its numbers
   only. The entry never holds post text, anchor text, names, member or
   moderator ids, or account details. A series' internal id is not a
   member id and cannot be turned into one outside its own site's
   tables, which erasure clears. An internal id is never reused after
   its series is dropped or erased.
7. Before warm-up ends, an indicator says "not enough data" and acts on
   nothing. The exhaustion rest (group 3 rule 1) and group 10's fixed
   threshold are the only exceptions, as those groups say.
8. An input that is not a number or is infinite is refused and written
   to the audit log, and the series keeps its last good value. After 3
   refusals in a row, the series says "not enough data" until a good
   point arrives.
9. All timers, bar closes and rest ends use the service's own clock,
   never a time sent by a site.

Group 10 is kavach's. Rules 3, 4, 6, 8 and 9 also hold for it, and
rule 7 holds except for the fixed threshold of group 10 rule 1. Spec
0072's contract takes the place of rules 1, 2 and 5: like every
`SpamSignal` input, its evidence reaches the spam risk method.

### Who owns what

- The tuning module of spec 0053 owns every series, bar and label of
  groups 1 to 8, in its own per-site PostgreSQL tables under row-level
  security (AGENTS.md section 5.2). Nothing is shared between sites;
  the memory cache keys every entry by site and never returns one
  site's entry for another.
- The nightly work of groups 3, 5, 6 and 8 runs as steps of spec 0053's
  nightly job sequence, in spec 0024's heavy job slot, at the lowest
  priority and yielding to live work.
- Ranking reads rest labels through that module's public API, as a
  trust signal with its own RS row in spec 0027 once approved.
- Shown counts come from sujhav's public API (spec 0028); decisions and
  reasons come from spec 0053's labels. Post authors and post removals
  come from sampark's public API and post events (spec 0021), never
  from sampark's store.
- The Coach (spec 0041) owns group 9's page state. It reads spec
  0040's stored rows and keeps no copy of them.
- Kavach owns group 10.
- The indicator maths is Rust in the service, because it runs on the
  live path (spec 0055, identity rule 3). It moves into ganit (spec
  0066) only if a ganit job needs it.

### Group 1: one adaptive average

Ideas 80, 81 and 308.

- Every decision series and daily series this spec reads is smoothed by
  one adaptive average. For each new point, take the efficiency ratio
  `r` over the last `n` steps (default 10). Set
  `s = (r × (f − w) + w)²`, with the fast share `f = 2/3` and the slow
  share `w = 2/31`. Then `m = m + s × (x − m)`.
- So the line follows quickly when the series moves in a straight line
  and slowly when it jumps about.
- The first value of `m` is the mean of the warm-up points.
- Each rate signal of spec 0027 that a later spec moves to this average
  keeps its door and its RS test. This spec moves none by itself.
- Numbers are 64-bit floats in the running sums; stored window points
  may be 32-bit.

### Group 2: decision bars and warm-up

Ideas 192 and 325.

- A decision bar closes after `N` decisions on its series. Its value is
  accepts divided by decisions in the bar. Defaults for `N`: 5 for one
  destination, 20 for a section, 50 for a site. `N` is
  at least 2.
- A decision repeated for the same suggestion counts once.
- A bar also closes after 90 days with whatever it holds, if it holds
  at least 2 decisions. With fewer, it waits. The 90-day close is
  checked when the series is next read and in one nightly sweep that
  stops after 10,000 series and carries on the next night.
- An indicator that looks back `n` steps needs `n + 1` closed points
  before it may act. The adaptive average needs 11.
- Daily series (groups 3, 6 and 9) use days. A day with no data is a
  missing day, not a zero. Group 10 is the one exception: a day with no
  new links is a real 0.

### Group 3: rest rules

Ideas 17 (rule 1) and 4 (rule 2). Idea 41 (an over-used anchor phrase)
is left out: any swap of anchor changes spec 0027's relevance inputs
and spec 0023's query window, and spec 0027's link rule 2 already caps
how often one anchor text is used.

Both rules only lower, through the trust door. They have their own
warm-up and ending, below; group 4 does not apply to rule 1.

An author here is the post's user id in sampark's post facts (spec
0021), read through sampark's public API, never the acting id of a
save. Guests (user id 0) count together as one author; a deleted
member keeps their user id. Authors are read only to count them, and
no member id is stored.

1. Exhaustion rest. The run of a destination is a ring of its last
   `rest_after` counted rejects on one site (default 9, at least 3 and
   at most 20). Rejects with the reasons `not_relevant`, `weak_target`
   and `better_target`, or with no reason, count. Rejects with
   `wrong_anchor`, `too_many_links`, `already_covered` or `other`
   neither count nor break the run, since spec 0053 does not read them
   as about the destination. A moderator's remove (undo) of a link to
   it counts as a reject. An approval, and the end of a rest, empty the
   ring. This rule needs no warm-up beyond its own ring.
   - At most one reject per source post counts within the ring.
   - When the ring is full, the destination rests for 30 days only if
     its rejects come from at least 3 different source threads and from
     posts by at least 3 different authors. If not, nothing happens,
     and each new reject pushes out the oldest.
   - The ring keeps only source post ids and thread ids, at most 20
     pairs, in its own table keyed by destination with an index on post
     id. Post authors are read when the ring is full.
   - A reject on a post that moderators later remove, or that is
     erased, drops out of the ring, and so does a reject that spec 0053
     withdraws (a rejection restored from the trash). None of these
     ends a rest already started.
2. Over-shown rest. Each night, for destinations shown in the last 7
   days and for every destination resting under this rule, a
   destination's shown share is its shows in its section over those
   days divided by the section's shows over the same days. Each source
   post counts at most once a day toward a destination's shows.
   - A rest can start only when shows come from posts by at least 3
     different authors, and only when the share is still above the
     band with the shows of the author with the most shows left out.
     Decisions on posts by that author are left out of the RSI too. So
     one account's flood cannot start a rest.
   - A resting destination's rest days are missing points in its own
     share series, so they do not pull its band down. A resting
     destination that is no longer shown is not checked against its
     band; the 30-day cap ends its rest.
   - A section with no shows gives a missing day.
   - When the share is above its band (the adaptive average plus two
     noise widths) and the RSI of its accept rate's decision bars is
     below 50, and the author checks above pass, it rests. Decisions
     count once per source post toward that RSI.
   - It ends when the share is back inside the band under group 4's
     rules, and after 30 days at most. After a rest it may rest again
     only once a new accept bar holding at least one accept has closed.

- A resting destination drops below every destination that passed the
  gate and is not resting. Resting destinations keep their order among
  themselves. Nothing that passed the gate is refused: a resting
  destination stays eligible, though caps that keep only the top few
  (spec 0027's link cap, tuned by job 47, and spec 0023's limits per
  anchor phrase) may leave it
  out, as they would any destination in that place.
- A moderator's approval of a resting destination ends any rest at
  once, whatever group 4 says.
- A rest's end date is checked whenever the label is read, so a
  nightly job that has not finished can never hold a rest past it.
- The explanation says "Resting: rejected 9 times in a row" (the number
  read from `rest_after`) or "Resting: shown more often than usual". It
  never names a moderator.

### Group 4: rules against flapping

Ideas 104 and 106. They hold for the over-shown rest (group 3 rule 2),
the section states (group 8) and the refresh cards (group 9).

- Three windows. A label changes only when the adaptive averages over
  short, medium and long windows (defaults 5, 10 and 20 points) all
  have a net change of the same sign since the last label change, or,
  before the first change, since warm-up ended.
- Margin. A label for a rise changes only when the value is above its
  line (the adaptive average itself, not a band edge) by more than `h`
  noise widths (default `h = 1`). A label for a fall changes only when
  the value is below it by the same margin. The margin is checked
  after the point updates the line, and it must hold for 2 points in a
  row.
- A label that changed may not change again for 3 points.
- Module promotion keeps spec 0053's rules and job 228.

### Group 5: steadier tuner targets

Ideas 224 to 230. Changes spec 0053, step 2 ("Propose").

The tuner keeps its pass rule, bounds, one-change watch and 5% limit.
Rules 1 to 3 apply only to the scoring-model weights that job 45
trains, since only they have a nightly trained value. The weight on the
spam risk column is left out of rules 1 to 3 and 5 and keeps spec
0053's step 2 as written, so no guard is built from it (spec 0072,
SG04). Retrieval settings (job 115), hyperparameters (job 120) and
thread-vector weights (job 116) keep spec 0053's step 2 as written,
apart from rules 4 and 5. Tunable weights are always above 0 (spec
0053). "The target" below is the latest trained value.

1. Target. Each night's trained value is kept as a series. A step is
   proposed only if it points toward both the latest trained value and
   the EMA of the last 7. If they lie on opposite sides of the current
   value, the weight gets no proposal that night.
2. Direction. The short EMA (3 nights) of the trained value minus the
   long EMA (7 nights) must not have the opposite sign to the step. A
   flat trend passes.
3. Out of band. Rules 1 and 2 apply only while the target is outside
   the current value plus or minus two noise widths of the trained
   series. Inside that band, the weight keeps stepping toward the
   target, each step the smaller of rule 5's step and the distance
   left, until it equals the target. Then it has reached it.
4. Calm night. Each night the tuner replays the check set under the
   current settings and keeps that agreement as a series (spec 0053
   gives about 2 seconds per 100,000 pairs). When its noise width over
   the last 10 nights is larger than the best proposal's expected gain,
   nothing is proposed, and the history says "skipped: results too
   noisy".
5. Step size. Every step, inside the band or not, is still a proposal
   that must pass spec 0053's pass rule. A continuous step is
   `5% × min(1, q / noise)`, where `noise` is the noise width of the
   setting's series (its trained series, or for other settings its last
   10 values), and
   `q = max(q_min, 1% of the current value)`, with `q_min` a setting
   above 0 and at most 1% of the setting's smallest allowed value (or
   of its default, when its bound starts at 0). The noise floor
   `w_min` of these series defaults to half of `q_min`, so a quiet
   series takes the full 5% and a noisy one a smaller step. The target
   is first clamped into the setting's bound. A step never goes past
   the target. A discrete setting still moves one approved value.

### Group 6: brakes on autotuning

Ideas 243, 265 and 266. Changes spec 0053, steps 5 and 6.

1. Brake. When a site has at least 200 decisions in the last 7 days
   and at least 200 in the 28 days before them, and its accept rate
   over the 7 days is more than 15 percentage points below its pooled
   rate over the 28 days (all accepts divided by all decisions), no
   step is promoted on that site.
   - A step already under watch has its watch lengthened while the
     brake is on, by at most 28 days in total for that step, however
     often the brake goes on and off. Jobs 99 and 157 keep watching all
     through, and job 99 reverts the step at once if spec 0053 says so.
     When the 28 days are used up, the watch ends as spec 0053 says and
     the owner gets an alert.
   - When the brake goes on, the 28-day pooled rate is frozen as its
     reference. The brake lifts when the 7-day rate is no more than 5
     percentage points below that frozen reference.
   - A brake lasts 28 days at most. Then it lifts, a fresh reference is
     taken from the 28 days just ended, and the owner gets an alert.
   - With fewer decisions than these, the brake does not start. A brake
     that is on stays on until it lifts or reaches its 28-day cap.
     Decisions on posts moderators removed are left out of its counts.
2. Only above its own line. A step is promoted only when the latest
   agreement of group 5 rule 4's nightly series is at or above its
   adaptive average.

- Neither rule ever shortens a watch, skips a check or makes a change.
  The tuning history says which one held a step.

### Group 7: reject reasons break ties

Ideas 268 to 271. Changes spec 0053, step 2.

- Each tunable key may be linked to the reasons it answers. When a
  reason's nightly share of decisions is rising (its EMA over 7 nights
  above its EMA over 28 nights) and two proposals' expected gains
  overlap within their 95%
  ranges, the one whose key answers that reason is taken.
- With no overlap, the larger expected gain is taken, as spec 0053
  says. The search still covers every tunable setting.
- In version 1 only `better_target` has keys to point at: the
  scoring-model weights (job 45) and thread-vector weights (job 116).
  `wrong_anchor`, `too_many_links` and `already_covered` point at
  nothing until spec 0023 or spec 0027 marks a key `tunable` that
  answers them. A reason that points at nothing changes nothing.

### Group 8: section states and the baseline, as reports

Ideas 291 and 331. Reports only; no module mode or chain changes.

- Each segment of spec 0053 gets one of three states each night from
  its accept rate's decision bars: "steady trend" (efficiency ratio
  above 0.4 and the series moved), "back and forth" (below 0.2) or
  "unclear" (between, or in warm-up). The state moves under group 4's
  rules.
- The state is written to the tuning history and offered to the lab
  (spec 0056) as a way to split its comparisons. Nothing else reads it.
- Each night, as the step after the tuner in spec 0053's job sequence,
  the check set is replayed from stored feature scores under the plain
  baseline of the release bar (AGENTS.md section 3 item 6) and under
  the live settings. The history records both figures side by side.
  It runs in spec 0024's heavy job slot at the lowest priority, within
  10 minutes and the tuner's 256 MB. A night it cannot run is marked
  "put off", never "failed".

### Group 9: Coach refresh cards and "did it help"

Ideas 351 and 354. Adds to spec 0041's "What needs a refresh" and "Did
the change help" tasks.

- Series per page: click-through rate from the search connectors (spec
  0040), by day. Average position is shown beside it but decides
  nothing; a lower position is better.
- Each day is divided by its weekday's mean over the 8 weeks before
  that day, to take out the weekday pattern. A day is missing when that
  mean is 0, when fewer than 4 of those 8 weeks have data, or when the
  day has fewer than 20 impressions.
- Refresh card. When the click-through rate stays below its band (the
  adaptive average minus two noise widths) under group 4's rules, a
  refresh card is raised. Cards are ordered by how far below the band
  they are, in noise widths.
- Did it help. On day 31 after an edit and later, the Coach compares
  days 4 to 31 after the edit with the 28 days before it. It says:
  - "Helped" when the after mean is above the before mean by more than
    two noise widths;
  - "Hurt" when it is below by more than two noise widths;
  - "Normal ups and downs" otherwise;
  - "Too early to say" before day 31, or when fewer than 14 of the 28
    days on either side have data.
- The card shows the line, its band and the edit date, in plain
  English. It is advice only, and the editor can keep the original.
  Search traffic can be pushed by outsiders, so the card says the
  answer comes from search figures.
- A nightly job in spec 0024's heavy job slot, at the lowest priority,
  reads only the last 90 days of spec 0040's rows, ordered by page, one
  page at a time, within 256 MB and 30 minutes per night; a site it
  does not finish carries on the next night. Pages with an edit in the
  last 31 days go first. On the first night the job reaches a page
  after an edit, it stores the before-edit mean and noise width, so
  the "did it help" answer never needs older rows; if those rows are
  already outside the 90 days, the answer is "Too early to say". It
  keeps at most 256 bytes per page (one series state and those stored
  figures) in the Coach's own table, not in memory.

### Group 10: two link patterns over time

Ideas 154 and 157. Kavach owns them, as link-family evidence in
`SpamSignal` (spec 0072). Both count against the posts that carry the
links, never against the destination, and only the author's own saves
under their own non-zero author id count, as spec 0072's same-author
rule says.

1. Link burst. For each author, kavach keeps one daily series: the
   day's largest number of new links their own posts added to any one
   destination. A day with at least 5 such links that is above that
   author's own band (adaptive average plus three noise widths) adds
   evidence to those posts. A fixed site-wide threshold (default 10
   links to one destination in a day) also adds evidence at any time,
   before or after warm-up, so a slow steady rise cannot hide. A day
   with no new links counts as 0; those zeros are filled in when the
   author's next link arrives or the series is read, not by a nightly
   write. The destination's `SpamSignal` is unchanged.
2. Link churn. A post whose author's own saves add and then remove the
   same link three or more times within 14 days adds evidence to that
   post. Kavach compares only the link sets of the last 20 revisions it
   already keeps, however many there are.

- Kavach keeps at most 100,000 author series per site, at 192 bytes
  each (about 19 MB), in its own per-site tables on disk, and drops an
  author's series after 30 days with no new links. At the cap no new
  series starts and no kept one is pushed out; an author without a
  series is judged by the fixed threshold only. A member's erasure
  removes their series.
- Quotes and Sutradhar's own inserted links are left out, as spec 0072
  says. No member sanction follows.
- Ideas 153, 155, 156 and 158 are left out: 156 (link rings) and 158
  (copied titles) repeat spec 0072's reciprocity and duplicate
  evidence; 155 (traffic bursts) belongs to darshak; 153 would read the
  ranking score, which kavach must not use.

### Owner decisions

1. Family. Each entry goes in the family of the part it acts on, not a
   family of its own (owner, 16 November 2026: "Spread them out"). The
   table below names each one's family. The exact family ids are set
   when the catalogue of spec 0055 is built.
2. Groups 5, 6 and 7 change spec 0053's approved behaviour. The owner
   approved all three on 16 November 2026: "Approve all three". Spec
   0053's tuner section points here. Groups 1 to 4 and 8 to 10 add
   behaviour to specs 0027, 0041, 0053 and 0072 and need the
   owner's approval (AGENTS.md section 12).
3. Profiles by state (idea 282) are not in this spec. They would need
   presets the owner approves for each state.

### Catalogue entries

About 15 method candidates come from this spec. Each needs its own
sheet, a distinguishing test and lab admission before it counts:

| Candidate | Group | Family: the part it acts on |
|---|---|---|
| Adaptive average | 1 | Ranking order (spec 0027) |
| Decision bars | 2 | Feedback learning (spec 0053) |
| Exhaustion rest | 3 | Ranking order, trust door (spec 0027) |
| Over-shown rest | 3 | Ranking order, trust door (spec 0027) |
| Three-window agreement | 4 | Ranking order (spec 0027) |
| Margin by noise width | 4 | Ranking order (spec 0027) |
| Two-target check | 5 | Tuning (spec 0053) |
| Direction check | 5 | Tuning (spec 0053) |
| Calm-night check | 5 | Tuning (spec 0053) |
| Step size by noise | 5 | Tuning (spec 0053) |
| Brake | 6 | Tuning (spec 0053) |
| Promote only above its own line | 6 | Tuning (spec 0053) |
| Reason tie-break | 7 | Tuning (spec 0053) |
| Section state | 8 | Feedback learning (spec 0053) |
| Weekday-adjusted band for pages | 9 | Editorial Coach (spec 0041) |

An entry used by more than one part, such as the adaptive average or
the rules against flapping, is still one entry in one family (spec
0055, identity rule 2). The other parts use it by its id. Kavach's two
patterns are kavach detectors with their own entries under spec 0072.
Different lengths or margins of one candidate are settings, never more
entries.

## Limits

- State per series: a ring of the last 21 points as 32-bit floats (84
  bytes), running sums and averages as 8 64-bit floats (64 bytes), and
  counters, label and timers (at most 40 bytes): at most 192 bytes.
- Series kept: per destination, its accept rate and its shown share;
  per section and per site, a handful each. A destination with an open
  exhaustion ring also has at most 20 id pairs (320 bytes) in the ring
  table.
- All series live in the tuning module's tables. A cache in memory
  holds the most recently used ones: at most 24 MB on a PC under 16 GB
  and 64 MB at 16 GB or more, across all sites, taken from the "every
  other part" row of spec 0024's memory table. When it is full, the
  least recently used series leaves the cache.
- Ranking reads one post's labels in one batch read with a 5 ms limit.
  When the limit is hit, that request uses no labels, so nothing is
  lowered.
- Each new decision updates its series in constant time from the
  running sums. Shown shares (group 3) are computed in the
  nightly job, not on the live path. That job streams the section
  totals from sujhav's public API, updates only destinations shown in
  the last 7 days and those resting under group 3 rule 2, and runs in
  spec 0024's heavy job slot within 10 minutes and 256 MB a night; a
  site it does not finish carries on the next night.
- Groups 5 to 7 add at most 1% to the nightly tuner run. The bench
  measures this on Dell (spec 0059).
- Group 8's nightly replay: 10 minutes and the tuner's 256 MB at most.
- Group 9: at most 256 bytes per page on disk; 90 days of rows read;
  256 MB and 30 minutes a night.
- On disk, the series tables take about 400 MB per site with 1,000,000
  destinations, before row overhead, plus at most 320 MB for rings if
  every destination had a full one.

## Footprint budget

None. Nothing here runs on public pages or changes what the add-ons
send.

## How it is tested

| Test | What it proves |
|---|---|
| IS01 Never raises (property; Kani proof of the trust door) | Whatever values the indicators take, the set that passes the gate is unchanged (with RS00); a resting destination stays eligible; destinations that do not rest keep their order among themselves, and so do those that rest. |
| IS02 Adaptive average | After a step from 0 to 1 it is above 0.9 within 10 points; on alternating noise it moves less than an EMA of length 10; on a flat series it stays put. |
| IS03 Decision bars and bad input | A series with 3 decisions in 90 days makes one bar; with 1 it makes none. A repeated decision counts once. Warm-up gives "not enough data" and no action. Three non-numbers in a row give "not enough data", each one logged. |
| IS04 Exhaustion rest | A full ring of 9 rejects as "Not relevant", from 3 threads and posts by 3 authors, rests the destination; 8 do not; 9 as "Enough links here" or "Wrong words linked" neither rest it nor break a run. An approval empties the ring. After a rest ends, one more reject does not rest it again. One account replying 9 times across 3 threads does not rest it; nine guest posts count as one author; a moderator's edit of a member's post does not change its author; a reject withdrawn by spec 0053 (restored from the trash) drops out; two rejects on one post count once; 1,000 rejects from one thread keep the ring at 9 and never rest it; a reject on a post moderators later remove drops out. |
| IS05 Over-shown | Shown above its band with falling accepts, from posts by 3 authors: rests. Above its band with rising accepts: does not. Shows from posts by one or two authors cannot start a rest; a post counts once a day. One account flooding posts, beside a few shows from 2 other members, cannot start a rest, since the share is not above the band with that account's shows left out. A flat accept series (RSI 50) does not rest it. A section with no shows gives a missing day and no audit entry. A resting destination that is no longer shown has a share of 0 and leaves its rest within 30 days; its rest days do not lower its band, and it does not rest again until a new bar with an accept closes. |
| IS06 Anchors untouched | No indicator of this spec changes spec 0023's anchor choice: with every indicator at any value, the anchor chosen among spec 0023's candidates is the same. |
| IS07 No flapping | A series that crosses its line and back each point, from the first point after warm-up, never changes its label, since the margin must hold for 2 points in a row. A flat series with noise width at its floor changes no label. |
| IS08 Tuner checks | Every proposal taken was found by spec 0053's own search, inside the same bound and step rule, and no step is above 5% or goes past the target. From a weight of 1.00 and a constant target of 1.30, the weight reaches exactly 1.30 through steps of at most 5% (as spec 0053 test 19), including when `q_min` is at its upper bound. A target outside the bound is clamped first. A noise width at its floor gives the full step. The spam risk column's weight is moved only by spec 0053's step 2 as written. |
| IS09 Brake | A 16-point fall with 200 decisions on each side stops promotion and lengthens a watch by at most 28 days in total, even when the brake goes on and off, then alerts. Job 99 can still revert during the longer watch. The brake lifts when the rate is no more than 5 points below its frozen reference, not by the reference sliding down, and also when the rate recovers above the reference. A lasting drop lifts the brake after 28 days with a fresh reference and an alert. With 150 decisions in either window it does not apply. No step is reverted by the brake. |
| IS10 Reason tie-break | With overlapping gains and a rising `better_target` share, a job 45 weight wins; with clearly unequal gains, the larger wins. A reason with no linked key changes nothing. |
| IS11 Section state | A state change writes to the history and changes no module mode and nothing staff see. |
| IS12 Baseline report | Each night has both figures, or is marked "put off". |
| IS13 Did it help | Fixed series give each of the four answers. A Monday dip seen every week is not "Hurt". A weekday mean of 0 makes that day missing. A day under 20 impressions is missing. On day 31 after an edit, the answer uses the stored before-edit figures and reads no row older than 90 days. Before day 31 the answer is "Too early to say". |
| IS14 Link patterns | An author's own burst adds evidence to their posts; the destination's `SpamSignal` is unchanged. Links by many other authors to one page add nothing to it. A new author with 10 links to one page in a day adds evidence, and so does a warmed-up author whose count rose by 3 a day until it reached 10. At the cap of 100,000 series, no kept series is pushed out. Add-remove three times by the author adds evidence; by a moderator, nothing. With 30 revisions, the last 20 are still compared. An erased member's series is gone. |
| IS15 No spam input | The gate check refuses any indicator of groups 1 to 9 that declares `SpamSignal`, the spam risk column or a value computed from them, other than SG04's admitted tuner replay and job 45 training (with SG04). |
| IS16 Not tunable | A registry test over this spec's setting keys finds none marked `tunable`. |
| IS17 Audit privacy | Every audit entry from this spec holds only an indicator id, a point and numbers. |
| IS18 Memory | With the cache full, a new series pushes out the least recently used one, and memory stays within the stated cap. |

## Changes from the source

Written fresh from the list of 400 ideas in task #218. Changed from that
list:

- Idea 4: a high RSI of accepts alone is not a reason to rest a
  destination, so the over-shown rest needs both a high share of shows
  and falling accepts.
- Idea 154: the burst counts per author, against their own posts,
  never against the destination.
- Idea 41: left out, as group 3 says.
- Idea 157: only links, and only the author's own saves.
- Idea 230: the step shrinks when the series is noisy, not when it is
  quiet.
- Ideas 291 and 331: reports only, so no module mode changes.
