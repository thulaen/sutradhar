# 0022: Reading and indexing

Reading turns each guest-visible post, resource and hub page into
readable text, pieces, vectors and name records, and keeps the index
that matching (spec 0023) searches. It runs in the service on the
owner's PC. It is roadmap Phase 4 item 3.

## Words used here

- Token: the unit a model counts text in; roughly four tokens for
  every three English words. Each model counts in its own way.
- Piece: a stretch of one post's text, at most the piece size, that a
  model reads at once. The piece size is 1,024 tokens by default and
  never fewer; the owner may set it higher.
- Vector (embedding): a list of numbers a model writes for a text;
  texts with similar meaning get similar vectors.
- Embedder: the model that writes the vectors: Qwen3-Embedding-0.6B at
  its published precision, unless an administrator turns on a
  community model (spec 0024) or paid embeddings (spec 0025).
- Index: the stored vectors, plus records of which post, thread,
  resource or hub page each belongs to.
- Thread-level index: each destination's title and summary vectors,
  built first so that suggestions can start. Per-post index: the
  vectors of every post's pieces and extras, built after it.
- Coverage: destinations in the thread-level index and posts read,
  each out of the eligible total.
- potion (potion-retrieval-32M): a fast, simple model for a first
  sweep. The entity model, our own and built in pure Rust (spec 0026),
  finds names (entities); the same pure-Rust layer finds sentences and
  noun phrases.
- Lane: a queue of reading work with its own priority.
- Checkpoint: a saved record of finished work, so a restart carries on.
- Guest-visible: content a logged-out visitor may see. Only this is
  ever read.
- NFC: one standard way of writing accented letters, so the same word
  is always stored the same way.
- Code point: one character as Unicode numbers it. Positions are
  counted in code points, so an emoji counts as one character and the
  add-on and the service agree on where a word starts.
- Ranged fetch: asking for a very long post one slice at a time.

## What it does

### Scope

- Reads guest-visible thread posts, resource descriptions and updates
  when XenForo Resource Manager is present, and approved hub pages when
  the hub module is on.
- Out of scope here: search, reranking, anchors and blocking
  suggestions (spec 0023); model files, `models.lock`, community
  models, the rule against retraining, the uninstall table and the memory table
  (spec 0024); paid embeddings (spec 0025); progress and the live
  stream (spec 0016); visuals (spec 0017); the parser and position map
  (spec 0026); the signed contract (spec 0021); jobs (spec 0019);
  settings storage (spec 0014); the review page, insertion and undo
  (spec 0028).

### Pipeline

Each stage commits its result before the next starts, so a crash
repeats at most the unfinished stage.

1. Receive: the service fetches guest-visible items, from an event or
   from the backlog cursor; a very long post comes by ranged fetch. A
   WordPress post over 16 MiB is read in parts of at most 16 MiB.
2. Extract: the parser (spec 0026) turns stored BBCode into typed
   segments.
3. Record: existing links and state, for every guest-visible post,
   read further or not.
4. Language: a post whose main language is English goes on in full;
   any other is skipped and shown.
5. Split: pieces with the title prefix and overlap; extras gathered.
6. First pass, during a backlog sweep only: potion vectors (never a
   suggestion).
7. Full reading: embedder vectors for pieces, extras and the title.
8. Entities: sentences, entities and noun phrases.
9. Summary: the post's vector is added to its thread's running sum,
   and the thread's summary vector is written again from that sum.

After the first pass, the embedder builds the thread-level index
first, then the per-post index, with new and edited posts always
first.

### What is read

- Thread posts: every guest-visible post, first post and replies, with
  no size limit and no staff warning.
- Resources (only with Resource Manager): the description, read like a
  first post, and each guest-visible update, read like a reply; pieces
  carry the resource title. Reviews are never read. A resource's
  discussion thread is read as a normal thread.
- Hub pages (only with the hub module on): the hub's own written text.
  The threads it lists are recorded as links, not read. Hubs only
  receive links.
- Never read: private conversations, drafts, previews, staff-only or
  members-only content, profile posts, signatures, polls and anything
  the add-on does not mark guest-visible.

### Extraction and skipped zones

The parser works from the stored message, never the rendered page. It
returns segments of these kinds:

- Skipped: quotes at any depth with their attribution line; code
  (`[CODE]`, `[ICODE]`, `[PHP]`, `[HTML]`); spoilers (`[SPOILER]`,
  `[ISPOILER]`). Skipped text is in no piece and is never an anchor.
  Signatures are not in the stored message and are never sent.
- Body: ordinary text, formatting, lists, headings, tables, `[PLAIN]`
  content, the visible text of existing links, mentions and the content
  of unknown add-on BBCode. Mentions and unknown BBCode are read but
  marked never-anchor.
- Extras: image alt text, stored link-preview titles, attachment file
  names and stored embedded-media titles. Nothing is fetched from the
  internet to find a title.
- Masked: words the forum's censor would hide. The add-on sends their
  positions; they become a neutral mask, so they are neither read nor
  stored in clear.

Readable text is NFC, as XenForo stores it. Every position is in code
points. The parser supplies a map from readable text back to the
stored message.

### Language (English only)

- A language identifier (lingua, subject to admission) checks each
  paragraph.
- A post is judged by its main language: the one holding most of its
  letters. An English post is read in full, other-language paragraphs
  included, and their count is recorded. Any other post is skipped and
  shown as "skipped: not English" with the detected language.
- A paragraph under 20 letters takes the language of the rest of its
  post, or of its thread if the post has nothing else.
- A skipped post has no pieces, vectors or entity records and is never
  an anchor; its links and state are still recorded.
