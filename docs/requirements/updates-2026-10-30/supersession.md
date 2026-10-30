# Rules replaced on 30 October 2026

Status: reconciliation record. Earlier text is preserved where it was
written; it must not be implemented after its replacement. Only the
conflicting part of each earlier rule is replaced; the rest stays in force.

Read with the [requirements update](README.md),
[current decisions](../current-decisions.md) and the
[earlier overrides](../overrides.md).

## Branches, pull requests and publication

- R068 (agents prepare branches and pull requests): the branch and pull
  request part is replaced by R083 and R084. The separate GitHub App
  identity, no release tags from agents and no owner impersonation remain.
- R069 (full checks before commit; push rechecks; bot pre-commit exception):
  checks before commit and push rechecks remain. GitHub reviews and merge
  checks after pushing are replaced by publisher checks before `main`
  advances (R084). The Dependabot and Release Please exception is suspended
  until a main-only design is approved (R083, R095).
- R070 (pull-request-only `main`, squash merges, auto-merge): replaced by
  R083 and R084. No direct, force or deletion updates by anyone except the
  qualified publisher's fast-forward remain. Owner approval of protected
  rules and outside contributions remains.
- R071 (revert pull request when `main` breaks): the revert remains the
  first response, now published as a revert commit through the publisher.
  The weekly Release Please proposal flow needs a main-only design.
- R080 (manual previews for pull requests; the `gh-pages` branch): there
  are no pull requests and no second branch. `dev` after `main` updates,
  frozen versions and `latest` rules remain. The replacement is the
  proposed
  [SPEC-002 revision](../../specifications/manual-publishing-main-only.md).
- R076 (Release Please; weekly grouped Dependabot): both work through
  pull requests. The proposed
  [SPEC-003 release revision](../../specifications/release-main-only.md)
  replaces them, pending the owner's decision.
- [Release contract](../../dev/releasing.md) steps 1–3 (Release Please
  branch and pull request), the pull-request preview list, the
  `gh-pages` hosting text and the revert pull request: replaced by the two
  revisions above and R071 as amended.
- Q193 and Q196 (proposal branches, pull requests, trusted auto-merge):
  replaced by R083 and R084. Q201's separate App identity remains.
- Q204 (bot proposal commits before checks): suspended, see R069 above.
- AGENTS.md "Commit, push and review", as committed in `691559a`: replaced
  by its "Trunk-based development on main" section.
- [Documentation bootstrap](../../dev/bootstrap.md) steps 7 and 8 (push a
  branch, open a pull request): replaced by publication through the
  main-only publisher. The documentation-only profile itself remains.
- [Earlier handoff](../../dev/claude-handoff-2026-10-30.md) step 5 (publish
  `691559a` to `docs/project-foundation` and open a draft pull request):
  replaced. The fixed proposal-branch publisher policy is retired as a
  publication route; its preserved modules and evidence remain inputs.
- [Gate contract](../../dev/gates.md) "Merge" stage and the
  [repository setup](../../dev/repository-setup.md) pull-request ruleset and
  auto-merge settings: replaced by the publish stage and settings in
  [SPEC-004](../../specifications/main-only-publication.md).
- [SPEC-003](../../specifications/agent-release-coordination.md) section
  "Repository admission" and its auto-merge acceptance example: replaced by
  SPEC-004 once approved. The rest of SPEC-003 keeps its approval.

## Reviews

- R057 (required GitHub AI review): the AI review remains mandatory. Without
  pull requests it runs through the qualified main-only review process in
  SPEC-004. Code, performance and security reviews and honest self-review
  labelling are unchanged.

## Source limits

- R056 (80 columns and 300 code lines for PHP and Python; documentation has
  no line cap): replaced by R089. The limits now count physical lines, apply
  to every first-party text file including documentation, and add a 60-line
  function limit.
- AGENTS.md "Implementation boundaries" limit bullet and
  [gate contract](../../dev/gates.md) "Engineering checks" limit paragraph:
  replaced by R089.

## Engineering practice and performance

- R055 (controlled repeated comparisons; mapped NASA/Meta rulebook):
  extended by R088 (primary-source research and per-rule enforcement) and
  R090 (20× gate). "No known real regression" remains.
- R007 and rulebook R17 (behaviour-first, test-first plan): strengthened by
  R087 into mandatory red, green, refactor evidence.

## AutoIssues request

The attachment is itemised in the [source register](autoissues-source.md).
These parts are replaced, not dropped:

- Fix branches `fix/AI-<n>-<slug>`: replaced by commits on `main` carrying
  `Fixes: AI-<n>` (R083).
- "A merged commit" sets fixed: replaced by a published `main` commit.
- Nightly snapshot pull request through the bot exception: replaced by Dell
  nightly exports plus reviewed snapshots published with milestones (R095).
- Test results "from main and nightly runs only, never from PRs": the rule
  becomes published-`main` candidates and nightly runs only; uncommitted
  development runs never file issues.
- "Libraries already in the product": no product code exists yet. The
  statement is a requirement to verify, not a fact (R092).
- 16 GB Dell memory: the current host was measured; the approved 8 GB lab
  budget governs QA stages (R092).

## Not changed

R001–R006, R008–R054, R058–R067, R072–R075, R077–R079, R081–R082 and
every requirement not named above remain as recorded. R007 is strengthened
by R087; R055–R057, R068–R071, R076 and R080 are changed only as listed
above. SPEC-001, SPEC-002 and SPEC-003 keep their approval except for
the sections listed in the
[approval delta](../../specifications/approval-delta-2026-10-30.md).
