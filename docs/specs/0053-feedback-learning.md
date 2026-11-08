# 0053: Feedback learning and bounded autotuning

This part learns from what moderators approve, reject and remove, and uses
it to tune Sutradhar's settings in small, checked steps. It belongs to the
feature waves of roadmap Phase 6. The Rust module is `sutradhar-tuning`
(an English placeholder name); the maths lives in ganit.

Each time a moderator approves, rejects or removes a link, Sutradhar learns
a little about what this site's staff find useful. It may nudge one weight
by at most 5% at a time, only inside limits the owner approved, only after
checking in the background that the change would have helped, and it
writes down why, what it gains and what it costs. If the change later turns
out worse, Sutradhar puts back that one change only, explains why, and
leaves everything else alone. It never switches a method off and never
turns on automatic linking.

Example: moderators keep choosing newer threads. The nightly run tries
raising the freshness weight from 1.00 to 1.05. On held-back recent
decisions, the moderators' chosen thread now comes first 1.4 points more
often (range 0.6 to 2.2), and nothing else gets worse beyond tolerance. A
new settings version is written with the note "freshness weight 1.00 to
1.05: gains 1.4 points of agreement with moderators' choices; costs 0.3%
fewer suggestions in old-hardware sections". Two weeks later accept rates
are steady, so it stays.

## What it does

### Words used here

- Decision: approve, reject or remove (undo), with any reason buttons
  (spec 0028).
- Label: what a decision teaches: a positive or negative pair, a
  preference of one destination over another, or nothing.
- Tunable setting: a setting the settings store marks `tunable`, with its
  bound and step rule. Only these may move automatically.
- Step: one change to one tunable setting, or to one declared linked
  group.
- Background comparison: replaying recent decisions and link
  reconstruction under the current and proposed settings, without changing
  what moderators see.
- Watch: the period after a step during which live figures are checked.
- Module mode: shadow (suggestions made, never shown) or review (shown to
  staff). There is no automatic mode in version 1.
- Segment: a part of a site judged on its own, such as one section.

### Reason buttons and labels

Reasons are optional buttons, never free text.

| Reason | Shown as | Label |
| --- | --- | --- |
| `not_relevant` | Not relevant | negative pair |
| `wrong_anchor` | Wrong words linked | negative anchor only |
| `better_target` | A better thread exists | preference: chosen wins |
| `weak_target` | Target not good enough | negative pair |
| `too_many_links` | Enough links here | none |
| `already_covered` | Already covered | none |
| `other` | Other | none |

- Approve: a positive pair; the chosen destination is also preferred over
  the alternatives shown above it.
- Reject with no reason: a weak negative, at half weight.
- Remove (undo) by a moderator: a negative pair, stronger than a
  rejection, because the link was seen in place.
- Recalculated, parked, retired or stale suggestions make no label. A
  rejection restored from the trash bin is withdrawn, and so is its label.
  The 90-day purge leaves only id-only labels of decisions that stand.
- How long a placed link survives later edits (ganit job 310) is also a
  label: a link removed soon is weak evidence against it.
- One label table serves this tuner and ganit's training of the small
  networks.
- There is no author opt-out. Analytics consent is a separate matter.
- Decisions by an account without the review permission never reach
  labels, because spec 0028 refuses them.

### Who does what

- ganit proposes setting changes, trains the scoring model, estimates
  accept rates and rollback risk, judges comparisons, cleans labels and
  watches for sudden shifts.
- The Rust tuning module keeps the labels, enforces every bound and step
  through the settings store, replays rankings, shadow-tests and activates
  each ganit output, writes the history and reverts.
- ganit never writes Sutradhar's tables and never sees post text. A
  durable job starts every ganit run.

### The tuner, one careful step at a time

A nightly durable job sequence at the lowest priority, yielding to live
work:

1. Freeze. Note the active settings version, the labels up to a cut-off
   and the latest reconstruction run. Split them by thread and by time;
   the newest 20% of decisions are the check set. Export them with ids and
   numbers only. Label weights come from ganit jobs 154 (moderators who
   accept everything), 153 (noise in the reconstruction set) and 146
   (moderator position bias).
2. Propose. ganit searches each tunable setting only within one step of
   its current value (job 115 for retrieval settings, 120 for scoring-model
   hyperparameters, 116 for thread-vector weights): at most 5% either way
   for a continuous weight, the next approved value for a discrete one,
   inside its bound, never zero or below. The tuner passes the bounds and
   step rules from the settings store and checks the answer against them
   again. The single change, or declared linked group, with the largest
   expected gain is taken. One change, one reason.
