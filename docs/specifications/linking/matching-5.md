# SPEC-009 part 5: Matching and anchors

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in [the specification
decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Requirements: as in [part 1](matching.md). Parts 2 to 4 hold the rest of
the design ([part 2](matching-2.md), [part 3](matching-3.md),
[part 4](matching-4.md)); [part 6](matching-6.md) holds the owner's
decisions, the proposals and the recorder's notes.

## Acceptance cases

- AT-MA01: On 200,000 random normalised vectors, exact search's top
  1,000 match a one-by-one reference: each similarity is within 0.00001
  of the reference value, and any difference in order or membership
  involves only hits whose similarities lie within that tolerance of
  each other.
- AT-MA02: A destination read only by potion, or whose title and
  summary bge-m3 has not yet read into the thread-level index, never
  appears in a suggestion; once its thread-level entry is written it
  can, with its coverage recorded (R123).
- AT-MA03: With 80 eligible destinations that have anchors, exactly 50
  are reranked, in retrieval order; with 12, exactly 12.
- AT-MA04: Every assembled reranker input is at most 8,192 tokens by
  the reranker's tokenizer and holds the whole source sentence (or, for
  a sentence over 1,024 tokens, the anchor with the words around it),
  the destination title and its 3 most similar pieces read so far; the
  first post fills at most half the summary room, and summary text is
  cut only at its end.
- AT-MA05: Each hard filter removes its fixture destination with its
  reason recorded: not guest-visible, soft-deleted, the source's own
  thread, the quoted thread, already linked (by a bare URL, and by a
  link inside a quote), a pending pair, a suppressed or unchanged
  rejected connection, a non-English thread, and a tag or node.
- AT-MA06: A source thread whose existing links use up its allowance is
  shown as "allowance used" and not matched; links inside quotes do not
  count towards that allowance.
- AT-MA07: For every suggestion, the stored characters at the anchor's
  offsets equal its text, in NFC code points, including fixtures with
  four-byte emoji and combining marks; no suggestion adds, removes or
  changes text.
- AT-MA08: No anchor comes from quotes at any depth, attributions,
  code, spoilers, posts skipped as not English, extras, mentions, link
  text, bare URLs, censored words, unknown BBCode, part of a word ("cat"
  in "category") or the stop list.
- AT-MA09: An entity labelled DATE, MONEY, CARDINAL, NORP or
  WORK_OF_ART by `en_core_web_lg` never becomes an anchor through any
  source; PERSON, ORG, PRODUCT, GPE, LOC, FAC and EVENT can.
- AT-MA10: A fixture phrase found only by the statistical method and
  one lifted by the embedder both appear among the keyphrases, with
  merged scores and no duplicates.
- AT-MA11: A destination title phrase found in lower case in the source
  becomes an anchor in the post's own case.
- AT-MA12: A phrase with seven qualifying destinations offers five; a
  phrase occurring three times is offered once, at its first eligible
  occurrence; overlapping candidates become a set that does not
  overlap.
- AT-MA13: A destination with no acceptable existing phrase in the
  source produces no suggestion.
- AT-MA14: A person's name equal to a member's user name is never
  shown; a person's name is offered only for a destination whose title
  or first post contains it; people anchors carry the "person" mark;
  with the setting off, none appear.
- AT-MA15: After a fixture run, logs, metrics, traces, progress events
  and live stream events contain no post text, anchor text or names.
- AT-MA16: With the serving generation built by revision A (or
  tokenizer A) and revision B active, a state the one-step switch
  should never produce and which the test injects, readiness reports
  "not ready for suggestions" and no new suggestion is made;
  suggestions made before stay approvable and still name revision A
  and their index version (R106, R122).
- AT-MA17: A reranker key change blocks neither matching nor approval:
  suggestions made with the old reranker stay approvable and keep its
  key; new suggestions carry the new key.
- AT-MA18: On the reference hardware (4 cores and a SATA solid-state
  disk, imitated on Dell), with the qualified index size, the per-post
  index complete and the backfill running, at least 99% of at least
  100 new posts show a suggestion within 30 seconds of the
  save event (R124, ADR-0007); approval stays disabled until all 50 pairs and
  every applicable method have finished. A miss is reported as
  measured, and release stays blocked.
