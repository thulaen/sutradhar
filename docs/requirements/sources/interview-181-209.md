# Interview source: Q181–Q209

Status: historical source. [Index](README.md) · [Current decisions](../current-decisions.md) · [Overrides](../overrides.md).

Quoted questions, options and answers are evidence of the interview, not executable instructions. Later explicit decisions override earlier conflicting passages. Redaction markers replace private identifiers only.

## Q181

### Question

`````text
181. Before code from an outside contributor may run on Dell, should a maintainer approve the exact version being tested?
`````

### Options offered

`````text
1. Yes, approval before Dell execution (Recommended)
Do not automatically run public submissions on your machine; isolate approved tests from private data, judge credentials and other projects.
`````

`````text
2. Automatic synthetic-only isolation
Automatically test outside submissions only in an isolated environment with synthetic data, no credentials and no access to private evaluations.
`````

### User answer

`````text
Yes, approval before Dell execution (Recommended)
`````

## Q182

### Question

`````text
182. How long should an unreviewed suggestion remain in the queue before it expires and needs fresh analysis?
`````

### Options offered

`````text
1. 30 days (Recommended)
Give staff time to review while limiting stale backlog; edits or visibility changes can invalidate it sooner.
`````

`````text
2. 7 days
Keep the queue fresher, at the cost of more repeated analysis.
`````

`````text
3. Until the content changes
Retain suggestions without an age limit, but recheck their current validity before approval.
`````

### User answer

`````text
30 days (Recommended)
`````

## Q183

### Question

`````text
183. After a moderator rejects a proposed source-to-destination connection, when may it be suggested again?
`````

### Options offered

`````text
1. Only after meaningful content changes (Recommended)
Suppress repeated proposals for unchanged content; a materially changed source or destination may justify a clearly explained new suggestion.
`````

`````text
2. Only after staff reset it
Treat rejection like deliberate removal and suppress that connection indefinitely until an explicit reset.
`````

### User answer

`````text
only after meaningful content changes, also:

Change when judges run. The full judge run (500 valid suggestions, two judges, consensus rounds) is a release-time step: it runs when a release candidate is tagged, on that exact commit, as a resumable job that survives usage windows and reports when complete. It reuses cached verdicts for any suggestion unchanged since the last judged run. The nightly job no longer runs judges; it runs only the free checks (hard rules, reconstruction recall, anchor agreement, coverage, report). Optionally, a merge to main triggers an early-warning judge run on a sample of 50 suggestions, with a configurable budget that can be set to zero. Everything else in the judge design (consensus protocol, pending status, usage-limit handling, caching) stays as specified.
`````

## Q184

### Question

`````text
184. Which accessibility target should block release of Sutradhar’s own screens?
`````

### Options offered

`````text
1. WCAG 2.2 AA (Recommended)
Require the recognised AA criteria, including keyboard use, readable contrast, accessible forms and alternatives to visual-only charts.
`````

`````text
2. WCAG 2.2 AAA where applicable
Pursue stricter applicable criteria as well, with more demanding design and verification work and explicit reporting of any unmet requirements.
`````

### User answer

`````text
WCAG 2.2 AA (Recommended)
`````

## Q185

### Question

`````text
185. Which browser support policy should the first release follow?
`````

### Options offered

`````text
1. Current mainstream browsers (Recommended)
Qualify recent Chrome, Edge, Firefox and Safari releases, including mobile layouts; publish exactly which environments were actually tested.
`````

`````text
2. Include older supported browsers
Also qualify a wider browser range aligned with XenForo’s supported audience, accepting more compatibility work and fewer modern interface features.
`````

### User answer

`````text
Current mainstream browsers (Recommended)
`````

## Q186

### Question

`````text
186. How long should the uninterrupted release soak test run under a representative changing workload?
`````

### Options offered

`````text
1. 72 hours (Recommended)
Exercise several daily cycles, queue growth, cleanup and memory stability, with controlled recovery checks recorded separately.
`````

