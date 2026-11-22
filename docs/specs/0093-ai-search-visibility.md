# 0093: AI search visibility

See whether supplied AI answers mention the site, cite its pages or
make claims its published pages contradict. See recorded visits from
assistant sites and check crawler access separately. Each view explains
what the evidence can support.

DR-086 limits version 1 answer analysis to files supplied by the owner.
Spec 0042 owns the archive and brand mention rate (298 and 288).
Live answer checks, API spending and automatic answer monitoring are
after version 1. Spec 0092 owns general rival and search research.
Spec 0094 owns scheduled reports. The local measures here use separate
evidence from the crawler and referral views.

Fictional example. A cycle forum imports an answer about winter tyre
pressure. It cites another site. A second answer describes a product
warranty that the forum's dated official page contradicts. The owner
reviews the difference and asks staff to draft a correction. The
recorded referrals for that week do not establish where any answer
came from or whether it was seen by readers.

## Words used here

- **Prompt** means a question or request made to an assistant.
- **Persona** means a fictional reader role used to organise prompts.
- **Citation** means a page address explicitly supplied by an answer.
- **Referral** means a recorded visit with a recognised referring host.
- **Sample** means one declared set of supplied answer observations.

## What this spec designs

| Feature | Version 1 treatment | Requirement |
|---|---|---|
| 55 AI crawler control centre | Preview and approve a supported robots rule change | REQ-0093-ROBOTS |
| 56 AI referral analytics | Show admitted human referrals and covered landing pages | REQ-0093-REFERRALS |
| 285 Prompt library | Local prompts drawn from approved evidence or owner entries | REQ-0093-PROMPTS |
| 286 Persona-based prompt sets | Group prompts by a fictional reader role | REQ-0093-PROMPTS |
| 289 Citation tracking | Track explicit page references in supplied answers | REQ-0093-CITATIONS |
| 290 AI share of voice | Compare named brands within one declared sample | REQ-0093-VOICE |
| 291 AI answer sentiment | Show supported tone labels with evidence | REQ-0093-TONE |
| 292 Accuracy checker | Compare claims with current official passages | REQ-0093-CLAIMS |
| 293 Source gap | Show observed citations and gaps by topic | REQ-0093-CITATIONS |
| 294 AI visibility by language | After version 1 under DR-081 | REQ-0093-SCOPE |
| 295 AI answer change alerts | Automatic monitoring is after version 1 | REQ-0093-SCOPE |
| 296 AI citation recommendations | Evidence-backed page improvement proposals | REQ-0093-DRAFTS |
| 297 AI crawler access check | Inspect the active site's declared policy per page | REQ-0093-ACCESS |
| 300 Brand facts page draft | Draft from current official pages | REQ-0093-DRAFTS |
| 301 AI misinformation correction | Review contradictions and draft corrections | REQ-0093-CLAIMS |
| 302 AI product descriptions | Show supplied descriptions with their evidence | REQ-0093-CLAIMS |
| 303 AI competitor positioning | Compare supported descriptions of named rivals | REQ-0093-TONE |
| 304 Prompt clustering | Group prompts locally by meaning and intent | REQ-0093-PROMPTS |

DR-118 permits the separate
[live Coach evidence](0133-dataforseo-live-coach-evidence.md)
views. They use their own labelled rows in the shared provider
store. They do not change this supplied-answer archive, costs
or tracking policy.

## Prompts and supplied answers

1. The owner edits sets and fictional persona labels. Candidate
   questions may come from guest-visible discussions or permitted
   query evidence from spec 0040. Strip names, account details and
   other identifying text before saving a derived prompt. Keep raw
   connector terms in spec 0040's stores. A local grouping rule
   must pass fixtures for same and different intents under QR-065.
2. The owner supplies each file through spec 0100's bounded import.
   Require a manifest with permission to use it, a source, file
   hash, collection method, service and reported surface, reported
   model or "unknown", time, stable sample and observation IDs,
   and immutable prompt version. Record expected observations or
   unknown, with completed, failed and missing coverage. A deliberate
   repeat has a new observation ID. Identical re-import is a no-op.
   A changed payload at the same ID is refused.
