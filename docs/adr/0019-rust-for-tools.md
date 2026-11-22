# ADR-0019: Rust, bounded shell and packaged Python libraries

Superseded in part by [ADR-0021](0021-version-1-product-stack-and-scope.md)
(background processing is PostgreSQL outbox and our own Axum/Tokio
scheduler, not Kafka or Airflow; Apalis is retained).

Decided by the owner on 17 November 2026. Revised on 18 November 2026 by
[ADR-0020](0020-version-1-documentation-and-architecture-baseline.md) and
on 19 November 2026 by the owner's packaged-Python decision. It replaces
the Go and shell policy of ADR-0017. ADR-0017 remains only as history.

## Decision

- Rust is the language for all first-party production services,
  integration, adapters, orchestration definitions expressed as code,
  substantive operations and developer tools.
- First-party Go is banned. No migration is specified without an inventory
  proving that the component exists. Pinned third-party Go programs are
  allowed under the normal licence, hash and security controls.
- Bounded operational shell is allowed with strict failure options, fixed
  argument arrays, explicit environments and working directories,
  ShellCheck, shfmt and failure-path tests. It contains no `eval`, business
  logic, structured-data parsing or commands constructed from untrusted
  text.
- The product must use approved Python packages and libraries for the
  Python-native BGE, spaCy, Optuna and Airflow capabilities. Their
  algorithms are not reimplemented in Rust merely to avoid the Python
  runtime.
- No first-party Python production code is permitted. This bans hand-written
  or generated `.py` production modules, Airflow DAGs, operators, hooks,
  sensors, callables, Optuna objectives, model-serving adapters, evaluation
  adapters and custom spaCy components.
- Rust invokes pinned packages through an admitted embedding bridge,
  supported packaged executable or packaged service interface. Declarative
  TOML, YAML and JSON configuration is permitted. Airflow uses declarative
  DAG definitions consumed by a pinned DAG-factory package and packaged
  operators; custom Python callables are forbidden.
- Python packages cannot autonomously alter permissions, source integrity,
  evaluation labels or thresholds, protected configuration or release
  control. Rust retains all first-party product policy and release
  authority.
- Airflow replaces Apalis. Kafka and Airflow own background delivery and
  orchestration under spec 0019; Rust/Tokio remains appropriate for
  product-facing workers and integration.
- First-party Python may be used only for non-production tests and local
  development checks where the owning specification permits it. Such files
  are never shipped, loaded by production or used to circumvent the ban.

The allowed-language list and exact boundaries are in
[AGENTS.md section 4](../../AGENTS.md#4-languages-and-files).

## Why

Rust provides strong first-party product boundaries and memory safety.
The approved optimisation and NLP ecosystems are Python-native and their
maintained packages should be reused. Package reuse is different from
creating a second first-party production codebase: the latter would split
ownership, review and operational authority.

## Trade-offs

The project needs an admitted Rust/Python bridge, declarative Airflow
configuration, exact compatible dependency locks and more integration
evidence. Some package capabilities may not be usable without custom
Python. Such a capability is blocked until a supported packaged or
Rust-hosted route exists; a Python adapter is not an acceptable shortcut.
Shell remains unsuitable for business rules and structured-data processing.
Package autonomy never includes release authority.

## Alternatives not chosen

- First-party Go: prohibited by the owner.
- First-party Python production services, DAGs or adapters: prohibited by
  the owner on 19 November 2026.
- Pure-Rust reimplementation of BGE, spaCy or Optuna: rejected because it
  would discard maintained packages and create unnecessary parity risk.
- Unbounded shell: rejected because it would erase module and trust
  boundaries.
- A universal 300-line commit cap: replaced by coherent, reviewable
  changes and serial integration.

## How we check it

The language check refuses every first-party production `.py` file and
Python source embedded in configuration, templates or generated artefacts.
Dependency checks prove exact package and transitive-artefact locks.
Architecture checks require the admitted bridge or packaged interface and
refuse custom Airflow callables and spaCy components. Rust/package contract
tests cover every input, output and failure. Architecture checks prevent
packages or Airflow from changing protected release policy. Resource tests
admit only measured profiles within the applicable memory and latency
budgets.
