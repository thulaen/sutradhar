# SPEC-002: public user manual and publishing

Status: APPROVED for bounded implementation. See [approval record](approvals.md).

Audience: owner, documentation writers and implementation agents.
Outcome: a beginner can understand the product and, once implemented, install
and use it from the manual without reading source code or using a terminal.

Requirements: R047–R051, R072–R075 and R078–R080. The full Q192 brief remains
authoritative subject to Q157, Q195, Q203 and Q207.

## Content and source boundaries

Keep public user content under manual and developer material under docs/dev.
Do not publish architecture, internal implementation or module-development
instructions into the manual. Every page identifies its audience and the
observable outcome. Where one sentence of background is necessary to make a
user decision, provide that sentence without turning it into developer material.

Keep the requested navigation: Home; Getting started; Using Sutradhar; Modules;
Configuration; Operations; Troubleshooting; Reference. Each module uses the
same user-oriented layout. Commands are advanced reference, not the primary
installation path. The guided path accommodates authorised host preparation and
ends in an understandable health result.

The manual describes available behaviour. Until installation, review, dashboards
or another feature works, its page uses a clear Not yet available banner and
states the present limitation. Never show planned functionality as operational,
invent live screenshots or imply that the complete installation trial has
passed. Automatic link insertion is unavailable in v1.

## Generation and design

Use MkDocs Material, Mike, mkdocs-click,
mkdocs-git-revision-date-localized-plugin, mkdocs-llmstxt, codespell,
Playwright and axe, with pinned reviewed dependencies. Do not add an API reference
or mkdocstrings to this user manual.

Generate settings from the approved Pydantic settings models, commands from
the Typer command structure and user errors from a central SUT-NNN catalogue.
Each generated item explains when a user needs it, its defaults or expected
result and the action to take. Error messages include their code and manual
link. Generate release notes from the maintained changelog.

Capture real implemented UI using synthetic fixture data in light and dark
themes. Never open private datasets to create manual screenshots. Before the
screens exist, report the product gap instead of substituting a mock-up presented
as a capture. Generation imports only admitted safe definitions and must not
start production services or access live forum credentials.

Apply the requested palette research and BBC/GOV.UK patterns through CSS tokens
and minimal template overrides. Record palette sources and design decisions in
developer docs. Use system fonts, accessible focus states, underlined links,
readable line lengths, the requested type/spacing rules and minimum touch
targets. Verify both themes and all supported interactive elements.

## Build and publication

Every PR builds the site and runs the applicable checks in isolation. Upload a
scrubbed build artifact. A protected publishing job writes a preview under a
safe unique branch-derived path, preventing path collisions or overwriting dev,
latest or a release version. Remove that preview after the PR closes.

Every merged main update builds and publishes dev. Store a frozen manual for each
full approved version, including patch numbers. Candidate tags may produce a
clearly labelled candidate preview but cannot advance latest. After all release
gates pass, publish the matching full release version and update latest. Retain
older frozen manuals and the version switcher. Before the first release, make
the absence of a stable release explicit rather than creating a fake latest.

Use the gh-pages branch as the Pages publishing source, with generated content
and appropriate aliases compatible with branch-based Pages deployment. The
protected publisher owns writes; the build job has no release or repository
administration credentials. No GitHub Pages execution must depend on a sleeping
editing computer. Publication pauses when the validation host or a required
service is unavailable and reports the delay.

Target a full build including screenshots within four minutes and a main update
published within five minutes under qualified available-service conditions.
Measure these targets as the manual grows. A missed target is reported and
corrected rather than hidden by skipping required checks.

## Checks and scheduled care

Run strict MkDocs builds, internal/external link checking, codespell, token-pair
contrast checks, per-page axe checks and keyboard navigation coverage. Execute
real Python examples with mktestdocs and enforce the requested bash-block style.
Historic quoted source documents are not runnable user-manual examples.

A user-visible change to settings, commands, errors, modules, review or dashboard
behaviour must update an affected manual page; the failure identifies the change.
Changes which have no user-visible effect need an explicit checked reason, not
an arbitrary touched-file workaround. Generated content is checked against its
source, not manually duplicated to satisfy the rule.

The manual build itself, every generator and screenshot capture must have no
access to private corpora, data directories or raw lab results. Do not mount
those directories into the build worker or rely only on scanning the final
output. The sole eligible lab-results input is an explicitly admitted aggregate
metrics JSON export, privacy-reviewed before it crosses into the build context.
Screenshot applications receive only the versioned synthetic fixtures approved
for that purpose. Test the denied read attempts as well as the output: real post
text, member data, forum identity, private paths and credentials must not enter
pages, screenshots, logs or artifacts.

The weekly gardener reviews changes since the last gardening commit, updates
affected pages and opens the requested manual update PR. Front matter records
audience and reviewed source commit; git supplies the last-updated display.
Warn, without failing, about pages not reviewed for 60 days. The gardener follows
the same protected PR and validation-host availability rules as other agents.

## Acceptance examples

- A PR changes a setting or visible button but not its manual explanation: the
  manual-change check fails and identifies the affected feature.
- A private path or real post appears in a capture or generated page: publication
  is blocked, and the unsafe artifact is not uploaded.
- A generator or screenshot job attempts to read a private corpus, data directory
  or raw lab-result file: access is refused even if it would not have printed the
  content. Only the admitted metrics export and synthetic fixtures are available.
- A PR from an untrusted fork cannot run with publishing credentials or write to
  a stable manual path.
- A preview appears for the intended PR, remains isolated from other previews
  and is removed after closure without deleting any released manual.
- Release 1.0.1 leaves the 1.0.0 manual unchanged; latest advances only after
  the release gates pass on the corresponding candidate.
- Both themes pass contrast/axe checks and support keyboard-only navigation.
- The complete Getting started path is followed literally on a clean qualified
  environment once the product exists, ending in the first accepted link.
- If that path cannot yet be completed, the manual reports the missing product
  capability and does not claim full manual acceptance.

## Approval and rollout

Approval covers the manual toolchain, generation and publication. It does not
approve unimplemented product behaviour. Publish truthful current-state content
first, extend references as approved features become real, and complete the
literal clean-install trial before declaring the manual finished.
