# 0006: Main-only publication and GitHub settings

`main` is the only branch, in Forgejo and on GitHub. Forgejo's door
decides what lands on Forgejo's `main` (0001). The sync, `forge-sync`,
is then the only thing that moves GitHub's `main`, only by fast-forward,
with a full record and safe recovery. This part also lists the GitHub
settings that make this hold. It is roadmap Phase 1 items 3, 4 and 9,
and follows [AGENTS.md section 9](../../AGENTS.md#9-git-and-github).

## What it does

### On Forgejo

1. Only `main` and release tags are accepted. Other branches, force
   pushes and deletes are refused by the door (0001).
2. The door runs the push tier on every push of `main`. The push tier
   refuses a commit without all six passing review records in
   `reviews/` ([AGENTS.md section 7.1](../../AGENTS.md#71-the-six-reviews)).
3. A protected change (AGENTS.md section 7.2) is accepted only when it
   is signed with the owner's key. The signed statement names the
   candidate tree, the commit identifiers and a SHA-256 digest of the
   protected files' hashes. Approval of one candidate never approves
   another. Agents never make or alter such a signature.

### The sync's steps for `main`

After each push accepted on Forgejo, the sync:

1. Takes the new commits from the door's accepted copy and checks that
   they form a straight line from the base, the GitHub `main` it last
   wrote. Each commit must carry the door's acceptance record.
2. Reads live GitHub `main`. If it is not the base, it refuses the push
   as stale or unexpected and raises an urgent task.
3. Applies the allow-list and the gitleaks scan of AGENTS.md section
   9.1. If it is unsure, it sends nothing and raises an urgent task.
4. Uploads the commits in order and checks that each identifier GitHub
   returns equals the local one.
5. Final check, repeated on every attempt, recovery included: within 10
   minutes before the update, it reads `main` again and reads the active
   GitHub rules. If the rules differ from the approved settings below,
   it stops and raises an urgent task.
6. Moves `main` with a non-forced update, so GitHub refuses anything
   but a fast-forward.
7. Reads `main` and the tree back, writes an unchangeable publication
   receipt, and adds the gate's latest journal head to an anchor log
   that the gate may add to but never rewrite.

Tags and releases made by the release step (0005) are sent the same
way, after the commit they name is on GitHub. The sync never makes a
branch, never forces an update and offers no general GitHub access.

### Recovery

A durable journal records each step. After a crash, the sync reads
GitHub:

- if `main` already equals the new tip, it checks it and writes the
  receipt;
- if `main` equals the base, it resumes at step 5, never later;
- anything else stops for a person to look at, with an urgent task.

A retry never moves `main` twice. A broken `main` is mended by a revert
commit through the same door and sync, with the same checks.

### The early commits

The first commits came before the gate's checks existed. For those, the
door's copy holds the bootstrap record of AGENTS.md section 7.3, and the
sync checks it before upload. A check is required only from the commit
that switches it on; before then nothing is reported under its name.

### GitHub CI

GitHub CI re-runs the full gate on every push from the sync, as a second
wall. A failure there is a broken rule and becomes an urgent task. The
rules of AGENTS.md section 9.3 hold: no code from pull requests, no
`pull_request_target` or `workflow_run`, no event text in a `run:` step,
read-only permissions by default, actions pinned by commit hash with
`persist-credentials: false`.

## GitHub settings

Agents do not change GitHub security settings. The owner makes these
changes on the GitHub website.

### Rulesets

1. "Sutradhar protected main": blocks deletion and force pushes and
   requires linear history. Its bypass list is empty.
2. "Sutradhar main publisher": targets the default branch with "Restrict
   updates" and "Restrict creations". Its bypass list holds the sync's
   identity only, in "always" mode so each use is logged. Rules in one
   ruleset are all bypassed by that ruleset's bypass list, so the
   integrity rules live in ruleset 1, which binds the sync too.
3. "Sutradhar no other branches": targets every branch but `main` with
   "Restrict creations" and "Restrict updates", and an empty bypass
   list.
4. Tag rulesets block tag updates and deletion. Only the sync may create
   tags, and only release tags already made in Forgejo.

### Repository settings

1. Pull requests stay open so outside contributions can arrive, but
   none is ever merged on GitHub. The sync turns each into a Forgejo
   task in "Later" (AGENTS.md section 9.2).
2. Squash merging stays allowed, since GitHub's linear-history rule
   needs it. Auto-merge is off.
3. Dependency alerts stay on. Dependency security update pull requests
   and version updates stay off; the weekly Dell task does that work
   (0005).
4. Actions workflow permissions are read-only by default.

### Credentials

1. The sync uses a fine-grained GitHub token limited to this repository,
   kept in the sync's own account on Dell. Until the owner makes it,
   only agents on Dell use the owner's signed-in `gh` tool, never runner
   jobs.
2. The GitHub credential on the owner's editing computer is read-only.
   It never needs to push.
3. Keys are moved straight to their own protected place on Dell and
   removed from the computer that downloaded them. Old unused keys are
   retired.

### Limits that cannot be removed

A repository administrator, the owner included, can always edit or
delete rulesets. The design spots this rather than preventing it: the
sync reads the active rules before each push and refuses when they
differ from the approved settings. GitHub shows bypass lists only to
those who may edit the rules, so the owner checks the bypass lists in a
quarterly review.

## Limits

- The final check must fall within 10 minutes before the update.
- Sync and door work run within the Dell budget of AGENTS.md section 11.
- A slow check is never cut short (0001).

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Rust integration tests in `crates/forge-sync`, against scratch Forgejo
repositories and a fake GitHub server:

- Commits not in a line from the base, or lacking the door's acceptance
  record, are refused.
- A base older than live GitHub `main` is refused as stale.
- A change on GitHub between the check and the update stops the update.
- An uploaded commit with a different identifier stops publication.
- The receipt matches GitHub's `main` and tree.
- Requests for branches, forced updates or other endpoints are refused
  without showing a credential.
- A crash after each journal step recovers with no double update and
  always repeats the final check.
- A revert goes through the same route and checks.
- A rule changed after step 3 stops the update at step 5.
- A protected change with a signature from another key, over another
  tree, or naming a different digest, is refused by the door.
- An early commit without its bootstrap record is refused; a check is
  not required before the commit that switches it on.

On a temporary public test repository owned by the owner, with the same
rulesets, since GitHub has no dry run for public repositories on this
plan:

- The sync is refused for a force update and a non-linear commit.
- The owner's own `git push` to `main` and to a new branch is refused.
- An update touching workflow files without the workflow right is
  refused.
- Crash and recovery at each journal step.

## Changes from the source

- Forgejo is the home and its door decides; the old protected publisher
  on Dell becomes the door plus the sync.
- Three GitHub Apps are replaced by one fine-grained token for the sync.
- GitHub required checks before the update are replaced by the door's
  push tier; GitHub CI re-runs the gate afterwards as a second wall.
- Owner approval by GitHub comment is replaced by the owner's key
  signature.
- The hosted AI review is replaced by the six reviews.
- The sync may send release tags and releases, as AGENTS.md section 9.1
  asks.
- Pull requests stay open for outside contributions, which are never
  merged on GitHub.
- The fallback of a private candidate reference on GitHub is dropped,
  since no check runs on GitHub before the update.
- The agent account's lack of administrator rights is dropped, since
  AGENTS.md section 11 gives agents sudo (see 0008).

Source: `docs/specifications/main-only-publication.md` and
`docs/specifications/main-only-github-settings.md` in the archive
`archive-2026-11-08-old-repo`.
