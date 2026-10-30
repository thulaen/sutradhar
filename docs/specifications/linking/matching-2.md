# SPEC-009 part 2: Matching and anchors

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in [the specification
decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Requirements: as in [part 1](matching.md), which holds the scope, search,
filters and anchors. Part 3 is [the comparison test, data, interfaces
and settings](matching-3.md); part 4 is [failure, privacy, performance
and testing](matching-4.md); part 5 is [acceptance](matching-5.md);
part 6 is [the owner's decisions](matching-6.md).

## Design, continued

### Reranking (R103)

- gte-reranker-modernbert-base scores the top 50 eligible destinations
  that have an anchor, or all of them when fewer remain. The number 50
  is fixed by R103 and is not a setting.
- Query side, at most 1,024 tokens (an eighth of the window, P-MA08):
  the source thread title, then the source paragraph around the source
  sentence. The sentence is always whole. A longer paragraph keeps the
  sentence and its nearest neighbouring sentences; a sentence over the
  limit keeps the words around the anchor.
- Document side: the destination title; its 3 pieces most similar to
  the source piece holding the sentence, best first, chosen from the
  pieces the per-post index holds so far (R123); then its summary text
  (SPEC-008), cut at the end to fill the window. The first post may
  fill at most half of the room left for summary text, so the other
  selected posts are always heard (P-MA08). A selected post already
  wholly inside a chosen piece is left out of the summary text.
- Tokens are counted with the reranker's own tokenizer. An assembled
  input over 8,192 tokens, markers included, is refused with an error
  code, never cut silently.
- Pairs run in retrieval order, best first, one pair per model call in
  backfill work (P-MA12), and each result is published as it arrives.
- Each score is stored with the reranker's compatibility key
  (SPEC-010).

### Suggestions

- Results are grouped by anchor phrase: up to five destinations per
  phrase, best reranker score first (R011). Each destination must pass
  the minimum reranker score.
- Each suggestion records: the source post and its content version; the
  anchor text, offsets in code points and source labels; a people flag;
  for each destination, its kind, canonical id, retrieval, anchor and
  reranker scores, best pieces and whether all its posts had been read;
  the model versions (the keys of every model used) and the index
  version (the serving generation and its embedder key) it was made
  with; the index coverage at the time; the settings version; and its
  state (R122, R123).
- States: analysing (with what it still waits for), complete, replaced
  (by a re-score, below) and shadow (stored but not shown, for testing;
  R009). Approval, rejection and expiry are F009's.
- Suggestions are never held (R122). The model and index versions a
  suggestion records matched when it was made, which matching ensures
  by construction (the key check under "Model changes" below), so a
  complete suggestion stays approvable whatever model is active later.
  Approval rests on F009's re-check of content, offsets and visibility
  (R013, R016), not on the current model.
- While a post is analysing, "k of 50 reranked" and the methods still
  pending appear in SPEC-011's per-post analysis progress row, carried
  on SPEC-011's single live stream and visible to anyone with
  review-page permission.
- Rejected candidates keep their reason for 30 days (P-MA11), for the
  "Why no suggestions?" and anchor heat-map views (SPEC-012).
- Moderators approve every link (R009). The minimum anchor and reranker
  scores are calibrated on Dell before release; until then matching
  runs in shadow only.
- Thread link allowances, expiry and the re-check at approval belong to
  F009 (R011, R013, R016).

### Live delivery and queue figures (R123)

- Each source post's suggestions go to PHP the moment that post
  completes, never per batch, and every later change to them goes the
  same way.
- Once PHP has stored them, an event on a `suggestions` topic of
  SPEC-011's single live stream route (server-sent events) reaches
  every open review page. Only viewers with review-page permission
  receive the topic. An event carries ids and the state only; the page
  then fetches the suggestion through PHP, which checks the viewer's
  permission and the content's visibility (R013). Where streaming
  fails, SPEC-011's polling fallback applies.
- The review page and the health page show the depth of the live and
  backfill queues, the processing rate (posts completed a minute,
  measured) and the index coverage (thread-level and per-post). The
  service publishes them as metrics (R127,
  [ADR-0006](../../adr/0006-observability.md)) and on SPEC-011's live
  stream; SPEC-011 and SPEC-012 draw them.

### The two queues (R123)

- Nothing is suggested until the thread-level index is complete. By
  estimate that takes a day or two for 50,000 threads with bge-m3 on an
  ordinary processor; the owner had expected minutes and accepted the
  estimate. Progress shows it (SPEC-011).
- Then suggestions start at once, from both queues, while SPEC-008
  builds the per-post index in the background.
- Live queue: new and edited posts, worked within seconds and always
  first. Each is read (SPEC-008) and then matched as a source.
- Backfill queue: the historical catalogue, in SPEC-008's order of
  thread views and recency. Each post is read into the per-post index
  and, if it can take a link (Q-MA07; "Existing posts" below), matched
  as a source; its suggestions appear the moment it completes.
- Reading is not held back by matching. Matching an existing post costs
  far more than reading it, so backfill model calls alternate between
  reading and matching while both have work; the per-post index is then
  complete in about twice reading's own time, not matching's.
- Every queued item carries the time it was queued, so queue age is
  measured as well as depth (R128).

### Re-scoring once the per-post index is complete (R123)

- A suggestion made while the per-post index was incomplete is
  approvable as usual and records the coverage at the time.
- When SPEC-008 signals that the per-post index is complete, every such
  suggestion still pending is re-scored, in the backfill queue ahead of
  new backfill matching and in the same order. Its source post is
  searched again over the complete index and its top 50 reranked; a
  pair whose assembled input is unchanged keeps its stored score.
- For each anchor phrase, if the re-score finds a clearly better
  target, the pending suggestion is replaced in one step: it becomes
  "replaced", pointing to its successor, which reaches the review page
  live. Clearly better means a destination whose reranker score beats
  the pending suggestion's best by at least the replacement margin
  (`matching.rescore.replace_margin`, calibrated on Dell with the
  minimum scores). Otherwise the suggestion stays, with the re-score
  and the full coverage recorded.
- Approved, rejected and removed suggestions are not re-scored. An
  approval that arrives before the replacement wins.
- Progress shows how many re-scores remain, with a forecast (SPEC-011).

### The first useful suggestion within 30 seconds (R015, R124)

The owner set the hardware for this promise: a stated minimum of 4
processor cores and a SATA solid-state disk (Q-MA02; R124). Its
service level is 99% within 30 seconds, from a post being saved to its
first useful suggestion, for new posts; edited posts take the same
live queue and are measured beside it
([ADR-0007](../../adr/0007-service-levels.md)). The owner chose this
knowing that search alone may take about 12 seconds there. The
estimates below show that the target is at risk on this hardware: it
must be measured, and the result is reported as measured.

Rough estimates for one new post, to be replaced by measurement.
SPEC-008 (part 4) assumes the reference processor manages about 1.5
to 3 × 10^11 operations a second (the upper figure was first assumed
for 8 cores; 4 cores may manage about half), so model steps are given
across that range.

- Receive, extract and check the language: under 1 second.
- bge-m3 on a typical post of about 200 tokens: about 0.5 to 1 second.
- spaCy's `en_core_web_lg`: under 1 second.
- Exact search over the thread-level index alone (about 50,000 summary
  vectors, about 200 MB): well under a second. Over a complete index of
  about 1.5 million vectors (about 6 GB at 32-bit), read from disk in
  blocks: about 12 seconds from a SATA solid-state disk, which reads
  about half a gigabyte a second; less when some vectors are already in
  memory. The disk, not the processor, sets this time.
- Waiting for a backfill call already running (SPEC-008): up to about
  2.4 to 5 seconds for a reading call. For a reranker call in backfill
  matching, typically 2 to 8 seconds, and about 11 to 22 seconds for
  the largest possible pair.
- Anchor candidates and phrase vectors: about 1 to 2 seconds.
- The first reranked pair, of about 2,000 to 4,000 tokens: about 2 to 8
  seconds. About 39 million of the reranker's 149 million numbers are
  a word-piece table; the rest need roughly 0.25 billion operations per
  token.
- Checks, storing and display: about 1 second.

With the complete index, the typical total is about 18 to 26 seconds
when no backfill call is running and about 20 to 34 seconds when a
typical one is; with the largest reranker pair running it is about 30
to 48 seconds. The typical case therefore sits close to 30 seconds and
the worst case passes it. Whether 99% of posts are served within 30
seconds depends on the processor's real speed, how much of the index
stays in the disk cache and how often a long backfill call is running;
only measurement settles it. While the per-post index is still being
built, search is much quicker. Reranking all 50 pairs takes about 2 to
4 minutes. The target is approached as follows:

- Models stay loaded; nothing is loaded per request.
- The live queue goes first; backfill work yields after each model
  call.
- Reranking runs best first. Under the owner's answer to Q-MA01 (the
  first fully checked candidate), the first candidate that passes the
  minimum score and every hard rule available at that point is shown at
  once, labelled "analysis continuing". Approval stays disabled until
  all 50 are reranked, every applicable method has finished and every
  hard rule has passed (R015, R019).
- If nothing passes, the page shows "no suggestion yet, analysis
  continuing" within 30 seconds, then the final result.
- Until the per-post index is complete, each suggestion shows the
  coverage it was made with, because R015 requires missing evidence to
  be visible.
- Backfill pauses by itself when live suggestion times rise, and under
  memory pressure, as the owner's queue-fairness and backpressure
  features ask (OF-30 and OF-29 in the [observability
  features](../../dev/observability-features.md)).
- If measurement misses the target, live calls get a reserved share of
  processor cores while backfill calls run beside them (R020's reserved
  capacity; P-MA12). On 4 cores this is likely to be needed.
- If that is still not enough, the measured shortfall goes to the owner
  with the options, such as reranking the best thread-level match while
  the full scan is still reading from disk. It is never reported as met.

Qualification: over at least 100 new posts, at least 99% of
the times from PHP's save event to the suggestion appearing on the
review page are at most 30 seconds, on the reference hardware (4 cores
and a SATA solid-state disk, imitated on Dell), with the qualified
workload's index size, the per-post index complete and the backfill
running. If this is not met, the target is not qualified and release
stays blocked; a SPEC-007 limit report does not excuse it. In use,
every post's time is recorded as a metric, from which ADR-0007's
service level and its alerts are computed (R127, R128). A forum whose
administrator confirmed a community model projected slower than 30
seconds runs outside the target, and the health page says so (R125).
Posts saved while new suggestions are paused for a model rebuild
(R122) are matched first after the switch and reported separately.

### Existing posts as sources

- Existing posts become sources in the backfill queue, in SPEC-008's
  order of thread views and recency (R123). This replaces the draft's
  order of threads with the strongest unlinked matches first.
- Reranking all 50 candidates for one post costs about 2 minutes of
  full processor time at SPEC-008's assumed rate, perhaps twice that on
  4 cores, so doing it for every one of a million posts would take
  about four to eight years. More cores shorten this in proportion.
- Under the owner's answer to Q-MA07, a post goes to the reranker only
  when it can host a link: it has anchor candidates, its thread has
  allowance left, and at least one eligible destination has an
  acceptable anchor in it. Posts such as "Thanks!" are still read into
  the per-post index but never reach the reranker. Each 100,000 such
  posts still need roughly 4 to 5 months of full processor time at the
  assumed rate, perhaps twice that on 4 cores; the measured forecast is
  shown, while new posts keep the 30-second target.

### New destinations

When SPEC-008 finishes reading a new thread, the existing posts nearest
to its summary vector (exact search, best 200; P-MA10) are queued as
sources in the backfill queue, never the live queue, taking their
places by their threads' views and recency, so older posts can gain
links to new content (R008).

### Model changes and the revision rule (R106, R122)

- Every suggestion records the model and index versions it was made
  with, and is never held because of a later change ("Suggestions",
  above).
- An embedder change that needs a new index (an update, a community
  model or, later, a paid service under R126): SPEC-008 builds the new
  generation with the new model while the old generation stays in place
  (SPEC-010). New suggestions pause for the whole rebuild, from both
  queues (R122; SPEC-010's MD-04): readiness reports "not ready for
  suggestions: model update in progress", and the review page and the
  health page say so, with the rebuild's progress. Suggestions already
  made stay approvable. When the new generation is complete, the switch
  is made in one step and matching resumes, posts saved during the
  pause first. No suggestion is ever made from a mismatched index. A
  rollback to the kept previous generation (SPEC-010) is also one step.
- Safety check: before each matching step, the active embedder's key
  and tokenizer are compared with the serving generation's (SPEC-008,
  SPEC-010). The one-step switch means they should never differ. If
  they do, for example after an index is restored from an older copy,
  no new suggestion is made; readiness reports "not ready for
  suggestions" with a plain-English reason and error code; the review
  page shows the same message; progress shows "waiting for index
  rebuild". This is R106's "block suggestions until rebuilt", which
  after the owner's Q-MA06 answer applies to new suggestions only.
- A reranker or spaCy change needs no index rebuild: the switch follows
  SPEC-010's self-test and new suggestions do not pause. Suggestions
  already made stay approvable under the keys they record. Posts whose
  spaCy records came from the old model get no new anchor candidates
  until SPEC-008 has re-read them.
- After the switch, readiness returns to ready.

### People's names as anchors

People are an allowed entity type (R104), with moderator approval (R009)
and guest-visible content only (R013). Safeguards, as the owner chose
for Q-MA05 (strict safeguards):

1. Only names already in the guest-visible source text, never in
   quotes, mentions or signatures.
2. Never a member's user name: PHP checks each candidate name before it
   is shown (`MemberNameCheck`, part 3), and mentions are never
   anchors.
3. The destination must be about that person: the same name appears in
   the destination's title or first post.
4. The suggestion is marked "person" on the review page, so moderators
   look twice; every link still needs approval.
5. A setting turns people anchors off for a site.
6. Person entity records are purged with their posts and with erasure
   requests (R052), and are shown only on admin pages (R111).
