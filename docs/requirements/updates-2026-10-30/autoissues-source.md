# AutoIssues source register, part 1

Status: owner request itemised on 30 October 2026. Every substantive item
of the attached AutoIssues request appears here or in
[part 2](autoissues-source-2.md) with its specification section and
acceptance tests. Items marked CHANGED are altered only by the named later
rule; nothing is dropped.

Specification files live in
[SPEC-005](../../specifications/autoissues/README.md) and
[SPEC-006](../../specifications/autoissues/quota.md). Test identifiers are
defined in the
[acceptance catalogue](../../specifications/autoissues/acceptance.md) and
[its second part](../../specifications/autoissues/acceptance-2.md).

## Scope and packaging

- AS-01: a Buganizer-inspired defect tracker named autoissues, fed by a
  development-only QA stack. Spec: README. Tests: AT-A01.
- AS-02: tracker, QA stack, dashboards and compose files live under `tools/`
  and are excluded from product packages. Spec: README "Packaging".
  Tests: AT-A01, AT-A02.
- AS-03: product instrumentation (OpenTelemetry, prometheus-client,
  sentry-sdk) stays, off by default, switched on only by the QA profile.
  CHANGED by R092: existence must be verified and each library admitted.
  Spec: qa-stack "Instrumentation". Tests: AT-Q05, AT-Q06.

## Storage

- AS-04: importable package `autoissues` with a CLI `autoissues`.
  Spec: interfaces "CLI". Tests: AT-I01.
- AS-05: one SQLite file in WAL mode, default path
  `tools/autoissues/data/autoissues.db`, ignored by Git. CHANGED by R096:
  the live database lives in the authorised Dell live-data area; the
  in-repository path is only a Git-ignored default for disposable tests.
  Spec: storage "Location". Tests: AT-S01.
- AS-06: schema version and migrations. Spec: storage "Migrations".
  Tests: AT-S02, AT-S03.
- AS-07: single writer; ingest runs serialised through a lock.
  Spec: storage "Single writer". Tests: AT-S04, AT-S05.

## Data model

- AS-08: issue identifier `AI-<sequence>`. Spec: storage "Issues".
  Tests: AT-S06.
- AS-09: title and type, one of bug, flaky_test, performance, regression,
  security, code_quality, coverage, mutation_survivor, hotspot, slow_span,
  alert, eval_gate, fuzz_crash, docs, task. Spec: storage "Issues".
  Tests: AT-S07.
- AS-10: component path in a tree defined in
  `tools/autoissues/components.yaml`, each with default hotlists and SLOs.
  Spec: lifecycle "Triage". Tests: AT-L01.
- AS-11: priority P0–P4 and severity S0–S4. Spec: storage "Issues".
  Tests: AT-S07.
- AS-12: status new, triaged, assigned, in_progress, fixed, verified,
  closed, wont_fix, not_reproducible, duplicate. Spec: lifecycle "States".
  Tests: AT-S07, AT-L16.
- AS-13: resolution reason intended, infeasible or obsolete.
  Spec: lifecycle "Suppression". Tests: AT-L11.
- AS-14: assignee (agent name) with lease expiry. Spec: lifecycle "Claims".
  Tests: AT-L03.
- AS-15: reporter (adapter name or agent). Spec: storage "Issues".
  Tests: AT-S07.
- AS-16: fingerprint unique while open. Spec: storage "Issues".
  Tests: AT-S08.
- AS-17: occurrences: count, first_seen, last_seen, last_run_id.
  Spec: lifecycle "Duplicates". Tests: AT-L05.
- AS-18: sla_due, created, updated. Spec: lifecycle "Triage".
  Tests: AT-L02.
- AS-19: evidence rows with kind log, trace_id, flamegraph, sonar_rule,
  glitchtip_event, benchmark_before_after, sarif, reproducer or url, a path
  or URL and a summary. Spec: storage "Evidence". Tests: AT-S09.
- AS-20: links blocks, blocked_by, duplicate_of, parent, child.
  Spec: storage "Links". Tests: AT-S10.
- AS-21: hotlists and issue_hotlists. Spec: storage "Hotlists".
  Tests: AT-S11.
- AS-22: history of every field change with who, when, old and new.
  Spec: storage "History". Tests: AT-S12.
- AS-23: comments. Spec: storage "Comments". Tests: AT-S13.
- AS-24: runs with source, started, finished and counts filed, updated,
  verified and suppressed. Spec: storage "Runs". Tests: AT-S14.

## Lifecycle