3. Compare in the background. Replay ranking from the stored feature
   scores, so no model runs, giving current and proposed scores for every
   pair in the check set. The frozen rule:
   - agreement with moderators' choices must improve, with the lower end
     of its paired 95% range above zero (jobs 128 and 129, with job 300's
     variance reduction);
   - link reconstruction and anchor agreement may not fall beyond the
     matching tolerance (job 132, "no worse than");
   - when many settings were tried, job 130 controls false discoveries;
   - job 147 scores the new policy offline from logged decisions;
   - hard rules are untouched, since they are not tunable.
4. Record. A proposal record holds the reason, the benefits and drawbacks
   (measured, with ranges, by segment), the before-and-after figures, the
   exact values, and the versions of every ganit output used.
5. Promote. If it passes and autotuning is on, write a new settings
   version through the settings store as actor `tuner`. Parts take it
   between jobs; each job keeps the version it started with. A setting
   that needs a restart is only proposed to an administrator.
6. Watch. For 14 days or 200 decisions, whichever is later, job 99
   estimates the chance the change made things worse, and job 157 watches
   the accept rate for a sudden shift. When job 99's chance passes the
   rollback level, revert that step only, through the linked-group revert,
   record why, and pause that setting for 30 days.

- While a step is under watch, no other step is promoted anywhere: one
  change under watch at a time across all pipelines.
- With under 200 decisions in the check set, nothing is proposed. Job 229
  can raise that minimum for a site.
- Autotuning is on at install and starts once 200 decisions exist.
- An administrator's own change to a tunable setting pauses autotuning of
  that setting until they turn it back on, so the tuner never fights a
  person.
- Agents may read tuning state. They change settings only when a person
  has asked, through the settings store's instruction reference.
- Each site tunes on its own data only.

### The scoring model and calibration

- ganit trains the scoring model nightly (job 45): logistic fusion now;
  LambdaMART once at least 2,000 decisions exist and its lab experiment
  has passed. Feature importance (job 46) feeds the explanation.
- A newly trained set of weights is a ganit output, shadow-tested like any
  other. It is a target: the tuner moves one weight towards it by at most
  5% per step, each step shadow-tested.
- Calibration (job 33) is refreshed nightly, and redone when job 192 sees
  final scores shift after a change. It changes what confidence means, not
  the order, so it is not a tunable setting.
- The transformer models themselves are not retrained on this hardware.
  Only the small networks (the embedding adapter of job 273 and the
  classifier heads) are trained, from these labels.

### Module modes: shadow and review

- Every module that makes suggestions has promotion settings: minimum
  decisions, minimum accept rate and observation period.
- Job 96 estimates accept rates per module and site and makes the
  promotion decision; job 139 sets the error levels for automatic
  demotion; job 228 replays the policy on history to choose settings that
  stop flapping. The owner's thresholds bound them: ganit's numbers apply
  only inside them.
- Shadow to review: a shadow module has no decisions, so it is judged by
  link reconstruction against the module in review.
- Review to shadow: when the lower end of the 95% range of the accept rate
  over the latest 200 decisions falls below 40%. It returns only above
  50%, after a full observation period.
- No mode exists beyond review.
- Every promotion and demotion is audited and shown on the health page.

### Accept rates, drift and the history page

- Live counts of decisions and accepts are shown; their ranges come from
  the latest job 96 output.
- Drift: job 157's changepoints on accept rate per module and job 196's
  drift in scoring features raise an alert and an automatic issue; job 196
  also says when the scoring model should be retrained. Job 306 separates
  the effect of review-page changes, so a page change is not taken for a
  tuning gain.
- The tuning history page shows every step as a line: what changed, why,
  gains, costs, the figures and what happened under watch, with no post
  text. Nothing is a black box.

### Edge cases

- A model change: labels record their model keys; replay uses only pairs
  scored by the current reranker, and tuning pauses until 200 such
  decisions exist.
- A playbook switch restarts the watch, because the baseline moved.
- A weight that keeps hitting its bound stays there, and the history says
  "at its upper bound". Widening a bound is the owner's decision.
- A method whose best weight seems to be its minimum is reported as a
  question for the lab, never switched off.
- Seasonal swings: the watch compares like with like by segment, with job
  157's changepoints beside it.
- Overfitting from many tries is held by one small step, a held-out check
  set and the watch.

### Data

In Sutradhar's PostgreSQL with RLS forced. The tuning role writes these
tables, the API role reads them, and moderator ids are visible only to the
tuning role as pseudonymous refs. Labels come only from decisions on
guest-visible content and are purged with that content.

