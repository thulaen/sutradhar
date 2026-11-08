# 0055: Method research and admission catalogue

A method is one distinct way of computing something useful, such as a
way of scoring how well two threads match, or of estimating how sure we
are. This part says how a method is written down (the catalogue), how it
earns a place (admission), how we prove it is not a copy of another
(distinctness), and how the running service calls every method that
applies, never quietly skipping one. The release bar counts methods, and
forbids padding: renamed copies, changed settings and rearranged recipes
do not count. It is roadmap Phase 7 item 1, and feeds release gate item
1 in [AGENTS.md section 3](../../AGENTS.md#3-the-goal-and-the-release-bar).

Example: someone proposes "BM25 with k1 = 1.5" as a new method. The
catalogue already has BM25 (M-000001). The check sees the same code path
with a different number, refuses the entry and says: "This is a setting
of M-000001, not a new method." BM25F, which scores title and body as
separate fields, may enter as a candidate, because a small example shows
it ranks a title match above a body match where BM25 does not.

## What it does

### Words used here

- Method: one distinct procedure with its own inputs, outputs and maths.
  A setting, a preset or a wrapper around the same procedure is not a
  new method.
- Catalogue: the list of every method identity, from first idea to
  release, kept as text files in the repository.
- Admission record: the evidence that a method may count towards the
  release: sources, distinctness, specification, rights, correctness,
  usefulness, resources, integration and reviews.
- Stage: where a method stands in the catalogue (below).
- Probe set: a fixed, seeded set of made-up inputs on which every method
  of one output kind is run, so their behaviour can be compared.
- Behaviour digest: a BLAKE3 fingerprint of a method's outputs on its
  probe set. If the code changes what a method computes, the digest
  changes.
- Applicable method: a method registered for a pipeline step whose
  required inputs are available for the item being worked on.
- Pipeline: a named chain of work, such as ranking link candidates
  (`setu.rank`) or scoring spam risk (`kavach.spam`).

### One identity per procedure

1. Each method has one stable id, `M-NNNNNN`, never reused. Other names
   are aliases on the same entry.
2. A procedure used in several features (linking, spam, confidence) is
   still one method, counted once.
3. An entry names its `implementation`: the Rust path of the function
   that computes it, such as `rust:setu::rank::methods::bm25::score`,
   or the ganit engine and function, such as `ganit:jaal::pagerank`.
   ganit answers the numerical questions; the service does the live,
   per-post work. Two entries may not share an implementation.
4. An entry lists its `parameters` with their approved ranges. A
   parameter is a setting; changing it never makes a new identity.
5. A research variant earns its own id only with all four of: a sourced
   difference in objective, assumptions, representation or computation;
   a small example on which the difference changes the intended
   behaviour; a useful role measured on its own; and a reviewer's note
   saying why it is more than a setting or an alias.
6. A rejected or duplicate entry is kept as excluded, with its reason
   and, for a duplicate, the surviving id, so it cannot return under
   another name.

### Stages, in order

`candidate` → `research_ready` → `spec_approved` → `implemented` →
`verified` → `release_qualified`. Any stage may move to `excluded`.

- `spec_approved` needs the owner's recorded approval of the method's
  sheet. The owner approves sheets in batches by family, each with a
  plain-English summary per method.
- `implemented` needs production code registered under the entry's id:
  in the method registry, or in a ganit engine used by a job in ganit's
  job registry. Lab code never counts; a winning idea ships only as
  production code.
- `verified` needs the admission evidence, all current.
- `release_qualified` needs every admission area passed, rights cleared
  with no open patent concern, every dependency's licence compatible
  with AGPL-3.0-or-later, and resource use measured in the qualified
  environment.
- Passing one stage never passes another. Unrun checks stay unrun.
- A stage moves forward only through `sutra methods advance`, which
  checks the evidence and writes the change. Editing the stage field by
  hand fails the gate.

### Counting towards the release

- Counted: entries at `release_qualified` whose evidence matches the
  release candidate's exact commit and behaviour digest, and which a
  shipped pipeline runs for site owners, in the service or in ganit,
  each procedure once.
- Never counted: candidates, excluded entries, aliases, parameter
  variants, lab-only code, and entries whose evidence is stale.
- `sutra methods count` prints counts by stage, family and pipeline.
  The nightly exports them as gauges. The release gate refuses a
  candidate with fewer than 6,000,000 counted methods. Source access,
  research progress and qualified progress are shown separately.
- "Independently qualified" means each method has its own evidence, run
  on its own, and its review is signed by someone other than its author.

### Distinctness: three automatic checks, then a test and a person

Each check is cheap first and strict last.

1. Same code, other numbers (gate): two entries whose implementation
   paths are equal, or whose sheets differ only in `parameters`, are
   refused as parameter variants (SUT-501).
2. Similar description (gate): each new summary is compared with every
   other by the capability count's own similarity check, the same code,
   not a copy. Above its threshold the entry fails, naming the likely
   duplicate, unless its `distinctness.reason` explains the difference
   (SUT-502).
3. Same behaviour (nightly): every implemented method is run on the
   probe set of its output kind. The outputs are exported as an Arrow
   snapshot, and a Mulyankan job in ganit (its engine for evaluation
   statistics) compares them. Two methods whose outputs are equal, or
   whose orderings agree with a Spearman rank correlation of at least
   0.995 on every probe batch, are flagged "behaves the same" (SUT-503).
   A flagged pair cannot both count until the newer one's
   distinguishing test passes and a reviewer accepts it. Rank
   correlation is used because it catches a rescaled copy (a monotone
   transform) with one number; subtler cases are left to the
   distinguishing test and the reviewer.
4. The distinguishing test (gate): every entry at `implemented` or later
   names a test, `distinguishes_from_<id>`, built from its small
   example. It must pass, and it must fail when the method is swapped
   for its nearest neighbour; the gate runs it both ways. A reviewer who
   is not the author signs the distinctness review.

### The production method contract

Every method in the service implements one trait, in the crate
`sutradhar-methods` (an English placeholder until the owner names it).
A ganit method runs inside its ganit job; its versioned output
(parameters, scores or a model) reaches the live path through a method
that reads the active output version, so the run states cover it too.

- `Method`: `id()`, `version()`, `pipeline()`, `inputs()` (the evidence
  kinds it needs, and those it can use if present), `output()` (its
  output kind), `cost()` (cheap, moderate or heavy, measured) and
  `run(input, budget) -> Result<MethodOutput, MethodError>`. `run` is
  synchronous and pure: no clock, no network, no files. The caller
  passes the evidence and a work budget. Long runs go on the role's
  rayon pool, never on Tokio's threads.
- `MethodOutput` carries the value, a short machine-readable explanation
  (the parts a moderator's "why" view shows) and the method's version.
- `Registry`: built once at start-up by explicit calls, one
  `register(&mut Registry)` function per crate, wired by the Sutradhar
  Boot composition root. No plug-in loader and no global registration
  macro. A gate test checks that every catalogue entry at `implemented`
  or later is registered with the same id, and every registered method
  has an entry.
- `Registry::applicable(pipeline, available)` returns, for one item,
  each registered method of that pipeline as `Ready` (all required
  inputs present), `Waiting { missing }` or `Disabled { by, at }`.
- `Runner` runs the ready methods for one item, isolating each. A failed
  or panicking method becomes `Failed { code }` with its plain-English
  message; its dependants are withheld; the others still run. A waiting
  method keeps its settings and positive weight. Cheap methods run
  first; every applicable one still runs before approval.
- `GanitInput(job)`: an evidence kind meaning "the active, shadow-tested
  output of ganit job N". If no output was ever activated, the method
  waits. If the job is overdue, the last good output is used and an
  AutoIssue (an automatic issue in the tracker) is filed.
- An administrator may switch a failing method off. This is recorded in
  the audit log with a reason; the method then no longer counts as
  applicable, and the health page says so.

Example: a reply arrives before its thread's reaction counts are read.
The "destination quality" method shows "waiting for reaction counts";
the other ranking methods run; the suggestion is shown as "analysis
continuing" and cannot be approved until the quality method has run.
How its score is used belongs to the ranking specification.

### Crates and boundaries

- `sutradhar-methods`: the trait, ids, versions, registry, runner, run
  states, the catalogue types and the checks of this part. It depends
  on no feature crate; feature crates depend on it.
- Feature crates (`setu`, `kavach`, `sanket` and others) implement their
  methods and call `register`.
- `sutradhar-pariksha` (evaluation, the method lab's spec) supplies
  probe sets, link reconstruction runs and their Arrow exports; ganit's
  Mulyankan computes the intervals and verdicts used for usefulness.
- `sutra methods` and the management API's `methods` routes only read
  and advance the catalogue through `sutradhar-methods`.
- Swappable parts are traits with test fakes: `CatalogueStore` (TOML
  files; in-memory fake), `EvidenceStore` (Dell's evidence folder;
  in-memory fake), `TextSimilarity` (the capability check; fake by word
  overlap) and `ProbeSource` (seeded generator; fixed fake).
- The catalogue never runs a method, and the runner never edits the
  catalogue.

### The catalogue in the repository

- Place: `catalogue/`, with `candidates/`, `sources/` and an index file
  per folder joining the part files, each within the 300-line and
  80-character limits.
- Format: TOML, read by the `toml` crate into Rust types with validation
  (garde). The earlier YAML catalogue is converted, checked by a round
  trip: the joined TOML must equal the joined YAML field for field.
- The Rust types are the one source of truth for the shape; a generated
  schema is kept for readers.
- Fields per entry: id, name, aliases, family, kind, stage,
  counts_toward_release (computed, never typed by hand), source_ids,
  summary, proposed use, applicability, input and output contracts,
  distinctness (status, hypothesis, reason, nearest ids, duplicate_of),
  test ideas, resource plan, rights, exclusion, `implementation`,
  `pipeline`, `parameters` (name, type, default, approved range),
  `output_kind`, `distinguishing_test`, `sheet` (the method's own
  specification page), `lab_run` (its explaining lab run, with recorded
  inputs and outputs) and `admission`.
- Measured fields stay empty until a qualified run fills them; empty
  means unknown, never zero.

### The admission record

One record per method version, `catalogue/admission/M-NNNNNN.toml`,
holding references and digests only, never private data:

- `method_version` and `behaviour_digest`;
- `commit`, the dependency lock digest and the probe set version;
- one block per area (source, distinctness, specification, rights,
  correctness, usefulness, resources, integration, reviews), each with
  its status (`unrun`, `passed`, `failed`), the evidence bundle's id and
  SHA-256, the date and who signed it;
- `reuse`: evidence from an older version is reused only when the
  behaviour digest, the declared inputs and the probe set version are
  unchanged.

A change that alters a method's behaviour digest without a new
`method_version` fails the gate (SUT-504).

### Evidence bundles on Dell

Evidence lives in the evidence store on Dell's project storage, never in
public Git when it touches private data: test reports, usefulness
results with their frozen protocol and the ganit output version (job,
code version, snapshot, seed) that computed their intervals, resource
measurements with the environment fingerprint, and review notes. The
admission record names each bundle by id and digest, so a public reader
can check that it exists without seeing it.

### Run states in the service

One row per item and method in PostgreSQL, with RLS forced. Each role's
policy lets it read and write only the rows of the pipelines it runs.

| Column | Meaning |
| --- | --- |
| `subject_kind`, `subject_id` | what was worked on, such as a candidate |
| `method_id`, `method_version` | which method, which version |
| `state` | ready, waiting, running, finished, failed, disabled |
| `missing` | evidence kinds still missing, when waiting |
| `error_code` | SUT code when failed |
| `settings_version` | the immutable settings version used |
| `started_at`, `finished_at` | for durations and queue age |

Rows are derived data, purged with their subject.

### Commands, API and events

- `sutra methods list [--stage] [--family] [--pipeline]`, `show <id>`
  and `count`.
- `sutra methods check`: every catalogue rule here; the gate runs it on
  every commit.
- `sutra methods advance <id> <stage>`: moves one entry forward after
  checking its evidence, and writes the change for the commit.
- `sutra methods exclude <id> --reason <category>`: records an exclusion
  with its reason and, for a duplicate, the surviving id.
- `sutra methods fingerprint [<id>]`: runs the probe set and prints
  behaviour digests (lab only).
- Management API, read-only, capability CAP-157: `GET /api/methods`
  (paged list with filters and counts) and `GET /api/methods/{id}` (one
  entry, its stage, admission areas and run statistics: counts,
  durations, failures; never private data). The Methods screen in the
  Dioxus app shows them, to users with the review permission.
- Events (sanchar), ids only: `method.state_changed` (subject ids,
  method id, old and new state, error code); `catalogue.stage_changed`
  (method id, old and new stage, commit); `method.disabled` and
  `method.enabled` (method id, a pseudonymous id for who, when, reason
  code).

### Failure handling

Every failure is shown in plain English with a SUT code and a manual
link.

- SUT-500: catalogue file unreadable or invalid; the gate fails and
  names the file and line.
- SUT-501: parameter variant of an existing method.
- SUT-502: description too similar to another entry, with no reason.
- SUT-503: behaves the same as another method on the probe set.
- SUT-504: behaviour changed without a new method version.
- SUT-505: a registered method has no catalogue entry, or the reverse.
- SUT-506: a method failed while running; the message names the method,
  the item and what the moderator can do.
- SUT-507: a method exceeded its work budget; treated as failed for that
  item, retried once on the next pass, then reported.
- SUT-508: stage change refused: evidence missing, stale or failed.
- SUT-509: a ganit output a method needs is overdue; the last good
  output is used, the method says so, and an AutoIssue is filed.

A failed method never stops the others; its dependants are withheld.
Whether the item can still be approved is the pipeline's rule: for
ranking, it cannot until the method runs or an administrator disables
it. Nothing missing is invented or counted as zero.

### Security and privacy

- Methods receive only guest-visible evidence; the runner has no route
  to anything else. ganit receives only ids, numbers and word counts,
  never post text.
- The catalogue and admission records are public, so they hold sources,
  digests and verdicts, never site text, names or private paths. A gate
  scan checks this.
- Explanations in `MethodOutput` carry ids, numbers and labels, never
  post text. Screens fetch text separately, with permission checks.
- Methods are safe Rust; no method uses `unsafe`.

### Observability

- Metrics, ids and counts only: `sutradhar_methods_runs_total` by
  method, pipeline and outcome; `sutradhar_methods_run_seconds`
  histogram; `sutradhar_methods_waiting` gauge by missing evidence kind;
  `sutradhar_methods_counted` gauge by family (the release count).
- A trace span per method call, with method id and version.
- The health page lists failing methods with their codes and the items
  affected. A method failing more than 1% of its calls for an hour files
  an AutoIssue.

### Edge cases

- A source withdrawn or retracted: the entry stays, with a note; its
  evidence is reviewed again.
- A method useful only as a diagnostic counts only with an approved
  diagnostic purpose and a measured result.
- A stochastic method: the probe set records its seed; the digest is
  taken over a fixed seed, and its statistical property over many.
- Two methods that agree on the probe set but differ on real data: the
  probe set is too weak; the flag is cleared only by a distinguishing
  test, and the probe set gains the example.
- A method wrapping a library that is later switched off falls back to
  `implemented` and stops counting.
- A method whose evidence touched data later erased: its evidence is
  marked stale and re-run on the regenerated snapshot.

### Ganit jobs this part uses

Each is called by karya (the job system) on Dell; its versioned output
(code version, snapshot, seed, metrics) is named in the admission
record. The service never recomputes these statistics.

- 126, 127, 129 and 132 (Mulyankan: pass-rate intervals, recall and
  anchor-agreement intervals, bootstrap differences, "no worse than"
  decisions): each method's usefulness evidence.
- 130 (false-discovery control): many methods are tested for
  usefulness, and without it some would pass by luck.
- 133 and 144 (eval sampling and smallest adequate sample): how many
  cases each usefulness test needs.
- The behaviour-fingerprint comparison, a registered Mulyankan job; its
  number is set in ganit's job registry.

## Limits

- `sutra methods check` over 10,000 entries: at most 5 seconds.
- `GET /api/methods` with 10,000 entries: at most 200 ms at the 95th
  percentile.
- The runner's own work per method call (state, isolation, recording):
  at most 20 microseconds, excluding the method itself.
- The nightly fingerprint sweep for 2,000 methods ends within 3 hours,
  resumable, under the load governor.
- Memory in the service: the catalogue held for the API at most 16 MB
  for 10,000 entries; the registry under 1 MB in each role that runs
  methods. Each method's own memory is measured with it and counted in
  its role's budget, inside the service slice, within the memory limits in
  [AGENTS.md section 5.6](../../AGENTS.md#56-limits-on-the-owners-pc).
- The fingerprint sweep runs on Dell as a heavy job in the queue, one at
  a time, using at most 1 GB.
- Every benchmark here follows the slowdown rule of AGENTS.md section 6
  item 8.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

All on synthetic data. Paths are under `crates/sutradhar-methods/`
unless they start with `tests/acceptance/`.

- AT-MC01: a catalogue converted from the earlier YAML loads with every
  entry equal field for field. `tests/convert.rs`, regression.
- AT-MC02: two entries with the same implementation path fail
  `sutra methods check` with SUT-501, naming both.
  `tests/check_identity.rs`, unit.
- AT-MC03: an entry that differs from M-000001 only in a parameter's
  value fails with SUT-501, saying it is a setting of M-000001.
  `tests/check_identity.rs`, unit.
- AT-MC04: for any generated catalogue, no id is reused, every alias
  belongs to one entry, every `duplicate_of` points to a known,
  non-excluded, different id, and every excluded entry has a reason.
  `tests/props_catalogue.rs`, property.
- AT-MC05: a new summary close to an existing one with no written reason
  fails with SUT-502, naming the likely duplicate; with a reason, it
  passes. `tests/check_similar.rs`, unit, with the fake similarity.
- AT-MC06: method B equal to method A times two plus one is flagged
  SUT-503 by the probe sweep and does not count. `tests/fingerprint.rs`,
  property, with a fake ganit answering from fixed outputs; plus a ganit
  unit test of the Mulyankan job.
- AT-MC07: two methods whose top results differ on the distinguishing
  example are not flagged. `tests/fingerprint.rs`, unit.
- AT-MC08: an entry's distinguishing test fails with the method swapped
  for its nearest neighbour and passes with the real one.
  `tests/distinguishing.rs`, unit.
- AT-MC09: a method whose probe output changes while its version stays
  the same fails the gate with SUT-504. `tests/behaviour_digest.rs`,
  regression.
- AT-MC10: a registered method with no entry, or an entry at
  `implemented` with no registration, fails with SUT-505.
  `tests/registry_sync.rs`, unit.
- AT-MC11: a stage set by editing the file fails the gate; `advance`
  with full evidence moves one step; with missing evidence it is
  refused with SUT-508. `tests/advance.rs`, unit;
  `tests/cli_snapshots.rs`, snapshot.
- AT-MC12: one short of the release count, plus one entry with stale
  evidence, reports one short and the release gate refuses; with the
  full count and current evidence, it passes. `tests/count.rs`, unit.
- AT-MC13: with methods A (ready), B (missing input) and C (ready), A
  and C finish, B is "waiting" with the missing kind named, and B's
  settings and weight are unchanged. `tests/runner.rs`, unit.
- AT-MC14: method A panics and D depends on A: A is failed with SUT-506,
  D is withheld, the others finish and the process keeps running.
  `tests/runner.rs`, unit; `tests/chaos_isolation.rs`, chaos.
- AT-MC15: for random sets of available evidence, a method is ready
  exactly when all its required inputs are present, and is never
  silently absent from the list. `tests/props_applicable.rs`, property.
- AT-MC16: one registry shared by eight threads recording run states for
  one item: under loom, each method's final state is recorded once and
  none is lost. `tests/loom_runner.rs`, loom.
- AT-MC17: `check` over 10,000 entries on Dell ends within 5 seconds and
  passes the slowdown rule. `benches/check.rs`, criterion.
- AT-MC18: with 10,000 entries, the quick load test sends list requests
  for 60 seconds; the 95th percentile stays at or under 200 ms with no
  errors. `tests/load_api.rs`, load (goose), at most 2 minutes.
- AT-MC19: after a fixture run, logs, metrics, traces and events hold no
  fixture text. `tests/privacy_scan.rs`, unit.
- AT-MC20: an administrator with the review permission sees entries with
  stage, sources and admission areas on the Methods screen; without it,
  they are refused by the Cedar rules.
  `tests/acceptance/cap_157_browse_methods.rs`, acceptance, plus a
  Dioxus component test of the screen.

Rule-critical code: the counting rule and the registry sync. Besides
100% line, branch and MC/DC coverage, they need zero surviving mutants,
the property tests above, and a Kani proof that `counted()` never counts
an entry below `release_qualified` or with stale evidence.

Test-first order: catalogue types and loader (MC01, MC04); identity and
parameter checks (MC02, MC03); similarity through the fake, then the
shared real one (MC05); trait, registry and sync (MC10); applicability
and waiting (MC13, MC15); isolation and chaos (MC14); run-state
recording and loom (MC16); probe sets, digests and fingerprints (MC06,
MC07, MC09); distinguishing tests both ways (MC08); advance and
counting (MC11, MC12); API, screen, benchmarks, load and privacy (MC17
to MC20).

## Changes from the source

- The release count is 6,000,000 methods, as AGENTS.md section 3 sets,
  not 2,000.
- ganit methods, the fingerprint job and its tests are rewritten as
  Rust in ganit.
- Method explanations are Rust lab runs with recorded inputs and
  outputs, not the old notebooks.
- The Methods screen is a Dioxus app screen built from SDS components.
- The count command is `sutra methods count`, built with Cargo.
- The old 20x goal for the runner is replaced by the 5% median rule.
- Memory is set against the limits in AGENTS.md section 5.6, and the
  sweep runs in Dell's heavy-job queue.
- One run state is renamed "finished", to keep status words out.

Source: `docs/specifications/linking/methods.md`,
`docs/specifications/linking/methods-2.md` and
`docs/specifications/linking/methods-3.md` in the archive
`archive-2026-11-08-old-repo`.
