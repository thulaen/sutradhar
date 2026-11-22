# 0149 Causal estimators and placebos

This spec owns three local analysis variants for the
[Scientific Experimentation Engine](0148-scientific-experimentation-engine.md).
[Spec 0150](0150-experiment-source-and-sensor-gates.md) owns source and sensor
admission. [Spec 0094](0094-reporting-forecasts-proof-and-experiments.md) owns
randomised assignment, exposure, guardrails, approval and rollback. Spec 0148
owns registration, lifecycle and evidence review. These estimators produce
evidence receipts. They have no editorial, guidance, provider or ranking
effects and cannot approve a transition.

The randomised route identifies an intention-to-treat effect from an admitted
assignment. The observational routes may estimate a causal effect conditional
on independently qualified identification assumptions and sensitivity
bounds. Their receipts always identify the observational design. Prediction,
pre-period fit and placebo checks cannot establish its assumptions.

Figures: every numeric limit, threshold and budget in this spec is proposed
until measured on the Dell or settled by a QR.

## Exact input and receipt

### DES-0149-ANALYSIS-BOUNDARY

The internal Rust boundary implements the data shape used by
`API-0148-FINAL-WORK`. It takes the site's plan and checked facts
through 0148 and 0150. It accepts one registered estimator variant
for each invocation. Use the 0148 schema version, opaque
site-local identifiers, SHA-256 digests and reasoned optional fields.
Counts use checked unsigned 64-bit values. Exact derived values are
reduced signed rationals with positive denominators. Preserve the
primary metric's unit, native calendar, ordering and registered sign.

| Typed input or result | Boundary and checks |
|---|---|
| Registered analysis | 0148 supplies the immutable plan link, `source_plan_digest`, `analysis_plan_digest`, exact variant, metric, frozen groups, fit and validation windows, comparison profile, decision claims, registered margins, authority and expiry. Group and day keys resolve only in the frozen site and metric. Scope proof binds each source projection. |
| Frozen pre-only values | The declared producer supplies admitted aggregates and explicit cell states through 0150. Resolve only the expected groups and day keys. Accept an observed zero only with the exact coverage proof. A missing, suppressed, non-final or unauthorised input holds unless the registered producer proves the complete compatible group aggregate. |
| Final-look values | The claimed ordinal and all three current source receipts must match the installed 0148 claim before the Rust caller opens the pinned matrix. Check `actual_input_manifest_digest`, ordered scope, variant and arithmetic, admission, deadline, stop and erasure generations. The estimator consumes the one registered primary outcome. It receives the other producers' eligibility proof through the claim. |
| 0104 bounded proposal | A permitted worker sees immutable typed numerical values and opaque scope keys. Rust checks the exact pinned inputs, variant, scope and arithmetic against its bounded result. Keep private text, raw URLs, tokens, paths, member identifiers and provider payloads outside the worker and numeric fixture. A worker proposal has no receipt or decision authority. |
| PlaceboAudit | Bind analysis, exact profile, pseudo-start or sham map, kind, eligible denominator, reasoned missing scope, exact signed gap where eligible, absolute gap, threshold count and outcome. Keep the descriptive result linked to its individual check. Approval remains with the owning admission. |
| CausalVerdict | The authoritative signed result binds the claim and final input identities, exact registered design, estimand and metric, qualified state and margins, effect and outward-rounded bounds where permitted, decision class where passed, error-budget, placebo, sensitivity and admission receipts. An unqualified outcome preserves only the permitted state and safe reason. Scope or erasure withdrawal fences subsequent reads. |

The committed plan binds the exact observational treatment and control
scope, final pairs or fitted weights where used and their admitted
pre-only inputs before exposure under 0148. The blinded randomised
plan commits its rules and seed before disclosure. Its realised
assignment and derived sham assignments are sealed separately and
bind the same `analysis_plan_digest`. Any later verification of a
sealed fit must reproduce those choices from the pinned pre-only
inputs. Placebo fits use their own frozen windows and cohorts.

Use 0148's error contracts at the owning result boundary.
`ERR-0148-AUTH` covers a current authority or site failure.
`ERR-0148-STALE` covers a changed plan, snapshot or lease expectation.
`ERR-0148-SOURCE` covers an unqualified required producer or incomplete
exact cell evidence. `ERR-0148-HOLD` covers confirmed overlap, withdrawal
or an actual interruption at release. `ERR-0148-PROFILE` covers missing
method or platform qualification. `ERR-0148-LIMIT` covers an exact work,
input or lease excess. Preserve the registered numerical hold reason
for failed placebo, sensitivity, support or decision predicates.
Only the existing 0148 lifecycle governs release and supersession.

| Record | Required fields |
|---|---|
| Registered analysis | Parent plan and source plan digests, `analysis_plan_digest`, decision claim IDs and typed nulls, active owned site, estimator and arithmetic versions, predeclared hypothesis, direction, estimand, outcome unit, one primary metric, native calendar and ordered day keys, intervals and release deadline, eligible groups and frozen page membership, independence or dependence structure, minimum useful effect, power profile, guardrails, final-look and error-budget identities, permitted authority and expiry |
| Metric profile | Producer and property, source projection, attribution and revision rules, exact measure and numerator/denominator, finality and coverage predicate, explicit zero rule, permitted query or audience scope, retention and disclosure policy |
| Final evidence | Exact registered group/day matrix and snapshot digest, current scope and source proof from 0150, explicit evidence status per cell, assignment or realised intervention receipt, contamination and guardrail status, pinned method admission |
| Analysis receipt | Plan, look, source, method, numeric-profile and release digests, design and estimand, cohort and coverage, exact effect and signed authoritative decision bounds where permitted, direction and registered decision margins, evaluation and sensitivity states, false-alarm budget used, placebo coverage and result, safe hold reasons, independent admission receipt |

