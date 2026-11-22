# 0027: Candidates, relevance and ranking

Ranking decides which reranked destinations a moderator sees, in what
order, and how sure the system is. It sits between matching's reranking
and the assembled suggestion (spec 0023, steps 6 and 7), and uses
ganit's outputs to do it. It is roadmap Phase 4 item 3.

In short: matching fuses the champion-bounded BM25 and BGE candidate
sets and the reranker scores each selected pair. Ranking keeps only the
clearly relevant ones (a hard gate: popularity or need can never buy
relevance), orders those by need and quality, and offers at most five
per phrase. Final selection returns 3–6 suitable internal destinations
for the source where that many qualify, each with a "likely to be
accepted" figure and an explanation a moderator can read.
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
- Recommendation engine: matching (spec 0023) and ranking together,
  the one path that chooses destination threads or pages for a source
  phrase or passage, as each suggestion does. Any other feature that
  shows destinations as related to a page or passage asks for them
  through that whole path, never `shodhan.rank` alone: the source is
  a guest-visible page, every candidate passes spec 0023's hard
  filters and this spec's relevance gate, no member data is sent, the
  feature keeps the order it gets back and keeps no ranking of its
  own, and a person approves each destination before a reader sees it
  ("The link-equity rules", item 9). Matching people to threads (spec
  0051) and lists not chosen for a source page or passage, such as
  features.md item 28's "most recommended" sections, are not this
  engine (spec 0064, "Link ranking and protection" scope row).
- Scoring model: the champion artefact turning feature scores into one
  final score. Logistic fusion is the initial form (a weighted sum
  passed through a curve from 0 to 1). The eight-head neural predictor,
  LambdaMART and every alternative enter only through spec 0077.
- Confidence: how likely a moderator is to accept the link, with a
  range (ganit job 33).
- Link set: the links chosen for one post: which anchors and which
  destinations, with no overlapping anchors, no repeated destination, a
  link cap and some variety (ganit job 1).
- Degraded mode: the plain fallback when ganit's live service does not
  answer in time.
- Challenger evaluation: spec 0077's controlled comparison and atomic
  champion transition. Ranking has no shadow-activation route of its own.
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

[Spec 0119](0119-cross-site-link-suggestions.md) owns the separate
pair workflow for feature 10. The ordinary candidate and ranker
contracts in this spec stay scoped to one site.

In scope: the graph candidate input; the starting ranking methods; the
final score, confidence, relevance gate and link-equity rules; the 3–6
suitable-destination gate; job 1's link set and degraded mode; bounded
live exploration; playbooks and situations; provisional and complete
rankings; and the explanation, "no suggestions" and review priority.

Out of scope: everything spec 0023 owns, including hybrid BM25/BGE
retrieval, fusion, reranking depth and hard filters (never weights
here); spec 0077's training, Optuna optimisation, evaluation,
activation and rollback; ganit's engines and job code; the review page
and insertion (spec 0028); training-data acquisition (spec 0053);
settings storage (spec 0014); spam detection itself (kavach); reactions
and votes.

### Where ranking sits

Spec 0072 owns Buddhi's full-body method and the evidence behind
kavach's `SpamSignal`. Both use the decision path here. The gate check
also refuses every declaration spec 0072 forbids, for `SpamSignal`, the
spam risk column, kavach's other findings, the fused score, the
calibrated probability, content-hint data, our estimate, search-update
chart observations, the site's Search Console and GA4 series, and any
value computed from them.

For one source post, after spec 0023 has reranked its candidates:

1. Gather evidence for each pair: matching's scores; destination facts
   read by sampark (state, prefix, age, solved answer, author; never
   popularity counts such as views, reactions or replies); darshak's
   human-only rates; kavach's spam signal; spec 0026's typed spaCy
   features; and the active champion outputs of the ganit jobs below.
2. Ask the method registry (spec 0055) which ranking methods apply;
   run the ready ones; mark the rest waiting.
3. Compute the relevance score with the active scoring model and the
   confidence with the active calibration; apply the relevance gate,
   then order the passing destinations by relevance, need and quality.
4. Ask ganit's live service for job 1's link set, within its time
   limit; check its answer against every hard rule, the link-equity
   rules and the suitable-destination count; on time-out, error or a
   broken rule, use degraded mode.
5. Apply the playbook's display threshold and hand the list to
   spec 0023's step 7.

The first useful suggestion is ranked with the methods ready then and
marked provisional. Ranking is redone as each method finishes, and is
complete when every applicable method and every pair in the champion's
bounded reranking set has finished.

### Candidate generation ownership

Spec 0023 is the single hybrid retrieval route. It runs exact BM25, the
admitted active dense channel, and the mandatory BGE-M3 sparse and
multi-vector channels, unions their canonical destination identities and
applies champion RRF and candidate bounds before reranking. The active
dense channel is BGE-M3 by default or the sole paid Bedrock exception
under specs 0025 and 0077. Ranking never runs a second BM25 index, fills
only dense-search shortfalls, or merges a separate list after spec 0023's
cut.

Graph candidates are the only additional input here: the top admitted
link-prediction identities per thread from Jaal's job 256
(Adamic–Adar, resource allocation and Katz). They enter spec 0023's
candidate stream before its bounded reranking selection and cannot
replace a required BM25 or BGE channel. The ranking module holds no
graph algorithm of its own.

Every candidate, whatever its source, must pass spec 0023's hard
filters, have every required title and source representation and
complete-reading receipt, and have an existing-words anchor. Duplicate
canonical destinations count once. Missing graph coverage is reported;
it never exempts a candidate from those rules.

### The starting ranking methods

Each is a catalogue entry (spec 0055) with its sheet, distinguishing
test and lab run, and must pass admission to count.

| Method | What it adds | Evidence |
|---|---|---|
| Reranker relevance | matching's reranker score | reranker |
| Retrieval similarity | matching's fused and per-channel scores | BM25/BGE |
| Lexical match (BM25) | exact words and codes | tantivy |
| Title overlap | phrase words in the title | spec 0026 |
| Entity agreement | same people, products, places | spec 0026 spaCy features |
| Keyphrase agreement | shared statistical keyphrases | spec 0026 |
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
| Position weight | a first-post link weighs more | spec 0026 |

- Destination quality, freshness, need and linkability are order-stage
  methods: they only reorder destinations that passed the gate. Every
  other row is a relevance method, feeding the scoring model, the
  calibrated probability and the gate.
