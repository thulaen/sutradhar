# 0023: Matching and anchors

Matching finds, for each source post, the destinations worth linking
to and the existing words that would carry each link. It searches the
index that reading (spec 0022) builds, reranks the best candidates and
makes suggestions in real time, per post. It is roadmap Phase 4
item 3.

## Words used here

- Exact search: comparing a query vector with every stored vector,
  with no shortcut that could miss one.
- Embedder: Qwen3-Embedding-0.6B at its published precision, unless a
  community model (spec 0024) or paid embeddings (spec 0025) is
  active.
- Thread-level index, per-post index and coverage: as in spec 0022.
- Index version: the serving generation's id with its embedder key.
  Model versions are the keys of the models used.
- Live queue and backfill queue: new and edited posts, always first;
  and the historical catalogue, in order of thread views and recency.
  Reading calls them its live and backlog lanes.
- Server-sent events: one open web connection over which updates keep
  arriving at a page.
- Destination: a thread, resource or hub page a link could point to. A
  candidate is a destination being considered for one source post.
- Reranker: a second model that reads the source and a candidate
  together and scores the match. Slower than search, more accurate.
- Anchor: the existing words in the source post that become the link.
- Named entity: a name such as a person, organisation, product, place
  or event. Noun phrase: words built around a noun ("the spare power
  supply"). Keyphrase: a phrase that sums up a post.
- Recall at 5: the share of test cases with the right destination in
  the first five offered.
- MS MARCO: a public collection of search questions whose terms say it
  is for non-commercial research.

## What it does

### Scope

In scope: exact search; candidates and hard filters; anchor candidates
and exclusions; reranking the top 50 within 8,192 tokens; suggestion
records; real-time suggestions with the two queues, re-scoring and live
delivery; the 30-second first suggestion; model changes; safeguards for
people's names; and the comparison test before release.

Out of scope: reading, both indexes, queue order and the index budget
(spec 0022); model files and the memory table (spec 0024); progress,
the live stream and visuals (specs 0016 and 0017); the other ranking
methods and the final ranking (spec 0027); the review page, approval,
insertion, undo and expiry (spec 0028); wrapping the link tag
(spec 0026); release quality judging (spec 0057).

### Pipeline for one source post

1. Preconditions: the post is guest-visible and English; the embedder
   and entity model have read it in the serving generation; the
   thread-level index is complete; no embedder rebuild is running; and
   the serving generation's key matches the active models. Otherwise
   matching waits, visibly.
2. Exact search with each of the post's piece vectors over the whole
   thread-level index and the per-post index as far as it is built. The
   coverage is noted.
3. Hits are grouped into destinations and scored.
4. Hard filters remove ineligible destinations, with reasons kept.
5. Anchor candidates are found once for the post; each destination gets
   its best acceptable anchor or drops out.
6. The top 50 remaining destinations are reranked, best first.
7. Suggestions are assembled with model and index versions and the
   coverage, and handed to ranking (spec 0027) and the review page the
   moment the post completes, never per batch. They stay "analysis
   continuing" until every applicable method has finished.

### Exact search

- All vectors are normalised, so similarity is a dot product.
- Each source piece vector is compared with every stored vector of
  other destinations: summary vectors, and pieces and extras as far as
  built. Blocked matrix arithmetic over the segment files gives the
  same result, within rounding, as one-by-one comparison. What is not
  yet stored shows as coverage, never as a miss.
- Extras similarities are multiplied by the extras weight (0.5).
- The best 1,000 hits per source piece are kept. Ties break by id, so
  results repeat.
- Every search carries the active embedder's key; the index refuses a
  search whose key differs.
- Removed rows are skipped, and results are filtered again by current
  visibility before use.

### Destinations and their scores

- Hits group into destinations: a thread (replies belong to it), a
  resource or a hub page. Redirects and merges resolve to the
  canonical destination.
- Retrieval score: 0.7 × the best piece similarity + 0.3 × the
  similarity of the source piece to the destination's summary vector.
  A destination with no pieces in the per-post index yet is scored on
  its summary alone, and the suggestion records that.
- The target is normally the thread. It is a specific reply when the
  single best piece lies in that reply and beats the first post's best
  piece by at least 0.05. Duplicate checks still use the thread.

### Hard filters before reranking

A destination is removed, with its reason recorded, when:

- it is not guest-visible now, or is deleted, soft-deleted or awaiting
  moderation;
- the embedder has not written its title vector and first summary
  vector in the serving generation (a destination read only by potion
  never qualifies); its other posts need not be read yet;
- its thread is not English;
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

1. Named entities of allowed types, from our own pure-Rust entity
   model (spec 0026), which labels each name as person; organisation;
   product, brand or object; place; or event. Brands labelled as
   organisations count as brands too.
2. Keyphrases from two methods, merged. Statistical candidates (one to
   four words between stop words and punctuation, scored on frequency,
   first position, spread and capitals) are re-scored by the embedder
   (the phrase vector's similarity to the post's own piece). The merged
   score is half of each; the top 20 per post are kept. The statistical
   method is written by us from published descriptions after a rights
   review, or comes from an admitted library.
3. Destination title phrases: noun phrases and entities of a candidate
   destination's title found in the source, on whole words and
   regardless of case, keeping the post's own case.
4. Noun phrases, without leading words such as "the" or "my".

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

- Anchor score: 0.6 × the similarity of the phrase vector to the
  destination's title or summary vector (the higher), + 0.3 × the share
  of the phrase's words in the destination title, + 0.1 × the
  keyphrase score where there is one.
- Phrase vectors are made once per post, in one batch.
- A destination's anchor is its best candidate above the minimum
  anchor score. A destination with none drops out: the system abstains
  rather than change any text.
- The source sentence for reranking is the one holding that anchor.

### Reranking

- gte-reranker-modernbert-base scores the top 50 eligible destinations
  that have an anchor, or all when fewer remain. 50 is fixed, not a
  setting.
- Query side, at most 1,024 tokens (an eighth of the window): the
  source thread title, then a window of the source text around the
  anchor. The query is a question put to the model, not stored post
  text, so the rule that no text is cut does not apply to it. The
  window rule: take the source sentence whole; if room is left, add
  its nearest neighbours in the paragraph, one each side in turn; if
  the sentence alone is over the room, take the anchor and the whole
  words on each side of it, the same number each side where the text
  allows, until the room is full.
- Document side: the destination title; then its pieces most similar
  to the source piece holding the sentence, best first, from the
  pieces held so far: up to 3, as many whole pieces as fit in the
  pieces' room, three-eighths of the window (3,072 reranker tokens);
  then summary pieces.
- Pieces are sized in embedder tokens (spec 0022), and the reranker
  counts in its own. The largest piece size, 3,072 embedder tokens,
  maps to a reranker count measured on the corpus and recorded with
  the generation; the measured figure, not an estimate, is used. So
  that no piece fits nowhere, the best-scoring piece always enters. If
  it is over the pieces' room in reranker tokens, it is split at
  sentence boundaries (words for a sentence too long) into parts that
  each fit. Each part is scored in its own pair with the same query
  and title, and the destination takes the highest score. Other pieces
  join only whole, while they fit. Summary pieces are
  whole pieces of the first post and the accepted answer, most similar
  to the source first, added while whole pieces fit the room left. The
  first post fills at most half that room. A piece already chosen is
  not repeated.
- No text is cut on the document side: every post's text reaches
  pieces (spec 0022), and each piece goes in whole, in split parts
  that together hold all of it, or not at all. The window holds only
  some of a long thread, but every post still counts in full through
  the summary vector.
- Tokens are counted with the reranker's tokenizer. An input over
  8,192 tokens, markers included, is refused with an error code.
- Pairs run in retrieval order, best first, one pair per model call in
  backfill work, and each result is published as it arrives.
- Each score is stored with the reranker's compatibility key.

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
  its top 50 reranked. A pair whose input is unchanged keeps its score.
- Per anchor phrase, if a destination's reranker score beats the
  current best by at least `matching.rescore.replace_margin`, the
  suggestion is replaced in one step: it becomes "replaced", points to
  its successor, and the successor reaches the review page live.
  Otherwise it stays, with the re-score and full coverage recorded.
- Approved, rejected and removed suggestions are not re-scored. An
  approval that arrives before a replacement wins.
- Progress shows the re-scores left, with a forecast.

### The first useful suggestion within 30 seconds

The promise holds on a stated minimum of 4 processor cores and a SATA
solid-state disk. Its service level is 99% of new posts within 30
seconds from save to first useful suggestion; edited posts use the
same queue and are measured beside it. The target is at risk on this
hardware and is reported as measured.

Rough estimates for one new post, at 1.5 to 3 × 10^11 operations a
second:

- Receive, extract, language: under 1 second.
- Qwen3-Embedding-0.6B on about 200 tokens: about 0.6 to 1.2 seconds.
- Entity model: under 1 second.
- Search over the thread-level index alone (about 50,000 vectors,
  about 200 MB): well under a second. Over a whole index of about 1.5
  million vectors (about 6 GB) read in blocks from a SATA disk at about
  half a gigabyte a second: about 12 seconds, less with disk cache.
- Waiting for a running backfill call: 3 to 6 seconds for reading;
  typically 2 to 8 seconds for a reranker call, 11 to 22 seconds for
  the largest pair.
- Anchors and phrase vectors: about 1 to 2 seconds.
- The first reranked pair (2,000 to 4,000 tokens): about 2 to 8
  seconds. About 39 million of the reranker's 149 million numbers are a
  word-piece table; the rest need about 0.25 billion operations per
  token.
- Checks, storing and display: about 1 second.

With the whole index, typical totals are about 18 to 26 seconds with
no backfill call running, 20 to 34 with a typical one, and 30 to 48
with the largest. Reranking all 50 takes about 2 to 4 minutes. The
target is approached so:

- Models stay loaded; nothing loads per request.
- The live queue goes first; backfill yields after each model call.
- Reranking runs best first. The first candidate that passes the
  minimum score and every hard rule available so far is shown at once,
  marked "analysis continuing". Approval stays off until all 50 are
  reranked, every applicable method has finished and every hard rule
  has passed.
- If nothing passes, the page shows "no suggestion yet, analysis
  continuing" within 30 seconds, then the final result.
- Until the per-post index is complete, each suggestion shows its
  coverage.
- Backfill pauses by itself when live suggestion times rise (queue
  fairness) and under memory pressure (backpressure).
- If measurement misses the target, live calls get a reserved share of
  processor cores while backfill runs beside them; on 4 cores this is
  likely to be needed.
- If that is not enough, the measured shortfall goes to the owner with
  options, such as reranking the best thread-level match while the
  full scan still reads from disk. It is never reported as met.

Qualification: over at least 100 new posts, at least 99% of times from
the save event to the suggestion on the review page are at most 30
seconds, on the reference hardware imitated on Dell, with the
qualified index size, the per-post index complete and backfill
running. If not, the target is not qualified and release stays
blocked. Every post's time is recorded as a metric, from which the
service level and its alerts are computed. A site whose administrator
confirmed a community model projected slower than 30 seconds runs
outside the target, and the health page says so. Posts saved during a
model-rebuild pause are matched first after the switch and reported
separately, outside the service level.

### Existing posts as sources

- Existing posts become sources in the backfill queue, in
  views-and-recency order.
- Reranking all 50 for one post costs about 2 minutes of full
  processor time, perhaps twice that on 4 cores; for a million posts,
  about four to eight years.
- So a post goes to the reranker only when it can host a link: it has
  anchor candidates, its thread has allowance left, and at least one
  eligible destination has an acceptable anchor in it. Posts such as
  "Thanks!" are read into the index but never reranked. Each 100,000
  such posts still need about 4 to 5 months of processor time, perhaps
  twice on 4 cores; the measured forecast is shown.

### New destinations

When reading finishes a new thread, the 200 existing posts nearest to
its summary vector (exact search) are queued as sources in the
backfill queue, never the live queue, in their threads'
views-and-recency places, so older posts can gain links to new
content.

### Model changes

- Every suggestion records the model and index versions it was made
  with and is never held for a later change.
- An embedder change that needs a new index (an update, a community
  model or paid embeddings): reading builds a new generation while the
  old stays. New suggestions pause for the whole rebuild, from both
  queues: readiness, the review page and the health page say "not
  ready for suggestions: model update running", with the rebuild's
  progress. Suggestions already made stay approvable. When the new
  generation is whole, the switch is one step and matching resumes,
  posts saved during the pause first. No suggestion is ever made from a
  mismatched index. A rollback to the kept generation is also one step.
- Safety check: before each matching step, the active embedder's key
  and tokenizer are compared with the serving generation's. If they
  differ (for example after restoring an older index), no new
  suggestion is made; readiness says "not ready for suggestions" with
  a plain reason and code; the review page says the same; progress
  shows "waiting for index rebuild". This blocks new suggestions only.
- A reranker or entity-model change needs no rebuild: it switches
  after spec 0024's self-test and new suggestions do not pause. Posts
  whose entity records came from the old model get no new anchor
  candidates until re-read.
- After the switch, readiness returns to ready.

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

### The comparison test before release

- On Dell, the chosen set (Qwen3-Embedding-0.6B with
  gte-reranker-modernbert-base) is compared with all-MiniLM-L6-v2 and
  the ms-marco-MiniLM reranker in its 6-layer and 12-layer sizes; also
  arctic-embed-s, and potion-base-8M in a first-pass stage.
  jina-reranker-v1-tiny-en joins only if it loads from data-only files
  without repository code; otherwise it is excluded with the reason.
  Every model runs in its production form. Pinning is spec 0024's.
- Data: link reconstruction on the site's guest-visible English
  content on Dell. Members' internal links are hidden in a test copy
  (words stay, link removed), and each set tries to recover the
  destination and anchor. At least 2,000 links, sampled at random with
  a fixed seed from the 50,000-thread tier across the subgroups, kept
  apart by thread and time from tuning data; the unseen final
  assessment data is never used.
- Subgroups: thread type, post length, destination popularity (inbound
  links, reported separately because reconstruction favours popular
  threads) and forum section.
- Frozen protocol: before any score, a protocol file records the
  models and revisions, sampling, metrics, subgroups, interval method
  and winning rule, and its SHA-256 is recorded. A run with a different
  checksum is invalid. Hard cases are never dropped and the sample is
  never redrawn to get a pass.
- Metrics: recall at 5 (main); recall at 1 and 3; mean reciprocal rank;
  retrieval recall at 50; anchor agreement (the suggested anchor
  overlaps the member's link text); 95th- and 99th-percentile latency;
  peak memory; throughput.
- By stage: each embedder with the chosen reranker; each reranker on
  the same Qwen3-Embedding-0.6B top-50 lists; potion-base-8M against
  potion-retrieval-32M on shortlist recall at 100 and sweep speed; a
  no-reranker line for information.
- Shorter windows: each challenger runs at its own window (512 tokens
  or fewer). Pieces are re-split under the same rules; reranker inputs
  keep the shares: an eighth for the query, three-eighths for pieces,
  the rest for summary pieces.
- A challenger clearly wins only if: the lower end of the paired 95%
  interval (by re-sampling) of its recall-at-5 gain is at least +2
  points; no subgroup is worse with 95% confidence; anchor agreement's
  measured difference is zero or better and its interval's lower end is
  no lower than −1 point (a measurement tolerance, not leave to get
  worse); and it stays within the memory cap and the 30-second target
  on the reference hardware. A retrained version of the chosen model
  needs only a lower end above zero.
- The report names exact revisions and stays on Dell; a public summary
  needs a privacy review. A clear win is only a proposal: replacing a
  model needs the owner's approval, as would a window under 1,024 or
  8,192 tokens.
- Each model's admission record states whether its training data
  includes MS MARCO and quotes its terms. Such use is recorded, never a
  reason to refuse; this is the owner's allowance, not legal clearance.

### Data stored, and uninstall

- Anchor candidates per post, with spans, sources and scores, rebuilt
  when the post changes; phrase vectors, kept until it changes.
- Suggestion records with scores, versions, coverage, any re-score or
  replacement, and rejection reasons.
- The comparison protocol, results and evidence, on Dell only.

All of it is derived from guest-visible content and purged with it.
Phrase vectors are always deleted at uninstall with no export.
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
  approve, reject or remove with any reason, for learning and
  retraining.

### Settings and defaults

Fixed, not settings: exact search; top 50 reranked; up to 3 best
whole pieces; the 8,192-token window; the four anchor sources; the
allowed entity types; merged keyphrases; moderator approval of every
link; suggestions never held; the thread-level index first; the two
queues; the pause during an embedder rebuild.

| Setting | Default |
|---|---|
| `matching.search.hits_per_piece` | 1,000 |
| `matching.score.piece_weight` | 0.7 |
| `matching.score.summary_weight` | 0.3 |
| `matching.target.reply_margin` | 0.05 |
| `matching.rerank.min_score` | calibrated on Dell |
| `anchors.min_score` | calibrated on Dell |
| `matching.rerank.first_post_share` | 0.5 |
| `anchors.score.weights` | 0.6, 0.3, 0.1 |
| `anchors.keyphrase.weights` | statistical 0.5, embedder 0.5 |
| `anchors.keyphrase.max_per_post` | 20 |
| `anchors.length` | 3 to 80 characters, at most 8 words |
| `anchors.alternatives_per_phrase` | 5 (never above 5) |
| `anchors.people.enabled` | true |
| `matching.reverse.sources_per_destination` | 200 |
| `matching.rejections.keep_days` | 30 |
| `matching.rescore.replace_margin` | calibrated on Dell |

Shadow only until the calibrated values are set. Settings move only
within approved bounds.

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

- Memory: inside the service slice, within the memory limits in
  AGENTS.md section 5.6. Matching adds the reranker's working memory
  for inputs up to 8,192 tokens (one pair at a time on a PC under
  16 GB) and the scan buffer (64 MB by default, 256 MB on a PC with
  16 GB or more, part of reading's index share); disk
  cache counts inside the cap. Memory-saving attention is used, and the
  peak at a full 8,192-token pair is measured.
- The comparison test is a lab comparison run on Dell. It goes through
  the heavy-job queue (spec 0059) as `pankti run -- CMD` and takes one
  place: 4 GB and 8 CPU threads (AGENTS.md section 11). Only retraining
  takes both places (spec 0024). The chosen set is also measured under
  the production cap on both sizes of PC.
- Reranker input at most 8,192 tokens; query side at most 1,024.
- At most 1,000 hits per piece, 50 pairs per post, 5 destinations per
  phrase, 20 keyphrases per post, 200 reverse sources per new thread.
- Service level: 99% of new posts within 30 seconds on 4 cores and a
  SATA solid-state disk.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Test first. Unit tests need no download: a fake embedder, a fake
reranker giving fixed scores from text features, and stored entity
spans stand in for the models.

- Exact search against a plain one-by-one reference on random vectors.
- Property tests: anchor text equals the source characters at its
  offsets; anchors never fall in excluded zones and never overlap; at
  most five destinations per phrase; every reranker input is within
  8,192 tokens and holds the source sentence, the title and a
  first-post share no larger than half the summary room; every piece
  in it is whole.
- Fixtures: the synthetic 300-thread fixture gains destinations that
  trip each filter and exclusion, people's names that are user names,
  products named like members, censored words, quoted threads, links in
  quotes, merged and redirected threads, and a resource with its
  discussion thread. All text is made up.
- Integration on Dell with the real pinned models: reranker scores
  match recorded values within a stated tolerance; the live path from
  save to display is timed with the service limited to 4 cores and its
  disk reads to a SATA disk's rate.
- Recovery tests stop the service during reranking; mismatch tests
  load another revision or tokenizer and check only new suggestions
  are blocked.
- A log scanner checks fixture runs leave no post text, anchors or
  names in logs, metrics, traces, progress or stream events.
- Mutation tests cover the hard rules. Private-data runs stay on Dell.
- Benchmarks under the median rule, on the reference hardware and on
  Dell: time to first useful suggestion (99th percentile) and to all
  50; search time against index size and disk type; rerank time and
  memory by input length; the live wait behind backfill; thread-level
  index build time; backfill throughput and forecast; re-score time;
  rebuild pause length; delay from the add-on storing a suggestion to
  its event reaching a page; peak memory with both queues; and the
  effect on site page times.

Acceptance tests (Rust):

1. On 200,000 random normalised vectors, exact search's top 1,000
   match the reference: each similarity within 0.00001, and any
   difference in order or membership only among hits within that
   tolerance of each other.
2. A destination read only by potion, or not yet in the thread-level
   index, never appears in a suggestion; once its entry is written it
   can, with coverage recorded.
3. With 80 eligible destinations with anchors, exactly 50 are
   reranked, in retrieval order; with 12, exactly 12.
4. Every reranker input is at most 8,192 tokens and holds the whole
   source sentence (or the anchor and nearby words, for a sentence over
   1,024 tokens), the title and the 3 most similar pieces read so far
   (fewer only when a larger piece size leaves room for fewer);
   the first post fills at most half the summary room; every piece on
   the document side is whole or split into parts that hold all of it.
   The query window follows the window rule.
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
17. A reranker key change blocks neither matching nor approval; old
    suggestions keep the old key; new ones carry the new key.
18. On the imitated reference hardware, with the qualified index, the
    per-post index complete and backfill running, at least 99% of at
    least 100 new posts show a suggestion within 30 seconds; approval
    stays off until all 50 pairs and every method finish. A miss is
    reported as measured and release stays blocked.
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
24. On synthetic results: +2.4 lower bound, no subgroup loss, anchor
    agreement +0.2 (lower end −0.9) clearly wins; +1.6 does not; +3.0
    with a significant subgroup loss does not; anchor agreement −0.3
    (lower end −0.9) does not; a challenger over the memory cap does not.
25. A 512-token challenger gets re-split pieces and reranker inputs
    within its window, in the stated shares.
26. Every model's admission record states its MS MARCO use; none is
    refused for it.
27. Peak memory while reranking 50 maximal pairs beside the reading
    models stays under the slice cap; under injected pressure, backfill
    matching pauses before the cap.
28. A crash after 30 of 50 pairs keeps the 30 scores and repeats at
    most the pair being scored.
29. In shadow mode suggestions are stored but never shown.
30. The analysis row shows "k of 50 reranked" to a user with
    review-page permission and nothing to one without.
31. With a backfill reranker call running, a new post's model work
    starts as soon as it ends; the wait is reported.
32. The first-pass stage reports shortlist recall at 100 and sweep
    speed for both potion models; arctic-embed-s is compared;
    jina-reranker-v1-tiny-en is tested or listed as excluded with the
    reason.
33. Every suggestion records every model key, the serving generation
    and its embedder key, and the coverage; after a reranker or entity
    model change, earlier suggestions stay approvable.
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
41. The comparison harness is tested on synthetic results with known
    answers for each branch of the "clearly wins" rule.
42. With a 1,000-reply destination, the document side holds only whole
    pieces: the 3 best, then first-post and accepted-answer pieces
    while they fit, the first post at most half that room. No piece is
    cut, none repeats, and the destination's summary vector comes from
    all 1,001 posts.
43. At a piece size of 3,072 embedder tokens, the measured reranker
    count is recorded. A best piece over the pieces' room is split
    into parts at sentences, each scored in its own pair within 8,192
    tokens; the parts together hold every character of the piece, and
    the destination's score is the highest part's.
44. A source sentence of 3,000 tokens gives a query of the anchor and
    the same number of whole words on each side, where the text allows,
    within 1,024 tokens;
    a short sentence gives the sentence whole plus neighbours in turn.

## Changes from the source

- Matching is rewritten as Rust on Sutradhar Boot, on the owner's PC.
- Models are pure-Rust models (candle or tract), in their data-only
  production forms.
- The site never calls the owner's PC: the service sends suggestions
  and readiness to the add-on and collects review decisions from it.
- The old speed goal and its limit report are replaced by the 5%
  median rule; the 30-second service level still blocks release.
- Paid embeddings come through AWS Bedrock (spec 0025).
- Memory follows the limits in AGENTS.md section 5.6.
- The document side holds whole pieces only, in place of summary text
  cut at its end.
- The embedder is Qwen3-Embedding-0.6B, and names come from our own
  pure-Rust entity model, not a ready-made one.

Source: `docs/specifications/linking/matching.md`,
`linking/matching-2.md`, `linking/matching-3.md`, `linking/matching-4.md`,
`linking/matching-5.md` and `linking/matching-6.md` in the archive
`archive-2026-11-08-old-repo`.
