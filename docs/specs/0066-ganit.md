# 0066: ganit, the shared maths engine

ganit ("mathematics") is Sutradhar's one home for numerical work:
optimisation, calibration, learning, inference, statistics, forecasts,
simulation, graph analysis and the rest. Other parts ask it numbered
questions, called jobs, and get back versioned answers they check
before use. ganit decides the maths; the calling part applies it. It is
Rust, like the rest of the product. Roadmap Phase 5 item 4 builds
ganit; a job whose caller comes in an earlier roadmap item is built
with that caller.
GanitEvolve (spec 0012) is a separate tool that uses ganit's maths; this
spec does not describe it.

Example: each night the `graph` module exports the site's link graph as
ids and weights. ganit's Jaal engine runs job 251 (internal PageRank)
with the run's seed, and writes a stamped output. Ranking shadow-tests
and activates it. Job 259 then reads it to work out the PageRank gain
if a link is accepted, and the "why" panel shows that gain (spec 0027).

## What it does

### Who calls it

The "Used by" column of the job table below names every spec that asks
a numbered job; this list groups them.

- Ranking (spec 0027) asks job 1 live, and uses the most nightly jobs.
  Specs 0068 and 0069 read the same outputs as steps of method chains;
  the table names their owner, spec 0027.
- Staff suggestions, link additions, feedback learning and feedback
  harvest (specs 0028, 0029, 0053, 0054).
- Analytics, search connectors, search charts, the SEO crawler, the web
  link study, site checks and the fix list (specs 0034, 0040, 0043,
  0044, 0046, 0047, 0048).
- The knowledge graph, sorting and karma, moderation hints, hubs and the
  coach additions (specs 0049, 0050, 0051, 0052, 0042).
- Install and upgrade, audit and alerts, search services, hisaab, the
  test runner and the models (specs 0030, 0032, 0038, 0035, 0011,
  0024).
- The method catalogue, the method lab, quality judging, the research
  rules and GanitEvolve (specs 0055, 0056, 0057, 0013, 0012), which run
  on Dell.

Some callers use ganit's maths through named entry points instead of
numbered jobs, such as the settings what-ifs and before-and-after
figures (specs 0020, 0014), the visual layer's layouts, projections and
forecasts (spec 0017), the language layer's training (spec 0026),
GanitEvolve's search maths (spec 0012), the module-promotion measure
(spec 0015), safe running's band maths (spec 0033), the test room's
comparisons (spec 0039), the Editorial Coach's gap and uncertainty maths
(spec 0041) and the backlink crawl's maths (spec 0045). Each entry point
joins the registry by name together with the caller that needs it, with
its inputs, output, tests and whether it runs live, as a karya job of
kind `ganit.<name>`, where a name is never a number, or on Dell
through the program interface. karya, the live interface and the Dell
program interface refuse a name the registry does not hold.

### The engines

Each engine is built once, as a module of the `ganit` crate. Job
numbers follow the owner's original list, grouped by engine. A new job
takes the next free number after 354; a number is never reused, and a
merged job keeps its number as a pointer. Jobs from 322 on each belong
to one engine although their numbers lie outside its range; the
registry names the engine.

- **Chayan** (selection), jobs 1 to 32: optimisation and decisions,
  such as the link set per post, review order and the nightly schedule.
- **Mapan** (measurement), jobs 33 to 44: calibration, conformal sets
  and thresholds.
- **Shiksha** (learning), jobs 45 to 93: supervised
  models, such as the scoring model and the classifiers.
- **Anuman** (inference), jobs 94 to 114: Bayesian priors,
  posteriors and probabilities.
- **Samanjan** (adjustment), jobs 115 to 125: tuning settings within
  their bounds.
- **Mulyankan** (evaluation), jobs 126 to 145: intervals, paired
  comparisons, false-discovery control and "no worse than" decisions.
  Statistics live here once (spec 0013).
- **Vikalp** (alternative), jobs 146 to 149: scoring other policies
  offline from logged decisions.
- **Satya** (truth), jobs 150 to 156: label truth and agreement
  between judges, moderators and checks.