- Skipped counts appear in the progress view and per thread on the
  admin pages.
  An edit runs the check again.
- A thread's language comes from its title and first post. A thread
  that is not English is never a destination; its English replies are
  still read as sources.

### Pieces

- Tokens are counted with the active embedder's own tokenizer, start
  and end markers included. The tokenizer's identity (files and
  checksums) is part of the generation key, so pieces cut for one model
  are never read by another.
- Piece size: `reading.pieces.tokens`, in the embedder's tokens, 1,024
  by default. It is never below 1,024; the owner may set it higher,
  with no fixed top (the owner, 31 October 2026: "1024 or higher"; and
  10 November 2026: "No fixed top"). The top is set only by the
  embedder's window, the 64 KiB text cap per piece and the memory check
  below. Spec 0023 records the
  reranker tokens this maps to, measured, and always lets the best
  piece into the reranker.
- A piece size larger than the active embedder's window is refused
  with a plain message, such as "This piece size is longer than the
  embedder can read" (spec 0024, MF-C07).
- A size above 1,024 is allowed only once a measured run on that PC
  shows the embedder stays within its row in spec 0024's memory table
  (1,300 MB under 16 GB, 1,400 MB at 16 GB or more) at that size, and
  that the time from saving a post to its first suggestion still meets
  spec 0023's limit at that size. Otherwise the setting is refused
  with a plain message, such as "This piece size needs more memory
  than the embedder has" or "This piece size makes the first
  suggestion too slow", and the old size stays.
- A piece is the thread title, a blank line, then body text, all
  within the piece size and within 64 KiB of UTF-8 text, a hard cap.
  Whichever limit comes first ends the piece, by the same split rules.
- A paragraph is text between line breaks or between block tags (list
  items, headings, table rows).
- Whole paragraphs are packed until the next would not fit. A post
  that fits in one piece has exactly one.
- Overlap: `reading.pieces.overlap_tokens`, 64 by default, from 0 to a
  quarter of the piece size. Each further piece starts with the last
  whole sentences of the one before, added from the end while they fit
  in the overlap. If the last sentence alone is longer, the overlap is
  the last whole words that fit. Only a setting of 0 gives no overlap.
- A paragraph over the body budget splits at sentence boundaries, found
  by a rule-based splitter (no model needed); a sentence over the
  budget splits at words, with the same overlap.
- No cut falls inside a word, a combining sequence or an emoji
  sequence, with one exception: a single word longer than a piece, in
  tokens or in bytes, is split at grapheme boundaries, and the split is
  counted and shown. Every readable character is in at least one
  piece: nothing is truncated, however long the post.
- Changing the piece size or the overlap re-splits every post. It is a
  rebuild for new reading rules: the serving generation and
  suggestions keep running until the new one is whole.
- Before every model call the input is counted again with that model's
  tokenizer. An input over the model's window, or a piece over the
  piece size, is refused with an error code and reported as a defect.
  Nothing is ever cut silently.

### Very long posts

- XenForo's largest post is 16 MiB, the size of its message column.
  16 MiB is also the most the text engine takes as one message (spec
  0026).
- A WordPress post over 16 MiB is read in full, in parts of at most
  16 MiB, so nothing is cut or refused. Each part is fetched in slices
  of at most 1 MiB by `ReadRange`, the WordPress plugin's ranged fetch
  (spec 0021). It is then parsed, split, read and dropped from memory
  before the next.
- A part is a fixed byte range of at most 16 MiB, ending on a
  character boundary. The parser's open tags and positions carry into
  the next part, and so do the unfinished paragraph, piece and
  overlap. So a part edge needs no paragraph boundary, and the pieces
  are the same as if the post were read whole.
- Checksums: each part's checksum is checked before the part is
  parsed and before any of its results are stored. Its pieces,
  vectors, spans and links are stored as staged, and none of the
  post's pieces is served until the whole post's checksum passes. Then
  the staged results replace the old version in one step.
- If a part's or the whole post's checksum fails, or the post changes
  part-way, every staged result of that read is dropped and the new
  version is read from the start. The old version serves meanwhile.

### Extras

- Each extra is read in full. A post's extras are joined and read as
  their own title-prefixed inputs within the piece size; more extras
  form another input. An extra longer than one input splits like body
  text, with the same overlap.
- Extras vectors are stored apart from pieces, carry weight 0.5 in
  matching and are flagged never-anchor.

### Entities, sentences and noun phrases

- The entity model runs on the body text of each read post, paragraph
  by paragraph in small batches, so memory stays bounded. Extras are
  not parsed.
- Stored for matching: sentence boundaries, named entities of the
  types spec 0023 allows with their labels, and noun phrases.
  Destination titles are parsed the same way.
- Not stored: other entity types, the grammar tree, word vectors.
- If the entity model is unavailable, vectors are still written; the
  post shows "waiting for entity reading" and suggestions that need it
  are withheld.

### Existing links and state

- For every guest-visible post, read or skipped, the service records
  its state (visibility as reported, language result, read or skipped)
  and its existing links.
- Links listed: `[URL]` links in all three forms, bare URLs the forum
  turns into links, and link previews. Internal URL forms are parsed
  with the board settings; the add-on then resolves redirects and
  merges to canonical ids in one batched call.
- Stored per link: position, an inside-a-quote flag, and the target's
  kind and canonical id, or "external".
- Links inside quotes do not use up the thread's link allowance. They
  still count as an existing connection for the no-repeat rule.

### Incremental re-reading

- Each item has a content version: a hash of its readable text,
  extras, title and the reading-rules version. Only a changed version
  is read again.
