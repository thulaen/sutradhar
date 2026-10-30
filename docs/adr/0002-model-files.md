# ADR-0002: One lock file governs every model

Status: accepted by the owner, 30 October 2026. Requirements: R098, R106.
The owner's exact words are in the
[model decisions source](../requirements/sources/model-decisions-2026-10-30.md).
Community models are added by
[ADR-0003](0003-community-models.md).

## Context

The product uses several pretrained models (R098). Their files are large,
come from outside the project, carry licences and can change upstream. The
forum must never run a model whose search index was built with another
version, and loading a model must never run code from its repository.

## Decision

### `models.lock`

A single file in the repository, `models.lock`, is the only source of truth
for every model Sutradhar ships (the Verified tier; a forum's community
models are pinned in its per-site overlay, ADR-0003): name, purpose,
official source repository, exact revision, file list with a SHA-256 per
file, size, licence and the index-compatibility version. Setup, tests,
the nightly run and the health page all read it.

### Install

- During add-on setup, an "Install models" step shows each model with its
  purpose, size and licence. One click downloads them from the official
  source at the pinned revision, verifies every file against the SHA-256 in
  `models.lock`, stores them in the service's data directory with
  restricted permissions, and refuses any file that fails verification.
- Loading never executes code from the model repository:
  `trust_remote_code` is false. Only data-only formats are loaded:
  safetensors for Hugging Face models and spaCy's own pipeline format,
  loaded from a directory; never pickle files. The owner's answer first
  said "safetensors only"; the follow-up answer chose data-only formats
  without pickle, which keeps Q163 and allows `en_core_web_trf`.
- The same step runs headless as `sutra models install`. Servers without
  internet access use `sutra models import <bundle>` or a file upload in
  the wizard, verified the same way.
- Models are never auto-updated. When a release pins a new revision, the
  health page shows "update available" with a button. Updating downloads
  the new revision alongside the old one, rebuilds the index with it and
  only then switches over, so the forum never runs a model whose index does
  not match.

### Testing

- Tests use the same `models.lock`.
- Unit tests use a deterministic fake embedder and need no download.
- Integration tests, the evaluation and the nightly run use the real
  pinned models from the local cache; continuous integration caches model
  files keyed by revision.
- A test asserts that the cached files match the checksums in
  `models.lock` and that the index metadata records the model revision it
  was built with.

### Health page

- One row per model: status (not installed, downloading with progress,
  verifying, loaded, degraded, failed, update available, index mismatch),
  revision, time the checksum was last verified, load time, memory in
  use, device, inference latency p50 and p95 over the last hour, error
  count over the last hour and the last error message.
- A "Run self-test" button sends a fixed input through each model and
  compares the result with the expected output recorded in `models.lock`.
- Readiness is "not ready" while a required model is missing or failed;
  "degraded" (still ready, with a warning) when latency or error rate
  exceeds its budget; and it warns when the index was built with a
  different revision. In that case suggestions are blocked and readiness
  reports not ready for suggestions until the index is rebuilt (follow-up
  answers).
- Uninstalling deletes the downloaded model files (R050 as amended).

## Consequences

- One reviewed file changes when any verified model changes.
- Recorder's notes for the feature specification: the first install
  downloads several gigabytes; expected self-test outputs need tolerances
  for small numeric differences across hardware; `models.lock` is a
  lockfile under the source limits (R089); downloads into the project's
  own environments follow the existing approval rule for downloads.
