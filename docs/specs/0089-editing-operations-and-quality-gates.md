# 0089: Editing operations and quality gates

Edit a draft as a proposal, then check the exact revision before
it can go for human approval. Spec 0084 owns sources and rights.
Spec 0042 owns its own Coach advice. Advisory checks from that
spec cannot replace a required toolkit gate. Reuse only spec
0052's evidence retrieval, draft submission and base checks.
Its publishing and advisory novelty exceptions do not apply.
Spec 0042 scores or switches cannot bypass a toolkit gate.

Fictional example. An editor trims a sourced quote and refreshes a
price. The new quote needs permission for the exact use. The price
must match the current source. The editor sees both changes and
the reasons a draft cannot yet go for approval.

## Words used here

- **Candidate** means one immutable revision proposed for checking.
- **Gate** means a required check before approval submission.
- **Diff** means the differences between versions.
- **Check record** means the evidence and outcome for one gate.
- **Applicability** means whether a gate is required for this form.

## What this spec designs

| Feature | What staff reviews | Requirement |
|---|---|---|
| 1118 Shorten to target length | Propose a shorter revision within a requested maximum | REQ-0089-EDIT |
| 1119 Expand thin sections | Sourced expansion of named thin sections | REQ-0089-EDIT |
| 1120 Simplify jargon | Plain wording with eligible glossary links | REQ-0089-EDIT |
| 1121 Restructure into steps | Ordered step or list proposal preserving meaning | REQ-0089-EDIT |
| 1122 Add examples | Examples marked observed or illustrative | REQ-0089-EDIT |
| 1123 Add analogies | Analogies with limits | REQ-0089-EDIT |
| 1124 Shift tone | Tone revision under an approved brand profile | REQ-0089-EDIT |
| 1125 Remove repetition | Suggested removal of duplicate wording | REQ-0089-EDIT |
| 1126 Tighten introductions | Tighter introduction retaining purpose and facts | REQ-0089-EDIT |
| 1127 Clear next steps | Next steps scoped to real actions | REQ-0089-EDIT |
| 1128 Add transitions | Transitions with no new factual promises | REQ-0089-EDIT |
| 1129 Convert to Q&A | Question-and-answer structure with source references | REQ-0089-EDIT |
| 1130 Convert to a table | Table preserving fields, units and context | REQ-0089-EDIT |
| 1131 Merge two drafts | Merged proposal with both source histories | REQ-0089-STRUCTURE |
| 1132 Split one draft | Separate drafts with declared evidence and dependencies | REQ-0089-STRUCTURE |
| 1133 Refresh outdated sections | Changed selections only, with freshness evidence | REQ-0089-STRUCTURE |
| 1134 Source unsupported claims | Locate eligible evidence for a claimed fact | REQ-0089-FACTS |
| 1135 Remove unsupportable claims | Show unsupported claim removal as a diff | REQ-0089-FACTS |
| 1136 Balance pass | Show supported alternatives and known gaps | REQ-0089-FACTS |
| 1137 Inclusive article language | Context-sensitive inclusive wording suggestions | REQ-0089-EDIT |
| 1138 Spoilers and content warnings | Supported spoilers and warnings as proposals | REQ-0089-FORMAT |
| 1139 BBCode clean-up | Admitted BBCode cleanup in a draft copy | REQ-0089-FORMAT |
| 1140 WordPress block clean-up | WordPress-only block cleanup in a draft copy | REQ-0089-FORMAT |
| 1141 Pull quotes | Exact rights-checked quotation | REQ-0089-RIGHTS |
| 1142 Embedded media captions | Sourced caption with media rights and alternatives | REQ-0089-FORMAT |
| 1143 Article tables of contents | WordPress-only table of contents draft | REQ-0089-FORMAT |
| 1144 Key takeaways boxes | Sourced takeaway box | REQ-0089-EDIT |
| 1145 Tip and warning callouts | Tip or warning cue with its evidence | REQ-0089-EDIT |
| 1146 Footnote formatting | Citation-linked footnote proposal | REQ-0089-FORMAT |
| 1147 Consistent numbers and units | Checked number and unit consistency | REQ-0089-FACTS |
| 1148 Consistent dates and times | Typed dates, time zones and precision | REQ-0089-FACTS |
| 1149 Consistent names | Verified name or alias with scope | REQ-0089-FACTS |
| 1150 Trim quotes faithfully | Exact quoted spans and visible omission marks | REQ-0089-RIGHTS |
| 1151 Line editing | Line-by-line diff and reasons | REQ-0089-EDIT |
| 1152 Second-assistant copy-edit | Independent copy-edit against a pinned draft | REQ-0089-REVIEW |
| 1153 Citation coverage | Every factual claim has an eligible citation | REQ-0089-FACTS |
| 1154 Source freshness | Each cited source passes its age and revision rules | REQ-0089-FACTS |
| 1155 Unsupported claims | Unsupported claims hold approval | REQ-0089-FACTS |
| 1156 Recalculated numbers | Recomputed quantities match admitted source data | REQ-0089-FACTS |
| 1157 Exact quotes | Quotes match permitted exact text | REQ-0089-RIGHTS |
| 1158 Expert review for sensitive topics | Qualified review for sensitive claims | REQ-0089-RISK |
| 1159 Dangerous instructions | Warnings and qualified review for dangerous material | REQ-0089-RISK |
| 1160 Defamation risk | Qualified review of risky named-party claims | REQ-0089-RISK |
| 1161 Copyright | Rights and permitted quotation length | REQ-0089-RIGHTS |
| 1162 Image rights | Current rights for every proposed image | REQ-0089-RIGHTS |
| 1163 Member privacy | Exact consent for permitted personal evidence | REQ-0089-RIGHTS |
| 1164 Sponsor disclosure | Visible applicable sponsorship and affiliate labels | REQ-0089-RISK |
| 1165 AI-assistance labels | Required AI assistance label | REQ-0089-RISK |
| 1166 Scaled content | No bulk near-copy candidate can pass | REQ-0089-NOVELTY |
| 1167 Added value | Documented new value | REQ-0089-NOVELTY |
| 1168 Search intent match | Evidence-backed intent | REQ-0089-NOVELTY |
| 1169 Readability threshold | Admitted audience readability floor | REQ-0089-STYLE |
| 1170 Accessibility | Accessible visuals, headings and alternatives | REQ-0089-FORMAT |
| 1171 No broken links | No known broken or unchecked link | REQ-0089-LINKS |
| 1172 No links to hidden pages | No hidden, noindexed or redirected internal target | REQ-0089-LINKS |
| 1173 Brand voice threshold | Admitted brand voice floor | REQ-0089-STYLE |
| 1174 House style compliance | Current house style | REQ-0089-STYLE |
| 1175 Spoilers and warnings in place | Supported warnings and spoilers in their intended place | REQ-0089-FORMAT |
| 1176 Regional legal rules | Qualified review for applicable regional rules | REQ-0089-RISK |
| 1177 Age-appropriateness check | Admitted age suitability review | REQ-0089-RISK |
| 1178 Topic not already covered | New draft has distinct coverage or is an exact update | REQ-0089-NOVELTY |
| 1179 Minimum substance | Admitted minimum substance | REQ-0089-NOVELTY |
| 1180 No keyword stuffing | Keyword use supports meaning without stuffing | REQ-0089-STYLE |
| 1181 Rich result preview | Inert rich-result eligibility preview | REQ-0089-FORMAT |
| 1182 Translation quality | After version 1 under DR-081 | REQ-0089-SCOPE |
| 1183 Plain-English score | Admitted plain-English floor for help pages | REQ-0089-STYLE |
| 1184 No policy contradictions | No known contradiction with the current site policy | REQ-0089-RISK |
| 1185 Second-assistant check | Independent review required before submission | REQ-0089-REVIEW |
| 1186 Human sign-off | Named human checklist required before submission | REQ-0089-REVIEW |
| 1187 After publishing | Read-only comparison after staff publish | REQ-0089-VERIFY |

## Editing proposals

1. Bind each operation to the current site, actor, form, source
   rights, target scope and selected revision. Admit its structured
   inputs and capability under spec 0020. Preserve original
   versions. Show a diff with changed claims, sources, dates,
   units, names, warnings and rights-sensitive excerpts.
2. Length, style and structure changes preserve meaning and scope.
   Examples say whether they are illustrative. An expansion adds
   only supported details. Splits and merges preserve the inputs,
   resulting versions and citation lineage. Refresh only selected
   outdated sections. Check actions against live grants and policy.