- An edit that changes only skipped zones changes no piece; only its
  links and the new version are recorded.
- New and edited posts go to the live lane.
- A thread title edit changes every piece. The title, the first post
  and the accepted answer are re-read in the live lane, the other posts
  in the backlog lane. Old pieces serve until each is replaced, and
  each re-read post swaps its old share of the thread's sum for its
  new one.
- Moved, merged and split threads: moved posts count as edited, and
  their visibility is checked again.
- Deletion, loss of guest visibility or an erasure request removes the
  item from the serving index at once and purges its stored data. Its
  share leaves its thread's sum in the same step.
- A daily reconciliation compares the add-on's list of ids, last-edit
  times and visibility with the index and repairs differences.

### Thread summary vector

- A thread's summary vector is built from every read post, each read
  in full. No post is picked out and no text is cut.
- A post's vector is the average of its pieces' normalised vectors,
  normalised again, so the whole post counts with no extra model run.
- Four parts, each with its own weight: the title; the first post; the
  accepted answer, if any; and the other posts together, as the
  normalised average of their post vectors. The vector is the weighted
  sum of the parts, normalised again. A missing part is left out.
- The title always weighs most. The first post and the accepted answer
  each weigh more than the other posts together. Settings that break
  this order are refused.
- The starting weights are 0.4, 0.25, 0.2 and 0.15. The Dell lab tunes
  them by link reconstruction (spec 0023): the forum's existing links
  are rebuilt, and the weights that rebuild them best, within the
  order above, become the defaults.
- Only guest-visible posts that are read count. Reactions do not
  choose or weigh posts.
- Streaming: no thread is ever held whole. Each destination keeps a
  running sum of its other posts' vectors at 64-bit precision, with
  their count. A newly read post adds its vector. An edit takes out
  the old vector, worked out from the old pieces before they are
  removed, and adds the new one. A removal takes its vector out.
  Memory per thread is one sum, 8 KiB, whatever the thread's length.
- The accepted answer is not in the other posts' sum. When an answer
  is marked, changed to another post or unmarked, that thread's sum is
  worked out again from its stored per-post vectors (each the average
  of the post's stored piece vectors), one post at a time, with no
  model run and no re-reading. The summary vector is then written
  again. A newly marked answer leaves the sum; an unmarked one joins
  it.
- The summary vector is written again from the sum and the three other
  parts after each change. The daily reconciliation works out each
  sum changed that day again from the stored pieces, one post at a
  time, so rounding never builds up.
- Changing a summary weight rewrites the summary vectors from the
  stored sums in the background, with progress, with no model run.
  Each old vector serves until its new one is written.
- Resources use the same rule, with updates for replies and no
  accepted answer. Hub pages use their title and own text.

### First pass with potion

- potion looks up a stored vector for each token and averages them, so
  it is very fast on an ordinary processor.
- At the start of the first backlog run it sweeps every eligible item:
  extraction, links and state, language, splitting and potion vectors.
  This gives exact totals for honest progress estimates, and each
  thread's eligible posts for the summaries. The
  sweep's time is measured and shown.
- It gives an early shortlist of each thread's nearest threads, by
  exact search over potion vectors. The shortlist feeds progress
  counts only. It never orders reading, is never a suggestion and never
  reaches moderators. A change of potion's revision is spec 0024's.

### Reading order: thread-level index first

- The live lane is always first: new and edited posts are read within
  seconds, per post, never per batch. A new thread's title and summary
  are made in the live lane, so it is a destination at once.
- Phase 1, the thread-level index: for each eligible destination the
  embedder reads its title, its first post in full and its accepted
  answer in full, and writes the title vector and a first summary
  vector from them. Their pieces join the per-post index as written.
  When every eligible destination has both vectors, the thread-level
  index is complete and suggestions may start. Estimate: a day or two for
  50,000 threads on an ordinary processor; measured figures replace it.
- Phase 2, the per-post index: remaining posts are read in the backlog
  lane, one thread's posts together before the next thread. Each post
  joins its thread's running sum as it is read, and the summary vector
  is written again at each checkpoint and when the thread is fully
  read. Until then the summary vector holds the posts read so far, and
  coverage records that. Each post's suggestions can appear the moment
  it is read.
- Both phases take threads highest first by view count divided by one
  plus the years since the last post, newer first on ties. Resources
  and hub pages use the same rule, counting no views where none are
  recorded. Matching scores existing posts in this order.
- Coverage is counted for the serving generation. Matching records it
  with each suggestion made before the per-post index is complete.
  When the last eligible post is read, reading signals matching once
  that the per-post index is complete, and matching re-scores.

### The first backlog run

- Two lanes: live (new and edited posts, visibility changes) and
  backlog (the historical catalogue, in the order above). Live work is
  always taken first.
- A backlog slice is one model call: one piece per embedder call, a
  token-limited group of paragraphs per entity call, and one pair per
  reranker call in backlog matching. After each backlog call, waiting
  live work is served.
- A model call cannot be stopped part-way, so live work waits at most
  for one backlog call: in the worst case one full piece. At 1,024
  tokens that is about 3 to 6 seconds on the reference hardware (4
  processor cores and a SATA solid-state disk), to be measured. A
  larger piece size makes the wait longer; the settings screen shows
  the measured wait before the change is made.
- The backlog lane runs at the lowest processor priority and idle disk
  priority. The live lane runs below normal but above the backlog.
- The backlog pauses under load. It watches pressure (the share of
  time processes wait for processor or disk) on the owner's PC from
  programs outside Sutradhar's slice. It pauses when pressure stays
  above 20% over 10 seconds and resumes once it stays below 10% for 60
  seconds. The live lane never pauses.
- On the site, the add-on feeds the backlog as a XenForo job with a
  short time limit, reading small batches by id cursor for the sweep,
  or by the ids the service asks for, never by offset.
- The first run takes about one to several weeks per million posts.
  Suggestions do not wait for it: they start once the thread-level
  index is complete. The admin estimate always comes from measured
  throughput. A paid embedder cannot shorten the local embedder's run;
  it replaces the local embedder for the whole index.
- A batch holds at most 30 seconds of model work, and a checkpoint
  follows every committed batch, so a crash repeats at most one minute
  of work per worker.
- Progress shows the stages (first pass, thread-level index, per-post
  index, entities, summaries), counts read and skipped by reason,
  failed by error code, and any pause with its reason. Each lane's
  queue depth, the processing rate and the coverage also show on the
  review and health pages.

### Data stored

Reading records live in PostgreSQL under the site's row-level
security. Vectors live in segment files in the service's own data
folder, readable only by the service account. Nothing is stored in a
search service. All of it is derived from guest-visible content and
purged when that content is deleted, hidden or erased.

Per item (post, resource description or update, hub page):

- ids, container, position, dates, content version, reading-rules
  version;
- state: visibility, main language, and read (with the count of
  other-language paragraphs), skipped (with reason) or failed (with
  code);
- existing links with positions, inside-a-quote flag and canonical
  targets;
- readable text with masks, and the position-map version;
- positions of skipped zones and extras, for the "post X-ray" view;
- pieces: offsets, token count, text, embedder vector, potion vector;
- extras: kind, text and vector, flagged never-anchor;
- for a post read in parts: the part boundaries and the running
  checksum, until the post is fully read;
- sentence boundaries, allowed entities with labels, noun phrases, and
  the entity model's key.

Per destination (thread, resource, hub page):

- canonical id, title, title vector, language, eligibility;
- view count and last post date, for the reading order;
- in the serving generation, whether its title and summary vectors are
  written, and how many of its eligible posts are read;
- summary vector; the running sum of the other posts' vectors (64-bit)
  and their count; the summary settings version.

Per index generation:

- generation id (the index version each suggestion records); the
  embedder's compatibility key (entry id, repository, revision,
  index-compatibility version, vector size); the tokenizer's identity;
  potion's key; the reading-rules version; start and completion times,
  and when its thread-level and per-post indexes finished; item and
  vector counts; coverage.