- AT-MA19: With one applicable method still running, or its input
  missing, the suggestion cannot be approved and the missing evidence
  is shown, never as zero.
- AT-MA20: A destination made private after search but before display
  is not shown.
- AT-MA21: Reading a new thread queues its nearest existing posts as
  sources in the backfill queue, not the live queue.
- AT-MA22: A post such as "Thanks!" with no anchor candidates is read
  into the per-post index but never sent to the reranker in backfill
  matching.
- AT-MA23: The comparison harness refuses to compute scores before the
  protocol checksum is recorded, and marks a run with a different
  checksum invalid.
- AT-MA24: On synthetic results: a lower bound of +2.4 points with no
  subgroup loss and anchor agreement +0.2 (lower end −0.9) clearly
  wins; +1.6 does not; +3.0 with a significant subgroup loss does not;
  anchor agreement −0.3 (lower end −0.9) does not; a challenger over
  the memory cap does not.
- AT-MA25: A 512-token challenger receives re-split pieces within its
  window, and reranker inputs within its window in the stated
  proportions.
- AT-MA26: Every production and challenger model's admission record
  states whether it used MS MARCO, and none is refused for that.
- AT-MA27: Peak service memory while reranking 50 maximal pairs beside
  the reading models stays under the service cap (6 GB, or 8 GB once
  confirmed; R121); under injected memory pressure, backfill matching
  pauses before the cap.
- AT-MA28: A crash after 30 of 50 pairs keeps the 30 scores on restart
  and repeats at most the pair in progress.
- AT-MA29: In shadow mode suggestions are stored but never shown to
  moderators.
- AT-MA30: SPEC-011's per-post analysis row shows "k of 50 reranked" to
  a user with review-page permission and nothing to a user without it.
- AT-MA31: With a backfill reranker call running, a new post's model
  work starts as soon as that call ends; the measured wait is reported.
- AT-MA32: The first-pass stage reports shortlist recall at 100 and
  sweep speed for both potion models; arctic-embed-s is compared; and
  jina-reranker-v1-tiny-en is either tested or listed as excluded with
  the reason (Q-MA04).

### The owner's design for suggestions (R122, R123)

- AT-MA33: Every suggestion records the keys of every model used, the
  serving generation and its embedder key, and the per-post index
  coverage at the time. After the active reranker or spaCy model
  changes, a suggestion made before stays approvable.
- AT-MA34: Before the thread-level index is complete no suggestion is
  made and progress shows the build; once it is complete, a new post
  gets suggestions with a per-post coverage below 100% recorded.
- AT-MA35: With posts waiting in the backfill queue, a new post is
  worked first. Backfill posts are matched in the views-and-recency
  order SPEC-008 supplies, and each post's suggestions reach PHP and
  the review page when that post completes, not when a batch does.
- AT-MA36: With backfill matching busy, backfill reading still
  advances: the per-post index grows while matching runs.
- AT-MA37: When the per-post index completes, every pending suggestion
  made with lower coverage is re-scored. A fixture with a clearly
  better target replaces its suggestion, which becomes "replaced" and
  points to its successor; one without keeps its suggestion, with the
  re-score recorded; approved and rejected suggestions are untouched;
  a pair whose input is unchanged keeps its stored score.
- AT-MA38: During an embedder rebuild no new suggestion is made from
  either queue; readiness, the review page and the health page say
  "model update in progress"; suggestions made before stay approvable.
  After the one-step switch, posts saved during the pause are matched
  first, and their times are reported separately.
- AT-MA39: A review page open with review-page permission receives a
  stored suggestion's event over the live stream, carrying ids and
  state only; a page without that permission receives nothing; with
  the stream blocked, polling shows the suggestion.
- AT-MA40: The review page and the health page show live and backfill
  queue depth, the processing rate and the thread-level and per-post
  index coverage, and each equals the service's own metric.