3. BBCode and WordPress block changes are draft-only. Use admitted
   platform serialisers. Features 1140 and 1143 need WordPress.
   Preserve required tokens, links, accessibility, source anchors
   and visible meaning. Refuse unsupported markup or capability.
   A spoiler hides presentation only. Staff-only data stays
   separately authorised and cannot enter a public draft.

### Editing operations (1118–1146)

Apply the editing proposal flow to these forms. Spec 0089 owns the
candidate and required checks. Specs 0020 and 0052 supply admission,
eligible retrieval and draft checks. Use spec 0084 for evidence and
exact-use rights. Use the spelling and writing suite in specs 0121
and 0122 only through its admitted language and range profile. Its
suggestions go through the same proposal and review. They cannot
alter quoted text, terms or required checks by implication. DR-113
allows admitted Writing Suite input. This slice grants no translation
or site-language expansion. Spec 0124 and DR-114 retain their separate
local selected-text routes.

Pin the active site, actor, action, audience, purpose, target, exact
base revisions and selected spans. Record immutable candidate and
evidence digests, source and rights generations, capability, policy,
form, method and renderer versions where used. Bind spans to the
canonical source and preserve their original coordinates. Treat
imported text, markup, evidence and model output as inert data.
Send only approved evidence through the existing local or separately
authorised route. A suggestion cannot expand retrieval or disclosure.

Project both revisions through the recipient's current intersected
readable field scope before forming a diff or table. Counts, omissions
and totals must not reveal a filtered span. Where a table uses 0034
or 0094 data, bind the same release IDs to all cells, alternative
text and output. Apply disclosure history to subtotals, differencing
and inferred suppressed rows.

Show a scoped diff between the pinned original and exact candidate.
Identify changed claims, qualifiers, units, dates, citations, terms,
warnings, links and rights. An ineligible span does not appear in
a diff, excerpt, prompt, preview, export or log. Use a safe omission
reason without exposing the span. Show unsupported claims as held.
Only an eligible editor can approve the exact candidate after the
current required checks. An edit or change in rights, policy, source
or destination invalidates relevant checks and any affected approval.
Recheck current access and hashes before save, preview, submission,
approval, copy or file hand-off, and again on queued work, replay and
restore. Erasure or withdrawal under specs 0032 and 0118 invalidates
affected derived work and replays. The original stays unchanged during
editing and follows the same retention, access and erasure rules.

Limit input, claims, proposed operations, render and export using
the existing 0078, 0089 and 0100 budgets.
Bind the request identity to site, actor, action, exact request and
input digests, current grant and source generations. Refuse reuse for
a different input. On retry recheck rights before showing a prior
result and reproject disclosure. Erased or withdrawn input cannot
restore an old receipt. Reconcile uncertain saves under that identity.
Compare current parent revisions and store the proposals, receipt
and outbox atomically. External file parts stay hidden until the
full set can be released under 0032 and 0118, or are removed.
Show an unavailable reason on a failed qualification, and hold the result
when inputs or checks are incomplete. The only effects are local
proposal, check and audit records and approved manual hand-off.
No native draft, site write, live link change, activation or publication
is created. Add zero public-page queries, time and bytes.

#### Wording and layout

- **1118, shorten to target length.** Bind one admitted counting
  profile and the requested ceiling. Show original and proposed counts
  under the same profile. Keep evidence, material exceptions, quotes,
  accessibility and required disclosures. Hold an impossible target.
- **1119, expand thin sections.** Pin the selected sections and their
  evidence gaps. Add cited details only where the evidence supports
  them. Hold unsupported additions and keep other sections intact.
- **1120, simplify jargon.** Use a current approved same-site glossary
  entry for the exact meaning. Keep required terms with a plain
  explanation. Propose an eligible link and hold ambiguous meanings
  or unproved link targets. No glossary write follows.
- **1121, restructure into steps.** Preserve prerequisites,
  dependencies, alternatives, stop conditions and non-sequential
  choices. Apply the admitted order only to steps with proof.
  A list cannot turn alternatives into a required sequence.
- **1122, add examples.** Label an example as a cited observation or
  a clearly fictional illustration. Bind an observed example to its
  rights and evidence. A fictional example cannot imply that a real
  person acted, a result was measured or a claim has been proved.
- **1123, add analogies.** State the useful correspondence and where
  it ends. Keep the facts and caveats in the original explanation.
  Hold an analogy that would change the meaning or infer a guarantee.
- **1124, shift tone.** Bind the approved audience and exact brand
  profile under spec 0097. Propose changes to voice while preserving
  claims, protected phrases, quote spans and required disclosures.
  A brand switch requires a new proposal.
- **1125, remove repetition.** Identify exact repeated spans and the
  reason for each removal. Keep each distinct limitation, scoped
  instruction, source or warning. Hold a cut that removes a required
  reminder where a reader would act.
- **1126, tighten introductions.** Edit only the identified opening.
  Keep its purpose, audience, matter covered and material
  qualification. Do not add a stronger claim than the body supports.
- **1127, clear next steps.** Use current, verified actions for the
  intended audience, with prerequisites and any eligible link
  under the existing link check. Hold an unavailable route.
  The draft cannot run an action or imply it succeeded.
- **1128, add transitions.** Bind the adjacent sections and their
  established relationship. Use a neutral bridge when cause, order
  or agreement is unknown. Hold unsupported facts and promises.
- **1129, convert to Q&A.** Pair each question with its scoped answer
  and evidence. Mark an unresolved question as such. Preserve
  qualifications and check questions for unsupported premises.
- **1130, convert to a table.** Preserve labels, headers, units,
  cohorts, dates, source anchors and missing values. Check reading
  order and a text equivalent where required. Hold a format that
  would present unlike values as equivalent or replace unknown
  values with zero. A table cannot expose a suppressed value.
- **1137, inclusive article language.** Use admitted audience,
  locale and terminology evidence. Preserve self-identification,
  names and exact quotations. Offer an explained local suggestion
  for uncertain wording. Do not infer an identity or mandate a
  replacement when the context does not support one.
- **1144, key takeaways box.** Tie each point to the exact body claim
  and source. Carry relevant qualifiers and warnings into the box.
  A summary cannot add a claim or strengthen it.
- **1145, tip and warning callouts.** Classify optional advice and
  required warnings with current policy, evidence and audience.
  Keep the warning before the affected action and preserve severity
  and an accessible label. Hold unsafe or unsupported wording for
  qualified review. Formatting cannot clear a risk gate.

#### Revisions and evidence

- **1131, merge two drafts.** Pin two distinct, eligible current
  drafts in the active site. Preserve both originals, source
  lineage, each claim and material context. Intersect per-span use,
  audience, purpose, channel, destination and credit grants. Hold
  incompatible grants or contradictions. A changed original holds the whole
  merge. Create one
  local candidate with both parents and one receipt.
- **1132, split one draft.** Pin one current draft and the exact
  split boundary in source and rendered form. Propose exactly two
  children with the same original parent. Each must retain its applicable
  context,
  rights, claims and sources. Cite dependencies that the intended
  audience can use. Release both local proposals together or
  neither. Keep the original intact.
- **1133, refresh outdated sections.** Pin selected outdated
  span digests, the reason each is stale and exact eligible stored
  evidence revisions under spec 0084 for replacement. Change only
  selected spans. If a connected claim outside the selection
  would become misleading, hold and ask the editor to revise
  the selection. Do not silently update a second span.
- **1134, source unsupported claims.** Pin each claim as written and
  use only already-authorised stored evidence under spec 0084.
  These actions start no look-up, provider spend, crawl or new
  source access. Keep unproved claims and refresh gaps held and
  describe their unknowns. Check population, method, scope, date and
  rights before adding an exact citation. Distinguish direct support from a
  related
  reference or conflicting evidence. Leave an unsupported claim held.
- **1135, remove unsupportable claims.** Show the proposed removal,
  its evidence gap and any dependent statements. A withheld original
  appears only as a safe omission. Preserve surrounding meaning and
  run checks again. A weaker paraphrase cannot carry the same
  unsupported assertion forward.
- **1136, balance pass.** Show materially relevant, comparable
  supported views alongside what each source can establish.
  Preserve known uncertainty, strength of evidence and limits.
  Treat a missing position as a gap. Do not fabricate opposition
  or give weak evidence equal weight by default.

#### Markup, rights and references

