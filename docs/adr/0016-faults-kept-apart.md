# ADR-0016: Faults kept apart from the work list

Decided by the owner on 6 November 2026 ("No, keep them apart
(Recommended)") and again on 10 November 2026 ("Keep apart, link when
taken").

## Decision

- Faults found by tests, analysers, monitors and agents go to
  autoissues' own list (spec 0063), never into Forgejo's task list.
- A Forgejo task is made only when an agent takes a fault on to fix
  it. The task links to the fault, and the fault links back.
- Forgejo stays the place for status, tasks and milestones
  ([AGENTS.md section 2](../../AGENTS.md#2-where-the-truth-lives)).
- AGENTS.md section 8 states the rule. Spec 0063 is its one design,
  including what a linked task may hold, since Forgejo issues are
  copied to GitHub.
  Other pages point there rather than repeat it.

## Why

- Many faults repeat or turn out to be false. Copying each one into
  Forgejo would fill the backlog that AGENTS.md section 10 keeps small.
- One fault record keeps its history across repeats; a task is made
  only for real repair work.

## Trade-offs

- An owner reading Forgejo alone does not see faults nobody has taken
  on yet. The fault list on Dell shows them.
