# SPEC-008 part 3: Reading and indexing

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in the
[specification decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Requirements: as in [part 1](reading.md). Part 2 is [links, re-reading,
summaries, the reading order and the backlog run](reading-2.md); part 4
is [model files, failure, privacy, performance and
testing](reading-4.md); part 5 is [acceptance cases](reading-5.md);
part 6 is [the owner's decisions](reading-6.md).

## Design, continued

### Data stored

All reading data, the vectors included, lives in the Python service's
own store (bhandar) in its data directory, with restricted
permissions, as the owner chose (Q-RD08); nothing of it is stored in
the search service. It is derived from guest-visible content and is
purged when that content is deleted, hidden or erased (R052).

Per item (post, resource description or update, hub page):

- ids, container, position, dates, content version and reading-rules
  version;
- state: visibility as reported, main language, and read (with the
  count of paragraphs in other languages, Q-RD02), skipped (with
  reason) or failed (with error code);
- existing links, with positions, an inside-a-quote flag and canonical
  targets;
- readable text with masks, and the F005 position-map version;
- positions of skipped zones and extras, for the admin "post X-ray"
  view (SPEC-012);
- pieces: offsets, token count, text, embedder vector, potion vector;
- extras: kind, cut text and vector, flagged never-anchor;
- sentence boundaries, allowed entities with labels, and noun phrases,
  with the entity pipeline's key.

Per destination (thread, resource, hub page):

- canonical id, title, title vector, language and eligibility;
- view count and last post date, for the reading order (R123);
- in the serving generation, whether its title and summary vectors are
  written (the thread-level index) and how many of its eligible posts
  the embedder has read;
- summary subset (post ids), cut-text vectors, summary vector, summary
  text and the summary settings version.

Per index generation:

- generation id, which is the index version each suggestion records
  (R122); the embedder's compatibility key as SPEC-010 defines it
  (entry id, repository, revision, index-compatibility version and
  vector size); the tokenizer's identity (its files and checksums);
  potion's key for the first-pass store; the reading-rules version;
  start and completion times, and the times its thread-level index and
  per-post index completed; item and vector counts; coverage (R123).

Vectors are kept in fixed-size segment files, with a table from row to
piece and a list of removed rows that background compaction clears.
They are stored at 32-bit precision (Q-RD07). SPEC-009 scans the
segments in blocks for exact search.

### Uninstall classes

SPEC-010 owns the table that classifies every store for uninstall under
R050 as amended. Reading's stores fall as follows:

- Always deleted, with no keep option and no export step, as embeddings
  or part of the index: every vector (pieces, extras, titles,
  summaries, cut texts and the first pass); segment files, row tables
  and removed-row lists; and index generation records.
- Keeping R050's export offer and keep-or-delete choice: the reading
  records the index is built from (per-item state and skip reasons,
  language, readable-text copies, piece, extras and summary text,
  entity and noun-phrase spans, existing-link records and summary
  subsets), reading checkpoints and the backlog cursor. Reading's
  settings follow SPEC-010's table for the add-on's settings.

### Index generations and the revision check

- A generation is the full set of vectors made by one embedder and
  tokenizer under one reading-rules version. Exactly one serves.
- A rebuild, for a new model revision, a new tokenizer or new reading
  rules, writes a new generation alongside the serving one and switches
  to it in one step only when it is complete; a half-built generation
  is never served (Q148 starred case). The update flow and rollback
  belong to SPEC-010.
- While a new embedder rebuilds the index, new suggestions pause, so
  the new model runs alone; suggestions already made stay approvable
  (SPEC-010's MD-04, R122). Drafter's reading, for the owner to
  confirm: R122 names a change of embedding model, so a rebuild for new
  reading rules alone keeps the serving generation and its suggestions
  running.
- The first generation is the one exception to waiting for completion,
  because there is nothing older to serve: it serves once its
  thread-level index is complete, with its coverage recorded, while its
  per-post index is built (R123; part 2).
- At start-up and whenever a model is loaded, the service compares the
  active embedder's key and tokenizer with the serving generation's,
  following SPEC-010's compatibility rule. A difference is reported to
  readiness; SPEC-009 then makes no new suggestions and readiness says
  "not ready for suggestions" until the rebuild completes (R106).
  Suggestions already made stay approvable (R122).
- Stored spaCy records whose key differs from the active pipeline's
  are stale: they count as missing evidence, are re-read in the backlog
  lane, and SPEC-009 withholds suggestions from those posts.

### The index size budget

SPEC-010's community-model check (MF-C09) and its install preflight use
this budget and projection.

- Projected size in bytes = V × D × B + V × R + T + F, where:
  - V is the number of stored vectors: pieces, extras inputs, titles,
    summaries and kept cut-text vectors;
  - D is the vector size (1,024 numbers for bge-m3);
  - B is the bytes per number: 4, since vectors are 32-bit (Q-RD07);
  - R is the row-table cost per vector, about 16 bytes;
  - T is the stored text and records, measured per piece (assumed 2 KB
    per piece until measured);
  - F is the first-pass store: its vectors times potion's vector size
    times 4 bytes.
- Before the first pass, V is estimated as 1.5 per post (P-RD09);
  afterwards the exact count is used.
- Example: a million posts give about 1.5 million vectors, about 6 GB
  of 32-bit vectors plus about 3 GB of text and records. A rebuild needs
  room for a second generation until the switch; SPEC-010's preflight
  checks free disk for that.
- `reading.index.budget_gb` (default 25, P-RD09) is a ceiling, not a
  reservation. Install and rebuild warn when the projection passes 80%
  of it; the backlog stops writing, with a plain message, before the
  index would exceed it, and the serving index stays usable.
- The index's share of memory is the scan buffer (256 MB by default,
  SPEC-009) plus the row tables (about 24 MB per 1.5 million vectors).
  SPEC-010's memory table (6 GB, or 8 GB once Dell measurements confirm
  the estimate, R121) counts this share.

### Interfaces between PHP and Python

Transport, signing, replay protection and message schemas belong to
F004 (R043, R077; Q148 starred security cases). Python never reads the
XenForo database; PHP supplies approved data (R043).

- `ReadBatch` (PHP to Python, from the backlog cursor or on request).
  For each item: kind, id, container id and title, the container's view
  count and last post date, node id, prefix id, position, first-post
  flag, solution flag, post and edit dates, stored message, reaction
  counts by type, attachment names, stored link-preview and media
  titles, censor positions, content version and `guest_visible`, which
  must be true. Each batch is capped by size
  (1 MB of message text by default) so PHP's extra memory stays under
  8 MB per request (R022).
- A message over the batch cap is sent in the batch as a header only:
  its length in code points, its content version and a checksum.
- `ReadPart` (Python to PHP) is the ranged fetch: it asks for one slice
  of one message (start and length in code points, at most 1 MB of
  text) for that content version. PHP cuts only between characters.
  Python joins the slices and checks the checksum; if the post changed
  meanwhile, it starts again with the new version (P-RD14).
- `ContentEvent` (PHP to Python): post saved, edited, soft-deleted,
  hard-deleted or restored; thread title edited, moved, merged or
  split; node or thread visibility changed; resource or hub page
  changed; erasure requested. Events carry ids and versions only, and
  Python fetches content as above (Q148: send ids, fetch on demand).
- `ResolveLinks` (Python to PHP): internal link targets to canonical
  ids, following redirects and merges.
- `ListingPage` (PHP to Python, for reconciliation): ids, last-edit
  times and visibility, by id cursor; for threads also view counts and
  last post dates, which refresh the reading order (R123).
- `ReadStatus` (Python to PHP): per item, read, skipped with reason, or
  failed with code, for the admin pages.
- `BoardSettings` (PHP to Python): board address form, friendly-URL
  setting and route prefixes for parsing links; sent at start-up and
  whenever they change.

Inside the service, reading offers SPEC-009 the serving generation's
metadata (its id included) and segments, piece texts, extras, spans,
existing links, summaries, destination eligibility, the coverage, and
a signal, given once, when the per-post index is complete (R122,
R123).

### Settings and defaults

Settings live in the settings system (F001) with versioned, atomic
changes; each batch records the settings version it used (R025).
Settings are tuned only within approved bounds (R024).

Fixed by the owner's decisions, not settings: pieces of at most 1,024
tokens; overlap of about 15%; the title prefixed to every piece;
quotes, code, spoilers and signatures skipped; English only, judged by
each post's main language; dense vectors only, stored at 32-bit;
full-precision bge-m3; the thread-level index first and the
views-and-recency order (R123).

Settings, with their defaults (accepted with the proposals):

- `reading.summary.top_reacted` (5), `reading.summary.first_replies`
  (5), `reading.summary.recent` (3), `reading.summary.total` (20) and
  `reading.summary.cut_chars` (400), all from ADR-0001;
- `reading.summary.title_weight` (2.0; P-RD05) and
  `reading.summary.rebuild_delay_minutes` (10; P-RD13);
- `reading.extras.item_chars` (200) and `reading.extras.weight` (0.5);
- `reading.language.min_letters` (20; P-RD10);
- `reading.index.budget_gb` (25; P-RD09);
- `reading.backlog.enabled` (true) and `reading.batch.max_seconds`
  (30);
- `reading.backlog.pause_pressure_percent` (20, over 10 seconds) and
  `reading.backlog.resume_pressure_percent` (10, over 60 seconds);
- `reading.php.batch_bytes` and `reading.php.part_bytes` (each
  1,048,576);
- `reading.reconcile.interval_hours` (24).

Changing a setting that alters vectors (the summary counts, cut length
or title weight) rebuilds the affected summaries in the background.