- **1138, spoilers and content warnings.** Pin exact eligible spans,
  audience and admitted platform presentation. Check visible warning,
  keyboard access and text alternatives. Prevent unintended disclosure
  in the review projection, excerpt, alternative text and export.
  Keep staff-only spoiler content out of public payload, search
  snippets, model input and derivative caches, even under a spoiler.
- **1139, BBCode clean-up.** Use only the admitted inert BBCode
  parser and serialiser for the exact versioned tag and attribute
  grammar. Refuse custom callbacks, external references and active
  renderers. Check the approved
  semantic structure and accessible text after the final serialisation.
  Hold unknown or ambiguous nodes rather than dropping their content.
- **1140, WordPress block clean-up.** Require an admitted WordPress
  block profile and scope. Preserve nested order, IDs, anchors,
  eligible attributes, typed core block and inner block lineage.
  Check real emitted post content in an inert parsing fixture for
  semantic and literal-markup parity. Hold raw, dynamic or disallowed
  blocks, other hosts and unqualified serialisers.
- **1141, pull quotes.** Select a verbatim eligible span from the
  exact revision. Check scope, short-quote ceiling, exact-use grant,
  context, permitted credit and visible attribution for the new
  placement. Hold if presentation would change the meaning. Any
  separately proposed trimmed quote follows feature 1150.
- **1142, embedded media captions.** Match the caption and text
  alternative to the exact media and approved evidence. Pin asset
  rights, credit and eligible use under specs 0087 and 0097. Preview
  locally and inertly. Use the exact approved rendered asset, avoid
  guessed identities and hold changed media or missing alternatives.
  Timed tracks are separate under spec 0087.
  A caption cannot authorise an embed or remote load.
- **1143, article table of contents.** Require WordPress and its
  admitted heading and anchor profile. Derive order and hierarchy
  from permitted headings in the current candidate. Check final
  internal targets, duplicate IDs and correct accessible labels.
  Hold ambiguous targets without inventing usable links.
- **1146, footnote formatting.** Bind each marker to the exact
  approved citation and its source span. Resolve repeated references
  and stable ordering under the admitted serialiser. Check marker,
  note and return link in the final rendered structure. Hold missing
  notes, altered source meaning and inaccessible navigation.

#### Wave 3 sourced wording and atomic draft fixtures

Use the existing local 0089 forms, synthetic scoped sources and
current rights. QR-189 and QR-078 retain profile and review proof.
The source remains intact and the exact candidate keeps review,
scope and hand-off checks. No native edit or publication follows.

| Feature | Existing form and requirement | Planned fixture and shared checks |
|---|---|---|
| F-1118 | Shorten to target length · REQ-0089-EDIT | FX-0089-W3-SHORTEN-COUNT-DISCLOSURE-CEILING · UT-0089-EDIT. Pin the admitted count profile and target maximum. At the exact and one-over boundaries use a synthetic Unicode sample and report both input and candidate under one profile. Hold a fitting candidate if it loses a material caveat, quote, accessibility element or required disclosure. Preserve original. |
| F-1119 | Expand thin sections · REQ-0089-EDIT | FX-0089-W3-EXPAND-SELECTED-CURRENT-EVIDENCE · UT-0089-EDIT and IT-0084-VERSION. Pin one selected span and its evidence gap. Offer one exact eligible stored fact and reject a stale fact and a source limited to another section. Keep a scoped sourced diff and all unrelated text intact. Source withdrawal holds the affected wording. No retrieval. |
| F-1120 | Simplify jargon · REQ-0089-EDIT | FX-0089-W3-JARGON-CANONICAL-GLOSSARY-MEANING · UT-0089-EDIT and UT-0089-LINKS. Use one current approved same-site glossary entry for the exact intended meaning. Explain a required term without replacing its meaning. Same spelling with ambiguous meaning or a hidden/moved target holds the affected proposal or link. No glossary change or new source read. |
| F-1121 | Restructure into steps · REQ-0089-EDIT | FX-0089-W3-STEPS-PREREQUISITE-ALTERNATIVES · UT-0089-EDIT. Keep one prerequisite, a safe-stop condition and two independent optional alternatives in the sourced rewrite. Require evidence for any imposed order. Hold a candidate that turns an option into a mandatory sequence, changes a dependency or moves the stop condition. Local wording only. |
| F-1122 | Add examples · REQ-0089-EDIT | FX-0089-W3-EXAMPLES-OBSERVED-FICTIONAL-PROOF · UT-0089-EDIT and BDD-0089-RIGHTS. Compare a permitted cited observation and a clearly labelled fictional example. Revoke the observation grant and withhold it. Reject fictional wording that says a real person participated, an outcome was measured or the example proves a claim. Preserve label, scope and source where eligible. |
| F-1123 | Add analogies · REQ-0089-EDIT | FX-0089-W3-ANALOGY-CORRESPONDENCE-CAUSAL-HOLD · UT-0089-EDIT. Give an analogy that states its limited correspondence and retains the original caveat. Compare wording that silently implies causation or a guarantee. Hold the unsupported inference and keep factual explanation and limits unchanged. |
| F-1124 | Shift tone · REQ-0089-EDIT | FX-0089-W3-TONE-EXACT-BRAND-QUOTE-BOUNDARY · UT-0089-EDIT and BDD-0089-RIGHTS. Bind one approved audience and exact 0097 brand profile. Replace the profile after preview and require a new proposal. Hold tone wording that alters a protected quote, approved phrase, claim or mandatory disclosure. Record the exact eligible version and sourced diff. |
| F-1125 | Remove repetition · REQ-0089-EDIT | FX-0089-W3-REPETITION-KEEP-LOCAL-WARNINGS · UT-0089-EDIT. Pin spans proposed for removal and explain why each is redundant. Use one repeated sentence and similar warnings required in two distinct places, with two different material qualifiers. Remove only the true duplicate. Hold deletion that obscures any limitation, scoped instruction or required warning at the reader action point. |
| F-1126 | Tighten introductions · REQ-0089-EDIT | FX-0089-W3-INTRO-SCOPE-ONLY-NO-PROMISE · UT-0089-EDIT. Pin only the opening span. Propose shorter wording preserving purpose, audience, subject and scope. Hold an edit that drops the material qualification or adds a stronger promise than the evidence supports. Keep all body text and sources unchanged. |
| F-1127 | Clear next steps · REQ-0089-EDIT | FX-0089-W3-NEXT-STEPS-CURRENT-ELIGIBLE-ROUTE · UT-0089-EDIT and UT-0089-LINKS. Use one verified eligible audience action with its prerequisites and, if proved, current link. Change a second route to unavailable and hold it. A planted instruction cannot execute anything. The suggestion never claims the action succeeded. |
| F-1128 | Add transitions · REQ-0089-EDIT | FX-0089-W3-TRANSITION-UNKNOWN-CAUSE-ORDER · UT-0089-EDIT. Bind two exact adjacent sections and only the established relationship. Supply neutral wording when timing, causation or agreement is unknown. Hold a transition that invents a chronology, causal conclusion or promise. The sections and their facts stay intact. |
| F-1129 | Convert to Q&A · REQ-0089-EDIT | FX-0089-W3-QA-SCOPE-UNANSWERED-PREMISE · UT-0089-EDIT. Bind an exact question to its narrowly supported answer and citation. Label a second question unresolved and hold a loaded premise without support. Preserve each material qualification. Do not infer an answer from neighbouring evidence. |
| F-1130 | Convert to a table · REQ-0089-EDIT | FX-0089-W3-TABLE-SUPPRESSED-UNKNOWN-ZERO · UT-0089-EDIT and IT-0089-FORMAT. Use cohorts with distinct units, source anchors and dates. Keep explicit headers, reading order and required text equivalent. Show observed zero separately from unknown. Omit a suppressed value and any revealing complement in the whole result. Hold an output that equates incompatible units or inserts information. |
| F-1131 | Merge two drafts · REQ-0089-STRUCTURE | FX-0089-W3-MERGE-PARENT-GRANT-INTERSECTION · IT-0089-STRUCTURE and IT-0019-OUTBOX-ATOMIC. Merge two distinct eligible same-site parents with current exact revisions. Intersect per-span use, audience, purpose, channel, destination and credit grants. Conflicts, stale parents or incompatible grants hold the whole candidate. Simulate crash at commit and lost reply under the same operation identity. Resolve one local candidate with both parents and one receipt or none. Keep originals and source history. |
| F-1132 | Split one draft · REQ-0089-STRUCTURE | FX-0089-W3-SPLIT-ATOMIC-CHILDREN-CONTEXT · IT-0089-STRUCTURE and IT-0019-OUTBOX-ATOMIC. Pin exact source/render boundary and original revision. Hold a split that separates a rule from a material exception without preserving its applicable context. Each child needs self-contained rights, claims, sources and permitted dependency references. Simulate a crash after staging one child and revoke rights before replay. Expose both approved local children under one receipt or neither. Preserve original. |

