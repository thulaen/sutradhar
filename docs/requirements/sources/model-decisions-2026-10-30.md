# Owner decisions on models and full reading, 30 October 2026

Status: historical source record. The owner's answers are kept as given;
option labels are the option the owner chose, and free text is quoted
exactly inside the code blocks. The current requirements are in the
[models update](../updates-2026-10-30/models.md).

## Before the 28 questions

- Allow models trained on MS MARCO (research-only data terms): "Allow them".
- Model set, free text:

```text
arctic-embed-s, jina-reranker-v1-tiny-en and potion-base-8M and full spaCy’s English model.
```

- Entity spotting (spaCy small English model) in the first release:
  "In first release".
- Request during the questions:

```text
use the askuserquestiontool so that we settle this model thing. also want threads to be read in chunks but in full so that there won't be truncations or short summaries. i want things done in full so ask me 28 questions
```

## Round 1

- spaCy model: the transformer pipeline (`en_core_web_trf`), with:

```text
we change the 2 gb limit to 4 gig gb limit, and now that we have approved the change can we now use better models, that the earlier i selected
```

- Challengers in the comparison test: "Yes, as challengers" (option
  named all-MiniLM-L6-v2, bge-small-en-v1.5 and the ms-marco-MiniLM
  reranker).
- Languages: "English only, flag others" (option: posts detected as another
  language are skipped and shown as skipped, never silently mixed in).
- Runtime: "PyTorch (sentence-transformers)".

## Round 2

- Embedding model: "bge-m3".
- Reranker: "gte-reranker-modernbert-base".
- First pass: "potion-retrieval-32M" (question: the very fast first-pass
  model to sweep the backlog of existing posts).
- Memory: "Service 4 GB, lab stays 8 GB" (option: the Python service and
  all its workers share an operating-system cap of 4 GB on a forum host;
  the lab keeps its 8 GB combined ceiling).

## Round 3

- Fitting bge-m3: "Full precision, raise the limit".
- Indexing: "bge-m3 for everything".
- Piece size: "Up to 1,024 tokens" (option: most posts fit whole; long
  ones split at paragraph boundaries).
- Overlap, free text:

```text
about 15%, also when removing the add-on it should also remove the python stuff, and stop the embeddings and the rest of the python service stuff, disabling the addon should just stop the python service stuff but not remove the components from the server.
```

## Round 4

- Exact limit: "6 GB".
- Thread view: "Every piece plus a thread summary vector".
- Long threads, free text:

```text
no cap on reading, a cap on summarising. Every post in every thread is read for existing links, language, state, and per-post index entries, incrementally (only new and edited posts are reread), with no size limit and no staff warning. The thread-level topic representation uses a selected subset, about 20 posts: the title (weighted), the first post in full up to the model limit, the accepted answer if any, the 5 highest-reacted replies, the first 5 replies, and the 3 most recent posts, each truncated to its first 400 characters, excluding quotes, code, and signatures. Make the counts and the truncation length settings with these defaults, and record the rule as an ADR.
```

- Quotes: "Skip quotes".

## Round 5

- Code blocks, spoilers and signatures: "Skip all three".
- Non-text parts (image descriptions, link titles, attachment names and
  embedded media titles): "Yes, read them as short extras" (option: read
  with low weight, never used as link anchors).
- Titles: "Prefix the thread title to each piece".
- Reranker depth: "Top 50".

## Round 6

- Reranker input: "Source paragraph + destination's best pieces" (option:
  the source sentence with its paragraph and the thread title, against the
  destination's title, summary text and its 3 best-matching pieces, within
  8,192 tokens).
- Anchor sources: named entities from spaCy, keyphrases, destination title
  phrases, noun phrases.
- Entity types: products and brands, organisations, places and events,
  people.
- Keyphrases: "Both, merged" (option: statistical candidates re-scored by
  bge-m3).

## Round 7

- Learning: "Also retrain models in the lab" (option: periodically
  fine-tune bge-m3 or the reranker on Dell from decisions on guest-visible
  content; each new version needs a comparison test and owner approval).
