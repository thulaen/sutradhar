# 0011: Engineering standard for the gate

This part brings the practices of Google's engineering system into
aegis, the gate: testing at head, one findings platform, ownership and
review, identical builds, code search, change splitting, continuous
fuzzing, test sizes and an agent workflow. It adds no second way to pass
and loosens nothing. It is roadmap Phase 2 items 2 and 4 (the check
catalogue and the Google-style standards).

## What it does

### Rules that hold throughout

1. Every check in `gate/catalogue.toml` still blocks at the commit, push
   and release tiers. Nothing here skips, samples or sets aside a check.
2. Every defect is fixed on sight with a failing regression test, and
   every commit carries a `Defects:` line.
3. Owner records stay word for word, and locked evidence is untouched.
4. All of this is Rust, run inside aegis in pinned container images,
   within the heavy-job limits, and every result is evidence aegis
   signs.
5. Every threshold lives in the gate's TOML settings, never in code.

### GG-01 Testing at head

After every commit lands on `main`, and every night, aegis runs every
check again on the tip of `main` in a clean pinned image. A red head
raises an urgent task and stops further commits except the fix. Results
feed the code health dashboard (GG-10).

### GG-02 Culprit finder

When head turns red, aegis bisects the commits since the last green
run, re-running only the failing checks, until it names the first bad
commit. It then files the task against that commit with the failing
output. Bisection runs as ordinary heavy jobs; its result is signed
evidence.

### GG-03 Flaky tests, never set aside

Each failing test is run again a set number of times in the same job to
class it as fixed or flaky. ganit's flaky-test probability score rates
it. A flaky test is a defect, never skipped or set aside: the run stays
failed and the defect is fixed on sight. The class only speeds up
finding the cause.

### GG-04 One findings platform

1. Every analyser (Clippy, nirikshak, PHPStan, squawk, ShellCheck, the
   private-data checks and the language checks) reports findings in one
   shape: place, rule, message, severity and, where possible, a fix a
   machine can apply.
2. Agents and reviewers see findings as comments on the change.
3. A "not useful" mark records disagreement. Aegis tracks each rule's
   not-useful rate. A rule above the set limit is itself a defect for
   its owner to fix, never quietly turned off.

### GG-05 Owners and review

1. An `OWNERS` file in each folder names its owners: people or agent
   roles.
2. A change needs a reviewer who is not its author, approval from an
   owner of every folder it touches, and, for each language it touches,
   a readability approver for that language.
3. Gate, policy and owner-record folders name the owner alone.
4. Approvals bind to the exact tree and are signed. A later edit voids
   them.
5. Review comments may carry suggested edits that the author applies
   with one command.
6. First owners: the owner for gate, policy and owner records; the
   coordinator role for everything else until named owners exist.
7. A readability approver is an agent that passed a readability
   exercise for that language, checked once by the owner.
8. This sits beside the required reviews of AGENTS.md section 7.1; it
   replaces none of them.

### GG-06 Identical builds

1. Every release artefact and image is built twice from the same commit
   in fresh containers, with the network off after dependencies are
   fetched. The two outputs must match byte for byte.
2. Aegis records how each was built (source tree, image digests, tool
   versions, flags, environment) as signed provenance in the style of
   SLSA build provenance.
3. A mismatch is an urgent defect naming the differing file.

### GG-07 Code search

1. nirikshak's versioned facts become a searchable index of
   definitions, references, callers and implementations across Rust,
   TypeScript, PHP and SQL.
2. `aegis search` answers "where is this defined, who calls it, what
   implements it".
3. Before adding a new function, type or module, an agent searches for
   an existing one, and the change records the search. The duplication
   check still blocks.

### GG-08 Large changes split

`aegis split` takes a large mechanical change (a rename, an API move, a
code rewrite by rule) and cuts it into the smallest independent commits,
grouped by `OWNERS`. Each is judged by the full gate and reviewed by
that area's owner. A part that fails holds back only itself.

### GG-09 Continuous fuzzing

Every fuzz target runs nightly within its time share. New crashes are
grouped by stack signature, cut down to the smallest failing input,
filed as tasks with the input attached, and bisected with GG-02. When a
fix lands, the crashing input joins the regression set and aegis
confirms it no longer fails.

### GG-10 Test sizes and code health

1. Every test declares a size:
   - small: one process, no network, no disk outside its temporary
     folder;
   - medium: one machine, local services only;
   - large: everything else, scheduled.
2. Aegis enforces each size's limits and fails a test that goes over
   its size. A test with no size is refused.
3. A code health dashboard shows head status, flake scores, findings and
   their not-useful rates, coverage, mutant scores, build
   reproducibility and the time from red head to green.

### GG-11 Agent helpers that only propose

Three helpers, each run by the agent that started it and on that
agent's own usage:

- a build-repair helper that proposes a fix for a build or lint failure;
- a review helper that turns a review comment into a suggested edit;
- a test helper that proposes missing tests for uncovered lines.

Each output is an ordinary change, judged by the gate and reviewed.
Nothing is applied without review.

### GG-12 Agent workflow

1. For a task over the plan threshold, the agent first writes a plan:
   goal, files, tests and risks. The owner or a reviewer may comment on
   it.
