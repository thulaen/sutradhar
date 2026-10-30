# Public manual design

Status: design specification awaiting approval and implementation.
Audience: documentation maintainers and reviewers.
Purpose: make the public manual understandable without reading source code.

## Authority and boundaries

The manual requirements originate in interview Q192. Q195 preserves every full
release version, Q200 permits reviewed anonymous aggregate results, Q203 pauses
publishing targets during outages, and Q207 makes point-and-click installation the
main user path. The [current decisions](../requirements/current-decisions.md)
resolve earlier alternatives. The [publishing specification](../specifications/manual-publishing.md)
owns build and deployment behaviour; this page owns visual and writing decisions.

The manual is for forum owners, administrators and moderators. Every page begins
with its audience and what the reader can accomplish. Developer architecture,
module development and internal implementation stay under `docs/dev/` and are not
published as manual pages. One necessary explanatory sentence is enough when a
user needs to understand a consequence.

## Visual sources

| Source | Intended use | Excluded material | Evidence state |
| --- | --- | --- | --- |
| [Kraken public website](https://www.kraken.com/) | Colour references for independently authored light and dark themes | Logos, marks, imagery, stylesheet code and suggestions of affiliation | Public homepage read on 2026-10-30; stylesheet values still to be recorded and checked |
| [BBC GEL](https://www.bbc.co.uk/gel) | Typography hierarchy, spacing and responsive layout principles | Reith fonts, BBC marks and copied components | Detailed typography and breakpoint source verification pending |
| [GOV.UK Design System](https://design-system.service.gov.uk/) | Clear steps, visible focus, underlined links, summary lists and understandable errors | Crown, Transport font, branded header and footer | Exact applicable component sources still to be checked |
| [WCAG 2.2](https://www.w3.org/TR/WCAG22/) | AA acceptance requirements | Claims that automated checks alone prove conformance | Required acceptance standard, not a completed assessment |
| [Google documentation guide](https://google.github.io/styleguide/docguide/) | Readable linked Markdown with one authoritative explanation | Claims of external certification | Guide index read on 2026-10-30 |

Public access does not by itself establish reuse rights. Record source review and
any required attribution before admitting a dependency, asset or copied material.
Colour values will be extracted as factual references and independently arranged;
vendor stylesheet implementations will not be committed.

## Required presentation

Use a system font stack without external font requests. Body text is 19 px on
desktop and 16 px on mobile, with about 70 characters per line, an 8 px spacing
grid and touch targets of at least 44 px. Provide one display size, two heading
sizes, body and small text. Record verified GEL breakpoints before implementation.

Use left-aligned text, underlined links and a conspicuous yellow focus indicator
with a black outline. Test the focus treatment against both themes, including
keyboard controls, menus, search, details and the step-by-step navigation.

Customise MkDocs Material through independently written CSS variables first.
Template overrides are allowed only where CSS cannot express the required
behaviour. Preserve search, copy buttons, instant navigation, tabs and the theme
switch. The initial theme follows the operating system; an explicit user choice
overrides it.

Required patterns are a "Start now" home action, numbered Getting started steps
with expandable detail, summary lists for settings and requirements, inset notes,
details controls for optional depth, and destructive-action warnings for
uninstalling, deleting data and rotating keys. Error-code pages use an error
summary, and unfinished capabilities use a notification banner. Preserve the
underlined links and visible focus treatment in every pattern.

The future `manual/stylesheets/tokens.css` must name every colour by role and
record its source. Its light and dark schemes must cover backgrounds, surfaces,
body text, secondary text, links, buttons, focus, borders, success, warning and
errors. Each foreground/background pair must be tested before use. No palette
values or passing contrast results are invented in this foundation document.

## Writing and availability

Use British spelling, short sentences and numbered instructions. Tell readers
what to click and what to expect. Explain the first use of each approved system
name, for example internal linking (setu, "bridge"). Keep exact setting names in
code formatting. Do not use marketing language or describe a difficult action as
"simply" or "just".

The normal installation route is guided and point-and-click. An agent or host
administrator performs necessary server preparation. Command-line reference is
optional advanced material, not a requirement imposed on beginners. Each shell
example uses a `bash` fence containing one command, without a leading prompt.

Only describe behaviours that exist in the documented version. Use a clearly
labelled "Not yet available" banner for unfinished capabilities. In particular,
v1 has no automatic link insertion or author-editor linking mode. Do not write an
installation walkthrough that implies an installable release exists today.

## Verification required before publication

Use only synthetic fixtures for screenshots and capture both themes. Check every
published page for keyboard use, layout at desktop and mobile widths, meaningful
headings, link text, focus visibility and automated accessibility violations.
Automated axe and contrast results supplement manual accessibility checks; they
do not replace them.

Test the complete installation instructions on a clean qualified Linux
environment once an installable version exists. Record discrepancies as product
or manual defects and correct them before declaring the manual complete.

Current status: no theme, generated reference, screenshot, rendered site,
deployment or clean-install walkthrough has passed these checks yet.
