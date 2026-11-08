# 0004: Manual publishing

The manual lets a beginner understand Sutradhar, then install and use it
without reading source code or using a terminal. This part covers what
the manual holds, how it is built and checked, and how it is published.
It is roadmap Phase 5 item 7 (the manual built into the app) and
Phase 7 item 2 (the public manual on GitHub Pages).

## What it does

### Content

1. User content lives in `manual/`. Developer material lives under
   `docs/`. The manual never holds architecture, internal design or
   module-building guides. Where one sentence of background helps a user
   decide, it gives that sentence and no more.
2. Every page names its audience and the result the reader will see.
3. Navigation: Home; Getting started; Using Sutradhar; Modules;
   Configuration; Operations; Troubleshooting; Reference. Every module
   page uses the same user-facing layout.
4. The guided install path covers getting the owner's PC ready and ends
   in a health result the owner can understand. Commands are advanced
   reference, never the main install path.
5. The manual describes only what works. A feature that does not work
   yet carries a clear "Not yet available" banner stating the present
   limit. The manual never shows a feature that does not exist as
   working, never makes up a live screenshot and never implies the full
   install trial has passed.
6. Automatic link insertion is not available in version 1, and the
   manual says so.
7. The administration and WordPress safety pages are part of the
   manual, under the same rules.

### Generated parts

1. Settings pages come from Sutradhar Boot's typed settings. Command
   pages come from the `sutra` command's structure. Error pages come
   from one central catalogue of `SUT-NNN` codes.
2. Each generated item says when a user needs it, its default or
   expected result, and what to do.
3. Every error message on screen carries its code and a link to its
   manual page.
4. Release notes come from the changelog.
5. There is no API reference in the user manual.
6. Generators read only admitted safe definitions. They never start
   product services or touch live forum credentials.

### Screenshots

1. Screenshots are taken from the real Dioxus app, using made-up fixture
   data, in light and dark themes.
2. Private data is never opened to make a screenshot.
3. Before a screen exists, the build reports the missing screen. It
   never puts in a mock-up shown as a capture.

### Design

1. The manual follows SDS: GOV.UK and BBC GEL patterns, through design
   tokens and the smallest template changes. Palette sources and design
   choices are recorded in the developer docs.
2. System fonts, clear focus states, underlined links, readable line
   lengths, the set type and spacing rules and minimum touch target
   sizes. Both themes and every interactive part are checked.

### Builds and previews

1. The manual is built with mdBook, pinned in `gate/tools.toml`.
2. At the push tier, Dell builds the candidate's manual in an isolated
   workspace that holds only the approved made-up fixtures and the
   admitted metrics export, and runs every check below.
3. The built site is stored as a new, unchangeable snapshot named after
   the candidate tree. The owner can see its pages or screenshots on
   request. Previews never go to GitHub Pages, so nothing needs removing
   and no preview can clash with another path.

### Publication

1. GitHub Pages uses its "GitHub Actions" source, so no branch holds the
   site.
2. One workflow runs on the push that the sync makes to GitHub's `main`.
   It runs on a GitHub-hosted runner with read-only contents and only
   the Pages deployment rights. It:
   1. builds `dev` from that `main` commit with pinned tools;
   2. adds every frozen release manual, downloaded from the release
      asset attached when that version was released;
   3. writes the version switcher data, pointing `latest` at the newest
      qualified release, or stating that no stable release exists yet;
   4. uploads the whole site and deploys it to Pages.
3. Each full version, patch numbers included, keeps a frozen manual.
   Frozen manuals are never rebuilt. Each release attaches its built
   manual as a release asset whose hash is recorded on Dell. The
   workflow checks the downloaded hash before using it. A changed or
   missing asset stops the deployment.
4. A candidate tag may give a clearly labelled candidate preview, but
   never moves `latest`. `latest` moves only after every release check
   has passed and the release step has published the release with its
   manual asset.
5. Older frozen manuals and the version switcher are kept. Before the
   first release, the site states plainly that no stable release
   exists, rather than showing a false `latest`.
