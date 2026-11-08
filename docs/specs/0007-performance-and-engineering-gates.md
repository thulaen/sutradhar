# 0007: Performance gate and engineering gates

This part sets three checks of the gate: the speed check, the test-first
check and the source limits check. The speed check uses the 5% median
rule of [AGENTS.md section 6](../../AGENTS.md#6-code-standards) rule 8.
It is roadmap Phase 2 item 7 (the benchmark runner, its baselines and
the slowdown check) and item 9 (the checks that guard the project).

## What it does

### Speed check results

For each candidate the speed check reports one of:

- PASSED: every mapped scenario meets its target and its baseline.
- VALID_REUSED: unchanged units whose earlier evidence still matches.
- NOT_APPLICABLE: the inventory proves no executable unit changed.
- FAILED or BLOCKED: anything else, including missing, noisy, stale or
  unsupported evidence and missing tools.

### Units and the map

1. A trusted inventory lists every changed executable unit of product
   and tool code: function, method, closure, query, script and
   generated source.
2. A protected map in the benchmarks folder assigns each unit to one or
   more scenarios. Small helpers are measured through the operation that
   calls them when their own timings are unreliable.
3. An unmapped changed unit makes the check BLOCKED. Code that is not a
   hot spot is not left out.
4. Test code is not a speed unit. Its speed is governed by the suite
   time budgets (the fast push suite under 5 minutes; a tier over 30
   minutes adds a task, as AGENTS.md section 7 says) and the slow-test
   rule in autoissues.

### Two hard rules

1. **Targets.** Every changed or affected scenario must meet the speed
   target its specification sets. Targets come from what users need,
   for example reading a typical post within 20 microseconds, or a
   preview within 1 second.
2. **Baselines.** A slowdown blocks when the median of at least 30 runs
   is more than 5% slower than the baseline kept in the repository and
   the difference is larger than the spread of the runs. A baseline may
   become faster with any commit (a faster result updates it, with the
   candidate's tree), and slower only through an owner waiver.

### Baselines

1. Changed code is compared with the version before it, from the last
   accepted commit. New code is compared with a justified reference:
   normally the simplest correct idiomatic version, or an established
   library doing the same work.
2. Baseline review refuses references made slow on purpose: added
   sleeps, worse algorithms, caches turned off that the real system
   would have, or unrealistic inputs.
3. A recorded baseline never changes without a new review.

### Measurement

1. Baseline and candidate run in the runner's boxes on Dell, with the
   same runtime, dependency lock, settings and inputs, under the
   performance governor, and the runner signs the results. Benchmarks
   run alone, holding both heavy-job places (AGENTS.md section 11).
2. Cores are pinned, and the frequency governor, the load and other
   work on Dell are recorded. Baseline and candidate runs take turns
   (baseline, candidate, baseline and so on) so they share the noise.
3. Warm-up runs are thrown away. At least 30 measured pairs are taken.
4. Reports give the median, interquartile range, 95% bootstrap interval
   and coefficient of variation. A coefficient over 10% makes the run
   noisy, which blocks.
5. Peak memory is recorded for both.
6. Outputs must match:
   - for unchanged behaviour, baseline and candidate outputs are the
     same;
   - for an intended change, such as a bug fix, the comparison uses
     inputs whose expected output did not change, plus a reviewed
     reference for the new behaviour on the changed inputs, where the
     candidate must pass its own approved tests.
   Any mismatch fails the check, whatever the speed.
7. Rust benchmarks use criterion with the harness. PHP uses the
   admitted PHP benchmark harness.
8. Time inside model inference (the pure-Rust models) is measured and
   reported apart from application code.

### Profiling

Before any speed work, read the current bottlenecks list and profile
the scenario. Work from measured hot spots, not guesses. The change
names the profile used and the hot spot it targets. A speed fix for an
autoissues task attaches before and after numbers from the same
benchmark.

### Reuse and invalidation

Evidence is keyed by the hashes of the measured units and their call
dependencies, the scenario definitions, inputs, dependency lock, runtime
and a fingerprint of Dell. Unchanged units reuse it. A change to any key
part measures again.

### Tiers

The commit tier runs the check for changed units. The push tier checks
that the evidence keys match the exact candidate and measures again
where evidence is stale. A documentation-only candidate gets
NOT_APPLICABLE from the inventory. A missing tool never gives
NOT_APPLICABLE.

### Bounded inputs to speed reports

1. Each check, update or dashboard command has two positive settings:
   at most 8,388,608 bytes (8 MiB) per file and 33,554,432 bytes
   (32 MiB) for the whole command. The options are `--input-max-bytes`
   and `--input-total-max-bytes`, both global. Existing handler APIs
   pass these defaults explicitly.
2. All inputs of one command share one budget. The real bytes read are
   counted (UTF-8, JSON syntax and white space), never the file size
   the system reports.
3. A read takes at most the smaller of what is left of the two
   allowances, plus one byte. That byte only detects excess; it never
   raises the allowance.
4. Zero settings, overflow, excess bytes, short reads and bad JSON are
   refused. A failed budget is never reused. No evidence or output
   follows an input failure.
5. The shared bounded reader does the reading. This rule adds no path or
   link restriction and changes no speed or noise threshold.
6. A larger real workload needs a recorded setting change and a
   resource check first. These values protect; they do not prove that
   the whole process stays within its memory cap.

### Test-first evidence

1. A commit that adds behaviour or fixes a defect needs at least one new
   or changed test that fails when run against the parent commit's
   source with the candidate's tests, and passes on the candidate.
2. The failure must be about behaviour: a failed assertion or a wrong
   error. A build failure or a missing module does not count, unless a
   reviewed note explains that the unit did not exist before. The
   better route is a small stub so the test fails on behaviour.
3. The runner does this red check in its box and signs it. A pasted log
   is never trusted.
4. The `TDD:` line of the commit message follows AGENTS.md section 9: it
   names the test that failed before the change, or says
   `TDD: none (docs only)`.
5. Tests added to existing code to close coverage or mutant gaps may
   pass on the parent; their proof is the coverage or mutant-kill
   result. A pure refactor keeps every test green and edits no test
   except to move it.
6. The check refuses a missing or contradicted `TDD:` line.
7. Unit tests are required for executable behaviour. Integration,
   property, security, recovery and other tests follow where the code
   meets other parts, under the coverage and mutant rules of AGENTS.md
   section 6.

### Fast development checks

A fast Cargo command runs syntax, formatting, static analysis, type
checks and the affected unit tests on Dell after edits. It is a helper
for the author, not a gate check, and never admits a commit or push.

### Source limits

1. **Counting.** Text is decoded as UTF-8; a first-party text file that
   does not decode fails. Line breaks are LF, CRLF and a lone CR; CRLF
   counts once. The line count is the number of breaks, plus one if the
   file is not empty and does not end with a break. Every line counts:
   blank lines, comments, doc comments and licence headers.
2. **Characters.** Line length is the number of Unicode code points,
   not counting the break. Tabs are refused in first-party text, and so
   are zero-width and bidirectional control characters.
3. **Functions.** A function, method or closure runs from its first
   line, attributes included, to its last line. Nested functions count
   inside their parent and on their own. Rust positions come from its
   parser; PHP positions from the admitted PHP parser.
4. **Limits.** A source file has at most 300 lines; a function at most
   60 lines, 5 arguments, 4 levels of nesting and a complexity score
   under 15 (AGENTS.md section 6 rule 11). First-party text lines have
   at most 80 code points.
5. **Exceptions.** A line holding one unbreakable link and nothing else
   may have up to 200 code points. Quoted historical source text keeps
   its exact length. Lock files written by tools are classed as lock
   files and checked by their tool. Every other breach fails at the
   commit and push tiers.
6. **No evasion.** Minified or encoded first-party content is refused.
   Splitting strings only to dodge the count is a review finding.
   Generated files are measured like hand-written ones. A function split
   only to beat the limit, with no clear duty of its own, is a review
   finding under DRY, KISS and SOLID.
7. **Classes.** A protected classification file in `gate/` classes
   every tracked path as first-party text, first-party data, lock file,
   third-party or binary. An unclassified path fails. Changing a class
   is a protected change. Its input bounds are in 0009.
8. **Rule matrix.** Each rule of the rule matrix (AGENTS.md section 6
   rule 3) names how it is enforced: a check, a test pattern or a review
   item, and the tier that blocks on it. A rule with no check yet is a
   review item, never a pass by default.

## Limits

- Speed report inputs: 8 MiB per file and 32 MiB per command, by
  default.
- Benchmarks run alone on Dell. Other heavy jobs run at most two at
  once, each held to 4 GB and 8 CPU threads, with no swap, on the
  approved SSD.
- Memory budgets of the specifications apply to every measured run,
  within AGENTS.md section 5.6.

## Footprint budget

This part adds nothing to public pages. The speed targets of each
feature's own specification still apply to its public-page work.

## How it is tested

Rust tests in aegis, each with a planted fault:

- An unmapped changed unit blocks the check; a changed test function is
  not treated as an unmapped unit.
- A baseline with an added sleep is refused at review; a recorded
  baseline changed without review is refused.
- A scenario over its specification target fails.
- A median more than 5% slower than the baseline, with a difference
  beyond the spread, fails; one within 5% passes; a faster result
  updates the baseline with the candidate's tree.
- A coefficient of variation over 10% is reported as noisy and blocks.
- An output mismatch between baseline and candidate fails, whatever the
  speed. A bug fix passes when unchanged inputs match the baseline and
  changed inputs match the reviewed new reference.
- Unchanged units reuse evidence; a dependency lock change voids it.
- A docs-only inventory is NOT_APPLICABLE; a missing benchmark tool
  with changed code is BLOCKED.
- A speed fix that does not name the profiled hot spot fails.
- Model inference time is reported apart from application time.
- Input bounds: a file of exactly 8 MiB passes when the command has room
  left; one more byte refuses. Combined limits, growing and trickling
  readers, real byte counting, zero and overflow settings, bad and
  partly read data, and no output after a refusal are all covered. Each
  new test runs 20 times fresh.
- A new test that passes on the parent is not test-first evidence; one
  failing only by a missing module without the reviewed note is
  refused.
- A refactor commit that edits test assertions is refused.
- A passing fast development command cannot admit a commit whose full
  tier fails.
- A 301-line source file fails; 300 lines with no final break passes;
  300 lines plus one trailing blank line fails.
- An 81-code-point line fails, in Markdown and YAML too; a line of 80
  wide characters passes the length rule.
- A 61-line Rust function with a two-line attribute fails, and so does a
  61-line PHP method counting its attribute.
- Lone CR breaks, tabs and bidirectional controls cannot hide lines or
  characters from the count.
- An unclassified path fails; a third-party path is not edited and is
  reported apart.

## Changes from the source

- The 20x speed goal, its limit report and its progress dashboard are
  replaced by the 5% median rule of AGENTS.md section 6 rule 8.
- The "best recorded result" ratchet becomes the baseline kept in the
  repository, slower only through an owner waiver.
- Measurement and profiling use Rust and PHP tools only.
- Model inference is pure-Rust models, measured apart.
- The heavy-job limits follow AGENTS.md section 11, not the old 3.5 GB
  and two-compile limit.
- The `TDD:` line follows the commit form of AGENTS.md section 9.
- The 300-line limit covers source files as AGENTS.md section 6 defines
  them, plus the argument, nesting and complexity limits; line-length
  and character rules cover all first-party text.
- The task runner's fast command becomes a Cargo command.
- The old activation stages are replaced by AGENTS.md section 7.3.
- The classification file moves into `gate/` as a protected file.

Source: `docs/specifications/performance-gate.md`,
`docs/specifications/performance-gate-inputs.md`,
`docs/specifications/performance-gate-rust.md` and
`docs/specifications/engineering-gates.md` in the archive
`archive-2026-11-08-old-repo`.