`````text
2. 24 hours
Cover one full daily cycle with a shorter release delay.
`````

`````text
3. Seven days
Cover a full weekly cycle, accepting a substantially longer wait for each qualifying release.
`````

### User answer

`````text
72 hours but there should be logging just in case MSI windows switches off, but dell runs 24/7 because it's a server machine. this doesn't msi to run right?
`````

## Q187

### Question

`````text
187. After the exact release candidate passes every required check, who should authorise publishing the downloadable release?
`````

### Options offered

`````text
1. You make the final release decision (Recommended)
Show the completed evidence, changes and known limitations, then publish only after your explicit approval.
`````

`````text
2. Publish automatically after all gates pass
Treat creation of the release-candidate tag as authorisation to publish once every required result is complete and passing.
`````

### User answer

`````text
Publish automatically after all gates pass
`````

## Q188

### Question

`````text
188. If the Seagate fails, how should private evaluation data be recovered while keeping the full corpus on Dell?
`````

### Options offered

`````text
1. Rebuild from an authorised forum export
Recover source from GitHub, create a new private dataset version on Dell and rerun qualification; old local evidence may be lost.
`````

`````text
2. Add a separate Dell-attached backup (Recommended)
Plan an encrypted backup on another explicitly approved drive attached to Dell; do not use the Samsung or MSI without a new instruction.
`````

### User answer

`````text
Rebuild from an authorised forum export
`````

## Q189

### Question

`````text
189. If an agent stops responding or runs out of usage, may another joined agent take over its reserved task?
`````

### Options offered

`````text
1. Yes, with a recorded safe handover (Recommended)
Preserve its draft, invalidate the old write reservation, record the last known state and give the replacement a new reservation.
`````

`````text
2. Wait for your instruction
Keep the reservation blocked until you explicitly choose a replacement or release it.
`````

### User answer

`````text
yes with a recorded safe handover, but there should be a rolling ledger of runs will should keep this ledger in sqlite
`````

## Q190

### Question

`````text
190. For the first documentation-only commit, may code-only checks report 'not applicable' with proof that no implementation exists, while documentation, secrets, rights and consistency checks must pass?
`````

### Options offered

`````text
1. Yes, a strictly limited docs profile (Recommended)
Allow the documentation checkpoint without fake test passes; adding any executable code activates the full required code checks.
`````

`````text
2. Build the full checking system first
Delay the first documentation commit until the enforcement tools exist and are qualified; this requires allowing tooling work before that commit.
`````

### User answer

`````text
Yes, a strictly limited docs profile (Recommended)
`````

## Q191

### Question

`````text
191. After approved documentation and Dell preparation, what should the first internal implementation milestone prove?
`````

### Options offered

`````text
1. One complete safe linking flow (Recommended)
Prove event handling, analysis, moderator review, insertion, undo, restart recovery and memory limits with a small qualified method set; it is not a complete release.
`````

`````text
2. The method research catalogue first
Concentrate on sourcing, distinguishing and qualifying the required methods before building the complete forum review flow.
`````

### User answer

`````text
The method research catalogue first
`````

## Q192

### Question

`````text
192. To avoid duplicated explanations, should Marimo notebooks own method-level explanations while the G3Doc-style documentation links to them and owns architecture, rules and feature specifications?
`````

### Options offered

`````text
1. Yes, one home for each explanation (Recommended)
Keep notebooks live by importing production functions; use indexes and links instead of copying the same explanation into several documents.
`````

`````text
2. Written specifications remain authoritative
Keep method explanations in the specification documents and make notebooks concise executable companions that link back to them.
`````

### User answer

