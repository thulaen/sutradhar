# 0023: Matching and anchors

Matching finds, for each source post, the destinations worth linking
to and the existing words that would carry each link. It searches the
index that reading (spec 0022) builds, reranks the best candidates and
makes suggestions in real time, per post. It is roadmap Phase 4
item 3.

## Words used here

- Exact search: evaluating every eligible stored entry for the
  representation being searched, with no approximate structure that could
  miss one. Dense, sparse and multi-vector searches have separate typed
  semantics and receipts.
- Embedder: pinned `BAAI/bge-m3`, revision
  `5617a9f61b028005a4858fdac845db406aefb181`, through spec 0024's
  admitted package interface. It supplies the default dense and mandatory
  sparse and multi-vector representations. Optional paid embeddings follow
  spec 0025 and never mix with BGE-M3 dense in one dense-index generation;
  the active hybrid candidate still includes BGE-M3 sparse and
  multi-vector representations in the same complete hybrid generation.
- Thread-level index, per-post index and coverage: as in spec 0022.
- Index version: the serving generation's id with its complete active
  dense, BGE-M3 non-dense, tokenizer, representation and spaCy
  compatibility keys. Model versions are the immutable keys of the
  admitted artefacts used.
- Live queue and backfill queue: new and edited posts, always first;
  and the historical catalogue, in order of thread views and recency.
  Reading calls them its live and backlog lanes. Spec 0019 owns their
  PostgreSQL queue, transaction outbox and Rust scheduler.
- Server-sent events: one open web connection over which updates keep
  arriving at a page.
- Destination: a thread, resource or hub page a link could point to. A
  candidate is a destination being considered for one source post.
- Reranker: pinned `BAAI/bge-reranker-v2-m3`, revision
  `953dc6f6f85a1b2dbfca4c34a2796e7dde08d41e`, reading one source and
  candidate pair through the admitted package interface. It is the only
  production cross-encoder route.
