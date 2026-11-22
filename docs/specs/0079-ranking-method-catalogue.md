# 0079: Ranking method catalogue

The ranking catalogue began with 390 candidate methods across 30 families,
with 13 in each. The release floor is 360 distinct, qualified methods,
with at least 12 in every family (ADR-0021, DR-022). The 13th original
entry allows for one rejected candidate only if the
remaining entries are in scope and independently qualify. Spec 0055
owns how a method is admitted. This spec owns
the list, the families and the shared contract. Spec 0027 owns the ranking
pipeline that calls the methods, spec 0072 the models Buddhi and Rakshak,
spec 0057 the quality bars and spec 0056 the lab.

Example: Family 2 lists BM25, BM25F, BM25+ and BM25L as four methods. Each
has its own formula. "BM25 with k1 = 1.5" is not a fifth: it is a setting of
BM25, and the check in spec 0055 refuses it. When Sam proposes BM25F, the
check finds it differs on a small example (a title match outranks a body
match) and admits it as a candidate.

Every row below is a candidate. Being listed admits nothing: a method counts
only when it reaches `release_qualified` under spec 0055 with its own
evidence. Attributions are from general knowledge; the research agent
verifies each against its source before the method's sheet is approved.

## Words used here

- **Method**: one ranking signal with its own mechanism, listed as one row.
- **Family**: one of the 30 groups that organise the candidate methods.
- **Candidate**: a listed row. Being listed admits nothing.
- **Qualified**: at `release_qualified` under spec 0055, with its own
  evidence. Only qualified methods count toward the floor.
- **Floor**: at least 12 qualified methods in every family and 360 in all.
- **Spare**: an eligible extra candidate beyond the family floor.
  Each candidate still needs its own qualification.
- **Cost class**: C0 per page and precomputed; C1 cheap per pair; C2
  embedding per pair; C3 cross-encoder; B batch job.
- **Basis**: PA published algorithm, ST standard, OD original design.
- **Abstain**: return no score and say why when the data is missing.

## What it does

### What counts toward the floor

