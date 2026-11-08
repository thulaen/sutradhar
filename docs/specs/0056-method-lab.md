# 0056: Method lab

The lab is where Sutradhar's methods are tried, measured and explained,
away from the sites. It produces the evidence that method admission
(spec 0055) needs. It is roadmap Phase 7 item 1. It has three parts:

- Experiments: small Rust programs, one per question, that run the real
  production code on a fixed dataset with a fixed random seed and
  record exactly what happened. They are the evidence.
- ganit, the Rust numerical engine, turns those records into numbers
  you can trust: ranges, comparisons and "is it really better?"
  verdicts.
- Lab runs that explain: each method's explanation is a Rust lab run
  with recorded inputs and outputs, shown step by step with charts in
  the Dioxus app's lab screen. They read the same records and call a
  lab instance of Sutradhar through its API.

Nothing in the lab runs on a site's server, and a site request never
starts a lab run. A winning idea reaches sites only as production code,
through the normal checks.

Example: someone wonders whether title matches should count more. They
write experiment `exp-0043-title-weight`. On Dell it runs link
reconstruction on the 2,000-thread tier at two title weights, with seed
7, and exports each case's outcome as an Arrow file. ganit's Mulyankan
compares the two: recall at 5 rose 1.8 points, range 0.9 to 2.7. The
report says "propose through the tuner"; the method's explaining lab
run shows the same result as a chart.

## What it does

### Words used here

- Experiment: one Rust binary in the `lab` crate that answers one
  question. Its report has six sections: Purpose, Inputs, Method,
  Experiment, Results and Decision.
- ganit: the numerical engine. Its batch jobs run on demand. Mulyankan
  is its engine for evaluation statistics, Shiksha for supervised
  models.
- Arrow: a standard file format for tables of numbers, read with the
  `arrow` crate, with the schemas kept in `contracts/`.
- Explaining lab run: a recorded lab run, with its inputs and outputs
  committed, that explains one method. The gate can replay it.
- Smoke run: a tiny run of an experiment (`--smoke`) on the committed
  synthetic fixture, to prove it still works.
- Dataset snapshot: a fixed, read-only copy of data, named by the digest
  of its content.
- Tier: the size of a private dataset on Dell: about 2,000 threads
  (development) and about 50,000 threads (nightly).
- Frozen protocol: the file that fixes, before any score is computed,
  what will be measured and what counts as a win.
- Link reconstruction: hiding links members wrote, in a test copy, and
  checking whether they are found again.
- Environment fingerprint: CPU model, governor, kernel, image digest,
  container limits and temperatures, recorded with every timed result.

### Crates and boundaries

- `sutradhar-pariksha` (evaluation): a production library, with no
  `unsafe`, no network, no lab dependency and no statistics code. It
  holds datasets and snapshots, splits, link reconstruction runs,
  per-case outcomes and their Arrow exports, and probe sets. It
  produces counts and per-case outcomes; ganit computes the ranges and
  verdicts. Ranking and tuning use it. It never imports `lab`.
- `lab`: one binary per experiment in `crates/lab/src/bin/`, plus
  `lab::support` (runner, context, results). It imports production
  crates directly; production crates never import it, and an
  import-boundary test enforces that.
- Explaining lab runs live under `lab/runs/<area>/<topic>/`, one home
  per method explanation; the catalogue links to it and never repeats
  it.
- Swappable parts are traits with test fakes: `DatasetSource`
  (synthetic fixture, generated posts, Dell tier snapshot; in-memory
  fake), `ResultSink` (files; in-memory fake), `EnvProbe` (fixed fake),
  `Governor` (a fake that pauses on command) and `GanitClient` (ganit's
  versioned job API; a fake that answers from fixed outputs).

### The experiment contract

1. `Experiment` trait: `card()` returns the Purpose, Inputs, Method and
   decision rule as data; `protocol()` returns the frozen protocol,
   naming the ganit job that will judge it (for example job 128, paired
   comparisons); `run(ctx)` returns per-case outcomes. The runner writes
   the Experiment and Results sections from what actually ran, and the
   author writes the Decision.
