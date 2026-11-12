# 0013: Research rules

These rules answer one question: how do we know an idea is true before
it ships? They set the research card that every idea starts with, one
proof standard shared by GanitEvolve (0012) and the lab, the research
memory, rival methods, sealed model bundles, safe twins for fast code,
and queue maths as a second opinion. It is roadmap Phase 2 item 6.

## What it does

### Research first

These rules carry the old project's King Solomon research framework
(the owner asked for it by name, and for it to live inside GanitEvolve).
The research card below is "King Solomon's card" in the owner's words.

1. Every algorithm question and every feature idea starts with a
   research card. It is not a second route; it is the front of the
   GanitEvolve and lab routes.
2. A card holds: the wish in plain words; how it works now; what must not
   break; the nearest thing already known (memory, method catalogue,
   feature list); the budget; and one test that would show the idea wrong;
   and its scope, the exact paths it covers, with no wildcards. It is kept
   in the repository, and a receipt names it by its hash. A card is
   accepted by a record in `reviews/` that names its hash and the agent of
   another product who accepted it; for a GanitEvolve problem the record
   also names every hash that 0012's paragraph on the card's acceptance
   record lists, and the accepting product follows the stricter rule in
   0012, "The effect contract". Any edit to the card, or for a GanitEvolve
   problem to anything whose hash the record names, makes that record
   stale, so it is a draft again. It never makes up a number. A needed
   value is written "to research".
3. A gate check refuses a feature specification or change with no card.
   For source files this is `ganit.every-change` (0012): every receipt
   names its card, and agents must start each change from one (owner,
   10 November 2026).
4. Each idea gets a size from a fixed checklist, never from the person
   who wants it:
   - **Small:** uses admitted methods and researched values, adds no
     number, ranks nothing. The card, a search of the memory and the
     method catalogue, and one small GanitEvolve run on made-up data
     that tries the failing case.
   - **Measure:** needs a value not yet researched (a limit, window or
     weight). The card plus a run sized to find that value.
   - **Prove:** a new method, a rival, anything that ranks or words
     member-facing text, or a speed claim. The card plus the full proof
     standard.
5. Anyone may ask for a bigger size, never a smaller one. A small idea
   that meets an unresearched value becomes a measure at once.
6. Each card records the time it took; the owner sees the median per
   size. Small runs have their own short queue and block no unrelated
   work. The share of ideas sent to Prove is reported, so the checklist
   can be tightened.
7. A run that ends inconclusive is an answer: the feature stays
   unavailable and labelled so. It never ships on a guessed value.

### The one proof standard

This section is the standard's only home. The lab and GanitEvolve name
it and copy none of it. A result counts only if every step holds.

1. **Freeze first.** The claim, the unit of observation (page, site,
   workload, day, workflow), the stopping rule (fixed, looks set in
   advance or sequential), the comparator, the smallest worthwhile gain and the
   largest harm allowed are written and hashed before any scoring.
   Stopping when convenient is refused. An edit makes a new protocol
   that needs fresh approval; old results never carry over.
2. **Qualify the evaluator** with the six self-checks below.
3. **Honest data.** Hidden and fresh inputs; splits by thread and time;
   a final-assessment type that no tuning code accepts. Missing values
   stay missing.
4. **Statistics once,** in ganit: intervals, paired and bootstrap
   differences, false-discovery control over the attempt count, early
   stopping, and "no worse than" within a margin. Interval arithmetic is
   for tests only.
5. **A result kind,** never a bare pass.
6. **Combined ideas earn their keep.**
7. **An independent route and a reviewer.**
8. **One signed receipt.**

### Claim types

Each claim is listed apart and needs its own evidence. Evidence for
something else supports nothing: a passing build alone supports no
claim. A defect fix is not a research claim; it follows AGENTS.md
section 6 rule 1 and autoissues.

| Claim | Evidence |
| --- | --- |
| Same behaviour | Same inputs, results and errors |
| Faster | Timed outside the candidate, repeated, fresh inputs |
| Better | Test declared in advance, held-out cases |
| A guarantee | A derivation, plus code that meets it |
| New | A prior-art record and an example |

