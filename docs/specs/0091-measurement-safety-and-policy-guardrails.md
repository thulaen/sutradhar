# 0091: Measurement, safety and policy guardrails

See what happened to reviewed content and stop unsafe proposals.
Spec 0089 owns exact-draft gates and live comparison. Spec 0090
owns local hand-off. Spec 0094 owns shared reports and experiments.
Spec 0020 owns MCP permissions, untrusted text markers and assistant
revocation. Spec 0042 owns its AI drafting controls and labels.
This toolkit produces cited proposals and never publishes.

Fictional example. A moderator sees that a guide's first month has
too few measured visits to compare. The app leaves the value
unknown. Staff triage a complaint and the owner holds local hand-off
for the cited revision while staff review it.

## Words used here

- **Cohort** means a group with shared eligibility and measurement.
- **Recipe** means versioned steps for preparing a draft form.
- **Guardrail** means a policy or permission that can hold an action.
- **Confirmed publication** means a matched observation under spec 0089.
- **Policy revision** means a checked version of the applicable rules.

## What this spec designs

| Feature | What staff reviews | Requirement |
|---|---|---|
| 1263 Piece performance page | Observed approved-piece metrics with scope | REQ-0091-MEASURE |
| 1264 Assistant versus human content | Comparable authoring cohorts without causal inference | REQ-0091-MEASURE |
| 1265 Best recipes | Recipe evidence with eligibility and reasons | REQ-0091-RECIPES |
| 1266 Human edit amount | Versioned human edit amount without raw text in reports | REQ-0091-MEASURE |
| 1267 Rejection reasons | Approved rejection reasons by defined cohort | REQ-0091-MEASURE |
| 1268 Idea to publication time | Time between dated idea and confirmed publication | REQ-0091-MEASURE |
| 1269 Reader feedback | Protected themes from eligible public feedback | REQ-0091-FEEDBACK |
| 1270 Winning patterns | Proposed recipe revision requiring review | REQ-0091-RECIPES |
| 1271 Series performance | Comparable series evidence and coverage | REQ-0091-MEASURE |
| 1272 AI cost per piece | Evidence-backed cost with unknowns | REQ-0091-MEASURE |
| 1273 Member reactions | Protected aggregate reactions | REQ-0091-FEEDBACK |
| 1274 New questions after publishing | Sourced questions after confirmed publication | REQ-0091-FEEDBACK |
| 1275 Value estimate | Labelled value estimate using approved unit basis | REQ-0091-MEASURE |
| 1276 Visual engagement | Admitted aggregate visual interaction evidence | REQ-0091-MEASURE |
| 1277 Template copy performance | Protected supplied metrics by template and window | REQ-0091-MEASURE |
| 1278 Failing quality gates | Counts of exact gate outcomes and unavailable checks | REQ-0091-MEASURE |
| 1279 Writer and assistant scorecards | Role-scoped, context-labelled scorecards | REQ-0091-MEASURE |
| 1280 First 30 days search performance | First 30 days of comparable organic search data | REQ-0091-MEASURE |
| 1281 Portfolio balance | Type, topic and freshness coverage | REQ-0091-MEASURE |
| 1282 Pieces to retire | Retirement proposal with evidence and unknowns | REQ-0091-RECIPES |
| 1283 No member impersonation | No impersonation; approved staff or system attribution | REQ-0091-IDENTITY |
| 1284 Historical reference. See 1283, No member impersonation. | Historical alias F-1284 to F-1283 | REQ-0091-SCOPE |
| 1285 Historical reference. See 1165, AI-assistance labels. | Historical alias F-1285 to F-1165 | REQ-0091-SCOPE |
| 1286 Prompt-injection defence | Untrusted source text cannot authorise actions | REQ-0091-TRUST |
| 1288 Sensitive topic blocklist | Owner-approved sensitive topic rule | REQ-0091-POLICY |
| 1289 Hate and harassment block | Hold hateful or harassing draft content | REQ-0091-POLICY |
| 1290 Safe messaging on self-harm | Admitted safe-messaging review for self-harm topics | REQ-0091-POLICY |
| 1291 Political content rules | Current owner rules for political and election content | REQ-0091-POLICY |
| 1292 Daily publishing limits | Local quota for ready hand-offs, with no publish action | REQ-0091-STOP |
| 1293 AI content kill switch | Stop generation through the shared kill switch | REQ-0091-STOP |
| 1294 Legal AI labels | Applicable AI disclosure in the draft | REQ-0091-IDENTITY |
| 1295 Children's data protection | Block targeting or collection of children's data | REQ-0091-POLICY |
| 1296 Trademark guidance | Trademark guidance with approved scope and sources | REQ-0091-POLICY |
| 1297 Disclaimer library | Versioned disclaimers with qualified review | REQ-0091-POLICY |
| 1298 AI content takedown | Complaint intake and staff recovery proposal | REQ-0091-STOP |
| 1299 Nightly red-team tests | Nightly isolated synthetic policy checks | REQ-0091-TRUST |
| 1300 Secret and data filtering | Filter secrets and identifying data before exposure | REQ-0091-TRUST |
| 1301 Approved models | Exact permitted model, route and content type | REQ-0091-MODELS |
| 1356 Target queries per draft | Local target queries with evidence and caps | REQ-0091-DRAFT |
| 1357 Draft auto-save | Scoped save of draft edits without approval | REQ-0091-DRAFT |

## Measurement and improvements

1. Separate pre-publication edits, rejection and gate counts
   from published-piece metrics. Include held and rejected drafts
   in their eligible denominators. Attribute page results to an
   exact revision only with confirmed mapping and an effective
   interval free of version overlap or switches. Show coverage
   and definition. Staff claims and later crawls cannot set or
   restart the first 30 days. Without a pinned publication time,
   show unknown or partial.
2. Use eligible protected aggregates in specs 0034 and 0040
   for visits, search, reactions and visuals. Use admitted
   supplied totals for email and social templates. Require like
   definitions, periods and populations. Prevent deduction across
   combined views and exports. Add no public tracking and never
   infer a person. Missing evidence stays unknown.
3. Compare human and assistant work in eligible labelled groups.
   Show sample limits and confounds. Spec 0094 owns experiments.
   Use exact diffs and recorded rejection reasons. Scorecards
   follow spec 0101 writer access and leaderboard authority.
   Keep raw individual history out of shared reports. Protect
   scope and counts in every projection. Scores decide no roles,
   pay or link ranking.
