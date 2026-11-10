# 0012: GanitEvolve

GanitEvolve searches for better code the way DeepMind's AlphaEvolve
does: a language model proposes changes to marked parts of a program,
automatic evaluators score each change, and an archive of good, varied
programs supplies the parents and examples for the next proposals. It
also writes the speed receipt that every new or changed source file
must carry ([AGENTS.md section 6](../../AGENTS.md#6-code-standards)
rule 9). It is roadmap Phase 2 item 6. The proof standard it shares
with the lab is in 0013.

## What it does

### Who does what

1. **The driving agent** (Codex, Claude or Gemini) reads each task
   packet, proposes a change and submits it, inside its own chat and on
   its own usage. Its tokens are the only model cost. GanitEvolve never
   calls a model and holds no key. Nothing runs unless an agent drives.
2. **`ganit-evolve`**, a Rust command in its own crate, keeps the run,
   builds task packets, checks each submission, runs the evaluators in
   sealed containers through the heavy-job queue, and records results.
3. **ganit**, the Rust numerical engine, does the maths: choosing parents
   and examples from the archive, the diversity grid, island migration,
   hint choice, and deciding whether a gain is real.
4. **aegis** judges any winner as an ordinary change. Nothing evolved
   lands without the full gate and review.

### Duties

| Duty | Who | May not be |
| --- | --- | --- |
| Explorer | the driving agent and its subagents | - |
| Challenger | writes hidden tests and an independent route | the driver's product |
| Judge | scores against the checklist | the driver's product |
| Reviewer | re-runs the winner | the driver's product |

A subagent belongs to its parent's product: Claude's helpers are Claude.
So a challenger, judge or reviewer is another product.

### The loop

1. `ganit-evolve start <problem>` opens a run with its budget: a number
   of candidates, a time limit, or until the agent stops. With no budget
   given, the default is 20 candidates or two hours, whichever comes
   first.
2. `ganit-evolve next <run>` prints one task packet: the parent program
   with its EVOLVE-BLOCK regions, two to four inspiring programs from
   the archive, the evaluator's feedback on the parent (scores, errors,
   timings, profile highlights), one hint, any judgements, the rules and
   the exact answer format, and the path to save the answer to.
3. The agent writes its change as SEARCH/REPLACE blocks, only inside
   EVOLVE-BLOCK regions, saves it there and runs
   `ganit-evolve submit <run> <file>`.
4. GanitEvolve refuses a change that touches anything outside the
   blocks, breaks the format or repeats an earlier candidate. Otherwise
   it queues the evaluation steps below.
5. `ganit-evolve wait <run>` returns the scores. The agent may submit up
   to the set batch limit before waiting, so it is not idle while Dell
   evaluates.
6. Repeat from step 2 until the budget ends or the agent stops.
   `ganit-evolve status <run>` shows the best so far.
7. `ganit-evolve propose <run>` turns the best candidate into a change:
   the patch, new regression and property tests, benchmark evidence
   with its environment fingerprint, and a walkthrough. It then goes
   through aegis like any change.

Any agent may resume a run, so a run started from Claude can go on from
Codex or Gemini, and models mix, as AlphaEvolve mixes a fast and a
strong model.

### Helpers

1. Each packet may be answered by up to `explore.subagents` helpers at
   once, each given the packet and one direction: speed, algorithm,
   architecture, system design or a new technique.
2. Each helper hands back an answer and a note on its idea. The driver
   submits them under the batch limit, recording each as
   `PARENT/HELPER` (for example `claude/helper-2`).
3. Directions that keep failing show in the research memory (0013), so
   later helpers start elsewhere.

### Evaluation steps

Each step is cheaper than the next. A candidate stops at its first
failure, and the failure becomes feedback in the next packet.

1. **Build.** Compile with the pinned toolchain and strict warnings. Any
   warning fails.
2. **Safety.** A parse of the source, not a text search, refuses input
   and output, files, environment variables, system calls, processes,
   network, clocks, random numbers, `unsafe`, state kept between calls,
   and anything else on the problem's forbidden list.
3. **Correctness.** Public tests, then hidden tests the agent never
   sees, with fresh random inputs each time, compared with the reference
   within the problem's tolerance. A debug build with overflow and
   bounds checks runs them.
4. **Measuring.** For a `speed` problem, GanitEvolve times an optimised
   build, outside the candidate, three times, keeping the median. A run
   whose spread is over the limit is repeated, never accepted. Timing
   runs alone in the heavy-job queue, with its fingerprint (0007). ganit keeps
   a gain only if it beats the noise at 95% confidence and the 5% rule
   of 0007. A `score` problem runs its scoring step here instead. Guards
   are checked in this step (below).