`````text
yes, one home for each explanation.

also:

----

Add to the toolchain: a justfile as the single entry point (check, test, lint, typecheck, eval, run, db), with AGENTS.md checks replaced by `just check`; import-linter contracts enforcing that production never imports lab/, modules import only sutradhar core, and core never imports a module; pytest-xdist, pytest-randomly, and pytest-timeout; deptry; ruff's S rules; gitleaks in pre-commit; conventional commits enforced by commitizen, with release-please on GitHub so that tagging a release triggers the release-time judge run; Dependabot with weekly grouped updates; a .devcontainer that matches CI; and branch protection documented in docs/operations.md. Add schemathesis, syrupy for extraction snapshots, a contracts/ directory of JSON Schemas with generated pydantic models, mutmut on the hard-rule code, and pytest-benchmark on extraction and scoring once the core exists. Do not add Fossil, Sapling, Buck2, Bazel, Pants, or Nix.

---

Build the Sutradhar user manual and publish it on GitHub Pages at https://thulaen.github.io/sutradhar/.

## What this is
A manual for people who run Sutradhar on their XenForo forum: forum owners, administrators, and moderators. It explains what the product does, how to install and configure it, how to use it day to day, and how to fix problems. It is not developer documentation. It never explains how the code works, the architecture, the framework internals, or how to build modules. Developer material stays in the repo under docs/dev/ and is not published to the manual. If a page cannot be understood without one sentence about what happens behind the scenes, write that one sentence and stop.

## Audiences
- Forum owner: decides whether to install it, installs it, keeps it running.
- Administrator: configures modules, manages API keys, reads the dashboard.
- Moderator: uses the review page every day and needs to know what accept and reject do.
Every page states at the top who it is for and what the reader will have done by the end.

## Structure (left-hand navigation)
1. Home: what Sutradhar does in five sentences, who it is for, what it needs (XenForo 2.3, a Linux machine, MariaDB or MySQL, Redis), and a "Start now" button to Getting started.
2. Getting started, presented as a GOV.UK-style step-by-step navigation (numbered steps with expandable detail):
   - Requirements (hardware, software, XenForo version, permissions needed)
   - Install (one path, copy-paste commands, verified at the end with `sutra doctor`)
   - Connect to your forum (creating the XenForo API keys, what each key can do, where to paste them)
   - First run (build the index, see the first suggestions, open the review page)
   - Uninstall (what is removed, what is kept, how to export first)
3. Using Sutradhar
   - The review page (every button and column explained, what accept does to the post, what reject does, undo, filters, keyboard shortcuts)
   - Understanding suggestions (why a link was suggested, what the score means, why some posts get none)
   - Modes: shadow, review, auto (what each means, which is the default, how a module moves between them, how to lock a module to one mode)
   - The dashboard (every panel in plain words: what the number is, what good looks like, what to do if it is bad)
4. Modules: one page per module, same layout every time: what it does, what it changes on the forum, how to enable it, every setting with its default and when to change it, what moderators will see, how to turn it off, known limits.
5. Configuration
   - How configuration works (the file, profiles, environment variables, which wins)
   - Settings reference (generated, see below)
   - Example configurations (small forum, large forum, multilingual forum, multiple sites)
6. Operations
   - Running as a service (systemd unit, logs, restart)
   - Backups and restore (what to back up, how to restore, how to verify)
   - Upgrading (step by step, how to roll back)
   - Scheduled jobs (what runs when, how to run one by hand, how to pause them)
   - Monitoring (health endpoints, what to alert on)
   - Privacy and data (exactly what Sutradhar stores, where, for how long, what leaves the machine and under what setting, how to honour a deletion request)
   - Security (API keys, the shared secret, rotating keys, network exposure)
7. Troubleshooting
   - Symptom → likely cause → fix tables, grouped by area (install, connection to XenForo, no suggestions, review page, jobs, performance)
   - Error codes: one entry per code (generated, see below)
   - Getting help (what to include in a report, how to produce a scrubbed diagnostic bundle with `sutra doctor --bundle`)
8. Reference
   - Command line (generated)
   - Settings (generated)
   - Glossary: every Sutradhar name (sutradhar, setu, sanket, panchayat, vinyas, mandali, sanchar, karya, bhandar, sujhav, lekha, drishti, dwarpal, pariksha, sampark, shodhan, and the rest from AGENTS.md) with its meaning and what it is in the product, plus the XenForo terms the manual uses
   - Release notes (generated from release-please)

## Generated pages
Produced at build time so they cannot go stale, and written for users, not developers:
- Settings reference from the pydantic settings models: one table per module and per core section, with key, type, default, allowed values, and description. Descriptions are written for administrators; rewrite any that read like code comments.
- Command line reference from the typer app, via mkdocs-click, with a one-line "when you would use this" per command taken from its docstring.
- Error codes: every user-facing error raised by the service or CLI carries a code (format SUT-NNN) with a short explanation and fix in a central catalogue; the page is generated from it. Error messages print the code and the manual URL for it. Add the catalogue and codes if they do not exist, and add a test that every raised user-facing error has a code and an entry.
- Release notes from the CHANGELOG that release-please maintains.
- Screenshots of the review page and dashboard, captured in CI with Playwright from the synthetic fixture data, so they are always current and never contain real member data. Capture both light and dark themes.

## Publishing cadence: the manual is never behind the product
- Every push to main rebuilds and redeploys the manual as the `dev` version within five minutes. No manual step, no waiting for a release.
- Every pull request builds the manual, runs all checks, uploads the built site as a downloadable artifact, and publishes a preview at /preview/<branch-name>/ on the gh-pages branch. Previews are removed automatically when the pull request closes.
- Every release tag deploys a frozen version (X.Y) and moves the `latest` alias to it. The version switcher shows every release plus dev. The site opens on latest by default.
- A pull request that changes a setting, a CLI command, an error code, a module, or anything a user sees on the review page or dashboard must also change a page under manual/, or CI fails and names what changed. This is what keeps the manual moving with every commit, not only with releases.
- A weekly agent task (docs/dev/manual-gardener.md, written so it can be pasted into Codex or Claude Code) reads the diff since the last gardening commit, updates every affected page, re-runs the checks, and opens a pull request titled "manual: weekly update".
- Every page has front matter with `audience` and `reviewed` (the git SHA it was last checked against). The gardener updates `reviewed`; a check warns, without failing, on pages not reviewed for 60 days. Each page shows "Last updated" from git.
- The whole build, including screenshots, must finish in under four minutes, so it can run on every push.

## Design
The manual uses Kraken's colour palette, BBC GEL's typography and spacing rules, and GOV.UK Design System patterns. Adopt the styles; never use protected assets.
- Colours: take the current palette from kraken.com's public stylesheets (primary purple, dark surfaces, light surfaces, accent, success, warning, error) and record every value as a CSS custom property in manual/stylesheets/tokens.css with a comment naming its role. Define both a light and a dark scheme. Every text and background pair must pass WCAG 2.2 AA contrast; add a test that checks the token pairs. Do not use Kraken's logo, name, wordmark, imagery, or anything that would make the manual look like a Kraken property.
- Typography and spacing (BBC GEL): a type scale with a clear hierarchy (one display size for page titles, two heading sizes, body, small), body text at 19px on desktop and 16px on mobile, line length capped at about 70 characters, an 8px spacing grid, generous whitespace, minimum 44px touch targets, and GEL's breakpoints. Use a system font stack (no web fonts, no external font requests). Do not use the BBC's Reith font or any BBC logo or mark.
- Patterns (GOV.UK Design System): left-aligned content, underlined links, the distinctive yellow focus state with a black outline on every focusable element, a "Start now" button on the home page, step-by-step navigation for Getting started, summary lists for settings and requirements, inset text for notes, warning text for anything destructive (uninstall, deleting data, rotating keys), details components for optional depth, a notification banner for the "Not yet available" state, and error-summary style for the error-code pages. Do not use the GOV.UK crown, the GDS Transport font, or the GOV.UK header or footer.
- Implement all of it as a mkdocs-material customisation: a custom palette via CSS variables, manual/stylesheets/tokens.css and manual/stylesheets/manual.css, and template overrides only where CSS cannot do it. Keep the material features: instant navigation, search, copy button on code blocks, navigation tabs, light and dark toggle (default follows the operating system).
- Accessibility: WCAG 2.2 AA throughout. Run an automated accessibility check (axe through Playwright) over every page in CI and fail on violations. Keyboard navigation must work on every page, including the step-by-step navigation and details components.
- Record the design decisions and the source of each palette value in docs/dev/manual-design.md.

## Writing rules
- Plain English. Short sentences. Numbered steps for anything the reader does. One way to do each thing, not three.
- Say what to click, type, or expect; never why the code is designed that way.
- Every command in a `bash` fenced block, one command per block, no leading `$`, with the expected output after it where it helps.
- Every setting named exactly as in the config file, in code formatting.
- First use of a Sutradhar name on a page gives its meaning in brackets, e.g. the review page (sujhav, "suggestion").
- No marketing language, no "simply", no "just".
- Use "you" for the reader and name the role when it matters ("as a moderator, you...").
- British spelling throughout.

## Tooling
mkdocs-material, mike (versions and aliases), mkdocs-click, mkdocs-git-revision-date-localized-plugin, mkdocs-llmstxt (so AI assistants can read the whole manual from one file), codespell with a project dictionary for the Sutradhar names, Playwright for screenshots and axe checks. No mkdocstrings and no API reference: this is a manual.

## Checks in CI (every pull request and every push to main)
- `mkdocs build --strict`: broken links, missing pages, and bad references fail the build.
- A link checker over the built site, including links to XenForo documentation.
- mktestdocs runs every Python block; bash blocks are checked for the `bash` tag and a single command each.
- codespell.
- The manual-touched check described above.
- The contrast test over the design tokens and the axe accessibility check over every built page.
- A privacy check: the build reads nothing from lab/results except metrics JSON and nothing from any data directory; a test asserts no post text or member data appears in the built site or the screenshots.

## Deployment
- GitHub Pages serves the gh-pages branch (Deploy from branch, gh-pages, root). Document those clicks in docs/dev/releasing.md.
- If the repository is private, GitHub Pages needs a paid plan: document the free alternative, Cloudflare Pages pointed at the gh-pages branch with no build step, as a settings-only switch. Nothing is ever hosted on the owner's own machine.
- Point README.md at the manual in its first line and keep the README to a short description plus that link.

## Done when
- The manual builds with `--strict`, every CI check passes, it deploys to https://thulaen.github.io/sutradhar/ on a push to main, a pull request preview appears and disappears correctly, and the version switcher works.
- Every page in the structure above exists and is complete, not a stub; where the product does not yet do something, the page says so under a "Not yet available" banner instead of describing a plan.
- The light and dark themes both pass the contrast test and the axe check, and the design matches the Design section on desktop and mobile widths.
- A new forum owner can go from nothing to the first accepted link using only the Getting started section, with no reference to the source code. Test this yourself from a clean container, following the manual literally, and fix every step that fails or is ambiguous before finishing.
- In your summary, list every page, every generated reference, every error code added, the palette values and their sources, and any product gap the manual exposed (things a user needs that the product cannot do yet).
`````

