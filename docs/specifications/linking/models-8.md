# SPEC-010 part 8: proposals and recorder's notes

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in the
[specification decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Part of [SPEC-010](models.md). The decisions that needed the owner's
judgement, with the answers, are in
[part 7](models-7.md#decisions-answered-by-the-owner).

## Proposals

Technical choices the requirements left open. The owner accepted all
eighteen as a group on 30 October 2026, with no exceptions. Each names
the alternative it was weighed against. Where a later answer changes
how one reads, a note follows it.

- MP-01: `models.lock` is TOML without indentation, so every SHA-256
  line fits in 80 characters. (Alternative: JSON, whose nested lines
  would be too long.)
- MP-02: lab-only models, such as SPEC-009's challengers, are pinned in
  `models.lock` with role `lab_only` and `ship = false`, and never
  installed on a forum host. (Alternative: a separate lab lock file.)
- MP-03: a first-pass model mismatch pauses only the early shortlist
  until its store is rebuilt, because no suggestion depends on it;
  SPEC-008 follows this. (Alternative: block all suggestions.)
- MP-04: the model memory budget (`models.memory_budget`, `auto`) is
  what the memory table leaves for models after its other rows.
  (Alternative: a fixed figure set by the administrator.)
- MP-05: every model is loaded once, in one model-host process, as
  SPEC-008 sets out. Files are fully re-hashed when a process first
  loads them; a reload within the same process checks file identity,
  size and modification time only; the daily job and every new process
  re-hash in full. A model stays loaded for the process's life, with
  the exceptions in part 2. (Alternative: a full re-hash on every load,
  which turn-taking would make costly.) Under MD-04's answer (R122),
  turn-taking remains only for trials.
- MP-06: a Verified model that fails its self-test is "failed", and
  suggestions are not ready, because changed outputs may no longer
  match the index. (Alternative: "degraded" only.)
- MP-07: a model is "degraded" when the last hour's p95 latency is over
  twice the figure the host's own self-test measured at install, or
  more than 1% of calls fail. (Alternative: fixed budgets per model in
  `models.lock`.)
- MP-08: a Hugging Face access token is held in memory for its job only
  and entered again for any later download. (Alternative: kept
  encrypted in the service's secret store.)
- MP-09: community models must declare English or "multilingual",
  since the first release is English only. (Alternative: no language
  rule; the trial decides.)
- MP-10: activation needs a finished trial on the forum's own links;
  the administrator may still activate a model that scores worse,
  subject to MD-05, now a warning to confirm (R125). (Alternative: the
  trial is optional.)
- MP-11: servers without internet access may add community models
  through a bundle made by `sutra models bundle <repo>@<revision>` on a
  connected machine, carrying the recorded Hub metadata so the same
  check runs offline. (Alternative: community models need the Hub.)
- MP-12: if `models.community.enabled` is switched off while a
  community model is active, it keeps running until the administrator
  rolls back, with a banner explaining this. (Alternative: automatic
  return to the Verified model, with a rebuild.)
- MP-13: the reranker is retrained first, and bge-m3 only once
  reranker retraining stops improving, because a new reranker needs no
  index rebuild. (Alternative: both every cycle.)
- MP-14: retraining fine-tunes the reranker in full and bge-m3 with
  LoRA, merged back into ordinary weights; a LoRA library such as
  `peft` needs admission. (Alternative: train only bge-m3's top layers,
  with no new library but possibly less learning.)
- MP-15: retraining is checked monthly and runs when at least 2,000 new
  decisions exist since the last version. (Alternative: only when the
  owner asks.)
- MP-16: a small host helper, installed at setup, may only start, stop
  and remove Sutradhar's own service and files; disabling, enabling and
  uninstalling use it. (Alternative: the administrator or an agent runs
  `sutra uninstall --system` and the start and stop commands.) Under
  R129 it reads as follows: the helper acts only on Sutradhar's compose
  project. It may start and stop that project's containers, remove its
  containers, images, networks and compose file, and delete
  Sutradhar's own files inside the recorded data directory; it is
  removed last at uninstall (part 4).
- MP-17: at uninstall, temporary copies that only speed pages up
  (SPEC-012's visual caches and SPEC-011's live progress state) are
  deleted without export, because they are rebuilt from records that
  keep R050's export offer. (Alternative: offer export and a keep
  choice for them too.) Under this principle the drafter also
  classifies SPEC-012's item 41 cache and item 56 prepared figures,
  and the monitoring data of R127, as deleted without export (part 4,
  recorder's note 27).
- MP-18: the Models page's lifecycle labels (checking, added
  (inactive), trial running, rebuilding, active, previous) are separate
  from ADR-0002's health statuses, which stay exactly as recorded.
  (Alternative: add these labels to the ADR-0002 status list.)

## Earlier numbers

Other drafts cite this draft's first numbering. Old to new: MD-01 is
now MD-06; MD-02 is MP-01; MD-03 is MD-01; MD-04 is MP-02; MD-05 is
MD-02; MD-06 is MP-03; MD-07 is MD-04; MD-08 is MP-04; MD-09 is MP-05;
MD-10 is MP-06; MD-11 is MP-07; MD-12 is MP-08; MD-13 is MD-10; MD-14
is MP-09; MD-15 is MP-10; MD-16 is MP-11; MD-17 is MD-07; MD-18 is
MP-12; MD-19 is MP-13; MD-20 is MP-14; MD-21 is MD-08; MD-22 is MP-15;
MD-23 is MD-09; MD-24 is MP-16. MD-03 (everyday memory), MD-05 (the
30-second target), MP-17 and MP-18 are new.

## Recorder's notes

These are the drafter's interpretations, not owner decisions.

1. ADR-0003's "the nightly run verifies only the Verified tier" is read
   as the project's nightly run on Dell, which cannot know each forum's
   community models. Each forum host also checks every installed model
   daily.
2. The owner's first install answer said "safetensors only"; the later
   answer chose data-only formats without pickle (ADR-0002). R106 listed
   ONNX for community models only, which is why MD-01 asked the owner;
   R120 adds bge-m3's official ONNX export to R106's formats.
3. SPEC-009 owns the R100 challenger comparison, its "clearly wins"
   rule and the earlier set's place in it (its Q-MA03 and Q-MA04). This
   specification only pins the challengers (MP-02).
4. ADR-0002's "rebuilds the index with it, and only then switches" is
   applied to models whose outputs form an index. The reranker has no
   index; stored spaCy outputs follow the compatibility key rule.
5. `[model.previous]` lets a forum keep running its installed model
   after an add-on upgrade until the administrator presses Update, as
   ADR-0002's update button implies.
6. ADR-0003 says to "compare the vector size with the active index and
   show the full-rebuild time", so a different size is shown, not
   refused; refusal comes only from SPEC-008's index size budget
   (MF-C09). The owner's fixture case "dimension mismatch" tests this.
7. A model's window is the smallest of its declared limits, counted in
   its own tokenizer's tokens, so "accepts at least 1,024 tokens" is
   never assumed from the architecture alone.
8. Trials use only links written by members, because links Sutradhar
   inserted would favour the active model.
9. MD-05's answer (R125) waives R015 only for the forum whose
   administrator confirms the warning. The health-page wording and the
   use of that forum's own trial figures for its alerts (ADR-0007) are
   the drafter's reading.
10. "Decisions on guest-visible content" (R105) is read as guest-visible
    both when decided and at export. Rejections for reasons other than
    relevance are left out so they do not teach the model that a good
    match was wrong.
11. "Keeps its components on the server" (R108) is read as keeping
    everything: environment, model files, embeddings, index and data.
12. Uninstall deletes files in the ordinary way. Wiping storage blocks
    cannot be promised on solid-state drives and was not requested.
13. Recorded Hub fixtures are JSON data files, proposed as first-party
    data under the source-limit classes; if that class must keep
    80-character lines, long responses are split into smaller files,
    never encoded.
14. New names use English placeholders until approved: the component
    `models` (package `sutradhar.models`), the model-host process
    (SPEC-008's term), the admin permissions "Manage models" and "View
    models", and the tier "site-trained".
15. Memory figures are arithmetic from file sizes and published
    parameter counts, not measurements. Rows from other drafts use their
    stated allowances: SPEC-011's 40 MB, SPEC-012's 256 MB of caches,
    four 64 MB builds and 1 GB projection, SPEC-009's 256 MB scan buffer
    and SPEC-008's row tables. The two 128 MB rows for SPEC-008's and
    SPEC-009's working memory are placeholders until they state figures.
    The in-service telemetry row (64 MB) and the monitoring containers'
    512 MB and 128 MB are ADR-0006's estimates, also unmeasured.
16. The format facts in part 7 come from the coordinator's check on
    Dell; the drafter ran nothing. Full revisions for the reranker and
    potion are written by `sutra models pin`; only their first
    characters were reported.
17. Licences of the shipped models are not restated here; admission
    records are the single home for them.
18. A change to a Verified entry is an owner approval of a shipped model
    (R027, R100). It is not a gate change, so R116's question route for
    gate changes does not apply to it as such.
19. Any other model a later specification needs on a forum host, such
    as a language identifier for SPEC-008, must be pinned in
    `models.lock` and follow every rule here.
20. A trial pool of 5,000 threads gives figures that differ from
    whole-forum recall; the page labels them as trial-pool figures.
21. SPEC-008's and SPEC-009's drafts say their stored data is deleted
    on uninstall. Under R050 as amended only embeddings, the index and
    model files are always deleted, so the part 4 table keeps the export
    offer and keep-or-delete choice for their other records (reading
    records, suggestion records, rejection reasons). Those drafts should
    point to that table. Deleting such records always would be a change
    to R050 for the owner.
22. ADR-0002's status list is unchanged. The first draft's "update
    required" is replaced by ADR-0002's "not installed" for the pinned
    revision.
23. Memory thresholds use unreclaimable memory and memory pressure;
    SPEC-008's backlog pause uses processor and disk pressure. The two
    work together: either can slow the backlog.
24. R122's pause covers a rebuild for a changed embedder. A trial
    changes no model, so it keeps taking turns with the active embedder
    and suggestions continue during it.
25. During a rebuild, posts written or edited are read into the new
    generation. If the activation or update is cancelled or fails, the
    restored model reads them as it does after a disable.
26. Retraining bge-m3 from its ONNX export (R120) needs its weights
    read into a trainable form and the result exported again; both
    steps, and checking them by comparing outputs, are the drafter's
    design.
27. The classification of SPEC-012's item 41 cache and item 56 prepared
    figures, and of the monitoring data of R127 (Prometheus's 15-day
    store and the 7 days of traces), as deleted without export is the
    drafter's, following the accepted MP-17 principle. The owner may
    move any of them to the export-offered list.
28. Following R129's recorder's note, the "service definition" and
    MP-16's host helper are read as the compose project. The
    `unless-stopped` restart policy and the disabled marker are the
    drafter's means of keeping a disabled forum stopped. Dell's test
    sandbox is a separate matter and is unchanged.
