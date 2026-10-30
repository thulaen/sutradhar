# SPEC-010 part 4: retraining, disabling, uninstalling and failures

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in the
[specification decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Part of [SPEC-010](models.md). Requirements: R105, R108, R050 as
amended, R120, R127, R129, R013, R017, R018, R020, R023, R026, R051,
R052, R058, R059, R060, R072, R092 as amended, R119.

## Design: retraining in the Dell lab (R105)

### Where and what

- Retraining runs only in the Dell lab, inside the runner's sandbox
  with no network, within the 9 GB lab ceiling for the whole job (R092
  as amended); never on a forum host, and never on the Windows
  computer, which only runs the app (R072, R119).
- It retrains bge-m3 or the reranker from moderators' approve, reject
  and remove decisions on guest-visible content (owner's answer). The
  reranker goes first, because a new reranker needs no index rebuild on
  the forum (MP-13).
- Base weights come from the lab's verified model cache, checked
  against `models.lock` before use. bge-m3's base is its official ONNX
  export (MD-01, R120): the lab reads the export's weight tensors,
  which are data only, into a trainable copy of the same architecture,
  and writes the retrained model back as an ONNX export at full
  precision, so the forum runs it as it runs bge-m3. Both conversions
  are checked by comparing outputs: the trainable copy against the
  export before training, and the new export against the trained copy
  after it.

### Training data

- Source: an authorised export of decisions from the forum to Dell, by
  the same route as the private datasets (R058). Each decision carries
  the guest-visible text needed to rebuild the model's inputs.
- Eligible only when source and destination were guest-visible when
  the decision was made and still are at export (R013). Training must
  start within 7 days of the export, or a fresh export is needed.
- Labels: approved gives a positive pair; removed after insertion gives
  a negative; rejected for a relevance reason gives a negative;
  rejected for another reason (for example the link limit was reached,
  or the thread was already linked) is left out; expired suggestions
  are left out. The reason-to-label table goes into each manifest.
- Always left out: moderator identities, authors' account details,
  private messages, analytics and staff-only content.
- Inputs are built with the production reading and matching functions
  (R026: the lab imports production code; production never imports lab
  code), so training sees what the service sees.
- Split by thread and time to prevent leakage (R059): the newest 20% of
  decisions form the held-out set; no held-out thread is in training.
- A run starts when at least 2,000 new decisions exist since the last
  trained version (`lab.retrain.min_decisions`, checked monthly,
  MP-15).

### Method within 9 GB (MP-14)

The reranker is fine-tuned in full. bge-m3 is fine-tuned with LoRA
(small added weight matrices trained while the base stays frozen),
merged back into ordinary full weights afterwards, so the forum runs
the same architecture with no extra library. Both use gradient
checkpointing (recomputing instead of storing intermediate results), a
measured batch size, and shrink batches before the ceiling (R023). New
training libraries need admission records first.

### Outputs

- A candidate model folder: data-only weights in its base's format
  (ONNX for bge-m3, safetensors for the reranker) and configuration,
  the same architecture as its base, no custom code.
- A training manifest (TOML): base entry, revision and file digests; a
  SHA-256 over the sorted decision ids and content hashes (no text);
  counts per label; split rule; settings; random seed; code commit;
  dependency-lock digest; measured peak memory; duration; and the
  output file digests.

### Comparison test, fixed before scoring

- The rule is fixed before training starts and its digest goes into the
  manifest; changing it afterwards voids the run.
- The rule itself is MD-08's answer, "better, with small tolerance":
  the same method as SPEC-009's challenger rule (Q-MA03), with paired
  95% bootstrap intervals by thread (re-sampling the test cases many
  times), recall at 5 better with the lower end of its interval above
  zero, anchor agreement and reconstruction recall with a measured
  change of at least zero and a lower end no lower than minus 1 point,
  no subgroup worse with 95% confidence, hard safety rules at 100%, and
  latency and memory within the forum-host budgets.
- It measures held-out decisions and a reconstruction sample of
  member-written guest-visible links.
- The lab runner signs the report, including every failure.

### Owner approval and the route into use

- The coordinator puts the signed report to the owner as a question.
  The answer is recorded in the approval record, naming the model's
  SHA-256, the manifest digest and the report digest. Without it, no
  release command proceeds (R105).
- The route is MD-07's answer, site-only and never published. The
  model becomes "site-trained" for that forum only: `sutra models
  release-site <manifest>` builds a bundle signed with the lab's key; on
  the forum host, `sutra models import` checks the signature against
  the lab public key recorded at setup and every file's SHA-256, then
  adds a site-trained overlay entry. Self-test, trial, activation and rollback
  follow part 3. Site-trained models are never published.

### Retention and erasure

- Training exports, manifests and reports stay on Dell (R058) under the
  lab's retention and R052.
- When a deletion or erasure touches a decision in a training set, the
  lab purges that data and flags every model trained on it. Under
  MD-09's answer a flagged model stays in use and is replaced by the
  next retraining, within 30 days.

### Lab commands

`sutra models retrain --role <role>`, `sutra models compare
<manifest>` and `sutra models release-site <manifest>`: Dell lab only,
each with progress and a signed result.

## Design: disabling the add-on (R108, R129)

The whole of Sutradhar runs from its default compose file (R129): the
Python service and the bundled Prometheus server and OpenTelemetry
collector (R127) are containers of one compose project, each with its
memory cap. The host helper (MP-16) acts on that project.

1. XenForo's add-on active-state change calls the PHP side, which sends
   a signed stop request to the service and asks the host helper to
   stop the compose project.
2. The service stops accepting work. Running jobs save checkpoints
   within 60 seconds (R020). The model-host process unloads its models
   and every worker exits, then the service exits. The helper then
   stops every other container of the project.
3. Each container's restart policy is `unless-stopped`, so containers
   stopped this way stay stopped after a host restart. A persistent
   "disabled" marker in the data directory also makes the service exit
   at once if its container is started any other way.
4. Kept on the server: everything. The compose file, the container
   images, every model file, the overlay lock, embeddings, index,
   reading and suggestion data, checkpoints, settings, audit and the
   monitoring data.
5. Admin pages show "Service stopped: add-on disabled". Nothing runs and
   no new suggestions appear.
6. If the service has not stopped after 90 seconds, its container is
   stopped forcibly at the compose stop timeout; the next start
   recovers from checkpoints (R020, R023).

Re-enabling clears the marker and asks the host helper to start the
compose project's containers. The service verifies and loads its
models and resumes its jobs. SPEC-008 catches up posts written or
edited while it was disabled.

## Design: every store and what uninstall does to it

SPEC-010 owns this table for SPEC-008 to SPEC-012 and for the
monitoring data of R127 (R108, R050 as amended). Embeddings, the index
and model files are always deleted, with no keep option and no export
step (owner's answers). Temporary copies and telemetry are deleted
without export under MP-17, which the owner accepted with the other
proposals. All other owned data keeps R050's export offer and
keep-or-delete choice; data kept stays in the data directory, with
restricted permissions, for a later reinstall.

Always deleted, no export, no keep option:

- SPEC-008: every vector (pieces, extras, titles, summaries, kept
  cut-text vectors and the first-pass store); vector segment files,
  row tables and removed-row lists; index generation records.
- SPEC-009: phrase vectors used for anchors.
- SPEC-010: model files of every tier, previous revisions, staging
  downloads, verified markers, the overlay lock, and trial vectors.

Deleted without export or keep option, as temporary copies rebuilt
from the records kept below or as short-lived telemetry (MP-17):

- SPEC-011: the live progress state, one row per operation in the
  service's state store.
- SPEC-012: the visual caches that only speed pages up; item 41's
  cache of scored candidates and judged results; and item 56's
  prepared figures in the add-on's own XenForo table, which the
  add-on's uninstall drops with its other tables.
- Monitoring data (R127): the Prometheus server's 15-day metrics store
  and the 7 days of sampled traces in Sutradhar's database.

The rows for items 41 and 56 and for the monitoring data are the
drafter's classification under the accepted MP-17 principle, shown
here for the owner to see: items 41 and 56 are rebuilt from SPEC-009's
suggestion and decision records and the link records, and the
telemetry holds only identifiers, counts and durations and expires on
its own within days.

Export offered, then kept or deleted as the administrator chooses:

- SPEC-008: reading records (per-item state and skip reasons,
  language, readable-text copies, piece, extras and summary text,
  entity and noun-phrase spans, existing-link records, summary
  subsets), reading checkpoints and the backlog cursor.
- SPEC-009: suggestion records with states and scores, reranker
  scores, review decisions with rejection and removal reasons (R018),
  rejected-candidate reasons, anchor candidates and suppressed
  connections (R017).
- SPEC-010: check and trial results, licence acceptances, accepted
  warnings (MD-05, MD-10), audit events, activation and rollback
  history.
- SPEC-011: progress history (completed runs and durations) and the
  audit record of each cancellation.
- SPEC-012: visual datasets (projection coordinates, layout positions,
  cluster records and their stored labels, weekly snapshots, daily
  reports, the error log and the feed buffer).
- Settings and settings history of every module (F001).

Removed as installed components (R108, R129): the compose project's
containers, their images and networks, then the compose file and,
last, the host helper. The data directory, mounted into the
containers, is not removed with them; what the administrator chose to
keep stays in it.

Never removed: links already published in posts (R017), XenForo's own
admin log (which receives the deletion receipt), and Dell lab data,
which follows its own R052 rules.

## Design: uninstalling (R108, R050 as amended)

- The confirmation page lists the always-deleted items and those
  deleted without export, with their sizes, and says plainly that they
  are deleted with no keep option and no export step. It then offers
  the export and the keep-or-delete choice for everything else, and
  states that published links remain.
- Steps, as one resumable job with progress:
  1. stop every container of the compose project as for disabling;
  2. delete the always-deleted items and those deleted without
     export, only inside the recorded data directory, never following
     symbolic links, refusing any path outside it, and counting files
     and bytes;
  3. export, keep or delete the other data as chosen;
  4. through the host helper, remove the compose project's containers,
     images and networks and the compose file, then the helper itself;
  5. record a deletion receipt (counts, bytes, time) in XenForo's admin
     log.
- An interrupted uninstall continues when run again; the page shows
  what remains. Files are deleted, not wiped (recorder's note 12).

## Failure behaviour

Each failure has a plain-English message, a central error code and a
manual link (R051). Model failures never affect the forum's ordinary
pages; PHP gives up quickly when the service does not answer.

- MF-E01, network failure or Hub unavailable during download: up to
  `models.download.retries` attempts with growing pauses; staged parts
  are kept; then failed, resumable later.
- MF-E02, pinned revision or file gone upstream: stop and offer the
  offline bundle. Never fall back to another revision.
- MF-E03, size or SHA-256 mismatch: the staged file is deleted and
  fetched once more; a second mismatch fails the model and raises a
  possible-tampering alert. The file is never loaded.
- MF-E04, not enough disk: refused in preflight; during a download, it
  stops with staged parts kept.
- MF-E05, forbidden content or unsafe archive member (pickle at any
  depth, code, unlisted file, link, absolute or `..` path): the whole
  model or bundle is refused and an alert raised.
- MF-E06, configuration asks for code or an unlisted module type: load
  refused.
- MF-E07, memory: a load that would break the memory table is refused
  with the reason (R023). A model-host process ended by the operating
  system restarts and recovers from checkpoints; three ends within
  10 minutes mark the newest-loaded model failed and isolate it (R019).
- MF-E08, self-test failure: model failed; suggestions not ready.
- MF-E09, index mismatch: new suggestions blocked, earlier ones stay
  approvable (R122); rebuild offered.
- MF-E10, installed revision in no lock: never loaded; the row shows
  "not installed" for the pinned revision.
- MF-E11, overlay digest mismatch: community and site-trained models
  refused; if one held an active role, that role is unavailable and
  readiness is "not ready". Alert raised.
- MF-E12, an installed file fails re-verification: model failed;
  re-installation offered.
- MF-E13, Hub search or check error, including rate limits: shown, with
  at most one retry; fresh cached results are still used.
- MF-E14, access token refused or access not granted: needs action;
  the token is discarded.
- MF-E15, rebuild or activation failure: the active model is loaded
  again and new suggestions resume on the old index (R122); the
  candidate stays inactive; the partial generation is kept for
  resuming.
- MF-E16, rollback impossible (previous files missing or failing
  verification): refused; the active model is unchanged.
- MF-E17, service did not stop in time: its container is stopped
  forcibly through the compose project (R129).
- MF-E18, interrupted uninstall: resumable.
- MF-E19, lab training near the 9 GB ceiling: batches shrink; if still
  too large, the run is refused and reported, never a silent crash.
- MF-E20, release of a lab model without an owner approval record:
  refused.
