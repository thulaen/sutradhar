# SPEC-007 part 1: the 20× performance gate

Status: APPROVED. Protected-gate design awaiting the owner's approval.
Requirements: R090, R055, R021–R023. Part 2 is the
[engineering gates](engineering-gates.md).
Approved by the owner on 30 October 2026 (see the approval record in
`docs/specifications/approvals-2026-10-30.md`); it supersedes any
"awaiting approval" wording below.

## Results

For each candidate the gate reports one of:

- PASSED_SPEEDUP (result A): repeatable measurements show at least 20×.
- PASSED_LIMIT_REPORT (result B): a reviewed evidence report shows 20× is
  not reasonably achievable under the declared constraints, and no
  regression, incorrect output or resource-budget breach exists.
- VALID_REUSED: unchanged units whose earlier evidence still matches.
- NOT_APPLICABLE: the inventory proves no executable unit changed.
- FAILED or BLOCKED: anything else, including missing, noisy, stale or
  unsupported evidence and missing tooling.

Result B is a constrained engineering conclusion, not a proof that nothing
faster could exist.

## Units and mapping

The trusted inventory lists every changed executable unit of product and
tooling code (function, method, closure, query, script and generated
source). A protected map, `benchmarks/map.yaml`, assigns each unit to one
or more scenarios. Small helpers are measured through the representative
operation that calls them when isolated timings are unreliable. An
unmapped changed unit makes the gate BLOCKED; non-hotspot code is not
omitted (AT-P01).

Test code is not a performance unit: its speed is governed by the suite
time budgets (fast push suite under five minutes) and the slow-test rule
in AutoIssues, not by the 20× aim (AT-P10).

## Baselines

Each scenario has a fixed baseline chosen and reviewed before optimisation:

- changed code: the predecessor implementation of the same behaviour, from
  the last published commit;
- new code: a justified reference, normally the simplest correct idiomatic
  implementation or an established library doing the same work.

Baseline review rejects deliberately slow references: added sleeps,
pessimised algorithms, disabled caches the real system would have, or
unrealistic inputs. A baseline, once recorded for a scenario, is immutable;
a changed baseline needs a new review (AT-P02).

## Measurement

Baseline and candidate run in the runner's sandboxes on the same Dell
host, runtime, dependency lock, settings and inputs, and the runner signs
the results. Outputs must match, which the harness checks:

- for unchanged behaviour, baseline and candidate outputs are identical;
- for an intentional behaviour change (for example a bug fix), the
  comparison uses inputs whose expected output did not change, plus a
  reviewed reference for the new behaviour on the changed inputs; the
  candidate must match the candidate's own approved tests there
  (AT-P08, AT-P11).

Rules:

- pinned processor cores and recorded frequency governor, load and other
  tenants' activity; the host also runs another project's services, so
  runs are interleaved (baseline, candidate, baseline …) to share noise;
- warm-up runs discarded; at least 30 measured pairs, more until the 95%
  bootstrap interval of the speed-up ratio is narrower than ±10%;
- report median, interquartile range, interval and coefficient of
  variation; a coefficient over 10% makes the measurement noisy (BLOCKED);
- memory peak recorded for both; budgets in R021–R023 apply.

PASSED_SPEEDUP needs the lower bound of the interval at or above 20×
(AT-P03). Python uses pytest-benchmark plus the harness; PHP uses the
admitted PHP benchmark harness (PHPBench or an equivalent after admission).

## Limit report

PASSED_LIMIT_REPORT requires a report bound to the exact sources and
configuration, containing:

1. the baseline and candidate measurements above;
2. profiles (py-spy or cProfile for Python; the admitted PHP profiler) that
   identify the real bottleneck, after checking `bottlenecks.md`
   (AT-P04);
3. alternatives investigated: algorithmic, data-structure, batching,
   caching, I/O, concurrency within budgets, vectorised or native library
   routes, and database query changes, each with recorded experiments and
   reasons for rejection;
4. a practical limit analysis: for example the measured share of time in
   unavoidable I/O, the theoretical floor from Amdahl's law, or a lower
   bound on required work;
5. confirmation of no regression against the baseline (interval lower
   bound at least 1.0), correct output and resource budgets met;
6. an independent performance review recorded under R057 (AT-P05).

A report that excuses a regression, wrong output, weakened privacy, extra
memory or a missed budget fails.

## Profiling

Before any optimisation, and for every limit report, read the current
`bottlenecks.md` and profile the scenario; work from measured hotspots,
not guesses (AS-76). The report names the profile used and the hotspot it
targets. A performance fix for an AutoIssues issue attaches before and
after numbers from the same benchmark (AT-P04, AT-P12).

## Activation

Until the harness is qualified, changed executable code in the bootstrap
scope reports NOT_YET_ACTIVE with a reviewed limit report as the interim
result ([activation
order](autoissues/quota-recovery.md#bootstrap-and-activation-order)).
Outside that scope missing tooling is BLOCKED.

## Reuse and invalidation

Evidence is keyed by the hashes of the measured units and their call
dependencies, scenario definitions, inputs, dependency lock, runtime and
host fingerprint. Unchanged units reuse it; a change to any key component
re-evaluates. No compounded new 20× is demanded from unchanged code
(AT-P06).

## Commit and push

The commit profile runs the gate for changed units. The push profile and
publisher validate that the evidence keys match the exact candidate and
rerun measurements whose evidence is stale. Documentation-only candidates
get NOT_APPLICABLE from the inventory; absent tooling never gives
NOT_APPLICABLE (AT-P07).

## Acceptance cases

- AT-P01: An unmapped changed unit blocks the gate.
- AT-P02: A baseline containing an artificial sleep is rejected at review;
  changing a recorded baseline without review is refused.
- AT-P03: A fixture with a 25× median but a lower bound of 18× does not
  pass result A; one with a lower bound of 21× does.
- AT-P04: A limit report without profiles naming the bottleneck fails.
- AT-P05: A limit report without an independent review fails.
- AT-P06: Unchanged units reuse evidence; a dependency lock change
  invalidates it.
- AT-P07: A docs-only inventory is NOT_APPLICABLE; a missing benchmark tool
  with changed code is BLOCKED.
- AT-P08: Output mismatch between baseline and candidate fails, whatever
  the speed.
- AT-P09: A coefficient of variation over 10% is reported as noisy and
  blocks.
- AT-P10: A changed test function is not treated as an unmapped unit.
- AT-P11: A bug fix passes the output rule when unchanged inputs match the
  baseline and changed inputs match the reviewed new reference.
- AT-P12: A limit report or performance fix that does not name the
  profiled hotspot fails.

## Honest expectation

For much I/O-bound or already-efficient code, 20× will not be reachable and
result B will be the common honest outcome. Each limit report costs real
profiling and review time; that cost is part of this requirement.
