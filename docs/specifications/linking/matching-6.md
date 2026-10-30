# SPEC-009 part 6: Matching and anchors

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in [the specification
decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Requirements: as in [part 1](matching.md). Parts 2 to 5 hold the rest of
the design and the acceptance cases ([part 2](matching-2.md),
[part 3](matching-3.md), [part 4](matching-4.md),
[part 5](matching-5.md)).

## Decisions (answered by the owner)

The owner answered these on 30 October 2026, in rounds 6 to 9 of the
specification decisions ([part
1](../../requirements/sources/specification-decisions-2026-10-30.md)
and [part
2](../../requirements/sources/specification-decisions-2026-10-30-2.md)).
Each question and its options are kept as history; the first option
was the drafter's recommendation.

- Q-MA01. R015 asks for the first useful suggestion within 30 seconds,
  while approval waits for every method. What counts as "first
  useful"?
  - (recommended) The first candidate the reranker has scored that
    passes every check available so far, shown at once but not
    approvable until all 50 are scored and every method has finished.
  - The best match from search alone, before any reranking, marked
    provisional.
  - Only the finished result after all 50 are scored, which is likely
    to miss 30 seconds on an ordinary processor.

  Owner's answer: the first fully checked candidate (the first option).
- Q-MA02. On what hardware is the 30-second target promised?
  - (recommended) A stated minimum of 8 processor cores, a fast NVMe
    solid-state disk and the 6 GB cap, imitated on Dell and published.
  - 4 cores and a SATA solid-state disk, matching smaller hosts; search
    alone may take about 12 seconds, so the target is unlikely.
  - Dell as it is, with every core.

  Owner's answer: 4 cores and a SATA solid-state disk (R124), chosen
  knowing that search alone may take about 12 seconds there. The
  service level was later set at 99% of new posts within 30 seconds
  ([ADR-0007](../../adr/0007-service-levels.md); measuring edited posts
  beside it is the recorder's reading). Part 2 shows that the
  target is at risk on this hardware and must be measured.
- Q-MA03. What makes a challenger "clearly win" (part 3 explains)?
  - (recommended) Recall at 5 at least +2 points better at the low end
    of its 95% range; no subgroup worse; anchor agreement measured no
    worse, with only its range allowed to dip to −1 point; memory and
    the 30-second target met.
  - Any improvement whose range stays above zero, as SPEC-010's MD-08
    asks of retrained models; the same other conditions.
  - At least +5 points at the low end.

  Owner's answer: at least +2 points (the first option).
- Q-MA04. Should your earlier choices (arctic-embed-s,
  jina-reranker-v1-tiny-en, potion-base-8M) also be tested?
  - (recommended) Yes, each that loads from data-only files without
    code from its repository, with potion-base-8M in a first-pass
    stage. jina-reranker-v1-tiny-en's own instructions appear to need
    repository code, so it will probably be excluded, with the reason.
  - No, only the three challengers you named.

  Owner's answer: yes, those that load safely (the first option).
- Q-MA05. People's names as anchors: which safeguards?
  - (recommended) Never a member's user name; only when the destination
    names the same person in its title or first post; marked "person"
    for moderators; a setting to switch them off.
  - The same, but for any destination the reranker accepts.
  - Off by default; each site chooses to turn them on.

  Owner's answer: strict safeguards (the first option).
- Q-MA06. You chose "block suggestions until rebuilt" when the index
  was built with another model revision. Can suggestions made before
  the mismatch still be approved meanwhile?
  - (recommended) No: they are held and checked again after the
    rebuild.
  - Yes, when each was made with one consistent model and index.

  Owner's answer: neither option, but the owner's own design (R122,
  R123), summarised here. Never hold suggestions: each records the
  model version and index version it was made with and is approvable,
  since those two matched when it was made; approving a link does not
  depend on the current model. Suggestions are made and shown in real
  time, per post, never per batch. The thread-level index is built
  first, and suggestions start when it is complete while the per-post
  index is built in the background. A live queue for new and edited
  posts is worked within seconds and always first; a backfill queue for
  the historical catalogue is worked in the background in order of
  thread views and recency, each post's suggestions appearing the
  moment it completes. Suggestions made before the index is complete
  record the coverage and are re-scored once it is complete, replaced
  if a clearly better target is found. The review page receives new
  suggestions live over server-sent events, and queue depth, processing
  rate and index coverage are shown on the review and health pages.
  The owner also asked to keep serving with the old model until the
  new index was complete; asked to reconcile that with SPEC-010's
  MD-04, the owner chose to pause new suggestions during a rebuild,
  with the switch still made in one step (R122). Asked about R098, the
  owner confirmed that bge-m3 builds the thread-level index from titles
  and summaries, its suggestions approvable, at about a day or two for
  50,000 threads rather than minutes.
- Q-MA07. Scoring all 50 candidates for one existing post takes about 2
  minutes of processor time, so every post of a million-post forum
  would take about four years. Which existing posts are scored?
  - (recommended) Only posts that can actually take a link, strongest
    matches first, with a measured forecast; roughly 4 to 5 months per
    100,000 such posts, less with more cores.
  - As recommended, but only posts from the last few years (you choose
    how many); older ones are shown as "not analysed".
  - Every eligible post, however long it takes.

  Owner's answer: linkable posts, best first (the first option). Asked
  later which order the backfill queue follows, the owner chose thread
  views and recency (R123). This answer therefore limits which posts
  are reranked, within that order; strongest matches no longer set it.

## Proposals

Technical choices put to the owner as a group. The owner accepted all
twelve on 30 October 2026. Since then, R123 has turned the backlog work
these proposals mention into the backfill queue.

- P-MA01: Challengers with shorter windows run at their own window,
  with pieces re-split and reranker inputs scaled in proportion.
- P-MA02: Both the 6-layer and 12-layer ms-marco-MiniLM rerankers are
  tested.
- P-MA03: A link points to a specific reply when that reply's best
  piece beats the first post's by at least 0.05 in similarity.
- P-MA04: spaCy labels map to the owner's types as part 1 lists.
- P-MA05: A resource and its discussion thread count as one
  destination, and the resource is offered.
- P-MA06: Search keeps the best 1,000 hits per piece; retrieval score is
  0.7 of the best piece plus 0.3 of the summary.
- P-MA07: Anchor score weights 0.6, 0.3 and 0.1; keyphrases half
  statistical, half embedder, top 20 per post; anchors 3 to 80
  characters and at most 8 words.
- P-MA08: The reranker's query side gets an eighth of the window; the
  first post fills at most half the summary room.
- P-MA09: The comparison uses at least 2,000 links, a fixed seed and
  the listed subgroups.
- P-MA10: A new thread queues its 200 nearest existing posts as
  sources.
- P-MA11: Rejection reasons are kept for 30 days.
- P-MA12: Backlog reranking runs one pair per model call; if the
  30-second target is missed, live calls get reserved processor cores.

## Recorder's notes

These are the drafter's interpretations, not owner decisions.

- R103's "top 50 candidates" is read as the top 50 destinations that
  have an acceptable existing phrase; a destination without one cannot
  be linked (R010), so reranking it would waste time.
- The "source sentence" is the sentence holding the best anchor for
  that destination. R103 lists the reranker's inputs but not their
  order or shares; those are proposals.
- spaCy usually labels brands as organisations, hence ORG under
  products and brands; "places" is read as GPE, LOC and FAC.
- R109's refusal of short windows covers community models; the R100
  comparison is not a community-model admission.
- Matching existing posts against new threads is read from R008's
  "continuously handle new and edited threads".
- All timings are arithmetic from published parameter counts and a
  SATA disk's typical read rate, not measurements. The processor rate
  range is SPEC-008's assumption; its lower half for 4 cores is a
  drafter's guess.
- R106's "block suggestions until rebuilt" is read, after the owner's
  Q-MA06 answer and R122, as blocking new suggestions only. With the
  one-step switch it is a safety check that should never trigger.
- R122's pause names the embedding model. A reranker or spaCy change
  needs no index rebuild, so it is read as not pausing suggestions.
- The owner did not say how the backfill's processor time is shared
  between reading and matching. Alternating calls is the drafter's
  reading, so that the per-post index completes, and re-scoring
  happens, within weeks rather than the years matching would take.
- The owner did not define "clearly better". The drafter reads it per
  anchor phrase as a reranker score higher by a margin calibrated on
  Dell, and runs re-scores ahead of new backfill matching. Both need
  the owner's confirmation if the calibration shows a real choice.
- How views and recency combine into one order is SPEC-008's to
  define; matching follows it.
- Posts saved while new suggestions pause for a rebuild are reported
  separately from the 30-second figures and do not count towards the
  service level, because the owner chose the pause knowing new posts
  get no suggestions during it (MD-04).
- Live stream events carry ids and state only, and the review page
  fetches the text through PHP, to keep post text off the stream (R013,
  R127). `SuggestionBatch` was renamed `SuggestionUpdate` because
  suggestions are never sent per batch.
- Under SPEC-008's Q-RD02 answer a post is read whole or skipped by its
  main language, so the anchor exclusion covers whole posts.
- The uninstall classes treat suggestion records as the add-on's other
  owned data, because moderators' review history depends on them;
  SPEC-010's table is the authority.
- The jina note rests on its published instructions, not a test; the
  comparison harness confirms it.
- The stop list of generic phrases is first-party data under the
  source-limit rules and needs review. The keyphrase method needs a
  rights review or an admitted library (R027, R067) before use.