- **Parivartan** (change), jobs 157 to 175: changepoints in a series.
- **Vichalan** (deviation), jobs 176 to 190: anomalies.
- **Bahav** (drift), jobs 191 to 200: shifts in a distribution, and
  when to recalibrate or retrain.
- **Bhavishya** (future), jobs 201 to 221: forecasts with ranges.
- **Gati** (motion), jobs 222 to 225: dynamics, such as attention decay.
- **Anukaran** (simulation), jobs 226 to 231: replay and simulation.
- **Niyantran** (control), jobs 232 to 237: feedback controllers.
- **Kshamata** (capacity), jobs 238 to 250: cost, memory
  and capacity models.
- **Jaal** (web), jobs 251 to 267: graph analysis (below).
- **Chitran** (depiction), jobs 268 to 272: graph layouts.
- **Varnakram** (spectrum), jobs 273 to 279: spectral methods and linear
  algebra, such as whitening and spectral graph embeddings.
- **Sannidhya** (nearness), jobs 280 to 293: embedding geometry, such as
  hubness and near-duplicate clusters.
- **Padbandh** (phrase), jobs 294 to 297: phrase statistics.
- **Karan** (cause), jobs 298 to 308: causal effects.
- **Ayu** (lifespan), jobs 309 to 321: survival analysis.

### The job registry

ganit's job registry is a data file in the `ganit` crate. Each entry
names the job's number, engine, inputs (its snapshot schema), output
schema, schedule (live, nightly or on demand), consumers, tests,
capability id and whether it draws random numbers. karya refuses a job
kind the registry does not hold (specs 0019, 0056). The table below
lists every job another spec names; a gate check makes sure each of
them, and each named entry point above, is in the data file.

