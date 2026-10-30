# Releases and manual versions

Audience: maintainers and release agents. Outcome: publish only the exact
candidate that completed every required release condition.

Status: release contract. Release Please, judge orchestration, packaging and
manual publication require implementation and qualification. No release is
claimed here.

## Release authority

Ordinary trusted work may merge unattended after all required checks. The
owner's approval remains necessary for feature specifications and changes to
governing, privacy, checking and release rules. Coding agents do not create
release tags. Only the restricted release publisher can create candidate and
final tags.

A weekly schedule prepares or updates a Release Please proposal. It is not a
promise to publish an incomplete product every week. The complete first release
remains blocked until all agreed scope and at least 6,000 qualified distinct
methods are complete. Development checkpoints may continue while this is
pending.

## Candidate to final release

1. Prepare the version and changelog through the Release Please proposal branch.
2. Run every applicable merge check, including the protected decision checks.
3. Allow its eligible PR to squash-merge when the required checks are complete.
4. Freeze the exact proposed release content and create a candidate tag such as
   `v1.0.0-rc.1` through the restricted publisher.
5. Start durable release qualification against that content, including both
   judge sets, all required group thresholds, installation evidence and
   the 72-hour soak.
6. Keep any GitHub release record a draft while a required result is missing,
   failed, pending, stale or inapplicable without an approved reason.
7. After all conditions pass, create the final tag, such as `v1.0.0`, for
   the same qualified code and publish the matching verified package and
   release notes.
8. Publish the frozen full-version manual and move `latest` to it.

Prepare final version metadata before freezing the candidate so promotion does
not quietly modify the code. If a necessary change occurs, create a new
candidate and rerun all affected checks. Verify artifact hashes and the tagged
commit again at publication. Never retag an existing version to disguise a
changed candidate.

Default Release Please behaviour includes tag and GitHub-release creation after
its release PR merges. The Sutradhar integration must hold back that default
final publication path and prove the candidate-to-final sequence with tests.
Do not assume a configuration flag or integration exists until its current
supported behaviour has been verified.
[Release Please documentation](https://github.com/googleapis/release-please)

## Long-running checks

Release judging resumes from durable checkpoints across subscription usage
windows. It uses at least 500 fixed and 500 fresh suggestions, assessed
separately, with the approved two-provider consensus protocol. Nightly checks
do not invoke judges by default. Cached verdicts are reusable only when their
full identity and qualification remain valid. See
[the gate contract](gates.md#release-suggestion-assessment).

The soak runs under a supervisor on Dell and remains observable after the
editing machine or chat closes. Preserve incomplete and failed runs. Neither
an elapsed timer nor a directory named "passed" proves a live process
completed its checks. Resume only genuine resumable work; a stopped
uninterrupted soak must start its required duration again.

All development pauses if Dell is unavailable. A provider outage or quota limit
leaves required judging pending. No automatic paid fallback, reduced sample,
lowered confidence threshold or missing-review waiver is permitted.

## Manual versions

The public site is hosted at
[the manual URL](https://thulaen.github.io/sutradhar/),
using the generated `gh-pages` branch. It is not hosted on the owner's machine.
The [repository setup runbook](repository-setup.md#manual-publication)
contains the GitHub Pages clicks.

- Every push to `main` publishes the checked `dev` manual, with a target of five
  minutes while required infrastructure is available.
- Every pull request produces a downloadable checked site and a clearly labelled
  preview. Preview publication uses trusted code and synthetic content only.
- Use a safely encoded, collision-resistant branch identifier beneath the
  requested preview route. Do not insert raw branch text into shell commands
  or file paths.
- Delete a preview when its pull request closes. This does not delete its source
  branch or qualified release manual.
- Each final full version, including patch versions, has a separate frozen
  manual. The version switcher shows those releases and `dev`.
- An optional major/minor shortcut may point to the latest qualified patch,
  while full-version manuals remain unchanged. `latest` is the normal opening
  version after the first complete release exists.
- Before any qualified release exists, the site must show its development
  status; it cannot invent a `latest` production release.

Mike writes commits when it deploys documentation versions. Its route must use
the restricted manual publisher identity. Use redirect or copy aliases
compatible with the chosen branch-based hosting configuration, not unqualified
symlink assumptions. [Mike documentation](https://github.com/jimporter/mike),
[GitHub Pages publishing sources](https://docs.github.com/en/pages/getting-started-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site)

The publisher builds and deploys accepted source; it does not author new manual
content. Agent-authored manual changes, including the weekly gardener, arrive in
PRs. Preview source is deliberately unmerged and clearly labelled; it cannot
replace `dev` or `latest`.

## Manual acceptance

The full build, screenshots and checks target less than four minutes under the
qualified workload. Run strict MkDocs compilation, link checks, spelling,
executable-example checks, manual-change coverage, token-contrast checks and axe
over every built page. Test keyboard navigation separately; automated results
alone do not prove WCAG 2.2 AA.

Use synthetic screenshot fixtures in light and dark themes. Never publish
private posts, names, domain metadata, hidden attachment contents or raw lab
results. Reviewed anonymous aggregate metrics are the only permitted real-data
statistics. Generated references must match the current implemented settings,
commands and error catalogue; absent features receive truthful
"Not yet available" notices.

The beginner installation path is point-and-click, with necessary server
preparation handled by an agent or authorised host administrator. Copy-and-paste
commands belong in optional advanced reference. Test the guided path from a
clean qualified environment to the first approved link before declaring it
complete.

## Failure and rollback

If a merged change breaks `main`, open a revert PR immediately. That revert
still passes the applicable checks; do not bypass protection. A rejected
release candidate stays unpublished, with its evidence preserved and the
reason visible.

Test package upgrade and rollback through approved XenForo mechanisms. Pausing
affected features during an upgrade, backup handling and allowed owner overrides
follow the approved product specification. A manual alias rollback cannot stand
in for a tested application rollback.
