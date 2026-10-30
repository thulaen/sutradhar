# SPEC-008 part 5: Reading and indexing

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in the
[specification decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Requirements: as in [part 1](reading.md). Parts 2 to 4 hold the rest of
the design ([part 2](reading-2.md), [part 3](reading-3.md),
[part 4](reading-4.md)); part 6 holds the
[owner's decisions, proposals and recorder's notes](reading-6.md).

## Acceptance cases

- AT-RD01: An item sent with `guest_visible` false is refused and
  logged by id and code only; private conversations, drafts, staff-only
  posts and signatures never appear in the store.
- AT-RD02: A post with three levels of nested quotes, a code block
  containing `[url]`, a spoiler and an inline spoiler yields pieces
  holding none of their text; the text around them is read.
- AT-RD03: A 300-token English post yields exactly one piece, which
  starts with the thread title and holds at most 1,024 tokens, title
  included, counted by the active embedder's tokenizer.
- AT-RD04: A 5,000-token post of 20 paragraphs yields pieces of at most
  1,024 tokens, each starting at a paragraph or sentence boundary, with
  each overlap between 128 and 205 tokens.
- AT-RD05: A 3,000-token paragraph is split at sentences; a 2,000-token
  sentence without punctuation is split at words; no piece exceeds
  1,024 tokens and no word is cut.
- AT-RD06: For any generated post, every readable character belongs to
  at least one piece (property test).
- AT-RD07: A French post is shown as "skipped: not English" with no
  pieces, vectors or entities, yet its links and state are recorded. A
  French paragraph inside a mostly English post is read with the rest
  of the post, whose state records one paragraph in another language; a
  mostly French post with one English paragraph is skipped whole
  (Q-RD02).
- AT-RD08: A 12-letter reply in an English thread takes the thread's
  language and is read.
- AT-RD09: Alt text, attachment names and stored link-preview titles
  become extras with weight 0.5, appear in no piece and carry the
  never-anchor flag; no network request is made for a title.
- AT-RD10: Editing one reply in a 2,000-post thread re-reads that reply
  alone, plus the summary if the reply is in the subset.
- AT-RD11: An edit that changes only a quote causes no embedder call
  and records the new content version.
- AT-RD12: A title edit re-reads the first post and summary in the live
  lane and queues every other post of the thread in the backlog lane;
  the queued count is shown.
- AT-RD13: In a fixture question thread with 60 replies, the summary
  subset holds the title, the first post, the accepted answer, the 5
  highest-reacted replies, the first 5 replies and the 3 most recent
  posts, topped up to 20 with the next most-reacted and without
  duplicates; every selected post but the first is cut to 400
  characters after removing quotes, code and spoilers. Reactions of
  every type count equally, Sad and Angry included (Q-RD05), and the
  first post's vector is the average of its pieces' vectors (Q-RD06).
- AT-RD14: Changing `reading.summary.recent` from 3 to 4 rebuilds the
  affected summaries in the background with progress; each old summary
  serves until its replacement is written.
- AT-RD15: With 10,000 backlog posts queued, a new post waits only for
  the one backlog call already running (measured), never a whole batch.
- AT-RD16: Synthetic pressure above 20% for 10 seconds in the forum's
  PHP service pauses the backlog within 5 seconds, shown as "paused:
  forum busy"; it resumes after pressure stays below 10% for 60
  seconds. The same pressure inside Sutradhar's own service does not
  pause it. The live lane keeps working throughout.
- AT-RD17: Killing the service mid-backlog and restarting it resumes
  from the last checkpoint and repeats at most one minute of work per
  worker. Piece boundaries match an uninterrupted run exactly, and each
  vector has cosine similarity of at least 0.9999 with its counterpart.
- AT-RD18: A duplicated save event and a delete arriving before an
  older save leave the post deleted from the index.
- AT-RD19: When a node becomes private, every piece, vector, span and
  extra of its posts leaves the serving index within 60 seconds, and
  summaries that used them are rebuilt.
- AT-RD20: An erasure request purges the post's text, pieces, vectors,
  spans, extras and summary contributions, and any Dell lab copies.
- AT-RD21: The serving generation records the embedder's key and the
  tokenizer's identity; a different active key or tokenizer is reported
  to readiness as a mismatch.
- AT-RD22: With all pickle loading disabled, every production model
  loads in the form the owner chose (R106, R120): bge-m3's official ONNX
  export through ONNX Runtime at full precision, en_core_web_lg without
  PyTorch, the reranker and potion. A fixture model holding a pickle
  file is refused at install and load.
- AT-RD23: The file check classifies a spaCy file that wraps a PyTorch
  pickle archive as not data-only.
- AT-RD24: A 150,000-character post is read in full with peak service
  memory under the cap and checkpoints between pieces.
- AT-RD25: During a backlog run on the 50,000-thread tier, peak service
  memory stays under the service's cap (6 GB, or 8 GB once Dell
  measurements confirm the estimate, R121); with injected memory
  pressure, batches shrink and the backlog refuses new work, with a
  reason, before the cap.
