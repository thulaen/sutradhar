# SPEC-005 adapters

Status: APPROVED. Part of [SPEC-005](README.md).
Approved by the owner on 30 October 2026 (see the approval record in
`docs/specifications/approvals-2026-10-30.md`); it supersedes any
"awaiting approval" wording below.

## Contract

An adapter is a pure parser plus a small fetcher, run by the gate account.
It never executes candidate code; reports produced by candidate code come
from the runner's sandbox as signed bundles, and service data is fetched
by the gate inside the QA network namespace. The fetcher stores each report
as run evidence with its SHA-256. The parser turns the stored report into
findings, each with:
fingerprint, type, priority hint, severity hint, path for triage, title,
evidence references and summary. It also returns the set of fingerprints
the source was able to observe in this run and a completeness flag, which
together answer "is this signal present in this run" for verification.
A run whose report is missing, truncated or unparsable is incomplete and
cannot verify anything (AT-D00).

Parsers are deterministic and side-effect free. Report sizes, item counts
and HTTP response sizes are bounded; exceeding a bound makes the run
incomplete rather than silently truncating it.

## Fingerprints

Fingerprints are SHA-256 over a versioned, normalised key. Normalisation
removes run-specific noise (timestamps, absolute paths outside the
repository, memory addresses, random identifiers, line numbers where the
source gives a stable rule location) and keeps meaning. Keys per source:

- tests: test node id plus failure class;
- SonarQube: issue key;
- GlitchTip: issue id;
- alerts: alert name plus sorted labels, excluding volatile labels;
- SARIF: rule id plus normalised location;
- dependency advisories: advisory or CVE id plus package name;
- benchmarks: benchmark name;
- Pyroscope: function plus module;
- Tempo: TraceQL span signature (service, span name, kind);
- Loki: message template; evaluations: rule and cluster id.

Each adapter has fixture pairs proving the same finding from two runs gives
the same fingerprint and different findings do not collide (AT-D01).

## Tests

JUnit XML from pytest and PHPUnit for published `main` candidates and
nightly runs. Failures and errors file bug issues. A test that both fails
and passes in one run, including under pytest-repeat, files flaky_test.
Tests slower than 5 seconds file P3 performance issues with the duration
(AT-D02, AT-D03, AT-D04). Development runs never ingest.

## Benchmarks

pytest-benchmark JSON (and the PHP benchmark format chosen in SPEC-007) is
compared with the stored baseline for the same benchmark identity and
environment. A median slowdown over 20%, confirmed by the repeated-measure
rule in SPEC-007, files performance P2 with before and after numbers
(AT-D05). The nightly then profiles the regressed benchmark with py-spy
(or the admitted PHP profiler) and attaches the flame graph (AT-D06).

## Coverage

Coverage XML (coverage.py for Python; the admitted PHP coverage driver).
Any first-party line or branch below the required 100% files coverage P2
with the uncovered lines; hard-rule code is tagged for priority
(AT-D07). Decision and MC/DC evidence come from the gate reports in
SPEC-001, not from this adapter.

## Mutation

mutmut results for Python and the admitted PHP mutation tool's report.
Surviving mutants file mutation_survivor P2, one issue per function, with
the mutant diffs as evidence (AT-D08).

## SonarQube

After each analysis, fetch issues created since the last run through the
SonarQube web API. Map BLOCKER and CRITICAL to P1, MAJOR P2, MINOR P3,
INFO P4; keep bug, vulnerability, security hotspot and code smell types; a
failed quality gate files P1. Webhook payloads trigger the same fetch
(AT-D09, AT-D10).

## GlitchTip

Poll unresolved issues through GlitchTip's Sentry-compatible API and accept
its alert webhooks. An error issue is bug P1 when events exceed the
`triage.yaml` threshold, else P2, with event URL, culprit and stack summary.
Transactions whose p95 exceeds the endpoint budget file performance P2
(AT-D11, AT-D12).

## Alerts

Alert rules in `tools/qa/prometheus/rules.yaml`: job failed, dead letters,
memory above budget, p95 above budget, embedding throughput below target,
index stale. Alertmanager webhooks file alert issues. A resolved webhook
only triggers the gate to query Prometheus; absence for verification comes
from that query, never from the webhook alone (AT-D13).

## Tempo

A nightly TraceQL search for spans over budget: database queries over
200 ms, HTTP calls over 1 s, handler spans over 500 ms. Each signature files
slow_span P3 with up to ten trace ids (AT-D14).

## Pyroscope

Top functions by CPU in nightly jobs and the soak. A function newly in the
top ten, or with self-CPU growth over 50% since the previous nightly, files
hotspot P3 with a flame graph link (AT-D15). "Newly in the top ten" is a
rank-relative signal: it is tracked but earns no quota credit, because
fixing one hotspot can promote another (SPEC-006 credit rule 3).

## Loki

ERROR lines without a GlitchTip event id file bug P2, grouped by message
template. A WARN volume over three times the seven-day median files P3
(AT-D16).

## Security

SARIF from semgrep, trivy and ZAP; JSON from pip-audit, gitleaks and
`composer audit`. CRITICAL is P0/S0, HIGH P1/S1, MEDIUM P2, LOW P3, with
rule or advisory, location and fix version. A gitleaks finding never copies
the secret value into the store or a snapshot; only the rule and location
(AT-D17, AT-D18).

## Evaluation

Lab metrics JSON from the evaluation specification (F023). A hard-rule
failure is P0; a recall or anchor-agreement regression against the best
previous run is P1; a failure cluster of ten or more cases is P2 with ten
example identifiers; contested cases are not issues (AT-D19). Example
identifiers point to private Dell evidence, never post text.

## Fuzzing

Each unique atheris crash files fuzz_crash P1 with the reproducer stored as
private evidence. A PHP fuzzer is added only after admission (AT-D20).

## Documentation

Manual and documentation checks failing on published `main` file docs P3
(AT-D21).

## Agents

`autoissues new` files an agent finding with reporter set to the agent and
a required evidence reference. Agent findings start in the
`agent-reported` hotlist and need confirmation (a reproducer or an
independent adapter signal) before they can earn quota credit (AT-D22).