Use 0148's canonical framing. Before revealing assignments or accessing
outcomes, the preregistration binds `analysis_plan_digest` to the immutable
analysis record. The digest covers exact source plan identity, metric and
calendar, ordered unit and dependence layout, pre-fit and analysis windows,
placebo dates and selection recipes, weights or allocation rules,
contribution widths, sensitivity grid, qualified numeric and power profiles,
decision claims, and error allocation. Store the actual realised assignment
and any pre-exposure derived placebo assignments in a separate sealed
manifest. Its receipt binds the same `analysis_plan_digest`. Neither digest
includes its own value. Final producer cells and watermark proof belong to
the final look receipt, never to an editable registration.

A candidate metric is mean final same-site organic Web Search clicks per
frozen eligible canonical page-day. It requires an independently admitted
complete page-day source for the registered property and surface. A query
matrix, revised aggregate scope or omitted day cannot fill a missing cell.
Another metric can be registered only under its own admitted profile. Never
substitute a metric, group, source or comparison after registration.

For this count profile, a group-day value is its eligible click sum divided
by the number of frozen pages in the group. The denominator excludes no page
after the start. For a complete window W, Y(g,W) is the unweighted mean of
the group-day values over its ordered native days. The estimator therefore
weights days equally. An explicit observed zero enters as zero. A missing,
suppressed, non-final or unauthorised cell prevents the exact complete matrix
unless the registered producer proves a complete compatible group aggregate.
A report can show safe coverage while withholding the result. Withdrawal or
erasure invalidates the dependent receipt through 0148 and 0150.

## Proposed initial profile

These are design defaults and hard limits for the initial profile candidate.
They need independent qualification before registration. A narrower current
source, machine or authority limit always applies. Profile versions are
immutable and a replacement needs separate qualification.

| Parameter | Proposed initial value and boundary |
|---|---|
| Error and power | One final look, total decision alpha 1/20 and required planning power at least 4/5. Default diagnostic family is descriptive. If formal diagnostics are separately admitted, take 1/100 from the signed total and leave at most 1/25 for the primary claim. |
| Useful effect | For the clicks metric propose the greater of 1/100 click per page-day and 1/20 of the frozen treated-eligible baseline rate, determined before allocation across all eligible randomised groups. The owner confirms the exact margin, sign and planning alternative. Default alternative is twice the useful margin and gives no pass without power evidence. |
| Scope | One site, at most 256 groups and 32 frozen pages per group. At most 8,192 distinct pages and 224 native days per snapshot. Disallow overlapping groups. Count fields accept 0 through 1,000,000,000 per page-day. This input cap does not certify a tight inferential bound. |
| Randomised | At least 8 and at most 32 independent strata. Each stratum has an even size from 4 through 16, with equal allocation and at least 2 whole groups per arm. Apply the power floor and full limits together. |
| Matched | At least 16 and at most 64 treated groups, at most 192 eligible controls, at most 12,288 candidate edges and up to 8 declared in-space pseudo-treated cohorts. |
| Synthetic | 5 through 8 eligible donors for the main fit and at least 4 for each held-out donor placebo. Weight denominator Q = 10. At most 19,448 exact grid points per fit. Use daily outcome predictors only in this profile. |
| Pre history | Pre means and held-out validation use the last 28 complete clean days before the signed anticipation cutoff. Both observational variants fit using the preceding 56 complete clean days. The signed pre-period closes before treatment or anticipation could begin. |
| Final window | 28 complete native days from P0 under 0148. P0 is fixed at registration. Source deadlines and finality come from 0148/0150. |
| Placebos | Pseudo-starts are 112, 84 and 56 native days before the first date barred from the pre-period. All 3 need valid complete results. In space, randomised studies use four registered sham splits of their untouched control groups, synthetic uses each eligible donor once, and matched uses a frozen list of distinct pseudo-treated control cohorts of the same size as the actual cohort. At least 4 valid in-space results are required. |
| Pre-fit and placebo tolerances | For matched and synthetic validation, the exact root mean squared validation gap and absolute mean validation gap must each be at most the registered minimum useful effect. Require the difference between the first and second 14-day mean gaps to be at most that margin in absolute value. Any eligible placebo with absolute gap at or above the useful margin holds. Passing requires independent proof of comparability and bias limits. |
| Arithmetic and work | Reduced rationals at most 512 bits in each numerator and denominator after reduction. Compute with bounded exact intermediates up to 1,024 bits. Round interval ends outwards to a grid of 1/10,000 of the primary unit. Cap exact solver work at 150,000,000 simple counted arithmetic steps per analysis and 64 MiB scratch. Stop at a tighter admitted machine or queue limit. |

Freeze the native calendar, precise intervention and exposure edges, pre
windows and final evaluation window separately. Validate every native-day
boundary through the registered calendar version, including clock
changes. Use the actual source watermark at the final look. Consume
the sole look through 0148's durable claim within the inclusive signed
eligibility interval. Open the pinned outcomes only under that
claim and current authority. Recovery under 0148 may reread those
exact pinned bytes within the original deadline and fences. It
consumes no new look. On clock or watermark uncertainty, hold. Interim
status may show elapsed time,
source completeness, exposure and guardrails. It carries no evolving
efficacy estimate, confidence interval or optional early efficacy decision.
A missed look cannot be rescheduled. A later source correction supersedes
the consumed result as inconclusive and does not create a replacement look.

A possible contamination alert pauses the look before any outcome read.
Resume only after an independent reviewer proves that exact alert false
and the owner approves the appended correction under 0148. Recheck all
gates against the same frozen plan before its original deadline. A false
alert correction cannot cure confirmed contamination, change the frozen
evidence policy, or reopen a claimed or consumed look.

## VAR-0149-BLOCKED-ITT-DID

### DES-0149-ALLOCATION-CONTRAST

