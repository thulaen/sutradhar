# SPEC-008 part 2: Reading and indexing

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in the
[specification decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Requirements: as in [part 1](reading.md), which holds the scope and how
text is read. Part 3 is [stored data, interfaces and
settings](reading-3.md); part 4 is [model files, failure, privacy,
performance and testing](reading-4.md); part 5 is [acceptance
cases](reading-5.md); part 6 is [the owner's decisions](reading-6.md).

## Design, continued

### Existing links and state

- For every guest-visible post, whether it is read in full or skipped
  (another language, or no readable text), the service records its
  state and its existing links (R101). State means the visibility PHP
  reports, the language result, and read or skipped.
- Python lists each post's links: `[URL]` links in all three forms,
  bare URLs that XenForo turns into links, and link previews (Q148
  starred case). Internal URL forms are parsed with the board settings
  PHP supplies; PHP then resolves redirects and merges to canonical ids
  in one batched call.
- Stored per link: its position, whether it sits inside a quote, and
  the target's kind and canonical id, or "external".
- Links inside quotes do not use up R011's allowance, because R011 says
  quotes do not consume it. They still count as an existing connection
  for R012's no-repeat rule, since the page already shows that link
  (P-RD12). SPEC-009 applies both counts.

### Incremental re-reading

- Each item carries a content version: a hash of its readable text,
  extras, title and the reading-rules version. Only a changed version
  is read again (R101).
- An edit that changes only skipped zones, such as a quote, changes no
  piece, so nothing is re-read; its links and the new version are
  recorded.
- New and edited posts go to the live lane.
- A thread title edit changes every piece, because the title is
  prefixed. The first post and the summary are re-read in the live
  lane, the other posts in the backlog lane. Old pieces keep serving
  until each is replaced.
- Moved, merged and split threads: moved posts are treated as edited,
  and their visibility is checked again.
- Deletion, loss of guest visibility or an erasure request removes the
  item from the serving index at once and purges its stored data
  (R052; Q148 starred cases). Affected summaries are rebuilt.
- A daily reconciliation compares PHP's list of ids, last-edit times
  and visibility with the index and repairs differences (Q148 starred
  case: lost events).

### Thread summary vector (R102, ADR-0001)

- The subset is as ADR-0001 states: the title (weighted); the first post
  in full; the accepted answer, if any; the 5 highest-reacted replies;
  the first 5 replies; the 3 most recent posts; then, after duplicates
  are removed, the next most-reacted replies until there are 20 posts.
  The counts, the total and the 400-character cut are settings with
  these defaults.
- Only guest-visible posts that are read (main language English) are
  chosen; an ineligible post is passed over and the next one takes its
  place.
- "Highest-reacted" counts all reactions equally, Sad and Angry
  included: a post's reactions of every type are added up (Q-RD05).
- Each selected post except the first is cut to its first 400
  characters of readable text, at or before a word boundary, with
  quotes, code, spoilers and signatures already excluded.
- The vector is the weighted average of normalised embedder vectors:
  the title (weight 2, P-RD05), the first post and each other selected
  post's cut text (weight 1), normalised again. The first post's vector
  is the average of its pieces' vectors, so the whole post counts,
  however long, with no extra model run (Q-RD06).
- Summary text, which SPEC-009's reranker reads: the title, the first
  post's readable text, then each selected post's cut text in subset
  order. SPEC-009 caps the first post's share of it in the reranker's
  input.
- Cut-text vectors of selected posts are kept, so a rebuild reads only
  newly selected posts. Rebuilds after reaction changes wait at least
  10 minutes per thread (P-RD13) to batch rapid changes.
- Changing a summary setting rebuilds the affected summaries in the
  background with progress, as ADR-0001's note asks. Each thread's old
  summary serves until its new one is written.
- Resources use the same rule with updates in place of replies and no
  accepted answer. Hub pages use their title and own text (Q-RD04).

### First pass with potion-retrieval-32M

- potion is a static model: it looks up a stored vector for each token
  and averages them, so it is very fast on an ordinary processor.
- At the start of the first backlog run it sweeps every eligible item:
  extraction, links and state, language, splitting and potion vectors.
  This gives exact totals, so progress estimates are honest from the
  start, and each thread's eligible posts with their reactions, which
  the summary subsets need. The sweep's time is measured and shown.
- It gives an early shortlist: each thread's nearest threads by exact
  search over potion vectors. The shortlist feeds progress counts only;
  it does not order reading (R123). It is never a suggestion and never
  reaches moderators (R098). What happens when potion's revision
  changes is SPEC-010's MP-03.

### Reading order: thread-level index first (R123)

This replaces P-RD07's neighbour order and the earlier reading of R098
as "the embedder has read all of the destination".

- The live lane is always first: new and edited posts are read within
  seconds, per post, never per batch. A new thread's title and summary
  are made in the live lane, so it is a destination at once.
- Phase 1, the thread-level index: for each eligible destination, the
  embedder reads its title and its summary subset (the first post in
  full, as its pieces, and each other selected post's cut text) and
  writes the title and summary vectors. The first post's pieces join
  the per-post index as they are written. When every eligible
  destination has both vectors, the thread-level index is complete and
  SPEC-009 may start making suggestions: bge-m3 has then read both
  sides, the destination through its title and summary (R098 as
  changed by R123). By the recorder's estimate in R123 this takes about
  a day or two for 50,000 threads on an ordinary processor, which the
  owner accepted; measured figures replace it.
- Phase 2, the per-post index: the remaining posts are read in the
  backlog lane, one thread's remaining posts together before the next
  thread starts. Each post's suggestions can appear the moment it is
  read (SPEC-009, whose Q-MA07 limits which existing posts are scored).
- Both phases take threads in R123's backfill order, by thread views
  and recency: highest first by the thread's view count divided by one
  plus the years since its last post, newer first on ties (recorder's
  note in part 6). Resources and hub pages are ranked by the same rule,
  counting no views where none are recorded. SPEC-009 scores existing
  posts in this order.
- Coverage (R123): reading counts, for the serving generation,
  destinations in the thread-level index and posts read, each out of
  the eligible total. SPEC-009 records the coverage with each suggestion
  it makes before the per-post index is complete. When the last
  eligible post is read, reading tells SPEC-009 once that the per-post
  index is complete, and SPEC-009 re-scores those suggestions, replacing
  a pending one if a clearly better target is found.

### The first backlog run (R107)

- Two lanes, R123's two queues: live (new and edited posts, visibility
  changes) and backlog (R123's backfill queue for the historical
  catalogue, in the order above). Live work is always taken first.
- A backlog slice is one model call, and backlog calls are kept small:
  one piece per embedder call, a token-limited group of paragraphs per
  spaCy call, and one pair per reranker call in SPEC-009's backlog
  matching (P-RD08). After each backlog call the scheduler serves any
  waiting live work before starting the next.
- A model call cannot be stopped part-way, so live work waits at most
  for the one backlog call already running. Worst case for reading: one
  full 1,024-token bge-m3 piece, about 2.4 to 5 seconds on the reference
  hardware (4 processor cores and a SATA solid-state disk, R124; an
  estimate to be measured). Typical pieces are shorter and quicker.
  SPEC-009 states the wait for its reranker calls.
- The backlog lane runs at the lowest processor priority and the idle
  disk priority. The live lane runs below normal priority but above the
  backlog.
- The backlog pauses under forum load (P-RD06). Linux reports, for
  each group of processes, the share of time they spend waiting for the
  processor or disk ("pressure"). The pause watches the forum's own
  services (PHP, the web server and the database), identified at setup,
  not the whole host: otherwise the backlog's own work would make the
  host look busy and pause itself. It pauses when that pressure stays
  above 20% over 10 seconds and resumes once it stays below 10% for 60
  seconds. If the forum's services cannot be identified, the host total
  minus Sutradhar's own share is used, and the admin page says so. The
  live lane does not pause.
- On the PHP side the backlog feeder runs as a XenForo job with a short
  time limit, reading small batches by id cursor for the sweep, or by
  the ids Python asks for in the reading order, never by offset (Q148
  starred cases).
- The first run is long: about one to several weeks per million posts
  by the estimate in part 4, against the owner's "days are fine". The
  owner accepted this (Q-RD01). Suggestions do not wait for it: they
  start once the thread-level index is complete (R123). The estimate
  shown on the admin pages always comes from measured throughput
  (SPEC-011), never an invented figure. A paid embedder (R126; part 1)
  cannot shorten bge-m3's run, because vectors from different models
  cannot share an index; it replaces bge-m3 for the whole index.
- A batch is a group of calls holding at most 30 seconds of model work;
  a checkpoint follows every committed batch, so a crash repeats at
  most one minute of work per worker (R020).
- Progress shows the stages (first pass, thread-level index, per-post
  index, entities, summaries), counts read and skipped by reason,
  failed by error code, and any paused state with its reason (R008,
  R110). Each lane's queue depth, the processing rate and the index
  coverage are also shown on the review page and the health page
  (R123; SPEC-011).