Vectors are kept at 32-bit precision in fixed-size segment files,
with a table from row to piece and a list of removed rows that
background compaction clears. Matching scans the segments in blocks.

### Uninstall classes

Spec 0024 owns the uninstall table. Reading's stores fall as follows:

- Always deleted, with no keep option and no export: every vector
  (pieces, extras, titles, summaries, running sums, first pass), segment
  files, row tables, removed-row lists and generation records.
- With the export offer and keep-or-delete choice: the reading records
  the index is built from (state, skip reasons, language, readable-text
  copies, piece and extras text, entity and noun-phrase spans, link
  records), reading checkpoints and the backlog
  cursor. Reading's settings follow spec 0024's rule for settings.

### Index generations and the revision check

- A generation is the full set of vectors made by one embedder and
  tokenizer under one reading-rules version. Exactly one serves.
- A rebuild (new model revision, tokenizer or reading rules) writes a
  new generation beside the serving one and switches in one step only
  when it is whole. A half-built generation is never served in a
  rebuild. The update flow and rollback are spec 0024's.
- While a new embedder rebuilds the index, new suggestions pause;
  suggestions already made stay approvable. A rebuild for new reading
  rules alone keeps the serving generation and suggestions running.
- The first generation serves once its thread-level index is
  complete, with coverage recorded, while its per-post index is built.
- At start-up and whenever a model loads, the active embedder's key
  and tokenizer are compared with the serving generation's. A
  difference is reported to readiness; no new suggestions are made and
  readiness says "not ready for suggestions" until the rebuild ends.
  Suggestions already made stay approvable.
- Entity records whose key differs from the active entity model's are
  stale: they count as missing evidence, are re-read in the backlog
  lane, and suggestions from those posts are withheld.

### The index size budget

Spec 0024's community-model check and install preflight use this.

