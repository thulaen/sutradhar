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

How Forgejo refuses a failing push is in the
[door specification](../specs/0001-door.md). The rules are in
[AGENTS.md sections 8 and 9](../../AGENTS.md#8-how-a-session-works).

## Agent accounts and credentials

Decided by the owner on 22 November 2026 (DR-161): each agent pushes as
its own Forgejo user, never as the owner, and the first account is
`agent-claude`, which is not the runner account of spec 0060. It is an
ordinary user, not an administrator, with write access to
`sutradhar-owner/sutradhar` and a token limited to that repository,
kept in a git credential helper outside the repository. The
door gate still checks every push, and AGENTS.md section 9 is unchanged.

The details below are agent proposals until the owner answers QR-254:

- Two agents never share an account or a token.
- Token scopes are `write:repository` and `write:issue` (tasks, labels
  and proof comments, AGENTS.md section 8) and no other. It has no
  access to the release key and cannot sign a release. The owner also
  protects `v*` tags in Forgejo so the account cannot create one.
- The token expires within 90 days (proposed). Rotating it is an owner
  task in Forgejo; an expired token stops every push until it is
  rotated.
- The agent harness of task #166 holds the token for a managed agent,
  which still holds no token itself (AGENTS.md section 11). A session
  under the shared account that has sudo can read any token on Dell, so
  it is never described as contained.
- The helper is an encrypted keyring readable by one operating-system
  user. The plain-text `store` helper is not allowed.
- The token is never written to a file in the repository, a remote
  address, a commit message, a review record, a log or the product
  vault, and an agent never prints it. The gitleaks check on commits and
  pushes (AGENTS.md section 9) is the check behind this.
- The owner sets Forgejo protection on `main`: this account on the push
  allowlist and force-push off. The account cannot change repository
  settings. The door refuses every ref other than `main` and release tags.
- If the token leaks, the owner revokes it, checks the push history and
  issues a new one.
- A push that fails the push tier is refused at the door whether or not
  the local hooks ran. Agents never use `--no-verify`. The local
  pre-push hook is not a substitute for the door, so no token is issued
  to an agent until the owner records in Forgejo that the door is
  installed. That record is owner-trusted; nothing checks it by
  machine.

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