Use the exact 0094 manifest for concurrent blocked randomisation of whole
pages or admitted page groups. The profile freezes blocks, group membership,
assignment ratio, assignment algorithm and seed commitment before assignment.
Block covariates come from the pre-period. Pin the ordering and bounds
before assignment. The proposed 0094 assignment method takes one 32-byte
operating-system seed. Use the existing 0094 commitment and encrypted seed
custody. The committed plan, excluding seed-dependent outputs, binds the
algorithm version and sorted strata and group keys. For each stratum take
the first 12 bytes of SHA-256 of the canonical tuple of the method ID,
plan-core digest and stratum key as its nonce. Refuse any nonce collision.
Use RFC 8439 ChaCha20 with that nonce, the root seed as key and counter 1.
Read successive unsigned little-endian 64-bit words. Shuffle the ordered
groups from the last index down, choosing a uniform earlier-or-equal
index. For a range of size m reject values at or above
floor(2^64 / m) * m, then use the remainder modulo m. The first half
becomes treatment. Choose at most 64 draws per index and hold if exhausted.
A retry uses the same seed, manifest and draw order. This profile cannot
replace a different 0094 allocation contract.

Analyse every assigned group in its original arm. Record exposure
departures separately. Do not restrict analysis to groups that followed
their assignment or replace sampled groups. Shared interference and
effective exposure boundaries must pass 0094 and 0150 checks.

For group g, let D(g) = Y(g,post) - Y(g,pre). For block b with n(b) groups
out of N frozen groups, calculate

```text
d(b) = mean of D in assigned treatment - mean of D in assigned control
effect = sum over b of (n(b) / N) * d(b)
```

The estimand is the average effect of assignment on group-level daily rates
for the frozen groups. Pre-subtraction adjusts for baseline. The admitted
random assignment identifies this effect. Where there are enough groups in
each arm and the admitted design supports it, use the conservative Neyman
variance estimate

```text
sum over b of (n(b) / N)^2 * (sT(b)^2 / nT(b) + sC(b)^2 / nC(b))
```

Here nT and nC count groups and sT and sC are the corresponding sample standard
deviations of D. Use groups as assignment units. Fixed arm counts make
assignment within each stratum dependent. The interval below requires
independently randomised strata and handles all assignment dependence within
each stratum. This variance expression does not by itself qualify an interval.
The signed profile binds the supported group counts, cluster assumptions,
critical-value or finite-sample method, coverage tests and degeneracy rule. Hold
if these fail. An exact enumeration of the blocked assignment space can test a
sharp assignment null if separately declared. It does not supply an exact test
or interval for the heterogeneous average effect.

## VAR-0149-MATCHED-DID

### DES-0149-PRE-ONLY-MATCH

Read a recorded intervention on the active owned site and untouched
same-site comparison groups. This variant never assigns exposure. Freeze the
treated groups, donor candidates, anticipation cutoff, fit and validation
windows, support and caliper rules, and the disjoint pairing algorithm before
any study outcome is read. Use the exact eligibility and coverage profile
for both sides. The estimand covers the registered matched treated groups.

Build edges only within the same frozen topic, template and source coverage
classes. Let M be the registered useful margin. Let a be the absolute gap
between the groups' means on the 56 training days. Let b be their absolute
difference in first-to-last 28-day training changes. Admit an edge if both a and
b are at most M and every other registered scope gate passes. Its exact cost is
a + b. Solve for maximum cardinality, then minimum total rational cost, using
successive shortest augmenting paths on the bipartite unit-capacity flow graph.
Use Bellman-Ford in canonical edge order on the residual graph. Conceptually add
the secondary integer cost -2^(E-i) to edge i in canonical pair order for E
edges. Compare primary cost first, then this secondary cost. Represent the
secondary cost as a bounded signed integer of at most 12,290 bits in this solver
only. Reverse edges negate both components. This selects the lexicographically
smallest ordered pair list at the same cardinality and cost. Count each residual
edge relaxation against the solver work cap. Hold if full matching, safe
arithmetic or a completed optimum is unavailable. Do not drop treated groups
from the estimand.

For validation, use the unweighted mean daily difference between the
treated and matched control rates across all frozen pairs. Apply its fit
and trend gates only to the separate 28-day validation period. Compare
mean squared gap against the square of the tolerance without rounding.
Use those same 28 days as the pre mean for DiD.

Apply the registered pre and post windows to each pair. The estimator is the
mean of D(treated) - D(matched control), with equal pair weights. Hold for
unproved measurement or timing, unmatched scope, anticipation, inter-group
spillover, overlapping interventions, differential trends inconsistent with
the registered bound, or a potentially material unbounded confounder. Power
and interval qualification apply at the frozen independent or dependence
block level. Repeated days and multiple groups on one site do not establish
independence. The conditional causal interpretation requires stable
comparable measurement, common support, absence of anticipation and
interference, and a defensible bound on the untreated trend difference.

## VAR-0149-SYNTHETIC-CONTROL

### DES-0149-PRE-ONLY-DONOR-GRID

Use one treated group and an ordered pool of comparable unaffected groups
from the same site. Freeze the candidate pool, eligibility and spillover
rules, intervention edge, predictors, scaling, fit objective and separate
pre-validation window. The fit sees only eligible pre-training values. A
predictor's target, scale and weight are frozen in the admitted profile.

Use nonnegative integers k(j) with sum Q, where positive integer Q is the
versioned weight denominator. Set donor weight w(j) = k(j) / Q. Minimise the sum
of squared same-day gaps against the treated series over the 56 training days.
Each day has equal weight and shared rate units. Compute each weighted
prediction and square in frozen native-day and donor order. Do not use a
separate predictor or rescale one group differently. Choose the
lexicographically smallest weight vector in canonical donor order on equal
objective. Enumerate the entire admitted grid in that order. Before starting,
compare the number of grid points, binomial(Q + J - 1, J - 1) for J donors, and
the per-point work and arithmetic bounds with the admitted caps. Hold overflow,
timeout, missing term, no eligible donors, or incomplete optimisation. Do not
release the best incomplete fit or reduce Q as a fallback.

