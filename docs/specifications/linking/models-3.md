# SPEC-010 part 3: community models

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in the
[specification decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Part of [SPEC-010](models.md). Requirements: R109, R106, R122, R125,
R013, R015, R025, R044, R052, R101, R103; ADR-0003 and ADR-0007.

## Design: who decides

The owner approves the models Sutradhar ships in `models.lock`. Each
forum's administrator may choose community embedding and reranker
models for their own site (owner's follow-up answer, ADR-0003). The
Verified models stay installed as the Verified tier.

## Design: search ("Models", then "Add model")

- Admin-only page. Inputs: search words; task (embedding or reranker);
  language (default English, MP-09); maximum size (default
  `models.community.max_size`); licence (any, or chosen licences).
  Results are sorted by downloads, as the owner asked; likes and last
  updated are offered as other sorts.
- The service asks the Hub API (`huggingface_hub`, `HfApi`) for 20
  results per page, with the extra fields the admitted library version
  returns in the same call (parameter counts, gated flag, card data).
  Results are cached for 10 minutes, and each administrator sends at
  most one Hub request per second.
- Each result shows name, author, downloads, likes, size on disk, vector
  size, maximum input length, languages, licence, last updated and its
  tier badge: Verified, Compatible or Not supported with the reason.
- Vector size and maximum input length come from small configuration
  files, read for the visible results only (each under 1 MiB); they
  show "checking" until known.
- When `models.community.allowed_authors` is not empty, other authors
  show as Not supported, reason MF-C02.

## Design: compatibility check, before any download

`sutra models check <repo>[@<revision>]` and the "Check" button run the
same action. It uses `model_info` with file metadata, plus
`config.json`, `tokenizer_config.json` and, where present,
`modules.json`, `sentence_bert_config.json`,
`config_sentence_transformers.json`, the pooling configuration and
fastembed's model description. A check never fetches weights.

Steps, each with a reason code shown in plain English:

- MF-C01: the repository name is malformed or the repository is not
  found. Names must match the Hub's `owner/name` pattern before any
  request is sent.
- MF-C02: the author is not in `models.community.allowed_authors`.
- MF-C03: gated. The page asks for an access token (MP-08). Without
  access the result is "needs action", not a refusal.
- MF-C04: no data-only weights. Refused unless the repository has
  safetensors weights (single or split into parts with an index file),
  or ONNX weights that fastembed can run (owner's follow-up answer).
- MF-C05: custom code required. Refused if a configuration has
  `auto_map` or a custom pipeline, refers to Python files, or names a
  sentence-transformers module type outside the allowlist.
- MF-C06: unsupported architecture. It must be supported by the
  admitted sentence-transformers version (safetensors) or the admitted
  fastembed version (ONNX). A reranker must give one score per pair.
- MF-C07: input window too small. Embedding models must accept at least
  1,024 text tokens and rerankers at least 8,192 (owner's follow-up
  answer; R101, R103). The window is the smallest declared limit: the
  sentence-transformers maximum sequence length, the tokenizer's
  maximum length, and the architecture's position limit. A model that
  declares no limit is refused. Tokens are counted with the community
  model's own tokenizer: SPEC-008 cuts its pieces with that tokenizer,
  whose fingerprint is part of the model's compatibility key and of
  SPEC-008's generation key. Any input longer than the window is
  refused with an error and never cut short silently.
- MF-C08: over the memory budget (estimate below).
- MF-C09: the index size that SPEC-008's projection formula gives for
  this model's vector size passes SPEC-008's index size budget.
- MF-C10: no stated licence. Not a refusal under MD-10's answer
  (R125): the verdict is Needs action, and the model may be added only
  after the administrator accepts the stronger warning (download,
  step 2).
- MF-C11: English not among the declared languages and the model not
  declared multilingual (MP-09).
- MF-C12: a configuration file over 1 MiB, or a total download over
  `models.community.max_size`.

Shown with every result, without refusing:

- the vector size beside the active index's, with "full index rebuild
  needed" and an estimated rebuild time from a sample of the forum's
  text counted in the model's own tokens (a new model never shares an
  index with the old one);
- an estimated CPU latency per piece;
- the licence, with an explicit accept step before any download.

The verdict is Compatible, Not supported or Needs action, with every
reason. It is stored with the revision checked and a SHA-256 of the
metadata used, so a later download can prove what was checked.

### Memory estimate

- Parameters come from the safetensors metadata (exact counts) or, for
  ONNX, from the file size.
- Resident memory is estimated as parameters times 4 bytes (full
  precision on a CPU) times 1.2, plus a working-memory allowance for one
  batch at the full window, from Dell measurements of the same
  architecture family.
- The estimate must fit the model's row in the memory table (part 5):
  the candidate replaces the active model of its role. During a trial
  the two take turns; during a rebuild the candidate runs alone while
  new suggestions pause (MD-04, R122).
- Estimated latency uses this host's measured speed from the Verified
  models' self-tests. Both figures are labelled as estimates.

## Design: download (`sutra models add <repo>[@<revision>]`)

1. A passing check on the same revision within 24 hours is required.
   If the repository's current revision has moved, the check runs again
   on the new revision and the administrator confirms it.
2. The administrator accepts the licence and, for a gated model,
   provides an access token. For a model that states no licence
   (MF-C10), the page instead shows a stronger warning: no permission
   to use the model is shown, and its use is the administrator's own
   responsibility. The model is added only if the administrator
   accepts it, and the acceptance is recorded in the audit log
   (MD-10, R125).
3. Only the files needed are selected: weights in the chosen format
   (with any part index), configuration, tokenizer files, and the
   licence and model card for the record. Never fetched: `.bin`, `.pt`,
   `.pth`, `.ckpt`, `.pkl`, `.pickle`, `.joblib`, `.h5`, `.msgpack`,
   `.npy`, `.py`, compiled libraries or archives.
4. Files download into `staging/`, resuming after interruptions, with a
   progress bar in bytes (SPEC-011). Two published fingerprints check
   them during download. Large files are kept on the Hub by Git LFS
   ("large file storage"), which publishes each file's SHA-256. Small
   files are kept in Git itself, which publishes a "blob identifier": a
   fingerprint Git computes from the file's bytes. Sutradhar records its
   own SHA-256 of every file. A file larger than declared is cut off and
   refused.
5. The overlay entry is written: repository, revision, per-file size
   and SHA-256, format, runtime, measured vector size and window,
   tokenizer fingerprint, licence and licence-text SHA-256, check id and
   licence-accept event. The overlay's digest in the store is updated;
   files move to their read-only folder.
6. The model is "added (inactive)". It is never updated automatically:
   to use a newer revision, the administrator adds that revision as a
   new entry.

## Design: community self-test

Community models have no publisher-recorded expected outputs, so the
first self-test checks behaviour:

- outputs have the declared vector size, are finite and non-zero, and
  are normalised when the configuration says so;
- the same input twice gives the same output within 0.000001;
- a fixed paraphrase pair scores higher than an unrelated pair
  (embedder), and a relevant passage higher than an unrelated one
  (reranker).

On success its outputs are written into the overlay as the expected
values for later self-tests on this host. A failure keeps it inactive.

## Design: trial on the forum's own links

`sutra models trial <repo>` and the "Trial" button run link
reconstruction: hide an existing link and see whether each model finds
its destination again.

- Sample: up to `models.trial.sample_size` (default 500) existing
  internal links that members wrote in guest-visible posts (R013).
  Links inserted by Sutradhar are excluded so the active model is not
  favoured. The sample uses a recorded random seed, spread across
  forums in proportion to their link counts.
- Pool: the true destinations plus guest-visible distractor threads up
  to `models.trial.pool_size` (default 5,000), with the same seed.
- For each link, the source context with that link unlinked (in memory
  only; the forum is never changed) goes through SPEC-009's matching
  and anchor functions twice, once per model, over the same pool.
- An embedder candidate embeds the pool and sources with pieces cut by
  its own tokenizer, taking turns with the active embedder, so
  suggestions continue during a trial (R122's pause applies to
  rebuilds); the active model reuses its stored vectors. A reranker
  candidate reranks the same top 50 from the active embedder.
- Results side by side: recall at 1, 3 and 5 (the share of links whose
  true destination is first, in the top 3, in the top 5), anchor
  agreement (as SPEC-009 defines it), latency p95 per piece, projected
  time to the first suggestion and the estimated full reindex time,
  each with sample size and 95% interval.
- The trial is a background job at low priority with progress and
  checkpoints. It stores numbers and post and thread ids, never post
  text. SPEC-012's trial arena reads these results.

## Design: activation and rollback

- `sutra models activate <repo>` or "Activate", after a finished trial
  (MP-10).
- The 30-second first suggestion (R015): if the trial projects that the
  candidate misses it on this host where the active model meets it,
  activation goes ahead only after a warning stating both projected
  times, once the administrator confirms; the confirmation is recorded
  in the audit log (MD-05, R125). From the switch, that forum runs outside the
  qualified target: its health page says "outside the 30-second target:
  community model chosen by an administrator", and its alerts use its
  own trial figures (ADR-0007). R015 is waived for that choice only.
- Embedder: before starting, the page states that new suggestions
  pause until the rebuild completes, with the estimated time. SPEC-008
  then builds a complete new generation (every eligible piece and
  summary vector, cut with the candidate's tokenizer) in the
  background; the candidate runs alone and suggestions already made
  stay approvable (MD-04, R122). Posts written or edited during the
  build are read into the new generation. On completion one atomic
  settings change switches model and index together in one step, and
  new suggestions resume.
- Reranker: the switch follows the self-test; no rebuild is needed.
  Each suggestion records which reranker scored it.
- Before the switch, an activation can be cancelled. The active model
  is loaded again, new suggestions resume, and posts written or edited
  during the build are read by it as after a disable (part 4). The
  partial generation is kept for resuming or removed at the
  administrator's request.
- After the switch, the previous model and its index are kept for
  one-click rollback until the next activation for that role. Their
  disk use is shown.
- `sutra models rollback --role <role>` or "Roll back" switches back in
  one action. For an embedder, posts read since the switch are read
  again with the restored model; only those posts wait.
- `sutra models remove <repo>` deletes an inactive community model's
  files and overlay entry. The active and previous models cannot be
  removed; activate or roll back first.

## Design: audit, badge and health

- Every step writes an audit event (lekha): check, licence accept,
  stronger warning accepted for a model with no stated licence (MD-10),
  access token supplied (the fact, never the value), download start,
  finish or failure, verification, self-test, trial start and result,
  slower-target warning confirmed (MD-05), activation start, switch,
  cancellation, rollback, removal and each settings change. Fields:
  time, actor (an administrator reference or "system"), action,
  repository, revision and result code. Identifying details are kept
  apart from the minimal event (R052).
- Community rows carry the badge "community, untested by Sutradhar" and
  the same status, latency, error and index-match reporting as Verified
  rows (ADR-0003). A model added without a stated licence also shows
  "no stated licence: accepted by an administrator on <date>", and an
  active model chosen against a slower trial shows the outside-target
  line above (R125).
- The project's nightly run verifies only the Verified tier (ADR-0003);
  each forum host's daily check verifies every installed model.

## Settings

Settings follow R025: applied atomically between jobs, with history;
those needing a restart of the model-host process are marked.

- `models.community.enabled`: true (owner's default). False hides "Add
  model" and refuses community commands; active ones: MP-12.
- `models.community.allowed_authors`: empty, meaning any author
  (owner's default).
- `models.community.max_size`: `auto`, the largest size whose memory
  estimate fits its row in the memory table (proposed).
- `models.memory_budget`: `auto`, what the memory table leaves for
  models (MP-04); restart marked.
- `models.disk.margin`: 2 GB. `models.upload.chunk_size`: 4 MiB.
- `models.download.retries`: 5, with growing pauses.
- `models.hub.timeout`: 30 seconds per Hub request.
- `models.trial.sample_size`: 500. `models.trial.pool_size`: 5,000.
- `models.health.latency_factor`: 2.0 and `models.health.error_rate`:
  1% (MP-07).
- `models.active.embedder`, `models.active.reranker`: changed only by
  activation, update and rollback.

## Commands

Every command uses the same action as its page (R044), needs the same
permission, shows progress when it can run longer than two seconds
(R110), offers `--json` output and returns a non-zero exit code on
failure. Help text starts with the meaning of the glossary name.

- `sutra models status`: the health rows.
- `sutra models install`, `bundle <file>`, `import <bundle>`: part 2.
- `sutra models verify [<id>]`, `self-test [<id>]`, `update [<id>]`.
- `sutra models search <words> [--task] [--language] [--max-size]
  [--licence]`.
- `sutra models check <repo>[@<revision>]`.
- `sutra models add <repo>[@<revision>]`. An access token is read only
  from a hidden prompt or `--token-stdin`, never from an argument. The
  licence is shown and accepted at a prompt, or unattended with
  `--accept-licence <SHA-256 of the licence text>`, which binds the
  acceptance to that exact text.
- `sutra models trial <repo>`, `activate <repo>`,
  `rollback --role <role>`, `remove <repo>`.
- Dell only: `sutra models pin <repo>@<revision> --role <role>` and
  `sutra models lock-check` (part 1); retraining commands in part 4.
