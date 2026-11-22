# 0046: Web link study

The link study answers one question for a site owner: how good are the
links to my site, and how sure are we? It reads link records and
permitted reports that the owner supplies. The study fetches nothing.
Every number says what kind it is. The screen label is "Link study"; the
module is `subramanian`. It is part of roadmap Phase 6; "Which spec
designs each feature" in [features.md](features.md) lists the features
it designs in whole or in part.

Example: "Service A reports 412 pages linking here (service figure, read
2 Nov). Our crawl has checked 57. 3 links: not seen since 14 Oct."

## What it does

### Ground rules

1. It reads and reports evidence and holds local review records. It
   never changes a site link, moderator decision or member text.
2. Staff-only screens in the desktop app; guests see nothing new.
3. Each site and each agency client keeps its own records. Nothing is
   shared between installs.
4. It starts on. With no source it shows "not measured", never zero.
5. The study opens no network connection. Only spec 0045 may
   check approved leads under 0082 and the admitted 0152 profile.
   The wider [0153 index](0153-public-web-backlink-index.md) has
   separate scope. Its [0154 slices](0154-backlink-index-site-slices.md)
   can be a distinct observed source only under their own admission.

### What it reuses

| Part | Reuses |
| --- | --- |
| Link records, leads, block pages | Backlink crawl (spec 0045) |
| "Not seen since", "not measured", no mixing | Backlink crawl rules |
| File validation and private staging | Bulk import (spec 0100) |
| Own page facts; visit totals | Crawler (spec 0044); darshak (spec 0034) |
| Before and after tests | ganit jobs 298, 299, 308; Coach items 20, 21 |
| Survival of links | ganit jobs 309, 310; feedback learning (spec 0053) |
| "Not enough evidence" wording | Editorial Coach (spec 0041) |
| Research; settings | Ganit Vikas (spec 0012); settings (spec 0014) |

New in this part: one label rule and a refusal to compare unlike
numbers; link states with a reason; six reports; a "needs more evidence"
box; source-labelled file intake and made-up test worlds.

Sources: crawl records, permitted owner-supplied service reports,
Search Console and Bing link exports, and pasted page addresses. The
legal and privacy record admits fields, retention and deletion for
each file source before import. It adds no live provider or provider
key for link data (DR-076). Public link maps are leads under spec
0045 and ADR-0022.

### Labels on every number

Each figure carries a kind, source, date, sample size and range.

| Kind | Meaning |
| --- | --- |
| Seen | Our crawl read the page and found the link |
| Own data | The owner's own Search Console, GA4 or darshak |
| Service figure | A supplied service figure, with source and date |
| Modelled | Our calculation, with a range and its inputs |
| Not measured | No source, or the data cannot support it |

1. Kinds and sources are never added together. Two services sit side by
   side.
2. Numbers compare only if source, dates and settings match. Otherwise
   the screen says "do not compare", gives the reason and shows no
   difference.
3. Other answers: "not enough evidence", "out of date", "sources
   disagree". No vendor score is copied or implied.
4. Take the source name and date from the admitted file's evidence
   record. Show required credits.
5. Supplied service figures stay labelled and never feed
   Sutradhar's modelled figures.

### Link states

A link is never called lost, removed or "probably gone".

| State | Shown when |
| --- | --- |
| Seen | The last read found the link |
| Not seen since (date), page read | Page read, link absent |
| Not seen since (date), not read | Page could not be read |
| Page gone | 404 or 410: the page went, not the link |
| Back again | Seen after a confirmed absence or a verified gone source page. Keep both dates |

1. Timeouts, 429, 5xx, DNS errors, robots denial, 401 to 403, block pages
   and a paused crawl never change "seen". They show "due for recheck".
2. Absence from a comparable later file means "not listed in this
   file". It does not establish that the provider dropped the link.
3. Dates kept: first seen, last seen, last checked, last read, and the
   reason a link was not seen.
4. Rechecks for non-rivals use the backlink crawl's lead list (kind 3). The
   deterministic ordering by change rate and link value is a readiness
   blocker owned by the link-study evaluation record; rechecks cannot run
   until its formula, tie-breaks and fixtures are admitted.

### The six reports

1. Links to my pages: new links, not seen since, referring sites, link
   words as kept, and growth. Growth marks each date the checking
   changed, so more looking is not shown as more links.