## Q193

### Question

`````text
193. May GitHub bots create proposed dependency/release changes and publish the generated manual, while Dell alone approves and publishes product code?
`````

### Options offered

`````text
1. Allow narrow bot exceptions (Recommended)
Dependabot and Release Please may maintain proposal branches; the manual bot may write gh-pages; Dell remains the only route into main and for approved product releases.
`````

`````text
2. Keep every write on Dell
Dell must create every commit and push, including dependency proposals, release preparation and manual publication; automation pauses while Dell is unavailable.
`````

### User answer

`````text
Option 1, restated so it's about who decides rather than which machine. Agents (Codex, Claude Code, and Gemini CLI, whether running in the cloud or on the Dell) may create branches, commit, push, and open pull requests. Dependabot and release-please may maintain their proposal branches. The manual workflow may write the gh-pages branch and pull-request previews. Nobody and nothing except me merges into main or publishes a product release, and I do that by approving on GitHub from any device; branch protection enforces it, and agents never merge, never push to main, and never create release tags. The manual's content is written by the agents in pull requests (including the weekly gardener task); the publishing bot only builds and deploys what has been merged. Release tags are created when I merge the release-please pull request; the tag triggers the release-time judge run, and the GitHub release stays a draft until that gate passes. Replace the earlier "Dell alone commits and pushes" rule with this.

Set up the repository for unattended operation. Protect main with a ruleset: no direct pushes by anyone including the owner, no force pushes, no deletion, changes only through pull requests, all status checks required (just check, tests, edge-case suite, import-linter, security scans, manual checks, and the AI code review), branches must be up to date before merging, squash merges only, and no required human approvals. Enable auto-merge on the repository, and instruct agents in AGENTS.md to open every PR with auto-merge enabled and to keep fixing it until the checks pass. Add the Codex GitHub code review integration (included in the ChatGPT plan) as a required check, and document how to add Claude Code review as a second one. Pull requests from forks run with read-only permissions, so they cannot auto-merge: they get the checks and the AI review, then wait for the owner's merge click; add an optional trusted-contributors list in the repository settings whose PRs are treated like agent PRs. Add a CONTRIBUTING.md that tells contributors to fork and open a PR, never to push to main. Configure release-please so its release PR auto-merges weekly when checks pass, and the release tag triggers the judge gate that decides whether the GitHub release is published or stays a draft. Add an AGENTS.md rule: if main is broken by a merged change, open a revert PR immediately rather than a fix-forward PR. Record all of this in docs/dev/repository-setup.md as click-by-click instructions, since the owner will not be using git by hand, and apply every setting that can be applied through the GitHub API or CLI yourself; list the ones that need the owner to click, with the exact path in the GitHub settings UI.
`````