2. A binary is three lines: `fn main() { lab::run::<Exp0042>() }`.
   Seeds, recording, protocol freezing, the governor, progress, smoke
   mode, the Arrow export and the ganit call live once, in the runner.
3. The runner, in order: read the command line (`--smoke`, `--dataset`,
   `--seed`); refuse a dirty working copy for a recorded run; resolve
   the dataset to a snapshot id; write the protocol and its SHA-256
   before any scoring; ask the governor before starting and at each
   chunk boundary; run with a progress handle; export outcomes as
   Arrow; free its working set and only then ask ganit for the
   protocol's job, so only one heavy phase runs at a time; write
   `results.json`; render the report.
4. Fixed seeds: every random draw comes from one seeded generator
   (`rand`); the ganit job receives its own seed. Same commit, snapshot
   and seeds give the same numbers, except timings.
5. Chunked and resumable: long experiments checkpoint after each chunk.
   A governor pause or a killed run resumes, never restarts from
   scratch.

### The evaluation kit, one copy for everyone

Link reconstruction is run once, in `pariksha`, and its statistics are
computed once, in Mulyankan. The model comparison, the retraining test,
the measure, the ranking experiments and the tuner all use them.

- Splits by thread and by time: the newest share of decisions or links
  is held out, and no held-out thread appears in tuning.
- The final-assessment data is a separate Rust type, `FinalAssessment`,
  which no tuning or experiment function accepts, so leakage is refused
  by the compiler. The Arrow export marks the split, and ganit refuses
  an input whose split marks are missing.
- Per-case outcomes: the rank of the right destination, the anchor
  overlap, latency and memory, by case and subgroup.
- Statistics in Mulyankan: recall intervals (job 127), paired
  comparisons (128), bootstrap differences (129), false-discovery
  control (130), early stopping (131) and "no worse than" decisions
  (132). The model comparison's "clearly wins" and retraining's
  "better" rules are protocol parameters for these jobs.
- One proof standard for the lab and GanitEvolve: the frozen protocol
  states the unit, the stopping rule, the smallest gain and the largest
  harm. Where this page words a test differently, the research rules of
  GanitEvolve win.
- Probe sets for method fingerprints: seeded generated inputs per output
  kind, versioned.

### Explaining lab runs

- Data reaches an explaining run in two ways only: Arrow exports written
  by experiments and jobs, and the versioned API of a lab instance of
  Sutradhar on Dell, running on synthetic data or Dell's test sites. It
  never reads a database.
- From private tiers it gets ids, numbers and word counts, never post
  text. Text examples come only from the synthetic fixture, through the
  API.
- It may hold explanation, made-up examples, calls to the API and to
  ganit's engines, and charts. It may not hold a second implementation
  of a production method, private data or secrets. Its committed inputs
  and outputs come from synthetic data only. The gate checks each with
  gitleaks and the privacy scan.
- Replays: the nightly replays every explaining run from its recorded
  inputs on synthetic data, and the gate replays those a change
  touches. A run whose outputs no longer match its recording fails the
  check, naming the run and the step.
- Explaining runs never write evidence. Evidence comes only from
  experiment binaries and the ganit outputs they record.
- The lab screen shows progress by polling the lab instance's progress
  API.

### Where the lab runs

- Only on Dell, as heavy jobs in Dell's heavy-job queue (spec 0059),
  started as `pankti run -- CMD`. A lab comparison run takes one place:
  4 GB and 8 CPU threads (AGENTS.md section 11), one lab job at a time.
  Retraining (spec 0024) takes both places, 8 GB and 16 threads, with
  `pankti run --both`, so it runs alone.
- Private tiers are mounted read-only into the runner only. Arrow
  exports carry no text. Lab jobs have no network route outside Dell.
- Models used by experiments load through the production model host at
  their published precision, never lower; no int8.

### Small neural networks: trained in ganit, served by the service

- Training: pure-Rust models (candle) in ganit's batch jobs on the CPU,
  with resumable checkpoints and weights exported as safetensors.