- Projected bytes = V × D × B + V × W + T + F, where V is the number of
  stored vectors (pieces, extras inputs, titles, summaries, and each
  running sum counted twice, as it is 64-bit); D is the vector size
  (1,024, the size Qwen3-Embedding-0.6B's model card gives); B is 4
  bytes per number; W is the row-table cost, about 16 bytes per vector;
  T is stored text and records (2 KB per piece until measured); F is
  the first-pass store (its vectors × potion's vector size × 4 bytes).
- Before the first pass, V is 1.5 per post; afterwards the exact count
  is used.
- Example: a million posts give about 1.5 million vectors, about 6 GB
  of vectors and about 3 GB of text and records. A rebuild needs room
  for a second generation until the switch.
- `reading.index.budget_gb` (default 25) is a ceiling, not a
  reservation. Install and rebuild warn when the projection passes 80%
  of it. The backlog stops writing, with a plain message, before the
  index would exceed it; the serving index stays usable.

### Messages between the add-on and the service

Signing, replay protection and schemas are spec 0021's. The service
never reads the site's database; the add-on supplies the data.

- `ReadBatch` (add-on to service, from the cursor or on request): for
  each item, kind, id, container id and title, the container's view
  count and last post date, section id, prefix id, position, first-post
  and solution flags, post and edit dates, stored message, reaction
  counts by type, attachment names, stored preview and media titles,
  censor positions, content version and `guest_visible`, which must be
  true. A batch holds at most 1 MiB of message text, so the add-on's
  extra memory stays under 8 MB per request.
- A message over the batch cap is sent as a header only: length in
  code points, content version and checksum.
- `ReadPart` (the ranged fetch): one slice of one message (start and
  length in code points, at most 1 MiB) for that content version. The
  add-on cuts only between characters. The service joins the slices
  and checks the checksum; if the post changed, it starts again.
- `ContentEvent`: post saved, edited, soft-deleted, hard-deleted or
  restored; thread title edited, moved, merged or split; section or
  thread visibility changed; resource or hub page changed; erasure
  requested. Events carry ids and versions only. The add-on keeps them
  in its database-backed queue and the service collects them, then
  fetches content as above.
- `ResolveLinks`: internal link targets to canonical ids, following
  redirects and merges.
- `ListingPage` (for reconciliation): ids, last-edit times and
  visibility by id cursor; for threads also view counts and last post
  dates, which refresh the reading order.
- `ReadStatus` (service to add-on): per item, read, skipped with
  reason, or failed with code, for the admin pages.
- `BoardSettings`: board address form, friendly-URL setting and route
  prefixes for parsing links; fetched at start-up and when they change.

Inside the service, reading offers matching through its public API:
the serving generation's metadata and segments, piece texts, extras,
spans, existing links, summaries, destination eligibility, coverage,
and a one-time signal when the per-post index is complete.

### Settings and defaults

Settings use spec 0014, with versioned, atomic changes; each batch
records the settings version it used. Settings move only within
approved bounds.

Fixed, not settings: pieces never below 1,024 tokens; every post,
extra and part read in full; parts of at most 16 MiB for a longer
WordPress post; the order of the summary weights; the title in every
piece; quotes, code, spoilers and
signatures skipped; English only, by main language; dense vectors at
32-bit; Qwen3-Embedding-0.6B at its published precision; the
thread-level index first; the views-and-recency order.

| Setting | Default |
|---|---|
| `reading.pieces.tokens` | 1,024 (1,024 or more; no fixed top) |
| `reading.pieces.overlap_tokens` | 64 (0 to a quarter of the piece size) |
| `reading.summary.title_weight` | 0.4 (tuned on Dell) |
| `reading.summary.first_post_weight` | 0.25 (tuned on Dell) |
| `reading.summary.accepted_weight` | 0.2 (tuned on Dell) |
| `reading.summary.other_posts_weight` | 0.15 (tuned on Dell) |
| `reading.extras.weight` | 0.5 |
| `reading.language.min_letters` | 20 |
| `reading.index.budget_gb` | 25 |
| `reading.backlog.enabled` | true |
| `reading.batch.max_seconds` | 30 |
| `reading.backlog.pause_pressure_percent` | 20 (over 10 seconds) |
| `reading.backlog.resume_pressure_percent` | 10 (over 60 seconds) |
| `reading.php.batch_bytes` | 1,048,576 |
| `reading.php.part_bytes` | 1,048,576 |
| `reading.reconcile.interval_hours` | 24 |

Changing a summary weight rewrites the summary vectors in the
background from stored sums, with no model run. Changing the piece
size or the overlap is a rebuild for new reading rules.

### Model files

- "Data-only" means a file that cannot run code when opened. Files
  that can (such as pickle files) are never loaded.
- Qwen3-Embedding-0.6B is used only through its publisher's official
  safetensors files, a data-only format.
- The entity model is our own, trained in the Dell lab and shipped as
  safetensors (spec 0026).
- The reranker (gte-reranker-modernbert-base) and potion ship
  safetensors files.
- For every revision spec 0024 pins: every file is listed with size
  and SHA-256 and classified by content, not name; containers are
  opened and checked inside, and code-carrying data at any depth fails;
  only passing files go in `models.lock`, and only those are
  downloaded; the loader checks each file's format again before opening
  it and never runs code from a model's repository.
- Reading uses models only through spec 0024's model interface.
- The language identifier, sentence splitter, model runtime and entity
  model each need an admission record; any file the identifier needs is
  pinned in `models.lock`.

### Failure behaviour

Every failure shows in plain English with an error code and a manual
link. Missing results show as waiting or failed, never invented or
treated as zero.

- Embedder missing or failed: full reading stops, items wait visibly,
  no suggestion can start, readiness is "not ready".
- Entity model missing or failed: vectors continue; items show
  "waiting for entity reading"; suggestions that need it are withheld.
- potion missing or failed: potion vectors and the shortlist are
  skipped and the first pass shows as degraded; the rest of the sweep
  and the order are unchanged.
- Language identifier failed: the item waits and is retried, never
  assumed English.
- Input too long for a model: refused with a code and reported as a
  defect.
- One item fails extraction or reading: it is marked failed with its
  code and retried with growing delays; the rest continue.
- Ranged fetch interrupted, or the post changed: the fetch starts again
  for the current version; partial text is never read.
- Memory nearing the cap: batches shrink, then the backlog refuses new
  work with a stated reason, before the hard cap. The live lane keeps
  its reserved share.
- Index size budget reached: the backlog stops writing with a plain
  message; the serving index stays usable.
- Crash or forced stop: work resumes from the last checkpoint. Writes
  are idempotent, keyed by item id, content version and model key.
- Site unreachable: the backlog waits; events stay in the add-on's
  queue.
- Duplicate or out-of-order events: versions decide; a deletion is
  never undone by an older save.
- Disk full: writing stops, the serving generation stays usable, and
  the fault is reported.
- Key mismatch: reported to readiness; no new suggestions; those
  already made stay approvable.

### Privacy and security

- Only guest-visible content enters. The service refuses, and logs by
  id and code only, any item whose `guest_visible` is not true.
- Visibility loss, deletion and erasure remove content from serving at
  once and purge text, vectors, spans and summary contributions.
  Matching also filters results by current visibility.
- Vectors are treated as content, since text can partly be recovered
  from them: they follow the same deletion, erasure and lab-purge
  rules, and are always deleted at uninstall with no export.
- Logs, metrics, traces and progress events carry ids, counts,
  durations and error codes, never post text, names or anchors.
- Reading makes no network calls; models are local files. The one
  exception is paid embeddings, off by default (spec 0025).
- Censored words are masked before reading or storage.
- Entity records include people's names. They are stored only as
  positions and labels, purged with their posts, and shown only on
  admin pages.
- The parser is protected against tag bombs and slow patterns.
  Reading time per item is bounded per batch, never by truncation.
- Lab copies on Dell hold guest-visible content only and are purged or
  regenerated when their posts are deleted.

## Limits

- Memory: everything counts inside the service slice, within the memory limits
  in AGENTS.md section 5.6. Spec 0024's memory table holds the model weights.
  Reading adds batch working memory, the index's share and disk cache used while
  scanning, all measured inside the cap.
- The index's memory share is the scan buffer (64 MB by default;
  256 MB on a PC with 16 GB or more; spec 0023) plus row tables (about
  24 MB per 1.5 million vectors).
- Each model loads once, in one model host shared by both lanes; no
  worker loads its own copy.
- Batches are limited by tokens, not items, so a long post cannot spike
  memory.
- Reading every post in full streams; it never holds a whole thread.
  A post is read one piece at a time, and its vector is a running
  average of its pieces. A thread keeps one running sum of 8 KiB. A
  WordPress post over 16 MiB is held one part at a time, never whole,
  so it peaks no higher than XenForo's largest post.
- One message of up to 16 MiB is read as a stream (spec 0026): one
  copy of the message, freed after reading; a streaming parser capped
  at 4 MiB; text straight into the piece builder; pieces to the
  embedder queue one at a time. Worked peak for a 16 MiB message:
  about 26.3 MB; with one ordinary batch and the running sums beside
  it, about 33 MB. Only one message over 1 MiB is read at a time.
- So reading's peak does not grow with thread or post length. It stays
  in its row of spec 0024's memory table (48 MB under 16 GB, 128 MB at
  16 GB or more), with no change to the row, and the whole service
  stays under the 85% line of its slice (3,825 MB under 16 GB,
  5,950 MB at 16 GB or more).