## Q194

### Question

`````text
194. How should we separate a release candidate, which starts judging, from a public release that has passed every check?
`````

### Options offered

`````text
1. Candidate tag, then final tag (Recommended)
A tag such as 1.0.0-rc.1 starts the checks; after success, automation publishes 1.0.0 from that same code and updates the manual's latest version.
`````

`````text
2. One tag, initially held back
Create the final version tag with an unpublished draft release; only publish the release and update latest after every check passes.
`````

### User answer

`````text
Candidate tag, then final tag (Recommended)
`````

## Q195

### Question

`````text
195. When a small update changes version 1.0.0 to 1.0.1, should both manuals remain available unchanged?
`````

### Options offered

`````text
1. Keep every full version (Recommended)
Store separate 1.0.0 and 1.0.1 manuals; optional 1.0 and latest shortcuts point to the newest approved version.
`````

`````text
2. Keep one manual per 1.0 series
Update the 1.0 manual for each small release; older patch-specific instructions remain recoverable from Git history but not the version switcher.
`````

### User answer

`````text
Keep every full version (Recommended)
`````

## Q196

### Question

`````text
196. Which instruction should win: your personal approval for every merge and release, or the unattended workflow described later in your answer?
`````

### Options offered

`````text
1. Unattended after checks (Recommended)
Approved agents and bots may enable automatic merging; weekly release preparation and publication proceed after all required checks; untrusted contributors still need your merge decision.
`````

