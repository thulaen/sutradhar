# Proposed mapped coding rulebook

Audience: the owner, implementers and reviewers. Outcome: approve one practical
PHP/Python rulebook with traceable influences and explicit enforcement.

Status: proposed mapping requiring owner approval. Existing interview
requirements remain binding; the additional mappings below are not silently
approved merely because this document exists. This is not NASA, JPL or Meta
certification, nor a claim that Sutradhar meets a safety-critical software
standard.

## Sources and limits

The [Power of Ten](https://www.spinroot.com/p10/) primarily addresses C. Its
memory-allocation, preprocessor and pointer rules cannot be copied literally
into ordinary PHP and Python and then described as enforced. The
[NASA coding-standard guidance](https://swehb.nasa.gov/pages/viewpage.action?navigatingVersions=true&pageId=85426354)
supports language-appropriate standards. Use the source to inform the rule, then
prove the adapted rule in this project's languages.

Meta's [Infer documentation](https://fbinfer.com/docs/about-Infer/) informs the
use of automated defect analysis and explicit handling of analysis scope. It is
not a universal Meta policy for every language. Its presence in this
bibliography does not establish a PHP 8.5 or Python checking route. Select and
qualify appropriate tools through the approved toolchain specification instead.

## Mapping proposed for approval

- R01
  - Source influence: Simple control flow
  - Sutradhar rule: Prefer small explicit state transitions; use iterative
    bounded workers for corpus traversal. Reject hidden unbounded recursion.
  - Acceptance evidence: Static checks and adversarial depth tests
- R02
  - Source influence: Bounded loops
  - Sutradhar rule: Every input-sized loop has a declared chunk, limit,
    deadline or resumable cursor.
  - Acceptance evidence: Oversized-input, cancellation and restart tests
- R03
  - Source influence: Predictable memory
  - Sutradhar rule: Bound allocations, queues, model loading and worker totals
    instead of pretending dynamic allocation is absent.
  - Acceptance evidence: Aggregate peak measurements and hard-limit recovery
- R04
  - Source influence: Small units
  - Sutradhar rule: Keep PHP/Python files within the agreed 300 code lines
    and 80 columns; split responsibilities rather than compressing code.
  - Acceptance evidence: Source measurement and maintainability review
- R05
  - Source influence: Invariant checking
  - Sutradhar rule: Assert meaningful internal invariants without side effects;
    validate untrusted data with production checks that cannot disappear.
  - Acceptance evidence: Boundary tests and disabled-assertion tests
- R06
  - Source influence: Narrow scope
  - Sutradhar rule: Keep mutable state local or explicitly owned; avoid
    hidden global service access.
  - Acceptance evidence: Ownership review and isolated tests
- R07
  - Source influence: Checked results
  - Sutradhar rule: Validate external inputs and handle failures, return
    values and partial results deliberately.
  - Acceptance evidence: Malformed data and fault-injection tests
- R08
  - Source influence: Limited preprocessing
  - Sutradhar rule: Keep code generation explicit, pinned and reviewable; no
    opaque dynamic generation at runtime.
  - Acceptance evidence: Regeneration comparison and generated-output review
- R09
  - Source influence: Limited indirection
  - Sutradhar rule: Prefer typed contracts and explicit ownership over
    reflective access and hidden cross-module coupling.
  - Acceptance evidence: Static contracts and runtime-boundary tests
- R10
  - Source influence: Warnings and analysis
  - Sutradhar rule: Treat applicable runtime warnings and analyzer failures
    as blockers; qualify coverage and limitations.
  - Acceptance evidence: Deliberately seeded defects and diagnostic checks

These are independently written project adaptations. They do not quote or adopt
every original rule verbatim. In particular, no arbitrary assertion count is
used as a substitute for useful tests, and no PHP/Python process is claimed to
avoid all allocation after startup.

## Additional project rules

- R11
  - Rule: Analyse changed code and relevant dependencies; do not ignore whole
    directories to obtain a pass.
  - Evidence: Scope manifest and seeded violation checks
- R12
  - Rule: Track ownership across async work, retries and shutdown.
  - Evidence: Race, cancellation and stale-worker tests
- R13
  - Rule: Preserve cleanup and resource-release paths on every failure.
  - Evidence: Fault-injection and leak checks
- R14
  - Rule: Make trust boundaries visible in APIs and schemas.
  - Evidence: Contract tests, permission tests and review
- R15
  - Rule: Keep production independent of the laboratory and modules
    independent of each other's internals.
  - Evidence: Static and dynamic import checks
- R16
  - Rule: Keep a single reviewed implementation for each behaviour.
  - Evidence: Duplication review and notebook import checks
- R17
  - Rule: Use behaviour-first specifications and tests before implementation.
  - Evidence: Approved specification and failing-to-passing evidence
- R18
  - Rule: Preserve exact evidence scope, provenance and uncertainty.
  - Evidence: Current-candidate receipts and reviewer checks

R11–R14 reflect the project's application of static-analysis and resource-safety
ideas; they are not represented as verbatim Meta rules. R15–R18 express the
owner's architecture and evidence requirements. Use DRY, KISS and SOLID as
concrete design review criteria, not scores that can pass merely because the
acronyms appear in a document.

## Binding requirements already agreed

The owner's interview separately requires 100% first-party line, function,
branch, decision and modified condition/decision coverage; the complete review
and gate set; independent rights admission; source preservation; no known real
performance regression; memory limits; and the no-bypass release rules. The
[gate contract](gates.md) owns their full definitions. This mapping cannot
weaken those requirements or substitute a smaller tool-supported subset.

## Admission and enforcement

Before activating this mapping:

1. The owner approves its revision and any new restrictions it introduces.
2. Each rule has an identified language-specific checker or an explicitly scoped
   review/test combination.
3. Seed a violating example and confirm the required stage refuses it.
4. Demonstrate the rule on PHP 8.5, Python, generated output and relevant
   boundaries.
5. Record unsupported cases as blockers or propose a rule revision for approval.

A checker name is not enforcement evidence. An annotation that silences a
finding must not become an informal waiver. Fix the issue or change the
approved rule through the protected process, preserving the reason and
evidence.
