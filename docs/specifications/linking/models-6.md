# SPEC-010 part 6: testing and acceptance cases

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in the
[specification decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Part of [SPEC-010](models.md). Requirements: R087, R054, R058, R072,
R110, R119, R120, R121, R122, R125, R129; ADR-0002 and ADR-0003
(testing sections).

## Testing

### Test-first

Every behaviour here is built red, green, refactor (R087): a meaningful
test is written and seen to fail for the intended reason before the
code exists. Commits declare `TDD: test-first`, and the runner records
the red and green runs (SPEC-007 part 2). The tests that the owner's
answers to MD-01 and MD-02 change (R120), listed with those decisions
in part 7, are applied below.

### Where tests run

- Unit and fixture tests: the Dell runner sandbox, or isolated cloud
  workers using public synthetic fixtures. No network in either.
- Integration tests with the real pinned models: Dell, from the local
  model cache keyed by revision; continuous integration caches model
  files by revision (ADR-0002).
- All work is on Dell; nothing runs on the Windows computer, which only
  runs the app (R072, R119). Tests read the same `models.lock` as the
  product (ADR-0002).

### Unit tests (no downloads)

- Lock reader and canonical writer: round trip, schema errors, unknown
  fields, branch names instead of revisions, line length of the output.
- Verifier: good file, changed byte, truncated file, extra file, wrong
  permissions, symbolic link; full hash on first load, identity check
  on a reload in the same process, full hash again in a new process.
- Content inspection, with small synthetic files that copy the real
  layouts: a pickle stream renamed `.safetensors`; a PyTorch ZIP with
  `data.pkl`; a msgpack container with such a ZIP about 1 MB in, as in
  `en_core_web_trf`; a `.py` file; an executable header; and clean
  msgpack files like `en_core_web_lg`'s, which must pass.
- Pre-load scan: `auto_map`, custom pipeline, unlisted module type,
  unlisted spaCy function, ONNX custom operator set, ONNX external
  weight path with `..`; a Verified ONNX entry in bge-m3's layout (a
  graph with an external weight file) passes (R120).
