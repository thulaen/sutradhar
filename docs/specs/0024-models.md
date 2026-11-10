# 0024: Models

This part covers every model Sutradhar runs: the lock file that pins
them, installing and checking their files, loading them in pure Rust,
updates and rollback, community models chosen per site, why the
transformer models are never retrained, and what stopping and removal
do to every stored item. It is roadmap Phase 4 item 3. Reading and
indexing are in 0022, matching and anchors in 0023, progress in 0016
and the visual layer in 0017.

## What it does

### Terms

- Model: files of learned numbers (weights) plus the settings that say
  how to use them. Embedder: turns text into a list of numbers (a
  vector), so similar texts get similar vectors. Reranker: reads a pair
  of texts and scores how well they match.
- Text tokens: the word pieces a model reads; 1,024 tokens is roughly
  700 to 800 English words. Each model has its own tokenizer.
- The Hub: the public Hugging Face service that hosts model
  repositories. Gated model: one whose publisher makes each account
  accept terms first. Access token: the Hub account secret needed for
  gated models.
- Revision: the full 40-character commit id of a model repository.
  Branch names and tags can move, so they are never used.
- Checksum: the SHA-256 fingerprint of a file's exact bytes.
- Data-only formats: safetensors (numbers plus a small header) and the
  graph format (an open standard describing a model as a graph of
  standard operations, with external weight files). Neither can run
  code when opened. Pickle files, a save format whose loading can run
  any code, are never fetched, imported or loaded.

### Tiers and roles

1. Tiers:
   - Verified: in `models.lock`; the owner approves these.
   - Compatible: passes the compatibility check and is chosen by the
     owner for one site only, pinned in that site's overlay lock, with
     the badge "community, untested by Sutradhar".
   - Not supported: fails the check, shown with the reason.
2. Each lock entry has one role, and exactly one model is active per
   role per site:
   - `embedder`: reads every eligible post and builds the index
     (Qwen3-Embedding-0.6B, from its publisher's official safetensors
     at the published precision). Community models may take it.
   - `reranker`: scores the top 50 candidates
     (gte-reranker-modernbert-base). Community models may take it.
   - `first_pass`: sweeps the backlog for an early shortlist
     (potion-retrieval-32M). Verified only.
   - `nlp_pipeline`: entities and parsing, through Sutradhar's own
     pure-Rust entity model and parser (0026), trained in the Dell lab
     and shipped as safetensors. Verified only.
   - `lab_only`: pinned with `ship = false` for lab comparisons, such
     as 0023's challengers, and never installed on the owner's PC.
   - `judge`: the grading model of 0057's `ModelJudge`. Verified only,
     and never loaded for matching, ranking or the first pass.
3. Any other model a later specification needs, such as a language
   identifier for 0022, is pinned in `models.lock` and follows every
   rule here.

### `models.lock`

1. It sits at the repository root and is the only source of truth for
   every shipped model. Setup, tests, the nightly run and the health
   screen all read it. A changed entry is a change to a shipped model
   and needs the owner's approval.
2. It is TOML without indentation, so a SHA-256 line fits in 80
   characters. Only `sutra models pin` writes it, in one canonical
   layout. `sutra models lock-check` confirms it parses, matches the
   schema and is byte for byte what the writer would produce. It is a
   tool-written lock file, so it may pass 300 lines; its lines still
   stay within 80 characters.
3. Fields of each `[[model]]` entry:
   - `id`, `role`, `purpose`: a stable name kept across revisions, the
     role, and a plain-English purpose for the install screen.
   - `source`, `repo`, `revision`: the official repository and a full
     commit id.
   - `format`: `graph` or `safetensors`, each with a fixed list of
     allowed file types.
   - `runtime`: `tract` (graph format) or `candle` (safetensors).
   - `dtype`: the published precision, never lowered below it; for
     Qwen3-Embedding-0.6B, its published 16-bit numbers.
   - `pooling`, `normalise`: how one vector is formed from the
     outputs, applied by Sutradhar where the runtime does not; for
     Qwen3-Embedding-0.6B, the last token's vector, normalised, with
     the instruction prefix on queries only, as its model card says.
   - `vector_size`, `max_tokens`: output size and input window.
   - `index_compat`: a whole number raised whenever anything that
     changes the vectors changes (revision, pooling, normalisation,
     prefix text, window or precision).
   - `tokenizer_sha256`: a fingerprint over the tokenizer files, so
     pieces cut by one tokenizer are never read with another.
   - `licence`, `licence_sha256`, `data_terms` (for example
     research-only training data): shown at install.
   - `memory_mb`: resident memory measured on Dell when pinned.
   - `admission`: the admission record that cleared the model.
   - `[[model.file]]`: every file to fetch, with byte size and
     SHA-256. Only listed files are ever downloaded or loaded.
   - `[model.self_test]`: fixed inputs, expected outputs and
     tolerance, recorded on Dell with the pinned libraries.
   - `[model.previous]`: one earlier revision with its own file list,
     still accepted, so an upgraded install keeps running its model
     until the owner presses Update, and for rollback.
   - `ship`: `false` for `lab_only`; shipped when absent.
4. `sutra models pin <repo>@<revision> --role <role>` runs on Dell. It reads the
   Hub's file list for that revision, picks only allowed file types (never
   pickle files), downloads them into the lab cache, hashes each, inspects every
   file's contents, loads the model offline, measures memory, records self-test
   outputs and writes the entry. It refuses a revision without the needed
   data-only files and says what is missing.
5. The format check of 30 October 2026, kept as fixtures:
   - The reranker (revision starting `f7481e60`) and potion (starting
     `6fc8051f`) ship `model.safetensors`.
   - Qwen3-Embedding-0.6B's revision is chosen and its file list
     recorded by `sutra models pin` on Dell, which refuses it unless
     its weights come as safetensors with no pickle file needed.
   - A model file that hides a pickle archive inside another container
     is refused; the fixtures keep one such layout.

### The per-site overlay lock and files on the owner's PC

1. `<data directory>/models/<site>/site-models.lock` is runtime data,
   not a repository file. It has the `models.lock` schema plus `tier`
   (`community`), `added_at`, `check_id`,
   `licence_accept_event` and each large file's Hub-published SHA-256
   beside Sutradhar's own.