- Scope: small networks only, each competing inside an existing job: an
  embedding adapter against whitening in job 273 (trained on
  reconstruction pairs and accepted links); neural classifier heads on
  embeddings against GLM or gradient-boosted trees inside each
  classifier job (for example 66, 68 and 74); and universal
  differential equations against the plain equation in job 222.
- The winner is on, and every rival that passes ships sealed and
  selectable. A network that fails is not exported.
- Never: fine-tuning the transformer models themselves (the embedder,
  the reranker) on this hardware.
- The competition is an experiment like any other: frozen protocol,
  Mulyankan's verdict, and the winner chosen by the job's own measure.
- The old Tri-Rank scoring method is tested as a rival, not chosen as a
  ranker. The 34 old text-style signals are not tested in the lab.
- Serving: the inference worker loads the weights with the
  `safetensors` crate and runs the forward pass itself, with `faer` for
  the matrix maths.
- The parity test, a gate check: training exports fixed test inputs
  with their expected outputs beside the weights; the serving forward
  pass runs the same inputs and must match within the stated tolerance
  (set per model in its export; by default within 1e-5 absolute or
  1e-4 relative, whichever is looser), or the gate fails and the
  weights are not activated.
- Each exported network is a versioned ganit output: shadow-tested,
  activated and rolled back like any other.

### Experiment results

`lab/results/<experiment>/<run-id>/`, where the run id is the start time
and a short digest:

- `results.json`: experiment id and version; commit; dependency lock
  digest; dataset snapshot id and digest; seed; protocol SHA-256;
  environment fingerprint; governor pauses (count and total time);
  duration; peak memory; the ganit output that judged it (job number,
  code version, snapshot, seed and its own metrics); each metric with
  its range and sample size; subgroup results; the verdict; and the six
  report sections as text.
- `outcomes.arrow`: the per-case outcomes ganit read.
- The report, generated by the runner (marked as generated) with its
  charts, and shown in the lab screen.
- `protocol.toml`: the frozen protocol, written before scoring.
- `checkpoint/`: chunk checkpoints, deleted when the run ends.

Private runs stay on Dell. The folder is ignored by Git. Nothing is
published automatically; a privacy-reviewed summary may be published
with a milestone when the owner agrees. A results file names the
snapshot by id and digest, never by a private path.

### Dataset snapshots

| Field | Meaning |
| --- | --- |
| `id` | BLAKE3 of the snapshot's sorted content records |
| `kind` | synthetic, generated, tier-2000, tier-50000 |
| `made_from` | export id and date (private tiers), or generator seed |
| `rules_version` | the reading rules version used to read it |
| `threads`, `posts`, `links` | counts |
| `erased` | ids removed by later erasure, as counts only in public |

Snapshots are immutable and locked once written. Erasure never edits
one: it makes a new snapshot without the erased content, and marks
every result made from the old one "made from erased data; re-run on
<new id>". The old snapshot's content and its Arrow exports are then
purged, keeping only its record.

### Evidence for method admission

When an experiment is declared as admission evidence for a method, the
runner also writes an evidence bundle to the evidence store with the
method id, its version and behaviour digest, the admission area it
serves and the ganit output version. The bundle's id and SHA-256 go
into the method's admission record.

The lab writes nothing to the service's PostgreSQL store. The lab
instance has its own database, with RLS forced like every other.

### Commands

All run on Dell through the governor.

- `sutra lab run <experiment> [--smoke] [--dataset <tier>]
  [--seed <n>]`: one experiment, its ganit verdict and its report.
- `sutra lab smoke [--changed]`: smoke runs, all or those affected.
- `sutra lab replay [--changed]`: replays of explaining runs.
- `sutra ganit <job> --fixture <name>`: one ganit job on fixture data.
- `sutra lab list` and `sutra lab show <run-id>`: list experiments and
  results (CAP-158), reading the results folder.

Capabilities served here: CAP-158 (run a method experiment and get its
report) and CAP-159 (compare candidate models with those in use).

### Calls to ganit

- The runner calls ganit's versioned job API with the job number, the
  Arrow file's path and digest, the protocol's parameters, a seed and a
  time limit. karya makes the call in Dell's nightly; the runner makes
  it on demand. ganit never schedules itself.
