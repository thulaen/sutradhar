# Mapped engineering rulebook: NASA/JPL and Meta for PHP and Python

Status: APPROVED revision of the [earlier rulebook draft](rulebook.md),
awaiting the owner's approval. Not NASA, JPL, MISRA or Meta certification
or compliance. Researched 30 October 2026 from primary public sources.
Approved by the owner on 30 October 2026 (see the approval record in
`docs/specifications/approvals-2026-10-30.md`); it supersedes any
"awaiting approval" wording below.

## Sources

- G. J. Holzmann, "The Power of Ten: Rules for Developing Safety-Critical
  Code", IEEE Computer 39(6), June 2006, pp. 95–97,
  doi:10.1109/MC.2006.212 (`spinroot.com/p10`).
- JPL Institutional Coding Standard for C, JPL D-60411, version 1.0,
  3 March 2009 (public release copy; MISRA levels omitted from it). It
  separates "shall" rules, which need an independent waiver to break, from
  "should" rules, which need a written justification; the 60-line and
  six-parameter rule is a "should" there. Sutradhar's owner made the
  60-line limit mandatory (R089).
- NASA NPR 7150.2D, chapters 3 and 4, and NASA-STD-8739.8B (2022).
- Meta engineering publications: code review (2022), rapid release
  (2017), Infer (2015) and "Scaling static analyses at Facebook" (CACM
  2019), Pysa and Pyrefly documentation, Sapienz and SapFix (2018),
  predictive test selection (2018), probabilistic flakiness (2020),
  incident (SEV) culture talks, holistic configuration management
  (SOSP 2015), mutation testing (arXiv 2010.13464), ACH (arXiv 2501.12862)
  and TestGen-LLM (arXiv 2402.09171).

The phrase "move fast with stable infrastructure" has no Meta primary
source that was found; it is not cited as Meta policy.

## How to read each rule

Each rule gives its origin, the Sutradhar rule, its purpose, how it is
enforced and at which stage. "Commit" means the full pre-commit profile;
"review" means the recorded code review. A rule without a qualified checker
is enforced by review and tests until one is admitted; it is never assumed.

## Control flow and bounds

- E01 (P10 rule 1, JPL 4 and 11): no `goto` in PHP; no recursion unless an
  explicit, tested depth cap exists. Purpose: analysable call graphs and
  bounded stacks. Enforcement: static rule for `goto`; call-graph check for
  recursion with an allow-list naming each capped case. Stage: commit.
- E02 (P10 rule 2, JPL 3): every loop over input, retries, pagination or
  queue drains has an explicit cap, deadline or resumable cursor and fails
  with an error when exceeded. One marked main loop per worker is allowed.
  Purpose: no runaway work. Enforcement: review checklist plus oversized
  input and cancellation tests. Stage: commit.
- E03 (P10 rule 3, JPL 5, purpose only): bounded batches, streaming instead
  of whole-corpus loads, explicit memory limits and refusal before the cap.
  Purpose: predictable memory within the 2 GB and 8 MB budgets.
  Enforcement: memory measurements and limit tests. Stage: commit.
- E04 (JPL 7): never wait for another process by sleeping; use explicit
  signals, locks or polling with a deadline. Purpose: no timing races.
  Enforcement: static search for sleeps outside an allow-list; review.

## Function size and structure

- E05 (P10 rule 4, JPL 25): at most 60 lines per function, method or
  closure (R089) and at most six parameters. Purpose: units that can be
  understood and verified alone. Enforcement: source-limit checker
  (SPEC-007); Ruff `PLR0913` for Python parameters; PHP equivalent after
  admission. Stage: commit.
- E06 (NPR 7150.2D SWE-220): cyclomatic complexity at most 15 per function.
  Purpose: testable decisions. Enforcement: Ruff `C901` for Python; PHP
  analyser rule after admission. Stage: commit.
- E07 (JPL 24, 31): one statement per line; imports and `use` statements at
  the top of the file. Enforcement: formatters and linters. Stage: commit.

## Data, scope and state

- E08 (P10 rule 6, JPL 13): smallest possible scope; no shadowing; no
  module-level mutable state except declared, owned singletons. Purpose:
  fewer places that can corrupt data. Enforcement: linters and review.
- E09 (JPL 8, 6): each table, queue or file has one writer component;
  PHP and Python communicate only through the signed contract (R043).
  Purpose: no conflicting writers. Enforcement: import contracts, database
  permission tests and review.
- E10 (JPL 9): avoid nested locks; when unavoidable use one documented
  order; release in the scope that acquired (context managers, `finally`).
  Enforcement: review plus concurrency tests.
