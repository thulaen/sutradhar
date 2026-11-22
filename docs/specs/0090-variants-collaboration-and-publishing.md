# 0090: Variants, collaboration and publishing

Plan and review variants of a draft, then give staff an approved
copy to use in their site tools. The toolkit makes cited proposals.
It cannot publish, upload, schedule a site write or undo a page.

Spec 0084 owns evidence and rights, spec 0089 owns required gates,
and spec 0101 owns staff permissions. Spec 0020 owns tool access.
Spec 0094 owns experiments. Specs 0096 and 0098 own social and
newsletter hand-off. This spec owns editorial coordination.

Fictional example. Two editors propose a shorter guide and a
forum version. Both point to the same approved master. A changed
source holds the forum version for review. Staff take the guide
pack after approval and publish it within their site.

## Words used here

- **Master** means the exact source draft a variant derives from.
- **Variant** means another proposal with its own audience or form.
- **Hand-off** means approved local copy or file use by staff.
- **Embargo** means a time before which staff cannot take a pack.
- **Conflict** means two proposals disagree about the same revision.

## What this spec designs

| Feature | What staff reviews | Requirement |
|---|---|---|
| 1188 Beginner and expert versions | Beginner and expert versions from one source | REQ-0090-VARIANT |
| 1189 Length versions | Short, standard and long scoped proposals | REQ-0090-VARIANT |
| 1190 Audience versions | Named audience variants without personal targeting | REQ-0090-VARIANT |
| 1191 Channel versions | Forum, WordPress and email versions | REQ-0090-VARIANT |
| 1192 Skimmable versions | Skimmable list with equivalent facts | REQ-0090-VARIANT |
| 1193 Regional versions | After version 1 under DR-081 | REQ-0090-SCOPE |
| 1194 Seasonal versions | Dated seasonal wording | REQ-0090-VARIANT |
| 1195 Easy-read versions | Easy-read structure with equivalent meaning | REQ-0090-VARIANT |
| 1196 Printable versions | Accessible printable draft | REQ-0090-VARIANT |
| 1197 Section announcement versions | Section-specific wording for an eligible target | REQ-0090-VARIANT |
| 1198 SEO split test variants | Title or description variants for separately approved tests | REQ-0090-TEST |
| 1199 Persona introductions | Fictional persona openings | REQ-0090-VARIANT |
| 1200 Expert sidebars | Advanced sidebar with evidence scope | REQ-0090-VARIANT |
| 1201 Glossary-rich versions | Glossary-rich text with eligible definitions | REQ-0090-VARIANT |
| 1202 Variant manager | One master and its exact variant revisions | REQ-0090-LINEAGE |
| 1203 Variants follow the master | New local proposals when the master changes | REQ-0090-LINEAGE |
| 1204 Variant performance comparison | Comparable variant evidence with unknowns | REQ-0090-MEASURE |
| 1205 Regional terms | After version 1 under DR-081 | REQ-0090-SCOPE |
| 1206 Formal and informal versions | Formal and informal channel wording | REQ-0090-VARIANT |
| 1207 Channel summaries | One-line and paragraph summaries | REQ-0090-VARIANT |
| 1223 Assign drafts | Assign an eligible author or editor | REQ-0090-COLLAB |
| 1224 Inline draft comments | Comments pinned to an exact span or revision | REQ-0090-COLLAB |
| 1225 Suggestion mode | A separate suggestion with a visible diff | REQ-0090-COLLAB |
| 1226 Assistant handoff | A scoped hand-off between authorised assistants | REQ-0090-COLLAB |
| 1227 Commissioning briefs | Sourced commissioning brief | REQ-0090-COLLAB |
| 1228 Guest author submissions | Admitted signed-site draft intake | REQ-0090-MEMBERS |
| 1229 Contributor agreement tracking | Versioned contributor agreement evidence | REQ-0090-MEMBERS |
| 1230 Deadline reminders | Local deadline reminder | REQ-0090-CALENDAR |
| 1231 Content board | Site-scoped editorial board | REQ-0090-COLLAB |
| 1232 Team draft templates | Versioned team template | REQ-0090-COLLAB |
| 1233 Reusable snippet library | Scoped rights-checked reusable snippets | REQ-0090-COLLAB |
| 1234 Named piece owner | Named responsible staff member | REQ-0090-COLLAB |
| 1235 Review-by dates | Dated review and stale state | REQ-0090-CALENDAR |
| 1236 Owner review reminders | Local review reminder to the owner | REQ-0090-CALENDAR |
| 1237 Parallel section drafting | Independent proposed sections and reviewed merge | REQ-0090-COLLAB |
| 1238 Writer feedback | Role-scoped writer feedback | REQ-0090-COLLAB |
| 1239 Editorial notes history | Versioned editorial notes | REQ-0090-COLLAB |
| 1240 Bylines and acknowledgements | Consented bylines and acknowledgements | REQ-0090-MEMBERS |
| 1241 Member article requests | Admitted site requests for articles | REQ-0090-MEMBERS |
| 1242 Member content votes | Admitted protected request counts | REQ-0090-MEMBERS |
| 1243 Two-key publishing | Draft and exact human approval for manual use | REQ-0090-HANDOFF |
| 1244 Scheduled publishing | Local publication plan and staff reminder | REQ-0090-CALENDAR |
| 1245 Multi-site drafts | Separate single-site packs and canonical proposals | REQ-0090-SITES |
| 1246 Save as WordPress draft | WordPress-ready local draft pack | REQ-0090-HANDOFF |
| 1247 One-step thread creation | XenForo-ready local thread pack | REQ-0090-HANDOFF |
| 1248 Full post creation | WordPress-ready post fields and approved assets | REQ-0090-HANDOFF |
| 1249 Changes approved as a diff | Exact approved diff for manual edit | REQ-0090-HANDOFF |
| 1250 Unpublish or roll back | Local unpublish or restore proposal for staff | REQ-0090-RECOVERY |
| 1251 Embargoed release | Local embargo and timed hand-off eligibility | REQ-0090-CALENDAR |
| 1252 Placement requests | Placement request draft | REQ-0090-HANDOFF |
| 1253 Forum and blog cross-posts | Separate forum and blog packs for manual use | REQ-0090-SITES |
| 1254 Publishing checklists | Versioned checklist for manual publication | REQ-0090-HANDOFF |
| 1255 After publishing | Local request to review internal links | REQ-0090-FOLLOWUP |
| 1256 After publishing | Local social sharing proposal under 0096 | REQ-0090-FOLLOWUP |
| 1257 After publishing | Local newsletter proposal under 0098 | REQ-0090-FOLLOWUP |
| 1258 Series navigation updates | Proposed navigation refresh with fresh approval | REQ-0090-FOLLOWUP |
| 1259 Publishing permissions | Show admitted staff permissions without granting any | REQ-0090-HANDOFF |
| 1260 Publication log | Record observed manual publication and approvals | REQ-0090-FOLLOWUP |
| 1261 Emergency unpublish switch | Owner-only local hand-off hold and manual recovery plan | REQ-0090-RECOVERY |
| 1262 Media upload while publishing | Rights-checked media manifest for manual use | REQ-0090-HANDOFF |

## Variants and revisions

1. Pin each variant to one immutable master revision, site, form,
   intended use, rights and evidence. Show what varies and what
   sources remain required. Never use a real reader profile to
   choose a draft. Regional wording and terms in 1193 and 1205
   are after version 1 under DR-081.
2. A change to the master may queue one new local proposal per
   variant using a stable site, master revision and form key.
   Preserve the previous revision, but
   withdraw its ready decision when affected facts or rights
   change. An edit cannot inherit approval. If the process fails,
   keep the proposal in review and show why.
3. Compare eligible variants using protected, scoped observations
   from specs 0034 and 0040. Use admitted methods and the same
   canonical definition, exact variant version, population and dates.
   Apply source suppression. Missing evidence stays unknown.
   A difference cannot establish causation. Spec 0094 alone
   owns experimental assignment, activation and outcome. Passing
   a variant here cannot approve a test.

### Variant forms and comparisons (1188 to 1206)

Each proposal records its exact same-site master, purpose, intended
form and audience, evidence and rights, and the differences from the
master. Pinned claims, units, meaning, required warnings, disclosures,
citations, source credit and access boundaries survive an adaptation.
Recheck exact grant and credit placement at clipboard or file
release, queue, retry and restore. Withdraw affected managed copies
under 0032 and 0118. An omitted
field and its permitted reason remain visible in review. When a
required field cannot fit, hold the form. Each eligible variant needs
its own current 0089 gates and exact human approval for the ordinary
local hand-off. A draft or its approval creates no publication, send,
assignment, activation or ranking effect.