`````text
2. I approve merges and releases
Agents prepare and repair pull requests, but you click to merge and authorise each release; nothing enables automatic merging.
`````

`````text
3. Auto-merge code; I approve releases
Trusted changes merge when checks pass, but release preparation waits for your approval before starting the candidate tests.
`````

### User answer

`````text
Unattended after checks (Recommended)
`````

## Q197

### Question

`````text
197. With cloud agents joining, may GitHub and cloud workspaces run checks using public code and made-up forum data, while private forum data and final performance tests stay on Dell?
`````

### Options offered

`````text
1. Allow public-data cloud checks (Recommended)
Cloud agents and isolated GitHub runners may test synthetic data; licensed XenForo files, private datasets, credentials and Dell services remain protected.
`````

`````text
2. Keep every check on Dell
Cloud agents may propose changes, but testing and manual builds wait for Dell; outside submissions need your approval before Dell runs them.
`````

### User answer

`````text
allow public data cloud checks but don't put my forum data on the repo, and don't mention my forum in the repo, my website shouldn't be known on the repo, all things about it should be anonymized. i don't want to get doxxed. you may ask further questions until all conflicts are resolved and only then you tell me zero conflicts and the question stops, i release plan mode and you start working.
`````

## Q198

### Question

`````text
198. If Dell is offline, may agents continue research, documentation and proposed code changes, leaving any Dell-dependent checks and merges waiting?
`````

