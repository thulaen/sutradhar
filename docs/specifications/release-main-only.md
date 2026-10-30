# SPEC-003 revision: releases and dependency updates on main only

Status: APPROVED replacement for SPEC-003's "Release sequence" step that
uses a Release Please proposal pull request, and for the weekly grouped
Dependabot pull requests named in R076. Everything else in
[SPEC-003](agent-release-coordination.md) and the
[release contract](../dev/releasing.md) keeps its approval and gates.
Awaiting the owner.
Approved by the owner on 30 October 2026 (see the approval record in
`docs/specifications/approvals-2026-10-30.md`); it supersedes any
"awaiting approval" wording below.

## Why

Release Please prepares versions through a release pull request on a
proposal branch, and Dependabot's version and security updates arrive as
pull requests on branches. Main-only development (R083) allows neither.
R076 names both tools, so replacing them is the owner's decision.

## Version and changelog preparation

Proposed: Commitizen, already required by R076 for conventional commits,
prepares the version bump and changelog on Dell with its local bump
command, which needs no branch or pull request. The weekly preparation job:

1. starts only when `main` is green and the release scope check allows a
   candidate (the full release stays blocked until all agreed scope and
   6,000 qualified methods exist; before then the job reports why and
   stops);
2. runs the bump on a clean Dell checkout of `main`, producing one version
   commit with the changelog and version files;
3. runs the full commit and push profiles on it like any other commit;
4. hands it to the main-only publisher (SPEC-004).

Release Please is then not used. The alternative is to keep Release Please
only for changelog generation through its library interface, if its
current version supports that without pull requests; that must be verified
before it could be chosen.

## Candidate and final tags

Tags are not branches and remain allowed. After the version commit is on
`main`, the release identity creates the candidate tag (for example
`v1.0.0-rc.1`) on that exact commit and starts the durable release run.
The final tag, the GitHub release (a draft until every gate passes) and the
frozen manual asset follow only after success, for the same commit. The tag
rulesets gain the release App as the only creator. Coding agents never
create tags.

## Dependency updates

Dependabot alerts stay on and feed the AutoIssues security adapter, as do
`pip-audit`, `composer audit` and Trivy. Dependabot version updates and
security update pull requests stay off. A weekly dependency task updates
the locked versions on Dell, grouped as R076 asked (Python, PHP, tools),
runs the full profiles including the 20× gate's reuse and invalidation
rules, admits any new dependency through the rights process and publishes
one milestone commit per group through the publisher. A security finding
rated P0 or P1 is handled immediately under the AutoIssues queue policy.

## Revert

A broken `main` gets a revert commit through the publisher as the first
response (R071 as amended).

## Acceptance cases

- AT-R01: the weekly job refuses to prepare a release while required scope
  is incomplete and reports the reason.
- AT-R02: the version commit passes the same profiles as any commit and
  is published only through SPEC-004.
- AT-R03: only the release identity can create candidate and final tags;
  an agent's tag attempt is refused.
- AT-R04: the final tag points to the same commit as the passing
  candidate; a changed commit needs a new candidate.
- AT-R05: a dependency group update that fails any gate is not published
  and files AutoIssues issues.
- AT-R06: no branch or pull request is created by any release or
  dependency step.

## Decisions requested

- Replace Release Please with Commitizen's local bump (recommended), or
  investigate a pull-request-free Release Please route first.
- Replace Dependabot version and security pull requests with the weekly
  Dell dependency task, keeping Dependabot alerts.
