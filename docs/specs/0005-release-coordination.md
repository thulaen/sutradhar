# 0005: Agent and release coordination

This part lets agents work together unattended while the owner's
protected decisions, private data and release authority stay enforced.
It adds only what is not already in
[AGENTS.md section 8](../../AGENTS.md#8-how-a-session-works) (the
session steps and file locks) and
[section 8.1](../../AGENTS.md#81-releases) (the release task), and sets
out the release steps in full. It is roadmap Phase 7 item 3.

## What it does

### Who holds which power

1. Writing code, recording check results, recording owner approval and
   publishing a release are separate powers, each held by its own
   identity on Dell.
2. Private keys and tokens stay with the protected services on Dell,
   outside any candidate checkout. Coding agents receive none of them,
   and no ambient owner credential.
3. The protected services expose no general Forgejo or GitHub access,
   no shell and no free reference update. No coding identity can make a
   tag, write `main` other than by an ordinary push through the door,
   weaken a workflow, or publish through an existing tag.
4. Check results are accepted only from qualified runners under the
   current policy. Owner approval is accepted only as a signature made
   with the owner's key over the exact change and its version
   ([AGENTS.md section 7.2](../../AGENTS.md#72-waivers)). A label, a
   self-written file, a forged report or a comment is never approval.

### Joining, leases and handover

1. Agents join explicitly. Mailbox messages are transport, never leave
   to edit another agent's files or approve its changes.
2. A private run ledger on Dell records each agent's identity, its task,
   the approval of the specification it works under, its current lease
   and its activity. It also records runs, attempts, file ownership,
   evidence references, publication state and handovers. It never holds
   a copy of a credential. Only scrubbed summaries leave Dell.
3. One writer per file. A clashing lock request is refused or queued.
4. A handover keeps the draft and its checkpoints, records the reason,
   revokes the old lease and issues a new one. A stale worker is shut
   out of later writes, even if it wakes after its replacement starts.
5. No agent discards another worker's staged or uncommitted work to
   make its own task easier.
6. Writes use durable transactions. Before a half-finished write is
   resumed, a recovery check runs. A retry never makes a second commit,
   tag, release or manual deployment.

### When Dell is not available

1. Work starts, and pushes are admitted, only after Dell is shown to be
   available now. An old green result is not proof.
2. While Dell is unavailable, all ordinary project work pauses. A health
   check may notice the return and report state, but does no project
   work during the pause.
3. Queued work and durable progress are kept.
4. A needed service being down pauses the manual's deadlines and leaves
   release checks unfinished.
5. The availability monitor, lease expiry, job pause and scheduled jobs
   are all tested before the pause rule is claimed to be enforced.

### Reviews

The six reviews of AGENTS.md section 7.1 must each name the current
candidate and finish. A reaction, silence, a stale review, a partial
scope or a skipped job is never a pass. Self-review is labelled as such.

### A broken `main`

If `main` breaks, the first response is a revert commit, pushed through
the door like any other commit, never an unrelated fix forward.

### Dependency updates

1. A weekly dependency task on Dell updates the locked versions, grouped
   as Rust crates, PHP libraries and pinned tools.
2. Each group runs the full tiers, including the speed check's reuse and
   invalidation rules (0007). Any new dependency is admitted through the
   licence rules of AGENTS.md section 5.7.
3. Each group lands as one commit through the normal session steps.
4. A group that fails any check is not pushed, and autoissues turns its
   findings into Forgejo tasks.
5. Security findings from GitHub's dependency alerts, cargo-audit,
   `composer audit` and the container image scanner feed autoissues. A
   finding of the two highest severities becomes an urgent task at once.
6. No dependency step makes a branch or a pull request.

## The release steps

A release is a Forgejo task, as AGENTS.md section 8.1 sets out. Its run
goes as follows.

1. **Scope check.** The release starts only when `main` passes and the
   release bar of AGENTS.md section 3 allows a candidate. If the scope
   or the counts of qualified methods, capabilities or variants fall
   short, the step reports why and stops, even when every other check
   passes.
2. **Version commit.** On a clean Dell copy of `main`, the release step
   makes one version commit holding the new version and the changelog,
   both from the commit messages. It passes the commit and push tiers
   like any commit.
3. **Candidate tag.** The release step makes a candidate tag (for
   example `v1.0.0-rc.1`) on that exact commit and starts a durable
   release run. Only the release step may make tags; the door refuses
   any other tag push (0001).
4. **Release run.** The run carries out:
   - the full quality assessment on fixed and fresh data;
   - the resource and environment checks, including the peak memory,
     load and soak tests on the test machine that matches the owner's
     PC;
   - a 72-hour supervised soak.
   It resumes across provider usage windows. It records the exact
   content, model, scoring guide and data identities. It never reruns or
   publishes a changed candidate on old evidence.
5. **Quality bars.** The run requires no fall in reconstruction recall
   or anchor agreement, and requires the hard-rule and coverage checks
   to pass. After launch it also includes the review-page acceptance
   rate over the latest 200 decisions. That rate's definition and its
   bar are set in the evaluation specification (0057); the check never
   makes up a bar and never passes while that bar is missing.
6. **Nightly runs.** Fixed nightly runs keep reporting reconstruction
   recall, anchor agreement, hard rules and coverage, without AI judges.
7. **Final release.** After success, the release step makes the final
   tag and the release for the same commit as the passing candidate. The
   release stays a draft until every check has passed. It signs and
   publishes the checked artefacts and moves the matching manual version
   and `latest` (0004). The sync copies the tag and release to GitHub.
8. **Same code.** Any packaging or version change that alters the
   tested artefact needs the same checks again. A changed commit needs a
   new candidate.
9. **Failed candidates.** A failed candidate is kept, with a scrubbed
   summary of why it failed. No final tag, product release or manual
   promotion follows.
10. **Recovery.** If the release step crashes after making an artefact,
    recovery checks what exists and finishes once, never making a second
    release.

## Limits

- Release-tier memory, load and soak tests run on test machines that
  match both sizes of owner's PC, within the memory limits in AGENTS.md
  section 5.6.
- The soak lasts 72 hours.
- Dell heavy jobs, including dependency group runs, follow AGENTS.md
  section 11.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Rust integration tests on the coordination and release crates, with
scratch Forgejo repositories:

- Two agents ask for the same file: only one gets a valid lease.
- A stopped agent returns after handover: its stale lease cannot write.
- A worker asks for a `main` update outside the door, a tag, a release
  or a free endpoint: it is refused, and no credential is shown.
- A worker fakes a passing check or an owner approval: it is refused,
  and the attempt is recorded privately.
- A protected change stays blocked until it carries the owner's
  signature.
- A cached availability result is stale, or Dell disappears: new work
  and admission pause, and the true state is reported.
- The release step refuses to start while scope is short, and says why.
- The version commit passes the same tiers as any commit.
- Only the release step can make candidate and final tags; an agent's
  tag push is refused.
- The final tag names the same commit as the passing candidate; a
  changed commit needs a new candidate.
- A candidate that lacks scope or fails judging or the soak gets no
  final tag, product release or manual promotion.
- The release step crashes after making an artefact: recovery finishes
  once and makes no duplicate.
- A dependency group that fails a check is not pushed, and its findings
  become Forgejo tasks.
- No release or dependency step makes a branch or a pull request.

## Changes from the source

- GitHub App identities and a publisher broker are replaced by Forgejo,
  its door and the sync; agents push only through the door.
- Pull requests, squash merges, auto-merge and fork checks are gone;
  outside work follows AGENTS.md section 9.2.
- Owner approval is the owner's key signature, not a GitHub comment.
- The GitHub AI review is replaced by the six reviews.
- Releases are made only when the roadmap names one or the owner asks,
  not weekly.
- The release bar is the one in AGENTS.md section 3, in place of the
  older 6,000-method bar.
- Version and changelog tools are replaced by the release step working
  from commit messages.
- Dependency bots are replaced by the weekly Dell task; the old audit
  tool for another language is dropped.
- The speed check is the 5% median rule of AGENTS.md section 6, not the
  old 20x gate.
- High-severity security findings become urgent Forgejo tasks.

Source: `docs/specifications/agent-release-coordination.md` and
`docs/specifications/release-main-only.md` in the archive
`archive-2026-11-08-old-repo`.
