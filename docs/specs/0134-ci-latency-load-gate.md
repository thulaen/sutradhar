# 0134: CI latency load gate

The load gate measures how long typical requests take under
repeatable load. It blocks a confirmed p95 regression. p95 is
the time within which 95 in 100 measured requests finish.
It also records the median (p50) and the slower tail (p99).

Example: a candidate change makes the owner-local review read slower.
On the commit tier both sides run 500 requests; the candidate's p95 is
130% of the signed baseline, over the 120% limit, so a first breach
opens an investigation. A later valid pair breaches again, so the
failure is confirmed and the commit is blocked. If the runner had been
disturbed, the pair would be invalid and the result "Load test could
not be verified". All figures are illustrative.

DR-121 adds this check to commits, pushes, staging and releases.
Spec 0007 and AGENTS.md retain their separate hot-path gates,
including the existing timing, race and instruction limits.

## Words used here

| Word | Meaning |
|---|---|
| Pair | One run of the reference and the candidate on one trusted runner lease |
| Workload | The signed list of requests the runner offers |
| Baseline | The signed timings of the accepted reference build |
| p95, p99, p50 | The time within which 95, 99 or 50 in 100 requests finish |
| Unverified | No trusted result exists; this is not a pass or a failure |
| Lease | The exclusive timed slot on the runner |
| Monotone clock | A clock that never goes backwards |
| Epoch | A counter that rises each time the trust settings change |
| Oracle | The expected answer a test checks against |
| Drain | Waiting for the last offered requests to finish |
| Namespace | A named area of the evidence store |

## What this spec designs

The separate p95 load gate and signed evidence for specs 0001 and 0005.
Spec 0007 retains its hot-path rules.

## Runner and workload

