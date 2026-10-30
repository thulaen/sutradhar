# SPEC-010: Models: files, community models, retraining and removal

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in the
[specification decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Requirements: R106, R109, R105, R108, R050 as amended, R021 and R092 as
amended, R120, R121, R122, R125 and R129; also R013, R015, R018, R020,
R023, R025, R027, R028, R043, R044, R051, R052, R089, R090, R098, R099,
R100, R110, R119, R123 and R127. Decisions: ADR-0002, ADR-0003,
ADR-0006 and ADR-0007.

Drafted on 30 October 2026 by a Claude agent, following the owner's
request to draft the linking specifications now (R118), revised the
same day after an independent review, and revised again to apply the
owner's answers, each change marked with its requirement number. The
ten choices that needed the owner's judgement, each with its answer,
are in [Decisions](models-7.md#decisions-answered-by-the-owner); the
technical ones, which the owner accepted as a group, are in
[Proposals](models-8.md#proposals).

Features served (see the [feature register](../README.md)):

- F021, point-and-click installation, upgrade and removal: installing,
  updating, disabling and uninstalling models and the Python service.
- F019, third-party isolation: community models and data-only formats
  (R028), never running code from a model repository.
- F010, feedback learning: retraining in the Dell lab (R105).
- Contributions to F001 (settings), F018 (pages and `sutra` commands
  through one action layer), F022 (audit and diagnostics) and F006
  (admission records for models and libraries).

Other linking drafts are referred to by number: SPEC-008 (reading and
indexing), SPEC-009 (matching and anchors), SPEC-011 (progress system)
and SPEC-012 (visual layer).

## Two format conflicts, resolved (R120)

A check of the official files on 30 October 2026 (part 7) found that
two of the owner's model choices in R098 conflicted with the owner's
rule that only data-only formats are loaded and pickle files never are
(R106, R028, ADR-0002): bge-m3 publishes its PyTorch weights only as a
pickle file (MD-01), and `en_core_web_trf` stores its transformer
weights as a pickle inside spaCy's own files (MD-02).

The owner resolved both (R120). bge-m3 is used through its publishers'
official ONNX export at full precision, run by ONNX Runtime; spaCy's
`en_core_web_lg`, which is data only, replaces `en_core_web_trf` and
runs without PyTorch. `sutra models pin` still refuses every pickle
file, and part 7 keeps the check as the record of what was found.

## Parts

1. This page: scope, terms, tiers, roles and `models.lock`.
2. [Verified models](models-2.md): install, offline import, verify,
   the model-host process, update, rollback, the compatibility rule,
   health and interfaces.
3. [Community models](models-3.md): search, compatibility check,
   download, self-test, trial, activation, rollback, audit, settings
   and commands.
4. [Retraining, disabling and uninstalling](models-4.md), with the
   table of every store and what uninstall does to it, and failure
   behaviour.
5. [Privacy, security and budgets](models-5.md), with the whole memory
   table for the service cap (6 GB, or 8 GB once Dell measurements
   confirm the estimate, R121) and the monitoring allowance.
6. [Testing and acceptance cases](models-6.md).
7. [Format check and the owner's decisions](models-7.md).
8. [Proposals and recorder's notes](models-8.md).
9. [Files on the forum host](models-9.md): the per-site overlay lock,
   active models and file storage, moved from this page.

## Scope

### In scope

- `models.lock`: its contents, file format, the tools that write and
  check it, and its class under the source limits (R089).
- The per-site overlay lock that pins a forum's own community models.
- Installing, importing offline, verifying, loading, updating, rolling
  back and monitoring every model used on a forum host: the R098 set as
  changed by R120 (bge-m3 through its official ONNX export,
  gte-reranker-modernbert-base, potion-retrieval-32M and spaCy's
  `en_core_web_lg`) and any model a later approved specification adds.
- The model side of the compatibility rule: new suggestions are
  blocked while the index was built with a different model revision
  (suggestions already made stay approvable), and new
  suggestions pause while a new embedder rebuilds the index (R122).
- Community embedding and reranker models from Hugging Face (R109).
- Retraining bge-m3 or the reranker in the Dell lab (R105), the
  comparison test, the owner's approval and the route into use.
- Disabling and uninstalling the Python service and the rest of
  Sutradhar's compose project (R129), and the table that classifies
  every store of SPEC-008 to SPEC-012 for uninstall (R108, R050 as
  amended).
- The whole memory table for the service cap (6 GB, or 8 GB once Dell
  measurements confirm the estimate, R121), including SPEC-008 to
  SPEC-012's items and the monitoring services' separate allowance,
  and retraining within the 9 GB lab ceiling.
- The admission and privacy-review records this design needs.

### Out of scope

- Reading, splitting into pieces, embedding, the index, its size budget
  and projection formula, and the first backlog run (SPEC-008).
- Matching, reranker inputs, anchors, the 30-second first suggestion,
  and the R100 challenger comparison with its "clearly wins" rule
  (SPEC-009). This specification only pins the challengers' files.
- The progress handle and its views (SPEC-011); visuals such as model
  cards and the model trial arena (SPEC-012), which read data stored
  here.
- The export mechanism for the add-on's other owned data at uninstall,
  and the installer's other steps (F021, F022).
- Capturing moderators' decisions and reason buttons (R018), which the
  review page owns. Retraining only consumes them.

## Terms

- Model: a set of files holding learned numbers (weights) plus the
  configuration that says how to use them.
- Embedder: a model that turns text into a list of numbers (a vector)
  so that similar texts get similar vectors. Reranker: a model that
  reads a pair of texts and scores how well they match.
- Text tokens: the word pieces a model reads. A piece of up to 1,024
  tokens is roughly 700 to 800 English words. Each model has its own
  tokenizer, the rule that cuts text into its tokens.
- Access token: a secret key from a Hugging Face account, needed to
  download gated models. Always called "access token" here.
- Hugging Face Hub (the Hub): the public service that hosts model
  repositories. Gated model: one whose publisher requires each account
  to accept terms before downloading.
- Revision: the exact Git commit identifier (40 hexadecimal characters)
  of a model repository. It names exact file versions, unlike a branch
  name, which can move.
- Checksum: a SHA-256 fingerprint of a file's exact bytes. Any change to
  the file changes the fingerprint.
- safetensors: a weight file format that holds only numbers and a small
  header, and cannot run code when opened.
- Pickle: Python's own save format. Opening a pickle file can run
  arbitrary code, so it is never loaded (R106).
- ONNX: an open format describing a model as a graph of standard
  operations, run by ONNX Runtime. fastembed is a library that runs
  ONNX models through ONNX Runtime.
- Index: the stored vectors that SPEC-008 builds and SPEC-009 searches.
- Docker compose: a file listing the containers an application runs,
  each with its memory cap; the compose project is the set of
  containers, networks and volumes started from it. The whole of
  Sutradhar runs from its default compose file (R129).

## Tiers

- Verified: a model in `models.lock`. The owner approves these (R027,
  R100; ADR-0003, "Who decides").
- Compatible: a model that passes the compatibility check and is chosen
  by a forum's administrator for that site only, then pinned in the
  per-site overlay with the badge "community, untested by Sutradhar".
- Not supported: a model that fails the check, shown with the reason.
- Site-trained (MD-07's answer): a model retrained in the Dell lab and
  approved by the owner, pinned in that forum's overlay.

## Roles

Each lock entry has one role. Exactly one model is active per role.

- `embedder`: reads every eligible post and builds the index (bge-m3,
  R098, through its official ONNX export, R120). Community models may
  take it.
- `reranker`: scores the top 50 candidates (gte-reranker-modernbert-base,
  R103). Community models may take it.
- `first_pass`: sweeps the backlog for an early shortlist
  (potion-retrieval-32M). Verified only.
- `nlp_pipeline`: entities and parsing (`en_core_web_lg`, which
  replaced R098's `en_core_web_trf`, R120). Verified only.
- `lab_only`: pinned for lab comparisons, such as SPEC-009's
  challengers, and never installed on a forum host (MP-02).

## `models.lock`

### Place, format and authority

`models.lock` sits at the repository root. It is the only source of
truth for every model Sutradhar ships (R106, ADR-0002). Setup, tests,
the nightly run and the health page all read it. A changed entry is a
change to a shipped model and needs the owner's approval (R027, R100).

The file uses TOML, a plain settings format that Python's standard
library reads, written without indentation so that a SHA-256 line fits
in 80 characters (MP-01). Only `sutra models pin` writes it, in one
canonical layout; `sutra models lock-check` confirms that it parses,
matches the schema, and is byte for byte what the writer would produce.
It is expected to exceed 300 lines; under MD-06's answer it is a
tool-written lockfile, exempt from the 300-line limit, with lines still
within 80 characters.

### Example entry

A neutral example in the form bge-m3's entry takes (R120); real
entries are produced by `sutra models pin`.

```toml
schema = 1

[[model]]
id = "embedder-example"
role = "embedder"
purpose = "Reads every eligible post and builds the search index."
source = "huggingface"
repo = "example-org/example-embedder"
revision = "<40 hexadecimal characters>"
format = "onnx"
runtime = "onnxruntime"
dtype = "float32"
pooling = "first_token"
normalise = true
vector_size = 1024
max_tokens = 8192
index_compat = 1
tokenizer_sha256 = "<64 hexadecimal characters>"
licence = "<identifier from the admission record>"
licence_sha256 = "<64 hexadecimal characters>"
data_terms = "<for example: research-only training data, R099>"
memory_mb = 2300
admission = "<admission record identifier>"

[[model.file]]
path = "onnx/model.onnx"
size = 1000000
sha256 = "<64 hexadecimal characters>"

[[model.file]]
path = "onnx/model.onnx_data"
size = 2270000000
sha256 = "<64 hexadecimal characters>"

[model.self_test]
inputs = ["<fixed neutral sentence 1>", "<fixed neutral sentence 2>"]
expected = [0.8123]
tolerance = 0.002

[model.previous]
revision = "<40 hexadecimal characters>"
index_compat = 1
```

### Fields

- `id`, `role`, `purpose`: a stable entry name kept across revisions,
  the role, and a plain-English purpose for the install page.
- `source`, `repo`, `revision`: the official source repository and a
  full commit identifier; branch names and tags are refused.
- `format`: `onnx`, `safetensors` or `spacy-directory`. `onnx` serves
  bge-m3's official export (R120) and community models (R106). Each
  format has a fixed list of allowed file types.
- `runtime`: `onnxruntime` (bge-m3, R120), `sentence-transformers`,
  `spacy` (without PyTorch, for `en_core_web_lg`) or, for community
  models, `fastembed`.
- `dtype`: `float32` for full precision (R098).
- `pooling`, `normalise`: how one vector is formed from the model's
  outputs, applied by Sutradhar where the runtime does not do it; for
  bge-m3, the first token's vector, normalised, as its model card
  describes (R120).
- `vector_size`, `max_tokens`: output size and input window.
- `index_compat`: a whole number raised whenever anything that changes
  the produced vectors changes (revision, pooling, normalisation,
  prefix text, window or precision).
- `tokenizer_sha256`: a fingerprint over the tokenizer files, so pieces
  cut by one tokenizer are never read with another (SPEC-008).
- `licence`, `licence_sha256`, `data_terms`: shown at install.
- `memory_mb`: resident memory measured on Dell when pinned.
- `admission`: the admission record that cleared the model (R067).
- `[[model.file]]`: every file to fetch, with byte size and SHA-256.
  Only listed files are ever downloaded or loaded.
- `[model.self_test]`: fixed inputs, expected outputs and tolerance,
  recorded on Dell with the pinned libraries.
- `[model.previous]`: one earlier revision, with its own file list,
  still accepted so an upgraded forum keeps running its installed model
  until the administrator presses Update, and for rollback.
- `ship`: `false` for `lab_only` entries; when absent, it is shipped.

### Writing an entry

`sutra models pin <repo>@<revision> --role <role>` runs on Dell, where
all work happens (R119), never on the Windows computer, which only runs
the app (R072). It reads the Hub's file list for that revision, chooses
only allowed file types (for bge-m3, the `onnx/` export and never the
pickle files, R120), downloads them into the lab cache, computes each
SHA-256, inspects every file's contents (part 5), loads the model
offline, measures memory, records self-test outputs and writes the
entry. It refuses a revision without the needed data-only files and
reports what is missing.

The per-site overlay lock, the active-model settings and file storage
on the forum host are in [part 9](models-9.md).
