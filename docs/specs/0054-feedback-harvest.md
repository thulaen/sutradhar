# 0054: Feedback harvest

This part records, for each moderator decision, which ranking the moderator
was shown and where the item sat in it, so that later learning can allow
for position bias. It joins this capture to feedback learning (spec 0053)
and ganit job 146 (moderator position bias), without a second feedback
store, tuner or calibration engine. It belongs to the feature waves of
roadmap Phase 6.

Example: a moderator picks the eighth suggestion. Sutradhar records that
position and the exact ranking shown. That does not prove the eighth
suggestion is better than the others.

## What it does

### Owners

- The Rust service owns checked events, lasting counts, exports and
  activation checks.
- ganit owns approved statistical estimates; the service applies their
  versioned results.
- Visitor clicks stay in the separately consented analytics contract.
- Restricted pseudonymous moderator refs stay governed by spec 0053. The
  lack of identity columns is not taken as proof of anonymity.

### What is captured

1. Each captured event binds: site identity, platform, the permitted
   action, decision identity, suggestion identity, the ranking generation
   that produced it, the displayed rank, source and destination versions,
   model, index and settings versions, and the time observed.
2. The event names the screen that actually displayed the item. Making a
   candidate does not prove it was shown.
3. Existing decision meanings and label weights stay as spec 0053 sets
   them. Ignoring an item does not invent a rejection.
4. WordPress editor actions and XenForo moderator actions use each
   platform's own permission checks.
5. Wrong sites and stale or unpermitted actions are refused. An event
   cannot gain authority by naming an actor or site.
6. Nothing is pooled or trained across sites because they share one
   install.

### What is published

- Versioned per-site aggregates, each with its window, generation, counts,
  schema and source identity.
- The service exports only permitted ids and numbers to ganit.
- Each ganit output names its method, assumptions, input snapshot,
  uncertainty, eligibility and limits. An unsupported version fails
  openly.

### Repeats, recovery and erasure

- A repeated delivery of one accepted event never counts twice, even after
  a restart. Event identity and atomic intake use the existing durable
  jobs and tables; suppression held only in memory is not enough.
- Two payloads with the same event identity but different content fail
  openly, and are never merged.
- A late event keeps the generation actually displayed.
- Replays of the same admitted inputs in the same order give the same
  aggregate.
- An interrupted aggregation never publishes a partial result as whole.
- Work, queue size and storage are bounded through the existing resource
  rules.
- Erasure, withdrawn decisions and retired content invalidate dependent
  labels and aggregates under spec 0053 and the retention rules. An older
  generation never shields kept data from erasure.
- A failed estimate stays unavailable. Missing, zero, invalid and sparse
  evidence stay told apart.

### The maths: rules before any estimator

No estimator is defined here. The existing duty of job 146 stays. A full
numerical amendment, approved by the owner, must come before the method is
coded. No new reorder stage and no weaker relevance gate is allowed, and
bounded activation and moderator approval still apply.

The amendment must hold to these rules:

1. Let N(s,g,t,k) count eligible displayed observations for site s,
   generation g, target t and displayed rank k, and A(s,g,t,k) their
   accepted decisions. The population and the rule for repeated displays
   are fixed before any aggregation. With one yes-or-no decision per
   observation, 0 <= A <= N. Changing the observation unit changes what is
   estimated, and needs review.
2. An examination model may assume P(accept) = e(k) * q(t), under stated
   sampling, exposure and independence assumptions. Counts of this kind
   alone do not identify both factors, so the model is not taken as a known
   model of moderator action.
3. Say what is estimated: relative exposure, expected acceptance or
   comparative ranking loss. They are not interchangeable.
4. Record the sampling and exposure policy and the information needed to
   identify it. Rank alone does not establish examination probability, and
   a prior inferred from rank alone is not measured exposure.
5. Any weighting needs observation probabilities above zero where it is
   used. Missing support is never replaced by a guessed floor.
6. Clipping or normalising, if chosen, is a changed estimator with its own
   assumptions and checks, never hidden as error handling.
7. Show that the method fits moderator decisions, separately from search
   clicks. Analytics consent and moderator privacy rules still apply.
8. No default prior, probability floor, general gain or loop-safety promise
   is assumed.
9. No live rank-swap experiment starts from this part. One would need its
   own permission path and an approved, bounded protocol.

Worked counter-examples the rules rest on:

- Under p = e * q, an observed p = 0.2 fits both e = 0.5, q = 0.4 and
  e = 0.25, q = 0.8. More counts of the same kind do not resolve this
  without more information.
- For N = 100 displays and A = 10 accepts with e = 0.5, A/e = 20 weighted
  accepts, while A/(N*e) = 0.2 has other units. With e = 0.05, A/(N*e) is
  2.0, so it cannot be called a probability in [0,1]. Clamping it in
  silence would change the estimator and hide the failure.

Research still needed: read the full candidate estimators (propensity
weighting with controlled rank swaps, Joachims, Swaminathan and Schnabel,
2017, sections 4, 5.3 and 7.2; estimation from several logged ranking
functions, Agarwal and colleagues), check their assumptions against the
moderator event contract, compare them with job 146, set uncertainty and
support rules, and derive the expected test answers independently. Use
verified citations. Reading a paper gives no licence to its code.

Checked sources only: the full text of the Agarwal paper and of the
Wang paper on personal search is read and its method checked before
either is used. The old source mixed up titles, years and venues in
another citation; cite each paper by its checked title, authors, year
and venue, never by the old source's wording.

- https://www.cs.cornell.edu/~tj/publications/joachims_etal_17a.pdf
- https://arxiv.org/abs/1812.05161

## Limits

- Work, queue size and stored data are bounded through the existing job and
  resource rules, inside the service slice, within the memory limits in
  AGENTS.md section 5.6. Exact limits come from approved settings and
  measurement, not from imported defaults.
- Per site only: no site sees another site's counts, models or moderator
  refs.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Rust tests, with SQL tests for RLS and PHP tests for each add-on.

1. One accepted event delivered three times, including after a restart,
   counts as one acceptance.
2. Two different eligible display events count as two displays; an
   identity reused with different content is refused, not merged.
3. A wrong site, platform permission or content version is refused.
4. A late event keeps the ranking generation actually displayed.
5. An interrupted aggregation resumes with no record missed or doubled.
6. No observations gives "unavailable", never a made-up neutral
   confidence.
7. Consented visitor analytics can never become moderator labels.
8. Erasure invalidates dependent snapshots and leaves other sites alone.
9. Cross-site tests cover reads, writes, replay and erasure: both add-ons
   follow the same contract with their own permission checks, and no site
   receives another's counts, models or moderator refs.
10. Every output records its exact inputs and versions, and a stale output
    cannot activate.
11. Negative cases block correction: unknown e, e = 0, mixed generations,
    no eligible displays, a missing logging policy, unsupported site
    pooling, and finite samples that do not fit the assumed model.
12. Numerical expected answers are derived independently once an estimator
    is chosen; missing support and inputs that cannot be identified block
    correction.
13. Recovery and resource use are measured under the gate.

Source: `docs/specifications/linking/feedback-harvest.md` and
`linking/feedback-harvest-research.md` in the archive
`archive-2026-11-08-old-repo`.
