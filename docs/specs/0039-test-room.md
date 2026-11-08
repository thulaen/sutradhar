# 0039: Test room

The test room answers one question for an editor: can a simple machine
reader use my pages? Small reader programs try to use a frozen copy of
one site, in a locked area with no network. They look for a fact, name
an author, compare two items or cite the original page, and the room
reports what they could not do. It is advice only. It is roadmap Phase 5
item 8. In code it is the `mirror` module.

## What it does

### Three rules

- It reports only what our own readers did, never what an outside search
  engine or chatbot does.
- The set-up is frozen. Changing it starts a new epoch (a numbered
  version of the set-up). Numbers from different epochs are not compared.
- It reports the symptom (these pages failed). It never claims a cause.

Example: ten readers look for the warranty length in the staff guides.
Three cannot say, because two guides disagree. The report says "3 of 10
test readers could not tell the warranty, on these two pages."

### What it reuses

It reuses the crawler's page copies ([0044](0044-seo-signals-crawler.md)),
the Coach's AI readability advice, contradiction tip and related
questions ([0041](0041-editorial-coach.md),
[0042](0042-coach-additions.md)), Search Console questions
([0040](0040-search-connectors.md)), sealed sandboxes and the controller,
safe running ([0033](0033-safe-running.md)), durable jobs
([0019](0019-events-jobs-recovery.md)), the one progress system
([0016](0016-progress-system.md)), ganit's intervals and comparisons,
GanitEvolve and the lab on made-up sites ([0012](0012-ganit-evolve.md),
[0056](0056-method-lab.md)), row-level security, model admission
([0024](0024-models.md)) and the list of approved metric sources.

It adds no second crawler, rubric, AI list, job system or kind of tip.
New here: the frozen set, reader profiles and epochs, the task library,
the scorer and failure classes, traces, and the seven test families.

### The frozen set

1. A staff member with the permission "Run reader tests" picks one
   registered site (XenForo or WordPress) and presses "Freeze".
2. The set holds guest-visible pages taken from the crawler's page
   copies, as fetched, each with address, copy date, page version and
   canonical address. The room fetches nothing. A page with no copy
   shows "no copy: crawl first". Freezing never starts a crawl.
3. A set is never edited. Its identity is its list of addresses and copy
   fingerprints.
4. It is the room's own sealed copy, with its own expiry
   (`mirror.frozen_keep_days`), since crawler copies expire after
   `seo.crawl.page_copy_days` (30). It counts in the room's disk limit.
5. Guest view beats frozen. A page that leaves guest view is removed from
   every set at the next crawl. Its past results stay as counts only.
   A page is also removed on expiry, site removal and erasure request.
6. Scope is staff pages and guides. Member threads stay out until
   research shows they are safe and useful.
7. A set is never public, never sent out and never used to decide what
   guests may see. Pasted, uploaded and unsaved drafts are not tested
   here; they get the Coach's checks only.

### The locked room

- It runs in its own sandbox built from Linux's own controls under the
  extension isolation rules (spec 0037), started by the controller,
  the one part that does privileged work. No container program is used
  on the owner's PC. The frozen set is read-only. It writes only to its
  own result area. No network, no keys, no extra socket, no host files.
- Zero outside calls is proven, not assumed. A test tries to connect from
  inside and must fail. The network layer counts refused attempts, and a
  reader that tries even once is recorded as a defect.
- Pages and model files are untrusted data. No page script is run.
  Reading a page as a browser draws it needs the crawler's headless
  browser to be admitted; until then that reading shows "not measured".
- The scorer is a separate small program. It cannot load a model
  runtime, and the build refuses if it tries. It receives only typed
  result records.
- Sandboxes share the host's kernel. Research decides whether that is
  enough for untrusted page text and model files. A stronger layer (a
  user-space kernel or a small virtual machine) needs admission.
- The room never uses the Coach's AI order, since that may end at a paid
  service outside the PC. A missing reader model blocks the tasks that
  need it, with a reason, and nothing takes its place.

### Readers

- A reader is a small Rust program with a declared set-up, its profile:
  how it reads (page source or plain text, with or without structure),
  how much it holds at once, how many links deep it goes, how it picks
  passages, how it treats canonical addresses, and its time and memory
  limits.
- Program readers follow fixed rules and use no model. Model readers use
  a small local pure-Rust model that has passed model admission. Until
  one is admitted, model readers show "unavailable". Program readers
  come first.
- Several profiles run on each task, so one reader's habit is not taken
  for the site's fault. The size of a defensible population is set by
  research.
- Readers have plain names, such as "text-only reader, small context".
  No reader is named after an outside engine, and no outside brand shows
  on screens or in records. A profile shaped by a published vendor rule
  records "informed by a published rule", which proves nothing about the
  vendor.
