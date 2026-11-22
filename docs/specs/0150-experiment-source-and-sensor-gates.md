# 0150 Experiment source and sensor gates

Figures: every numeric limit, threshold and budget in this spec is proposed
until measured on the Dell or settled by a QR.

## What it does

This spec supplies versioned own-site observations to the local
[Scientific Experimentation Engine][parent]. An experiment freezes one
primary measure, the assignment unit and windows, and one required signed
profile each for Search Console, GA4 and darshak. All three must pass their
own checks for any verdict. A missing date, unproved scope or unsupported
profile holds the affected checkpoint. A diagnostic cannot supply another
producer's measure.

All scientific evidence comes through admitted APIs. Reuse 0040's exact
read-only Search Analytics and GA4 Data API integrations and the audited
local darshak aggregate interface under 0034. Each experimental profile
needs separate qualification. There is no manual import, BigQuery
connection, new connector, public endpoint, beacon or provider call from
Python. A fenced script receives only a pre-authorised immutable aggregate
projection from 0148, with no network, source handles or credentials.
Evidence and derivatives stay outside 0072 ranking, detection, method
research, promotion and release qualification. DR-124 and 0138 retain
their own advisory purpose, evidence gate and authority.

## Plan and source boundary

Before assignment, freeze a finite manifest of expected cells, each for
one assigned unit, source-native day, measure and qualified producer.
Use only admitted owned pages or frozen disjoint groups of them under the
parent assignment contract. Bind exact source unit identity, content and
exposure revision, property, credential scope, coverage, counting basis,
consent and disclosure. Reject ambiguous mappings, duplicate assignment,
hidden pages and mixed sites. A moved page, identity change, or new
property requires current exact proof. Never reassign old observations.

Pin three required producer profiles and designate one primary outcome.
The others supply their own required health and comparison evidence, with
their scope and measurement purpose explicitly stated. Freeze all checks
and scope for each before assignment. If a cross-source comparison is
required, prove that its source meanings, units, population and intervals
are compatible. A separately qualified source-native three-check profile
can attest each required source individually where comparison would mix
different measures. Pin that profile from the outset. Hold where neither
qualified mode exists. No one- or two-source verdict, silent substitution,
changed primary metric or moving baseline. A new scope needs a new plan.

Seal a fixed source_plan_digest in the 0148 registration. Hash the
canonical source plan under the approved versioned encoding. It binds
the expected cell manifest digest and the exact per-source and per-phase
plans in canonical key order. Each phase plan names its profile revision,
allowed units and measure, exact producer-native interval keys and UTC
bounds, source clock and time-zone database revision, coverage proof
required, earliest eligible watermark and wait limit. Mark unused phases
explicitly. A different cell, profile, phase, scoping rule, clock or
window needs a new plan. Measured source generations and signed
corrections are referenced by receipts and do not rewrite the frozen
source plan. Compare its digest before acquisition, gate approval,
final-look claim and result release.

The existing producer brokers hold source credentials. The engine reads
safe aggregates and protected receipts via the exact approved local action.
The 0019 scheduler runs bounded source acquisition only under its owning
admitted recurring or expressly approved request cases and 0040/0034
controls. Open the plan with one durable identity. Admit the finite cells
and the full upper bound of calls, bytes, rows, memory and time before
dispatch. Enforce the tighter of the producer's own limits and 0078
leases. At most one science pull may run per connection at once within
those limits. Request one logical effect per key under the owning retry
rules. Reuse eligible stored evidence where its scope matches. Typing, a
field preview, view, checkpoint, script, cache miss or export cannot
start a provider call. UI previews read only existing approved
metadata or saved evidence.

The producer validates eligibility, wire shape, native interval,
generations, coverage, units and disclosure before staging. For GSC
and GA4, resolve 0040's originating pull reference in the existing
cell source reference and bind its verified generation and applicable
coverage proof in the existing producer-phase receipt. Keep the
frozen expected cell manifest. A different generation does not supply
a current cell merely because its property, day, scope and value
match. Connector coverage remains subject to the source qualification
below. Commit only the exact local aggregate projection and safe
receipt. The checkpoint reader consumes sealed rows and evidence from
Alekh. A source exhausted
by quota, capacity or permission holds the affected required panel. There
is no fallback read, extra public-page work or partial result treated as
a complete cell.

## Evidence records