2. Links to dead pages, so a redirect can be set. Advice only.
3. Link value: visits from each linking site, from darshak totals and
   imported visitor numbers. Too few visits shows "too few".
4. Survival: how long inserted links lasted (ganit job 310). Outside
   links get a range only after enough history; otherwise "not enough
   history".
5. Before and after: matched untouched pages beside changed ones (ganit
   jobs 298, 299, 308). If they moved alike: "correlation, not cause".
6. Rival overlap: compare compatible supplied lists for the named
   rival and the owner's site. Report links present only in the
   rival list, with each file's coverage. This is a reading list.
   A rival lead can be checked once if spec 0082 admits the read.

#### Supplied rival overlap (264)

Compare two permitted owner-supplied lists for the active site
and a named rival, with matching provider, observation window,
URL scope, measurement profile and declared coverage. Require
admitted destination scope for each file, using the active site's
authoritative identity and the exact rival registry version.
Reject rows or a manifest mapped to another target. Match by the
same admitted referring-page identity on both lists. A referring
page contributes one presence per list, regardless of its link
row count. Keep original link evidence with stable link row IDs
and destination references. An identical row replay collapses.
Treat a conflicting replay as invalid evidence. Show both, own list only, and
rival list only, with the compared counts and each list's
coverage. “Rival list only” means this file pair, with no claim
about the web. For comparable partial lists, label each group as
observed within the supplied records. Neither file absence nor
partial coverage gives a claim about links outside the sample.
Omit the comparison when rights, target identity or scope fail.

REQ-0046-RIVAL-OVERLAP requires this comparison and its source
evidence in the local Link study view. IT-0046-RIVAL-OVERLAP uses synthetic
duplicates, two observed destinations from one page, distinct
pages on one domain, wrong destination, replay collision,
mismatched projections, partial lists and withdrawal before display,
replay and restore. A report request reads the admitted records only
and opens no connection.
QR-129 owns exact file profile and comparison evidence.

| Requirement | Verification contract |
|---|---|
| REQ-0046-RIVAL-OVERLAP | IT-0046-RIVAL-OVERLAP: use synthetic duplicates, two observed destinations from one page, distinct pages on one domain, wrong destination, replay collision, mismatched projections, partial lists and withdrawal before display, replay and restore. A report request reads the admitted records only and opens no connection |

### Backlink review and digital PR (270–284)

Show one site, source and evidence window on each page. Import
from permitted owner-supplied files under 0100. Each import
binds source and report subtype, declared property and evidence
of its selection, file hash, observed interval, coverage and
source row identity. If a file omits its property, verify a
matching owner-supplied export receipt under the source profile
admitted by QR-130. For Bing, that evidence must bind the exact
file digest, provider, report subtype and page, domain or anchor
level, observed interval and selected native property. The owner
picks the file in 0100's existing safe picker. Validate the digest
of the exact opened bytes under the current site and property
mapping. A matching file name, row count or typed property label
does not establish provenance. The receipt supplies no separate
rights or coverage. If the Bing match is unproved, leave the prior
eligible view unchanged. Say “This file could not be matched to the
selected Bing property.” Do not fetch proof, reopen a replacement
file by its path or merge report levels to fill a gap. Replay of
an admitted import retains its original dated source evidence.
Show observed or discovery time separate from acceptance time.
Preview exact accepted and rejected counts before approval. This
route has no upload, provider key or live provider connection. Reuse the
[Bing source guide][bing]
and [Search Console report guide][gsc-links] at admission.
A dated link report cannot establish a current direct observation,
an unreported position or relation. Documented unknown sentinels
stay unknown, even when encoded as zero. The study never asks
the crawler for an unapproved page.