| Feature | Exact form, lineage or measure |
|---|---|
| 1188 Beginner and expert | Bind both approved audience profiles to the same master. Explain prerequisites and terms in the beginner version. Add only sourced advanced detail in the expert version. Required qualifications and safety meaning are common to both. |
| 1189 Length | Propose short, standard and long drafts with the admitted bounds and counting rule for each form. Show the difference from the master. Optional material can move or fall away under the profile. A longer version cannot add padding as evidence, and a shorter one cannot lose a required condition. |
| 1190 Audience | Use named editorial profiles such as new members, experts and sponsors. Check each destination's disclosure and rights again. Never select content for an individual, expose private examples or inherit a sponsor-only right for a public form. |
| 1191 Channel | Bind the admitted eligible forum, WordPress article or email-copy profile and exact intended destination. Check visible meaning, tokens and permitted formatting for each draft. A site pack uses its own native form capability. Email copy stays local under 0088, 0098 and DR-116. No recipient export, connection, native draft or send. |
| 1192 Skimmable | Produce ordered headings and bullets for the same scope. Preserve the relationship of each claim, prerequisite, source and caution to its item. Use a layout with unambiguous reading order. Hold an extraction that changes order where order matters. |
| 1193 Regional versions | DR-081 defers UK/US or other regional site variants. British English remains the site variant language. DR-113 does not admit a regional destination profile, conversion, job, hand-off or restored proposal. |
| 1194 Seasonal | Bind the current event or season, approved site time zone, intended interval, destination and source revisions. Keep proposed dates labelled. Qualify time precision and boundaries before dating ready copy. A future or expired revision follows its local review and hand-off policy. No clock change can switch live wording. |
| 1195 Easy-read | Use the admitted accessible layout, short units and the required text alternatives, and preserve meaning, warning and action order. Only approved illustrations enter the form. An automated readability grade cannot stand in for the form's accessibility and meaning review. |
| 1196 Printable | Prepare the admitted accessible print layout against the exact master and show the intended page profile. Check pagination, reading order, complete notices, source references and accessible alternatives. Use QR-039 and 0100 for any qualified file output. If that output profile is unavailable, keep the layout in review. No print or upload action. |
| 1197 Section announcement | Bind the exact same-site section, intended readers, current native form capability, allowed purpose and approved section facts. A namesake section or a private source cannot supply public copy. Each section has its own approved local draft. |
| 1198 SEO split test variants | Prepare only the admitted title or description candidate for feature 21 under spec 0094. Bind exact field and proposed value, candidate digest, native page and field revision, baseline value, site and scope. To attach to an existing experiment, pin spec 0094 experiment ID and immutable plan revision. Unattached wording creates no plan or assignment. A draft cannot select pages for treatment, assign groups, arm, activate or change live content. |
| 1199 Persona introductions | Use a labelled fictional audience profile whose traits, task and intended purpose are stated. No saved reader identity, inferred sensitive characteristic or member history enters the profile. Proposed wording cannot select a visitor or personalise delivery. |
| 1200 Expert sidebars | Keep the sidebar separately addressable in the proposal. Bind each advanced claim and citation to the master scope and its own evidence. Preserve sequence and accessible navigation. Required warnings and prerequisites stay available in the main text. |
| 1201 Glossary-rich | Bind an eligible glossary revision, term senses, audience and current targets. Explain terms where they aid comprehension. Hold a conflicting definition or unproved link. Repeated labels cannot count as extra explanation or insert site links. |
| 1202 Variant manager | Keep one immutable parent revision for each variant revision and one site boundary throughout. Show master status, variant form and destination, source dependencies and current gate and approval state. Reject a second parent, a cycle or cross-site attachment. The manager does not reassign a prior approval. |
| 1203 Variants follow the master | Consume the committed same-site master revision through the existing local queue. Deduplicate by site, master identity and revision, and the stable configured variant identity and profile. Preserve distinct variants of one form for different audience or destination. Replays share the logical proposal. An out-of-order event cannot replace a newer ready view or resurrect old rights. Withdraw affected readiness on changed rights or evidence and require a fresh human review of the new proposal. Keep prior authorised history. |
| 1204 Variant performance comparison | Use an admitted 0034 or 0040 measure with exact site, canonical page and revision, proved variant exposure interval, metric time basis, eligible population and coverage. Hold shared-exposure or mixed-period buckets unless evidence supports a split. Never allocate page totals by assumption or join personal histories. Keep source denominators, coverage and suppression. Use the existing disclosure history to withhold inferable totals or differences. Do not merge incomparable provider and site measures. Unknown and suppressed stay distinct, and observed differences do not prove a cause. Any experiment result remains under spec 0094. |
| 1205 Regional terms | DR-081 defers automatic regional terminology and audience or market mappings. Ordinary British-English wording can use the site's approved vocabulary under its existing rules. No region setting, substitution pass, job or restore route is added. |
| 1206 Formal and informal | Use the approved channel and audience tone profiles for separate local proposals. Preserve claims, obligations, disclosures, calls to action and semantic state in both. A tone change cannot soften a warning, introduce a promise or imply authority. |

#### Variant evidence and deferral fixture bindings

QR-079 and QR-191 hold the exact local forms, evidence and
release profiles. QR-039 separately holds PDF output. The
1193 and 1205 fixtures enforce DR-081's deferral across all
routes. DR-113 and DR-133 do not admit site market variants.
A refusal case has no positive design or variant.

- `DES-0090-W3-EXACT-SITE-VARIANT` pins master revision,
  purpose, audience, rights, disclosures and applicable
  form, accessibility and time evidence before hand-off.
- `DES-0090-W3-BOUNDED-EXPERIMENT-WORDING` prepares only
  the eligible F-0021 field and exact scope. The 0094
  experiment still owns assignment and activation.
- `DES-0090-W3-EXACT-VARIANT-LINEAGE` holds replay to
  current evidence, rights and the one admitted parent.
- `DES-0090-W3-COMPARABLE-RELEASED-EVIDENCE` uses only
  comparable 0034/0040 observations with disclosure proof.

