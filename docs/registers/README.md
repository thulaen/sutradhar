# Project registers

These registers hold the facts every agent needs before it edits
anything. They record decisions, constraints, risks, open questions and
sources. They do not record work status: Forgejo holds that. A register
row may be superseded, but it is never deleted; a new row names the row
it replaces.

## How the registers are used

- Read the constraints and decisions before you design or code.
- When an owner answer changes a rule, add a decision row first, then
  change the rule file or specification (ADR-0020's decision hierarchy).
- A row that contradicts another is a blocking documentation defect.
  Raise it as a question row; do not choose silently.
- Evidence beats memory: a fact about the outside world needs a source row
  with a retrieval date, or it stays an unverified claim.

- A row marked "Owner confirmed in chat" is an agent's record of an owner
  answer given in a session, checked only against that session's transcript,
  not signed. A signed record replaces it when one exists.

## Identifiers

Identifiers are stable and never reused.

| Prefix | Register | File |
|---|---|---|
| CR-nnn | Constraint | [constraints.md](constraints.md) |
| DR-nnn | Decision | [decisions.md](decisions.md) |
| RK-nnn | Risk | [risks.md](risks.md) |
| QR-nnn | Question | [questions.md](questions.md) |
| SR-nnn | Source | [sources.md](sources.md) |

Other identifiers keep their own places: ADR numbers in
[docs/adr](../adr/), specification numbers in [docs/specs](../specs/),
words in the [glossary](../glossary.md), gate items in
[docs/standards/gate-items.md](../standards/gate-items.md) and tasks in
Forgejo.

## Document index

| Need | Where |
|---|---|
| Rules for agents and contributors | [AGENTS.md](../../AGENTS.md) |
| Why each choice was made | [docs/adr](../adr/) |
| What each part does, its limits and tests | [docs/specs](../specs/) |
| Features, one owning spec each | [docs/specs/features.md](../specs/features.md) |
| Route to version 1 | [docs/roadmap.md](../roadmap.md) |
| Words and names | [docs/glossary.md](../glossary.md) |
| Style and vocabulary | [docs/standards](../standards/) |
| Research on technical choices, with experiments | [docs/research/dossier.md](../research/dossier.md) |
| Constraints, decisions, risks, questions, sources | This folder |

## Roles

A role is a hat, not a person. One agent or person may wear several, but
an author never reviews their own change.

| Role | Responsibility |
|---|---|
| Owner | The human who decides scope, approves waivers and signs with the owner key. Only the owner changes a goal, a phase or a protected rule. |
| Specification owner | Keeps one specification true and free of overlap. |
| Implementer | Writes tests first, then code, for one slice. |
| Reviewer | Reviews a change they did not write, as a fresh subagent. |
| Release approver | An agent that approves a release only when every check, review and quality bar passes at 100% (ADR-0021). Cannot waive a check. |
| Research agent | Reads public sources, records them in the source register and never copies restricted code. |
| Scout | Monthly agent that reads Spring Boot's public release notes (ADR-0022). |
| Register keeper | Adds rows and keeps identifiers stable. |

## Change control

A change to a rule, scope, stack or interface needs: the owner's answer,
a decision row, the affected specifications named, the test and
documentation impact stated, and the tasks re-cut in Forgejo. A decision
may not be weakened to match code, and a test may not be weakened to hide
a defect.