- Reader and scorer are different programs. A reader never marks its
  own work.

### The epoch

An epoch is a fingerprint of the whole set-up: reader programs and
versions, model file fingerprint, prompts, retrieval and context rules,
task definitions, seeds, scoring rules and limits. It leaves out the
frozen set.

- Any change makes a new epoch. Old results stay under the old epoch.
- Allowed comparisons: same epoch, two sets ("did my edit help?"), and
  same set, two epochs (a bridge showing what the new set-up changed).
  Different epochs on different sets are refused.
- Seeds are fixed. Model files are pinned by fingerprint. One model
  family per epoch. No model name appears in product wording or rules.
- A fake deterministic reader pack must run the same tasks, scoring and
  results with no rebuild, which proves a reader can be replaced.

### Tasks

- Kinds: find a fact, locate a named thing, compare two items, name the
  author, cite the source. An editor may add custom tasks.
- Questions come from the Coach's related questions, Search Console
  question queries or an editor. The private-data check runs on each.
  A question never contains its own answer.
- An editor may add an answer key to one task, such as "2 years". It
  belongs to that task only, shows as "editor's key", and is never a fact
  list or reused. It is checked by exact match, for the fixed value types
  of the Coach's contradiction tip. Free-text answers are shown, not
  marked. Without a key, correctness is unknown and never reported.
- A task changed by an editor is a new task version, in a new run.

### Scoring: four measures, no score

The scorer is a fixed rule set. A reader's answer is an observation; a
model never gives a verdict. The same input always gives the same
verdict.

| Measure | Asks |
| --- | --- |
| Answered (with a key) / Had enough evidence (without) | Did readers give the keyed answer, or was the answer on the page? |
| Same source | Did readers use the same passages? Low means pages compete. |
| Right page cited | Did answers cite the original, not a copy or part? |
| Finished | Did readers complete the task, and where did they stop? |

- Each measure is given per task, per page group, per frozen set and per
  epoch. A rate always shows its counts beside it.
- Below `mirror.min_tasks` tasks or readers, it shows "not enough
  evidence", never a rate.
- No single score and no blended "AI readiness" number. Measures are
  never added together or to Search Console or GA4 figures.
- Each measure ships whole or not at all. "Answered", "Had enough
  evidence" and "Finished" come first. "Same source" and "Right page
  cited" show "not available" until their rules pass research.
- Each task repeats `mirror.repeats` times. Repeats that differ by more
  than `mirror.tolerance` are marked "unstable", not hidden.
- Each measure is on the approved metric list with its source, its
  decision and what it is compared with. Room results are one labelled
  source, our own readers only, never added to other sources.

### Failure classes

Each failed task gets one class by fixed rule: wrong page; wrong entity;
page lacks the evidence; cited a copy, not the original; answer not found
in anything read (made up); could not reach the page (navigation trap);
readers disagree. A task that ran out of time or memory is "no result:
room limit", a room fault never counted against the site. Many such
results raise a plain notice.

### Traces

Per task, a run keeps pages visited, passages taken, evidence retrieved,
citations chosen and final answers. Two runs of one epoch can be replayed
side by side, as the draft diff does. Traces expire with their set, are
held per site and never leave the PC. Answers are untrusted text, shown
escaped and never read as instructions.

### What the editor sees

- A failed page shows a "tested" line beside the matching item of the
  Coach's AI readability rubric, for example "answer sits under a vague
  heading. Tested: 3 of 10 readers failed." With no matching item it
  shows "observed, no rubric item" and the trace. The rubric is not
  extended here; a gap goes to research.
- The Coach's helper test (one passage sent to the staff member's own
  helper) is a different instrument. Its results and the room's are never
  merged, and each names its instrument.
- Nothing blocks a draft or changes a page. A repair idea is a proposal
  through the normal staff approval, keeping the original.
- Screens cover: no set yet, running, waiting for memory, stopped by
  pressure, resumed, model unavailable, and "not measured" for any gap.
- Results are readable through the action layer and by agents, who
  cannot start a run. Tested lines appear in the MCP tool
  `get_editor_hints`; there is no new tool.
- Before and after an edit, two sets of one epoch can be compared:
  failures gone and new. A change is a link, not proof of cause.
- A run starts only when a staff member presses start. There is no
  schedule in version 1.

### The seven test families

Narrow experiments find why a reader failed, without a second checker or
score. Each family passes GanitEvolve research before it ships, after the
core room. Old labels (sc-geo numbers) are kept so nothing is lost.

- A. Find the obstacle (001 to 005): shrink a failing page; hand the
  reader the right passage; compare with a facts-only card; add headings
  or nearby sentences one at a time; move split points.
