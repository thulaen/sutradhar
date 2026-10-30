# Owner decisions on publication and pace, 30 October 2026

Status: historical source record. The owner asked for these questions
and answered them through the Claude session's question form; the
third round also includes messages the owner typed. The owner's
request:

```text
ask me these questions so we settle them with the decision tool, also i want frequent commits and pushes not one big bang commits and pushes.
```

Answers are kept exactly inside the code blocks. The current requirements
are in the [publication update](../updates-2026-10-30/publication.md) and
[ADR-0005](../../adr/0005-local-publication.md).

## 1. Build order

Asked: what order the remaining build work should follow (the approved
order: test-first gate, performance harness, publisher, AutoIssues 1–5;
the test-first gate half-built). Offered: "Gate, then publisher
(Recommended)", "Publisher immediately", "Keep approved order".

```text
do everything in full now and then work towards the full working version 1 with frequent commits and pushes after each milestone
```

## 2. Interim push

Asked: until the publisher works, how commits should reach GitHub, given
that GitHub accepted `main` only through pull requests and the owner had
ruled out disabling protection, using the owner's login or making
branches. Offered: "Wait for the publisher (Recommended)", "You push them
yourself", "Temporary pull requests". The owner chose "Other":

```text
agent clone (on the Dell)
   │  git commit           → pre-commit hook: fast gate (format, lint, types, fast tests)
   │  git push local main  → pre-receive hook on the local repo: FULL gate in a clean checkout
   ▼                          rejected if red; nothing leaves the agent's clone
local bare repo (on the Dell, owned by a separate "publisher" user, holds the deploy key)
   │  post-receive: push main → GitHub automatically (or `just publish` if you prefer a second step)
   ▼
GitHub main (ruleset: only the deploy key may update it; branch creation blocked for everyone)
   │  CI re-runs the gate as a safety net; red → auto-revert through the local gate + P0 autoissue

Other. No branches anywhere, all work on the Dell, agents commit and push to a local repository first, and main flows to GitHub from there.

1. Create a bare repository on the Dell owned by a dedicated OS user (sutradhar-publisher). That user alone holds the GitHub deploy key with write access. Agents run as a different OS user, clone from the local repository over local SSH, and have no GitHub credentials. Give me the exact clicks for adding the deploy key.
2. The local repository has a pre-receive hook that runs the complete gate (`just check`) on the pushed commits in a fresh, clean worktree and rejects the push if anything fails, returning the full report. This is the hard block: it runs on the receiving side, so --no-verify cannot skip it, and agents cannot edit it because they do not own the repository. Pushes must be fast-forward and to main only; the hook rejects any other ref, so no branch can ever be created locally.
3. Every agent clone installs pre-commit and pre-push hooks that run the fast part of the gate, so failures surface early; the pre-receive hook never trusts them and always runs everything.
4. A post-receive hook pushes main to GitHub immediately with the deploy key, with retry and backoff on network failure and an alert if it stays behind. (If I later want publishing as an explicit second step, `just publish` does the same push on demand; default is automatic.)
5. GitHub rulesets: main accepts updates only from the deploy key, with force pushes and deletions blocked and linear history required; a second ruleset on all other branches blocks creation, so no branch can exist on GitHub either. Nobody has bypass, including me. Give me the exact clicks.
6. CI on GitHub re-runs the same gate on main as a safety net. If main turns red, the publisher commits a revert through the local gate, pushes it, and files a P0 autoissue; the agent fixes forward.
7. Outside contributors may open pull requests from their own forks; the publisher imports an accepted one as a single commit through the local gate and closes the pull request. No branch is created here.
Build this before any feature work; until it works, commits stay in agent clones. Record it as an ADR.
```

## 3. Commit size

Asked: how small commits should be from now on. Offered: "Each passing
slice (Recommended)", "Each part of a step", "One per step".

```text
One per step
```

## 4. Linking specifications

Asked: when to draft the linking specifications (R098–R111), which were
not yet written, for approval. Offered: "While you do GitHub steps
(Recommended)", "Now, before more gate work", "After AutoIssues 1–5".
The owner answered:

```text
i dont want to do github things, i am a noob, don't ask me to do things, you should do them, agents stay on master or main branch always, no branches, but contributors or other people they make pull requests that my agents can review and then merge.
```

## Follow-up questions

An independent review of the first records found points the answers left
open. The owner chose these options in a second round.

5. GitHub sign-in. Asked how to use the editing computer's GitHub
   sign-in, which has administrator rights, given the earlier decision
   that it should be read-only. Offered: "Setup only, then reduce
   (Recommended)", "Setup, keep for upkeep", "Don't use it".

   ```text
   Setup only, then reduce (Recommended)
   ```

6. Gate changes. Asked who approves changes to the safety machinery
   itself (the gate's rules, the hooks, the runner, GitHub settings), which
   used to need a GitHub comment. Offered: "You, via a question here
   (Recommended)", "Independent AI review", "The gate alone".

   ```text
   You, via a question here (Recommended)
   ```

7. Where to edit. Asked where editing should happen, given "all work on
   the Dell" and that Claude runs on the Windows computer. Offered: "Edit
   here, commit on Dell (Recommended)", "Everything on Dell".

   ```text
   Edit here, commit on Dell (Recommended)
   ```

8. Linking specifications. Asked again when to draft them, as the fourth
   answer had not said. Offered: "After publishing works (Recommended)",
   "Right now", "After AutoIssues 1–5".

   ```text
   Right now
   ```

## Third round: where the work happens

After the test-first gate was built, the owner asked about the editing
and copying step. These are the exact words, in order. Some were
answers in the question form and some were typed as messages; the
headings say what was asked where there was a question. "msi" in the
first question is the owner's Windows computer.

### 9. The owner's question

```text
wouldnt it makes sense to just dell for testing and lab, then we commit and push from msi is this what we are doing?
```

### 10. How commits flow

Asked how commits should flow. Offered: "Commit on MSI, push to Dell
(Recommended)", "Keep committing on Dell", "Push straight to GitHub".
The owner chose "Other":

```text
would it be better to move the repo to dell and then ssh from there, then no more copying and everything is just straight to github, commits on dell check everything, push checks everything and then gh checks everything too via ci and failed things because autoissues?
```

### 11. Which connection

Asked whether an existing app connection was the Dell:

```text
setup dell connection via cable
```

### 12. When to move

Asked when to move the work to Dell:

```text
After this commit (Recommended)
```

### 13. Failures

The owner added:

```text
if it fails let me know first before doing anything
```

### 14. Reaching Dell

Asked how to reach Dell from the app:

```text
try Dell Ubuntu
```

After the first attempt to start the Dell session failed:

```text
Try again
```

After the second attempt failed:

```text
solve it make dell ubuntu work
```

### 15. Handing over

Once the Dell connection worked, the owner asked:

```text
give me a prompt in this chat i will create a new session with dell ubuntu and ou will takeover from there?
```