1. A method counts only if it is `release_qualified`, is called by a shipped
   ranking pipeline, and has a different signal or mechanism from every
   other counted method (spec 0055's three distinctness checks).
2. A setting, preset, wrapper or alias never counts. Two methods that differ
   only in numbers are one method.
3. A method counts in one family only. If it could fit two, it keeps the one
   named in the table.
4. The floor is at least 12 counted methods in each of the 30 families and at
   least 360 in total. A family with fewer than 12 blocks the release even
   when the total is higher.
5. The count is descriptive evidence for this floor. It never stands in for
   the quality bars of spec 0057, which always apply.

### The shared contract

Every method states these fields in its sheet (spec 0055 adds identity,
stage and admission fields).

| Field | Rule |
|---|---|
| Input kind | Pair of threads, pair of passages, a set of candidates, or one page |
| Output | A score in [0, 1], or a signed score in [-1, 1] marked as such, plus a one-sentence plain-English reason a moderator can read |
| Missing data | The method abstains and says why. It never returns zero for "unknown" |
| Ties | Broken by a stable hash of the two ids and the run seed, so a rerun gives the same order |
| Determinism | The same input and seed give the same output on Linux and Windows |
| Numerical precision | 64-bit floats for scores, and a stated tolerance for model outputs |
| Cost class | C0 per page and precomputed; C1 cheap per pair; C2 embedding per pair; C3 cross-encoder; B batch job |
| Data used | Guest-visible text and darshak's human-only aggregates. No private posts, messages or personal data |
| Where it runs | Rust for every live decision. A batch method may run in the Python sidecar but writes plain numbers that Rust reads (ADR-0021) |
| Failure | A failed method is skipped with a recorded reason and the pipeline continues, except that a failed safety or guardrail method fails closed: the candidate is refused, not skipped |

### How the pipeline calls methods

Spec 0027 runs qualified active methods within the supported machine
and resource bounds of [0078](0078-memory-budget-and-turn-taking.md).
Its stages read C0 values from storage, use C1 methods for admitted
candidate checks, use C2 methods to score survivors and use C3 methods to
rerank the bounded set. Cost class describes work at an admitted stage.
A feature fact enters only its named post-relevance recipient under 0027.
A method may be skipped only for a recorded reason: its inputs are
missing, the stage budget cut it, or an allowed guardrail was switched
off. A safety or guardrail method that fails closes the affected path
and refuses the candidate.
The learned combiners of family 13 and fusion methods of family 16
consume every score from an exact active method and input version
admitted for that consumer. Research witnesses, including the offline
Family 29 cases, stay outside the serving stages and combiner inputs.

### Family rules

| Family | Owner of the signal | Extra rule |
|---|---|---|
| 1 to 4 | Text engine (shodhan) and models (spec 0024) | Never use text from quotes or masked zones |
| 5 to 7 | ganit engines on the link and entity graphs (specs 0049, 0066) | Graph is rebuilt from guest-visible links only |
| 8 | darshak (spec 0034) | Human-labelled visits only; includes query-to-page click satisfaction, long clicks against quick bounces |
| 9 to 11 | Site data and moderation (specs 0050, 0051) | Moderator decisions are feedback, not labels for the bar sets |
| 12 | Rakshak (spec 0072) | Output is a caution, never a sanction on a member |
| 13, 15, 16 | Learning and optimisation (specs 0053, 0077) | Learned only on feedback data; never on the bar or sealed sets |
| 14 | Pipeline set-level step | Works on the candidate set, not on one pair |
| 17 to 23 | Text engine and models | Models run through ONNX in Rust (spec 0024) |
| 24 | Site structure (spec 0047) | Uses crawl data from Leviathan only when present |
| 25, 30 | Leviathan (specs 0044, 0045) and Search Console (spec 0040) | Abstain when no crawl or connection exists |
| 26 | Pipeline rules | Fully transparent: every rule can be shown to a moderator |
| 27, 28, 29 | Site data, text engine, media metadata | Image models run on demand under spec 0078 |

## The catalogue

Basis: PA published algorithm, ST standard, OD original design. Cost: C0, C1,
C2, C3 or B as above.

### Family 1: Exact and phrase matching

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-01-01 | Exact phrase in title | The candidate anchor text appears whole in the target title | OD | C1 |
| RM-01-02 | Ordered phrase with slop | Anchor terms appear in order within k words in the target | PA | C1 |
| RM-01-03 | Unordered window match | Anchor terms all appear within a window in any order | PA | C1 |
| RM-01-04 | Title prefix match | Anchor is a prefix of the target title | OD | C1 |
| RM-01-05 | Character trigram overlap | Dice overlap of character trigrams of anchor and title | PA | C1 |
| RM-01-06 | Bounded edit distance | Levenshtein distance between anchor and title terms under a bound | PA | C1 |
| RM-01-07 | Transposition-aware distance | Damerau-Levenshtein distance for typos | PA | C1 |
| RM-01-08 | Phonetic name match | Double Metaphone codes of names and product words agree | PA | C1 |
| RM-01-09 | Abbreviation match | Site dictionary expands acronyms before exact matching | OD | C1 |
| RM-01-10 | Longest common substring | Share of the anchor covered by the longest common substring | PA | C1 |
| RM-01-11 | Title contained in body | The target title appears verbatim in the source sentence | OD | C1 |
| RM-01-12 | Word shingle resemblance | Jaccard resemblance of word shingles of anchor sentence and target lead | PA | C1 |
| RM-01-13 | Title set equality | Anchor and title have the same word set ignoring order | OD | C1 |

### Family 2: Probabilistic text models

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-02-01 | BM25 | Okapi term weighting with length normalisation over title and body | PA | C1 |
| RM-02-02 | BM25F | BM25 with separate weighted fields | PA | C1 |
| RM-02-03 | BM25+ | BM25 with a lower bound on term-frequency normalisation | PA | C1 |
| RM-02-04 | BM25L | BM25 variant with a different length normalisation | PA | C1 |
| RM-02-05 | TF-IDF cosine | Log term frequency times inverse document frequency, cosine | PA | C1 |
| RM-02-06 | Dirichlet query likelihood | Language model with Dirichlet smoothing | PA | C1 |
| RM-02-07 | Jelinek-Mercer likelihood | Language model with linear interpolation smoothing | PA | C1 |
| RM-02-08 | Two-stage smoothing | Dirichlet then interpolation smoothing | PA | C1 |
| RM-02-09 | PL2 | Divergence from randomness with Poisson and Laplace normalisation | PA | C1 |
| RM-02-10 | DPH | Divergence from randomness with a hypergeometric model | PA | C1 |
| RM-02-11 | InL2 | Inverse-document-frequency randomness model with Laplace | PA | C1 |
| RM-02-12 | Axiomatic F2EXP | Axiomatic retrieval function with an exponential length penalty | PA | C1 |
| RM-02-13 | Pivoted length normalisation | Vector-space weights pivoted on average length | PA | C1 |

### Family 3: Semantic embeddings

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-03-01 | Dense cosine | Single-vector cosine with the dense BGE-M3 embedding | PA | C2 |
| RM-03-02 | Sparse lexical weights | BGE-M3 learned sparse weights matched by dot product | PA | C2 |
| RM-03-03 | Multi-vector late interaction | BGE-M3 token vectors scored by maximum similarity | PA | C2 |
| RM-03-04 | Static mean embedding | Mean of static word vectors, cosine, as a fast pass | PA | C1 |
| RM-03-05 | Title-only dense | Cosine of title vectors only | OD | C2 |
| RM-03-06 | Passage-max dense | Maximum cosine over the target's passage vectors | OD | C2 |
| RM-03-07 | Passage centroid | Cosine against the mean of the target's passage vectors | OD | C2 |
| RM-03-08 | Title-boosted centroid | Centroid with the title weighted higher | OD | C2 |
| RM-03-09 | Instruction-prefixed asymmetric | Query and document encoded with different prefixes | PA | C2 |
| RM-03-10 | Binary-quantised Hamming | Hamming distance of sign-quantised vectors | PA | C1 |
| RM-03-11 | Product-quantised distance | Asymmetric distance over product-quantised codes | PA | C1 |
| RM-03-12 | Truncated embedding coarse pass | Cosine on a shortened leading part of the vector | PA | C1 |
| RM-03-13 | Mutual nearest neighbours | Each is in the other's top k by embedding | OD | C2 |

### Family 4: Cross-encoder reranking

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-04-01 | Title pair score | BGE reranker over source sentence and target title | PA | C3 |
| RM-04-02 | Title and lead score | Reranker over sentence and title plus opening paragraph | PA | C3 |
| RM-04-03 | Passage-max score | Highest reranker score over the target's passages | OD | C3 |
| RM-04-04 | Top-two passage mean | Mean of the best two passage scores | OD | C3 |
| RM-04-05 | Sentence window score | Reranker over a window of sentences around the anchor | OD | C3 |
| RM-04-06 | Thread-context score | Reranker with thread title and parent post as context | OD | C3 |
| RM-04-07 | Bidirectional mean | Average of source-to-target and target-to-source scores | OD | C3 |
| RM-04-08 | Asymmetry difference | Difference between the two directions as a caution | OD | C3 |
| RM-04-09 | Entity-masked score | Reranker with names masked to test topical match | OD | C3 |
| RM-04-10 | Strided long-document score | Chunked scoring of long targets with stride | PA | C3 |
| RM-04-11 | Distilled student score | A smaller distilled reranker as a cheaper pass | PA | C2 |
| RM-04-12 | Margin over baseline | Reranker score minus the lexical baseline score | OD | C3 |
| RM-04-13 | Early-exit cascade | Cheap model first, full model only when uncertain | PA | C3 |

### Family 5: Link-graph authority

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-05-01 | PageRank | Random-surfer importance on the internal link graph | PA | B |
| RM-05-02 | Personalised PageRank | PageRank restarting from the source thread | PA | B |
| RM-05-03 | Topic-sensitive PageRank | PageRank biased toward a topic hub | PA | B |
| RM-05-04 | HITS authority | Authority score from the mutual reinforcement with hubs | PA | B |
| RM-05-05 | HITS hub | Hub score of the source thread | PA | B |
| RM-05-06 | SALSA | Stochastic authority and hub scores from a bipartite walk | PA | B |
| RM-05-07 | TrustRank | Trust propagated from staff-chosen seed threads | PA | B |
| RM-05-08 | Distrust propagation | Distrust propagated backwards from known-bad threads | PA | B |
| RM-05-09 | Katz centrality | Weighted count of walks of all lengths | PA | B |
| RM-05-10 | Eigenvector centrality | Leading eigenvector of the link matrix | PA | B |
| RM-05-11 | Harmonic centrality | Sum of inverse distances to the thread | PA | B |
| RM-05-12 | Log in-degree | Logarithm of the number of distinct linking threads | OD | C0 |
| RM-05-13 | PageRank gain | Change in target PageRank if the link is added | PA | B |

### Family 6: Hub and community detection

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-06-01 | Louvain communities | Modularity optimisation; same-community bonus | PA | B |
| RM-06-02 | Leiden communities | Refined modularity optimisation; same-community bonus | PA | B |
| RM-06-03 | Label propagation | Communities from neighbour majority labels | PA | B |
| RM-06-04 | Infomap | Communities from the map equation | PA | B |
| RM-06-05 | k-core depth | Core number of the target in the link graph | PA | B |
| RM-06-06 | k-truss support | Triangle support of the connecting edge | PA | B |
| RM-06-07 | Bridge threads | Threads linking otherwise separate communities | PA | B |
| RM-06-08 | Sampled betweenness | Approximate betweenness centrality of the target | PA | B |
| RM-06-09 | Articulation points | Targets whose removal splits the graph | PA | B |
| RM-06-10 | Spectral clustering | Clusters from the Fiedler vector | PA | B |
| RM-06-11 | Hub topic selection | Best hub for a topic by modularity contribution | OD | B |
| RM-06-12 | Overlapping communities | Clique percolation communities | PA | B |
| RM-06-13 | Stochastic block model | Block membership by statistical inference | PA | B |

### Family 7: Entity and knowledge graph

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-07-01 | Entity Jaccard | Overlap of linked named entities | OD | C1 |
| RM-07-02 | Confidence-weighted entity overlap | Overlap weighted by entity-linking confidence | OD | C1 |
| RM-07-03 | Salience-weighted overlap | Overlap weighted by how central each entity is to the text | PA | C1 |
| RM-07-04 | Type-compatible overlap | Overlap only between entities of compatible type | OD | C1 |
| RM-07-05 | Entity co-occurrence PMI | Pointwise mutual information between entities across the site | PA | B |
| RM-07-06 | Graph path distance | Shortest path between entities in the site knowledge graph | PA | B |
| RM-07-07 | Knowledge-graph embedding similarity | TransE-style embedding similarity of entities | PA | B |
| RM-07-08 | Attribute match | Model numbers and attribute values agree | OD | C1 |
| RM-07-09 | Alias canonicalisation | Aliases resolved to one entity before matching | OD | C1 |
| RM-07-10 | Sense agreement | Both mentions resolve to the same sense | OD | C1 |
| RM-07-11 | Relation compatibility | Relation types such as requires or part-of agree | OD | C1 |
| RM-07-12 | Entity coverage gap | Target explains entities the source only mentions | OD | C1 |
| RM-07-13 | Taxonomy distance | Distance between categories in the tag hierarchy | PA | C0 |

### Family 8: Reader behaviour (darshak)

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-08-01 | Debiased click rate | Click-through of similar links, corrected for position | PA | C0 |
| RM-08-02 | Long-click share | Share of clicks followed by a long stay on the target | OD | C0 |
| RM-08-03 | Quick-bounce share | Share of clicks followed by a return within seconds (negative) | OD | C0 |
| RM-08-04 | Last-click success | Share of sessions ending on the target after the click | PA | C0 |
| RM-08-05 | Target dwell time | Median time on the target after arriving from the source | OD | C0 |
| RM-08-06 | Scroll depth | Median depth reached on the target | OD | C0 |
| RM-08-07 | Pogo-sticking rate | Click, return and click another target pattern | PA | C0 |
| RM-08-08 | Transition probability | Markov probability of moving from source to target | PA | C0 |
| RM-08-09 | Session co-visit lift | Item-item co-occurrence lift in human sessions | PA | B |
| RM-08-10 | Accepted-link survival | Accepted links that are later kept rather than removed | OD | C0 |
| RM-08-11 | Site-search satisfaction | Click satisfaction on the site's own search results | PA | C0 |
| RM-08-12 | Reading completion | Time on the target against its expected reading time | OD | C0 |
| RM-08-13 | Dynamic Bayesian click model | Click model that separates attractiveness from satisfaction | PA | B |

### Family 9: Freshness and time

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-09-01 | Update recency decay | Exponential decay since the target's last substantive update | OD | C0 |
| RM-09-02 | Age gap | Difference between source and target dates | OD | C0 |
| RM-09-03 | Burst detection | Kleinberg bursts in a topic's activity | PA | B |
| RM-09-04 | Seasonal relevance | Periodic topic demand across years | OD | B |
| RM-09-05 | Time-decayed PageRank | PageRank with older links weighted less | PA | B |
| RM-09-06 | Freshness demand | Traffic spike on a topic suggests recent answers are wanted | PA | C0 |
| RM-09-07 | Content half-life | Evergreen against news-like classifier | OD | C0 |
| RM-09-08 | Edit substance | Substantive edit against trivial edit | OD | C0 |
| RM-09-09 | Release-date alignment | Post date against the product's release date | OD | C0 |
| RM-09-10 | Reply velocity | Recent reply rate of the target thread | OD | C0 |
| RM-09-11 | Stale accepted answer | Accepted answer older than the current product version | OD | C0 |
| RM-09-12 | Rising together | Temporal correlation of topic activity | PA | B |
| RM-09-13 | Dead-thread gap | Long time since the last reply | OD | C0 |

### Family 10: Page structure and link position

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-10-01 | Paragraph position prior | Earlier paragraphs are better places for a link | OD | C1 |
| RM-10-02 | Heading context | Relevance of the nearest heading to the target | OD | C1 |
| RM-10-03 | Paragraph link density | Penalty when the paragraph already has many links | OD | C1 |
| RM-10-04 | Distance to nearest link | Tokens to the closest existing link | OD | C1 |
| RM-10-05 | Block type | Classifier: body, quote, signature, boilerplate | OD | C1 |
| RM-10-06 | Visibility likelihood | Estimated chance the position is seen without scrolling | OD | C1 |
| RM-10-07 | List, table and code context | Suitability of the surrounding block type | OD | C1 |
| RM-10-08 | Sentence role | Topic sentence against aside | OD | C1 |
| RM-10-09 | Markup depth | Deeply nested markup lowers the position's value | OD | C1 |
| RM-10-10 | Thread position | Opening post against reply | OD | C0 |
| RM-10-11 | Page-type compatibility | Source and target types fit (guide to reference) | OD | C0 |
| RM-10-12 | `Edit-in-progress guard` | Avoid text edited in the last hour | OD | C1 |
| RM-10-13 | Link budget left | Remaining link allowance on the source page | OD | C0 |

### Family 11: Quality and trust

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-11-01 | Solved flag | Thread has an accepted answer | OD | C0 |
| RM-11-02 | Wilson reaction score | Lower bound of the likes-based usefulness | PA | C0 |
| RM-11-03 | Author trust level | Site trust level of the author | OD | C0 |
| RM-11-04 | Staff authorship | Written or confirmed by staff | OD | C0 |
| RM-11-05 | Completeness | Length, structure and attachments predict a complete answer | OD | C0 |
| RM-11-06 | Reading-level fit | Reading grade suits the audience | PA | C0 |
| RM-11-07 | Error rate | Spelling and grammar errors as a quality proxy | OD | C0 |
| RM-11-08 | Edit stability | Few corrections after posting | OD | C0 |
| RM-11-09 | Report history | Moderation reports lower the score | OD | C0 |
| RM-11-10 | Reference presence | The post cites documentation | OD | C0 |
| RM-11-11 | Answer-quality classifier | Supervised classifier on moderator decisions | OD | C2 |
| RM-11-12 | Thin-content detector | Low information per token | OD | C0 |
| RM-11-13 | Bayesian helpfulness | Votes shrunk toward the site mean | PA | C0 |

### Family 12: Spam resistance (Rakshak)

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-12-01 | Link ring detector | Closed reciprocal clusters of linking | PA | B |
| RM-12-02 | Author anchor history | Share of the author's past links with exact-match commercial anchors | OD | C0 |
| RM-12-03 | Account velocity | New account posting and linking too fast | OD | C0 |
| RM-12-04 | Affiliate pattern | Affiliate parameters and commercial domains | OD | C1 |
| RM-12-05 | Link farm hub | Pages that link out to many low-quality pages | PA | B |
| RM-12-06 | Template spam | Shingle similarity to known spam templates | PA | C1 |
| RM-12-07 | New-account burst | Temporal anomaly among new accounts | OD | B |
| RM-12-08 | Keyword stuffing | Repeated phrase density | PA | C1 |
| RM-12-09 | Content spinning | Near-duplicates with synonym swaps | PA | C2 |
| RM-12-10 | Mutual-reaction ring | Accounts liking each other in rings | PA | B |
| RM-12-11 | Link-to-text ratio | Links per word far above the site norm | OD | C1 |
| RM-12-12 | Redirect and shortener | Shorteners and long redirect chains | OD | C1 |
| RM-12-13 | Owner blocklist match | Domains on the owner's local list | OD | C0 |

### Family 13: Learning to rank

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-13-01 | LambdaMART | Gradient-boosted trees with lambda gradients | PA | C1 |
| RM-13-02 | RankNet | Pairwise neural ranking | PA | C1 |
| RM-13-03 | LambdaRank | Pairwise neural ranking with metric-weighted gradients | PA | C1 |
| RM-13-04 | ListNet | Listwise top-one probability loss | PA | C1 |
| RM-13-05 | ListMLE | Listwise maximum likelihood of the ordering | PA | C1 |
| RM-13-06 | SoftRank | Smoothed rank distribution loss | PA | C1 |
| RM-13-07 | RankSVM | Pairwise linear support vector machine | PA | C1 |
| RM-13-08 | AdaRank | Boosting directly on a ranking metric | PA | C1 |
| RM-13-09 | Coordinate ascent | Linear weights tuned one at a time on the metric | PA | C1 |
| RM-13-10 | Random-forest regression | Pointwise forest over the method scores | PA | C1 |
| RM-13-11 | Pointwise logistic regression | Logistic model over the method scores | PA | C1 |
| RM-13-12 | Neural ranking head | The small neural network of ADR-0021 over all signals | OD | C1 |
| RM-13-13 | Dueling bandit gradient descent | Online pairwise learning from moderator choices | PA | B |

### Family 14: Diversity re-ranking

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-14-01 | Maximal marginal relevance | Trade relevance against similarity to chosen links | PA | C1 |
| RM-14-02 | Semantic diversity | Cluster cap on embeddings; no near-restatements | OD | C2 |
| RM-14-03 | Source diversity | Cap on links from one long thread or resource | OD | C1 |
| RM-14-04 | Reader-intent diversity | Cover setup, troubleshooting and compatibility intents | PA | C1 |
| RM-14-05 | Solution diversity | Cover different fixes or approaches | OD | C2 |
| RM-14-06 | Subtopic diversity | Cover aspects of a broad subject | PA | C1 |
| RM-14-07 | Evidence diversity | Mix official, verified and community experience | OD | C1 |
| RM-14-08 | Entity and version diversity | Cover relevant version variations | OD | C1 |
| RM-14-09 | Information-gain diversity | Choose targets adding details missing so far | OD | C2 |
| RM-14-10 | Freshness and time diversity | Mix recent and lasting answers | OD | C1 |
| RM-14-11 | Authority diversity | Mix staff, expert and member voices | OD | C1 |
| RM-14-12 | Audience-level diversity | Mix beginner, intermediate and expert explanations | OD | C1 |
| RM-14-13 | Format diversity | Mix guides, Q&A, reference and discussions | OD | C1 |

### Family 15: Calibration and uncertainty

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-15-01 | Platt scaling | Logistic map from score to probability | PA | C1 |
| RM-15-02 | Isotonic regression | Monotone map from score to probability | PA | C1 |
| RM-15-03 | Temperature scaling | One temperature on model logits | PA | C1 |
| RM-15-04 | Beta calibration | Three-parameter beta map | PA | C1 |
| RM-15-05 | Histogram binning | Bin-wise observed frequency | PA | C1 |
| RM-15-06 | Split conformal sets | Prediction sets with a coverage guarantee | PA | C1 |
| RM-15-07 | Bootstrap interval | Resampled confidence interval on a score | PA | B |
| RM-15-08 | Ensemble disagreement | Spread across seeds as uncertainty | PA | C2 |
| RM-15-09 | Bayesian shrinkage | Sparse signals pulled toward a prior | PA | C0 |
| RM-15-10 | Abstain threshold | Threshold from the reliability diagram | OD | C1 |
| RM-15-11 | Selective prediction | Risk-coverage cut for showing a suggestion | PA | C1 |
| RM-15-12 | Per-forum recalibration | Recalibrate on a forum after a drift test | OD | B |
| RM-15-13 | Quantile bounds | Quantile regression bounds on the score | PA | B |

### Family 16: Rank fusion and ensembles

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-16-01 | Reciprocal rank fusion | Sum of reciprocal ranks across methods | PA | C1 |
| RM-16-02 | CombSUM | Sum of normalised scores | PA | C1 |
| RM-16-03 | CombMNZ | CombSUM times the number of methods that scored | PA | C1 |
| RM-16-04 | Borda count | Rank-position voting | PA | C1 |
| RM-16-05 | Condorcet and Copeland | Pairwise majority voting | PA | C1 |
| RM-16-06 | Z-score linear fusion | Weighted sum of z-normalised scores | PA | C1 |
| RM-16-07 | Min-max linear fusion | Weighted sum of min-max normalised scores | PA | C1 |
| RM-16-08 | Logistic stacking | Meta-learner over method scores | PA | C1 |
| RM-16-09 | Mixture of experts | Gating chooses weights by context | PA | C1 |
| RM-16-10 | Kemeny approximation | Rank aggregation minimising disagreements | PA | C1 |
| RM-16-11 | Bayesian model averaging | Posterior-weighted combination | PA | C1 |
| RM-16-12 | Bandit method selection | Thompson sampling or LinUCB picks a method per context | PA | B |
| RM-16-13 | Cheap-then-expensive cascade | Early filter then full ranking | PA | C1 |

### Family 17: Intent and topic models

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-17-01 | LDA topic cosine | Latent Dirichlet allocation topic vectors | PA | B |
| RM-17-02 | NMF topic cosine | Non-negative factorisation topic vectors | PA | B |
| RM-17-03 | Embedding-cluster topics | Clusters of embeddings with class TF-IDF labels | PA | B |
| RM-17-04 | Zero-shot intent | Entailment model scores intent labels | PA | C2 |
| RM-17-05 | Supervised intent | Classifier: setup, troubleshoot, compatibility, how-to | OD | C2 |
| RM-17-06 | Question or statement | Sentence type match | OD | C1 |
| RM-17-07 | Task stage | Before purchase, install or fix | OD | C2 |
| RM-17-08 | Keyphrase overlap | TextRank or RAKE keyphrases overlap | PA | C1 |
| RM-17-09 | Category classifier | Taxonomy category agreement | OD | C2 |
| RM-17-10 | Topic hierarchy distance | Hierarchical clustering distance | PA | B |
| RM-17-11 | Latent semantic analysis | SVD concept similarity | PA | B |
| RM-17-12 | How-to template match | Instruction pattern in source and target | OD | C1 |
| RM-17-13 | Topic drift | Difference between the passage topic and the thread topic | OD | C2 |

### Family 18: Version and compatibility

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-18-01 | Version range overlap | Intersection of semantic-version ranges | ST | C1 |
| RM-18-02 | Compatibility table | Product-model compatibility lookup | OD | C0 |
| RM-18-03 | Platform tag agreement | Operating system and platform tags match | OD | C0 |
| RM-18-04 | Error code match | Same error code appears in both | OD | C1 |
| RM-18-05 | Version proximity | Numeric-aware closeness of mentioned versions | OD | C1 |
| RM-18-06 | Deprecation awareness | Target is about a removed feature | OD | C0 |
| RM-18-07 | Latest compatible | Prefer the newest compatible answer | OD | C0 |
| RM-18-08 | Version conflict | Source on a newer version, target for an older only | OD | C1 |
| RM-18-09 | Hardware revision | Hardware revision agreement | OD | C1 |
| RM-18-10 | Runtime version | Language and runtime versions agree | OD | C1 |
| RM-18-11 | Release-note link | Target mentions a fix for the mentioned version | OD | C1 |
| RM-18-12 | Breaking-change flag | Target aware of a breaking change | OD | C0 |
| RM-18-13 | Edition agreement | Free against paid edition match | OD | C1 |

### Family 19: Passage and section ranking

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-19-01 | Window BM25 | BM25 over sliding passage windows | PA | C1 |
| RM-19-02 | Passage dense max | Maximum dense similarity over passages | OD | C2 |
| RM-19-03 | Heading boost | Passage under a matching heading ranks higher | OD | C1 |
| RM-19-04 | Answer span score | Score of the best answer-like span | PA | C2 |
| RM-19-05 | Opening summary match | Match against the first paragraph | OD | C1 |
| RM-19-06 | Sentence alignment | Anchor sentence aligned to target sentences | OD | C2 |
| RM-19-07 | Top-k passage mean | Mean of the best k passages | OD | C1 |
| RM-19-08 | Passage spread | Relevant passages spread across the target | OD | C1 |
| RM-19-09 | Fragment target choice | Pick the in-page anchor to link to | OD | C1 |
| RM-19-10 | Code-block match | Match against code blocks for technical text | OD | C1 |
| RM-19-11 | Table row match | Match against table rows | OD | C1 |
| RM-19-12 | Passage position prior | Answers tend to appear early | OD | C1 |
| RM-19-13 | Self-contained passage | Classifier: needs no outside context | OD | C2 |

The distinctness check compares RM-19-02 to RM-03-06. On the same
unit query vector (1,0) and target passage vectors (3/5,4/5) and
(0,1), both plain dense maximums are 3/5. For identical normalised
inputs the two definitions take the maximum of the same dot
products. In CT-03 keep the input vectors, passage order and
representation generation and change just the label and family.
RM-03-06 owns this definition. Keep RM-19-02 as a reference
without a second count. Refuse a second identity or family slot
for the same output even if each floor would pass. A different
mechanism needs its own identity and use proof under QR-234.

For RM-19-09, pass an exact native answering-post permalink under
0027 IT-0027-W1-ANSWERING-POST. Refuse a guessed fragment on the
same page even if its section text matches. The owned answering-post
rule still requires current revision, visibility and approval.
A fragment method has no separate qualifying evidence without
an independently distinctive method and proof of actual use
in that owned route.

### Family 20: Anchor text and link context

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-20-01 | Anchor naturalness | Fluency of the sentence with the anchor, under a small language model | PA | C1 |
| RM-20-02 | Anchor-title similarity | Anchor words against the target title | OD | C1 |
| RM-20-03 | Anchor specificity | Inverse document frequency of anchor words | PA | C1 |
| RM-20-04 | Anchor diversity budget | Penalise repeating the same anchor for one target | OD | C0 |
| RM-20-05 | Exact-match penalty | Penalise over-optimised exact-match anchors | OD | C0 |
| RM-20-06 | Anchor length preference | Prefer two to five words | OD | C1 |
| RM-20-07 | Sentence relevance | Relevance of the surrounding sentence | OD | C2 |
| RM-20-08 | Nesting guard | Anchor may not overlap an existing link | OD | C1 |
| RM-20-09 | Noun-phrase check | Anchor is a complete phrase | PA | C1 |
| RM-20-10 | Anchor ambiguity | Same anchor could point to several targets | OD | C1 |
| RM-20-11 | Anchor-entity agreement | Anchor entity matches the target entity | OD | C1 |
| RM-20-12 | Already linked | Target already linked elsewhere in the post | OD | C1 |
| RM-20-13 | Descriptive anchor | Classifier: descriptive against "click here" | OD | C1 |

### Family 21: Deduplication and canonical selection

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-21-01 | MinHash | Estimated Jaccard of shingle sets | PA | C1 |
| RM-21-02 | SimHash | Hamming distance of fingerprints | PA | C1 |
| RM-21-03 | Exact content hash | Identical text after normalisation | ST | C0 |
| RM-21-04 | Embedding near-duplicate | Cosine above a threshold | OD | C2 |
| RM-21-05 | Canonical by authority | Pick the highest-authority member of a cluster | OD | B |
| RM-21-06 | Quote-stripped duplicate | Compare after removing quotes | OD | C1 |
| RM-21-07 | Cross-post duplicate | Same content in two threads | OD | C1 |
| RM-21-08 | Template subtraction | Remove shared boilerplate before comparing | OD | C1 |
| RM-21-09 | Duplicate question | Paraphrase detection between questions | OD | C2 |
| RM-21-10 | Merge history | Use moderator thread merges as canonical hints | OD | C0 |
| RM-21-11 | Moved-thread follow | Follow redirects to the canonical thread | OD | C0 |
| RM-21-12 | Reciprocal loop guard | Avoid a link when the target already links back through an accepted link | OD | C0 |
| RM-21-13 | Near-twin target dedupe | Do not link to two near-identical targets | OD | C1 |

### Family 22: Originality and information gain

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-22-01 | Novel-entity gain | Entities in the target that the source lacks | OD | C1 |
| RM-22-02 | Novel n-gram share | Share of target n-grams absent from the source | OD | C1 |
| RM-22-03 | Topic divergence | KL divergence of topic distributions | PA | B |
| RM-22-04 | Set-cover redundancy | Redundancy against targets already linked | PA | C1 |
| RM-22-05 | Unique-solution count | Number of distinct fixes the target offers | OD | C2 |
| RM-22-06 | Specificity gain | Target is more detailed than the source mention | OD | C2 |
| RM-22-07 | Compression novelty | Normalised compression distance | PA | C1 |
| RM-22-08 | Surprisal | n-gram language-model surprisal of the target given the source | PA | C1 |
| RM-22-09 | Open-question coverage | Target answers questions the source leaves open | OD | C2 |
| RM-22-10 | Counter-evidence | Target adds a different viewpoint | OD | C2 |
| RM-22-11 | Source originality | Target is original, not copied | OD | C1 |
| RM-22-12 | First-source preference | Earliest member of a duplicate cluster | OD | C0 |
| RM-22-13 | Detail density | Numbers, steps and specifics per token | OD | C1 |

### Family 23: Query rewriting and expansion

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-23-01 | RM3 feedback | Pseudo-relevance feedback with a relevance model | PA | C1 |
| RM-23-02 | Rocchio expansion | Vector feedback from top results | PA | C1 |
| RM-23-03 | Thesaurus expansion | Synonyms from a licensed thesaurus | PA | C1 |
| RM-23-04 | Embedding neighbour terms | Nearest terms in embedding space | PA | C2 |
| RM-23-05 | PMI term expansion | Terms co-occurring on the site | PA | C1 |
| RM-23-06 | Spelling correction | Corrected query terms | PA | C1 |
| RM-23-07 | Lemmatisation | Normalised word forms (spaCy) | ST | C1 |
| RM-23-08 | Compound split and join | Wifi and wi-fi style variants | OD | C1 |
| RM-23-09 | Acronym expansion | Expand abbreviations | OD | C1 |
| RM-23-10 | Site jargon map | Terms learned from accepted links | OD | B |
| RM-23-11 | Key-term selection | Keep only the most informative terms | PA | C1 |
| RM-23-12 | Entity-aware rewrite | Replace pronouns with the entity | OD | C2 |
| RM-23-13 | Sentence to keyphrase | Turn the anchor sentence into a short query | OD | C1 |

### Family 24: Site-wide and section-level signals

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-24-01 | Section authority | PageRank of the target's forum or section | PA | B |
| RM-24-02 | Section focus | Topical entropy of the section | OD | B |
| RM-24-03 | Cross-section flow | Limit links between unrelated sections | OD | C0 |
| RM-24-04 | Section freshness | Recent activity in the section | OD | C0 |
| RM-24-05 | Section traffic share | Human visits to the section | OD | C0 |
| RM-24-06 | Click depth | Clicks from the home page to the target | PA | B |
| RM-24-07 | Orphan rescue | Target has no inbound links | OD | C0 |
| RM-24-08 | Hub coverage gap | Hub lacks links to member threads | OD | B |
| RM-24-09 | Section link balance | Even link density across sections | OD | B |
| RM-24-10 | Breadcrumb distance | Distance in the site hierarchy | OD | C0 |
| RM-24-11 | Cannibalisation risk | Two pages competing for the same query | OD | B |
| RM-24-12 | Section quality mean | Mean quality of threads in the section | OD | C0 |
| RM-24-13 | Size normalisation | Adjust for section thread counts | OD | C0 |

### Family 25: Page experience and usability

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-25-01 | Load-speed class | Target's load class from a crawl audit | OD | C0 |
| RM-25-02 | Mobile friendliness | Mobile layout passes basic checks | OD | C0 |
| RM-25-03 | Largest-paint class | Simulated largest-content timing class | OD | C0 |
| RM-25-04 | Interstitial presence | Intrusive overlays on the target | OD | C0 |
| RM-25-05 | Broken-link risk | Target's own outgoing 404 rate | OD | C0 |
| RM-25-06 | Redirect chain length | Hops before the target loads | OD | C0 |
| RM-25-07 | Accessibility score | Accessibility checks on the target | OD | C0 |
| RM-25-08 | Observed guest fetch | A guest crawl can read the target without logging in | OD | C0 |
| RM-25-09 | Page weight | Total bytes of the target | OD | C0 |
| RM-25-10 | Layout-shift risk | Elements that move while loading | OD | C0 |
| RM-25-11 | Secure content | HTTPS and no mixed content | ST | C0 |
| RM-25-12 | Reading-level match | Target's reading level suits the source's | PA | C0 |
| RM-25-13 | Ad density | Share of the viewport taken by ads | OD | C0 |

### Family 26: Transparent rule-based adjustments

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-26-01 | Relevance gate | Drop candidates below a topical relevance floor | OD | C1 |
| RM-26-02 | Same-thread exclusion | Never link a thread to itself | OD | C0 |
| RM-26-03 | Link count cap | At most n suggestions per post | OD | C0 |
| RM-26-04 | Minimum link spacing | Minimum tokens between suggested links | OD | C1 |
| RM-26-05 | Target blocklist | Owner-listed targets never suggested | OD | C0 |
| RM-26-06 | Permission exclusion | Locked, private and hidden threads excluded | OD | C0 |
| RM-26-07 | Language mismatch | Exclude targets in another language | OD | C0 |
| RM-26-08 | Recent-suggestion guard | Target already suggested for this author's recent posts | OD | C0 |
| RM-26-09 | Rel policy | Apply the owner's nofollow or sponsored policy | OD | C0 |
| RM-26-10 | Staff pin boost | Owner-pinned targets rank higher | OD | C0 |
| RM-26-11 | Deleted target guard | Never suggest deleted or unpublished targets | OD | C0 |
| RM-26-12 | Section allow-list | Only chosen sections can be targets | OD | C0 |
| RM-26-13 | Daily accepted cap | Daily limit on accepted links | OD | C0 |

### Family 27: Author and source provenance

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-27-01 | Topical expertise | Author's accepted answers on the topic | OD | C0 |
| RM-27-02 | Verified badge | Staff or verified status | OD | C0 |
| RM-27-03 | Author tenure | Time as an active member | OD | C0 |
| RM-27-04 | Asker follow-up | The original poster came back to the thread to follow up | OD | C0 |
| RM-27-05 | Quoted-source attribution | The post attributes its sources | OD | C1 |
| RM-27-06 | Vendor account | Official vendor account detection | OD | C0 |
| RM-27-07 | Documentation citation | Outbound links to official documentation | OD | C1 |
| RM-27-08 | Asker-confirmed solution | The asker confirmed the fix | OD | C0 |
| RM-27-09 | Style consistency | Stable writing style over time | PA | B |
| RM-27-10 | Contributor diversity | Several independent contributors in the thread | OD | C0 |
| RM-27-11 | Provenance chain | Source cites a primary source | OD | C1 |
| RM-27-12 | Moderator edit marker | Edited by a moderator for accuracy | OD | C0 |
| RM-27-13 | Newcomer caution | Low reputation lowers confidence | OD | C0 |

### Family 28: Multimodal and rich-media relevance

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-28-01 | Image alt-text match | Alt text against the anchor context | OD | C1 |
| RM-28-02 | Diagram or screenshot | How-to targets with helpful images | OD | C1 |
| RM-28-03 | Video caption match | Caption text against the context | OD | C1 |
| RM-28-04 | Attachment type | Logs and documents attached to the target | OD | C0 |
| RM-28-05 | Image-text similarity | Joint image and text embedding similarity | PA | C2 |
| RM-28-06 | Screenshot text match | Recognised text in screenshots | PA | C2 |
| RM-28-07 | Table presence | Tables suit comparison links | OD | C0 |
| RM-28-08 | Alt-text quality | Descriptive alt text | OD | C1 |
| RM-28-09 | Video transcript match | Transcript text against the context | OD | C1 |
| RM-28-10 | Rich-result markup | How-to or FAQ markup present | ST | C0 |
| RM-28-11 | Hotlink risk | Images loaded from other sites | OD | C0 |
| RM-28-12 | Gallery-only penalty | Thread with images and little text | OD | C0 |
| RM-28-13 | Download link presence | Files offered for the topic | OD | C0 |

### Family 29: Language, location and device context

Under DR-081, language facts in version 1 can enforce eligibility.
Research entries that need multilingual or per-market inputs stay
after version 1. Observed site facts give no locale preference or
market adaptation input. Of the 13 original rows below, DR-081
defers six. At most seven can enter version 1 from these rows,
before checking distinctness, use and individual qualification.
The family needs at least five further eligible methods to meet
its existing floor. QR-233 binds candidate design and proof under
0055. Keep the six deferred identities as deferred research.

| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-29-01 | Language identification | Detected language agrees | PA | C1 |
| RM-29-02 | Spelling variant | British against American spelling. After version 1 (DR-081). | OD | C1 |
| RM-29-03 | Regional availability | Product available in the reader's region. After version 1 (DR-081). | OD | C0 |
| RM-29-04 | Units and currency | Units and currency agree. After version 1 (DR-081). | OD | C1 |
| RM-29-05 | Time-zone relevance | Event times and zones agree | OD | C1 |
| RM-29-06 | Device context | Mobile against desktop issue | OD | C1 |
| RM-29-07 | Transliteration | Handles transliterated terms. After version 1 (DR-081). | OD | C1 |
| RM-29-08 | Script agreement | Same writing system | ST | C0 |
| RM-29-09 | Geographic proximity | Places named are near each other | PA | C1 |
| RM-29-10 | Regional rules | EU, UK and US rule agreement. After version 1 (DR-081). | OD | C0 |
| RM-29-11 | Browser and system tags | Browser and operating system context | OD | C1 |
| RM-29-12 | Tokenisation quality | Language-specific tokeniser confidence | OD | C1 |
| RM-29-13 | Reader locale preference | Locale mix from darshak aggregates. After version 1 (DR-081). | OD | C0 |

#### Research source and receiving boundaries

Use [QR-233](../registers/questions.md) for the seven version 1
research candidates below. Record existing source evidence, the
missing owning projection and the receiving operation separately.
A source that can produce an underlying fact keeps its existing
purpose and authority. The candidate record grants no new use.

| Research record field | Required evidence |
| --- | --- |
| `candidate_id` and `witness_ref` | Exact existing RM row and versioned synthetic witness. Bind comparison cases to the same input and output kind under 0055. |
| `source_refs` | Exact existing owner, record or fixture, version and permitted purpose for each source. A source lead or a synthetic fixture gives no permission to read live evidence. |
| `producer_binding` | Unbound for every row below. A future proposal must identify one owning operation, its exact output kind, range and schema, source provenance and finality, permitted site and subjects, source revisions and current rights. |
| `receiving_binding` | Unbound for every row below. A future proposal must identify one owning operation, exact input schema and purpose, scope, allowed effect, missing and withdrawn evidence behaviour. A proposed 0027 order-stage consumer needs its own 0055 admission. |
| `scope_proof` | Site, exact source and target identity and revisions where applicable, source and use policy, current authority and erasure fences, and evidence of eligibility for the declared use. Synthetic proof applies only inside its test fixture. |

Each row below identifies the source to consult and an open producer
obligation. The receiving operation is unbound in all seven cases.
The owners record an exact admitted binding, or keep the candidate
at offline research. The six deferred entries keep their decisions.

| Candidate | Existing source owner and unresolved producer obligation |
| --- | --- |
| RM-29-01 | 0018 and 0026 own language eligibility and its evidence. Establish a separate pair output and purpose beyond that eligibility check and the existing RM-26-07 exclusion. The current language decision keeps its sole authority. |
| RM-29-05 | 0098 F-0660 owns the confirmed event ID, revision and UTC window. 0084 F-0749 and F-0753 own the exact reported time, scope and original span. Identify an owning join for their current same-event projection and the exact report-time precision it can prove. |
| RM-29-06 | 0115 owns declared mobile and desktop Audit render profiles. Its viewport is a measured rendering input. Identify a source operation for exact document-authored device applicability and prove its authorised meaning before any pair use. |
| RM-29-08 | 0026 owns script and source-span evidence for text and anchor checks. Identify any distinct ordinary-text script projection with its exact scope and prove that it differs from the existing language and anchor checks. |
| RM-29-09 | 0049 owns the admitted identity match for named entities. Identify a rights-cleared place evidence and coordinate-precision profile for the exact entity revisions. Prove unknown and ambiguous identities are withheld without guessing a reader location. |
| RM-29-11 | 0084 F-0749 owns reported scoped claims and exact original spans. Identify an owner for the typed browser and operating system pair projection and completeness proof. Compare the existing RM-18-02 and RM-18-03 behaviour. |
| RM-29-12 | 0026 owns token offsets and the admitted pipeline capabilities. Identify the exact confidence output, calibration evidence and versioned scope for this proposed metric. Preserve the difference between tokeniser support and trained capability. |

Run specified local synthetic comparators in their own fixtures.
Hold rows awaiting an exact witness. For all seven rows an attempt
to read through an unbound receiving operation gives no candidate
value, order or setting effect. The study outcomes cannot set a
typed score or abstention mapping. If an owning pair of contracts is later
admitted, test its exact receiver with positive scoped proof and the
same value under a foreign site, other operation or changed source or
target revision. Hold the mismatched result before use. Repeat after
withdrawal and after restoring an older snapshot. Missing or refused
inputs stay unknown. Replay the same lineage without duplicating it.
For an admitted 0027 use, join the applicable RS check, RS00 and RS-REG.
A target below relevance stays outside the order stage. Compare cost,
usefulness and distinctness under 0055 before seeking qualification.
This record assigns no task, runner, ranking input or method count.

#### Offline event time evidence

Under [QR-233](../registers/questions.md), test existing candidate
`RM-29-05` with one current authorised own-site event. Feature F-0660
in [0098](0098-sponsors-crm-and-community-growth.md) provides its
exact event identity and revision, confirmed UTC start and end, and
explicit offset choice for a repeated local time. Use only its approved
public scope. Compare a report for that exact event and scope from an
eligible same-site British-English post. Features F-0749 and F-0753
in [0084](0084-research-sources-planning-and-outlining.md) keep its
reported time and original span separate from native post and observation
times. The synthetic fixture pins the event mapping and prelabels the
ordinary-text date, clock and explicit numeric offset. This tests the
time comparison. Claim extraction needs its own qualification.

Fix the synthetic event UTC window to 06 April 2030, 09:30–10:15.
The first report states 10:30–11:15 with a `+01:00` offset. The second
states the same date and clock times with a `+02:00` offset. Subtract
each explicit offset from its stated start and end. The first gives
09:30–10:15 UTC. That reported window matches. The second gives
08:30–09:15 UTC, so its window differs. An equal UTC start with a
different end differs as well. These values prove no named-zone
applicability.
Keep both exact source references and the reported scope on each result.
An observation of agreement conveys no independent verification.

Return unavailable for a month, date without times, missing or invalid
offset, unresolved event mapping, quote-only source or non-positive
duration. For a report that explicitly names a zone, require qualified
zone rules and a valid choice of offset for repeated time. A skipped
or unresolved local time supplies no window. Numeric offsets alone
leave named-zone applicability unknown. Withhold the result and its
source span for another site, obsolete revision, cancellation or lost
rights. A native post time, site zone or reader clock supplies no missing
report field. This probe uses no reader location, locale preference,
market data or outside calendar.

Hold native post dates, platform tags and version inputs equal in the
[0055 distinctness comparison](0055-method-research-catalogue.md).
Check the exact owning behaviour of `RM-09-02` age gap and `RM-09-09`
release-date alignment, and of `RM-18-01` version range overlap and
`RM-18-03` platform tag agreement. Those inputs preserve their own
verified meaning as the reported event windows differ. If an owning
candidate already compares the same confirmed start and end for the
same event scope, prove the duplicate and retain one method identity.
Verify remaining nearest neighbours independently under 0055. The
witness creates no action, setting, ranking input or qualified count.
An owning shipped use requires its own producer, consumer and evidence.

#### Offline language evidence cases

For RM-29-01, give two already eligible source-target pairs with
the same language agreement and different ordinary text. A
bare check that repeats the 0018 language decision or reverses
RM-26-07 adds no ranking method under 0055. Require independent
distinctness and an admitted use before any Family 29 count.

Test two research procedures against current eligible 0026
synthetic evidence. First, use complete sentences in ordinary
text zones, with the existing case fold and word segmentation.
Count full sentences matching the synthetic eight-word template
`the WORD WORD is in the WORD WORD`, where each WORD is one
ordinary word span. “The little seal is in the quiet room.”
gives 1/1, and “In is the quiet room the little seal.” gives
0/1 with the same words. A match only in a quote gives unavailable
if there is no eligible sentence. A nonmatch is no evidence of
a different language.

Second, remove each current voting paragraph in turn. Count
omissions where the English letter count remains positive and
strictly above each other language separately. Divide by the
number of voting paragraphs. Give English 60 and 60 letters
and French 44 for 3/3, then English 120 and French 22 and 22
for 2/3. With English 60 and 44 and French 44, one omission
ties and gives 2/3. Require at least two current voters.
An inherited short item with no voting rows gives unavailable.
The synthetic omission result leaves the ordinary 0026 tie
rule and eligibility decision in force.

Use current authorised evidence. Wrong site, stale revision
or incomplete voting evidence gives no result. Both cases
stay offline. To leave research, an approved owning feature
must bind the exact output, single producer and sole receiving
operation through 0055. Preserve 0018 language authority and
0027 filtering. An offline witness supplies no qualified count.

#### Offline browser and operating system pair evidence

Under [QR-233](../registers/questions.md), use an offline research
witness for existing candidate `RM-29-11`. Compare explicit browser
and operating system pairs for one named operation in already eligible
same-site British-English text. Use the current authorised revision
and original ordinary-text spans under [0026](0026-text-engine.md).
Reuse [0084's reported claim rules][browser-claim-source]
for exact scope and provenance. Prelabel the operation, browser, operating
system and explicit completeness cue in the synthetic spans. This tests
matching and its evidence bounds. Extractor qualification is separate.

The fixture uses synthetic browser IDs A and B and operating system
IDs X and Y. The source needs pair `(A, X)` for the named operation.
One target explicitly lists the complete pairs `(A, X)` and `(B, Y)`.
The other lists `(A, Y)` and `(B, X)`. Both targets have the same
individual browser tags and the same individual operating system tags.
The first reports a supported pair. The second reports exclusion for
that exact operation and pair within its stated complete list. These
results describe the stated scope. Actual applicability requires its
own proof.

Repeat the second case with no completeness cue, a different operation,
ambiguous pairing, conflicting statements or pairs found only in quoted
or excluded spans. Return unavailable in each branch. Separately offer
a stale revision, another site or withdrawn rights. Withhold the result
and source spans. An absent pair in partial evidence remains unknown.

Use the same inputs in the [0055 distinctness procedure][browser-distinctness].
Compare `RM-18-03` platform tag agreement and `RM-18-02` compatibility
table with their exact owned behaviours. Flat tag agreement ties the
targets. If either owning method already distinguishes the complete
pair relation in the same scope, record the duplicate witness and keep
one method identity. Verify other nearest neighbours under 0055 before
claiming any distinction. The probe uses synthetic document statements
only and creates no device observations, settings, live ranking inputs
or qualified count. A shipped use needs its own admitted producer,
consumer, exact scope and evidence under 0055 and QR-233.

### Family 30: Crawl, discovery and index-tier signals
[browser-claim-source]: 0084-research-sources-planning-and-outlining.md
[browser-distinctness]: 0055-method-research-catalogue.md


| ID | Method | Mechanism and signal | Basis | Cost |
|---|---|---|---|---|
| RM-30-01 | Index tier | Guest-visible and indexed status of the target | OD | C0 |
| RM-30-02 | Crawl depth | Depth from the home page in the crawl | PA | C0 |
| RM-30-03 | Noindex guard | Never link to pages marked noindex | OD | C0 |
| RM-30-04 | Canonical agreement | Target's canonical tag points to itself | ST | C0 |
| RM-30-05 | Sitemap presence | Target listed in the sitemap | ST | C0 |
| RM-30-06 | Crawl frequency | How often the target is revisited | OD | C0 |
| RM-30-07 | HTTP status | Target returns a plain success | ST | C0 |
| RM-30-08 | Index coverage | Target appears in the search engine's index | OD | C0 |
| RM-30-09 | Impressions without clicks | Search Console opportunity signal | OD | C0 |
| RM-30-10 | Crawl orphan | Target not reachable by crawling links | PA | C0 |
| RM-30-11 | Link discoverability | Link visible in the initial HTML, not only after scripts | OD | C0 |
| RM-30-12 | Soft-404 risk | Page looks like an error with a success code | OD | C0 |
| RM-30-13 | Response-time class | Server response time class | OD | C0 |

## Data

- The catalogue is this file: each row has an ID, method name, mechanism and
  signal, basis and cost class. The method sheets of spec 0055 hold the
  per-method fields, evidence and state.
- Scores and the recorded reasons for skips are produced by the pipeline of
  spec 0027; this spec adds no table of its own (proposed; confirm at the
  data review).
- Methods read guest-visible text and darshak's human-only aggregates. They
  read no private post, message or personal data.
- A batch method writes plain numbers that Rust reads (ADR-0021).

## Settings

This spec has no setting of its own. The floor of 12 per family and 360 in
all is a release requirement, not a setting, and nothing may lower it. Stage
budgets and guardrail switches belong to spec 0027, and admission settings
to spec 0055.

## Failure codes

The codes below are proposed. Codes SUT-4700 to SUT-4799 are proposed as the
range reserved for this spec; a decision record must confirm it first.

| Code | Meaning |
|---|---|
| SUT-4700 | Method skipped: its inputs are missing (it abstains) |
| SUT-4701 | Method skipped: the stage budget cut it |
| SUT-4702 | Method skipped: a guardrail was switched off |
| SUT-4703 | Method failed; skipped with a recorded reason (non-safety methods only) |
| SUT-4704 | Release blocked: a family has fewer than 12 qualified methods |
| SUT-4705 | Release blocked: fewer than 360 qualified methods in all |
| SUT-4706 | Row refused: duplicate mechanism and signal (spec 0055) |
| SUT-4707 | Score outside its range, or not repeatable with the same seed |
| SUT-4708 | Candidate refused: a safety or guardrail method failed |

## Security and privacy

- Only guest-visible text and darshak's human-only aggregates are used. No
  private post, message or personal data is read.
- Family 12 (Rakshak) output is a caution and never a sanction on a member.
- Learned methods never train on bar or sealed sets.
- Live ranking runs only Rust code; the Python sidecar writes numbers only.
- A method's reason is plain English a moderator can read, and holds no
  secret or personal data.

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| RM-R01 | The catalogue MUST list 30 families, each with at least 12 candidate methods, and at least 390 in all. | CT-01 |
| RM-R02 | Every row MUST have a unique ID, a method name, a mechanism and signal, a basis and a cost class. | CT-02 |
| RM-R03 | No two methods MAY share both mechanism and signal; a duplicate is refused by spec 0055's checks. | CT-03 |
| RM-R04 | A release MUST NOT proceed with fewer than 12 qualified methods in any family or fewer than 360 in total. | CT-04 |
| RM-R05 | A method MUST follow the shared contract: range, abstain on missing data, stable ties, determinism, reason string. | UT-RM-01 to 06 |
| RM-R06 | A skipped method MUST leave a recorded reason; none is skipped silently. | UT-RM-07 |
| RM-R07 | Live ranking MUST run only Rust code; batch results from the sidecar are read as plain numbers. | IT-RM-01 |
| RM-R08 | Family 12 output MUST be a caution and MUST NOT sanction a member. | BD-RM-01 |
| RM-R09 | Learned methods MUST NOT train on bar or sealed sets. | IT-RM-02 |
| RM-R10 | Family 8 MUST use human-labelled visits only. | UT-RM-08 |
| RM-R12 | A failed non-safety method MUST be skipped with SUT-4703. A failed safety or guardrail method MUST refuse the candidate with SUT-4708 and MUST NOT skip. | UT-RM-09 |
| RM-R11 | Each method MUST keep within its cost class budget on the reference 8 GiB machine. | PT-RM-01 |

## How it is tested

Everything here is designed, not run.

- **Catalogue tests (CT):** count families and rows, ID uniqueness, field
  presence, duplicate mechanism and signal, and the release floor rule, from
  the files themselves. CT-04 uses the 0064 Family 29 fixture to
  exclude the six deferred methods even when they have valid
  research evidence for their own scope.
- **Unit tests (UT-RM):** for every method, hand-worked cases whose answers
  come from the source paper's worked example or from a hand calculation, not
  from the implementation. Edge cases: empty text, one word, the longest
  allowed text, missing signals, ties, invalid encodings.
- **UT-RM-09:** force a non-safety method to fail and see SUT-4703 and the
  pipeline continue. Force a safety method and a guardrail method to fail
  and see the candidate refused with SUT-4708, never skipped.
- **Property tests (PT-RM):** scores stay in range; swapping equal inputs gives
  equal scores; the same seed repeats on both systems; cost stays in budget.
- **Behaviour (BD-RM):** a moderator sees a plain reason for each suggestion;
  a cautioned link shows the Rakshak reason, not an accusation.
- **Integration (IT-RM):** a pipeline run on a synthetic site calls every
  applicable method or records why not; no live call touches Python.
- **Per-family generated tests:** each family has a generator whose inputs and
  oracles are defined in its method sheets, such as graph families checked on
  small graphs with known answers.
- **Quality evidence:** per-method ablations in the lab (spec 0056) and the
  quality bars of spec 0057. A method with no measured use is not counted.

## Changes from the source

This list is new. It replaces the counting machinery of specs 0067 to 0071.
The shared contract and the distinctness rules reuse spec 0055's. Spec 0055's
note that catalogue counts are not a release gate is narrowed: the ranking
floor of this spec is a release requirement alongside the quality bars.