4. Keep admitted measured cost and estimates separate, with date,
   units and scope. A value estimate names the owner-set unit
   value and eligible population. Do not use it as recorded income.
   Measure idea-to-publication time only with both dated events.
   Separate series and whole-portfolio definitions.
5. Review protected feedback from eligible guest-visible replies.
   Show sourced questions with no private identifiers. Recipe
   updates, improved patterns and retirement entries are new
   proposals. Pin basis, version and expected effect. A person
   approves only the new proposal recipe version. It cannot change
   protected prompts, admission, roles, safety switches or training.
6. Require one main and at most four extra scoped target queries.
   Use admitted spec 0092 evidence. Draft checks use those saved
   queries. Autosave reuses the encrypted 0101 writer space,
   0042 draft store, retention and `team.autosave_s`. Require an
   expected revision, safe retry, conflict view and erasure check
   on replay or restore. Autosave never inherits approval.

### Piece results, cohorts, recipes and edits (1263 to 1266)

Use 0034 and 0040 protected source projections and 0094's report
contract. Pin the site, exact source and revision scope, measure
definition, observation window and coverage, eligible population and
disclosure generation. Apply the existing disclosure history across
previews, charts, comparisons, files and prior releases. Show missing,
incomplete and suppressed evidence separately. A correction, loss of
scope or erasure invalidates affected views before replay or restore.

| Feature | Exact measurement or proposal contract |
|---|---|
| 1263 Piece performance page | Bind the exact approved piece and canonical destination to a matched 0089 observation and proved effective revision interval. Pin source property, native time basis and canonical version. Never distribute a mixed-revision bucket by assumption. Attribute results only for observed windows with a unique eligible active revision. Show definition, source, scope and as-of date. Split only where eligible evidence supports the split. Overlap or absence gives partial or unknown evidence, never an inferred launch, zero or winner. Loss of rights to the matched observation invalidates the dependent piece attribution before current display or export, including a restored cached report. Treat it as unknown if no other eligible proof covers that interval, even when the analytics totals remain eligible for their own scope. |
| 1264 Assistant versus human content | Assign a cohort only from the admitted evidence of actual authorship and revision history. Keep mixed and unknown provenance separate from unambiguous groups. Compare the same admitted measure and like audience, form, topic scope, window and coverage where applicable. Label remaining confounds and uncertainty. Small or reconstructible comparison cells stay held. No causal attribution, personal score disclosure or individual identification. |
| 1265 Best recipes | Bind prepared content to the exact recipe version and eligible measurement cohort. State the qualified criteria, windows, denominators and uncertainty. Keep unknown, incomparable or tied outcomes visible instead of choosing a winner. A recommendation pins the supporting evidence and can propose a new reviewed recipe version only. It cannot alter protected prompts, model admission, roles, training, active recipes or ranking. |
| 1266 Human edit amount | Compare an exact eligible assistant draft and its descendant human edit revision using the admitted local diff profile. Pin the visible field scope and counting rule and distinguish changes by attributable actor from unknown or mixed edits. Keep defined normalisation and units visible. The report uses permitted counts or aggregates only, with no raw text or private author history. A missing predecessor, incomparable revision or inseparable intervening AI or unknown edits yield unknown. Edit amount can inform editorial review only. |

QR-193 refines QR-080 for these profiles. Apply the existing role and
privacy checks across all projections, even when an exact internal
measurement exists. No additional tracking, provider request,
experiment decision, personal scoring effect or ranking input follows.

### Content outcomes and improvement proposals (1267–1282)

Use one site and the admitted evidence for the exact eligibility,
measure, definition, cohort, source revision and time window.
Distinguish pre-publication decisions from observed public results.
Bind a public measure to the confirmed piece revision and its
proved exposure interval. Hold attribution if an aggregate spans
multiple revisions or destinations without admissible separation.
Missing and suppressed values remain distinct from measured zero.
Preserve definitions, denominator, coverage, exclusions and
uncertainty. Publish no personal history, private text, identifiable
small groups or hidden cohort sizes in shared views or exports.
Bind the same producer release IDs, recipient, epoch and current
source rights to all fields, filters, counts, prose and files.
Apply common disclosure history to combinations and tiny differences.

Read only existing eligible site, connector and supplied-file
evidence. Recheck grants, consent, provenance and source and
erasure generations at read, review, queued work, export and
restore. A withdrawn input invalidates affected managed results
under specs 0032 and 0118. Scope records under spec 0108 and use
the existing 0078 and 0100 limits. Add no public tracking, live
provider route, site write or ranking signal. Spec 0094 owns
experiment design and results. QR-194 qualifies these forms.

- **1267, rejection reasons.** Record the permitted category from
  the current approved rejection decision, exact draft revision
  and cohort. Count each logical decision once. A held or withdrawn
  draft has its own state. Show the eligible decision denominator
  and protect rare reasons. Do not infer a reason from private notes.
- **1268, idea to publication time.** Use the dated idea linked to
  the exact published piece and the independently proved original
  publication time from admitted native proof under 0021 or 0044.
  A spec 0089 live match proves content separately. Preserve time
  precision and source. A first crawl, draft date or later revision
  cannot stand in for publication. Hold missing or inverted pairs.
- **1269, reader feedback.** Use eligible guest-visible replies
  and comments from the current corpus. Deduplicate exact native
  IDs, preserve coverage and release only protected themes with
  permitted evidence. Strip identifying details. Consent for a
  quote is a separate exact-use decision under specs 0084 and 0098.
- **1270, winning patterns.** Create a cited local recipe proposal
  from comparable versioned evidence with limits and uncertainty.
  Keep the existing recipe until a named person approves the exact
  new version. A proposal cannot change protected prompts,
  safety rules, model admission, roles or training. Observed
  differences give no causal claim or automatic activation.
- **1271, series performance.** Pin series membership and period
  under the admitted definition and exact confirmed piece revisions.
  Disclose which pieces have evidence. Deduplicate members and
  keep additive and non-additive measures separate. Do not sum
  overlapping populations or join incomparable windows.
- **1272, AI cost per piece.** Attribute only admitted run or
  receipt evidence tied to the exact draft lineage, owner-approved
  route, currency, units, date and measured or estimated basis.
  Keep bill evidence owner-private. Count a billed event once and
  keep missing charges unknown.
  Show local resource estimates separately from measured charges.
  No new price lookup, spend, currency conversion or credential use.
