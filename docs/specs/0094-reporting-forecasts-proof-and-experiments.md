# 0094: Reporting, forecasts, proof and experiments

See what changed, what might happen next and what a fair test could
establish. Reports combine approved evidence from the active site.
Spec 0034 owns visit analytics. Spec 0040 owns copied search data.
Spec 0043 owns charts, spec 0048 owns the Fix list and goal alerts,
and spec 0066 owns the shared maths. Spec 0100 owns delivery and file
safety; this spec owns the report snapshot master, a fixed record
from which the report files are made.

DR-087 permits local files for manual upload to Google Slides,
Google Sheets and a data warehouse. Sutradhar makes no direct
connection to those services. PDF output waits for QR-039.

Fictional example. A cycle forum tests clearer titles on one half of
a group of guides. Each guide has one title for everyone. The report
separates the randomised result from the traffic forecast. The owner
exports a fixed copy for a meeting and uploads the slide file by hand.

## Words used here

- **Metric** means a number with a stated calculation and population.
- **Snapshot master** means one saved report revision with its evidence.
- **Control** means comparable pages or sections left unchanged.
- **Guardrail** means a measure with a limit that can stop a test.
- **Cohort** means a group defined by the same event and time window.

## What this spec designs

| Feature | What the owner or staff sees | Requirement |
|---|---|---|
| 20 Before-and-after proof | Matched comparison with an interval | REQ-0094-PROOF |
| 21 SEO split testing | Fixed title or description per randomly assigned page | REQ-0094-ASSIGN |
| 22 Weekly top three | Up to three evidenced Fix list actions | REQ-0094-PLANNING |
| 23 90-day forecast | Traffic ranges with and without queued changes | REQ-0094-FORECAST |
| 24 Search demand while editing | Local suggestions from a loaded editor map | REQ-0094-EDITOR |
| 305 Scheduled reports | Local snapshot and approved email report | REQ-0094-DELIVERY |
| 306 White-label reports | Owner-approved branding for exported copies | REQ-0094-SNAPSHOT |
| 307 Executive summary | One local screen with coverage and priorities | REQ-0094-SNAPSHOT |
| 309 Google Sheets export | Compatible local table file | REQ-0094-EXPORT |
| 310 Data warehouse export | Local table file and schema | REQ-0094-EXPORT |
| 311 Search registration attribution | Admitted aggregate evidence of search arrivals | REQ-0094-PRIVACY |
| 312 Search traffic value | Estimate based on owner-supplied unit values | REQ-0094-PRIVACY |
| 313 Anomaly detection | Admitted metric checks and reasons when absent | REQ-0094-METRICS |
| 315 Search member cohorts | Admitted aggregate groups only | REQ-0094-PRIVACY |
| 316 Report template library | Versioned reusable layouts | REQ-0094-SNAPSHOT |
| 317 Data freshness indicators | Per-source observation time and state | REQ-0094-METRICS |
| 318 Report comments | Local notes pinned to a report revision | REQ-0094-SNAPSHOT |
| 319 KPI glossary | Definitions, units, scopes and limits | REQ-0094-METRICS |
| 320 Experiment registry | Signed plan, assignment, outcomes and history | REQ-0094-PLAN |
| 321 Sample size calculator | Proposed units and duration with assumptions | REQ-0094-STATS |
| 322 Sequential testing statistics | Planned interim reads with stopping rules | REQ-0094-STATS |
| 323 Holdout sections | Whole sections withheld from the tested behaviour | REQ-0094-ASSIGN |
| 324 Section feature flags | Scoped, reversible switches for admitted operations | REQ-0094-ASSIGN |
| 325 WordPress template experiments | Pre-approved, fixed page templates | REQ-0094-ASSIGN |
| 326 Experiment idea generator | Draft ideas from eligible findings | REQ-0094-LEARNING |
| 327 Experiment learnings library | Versions of successful, null and stopped results | REQ-0094-LEARNING |
| 328 Guardrail metrics | Stop on breached or missing safety evidence | REQ-0094-GUARD |
| 329 Experiment conflict detector | Refuse overlapping or incompatible plans | REQ-0094-GUARD |

## Reports and local files

1. Seal one master per report revision. Include site, time zone,
   template and calculation versions, source versions and coverage,
   labelled metric values, assumptions, sections and safe chart data.
   Save its hash. Use this same revision for the screen and exports.
   The summary shows evidence, open actions and missing measures.
2. Reuse the metric dictionary in spec 0100. Each result names unit,
   population, window and freshness. Show current, stale, partial
   or unavailable under its source contract. A missing value stays
   unknown. Spec 0066 supplies admitted anomaly results. Mark
   ineligible metrics and missing or stale inputs in the glossary
   and report. No comparison can combine unlike definitions.
3. Reuse spec 0048 report comments, pinned to a local report revision
   and actor. Comments stay local. Templates version their
   sections, definitions and owner-supplied branding. Branding
   changes presentation only. Exports use the approved projection
   of the master, the subset that can safely leave the app.
4. Spec 0100 owns the single self-contained, escaped HTML renderer,
   file ledger, safe export rules and sensitive-data checks. Derive
   formats from one allow-listed projection through spec 0032.
   Omit private evidence,
   comments, sensitive provenance and sensitive fields. Recheck
   current permissions and erasure on render, retry and send.
   Use owner-only files, neutralise CSV formula cells and forbid
   external references, scripts and active content in every format.
   Propose PPTX for Slides, CSV for Sheets, and CSV or Parquet with
   a schema for warehouses. QR-067 must admit compatibility and
   safe renderers. Record master hash, export version and omissions
   in the local ledger. Warn before export that shared copies
   cannot be recalled.