A claim moves only forward: waiting, supported, suspended, revoked,
superseded. A revoked claim returns only with new evidence.

### Six evaluator self-checks

Before an evaluator's scores count, a receipt must show each of these
exactly once, all passing, for the same evaluator version. Six copies
of one check, a missing check or an unknown extra kind fails. The value
for each is "to research".

1. The same code under two labels scores the same.
2. A version broken on purpose is caught.
3. A known slowdown or speed-up is measured at its true size.
4. Dropped or cut-short runs are found.
5. Renaming candidates or reordering runs changes no winner.
6. A separately worked exact answer matches.

### Result kinds

Proved or found; counterexample found; unresolved (timed out or killed
twice); inaccurate; fitted but unproven; search exhausted. Only the
first two settle a question. Unresolved is never a pass. Running out of
arithmetic range or of checks means "unresolved", never "different".

### Combined ideas and attempts

1. Two mechanisms together are kept only if they beat the best one alone
   by a margin, shown with an interval, not one number. The margin is
   "to research".
2. Every attempt at one question, failed and abandoned ones too, is
   counted and fed to false-discovery control (ganit job 130), so the
   best of many tries is never shown as a single try. The same count
   reaches it from the lab and GanitEvolve alike.

### Independent route and reviewer

1. Three duties: the explorer is the driving agent; the challenger
   writes hidden tests and an independent answer route; the reviewer
   re-runs the winner. Challenger and reviewer are never the driving
   agent, and one agent holds one duty per question.
2. A route is recorded by what it shares with the candidate (code,
   library, data), as fields the check reads. An oracle on the same
   library is not independent. A route's name, or a second session,
   proves nothing.
3. The challenger is judged on valid counterexamples; nobody is judged
   on agreement.

### The receipt

1. One signed record per result: the hash of the tested bytes, taken by
   the tool and never the submitter; the evaluator version and its six
   check results; the protocol hash; the data digest; the environment
   fingerprint; the seed; the reviewer; the result kind.
2. The gate compares the hash with the change it judges. A byte changed
   after the test fails. A receipt cannot be reused for another run.
3. Receipts come only from the tool that ran the work. A forged one is
   refused. No field is a bare yes or no flag.
4. A result and its memory record are written in one step. A crash
   between them leaves a waiting write that is retried or reported,
   never doubled or relabelled as a success.

### The research memory

1. The memory holds what is known. Forgejo holds the work under way.
   They never repeat each other:

   | Thing | Its one home |
   | --- | --- |
   | Tasks, milestones, who does what | Forgejo |
   | Faults found by checks | autoissues |
   | Owner answers and rulings | The owner's records |
   | Why a catalogue method was rejected | The catalogue's `exclusion` |
   | Numbers of a run | Lab results; the GanitEvolve archive |
   | Agent chat and handovers | The agent mailbox |
   | What was learnt, what failed, why | The research memory |

2. The memory never copies status, owner, assignee, due date or fault
   text. A Forgejo task may cite a memory id; the memory never cites
   Forgejo. A code fault is a task, never a negative finding. A
   disproved idea is a negative finding, never a fault. A record may
   point at an owner answer but never restates or overrides one.
3. Four kinds of record:
   - Episodic: what happened in one run or inquiry, with its receipt.
   - Semantic: what we hold true, with its evidence.
   - Procedural: a tested way to do something.
   - Negative: an idea that failed, with the mechanism, pointers to the
     scores, the reason, any counterexample and the conditions that
     would justify trying again. Never a penalty. For a catalogue method
     the reason stays in `exclusion`; the record adds only the lesson.
4. A record's shape:
   - identity: the SHA-256 of kind, topic and the normalised
     one-sentence statement;
   - the statement in plain words, its kind and scope;
   - evidence pointers, each with the path or id and the content hash of
     what it cites, plus the commit and data digest where there are any;
   - state: valid, stale, refuted, superseded or waiting, nothing else;
   - when made, by whom (agent product and role, not a person), and a
     supersede link.
