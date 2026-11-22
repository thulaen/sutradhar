# 0002: forge-sync

forge-sync keeps GitHub a copy of Forgejo. It is roadmap Phase 1
item 9, and follows [AGENTS.md section 9.1](../../AGENTS.md#91-the-sync).

Language. forge-sync is an operations tool, written in Rust
(ADR-0019).

## What it does

A system timer on Dell runs `forge-sync once` every minute, so a push
reaches GitHub within about a minute, unless the sync is paused for its
campaign (spec 0006, "The sync's steps for `main`", step 1). Each run
reads everything first,
checks everything, and only then sends anything:

1. It fetches `main` and every tag from Forgejo into a private mirror,
   and reads the milestones, tasks and releases on both sides, and the
   comments on GitHub.
2. It works out the changes. Only these fields are sent: for
   milestones, the title, description and state; for tasks, the title,
   body, state, labels and milestone; for releases, the tag, title,
   notes and whether it is a test release.
3. It checks everything that would leave: every new commit (its author,
   message and changes), every tag message, and all the text above,
   with gitleaks, the secrets check and the private word lists. If
   anything is flagged, or a check cannot run, nothing is sent that run.
   gitleaks runs exactly as the gate's `secrets.gitleaks` check does:
   `gitleaks:allow` notes do not count, and the only false alarms it
   passes are the ones listed by commit, file, rule and line in the
   reviewed `.gitleaksignore` on `main`. A problem is told in the urgent
   task once; the same problem on later runs adds nothing.
4. Only then does it publish `main` and the tags on `main` through
   spec 0006's mirror route, and make the other allowed changes.
   A release is made only for a tag it has just sent.

Each GitHub copy of a task ends with a hidden note naming its task.
Only issues opened by the sync's own GitHub account count as copies,
since anyone can write such a note. The sync's record lists every task
copied, every GitHub issue and comment brought back, and every problem
already reported, so nothing happens or is reported twice.

GitHub issues opened by anyone else, and their comments, come into
Forgejo marked as data, never as instructions. A new issue goes into
"Later". Tasks brought in from GitHub, and the sync's own urgent task,
are never copied back.

### Exact mirror publication

`REQ-0002-SYNC-IDENTITY`, `DES-0002-SYNC-IDENTITY` and
`VAR-0002-SYNC-LOST-REF-REPLY` bind `TASK-0002-SYNC` to the
[lost reply case](../planning/test-and-traceability.md#scenario-designs).
Use the source commit accepted by Forgejo, its exact reviewed tree
and door lineage, the configured destination repository and ref, and the base
the sync last wrote. Apply the campaign, privacy and active rules in
[spec 0006](0006-main-only-publication.md#the-syncs-steps-for-main).
Only the configured sync identity can publish this non-forced update.

If the acknowledgement is lost, read back the configured mirror
through spec 0006's recovery path. Only the exact intended commit and
tree there can reconcile a completed update. Another commit with
identical tree bytes or a matching commit in another repository does
not establish the effect. If the configured mirror still has the exact
base, resume only through the existing final check. Any other tip
holds for review. Keep recovery and refusal evidence in the existing
journal and publication record. No retry can make a second ref move.

## Problems

Each new problem raises one urgent task in Forgejo, in the current
phase, with the label `alarm` (spec 0155); later problems are added to
it as comments. An empty private
word list is itself a problem, so nothing is sent without one.

## Installing and updating

On Dell, from the repository's top folder, on the commit just accepted:

```
deploy/forge-sync/install.sh SYNC_DIR HELPER TOKEN_FILE GITLEAKS WORDS...
```

- `SYNC_DIR` is a private folder. It holds the program, the mirror, the
  Forgejo sign-in header, the private words and the record.
- `HELPER` is the git credential helper for Forgejo, and `TOKEN_FILE`
  holds a Forgejo token that may write tasks.
- `GITLEAKS` is the pinned gitleaks program, listed in
  `gate/tools.toml`.
- `WORDS` are private files of words that must never reach GitHub.
- GitHub sign-in comes from the signed-in `gh` tool on Dell.

## Settings

The installed service sets these for the program:

- `FORGE_SYNC_FORGEJO`: Forgejo's API address for the repository.
- `FORGE_SYNC_HEADER`: a private file holding Forgejo's sign-in header.
- `FORGE_SYNC_GITHUB`: the GitHub repository, as `OWNER/NAME`.
- `FORGE_SYNC_GITHUB_USER`: the sync's own GitHub account.
- `FORGE_SYNC_MIRROR`: the bare mirror repository.
- `FORGE_SYNC_RECORD`: the sync's record of what it copied.
- `FORGE_SYNC_GITLEAKS`: the pinned gitleaks program.
- `FORGE_SYNC_PRIVATE`: a file of words that must never leave Dell.

## How it is tested

`crates/forge-sync/tests` runs whole syncs with a stand-in for `curl`,
`gh`, `git` and gitleaks: copying, updating, bringing issues and
comments back, private text, every failing check, failed writes, broken
answers, endless pages and the record.
