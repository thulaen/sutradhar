# Test-first gate

Owner: the Git coordinator. Status: implemented and checked on Dell;
activation step 3 of the approved order (SPEC-007 part 2, "Test-first
evidence", AT-T01, AT-T02; AT-G19). Part of the check runner
([runner record](runner.md)); standard library only; Dell only. Turning
it on in the repository policy changes the gate itself, so it was put to
the owner as a question (R116); the owner approved it on 30 October 2026
("Approve (Recommended)"), and this commit makes it active in the docs,
commit and push profiles.

## What it checks

The runner takes the proposed commit message (`--message FILE`), records
its SHA-256 in the signed bundle, and `verify` needs the same message
(`--message FILE|none`). Commits are made with `--cleanup=verbatim`, so
the stored message is exactly the hashed one; the gated Dell repository
will read it from the commit object. The message must hold exactly one
`TDD:` line; a missing, repeated or unknown one fails (AT-G19).

- `TDD: test-first`: at least one new or changed test must fail on the
  parent's code by assertion and pass on the candidate. The runner builds
  the red tree itself (the parent's files with the candidate's test
  files, as the policy's test patterns choose) and runs the pinned harness
  on the candidate and then on the red tree, each in its own sandbox. An
  error or absence on the parent counts only for tests named in a
  reviewed new-unit record (AT-T01).
- `TDD: retrofit`: only tests and documentation may change, and at least
  one test must be new or changed. Its proof is coverage or mutation.
- `TDD: not-applicable (refactor)`: no test may be edited; tests may move
  between classes and files (AT-T02).
- `TDD: not-applicable (documentation|dependency|configuration|secret)`:
  every changed path must match the policy's paths for that reason. The
  policy lists no secret paths yet, so that reason cannot pass until the
  owner names some.

A test is new or changed when its module, any class it inherits from or
its own method changed, judged by syntax-tree fingerprints, so layout and
comments never count. The harness reports each test's keys from the code
that defines it, ids from the test's class and method, and which tests
actually started. Only changed test modules run, or every module when a
changed helper file may be inherited.

## Accepted limits

- Red and green prove that a test tells the parent from the candidate,
  not that it tests the intended behaviour; that is a review matter.
- Code under test runs in the harness's process, so it could forge
  results, keys (for example by setting `__module__` or `__qualname__`)
  or ids on purpose; as for unit tests, that is a review matter.
- New-unit and review records are interim and unsigned until reviewer
  signing exists; anyone writing the evidence file can name a reviewer.
- A test whose only change is a data file beside it, or whose decorator
  hides its function without `functools.wraps`, does not count as
  changed; the gate then fails safe (FAILED), and touching the test
  itself is the remedy.
- A refactor that changes a test file's import path (for example after
  moving a function) is refused; the import change goes in a retrofit
  commit of its own.
- A change that makes the parent hang shows as a blocked red run.
- Parsing is bounded per file (1 MiB) and by the export limits, not by a
  total cap.

## Evidence (Dell, 30 October 2026)

Private logs and the qualification run are in locked Dell directories
named in the private operator note.

- Test-first: tests were run on Dell before each change and failed on
  behaviour: the first version against stubs (20 failures, 20 errors from
  stub results), the first review's fixes (17 and 7), the second review's
  (6 and 1) and the module-selection fix found by the benchmark (1).
  Tests written after code to close gaps are labelled as retrofits.
- Independent review: two passes (13 findings, then 3 open and 8 new);
  every finding was fixed or recorded above as an accepted limit.
- Tests: 431 across `tools`, all passing; 100% line and branch coverage
  of the runner (1,237 statements, 448 branches).
- Mutation (mutmut 3.8.1), final code: 3,427 mutants; 3,424 killed, 2
  timed out and 1 survivor reviewed as equivalent (the harness's log
  detail level 2 versus 3 changes nothing). Earlier survivors were
  killed by new tests or removed by simplifying the code.
- Real sandbox (systemd, runner started as root, qualification key), new
  runner and policy on this tree: the commit itself PASSED as test-first
  (32 qualifying tests; red run FAILED on the parent as it should) and
  verify PASSED; refused with FAILED, each for its own reason: a new test
  that already passes on the parent, no declaration, a documentation
  declaration for code, a refactor that edits a test. An earlier run
  found a module named like a test file; it was renamed (`tdd.py`) and a
  test now guards the rule.

## Performance gate (SPEC-007), result B

Unit: the test-first check. Reference, fixed before measuring: the
simplest correct check, which runs the whole suite in detail mode on the
candidate and on the red tree. Measured on Dell, one pinned core, 15
interleaved pairs, both finding the same 32 qualifying tests:

- reference 4.78 s, gate 4.18 s: 1.14 times faster (95% interval
  1.12–1.17); spread 2.3% and 1.9%, inside the 10% rule;
- choosing the modules (parsing the changed test files) takes 49 ms;
- floor: the gate must run every new or changed test and every test that
  inherits one, on both trees; for this commit that is 168 of 431 tests
  on the candidate and 135 on the red tree, because the commit changes
  the shared test helpers, so 20 times is not reachable here. Commits that
  change a few test modules save far more.

An earlier version that ran only the changed modules was 1.62 times
faster but missed tests inheriting the changed helpers; correctness won.
