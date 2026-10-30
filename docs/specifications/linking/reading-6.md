# SPEC-008 part 6: Reading and indexing

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in the
[specification decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Requirements: as in [part 1](reading.md). Parts 2 to 4 hold the rest of
the design ([part 2](reading-2.md), [part 3](reading-3.md),
[part 4](reading-4.md)); part 5 is the
[acceptance cases](reading-5.md).

## Decisions (answered by the owner)

These choices needed the owner's judgement and are kept as history.
The first option of each was the drafter's recommendation. Questions
about model file formats were asked once, in SPEC-010 (MD-01 for
bge-m3, MD-02 for spaCy); their answers are R120 (part 4). Each answer
below is applied in the design.

- Q-RD01. The first reading run will take longer than "days". You said
  days are fine; the estimate is one to several weeks per million posts
  (measured later), because full-precision bge-m3 is slow on an
  ordinary processor. New and edited posts keep their 30-second target
  throughout. What should happen?
  - (recommended) Accept it: the run works in the background at the
    lowest priority, with a measured forecast on the admin pages.
  - Give the service more processor cores while the first run lasts;
    the time falls roughly in proportion.
  - Read the most recent years first and show older threads as "not
    yet read"; the total is the same, but recent content is ready
    sooner.

  Owner's answer: accept it, and also allow paid embeddings from Google
  and OpenAI. In follow-ups the owner made them an optional choice for
  the administrator, off by default, replacing bge-m3 for the whole
  index, and asked for a separate specification, drafted next (R126;
  part 1 holds the pointer). R123 later set the reading order:
  suggestions start once the thread-level index is complete (part 2).
- Q-RD02. You said other languages are skipped and shown as skipped,
  never silently mixed in. What about a mostly English post with a
  paragraph in another language?
  - (recommended) Skip that paragraph, read the English ones, and show
    the post as "partly skipped".
  - Judge the whole post by its main language and read all of it.

  Owner's answer: judge by main language. A post whose main language is
  English is read in full (part 1); P-RD10 then matters mainly for very
  short posts.
- Q-RD03. Should resource reviews be read?
  - (recommended) No: read each description and update in full, but
    not reviews, which are ratings; R011 counts a resource "including
    updates" only.
  - Yes, read guest-visible reviews like replies.
  - Read the description only.

  Owner's answer: no reviews.
- Q-RD04. How are hub pages read? R039 makes hubs optional collections
  of useful public material whose own links each need approval and
  follow the five-link default.
  - (recommended) Read a hub's own written text; record the threads it
    lists as links rather than reading their titles, so it does not
    seem to match every topic; hubs only receive links, because their
    own links are curated under R039.
  - Also read the listed titles, at low weight.
  - Also suggest new links inside hubs, each approved under R039.

  Owner's answer: own text only.
- Q-RD05. What does "highest-reacted" count?
  - (recommended) Reactions other than Sad and Angry, which R037 treats
    as negative.
  - All reactions equally.
  - XenForo's reaction score.

  Owner's answer: all reactions equally, Sad and Angry included.
- Q-RD06. ADR-0001 reads the first post "in full, up to the model's
  limit" for the summary. How?
  - (recommended) As the average of its pieces' vectors: the whole post
    counts, however long, with no extra model run or memory spike.
  - As one input of up to 8,192 tokens, the literal reading; longer
    first posts are cut there, and it costs more memory and time.

  Owner's answer: the average of its pieces.
- Q-RD07. How precisely are vectors stored? At 32-bit with 1,024
  numbers each they take about 4 GB per million pieces, and search
  reads all of them for each new post.
  - (recommended) 32-bit, matching your choice of full precision.
  - 16-bit: about 2 GB per million pieces and half the search time;
    similarity scores change very slightly. It does not shorten reading.

  Owner's answer: 32-bit.
- Q-RD08. Where does the vector index live? Your earlier answer (Q017)
  made a search service such as Elasticsearch or OpenSearch a
  requirement for linking.
  - (recommended) In the Python service's own data directory, searched
    exactly in blocks; deleted on uninstall with the other index data.
  - In the search service, using its exact scoring under its own memory
    allowance; probably slower.

  Owner's answer: the service's own folder. No vectors or index files
  are stored in the search service.

## Proposals

Technical choices, accepted as a group by the owner on 30 October
2026. Two were changed by later answers: P-RD07's order was replaced by
R123's, and P-RD10 has little effect under the answer to Q-RD02.

- P-RD01: The title counts inside a piece's 1,024 tokens, counted with
  the active embedder's own tokenizer, so no model reads more than the
  1,024 tokens R101 promises and a community model with R109's smallest
  window still reads every piece whole.
- P-RD02: An over-long paragraph splits at sentences; an over-long
  sentence splits at words.
- P-RD03: "About 15%" overlap means 12.5% to 20% of 1,024 tokens, in
  whole sentences where possible.
- P-RD04: A thread whose title and first post are not English is not a
  destination; its English replies are still read as sources.
- P-RD05: The title counts twice in the summary vector.
- P-RD06: The backlog pauses when the forum's own services wait more
  than 20% of the time over 10 seconds, and resumes after 60 seconds
  below 10%.
- P-RD07: After live work, the embedder reads threads near recent
  activity first, then threads with the most unlinked neighbours.
  Replaced by R123: the thread-level index first, then the per-post
  index, both by thread views and recency (part 2).
- P-RD08: Backlog model calls are one piece or one pair each, so live
  work waits for at most one call.
- P-RD09: The index size ceiling is 25 GB by default; projections
  assume 1.5 vectors per post until the first pass counts them.
- P-RD10: A paragraph under 20 letters takes the language around it.
  Kept; since posts are judged by their main language (Q-RD02), it
  matters mainly for very short posts.
- P-RD11: Extras are cut to 200 characters each and weigh half as much
  as body text in matching.
- P-RD12: Links inside quotes count against repeated connections
  (R012) but not against the link allowance (R011).
- P-RD13: Summary rebuilds after reaction changes wait at least 10
  minutes per thread.
- P-RD14: PHP batches and ranged-fetch slices carry at most 1 MB of
  text each.

## Recorder's notes

These are the drafter's interpretations, not owner decisions.

- "Piece" follows R101; "chunk" in the owner's request means the same.
- R107 ("new and edited posts go first") replaces Q018's earlier
  "finish existing threads first".
- "Link titles" is read as the titles of link previews XenForo already
  stores. Media titles are used only where XenForo stores them.
- Mentions and unknown BBCode are not in R101's skip list, so they are
  read, but they are never anchors (Q148).
- R101's "only new and edited posts are read again" is taken to include
  re-reading a thread after a title edit, because the title is in every
  piece.
- "The embedder has read the destination" (R098) was first taken to
  mean all of it. R123 replaced that reading: the destination is read
  through its title and summary first, and suggestions made before its
  per-post index is complete record the coverage and are re-scored
  (part 2).
- R123's "ordered by thread views and recency" is read as one score: a
  thread's view count divided by one plus the years since its last
  post, highest first, newer first on ties, so that a popular old
  thread and a busy recent one both come early. (Alternative: views
  first, recency only to break ties.) SPEC-009 uses the same order.
- After R123, potion's sweep keeps R098's early shortlist and gives the
  totals and the eligibility and reaction data the summaries need, but
  no longer decides what bge-m3 reads first: asked which order
  applies, the owner chose views and recency over matches.
- A post's main language (Q-RD02) is read as the language that holds
  most of its letters.
- R122's switch "when it is complete" is read as the whole new
  generation, per-post index included, as MD-04's option said: new and
  edited posts get no suggestions for the whole rebuild. Switching once
  the new thread-level index is complete would shorten the pause; that
  would be the owner's choice. The first generation serving before its
  per-post index is complete is read from R123 ("suggestions start as
  soon as it is complete"); "a half-built generation is never served"
  is read as a rule for rebuilds.
- The uninstall classes follow R050 as amended word for word: only
  vectors and the index's own files are always deleted; readable-text
  copies, spans and link records keep R050's export offer, as in
  SPEC-010's table, which is the authority. (Alternative: treat them as
  part of the index, always deleted, because they exist only to serve
  it.)
- The 400-character cut is made at or before a word boundary. Polls are
  not posts and are not read.
- Timings come from published parameter counts and arithmetic, not
  measurements. The model file findings are the coordinator's Dell
  check of 30 October 2026.
- The language identifier, the sentence splitter, ONNX Runtime and
  en_core_web_lg need admission records (R027; R120); any model file
  the identifier needs is pinned in `models.lock` (SPEC-010).
