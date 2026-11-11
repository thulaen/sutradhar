# 0027: Candidates, relevance and ranking

Ranking decides which reranked destinations a moderator sees, in what
order, and how sure the system is. It sits between matching's reranking
and the assembled suggestion (spec 0023, steps 6 and 7), and uses
ganit's outputs to do it. It is roadmap Phase 4 item 3.

In short: matching finds up to fifty threads per post and the reranker
scores each pair. Ranking keeps only the clearly relevant ones (a hard
gate: popularity or need can never buy relevance), orders those by
need and quality, and offers at most five per phrase, each with a
"likely to be accepted" figure and an explanation a moderator can read.
Example: for "spare power supply" the reranker likes thread A best,
but A is three years old, closed and marked "Outdated"; B is newer,
solved and names the product. B is shown first ("Likely accept 78% (70
to 85%)") and A second; the "why" panel shows reranker view, quality,
freshness and graph relevance as separate lines.

## Words used here

- Ranking method: a catalogue method (spec 0055) of pipeline
  `shodhan.rank` that scores one destination for one source phrase,
  from 0 to 1. Feature score: one method's normalised score for a pair.
  A relevance method's input and output kinds, alone or in a chain, are
  spec 0068's feature table (spec 0068, "Output kinds and probe sets",
  item 3).
- Scoring model: the formula turning feature scores into one final
  score: logistic fusion now (a weighted sum passed through a curve
  from 0 to 1), LambdaMART later if it wins (ganit job 45).
- Confidence: how likely a moderator is to accept the link, with a
  range (ganit job 33).
- Link set: the links chosen for one post: which anchors and which
  destinations, with no overlapping anchors, no repeated destination, a
  link cap and some variety (ganit job 1).
- Degraded mode: the plain fallback when ganit's live service does not
  answer in time.
- Shadow test: running a new ganit output beside the active one on the
  same posts before it is used.
- Playbook: a named preset (spec 0014), Balanced by default or
  Aggressive.
- Overlooked content: destinations few links point to, as job 324's
  need score measures it.
- Graph relevance: closeness in the link map (ganit's Jaal engine).
- Situation: one tested playbook case with an expected outcome a test
  can check.
- Provisional: ranked with some applicable methods still running;
  shown, never approvable.

## What it does

### Scope

In scope: extra candidate generators; the starting ranking methods;
the final score, confidence, the relevance gate and the link-equity
rules; job 1's link set and degraded mode; live exploration; playbooks
and situations; provisional and complete rankings; the explanation,
"no suggestions" and review priority; and activating, shadow-testing
and rolling back ganit outputs here.

Out of scope: everything spec 0023 owns (hard rules are filters there,
never weights here: no score can buy past a safety rule); ganit's
engines and job code; the review page and insertion (spec 0028);
training and tuning from decisions (spec 0053); settings storage
(spec 0014); spam detection itself (kavach); reactions and votes.

### Where ranking sits

For one source post, after spec 0023 has reranked its candidates:

1. Gather evidence for each pair: matching's scores; destination facts
   read by sampark (state, prefix, age, solved answer, author; never
   popularity counts such as views, reactions or replies); darshak's
   human-only rates; kavach's spam signal; shodhan's entities and
   keyphrases; and the active outputs of the ganit jobs below.
2. Ask the method registry (spec 0055) which ranking methods apply;
   run the ready ones; mark the rest waiting.
3. Compute the relevance score with the active scoring model and the
   confidence with the active calibration; apply the relevance gate,
   then order the passing destinations by relevance, need and quality.
4. Ask ganit's live service for job 1's link set, within its time
   limit; check its answer against every hard rule and the link-equity
   rules; on time-out, error or a broken rule, use degraded mode.
5. Apply the playbook's display threshold and hand the list to
   spec 0023's step 7.

The first useful suggestion is ranked with the methods ready then and
marked provisional. Ranking is redone as each method finishes, and is
complete when every applicable method has finished and all 50 pairs
are reranked.

### Candidate generators

Matching's exact search is the main generator, unchanged. Two more sit
behind one trait, `CandidateGenerator`:

- Lexical: BM25 over destination titles and piece text with tantivy,
  for exact product names and codes that vectors can blur; its k1 and
  b come from Samanjan's job 115 within their bounds.
- Graph: the top link-prediction candidates per thread from Jaal's job
  256 (Adamic–Adar, resource allocation, Katz), read from its active
  output. The ranking module holds no graph code of its own.

Every candidate still passes all of spec 0023's hard filters
(including "the embedder has read its title and summary") and needs an
existing-words anchor. Extra candidates only fill places when meaning
search finds fewer than 50 eligible destinations, so spec 0023 is
unchanged; merging the lists before the cut is a lab experiment first.

### The starting ranking methods

Each is a catalogue entry (spec 0055) with its sheet, distinguishing
test and lab run, and must pass admission to count.

| Method | What it adds | Evidence |
|---|---|---|
| Reranker relevance | matching's reranker score | reranker |
| Retrieval similarity | matching's retrieval score | vectors |
| Lexical match (BM25) | exact words and codes | tantivy |
| Title overlap | phrase words in the title | shodhan |
| Entity agreement | same people, products, places | shodhan |
| Keyphrase agreement | shared statistical keyphrases | shodhan |
| Graph relevance | link-map closeness | jobs 256, 276, 278 |
| Acceptance prior | past accepts for similar pairs | job 94 |
| Destination quality | rates only: solved, engaged visits | jobs 325, 66, 68 |
| Freshness | attention decay; outdated answers | jobs 222, 109 |
| Prefix caution | "Closed", "Outdated", "Scam alert" | prefixes |
| Spam risk | kavach's signal on the destination | kavach |
| Rejection-prone target | targets often rejected | job 97 |
| Self-promotion caution | same author on both sides | authors |
| Need (overlooked) | inbound links, depth, unfound | jobs 324, 49, 253 |
| Anchor quality | specificity, generic anchors | jobs 51, 296 |
| Linkability | how many posts could link here | job 52 |
| Position weight | a first-post link weighs more | shodhan |

- Destination quality, freshness, need and linkability are order-stage
  methods: they only reorder destinations that passed the gate. Every
  other row is a relevance method, feeding the scoring model, the
  calibrated probability and the gate.
- Reaction rates (the share of Sad and Angry reactions, which count as
  negative; never totals) feed destination quality once reactions are
  supplied; until then that input waits, never zero.
- Graph relevance is always its own line in the explanation.
- Where a small neural head wins inside its job (such as job 66's
  thread type), Rust serves it with its own forward pass, gated by the
  lab's parity test (spec 0056); the method's contract is unchanged.
- PageRank-like ideas are written from published descriptions, with no
  claim of equivalence to Google's systems.

### The final score and confidence

- Each method returns a feature score from 0 to 1 and a direction.
- The final (relevance) score is the active scoring model applied to
  the relevance methods' scores only: logistic fusion, one weight per
  method (job 45, trained by Shiksha), still a weighted sum explainable
  line by line. Every weight is positive and inside its bound. Newly
  trained weights are a target reached by bounded, shadow-tested steps
  of at most 5% (spec 0053).
- A method chain of spec 0068 fuses only the columns its own feature
  table holds. Job 45 trains the weights for each such set of columns
  on the same terms, and job 33 fits the calibration for each such set
  and fusion method on the same terms; spec 0069 sets its own rule for
  its shape's fusers and calibration. The live service fuses every
  ready relevance method, as above.
- A waiting or failed method has no feature score, so the pair has no
  complete final score. A provisional one, from finished methods only,
  orders the first useful suggestion and is marked so. Nothing missing
  counts as zero.
- The `Ranker` trait applies the model. LambdaMART (job 45, once 2,000
  decisions exist) is served only after its experiment passes and the
  owner approves.
- Confidence: the `Calibrator` trait applies job 33's active
  calibration table (Mapan), with its range. Below 200 decisions the
  figure says "estimated from the forum's own links". The ranking
  module computes no statistics itself.
- Every ganit output used records its version on the suggestion, so a
  suggestion can always be explained and replayed.

### The relevance gate

"Relevance is a hard gate (calibrated acceptance probability and
reranker relevance both above their per-forum thresholds). Need and
quality only reorder candidates that pass the gate, and can never lift
one over it." So ranking runs in two stages:

