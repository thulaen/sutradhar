# SPEC-007 part 2: test-first evidence and source limits

Status: APPROVED. Protected-gate design awaiting the owner's approval.
Requirements: R087–R089, R054, R056 as replaced. Part 1 is the
[performance gate](performance-gate.md).
Approved by the owner on 30 October 2026 (see the approval record in
`docs/specifications/approvals-2026-10-30.md`); it supersedes any
"awaiting approval" wording below.

## Test-first evidence

For each commit that adds behaviour or fixes a defect, the gate requires at
least one new or changed test that:

1. fails when run against the parent commit's source with the candidate's
   tests, and
2. passes on the candidate.

The failure must be behavioural: an assertion failure or an expected
exception mismatch. A syntax error, collection error or missing-module
error does not qualify unless a reviewed note explains that the unit did not
exist before; the preferred route is to add a minimal stub so the test
fails on behaviour. The runner performs this red check in a sandbox and
signs it, rather than trusting a pasted log (AT-T01).

Commits declare one of:

- `TDD: test-first` for new behaviour and bug fixes;
- `TDD: retrofit` for tests added to existing code, such as the preserved
  publisher modules, or to close coverage or mutation gaps; these tests
  may pass on the parent, and their proof is the coverage or mutant-kill
  result (see [AutoIssues proof of fix](autoissues/lifecycle.md#proof-of-fix));
- `TDD: not-applicable` with one reason: documentation, pure refactor
  (all tests green, no test edits except moves), dependency, secret or
  configuration change, each with its own evidence.

The gate refuses a missing or contradicted declaration (AT-T02). Until the
gate is qualified it reports NOT_YET_ACTIVE inside the bootstrap scope, with
the runner's recorded red and green runs as the interim result
([activation
order](autoissues/quota-recovery.md#bootstrap-and-activation-order)).

Unit tests are mandatory for executable behaviour. Integration, property,
security, recovery and other tests follow the boundary, under the existing
coverage and mutation rules (R054).

## Fast development checks

`just fast` runs syntax, formatting, static analysis, type checks and the
affected unit tests on Dell after meaningful edits. Its result is advisory
and never admits a commit or push (AT-T03).

## Source limits

Applies to every first-party text file, including documentation, tests,
tooling, configuration and generated source.

### Counting rule

1. Decode as UTF-8; an undecodable first-party text file fails.
2. Line breaks are LF, CRLF and lone CR; CRLF counts once. The physical
   line count is the number of line breaks, plus one if the file is
   non-empty and does not end with a break.
3. Every physical line counts: blank lines, comments, docstrings and
   license headers included.
4. Line length is the number of Unicode code points excluding the break.
   Tab characters are refused in first-party text. Zero-width and
   bidirectional control characters are refused (they hide content).
5. A function, method or closure spans from its first line (including
   decorators or attributes) to its last line, inclusive. Nested functions
   count inside their parent and on their own. Python uses the standard
   `ast` positions; PHP uses the admitted PHP parser's start and end lines.

Limits: 300 physical lines per file, 80 code points per line, 60 lines per
function. The owner's 30 October 2026 decisions allow only these
exceptions: a line holding a single unbreakable link and nothing else may
be up to 200 code points; lines inside historical quotation code blocks
under `docs/requirements/sources/` keep their exact length; tool-written
lockfiles are classified as lockfiles and verified by their tool. Any other
violation is a hard failure in the commit and push profiles (AT-T04 to
AT-T07).

### No evasion

The check refuses minified or encoded first-party content. String
splitting that exists only to dodge the measurement is a review finding.
Generated files are measured like hand-written ones. A
function split purely to beat the limit, without a coherent responsibility,
is a code-review finding under DRY, KISS and SOLID.

### Classification

A protected `tools/limits/classes.yaml` classifies every tracked path as
first-party text, first-party data, lockfile, third-party or binary. An
unclassified path fails. Changing a classification is a protected change.
The current inventory and the decisions it needs are in the
[source limits inventory](../dev/source-limits-inventory.md).

### Activation

The source-limit checker is the first gate activated. Before it is
qualified, the runner's signed Dell measurement with this exact counting
rule is the interim result. The rulebook's other rules activate when their
checkers are qualified; until then they are review items, never passes by
default.

## Rulebook mapping

The NASA/JPL and Meta practices mapped to PHP and Python are in the
[mapped rulebook](../dev/rulebook-mapping.md). Each mapped rule names its
enforcement: a checker, a test pattern or a review item, and the stage
that blocks on it.

## Acceptance cases

- AT-T01: A new test that passes on the parent is not test-first evidence;
  one failing only by import error without the reviewed note is refused.
- AT-T02: A refactor commit that edits test assertions is refused.
- AT-T03: A green `just fast` cannot admit a commit whose full profile
  fails.
- AT-T04: A 301-line file fails; 300 lines with no final newline passes;
  300 lines plus one trailing blank line fails.
- AT-T05: An 81-code-point line fails, including in Markdown and YAML; a
  line of 80 wide characters passes the length rule.
- AT-T06: A 61-line Python function with a two-line decorator fails, and
  a PHP method of 61 lines including its attribute fails.
- AT-T07: CR-only line breaks, tabs and bidirectional controls cannot hide
  lines or characters from the count.
- AT-T08: An unclassified path fails; a third-party path is not edited and
  is reported separately.