5. Use spec 0019's schedule rules and 0099's approved send step.
   Seal once per planned slot. A replay of an expired or erased
   master shows unavailable and sends nothing. A missing time moves to the
   first valid instant. A repeated time fires at its first
   occurrence. Catch up at most one missed slot unless set to skip.
   Use specs 0100 and 0111 for optional email reports. The default
   body is Headline. Higher levels require the existing owner
   switches and recipient approval. Attach only an expressly
   approved, admitted format and projection within the same
   disclosure and size limits. PDF waits for QR-039. Recheck
   current approval at send. Track mail outcomes apart from the
   sealed file. Uncertain receipt never starts an automatic resend.
   Scheduler readiness follows QR-081.

### Local Slides report

DR-087 assigns this local report format to 0094. The candidate
is `VAR-0094-SLIDES-PPTX` under `DES-0094-FILES` and
`REQ-0094-EXPORT`. Hold its offer and positive task derivation
until QR-067 and QR-132 admit the exact profile and spec 0076
admits the work package. A file extension supplies no proof.

Bind one frozen report master, approved field projection,
template revision, renderer and package profile, ordered slide
plan, exact local destination, rights and erasure generation.
The slide plan keeps each selected section, title, metric value,
unit, date, coverage label and omitted-item note in its approved
order. Preserve required report text as editable text in the
target, including any approved visible source label. Include a
permitted local chart only under an admitted representation and
retain its approved labels and values. Omit unselected fields,
private comments, notes and sensitive provenance. Reuse the approved
file effect in 0100 and the 0032 and 0118 rules for staged bytes,
receipts and current disclosure.

The package profile names exact conformance, allowed part and
relationship types, fonts and embedded assets, resource limits and
the renderer and independent checker with their versions, hashes
and licence proof. The checker verifies the package reached through
admitted protected staging. Bind the checked PPTX payload digest and
its protected storage record to the same action. Refuse unsupported
parts, scripts, macros, external relationships,
linked sources, unknown document fields and out-of-profile sizes.
Bind output and profile digests to the one original ledger effect.
Missing proof or a changed master, profile, scope, grant or
destination holds the proposed export. Reopening or replay with
erased sources supplies no new file or receipt.

Use `SRC-0094-SLIDES-PACKAGE` for published presentation markup and
packaging rules. Its catalogue lists 5th edition Part 1 (2016)
and Part 2 (2021). Pin the exact applicable part editions and
conformance class in the profile, including any applicable
compatibility or transitional rules. Use `SRC-0094-SLIDES-OPEN`
for the published web route to open a PowerPoint file with Google
Slides in Drive. Pin the source section, retrieval date and mode.
Record a publisher revision where available.
These sources establish format and target claims within their
scope. Candidate compatibility needs the exact import witness.
[Package standard][report-slides-package] and
[manual opening instructions][report-slides-open].

For `FX-0094-SLIDES-MANUAL-IMPORT` in E2E-0094-EXPORT, prepare one
synthetic frozen master with approved text, metric labels and
values, units, missing coverage, a safe chart and excluded
private fields. Independently inspect the staged package.
Check exact ordered content, absence of omitted data and absence
of outbound requests in the product and automated fixture.
The separately authorised human qualification uses that same
synthetic file digest in the documented manual target. Record the
dated target and observed import mode, reviewed fields,
legibility, text editability, permitted losses and exact outcome.
A source citation or offline validation alone supplies no
compatibility receipt. Test wrong profile, a planted active or
external relationship, an over-limit package and erasure or grant
withdrawal before publication. Each eligible approved export gives
one protected local copy with the matched ledger receipt. An
unavailable profile has no export. The owner uploads manually
outside Sutradhar.

[report-slides-package]:
  https://ecma-international.org/publications-and-standards/standards/ecma-376/
[report-slides-open]: https://support.google.com/docs/answer/9310378

### Reporting actions (305–319)

Browse and select authorised templates for the active site.
A clone gets a new site-owned ID. Saving an edit increments its
revision. The report master freezes the approval, template revision,
metric definitions, data sources, item evidence and branding refs.
A template or item ref grants no permission. Each template names
title, section order, metric IDs, sources and chart or table views.
A scheduled run
pins one template, closed window, active site grant and recipient
policy at its slot. Key the master by site, schedule ID and version,
and planned UTC slot. Key each delivery separately by the master,
recipient, approved level, projection and attachment digests.
Check permissions, policy, expiry and erasure at publication and at
send, including after catch-up or quiet hours. An uncertain mail
outcome stays unknown and needs review. It never acks or resends.
PDF renders only under the admitted QR-039 profile.
Editing or deleting a template does not rewrite old masters. Erasure
or source withdrawal still closes the affected masters and exports.

The local executive summary reads the same frozen measures and Fix
list evidence as the report. List coverage first, then current
supported findings, then up to three supported actions in common
units, with spec 0048 ordering. Show “No supported action for this
period” when appropriate. Brand names, colours and admitted logo
assets affect presentation only. Revalidate current site and rights
for each exact asset revision at render. Invalid or erased branding
gives a labelled plain render with its own output digest and receipt.
Never overwrite a prior file or hide source, time, units or coverage.

Stage each whole export under the protected ledger. The receipt
binds master hash, format and glossary versions, data and schema
digests, projection and erasure generation. Recheck the current
grant before atomic publication of the whole bundle. Revoke data
and schema together. Sheets gets local UTF-8 CSV under the admitted
0100 profile. Preserve leading zeroes for textual IDs in the bytes.
Use 0100 text-cell neutralisation and record it in the manifest.
Warehouse CSV and Parquet bind the same exact versioned schema,
with types, units, native ISO dates and nullability. The transport
profile distinguishes null, empty text and zero. Unadmitted exports
display “This file format is unavailable”. There is no upload route.