For native day t, predicted untreated rate S(t) = sum of w(j) * Y(j,t). Report
the mean of Y(treated,t) - S(t) over the fixed final window as the synthetic
estimate. Show pre-training fit separately from held-out clean pre-validation
gap, error and signed drift diagnostics. Compare mean squared validation gap
with the squared tolerance without rounding. No post values may change the
weights, donors or bounds. The conditional interpretation requires stable
untreated comparison weights over the target horizon, adequate joint pre-support
and no material untreated shock difference, anticipation, intervention on donors
or spillover outside the registered bias bound. Hold if the validation profile
cannot support those conditions.

## Sensitivity and placebos

### DES-0149-EXACT-PLACEBOS

For both observational variants, use clean pre-training and disjoint
pre-validation periods. A pre-trend slope or fit check uses a signed
tolerance and minimum support. Absence of a statistically detected pre-trend
is insufficient. Independently review exposure, shared dependencies,
concurrent changes and potential common and differential shocks. An
unbounded material confounder or unknown contamination state blocks the
conditional claim even if predictive fit is good.

Define signed rational B >= 0 as a declared bound on the absolute average
remaining untreated trend bias for matched DiD, or untreated prediction bias
over the final window for synthetic control. Record its units, evidence,
justification and the exact sensitivity grid including zero and the largest
admitted bound. A bound derived through statistical estimation requires
joint error accounting with the interval. No bound may be selected using the
final outcomes or inferred from fit alone. For each admissible B, widen the
qualified sampling interval [L,U] to [L-B,U+B]. This interval is conditional
on the bias bound and the qualified dependence and sampling model. Compare
the widest admissible interval against the registered decision margins.
Report where a conclusion first fails or holds across the declared grid.
If a sampling interval is unqualified, a point estimate and sensitivity
range may be shown as exploratory where allowed, without a causal verdict.

For matched DiD and synthetic control use the same frozen profile in both
placebo families. An in-time placebo
uses a predeclared pseudo-start inside the clean pre-period, with its fixed
look ending before the anticipation cutoff. Refit or rematch using only
evidence earlier than that pseudo-start. An in-space placebo uses a
predeclared unaffected group or matched-size donor cohort as pseudo-treated
at the recorded start. Exclude both pseudo-treated and actually treated
groups from its donor pool. Reapply eligibility and fit rules.
Keep separately registered training and validation information available for
every placebo. Record excluded cases and safe reasons for both families.
No post-fit selection of favourable dates, units or error summaries.

Evaluate each placebo with the same gap estimator and a post window of the same
length as the planned final window. Record the signed gap, its absolute value,
eligible denominator and count at or above the registered tolerance. Each family
has a registered minimum valid count and maximum allowed count at or above that
tolerance. These are descriptive diagnostics. A placebo p-value needs a
separately qualified exchangeability and inference profile. In the descriptive
profile, exceeding the declared placebo count limit or missing coverage holds
the conditional claim. A clean placebo family provides no proof of
exchangeability or identification.

### Placebos for blocked randomised trials

Freeze both families and the exact primary source support in the
`analysis_plan_digest`. A placebo observes an inert label comparison.
It cannot allocate treatment, apply a change or enrol a new group.

- In time, for each of the three registered pseudo-start days, retain the
  actual frozen arm labels and strata. Compare the 28 clean native days
  immediately before that pseudo-start with the 28 starting on it.
  Both windows end before anticipation or exposure. Calculate the same
  weighted block DiD using only those two windows, with no fit or
  donor pool. Require complete eligible coverage for both arms.
- In space, use only the registered, independently verified untouched
  controls in each original stratum. Freeze their canonical order and
  use it to derive four sham arm maps after assignment and before
  exposure. For each stratum, let n be its control count. The sham test
  is the first floor(n/2) controls and the sham control is the remainder,
  after cyclic rotation by zero or one position. In maps 0 through 3,
  the low bit selects the rotation in the first canonical stratum and
  the next bit selects it in the second. All later strata use rotation
  zero. There must be at least two control groups per stratum and four
  distinct complete maps. Compare the same registered baseline and
  final 28-day windows. Calculate block DiD for each map, weighting each
  stratum by its verified frozen control count divided by the total
  control count. Use no actually treated unit as comparison evidence.

Bind placebo class, pseudo-start or sham-map digest, exact profile and
eligible coverage in each receipt. Require every planned randomised
placebo to be eligible. Each must have absolute gap strictly below
the registered useful-effect margin. Equality holds. An unavailable,
contaminated or failed placebo holds the registered final decision.
The four sham maps deliberately reuse the same controls and are
descriptive diagnostics. Their count is not a count of independent
trials or exchangeable draws. An exact sharp-null test, if separately
qualified, is a different registered analysis. Synthetic controls and
donor refits do not enter this randomised variant.

## Deterministic final interval and prospective sample

### DES-0149-INTERVAL-DECISION

Define X(j) as a weighted contribution to the exact estimator, and freeze
a proved valid range width R(j) for each contribution. Widths include the
estimator weight. Set S = sum of R(j)^2. Bounds come from the registered
producer and grouping and a qualified support argument. A historical
maximum, a convenience cap or a clipped observed outcome is not proof of
a bound on the possible values. A bound violation holds the result.

| Variant | Contributions and admitted dependence |
|---|---|
| Blocked ITT | One contribution per independently randomised stratum, X(b) = (n(b)/N) * d(b). If every potential group-day count rate is in [0,U], each stratum has width at most 4*U*n(b)/N. An independently proved tighter assignment range may be pinned. The stratum's expectation is its weighted assignment effect. The Neyman variance above is diagnostic only for this profile. |
| Matched DiD | One weighted sum of pair contrasts per registered independent dependence block, weights 1/P for P frozen pairs. A prior common [0,U] rate bound gives width at most 2*U*P(b)/P for P(b) pairs in the block, since the pre-period is conditioned on. Bound all within-block dependence through the block width. The conditional interpretation also needs the registered drift-bias bound. |
| Synthetic control | One daily post gap divided by 28. With a common [0,U] rate bound the width is at most 2*U/28. For a single time series require an independently qualified bounded-innovation model. For every admissible history, the average of the 28 predictable mean daily gaps must differ from the target effect by at most B. Each day's predictable mean conditions on earlier days and frozen inputs. The centred differences then form a martingale. Autocorrelation or ordinary model fit alone supplies no such proof. |

