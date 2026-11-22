# ADR-0001: Languages and files

Superseded in part by [ADR-0021](0021-version-1-product-stack-and-scope.md)
(background processing is PostgreSQL outbox and our own Axum/Tokio
scheduler, not Kafka or Airflow).

Decided by the owner on 8 November 2026. Revised by ADR-0019,
[ADR-0020](0020-version-1-documentation-and-architecture-baseline.md) and
the owner's packaged-Python decision of 19 November 2026.

## Decision

Rust is the language for all first-party production services, integration,
adapters and substantive operations and developer tools. CSS with
Tailwind, strict TypeScript for small browser interop modules and PHP for
the two site add-ons remain allowed, with the other file kinds listed in
[AGENTS.md section 4](../../AGENTS.md#4-languages-and-files).

First-party Go is banned. Bounded operational shell is allowed under
ADR-0019. The product must use approved Python packages and libraries for
BGE, spaCy, Optuna and Airflow, but no first-party Python production code
is permitted. Rust owns integration through admitted embedding, packaged
executable or packaged service interfaces. Declarative configuration is
allowed; first-party production Airflow DAGs, Optuna objectives, adapters
and custom spaCy components are not.

First-party Python is limited to explicitly permitted non-production tests
and development checks. Ready-made tools and packages of any implementation
language require approved pins, hashes, licences and security checks.

## Why

A small number of explicit language boundaries keeps the product coherent
while reusing maintained Python-native libraries. The boundary prevents a
second first-party production stack without forcing risky reimplementation
of specialist algorithms.

## Trade-offs

The Rust/package bridge and declarative orchestration need compatible
locks and integration evidence. A package feature that needs custom Python
is blocked until a supported packaged or Rust-hosted route exists. Neither
shell nor a Python package may contain hidden first-party business rules or
alter protected release policy.

## How we check it

The language and artefact checks refuse first-party production Python,
including embedded or generated Python. Contract tests cover Rust/package
interfaces and failures. ShellCheck, shfmt and failure-path tests cover
shell. Dependency, licence, hash and security checks cover every Python
package and transitive artefact. The architecture check enforces protected
release authority.