| Job | Engine | What it answers | Used by |
|---|---|---|---|
| 1 | Chayan | Best link set per post: no overlapping anchors, no repeated destination, the link cap, a variety term (live) | 0027 |
| 2 | Chayan | How much better a new destination must be to replace a waiting suggestion | 0027 |
| 3 | Chayan | Display threshold: the lowest score worth a moderator's time | 0027 |
| 4 | Chayan | Review-queue order giving the most accepted links per moderator minute | 0027, 0028 |
| 5 | Chayan | Split of a fixed capacity across competing claims, with floors | 0048 |
| 7 | Chayan | Anchor-text variety targets per destination | 0027 |
| 11 | Chayan | Nightly job schedule under the memory ceiling and quiet window | 0035, 0040 |
| 14 | Chayan | Retention plan for metrics and traces under the disk budget | 0035 |
| 23 | Chayan | Best threads per topic for each hub | 0052 |
| 24 | Chayan | One primary hub per thread | 0052 |
| 25 | Chayan | Expert routing with workload caps | 0051 |
| 26 | Chayan | Fair split of the review queue across moderators | 0028 |
| 27 | Chayan | Long threads most in need of a summary | 0052 |
| 28 | Chayan | The single recommended action for the week | 0048 |
| 31 | Chayan | Open cases most worth a person's look, by information gained per cost | 0028, 0048 |
| 32 | Chayan | Tonight's experiment slate by value and cost | 0056 |
| 33 | Mapan | Calibrated acceptance probability per suggestion | 0027, 0053 |
| 34 | Mapan | Conformal count of alternatives per post, never above five | 0027 |
| 35 | Mapan | Reranker precision curve and threshold per site | 0027 |
| 44 | Mapan | Rising and trending thresholds per forum | 0050 |
| 45 | Shiksha | Scoring model: logistic fusion, LambdaMART once it wins | 0027, 0053 |
| 46 | Shiksha | Feature importance for each score | 0027, 0053 |
| 47 | Shiksha | Link density cap by post length | 0027 |
| 48 | Shiksha | Cap on posts in one thread linking the same destination | 0027 |
| 49 | Shiksha | Diminishing returns of inbound links per destination | 0027, 0029 |
| 50 | Shiksha | Review time per suggestion | 0028, 0029 |
| 51 | Shiksha | Anchor quality | 0027 |
| 52 | Shiksha | Linkability: how many posts could link to a destination | 0027 |
| 56 | Shiksha | Which added links get clicked | 0034 |
| 66 | Shiksha | Thread type: guide, question, discussion or showcase | 0027, 0056 |
| 67 | Shiksha | Question detector | 0051 |
| 68 | Shiksha | Likely solution in unsolved threads | 0027, 0052, 0056 |
| 74 | Shiksha | Spam classifier | 0056 |
| 79 | Shiksha | Query intent | 0042, 0044 |
| 80 | Shiksha | Branded versus non-branded query share | 0044 |
| 81 | Shiksha | Crawl-budget model behind crawl-waste advice | 0044 |
| 82 | Shiksha | Indexing probability per page | 0040 |
| 83 | Shiksha | Thin-content value behind noindex proposals | 0044 |
| 94 | Anuman | Acceptance priors for similar pairs | 0027 |
| 95 | Anuman | Exploration posteriors per site, destination type and anchor type | 0027 |
| 96 | Anuman | Accept rates per module and site, and the promotion decision | 0053 |
| 97 | Anuman | Destinations that attract rejections | 0027, 0028 |
| 98 | Anuman | Flaky-test probability | 0011 |
| 99 | Anuman | Chance that a change made things worse | 0053 |
| 102 | Anuman | Expert skill per topic | 0051 |
| 104 | Anuman | Reactor reliability | 0050 |
| 105 | Anuman | Best-reply priors per forum | 0050 |
| 106 | Anuman | Karma as a Bayesian average | 0050 |
| 109 | Anuman | Chance that a solved answer is now outdated | 0027, 0052 |
| 113 | Anuman | Rising surprise scores | 0050 |
| 115 | Samanjan | Retrieval settings, such as BM25 k1 and b | 0027, 0053 |
| 116 | Samanjan | Thread-vector weights | 0053 |
| 120 | Samanjan | Scoring-model hyperparameters | 0053 |
| 121 | Samanjan | Hot's decay per forum, replayed against engagement | 0050 |
| 123 | Samanjan | Keyword-index merge policy | 0038 |
| 126 | Mulyankan | Pass-rate intervals and pass marks | 0055, 0056, 0057 |
| 127 | Mulyankan | Recall and anchor-agreement intervals | 0055, 0056, 0057 |
| 128 | Mulyankan | Paired comparisons on the same posts | 0053, 0056, 0057 |
| 129 | Mulyankan | Bootstrap intervals for differences in ranking measures | 0053, 0055, 0056 |
| 130 | Mulyankan | False-discovery control over many tries | 0013, 0053, 0055, 0056 |
| 131 | Mulyankan | Stopping a trial early when the answer is clear | 0056 |
| 132 | Mulyankan | "No worse than" decisions with a stated margin | 0053, 0055, 0056 |
| 133 | Mulyankan | Evaluation samples that match the site's mix | 0055, 0056, 0057 |
| 136 | Mulyankan | When to re-score suggestions made at partial index coverage | 0027 |
| 139 | Mulyankan | Error levels for automatic demotion | 0053 |
| 142 | Mulyankan | Precomputed surfaces for the threshold sliders | 0027 |
| 143 | Mulyankan | When a hub needs refreshing | 0052 |
| 144 | Mulyankan | Smallest sample that gives an analysis its precision | 0055, 0056 |
| 145 | Mulyankan | Utility of accuracy, speed and memory, for choosing a model | 0056 |
| 146 | Vikalp | Moderator position bias | 0053, 0054 |
| 147 | Vikalp | New ranking policies scored offline from logged decisions | 0027, 0053 |
| 149 | Vikalp | New exploration rates scored offline | 0027 |
| 150 | Satya | Judge agreement per run | 0057 |
| 151 | Satya | Best estimate of truth from judges, moderators and reconstruction | 0056, 0057 |
| 152 | Satya | Judge bias and deference | 0057 |
| 153 | Satya | Label noise in the reconstruction set | 0053, 0056 |
| 154 | Satya | Moderators who accept everything | 0032, 0053 |
| 155 | Satya | Consistency between moderators on similar cases | 0032 |
| 156 | Satya | Clusters of disagreement that point to an unclear rubric | 0057 |
| 157 | Parivartan | Changepoints in accept rate per module | 0053 |
| 160 | Parivartan | Changepoints in Googlebot's crawl rate | 0044 |
| 161 | Parivartan | Changepoints in organic clicks | 0034, 0043, 0044 |
| 162 | Parivartan | Changepoints in many rankings at once | 0044 |
| 165 | Parivartan | Changepoints in memory per process | 0057 |
| 168 | Parivartan | Changepoints in moderator decisions per day | 0028 |
| 176 | Vichalan | Anomalies in request rate | 0034, 0043 |
| 178 | Vichalan | Reactions piling onto one thread | 0050 |
| 179 | Vichalan | Crawling of non-indexable addresses | 0044 |
| 180 | Vichalan | Server errors served to bots | 0044 |
| 192 | Bahav | Score shift after a model or settings change, for recalibration | 0027, 0053 |
| 196 | Bahav | Drift in scoring features, and when to retrain | 0053 |
| 198 | Bahav | Shift in traffic sources | 0034 |
| 200 | Bahav | Karma inflation and its normalisation factor | 0050 |
| 201 | Bhavishya | Disk-use forecast | 0035, 0038 |
| 204 | Bhavishya | Organic clicks forecast | 0034, 0043 |
| 205 | Bhavishya | Googlebot requests forecast | 0044 |
| 206 | Bhavishya | Review-queue size forecast | 0028 |
| 211 | Bhavishya | Busy hours by weekday | 0040 |
| 212 | Bhavishya | Seasonal traffic | 0034, 0043 |
| 214 | Bhavishya | Estimate for the last 2 to 3 days of Search Console data | 0040, 0043 |
| 215 | Bhavishya | Missing days of traffic, shown as estimates | 0034 |
| 217 | Bhavishya | Moderator availability by hour | 0028 |
| 222 | Gati | Attention decay per thread, for freshness | 0027, 0056 |
| 226 | Anukaran | Queue lag under projected volume | 0013 |
| 228 | Anukaran | Promotion policy replayed on history, to stop flapping | 0053 |
| 229 | Anukaran | Trial sample sizes | 0053, 0056 |
| 238 | Kshamata | Memory per part for any settings | 0035 |
| 239 | Kshamata | Re-embed and reindex time | 0035 |
| 240 | Kshamata | Time per post | 0035 |
| 241 | Kshamata | Index and cache disk size | 0035, 0038 |
| 242 | Kshamata | Estimator accuracy against actuals | 0035 |
| 243 | Kshamata | Gate time and queue wait per push | 0035 |
| 244 | Kshamata | Database pool sizes | 0035 |
| 245 | Kshamata | Worst-case latency from load tests | 0013, 0035 |
| 246 | Kshamata | Alert thresholds tuned from history | 0032, 0035 |
| 247 | Kshamata | Memory limits sized from measured peaks | 0035 |
| 248 | Kshamata | Whether each hot kernel is bound by compute or by memory | 0035 |
| 249 | Kshamata | Largest possible speed-up per hot path | 0035 |
| 250 | Kshamata | Build cache budgets from hit rates | 0035 |
| 251 | Jaal | Internal PageRank | 0027, 0044, 0049 |
| 252 | Jaal | PageRank per topic hub | 0052 |
| 253 | Jaal | Orphan threads and click depth | 0027, 0044 |
| 254 | Jaal | Link communities and the threads that bridge them | 0049 |
| 255 | Jaal | Core knowledge threads (k-core) | 0049, 0052 |
| 256 | Jaal | Link-prediction scores: Adamic–Adar, resource allocation, Katz | 0027, 0049 |
| 258 | Jaal | Pairs of threads that link only to each other, as spam-pattern flags | 0027, 0044 |
| 259 | Jaal | PageRank gain if a suggestion is accepted | 0027 |
| 262 | Jaal | Topic cohesion per forum (clustering coefficient) | 0049 |
| 263 | Jaal | Rings of accounts reacting to each other | 0050, 0051 |
| 264 | Jaal | Tag co-occurrence | 0049 |
| 265 | Jaal | Pages competing for the same queries | 0042, 0044, 0047 |
| 271 | Chitran | Small neighbourhood graphs per review card | 0028 |
| 273 | Varnakram | Embedding whitening, shipped only if it wins | 0012, 0013, 0024, 0053, 0056 |
| 275 | Varnakram | Co-reading factorisation | 0012 |
| 276 | Varnakram | Spectral graph embeddings | 0027, 0049 |
| 278 | Varnakram | Clusters of the combined link and meaning graph: the one topic definition | 0027, 0042, 0049, 0052 |
| 280 | Sannidhya | Hubness correction | 0027 |
| 284 | Sannidhya | Topic names for job 278's clusters | 0049 |
| 286 | Sannidhya | Near-duplicate clusters and their canonical thread | 0027, 0047, 0049 |
| 289 | Sannidhya | Content gaps from on-site search and Search Console queries | 0042, 0044 |
| 295 | Padbandh | Nicknames and synonyms | 0049 |
| 296 | Padbandh | Generic anchors | 0027 |
| 298 | Karan | Effect of added links on destination clicks | 0034, 0046 |
| 299 | Karan | Effect of each release on organic traffic | 0046 |
| 300 | Karan | Variance reduction for accept-rate comparisons | 0053 |
| 302 | Karan | Effect of IndexNow notices on crawl speed | 0044 |
| 303 | Karan | Effect of meta-description length on click-through | 0044 |
| 304 | Karan | Effect of structured data on click-through | 0044 |
| 306 | Karan | Effect of review-page changes on accept rate | 0053 |
| 307 | Karan | Regressions in live figures after an upgrade | 0030 |
| 308 | Karan | Before-and-after traffic per linked thread | 0034, 0046 |
| 309 | Ayu | How acceptance chances fade with a suggestion's age | 0028, 0046 |
| 310 | Ayu | How long inserted links survive later edits | 0046, 0053 |
| 311 | Ayu | Time until Google first crawls a new thread | 0044 |
| 317 | Ayu | Time to first reply, and the chance a thread stays unanswered | 0051 |
| 318 | Ayu | Expert response times | 0051 |
| 321 | Ayu | Time to decision per moderator and queue | 0028 |
| 322 | Shiksha | Bot probability per session | 0034 |
| 323 | Satya | Inflation check across script counts, server logs and Search Console | 0034, 0035, 0044 |
| 324 | Shiksha | Need score per thread | 0027, 0029, 0034, 0044, 0047 |
| 325 | Anuman | Quality floor per destination, shrunk for low traffic | 0027, 0034 |
| 346 | Kshamata | Restore-time model | 0035 |
| 351 | Kshamata | Agent cost per change that lands | 0035 |
| 354 | Mulyankan | Behaviour-fingerprint comparison: whether two methods behave the same | 0055 |