For the synthetic row, the condition applies to the average of all 28
predictable daily means. The per-day centred increment has its conditional
range width fixed by the registered support. The other two variants use
independence across their declared blocks. Hold if those assumptions cannot
be supported. Use Hoeffding's bounded-variable moment inequality and, for
the synthetic path, its conditional form applied in native-day order. These
assumptions give a conservative interval. Site and calendar identity
provide no evidence of daily independence. This profile may be infeasible
on ordinary sites. A more powerful interval needs its own qualification.

For positive rational per-claim alpha, find the least integer k in 1 to 128 with
2*(3/8)^k <= alpha. This safely bounds 2*exp(-k) since e > 8/3. If no k exists,
hold. With S > 0 choose radius h as the least nonnegative multiple of 1/10,000
whose square is at least k*S/2. Find that grid index by bounded integer
bisection using exact rational comparison. Start with low zero and high one,
doubling the high index until it brackets the threshold. Refuse overflow. For S
= 0 require independently proved deterministic zero widths throughout and use h
= 0. An observed sample variance of zero does not establish zero width.

The core interval is [effect-h, effect+h]. For observational profiles
add the signed bias allowance B to each radius as specified above.
Round the outer bounds outwards on the same grid. Compare strict
decision boundaries to the rounded outward result. The exact error
test uses the same bound and allocation. For the absolute distance between
the effect and the point null, set x = max(0, distance - B), with B = 0
for randomised analysis. A conservative two-sided p bound is
min(1, 2*(3/8)^min(128,floor(2*x^2/S))) when S > 0. For proved S = 0
it is zero for a strict separation and one at the boundary. Only a
strict lower bound above the positive useful margin and a valid effect
direction can support benefit. A p bound is a frequency guarantee under
the registered model and null.

Use the same proposed interval for deterministic power screening.
Write M for the positive useful margin, A for the registered planning
alternative in the beneficial direction, and B = 0 for the randomised
profile. Freeze S's prospective certified upper bound using the proposed
layout. Let d = A - M - h - 2*B - 1/10,000. If d <= 0 the plan is
infeasible. Otherwise let kP = min(128,floor(2*d^2/S)) for positive S.
The conservative power floor is 1 - (3/8)^kP. Require at least 4/5.
For proved S = 0 use power floor one only when d > 0. The calculation
covers an adverse bias shift and outward rounding. It assumes the
registered planning alternative and admissible support, and does not
promise exposure, complete rows or a realised effect.

For randomised recruitment, enumerate supported even stratum sizes from
the least eligible allocation, by increasing total groups and then
canonical allocation vector. Use the frozen pre-only eligible order,
existing 0094 allocation and QR-133/0066 prospective checks. Select the
first layout meeting the signed 4/5 floor and exact coverage and
traffic guards for the fixed 28 days. Refuse a space above 10,000
candidate layouts before using that solver. Existing 0094 implementations
must still qualify their own sample plan through 0066. For observational
profiles re-evaluate the exact eligible cohort after the pre-only fit
and before reading outcomes. Infeasibility holds the registered analysis
with no post-outcome redesign, rounding gamble or fallback method.

## Power, final decisions and false alarms

Each preregistered decision claim binds a stable ID, class, effect
orientation, estimand, null and alternative, exact rational margin or
margins, test and interval profile, power profile, and error allocation.
Choose the class before outcome access. The default registers a single
benefit claim. A family with more than one claim needs prospective
qualification and explicit error allocation. Orient benefit as positive.

| Decision class | Exact hypothesis and signed decision rule |
|---|---|
| `benefit` | Register M > 0. Null is effect <= M, alternative effect > M. A signed lower decision bound strictly above M can support the claim. |
| `harm` | Register harm magnitude H > 0 and oriented threshold -H. Null is effect >= -H, alternative effect < -H. A signed upper decision bound strictly below -H can support the claim. |
| `equivalence` | Register exact limits E_low < 0 < E_high. Null is effect <= E_low or effect >= E_high. Alternative is effect strictly between them. The qualified intersection-union decision requires both signed endpoints strictly inside the margins. |

For the corresponding one-sided test, use directed distance from its
null boundary. For benefit it is effect - M. For harm it is -H - effect.
For equivalence use effect - E_low and E_high - effect separately. Subtract
B and use x = max(0, distance - B). With S > 0 the one-sided p bound is
(3/8)^min(128,floor(2*x^2/S)). With proved S = 0 use zero on strict positive
separation after the allowance and one otherwise. Both equivalence tests
must pass at the claim allocation. The outward interval rule remains
mandatory even if the one-sided test passes.

A qualified claim requires its signed test at the allocated alpha, all gates
and the exact bounds rule above. An equality at a boundary is insufficient.
Record `claim_id`, `decision_class`, the registered margins and oriented
effect, `decision_lower`, `decision_upper`, interval and arithmetic versions,
and the error-budget receipt. These bounds are the exact rational values
after outward rounding on the registered grid, including any admitted bias
allowance. Seal them in the authoritative 0149 receipt for 0148 and 0151.
A read or Lawbook review uses those signed values and the qualified result.
It cannot use tighter intermediate bounds or recalculate the decision.
Display precision may be coarser without changing the evidence or outcome.
Hold an unqualified harm or equivalence claim instead of borrowing a
benefit power calculation or inferring it from a non-significant result.