3. Reject unadmitted schemas, unsupported language, missing
   provenance and data that fails the private-data check. Do not
   execute embedded text, render its markup or fetch its references.
   Keep accepted answers escaped and in spec 0042's single archive.
   Label the results "supplied sample". The import never starts a
   model call or sends a prompt.
   Supplied answer files are plain files; archives are not accepted.
   Supplied answer text never enters a prompt or a model. Cap each
   answer at 64 KiB and each file at the spec 0100 limit (answer cap
   proposed).

## What the supplied answers show

1. Group results by the declared sample, service, surface, model,
   prompt set, collection method and time window. Show counts and
   coverage. A failed or missing answer never counts as a result.
   Unknown model identity stays separate. Show unknown coverage as
   unknown and compare only matching scopes.
2. Reject unsafe addresses from derived citations. Drop credentials,
   queries and fragments from records and previews. Count a page
   once per answer by admitted canonical identity. Leave a page
   unresolved if stripping its query changes that identity. Match
   exact admitted hosts. Show source gaps only in covered answers
   that cite another source and establish no active-site citation.
   Incomplete citation coverage stays unknown.
3. Reuse spec 0042's brand matcher and rate for the site and the
   owner's named rivals. Use the same denominator of unique admitted
   answers in the sample for every brand. Count an answer once
   per named brand. Show ambiguous answers separately. Suppress rates
   below the minimum. Rates may total over 100% across brands.
4. Describe tone or positioning only from attributable passages and
   a versioned local classification rule. Show the passage and its
   source. For tone use favourable, mixed, neutral, unfavourable
   or unknown. QR-065 needs labelled fixtures for ambiguity and
   agreement before classification can run. Avoid pooled scores.

## Claims, corrections and drafts

1. Compare claims with the latest eligible version of guest-visible
   official pages chosen on the active site. Show supported,
   contradicted or insufficient evidence with the claim, passage,
   content version and dates of both observations. Stale or
   conflicting pages require rechecking. A contradiction now does
   not prove falsity when the answer was captured. Store no
   separate list of purported facts.
2. A supported contradiction may open a local review card. For
   F-0301 bind the exact answer, claim span, official evidence span,
   applicable source revisions, effective interval and verdict basis.
   An as-captured correction needs independently proved official
   facts for that capture time. A current comparison alone may show
   the observed change with both dates. It gives no basis to say
   the earlier answer was wrong. If approved local wording is used,
   keep a current-only update explicitly dated and separate from
   any as-captured conclusion.

   Before release, check the same time basis and current evidence
   again. If the effective interval or official evidence changes,
   hold the affected wording until reviewed against current support.
   Replay or restore cannot revive the old verdict. Eligible old
   observations stay dated within their own rights. Staff can draft
   a correction or a facts page through the existing Coach review
   route (spec 0041), with cited passages. Show how supplied answers
   describe a product, with the answer and the approved product
   page if available. Recommendations for clearer citation use
   evidence and cannot promise citation or ranking changes.
3. Evidence and drafts stay on the local route. If that route is
   unavailable, show evidence and leave wording to staff. The
   correction workflow sends nothing to an assistant or a rival and
   has no publish action.

## Referrals and crawler access

[Spec 0135](0135-ai-fetches-referrals-and-ranking.md) provides the
native AI traffic UI and uses the same protected referral source.
It does not duplicate the 0034 human aggregate.

1. Use spec 0034's human-only, privacy-checked referring-host counts.
   Use a versioned assistant host list from QR-065. A missing
   referrer is unknown. Until spec 0034 admits its visit floor, show
   "not measured". A landing view needs a privacy-checked joint
   count by host and destination page ID, with no referring path.
   Separate host and page totals cannot establish a landing page.
2. Reuse specs 0044 and 0082 for the current robots policy and
   access evidence. Show whether a named crawler is allowed,
   disallowed or unknown for the page and why. A declaration
   neither proves a visit nor grants access to a private page.
3. Preview a named AI crawler change with affected paths, trade-offs
   and undo. Bind owner approval to the exact site, actor, before
   version and proposed result. Use spec 0021's signed idempotent
   action with the WordPress robots filter or XenForo setting.
   Preserve effective decisions for Googlebot, Bingbot and all
   untargeted groups, including shared and fallback rules. Refuse
   apply on a stale version and undo after an intervening edit.
   QR-065 must admit the route and owner-approved measured robots
   budgets. Before admission, or when the preview cannot prove its
   effects, show advice only.