| Record and key | Required fields |
|---|---|
| Source profile (site, profile, revision) | Producer (gsc, ga4 or darshak), verified property or collector scope, allowed endpoint/action, metric/version/unit, exact filters and grouping, native clock and finality, population/consent, supported evidence and disclosure proof rules, limits, approved purpose, qualification receipt and expiry. |
| Source plan (site, experiment, plan revision) | Fixed source_plan_digest, primary profile, exactly three required profile revisions (one per named producer), canonical per-source/per-phase plan tuples, expected cell manifest and unit/exposure mapping digests, native windows, UTC bounds, maturity and wait rules and time-zone database revision, comparison mode, current authority, censoring and contamination policy reference, grant/policy/pause/erasure generations. |
| Cell (site, experiment, profile generation, unit, native day, measure) | Fixed source_plan_digest, exact phase plan key, canonical unit and source reference, native interval, verified producer generation or protected response revision, count or nullable numerator and denominator for eligible measure, units and attribution revision, coverage and source receipt, rights and disclosure generations, typed state and safe reason. |
| Pull receipt (site, plan, logical pull) | Fixed source_plan_digest and exact producer/phase plan key, canonical request and response digests, profile and permitted property, exact filter/group/date scope, native and application metadata, completion and coverage proof, scope/limits, safe accepted row counts, previous generation, grant/erasure and protected signer. |
| SensorCalibrationReceipt (site, experiment, checkpoint, revision) | Frozen 0148 plan digest, fixed source_plan_digest and expected manifest digest, phase, ordered producer_phase_proof_refs[3], sealed safe cell and pull receipt manifests, exact native intervals, per-source coverage and health, qualified comparison mode, contamination/correction references, decision (eligible, held or withdrawn) and authority generations. Each producer-phase proof reference binds producer, exact phase plan key, profile ID and revision, proof ID, revision, generation and digest, coverage proof digest and native maturity watermark. |

One composite `SensorCalibrationReceipt` belongs to each exact
site, experiment, checkpoint and receipt revision. Its three
references have canonical order: GSC, GA4, darshak. One such
reference attests the producer's exact frozen phase and the
applicable cell and pull receipt digests. One pull receipt
still attests one logical request. A producer can attest
incomplete evidence for its own phase. That reference can
support a hold only. A missing, duplicated or unidentifiable
producer prevents sealing a composite and holds the checkpoint
with the existing safe reason.

Cell state is measured, unknown, partial, suppressed, refused,
unavailable or withdrawn. The value exists only for an eligible measured
observation. Zero must be an explicit eligible producer value or a
separately qualified closed-universe zero proof for the exact cell.
A missing row, day, permission, classification or suppressed value is
never zero. Apply 0034's joint disclosure and release history, including
derived counts, complementary values, reasons and prior outputs.
Do not expose raw query terms, private paths, visitor/member/session IDs,
credentials, logs, response bodies or copyable custody pointers.

## Search Console profile

Use the existing 0040 Search Analytics path and a verified own property.
Pin provider Pacific day, surface, exact page canonical identity, metric,
request and response aggregation, filters and grain. Use equal inclusive
startDate and endDate for one planned day, the exact type, and dataState
final. Require qualified mature coverage for the exact cell. Page aggregation
is required for eligible page outcomes. The provider documents explicit
aggregationType byPage without a page filter or page grouping. An
unfiltered aggregate with neither a page key nor exact page scope
identifies no assigned page. If the request filters or groups by page,
use aggregationType auto as specified in the API reference. For an
exact-page profile send one case-sensitive page equals filter for the
verified canonical URI and verify responseAggregationType byPage.
A grouped page profile likewise needs byPage in the response and shows
observed rows only until independent completeness is proved. Refuse
byProperty or unknown basis for all page outcomes. Preserve query,
device, country and appearance series separately. Never use segmented
rows to reconstruct an unsegmented denominator or add overlapping
metrics. An experimental segmented outcome needs its own qualified
native profile.

Recognise separate exact API values web, image, video, news, discover and
webMultimodal. The existing 0040 integration enumerates only the first
five. Qualify webMultimodal independently before using it. Google
documents Lens, Circle to Search and image uploads under that type.
Multimodal text-query rows are unavailable. Keep googleNews (app and
news.google.com) separate from the news tab, and unavailable here unless
independently admitted. Do not assume disjointness for totals or merge
webMultimodal into web or image. Hold unsupported metric or surface
combinations.

A brand profile uses native brandedClassification BRANDED or NON_BRANDED
under the documented equals rule and exact admitted request shape. Google
documents data from 21 February 2026 for qualifying top-level properties.
A query filter cannot be combined with grouping or filtering by
brandedClassification. Prove the exact per-property, surface, period
and aggregation support separately. The provider label is a documented
classification, conditional on its proved coverage. Anonymised,
unclassifiable, truncated and missing data stays unknown. Do not classify
by scraped or user-supplied names, deduce from totals or mix a
multimodal slice with a brand result without a separately documented
and admitted compatible combination. Retain no query text.

A required panel needs an independently validated source contract for
each expected cell's exact population and page-day metric. Require an
authoritative provider guarantee or equivalent qualified producer
attestation bound to that exact request, proof scope and generation.
Proof must explain why any applicable masking, dropped detail and caps
cannot omit a required observation, including how zeros are established.
An attestation has to cite upstream guarantees or authoritative coverage
evidence for the exact populated scope. A local assertion that every
requested URI was tried or every returned row was saved does not qualify.
Neither final status nor a signed local receipt proves upstream
completeness. A present row, short page, canonical-page filter, repeated
successful calls, complete pagination, chart total or segmented sum
alone fails this test. The documented API disclaims all-row coverage.
Without an admitted exact proof, keep the report observations separate
and hold the science verdict. No bulk export, import or alternate route.

Use clicks and impressions in native units. Sum only proved disjoint
compatible populations. CTR uses clicks divided by eligible positive
impressions. Position, where separately eligible, uses the owning native
impression-weighted basis. A property's position or total does not become
a page-day value or arm count.

## GA4 profile