- E11 (JPL 12, 17): enums give all values or none; integer ranges are
  checked at storage and contract boundaries. Enforcement: schema tests.

## Checking results and inputs

- E12 (P10 rule 7, JPL 14, 15): handle every returned failure value (for
  example PHP functions returning `false`); discard a value only with a
  stated reason; validate parameters at every public boundary. Purpose:
  errors never vanish. Enforcement: PHPStan or Psalm strict rules, Ruff,
  type checks and malformed-input tests. Stage: commit.
- E13 (P10 rule 5, JPL 16, adapted): PHP removes assertions in its
  production mode (`zend.assertions=-1`) and Python does under `-O`. The
  sources allow disabling assertions after testing; Sutradhar's own choice
  is that required checks are always-on guard code raising a typed error.
  Plain `assert` is for tests and internal invariants only. No
  assertion-count quota.
  Enforcement: static rule refusing `assert` for input validation;
  disabled-assertion test runs. Stage: commit.
- E14 (JPL 18, 19): parenthesise mixed operators; Boolean expressions have
  no side effects. Enforcement: linter rules and review.

## Dynamic behaviour

- E15 (P10 rules 8 and 9, JPL 20 and 29, purpose only): no `eval`, no
  dynamic includes from data, no monkeypatching in production, and no
  calls chosen by string name where static analysis matters. Keep feature
  and configuration switch combinations few and tested. Enforcement:
  static rules and review. Stage: commit.
- E16 (JPL 1): rely only on documented PHP 8.5 and Python 3.14 behaviour;
  no reliance on implementation accidents. Enforcement: review and
  multi-version tests where supported.

## Analysis and warnings

- E17 (P10 rule 10, JPL 2, SWE-135): zero warnings from the strictest
  practical settings: PHP error reporting at maximum with strict types,
  PHPStan or Psalm at the strictest level, Ruff and a Python type checker
  in strict mode. Rewrite code that confuses the tool instead of
  suppressing it. Enforcement: commit profile.
- E18 (Meta Infer and CACM 2019): report analysis on the change being
  committed, not as a later batch list, and track how often findings are
  acted on. Enforcement: findings attached to the commit gate and fed to
  AutoIssues.
- E19 (Meta Pysa; SWE-185, 207): taint analysis for untrusted forum data
  reaching sinks (SQL, HTML, shell, file paths). Tool support for Python
  3.14 and PHP 8.5 must be qualified; until then review plus security
  tests. Stage: commit.

## Tests, coverage and mutation

- E20 (SWE-189, 190, 191): measure coverage from real test runs and keep
  regression tests, including security regressions. The project's 100%
  line, branch, decision and MC/DC rule stays (R054). No PHP or Python tool
  was found that measures MC/DC; it is shown with reviewed truth-table
  tests per compound decision. Stage: commit.
- E21 (Meta probabilistic flakiness): never retry until green. Record
  retry outcomes, quarantine a flaky test only with an AutoIssues owner
  issue, and keep it visible. Stage: commit and nightly.
- E22 (Meta mutation research and ACH): mutation testing on all
  first-party code, with extra concern-targeted mutants for permission,
  visibility and privacy checks. Stage: commit.
- E23 (Meta TestGen-LLM): an AI-written test is accepted only if it runs,
  passes repeatedly and adds coverage or kills a mutant. Stage: review.

## Review, change and incident practice

- E24 (Meta code review; SWE-087): every change is reviewed; a review
  records what was examined, so a rubber stamp is detectable. Code,
  performance and security reviews remain mandatory (R057).
- E25 (Meta SapFix): every machine-generated fix needs human or
  independent-agent approval and cannot publish itself (R094).
- E26 (Meta rapid release and Gatekeeper): unfinished features stay
  behind off-by-default switches instead of branches (R083); releases go
  through candidate stages before final publication.
- E27 (Meta configuration management): configuration is a reviewed,
  versioned change with history (R025).
- E28 (Meta SEV culture): incidents are recorded and reviewed without
  blame, with follow-up actions filed in AutoIssues.
- E29 (JPL waivers): any exception to a rule needs a written waiver with
  the reason, scope and an independent review, kept in a register. A
  waiver never covers mandatory tests, safety rules or quota debt.

## Not adopted

- Fixed memory after start-up, pointer rules, preprocessor rules, C
  integer typedefs and MISRA rules: C-specific.
- Predictive test selection: not needed at this scale; the full suite
  runs.
- Hack and Zoncolan: not available for PHP 8.5.

## Approval and activation

Approving this mapping replaces the earlier draft's R01–R10 with E01–E29;
the earlier R11–R18 project rules stay. Each rule activates only when its
checker or test pattern is qualified with a seeded violation that the
stage refuses.