4. If a sent change loses its reply, keep its original signed intent
   and show “Change result unconfirmed”. Hold another dispatch and
   undo while its result is unknown. Under current read authority,
   reconcile the original effect through the 0021 native receipt
   boundary and the read-only result path admitted for this adapter
   under QR-065. Match the site, logical effect ID, intent digest,
   approved revisions and exact native outcome. An absent, expired
   or ambiguous receipt keeps the result unconfirmed. A known
   terminal refusal shows its own reason. Result recovery changes
   neither the native policy nor the original approval.
5. A verified committed receipt shows “Earlier change applied” with
   its own date and revision. The current policy uses separately
   fresh same-site evidence from the existing robots read. When it
   proves a later revision, show “Rules have changed since then”.
   Keep the later edit and refuse undo of the older result. If the
   current read is stale or unavailable, show “Current rules could
   not be verified” under SUT-2202. Keep the historical result
   separate. A lost role or grant withholds the result or policy
   fields it covered under 0020.

## Supplied sample contracts for features 285 to 304

The detailed rules above define the shared pipeline. These views
may only consume its admitted records and the single 0042 archive.
They never send a prompt, fetch a citation or use a live model key.

### Prompt sets and answer identity

A prompt group binds site, group ID, approved source references,
exact immutable prompt versions and grouping profile. Distinguish
owner-written prompts, scrubbed forum questions and permitted
questions derived from connector evidence. Keep source permissions
and removal attached to every derived prompt version. Do not
store raw search terms here. A displayed prompt is the admitted
scrubbed value, with its source type and exact revision.

A persona is a fictional owner-selected reader role and purpose.
It carries a site, ID, label, note and version. It contains no
actual member identity and never infers a person's attributes.
Show set membership and keep it separate from intent grouping.
Different personas may use the same prompt version. Changing the
role or its prompt list creates a version without changing prior
observations. Intent groups use one admitted local versioned
profile over authorised prompt IDs. Repeatable ties use stable
prompt IDs. Show uncertain assignments for review.

Spec 0042 keys an archive entry by site, sample and observation
ID. Bind it to source rights, payload hash, collection method,
exact prompt and declared persona-set version, capture and
acceptance times, service, surface and reported model. Require
the prompt version to belong to the declared set. Identical
accepted records at that key replay without change. Changed
source, scope, payload or versions give a conflict. Relabelling
the same capture with another ID cannot add an observation.
A distinct repeat binds independent capture evidence and its
own observation ID. Equal answer text from two proved captures
can count twice.

Pin cohort membership, expected coverage, comparable method,
time window and source revisions. Use expected, valid, failed
and missing counts when proved, otherwise show unknown coverage.
Only accepted completed answers are valid. Unknown model
identity stays separate and is never guessed.

### What a sample can establish

| Result | Exact evidence boundary |
|---|---|
| Brand mention rate (288) | Spec 0042 owns the rate. In one admitted comparable sample, count unique valid answers naming the exact approved brand identity, then divide by all unique valid answers in that group. Show this as confirmed mentions. The denominator includes all valid answers, including ambiguous ones. Give disjoint confirmed-hit, confirmed-no-hit and ambiguous counts, plus the minimum and coverage |
| Citations and source gaps (289, 293) | Count each admitted canonical page once per answer, keeping distinct cited pages. A source gap needs one pinned comparable prompt-topic cohort with complete citation coverage. Show a topic gap only if that cohort cites an outside source and no complete answer in the cohort cites the active site. A partial answer may show its observed citations but cannot prove its missing citations. An unresolved address or capture scope blocks a gap claim |
| Share of voice (290) | Reuse the same count and denominator per approved brand, showing exact sample, any co-mentions and named rivals. Shares may sum above 100%. Never pool different services, methods, models or scopes. Keep ambiguous matches visible |
| Tone and positioning (291, 303) | Attach the evaluated answer version, attributable span, named entity and admitted classifier version. The tone label uses the five choices above. Positioning describes the observed claim or role in those spans. Conflicting, ambiguous or insufficient evidence stays unknown. It cannot reveal audience sentiment or market position |
| Accuracy and product descriptions (292, 302) | Pin an active-site eligible official page and a supplied answer. A claim comparison needs matching subject and applicable scope, including time, unit and audience where relevant. Cite both exact spans. Label current comparison separately from capture-time comparison. A current source may support a current verdict only. Use an as-captured verdict only when admitted evidence proves the page version and claim's effective range covered that capture. A conflicting or future-effective source cannot settle a past claim. A description can appear with no verdict. Withdrawn or disputed evidence leaves accuracy unknown |