Reuse exact read-only 0040 GA4 Data API and verified property binding.
Pin the property-native date and time-zone revision, exact unit mapping,
retained metric/event ID, counting method, attribution version and scope,
filters and eligible population. Current 0040 daily landing-page by
session channel/device/country reports have generic key-event counts.
A selected key-event outcome needs a separate qualified profile. A new
key-event definition has no retroactive effect. A landing path is the
first pageview's path and gives no host on a multi-host property. Require
dedicated own-property mapping or qualified exact host scope. Organic
Search as channel gives no Google-only proof. Require a compatible
native source definition when Google-only scope is necessary.

Qualify the actual dimensions and metric combination. Do not use a
selected-event filter to silently change the denominator. A separate
denominator report requires its own exact compatible population proof.
For each native day bind the returned metadata, property and exact
request definition to the source receipt. Do not reuse a report exposed
under a different source, key-event scope or release generation.
Return actual eligible native values only. Require mature daily
observation under the profile's admitted finality and correction rules,
whole relevant native interval, source/report generation and proof all
required scoped cells are available. Preserve samplingMetadatas,
subjectToThresholding, dataLossFromOtherRow, timeZone, row count and
report scope in the protected receipt. In the initial exact profile,
sampled, thresholded, rolled-up (other), incompatible or omitted
evidence holds the affected panel. A completed paginated report or
freshness watermark alone does not prove a complete result. Keep late
corrections separate and fence dependent results before re-evaluation.

## Darshak profile

Use the existing local 0034 human aggregate with exact site, measurement
mode, closed window, page/thread revision and reader metric version.
Pin numerator, measured human denominator, eligibility and end-beacon
coverage, release scope, consent and erasure generation. A named
reader-depth outcome uses one exact admitted metric and population.
For example, a qualified band share divides its observed band count by
the eligible measured human denominator for that same profile. Disclose
its coverage against the eligible population. A missing end beacon
does not identify a shallow visit. Visible time alone does not prove
active reading duration. Anonymous and consented modes cannot substitute
one another. The added block reach or beacon fields in QR-221 remain
outside this profile.

Only the owning collector filters verified human, crawler and automation
classes before eligible aggregate release. It binds the exact population
and treatment. Do not subtract a suspicious burst from page/day or arm
counts. Do not make a visitor or session join to prove comparability.
Missing or incompatible end-beacon data, suppression or revocation
holds the affected observable outcome.

## Checkpoint health and custody

Recheck all three named sources before each frozen checkpoint. Healthy
FC-32 is a separate analytical diagnostic and cannot verify individual
cells, assignment balance, coverage or a compatible clock. Keep the
existing 0146 rule, including its ability to compare two eligible sources
and report Unchecked for unsupported comparisons. A diagnostic ratio
cannot identify ad blocking, consent loss, automation or the number of
missing key events. Show compatible discrepancies with their exact
source labels. Do not impute, correct, rescale or allocate counts to arms.
A source-native three-check profile is still three independently qualified
producer proofs, with no waiver or missing-source fallback. A required
unknown, broken or incomparable profile holds the verdict.

All three receipts remain part of the gate and claim. The
registered primary profile supplies the ordered unit and native-day
cells of the 0149 efficacy matrix. Other qualified source
values support the declared checks and stay in their own measures.
A registered guardrail keeps its separate release effect. Its
values and corroborating counts cannot fill or alter a primary
cell or change the estimator's registered measure. Refuse a
changed matrix or profile at the existing sealed hand-off before
opening outcomes, even when a substituted cell has the same number.

GSC uses Pacific days, GA4 uses property days, and darshak uses its
admitted native window. Construct exact instants from the pinned
time-zone database, including daylight-saving gaps and folds. Use
whole native intervals inside the approved plan windows. A date label
does not align intervals by itself. For required common windows,
incompatible or partial intervals hold. Retain separately labelled
source-native coverage only where the frozen profile supports it.
Within each source retain planned and observed cells separately.
Do not drop outage days, substitute another source or change look
timing after assignment. Use 0148's predeclared missingness, look and
contamination policy.

For every relevant phase, freeze each producer's last required native
interval end and earliest closing instant in UTC, according to its
qualified maturity rule. The parent uses the maximum of the frozen
outcome-window end and the three applicable source closing instants as
its earliest final-look eligibility. This is a scheduling bound.
At the actual claim every source must independently prove its current
watermark closes every required native interval. Check the exact source
scope and coverage. Lag, revision, daylight-saving differences or a
delayed proof cannot move the frozen schedule, extend the hard deadline
or turn a partial final day into full coverage. If the bound lies
outside the frozen deadline, hold with the exact timing reason. An
early source watermark cannot start a look ahead of the scheduled time.