| Feature | Evidence and result |
|---|---|
| 270 Bing inbound links | Show the owner's approved Bing Webmaster Tools link export for the exact mapped property. Support admitted page, domain and anchor report levels. Preserve each sampled source unit and its coverage. With no admitted inbound links export profile, show unavailable. No new 0040 call |
| 271 Search Console links import | Use the owner-supplied Links export, expected property and declared subtype. Keep latest links, sampled links and aggregated views distinct. Keep source canonical/root-domain grouping, units and truncation without inferring whole-web coverage. Reject an unknown or mismatched property, unadmitted schema or unsupported rights. The connector stays read-only |
| 272 Provider backlink evidence | Admit each of Ahrefs, Semrush, Majestic and Moz files separately by schema and allowed use. Keep recorded source labels and dates. A file cannot create a quota, billing, connector or live refresh |
| 273 Link change notices | Detect a first observed link, a complete read with the link absent, a verified gone source page and a later confirmed return. A success after timeout or gap merely confirms seen. Use the state rules above. Show the actual evidence and coverage, never “lost”. Files without a listed link can say only “Not listed in this file”. Timeout, denial, pause or stale source cannot manufacture a change |
| 274 Risk review | Show observed link and type, evidence date, unusual relation or placement and the admitted rule. Distinguish observed paid-link evidence from an unexplained pattern. Third-party composite scores are neither copied nor used to judge a link. Explain that most sites need no action. No accusation, automatic removal or disavow |
| 276 Outreach targets | Require an eligible own resource and an admitted 0092 topic match to a specific comparable resource with current version and relation. Show a referring-page lead only where an admitted 0046 source observation names its exact link to that resource. Keep the scoped referrer and target IDs and dates. A candidate gives no contact information or authorisation |
| 277 Unlinked mentions | Use a versioned 0097 mention for the active site and exact admitted brand alias. Require a source-bound excerpt range and a complete checked link map for that range. Match destinations against the current authorised own-site URL identity. An unsupported or incomplete map gives “Link status unknown”. An absence flag alone proves nothing. An unlinked result describes only the inspected range |
| 278 Broken target reclamation | An eligible non-stale referring observation names the exact own-site destination. Require current 0044 evidence of HTTP 404 or 410 at that destination with no redirect chain. A soft error page with HTTP 200, timeout, denial or redirect gives no broken-target claim. Show a reviewable list. Nothing adds a redirect or edits an outside page |
| 282 Anchor text profile | Group distinct link occurrences by source, referring page and target, retaining the admitted occurrence key where observed. Two observed anchors on the same page can contribute two occurrences. A supplied aggregate cannot become individual occurrences. Keep scrubbed text, known image-only or empty, and withheld or unknown separate. Show sample denominator and source; do not infer intent by anchor type |
| 283 Link growth | Chart one pinned source and cohort with the exact admitted observation identity and interval dates. Add coverage and comparability labels. A change of cohort or coverage, an unknown interval or a new source gives no growth rate. Keep reports of new observations separate from claims of new links. A complete scoped zero, when admitted, remains distinct from missing evidence |

Use the same link state and comparison functions for the review
table, charts and notices. Order accepted states by evidence
time and stable observation identity. An identical replay changes
no state. Late evidence can correct a historical row but cannot
dispatch an older transition as new. Bind one notice ID under
0099 to site, source, link and logical transition, including its
prior and new direct observations. Retries and corrected evidence
reuse the same ID for that transition. Withdrawal retracts the
dependent notice without a new alert.
Only local staff views are used here. Never infer a link from
a mention, or acquisition from a file's first appearance alone.

#### Disavow files (275)

Open “Prepare a disavow file” only when the owner supplies current
Search Console evidence of an active manual action for unnatural
inbound links, for the exact mapped URL-prefix property they
own. Feature 275 uses this stricter trigger. The provider also
describes use where an action appears likely. Verify the supplied
property, type, status, source revision and date before admission. A stale,
lifted, unsupported,
wrong-property or unresolved action gives no export. Do not
treat risk flags or a third-party score as manual-action evidence.

Use the [Search Console file guide][disavow] and
[manual actions guide][manual-actions]. Show the link evidence
and an owner-entered reason and removal effort for each selected
entry. A replacement needs either the exact supplied current list
with a checked property and file digest, or the owner's current
record of no list for that property. Bind owner, scope, evidence
age and digest to the whole proposed replacement. Parse valid
lines despite blank or comment lines. A comment is never an entry.
Refuse unreadable, malformed, missing or ambiguous baseline data,
and never infer an empty original from a failed parse. Show all
retained, added and removed entries. Reordering changes no entry.
The default is an exact source URL. A domain entry needs separate
per-entry confirmation, including its wider scope. Refuse an empty
output or a whole-list clear.

