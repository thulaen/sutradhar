# ADR-0005: Publication through a gated repository on Dell

Status: accepted by the owner, 30 October 2026. Requirements: R112–R118.
The owner's exact words are in the publication
[decisions](../requirements/sources/publication-decisions-2026-10-30.md).

## Context

GitHub accepted `main` only through pull requests, while the owner's
rules forbid branches, disabling protection and publishing with the
owner's login. The approved SPEC-004 publisher needed three GitHub Apps,
a test repository and owner approval comments, all owner actions. The
owner chose a simpler route, wants frequent pushes after each milestone,
and does not want to perform GitHub steps.

## Decision

"No branches anywhere, all work on the Dell, agents commit and push to a
local repository first, and main flows to GitHub from there."

```text
agent clone (Dell, agent account, no GitHub credentials)
  commit       -> pre-commit hook: fast gate
  push main    -> pre-receive hook: FULL gate in a clean copy;
                  rejected if red; nothing leaves the agent's clone
local bare repository (Dell, publisher account, holds the deploy key)
  post-receive -> push main to GitHub (or `just publish`)
GitHub main (only the deploy key may update it; no other branch)
  CI re-runs the gate; red -> revert through the local gate + P0
```

1. A bare repository on Dell is owned by a dedicated operating-system
   account, `sutradhar-publisher`, which alone holds the GitHub deploy key
   with write access. Agents run as a different account, clone from the
   local repository over local SSH and have no GitHub credentials.
2. The repository's pre-receive hook runs the complete gate (`just
   check`) on the pushed commits in a fresh, clean copy and rejects the
   push with the full report if anything fails. It runs on the receiving
   side, so `--no-verify` cannot skip it, and agents cannot edit it
   because they do not own the repository. Pushes must be fast-forward
   and to `main` only; any other reference is refused, so no branch can
   ever be created locally.
3. Every agent clone installs pre-commit and pre-push hooks running the
   fast part of the gate, so failures surface early. The pre-receive hook
   never trusts them and always runs everything.
4. A post-receive hook pushes `main` to GitHub immediately with the deploy
   key, with retry and backoff on network failure and an alert if it
   stays behind. `just publish` does the same push on demand; automatic
   is the default.
5. GitHub rulesets: `main` accepts updates only from the deploy key, with
   force pushes and deletions blocked and linear history required; a
   second ruleset blocks creating any other branch. Nobody has bypass,
   the owner included.
6. CI on GitHub re-runs the gate on `main` as a safety net. If `main`
   turns red, the publisher commits a revert through the local gate,
   pushes it and files a P0 AutoIssue; the agent fixes forward.
7. Outside contributors may open pull requests from their own forks; the
   publisher imports an accepted one as a single commit through the local
   gate and closes the pull request. No branch is created here.

The owner's other answers settle the rest. First round: agents do the
GitHub and Dell setup (R113); agents review contributors' pull requests
(R114); one commit per step, pushed after each milestone (R115).
Follow-up round: the editing computer's GitHub sign-in is used once for
setup and then reduced to read-only (R113); the owner approves changes
to the gate itself by answering a question in the session (R116); files
are edited on the Windows computer and every commit, test and push
happens on Dell (R117); the linking specifications are drafted now
(R118).

## Recorder's notes

These are the coordinator's choices for carrying out the decision, not
owner decisions; the
[publication update](../requirements/updates-2026-10-30/publication.md)
gives the reasons.

- "Fresh, clean worktree" is met by the runner's exact export, because
  R083 forbids extra Git worktrees.
- `just check` is the runner's commit profile for each new commit
  against its parent, with the commit message as the test-first
  declaration; `just fast` is the fast part; agents reach the bare
  repository through an SSH key limited to Git, with no shell.
- GitHub implements "only the deploy key" by making deploy keys the
  rulesets' sole bypass entry; no person is on it.
- CI runs every part of the gate that does not need the Dell sandbox.
- Imported contributions keep the contributor's credit in the commit.
- Code, performance, security and AI reviews stay required; their signed
  review records are evidence the gate checks before it accepts a push.
- Until AutoIssues exists, a red `main` is a P0 in the private ledger,
  reported to the owner; alerts go to the same place.
- One coordinator still moves shared Git state (R085): it is the only
  agent account that pushes to the bare repository.

## Consequences

- Replaced parts of SPEC-004: the three GitHub Apps, the publication
  inbox, the attester's statuses, the approval verifier with owner
  approval comments, and turning pull requests off. Kept: one route to
  `main`, fast-forward only, no force pushes or deletions, exact candidate
  identification, the runner's signed evidence, independent reviews and
  receipts for every update.
- The gate runs before anything leaves Dell; GitHub's own checks follow
  the push, so the revert-and-fix-forward path is the recovery route.
- Still open, not decided here: how commits made before the route
  existed enter the bare repository, since they predate the test-first
  check (put to the owner under R116); `just` and the Git hooks need an
  admission record; closing and commenting on contributors' pull requests
  needs a GitHub identity with pull-request access, chosen when that part
  is built and approved under R116.
