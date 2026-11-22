# 0042: Coach additions

These additions widen the Editorial Coach
([0041](0041-editorial-coach.md)). They help an editor see what a page
lacks, where it disagrees with the rest of the site, and what readers
still ask; set one rule for which model writes the words; give staff a
first draft of a new piece; warn on near-copy drafts; check whether
outside chatbots name the site; and let reader habits help pick hub
topics. All of it is advice only. It is part of roadmap Phase 6; "Which
spec designs each feature" in [features.md](features.md) lists the
features it designs in whole or in part.

Example: an editor pastes a Word draft of a router guide. The Coach
says "the answer first appears in section 4"; "this guide says the
warranty is 2 years, another says 3"; "readers also ask how to reset it,
and a thread answers that, so link it". Nothing changes by itself.

## What it does

### Reuse

No second checker or workspace, and the Coach adds no score of its
own; feature 235's score is a separate, approved measure.

| New part | Reuses |
| --- | --- |
| Narada checks | Edit card; `check_draft`; content draft checks |
| Contradiction tip | Content index passages and versions; draft diff |
| Paste and upload | `check_draft`; the "leaves your computer" notice |
| Related questions | Search Console ([0040](0040-search-connectors.md)); SEO jobs 79, 265, 289 |
| Wording order | Coach wording routes; notice EC07; cap EC08 |
| AI readability | `check_draft`; the wording order |
| AI drafting | Content drafting route, evidence pack and approval ([0052](0052-hubs-summaries-drafts.md)) |
| Near-copy warning | Content novelty check (shared 8-word runs) |
| Chatbot evidence | Supplied files under spec 0093; `compare_edit_outcomes` |
| Reader patterns | Hub topics (ganit job 278); darshak flows |

### Narada's new checks

Narada is the module that holds these checks. Claims needing sources,
novelty, freshness, readability, orphans and cannibalisation are already
covered elsewhere. These eight are new:

| Check | Asks | Reads |
| --- | --- | --- |
| Answer early | Is the main question answered near the top? | Text |
| Enough facts | Are there real claims, or padding? | Text |
| Ages fast | Which prices, versions, dates will go stale? | Text |
| Alt text useful | Adds more than nearby words; no stuffed keywords? | Text |
| Caption adds | Does the caption add context? | Text |
| Transcript present | Does audio or video have a transcript? | Text |
| Fits its kind | Guide, product, category, forum: own rubric | Text |
| Observed, not judged | Tips say what was seen, not "low quality" | Wording |

- Each is advice. Only the approved hard checks ever block.
- Every tip names its check and check version. The page type picks the
  rubric, so no universal word count applies. No pass mark, no score.
- Phone and desktop first-screen previews need the crawler's headless
  browser check to be admitted. Until then they say "not measured".
- Alt text is compared by words only. Matching a picture to its alt
  text needs an image model and is not in version 1.
- Media tips apply to staff pages and guides first, not member posts.
- No waivers: nothing blocks, so nothing needs one. An editor may set a
  tip aside; it is noted in history, needs no reason and hides nothing.

### Editor content score display

Feature 235 shows the same approved draft content score in both site
editors. The measure is one the method lab admits (spec 0056). The
service scores only saved drafts of guest-visible content: drafts the
content module holds (spec 0052) and posts a staff member sends for
scoring through the signed contract of spec 0021. Conversation drafts
and other private text are never read. It scores a saved version when
it next calls the site and stores the result through each platform's
own contract, in the XenForo add-on's or the WordPress plugin's tables,
since the site never calls the service. Both editors show that stored
score for the saved version and measure version, and say when it is
older than the text on screen. Typing sends no requests to Sutradhar,
the add-on's routes or the plugin's routes. The panel displays only the named
measure with evidence for the draft version. Coach tips still show
reasons and have no overall score. The panel does not change the draft
or publish anything.

### Saved runs

Each run is saved against the exact draft or page version and never
changed. The editor can compare two runs: tips gone, tips new, check
versions changed. Each tip says what would clear it. This reuses the
draft diff.

### Contradiction tip, with no fact list

Nothing is stored as the truth; there is no staff fact list.

1. The Coach finds a value in the draft or page: a price, a length of
   time, a version or a date.
2. It looks for the same value about the same subject on other staff
   pages of the site.
3. If they differ, it shows both passages side by side, with versions
   and dates. Staff decide which page to fix, or leave both (they may
   differ on purpose, such as two models).

- Neither page is called right or official. It never edits either page.
- One tip per pair of pages per value. A page edit triggers a recheck.
- A fixed list of value types. An unknown type gives no tip. It never
  guesses from recency or page importance.
- Member posts are never a side of the comparison and never mined.
- One site only. Nothing crosses to a paired site.
- Not included: a fact life cycle, named fact owners, "what was true in
  March" lookups, per-fact recheck times, serving facts to other
  installs.

### Paste and upload

An editor may paste text, or upload a file, from their own draft that is
not on the site yet.

- Own draft only (EC06). The notice asks the editor to confirm the text
  is theirs to share.
- Types: plain text, Markdown and Word. Others need an admitted reader.
  PDF is refused, with a note to paste, because its text comes out
  unreliably.