A job whose consumer does not exist ships with that consumer, never
before it.

### How jobs run

- Pure Rust, in the `ganit` crate. No other language, no native maths
  library and no C compiler (AGENTS.md section 4).
- `ganit` depends on `sutradhar-methods` for the one rank-correlation
  function that Mulyankan's behaves-the-same job and `qualify` both use
  (spec 0055, check 3; spec 0067). Nothing in `sutradhar-methods`
  depends on `ganit`.
- Inputs are Arrow snapshots of ids, numbers and word counts, never post
  text, names or search terms (specs 0019, 0027, 0044, 0049). Named
  entry points follow the same rule, with one exception: the language
  layer's training, and its held-out scoring on admitted test sets,
  read only sentences from the datasets that spec 0026 lists as usable
  data, such as the treebanks and the named-entity sets, never post
  text, names from posts, search terms or tagged copies of real posts.
  Held-out scoring on the forum gold set gets from the language
  layer only token positions, gold and predicted tag ids, spans as
  token ranges and arcs as head positions with relation ids, never a
  word, lemma or sentence text.
- Deterministic: every random draw comes from one generator seeded by
  the run's seed. The same code version, snapshot, settings and seed
  give the same numbers (spec 0056).
- Where it runs, and its one public interface, in three parts:
  - The live part is a module of the service on the owner's PC, wired
    at the composition root with typed settings and its own health and
    readiness (spec 0015). Live jobs and named entry points, such as
    job 1 and the settings what-ifs, answer through its versioned
    interface within a strict time limit; when a call times out or the
    module is not ready, callers use their degraded mode (specs 0020,
    0027).
  - The batch part is one batch runner that karya starts at the start
    of the nightly window and stops at its end, or when no `ganit.*`
    jobs are left; karya asks it to run each job, one at a time,
    through its versioned interface (spec 0019).
  - On Dell, the lab, quality judging and GanitEvolve call it through
    the program interface of specs 0056 and 0012, and ganit's own tests
    run there.
  ganit never runs on a site's server, and the add-ons never call it.
