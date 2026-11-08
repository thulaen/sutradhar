# 0057: Quality judging and release assessment

Before a release, two judges each read at least 1,000 of Sutradhar's
link suggestions and say, for each, whether it is a good link and why.
By default both judges are local pure-Rust graders, built on different
methods, and nothing leaves Sutradhar. Only when the owner turns it on,
an AI judge may take part: a model on AWS Bedrock, with the owner's own
key, or an AI tool the owner connects through Sutradhar's MCP server.
The judges first judge without seeing each other's answers. Where an AI
judge disagrees it sees the other's reasoning, without knowing whose,
and judges again; a third round makes a failure name the rule broken
and a pass answer it. Cases never settled count as failures. Results count only
if the lower end of the 95% confidence
range is at least 95%, so a lucky sample cannot pass. Then every release
rule is checked together, and the release goes ahead only if all pass;
otherwise the screen says exactly which failed and why. Every release is
also battle tested. It is roadmap Phase 7 item 1, the release gate of
[AGENTS.md section 3](../../AGENTS.md#3-the-goal-and-the-release-bar).

Example, with an AI judge on: 500 fresh suggestions; the judges agree on 470 in
round one and settle 22 more later; 8 stay contested and count as failures. Of
the 500, 485 pass: the lower end of the range is about 95.1%, so this passes.
With 484 it would be about 94.9% and fail, even though 96.8% passed.

## What it does

### Words used here

- Judge: a grader that reads only what it is given and returns a
  verdict. Local judges are pure-Rust graders and are the default. An
  AI judge is a Bedrock model or an AI tool connected through the MCP
  server, used only when the owner turns that route on, with no tools.
- Judging pack: a batch of suggestions with their context, the rubric
  (the written rules for a good link) and the verdict format.
- Verdict: pass or fail, a reason code and one line of explanation, in a
  strict JSON format.
- Round: one pass of judging; up to three.
- Lower 95% bound: the lowest true pass rate consistent with the
  results, 95 times out of 100 (Clopper-Pearson intervals).
- Release candidate: an exact tagged version being assessed.
- Soak: 72 hours of supervised, uninterrupted running.
- Protocol file: the frozen rules for sampling, intervals and subgroups,
  fixed with its SHA-256 before any scoring.

The rubric's content is its own owner-approved file. How suggestions
are made, the reconstruction protocol and the release tagging route
belong to other specifications.

### Crates and traits

- `eval`, a crate under `tools/`, never shipped in the add-ons: packs,
  judges, rounds, caching, verdict records and the Arrow export of
  verdicts (ids, verdicts, reason codes and rounds; never post text). A
  karya job on Dell runs it. Its records live in PostgreSQL in their own
  schema, with RLS forced.
- ganit, which karya triggers after judging: Mulyankan job 126
  (pass-rate intervals and pass marks), 127 (recall and anchor-agreement
  intervals), 128 (paired comparisons on the same posts, for "no worse
  than the last release") and 133 (sampling that matches the site's
  mix); Satya, ganit's engine for truth, job 150 (judge agreement), 152
  (judge bias and deference) and 156 (clusters of disagreement that
  point to an unclear rubric). Each output carries its reproducibility
  stamp (code version, snapshot, seed) and passes the numerical guards.
  Job 151 (a combined best estimate of truth) is a report only, because
  the counting rule below is fixed.
- Results are read in the Dioxus app's screens and in Rust lab runs with
  recorded inputs and outputs, through the versioned API and the Arrow
  exports.
- `publisher::assess` reads every criterion's evidence and gives the
  release decision.

```rust
pub trait Judge: Send + Sync {
    fn id(&self) -> JudgeId;
    async fn judge(&self, pack: &Pack, round: Round)
        -> Result<RawVerdicts, JudgeError>;
}
pub trait VerdictCache: Send + Sync {
    async fn get(&self, key: &CacheKey) -> Result<Option<Verdict>>;
    async fn put(&self, key: &CacheKey, v: &Verdict) -> Result<()>;
}
pub trait Criterion: Send + Sync {
    fn id(&self) -> &'static str;
    async fn assess(&self, rc: &Candidate) -> Assessment;
}
```

- `RuleJudge` checks each suggestion against the rubric's written rules
  over the parsed text (0026); `ModelJudge` grades it with the pinned
  pure-Rust models (the reranker and the entity model of 0024), with
  its own thresholds. They share no code beyond the pack reader, so
  each judges independently.
- `BedrockJudge` sends a pack through the provider client of 0025 with
  the owner's key; `McpJudge` offers a pack to the owner's connected AI
  tool as a read-only MCP resource and takes its verdicts back as a
  draft. Both exist only while the owner has turned their route on.
- `FakeJudge` in tests returns scripted verdicts, malformed and hostile
  ones included.
- Each release rule is one `Criterion`; adding one touches nothing else.
  A missing or stale piece of evidence is a fail, never a pass.
- Redaction uses the shared scrubber with a judging profile. Every
  interval comes from job 126 or 127, never a second copy; the service
  only checks each output's stamp, guards and golden tests.
- Packs are files, verdicts are JSON lines, and the release decision is
  the logical AND of the criteria.

### Samples and the protocol

1. Two samples, assessed separately: at least 500 fixed suggestions,
   kept from release to release, and at least 500 fresh ones made by
   this candidate on unseen final-assessment data.
2. Unseen data is split by thread and by time, so no thread or later
   post used in tuning appears in the assessment.
3. Subgroups: thread type, post length, destination popularity, site
   section, and reply versus thread target. Each is reported, and a
   subgroup worse with 95% confidence fails.
4. The protocol file fixes the sample sizes, seeds, interval method,
   subgroups and every threshold before scoring. A changed protocol
   starts a fresh assessment. Samples are never redrawn to get a pass,
   and difficult cases are never dropped.

### Packs

- Batches of 25 suggestions. Each holds the anchor sentence with bounded
  surrounding context, the target's title and a bounded excerpt of the
  target; a reply target includes its real reply context.
- Bounds are whole sentences up to about 300 characters, never cut
  mid-word.
- Identifying data is removed: emails, phone numbers, member names and
  the site's name and addresses.
- Post text is untrusted data: it sits in clearly marked fields, and the
  rubric tells the judge to ignore instructions inside it.
- The verdict schema is strict: pass or fail, a reason code from the
  rubric's list and one line of at most 200 characters.

### Judging, rounds and reliability

- Local judges run in the `eval` job with no network, reading only the
  pack folder. They are deterministic: the same pack gives the same
  verdicts.
- AI judges run only on the owner's PC, where the owner's Bedrock key
  and MCP connections live, and only while the owner has turned the
  route on (AGENTS.md section 5.5). Packs for them are built there by
  the same pack code; only verdicts, reason codes and pack ids return
  to the release evidence. A Bedrock judge goes only through the
  provider client; an MCP session sees one site's packs and nothing
  else, under the Cedar rules. No prompt transcripts are kept.
- Round 1 is blind. Round 2, for disagreements where an AI judge takes
  part, shows each AI judge the other's verdict and reason without
  naming its source. Round 3, for those still split, requires a fail
  to cite rubric criteria and a pass to answer them. Local judges do
  not change their answers, so a split between two local judges is
  contested at once. Unresolved cases count as failures and are listed
  as contested.
- One repair of an invalid verdict is allowed; the first invalid answer
  still counts in that judge's and round's invalid rate. More than 2%
  invalid blocks the release.
- First-round agreement below 95% blocks the release until an approved
  rubric or judge correction and a fresh assessment.
- Flip rates per judge and round are recorded. Suspicious deference is
  flagged for the owner: a judge whose round-2 changes towards the other
  are more than twice the other's, with 20 or more changes (job 152).
- The judges also find coverage opportunities, independently and blind
  to Sutradhar's output.
- Later moderator decisions feed learning, never a finished gate result.

### Caching and resuming

- Judging starts only on an exact release-candidate tag.
- Cache key: the content hash of the pack item, the judge's model,
  rubric, prompt, schema and redaction versions, the round, and the hash
  of the opposing reasoning. Any change misses the cache.
- An AI provider's usage limit pauses the job with its reason; it resumes
  when the window reopens and never repeats judged items.

### Nightly work and the soak

- Nightly: non-judge checks only (hard rules, reconstruction, anchor
  agreement, coverage on the fixed set, subgroup drift of scores).
- An optional warning when merges have changed 50 suggestions' scores
  uses the local judges; AI judges join only within a budget, zero by
  default, and only when the owner has turned them on.
- A warning when the last 200 decisions' lower bound falls 10 points
  below the release's judged pass rate.
- Drift sampling by AI judges runs only when the owner has turned them
  on; the local judges may run it at any time.
- The soak: 72 hours of the candidate's `prod` profile, ganit's nightly
  jobs included, with production limits, supervised, with durable
  progress. It survives the editing computer switching off. Any crash,
  lost or duplicated work, memory over budget or service-level breach
  fails it and restarts the clock. Elapsed time uses a monotonic clock.

### The release assessment

`publisher::assess` checks every criterion for the exact candidate;
each must pass with current evidence:

1. Hard safety rules at 100%, every starred hard case included.
2. Each required quality result, on each sample, with a lower 95% bound
   of at least 95%, and no subgroup worse with 95% confidence.
3. Coverage of independently identified valid linking opportunities at
   95%, with the same bound.
4. Reconstruction recall and anchor agreement no worse than the last
   release.
5. Judge reliability: invalid verdicts at most 2%; first-round agreement
   at least 95%.
6. The 72-hour soak passed.
7. The counts of AGENTS.md section 3: at least 6,000,000 distinct
   qualified methods, 6,000,000 shipped capabilities and 16
   quadrillion checked variants, each counted only with its evidence,
   with no padding.
8. Every gate check of the candidate passed at 100%, the slowdown rule
   and the release-tier memory test included; 100% line, branch and
   MC/DC coverage, zero surviving mutants, property tests and the Kani
   proofs of the hard rules.
9. No open P0 in AutoIssues (the automatic issue tracker), and the
   release gate's tracker rules met.
10. All agreed release scope present.
11. The licence duties met: the release's source archive and third-party
    notices (cargo-about and ganit's licence check) are published with
    it, and every dependency is allowed and AGPL-compatible.
12. Every ganit output used as evidence carries a stamp matching this
    candidate and the frozen protocol; a stale one is missing evidence
    (SUT-855).
13. Battle tested: every check below passed for this candidate, the
    add-ons included, with current evidence.
14. The release artefacts are signed, all six reviews pass and the owner
    approves.

The Assessment screen and `sutra release assess` show each criterion
with its result, its evidence and, for a failure, why in plain English.
Release notes list capabilities added, changed and retired.

### Battle testing every release

The owner's rule: every release candidate, the add-ons included, gets
fault injection, performance improvements, full defect checking,
security checking, resource management, storage compression, memory
management, profiling improvements, chaos testing, QA and load testing,
so each release is battle tested not to break, bend, fold or run out of
memory.

- Never breaks: no crash, lost or duplicated work, or corrupted data
  under any injected fault; every fault ends in recovery or a plain
  error with its code.
- Never bends: under the full load test, 99% of new posts get a first
  suggestion within 30 seconds, the app's targets hold, and an add-on
  uses at most 8 MB per request.
- Never folds: at twice the expected peak, Sutradhar slows, sheds
  background work and refuses new work with an explanation; it never
  collapses, and site pages are never affected.
- Never runs out of memory: zero out-of-memory kills in the soak, load and chaos
  runs; every part under its budget; the combined peak within the limits of
  [AGENTS.md section 5.6](../../AGENTS.md#56-limits-on-the-owners-pc).

The eleven checks, each with evidence stored under the candidate's tag.
A missing, stale or skipped result fails criterion 13. Every failure
becomes an AutoIssue.

1. Fault injection: a fault matrix for every dependency and boundary,
   service and add-ons: PostgreSQL and the site's database down, slow
   or refusing; the site's API erroring or timing out; disk full; the
   event stream cut; ganit down (degraded mode); a model file damaged;
   malformed and replayed signed requests; the service unreachable from
   an add-on (site pages unaffected, the add-on within 8 MB). Injected
   through wiremock and test doubles in unit and integration tests, and
   through process and control-group controls.
2. Performance: the slowdown rule passes, and the release report lists
   each hot path's improvement work this release, or the stop rule's
   reason.
3. Full defect checking: every test kind, 100% coverage, zero surviving
   mutants and Kani proofs on rule-critical code, fuzzing at its
   nightly length, static analysis in every allowed language (clippy,
   PHPStan at its strictest, Psalm taint), and no open P0 or P1 for a
   shipped component.
4. Security checking: cargo-deny, cargo-audit, composer audit, gitleaks,
   trivy on every release image, fuzzing of the management API from its
   OpenAPI document, and the signature and replay tests.
5. Resource management: under the full load test every part stays
   within its CPU, process and memory limits; database pools add up
   below the connection limit; no file handle or task leak over the
   soak.
6. Storage compression: stored text, snapshots and backups are
   compressed (zstd) and verified; disk used per 1,000 posts and the
   compression ratio may not get more than 5% worse than the last
   release.
7. Memory management: each part's peak against its budget in the soak,
   load and chaos runs; no upward trend in memory over the soak
   (ganit's job 165 finds change points); the inference watchdog's 80%
   and 90% steps proven by test.
8. Profiling: flame graphs, heap profiles and model operator profiles of
   the pure-Rust models for the candidate, compared with the last
   release's; any regression is an AutoIssue and blocks until fixed or
   explained.
9. Chaos testing: each part killed mid-work recovers within 60 seconds
   with no lost or duplicated work; the noisy-neighbour test holds the
   service levels.
10. QA: the full acceptance suite, every app screen with visual
    regression and accessibility checks, and each add-on installed,
    used, upgraded and removed on its test site by a written checklist.
11. Load testing: the 2-minute quick load test in the gate, and the full
    load test at twice the expected peak on the 50,000-thread tier,
    with the million-post workload projected from measured scaling. It
    must meet "never bends" and "never folds".

### Data model

In the `eval` schema of PostgreSQL on Dell, every table with RLS forced:

- `protocols`: id, SHA-256, frozen at, contents.
- `packs`: id, candidate tag, sample, items, redaction version.
- `verdicts`: pack item, judge, round, verdict, reason code, valid,
  repaired, cache key.
- `results`: candidate, measure, sample, subgroup, passes, total, lower
  bound, passed.
- `soaks`: candidate, start, checkpoints, events, result.
- `assessments`: candidate, criterion, result, evidence id, time.

### Failure handling

- SUT-850: judge unavailable; paused.
- SUT-851: invalid verdict after repair.
- SUT-852: invalid rate over 2%.
- SUT-853: first-round agreement under 95%.
- SUT-854: protocol changed after scoring began; the run stops.
- SUT-855: evidence missing or stale for a criterion.
- SUT-856: soak interrupted; restarted.
- SUT-857: provider terms not verified; judging refused.
- SUT-858: candidate not an exact tag.
- SUT-859: a credential visible to the wrong judge; the run stops.

Edge cases:

- A suggestion whose target is hidden between pack building and judging
  is dropped from both samples and replaced by the protocol's next item,
  recorded, never silently.
- An AI judge's model retired by its provider mid-run: the run stops; a new
  model needs a rubric check and a fresh assessment.
- Both judges fail the same case for different reasons: a failure, with
  both reasons kept.
- A fresh sample smaller than 500 because the site is small: the
  assessment fails; the sample is not padded.

### Security and privacy

- By default nothing leaves Sutradhar: local judges grade in place.
  Bounded, redacted packs leave the owner's PC only through Bedrock or
  an MCP tool, and only while the owner has turned that route on. For
  Bedrock, account terms, training controls, service locations,
  no-tools enforcement and credential isolation are verified and dated
  before any use (SUT-857 otherwise).
- Packs and verdicts stay on Dell; only reviewed anonymous aggregates
  may be public.
- Each AI judge sees only its own credential; no judge sees the
  other's identity.

### Screens, capabilities and accessibility

- Judging screen, `sutra judge <tag>`: CAP-162, "Get a release
  candidate's links judged by two judges", with a switch, off by
  default, for each AI route.
- Assessment screen, `sutra release assess`: CAP-153, "Release a
  version, or see exactly why it was refused".
- Capability counts on the assessment: CAP-155; the release number:
  CAP-156; release notes: CAP-154.
- Soak screen, `sutra soak <tag>`: NEW-036-A, "Run and watch a 72-hour
  supervised soak of a release candidate".
- Contested cases list: NEW-036-B, "Review the cases the two judges
  never agreed on".
- The screens are Dioxus app screens from SDS components, WCAG 2.2 AA.
  Results are a table with the measure, passes, total and the bound in
  words ("485 of 500 passed; at least 95.1% with 95% confidence").
  Contested cases are a list with each judge's reason. Charts have text
  equivalents.

### Observability

`sutradhar_eval_items_total{judge,round,outcome}`,
`sutradhar_eval_invalid_ratio{judge,round}`,
`sutradhar_eval_agreement_ratio{round}`,
`sutradhar_eval_flip_ratio{judge,round}`,
`sutradhar_soak_elapsed_seconds` and `sutradhar_assess_criterion{id}`
(1 pass, 0 fail). Judging pauses and resumes are logged with reasons. A
failed nightly check files an AutoIssue.

## Limits

- Building packs: at least 1,000 suggestions a minute.
- Scoring a full assessment: under 60 seconds.
- The release assessment: under 2 minutes, excluding the soak.
- Resuming after a usage window reopens: within 60 seconds.
- An AI judge's own time belongs to its provider and is reported, not
  gated. Each target follows the slowdown rule of AGENTS.md section 6
  item 8.
- On Dell, judging runs as a heavy job through the heavy-job queue
  (spec 0059), `pankti run -- CMD`, in one place of 4 GB and 8 CPU
  threads (AGENTS.md section 11): the `eval` orchestrator about
  128 MiB and the two local judges about 512 MiB each while judging,
  the models loaded once; then ganit, about 1 to 1.5 GB, for the
  statistics, never at the same time as the judges (estimates).
- On the owner's PC, AI judging runs in the service as one job in the
  heavy job slot of 0024, within the memory limits in AGENTS.md
  section 5.6.
- The soak and the full load test run on pinned test machines that
  match both sizes of owner's PC, at the `prod` limits of AGENTS.md
  section 5.6.
- The released product is not changed by this part.

## Footprint budget

This part adds nothing to public pages. Battle testing checks that site
pages load as without the add-ons when the service fails.

## How it is tested

Paths are under `tools/eval/` unless they say otherwise.

- QJ-01: packs from fixture suggestions hold 25 items each, with anchor
  sentence, bounded context, target title and excerpt, reply context for
  replies, and whole sentences only. `tests/packs.rs`, unit and
  property.
- QJ-02: seeded emails, phones, member names and the site's name appear
  in no pack. `tests/redact_props.rs`, property.
- QJ-03: round 1 packs contain no other verdict. `tests/rounds.rs`,
  unit.
- QJ-04: in round 2 each judge sees the other verdict and reason with no
  judge name or model. `tests/rounds.rs`, unit.
- QJ-05: a case still split after round 3 is a failure and is listed as
  contested. `tests/score.rs`, unit.
- QJ-06: a malformed verdict repaired once is used and still counts in
  the invalid rate; a malformed repair stays invalid. `tests/repair.rs`,
  unit.
- QJ-07: 2.1% invalid verdicts, or 94.9% first-round agreement, fail the
  assessment with SUT-852 or SUT-853. `publisher/tests/assess.rs`, unit.
- QJ-08: 485 of 500 passes; 484 of 500 fails, under the protocol's
  method. A ganit unit test of job 126 with golden output;
  `tests/interval_props.rs`, property (the stamp and guard checks).
- QJ-09: a protocol file changed after scoring begins stops the run with
  SUT-854. `tests/protocol.rs`, unit.
- QJ-10: changing any cache key part misses the cache; changing none
  hits and the judge is not called. `tests/cache_props.rs`, property.
- QJ-11: a judge's usage-limit answer pauses the job with the reason;
  it later resumes with no item judged twice. `tests/resume.rs`, unit;
  `tests/chaos/eval_kill.rs`, chaos.
- QJ-12: a fake judge's scripted obedience to instructions in posts is
  caught by the schema check, and the real judges' results on the
  adversarial fixture are reported. `tests/injection.rs`, unit.
- QJ-13: with AI judging off, no pack leaves Sutradhar and no AI judge
  is built; with it on, a Bedrock judge uses only the provider client
  and the owner's key, and an MCP session sees only its one site's
  packs. Local judges run with no network. `tests/routes.rs`, unit.
- QJ-23: the two local judges give the same verdicts for the same pack
  and share no grading code. `tests/local_judges.rs`, unit and
  property.
- QJ-14: a branch name or untagged commit is refused with SUT-858.
  `tests/candidate.rs`, unit.
- QJ-15: with each criterion failing or missing in turn, the release is
  refused and the criterion named. `publisher/tests/assess.rs`, unit;
  `tests/acceptance/cap_153_release.rs`.
- QJ-16: a process killed during a soak fails it, records why and
  restarts its clock. `tests/chaos/soak_kill.rs`, chaos.
- QJ-17: with a warning budget of zero, the nightly calls no judge.
  `tests/nightly.rs`, unit.
- QJ-18: every screen and state passes accessibility checks (axe through
  a headless browser) and matches its screenshots. Visual and
  accessibility tests of the Dioxus app.
- QJ-19: scripted verdicts with known agreement, and one judge deferring
  three times as often: jobs 150 and 152 match the golden value within
  tolerance and flag the deferring judge. ganit unit tests with golden
  output.
- QJ-20: a ganit output whose stamp names another candidate or protocol
  fails criterion 12 with SUT-855. `publisher/tests/assess.rs`, unit.
- QJ-21: for any combination of criterion results, the decision is
  release only if all passed with current evidence.
  `publisher/proofs/decide.rs`, Kani proof; mutation testing of
  `publisher`.
- QJ-22: the verdict export's Arrow schema holds ids, verdicts, reason
  codes and rounds only. `tests/arrow_export.rs`, unit.

Battle-testing scenarios:

- BT-01: evidence for ten of the eleven checks fails criterion 13,
  naming the missing one. `publisher/tests/battle.rs`, unit.
- BT-02: each fault in the matrix, injected in turn, ends in recovery or
  a plain error with its code, with no lost or duplicated work.
  `tests/chaos/fault_matrix.rs`, chaos.
- BT-03: with the service unreachable, slow or answering wrongly, site
  pages with each add-on load as without it, and its peak stays within
  8 MB. PHPUnit tests on each test site.
- BT-04: at the expected peak, 99% of new posts get a first suggestion
  within 30 seconds and the app meets its targets. Full load test.
- BT-05: at twice the expected peak, background work pauses, new work is
  refused with an explanation where needed, and site pages are
  unaffected. Overload test.
- BT-06: across the soak, load and chaos runs, no out-of-memory kills,
  every part under budget, and the peak within the section 5.6 limits.
  `tests/chaos/memory_budget.rs`, chaos.
- BT-07: over the 72-hour soak, job 165 finds no upward change point in
  any part's memory. ganit golden test; `publisher/tests/battle.rs`.
- BT-08: a candidate 6% worse in disk use per 1,000 posts fails check 6.
- BT-09: a flame graph showing a hot path 10% slower files an AutoIssue
  and the release waits.
- BT-10: a dependency with a known advisory, or a release image with a
  high-severity finding, refuses the release (check 4).
- BT-11: a benchmark failing the slowdown rule against its
  fingerprint-matched baseline, or a release report with no hot-path
  work and no stop-rule reason, fails check 2. Criterion benchmarks.
- BT-12: one surviving mutant in rule-critical code, coverage below
  100%, a static-analysis finding in any language, or an open P1 for a
  shipped component each fail check 3 by name.
- BT-13: a planted secret, a Psalm taint path in an add-on, or an API
  request from the OpenAPI fuzzer that crashes a route each fail check 4
  by name. Plus a fuzz target for the API.
- BT-14: under the full load test, CPU, processes, file handles and
  database connections read every second stay within limits, pools stay
  below the connection limit and nothing grows without bound.
- BT-15: each part killed mid-work recovers within 60 seconds with no
  lost or duplicated work, and the noisy-neighbour run keeps the
  service levels. `tests/chaos/kill_each.rs`, chaos.
- BT-16: on each test site the written checklist (install, use, upgrade
  and remove the add-on) and every screen's screenshots and
  accessibility checks pass, with a record naming the candidate.

BT-08 to BT-13 are unit tests in `publisher/tests/battle.rs`.

Rule-critical code: the release decision, the cache key, redaction and
the round rules. They need zero surviving mutants, property tests, and
Kani proofs where tractable, for example "the decision is release only
if every criterion passed with current evidence" and "an unresolved
case is never counted as a pass". ganit's jobs follow its own gate:
unit and property tests, numerical edge cases (empty data, one
observation, all passes, ties) and golden outputs with stated
tolerances.

Test-first order: the protocol file, the verdict export and the ganit
jobs with golden outputs (QJ-09, QJ-22, QJ-08, QJ-19); packs and
redaction (QJ-01, QJ-02); rounds with `FakeJudge` (QJ-03 to QJ-06);
cache, resume and candidate check (QJ-10, QJ-11, QJ-14); local judges
and AI routes on synthetic packs (QJ-12, QJ-13, QJ-23); criteria
and the assessment with the proof and mutants (QJ-07, QJ-15, QJ-20,
QJ-21); nightly, soak and screens (QJ-16 to QJ-18). Then battle testing:
criterion 13 (BT-01); checks 2, 3, 4, 6 and 8 (BT-08 to BT-13); the
fault matrix (BT-02, BT-03); load, overload and resources (BT-04,
BT-05, BT-14); chaos and QA (BT-15, BT-16); memory and leaks over the
soak (BT-06, BT-07).

## Changes from the source

- The release counts follow AGENTS.md section 3: 6,000,000 methods,
  6,000,000 capabilities and 16 quadrillion checked variants, not
  2,000 and 2,000; signed artefacts, the six reviews and the owner's
  approval join as criterion 14.
- ganit's statistics jobs and their tests are rewritten as Rust.
- Results are read through Rust lab runs with recorded inputs and
  outputs, not the old notebooks.
- Screens are Dioxus app screens from SDS components; their visual and
  accessibility tests are Rust tests.
- Commands are `sutra judge`, `sutra soak` and `sutra release assess`,
  built with Cargo.
- Memory follows the limits in AGENTS.md section 5.6, not the old
  5.6 GB with headroom.
- Local pure-Rust judges are the default, in place of two outside AI
  command-line judges in containers. AI judges come only through
  Bedrock or an MCP-connected tool, when the owner turns that on.
- The soak and full load test run on the pinned test machine, not on
  Dell with borrowed room.
- The old 20x goal and 5% ratchet become the 5% median slowdown rule.
- Static analysis and audit tools are those of the allowed languages.
- Battle testing covers the WordPress plugin as well as the XenForo
  add-on.
- Model operator profiles are those of the pure-Rust models.

Source: `docs/specifications/operations/judging.md`,
`docs/specifications/operations/judging-2.md` and
`docs/specifications/operations/judging-3.md` in the archive
`archive-2026-11-08-old-repo`.
