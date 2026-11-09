# 0003: Toolchain and check runner

The check runner takes one exact version of the code, runs every check
that applies to it in a locked-down box, and signs the result. It is
the one trustworthy way to say whether a change may be committed or
pushed, and why. It is part of aegis, the gate, and is roadmap Phase 2
items 1 and 3. The tiers it serves are in
[AGENTS.md section 7](../../AGENTS.md#7-the-gate).

## What it does

### Entry points and profiles

1. Cargo commands give the entry points: check, test, lint, type check,
   evaluation, run and database. The normal check command runs the full
   profile of checks for the tier. Single development commands help
   find a fault but never stand in for that profile.
2. The runner reports the profile it chose and every check that the
   profile requires, with a plain reason for any check that does not
   apply.
3. The documentation-only profile is chosen only when something changed
   and every changed path is documentation with no executable bit,
   before or after. It still runs source capture, links, consistency,
   privacy, secrets, rights and review checks. Any executable source or
   workflow change ends it, and the runner picks the code profile.
4. Quoted historical source requests are data. The runner never runs
   their contents.

### Evidence and timing

1. Checks run before a commit exists. The evidence binds the tested
   file and content list, the settings, the dependencies and the
   environment. The commit must match that list; any difference voids
   the result, whatever the file names.
2. The push tier runs again at Forgejo's door and checks that the
   evidence still matches.
3. Release judging and the 72-hour soak belong to the release tier
   only. Ordinary checks never pretend to have run them.

### Results

1. A structured result holds: the candidate and content identity, the
   profile and policy version, the environment, the dependency lock,
   each check's name, outcome and reason, timing, review provenance and
   evidence references.
2. Outcomes are PASSED, FAILED, BLOCKED or NOT_APPLICABLE. An absent or
   skipped result never becomes PASSED. A worker crash or partial
   report never counts as success.
3. Exit codes: 0 PASSED, 1 FAILED, 2 BLOCKED.
4. Bundles are canonical JSON signed with OpenSSH under the namespace
   `sutradhar-evidence`. `verify` rechecks the signature, tree, base,
   profile, plan, pins and maximum age, and works out the status again.
5. Private raw output stays on Dell. Only scrubbed summaries leave it.

### Who may decide

1. One set of check definitions is used on Dell and in GitHub CI. No
   recipe holds its own copy of the policy. AGENTS.md holds the rules;
   this part holds the runner's behaviour.
2. Candidate code and ordinary workers cannot mint a trusted pass or an
   owner approval.
3. The policy is read from a protected copy, never from the candidate,
   and is loaded strictly.
4. The trusted runner is a pinned Rust build whose hash is kept in the
   protected policy. A candidate that changes the runner still cannot
   judge itself.
5. A change to the runner, the gate policy or the workflow files is a
   protected change under
   [AGENTS.md section 7.2](../../AGENTS.md#72-waivers), judged by the
   runner already accepted.

### The locked-down box

1. The candidate is exported blob by blob. Links, submodules, `.git`
   entries, unsafe or non-UTF-8 names, corrupt objects and oversize
   exports are refused.
2. The parent tree goes only to checks that name it, read-only, and is
   checked again afterwards.
3. Each pinned tool is checked, copied, checked, bound read-only and
   checked once more after the checks.
4. Each check runs as one container job with no network, no devices or
   capabilities, private temporary folders in memory, hidden host
   folders, and limits on memory, processes and time.
5. The secret scan never reads settings or ignore files that the
   candidate supplies.
6. A failure of the box itself is BLOCKED, never FAILED.
7. The refusal probe passes only when every forbidden attempt meets a
   real operating-system refusal.

### Tools

1. Tests run under cargo-nextest, with random order, parallel workers
   and timeouts. Mutants use cargo-mutants, benchmarks criterion,
   snapshot tests insta, and unused-dependency checks cargo-machete.
   Lints are Clippy with the owner's lint set. Commit message checks and
   changelogs use cocogitto and git-cliff. Secrets use gitleaks.
2. Contract shapes are Rust types with serde and garde. Generated
   shapes are checked against their source and still meet every quality
   and size limit.
3. Module walls are checked by nirikshak's module checks: product code
   never imports lab code, core never imports feature modules, and a
   feature module uses core contracts, never another module's insides.
4. The management API is fuzzed against its published contract.
5. Every tool and its dependencies are pinned by version and hash in
   `gate/tools.toml`, with a licence check. PHP 8.5 support and the
   coverage asked for must be proven for the PHP adapters.

### Where work runs

1. All test, build, dependency and cache work runs on Dell. The owner's
   editing computer runs none of it.
2. Public, made-up checks may also run in GitHub CI. Private data,
   licensed forum code and final speed measurements stay on Dell.
3. All work pauses when Dell cannot be confirmed as available.

## Limits

- A missing tool, an unknown check, a bad report, expired evidence,
  changed content, a missing private input or a failed privacy scan
  gives FAILED or BLOCKED, never a smaller passing profile. The report
  names the failing check in plain English, its code and the next step.
- A timeout stops the work safely and keeps enough private evidence to
  find the cause.
- Parallel test workers share the declared resource budget: at most two
  heavy jobs at once on Dell, each held to 4 GB and 8 CPU threads.
- Tests may not change tracked source in the candidate under review.
- Targets, measured on the real workload and never claimed from an
  empty repository: fast push checks under 5 minutes; the private
  development evaluation under 10 minutes.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Rust integration tests in the aegis crates, run on scratch repositories:

- A documentation-only candidate may report code checks as
  NOT_APPLICABLE with proof; secrets, privacy, consistency and reviews
  still run.
- An executable file added to that candidate makes the runner refuse
  the documentation-only profile and choose the code profile.
- Source changed after the checks makes admission refuse the stale
  evidence, even with the same file names.
- A forged, absent, truncated or skipped check result never yields a
  passing status.
- A product-to-lab import, a core-to-module import or a module-to-module
  import of insides fails the module check.
- Bad API data, or a shape that no longer matches its source, fails the
  contract check without sending private data anywhere.
- A secret or an identifying forum detail in a source file, artefact or
  log fails the public-output check and the output is held back.
- With parallel workers, random order, timeouts and mutation failures,
  the result stays reproducible and names the exact failing evidence.
- A development subset passing while a required check fails leaves the
  overall result failed.
- The runner's own failure paths are covered, at the coverage AGENTS.md
  section 6 sets.
- A deliberately broken candidate is refused by every check it should
  fail.
- The refusal probe passes on the container box.

## Changes from the source

- The runner is rewritten as Rust crates in the Cargo workspace.
- The task runner file is replaced by Cargo commands.
- Each test, mutation, benchmark, snapshot and lint tool is replaced by
  its Rust tool named above.
- Generated data models are replaced by Rust types with serde and garde.
- The notebook lab is replaced by Rust lab runs with recorded inputs
  and outputs.
- The import-contract tool is replaced by nirikshak's module checks, and
  the API fuzz tool by the management API fuzz harness.
- Checks run as container jobs, not systemd transient services.
- The trusted runner is a pinned Rust build, not source copied from the
  parent commit.
- GitHub statuses, proposal bots and pull-request reviews are gone:
  Forgejo's door runs the push tier, and the required reviews of AGENTS.md
  section 7.1 block like every other check.
- Release and dependency proposal bots are dropped; their work happens
  on Dell, as 0005 describes.
- The heavy-job limit follows AGENTS.md section 11.

Source: `docs/specifications/bootstrap-toolchain.md` and
`docs/specifications/bootstrap-toolchain-rust.md` in the archive
`archive-2026-11-08-old-repo`.