**Additional evidence needed.** F-1118 needs the admitted count
profile. F-1130 needs the exact protected output profile and
joint disclosure. F-1131 and F-1132 need current parent and
span grants and atomic candidate, receipt and outbox evidence.

QR-189 qualifies the per-form profiles and acceptance cases below
the owning requirements under QR-078. The Wave 3 rows trace this
slice. They describe test design and do not certify an implementation.

Compare heading anchors and footnote targets in the final serialised
projection, with stable identifiers, reading order and no unresolved
fragment collision. Hold unqualified host or renderer output.

### Exact wording edits and first quality gates (1147–1176)

Use 0020's current site, actor, action and object scope and the
0052 local draft lifecycle. Bind each suggestion to its source
draft revision, selected fields or spans, target, purpose and
evidence. Keep the original. A reviewer sees the exact changes
and their reason. Saving a reviewed suggestion creates a new local
candidate only. No edit to the live site or automatic application
of a patch follows.

The quality candidate binds exact draft and target revisions,
audience, form, locale, destination, ordered evidence and rights
generations, policy, method and current grants. Assess the rendered
projection where the rule depends on appearance or reading order.
Each gate records pass, fail, unavailable or a prescribed reason
for inapplicability under the admitted policy. Missing records,
unknown applicability, changed evidence, expired review, model
errors and timeouts hold the affected candidate. A caller cannot
set a pass or choose the reason for inapplicability.

Apply the current complete manifest in the same fenced submission
as the exact candidate. Recheck the complete applicable manifest, exact
approval and authority
at review, preview, replay and restore. Fence policy, stop generation,
proof and withdrawal together at clipboard or file hand-off. A change to
wording, source,
target, grants, method or audience invalidates affected proof,
even if some displayed text is unchanged. Record only permitted
proof and safe reasons. Use 0032, 0108 and 0118 for protected local
processing, erasure, role checks and hand-off. No failed check
leaks the hidden source, claim or party. Limits follow 0078 and
the existing claim and proposal bounds. Overflow holds the
candidate without checking a misleading truncated sample.

| Feature | Exact editing or gate contract |
| --- | --- |
| 1147 Numbers and units | Bind each quantity, source, dimension, precision and unit to its exact span. Apply only admitted equivalence and rounding rules. Keep original values and uncertainty visible in the diff. A styling change cannot change a threshold, currency, quoted value or incompatible measurement. |
| 1148 Dates and times | Record the original date or instant, zone and precision, with effective, event and observation time kept separate. Use the admitted British English display profile. Apply 0019's time rules. A missing day, unresolved local hour or conflicting dates stays open. No timezone assumption or change of instant. |
| 1149 Names | Match each proposed name to the same verified person, product or variant using approved identity and alias evidence. Use permitted display names and spelling. Do not resolve a shared name, rename a speaker inside a quotation, reveal withheld identity or alter a product identifier. |
| 1150 Quote trimming | Keep an ordered mapping to exact permitted source spans and use visible omission marks. Keep context, polarity, qualifications and credit. Recheck feature 658 and 0084/0098 exact use and expiry, and the admitted short-quote ceiling. If the remaining quotation misleads or the permitted span is lost, hold it. |
| 1151 Line edit | Use exact paragraph and sentence spans and an ordered local diff. Mark changes to claim, units, date, source, warning or uncertainty separately. Reject an out-of-span edit, stale candidate or unsupported new meaning. A clearer sentence cannot repair unresolved evidence by assertion. |
| 1152 Second-assistant copy-edit | Bind an independently admitted assistant to the exact candidate, first producer identity, request, permitted source projection and expiry. Use the protected 0020/0118 route and bind the single 0042/0091 owner stop generation at queue, action start, prompt, result, copy and restore. Recheck consent and grants at those boundaries. Discard a late result from a cancelled generation. Resume permits new work only. A refused or unavailable reviewer holds the requested edit without fallback. Keep a reviewed result as a fresh candidate, with the original intact and affected checks invalidated. This opinion cannot fill 1185 or 1186. |
| 1153 Citation coverage | Map every factual claim, including headings, cards, captions and numeric labels, to a currently eligible exact source version and span or field. Opinion or fiction must be labelled and its factual premises still checked. A bibliography entry alone cannot cover an unmatched claim. |
| 1154 Freshness | Compare source revision, observation time, claim-effective period and expiry against the admitted 0084 profile. Hold when a replacement, lost access, uncertain age or changing fact makes the cited evidence stale. Checking uses current eligible stored evidence and triggers no refresh. |
| 1155 Unsupported claims | Confirm the cited passage supports the exact subject, scope, value, attribution and level of certainty. Contradiction or weaker evidence holds. Read only currently eligible stored evidence and advisory observations where admitted. Keep the independent 0089 result and 0052 policy holds even on favourable 0042 advice. No remote route follows. Repetition, title overlap and fluent wording cannot qualify a fact. |
| 1156 Numbers recalculated | Recompute from the same authorised typed inputs, denominator, aggregation, interval, unit and rounding. Use a bounded admitted calculation with exact or specified precision. Test missing, non-finite, overflow, zero denominator and mismatched grain. Protected rows stay with the source owner. Recalculation cannot recover a suppressed value or run supplied expressions. |
| 1157 Exact quotes | Match each visible quoted span to the source revision, exact text and eligible speaker attribution, allowing only the admitted representation of visible editorial omissions. Hold inserted words, hidden omissions, changed meaning or revoked reuse. |
| 1158 Sensitive topics | Identify claims requiring expert review under the admitted policy. Require a current named human with verified subject and audience scope and, where applicable, jurisdiction. Bind the review to exact wording, evidence, policy, qualification and expiry. A good automated result or earlier review cannot substitute. |
| 1159 Dangerous instructions | Hold prohibited or unsupported material. Otherwise require current qualified review of the exact instructions and visible warnings within the approved conditions and destination. Check positioning, access to warnings, and applicable exclusions. A warning or assent alone cannot clear a prohibited instruction. |
| 1160 Named-party risk | Review every claim that identifies a person or company, including by context. Require scoped current evidence and any prescribed qualified review for disputed, risky or uncertain wording. Record safe reasons and keep allegations and corroboration distinct. A risk result is not a legal verdict. |
| 1161 Copyright | Check source and rights for exact content and cumulative excerpt length per source under the admitted short-quote profile. Count related excerpts together and hold disguised near copies, unresolved use or missing attribution. A citation or licence alone does not bypass the ceiling. |
| 1162 Image rights | Bind every asset and final derivative hash, allowed use, audience, destination, expiry, credit and any required subject permissions under 0087/0097/0098. Apply checks to previews, thumbnails, crops, inline graphics and exports. Hold affected content if a source, credit or grant changes. |
| 1163 Member privacy | Bind current signer and consent generation for the exact purpose, person, data, scope, audience and destination, including feature 658 where applicable. A role or reuse right alone cannot stand for consent. Inspect visible wording and derived files for reidentification. No grant admits 0052-excluded fields, protected source text or disallowed recipients. Revocation and erasure fence previews, models, queues, managed copies and restore. Reject prior consent from older backup. Show that already shared copies are beyond managed recall. |
| 1164 Sponsor disclosure | Apply 0101's applicable verified relationship profile to the exact current rendered draft, including reader notice, affected segments and paid link treatment. Required notices must be visible and supported in the proposed placement variant. Check the exact visible and relation data in clipboard and file projections. Hold where serialisation strips either. An attribute, sponsor approval or hidden label alone cannot pass. |
| 1165 AI-assistance labels | Read 0042's eligible provenance and the current applicable owner/legal disclosure policy under 0101/0091 with the dated primary rule and qualified approval under QR-080. Distinguish internal provenance from a public label. When required, verify the exact visible wording and placement on the candidate and final clipboard/file projection. Hold lost notices and required relation data. A public-label setting cannot turn off a label required by the admitted policy. Unknown required provenance holds. F-1285 remains a reference to this gate. |
| 1166 Scaled content | Compare the candidate and eligible same-site batch peers and current corpus with the admitted near-copy method, coverage and threshold. Changed headings, keywords or template values alone do not make pages distinct. Missing corpus or applicability evidence holds. No live crawl, audience profiling or site update. |
| 1167 Added value | Require an eligible same-site comparison and named material contribution. An update pins its exact target and substantive diff. Do not count fact-free rephrasing, renamed examples or repetition as value. Missing corpus material is unknown. Keep this review distinct from the near-copy gate. |
| 1168 Search intent | Match candidate scope and format to current eligible intent evidence for the exact subject, target and audience under 0084/0092. Mixed, absent or stale evidence remains visible. Do not infer intent from popularity, generated angles or another market. No search, provider refresh or ranking effect. |
| 1169 Readability | Use the admitted audience, text kind, locale and measurement profile with sample requirements, scoring direction and threshold under QR-078. Reuse eligible 0123 measures without turning every writing-suite report into a site gate. Display the measure and coverage; short or unsupported material follows only an explicit prescribed rule. |
| 1170 Accessibility | Inspect actual admitted-rendered heading structure, text alternatives, readable visuals and equivalent data or descriptions where needed. Use 0087 for visual alternatives and protect restricted values in every form. Prove the declared variants or hold the unsupported scope. Passing checks do not certify complete conformance. |
| 1171 Broken links | Require current eligible evidence for each exact destination and applicable fragment under the existing 0044/0047 observation and draft checks. Refuse absent, malformed, proven broken, stale or unchecked evidence with a safe reason. An admitted static, supplied or cached observation can cover only its exact scope. This action starts no fetch. |
| 1172 Hidden internal links | For each internal destination require the exact same-site current eligible canonical page and compatible guest visibility and disclosure. Redirected, hidden and noindexed internal destinations hold, even if they return success. Ambiguous site, version or target evidence holds without disclosing inaccessible objects. |
| 1173 Brand voice | Use an owner-approved site and content-type voice profile under 0097, with admitted version, scoped evidence and threshold. Record proof and approved exceptions. A different site's profile, stale rule or generic scoring cannot pass. Do not change attribution, facts or warnings to meet a style score. |
| 1174 House style | Check exact current rule revision, applicable form, display locale, exemptions and protected spans. Keep citations, quoted text and immutable tokens intact. Report rule-level failures or prescribed inapplicability. A generic English style profile cannot stand in for a missing site rule. |
| 1175 Spoilers and warnings | Verify approved visible warnings, accessible reveal and intended order in the admitted platform variant. Hold where required material is inaccessible, clipped or misplaced. A spoiler provides presentation only; staff-only text and answer keys stay outside the public projection, payload and export. |
| 1176 Regional rules | Use only dated, admitted official rules under 0101/QR-054 for the exact audience, material, jurisdiction and effective scope. Bind a current qualified human review of the exact rendered candidate and rule versions. Unknown applicability or conflicting rules hold pending that review. This adds no new locale, market, legal verdict or service action. |

