# SPEC-005 tool support record

Status: research evidence, 30 October 2026, from primary sources
(package indexes, official documentation and release notes), with an
independent re-check of the decision-critical claims. Nothing was
installed or run. This is not an admission record: each tool still needs
version pinning, licence and rights review and a qualification run on
Dell before use.

## Confirmed for the project's versions

Python 3.14 support stated by the project:

- pytest 9.2.0 (JUnit XML; xunit2 default), pytest-benchmark 5.3.0,
  coverage.py 7.17.0 (branch coverage under the default `sys.monitoring`
  core; that core lacks dynamic contexts and plugins), mutmut 3.8.1
  (needs `os.fork`; Linux only), atheris 3.1.0 (Linux x86_64 wheel only),
  py-spy 0.4.2, pip-audit 2.10.1 (JSON, no SARIF), FastAPI 0.162.0 (still
  0.x), marimo 0.25.2, PyYAML 6.0.3, ruamel.yaml 0.19.1, sentry-sdk
  2.73.0, opentelemetry-sdk 1.45.2, prometheus-client 0.26.0, locust
  2.47.0, semgrep CLI 1.183.0.

PHP 8.5 support stated by the project:

- PHPUnit 13.4.5 (JUnit XML), Infection 0.35.8, Xdebug 3.5.3 (coverage
  driver), PHPStan 2.3.4, Psalm 6.20.4.

Services:

- SonarQube Community Build 26.10: Python 3.14 and PHP 8.5 analysis
  stated since 25.11; about 4 GB memory for up to one million lines.
- GlitchTip 6.2 (MIT): 512 MB recommended; PostgreSQL 14 or newer.
- grafana/otel-lgtm 0.38.1: stated to be for development and testing only,
  which matches this use; bundles Prometheus, Tempo, Loki, Pyroscope.
- Alertmanager 0.34.2, Trivy 0.76.1 (SARIF), OWASP ZAP 2.17.0 (SARIF
  report template), gitleaks 8.30.1 (SARIF and JSON).

## Not confirmed: these block their adapter until resolved

- pytest-repeat 0.9.4: no Python 3.14 statement; no release since April
  2025. Alternative: detect flakiness from repeated runs in the harness.
- pyroscope-io 1.4.0: Python 3.14 wheels exist but no written support
  statement.
- PCOV 1.0.12: no PHP 8.5 statement upstream. Xdebug is the confirmed
  coverage driver.
- Pyroscope for PHP: no PHP SDK; only CPU profiles through eBPF with
  Grafana Alloy. The PHP hotspot adapter needs a separate design.
- nikic/php-fuzzer 0.0.11: described by its author as experimental; no PHP
  8.5 statement. No PHP fuzzer is assumed.
- semgrep: PHP 8.5 syntax support not stated.
- Composer 2.11.0: its own PHP 8.5 support was not checked.

## Rights and operating notes

- Semgrep community rules are licensed for internal business use only and
  may not be redistributed or offered as a service. Internal scanning
  fits; rules must not be copied into this public repository.
- Tempo, Loki and Pyroscope are AGPL-3.0. They run unmodified as local
  development services and are not distributed; rights review must record
  that.
- SonarQube refuses webhooks to loopback and private addresses by default
  and never retries; the local receiver needs that instance setting changed
  on Sutradhar's own SonarQube only, plus polling as a backstop.
- `composer audit` exit code 1 means findings or a failed run; the adapter
  must parse JSON, never rely on the exit code.
- Pyroscope and Loki have no authentication; they stay inside the QA
  namespace behind the read-only proxy.
- GlitchTip webhook signing was not found; its webhook endpoint relies on
  a shared secret in the URL path plus the QA namespace boundary.
- Tempo search results can vary between identical queries; the Tempo
  adapter treats a run as complete only when the query reports no
  truncation, and verification uses repeated nightly runs.

## Sources

The full research reports, with every source address and the re-check log,
are kept as private session evidence on Dell; addresses longer than the
line limit are not reproduced here.