| Feature | Requirement | Design and variant | Named fixture and shared tests | Independent fixture check |
| --- | --- | --- | --- | --- |
| F-1193 | REQ-0090-SCOPE | DR-081 refusal only | FX-0090-W3-REGIONAL-SITE-VARIANT-REFUSAL, E2E-0090-SCOPE, UT-0090-VARIANT | Try a US spelling and units variant from each entry point including restored work. Refuse before creating job, preset, candidate or hand-off. An eligible ordinary British-English proposal remains available. |
| F-1194 | REQ-0090-VARIANT, REQ-0090-CALENDAR | DES-0090-W3-EXACT-SITE-VARIANT, VAR-0090-W3-VERIFIED-SITE-TIME-WINDOW | FX-0090-W3-SEASONAL-COPY-TIME-FENCE, UT-0090-VARIANT, UT-0090-CALENDAR | Bind the source date, local zone, destination and interval. Try unresolved repeated hour and expiry between review and copy. Hold dated ready copy; passage of time cannot switch live wording. |
| F-1195 | REQ-0090-VARIANT | DES-0090-W3-EXACT-SITE-VARIANT, VAR-0090-W3-EQUIVALENT-WARNINGS-AND-READING-ORDER | FX-0090-W3-EQUIVALENT-EASY-READ, UT-0090-VARIANT, IT-0089-FORMAT | Compare a complete equivalent short layout and one losing a prerequisite or accessible alternative. Even a favourable score keeps the incomplete candidate held. Preserve meaning, reading order and warning placement. |
| F-1196 | REQ-0090-VARIANT, REQ-0090-HANDOFF | DES-0090-W3-EXACT-SITE-VARIANT, VAR-0090-W3-ACCESSIBLE-LOCAL-PRINT-REVIEW | FX-0090-W3-EXACT-PRINT-PROJECTION, IT-0089-FORMAT, UT-BG-27, BDD-0090-HANDOFF | No PDF while QR-039 is unresolved. For a separately admitted local file use sealed exact bytes with accessible layout and full disclosure and credit. A missing reference, clipped notice or removed credit holds hand-off. No printer or upload. |
| F-1197 | REQ-0090-VARIANT, REQ-0090-SCOPE | DES-0090-W3-EXACT-SITE-VARIANT, VAR-0090-W3-CURRENT-SAME-SITE-SECTION | FX-0090-W3-EXACT-SECTION-ANNOUNCEMENT, UT-0090-VARIANT, E2E-0090-SCOPE | Bind exact section and native capability before opening sources. A namesake, hidden fact or stale target cannot enter public wording. No cross-site or publication action. |
| F-1198 | REQ-0090-TEST, REQ-0090-SCOPE | DES-0090-W3-BOUNDED-EXPERIMENT-WORDING, VAR-0090-W3-EXACT-PLAN-FIELD-AND-BASELINE | FX-0090-W3-EXPERIMENT-OPTION-ONLY, IT-0090-TEST, IT-0094-GUARD | Under the admitted F-0021 WordPress profile, prepare a local title or description bound to exact site, page, field, candidate digest and current baseline revision and value. Save one unattached proposal and verify no experiment record is created. In a separate run pin the exact existing 0094 experiment ID and immutable plan revision as a proposal reference. Change the field, ID, plan revision or native baseline before hand-off. Hold the affected proposal, including on replay. An exact local approval permits only wording for review or hand-off. IT-0094-GUARD supplies shared stale-baseline proof only. Observe no page selection, plan change, reservation, assignment, arming, activation or native write in either run. |
| F-1199 | REQ-0090-VARIANT, REQ-0090-SCOPE | DES-0090-W3-EXACT-SITE-VARIANT, VAR-0090-W3-LABELLED-FICTIONAL-AUDIENCE | FX-0090-W3-FICTIONAL-PERSONA-ONLY, UT-0090-VARIANT, E2E-0090-SCOPE | Use only an admitted labelled fictional audience and purpose. Refuse individual reader records and requests to infer sensitive traits. No personal selection or delivery. |
| F-1200 | REQ-0090-VARIANT | DES-0090-W3-EXACT-SITE-VARIANT, VAR-0090-W3-CURRENT-CITATION-AND-WARNINGS | FX-0090-W3-EXPERT-SIDEBAR-MAIN-TEXT-FENCE, UT-0090-VARIANT, IT-0089-FORMAT | Use eligible advanced evidence with an accessible sidebar after the essential main text. Move an essential warning into the optional panel or withdraw its citation. Hold affected wording, with no warning loss or gated access change. |
| F-1201 | REQ-0090-VARIANT | DES-0090-W3-EXACT-SITE-VARIANT, VAR-0090-W3-ELIGIBLE-DEFINITION-AND-TARGET | FX-0090-W3-CURRENT-GLOSSARY-TERM-SENSE, UT-0090-VARIANT, UT-0089-LINKS | Compare a current eligible sense with a retired definition, hidden target and unexplained repeated label. Use only approved explanations and checked targets. No link insertion or keyword padding. |
| F-1202 | REQ-0090-LINEAGE, REQ-0090-SCOPE | DES-0090-W3-EXACT-VARIANT-LINEAGE, VAR-0090-W3-EXACT-ONE-PARENT-REVISION | FX-0090-W3-SINGLE-IMMUTABLE-VARIANT-PARENT, IT-0090-LINEAGE, E2E-0090-SITES | Use one pinned same-site parent revision. Refuse a second parent, cycle or parent from another site. The version and existing history survive, with no inherited approval. |
| F-1203 | REQ-0090-LINEAGE, REQ-0090-SCOPE | DES-0090-W3-EXACT-VARIANT-LINEAGE, VAR-0090-W3-STABLE-VARIANT-IDENTITY-AND-PROFILE | FX-0090-W3-MASTER-EVENT-REPLAY-FENCE, IT-0090-LINEAGE, E2E-0090-SCOPE | Duplicate and reorder committed master events for two same-form variants with distinct audiences. Reconcile by site, master and revision, and stable variant identity and profile. Keep separate current proposals. Revoke a source and restore; stale readiness cannot return. |
| F-1204 | REQ-0090-MEASURE, REQ-0090-SCOPE | DES-0090-W3-COMPARABLE-RELEASED-EVIDENCE, VAR-0090-W3-PROVED-EXPOSURE-AND-DISCLOSURE | FX-0090-W3-PROTECTED-VARIANT-OBSERVATION, UT-0090-MEASURE, E2E-0090-SCOPE | Compare exact 0034 or 0040 same-site measures with proved exposure, population and coverage. Hold mixed periods or unproved splits. Suppress overlapping totals that reveal a withheld count and recheck after revoke at filter and export. No causal winner. |
| F-1205 | REQ-0090-SCOPE | DR-081 refusal only | FX-0090-W3-REGIONAL-TERMS-REFUSAL, E2E-0090-SCOPE, UT-0090-VARIANT | Try inferred market, term remapping and old mapper replay. No deferred setting, preset, job, native or tool action. The approved British-English glossary remains eligible. |
| F-1206 | REQ-0090-VARIANT | DES-0090-W3-EXACT-SITE-VARIANT, VAR-0090-W3-FORMAL-AND-INFORMAL-EQUIVALENCE | FX-0090-W3-TONE-WITHOUT-SEMANTIC-CHANGE, UT-0090-VARIANT | Compare formal and informal local wording to the same approved facts, obligations and warning. Hold an added guarantee or softened warning. Preserve required scope in both forms. |

QR-191 refines QR-079 for these variants. Qualify the exact language,
form limits, source and rights generations, comparable measures and
file profiles. Exercise concurrent master revisions, crash, replay,
withdrawal and restore. Manual hand-off still uses the owning route.

#### Wave 3 local accessibility and channel variant fixtures

QR-191 and QR-079 retain the exact form and current gate checks.
Use an eligible same-site master and exact per-variant rights.
Hold unsupported forms. DR-116 continues to exclude send,
provider access and recipient export from local email copy.

| Feature | Owning form and requirement | Planned fixture and shared checks |
|---|---|---|
| F-1188 | Beginner and expert versions · REQ-0090-VARIANT | FX-0090-W3-BEGINNER-EXPERT-SAME-MASTER-WARNING · UT-0090-VARIANT and E2E-0090-SCOPE. Bind both approved audience profiles to the same exact master. Keep prerequisites and warnings in the beginner candidate and require sources for extra advanced detail. Hold a removed warning or unsourced assertion. Each form retains scope, obligations and its own review. |
| F-1189 | Length versions · REQ-0090-VARIANT | FX-0090-W3-LENGTH-EXACT-BOUND-CONDITIONS · UT-0090-VARIANT and E2E-0090-SCOPE. Use exact admitted counting rule and bounds per short, standard and long profile. Test each bound and one word over and show the difference from master. Refuse shorter text missing a required condition and unsupported padding. Optional reordering cannot change facts or source scope. |
| F-1190 | Audience versions · REQ-0090-VARIANT | FX-0090-W3-AUDIENCE-NAMED-RIGHTS-NO-TARGETING · UT-0090-VARIANT and E2E-0090-SCOPE. Use named editorial profiles and exact destination rights. Try sponsor-only evidence in the public form and individual reader behaviour as selection input. Withhold restricted examples and personal selection. Keep rights and disclosures per audience without inheriting another profile. |
| F-1191 | Channel versions · REQ-0090-VARIANT | FX-0090-W3-CHANNEL-QUALIFIED-LOCAL-PROFILES · UT-0090-VARIANT, BDD-0090-HANDOFF and E2E-0090-SCOPE. Compare exact eligible forum, WordPress and local email copy profiles with destination-bound visible meaning, delimiters/tokens and permitted formatting. Wrong-platform markup holds. Refuse recipient export or sending through 0100 or another connector. Approved channel wording grants no native draft, send or account effect under DR-116. |
| F-1192 | Skimmable versions · REQ-0090-VARIANT | FX-0090-W3-SKIMMABLE-ORDER-ITEM-CAUTION · UT-0090-VARIANT and E2E-0090-SCOPE. Create a local headings and bullets variant from exact same master. Keep reading order and each item’s associated claim, evidence, prerequisite and caution. Hold an extraction that reorders mandatory steps or moves a caution away from its governing item. No change in underlying meaning or approval inheritance. |
## Editorial work and member input

1. Use specs 0020 and 0101 roles. Assign an eligible owner or
   staff member. Assignment cannot share a private draft. Explicit
   sharing must name the target and scope. Recheck both actors'
   current grants under QR-055. Pin comments and changes to a
   span and revision. Mark lost spans. Templates and snippets
   pin sources, purpose, rights and expiry.
2. Give each assistant only its admitted evidence and section
   revision. Recheck recipient tool and owner permissions on
   preview, queued work and replay. Other people's private
   drafts stay on the PC even with an owner-enabled route.
   Require admitted local processing or refuse. Transfer no
   credential, private note, raw pack or approval by default.
   A named person resolves conflicts into a new cited version.
3. QR-079 admits typed guest, member and vote routes through
   site sign-in and specs 0021 and 0101. The service pulls signed
   input. Admit bounded, idempotent intake, current consent and
   measured site footprint first. Use staff drafts or approved
   supplied totals otherwise. Unsupported routes stay unavailable.
   Export no voter identity. Keep only protected vote totals.
   Recheck exact reuse and attribution under specs 0084 and 0098.
4. Local reminders inherit specs 0019 and 0099 clock and catch-up
   rules. Scheduler readiness follows QR-081. Use one stable
   task key, planned instant and site zone. Report a missed
   hand-off without promising exact release. Check current time,
   embargo and pack expiry at copy. A due time cannot publish,
   send or approve.

### Channel summaries and collaboration (1207, 1223–1236)

Spec 0090 owns these proposals and local coordination records.
Use specs 0020, 0052, 0084, 0089 and 0101 for admitted workflow
actions, evidence, required checks and staff rights. Pin each
request to one active site, actor, action, target and expected
revision. Pin the relevant scope, evidence, grants, policy,
capability and recipients or assignees. Reject mismatches before
the request can reveal a private object. Check both sides of
each transfer and use the current rights under QR-055.

