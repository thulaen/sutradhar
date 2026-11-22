# 0041: Editorial Coach

The Editorial Coach answers one question for an editor: what is the
smallest edit worth testing on this page? The editor opens an exact
passage, sees the evidence, compares the wording and can keep the
original. It is advice only. It is part of roadmap Phase 6; "Which spec
designs each feature" in [features.md](features.md) lists the features
it designs in whole or in part. Its later additions are in
[0042](0042-coach-additions.md).

## What it does

### One thin layer

The Coach reads and presents what Sutradhar already holds. It adds no
second drafting route, overlap check, connector or test system. Feature
numbers below are those of the archive's feature register, except
features 36 and 235, which are numbered as in features.md.

| Coach task | Reuses |
| --- | --- |
| Write or update | Content overlap check ([0052](0052-hubs-summaries-drafts.md)); features 16, 36, 91 |
| Where is the answer | Features 7, 12, 17, 27, 246 |
| What needs a refresh | Feature 252; content draft checks |
| Write from our expertise | Content drafting route and evidence pack; feature 245 |
| What to preserve | Content draft checks; draft diff |
| Did the change help | Features 20, 21 |
| Search and analytics facts | Connectors ([0040](0040-search-connectors.md)); feature 483; darshak |

New here: the edit card, the keep-the-original action, the rules below
and the five tools.

### The edit card

1. Each tip shows the exact page version, the passage, the proposed
   wording, live evidence, dates, data freshness, sample size, and the
   explanations it kept or ruled out.
2. It shows the deciding reasons, never one score of its own (feature
   235 in features.md, a separate approved measure, spec 0042).
3. A forecast the data cannot support is hidden. Example: 60 clicks from
   3,000 views, where similar pages get 3%, is a gap of 30 clicks to
   look at, not a promise of 30 more.
4. "Leave it alone" and "not enough evidence" are normal answers. A draft
   can pass with no suggestions.

### Order of checks

1. Permission, freshness, sample size, and whether the action is safe.
2. Confirm demand and fetch the passage.
3. Rank by size of the gap, how directly the text supports it, the
   author's effort and the risk to pages doing well.
4. Only ranked tips reach the editor.

GA4 may rank Coach tips. This holds for the Coach only: link ranking
stays on darshak rates. GA4 figures are never added to other counts.

### What each source can show

- Search Console: demand for a task on this site. It may hide rows, so
  page totals stay apart from known-query totals. Compare like device
  and position.
- GA4: landing-page results. Clicks and sessions count differently.
  Never claim one query or paragraph caused a sign-up.
- darshak: approved cookie-free totals only. A click count without the
  number of views gives no click rate. Missing values stay unknown.
- The content index: exact passages and versions. A passage is fetched
  before it is cited. A word match alone does not prove a paragraph
  confused readers.
- A traffic fall alone never proves a statement false; demand is checked
  first. A date change alone is not freshness.

### What the Coach may change

- Staff guides: a proposed revision delivered to WordPress or XenForo's
  own editor, with a keep-the-original action. Staff publish it. It is
  never applied by itself.
- Titles: a suggestion only, and only where the page text shows a
  mismatch. This is a narrow exception to the SEO rule against title
  proposals ([0044](0044-seo-signals-crawler.md)).
- Member posts: only through the owner's separate member-text option,
  with a moderator approving each change. That option has its own design.
- A later human edit cancels a stale proposal. Undo keeps later edits.
- Findings are advice only. The approved hard checks (such as private
  data or an unsupported quote) stay the only blockers. Automatic checks
  are read-only, and a check still waiting never counts as passed.

### Using a member's answer

The site's users decide, through a bounded setting.

| `coach.member_answer_use` | Effect |
| --- | --- |
| `link_or_short_quote` (default) | A link, or a credited quote up to 50 words |
| `credited_reuse` | Longer reuse, always credited |
| `with_member_consent` | Only with that member's agreement |

Credit and permission always stay visible. One person's report stays
separate from a repeatable result.

### Where wording comes from

Sutradhar computes every number, permission and reference; a model
never supplies them.

1. The local route is an exact-pinned packaged generative-model
   application behind a typed Rust-owned interface admitted under spec
   0024. Sutradhar writes no model runtime, custom model component or
   first-party Python production code. The exact package, generative
   model artefact, dependency hashes, quality evidence and measured
   resource profile are readiness blockers: until all are admitted, the
   local route is unavailable. BGE retrieval models are not generative
   writing models and cannot fill this role.
