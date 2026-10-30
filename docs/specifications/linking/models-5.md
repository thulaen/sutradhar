# SPEC-010 part 5: privacy, security and budgets

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in the
[specification decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Part of [SPEC-010](models.md). Requirements: R106, R109, R013, R021 and
R092 as amended, R120, R121, R122, R125, R127, R129, R023, R027, R028,
R043, R051, R052, R067, R072, R075, R090, R119; ADR-0006.

## Privacy and security

### Why `trust_remote_code` is never enabled

Some model repositories include Python code and ask the loading library
to run it (`trust_remote_code`). Sutradhar never allows this, for the
reasons the owner's decision records (ADR-0003):

- that code would run inside the Python service with all its
  permissions: it could read forum content, keys and the data
  directory, and reach the network;
- checksums fix files, not behaviour: the code could download or build
  further code while running, outside the lock;
- neither Sutradhar's reviews nor a forum's administrator can inspect
  that code at the click of a button, and it can change with every
  revision;
- data-only formats cannot run code when opened, so refusing remote
  code still leaves every model of a supported architecture usable.

Enforcement: the value is one constant; a static check fails the build
if any production call passes anything else; configurations asking for
custom code are refused before any model library sees them (MF-C05,
MF-E06).

### Pickle and other executable formats

- Only allowlisted file types are fetched, imported or loaded:
  safetensors weights, JSON and text configuration, tokenizer files,
  SentencePiece models, spaCy pipeline folder files, and ONNX graphs
  with their external weight files.
- Every file's whole content is inspected, in the same pass as its
  checksum, not just its name or first bytes: pickle streams, ZIP
  archives holding a `data.pkl` (PyTorch's own format), executable or
  library headers, and the same patterns nested inside container
  formats such as msgpack, are refused (MF-E05). The check on Dell
  found exactly this case in `en_core_web_trf`, about 1 MB into a
  msgpack file (part 7), which is why `en_core_web_lg` replaced it
  (R120).
- Production code may not call `pickle.load`, `pickle.loads`,
  `torch.load`, `joblib.load`, or `numpy.load` with pickles allowed, nor
  import `dill` or `cloudpickle`; a static check enforces this. A
  library that calls `torch.load` itself cannot be confined by that
  check, which is why such files are refused before loading.
- A test replaces Python's unpickler and PyTorch's loader with stubs
  that fail, then loads every production model; loading must succeed.
- sentence-transformers module types and spaCy registered functions
  must be on reviewed allowlists, because both libraries can otherwise
  import a class named in a configuration file.
- ONNX: no custom operator library is ever registered; operator sets
  must be standard ones bundled with ONNX Runtime; external weight
  paths must stay inside the model folder. These checks apply to the
  Verified bge-m3 export (R120) as to community models.

### Archives

Bundles are read as streams. Only regular files listed in a lock are
accepted. Links, devices, absolute paths, `..` parts, repeated names and
members larger than declared are refused (MF-E05).

### Network access

- Who: only a separate fetcher process may reach the network, and only
  the official Hub hosts over HTTPS with certificate checks. The
  model-host process and the rest of the service have no outbound
  network beyond the local link to PHP. The fetcher stays inside the
  service's memory cap (R021).
- When: Verified install, update and bundle creation; community search,
  check and add, all started by an administrator. Never in the
  background and never to look for updates.
- What is sent: the search words an administrator types, repository
  names, file requests, an access token for gated models, and normal
  connection data (server address, time, a neutral user agent naming
  only the libraries). Never forum content, member data, counts or
  settings. Library telemetry is switched off.
- `models.community.enabled = false` stops all Hub contact for
  community models; Verified models can then come from an offline
  bundle.
- Admission and privacy review before first use (R027, R067):
  `huggingface_hub`, fastembed, ONNX Runtime (which bge-m3 needs,
  R120) and this network route; `en_core_web_lg` needs its own model
  admission record.
  The review covers endpoints, data sent, the Hub's own logging
  (outside Sutradhar's control), proxies and failure modes. The manual
  tells forum owners what the Hub can see.

### Licences

- Verified models: each admission record holds the licence and any
  data terms, such as research-only training data (R099).
- Community models: the licence is shown in full and must be accepted
  before download. The record holds an administrator reference, time,
  repository, revision, licence identifier and the SHA-256 of the
  licence text. Identifying details are kept apart from the minimal
  audit event and can be erased (R052).
- A repository with no stated licence: allowed after a stronger
  warning that the administrator accepts, recorded in the audit log
  with the repository and revision; its row then says so (MD-10, R125;
  part 3).
- For gated models, the publisher's terms are accepted on the Hub by
  the account that owns the access token; Sutradhar records only that
  access was granted.

### Access tokens

- Typed into a masked field or read from a hidden prompt or standard
  input; never taken from a command-line argument, which other users of
  the host can see.
- Held in memory only for the job that needs it (MP-08) and passed to
  the fetcher through a private pipe, not an environment variable.
- Never written to logs, audit, error messages, metrics, progress
  messages, diagnostics bundles, URLs or disk. Every log and bundle
  path scrubs anything shaped like a Hub access token (R051).

### Files, content and agents

- Only the service account can read model files: folders `0700`,
  verified files `0400`. The overlay digest detects outside edits.
- Trials and retraining use guest-visible content only (R013),
  supplied through PHP (R043). Trial results store numbers and ids,
  never text; member text appears only on admin pages (R111).
- Retraining leaves out moderator identities, keeps data on Dell,
  follows erasure (R052) and never publishes site-trained models.
- A community model cannot run code but can rank badly. Every link
  still needs a moderator's approval (R009); the trial, badge, rollback
  and audit log limit the harm.
- Repository files and logs never name the owner's forum or paths
  (R075). Agents downloading model files into Dell or
  continuous-integration caches follow the approval rule for downloads
  (ADR-0002); all work is on Dell, and nothing runs on the Windows
  computer, which only runs the app (R072, R119).

## Performance and resource budgets

### What counts against the service cap

- The service and all its processes (main, model host, fetcher) run
  in one container, whose operating-system memory group is capped at
  6 GB, or 8 GB once Dell measurements confirm the estimate (R021 as
  amended, R121, R129). Moving work into another process cannot escape
  it.
- The bundled Prometheus server and OpenTelemetry collector (R127) run
  in their own containers with their own caps: a separate, declared
  allowance on top of the service's cap, published in the resource
  figures (R121, ADR-0006). By estimate, to be measured on Dell, about
  512 MB for Prometheus and 128 MB for the collector. They are not
  rows of the table below and cannot take memory from it.
- The thresholds use memory that cannot be reclaimed: what programs
  hold for themselves plus kernel memory that cannot be freed.
  Disk cache (copies of recently read files, which the system drops the
  moment memory is needed) is left out, because exact scans of the
  vector files fill the group with it. The service also watches memory
  pressure: the share of time tasks wait for memory, as Linux reports.
- Batches shrink when unreclaimable memory reaches 90% of the cap or
  pressure exceeds 10% over 10 seconds. New work is refused, with the
  reason, at 95% or 25%. Normal batches resume below 85% and 5% for
  60 seconds. Disk cache is still measured and reported (SPEC-008).
- Recovery after the operating system ends a process is tested,
  because a cap cannot itself guarantee graceful behaviour (R023).

### The memory table

Unreclaimable memory in MB (1 GB is 1,024 MB). Weights come from file
sizes or published parameter counts; everything else is an estimate or
another specification's allowance, all to be replaced by Dell
measurements. Rows follow the owner's answers to MD-01 and MD-02
(R120).

- Service base, Python and its libraries in the main and model-host
  processes (SPEC-010): about 700.
- bge-m3, full precision through ONNX Runtime: 2,270 of weights (the
  official ONNX export's size, R120) plus 230 working: 2,500
  (SPEC-008).
- gte-reranker-modernbert-base: about 600 of weights plus 300 working
  with memory-saving attention at 8,192 tokens, pairs one at a time if
  needed: 900 (SPEC-009 measures it).
- spaCy pipeline, `en_core_web_lg` (R120): about 700, mostly its
  617 MB word-vector table (SPEC-008).
- potion-retrieval-32M: about 150, only during a backlog sweep.
- Index share: the 256 MB scan buffer (SPEC-009) plus row tables of
  about 24 MB per 1.5 million vectors (SPEC-008): about 280 for a
  million posts.
- Reading batches and summaries (SPEC-008) and matching and anchors
  (SPEC-009): 128 each, held until those drafts state figures.
- Progress system and live channel (SPEC-011): 40.
- Visual caches (SPEC-012): 256. Visual builds: four of 64, so 256.
- Map projection (SPEC-012): 1,024 while it runs; it pauses during
  rebuilds.
- Queues, messages to and from PHP, and the fetcher: 100.
- Metrics registry and batch span queue in the service (R127): under
  64, ADR-0006's estimate.

Totals against the lines at 6 GB (shrink 5,530; refuse 5,837; cap
6,144):

- everyday, with nothing optional running: about 6,050, already past
  the refuse line;
- during a backlog sweep: about 6,200, over the cap;
- with the map projection: about 7,080 to 7,230, over the cap.

So, by these estimates, the owner's model set at full precision plus
the visual layer does not fit 6 GB. (The owner was asked MD-03 on the
earlier totals of 5,980 and 7,150; the telemetry row of R127 came
later.) Under MD-03's answer (R121) the cap becomes 8 GB if Dell
measurements confirm the estimate; until then 6 GB stands. At 8 GB
(shrink 7,373; refuse 7,782; cap 8,192) the largest total above, about
7,230, stays under the shrink line, at about 88%. Under MD-04's answer
(R122) a rebuild adds no row: new suggestions pause and the new
embedder takes the serving one's place, alone, until the switch.
`models.memory_budget` (`auto`, MP-04) is what the table leaves for
models after the other rows.

### The 9 GB lab ceiling

- A retraining job's whole process tree runs in one 9 GB memory group,
  scheduled alone among heavy lab work (R092 as amended).
- Fully fine-tuning bge-m3 with the usual optimiser needs roughly four
  copies of its weights (weights, gradients and two optimiser values
  each), about 9.1 GB before working memory, so it cannot fit; hence
  LoRA (MP-14). Fully fine-tuning the reranker needs roughly 2.4 GB plus
  working memory for 8,192-token inputs with gradient checkpointing,
  and is expected to fit. Both are measured before a run is accepted.

### Time and disk

- The first install downloads several gigabytes; the page shows the
  expected time from the measured speed.
- The full re-hash of about 3.5 GB happens once per model-host process
  (MP-05), adding seconds to tens of seconds per start, measured.
- Disk: active files, one previous revision per role, inactive
  community models, staging, and the previous index kept for rollback.
- A compatibility check should finish within 10 seconds plus the Hub's
  own response time.
- A default trial embeds up to 5,000 threads' pieces; with a model as
  large as bge-m3 on a CPU this may take hours, at low priority with
  progress and an estimate.
- A full bge-m3 rebuild takes about as long as the first run: one to
  several weeks per million posts (SPEC-008's estimate), and new
  suggestions pause for all of it (MD-04, R122). The update and
  activation pages state the estimate before the administrator starts.
- Monitoring data on disk (R127): the Prometheus server's 15-day
  metrics store and 7 days of sampled traces in Sutradhar's database;
  their sizes are measured on Dell and shown with the other disk use.

### The SPEC-007 20× gate

- Applies to every first-party executable unit built from this
  specification: lock reader and writer, verifier and content
  inspection, archive reader, pre-load scan, compatibility checker,
  memory estimator, fetcher, loaders, self-test, trial runner, switch,
  uninstall deleter and the training pipeline's own code.
- Baselines for new code are the simplest correct implementations, for
  example a plain read loop with the standard hash library, sequential
  per-file download, and a brute-force trial over the whole pool.
- Most units are bound by disk, network or third-party model libraries,
  so a reviewed limit report (result B) is the likely honest result for
  the verifier, fetcher, loaders and trial. The checker, lock parsing
  and archive checks may reach 20× against naive baselines. Inference
  inside third-party libraries is not a first-party unit; the code
  around it is measured through the representative operation.
- Benchmarks run in the Dell runner sandbox without network; the
  fetcher is measured against a local fixture server.

### What is measured and recorded

Per model: load time, unreclaimable memory, p50 and p95 latency per
piece or pair at a fixed batch size, and throughput. Also: every row of
the memory table, the monitoring containers' memory against their caps
(R121), peak memory pressure, swap time between models when they take
turns in a trial, hashing speed, fixture download speed, check and trial
durations, rebuild throughput (with SPEC-008), lab peak memory and
training duration, each with the exact candidate, lock entry and host
profile.
