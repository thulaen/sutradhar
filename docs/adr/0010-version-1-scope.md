# ADR-0010: Version 1 scope and release bar

Decided by the owner on 8 November 2026.

## Decision

Version 1 holds the full scope and the release bar in
[AGENTS.md section 3](../../AGENTS.md#3-the-goal-and-the-release-bar).

## Why

The owner wants version 1 to be complete and proven, not a partial
product.

## Trade-offs

The release bar is very high, and the variant count alone needs very
large computing time. Phase 7 may take a long time. Counts must be
honest: a method, capability or variant counts only with its evidence.

## How we check it

The release gate counts methods, capabilities and variants from signed
evidence, and refuses a release below the bar.