2. A paid route (AWS Bedrock with the owner's own key) is used only when
   the owner turns it on and sets a spending cap. It is off by default.
3. An AI tool the owner connects through Sutradhar's MCP server may read
   the evidence and draft wording. Every such draft is only a suggestion
   that a moderator must accept, labelled with where it came from.
4. Wording follows the ordered list and shown fallback in
   [0042](0042-coach-additions.md).
5. With no wording route, the Coach still shows evidence and the
   original passage, and invents no wording.

- A staff member's own unpublished draft may go to a route that leaves
  the owner's PC (the paid route or an MCP tool) only with a clear "this
  leaves your computer" notice, shown first every time.
- Other people's drafts, private member text and staff-only text never
  leave the PC. Sending nothing is always possible.
- The content drafting rule that Sutradhar calls no language model is
  loosened for the Coach, in these ways only.

### Paired sites

Sites stay separate, with one exception. For two registered sites with
the same owner, the repeat check may read both, only if the owner turns
it on for that pair. It reports overlap and suggests a link. It never
copies text across and never merges their data. No pair exists until
the owner names one.

### Switching on

Unlike most features, the Coach stays off until it passes a blind test.
Editors compare evidence-backed tips with generic tips on staff pages of
a test site. The test records whether the source supports the edit,
whether it answers the reader's question, how long checking takes, and
how often leaving the text alone was right. Editor acceptance is kept
apart from search results. The owner approves the pass mark before the
test. The blind test also covers the tips added in
[0042](0042-coach-additions.md). After passing, the Coach starts on with
settings whose values and evidence have been admitted; a blocked dependent
check stays off with its reason shown.

### Tools

The desktop app and the MCP server show the same evidence. Each tool is
limited to the authorised site, user, visibility and version, is checked
against the Cedar rules, and wraps an existing feature.

| Tool | Returns | Wraps |
| --- | --- | --- |
| `find_editorial_opportunities` | Shortlist, and items held back | 16, 12, 17 |
| `get_editor_hints` | Passages and tips for one version | 7, 27, 246 |
| `prepare_writing_brief` | Overlap, home, outline, gaps | Content checks, 36, 91 |
| `check_draft` | Read-only warnings and a preview | Content checks, 252 |
| `compare_edit_outcomes` | Plan or results, with uncertainty | 20, 21 |

For feature 36 in features.md, `prepare_writing_brief` needs enough
eligible source posts or pages to support the brief. An eligible source
is distinct, guest-visible and qualified under the
[research rules](0013-research-rules.md). Our own lab runs set the
minimum; until it is recorded, no brief is made. Below it, the tool
reports "not enough evidence" and makes no brief.
VAR-0041-W1-SAME-SITE-BRIEF may use exact retained keyword rows
through the current 0132 selection and brief grants. The rows
are advisory evidence. Count only distinct eligible posts or
pages towards the F-0036 source minimum. A keyword selection
cannot admit a brief with no recorded minimum or too few
qualifying pages. Use the original snapshot and row and retain
their dates and market. Reuse requires no provider request.

Later additions, keeping the same five tools:

- `check_draft` takes pasted and uploaded input and the new checks.
- `prepare_writing_brief` gains related questions.
- `get_editor_hints` shows "tested" lines from the test room
  ([0039](0039-test-room.md)).
- `compare_edit_outcomes` shows chatbot-check results, read only.

### Saved drafts and editorial work (235–246)

The desktop Coach opens a saved revision under the existing draft
or page owner. Native site editors keep their saved-score panels
under 0042. The one-off rival comparison belongs to 0131, and
fan-out to 0117. General Coach checks here run on saved text or
an explicitly submitted snapshot. They add no requests per
keystroke or work to guest page responses.

#### Evidence on a saved revision

| Feature | Source, rule and result |
|---|---|
| 236 Readability | Use spec 0123's R05 and R25 measures on authorised prose. Show the mean words per eligible sentence, passive clauses out of the clauses classified, and any reading-age band from a separately admitted British English conversion. Show exact denominators, measure/profile IDs and excluded sections. Omit a band if its eligible text or conversion is missing. The band describes a text estimate and cannot identify the reader |
| 237 Term coverage | Use the site topic index and admitted retrieval to select comparable guest-visible pages on the same approved topic. From one selected closed Search Console window and compatible projection, use authorised impressions to choose up to five eligible pages in descending order, then stable page ID. Exclude the draft's live counterpart and withdrawn evidence. Use one pinned vocabulary profile. Build the vocabulary independently from the readable peer pages, then report each term as present, absent or unknown in the exact draft and each peer. Keep source ranges and counts. Missing coverage remains unknown |
| 239 Heading outline | Use the authorised saved brief in the shared 0052 route and the normal 0041 brief minimum. Keep ordered H2 and subordinate H3 proposals with exact current source support. Use an independently eligible brief topic for a genuinely unanswered question. Recheck each heading's support when a source or version changes. A proposal changes no draft |
| 240 Brand voice | Resolve the exact approved site guide in 0097 feature 412. Apply its allowed current voice and tone checks, with policy and scope from 0128 where that engine is used. Display each finding beside its source rule, guide revision and exact text range. A missing or revoked guide shows unavailable. Examples act as data and cannot grant authority |
| 246 Rewrite suggestions | Use the existing 0041 wording order on an exact authorised unclear paragraph only. A local writing check can explain the problem from 0122 or 0124. Apply the stricter site route and text egress controls. The review shows source and proposed wording with unchanged names, numbers, quotes and references unless an independently supported edit is specified. A missing or stale proposal leaves the original and evidence visible |

Readability considers the admitted tokenisation, sentence and
clause coverage once per exact revision. Suppressed, unreachable
or uncertain text never counts as covered. Use the same 0123
reports and 0122 ranges when the workspace presents those results.

For term evidence, bind all rows to site, subject, comparable
window, sample definition, index generation and source revision.
The top-five rule only sorts candidates that already pass the
topic and source eligibility gates. Display the selected window
and covered page count before the overlap. Build the term set from
the admitted vocabulary profile and eligible peer ranges. The
admission sets minimum readable peer and term support across pages.
An unreadable range or an unproved page projection cannot
establish absence. Search terms and suppressed query rows cannot
supply the vocabulary. The report establishes no causal claim or
link rank.

For F-0239, retain an exact dependency for each H2 and H3 in the
saved 0052 brief projection. Bind the heading and its parent to
the current brief and draft revisions, approved topic, permitted
source revision and specific supporting ranges. The normal 0041
source minimum is a requirement for the whole outline. Every
supported heading also needs its own current eligible range.

Recheck both requirements before showing an outline or reopening
a saved preview. If a heading loses its only eligible support,
withhold that heading and dependent text, citations and previews.
If its H2 is withheld, hold its subordinate headings in that
outline. Do not silently move them to another heading. Other
supported branches may remain under their own current evidence
if the brief still meets its admitted minimum. Show an unanswered
question only when an independently eligible brief topic supports
that question without carrying forward withdrawn wording or facts.
Otherwise omit the heading. Below the brief minimum withhold the
whole outline. A cached result, replay or restore cannot bypass
these checks. This review changes no draft or publication state.

For all suggestions, validate scope, visibility, rights and exact
source revisions before analysis and before delivery. Challenges,
removed data and paused routes stop queued work. Clear previews
that no longer match. For 0084 F-0756 reuse, check the exact draft,
entitled question, own-site coverage and receiving scope again at
delivery. A still-readable draft gives no proof of the linked
page or the receiving pack's rights. Exercise the joined release
with IT-0041-EVIDENCE and FX-0084-W3-GAP-EXACT-DRAFT.
For rewriting use the evidence-only path
when no wording route is available. Recheck own-draft rights,
visibility, the specific route grant, exact egress consent and
factual ranges before sending. A revoked grant stops retry or
fallback that depends on it. A compare view, moderator review
and native application are distinct steps. Recheck the accepted
revision before any existing authorised native action. No new
wording route or indexing system is provided here.

#### Inventory and editorial views

Use the existing content store and native site records. The
inventory lists every eligible published page in the admitted
site catalogue. Show type, permitted author, verified first
publication, age, one source's traffic and counted links with
coverage. Include admitted unindexed pages and label their missing
link evidence. Show the as-of date and catalogue coverage. Use
whole days since verified original publication on the pinned site
calendar, including its time zone revision. Later edits and later
publications cannot reset the first date. Unauthorised, suppressed
and unknown fields stay unavailable with a safe reason. Filter
eligible fields, rows and totals before delivery. This inventory
adds no page reads or visits. It does not change the separately
approved local export routes in spec 0100.

Editorial stages and calendar dates are metadata on a work item
linked to the existing brief, saved draft or native publication
mapping. There is one accountable item per site and editorial
object. Changing the metadata cannot edit a CMS draft. There is
no second content document or publication queue.

| Record | Contract |
|---|---|
| Editorial work item | Site, item and existing content IDs, title reference, stage brief/draft/review/published, assignee ID or none, due date or none, item revision, expected exact content revision, actor and authority generation. Published also binds native object ID and confirmed publication revision and time |
| Stage change plan | Item ID, expected item and content revisions, current and proposed stage, optional assignee or due date update, actor, scope and exact approved change. Use existing action, audit, idempotency and encrypted ledger contracts |
| Calendar row | Item ID, stage, authorised assignee, date type due or published, date, site time zone, displayed title reference and item revision. Rows never expose unapproved draft text |

The authorised staff member selects a permitted assignee, sets
a due date and moves the item by confirming its exact metadata
change. A brief needs a current authorised brief. Draft needs
a saved revision, and review needs that version in the native
review scope. Allow brief to draft, draft to review and review back to draft.
A stage action carries no publication authority. Only the native
completion reader can record published. It asks the content owner
under 0052 and the paired adapter under 0021 for a committed
event or signed completion read. Require site, logical publication
effect ID, native object ID, exact approved revision, confirmed
published time and monotone event sequence. Refuse a caller's
receipt or an adapter without an admitted exact payload. The event
binds an existing approved publication, never causes one.

For XenForo, accept a completed 0052 result bound to the exact
native thread or resource and confirmed by the paired add-on.
For WordPress, require the paired plugin's signed post ID, exact
revision and committed public state backed by the native event
sequence. A raw status, UI event or public page observation is
insufficient on either platform. Keep each adapter's route
unavailable until QR-127 admits its native contract.

Match each effect once and preserve per-item native order. A reject
or revised draft clears review for that predecessor. An older
completion can update that version's history only when independently
proved. It cannot move a different current revision to published.
A valid published record remains the history of its own revision.
Use the current event and erasure epochs on restore. Unknown or
unadmitted native authority leaves publication unconfirmed.

Assignee changes and date edits need current staff permission on
the item and any target assignee. Assignment alone grants no draft
access. Deny a stale or competing revision atomically. Replay of
the same accepted action changes metadata once. Store only the
protected metadata and exact refs. A removed actor or resource
stops delivery, including schedule and inventory results.

The calendar opens from the same editorial view. Choose one
month in the site's time zone. Provide matching month and list
views with previous and next month controls, stage and eligible
assignee filters, and an “Undated” list. Sort by date, date type
and stable item ID. Read the live title and assignee grants before
filtering, counting or paging, so a hidden value cannot leak by
filter or total. Due dates stay local calendar dates. Apply the
pinned site time zone revision once to the native publication time.
When both dates are present, keep both labelled.
Edits open the exact work-item plan. The calendar creates no
publication job, remote feed, invite or notification.

#### Acceptance contracts

| Requirement | Verification contract |
|---|---|
| REQ-0041-READABILITY | IT-0041-READABILITY: use two synthetic British English sections with different clause counts, protected text and too little evidence for an age band. Match 0123 counts, preserve original ranges, and show incomplete scope without invented age |
| REQ-0041-EVIDENCE | IT-0041-EVIDENCE: compare eligible peer pages with a cross-topic leader, no complete source, an exact tie, a peer-only term, an incomplete or suppressed peer and a withdrawn page. Give an outline with supported and unanswered headings, and apply an approved voice guide then revoke it. Evidence remains scoped by site, source and revision |
| REQ-0041-INVENTORY | IT-0041-INVENTORY: index guest pages with private drafts, admitted unindexed pages, unknown original dates, a republished page, withdrawn display permission, suppressed counts and missing link evidence. Page through a stable snapshot without mixing sources or granting hidden entries |
| REQ-0041-WORK | IT-0041-WORK and BDD-0041-CALENDAR: move a synthetic item through brief, draft and review, send it back, then receive one committed native completion. Try a caller-supplied receipt, missing adapter, wrong native revision, rejection, newer edit, replay and restore. Test due/published labels, undated items, denied assignees before filter/count, calendar/list parity and time zone revision. No action publishes |
| REQ-0041-REWRITE | BDD-0041-REWRITE: offer an exact draft paragraph with factual claims, then change it before review and fail the wording route. Revoke a send grant during an attempt and test the evidence-only route. Keep evidence and original available. No stale proposal, unsupported changed fact, unapproved fallback transfer or write |

QR-127 owns exact admission of the measures, index and metric
evidence, native contracts, privacy, calendar and resource limits.

#### Wave 2 saved-revision fixture bindings

Use the existing checks above with synthetic sites and content.
QR-127 still owns admission of measures, data and native profiles.
A fixture can establish the expected answer for its own synthetic
case only.

| Feature and requirement | Design and fixture | Evidence to require |
|---|---|---|
| F-0236 · REQ-0041-READABILITY | DES-0041-W2-READABILITY-COVERAGE · FX-0041-W2-READABILITY-COVERAGE · IT-0041-READABILITY | Use the same authorised British English ranges under 0123 R05 and R25 for the full text and short section. Check mean words per eligible sentence and passive out of classified clauses with their denominators. Preserve the intentional passive wording and exclude the protected quotation from this fixture's admitted prose. Omit the band with insufficient eligible text or without a separately admitted conversion. |
| F-0237 · REQ-0041-EVIDENCE | DES-0041-W2-OWN-TOPIC-TERM-COVERAGE · FX-0041-W2-OWN-TOPIC-TERM-COVERAGE · IT-0041-EVIDENCE | From one selected closed Search Console projection, rank only eligible comparable same-topic guest-visible pages by admitted impressions, cap at five and break ties by stable page ID. Exclude the live counterpart, withdrawn pages and a cross-topic leader. Use one admitted vocabulary profile and exact readable peer ranges, including a peer-only term. Missing or suppressed evidence stays unknown. Refuse an unsupported window and never use search query terms as the vocabulary. |
| F-0239 · REQ-0041-EVIDENCE | DES-0041-W2-CITED-HEADING-OUTLINE · FX-0041-W2-CITED-HEADING-OUTLINE · IT-0041-EVIDENCE | Use an eligible saved 0052 brief above its admitted minimum. Withdraw the only range supporting one H2 while the remaining brief still qualifies. Withhold its text, citations and H3s and keep a separately supported branch. An independently eligible open question may remain. The withdrawn heading cannot turn into a question by dropping its citation. Replay and restore the same saved preview and try a changed brief or draft revision. Separately fall below the minimum and withhold the entire outline. Preserve the saved draft. |
| F-0240 · REQ-0041-EVIDENCE | DES-0041-W2-APPROVED-VOICE-GUIDE · FX-0041-W2-APPROVED-VOICE-GUIDE · IT-0041-EVIDENCE | Give an exact passage and a current site guide admitted by 0097 feature 412. Identify a supported finding by rule, guide revision and passage range. Replace or revoke the guide and put an instruction in an example. Refuse old evidence and treat examples as data. Where used, stay within the 0128 approved procedure and scope. |
| F-0242 · REQ-0041-INVENTORY | DES-0041-W2-ADMITTED-SITE-INVENTORY · FX-0041-W2-ADMITTED-SITE-INVENTORY · IT-0041-INVENTORY | Page one admitted published catalogue snapshot, including an admitted unindexed page. Keep hidden author, unknown first publication, suppressed metric and missing link evidence unavailable. Preserve a verified first publication when the page is republished. Age uses the pinned site calendar. Filter revoked scope before visible totals. Show one traffic source, date and coverage. No extra page read. |
| F-0243 · REQ-0041-WORK | DES-0041-W2-EXACT-WORK-STAGE · FX-0041-W2-EXACT-WORK-STAGE · IT-0041-WORK | Use exact confirmed metadata changes to assign, draft and review, then try a fabricated caller receipt, stale event, wrong native revision and unsupported adapter. Assignment grants no draft access. Only one admitted 0052 and 0021 committed completion for the exact revision records its publication. Replay changes no metadata again. An independently proved older completion remains that revision's history. No stage action publishes. |
| F-0244 · REQ-0041-WORK | DES-0041-W2-SITE-CALENDAR · FX-0041-W2-SITE-CALENDAR · BDD-0041-CALENDAR | Inspect one month in both calendar and list. Use a due date, a near-midnight native publication, an undated item and a withheld assignee. Apply the pinned time-zone revision to the publication once and preserve the due day as a local date. Match views and labelled dates after visibility filtering. No publication job, invite or external event. |
| F-0246 · REQ-0041-REWRITE | DES-0041-W2-EXACT-PARAGRAPH-ADVICE · FX-0041-W2-EXACT-PARAGRAPH-ADVICE · BDD-0041-REWRITE | Open an authorised saved paragraph with a quotation and factual ranges. Make a new revision while wording is pending. Revoke the exact egress consent before the simulated send. Drop stale proposals, preserve the revised original and show warranted local evidence. No unapproved fallback transfer, invented factual edit or native write. Use 0042 EB01–EB08 for existing wording-route conditions only. |

### Editorial resources and review checks (247–258)

Apply the saved-authority and revision rules above. The editor
selects a saved revision or explicitly submits a bounded local
snapshot for a check. Snapshot findings cannot change that saved
revision. Features below add no page queries, bytes or CPU work
for guests, and send no requests while an editor types.

#### Resources and stage policy

- **Image suggestions (247).** Search only already indexed
  asset metadata from the active site's admitted media library.
  Use spec 0021's admitted signed media scope from WordPress
  media or XenForo attachment authority. Bind the audience,
  exact media revision and current native read grant. Apply
  spec 0097 asset rights, permitted use and expiry. Show
  eligible alt text and source identity. An admitted complete
  scope with no eligible candidate says “No eligible images in
  this library”. An unadmitted or incomplete index says “This
  media library could not be checked”. Rank only eligible
  candidates using the admitted local keyword rule. A preview
  reads only an already held protected derivative. Match the
  source site and asset revision, native grant, exact source
  and derivative digests and 0097's preview rights for this
  audience and use. A source licence or cached thumbnail alone
  supplies no derivative authority. Recheck before showing it,
  when reopening a preview and on replay. Clear the affected
  preview when its proof no longer qualifies. A still-eligible
  metadata suggestion may say “Preview unavailable” and show
  only fields allowed on their own evidence. If the source grant
  is gone, withhold the affected suggestion and preview together.
  No fallback to a different rendition, direct URL or remote image.
  No asset fetch, remote search or generation. A preview grants
  no attachment authority. Recheck revision, rights and native
  permission if a person later accepts an exact attachment.
- **Templates (248).** Offer original, versioned outlines for
  a guide, review and comparison. Each has an ordered section
  list, plain prompts and an optional evidence slot per section.
  Guide has scope, materials, steps and pitfalls. Review has
  intended use, evidence, strengths and limits. Comparison has
  alternatives, common criteria, observed differences and limits.
  Offer each as a new reviewable outline in the existing draft
  flow. Existing draft text is untouched unless a person later
  reviews and accepts the exact proposal.
- **Author profile check (249).** WordPress only. Inspect the
  display name, biography and optional public author-page
  field for the author mapped to an eligible page. Require a
  signed spec 0021 profile-summary capability, binding site,
  page, author, revision, audience and exact safe field mask.
  Return only presence flags for already public fields.
  Each flag is present, empty, not exposed or unavailable.
  The native adapter treats whitespace-only as empty. Show field
  guidance. A verified same-origin native profile address may
  be shown as text without opening it.
  Do not read contact, account or login details, follow profile
  links, verify credentials or apply an edit. XenForo reports
  “This check is available for WordPress sites”.
- **Performance by author, category and format (250).** The
  desktop view groups admitted catalogue rows by one chosen
  dimension. Use one measure, source and closed window at a
  time. Join the page-day to its historical grouping only when
  the index proves one identity per dimension for the native
  day. Keep absent or unresolved identity out and explain
  coverage. Use the existing 0043 aggregation and source rules.
  Serve only a signed release projection with one site, source,
  measure, grouping dimension and closed source calendar week.
  Pin the release ID, evidence and privacy policy version. The
  source owner must admit protection against differencing across
  all windows, dimensions, revisions, totals, counts and
  complements. Where a projection cannot prove that protection,
  withhold it as a whole. Table, sorting and pagination reuse
  the same authorised release. No ad hoc aggregate, export,
  personal fields, public author score or individual visit path.
- **Category approval rules (251).** The owner may choose
  an admitted review requirement for an exact native category.
  Options are the existing native policy or an additional
  distinct authorised staff reviewer on the same revision.
  Child categories keep their own rules. Missing or ambiguous
  mapping blocks this added route. For all applicable rules,
  enforce the most restrictive outcome and keep every native,
  evidence and privacy check. Change rules through specs 0014
  and 0020. Show the exact category, current and proposed
  values. Bind the exact complete category set, site, object,
  saved revision, policy versions, reviewer accounts and current
  grant epoch into the existing approved publication plan.
  Recompute category and permission at approval and immediately
  before the admitted 0052 and 0021 effect. Require a distinct
  second active staff reviewer who is neither the author nor
  the requester. A moderator cannot sign twice using role
  changes, and a stale vote, moved category or new revision
  requires new review. Native policy and ordinary native
  actions retain their own authority. This added rule governs
  Sutradhar's category-controlled route only. An adapter that
  cannot enforce it at commit withholds that route. Review,
  ready stage and native publication remain separate.

#### Previews and exact checks

- **Fact-check flags (252).** Find candidates for numerical,
  price and date claims using the admitted 0026 text analysis
  and 0042 contradiction evidence. Show the exact passage, type
  and a pinned local claim-and-source record, including rights
  generation. Compare subject, time of applicability, unit and
  currency only when all relevant fields match. Keep supported,
  conflicting, evidence missing and ambiguous distinct. An
  uncertain subject or unmatched measure cannot create a
  conflict. An ambiguous numeral stays a possible claim.
  Distinguish code, quotations, versions and dates by profile.
  Conflicting claims require a person to resolve them. These
  flags do not establish truth,
  generate replacement facts or make an external request.
- **Search and social previews (253).** In the editor, use
  current local form fields and already loaded admitted assets.
  Render a search preview from title, description and eligible
  display address. Render social from those fields and the
  selected admitted 0096 destination profile and asset. Paint
  text and local media only. There is no active link, script,
  remote image or automatic asset fetch. An unavailable profile
  or asset has its own labelled incomplete state. Show estimated
  layout with a local clipping indicator. A changed field
  redraws only that presentation in the editor. No keystroke
  triggers a service or remote call, model or background check.
  A missing field shows its own placeholder. Say “Example
  preview. Search and social sites may show it differently.”
- **Repurpose a thread (254).** The editor chooses an eligible
  guest-visible thread and reviews one exact article draft under
  0052. The social draft can use that article as source only after
  a person publishes it through the existing native route and
  obtains current proof of the exact guest-visible revision.
  Then the owner supplies input and requests a social draft
  under 0096. Request a 0098 local newsletter draft next using
  the same authorised published article as factual evidence.
  A reviewed social draft may supply permitted presentation only.
  Bind site, scope, audience, stage and upstream revision IDs,
  supporting ranges, reuse rights and each separate approval.
  Recheck all lineage at each hand-off, replay and restore.
  Require current native publication and guest-visibility proof
  for the exact article revision used as factual evidence. A
  later publication or revision at the same URL cannot use the
  earlier receipt, even with identical title or text. Keep any
  earlier verified publication as dated history within its own
  current rights. Hold affected social and newsletter previews
  and hand-offs when the source article changes. Preserve the
  saved local draft under current access and leave its review
  held. An expressly requested new proposal needs eligible proof
  for the new revision and its own review. Revocation withholds
  every affected derivative. Restored copies stay subject to the
  same check. Review here grants no outside send or publication.
- **Experience and expertise (255).** For each eligible article
  revision show “What did you try?”, “How did you check the
  result?” and “What limits should readers know?”. Allow an
  author to supply references and mark an entry supplied,
  missing or not applicable with a reason. Distinguish a claim
  from a checked reference. The checklist reports no inferred
  qualification, public badge, quality score or evidence of
  expertise based on name, popularity or biography.
- **Editor accessibility checks (256).** Run on request from
  the editor on an exact bounded snapshot. Read its existing
  structured fields. Flag absent alternative text for an
  informative image, a declared decorative image with reading
  text, missing headings or a skipped level, empty link text
  and a missing media transcript field where the profile
  requires one. Verify each finding with source location,
  rule ID and coverage. Cases needing rendered layout or human
  judgement say “Check this manually”. No change or claim of
  conformance follows the output. Use 0042's media rules, rather
  than a second caption or transcript engine.
- **Score history (257).** Read only the approved 0042 measure
  and its saved revisions. The owning score store binds site,
  draft ID, saved revision, measure and rubric versions, scope,
  immutable result ID and source generations. For one entry an
  identical redelivery is idempotent; conflicting contents
  quarantine that entry. Show saved and measurement dates
  separately. An earlier valid score is history for that
  revision. Only a score bound to the current revision can
  describe the current draft. Compare points only for matching
  measure, rubric and scope. Keep formerly eligible removed
  points as undisclosed gaps. An admitted retention policy
  governs expiry. Recheck authority, scope, evidence erasure
  and epochs before display and after restore. Other scores
  never enter the history.
- **Internal content gaps (258).** Resolve admitted same-site
  topic and mention ranges from the current index. Count
  distinct eligible pages, using the admitted minimum and topic
  identity. Check current eligible topic pages for a qualifying
  direct answer through the existing 0042 related-question
  and overlap evidence. A repeat mention with no qualifying
  answer may suggest a cluster idea only with a signed complete
  candidate-universe receipt for that exact intent and site
  index generation. A retrieved shortlist, missing page, or
  stale, inconsistent or withdrawn evidence cannot prove
  absence. An existing answer points to an authorised page
  through the normal link review. Hold incomplete scopes.
  Merge aliases and overlapping intent before suggesting an
  idea. Recheck the exact intent, live evidence and actor scope
  again before saving the idea or staging an ordinary link.
  Never create a page or feed live link ranking.

#### F-0258 proof for a new cluster idea

0041 owns the advice and its REQ-0041-CHECKS and
DES-0041-W2-ANSWER-COVERAGE contract. Keep the positive answer
and absence cases separate. One permitted direct answer on the
exact intent and current same-site page revision supplies an
existing-answer result under 0042's own qualifying rules. It can
lead to ordinary link review. This result needs no claim about
every other page. It confers no cluster suggestion, link write or
authority over a page whose rights are missing.

A new cluster idea needs the QR-128 qualified count profile and
complete-universe receipt. Count each distinct eligible published
canonical page with an entitled mention once for the resolved intent
and frozen site generation. Repeated spans, aliases and equivalent
occurrences on the same page add no page. QR-128 must bind the
count's approved minimum, units, eligibility, provenance and version.
The source floor in F-0036 and a search shortlist supply no minimum.

The complete-universe profile must identify its single authorised
producer, reader, signed purpose and lawful source and reuse scope.
Bind the exact site, normalised intent and its version, site index
generation, applicable eligibility profile, canonical page and
revision set commitment, and current coverage and authority proofs.
The profile must explain how each required page is included or its
exclusion independently proved. Resolve all eligible candidates for
the direct-answer check. An unassessed or unreadable candidate
cannot prove absence. An otherwise authentic receipt for another
intent or index generation supplies no coverage even when the set
and counts appear equal. The fact that 0022 has an index and 0042
lists related questions grants no completeness authority.

Until both profiles qualify, or when evidence is incomplete, say
“Not enough evidence to check site coverage” and offer no new
cluster idea. A separately eligible existing answer keeps its own
link-review path. Recheck scope before display, replay, restore and
any existing eligible save. No new save route, public-page work or
ranking input follows this proof.

#### Common records and proof

Bind a resource, result or draft to site, actor, content ID,
immutable revision or local snapshot generation, source evidence
IDs, rule and policy versions, optional exact ranges and coverage
state. Rights, visibility, erasure, scope and revision are checked
before analysis and before display, copy or review. If scope
changes, clear live results and withhold stale advice. Fan-out Copy
uses 0117's active site, Cedar and exact revision checks at
clipboard dispatch. Sealed storage and disposal follow 0032 and
0118. Use the owning
modules for histories, media, analytics, drafts and publication.
A failed rule labels the missing result without guessing.
QR-128 requires exact source, native and privacy evidence.

| Requirement | Verification contract |
|---|---|
| REQ-0041-RESOURCES | IT-0041-RESOURCES: filter own licensed media before ordering. Match an already held preview to the exact asset, derivative and current 0097 preview grant. Independently change the derivative or its rights while keeping eligible source metadata. Hold stale, expired, remote or wrong-site renditions. Distinguish a complete empty library from incomplete scope. Render each local outline independently and show its original prompts and evidence slots without editing a draft |
| REQ-0041-AUTHOR | IT-0041-AUTHOR: test the exact WordPress presence mask, whitespace, missing rights, a remote author address and XenForo refusal. Compare historical author, category and format scopes; reject mixed or unknown joins. Mask overlapping totals, memberships and differing releases; a source unable to protect suppressed complements returns no projection |
| REQ-0041-POLICY | IT-0041-POLICY: confirm an exact category rule, then change the category or policy during native review. Require distinct current reviewers for that revision. Deny wrong actor, author as second reviewer, lost grants, stale approval, competing saves and a weaker override. Refuse an adapter that cannot enforce the current rule at commit |
| REQ-0041-PREVIEWS | E2E-0041-PREVIEWS: provide unsaved fields, missing media, a remote asset reference, an expired native and social profile, and international text locally. Asset fetches and active links stay off. Watch traffic while typing. Run accessibility on request; show exact evidence or a manual check. No provider request, draft write or guest footprint |
| REQ-0041-CHAIN | IT-0041-CHAIN: stage article, social and newsletter in order. Hold social before exact guest-visible publication proof. Move the current article from R1 to R2 at the same URL before preview or hand-off. Require exact current article proof and separately reviewed drafts. Test withdrawn quote, changed audience, replay and restore. No copied authority, outside send or publication by the chain |
| REQ-0041-CHECKS | IT-0041-CHECKS: seed unsupported and comparable contradictory facts plus mismatched subjects, dates, units and currencies. Include a voluntary checklist, changed rubric on the same draft revision, a private source, duplicate and conflicting score replies, withdrawal after restore, and a qualifying answer outside the shortlist. Show evidence, gaps and refusal distinctly. Never infer authority or write content |

#### Wave 2 fixture bindings for F-0247 to F-0258

Each fixture specialises an existing verification contract above.
Use synthetic admitted inputs and stand-in native outcomes. QR-128
still requires separate source, native and privacy evidence for each
profile. Keep missing evidence visible and refuse the affected route.

| Feature and requirement | Design and fixture | Evidence to require |
|---|---|---|
| F-0247 · REQ-0041-RESOURCES | DES-0041-W2-MEDIA-SUGGESTIONS · FX-0041-W2-MEDIA-SUGGESTIONS · IT-0041-RESOURCES and 0097 IT-0097-ASSETS | Show eligible already-indexed own-site metadata and one exactly bound local preview. Keep the metadata grant and change the preview digest, use grant or expiry before display. Show only eligible metadata with Preview unavailable. Reopen or restore the older cached preview and withdraw the native asset grant. Refuse the stale pixels and hide the whole affected suggestion on source withdrawal. Distinguish a complete empty index from incomplete scope. No URL fallback, fetch, conversion or attachment. |
| F-0248 · REQ-0041-RESOURCES | DES-0041-W2-LOCAL-TEMPLATES · FX-0041-W2-LOCAL-TEMPLATES · IT-0041-RESOURCES | In the same authorised draft flow, compare the ordered prompts and optional evidence slots for the guide, review and comparison. Each becomes its own reviewable outline. Preserve the opening draft revision and original text. |
| F-0249 · REQ-0041-AUTHOR | DES-0041-W2-WP-PUBLIC-FIELD-PRESENCE · FX-0041-W2-WP-PUBLIC-FIELD-PRESENCE · IT-0041-AUTHOR | Use a synthetic WordPress signed profile-summary grant for the exact public field mask. Distinguish present, whitespace-only, unexposed and unavailable. Refuse on withdrawn or missing rights, wrong scope and XenForo. Return presence only. No profile navigation or credentials. Hold the route without a proven 0021 capability. |
| F-0250 · REQ-0041-AUTHOR | DES-0041-W2-AUTHOR-RELEASE · FX-0041-W2-AUTHOR-RELEASE · IT-0041-AUTHOR | Compare one compatible signed release for one site, source, metric, grouping and closed native week with same-day ambiguous membership and an overlapping total. Exclude unknown page-day mappings. Withhold the projection if complementary or cross-release differencing is possible. Sort and page the same authorised release. Withdrawn source withholds the release. |
| F-0251 · REQ-0041-POLICY | DES-0041-W2-CATEGORY-REVIEW · FX-0041-W2-CATEGORY-REVIEW · IT-0041-POLICY | Use two native categories and a revision requiring a distinct active reviewer. Try the author and requester as reviewer, move category or change the policy before commit, then remove commit-time enforcement. Confirm only the exact current policy and review under an admitted adapter. Hold stale approvals and keep native rules in force. |
| F-0252 · REQ-0041-CHECKS | DES-0041-W2-CLAIM-EVIDENCE · FX-0041-W2-CLAIM-EVIDENCE · IT-0041-CHECKS | Compare matching supported and conflicting price or date records with mismatched subjects, effective dates, currency or unit, missing evidence and a code literal. Check the exact passage, admitted type, source and rights generation. Keep ambiguity separate from conflict and missing evidence. No lookup or invented correction. |
| F-0253 · REQ-0041-PREVIEWS | DES-0041-W2-LOCAL-SEARCH-SOCIAL-PREVIEW · FX-0041-W2-LOCAL-SEARCH-SOCIAL-PREVIEW · E2E-0041-PREVIEWS | Type a title and description locally and change profile. Check an absent field, remote asset and expired profile. Show named incomplete states and a local approximation with its limit. Observe no network activity, active link, model or service check on keystrokes. |
| F-0254 · REQ-0041-CHAIN | DES-0041-W2-REPURPOSE-LINEAGE · FX-0041-W2-REPURPOSE-LINEAGE · IT-0041-CHAIN | Review one exact article draft. Hold social until independent native proof of that published guest-visible revision. Then stage distinct local social and newsletter drafts from R1. Publish R2 at the same URL with the same title and text, then replay the valid old receipt and try preview, hand-off and restore. R1 keeps only authorised dated history and no current downstream approval. Hold affected previews and drafts for current review. A separately requested R2 proposal uses exact proof and independent review. Withdraw a quote as a separate case. Reuse BDD-0096-COMPOSE and IT-0098-NEWSLETTER. No automatic send or publication. |
| F-0255 · REQ-0041-CHECKS | DES-0041-W2-ARTICLE-CHECKLIST · FX-0041-W2-ARTICLE-CHECKLIST · IT-0041-CHECKS | Bind the checklist to the exact article revision. Give a supplied claim with a checked reference, a supplied claim alone, a missing answer and a reasoned not-applicable entry. Keep those states distinct. Show no inferred author expertise, score or badge. |
| F-0256 · REQ-0041-PREVIEWS | DES-0041-W2-SNAPSHOT-ACCESSIBILITY · FX-0041-W2-SNAPSHOT-ACCESSIBILITY · E2E-0041-PREVIEWS | Explicitly check a bounded snapshot. Include informative and decorative images, empty link text, skipped headings and a layout-only concern. Require exact location, rule and coverage for a supported finding and “Check this manually” for the layout case. Preserve the draft and metadata. |
| F-0257 · REQ-0041-CHECKS | DES-0041-W2-APPROVED-SCORE-HISTORY · FX-0041-W2-APPROVED-SCORE-HISTORY · IT-0041-CHECKS and 0042 EA13 | Supply the one admitted 0042 measure on two eligible saved revisions. Redeliver an identical result, deliver a conflicting result for the same key and change rubric on the same revision. Deduplicate the replay, quarantine the conflict and compare matching measure, rubric and scope only. Withdraw evidence and restore. Keep removed points as undisclosed gaps. No private, unsaved or additional scoring route. |
| F-0258 · REQ-0041-CHECKS | DES-0041-W2-ANSWER-COVERAGE · FX-0041-W2-ANSWER-COVERAGE · IT-0041-CHECKS | Compare distinct eligible pages and a known qualifying answer, a signed complete candidate universe with a missing answer, and an incomplete shortlist omitting a qualifying answer. Respect alias and intent merging and the admitted minimum. Guide review to the known answer, offer one idea only for proved missing coverage, and hold the shortlist case. No native creation or live rank input. Use an otherwise eligible existing-answer control under 0042 with no absence receipt. For the cluster branch use a fixture-only three-page minimum and separate complete eligible absent-answer groups with two and three canonical mention pages. Repeat mentions on one page or its alias. Only the equality case with exact current proof offers a cluster idea. Omit the minimum, replace the receipt with a shortlisted subset, and separately offer a valid same-looking receipt for a different intent or index generation. Report insufficient evidence and offer no idea. The fixture supplies no production minimum or signing authority. |

The [keyword lookup](0132-dataforseo-keyword-research.md) opens
only on request. Saved keyword evidence can inform the local brief
under 0132. Typing never calls the provider.

### Wave 1 editorial evidence and advice bindings

Use the existing edit card, protected evidence pack and exact saved
revision. Bind site, actor, purpose, rights, source and revision,
eligible population, coverage and freshness. Preserve the existing
blind-test and evidence-floor gates. Unknown, private, suppressed or
withdrawn facts cannot establish a claim or reach an unauthorised
view. Keep evidence when a wording route is unavailable.

| Design and variant | Admitted boundary |
|---|---|
| DES-0041-W1-ADVICE and VAR-0041-W1-STAFF-ADVICE | Current staff edit cards or a local proposal on the exact cited revision. No native effect, alert or ranking input. |
| DES-0041-W1-ANSWER-ASSESSMENT-REFUSAL | F-0027 stays held under QR-104. The outcome cannot be inferred from the F-0025 rescue card or the F-0258 page-level coverage check. A later evidence-only or review-only scoring route needs its own approved contract. |
| DES-0041-W1-CITED-DRAFT and VAR-0041-W1-LOCAL-CITED-DRAFT | Local drafts from eligible sources and the approved release projection, before separate staff approval. |
| DES-0041-W1-SUMMARY and VAR-0041-W1-SUMMARY-EVIDENCE | Cite and revalidate exact covered posts and pagination as applicable. Native summary presentation and safety remain with 0052. |
| DES-0041-W1-BRIEF and VAR-0041-W1-SAME-SITE-BRIEF | The existing prepare_writing_brief route uses only current eligible same-site sources and the recorded feature 36 minimum. |

| Feature and binding | Planned fixture and result |
|---|---|
| F-0025 · REQ-0041-W1-RESCUE · DES-0041-W1-ADVICE · VAR-0041-W1-STAFF-ADVICE | FX-0041-W1-RESCUE, BDD-0041-W1-RESCUE. Contrast high demand with weak and complete answers and evidence below the admitted floor. Only the supported rescue shows a staff card. An expert alert needs its separate approved native route and eligibility; a weak accepted answer does not automatically qualify under 0051. |
| F-0027 · REQ-0041-W1-ANSWER-ASSESSMENT-HOLD · DES-0041-W1-ANSWER-ASSESSMENT-REFUSAL | FX-0041-W1-ANSWER-ASSESSMENT-HOLD. The planned check is BDD-0041-W1-ANSWER-ASSESSMENT-HOLD. As site-authorised staff, request the unadmitted assessment for synthetic accepted answers: a complete answer, an answer missing a prerequisite and an answer dependent on a private source. Repeat with a changed native revision and an old score in replay. Withhold the assessment, grade, flag and source text. Verify unchanged answer text and accepted state. No stored ranking value, alert, native write or new intake. This fixture covers only the QR-104 refusal. |
| F-0028 · REQ-0041-W1-CONSENSUS · DES-0041-W1-CITED-DRAFT · VAR-0041-W1-LOCAL-CITED-DRAFT | FX-0041-W1-CONSENSUS, IT-0041-W1-CONSENSUS. Exercise a repeated recommendation from the same permitted source, disagreement and source erasure. Use the approved counting unit and population; hold without their exact proof. Cite each supported claim and withdraw it after its evidence expires. No new person join or hub publication. |
| F-0029 · REQ-0041-W1-STATE-REPORT · DES-0041-W1-CITED-DRAFT · VAR-0041-W1-LOCAL-CITED-DRAFT | FX-0041-W1-STATE-REPORT, BDD-0041-W1-STATE-REPORT. Compare eligible closed polls and trends, a sparse group and an erased source. Use 0034 disclosure and 0094 provenance. Draft a dated report with the supported measures, population and limitations. Protect withheld groups and complements. No member data or publication. |
| F-0030 · REQ-0041-W1-COMPARISON · DES-0041-W1-CITED-DRAFT · VAR-0041-W1-LOCAL-CITED-DRAFT | FX-0041-W1-COMPARISON, BDD-0041-W1-COMPARISON. Use exact named subjects, an ambiguous identity and withdrawn evidence. Cite eligible posts, keep disagreement visible and withhold unsupported comparison. Spec 0085 keeps its separate draft form authority. |
| F-0031 · REQ-0041-W1-PRODUCT-DRAFT · DES-0041-W1-CITED-DRAFT · VAR-0041-W1-LOCAL-CITED-DRAFT | FX-0041-W1-PRODUCT-DRAFT, IT-0041-W1-PRODUCT-DRAFT. Compare similar names, a verified model, an absent or conflicting Wikidata fact and exact replay. The current 0049 identity joins only eligible discussions. Exact replay returns the existing draft identity for that accepted intent. Withhold uncertain identity. Use already-admitted 0049 facts; the card cannot start a lookup. |
| F-0032 · REQ-0041-W1-PRUNING · DES-0041-W1-ADVICE · VAR-0041-W1-STAFF-ADVICE | FX-0041-W1-PRUNING, BDD-0041-W1-PRUNING. Compare an old sparse page with observed demand, an eligible low-demand duplicate and missing observations. Show the exact 0044/0047 evidence and supported improve, merge or noindex advice. Missing traffic remains unknown. No change is applied. |
| F-0033 · REQ-0041-W1-THREAD-CONTENTS · DES-0041-W1-SUMMARY · VAR-0041-W1-SUMMARY-EVIDENCE | FX-0041-W1-THREAD-CONTENTS, IT-0041-W1-THREAD-CONTENTS. Bind an outline or per-page source preview to exact covered revisions and pagination. A move invalidates the affected result. Source gaps stay visible. Public contents and per-page blocks remain held under the separate native placement and footprint contract. |
| F-0034 · REQ-0041-W1-SPLIT-PROPOSAL · DES-0041-W1-ADVICE · VAR-0041-W1-STAFF-ADVICE | FX-0041-W1-SPLIT-PROPOSAL, BDD-0041-W1-SPLIT-PROPOSAL. Compare two supported distinct questions with a single topic, then reorder posts after preview. Hold an unsupported or stale split. Source-linked staff advice grants no native move, redirect, publication or moderation notice. |
| F-0035 · REQ-0041-W1-CITED-SUMMARY · DES-0041-W1-SUMMARY · VAR-0041-W1-SUMMARY-EVIDENCE | FX-0041-W1-CITED-SUMMARY binds the existing 0052 covered-reply, uncovered-append and guest-visibility cases. Cite the exact covered range. An uncovered append keeps the approved label and link to the first uncovered reply in the admitted 0052 variant. A change to a covered reply invalidates the summary and invokes 0052's hide fence. No new public placement or duplicate native test. |
| F-0036 · REQ-0041-W1-BLOG-BRIEF · DES-0041-W1-BRIEF · VAR-0041-W1-SAME-SITE-BRIEF | FX-0041-W1-BLOG-BRIEF, BDD-0041-W1-BLOG-BRIEF. Test one fewer distinct eligible source than the recorded lab minimum, exactly that minimum, withdrawal and a paired-site source. Qualifying sources are guest-visible and research-qualified. Join 0132 FX-0132-SAVED-BRIEF-BINDING to show that 20 eligible keyword rows cannot replace an absent minimum or a missing qualifying page. Revoke a stale outline and refuse the foreign text. Pair-only overlap permission transfers neither content nor brief scope. No publication. |

DR-101 permits opt-in local desktop typing only under 0117's pinned
matrix and revision gate. These ordinary Coach cards inherit none
of that route's typing, rival or separate storage authority. The
0132/0133 paid routes under DR-117/118 require their own exact
request and approval. Read an independently admitted saved
projection only when the current local scope permits. No draft
event, replay or keystroke starts a paid lookup. Spec 0135 owns
verified AI fetches and separate referral reports under DR-122;
it grants no editorial workspace, provider or scoring authority.

QR-104 retains unresolved exact counts, source scopes, public
effects and footprints. The existing 0041 lab minimum still
governs the brief. Fixture and variant identities name planned
obligations and do not establish a successful run or route admission.

### Answer completeness options (F-0027)

Under QR-104 the owner can retain the feature's original score and
staff review flag through a separately admitted rubric, or choose
evidence-only advice in its place. Scoring needs an explicit staff
exception to the 0041 and 0042 score boundary. Neither option supplies
permission
to build or run an assessment before the exact route qualifies. Keep
the existing F-0027 refusal and its planned negative case.

Use the existing protected 0041 staff edit card. An approved action
opens one exact question and its accepted reply on the active site.
Require current 0020 and 0101 rights and a separately qualified
platform profile for the native acceptance fact. Bind site, thread,
native acceptance identity and generation, title and first-post
revision, accepted reply ID and revision, 0022 content generation and
read profile, rubric version, source coverage, and current grant and
erasure generations. Recheck before display, replay or restore. Only
already held guest-visible ranges may supply evidence. A forum's
observed acceptance proves which reply was chosen. Votes, rankings
and text similarity never establish acceptance. Reuse 0022's
mark, replace and unmark evidence. Withhold an unqualified adapter
including a WordPress forum adapter without exact native proof.

Use fixed ordered required items from the exact question. The
candidate rubric checks the direct answer and any additional
explicit requirement or prerequisite. Pin the item roster, rubric
and reviewer rules before inspecting the answer. Keep the direct
item first and the remaining distinct requirements in source order.
Each item cites a question range and carries an evidence outcome
and permitted answer range. The proposed bound is 1–12 distinct
items per exact question. Refuse an empty set or a thirteenth item
with insufficient evidence and no partial score. QR-104 must admit
the exact extraction, source and assessment bounds before use.
Use “Addressed”, “Missing from this answer” or “Needs review”.
An admitted local method must support each outcome. A missing
passage counts as an omission only with complete, eligible answer
coverage and a clear question requirement. Ambiguous requirements
or incomplete coverage give “Needs review”. The card describes
question coverage. A bare restatement supplies no answer evidence.
For a reply incompatible with a required item, show its gap when
the eligible evidence proves it. Otherwise show “Needs review”.
Factual correctness outside the eligible evidence
requires a separately admitted check. Evidence-only advice shows
the items and their reasons without an aggregate or a weak flag.

For the score option the candidate calculation counts addressed
items out of all required items, each once. Label this fraction
“Required items addressed”
and show the rubric version and reasons only for a non-empty review with
every item resolved. Do not drop an unknown item to raise the score.
A staff-only flag needs the rubric's separately approved weak-result
predicate. The 0056 proof and the 0041 blind review must qualify the
exact rubric, abstention rules and flag before use. The candidate
sets no shipping pass mark. For an eligible weak result, show
“Review answer coverage” on the matching card and beside its
entry in the existing protected staff review view. Recheck current
acceptance and staff rights in both places. A separately admitted
forum adapter produces the same score, reasons and flag for the
same eligible evidence. An adapter without exact acceptance proof
has no result or flag. The result has no general Coach grade,
0042 F-0235 saved-draft score surface, F-0257 score history, RS16
ranking input, provider request, native notification or change to
the question, answer or acceptance state. An existing editor hint
tool grants no access to these assessment fields. Use separate
synthetic editorial qualification. No 0057 ranking dataset, judge
label, protected quality threshold or ranking calibration is reused.
An alias or restore cannot deliver an editorial result to ranking.

Say “No accepted answer” only with a current authoritative unmark or
confirmed empty acceptance. Say “This answer changed. Review the
current version.” when a still-authorised reader's snapshot is stale.
For incomplete, unsupported or insufficient evidence say “We don’t
have enough evidence to check this answer.” Deny a foreign or
withdrawn view through existing scoped refusal without exposing
protected source or reason details. A prior result covers only its
original eligible version and cannot become the current result on
replay. Use existing run retention and erasure rules.

Extend the F-0027 planned test with separate synthetic trials only
after binding the selected route. Use three explicit required items.
A direct reply that supplies all three has three addressed items. An
eligible complete reply lacking the third prerequisite has two
addressed and one missing. Evidence-only shows those rows. Under the
score candidate they yield 3/3 and 2/3. Use a fixture-only weak flag
rule requiring every item, so only the second score has a staff
flag on both protected views. It supplies no product threshold.
Check equivalent eligible evidence on each admitted forum adapter.
An ambiguous third item
gives no score or weak flag. In separate replies merely repeat the
question or explicitly contradict the required prerequisite. Neither
counts as addressed. A private-dependent assessment shows
only the generic insufficient-evidence message. Verify exact
same-version replay, then accept another native reply with identical
text on a new acceptance generation, edit the question, and unmark
the reply in separate trials. No old result becomes current. Revoke
staff access while display is staged and restore an earlier copy.
Neither discloses the result. Compare no items, 12 eligible items
and 13 otherwise eligible items under the candidate bound. The
middle case can proceed after qualification. The others are held.
Check no state change, native alert,
public-page work, provider activity or ranking input.

### Left out on purpose

Typing-time checks; comparing colleagues' unsaved drafts; a separate
content workspace; cross-owner reports; rival pages; automatic
publishing; co-editing; branches; AI-text detectors; organisation fact
registers. A site's typed business details
([0047](0047-site-structure-checks.md)) are not a fact register: they
are one record per site, compared with its pages, never treated as the
truth and never used to settle a contradiction.

DR-101 admits live draft checks and the fan-out rehearsal in
[spec 0117](0117-editor-coach-query-fan-out.md). DR-115 assigns
feature 238 to the separately authorised
[one-off public page comparison](0131-one-off-rival-page-comparison.md).

Translated editions (feature 98) are after version 1 under DR-081.
Version 1 offers no translation drafts, regional demand jobs or
translated page publication through the Coach.

### Logic and maths

Decision logic is Rust, proved with Kani where it decides. Gap and
uncertainty maths are in ganit. The site add-ons make no outgoing
requests and do no work per keystroke.

### Start a scientific experiment

An eligible current suggestion can offer **Start experiment** under
[0148](0148-scientific-experimentation-engine.md). Pass only its
same-site versioned reference. The owner separately approves
registration and any eligible exposure. Taking or dismissing ordinary
advice creates no experiment or page reservation. The Coach may show
a current 0151 evidence receipt. A guidance weight or rule state
changes only through its exact owner-confirmed advisory effect.

## Limits

- Models and every Coach job run inside the service slice, within the memory
  limits in AGENTS.md section 5.6. The Coach screens run in the desktop
  window's cap.
- The paid route is never called without a key and a spending cap.
- Member quotes are capped at 50 words by default.
- Sample-size floors and the bounds for any paid spending cap are readiness
  blockers owned by the Coach evaluation research record. A feature that
  depends on a sample floor cannot build until its floor, fixtures and
  evidence are admitted; it shows "not enough evidence" at run time. The
  paid route remains off until the owner sets a cap inside admitted bounds.

## Footprint budget

This part adds nothing to public pages. Proposed revisions reach the
site's own editor for staff and change nothing until staff publish.

The Coach's editorial plan stays local. Outside calendar connections
and iCalendar export are not built (spec 0100, DR-076).

## Wave 1 saved editorial review bindings

Use the authorised saved revision, exact cited source ranges,
site, actor, audience, purpose, rights and current grants.
Withdraw a proposal if its revision, source or authority changes.
These bindings add no copy egress, native edit, provider request
or public-page work. Existing wording and approvals retain their
own owners. DR-101/0117 gives no typing or fan-out authority here.

| Feature and owning design | Exact planned fixture and boundary |
|---|---|
| F-0089 · REQ-0041-W1-QUOTE-REVIEW · DES-0041-W1-MEMBER-ADVICE | FX-0041-W1-QUOTE-REVIEW, IT-0041-W1-QUOTE-REVIEW. Compare an admitted quote-heavy reply, an essential credited quote and a later edit. Use 0026’s proved quote boundaries and 0089 review, with the approved sample and evidence rule. Keep attribution and later edits. Hold unsupported trimming. Any native member-text change still needs its separate purpose-specific authority and current moderator approval; the existing narrow 0029 edit option supplies no quote-rewrite permission. |
| F-0090 · REQ-0041-W1-NOISE-REVIEW · DES-0041-W1-MEMBER-ADVICE | FX-0041-W1-NOISE-REVIEW, BDD-0041-W1-NOISE-REVIEW. Contrast an eligible low-content reply, a substantive reply and unsupported WordPress reactions. Use only admitted 0051 moderation evidence and a supported site. Show a local moderation proposal while retaining text and authorship. No conversion without a qualified native route, and no deletion, reaction or actor impersonation by the proposal. |
| F-0091 · REQ-0041-W1-TROUBLESHOOTING · DES-0041-W1-CITED-LOCAL-DRAFT | FX-0041-W1-TROUBLESHOOTING, shared UT-0084-PACK and BDD-0085-HOWTO. Compare exact solved-thread evidence with proved prerequisites, an unproved step and a private or withdrawn source. Use current 0041 member-answer permissions, preserve credit and cite each supported step. Hold the affected draft on withdrawal. No invented instructions or publication. |
| F-0092 · REQ-0041-W1-VERSION-TAG · DES-0041-W1-EXACT-METADATA-ADVICE | FX-0041-W1-VERSION-TAG, BDD-0041-W1-VERSION-TAG. Compare a proved product and version span, ambiguous numbers and an updated post with an older tag. Use eligible 0049 identity evidence for the exact revision. Show a scoped tag or prefix proposal only if the match is proved. No native tag, prefix or identity change. |
| F-0093 · REQ-0041-W1-EXPIRED-CONTENT · DES-0041-W1-DATED-ADVICE | FX-0041-W1-EXPIRED-CONTENT, BDD-0041-W1-EXPIRED-CONTENT. Use a verified end date, an uncertain date and an eligible current replacement under the existing 0090 dated-plan rules. A changed schedule invalidates the preview. Show supported dated notice text or evidence alone; invent no successor. No label, redirect, text change or expiry job follows. |
| F-0094 · REQ-0041-W1-SNIPPET-ADVICE · DES-0041-W1-CITED-LOCAL-DRAFT | FX-0041-W1-SNIPPET-ADVICE, BDD-0041-W1-SNIPPET-ADVICE. Compare a qualifying exact answer and an admitted matching snippet observation with absent or mismatched evidence. Offer supported concise definition, list or table advice. Without proved 0040 source and scope, withhold the snippet claim. No search display promise, provider refresh or page change. |
| F-0095 · REQ-0041-W1-DOCUMENTATION-CITATION · DES-0041-W1-CITED-LOCAL-DRAFT | FX-0041-W1-DOCUMENTATION-CITATION, IT-0041-W1-DOCUMENTATION-CITATION and shared UT-0084-PACK. Compare current owner-approved official documentation evidence with a retired domain and unapproved redirect. Cite the exact admitted address and version with accurate label and provenance. Revoke the draft on source withdrawal. No fetch, redirect-follow, insertion or borrowed trust from a familiar domain. |
| F-0096 · REQ-0041-W1-ARTICLE-FAQ · DES-0041-W1-CITED-LOCAL-DRAFT | FX-0041-W1-ARTICLE-FAQ, BDD-0041-W1-ARTICLE-FAQ. Compare an eligible same-site WordPress article and authorised forum-question evidence with private and withdrawn questions and another platform. Only a current, scoped WordPress FAQ draft may follow. Use 0085 source and 0090 local hand-off rules. A pair or writing licence grants no content transfer, native insertion or public markup. |
| F-0097 · REQ-0041-W1-ARCHIVE-INTRO · DES-0041-W1-CITED-LOCAL-DRAFT | FX-0041-W1-ARCHIVE-INTRO, BDD-0041-W1-ARCHIVE-INTRO. Compare thin and sufficient archived content, a protected member source and a later edit. Give an evidence-backed local intro only for the eligible exact archive. Hold private or revoked evidence and a stale proposal. The 0049 tag-description effect retains its own scope and approval. No intro or tag text is applied here. |
| F-0098 · REQ-0041-W1-TRANSLATED-EDITION-DEFER · DES-0041-W1-SITE-LOCALE-REFUSAL | FX-0041-W1-TRANSLATED-EDITION-DEFER, E2E-0041-W1-TRANSLATED-EDITION-DEFER. Restore a translated-edition preference, scheduled draft and pending hreflang effect. Refuse the deferred DR-081 feature across owner, staff, agent and restore routes. DR-113 writing-language scope and the F-0564 observation audit supply no edition, market, schedule, publication or F-0043 markup authority. No positive F-0098 result in version 1. |
| F-0099 · REQ-0041-W1-ABANDONED-DRAFT · DES-0041-W1-SAVED-DRAFT-SCOPE | FX-0041-W1-ABANDONED-DRAFT, IT-0041-W1-ABANDONED-DRAFT. Compare an old but still current authorised saved WordPress staff draft, an inaccessible colleague's draft and a private member draft. Filter on current ownership and purpose before search, counts, status or view. Show supported demand evidence on the eligible draft only; hold without an admitted matching evidence scope. No cross-owner or member indexing, revival, exposure or publish action. |
| F-0100 · REQ-0041-W1-CONTENT-CALENDAR · DES-0041-W1-DATED-ADVICE | FX-0041-W1-CONTENT-CALENDAR, shared BDD-0041-CALENDAR and UT-0090-CALENDAR. Compare eligible seasonal date evidence with missing demand, a local clock boundary and a revised plan. Show a bounded staff-only local plan with assumptions, proposal dates and exact current version. A due date creates no dispatch, publication, outside calendar or connector authority. |

Keep existing source, drafting and native checks with their owners.
QR-108 retains exact source, rights, effect and platform admission.
These named fixtures trace the existing Wave 1 scenarios and refusal
boundaries; they supply no implementation or execution evidence.

## How it is tested

Rust tests for each point:

- EC01: every tip has a page version, a source and a date, or it is not
  shown.
- EC02: not enough data gives "not enough evidence", with no forecast.
- EC03: keep-the-original leaves the page unchanged.
- EC04: nothing is applied or published without a person.
- EC05: a human edit after a preview cancels that proposal.
- EC06: another person's draft, or private text, never reaches a route
  that leaves the PC.
- EC07: sending a draft shows the notice first, every time.
- EC08: a paid route with no key or cap is never called.
- EC09: with no wording route, evidence still shows and no wording is
  invented.
- EC10: a denied page, another owner's site, missing GA4 and suppressed
  counts each give a clear, correct result.
- EC11: a plausible claim with no source is flagged.
- EC12: the paired-site check is off by default and refuses different
  owners.
- EC13: the Coach stays off until the recorded blind test passes.
- EC14: a Coach warning never blocks a draft; a hard check still does.
- With no recorded minimum, no brief is made. A brief is refused with
  one fewer eligible source than the lab minimum. At the minimum, it can
  proceed when other checks pass.
- An MCP draft is stored only as a suggestion and is never applied
  without a moderator's acceptance.

## Changes from the source

- Local writing tips use an admitted packaged generative-model
  application behind typed Rust integration; the owner's MCP tool may
  draft wording only as a suggestion a moderator accepts (AGENTS.md 5.5).
  The local route is unavailable until its admission blockers close.
- The paid route is AWS Bedrock with the owner's own key.
- Sutradhar implements no local model runtime or first-party Python
  production integration.
- Decision logic and adapters are rewritten as Rust, proved with Kani;
  maths uses ganit.
- The admin app becomes the desktop app built from SDS.

Source: `docs/specifications/signals/editorial-coach.md` in the archive
`archive-2026-11-08-old-repo`.
