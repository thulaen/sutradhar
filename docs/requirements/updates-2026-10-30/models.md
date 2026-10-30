# Requirements update: semantic models and full reading

Owner: project owner. Recorded by the Claude coordinator from the owner's
answers on 30 October 2026 (3 before the 28 questions, the 28, and four
follow-up rounds of four), after a check of each candidate model's own published
card, paper or repository. The owner's exact words are in the
[model decisions source](../sources/model-decisions-2026-10-30.md).
Status: owner requirement decisions. They approve requirements, not the
linking feature specification that must implement them, which still needs
drafting and approval. An independent fidelity review compared these
records with the answers; its findings are reflected here.

Rules recorded as architecture decision records:
[ADR-0001 thread reading](../../adr/0001-thread-reading.md) and
[ADR-0002 model files](../../adr/0002-model-files.md). Community models,
the progress system and the visual layer are in the
[platform update](platform.md).

## Changed requirements

### R021 and R092 (amended): memory

- The Python service and all workers belonging to it share an
  operating-system cap of 6 GB on a forum host, replacing 2 GB in R021 and
  the "keeps its 2 GB limit" statement in R092. The owner's path was 2 GB,
  then 4 GB, then 6 GB.
- The lab profile's combined ceiling becomes 9 GB, replacing 8 GB, for
  all lab work including model retraining (R105).

### R050 (amended for model data): uninstall

Uninstalling the add-on always deletes its embeddings, its search index and
its downloaded model files, with no keep option and no export step.
R050's export offer and explicit keep-or-delete choice still apply to the
add-on's other owned data, and published links remain (R017).

## New requirements

### R098: production models

- Embedding: `BAAI/bge-m3` at full precision, dense vectors only, used to
  read and index every eligible post.
- Reranker: `Alibaba-NLP/gte-reranker-modernbert-base`.
- First pass: `minishlab/potion-retrieval-32M` sweeps the backlog of
  existing posts to give an early shortlist and to decide what bge-m3
  reads first. No suggestion reaches moderators until bge-m3 has read both
  the source and the destination.
- Entities and parsing: spaCy `en_core_web_trf`, included in the first
  release.
- Runtime: PyTorch with sentence-transformers (and spaCy).
- First release is English only: posts in other languages are skipped and
  shown as skipped.
- Eligible means guest-visible (R013) throughout these requirements.

### R099: MS MARCO-trained models allowed

The owner allows models whose training used MS MARCO, whose terms say the
data is for non-commercial research purposes only.

### R100: comparison test with challengers

Before release the chosen set is tested on the forum's own data against
the challengers the owner named: `all-MiniLM-L6-v2`, `bge-small-en-v1.5`
and the ms-marco-MiniLM reranker. A challenger replaces a chosen model only
if it clearly wins on the forum's data, and a named replacement still needs
the owner's approval (R027).

### R101: reading every post in full

- Every eligible (guest-visible) post in every thread is read for existing
  links,
  language, state and its per-post index entries, with no size limit and no
  staff warning. Reading is incremental: only new and edited posts are read
  again.
- Per-post pieces hold up to 1,024 tokens, split at paragraph boundaries,
  with about 15% overlap; the thread title is prefixed to each piece.
- Every piece plus one thread summary vector is kept (R102).
- Quotes, code blocks, spoilers and signatures are skipped in per-post
  reading. Image descriptions (alt text), link titles, attachment names and
  embedded-media titles are read as short low-weight extras and are never
  link anchors.

### R102: one thread summary vector

Each thread also has one summary vector built from a selected subset of
posts, with defaults that are settings, as recorded in ADR-0001.

### R103: matching

- Nearest-neighbour search over all pieces is exact.
- The reranker scores the top 50 candidates. For each pair it reads the
  source sentence with its paragraph and the thread title, against the
  destination's title, summary text and its 3 best-matching pieces, within
  its 8,192-token window.

### R104: anchors

- Candidate anchor phrases come from four sources: spaCy named entities,
  keyphrases, destination title phrases and noun phrases. R010 is
  unchanged: only existing source words are linked.
- Entity types allowed: products and brands, organisations, places and
  events, and people. R009 (moderator approval) and R013 (guest-visible
  only) apply.
- Keyphrases: both methods, merged (statistical candidates and bge-m3
  scoring).

### R105: learning and retraining

Moderators' approve, reject and remove decisions (R018) also feed periodic
retraining of bge-m3 or the reranker in the Dell lab, from decisions on
guest-visible content, within the lab's 9 GB ceiling. Each new
model version needs a comparison test and the owner's approval before use.

### R106: model files

`models.lock` in the repository is the only source of truth for every
model Sutradhar ships (the Verified tier); a forum's own community models
are pinned in its per-site overlay on it (R109, ADR-0003), following the
owner's answer that administrators choose community models per site.
Models are installed, verified, loaded, updated and monitored as recorded
in ADR-0002. Only data-only weight formats are loaded: safetensors,
spaCy's own pipeline format loaded from a directory, and ONNX for
community models run through fastembed (R109). Pickle files are never
loaded, and code from a model repository is never executed. When the
index was built with a different model revision, suggestions are blocked
and readiness reports not ready for suggestions until the index is
rebuilt.

### R107: the first backlog run

The first run over existing posts works in the background at the lowest
priority, pauses under forum load and may take days. New and edited posts
go first; progress and checkpoints stay visible (R008, R020).

### R108: disabling and uninstalling

Disabling the add-on stops the Python service (embeddings, workers and the
rest of the service) and keeps its components on the server. Uninstalling
stops the service and removes its installed components, embeddings, index
and model files, without an export step (R050 as amended above).

## Points the feature specification must resolve

These are the recorder's notes, not owner decisions.

- bge-m3 at full precision is the slowest choice on a CPU: the backlog run
  and the 30-second first suggestion (R015) must be measured, and the
  specification must show how both are met (for example, whether reranking
  continues after the first suggestion appears).
- The rule for "clearly wins" in R100 is fixed before any scoring.
- The R100 challengers read fewer tokens at once (512 or less) than the
  1,024-token pieces; the specification settles how they are compared.
- Whether the earlier set (arctic-embed-s, jina-reranker-v1-tiny-en,
  potion-base-8M) also joins the comparison as challengers.
- Privacy safeguards for person-name anchors.
- Retraining dependencies, training data, a route for a lab-trained model
  into the lock file, and its comparison rule.
- Whether the title counts inside the 1,024 tokens; how a paragraph over
  1,024 tokens is split; the reading rules for resources and hub pages
  (R008, R014).
- How `en_core_web_trf` and any bge-m3 file are confirmed to be available
  in data-only formats at the pinned revisions.
- `models.lock` is a lockfile under the source-limit classes (R089).
