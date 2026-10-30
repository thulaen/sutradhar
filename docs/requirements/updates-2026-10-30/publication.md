# Requirements update: publication and pace

Owner: project owner. Recorded by the Claude coordinator from the owner's
answers of 30 October 2026, given through the question form in two
rounds. The owner's exact words are in the
[publication decisions](../sources/publication-decisions-2026-10-30.md).
Status: owner requirement decisions. Recorder's notes, marked as such,
are the coordinator's reading, not owner decisions.

## R112: publication through a gated repository on Dell

"No branches anywhere, all work on the Dell, agents commit and push to a
local repository first, and main flows to GitHub from there." Agents
commit in Dell clones and push `main` to a local bare repository owned
by a separate publisher account (`sutradhar-publisher`), which alone
holds the GitHub deploy key; agents hold no GitHub credentials. Its
pre-receive hook runs the complete gate (`just check`) on the pushed
commits "in a fresh, clean worktree" and rejects the push with the full
report if anything fails; pushes must be fast-forward and to `main`
only. Agent clones run the fast gate in pre-commit and pre-push hooks,
which the pre-receive hook never trusts. A post-receive hook pushes
`main` to GitHub at once with retry, backoff and an alert if GitHub stays
behind; `just publish` does the same on demand. GitHub: `main` accepts
updates only from the deploy key, force pushes and deletions blocked,
linear history required, branch creation blocked, "Nobody has bypass,
including me". CI on GitHub re-runs "the same gate" on `main`; if `main`
turns red, the publisher commits a revert through the local gate, pushes
it and files a P0 AutoIssue, and the agent fixes forward. "Build this
before any feature work; until it works, commits stay in agent clones."
The design is [ADR-0005](../../adr/0005-local-publication.md).

## R113: agents carry out setup

"i dont want to do github things, i am a noob, don't ask me to do
things, you should do them". Agents make the GitHub and Dell changes the
approved designs need, instead of giving the owner clicks to follow (the
owner's earlier "Give me the exact clicks" is replaced by this answer);
agents still record every setting they apply. Follow-up: the editing
computer's GitHub sign-in, which has administrator rights, is used for
the one-time setup only and then reduced to read-only. Publishing uses
the deploy key alone, never the owner's login. Recorder's note: a step
only the owner can complete, such as signing in to an account, stays
listed as an external dependency.

## R114: outside contributions

"agents stay on master or main branch always, no branches, but
contributors or other people they make pull requests that my agents can
review and then merge." Contributors open pull requests from their own
forks; agents review them; the publisher imports an accepted one as a
single commit through the local gate and closes the pull request.
Recorder's note: "merge" is read as that import, which creates no branch
here, and the contributor's code runs only inside the gate's sandbox.

## R115: pace and commit size

"do everything in full now and then work towards the full working
version 1 with frequent commits and pushes after each milestone", with
"One per step" as the commit size. Recorder's note: a step is one item of
the build sequence (for example the test-first gate, the publication
route, each AutoIssues milestone), so each finished step is one commit,
pushed straight away, rather than one large batch.

## R116: owner approval of changes to the gate itself

Changes to the safety machinery itself (the gate's rules, the hooks, the
runner, GitHub settings) are approved by the owner answering a question
in the session before the change is pushed. This replaces approval by
GitHub comment for those changes. Recorder's note: the other approvals
AGENTS.md requires (feature specifications, protected requirements,
privacy and release rules) are unchanged and are also asked through the
question form, as this update was.

## R117: where work happens

Editing happens on the Windows computer; files are copied into the Dell
agent clone, and every commit, test and push happens on Dell. Nothing
runs on Windows (R072 unchanged).

## R118: linking specifications now

The linking feature specifications for R098–R111 are drafted right
away, before the publication route is built, for the owner's approval.
Recorder's note: drafting specifications is design work, not the
feature work that R112 says must wait for the route.

## Recorder's notes on carrying out R112

- "Worktree": R083 forbids extra Git worktrees, so the gate uses the
  runner's exact export of the pushed tree, which is a fresh, clean copy
  without creating a Git worktree.
- "Nobody has bypass": GitHub can let only the deploy key update `main`
  by making deploy keys the rulesets' sole bypass entry; no person, the
  owner included, is on it.
- "The same gate": GitHub's hosts cannot run the Dell sandbox, so CI runs
  every part of the gate that does not need Dell; Dell remains the gate
  that admits commits.
- Until AutoIssues exists, a red `main` is recorded as a P0 in the
  private ledger and reported to the owner.

## Open, for the owner

- Commits made before the route existed (from the foundation onwards)
  predate the test-first check, and some carry no declaration, so they
  cannot pass every part of the new gate. How they enter the bare
  repository changes the gate's rules, so it is put to the owner under
  R116 when the route is built.
- Until the Dell agent clone exists, commits are made in a Dell clone
  prepared for that purpose and copied back to the Windows checkout.

## Replaced or changed

- R084 (protected publisher with separate owner-approval and
  check-reporting authority, GitHub App identities): the mechanism is
  replaced by R112 and R116. Independent reviews, exact candidate
  identification, fresh checks, refusal of stale updates, audit evidence
  and the bans on force pushes, disabling protection and publishing with
  the owner's login remain.
- R083 (no pull requests): agents still create none; contributors' pull
  requests from forks are accepted under R114.
- R070's remaining owner approval of outside contributions: replaced by
  the agents' review of R114 (recorder's note: a contribution's code first
  runs only inside the gate's sandbox). Owner approval of changes to the
  gate itself continues under R116.
- R086 (small coherent commits, prompt publication): one commit per step,
  pushed after each milestone (R115), through R112.
- The approval record's answer 3: the GitHub-comment approval route is
  replaced by R116; the editing computer's GitHub access becomes
  read-only after setup (R113). Passwordless administrator access on Dell
  is unchanged.
- [SPEC-004](../../specifications/main-only-publication.md) and its
  [GitHub settings](../../specifications/main-only-github-settings.md):
  replaced as listed in ADR-0005. Sections 1 to 6 of the
  [owner actions](../../dev/owner-actions-2026-10-30.md) become agent
  work, adapted to ADR-0005.