Only one exact URL or one prefixed domain per line. Validate
against the admitted Google profile. Refuse wildcard and subpath
patterns, control characters and ambiguous or unsafe syntax.
Write UTF-8 with LF, a
`.txt` suffix, no comments, at most 2,048 characters per URL,
100,000 lines and 2,000,000 bytes. Canonicalise duplicates by
type and exact admitted URL or domain identity. An HTTP URL
in this file is inert source data and causes no fetch. Preview
with the mapped property, file digest and exact lines.
Recheck owner, property, source evidence, expiry and exact payload
immediately before protected local hand-off under 0100 and 0118.
Pin the active manual-action proof and exact currently admitted
baseline to the confirmation. The baseline is the bound current
list and file digest, or that property's exact current no-list
record. A newly accepted baseline, withdrawn proof or a change
from no list to an existing list invalidates the old confirmation
before any file release. Review the new baseline and replacement
before confirming again. Never silently merge or replace entries.

Write one approved file or withhold it. A replay after an unknown
local outcome reconciles the exact 0100 receipt. It cannot
complete a stale stage or issue a second copy. A superseded local
hand-off remains dated history only within current rights. It
supplies no current upload instruction or external outcome.
Keep all managed copies protected. If the file profile or output
is unadmitted, withhold the hand-off.
Give basic instructions for the owner to use Search Console's
disavow tool by hand. Explain that the upload replaces the
selected property's list and can affect child properties. HTTP
and HTTPS properties are separate. Parent and child lists combine
where the provider applies both. This is information from the
official guide, with no live verification in Link study.
Never upload, cancel a list, submit reconsideration or claim an
external outcome. The owner can record the hand-off and time as
an owner-reported event.

#### Outreach email (280)

DR-116 puts third-party outreach email outside version 1. Saved
local outreach, PR and guest content records grant no email
capability. Do not offer a send route, create a job, reuse an
alert recipient or accept a legacy or restored outreach approval.
The existing spec 0100 alerts and reports retain their own scope.

REQ-0046-OUTREACH-SCOPE requires that no outreach route can send.
E2E-0046-OUTREACH-SCOPE tries a saved recipient, a former task, an
old approval and a restore. No outreach mail is attempted. An
independently authorised report can still use its existing route.

| Requirement | Verification contract |
|---|---|
| REQ-0046-OUTREACH-SCOPE | E2E-0046-OUTREACH-SCOPE: try a saved recipient, a former task, an old approval and a restore. No outreach mail is attempted. An independently authorised report can still use its existing route |

#### Local follow-up records (279, 281 and 284)

Use the existing site-scoped action layer for local outreach
pipeline, campaign and guest content records. No native CMS
effect or outside service follows a record change.

| Record | Required scope |
|---|---|
| Outreach item | Site, item and approved lead reference, audience and allowed contact reference or none, assignee, due day, status research/ready/contacted/replied/closed, current revision and evidence dates. Contacted and replied record an owner's manual confirmation only |
| PR campaign | Site, campaign, purpose, owner and contributors, date scope, selected outreach and guest items, independently approved local notes, revision. Show scoped status counts only. No unsupported campaign impact claim |
| Guest content item | Site, campaign or none, exact eligible evidence or manually supplied destination, authorised creator reference, type proposed/drafted/confirmed, publication date or unknown, source and revision. A confirmed item requires current evidence of the named external publication |

Require current staff access to each item and parent campaign
before update or filtered counts. Bind the approved contact
reference to its admitted purpose, consent or other recorded
permission and retention limit under 0098. A lost contact grant
removes that reference and every derived result. It cannot be
restored through an older campaign record. Store no mailbox
content.

Allow outreach moves from research to ready, ready to contacted,
contacted to replied, or any open state to closed. Contact and
reply are the owner's dated manual statements, with no assertion
of verified delivery. Guest content goes from proposed to drafted
or confirmed. Under QR-130, confirmation needs admitted supplied
evidence for the exact local item and revision, observed work,
mapped external destination, observed publication time and source
evidence ID. Use a current authorised identity mapping under the
admitted evidence profile. A matching title, reused URL, owner
note or draft alone gives no publication proof. Show an eligible
external publication as a dated observation. It does not prove
the item is still online.