Freeze a positive minimum useful effect in the primary metric's units, the
effect direction, error budget, final decision test and interval, baseline
variation and traffic proof, smallest planned effect to detect, and target
power. A direction where smaller values are better is expressed through a
registered sign. Use 0094 and QR-067/133 for randomised group-level sample
qualification. The observational profile separately validates interval
coverage and detection under admitted dependence, bias, placebo failures and
missingness. The 28-day window alone establishes neither power nor support.
Do not calculate post-hoc power from the estimated effect.

A supported worthwhile benefit needs a qualified lower bound strictly above
the registered minimum in the beneficial direction and all current gates.
An estimate near zero, a non-significant result, or an interval crossing the
decision boundary is inconclusive about worthwhile benefit. If
a qualified outward-rounded interval lies wholly below the useful benefit
margin and the only registered claim is benefit, 0148 can show that
interval with Inconclusive. Evidence against requires a separately
preregistered harm or equivalence rule that passes. A practically
negligible result requires a preregistered equivalence test,
independently justified margins and adequate power. The corresponding
interval must lie strictly within both margins.
Equivalence supports a scoped absence of a meaningful effect, without
asserting exact zero. Report harm only through its own registered decision
rule. A missing result, contaminated result or stopped exposure cannot
establish any verdict.

A plan chooses one primary variant and one final look. Other views are
descriptive unless registered as a decision family before any read. If a
family contains multiple formal hypotheses, including diagnostic flags with
p-values, freeze its complete membership and total budget in 0148. Allocate
nonnegative rational budgets with sum no greater than the approved total.
For a fixed family using Holm, compare ordered valid p-values at rank i with
the family's budget divided by (m - i + 1) for m registered hypotheses. Break
ties by canonical test key, stop rejection at the first failure, and reserve
the slots of held or missing tests without allowing them to reject. Use
simultaneous decision bounds from a fixed prospective budget allocation
or a separately qualified joint interval method. The observed Holm rank
alone does not define a confidence interval. One-sided equivalence tests
form one intersection-union claim within its allocated budget. Use 0094's signed
look budget for its randomised trial, with K = 1 here. Never recycle a
missed look, transfer failed alpha, or choose a favourable model afterwards.
Guardrails stop exposure regardless of efficacy and consume no efficacy
budget. One final receipt is immutable apart from separate supersession.

## Reproducibility and qualification

### DES-0149-CANONICAL-REPLAY

Use bounded checked integer counts and reduced rational arithmetic for
decisions, weights, exact means and fit comparisons. Pin the numeric
profile for variance, critical values and interval bounds. Use exact
comparisons and outward rounding to the registered rational grid. If
interval uncertainty at a boundary prevents a decision, hold the decision.
Overflow, indeterminate comparison or an unqualified interval routine
produces no partial verdict. Use canonical key order, stable tie tests,
explicit UTC and native-day versions, normalised fractions with positive
denominators, and 0148's canonical receipt framing.

Identical plan, versions, source bytes, assigned IDs, seed where used and
calendar produce the same canonical statistical payload and digest on each
admitted platform. Signature metadata, execution time and machine ID are
separate evidence. Cross-platform qualification compares canonical payload
bytes at ties, exact boundaries, supported maxima and one-over inputs.
Where an exact admitted numeric path is unavailable, hold local
recomputation. A separately admitted reference profile may display its
sealed and verified output read-only, without claiming an independent replay.

Before admission, freeze a finite validation manifest with 12 through
24 scenarios per claimed variant, including exact null, useful-margin
boundary, prespecified alternative, unequal group sizes, rare counts,
heterogeneous effects or dependence at the admitted edge, bias at its
allowed maximum, missing source, contamination and assumption failure.
Use exact analytical small fixtures and 2,000 independent seeded
replicates per stochastic scenario. Freeze all scenario data generation,
replicate seed IDs, expected supports and caps before the validation
run. Derive replicate nonces from the committed validation seed and
canonical scenario and replicate IDs under a namespace separate from
assignment. Missing or adverse scenarios stay in the validation ledger.

Let F be the number of frozen calibration assertions, at most 72 per
variant. At a null or
decision margin, verify the upper binomial tail-confidence limit for
false decisions is at most the allocated alpha. Equivalently for X
false decisions in 2,000 trials, the exact binomial CDF at X under
the allocated alpha must be at most 1/(100*F). At the planned
alternative, the exact upper-tail probability of at least the
observed successes under success rate 4/5 must be at most 1/(100*F).
Compute tails by exact bounded rational binomial sums with a separate
32,768-bit integer cap for validation. Keep the proof and seed set.
Admit only if all required assertions and refusal cases pass. Finite
validation checks the qualified envelope and cannot establish assumptions
on a new site. No calibration outcome tunes an existing plan.

For each variant, independent admissibility review pins source and metric,
foreseeable exclusions, interference, estimator assumptions, statistical
and arithmetic proof, bounded execution and fixture receipts. An
observational profile requires its own site-scoped qualification. The
shared formulas grant no data access and transfer no approval between
variants. Delayed, stale or revoked authority cannot reopen a result.

## Planned checks