- Search: "Exact search".
- Backlog run: "Background, low priority, days are fine" (option: runs
  with the lowest CPU priority and pauses under forum load; new and edited
  posts are always handled first; progress and checkpoints visible).
- Model files, free text:

```text
a combination. A single file in the repo, models.lock, is the only source of truth for every model: name, purpose, official source repository, exact revision, file list with SHA-256 per file, size, licence, and the index-compatibility version. Setup, tests, the nightly, and the health page all read it.

Install: during add-on setup, a "Install models" step shows each model with its purpose, size, and licence, and one click downloads them from the official source at the pinned revision, verifies every file against the SHA-256 in models.lock, stores them in the service's data directory with restricted permissions, and refuses any file that fails verification. Loading never executes code from the model repository (safetensors only, trust_remote_code false). The same step is available headless as `sutra models install`, and for servers without internet access there is `sutra models import <bundle>` plus a file-upload option in the wizard, verified the same way. Models are never auto-updated: when a release pins a new revision, the health page shows "update available" with a button; updating downloads the new revision alongside the old one, rebuilds the index with it, and only then switches over, so the forum never runs a model whose index doesn't match.

Testing: tests use the same models.lock. Unit tests use a deterministic fake embedder and need no download; integration tests, the eval, and the nightly use the real pinned models from the local cache, and CI caches the model files keyed by revision. A test asserts that the cached files match the checksums in models.lock and that the index metadata records the model revision it was built with.

Health page: one row per model with status (not installed, downloading with progress, verifying, loaded, degraded, failed, update available, index mismatch), revision, checksum verified time, load time, memory in use, device, inference latency p50 and p95 over the last hour, error count over the last hour, and the last error message. A "Run self-test" button sends a fixed input through each model and compares the result with the expected output recorded in models.lock. Readiness reports not ready while a required model is missing or failed, degraded (still ready, with a warning) when latency or error rate exceeds its budget, and a warning when the index was built with a different revision. Record this as an ADR.
```

## Follow-up round 1

- Weight formats: "Data-only formats, no pickle" (option: safetensors for
  Hugging Face models plus spaCy's own binary pipeline format, loaded from
  a directory; never pickle files).
- Retraining, free text (the community model request that followed is in
  the [platform requests](platform-requests-2026-10-30.md)):

```text
raise the limit to 9 gb
```

- Uninstall: "Always delete all of them" (embeddings, the search index and
  downloaded model files).
- Index revision mismatch: "block suggestions until rebuilt" (the progress
  and visuals requests that followed are in the platform requests).

## Follow-up round 2

- bge-m3 outputs: "Dense vectors only".
- Summary size: "Top up to 20 with next most-reacted" (option: after the
  listed posts, duplicates removed, add the next most-reacted replies until
  about 20; all counts stay settings).
- 400-character cut: "Every reply; first post in full" (option: the first
  post is read in full up to the model limit; every other selected post is
  cut to 400 characters without quotes, code, spoilers and signatures).
- First pass: "Early shortlist; suggest only after bge-m3" (option: it
  sweeps the backlog to decide what bge-m3 reads first and to give early
  candidates, but no suggestion reaches moderators until bge-m3 has read
  both source and destination).

## Re-check round 1

- "raise the limit to 9 gb" covers: "Whole lab ceiling 9 GB".
- Community models and named-model approval: "Owner approves defaults;
  admins choose per site" (option: R027 and R100 govern the models
  Sutradhar ships in models.lock; each forum's administrator may still pick
  a community model for their own site under R109).
- fastembed: "Yes, ONNX via fastembed too" (option: adds ONNX files as an
  approved data-only format and fastembed as a second runtime for community
  models).
- Short input windows: "Refuse below the piece size" (option: embedding
  models must accept at least 1,024 tokens and rerankers at least 8,192).

## Re-check round 2

- Visual data sources, free text:

```text
any real system data, plus an error log that is deduped, plus a live feed that is deduped.
```

- Embedding explorer: "One point per thread".
- Readiness while suggestions are blocked: "Not ready for suggestions".
- Export before uninstall deletes: "No export, just delete".
