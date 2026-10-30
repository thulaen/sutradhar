# Approval delta: 30 October 2026 requirements update

Status: APPROVED. Identifies exactly which parts of the approved designs
change and which keep their existing approval. Approved design bodies are
not edited; their replacements live in new documents.
Approved by the owner on 30 October 2026 (see the approval record in
`docs/specifications/approvals-2026-10-30.md`); it supersedes any
"awaiting approval" wording below.

Read with the [approval record](approvals.md) and the
[supersession table](../requirements/updates-2026-10-30/supersession.md).

## SPEC-001: toolchain and check runner

Keeps approval: everything not listed below, including the profiles, the
structured result, PASSED/FAILED/BLOCKED/NOT_APPLICABLE outcomes, failure
behaviour and the Windows execution ban.

Changed, replacement awaiting approval:

- "Behaviour", paragraph on GitHub AI review and merge checks after
  pushing: replaced by SPEC-004's pre-publication review and checks.
- "Behaviour", recognised proposal-bot exception: suspended (R083).
- Profile contents: add the test-first evidence, source limits and
  performance gate from SPEC-007 and the queue and quota gate from
  SPEC-006 to the commit and push profiles.
- Outcomes: add NOT_YET_ACTIVE, accepted only inside the bootstrap scope
  and activation order in
  [quota recovery](autoissues/quota-recovery.md).
- Execution: candidate code runs only in the runner's sandboxes and
  results are signed, under the
  [trust boundaries](trust-boundaries.md).
- Acceptance examples: add SPEC-006 and SPEC-007 cases.

## SPEC-002: manual publishing

Keeps approval: separation of the manual from developer documents,
generation, honest "not yet available" treatment, version retention and
`latest` promotion only after release qualification.

Changed, replacement awaiting approval:

- "Build and publication": pull-request previews and the `gh-pages`
  branch. Replaced by the proposed
  [manual publication revision](manual-publishing-main-only.md): private
  candidate previews on Dell and Pages deployment through GitHub Actions
  without a branch.

## SPEC-003: agent and release coordination

Keeps approval: trust boundaries and separate identities, joining,
ownership and the private run ledger, availability and pause rules,
reviews, the release sequence's gates and its acceptance examples other
than those below.

Changed, replacement awaiting approval:

- "Repository admission": replaced by
  [SPEC-004](main-only-publication.md).
- "Trust boundaries": "workers submit bounded patch/branch/PR requests"
  becomes main-only publication requests to the protected publisher; the
  publisher may fast-forward `main` and nothing else.
- "Release sequence": Release Please's weekly proposal pull request and
  Dependabot's update pull requests. Replaced by the proposed
  [release revision](release-main-only.md); until approved, no release
  preparation or dependency update runs.
- Acceptance examples "a worker requests a main update … the publisher
  refuses it" and "an ordinary trusted PR … can auto-merge": replaced by
  SPEC-004's cases. Refusal of tags, releases and arbitrary endpoints
  remains.

## New designs awaiting approval

- [Trust boundaries](trust-boundaries.md): sandboxed execution, signed
  evidence, separate accounts and hardware-signed owner approval, shared
  by SPEC-004 to SPEC-007.
- [SPEC-004](main-only-publication.md): main-only protected publication
  and the exact GitHub settings and App permission changes.
- [SPEC-005](autoissues/README.md): AutoIssues and its QA integrations.
- [SPEC-006](autoissues/quota.md): queue and quota gate.
- [SPEC-007](performance-gate.md) with its
  [engineering gates](engineering-gates.md): 20× performance gate,
  test-first evidence and source limits.
- [Mapped rulebook](../dev/rulebook-mapping.md): the NASA/JPL and Meta
  mapping for PHP and Python.
- [Live data storage](../dev/storage-live-data.md): the storage revision
  separating live databases from preserved snapshots (R096).

Approval of these is requested as one grouped decision, with the separate
choices listed in each document. Nothing here is implemented yet.