Bind request identity to actor, site, action and canonical digest.
Commit authorisation, exact record revision, receipt and outbox
together. Same key and different digest conflicts. Reproject a
prior result under current rights and hide erased objects. Recheck rights at
preview, commit, hand-off, queued
execution, replay and restore. Withdraw affected managed copies,
cached views and queued work on rights loss or erasure under specs
0032 and 0118. Treat content and metadata as inert. Use bounded,
site-scoped records, queue and exports under specs 0078, 0100 and
0108. Audit decisions without private text, identity attributes
or credentials. No recipient send, new provider route, public page
effect or approval by implication. Any signed site intake remains
the distinct, separately admitted read in feature 1228.

Apply current signed site staff and per-object permissions to team
actions and boards before showing cards, counts or cursors. Disable
commands for hidden objects. A command compares the current record and
actor at commit. Template and snippet reuse produces a fresh scoped
0052 proposal. Responsible staff ownership grants no other site rights.

#### Channel summary

- **1207, channel summaries.** Derive one line and one paragraph
  from the same exact master, applicable evidence, rights and
  audience. Label the channel and format. Pin its admitted line,
  paragraph and length rules. Keep material limits, disclosures
  and supported meaning in both forms. If an admitted format
  cannot fit the required facts, hold that form. Review each
  exact rendition afresh when evidence, audience or master
  changes. Both outputs stay as local drafts under spec 0096's
  manual hand-off boundary. No channel lookup or dispatch.

#### Channel summary fixture binding

QR-079 and QR-192 hold the exact channel and audience forms,
bounds and review. `DES-0090-W3-REVIEWED-CHANNEL-SUMMARIES`
uses one current master and separately reviewed renditions.
0096 owns the manual boundary. No new channel or send.

| Feature | Requirement | Design and variant | Named fixture and shared tests | Independent fixture check |
| --- | --- | --- | --- | --- |
| F-1207 | REQ-0090-VARIANT, REQ-0090-HANDOFF | DES-0090-W3-REVIEWED-CHANNEL-SUMMARIES, VAR-0090-W3-ONE-MASTER-TWO-REVIEWED-FORMS | FX-0090-W3-EXACT-DUAL-CHANNEL-SUMMARY, UT-0090-VARIANT, BDD-0090-HANDOFF | Pin one master and the exact line and paragraph profiles. Hold the form if its length cannot preserve a material caveat or disclosure. Changed master, audience or rights invalidates its approval. Keep reviewed copies local under 0096, with no lookup or dispatch. |

#### Local editorial work

- **1223, assign drafts.** Bind assigner and eligible current
  author or editor to the exact local task and draft. Check the
  requested assignment against its current revision and limit.
  Assignment carries no new read, share, ownership or approval
  right. Require an explicit, separately authorised share for a
  private draft. Reassignment fences access from the earlier task.
- **1224, inline draft comments.** Pin the exact revision,
  field and span with a stable source anchor. Keep the comment
  within its authorised audience. On revision change mark a
  missing or ambiguous anchor as detached. Show its original
  only while still entitled. Any reattachment is an explicit
  reviewed action against the current revision.
- **1225, suggestion mode.** Keep a separate proposal with
  affected spans and a visible diff. Accept or reject against
  the exact current draft as an eligible editor. Acceptance
  creates a new local revision, with new content checks. On
  competing edits, keep a conflict for human review. No silent
  merge, inherited approval or native edit.
- **1226, assistant hand-off.** A current staff plan names the
  sending and receiving assistants, target section revision,
  purpose and proposed evidence. Require distinct owner permission
  for the enrolled receiver and exact allowed projection under 0118.
  Before dispatch recheck authenticated recipient, process and actor
  grants. The plan itself authorises neither routing nor disclosure.
  Other people's private drafts require admitted local processing
  or stay held. Carry no tokens, private notes, raw source pack
  or approval. Fence each queued or repeated dispatch and returned
  output on current rights and revision. Discard late or parked data
  after withdrawal. Record the safe result without private payloads. A
  missing safe receiver holds
  the hand-off without fallback. A person resolves conflicts.
- **1227, commissioning brief.** Record exact site, person or
  role, purpose, intended audience, scope, source pack, terms
  and review expectations. Mark proposed terms and due dates as
  proposals. Keep unapproved private source details out of the
  brief. Approval of the brief cannot contact, assign or sign
  for the writer.
- **1231, content board.** Derive Awaiting work, Writing and
  Editorially finished columns from admitted local workflow states. Label
  finished as an editorial state. Bind moves to the current
  record, actor, allowed transition and site. Exclude private
  items from cards, aggregates and continuation data for
  ineligible readers. No board move grants access, acceptance,
  publication or delivery.
- **1232, team draft templates.** Version the team, form, purpose,
  admitted typed placeholders, defaults, source and rights.
  Validate a new draft against the pinned template and current
  rights. An update or retirement does not rewrite existing
  drafts. A template cannot create an action, field permission
  or approval. Refuse a wrong team or disallowed default.
- **1233, reusable snippets.** Bind the exact words, source
  spans, rights, use, audience, applicability, attribution and
  expiry. Check insertion against the destination and current
  applicable rules, including disclosures under spec 0101.
  Mark an affected draft for review when a snippet is replaced
  or withdrawn. Do not insert, expand or copy inaccessible
  source text through a reusable field.
- **1234, named piece owner.** Record the eligible staff member
  responsible for maintenance of the piece independently of
  a writing or editing assignment. Change ownership only by a
  currently authorised action with the expected revision.
  Handle an ineligible owner as unassigned and needing review.
  Ownership itself confers no staff role, content right or
  publication authority.

#### Guest submissions and agreements

- **1228, guest author submissions.** General guest-author intake
  has no admitted platform route. Until QR-079 and QR-192 qualify
  a bounded native signed-in form and protected signed PC pull,
  staff use their eligible local draft path. DR-080 establishes
  native sign-in and signed messages. Sponsor and lead forms have
  independent purposes. Do not reuse them here. No anonymous form,
  callback, staff application access or direct owner-PC route.
  Qualification must bind submitter and reviewer separately, exact
  platform, capability, consent generation, digest, site, allowed
  fields and caps. Reject active content, attachments and unrelated
  metadata. Receive native submit and outbox together, then dedupe
  the signed PC pull by site, intake ID and immutable revision.
  Check signatures, scopes and current consent, including replay,
  revocation and restore. Reproject only a safe current receipt.
  Intake adds no site account, role, reuse or publication right.
- **1229, contributor agreement tracking.** Keep a protected,
  versioned record keyed by site and immutable agreement ID. One
  contributor may have separate agreements. Bind the signed and
  approved revisions, source/erasure state and its
  exact terms digest, contributor reference, permitted use,
  destination, audience, channel, expiry, acceptance evidence and
  correction or withdrawal state. Show
  missing, expired, withdrawn and unknown evidence distinctly.
  A recorded agreement is not proof of permission for a new
  excerpt or use. Recheck the exact-use grant under specs 0084
  and 0098. Tracking does not send, sign, renew or extend an
  agreement. Withdraw derived views and jobs under spec 0032.

#### Dates and reminders

Run a local-only 0019 job for the authorised desktop notice surface.
Apply 0099's site-zone and quiet-hours rules only. Pin site, object,
action, durable occurrence, planned instant, deadline and current
recipient or ownership. Preserve local time, zone and the resolved
instant. Refuse unknown clock or ambiguous offset. Defer for quiet
hours only while eligible. Expired or inactive owner/site work gets
no catch-up notice. Compare deadline, role and active record, then
commit the notice fence and receipt together. Use a durable key across
restart and restore. A revision change cannot repeat a delivered
notice. A later occurrence needs a new exact approved plan. Closure,
rights loss or erasure withdraws stale notices. No outside 0099
delivery, even if configured. Keep text and identifiers private.

- **1230, deadline reminders.** Bind the current assignment
  milestone, due date and eligible staff recipient. Keep the
  reminder separate from the assignment change. A removed or
  completed assignment cancels its old reminder. Show only
  authorised local due or missed status. No provider delivery
  or change to the work.
- **1235, review-by dates.** Pin the time-sensitive piece and
  exact revision, review date, evidence and authorised setter.
  Show stale or review due when the admitted boundary is reached.
  A date edit does not mark review complete. A new exact review
  or revision needs its own approved review date or an explicit
  no-date outcome under the form profile.
- **1236, owner review reminders.** Bind the named piece owner,
  review date and their current revisions. Reassign or close by
  recording a new eligible plan when needed. Fence notice to
  the previous owner. A reminder is a local prompt only.
  It cannot change the deadline or complete the review.

QR-192 qualifies these exact form profiles under QR-079, with
the shared rights, clock and deferral checks referenced there.

#### Wave 3 local assignments, rights and calendar fixtures

QR-192 and QR-079 retain exact local form, clock and route
qualification for F-1223–F-1236. F-1237 uses QR-193 and
the same F-1226 bounded recipient contract. F-1228 holds
live intake pending exact signed producer and pull admission.
Synthetic examples grant no route, dispatch, native write or send.