Report aggregates use the consented 0034 joint projection only
where admitted. For each completed registration bind the last
eligible human search arrival on the same site in the inclusive
seven elapsed days ending at registration. Count one per account at most,
with stable receipt ties. Use registration native day for 311,
and the attributed arrival's native day for 315. Require consent
at both events and on release. Keep identifiers within 0034 and
use no identity inference or later linking. Send only bounded
site group aggregates with observed coverage. Late correction or
withdrawal replaces the affected group generation and drops prior
results from reads. If the difference cannot be recomputed safely,
withhold the group.
Suppress groups, shares, totals and complements when the approved
privacy floor cannot be proved, including across saved filters
and report revisions. Show attribution and cohort routes as
“Not measured” while the exact joint projection is unavailable.
Traffic value uses one eligible visit unit and one owner's signed
rate revision in currency per visit. Its effective native-day interval
must cover the entire window. If multiple rates apply, show separate
eligible subperiods only where daily coverage proves exact overlap.
Store rates as non-negative decimals with up to six fraction digits.
Use exact multiplication and half-even rounding to the currency's
minor unit once per displayed estimate. Show count, unit price,
currency, effective date and “Estimated traffic value”. A measured
zero is zero. Missing, mixed-unit or stale inputs yield unknown.

Read admitted anomaly results from the owning 0043 or 0048
metric job using 0066 methods. Pin metric and method versions,
closed baseline, measured value, expected band and reason.
No registered job or incompatible, partial or stale evidence yields
“Anomaly check unavailable” with its reason. The linked chart
uses the exact 0043 filters and as-of generation. Record native
interval, observation time and source watermark in each master.
Show frozen freshness and current staleness separately.

For F-0313, open the source chart from a saved anomaly only with
the original master, site and exact result version, native interval,
metric and method profile, filters, source generation and approved
coverage. Recheck current read and archive rights. Use an already
admitted 0043 read of that exact retained generation. Date the view
as the report's original chart evidence. If that generation is
missing or no longer eligible, show “The chart used for this
report is unavailable.” A newer observation for the same date
and filters cannot replace the linked point or revise the saved
finding. The report may still show its dated result when its own
evidence qualifies. No new history reader, ordinary current chart
redirect or source pull follows from this link.

Source withdrawal revokes cached projections and managed files
as well as queued work. Published external copies follow 0100.

Each local comment binds site, report revision, author, comment ID
and report-review scope under 0048. Client read-only access has no
comment visibility. Recheck the reader's grant, including on replay.
Comment text stays out of logs, exports and mail. Report glossary
rows have metric ID and definition version, plain label, unit,
formula or source-provided meaning, population, day basis, and
coverage rule. An unknown or retired measure shows its historical
definition if still authorised. It cannot silently change unit.

## Evidence and planning

1. For before-and-after proof use equal predeclared windows and similar
   unchanged pages from the same site and metric source. Match using pre-
   change data. Date the change from spec 0048's journal or show the observed
   interval for an outside change. Show effect estimates, intervals and gaps.
   A changing control or incompatible pre-trend makes the result muddied or
   unavailable. Observations support association. Only an admitted randomised
   plan can support a causal result in this route. The separately
   observational route in spec 0148 can issue its own conditional
   estimate under ADR-0026 and 0149.
   It uses the 0150 source gates and cannot upgrade a result from this
   report. A 0148 randomised plan uses this spec's assignment, moderator
   exposure and stopping authority.
2. Reuse the organic-click baseline in ganit jobs 204 and 212
   for the original 90-day window. Admit a conditional queued-change
   scenario under QR-067. Show ranges, as-of date, assumptions
   and input and method versions. Without admitted effect evidence,
   keep a supported baseline and mark the scenario unavailable.
   QR-067 requires interval calibration and backtests.
3. Once per site week show up to the three Fix list actions with
   highest supported expected impact, in common units and with
   stable ties. Use the evidence and confidence from spec 0048.
   Show fewer when evidence is missing or scopes cannot be compared.
   A click queues a review. It never applies the change.
4. Reuse spec 0029's admitted editor route. Load a bounded map for
   the authorised author or moderator before typing, using approved
   aggregate demand under spec 0040. Omit private terms. Each
   keystroke uses the local map without a request. Stale or missing
   evidence, or a withdrawn grant, closes the demand view.
5. Registration attribution and cohorts require consented joint arrival and
   registration aggregates under spec 0034. This route needs admission under
   QR-067. Until then show "not measured". Keep separate totals and added-link
   attribution out of it. Apply the source minimum group size, suppression
   across totals and access limits to every display and file. Search traffic
   value multiplies an eligible measured count by the owner-set value for that
   unit. Show the currency, input and date and label it an estimate, without
   treating it as revenue or adding unlike currencies.

## Experiments

1. Register a versioned hypothesis, eligible units and scope, one
   versioned primary metric, guardrails, duration, sample method,
   stop rules and rollback. Freeze the plan and eligible units
   before assignment. Analyse units in their original assigned group.
   Compute power and duration from baseline variation, minimum
   effect, desired power, error budget, assignment unit and traffic
   bounds through spec 0066. Abstain if no valid inputs. Sequential
   testing uses a predeclared number and timing of checks and an
   admitted cumulative error rule. Missed checks, dropouts, outside
   changes or contamination follow predeclared handling. Unresolved
   evidence or expiry yields inconclusive or muddied.
2. Randomise stable whole pages or sections before exposure. Keep
   the assignment and seed fingerprint. The same page has the same
   version for readers and crawlers. Holdout sections keep the
   agreed baseline. Link hold-backs reuse spec 0029's opt-in, shown
   cost and immediate moderator release. Exclude their evidence from
   model fitting and ranking qualification. Section flags apply
   only to admitted actions.
   A WordPress template test selects pre-existing approved templates
   through the server-side add-on. Unsupported sites or actions
   stay advice only. QR-067 owns assignment, interference and
   platform admission for each variant.