5. A record never holds member or post text, real site or member names,
   or secrets. A private tier appears only as ids, counts and digests.
6. Records are append-only. Age, disk pressure and failure never delete
   one. A changed finding is a new record that supersedes the old.
7. A lesson learnt from outcomes enters as "waiting" until a second check
   accepts it.

### Repeats, conflicts and regressions

1. The same identity cannot be written twice. A second writer adds an
   evidence pointer and a count of independent confirmations to the
   first record.
2. Before a write, the writer must show the nearest record and catalogue
   entry. A statement that has a home elsewhere is refused.
3. A recall count counts recalls; rewriting does not raise it.
4. Two valid records on one topic with opposite verdicts are both marked
   "in conflict". Neither is recalled as fact until the reviewer settles
   it with a new record that supersedes one.
5. A record that clashes with an owner answer is refused and reported.
6. Before a card is accepted, the memory is searched. A refuted record
   on the same idea shows first, with its reconsider condition. The card
   must show that the condition now holds, or stop.
7. A refuted record never goes back to valid; only a new record with new
   evidence supersedes it.

### Validity and stale spreading

1. Exactly one function decides whether a record may be recalled as
   fact. Only "valid" qualifies, in every reader and screen.
2. When a cited file, result or method changes hash, every record
   citing it turns stale at once, and every record linked to those by a
   concept-map edge shows "depends on a stale record". The change is
   written to a journal.
3. Recording the same statement again with fresh evidence makes it valid
   again.

### Recall

1. Ask in plain words or by concept. Results are ranked by evidence
   strength, then freshness. Stale records appear only when asked for,
   and are marked.
2. `research.memory.recall_chars` bounds the answer. When it cuts, the
   answer says how many records it left out and why.
3. Plain-word recall comes first. Meaning-based recall is added only if
   research shows plain recall misses.

### The concept map

1. A point on the map is a thing already named elsewhere: a method, a
   feature, a ganit job, a rule, a data kind. The map uses those ids and
   makes none of its own.
2. An edge has a type and must cite a record. Types: uses, rival of,
   supersedes, contradicts, borrowed from, depends on, measured by.
3. An edge's identity is its two ends and its type. Writing it again
   adds evidence, never a second edge.
4. Weight comes from the cited record's evidence strength. Nobody types
   a weight in. A stale citation lowers the edge to "unknown".
5. It answers: an idea's neighbours; paths up to
   `research.memory.map_depth`; what depends on a source; what
   contradicts a claim.
6. It is rebuilt from the records, so it never disagrees with them. It
   shows as a list on the lab results screen; a picture comes only if
   the list proves hard to read.

### Where the memory lives

1. Records are files in the research folder on Dell, beside the
   GanitEvolve archive, under the data SSD rules. No database is used, so
   the row-level security rule does not arise. A small index is derived
   and can be deleted and rebuilt.
2. Every command uses the one durable store. A round trip keeps cited
   files and commits. An unavailable store stops the write and says so;
   there is no fallback and no store name in code.
3. `research.memory.disk_cap` stops new writes with a message. It never
   deletes anything.
4. Whether files are enough at the real volume is "to research"; if
   recall becomes slow, the owner is asked.

### Rival methods: winner on, others selectable

1. A rival answers the same job as another method (for example a
   learned adapter against whitening for job 273). Signals blended into
   one score are not rivals.
2. The lab runs rivals under the proof standard. The winner is on for
   every site from the start, with researched values. Every rival that
   passes the whole standard stays selectable; one that fails is kept as
   a memory record, not shipped. Each selectable rival stays tested on
   every release, and that time is reported.
3. The choice is one setting per site, `method.<job>.choice`. Site
   admins may switch it, and agency staff may for their own client
   sites. No site's choice reaches another site or install.
4. The settings screen marks the winner. For each rival it shows, from
   the released evidence and in plain words: how it did against the
   winner with its range, its speed, its memory, and what it is better
   at, if anything. It never shows one score alone.
5. A switch goes through the shadow test and canary on that site's own
   data, with one-step undo that shows the old value first.