#### Wording and evidence fixture bindings

QR-078 and QR-190 hold exact profiles, rights and proof.
QR-054 applies when an exact dated rule is relevant. These
synthetic fixtures use the unchanged Wave 3 scenarios and
existing shared tests. They confer no eligibility by themselves.

- `DES-0089-W3-EXACT-EDIT-EVIDENCE` pins exact facts,
  source scope, precision, age, units and visible claim fields.
- `DES-0089-W3-CURRENT-QUOTE-AND-ASSET-RIGHTS` binds
  exact source, credit, use, audience and current derivative.
- `DES-0089-W3-LOCAL-REVISIONED-EDIT` uses the 0052
  local candidate and, where admitted, the exact protected
  reviewer route under 0020/0118 and the 0042/0091 stop.
- `DES-0089-W3-SCOPED-HUMAN-REVIEW` requires exact current
  human qualification, wording and source evidence. It keeps
  prohibited material held and rechecks visible warnings.

| Feature | Requirement | Design and variant | Named fixture and shared tests | Independent fixture check |
| --- | --- | --- | --- | --- |
| F-1148 | REQ-0089-FACTS | DES-0089-W3-EXACT-EDIT-EVIDENCE, VAR-0089-W3-ORIGINAL-TIME-AND-PRECISION | FX-0089-W3-TYPED-DATE-PRECISION, UT-0089-FACTS | Format an exact instant with a proved zone and retain original precision for a month-only source. Compare an unresolved repeated hour and conflicting effective and observed times. Leave open facts unresolved; never change the instant. |
| F-1149 | REQ-0089-FACTS, REQ-0089-RIGHTS | DES-0089-W3-EXACT-EDIT-EVIDENCE, VAR-0089-W3-SAME-IDENTITY-AND-ALIAS | FX-0089-W3-VERIFIED-PERMITTED-NAME, UT-0089-FACTS, BDD-0089-RIGHTS | Use an exact eligible alias and distinguish a similar name or wrong variant. Hold an ambiguous identity or withheld quote credit without changing source wording, guessing identity or exposing a protected label. |
| F-1150 | REQ-0089-RIGHTS | DES-0089-W3-CURRENT-QUOTE-AND-ASSET-RIGHTS, VAR-0089-W3-ORDERED-SPANS-AND-OMISSIONS | FX-0089-W3-FIDELITY-OF-TRIMMED-QUOTE, BDD-0089-RIGHTS, BDD-0098-CONSENT | Map retained spans to the original approved quote in order, with visible omission marks. Removing a qualification or the marks, missing short-quote profile or revoked F-0658 consent holds local copy. |
| F-1151 | REQ-0089-EDIT, REQ-0089-FACTS | DES-0089-W3-LOCAL-REVISIONED-EDIT, VAR-0089-W3-EXACT-BOUND-SPAN-AND-REVISION | FX-0089-W3-LOCAL-SENTENCE-DIFF, UT-0089-EDIT, UT-0089-FACTS | Propose an in-range sentence change on the exact local revision. Flag changed uncertainty and facts. Hold a crossed range or a newer revision race. The original survives. |
| F-1152 | REQ-0089-REVIEW, REQ-0089-SCOPE | DES-0089-W3-LOCAL-REVISIONED-EDIT, VAR-0089-W3-SEPARATE-PRODUCER-AND-GENERATION | FX-0089-W3-INDEPENDENT-COPYEDIT-STOP-FENCE, IT-0089-REVIEW, E2E-0089-SCOPE | Bind an independent admitted reviewer, exact local candidate, first producer, permitted source projection, expiry and current 0042/0091 stop generation. Try the same producer, an unavailable or unapproved transport and timeout. Withdraw consent after queueing and revoke the reviewer grant before prompt in separate runs. Neither sends protected input or takes a fallback route. Stop after an eligible dispatch and deliver its late result after resume and through restore. The cancelled result creates no candidate. A separately submitted fresh request can yield a new local candidate with the original text retained and affected checks invalidated. In another run withdraw scope after preview and refuse both clipboard and any admitted local file at hand-off and stale replay. No copy-edit satisfies 1185 or 1186. |
| F-1153 | REQ-0089-FACTS | DES-0089-W3-EXACT-EDIT-EVIDENCE, VAR-0089-W3-EXACT-ANCHOR-ON-EVERY-SURFACE | FX-0089-W3-PER-CLAIM-CITATION-MAP, UT-0089-FACTS | Cite an eligible body claim and a factual premise inside an opinion. An unbound chart label or bibliography-only match holds the gate. Check each visible factual claim against its own exact anchored evidence. |
| F-1154 | REQ-0089-FACTS | DES-0089-W3-EXACT-EDIT-EVIDENCE, VAR-0089-W3-AGE-EXPIRY-AND-SOURCE-REVISION | FX-0089-W3-OBSERVATION-FRESHNESS-FENCE, UT-0089-FACTS, IT-0084-VERSION | Use an admitted age and expiry profile. A newer file revision cannot renew a stale observation. Distinguish equality at the explicit boundary, unknown event date and supersession. No source refresh. |
| F-1155 | REQ-0089-FACTS | DES-0089-W3-EXACT-EDIT-EVIDENCE, VAR-0089-W3-SCOPE-AND-CERTAINTY-MATCH | FX-0089-W3-EXACT-CLAIM-SUPPORT, UT-0089-FACTS | Pair exact support with a weaker proposition, wrong variant and contradiction. Favourable 0042 advice cannot repair the contradiction or replace an applicable 0052 check. A pass covers only this claim gate. |
| F-1156 | REQ-0089-FACTS, REQ-0089-SCOPE | DES-0089-W3-EXACT-EDIT-EVIDENCE, VAR-0089-W3-COMPATIBLE-INPUTS-AND-ROUNDING | FX-0089-W3-TYPED-RECOMPUTATION-BOUNDARY, UT-0089-FACTS, E2E-0089-SCOPE | Recompute a permitted typed aggregate with its exact grain and denominator. Apply the pinned tie rounding to a halfway value. Reject non-finite inputs, wrong denominator or a suppressed group. No raw input or supplied expression execution. |
| F-1157 | REQ-0089-RIGHTS | DES-0089-W3-CURRENT-QUOTE-AND-ASSET-RIGHTS, VAR-0089-W3-VISIBLE-QUOTE-AND-CONTEXT | FX-0089-W3-EXACT-QUOTATION-AND-SPEAKER, BDD-0089-RIGHTS, IT-0084-RIGHTS | Use an exact eligible source span, punctuation and speaker credit. Undisclosed omissions, changed marks outside the admitted representation or reassigned speaker hold. Withdraw the grant during review and deny copy. |
| F-1158 | REQ-0089-RISK, REQ-0089-REVIEW | DES-0089-W3-SCOPED-HUMAN-REVIEW, VAR-0089-W3-CURRENT-QUALIFICATION-AND-DIGEST | FX-0089-W3-EXACT-HUMAN-SENSITIVE-REVIEW, BDD-0089-RISK, IT-0084-REVIEW | Bind a fictional eligible reviewer to the exact subject, jurisdiction when applicable, evidence and wording. Wrong remit, expired qualification, altered claim or model-attested approval keeps it held. |
| F-1159 | REQ-0089-RISK, REQ-0089-FORMAT | DES-0089-W3-SCOPED-HUMAN-REVIEW, VAR-0089-W3-WARNINGS-ON-EXACT-OUTPUT | FX-0089-W3-VISIBLE-WARNING-AND-PROHIBITION, BDD-0089-RISK, IT-0089-FORMAT | Use only opaque benign synthetic material and bind one required warning, approved conditions and destination to the exact candidate and current qualified human review. Confirm intended order and access in the permitted preview, clipboard and admitted local file separately. Starting from the reviewed preview, clip the warning or remove its accessible equivalent in each final output in separate runs. Move a warning outside its required reading position. Hold each affected hand-off without partial or fallback output. Expire or revoke the reviewer after preview and hold copy, file and stale replay even where the warning survives. A prohibited marker with apparent assent and a visible warning remains held. No operational instruction or site effect. |
| F-1160 | REQ-0089-RISK, REQ-0089-FACTS | DES-0089-W3-SCOPED-HUMAN-REVIEW, VAR-0089-W3-DIRECT-AND-CONTEXTUAL-IDENTIFICATION | FX-0089-W3-NAMED-PARTY-QUALIFIED-SCOPE, BDD-0089-RISK, UT-TG-18 | Use fictional opaque party references. Try direct and contextual identification, unavailable independent evidence and changed text after human review. Keep allegation and corroboration distinct, with no identifying private reason or legal verdict. |
| F-1161 | REQ-0089-RIGHTS, REQ-0089-NOVELTY | DES-0089-W3-CURRENT-QUOTE-AND-ASSET-RIGHTS, VAR-0089-W3-SOURCE-AGGREGATE-CEILING | FX-0089-W3-CUMULATIVE-COPY-RIGHTS, BDD-0089-RIGHTS, UT-0089-NOVELTY | Use invented source spans and an admitted exact-use ceiling. Compare the ceiling and just above across related excerpts. Splitting excerpts, dropping credit, revoking rights or using a near-copy paraphrase cannot gain release. |
| F-1162 | REQ-0089-RIGHTS, REQ-0089-FORMAT | DES-0089-W3-CURRENT-QUOTE-AND-ASSET-RIGHTS, VAR-0089-W3-CURRENT-CROP-CREDIT-AND-SUBJECT-GRANT | FX-0089-W3-EXACT-IMAGE-DERIVATIVE-RIGHTS, BDD-0089-RIGHTS, BDD-0087-RIGHTS, IT-0089-FORMAT | Bind a synthetic crop and derivative hash to current use, audience, destination and credit. Try an ungranted crop, lost credit, withdrawn subject permission and old restored thumbnail. Hold every affected preview and export. |

