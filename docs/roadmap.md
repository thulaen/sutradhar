# Roadmap to version 1

This is the only roadmap. It lists the phases, what each must deliver and
how we know each is finished. It never states status: that is in
Forgejo, one milestone per phase. How the roadmap may change is in
[AGENTS.md section 12](../AGENTS.md#12-docs), and how a phase is closed
is in [AGENTS.md section 10](../AGENTS.md#10-keeping-the-backlog-small).

Phases run in order. A phase starts when the one before it is closed.
"Every allowed language" below means every allowed language that is
switched on, as in [AGENTS.md section 7](../AGENTS.md#7-the-gate).

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
7. The heavy-job queue with its systemd limits, with tests that a third
   job waits and that a timed benchmark holds both places.
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

1. Bring the useful gate code over from the archive, one part at a time.
   Each part must pass every check already built and the required reviews.
2. Fill `gate/catalogue.toml` with every check and every gate item, with
   no repeats, each with a planted-fault test per allowed language.
3. Pin every tool in `gate/tools.toml`.
4. Write the Google-style guides and the rule matrix in
   `docs/standards/`.
5. Build the MC/DC tool for Rust and prove it on hand-worked cases.
6. Build GanitEvolve and its receipts, from its specification.
7. Build the benchmark runner, its baselines and the slowdown check.
8. Build the signing core of tijori, used by review records, waivers,
   signed runs and the site link.
9. Build the checks that guard the project itself:
   - allowed languages and file types only;
   - one rule file and one roadmap;
   - no status statements in docs;
   - the commit message form;
   - GanitEvolve receipts;
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
only, since its long tests come in Phase 5); GitHub CI gives the same
results; and the required reviews of the phase pass.

## Phase 3: First slice

A slice is a thin piece of the product that works from start to end.

Forgejo milestone: `Phase 3: First slice`.

1. Sutradhar Boot: composition root, typed settings and profiles,
   lifecycle, health, one error shape, logs, metrics and audit.
2. The database starter: PostgreSQL, migrations, the forced RLS check
   and the app role.
3. SDS: the first accessible components, tokens and the component
   catalogue.
4. The Dioxus desktop window.
5. The management API on the local socket, with Cedar permissions.
6. The background service under systemd, in one systemd slice with
   PostgreSQL and the memory cap, with durable jobs and the outbox.
7. Tests for start-up, wiring failures, bad settings, crashes, restarts,
   unfinished jobs and closing the window.
8. Move the gate, `forge-sync` and the signing core of tijori onto
   Sutradhar Boot.

Finished when: items 1 to 8 are met, the slice passes every gate, a
recorded demonstration shows it working, and the required reviews of the
phase pass.

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
recorded demonstration shows both working on the test sites, and the six
reviews of the phase pass.

## Phase 5: The rest of the core

Forgejo milestone: `Phase 5: Core`.

1. Install, update, repair, backup, restore and removal from the app, and
   the `.deb`, with tests for interrupted updates.
2. Self-healing and recovery.
3. Retention.
4. ganit, darshak, hisaab and the rest of tijori.
5. The `sutra` command and the MCP server.
6. Cloud helpers on the owner's own accounts.
7. The manual, built into the app.
8. Two pinned test machines that match both sizes of the owner's PC:
   one under 16 GB, and one with 16 GB or more.
9. The release-tier peak memory, load and soak tests on those machines.

Finished when: items 1 to 9 are met and pass every gate, each has a
beginner guide in the manual, and the required reviews of the phase pass.

## Phase 6: Feature waves

Forgejo milestone: `Phase 6: Features`.

Build every feature in the feature list, `docs/specs/features.md`, in
this order:

1. The first range in the feature list.
2. The second range.
3. The third range.
4. The fourth range.

Within each range, build the features in four kinds, in this order:
insight-only features (they only show information), then review-page
changes, then drafts (changes prepared for a moderator to approve), then
public-page features.

Finished when: every feature in the list is built, passes every gate and
is shown in a recorded demonstration on both test sites, and the six
reviews of the phase pass.

## Phase 7: Version 1

Forgejo milestone: `Phase 7: Version 1`.

1. The release gate in
   [AGENTS.md section 3](../AGENTS.md#3-the-goal-and-the-release-bar).
2. The public manual on GitHub Pages.
3. Signed release artefacts.

Finished when: items 1 to 3 are met, the required reviews of the phase pass,
the owner approves, and `v1.0.0` is tagged in Forgejo with the same tag on
GitHub.

## Later

Forgejo milestone: `Later`. This holds ideas not placed in a phase.
