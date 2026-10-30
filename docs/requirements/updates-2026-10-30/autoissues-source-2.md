# AutoIssues source register, part 2

Status: continuation of [part 1](autoissues-source.md). Same conventions.

## Ingest adapters

- AS-44: every adapter maps findings to issues with fingerprint, type,
  severity, component, evidence and a "signal present in this run" answer
  used for verification. Spec: adapters "Contract". Tests: AT-D00.
- AS-45: pytest JUnit XML from `main` and nightly runs only. Failures become
  bug issues; a test failing and passing in one run, or across pytest-repeat,
  becomes flaky_test; tests slower than 5 seconds become P3 with the
  duration. CHANGED by R083: "never from pull requests" becomes "never from
  unpublished development runs". PHP: PHPUnit JUnit XML is mapped the same
  way. Spec: adapters "Tests". Tests: AT-D02, AT-D03, AT-D04.
- AS-46: pytest-benchmark JSON against the stored baseline; a regression
  over 20% becomes performance P2 with before and after numbers; the nightly
  runs py-spy on the regressed benchmark and attaches its flame graph.
  Spec: adapters "Benchmarks". Tests: AT-D05, AT-D06.
- AS-47: coverage XML; total or hard-rule branch coverage below threshold
  becomes coverage P2 with uncovered lines. The threshold is the agreed 100%
  first-party requirement (R054). Spec: adapters "Coverage". Tests: AT-D07.
- AS-48: mutmut survivors on hard-rule code become mutation_survivor P2, one
  issue per function. PHP survivors from the admitted PHP mutation tool are
  mapped the same way. Spec: adapters "Mutation". Tests: AT-D08.
- AS-49: SonarQube issues created since the last run, fetched through its
  API; BLOCKER and CRITICAL to P1, MAJOR P2, MINOR P3, INFO P4; types bug,
  vulnerability, security hotspot and code smell; a failed quality gate is
  P1; fingerprint is the Sonar issue key. Spec: adapters "SonarQube".
  Tests: AT-D09, AT-D10.
- AS-50: GlitchTip unresolved issues polled through its Sentry-compatible
  API, plus alert webhooks; an error issue is bug P1 when events exceed the
  `triage.yaml` threshold, else P2, with event URL, culprit and stack
  summary; transactions whose p95 exceeds the endpoint budget become
  performance P2. Spec: adapters "GlitchTip". Tests: AT-D11, AT-D12.
- AS-51: Prometheus alert rules in `tools/qa/prometheus/rules.yaml` (job
  failed, dead letters, memory above budget, p95 above budget, embedding
  throughput below target, index stale) arrive through Alertmanager webhooks
  as alert issues fingerprinted by alert name plus labels.
  Spec: adapters "Alerts". Tests: AT-D13.
- AS-52: a nightly TraceQL query for spans over budget (database queries over
  200 ms, HTTP calls over 1 s, handler spans over 500 ms) files slow_span P3
  with trace ids. Spec: adapters "Tempo". Tests: AT-D14.
- AS-53: Pyroscope top CPU functions in nightly jobs and the soak; a function
  newly in the top ten, or growing over 50% since the previous nightly,
  becomes hotspot P3 with a flame graph link. Spec: adapters "Pyroscope".
  Tests: AT-D15.
- AS-54: Loki ERROR lines not tied to a GlitchTip event become bug P2 grouped
  by message template; a WARN spike over three times the seven-day median
  becomes P3. Spec: adapters "Loki". Tests: AT-D16.
- AS-55: SARIF from semgrep, trivy and ZAP and JSON from pip-audit and
  gitleaks: CRITICAL P0/S0, HIGH P1/S1, MEDIUM P2, LOW P3, with rule or CVE,
  location and fix version. PHP dependency findings use `composer audit`.
  Spec: adapters "Security". Tests: AT-D17, AT-D18.
- AS-56: lab metrics JSON: a hard-rule failure is P0; recall or
  anchor-agreement regression against the best previous run is P1; a failure
  cluster of ten or more cases is P2 with ten examples; contested cases are
  not issues. Spec: adapters "Evaluation". Tests: AT-D19.
- AS-57: each unique atheris crash becomes fuzz_crash P1 with its reproducer
  attached. A PHP fuzzer is added only if one is admitted.
  Spec: adapters "Fuzzing". Tests: AT-D20.
- AS-58: manual or documentation checks failing on `main` become docs P3.
  Spec: adapters "Documentation". Tests: AT-D21.
- AS-59: agents use `autoissues new` for findings outside the current task.
  Spec: adapters "Agents". Tests: AT-D22.

## QA stack

