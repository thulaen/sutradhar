# SPEC-001: toolchain and check runner

Status: APPROVED for bounded implementation. See [approval
record](approvals.md).

Audience: owner, maintainers and implementation agents.
Outcome: one trustworthy way to run applicable checks and explain why a change
may or may not be committed, pushed or merged.

Requirements: R007, R053–R058, R067, R069–R077 and R081.

## Behaviour

The justfile provides check, test, lint, typecheck, eval, run and db entry
points. The normal check entry point runs the complete applicable pre-commit
profile. Individual development commands help diagnose failures but do not
substitute for that profile. The runner reports the selected profile and every
required check, including explicit reasons for anything not applicable.

The initial documentation-only profile is narrowly limited to a change with no
product or executable tooling implementation. It still checks source capture,
links, consistency, privacy, secrets, rights and review evidence. Executable
source or workflow behaviour ends that bootstrap exemption. Historical quoted
source requests are data; the runner must never execute their contents.

Ordinary agent commits require completed applicable checks before the commit.
Before a commit exists, evidence binds the tested file/content manifest,
configuration, dependencies and environment. The resulting commit must match
that manifest; any mismatch invalidates the result. Push performs the agreed
fast recheck and validates unchanged evidence. GitHub's current-candidate AI
review and required merge checks occur after pushing.

Recognised dependency/release proposal bots alone may create proposal commits
before those checks. Their results remain untrusted until the full applicable
merge profile passes. Release judging and the 72-hour soak remain release-only;
ordinary code checks must never pretend to have run them.

## Interfaces and ownership

Keep the orchestration in small developer-tool modules, independent of the
production service and the notebook lab. Recipes call the same check definitions
locally on an authorised host and in CI. They do not contain separate competing
copies of the policy. The existing governing documents remain the home for the
rules, and this specification defines the runner's behaviour.

A structured result contains the candidate/content identity, profile and policy
version, environment identity, dependency lock identity, check identifiers,
outcomes, applicability reasons, timing, review provenance and evidence
references. Outcomes are PASSED, FAILED, BLOCKED or NOT_APPLICABLE. No absent or
skipped result is silently promoted to PASSED. Private raw output stays private;
only scrubbed summaries go to GitHub.

The protected coordinator in SPEC-003 validates and publishes required GitHub
statuses. Candidate code and ordinary workers cannot mint their own trusted pass
or owner approval. Changes to the checker, gate policy or workflow definitions
are protected changes requiring the owner, with the existing trusted checker
used to review the candidate.

## Requested tools and boundaries

Use the tools explicitly requested in Q192: import-linter, pytest-xdist,
pytest-randomly, pytest-timeout, deptry, Ruff security rules, gitleaks,
Commitizen, Release Please, Dependabot, a CI-matching devcontainer,
Schemathesis, Syrupy, mutmut and pytest-benchmark. Each executable and its
dependencies needs a pinned, licensed and compatible admission record.
The approved PHP/Python rulebook supplies the language-specific adapters;
qualification must prove their PHP 8.5 support and the requested coverage.

Production must not import lab code; core must not import feature modules;
feature modules use core contracts rather than reaching into other modules.
JSON Schemas are the canonical contract definitions, with generated Pydantic
models checked for consistency. Generation does not exempt first-party outputs
from applicable quality or source limits.

The Windows editing computer performs no test/build/dependency/cache work.
Public synthetic checks may run in isolated cloud workers. Private datasets,
licensed forum code and final performance measurements remain on the authorised
validation host. The devcontainer must not create a Windows execution loophole.
All work pauses when validation-host availability cannot be established.

## Failure behaviour

Missing tools, an unknown applicable check, bad reports, expired evidence,
changed content, unavailable private prerequisites and failed privacy scanning
produce FAILED or BLOCKED, not a reduced passing profile. Report the failing
check in plain English, its code and the next concrete action.

Timeouts terminate the relevant work safely and retain enough private evidence
to diagnose the outcome. Parallel test workers share the declared resource
budget. A worker crash or partial report cannot count as success. Tests that
modify tracked source are not allowed to change the candidate under review.

## Acceptance examples

- Given an initial documentation-only candidate, code-only checks may report
  NOT_APPLICABLE with proof; secrets, privacy, consistency and reviews still
  run.
- Given an executable file added to that candidate, the runner refuses the
  documentation-only profile and selects the applicable code profile.
- Given source changed after checks, commit/push admission rejects the stale
  evidence even if filenames and the branch name are unchanged.
- Given a forged, absent, truncated or skipped check result, the trusted
  publisher refuses a passing required status.
- Given a production-to-lab import, core-to-module import or module-to-module
  implementation import, the import contract fails.
- Given invalid API data or a changed schema/generated model mismatch,
  contract checks fail without sending private data to a hosted worker.
- Given a secret or identifying forum detail in a source file, artifact or log,
  the public-output gate fails and the unsafe output is withheld.
- Given parallel workers, random order, timeouts and mutation failures, the
  required result stays reproducible and reports the exact failing evidence.
- Given a development subset passing while a required check fails, the overall
  result remains failed. No subset can authorise an ordinary agent commit.

Qualification must also exercise the runner's own failure paths and required
first-party coverage. A target such as fast push checks under five minutes or
the private development evaluation under ten minutes is measured on the
qualified workload; it is not claimed from an empty repository.

## Approval and rollout

Approval covers this runner and its tests, not unrelated production features.
First qualify tool versions and check adapters, then build failing examples and
the runner, then bind the verified checks to the already protected repository.
Until real evidence publishers exist, required contexts remain pending rather
than being manually marked green for convenience.