3. Reserve scope atomically across time, pages, sections, templates
   and shared dependencies. Refuse conflicts before any apply.
   Use spec 0020's live owner confirmation on the exact site, plan,
   interval and original versions. Preserve moderator approval on
   affected content. Spec 0021's signed conditional request applies
   once and retains original versions for undo.
4. A breach, missing guardrail data, revocation or conflict stops
   exposure. The add-on uses an admitted exposure fence and expiry.
   A stale, offline or partial apply must serve the safe baseline
   without waiting for the PC. Fence queued writes and refuse delayed
   replies. Undo is idempotent and conditional on the original plan
   and current version. Never overwrite an intervening edit. Show
   failure or conflict. Adoption needs separate approval.
5. Suggest experiments from qualified local findings. Suggestions
   cannot start tests. Keep results, including inconclusive and
   stopped outcomes, with the plan, coverage, intervals and reason.

### Experiment controls (320–329)

An experiment has a stable experiment ID, site and immutable revisions.
A plan binds the declared test question, active site and scope,
time interval in UTC, native day basis, pages or whole sections,
primary metric version, guardrail plan, assignment and analysis
profiles, traffic and sample targets, and approved rollback
profile. Its receipt also binds before versions, expiring owner
and moderator approvals, dependency footprint and hash.
Track proposals, authorised plans, exposure and outcomes separately.
Any material change after assignment makes a fresh plan and leaves
the prior assignment and outcome intact. A withdrawal or correction
revokes evidence without starting a replacement. Freeze the exact
ordered native units and descendant manifest before assignment.
Sign its digest, ratio, strata, pinned deterministic assignment
method and operating-system seed commitment. Store the encrypted
seed for replay. Record moves, refusals and dropouts as exposure
deviations. None changes the originally assigned unit or group.

Use an admitted 0066 sample profile with the frozen metric type,
smallest worthwhile difference, baseline variation or event rate,
planned false alarm rate and target power, the chance of detecting
that difference when it exists. The calculator gives required units
per arm and elapsed days. Round both up. Use whole assignment
units and eligible daily traffic bounds. Where assignments are
whole sections, require admitted cluster count, sizes, correlation,
between-section variation and traffic in each arm. Refuse to treat
page views as independent sections. Zero or insufficient eligible
traffic yields unknown. First require an eligible frozen bound for
each planned arm at the approved duration cap. A missing required
bound makes the estimate unavailable. Then compare each arm with its
own rounded whole-unit target. Any shortfall, including a proved
zero bound for one arm with otherwise eligible traffic, makes the
plan infeasible under the saved assumptions. Equality meets the
arm target. Record inputs and the signed profile separately from
the activation plan.

Sign the null, direction, test and interval profile, worthwhile
effect, cumulative whole-unit targets, look deadlines and exact
rational total error budget before assignment. The profile must
prove per-look validity for the assignment and censoring rules,
including clustering and look timing. For K declared checks counting
the final check, compare p to total alpha divided by K, inclusive.
An efficacy result also needs the agreed effect direction and size
and all current guardrails. Guardrails can stop exposure regardless
of efficacy and use none of its budget. An expired or incomplete
check is missed and never recycled. Commit one frozen read, result
and budget receipt per plan version and check ordinal. A correction
marks a consumed result superseded and inconclusive; no re-read or
replacement look. Missing, unplanned or denied reads cannot pass.

A whole-section holdout keeps the signed safe versions and named
feature set for its frozen descendant footprint. Results cover
that bundle only. For 0029 link holding only, moderators can release
eligible individual links. Log the exposure change without moving
any experimental unit or changing other section holds. An owner-approved
flag changes only the named admitted feature, site and section.
Parent and child plans covering the same readers reserve the
intersection. On WordPress, assign only existing, admitted templates
to stable page identities. Guest, crawler and cache views for the
same page use the same assigned version. Use the signed add-on
exposure table locally. No request to the PC on a public view.
Every cache that can answer a guest or crawler, including CDN,
must prove it checks the signed exposure fence before serving
a treatment. Otherwise disable treatment caching on that path.
No treatment with unproved path control. No stale-if-error for
treatment. Expired state serves the safe original. Admit the exact
0021 public footprint and cache paths before enabling.

Represent conflicts as intersecting stable page or section
footprints, mutable resource IDs and uncertainty about dependency
coverage during overlapping half-open UTC time intervals. Reserve
at activation with current 0020 approval, scope, versions and
lease deadline in one serial site-owned generation. Recheck at
dispatch and renewal. Retain mutable reservations until no live
effect remains or rollback is proved. Conflict or uncertain scope
waits. Native apply and undo use separate stable 0021 effect IDs.
When acknowledgement is lost, read back the exact native version
and receipt. Never repeat an uncertain effect with a new ID.

Guardrails bind source revision, cutoff, required final days,
maximum evidence age, threshold, comparison and stable priority.
A breach or missing input revokes queued work and renewal. Select
the first failing condition by priority then ID. Lease ends no
later than the signed plan deadline or next required guardrail
cutoff, and never exceeds five minutes. Native exposure revalidates
the lease on each served treatment path, fails closed without
fresh proof and stops locally at expiry even when revocation
cannot reach the site. Reconcile native receipt on reconnect and
restore only if the effect still owns the current native version.
Keep a later staff edit and report its conflict.

When the guardrail stop reply is lost, retain the original
unresolved native effect across a PC restart under 0019. Bind its
0021 request ID, site, operation, intent digest and native target
to the original plan revision, lease and reserved footprint. A
safe baseline served after expiry does not verify that native
effect. Refuse a receipt for another effect or lease even if its
signature is valid and the site, target and baseline match.
Keep the existing reservation and fence renewal and native writes
while the outcome is unresolved. SUT-2303 remains the stop reason.
Show “Change not yet verified” until exact reconciliation under
current read authority. Confirm only the matching effect and read
the current native version separately. Clear the original
reservation only when no live effect remains or rollback is
proved. Reconciliation issues no repeat write and grants no new
undo of a later edit.