- Batch jobs are karya jobs of kind `ganit.<job>`, one at a time, in
  the nightly window, in spec 0024's heavy job slot. ganit never
  schedules itself; it checkpoints, so a paused run resumes (spec 0019).
  On Dell, heavy runs go through pankti (spec 0059).
- Each output carries its reproducibility stamp: job, ganit code
  version, snapshot digest, settings digest, seed and metrics. ganit
  writes it to the shared data folder and never writes Sutradhar's
  database. karya checks its schema and the numerical guards (no NaN,
  infinity or out-of-range value). The consumer shadow-tests it, then
  activates it or keeps the last good one, and can roll it back (specs
  0019, 0027).
- An output that feeds ranking reaches it either as the input of a
  catalogue method (spec 0055) or as a typed signal with its provenance,
  freshness and confidence (spec 0027, spec 0064). ganit never chooses a
  signal's door and never moves a link past the relevance gate.
- Each engine function used as a method has one catalogue entry, named
  `ganit:<engine>::<function>`, such as `ganit:jaal::pagerank`
  (spec 0055). Seeds, weights and settings never make a new method.
- Each outside library has one job in `gate/packages.toml`. A library
  other specs already name for this work, such as `faer` for matrix
  maths or `candle` for small networks (spec 0056), is listed there
  before ganit uses it. Which others are needed is to research.

