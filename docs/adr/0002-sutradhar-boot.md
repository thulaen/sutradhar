# ADR-0002: Sutradhar Boot

Decided by the owner on 8 November 2026.

## Decision

Build Sutradhar Boot, our own Rust application framework in the spirit of
Spring Boot, and build every part of the product on it. Its features are
listed in [AGENTS.md section 5.1](../../AGENTS.md#51-sutradhar-boot).

## Why

- Spring Boot's strengths (one place that wires the parts, typed
  settings with profiles, starters, health checks, one error shape,
  clear module walls) make large systems easy to test and change.
- Doing it in Rust keeps one language and checks the wiring when the
  code is compiled, not when it runs.
- Typed in-app events saved first in an outbox give the safety of a
  message server without running one.

## Trade-offs

- We own and maintain the framework. To keep it small, it wraps
  maintained libraries (Axum, Tokio, Tower, Tracing, Serde, sqlx) and
  adds only the glue.

## How we check it

- The architecture check refuses module cycles, reaching into another
  module's insides, globals, singletons and service locators.
- Tests prove start-up order, wiring failures, bad settings, health,
  graceful shutdown and that no event is lost when the service is
  killed mid-write.