| Feature | Owning form and requirement | Planned fixture and shared checks |
|---|---|---|
| F-1223 | Assign drafts · REQ-0090-COLLAB | FX-0090-W3-ASSIGN-EXACT-TASK-SEPARATE-SHARE · BDD-0090-COLLAB and E2E-0090-SCOPE. Use exact site, task and draft revision and a current eligible assigner. Hold assignment to an editor without an expressly authorised private share. With that share, admit one eligible local assignment only. Race role loss and reassignment and fence prior access. A task supplies no read, ownership or approval right. |
| F-1224 | Inline draft comments · REQ-0090-COLLAB | FX-0090-W3-INLINE-COMMENT-DETACHED-ANCHOR · BDD-0090-COLLAB and E2E-0090-SCOPE. Pin comment to exact revision, field and stable span under current audience grant. Drop or duplicate the anchor in a newer revision and show detached only where entitled. Withdraw old read access and suppress old content and protected details. No implicit reattachment or draft edit. |
| F-1225 | Suggestion mode · REQ-0090-COLLAB | FX-0090-W3-TRACKED-SUGGESTION-EXPECTED-REVISION · BDD-0090-COLLAB and IT-0090-LINEAGE. Keep one separate tracked diff on the exact eligible draft revision. A current authorised editor’s exact acceptance gives one new local checked revision. Competing changes or rights loss hold a conflict or safe denial. Replaying the old acceptance gives no overwrite, inherited approval or native write. |
| F-1226 | Assistant handoff · REQ-0090-COLLAB | FX-0090-W3-ASSISTANT-HANDOFF-RECIPIENT-FENCE · BDD-0090-COLLAB and E2E-0090-SCOPE. Pin exact sending actor, authenticated enrolled receiver and process grant, purpose, section revision and owner-approved disclosure projection. Refuse a broader or unapproved recipient and other people’s private draft on an off-PC route. Revoke while queued and discard a late result after withdrawal. No token, private note, raw pack, approval inheritance or fallback route. |
| F-1227 | Commissioning briefs · REQ-0090-COLLAB | FX-0090-W3-COMMISSION-BRIEF-PROPOSED-TERMS · BDD-0090-COLLAB and E2E-0090-SCOPE. Use exact site, role, purpose, audience, permissible source pack and review scope. Mark terms and due dates as proposed and withhold a private unapproved excerpt. An instruction to contact or appoint a writer stays inert. Approving the brief does not assign, contact, sign or make a commitment. |
| F-1228 | Guest author submissions · REQ-0090-MEMBERS | FX-0090-W3-GUEST-INTAKE-ROUTE-HOLD-REPLAY · IT-0090-MEMBERS and E2E-0090-SCOPE. Without exact QR-079 and QR-192 admission for the 0021 native form and protected signed pull, show unavailable before pulling content. An independently eligible staff draft remains local. In a separate admitted synthetic adapter case bind submitter and reviewer separately, site, capability, schema, intake ID, revision, digest, consent generation and allowed bytes. Commit native submit and outbox together. Exact replay under current rights keeps one protected intake and its current safe receipt. The same site, intake ID and revision with changed digest refuses without replacing the intake. Withdraw consent before another pull and reject stale retry and restored access. Refuse wrong site, over-cap input, active content, attachments, an anonymous sender and substituted sponsor or lead scope. No account, role, reuse or publication grant. |
| F-1229 | Contributor agreement tracking · REQ-0090-MEMBERS | FX-0090-W3-AGREEMENT-INDEPENDENT-SCOPE-WITHDRAWAL · IT-0090-MEMBERS and E2E-0090-SCOPE. Use two protected immutable agreement IDs for the same contributor, distinct exact terms digests, purposes and evidenced acceptance. Distinguish unknown, expired and withdrawn. Withdraw only the affected scope and hold derived work. A separate excerpt grant must still pass 0084/0098. No inferred permission, sign, renew or send. |
| F-1230 | Deadline reminders · REQ-0090-CALENDAR | FX-0090-W3-DEADLINE-ONE-ELIGIBLE-LOCAL-NOTICE · UT-0090-CALENDAR and shared UT-0019-DST only for admitted occurrence identity and catch-up. Pin the current assignment milestone, eligible staff recipient, local time, zone revision, proved instant, deadline and durable occurrence. An unresolved repeated hour or unknown clock holds before scheduling and cannot inherit 0019's automatic fold mapping. In a resolved eligible run cross the deadline while the PC sleeps and catch up after wake before expiry. Commit the local notice fence and receipt together and observe at most one desktop notice. Replay after restart or restore without repeating it. Run quiet hours until after expiry, then separately close the assignment or revoke the role before catch-up. No stale notice is delivered. An existing configured 0099 delivery route still receives no reminder. Show only safe authorised local due or missed status, with no protected text or assignment change. |
| F-1231 | Content board · REQ-0090-COLLAB | FX-0090-W3-CONTENT-BOARD-FILTER-BEFORE-COUNT · BDD-0090-COLLAB and E2E-0090-SCOPE. Derive editorial columns from current allowed workflow state and expected revision. Omit a now-hidden card before counts, labels, pagination and cursor. Refuse the stale card command without confirming it exists. An editorial finish marker confers no access, acceptance, publication or delivery. |
| F-1232 | Team draft templates · REQ-0090-COLLAB | FX-0090-W3-TEAM-TEMPLATE-RETIRED-NO-DEFAULT-WIDEN · BDD-0090-COLLAB and E2E-0090-SCOPE. Validate a current pinned team/form/purpose template against typed placeholders and rights. Retire its version without rewriting existing drafts. Try wrong-team creation, active instruction and private-field default. Hold ineligible creation without granting field visibility, execution or approval. |
| F-1233 | Reusable snippet library · REQ-0090-COLLAB | FX-0090-W3-SNIPPET-EXACT-RIGHTS-DESTINATION · BDD-0090-COLLAB and BDD-0090-HANDOFF. Bind exact sourced words, attribution, destination, use/audience, expiry and current 0101 disclosure. Refuse expired or wrong-purpose text. Withdraw quote rights after insertion and hold clipboard, file and replay of dependent draft. Current permitted text retains its credit. No hidden snippet expansion. |
| F-1234 | Named piece owner · REQ-0090-COLLAB | FX-0090-W3-PIECE-OWNER-NO-ROLE-INHERITANCE · BDD-0090-COLLAB and E2E-0090-SCOPE. Replace a named eligible owner using current authoriser and expected piece revision. Keep the distinct writing assignment. Revoke the new owner’s eligibility and show unassigned with current review, holding older assignment replay. No staff, share, content or approval grant. |
| F-1235 | Review-by dates · REQ-0090-CALENDAR | FX-0090-W3-REVIEW-BY-BOUNDARY-REVISION-HOLD · UT-0090-CALENDAR and UT-0019-DST. Pin exact piece revision, authorised setter, review evidence and local date/zone. Cross the resolved due boundary and show review due or stale as eligible. Hold ambiguous local time, older restore and changed revision until a newly approved date or permitted no-date outcome. Date edits cannot complete review. |
| F-1236 | Owner review reminders · REQ-0090-CALENDAR | FX-0090-W3-OWNER-REVIEW-NOTICE-REPLAY-FENCE · UT-0090-CALENDAR and UT-0019-DST. Queue a desktop-only occurrence for the exact review date and owner revision. Reassign or change date before dispatch and fence stale recipient. Restore after a delivered notice and replay its key with zero duplicate. A later eligible occurrence needs separately approved exact plan. No outside delivery or review completion. |
| F-1237 | Parallel section drafting · REQ-0090-COLLAB | FX-0090-W3-PARALLEL-SECTIONS-REVIEWED-MERGE · BDD-0090-COLLAB and E2E-0090-SCOPE. Keep two scoped contributions on the exact common base and authorised sources. Try an overlap, stale base and private evidence outside the assigned section. Apply F-1226 receiver and disclosure rules before assistant input, result and merge. Revoke consent and discard late output. Only a named editor resolves conflicts into a new candidate with fresh checks and lineage. No overwrite, source transfer or inherited approval. |

## Manual publication and recovery

1. Apply all required spec 0089 gates. Named human approval
   pins actor grant, candidate, sources, rights, policy, gates,
   capability, destination, manifest and embargo. Reuse only
   spec 0052's read, submit and check steps. MCP has no approval,
   publishing, clipboard or file hand-off grant.
2. Check all versions, current grant, clock, rights, hold and
   expiry atomically at every hand-off, including retry and asset
   extraction. Use spec 0032 recipient checks, scrub and audit
   and spec 0100 bounds. Refusal leaves no partial output.
   Pair WordPress and XenForo-only packs. Unsupported formats
   remain in review. Use approved local assets and no upload.
3. A cross-site plan is owner-only desktop metadata. No cross-site
   MCP action, job or token. Each site builds, cites, approves
   and copies its own pack. No default transfer of evidence,
   grants or assets. Canonical proposals require eligible public
   address, version and rights. Staff decide site changes.
4. Bind a publication record to site, candidate, destination,
   named human approval, assistant attribution and timestamps
   with audit-safe references. Separate staff claim from spec
   0089's matched, different and unavailable observations.