- Text only. Macros, links and embedded objects are never run or
  followed. Tracked changes are read as final text. Comments are ignored.
- A small separate Rust step reads the file under a size limit and an
  expanded-size limit, so a booby-trapped file cannot use up memory.
- The private-data check runs first, before any notice and any send.
- Kept for the editor alone, for `coach.draft.keep_days` or until they
  delete it. It reaches a route that leaves the PC only after the notice.
- The "page version" for EC01 is a draft version: a time and a
  fingerprint.
- Checks are read-only. "Did the change help" is not offered, as nothing
  is live.

### Related questions

The Coach lists questions readers ask near a topic and says which the
page answers. Sources are labelled and never mixed:

- Seen: Search Console queries phrased as questions, and the site's own
  search terms through the existing filtered setting.
- Suggested: from a wording route (below), marked "not seen".

Each question is answered here; answered by another page (link it,
through the normal link suggestion with moderator approval); or not
answered (a content gap). A link beats repeating an answer. If the
closest page already competes with this one for the same queries, the
tip stays quiet, because the cannibalisation advice owns that case.
Three kinds count: a follow-up, a broader and a narrower question.
Rephrasings, spelling variants and translations do not. The list is
cached by the draft's outline, not by the clock.

### Wording order, with a shown fallback

The owner sets an ordered list of wording routes. The default is the
owner's order (archive `signals/coach-additions-2.md`):

1. The owner's own AI agent (such as Claude, Codex or Gemini),
   connected through Sutradhar's MCP server (spec 0020). The Coach asks
   it for wording through a wording-route trait that the MCP server
   implements as an MCP sampling request on the owner's live
   `sutra mcp` session for the draft's own site; the composition root
   wires it in. It is used
   only while the owner has turned that route on (AGENTS.md section
   5.5) and such a session is open and allows sampling.
2. An exact-pinned packaged generative-model application on the owner's
   PC, behind a typed Rust-owned interface admitted under spec 0024.
