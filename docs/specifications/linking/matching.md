# SPEC-009: Matching and anchors

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in [the specification
decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Requirements: R103, R104, R099, R100, R015 and R106's rule on index
revisions; R122 (model changes and suggestion records), R123
(suggestions in real time) and R124 (the 30-second target's hardware
and service level), from the owner's answers; also R009–R013, R017,
R019, R027, R050, R053, R059–R060, R120, R121 and R125–R128 where
matching touches them.
Features served: F008 (candidate generation, relevance and ranking: the
model stages), F005 (choosing anchors; wrapping them stays in F005),
F009 (suggestion records for the review page), F023 (the comparison
test before release) and F006 (admission notes for R099).

Parts: this file (scope, search, filters and anchors);
[part 2](matching-2.md) (reranking, suggestions, live delivery, the
two queues, re-scoring, the 30-second target, existing posts, model
changes and people's names); [part 3](matching-3.md) (the comparison
test, stored data, interfaces and settings); [part 4](matching-4.md)
(failure, privacy, performance and testing); [part 5](matching-5.md)
(acceptance cases); [part 6](matching-6.md) (the owner's decisions,
proposals and recorder's notes).
Reading and the index are [SPEC-008](reading.md). SPEC-010 owns model
files, community models, retraining, the uninstall table and the
memory table (a 6 GB service cap, or 8 GB once Dell measurements
confirm the estimate, R121); SPEC-011 owns progress and the live
stream; SPEC-012 owns the visual layer.
Sources:
[models update](../../requirements/updates-2026-10-30/models.md) and
[specifications update](../../requirements/updates-2026-10-30/specifications.md)
(R120–R129).

## Words used here

- Nearest-neighbour search: finding the stored vectors most similar to
  a query vector. Exact search compares the query with every stored
  vector, with no shortcut that could miss one.
- Embedder: the model that writes the vectors: bge-m3, run from its
  official ONNX export by ONNX Runtime (R120), unless a community model
  is active (SPEC-010) or, once its own specification is approved, a
  paid embedding service the administrator chose (R126).
- Thread-level index: the title and summary vectors of each thread,
  resource or hub page, written by the embedder (SPEC-008). It is
  built first (R123).
- Per-post index: the vectors of every post's pieces and extras, built
  in the background (R123). Most of it fills after the thread-level
  index is complete; first posts' pieces and live posts join it earlier
  (SPEC-008, part 2).
- Index coverage: how much of the serving generation's index is
  written: destinations in the thread-level index and posts read, each
  out of the eligible total (SPEC-008).
- Index version: the serving index generation's id, with the embedder
  key it was built with (SPEC-008, SPEC-010). Model versions are the
  keys of the models used.
- Live queue and backfill queue: new and edited posts, always worked
  first; and the historical catalogue, worked in the background in
  order of thread views and recency (R123). SPEC-008 reads posts in the
  same two queues, which it calls its live and backlog lanes.
- Server-sent events: one open web connection over which the server
  keeps sending updates to a page (SPEC-011's live stream).
- Destination: a thread, resource or hub page a link could point to. A
  candidate is a destination being considered for one source post.
- Reranker: a second model that reads the source text and a candidate's
  text together and scores how well they match. It is slower than
  vector search and more accurate.
- Anchor: the existing words in the source post that would become the
  link (R010).
- Named entity: a name spaCy finds, such as a person, organisation,
  product, place or event. Noun phrase: a group of words built around a
  noun, such as "the spare power supply". Keyphrase: a phrase that sums
  up what a post is about.
- Recall at 5: the share of test cases in which the right destination
  is among the first five offered.
- MS MARCO: a large public collection of search questions and answers
  whose terms say it is for non-commercial research.

## Scope

In scope:

- Exact nearest-neighbour search over all stored pieces, extras and
  summary vectors (R103).
- Candidate destinations and the hard filters applied before reranking.
- Anchor candidates from the four sources, the allowed entity types,
  exclusions and merged keyphrases (R104, R010).
- Reranking the top 50 with the inputs R103 names, within 8,192
  tokens.
- Suggestion records with up to five destinations per phrase (R011),
  scores, model and index versions, index coverage and state; rejection
  reasons; and the "k of 50 reranked" figure for SPEC-011's per-post
  analysis row.
- Suggestions made and shown in real time, per post: the live and
  backfill queues, the start once the thread-level index is complete,
  re-scoring once the per-post index is complete, live delivery to the
  review page over server-sent events, and the queue figures on the
  review and health pages (R123).
- How the 30-second first suggestion is approached with full-precision
  bge-m3 on the stated minimum of 4 cores and a SATA solid-state disk,
  with its service level of 99% within 30 seconds (R015, R124).
- Model changes: suggestions are never held; new suggestions pause
  while a new embedder rebuilds the index; and a key mismatch still
  blocks new suggestions as a safety check (R106, R122).
- Safeguards for anchors that are people's names (R104).
- The comparison test with the named challengers (R100) and the MS
  MARCO allowance (R099).

Out of scope, owned elsewhere:

- Reading, the thread-level and per-post indexes, the order of both
  queues and the index size budget: SPEC-008. Model files,
  `models.lock`, community models, retraining, the uninstall table and
  the memory table: SPEC-010. Progress, the live stream and visuals:
  SPEC-011 and SPEC-012.
- The other relevance, spam, quality and confidence methods and the
  final ranking across them: the rest of F008, with the method
  catalogue (F006).
- The review page, approval, insertion, undo and expiry (F009), and
  placing the link tag around an anchor (F005).
- Release quality judging with AI judges (F023, R059–R066).

## Design

### Pipeline for one source post

1. Preconditions: the post is guest-visible and English (judged by its
   main language, SPEC-008's Q-RD02); the embedder and spaCy have read
   it in the serving generation; the thread-level index is complete
   (R123); no embedder rebuild is under way (R122); and the serving
   generation's key matches the active models (SPEC-010). Otherwise
   matching waits, visibly.
2. Exact search with each of the post's piece vectors over everything
   the serving generation holds: the whole thread-level index and the
   per-post index as far as it has been built. The coverage is noted.
3. Hits are grouped into destinations and scored.
4. Hard filters remove ineligible destinations, with reasons kept.
5. Anchor candidates are found once for the post; each destination gets
   its best acceptable anchor or drops out.
6. The top 50 remaining destinations are reranked, best first.
7. Suggestions are assembled, record their model and index versions
   and the coverage, and are handed to F008's ranking and F009's review
   page the moment the post completes, never per batch (R122, R123).
   They stay "analysis continuing" until every applicable method has
   finished (R015).

### Exact search (R103)

- All vectors are normalised, so similarity is a dot product.
- Each source piece vector is compared with every stored vector of
  other destinations in the serving generation: summary vectors (from
  the thread-level index), and pieces and extras (the per-post index,
  as far as it has been built). The work is done as blocked matrix
  arithmetic over SPEC-008's segment files and gives the same result,
  within rounding, as comparing the vectors one by one. Search is exact
  over what is stored; what is not yet stored shows as coverage, never
  as a miss (R015, R123).
- Extras similarities are multiplied by the extras weight (0.5,
  SPEC-008's P-RD11).
- The best 1,000 hits per source piece are kept (P-MA06). Ties are
  broken by id, so results are repeatable.
- Every search carries the active embedder's key, and the index
  refuses a search whose key differs (SPEC-010's defence in depth).
- Removed rows are skipped, and results are filtered again by current
  visibility before use (Q148 starred case).

### Destinations and their scores

- Hits group into destinations: a thread (replies belong to their
  thread), a resource or a hub page. Redirects and merges resolve to
  the canonical destination (Q148).
- Retrieval score: 0.7 × the best piece similarity, plus 0.3 × the
  similarity between the source piece and the destination's summary
  vector (P-MA06). A destination with no pieces in the per-post index
  yet is scored on its summary similarity alone, and the suggestion
  records that (R123).
- The link target is normally the thread. It is a specific reply when
  the single best-matching piece lies in that reply and beats the first
  post's best piece by at least 0.05 (P-MA03). Duplicate checks still
  use the parent thread (R012).

### Hard filters before reranking

A destination is removed, and its reason recorded, when:

- it is not guest-visible now, or is deleted, soft-deleted or awaiting
  moderation (R013; Q148);
- the embedder has not read its title and summary (its entry in the
  thread-level index) in the serving generation; a destination read
  only by potion never qualifies (R098 as changed by R123). Its other
  posts need not be read yet: the coverage is recorded and the
  suggestion is re-scored later (part 2);
- its thread is not English (SPEC-008's P-RD04);
- it is the source's own thread, or the thread the source post quotes
  (Q148 starred cases);
- the source thread already links to it, counting bare URLs and links
  inside quotes, or a suggestion for that pair is pending (R012);
- the connection was removed and is suppressed, or was rejected and
  neither side has changed meaningfully since (R017);
- it is not a permitted kind: threads, resources when Resource Manager
  is present, hub pages when that module is enabled; never tags, nodes
  or member pages (R014).

A resource and its discussion thread count as one destination for these
checks, and the resource is offered (P-MA05). A source thread that has
used its link allowance is shown as "allowance used" and is not
matched; existing links count towards that allowance, except links
inside quotes (R011; Q148 starred case).

### Anchor candidates (R104)

Candidates come from four sources, found once per source post from
SPEC-008's stored spans:

1. Named entities of the allowed types, mapped from the labels of
   spaCy's `en_core_web_lg` (R120; P-MA04): products and brands as
   PRODUCT, plus ORG, which spaCy usually gives to brands;
   organisations as ORG; places as GPE, LOC and FAC (countries and
   towns, other locations, and buildings or venues); events as EVENT;
   people as PERSON.
2. Keyphrases from both methods, merged. Statistical candidates (word
   groups of one to four words between stop words and punctuation,
   scored on frequency, first position, spread and capitals within the
   post) are re-scored by the embedder (the similarity of the phrase's
   vector to the post's own piece). The merged score is half of each,
   and the top 20 per post are kept (P-MA07). The statistical method is
   implemented first-party from published descriptions after a rights
   review (R067), or an admitted library is used (F006).
3. Destination title phrases: noun phrases and entities of a candidate
   destination's title found in the source text, matched regardless of
   case and on whole words, keeping the post's own case.
4. Noun phrases from spaCy, without leading words such as "the" or
   "my".

The same span found by several sources becomes one candidate that keeps
all its source labels.

### Never an anchor

These exclusions are hard rules (R010, R013, R101; Q148 starred cases):

- text in quotes at any depth, quote attributions, code or spoilers;
- text SPEC-008 skips as not English (whole posts, because a post is
  judged by its main language under SPEC-008's Q-RD02 answer);
- extras, mentions, the text of existing links, bare URLs, or text
  inside a URL;
- censored words and the content of unknown add-on BBCode;
- part of a word ("cat" in "category"), or a cut through a combining
  character sequence;
- a span the F005 placement check cannot wrap safely, such as one that
  crosses a formatting tag unevenly;
- generic phrases on a reviewed stop list ("this", "here", "the
  thread", "click here"), and numbers or dates on their own;
- an entity of a type that is not allowed, whichever source found it;
- a member's user name (the people safeguards in part 2);
- under 3 or over 80 characters, or more than 8 words (P-MA07).

Only existing words are linked. An anchor's text must equal the stored
source characters at its offsets, and no text is added, removed or
changed (R010). A phrase that occurs several times is offered only at
its first eligible occurrence. Overlapping candidates are reduced to a
set that does not overlap, keeping the higher scores.

### The best anchor for each destination

- Anchor score: 0.6 × the similarity between the phrase's vector and
  the destination's title or summary vector (whichever is higher), plus
  0.3 × the share of the phrase's words found in the destination title,
  plus 0.1 × the keyphrase score where there is one (P-MA07).
- Phrase vectors are computed once per source post, in one batch.
- A destination's anchor is its highest-scoring candidate above the
  minimum anchor score. A destination with none drops out: the system
  abstains rather than change any text (R010).
- The source sentence for reranking is the sentence that holds that
  anchor.