For an admitted WordPress template plan, a lost native apply reply
leaves the original effect unconfirmed. Keep the assignment and
reservation while the effect is unresolved. Show “Change not yet
verified”. Reconcile that effect by exact receipt and revision under
current read authority. Hold another apply or undo on the unresolved
effect. While the PC is offline, each admitted guest, crawler and
cache path stops treatment at the lease deadline and serves the safe
baseline. An unproved cache path that cannot be safely excluded from
treatment caching makes the route unavailable before apply (SUT-2304).
Label the reason. A recovered receipt proves only the original native
outcome. A later edit keeps its bytes and refuses stale undo. The
0148 exposure and final-look rules govern scientific evidence for a
study using that route.

Generate ideas from authorised same-site findings as local
proposals. Include why, target scope, one measurable hypothesis,
evidence, assumptions and risks. A missing input produces a
research question. Ideas cannot start plans. The local findings
library lists all completed, withheld and stopped outcomes with
versions, data coverage and uncertainty. Hide revoked evidence
and preserve a safe reason for the withheld result. Site scopes
and current staff grants apply to library, reads and eligible
exports. Retain derivative evidence within the shortest source,
policy and site retention limit. Cancel jobs and invalidate output
and exports under 0032 and 0118 on withdrawal. Keep only permitted
non-identifying outcome and assignment counts. A prior result or
suggestion grants no authority to start a new experiment.

### Named report and experiment fixtures

Each fixture records a planned check under spec 0076. QR-067,
QR-132 and QR-133 hold the source, method and platform proof.

- `DES-0094-REPORT` uses `VAR-0094-JOINT-COHORT` for admitted
  0034 joint cohort aggregates and `VAR-0094-TEMPLATE-REVISION`
  for immutable masters. `VAR-0094-SOURCE-WATERMARK`,
  `VAR-0094-REVISION-REVIEW` and `VAR-0094-METRIC-DEFINITION`
  use the existing freshness, staff comment and glossary paths.
- `DES-0094-STATISTICS` uses `VAR-0094-SAMPLE-CALCULATOR` and
  `VAR-0094-SIGNED-LOOKS` for 0066 sizing and declared reads.
  The calculator allocates no units. Depleted or withdrawn
  evidence leaves the check without a pass.
- `DES-0094-EXPERIMENT` uses `VAR-0094-PLAN-REVISION`,
  `VAR-0094-SECTION-HOLDOUT`, `VAR-0094-SECTION-FLAG`,
  `VAR-0094-WP-TEMPLATE`, `VAR-0094-GUARDRAIL-STOP` and
  `VAR-0094-SCOPE-RESERVATION` within the existing signed plan
  and native contracts. The fixed WordPress template profile is
  shared with F-0021. Missing host, cache or action admission
  leaves the safe baseline. Fixtures use synthetic approvals and
  effects. Live exposure still requires current exact authority,
  native receipts and the measured platform footprint.
- `DES-0094-LEARNINGS` uses `VAR-0094-LOCAL-IDEA` and
  `VAR-0094-LOCAL-OUTCOME` for local proposals and authorised
  results. Reusing an outcome still requires a fresh approved
  plan before exposure. Holdout results stay out of fitting and
  ranking qualification.