6. Only the chosen method is loaded. A switch that would not fit in the
   memory limits of AGENTS.md section 5.6 is refused, with the reason.
7. A release that changes the winner leaves each site's choice alone and
   says a different winner now exists.
8. Being selectable adds no count. A rival counts toward the methods bar
   only if it is itself distinct and qualified.
9. If a chosen rival loses its qualification, the site goes back to the
   winner through the normal rollback, with a notice. This is a stated
   exception to the rule against fallbacks.
10. "Every method must run" applies to the job's chosen method only.
    Rivals stay idle until chosen.

### Sealed model bundles

1. Every trained model leaves the lab as one sealed bundle, and the
   engine refuses anything else. A bundle holds:
   - the weights, in a data-only format (safetensors);
   - a SHA-256 for the bundle and for each file;
   - the version and the job it serves;
   - the training data's identity: a snapshot digest, never text;
   - the feature contract: inputs in order, units, the missing-value
     rule;
   - test results, including fixed inputs with expected outputs and the
     tolerance for the Rust parity test;
   - the seed, tool versions and environment fingerprint;
   - the receipt for this exact bundle hash, with its result kind;
   - measured size and peak memory, and the rights note.
2. At load, the engine checks every field and hash, that the receipt
   names this hash, that the feature contract matches, that the parity
   test passes and that it fits in memory. Any failure refuses the
   bundle, and the bundle in use keeps running.
3. Training happens only in the lab, in ganit. Serving is a Rust
   decision. A generated test set is an input to a judgement, never the
   judgement.
4. A bundle trained on one install's data never goes to another install.
5. Whether weights can reveal training text is "to research". Until it
   is answered, only bundles trained on made-up data ship; those trained
   on a private tier stay on Dell.

### Fast routines keep a slower safe twin

1. Every fast routine (an evolved kernel or hot path) has a safe twin,
   written first and never deleted. It is slower and simpler.
2. The twin is independent: it shares no code or library with the fast
   routine. A twin on the same library checks only data passing.
3. Every change runs both on the same inputs plus fresh random ones,
   including errors and badly conditioned data. Their difference stays
   within the declared tolerance. The twin's hash is in the receipt.
4. A test build with the fast path off still gives right answers.
5. At install and upgrade, the fast routine runs a sealed known-answer
   set made by the twin. The twin's code need not ship. A mismatch holds
   the job and tells the admin; it never swaps in a slower routine.
6. Faster never means in charge: fast output is evidence, safe code
   decides.
7. After `research.attempts.review_after` failed tries to beat the twin,
   the owner is asked whether to drop the target. The attempts are kept
   as a negative record.

### Queue maths as a second opinion

1. Three closed formulas, and no more: Little's law (the average number
   waiting is the arrival rate times the average wait); M/M/s (several
   identical workers, random arrivals and job times: the chance of
   waiting and the mean wait); M/G/1 (one worker, any spread of job
   times: the mean wait).
2. They cross-check ganit jobs 226 and 245, beside the simulation job
   226 already uses. They never decide.
3. The screen shows "formula says X, measured says Y". Beyond
   `research.queue.disagree_margin` it adds a tip that arrivals may come
   in bursts or jobs may vary more than assumed. A tip changes no
   setting.
4. If arrivals meet or pass capacity, it says "no steady state: the
   queue grows" and gives no wait. A missing input shows "not measured".
5. Finite buffers, abandonment, priority queues, estimation and
   simulation methods are left out; a job that needs one asks through a
   card.
6. To research: how far formula and measurement may differ on bursty
   forum traffic before the tip misleads.

### Settings

| Setting | Default | Allowed range |
| --- | --- | --- |
| `research.memory.recall_chars` | to research | above zero, safe top |
| `research.memory.disk_cap` | to research | above zero, safe top |
| `research.memory.map_depth` | to research | 1 to safe top |
| `research.attempts.review_after` | to research | 1 to safe top |
| `research.twin.selftest_cases` | to research | safe bottom to safe top |
| `research.queue.disagree_margin` | to research | above zero, safe top |
| `method.<job>.choice` | the winner | the job's selectable list |