- **1273, member reactions.** Use comparable protected site aggregates
  with current reaction definition, exact staff-content revision and
  population. Suppress small or identifying groups. Keep missing
  instrumentation distinct from no observed reactions. No member
  list or identity inference comes from the measure.
- **1274, new questions after publishing.** Accept eligible current
  guest-visible question evidence after a proved publication time.
  Bind source, date and relationship to the piece. Compare the
  eligible earlier corpus before calling a question new. Label
  unknown coverage and use safe paraphrase where rights require it.
  Order in time does not establish that the piece caused the question.
- **1275, value estimate.** Apply the approved owner-set unit value,
  currency and effective revision to the eligible measured quantity.
  Use spec 0094's signed per-unit value basis and fixed rounding.
  Show assumptions, date and coverage. Hold unknown or
  incompatible units, currencies or values. Label output estimated
  value. Keep it separate from recorded income.
- **1276, visual engagement.** Use the admitted aggregate interaction
  definition and exact visual, rendition, placement and exposure
  scope. Distinguish impression, interaction and missing evidence.
  Hold a metric that belongs to a page total or another visual.
  There is no new beacon, tracking code or viewer history.
- **1277, template copy performance.** Admit only authorised supplied
  email or social totals through spec 0100. Pin channel, provider
  definition, template and rendition, cohort, denominator and period.
  Deduplicate exact observations and replace corrected evidence.
  Keep unlike rates and source coverage separate. No provider read,
  dispatch or recipient list.
- **1278, failing quality gates.** Count current eligible outcomes
  from exact gate records under spec 0089, once per admitted decision
  key and exact gate and candidate revision. Count pass, fail,
  unavailable and prescribed inapplicability by policy. Distinguish
  held, unfinished, discarded, retried and superseded workflow states.
  A retry or renamed gate version
  cannot inflate failures or silently join unlike results.
- **1279, writer and assistant scorecards.** Apply current writer
  visibility. Teams and leaderboards require exact 0101 opt-in.
  Keep creator-only scores scoped to their creator. Use
  labelled comparable groups, measures and coverage. Give staff
  only the authorised actor's or cohort's projection. No raw
  personal history or derivation of a hidden personal score. A
  score cannot change a role, pay, work allocation or ranking.
- **1280, first 30 days search performance.** Pin the original proved
  publication time and define the first 30 days using the admitted
  0040 native calendar and watermark. Recheck property permission.
  Bind canonical property, query scope,
  metric version, coverage and exact piece revision. Keep partial
  boundary days or changed revisions out unless evidence supports
  precise attribution. An uncertain start or missing days yield
  unknown or partial. A recrawl or revision cannot restart the window.
- **1281, portfolio balance.** Use the admitted active-site catalogue
  and current type, topic and freshness definitions. Show unclassified
  items and missing dates. Distinguish counts from overlapping
  topic shares. Cross-site dashboard tiles follow spec 0102 and
  remain separately scoped. This feature pools no site records.
- **1282, pieces to retire.** Make an evidence-based local proposal
  for the current piece and revision, with age, duplication or
  update evidence, known dependencies, coverage and uncertainty.
  A low or absent metric does not settle retirement. The proposal
  cannot remove, redirect, rewrite, archive or stop indexing a page.

## Policy, authorship and stops

1. Apply the active policy on each input, proposal and hand-off.
   Output with missing, unknown or prohibited status stays held.
   Use specs 0014 and 0020's authenticated owner-only change
   route. Owner policy, a model and a reviewer cannot override
   protected rules. Keep the required spec 0089 gates.
2. Never impersonate a member. Staff and system attribution uses
   its approved label and actor. Applicable AI labels use dated
   primary rules and qualified approval under QR-080. Include
   labels in reviewed proposals only. No public setting or page
   changes follow. Historical references resolve to their one
   controlling feature.
3. Sensitive-topic, hateful, harassing, political and age rules
   use the owner's admitted policy and current grants. Hold
   content targeting or collecting children's data. Require
   admitted safe-messaging guidance and qualified review for
   self-harm topics. Trademark and disclaimer drafts cite
   current policy and qualified review. Disclaimer wording
   cannot excuse a failed gate.
4. Use spec 0042's single owner-only stop and resume. Bind its
   state revision to queue, start, prompt, result and any
   affected copy or restore. Fence queued work, discard late
   output and never resume cancelled jobs automatically. Human
   editing and safety review remain available. Use a local
   hand-off quota with one atomic charge per idempotent key.
   Check site day, retry and concurrent clipboard or file copy
   before any output. It cannot count or stop site publication.
5. Triage complaints with exact candidate, role and duplicate
   handling. A named authorised decision is required for any
   action. Only the owner pauses local hand-off through spec 0090.
   Staff decide recovery in site tools. There is no public effect.

### Authorship, policy and local stops (1283–1296)

Pin policy, site, actor, purpose, content type, exact input and
candidate, intended audience, route, source rights and evidence.
Check admission before reading protected input or dispatching work,
and at result, review, enqueue, replay, hand-off and restore.
A missing, prohibited, stale or uncertain check holds the affected
route or output. Report a safe reason without replaying unsafe
text or exposing a private subject. A reviewer cannot override a
protected rule. Apply current rights and erasure to held results,
queues, extracts and audit references under specs 0032 and 0118.
No rule here posts, moderates, changes an account or writes a site.

- **1283, no member impersonation.** Bind proposed actor and
  attribution to the eligible staff or system label and exact
  current AI provenance. Refuse a member identity or voice as a
  substitute actor, even if supplied in the prompt. A permitted
  attributed quotation is a separately approved exact-use case.
  Approval does not grant a native account action.
- **1284, historical reference.** Resolve F-1284 to F-1283 under
  the same documented trace and decision. No independent
  capability, setting, call, count or attribution rule.
- **1285, historical reference.** Resolve F-1285 to F-1165 and use
  the documented 0089 AI-assistance label gate. No independent
  capability, setting, call, count or parallel label gate. Feature
  1294 owns legal applicability evidence for that single label gate.