- Reaction rates (the share of Sad and Angry reactions, which count as
  negative; never totals) feed destination quality once reactions are
  supplied; until then that input waits, never zero.
- Graph relevance is always its own line in the explanation.
- A neural prediction head, including spec 0077's eight-head predictor
  or a small head proposed by job 66, is a typed feature produced by an
  admitted packaged route. Rust owns its integration and policy checks;
  no bespoke Rust forward pass or first-party Python adapter is allowed.
  A package route that cannot serve the admitted artefact blocks it.
- PageRank-like ideas are written from published descriptions, with no
  claim of equivalence to Google's systems.

### The final score and confidence

- Each method returns a feature score from 0 to 1 and a direction.
- The final (relevance) score is the active champion scoring model
  applied to the relevance methods' scores only. The initial logistic
  fusion has one positive, bounded weight per method (job 45), and is
  explainable line by line. New weights, architectures and scoring
  methods are immutable challengers under spec 0077; there is no
  separate gradual or 5% activation route.
- A method chain of spec 0068 fuses only the columns its own feature
  table holds. Job 45 proposes weights for each such set of columns,
  and job 33 proposes calibration for each feature set and fusion
  method. Spec 0077 evaluates and promotes the complete artefact;
  spec 0069 retains its shape-specific feature contract. The live
  service fuses every ready relevance method, as above.
- A waiting or failed method has no feature score, so the pair has no
  complete final score. A provisional one, from finished methods only,
  orders the first useful suggestion and is marked so. Nothing missing
  counts as zero.
- The `Ranker` trait applies the active model through its admitted
  interface. LambdaMART, the eight-head neural predictor and every
  alternative require a supported packaged route, spec 0077 promotion
  and the normal owner release approval.
- Confidence: the `Calibrator` trait applies job 33's active champion
  calibration table (Mapan), with its range. Below 200 decisions the
  figure says "estimated from the forum's own links". The ranking
  module computes no statistics itself.
- Every output used records its version and champion manifest digest on
  the suggestion, so a suggestion can always be explained and replayed.

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
   place), and the admitted fetch evidence in spec 0135. No value of
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

A feature passes page facts to ranking only through its owning spec
and an admitted contract under AGENTS.md. A candidate in the tables
has no serving path. A contract tied to an existing job below uses
that job's approved result only. It adds no second input or weight.
These rules hold for every signal:

1. A signal is a catalogue entry (spec 0055), with its sheet, its input
   kind (rate, share, fact or label), its source features, its door and
   its test. It counts only once admitted.
2. A signal is only a rate, a share, a fact or a label. It reorders
   links that already passed the relevance gate, and nothing else. It
   never feeds the scoring model, the calibrated probability or the
   gate.
3. A signal enters by one of five doors, all in the order stage:
   - quality: a rate or fact about how good the target is (job 325's
     floor, shrunk towards the site average for low traffic);
   - need: how overlooked the target is (job 324);
   - freshness: how current the target is (job 222);
   - trust: a caution that only lowers a target's place, or freezes its
     traffic inputs at their last good values. It never raises one.
     The existing caution methods (prefix, spam risk, self-promotion)
     stay as they are; a trust signal acts in the order key only.
   - verified AI fetch: one qualifying fact for an exact public revision
     under [spec 0135](0135-ai-fetches-referrals-and-ranking.md).
     DR-122 admits a bounded order adjustment, with no count or rate.
     Use the exact 0135/DR-153 batch ceiling and retain ordinary
     eligibility and final selection when an adjustment is withheld.
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
7. A report, proposal, draft or research measure can have "signal:
   none". Keep its outputs out of the order, gate and ranking
   feature store. Existing eligible facts owned elsewhere keep
   their own route. Reusing a rank result for a report or draft
   does not allow that output to feed back into ranking.

Every signal is held by two shared tests:

- RS00 (`tests/props_signals.rs`, property; Kani proof of the order
  key): for every admitted signal, give each destination below the gate
  the best possible value and each passing one the worst. The set that
  passes the gate is unchanged, and every destination below it is
  refused with SUT-550.
- RS-REG (`tests/signal_registry.rs`): every signal entry names its
  source owner, input kind, door, fixture and admission record.
  A candidate or excluded id cannot register through config,
  optimiser, script, restore or an alias. A general popularity
  count, missing proof or a door outside the five fails. RS13 and
  RS14 enter through job 324. Other admitted structural inputs
  retain their own operators, including job 49. Refuse an extra
  weight for an existing input and check "signal: none" groups
  cannot write to the feature store.

Each row's own test, below its table, uses one fixture: an off-topic
target (below the gate) beside on-topic targets that pass.

#### From the new features (Range 4)

| ID and fixture | Owning spec and feature | Proposed input | Door |
|---|---|---|---|
| RS01 | 0044, 1314 | Failed mobile check (fact), lower only | trust |
| RS02 | 0044, 1320 | Broken head HTML (fact) | trust |
| RS03 | 0044, 1322 | Target differs from section topics (share) | trust |
| RS04 | 0044, 1327 | Date of last real rewrite (fact) | freshness |
| RS05 | 0044, 1328 via 0034 | Admitted human rate from log evidence | quality |
| RS06 | 0044, 1329 | Missing counter (fact), traffic input unavailable | trust |
| RS07 | 0044, 1333 | Crawled but inactive section share | need |
| RS08 | 0034, 1341 | Assisted-registration rate with shrinkage | quality |
| RS09 | 0034, 1344 | Leave-at-once rate | quality |
| RS10 | 0034, 1354 | Play rate on media targets | quality |
| RS11 | 0034, 1355 | Trait correlation report | No signal |
| RS12 | 0034, 1346 | Owner's report grouping of addresses, sources or referrers | No signal |

Rows RS01 to RS10 are candidates. Each numbered fixture below is
required after its own source contract has been approved. RS05 also
needs the log consent and provenance admission of specs 0034 and
0044. Existing human quality rates still come through job 325.

- RS01: the off-topic target passes every mobile check and is still
  refused; an on-topic target that fails only moves down.
- RS02: a clean head on the off-topic target does not let it pass; a
  broken head only lowers an on-topic target.
- RS03: the off-topic share can only lower; an off-topic target with a
  share of zero is still refused.
- RS04: a rewrite dated today on the off-topic target leaves it
  refused.
