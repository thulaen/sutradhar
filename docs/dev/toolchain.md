# Development toolchain contract

Audience: agents and contributors. Outcome: implement one consistent checking
interface with the requested tools and explicit language coverage.

Status: requested toolchain. Packages and versions need admission and qualification
on an approved execution host. Listing a tool does not mean it is installed,
compatible with PHP 8.5 or Python, licensed for every use, or sufficient for a gate.

## Single entry point

The justfile is the normal entry point once implemented. `AGENTS.md` points to
`just check` rather than maintaining a second changing command checklist.

| Command interface | Responsibility |
| --- | --- |
| `just check` | Full applicable pre-commit profile and evidence manifest |
| `just test` | Behavioural, property, regression and relevant integration tests |
| `just lint` | Approved style, maintainability and static security rules |
| `just typecheck` | Approved Python and PHP type/static checks |
| `just eval` | Explicitly selected deterministic or release evaluation profile |
| `just run` | Approved local development services on an allowed host |
| `just db` | Documented operations on Sutradhar-owned development data |

`just db` is not permission for Python to access or write XenForo's database.
No command silently switches to paid APIs, downloads unadmitted models, starts
unauthorised services or moves computation to the Windows editing machine.

Profiles distinguish documentation, ordinary development, private development
evaluation, nightly deterministic checks and release qualification. Selection is
policy-driven and recorded; agents may not select a weaker profile to bypass a
failure. Required tools being unavailable blocks the corresponding stage.

## Requested tools

| Tool or tool group | Requested role | Qualification needed |
| --- | --- | --- |
| just | Common command entry point | Profile selection, failure propagation and host refusal |
| import-linter | Python dependency boundaries | Direct, indirect and forbidden runtime import cases |
| pytest, pytest-cov | Python behaviour and coverage | Coverage scope, subprocess/native boundaries and missing branches |
| pytest-xdist | Parallel test workers | Bounded aggregate memory and isolated fixtures |
| pytest-randomly | Order-dependence checks | Reproducible seed and useful failure reproduction |
| pytest-timeout | Bounded test execution | Cleanup and evidence preservation on timeout |
| hypothesis | Generated property cases | Domain invariants and failure shrinking |
| deptry | Dependency declarations | Optional packs and import ownership |
| ruff, including S rules | Python style and security patterns | Project policy and justified applicability |
| mypy | Python type checking | Strictness, third-party stubs and ignored diagnostics |
| gitleaks | Pre-commit secret scanning | Candidate and generated-artifact detection without leaking secrets |
| commitizen | Conventional commit messages | Ordinary commits and squash PR titles, including the required `manual` type |
| release-please | Weekly release proposals and changelog | Protected two-stage release integration |
| Dependabot | Weekly grouped dependency proposals | Admission, compatibility and full merge checks |
| devcontainer | Reproducible development environment | Matches qualified CI; runs on Dell/cloud only |
| schemathesis | Contract-driven API tests | Authentication, malformed inputs and stateful failure paths |
| syrupy | Extraction snapshots | Reviewed semantic changes and no private content |
| JSON Schema and generated Pydantic models | Shared contracts | Generator pinning, drift and both-language validation |
| mutmut | Hard-rule mutation testing | Unsupported cases and unwaived equivalent-mutant handling |
| pytest-benchmark | Extraction and scoring comparisons | Stable repeated measurements and comparable inputs |

Named tools are requested, not interchangeable by agent preference. Ask before
replacing a named library. Additional tools need evidence-backed admission through
the approved process. Do not add Fossil, Sapling, Buck2, Bazel, Pants or Nix.

The required weekly gardener PR title is `manual: weekly update`. Configure the
commit-title checks to admit that documented conventional-commit type rather than
rejecting it because a tool's default type list is narrower. It remains subject to
all documentation checks and must not trigger an unintended product version bump.

PHP-specific lint, static analysis, behavioural, mutation and coverage instruments
must be selected and qualified for PHP 8.5 and the XenForo integration. Python-only
tools do not satisfy PHP requirements. Likewise, a branch coverage tool alone
does not satisfy decision or modified condition/decision coverage. Browser code,
generated code and other first-party code need an explicit approved measurement
route rather than silent exclusion.

## Import rules

Production code must never import `lab/`. Core must never import a feature module.
A module may import its own internals and approved core contracts, but must not
reach into another module's implementation. Dependencies interact through core
interfaces and events. External dependencies still require admission and belong
to a declared module or core boundary.

Test direct and indirect imports, optional imports, dynamic plugin discovery and
registration side effects. A static import-linter pass does not alone prove all
runtime import paths obey the boundary.

## Contracts

Place cross-component JSON Schemas under `contracts/`. Generate Python Pydantic
models through a pinned, reviewed generator. PHP must validate the same boundary
contracts. Generated-output drift fails checks. Keep schema evolution, compatible
versions, errors and migration behaviour in the approved feature specifications.
Do not invent protocol fields merely to make a generator run.

## Reproducibility and evidence

Pin tool, runtime, generator, model and package versions in the appropriate admitted
manifests. Cache keys include the versions and relevant configuration. Reusing an
artifact must not hide an expired, changed or inapplicable result. Test workers,
native libraries, containers and background services all count towards their
declared aggregate resource profile.

The test fixture is synthetic. Private performance datasets stay on Dell, with
only scrubbed summaries allowed in public results. Public cloud checks never
receive licensed XenForo source or private forum exports. Developer containers
must not bake secrets or private datasets into a shareable image.

## Manual tools

The public manual uses the approved MkDocs Material stack: Mike, mkdocs-click,
mkdocs-git-revision-date-localized-plugin, mkdocs-llmstxt, Codespell, Playwright,
axe and mktestdocs. It has no mkdocstrings API reference. Generated settings,
commands, error codes and release notes must describe actual implemented behaviour.
The manual specification owns the complete build and writing requirements.