- **1286, prompt-injection defence.** Treat source content and
  metadata as data under 0020 feature 1287. Marking does not grant
  trust. Refuse missing provenance before any model or tool call.
  Bind allowlisted typed authority outside any prompt. A source,
  attachment or generated result cannot authorise a tool, role,
  data release, network call or write. Validate before tool work
  and at output. Hold a result that tries to cross the permitted
  scope. Use synthetic inert fixtures without secrets or network.
- **1288, sensitive topic blocklist.** Use admitted typed topic
  rules and the authenticated owner-only policy change path.
  Pin rule, effective scope and revision. Hold a matching or
  unresolved outcome before generation or release as applicable.
  Source text and reviewer input cannot edit or narrow a rule.
- **1289, hate and harassment block.** Apply the admitted current
  policy to the exact draft and context. Hold prohibited material
  and unresolved cases. An attributed quotation does not bypass
  the policy. Safe reasons identify the rule without echoing the
  blocked text or an affected identity.
- **1290, safe messaging on self-harm.** Hold this content without
  admitted current safe-messaging guidance and qualified review
  for the intended audience and exact rendition. Prohibit detailed
  harmful methods, encouragement and glamorisation under the
  applicable safety rules. Keep protected context within eligible
  review. No automated counselling, outreach or service referral
  action. Use synthetic non-graphic fixtures.
- **1291, political content rules.** Use the versioned owner rule
  and admitted current scope for political and election content.
  Hold unresolved applicability, stale time limits, unsupported
  claims or protected targeting. The policy check cannot create
  targeted delivery or alter the protected rules.
- **1292, daily publishing limits.** Charge the existing local
  hand-off quota for one authorised clipboard or file release
  under one durable action under 0019, 0090, 0032 and 0118.
  Bind approved payload, recipient, actor, current grants, stop,
  site day, limit and revision. Commit charge, release and receipt
  as one guarded outcome at actual exposure. Keep staged files
  hidden until then. Same key with changed input conflicts.
  Reconcile uncertain output before any retry. Never debit or
  release twice. Reject stale rollover and restored quota state.
  The limit governs local hand-off. It cannot enforce native limits.
- **1293, AI content kill switch.** Reuse the single owner-only
  stop and resume in spec 0042. Bind its durable revision at
  queue, start, prompt, result, release and restore. Fence queued
  and running work on stop and discard late results from the
  prior revision, including at copy and restore. Keep human tips and
  evidence in their permitted non-drafting scope and editing within
  their existing scope. Resume does not revive cancelled jobs.
  An unavailable current state holds AI drafting and affected
  release. There is no competing switch, automatic resume or
  action on live content.
- **1294, legal AI labels.** Pin admitted dated primary rule,
  jurisdiction, effective date, content and audience category,
  provenance and qualified review for the exact rendition under
  QR-054. Supply exact applicability to the 0089/1165 and 0101
  visible-label gate. Hold unknown or
  conflicting applicability and a required label that is stale or
  missing. The editor's optional label setting cannot clear an
  applicable requirement. Label copy stays manual. Add zero
  public-page queries, time or bytes and no native insertion.
- **1295, children's data protection.** Hold input, candidate or
  hand-off that targets children or seeks to collect their data.
  Distinguish protective discussion from targeting or collection.
  Use the declared policy and exact evidence. Do not
  infer a child's identity or build an age profile to run the check.
  Hold uncertain collection or targeting and show a safe reason
  without recording the identifying detail.
- **1296, trademark guidance.** Show scoped guidance with current
  admitted evidence and qualified review for the proposed use.
  Keep an approved attribution distinct from permission, affiliation
  or registration. Hold an unsupported authority claim or uncertain
  applicability. The proposal does not change assets, grant rights
  or decide a legal dispute.

Qualified political, self-harm and trademark rules apply to the exact
jurisdiction, audience, date and rendition. They cannot waive 0089.
QR-194 extends QR-080 with exact roles, boundaries, applicable
policies and refusal checks. The safe fixtures show no native
publication, contact or unintended disclosure.

### Exact safety records and model checks (1297–1301)

Use the existing policy and stop controls, with 0020 authorisation,
0032 erasure and 0118 protection. Bind each result to its site,
request, draft or route revision, actor, audience, current grants,
source and policy. Recheck before work, result and hand-off. Missing
or revoked evidence holds affected output. Record safe reasons and
receipts without draft text, secrets or reporter identity.

- **1297, disclaimer library.** Store versioned approved medical and
  financial wording with exact purpose, topic, audience, jurisdiction,
  language, effective interval, dated source and named qualified review.
  Choose only an applicable current entry in a local proposal. Recheck
  the exact final wording, placement and policy at review and hand-off.
  An expired, generic or unmatched entry cannot clear the 0089 gate.
  No automatic insertion, legal finding or advice from a template.
- **1298, AI content takedown.** Record a complaint against the exact
  known site item and version through its admitted private intake, or
  record supplied evidence in the authorised staff view. Keep opaque
  case and reporter references and only the fields needed for triage.
  A duplicate receipt refers to the same scoped case. Changed or
  conflicting evidence needs review. A named staff decision can make a
  local recovery proposal. Only the owner can use the existing 0090
  local hold. The site's own staff separately decide and carry out any
  native change. A complaint does not prove model involvement, delete
  content, send a message or open a new reporting route.
- **1299, nightly synthetic safety checks.** Use the admitted 0019
  schedule and bounded 0078 isolated test profile under QR-080. Run
  only approved synthetic policy fixtures with no live member data,
  real secrets, network, platform calls, product state writes or tool
  execution beyond the isolated check. Return minimal signed
  synthetic results to the host. Only the authorised host applies
  a route hold under the current policy. Record fixture, candidate and
  expected refusal or safe-result fingerprints without test payload
  in the report. Missing isolation, timeout or failed proof holds the
  affected route under the existing safety policy. A retry or later pass
  does not reopen it. Require the existing authorised human review.
- **1300, secret and data filtering.** Evaluate the exact authorised
  projection before any visible draft, model context, result, tool
  response, log, clipboard, file or cache can carry it. Check safe
  representations, parsed fields, metadata and combined identifying
  details. Apply 0032 and the current minimum disclosure policy.
  Uncertain, forbidden or unparsable fields hold the affected surface
  or use an admitted safe omission. Never echo the matched value in a
  reason. Repeat on hand-off and restore. Filtering does not grant
  consent, increase visibility or make an unapproved route eligible.
