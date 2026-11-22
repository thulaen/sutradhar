# 0083: Meaningful lines and anti-padding

The owner set a tracked target of four million meaningful lines for Behemoth
Boot and four million for Leviathan, with a hard-blocking gate and rules that
stop padding (ADR-0022). This specification defines what a meaningful line is,
how it is counted, which padding tricks the harness and the gates refuse, and
how the target blocks a release. It adds no new quality bar: completion of a
framework or a crawler is judged by the parity ledger (spec 0081), the
conformance suites (spec 0082) and the tests and proofs. A count alone never
completes anything.

Example: an agent splits one 20-token statement over ten lines. The count uses
logical lines, so it still counts once. Another agent copies a module and
renames the variables; the copy check refuses it. A third adds an empty
function that nothing calls; dead-code analysis refuses it. A fourth adds a
large block of generated tables without the generated marker; the check refuses
the file. Real code that is tested, not copied, and has a reason in the ledger
is counted.

Values marked "proposed" are design choices to be checked by measurement.

## What it does

### Words used here

- **Target**: one of Behemoth Boot or Leviathan, each with its own count.
- **Physical line**: a line of text in a file.
- **Logical line**: one statement or declaration, found by parsing the code,
  however many physical lines it spans or shares.
- **Meaningful line**: a logical line that passes every test in "What counts".
- **Padding**: code, tests or text added to raise a count and not to do
  something the specification needs.
- **Traceable**: linked to a parity item (spec 0081) or a conformance item
  (spec 0082) or another approved requirement.

### What counts

A logical line is meaningful only when all of these hold.

1. It is in a first-party source file of an allowed language, in a crate that
   belongs to the target.
2. It is not blank, a comment or a documentation comment. Documentation is
   required (AGENTS.md section 6) but never counted.
3. It is not generated and not vendored. A generated file must carry the
   generated marker and match its sources by hash (AGENTS.md section 8.5).
4. It is not part of a duplicate region: two runs of 100 or more equal tokens,
   or a near-duplicate found by the clone check, are counted once, at the first
   occurrence.
5. It is reached by a test. Line coverage is measured by the coverage tool the
   gate uses (spec 0007).
6. It is in a function that survives mutation checks: at least the proposed
   share of the mutants of that function (70%, proposed, higher for the strict
   core) are killed, and each survivor is proved equivalent or flagged.
7. Its module is traceable. A module with no link to a parity item, a
   conformance item or another approved requirement counts for nothing and is
   reported as an orphan.
8. It is not behind a feature switch that is never enabled in any shipped
   profile.

Test code is counted separately and never added to the target. A line in a test
file does not make a framework larger.

### Anti-padding checks

Each check is a build-time gate with a planted-fault test (spec 0003): one file
that tries the trick and must fail with a named message, and one honest file
that must pass.

| Check | Refuses |
|---|---|
| AP-01 Clone | Copies and near-copies, including renamed variables and reordered statements, in one file or across crates |
| AP-02 Dead code | Items that nothing reaches, unused dependencies, unused parameters and unreachable branches |
| AP-03 Generated bulk | Generated or table-like code without the generated marker or without a matching source |
| AP-04 Formatting inflation | Splitting statements across lines; the count uses logical lines |
| AP-05 No-op code | Empty bodies, functions that return a constant and are called nowhere, assignments never read |
| AP-06 Forwarding layers | Wrappers and traits that only forward a call, with no behaviour or boundary of their own |
| AP-07 Empty tests | Tests that assert nothing or that no mutant can fail; they do not raise coverage credit |
| AP-08 Orphan code | Modules with no traceable requirement |
| AP-09 Macro bloat | Macro expansions or derived code counted by expanded size; they count once, at the macro |
| AP-10 Comment padding | Comments or documentation written as code-looking text |
| AP-11 Test-only code in product | Production code present only to be called from a test |
| AP-12 Config bloat | Large lists of settings or constants with no reader |

The heuristic checks (AP-05, AP-06, AP-08) report findings to a reviewer; a
reviewer's written verdict, kept in the review record, can clear one. The
automatic checks (AP-01 to AP-04, AP-07 and AP-09 to AP-12) block without
a verdict.

### How the gate works

| Tier | What runs | Blocks |
|---|---|---|
| Commit | AP-01 to AP-05 and AP-09 to AP-12 on changed files, and the traceability check on new modules | The commit |
| Push | All anti-padding checks on the whole target, and a report of meaningful lines | The push |
| Release | Full coverage and mutation counts, all checks, and the target check below | The release |

Release target check: a Behemoth Boot release is refused if its meaningful
lines are below the target of four million, and so is a Leviathan release. Both
parity or conformance ledgers must also be complete; neither alone is enough.
The only route past a shortfall is the owner's signed waiver (AGENTS.md section
7.2), made after the ledger shows the specification complete with fewer lines.
A waiver names the target and the count, and expires in 30 days or less
(BJ-29 of spec 0114). No agent can create one.

The report records, per target and per crate: physical, logical, meaningful,
duplicate, uncovered, mutation-survivor and orphan counts, the tool versions
and the exact revision. It is signed in the signature log (spec 0065).

### The tools behind it

The gate depends on four things, each behind a trait with a test fake, so the
choice of tool can change without changing the rules.

- `LineClassifier`: parses code and classifies logical lines. The code analyser
  nirikshak (spec 0061) already reads Rust with a parser; it is the planned base
  for Rust. Other allowed languages need a classifier before they can count.