- `feedback_label`: `decision_id`, `suggestion_id`, `destination_id`;
  `kind` (positive, negative, weak negative, anchor negative, or
  preference with the preferred destination); `reason`; `weight`;
  `made_at`; `split` (tuning or check, fixed when made); `content_version`
  of both sides, so a later edit can retire it; `model_keys`.
- `tuning_proposal`: `id`, `pipeline`, `keys` (one setting or a linked
  group), `from` and `to`; `reason`, `benefits`, `drawbacks` (plain
  English from the figures); `figures` (JSON: each measure before and after
  with its interval and sample size, by segment); `evidence_digest`
  (SHA-256 of the frozen inputs and the rule); `ganit_outputs`; `state`
  (proposed, failed_check, promoted, watching, kept, reverted, withdrawn);
  `settings_before`, `settings_after`; `made_at`, `decided_at`,
  `watch_until`.
- `module_mode` (module, segment, mode, since, evidence digest) and
  `module_mode_event` for every change.
- `accept_rate`: per segment and day, decisions, accepts and interval. The
  latest-200 window is computed, not stored twice.
- Before-and-after results also go to the settings store's results table
  for their settings version.
- On uninstall, labels, proposals and modes are owned data under the
  export offer and keep-or-delete choice.

### Actions, events and export

| Action | What it does | Capability |
| --- | --- | --- |
| `setu.tuning.status` | autotuning state, watch, last steps | CAP-077 |
| `setu.tuning.enable` | turn autotuning on or off | CAP-077 |
| `setu.tuning.history` | every proposal with its evidence | CAP-077 |
| `setu.tuning.revert` | an administrator reverts one step | CAP-077 |
| `setu.module.mode` | show or set a module's mode | CAP-045 |
| `setu.accept_rate` | accept rates and drift by segment | CAP-047 |

- The same actions are `sutra tuning status|history|enable|disable|revert`
  and `sutra setu module mode`.
- Events carry ids and figures only: `tuning.proposed`, `tuning.promoted`,
  `tuning.reverted`, `tuning.paused`, `module.mode_changed`,
  `accept_rate.drift`.
- The training export to ganit holds labels with ids, content hashes and
  reason codes only: no text, no moderator identities, nothing from content
  no longer visible.

### Failure codes

The tuner's failures never affect suggestions.

- SUT-600: too few decisions for a step; nothing proposed tonight.
- SUT-601: the proposal failed its background check; recorded with its
  figures, nothing changed.
- SUT-602: settings changed during the comparison; withdrawn and retried
  tomorrow.
- SUT-603: live figures worsened under watch; that step was reverted and
  its setting paused for 30 days.
- SUT-604: a ganit output is overdue (job 96, 99, 157 or 196); the last
  good output is used and an automatic issue is filed.
- SUT-605: the settings store refused the tuner's version (bound or step);
  a tuner defect, filed as an automatic issue.
- SUT-606: a module was demoted to shadow; the page says why, with the
  figures.
- SUT-607: the nightly run passed its window; it stops at the next safe
  point and resumes from its checkpoint.
- SUT-608: ganit proposed a value outside its bound or step, or returned a
  guarded value (not a number, infinity); refused before the settings
  store sees it, and an automatic issue is filed.

### Security and privacy

- Labels hold ids, codes and hashes, never post text. Exports to ganit
  hold ids, numbers and word counts only, checked by a contract test.
- Moderators appear only as pseudonymous ids, for fairness weighting.
  Names stay in the separate private part, which can be erased. No page
  ranks individual moderators.
- Erasure deletes labels touching erased content; proposals that used them
  are marked "evidence reduced" and their settings stay.
- Logs, metrics and events carry ids, counts and figures only.

### Observability

- Metrics: proposals by state; steps promoted and reverted; time under
  watch; accept rate by segment with its interval; decisions available;
  tuner run time; module modes by segment.
- Alerts: a revert (information); two reverts of one setting in 90 days
  (an automatic issue: the bound may be wrong); a module demoted; drift.

## Limits

- A continuous weight moves at most 5% of its current value per step; a
  discrete setting moves one approved step; values stay inside their bound
  and above zero.
- One step under watch at a time; watch of 14 days or 200 decisions,
  whichever is later; a reverted setting pauses for 30 days.
- At least 200 decisions in the check set; LambdaMART needs at least 2,000.
- Memory, inside the service slice, within the memory limits in
  AGENTS.md section 5.6: the tuner runs in spec 0024's heavy job slot
  and uses at most 256 MB, streaming labels and stored scores in
  chunks; labels are about 200 bytes each (100,000 labels about 20 MB
  on disk); nothing is held between nights; ganit's nightly jobs run
  one at a time in the same slot, at most 0.8 GB on a PC under 16 GB
  and 1.2 GB otherwise, never beside a backfill peak. Estimates to be
  measured.