- **1301, approved models.** Bind the existing owner-managed
  allow-list to site, content type, exact model/package identity,
  version, approved route, effect and resource profile. Intersect the
  entry with 0024 admission, 0042 route authority and current 0020
  grants. Use the owner-only local settings approval of the exact
  digest and route. Assistant or contributor settings grants give no
  authority here. At queue start, immediately before prompt dispatch,
  at result and at release recheck model, actor and route approval.
  Removal or scope change holds pending work and prevents old output
  becoming eligible. A listed model gives no package installation,
  provider call, fallback, transfer or expansion of rights.

QR-195 and QR-080 qualify exact records, isolation, refusal and
recovery proof. Preserve the existing policy and release authorities.

### Safety intake and release fixture bindings

Use synthetic sites, opaque case references and inert failure
markers. These cases add no reporting or provider route. QR-195
and QR-080 hold exact protected review, isolation and model
admission. Reuse the 0032 disclosure and 0118 restore fences.

- `DES-0091-W3-EXACT-COMPLAINT-TRIAGE` binds one site item
  and revision to the admitted private intake and accountable
  staff review. The owner alone can use the existing 0090 hold.
- `DES-0091-W3-SEALED-SYNTHETIC-SAFETY` binds the 0019
  scheduled fire and attempt and the admitted 0078 test profile to
  minimal signed evidence. A result for another fire cannot satisfy
  current proof, even with the same fixture, candidate and expected
  outcome. Apply the existing missing-proof rule to the affected
  route. Only the host can apply a hold.
- `DES-0091-W3-MINIMUM-SAFE-PROJECTION` checks the authorised
  representation before its creation or release and again
  on restore. The existing policy decides hold or omission.
- `DES-0091-W3-EXACT-MODEL-ELIGIBILITY` intersects the
  owner-approved entry with 0024 admission and 0042 route
  authority at the existing 0020 permission boundary.

| Feature | Requirement | Design and variant | Named fixture and shared tests | Independent fixture check |
| --- | --- | --- | --- | --- |
| F-1298 | REQ-0091-STOP, REQ-0091-SCOPE | DES-0091-W3-EXACT-COMPLAINT-TRIAGE, VAR-0091-W3-PRIVATE-COMPLAINT-AND-STAFF-RECOVERY | FX-0091-W3-EXACT-PRIVATE-COMPLAINT, IT-0091-STOP, IT-0090-RECOVERY | Use a protected complaint for a known item revision. Replay the receipt, change the revision and introduce an unnecessary synthetic reporter field. One same-scope case for the exact repeat. Conflicting evidence goes to review. Keep reporter identity out of safe reasons. An eligible staff decision may propose recovery; only an independently authorised owner hold stops local hand-off. Refuse a native removal, message or finding of model involvement. |
| F-1299 | REQ-0091-TRUST | DES-0091-W3-SEALED-SYNTHETIC-SAFETY, VAR-0091-W3-ISOLATED-SCHEDULED-SAFETY | FX-0091-W3-NIGHTLY-SAFETY-HOST-HOLD, E2E-0091-TRUST | In the admitted synthetic run, compare a safe result with a planted inert secret marker, missing isolation and timeout. Deny simulated network and product writes inside the check. The payload-free receipt binds the exact fire, attempt, fixture, candidate and expected outcome. With the current receipt missing, offer a valid clean result from an earlier fire of the same schedule with the same fixture, candidate, expected outcome and profile. Current proof remains missing and the host holds the affected route under current policy. Compare an eligible current result. A subsequent clean retry grants no reopening without existing human review. |
| F-1300 | REQ-0091-TRUST, REQ-0091-SCOPE | DES-0091-W3-MINIMUM-SAFE-PROJECTION, VAR-0091-W3-CHECKED-AUTHORISED-SURFACE | FX-0091-W3-COMBINED-DISCLOSURE-FENCE, E2E-0091-TRUST | Use inert synthetic markers in a draft field, metadata and caption. Give two separately allowed facts that identify a fictional person in combination. Try model context, log, clipboard and an old file after restore. Only the existing safe projection may pass. Hold uncertain or forbidden representations, with no matched value in output, reason, log or cache. |
| F-1301 | REQ-0091-MODELS | DES-0091-W3-EXACT-MODEL-ELIGIBILITY, VAR-0091-W3-OWNER-APPROVED-EXACT-MODEL-ROUTE | FX-0091-W3-MODEL-DISPATCH-REVOCATION, IT-0091-MODELS | Admit a synthetic exact site, type, model digest, route and resource profile. Try another digest, unapproved content type and an assistant settings grant. Race model removal and route revocation separately against prompt dispatch, result and release. Hold stale work without installing, calling another provider, choosing a substitute or reusing an old result. |

## Tool boundaries and admitted models

1. Spec 0020 owns grants, untrusted text and revocation. No source,
   attachment, model response or file can authorise an action.
   Use typed preflight and output checks, separate from model
   judgement. Bind exact policy, model, role and rights at queue,
   start, prompt, result, hand-off and replay. Other writers'
   private drafts remain on the PC even with an enabled route.
   Hold unknown privacy, identity or authority.
2. The owner may approve or revoke a typed model allow-list per
   content type with an audit record. Intersect it with spec
   0024 admission, 0042 route, site grants and resource limits.
   It cannot admit a new model or enable off-PC access. Recheck
   versions before use and before keeping output. No fallback.
3. Nightly checks use sealed synthetic data with no credentials,
   network or writes. Bound queue and time. A violation holds
   the affected route pending human review. A later pass cannot
   release it automatically. QR-080 admits fixtures and isolation.

### Target queries and local autosave (1356–1357)

Use the existing 0020 local actions, 0101 writer permissions and
0042 draft store. The session supplies the single site and actor.
Check the exact draft, owner or authorised share, revision, action
scope and current grant. Apply 0032 erasure, 0108 site fencing and
0118 protected storage and transport. A user or enrolled assistant
can save only a permitted local candidate with this authority.

- **1356, target queries per draft.** Save exactly one main query
  and zero to four additional distinct queries for the eligible
  draft revision once targets are set. Pin each current site-local
  0092 evidence ID, eligible query and scope. Bound and normalise
  the proposed values by the admitted query profile. Reject
  duplicate, missing main, excess, wrong-site, erased or unsupported
  targets. Hold if a required current query is unavailable. Store
  the target set with the draft revision and use only those saved
  targets in its 0089 checks. Changing them makes affected checks
  and approvals stale. A target gives no lookup, refresh, spend,
  live site setting, market change or rank signal.
