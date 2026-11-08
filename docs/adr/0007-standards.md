# ADR-0007: Coding standards

Decided by the owner on 8 November 2026.

## Decision

Google style everywhere, one rule matrix drawn from the strictest safety
standards, our own MC/DC tool, and proofs for critical code. The rules
are in [AGENTS.md section 6](../../AGENTS.md#6-code-standards).

## Why

- These standards come from systems where failure is not allowed. They
  give clear, checkable rules: small functions, bounded loops, no hidden
  state, every decision tested.
- MC/DC proves each condition in a decision really changes the outcome.
  No ready-made tool does this for Rust, so we build one.

## Trade-offs

- These are quality targets, not certificates. No outside body certifies
  Sutradhar.
- Our MC/DC tool must itself be proven on known cases before it counts.

## How we check it

- Each matrix cell has a check, or a subagent checklist where no tool can
  exist, and a planted violation it must refuse.
- The MC/DC tool is tested against hand-worked examples.