Use QR-190 with QR-078 to qualify exact gate inputs and evidence,
current reviewer and refusal paths. Preserve separate 1185 independent
check, 1186 human sign-off and 1187 read-only observation after staff
publish. This slice adds no settings, publication, site mutation or
rank signal.

## Gate decisions

1. Admit the full gate manifest for features 1153–1186 and mark
   1182 after version 1. Use the declared form, purpose, locale,
   audience and policy to determine applicability. Each gate
   returns pass, fail, unavailable or a prescribed reason for
   inapplicability. Unknown applicability, missing records and
   timeouts hold. A caller or tool cannot mark a gate inapplicable.
2. Bind every record to candidate, evidence, rights, policy,
   audience, destination, grant, method and result. Check 0052's
   base policy, links and novelty on save and before approval.
   Atomically verify the current complete all-pass record at
   submission and hand-off. Fence queued or replayed decisions.
   An edit, changed source, lost grant or policy change invalidates
   the affected record. Staff can edit or reject a held draft.
3. Bind each claim to a supported passage or field, scope, version
   and source age. Recompute numbers with typed lineage, units and
   rounding. Unsupported, conflicting or stale facts hold approval.
   Rewording cannot verify a disputed fact.
4. Require spec 0084's signed exact-use grant under spec 0098 and
   QR-072 before generation, diff, model, review, export and replay.
   Check exact spans, visible omissions, context and credit.
   Apply the admitted short-quote ceiling even with permission.
   Media rights follow specs 0087 and 0097. Withhold unusable
   excerpts. Register derivatives, jobs and copies under spec 0032.
5. Run the named-party risk check on every claim about an identified
   person or company. Unknown or risky claims hold pending qualified
   review. Review dangerous and sensitive content, including
   warnings and age checks, by exact revision, source, audience,
   policy and jurisdiction. Use spec 0101's rendered disclosures,
   current rules and AI labels. Warnings do not admit prohibited
   or unsupported material.

## Evidence, independent review and live comparison

1. Use admitted novelty and subject tests. Compare only eligible
   current site evidence. A new piece must show distinct value.
   An update pins a target and shows substantive change rather
   than taking a second page for the same topic. Hold near copies,
   insufficient substance and unsupported intent. QR-078 admits
   the method and versioned thresholds.
2. Admit audience readability, brand, house style and keyword
   criteria separately. For a chart, caption or warning use the
   admitted rendered structure. An inert rich-result preview may
   report eligible, ineligible or unavailable with scope and evidence.
   It cannot promise a search appearance.
   Subtitle or draft translation checks remain after version 1.
3. Require a successful link check for each proposed link. Read current
   eligible evidence under specs 0044 and 0047 and refuse broken links and
   redirected, hidden or noindexed internal targets. Unknown evidence holds.
   No new unrestricted fetch or link insertion path is created.
4. A second-assistant copy edit (1152) produces a fresh revision.
   A separate independent gate (1185) checks the pinned revision
   and cannot be filled by its author. Bind reviewer, permissions
   and exact evidence scope. Recheck rights, grant and spec 0020's
   owner off-PC settings on every queued or replayed check. No
   fallback. An unavailable reviewer holds. A named human
   checklist (1186) binds the same candidate before submission.
   Neither review grants approval or can publish.
5. After staff publish manually, bind the exact approved candidate
   and canonical destination to fresh own-site evidence from the
   admitted crawl. Compare the same content fields. Show matched,
   different or unavailable and the observed page version. Stale,
   ambiguous or wrong-site evidence cannot confirm. No new
   fetch loop, site repair or write follows.

### Final gate and live comparison contracts (1177 to 1187)

Use the current gate manifest and its shared decision record. Fix the
candidate, source and rights revisions, site, form, audience, destination,
policy, reviewer or method and result before submission. A new candidate
or changed evidence holds the applicable gate. The gate records only
eligible evidence and safe reasons, including when the complete check
cannot run. Existing error codes and the atomic all-pass check apply.