- **1357, draft autosave.** Use the existing `team.autosave_s`
  interval and existing feature 715 draft versions with 0101 bounds
  and the existing retention of the 0052 proposal store. Bind
  assistant work to the existing 0042/0091 stop generation.
  Discard late results after stop even if resumed.
  Keep authorised human edits available. Coalesce eligible
  pending edits within the current writer and draft scope. Bind the
  site, draft, expected base, actor and current grant to the save
  key and canonical edit bytes. Check rights and erasure at commit.
  Save the new local revision and safe receipt in one transaction. An identical
  authorised retry can recover its saved outcome. Changed inputs under the
  same key conflict. A version race preserves the current edition
  and offers the writer a current scoped conflict view. Recheck
  after reconnect and restore. Revocation or erasure withholds
  pending work and cannot resurrect an older revision. Show saved
  only after the durable receipt and show pending or failed work
  accurately. No inherited approval, native write or publication.

| Feature and shared requirement | Owning design and variants |
|---|---|
| F-1356 and REQ-0091-DRAFT | DES-0091-DRAFT-TARGETS binds the saved draft revision and exact 0092 evidence to 0089 checks. VAR-0091-LOCAL-TARGETS uses the admitted local query profile. |
| F-1357 and REQ-0091-DRAFT | DES-0091-DRAFT-AUTOSAVE uses F-0715's local draft update under 0020, with the existing 0052 proposal store and 0101 writer permissions. VAR-0091-AUTOSAVE-WRITER uses the authorised writer's current grant. VAR-0091-AUTOSAVE-ASSISTANT requires the existing enrolled assistant grant and current stop generation. |

The autosave interval takes the 0101 value for the next draft.
The default is 30 seconds and the allowed range is 5 to 600 seconds.
On the existing F-0715 update route, a stale revision uses SUT-803
under spec 0020. Erased or inaccessible drafts keep that route's
safe inaccessible response. Autosave adds no new store or role.

### Wave 4 target and autosave fixture cases