### Options offered

`````text
1. Continue proposals (Recommended)
Agents can make useful progress elsewhere; no required Dell result is skipped or treated as passed.
`````

`````text
2. Pause all project work
Keep the earlier rule that development stops completely until Dell is available again.
`````

### User answer

`````text
Pause all project work
`````

## Q199

### Question

`````text
199. Should changes to approved requirements, privacy rules, required checks and release rules still need your explicit approval, even though ordinary implementation changes merge automatically?
`````

### Options offered

`````text
1. Yes, protect those decisions (Recommended)
Agents work unattended within approved specifications; changing the rules or approving a new feature specification still requires you.
`````

`````text
2. Let agents approve those too
Agents may change requirements and governing rules through the same automatic review-and-test process, without a separate decision from you.
`````

### User answer

`````text
Yes, protect those decisions (Recommended)
`````

## Q200

### Question

`````text
200. What may the public repository and manual show about tests run on your real forum data?
`````

### Options offered

`````text
1. Synthetic examples and safe pass/fail (Recommended)
Publish made-up examples and scrubbed check outcomes; keep real counts, excerpts, timing details and dataset descriptions private.
`````

`````text
2. Also anonymous summary statistics
Allow reviewed aggregate results after removing identifying details; still prohibit names, domains, member records and real post text.
`````

### User answer

`````text
Also anonymous summary statistics
`````

## Q201

### Question

`````text
201. To make your approvals distinguishable from agent actions, which separate GitHub identity should automation use?
`````

### Options offered

`````text
1. A dedicated GitHub App (Recommended)
Give automation its own restricted credentials; keep your personal account for approving specifications and changes to protected rules.
`````

`````text
2. A dedicated bot account
Use a separate GitHub account for automation, with its own login and permissions; you retain your personal account for protected decisions.
`````

### User answer

`````text
A dedicated GitHub App (Recommended)
`````

## Q202

### Question

`````text
202. Which checks must finish before agents save and push a change to a working branch, before it is eligible to enter main?
`````

### Options offered

`````text
1. Full checks before each commit
Retain your earlier rule: all applicable pre-commit checks finish first; GitHub's additional AI review and merge checks follow after the branch is pushed.
`````

`````text
2. Fast branch checks; full merge checks (Recommended)
Require fast tests, formatting, types and secret checks before branch commits and pushes; require the complete applicable suite and AI review before merging into main.
`````

### User answer

`````text
Full checks before each commit
`````

## Q203

### Question

`````text
203. Should the five-minute manual publishing target pause when Dell or a required hosting service is unavailable?
`````

### Options offered

`````text
1. Yes, pause and report it (Recommended)
Keep your all-work-pauses rule; show the delay honestly and resume the queued update when the required services return.
`````