- RS05: a joined log row changes neither the human denominator
  nor the gate. An unjoined or bot row cannot supply a human rate.
  Any later human source needs separate proof under specs 0034
  and 0044 (test 38).
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
- RS11 and RS12: change report traits and groups. Check no index,
  ranking input or order change. A separate per-target proposal needs
  an owning-spec amendment and admission before it can do more.

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

| ID and fixture | Owning spec | Input and boundary | Route |
|---|---|---|---|
| RS13 | 0044 | Inbound links and click depth | Job 324 need input |
| RS14 | 0044, own-site figures from 0040 | Impressions at positions 8 to 20 | Job 324 need input |
| RS15 | 0044 | Own-site pages competing for the same query (fact) | Candidate trust |
| RS16 | 0041, F-0027 | Answer completeness rate is a candidate source only. QR-104 governs the editorial assessment. A separate ranking admission must establish the exact eligible rate, source and proof. | Held before candidate quality admission |
| RS17 | 0049 | Hub coverage share | Candidate need |
| RS18 | 0034 | Link satisfaction and exit rates in an admitted human population | Candidate quality |
| RS19 | 0043 | Evergreen label | Candidate freshness |
| RS20 | 0034 | Job 176 flag under the traffic freeze rule only | Existing freeze path |
| RS21 | 0041, 92 | Target version fact | Candidate freshness |
| RS22 | 0041, 93 | Target expiry fact, with no duplicate freshness input | Candidate trust |
| RS31 stock boundary | 0103, 580 | Out-of-stock or unconfirmed stock uses the owned eligibility rule | No stock signal |
| RS23 | 0034 | Bot share under the traffic freeze rule only | Existing freeze path |
| RS24 | 0040 | The same own-site position band already used under RS14 | No extra input |
| RS25 | 0042, 235 | Editor score describes a draft only | No signal |
| RS26 | 0047, 45 and 330 | Slow template (fact) | Candidate trust |
| RS27 | 0101 | Admitted disclosure fact only | Candidate trust |
| RS28 | 0034 | Human engagement rate already owned by job 325. No direct GA4 or audience input | No extra input |
| RS29 | 0084 | Best-source order is internal to the source pack | No signal |
| RS30 | 0072, produced by 0104 | Optional `script_quality_rate` for one destination under RM06 | Quality only if admitted and on |

Use a candidate only after the named source has an approved contract
and its behaviour is admitted. Spec 0055 binds the input, scope,
consumer, tests and current evidence. The rows above do not approve
a candidate. Stock eligibility, RS25 and RS29 have no ranking
feature-store key or fallback. RS24 and RS28 cannot register a
second weight or a second source.

- RS13, RS14 and RS17: the highest need leaves the off-topic target
  refused. Need falls with inbound links (test 26). RS24 cannot
  change job 324's inputs or weight.
- RS15, RS20, RS22, RS23, RS26 and RS27: only lower or freeze where the
  owning rule allows it. The off-topic result stays refused.
  RS20 and RS23 freeze only admitted traffic rates under spec 0034.
  Test a missing last good rate, no direct demotion and clean-day
  return. Non-traffic inputs stay current.
- RS16: reuse RS-REG to check that the held F-0027 assessment, an
  advice-only outcome and a restored old rate register no candidate
  input. If a producer is separately admitted, RS00 applies to a rate
  of 1.0 on the off-topic target. It stays refused as its view count
  rises (test 25).
- RS18: a rate of 1.0 leaves the off-topic target refused. Raising
  its view count changes nothing (test 25). RS28 cannot add GA4 or
  alter job 325's human rate or weight.
- RS19 and RS21: the freshest value leaves the target refused.
- RS31 stock: check spec 0103 eligibility with no separate score
  or trust write. RS25 and RS29: save an editor score and reorder a
  source pack. Check candidate set, gate, scores, stored inputs and
  order. Replaying either output or a restored alias changes none.
- RS30: RM06 covers the switch, scope, missing result and fences
  for the admitted champion. RS00 still applies.

Into hard filters only (rule 5), which can only refuse: Technical
health (canonical, soft 404); Site crawler and technical audit
(indexable, canonical, status); International and multi-site and
Localisation (British-English eligibility under DR-081); Distribution
and syndication (canonical);
More technical checks (blocked, duplicate); safety and policy
guardrails (the independently owned blocklist). Current public
visibility and canonical facts stay with spec 0023. Draft quality
receipts, variant plans and hand-offs cannot write that index or
change the filter.

