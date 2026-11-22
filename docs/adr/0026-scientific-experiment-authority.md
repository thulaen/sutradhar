# ADR-0026: Scientific experiment evidence and authority

The owner approved these design choices on 21 November 2026.
See DR-143 to DR-147 in the [decision register](../registers/decisions.md).

## Decision

- Keep 0094's randomised exposure and before-and-after rules. Allow a
  separate own-site observational route under 0148 and 0149 to issue
  a conditional causal estimate only after independent qualification
  of its assumptions, method, evidence, sensitivity bounds and proof
  for the registered comparison. A result whose assumptions cannot
  be supported remains inconclusive.
- Use the existing read-only Search Console Search Analytics and GA4
  Data API connectors and the local darshak aggregate API. Require
  a current qualified receipt from each at every verdict checkpoint.
  Pin one primary metric and keep each source's native population
  and units. Missing coverage holds the verdict. File imports, bulk
  export, Matomo and rival evidence cannot supply this route.
- Use a separate **Start experiment** action. Freeze the scope and
  evaluation rules before exposure. Ordinary cards and suggestions
  keep their existing workflows and outcome language.
- Require an exact owner confirmation for each qualified advisory
  boost, demotion, retirement or reactivation. Mandatory audit and
  safety rules keep their existing authority. Experimental data and
  derivatives stay outside live ranking, training, ranking trials,
  release proof and automated editing or publishing.
- Allow an independently proved false alert to clear its own hold
  only with exact owner confirmation before the final look. Keep
  both the alert and the correction. Check the original plan and
  deadline again. Confirmed contamination blocks that verdict.
  A later correction may withdraw a result without reopening it.

## Reason and consequences

An observational comparison can give evidence about a specific
change only under explicit assumptions. Give that route a separate
admission and label so an ordinary observed result or good pre-period
fit cannot gain causal authority. The separate experiment action
lets a person understand the protected group, metric and dates
before the study has an effect.

The 0072 evidence boundary gains this advisory reader alongside
the distinct ADR-0024 Update Lab reader. Neither reader grants
rights to the other's inputs or to any ranking, spam or detection
consumer. A narrower grant or shorter source lifetime continues
to govern derived evidence and stored results.

[0148](../specs/0148-scientific-experimentation-engine.md) owns
the plan and permission boundary.
[0149](../specs/0149-causal-estimators-and-placebos.md) owns
estimator qualification and outcomes.
[0150](../specs/0150-experiment-source-and-sensor-gates.md) owns
source and contamination receipts.
[0151](../specs/0151-site-lawbook-calibration-and-replay.md) owns
calibration, the Site Lawbook and replay custody.
