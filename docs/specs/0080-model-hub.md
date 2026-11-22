# 0080: Model Hub

The Model Hub is the screen and the service where a user finds, downloads,
checks, loads, swaps and removes models, inside Sutradhar. It is like a
simple model library: a short shelf of recommended models at the top, an open
search below, and plain-English badges that say whether a model fits the PC.
It follows ADR-0021: ONNX and GGUF models, a small installer, an offline
starter bundle, hash-checked downloads, safe file formats only, and every
model load decided by the scheduler. Spec 0024 keeps what a model must prove
before it is admitted (files, hashes, quarantine and the pinned set); this
spec owns how a person finds and uses models. Spec 0078 owns the memory
numbers. Spec 0030 tracks every downloaded file in the file ledger.

Example: Dev opens the Hub on an 8 GiB laptop. The shelf shows the embedding
model, the reranker and a small language model. Each has a badge: "Fits
easily", "Tight fit" or "Too big for this PC". Dev clicks Download on the
embedding model. A progress bar shows the speed, a pause button works, and when
the file's checksum matches, the model appears as "Ready". Dev then searches
the open catalogue for a larger language model, sees "Too big for this PC", and
decides not to override it.

## What it does

### Words used here

- **Model**: a set of files that a loader can run: ONNX, safetensors or GGUF.
- **Shelf**: the curated list of recommended models at the top of the Hub.
- **Open search**: a search of a public model catalogue by name, model ID or
  address.
- **Fit badge**: "Fits easily", "Tight fit" or "Too big for this PC".
- **Revision**: the exact version of a model, named by a long identifier.
- **Offline starter bundle**: a separate download holding the starter models,
  for a PC with no internet.
- **Heavy model**: a model that needs more than 1 GiB when loaded (spec 0078).

### The shelf

1. The shelf holds only models with permissive licences (MIT, Apache-2.0,
   BSD), each pinned to an exact revision and file hash.
2. At the start of version 1 the shelf holds the target stack of ADR-0020:
   BGE-M3 (revision 5617a9f61b028005a4858fdac845db406aefb181, MIT) and the
   BGE reranker v2 m3 (revision 953dc6f6f85a1b2dbfca4c34a2796e7dde08d41e,
   Apache-2.0), plus one small language model and one small embedding model
   chosen from the dossier. These revisions come from ADR-0020 and are checked
   against the model maker's page in the research dossier before release.
3. Changing the shelf is a documentation change with a decision record, not a
   setting.
4. A model on the shelf has a one-line plain description, its size on disk, its
   size when loaded, its licence and what it is for (matching, reranking or
   writing help).

### Open search

- A search box takes a name, a model ID or an address. Results show name,
  maker, size, format, licence and a fit badge.
- Search needs the internet and is off in offline mode. It sends only the text
  the user typed.
- A result with a non-commercial, unknown or custom licence shows a warning and
  asks for an extra confirmation before download. Nothing is blocked outright.
- Only models in safe formats can be downloaded (see "Safe formats").
- Importing a local file follows the same checks.

### Safe formats

1. Allowed: ONNX, safetensors and GGUF files, plus the small text files that
   describe them (configuration, tokeniser files).
2. Refused: any format that can run code when loaded, such as Python pickles,
   and any file in an archive that unpacks outside the model's folder. The
   refusal says why in plain English.
3. Every file is checked by type from its contents, not its name.
4. A model with a bundled script or custom loader code is refused.
5. A model that passes is placed in quarantine and admitted under spec 0024
   before it can load.

### Downloading

- Downloads can pause and resume, including after a restart, and work from a
  local copy once saved.
- Each file is checked against its hash. A shelf model has a pinned hash; an
  open-search model records the hash the source gave and the hash we computed,
  and warns if the source gave none.
- A download is a promise of disk space (spec 0078). With too little disk it
  waits or is refused, naming the space needed.
