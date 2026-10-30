# SPEC-010 part 2: verified models

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in the
[specification decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Part of [SPEC-010](models.md). Requirements: R106, R098, R120, R121,
R122, R123, R129, R015, R020, R022, R023, R025, R044, R051; ADR-0002.

## Design: install

### The "Install models" setup step

1. Preflight. The service's container is running (R129); its data
   directory is writable only by the service account; free disk space
   covers the download, the index size from SPEC-008's projection
   formula (room for a second index generation during a rebuild
   included, and within SPEC-008's index size budget) and a margin
   (`models.disk.margin`, default 2 GB); the memory table in part 5
   fits, with the monitoring containers' declared allowance (R121);
   and the official source is reachable. If it is not, the page offers
   the offline bundle route.
2. The page lists each Verified model with its purpose, download size
   and licence (the full text opens in place), any data-terms note such
   as research-only training data (R099), the total size and an
   estimated time at the measured download speed.
3. One click, "Install models", starts one background job (karya) with
   a progress bar counted in bytes and one stage per model (SPEC-011).
   The same action runs headless as `sutra models install`.
4. Each listed file is downloaded from the entry's official source at
   the pinned revision into `staging/`, over HTTPS with certificate
   checks. An interrupted file resumes where it stopped: the download
   asks the server for only the missing bytes (an HTTP "range request").
   The SHA-256 is computed while the file is written.
5. Size and SHA-256 are compared with `models.lock`. On a mismatch the
   staged file is deleted (it is an unverified download, not forum
   data) and fetched once more from the start; a second mismatch fails
   the model with MF-E03 and raises an alert. A failed file is never
   moved, loaded or kept.
6. When every file of a model has passed, the files move by an atomic
   rename to `<entry id>/<revision>/` and become read-only. A marker
   beside that folder, `<revision>.verified.toml`, records the time and
   digests.
7. The model is loaded and self-tested (below). Load time and memory
   are recorded, and the status becomes "loaded".
8. The audit log (lekha) records the start, each model's result and
   the finish.

The step can be run again at any time: files already present are
re-hashed and skipped when they match.

For bge-m3, Sutradhar fetches only the official ONNX export
(`onnx/model.onnx` and `onnx/model.onnx_data`) with its tokenizer and
configuration files, never the pickle files (R120). For the spaCy
pipeline, `en_core_web_lg` (R120), Sutradhar fetches only the files of
the pipeline folder listed in the lock. It never installs or imports
the spaCy model package, so no Python code shipped with the model runs.

### Offline bundles and upload

- `sutra models bundle <output file>`, run on any machine with internet
  access and the same Sutradhar release, downloads and verifies the
  Verified files and writes an uncompressed tar archive with a
  `bundle.toml` manifest listing entry ids, revisions, paths, sizes and
  SHA-256 values.
- `sutra models import <bundle>`, or "Upload bundle" in the setup step,
  reads the archive as a stream. It refuses any member that is not a
  regular file, has an absolute path, contains `..`, repeats a name, or
  is not listed in `models.lock` for that entry and revision. Each file
  is hashed and compared with `models.lock` itself; the bundle's own
  manifest is never trusted alone. Steps 6 to 8 above then follow.
- Upload sends the bundle in chunks (`models.upload.chunk_size`,
  default 4 MiB). PHP forwards each chunk to the service and releases
  it, so PHP's added peak memory stays under 8 MB per request (R022).
  An interrupted upload resumes from the last confirmed chunk.
- Community models on servers without internet access: MP-11.

## Design: verify

- When: after every download or import; when the model-host process
  first loads a model (below); once a day by a scheduled job for every
  installed model; and on demand with `sutra models verify` or the
  health page's "Verify now" button.
- What: every listed file is present with the right size and SHA-256;
  no unlisted file sits in the revision folder; owner and permissions
  are unchanged.
- Result: the time of the last successful check is shown on the health
  page. A failure marks the model failed (MF-E12), raises an alert and
  offers re-installation.
- Verification reports progress in bytes, and the self-test in models,
  through the one progress system (SPEC-011, ADR-0004).

## Design: the model-host process

### One process, one copy of each model

Every model is loaded once, in one model-host process shared by every
lane, as SPEC-008 sets out; no worker loads its own copy. Trials,
rebuilds and self-tests use the same process. The fetcher is a separate
process that downloads files and never loads a model.

### Loading a model

1. Hub network use and library telemetry are switched off before any
   model library is imported (offline mode), so loading cannot download.
2. Only the local folder path is passed in, never a repository name.
3. The pre-load scan runs: the folder must hold exactly the locked
   files; every file's contents pass the inspection in part 5;
   configuration files must not ask for custom code (`auto_map` or a
   custom pipeline); sentence-transformers module types, spaCy
   registered functions and ONNX operator sets must be on reviewed
   allowlists; ONNX external weight files must sit inside the folder.
4. The runtime is called with `trust_remote_code` set to false as a
   fixed constant that no setting, environment variable or argument can
   change, with local files only. PyTorch models must load from
   safetensors. ONNX models load from the local path with downloading
   disabled: bge-m3 through ONNX Runtime (R120), community models
   through ONNX Runtime or fastembed. Where the runtime does not form
   the vector, Sutradhar applies the lock's `pooling` and `normalise`
   (for bge-m3, the first token's vector, normalised). spaCy loads
   `en_core_web_lg`'s pipeline folder without PyTorch (R120).
5. Loading does not start when the lock's `memory_mb` would break the
   memory table (part 5). After loading, measured resident memory is
   compared with the table; a model over its row is unloaded with
   MF-E07 (R023).
6. The device, load time and memory are reported to the health page.

CPU priority and thread counts follow SPEC-008's settings.

### Checking files at load (MP-05)

- The first time the model-host process loads a model, every file is
  fully re-hashed.
- While that process lives, reloading the same files (for example when
  two embedders take turns during a trial, part 3) checks only each
  file's identity, size and modification time. The files are read-only
  and owned by the service account, so changing them needs that
  account's rights; the daily job still re-hashes everything.
- A new model-host process (after a restart or crash) re-hashes again.

### How long a model stays loaded

A model stays loaded for the life of the model-host process, except:

- the first-pass model is loaded only while a backlog sweep runs;
- during an embedder trial, the two embedders take turns, and
  suggestions continue (part 3);
- during a rebuild for a new embedder, new suggestions pause, so the
  serving embedder is unloaded and the new one runs alone until the
  switch or a cancellation (MD-04, R122);
- a model whose role changes or is rolled back is unloaded after its
  last request finishes.

Under MD-03's answer (R121) every model stays loaded side by side; the
stage turn-taking of MD-03's other option is not built.

## Design: update, never automatic

1. A new add-on release (F021, R050) may pin a new revision. When the
   installed revision equals the entry's `[model.previous]`, the health
   page shows "update available" with a button. The installed model
   keeps running, because the previous revision is still in the lock.
2. The administrator presses "Update" (`sutra models update [<id>]`).
   Nothing is downloaded before that.
3. The new revision is downloaded alongside the old one, verified, then
   loaded in its turn, never beside a model of the same role, and
   self-tested; for the embedder, once step 4 has paused suggestions.
4. For a model whose outputs form an index (`embedder`, `first_pass`),
   SPEC-008 builds a new index generation with the new revision in the
   background. For the embedder, new suggestions pause for the whole
   rebuild, so the new model runs alone (MD-04, R122); the page says so
   before the administrator starts it. Suggestions already made stay
   approvable, because each records the model and index version it was
   made with. The old model's files and index stay on disk, unchanged.
5. When the new generation is complete, one atomic settings change
   switches model and index together in one step, so there is no
   mismatch period (R122). New suggestions then resume, the live queue
   of new and edited posts first (R123). No suggestion mixes the two.
6. The previous revision and its index are kept for rollback until the
   next update of that role.
7. For `reranker` and `nlp_pipeline` there is no index rebuild: the
   switch follows the self-test. Stored outputs that depend on them
   follow the compatibility rule below.
8. A model whose installed revision is in neither `revision` nor
   `[model.previous]` is never loaded (MF-E10). Its row shows ADR-0002's
   "not installed" for the pinned revision, with the Update button, and
   readiness is "not ready" until it is installed.

`sutra models rollback --role <role>` returns to the kept previous
revision in one action when its files verify. For the embedder, posts
read after the switch are read again with the restored model; only
those posts wait, and every other suggestion continues.

## Design: the compatibility rule

- A model's compatibility key is its entry id, repository, revision,
  `index_compat`, vector size and tokenizer fingerprint.
- SPEC-008 stores the embedder's key in each index generation record
  (and the first-pass key for its shortlist store). Any other stored
  output that depends on a model, such as entity records from the spaCy
  pipeline, stores that model's key. Each suggestion records the keys
  of every model that produced it.
- A stored output may be used only while its key equals the active
  model's key for that role. Otherwise it counts as missing evidence:
  visible, never treated as zero (R015).
- Suggestions are the exception: each records the model and index
  version it was made with, which matched by construction, so one
  already made stays approvable whatever the current model; approving
  a link never depends on the active key (R122).
- When the serving generation's key differs from the active embedder's
  key, new suggestions are blocked, readiness reports "not ready for
  suggestions", and the health row shows "index mismatch" until the
  rebuild completes (owner's follow-up answers, ADR-0002). The update
  and activation flows switch model and index together, so this guards
  against changes made any other way.
- Defence in depth: every search request carries the active key, and
  the index refuses a request whose key differs (MF-E09).
- A first-pass mismatch pauses only the early shortlist (MP-03),
  because no suggestion depends on it (R098).

## Design: health, self-test and readiness

### Health rows

One row per model, as ADR-0002 sets out: status, revision, time the
checksum was last verified, load time, memory in use, device, inference
latency p50 and p95 over the last hour (the times within which half and
95% of calls finish), error count over the last hour and the last error
message, which never contains post text or secrets.

Statuses are exactly ADR-0002's: not installed, downloading (with
progress), verifying, loaded, degraded, failed, update available,
index mismatch. Community and site-trained rows add their badge
(part 3). The Models page's lifecycle labels are separate (MP-18).

### Self-test

"Run self-test" (`sutra models self-test [<id>]`) sends the fixed
inputs recorded in `models.lock` through each model:

- embedder and first pass: four short neutral sentences; the vector
  size, vector lengths and the six pairwise similarities must match
  within the tolerance (default 0.002). bge-m3's expected values come
  from its ONNX run; `sutra models pin` also checks that run once
  against the example scores on the publisher's model card, since the
  pickle cannot be opened to compare (R120);
- reranker: four fixed text pairs; the scores must match within 0.01
  and keep the same order;
- spaCy pipeline (`en_core_web_lg`): two fixed sentences; the named
  entities with their spans and types, and the noun phrases, must
  match exactly.

Tolerances absorb small numeric differences between processors
(ADR-0002 recorder's note). A failure marks the model failed and
suggestions not ready (MP-06).

### Readiness

- Not ready: a required model is missing or failed.
- Not ready for suggestions: an index mismatch, a failed self-test,
  or "model update in progress" while new suggestions pause for a
  rebuild (R122). Suggestions already made stay approvable in every
  case.
- Degraded, still ready with a warning: latency or error rate over its
  threshold (MP-07).
- Ready: everything else.

### Metrics and alerts

Metrics go through the health and metrics component (drishti), with
English names: `sutradhar_model_inference_seconds` (histogram),
`sutradhar_model_errors_total`, `sutradhar_model_resident_bytes`,
`sutradhar_model_status` and
`sutradhar_model_checksum_verified_timestamp`, labelled by entry id and
role. Alerts are grouped on the dashboard and in XenForo staff alerts
with a central error code and a manual link (R051).

## Interfaces for other components

- `ModelRegistry.get(role)` returns the only handle other components
  may use: `Embedder.embed(texts)` gives a matrix of 32-bit numbers,
  normalised as the lock entry says; `Reranker.score(pairs)` gives one
  number per pair; `NlpPipeline.parse(texts)` gives parsed documents.
  `Embedder.tokenizer()` gives the tokenizer SPEC-008 cuts pieces with.
- `ModelRegistry.key(role)` returns the compatibility key;
  `ModelRegistry.require_key(role, key)` raises `IndexMismatch`.
- Every runtime sits behind the same interfaces. A deterministic fake
  embedder implements `Embedder` for unit tests without downloads
  (ADR-0002).
- Events (sanchar): `models.status_changed`, `models.switched`,
  `models.rolled_back`, `models.verify_failed`.
- Actions: every page button and `sutra models` command calls the same
  permission-checked action (R044). Changes need the XenForo admin
  permission "Manage models"; viewing health needs "View models".
- Health endpoints: `/actuator/health/models` and `/actuator/models`.
- PHP reaches the service only through signed requests (dwarpal) and
  never reads model files (R043).
