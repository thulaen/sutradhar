# ADR-0002: Sutradhar Boot

Superseded in part by [ADR-0021](0021-version-1-product-stack-and-scope.md)
(background processing is PostgreSQL outbox and our own Axum/Tokio
scheduler, not Kafka or Airflow).

Decided by the owner on 8 November 2026.
[ADR-0020](0020-version-1-documentation-and-architecture-baseline.md)
revises the event-delivery choice.
[ADR-0022](0022-behemoth-boot-and-leviathan.md) renamed Sutradhar Boot to
Behemoth Boot.

## Decision

Build Sutradhar Boot, our own Rust application framework in the spirit of
Spring Boot, and build the product-service parts on it. Its features are
listed in [AGENTS.md section 5.1](../../AGENTS.md#51-behemoth-boot).

Kafka carries durable platform events and Airflow orchestrates background
work under spec 0019. Boot supplies typed Rust product integration and
workers; it does not create a second PostgreSQL durable-job or product-
outbox execution path. Approved Python packages connect through admitted
embedding, packaged executable or packaged service interfaces. First-party
Python production modules never become hidden Boot modules.

## Why

- One composition root, typed settings, starters, health checks, one error
  shape and clear module walls make a large system easier to test and
  change.
- Rust keeps product-service wiring explicit and checked by the compiler.
- One platform event and orchestration route avoids competing durable job
  systems and gives replay and recovery one contract.

## Trade-offs

We own and maintain the framework. To keep it small, it wraps maintained
libraries and adds only the glue. Kafka/Airflow integration adds
idempotency, schema, recovery and resource obligations; a broker does not
by itself prove end-to-end exactly-once effects.

## How we check it

- The architecture check refuses module cycles, hidden access, globals,
  singletons, service locators and duplicate job executors.
- Tests prove start-up order, wiring failures, bad settings, health and
  graceful shutdown.
- Failure and replay tests prove that repeated delivery cannot repeat an
  externally visible side effect and that reconciliation detects missing
  or conflicting results.