- B. Order and crowding (006, 007, 018, 055): move evidence; add
  plausible clutter; add near-identical sibling pages; repeat one source
  to show repeats are not extra proof.
- C. Sources (008 to 011, 013, 021): shuffle source labels; withhold one
  source; give sources apart, then together; place or date duplicate
  copies differently; give old and new evidence together; withhold needed
  evidence so the reader must say it cannot tell.
- D. Meaning traps (012, 014, 017, 022, 056): a quote then its rejection;
  a summary of a summary; multi-condition questions; the same facts as
  prose, list and table; shuffled table rows and columns.
- E. Wording (015, 016, 053): unfamiliar customer wording; a truthful
  bridge sentence; questions taken from the page against questions
  written independently, to catch an answer hidden in the test.
- F. Structure and reach (019, 020, 023 to 028): link or label changes
  that rescue a lost reader; preview against full page; one claim traced
  through page source, drawn text and the accessibility tree; before and
  after consent choices; clicks to the evidence; site preview limits;
  section links after a heading edit; one resource withheld at a time.
- G. Repair comparisons (045, 048, 050 to 052): replay a known wrong
  answer; replay past successes, which must not be lost; original,
  repair A, repair B and both; a formatting-only placebo; compare edits
  by failures, protected successes and preserved facts.

Family F reads only what the crawler and static checks already keep. It
adds no checker; without the data a test shows "not available" or "not
measured". Families A and F search for the smallest change that helps,
stop at `mirror.search_steps_max` and report "smallest found", never
"smallest possible".

### Rules for every test

- A variant is made inside the room from a frozen set and labelled "test
  copy". It is never published, shown to guests or offered as a rewrite.
  A wording idea is only a proposal; a moderator still approves links and
  changes to member text.
- A variant keeps every value and qualification of the original (a
  price, a date, "except in..."). A fixed rule compares them. A variant
  that loses one is dropped and logged.
- Each test has three fixtures: a case it must catch, a control it must
  not flag, and a missing-input case that gives "not measured".
- Failed and inconclusive runs are kept and shown.
- A repair earns no credit if a formatting-only change helps as much.
- A result is true of these readers on this set only. It never says what
  an outside service sees, how a page ranks or why it was left out.
- Epochs, sets and results are never shared between sites or installs,
  and never pooled into a benchmark.

### Left out on purpose

Probing live outside AI systems; a second bot that fetches pages; a fact
ledger as ground truth (per-task keys instead); cause finding and counts
of "exposed" or "unobserved" pages; routing causes to other tools;
recalibrating weights from reruns; any federation product; two model
families at once; feeding results to a release gate that decides.

### Settings

| Setting | Default | Allowed range |
| --- | --- | --- |
| `mirror.enabled` | on | on or off |
| `mirror.scope` | staff pages and guides | page types; threads off |
| `mirror.memory_share` | set by research | share of the slice, safe floor and top |
| `mirror.cpu_share` | set by research | above zero, safe top |
| `mirror.disk_max_bytes` | set by research | above zero, per site |
| `mirror.frozen_keep_days` | set by research | at least 1, safe top |
| `mirror.readers_at_once` | set by research | 1 up, safe top |
| `mirror.reader_seconds` | set by research | above zero, safe top |
| `mirror.run_seconds` | set by research | above zero, safe top |
| `mirror.models` | none until one is admitted | admitted models only |
| `mirror.repeats` | set by research | at least 1, safe top |
| `mirror.tolerance` | set by research | above zero, safe top |
| `mirror.min_tasks` | set by research | at least 1, safe top |
| `mirror.tasks_max` | set by research | above zero, safe top |
| `mirror.depth_max` | set by research | 0 up, safe top |
| `mirror.context_max` | set by research | above zero, safe top |
| `mirror.families` | each on once its research passes | per family |
| `mirror.variants_max` | set by research | above zero, safe top |
| `mirror.search_steps_max` | set by research | above zero, safe top |

"On" means the room is available; it never runs by itself. Researched
values replace "set by research" before the build.

### Research before the build

GanitEvolve, on made-up sites with known answers and planted faults:
population size, repeats and tolerance; the fewest tasks and readers for
a rate; which value types can be keyed reliably; whether program reader
failures line up with Coach rubric items (one study shared with
[0042](0042-coach-additions.md)); which small local model, if any, fits
the share, licence and admission; memory and time per reader on
real-sized sets; for each family, whether it finds real obstacles while
its control stays quiet, and whether it adds anything beyond the
text-only rubric (a family that adds nothing stays off); search limits;
and whether Linux sandboxes are enough isolation.

### Changes to other parts

