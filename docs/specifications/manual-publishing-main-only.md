# SPEC-002 revision: manual publication without branches or pull requests

Status: APPROVED replacement for the "Build and publication" section of
[SPEC-002](manual-publishing.md) and its preview acceptance examples.
Everything else in SPEC-002 keeps its approval. Awaiting the owner.
Approved by the owner on 30 October 2026 (see the approval record in
`docs/specifications/approvals-2026-10-30.md`); it supersedes any
"awaiting approval" wording below.

## What changes and why

SPEC-002 builds a preview for every pull request and publishes through a
`gh-pages` branch. Main-only development (R083) removes pull requests and
forbids any second branch, so both mechanisms need replacing. The goals
stay: checked builds, isolated previews, `dev` after each `main` update,
a frozen manual for every full version and `latest` only after release.

## Candidate previews

During the push profile, Dell builds the candidate's manual in an isolated
workspace with only the approved synthetic fixtures and the admitted
metrics export, runs every SPEC-002 check, and stores the built site in a
new immutable snapshot named after the candidate tree. The coordinator can
show the owner pages or screenshots from it on request. Previews never go
to GitHub Pages; there is nothing to remove afterwards and no path a
preview could collide with (AT-MP01).

## Publication route

GitHub Pages is switched to its "GitHub Actions" source, so no branch holds
the site. One workflow, triggered by the push event that the publisher's
`main` update creates, runs on a GitHub-hosted runner with read-only
contents and only the Pages deployment permissions. It:

1. builds `dev` from the published `main` commit with pinned tools;
2. adds every frozen release manual, downloaded from the release asset
   attached by the release identity when that version was released;
3. writes the version switcher data, pointing `latest` to the newest
   qualified release, or showing that no stable release exists yet;
4. uploads the assembled site and deploys it to Pages.

Frozen manuals are never rebuilt: each release attaches its built manual as
a release asset whose hash is recorded on Dell, and the workflow checks the
downloaded hash before including it (AT-MP02). A changed or missing asset
blocks deployment.

The workflow file itself is a protected change. It uses only pinned
official actions. No private data, Dell access or secret is available to
it, because everything it builds is already public source.

## Timing and outages

The five-minute publication target is measured from the `main` update to
the Pages deployment. Deployment pauses when GitHub or a required service is
unavailable and reports the delay (R073).

## Acceptance cases

- AT-MP01: a candidate preview is built and checked on Dell, stored under
  its tree identity and never published.
- AT-MP02: a frozen manual asset whose hash differs from the recorded one
  blocks deployment.
- AT-MP03: a `main` update publishes `dev` without touching any frozen
  version or `latest`.
- AT-MP04: `latest` moves only when the release identity has published a
  qualified release with its manual asset.
- AT-MP05: the workflow cannot write repository contents, create branches
  or read anything private.
- AT-MP06: before the first release, the site states that no stable
  release exists.

## Settings needed

Pages source "GitHub Actions"; the workflow's permissions limited to
reading contents and deploying Pages. These are added to the SPEC-004
settings list when this revision is approved.