A changed item or conflicting or withdrawn proof holds its current
confirmation, including before campaign counts, replay and restore.
A permitted old observation stays dated history for its own
matched item. It cannot confirm a replacement. A declined or
uncertain source cannot confirm an item. These checks open no
external page. Campaign linking is atomic and rechecks every member's site and
grant. Protect IDs and notes under 0098 and 0118. These views
harvest no contacts, probe no mailboxes, schedule no messages and
fetch no destinations. No approval for a lead or campaign
is approval to contact someone. Updates bind exact actor and
revision. Use the existing deduplication key across retries and restore.
A stale or replayed request cannot create a second item.
Refuse another site's data, dependency loss or unadmitted rights.
Erasure removes affected records and derived outputs from current
views, replay and restore. Retain only the safe audit outcome.
The public-page footprint remains zero. QR-130 owns source,
privacy and workload evidence.

| Requirement | Verification contract |
|---|---|
| REQ-0046-IMPORT-LINKS | IT-0046-IMPORT-LINKS: import an approved synthetic report per source. For an exact property-less Bing file, require the admitted same-file and property receipt. Substitute same-name bytes, report subtype, level, observed interval or property under a valid other receipt. Hold without changing earlier eligible records. Also try omitted coverage, unknown sentinel, incompatible schema, unsafe payload and replay. No call or mixed provider total |
| REQ-0046-LINK-READOUTS | IT-0046-LINK-READOUTS: test confirmed absence then return versus timeout then seen, stale and duplicate evidence, stable notice IDs, compatible cohort and missing data, exact lead relation, incomplete mention map, soft 404, redirected and stale target, and two distinct anchors on one referring page. Replay and revoke evidence |
| REQ-0046-DISAVOW | E2E-0046-DISAVOW: require exact owner, URL-prefix property and active manual-action proof. Change the valid prior list after confirmation, then separately replace a no-list record with proof of a current list. Hold both old hand-offs. Try lost reply before and after a proved local commit. Retain malformed, reordered and commented baseline, domain, protocol, child scope and file-boundary cases. Verify one exact eligible local file and zero provider writes |
| REQ-0046-PR-LOCAL | IT-0046-PR-LOCAL: confirm scoped item, parent, consent, evidence and current revision. Try another site's actor, invalid stage, lost contact permission and a guest record without publication proof. Replay and restore. No automatic site, contact, mail or posting request |

#### Wave 2 local outreach and refusal bindings

These cases use synthetic local records. The current site, staff
scope, parent scope, item revision and applicable contact or evidence
permission must match. QR-130 retains the source, privacy and workload
qualifications. Current permission applies again on replay and restore.

| Feature | Owning requirement and design | Planned fixture and expected evidence |
|---|---|---|
| F-0279 | REQ-0046-PR-LOCAL · DES-0046-W2-MANUAL-PIPELINE | FX-0046-W2-MANUAL-PIPELINE · IT-0046-PR-LOCAL. Confirm a scoped lead and exact local stage update. Check research, ready, an owner-dated manual contacted or replied entry and closure. Refuse a stale or unpermitted move. Withdraw the contact permission and restore an earlier record. Remove the affected reference and derivatives, and never replay or duplicate an old update. A manual entry claims no verified delivery and triggers no contact. |
| F-0280 | REQ-0046-OUTREACH-SCOPE · DES-0046-W2-OUTREACH-SEND-REFUSAL | FX-0046-W2-OUTREACH-SEND-REFUSAL · E2E-0046-OUTREACH-SCOPE. Under DR-116, try to send from a local outreach record, borrow an alert recipient, reuse an old approval and restore a former outreach job. Require refusal before a send capability, queued work or dispatch. A previously and independently authorised 0100 report retains only its exact report purpose. No outreach email in version 1. |
| F-0281 | REQ-0046-PR-LOCAL · DES-0046-W2-CAMPAIGN-MEMBERSHIP | FX-0046-W2-CAMPAIGN-MEMBERSHIP · IT-0046-PR-LOCAL. Attach eligible local outreach and guest items to an authorised campaign atomically. Try another site's item, a stale campaign, a denied item and scope withdrawal. Counts use current authorised membership and statuses only. Refuse the whole mismatched update and hide withdrawn derivatives. No impact inference, contact effect or publication. |
| F-0284 | REQ-0046-PR-LOCAL · DES-0046-W2-MANUAL-GUEST-CONTENT | FX-0046-W2-MANUAL-GUEST-CONTENT · IT-0046-PR-LOCAL. Compare a proposed item, a local draft and an exact independently admitted publication observation for item R1. Reuse the destination and title for a different external work or local revision R2 and replay the R1 observation. Withhold R2 confirmation and any derived confirmed count. A newly admitted exact R2 observation supports its own dated confirmation. Hold missing, wrong or withdrawn proof. Replay and restore preserve only the eligible original history and do not duplicate a confirmation. No outside lookup, contact or publication. |