| Feature | Requirement | Design and variant | Fixture and test | Named fixture assertion |
|---|---|---|---|---|
| F-0315 | REQ-0094-PRIVACY | DES-0094-REPORT, VAR-0094-JOINT-COHORT | FX-0094-W2-COHORT, UT-0094-PRIVACY | Use attributed-arrival native days and adjacent windows. A sparse group revealed by differencing holds its complements across summaries and files. A missing eligible joint projection shows “Not measured”. |
| F-0316 | REQ-0094-SNAPSHOT | DES-0094-REPORT, VAR-0094-TEMPLATE-REVISION | FX-0094-W2-TEMPLATE, IT-0094-SNAPSHOT | Clone a same-site template, seal its master and edit the clone. The clone has a new ID and the master retains its original revision and source scope. A second-site read fails. |
| F-0317 | REQ-0094-METRICS | DES-0094-REPORT, VAR-0094-SOURCE-WATERMARK | FX-0094-W2-FRESHNESS, UT-0094-METRICS | Freeze observed, partial, stale and missing rows with distinct watermarks. Revoking one source holds its projection and pending export. Missing remains unknown. |
| F-0318 | REQ-0094-SNAPSHOT | DES-0094-REPORT, VAR-0094-REVISION-REVIEW | FX-0094-W2-COMMENT, IT-0094-SNAPSHOT | Pin a comment to revision one and seal revision two. Client and revoked reviewer reads fail. The export and mail fixtures contain no comment text. |
| F-0319 | REQ-0094-METRICS | DES-0094-REPORT, VAR-0094-METRIC-DEFINITION | FX-0094-W2-GLOSSARY, UT-0094-METRICS | Check clicks, impressions, position and traffic value against their pinned versions and units. A never-known ID is unavailable. A permitted historical definition retains its original unit. |
| F-0320 | REQ-0094-PLAN | DES-0094-EXPERIMENT, VAR-0094-PLAN-REVISION | FX-0094-W2-PLAN, IT-0094-PLAN | Amend an assigned plan. The prior assignment and outcome keep their revision. Modified, revoked or expired synthetic confirmation creates no exposure. |
| F-0321 | REQ-0094-STATS | DES-0094-STATISTICS, VAR-0094-SAMPLE-CALCULATOR | FX-0094-W2-SAMPLE, UT-0094-STATS | Compare rounded-up units and days with a bounded synthetic rate. For targets of 12 units per arm, bounds of 12 and 12 at the cap meet both targets. Bounds of 10 and 14 give an infeasible estimate under the frozen inputs. A proved zero bound for one of two arms with otherwise eligible traffic is infeasible. If one required arm bound is missing, withhold the estimate even when the known arm falls short. Missing cluster variation or zero overall traffic also leaves it unavailable. Every branch creates zero assignments. |
| F-0322 | REQ-0094-STATS | DES-0094-STATISTICS, VAR-0094-SIGNED-LOOKS | FX-0094-W2-INTERIM, UT-0094-STATS | Sign three checks. Miss the first and replay the second at the inclusive threshold. The threshold tie is eligible only with the other signed checks met. Keep one receipt for the replay and no pass after coverage withdrawal. |
| F-0323 | REQ-0094-ASSIGN | DES-0094-EXPERIMENT, VAR-0094-SECTION-HOLDOUT | FX-0094-W2-HOLDOUT, BDD-0094-ASSIGN | Move a page after assignment and simulate a moderator releasing one eligible 0029 link. Analysis keeps the original group and the remaining signed holdout. No holdout row reaches fitting or ranking qualification. |
| F-0324 | REQ-0094-PLAN, REQ-0094-ASSIGN, REQ-0094-GUARD | DES-0094-EXPERIMENT, VAR-0094-SECTION-FLAG | FX-0094-W2-SECTION-FLAG, IT-0094-PLAN, IT-0094-GUARD | Replay one exact admitted section effect through a simulated add-on. The sibling and denied setting have zero effects. A later staff edit survives conditional undo. |
| F-0325 | REQ-0094-ASSIGN, REQ-0094-GUARD | DES-0094-EXPERIMENT, VAR-0094-WP-TEMPLATE | FX-0094-W2-WP-CACHE, BDD-0094-ASSIGN, IT-0094-GUARD | Serve cached guest and crawler views of one fixed page. With the PC offline, an expired or uncertain lease gives the safe baseline. XenForo and a delayed lease leave it unchanged. |
| F-0326 | REQ-0094-LEARNING | DES-0094-LEARNINGS, VAR-0094-LOCAL-IDEA | FX-0094-W2-IDEA, BDD-0094-LEARNING | Give an allowed same-site finding, a missing metric and a second-site result. Show one linked proposal or a research question. Deny wrong-site evidence and direct activation. |
| F-0327 | REQ-0094-LEARNING | DES-0094-LEARNINGS, VAR-0094-LOCAL-OUTCOME | FX-0094-W2-LIBRARY, BDD-0094-LEARNING | Record success, null, cancellation and muddied outcomes. Withdraw one source and keep only permitted outcome context and a safe reason. Replaying a historical outcome starts no test. |
| F-0328 | REQ-0094-GUARD | DES-0094-EXPERIMENT, VAR-0094-GUARDRAIL-STOP | FX-0094-W2-GUARDRAIL, IT-0094-GUARD | Drop a required input before renewal and simulate a lost stop reply. Local expiry stops treatment and fences stale replies. Readback preserves a later staff edit and reports a safe conflict. Use the separate IT-0094-GUARD stop-reply case for the PC restart, wrong-effect and wrong-lease refusals and exact recovery of the guarded footprint. |
| F-0329 | REQ-0094-GUARD | DES-0094-EXPERIMENT, VAR-0094-SCOPE-RESERVATION | FX-0094-W2-CONFLICT, IT-0094-GUARD | Race reservations sharing a mutable dependency. At most one can win. Adjacent half-open intervals can proceed once disjointness is proved. Uncertain rollback keeps its reservation. |

### Wave 2 registration, value and anomaly checks

Use DES-0094-REPORT and the existing source, exact unit,
consent and suppressed-result contracts. Each case needs its
own admitted data profile before a build task.

| Feature and scope | Exact planned case |
|---|---|
| F-0311 · REQ-0094-PRIVACY · VAR-0094-W2-REGISTRATION-DAY | FX-0094-W2-REGISTRATION-DAY extends UT-0094-PRIVACY. Use two eligible arrivals for one completed registration, an arrival on the exact elapsed-time boundary and one outside. Select the last eligible arrival with stable receipt ties. Count once on the registration native day. Late correction or consent withdrawal replaces or withholds the group. Marginals and suppressed complements give no result. |
| F-0312 · REQ-0094-PRIVACY · VAR-0094-W2-SIGNED-VISIT-RATE | FX-0094-W2-TRAFFIC-VALUE extends UT-0094-PRIVACY. Three eligible visits at a synthetic GBP 0.015 rate give an estimated GBP 0.04 after half-even rounding. Preserve an observed zero. Refuse clicks as visits, mixed currencies and unproved coverage across changed rates. Use only exact covered rate intervals. |
| F-0313 · REQ-0094-METRICS · VAR-0094-W2-SOURCE-ANOMALY | FX-0094-W2-SOURCE-ANOMALY extends UT-0094-METRICS. Compare a valid anomaly with changed population, partial dates, stale inputs and missing baseline. Seal a qualified observed value of 50 against a band of 20 to 40. Correct the same provider day and filters to 35 in a later generation. An eligible open of the saved card uses the original 50 with its date and scope. Remove or revoke the original chart generation and show the chart unavailable, with no substitution by 35 and no change to an otherwise eligible saved finding. No pull or alert. |

## Data, settings and limits

Keep site-scoped report revisions, schedules, comments, experiment
plans, assignments and decisions under spec 0108. Follow spec 0032
for audit, erasure and restore. Withdraw affected masters, managed
files and queued renders. Restore and replay must repeat that check
before exporting or sending. Regenerate from eligible evidence only.
Shared copies are outside recall. Local master retention uses the
table. Managed files use spec 0100's `export.keep_days`.
Reporting adds zero public-page work. Before activation, QR-067
needs approved per-platform query, time and byte budgets and
measured cache parity. Proposed settings follow.

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `report.schedule_limit` | schedules per site | 10 | 1 to 50 | next save |
| `report.keep_days` | days from local seal | 90 | 1 to 365 | next retention pass |
| `experiment.active_limit` | active tests per site | 3 | 1 to 10 | next activation |