### Review, coverage and exclusions

Citation advice, a facts page draft and a correction card follow
0041's staff review route using local admitted sources and local
wording only. No Bedrock, connected MCP wording, provider fetch
or other fallback can receive any of this input. Treat supplied
content as inert text. With no local helper, show eligible evidence
alone. Bind each draft claim to exact source ranges and versions.
Recheck site, actor, source permission and evidence at every
review and hand-off. An ordinary staff publication of a facts
page is a separate 0052 and 0021 action with independent current
permission and evidence. This view grants no publication. A
correction never leaves the local review route.

Keep draft text and scoped evidence references in their owning
stores, with no separate list of facts. A source version change
invalidates affected support until review. A denied read stays
denied through copies, exports, notes and reports.

Feature 297 uses the exact current guest-readable origin and
path, an admitted crawler token, and same-origin robots evidence
with source, generation and observation time from 0044 and 0082.
Evaluate the effective rule with that admitted parser and its
group and match precedence. Stale, missing or conflicting rule
evidence or unproved page eligibility gives unknown. Reasons name
the matching rule or safe failure. No target probe, network
discovery or claim of a visit. This read cannot create or confirm
the feature 55 change action, which needs its own plan and proof.

The archive uses 0042 ownership and 0118 protection. Keep one
accepted answer for its retention interval, with scope-safe
history and metadata. A previous spelling or prompt version
shows its own label. Purge on erasure and suppress replay and restore under the
current evidence epoch. History shows only authorised cohort
metadata. Use the same admitted cohort, counts, visibility and
suppression for rates, zeroes, history and count-only MCP output.
Expose only release-safe summaries, with protection against
differencing filters and overlapping groups. No arbitrary MCP
cohort projection. A grant or evidence loss withholds related
counts, notes and historical prompt metadata unless the remaining
information is separately authorised and safe to disclose.

Features 287, 294, 295 and 299 are outside version 1. Current,
legacy, restored and queued settings cannot start an answer check,
open a provider-key route, create a language visibility report,
send an answer-change alert or calculate a billable run.
Spec 0121 multilingual text checking grants none of these paths.
QR-131 requires exact acceptance and release evidence for the
supported views and these version boundaries.

## Data, settings and limits

Use the site-scoped archive in spec 0042 and forced row-level security
under spec 0108. Bind local prompts, labels, evidence and review cards
to the active site. Invalidate derived results when evidence is
withdrawn or erased. Revocation stops pending access. Restore and replay
cannot revive withdrawn evidence. Count archive age from acceptance.
Spec 0032 owns audit and erasure.

Values below are proposed. Reuse specs 0078 and 0100 for job and import
limits. The robots action adds no queries, time or bytes to ordinary
public pages. A separately measured robots-response exception is
required before the action can apply.

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `visibility.sample_min_answers` | whole valid answers | 5 | 1 to 100 | next analysis |
| `coach.chatbot.keep_days` | days in the single archive | 90 | 1 to 365 | next retention pass |
| `visibility.prompt_limit` | whole prompts per site | 500 | 1 to 5,000 | next save |

## Failure codes

DR-086 reserves SUT-2200 to SUT-2299. SUT-2200 means an import was
refused. SUT-2201 means insufficient or incomparable evidence.
SUT-2202 means the robots policy needs rechecking. SUT-2203 means
the platform route or footprint is unavailable. SUT-2204 means
a configured limit was reached. Permission errors follow spec 0020.

## Security and privacy