[disavow]: https://support.google.com/webmasters/answer/2648487?hl=en
[manual-actions]: https://support.google.com/webmasters/answer/9044175?hl=en
[bing]: https://www.bing.com/webmasters/help/backlinks-c3a334e8?source=card
[gsc-links]: https://support.google.com/webmasters/answer/9049606?hl=en

### Wave 2 supplied backlinks and Link study bindings

Use synthetic permitted files and stored observations. Pin the exact
site or client, source, rights, property or target, observed dates,
subtype and coverage. QR-129 and QR-130 retain file, privacy and
comparison qualifications. Link study starts no network read, login,
monitor or follow-up. Nothing here expands the separate 0045 and
0082 lead rules or grants contact, report delivery or site changes.

| Feature | Owning requirement and design | Planned fixture and expected evidence |
|---|---|---|
| F-0264 | REQ-0046-RIVAL-OVERLAP · DES-0046-W2-SUPPLIED-RIVAL-COMPARISON | FX-0046-W2-SUPPLIED-RIVAL-COMPARISON · IT-0046-RIVAL-OVERLAP. Give two permitted comparable lists with a referrer repeated under distinct link rows, a shared referring page, separate pages on one domain, a wrong destination and a conflicting replay. Count one presence per exact referring page per list. Show both, own-list-only and rival-list-only against each file's scope. Keep comparable partial results observed-only. Recheck current rights, replay and restore. No link claim outside the files or lead read. |
| F-0270 | REQ-0046-IMPORT-LINKS · DES-0046-W2-SOURCE-LABELLED-IMPORT | FX-0046-W2-BING-LINK-EXPORT · IT-0046-IMPORT-LINKS. Give an admitted property-less Bing export and its matching exact file and property proof. Try another file with the same name and row count, a changed report level or observed interval, a missing receipt and a wrong property. None can replace the valid earlier import. Replay the exact eligible file without changing its dates or coverage. Keep page, domain and anchor views in their native units and scope. No connector call or unadmitted receipt form. |
| F-0271 | REQ-0046-IMPORT-LINKS · DES-0046-W2-SOURCE-LABELLED-IMPORT | FX-0046-W2-GSC-LINK-EXPORT · IT-0046-IMPORT-LINKS. Use admissible latest-links, sampled and grouped Search Console files as distinct subtypes for an exact property. Try a foreign property, an undeclared grouping, unsafe data and a documented zero sentinel. Preserve eligible scope, grouping, truncation and distinct units. A documented unknown is never zero. No outside request or connector write. |
| F-0272 | REQ-0046-IMPORT-LINKS · DES-0046-W2-SOURCE-LABELLED-IMPORT | FX-0046-W2-PROVIDER-LINK-FILES · IT-0046-IMPORT-LINKS. Independently test admitted and refused-rights file fixtures for Ahrefs, Semrush, Majestic and Moz. A missing schema or allowed-use record holds its input without contaminating another source. Keep dates and source credits with eligible rows. No summed provider total, key, spend or refresh. |
| F-0273 | REQ-0046-LINK-READOUTS · DES-0046-W2-DIRECT-LINK-STATE | FX-0046-W2-LINK-STATE-NOTICES · IT-0046-LINK-READOUTS. Compare stored first seen, full page read without the link, a verified gone source page and a later confirmed link return. Contrast timeout then seen, comparable file omission and late or repeated evidence. A timeout cannot establish absence and a later successful read alone gives no return notice. Keep file omission separate. Reuse the local logical transition ID and withhold withdrawn notices. No scheduled read from a notice. |
| F-0274 | REQ-0046-LINK-READOUTS · DES-0046-W2-PROTECTED-LINK-REVIEW | FX-0046-W2-RISK-REVIEW-EVIDENCE · IT-0046-LINK-READOUTS. Show dated link, relation or placement evidence and the applicable admitted rule. Compare an ambiguous pattern and separately confirmed manual-action evidence for the exact own property. A manual action remains separate evidence and cannot by itself identify a link as its cause. Explain the evidence limit. No vendor score, accusation, removal or disavow effect. |
| F-0275 | REQ-0046-DISAVOW · DES-0046-W2-EXACT-DISAVOW-DRAFT | FX-0046-W2-EXACT-DISAVOW-DRAFT · E2E-0046-DISAVOW. Match the exact owned URL-prefix property, current active unnatural-inbound-link action and prior list or explicit no-list proof. Keep a valid confirmation, then replace the list or admit a first list where there was none. Hold release, including after restore. After a proved hand-off, lose its reply and reconcile the same receipt within rights as history only if the baseline changed. Retain comment, malformed file, HTTP and HTTPS sibling, child scope, domain-entry, size limit and whole-list refusal cases. No upload, deletion or external outcome. |
| F-0276 | REQ-0046-LINK-READOUTS · DES-0046-W2-PROTECTED-LINK-REVIEW | FX-0046-W2-EXACT-RESOURCE-LEAD · IT-0046-LINK-READOUTS. Require the eligible own resource, admitted 0092 topic match to an exact comparable peer resource and current source evidence of the referring page's link to that exact resource. Compare a mismatched resource and wrong observed target. Show only supported inert research leads with dates and scope. No contact lookup, send, collection or live monitoring. |
| F-0277 | REQ-0046-LINK-READOUTS · DES-0046-W2-ADMITTED-MENTION-RANGE | FX-0046-W2-ADMITTED-MENTION-RANGE · IT-0046-LINK-READOUTS. Use 0097's current versioned eligible reference and exact approved alias. Compare a linked excerpt, a complete checked range with no link to an authorised own destination and an incomplete map. Keep observed linked, observed unlinked and unknown separate. An unmapped destination or incomplete range gives no absence claim. No monitoring or follow-up read. |
| F-0278 | REQ-0046-LINK-READOUTS · DES-0046-W2-PROTECTED-LINK-REVIEW | FX-0046-W2-BROKEN-OWN-TARGET · IT-0046-LINK-READOUTS. Bind a current eligible stored referring observation to the exact own target. Only matching current 0044 direct 404 or 410 without a redirect supports a reviewable dead target. Exclude soft 404, timeout, denied, redirected and stale observations. No redirect or referring-page edit. |
| F-0282 | REQ-0046-LINK-READOUTS · DES-0046-W2-OBSERVED-ANCHOR-OCCURRENCES | FX-0046-W2-OBSERVED-ANCHOR-OCCURRENCES · IT-0046-LINK-READOUTS. Compare distinct eligible same-page occurrences, duplicate delivery, known empty or image-only anchor and withheld or unknown text. Keep one row per admitted occurrence key, separate sourced aggregates and disclose only approved scrubbed text, sampled denominator and permitted omission counts. Unknown or unsafe text remains separate. No intent claim. |
| F-0283 | REQ-0046-LINK-READOUTS · DES-0046-W2-COMPARABLE-LINK-WINDOWS | FX-0046-W2-COMPARABLE-LINK-WINDOWS · IT-0046-LINK-READOUTS. Compare closed compatible observed periods for one pinned source and cohort, then add crawl coverage or omit a day. Show change in observations only where scope matches. Changed or unknown coverage withholds the growth rate. Complete admitted zero stays distinct from missing. Do not infer acquisition or removal from coverage changes. |