5. A current confirmed publication can create one local request
   per destination, version and kind. Link, social, newsletter and
   series requests follow their own approval routes. Erasure,
   changed visibility, expiry or pause cancels queued requests
   and fences replay. A request cannot insert links or send.
6. Stamp owner-only pause and resume. A pause stops queued and
   new hand-offs. Provide the target, known prior version and a
   manual recovery plan. If the target changed, show conflict.
   The toolkit cannot hide, unpublish, overwrite or roll back
   live content.

### Editorial collaboration, manual hand-off and recovery (1237 to 1262)

Use the existing editorial and hand-off rules above. Bind the active
site and actor, exact form and candidate, evidence and rights, target
and destination, policy, gate decisions and named human review. Scope
staff and contributor permissions to the exact action. New approval is
required when any bound revision changes. An approval, observation or
reminder cannot grant a native effect. Recheck the complete binding at
clipboard, file and asset extraction, retry and restore. Treat imported
text, media, comments and assistant proposals as data.

| Feature | Exact collaboration or intake contract |
|---|---|
| 1237 Parallel section drafting | Give each eligible collaborator an exact section and base revision with only permitted sources. Keep submissions separate. Apply exact 1226 recipient and disclosure grant to each assistant route, before input, receipt and merge. Hold other writers' private drafts without an admitted local route and keep raw source packs private. A named editor reviews the common base and differences, resolves overlaps and creates a new candidate with section and source lineage. A late or conflicting edit cannot overwrite another contribution or inherit approval. |
| 1238 Writer feedback | Bind a note to its author, intended recipient, exact draft revision and applicable span. Keep feedback visible only in the expressly shared scope. Identify a lost anchor and retain the original where permitted. Feedback cannot change the draft, author rights or score. |
| 1239 Editorial notes history | Keep attributable note revisions on the exact site, piece and approved audience. Display only eligible history, separating corrections, lost anchors and erasure from current notes. A current share cannot expose a withdrawn note, private reviewer identity or restricted prior wording. |
| 1240 Bylines and acknowledgements | Use the contributor's exact current grant for the name or alias, relation to the piece, wording, destination and audience. Keep required credits with each derivative. Staff and assistants cannot accept on the contributor's behalf. Hold changed or withdrawn attribution. |
| 1241 Member article requests | Accept an eligible member's typed request only through the admitted native sign-in and signed pull of specs 0021 and 0101 under QR-079. Bind exact signed reply, site, request ID, kind, schema, digest, authority and consent generation, allowed fields and scope. Recheck before persisting and refuse revoked consent on replay. Check duplicate receipts and bounds before keeping the protected proposal. Unadmitted native intake stays unavailable. Staff may create a separate local proposal without claiming it was a member submission. |
| 1242 Member content votes | Use the admitted native producer and current request/version, scope and consent. The producer checks eligibility and duplicate votes under its own contract. The service receives only permitted protected totals, with correction, withdrawal and release generation. Hold sparse, incomplete or inferable results. Approved supplied totals retain their own source and scope. Voter identity, personal text and raw ballots stay out of shared projections. Prevent overlapping releases and differences disclosing a vote. Recheck consent and release generation on retry or restore. |

The publication forms below create reviewed local packs, observations
or staff requests. Apply 0032 recipient and audit rules, 0100 output
bounds and 0118 protected files. Permit only the exact approved
platform and destination. Refusal leaves no partial hand-off. Treat an
uncertain copy result as uncertain and reconcile the existing receipt
before any repeat. No tool can approve, take a pack, send a recipient
message or write to a site.

| Feature | Exact publication, recovery or follow-up contract |
|---|---|
| 1243 Two-key publishing | Require an attributable assistant draft and separate named human approval of the exact candidate and current 0089 gates. A held gate or changed destination blocks hand-off. Give the approved pack to eligible staff for manual use. Record the approval, copy and any later publication observation separately. |
| 1244 Scheduled publishing | Record a planned local time with site zone, time precision and resolution of ambiguous slots. Under 0019, 0099 and QR-081 create at most one local reminder per approved plan and occurrence. Revision, pause or expiry fences queued reminders. Show a missed hand-off on catch-up. The clock cannot approve, send or publish. |
| 1245 Multi-site drafts | Keep the plan as owner-only desktop metadata. Each registered site uses a separate active session and independently establishes permitted facts, assets, staff approval and hand-off. Propose a canonical address only with eligible exact public target, version and rights under 0102. Hold a stale or unresolved target. No cross-site tool command, copied grant, asset transfer or canonical write. |
| 1246 Save as WordPress draft | Require an admitted WordPress draft-field profile, including the eligible post type and target scope. Give staff the exact local pack and the intended WordPress form. Hold wrong host, changed schema or unsupported field. Creating or updating a native draft is a separate manual act. |
| 1247 One-step thread creation | Require XenForo and admitted node capabilities. Review the local title, body, prefix, tags and optional poll as one pack against current rules. An unapproved field cannot silently disappear. No thread, poll or attachment is created by the toolkit. |
| 1248 Full post creation | Require WordPress, the intended post type, and allowed category, tags, excerpt and featured-image references. Check field values, rights and alternative text in the exact pack. Unsupported taxonomy or an ungranted asset holds. No post or media record is created. |
| 1249 Changes approved as a diff | Bind the known live base revision, exact local candidate, changed fields and approved evidence. Show additions, removals and protected context safely. If the current base differs, show a conflict and require a fresh diff and approval. The toolkit cannot apply even an approved patch. |
| 1250 Unpublish or roll back | Require current authority for the exact recovery plan. Original publication approval grants no recovery authority. Bind action kind, canonical target, expected native revision and recoverable before-revision digest and grant where relevant. Unknown or changed revisions hold. Staff operate native tools separately. Present the scope and consequences as a recovery proposal. Unknown or intervening state holds instructions tied to the old revision. A manual reversal needs its own plan, evidence and approval. An approved local unpublish proposal cannot release earlier-version restore instructions even if its site, target and current revision match and the earlier digest is eligible. Require separate approval of the exact restore proposal. No hide, restore, delete or live undo. |
| 1251 Embargoed release | Bind the exact pack to the approved embargo, its authoritative instant and source zone and precision. Recheck the admitted clock at hand-off. Before the boundary, withhold copy, files and asset extraction. After the boundary, eligibility still needs current approval and rights. Unknown time, rollback or a changed embargo holds. Nothing is dispatched at the boundary. |
| 1252 Placement requests | Prepare a separately approved local request for the exact eligible destination, placement type, audience and dates. Use current native capability and applicable sponsor or safety review. Missing eligibility holds. A draft changes no pin, featured position, slot, recipient or communication. |
| 1253 Forum and blog cross-posts | Create separate platform-specific local packs with a permitted original source, each site's independent grants and approval, and any eligible canonical proposal. Preserve rights and attribution per destination. A hold or successful copy on one side creates no effect or authority on the other. |
| 1254 Publishing checklists | Bind the checklist revision to the exact form, candidate and applicable policy. Record each item against its eligible proof or prescribed inapplicability. Separate required gate outcomes and manual confirmations. A missing or obsolete checklist cannot bypass 0089, 1186 or exact human approval. |
| 1255 Internal-link follow-up | After an exact matched 0089 observation, deduplicate a local review request by destination, published revision and link-review kind. Hold if visibility or authority changes. Existing link review supplies later checks and approval. No link, ranking update or automatic fetch follows from this request. |
| 1256 Social sharing follow-up | Bind the current matched publication, eligible public projection, asset rights and approved local 0096 destination profile. Create one local sharing proposal for the revision and route. Recheck on copy. No platform connection, account discovery or posting. |
| 1257 Newsletter follow-up | Bind the current matched publication and eligible content to a local 0098 newsletter plan. Recheck current purpose, rights, applicable consent and suppression before permitted copy. No subscriber identity, list transfer, campaign creation or sending. DR-116 confers no outreach route. |
| 1258 Series navigation updates | Use a current matched publication and the exact same-site series membership, order, navigation base and guest-visible destinations. Create a versioned local navigation diff once per effective revision. A move, changed sequence or lost visibility requires fresh review. Approval releases a local proposal only. |
| 1259 Publishing permissions | Show only the current eligible native rights observation and applicable product action grants for the authorised actor, target and platform under 0020, 0021 and 0101. Distinguish unproved, refused and allowed scope. No tool or screen can set a native role or publish from this view. |
| 1260 Publication log | Record site, candidate, destination, manual claim, pinned approval and safe assistant attribution under current actor scope. Keep claim timestamp distinct from the 0089 observed time and result. Preserve correction history and deduplicate the exact receipt. No confirmation or attribution can be inferred from a copied pack or absent observation. |
| 1261 Emergency unpublish switch | Only the owner can place or remove a local hand-off pause. Persist its generation apart from packs. Check it atomically at actual copy or file release, also after retry or archive restore. Hold missing or stale pause history. Keep manual recovery authority separate. A lost or stale request cannot revive a paused hand-off. Resume checks rights, expiry and exact approval again. This is separate from the AI generation stop. No live page is unpublished. |
| 1262 Media upload while publishing | Supply a local manifest for eligible supplied assets only. Apply 0097 destination/use rights and exact 0087/0100 safe format and projection. A stock URL or manifest cannot start a lookup, download, purchase or activation. Bind original and final bytes and credit to the approved pack. Pin asset, derivative digest, destination profile, credit, alternative, safe format and expiry. Check staged assets and final pack together before exposing either. Hold a partial pack, substituted file, unusable destination or revoked right. No host media upload, attachment or library mutation. |

