# ADR-0008: Forgejo, GitHub and releases

Decided by the owner on 8 November 2026.

## Decision

- Forgejo on Dell is the source of truth for code, tasks, milestones and
  releases.
- Trunk-based development: every agent works on `main`, and there are
  no other branches. Forgejo's door gate runs the push tier on every
  push and refuses any push that fails.
- `forge-sync` copies to GitHub one way, sending only
  allowed fields, and brings GitHub issues, comments and pull requests
  back into Forgejo.
- SemVer versions, Conventional Commits, and release notes made from
  commits. Contributors sign off with the DCO.
- Agents may commit, push and release on their own. The release key is
  kept encrypted on Dell, and every use is logged.

The rules are in
[AGENTS.md sections 8 and 9](../../AGENTS.md#8-how-a-session-works).

## Why

- With one source of truth, there is no doubt about which list is right.
- One branch is easy to follow: what is on `main` is the project.
- Checking every push at the door, then syncing straight away, stops
  work from piling up.
- GitHub CI re-running the gate gives a second, independent wall.

## Trade-offs

- Agents must keep each change small and pull in others' work often, or
  their pushes clash. File locks reduce this.
- GitHub is the only off-Dell copy of the repository. Private items left
  out of the sync exist only on Dell.
- Agents have sudo, so the Forgejo lock and the release key rely on
  agents keeping the rules. The checksum guard, the release log and
  GitHub CI can spot a breach, but cannot prevent one.

## How we check it

- A failing push is refused, even when pushed with hooks skipped.
- A push to any branch other than `main` is refused.
- A test task, milestone and release appear on GitHub with private text
  removed.
- A changed Forgejo hook raises an urgent task in Forgejo and an alert
  issue on GitHub.