### Evidence box

1. A coverage estimate from two supplied sources' overlap is research
   only. It stays "not measured" until it passes on made-up worlds with
   known truth; if it never passes, it never ships.
2. A report with too little evidence offers a permitted recheck,
   supplying a file or adding a named lead under spec 0045.
   Repeat requests merge. An evidence request fetches nothing.

### Retiring a source

An expired or withdrawn file can no longer support a figure. Mark
missing evidence and remove data under the owning retention and
erasure rules. Restore applies the same restrictions.

### Settings

| Setting | Default | Allowed range |
| --- | --- | --- |
| `link.study.fresh_days` | blocked: no admitted value | safe bottom and top |
| `link.study.min_sample` | blocked: no admitted value | above zero, safe top |
| `link.study.compare_window_days` | blocked: no admitted value | safe bottom and top |
| `link.study.survival_min_days` | blocked: no admitted value | safe bottom and top |
| `link.study.referral_min_visits` | blocked: no admitted value | at least darshak's floor |
| `link.study.service_keep_days` | blocked: no admitted value | follows admitted file rights |
| `link.study.memory_share` | blocked: no admitted value | fixed share of the service slice |

Every value marked "blocked" is a readiness blocker owned by the link-study
Ganit Vikas research record and the file source's legal and privacy
record. Admit the value, bound, fixture and approval before running
the dependent report, import or retention behaviour. Each needs a
finite admitted bound.