The owner manages imports, settings and crawler changes. Local reads and staff
drafts need the active site's grants (specs 0020 and 0101). Files and answers
are untrusted. Keep raw answers, claims and other evidence out of logs, MCP
results and external reports. The Coach's `compare_edit_outcomes` can return
an approved count-only summary. No new credential, outbound answer request,
telemetry or ranking signal is added.

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| REQ-0093-PROMPTS | Prompts MUST be site-scoped, scrubbed, bounded and locally grouped by admitted rules. | UT-0093-PROMPTS, BDD-0093-PROMPTS |
| REQ-0093-IMPORT | Only owner-approved and admitted answer files MUST enter one archive, with deduplication and no outbound calls. Supplied answer text MUST NOT enter a prompt or model. | IT-0093-IMPORT, UT-0093-NOPROMPT |
| REQ-0093-CITATIONS | Citation views MUST show distinct references, evidence, sample coverage and observed source gaps. | UT-0093-CITATIONS |
| REQ-0093-VOICE | Brand shares MUST use one named denominator and disclose its coverage. | UT-0093-VOICE, BDD-0093-VOICE |
| REQ-0093-TONE | Tone and positioning MUST show attributable evidence and preserve unknown. | UT-0093-TONE |
| REQ-0093-CLAIMS | Claim verdicts MUST cite current official evidence or show insufficient evidence. | BDD-0093-CLAIMS |
| REQ-0093-DRAFTS | Drafts MUST follow local review and MUST NOT publish or send corrections. | IT-0093-DRAFTS |
| REQ-0093-REFERRALS | Referral reports MUST use admitted human aggregates and MUST NOT infer landing pages from separate totals. | UT-0093-REFERRALS, BDD-0093-REFERRALS |
| REQ-0093-ACCESS | A read-only access check MUST report effective policy and unknowns without fetching blocked or private pages. | UT-0093-ACCESS |
| REQ-0093-ROBOTS | Crawler changes MUST require owner approval, admitted scope, fresh policy and safe undo. Protected and untargeted rules MUST retain their effect. | IT-0093-ROBOTS, BDD-0093-ROBOTS |
| REQ-0093-SCOPE | Version 1 MUST NOT enable live answer checks, API spending, multilingual views or automatic answer monitoring. | E2E-0093-SCOPE |
| REQ-0093-ISOLATION | All actions MUST enforce site and role and respect source withdrawal, erasure and retention. | IT-0093-ISOLATION |
| REQ-0093-LIMITS | Limits MUST bound inputs and jobs and MUST add zero work to ordinary public-page responses. | E2E-0093-LIMITS |

## How it is tested

Designed, not run. Spec 0076 governs readiness. QR-065 tracks the
source formats, measurement rules and crawler action admission.

| Test | Fixture and expected result |
|---|---|
| UT-0093-PROMPTS, BDD-0093-PROMPTS | Given two similar questions and one private question, the owner sees the repeatable group and the private question is refused. Exercise the prompt cap. |
| IT-0093-IMPORT | Repeat an import. Refuse conflict, unsafe data and missing permission. Verify no fetch or execution and show missing sample coverage. Oversize case: an answer of 64 KiB (proposed cap) is accepted whole, one of 64 KiB plus one byte is refused alone with the other answers in the file kept, and a file over the spec 0100 limit is refused whole; no partial text is stored from a refused answer. |
| UT-0093-NOPROMPT | Import answers holding instructions ("ignore your rules", "call this tool"), template markers and a claimed system message. With a recording fake for every prompt builder and model client, run import, citation, voice, tone and draft views. The fake sees no call, and no prompt or model input contains any byte of the answer text. A source scan finds no path from answer text to a prompt builder. |
| UT-0093-CITATIONS | Count two distinct pages in one answer and collapse a repeated page. Keep an unsafe or unresolved reference out. A comparable topic cohort with an own-site citation shows no source gap. Incomplete capture cannot prove absence. |
| UT-0093-VOICE, BDD-0093-VOICE | Given two imported service samples, the owner sees separate brand counts, shared denominators, ambiguity and incomplete coverage. Exercise the minimum. |
| UT-0093-TONE | Compare attributable text and an ambiguous passage. The unknown label remains visible. |
| BDD-0093-CLAIMS | Compare capture-time and current claims using synthetic sources. Try a future-effective version, conflicting contemporaneous sources or later page change. Show recheck or insufficient when matching effective evidence is missing. F-0301 keeps a current-only difference separate from an as-captured correction. A changed claim interval after drafting holds that stale wording and replay. |
| IT-0093-DRAFTS | Try a facts draft, no local helper, hostile markup, MCP wording fallback and source revocation during hand-off. Ordinary staff publication requires separate current 0052 authority. This view cannot send, publish or expose raw output to MCP. |
| UT-0093-REFERRALS, BDD-0093-REFERRALS | Given admitted human visits and separate host and page totals, the owner sees referrals without landing breakdown. With no admitted floor, show not measured. |
| UT-0093-ACCESS | Show allowed, denied and unknown on matching site, path and named token. Test wildcard and exact group precedence, wrong origin, private page, stale or conflicting rules, and an attempt to create or confirm a feature 55 effect. Observe no fetch or change. |
| IT-0093-ROBOTS, BDD-0093-ROBOTS | On each QR-065 admitted WordPress filter and XenForo setting fixture, preview and apply the exact approved change once. Reject stale or uncertain policy, protected effects and unauthorised or conflicting undo. Lose the reply after native commit and then make a later independent policy edit. Show “Change result unconfirmed” before exact read-only recovery. Show “Earlier change applied” for the recovered historical receipt and, only with fresh evidence, “Rules have changed since then” for the current policy. Preserve the later bytes and refuse undo of the older result. Offer stale or unavailable current-policy evidence, an absent receipt and a lost read grant in separate branches. Current policy stays unverified, the absent receipt leaves the effect unconfirmed, and lost authority withholds its result. No replay, recovery or undo performs another native write in this sequence. |
| E2E-0093-SCOPE | Restart and run jobs after an import. Observe no live answer requests, cost actions, language views or monitored alerts. |
| IT-0093-ISOLATION | Refuse cross-site access. Try overlapping count-only filters, expired evidence, withdrawn notes and revoked grants, then restore and retry. Suppress unsafe counts and erased history metadata. |
| E2E-0093-LIMITS | Exercise each bound and its neighbours. Measure ordinary public requests and the separate robots response. |

