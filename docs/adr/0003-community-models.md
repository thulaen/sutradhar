# ADR-0003: Community models from Hugging Face, never with remote code

Status: accepted by the owner, 30 October 2026. Requirement: R109. The
owner's exact words are in the
[platform requests](../requirements/sources/platform-requests-2026-10-30.md)
and, for the follow-up answers, the
[model decisions](../requirements/sources/model-decisions-2026-10-30.md).

## Context

ADR-0002 governs the pinned models in `models.lock`. The owner wants forum
administrators to add other models with a point-and-click browser, while
keeping the pinned models as the Verified tier and never running code from
a model repository.

## Decision

### Who decides

The owner approves the models Sutradhar ships in `models.lock` (R027,
R100). Each forum's administrator may choose community models for their
own site (follow-up answer).

### Tiers

- Verified: the models in `models.lock`.
- Compatible: models that pass the compatibility check below.
- Not supported: models that fail it, with the reason shown.

### Models page: Add model

Search the Hugging Face Hub API (`huggingface_hub` `HfApi`) with filters
for task (embedding, reranker), language, maximum size and licence, sorted
by downloads. Each result shows name, author, downloads, likes, size on
disk, vector size, maximum input length, languages, licence, last updated
and its tier badge.

### Compatibility check, before any download

Using `model_info` with file metadata and the configuration files:

- refuse models without safetensors weights, except that ONNX weights
  run through fastembed are also accepted (follow-up answer: ONNX is an
  approved data-only format and fastembed a second runtime for community
  models);
- refuse models whose configuration requires custom code (`auto_map` or a
  custom pipeline), since `trust_remote_code` is never enabled;
- require an architecture supported by sentence-transformers or
  fastembed;
- refuse embedding models that accept fewer than 1,024 tokens and
  rerankers that accept fewer than 8,192, so pieces are never cut short
  (follow-up answer; R101, R103);
- estimate memory from the parameter count and refuse if it exceeds this
  server's model budget, and show an estimated CPU latency;
- compare the vector size with the active index and show the full-rebuild
  time estimated from the forum's post count;
- prompt for a Hugging Face token for gated models;
- show the licence with an explicit accept step.

### Download

Resolve the repository's current commit at selection time; record the
repository id, commit and per-file SHA-256 in the forum's local model lock
(a per-site overlay on `models.lock`); verify every file against the Hub's
LFS metadata during download; store it under the data directory with
restricted permissions; resumable, with progress. From then on the model is
pinned exactly like the defaults and never auto-updated.

### After download

- Run the self-test.
- Offer a trial that runs link reconstruction on a sample of the forum's
  existing internal links with the candidate model against the active one,
  showing recall at 1, 3 and 5, anchor agreement, latency p95 and the
  estimated full reindex time side by side.
- Activate builds the new index in the background while the old one
  serves, switches on completion, and keeps the previous model and index
  for one-click rollback.
- Every step writes to the audit log.

### Health page, settings and command line

- Community models carry a "community, untested by Sutradhar" badge and the
  same status, latency, error and index-match reporting as verified models.
- Settings: `models.community.enabled` (default true),
  `models.community.allowed_authors` (empty means any),
  `models.community.max_size`, and the memory budget.
- Command line: `sutra models search`, `sutra models check <repo>`,
  `sutra models add <repo>`, `sutra models trial <repo>`,
  `sutra models activate <repo>`, `sutra models rollback`.

### Tests

The compatibility checker is tested against recorded Hub API fixtures
covering: safetensors missing, custom code required, unsupported
architecture, oversize, dimension mismatch and gated. The nightly run
verifies only the Verified tier.

## Why `trust_remote_code` is never enabled

- Code from a model repository would run inside the Python service with
  its permissions: it could read forum content, credentials and the data
  directory, and reach the network.
- Checksums pin files, not behaviour: repository code can download or
  generate further code at run time, outside the lock.
- Neither Sutradhar's reviews and tests nor the forum's administrator can
  inspect that code at the click of a button, and it can change with any
  new revision.
- Data-only weight formats (safetensors, and ONNX for fastembed) cannot
  execute code when loaded, so refusing remote code still leaves every
  model in a supported architecture usable.

## Consequences

- A model that needs custom code is shown as Not supported, with the
  reason, rather than run.
- Recorder's notes for the feature specification: the Hub client,
  fastembed, ONNX Runtime, network access from the forum host and the
  licence-accept record need admission and privacy review; the per-site
  overlay follows the same verification and uninstall rules as
  `models.lock` (R106, R108); the trial's sample of existing links uses
  guest-visible content only (R013); the fixtures should also cover ONNX
  weights and input windows below the minimum.