5. **Finalists only.** The full aegis gate on the would-be change.

`evaluation.parallel` (1 to 3, default 1) sets how many candidates may
run steps 1 to 3 at once. The measuring step always runs alone, so
timings are not shared with another candidate's work. Results are
recorded in submission order, whatever finishes first. Every step goes
through Dell's heavy-job queue.

### Guards

1. A problem may list guards: `[[evaluation.guard]]` with `name`,
   `higher_better` and `worst_loss` (such as 0.05 for 5%).
2. The measuring step's output may carry `"also"`: for each guard, the
   candidate's and the reference's series from the same run.
3. A guard whose median ratio, turned so above 1 is better, is below
   `1 - worst_loss` fails the candidate, naming the guard and its loss.
   A missing guard series fails too.
4. Guards are hard limits. Only the objective decides a gain. For
   example, a candidate twice as fast that needs 40% more memory fails a
   5% memory guard.

### Objectives beyond speed

1. A problem names its objective: `speed` or `score`.
2. A `score` problem's evaluator ends with a scoring step that prints
   its scores, higher being better, measured by tools and never by an
   AI: for example the architecture checks' findings, module walls
   crossed, test count, memory, start-up time or a quality measure.
3. A `score` candidate is kept only when its scores beat the best by the
   protocol's smallest gain, at the protocol's confidence, on repeated
   scores.
4. Earlier steps are hard limits: the tests, the safety parse, the
   architecture checks once built, and the source limits. A candidate
   that fails one scores nothing.
5. A candidate may use only admitted dependencies. A new library is a
   separate admission, never brought in by a candidate. Stability is
   measured: the scoring step repeats, and property tests and fuzzing
   run where the problem has them.

### Judging

1. `ganit-evolve judge` records one agent's score (0 to 10) and reason
   for one candidate against the problem's pinned `checklist.toml`:
   stable, simple, in the style of Sutradhar Boot (AGENTS.md section
   5.1), new, and its risks.
2. It refuses any agent whose product drove the run.
3. Judgements show in later packets beside the candidate, and their
   median is a diversity measure, so good but different ideas survive.
4. A judgement never passes, fails or promotes a candidate. It is shown
   as advice.

### Hints

1. `ganit-evolve hint --run RUN --agent AGENT --model MODEL --text TEXT`
   adds a hint of one line, at most 300 characters. Any agent may add
   one.
   Hints are instructions, never answers, and packets show them so.
2. ganit chooses one hint per packet: a hint never used goes first;
   otherwise the one with the highest share of real gains plus a bonus
   that shrinks with use (the UCB1 rule). Ties go to the oldest.
3. Each answer records its hint, so later choices count it.

### The archive

1. Every candidate is stored, named by its SHA-256, with its parent, the
   packet it answered, the driving agent and model as the agent reports
   them, its scores, its evaluator output and the attempt count of its
   question.
2. Storage is a folder of append-only records and blobs on Dell,
   outside the repository, within the cache budgets. No database table
   is used, so the row-level security rule does not arise.
3. A diversity grid (MAP-Elites) keeps the best candidate in each cell
   of the problem's chosen measures, for example speed band against code
   length.
4. Several islands evolve apart and now and then share their best
   candidates, so no line settles too early.
5. Parent choice favours strong cells but keeps a set share for weaker
   ones. The shares are settings.

### Guarding against reward hacking

1. Candidates change only EVOLVE-BLOCK regions. The evaluator, tests,
   reference and timing code are pinned by hash, read-only and out of
   the candidate's reach. If the evaluator's files change during a run,
   the run refuses to go on until a new run starts.
2. Hidden tests and fresh random inputs stop answers being learnt by
   heart. Outputs are compared, never just counted.
3. A candidate suddenly far faster than the archive best is measured
   again on new inputs before it is believed.
4. Known tricks (caching across calls, spotting the test harness,
   special cases for input sizes) are listed per problem and checked.
5. Every run's evidence is signed. Promotion needs the full gate, a
   reviewer who is not the driving product, and the signed receipt whose
   hash equals the change's hash.

### A problem definition

Each problem lives in its own folder under the GanitEvolve crate:

1. `problem.toml`: name, language, the source files and their
   EVOLVE-BLOCK regions, the reference, tolerances, the input
   generator, public and hidden test sets, objectives (the main one
   first), diversity measures, guards, forbidden constructs, the run
   budget defaults and the timing settings.