Researched values replace "to research" before the build is approved.
The rules start on. No value is zero, and none can be set past its safe
top. Every value goes through GanitEvolve research first. The agent
limit is the agent mailbox's own.

## Limits

- The memory runs on Dell only and ships to no site. It loads on demand,
  as one heavy job within AGENTS.md section 11. Only made-up data, ids
  and counts go in; nothing is shared between installs.
- A rival switch must fit within the memory limits in AGENTS.md section 5.6;
  only the chosen method is loaded.
- A bundle is refused if it does not fit in memory.
- Recall size and disk use are capped by the settings above; the disk
  cap stops writes and never deletes.
- A fixture-only run is labelled as such and counts for nothing.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Rust tests on the research rules, the memory and the bundle loader,
with the deciding rules proved by Kani where they fit:

- An idea with no card is refused at the gate.
- A small idea that meets an unresearched value becomes a measure.
- Asking for a smaller size is refused; a bigger size is allowed.
- Six copies of one self-check, or a missing one, fail the evaluator.
- A passing build alone leaves a speed claim unsupported.
- A byte changed after the test fails the receipt check.
- A double kill reads "unresolved" and is never a pass.
- The driving agent cannot also be reviewer or challenger.
- The same attempt count reaches false-discovery control from the lab
  and GanitEvolve alike.
- A lab result passes only if it also meets the one standard.
- The same statement written twice leaves one record and two pointers.
- A write holding a task, assignee, status or fault is refused.
- Record in one command and recall in a new process: same record, same
  evidence, same state.
- Changing a cited file turns every citing record stale at once, and its
  dependants show it.
- Every state reaches every reader; only "valid" returns as fact.
- Two opposite valid records show "in conflict", not as facts.
- A refuted idea shows first on a new card, with its reconsider rule.
- A recall over its budget says how many records it left out.
- Recording again with fresh evidence makes a stale record valid.
- No command deletes a record; a full disk refuses new ones.
- An edge written twice stays one; a typed weight is refused; the map
  rebuilt from records matches the stored map.
- A rival that passes shows beside the winner; one that fails does not.
- A site with no choice runs the winner.
- A switch is shadow-tested, and undo shows the old value first.
- A switch that would not fit in memory is refused with its reason.
- A bundle with a missing field, bad hash or wrong receipt is refused,
  and the active model keeps running.
- No bundle holds post text, a real name or a secret.
- Rust output that differs from the lab's fixed outputs by more than the
  tolerance gets the bundle refused.
- A failed known-answer set holds the job and tells the admin.
- A twin that shares code with its fast routine fails review.
- With the fast path off in a test build, answers are still right.
- Arrivals at or above capacity give "no steady state", never a wait.
- A missing queue input shows "not measured"; a tip changes no setting.
- Finite buffers, abandonment and priority queues are not present.
- Attempts past `review_after` open a question and stop; they never
  loop.

## Changes from the source

- Rules that decide (card size, claim state, receipt check, admission,
  validity, repeats, conflicts, stale spreading, rival pass, bundle
  admission, twin tolerance) are rewritten as Rust checks, proved with
  Kani, Verus or Creusot where they fit.
- Statistics, ranking of evidence strength and the queue formulas are
  rewritten as Rust in ganit.
- Screens, including the method choice and the lab results list, are in
  the Dioxus app.
- Fast routines and their twins are Rust; the old kernel language and
  its validation page are dropped.
- The old research department and memory programs are not carried; only
  their rules come across, under plain names.
- The 400-line file rule of the old code gives way to the limits of
  AGENTS.md section 6 rule 11.
- The bundle parity test is the Rust parity test, with pure-Rust models.

Source: `docs/specifications/ganit-evolve/research-rules.md`,
`docs/specifications/ganit-evolve/research-rules-2.md`,
`docs/specifications/ganit-evolve/research-rules-3.md` and
`docs/specifications/ganit-evolve/research-rules-4.md` in the archive
`archive-2026-11-08-old-repo`.
