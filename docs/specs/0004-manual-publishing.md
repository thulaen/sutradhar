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

1. Settings pages come from Behemoth Boot's typed settings. Command
   pages come from the `sutra` command's structure. Error pages come
   from one central catalogue of numeric `SUT-` codes.
2. Each generated item says when a user needs it, its default or
   expected result, and what to do.
3. Every error message on screen carries its code and a link to its
   manual page.
4. Release notes come from the changelog.
5. There is no API reference in the user manual.
6. Generators read only admitted safe definitions. They never start
   product services or touch live forum credentials.
7. Every build also writes `llms.txt`, a plain-text index of the
   manual's pages with a one-line summary each, so AI tools can read
   the manual (the source's `mkdocs-llmstxt`). It is made by the build
   from the pages, never written by hand, and holds nothing a page does
   not.

### Screenshots

1. Capture the real Tauri app with Angular and SDS on the admitted
   desktop profiles, with made-up data and both themes. Where setup or
   a screen differs by platform, show the exact labelled variant.
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
2. At push, Dell builds the manual in an isolated workspace with
   approved synthetic fixtures and the admitted metrics export.
   Admitted platform runners make desktop captures. Dell verifies
   signed evidence for the exact
   candidate, build, profile, fixture and theme. Missing or foreign
   evidence fails. QR-099 owns Windows runner admission.
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

   Before upload, verify that `dev` has the generated references,
   required captures and any admitted metrics input for its exact
   synced commit. Use only public inputs admitted by this workflow.
   If any required input is missing or belongs to another build,
   hold the deploy and keep the published site.

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

   Bind the served version subtree to its exact verified frozen
   asset through the admitted deployed-file manifest. A matching
   archive digest cannot stand in for served file verification.
   A label, redirect or mirror asset alone supplies no result.

   For an exact release read-back under 0005, verify the versioned
   Pages URL against the signed release identity and its frozen asset
   digest through the authorised deployment evidence. Use the
   observed deployment for this read. Keep the original verified
   publication receipt as history. A newer release on `latest`
   leaves the older exact version check eligible. Check `latest`
   independently against the current qualified switcher. Missing or
   mismatched read-back gives no new success and preserves verified
   history. A read never moves `latest`, changes a frozen edition,
   starts a deployment or repeats a release effect.
6. The workflow file is a protected file under
   [AGENTS.md section 9.3](../../AGENTS.md#93-github-ci). It uses only
   official actions pinned by commit hash. It has no private data, no
   access to Dell and no secret, because all it builds is already public
   source. No Pages step depends on the owner's editing computer being
   awake.

#### Frozen asset arrives after the synced main run

A main-triggered run can finish before the final release and its frozen
asset reach the mirror. A later asset or mirror receipt does not prove
a Pages deployment. A hold before dispatch leaves the published site,
frozen editions and `latest` in place. After an authorised dispatch
with an uncertain result, keep the last verified publication receipt
and show the deployment as unverified until exact read-back. The held
receipt grants no repeat or rollback.

QR-245 owns the protected trigger or rerun for this case. With an
admitted route, match the exact synced commit, verified release and
frozen asset digest. Bind the current authorised Pages deployment
and the complete proposed version map before dispatch. Preserve
current `dev`, the newest qualified `latest` and every frozen
edition. A late run may add a missing verified edition under the
admitted route. Hold a stale bundle if the current deployment or
required inputs change before the protected dispatch step. Rebuild
only from the current authorised complete plan. If the route cannot
prove it preserves newer publication, hold. Reconcile an uncertain
deploy against its exact recorded attempt and owning read-back.
A missing listing or elapsed deadline cannot prove no deploy. Retry
only under the admitted route after proving the original did not
perform the effect, fencing the old attempt and checking current
authority and expected deployment. If the original
completed, return its verified read-back without deploying again.
An already verified tag or release keeps its own receipt under 0005.

### The manual in the app

Package the manual in the app. Keep same-release recovery help and
stable local error links in the verified installer bundle. Open them
without launching its installer, app or engine. Spec 0030's separate
trust check applies before treating the bundle as genuine. For a paused
update, match the guide's signed release and recovery step to the
original recorded update under specs 0030 and 0031. A prior release's
verified guide cannot substitute for that plan's help. If the match
is unproved, keep **Update paused** with Resume and Rollback held.
Opening the guide cannot launch the app or change the installation.

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
- Under a separately admitted QR-245 late route, prepare a full site
  bundle against the current authorised Pages deployment A. Keep the
  verified release, mirrored asset and complete proposed version map
  fixed. Before dispatch let another authorised run commit deployment
  B with the same served files and visible version map as A. The pending
  bundle's expected deployment is still A. Hold without upload or
  deploy until a fresh authorised complete plan binds B. Keep the
  existing site and release and mirror receipts. The exact control
  adds the missing verified edition under the complete plan bound
  to B and preserves `dev`, qualified `latest` and all frozen editions.
  Use synthetic deploy effects only.
- A `main` update publishes `dev` without touching any frozen version
  or `latest`. Offer a missing required input or a capture from another
  commit. The workflow holds before upload and preserves the prior site.
- `latest` moves only when the release step has published a qualified
  release with its manual asset.
- Release 1.0.1 leaves the 1.0.0 manual unchanged. After the
  qualified 1.0.1 update to `latest`, read back the exact 1.0.0
  versioned URL and deployed frozen bytes. Use the observed current
  deployment with the original signed asset binding. The 1.0.0 read
  passes and the separate `latest` read returns 1.0.1. Offer 1.0.1
  bytes at the 1.0.0 URL, a missing version route and a candidate
  preview in separate trials. Also offer a plausible version subtree
  with the wrong frozen asset or deployment binding. Each case
  withholds new success and retains
  verified publication history. The check changes no pointer,
  deployment, tag, signature or release.
  In a separate read-only case keep the 1.0.0 versioned URL,
  observed deployment and all served files exact. Public access
  still works. Offer only the original verified deployment
  receipt after it no longer meets the admitted current proof
  requirements. Keep that receipt as history. Withhold a fresh
  verified read-back or closure result. The exact current proof
  under the admitted reader clears only this check. The public
  read and the proof check change no release or deployment state.
- The workflow cannot write repository contents, make branches or read
  anything private.
- Before the first release, the site states that no stable release
  exists.
- Both themes pass the contrast and accessibility checks, driven by a
  headless browser, and every page works by keyboard alone.
- MAN-PLATFORM: follow Getting started on clean Ubuntu 26.04,
  Debian 13 and Windows 11. Reach the first accepted link on XenForo
  2.3 and each supported WordPress major. Exercise native grant
  refusal under spec 0031 IP-15 and spec 0030's Windows pre-launch
  trust check with a changed installer. On Ubuntu and Debian,
  reuse IT-0030-VERIFIED-STAGE from spec 0005. Follow the manual
  through the graphical verifier and native installer. Plant a
  changed package after verification and separately remove proof
  of the same staged copy at handoff. The owning 0030 check gives
  SUT-832 before any install effect. Follow the same-release help
  to repeat verification, with no effect until the check passes.
  Check both themes, keyboard, screen-reader use and exact
  candidate screenshots.
- MAN-OFFLINE: disconnect the network. Find local help for install,
  pairing, backup, restore, removal and an error code. Repeat
  recovery with no installed app or engine, using verified media.
  Join 0030 test 16 and 0031 IP-40 for a paused update. Stop the app
  and engine after the data format changes. Offer verified guides
  for both the previous release and the recorded update. Open only
  the guide that matches the recorded held step. In separate cases
  give only the earlier guide or withhold the original update record.
  Show the safe hold with no Resume or Rollback. Keep current files
  and data. Opening help launches neither payload. Hold the
  walkthrough if platform proof is missing.
  In a separate offline case keep the paused update on release B.
  Offer the verified B guide and a later independently verified
  release C bundle and guide on local media. Both guides list the
  same error code. With the app and engine stopped, follow the
  recorded error link to B's exact held step. Withhold the B guide
  in a separate run. Keep the safe hold even though C is genuine.
  Both runs keep the current data and launch neither installer,
  app nor engine. Reading B grants no Resume or Rollback.
  On each admitted Linux and Windows offline recovery profile, keep
  the exact recorded hold, independently verified current guide and
  current package fixture. Provide valid manifest M and release
  record R. Offer a genuine earlier fixture verifier qualified
  only for the manifest check, under 0065's existing test route.
  This verifier has no admitted reader for the required record R.
  Allow the verified guide to open for read-only help. Hold native
  package use, recovery launch and Resume or Rollback. Preserve
  current data. The exact current admitted verifier is the control
  for package proof and supplies no automatic recovery action.
  In the IP-26 current-data-format branch, retain the independently
  verified previous payload A and recorded update B's verified local
  guide. The current data format is incompatible with A. Offer a
  genuine page from B for an interrupted download as the held
  rollback step's help. It cannot match this recorded step. Keep
  Update paused, data and receipts intact. The correct B page opens
  read-only and gives the current data format reason from 0030.
  Neither branch starts A, resumes writes, offers Resume or Rollback,
  or reports a verified return or SUT-835.

## Changes from the source

- The site generator and its plug-ins are replaced by mdBook; the
  `llms.txt` index is kept, made by our own build step.
- Settings, command and error pages come from Rust types and the `sutra`
  command, not from the old model and command libraries.
- Desktop screenshots come from the real Tauri window. The
  publication checks use a browser for the built manual.
- Runnable examples are Rust and shell examples run as tests.
- Previews per pull request and the `gh-pages` branch are gone, since
  only `main` exists.
- The weekly update opens a Forgejo task, not a pull request.
- Release identity duties fall to the release step of 0005.
- The manual is also built into the desktop app, as the roadmap asks.

Source: `docs/specifications/manual-publishing.md` and
`docs/specifications/manual-publishing-main-only.md` in the archive
`archive-2026-11-08-old-repo`.