## Failure codes

DR-087 reserves SUT-2300 to SUT-2399. SUT-2300 means insufficient
evidence. SUT-2301 means an unsafe export. SUT-2302 means a plan
conflict. SUT-2303 means a guardrail stopped exposure. SUT-2304
means the platform route is unavailable. SUT-2305 means a limit
was reached.

## Security and privacy

Check site and actor under specs 0020 and 0101. Keep private evidence
and sensitive fields out of shared files. Keep report evidence out
of link ranking, tuning and qualification under specs 0057 and 0077.
The analytics freeze rule (RS20 in spec 0027) uses its own spec 0034
job. Report and experiment findings do not supply that flag.

## Requirements

| ID | Requirement | Test design and expected result |
|---|---|---|
| REQ-0094-SNAPSHOT | Reports MUST seal a revision before display or export, and scope comments and branding. | IT-0094-SNAPSHOT: change the template and add a comment; the old revision and exported figures stay fixed. |
| REQ-0094-METRICS | Reports MUST show source, scope, definitions, freshness and admitted anomalies. | UT-0094-METRICS: exercise current, stale, partial and missing sources, unlike definitions and an ineligible metric. |
| REQ-0094-EXPORT | Exports MUST use one approved projection and MUST NOT call cloud destinations. | E2E-0094-EXPORT: verify one master across formats. Test compatible files, formula cells, external assets and no outgoing request per format. Use FX-0094-SLIDES-MANUAL-IMPORT for the separate DR-087 Slides candidate and its qualification hold. |
| REQ-0094-DELIVERY | A schedule MUST seal once per slot and send only current, approved email reports. | IT-0094-DELIVERY: replay a slot, cross both clock changes, revoke a recipient and separate saved files, refused attachments and uncertain sends. |
| REQ-0094-PROOF | The report MUST show a matched estimate and interval with observational limits. | UT-0094-PROOF: check matched, changed and absent controls and noncomparable pre-trends. |
| REQ-0094-FORECAST | The report MUST show supported ranges and withhold unsupported scenarios. | UT-0094-FORECAST: compare backed and unsupported effects and check intervals on held-back history. |
| REQ-0094-PLANNING | The list MUST order supported actions and MUST NOT apply a queued change. | BDD-0094-PLANNING: given two supported actions, show two with reasons and submit one to review without applying. |
| REQ-0094-EDITOR | The map MUST match demand locally from an admitted, authorised view. | IT-0094-EDITOR: type with current, stale and absent maps; observe zero new requests. |
| REQ-0094-PRIVACY | Attribution and cohorts MUST use approved aggregates. Unit values MUST be labelled. | UT-0094-PRIVACY: suppress a sparse group and revealing subtotal, refuse incompatible currencies and mark missing registration evidence. |
| REQ-0094-PLAN | Activation MUST require an immutable approved plan with named controls. | IT-0094-PLAN: reject missing or revoked approval and a changed signed plan. Apply one duplicate request only once. |
| REQ-0094-STATS | Tests MUST use predeclared sample, interim and stop rules. | UT-0094-STATS: test missing baseline, sample limits, unplanned interim reads and inconclusive expiry. |
| REQ-0094-ASSIGN | All readers MUST receive the same assigned version for each eligible page. | BDD-0094-ASSIGN: given the cycle forum, check reader, crawler and cache parity, baseline in unaffected sections, and refusal of a wrong platform or template. |
| REQ-0094-GUARD | Tests MUST refuse conflicts and stop exposure safely. | IT-0094-GUARD: try concurrent overlap, partial apply, guardrail loss, offline expiry, restart, late replies and repeated undo. Preserve baseline and refuse overwrite on conflict. In the admitted WordPress template fixture, lose the apply reply with warm treated cache entries. Disconnect the PC and verify baseline at expiry on each admitted serving path. Reconnect and reconcile only the original effect after an independent staff edit. Hold stale undo and preserve the later bytes. A cache path that cannot enforce the lease and cannot safely be excluded refuses activation before any apply. In the separate FX-0094-W2-GUARDRAIL case, omit a required daily input and let the native guardrail stop commit. Lose its reply and restart the PC. At expiry verify baseline on each admitted serving path. Preserve the exact unresolved stop identity and reserved footprint. In separate attempts offer authenticated receipts for a different effect or an earlier lease with the same site, target and visible baseline. Neither closes the original outcome, releases the reservation, renews exposure or issues another write. Report SUT-2303 with “Change not yet verified”. A current scoped read of the exact original receipt and native version can close only that stop and can release its footprint only with proof that no live effect remains or rollback is proved. Replay returns the same receipt. If an independent later staff edit changed the native version, preserve its bytes and report conflict. Use synthetic effects only. |
| REQ-0094-LEARNING | Ideas MUST remain proposals, and the library MUST record all outcomes. | BDD-0094-LEARNING: given an inconclusive test, retain its reason and let the owner review a new idea without starting it. |
| REQ-0094-LIMITS | Actions MUST enforce site grants, settings and public footprint budgets. | E2E-0094-LIMITS: try wrong site, each limit and its neighbours, erasure during queued export, expired replay and measured platform request budgets. |

## How it is tested

Designed, not run. The table names the test designs. Spec 0076 and
QR-067 require feature-specific method, platform, privacy, limits,
scheduling and import fixtures before implementation admission.

### Named feature bindings

These are planned feature fixtures under spec 0076. QR-067 holds
method, renderer and editor-map admission. QR-132 holds the report
profiles, QR-133 the experiment proof and QR-039 the PDF renderer.

- `DES-0094-PROOF` uses `VAR-0094-LOCAL-PROOF` for same-site
  matched observations and their limits. `DES-0094-FORECAST` uses
  `VAR-0094-90-DAY-CLICKS` for the existing organic-click baseline
  and the conditional admitted queued scenario.
