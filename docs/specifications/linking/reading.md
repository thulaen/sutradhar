# SPEC-008: Reading and indexing

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in the
[specification decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Requirements: R098, R101, R102, R107 and ADR-0001; also R008, R011,
R013, R014, R019–R023, R028, R050, R052, R053 and R106 where reading
touches them; and R120–R124, R126 and R127 from the owner's answers.
Features served: F004 (guest-visible ingestion), F003 (backlog work and
recovery), F005 (text extraction, which this spec uses) and F017 (hub
pages as destinations).

Parts: this file (scope and how text is read); [part 2](reading-2.md)
(links, re-reading, summaries, the first pass, the reading order and
the backlog run);
[part 3](reading-3.md) (stored data, uninstall classes, the index size
budget, interfaces and settings); [part 4](reading-4.md) (model files,
failure, privacy, performance and testing); [part 5](reading-5.md)
(acceptance cases); [part 6](reading-6.md) (the owner's decisions,
proposals and recorder's notes).
Related: [SPEC-009 matching and anchors](matching.md); SPEC-010 (model
files, community models, retraining, disabling and uninstalling, and
the memory table: 6 GB, or 8 GB once Dell measurements confirm the
estimate, R121); SPEC-011 (progress and the live stream); SPEC-012
(visual layer).
Sources:
[models update](../../requirements/updates-2026-10-30/models.md),
[decisions update](../../requirements/updates-2026-10-30/specifications.md)
(R120–R129), [ADR-0001](../../adr/0001-thread-reading.md),
[ADR-0002](../../adr/0002-model-files.md) and
[ADR-0006](../../adr/0006-observability.md).

## Words used here

- Token: the unit a model counts text in; roughly four tokens for every
  three English words. Each model has its own way of counting.
- Piece: a stretch of one post's text, at most 1,024 tokens, that a
  model reads in one go (R101; earlier called a chunk).
- Vector (embedding): a list of numbers a model writes for a text;
  texts with similar meaning get similar vectors.
- Embedder: the model that writes the vectors. It is bge-m3, used
  through its publishers' official ONNX export at full precision and
  run by ONNX Runtime (R120), unless an administrator activates a
  community model (SPEC-010).
- Index: the stored vectors plus the records saying which post, thread,
  resource or hub page each one belongs to.
- Thread-level index: each destination's title and summary vectors,
  written by the embedder and built first, so that suggestions can
  start (R123). Per-post index: the vectors of every post's pieces and
  extras, built after it in the background.
- Coverage: how much of the index is written: destinations in the
  thread-level index and posts read, each out of the eligible total
  (R123).
- potion (potion-retrieval-32M): a much faster, simpler model for a
  first sweep. en_core_web_lg: spaCy's English model, which finds names
  (entities), sentences and noun phrases (R098). It replaced
  en_core_web_trf, whose transformer weights are a pickle (R120).
- Lane: a queue of reading work with its own priority.
- Checkpoint: a saved record of finished work, so a restart carries on
  from there instead of starting again.
- Guest-visible: content a logged-out visitor may see; the only content
  that may be read (R013).
- NFC: a standard way of writing accented letters, so the same word is
  always stored the same way (for example "é" as one character, not
  "e" followed by a separate accent mark).
- Code point: one character as the Unicode standard numbers it.
  Counting positions in code points makes an emoji count as one
  character although it takes four bytes, so PHP and Python agree on
  where a word starts.
- Ranged fetch: asking for a very long post a slice at a time (for
  example its first megabyte, then the next), so that no single request
  has to carry all of it.

## Scope

In scope:

- Which content is read: guest-visible posts in threads, resource
  descriptions and updates when XenForo Resource Manager is present,
  and approved hub pages when that optional module is enabled (R013,
  R014).
- Turning each item's stored text into readable text, skipped zones and
  short extras, and recording its links and state (R101).
- The English-only language check, judged by each post's main
  language, with skipped posts shown (R098; Q-RD02).
- Splitting into pieces with the title prefixed and about 15% overlap.
- Embedder vectors for every piece, extras record, title and thread
  summary (R098, R102).
- spaCy's sentences, entities and noun phrases, stored for SPEC-009.
- The potion first pass, the thread-level index first and the per-post
  index after it, the views-and-recency reading order and the index
  coverage (R123).
- The first backlog run and incremental re-reading (R101, R107).
- Index generations, the keys they record (the input to R106's
  blocking rule), the index size budget and its projection.
- How the production model files are checked for data-only formats.

Out of scope, owned elsewhere:

- Search, reranking, anchors and blocking suggestions: SPEC-009.
- Installing, verifying, updating and removing models, `models.lock`,
  community models, retraining, the uninstall table for every store and
  the whole memory table (6 GB, or 8 GB once Dell measurements confirm
  the estimate, R121): SPEC-010.
- Paid embeddings (R126): an administrator may choose a paid embedding
  model from Google or OpenAI instead of bge-m3 for the whole index,
  with their own key and a spending cap, sending guest-visible text
  only; off by default. A separate specification, drafted next, covers
  providers, consent, privacy, costs and keys. Reading keeps one
  embedder per index generation, so choosing one, or switching back,
  is a full rebuild (part 3).
- The progress handle, its views and the live stream: SPEC-011.
  Visuals: SPEC-012.
- The BBCode parser and position map (F005), the signed PHP–Python
  transport (F004), the job framework (F003) and settings storage
  (F001). This spec states only what it needs from them.
- The review page, insertion and undo (F009).

## Design

### Pipeline

Each item passes through these stages. Each stage commits its result
before the next starts, so a crash repeats at most the unfinished stage.

1. Receive: PHP sends guest-visible items, from an event (new or
   edited) or from the backlog cursor; a very long post arrives by
   ranged fetch (part 3).
2. Extract: the F005 parser turns stored BBCode into typed segments.
3. Record: existing links and state, for every guest-visible post,
   whether or not it is read further (part 2).
4. Language: a post whose main language is English goes on in full;
   any other post is skipped and shown (Q-RD02).
5. Split: pieces with the title prefix and overlap; extras gathered.
6. First pass, during a backlog sweep only: potion vectors (cheap;
   never a suggestion).
7. Full reading: embedder vectors for pieces, extras and the title.
8. Entities: spaCy sentences, entities and noun phrases.
9. Summary: the thread's summary vector and summary text are rebuilt
   when the item changes the selected subset.

Across items, the embedder works in two phases after the first pass:
the thread-level index first, then the per-post index, with new and
edited posts always first (part 2; R123).

### What is read

- Thread posts: every guest-visible post, first post and replies, with
  no size limit and no staff warning (R101).
- Resources, only when Resource Manager is present: the description,
  read like a first post, and each guest-visible update, read like a
  reply; pieces carry the resource title. Reviews are not read: they
  are ratings, and R011 counts a resource "including updates" only
  (Q-RD03). A resource's discussion thread is read as a normal thread.
- Hub pages, only when the hub module is enabled (R039): a hub's own
  written text is read; the threads it lists are recorded as links, not
  read, so a hub does not seem to match every topic. Hubs only receive
  links, because their own links are curated under R039 (Q-RD04).
- Never read: private conversations, drafts, previews, staff-only or
  members-only content, profile posts, signatures and anything PHP does
  not mark guest-visible (R013; Q148 starred cases).

### Extraction and skipped zones

The F005 parser works from the stored message, never the rendered page
(Q148). For reading it must return segments of these kinds:

- Skipped: quotes at any depth, including their attribution line; code
  (`[CODE]`, `[ICODE]`, `[PHP]`, `[HTML]`); spoilers (`[SPOILER]`,
  `[ISPOILER]`). Skipped text is in no piece and is never an anchor.
  Signatures are not part of the stored message and are never sent.
- Body: ordinary text, formatting, lists, headings, tables, `[PLAIN]`
  content, the visible text of existing links, mentions, and the
  content of unknown add-on BBCode. Mentions and unknown BBCode content
  are read but marked never-anchor (Q148).
- Extras: image descriptions (alt text), titles of link previews that
  XenForo has already stored, attachment file names, and embedded-media
  titles that XenForo has already stored. Nothing is fetched from the
  internet to find a title.
- Masked: words XenForo's censor would hide. PHP sends their positions;
  they are replaced by a neutral mask in the readable text, so they are
  neither read nor stored in clear (Q148 starred case).

Readable text is in NFC form, as XenForo stores it, and every position
is counted in code points, so PHP and Python agree (Q148 starred
cases). F005 supplies a map from readable text back to positions in the
stored message.

### Language (English only)

- An admitted language identifier (candidate: lingua-language-detector,
  subject to admission under R027) checks each paragraph.
- A post is judged by its main language, the language that holds most
  of its letters (owner's answer to Q-RD02). A post whose main language
  is English is read in full, any paragraphs in other languages
  included; how many such paragraphs it has is recorded with its state.
  Any other post is skipped and shown as "skipped: not English" with the
  detected language (R098).
- A paragraph with fewer than 20 letters takes the language of the rest
  of its post, or of its thread if the post has nothing else (P-RD10),
  because detection on very short text is unreliable (Q148). Since
  posts are judged whole, this matters mainly for very short posts,
  which take their thread's language.
- A skipped post has no pieces, vectors or entity records and is never
  an anchor. Its links and state are still recorded (R101).
- Skipped counts appear in progress (SPEC-011) and per thread on the
  admin pages (SPEC-012). An edit runs the check again.
- A thread's language comes from its title and first post. A thread
  that is not English is not offered as a destination; its English
  replies are still read as sources (P-RD04).

### Pieces

- Tokens are counted with the active embedder's own tokenizer, including
  its start and end markers. The tokenizer's identity (its files and
  checksums, from `models.lock` or the site's overlay) is part of the
  generation key (part 3), so pieces cut for one model are never read
  by another.
- The title counts inside the 1,024 tokens (P-RD01): a piece is the
  thread title, a blank line, then body text, all within 1,024 tokens.
- A paragraph is text between line breaks, or between block tags such
  as list items, headings and table rows.
- Whole paragraphs are packed into a piece until the next one would not
  fit. A post that fits in one piece has exactly one piece.
- Overlap (P-RD03): each further piece starts with the last whole
  sentences of the previous piece, added from the end until they reach
  12.5% to 20% of 1,024 tokens (about 15%, or 154 tokens). If no run of
  whole sentences lands in that range, the overlap is cut at a word
  boundary at 154 tokens.
- A paragraph longer than the body budget is split at sentence
  boundaries, found by spaCy's rule-based sentence splitter (no model
  needed); a single sentence longer than the budget is split at word
  boundaries, with the same overlap (P-RD02).
- No cut falls inside a word, a combining character sequence or an
  emoji sequence. Every readable character is in at least one piece:
  nothing is truncated, however long the post (R101).
- Before every model call, the input is counted again with that
  model's own tokenizer. An input over the model's window, or a piece
  over 1,024 tokens, is refused with an error code and reported as a
  defect. Some libraries cut over-long input without saying so; this
  check runs first, so nothing is cut silently.

### Extras

- Each extra is cut to its first 200 characters at a word boundary
  (P-RD11). A post's extras are joined and read by the embedder as their
  own title-prefixed input within 1,024 tokens; further extras form
  another input under the same rule.
- Extras vectors are stored apart from pieces, carry a low weight (0.5
  by default, applied in SPEC-009 matching) and are flagged
  never-anchor (R101).

### Entities, sentences and noun phrases

- en_core_web_lg (R120) runs on the body text of each post that is
  read, paragraph by paragraph in small batches, so memory stays
  bounded for very long posts. Extras are not parsed.
- Stored for SPEC-009: sentence boundaries, named entities of the types
  SPEC-009 allows with their labels, and noun phrases. Destination
  titles are parsed the same way, for title phrases.
- Not stored: other entity types, the full grammar tree and spaCy's
  word vectors.
- If the entity pipeline is unavailable, vectors are still written; the
  post is shown as "waiting for entity reading" and SPEC-009 withholds
  suggestions that need it (R019).
