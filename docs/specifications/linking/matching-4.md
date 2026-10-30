# SPEC-009 part 4: Matching and anchors

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in [the specification
decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Requirements: as in [part 1](matching.md). Part 2 is [reranking,
suggestions, the two queues and the 30-second target](matching-2.md);
part 3 is [the comparison test, data, interfaces and
settings](matching-3.md); part 5 is [acceptance](matching-5.md); part 6
is [the owner's decisions](matching-6.md).

## Failure behaviour

Failures are shown in plain English with an error code and a manual
link (R051); nothing missing is invented or counted as zero (R015).

- Reranker missing or failed: search and anchors still run; suggestions
  stay "waiting for reranker" and cannot be approved.
- Embedder missing or failed: nothing new is matched; readiness is "not
  ready". bge-m3 runs from its official ONNX export through ONNX
  Runtime and spaCy uses `en_core_web_lg` (R120, SPEC-010's MD-01 and
  MD-02), so both load from data-only files on a forum host.
- spaCy records missing or stale: no anchor candidates for those posts,
  so no new suggestions from them (R019); suggestions already made are
  not held (R122).
- Thread-level index not yet complete: nothing is suggested; progress
  shows its build (part 2).
- Embedder rebuild under way: new suggestions pause, as part 2 says;
  suggestions already made stay approvable (R122).
- Key mismatch: new suggestions blocked as in part 2; suggestions
  already made are not held (R122).
- Live stream unavailable: the review page falls back to SPEC-011's
  polling; no suggestion is lost, because PHP has stored it first.
- A vector segment unreadable or damaged: matching stops; readiness
  reports it; SPEC-008 rebuilds the segment from stored pieces.
- Visibility check failed or timed out: nothing is shown; it is retried.
- An assembled pair over 8,192 tokens (which the design prevents): it is
  refused with a code, never cut silently, and reported as a defect.
- Memory pressure: backfill matching pauses before the cap (R023), as
  the owner's backpressure feature (OF-29) asks; live reranking runs
  one pair at a time.
- Live suggestion times rising: backfill pauses until they fall (the
  owner's queue-fairness feature, OF-30).
- Crash during reranking: finished scores are kept and at most the pair
  in progress repeats (R020).
- No candidate: "no suggestion", with the main rejection reasons; never
  an invented suggestion.

## Privacy and security

- Only guest-visible content is matched. Visibility is checked again
  before display and again at approval (R013).
- Logs, metrics, traces, progress events and the live stream's
  `suggestions` events carry ids, counts, durations and codes, never
  post text, anchors or names. Metrics and tracing are on by default
  (R127, [ADR-0006](../../adr/0006-observability.md)), and a test
  enforces this. The per-post analysis row shows counts only. A review
  page fetches a suggestion's text through PHP, with the viewer's
  permission and the content's visibility checked (R013).
- Comparison data stays on Dell (R058, R075).
- People's names follow the safeguards in part 2.
- No model downloads code or calls the network during matching
  (SPEC-010).

## Performance and resource budgets

- Memory: SPEC-010 owns the whole memory table (6 GB, or 8 GB once
  Dell measurements confirm the estimate; R121), including the
  reranker's weights. Matching adds the reranker's working memory for inputs of
  up to 8,192 tokens and the index's scan buffer (256 MB by default,
  part of the index share SPEC-008 defines); disk cache counts within
  the cap. Memory-saving attention must be used, and the peak at a full
  8,192-token pair is measured.
- The 20× gate (SPEC-007): search, grouping, filters and anchor finding
  aim for result A against reviewed simple references, such as one
  query at a time over a plain array. Reranker and embedder inference
  are expected to need result B, a reviewed limit report, because the
  owner fixed the models and full precision.
- Must be measured, on the reference hardware of 4 cores and a SATA
  solid-state disk (R124) as well as on Dell: time to the first useful
  suggestion (the 99th percentile, against ADR-0007's 99% within 30
  seconds) and to completion of all 50; exact search time against
  index size and disk type; rerank time and memory by input length;
  the live wait behind a backfill call; the thread-level index's build
  time (estimated at a day or two for 50,000 threads); backfill
  matching throughput and its forecast; the time to re-score once the
  per-post index is complete; how long new suggestions pause during an
  embedder rebuild; the delay from PHP storing a suggestion to its
  event reaching a review page; peak memory with live and backfill work
  together; and the effect on forum page times. No figure here is a
  measurement yet.

## Testing

- Test-first (R087): each behaviour starts with a test that fails for
  the intended reason, then the smallest change that passes it.
- Unit tests need no download: a fake embedder, a fake reranker giving
  fixed scores from text features, and stored spaCy spans stand in for
  the models (ADR-0002).
- Exact search is tested against a plain one-by-one reference on random
  vectors, comparing within a stated tolerance.
- Property tests check that anchor text always equals the source
  characters at its offsets; anchors never fall in excluded zones and
  never overlap; at most five destinations are offered per phrase; and
  every reranker input stays within 8,192 tokens and holds the source
  sentence, the destination title and a first-post share no larger
  than half the summary room.
- Fixtures: the synthetic fixture of about 300 threads (R058) gains
  destinations built to trip each filter and exclusion, people's names
  that are also user names, products named like members, censored
  words, quoted threads, links inside quotes, merged and redirected
  threads, and a resource with its discussion thread. All text is
  synthetic.
- Integration tests on Dell use the real pinned models, bge-m3 through
  ONNX Runtime and `en_core_web_lg` (R120): reranker scores for fixture
  pairs match recorded values within a stated tolerance, and the live
  path from save event to display is timed on the reference hardware,
  imitated on Dell by limiting the service to 4 cores and its disk
  reads to a SATA solid-state disk's rate where Dell has no such disk.
- Recovery tests stop the service during reranking; mismatch tests load
  a different model revision or tokenizer and check that only new
  suggestions are blocked.
- Queue tests: a live post overtakes a waiting backfill post; backfill
  follows the views-and-recency order SPEC-008 supplies; suggestions
  reach PHP per post; a model rebuild pauses both queues; re-scoring
  starts when the per-post index completes and replaces only on a
  clearly better target.
- Live stream tests check that a stored suggestion's event reaches a
  review page with permission and no other, and that polling takes
  over when the stream is blocked.
- The comparison harness is tested on synthetic results with known
  answers, including each branch of the "clearly wins" rule.
- A log scanner checks that fixture runs leave no post text, anchors or
  names in logs, metrics, traces, progress events or live stream
  events.
- Mutation tests cover the hard rules (R077). Private-data runs stay on
  Dell (R058).