- Anchor: the existing words in the source post that become the link.
- Named entity: a name such as a person, organisation, product, place
  or event. Noun phrase: words built around a noun ("the spare power
  supply"). Keyphrase: a phrase that sums up a post.
- Recall@40: the share of judged relevant destinations present in the
  first 40 retrieval results, defined and labelled under spec 0057.
- MS MARCO: a public collection of search questions whose terms say it
  is for non-commercial research.

## What it does

### Scope

In scope: exact BM25, active dense and BGE-M3 non-dense retrieval; hybrid
fusion; candidates and hard filters; anchor candidates and exclusions;
cross-encoder reranking
of the champion's bounded candidate set within 8,192 tokens; suggestion
records; real-time suggestions with the two queues, re-scoring and live
delivery; the 30-second first suggestion; model changes; safeguards for
people's names; and frozen component evaluation.

Out of scope: reading, both indexes, queue order and the index budget
(spec 0022); model files and the memory table (spec 0024); progress,
the live stream and visuals (specs 0016 and 0017); the other ranking
methods and the final ranking (spec 0027); the review page, approval,
insertion, undo and expiry (spec 0028); wrapping the link tag
(spec 0026); release quality judging (spec 0057).

### Pipeline for one source post

1. Preconditions: the post is guest-visible and English; the active dense
   embedder and BGE-M3 have written the complete dense, sparse and
   multi-vector set, and spaCy its required language features in the
   serving generation; the thread-level index is complete; no embedder
   rebuild is running; and every serving key matches the active admitted
   artefacts. Otherwise matching waits, visibly.
2. Exact BM25, dense, sparse and multi-vector searches run for each
   source piece over the whole thread-level index and the per-post index
   as far as built. Coverage is recorded separately by channel.
3. Hits are grouped into destinations and fused by the versioned hybrid
   rule below.
4. Hard filters remove ineligible destinations, with reasons kept.
5. Anchor candidates are found once for the post; each destination gets
   its best acceptable anchor or drops out.
6. The champion's bounded reranking set is scored by the BGE cross-
   encoder, best first.
7. Suggestions are assembled with model and index versions and the
   coverage, and handed to ranking (spec 0027) and the review page the
   moment the post completes, never per batch. They stay "analysis
   continuing" until every applicable method has finished.

### Exact hybrid search

- BM25 evaluates destination title, body, heading and phrase fields with
  the champion's locked `k1`, `b` and field weights. Tantivy executes the
  lexical index; early termination or another approximate top-k shortcut
  is forbidden because it could hide an eligible exact-match destination.
- Dense vectors are normalised and compared by dot product with every
  eligible dense entry: destination dense summaries and piece and extras
  vectors as far as built. Blocked arithmetic must agree with the
  one-by-one reference within the frozen rounding tolerance.
- Sparse search evaluates the pinned BGE lexical-weight scoring contract
  for every eligible source and destination entry. An inverted index may
  avoid arithmetic for entries whose score is exactly zero, but may not
  drop a non-zero match or introduce approximate scoring.
- Multi-vector search evaluates the admitted BGE late-interaction
  contract between every eligible source and destination token-row set,
  respecting masks and source-piece identity. Any batching or blocking is
  exact and checked against a small exhaustive reference. A dense score
  is never substituted for late interaction.
- Extras scores are multiplied by the extras weight (0.5) before ranking
  within each representation. Removed rows are skipped and current
  visibility is checked again after search.
- Each required BM25, active dense, BGE-M3 sparse and BGE-M3 multi-vector
  channel keeps the champion manifest's bounded candidate count per source
  piece, within spec 0077's 20–500 range.
  The initial champion's exact per-channel counts are mandatory readiness
  values; implementation is blocked if its manifest omits one. Ties break
  by canonical destination and row id. Every search carries the complete
  generation and representation keys; a mismatch is refused. Unbuilt
  entries are coverage gaps, never negative evidence.

### Destinations and their scores

- Hits group into destinations: a thread (replies belong to it), a
  resource or a hub page. Redirects and merges resolve to the canonical
  destination. The candidate set is the union of BM25, dense, sparse and
  multi-vector hits, with source and channel provenance retained.
- Initial fusion uses reciprocal rank fusion over those four ranked
  channels; the dense channel also includes the dense thread-summary
  evidence: `score(d) = sum(weight_r / (rrf_k + rank_r(d)))`. The champion
  configuration pins `rrf_k` in spec 0077's 1–120 range and normalised,
  strictly positive production weights for BM25, active dense, BGE-M3
  sparse and BGE-M3 multi-vector. Zero is diagnostic only and cannot
  disable a required production channel. A list with no hit for `d`
  contributes nothing; it does not fabricate a similarity of zero.
  Duplicate hits for one destination within one list collapse to its best
  rank before fusion. Spec 0077 alone changes or promotes the parameters.
- A destination with no per-post pieces yet can qualify only through its
  complete thread-level entries; the suggestion records that coverage.
  Its dense summary participates only in the dense channel; sparse and
  multi-vector evidence comes from the title, first post and accepted
  answer as stored by spec 0022.
- The target is normally the thread. It is a specific reply when the
  single best dense-scoring piece lies in that reply and beats the first
  post's best dense-scoring piece by at least 0.05. This target-selection
  margin does not replace hybrid fusion or the relevance gate. Duplicate
  checks still use the thread.

### Hard filters before reranking

A destination is removed, with its reason recorded, when:

- it is not guest-visible now, or is deleted, soft-deleted or awaiting
  moderation;
- the active dense producer has not written its complete title and first
  dense summary representations, or BGE-M3 has not written the required
  sparse and multi-vector title representations, in the serving
  generation, or any required spaCy language feature is stale or missing;
  its other posts need not be read yet, but their absence remains explicit
  coverage;
- the source or destination fails spec 0018's current British-English
  eligibility check;
- it is the source's own thread, or a thread the source post quotes;
- the source thread already links to it (bare URLs and links inside
  quotes count), or a suggestion for that pair is waiting for review;
- the connection was removed and is suppressed, or was rejected and
  neither side has changed meaningfully since;
- it is not a permitted kind: threads; resources with Resource
  Manager; hub pages with the hub module; never tags, sections or member
  pages.

A resource and its discussion thread count as one destination, and the
resource is offered. A source thread that has used its link allowance
shows "allowance used" and is not matched; existing links count
towards the allowance, except links inside quotes.

### Anchor candidates

Found once per source post from reading's stored spans, from four
sources:

1. Named entities of allowed types from the pinned packaged spaCy
   pipeline (spec 0026), mapped as person; organisation; product, brand
   or object; place; or event. Brands labelled as organisations count as
   brands too. Unknown or unmapped source labels are refused, not guessed.
2. Keyphrases from two methods, merged. Statistical candidates (one to
   four words between stop words and punctuation, scored on frequency,
   first position, spread and capitals) are re-scored with the active
   dense representation (the phrase dense vector's similarity to the
   post's own dense piece). The merged score is half of each; the top 20
   per post are kept. The statistical method is written by us from
   published descriptions after a rights review, or comes from an admitted
   library.
3. Destination title phrases: supported spaCy noun chunks and entities
   of a candidate destination's title found in the source, on whole words
   and regardless of case, keeping the post's own case.
4. Noun chunks where the admitted pipeline supports them, without
   leading words such as "the" or "my". Unsupported noun chunks produce
   no substitute span source.

A span found by several sources becomes one candidate keeping all its
source labels.

### Never an anchor

Hard rules:

- text in quotes at any depth, quote attributions, code or spoilers;
- posts skipped as not English;
- extras, mentions, the text of existing links, bare URLs, text inside
  a URL;
- censored words and unknown add-on BBCode content;
- part of a word ("cat" in "category"), or a cut through a combining
  sequence;
- a span the placement check (spec 0026) cannot wrap safely, such as
  one crossing a formatting tag unevenly;
- generic phrases on a reviewed stop list ("this", "here", "the
  thread", "click here"), and numbers or dates on their own;
- an entity of a type that is not allowed, whichever source found it;
- a member's user name;
- under 3 or over 80 characters, or more than 8 words.

Only existing words are linked: an anchor's text must equal the stored
characters at its offsets, and no text is added, removed or changed. A
phrase that occurs several times is offered only at its first eligible
place. Overlapping candidates are cut to a non-overlapping set,
keeping the higher scores.

### The best anchor for each destination

- Anchor score: 0.6 × the similarity of the active dense phrase vector to the
  destination's title or summary vector (the higher), + 0.3 × the share
  of the phrase's words in the destination title, + 0.1 × the
  keyphrase score where there is one.
- Active dense phrase vectors are made once per post, in one batch.
- A destination's anchor is its best candidate above the minimum
  anchor score. A destination with none drops out: the system abstains
  rather than change any text.
- The source sentence for reranking is the one holding that anchor.

### Reranking

- `BAAI/bge-reranker-v2-m3` at revision
  `953dc6f6f85a1b2dbfca4c34a2796e7dde08d41e` scores eligible fused
  destinations that have an anchor. The initial champion takes 50 or all
  when fewer remain. Spec 0077 may tune that count only within 10–100
  and never above the unique fused candidate count. It is a versioned
  optimisation parameter, not an ordinary staff setting.
- Query side, at most 1,024 reranker tokens (an eighth of the window):
  the source thread title, then a local evidence window around the
  anchor. Take the source sentence whole; if room is left, add its
  nearest neighbours in the paragraph, one each side in turn. If the
  sentence alone is over the room, take the anchor and whole words on
  each side, the same number each side where the text allows.
- Document side: the destination title; then its pieces most similar to
  the source piece holding the sentence, best first, from the pieces held
  so far: up to 3, as many whole pieces as fit in the pieces' room,
  three-eighths of the window (3,072 reranker tokens); then summary
  pieces.
- Pieces are sized in active dense tokens (spec 0022), and the
  cross-encoder counts in its own tokenizer. The piece size has no fixed
  top (owner, 10 November 2026: "No fixed top"). Its measured reranker-token
  mapping
  is recorded with the generation; an estimate is not used. The best
  piece always enters. If it exceeds the pieces' room, split it at
  sentence boundaries, or words for an over-long sentence, into parts
  that each fit. Other pieces join only whole. Summary pieces are whole
  first-post and accepted-answer pieces, most similar first; the first
  post fills at most half that room. No piece repeats.
- Complete reading and local reranking evidence are different receipts.
  Spec 0022 reads and represents every authorised post in full. A cross-
  encoder pair contains only the explicit local query and document spans
  above. Its receipt records included offsets, omitted coverage, tokenizer
  digest and model key. A pair score is local model evidence, never a
  claim that the cross-encoder read the whole source or destination, and
  never factual evidence by itself. Ranking must retain the complete-
  reading receipt and coverage; if omitted context is necessary for a
  protected relevance decision, it abstains or blocks rather than
  treating the local pair as complete.
- Piece and part scores are aggregated by the champion's spec 0077 source
  aggregation rule: maximum initially, or the admitted mean-of-best or
  reciprocal-source fusion challenger. The selected rule and all source
  scores remain replayable.
- Tokens are counted with the pinned reranker tokenizer. An input over
  8,192 tokens, markers included, is refused with an error code; no text
  is silently dropped to make it fit.
- Pairs run in fused retrieval order, best first, one pair per model call
  in backfill work, and each result is published as it arrives. Pinned
  Python packages execute the model through spec 0024's admitted
  interface; no first-party Python production adapter or second reranker
  is introduced.
- Each score is stored with the reranker package, artefact and tokenizer
  compatibility keys and the candidate manifest digest.

### Suggestions

- Grouped by anchor phrase: up to five destinations per phrase, best
  reranker score first. Each must pass the minimum reranker score.
- Each records: source post and content version; anchor text, offsets
  in code points and source labels; a people flag; per destination, its
  kind, canonical id, retrieval, anchor and reranker scores, best
  pieces and whether all its posts were read; the model versions and
  index version; the coverage; the settings version; and its state.
- States: analysing (with what it waits for), complete, replaced (by a
  re-score) and shadow (stored, not shown, for testing). Approval,
  rejection and expiry are spec 0028's.
- Suggestions are never held for a later model change. The versions a
  suggestion records matched when it was made, so a complete
  suggestion stays approvable whatever model is active later. Approval
  rests on spec 0028's re-check of content, offsets and visibility.
  (The opt-in hold-back test of links in spec 0029 is a separate
  matter.)
- While a post is analysing, "k of 50 reranked" and the methods still
  running appear in its analysis row (spec 0016), to anyone with
  review-page permission.
- Rejected candidates keep their reason for 30 days, for the "Why no
  suggestions?" and anchor heat-map views.
- Moderators approve every link. The minimum anchor and reranker
  scores are calibrated on Dell before release; until then matching
  runs in shadow only.

### Live delivery and queue figures

- Each post's suggestions go to the add-on the moment the post
  completes, and every later change goes the same way.
- Once the add-on has stored them, an event on the `suggestions` topic
  of the live stream (spec 0016) reaches every open review page whose
  viewer has review-page permission. An event carries ids and state
  only; the page then fetches the suggestion through the add-on, which
  checks permission and visibility. If streaming fails, polling takes
  over.
- The review and health pages show live and backfill queue depth, the
  processing rate (posts finished a minute, measured) and both
  coverages. The service publishes them as metrics and on the stream.

### The two queues

- Nothing is suggested until the thread-level index is complete
  (estimated at a day or two for 50,000 threads). Progress shows it.
- Then suggestions start at once, from both queues, while reading
  builds the per-post index.
- Live queue: new and edited posts, worked within seconds and first;
  each is read, then matched.
- Backfill queue: the catalogue, in views-and-recency order. Each post
  is read and, if it can take a link (below), matched; its suggestions
  appear the moment it completes.
- Reading is not held back by matching: backfill model calls alternate
  between reading and matching while both have work, so the per-post
  index completes in about twice reading's own time.
- Every queued item carries its queue time, so queue age is measured
  as well as depth.

### Re-scoring once the per-post index is complete

- A suggestion made with incomplete coverage is approvable as usual
  and records its coverage.
- When reading signals completion, every such suggestion still waiting
  for review is re-scored, in the backfill queue ahead of new backfill
  matching, in the same order: searched again over the whole index and
  the champion's bounded reranking set scored. A pair whose input is
  unchanged keeps its score.
- Per anchor phrase, if a destination's reranker score beats the
  current best by at least `matching.rescore.replace_margin`, the
  suggestion is replaced in one step: it becomes "replaced", points to
  its successor, and the successor reaches the review page live.
  Otherwise it stays, with the re-score and full coverage recorded.
- Approved, rejected and removed suggestions are not re-scored. An
  approval that arrives before a replacement wins.
- Progress shows the re-scores left, with a forecast.

### The first useful suggestion within 30 seconds

The service level is at least 99% of new posts within 30 seconds from
save to first useful suggestion on the declared reference profile of 4
processor cores and a SATA solid-state disk. Edited posts are measured
separately under the same floor. This is a mandatory qualification gate,
not an estimate or a target that a slower model choice may waive.

Parameter counts and dense-only disk arithmetic are not timing evidence.
The release receipt measures, for the exact champion and package lock:
receipt and parsing; active dense, BGE-M3 non-dense and spaCy work; each
hybrid retrieval channel; candidate fusion; anchor construction; each
cross-encoder pair; queue wait; storage; and delivery. It includes dense,
sparse and multi-vector index sizes, disk cache state, concurrent
backfill, package and bridge memory, p95/p99 latency and the complete
candidate count.

- Admitted models stay loaded; nothing loads per request.
- The live queue goes first; backfill yields after each admitted model
  call and under queue-fairness or memory pressure.
- Reranking runs best first. The first candidate that passes the minimum
  score and every hard rule available so far may appear as provisional,
  marked "analysis continuing". Approval stays off until every candidate
  in the champion's reranking set and every applicable method has
  finished and every hard rule has passed.
- "No suggestion yet, analysis continuing" is an honest waiting state,
  not a useful suggestion and not evidence that the service level passed.
- Until the per-post index is complete, each suggestion shows its
  coverage. Incomplete coverage never counts as a negative relevance
  result.
- Live calls receive their admitted resource share. If the target still
  fails, the exact shortfall blocks release; reducing source reading,
  dropping a retrieval channel, weakening the relevance gate or excluding
  a supported model profile is not a permitted remedy.

Qualification uses at least 100 new posts, with at least 99% at most 30
seconds, on the controlled reference profile with the qualified complete
index and backfill running. Every post's time is recorded. A miss blocks
release. Posts saved during a model-rebuild pause are reported in a
separate recovery measure and matched first after the switch; they cannot
be hidden from the recovery report or used to make the steady-state
figure look better.

### Existing posts as sources

- Existing posts become sources in the backfill queue, in
  views-and-recency order. Backfill duration and forecasts come only from
  measured champion throughput on the exact corpus and machine profile.
- A post goes to the cross-encoder only when it can host a link: it has
  anchor candidates, its thread has allowance left, and at least one
  eligible destination has an acceptable anchor in it. Posts such as
  "Thanks!" are read into the complete index but need no cross-encoder
  pair. This avoids unnecessary work without truncating reading or
  weakening candidate evaluation for eligible posts.

### New destinations

When reading finishes a new thread, the 200 existing posts nearest to
its summary vector (exact search) are queued as sources in the
backfill queue, never the live queue, in their threads'
views-and-recency places, so older posts can gain links to new
content.

#### F-0002 verification binding

`REQ-0023-REVERSE-SOURCE` uses `DES-0023-NEW-THREAD-QUEUE` in this
section. When 0022 finishes reading a new eligible thread, query the
compatible serving index by exact summary-vector distance. Enqueue at
most the existing 200 nearest older posts for the established backfill
lane. Keep their existing thread order by views and recency. Distance
selects sources for evaluation. The ordinary eligibility, anchor and
relevance checks still decide which suggestions enter 0028 review.

`VAR-0023-REVERSE-XENFORO-THREAD` covers the native XenForo 2.3
thread trigger and the matching preconditions in this spec. Bind the
same-site thread ID and revision to its current index generation and
apply the existing worker and review authority. A private or stale
target yields no current suggestion. An incompatible generation holds
matching with its existing readiness reason. Existing queue receipt,
resource and effect rules remain with 0019, 0022, 0027 and 0028. This
route adds zero public-page queries, time or bytes and makes no site edit.

`IT-0023-REVERSE-QUEUE` binds existing tests 20–22 under How it is
tested. For a synthetic eligible new thread without distance ties,
admit the exact 200 of 201 older posts and dispatch them through
backfill. Live work keeps its own lane and priority. Include a
source with no usable anchor, an irrelevant popular source and a
target withdrawn before display. Require current eligible entries.

The register title says "new content". This section defines a new
thread trigger. A WordPress post, edit, reply or resource trigger needs
its own exact source and destination contract before it counts. Pin
stable ties at the exact 200-source boundary and replay of the native
read trigger before derivative work. The cases above are designs.

### Model and configuration changes

- Every suggestion records the complete candidate manifest, model,
  package, tokenizer, spaCy feature and index versions it used and is
  never held for a later change.
- An embedder change that needs a new index, including an active dense
  identity change, immutable BGE-M3 non-dense revision or tokenizer
  change, or admitted paid-route change, builds a separate generation
  under spec 0024 while the old stays. New suggestions pause from both
  queues and show "not ready for suggestions: model update running".
  Existing suggestions remain approvable. When the new generation is
  whole, spec 0077's controller decides promotion or rollback; matching
  resumes only after the atomic serving-generation switch and protected
  readiness checks. No suggestion uses a mismatched index.
- Before each matching step, active dense, BGE-M3 non-dense, tokenizer,
  representation, reranker, spaCy and manifest keys are compared with the
  serving generation. Any difference blocks new suggestions with a plain
  reason and code; earlier valid suggestions stay approvable.
- A cross-encoder change needs no embedding rebuild, but still requires
  spec 0077 evaluation and promotion. A spaCy pipeline or feature-schema
  change requires affected language features to be rebuilt; posts with
  stale features get no new anchor candidates. Neither route switches on
  a package self-test alone.
- A rollback restores the complete prior champion and its compatible
  generation. Readiness returns only after every key and protected check
  passes; the switch itself is not proof of readiness.

### People's names as anchors

1. Only names already in the guest-visible source text, never in
   quotes, mentions or signatures.
2. Never a member's user name: the add-on checks each candidate name
   (`MemberNameCheck`) before it is shown; mentions are never anchors.
3. The destination must be about that person: the same name is in its
   title or first post.
4. The suggestion is marked "person" on the review page.
5. A setting turns people anchors off for a site.
6. Person entity records are purged with their posts and with erasure
   requests, and shown only on admin pages.

### Frozen reconstruction and component evaluation

- The production evaluation uses the admitted active dense producer
  (BGE-M3 by default or the sole exact paid-dense exception), pinned
  BGE-M3 sparse and multi-vector producers, and the pinned BGE
  cross-encoder through their admitted routes. There is no Qwen, potion,
  GTE, MiniLM or other second production retrieval or reranking stack.
  A diagnostic ablation may disable one channel or the cross-encoder, but
  it is never eligible for promotion when a required channel is absent.
- Data: link reconstruction on the site's guest-visible English content
  on Dell. Members' internal links are hidden in a test copy (words stay,
  link removed), and each configuration tries to recover the destination
  and anchor. At least 2,000 links are sampled with a fixed seed from the
  50,000-thread tier across subgroups, kept apart by thread and time from
  tuning data; the unseen final assessment data is never used.
- Subgroups: thread type, post length, destination popularity (inbound
  links, reported separately because reconstruction favours popular
  threads), forum section and every other stratum required by spec 0057.
- Frozen protocol: before any score, a protocol file records exact model,
  package and configuration identities, sampling, metrics, subgroups,
  interval method and controller inputs. Its SHA-256 is recorded. A run
  with a different checksum is invalid. Hard cases are never dropped and
  the sample is never redrawn to get a pass.
- Component diagnostics: BM25, active dense, BGE-M3 sparse and BGE-M3
  multi-vector channels report retrieval Recall@40 and candidate coverage;
  fused retrieval reports MRR and Recall@40; the cross-encoder reports
  ranking changes and severe local-evidence failures; anchors report exact
  and overlapping agreement; final ranking uses spec 0057's nDCG@10,
  Precision@6, link-suitability, calibration and severe-failure measures.
  Latency, memory and throughput include the complete package and
  representation set.
- Link reconstruction is diagnostic evidence, not the release metric or
  a second promotion rule. Spec 0057 owns labels and thresholds. Spec
  0077 alone compares champion and challenger and decides `PROMOTE`,
  `KEEP_CHAMPION`, `ROLL_BACK` or `BLOCK_FOR_DATA_QUALITY`. Mandatory
  nDCG@10 ≥ 0.95, the per-forum Precision@6 improvement and every
  protected gate apply regardless of a reconstruction win.
- The report names exact revisions and stays on Dell; a public summary
  needs a privacy review. A window below the owner's 1,024-piece-token or
  8,192-reranker-token floor is not a candidate without a new owner
  decision and ADR.
- Each model's admission record states whether its training data includes
  MS MARCO and quotes its terms. That provenance is recorded, not treated
  as legal clearance or as permission to weaken independent evaluation.

### Data stored, and uninstall

- Anchor candidates per post, with spans, sources and scores, rebuilt
  when the post changes; active dense phrase vectors, kept until it changes.
- Suggestion records with scores, versions, coverage, any re-score or
  replacement, and rejection reasons.
- The comparison protocol, results and evidence, on Dell only.

All of it is derived from guest-visible content and purged with it.
Active dense phrase vectors are always deleted at uninstall with no export.
Anchor-candidate records, suggestion records and rejection reasons keep
the export offer and keep-or-delete choice (spec 0024's table).

### Messages between the service and the add-on

Signing and schemas are spec 0021's.

- `SuggestionUpdate` (service to add-on): one post's new suggestions,
  sent the moment it completes, and later state changes, re-scores and
  replacements.
- `VisibilityCheck`: source and destination ids, to confirm guest
  visibility just before display.
- `MemberNameCheck`: candidate names, normalised; the add-on answers
  which equal a user name, and those are dropped. The add-on never
  sends the member list.
- `Readiness` (service to add-on and the health page): ready, and ready
  or not for suggestions, with reason and code, including "model
  update running".
- Queue figures: queue depth and oldest age, processing rate, and both
  coverages, as metrics and on the live stream.
- `ReviewDecision` (add-on to service, collected by the service):
  approve, reject or remove with any reason, for learning in the small
  networks (spec 0024).

### Settings and defaults

Fixed product rules: exact BM25, active dense and BGE-M3 non-dense
channels; complete-reading and
local-reranking receipts; up to 3 best whole pieces; the 8,192-token
window; the four anchor sources; allowed entity types; merged keyphrases;
moderator approval of every link; suggestions never held for a later
model; the thread-level index first; two queues; and the embedder-rebuild
pause.

Candidate counts, channel weights, RRF constant, cross-encoder source
aggregation and inference batching are read-only values from the current
champion manifest, not site settings. The initial cross-encoder candidate
count is 50. Spec 0077 supplies their bounds and is their only change
route. Spec 0057 owns evaluation labels and protected thresholds.

| Setting | Default |
|---|---|
| `matching.target.reply_margin` | 0.05 |
| `matching.rerank.min_score` | calibrated under spec 0057 |
| `anchors.min_score` | calibrated under spec 0057 |
| `matching.rerank.first_post_share` | 0.5 |
| `anchors.score.weights` | 0.6, 0.3, 0.1 |
| `anchors.keyphrase.weights` | statistical 0.5, active dense 0.5 |
| `anchors.keyphrase.max_per_post` | 20 |
| `anchors.length` | 3 to 80 characters, at most 8 words |
| `anchors.alternatives_per_phrase` | 5 (never above 5) |
| `anchors.people.enabled` | true |
| `matching.reverse.sources_per_destination` | 200 |
| `matching.rejections.keep_days` | 30 |
| `matching.rescore.replace_margin` | calibrated under spec 0057 |

Live approval remains unavailable until the required calibrated values
and protected quality gates pass. Settings move only within approved
bounds and cannot alter the champion manifest directly.

### Failure behaviour

Shown in plain English with an error code and a manual link; nothing
missing is invented or counted as zero.

- Reranker missing or failed: search and anchors still run;
  suggestions stay "waiting for reranker" and cannot be approved.
- Embedder missing or failed: nothing new is matched; "not ready".
- Entity records missing or stale: no anchor candidates for those
  posts; suggestions already made are not held.
- Thread-level index not complete: nothing is suggested; progress
  shows its build.
- Embedder rebuild running, or key mismatch: new suggestions pause or
  are blocked; earlier ones stay approvable.
- Live stream unavailable: polling takes over; nothing is lost, since
  the add-on stored it first.
- A vector segment unreadable: matching stops, readiness says so, and
  reading rebuilds the segment from stored pieces.
- Visibility check failed or timed out: nothing is shown; it retries.
- A pair over 8,192 tokens: refused with a code and reported as a
  defect.
- Memory pressure: backfill matching pauses before the cap; live
  reranking runs one pair at a time.
- Live times rising: backfill pauses until they fall.
- Crash during reranking: finished scores are kept; at most the pair
  being scored repeats.
- No candidate: "no suggestion", with the main rejection reasons.

### Privacy and security

- Only guest-visible content is matched, checked again before display
  and at approval.
- Logs, metrics, traces, progress and `suggestions` events carry ids,
  counts, durations and codes, never post text, anchors or names. The
  analysis row shows counts only. A review page fetches text through
  the add-on, with permission and visibility checked.
- Comparison data stays on Dell.
- No model downloads code or calls the network during matching.

## Limits

- Memory stays inside AGENTS.md section 5.6 and includes all admitted
  local model packages and runtimes, active dense and BGE-M3
  representation buffers, BM25 and hybrid candidate sets, cross-encoder
  inputs and outputs, index buffers and disk-cache allowance. The initial
  backfill profile runs one pair at a time; every declared machine profile
  measures the full 8,192-token pair and concurrent live workload. No
  unverified attention or memory-saving feature is assumed.
- Frozen component evaluation runs on Dell through spec 0059's sealed
  heavy-job route and spec 0077's registered evaluation workflow.
  The champion is
  also measured under the production cap on every declared owner-PC
  profile, not inferred from Dell's development-job limits.
- Reranker input is at most 8,192 tokens; query side at most 1,024.
- Each required retrieval channel uses its champion count within 20–500;
  cross-encoder pairs use the champion count within 10–100, initially 50.
  Other caps are 5 destinations per phrase, 20 keyphrases per post and
  200 reverse sources per new thread.
- Service level: at least 99% of new posts within 30 seconds on 4 cores
  and a SATA solid-state disk, with no slower-profile waiver.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Test first. Unit tests need no download: fake BM25, active dense and
BGE-M3 non-dense channels, a fake cross-encoder giving fixed scores, and
stored spaCy feature spans stand in for the packages.

- Each channel is checked against a plain exhaustive reference: BM25
  field scoring, dense dot products, sparse lexical weights and multi-
  vector late interaction. Fusion tests use known ranks, duplicates,
  missing channels and boundary weights.
- Property tests: anchor text equals the source characters at its
  offsets; anchors never fall in excluded zones and never overlap; at
  most five destinations per phrase; channel and reranker counts stay in
  the champion registry; every reranker input is within 8,192 tokens;
  local evidence receipts identify all included and omitted spans; and
  no pair is labelled as a complete source reading.
- Fixtures: the synthetic 300-thread fixture gains destinations that
  trip each filter and exclusion, people's names that are user names,
  products named like members, censored words, quoted threads, links in
  quotes, merged and redirected threads, and a resource with its
  discussion thread. All text is made up.
- Integration on Dell with the real pinned packages: BGE representations
  and cross-encoder scores match frozen receipts within stated
  tolerances; spaCy offsets and labels pass spec 0026; the live path is
  timed with 4 cores and controlled SATA-rate disk reads.
- Recovery tests stop the service during reranking; mismatch tests load
  another revision, tokenizer or package lock and check new suggestions
  are blocked.
- A log scanner checks fixture runs leave no post text, anchors or names
  in logs, metrics, traces, progress or stream events.
- Mutation tests cover the hard rules. Private-data runs stay on Dell.
- Benchmarks under the median rule, on the reference hardware and Dell:
  time to first useful suggestion (99th percentile) and to the complete
  reranking set; each channel's search time against index size and disk
  type; fusion and rerank time and memory by input length; live wait
  behind backfill; index build time; backfill throughput and forecast;
  re-score time; rebuild pause; delivery delay; peak package and index
  memory with both queues; and the effect on site page times.

Acceptance tests (Rust):

1. On a seeded corpus with 200,000 dense vectors and corresponding BM25,
   sparse and multi-vector fixtures, each channel's champion-bounded top
   results match its exhaustive reference. Dense similarities are within
   0.00001; sparse and multi-vector scores meet their frozen numerical
   tolerance; any order difference is only among tied hits.
2. A destination with any required title representation or language
   feature missing, or absent from the thread-level index, never appears
   in a suggestion. Once its complete entry is admitted it can appear,
   with coverage recorded.
3. With the initial champion and 80 eligible destinations with anchors,
   exactly 50 are reranked in fused order; with 12, exactly 12. Registry
   fixtures also verify 10 and 100 cross-encoder candidates and refuse
   counts outside 10–100 or above the unique fused count.
4. Every reranker input is at most 8,192 tokens and holds the declared
   local source evidence, title and up to 3 most similar pieces read so
   far; the first post fills at most half the summary room. Every included
   piece is whole or split by the explicit rule. The receipt identifies
   omitted source and destination spans and never claims a complete pair
   reading. The query window follows the window rule.
5. Each hard filter removes its fixture destination with its reason:
   not guest-visible, soft-deleted, own thread, quoted thread, already
   linked (bare URL, and link inside a quote), a pair waiting for
   review, a suppressed or unchanged rejected connection, a non-English
   thread, and a tag or section page.
6. A thread whose links use up its allowance shows "allowance used"
   and is not matched; links inside quotes do not count.
7. For every suggestion, the stored characters at the offsets equal
   the anchor, in NFC code points, with four-byte emoji and combining
   marks; no suggestion changes text.
8. No anchor comes from quotes, attributions, code, spoilers, skipped
   posts, extras, mentions, link text, bare URLs, censored words,
   unknown BBCode, part of a word, or the stop list.
9. A name of a refused kind (spec 0026's list, such as dates, money,
   numbers, nationalities and titles of works) never becomes an
   anchor; person, organisation, product, place and event names can.
10. A phrase found only by the statistical method and one lifted by
    the embedder both appear, with merged scores and no duplicates.
11. A title phrase found in lower case becomes an anchor in the post's
    own case.
12. A phrase with seven qualifying destinations offers five; a phrase
    found three times is offered once, at its first eligible place;
    overlapping candidates become a non-overlapping set.
13. A destination with no acceptable phrase gives no suggestion.
14. A name equal to a user name is never shown; a name is offered only
    for a destination whose title or first post holds it; people
    anchors carry "person"; with the setting off, none appear.
15. After a fixture run, logs, metrics, traces, progress and stream
    events hold no post text, anchors or names.
16. With the generation built by revision A (or tokenizer A) and B
    active, injected on purpose, readiness says "not ready for
    suggestions" and nothing new is made; earlier suggestions stay
    approvable and still name revision A and their index version.
17. A reranker key change leaves earlier valid suggestions approvable
    with their recorded key, but new matching waits for spec 0077 to
    promote a compatible key atomically. No run mixes reranker
    generations.
18. On the imitated reference hardware, with the qualified index, the
    per-post index complete and backfill running, at least 99% of at
    least 100 new posts show a suggestion within 30 seconds; approval
    stays off until the champion's complete reranking set and every
    method finish. A miss is reported as measured and release stays
    blocked.
19. With one method still running or its input missing, the suggestion
    cannot be approved and the missing evidence shows, never as zero.
20. A destination made private after search but before display is not
    shown.
21. Reading a new thread queues its nearest existing posts in the
    backfill queue, not the live queue.
22. A "Thanks!" post with no anchor candidates is read but never
    reranked in backfill.
23. The comparison harness refuses to score before the protocol
    checksum is recorded, and marks a different checksum invalid.
24. On synthetic results, the harness records the supplied metric,
    subgroup, anchor-agreement, calibration and resource evidence,
    including harmful lower bounds and cap breaches. It neither promotes
    a challenger nor waives a failure; spec 0077 alone evaluates the
    evidence and returns the controller outcome.
25. Registry fixtures accept per-channel retrieval bounds of 20 and 500
    and cross-encoder bounds of 10 and 100. They reject 19, 501, 9, 101,
    non-integers and values outside the registered parameter domain.
26. Every model's admission record states its provenance and its exact
    MS MARCO dataset, licence, terms and permitted-use evidence. Missing
    or incompatible evidence blocks admission and cannot be waived
    because a dataset or model is publicly downloadable.
27. Peak memory while reranking the initial champion's 50 maximal pairs
    beside the reading models stays under the slice cap; under injected
    pressure, backfill matching pauses before the cap.
28. A crash after 30 of the initial champion's 50 pairs keeps the 30
    scores and repeats at most the pair being scored.
29. In shadow mode suggestions are stored but never shown.
30. With the initial champion, the analysis row shows "k of 50
    reranked" to a user with review-page permission and nothing to one
    without; another promoted depth is reported from its own receipt.
31. With a backfill reranker call running, a new post's model work
    starts as soon as it ends; the wait is reported.
32. Channel diagnostics report shortlist recall and sweep speed for
    BM25, the active dense route, and BGE-M3 sparse and multi-vector
    retrieval separately. Fusion and cross-encoder ablations record each
    component's effect;
    a configuration missing a required production channel is diagnostic
    evidence only and cannot be promoted.
33. Every suggestion records every model key, serving generation,
    spaCy pipeline and package identities, feature-schema identity and
    coverage. After a compatible promotion, earlier valid suggestions
    stay approvable with their original identities.
34. Before the thread-level index is complete nothing is suggested and
    progress shows the build; after it, a new post gets suggestions
    with coverage below 100% recorded.
35. With backfill posts waiting, a new post is worked first; backfill
    follows the views-and-recency order; each post's suggestions reach
    the add-on and review page when that post completes.
36. With backfill matching busy, backfill reading still advances.
37. When the per-post index completes, every waiting suggestion with
    lower coverage is re-scored: one with a clearly better target is
    replaced and points to its successor; one without keeps its
    suggestion with the re-score recorded; approved and rejected ones
    are untouched; an unchanged pair keeps its score.
38. During an embedder rebuild nothing new is suggested from either
    queue; readiness, review and health pages say "model update
    running"; earlier suggestions stay approvable. After the switch,
    posts saved during the pause go first and are reported separately.
39. A review page with permission gets a stored suggestion's event
    (ids and state only); one without gets nothing; with the stream
    blocked, polling shows it.
40. The review and health pages show queue depth, rate and both
    coverages, each equal to the service's own metric.
41. Synthetic harness fixtures with known answers verify every emitted
    metric, failure flag and receipt. The harness emits no activation,
    promotion or rollback decision; spec 0077 consumes the receipt.
42. With a 1,000-reply destination, the document side holds only whole
    pieces: the 3 best, then first-post and accepted-answer pieces
    while they fit, the first post at most half that room. No piece is
    cut, none repeats, and the destination's dense summary vector comes
    from all 1,001 posts.
43. At a piece size whose pieces exceed the pieces' room, the measured
    reranker count is recorded. A best piece over the pieces' room is split
    into parts at sentences, each scored in its own pair within 8,192
    tokens; the parts together hold every character of the piece. The
    champion's source-aggregation rule, initially the maximum part
    score, determines the destination score. Alternatives are replayable
    evidence only until promoted through spec 0077. A piece size whose
    measured run misses the first-suggestion limit is refused, and the
    old size stays.
44. A source sentence of 3,000 tokens gives a query of the anchor and
    the same number of whole words on each side, where the text allows,
    within 1,024 tokens;
    a short sentence gives the sentence whole plus neighbours in turn.

## Changes from the source

- Matching's first-party integration is Rust on Behemoth Boot, on the
  owner's PC. Exact-pinned third-party packages execute BGE and spaCy;
  no first-party production Python or bespoke model runtime is allowed.
- The site never calls the owner's PC: the service sends suggestions
  and readiness to the add-on and collects review decisions from it.
- Speed follows the 5% median rule, and the 20x goal and its limit
  report are kept (spec 0007); the 30-second level still blocks release.
- Paid embeddings come through AWS Bedrock (spec 0025).
- Memory follows the limits in AGENTS.md section 5.6.
- The document side holds whole pieces only, in place of summary text
  cut at its end.
- The default embedder is the pinned BGE-M3 package route. The sole exact
  admitted Bedrock model may replace only its dense computation under
  specs 0025 and 0077; BGE-M3 sparse and multi-vector production remains
  mandatory. The reranker is the pinned BGE cross-encoder package route,
  and language features come from the pinned spaCy pipeline through spec
  0026's typed Rust-owned contract. Spec 0077 owns their evaluation and
  atomic promotion.

Source: `docs/specifications/linking/matching.md`,
`linking/matching-2.md`, `linking/matching-3.md`, `linking/matching-4.md`,
`linking/matching-5.md` and `linking/matching-6.md` in the archive
`archive-2026-11-08-old-repo`.