### Jaal, graph analysis

Jaal owns the pure-Rust graph algorithms (spec 0064). The service holds
no graph library; the `graph` module stores nodes and edges, exports
them and applies Jaal's results (spec 0049). Jaal reads the thread link
graph, the entity graph, the reaction graph (job 263), the tag graph
(job 264) and the page and query graph (job 265). Edges are typed,
directed where the source is directed, and weighted.

- Centrality: internal PageRank (251), PageRank per topic hub (252) and
  the PageRank gain if one suggestion is accepted (259).
- Community detection: link communities and the threads that bridge
  them (254), topic cohesion (262) and rings of accounts reacting to
  each other (263). Varnakram's job 278 uses Jaal's graph for the one
  topic definition. Which community method each job uses is to research.
- Structure: orphan threads and click depth (253), core knowledge
  threads by k-core (255), pairs of threads that link only to each other
  (258), tag co-occurrence (264) and pages competing for the same
  queries (265).
- Link prediction: Adamic–Adar, resource allocation and Katz scores, and
  the top candidates per thread (256).

Iterative algorithms stop at the problem's tolerance or its iteration
cap, both set per job by research. The graph for one job is held in a
compact form within the batch memory budget; what happens past that
size is to research.

NetworkX's public list of graph operations is a clean-room reference
for which operations to offer, as spec 0064 says. No NetworkX code is
read, copied or run. Each algorithm is written from its published
definition. A further operation from that list enters as a catalogue
candidate (spec 0055) and is admitted through the lab (spec 0056) before
any job uses it.

Pixie-style bounded random walks are lab candidates only. They are
compared with named baselines on held-out sites, and reach a job only
once the method lab admits them (specs 0049, 0056, 0064). Their walk
counts and lengths are to research.

## Limits