- AS-60: `tools/qa/docker-compose.qa.yaml` profiles: observe (grafana/otel-lgtm
  for Grafana, Tempo, Loki, Prometheus and the collector, plus Pyroscope),
  errors (GlitchTip with Postgres and Valkey) and sonar (SonarQube Community
  Build). Spec: qa-stack "Profiles". Tests: AT-Q01.
- AS-61: a `qa` settings profile turns on OTLP export, Prometheus scraping of
  `/actuator/metrics`, sentry-sdk to GlitchTip with traces_sample_rate 1.0,
  pyroscope-io profiling and JSON logs to Loki. Spec: qa-stack
  "Instrumentation". Tests: AT-Q05, AT-Q06.
- AS-62: prebuilt Grafana dashboards and alert rules in `tools/qa/grafana`
  and `tools/qa/prometheus`. Spec: qa-stack "Dashboards". Tests: AT-Q07.
- AS-63: nothing in `tools/qa` enters any wheel, image or release.
  Spec: README "Packaging". Tests: AT-A01, AT-A02.
- AS-64: `just qa` runs the ephemeral stack in sequence: tests, benchmarks,
  coverage and mutation with ingest; observe and errors profiles with the
  soak (locust against `/suggest` plus one full nightly job) and ingest from
  GlitchTip, Prometheus, Tempo, Pyroscope and Loki, then stop them; sonar
  alone for Python and PHP, ingest, stop; security scanners and ingest;
  evaluation adapters; export the snapshot. Spec: qa-stack "Sequence".
  Tests: AT-Q02, AT-Q03.
- AS-65: each stage has a memory ceiling; two heavy stages never run at
  once. The approved 8 GB lab budget governs (R092).
  Spec: qa-stack "Budgets". Tests: AT-Q03, AT-Q04.
- AS-66: `just observe` starts only the observe profile and prints the
  Grafana address. Spec: qa-stack "Interactive". Tests: AT-Q08.
- AS-67: `docs/dev/observability.md` documents every endpoint with example
  PromQL, TraceQL, LogQL and Pyroscope queries. Spec: qa-stack
  "Observability guide". Tests: AT-Q09.

## Agent rules

- AS-68: before a task run `autoissues next --json` on Dell, or read
  `autoissues/open.json` elsewhere. Spec: quota "Queue policy".
  Tests: AT-G01.
- AS-69: fix every open P0 anywhere and every P1 and P2 in the component
  being touched before the assigned task; no other work while a P0 is open.
  CHANGED by R094: work needed to fix the P0 stays allowed.
  Spec: quota "Queue policy". Tests: AT-G02, AT-G03, AT-G04.
- AS-70: claim an issue before working on it. Spec: lifecycle "Claims".
  Tests: AT-L03, AT-L24.
- AS-71: read and cite an issue's evidence in the change. CHANGED by R083:
  cited in the commit message and milestone record. Spec: lifecycle
  "Proof of fix". Tests: AT-L06.
- AS-72: a performance fix includes before and after numbers from the same
  benchmark. Spec: lifecycle "Proof of fix"; SPEC-007. Tests: AT-L17.
- AS-73: branch `fix/AI-<n>-<slug>` and trailer `Fixes: AI-<n>`. CHANGED by
  R083: no branch; the trailer remains. Spec: lifecycle "Proof of fix".
  Tests: AT-L06.
- AS-74: never mark an issue verified yourself. Spec: lifecycle
  "Verification". Tests: AT-L18.
- AS-75: file out-of-scope findings with `autoissues new` instead of fixing
  them. Spec: adapters "Agents". Tests: AT-D22.
- AS-76: check `bottlenecks.md` before performance work and work from
  measured hotspots. Spec: SPEC-007 "Profiling". Tests: AT-P04, AT-P12
  (defined in the performance gate specification).
- AS-77: a weekly sweep clears P3 and P4 issues oldest first, batched by
  rule. Spec: lifecycle "Sweeps". Tests: AT-L19.

## Done when

- AS-78: every adapter is tested with recorded fixtures, including
  fingerprint stability, verification after three green runs, regression
  reopening, baseline grouping and the flood cap. Tests: the per-adapter
  matrix in acceptance part 2 (AT-D01, AT-L07, AT-L10, AT-L14, AT-L15 run
  for every adapter).
- AS-79: `just qa` runs end to end on Dell within its memory ceilings and
  produces a snapshot. CHANGED by R095: no pull request. Tests: AT-Q02.
- AS-80: the dashboard runs headless and interactively. Tests: AT-I08.
- AS-81: seed the tracker with one `just qa` run and report the filed issues
  by priority and the top ten bottlenecks. Product-dependent stages (soak,
  nightly job, evaluation) report "no target yet" until the product exists;
  they become mandatory when it does. Tests: AT-Q10, AT-Q11.