- `CloneDetector`: finds copies and near-copies. The copy check of the gate
  (spec 0007) is the base.
- `CoverageSource`: line coverage per file. The tool is chosen by the research
  dossier for the Bazel build on both systems (QR-023).
- `MutationSource`: mutant results per function. The tool is chosen the same
  way (QR-023).

Until a tool is admitted for an allowed language, that language's lines count
as zero, and the report says so.

### Limits

- The count at commit runs only on changed files. At push, a whole-target
  count is incremental, keyed on changed inputs (proposed); the full count
  runs in the release tier. A full count on a large target must stay
  inside the heavy-job slot and the memory budget of spec 0078 (proposed:
  the Linux reference machine under 3 minutes at push, to be measured;
  the push tier is a 5-minute suite).
- Mutation runs are sampled at push and complete at release.

### Data

Reports are files in the repository's evidence area, signed, one per run, with
a short generated summary. The count is never a setting.

### Settings

The count and the targets are never settings. The four million lines for
each target is the owner's decision (ADR-0022) and changes only by a new
decision. The one adjustable value is a proposed time limit.

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `gate.count_push_min` | minutes | 3 (proposed) | 1 to 5 (proposed) | next push |

There is no setting that skips a check, counts an unadmitted language, or
lowers the target. A waiver is a signed owner record that expires in
30 days or less
(BJ-29 of spec 0114; AGENTS.md section 7.2), not a setting.

### Failure behaviour

| Failure | Effect | Recovery |
|---|---|---|
| A tool cannot run | The check fails; it never skips | Fix the tool or pin it again |
| Coverage or mutation data is missing | The affected lines count as zero and the report says so | Run the tool |
| A classifier cannot parse a file | The file counts as zero and is listed | Fix the parser or the file |
| A reviewer verdict is missing for a heuristic finding | The gate blocks | The reviewer writes a verdict |
| The target is not reached at release | The release is refused | More real work, or the owner's waiver |

### Failure codes

The codes below are proposed. Codes SUT-4900 to SUT-4999 are proposed as the
range reserved for this spec; a decision record must confirm it first.

| Code | Meaning |
|---|---|
| SUT-4900 | A tool could not run; the check fails and never skips |
| SUT-4901 | Coverage or mutation data missing; lines count as zero |
| SUT-4902 | A file could not be parsed; it counts as zero and is listed |
| SUT-4903 | Heuristic finding without a reviewer verdict; gate blocks |
| SUT-4904 | An anti-padding check (AP-01 to AP-12) refused a file |
| SUT-4905 | Release refused: meaningful lines below the target |
| SUT-4906 | Release refused: the ledger is incomplete |
| SUT-4907 | Report refused: unsigned, or tools or revision not named |

### Security and privacy

- Only the owner's signed waiver (AGENTS.md section 7.2) can pass a
  shortfall. It names the target and the count, expires in 30 days or less
  (BJ-29 of spec 0114), and no agent can create one.
- Reports are signed in the signature log (spec 0065) and name the tools,
  their versions and the exact revision, so a count cannot be altered
  unseen.
- A reviewer's written verdict is the only way to clear a heuristic finding.
- The gate reads first-party source and tool results only. It reads no
  secret, and reports hold counts, not file contents.

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| ML-R01 | Only logical lines passing every rule in "What counts" MUST be counted. | UT-ML-01 to 08 |
| ML-R02 | Splitting a statement over several physical lines MUST NOT change the count. | UT-ML-09 |
| ML-R03 | A copy or near-copy MUST be counted once and refused by AP-01. | PL-ML-01 |
| ML-R04 | Dead, no-op, empty-test, orphan and forwarding code MUST be flagged as in the table. | PL-ML-02 to 06 |
| ML-R05 | Test code MUST NOT count toward a target. | UT-ML-10 |
| ML-R06 | A missing tool or missing data MUST fail or count as zero, never be skipped. | FI-ML-01 |
| ML-R07 | The release MUST be refused below target unless the owner's signed waiver names it. | BD-ML-01 |
| ML-R08 | Ledger completeness MUST be required in addition to the count. | BD-ML-02 |
| ML-R09 | The report MUST be signed and name tools, versions and revision. | UT-ML-11 |
| ML-R10 | Every anti-padding check MUST have a planted-fault pair. | PL-ML-07 |

## How it is tested

Designed, not run.

- **Unit (UT-ML):** each rule in "What counts" with one counted example and one
  refused example; logical against physical lines on a statement split ten ways;
  test code excluded; signed report fields.
- **Planted-fault pairs (PL-ML):** for every check, one file that pads in that
  way and must be refused with the named message, and one honest file that must
  pass. Examples: a function copied with renamed variables; an empty exported
  function; 50 lines of generated tables with no marker; a trait whose only
  method calls another trait.
- **Behaviour (BD-ML):** an agent tries each trick in turn and the gate refuses
  every one; a release at 3.9 million lines with a complete ledger is refused,
  and passes only with the owner's waiver; a release at 4.1 million lines with
  an incomplete ledger is refused.
- **Fault injection (FI-ML):** the coverage tool crashes; mutation data is
  stale; a file cannot be parsed.
- **Measurement:** the time of a full count on the reference machine.

## Changes from the source

ADR-0013's ban on vanity numbers is narrowed to allow this one tracked measure,
with these anti-padding rules and the ledger requirement. The existing copy
check and code analyser are reused, not rewritten.