- `DES-0094-EXPERIMENT` uses `VAR-0094-WP-TEMPLATE` for the
  existing approved WordPress template profile. The fixture uses
  synthetic scope and approval records. Real exposure still needs
  current plan, owner and moderator approvals and host proof.
- `DES-0094-ADVICE` uses `VAR-0094-WEEKLY-FIX-LIST` for review
  queues and `VAR-0094-EDITOR-MAP` for local typing matches from
  the admitted aggregate demand view.
- `DES-0094-REPORT` uses the `VAR-0094-SCHEDULED-MASTER`,
  `VAR-0094-BRANDED-FILE` and `VAR-0094-EXECUTIVE-SUMMARY`
  variants. A master stays frozen through template edits. Scope,
  withdrawal and erasure still govern each display, file and send.
  The existing optional 0100/0111 email approval governs delivery.
- `DES-0094-FILES` uses `VAR-0094-SHEETS-CSV` and
  `VAR-0094-WAREHOUSE-BUNDLE`. The separate DR-087 candidate
  `VAR-0094-SLIDES-PPTX` uses `FX-0094-SLIDES-MANUAL-IMPORT`.
  Under DR-087 the owner uploads
  compatible files manually. Each format uses the current safe
  projection and admitted 0100 transport profile. The fixtures
  assert zero requests to report destinations.

Spec 0138 uses its own DR-124 authority for eligible settled
own-site adverse evidence. Its advisory outputs supply no forecast
inputs, experiment assignments or before-and-after proof here.

| Feature | Requirement | Design and variant | Fixture and test | Named fixture assertion |
|---|---|---|---|---|
| F-0020 | REQ-0094-PROOF | DES-0094-PROOF, VAR-0094-LOCAL-PROOF | FX-0094-W1-PROOF, UT-0094-PROOF | Use equal predeclared windows, one unchanged same-site control and two rejected controls. Only the comparable match yields an observational estimate and interval. |
| F-0021 | REQ-0094-ASSIGN and REQ-0094-GUARD | DES-0094-EXPERIMENT, VAR-0094-WP-TEMPLATE | FX-0094-W1-ASSIGNMENT, BDD-0094-ASSIGN and IT-0094-GUARD | With synthetic approved templates, compare visitor, crawler and cache outputs. One stable assignment is visible. An expired plan restores baseline, and undo keeps an intervening edit. In the joined WordPress branch, lose the committed reply, go offline with a warm treatment cache and require baseline at lease expiry. Keep the exact reservation and show the unconfirmed result until verified. Recover the original receipt without applying or undoing again, preserving any later staff edit. Use 0148 IT-0148-LOCK for the assignment, guarded measurement and frozen dates. |
| F-0022 | REQ-0094-PLANNING | DES-0094-ADVICE, VAR-0094-WEEKLY-FIX-LIST | FX-0094-W1-WEEKLY-THREE, BDD-0094-PLANNING | Give two comparable actions and one unsupported action. The weekly list shows two reasons, and submitting one action produces a review entry with no site edit. |
| F-0023 | REQ-0094-FORECAST | DES-0094-FORECAST, VAR-0094-90-DAY-CLICKS | FX-0094-W1-FORECAST, UT-0094-FORECAST | Use a held-back baseline fixture and compare a supported queued effect with a missing effect. The 90-day baseline keeps its range, and only the supported scenario can show a range. |
| F-0024 | REQ-0094-EDITOR | DES-0094-ADVICE, VAR-0094-EDITOR-MAP | FX-0094-W1-EDITOR-DEMAND, IT-0094-EDITOR | Type with a current map, then revoke the grant and try stale and absent maps. Only the first yields a match. Typing in all four conditions produces zero requests. |
| F-0305 | REQ-0094-DELIVERY | DES-0094-REPORT, VAR-0094-SCHEDULED-MASTER | FX-0094-W2-SCHEDULE, IT-0094-DELIVERY | Replay a planned slot over restart and a repeated hour. One master survives. A revoked recipient gets no mail, and an uncertain outcome never causes another send. |
| F-0306 | REQ-0094-SNAPSHOT | DES-0094-REPORT, VAR-0094-BRANDED-FILE | FX-0094-W2-BRANDING, IT-0094-SNAPSHOT | Give an approved logo, an erased logo, an active payload and a different-site logo. The allowed render and plain fallback keep the master hash and have distinct file receipts. |
| F-0307 | REQ-0094-SNAPSHOT and REQ-0094-PLANNING | DES-0094-REPORT, VAR-0094-EXECUTIVE-SUMMARY | FX-0094-W2-SUMMARY, IT-0094-SNAPSHOT and BDD-0094-PLANNING | Change the source chart after sealing. The summary retains coverage, time and units, and keeps missing or unlike measures out of its up to three review actions. |
| F-0309 | REQ-0094-EXPORT | DES-0094-FILES, VAR-0094-SHEETS-CSV | FX-0094-W2-SHEETS-CSV, E2E-0094-EXPORT | Export text IDs, native dates, non-ASCII text, nulls and a formula-shaped cell. The protected local file retains the admitted projection. Revocation before publication leaves no output. |
| F-0310 | REQ-0094-EXPORT | DES-0094-FILES, VAR-0094-WAREHOUSE-BUNDLE | FX-0094-W2-WAREHOUSE-PAIR, E2E-0094-EXPORT | Compare typed CSV and admitted Parquet on decimals, native dates, null, empty text and zero. Each export binds its schema and publishes or withdraws as one bundle. |

## Changes from the source

DR-087 changes direct Sheets and warehouse exports to local files
and adds a Google Slides-compatible local format. PDF waits for
QR-039. Scheduled reports use the existing approved email route. Experiments
use scoped approval, fixed assignments and measured platform limits.
