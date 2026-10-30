# SPEC-008 part 4: Reading and indexing

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in the
[specification decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Requirements: as in [part 1](reading.md). Part 2 is [links, re-reading,
summaries, the reading order and the backlog run](reading-2.md); part
3 is [stored data, interfaces and settings](reading-3.md); part 5 is
[acceptance cases](reading-5.md); part 6 is
[the owner's decisions](reading-6.md).

## Model files and data-only formats

"Data-only" means a file that cannot run code when it is opened.
"Pickle" is a Python format that can, and R106 says pickle files are
never loaded.

What was found, checked on Dell from the Hugging Face listings on 30
October 2026 at the revisions then current:

- bge-m3 publishes its PyTorch weights only as a pickle file
  (`pytorch_model.bin`, 2.27 GB). It also publishes an official ONNX
  export at full precision. There is no safetensors file and no other
  branch.
- en_core_web_trf's transformer file is a spaCy container holding a
  PyTorch archive saved with pickle, so loading it means unpickling.
- en_core_web_lg, a different spaCy model without a transformer, has
  model files with no pickle in them.
- gte-reranker-modernbert-base and potion-retrieval-32M ship safetensors
  files.

So, under R106 as written, bge-m3 and en_core_web_trf cannot be loaded
in their published forms. SPEC-010 put the choices to the owner, who
decided (R120):

- bge-m3 is used through its publishers' official ONNX export at full
  precision, run by ONNX Runtime, never through its pickle weights
  (MD-01). The export joins the formats R106 lists.
- spaCy's en_core_web_lg replaces en_core_web_trf (MD-02). It runs
  without PyTorch.

Reading uses the embedder and the entity pipeline only through
SPEC-010's model interface, so the rest of its design does not change.
ONNX Runtime and en_core_web_lg still need admission records (R027)
before full reading and entity reading run on a forum host.

How files are checked, for every revision SPEC-010 pins:

1. Every file is listed with its size and SHA-256 checksum and
   classified by its content, not its name.
2. Containers are opened and checked inside too, because a spaCy file
   can wrap a PyTorch archive. Pickle data at any depth fails.
3. Only passing files are recorded in `models.lock`, and installation
   downloads only recorded files.
4. The loader checks each file's format again before opening it and
   never enables `trust_remote_code`.
5. A test disables all pickle loading and then loads every production
   model in its chosen form, as R106 requires: bge-m3's ONNX export
   through ONNX Runtime, en_core_web_lg, the reranker and potion
   (AT-RD22). The owner chose data-only forms (R120), so no pickle
   exception exists.

## Failure behaviour

Every failure is shown in plain English with a central error code and a
manual link (R051). Missing results are shown as waiting or failed,
never invented or treated as zero (R015, R019).

- Embedder missing or failed: full reading stops and items wait
  visibly; no suggestion can start (SPEC-009); readiness is "not
  ready".
- Entity pipeline missing or failed: vectors continue; items show
  "waiting for entity reading"; suggestions that need it are withheld.
- potion missing or failed: potion vectors and the shortlist are
  skipped and the first pass is shown as degraded; the rest of the
  sweep and the reading order are unchanged, since potion no longer
  orders reading (R123).
- Language identifier failed: the item waits and is retried. It is
  never assumed to be English.
- Input too long for a model: refused with a code and reported as a
  defect; never cut silently.
- One item fails extraction or reading: that item is marked failed with
  its code and retried with growing delays; the rest continue (R019).
- Ranged fetch interrupted or the post changed during it: the fetch
  starts again for the current version; partial text is never read.
- Memory nearing the cap: batches shrink, then the backlog refuses new
  work with a stated reason, before the hard cap is reached (R023). The
  live lane keeps its reserved share.
- Index size budget reached: the backlog stops writing, with a plain
  message; the serving index stays usable.
- Crash or forced stop: work resumes from the last checkpoint. Writes
  are idempotent (repeating one changes nothing), keyed by item id,
  content version and model key (R020).
- PHP unreachable: the backlog waits; events stay in PHP's
  database-backed retry queue (F004; Q148 starred case).
- Duplicate or out-of-order events: versions decide, and a deletion is
  never undone by an older save (Q148 starred cases).
- Disk full: writing stops, the serving generation stays usable for
  reading, and the fault is reported.
- Key mismatch: reported to readiness; SPEC-009 makes no new
  suggestions, and those already made stay approvable (R122).

## Privacy and security

- Only guest-visible content enters (R013). Python refuses, and logs by
  id and code only, any item whose `guest_visible` flag is not true.
- Visibility loss, deletion and erasure remove content from serving at
  once and purge stored text, vectors, spans and summary contributions
  (R052). SPEC-009 also filters search results by current visibility.
- Vectors are treated as content: text can partly be recovered from
  them, so they follow the same deletion, erasure and lab-purge rules
  as the text itself. At uninstall they are always deleted, with no
  export step (R050 as amended).
- Logs, metrics, traces and progress events carry ids, counts,
  durations and error codes, never post text, names or anchors; a test
  enforces this for metrics and spans (R127,
  [ADR-0006](../../adr/0006-observability.md)).
- Reading makes no network calls; the models are local files
  (SPEC-010). The one future exception is a paid embedder that an
  administrator chooses (R126, off by default), which would send
  guest-visible text to its provider; the separate paid-embeddings
  specification states its rules (part 1).
- Censored words are masked before reading or storage.
- Entity records include people's names (R104). They are stored only
  as positions and labels within posts, purged with their posts, and
  shown only on admin pages (R111).
- The parser is protected against tag bombs and slow patterns (F005;
  Q148). Reading time per item is bounded per batch, never by
  truncating text.
- Lab copies on Dell hold guest-visible content only and are purged or
  regenerated when their posts are deleted (Q148 starred case).

## Performance and resource budgets

### Memory

- SPEC-010 owns the whole memory table, including the models' weights,
  for the service's cap: 6 GB, or 8 GB once Dell measurements confirm
  the estimate that everyday use does not fit 6 GB (R021 as amended,
  R121). Reading adds working memory for its batches, the index's
  memory share (part 3) and disk cache used while vectors are scanned;
  all three are measured and counted inside the cap.
- Each model is loaded once, in one model-host process shared by both
  lanes. No worker loads its own copy.
- Batches are limited by token count, not item count, so one long post
  cannot cause a memory spike.
- PHP adds at most 8 MB per request (R022), helped by the ranged fetch.
  The search service has its own separate allowance (R021, R022).

### Time

Rough arithmetic, to be replaced by measurement:

- About 256 million of bge-m3's 568 million numbers are a lookup table
  of word pieces, which costs almost nothing to use. The rest needs
  about 0.7 billion operations per token. Running the ONNX export at
  full precision through ONNX Runtime (R120) does not change this count.
- The reference hardware (4 processor cores and a SATA solid-state
  disk, R124) is assumed to manage about 1.5 to 3 × 10^11 operations a
  second: the upper figure was first assumed for 8 cores, and 4 cores
  may manage about half. One full 1,024-token piece then takes about
  2.4 to 5 seconds. SPEC-009 uses the same range.
- A million posts of about 150 tokens come to about 165 million tokens
  with titles: about 1.2 × 10^17 operations, or 4 to 10 days of
  uninterrupted full use. en_core_web_lg has no transformer, so spaCy
  is expected to add much less than the quarter en_core_web_trf would
  have added; this is to be measured.
- Lowest priority and pauses stretch this, so the first run is
  estimated at one to several weeks per million posts on a busy forum;
  the owner accepted this (Q-RD01). The thread-level index, which
  suggestions wait for, is estimated in R123 at about a day or two for
  50,000 threads (part 2), longer at the lower processor rate.
  Measured figures replace these.

### The 20× gate (SPEC-007)

- Every changed executable unit of reading code is measured by the
  gate. New code is compared with a reviewed simple reference, such as
  reading posts one at a time without batching.
- Model inference dominates. The owner fixed full-precision bge-m3, so
  shrinking or swapping the Verified models is not an option. For
  inference, result B (a reviewed limit report naming the measured
  bottleneck, with the batching and threading experiments tried) is the
  expected honest outcome.
- Units outside inference (extraction glue, splitting, language checks,
  subset selection, queues) aim for result A, a measured 20× speed-up.

### What must be measured

- Throughput per stage, in posts and tokens per hour, on the reference
  hardware and on Dell, and the time for one full piece.
- Time for the first-pass sweep and for the thread-level index, per
  thousand destinations, so the wait before suggestions start is
  forecast from measurement (R123).
- Peak service memory, including disk cache, during the backlog, a
  5 MB post and a summary rebuild.
- Index size per million posts, against the projection in part 3.
- Forum impact: PHP page time (95th percentile) and database load with
  the backlog running and stopped.
- Time from a pressure spike in the forum's services to pausing
  (target 5 seconds); work repeated after a forced stop (at most one
  minute per worker); live-lane wait behind a backlog call.

## Testing

- Test-first (R087): each behaviour starts with a test that fails for
  the intended reason, then the smallest change that passes it, then
  refactoring with the tests passing.
- Unit tests need no download: a deterministic fake embedder and a
  fake tokenizer stand in for the models (ADR-0002). A fake model with
  a small window tests the refusal of over-long input. Integration
  tests repeat the splitter checks with the real pinned tokenizer.
- Property tests generate posts and check that every readable character
  is in a piece; no piece exceeds 1,024 tokens; overlaps stay in the
  accepted band; no cut splits a word or a combining sequence; skipped
  zones and extras never enter pieces; the same input always gives the
  same pieces.
- Fixtures: the committed synthetic fixture of about 300 threads (R058)
  gains nested and unclosed quotes, code holding text that looks like
  BBCode, spoilers, mentions, unknown BBCode, censored words, mixed and
  short-language posts, a 5 MB post, a tag bomb, image-only and
  quote-only posts, links inside quotes, attachments, link previews,
  NFC and NFD text, four-byte emoji, CRLF line endings, zero-width
  characters, a question thread with an accepted answer and reactions
  of every type, threads with known view counts and last post dates, a
  resource with updates and reviews, and a hub page. All text is
  synthetic.
- Integration tests on Dell use the real pinned models in the forms
  the owner chose (R120): bge-m3's official ONNX export through ONNX
  Runtime, and en_core_web_lg. Vectors for fixture texts match recorded
  expectations within a stated tolerance (cosine similarity at least
  0.999), because results differ slightly across hardware (Q148).
- Order tests, with the fake embedder, check that every destination's
  title and summary vectors are written before any other backlog post
  is read, that threads follow the views-and-recency order, and that
  coverage and the completion signal are right (R123; AT-RD37 to
  AT-RD40).
- Contract tests check every PHP–Python message, including `ReadPart`,
  against its JSON Schema (R077).
- Recovery tests kill the service mid-batch; memory tests run under
  the service's cap (6 GB, or 8 GB once Dell measurements confirm the
  estimate, R121); pause tests apply synthetic pressure to the forum's
  services and to Sutradhar's own.
- Mutation tests cover the hard rules: guest-visible only, skipped
  zones, no truncation, never-anchor flags (R077).
- Private-data runs use the 2,000- and 50,000-thread tiers on Dell only
  (R058).