Check current confirmed 0137 incident, applicable surface, dated window
and revision. Read the scoped half-open freeze in
[0048](0048-fix-list.md#site-work-acceptance-contracts-539550) and the
separately admitted 0147 reactive/settling policy only for their owning
purposes.
Pin which exact predeclared condition applies. Unclear exposure or
unavailable incident evidence holds the affected gate. Confirmed
contamination follows the terminal disposition in 0148 and cannot
be waived or overturned through this route.

A verified false alert can clear its scoped hold only before 0148's
atomic final-look claim and freeze, within the unchanged deadline.
Bind the original alert and receipt, the exact affected site,
footprint, surface, dates and frozen source_plan_digest. Require the
current authoritative evidence proving that exact alert was false, the
eligible independent reviewer's signed attestation of that evidence
and scope under the existing receipt authority, and the owner's
separate exact confirmation under 0148.
Retain the original and the authoritative correction generations with
separate lineage. Recheck all three required sources and the other
0148 gates before continuing the same frozen plan. Do not change arms,
windows, primary measure, threshold, deadline or remaining look.

Before any final efficacy read, unblinding or calculation, the
referee atomically seals the 0148 plan, sole look ordinal and
final input digest. Its existing `effective_checks_digest`
includes the exact current composite checkpoint reference,
receipt revision and digest. Its `source_receipts[3]` contains
the same three exact ordered producer-phase proof references
from that checkpoint. At hand-off, replay and release require
the exact claim, composite and producer references under
current authority. A replacement composite or producer proof
cannot satisfy the consumed claim, even if the replacement
composite references the same three producer proofs.
Before that claim, the broker may validate and stage protected inputs
and the referee may inspect gated coverage metadata only. Source values
cannot reach the evaluator, UI, MCP or script as an efficacy read first.
The claim and a false-alert clearance compare the same plan and evidence
fence. Recovery after the claim uses only the exact sealed attempt.
A correction then gives supersession or withdrawal as applicable, with
no recomputation under changed evidence or replacement look. An
unresolved alert stays held. No automatic freeze, change, publication
or causal attribution follows.

Use existing 0065 receipt authority, 0032 erasure, 0108 scope,
0118 protected storage and transport, and the shared file ledger.
Bind authority, revocation and producer generation before staging,
checkpoint commit, reading, hand-off, replay and restore. Keep raw
API response data in the producer's bounded protected parsing path.
Retain only eligible aggregate rows and safe custody receipts in this
engine. Exact replay is inert. Reconcile conflicting results under
the owning acquisition identity. A verified correction atomically
replaces the exact source generation and fences dependent checkpoints.
A source correction permits revalidation under the parent’s frozen
policy only before the atomic final-look claim. After the claim, record
supersession or withdrawal without recomputation or a replacement look.
A withdrawn generation first loses access. Managed removal follows the
owning ledger.

## User-facing outcomes

| Condition | Copy |
|---|---|
| One required source or page-day lacks complete proof | “More source evidence is needed for this checkpoint.” |
| Unsupported search or classification profile | “Search Console cannot supply that measure here.” |
| Immature, incomplete or corrected date | “This date is missing or still changing.” |
| GA4 scope or metadata mismatch | “This Analytics report cannot verify the requested measure.” |
| Darshak denominator or coverage hold | “There is too little eligible reading data for this measure.” |
| Separately observed source discrepancy | “The sources disagree. No count has been adjusted.” |
| Incompatible required windows | “The sources cover different time windows.” |
| Update or current applicable hold | “This checkpoint is held while the change is reviewed.” |
| Grant, pause or erasure fence | “This source is no longer available for this experiment.” |

Use the existing parent error ownership and qualified codes. Show only
safe source, coverage and date reasons to the current authorised audience.
A description, export or receipt view cannot dispatch, approve or refresh.

## Requirements and synthetic acceptance

Codes SUT-2300 to SUT-2305 are owned by spec 0094 (DR-087); this spec only
reports them.

| Requirement | Named fixture and outcome |
|---|---|
| REQ-0150-API | IT-0150-API-SCOPE. Admit only fake versions of existing exact producer routes. Deny wrong site, credential, profile, revoked grant, manual file, alternate connector and Python network. |
| REQ-0150-GSC | IT-0150-GSC-COVERAGE. Compare unfiltered byPage aggregate, grouped-page auto and exact-page-filter auto. Verify returned byPage and the exact unit, and refuse an unfiltered total as a page cell. Hold omitted/capped pages, final but partial data, a repeat, no row and unproved zero. A synthetic independent complete attestation supports its exact test scope only. In a separate case request final web data for two eligible assigned pages in one own property. The final byPage response contains one observed row and ends on a short page. Offer its local pagination completion mark as whole-population coverage without independent attestation. The separately authorised 0040 report keeps the observed row and marks the omitted page unknown. With the other two source proofs current, hold the missing cell under ERR-0150-CELL-PROOF and the checkpoint under ERR-0148-SOURCE. The refused evidence supplies no final-look claim or efficacy read. |
| REQ-0150-SURFACES | UT-0150-SURFACE-BRAND. Distinguish six claimed native surface names with exact supported contracts. Refuse unsupported multiplex, invalid brand property/start date, query filter with brand dimension, masked remainder and undocumented multimodal query inference. No query text retained. |
| REQ-0150-GA4 | IT-0150-GA4-KEY-EVENT. Fake exact event and compatible denominator. Change event definition, host, property, source or counting basis. Hold sample, threshold, other bucket, missing cell, late correction and generic Organic Search as proof of Google origin. In a separate case, the independently complete exact native page-day and event have an eligible explicit zero and a positive compatible denominator. Accept that zero. With the same eligible denominator, omit the event row without a separately qualified closed-universe zero proof. Keep the cell unknown. Restore the zero row and independently set `subjectToThresholding` and `dataLossFromOtherRow` to true in separate responses. Hold each affected cell without a numeric value under ERR-0150-CELL-PROOF. The omitted or suppressed cell supplies no qualified phase proof or final look. A retained separately authorised 0040 report may still show partial coverage. |
| REQ-0150-DAR | IT-0150-DAR-READER. Compare qualified completed human band count and denominator with missing beacon, another consent mode, automation, suppressed cells and grant withdrawal. Refuse block and visitor requests. In a separate branch give 0034 a synthetic visit whose eligible end beacon arrives after midnight within the existing close allowance. Confirm its start-day assignment and use only the wholly closed source window with a current classification watermark. For the exact frozen site, page revision, metric and consent mode, accept the current eligible band with its matching measured-human denominator proof. Keep those identities and numeric values equal and substitute the denominator proof from a superseded collector release for that same native day. Refuse the combined proof under ERR-0150-CELL-PROOF before computing or releasing a rate. Hold the checkpoint. A locally closed day and matching counts cannot establish a current eligible denominator. Consume no final look. |
| REQ-0150-GATE | BDD-0150-THREE-SOURCE. Pass the exact current composite with GSC, GA4 and darshak producer-phase proof references in order under the qualified mode and frozen source_plan_digest. Fail each source independently, all but one and all but two, a duplicated or reordered producer, wrong phase and changed expected manifest. An absent or duplicated producer cannot seal a composite. A complete composite with an ineligible producer cannot qualify. In separate before-claim branches keep the frozen web page-day manifest, equal synthetic values and current GA4 and darshak proofs. Try a current independently complete byPage GSC proof for image and an older complete web proof superseded by a newer producer generation. Neither proof supplies the required web evidence. A qualified current web proof revalidates only the original frozen scope. Then separately withdraw the exact GSC grant after staging the matching proof and before claim. The mismatches hold proof. Withdrawal follows the owning authority refusal. None of these failures admits a claim or outcome read. Healthy FC-32 cannot replace a failed cell. Hold a required incomparable cross-source profile and accept only a separately qualified, frozen all-three source-native profile. Join 0040 AT-CN25 and 0148 IT-0148-RELEASE in one race with two commit orders. Stage a qualifying three-source composite and prepare the final claim. Reselect the exact same verified GSC property with the same site, connection, report scope, day and synthetic values. Advance its source scope generation and keep the GA4 and darshak proofs current. If reselection commits first, the older GSC proof cannot support the prepared claim. Consume no look and open no efficacy read. If the claim commits first, record exactly one consumed look with its original composite and three proof references. Stage an otherwise eligible result from that claim while its original proof is current. Commit reselection before result release. Refuse release and any further efficacy read through the superseded proof. Offer a replacement proof whose fixture values are equal. Refuse its identity before reading or comparing those values. Keep the original claim and refuse any replacement look. Join 0149 FX-0149-SOLE-FINAL-LOOK for the primary matrix witness. Vary the two corroborating series in isolated eligible fixtures while keeping the primary matrix and analysis identical. Every source proof still gates. A same-number corroborating cell cannot replace a primary cell under a consumed claim. |
| REQ-0150-WINDOW | BDD-0150-WINDOW-TIME. Test native clocks, daylight-saving boundaries, partial windows, missing day, current and corrected 0137 incident and stale scoped freeze. Exercise IT-0148-CORRECT with source maturity bound across all three, exact independent attestation and owner confirmation, and a race against the atomic claim. Confirmed contamination, missing reviewer evidence and corrections after the claim cannot resume the study. Join 0137 IT-0137-FEED and IT-0137-TIME with the 0148 final claim. Stage a qualifying checkpoint while the current JSON incident interval is outside the exact frozen exposure. Accept a newer authoritative JSON revision of that incident whose scope and corrected interval prove overlap. Then deliver the delayed 304 for the old projection with an otherwise fresh Date and an Atom fallback from a separate timed-out JSON poll whose plain-text summary repeats the old dates. Neither renews the older incident confirmation, displaces the current JSON revision or clears confirmed contamination. Test both commit orders with all three sensor proofs otherwise unchanged and current. If the revision commits first, consume no look and open no efficacy read. If the claim commits first, keep its one consumed look and commit the revision before release. Refuse late release and any further efficacy read. Preserve the frozen plan and admit no replacement look. |
| REQ-0150-CUSTODY | FI-0150-EXACT-REPLAY. Interrupt before acquisition, commit, and checkpoint and erasure boundaries. Revoke during pending delivery, replay exact key, change scope under old key and replace a producer generation. Check a final-value read before the atomic claim. Try to resume an interrupted claim with a replacement composite that has the same three producer proofs, then with a replacement producer proof. Only the original composite and exact three references under current authority can satisfy the claim. No pre-claim efficacy read, duplicate logical request, stale access or replacement look. |

Use pinned synthetic non-identifying units, fake source brokers, fake
clocks, simulated authority and designed receipts. Include the source
fence, three required status vector, generation and checkpoint inhibition
in 0148's Quint model. Replay through the qualified Quint Connect Rust
test route on Linux and Windows Bazel profiles. Verify eligible receipt
mapping, no denied dispatch, no missing-as-zero cell, and no result from a
fenced generation. This is acceptance design. Each exact real source
profile remains subject to independent technical and authority proof.

### Source gate trace identities

Use the same site, phase, profile owners and safe outcomes throughout
the [0076](0076-specification-completeness.md) trace. The interface
identities below name the existing boundaries and their synthetic
checks. The producer's receipt and current authority remain necessary
to use any real profile.

| Source identity | Provenance and use |
|---|---|
| SRC-0150-AUTHORITY | [ADR-0026](../adr/0026-scientific-experiment-authority.md), DR-144 and DR-147 bind API intake, all three receipts, one primary measure and correction before the sole look. |
| SRC-0150-CONNECTORS | 0040 owns read-only Search Analytics and GA4 Data API access. 0034 owns the darshak aggregate action, measurement and release history. 0019 and 0078 govern acquisition and resource admission. [QR-231](../registers/questions.md) tracks independent evidence for the exact scientific source profiles. |
| SRC-0150-GSC-REQUEST | [Search Analytics query][gsc-query] owns native request and response fields, aggregation, dates and exact filters. |
| SRC-0150-GSC-ROWS | [Search Analytics guide][gsc-guide] owns row retrieval and the limits on returned detail. |
| SRC-0150-GSC-SURFACES | [Search Analytics API update][gsc-update] owns the documented search-surface and native brand contract in this design. |
| SRC-0150-GSC-PRIVACY | [Search Console query limitations][gsc-privacy] explains withheld and limited detail. The source profile must still prove the exact cells. |
| SRC-0150-GA4-FIELDS | [GA4 dimensions and metrics][ga4-schema] owns report-field meanings. The qualified profile binds their eligible combination. |
| SRC-0150-GA4-METADATA | [GA4 report metadata][ga4-meta] owns report flags, record and time-zone evidence. |
| SRC-0150-GA4-QUALITY | [GA4 reporting limits][ga4-quality] covers sampling, thresholding and rolled-up data. |
| SRC-0150-GA4-MATURITY | [GA4 data freshness][ga4-freshness] describes processing. The exact admitted maturity and correction proof governs a study. |
| SRC-0150-FENCES | 0148 owns the registration, checkpoints and claim. 0137 supplies current scoped incident evidence. 0065, 0032, 0108 and 0118 own receipt authority, erasure, scope and protected custody. |
| SRC-0150-DIAGNOSTIC | 0146 owns FC-32. Its separate diagnostic can describe discrepancies. DES-0150-THREE-PROOFS governs verdict eligibility. |

| Variant | Predicate, exact boundary and existing case |
|---|---|
| VAR-0150-GSC-EXACT-PAGE | One verified own-property canonical URI and one Pacific day. The case-sensitive page equals filter sends `aggregationType=auto`. Require returned `responseAggregationType=byPage` and exact cell completeness proof. IT-0150-GSC-COVERAGE checks those requirements, missing proof and unproved zero. |
| VAR-0150-GSC-GROUPED-PAGE | The request groups by page and uses auto. The response must use byPage. The authoritative returned rows remain observations until independently complete for the expected cells. The unfiltered byPage total has no assigned page key. IT-0150-GSC-COVERAGE exercises both shapes. |
| VAR-0150-GSC-BRAND | The qualified top-level property, surface and exact period support native `brandedClassification`. Equality on BRANDED or NON_BRANDED follows the native request contract. A combined query filter or incomplete coverage holds. UT-0150-SURFACE-BRAND checks the class and unavailable cases. |
| VAR-0150-GSC-MULTIMODAL | An independently qualified profile uses `type=webMultimodal` and its own native population. The profile supplies no multimodal text-query rows. UT-0150-SURFACE-BRAND checks the surface split and disallowed query and unsupported combinations. |
| VAR-0150-GA4-KEY-EVENT | One separately qualified event and verified property-native landing scope, complete native interval and compatible denominator. IT-0150-GA4-KEY-EVENT checks the exact definition, metadata, denominator and hold reasons. |
| VAR-0150-DAR-READING-BAND | One qualified metric version and closed window of the owning collector. Count the named completed band over its eligible measured human denominator and report eligible coverage. IT-0150-DAR-READER checks scope, beacon coverage, suppression, consent and revocation. |
| VAR-0150-THREE-COMPARABLE | The frozen three-source plan requires compatible source meanings, population, units and intervals for the declared comparison. BDD-0150-THREE-SOURCE and BDD-0150-WINDOW-TIME check an eligible case and the hold where comparison or maturity cannot be proved. |
| VAR-0150-THREE-NATIVE | The frozen qualified plan uses all three independent native checks. Each check must pass for its own purpose and exact scope. BDD-0150-THREE-SOURCE checks the eligible three-proof result and refuses substitution, missing proofs and a changed comparison mode. |

| Requirement | Design and existing synthetic proof |
|---|---|
| REQ-0150-API | DES-0150-ACQUISITION uses SRC-0150-AUTHORITY and SRC-0150-CONNECTORS. FX-0150-APPROVED-PULL binds IT-0150-API-SCOPE. A qualified exact request uses one owning broker and one bounded logical identity. Wrong site, manual intake, new connector and Python network yield no dispatch. |
| REQ-0150-GSC | DES-0150-GSC-CELLS uses SRC-0150-GSC-REQUEST, SRC-0150-GSC-ROWS and SRC-0150-GSC-PRIVACY. FX-0150-GSC-PAGE-BASIS binds IT-0150-GSC-COVERAGE. Separate the unassigned total, observed grouped page and exact page. A separately complete proof applies only to the pinned population and generation. |
| REQ-0150-SURFACES | DES-0150-GSC-DIMENSIONS uses SRC-0150-GSC-REQUEST and SRC-0150-GSC-SURFACES. FX-0150-GSC-SURFACES binds UT-0150-SURFACE-BRAND. Preserve web, image, video, news, discover and qualified webMultimodal as distinct populations. Keep googleNews separate and refuse any unqualified surface, branded or query combination. |
| REQ-0150-GA4 | DES-0150-GA4-CELLS uses SRC-0150-GA4-FIELDS, SRC-0150-GA4-METADATA, SRC-0150-GA4-QUALITY and SRC-0150-GA4-MATURITY. FX-0150-GA4-EVENT-BASIS binds IT-0150-GA4-KEY-EVENT. A complete event report must also prove native scope, filters and measurement compatibility. |
| REQ-0150-DAR | DES-0150-DAR-CELLS uses SRC-0150-CONNECTORS. FX-0150-DAR-HUMAN-BAND binds IT-0150-DAR-READER. Collector release and coverage bind the metric, consent, human band and exact denominator. Refuse foreign consent, missing end-beacon proof and block or visitor data. |
| REQ-0150-GATE | DES-0150-THREE-PROOFS uses SRC-0150-AUTHORITY and SRC-0150-DIAGNOSTIC. FX-0150-THREE-PHASE-PROOFS binds BDD-0150-THREE-SOURCE. The expected phase and manifest require one current qualified receipt per named source. A missing or duplicated producer prevents sealing the composite. A changed digest or failed required proof holds the gate. |
| REQ-0150-WINDOW | DES-0150-NATIVE-WINDOW uses SRC-0150-AUTHORITY and SRC-0150-FENCES. FX-0150-NATIVE-CLOCK-RACE binds BDD-0150-WINDOW-TIME and IT-0148-CORRECT. Test complete intervals on all three clocks, the frozen deadline and both correction and claim orderings. |
| REQ-0150-CUSTODY | DES-0150-CUSTODY-FENCE uses SRC-0150-FENCES. FX-0150-CLAIM-FENCE binds FI-0150-EXACT-REPLAY. A drop or revocation before seal gives no visible payload. Exact replay uses the original sealed manifest under current scope, with no second look. |

| Interface identity | Exact owner boundary, design and proof |
|---|---|
| API-0150-PRODUCER-READ | DES-0150-ACQUISITION uses the existing 0040 or 0034 producer under 0019. The owning broker holds credential and request authority. The study receives the eligible aggregate projection and scoped proof. FX-0150-APPROVED-PULL. |
| API-0150-PHASE-PROOF | DES-0150-THREE-PROOFS returns the shown `SensorCalibrationReceipt` to the 0148 referee for the exact plan and phase. The referee checks eligibility and current authority at activation, claim and release. FX-0150-THREE-PHASE-PROOFS. |
| API-0150-FALSE-ALERT | DES-0150-NATIVE-WINDOW binds the exact original alert, frozen affected scope, current authoritative proof, eligible independent signature and 0148 owner confirmation to the original plan fence. FX-0150-NATIVE-CLOCK-RACE. |
| API-0150-SEALED-HANDOFF | DES-0150-CUSTODY-FENCE uses the unique 0148 claim for final values. Before claim the referee can inspect scoped coverage. After claim the protected worker can read only its pinned eligible inputs. FX-0150-CLAIM-FENCE. |
| DB-0150-SOURCE-PROFILE | DES-0150-ACQUISITION binds the recorded site, profile and revision to the owner broker, exact measure, permissions and proof. FX-0150-APPROVED-PULL. |
| DB-0150-FROZEN-PLAN | DES-0150-THREE-PROOFS binds the site, experiment, frozen revision and source plan digest to the expected cells and their declared phase profiles. FX-0150-THREE-PHASE-PROOFS. |
| DB-0150-CELL-PROOF | DES-0150-CUSTODY-FENCE binds the exact logical pull receipt and eligible aggregate cell to producer, site, phase, native day, measure and source generation. FX-0150-CLAIM-FENCE. |
| DB-0150-SENSOR-RECEIPT | DES-0150-THREE-PROOFS binds one composite `SensorCalibrationReceipt` to the checkpoint revision and its three ordered producer-phase proof references. The final claim pins the composite within the checks digest and the three exact producer references under 0148. FX-0150-THREE-PHASE-PROOFS and FX-0150-CLAIM-FENCE. |
| UI-0150-CHECKPOINT | DES-0150-THREE-PROOFS and DES-0150-CUSTODY-FENCE project the shown safe source, coverage and date reasons to the current allowed audience. A view causes no fetch, repair or release. FX-0150-THREE-PHASE-PROOFS and FX-0150-CLAIM-FENCE. |

Use the inherited 0148 outcome and 0020 projection rules. These
failure identities name source-gate checks within those existing
outcomes. Check each trigger with UT-0150-REFUSAL-BOUNDARY and each
allowed presentation with BDD-0150-REFUSAL-RECOVERY, reusing the
existing synthetic cases and User-facing outcomes above.

| Failure identity | Trigger, existing effect, recovery and fixture |
|---|---|
| ERR-0150-PRODUCER-SCOPE | API-0150-PRODUCER-READ refuses foreign site, unapproved purpose or withdrawn grant before intake. Apply ERR-0148-AUTH under 0020 without object disclosure. Recheck current authority under the frozen scope before any further admission. FX-0150-APPROVED-PULL. |
| ERR-0150-CELL-PROOF | DES-0150-GSC-CELLS, DES-0150-GA4-CELLS and DES-0150-DAR-CELLS retain the exact protected reason for unknown, partial, suppressed, refused, unavailable or withdrawn proof. Before claim an ineligible required cell gives ERR-0148-SOURCE and SUT-2300 under authorised scope. Preserve the typed value absence. FX-0150-GSC-PAGE-BASIS, FX-0150-GA4-EVENT-BASIS and FX-0150-DAR-HUMAN-BAND. |
| ERR-0150-PHASE-PROOF | API-0150-PHASE-PROOF finds an absent, duplicate, expired, incompatible or wrong-phase required receipt. Hold under ERR-0148-SOURCE. Only the original eligible scope can be rechecked within its frozen deadline. FX-0150-THREE-PHASE-PROOFS. |
| ERR-0150-WINDOW | DES-0150-NATIVE-WINDOW finds incompatible or partial native intervals, a not-yet-mature interval or maturity beyond the frozen deadline. The source hold permits no early read or deadline extension. FX-0150-NATIVE-CLOCK-RACE. |
| ERR-0150-CORRECTION | API-0150-FALSE-ALERT cannot clear a hold without current independent proof and exact owner approval before claim. Confirmed contamination follows 0148. Keep the existing hold. An eligible post-claim correction can supersede or withdraw through 0148. FX-0150-NATIVE-CLOCK-RACE. |
| ERR-0150-EVIDENCE-FENCE | API-0150-SEALED-HANDOFF rejects a changed claim manifest, a replacement composite or obsolete producer evidence under the existing 0148 stale boundary. Pause or withdrawal prevents further release under the owning fence. Reconcile only the original authorised identity. FX-0150-CLAIM-FENCE. |

| Formal obligation | Required witness |
|---|---|
| PRP-0150-EXACT-PHASE | DES-0150-THREE-PROOFS requires one exact current proof per named producer. FX-0150-THREE-PHASE-PROOFS witnesses one missing proof, a repeated source a wrong phase, a substituted exact scope or generation and a withdrawn producer grant before claim. Model these as distinct proof and authority identities. The owning integration fixtures check provider completeness and the canonical request scope. |
| PRP-0150-NO-IMPUTATION | DES-0150-GSC-CELLS, DES-0150-GA4-CELLS and DES-0150-DAR-CELLS retain value absence with typed reasons. FX-0150-GSC-PAGE-BASIS, FX-0150-GA4-EVENT-BASIS and FX-0150-DAR-HUMAN-BAND witness each route refusing own-source missingness as zero. |
| PRP-0150-ONE-CLAIM | DES-0150-NATIVE-WINDOW and DES-0150-CUSTODY-FENCE restrict efficacy reads to the exact sealed attempt. FX-0150-NATIVE-CLOCK-RACE and FX-0150-CLAIM-FENCE witness both orders of the correction race, pre-claim refusal, a replaced producer proof and a replaced composite under the same producer references. |
| PRP-0150-NO-LATE-RELEASE | DES-0150-CUSTODY-FENCE refuses affected hand-off and release under the current pause, grant and erasure fences. FX-0150-CLAIM-FENCE witnesses a staged old-generation result rejected at hand-off and restore. 0151 FX-0151-REPLAY-CUSTODY witnesses supersession after seal while an independent 0040 report grant stays current. Check that the report grant cannot restore scientific eligibility. |

## Primary source evidence

Checked 21 November 2026. The public manuals specify protocol, native
source dimensions and known limits. They provide no experiment data or
project admission.

[Search Analytics query][gsc-query] · [Search Analytics guide][gsc-guide] ·
[Search Analytics API update][gsc-update] ·
[Search Console query limitations][gsc-privacy] ·
[GA4 dimensions and metrics][ga4-schema] ·
[GA4 report metadata][ga4-meta] · [GA4 reporting limits][ga4-quality] ·
[GA4 data freshness][ga4-freshness]

[parent]: 0148-scientific-experimentation-engine.md
[gsc-query]: https://developers.google.com/webmaster-tools/v1/searchanalytics/query
[gsc-guide]: https://developers.google.com/webmaster-tools/v1/how-tos/all-your-data
[gsc-update]: https://developers.google.com/search/blog/2026/11/search-analytics-api-update
[gsc-privacy]: https://support.google.com/webmasters/answer/17011259?hl=en
[ga4-schema]: https://developers.google.com/analytics/devguides/reporting/data/v1/api-schema
[ga4-meta]: https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/ResponseMetaData
[ga4-quality]: https://developers.google.com/analytics/devguides/reporting/data/v1/reporting-data-expectations
[ga4-freshness]: https://support.google.com/analytics/answer/11198161