- The live part's memory, counted in spec 0024's row for other parts,
  and the nightly batch memory are the ganit figures spec 0027 sets.
  Job 1's time limit and its point of falling back to degraded mode are
  spec 0027's.
- Nightly batch jobs run one at a time in spec 0024's heavy job slot.
- Snapshots are written in streamed batches no larger than spec 0019
  allows.
- On Dell, ganit's verdict on a lab reconstruction run, and each lab
  job's place in the heavy job slot, are as spec 0056 sets.
- All of this sits within the memory limits in AGENTS.md section 5.6.
- Time limits per batch job, and Jaal's run time on the 50,000-thread
  tier, are to research.

## Footprint budget

ganit adds nothing to public pages.

## How it is tested

Rust tests in the `ganit` crate, run on Dell through pankti:

1. Small known graphs, with answers worked out by hand: an empty graph,
   one node, an isolated node, a path, a star, a directed cycle, a
   complete graph, two triangles joined by one edge, and a graph with
   two parts. For example, on a directed cycle every PageRank is equal;
   on two triangles joined by one edge, that edge is the only bridge and
   community detection returns the two triangles; on a path from the
   entry thread, click depths count up by one; a thread with no inbound
   link is an orphan.
2. Link prediction on small graphs matches hand sums: two threads whose
   only shared neighbour has two links score 1/ln 2 for Adamic–Adar and
   1/2 for resource allocation.
3. Job 258 flags two threads that link only to each other, and clears
   the flag once a third thread links to either.
4. Every expected value is worked out by hand from the algorithm's
   published definition, and the working is kept with the test. No
   NetworkX code, output or example result is used.
5. Property tests (`proptest`): on a graph with at least one node,
   PageRank values are not negative and sum to 1 within the problem's
   tolerance, and the empty graph gives an empty result; renaming nodes
   renames results and changes nothing else; edge order in the snapshot
   changes nothing; link-prediction scores on an undirected graph are
   symmetric; a node's core number never passes its degree; communities
   cover every node once.
6. Determinism: a job run twice with the same code version, snapshot,
   settings and seed gives the same numbers, and the same with a
   different thread count.
7. Every job: golden outputs within the problem's tolerance, and the
   numerical edge cases empty data, one observation, all accepts, ties
   and missing days.
8. Guards: an output holding NaN, infinity or an out-of-range value is
   refused, and the last good one stays active (specs 0019, 0049).
9. Contract: every snapshot schema holds ids, numbers and word counts
   only, and a field of text is refused; the training entry point and
   held-out scoring on admitted test sets refuse any sentence from no
   dataset in spec 0026's usable data, and held-out scoring on the
   forum gold set refuses any input holding a word, a lemma or sentence
   text.
10. A batch job killed mid-run resumes from its checkpoint and gives the
    same output.
11. On Dell, each Jaal job on the 50,000-thread tier stays within the
    batch memory budget, and its benchmark follows spec 0007's median
    rule.
12. A job number or entry point name the registry does not hold is
    refused by karya, the live interface and the Dell program
    interface, and every job and named entry point another spec names
    is in the registry with its engine.

## Changes from the source

- ganit was Julia, with a Fortran solver library for batch work. Both
  languages are gone. AGENTS.md section 4 makes the product Rust, so
  every engine and job is Rust.
- The serving container became the live module of the service, and the
  batch container became a batch runner that karya starts in spec
  0024's heavy job slot.
- The archive's memory figures give way to spec 0027's.
- One Julia package per engine becomes our own Rust code, or one
  outside library per job listed in `gate/packages.toml`. Jaal uses no
  outside graph library.
- Julia's quality tools give way to the gate's Rust checks; notebooks
  give way to Rust lab runs (spec 0056).
- The 23 engines and the owner's job numbers are kept. Consumers that
  were Elixir modules are now Rust parts of the service.
- New: NetworkX as a clean-room reference list, and Pixie-style walks as
  lab candidates only (spec 0064).

Source: `docs/dev/ganit.md` and `docs/dev/ganit-2.md` to
`docs/dev/ganit-5.md` in the archive `archive-2026-11-08-old-repo`.