2. The program: an ordinary source file whose fixed parts (the
   interface, argument checks and bindings) sit outside the blocks, so
   the calling form can never change.
3. The evaluator's tests and the reference, pinned by hash.
4. Objectives are explicit numbers, for example "median seconds for the
   shape set" (lower is better) with "largest scaled error" as a hard
   limit. A candidate that misses a hard limit scores nothing.
5. A whole-file problem needs no EVOLVE-BLOCK markers and uses the
   crate's own tests as the correctness step, so any Rust file can be
   evolved.

### The first problem: ganit's matrix kernels

1. ganit jobs 273 (embedding whitening) and 275 (co-reading
   factorisation) use a symmetric rank-k update and a general matrix
   product on shapes the jobs know: for example 1,024 columns, the
   embedding width, by up to tens of thousands of rows. The problem
   evolves a blocked double-precision rank-k update, then the matrix
   product, for those shapes.
2. Double precision only, as the owner's full-precision ruling asks.
3. The reference is a pinned, independent implementation built in the
   evaluation image.
4. Every result is compared with the reference using a backward-error
   bound scaled by n, the unit round-off and the input norms. The exact
   formula and constant are fixed in `problem.toml`.
5. Edge cases in every evaluation: n = 0 and 1; sizes not a multiple of
   any tile; leading dimensions larger than the matrix; both triangles
   and both transposes; alpha = 0; beta = 0 with NaN already in the
   output, which must vanish since it is defined as not read; NaN and
   infinity in the inputs spreading exactly as the reference does.
6. Threads are not allowed at first. Later, only with the thread count
   set by ganit's core plan.
7. Build and timing flags are part of the problem, never the candidate.
8. A winning kernel replaces the routine in ganit only through aegis,
   with its new tests, the safe-twin rules of 0013 and the owner's
   review of its evidence.
9. Later problems follow the same pattern: the Cholesky solve for job
   275, the matrix-vector product for the graph spectra, other Rust hot
   paths (with criterion and property tests), and ganit's heuristics,
   judged on replayed, made-up data.

### A receipt for every source change

1. Every new or changed shipped source file in a commit must be the
   exact program a GanitEvolve run proved. The proof is a receipt in
   `receipts/` whose `target` is the file and whose `tested` is the
   SHA-256 of its new bytes.
2. A shipped source file is a Rust, TypeScript, CSS, PHP or SQL file
   that any build target other than a test or a benchmark, under any
   feature set, compiles or
   includes (through `mod`, `#[path]` or `include!`), as the gate reads
   it from the build, whatever folder it sits in. Every other source
   file (tests, benchmarks, test data) goes with the receipt of the
   code it tests (AGENTS.md section 6 rule 9).
3. A deleted file needs nothing. A change to documents only is not
   affected.
4. The gate check `ganit.every-change`, run by aegis through
   `ganit-evolve gate`, blocks at the commit, push and release tiers.
   The door runs it on the receiving side, so skipping local hooks
   cannot avoid it.