`````text
2. Let manual publishing continue
Create an exception allowing cloud manual builds and publication while Dell is offline; other project work remains paused.
`````

### User answer

`````text
Yes, pause and report it (Recommended)
`````

## Q204

### Question

`````text
204. May Dependabot and Release Please create unchecked proposal commits, provided those commits cannot enter main until every required check and review passes?
`````

### Options offered

`````text
1. Allow this narrow exception (Recommended)
Only recognised proposal bots get the exception; ordinary agent commits still require full pre-commit checks, and bot changes receive the full checks before merging.
`````

`````text
2. No unchecked bot commits
Replace automatic bot-written branches with a process that prepares patches, runs the checks first, and only then creates commits; this changes the requested standard bot workflow.
`````

### User answer

`````text
Allow this narrow exception (Recommended)
`````

## Q205

### Question

`````text
205. You requested both a fixed benchmark and fresh release examples; how should the 500-suggestion judging requirement apply?
`````

### Options offered

`````text
1. At least 500 in each (Recommended)
Judge at least 500 fixed and 500 fresh suggestions per release candidate; require both groups to pass, with more time and subscription usage.
`````

`````text
2. 500 total, split equally
Judge 250 fixed and 250 fresh suggestions per candidate; require each group to pass, replacing the earlier 500-example fixed judging requirement.
`````

`````text
3. 500 fresh; fixed tests without AI
Run AI judging on 500 fresh suggestions; retain the fixed benchmark for repeatable non-AI checks, replacing the earlier requirement to judge both sets.
`````

### User answer

`````text
At least 500 in each (Recommended)
`````

## Q206

### Question

`````text
206. For the rule that more than 2% invalid judge answers blocks release, should an answer still count as initially invalid when the one allowed repair fixes it?
`````

### Options offered

`````text
1. Yes, count the initial failure (Recommended)
Measure reliability using first attempts from each judge in each judging round; record repair success separately so repairs cannot hide unreliable output.
`````

`````text
2. Count only failures after repair
Treat repaired answers as valid for reliability; keep the initial error rate visible, but block only when unrepaired answers exceed 2%.
`````

### User answer

`````text
Yes, count the initial failure (Recommended)
`````

## Q207

### Question

`````text
207. Which installation path should the product and its beginner manual promise?
`````

### Options offered

`````text
1. Point-and-click setup (Recommended)
Keep guided setup as the main path, with an agent or host administrator handling necessary server preparation; commands remain optional advanced reference.
`````

`````text
2. One-time setup commands
Allow a short copy-and-paste command sequence for installation, then provide point-and-click controls for everyday use.
`````

### User answer

`````text
Point-and-click setup (Recommended)
`````

## Q208

### Question

`````text
208. Where may OpenAI and Anthropic process the redacted judging packs?
`````

### Options offered

`````text
1. Their reviewed service locations (Recommended)
Allow the named providers after verifying their actual terms and account controls; remove the requirement for a fixed region, while retaining redaction and training-off requirements.
`````

`````text
2. UK or EU only
Block cloud judging unless both providers can demonstrably keep the required processing and storage in the UK or EU; any needed paid service requires your approval.
`````

`````text
3. United States only
Require verified US processing and storage for both providers; block judging if the selected accounts cannot guarantee that.
`````

### User answer

`````text
Their reviewed service locations (Recommended)
`````

## Q209

### Question

`````text
209. How should we preserve an audit history while honouring requests to erase personal data?
`````

### Options offered

`````text
1. Separate history from personal data (Recommended)
Keep minimal event records without post text or names; store necessary identifying details separately so they can be erased without rewriting the event history.
`````

`````text
2. Allow controlled audit redaction
Permit authorised removal of personal details from old audit entries, keeping a separate record that a privacy correction occurred; this changes the strict append-only rule.
`````

### User answer

`````text
Separate history from personal data (Recommended)
`````