1. The gate. A destination passes only if both its calibrated
   acceptance probability (job 33) and its reranker relevance are above
   the site's `ranking.gate.min_acceptance` and
   `ranking.gate.min_relevance`, bounded per-site settings (the second
   proposed from job 35's precision curve). No playbook changes them.
   The probability comes from relevance methods only. Job 3's display
   threshold applies after the gate and can only hide more. A
   destination that fails is refused as "below the relevance gate"
   (SUT-550).
2. The order. Among those that passed, the order key combines the
   relevance score with need (job 324: inbound links, click depth,
   search impressions at positions 8 to 20, useful but unfound content)
   and quality (job 325: a floor per target from rates, shrunk towards
   the site average for low-traffic threads: engaged-visit rate,
   solved, reports, outdated), each with a positive, bounded weight,
   then freshness (job 222) and trust (a caution that only lowers a
   place), as the signals section below sets out. No value of any of
   these can bring back a destination that failed.

- No popularity counts: total views, total reactions, reply counts and
  similar are never inputs; rates are used instead. Structural counts
  are allowed: job 324's inbound links, click depth and impressions,
  and job 49's diminishing-returns curve. Each method declares its
  inputs' kinds, and a gate check refuses any method declaring a
  popularity count.
- Diminishing returns: need falls as inbound links grow (job 49), so a
  well-linked thread gains less from each new one.
- Only human views feed ranking: darshak's human population alone.
  When a thread's bot share or anomaly flags pass their thresholds,
  its traffic features freeze at their last good values, and the
  explanation says "traffic frozen".
- Overlooked content gets a small, capped lift and a report column
  (spec 0029).

### Signals from other features

Every feature that learns something about a page passes it to ranking as
a typed signal. These rules hold for every signal:

1. A signal is a catalogue entry (spec 0055), with its sheet, its input
   kind (rate, share, fact or label), its source features, its door and
   its test. It counts only once admitted.
2. A signal is only a rate, a share, a fact or a label. It reorders
   links that already passed the relevance gate, and nothing else. It
   never feeds the scoring model, the calibrated probability or the
   gate.
3. A signal enters by one of four doors, all in the order stage:
   - quality: a rate or fact about how good the target is (job 325's
     floor, shrunk towards the site average for low traffic);
   - need: how overlooked the target is (job 324);
   - freshness: how current the target is (job 222);
   - trust: a caution that only lowers a target's place, or freezes its
     traffic inputs at their last good values. It never raises one.
     The existing caution methods (prefix, spam risk, self-promotion)
     stay as they are; a trust signal acts in the order key only.
4. Popularity, traffic and counts can never buy relevance. Total views,
   visits, plays, reactions, registrations and outside links are never
   inputs; the gate check refuses any signal declaring a popularity count
   (test 25). The structural counts above (inbound links, click depth,
   impressions) stay allowed, as for every ranking method.
   Rates come from human views only, and freeze as test 27 sets out.
5. A fact may also feed an existing hard filter (spec 0023), such as
   canonical, noindex, language or blocklist facts. A hard filter can
   only refuse, so it can never move a link up.
6. Some features supply data to an existing relevance method, such as
   entities for entity agreement. That is not a signal: the method's
   own entry and admission govern it. An idea that could tell relevance
   itself, such as journey stage agreement, becomes a relevance method
   only through its own entry and lab admission (spec 0055). Until
   then it gives nothing to ranking.
7. A feature with nothing to give says so: its entry says "signal:
   none", and the lists below name it.

Every signal is held by two shared tests:

- RS00 (`tests/props_signals.rs`, property; Kani proof of the order
  key): for every admitted signal, give each destination below the gate
  the best possible value and each passing one the worst. The set that
  passes the gate is unchanged, and every destination below it is
  refused with SUT-550.
- RS-REG (`tests/signal_registry.rs`): every signal entry names its
  source, kind, door and test; a count input, a missing test or a door
  outside the four fails; every feature group below with no signal has
  a "none" entry.

Each row's own test, below its table, uses one fixture: an off-topic
target (below the gate) beside on-topic targets that pass.

#### From the new features (Range 4)

| ID | Source | What it measures | Door |
|---|---|---|---|
| RS01 | 1314 | target fails mobile usability checks (fact) | quality |
| RS02 | 1320 | target's head HTML is broken (fact) | trust |
| RS03 | 1322 | target is far from its section's topics (share) | trust |
| RS04 | 1327 | date of the target's last real rewrite (fact) | freshness |
| RS05 | 1328 | fuller human rates from CDN logs (rate) | quality |
| RS06 | 1329 | counter missing on the target's template (fact) | trust |
| RS07 | 1333 | section's crawled but inactive share (share) | need |
| RS08 | 1341 | assisted-registration rate, shrunk (rate) | quality |
| RS09 | 1344 | leave-at-once rate from the target (rate) | quality |
| RS10 | 1354 | play rate on media targets (rate) | quality |
| RS11 | 1355 | target traits, such as steps or a table (fact) | quality |
| RS12 | 1346 | the owner's type label for the target (label) | trust |

- RS01: the off-topic target passes every mobile check and is still
  refused; an on-topic target that fails only moves down.
- RS02: a clean head on the off-topic target does not let it pass; a
  broken head only lowers an on-topic target.
- RS03: the off-topic share can only lower; an off-topic target with a
  share of zero is still refused.
- RS04: a rewrite dated today on the off-topic target leaves it
  refused.
- RS05: adding CDN rows that raise the off-topic target's rates leaves
  it refused; adding bot rows changes nothing (test 38).
- RS06: a target with no counter has its rates wait, never zero, and
  the off-topic target stays refused.
- RS07: the highest inactive share in the off-topic target's section
  leaves it refused.
- RS08: an assisted-registration rate of 1.0 on the off-topic target
  leaves it refused; raising its registration count by 100 changes
  nothing.
- RS09: a leave-at-once rate of zero on the off-topic target leaves it
  refused.
- RS10: a play rate of 1.0 on an off-topic video leaves it refused;
  multiplying its plays by 100 changes nothing.
- RS11: every trait present on the off-topic target leaves it refused.
- RS12: a label can only add caution; per-type calibration needs its
  own admitted entry, and without it the gate result is unchanged.

Relevance ideas only after their own admission (rule 6): 1332 journey
stage agreement; 1343 a pair's next-page rate, never alone past the
gate; 1356 query-to-anchor agreement, like feature 58.

None: 1313, 1315, 1316, 1317, 1318 (member-only pages
are never targets), 1319, 1321, 1323, 1324, 1325, 1326 (none new; it
keeps target facts current), 1330, 1331, 1334, 1335, 1336, 1337 (the lab
may use it to check fairness by segment), 1338, 1339, 1340, 1342 (it
explains, it does not score), 1345, 1347 (it cleans human rates; no new
signal), 1348, 1349, 1350, 1351, 1352, 1353 and 1357.

#### From the existing feature groups

| ID | Source group | What it measures | Door |
|---|---|---|---|
| RS13 | Linking and site structure | orphans, click depth | need |
| RS14 | Search Console intelligence | impressions at 8 to 20 | need |
| RS15 | Search Console intelligence | cannibalising rivals (fact) | trust |
| RS16 | Content | answer completeness rate | quality |
| RS17 | Topical authority | hub coverage (share) | need |
| RS18 | Reader behaviour | link satisfaction, exit rates | quality |
| RS19 | Search data and insight | evergreen label | freshness |
| RS20 | Protection; Reporting | regression and anomaly flags | trust |
| RS21 | Content quality | version tags, expired flags | freshness |
| RS22 | Content quality; E-commerce | expired, out-of-stock facts | trust |
| RS23 | Log files and bot analysis | bot share | trust |
| RS24 | Rank and visibility tracking | position band | need |
| RS25 | Content optimisation | content score rate | quality |
| RS26 | Page speed | slow template (fact) | trust |
| RS27 | Sponsors; Compliance; Commerce | sponsored, disclosure facts | trust |
| RS28 | Integrations; Audience | GA4 and engagement rates | quality |
| RS29 | Research and sources | best-source score (rate) | quality |

- RS13, RS14, RS17 and RS24: the highest need on the off-topic target
  leaves it refused; need falls with inbound links (test 26).
- RS15, RS20, RS22, RS23, RS26 and RS27: each fact can only lower or
  freeze; with every caution cleared, the off-topic target is still
  refused.
- RS16, RS18, RS25, RS28 and RS29: a rate of 1.0 on the off-topic
  target leaves it refused; multiplying its views by 100 changes
  nothing (test 25).
- RS19 and RS21: the freshest value on the off-topic target leaves it
  refused.

Into hard filters only (rule 5), which can only refuse: Technical
health (canonical, soft 404); Site crawler and technical audit
(indexable, canonical, status); International and multi-site and
Localisation (language match); Distribution and syndication (canonical);
More technical checks (blocked, duplicate); Quality gates (a failed
draft is never a target); Variants and personalisation (variants link
to the master only); Safety and policy guardrails (blocklist topics).

Data for existing relevance methods (rule 6), under their own entries:
Linking and site structure (anchor guard, into the equity caps);
Structure and linking (query-aligned anchors, into anchor quality;
density guard, into the caps); Site crawler (link reports, into graph
relevance); Brand entity and knowledge panel (entities, into entity
agreement); Measurement and learning (rejection reasons, into job 97);
More analytics, workflow, crawl and press features (content similarity,
into job 1's variety term, which only spreads links); Planning and
outlining, and Publishing (feature 1255), which propose candidates that
still pass the gate.

Relevance ideas only after their own admission: Search data and
insight (intent labels); Keyword and topic research (query clusters;
the opportunity score never); Article types (content type labels, for
per-type calibration).

None: Proof and planning (it measures the engine's effect); AI search;
Members and staff; Rich results and media; Competitive intelligence;
Backlinks and digital PR (outside links are not relevance); AI search
visibility; Experimentation (it tests ranking changes); Local presence,
listings, and reviews; Social publishing and engagement; Brand
monitoring and listening; Brand assets and guidelines; Surveys and
research; Workflow automation; Team, governance, and enterprise; Video
and podcast SEO; Data platform; Alerts and notifications; In-app SEO
assistant (it explains ranking); Platform extras; Community-led growth;
Paid search alignment; Other search engines; The MCP content toolkit;
Forum-native content; Learning content; Reference content; Community and
policy pages; Visual content; Audio and video production; Email and
messaging copy; Site copy and UX writing; Editing operations;
Collaboration; Staff and contributor access.

### The link-equity rules

One Rust function, `equity::check`, used by the selector (on job 1's
answer and on degraded mode's list) and again by spec 0028 at
approval. Each rule refuses with its own reason; the settings are
spec 0014 keys with approved bounds, defaults calibrated on Dell.

1. Existing words only: the anchor is text already in the post at its
   offsets (shodhan's placement check).
2. Anchor variety per target: once a destination has at least 4 links
   placed or waiting, no exact anchor text may exceed
   `linking.equity.anchor_max_share` (0.3) of them; job 7 sets the mix.
3. Caps per post by length: at most one link per
   `linking.equity.words_per_link` (150) words of readable body text,
   never above the thread's allowance (job 47 tunes it).
4. Per-target caps: at most `linking.equity.target_per_day` (3) and
   `linking.equity.target_per_week` (10) new links to one destination,
   counting approvals and applies waiting to run.
5. Sources spread: new links to one destination come from different
   threads, and at most `linking.equity.same_section_share` (0.5) of
   its new links in a week come from one forum section.
6. Canonical, indexable, guest-visible targets only: the canonical
   thread after merges and redirects, not marked noindex (as spec 0044
   reads it), and guest-visible now.
7. Post body only: never in signatures, quotes, code, spoilers, titles
   or extras.
8. No reciprocal-only pairs or loops: refused when the destination
   thread already links to the source thread and neither has an
   inbound link from a third thread, and when the link would close a
   loop of two or three threads made only of Sutradhar's links; job
   258's spam-pattern flags are shown beside it.
9. Review mode by default: every suggesting module starts in review
   (or shadow for testing), so a person approves each link; version 1
   has no other mode.

A refused destination drops out with its reason (SUT-551 to SUT-558,
one per rule 2 to 8; rule 1 keeps shodhan's codes; rule 9 is a
configuration check), and the next passing destination is offered.

The thread allowance is per thread with its replies (or resource with
its updates), never per post; quotes, navigation and mentions do not
count towards it.

### The link set per post (job 1) and degraded mode

- ganit's live service answers job 1 within a strict time limit: per
  anchor up to five destinations, no overlapping anchors, no
  destination twice in the post, the post's link cap (jobs 47 and 48),
  anchor variety (job 7) and a variety term among alternatives
  (near-duplicates from job 286 count as one).
- The answer is never trusted blindly: every hard rule and filter is
  checked, and anything that breaks one is dropped.
- Degraded mode, on time-out or error: per anchor, the top five that
  pass the gate, by order key, with the hard rules, the link-equity
  rules and the thread allowance; never a copy of job 1's optimiser.
  The suggestion records "chosen by the simple fallback", and job 1 is
  asked again when the live service is back, while the suggestion is
  still waiting for review.

### Live exploration

- To keep learning which destinations moderators value, a small share
  of alternatives' order is drawn by Thompson sampling (`rand_distr`)
  from job 95's posteriors per site, target type and anchor type. The
  share is a bounded setting; job 149 scores a higher share offline
  before it may rise. Off in Balanced; 5% in Aggressive, once job 149
  shows it helps.
- Exploration only reorders destinations that passed every rule and
  threshold; it never shows a weaker link than the playbook allows, and
  every link still needs approval.

### Playbooks

- Balanced (default) and Aggressive are spec 0014 presets inside the
  same bounds; neither is code.
- Aggressive may lower the display threshold within its bound (job 3
  gives Balanced's value), raises need's weight in the order stage
  within its bound, and puts older posts into backfill earlier. It
  never lowers the relevance gate.
- Both run the same hard rules, approval and allowance; a test proves
  the rule and approval code is the same under both.

### Playbook situations

More than 10,000 distinct, tested situations are needed, each with a
real difference and an executable expected outcome.

- A situation is a TOML file under `tests/situations/<area>/`: made-up
  posts and destinations (or a seeded generator call), the playbook,
  changed settings, fixed ganit outputs (scoring model, calibration,
  job 1's answer or a time-out), and the expected outcome: which
  destinations in which order, which refused and why, and whether it
  is approvable.
- One runner (`situations.rs`) runs them against the real ranking code
  with fakes for models and ganit, so they need no download.
- Each run records a decision trace (filters, methods, thresholds, and
  job 1 or degraded). Two situations with the same trace and outcome
  are the same, and the second fails (SUT-548) unless its file says
  what it adds. The digest code is shared with spec 0055's distinctness
  rule.
- A generated family counts once per distinct trace.
- The runner prints the count; the release gate needs more than
  10,000.

### Explanation, "no suggestions" and priority

- The explanation lists each method's score, weight and reason, each
  feature's importance (job 46), graph relevance on its own line, the
  PageRank gain if accepted (job 259), the confidence and range, and
  every version used. Ids and numbers only.
- "Why no suggestions?" lists matching's filter reasons, no acceptable
  anchor, below the display threshold, or methods waiting.
- Review priority comes from job 4 (the queue order giving the most
  accepted links per moderator minute); spec 0028 uses it.
- On the page, confidence shows as a percentage with a range, such as
  "Likely accept 78% (70 to 85%)".

### Ganit jobs used

Live, in ganit's live service: job 1 (the best link set per post).

Nightly, in ganit's batch runs, shadow-tested and activated here:

- Chayan: 3 (display threshold), 4 (review-queue order), 7 (anchor
  variety targets).
- Mapan: 33 (calibrated acceptance probability), 34 (conformal count of
  alternatives per post, never above five), 35 (per-site reranker
  threshold).
- Shiksha: 45 (scoring model), 46 (feature importance), 47 and 48
  (link density and same-target caps, inside the allowance), 51
  (anchor quality), 52 (linkability), 66 (thread type), 68 (likely
  solution).
- Anuman: 94 (acceptance priors), 95 (exploration posteriors), 97
  (targets that attract rejections), 109 (outdated solved answers).
- Samanjan: 115 (BM25 k1 and b and other retrieval settings; the rerank
  depth stays 50).
- Mulyankan: 142 (threshold-slider surfaces).
- Vikalp: 147 (new ranking policies scored offline), 149 (exploration
  rates scored offline).
- Bahav: 192 (score shift after a change, triggering recalibration).
- Gati: 222 (attention decay, for freshness).
- Jaal: 49 (diminishing returns), 251 (internal PageRank), 253 (orphans
  and click depth), 256 (link-prediction scores and candidates), 258
  (spam-pattern flags), 259 (PageRank gain if accepted).
- Varnakram: 276 (spectral graph embeddings as features), 278 (topic
  communities, a part of graph relevance).
- Sannidhya: 286 (near-duplicate clusters and canonical threads).
- Padbandh: 296 (generic anchors, joining matching's stop list).
- Order stage: 324 (need) and 325 (quality floor).

Matching takes job 2 for its re-score margin, job 136 for when to
re-score and job 280 for hubness correction.

### Ganit outputs

- `ganit_outputs` is one small module that loads, shadow-tests,
  activates and rolls back each consumed ganit output. Every job's
  output goes through it.
- A new output with NaN, an out-of-bound weight or worse shadow
  results is not activated (SUT-544); the active one stays.
- An overdue output: the last good one stays active, the explanation
  says how old it is, and an automatic issue is filed (SUT-543).
- A rollback is one step and audited.
- An output made from a snapshot older than a model change fails its
  shadow test until a new one is made.
- Each post is ranked wholly with one version of each output, even if
  an activation lands mid-ranking.

### Data stored

In PostgreSQL with row-level security forced on every table and the
site set per transaction. Every row is derived from guest-visible
content and purged with it.

`rank_feature`, one row per pair and method:

| Column | Meaning |
|---|---|
| `suggestion_id`, `destination_id` | the pair |
| `method_id`, `method_version` | catalogue identity |
| `state` | finished, waiting, failed |
| `raw`, `score` | the method's value and its 0 to 1 score |
| `reason` | short code for the explanation |
| `error_code` | when failed |

`rank_result`, one row per pair: `final_score`, `provisional`,
`confidence`, `confidence_low`, `confidence_high`, `position`,
`chosen_by` (job 1 or fallback), `explored`, `playbook`,
`settings_version`, and the versions of every ganit output used.

`ganit_output`, one row per consumed output version: job number,
version, ganit code version, snapshot digest, seed, metrics, state
(shadow, active, rolled back, retired), when activated and rolled back,
and by whom. The payload (weights, tables, scores) is stored beside it.
The ranking module writes this table from ganit's answer; ganit never
writes it.

Uninstall: these rows follow matching's suggestion records (export
offer and keep-or-delete choice).

### Interfaces

Management API routes on the local socket (Axum, described with
utoipa), used by the Dioxus app and `sutra`:

- `GET /api/suggestions/{id}/explanation`: the explanation.
- `GET /api/posts/{id}/no-suggestions`: "Why no suggestions?".
- `GET /api/ranking/threshold-preview?min_confidence=<x>`: answered
  from job 142's surfaces where available, else counted from stored
  results.
- `GET /api/ranking/playbook` and the action `rank.playbook.choose`
  (Balanced or Aggressive), writing a spec 0014 version.
- `GET /api/ganit/outputs?consumer=shodhan.rank` and the action
  `rank.ganit_output.rollback` (one output, one step, audited).

The site's review page gets explanations and confidence through the
signed site link (spec 0021), as ids and numbers.

Calls to ganit: job 1 through its live service's versioned API, with
ids and numbers only (never text), the trace context and a deadline;
nightly jobs through durable jobs.

Events (sanchar, through the outbox), ids and codes only:
`ranking.updated`, `ranking.method_failed`, `ranking.degraded` (job 1
missed; count per minute), `ganit_output.activated` and
`ganit_output.rolled_back`.

Command line: `sutra rank explain <suggestion>`, `sutra rank playbook`
and `sutra ganit outputs`, through the same actions.

### Code layout

- shodhan's `rank` module: the orchestrator, `Ranker`, `Calibrator`,
  the job 1 client and degraded mode, exploration, playbook thresholds
  and the explanation; each method under `rank::methods`; generators
  under `shodhan::generate`.
- Traits with test fakes: `CandidateGenerator`, `Ranker`,
  `Calibrator`, `LinkSetSolver` (a fake that answers, delays or fails
  on command), `SpamSignal`, `DestinationFacts` and `GanitOutputs`.
- Ranking runs in the inference worker on its rayon pool, because it
  needs the vectors only that process holds.

### Failure handling

Plain-English messages with codes:

- SUT-540: a method failed for a pair; the suggestion stays
  provisional, not approvable, and the explanation names it. An
  administrator may switch the method off with a recorded reason
  (spec 0055).
- SUT-541: a ranking input is missing (such as reactions); the method
  waits, never zero.
- SUT-542: no calibration active yet: nothing can pass the gate; the
  post shows "waiting for calibration" until job 33's first output
  (from link reconstruction) is active.
- SUT-543: a ganit output is overdue (above).
- SUT-544: a new output failed its shadow test or numerical guards.
- SUT-545: a generator failed; its candidates show as missing
  coverage; matching's search still runs.
- SUT-546: a weight outside its bound in a stored version or an output:
  refused; the previous stays.
- SUT-547: the ranking work budget for a post is exceeded: retried
  once in backfill, then reported.
- SUT-548: two situations with the same trace and outcome.
- SUT-549: job 1 timed out or failed: degraded mode was used; the
  suggestion says so and is asked again later.
- SUT-550 to SUT-558: the gate and link-equity refusals above.

A crash during ranking keeps finished feature scores; at most the pair
being scored repeats. If ganit's live service is down, suggestions keep
flowing in degraded mode; nothing waits on nightly jobs.

Edge cases:

- Ties break by destination id, so results repeat.
- A destination marked "Outdated" after ranking moves at the next
  re-ranking; approval re-checks only spec 0028's hard rules.
- Merged or redirected destinations are resolved by matching first.
- Same author on both sides: allowed, with the caution score and a
  mark.
- One slow method runs last; the time to complete records it.
- ganit's live service restarting: degraded mode meanwhile, no lost
  posts.

### Security and privacy

- Only guest-visible content and its facts are used.
- ganit receives ids, numbers and word counts, never post text; a
  contract test checks every request shape.
- Explanations, events, metrics and logs carry ids, scores and codes,
  never post text, anchors or names.
- Author identity is used only as "same author on both sides", a yes
  or no, never stored with the score.
- Spam signals are ranking hints, never an action against a member.

### Observability

- Metrics (ids and counts only): `sutradhar_rank_seconds`,
  `..._provisional`, `..._method_state_total`,
  `..._time_to_complete_seconds`, `..._degraded_total`,
  `..._job1_seconds`, `..._ganit_output_age_seconds` by job, and the
  generators' hit counts.
- A trace span per post's ranking, with a child span for the job 1
  call carried into ganit.
- Health page: failing methods, overdue outputs, degraded rate.
- Alerts: time to complete burning its target; degraded mode above 1%
  for an hour; an output overdue by two schedules.

## Limits

Speed, on Dell under fixed conditions, imitating the reference
hardware of 4 cores and a SATA solid-state disk where stated:

- New posts keep their 30-second first suggestion: ranking adds at
  most 20 ms for one post's 50 pairs at the 95th percentile, excluding
  model time; job 1 answers within 150 ms at the 95th percentile and is
  abandoned for degraded mode at 300 ms.
- Degraded mode on at most 1% of posts in a day on a healthy system
  (an alert, not a gate).
- 99% of new posts are approvable within 10 minutes of saving, on the
  reference hardware.
- The "why" panel opens within 300 ms; threshold previews within 1
  second over 100,000 suggestions waiting for review.
- Lexical generator within 50 ms per post at the 95th percentile on
  1,000,000 posts; graph candidates (a table read) within 5 ms.
- A new ganit output is shadow-tested and activated within 10 minutes
  of arriving, between jobs.

Memory, inside the service slice, within the memory limits in
AGENTS.md section 5.6, with 15% headroom; estimates until measured on
the 50,000-thread tier:

- Inference worker: at most 64 MB per post in flight; two posts at
  once (one live, one backfill).
- Active ganit outputs held for ranking (weights, calibration, priors,
  graph scores for the top candidates per thread): at most 64 MB on a
  PC under 16 GB, 150 MB otherwise.
- Lexical index: memory-mapped, at most 150 MB resident, as disk cache
  the kernel can take back.
- ganit's live service: about 100 MB on a PC under 16 GB, up to about
  0.4 GB with 16 GB or more. It and the ganit outputs count in spec
  0024's row for other parts' standing budgets. Its nightly batch runs one job
  at a time in 0024's heavy job slot: at most 0.8 GB under 16 GB, 1.2 GB
  otherwise.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

All tests use made-up data and fakes unless they say Dell. Paths are
relative to `crates/shodhan/`. Test-first order: scoring with a fixed
output; missing and failed methods; `ganit_outputs` and the loom test;
the job 1 check and degraded mode with the Kani proof; generators and
the join; the starting methods, each with its distinguishing test;
exploration, playbooks and situations; explanation; then speed, load,
Dell acceptance, ganit's own tests, regressions and the neural head.

Rule-critical code (the job 1 check, degraded mode, playbook
equivalence and every link-equity rule): 100% lines, branches and
MC/DC, no surviving mutants, property tests, and Kani proofs that an
accepted link set never has overlapping anchors, never repeats a
destination and never exceeds five per anchor (`proofs/linkset_check.rs`,
`proofs/equity.rs`).

Facts to measure or confirm: tantivy's BM25 indexes piece text by post
with offsets within the memory figures; job 1 meets 150 ms at the 95th
percentile on 4 cores for a post with 20 anchors and 50 destinations
each; the memory figures; what XenForo 2.3 stores for prefixes, solved
answers and views, read through sampark; job 45's fit keeps every
weight positive; SUT-540 to SUT-558 are unused elsewhere; how noindex
state is read and kept current; the equity defaults, calibrated by link
reconstruction before release.

Acceptance tests (Rust):

1. A pair with every method finished and an active model: its final
   score equals the model applied to its feature scores; every weight
   is positive and in bound. (`tests/rank_score.rs`;
   `tests/props_rank.rs`, property)
2. A pair with one method waiting for reactions is provisional, not
   approvable, says "waiting for reaction counts", and used no zero.
   (`tests/rank_missing.rs`)
3. A method failing for a pair shows SUT-540; other scores stand; the
   suggestion is not approvable. (`tests/rank_failure.rs`;
   `tests/chaos_rank.rs`, failure injection)
4. A first useful suggestion with three methods running is re-ranked
   as each finishes and is complete only after the last and all 50
   pairs. (`tests/rank_progressive.rs`)
5. Any job 1 answer, hostile ones included (overlapping anchors, a
   repeated destination, six alternatives, a filtered destination),
   offers nothing that breaks a hard rule. (`tests/props_linkset.rs`,
   property; Kani proof)
6. With the live service delayed past 300 ms or failing, degraded mode
   offers at most five per anchor that pass the gate and every equity
   rule, by order key, records SUT-549, and job 1 is asked again.
   (`tests/degraded.rs`, with the fake solver)
7. A lexical-only candidate not in the thread-level index never
   reaches a suggestion. (`tests/generate_filters.rs`)
8. Search finding 30 eligible destinations and the lexical generator
   40 more: the reranker sees at most 50, the first 30 from search
   unchanged. (`tests/generate_join.rs`)
9. One fixture under Balanced and Aggressive: hard-rule outcomes, gate
   results and approval path are the same; Aggressive offers at least
   as many, all past the gate. (`tests/playbooks.rs`; situations)
10. A new output with NaN, an out-of-bound weight or worse shadow
    results is not activated, SUT-544 is recorded, the active output
    stays. (`tests/ganit_outputs.rs`; property over generated outputs)
11. A one-step rollback: the previous version serves the next post and
    the audit log holds both events. (`tests/ganit_outputs.rs`)
12. An overdue job: the last good output is used, its age shown,
    SUT-543 filed. (`tests/ganit_outputs.rs`)
13. An explanation lists every applicable method with score, weight
    and reason, graph relevance on its own line, and no post text.
    (`tests/explain.rs`, snapshot with insta)
14. Exploration at 10% over 10,000 seeded posts changes about 10% of
    orders and never shows a destination below the threshold.
    (`tests/explore.rs`, property)
15. Every situation file's expected outcome holds and no two share a
    trace and outcome. (`tests/situations.rs`)
16. With the ranker shared by live and backfill, loom explores an
    output activation during ranking: each post is ranked wholly with
    one version of each output. (`tests/loom_outputs.rs`)
17. One post's 50 pairs on Dell's bench cores: ranking takes at most
    20 ms at the 95th percentile, and its median of at least 30 runs is
    no more than 5% slower than its baseline. (`benches/rank.rs`)
18. A 2-minute mix of ranking requests with the live service slow a
    tenth of the time keeps throughput and memory within baseline.
    (`tests/load_rank.rs`, load)
19. On the reference hardware imitated on Dell, with 100 new posts and
    backfill running, 99% are complete within 10 minutes.
    (`tests/acceptance/suggest_links.rs`, Dell)
20. Job 1 on fixture posts gives no overlapping anchors or repeated
    destinations and meets the cap. (ganit's own unit and property
    tests)
21. A fixed bug's regression test fails on the parent commit and passes
    after. (`tests/regression/`)
22. A neural head that won job 66, with its exported test cases: Rust's
    forward pass matches the lab's recorded outputs within tolerance,
    and the method's scores equal the expected ones.
    (`tests/neural_head.rs`; the lab's parity check)
23. Acceptance probability 0.62 against a threshold of 0.65, with high
    relevance, is refused with SUT-550 whatever its need and quality.
    (`tests/gate.rs`)
24. For any candidates and any need and quality values, every offered
    destination passed both thresholds, and maximum need and quality
    change only the order of those that passed. (`tests/props_gate.rs`,
    property; Kani proof)
25. Multiplying a target's total views or reactions by 100 leaves the
    result unchanged; a method declaring a count input fails the gate
    check. (`tests/no_counts.rs`; `tests/method_inputs.rs`)
26. Of two equal destinations with 2 and 200 inbound links, the first
    has higher need, and each added link lowers need by less than the
    one before. (`tests/diminishing.rs`, property)
27. A thread whose bot share passes its threshold uses last good
    traffic values and the explanation says "traffic frozen".
    (`tests/frozen_traffic.rs`)
28. An anchor not found at its offsets is refused. (`tests/equity.rs`)
29. A destination with 9 links, 3 using "power supply": a fourth using
    it is refused (SUT-551). (`tests/equity.rs`)
30. A 300-word post where job 1 proposes 3 links keeps at most 2
    (SUT-552). (`tests/equity.rs`, property over lengths)
31. A destination that got 3 links today, or 10 this week, refuses a
    new one (SUT-553). (`tests/equity.rs`, fixed clock)
32. A destination with 5 new links this week, 3 from one section,
    refuses a fourth from that section (SUT-554). (`tests/equity.rs`)
33. A redirected destination resolves to its canonical thread; a
    noindex one and one guests cannot see are refused (SUT-555).
    (`tests/equity.rs`)
34. Of anchors in a signature, a quote and the body, only the body one
    passes (SUT-556). (`tests/equity.rs`)
35. With B linking to A and no third thread linking to either, and a
    loop A→B→C→A of Sutradhar's links, A→B and C→A are each refused
    (SUT-557, SUT-558). (`tests/equity.rs`; `tests/props_loops.rs`,
    property over random small graphs)
36. On a fresh install every suggesting module's mode is review (or
    shadow), and no setting can make it insert links without a person.
    (`tests/modes_default.rs`)
37. A job 1 answer and a degraded list breaking each rule: nothing
    that breaks a rule is offered. (`tests/props_equity.rs`, property)
38. Any change to a thread's bot-labelled views, human views unchanged,
    leaves every gate result, order and explanation unchanged.
    (`tests/props_human_only.rs`, property)
39. A thread whose first post and three replies hold 4 content links
    (quotes, navigation and mentions not counted), with a limit of 5:
    ranking a reply with three good destinations offers at most one
    link for the whole thread, the rest show "allowance used", and at
    the limit no reply in the thread is matched.
    (`tests/thread_allowance.rs`; `tests/props_allowance.rs`, property
    over generated threads)
40. Every signal from other features passes RS00, RS-REG and its own
    row test, as "Signals from other features" sets out: no signal value
    moves an off-topic target past the gate. (`tests/props_signals.rs`;
    `tests/signal_registry.rs`; `tests/signals/`)

## Changes from the source

- ganit, its live service and its nightly jobs are Rust (ganit, the
  Rust numerical engine), running inside the service slice rather than
  in their own containers.
- LambdaMART is served by a pure-Rust model in ganit, not a native
  library.
- The ranking module moves into shodhan, and old `setu` names become
  `rank`, since setu now names the link to the add-ons.
- The review page reaches ranking through the signed site link; the
  management API is served on the local socket only.
- Lab notebooks become Rust lab runs with recorded inputs and outputs.
- The situations count runs with Cargo, not a separate task runner.
- ganit's own job tests are Rust tests.
- Speed follows the 5% median rule; the 20x goal is kept (spec 0007).
- Memory budgets follow the limits in AGENTS.md section 5.6, with
  ganit's live service and nightly batch made to fit a PC under 16 GB.

Source: `docs/specifications/linking/ranking.md`, `linking/ranking-2.md`,
`linking/ranking-3.md` and `linking/ranking-4.md` in the archive
`archive-2026-11-08-old-repo`.