For 1261, order the owner pause against the actual local effect.
Staging a pack supplies no completion receipt. If the pause takes
effect first, release neither clipboard bytes nor a file. If the exact
hand-off commits first, pause subsequent releases. Keep the
original completed outcome even if its reply was lost. Resolve
the original action before any repeat under the existing rules.
Show its receipt only within current site, actor, rights and
erasure scope. A pause cannot settle an uncertain prior outcome.
Keep that action held until the original receipt or exact effect
can be verified. A restart, retry or older restored pack never
repeats a completed or uncertain transfer. Clipboard copy and
file release retain their own approvals and receipts. A prior
receipt does not authorise a second effect or manual recovery.

#### Scoped collaboration and manual hand-off fixtures

QR-079 and QR-193 hold exact authority, native capability,
rights, safe output and recovery. QR-055 holds applicable
signed actor evidence. QR-081 holds the admitted reminder job
and clock proof. These are distinct planned synthetic cases.
No feature in this group aliases another or gains authority
from its title. The unadmitted 1241/1242 native projections
are refusal cases until their producer proof is admitted.

- `DES-0090-W3-SCOPED-COLLABORATION-RECORD` pins exact
  site, actor, revision, subject and eligible audience.
- `DES-0090-W3-ADMITTED-PROTECTED-INTAKE` accepts only
  the owning signed scope or separately approved protected
  evidence where allowed. Keep consent and release current.
- `DES-0090-W3-EXACT-MANUAL-PACK` pins eligible platform,
  complete fields, evidence, rights and human approval.
  Keep independent sites separate. No native operation.
- `DES-0090-W3-LOCAL-TIME-OR-HANDOFF-FENCE` rechecks
  clock, approval, pause, destination and current rights.
  No toolkit send, approval by timer, publication or rollback.

| Feature | Requirement | Design and variant | Named fixture and shared tests | Independent fixture check |
| --- | --- | --- | --- | --- |
| F-1238 | REQ-0090-COLLAB, REQ-0090-SCOPE | DES-0090-W3-SCOPED-COLLABORATION-RECORD, VAR-0090-W3-NOTE-AUTHOR-RECIPIENT-AND-SPAN | FX-0090-W3-REVISION-BOUND-WRITER-FEEDBACK, BDD-0090-COLLAB, E2E-0090-SCOPE | Move the anchored paragraph after the approved note. An eligible reviewer sees a lost anchor without reassignment. Revoke recipient scope before replay and deny the note. No draft, grant or score change. |
| F-1239 | REQ-0090-COLLAB, REQ-0090-SCOPE | DES-0090-W3-SCOPED-COLLABORATION-RECORD, VAR-0090-W3-NOTE-REVISION-AND-ERASURE | FX-0090-W3-SCOPED-EDITORIAL-HISTORY, BDD-0090-COLLAB, E2E-0090-SCOPE, IT-0084-VERSION | Compare a corrected current note, a withdrawn prior note and erased wording. Change the viewer role and restore. Only eligible versions remain visible, and private reviewer identity or old wording cannot reappear. |
| F-1240 | REQ-0090-MEMBERS, REQ-0090-SCOPE | DES-0090-W3-SCOPED-COLLABORATION-RECORD, VAR-0090-W3-DESTINATION-BOUND-ALIAS-GRANT | FX-0090-W3-EXACT-CONTRIBUTOR-CREDIT, IT-0090-MEMBERS, IT-0084-RIGHTS | Use the exact current approved alias and relation to the piece. Swap destination, withdraw the grant and try a staff or assistant substitute. Hold affected attribution and derivatives before hand-off. |
| F-1241 | REQ-0090-MEMBERS, REQ-0090-SCOPE | DES-0090-W3-ADMITTED-PROTECTED-INTAKE, VAR-0090-W3-ADMITTED-NATIVE-REQUEST-BOUND-TO-RECEIPT | FX-0090-W3-SIGNED-MEMBER-REQUEST-REPLAY, IT-0090-MEMBERS, E2E-0090-SCOPE | Use one admitted signed pull for the exact site, request ID, field schema, digest and consent generation. Exact replay gets the same protected result only under current rights. Reject changed request ID, forged digest, wider fields or withdrawn consent. Missing adapter stays unavailable. |
| F-1242 | REQ-0090-MEMBERS, REQ-0090-MEASURE | DES-0090-W3-ADMITTED-PROTECTED-INTAKE, VAR-0090-W3-NATIVE-PRODUCER-AND-DISCLOSURE-FENCE | FX-0090-W3-PROTECTED-VOTE-RELEASE, IT-0090-MEMBERS, UT-0090-MEASURE | Use a corrected duplicate vote total under its approved request, native producer and release generation. Suppress sparse or reconstructible totals across releases. Withdraw consent and replay. Keep permitted supplied figures labelled separately. No raw ballots or voter identities. |
| F-1243 | REQ-0090-HANDOFF, REQ-0090-SCOPE | DES-0090-W3-EXACT-MANUAL-PACK, VAR-0090-W3-EXACT-CANDIDATE-AND-HUMAN-APPROVAL | FX-0090-W3-TWO-KEY-LOCAL-HAND-OFF, BDD-0090-HANDOFF, IT-0089-REVIEW | Compare an attributable assistant draft with a separate current human decision. Hold assistant-only assent, old revision approval and a concurrent gate or destination change. Only the exact fully approved local pack can reach eligible staff. |
| F-1244 | REQ-0090-CALENDAR, REQ-0090-SCOPE | DES-0090-W3-LOCAL-TIME-OR-HANDOFF-FENCE, VAR-0090-W3-ONE-APPROVED-OCCURRENCE | FX-0090-W3-LOCAL-PUBLICATION-REMINDER, UT-0090-CALENDAR, E2E-0090-SCOPE | Cross both local clock changes and restart at the planned due time. Use one local occurrence and recheck pause, cancellation, expiry and revision. A missed occurrence stays explicit. No send, approval or deferred publish. |
| F-1245 | REQ-0090-SITES, REQ-0090-HANDOFF | DES-0090-W3-EXACT-MANUAL-PACK, VAR-0090-W3-OWNER-PLAN-AND-DISTINCT-SESSIONS | FX-0090-W3-SEPARATE-SITE-DRAFT-PACKS, E2E-0090-SITES, BDD-0090-HANDOFF | Open two owned sites under distinct active sessions. A copied approval or asset grant and swapped canonical target hold affected packs. Owner-only plan metadata provides no cross-site command, merge or write. |
| F-1246 | REQ-0090-HANDOFF, REQ-0090-SCOPE | DES-0090-W3-EXACT-MANUAL-PACK, VAR-0090-W3-EXACT-WORDPRESS-DRAFT-PROFILE | FX-0090-W3-WORDPRESS-LOCAL-DRAFT-FIELDS, BDD-0090-HANDOFF, E2E-0090-SCOPE | Use an eligible WordPress type and current field schema. Try XenForo, a removed field and a native save. Release only eligible local fields or hold. A native draft is a separate manual act. |
| F-1247 | REQ-0090-HANDOFF, REQ-0090-SCOPE | DES-0090-W3-EXACT-MANUAL-PACK, VAR-0090-W3-EXACT-NODE-PREFIX-TAG-AND-POLL | FX-0090-W3-XENFORO-LOCAL-THREAD-PACK, BDD-0090-HANDOFF, E2E-0090-SCOPE | Use an eligible XenForo node and exact title/body, prefix, tags and optional poll. Withdraw one capability or swap WordPress. Hold rather than discard an unapproved field. No thread, ballot or attachment. |
| F-1248 | REQ-0090-HANDOFF, REQ-0090-SCOPE | DES-0090-W3-EXACT-MANUAL-PACK, VAR-0090-W3-TYPED-FIELDS-AND-CLEARED-ASSET | FX-0090-W3-WORDPRESS-POST-PACK-RIGHTS, BDD-0090-HANDOFF, BDD-0087-RIGHTS | Use current allowed taxonomy and an exact featured-image grant with its accessible alternative. Remove a term, revoke the image or swap post type. Release only the eligible reviewed local pack or hold. No post, media record or upload. |
| F-1249 | REQ-0090-HANDOFF, REQ-0090-SCOPE | DES-0090-W3-EXACT-MANUAL-PACK, VAR-0090-W3-CURRENT-NATIVE-BASE-AND-EXACT-LOCAL-DIFF | FX-0090-W3-REVIEWED-MANUAL-DIFF-CONFLICT, BDD-0090-HANDOFF, E2E-0090-SCOPE | Bind additions and removals to one exact candidate and current native base. An intervening base edit requires fresh review. Even an exact approved diff cannot execute through the tool. |
| F-1250 | REQ-0090-RECOVERY, REQ-0090-SCOPE | DES-0090-W3-EXACT-MANUAL-PACK, VAR-0090-W3-SEPARATE-RECOVERY-AUTHORITY | FX-0090-W3-EXACT-MANUAL-RECOVERY-PROPOSAL, IT-0090-RECOVERY, E2E-0090-SCOPE | Use a proved current canonical revision and eligible earlier digest with separate current authority. With a separately approved unpublish plan, release only that local proposal. Keep the same exact site, actor, canonical target and current native revision and supply an eligible earlier digest. Present the unpublish approval for an earlier-version restore proposal. Hold restore instructions pending their own exact approval. Separately swap the prior revision, insert an intervening edit or use old publication approval. No native unpublish, restore or undo. |
| F-1251 | REQ-0090-CALENDAR, REQ-0090-HANDOFF | DES-0090-W3-LOCAL-TIME-OR-HANDOFF-FENCE, VAR-0090-W3-AUTHORITATIVE-INSTANT-AND-CURRENT-RIGHTS | FX-0090-W3-EMBARGO-COPY-TIME-FENCE, UT-0090-CALENDAR, BDD-0090-HANDOFF | Compare before, exactly at and after the proved embargo instant under the admitted time profile. Before, uncertain time and rollback hold. Time readiness is only one condition; rights loss at actual copy holds even after the instant. Nothing dispatches. |
| F-1252 | REQ-0090-HANDOFF, REQ-0090-SCOPE | DES-0090-W3-EXACT-MANUAL-PACK, VAR-0090-W3-EXACT-LOCAL-PLACEMENT-WORDING | FX-0090-W3-ELIGIBLE-PLACEMENT-REQUEST, BDD-0090-HANDOFF, E2E-0090-SCOPE | Use one eligible destination, placement and dates, with applicable review. Change native capability or try unapproved placement and MCP pin. Hold invalid copy; no pin, feature, posting or communication. |