Data for existing relevance methods (rule 6), under their own entries:
Linking and site structure (anchor guard, into the equity caps);
Structure and linking (query-aligned anchors, into anchor quality;
density guard, into the caps); Site crawler (link reports, into graph
relevance); Brand entity and knowledge panel (entities, into entity
agreement); Measurement and learning (rejection reasons, into job 97);
More analytics, workflow, crawl and press features (content similarity,
into job 1's variety term, which only spreads links). Planning and
outlining may propose source-pack citations.
Confirmed publication under spec 0090 feature 1255 can create its
bounded local review request through existing approvals. Neither
output writes to the live index, candidate set or order inputs.

Research ideas without a serving route: search intent labels, query
clusters and content type calibration. Specs 0084 to 0098 supply no
ranking output as written. A new proposal requires an owning-spec
amendment and independent admission. An opportunity score, live rival
or AI-visibility measure, experiment result or draft stays outside
ranking.

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

#### F-0005 verification binding

`REQ-0027-ANCHOR-SHARE` uses `DES-0027-ANCHOR-EQUITY` in
link-equity rule 2. The shared `equity::check` evaluates the per-target
share of each exact anchor text in placed and waiting links when the
rule's four-link threshold applies. Use the approved value of
`linking.equity.anchor_max_share` under the existing 0014 registry. The
selector drops a refused choice with SUT-551. At approval, 0028
rechecks the current inventory through the same function. A refusal
leaves the suggestion waiting and contributes no new site effect.

- `VAR-0027-ANCHOR-SELECT` checks ordinary and degraded candidate
  selections with their existing eligibility and reason paths.
- `VAR-0027-ANCHOR-REVIEW` checks a current moderator decision in
  0028, with the admitted snapshot and pending-approval fences.

Both use the exact site, canonical destination and native anchor
representation of the admitted profile. No conferral of approval or
change to relevance, the link graph or the public-page budget follows
from the check. A separate authorised 0028 decision owns a write.

`UT-0027-ANCHOR-SHARE` binds 0027 test 29. With other equity rules
satisfied, vary prospective exact-anchor share below, at and above the
admitted maximum. At the maximum, rule 2 allows equality. Above it,
check SUT-551. Preserve the existing nine-link example.

`IT-0027-ANCHOR-RECHECK` binds the anchor-share part of 0028 test 10.
In a synthetic fixture at the stated 0.3 rule value, begin with nine
eligible links, two with the same anchor. Two competing proposals for
that anchor would individually give 3/10 and together give 4/11.
Keep all other caps satisfied. Check that approval on current state
allows at most one new effect and leaves the other suggestion waiting.

Pin the exact four-link activation boundary at prospective versus
already-counted state. Define the projected population, exact text
representation and same-target concurrency receipt before claiming
admission. Qualification uses the existing 0014, 0027 and 0028
contracts. This record supplies no new threshold or platform variant.

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

### The 3–6 suitable-destination gate

The final recommendation set for one source article or post, across
all of its anchors, contains 3–6 unique internal destination threads or
pages. This is a whole-source count, not five per anchor multiplied
into an unbounded post total.

1. When at least three destinations pass complete reading, permissions,
   hard filters, anchors, relevance, display and equity checks, the
   selector returns at least three and no more than six, ranked by the
   active champion and its diversity rule.
2. When more than six qualify, only six are returned. When fewer than
   three qualify, only the qualifying zero, one or two are returned and
   the shortfall is shown explicitly. The gate never lowers a threshold,
   repeats a destination or adds an unsuitable one to reach three.
3. A thread allowance or another hard cap may leave fewer than three.
   That is a visible unmet-minimum outcome, not a passed 3–6 gate and
   not permission to bypass the cap.
4. The count constrains recommendations, not moderator behaviour. Every
   destination still needs separate human approval, and no moderator is
   required to approve three. Version 1 never inserts links automatically.
5. Job 1, degraded mode, exploration, playbooks and diversity all use
   this same count check. None has a separate destination-count route.

### Live exploration

- To keep learning which destinations moderators value, a small share
  of alternatives' order is drawn by Thompson sampling (`rand_distr`)
  from job 95's posteriors per site, target type and anchor type. The
  share comes from the active champion configuration. Job 149 supplies
  offline evidence only; spec 0077 owns every change. The initial share
  is off in Balanced and 5% in Aggressive only after that promotion.
- Exploration only reorders destinations that passed every rule and
  threshold; it never shows a weaker link than the playbook allows,
  never changes the 3–6 count and every link still needs approval.

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

Cover every applicable playbook requirement and declared supported
variant in the spec 0076 test matrix. Each situation has a real
difference and an executable expected outcome.

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
- The runner prints the distinct trace count for coverage reporting.
  Spec 0064 requires the outcome of every applicable variant on every
  required target. Its trusted verifier applies the protected policy
  bound to the exact candidate and run.

### Explanation, "no suggestions" and priority

- The explanation lists each method's score, weight and reason, each
  feature's importance (job 46), graph relevance on its own line, the
  PageRank gain if accepted (job 259), the confidence and range, and
  every version used. Ids and numbers only. The spam risk method's
  reason codes by family appear only to holders of "View content
  hints" (spec 0072); other reviewers see its score, weight and
  importance and that a spam caution applied.
- "Why no suggestions?" lists matching's filter reasons, no acceptable
  anchor, below the display threshold, or methods waiting.
- Review priority comes from job 4 (the queue order giving the most
  accepted links per moderator minute); spec 0028 uses it.
- On the page, confidence shows as a percentage with a range, such as
  "Likely accept 78% (70 to 85%)".

### Batch link-value simulator (feature 1)

Preview how a proposed batch changes the site's model of internal
link value. The map and table show pages that gain, lose or have
no detectable change. Values describe the frozen graph and its
model. They predict no search position, visits or acceptance.

The owner selects a complete admitted active-site graph and
a batch of proposed link changes. Use the current admitted same-site
0021/0022/0049 projection.
Freeze the native eligible page universe, exact revisions, roles,
visibility generation, normalised graph profile and source receipts.
Only guest-visible indexable pages and admitted internal edges
qualify. Separate proved exclusions from unknown pages or edges.
Any unaccounted source, target or projection makes the whole-site
graph incomplete. Withhold its values and names. A stale native
projection cannot be replaced by assumed visibility. Read locally.
No new crawl or external request.

| Record | Required fields and types |
|---|---|
| Graph snapshot | Site ID, graph and source digests, profile version, visibility generation, completeness flag, ordered native page and revision IDs, directed edges from/to and positive admitted weight |
| Batch plan | Site, actor and grant generation, snapshot digest, ordered edits, optional alternative batch, created UTC and expiry |
| Edit | Stable edit ID, native source/version, target page/revision, typed add/remove or replacement, existing or proposed occurrence and graph-projector receipt. The projected pair records expected and new full weight or absence |
| Job receipt | Plan digest, pinned 251 profile and numerical bounds, state, input and output digests, attempts, used resources and erasure epoch |
| Result row | Native page reference, baseline and scenario mass, difference interval, gain/loss/indeterminate, explanation IDs and profile version |

A batch has 1–100 exact native occurrence edits. Validate source
and target revisions and existence against the frozen base. Refuse
self, foreign, private or unknown endpoints and repeated or
conflicting occurrence edits. A move is a removal and addition.
Order by stable source, target and edit ID. Fold distinct native
occurrences for one directed pair through its pinned graph projector.
It alone provides add, remove or new full pair weight. Removing
one occurrence may keep the pair. No arbitrary agent weight.
A projected no-op says “No change in the model”. A hypothesis
cannot be applied. Show why its native review is unavailable if
any existing eligibility check fails. No inherited approval UI.

Use the Jaal 251 calculation through 0066's separately admitted
advisory entry. Never call its candidate job publisher. Run the
base and edited graph through one pinned normalisation, damping,
dangling rule, teleport, precision and stop profile. Use a
disjoint protected store, result type and manifest. Exclude any
0077 activation, candidate or ranking consumer. The original
[PageRank description][batch-pagerank] is a reference for the
metric. Job 259's single-link estimate is a different interface.
Never add those estimates to model a batch.

For a pinned stochastic normalisation and 0 ≤ d < 1, bound
the exact L1 residual ||T(x) − x|| with conservative arithmetic.
Each vector error bound is (computed residual plus its error
allowance) divided by (1 − upper damping bound), plus any
qualified representation allowance. Require upper damping < 1.
Keep both bounds. Compute scenario minus base mass and expand
its interval by both bounds and subtraction roundoff. Round
intervals outwards. Gain needs lower bound > 0, loss upper
bound < 0. A proved zero is unchanged. An interval containing
zero is otherwise indeterminate. Express mass in percentage
points. Label a small positive or negative value rather than
display a proved direction as zero. No publish if convergence
or numerical proof fails. An alternative batch uses the same
base and profile. Show a direction reversal only if both
intervals prove opposite signs. Otherwise show uncertainty.

The accessible map shows the selected scenario's edit endpoints
first, then current eligible nodes by absolute delta at admitted
precision, then stable ID. Stop at 200 nodes and 400 edges.
Take edited edges first, then stable endpoint order. State
omitted scope. Search and keyboard table use the same result
generation and actor/site/scope-bound cursor. Return at most
100 rows per page. In each of gains and losses, sort by
decreasing absolute delta at displayed precision, then stable ID.
Sort indeterminate rows by ID. Selection shows
checked source and numerical bounds. Feature 613 receives only
a checked owner-local summary. No raw graph, native IDs or
link labels pass to any wording route or agent.

Use the registered 0019 advisory class and 0078 lease. Across the
base, scenario, alternative and serialisation, candidate ceilings
are 50,000 nodes, 500,000 edges per graph, 256 MiB RAM, 128 MiB
encrypted scratch, 16 MiB output, 120 active seconds and ten
elapsed minutes. One job per site. Stricter 251 bounds apply.
The logical key binds site, actor, purpose, exact graph and
model digest and canonical edit sets. Replay only its current
eligible receipt. Register files under 0030 and 0118 before use.
Check authority and visibility at claim and each checkpoint.
Atomically order publication against current scope, native
revision and erasure generations. Repeat those checks at each
view or summary. Pause, expiry or failed proof closes the run.
Clear scratch at exit or recovery before reuse. Retain sealed
results for 24 hours at most. Reject old restores under 0032.

No live rank, approved link, website state or workflow changes.
The review page can accept a separately prepared effect only
through the ordinary 0020, 0021 and 0028 gate. Simulated maps,
deltas and artefacts cannot become ranking inputs, fitness,
training records or promotion evidence. Zero public-page
queries, time or bytes.

| Requirement | Acceptance |
|---|---|
| SIM-01 | UT-SIM-01 checks exact synthetic graphs and uncertainty. For A→B→C→A with d=1/2 and uniform teleport and dangling mass redistribution, removing C→A gives base 1/3 each and scenario 4/17, 6/17, 7/17. Prove numeric bounds include all three deltas and preserve the gain/loss labels |
| SIM-02 | IT-SIM-02 tests simultaneous add/remove interactions, multiple underlying occurrences, wrong site and private endpoint, bounds at and one over, incomplete coverage, empty input and nonconvergence. No simulated rows on rejection |
| SIM-03 | BDD-SIM-03 shows map and keyboard table from the same frozen rows, ties, no-detectable-change, sensitivity with one alternative, stale source, 0042 read and exact explanation. No edits or rank change |
| SIM-04 | FI-SIM-04 races pause, withdrawal, worker crash and restore with publication. Replay the protected receipt. Erase abandoned scratch and reject a ranking consumer or effect template |
| SIM-05 | E2E-SIM-05 uses synthetic graphs only. Qualify timing, peak RAM, numerical agreement and protected isolation on Ubuntu 26.04, Debian 13 and Windows 11 under Bazel. Larger or unproved profiles show unavailable |

#### F-0001 verification binding

Use the same feature, behaviour and five obligations defined above.
The original SIM and test labels remain source locators. The stable
identities below refer to the same obligations and give no new test
or feature count.

| Requirement and design | Existing obligation and gate-facing case identity |
|---|---|
| `REQ-0027-SIM-NUMERIC`, `DES-0027-SIM-MODEL` | SIM-01 and UT-SIM-01 map to `UT-0027-SIM-NUMERIC`. Prove exact interval inclusion and sign for the three-node fixture. Compare strict positivity, strict negativity, equality and overlap with zero. |
| `REQ-0027-SIM-PLAN`, `DES-0027-SIM-INPUT` | SIM-02 and IT-SIM-02 map to `IT-0027-SIM-PLAN`. Exercise one and 100 edits, zero and 101, conflicting or repeated edits, whole-graph completeness, endpoint scope and the occurrence that leaves its pair in the graph. |
| `REQ-0027-SIM-VIEW`, `DES-0027-SIM-VIEW` | SIM-03 and BDD-SIM-03 map to `BDD-0027-SIM-VIEW`. Bind the map, keyboard table and feature 613 summary to the exact current result generation. Vary ties, omission, one alternative and stale source. |
| `REQ-0027-SIM-FENCE`, `DES-0027-SIM-RUN` | SIM-04 and FI-SIM-04 map to `IT-0027-SIM-RECOVERY`. Plant a crash on both sides of result commit, then try pause, visibility withdrawal, replay, erasure and stale restore. No native effect or ranking consumer. |
| `REQ-0027-SIM-PROFILE`, `DES-0027-SIM-PROFILE` | SIM-05 and E2E-SIM-05 map to `E2E-0027-SIM-PROFILE`. Qualify the admitted host and source combination at measured limits and with one excess or unproved input. |

| Variant | Applicability |
|---|---|
| `VAR-0027-SIM-UBUNTU` | Ubuntu 26.04 LTS x86-64 with an admitted native graph projection and the five named tests. |
| `VAR-0027-SIM-DEBIAN` | Debian 13 x86-64 with an independently measured complete profile and the same tests. |
| `VAR-0027-SIM-WINDOWS` | Windows 11 x86-64 with independent protection and measurement proof and the same tests. |

Bind each host independently to the exact admitted XenForo 2.3 or
WordPress graph and occurrence projector. Unsupported or unproved
source and host combinations show the existing unavailable result.
`UI-0027-SIM-PREVIEW` owns the local map and table. Use
`API-0027-SIM-ADVISORY` for the internal typed 251 handoff and
`DB-0027-SIM-CUSTODY` for the disjoint site store. These identities
grant no remote interface, approval or ranking action.

`SRC-0027-SIM-PAGERANK` refers to the published description below.
The product's exact batch edit projection and bounded preview
are original design. QR-153 owns qualification of the complete
source graph, native occurrence projection, numerical profile,
resource bounds and isolation. Freeze the exact schema and
redacted errors, prove custody and replay, and bind measured tests
before deriving an implementation task. The fixture damping is
independent of the eventual admitted production profile.

[batch-pagerank]: https://snap.stanford.edu/class/cs224w-readings/Brin98Anatomy.pdf

### Wave 1 structural and target evidence bindings

These bindings use existing same-site matching, relevance, hard
eligibility and link-equity rules. Recheck current native identity,
visibility, source revision and authority before review or any
separately admitted effect. No proposed binding changes membership,
weights, native navigation or a site's public footprint.

| Feature, requirement and design | Variant, fixture and planned result |
|---|---|
| F-0004 · REQ-0027-W1-CLICK-DEPTH · DES-0027-W1-DEPTH-ADVICE | VAR-0027-W1-OBSERVED-DEPTH, FX-0027-W1-CLICK-DEPTH, BDD-0027-W1-CLICK-DEPTH. Use the declared own-site home and eligible navigation graph under 0044 job 253 and 0047. Compare equally eligible pages with proved distances of three and four clicks. The original boundary is strictly more than three clicks. A missing edge or incomplete shorter-path coverage makes the affected depth unknown. Hold a placement if the page's value or source is unproved. A hub or link placement remains a reviewable proposal and changes no navigation. |
| F-0006 · REQ-0027-W1-MOVED-TARGET · DES-0027-W1-STALE-DESTINATION | VAR-0027-W1-MOVED-LINK-HOLD, FX-0027-W1-MOVED-TARGET, IT-0027-W1-MOVED-TARGET. Resolve the canonical current destination before offering a new candidate. Merge the former destination after approval and replay its effect ID, then compare a later source edit. Hold the pending old target as stale under 0028; an already-committed replay can only reconcile its own recorded effect. Retargeting an existing occurrence needs a separately admitted 0021/0028 replacement type, new effect and exact human approval. Preserve unrelated bytes. QR-103 retains this effect gate. |
| F-0007 · REQ-0027-W1-ANSWERING-POST · DES-0027-W1-ANSWER-IDENTITY | VAR-0027-W1-EXACT-ANSWER-POST, FX-0027-W1-ANSWERING-POST, IT-0027-W1-ANSWERING-POST. Use the exact approved native post identity and permalink within its current canonical guest-visible thread. Keep the thread's deduplication and equity identity. Hide the answering post between selection and approval, then check the same boundary before effect. No approval or application for the hidden, moved or unproved target. Hold when the native profile cannot prove the exact post and its revision; do not insert a guessed fragment or silently replace it with the thread link. |
| F-0008 · REQ-0027-W1-SEASONAL-INPUT · DES-0027-W1-SEASONAL-HOLD | VAR-0027-W1-SEASONAL-INPUT-HOLD, FX-0027-W1-SEASONAL-INPUT, IT-0027-W1-SEASONAL-INPUT. Compare repeated demand, a one-off spike and a history gap on the same eligible 0040 property, time and source profile. Include an off-topic target. Without an independently admitted seasonal history, coverage, estimator, validation and rank-signal authority, produce no seasonal order change. A 0094 forecast or existing freshness fact supplies no rank weight. A future admitted signal would still follow relevance, per-target evidence and 0077 champion admission. |
| F-0009 · REQ-0027-W1-GLOSSARY-TARGET · DES-0027-W1-GLOSSARY-PUBLICATION | VAR-0027-W1-PUBLISHED-DEFINITION, FX-0027-W1-GLOSSARY-TARGET, BDD-0027-W1-GLOSSARY-TARGET. Compare repeated term uses, ambiguous senses and revoked sources in an eligible local 0084/0090 definition proposal. List a glossary target only after separate native publication gives a current canonical, indexable, guest-visible same-site definition of the exact sense. Only relevant uses produce link suggestions through normal review. A draft or expired source gives no target. QR-103 retains publication and placement proof. |

The FX records bind the planned Wave 1 scenarios. Run their positive
conditions only on the owning admitted profiles. Replay and
missing-proof cases carry no effect, publication, model admission
or claim of executed verification.

### Ganit jobs used

Live, in ganit's live service: job 1 (the best link set per post).

The Rust scheduler under spec 0019 runs the typed batch jobs.
Their outputs are challenger evidence or immutable candidate inputs for
spec 0077, never self-activating configuration:

- Chayan: 3 (display threshold), 4 (review-queue order), 7 (anchor
  variety targets).
- Mapan: 33 (calibrated acceptance probability), 34 (conformal count of
  alternatives per anchor, never above five, with the whole-source 3–6
  gate unchanged), 35 (per-site reranker threshold).
- Shiksha: 45 (scoring model), 46 (feature importance), 47 and 48
  (link density and same-target caps, inside the allowance), 49
  (diminishing returns), 51 (anchor quality), 52 (linkability), 66
  (thread type), 68 (likely solution).
- Anuman: 94 (acceptance priors), 95 (exploration posteriors), 97
  (targets that attract rejections), 109 (outdated solved answers).
- Samanjan: 115 (BM25 k1 and b and other retrieval settings inside
  spec 0077's registered bounds; reranking depth is champion-owned).
- Mulyankan: 142 (threshold-slider surfaces).
- Vikalp: 147 (new ranking policies scored offline), 149 (exploration
  rates scored offline).
- Bahav: 192 (score shift after a change, triggering recalibration).
- Gati: 222 (attention decay, for freshness).
- Jaal: 251 (internal PageRank), 253 (orphans and click depth), 256
  (link-prediction scores and candidates), 258 (pairs of threads that
  link only to each other, as spam-pattern flags), 259 (PageRank gain
  if accepted).
- Varnakram: 276 (spectral graph embeddings as features), 278 (topic
  communities, a part of graph relevance).
- Sannidhya: 286 (near-duplicate clusters and canonical threads).
- Padbandh: 296 (generic anchors, joining matching's stop list).
- Order stage: 324 (need) and 325 (quality floor).

Matching takes job 2 for its re-score margin, job 136 for when to
re-score and job 280 for hubness correction. These are also candidate
inputs under spec 0077, not independent live tuning.

### Champion outputs

- `champion_outputs` is one read-side module that loads the exact output
  versions named by the active spec 0077 manifest and verifies their
  digests, schemas and compatibility. It cannot shadow-test, activate,
  tune or roll back an output.
- A proposed output with NaN, an out-of-bound weight or worse evaluation
  evidence is refused by spec 0077 (surfaced here as SUT-544); the
  active champion stays unchanged.
- An overdue output: the last promoted one stays active, the explanation
  says how old it is, and an automatic issue is filed (SUT-543).
- Rollback is solely spec 0077's verified atomic transition and receipt.
- An output made from an incompatible snapshot cannot be loaded into a
  new champion; the complete candidate must be re-evaluated.
- Each post is ranked wholly with one champion manifest and one version
  of each output, even if an atomic transition lands mid-ranking.

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
`settings_version`, `champion_manifest_digest`, and the versions of
every ganit output used.

Spec 0077 owns candidate manifests, evaluation evidence and transition
state. Ranking stores references to them, not a second mutable
`ganit_output` activation table or payload store.

Uninstall: these rows follow matching's suggestion records (export
offer and keep-or-delete choice).

### Interfaces

Management API routes on the local socket (Axum, described with
utoipa), used by the desktop app and `sutra`:

- `GET /api/suggestions/{id}/explanation`: the explanation.
- `GET /api/posts/{id}/no-suggestions`: "Why no suggestions?".
- `GET /api/ranking/threshold-preview?min_confidence=<x>`: answered
  from job 142's surfaces where available, else counted from stored
  results.
- `GET /api/ranking/playbook` and the action `rank.playbook.choose`
  (Balanced or Aggressive), writing a spec 0014 version.
- `GET /api/ranking/champion`: the active manifest identity and spec
  0077 transition receipts. There is no ranking activation or rollback
  action.

The site's review page gets explanations and confidence through the
signed site link (spec 0021), as ids and numbers.

Calls to ganit: job 1 through its live service's versioned API, with
ids and numbers only (never text), the trace context and a deadline.
Spec 0019 inserts each scheduled fire and its typed job in the same
transaction. Spec 0077 owns the protected ranking workflows.

Events (sanchar, through spec 0019's transactional outbox), ids and
codes only:
`ranking.updated`, `ranking.method_failed` and `ranking.degraded`
(job 1 missed; count per minute). Save local effects and their
events together. Retry delivery without changing the ranking twice.
Champion transition events belong to spec 0077.

Command line: `sutra rank explain <suggestion>`, `sutra rank playbook`
and `sutra rank champion`, through the same read or settings actions.

### Code layout

- shodhan's `rank` module: the orchestrator, `Ranker`, `Calibrator`,
  the job 1 client and degraded mode, exploration, playbook thresholds
  and the explanation; each method under `rank::methods`; the graph
  input adapter under `shodhan::generate`.
- Traits with test fakes: `GraphCandidateProvider`, `Ranker`,
  `Calibrator`, `LinkSetSolver` (a fake that answers, delays or fails
  on command), `SpamSignal`, `DestinationFacts` and `ChampionOutputs`.
- Ranking runs on the Rust worker's rayon pool over an immutable typed
  feature and score snapshot. It neither owns model vectors nor hosts a
  bespoke neural forward pass or Python runtime adapter.

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
- SUT-543: a champion ganit output is overdue (above).
- SUT-544: spec 0077 refused a proposed output's evaluation or
  numerical guards; ranking never activates it locally.
- SUT-545: the graph candidate input failed; its missing coverage is
  shown and spec 0023's required hybrid channels still run.
- SUT-546: a weight outside its bound in a stored or proposed output is
  refused; the previous promoted champion stays.
- SUT-547: the ranking work budget for a post is exceeded: retried
  once in backfill, then reported.
- SUT-548: two situations with the same trace and outcome.
- SUT-549: job 1 timed out or failed: degraded mode was used; the
  suggestion says so and is asked again later.
- SUT-550 to SUT-558: the gate and link-equity refusals above.

A crash during ranking keeps finished feature scores; at most the pair
being scored repeats. If ganit's live service is down, suggestions keep
flowing in degraded mode; nothing waits on nightly jobs. A shortfall
below three suitable destinations is visible and never bypasses a hard
rule.

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
  `..._job1_seconds`, `..._champion_output_age_seconds` by job, hybrid
  channel coverage from spec 0023, graph-input hit counts and the final
  suitable-destination count.
- A trace span per post's ranking, with a child span for the job 1
  call carried into ganit.
- Health page: failing methods, overdue outputs, degraded rate and
  unmet destination minimums.
- Alerts: time to complete burning its target; degraded mode above 1%
  for an hour; an output overdue by two schedules.

## Limits

Speed, on Dell under fixed conditions, imitating the reference
hardware of 4 cores and a SATA solid-state disk where stated:

- New posts keep their 30-second first suggestion: ranking adds at
  most 20 ms for the champion's complete reranking set at the 95th
  percentile, excluding model time; job 1 answers within 150 ms at the
  95th percentile and is abandoned for degraded mode at 300 ms.
- Degraded mode on at most 1% of posts in a day on a healthy system
  (an alert, not a gate).
- 99% of new posts are approvable within 10 minutes of saving, on the
  reference hardware.
- The "why" panel opens within 300 ms; threshold previews within 1
  second over 100,000 suggestions waiting for review.
- Hybrid retrieval and fusion obey spec 0023's measured limits. The
  graph candidate table read adds at most 5 ms per post.
- Champion evaluation and activation follow spec 0077's controlled
  schedule and gates; ranking has no separate ten-minute activation
  promise.

Memory stays inside the service slice and AGENTS.md section 5.6, with
15% headroom. These are admission ceilings, not estimates or proof that
a workload fits. Missing measurement blocks the affected profile:

- Inference worker: at most 64 MB per post in flight; two posts at
  once (one live, one backfill).
- Active champion outputs held for ranking (weights, calibration,
  priors and graph scores): at most 64 MB on a PC under 16 GB and
  150 MB otherwise.
- BM25 and BGE indexes are accounted once under specs 0022 and 0023.
  Ranking reserves no second lexical index.
- ganit's live service and its outputs count in spec 0024's standing
  budgets. Each registered batch job needs a measured, non-overlapping
  spec 0019 resource profile before admission; no fixed concurrent job
  or memory fit is assumed.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

All tests use made-up data and fakes unless they say Dell. Paths are
relative to `crates/shodhan/`. Test-first order: scoring with a fixed
output; missing and failed methods; `champion_outputs` and the atomic
transition test; the job 1 check and degraded mode with the Kani proof;
hybrid-input and graph contracts; the starting methods, each with its
distinguishing test; the 3–6 gate; exploration, playbooks and
situations; explanation; then speed, load, Dell acceptance, ganit's own
tests, regressions and packaged neural-head parity.

Only components listed in ADR-0020's strict core require its 100% line,
branch and MC/DC coverage and 100% killable-mutant rule, with proved and
independently reviewed equivalent-mutant exclusions. Other components
follow their risk-appropriate coverage contract. Property tests and
Kani proofs still show that an accepted link set never has overlapping
anchors, never repeats a destination, never exceeds five per anchor and
obeys the whole-source 3–6 rule without filling from unsuitable
candidates (`proofs/linkset_check.rs`, `proofs/equity.rs`).

Facts to measure or confirm: spec 0023's hybrid indexes meet their
resource and recall contracts; job 1 meets 150 ms at the 95th
percentile on 4 cores for a post with 20 anchors and the initial
champion's 50 destinations each; every memory ceiling; what XenForo 2.3
stores for prefixes, solved answers and views, read through sampark;
job 45's fit keeps every weight positive; SUT-540 to SUT-558 are owned by
this spec (spec 0028 cites SUT-550 to SUT-558 and owns SUT-570 to
SUT-581); how noindex state is read and kept current; and the equity
defaults, calibrated by link reconstruction before release.

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
   as each finishes and is complete only after the last method and every
   pair in the champion's reranking set. (`tests/rank_progressive.rs`)
5. Any job 1 answer, hostile ones included (overlapping anchors, a
   repeated destination, six alternatives for one anchor, a filtered
   destination), offers nothing that breaks a hard rule.
   (`tests/props_linkset.rs`, property; Kani proof)
6. With the live service delayed past 300 ms or failing, degraded mode
   offers at most five per anchor that pass the gate and every equity
   rule, by order key, records SUT-549, and job 1 is asked again.
   (`tests/degraded.rs`, with the fake solver)
7. A graph-only candidate missing a required representation,
   complete-reading receipt or eligible anchor never reaches a
   suggestion. (`tests/generate_filters.rs`)
8. BM25 and BGE channels return overlapping and disjoint destinations:
   spec 0023 deduplicates and fuses them under champion bounds, while
   ranking adds no second lexical list or post-cut merge.
   (`tests/generate_join.rs`)
9. One fixture under Balanced and Aggressive: hard-rule outcomes, gate
   results and approval path are the same; Aggressive offers at least
   as many, all past the gate and within the 3–6 whole-source maximum.
   (`tests/playbooks.rs`; situations)
10. A proposed output with NaN, an out-of-bound weight or worse
    evaluation evidence receives SUT-544 and no local activation; the
    active champion stays. (`tests/champion_outputs.rs`; property over
    generated outputs)
11. A verified spec 0077 rollback receipt restores the previous complete
    champion for the next post; ranking exposes no rollback action of
    its own. (`tests/champion_outputs.rs`)
12. An overdue job: the last promoted output is used, its age shown,
    SUT-543 filed. (`tests/champion_outputs.rs`)
13. An explanation lists every applicable method with score, weight
    and reason, graph relevance on its own line, and no post text.
    (`tests/explain.rs`, snapshot with insta)
14. A promoted test champion configured for 10% exploration over 10,000
    seeded posts changes about 10% of orders and never shows a
    destination below the threshold. (`tests/explore.rs`, property)
15. Every situation file's expected outcome holds and no two share a
    trace and outcome. (`tests/situations.rs`)
16. With the ranker shared by live and backfill, loom explores an atomic
    champion transition during ranking: each post uses one complete
    manifest and one version of each output. (`tests/loom_outputs.rs`)
17. The initial champion's 50 pairs for one post on Dell's bench cores:
    ranking takes at most 20 ms at the 95th percentile, and its median
    of at least 30 runs is no more than 5% slower than its baseline.
    (`benches/rank.rs`)
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
22. A packaged neural head promoted through spec 0077, with its exported
    test cases: the admitted package route matches frozen outputs within
    tolerance, Rust's typed integration preserves the contract, and no
    bespoke forward pass or Python adapter is present.
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
36. On a fresh install every suggesting module remains in shadow until
    the current spec 0077 champion admits review, subject to spec 0015's
    ceiling; no setting can make it insert links without a person.
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
41. With eight eligible destinations across several anchors, job 1 and
    degraded mode each return 3–6 unique destinations for the source,
    never more than five for one anchor and never more than six overall.
    (`tests/destination_count.rs`)
42. With two eligible destinations and a third that is diverse but
    unsuitable, the result contains only the two and shows the unmet
    minimum; no threshold or equity rule is relaxed.
    (`tests/destination_count.rs`)
43. With five otherwise eligible destinations but a thread allowance of
    one, the result contains one and reports the unmet minimum rather
    than bypassing the allowance. (`tests/destination_count.rs`)
44. Direct ranking API, CLI and event contracts cannot activate, tune or
    roll back an output; only a verified spec 0077 manifest transition
    changes the versions consumed. (`tests/champion_ownership.rs`)

## Changes from the source

- ganit's first-party engines, live service and workers are Rust,
  running inside the service slice. Spec 0019 runs nightly work
  through registered Rust jobs.
- LambdaMART, neural heads and alternative scoring models use admitted
  packaged execution behind typed Rust integration, never a bespoke
  pure-Rust forward pass or first-party Python production adapter.
- The ranking module moves into shodhan, and old `setu` names become
  `rank`, since setu now names the link to the add-ons.
- The review page reaches ranking through the signed site link; the
  management API is served on the local socket only.
- Lab notebooks become reproducible declarative package runs with
  recorded inputs, outputs and spec 0077 evaluation receipts.
- The situations count runs with Cargo, not a separate task runner.
- ganit's own job tests are Rust tests, with explicitly permitted
  non-production package parity checks where a specification needs them.
- Speed follows the 5% median rule; the 20x goal is kept (spec 0007).
- Memory budgets follow AGENTS.md section 5.6. Every ganit service,
  packaged worker and nightly job needs measured resource admission;
  none is assumed to fit a PC under 16 GB.
- The whole-source recommendation set has one 3–6 suitable-destination
  gate, and spec 0077 is the only optimisation, activation and rollback
  route.

Source: `docs/specifications/linking/ranking.md`, `linking/ranking-2.md`,
`linking/ranking-3.md` and `linking/ranking-4.md` in the archive
`archive-2026-11-08-old-repo`.