| Planned case | Exact behaviour to verify |
|---|---|
| [FX-0091-W4-EXACT-SAVE-TARGETS](../planning/feature-wave-4.md#acceptance-design-for-features-1338-to-1357) · BDD-0091-DRAFT-TARGETS | For an eligible draft with current own-site 0092 evidence, save one main and zero or four extras. In a proposed set refuse missing main, fifth extra, post-normalisation duplicate or wrong-site evidence. Withdraw evidence before the check or change the draft after capture. Only the eligible exact saved set drives 0089 checks. Stale checks and approvals confer no authority. No lookup or spend. |
| [FX-0091-W4-DURABLE-AUTOSAVE-FENCES](../planning/feature-wave-4.md#acceptance-design-for-features-1338-to-1357) · BDD-0091-DRAFT-AUTOSAVE | Lose the reply and retry the same key and bytes under current authority. Recover one saved revision and its existing receipt. Changed bytes under the key conflict. A newer revision survives a stale save. A revoked share gives no receipt or draft data. After erasure, restoring an older snapshot cannot restore the erased draft. |
| IT-0091-DRAFT-STOP | Stop and Resume before a delayed assistant save arrives. Keep the old-generation save withheld. An independently authorised human save may commit under its own key and revision. Neither route changes native content or publishes. |

QR-197 and QR-080 qualify exact input profiles, actor permission
and fixtures for commit, conflict and replay. Bind the admitted target
normalisation, field mapping, fixture inputs and evidence producer
before deriving tasks under spec 0076. The planned feature cases
use the shared draft implementation and create no new action.

## Data, settings and limits

Keep site-scoped metrics, draft versions, checks, policy and
decisions under spec 0108. Register drafts, prompts, outputs,
derivatives, scorecards, quarantined work, cached reports, queues
and managed files in spec 0032. Apply recipient scrub and audit
before output. Enforce erasure in replay and restore. Use specs 0078
and 0100 for job and file bounds. Add zero public-page queries,
time or bytes. Values below are proposed.

| Proposed setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `content_measure.keep_days` | days from observation | 90 | 1 to 365 | next retention pass |
| `content_safety.handoff_limit` | authorised copies per site day | 10 | 1 to 100 | next hand-off |

## Failure codes

DR-099 reserves SUT-3500 to SUT-3599. SUT-3500 means missing or
incomparable evidence. SUT-3501 means a policy hold. SUT-3502
means an unavailable or revoked model or route. SUT-3503 means
stop or quota applied. SUT-3504 means an invalid revision or
a limit. Permission errors follow spec 0020.

## Security and privacy

Enforce current site, scope and actor under specs 0020 and 0101.
Store audit-safe references only. Do not share private content,
identifiers, personal scores or secrets through reports or drafts.
No metric, recipe or policy rule changes a site or link ranking.

## Requirements

| ID | Requirement | Test design and expected result |
|---|---|---|
| REQ-0091-MEASURE | Metrics MUST name eligible versions, source, population, window and uncertainty. | UT-0091-MEASURE: test held denominators, delayed crawl, overlapping versions, partial 30 days, sparse cohorts, known cost and missing publication. |
| REQ-0091-FEEDBACK | Feedback MUST use eligible cited evidence and protected groups. | IT-0091-FEEDBACK: remove private, sparse and erased evidence and verify no recovery on replay. |
| REQ-0091-RECIPES | Changes MUST remain proposals with versioned evidence. | BDD-0091-RECIPES: given a candidate pattern, keep the prior recipe until approval. Refuse changing protected prompts, models, roles or training through it. |
| REQ-0091-IDENTITY | Labels and authorship MUST use admitted exact attribution. | UT-0091-IDENTITY: refuse member impersonation or missing applicability and show only an approved label proposal. |
| REQ-0091-POLICY | Policy MUST hold prohibited, unknown or unreviewed sensitive output. | BDD-0091-POLICY: given a blocked topic and missing review, disclose the safe reason and provide no ready copy. |
| REQ-0091-STOP | Stop and quota checks MUST fence queued and late output and MUST NOT edit a site. | IT-0091-STOP: race stop, resume and late output. Replay and restore cancelled work. Exercise concurrent clipboard and file charge, retry and local-day rollover with no site effect. IT-0091-DRAFT-STOP covers the enrolled assistant draft-save boundary above. |
| REQ-0091-TRUST | Inputs MUST be treated as data and MUST NOT alter authority. | E2E-0091-TRUST: run synthetic source overrides, planted fake secrets and wrong scopes without granting access, leaking text or writing a site. |
| REQ-0091-MODELS | Each model action MUST use the current admitted route and revision. | IT-0091-MODELS: change role, route or model during a job and refuse stale output without provider fallback. |
| REQ-0091-DRAFT | Autosave and query proposals MUST stay bounded and MUST NOT inherit approval. | UT-0091-DRAFT: accept one main and up to four extras, refuse excess or missing main, check duplicate and stale saves, conflict and erasure on restore. BDD-0091-DRAFT-TARGETS, BDD-0091-DRAFT-AUTOSAVE and IT-0091-DRAFT-STOP give the distinct feature cases above. |
| REQ-0091-SCOPE | Actions MUST enforce site, grant, limits and historical alias resolution and MUST NOT publish. | E2E-0091-SCOPE: resolve both aliases once, test setting bounds, erasure, wrong site and no public action or footprint. |

### Policy and attribution fixture bindings

QR-080 and QR-194 hold exact attribution, policy, stop and
release proof. QR-195 holds disclaimer scope and review.
QR-054 still holds applicable dated rule sources. Use inert
synthetic inputs. These fixtures never supply real protected
material, contact, instruction or external action.

- `DES-0091-W3-ADMITTED-ATTRIBUTION-EVIDENCE` keeps
  eligible actor evidence and separate quote rights.
- `DES-0091-W3-INERT-SOURCE-AUTHORITY-FENCE` validates
  exact typed authority outside text and generated content.
- `DES-0091-W3-CURRENT-QUALIFIED-POLICY` binds current
  rules and, where required, scoped competent human review.
- `DES-0091-W3-CURRENT-STOP-AND-ATOMIC-RELEASE` uses the
  one 0042 stop generation and governed local hand-off quota.

F-1284 and F-1285 have no independent fixture, capability or
count. The F-1283 canonical proof and the F-1165 canonical
label test own those references respectively. F-1294 supplies
applicability evidence to F-1165; it owns no label renderer.
F-1287's resource boundary stays with 0020.

| Feature | Requirement | Design and variant | Named fixture and shared tests | Independent fixture check |
| --- | --- | --- | --- | --- |
| F-1283 | REQ-0091-IDENTITY | DES-0091-W3-ADMITTED-ATTRIBUTION-EVIDENCE, VAR-0091-W3-EXACT-ELIGIBLE-STAFF-OR-SYSTEM | FX-0091-W3-NO-IMPERSONATION-CANONICAL, UT-0091-IDENTITY, E2E-0091-SCOPE | Use an approved staff or system label with exact provenance. Refuse a member voice or identity as the actor and an exact quote without separate reuse rights. The same proof resolves F-1284 once, without an independent action. |
| F-1286 | REQ-0091-TRUST | DES-0091-W3-INERT-SOURCE-AUTHORITY-FENCE, VAR-0091-W3-TYPED-AUTHORITY-OUTSIDE-SOURCE | FX-0091-W3-INERT-SOURCE-AND-RESULT, E2E-0091-TRUST | Use inert synthetic source, metadata and generated-result markers that claim authority. Attempt to omit source provenance or use a foreign grant. Refuse before model or tool work as applicable, and hold an out-of-scope result. No tool, connection or data transfer. |
| F-1288 | REQ-0091-POLICY | DES-0091-W3-CURRENT-QUALIFIED-POLICY, VAR-0091-W3-ADMITTED-OWNER-POLICY-REVISION | FX-0091-W3-CURRENT-OWNER-TOPIC-RULE, BDD-0091-POLICY, E2E-0091-SCOPE | Use one admitted owner-authenticated rule, then replay older policy and try a source or reviewer override. Only the current rule controls. A match or uncertain outcome stays held. |
| F-1289 | REQ-0091-POLICY | DES-0091-W3-CURRENT-QUALIFIED-POLICY, VAR-0091-W3-EXACT-DRAFT-AND-CONTEXT | FX-0091-W3-PROHIBITED-OR-UNCERTAIN-POLICY-HOLD, BDD-0091-POLICY | Use opaque non-graphic synthetic policy markers, including an attributed quote and an ambiguous case. Hold both prohibited and unresolved content. The safe reason contains no source wording or private identity. |
| F-1290 | REQ-0091-POLICY | DES-0091-W3-CURRENT-QUALIFIED-POLICY, VAR-0091-W3-EXACT-GUIDANCE-AUDIENCE-AND-RENDER | FX-0091-W3-QUALIFIED-SAFE-MESSAGING-REVIEW, BDD-0091-POLICY, E2E-0091-SCOPE | Use only non-graphic fictional markers. Compare an eligible exact reviewed rendition with missing guidance, an ineligible reviewer and changed rendition. Each invalid case holds. No harmful material, outreach or service action. |
| F-1291 | REQ-0091-POLICY | DES-0091-W3-CURRENT-QUALIFIED-POLICY, VAR-0091-W3-OWNER-RULE-AND-EXACT-APPLICABILITY | FX-0091-W3-DATED-ELECTION-POLICY-FENCE, BDD-0091-POLICY, E2E-0091-SCOPE | Use a fictional dated policy rule and change its effective revision. Hold unresolved applicability or an unauthorised targeting request. No targeted delivery or protected-policy change. |
| F-1292 | REQ-0091-STOP | DES-0091-W3-CURRENT-STOP-AND-ATOMIC-RELEASE, VAR-0091-W3-ATOMIC-EXPOSURE-CHARGE-AND-RECEIPT | FX-0091-W3-LAST-DAILY-LOCAL-QUOTA-SLOT, IT-0091-STOP | Race local copy and sealed file at the last permitted slot while site-day rollover and stop approach. Commit eligible exposure and charge once. After lost receipt or restore reconcile before retry. Same key with a different digest conflicts. No native publish count. |
| F-1293 | REQ-0091-STOP | DES-0091-W3-CURRENT-STOP-AND-ATOMIC-RELEASE, VAR-0091-W3-QUEUE-PROMPT-RESULT-AND-RELEASE-FENCE | FX-0091-W3-ONE-OWNER-STOP-GENERATION, IT-0091-STOP | Stop around prompt start and then supply previous-generation output, a queued file and a restored job. Refuse each affected release. Resume enables newly admitted work only and leaves permitted human review available. |
| F-1294 | REQ-0091-IDENTITY, REQ-0091-POLICY | DES-0091-W3-ADMITTED-ATTRIBUTION-EVIDENCE, VAR-0091-W3-QUALIFIED-APPLICABILITY-TO-CANONICAL-LABEL | FX-0091-W3-EXACT-LEGAL-LABEL-APPLICABILITY, UT-0091-IDENTITY, BDD-0091-POLICY | Use a fictional current dated rule, audience and exact qualified applicability decision. Change the rule or disable an optional preference; hold unknown or required missing evidence. Use F-1165 FX-0089-W3-AI-LABEL-CANONICAL-GATE for visible-label proof, including hidden rendition. No parallel label gate or native change. |
| F-1295 | REQ-0091-POLICY | DES-0091-W3-CURRENT-QUALIFIED-POLICY, VAR-0091-W3-DECLARED-PURPOSE-AND-MINIMUM-DATA | FX-0091-W3-CHILD-DATA-AND-TARGETING-HOLD, BDD-0091-POLICY, E2E-0091-TRUST | Use fictional request markers for child targeting, unnecessary data fields and uncertain scope with no person records. Hold each affected case before processing or release. Protective discussion supplies no identity inference or collection authority. |
| F-1296 | REQ-0091-POLICY | DES-0091-W3-CURRENT-QUALIFIED-POLICY, VAR-0091-W3-EXACT-USE-SOURCE-AND-QUALIFICATION | FX-0091-W3-SCOPED-TRADEMARK-REVIEW, BDD-0091-POLICY | Use synthetic scoped reviewed wording. Compare a stale source, unproved affiliation and request to change rights or an asset. Hold unsupported wording. Attribution does not prove permission or a legal determination. |
| F-1297 | REQ-0091-POLICY | DES-0091-W3-CURRENT-QUALIFIED-POLICY, VAR-0091-W3-EXACT-CURRENT-QUALIFIED-DISCLAIMER | FX-0091-W3-APPLICABLE-DISCLAIMER-ONLY, BDD-0091-POLICY, BDD-0089-RISK | Use an inert fictional disclaimer entry for one exact policy, audience and purpose. Change topic, jurisdiction, source or scope, or keep an underlying failed gate. Hold hand-off; a library entry cannot clear it or insert itself. |

## How it is tested

Designed, not run. Spec 0076 and QR-080 require feature-specific
evidence, version, policy, permissions, cost, privacy and safe
refusal fixtures.

## Changes from the source

Measurement stays observational unless spec 0094 establishes a
test result. Publish limits and takedown are local holds and staff
proposals. The two historical references add no new behaviour.

## Named local hand-off and guardrail cases

The named Wave 3 cases use the existing interface, applicability,
authority and recovery rules. Each case has the exact input and
observable outcome in its Wave 3 row.

| Feature | Named synthetic case |
|---|---|
| F-1263 | FX-0091-W3-PIECE-EXACT-REVISION-INTERVAL · UT-0091-MEASURE and E2E-0091-SCOPE. Establish a covered piece-level result from the exact matched observation. Withdraw the observation right and leave the analytics totals and their grants unchanged. With no other eligible observation for that interval, the current view and fresh export show unknown attribution. A restored cached result exposes no prior piece value. No source refresh or assumed zero. |
| F-1264 | FX-0091-W3-AUTHORSHIP-COHORT-PROVENANCE |
| F-1265 | FX-0091-W3-RECIPE-COMPARABLE-EVIDENCE |
| F-1266 | FX-0091-W3-EDIT-AMOUNT-DESCENDANT |
| F-1267 | FX-0091-W3-REJECTION-DECISION-DENOMINATOR |
| F-1268 | FX-0091-W3-IDEA-NATIVE-PUBLICATION-PAIR |
| F-1269 | FX-0091-W3-FEEDBACK-GUEST-THEME-RIGHTS |
| F-1270 | FX-0091-W3-PATTERN-PROPOSAL-REVIEW |
| F-1271 | FX-0091-W3-SERIES-EXACT-MEMBERSHIP |
| F-1272 | FX-0091-W3-COST-RECEIPT-LINEAGE. Give the exact site owner one synthetic admitted receipt for a confirmed charge of GBP 0.02. Bind its billed event, lineage, route, currency, units and observation date. Replay the exact event and keep GBP 0.02. Show an uncertain charge as unknown and keep an eligible local resource estimate and another currency separate. Try a writer or other site reading the bill evidence and revoke the source before retained copy, replay and restore. Disclose no private receipt or reconstructable amount to a denied viewer. A restored deferred F-0299 input supplies no estimate, cost ledger entry or charge. Reuse E2E-0042-ANSWER-COST-REFUSAL only for that refusal. No price lookup, new spending or credential use. |
| F-1273 | FX-0091-W3-REACTION-AGGREGATE-DISCLOSURE |
| F-1274 | FX-0091-W3-QUESTION-PROVED-NEWNESS |
| F-1275 | FX-0091-W3-VALUE-ESTIMATE-SIGNED-BASIS |
| F-1276 | FX-0091-W3-VISUAL-EXPOSURE-SCOPE |
| F-1277 | FX-0091-W3-TEMPLATE-COPY-SUPPLIED-CORRECTION |
| F-1278 | FX-0091-W3-QUALITY-GATE-DECISION-KEY |
| F-1279 | FX-0091-W3-SCORECARD-CURRENT-VISIBILITY. Use synthetic comparable scores for the exact active site. With the leaderboard off under 0101 TG-12, preserve an authorised creator's own view and deny comparisons. With exact opt-in and current grants, show only the permitted comparable team projection. Try a staff role without access to a creator-only score and a sibling-site request. Withdraw team opt-in before export and separately revoke the underlying grant. Check preview, filter, counts, export, replay and restore. Hold intersecting results and earlier compatible releases that expose a hidden individual score. No raw personal history, role, pay, work allocation or ranking effect. |
| F-1280 | FX-0091-W3-FIRST-THIRTY-DAYS-NATIVE-WINDOW |
| F-1281 | FX-0091-W3-PORTFOLIO-BALANCE-SITE-SCOPE |
| F-1282 | FX-0091-W3-RETIREMENT-PROPOSAL-EVIDENCE |