5. Each change needs a King Solomon research card (0013, "Research
   first"), a run sized by that card, a review by an agent of another
   product and a matching receipt that names the card by hash. The run
   takes the speed measures of AGENTS.md section 6 rule 8 for every hot
   path the file reaches.
   For kernels, the owner also reviews the evidence. For other files,
   the review and the receipt are the proof; the owner may tighten
   this.
6. A language with no safety parse cannot be evolved, so its source
   files cannot change until its parse exists. Rust has one; TypeScript,
   CSS, PHP and SQL each need theirs.

### Hard to skip

1. `gate-wiring`, in `ganit-evolve gate`: once the protected catalogue
   lists the `ganit.every-change` check, a commit may not remove or change that
   entry, drop a tier it runs at, or lower its severity from `block`.
   Adding checks or tiers is allowed. Only an owner-signed change under
   AGENTS.md section 7.2 can loosen it.
2. `ganit.every-change` covers GanitEvolve's own source too, so weakening it
   needs a proved, reviewed receipt.
3. The same wiring protects the forced-speed checks of AGENTS.md
   section 6 rule 8: no commit may remove them, take them off a tier
   they run at, or lower them from `block`. This is the guard. chaukidar
   (0058), which already watches the door's program on Dell, is a
   second line: a change to it made outside git raises an urgent task.

### Build order

1. The `ganit.every-change` check, test first.
2. The whole-file problem, so any Rust file can be evolved.
3. `judge` and judgements in packets; then `score` problems; then helper
   submissions and the agent prompt for fan-out.
4. Guards; then hints; then parallel checks.
5. Safety parses for TypeScript, CSS, PHP and SQL.
6. A first architecture problem, once the architecture checks exist.

## Limits

- Runs use only the driving agent's own usage, on demand. Each run
  declares its budget and stops cleanly when the agent stops; nothing
  runs unattended.
- Evaluations run in sealed containers (no network, no secrets,
  read-only source, the candidate's working copy only) through Dell's
  heavy-job queue: at most two jobs at once, each held to 4 GB and 8 CPU
  threads, no swap. The measuring step runs alone.
- At most 3 candidates in steps 1 to 3 at once.
- Hints are at most 300 characters.
- The archive stays within the Dell cache budgets.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Rust tests in the GanitEvolve crate and ganit:

- A run started from Claude: `next` gives a packet with the parent, two
  to four examples, feedback, the rules and the answer format, and the
  save path.
- A submission that edits a line outside every EVOLVE-BLOCK is refused
  with the line number, and nothing is evaluated.
- A submission identical to an earlier candidate is refused as a repeat.
- A candidate that reads a file or keeps state between calls fails the
  safety step, naming the construct, before any test runs.
- A candidate that passes the public tests but fails a hidden edge case
  (beta = 0 with NaN in the output) scores nothing, and the feedback
  names the failing case without showing the hidden inputs.
- A candidate 30 times faster than the archive best is measured again on
  fresh inputs and believed only if the result holds.
- Timing spread over the limit makes the measurement repeat; it is never
  stored as a result.
- A run started from Codex and stopped resumes from Gemini with its
  archive, islands and budget where they were.
- When the agent stops, no evaluation is left running and the state is
  saved.
- `propose` on a winner gives a change with the patch, new tests,
  benchmark evidence with its fingerprint and a walkthrough, and nothing
  is applied until aegis and review pass.
- Property: for any archive state, parent choice returns only stored
  candidates that passed every hard limit.
- An evaluation killed by the queue (failure injection) is queued again
  once; a second kill marks it "unresolved" with the reason, never a
  pass.
- Evaluator files changed since the run started make the run refuse to
  go on.
- An agent of the driving product cannot judge; another can.
- A judgement shows in the next packet and changes no result.
- A `score` problem keeps a candidate only when it beats the best by the
  smallest gain at the protocol's confidence.
- A candidate that fails a hard limit scores nothing, whatever its
  judges said.
- A helper's answer records the helper and its parent; the parent's
  product drives the run.
- A candidate that gains on the objective but loses more than a guard's
  `worst_loss` fails, naming the guard.
- An unused hint is offered before a used one; after that, the choice
  follows UCB1 and is the same for the same inputs. A packet names its
  hint and the answer records it.
- With `parallel = 3`, at most three candidates are in steps 1 to 3 at
  once, the measuring step never overlaps another, and results are recorded
  in submission order.
- A commit changing a source file without a matching receipt is refused,
  naming the file; with one, it passes.
- A deleted source file, a changed document, a test, benchmark or
  fixture, and the receipts folder's own copies need no receipt.
- Removing or changing the `ganit.every-change` entry, dropping a tier or
  lowering its severity is refused; adding is not.

## Changes from the source

- The maths side is rewritten as Rust in ganit.
- The first kernels and every evolved routine are Rust, in ganit.
- The safety step's forbidden list is restated for Rust, with `unsafe`
  added, as AGENTS.md section 6 rule 6 asks.
- The compiler choice for the old kernels is dropped.
- The run screen and briefing come from the Dioxus app.
- Receipts live in `receipts/`, and source files are the kinds AGENTS.md
  section 6 rule 9 lists; safety parses are needed for those kinds.
- The heavy-job limit follows AGENTS.md section 11, not one job of
  3.5 GB.
- The old 20x goal for hot paths is replaced by the 5% median rule.
- Spring Boot-style checks become Sutradhar Boot's rules.
- `gate-wiring` may be loosened only by an owner-signed change.
- The door runs the gate on the receiving side.

Source: `docs/specifications/ganit-evolve/README.md`,
`docs/specifications/ganit-evolve/ganit-evolve-2.md`,
`docs/specifications/ganit-evolve/ganit-evolve-3.md`,
`docs/specifications/ganit-evolve/ganit-evolve-4.md` and
`docs/specifications/ganit-evolve/ganit-evolve-5.md` in the archive
`archive-2026-11-08-old-repo`.
