# Development toolchain contract

Audience: agents and contributors. Outcome: implement one consistent checking
interface with the requested tools and explicit language coverage.

Status: requested toolchain. Packages and versions need admission and
qualification on an approved execution host. Listing a tool does not mean it
is installed, compatible with PHP 8.5 or Python, licensed for every use, or
sufficient for a gate.

## Single entry point

The justfile is the normal entry point once implemented. `AGENTS.md` points to
`just check` rather than maintaining a second changing command checklist.

- Command interface: `just check`
  - Responsibility: Full applicable pre-commit profile and evidence manifest
- Command interface: `just test`
  - Responsibility: Behavioural, property, regression and relevant
    integration tests
- Command interface: `just lint`
  - Responsibility: Approved style, maintainability and static security rules
- Command interface: `just typecheck`
  - Responsibility: Approved Python and PHP type/static checks
- Command interface: `just eval`
  - Responsibility: Explicitly selected deterministic or release
    evaluation profile
- Command interface: `just run`
  - Responsibility: Approved local development services on an allowed host
- Command interface: `just db`
  - Responsibility: Documented operations on Sutradhar-owned development data

`just db` is not permission for Python to access or write XenForo's database.
No command silently switches to paid APIs, downloads unadmitted models, starts
unauthorised services or moves computation to the Windows editing machine.

Profiles distinguish documentation, ordinary development, private development
evaluation, nightly deterministic checks and release qualification. Selection is
policy-driven and recorded; agents may not select a weaker profile to bypass a
failure. Required tools being unavailable blocks the corresponding stage.

## Requested tools

- Tool or tool group: just
  - Requested role: Common command entry point
  - Qualification needed: Profile selection, failure propagation and host
    refusal
- Tool or tool group: import-linter
  - Requested role: Python dependency boundaries
  - Qualification needed: Direct, indirect and forbidden runtime import cases
- Tool or tool group: pytest, pytest-cov
  - Requested role: Python behaviour and coverage
  - Qualification needed: Coverage scope, subprocess/native boundaries
    and missing branches
- Tool or tool group: pytest-xdist
  - Requested role: Parallel test workers
  - Qualification needed: Bounded aggregate memory and isolated fixtures
- Tool or tool group: pytest-randomly
  - Requested role: Order-dependence checks
  - Qualification needed: Reproducible seed and useful failure reproduction
- Tool or tool group: pytest-timeout
  - Requested role: Bounded test execution
  - Qualification needed: Cleanup and evidence preservation on timeout
- Tool or tool group: hypothesis
  - Requested role: Generated property cases
  - Qualification needed: Domain invariants and failure shrinking
- Tool or tool group: deptry
  - Requested role: Dependency declarations
  - Qualification needed: Optional packs and import ownership
- Tool or tool group: ruff, including S rules
  - Requested role: Python style and security patterns
  - Qualification needed: Project policy and justified applicability
- Tool or tool group: mypy
  - Requested role: Python type checking
  - Qualification needed: Strictness, third-party stubs and ignored diagnostics
- Tool or tool group: gitleaks
  - Requested role: Pre-commit secret scanning
  - Qualification needed: Candidate and generated-artifact detection
    without leaking secrets
- Tool or tool group: commitizen
  - Requested role: Conventional commit messages
  - Qualification needed: Ordinary commits and squash PR titles,
    including the required `manual` type
- Tool or tool group: release-please
  - Requested role: Weekly release proposals and changelog
  - Qualification needed: Protected two-stage release integration
- Tool or tool group: Dependabot
  - Requested role: Weekly grouped dependency proposals
  - Qualification needed: Admission, compatibility and full merge checks
- Tool or tool group: devcontainer
  - Requested role: Reproducible development environment
  - Qualification needed: Matches qualified CI; runs on Dell/cloud only
- Tool or tool group: schemathesis
  - Requested role: Contract-driven API tests
  - Qualification needed: Authentication, malformed inputs
    and stateful failure paths
- Tool or tool group: syrupy
  - Requested role: Extraction snapshots
  - Qualification needed: Reviewed semantic changes and no private content
- Tool or tool group: JSON Schema and generated Pydantic models
  - Requested role: Shared contracts
  - Qualification needed: Generator pinning, drift and both-language validation
- Tool or tool group: mutmut
  - Requested role: Hard-rule mutation testing
  - Qualification needed: Unsupported cases and unwaived
    equivalent-mutant handling
- Tool or tool group: pytest-benchmark
  - Requested role: Extraction and scoring comparisons
  - Qualification needed: Stable repeated measurements and comparable inputs

Named tools are requested, not interchangeable by agent preference. Ask before
replacing a named library. Additional tools need evidence-backed admission
through the approved process. Do not add Fossil, Sapling, Buck2, Bazel,
Pants or Nix.

The required weekly gardener PR title is `manual: weekly update`. Configure the
commit-title checks to admit that documented conventional-commit type rather
than rejecting it because a tool's default type list is narrower. It remains
subject to all documentation checks and must not trigger an unintended
product version bump.

PHP-specific lint, static analysis, behavioural, mutation and coverage
instruments must be selected and qualified for PHP 8.5 and the XenForo
integration. Python-only tools do not satisfy PHP requirements. Likewise, a
branch coverage tool alone does not satisfy decision or modified
condition/decision coverage. Browser code, generated code and other
first-party code need an explicit approved measurement route rather than
silent exclusion.

## Import rules

Production code must never import `lab/`. Core must never import a feature
module. A module may import its own internals and approved core contracts,
but must not reach into another module's implementation. Dependencies
interact through core interfaces and events. External dependencies still
require admission and belong to a declared module or core boundary.

Test direct and indirect imports, optional imports, dynamic plugin discovery and
registration side effects. A static import-linter pass does not alone prove all
runtime import paths obey the boundary.

## Contracts

Place cross-component JSON Schemas under `contracts/`. Generate Python Pydantic
models through a pinned, reviewed generator. PHP must validate the same boundary
contracts. Generated-output drift fails checks. Keep schema evolution,
compatible versions, errors and migration behaviour in the approved feature
specifications. Do not invent protocol fields merely to make a generator run.

## Reproducibility and evidence

Pin tool, runtime, generator, model and package versions in the appropriate
admitted manifests. Cache keys include the versions and relevant configuration.
Reusing an artifact must not hide an expired, changed or inapplicable result.
Test workers, native libraries, containers and background services all count
towards their declared aggregate resource profile.

The test fixture is synthetic. Private performance datasets stay on Dell, with
only scrubbed summaries allowed in public results. Public cloud checks never
receive licensed XenForo source or private forum exports. Developer containers
must not bake secrets or private datasets into a shareable image.

## Manual tools

The public manual uses the approved MkDocs Material stack: Mike, mkdocs-click,
mkdocs-git-revision-date-localized-plugin, mkdocs-llmstxt, Codespell,
Playwright, axe and mktestdocs. It has no mkdocstrings API reference.
Generated settings, commands, error codes and release notes must describe
actual implemented behaviour. The manual specification owns the complete
build and writing requirements.