- Loader: the Verified ONNX path through ONNX Runtime, and the
  `pooling` and `normalise` step on a tiny synthetic ONNX graph (first
  token's vector, normalised); spaCy loads with PyTorch absent (R120).
- Archive reader: link, device, absolute path, `..`, repeated name,
  oversize member.
- Compatibility checker against the recorded Hub fixtures below;
  memory estimator and window resolver, with property tests (a larger
  declared limit never lowers the resolved window).
- Compatibility rule, including the tokenizer fingerprint; readiness
  and the status machine, with a fake clock for the one-hour windows.
- Memory thresholds: synthetic memory statistics where disk cache fills
  the group (no shrinking), unreclaimable memory at 90% and 95%, and
  pressure at 10% and 25%.
- Uninstall classes: every store the code registers appears in the
  part 4 table, and a store missing from it fails the test.
- Settings validation and history; command parsing, including refusal
  of an access token given as an argument; audit events; scrubbing of
  tokens from logs and bundles. Flows needing an embedder use the
  deterministic fake embedder and fake tokenizer.

### Recorded Hub API fixtures

- A fixture recorder, run by an agent on Dell or a cloud worker, calls
  the public Hub API for chosen public repositories, strips headers,
  cookies and account data, and saves the responses as JSON with their
  provenance: date, endpoint, library version, repository, revision.
- Cases with no stable public example are synthetic variants of
  recorded responses, labelled as synthetic. Tests replay fixtures
  through a fake transport; an unrecorded request fails the test.
- Required cases, with the expected verdict:
  - safetensors missing: Not supported, MF-C04;
  - custom code required, both `auto_map` and custom pipeline: MF-C05;
  - unsupported architecture: MF-C06;
  - oversize, by memory and by `max_size`: MF-C08 and MF-C12;
  - dimension mismatch: Compatible, with "full index rebuild needed"
    and an estimate; and one whose projection passes SPEC-008's index
    size budget: MF-C09;
  - gated: without an access token, with a token lacking access, and
    with a token that has access;
  - ONNX weights through fastembed: Compatible;
  - embedding window of 512 tokens, reranker window of 4,096, and no
    declared window: MF-C07;
  - no licence: Needs action, MF-C10 (R125); English not declared:
    MF-C11;
  - both `.bin` and safetensors present: Compatible, and only the
    safetensors would be fetched;
  - revision moved between check and add;
  - Hub errors: rate limit and server error.
- The R098 file lists recorded on 30 October 2026 (part 7) are kept as
  fixtures, so `sutra models pin` is tested against the real layouts.

### Integration tests (Dell)

- Install from a local fixture server holding the pinned files,
  including resume after a cut connection, a corrupted byte and a
  missing file; offline bundle creation and import; chunked upload
  through PHP with its peak memory measured.
- Real pinned models load offline in the model-host process and pass
  their self-tests within tolerance; bge-m3 through ONNX Runtime
  matches the example scores on its publisher's model card, and
  SPEC-008's recorded vectors come from that run; `en_core_web_lg`
  loads without PyTorch (R120); cached files match `models.lock`;
  index generation records hold the revision and tokenizer used.
- Another revision's index blocks new suggestions until rebuilt;
  earlier ones stay approvable (SPEC-008 and SPEC-009 parts or fakes).
- Update alongside, new suggestions paused during the rebuild, earlier
  ones still approvable, one-step switch and rollback (R122); trial
  turn-taking; faults injected at each step.
- Community flow with a small public test model served by the fixture
  server: check, add, self-test, trial on the 300-thread synthetic
  fixture (R058), activate, roll back, remove; pieces re-cut with the
  candidate's tokenizer; an over-long input refused, not cut.
- With the default compose file (R129): disable stops every
  container, a host restart leaves them stopped, enable starts them;
  uninstall, interrupted and resumed, removes the containers, images,
  networks and compose file, then the host helper.
- Memory and speed: every row of the memory table measured inside the
  service's capped container, with the ONNX bge-m3 and
  `en_core_web_lg` (R120); the Prometheus and collector containers
  measured against their declared caps (R121); an exact scan that
  fills disk cache does not shrink batches; a load over its row
  refused; a process ended by the operating system recovers.
- Lab: retraining on tiny synthetic data with a tiny model inside a
  9 GB group, giving a manifest and a signed comparison report; the
  release command refuses without an approval record.

### Progress, documentation and nightly

- Every long operation emits start, advance and finish progress events
  (SPEC-011's enforcement test covers the jobs registered here).
- Manual pages for Install models, the Models page, Add model, trials,
  disabling and uninstalling are updated with the feature (F024).
- The project's nightly run verifies the Verified tier only (ADR-0003),
  including the ONNX operator and external-weight checks on bge-m3
  (R120).

## Acceptance cases

### Lock file, formats and loading

- AT-MF01: `sutra models lock-check` refuses a `models.lock` with a file
  entry lacking SHA-256 or size, an unknown field, a branch name as
  revision, or a non-canonical layout; the writer reproduces the
  committed file byte for byte.
- AT-MF02: every shipped entry has id, role, purpose, repository,
  revision, format, runtime, licence, compatibility version, tokenizer
  fingerprint, memory, admission reference, file list and self-test.
- AT-MF03: install fetches only listed files at the pinned revision; a
  file with a changed byte is refused, deleted from staging and
  reported as MF-E03, and the model is never loaded.
- AT-MF04: a pickle stream renamed `.safetensors`, a ZIP holding
  `data.pkl`, a msgpack file holding such a ZIP deep inside, and `.bin`,
  `.pt`, `.pkl` or `.py` files are never fetched, imported or loaded;
  with failing stubs in place of Python's unpickler and PyTorch's
  loader, every production model still loads.
- AT-MF05: a configuration with `auto_map`, a custom pipeline, an
  unlisted module type or an unlisted spaCy function is refused before
  any model library is called; the static check fails any production
  call passing `trust_remote_code` other than the false constant.
- AT-MF06: the model-host process starts offline without network; a
  load that would need a missing file fails instead of downloading.
- AT-MF07: an import bundle matching `models.lock` is accepted; one with
  an extra file, a `..` path, an absolute path, a link or a wrong
  checksum is refused, and installed models are unchanged.
- AT-MF08: a 3 GB bundle uploaded in chunks keeps PHP's added peak
  under 8 MB per request and resumes after an interrupted chunk.
- AT-MF09: `sutra models pin` refuses `pytorch_model.bin`,
  `colbert_linear.pt`, `sparse_linear.pt` and the trf pipeline's
  `transformer/model`, and accepts bge-m3's `onnx/model.onnx` with
  `onnx/model.onnx_data`, `en_core_web_lg`'s pipeline files and the
  safetensors files of the reranker and potion, from the recorded
  layouts (R120).

### Verified models: health, update and compatibility

- AT-MF10: the health page shows one row per model with every ADR-0002
  field and only ADR-0002's statuses.
- AT-MF11: self-test outputs within tolerance pass; outputs moved beyond
  tolerance mark the model failed and suggestions not ready; bge-m3's
  pinned outputs agree with its model card's example scores (R120).
- AT-MF12: a missing required model gives "not ready"; p95 latency or
  error rate over its threshold gives "degraded" and stays ready.
- AT-MF13: with an index generation built by another embedder revision
  or tokenizer, new suggestions are blocked while suggestions made
  earlier stay approvable (R122), readiness says "not ready for
  suggestions" and the row shows "index mismatch" until the rebuild
  finishes; a search request carrying another key is refused.
- AT-MF14: a release pinning a new revision shows "update available";
  nothing downloads before the click; during the rebuild no new
  suggestion is made, while suggestions made before it can still be
  approved; model and index switch together in one step and new
  suggestions resume (R122); with a fault injected at every step, no
  suggestion is ever made from a mismatched index.
- AT-MF15: a model file changed on disk is caught by the next new
  process's full hash, by the daily check, or by the identity check on
  a reload; the model fails and an alert is raised.
- AT-MF16: an installed revision that is neither current nor previous
  in the lock is never loaded, and its row shows "not installed".

### Community models

- AT-MF17: search with each filter, sorted by downloads, shows every
  listed field and the right tier badge, from fixtures.
- AT-MF18: each fixture case gives its expected verdict and reason code;
  no request is made beyond the recorded ones; a check fetches only
  configuration files under 1 MiB.
- AT-MF19: add records repository, revision, per-file SHA-256 and the
  tokenizer fingerprint in the overlay; large files are checked against
  LFS SHA-256 and small ones against blob ids; a moved revision triggers
  a new check.
- AT-MF20: after the gated tests, the access token's value appears in
  no file, log, audit row, error, progress message, diagnostics bundle
  or process argument.
- AT-MF21: no download without licence acceptance; the record holds
  licence identifier, licence-text SHA-256, repository, revision, time
  and administrator reference.
- AT-MF22: the community self-test checks size, finiteness,
  determinism and the paraphrase ordering; a failure keeps it inactive.
- AT-MF23: a trial on the synthetic fixture reports recall at 1, 3 and
  5, anchor agreement, p95 latency, projected first-suggestion time and
  reindex estimate for both models on the same seeded sample and pool;
  staff-only and Sutradhar-inserted links never appear in the sample.
- AT-MF24: a community embedder's pieces are cut with its own tokenizer
  and never exceed its window; an over-long input is refused with an
  error and never cut short.
- AT-MF25: a candidate projected to miss the 30-second first suggestion
  where the active model meets it is activated only after the warning
  is shown and confirmed; the confirmation is in the audit log, and
  after the switch the health page says the forum runs outside the
  30-second target (MD-05, R125). Without confirmation nothing changes.
- AT-MF26: activation states the pause first, rebuilds in the
  background with new suggestions paused and earlier ones approvable,
  switches in one step on completion and rolls back in one action;
  after rollback only posts read since the switch wait (R122).
- AT-MF27: every step listed in part 3 writes exactly one audit event;
  community rows show "community, untested by Sutradhar".
- AT-MF28: with `models.community.enabled` false, Add model is hidden
  and community commands refused; with an allowed-authors list, other
  authors show MF-C02.
- AT-MF29: an overlay edited outside the service is detected; its
  models are refused and Verified models are unaffected.
- AT-MF30: every page action and its `sutra models` command give the
  same result; a user without "Manage models" is refused both.
- AT-MF31: the nightly run verifies only Verified entries; no Hub
  request occurs unless an administrator searches, checks, adds,
  installs, updates or bundles; the model-host process has no network.

### Retraining

- AT-MF32: a training export holds only decisions whose source and
  destination are guest-visible; a fixture post made staff-only before
  export, moderator identities and non-relevance rejections are absent.
- AT-MF33: training runs inside a 9 GB memory group, records its peak,
  shrinks batches near the ceiling and refuses rather than crashing.
- AT-MF34: a model without a passing report under the rule fixed before
  training, or without an owner approval naming its SHA-256, cannot be
  released or imported; a site-trained bundle with a bad signature or
  checksum is refused on the forum host.
- AT-MF35: erasing content used in a training set purges it from the
  lab and flags every model trained on it.

### Disabling, uninstalling and memory

- AT-MF36: disabling stops the service and every process within 90
  seconds with checkpoints saved, then every other container of the
  compose project; every component and all data stay; after a host
  restart they stay stopped; enabling starts them and resumes (R129).
- AT-MF37: uninstall leaves no vectors, index files or model files of
  any tier, previous revision, staging or trial, and none of the stores
  part 4 lists as deleted without export; it offers no export or keep
  option for them; it offers export and keep-or-delete for every other
  store in the part 4 table; it removes the compose project's
  containers, images, networks and compose file, then the host helper;
  published links remain.
- AT-MF38: uninstall never deletes outside the recorded directories,
  never follows symbolic links, and finishes when resumed.
- AT-MF39: disk cache filling the group during an exact scan does not
  shrink batches; unreclaimable memory at 90% or pressure over 10%
  shrinks them; 95% or 25% refuses new work with the reason.
- AT-MF40: under the owner's answers to MD-03 and MD-04 (R121, R122),
  the measured peak of every table row together stays under the 90%
  line of the cap in force (6 GB, or 8 GB once confirmed); the
  monitoring containers stay within their declared caps; a load over
  its row is refused; a killed process recovers.
- AT-MF41: during a trial's turn-taking, a reload in the same process
  does not re-hash, and new and edited posts keep priority over trial
  batches.
- AT-MF42: the check refuses a community model whose memory estimate
  does not fit its row in the table.
- AT-MF43: a community model with no stated licence is added only after
  the administrator accepts the stronger warning; the acceptance is in
  the audit log and the model's row says it has no stated licence
  (MD-10, R125). Without acceptance nothing is downloaded.
