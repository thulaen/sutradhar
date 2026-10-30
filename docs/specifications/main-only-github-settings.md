# SPEC-004 settings: GitHub changes for main-only publication

Status: APPROVED. Nothing below has been changed on GitHub. The owner
applies these steps through the GitHub website after approval; agents do
not change security settings.
Approved by the owner on 30 October 2026 (see the approval record in
`docs/specifications/approvals-2026-10-30.md`); it supersedes any
"awaiting approval" wording below.

Live state read on 30 October 2026 (public API): `main` at `093b001`,
no other branches, no pull requests, no commit statuses. Ruleset
"Sutradhar protected main" (26172923) requires a pull request (squash only)
and nine status checks without any App binding; it also blocks deletion
and force pushes and requires linear history. Two tag rulesets block tag
updates, deletion and creation.

## Why each change is needed

- The existing pull-request rule makes main-only publication impossible.
  Keeping it while also publishing directly would claim two incompatible
  rules at once, so it must go.
- Without a pull-request rule, `main` must instead be closed to everyone
  except the Publisher App, while still requiring checks, so that a direct
  `git push` by anyone, including the owner, is refused.
- Rules in one ruleset are all bypassed by that ruleset's bypass actors.
  The Publisher's permission to move `main` therefore lives in its own
  ruleset, and the integrity rules live in another with no bypass, so the
  Publisher is still bound by checks, fast-forward-only and no deletion.
- Required checks are bound to the App allowed to report them, so a
  similarly named status from anyone else cannot satisfy them.
- A third ruleset stops anyone creating any other branch.
- Pull requests are switched off so nobody starts work that cannot land.

## Rulesets

1. "Sutradhar protected main" (existing 26172923): remove "Require a pull
   request before merging". Keep deletion, force-push and linear-history
   rules. Edit each required check to name its source App: the Attester
   for every check except `owner approval`, which names the approval
   verifier. Add `AutoIssues gate` and `performance gate` only when those
   gates activate. Leave the bypass list empty.
2. New "Sutradhar main publisher": target the default branch; rules
   "Restrict updates" and "Restrict creations"; bypass list: the Publisher
   App only, mode "always" so each use is audited.
3. New "Sutradhar no other branches": target all branches except `main`;
   rules "Restrict creations" and "Restrict updates"; empty bypass list.
4. Tag rulesets: unchanged. A later release design may add the release
   App as the only tag creator.

## Repository settings

- Settings, General, Features: turn off Pull requests.
- Keep squash merging allowed, because GitHub's linear-history rule needs
  it. Auto-merge has no effect once pull requests are off.
- Code security: keep Dependabot alerts on; keep Dependabot security
  update pull requests and version updates off until a main-only
  dependency design is approved.
- Actions workflow permissions: read-only by default (unchanged).

## Apps

- Existing proposal App: rename to "Sutradhar publisher"; permissions
  Contents write and Metadata read only; remove Pull requests and any
  other write permission. Workflows write is added later only if an
  approved change must publish workflow files.
- Register "Sutradhar attester": Commit statuses write, Checks write,
  Metadata read; install on this repository only.
- Register "Sutradhar approval verifier": Commit statuses write, Issues
  write (to open approval notification issues), Metadata read; install on
  this repository only.
- Each private key is downloaded once and moved straight to its own
  protected Dell location, then removed from the downloading computer.
- Retire the older unused publisher key that remains registered.

## Owner credentials

- Reduce the GitHub credential stored on the editing computer to
  read-only; it no longer needs to push. Agents run there.
- Record the owner's immutable GitHub user number in the protected policy
  for approval comments
  ([approval route](main-only-publication.md#owner-approval-comment)).
- GitHub shows ruleset bypass lists only to identities that can edit the
  rules, so no App receives that access. The owner checks the bypass
  lists when approving protected changes and in each quarterly review.

## Qualification before the real repository

GitHub has no dry-run mode for public repositories on this plan, so the
refusal behaviour must be tested where it cannot harm Sutradhar. The
proposal is a temporary public test repository owned by the owner, with
the same three rulesets and the three Apps installed. Tests:

- Q1: checks reported on an uploaded, unreferenced commit, then a
  fast-forward of `main`: accepted only when every required check passed
  from the right App.
- Q2: the Publisher is refused for a missing check, a force update and a
  non-linear commit.
- Q3: the owner's own `git push` to `main` and to a new branch are refused.
- Q4: a status with a required name posted by another identity does not
  satisfy the rule.
- Q5: an update touching workflow files without Workflows permission is
  refused.
- Q6: publisher crash and recovery at each journal step.

If the owner prefers not to create a test repository, the alternative is
testing only refusals that cannot change Sutradhar's `main` (Q3, Q4) on
the real repository and treating the first real publication as the
positive test. That gives weaker evidence and is not recommended.

## Limits that cannot be removed

A repository administrator, including the owner, can always edit or delete
rulesets and App permissions. The design detects this rather than
preventing it: the publisher reads the active rules for `main` before
each publication and refuses when they differ from the approved
configuration. GitHub hides bypass lists from non-administrators, so a
separate owner-run check of the bypass lists is part of each quarterly
review.
