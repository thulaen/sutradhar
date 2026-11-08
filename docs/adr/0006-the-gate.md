# ADR-0006: The gate

Decided by the owner on 8 November 2026.

## Decision

Every check and every [gate item](../standards/gate-items.md) blocks at
every tier, with subagent reviews and waivers only from the owner. The
rules are in [AGENTS.md section 7](../../AGENTS.md#7-the-gate).

## Why

- A check that can be skipped will be skipped. Blocking at every step
  stops defects before they pile up.
- Running the whole project at every commit would take hours. So the
  commit tier covers the changed code and everything that depends on
  it, and the push tier covers everything. The tiers grow in scope, not
  in leniency.
- Subagent reviews keep costs down while staying independent: each
  reviewer starts fresh and never wrote the change.

## Trade-offs

- Each commit costs time and usage. We accept this for quality.

## How we check it

- Each check is proven to refuse a planted fault in every allowed
  language that is switched on.
- A signed full run lists every check, its result and its proof.
