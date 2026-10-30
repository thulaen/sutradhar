# Repository setup

Audience: the repository owner and setup agents. Outcome: configure protected
collaboration through GitHub without asking the owner to use Git by hand.

Status: setup contract and click-by-click runbook. Every setting must be read back
from GitHub and recorded after application. This document is not an installation
receipt. Product source must not be pushed under the owner's identity as a
substitute for the dedicated automation identity.

Repository: [thulaen/sutradhar](https://github.com/thulaen/sutradhar).
Default branch: `main`. Public manual target:
[Sutradhar manual](https://thulaen.github.io/sutradhar/).

## Initial settings receipt

On 30 October 2026, the setup coordinator reported successful API application and
readback of these repository settings:

- Auto-merge enabled; squash enabled; merge commits and rebase merging disabled.
- Branch updating and automatic deletion of merged proposal branches enabled.
- Active `main` ruleset `26172923`, with the nine reserved check contexts.
- Tag immutability ruleset `26172929`, blocking tag updates and deletion.
- Tag creation ruleset `26172930`, blocking creation until the release identity is
  qualified and its narrowly scoped creation authority is configured.
- The additional default approval requirement for unattributed changes disabled,
  preserving unattended ordinary changes while the `owner approval` gate remains.

This is settings-state evidence reported by the coordinating setup task, not an
end-to-end enforcement result. Check producers, App identity, candidate execution,
required AI review and safe automatic publication remain pending. Re-read live
settings before relying on this historical receipt. The explicit `owner approval`
gate still protects owner decisions.

The owner subsequently approved the separate proposal App and its repository
scope. Registration and installation on this repository alone were visibly
confirmed. The private-key download was blocked by the browser; no usable private
key was retrieved or installed on Dell. The App is therefore installed but the
publishing path remains unavailable and unqualified. See the
[handoff](handoff.md) for the remaining credential and publisher work.

## Before changing settings

1. Confirm the repository identity, default branch, existing rulesets and current
   workflow state. Preserve existing unrelated settings and work.
2. Confirm Dell is available. Use its protected coordinator for private checks.
3. Identify the owner's GitHub user ID separately from all automation App IDs.
4. Prepare the exact requested changes and apply available API-supported settings.
5. Record the resulting settings, missing setup actions and acceptance tests.

Do not list a setting as complete merely because an API accepted an update.
Read it back and test the intended refusal where possible. A repository
administrator can change repository rules; the system must not claim to prevent
the owner from ever altering their own account or GitHub administration.

## Automation identity

GitHub Apps can act independently, with repository-scoped permissions and
short-lived installation credentials. [GitHub App documentation](https://docs.github.com/en/apps/creating-github-apps/about-creating-github-apps/about-creating-github-apps)

The owner-only setup steps are:

1. Open **GitHub → profile picture → Settings → Developer settings → GitHub Apps**.
2. Select **New GitHub App** and register the reviewed automation identity.
3. Grant only the reviewed repository permissions and install it on this repository.
4. Keep its private key and installation write tokens in the trusted publisher on
   Dell, never in a coding session, worker checkout, public artifact or transcript.
5. Use separate protected identities for proposal publication, check reporting,
   owner-approval verification and release publication. Give the proposal App only
   reviewed contents and pull-request capabilities, plus workflow-file permission
   only when required for an approved proposal. It receives no repository
   administration or check-reporting permission.

The setup agent may prepare supported registration and installation flows, but
must not claim an owner-only consent screen has been completed until it has.
Record App IDs and credential references privately; public docs need roles only.

Coding workers submit patches or bounded branch/PR operations to the trusted Dell
publisher. They never receive its raw App write token. The interface exposes no
product-release API, arbitrary endpoint forwarding or credential retrieval. It
checks the proposed operation, reservation, approved scope, candidate and evidence
before performing an authorised write.

GitHub contents-write permission also permits release operations. Consequently,
neither a proposal App's permission label nor a tag-creation ruleset alone proves
an agent cannot publish a release. Credentials must remain behind the restricted
interface, and release publication uses a separate protected identity.
[GitHub release API permissions](https://docs.github.com/en/rest/releases/releases#create-a-release)

The publisher and its refusal tests are an unimplemented specification that still
needs approval. App registration alone is not a qualified publishing path. Coding
workers cannot modify rulesets, forge required checks, approve protected decisions,
create release tags or access private production credentials.

## General repository settings

Open **Repository → Settings → General → Pull Requests**:

1. Enable **Allow squash merging**.
2. Disable **Allow merge commits** and **Allow rebase merging**.
3. Enable **Allow auto-merge**.
4. Enable updating pull-request branches if supported by the chosen workflow.
5. Ensure conventional commit titles are checked before the squash merge.

Automatic merging remains conditional on every protected check. It does not mean
unreviewed changes enter `main`. [GitHub auto-merge settings](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/managing-auto-merge-for-pull-requests-in-your-repository)

## Main branch ruleset

Open **Repository → Settings → Rules → Rulesets → New ruleset → New branch ruleset**:

1. Name the rule clearly and target exactly `main`.
2. Set enforcement to **Active**.
3. Leave the bypass list empty, including for administrators and the owner.
4. Require a pull request before merging.
5. Require the branch to be up to date before merging.
6. Require the checks listed in [the gate contract](gates.md#required-github-check-names).
7. Require linear history and restrict the merge method to squash where available.
8. Block force pushes and branch deletion.
9. Save, read the rules back and test an unauthorised direct push in a safe setup
   qualification procedure; do not damage the production branch to test protection.

Do not add a general required human-review count that would block every ordinary
unattended pull request. The separate `owner approval` check requires the owner's
current approval for protected decisions and outside contributions, while
returning an explained non-applicable result for ordinary trusted approved-scope
work. Protect that check's implementation and expected reporting identity.

Do not enable **Restrict updates** on `main` with no permitted actor and assume PR
merges will still work. Choose the rules that enforce PR-only changes without
making all authorised merges impossible. Qualify the actual configuration.
[GitHub ruleset semantics](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/available-rules-for-rulesets)

## Owner-approved paths and decisions

The protected classifier covers `AGENTS.md`, governance, requirements, feature
specifications, privacy, workflow/check definitions, trust configuration, release
configuration and their dependencies. It must examine renames and semantic rule
changes, not merely a fixed filename list that an agent can evade.

Use CODEOWNERS for review routing where useful, but do not assume routing alone
enforces approval. The verifier checks the approving user's immutable GitHub ID,
the current revision and the protected change. Agent comments, copied approvals
and stale reviews cannot satisfy it. A new feature implementation must cite the
approved specification revision even if the implementation PR changes no document.
[GitHub code-owner documentation](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners)

## Required AI review

The owner-controlled connection steps are:

1. Connect the repository to Codex using the official GitHub connection flow.
2. Open **ChatGPT → Settings → Code review**.
3. Select this repository and enable **Automatic review** under **Review code**.
4. Enable the relevant personal automatic-review preference and review trigger.
5. Open a representative test pull request and verify a completed review arrives.

The public instructions are at [Codex GitHub review setup](https://learn.chatgpt.com/docs/third-party/github).
Account entitlement and actual connected settings must be verified; this page does
not assert they are enabled or unlimited.

The protected `AI code review` check confirms completion for the current proposal
and handles findings according to the approved review policy. Do not invent a
native status check just because a review comment is visible. No-response,
quota-exhausted and stale-review outcomes remain blocked. Keep code, performance
and security engineering reviews in addition to this integration.

For a second Claude Code reviewer, first approve its adapter specification and
credential model, qualify its structured result and current-revision binding,
then add its result to the protected review check. Do not put a consumer login
credential inside public or fork execution. This is an optional second reviewer,
not an excuse to waive the mandatory review if the first provider is unavailable.

## Workflow permissions and outside contributors

Open **Repository → Settings → Actions → General → Workflow permissions**:

1. Use read-only permissions by default.
2. Grant job-level writing only to protected publishers, with the narrow required
   permission and no execution of untrusted pull-request content.
3. Keep fork jobs secret-free and read-only.
4. Keep untrusted code off the private runner until the owner approves that exact
   candidate. Use an isolated public runner for its synthetic checks first.

Do not use a privileged `pull_request_target` job to check out and execute an
untrusted head. Safe metadata handling still requires a protected implementation.
Read-only tokens do not independently enforce owner-only merging; enforce the
contributor rule through the `owner approval` check.

The optional trusted-contributor list is protected policy keyed by immutable
identities. There is no assumed native GitHub setting called "trusted contributors"
that provides all these guarantees. Show its approved management interface in the
setup instructions after implementation. Admission requires the owner and cannot
be changed by the very pull request seeking trust.

Default `GITHUB_TOKEN` events have restrictions on triggering later workflows and
Pages builds. Use the qualified App publication route or explicit dispatch where
appropriate, and test the whole event chain. Do not assume a successful API write
means the next workflow ran. [GitHub token behaviour](https://docs.github.com/en/actions/concepts/security/github_token)

## Release and tag protection

Create tag rulesets under **Settings → Rules → Rulesets → New tag ruleset** for the
approved candidate and final version patterns. Only the restricted release
publisher may create those tags. No coding worker may change or delete an existing
candidate or release tag. Keep any narrowly required tag-creation permission
separate from the `main` ruleset's empty bypass list.

A weekly Release Please proposal is automatically eligible only after its checks
pass. Its merge starts candidate preparation; it must not publish a final release
through an unmodified default path. Follow [the release contract](releasing.md).
The final tag points to the qualified candidate code; changed code needs a new
candidate and applicable evidence.

## Manual publication

Open **Repository → Settings → Pages → Build and deployment**:

1. Select **Deploy from a branch**.
2. Select `gh-pages` and `/(root)` once the approved publisher has created them.
3. Save and verify the public URL and its version routing.

Only the protected manual publisher writes the generated branch. Source changes
come through pull requests. PR previews contain the checked candidate manual;
they are clearly labelled previews and removed when the PR closes. Release manuals
remain frozen per full version. The `latest` alias moves only after release
qualification. See [the release contract](releasing.md#manual-versions).

## Private vulnerability reporting

Open **Repository → Settings → Security → Advanced Security** and enable private
vulnerability reporting where GitHub exposes that setting. Interface wording may
vary; verify the resulting **Security → Advisories → Report a vulnerability**
action. Do not publish a dummy report containing real sensitive information.

## Acceptance record

Record which of the following is configured, which has been tested and which is
pending. Keep credential details and private host information outside the repo.

- A direct or force push and branch deletion cannot bypass the `main` rules.
- Ordinary trusted PRs auto-merge only after every current check passes.
- Protected changes require the real owner's current approval.
- An untrusted fork cannot obtain secrets, private data, Dell execution or auto-merge.
- Forged, skipped, stale and absent checks do not pass.
- Coding sessions cannot obtain App write tokens or call release operations through
  the proposal publisher, including releases for already-existing tags.
- A newly pushed revision invalidates the necessary old approvals and results.
- A broken `main` produces a revert PR; a rules bypass is not used as a repair.
- Candidate, judge, final publication and Pages events actually run in sequence.
- Dell loss pauses all project work and records the paused deadlines.

This list remains unverified until results for the configured repository exist.