- A download goes to a temporary name and moves into place only when whole and
  checked. A partial file is never usable.
- All downloads go in the `models/` folder and into the file ledger. The
  setting `hub.download_dir` may move that folder, but only inside the
  owner's data folder, which satisfies spec 0030. A path that resolves
  outside that folder, by a link or by `..`, is refused.
- Where a model came from, its licence and its hash are saved with it and shown
  in the Hub.

### Fit badges

The badge uses spec 0078's formula: a model's need is its file size times 1.2
plus the context estimate. "Fits easily" is at most 40% of the free burst pool,
"Tight fit" fits, and "Too big for this PC" does not. With graphics memory
(Vulkan), the badge also shows how much could be offloaded. A power user may
override "Too big", up to the ceiling, with a warning that explains what may
slow down or stop.

### Loading, unloading and the scheduler

- Load and Unload are buttons, but the scheduler decides when (spec 0078). A
  load that has to wait shows "Waiting for 2 GB of memory to free up".
- On a budget below 10 GiB at most one heavy model is loaded.
- Idle models unload after a time the user can change.
- A model currently used by a running job cannot be deleted; the screen says
  which job.
- Local language models never serve live ranking.

### Swapping an embedding model

1. Each embedding model has its own index. Swapping never overwrites another
   model's index.
2. After a swap the Hub says how many pages need new vectors, how long that
   may take and how much disk it needs, and asks before it starts.
3. Until the new index is complete, ranking keeps using the previous complete
   index, so suggestions never stop.
4. A model that is the rollback set of the current champion (spec 0077) cannot
   be removed.

### Delete and move

Delete removes the files and their ledger entries after a confirmation that
names the size freed. A model on an external drive that is unplugged shows
"Drive not connected"; its jobs pause (spec 0038). A model can be moved to
another location through the storage wizard; its hash is checked again after
the move.

### MCP and scripts

The Hub's list, search, download, load, unload and delete actions are exposed
over local MCP, each with the same checks. An agent may start a download only
with the owner's live confirm when the model is not on the shelf.

### Offline starter bundle

A separate download holds the starter models, their hashes and a signed
manifest. Installing from it needs no internet. The same checks apply as for a
download. The bundle is built by the release pipeline from the shelf.

### Screens

WCAG 2.2 AA. The Hub has three views: Shelf, Search and Installed. Each model
card shows name, purpose, size, licence, source and badge; actions are buttons
with plain labels; progress is a bar with text; every badge has text, not only
colour; keyboard and screen-reader use is complete. Empty, loading, offline,
error and drive-missing states are written out.

### Data

- `model_files`: model, file, size, hash, source, revision, licence, place,
  state (downloading, quarantined, admitted, loaded, removed).
- `model_downloads`: model, bytes done, speed, pause state, last error.
- `model_overrides`: who overrode a badge, when and why.

### Settings

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `hub.search_enabled` | on or off | on | two values | at once |
| `hub.confirm_nonpermissive` | on or off | on | two values | at once |
| `hub.download_dir` | folder | `models/` in the home folder | a writable folder inside the owner's data folder | between downloads |
| `hub.max_parallel_downloads` | whole number | 1 (proposed) | 1 to 4 (proposed) | between downloads |
| `hub.bandwidth_limit_mbit` | whole number, 0 is none | 0 (proposed) | 0 to 10,000 (proposed) | at once |
| `hub.stall_timeout_s` | seconds | 60 (proposed) | 10 to 600 (proposed) | between downloads |
| `hub.max_search_results` | whole number | 50 (proposed) | 10 to 200 (proposed) | at once |
| `hub.min_free_disk_gib` | whole number | 2 (proposed) | 1 to 100 (proposed) | between downloads |
| `budget.model_idle_unload_s` | seconds | 300 | 30 to 86,400 | between jobs |

### Failure codes