6. The workflow file is a protected file under
   [AGENTS.md section 9.3](../../AGENTS.md#93-github-ci). It uses only
   official actions pinned by commit hash. It has no private data, no
   access to Dell and no secret, because all it builds is already public
   source. No Pages step depends on the owner's editing computer being
   awake.

### The manual in the app

The same manual is built into the desktop app, so the owner can read it
without a network.

### Checks

1. A strict build; internal and external link checks; a spelling check;
   contrast checks on every token pair; an accessibility check on every
   page; keyboard navigation coverage.
2. Every runnable example in the manual is run, and shell blocks follow
   the set style. Quoted historical source documents are not runnable
   examples.
3. A change to settings, commands, errors, modules, the review pages or
   dashboards that a user can see must update the affected manual page.
   The check names the change. A change with no visible effect needs a
   stated, checked reason, not a touched file. Generated content is
   checked against its source, never copied by hand to pass this rule.

### Keeping private data out

1. The build, every generator and the screenshot tool have no access to
   private data, data folders or raw lab results. Those folders are not
   mounted into the build at all; scanning the output is not enough.
2. The only lab input allowed is an admitted aggregate metrics export,
   privacy-reviewed before it enters the build.
3. The screenshot app gets only the versioned made-up fixtures approved
   for it.
4. Real post text, member data, forum identity, private paths and
   credentials never enter pages, screenshots, logs or artefacts.

### Weekly care

1. A weekly job reviews changes since its last run, updates the affected
   pages and opens a Forgejo task with the manual update.
2. Each page's front matter records its audience and the source commit
   it was last reviewed against. Git gives the "last updated" date.
3. A page not reviewed for 60 days gets a warning that does not fail the
   build.
4. The weekly job follows the same rules as other agents, including the
   pause when Dell is not available.

## Limits

- Targets: a full build with screenshots within 4 minutes; a `main`
  update published within 5 minutes, measured from the update to the
  Pages deployment, while the services it needs are up. Both are
  measured as the manual grows. A missed target is reported and fixed,
  never hidden by skipping a check.
- Publication pauses when GitHub, Dell or a needed service is down, and
  reports the delay.
- Build workers run within the heavy-job limits of AGENTS.md section 11.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Rust tests on the manual tools and workflow, plus checks on the built
site:

- A change to a setting or a visible button with no manual change fails
  the manual-change check, which names the feature.
- A private path or real post in a capture or generated page stops
  publication, and the unsafe artefact is not uploaded.
- A generator or screenshot job that tries to read private data, a data
  folder or a raw lab result is refused, even if it would print
  nothing. Only the metrics export and made-up fixtures are reachable.
- A candidate preview is built and checked on Dell, stored under its
  tree identity and never published.
- A frozen manual asset whose hash differs from the recorded one stops
  deployment.
- A `main` update publishes `dev` without touching any frozen version
  or `latest`.
- `latest` moves only when the release step has published a qualified
  release with its manual asset.
- Release 1.0.1 leaves the 1.0.0 manual unchanged.
- The workflow cannot write repository contents, make branches or read
  anything private.
- Before the first release, the site states that no stable release
  exists.
- Both themes pass the contrast and accessibility checks, driven by a
  headless browser, and every page works by keyboard alone.
- Once the product exists, the whole Getting started path is followed
  word for word on a clean test machine and ends in the first accepted
  link. Until it can be, the manual reports the missing capability and
  does not claim full acceptance.

## Changes from the source

- The site generator and its plug-ins are replaced by mdBook.
- Settings, command and error pages come from Rust types and the `sutra`
  command, not from the old model and command libraries.
- Screenshots come from the Dioxus app, captured with a headless
  browser.
- Runnable examples are Rust and shell examples run as tests.
- Previews per pull request and the `gh-pages` branch are gone, since
  only `main` exists.
- The weekly update opens a Forgejo task, not a pull request.
- Release identity duties fall to the release step of 0005.
- The manual is also built into the desktop app, as the roadmap asks.

Source: `docs/specifications/manual-publishing.md` and
`docs/specifications/manual-publishing-main-only.md` in the archive
`archive-2026-11-08-old-repo`.