## Wave 1 crawler controls and referral views

| Feature and design binding | Exact fixture and existing governing check |
|---|---|
| F-0055 · REQ-0093-ROBOTS · DES-0093-W1-NATIVE-ROBOTS-PLAN | FX-0093-W1-ROBOTS-CONTROL, IT-0093-ROBOTS and BDD-0093-ROBOTS. Preview one exact named-crawler change under the admitted WordPress filter or XenForo setting. Use the authenticated owner's exact approval, current actor, site, policy revision and signed 0021 effect. Compare the effective selected and protected decisions, including shared and fallback groups. Apply and undo once only on an admitted native fixture. Refuse stale or uncertain plans, conflicts and replay that would produce another effect. On each adapter lose the apply reply, change the policy independently and reconcile only the original receipt. Separate the dated applied result from the freshly verified current decision. Hold unknown or unauthorised reads and protect the later revision, with no repeat dispatch. |
| F-0056 · REQ-0093-REFERRALS · DES-0093-W1-HUMAN-REFERRAL-VIEW | FX-0093-W1-HUMAN-REFERRALS, UT-0093-REFERRALS and BDD-0093-REFERRALS. Compare admitted human, bot, missing host, below-floor, separate host and page totals, and eligible joint host/landing counts. Require the current QR-065 mapping, same-site source and privacy release. Show supported human host counts and only admitted joint landing counts. An unknown or suppressed group is not measured. No referring path, new collection or inferred landing join. |

F-0297 retains its separate stored-policy read and UT-0093-ACCESS.
It neither proves a crawler visit nor creates or approves a feature
55 change. F-0055 stays advice-only until QR-065 admits each native
route and its measured public footprint. No crawler impersonation,
live proof fetch or hosted hook follows from either binding.

Spec 0034 owns the human aggregates. DR-122 and spec 0135 keep
human referrals separate from verified crawler fetch facts. Feature
56 grants no collection, provider call, ranking input or broader
inference from a host name. Shared source and native tests retain
their owners and supply only their exact evidence.

## Wave 2 supplied-answer verification

These feature bindings use the existing owning requirements and
tests under spec 0076. Each synthetic fixture isolates one assertion
within its named test. Spec 0042 keeps the archive, mention rate and
its own Wave 2 cases. Counts here use the current applicable sample
from that archive through its admitted projection.

| Owning design | Applicable variant and condition |
| --- | --- |
| DES-0093-W2-PROMPTS | VAR-0093-W2-LOCAL-PROMPT uses current scrubbed prompt records and their authorised local grouping profile. VAR-0093-W2-FICTIONAL-ROLE uses only versioned owner-defined fictional roles. |
| DES-0093-W2-SAMPLE | VAR-0093-W2-CITATION-SAMPLE uses admitted canonical references and exact matched sample coverage. VAR-0093-W2-BRAND-COMPARISON uses the 0042 rate basis. VAR-0093-W2-ATTRIBUTED-TONE uses the admitted local rule and exact attributable spans. |
| DES-0093-W2-CLAIMS | VAR-0093-W2-OFFICIAL-COMPARISON compares supplied descriptions or claims with the exact eligible official own-site evidence for their stated scope and time. |
| DES-0093-W2-DRAFTS | VAR-0093-W2-LOCAL-STAFF-REVIEW uses the existing 0041 local protected review route. A missing local helper leaves evidence for staff. |
| DES-0093-W2-POLICY | VAR-0093-W2-STORED-ROBOTS-READ uses current same-origin admitted policy evidence for the exact guest scope. It grants only the read. |
| DES-0093-W2-V1-REFUSAL | The deferred language report and monitoring have refusal tests only. They have no supported version 1 variant. |