SUT-4200 file type refused; SUT-4201 archive path escape refused; SUT-4202 hash
mismatch, file discarded; SUT-4203 not enough disk; SUT-4204 licence
confirmation declined; SUT-4205 source unreachable; SUT-4206 model in use,
cannot delete; SUT-4207 model needs more than the ceiling; SUT-4208 drive not
connected.
A download with no progress for `hub.stall_timeout_s` seconds is stopped
(proposed) and resumes or fails with SUT-4205. Search returns at most
`hub.max_search_results` results (proposed). Before a download starts, the
Hub checks that free space covers the file plus `hub.min_free_disk_gib`
(proposed), and refuses with SUT-4203 if not. A failure leaves no
half-installed model.

### Security and privacy

- Downloaded files are untrusted until admitted. Nothing runs from a model
  folder except the pinned loaders.
- Search and download send no personal data; the Hub says what it sends.
- Open-search results from the internet are data, never instructions.
- Licence and source are recorded so the SBOM and notices page can list them.

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| MH-01 | The shelf MUST hold only permissively licensed models, each pinned by revision and hash. | UT-MH-01 |
| MH-02 | Only ONNX, safetensors and GGUF files and their text descriptors MUST be accepted, checked by content. | UT-MH-02, FZ-MH-01 |
| MH-03 | A download MUST be resumable, hash-checked and moved into place only when whole and valid. | UT-MH-03, FI-MH-01 |
| MH-04 | A non-permissive, unknown or custom licence MUST require an extra confirmation and MUST NOT be blocked outright. | BD-MH-01 |
| MH-05 | A fit badge MUST follow spec 0078's formula and thresholds. | UT-MH-04 |
| MH-06 | An override MUST stop at the ceiling and MUST be recorded. | UT-MH-05 |
| MH-07 | Swapping an embedding model MUST keep each model's index separate and MUST ask before rebuilding. | BD-MH-02 |
| MH-08 | Ranking MUST keep using the previous complete index until the new one is complete. | IT-MH-01 |
| MH-09 | Every model file MUST appear in the file ledger and be removed with its entries. | IT-MH-02 |
| MH-10 | A model in use or in the champion's rollback set MUST NOT be deleted. | UT-MH-06 |
| MH-11 | The offline bundle MUST install with no internet and the same checks. | IT-MH-03 |
| MH-12 | Hub actions over MCP MUST apply the same checks and confirms. | IT-MH-04 |
| MH-13 | Local language models MUST NOT serve live ranking. | IT-MH-05 |

## How it is tested

Designed, not run.

- **Unit (UT-MH):** a search capped at `hub.max_search_results`, with one
  value just inside and one just outside; every format check with real
  small fixtures (one allowed and one refused for each format), a pickle
  refused, an archive with a path escape refused, hash match and
  mismatch, the three badge thresholds with one value just inside and one
  just outside, override at and above the ceiling.
- **Fuzz (FZ-MH):** malformed model headers and archives never crash or load.
- **Fault injection (FI-MH):** a download stalls past
  `hub.stall_timeout_s` and is stopped; the free-space check refuses a file
  that does not fit; the network drops mid-download, the disk fills,
  the app is killed during the move into place, a drive disappears.
- **Behaviour (BD-MH):** the Dev scenario above; a non-commercial licence
  shows the warning; swapping the embedding model shows the rebuild cost.
- **Integration (IT-MH):** a swap while jobs run; the ledger after a delete;
  an offline install; an MCP download with and without the live confirm.
- **Accessibility:** every state of every screen on both systems.
- **Measurement:** time to download and verify the shelf models on a reference
  connection; memory while loading the largest shelf model on 8 GiB.

## Changes from the source

The source (spec 0024) held model admission only and assumed a fixed pure-Rust
set. The Hub, open search, safe formats, fit badges, swapping and the offline
bundle are new. Spec 0024 keeps admission, quarantine and integrity; this spec
owns the user's way to find and use models.