Key follow-ups by site, canonical target, matched publication and
candidate revision, and owning route. Recheck exact visibility,
approval, holds, expiry and rights at queue release and actual copy.
Use the existing link owner for later link and ranking work. These
requests queue no ranking, reindexing, fetch or link update.

QR-193 refines QR-079 for these exact forms. Exercise current signed
authority, consent generations, independent site scope, live-base and
clock races, lost receipts, replay, rights loss, erasure and restore.
Local proposals, hand-off and reminders add no outbound route or
public-page work. Native member intake remains unavailable until its
own contract and measured footprint are admitted.

## Data, settings and limits

Use site-scoped master, variant, assignment, review and release
records under spec 0108. Bind audit provenance to versions and
actors. Register derivatives, private writer and consent records,
queues, caches and managed copies under spec 0032. Revoke access
and recheck replay or restore. Pack expiry does not set download
link expiry or recall existing copies. Copies already taken
outside the app cannot be recalled. Use specs 0078 and 0100 for
limits. Add zero public-page queries, time or bytes. The settings
below are proposed.

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `variants.per_master_limit` | proposals per master | 10 | 1 to 50 | next create |
| `editorial.assignee_limit` | active assignments per site | 100 | 1 to 1,000 | next assign |
| `editorial.pack_days` | days after approval | 30 | 1 to 180 | next retention pass |

## Failure codes

DR-098 reserves SUT-3400 to SUT-3499. SUT-3400 means a version or
approval changed. SUT-3401 means a conflict needs review.
SUT-3402 means rights or permission blocked hand-off. SUT-3403
means the form or route is unavailable. SUT-3404 means a
limit was reached.

## Security and privacy

Use current site and role grants under specs 0020 and 0101.
Treat evidence as data. No cross-site grant transfer, public
action, recipient send, automatic activation or link ranking
input comes from this toolkit.

## Requirements

| ID | Requirement | Test design and expected result |
|---|---|---|
| REQ-0090-VARIANT | Variants MUST preserve evidence and disclose changed form and audience. | UT-0090-VARIANT: compare short, easy-read and unsupported regional drafts and keep missing evidence visible. |
| REQ-0090-LINEAGE | Changes MUST create new proposals without inherited approval. | IT-0090-LINEAGE: edit a master twice, replay one event and hold the affected variant for human review once. |
| REQ-0090-MEASURE | Comparisons MUST use comparable protected evidence. | UT-0090-MEASURE: refuse mixed windows and sparse cells; label an observed difference without causation. |
| REQ-0090-TEST | Test variants MUST NOT assign or activate an experiment. | IT-0090-TEST: prepare eligible variants and observe zero assignments, activation or public change. |
| REQ-0090-COLLAB | Work MUST enforce role, revision and isolated scopes. | BDD-0090-COLLAB: given two proposed edits to one section, show conflict and require reviewed merge. |
| REQ-0090-MEMBERS | Member inputs MUST use admitted site permissions, consent and protected counts. | IT-0090-MEMBERS: test signed site-bound intake, duplicate votes, wrong platform, expired consent, small cells and unavailable route. |
| REQ-0090-CALENDAR | Due or embargo times MUST affect local review only. | UT-0090-CALENDAR: cross due and embargo edges, both clock changes, missed slot and retry. Recheck pack expiry and observe no site write. |
| REQ-0090-HANDOFF | Copy MUST require exact current human approval and MUST NOT publish or upload. | BDD-0090-HANDOFF: given stale gate, target, grant or expiry, refuse clipboard, file and queued asset output without a partial pack. Pair both platforms. |
| REQ-0090-SITES | Cross-site plans MUST keep evidence, sessions and approval separate. | E2E-0090-SITES: use two sites and deny copying a source, rights record or approval between them. |
| REQ-0090-FOLLOWUP | Confirmed publication MUST offer local review requests only. | IT-0090-FOLLOWUP: test claimed and observed states. Replay one confirmed result and revoke visibility before follow-up; create no link or send. |
| REQ-0090-RECOVERY | A local hold MUST stop hand-off and MUST NOT change live content. | IT-0090-RECOVERY: pause during queued copy and exercise both effect orders and a lost reply for clipboard and file separately under FX-0090-W3-OWNER-PAUSE-RELEASE-RACE. Recheck scope on result recovery. Observe an intervening edit and refuse any automatic rollback. Run the separate exact and wrong-kind local approval cases in FX-0090-W3-EXACT-MANUAL-RECOVERY-PROPOSAL. |
| REQ-0090-SCOPE | Actions MUST enforce site, version and limits and MUST NOT execute publication. | E2E-0090-SCOPE: test deferrals, rights withdrawal, erase and restore, limits and direct or queued site actions. |

## How it is tested

Designed, not run. Spec 0076 and QR-079 require per-feature evidence,
format, signed intake, staff capability, replay and denial fixtures.

In IT-0090-RECOVERY exercise an approved clipboard copy and an
approved file release in separate cases. Hold after staging,
then commit owner pause before release. Require zero effects
and no success receipt through restart or old pack restore.
Reverse the order and lose the successful reply after the exact
effect receipt commits. Commit pause, restart, retry and restore.
The count stays at one. With current read scope show only that
existing receipt. Revoke read scope or erase its evidence before
recovery in separate cases. Withhold the detail and repeat no
effect. In another run withhold reliable completion evidence.
Report the original outcome as uncertain while later hand-offs
stay paused. Exact reconciliation can settle only that original
effect. A manual recovery plan keeps its separate approval.

## Changes from the source

Publication, scheduled release, rollback and uploads in the source
become planning and manual hand-off under the toolkit boundary.
Regional versions and terms are after version 1.

## Named local hand-off and guardrail cases

The named Wave 3 cases use the existing interface, applicability,
authority and recovery rules. Each case has the exact input and
observable outcome in its Wave 3 row.

| Feature | Named synthetic case |
|---|---|
| F-1253 | FX-0090-W3-FORUM-BLOG-SEPARATE-PACKS |
| F-1254 | FX-0090-W3-CHECKLIST-EXACT-EVIDENCE |
| F-1255 | FX-0090-W3-LINK-FOLLOWUP-MATCHED-REVISION |
| F-1256 | FX-0090-W3-SOCIAL-FOLLOWUP-LOCAL-PROJECTION |
| F-1257 | FX-0090-W3-NEWSLETTER-FOLLOWUP-PURPOSE |
| F-1258 | FX-0090-W3-SERIES-EXACT-LIVE-BASE |
| F-1259 | FX-0090-W3-PUBLISHING-RIGHTS-EXACT-ACTOR |
| F-1260 | FX-0090-W3-PUBLICATION-LOG-SOURCE-TIME |
| F-1261 | FX-0090-W3-OWNER-PAUSE-RELEASE-RACE |
| F-1262 | FX-0090-W3-MANIFEST-EXACT-ASSET-BYTES |