- Safe running: `karya.shed_order` gains `test_room`, first. Its attack
  tests gain a case of the room at full share.
- Crawler: the room may read a page copy and make its own sealed copy.
  Removal from guest view also removes the page from frozen sets.
- Coach additions: room results appear as "tested" lines on the same AI
  readability items. The AI order is unchanged; the room is not on it.
- Coach: `get_editor_hints` shows tested lines.
- The approved metric source list gains test-room results.
- The memory budget table gains a room row once measured.
- The settings register gains the `mirror.*` keys.

## Limits

- Everything fits: the room runs inside the service slice, within the
  memory limits in AGENTS.md section 5.6, with no swap and 15%
  headroom. The room is not an extra allowance.
- The room's sandbox has its own memory limit (`mirror.memory_share`),
  counted with every other role's budget. If the sum does not fit, the
  room's share shrinks or runs are refused; no other role is cut.
- Before a run, the promise check asks for room, declaring set size,
  readers at once and model size. With no room the run waits, "waiting:
  not enough memory", and no attempt is counted.
- Readers at once times each reader's need never passes the share.
- Under pressure the room is shed first, at the next task boundary. It
  unloads its model, goes ahead of maintenance and backfill, never
  touches live work, and resumes when pressure clears.
- A checkpoint is saved after each finished task, so a stop or crash
  loses at most the task it was running.
- Processor and disk shares keep the live site fast. It is one heavy job
  at a time, under the load governor.
- Time per reader, per run, tasks, depth, context, variants and search
  steps are capped by the settings above. Disk is capped per site.

## Footprint budget

This part adds nothing to public pages. It reads stored crawler copies
and never calls the site.

## How it is tested

Rust tests, with SQL tests for row-level security:

- MR01: a connection tried from inside the room fails, and a full run
  shows no refused attempt from any reader.
- MR02: a seeded private or staff-only page never enters a frozen set.
- MR03: a page that leaves guest view leaves every set at the next crawl.
- MR04: with no memory room, a run waits with a reason and no attempt.
- MR05: rising pressure stops the room first, at a task boundary, with
  no kernel kill and no change to live work.
- MR06: killing the sandbox mid-run resumes from the last finished
  task and repeats none.
- MR07: a reader that grabs memory is stopped alone; the site stays in
  band.
- MR08: at full share the live site's slowest 1% of responses stay in
  band.
- MR09: one site's sets, traces and results never appear for another
  site or agency client.
- MR10: a set goes on expiry, site removal and erasure request.
- MR11: all budgets plus the room's fit 85% of the service slice, or the
  build refuses.
- MR12: any set-up change makes a new epoch; a cross-epoch comparison on
  different sets is refused.
- MR13: a scorer that links a model runtime fails the build.
- MR14: the same record gives the same verdict; a reader's answer never
  sets one.
- MR15: with no key, no screen or record says "correct".
- MR16: a fake reader pack runs end to end with no rebuild.
- MR17: no record or screen names an outside engine, assistant or brand,
  or says what one does.
- MR18: each failed task gets one class; a room limit is never charged
  to a site.
- MR19: every rate shows its counts; a thin sample shows "not enough
  evidence".
- MR20: unstable repeats are marked, not hidden.
- MR21: a tested line attaches only to an existing rubric item.
- MR22: nothing here blocks a draft, changes a page or starts without a
  person.
- MR23: with every Coach AI source failing, the room is unaffected and
  calls none of them.
- MR24: every test has a catch case, a control and a missing-input case.
- MR25: a variant that changes a value or qualification is dropped and
  logged.
- MR26: no test copy is published, shown to guests or offered as a
  rewrite.
- MR27: failed and inconclusive runs stay visible and counted.
- MR28: a placebo that helps as much removes a repair's credit.
- MR29: a test needing data the room lacks shows "not measured" and adds
  no checker.
- MR30: a family stays off until its research record exists and passes.
- MR31: no output states a cause or counts unseen pages as broken.
- MR32: a search stops at its step limit and says "smallest found".

Scoring, failure classes, epoch identity, set membership and stopping
are proved with Kani where the state space allows.

## Changes from the source

- Decision code, maths and adapters are all rewritten as Rust; maths
  uses ganit.
- Admin pages become screens in the Dioxus app built from SDS.
- The high-assurance decision code becomes Rust proved with Kani.
- Model readers use pure-Rust models only.
- The memory rule follows AGENTS.md section 5.6: the room fits inside
  the service slice, not a flat 8 GB with 15% headroom.

Source: `docs/specifications/signals/machinemirror.md`,
`docs/specifications/signals/machinemirror-2.md` and
`docs/specifications/signals/machinemirror-3.md` in the archive
`archive-2026-11-08-old-repo`.