Use the shared gate catalogue and Bazel. The trusted runner
checks the exact candidate and loads the signed baseline and
reference through [0001's load-gate evidence][load-evidence]. Verify
its admitted protected profile before using the namespace. Without
that exact proof the load gate is unverified. Execute the load generator
and candidate in sealed local test environments, with only a
local test transport. Use the exclusive timed slot on Dell.
A staging or release host can measure only for its own admitted
runner class, profile and platform. Shared CI runners consume
signed results and may run diagnostic measurements. Diagnostic
numbers cannot qualify or fail the gate. An absent trusted
receipt leaves it unverified.

[load-evidence]: 0001-door.md#load-gate-evidence

Register a signed workload manifest separate from the candidate.
Each scenario names its supported product entry point, exact
synthetic work, stable fixture, platform, offered rate, duration,
response deadline and expected outcomes. Bind activation to a
reviewed required workload inventory maintained by the gate.
The Phase 2 bootstrap uses the door's normal signed-receipt
check with synthetic envelopes. It proves only that route.
Product scenarios become mandatory at admission of their
normal entry points: owner-local candidate review read,
protected add-on ingest and acknowledgement, and cached Coach
evidence read. Until then, show their exact prerequisite.
A staging or release candidate shipping an enabled entry point
cannot pass without its admitted scenario. Run the applicable
roster on every candidate using real entry points, fresh
synthetic site data and pinned dependencies. Do not use
live sites, accounts, provider APIs or ranking model variants.
A documentation-only commit or push still needs the scenarios
required by its protected roster. Spec 0003's inapplicability
proof for code checks cannot stand in for a required load receipt.
A missing required receipt leaves the tier unverified.
The sealed test can prefill synthetic cache or model state only
where the signed workload declares that state.

Use one deterministic interleaved workload per pair side,
offering the fixed rate per scenario in the admitted roster.
Assign a unique ID and exact scheduled time to every request.
Measure monotone scheduled-start to final byte, including queue
time. Keep offering requests even while others are slow.
Warm up both sides equally. Reset to their declared states.
Drain through the last offered deadline. An unserved or timed
out candidate request contributes its deadline. If a proved
generator fault prevents accounting for offered IDs, discard
that pair. A returned wrong response is a separate blocking
correctness fault and never a latency-based invalidation.

| Tier | Warm-up and measurement per side (proposed) | p95 relative limit (proposed) |
|---|---|---|
| Commit | 5 s warm-up, 10 s measured at 50 requests/s per scenario, 500 samples each | 120% of signed baseline |
| Push | Same short run on the door's trusted class | 120% |
| Staging and release | 15 s warm-up, 120 s measured at 100 requests/s per scenario, 12,000 samples each | 115% |

All figures in this table, the pair caps, 150 s, 25 minutes, the
90%–110% drift screen, the 150% ceiling and the 110% and 125% warnings
are proposed until measured on the Dell.

The default and maximum deadline is five seconds (proposed). Every
admitted deadline must exceed both blocking thresholds.
A profile unable to meet this rule is unavailable. No sample is
dropped as too slow. The commit gate checks the candidate tree,
the 0001 door checks the exact push, and spec 0005 checks staging
and release against the exact artefacts and tier. Staging and
release each run a long separate workload.

Reuse a receipt only for the same protected enclosing gate run,
tier, candidate, workload and baseline generation, with all
existing admission checks still satisfied. Recover a lost reply by
looking up that run's receipt. Repeated delivery of a receipt
counts the same pair once. It gives no new vote or attempt budget.
A receipt from a different gate run is inadmissible even if the
candidate bytes and thresholds match.

Arithmetic (proposed): a commit or push pair is two sides x (5 s + 10 s) = 30
s, plus drain and reset, about 40 s. The 150-second budget fits three such
pairs, not five, so at those tiers the time budget below decides the pair
count and the five-pair cap is only a ceiling. A staging or release pair is 2
x (15 s + 120 s) = 270 s, about 5 minutes with drain, so 25 minutes fits four
(five would leave no margin). Commit and push load work counts against their
tier budgets; release load work runs in the release long lane (DR-069), exempt
from the 30-minute rule; staging load work stays within its own 25-minute
budget. That budget plus the other staging checks must fit the 30-minute tier
total (AGENTS.md section 7); if it does not, the speed-up task is raised.
Commit and push load work adds about 120 s (three pairs) to those tiers'
totals.

Start with prebuilt sealed tier artefacts. Limit active short
load work to 150 seconds and long load work to 25 minutes,
including retry, resets and drain. Request the exclusive lease
only while the enclosing gate has enough time left, and use its
existing wait budget. Short checks cannot overrun the fast push
suite. An exhausted epoch or lease budget leaves this tier
unverified. Raise the existing long-tier task if applicable.

Before starting each pair, check the remaining monotone time on the
short or long load budget, the enclosing tier and the exclusive lease.
The signed workload must bound the pair's two warm-ups, measurements,
drains through their last offered deadlines, resets and finalisation.
Start only if that bound fits every remaining budget. Stop when any
deadline is reached. A wholly completed valid pair keeps its vote.
An incomplete pair has no vote. If the budget leaves an unsettled
scenario, record it as unverified. Keep the existing attempt caps and
record the limiting deadline in the signed run receipt.

Each scenario uses a normal route. The protected workload pins its
route, version, transport, committed synthetic fixture, initial
store digest and expected typed response.

| Scenario ID | Entry point and fixture | Response oracle |
|---|---|---|
| `LOAD-0134-DOOR` | Spec 0001 signed-receipt check with a matching synthetic candidate and trust package | Accept the receipt with the exact candidate, tier and decision |
| `LOAD-0134-REVIEW` | Spec 0028 `setu.suggestion.list` with two eligible suggestions and one foreign-site decoy | Return the two permitted cards in seeded order and no other card |
| `LOAD-0134-INGEST` | Spec 0021 normal paired outbox read and acknowledgement with one new event per offered work unit | Match the accepted cursor and one committed event per source identity |
| `LOAD-0134-COACH` | Spec 0132 `keyword_lookup_results` through an enrolled owner-local MCP session with `keyword.read`. Use one site-bound saved mock snapshot with exactly two selected eligible rows | Return those typed rows, freshness and source. No provider calls or charge effects |

## Baselines and noise

| Record | Fields |
|---|---|
| Workload manifest | Scenario ID, input fixture and expected response hashes, route and load driver versions, platform and runner class, rate, warm-up, timeout, tier, hard ceiling, profile hash and owner-approved rule revision |
| Baseline receipt | Accepted reference revision and exact build, fixture, profile, platform, host class, p50/p95/p99, calibration evidence, admission signature, baseline generation |
| Run receipt | Protected enclosing gate run ID, tier, candidate and reference build IDs, signed workload digest and baseline generation per scenario, fixture and profile hashes, runner lease nonce, protected decision epoch, exact artefact and driver digests, ordered pair attempt numbers and their sample counts, p50/p95/p99 and timeouts for both sides, validity reason, raw sample digest, verdict and signature |
| Review record | Exact candidate, tier and signed run receipt, scenario, valid pair attempt number and its signed p99 warning, reviewer, disposition, reason and date. This is independent of the numeric latency verdict |

The protected owner policy signs workloads. The sealed runner signs baseline
and run receipts outside the candidate process. The door and 0005 verify
current signer, profile hash, runner lease, platform, tier, candidate,
reference and decision epoch. Resolve the baseline's signed calibration
evidence against the exact admitted runtime and load-driver bytes in the
protected profile used for the pair. A historical baseline is unverified for
changed inputs even when its label and measured reference p95 agree. A
verified matching replacement still follows the existing baseline and limit
rules. Sign the initial baseline after three valid paired runs of the same
accepted reference, signed per tier in the long lane (DR-069), within at most
six attempts and 25 active minutes (proposed), and valid only when the three
p95 values agree within 10% of their median; otherwise the baseline stays
unsigned and the gate unverified. Use the median of the run percentiles for
each measure. Refuse zero, incomplete, censored or unsigned baseline samples.
Derive the hard ceiling as the smaller of 150% of that initial p95 and an
existing absolute ceiling admitted for the scenario. If there is no existing
absolute ceiling, use 150% of the initial p95. Round each measured duration
and computed relative, absolute or warning limit up to one whole microsecond.
A missing or incompatible baseline or ceiling leaves the gate unverified. The
door owns replacement. No candidate can loosen an admitted limit or baseline
through a change to the workload, timings, thresholds or renderer. Weakening
needs the existing signed owner waiver route. At staging and release no waiver
may relax a limit for the candidate under test. Elsewhere a waiver must be
signed before the pair begins, bound to the candidate and receipt, expire
within 30 days and be renewable once for the same check (BJ-29 of spec 0114,
SUT-1416, DR-159; time is the pinned signed time of the door or CI, checked
when the waiver is used), and never relax a ceiling above the existing
absolute ceiling. A waiver never reverses a confirmed failure of the exact
receipt.

Run the accepted reference and candidate on the same runner lease and profile.
Alternate pair order across rounds. Use preflight calibration and reference
drift to screen noise. Invalid means mismatched attestation, disturbed
exclusive lease, incomplete generator output, or reference p95 outside
90%–110% of its signed baseline. A candidate's slow samples, timeouts or p99
cannot make a round invalid. Allow at most five total pairs (proposed),
including at most two invalid pairs and at most three valid pairs. Calibration
is separate from the candidate workload and cannot be failed by its latency. A
remaining runner fault reports “Load test could not be verified”. Do not
report it as a performance regression or pass.

## Verdict and summary

Use nearest-rank percentiles over all measured requests.
For N sorted latency samples, index `ceil(q × N)` is the
one-based percentile at fraction q. Use fixed-point integer
comparisons in checked 64-bit arithmetic; an overflow makes the result
unverified. Candidate p95 breaches if it is strictly greater
than either its tier's relative limit against signed baseline
or the scenario's hard ceiling. Equality passes.

Count each valid pair's votes by exact tier, candidate, signed workload
and baseline generation, scenario ID, profile and runner class. Never
combine breaches from different scenarios. Finish on the first wholly
valid all-pass pair if no investigation has opened. Otherwise keep the
votes from every valid pair. A scenario's first breach opens an
investigation and needs a later valid pair. Two breaches for the same
scenario confirm failure. After a later valid pair, two passes for
that scenario settle it as passed. A pass followed by a breach alone
leaves it unsettled. Apply this to every new breach in a shared pair.
A confirmed failure blocks the candidate. The candidate passes only
when every required scenario has valid passing evidence and every open
investigation has settled. Invalid pairs give no votes. If a required
result remains unsettled at the pair or time cap, report unverified
unless a failure has already been confirmed. Preserve every pair,
exact vote and invalidity reason in the receipt.

Show p50 and p99 for each valid pair and their signed-baseline
differences. Warn for p50 above 110% or p99 above 125%. Neither
is a numeric load-gate failure. At staging and release a p99
warning on any valid pair requires an independent performance
review under AGENTS.md 7.1. Derive the required warning set from
the exact signed run receipt for the tier. It contains each valid
pair's p99 warnings, keyed by scenario and pair attempt number.
At staging or release require an accepted signed review for every
warning, bound to the exact candidate, tier, run receipt, scenario
and pair. Missing, partial, rejected or wrong-scope review evidence
holds progression. Keep the numeric p95 verdict unchanged. Rebase,
profile change or replacement evidence needs a new review. The
review can accept with reasons, request a new candidate or reject
on its own findings. It cannot waive a p95 failure or supply proof
for an independent 0007 hot-path check.

Default to an in-job summary. A permitted PR comment may mirror
only the signed summary without host or site identifiers,
secret material, traces or fixtures. State which tier ran and
whether its result passed, failed or is unverified. Show
baseline and candidate p95 against both limits, p50 and p99
warnings, sample counts, invalid attempts and review status.
Use the existing trust boundary for public report delivery.

## Data

The 0001 load-gate namespace is the authoritative store for signed workload,
baseline and run receipts, with the bound independent p99 review references
for the required warning set.
The sealed runner owns the raw sample vectors until it seals the receipt. Keep
vectors (at most 12,000 samples per scenario per side, proposed) in protected
runner scratch, bind their digest to the signed receipt, and clean scratch on
completion, interruption or recovery. Retain a vector only when the exact 0001
evidence profile names its protected store, authorised readers, retention and
cleanup. A profile that requires durable vector evidence holds until those
fields are admitted. Never include vectors or lease identifiers in a public
mirror.

## Settings

There is no product setting. Apply the signed tier, scenario, rate, deadline
and calibration profiles above. Changes follow the protected baseline and
owner-waiver rules, including the 30-day waiver expiry. A candidate cannot
select its profile.

## Failure codes

Record a typed latency result of `passed`, `confirmed_regression`
or `unverified`. Use `unverified` for missing or incompatible proof,
runner or generator faults, or unsettled scenarios at the cap. A
confirmed regression retains its verdict if another scenario is
unverified or a later measurement faults.
Bind the exact scenario, failed check and evidence reference when
available. Keep the independently blocking correctness fault and
required p99 review in their own fields. A wrong response remains a
fault and does not discard a valid latency vote. Only the existing
release and door rules decide progression. No new public SUT code.
Use “Load test could not be verified” for an unresolved runner fault.

## Security and privacy

Use synthetic requests, a sealed local transport and a trusted runner.
The candidate cannot sign its own result. Keep raw evidence and lease
identifiers out of reports mirrored to a permitted PR comment.

## Requirements

All tests use a monotone fake clock, synthetic request data
and fixed latency vectors. No source code or CI run is needed
to define this contract.

| Requirement | Acceptance |
|---|---|
| REQ-0134-P95 | UT-0134-P95 measures exact rank boundaries, timeout values, equality, relative-only and ceiling-only breaches and p50/p99-only shifts. Only p95 drives the numeric verdict |
| REQ-0134-NOISE | IT-0134-NOISE plants isolated runner interference, repeated valid regressions and two opposing initial pairs. Confirm the final offered ID and its deadline, candidate timeout versus generator fault, wrong-output failure, all invalid placements and the global time and attempt cap. Breach only A in pair one and only B in pair two. Confirm neither unless the same scenario breaches again. Resolve both with an all-pass pair. Exercise expiry before and during the deciding pair |
| REQ-0134-TRUST | E2E-0134-TRUST refuses a 31-day waiver, a staging or release waiver of a confirmed p95 failure, and a relaxation signed after the pair began. It supplies missing and tampered receipts, wrong platform, fixture, threshold, lease, epoch, artefact and candidate. Try a public-runner result and a proposed slower baseline. No forged evidence qualifies |
| REQ-0134-TIERS | E2E-0134-TIERS invokes all four tiers with the expected workloads. A short receipt cannot clear a release. Test p99-only review at staging/release and ensure the legacy 0007 checks are still enforced. Exercise LOAD-0134-COACH on its exact enrolled MCP grant and saved selection |

The release record binds gate version, signed workload and
baseline, per-scenario verdicts and any mandatory review.
Missing evidence blocks the applicable tier without calling it
a latency regression.

## Limits and footprint

Use the exact tier-specific rates, deadlines and paired-run budgets
above. Test traffic stays in the trusted runner. No request reaches a
live site or provider and this gate adds no work to public requests.

## How it is tested

UT-0134-P95 and IT-0134-NOISE cover bounds and confirmation.
E2E-0134-TRUST and E2E-0134-TIERS cover receipt and tier admission.
In E2E-0134-TIERS, use an exact documentation-only candidate with
proof for each inapplicable code check. Withhold a required load
receipt independently at commit and push. Even if all other
evidence is complete, the affected tier stays unverified.

In E2E-0134-TIERS, give staging a valid p95-passing pair with p99
warnings in two scenarios. Withhold all reviews, then accept one
warning's exact review and leave the other unreviewed. Separately
offer its otherwise valid signed review from the release tier or
another run of the same staging candidate. The numeric verdict
stays passed in each case and staging remains held. Supply exact
accepted reviews for both warnings. That clears only the staging
review hold. Give release its own complete case. Reuse neither
tier's evidence for the other. Apply 0007's independent outcomes.

In a separate E2E-0134-TIERS case, keep all three pairs valid in one
long-tier run. Scenario A breaches p95 in pair one and passes in pairs
two and three. All other p95 votes pass. Scenario B has the same p99
warning value in pairs one and three. The final p95 verdict is passed.
Accept only the exact review for B in pair three. Keep progression held
for B in pair one. Offer the pair-three review in the pair-one slot
of the same run and refuse it. Both exact reviews clear this review
hold. Keep the independent 0007 outcome in force.

In a separate release verifier case under E2E-0134-TIERS, use an
already admitted current replacement of the signed load evidence.
Keep the enclosing gate run, candidate, tier, workload, scenario
and pair attempt number fixed and give both receipts equal p95
and p99 values. Both carry the same p99 warning. Offer the valid
accepted review of the original receipt for the current one.
The exact signed receipt identity differs, so keep progression
held and the current numeric p95 pass. Only the exact accepted
review of the replacement clears this warning obligation. Keep
the 0007 checks separate. This fixture evaluates existing evidence
without starting a pair, changing admission or resetting a budget.

Extend IT-0134-NOISE and E2E-0134-TRUST with a timed-out candidate,
unaccounted generator ID, wrong response, missing required raw-proof
profile and scratch cleanup after interruption. E2E-0134-TRUST also
checks a lost reply and repeated receipt delivery. Recover the same
signed verdict with each pair counted once and no budget reset.
Reject a receipt from a different enclosing run or baseline
generation. Withhold the dedicated load store or its admitted profile
while the 0007 store stays readable. Offer a valid 0007 timing receipt
in the load slot. Each leaves the load gate unverified. Separately offer
a signed historical load baseline after changing only the runtime
calibration profile or driver bytes. Keep labels fixed and keep the
reference p95 inside the allowed band. Hold before counting any votes.
Verify the matching admitted control without changing either latency
limit. Check exact typed decisions and that no raw vector appears in
mirrored output.

## Changes from the source

DR-121 adds a separate p95 load verdict to the existing checks. The
staging and release p99 warning uses the independent review route above.
