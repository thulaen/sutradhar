# SPEC-005 acceptance cases, part 2

Status: APPROVED. Continues [part 1](acceptance.md).
Approved by the owner on 30 October 2026 (see the approval record in
`docs/specifications/approvals-2026-10-30.md`); it supersedes any
"awaiting approval" wording below.

## Adapters

Every adapter case uses recorded fixtures of that source's real output
format, captured from admitted tool versions with synthetic data.

Per-adapter matrix (AS-78): AT-D01 and the lifecycle cases AT-L07
(verification after clean runs), AT-L10 (regression reopening), AT-L14
(baseline grouping) and AT-L15 (flood cap) are each run once for every
adapter, using that adapter's own fixtures. A new adapter is not admitted
until its row of the matrix passes.

- AT-D00: Missing, truncated, oversized or unparsable reports make the run
  incomplete; incomplete runs never verify.
- AT-D01: For each adapter, the same finding in two runs has one
  fingerprint; distinct findings do not collide; volatile fields do not
  change it.
- AT-D02: pytest and PHPUnit JUnit failures and errors file bug issues.
- AT-D03: Fail-then-pass in one run or under pytest-repeat files
  flaky_test, not bug.
- AT-D04: A test over 5 seconds files P3 with its duration.
- AT-D05: A confirmed benchmark slowdown over 20% files performance P2
  with before and after numbers; noise inside the confidence rule does not.
- AT-D06: The nightly attaches a flame graph to the regressed benchmark's
  issue.
- AT-D07: Any uncovered first-party line or branch files coverage P2 with
  the lines.
- AT-D08: Surviving mutants file one mutation_survivor P2 per function.
- AT-D09: Sonar severities map to P1–P4 and types are preserved.
- AT-D10: A failed Sonar quality gate files P1.
- AT-D11: GlitchTip issues over the event threshold file bug P1, else P2.
- AT-D12: Transactions with p95 over budget file performance P2.
- AT-D13: Alertmanager firing alerts file alert issues; a resolved alert
  counts as absence only after the gate's own Prometheus query confirms it.
- AT-D14: TraceQL results over the span budgets file slow_span P3 with
  trace ids.
- AT-D15: New top-ten or 50%-growth functions file hotspot P3.
- AT-D16: Loki ERROR lines without a GlitchTip id file grouped bug P2; a
  WARN spike over three times the seven-day median files P3.
- AT-D17: Security severities map CRITICAL P0/S0 to LOW P3 with rule,
  location and fix version.
- AT-D18: A gitleaks finding stores no secret value anywhere.
- AT-D19: Evaluation hard-rule failures file P0, recall or anchor
  regressions P1, clusters of ten or more P2; contested cases file nothing.
- AT-D20: Each unique fuzz crash files fuzz_crash P1 with its reproducer.
- AT-D21: Failing documentation checks on `main` file docs P3.
- AT-D22: `autoissues new` requires evidence and files into
  `agent-reported`; unconfirmed agent findings earn no credit.

## QA stack

- AT-Q01: Each Compose profile starts only its own services under the QA
  account with rootless Podman, pinned images, namespace-only ports and
  Sutradhar-owned volumes.
- AT-Q02: `just qa` runs all stages in order end to end on Dell and writes a
  snapshot to a new Dell directory.
- AT-Q03: A stage cannot start while a previous heavy stage still holds
  memory; an injected overlap is refused.
- AT-Q04: A process or container exceeding the slice limit is contained by
  the 8 GB slice; total lab memory never exceeds it.
- AT-Q05: The `qa` profile enables each exporter and data reaches the local
  services.
- AT-Q06: Default and production profiles open no exporter connection and
  send no telemetry.
- AT-Q07: All Grafana dashboards and alert rules load without errors.
- AT-Q08: `just observe` starts only the observe profile and prints the
  local Grafana address.
- AT-Q09: Every example query in the observability guide runs against the
  fixture stack.
- AT-Q10: A stage without a target reports NO_TARGET_YET with inventory
  proof; the same stage with a target present cannot report it.
- AT-Q11: The seeding run's report lists every filed issue grouped by
  priority and the top ten bottlenecks, and matches the store (AS-81).
- AT-Q12: An agent account cannot start, stop, write to or administer any
  QA service; its read-only queries through the proxy succeed.

## Queue and quota (SPEC-006)

- AT-G01: `next --json` is available to agents before a task and reflects
  live leases.
- AT-G02: With an open P0, an unrelated commit is BLOCKED at commit, push
  and publish; once the P0's fix is published the block lifts, and a
  reopen restores it.
- AT-G03: With an open P0 and unpaid debt, a repair-lane commit attributed
  to the P0 (including reproducer or suppression work) and confirmed by a
  reviewer other than the author is admitted.
- AT-G04: An open P1 or P2 in a component blocks feature changes touching
  it and permits its fix.
- AT-G05: The same tree failing at commit, push and publish, with the same
  or different failed checks, is charged once.
- AT-G06: Changing the system clock does not move the cutoff.
- AT-G07: Credit needs all six conditions; each missing condition alone
  denies it; issues grouped under one root cause earn one credit.
- AT-G08: A reopened issue loses its credit and adds one unit of debt; a
  later genuine fix may earn credit again, with no net gain.
- AT-G09: With debt 20 and 6 real open issues, fixing all 6 satisfies the
  round; no issue is invented.
- AT-G10: A verified empty store with no charges passes as verified zero;
  with an owner deferral it passes but is not labelled verified zero.
- AT-G11: A missing, corrupt, older or truncated store is BLOCKED.
- AT-G12: While debt is unpaid, a repair-lane commit is admitted and its
  fixes count as pending credit; a mixed commit is treated as a feature.
- AT-G13: A candidate changing only the gate is judged by the previous
  qualified gate version.
- AT-G14: With the previous version unable to run, the sandboxed candidate
  gate must reproduce earlier decisions on a replayed journal; with no
  evaluator, everything is blocked.
- AT-G18: Refusal messages contain correct counts and next steps and no
  secrets.
- AT-G19: A candidate omitting its test-first declaration while a test
  fails is QUALITY_FAILED, not BLOCKED.
- AT-G20: A candidate that edits a QA service's configuration and makes it
  fail is QUALITY_FAILED, not INFRA_UNAVAILABLE.
- AT-G21: An agent hotlisting issues as needing an owner decision, or
  marking a P0 as a duplicate, does not change the open count or lift the
  block.
- AT-G25: UPDATE or DELETE on any quota record table is refused.

AT-G15 to AT-G17 and AT-G22 to AT-G24 are defined in
[quota recovery](quota-recovery.md#acceptance-cases).