- The add-on adds at most 8 MB per request; batches and slices carry at
  most 1 MiB of text each.
- At most 64 threads are being read at once across both lanes, each
  holding one running sum of 8 KiB, so the sums take at most 512 KiB.
  A thread waits for a free place before its first post is read.
- Pieces of the piece size (1,024 tokens by default, never fewer, with
  no fixed top); a batch at most 30 seconds of model
  work; at most one minute of repeated work per worker after a crash.
- Index size: ceiling `reading.index.budget_gb` (25 GB), warning at
  80%.
- Time, by arithmetic only: Qwen3-Embedding-0.6B has about 0.6 billion
  numbers, of which about a quarter are a near-free word-piece table;
  the rest needs roughly 0.9 billion operations per token. At 1.5 to
  3 × 10^11 operations a second on 4 cores, one full piece takes about
  3 to 6 seconds. A million posts of about 150 tokens come to about
  165 million tokens with titles: about 1.5 × 10^17 operations, or 6
  to 12 days of full use, stretched to one to several weeks by low
  priority and pauses. The entity model is expected to add much less;
  measured figures replace all of these.

## Footprint budget

This part adds nothing to public pages. The add-on's reading requests
stay under 8 MB of extra memory each, and the footprint harness
measures page time (95th percentile) and database load with the
backlog running and stopped.

## How it is tested

Test first: each behaviour starts with a test that fails for the right
reason.

- Unit tests need no download: a fixed fake embedder and fake
  tokenizer stand in for the models; a fake model with a small window
  tests refusal of over-long input. Integration tests repeat the
  splitter checks with the real pinned tokenizer.
- Property tests on generated posts: every readable character is in a
  piece; no piece exceeds the piece size; no overlap exceeds the
  overlap setting; no cut splits a word or combining sequence; skipped
  zones and extras never enter pieces; the same input always gives the
  same pieces.
- Fixtures: the synthetic fixture of about 300 threads gains nested
  and unclosed quotes, code holding BBCode-like text, spoilers,
  mentions, unknown BBCode, censored words, mixed and short-language
  posts, a 5 MB post, a 40 MiB WordPress post, a tag bomb, image-only
  and quote-only posts, links inside quotes, attachments, link
  previews, NFC and NFD text, four-byte emoji, CRLF line endings,
  zero-width characters, a question
  thread with an accepted answer and every reaction type, threads with
  known views and last post dates, a resource with updates and reviews,
  and a hub page. All text is made up.
- Integration tests on Dell use the real pinned models: vectors for
  fixture texts match recorded values with cosine similarity at least
  0.999.
- Contract tests check every add-on message, `ReadPart` included,
  against its Rust shape check.