| Feature | Exact decision and evidence |
|---|---|
| 1177 Age-appropriateness | Bind the intended age bands and purpose from the admitted audience profile, the exact content and the current applicable policy. Review words, media and the visible position of any warning together. Do not estimate a reader's age or use an individual profile. Missing audience or an unresolved suitability judgement holds submission. A warning cannot admit prohibited material. |
| 1178 Topic not already covered | Check current eligible coverage on this site, with known scope and the QR-078 method version. Record the strongest overlapping eligible topics and the remaining distinct contribution. A substantially covered topic holds a new page. An explicitly chosen existing target can pass as an exact, substantive update under the existing novelty contract. Missing coverage cannot prove novelty. Report no hidden titles, excerpts or match counts outside the current grant. |
| 1179 Minimum substance | Use the admitted form and purpose profile to check that each required part has useful distinct content supported where factual. Blank headings, padded prose and repeated claims add no substance. A shorter complete form can only pass against its own admitted profile. Thresholds and treatments of examples come from QR-078. |
| 1180 No keyword stuffing | Check the rendered candidate against the admitted language and form rules, with a reason and location for each finding. Keep titles, headings, links and body in their respective contexts. Consider justified repeated terms and exact quotations without altering them. A required keyword or favourable score cannot overrule a finding. Missing method evidence holds. |
| 1181 Rich result preview | Use the exact local rendition, admitted result type and current rule revision. Compare the relevant structured fields with visible content. Show eligible, ineligible or unavailable for that type, with evidence and a safe reason. A proved ineligible preview can satisfy a required preview check only where the exact form permits. Missing rules, type or evidence are unavailable and hold when required. The manifest owns the result. The preview loads no URLs, validator or active content and makes no visibility guarantee. |
| 1182 Translation quality | DR-081 keeps this site-draft gate after version 1. The manifest marks it deferred. Refuse direct, queued, replayed and restored back-translation requests. DR-113's writing-suite language allowance does not admit a site translation or a substitute approval route. |
| 1183 Plain-English score | Apply the admitted help-page audience and scoring profile to the exact rendition and show the matched threshold, method version and actionable spans. Preserve exact legal terms and necessary definitions. An unavailable or below-floor result holds, even if a different readability score passes. |
| 1184 No policy contradictions | Bind the exact effective rules for the site, form, section and audience. Compare each applicable assertion or instruction to its rule. Hold a contradiction, unresolved conflict, outdated rules or absent required rule evidence. Quote only eligible material and give the reviewer a reference to the applicable rule. Never change the rules as part of checking. |
| 1185 Second-assistant check | Require a separate permitted reviewer of the pinned candidate and evidence. Bind reviewer actor and method lineage, authorised input and result digests. An author cannot reuse an identity through a different client or model alias. Feature 1152 cannot substitute. Log no source text. Recheck grant at submission. Recheck scope and allowed transport before any input and on retry. Keep private evidence within its permitted processing boundary. An unavailable route or reviewer holds. A reviewer can record an independent result and reasons. It cannot edit, approve or publish. |
| 1186 Human sign-off | A named authorised person records each required checklist answer and evidence or prescribed reason for inapplicability against the exact candidate and current gate manifest. Deny tool signatures, incomplete checklists and stale identity, scope or revision. Sign-off permits submission only when the other applicable gates also pass. It carries no publication approval. |
| 1187 After publishing | Bind the staff publication record, approved candidate and exact same-site canonical target to a fresh admitted own-site observation. The observation’s capture time under the admitted clock evidence must prove it was made after the manual publication. An import or signing time cannot substitute. Equal timestamps at the available precision give unavailable when their order is unproved. Compare the exact approved artefact, canonical URL and eligible comparable fields under the qualified profile. Incomplete coverage is unavailable, even when observed common fields agree. Exclude private values and derived differences. Use current admitted snapshots and start no fetch. Record matched, different or unavailable with observed revision, coverage and safe field-level differences. A staff claim, redirect to another target, ambiguous page or stale evidence cannot confirm a match. There is no repair, write or special fetch loop. |

QR-191 refines QR-078 for these exact profiles. Check partial coverage,
wrong site, changed source during submission, revoked grants, reviewer
unavailability and replay. These gates cannot publish, add to ranking or
add public-page queries, time or bytes.

#### Wave 3 eligibility and exact review fixtures

QR-191 and QR-078 retain exact method, evidence and reviewer proof.
F-1182 is deferred. F-1185 holds on an unavailable independent
reviewer or inadmissible transport. F-1187 uses only current
admitted post-publication observations. No substitute source,
connector, ranking signal, fetch or site change follows.

| Feature | Owning form and requirement | Planned fixture and shared checks |
|---|---|---|
| F-1178 | Topic not already covered · REQ-0089-NOVELTY | FX-0089-W3-TOPIC-COVERAGE-EXACT-TARGET-UPDATE · UT-0089-NOVELTY and E2E-0089-SCOPE. Use same-site current eligible coverage and exact QR-078 method on a new near copy and a separately selected exact substantive update. Hold substantially covered new wording and incomplete coverage. Keep hidden match titles, excerpts and counts outside the permitted reasons. No new read or publication. |
| F-1179 | Minimum substance · REQ-0089-NOVELTY | FX-0089-W3-SUBSTANCE-REQUIRED-PART-EVIDENCE · UT-0089-NOVELTY and E2E-0089-SCOPE. Use a short help form with each required part complete against its exact admitted form and purpose profile. Compare repeated padding and blank required headings. Useful eligible facts, not length alone, control the result. A different form or unadmitted floor cannot count. |
| F-1180 | No keyword stuffing · REQ-0089-STYLE | FX-0089-W3-STUFFING-EXACT-RENDERED-CONTEXT · UT-0089-STYLE and E2E-0089-SCOPE. Check one rendered candidate with separate heading, link and body contexts under the admitted language/form rule. Preserve technical repetition and exact quotations where justified. Locate each supported finding and hold a required failed check even with a favourable score. Unknown method evidence holds. |
| F-1181 | Rich result preview · REQ-0089-FORMAT | FX-0089-W3-RICH-PREVIEW-LOCAL-RULE-PARITY · IT-0089-FORMAT and E2E-0089-SCOPE. Reuse the admitted 0044 F-1316 parser rule profile and UT-0044-W4-STRUCTURED-DATA only for local rule semantics. Bind exact saved local rendition, result type, visible content and current rule revision. Check malformed syntax, a recognised vocabulary with a failed eligibility rule, an eligible local result and a declared field contradicted by a completely observed visible counterpart. An unread region cannot prove a mismatch. Keep syntax, vocabulary and eligibility results distinct. A complete proved ineligible result satisfies the preview check only where the exact form permits that outcome. Missing rule, type or visibility evidence remains unavailable and holds a required preview. Treat a validator or remote resource reference as inert data. No active content execution, crawl, validator request or search-appearance claim. |
| F-1182 | Translation quality · REQ-0089-SCOPE | FX-0089-W3-SITE-TRANSLATION-AFTER-V1-REFUSAL · E2E-0089-SCOPE. Mark the manifest deferred under DR-081. Refuse desktop, MCP, direct, queued, replayed and restored back-translation requests. No check, new job, report, export or alternate approval. DR-113 writing-suite language scope cannot supply this site gate. |
| F-1183 | Plain-English score · REQ-0089-STYLE | FX-0089-W3-PLAIN-ENGLISH-EXACT-PROFILE-FLOOR · UT-0089-STYLE and E2E-0089-SCOPE. Use exact help-page audience, profile and rendition. Exercise below, at and above its admitted floor, preserving necessary definitions and exact legal terms. Remove the profile while another readability score passes. Only the admitted method and threshold count. Show scoped actionable spans or hold. |
| F-1184 | No policy contradictions · REQ-0089-RISK | FX-0089-W3-POLICY-EFFECTIVE-RULE-REVISION · BDD-0089-RISK and E2E-0089-SCOPE. Use exact current site/form/section and audience rules. Compare active and retired rules separately. Hold contradiction, unresolved conflict and changed policy during submission. Show safe eligible rule references without private text or rewriting policy. |
| F-1185 | Second-assistant check · REQ-0089-REVIEW | FX-0089-W3-INDEPENDENT-REVIEW-CURRENT-TRANSPORT · IT-0089-REVIEW and E2E-0089-SCOPE. Bind a distinct permitted reviewer actor and method lineage, exact candidate and allowed input/result digests. Refuse the author through another client or alias, self-check or F-1152 copy edit. Revoked off-PC permission stops dispatch or retry. Withdraw reviewer grant before submission and hold. No fallback, source log, edits, approval or publication. |
| F-1186 | Human sign-off · REQ-0089-REVIEW | FX-0089-W3-HUMAN-CHECKLIST-EXACT-MANIFEST · IT-0089-REVIEW and E2E-0089-SCOPE. Require named authorised human sign-off on the exact current candidate/manifest with every required item and evidence or prescribed inapplicability reason. Refuse a tool signature, missing item, unjustified inapplicability and stale revision. Valid sign-off supports submission only with all other applicable gates current. No publication or broader approval. |
| F-1187 | After publishing · REQ-0089-VERIFY | FX-0089-W3-LIVE-COMPARE-EXACT-POST-PUBLISH-SNAPSHOT · BDD-0089-VERIFY and E2E-0089-SCOPE. Use manual publication record and fresh admitted own-site observation after it for exact canonical target and approved artefact. Compare only eligible fields. Use a synthetic manual publication at 12:00:00 UTC. Hold matching complete content captured at 11:59:59 and imported after publication. Hold an equal timestamp with no finer order proof. With the same complete fields and a proved capture at 12:00:01, the other exact checks can return a match. A common-field match with incomplete coverage, different canonical or revoked snapshot stays unavailable. Report safe differences where proved. No connector, refetch, repair or write. |