| Evidence ID | Synthetic Given, When, Then |
|---|---|
| REQ-0149-ITT · UT-0149-ITT | Given unequal blocks with known exact group changes, compute the weighted effect in original arms. For the small arithmetic fixture use blocks of size 4 and 8 with effects 3 and -1 and expect 1/3. A switch of exposure preserves original assignment. A removed unit, unsupported dependence or unqualified interval holds. |
| REQ-0149-MATCH · UT-0149-MATCH | Given competing eligible edges with tied total cost, return the exact canonical optimal pairs. A foreign donor, unmatched frozen case, concurrent change or failed pre-validation yields a hold. |
| REQ-0149-SYNTH · UT-0149-SYNTH | Given a synthetic feasible donor grid with tied optima, obtain the exact canonical weight vector and final gap. Exact caps admit, one over holds, and timeout or post-outcome values cannot change or expose a fit. |
| REQ-0149-PLACEBO · IT-0149-PLACEBOS | Given exact randomised strata, reconstruct the four sham maps and three pseudo-starts, test missing untouched controls and a gap exactly at the margin. For each observational variant, check the exact fit windows, held coverage and treated-unit exclusion. A successful placebo never generates a causal approval. |
| REQ-0149-SENSITIVITY · UT-0149-SENSITIVITY | Given exact bounds with a raw interval beyond the useful margin and a widened interval crossing it, hold the conditional decision. For S = 1/8 and alpha 1/20 verify k = 4 and h = 1/2. At effect 3/5 and useful margin 1/10 verify equality holds. Unknown bias or unsupported dependence cannot become a pass. |
| REQ-0149-FINAL · IT-0149-FINAL | Given unsettled, final and corrected snapshots, verify one durable final claim and the exact deadline. Check exact claim recovery and authority withdrawal at the claim and outcome-read boundary. Exercise null, equivalence, missingness, contamination, Holm ties and unused budget. Recovery creates no second look or final transition and sends no provider request. |
| REQ-0149-REPLAY · IT-0149-REPLAY | Given the same canonical case on Linux and Windows, compare bytes and digests for all admitted variants. Refuse changed scope or seed, overflow and over-budget fit. Revocation during replay invalidates the receipt. |

## Readiness trace

Trace identifiers in this section bind the existing analysis and
evidence obligations. The cited sources explain their respective
assumptions and methods. Each site, method and evidence profile
still requires its exact independent admission under DR-143 and
DR-144. These synthetic fixtures exercise only their named scope.

| Requirement | Design and applicable variants | Source references | Fixture, existing check and evidence producer |
|---|---|---|---|
| REQ-0149-ITT | DES-0149-ALLOCATION-CONTRAST and DES-0149-ANALYSIS-BOUNDARY. VAR-0149-BLOCKED-ITT-DID. | SRC-0149-AUTHORITY, SRC-0149-ALLOCATION, SRC-0149-DESIGN-BASED, SRC-0149-ASSIGNMENT-NULL and SRC-0149-CHACHA | FX-0149-BLOCK-WEIGHTS, UT-0149-ITT. Rust arithmetic check and exact 0094 assignment manifest. |
| REQ-0149-MATCH | DES-0149-PRE-ONLY-MATCH and DES-0149-ANALYSIS-BOUNDARY. VAR-0149-MATCHED-DID. | SRC-0149-AUTHORITY, SRC-0149-PRETRENDS and SRC-0149-TREND-BIAS | FX-0149-TIED-PAIR-ORDER, UT-0149-MATCH. Rust matching check and independent same-site support review. |
| REQ-0149-SYNTH | DES-0149-PRE-ONLY-DONOR-GRID and DES-0149-ANALYSIS-BOUNDARY. VAR-0149-SYNTHETIC-CONTROL. | SRC-0149-AUTHORITY, SRC-0149-DONOR-PLACEBOS and SRC-0149-DONOR-SUPPORT | FX-0149-TIED-DONOR-WEIGHTS, UT-0149-SYNTH. Rust validation of the exact fit output and independent pre-support review. |
| REQ-0149-PLACEBO | DES-0149-EXACT-PLACEBOS. All three variants with their own registered cohorts and windows. | SRC-0149-AUTHORITY, SRC-0149-DONOR-PLACEBOS and SRC-0149-ASSIGNMENT-NULL | FX-0149-PLACEBO-MARGIN, IT-0149-PLACEBOS. Rust runner, exact source scope and independent diagnostic decision review. |
| REQ-0149-SENSITIVITY | DES-0149-INTERVAL-DECISION. VAR-0149-MATCHED-DID and VAR-0149-SYNTHETIC-CONTROL use the admitted bias bound. VAR-0149-BLOCKED-ITT-DID uses zero bias in this calculation. | SRC-0149-TREND-BIAS, SRC-0149-BOUNDED-SUM and SRC-0149-EQUIVALENCE | FX-0149-ROUNDED-MARGIN, UT-0149-SENSITIVITY. Rust exact arithmetic check and independent support and bias qualification. |
| REQ-0149-FINAL | DES-0149-ANALYSIS-BOUNDARY and DES-0149-INTERVAL-DECISION. All three variants. | SRC-0149-AUTHORITY, SRC-0149-SOURCES, SRC-0149-HOLM and SRC-0149-EQUIVALENCE | FX-0149-SOLE-FINAL-LOOK, IT-0149-FINAL and 0148 IT-0148-RELEASE. 0148 sole-claim receipt, 0150 source receipts and Rust final-bound evidence. |
| REQ-0149-REPLAY | DES-0149-CANONICAL-REPLAY and DES-0149-ANALYSIS-BOUNDARY. All three variants on each admitted platform. | SRC-0149-AUTHORITY, SRC-0149-REPLAY-CUSTODY, SRC-0149-CHACHA and SRC-0149-BINOMIAL-VALIDATION | FX-0149-EXACT-PAYLOAD-REPLAY, IT-0149-REPLAY. Platform runners and 0151 custody read-back of the exact numeric payload and digest. |