- AS-25: automatic triage from `tools/autoissues/triage.yaml`: component
  from path globs, priority and severity from source and finding severity,
  SLO from priority. Spec: lifecycle "Triage". Tests: AT-L01, AT-L02.
- AS-26: claiming assigns with a 24-hour lease; an expired lease returns the
  issue to triaged. Spec: lifecycle "Claims". Tests: AT-L03, AT-L04.
- AS-27: a commit with trailer `Fixes: AI-<n>` sets fixed. CHANGED by R083
  and R094: a published `main` commit whose fix was independently checked.
  Spec: lifecycle "Proof of fix". Tests: AT-L06.
- AS-28: only the system sets verified: the fingerprint is absent from
  three consecutive runs of its source on `main`, or two nightlies.
  Spec: lifecycle "Verification". Tests: AT-L07.
- AS-29: closed follows verified after seven days.
  Spec: lifecycle "Verification". Tests: AT-L08.
- AS-30: a fingerprint reappearing on a closed issue reopens it as a
  regression one priority higher. Spec: lifecycle "Reopening".
  Tests: AT-L10.
- AS-31: duplicate detection by fingerprint, with stable fingerprints per
  source: test node id plus failure class; Sonar issue key; GlitchTip issue
  id; alert name plus labels; SARIF rule id plus normalised location; CVE
  plus package; benchmark name; Pyroscope function plus module; TraceQL
  span signature. Spec: adapters "Fingerprints". Tests: AT-D01, AT-L05.
- AS-32: wont_fix is allowed only when the same change adds a suppression
  (Sonar exclusion, `nosemgrep` with reason, baseline entry or threshold
  change) with written justification; the CLI refuses wont_fix without a
  referenced commit. CHANGED by R083: "same pull request" becomes the same
  published commit. Spec: lifecycle "Suppression". Tests: AT-L11, AT-L12.
- AS-33: a suppressed signal that persists after the suppression is
  published reopens the issue. Spec: lifecycle "Suppression".
  Tests: AT-L13.
- AS-34: on a source's first run, P3–P4 findings go to `baseline.yaml` and
  one grouped issue per rule with its file list; later findings are
  individual. Spec: lifecycle "Flood control". Tests: AT-L14.
- AS-35: per-adapter run cap, default 50 new issues; beyond it, one adapter
  flood issue and the adapter stops. Spec: lifecycle "Flood control".
  Tests: AT-L15.

## Interfaces

- AS-36: `--json` on every CLI command. Spec: interfaces "CLI".
  Tests: AT-I02.
- AS-37: commands next (priority, then SLA due, then age; filters by
  component and type), list, show, claim, release, new, comment, link,
  hotlist, fixed, wontfix, dupe, verify-run (system only), bottlenecks,
  export and stats. Spec: interfaces "CLI". Tests: AT-I03, AT-I04.
- AS-38: a small FastAPI application in `tools/autoissues/api` for GlitchTip,
  Alertmanager and SonarQube webhooks and the CLI operations, bound to
  localhost by default. Spec: interfaces "API". Tests: AT-I05, AT-I06.
- AS-39: a marimo dashboard `tools/autoissues/dashboard.py`: open issues by
  component, priority, type and age; SLO breaches; filed versus verified per
  week; regression count; bottlenecks. Spec: interfaces "Dashboard".
  Tests: AT-I07.
- AS-40: the dashboard runs headless and interactively.
  Spec: interfaces "Dashboard". Tests: AT-I08.
- AS-41: `autoissues export --snapshot` writes `autoissues/open.json`,
  `autoissues/summary.md` (counts, SLO breaches, needs-owner-decision
  hotlist) and `autoissues/bottlenecks.md`. Spec: interfaces "Snapshots".
  Tests: AT-I09, AT-I12.
- AS-42: nightly snapshot committed through a bot exception as an
  auto-merged pull request. CHANGED by R095: nightly exports stay on Dell;
  reviewed snapshots are published with milestones through the publisher.
  Spec: interfaces "Snapshots". Tests: AT-I10.
- AS-43: bottlenecks view and file: top 20 CPU functions from Pyroscope for
  nightly jobs and the soak with change since the previous nightly; slowest
  20 Tempo spans by p95 with trace links; slowest endpoints and transactions
  from GlitchTip and Prometheus with p50, p95 and p99; slowest 20 tests;
  slowest jobs; largest soak memory growth; every row over a threshold shows
  its issue id. Spec: interfaces "Bottlenecks". Tests: AT-I11.
