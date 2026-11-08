# ADR-0009: Agents and the backlog

Decided by the owner on 8 November 2026.

## Decision

- Claude, Codex, Gemini and later agents may work on the project, all
  under the same rules.
- Strict limits on open and ready work, ideas parked in "Later", a signed
  check to close each phase, and a weekly summary, as set out in
  [AGENTS.md section 10](../../AGENTS.md#10-keeping-the-backlog-small).
- File locks and one shared queue for heavy jobs, as set out in
  [AGENTS.md section 11](../../AGENTS.md#11-the-shared-computer-dell).

## Why

- The earlier backlog grew because ideas went straight into the current
  work and tasks were left half-finished. These rules stop both.
- Dell is shared by agents, Forgejo, PostgreSQL and the test sites, so
  heavy jobs must queue.

## How we check it

- The stop check refuses to end a session whose task has no proof or
  blocked reason.
- A daily check raises an urgent task when a limit is broken or the
  weekly summary is late.