2. Only the service writes it. Its own SHA-256 is kept in the
   service's store; a mismatch means it was edited outside the service,
   and its models are refused (MF-E11).
3. It may only add entries. An entry with a Verified entry's `id` or
   `repo` is refused. It holds no personal data; people appear only as
   audit event ids.
4. For the embedder and reranker, the active model is a site setting
   (`models.active.embedder`, `models.active.reranker`) that defaults
   to the Verified entry. For every role, only activation, update and
   rollback change the active model or revision, as one atomic settings
   version between jobs, with history.
5. Storage: `<data directory>/models/<entry id>/<revision>/` holds
   verified, read-only files; `staging/` holds partial downloads;
   `trials/` holds trial results (numbers and ids) and trial vectors.
   Folders are `0700` and verified files `0400`, owned by the service
   account. Per role, the active model and one previous model are kept;
   inactive community models stay until the owner removes them. Disk use
   is shown on the Models screen.

### Installing Verified models

1. Preflight: the data directory is writable only by the service
   account; free disk covers the download, the index size from 0022's
   projection formula (with room for a second index generation during
   a rebuild, within 0022's index size budget) and a margin
   (`models.disk.margin`, 2 GB); the memory table fits; and the
   official source is reachable, or else the offline bundle route is
   offered.
2. The screen lists each Verified model with purpose, download size,
   licence (full text opens in place), any data-terms note, the total
   size and an estimated time at the measured download speed.
3. "Install models" starts one durable job with a progress bar in bytes
   and one stage per model (0016). `sutra models install` runs the same
   action.
4. Each listed file is fetched from the official source at the pinned
   revision into `staging/`, over HTTPS with certificate checks. An
   interrupted file resumes with a range request. The SHA-256 is
   computed while writing.
5. Size and SHA-256 are compared with `models.lock`. On a mismatch the
   staged file is deleted and fetched once more; a second mismatch fails
   the model with MF-E03 and raises a possible-tampering alert. A failed
   file is never moved, loaded or kept.
6. When every file of a model passes, the files move by atomic rename
   to `<entry id>/<revision>/` and become read-only, with a
   `<revision>.verified.toml` marker holding the time and digests.
7. The model is loaded and self-tested; load time and memory are
   recorded and the status becomes "loaded". The audit log records the
   start, each model's result and the finish.
8. The step can be run again: present files are re-hashed and skipped
   when they match. For each model only its listed safetensors,
   tokenizer and configuration files are fetched. No code shipped with
   a model ever runs.
9. Offline bundles: `sutra models bundle <file>`, on any connected
   machine with the same release, verifies the Verified files and
   writes an uncompressed tar with a `bundle.toml` manifest (entry ids,
   revisions, paths, sizes, SHA-256). `sutra models import <bundle>`,
   or "Import bundle" in the app, reads it as a stream and refuses any
   member that is not a regular file, has an absolute path, contains
   `..`, repeats a name, is larger than declared or is not listed in
   `models.lock` for that entry and revision. Each file is hashed
   against `models.lock` itself; the manifest is never trusted alone.
   Steps 6 and 7 follow.

### Verifying

- When: after every download or import; when the model host first
  loads a model; daily for every installed model; and on demand with
  `sutra models verify` or "Verify now".
- What: every listed file present with the right size and SHA-256; no
  unlisted file in the revision folder; owner and permissions unchanged.
- The last good check time is shown. A failure marks the model failed
  (MF-E12), raises an alert and offers re-installation. Progress is
  reported in bytes, and the self-test in models.

### The model host

1. Every model is loaded once, in one model host shared by every lane
   and site, as 0022 sets out; no worker loads its own copy. Trials,
   rebuilds and self-tests use it too. A separate fetcher downloads
   files and never loads a model.
2. Loading:
   - only the local folder path is passed in, never a repository name,
     and the loader has no network, so loading can never download;
   - the pre-load scan: the folder holds exactly the locked files;
     every file passes content inspection; configuration files must not
     ask for custom code (`auto_map` or a custom pipeline); model
     architectures, pipeline components and graph operator sets must be
     on reviewed allowlists; graph external weight files must sit inside
     the folder;
   - Graph-format models load through tract and safetensors models through
     candle, from local files only. Sutradhar applies the lock's
     `pooling` and `normalise` where the runtime does not;
   - loading does not start when `memory_mb` would break the memory
     table; after loading, measured memory over its row unloads the
     model with MF-E07;
   - load time and memory are reported to the health screen. Processor
     priority and thread counts follow 0022's settings.
3. File checks at load: the first load in a model host re-hashes every
   file in full; a reload in the same process (for example during a
   trial's turn-taking) checks only identity, size and modification
   time; a new model host (after a restart or crash) re-hashes again;
   the daily job always re-hashes in full.
4. A model stays loaded for the model host's life, except:
   - the first-pass model is loaded only during a backlog sweep;
   - during an embedder trial the two embedders take turns and
     suggestions continue;
   - during a rebuild for a new embedder, new suggestions pause, the
     serving embedder is unloaded and the new one runs alone until the
     switch or a cancellation;
   - a model whose role changes or is rolled back is unloaded after its
     last request.
   All other models stay loaded side by side.

### Updates, never automatic

1. A new release may pin a new revision. When the installed revision
   equals `[model.previous]`, the row shows "update available" with a
   button, and the installed model keeps running.
2. Nothing is downloaded before the owner presses "Update"
   (`sutra models update [<id>]`).
3. The new revision is downloaded beside the old one, verified, loaded
   in its turn (never beside a model of the same role) and self-tested;
   for the embedder, after step 4 has paused suggestions.
4. For `embedder` and `first_pass`, 0022 builds a new index generation
   in the background. For the embedder, new suggestions pause for the
   whole rebuild and the new model runs alone; the screen says so first,
   with the estimate. Suggestions already made stay approvable, because
   each records the model and index version it was made with.
5. When the generation is built, one atomic settings change switches
   model and index together, with no mismatch period. New suggestions
   resume, the live queue of new and edited posts first. No suggestion
   mixes the two.
6. The previous revision and its index are kept for rollback until the
   next update of that role.
7. For `reranker` and `nlp_pipeline` there is no rebuild: the switch
   follows the self-test, and stored outputs follow the compatibility
   rule.
8. An installed revision in neither `revision` nor `[model.previous]`
   is never loaded (MF-E10); its row shows "not installed" for the
   pinned revision, with Update, and readiness is "not ready".
9. `sutra models rollback --role <role>` returns to the kept previous
   revision in one action when its files verify. For the embedder,
   posts read since the switch are read again; only those wait.

### The compatibility rule

- A model's key is its entry id, repository, revision, `index_compat`,
  vector size and tokenizer fingerprint.
- 0022 stores the embedder's key in each index generation record, and
  the first-pass key with its shortlist store. Any other stored output
  that depends on a model, such as entity records, stores that model's
  key. Each suggestion records the keys of every model that made it.
- A stored output is used only while its key equals the active key for
  its role; otherwise it counts as missing evidence, shown and never
  treated as zero.
- Suggestions already made stay approvable whatever the current model.
- When the serving generation's key differs from the active embedder's,
  new suggestions are blocked, readiness says "not ready for
  suggestions" and the row shows "index mismatch" until rebuilt.
- Every search carries the active key, and the index refuses another
  key (MF-E09).
- A first-pass mismatch pauses only the early shortlist.

### Health, self-test and readiness

1. One row per model: status, revision, last checksum check, load time,
   memory in use, device, inference latency p50 and p95 over the last
   hour (the times within which half and 95% of calls finish), error
   count over the last hour and the last error, never holding post text
   or secrets.
2. Statuses are exactly: not installed, downloading (with progress),
   verifying, loaded, degraded, failed, update available, index
   mismatch. Community rows add their badge. The
   Models screen's lifecycle labels (checking, added (inactive), trial
   running, rebuilding, active, previous) are separate.
3. Self-test (`sutra models self-test [<id>]`) runs the inputs fixed
   in `models.lock`:
   - embedder and first pass: four short neutral sentences; vector
     size, vector lengths and the six pairwise similarities match
     within 0.002. Qwen3-Embedding-0.6B's expected values come from its
     pinned run, and `sutra models pin` checks that run once against
     the example scores on the publisher's model card;
   - reranker: four fixed pairs; scores within 0.01, same order;
   - entity model: two fixed sentences; entities with spans and
     types, and noun phrases, match exactly.
   A failure marks the model failed and suggestions not ready.
4. Readiness:
   - not ready: a required model is missing or failed;
   - not ready for suggestions: an index mismatch, a failed self-test,
     or "model update running" during a rebuild pause;
   - degraded, still ready with a warning: the last hour's p95 latency
     over `models.health.latency_factor` (2.0) times the self-test
     figure measured at install, or more than
     `models.health.error_rate` (1%) of calls failing;
   - ready: everything else. Suggestions already made stay approvable
     in every case.
5. Metrics, labelled by entry id and role:
   `sutradhar_model_inference_seconds` (histogram),
   `sutradhar_model_errors_total`, `sutradhar_model_resident_bytes`,
   `sutradhar_model_status` and
   `sutradhar_model_checksum_verified_timestamp`. Alerts are grouped in
   the app with an error code and a manual link.

### Interfaces

- `ModelRegistry::get(role)` returns the only handle other modules may
  use: `Embedder::embed(texts)` gives a matrix of 32-bit numbers,
  normalised as the lock says; `Embedder::tokenizer()` gives the
  tokenizer 0022 cuts pieces with; `Reranker::score(pairs)` gives one
  number per pair; `NlpPipeline::parse(texts)` gives parsed documents.
- `ModelRegistry::key(role)` gives the compatibility key;
  `ModelRegistry::require_key(role, key)` returns `IndexMismatch`.
- Every runtime sits behind these traits. A deterministic fake embedder
  serves unit tests without downloads.
- Events: `models.status_changed`, `models.switched`,
  `models.rolled_back`, `models.verify_failed`.
- Every screen button and `sutra models` command calls the same action,
  checked by Cedar: changes need "Manage models"; viewing needs "View
  models". Health and readiness checks sit on Sutradhar Boot.

### Community models

1. Who decides: the owner approves `models.lock`; per site the owner may
   pick community embedders and rerankers. Verified models stay
   installed.
2. Search ("Models", then "Add model"): words, task (embedding or
   reranker), language (default English), maximum size (default
   `models.community.max_size`) and licence. Sorted by downloads; likes
   and last updated are other sorts. 20 results per page from the Hub
   API through Sutradhar's own client, with parameter counts, gated
   flag and card data from the same call; cached for 10 minutes; at
   most one Hub request per second. Each result shows name, author,
   downloads, likes, size, vector size, input window, languages,
   licence, last updated and tier badge. Vector size and window come
   from small configuration files (each under 1 MiB) read only for
   visible results, showing "checking" until known.
3. The check (`sutra models check <repo>[@<revision>]`, or "Check")
   reads repository metadata and configuration files only, never
   weights. Reason codes, each shown in plain English:
   - MF-C01: name not of the `owner/name` form, checked before any
     request, or repository not found;
   - MF-C02: author not in `models.community.allowed_authors` (when
     that list is not empty);
   - MF-C03: gated; asks for an access token; without access the
     verdict is "needs action";
   - MF-C04: no data-only weights (safetensors, single or split with
     an index file, or graph-format weights tract can run);
   - MF-C05: custom code required (`auto_map`, a custom pipeline, a
     source-code file, or an architecture outside the allowlist);
   - MF-C06: architecture not supported by the admitted candle or tract
     version; a reranker must give one score per pair;
   - MF-C07: window too small: embedders need at least the configured
     piece size (`reading.pieces.tokens`, 1,024 by default and never
     fewer, spec 0022) and rerankers at least 8,192. The window is the
     smallest of the declared maximum sequence length, the tokenizer's
     maximum and the architecture's position limit, counted with the
     model's own tokenizer. A model declaring no limit is refused.
     Longer input is refused with an error, never cut short. A piece
     size larger than the active embedder's window is refused with a
     plain message, such as "This piece size is longer than the
     embedder can read", and the old size stays;
   - MF-C08: over the memory budget (below);
   - MF-C09: the index size from 0022's formula for this vector size
     passes 0022's index size budget;
   - MF-C10: no stated licence: "needs action"; added only after the
     owner accepts the stronger warning;
   - MF-C11: English not declared and not multilingual;
   - MF-C12: a configuration file over 1 MiB, or a download over
     `models.community.max_size`.
   Also shown, without refusing: the vector size beside the active
   index's, with "full index rebuild needed" and a rebuild estimate from
   a sample of the site's text in the model's own tokens; an estimated
   processor latency per piece; and the licence, with an accept step
   before any download. The verdict (Compatible, Not supported or Needs
   action, with every reason) is stored with the revision and a SHA-256
   of the metadata used.
4. Memory estimate: parameters from safetensors metadata, or from the
   graph file size; resident memory is parameters times 4 bytes times
   1.2, plus working memory for one batch at the full window from Dell
   measurements of the same family. It must fit the role's row in the
   memory table, since the candidate replaces the active model. Latency
   uses this PC's speed from the Verified self-tests. Both are labelled
   estimates.
5. Adding (`sutra models add <repo>[@<revision>]`):
   - a passing check on the same revision within 24 hours is needed; if
     the current revision moved, the check runs again and the owner
     confirms;
   - the owner accepts the licence and, for a gated model, gives an
     access token. With no stated licence, a stronger warning says no
     permission is shown and use is the owner's own responsibility; the
     acceptance is audited;
   - only weights in the chosen format (with any part index),
     configuration, tokenizer files, licence and model card are fetched.
     Never: `.bin`, `.pt`, `.pth`, `.ckpt`, `.pkl`, `.pickle`,
     `.joblib`, `.h5`, `.msgpack`, `.npy`, source-code files, compiled
     libraries or archives;
   - downloads go to `staging/`, resume, show bytes, and are checked
     against the Hub's published LFS SHA-256 (large files) or Git blob
     id (small files); Sutradhar records its own SHA-256. A file larger
     than declared is cut off and refused;
   - the overlay entry records repository, revision, per-file size and
     SHA-256, format, runtime, measured vector size and window,
     tokenizer fingerprint, licence and its SHA-256, check id and
     licence-accept event; the overlay digest is updated; files go
     read-only;
   - the model is "added (inactive)" and never updated automatically; a
     newer revision is added as a new entry.
6. Community self-test: outputs have the declared size, are finite and
   non-zero, and normalised when the configuration says so; the same
   input twice agrees within 0.000001; a fixed paraphrase pair scores
   above an unrelated pair (embedder), and a relevant passage above an
   unrelated one (reranker). On success the outputs become the expected
   values on this PC; a failure keeps it inactive.
7. Trial (`sutra models trial <repo>`): link reconstruction, hiding an
   existing link and seeing whether each model finds its destination.
   - Sample: up to `models.trial.sample_size` (500) member-written
     internal links in guest-visible posts, never links Sutradhar
     inserted; a recorded random seed, spread across forums in
     proportion to their link counts.
   - Pool: the true destinations plus guest-visible distractor threads
     up to `models.trial.pool_size` (5,000), same seed.
   - Each source context, with the link removed in memory only (the
     site is never changed), goes through 0023's matching and anchor
     functions once per model over the same pool. An embedder candidate
     embeds with pieces cut by its own tokenizer, taking turns with the
     active embedder, which reuses its stored vectors; suggestions
     continue. A reranker candidate reranks the active embedder's top
     50.
   - Results side by side: recall at 1, 3 and 5, anchor agreement (as
     0023 defines it), p95 latency per piece, projected time to first
     suggestion and full reindex estimate, each with sample size and
     95% interval, labelled as trial-pool figures.
   - A low-priority durable job with progress and checkpoints, storing
     numbers and ids, never post text. 0017's trial arena reads them.
8. Activation (`sutra models activate <repo>`) needs a finished trial,
   and is allowed even when the candidate scores worse:
   - if the trial projects that the candidate misses the 30-second
     first suggestion where the active model meets it, a warning states
     both times and activation needs the owner's confirmation, which is
     audited; from the switch the health screen says "outside the
     30-second target: community model chosen by the owner", and that
     site's alerts use its own trial figures. Without confirmation
     nothing changes;
   - embedder: the screen first states that new suggestions pause until
     the rebuild ends, with the estimate; 0022 builds a whole new
     generation cut with the candidate's tokenizer, the candidate
     running alone, posts written or edited meanwhile read into it; one
     atomic change switches model and index, and suggestions resume;
   - reranker: the switch follows the self-test; each suggestion records
     which reranker scored it;
   - before the switch, cancelling reloads the active model, resumes
     suggestions and has posts written meanwhile read by it; the partial
     generation is kept for resuming or removed on request;
   - after the switch the previous model and index are kept for
     one-click rollback until the next activation, with their disk use
     shown; rollback re-reads only posts read since the switch;
   - `sutra models remove <repo>` deletes an inactive community model's
     files and overlay entry; the active and previous ones cannot be
     removed.
9. Audit: one event for each of check, licence accept, no-licence
   warning accepted, access token supplied (the fact, never the value),
   download start, finish or failure, verification, self-test, trial
   start and result, slower-target warning confirmed, activation start,
   switch, cancellation, rollback, removal and each settings change.
   Fields: time, actor, action, repository, revision, result code;
   identifying details kept apart from the minimal event.
10. Community rows carry the badge and the same status, latency, error
    and index-match reporting as Verified rows; a no-licence model shows
    "no stated licence: accepted by the owner on <date>". The project's
    nightly run verifies only the Verified tier; the owner's PC checks
    every installed model daily.
11. With `models.community.enabled` false, "Add model" is hidden,
    community commands are refused and there is no Hub contact for
    community models; an active community model keeps running until the
    owner rolls back, with a banner. A PC without internet may add a
    community model through `sutra models bundle <repo>@<revision>` made
    on a connected machine, carrying the recorded Hub metadata so the
    same check runs offline.

### No retraining of the transformer models

1. The embedder and the reranker are never fine-tuned or retrained.
   This rests on the owner's answer Q-LB03: "No fine-tuning of the
   transformer models themselves on this hardware". Sutradhar has no
   job that fine-tunes them anywhere, so a request to fine-tune either
   finds no such job and is refused (0056, AT-LB25).
2. Learning from moderators' decisions happens only in the small
   networks of 0056 ("Small neural networks: trained in ganit, served
   by the service"): the embedding adapter of job 273 and the neural
   classifier heads. They are trained in ganit on Dell, take their
   labels from 0053, and are shadow-tested, activated and rolled back
   as ganit outputs, never as entries in `models.lock`.
3. The entity model is our own and is trained in the Dell lab (0026);
   it ships as a Verified model in `models.lock`.
4. Training data for the small networks: an authorised export of
   decisions to Dell, carrying the guest-visible text needed. Eligible
   only if source and destination were guest-visible when decided and
   still are at export. Never included: moderator identities, authors'
   account details, private messages, analytics and staff-only content.
   Training starts within 7 days of export. Training data, manifests
   and reports stay on Dell. An erasure that touches a training
   decision purges it from the lab and flags every network trained on
   it; a flagged network stays in use and is replaced by its next
   training within 30 days.
5. The quality judge's grading model (0057) is pinned in `models.lock`
   under the `judge` role in item 2 of "Tiers and roles". No matching
   model is loaded as a judge.

### Stopping and disabling

1. Stopping the service (from the app, or by systemd): it stops taking
   work; running jobs save checkpoints within 60 seconds; the model host
   unloads its models; workers exit; then the service exits. If it has
   not stopped after 90 seconds, systemd stops it forcibly and the next
   start recovers from checkpoints (MF-E17).
2. A site whose add-on is disabled, or that the owner pauses in the app,
   gets no work and no new suggestions; everything is kept (model files,
   overlay lock, vectors, index, reading and suggestion data,
   checkpoints, settings, audit). The app shows "Stopped for this
   site". Re-enabling verifies and loads models, resumes jobs, and 0022
   catches up posts written or edited meanwhile.

### Every store and what removal does

This part owns the table for 0016, 0017, 0022, 0023 and itself.
Removing a site, or uninstalling Sutradhar, deletes embeddings, the
index and model files always, with no keep option and no export.

1. Always deleted, no export, no keep option:
   - 0022: every vector (pieces, extras, titles, summaries, running
     sums, the first-pass store), vector segment files, row
     tables, removed-row lists and index generation records;
   - 0023: phrase vectors used for anchors;
   - this part: model files of every tier, previous revisions, staging
     downloads, verified markers, overlay locks and trial vectors.
2. Deleted without export, as temporary copies or short-lived
   telemetry: 0016's live progress state; 0017's visual caches, its
   cache of scored candidates and judged results, and its prepared
   figures; sampled metrics and traces kept by the service.
3. Export offered, then kept or deleted as the owner chooses:
   - 0022: reading records (per-item state and skip reasons, language,
     readable-text copies, piece and extras text, entity and
     noun-phrase spans, existing-link records),
     checkpoints and the backlog cursor;
   - 0023: suggestion records with states and scores, reranker scores,
     review decisions with rejection and removal reasons, rejected
     candidates, anchor candidates and suppressed connections;
   - this part: check and trial results, licence acceptances, accepted
     warnings, audit events, activation and rollback history;
   - 0016: progress history and the audit of each cancellation;
   - 0017: visual datasets (projection coordinates, layout positions,
     clusters and labels, weekly snapshots, daily reports, the error log
     and the feed buffer);
   - settings and settings history of every module.
4. Never removed: links already published in posts, the site's own
   admin log (which receives the deletion receipt), and Dell lab data,
   which follows its own rules.
5. Removal steps, one resumable durable job with progress: stop work as
   above; delete the always-deleted and no-export items only inside the
   recorded data directory, never following symbolic links, refusing
   any path outside it, counting files and bytes; export, keep or delete
   the rest as chosen; record a deletion receipt (counts, bytes, time)
   in the site's admin log. Files are deleted, not wiped. An interrupted
   removal continues when run again (MF-E18), and the screen shows what
   remains. The confirmation screen lists the deleted items with sizes,
   says plainly they have no keep option or export, offers export and
   keep-or-delete for the rest, and states that published links remain.
6. Every store the code registers must appear in this table.

### Security

1. The model's own code never runs: the loader takes no option to run
   repository code, and configurations asking for it are refused before
   any loader sees them (MF-C05, MF-E06). Reasons: such code would run
   with the service's rights, reading site content, keys and data and
   reaching the network; checksums fix files, not behaviour; nobody can
   inspect it at the click of a button; data-only formats leave every
   supported architecture usable.
2. Only allowlisted file types are fetched, imported or loaded: safetensors,
   JSON and text configuration, tokenizer files, SentencePiece models, and graph
   files with their external weights. Every file's whole content is inspected in
   the same pass as its checksum: pickle streams, ZIP archives holding a
   `data.pkl`, executable or library headers, and the same patterns nested in
   other containers are refused (MF-E05).
3. No production code calls a loader that can run code from a file; a
   static check enforces this. Graph-format models register no custom operator,
   use only standard operator sets, and keep external weights inside
   the folder; this applies to Verified and community models alike.
4. Network: only the fetcher may reach the network, only to the
   official Hub hosts over HTTPS with certificate checks, and only when
   the owner starts install, update, bundle creation, or a community
   search, check or add; never in the background, never to look for
   updates. It sends the search words typed, repository names, file
   requests, any access token, and normal connection data with a
   neutral user agent; never site content, member data, counts or
   settings. The model host has no outbound network. The Hub client
   and this route need admission and privacy review (endpoints, data
   sent, the Hub's own logging, proxies, failures); the manual tells
   the owner what the Hub can see.
5. Licences: Verified licences and data terms live in admission
   records. Community licences are shown in full and accepted before
   download; the record holds an owner reference, time, repository,
   revision, licence id and licence-text SHA-256, with identifying
   details kept apart and erasable. For gated models the publisher's
   terms are accepted on the Hub by the token's account; Sutradhar
   records only that access was granted.
6. Access tokens: typed into a masked field, or read from a hidden
   prompt or `--token-stdin`; never an argument; held in memory only for
   the job that needs it, passed to the fetcher through a private pipe,
   not an environment variable; never written to logs, audit, errors,
   metrics, progress, diagnostics bundles, URLs or disk. Every log and
   bundle path scrubs anything shaped like a Hub access token.
7. Trials use guest-visible content only; member text
   appears only on the owner's screens. A community model cannot run
   code but can rank badly; every link still needs a moderator, and the
   trial, badge, rollback and audit log limit the harm. Repository files
   and logs never name the owner's sites or paths.

### Failures

Each has a plain-English message, an error code and a manual link.
Model failures never touch the sites' pages.

- MF-E01 network or Hub failure: up to `models.download.retries` (5)
  attempts with growing pauses; staged parts kept; then failed,
  resumable.
- MF-E02 pinned revision or file gone upstream: stop and offer the
  offline bundle; never fall back to another revision.
- MF-E03 size or SHA-256 mismatch: delete, fetch once more; a second
  mismatch fails the model with a tampering alert.
- MF-E04 not enough disk: refused in preflight; mid-download it stops
  with staged parts kept.
- MF-E05 forbidden content or unsafe archive member: the whole model or
  bundle is refused, with an alert.
- MF-E06 configuration asks for code or an unlisted type: load refused.
- MF-E07 memory: a load that would break the table is refused with the
  reason; a model host ended by the operating system restarts from
  checkpoints; three ends within 10 minutes fail and isolate the
  newest-loaded model.
- MF-E08 self-test failure: failed; suggestions not ready.
- MF-E09 index mismatch: new suggestions blocked, earlier ones
  approvable; rebuild offered.
- MF-E10 installed revision in no lock: never loaded.
- MF-E11 overlay digest mismatch: community models
  refused; an active role they held becomes unavailable and readiness
  "not ready"; alert.
- MF-E12 an installed file fails re-verification: failed; re-install
  offered.
- MF-E13 Hub search or check error, rate limits included: shown, at
  most one retry; fresh cached results still used.
- MF-E14 access token refused or access not granted: needs action; the
  token is discarded.
- MF-E15 rebuild or activation failure: the active model is reloaded,
  new suggestions resume on the old index, the candidate stays
  inactive, the partial generation is kept.
- MF-E16 rollback impossible: refused; the active model is unchanged.
- MF-E17 service not stopped in time; MF-E18 interrupted removal. MF-E19
  and MF-E20 are not used: there is no transformer retraining and no
  site-trained release. Small-network training failures are handled by
  ganit (0056).

### Settings

Applied atomically between jobs, with history; those needing a model
host restart are marked.

- `models.community.enabled`: true. `models.community.allowed_authors`:
  empty (any author). `models.community.max_size`: `auto`, the largest
  size whose estimate fits its row.
- `models.memory_budget`: `auto`, what the memory table leaves for
  models after its other rows; restart marked.
- `models.disk.margin`: 2 GB. `models.download.retries`: 5.
  `models.hub.timeout`: 30 seconds per request.
- `models.trial.sample_size`: 500. `models.trial.pool_size`: 5,000.
- `models.health.latency_factor`: 2.0. `models.health.error_rate`: 1%.
- `models.active.embedder`, `models.active.reranker`: changed only by
  activation, update and rollback.

### Commands

Each uses the same action as its screen, needs the same permission,
shows progress when longer than two seconds, offers `--json` and exits
non-zero on failure. Help text starts with the meaning of the name.

- `sutra models status`, `install`, `bundle <file>`,
  `import <bundle>`, `verify [<id>]`, `self-test [<id>]`,
  `update [<id>]`.
- `sutra models search <words> [--task] [--language] [--max-size]
  [--licence]`, `check <repo>[@<revision>]`.
- `sutra models add <repo>[@<revision>]`: a token only from a hidden
  prompt or `--token-stdin`; the licence accepted at a prompt or with
  `--accept-licence <SHA-256 of the licence text>`.
- `sutra models trial <repo>`, `activate <repo>`,
  `rollback --role <role>`, `remove <repo>`.
- Dell only: `pin`, `lock-check` and the lab commands.

## Limits

1. Memory: everything counts in the service slice, within the memory
   limits in AGENTS.md section 5.6. Moving work into another process
   cannot escape it. Thresholds use unreclaimable memory (what programs hold
   plus kernel memory that cannot be freed), not disk cache, because
   exact scans fill the group with cache. The service also watches
   memory pressure (the share of time tasks wait for memory).
   - Headroom is 15%: the soft line is 85% of the service slice
     (3,825 MB of 4.5 GB; 5,950 MB of 7 GB). The desktop window has its
     own cap (0.5 GB or 1 GB) and is not part of this line.
   - Batches shrink at the soft line or pressure over 10% for 10
     seconds; new work is refused, with the reason, at 95% or 25%;
     normal batches resume below 80% and 5% for 60 seconds. Disk cache
     is still reported.
   - Units: GB is 10^9 bytes and MB is 10^6 bytes, as spec 0014 reads
     them.
   - Recovery after the operating system ends a process is tested.
2. The memory table, in MB, estimates to be replaced by Dell
   measurements. The first column is the default and holds on every
   PC; the second applies only on a PC with 16 GB or more:

   | Row | Under 16 GB | 16 GB or more |
   |---|---|---|
   | Service base, main and model host | about 300 | about 600 |
   | Qwen3-Embedding-0.6B, 16-bit (1,200 weights + working) | 1,300 | 1,400 |
   | Reranker (600 weights + working) (a) | 700 | 900 |
   | Entity model, our own (0026) | about 100 | about 100 |
   | potion-retrieval-32M, during a backlog sweep only | about 150 | about 150 |
   | Index share for a million posts (scan buffer, rows) | 88 | 280 |
   | Reading batches and summaries (0022) | 48 | 128 |
   | Matching and anchors (0023) | 48 | 128 |
   | Progress system and live channel (0016) | 40 | 40 |
   | Visual caches (0017) | 32 | 128 |
   | Visual builds of 64 each (0017): one, or two | 64 | 128 |
   | Queues, messages and the fetcher | 48 | 100 |
   | Metrics registry and span queue | under 32 | under 64 |
   | PostgreSQL, shared buffers and working memory | about 200 | about 600 |
   | Every other part's standing budget (b) | 450 | 600 |
   | Ordinary jobs above idle (c): one, or two at a time | 72 | 192 |
   | Heavy job slot: one scheduled heavy job at a time | 800 | 1,200 |
   | Bedrock provider client (0025), only with paid on | 64 | 64 |

   (a) One pair at a time on a PC under 16 GB. (b) Each set in its own
   specification. (c) Such as erasure or scoring.

   The index share is the scan buffer (0023: 64 MB under 16 GB,
   256 MB otherwise) plus row tables of about 24 MB per 1.5 million
   vectors (0022). Memory-mapped files, such as lexical and search
   indexes, are disk cache the kernel can take back; they are reported
   but are not part of any row.

   The heavy job slot holds one job at a time: ganit's nightly batch
   (0027), a scheduled visual job such as the map projection (0017), or
   another part's heavy job that its specification places here. While
   it runs, the reranker is unloaded and the backlog sweep pauses; a
   new post waits for the reranker to reload, a few seconds, measured.
   The slot is free during a rebuild.

   By these estimates, under 16 GB: everyday about 3,520; during a
   backlog sweep about 3,670; with the heavy job slot in use about
   3,620. With 16 GB or more: about 5,390, 5,540 and 5,690. A rebuild
   adds no row, because the new embedder replaces the serving one. The
   provider client's row counts only while a paid embedder serves, and
   then the local embedder is unloaded; in a fallback both are loaded,
   and the heavy job slot and the backlog sweep wait.
3. The measured peak of every row together must stay under the soft
   line, 85% of the service slice cap (3,825 MB under 16 GB, 5,950 MB
   with 16 GB or more), at the release-tier test, on both sizes of PC;
   a load over its row is refused.
4. Lab: a lab comparison run holds one heavy-job place (AGENTS.md
   section 11), through pankti.
5. Time: the first install downloads several gigabytes, with the expected time
   shown. The full re-hash of about 2 GB adds seconds to tens of seconds per
   model host start, measured. A compatibility check should finish within 10
   seconds plus the Hub's own response time. A default trial may take hours with
   an embedder of about 0.6 billion numbers, at low priority with an estimate. A
   full rebuild with Qwen3-Embedding-0.6B takes one to several weeks per million
   posts (0022's estimate), with new suggestions paused throughout; the screen
   states the estimate first.
6. Disk: active files, one previous revision per role, inactive
   community models, staging, and the previous index for rollback.
7. Hub: at most one request per second per owner session; 10-minute
   cache; 30-second timeout; configuration files under 1 MiB.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Built test first. Unit and fixture tests run with no network; tests
with the real pinned models run on Dell from a local cache keyed by
revision. Tests read the same `models.lock` as the product.

### Unit tests (Rust, no downloads)

- Lock reader and canonical writer: round trip, schema errors, unknown
  fields, branch names as revisions, output line length.
  `lock-check` refuses a file entry missing SHA-256 or size, an unknown
  field, a branch name, or a non-canonical layout; the writer
  reproduces the committed file byte for byte. Every shipped entry has
  id, role, purpose, repository, revision, format, runtime, licence,
  `index_compat`, tokenizer fingerprint, memory, admission, file list
  and self-test.
- Verifier: good file, changed byte, truncated file, extra file, wrong
  permissions, symbolic link; full hash on first load, identity check
  on reload in the same process, full hash in a new process.
- Content inspection with small synthetic files copying real layouts:
  a pickle stream renamed `.safetensors`; a ZIP holding `data.pkl`; a
  container file with such a ZIP about 1 MB in; a source-code file; an
  executable header; and clean safetensors files, which pass.
- Pre-load scan: `auto_map`, custom pipeline, unlisted architecture,
  unlisted pipeline component, graph custom operator set, graph external
  weight path with `..`; a Verified safetensors entry in
  Qwen3-Embedding-0.6B's layout passes.
- Loader: the graph-format path through tract and the safetensors path
  through candle, and `pooling` and `normalise` on tiny synthetic
  models; our own entity model loads.
- Archive reader: link, device, absolute path, `..`, repeated name,
  oversize member.
- Compatibility checker against recorded Hub fixtures; memory estimator
  and window resolver, with property tests (a larger declared limit
  never lowers the resolved window).
- Compatibility rule with the tokenizer fingerprint; readiness and the
  status machine with a fake clock for the one-hour windows.
- Memory thresholds on synthetic statistics: disk cache filling the
  group does not shrink batches; the 85% soft line or pressure over 10%
  shrinks; 95% or 25% refuses new work with the reason; below 80% and
  5% for 60 seconds, normal batches resume.
- Removal classes: every registered store is in the table; a missing
  one fails the test.
- Settings validation and history; command parsing, refusing a token as
  an argument; audit events; token scrubbing. Flows needing an embedder
  use the fake embedder and fake tokenizer.

### Recorded Hub fixtures

- A recorder calls the public Hub API for chosen public repositories,
  strips headers, cookies and account data, and saves JSON with date,
  endpoint, client version, repository and revision. Cases with no
  stable public example are synthetic variants, labelled so. Tests
  replay them through a fake transport; an unrecorded request fails.
- Cases and expected verdicts: no safetensors, MF-C04; custom code by
  `auto_map` and by custom pipeline, MF-C05; unsupported architecture,
  MF-C06; oversize by memory and by `max_size`, MF-C08 and MF-C12;
  vector size mismatch, Compatible with "full index rebuild needed"
  and an estimate; one over 0022's index budget, MF-C09; gated with no
  token, a token lacking access, and a token with access; graph-format weights,
  Compatible; embedding window 512, reranker window 4,096, and no
  window, MF-C07; no licence, Needs action, MF-C10; English not
  declared, MF-C11; both `.bin` and safetensors, Compatible, with only
  safetensors fetched; revision moved between check and add; rate
  limit and server error.
- Recorded file lists are fixtures: `pin` refuses a pickle weight file
  and a container file hiding a pickle archive, and accepts
  Qwen3-Embedding-0.6B's, the reranker's and potion's safetensors.

### Integration tests (Dell)

- Install from a local fixture server, with resume after a cut
  connection, a corrupted byte (refused, deleted from staging, MF-E03,
  never loaded) and a missing file; only listed files at the pinned
  revision are fetched. Bundle creation and import: a matching bundle
  is accepted; one with an extra file, `..`, absolute path, link or
  wrong checksum is refused and installed models are unchanged. A 3 GB
  bundle imports as a stream and resumes after an interruption.
- Real pinned models load offline and pass their self-tests within
  tolerance; outputs moved beyond tolerance fail the model. The
  embedder's pinned outputs match its model card's example scores, and 0022's
  recorded vectors come from that run. Cached files match
  `models.lock`; generation records hold the revision and tokenizer.
  The model host starts with no network; a load needing a missing file
  fails instead of downloading.
- With failing stubs in place of every code-running loader, every
  production model still loads. A static check fails any production
  call that could run model code.
- Health: one row per model with every field and only the listed
  statuses; a missing required model gives "not ready"; p95 or error
  rate over threshold gives "degraded" and stays ready.
- Another revision's or tokenizer's index blocks new suggestions while
  earlier ones stay approvable, readiness says "not ready for
  suggestions", the row shows "index mismatch", and a search with
  another key is refused.
- Update: "update available" shows; nothing downloads before the click;
  no new suggestion during the rebuild, earlier ones approvable; one
  step switch; rollback. With a fault injected at every step, no
  suggestion is ever made from a mismatched index.
- A file changed on disk is caught by the next new process's full
  hash, the daily check, or the reload identity check, failing the
  model with an alert. A revision in neither lock slot is never loaded.
- Community flow with a small public test model on the fixture server:
  search with each filter, sorted by downloads, with every field and
  badge; check (no request beyond the recorded ones, only configuration
  files under 1 MiB); add (overlay holds repository, revision, per-file
  SHA-256 and tokenizer fingerprint, LFS and blob checks, a moved
  revision re-checked); no download without licence acceptance, and the
  record holds licence id, text SHA-256, repository, revision, time and
  owner reference; a no-licence model only after the stronger warning,
  shown on its row; self-test (size, finiteness, determinism, paraphrase
  order; failure keeps it inactive); trial on the 300-thread synthetic
  fixture (recall at 1, 3, 5, anchor agreement, p95, first-suggestion
  and reindex figures for both models on the same seeded sample and
  pool; staff-only and Sutradhar-inserted links never sampled);
  activation with the pause stated first and one-step switch;
  the 30-second warning (only confirmed activation proceeds, audited,
  health line shown); rollback re-reading only posts since the switch;
  removal. Pieces are re-cut with the candidate's tokenizer and never
  exceed its window; an over-long input is refused, never cut.
- After the gated tests, the token's value appears in no file, log,
  audit row, error, progress message, diagnostics bundle or process
  argument. Every step writes exactly one audit event.
- `models.community.enabled` false hides Add model and refuses
  community commands; an allowed-authors list gives MF-C02 to others.
  An overlay edited outside the service is detected and its models
  refused, Verified models unaffected. Each screen action and its
  command give the same result; without "Manage models" both are
  refused. Only an owner-started action contacts the Hub; the model
  host has no network; the nightly run verifies only Verified entries.
- During trial turn-taking a reload does not re-hash, and new and edited
  posts keep priority over trial batches.
- Stopping saves checkpoints and ends within 90 seconds; a paused site
  keeps everything; re-enabling resumes. Removal, interrupted and
  resumed, leaves no vectors, index or model files of any tier,
  previous revision, staging or trial, and none of the no-export
  stores; offers no export for them and export with keep-or-delete for
  every other store; never deletes outside the recorded directories or
  follows symbolic links; published links remain.
- Memory: every row measured inside the capped slice with
  Qwen3-Embedding-0.6B and our own entity model, at both slice caps;
  the measured peak of all rows stays under the 85% soft line on both
  sizes of PC; an exact scan filling disk cache does not shrink
  batches; a load over its row is refused; a process ended by the
  operating system recovers. The check refuses a community model whose
  estimate does not fit its row.
- Lab: a request to fine-tune the embedder or the reranker is refused,
  and a small network's training runs in one heavy-job place. An export
  holds only guest-visible decisions; a fixture post made staff-only
  before export and moderator identities are absent. Training that has
  not begun within 7 days of its export is refused. Erasing content in a
  training set purges it and flags every network trained on it, and a
  flagged network still in use after 30 days raises a failure.
- Every long operation emits start, advance and finish progress events
  (0016's enforcement test covers these jobs). The nightly run includes
  the graph operator and external-weight checks on a graph-format
  fixture model.

### Speed

Each first-party unit (lock reader and writer, verifier and content
inspection, archive reader, pre-load scan, checker, memory estimator,
fetcher, loaders, self-test, trial runner, switch, removal deleter and
the training code) has benchmarks under AGENTS.md 6.8. The fetcher is
measured against a local fixture server. Recorded per model: load time,
unreclaimable memory, p50 and p95 latency per piece or pair at a fixed
batch size, and throughput; also every table row, peak pressure, swap
time between models in a trial, hashing speed, fixture download speed,
check and trial durations, rebuild throughput, and lab peak memory and
duration, each with candidate, lock entry and host profile.

## Changes from the source

- Rewritten as Rust; models run in pure Rust (tract for the graph
  format, candle for safetensors) instead of native model libraries.
- The embedder is Qwen3-Embedding-0.6B, run by candle from its
  safetensors, in place of the earlier embedder. The ready-made entity
  pipeline is replaced by our own pure-Rust entity model.
- The library-specific remote-code flag and loader bans are restated as
  "no model code ever runs" and "no code-running loader is called".
- The Hub is called by Sutradhar's own Rust client, not the Hub's own
  library.
- Work runs on the owner's Ubuntu PC under systemd, not on the forum's
  server or in a container project; the host helper is dropped.
- Owner screens are the Dioxus app; forum administrator steps are the
  owner's, per site, with Cedar permissions instead of site admin
  permissions.
- Bundle upload in chunks through the site add-on is dropped, since the
  site never calls the owner's PC; bundles are imported on the PC.
- Disabling: the site cannot signal the PC, so the owner stops work in the
  app or by systemd, and a disabled add-on stops work for its site.
- Memory limits follow AGENTS.md section 5.6, with PostgreSQL inside
  the service slice; the table fits the smaller PC's cap, and larger
  settings apply only on 16 GB PCs. Separate monitoring allowances are
  dropped.
- Speed follows the 5% median rule; the 20x goal is kept (spec 0007).
- Retraining the embedder and the reranker, and the site-trained tier,
  are dropped: the owner ruled out fine-tuning the transformer models
  on this hardware (Q-LB03, archive `linking/lab-3.md`).
- Health endpoints become Sutradhar Boot health and readiness checks;
  alerts appear in the app, not in site staff alerts.

Source: `docs/specifications/linking/models.md`, `models-2.md`,
`models-3.md`, `models-4.md`, `models-5.md`, `models-6.md`,
`models-7.md`, `models-8.md` and `models-9.md` in the archive
`archive-2026-11-08-old-repo`.