2. The agent then keeps a task list and ends with a walkthrough: what
   changed, the evidence, screenshots or recordings of the Dioxus app
   for screen work, and the `Defects:` line.
3. Plans and walkthroughs are posted on the Forgejo task. They are
   evidence, never approval.
4. Lasting lessons (for example, "the XenForo test forum needs
   development mode for commands") become knowledge items under
   `docs/knowledge/`, each with its source and date, read at the start
   of later tasks.

### Build order

1. GG-04 and GG-05 first, since the findings shape and `OWNERS` shape
   every later output and approval.
2. GG-10, then GG-01 and GG-02, then GG-03.
3. GG-06.
4. GG-07, then GG-08.
5. GG-09.
6. GG-11 and GG-12.

Each lands as its own test-first commits, judged by aegis as it stands
just before. Before building, check which existing checks already cover
part of GG-01 to GG-12, and merge into them rather than building twice.

### Settings and defaults

| Setting | Default |
| --- | --- |
| Run at head | after every commit and nightly |
| Flake re-runs | 5, in the same job |
| Not-useful limit per analyser rule | 10% over the last 100 marks |
| Small test limits | 1 process, no network, 60 s, 256 MiB |
| Medium test limits | local services only, 300 s, 1 GiB |
| Large test limits | scheduled, within the job place, 3.5 GiB, no swap |
| Fuzzing share each night | 60 minutes, shared across fuzz targets |
| Plan threshold | more than 3 files or 150 lines |
| Dashboard retention | 7 days of detail |

Every value has a default, a source and a test.

## Limits

- Every job runs within the Dell heavy-job limits: at most two at once,
  each held to 4 GB and 8 CPU threads, no swap (AGENTS.md section 11).
- Test sizes and the nightly fuzzing share are in the table above.
- Dashboard detail is kept for 7 days.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Each scenario becomes a failing Rust test before the code that passes
it: unit, property, integration, failure-injection and regression tests.

- Head: a commit lands and the run after it fails a check that passed
  on its parent; head is red, an urgent task names the check, and the
  next commit is refused unless marked as the fix.
- Culprit: five commits since the last green head, the third broke a
  test; the finder names the third after at most three bisection steps
  and attaches the output.
- Culprit recovery: the finder's job is killed half-way; it resumes
  from its checkpoint and reaches the same answer.
- Flakes: a test fails once and passes four re-runs; the job still
  fails, the test is labelled flaky with its rate, and a defect is filed
  for its owner. A test that fails all five times is labelled fixed and
  gets no flake label.
- Findings: Clippy, nirikshak and PHPStan each report one finding; all
  three appear in one shape with place, rule, severity and any fix.
- A rule with 11 not-useful marks in its last 100 findings gets a defect
  filed against its owner and stays on.
- Property: applying a finding's suggested fix never changes text
  outside the reported range.
- Owners: a change touching `crates/` and `docs/adr/` with only the
  `crates/` owner's approval is refused, naming the missing owner.
- An author approving their own change does not count.
- An approved change edited afterwards voids every approval.
- A gate policy change approved by anyone but the owner is refused.
- Builds: two clean builds of one commit give the same SHA-256 for every
  artefact and a signed provenance record listing tree, image digests,
  tools and flags. A build that embeds the build time is an urgent
  defect naming the differing file.
- Search: a Rust function called from the PHP add-on through the API
  and from TypeScript interop is found with all its definitions,
  references and callers.
- A new function whose body copies an existing one is refused by the
  duplication check, naming the existing function and the search that
  should have found it.
- Splitting: a rename across folders of three owners gives three
  commits, each judged by the full gate and each needing only its own
  owner's approval.
- Fuzzing: two crashes with the same stack signature give one task with
  both inputs, the smaller first. After a fix, the crashing input joins
  the regression set and the next run passes.
- Sizes: a small test that opens a network socket fails with "small test
  used the network"; a test with no size is refused.
- Helpers: a build error gives a proposed change only; nothing is
  applied until it passes the gate and review.
- Workflow: a task changing four files with no plan has its first commit
  refused with "plan required".
- A finished screen task's walkthrough holds app screenshots, the
  evidence links and a `Defects:` line.
- A knowledge item without a source or date is refused by the docs
  check.
- Byte-identical builds are shown for the Rust binaries, the `.deb` and
  the add-on zips, or the exact sources of difference are found and
  removed.

## Changes from the source

- Every analyser, search index and build covers only the allowed
  languages; analysers and maths once written in other languages are
  rewritten as Rust (nirikshak, ganit).
- Thresholds live in the gate's TOML settings.
- "P0 AutoIssue" becomes an urgent Forgejo task raised by autoissues.
- The heavy-job limits follow AGENTS.md section 11, not the old three
  jobs sharing 12 GB.
- Plans and walkthroughs go on the Forgejo task, not into `docs/work/`,
  to keep status out of the docs.
- Screenshots come from the Dioxus app, not from a browser test tool.
- The fixed count of 47 checks is replaced by the catalogue.
- Owner and readability approvals sit beside the required reviews.

Source: `docs/specifications/coding-harness/google-grade.md` and
`docs/specifications/coding-harness/google-grade-2.md` in the archive
`archive-2026-11-08-old-repo`.
