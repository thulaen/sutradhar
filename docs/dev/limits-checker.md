# Source-limit checker

Owner: the Git coordinator. Status: implemented and checked on Dell for
tree `919415cd`; activation step 1 of the approved order (SPEC-007
engineering gates; SPEC-006 quota recovery). Not yet the qualified gate:
it becomes that when the runner signs its results.

## What it does

`python3 -m tools.limits.cli ROOT [--classes FILE] PATH...` checks the
listed repository paths and prints one JSON report. Exit 0 means PASSED,
1 FAILED and 2 BLOCKED. With the owner's approved exceptions it enforces:

- at most 300 physical lines per first-party text or data file;
- at most 80 code points per line, except a line holding one unbreakable
  link whose surrounding text fits in 80 (at most 200 in all), and lines
  inside historical quotation code blocks under
  `docs/requirements/sources/` (CommonMark fence rules);
- no control characters other than LF and CR, no Unicode format
  characters (zero-width and bidirectional controls among them), no other
  default-ignorable characters (such as variation selectors and Hangul
  fillers), no NEL and no Unicode line or paragraph separators;
- at most 60 lines per Python function, method or lambda, decorators
  included;
- every path classified by the class list; lockfiles, binary files and
  third-party material are inventoried, not measured.

Blocked, never passed: no paths given; a missing or unreadable class
list; unsafe paths (backslashes, empty, `.` or `..` segments, which also
refuse absolute paths, symbolic links, missing files); PHP source until a
PHP parser is admitted; parser resource limits; any internal error.
Files over 4,000,000 bytes fail before being read and files over
1,000,000 characters fail before scanning or parsing; each file reports at
most 50 violations, text and function together, plus a count of the rest.
The report records the
class list's SHA-256 so the trusted runner can insist on the protected
copy through `--classes`. Standard library only. Run it on Dell, in the
sandbox, never on the editing computer.

## Evidence (Dell, 30 October 2026, tree `919415cd`)

- Test-first: the first 60 tests failed on behaviour against stubs, then
  passed; the tests for both review passes were also written first and
  failed before the fixes. Tests added after code are labelled as retrofits.
- 163 tests pass; line and branch coverage 100% (309 statements, 110
  branches), coverage.py 7.17.0. MC/DC by truth-table tests.
- Mutation (mutmut 3.8.1): 769 of 770 mutants killed; one reviewed
  equivalent mutant below. Other equivalent mutants were removed by
  rewriting code.
- Whole repository: 204 files checked, PASSED.
- First real run found literal invisible characters in the checker's own
  source; they were replaced with escapes.
- Peak memory about 20 MB (Python start-up alone about 10 MB) and 0.05 s
  per run, against the 8 GB lab budget and the five-minute push-suite
  target.
- Independent review (code, performance, security), two passes: all
  findings fixed except the class-list name (below); the reviewer
  accepted the equivalent-mutant argument.

### Reviewed equivalent mutant

`functions.python_spans` closes enclosing scopes with
`end < start`. The mutant `end <= start` cannot change any result: an
enclosing scope always ends after its inner scope starts; `def` and
`class` begin a new logical line; and a `lambda` keyword is always
preceded by a token (operator, delimiter or keyword) that is not part of
an earlier scope, so an end and a later start never coincide. Reviewed and
accepted by the independent reviewer under R055; not excluded silently.

### Deviation needing the owner's decision

The approved engineering-gates text names `tools/limits/classes.yaml`.
The checker uses `tools/limits/classes.txt` (one `class pattern` per
line) because no YAML parser is admitted for tooling. Options: approve
the plain-text format, or admit a YAML parser first.

## Performance gate (SPEC-007), result B

Baselines fixed before optimisation, bound by SHA-256 (private Dell
evidence): unit A reference = the first correct implementation, tree
`5958993b` (`ref/counting.py` `1d28b22f…`, `ref/policy.py` `bc6dabe4…`);
unit B reference = the simplest `ast.walk` implementation and the docs
loop, both in the benchmark script `3f5de949…`.

Measured on Dell, one pinned core, 30 interleaved pairs, identical
outputs on the 204 repository files (18 Python) and on 40 seeded faulty
files with 120 violations:

| Unit | Reference | Checker | Speed-up (95%) |
| --- | --- | --- | --- |
| A vs first version | 27.0 ms | 8.5 ms | 3.16 (3.16–3.17) |
| A vs docs loop | 4.0 ms | 8.5 ms | 0.48 (0.47–0.48) |
| B vs ast.walk | 11.7 ms | 9.7 ms | 1.21 (1.20–1.22) |

Profile of the final checker, by internal time: Python's parser
(`compile`, about 6 ms per run), the syntax-tree walk (`iter_child_nodes`,
about 5 ms), the statement walk (`iter_fields`), non-ASCII character
scans (`findall`), line splitting and fence numbering. Unit A's hotspots
before optimisation were per-line Python loops, per-position pattern
scanning, a character-class search over all text and per-fence overhead.

The docs loop checks less (tabs only, no CR breaks, naive fences), so it
is not the same behaviour; it is shown for context.

Experiments, each measured: unanchored long-line pattern (slower);
anchored multi-line pattern; split with a length fast path; C-level
`map` and `compress` with lazy fence search (kept); an explicit-stack
tree walk (0.71×, a regression, rejected); statement-list walk for
lambda-free files (kept, 1.21×). Rejected without building: a native
extension (adds a language, admission and rights work to save a few
milliseconds per run), parallel processes (start-up exceeds the work)
and content-hash caching (hashing costs about as much as checking).

Practical limit: reading and decoding alone takes 1.06 ms against the
unit A reference's 27.0 ms, so nothing could exceed about 25×, and 20×
would leave about 0.3 ms for all checking, below one C-level split of the
text. Unit B: the parser alone takes 6.4 ms of the reference's 11.7 ms,
so at most 1.8× without replacing Python's parser. No regression
against either reference; about 20 MB and 0.05 s against the 8 GB lab
budget and the five-minute push-suite target.

## Limits of this evidence

Interim, unsigned results on one host and one workload.