- Speed, on Dell under fixed conditions, held by the 5% median rule:
  - the nightly run on a site of 1,000,000 posts with 100,000 labels ends
    within 10 minutes of job time, not counting pauses for live work and
    the ganit runs; the ganit proposal and verdict each end within 5
    minutes;
  - replaying 100,000 pairs from stored scores takes at most 2 s;
  - a revert decided by the watch takes effect, between jobs, within 1
    minute;
  - the tuning history answers within 300 ms at the 95th percentile for
    10,000 proposals; accept-rate figures within 100 ms.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Rust tests in `crates/tuning/tests/`, with synthetic decisions and a fake
ganit that answers from fixed outputs (`LabelSource`, `SignalSource`,
`GanitClient`, `Replayer`, `SettingsWriter`, `LiveFigures`), plus ganit's
own tests where named.

1. Each reason button and decision kind makes the label in the table;
   recalculated, parked, retired or stale suggestions make none; a restored
   rejection withdraws its label (`labels.rs`).
2. For any history, the split puts no thread on both sides, and every
   check label is newer than every tuning label (property).
3. For any value, bound, step rule and ganit answer, a value passes only if
   a continuous weight moves at most 5%, a discrete one by one step, inside
   its bound and above zero; otherwise SUT-608 (property and Kani proof).
4. Three settings able to gain give exactly one proposal; 150 check-set
   decisions give none and SUT-600 (`proposer.rs`).
5. A proposal that helps the tuning set but not the check set fails with
   SUT-601 and nothing changes (`compare.rs`, and a ganit test).
6. A proposal that lowers anchor agreement beyond tolerance in one section
   fails and the record names that section.
7. A passing proposal gives a new settings version by `tuner` with reason,
   benefits, drawbacks and figures, and a running job keeps its old
   version (`promote.rs`). A restart-only setting becomes a proposal for an
   administrator.
8. A step whose harm chance passes the rollback level is reverted alone,
   its setting paused 30 days with SUT-603, others untouched; while a step
   is watched, nothing else is promoted (`watch.rs`).
9. A manual change pauses tuning of that setting (`manual_pause.rs`).
10. A module whose lower bound falls to 39% is demoted with SUT-606; at 45%
    it stays in shadow; it returns only above 50% after a full period; no
    flapping on any noisy series, and no mode beyond review after any
    promotions (`modes.rs`, property).
11. One moderator accepting everything cannot carry the accept rate once
    job 154's weights apply (`label_weights.rs`, and a ganit test with
    numerical edge cases).
12. Replay from stored scores equals a full re-ranking on the same fixture
    (`replay.rs`).
13. The nightly run and an administrator saving settings at once, under
    loom: the tuner withdraws with SUT-602 or writes after with the right
    parent; never a lost update.
14. The tuner killed mid-comparison resumes from its checkpoint with the
    same verdict (chaos, nightly).
15. Replay of 100,000 pairs on Dell takes at most 2 s, held by the 5% median
    rule (`benches/replay.rs`); a 2-minute mix of history and accept-rate
    requests stays at 300 ms and 100 ms at the 95th percentile (load).
16. Erasure deletes the labels and marks proposals "evidence reduced".
17. The history shows each step's change, reason, gains, costs and outcome
    with no post text (acceptance and snapshot tests).
18. The training export holds ids, hashes and reason codes only, nothing
    from content no longer visible, and nothing aimed at transformer
    retraining.
19. From a weight of 1.00 and job 45's target of 1.30, over several nights,
    each step moves at most 5%, passes its own shadow test, and the target
    is reached only through such steps (property and Kani proof).
20. Rule-critical code (the step check, the bound check, one change under
    watch, the revert of exactly one step, never beyond review): 100%
    lines, branches and MC/DC, every mutant killed, property tests and Kani
    proofs.
21. Each fixed bug gets a regression test that fails on the parent commit.

## Changes from the source

- Tuning runs per site on the owner's PC, not on each forum's server.
- ganit is the Rust numerical engine inside the service, not a separate
  batch container; its sub-modules are named by job number here.
- The small networks are trained by ganit in pure Rust, and LambdaMART is
  served by a pure-Rust model, not a native library.
- Memory fits the limits in AGENTS.md section 5.6, in place of the old
  5.6 GB cap.
- Speed goals follow the 5% median rule; the old speed-multiple goal is
  dropped.
- Coverage adds MC/DC and kills every mutant.

Source: `docs/specifications/linking/feedback.md`,
`linking/feedback-2.md` and `linking/feedback-3.md` in the archive
`archive-2026-11-08-old-repo`.