- AT-RD26: Censored words are masked in stored text and pieces.
- AT-RD27: With Resource Manager present, a resource's description and
  updates are read with the resource title prefixed and its reviews
  are not read (Q-RD03); without it, nothing about resources is read
  and no error is raised.
- AT-RD28: With the hub module enabled, a hub's own text is read and
  its listed threads are recorded as links only; disabling the module
  removes hub pages from the index.
- AT-RD29: Progress shows totals in posts for each stage, the
  thread-level and per-post index included, skipped counts by reason,
  failed counts by code and an estimate from measured throughput
  (SPEC-011). Each lane's queue depth, the processing rate and the
  index coverage also appear on the review and health pages (R123).
- AT-RD30: Offsets of stored spans in text with four-byte emoji and NFD
  input agree with the PHP fixture's offsets in code points.
- AT-RD31: With potion unavailable, the sweep and the backlog continue
  in the same views-and-recency order and the first pass is shown as
  degraded.
- AT-RD32: No `ReadBatch` exceeds its byte cap, and the add-on's extra
  PHP memory per request stays under 8 MB, measured with the add-on
  enabled and disabled.
- AT-RD33: A 5 MB post arrives by ranged fetch in slices of at most
  1 MB, each request under PHP's 8 MB limit; the joined text matches
  its checksum and is read in full. Editing the post mid-fetch makes
  the fetch start again for the new version.
- AT-RD34: An input one token over a fake model's window is refused
  with an error code and reported; nothing is truncated.
- AT-RD35: Links inside quotes are recorded with the inside-a-quote
  flag and do not count against the R011 allowance.
- AT-RD36: For a fixture forum, the projected index size lies within
  10% of the measured size after reading; a projection over the budget
  stops backlog writing with a plain message before it is exceeded.
- AT-RD37: On the fixture, after the first pass, every eligible
  destination's title and summary vectors are written before any post
  outside the summary subsets is read, live work apart. The thread-level
  index is reported complete only when every eligible destination has
  both, and SPEC-009 makes no suggestion before then (R123).
- AT-RD38: Fixture threads with known view counts and last post dates
  are read in the views-and-recency order in both phases; changing the
  potion shortlist leaves the order unchanged (R123).
- AT-RD39: Coverage given to SPEC-009, progress and the review and
  health pages counts destinations in the thread-level index and posts
  read, each out of the eligible total; reading the last eligible post
  gives the per-post completion signal exactly once (R123).
- AT-RD40: A thread created during phase 1 gets its title and summary
  vectors in the live lane and is a destination at once.
- AT-RD41: The first generation serves once its thread-level index is
  complete. During a rebuild for a new embedder, the new generation is
  not served before it is complete, no new suggestion is made, and the
  switch changes the serving generation id in one step (R122).