- Recovery, memory (under the slice cap) and pause tests (synthetic
  pressure outside and inside Sutradhar's slice).
- Mutation tests cover the hard rules: guest-visible only, skipped
  zones, no truncation, never-anchor flags.
- Private-data runs use the 2,000- and 50,000-thread tiers on Dell
  only.
- Benchmarks under the median rule: throughput per stage (posts and
  tokens per hour) on the reference hardware and Dell; one full piece;
  the first-pass sweep and the thread-level index per thousand
  destinations; peak memory with disk cache during the backlog, a 5 MB
  post, a 40 MiB WordPress post, a 5,000-post thread and a summary
  rewrite; index size per million posts against the
  projection; the time from a pressure spike to pausing (target 5
  seconds); the live wait behind a backlog call.

Acceptance tests (Rust):

1. An item with `guest_visible` false is refused and logged by id and
   code only; private conversations, drafts, staff-only posts and
   signatures never reach the store.
2. Three-level nested quotes, a code block holding `[url]`, a spoiler
   and an inline spoiler leave no text in any piece; the text around
   them is read.
3. A 300-token post gives exactly one piece, starting with the title,
   at most 1,024 tokens by the embedder's tokenizer.
4. A 5,000-token post of 20 paragraphs gives pieces of at most 1,024
   tokens, each starting at a paragraph or sentence boundary, with
   overlaps of whole sentences of at most 64 tokens. Where the last
   sentence of a piece is over 64 tokens, the next piece's overlap is
   the last whole words that fit in 64 tokens.
5. A 3,000-token paragraph splits at sentences; a 2,000-token sentence
   with no punctuation splits at words; no piece is too long and no
   word is cut.
6. For any generated post, every readable character is in a piece.
7. A French post shows "skipped: not English" with no pieces, vectors
   or entities, yet its links and state are recorded. A French
   paragraph in a mostly English post is read, with one other-language
   paragraph recorded; a mostly French post with one English paragraph
   is skipped whole.
8. A 12-letter reply in an English thread takes the thread's language
   and is read.
9. Alt text, attachment names and stored preview titles become extras
   with weight 0.5, in no piece, flagged never-anchor; no network
   request is made for a title.
10. Editing one reply in a 2,000-post thread re-reads that reply alone;
    the thread's sum swaps the reply's old vector for its new one, and
    equals a sum made from scratch within rounding.
11. An edit to a quote only makes no embedder call and records the new
    content version.
12. A title edit re-reads the title, first post and accepted answer
    in the live lane
    and queues every other post in the backlog lane; the count shows.
13. In a question thread with 60 replies of up to 3,000 tokens each,
    the summary vector uses every read post in full: it equals the
    weighted sum, normalised, of the title, the first post, the
    accepted answer and the average of all 58 other posts. Removing any
    one post's text from the input changes the vector. Reactions do
    not change it. Each post's vector is the average of its pieces.
14. Changing a summary weight rewrites summary vectors in the
    background with progress and no model run; old vectors serve until
    replaced. Weights that put the title below another part, or the
    first post or accepted answer below the other posts, are refused.
15. With 10,000 backlog posts queued, a new post waits only for the one
    backlog call running (measured).
16. Pressure above 20% for 10 seconds from programs outside the slice
    pauses the backlog within 5 seconds, shown as "paused: computer
    busy"; it resumes after 60 seconds below 10%. The same pressure
    inside Sutradhar's slice does not pause it. The live lane works
    throughout.
17. Killing the service mid-backlog resumes from the last checkpoint,
    repeating at most one minute per worker. Piece boundaries match an
    uninterrupted run exactly; each vector has cosine similarity at
    least 0.9999 with its twin.
18. A duplicated save event and a delete arriving before an older save
    leave the post deleted from the index.
19. When a forum section turns private, every piece, vector, span and extra of
    its posts leaves the serving index within 60 seconds, and their
    summaries are rebuilt.
20. An erasure request purges the post's text, pieces, vectors, spans,
    extras, summary contributions and any Dell lab copies.
21. The serving generation records the embedder's key and tokenizer; a
    different active key or tokenizer is reported as a mismatch.
22. With all code-carrying loading disabled, every production model
    loads in its chosen data-only form; a fixture model holding a
    pickle file is refused at install and load.
23. The file check marks a container wrapping a pickle archive as not
    data-only.
24. A 150,000-character post is read in full, peak memory under the
    cap, with checkpoints between pieces.
25. During a backlog run on the 50,000-thread tier, peak memory stays
    under the slice cap; with injected memory pressure, batches shrink
    and the backlog refuses new work, with a reason, before the cap.
    Peak memory while reading a 5,000-post thread equals that of a
    5-post thread within the measuring tolerance.
26. Censored words are masked in stored text and pieces.
27. With Resource Manager, descriptions and updates are read with the
    resource title and reviews are not; without it, nothing about
    resources is read and no error is raised.
28. With the hub module, a hub's own text is read and its listed
    threads are links only; turning the module off removes hub pages
    from the index.
29. Progress shows totals per stage, both indexes included, skipped by
    reason, failed by code and an estimate from measured throughput;
    queue depth, rate and coverage show on the review and health pages.
30. Span offsets in text with four-byte emoji and NFD input match the
    add-on fixture's offsets in code points.
31. With potion unavailable, the sweep and backlog keep the same order
    and the first pass shows as degraded.
32. No `ReadBatch` exceeds its byte cap; the add-on's extra memory per
    request stays under 8 MB, measured with it on and off.
33. A 5 MB post arrives in slices of at most 1 MiB, each request under
    8 MB; the joined text matches its checksum and is read in full.
    An edit mid-fetch restarts the fetch for the new version.
34. An input one token over a fake model's window is refused with a
    code and reported; nothing is truncated.
35. Links inside quotes carry the inside-a-quote flag and do not count
    against the thread's allowance.
36. The projected index size lies within 10% of the measured size; a
    projection over the budget stops backlog writing with a plain
    message before it is exceeded.
37. After the first pass, every eligible destination's title and
    first summary vectors are written before any post other than a
    first post or accepted answer is read, live work apart. The
    thread-level index is complete only when all have both, and no
    suggestion comes before.
38. Threads with known views and last post dates are read in the
    views-and-recency order in both phases; changing the potion
    shortlist leaves the order unchanged.
39. Coverage counts destinations in the thread-level index and posts
    read out of the eligible totals, everywhere it shows; reading the
    last eligible post gives the completion signal exactly once.
40. A thread made during phase 1 gets its title and summary vectors in
    the live lane and is a destination at once.
41. The first generation serves once its thread-level index is
    complete. In a rebuild for a new embedder the new generation is not
    served before it is whole, no new suggestion is made, and the
    switch changes the serving generation id in one step.
42. Setting `reading.pieces.tokens` to 1,023, or to any value under
    1,024, is refused with the setting's key, and the old size stays.
    At 2,048 every piece of a long post is at most 2,048 tokens, every
    readable character is still in a piece, and the change starts a
    rebuild for new reading rules while the serving generation runs.
43. With the default overlap, each further piece starts with whole
    sentences of at most 64 tokens from the end of the one before. At
    128 the overlap is at most 128; at 0 there is none; a value over a
    quarter of the piece size is refused. A single sentence longer than
    the overlap gives an overlap of whole words.
44. A 40 MiB WordPress post is read in three parts of at most 16 MiB.
    Every readable character is in a piece, and the pieces equal those
    of the same post read whole by a test build with the limit raised.
    No part is refused, and peak memory is no higher than for one
    16 MiB XenForo post. An edit after the second part restarts the
    read for the new version; no half-read pieces stay.
45. A 2,000-post thread's summary vector uses every read post: a test
    that changes the last post's text alone changes the vector. It
    equals, within rounding, a sum made from scratch over all posts.
    The daily reconciliation's recomputed sum matches the running one.
46. In a 300-post thread, marking reply 40 as the accepted answer,
    changing it to reply 90, then unmarking it, each gives a sum equal
    within rounding to one made from scratch with that answer left out
    (or with none left out after unmarking). No embedder call is made
    and no post is fetched again.
47. A 40 MiB WordPress post with a wrong checksum on its second part
    stores none of that part's results, serves none of the post's new
    pieces, and starts again; the old version serves meanwhile. With
    every part right but a wrong whole-post checksum, the staged
    pieces are dropped and none is served. With all checksums right,
    the new pieces serve in one step.
48. A 70 KiB paragraph with no break gives pieces of at most 64 KiB
    of text, split at words; a single 70 KiB word is split at grapheme
    boundaries, counted and shown; every character is in a piece.
49. An extra of 5,000 characters is read in full, split like body text
    with the overlap setting; no character of it is dropped.
50. Reading a 16 MiB post, measured with the counting allocator in a
    benchmark program (AGENTS.md section 6 rule 8), peaks
    under the reading row (48 MB under 16 GB), at about 33 MB with an
    ordinary batch beside it; the message buffer is freed when reading
    ends.
51. Setting a piece size of 2,048 with no measured run, or with a run
    whose embedder peak passes 1,300 MB, is refused with a plain
    message and the old size stays; with a passing run it is allowed.
    A piece size over a fake embedder's window is refused with its
    plain message.

## Changes from the source

- The reading service is rewritten as Rust, on Sutradhar Boot, on the
  owner's PC.
- Models are pure-Rust models (candle or tract), including
  Qwen3-Embedding-0.6B in place of the earlier embedder, and our own
  entity model in place of a ready-made one.
- The sentence splitter and language identifier are Rust.
- Reading records live in PostgreSQL with row-level security per site;
  vector segment files stay in the service's own data folder.
- The site never calls the owner's PC: the service collects content
  events from the add-on's queue and fetches board settings.
- The backlog pause watches pressure on the owner's PC from programs
  outside Sutradhar's slice, since the engine no longer shares a host
  with the forum; site impact is held by the footprint budget.
- Paid embeddings come through AWS Bedrock with the owner's own key
  (spec 0025).
- Pieces are 1,024 tokens by default and never fewer, with an overlap
  setting of 64 tokens by default, in place of R101's fixed overlap of
  about 15%. The owner set this on 31 October 2026 (R168): "make 1024
  default ... should be 1024 or higher, and there must be zero
  truncation content bodies must be read in full".
- The summary vector reads every post in full, by a running sum, in
  place of 20 chosen posts cut to 400 characters, by the owner's answers
  of 31 October 2026: "No truncation anywhere" and R170 ("the thread
  vector is now a weighted aggregate of all its posts' chunk vectors").
  Q-RD05's answer, all reactions counted equally with Sad and Angry
  included, chose the "highest-reacted" posts of the old summary; with
  every post read, no post is chosen by reactions. P-RD11's half weight
  for extras is kept; its cut to 200 characters is dropped under the
  same "No truncation anywhere".
- A WordPress post over 16 MiB is read in full, in 16 MiB parts.
- Speed follows the 5% median rule; the 20x goal is kept (spec 0007).
- Memory follows the limits in AGENTS.md section 5.6.
- Message contracts are checked by Rust shape checks.

Source: `docs/specifications/linking/reading.md`, `linking/reading-2.md`,
`linking/reading-3.md`, `linking/reading-4.md`, `linking/reading-5.md`
and `linking/reading-6.md` in the archive `archive-2026-11-08-old-repo`.