| Feature | Existing requirement, design and variant | Existing test and exact fixture assertion |
| --- | --- | --- |
| F-0285 | REQ-0093-PROMPTS, DES-0093-W2-PROMPTS, VAR-0093-W2-LOCAL-PROMPT | UT-0093-PROMPTS, FX-0093-W2-PROMPT-LIBRARY. Supply one owner-written prompt, one eligible guest-derived question and one permitted query-derived question. Keep the three source labels and immutable revisions. Refuse or scrub identifiers before save, then withdraw one source and withhold its derived prompt. No connector terms are copied in and no prompt is sent. |
| F-0286 | REQ-0093-PROMPTS, DES-0093-W2-PROMPTS, VAR-0093-W2-FICTIONAL-ROLE | BDD-0093-PROMPTS, FX-0093-W2-FICTIONAL-ROLE. Put one immutable prompt version into two fictional role sets. Change one set. The other set and earlier answer provenance keep their exact versions. An attempt to use a real member or infer a personal trait is refused. |
| F-0289 | REQ-0093-CITATIONS, DES-0093-W2-SAMPLE, VAR-0093-W2-CITATION-SAMPLE | UT-0093-CITATIONS, FX-0093-W2-CITATION-COUNT. Give one accepted answer with two distinct canonical pages, a repeat of one, an unsafe URL and an identity that depends on its query. Count exactly two eligible canonical pages once each. Refuse the unsafe reference and keep the query-dependent identity unresolved. No link is followed. |
| F-0290 | REQ-0093-VOICE, DES-0093-W2-SAMPLE, VAR-0093-W2-BRAND-COMPARISON | UT-0093-VOICE, BDD-0093-VOICE, FX-0093-W2-BRAND-VOICE. Use an admitted synthetic minimum and six unique valid answers with four confirmed mentions of one brand, three of another and two co-mentions. Give each brand six as denominator and show coverage and ambiguity separately. An incompatible model or method supplies a separate sample. |
| F-0291 | REQ-0093-TONE, DES-0093-W2-SAMPLE, VAR-0093-W2-ATTRIBUTED-TONE | UT-0093-TONE, FX-0093-W2-TONE-SPANS. Supply attributable favourable, mixed, neutral and unfavourable spans plus an ambiguous mention. Bind each known label to that answer version, entity and admitted rule revision. Preserve unknown for ambiguous attribution. No answer text enters a prompt or model. |
| F-0292 | REQ-0093-CLAIMS, DES-0093-W2-CLAIMS, VAR-0093-W2-OFFICIAL-COMPARISON | BDD-0093-CLAIMS, FX-0093-W2-CLAIM-TIME. Compare a dated supplied claim with a proved applicable official capture-time page and a different current page. A future-effective version cannot settle the earlier claim. Conflicting and withdrawn sources hold the affected verdict, with exact version evidence. |
| F-0293 | REQ-0093-CITATIONS, DES-0093-W2-SAMPLE, VAR-0093-W2-CITATION-SAMPLE | UT-0093-CITATIONS, FX-0093-W2-TOPIC-GAP. Pin one comparable topic cohort. Show a gap for complete evidence with a cited outside source and zero own-site citation across the cohort. Add one eligible own-site citation, then separately an unresolved reference or partial capture. The first removes the gap and the incomplete states cannot prove it. |
| F-0294 | REQ-0093-SCOPE, DES-0093-W2-V1-REFUSAL | E2E-0093-SCOPE, FX-0093-W2-LANGUAGE-REFUSAL. Try an unsupported-language answer and a restored language view through the current workspace and multilingual authoring. Refuse the unsupported import and leave the deferred visibility report unavailable. No language job, translation or report is created. |
| F-0295 | REQ-0093-SCOPE, DES-0093-W2-V1-REFUSAL | E2E-0093-SCOPE, FX-0093-W2-MONITORING-REFUSAL. Import two dated answers, restore an old alert rule, then replay it through the scheduler. Keep the permitted local observations. Verify zero answer-change alerts, repeating checks, provider requests and monitoring jobs. |
| F-0296 | REQ-0093-DRAFTS, DES-0093-W2-DRAFTS, VAR-0093-W2-LOCAL-STAFF-REVIEW | IT-0093-DRAFTS, UT-0093-NOPROMPT, FX-0093-W2-CITATION-ADVICE. Open current citation evidence and compare an admitted local wording route with one unavailable. Staff get a scoped proposal or evidence alone. The supplied answer never enters a prompt or model. No provider fallback, outbound message, native edit or promised citation. |
| F-0297 | REQ-0093-ACCESS, DES-0093-W2-POLICY, VAR-0093-W2-STORED-ROBOTS-READ | UT-0093-ACCESS, FX-0093-W2-CRAWLER-POLICY-READ. Match the exact guest-readable origin and path, admitted crawler token and current stored same-origin rule evidence. Show the proved allow or deny and matched rule. A private path, wrong origin or stale/conflicting rules gives unknown. No live probe, visit claim or feature 55 change. |
| F-0300 | REQ-0093-DRAFTS, DES-0093-W2-DRAFTS, VAR-0093-W2-LOCAL-STAFF-REVIEW | IT-0093-DRAFTS, FX-0093-W2-FACTS-DRAFT. Draft from two authorised current official site passages with exact source versions. Withdraw one before review. Withhold the affected claim support and keep only eligible private review. No separate fact store or publication effect. |
| F-0301 | REQ-0093-CLAIMS, REQ-0093-DRAFTS, DES-0093-W2-DRAFTS, VAR-0093-W2-LOCAL-STAFF-REVIEW | BDD-0093-CLAIMS, IT-0093-DRAFTS, FX-0093-W2-CORRECTION-DRAFT. A supplied answer matches the effective official figure at capture. A later change gives dated current evidence only. Separately give a proved as-captured contradiction and one local correction with both exact spans. Change its effective proof before hand-off and try old preview, replay and restore. Hold stale wording and retain eligible dated evidence only. Try revoked access, another site, sending and publishing. No external or native effect. |
| F-0302 | REQ-0093-CLAIMS, DES-0093-W2-CLAIMS, VAR-0093-W2-OFFICIAL-COMPARISON | BDD-0093-CLAIMS, FX-0093-W2-PRODUCT-DESCRIPTION. Use a supplied product description with an admitted alias, an ambiguous alias and an eligible matching official page. Show the exact attributed description and supported evidence where applicable. Missing official evidence or ambiguous identity gives no accuracy verdict. |
| F-0303 | REQ-0093-TONE, DES-0093-W2-SAMPLE, VAR-0093-W2-ATTRIBUTED-TONE | UT-0093-TONE, FX-0093-W2-RIVAL-FRAMING. Compare two attributable rival passages in one admitted comparable sample and a third with unclear identity. Keep exact spans, named entity, observed role and source revision. The ambiguous passage gives unknown. No market or audience score. |
| F-0304 | REQ-0093-PROMPTS, DES-0093-W2-PROMPTS, VAR-0093-W2-LOCAL-PROMPT | UT-0093-PROMPTS, BDD-0093-PROMPTS, FX-0093-W2-PROMPT-GROUPING. Group authorised prompt IDs under the same admitted profile with related, unrelated and ambiguous prompts and an exact tie. Repeat with reordered input, a withdrawn source and over-cap request. Stable prompt IDs break the repeatable tie. Keep uncertainty visible and apply the current cap and withdrawal. No send. |

DR-086 still limits answer analysis to owner-supplied files. The
supplied answer text stays out of all model input and MCP. Keep
review evidence and safe counts in their existing protected paths.
DR-081 still governs site-language scope. The 0135 fetch and referral
paths supply no answers, samples or authority to these features.

QR-065 and QR-131 still own exact source, privacy, rule, platform
and fixture proof. Confirm that each test binds the exact allowed
source and role, applicable versions, expected output, protected
evidence and refusal before task derivation. Fixture descriptions
show planned assertions; observed evidence stays separate.

## Changes from the source

Imported samples replace live answer input under DR-086. Features
294 and 295 are after version 1. Spec 0042 retains its archive and
mention rate. Paid answer collection and tracking costs there are
also after version 1.
