# 0012: GanitEvolve

GanitEvolve searches for better code the way DeepMind's AlphaEvolve
does: a language model proposes changes to marked parts of a program,
automatic evaluators score each change, and an archive of good, varied
programs supplies the parents and examples for the next proposals. It
also writes the speed receipt that every new or changed source file must
carry ([AGENTS.md section 6](../../AGENTS.md#6-code-standards) rule 9).
It is roadmap Phase 2 item 6. Language: `ganit-evolve` is a developer
tool written in Go (ADR-0017). ganit stays Rust; the Go tool runs it as
a separate program through a shared schema, never shared code. The proof
standard it shares with the lab is in 0013.

## What it does

### Who does what

1. **The driving agent** (Codex, Claude or Gemini) reads each task
   packet, proposes a change and submits it, inside its own chat and on
   its own usage. Its tokens are the only model cost. GanitEvolve never
   calls a model and holds no key. Nothing runs unless an agent drives.
2. **`ganit-evolve`**, a developer tool written in Go, keeps the run,
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

1. **Build.** Inside the sealed container, in this order: check that the
   candidate's target path lies inside the card's scope, the exact paths
   it lists (0013), and, for a whole-file problem, that the candidate tree
   matches the base outside `target.path`; check every file's hash against
   the input list and each dependency's accepted summary; mount files
   read-only from the copies whose hashes were checked; refuse a
   dependency that uses cgo (`import "C"`); have the validator prove that
   no dependency build script or procedural macro reads a clock, random
   numbers or state kept between runs, and prove, from the source and with
   no summary standing in for that analysis, each dependency build
   script's and procedural macro's reads against `build_reads` and its
   processes and writes against `build_actions` and the directives its
   build script emits against `build_directives`; any link argument,
   native library, library search path or prebuilt object (such as a Go
   `.syso` file) is refused; then, for each build this step makes, run the
   dependencies' accepted build scripts for that build and add the hashes
   of the files they generate to that build's part of the run's input
   list, resolve every `cfg` and `cfg_attr` and every Go build tag, expand
   every macro call, the candidate's and its dependencies' (procedural
   macros run as part of this expansion), and run the safety parse's check
   for forbidden constructs and build-time reads (step 2). A banned token
   inside a macro argument or a `cfg_attr` counts as used. Only then are
   the candidate and every dependency compiled with the pinned toolchain
   and strict warnings, each crate or Go package from its own checked
   expansion, so no macro runs again between the check and the compile; a
   candidate the check refuses is never compiled, so no compiler output
   reaches its feedback. Any warning fails.
2. **Safety.** Parse the source under the accepted problem's safety
   profile, not by text search. The problem's `effects` table names the
   safety profile, `numerical` or `operations` (see "The effect contract"
   below). Numerical code keeps the ban on input and output, files,
   environment variables, system calls, processes, network, clocks, random
   numbers and anything on the problem's forbidden list. Operations code
   uses only the actions and bounds declared in the accepted problem whose
   hash the card's acceptance record names, on a target path step 1 has
   checked lies inside the card's scope; the contract records that same
   checked path. An undeclared or unknown action, or one outside its
   bound, is refused. Both profiles ban state kept between calls (statics,
   thread locals, globals that hold state and Go runtime settings that
   keep a value, such as `debug.SetMemoryLimit`, `debug.SetGCPercent` and
   `runtime.GOMAXPROCS`, and Go `init` functions and package-level
   variables whose initialiser is not a constant, since they run before
   the first timed moment, and state kept through any standard-library or
   dependency API that affects the whole process, such as
   `mime.AddExtensionType`, `expvar.Publish`, `log.SetPrefix`,
   `gob.Register` or a flag the candidate defines itself, except flags a
   command-line entry file (the file that holds `main`) defines inside
   `main` and parses once from the recorded list under a
   `fixture.arguments` grant and never sets again; only a `fixture.state`
   grant, which is reset, may keep state), Rust `unsafe`, `#[no_mangle]`,
   `#[export_name]`, `#[link_section]`, `#[link]`, `extern` blocks,
   `asm!`, `global_asm!` and `naked_asm!`, `cfg(test)`, `cfg!(test)`,
   `#[cfg(test)]` and `cfg(doctest)` outside a base test module, code
   blocks in doc comments the candidate adds or edits, Go's
   `testing.Testing()`, an import of `testing` outside a test file, reads
   of process arguments or flags (such as `os.Args`, `flag.Lookup` or
   Rust's `std::env::args`) other than through a `fixture.arguments`
   grant, which the harness serves by starting the process with exactly
   the recorded list as its whole argument list, the first argument
   included (a fixed recorded name, the same in every build, with no flags
   added by the harness), in every build and test (a process it cannot
   start that way, such as a test binary, gets the list only through the
   grant's adapter, and then every direct read of process arguments or
   flags, such as `os.Args`, `flag.Args`, `flag.Parse`, `flag.Visit`,
   `flag.VisitAll`, `std::env::args` or `std::env::args_os`, is refused
   before any compile; a process that can be served neither way makes the
   problem refused at acceptance and again before launch), and reads of
   caller identity, the call stack, thread names, the executable path, the
   working directory, the process id, the host name, the user id or build
   information (such as Rust's `Location::caller()` and
   `std::thread::current().name()`, or Go's `runtime.Caller`,
   `runtime.Callers`, `debug.Stack` and `debug.ReadBuildInfo`), Go's
   `unsafe` package, `//go:linkname`, hand-written assembly, the `plugin`
   package, `import "C"` and raw system calls. Listing or reading any flag
   the candidate did not define, such as `test.v` through `flag.Lookup`,
   `flag.VisitAll` or `flag.CommandLine`, stays banned even with a
   `fixture.arguments` grant. Both also ban the candidate's own build
   scripts, procedural macros, `env!`, `option_env!` and `//go:generate`,
   always, and its own reads of files while it is built (`include!`,
   `include_str!`, `include_bytes!` and `#[path]` in Rust; `//go:embed` in
   Go), with one exception for those reads only: a read of a file whose
   path resolves inside the candidate tree, never one of the problem's
   fixture files in `effects/` (fixtures reach a candidate only at run
   time, through its grants), with no links and no `..`, and whose hash
   the run's input list names, never one of the hidden evaluation inputs.
   A dependency's build script, procedural macro or declarative macro that
   reads a file or setting while code is built runs only when its hash is
   in the run's input list and its accepted effect summary names every
   file and setting it reads, each held to the same rule: inside that
   dependency's pinned source, with no links and no `..`, named by hash in
   the input list and never a hidden evaluation input. A build-time path
   that cannot be resolved before the compile is refused. The check before
   the compile, and every compile, run in the sealed container and see
   only what the input list names; the hidden evaluation inputs and
   fixture files are not there. Step 1 makes every build: the debug test
   build, the optimised timing build and one build for each feature set
   and Go build tag that a shipped build, hook or CI step uses;
   `ganit-evolve` checks this list at acceptance and again before launch.
   Every build process ends before step 3, and steps 3 and 4 run the built
   program in a fresh sealed container holding only the built output and
   what the input list names, apart from fixture files, pinned tools and
   toolchain binaries, every file mounted read-only from its hash-checked
   copy, with no writable mount. Scratch space stays outside it too. The
   candidate never reaches a toolchain binary, and reaches the others only
   through its fixture adapters (the `fixture.write` adapter holds scratch
   space, and a numerical problem has none), which run outside it and
   enforce each grant's bound. Each adapter, with any pinned tool it runs,
   runs in its own sealed container holding only the resource its grant
   names, with no network and no secrets; it reads fixtures, records and
   its own programs only from read-only copies whose hashes were checked
   before launch, and scratch space is its only writable mount. Adapters
   get no host environment, and pinned tools get only their record's typed
   slots. The candidate's container filters its system calls so it can
   make only the calls its container allows. The hidden evaluation inputs
   stay with the harness outside that container, which sends each case in
   as data over a pipe and reads the answer back; the built program can
   never open them. Each test case and each timing repeat gets a fresh
   process. For each test case and each timing repeat, the harness reads
   at most the problem's largest answer per case, which must fit within
   the job's memory limit or the problem is refused at acceptance; it
   discards the program's error stream beyond that same bound. An answer
   longer than the largest answer per case fails the candidate, and so
   does a case or repeat not answered within the problem's per-case time
   limit (answered means its process has exited with its answer), at which
   the harness stops it; so a program that writes without end or never
   answers cannot exhaust or hang the harness. `ganit-evolve` checks and
   pins the problem, card, safety profile and sandbox settings before
   launch. A candidate cannot select, replace or widen its safety profile;
   a missing or stale acceptance is refused. Both profiles keep the sealed
   container and every language, privilege and privacy check. No
   candidate's code may start a shell or run Python: AGENTS.md section 4
   bans programs of ours starting a shell, and Python is not an allowed
   language. A kept shell file is evolved only once its own safety parse
   exists ("A receipt for every source change", item 6). No declaration
   can authorise what those checks or the problem's forbidden list forbid.
3. **Correctness.** Public tests, then hidden tests the agent never sees,
   with fresh random inputs each time, compared with the reference within
   the problem's tolerance. A debug build with overflow and bounds checks
   runs them, and the optimised timing build and every other build step 1
   makes run the same public, hidden and edge-case tests, so a candidate
   that behaves differently in any of those builds fails. The reference
   outputs and the comparison stay with the harness, outside the
   candidate's process, and only the harness writes feedback on a hidden
   case: the candidate's own output, errors and panic text from a hidden
   case never reach the feedback. The same holds for step 4's runs on
   generated inputs.
4. **Measuring.** For a `speed` problem, GanitEvolve times an optimised
   build, outside the candidate, three times, keeping the median. The two
   times are taken by pinned timing code the candidate cannot change: for
   an EVOLVE-BLOCK problem, the fixed part of the program, which reads the
   whole input into memory before the first time and writes the answer
   after the second (a pinned harness role whose input, output and timing
   code sits outside the safety profile's check; code inside an
   EVOLVE-BLOCK may not call or refer to it, and every name the fixed part
   uses resolves to a fully qualified pinned path, with no glob imports;
   the fixed part sits in its own inline module that imports nothing from
   the blocks; checked after macro expansion, every method call it makes,
   including one a macro generates (so the fixed part writes its answer as
   `<W as std::io::Write>::write_fmt(&mut w, format_args!(..))`, not with
   `writeln!`), is written in fully qualified form (such as
   `<std::time::Instant>::elapsed(&start)`), with trait methods as `<T as
   Trait>::method` (such as `<W as std::io::Write>::write_fmt`); and a
   block may not declare a trait, or any impl, generic and blanket ones
   included, that can apply to a type the fixed part uses, nor any impl
   whose self type is a type parameter or a reference to one, so no block
   item or macro other than the declared entrypoints can reach it); for a
   whole-file problem, the pinned benchmark harness. The second time is
   taken only after pinned code has fully encoded the answer into memory
   and every thread or goroutine the candidate started has ended. So the
   timed part excludes the pipe and the read-back, and no candidate signal
   moves either time. The harness also compares each timed answer with the
   reference within the problem's tolerance, and a mismatch fails the
   candidate. Resets and the set-up of fixtures and pinned tools happen
   outside the timed part, and before every timed call the pinned code
   passes each input or in-out buffer newly allocated and holding a fresh
   copy of the case's input, and each output-only buffer newly allocated
   and zero-filled, each with capacity equal to its length, so nothing
   from an earlier call can be reused. A run whose spread is over the
   limit is repeated, never accepted. Timing runs alone in the heavy-job
   queue, with its fingerprint (0007). ganit keeps a gain only if it beats
   the noise at 95% confidence and the 5% rule of 0007. A `score` problem
   runs its scoring step here instead. Guards are checked in this step
   (below).
5. **Finalists only.** The full aegis gate on the would-be change.

`evaluation.parallel` (1 to 3, default 1) sets how many candidates may
run steps 1 to 3 at once. The measuring step always runs alone, so
timings are not shared with another candidate's work. Results are
recorded in submission order, whatever finishes first. Every step goes
through Dell's heavy-job queue.

### The effect contract

The hidden evaluation inputs are the evaluator's files, the reference, the
input generator, the hidden tests and the fresh-input seed. Every check
against them compares content hashes, not paths, so a copy at another path
is still a hidden evaluation input. A build-time read of any file under a
problem folder or the evaluator is refused by path as well; this refusal
wins over the candidate-tree read exception, and only the target file
itself is read from a problem folder. A fixture file, fixture adapter or
pinned tool written or edited by a product that wrote or edited a hidden
evaluation input is refused. The records below live in the problem's
`effects/` folder ("A problem definition"), one file each. The safety
validator checks each record against its schema; fixture files are files,
not records, have no schema and are checked by content hash only:

- An effect summary (`ganitevolve-effect-summary/v1`) covers one
  dependency, procedural macro or pinned tool: `subject` (its name and its
  source and tool hashes), `build_reads` (every file and setting it reads
  while code is built), `build_actions` (every process it starts while
  code is built, each a pinned tool under the rules below; it writes only
  to the build's own output folder), `build_directives` (every directive
  its build script emits to the build tool) and `actions` (every action it
  can take while running, each with a resource and bound as in a grant; a
  summary describes, and only a grant authorises).
- A fixture adapter (`ganitevolve-fixture-adapter/v1`) is the pinned
  harness code that serves one grant to the candidate and enforces its
  bound: `action`, `resource`, `bound`, `program` (its hash) and `depends`
  (the hashes of what it depends on). Its program is our own Go program
  with its receipt (or, until its receipt is made, an adapter admitted as
  build order item 2 says), never a shell, an interpreter or a compiler.
  Like a pinned tool, it cannot see the hidden evaluation inputs, reaches
  only the resource its grant names, starts empty or from its fixture for
  each test case and timing repeat, and shows nothing in the feedback.
- A pinned tool record (`ganitevolve-pinned-tool/v1`): `program` and
  `depends` (hashes), and typed `arguments` and `environment` slots.
- Sandbox settings (`ganitevolve-sandbox/v1`): the runtime, sandbox and
  role settings.
- Resource and bound records (`ganitevolve-resource/v1`,
  `ganitevolve-bound/v1`): what each grant points to, with a typed field
  for each resource and bound the action table below lists.
- Fixture files (not records): the unchangeable inputs a `fixture.read`,
  `fixture.environment`, `fixture.clock`, `fixture.random` or
  `fixture.arguments` grant serves.

The accepted problem decides what a candidate may do while it is
evaluated. Its `effects` table has exactly `schema`, `profile`,
`entrypoints` and `grants`. The schema is `ganitevolve-problem-effects/v1`;
the profile is `numerical` or `operations`. Entrypoints are resolved
source symbols in the exact build, not guesses from file names.
Numerical grants are empty. Operations grants hold only the fixture
actions the problem needs. The candidate supplies neither this table
nor its acceptance.

Each grant has exactly `action`, `resource` and `bound`. Resource and
bound point into the same accepted problem; they are not paths or
limits a candidate supplies. Every reference resolves to a typed
record covered by the problem's hash. Bounds are finite, checked by
machine and enforced outside the candidate. The only actions are:

| Action | Resource and bound |
| --- | --- |
| `fixture.read` | Unchangeable fixture files; allowed names, calls and bytes |
| `fixture.write` | Private scratch space; allowed names, calls and bytes |
| `fixture.call` | Pinned fixture tool; typed arguments, calls and output |
| `fixture.environment` | Fixture keys and values; allowed keys and reads |
| `fixture.clock` | Recorded clock input; reads and returned range |
| `fixture.random` | Recorded seed and algorithm; calls and bytes |
| `fixture.state` | Private job state; operations, bytes and a lifetime of at most one test case or timing repeat |
| `fixture.arguments` | Recorded argument list of typed values, set by the harness before each case starts; count and bytes, fresh for each test case and timing repeat |

A tool resource pins its program and what it depends on by hash. A pinned
tool is either an entry in `gate/tools.toml` whose program hash matches
that entry and that the field `fixture_callable = true` marks as callable
from a fixture (a missing field means false, and aegis's tools list reader
accepts the field), or our own Go program with its receipt (AGENTS.md
section 4). For a `gate/tools.toml` entry, the acceptance names the hash
of that entry, flag included, and a false flag is refused at acceptance
and again before launch. A pinned tool is never a shell, an interpreter, a
compiler or any program that runs code it is given, and it never reads
code, settings or other files from scratch space the candidate can write.
It cannot see the hidden evaluation inputs. Its argument and environment
slots take only typed fixture values. Calls keep the job's resource and
time limits. Scratch space, a state resource and anything a pinned tool
keeps start empty or from their fixture for each test case and each timing
repeat, end with it, and never reach the feedback. File names stay inside
their fixture or scratch folder, with no links and no `..`. A wildcard
resource, an open-ended limit, an unknown action or an unresolved
reference is refused.

These permissions apply only inside the sealed container. There is no
network action: input that stands in for the network comes only through a
listed grant, such as `fixture.read`. No grant may name a hidden
evaluation input, and `fixture.random` never uses the fresh-input seed. A
grant cannot open a real connection, expose credentials, edit a site, add
a privilege or get round a forbidden construct. The pinned runtime and
test harness set up the job and handle its output; those roles are fixed
in the run's input list, and candidate code, or anything it can call,
cannot take them by a name, path or note. A dependency of numerical code
gains no action through the harness. Production sign-in routes and all
owner and moderator controls stay apart from evaluation.

The card's acceptance record ([0013](0013-research-rules.md)) names the
hashes of the card, the accepted problem, its sandbox settings, its effect
summaries, fixture adapters, pinned tool records, resource and bound
records, the fixed part of the program and every harness role, the
`gate/tools.toml` entry of each pinned tool, the contents of its fixture
files, and the pins of the safety validator, analysers and compilers it
uses. The problem's hash covers `problem.toml` and every file in its
`effects/` folder. That binds the safety profile, and everything that
decides what a candidate may do, to an accepted card. For every problem,
the accepting agent's product differs from every product that wrote or
edited the card or any file the problem's hash or the acceptance names
(`problem.toml`, the fixed part of the program and every harness role,
sandbox settings, summaries, adapters, pinned tool records, resource and
bound records, the `gate/tools.toml` entry of each pinned tool, fixture
files and pinned tools), since each was made or last accepted. Before each
candidate launches, `ganit-evolve` checks that it also differs from the
product driving the run at that moment and from every product that has
written or submitted a candidate in the run, since models may mix; a
helper counts as its parent's product. A launch is also refused when the
driving product, or any product that has written or submitted a candidate
in the run, wrote or edited a hidden evaluation input. An effect summary
is likewise accepted only by a product that did not write it and is not
driving the run, and `ganit-evolve` checks every summary's acceptor
against the driving product and the candidate writers at each launch, as
it does the card's acceptor; a summary never stands in for the validator's
own analysis of source that is in the input list. If no product is left to
accept, the problem cannot run. No other approval is needed for the safety
profile, and an acceptance without those hashes, or a description in
prose, supplies no safety profile.

After step 1's builds and before step 3, `ganit-evolve` records a
contract with exactly these fields:

```json
{
  "schema": "ganitevolve-effect-contract/v1",
  "acceptance": "<card acceptance record hash>",
  "target": {
    "candidate": "<candidate tree hash>",
    "path": "<exact relative source path>",
    "tested": "<tested source bytes hash>"
  },
  "inputs": "<run input list hash>"
}
```

The contract repeats no permission list: the acceptance binds the card,
the problem and the sandbox settings hash. The run's input list names,
with its hash, everything the candidate's check reads: its source and
every source file, dependency and tool it can reach, generated source,
build target and features, lock files, compiler and analyser pins, effect
summaries, fixture adapters, pinned tool records, fixture files and the
harness's roles. The run has one input list with a part for each build:
the part known before building is checked at the start of step 1, step 1
adds each build's generated files to that build's part, and the contract's
`inputs` is the hash of the whole list. Every link between them is
resolved; a lock file or a list of direct imports alone is not enough.

The sandbox settings hash covers the accepted runtime, sandbox and role
settings, and nothing about a candidate. `ganit-evolve` refuses to launch
a candidate when the settings it is about to use hash to a different value
from the one the acceptance names. Each candidate gets its own contract;
an earlier candidate's results do not carry over. Changing anything the
acceptance names needs a new acceptance. In the effects table, the
contract and every record in `effects/`, unsupported versions, repeated,
unknown, missing or null fields and malformed hashes are refused, with no
defaults.

The pinned safety validator follows calls and aliases through everything
in the input list, including indirect calls, callbacks, interfaces and
dependencies, for every build step 1 makes (step 2 lists them). It proves
that each possible action is declared and bounded, or refuses the
candidate. Unknown behaviour and a partial analysis are failures. A list
of imports or calls found by syntax alone, a signed declaration and
sampled traces from a run cannot prove that there are no other actions.
Accepted summaries for dependencies are checked against the exact source
and tool hashes; a library's name or admission grants nothing.

The safety step's raw output is finished once the contract is recorded. It
holds the contract hash, the input list hash and the proof for each action
and bound, and its pinned validator checks them. A receipt for an
operations candidate names its contract hash; a contract alone earns no
receipt. These records go in the archive, within its cache budgets.

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
   question. For hidden cases and step 4's generated inputs, the archive
   keeps only what the harness writes, never the candidate's own output,
   errors or panic text.
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

Each problem lives in its own folder under the GanitEvolve Go module:

1. `problem.toml`: name, language, the one source file (its `target.path`)
   and its EVOLVE-BLOCK regions, the hashes of the evaluator's tests, the
   reference and the input generator, tolerances, the public test sets and
   the names and hashes of the hidden test sets, which live in their own
   files, objectives (the main one first), diversity measures, guards,
   forbidden constructs, the `effects` table (its safety profile and
   grants), the largest answer per case (within the job's memory limit)
   and the per-case time limit, both required and never zero, the run
   budget defaults and the timing settings.
2. `effects/`: the records "The effect contract" lists, one file each,
   all named by hash in the card's acceptance record.
3. The program: an ordinary source file whose fixed parts (the
   interface, argument checks and bindings) sit outside the blocks, so
   the calling form can never change.
4. The evaluator's tests, the hidden test set files, the reference and the
   input generator, each pinned by hash, in the problem's folder. Before
   each launch `ganit-evolve` checks each of them against the hash
   `problem.toml` names, and refuses to launch on a mismatch.
5. Objectives are explicit numbers, for example "median seconds for the
   shape set" (lower is better) with "largest scaled error" as a hard
   limit. A candidate that misses a hard limit scores nothing.
6. A whole-file problem needs no EVOLVE-BLOCK markers and uses the crate's
   or Go module's own tests as the correctness step, so any Rust or Go
   file that meets the rules below can be evolved. Those tests are taken
   from the pinned base, never from the candidate's tree, and only tests
   that reach the candidate through its public interface count (Rust
   `tests/`, Go external `_test` packages). A target file's inline
   `#[cfg(test)]` test module, if the base has one, is taken unchanged
   from the base and lies outside the part the candidate may edit; it does
   not count as a public-interface test. A candidate that adds or edits a
   test module, or defines, imports, re-exports or renames any item or
   macro into a name in the Rust prelude, into a macro std or core exports
   (such as `assert_eq`, `matches` or `vec`), or into any other name the
   base's tests resolve outside the crate, is refused. Whole-file problems
   still need step 3's hidden tests. The pinned harness runs each case in
   its own process over the pipe. A candidate tree that differs from the
   base outside `target.path` is refused at the start of step 1. In a
   whole-file problem the state ban in step 2 and the name ban above apply
   to what the candidate adds or changes compared with the pinned base
   (every other ban applies to the whole file); a construct the base
   already has and the candidate keeps unchanged, such as a Rust immutable
   `static` lookup table, a Go `var ErrX = errors.New(..)` or the base's
   own `Result` alias, is not refused for being there. The exemption
   covers a base construct only when all three hold: it cannot hold
   mutable state (in Rust, its type has no `UnsafeCell` anywhere inside,
   following references and pointers, and a trait object or type parameter
   counts as possibly holding one, which rules out atomics, cells, locks,
   `OnceLock`, `LazyLock` and types wrapping them, and it is not `static
   mut` or `thread_local!`; in Go, which has no immutable variables, only
   a string, a number, a boolean, or an error value made by `errors.New`,
   or by `fmt.Errorf` with a constant format, constant string or number
   arguments and no `%w`); the validator proves that nothing reachable
   from the entrypoints, in base or candidate code, writes it, borrows it
   mutably or makes a mutating call on it after its initialiser; and its
   initialiser, with everything it calls, is unchanged. Every other base
   static, thread local or package-level variable is refused, including a
   Go slice, array or map table the candidate keeps unchanged, and so is a
   base Go `init` function; added or changed code that refers to such a
   variable, or takes the address of or slices a base Go package-level
   variable, is refused (a Rust shared borrow of an exempt static stays
   allowed, since its type has no `UnsafeCell`). So a Go file with a
   package-level table cannot be a whole-file problem; it can be evolved
   through EVOLVE-BLOCK regions instead.

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

1. Every new or changed source file in a commit must be the
   exact program a GanitEvolve run proved. The proof is a receipt in
   `receipts/` whose `target` is the file and whose `tested` is the
   SHA-256 of its new bytes.
2. A source file is any file of a kind AGENTS.md section 6 rule 9
   names, whether it ships or not. Tests, benchmarks and test data
   go with the receipt of the code they test (AGENTS.md section 6
   rule 9). A file counts as a test, benchmark or test data only when
   nothing but test or benchmark targets include, load or run it, as
   the gate reads it from the build: a Rust file that only a test or
   benchmark target compiles, or a file of another kind under a
   `tests/`, `benches/` or `fixtures/` folder that only tests read.
   A file a hook, the gate, CI, an install step or a shipped build
   runs or loads needs its own receipt, wherever it sits.
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
6. A language with no safety parse cannot be evolved, and until its
   parse exists its files need no receipt. AGENTS.md section 7.3 sets
   when `ganit.every-change` starts to cover each language. Each
   language needs its own parse: Rust, Go, TypeScript, CSS, PHP, SQL,
   shell, Cedar and Rego.

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
2. The Rust safety parse, with the pinned effect validator for both safety
   profiles, the operations profile's fixture actions and their
   enforcement, and the whole-file problem for Rust files. Then, in one
   later commit, the whole-file problem for Go files and the Go safety
   parse with the same for Go, so a Go receipt can be made from the commit
   that first requires one (AGENTS.md section 7.3). Which Go source may
   land before that commit is set in AGENTS.md section 4. The same commit
   makes aegis's tools list reader accept `fixture_callable`, with an
   aegis test that the field is accepted and a missing one means false.
   Problems with grants run only from that later commit, the one that adds
   the Go safety parse: the first fixture adapters, which land before that
   commit as GanitEvolve's own Go source and have no receipt yet, are
   admitted once, by a product other than their writer, pinned by hash in
   the problem's acceptance, and lose that admission at their first
   change: a changed adapter needs its own receipt, made under a problem
   whose adapters are all admitted or receipted, never one the changed
   adapter serves, before any later launch uses it.
3. `judge` and judgements in packets; then `score` problems; then helper
   submissions and the agent prompt for fan-out.
4. Guards; then hints; then parallel checks.
5. Safety parses for TypeScript, CSS, PHP, SQL, shell, Cedar and Rego.
6. A first architecture problem, once the architecture checks exist.

## Limits

- Runs use only the driving agent's own usage, on demand. Each run
  declares its budget and stops cleanly when the agent stops; nothing
  runs unattended.
- Evaluations run in sealed containers (no network, no secrets, read-only
  source, the candidate's working copy and no writable mount; toolchain
  binaries, and the scratch space, fixtures, adapters and pinned tools its
  accepted grants name, each adapter in its own sealed container holding
  only its grant's resource, and the hidden evaluation inputs, stay
  outside the candidate's container) through Dell's heavy-job queue: at
  most two jobs at once, each held to 4 GB and 8 CPU threads, no swap. The
  measuring step runs alone.
- At most 3 candidates in steps 1 to 3 at once.
- Hints are at most 300 characters.
- The archive stays within the Dell cache budgets.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Go tests in the GanitEvolve module and Rust tests in ganit:

- A run started from Claude: `next` gives a packet with the parent, two
  to four examples, feedback, the rules and the answer format, and the
  save path.
- A submission that edits a line outside every EVOLVE-BLOCK is refused
  with the line number, and nothing is evaluated.
- A submission identical to an earlier candidate is refused as a repeat.
- A numerical candidate that reads a file or keeps state between calls
  fails the safety step, naming the construct, before any test runs.
- A numerical candidate's file, network and process actions stay
  refused when its source, comments or submission claim the operations
  safety profile.
- A candidate whose target path lies outside the card's scope is refused
  at the start of step 1, before anything is built.
- A launch whose driving product, or a product that has written or
  submitted a candidate in the run, wrote or edited a hidden evaluation
  input is refused, including after a resume; a helper counts as its
  parent's product.
- A dependency that uses cgo is refused before anything is built.
- A dependency build script or procedural macro that reads a clock, random
  numbers or state kept between runs is refused before anything is built.
- An operations candidate's declared fixture action works only inside
  the accepted bounds and card scope, with every other check passing.
  The same action without its declaration is refused.
- An unknown action or unresolved effect analysis blocks evaluation;
  it is never treated as harmless or replaced by a smaller safety
  profile.
- Exceeding a declared action's bounds refuses the candidate. A
  declaration does not make a sandbox escape, private-data access or
  forbidden privilege acceptable.
- Candidate-selected profiles, edited bounds, substituted problems and
  widened card scope are refused before launch. A stale card, problem or
  sandbox settings hash needs a fresh acceptance.
- A declared shell or Python action stays refused, and a pinned tool
  that is a shell, an interpreter or a compiler, or that reads code or
  settings from writable scratch space, is refused before launch.
  Neither safety profile can switch off the sealed container or a
  language, privilege or privacy check.
- An action on the problem's forbidden list is refused even when its
  declaration is otherwise in scope.
- Missing safety profile enforcement or a required tool blocks the
  evaluation; no unmeasured action, skipped check or bare declaration
  earns a pass.
- An accepted numerical problem has empty grants. A pure kernel passes
  its effect check; a dependency or alias adding file, environment,
  clock, random, process, network or retained-state effects fails it.
- A bounded fixture operation passes only for its exact accepted
  problem, card, source and input list. Each changed hash, a
  stale acceptance (its card was edited), or a contract made for
  another candidate is refused before the candidate runs.
- Each grant action is tested at its accepted bound and beyond it.
  Unknown actions, wildcard resources, missing bound references,
  open-ended limits and bounds that cannot be enforced fail.
- Aliased process calls, callbacks, interface dispatch and dependency
  calls with unresolved effects fail. A partial analysis result cannot
  pass, even when it reports no findings.
- Changing a transitive dependency, generated file, build feature,
  effect summary, adapter, tool or harness role makes the input list
  stale. Leaving out a link fails instead of shrinking the scope.
- A fixture label cannot authorise a real credential read, network
  connection, site edit or sandbox escape. A candidate labelled as a
  harness remains untrusted. Production authorisation grants no
  evaluation access; owner and moderator controls remain enforced.
- A launch whose sandbox settings hash differs from the one its card
  acceptance names is refused before the candidate runs.
- An operations problem with a grant that reaches the evaluator, the
  reference, the input generator, the hidden tests or the fresh-input
  seed is refused before acceptance and again before launch.
- An acceptance record without the problem and sandbox settings hashes
  is refused.
- An acceptance of any problem, numerical or operations, is refused when
  its product wrote or edited the card or any file the problem's hash or
  the acceptance names, such as `problem.toml`, the fixed part, a harness
  role, sandbox settings, a summary, an adapter, a pinned tool record, a
  resource or bound record, a pinned tool's `gate/tools.toml` entry, a
  fixture file or a pinned tool, since that was made or last accepted,
  including on the first acceptance.
- For any problem, a candidate is refused before launch when
  the product driving the run at that moment, any product that has
  written or submitted a candidate in the run, or a helper's parent, is
  the accepting product, including after the run is resumed by another
  agent or models mix.
- For a numerical problem too, an effect summary accepted by the
  product that wrote it or that drives the run is refused, and a
  summary calling a dependency pure does not stop the validator finding
  that dependency's file or process actions in the input list.
- An effect summary, fixture adapter, pinned tool record, resource or
  bound record, fixture file, safety validator, analyser or compiler
  whose hash the acceptance does not name is refused before launch, and
  so is any change to `problem.toml` or a file in `effects/` after
  acceptance.
- A pinned tool record whose program is neither an entry in
  `gate/tools.toml` with a matching hash and `fixture_callable = true` nor
  our own Go program with its receipt is refused before launch.
- A fixture file, fixture adapter or pinned tool written or edited by a
  product that wrote or edited a hidden evaluation input is refused at
  acceptance and again before launch, so a near-copy of a hidden input
  cannot reach a grant. A numerical problem whose driving product wrote or
  edited a fixture file, and accepted the card, is refused; a fixture
  whose content hash matches a hidden evaluation input is refused at any
  path.
- A fixture or scratch name with `..` or a link is refused before
  launch, and a build-time read through `..` or a link to the hidden
  tests is refused before any compile.
- A pinned tool that can read a hidden evaluation input is refused
  before launch; one that passes the job's resource or time limits is
  stopped and the candidate fails; an argument or environment slot
  given anything but a typed fixture value is refused.
- A dependency's build script, procedural macro or declarative macro
  that reads the hidden tests while code is built finds nothing there
  and fails the build; one whose build-time reads its accepted effect
  summary does not name is refused before any compile; a build-time
  path that cannot be resolved before the compile is refused; a
  dependency whose summary names only files in the input list builds.
- Nothing written to scratch space or kept by a pinned tool or a
  `fixture.state` grant appears in the feedback.
- A receipt for an operations candidate names its contract hash; an
  operations receipt without one is refused.
- A `problem.toml` with no largest answer per case or per-case time
  limit, with either set to zero, or with a largest answer that does not
  fit within the job's memory limit, is refused at acceptance.
- `unsafe` or `#[no_mangle]` under `#[cfg(not(debug_assertions))]`, under
  one feature, or under one Go build tag is refused for the build that
  would compile it.
- An effects table, contract or record from `effects/` (other than a
  fixture file) with an unsupported schema version, a repeated, unknown,
  missing or null field (such as an extra `unlimited = true` in a bound
  record) or a malformed hash is refused.
- A candidate that uses `include_bytes!` on the hidden tests, or another
  build-time read the exceptions do not cover, is refused before any
  compile, and no compiler output reaches its feedback.
- Code that reads a file or setting while it is built is refused under
  both safety profiles unless the read resolves inside the candidate tree
  (or, for a dependency, its pinned source), has no links and no `..`, is
  named by hash in the run's input list and is none of the hidden
  evaluation inputs, with a dependency's reads also named in its accepted
  effect summary; a crate file that derives with a pinned macro or
  includes a listed file can still be evolved, and a pinned macro whose
  summary declares a read of the hidden tests is refused.
- Under both safety profiles, Rust `unsafe`, `#[no_mangle]`,
  `#[export_name]`, `#[link_section]`, `#[link]`, `extern` blocks, `asm!`,
  `global_asm!` and `naked_asm!`, and Go's `unsafe` package,
  `//go:linkname`, hand-written assembly, the `plugin` package, `import
  "C"` and raw system calls are each refused before any compile, naming
  the construct.
- A fixture adapter that opens a hidden evaluation input, or a resource
  outside its grant, is refused before launch or fails the candidate.
- A dependency's build script or macro that starts a process its summary's
  `build_actions` does not name, or writes outside the build's output
  folder, is refused before any compile; a process left running by a build
  is gone before step 3 and cannot read the hidden tests.
- Under both safety profiles, a candidate that keeps a value in a
  static, a thread local, a global or a Go runtime setting
  (`debug.SetMemoryLimit`, `debug.SetGCPercent`, `runtime.GOMAXPROCS`)
  between calls is refused, naming it; only a `fixture.state` grant may
  keep state, and it is reset.
- Under both safety profiles, a build script, a procedural macro, `env!`,
  `option_env!` or `//go:generate` in the candidate's own code is always
  refused before any compile, naming it; `include!`, `include_str!`,
  `include_bytes!`, `#[path]` and `//go:embed` are refused the same way
  unless the read exception covers that read.
- A fixture adapter whose program is not our own Go program with its
  receipt, and was not admitted as build order item 2 says, is refused
  before launch.
- A candidate whose answer is longer than the problem's largest answer per
  case fails. A candidate that writes without end to its error stream has
  the excess discarded and fails at the per-case time limit, and so does
  one that does not answer a hidden case or timing repeat in time,
  including one that writes its answer and then keeps running; the harness
  stays within its limits. A value kept in memory by one case's process is
  gone in the next case.
- The measured time stays within step 4's measured spread when only the
  size of the input sent over the pipe changes and the work is the same,
  and a candidate that signals readiness early or writes a fake end mark
  gets no shorter measured time; a candidate that returns a lazy result
  whose work happens while it is encoded, or returns while a background
  thread or goroutine finishes the answer, gets no shorter time, and fails
  if the answer is incomplete when the harness reads it.
- A candidate that is correct in the debug build but gives a wrong answer
  in the optimised timing build, for example through
  `cfg!(debug_assertions)`, fails.
- A dependency build script that emits a link argument, a native library
  or a library search path, or a dependency package holding a prebuilt
  object such as a Go `.syso` file, is refused before the candidate is
  compiled; so is a directive its summary's `build_directives` does not
  name.
- A procedural macro that gives clean output to the check and `unsafe` or
  `#[no_mangle]` output on a later run gains nothing: the compile uses the
  checked expansion and the macro does not run again.
- A banned construct that appears only after the candidate's own
  `macro_rules!` macro is expanded, one hidden behind
  `#[cfg_attr(all(), no_mangle)]` or another always-true `cfg_attr`, and
  `unsafe` code passed through a dependency's pass-through macro are
  each refused before any code is generated.
- A candidate with no `fixture.write` grant, or one past its grant's
  bound, cannot write to scratch space, and a direct write in its
  container is stopped.
- A dependency macro called on a fixture path, or a dependency build
  script that reads a fixture file, finds nothing there and fails the
  build; fixture files are not in any build container.
- In a whole-file problem, a base file with an inline test module can be
  evolved: the module is kept unchanged from the base and does not count.
  A candidate that adds or edits a test module, or defines its own
  `assert_eq`, is refused, and only the base's public-interface tests and
  the hidden tests count.
- Under both profiles, `cfg(test)`, `cfg!(test)` or `#[cfg(test)]` in the
  candidate's Rust outside a base test module, and `testing.Testing()` or
  an import of `testing` outside a test file in its Go, is refused before
  any compile, naming the construct.
- Under both profiles, a read of process arguments or flags, such as
  `os.Args`, `flag.Args` or `std::env::args`, is refused before any
  compile unless a `fixture.arguments` grant serves it; with that grant, a
  command-line entry file such as `main.rs` or `main.go` reads only the
  recorded, bounded argument list, fresh for each case. Listing or reading
  a flag the candidate did not define, through `flag.Lookup("test.v")`,
  `flag.VisitAll` or `flag.CommandLine`, is refused with or without the
  grant.
- With a `fixture.arguments` grant and a process the harness starts with
  the recorded list, a candidate learns neither the test build nor the
  executable path from `flag.Lookup("test.v")`, `os.Args[0]` or
  `std::env::args().next()`: the first is refused, and the others return
  the recorded list's first value.
- When the list reaches a test binary through the grant's adapter, a
  direct read such as `os.Args`, `flag.Visit` or `std::env::args` is
  refused before any compile.
- A problem with a `fixture.arguments` grant whose process can be started
  neither with the recorded list nor through the adapter is refused at
  acceptance and again before launch.
- A block impl whose self type is a type parameter or a reference to one,
  such as `impl<W: DerefMut<Target = Vec<u8>>> Enc for W`, is refused
  before any compile.
- A doc-comment code block the candidate adds or edits, or `cfg(doctest)`
  outside a base test module, is refused before any compile, naming it.
- A candidate that gives a wrong answer only under one feature set or Go
  build tag fails the tests that build runs.
- A problem that lists more than one source file is refused at acceptance.
- A problem with grants is refused before the commit that adds the Go
  safety parse. After it, a fixture adapter without a receipt is refused
  unless it was admitted once by a product other than its writer and is
  pinned by hash in the acceptance.
- Code inside an EVOLVE-BLOCK that calls or refers to the fixed part's
  input, output or timing code is refused before any compile, and that
  fixed code does not make a numerical speed program fail its profile
  check.
- A block that defines its own `Instant` or `write_fmt` reached by the
  fixed part's names, a block trait or impl for a type the fixed part uses
  (such as a by-value `elapsed` on `Instant`), a fixed part with a glob
  import, or a fixed-part method call not in fully qualified form, is
  refused before any compile.
- A block with a blanket impl, such as `impl<T> Hijack for T { fn
  write_fmt(self, ..) }`, that reaches the fixed part's `write_fmt` call
  is refused before any compile, and so is a fixed-part trait method call
  not written as `<T as Trait>::method` after macro expansion.
- An edit to the fixed part or a harness role after acceptance is refused
  before launch.
- A candidate that brings `assert_eq` or another prelude, std or core name
  into scope by defining it, `pub use`, import or rename, such as its own
  `matches!` or a public `fn Some`, is refused.
- A launch is refused when an effect summary's acceptor has become the
  driving product or a candidate writer, including after a resume.
- A `gate/tools.toml` entry without `fixture_callable = true`, such as the
  compiler, is refused as a pinned tool at acceptance and again before
  launch.
- Changing a pinned tool's `fixture_callable` to false, or editing its
  `gate/tools.toml` entry's program hash, after acceptance makes the
  acceptance stale, and the launch is refused.
- An acceptance by the product that set `fixture_callable = true` on a
  pinned tool's `gate/tools.toml` entry is refused.
- A candidate that embeds `problem.toml`, a hidden test file or any other
  file under a problem folder or the evaluator, for example with
  `//go:embed problems/<p>/problem.toml`, is refused before any compile.
- In a whole-file problem, a base file that keeps a Rust immutable
  `static` lookup table, a Go `var ErrX = errors.New(..)` or its own
  `Result` alias unchanged passes the state and name bans, while the same
  construct added or changed by the candidate is refused, and an `unsafe`
  block the base already has is still refused; a base Go slice, array or
  map table is refused even when unchanged.
- In a whole-file problem, a candidate that reassigns the base's `ErrX`,
  stores into a base atomic, or caches into a base package-level map is
  refused before any compile, and so is a base's unchanged mutable global;
  so is changed code that calls an unchanged base function which writes a
  plain base `var counter int`, or that calls an unchanged base setter
  such as `SetMode` and then reads the value it set.
- In a whole-file problem, a candidate that changes a function a base's
  unchanged package-level initialiser calls, such as `build` in `var t =
  build()`, is refused before any compile.
- In a whole-file Rust problem, a base `static HITS: Hits(AtomicU64)` that
  wraps state in a type of its own is refused, like a bare atomic, and so
  is a base `static S: &AtomicU64`, and so is a base `static S: &dyn Any`.
- In a whole-file Go problem, a base error value made with `fmt.Errorf`
  using `%w` or a non-constant argument is refused.
- Under both profiles, a candidate that keeps state through a process-wide
  standard-library or dependency API, such as `mime.AddExtensionType` then
  `mime.TypeByExtension`, or a flag it defines with `flag.String` and
  later sets or reuses as a store, is refused before any compile; a
  command-line entry file that defines its flags inside `main` and parses
  them once from the recorded list under a `fixture.arguments` grant
  passes, while one that defines them as package-level variables such as
  `var n = flag.Int(..)` is refused.
- A candidate that keeps a marker in its output buffer, or in an input or
  output slice's spare capacity, to return early on a later timed call
  gains nothing: each timed call gets newly allocated buffers whose
  capacity equals their length, inputs and in-out buffers holding a fresh
  copy of the case's input and output-only buffers zero-filled.
- In a whole-file Go problem, a candidate that takes the address of an
  exempt base variable and writes through it, such as base `var mode = 1`
  with changed code `p := &mode; *p = 2`, is refused before any compile.
- A hidden test set file, the evaluator's tests, the reference or the
  input generator whose bytes no longer match the hash `problem.toml`
  names is refused before launch.
- The archive record of a candidate that prints a hidden input in its
  output or panic text holds none of that text.
- An admitted fixture adapter with no receipt that has changed since its
  admission is refused before launch until its changed file has its own
  receipt, made under a problem whose adapters are all admitted or
  receipted, never one the changed adapter serves; a receipt made under a
  problem whose adapters are not all admitted or receipted, including one
  the changed adapter serves, is refused.
- Under both profiles, a read of caller identity, the call stack, a thread
  name, the executable path, the working directory, the process id, the
  host name, the user id or build information, such as
  `Location::caller()`, `runtime.Caller`, `os.Getwd`, `os.Getpid`,
  `os.Hostname`, `os.Getuid`, `std::env::current_dir`, `std::process::id`
  or `debug.ReadBuildInfo`, is refused before any compile.
- A problem whose builds leave out a feature set or Go build tag that a
  shipped build, hook or CI step uses is refused at acceptance and again
  before launch.
- A fixture adapter, or a pinned tool it runs, that reads a fixture or
  scratch file outside its grant finds nothing there, and fails.
- Editing a fixture file, a resource or bound record, an adapter program
  or a pinned tool after launch changes nothing an adapter serves or
  enforces.
- Under both profiles, a Go `init` function, or a package-level variable
  whose initialiser is not a constant, that the candidate adds or changes
  is refused before any compile; in a whole-file problem a base's
  unchanged `init` function is refused too, and its unchanged
  package-level variables follow the rule in "A problem definition".
- A candidate that defines its own `assert` or `debug_assert_ne` is
  refused, like one that defines `assert_eq`.
- A fixture adapter that opens a network connection or reads any host
  environment variable, or a pinned tool that does so beyond its record's
  typed slots, fails, and the candidate with it.
- A candidate that embeds a fixture file at build time, under either
  profile, is refused before any compile.
- A candidate that opens a fixture file's or pinned tool's path, or a
  toolchain binary's path, directly, or makes a system call the container
  does not allow, finds nothing there or is stopped, and fails.
- A candidate that drops edge-case handling only in the optimised build,
  for example under `cfg!(debug_assertions)`, fails the edge-case tests
  there.
- In a whole-file problem, a candidate that rewrites the crate's tests or
  changes a file outside `target.path` (such as adding a `build.rs` or a
  procedural macro crate) is refused at the start of step 1, before
  anything is built, and each test case runs in its own process.
- A candidate whose run-time code opens a hidden evaluation input's
  path, even with the validator's check passed, finds nothing there and
  fails.
- An action that appears only in the optimised timing build, or only
  under one build tag or feature set, is found and refused unless
  declared.
- A file written to scratch space, state kept in a `fixture.state`
  grant, or state kept by a fixture adapter or pinned tool in one test
  case or timing repeat cannot be read in the next.
- A swapped problem hash or sandbox settings hash is refused.
- Permissions supplied by the candidate are refused.
- Unknown fields in the effects table or the contract are refused. A
  valid signature alone cannot pass these cases.
- A missing safety tool, an unresolved bound, forged proof output or a
  failed archive write gives no receipt; no check is skipped.
- A candidate that passes the public tests but fails a hidden edge case
  (beta = 0 with NaN in the output) scores nothing, and the feedback
  names the failing case without showing the hidden inputs; a
  candidate that prints a hidden input in its panic text or output,
  in step 3 or step 4, does not get that text into the feedback.
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
- A commit changing a source file without a matching receipt is
  refused, naming the file; with one, it passes.
- Once that language's safety parse exists (item 6), a changed shell,
  Cedar or Rego file outside `tests/`, `benches/` and `fixtures/`, with
  no receipt, is refused. A Rust file only a test
  target compiles, and a shell file under `tests/` that only tests
  read, need none. A SQL file under `fixtures/` that a shipped build
  includes with `include_str!` needs its own receipt, and so does a
  shell file under `tests/` that a git hook runs.
- A deleted source file, a changed document, a test, benchmark or
  fixture, and the receipts folder's own copies need no receipt.
- Removing or changing the `ganit.every-change` entry, dropping a tier or
  lowering its severity is refused; adding is not.

## Changes from the source

- The maths side is rewritten as Rust in ganit.
- The first kernels are Rust, in ganit; whole-file problems may evolve any
  Rust or Go file that meets the whole-file rules.
- The safety step's forbidden list is restated for Rust, with `unsafe`
  added, and for Go, with the `unsafe` package, `//go:linkname`,
  hand-written assembly, the `plugin` package and `import "C"`, as
  AGENTS.md section 4 and section 6 rule 6 ask.
- The compiler choice for the old kernels is dropped.
- The run screen and briefing come from the Dioxus app.
- Receipts live in `receipts/`, and source files are the kinds AGENTS.md
  section 6 rule 9 lists; safety parses are needed for those kinds.
- The owner's instruction of 8 November 2026 ("wire ganitevolve into
  every coding task such that it's used without exceptions making it
  mandatory") had no exception list and covered shell files too. The
  owner confirmed on 10 November 2026 ("Every source file") that shell,
  Cedar and Rego files need receipts too. Tests, benchmarks and
  fixtures go with the receipt of the code they test. AGENTS.md
  section 6 rule 9 holds the list.
- The heavy-job limit follows AGENTS.md section 11, not one job of
  3.5 GB.
- Speed follows the 5% median rule; the 20x goal is kept (spec 0007).
- Spring Boot-style checks become Sutradhar Boot's rules.
- `gate-wiring` may be loosened only by an owner-signed change.
- The door runs the gate on the receiving side.
- The safety step has two profiles. The owner approved this on 11 November
  2026 ("Approve scoped safety profiles (Recommended)"): numerical code
  keeps the ban on file, network and process actions, and operations code
  may use only the actions and bounds its accepted problem declares, on
  paths inside its accepted research card's scope, inside the same sealed
  container and checks.

Source: `docs/specifications/ganit-evolve/README.md`,
`docs/specifications/ganit-evolve/ganit-evolve-2.md`,
`docs/specifications/ganit-evolve/ganit-evolve-3.md`,
`docs/specifications/ganit-evolve/ganit-evolve-4.md` and
`docs/specifications/ganit-evolve/ganit-evolve-5.md` in the archive
`archive-2026-11-08-old-repo`.