## Data, settings and limits

Keep site-scoped candidates, evidence references, check results
and approvals under spec 0108. Spec 0032 covers erasure, queued
work, caches and managed copies. Revoked evidence blocks access
and replays. Reuse specs 0078 and 0100 for job and export limits.
Draft checks add zero public-page queries, time or bytes.
These settings are proposed.

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `editing.claim_limit` | claims per candidate | 100 | 1 to 500 | next check |
| `editing.operation_limit` | proposed mutations per task | 20 | 1 to 100 | next task |
| `editing.check_days` | days for retained check records | 90 | 1 to 365 | next retention pass |

## Failure codes

DR-097 reserves SUT-3300 to SUT-3399. SUT-3300 means an applicable
gate failed. SUT-3301 means unavailable or stale evidence.
SUT-3302 means rights or review missing. SUT-3303 means a
revision changed. SUT-3304 means a limit was reached.

## Security and privacy

Check active site and current role under specs 0020 and 0101.
Treat source and model text as data. Keep private source text and
member details out of logs and unapproved prompts or exports.
Tool access cannot approve, apply, publish or affect link ranking.

## Requirements

| ID | Requirement | Test design and expected result |
|---|---|---|
| REQ-0089-EDIT | Edits MUST preserve the original and show a sourced diff. | UT-0089-EDIT: compare shorten, expand, example and tone outputs with unsupported claims held. |
| REQ-0089-STRUCTURE | Merge, split and refresh MUST preserve exact revision and source lineage. | IT-0089-STRUCTURE: merge two versions, split selected text and refresh one section without changing inputs. |
| REQ-0089-FACTS | Claims MUST pass citation, scope, age and recomputation checks. | UT-0089-FACTS: reject stale, missing, disputed or misrounded evidence and show the failed field. |
| REQ-0089-FORMAT | Formatting proposals MUST preserve scope, visibility and accessibility. | IT-0089-FORMAT: check unsupported platform, hidden text, missing alt text and inert structured data. |
| REQ-0089-RIGHTS | Quote, media and personal evidence MUST pass current exact-use rules. | BDD-0089-RIGHTS: given an altered quote and revoked image grant, hold approval and both hand-off routes. |
| REQ-0089-RISK | High-risk claims MUST pass qualified review and applicable policy. | BDD-0089-RISK: given an unreviewed sensitive claim and missing disclosure, show both blockers without publication. |
| REQ-0089-NOVELTY | Value and intent checks MUST use eligible source coverage and admitted rules. | UT-0089-NOVELTY: reject a near copy; compare exact-target substantive edit, unrelated draft and missing evidence. |
| REQ-0089-STYLE | Style checks MUST use current admitted audience and voice rules. | UT-0089-STYLE: try below, at and above an admitted floor and an unavailable measure. |
| REQ-0089-LINKS | Links MUST use current eligible targets. | UT-0089-LINKS: compare broken, stale, noindexed, redirected and allowed links without insertion. |
| REQ-0089-REVIEW | Required independent and human checks MUST bind the same revision. | IT-0089-REVIEW: deny former author, unavailable reviewer and forbidden transport. A copy edit invalidates checks. Compare independent and human records for the exact revision. |
| REQ-0089-VERIFY | Verification MUST compare the approved and observed revisions without writes. | BDD-0089-VERIFY: given a manual publication, show a match or difference within scope. Stale or ambiguous destination is unavailable. Observe no repair. |
| REQ-0089-SCOPE | Every applicable gate MUST pass before submission; the toolkit MUST NOT bypass or publish. | E2E-0089-SCOPE: exercise false inapplicability, missing gate, same-revision source change at submission, wrong site, timeout, replay, bounds, deferral and denied site actions. |

## How it is tested

Designed, not run. Spec 0076 and QR-078 require exact form and gate
evidence, rules, qualification and per-feature failure fixtures.

## Changes from the source

Editing changes remain proposals. Version 1 uses British English.
A live-page check reads only after staff publish through site tools.

## Named editorial review cases

The named Wave 3 cases use the existing interface, applicability,
authority and recovery rules. Each case has the exact input and
observable outcome in its Wave 3 row.

| Feature | Named synthetic case |
|---|---|
| F-1133 | FX-0089-W3-SELECTED-REFRESH-LINEAGE |
| F-1134 | FX-0089-W3-EXACT-STORED-CLAIM-SOURCE |
| F-1135 | FX-0089-W3-REMOVAL-DEPENDENT-CONCLUSION |
| F-1136 | FX-0089-W3-BALANCE-COMPARABLE-COVERAGE |
| F-1137 | FX-0089-W3-INCLUSIVE-LANGUAGE-CONTEXT |
| F-1138 | FX-0089-W3-SPOILER-DISCLOSURE-FENCE |
| F-1139 | FX-0089-W3-BBCODE-INERT-PROFILE |
| F-1140 | FX-0089-W3-WORDPRESS-BLOCK-PARITY |
| F-1141 | FX-0089-W3-PULL-QUOTE-PLACEMENT-RIGHTS |
| F-1142 | FX-0089-W3-MEDIA-CAPTION-EXACT-ASSET |
| F-1143 | FX-0089-W3-TOC-FINAL-ANCHOR-PARITY |
| F-1144 | FX-0089-W3-TAKEAWAYS-BODY-QUALIFIERS |
| F-1145 | FX-0089-W3-CALLOUT-SEVERITY-READING-ORDER |
| F-1146 | FX-0089-W3-FOOTNOTES-FINAL-RETURN-TARGET |
| F-1147 | FX-0089-W3-UNIT-PRECISION-THRESHOLD |

## Named review and local variant cases

The named Wave 3 cases use the existing interface, applicability,
authority and recovery rules. Each case has the exact input and
observable outcome in its Wave 3 row.

| Feature | Named synthetic case |
|---|---|
| F-1163 | FX-0089-W3-MEMBER-CONSENT-GENERATION |
| F-1164 | FX-0089-W3-SPONSOR-FINAL-PROJECTION |
| F-1165 | FX-0089-W3-AI-LABEL-CANONICAL-GATE |
| F-1166 | FX-0089-W3-SCALED-CONTENT-COVERAGE |
| F-1167 | FX-0089-W3-ADDED-VALUE-EXACT-TARGET |
| F-1168 | FX-0089-W3-INTENT-EXACT-AUDIENCE |
| F-1169 | FX-0089-W3-READABILITY-ADMITTED-PROFILE |
| F-1170 | FX-0089-W3-ACCESSIBILITY-RENDERED-PROOF |
| F-1171 | FX-0089-W3-LINK-EXACT-FRAGMENT |
| F-1172 | FX-0089-W3-INTERNAL-CANONICAL-VISIBILITY |
| F-1173 | FX-0089-W3-VOICE-APPROVED-EXCEPTION |
| F-1174 | FX-0089-W3-HOUSE-STYLE-PROTECTED-SPANS |
| F-1175 | FX-0089-W3-WARNING-FINAL-REVEAL |
| F-1176 | FX-0089-W3-REGIONAL-EXACT-REVIEW |
| F-1177 | FX-0089-W3-AGE-SUITABILITY-PROFILE |
