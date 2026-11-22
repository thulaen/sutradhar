# Roadmap to version 1

This is the only roadmap. It lists the phases, what each must deliver and
how we know each is finished. It never states status: that is in
Forgejo, one milestone per phase. How the roadmap may change is in
[AGENTS.md section 12](../AGENTS.md#12-docs), and how a phase is closed
is in [AGENTS.md section 10](../AGENTS.md#10-keeping-the-backlog-small).

Phases run in order. A phase starts when the one before it is closed.
"Every allowed language" below means every allowed language that is
switched on, as in [AGENTS.md section 7](../AGENTS.md#7-the-gate).

Before any implementation resumes, every Version 1 feature, declared
variant, interface, operational procedure and test design must satisfy
[spec 0076](specs/0076-specification-completeness.md). Every feature must
have one owner in the [feature register](specs/feature-register.md).
A range task cannot supply design detail during implementation. Documentation
completion does not authorise coding by itself: the phase order and all
other gates still apply.

The [implementation catalogue](planning/implementation-catalogue.md)
and [test traceability](planning/test-and-traceability.md) map phases
to scoped work and evidence.

## Phase 0: Save and settle

Forgejo milestone: `Phase 0: Save and settle`.

1. Save all earlier work in a private, read-only archive on Dell.
2. Write the decision records, this roadmap and AGENTS.md.
3. Install and pin the GitHub command-line tool on Dell, and sign it in.
4. Set the Forgejo owner password and keep it in the private notes on
   Dell.

Finished when: items 1 to 4 are met and the required reviews of the phase
pass. This phase's proof is kept on its milestone once Phase 1 item 6
creates it.

## Phase 1: Forgejo home

Forgejo milestone: `Phase 1: Forgejo home`.

1. Create the repository `sutradhar-owner/sutradhar` and push `main`.
2. Turn on Forgejo Actions with a pinned runner on Dell.
3. Allow only `main`. Refuse other branches, force pushes and deletes.
4. A door gate on Forgejo's side runs the push tier before it accepts a
   push, and refuses the push if any check fails.
5. Add the checksum guard on Forgejo's hooks and settings, the protected
   gate files and the GitHub CI files.
6. Create the milestones for every phase and for "Later", and the
   `status/ready`, `status/working`, `status/blocked` and
   `status/urgent` labels.
7. The [Dell heavy-job queue](specs/0059-pankti.md) with its resource
   limits, with tests that a third job waits and that a timed
   benchmark holds both places.
8. Carry over earlier work:
   1. Rewrite every earlier task and idea as a task in this repository.
      Each is placed as
      [AGENTS.md section 12](../AGENTS.md#12-docs) sets out.
   2. Keep a carry-over list task that names each earlier item and
      where it went, with no clashes, repeats or overlaps.
   3. Bring every earlier specification over into `docs/specs/`,
      rewritten for this repository, including the feature list in
      `docs/specs/features.md`.
9. Build `forge-sync` as described in
   [AGENTS.md section 9.1](../AGENTS.md#91-the-sync).
10. Turn on GitHub's private vulnerability reporting.
11. Owner step: make a fine-grained GitHub token for this repository only,
    for the sync.

Finished when: items 1 to 11 are met; a push that fails a check is
refused, even with hooks skipped; a push that passes lands on `main`;
GitHub matches Forgejo; and the required reviews of the phase pass.

## Phase 2: The gate at 100%

Forgejo milestone: `Phase 2: The gate`.

First, before the numbered items: build Ganit Vikas's build steps 1
and 2 from spec 0012 (part of item 6), so later work can get receipts
(owner, 11 November 2026).

1. Bring the useful gate code over from the archive, one part at a time.
   Each part must pass every check already built and the required reviews.
2. Fill `gate/catalogue.toml` with every check and every gate item, with
   no repeats, each with a planted-fault test per allowed language.
3. Pin every tool in `gate/tools.toml`.
4. Write the Google-style guides and the rule matrix in
   `docs/standards/`.
5. Build the MC/DC and mutation-analysis tools needed by the strict core
   in ADR-0020 and the risk-appropriate coverage checks for other
   components. Prove each on hand-worked and planted cases, including
   equivalent-mutant review evidence.
6. Build Ganit Vikas and its receipts, from its specification.
7. Build the benchmark runner, its baselines and the slowdown check.
8. Build the signing core of tijori, used by review records, waivers,
   signed runs and the site link.
9. Build the checks that guard the project itself:
   - allowed languages and file types only;
   - one rule file and one roadmap;
   - no status statements in docs;
   - the commit message form;
   - Ganit Vikas receipts;
   - the required review records;
   - waivers signed only with the owner's key;
   - the backlog limits.
10. Git hooks in Rust that run the commit tier on commit and fail when a
    tool is missing.
11. GitHub CI that re-runs the full gate.
12. Owner step: make the owner's signing key on the owner's PC, and commit
    its public key as `gate/owner-key.pub`.

Finished when: items 1 to 12 are met; a signed full run shows every
check that exists refusing its planted fault and passing clean code, in
every allowed language, at every tier (the release tier on planted faults
only, since framework long tests come in Phase 3B and product long
tests in Phase 5); GitHub CI gives the same
results; and the required reviews of the phase pass.

## Phase 3: Foundation and first product slice

Forgejo milestone: `Phase 3: First slice`. Close stages A, B and C
in order. The milestone closes after stage C. Follow
[ADR-0025](adr/0025-refactor-and-boot-first.md) and
[spec 0139](specs/0139-full-foundation-refactor.md).

### 3A. Full foundation refactor

After Phase 2, inventory the exact existing foundation. Complete each
approved refactor or migration and verify compatibility and recovery.
Accept the refactor only when the complete 0139 inventory, gate and
review evidence passes. This exit admits framework implementation.

### 3B. Build and qualify all of Behemoth Boot

Implement the complete verified scope in
[spec 0081](specs/0081-behemoth-boot-parity.md) and its framework groups.
Use framework fixtures. Close this stage only with signed evidence for
the exact complete parity ledger, platform matrix and Boot artefacts.
Partial parity leaves product implementation held.

### 3C. First product slice

Start downstream Sutradhar implementation after 3B. The item numbers
below are the existing Phase 3 item references in the owning specs.

1. Wire the qualified Behemoth Boot components into the first product
   service: composition root, typed settings and profiles, lifecycle,
   health, one error shape, logs, metrics and audit.
2. Wire the qualified database starter into the product. Check
   PostgreSQL, migrations, forced RLS and the app role.
3. SDS: the first accessible components, tokens and the component
   catalogue.
4. The Tauri desktop window with Angular and SDS.
5. The Cedar-checked management API on the owner-only Unix socket
   or Windows named pipe.
6. The supervisor starts the background service in an admitted
   resource profile. Spec 0019 owns the PostgreSQL queue, Rust
   scheduler and transactional outbox. Each job class has one
   durable execution path.
7. Tests for start-up, wiring failures, bad settings, crashes, restarts,
   unfinished jobs and closing the window.
8. Move the signing core of tijori onto Behemoth Boot. The gate and
   `forge-sync` are operations tools written in Rust under ADR-0019
   (AGENTS.md section 4) and stay off Boot.

Finished when: 3A and 3B have their exact signed exit evidence,
3C items 1 to 8 are met, the product slice passes every gate, a
recorded demonstration shows it working, and the required reviews
of the phase pass.

## Phase 4: XenForo, then WordPress

Forgejo milestone: `Phase 4: Site slices`.

1. The signed message contract between the service and the add-ons.
2. Test sites in containers on Dell, with made-up data, and the
   footprint test harness that measures queries, time and bytes per page.
3. The XenForo add-on: first read, suggest, review in the admin pages,
   accept and undo.
4. The same for the WordPress plugin, tested with Wordfence (a common
   WordPress security plugin) and a cache plugin active.

Finished when: items 1 to 4 are met, the footprint tests pass, a
recorded demonstration shows both working on the test sites, and the
required reviews of the phase pass.

## Phase 5: The rest of the core

Forgejo milestone: `Phase 5: Core`.

1. Install, update, repair, backup, restore and removal from the app, and
   the Linux and Windows packages in spec 0030, with tests for
   interrupted updates.
2. Self-healing and recovery.
3. Retention.
4. ganit, darshak, hisaab and the rest of tijori.
5. The `sutra` command and the MCP server.
6. Cloud helpers on the owner's own accounts.
7. The manual, built into the app.
8. Pinned test machines for every declared supported owner-PC profile,
   with exact hardware, operating-system and resource identities recorded.
9. Release-tier peak memory, p95 latency, load and soak tests on those
   machines, proving the admitted total and workload limits under
   AGENTS.md section 5.6 and spec 0078.

Finished when: items 1 to 9 are met and pass every gate, each has a
beginner guide in the manual, and the required reviews of the phase pass.

## Phase 6: Feature waves

Forgejo milestone: `Phase 6: Feature waves`.

Build spec 0064's version 1 scope and the features assigned to it in
the [feature register](specs/feature-register.md). Excluded and deferred
features follow their owning decisions. Keep the existing feature
range order below.

1. The first range in the feature list.
2. The second range.
3. The third range.
4. The fourth range.

Within each range, build the features in four kinds, in this order:
insight-only features (they only show information), then review-page
changes, then drafts (changes prepared for a moderator to approve), then
public-page features. A feature cannot become a ready build task until its
complete spec 0076 readiness record exists. The range order never
authorises an implementer to invent missing behaviour, variants, errors or
tests.

Finished when: every feature in the list and the expanded scope in
spec 0064 has feature-specific unit, BDD, integration, end-to-end,
edge-case and operational evidence and passes every applicable gate on
both test sites. Recorded demonstrations show each declared supported
variant, and the required reviews of the phase pass.

## Phase 7: Version 1

Forgejo milestone: `Phase 7: Version 1`.

1. The release gate in
   [AGENTS.md section 3](../AGENTS.md#3-the-goal-and-the-release-bar),
   with the qualification proof in spec 0064.
2. The public manual on GitHub Pages.
3. Signed release artefacts.

Finished when: items 1 to 3 are met, the required reviews of the phase pass,
the release approval under AGENTS.md section 3 is recorded, and
`v1.0.0` is tagged in Forgejo and mirrored to GitHub.

## Later

Forgejo milestone: `Later`. This holds ideas not placed in a phase.