- ganit answers with a versioned output: the verdict and metrics with
  their reproducibility stamp. Numerical guards refuse NaN, infinity and
  out-of-range values. The runner checks that the stamp's snapshot
  digest equals the one it sent.
- Trace context passes on every call.

### Ganit jobs used

- 126 and 127: pass-rate, recall and anchor-agreement intervals.
- 128 and 129: paired comparisons (McNemar) and bootstrap differences.
- 130: false-discovery control when one experiment tries many settings.
- 131: stopping a trial early when the answer is clear.
- 132: "no worse than" decisions with a stated margin.
- 133 and 144: eval sampling that matches the site's mix, and the
  smallest sample that gives the needed precision.
- 145: the utility of accuracy, speed and memory, for model comparison.
- 151 and 153: the best estimate of truth and the label noise in the
  reconstruction set, so popular-thread bias and wrong member links are
  measured.
- 229: trial sample sizes, set before a trial starts.
- 273, the classifier jobs (such as 66, 68 and 74) and 222: where small
  networks compete.
- 32: tonight's experiment slate by value and cost, for the lab
  scheduler (a karya schedule on Dell).

### Failure handling

Plain-English messages with SUT codes:

- SUT-520: recorded run refused because the working copy has
  uncommitted changes (a smoke run is allowed, marked "smoke, not
  evidence").
- SUT-521: dataset snapshot missing, or its digest does not match.
- SUT-522: protocol changed after scoring started; the run is void.
- SUT-523: the experiment asked for final-assessment data (refused by
  type; this code covers the run-time check of loaded files).
- SUT-524: governor pause longer than the run's limit; the run waits,
  checkpointed, and resumes when allowed.
- SUT-525: out of memory inside the lab limit; the run resumes from its
  last checkpoint with a smaller chunk, and the report records it.
- SUT-526: ganit failed, timed out or returned a guarded value; the
  outcomes are kept, the verdict is "not computed", and the call can be
  repeated without re-running the first phase.
- SUT-527: an explaining run's replay failed; the gate or nightly names
  the run and the step.
- SUT-528: the ganit output's snapshot digest does not match the
  export; the verdict is refused.
- SUT-529: a network's parity test failed; the gate fails and the
  weights are not activated.

A timed result taken while the governor paused the run, or while a live
threshold was crossed, is discarded and re-run. The nightly's full runs
of experiments marked `nightly_full` catch a smoke run that passes while
the full run fails.

### Security and privacy

- Private tiers never leave Dell, and never enter public Git, hosted
  runners or logs. Only synthetic data and generated posts run outside
  Dell.
- Arrow exports hold ids, numbers and word counts, never post text; a
  scanner test checks every export schema for text columns.
- The lab instance's API token is read-only and lab-only, and is never
  mounted into production or gate jobs.
- Lab runs and reports treat post text as data, never instructions.

### Observability

- Metrics, ids and counts only: runs by experiment and outcome,
  duration, peak memory, governor pauses, ganit calls and errors.
- Every governor wait and pause is logged.
- A nightly report lists every experiment and explaining run with its
  last result; a failure files an AutoIssue (an automatic issue in the
  tracker).

### Edge cases

- Ranges at 100% or with tiny samples: ganit refuses a verdict below the
  minimum sample size (job 144) and reports "too few cases".
- Picking the best of many settings overfits: only the held-out result
  of the chosen setting is reported, with job 130's correction.
- Site drift between runs: results compare only within one snapshot.
- Reconstruction favours popular threads: popularity is always a
  subgroup, and job 153 estimates the label noise.
- An explaining run relies on an API route that changed: its replay
  fails, and the run changes with the API's version.
- ganit unavailable: outcomes are kept; the verdict is computed later
  without re-running the first phase (SUT-526).
- Two lab jobs at once: the second waits for the governor's lock.

## Limits

- Each experiment's `--smoke`, its ganit call included: at most 60
  seconds. `sutra lab smoke --changed` fits the gate's 2-minute
  quick-load slot for a typical change. Per commit, only the smoke runs
  a change touches run; all run nightly.
- An explaining run's replay on synthetic data: at most 3 minutes.
- The runner's own work (protocol, recording, Arrow export, report): at
  most 5 seconds per run.
- Writing the outcomes of 100,000 cases as Arrow: at most 1 second.
- A reconstruction run over 2,000 links on the 50,000-thread tier,
  excluding time inside the models and in ganit: at most 10 minutes;
  ganit's verdict on it within 2 minutes.
- Every lab comparison job fits one 4 GB heavy-job place, one at a
  time, with no swap. Retraining fits both places, 8 GB, and runs
  alone.
- The first phase and the ganit phase never overlap; the runner frees
  its working set before calling ganit (about 1 to 1.5 GB).
- The embedder and the reranker together need about 2 GB,
  so an experiment that loads both leaves little room; the vector index
  is memory-mapped. This is measured, not assumed. The runner refuses
  an experiment whose declared peak exceeds the limit before it starts.
- Network training runs on the CPU inside the same one place of 4 GB,
  one job at a time, checkpointed. Served networks are small: at most
  64 MB of weights in the inference worker in all, inside the service
  slice and the memory limits in
  [AGENTS.md section 5.6](../../AGENTS.md#56-limits-on-the-owners-pc).
- The lab runs only on Dell and counts against Dell's budget, not the
  owner's PC's.
- Every benchmark here follows the slowdown rule of AGENTS.md section 6
  item 8.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

All on synthetic data unless they say Dell. Paths are under `crates/`
unless they start with `tests/acceptance/`.

- AT-LB01: an experiment run twice with the same commit, snapshot and
  seeds gives equal metrics in both `results.json` files.
  `lab/tests/reproducible.rs`, property over seeds, with the fake ganit;
  plus ganit's golden outputs.
- AT-LB02: every report has the six sections, and the Experiment and
  Results sections were written by the runner. `lab/tests/report.rs`,
  unit; `lab/tests/report_snapshots.rs`, snapshot (insta).
- AT-LB03: a protocol edited after scoring began voids the run with
  SUT-522. `lab/tests/protocol.rs`, unit.
- AT-LB04: with uncommitted changes, a recorded run is refused with
  SUT-520; a smoke run proceeds, marked "smoke, not evidence".
  `lab/tests/runner.rs`, unit.
- AT-LB05: for any generated threads and decisions, no thread is on both
  sides of a split and every held-out item is newer than every tuning
  item. `sutradhar-pariksha/tests/props_split.rs`, property; Kani proof
  on bounded inputs, `sutradhar-pariksha/proofs/split.rs`.
- AT-LB06: code that passes final-assessment data to a tuning function
  does not compile. `sutradhar-pariksha/tests/ui/final_assessment.rs`,
  compile-fail.
- AT-LB07: outcomes with a known answer for each branch of the "better"
  and "clearly wins" protocols get matching verdicts from the ganit
  jobs. ganit unit tests; `lab/tests/verdicts.rs`, unit with the fake.
- AT-LB08: a fixture with 50 hidden links and a fake linker that knows
  40 gives 40 hits, with each miss listed.
  `sutradhar-pariksha/tests/reconstruct.rs`, unit.
- AT-LB09: a governor fake that pauses mid-run: the run continues from
  its last checkpoint, records the pause and discards timings taken
  during it. `lab/tests/governor.rs`, unit.
- AT-LB10: an experiment killed during a chunk, or ganit killed during
  its job, resumes on restart and gives the same final metrics.
  `lab/tests/chaos_resume.rs`, chaos, nightly.
- AT-LB11: erasure of a post used by a snapshot makes a new snapshot,
  purges the old one and its Arrow exports except for records, and
  marks the results. `sutradhar-pariksha/tests/erasure.rs`, unit.
- AT-LB12: no Arrow export schema can hold post text, and the writer and
  ganit's reader see the same values.
  `sutradhar-pariksha/tests/arrow_contract.rs`, unit; plus a ganit unit
  test.
- AT-LB13: a ganit answer whose snapshot digest differs, or holding NaN,
  is refused with SUT-528 or SUT-526. `lab/tests/ganit_guard.rs`, unit;
  property over generated answers.
- AT-LB14: every committed explaining run holds no secrets or private
  data, and its recorded inputs and outputs come from synthetic data.
  `lab/tests/run_scan.rs`, unit.
- AT-LB15: every explaining run replays from its recorded inputs on
  synthetic data within 3 minutes and matches its recorded outputs.
  `sutra lab replay`, nightly; changed ones in the gate.
- AT-LB16: an import of `lab` from any production crate fails the
  boundary test. `tests/boundaries.rs`, unit.
- AT-LB17: each experiment's `--smoke` on Dell's bench cores ends within
  60 seconds and passes the slowdown rule. `lab/benches/smoke.rs`,
  criterion.
- AT-LB18: an experiment declaring a peak above the lab limit is refused
  before loading anything; the two phases never overlap.
  `lab/tests/memory.rs`, unit.
- AT-LB19: a 2-minute fixed-seed mix of reconstruction requests keeps
  throughput and memory within baseline.
  `lab/tests/load_reconstruct.rs`, load.
- AT-LB20: on Dell, a run's report and `sutra lab show` agree.
  `tests/acceptance/cap_158_run_experiment.rs`, acceptance.
- AT-LB21: on Dell, the model comparison runs as an experiment, applies
  the frozen "clearly wins" rule through jobs 128 and 129 and records
  exact revisions. `tests/acceptance/cap_159_compare_models.rs`,
  acceptance.
- AT-LB22: the serving forward pass matches every exported expected
  output within tolerance; weights altered by one bit in one tensor, or
  an expected output shifted past the tolerance, fail the gate with
  SUT-529 and are not activated. `nn/tests/parity.rs`, regression and
  property over the exported cases; plus a ganit export test.
- AT-LB23: a network that loses its job's competition is not exported,
  and the existing method's output is activated; a winner's weights,
  test cases and verdict are exported together as one versioned
  output, and passing rivals stay sealed and selectable. ganit unit
  test; `lab/tests/winner_only.rs`, unit with the fake.
- AT-LB24: a training job killed mid-epoch resumes from its checkpoint
  with the same seed and reaches the same weights. ganit chaos test,
  nightly.
- AT-LB25: a job request to fine-tune the embedder or the reranker
  finds no such job in ganit's registry and is refused. ganit unit test.

Rule-critical code: the split rule and the final-assessment barrier.
Besides 100% line, branch and MC/DC coverage, they need zero surviving
mutants, the property test above, and a Kani proof that `split` never
puts one thread on both sides.

Test-first order: splits, the final-assessment type and the proof (LB05,
LB06); reconstruction with a fake linker (LB08), then the Arrow contract
(LB12); `GanitClient`, its fake and guards (LB13), then the verdict jobs
(LB07); the experiment trait, runner, protocol and reports (LB02 to
LB04), then reproducibility (LB01); checkpoints, governor and resume
(LB09, LB10); snapshots and erasure (LB11); run scan and replays (LB14,
LB15); boundaries, memory, speed and load (LB16 to LB19); capability
tests (LB20, LB21); networks, with the parity test failing on purpose
first, then the forward pass, the competition and resume (LB22 to
LB25).

## Changes from the source

- The old notebooks become Rust lab runs with recorded inputs and
  outputs, replayed in the gate and nightly, and shown in the Dioxus
  app's lab screen.
- ganit, its jobs and their tests are rewritten as Rust.
- Network training uses pure-Rust models; the parity test compares the
  training side's exported outputs with the serving forward pass.
- The report is generated by the Rust runner, not an outside report
  tool.
- Commands are `sutra lab ...` and `sutra ganit ...`, built with Cargo.
- Lab jobs run in Dell's heavy-job queue rather than their own
  containers.
- Model time is excluded as time inside the pure-Rust models.
- The old 20x goals are replaced by the 5% median rule.

Source: `docs/specifications/linking/lab.md`,
`docs/specifications/linking/lab-2.md` and
`docs/specifications/linking/lab-3.md` in the archive
`archive-2026-11-08-old-repo`.