### Build-blocking research (Ganit Vikas)

1. What sample and age make a survival range better than "not enough
   history"?
2. How many visits make a per-link value stable and private?
3. How many matched pages make a before and after test fair?
4. Which dates make numbers comparable?
5. Do two services overlap enough for a coverage estimate to be right on
   made-up worlds with known truth?
6. Bounded intake and what each source lets Sutradhar keep.
7. A link strength score: only counted facts ship at first. Ganit Vikas
   researches a score, added only if it passes on made-up worlds with
   known truth.

### Changes this part makes to other parts

1. Backlink crawl (spec 0045): records carry `last_checked`, `last_read`
   and a reason a link was not seen.
2. Spec 0040 owns its own-site search data routes. Provider backlink
   data uses owner-supplied files. Live providers are outside the
   scope of specs 0045 and 0046. Receipt context follows the 0082
   display, export and legacy length checks.
3. SEO health (spec 0044): backlinks show "not measured" only with no
   source and no run.
4. darshak (spec 0034): keeps a referrer class and also a daily count by
   referring site; the path only if privacy research allows.

### Not part of it

1. No sharing between installs, peers, relays or cohorts learned across
   sites.
2. The crawl only visits leads. No crawl frontier of its own, and no
   other sites' text is kept, only link records.
3. No web graph is held and no reach, trust or rival rank or traffic
   score is shown, unless a score passes research.
4. AI-answer tracking has its own design.
5. No language-model tier, outside message bus, search cluster, graph
   database or extra deployable programs.

## Limits

- Runs inside its fixed memory share (`link.study.memory_share`) of the
  service slice, within the memory limits in AGENTS.md section 5.6, and
  stops first under pressure.
- File intake and retention use the admitted source bounds.
- The study itself makes no network calls.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Rust tests on made-up link worlds:

- Every figure shows kind, source, date, sample and range, or "not
  measured". None appears bare. No source and no run shows no zero.
- Different kinds or sources are never added. Unlike numbers are refused
  with a reason and show no difference.
- A missing link shows "not seen since", a date and a reason, never
  "lost" or "removed". A 404 or 410 shows "page gone", apart from the
  link.
- Each error or block in the state list leaves the link "seen" and due
  for recheck. A returning link shows "back again" with both dates.
- The study opens no connection and never reads a provider key.
  Restart and refresh with a rival saved. Check there is no repeat
  job and no live provider call. Import an admitted source-labelled
  file and keep each site's and client's figures separate.
- Too little evidence gives "not enough evidence" and no forecast. An
  evidence request fetches nothing, and repeats merge.
- Survival is hidden until the minimum history. Low visits show "too
  few". Matched pages that moved alike give "correlation, not cause".
- Planted-fault worlds: 10,000 links from one site count as one
  referring site; wider checking is not shown as gain; an unreadable page
  is not "lost". The expected answers are fixed first, and changing them
  needs an independent review.
- Expiry or withdrawal removes a file's figures, including after
  restore. Every setting has a safe bottom and top.
- Under memory pressure it stops first and stays inside its share.

## Changes from the source

- Link states, labels, comparisons and hidden figures are decided in
  Rust, with Kani proofs where they fit.
- The maths runs in ganit, the Rust numerical engine.
- Admin screens are a desktop app built from SDS components.

Source: `docs/specifications/signals/link-study.md` in the archive
`archive-2026-11-08-old-repo`.