| Fixture | Exact synthetic input and expected check |
|---|---|
| FX-0149-BLOCK-WEIGHTS | Use an arithmetic-only case with strata of 4 and 8 groups, split equally between arms. Every pre mean is 10. In the first stratum use post means 13 in test and 10 in control. In the second use 9 in test and 10 in control. The exact effect is 1/3. A changed exposure observation preserves both original arms. A missing assigned group or unsupported dependence holds. The proposed admission still requires at least 8 strata. |
| FX-0149-TIED-PAIR-ORDER | In the focused matcher check, put ordered groups T1, T2, C1 and C2 in one admitted pre-fit class. Set all training and validation daily rates to 10. All four costs are zero. Select (T1,C1) and (T2,C2), and verify gap 1 with post rates 11 for T1 and T2 and 10 for C1 and C2. Remove C2 and hold full matching. Introduce a foreign donor or changed coverage and refuse its use. The proposed full cohort has its own larger admission limits. |
| FX-0149-TIED-DONOR-WEIGHTS | Give five ordered donors and the treated group a rate of 10 on every training and validation day. With Q = 10 choose numerator vector (0,0,0,0,10). Give the treated group post rate 11 and the donors 10 for exact gap 1. Check the grid count 19,448 at eight donors and hold a ninth. A timed-out partial enumeration cannot produce the winning weights. A post-outcome perturbation can change the gap only. |
| FX-0149-PLACEBO-MARGIN | Give eight ordered randomised strata with two verified untouched controls each. Pin the cutoff as native day zero. Verify pseudo-starts -112, -84 and -56 and the four low-bit and next-bit rotation maps. Give all pre rates zero, then set the first ordered control's final daily rate to 1 only in the first stratum. One sham absolute effect is 1/8. At useful margin 1/8 the check holds. Also run the existing observational tests with the exact earlier fit windows and with treated and pseudo-treated cohorts excluded from the donor pool. |
| FX-0149-ROUNDED-MARGIN | With S = 1/8 and alpha = 1/20, require k = 4 and radius 1/2. Effect 3/5 gives lower bound 1/10. At useful margin 1/10, equality holds. With effect 3/4 and admitted B = 1/5, the bounds are 1/20 and 29/20 and withhold the benefit verdict even though the raw lower bound is 1/4. Unknown bias or unqualified support holds the decision. |
| FX-0149-SOLE-FINAL-LOOK | Compare one tick before signed eligibility, exact eligibility, exact deadline and one tick beyond it, with otherwise identical mature inputs. Only the inclusive eligible range can admit the one claim, and a repeat keeps its exact identity. In separate fixtures hold the sealed pre-registration and all three mature source proofs fixed. Commit authority withdrawal before the claim or after the claim and before the matrix opens. Both abort and show no outcome read or verdict. The first has no claim and the second retains its one consumed look. In a still-authorised fixture interrupt the worker after it opens outcomes and before it stages a result. Resume under 0148 on the exact claim and pinned values within the unchanged deadline. The retry creates no new claim, provider request or second verdict. A changed source, confirmed overlap or later correction cannot give a replacement look. With M = 1/10 and qualified bounds [1/100, 2/100] against only a registered benefit claim, present Inconclusive and add no Evidence against entry. Use the existing equivalence and Holm fixture predicates separately. In two isolated synthetic source-native fixtures use the same registered primary GSC click matrix, admitted estimator, seed or fitted weights, assignment and decision margins. Give the other two independently qualified sources different time series and exact separate proofs, with all gates including required guardrails eligible in both fixtures. Compare only the canonical primary effect, authoritative interval and registered decision class. Both claims and custody bundles must bind their own exact differing source proof sets, even when those statistical fields match. Keep the registered guardrail definitions and their satisfied dispositions fixed. In another fixture fail a required guardrail with the same primary matrix and qualified producer scopes. Apply its existing release hold. In a separate eligible fixture, consume the claim and hold before the first outcome read. Try a same-number value from a corroborating producer in the primary matrix. Refuse under the changed-input boundary before reading a substituted matrix. No second look or released verdict follows. |
| FX-0149-EXACT-PAYLOAD-REPLAY | Replay the same canonical plan, matrix, seed where required, estimator and arithmetic profile on admitted Linux and Windows. Compare numeric payload bytes and digest independently of execution metadata. Try a changed variant, source digest or seed, overflow and one-over approved solver work. A revoked input or lease fences the output. Keep failures with the scope-safe reason. |

## Primary references

- SRC-0149-AUTHORITY. [ADR-0026](../adr/0026-scientific-experiment-authority.md)
- SRC-0149-ALLOCATION.
  [Spec 0094](0094-reporting-forecasts-proof-and-experiments.md)
- SRC-0149-SOURCES. [Spec 0150](0150-experiment-source-and-sensor-gates.md)
- SRC-0149-REPLAY-CUSTODY.
  [Spec 0151](0151-site-lawbook-calibration-and-replay.md)
- SRC-0149-DESIGN-BASED. [Aronow and Middleton, design-based unbiased estimators](https://joelmidd.github.io/papers/AronowMiddleton_A%20class%20of%20unbiased%20estimators.pdf)
- SRC-0149-ASSIGNMENT-NULL. [Caughey et al., sharp and bounded assignment nulls](https://doi.org/10.1093/jrsssb/qkad080)
- SRC-0149-PRETRENDS. [Roth, limits of testing pre-trends](https://doi.org/10.1257/aeri.20210236)
- SRC-0149-TREND-BIAS. [Rambachan and Roth, sensitivity to nonparallel trends](https://scholar.harvard.edu/sites/scholar.harvard.edu/files/jroth/files/honestparalleltrends_main.pdf)
- SRC-0149-DONOR-PLACEBOS. [Abadie, Diamond and Hainmueller, synthetic controls and placebos](https://web.stanford.edu/~jhain/Paper/JASA2010.pdf)
- SRC-0149-DONOR-SUPPORT. [Abadie, synthetic control feasibility and validation](https://doi.org/10.1257/jel.20191450)
- SRC-0149-HOLM. [Holm, sequentially rejective family error control](https://www.jstor.org/stable/4615733)
- SRC-0149-EQUIVALENCE. [Lakens, practical equivalence testing](https://pure.tue.nl/ws/portalfiles/portal/80918653/lakeequi2017.pdf)
- SRC-0149-BOUNDED-SUM. [Hoeffding, bounded-sum probabilities](https://www.cs.rpi.edu/academics/courses/spring06/random/hoefding.pdf)
- SRC-0149-BINOMIAL-VALIDATION. [Clopper and Pearson, exact binomial limits](https://doi.org/10.1093/biomet/26.4.404)
- SRC-0149-CHACHA. [RFC 8439, ChaCha20](https://www.rfc-editor.org/rfc/rfc8439)