3. The paid route (AWS Bedrock with the owner's key), only with a key
   and a spending cap.

Sutradhar writes no model runtime, custom model component or first-party
Python production code for the local route. The exact package,
generative model artefact, dependency hashes, quality evidence and
measured resource profile are readiness blockers. Until all are
admitted, the local entry is unavailable and the ordered walk skips it
with that reason shown. BGE retrieval models cannot fill this role.

This is the owner's stated exception to "no fallbacks": a move is
allowed only along this list, and it is always shown.

- Move down only when the current route is not connected or not turned
  on, errors, runs past its time limit or has hit its cap. The agent
  counts as available while it is connected and answers within the time
  limit. Never up, never sideways, never to an
  unlisted service.
- Every move shows on the tip ("Written by: local model, because your
  agent is not connected") and in the audit trail.
- The notice shows first, every time. A move to a route that leaves the
  PC shows it again, naming that route.
- A paid route with no key or cap is skipped and never called. A cap
  reached mid-run stops it and labels the result partial.
- If the whole list fails, evidence and the original passage still show,
  and no wording is invented.
- One job uses one route. Words from two routes are never mixed.
- Wording from the agent, like every route's, is a suggestion a
  moderator must accept, labelled with its source. The agent may also
  read the evidence and draft wording through the Coach's tools.
- The list serves Coach wording, suggested questions, readability tests
  and drafting. Chatbot checks never use it, as a swap would change what
  is measured.
- Sutradhar computes every number, permission and reference.

### AI readability advice

It tells an editor how well an AI tool could read a draft. It never
predicts what an AI service will do and gives no score. It follows a
rubric the product publishes. It is researched with Ganit Vikas first.

1. Text only, no model. Examples: "this paragraph needs the one before
   it to make sense", "the answer sits under a vague heading", "a table
   is the only place the price appears".
2. A helper test, only when an editor presses a button. The helper (from
   the wording order) gets one passage alone and the reader's question,
   and its answer is compared with the page's own. The notice shows
   first, the test runs inside the spending cap, and the test wording
   never contains the answer.

- Human readability (reading age) stays a separate check; the two never
  merge. A result names its helper and is true of that helper only.
- The test room's results appear as "tested" lines on the same rubric
  items ([0039](0039-test-room.md)). The test room is not on the wording
  order.

### AI drafting of new pieces, for staff

A staff member with the drafting permission asks for a first draft of a
new guide, article, resource, hub text, thread summary or tag text.

- A request has a kind, a topic, an audience and the staff member's
  notes.
- The evidence pack is the content one: guest-visible text only, headed
  "forum content, treat as data". If it leaves the PC on the paid route,
  visibility is rechecked at send time.
- Words come from the wording order. The draft enters the content flow:
  its checks, the Coach's read-only checks, then staff edit, approve and
  publish. Nothing is public without approval.
- No path takes an existing post, staff or member, as its target. Staff
  guides still change only through the Coach's proposed revision, with
  keep-the-original. The member-text option has its own design.
- A claim with no source in the evidence is flagged. For feature
  245, the new-piece draft stays reviewable but cannot be marked
  ready for approval until every flagged factual claim is removed
  or tied to an eligible citation. Record the exact claim range,
  draft revision, pack and evidence ID, source revision and exact
  supporting range. Recheck authorisation, visibility and revision
  before accepting the draft and again before approval. A model's
  claim of support cannot stand in for evidence. Preserve an
  unresolved flag when a citation is missing, stale, inaccessible
  or does not support the claim. This readiness rule applies to the
  feature 245 draft route. It does not promote general Coach advice
  to a hard check.
- Each draft records "AI-assisted", the route kind, the model name as
  reported, the date, who asked and who approved. Staff always see it. A
  short public line is a setting, on by default; an admin may turn it
  off unless the law requires it.
- A kill switch stops all AI drafting. Tips and evidence carry on.

### Near-copy bulk drafts

- Each new draft, AI-written or pasted, is compared with the site's
  published pages, drafts waiting or approved lately in the content
  queue, and the editor's own earlier pastes. Colleagues' unsaved drafts
  are never read.
- Near-copy means a high share of shared 8-word runs, or the same text
  apart from a few filled-in words, such as a city name.
- The warning shows the pairs, the count and one example, and asks what
  each piece adds. Approval stays possible. It never blocks.
- The daily limit caps AI drafts started per site per day. At the limit
  a new AI draft request waits for the next day and the screen says so.
  A person's own writing and publishing is never limited.

Example: twenty near-identical city pages in an hour get a warning, and
the daily limit holds further requests until tomorrow.

### Wave 2 saved scores, overlap and cited drafts

Use synthetic saved revisions, evidence and stand-ins for admitted
native or wording routes. These fixtures require the existing
platform, measure, threshold, rights and resource admission. A fixture
or a named requirement creates no authority to score, send or publish.

| Feature and requirement | Design and fixture | Evidence to require |
|---|---|---|
| F-0235 · REQ-0042-W2-SAVED-EDITOR-SCORE | DES-0042-W2-PAIRED-STORED-SCORE · FX-0042-W2-PAIRED-STORED-SCORE · EA13 | Require the existing single 0056-admitted measure, named measure and rubric version, and the exact eligible saved revision. Use the same admitted content fixture independently on XenForo and WordPress. Each editor reads its own paired 0021 stored result. On unsaved changes show the result as older without a request from typing. Hold an unsupported native profile, and never score conversation or private text. Leave draft, publication and the ordinary Coach card unchanged. |
| F-0241 · REQ-0042-W2-NEAR-COPY-ADVICE | DES-0042-W2-ELIGIBLE-OVERLAP · FX-0042-W2-ELIGIBLE-OVERLAP · EB13 and EB14 | With a synthetic admitted share and count, compare high and low shared eight-word-run cases plus a filled-slot template. Include an eligible published or saved queue source and the editor's earlier paste. Exclude colleagues' unsaved drafts and withdrawn sources. Show pairs, count and a permitted example for an admitted match without claiming intent or blocking approval. No warning with an unadmitted profile. Spec 0052 alone governs its separate novelty and second-review route. |
| F-0245 · REQ-0042-W2-CITED-NEW-PIECE | DES-0042-W2-EXACT-CLAIM-REVIEW · FX-0042-W2-EXACT-CLAIM-REVIEW · EB09–EB12 and EB19 | Use the existing drafting permission and a new-piece request. Exclude private sources from the 0052 guest-visible pack. Simulate a missing, mismatched, expired or revoked citation and a lost send grant. Require the exact draft revision, claim and supporting ranges and live scope for readiness. A valid citation restores only its own claim support and still needs staff review. Refuse an existing-post target. Require the staff-visible drafting provenance. No unapproved egress or publication. |

### Chatbot evidence in version 1

DR-086 limits this route to files supplied by the owner under
[spec 0093](0093-ai-search-visibility.md). This spec owns the single
site-scoped answer archive and brand mention rate (298 and 288).
Keep the accepted answer and its manifest in the archive. Render it
as text. For each comparable group in 0093, divide the count of
valid answers that name the site by the count of valid answers in
the group. Show both counts and the supplied-sample label. Keep
groups separate. The shared alias matcher reports ambiguity separately.
Apply 0093's minimum, retention and erasure rules. A granted
`compare_edit_outcomes` call gets approved count-only summaries.

#### Wave 2 supplied-answer archive and rate verification

Spec 0042 owns the archive, mention rate and the two deferred routes.
Spec 0093 owns the admitted import and comparable cohort contracts.
Use its exact schema, permissions, immutable prompt and observation
provenance, source coverage and release-safe disclosure profile.
0093's brand comparisons use the existing 0042 rate by reference.

| Requirement | Owning design and applicable variant |
| --- | --- |
| REQ-0042-ANSWER-SCOPE | DES-0042-ANSWER-REFUSAL preserves the DR-086 version 1 refusal of live answer runs and their spend actions. Both deferred rows have refusal cases only. |
| REQ-0042-ANSWER-RATE | DES-0042-ANSWER-RATE and VAR-0042-SUPPLIED-MENTION-RATE count one admitted comparable group over its unique valid observations. The denominator includes ambiguous valid answers. Preserve the three separate counts, coverage, minimum and supplied-sample label. |
| REQ-0042-ANSWER-ARCHIVE | DES-0042-ANSWER-ARCHIVE and VAR-0042-SUPPLIED-ANSWER-ARCHIVE use one site archive for approved plain files with the exact 0093 manifest. Pin site, sample, observation and capture evidence. Use 0032 erasure and 0118 protected custody through replay and restore. |

| Feature | Requirement, design and variant | Planned test and exact fixture assertion |
| --- | --- | --- |
| F-0287 | REQ-0042-ANSWER-SCOPE, DES-0042-ANSWER-REFUSAL | E2E-0042-ANSWER-RUN-REFUSAL, FX-0042-W2-LIVE-ANSWER-REFUSAL. Attempt the historical Run AI check action with an owner session, a restored provider configuration and an old key or approval. Replay the old run after restart. No live answer action, key lookup, outbound dispatch, charge or archive observation is created. |
| F-0288 | REQ-0042-ANSWER-RATE, DES-0042-ANSWER-RATE, VAR-0042-SUPPLIED-MENTION-RATE | UT-0042-ANSWER-RATE, FX-0042-W2-SUPPLIED-MENTION-RATE. Use one synthetic comparable group with five distinct accepted valid answers: two confirmed hits, two confirmed no-hits and one ambiguous. Add a same-observation replay and one failed and one missing observation. Under an admitted synthetic minimum of five, show 2/5, all three disjoint counts and separate coverage. Below that minimum suppress the rate. No pooled service, personal inference or search ranking input. |
| F-0298 | REQ-0042-ANSWER-ARCHIVE, DES-0042-ANSWER-ARCHIVE, VAR-0042-SUPPLIED-ANSWER-ARCHIVE | IT-0042-ANSWER-ARCHIVE, FX-0042-W2-SUPPLIED-ANSWER-ARCHIVE. Accept one admitted owner-supplied plain answer at the exact site, sample and observation identity. Replay identical input once, refuse a changed payload at that identity and reject the same capture rekeyed. A separately proved capture with identical text may add one observation. Check acceptance-time retention, erased and wrong-site reads, expired source and stale restore. Answer text stays escaped and never enters model input or MCP. |
| F-0299 | REQ-0042-ANSWER-SCOPE, DES-0042-ANSWER-REFUSAL | E2E-0042-ANSWER-COST-REFUSAL, FX-0042-W2-ANSWER-COST-REFUSAL. With an owner session, load an old answer cost action, billable cap or restored budget job. Retry after restart using a separately valid live Coach spend approval. No answer-cost estimate, key lookup, new tracking ledger, reservation or charge follows. Keep the other route under its own authority. |

Spec 0117 owns local fan-out practice. Spec 0133 and DR-118 own
the separate approved and labelled live Coach evidence and spend
route. Practice or live evidence does not enter this answer archive,
rate or tracking-cost route by assignment. These feature bindings
supply no provider authority. ED01–ED12 remain after-version-1 design
history.

QR-065 and QR-131 still require exact source, privacy, cohort,
retention and fixture qualification. Check existing actor/site scope,
current source rights and erasure before releasing results or a
count-only Coach summary. The named synthetic assertions specify
expected evidence under 0076. Observed test and admission records
follow their separate owning checks.

### Paid chatbot design after version 1

Live answer checks and tracking costs (287 and 299) are outside
version 1 under DR-086. This later design, its run settings and tests
ED01–ED12 are retained as design history. They cannot open a run,
take a key or spend money in version 1.

An admin presses "Run AI check". Approved questions go to chosen outside
AI services; the screen shows, per service, which answers named the
site, with every answer kept in full.

- Only an admin starts a run, in the desktop app. MCP and agents may
  read results but cannot start one. There is no schedule.
- Before the run the screen shows services, questions, repeats and an
  estimated cost. A run whose estimate passes the cap is refused before
  any call. Each service has its own key and run cap; the month cap
  (`coach.ai.spend_cap_month`) covers all. A service with no key or cap
  is never called.
- An admin approves the question list before its first run and after
  any change. Questions come from related questions, Search Console
  question queries or an editor, and pass the private-data check.
- Only the question leaves: no page text, member data or site name.
- A failed or capped service stops its own remaining questions and is
  marked incomplete. Its questions never move to another service or to
  the wording order.
- Kept: the whole answer, with question, service, model name as
  reported, date, run and any sources given, for `coach.chatbot.keep_days`.
  Answers are untrusted text, shown escaped, never read as instructions.
  If a cited source is gone, the screen warns; the kept copy stays.
- Other sites named in an answer stay inside it; they are never
  extracted, ranked or tracked.
- Sutradhar, not a model, checks whether the site's name or addresses
  appear ("named in 3 of 5 repeats for this question"). Results are per
  service, never added across services, never one score. Too few
  questions or repeats gives "not enough evidence". A rise after an edit
  is a link, not proof.
- The label reads "API answer; may differ from what readers see". These
  answers are one labelled metric source, never mixed with another.
  Reading AI crawlers in logs is separate and not added.
- Not included: a "test it yourself" link; scoring then discarding the
  answer; "citation probability"; any claim about what a service does
  behind the scenes.

### Reader patterns for hubs

darshak already counts which pages a visit opens and in what order. For
a pair of pages, the rate is the share of visits to A that go on to B,
with the visit count beside it.

- The pair rate is one more edge in the graph behind ganit job 278, the
  single topic definition that hubs use. It can nudge which topics group
  and which become hub candidates, with the admitted reader-pattern weight.
  That weight is blocked as the settings table states, so this edge stays off
  until admitted.
- It cannot create a hub alone, waive the rule of at least 10
  guest-visible threads, skip the relevance gate or link approval, or
  reorder hub links.
- Only human visits count; crawlers and automation never do.
- Consenting visitors count, and so do pairs inside one anonymous visit.
  No link across visits or to a person. With no data, "not measured".
- Pairs below the minimum visit count give nothing. Individual paths
  never leave darshak; only the pair rate and count reach ganit.
- Staff see one reason line: "readers often go from A to B (rate,
  visits)". Each result is inert candidate evidence; spec 0077 alone evaluates,
  promotes, activates and rolls it back. A late or failed candidate leaves the
  verified champion unchanged. If the evidence shows no gain beyond the link
  and meaning graph, the candidate is not promoted and this edge stays off.

### In-app assistant evidence and review (608–617)

Turn a plain-English request into a typed local intent:
answer, chart explanation, report preview, plan or simulator
preview. Validate site, actor,
admitted metric and operator, unit, exact projection, interval,
action family and current grant. Bound request to 4 KiB, 100
rows and a two-second read, with stricter owning limits. Ask a
local question for ambiguity. Missing or sparse evidence stays
unknown. Refuse arbitrary SQL, network, provider or file paths. Rules compute
every number and query. A model can propose wording or the
typed intent only. Untrusted prompt, chart or source content
cannot grant an effect or expand the projection.

Use admitted 0040, 0043, 0048, 0094 and 0100 read projections.
A chart explanation pins the exact chart filters, axis unit,
frozen source version, window, coverage and uncertainty.
Bind each exact answer field or output span and revision to its
typed approved value, unit, source receipt and projection version,
scope, window, cutoff, derivation and uncertainty. Claim types
are measured, estimate, association or unknown. Use only registered
methods and joins. A citation alone cannot validate the wording.
Validate exact structured claims or render from the trusted
data. Withhold unproved evidence, cause, values and spans.
Recheck source grants, consent and erasure at generation, view,
cite and copy. Apply joint disclosure across adjacent claims.
No raw 0034 details, 0103 sales or private excerpts.
A report preview is an admitted 0094 master projection.

Daily briefing is off until the owner approves an exact site,
local date slot, zone, zone database and 0019 schedule revision.
Resolve clock changes under 0019 and skip missed dates.
Commit one protected result and receipt per site-day with
schedule generation, source revisions and wording mode. Name
current changes, eligible priorities and gaps. Pause, revoke or
expired slots hold publication. Keep a valid historical briefing
dated as observed; erase unauthorised content. The local
receipt and result create no site, delivery or action effect.

The planner can suggest existing authorised 0048 items or make
an inert proposed task with exact evidence, effort and prerequisites.
The what-if view uses 0027 feature 1's exact SIM-01 to SIM-05
contract for the frozen graph, map and page changes.
A single-suggestion estimate or a settings preview cannot replace
it. Until that exact profile is admitted show unavailable.
Show assumptions, bounds and sensitivity through a checked
owner-local summary with deterministic wording. Recheck site and
visibility at display. No graph or native identity enters a model.
No extrapolated sum, live ranking, traffic or causal claim. Neither
task wording nor simulation alters the site or ranking inputs.

Advisory memory permits owner-confirmed goal IDs, briefing
sections from the local catalogue, and requested day counts from
1 to 90 within source caps. Store site, type, purpose, source,
effective time, expiry and generation. Use 0020 owner-only
review and versioned edit or deletion with controlled history.
No learned role or setting under 0014. Recheck generations at
every read. 0032/0118 erasure and restore fence caches, plans
and copies. Memory changes no scope, ranking or other site.

Feature 616 and local briefing use a pre-provisioned admitted
0024 language package under 0080/0078 leases. The offline worker
has an operating-system outbound network block. Only approved
local IPC is available. No MCP sampling, Bedrock, provider or
download/update route even when configured. If it cannot run, show the exact
allowed data,
sources and a plain reason. This mode does not replace the
separately authorised wording order above. A daily task needs
the same local model and resource checks for generated wording.

Chat can prepare only an admitted proposal family. Bind plan
digest, site, shown actor, target and scope revisions, operation,
source and setting generations and expiry. Show the exact effect.
Use 0020 live owner confirmation where required, and the native
0028 moderator and subject rules. MCP, model and script cannot
confirm. Consume the approval, effect ID and 0019 outbox once.
The owning gate rechecks current conditions and uses 0021 for
native effects. A lost reply follows that route's exact recovery.
Unknown stays held. A model cannot publish, edit, spend or
change access. No replay of an old approval.

#### Named planning, claim and effect-review fixtures

These planned spec 0076 cases use synthetic data and approvals.
QR-152 holds typed read and native effect proof. QR-153 owns
the exact simulator admission. Named EA and EB cases and the
owning 0020/0027 tests retain their current boundaries.

`DES-0042-W2-ASSISTANT-REVIEW` uses the typed planner, advisory
memory and admitted effect preview. `DES-0042-W2-SIMULATOR-PREVIEW`
reads the exact F-0001 result under the current grant.
`DES-0042-W2-CLAIM-EVIDENCE` validates exact wording against
its approved proof. `DES-0042-W2-LOCAL-WORDING` uses the
isolated offline route only. Models, memory and summaries confer
no authority to assign, publish, call providers or change rank.

| Feature | Requirement | Design and variant | Fixture and existing test boundary | Named fixture assertion |
|---|---|---|---|---|
| F-0612 | REQ-0042-PLANNED-TASK | DES-0042-W2-ASSISTANT-REVIEW, VAR-0042-W2-INERT-TASK | FX-0042-W2-TASK-PLAN, EA01, EA07 | Propose one authorised existing item and one inert draft task. Show each admitted prerequisite and effort, and unknown cost separately. Changed evidence holds review and neither task is assigned or applied. |
| F-0613 | REQ-0042-BATCH-WHAT-IF | DES-0042-W2-SIMULATOR-PREVIEW, VAR-0042-W2-FROZEN-0027-SUMMARY | FX-0042-W2-BATCH-WHAT-IF, UT-SIM-01, IT-SIM-02, BDD-SIM-03, FI-SIM-04, E2E-SIM-05 | Read one exact current 0027 three-node result as a local summary. Reject an altered proposal, missing graph or stale receipt. No substitute single-edge estimate, new simulated row, live rank or graph change. |
| F-0614 | REQ-0042-ADVISORY-MEMORY | DES-0042-W2-ASSISTANT-REVIEW, VAR-0042-W2-SCOPED-MEMORY | FX-0042-W2-ADVISORY-MEMORY, EA04 and 0020 test 4 | Save one typed advisory value using current owner approval. Expire, erase and restore it. Current generation holds old copies. A prompt or foreign site cannot infer a preference or grant a setting change. |
| F-0615 | REQ-0042-VERIFIED-CLAIM | DES-0042-W2-CLAIM-EVIDENCE, VAR-0042-W2-TYPED-RECEIPT | FX-0042-W2-VERIFIED-CLAIM, EA01, EA07, EB19 | Use a true typed claim paired with a receipt. Change its value or unit, supply a plausible wrong citation and attempt a revealing derivation. The unsupported claim is withheld, with no sensitive excerpt or causal inference. |
| F-0616 | REQ-0042-OFFLINE-WORDING | DES-0042-W2-LOCAL-WORDING, VAR-0042-W2-OFFLINE-ADMITTED-MODEL | FX-0042-W2-OFFLINE-WORDING, EB01, EB05 | Use the admitted provisioned model with outbound network blocked. Remove its package or lease. Show approved local evidence and a reason. Configured MCP, Bedrock and download routes remain unreachable in this mode. |
| F-0617 | REQ-0042-PROPOSAL-REVIEW | DES-0042-W2-ASSISTANT-REVIEW, VAR-0042-W2-OWNING-EFFECT-GATE | FX-0042-W2-PROPOSAL-REVIEW, EB09, EB17 and 0020 tests 2–4 | Pin the exact plan, actor, target revision, scope and expiry. Deny changed target, revoked moderator and token or MCP confirmation. Reconcile a lost synthetic native reply under its original ID. Completed and unknown effects do not repeat. |

### Isolation

Each site or agency client keeps its own runs, drafts and rates. Nothing
is shared between sites or installs.

### Wave 2 typed assistant reads (608–611)

Use synthetic typed projections, clocks and receipts. QR-152
retains exact claim, privacy, offline wording and effect proof.
Source owners compute the values. A typed read, local wording or
preview grants no raw query, file, paid action or network route.

| Feature | Owning requirement and design | Planned fixture and expected evidence |
|---|---|---|
| F-0608 | REQ-0042-W2-SCOPED-DATA-ANSWER · DES-0042-W2-VALIDATED-TYPED-READ | FX-0042-W2-SCOPED-DATA-ANSWER · IT-0042-W2-SCOPED-DATA-ANSWER. Use an exact eligible measure, unit, source, window and projection from the admitted owner. Offer ambiguous sources, a protected count, foreign data and source text that requests an effect. Return verified values and receipts or a local clarification. Hold unavailable, sparse or unauthorised inputs. Reuse 0100 disclosure and read checks without widening its scope. |
| F-0609 | REQ-0042-W2-OFFLINE-DAILY-BRIEF · DES-0042-W2-ONE-LOCAL-BRIEF-SLOT | FX-0042-W2-OFFLINE-DAILY-BRIEF · IT-0042-W2-OFFLINE-DAILY-BRIEF. Start disabled and admit one exact owner-approved site, date slot, zone and 0019 schedule revision. Replay after restart, miss a date, revoke a source before release and remove the local model. Check one current safe site-day receipt, no late catch-up and the permitted evidence-only display when wording is unavailable. No paid, network or send action. |
| F-0610 | REQ-0042-W2-FROZEN-CHART-EXPLANATION · DES-0042-W2-VALIDATED-TYPED-READ | FX-0042-W2-FROZEN-CHART-EXPLANATION · BDD-0042-W2-FROZEN-CHART-EXPLANATION. Explain an authorised frozen 0043 chart with a missing row and unlike units, then change live filters. Use exact displayed axes, filters, source versions and interval with current rights, typed evidence receipts and uncertainty. Missing stays unknown. No cross-provider blend, live refresh or causal inference. |
| F-0611 | REQ-0042-W2-TYPED-REPORT-PREVIEW · DES-0042-W2-VALIDATED-TYPED-READ | FX-0042-W2-TYPED-REPORT-PREVIEW · BDD-0042-W2-TYPED-REPORT-PREVIEW. Resolve a supported plain-English request to its exact eligible 0094 master projection. Try a raw query, wrong site, changed or revoked source and an implied export. Show authorised typed preview values and scope only. Refuse stale or unsupported evidence. Preview creates no export, send, remote read or file authority. |

### Settings

| Setting | Default | Allowed range |
| --- | --- | --- |
| `coach.checks.<name>.enabled` | on after blind test | on or off |
| `coach.answer_early.window` | blocked: no admitted value | above zero, safe top |
| `coach.fact_density.floor` | blocked: no admitted value | above zero, safe top |
| `coach.related.max_per_page` | blocked: no admitted value | at least 1, safe top |
| `coach.upload.max_bytes` | blocked: no admitted value | above zero, safe top |
| `coach.upload.types` | text, Markdown, Word | may only be narrowed |
| `coach.draft.keep_days` | blocked: no admitted value | at least 1, safe top |
| `coach.ai.order` | owner's agent, local, paid | reorder or remove entries |
| `coach.ai.spend_cap_day` | none, so paid stays off | above zero, safe top |
| `coach.ai.spend_cap_month` | none, so paid stays off | above zero, safe top |
| `coach.ai.reply_seconds` | blocked: no admitted value | above zero, safe top |
| `coach.ai_read.repeats` | blocked: no admitted value | at least 1, safe top |
| `content.ai_drafting.enabled` | on | on or off |
| `content.ai_label.public` | on | on or off |
| `content.ai_drafts_per_day` | blocked: no admitted value | above zero, safe top |
| `content.near_copy.share` | blocked: no admitted value | above zero, below whole |
| `content.near_copy.count` | blocked: no admitted value | at least 2, safe top |
| `coach.chatbot.services` (after version 1) | none until key and cap | per service |
| `coach.chatbot.cap_run` (after version 1) | none, so a run is refused | above zero, safe top |
| `coach.chatbot.questions_max` (after version 1) | blocked: no admitted value | at least 1, safe top |
| `coach.chatbot.repeats` (after version 1) | blocked: no admitted value | at least 1, safe top |
| `coach.chatbot.keep_days` | proposed in spec 0093 | spec 0093 |
| `hubs.reader_pattern.enabled` | on | on or off |
| `hubs.reader_pattern.weight` | blocked: no admitted value | above zero, safe top |
| `hubs.reader_pattern.min_visits` | blocked: no admitted value | above zero |
| `hubs.reader_pattern.window_days` | blocked: no admitted value | 1 to 90 |

Every value marked "blocked" is a readiness blocker owned by this feature's
Ganit Vikas research record. That record must supply the value, safe bounds,
fixtures, supporting evidence and approval before the setting or any
behaviour depending on it can build or activate. There is no implicit
default; a missing reader-pattern value leaves that optional edge off, and
a missing safety, privacy or resource value refuses the affected operation.

### Build-blocking research

Ganit Vikas answers: which value types the contradiction tip finds
reliably and its false-alarm rate on real staff pages; how many related
questions help rather than flood, how to merge near-identical ones, how
much data "seen" needs; which text signs relate to a helper answering
correctly, repeats for a stable result, the cost of a test, whether
local and paid helpers agree, and tip wording that claims no more than
it shows; the share and count that mark a near-copy, how to tell a
deliberate series from empty pages, and a safe daily limit; the
reader-pattern weight, minimum visits and window under 0034 consent,
retention and release rules. DR-086 defers general paid chatbot research.

### Changes to other parts

- Coach: `check_draft` gains pasted and uploaded input and the new
  checks; `prepare_writing_brief` gains related questions;
  `compare_edit_outcomes` gains chatbot results, read only; the blind
  test covers the new tips. The five tools and rejected list stand.
- Content drafts: gain the contradiction tip, the AI label, the
  near-copy warning and the daily limit; the "calls no language model"
  rule changes for the drafting route. Content tools start on; the Coach
  stays off until its blind test passes.
- Hubs: topic candidates gain the reader-pattern edge.
- darshak ranking inputs gain a pair rate, for hubs only. The registry
  still refuses plain counts.

### Logic and maths

Decisions are Rust, proved with Kani where they decide: walking the
list, cap checks, the daily limit, near-copy rules, question approval,
name matching and pair suppression. Maths, stability and the graph
weight are in ganit. The add-ons call no AI service, send nothing and
do no work per keystroke.

## Limits

- Within AGENTS.md 5.6: the local model and all Coach jobs count inside
  the service slice; screens run in the window's cap.
- Uploads are capped by `coach.upload.max_bytes` and an expanded-size
  limit, read in a separate bounded step.
- Spending caps per day and month for Coach wording routes. No cap
  means no paid call. Chatbot spend is after version 1.
- The daily limit on AI drafts per site.
- Visits are kept 90 days; the reader-pattern window is 1 to 90 days.
- Legal labelling rules for AI text, and service terms and law on
  automated asking, differ by country and are checked before the build.

## Footprint budget

Set when the feature is built. The optional public "AI-assisted" line on
published drafts is the only public-page addition.

## How it is tested

Rust tests for each point:

- EA01: every new tip names its check, check version, page or draft
  version, source and date, or it is not shown.
- EA02: no new check blocks a draft; only approved hard checks do.
- EA03: no waiver screen, field or setting exists.
- EA04: a saved run never changes; comparing two runs shows what moved.
- EA05: two staff pages with different prices give one tip with both
  passages.
- EA06: a member post is never a side of a contradiction tip.
- EA07: an unknown value type gives no tip and no guess.
- EA08: pasted text from another person's draft never leaves the PC.
- EA09: a booby-trapped or oversized file is refused with the reason;
  macros and embedded objects are never run or followed.
- EA10: "seen" and "suggested" questions never share one count.
- EA11: where a competing page owns the case, the related-question tip
  is quiet.
- EA12: phone and desktop previews say "not measured" until admitted.
- EA13: both editors show the same stored score for the same saved
  version and measure version, and mark it as older once the text on
  screen changes. Typing sends no requests to Sutradhar, the add-on's
  routes or the plugin's routes and changes nothing on the site. A
  conversation draft is never scored.
- EB01: the list is walked in order and never reaches an unlisted
  service.
- EB02: every move shows on the tip and in the audit trail.
- EB03: a move to a route that leaves the PC shows the notice first.
- EB04: a paid route with no key or cap is never called; a cap reached
  mid-run stops it and labels the result partial.
- EB05: with every route failing, evidence shows and no wording is
  invented.
- EB06: no job mixes words from two routes.
- EB07: a readability test sends one passage, shows the notice, hides
  the answer from the test wording and names the helper.
- EB08: the text-only readability layer works with no model.
- EB09: AI drafting has no path to any existing post.
- EB10: each draft carries the label, route, model, date, asker and
  approver.
- EB11: nothing from AI drafting is public without a staff approval.
- EB12: a seeded private or staff-only post never reaches a pack.
- EB13: 20 near-copy drafts give a warning with pairs, and approval
  still works.
- EB14: a colleague's unsaved draft is never read by the near-copy
  check.
- EB15: at the daily limit, AI drafting waits and human publishing is
  untouched.
- EB16: the kill switch stops drafting; tips and evidence continue.
- EB17: an MCP-drafted wording is stored only as a suggestion and is
  applied only after a moderator accepts it.
- EB18: with the owner's agent route turned off, disconnected or past
  its time limit, the job moves to the local model, the notice names
  the reason, and no paid call is made while the local model answers.
- EB19: a feature 245 draft with a missing, mismatched, revoked
  or stale claim source stays outside ready-for-approval status.
  Submit exact current supporting ranges and obtain staff review.
  A receipt or quote from another site, a private source or a
  replay after scope withdrawal gives no approval. A route needing
  egress must recheck exact consent on send. The older feature 245
  claim-source use of EB18 refers to EB19.
- After version 1 only, ED01–ED12 apply to the paid chatbot design.
- ED01: only an admin in the desktop app starts a run; MCP can read,
  not start.
- ED02: a run whose estimate passes the cap is refused before any call.
- ED03: a service with no key or cap is never called.
- ED04: a capped service stops its questions; no other service is used.
- ED05: only question text is sent: no page text, member data or site
  name.
- ED06: an unapproved or changed question list cannot run.
- ED07: each answer is kept whole with question, service, model, date
  and sources.
- ED08: a vanished cited source shows the warning; the kept copy stays.
- ED09: results are never added across services, and never one score.
- ED10: too few repeats give "not enough evidence".
- ED11: other sites named in an answer are not extracted or tracked.
- ED12: answers carry the "may differ from what readers see" label.
- ED13: only human visits count; anonymous pairs stay inside one visit.
- ED14: a pair under the minimum visits gives nothing; no data shows
  "not measured".
- ED15: reader patterns alone never create a hub, break the 10-thread
  rule or reorder hub links.
- ED16: an individual visitor's path never leaves darshak.
- ED17: one client's runs and rates never appear for another client.

## Changes from the source

- The wording order keeps the owner's three routes. The staff member's
  own agent becomes the owner's agent, connected through the owner-only
  MCP server and used only when the owner turns that route on; the local
  route is an admitted packaged generative-model application behind
  typed Rust integration; the paid route is AWS Bedrock with the owner's
  key (AGENTS.md section 5.5).
- Sutradhar implements no local model runtime or first-party Python
  production integration. The local route remains unavailable until its
  package, artefact, quality and resource admission blockers close.
- Decision logic, maths and adapters are rewritten as Rust (ganit for
  maths), proved with Kani where they decide.
- Admin pages become screens in the desktop app built from SDS.
- Memory follows the limits in AGENTS.md section 5.6.

Source: `docs/specifications/signals/coach-additions.md`,
`docs/specifications/signals/coach-additions-2.md` and
`docs/specifications/signals/coach-additions-3.md` in the archive
`archive-2026-11-08-old-repo`.
