# 0047: Site structure checks

These checks show a site owner what is wrong with how the site is built,
once per cause rather than once per page. Every check is a report over
pages the crawler (spec 0044) has already read. Nothing here fetches a
new page, changes a page or writes to the site. It is advice only. It is
part of roadmap Phase 6; "Which spec designs each feature" in
[features.md](features.md) lists the features it designs in whole or in
part.

Example: a theme update drops the breadcrumb markup on thread pages. The
health report would list 9,800 faulty pages. These checks give one
finding, "Thread page layout, second revision: breadcrumbs missing", with
five separate counts.

There are four reports, each on its own screen: "Layout groups",
"Language links", "Shop details", and "Weak pages" with "Falling
pages". Market match is after version 1 (DR-081).

## What it does

### Shared rules

1. Registered sites only, XenForo and WordPress, each kept apart. No
   sharing between installs. Staff-only screens; guests see only what the
   site allows.
2. No second crawler and no new fetching. The new work is one extra look
   at HTML the crawler already fetched, and reports over the stored
   result. No report opens a network connection.
3. Advice only: no apply button, no title change, no link inserted. The
   test that walks every report screen covers these screens too.
4. A dependent feature starts only after every required value has an admitted
   default, safe bottom and top. A blocked value disables only its dependent
   optional report; a missing safety, privacy or resource bound fails closed.
5. A language declaration on another registered site remains
   outside this report's observations.
6. Each finding shows what, where, how many pages, evidence kind, crawl
   date and a written proposal for a person. Findings group into one
   alert per cause.

### The evidence floor (everywhere)

Every report has a floor: the least evidence it needs before it may
speak. Each floor is an admitted setting or remains blocked as the settings
table states. Below it the report says "not
enough evidence" and names what is missing. It never shows a low score, a
zero or a guess, and is never styled as success or failure.

| Word on screen | Meaning |
| --- | --- |
| Seen | The crawl read the page and found it |
| Not enough evidence (what is missing) | Below the floor; no figure |
| Not measured | No source, or the crawl has not run |
| Not applicable | Nothing to compare, such as one language only |
| Recheck due | Two looks too far apart to compare |

1. Every count names its population: "of 640 pages read", never a bare
   "98%".
2. A last-seen value stays, marked stale and dated. It is never blanked.
3. Different sources and kinds are never added or merged into one score.
   A score appears only if Ganit Vikas research proves it on made-up
   worlds with known answers.
4. A report that cannot run says so; it is never shown as passed.
5. No fallback to a weaker score when evidence is missing.

### Shared plumbing

1. The crawl pass writes a small `seo_structure` record per page and
   per-kind detail tables. No page text is kept beyond the page copies
   of spec 0044.
2. A report job runs after each full crawl and each changed-page pass,
   within the admitted report memory budget; `seo.structure.memory_share`
  remains blocked until its value and evidence are admitted.

### Layout groups

Pages are made by a few layouts (a thread page, a forum list, a shop
product, a WordPress archive). When a layout is broken, every page it
makes is broken. This report works out which pages share a layout and
reports a fault once, with counts.

Example: breadcrumbs missing on one thread layout gives: seen faulty 212,
seen clean 40, same layout but not yet read 9,548, fixed and re-read 0.
It never says "9,800 pages are broken" or "212 pages fixed".

The structure signature:

1. During the crawl's existing parse, a thin adapter lists each page's
   structure marks.
2. Kept (stable): landmark roles, structured-data types, headings by
   level, list and form shapes, the site's own stable component names,
   and class names an admin blesses for that site. The generic rule never
   assumes a class is stable.
3. Dropped (volatile): ids, hashed class names, tracking attributes,
   inline values and all text.
4. Repeated blocks count as one block of "many": a 5-paragraph and an
   18-paragraph article, or 6 and 8 related items, give the same
   signature.
5. Output: a hash and a short list of mark kinds, so a drift report can
   say which part moved. No text or markup is kept. The same rules and
   page give the same signature every time.

Three identities:

| Identity | Changes when |
| --- | --- |
| Layout family | Never for content, images, copy or personalisation |
| Revision | The listed SEO-relevant marks change materially |
| Variant | The admin declared an experiment marker and its value differs |

1. The SEO-relevant marks are a setting list: titles, canonical,
   hreflang, structured data, navigation links, pagination links and
   headings. The exact material-change normalisation and comparison rule is
   a readiness blocker owned by the layout research record; revision detection
   cannot build until its deterministic rule and fixtures are admitted.
2. Variants exist only if the admin names one marker (such as an HTML
   attribute that tells test arms apart). With no marker there are none.
   A variant never splits a family; a test that changes structure makes a
   new revision in the same family. Cookie-based arms are invisible to a
   guest crawl, and the report says so.
3. The declared page kind corroborates only. If the database calls pages
   "thread" and they form two structures the rule will not join, the
   report shows "declaration conflict" and does not trust the label. With
   no declared kind, pages still group by structure alone.
4. When the rule version changes, earlier history is marked "older
   rules" and no chart line crosses the break.

Group floor and counts:

1. A group needs a least number of pages and a least share read before it
   is called a layout ("not enough evidence: 2 pages is not a layout").
2. If too few crawled pages fit any group, the screen says "not enough
   evidence: only N of M pages could be grouped" and makes no claim per
   layout. The pages-to-layouts ratio is shown as a fact, never as pass
   or fail.
3. Five separate counts, never added: seen faulty, seen clean, same
   layout but not yet read, fixed and re-read, and group size. "Not yet
   read" comes from the database's guest page list matched to the
   group's page kind and path rule, and never appears as faulty. "Fixed"
   counts only pages re-read by check my fix. A fix is described as "this
   changes one layout", never "N pages fixed" before the re-read.
4. Visits or impressions may be attached to the seen-faulty set only,
   from totals, never scaled to the rest.

What a group can report (each from an existing per-page fact): layout
drift; structured-data changes inside a group; title patterns that
collide or cut off (report only; titles are never changed); navigation
link-shape change; pagination faults; canonical logic faults;
repeated-page patterns (ganit job 286); mixed revisions inside a group;
a new revision spreading between two crawls, as counts, never as
"stalled". The screen says "structural attribution" and never "caused".
These groups are also the page types for speed by page type.

A learned similarity step, if research ever adds one, may only propose a
candidate that the rule then checks. It never creates or changes an
identity.

### Page speed evidence (330–337)

Use each existing page-type identity and bind the exact signed
template revision from a native receipt at capture time. A later
mapping cannot assign revision history to field rows. Read only the
site's authorised 0034 field aggregates, 0044 and 0115 Audit
observations, or owner-supplied protected timings and snapshots.
Keep field, local test and simulated metrics separate. This report
adds no collection script, page work, live stress load, cache purge,
plugin switch or external service connection. Spec 0134 is the
separate CI load-test gate.

A speed profile pins site, template, material revision, measured
metric, unit, statistic, population, coverage floor and device,
processor, viewport, resource and cache state, runner, clock and
capture method. A budget adds an owner-set ceiling, version and effect
date. Compare only the same admitted source and metric profile.
A value above its ceiling shows a gap. Missing or unsupported
evidence shows “Not measured”. Historical rows retain their old
budget revision. Never compare field p75 with a simulation median.

For plugin or add-on cost, accept only matched owner-supplied
staging timings or a scheduled comparison of frozen, permitted
local fixture snapshots. The pair differs only in one named
extension, its version or its enabled flag. All other code, data,
host limits, cache state, requests and measurement method match.
Pin both manifests and dependency hashes, reset to the same cache
state between pairs and alternate baseline-first and changed-first.
Use the first five valid preregistered pairs, at most seven attempts.
Measure elapsed full-response time on the same monotonic clock.
Only a proved runner fault invalidates a pair. Keep the reason.
A request timeout is a censored sample and withholds the estimate.
Report median after-minus-before milliseconds, minimum and maximum
paired changes, and the signed capture receipt, including negatives. Different
versions alone cannot show that a particular plugin caused a cost.
No production extension is toggled.

Use [CSS Font Loading] terms on admitted 0115 static CSS evidence.
Record declared face, resource ID, bytes, broker start/end time,
preload and `font-display` when observed. Require the admitted
paint trace to link that face, clock and text span to a visible
fallback or measured layout shift. Missing timing stays unknown.
An unused rule or download alone does not prove text was blocked. Recommend only
supported changes, such as removing a duplicate verified request.
Never fetch a new font to resolve an unknown. Keep declarations,
loads and effects distinct.

Read the top-level HTML response and the qualified mapping of
the cache hops. Parse RFC 9211 [Cache-Status] as Structured Fields,
origin-nearest first, with mutually exclusive hit and forward per
hop. A whole-page hit needs a proved just-handling hop and complete
fresh HTTP 200 HTML. Outer hits can replay historical inner
metadata. Report stale, range, 304 and forwarded actions separately.
Asset hits, `Age` and `Via` alone prove no whole-page hit.
Missing or inconsistent evidence gives unknown.
A CDN finding names the proved edge, origin or asset scope,
source revision and coverage. Unproved rivals and markers stay out.
Use only already observed headers or signed edge evidence.

A speed notice needs an exact native template transition and an
admitted matched full-response median budget. Pin before and after
versions, positive timings, comparable profile and captures within
seven elapsed days on each side of the transition. From the first
five valid pairs in seven attempts, require after median above
budget and at least four ratios above 1.15. Use exact ratios.
Equality fails either comparison. Missing or censored data holds.
The 0099 notice key binds site, transition and budget revision.
Recheck latest evidence, budget revision, grants and destination
at send, including release after quiet hours. If a replacement
budget takes effect before dispatch, close the deferred notice
without a send. Applying a replacement budget cannot change an
uncertain send into a confirmed cancellation. Keep that effect
held by its original key under 0099 through restart. Deduplicate
via the outbox. Show “Slower after this change”.

Run mobile processor tests in an admitted 0115 offline paint
fixture replay. Pin architecture, renderer, 390 by 844 CSS-pixel
viewport, network trace and cold local cache. Apply the admitted
host CPU execution limit to one isolated worker, at 25% of its
reference allotment. Calibrate with five alternating pairs of the
same synthetic CPU-bound task. Require median wall-time ratio from
3.6 to 4.4 inclusive, with no virtual delay or artificial sleeps.
A failed tolerance gives “Mobile processor test unavailable”.
Label results local, and field INP unknown without interaction
evidence. An admitted supplied trace is inspection evidence only.

Suggest an image CDN action only for the proved existing image
edge, same-site rights and comparable transfers for that asset,
variant and cache state. Show measured transfer alongside separate
estimated savings from a reproducible local re-encode. Preserve
aspect ratio, accessibility text and asset licence. Unknown or
unsafe edge scope gives evidence gaps, with no remote change.
No DNS change, optimiser call, upload or source URL rewrite.

Cost and mobile fixture jobs run separately from 0044 Audit,
under one 0019/0078 lease per site and one CPU slot. Use at most
256 MiB, ten minutes and seven attempts including calibration
retries. A job that cannot fit is unavailable. No live network,
cross-site caches or production switch. Check current site grant,
source epoch and budget at claim, commit, view, send and restore.
Keep only derived facts and receipts under 0030/0118. Erase scratch
on finish or recovery. An active result is current for seven
days from its last capture and only for its admitted versions.
An older baseline can be evidence for the exact comparison.
A changed version or
withdrawal invalidates results and queued work. Store no raw
headers, keys or source HTML in reports. This feature adds no
ranking feed or guest page work. Test no extra queries, bytes or
public work on each supported add-on and host.

#### Named speed verification bindings

These planned cases use the existing 0047 speed contracts. QR-134
holds exact capture, proof and resource admission. Use synthetic
or authorised protected evidence with site, capture time and
native revision pinned. Recheck current source rights for display
and replay. Keep only derived facts and receipts. The cases add no
reader, report export, extra measurement call, ranking input,
public work or native change.

- `DES-0047-W2-BUDGET` uses `VAR-0047-W2-SIGNED-PROFILE` for
  budget gaps and `VAR-0047-W2-NOTICE-TRANSITION` for the
  existing 0099 notice route. Test the notice with a simulated
  outbox and destination. Real delivery keeps current authority.
- `DES-0047-W2-LOCAL-REPLAY` uses `VAR-0047-W2-EXTENSION-PAIR`
  and `VAR-0047-W2-OFFLINE-MOBILE` for bounded jobs within
  the existing 0019/0078 lease and the admitted replay.
- `DES-0047-W2-RENDER-FACTS` uses `VAR-0047-W2-FONT-PAINT`
  for observed facts from 0115. Missing proof holds visible claims.
- `DES-0047-W2-EDGE-FACTS` uses `VAR-0047-W2-WHOLE-HTML-CACHE`,
  `VAR-0047-W2-EDGE-SCOPE` and `VAR-0047-W2-IMAGE-ADVICE`
  for protected evidence and local advice. Unproved edge scope
  yields unknown. No provider, upload, rewrite, purge or switch
  route follows from the evidence.

| Feature | Requirement | Design and variant | Fixture and planned test | Named fixture assertion |
|---|---|---|---|---|
| F-0330 | REQ-0047-W2-TEMPLATE-BUDGET | DES-0047-W2-BUDGET, VAR-0047-W2-SIGNED-PROFILE | FX-0047-W2-TEMPLATE-BUDGET, BDD-0047-W2-TEMPLATE-BUDGET | Match the captured native revision and exact metric profile. At the ceiling show no gap. Above it show the gap. A simulation or missing field result cannot fill a field budget. Keep the prior budget version. |
| F-0331 | REQ-0047-W2-EXTENSION-COST | DES-0047-W2-LOCAL-REPLAY, VAR-0047-W2-EXTENSION-PAIR | FX-0047-W2-EXTENSION-COST, IT-0047-W2-EXTENSION-COST | Take the first five valid alternating pairs within seven attempts. Report the median after-minus-before and paired minimum and maximum, including negatives. A censored timeout, failed matching or insufficient pairs withholds the estimate. |
| F-0332 | REQ-0047-W2-FONT-EVIDENCE | DES-0047-W2-RENDER-FACTS, VAR-0047-W2-FONT-PAINT | FX-0047-W2-FONT-EVIDENCE, IT-0047-W2-FONT-EVIDENCE | Separate an unused declaration, a duplicate observed load and a proved fallback linked to the same clock and text span. Withheld paint timing shows unknown. No outside font request. |
| F-0333 | REQ-0047-W2-FULL-CACHE | DES-0047-W2-EDGE-FACTS, VAR-0047-W2-WHOLE-HTML-CACHE | FX-0047-W2-FULL-CACHE, IT-0047-W2-FULL-CACHE | Count one proved fresh, complete 200 HTML hit at the handling hop. Do not count historical inner metadata or an asset, stale, 304 or range response towards that result. An unproved hop shows unknown. |
| F-0334 | REQ-0047-W2-CDN-ROLE | DES-0047-W2-EDGE-FACTS, VAR-0047-W2-EDGE-SCOPE | FX-0047-W2-CDN-ROLE, BDD-0047-W2-CDN-ROLE | Give separate authorised HTML and image edge receipts. Show only the supported edge and origin roles and their scope. Hostname shape and withheld headers cannot prove a CDN. |
| F-0335 | REQ-0047-W2-SPEED-NOTICE | DES-0047-W2-BUDGET, VAR-0047-W2-NOTICE-TRANSITION | FX-0047-W2-SPEED-NOTICE, IT-0047-W2-SPEED-NOTICE | Require the after median above the active budget and four of five valid ratios strictly above 1.15. Do not count ties. A censored set holds. With five valid pairs at 100 ms before and 120 ms after, defer a notice under a 110 ms budget. If a 130 ms replacement takes effect before dispatch, prove zero sends through quiet-hours release and restart. If dispatch won under 110 ms and its receipt is lost, the original send stays uncertain after replacement and restart, with no resend or reroute. Preserve the withdrawal check. |
| F-0336 | REQ-0047-W2-MOBILE-CPU | DES-0047-W2-LOCAL-REPLAY, VAR-0047-W2-OFFLINE-MOBILE | FX-0047-W2-MOBILE-CPU, IT-0047-W2-MOBILE-CPU | Use five alternating calibration pairs on the isolated worker at 25% reference allotment. Ratios 3.6 and 4.4 qualify. Reject virtual-only timing, failed calibration or revoked profiles. Field INP remains unknown. |
| F-0337 | REQ-0047-W2-IMAGE-CDN | DES-0047-W2-EDGE-FACTS, VAR-0047-W2-IMAGE-ADVICE | FX-0047-W2-IMAGE-CDN, BDD-0047-W2-IMAGE-CDN | Compare like variants and cache state at a proved image edge. Show observed bytes and labelled local re-encode estimates separately. Withhold advice for unknown edge scope, missing rights or a proposal that drops required accessibility text. |

[Cache-Status]: https://www.rfc-editor.org/rfc/rfc9211.html
[CSS Font Loading]: https://www.w3.org/TR/css-font-loading/

### Language links and country targeting structure (564)

Version 1 inspects existing declarations on the owner's own site.
Use the Audit observations of spec 0115 and the current site's
allow-list under spec 0044. This audit does not translate, recommend
a market or fetch an unauthorised alternate. Show "cannot confirm"
when the alternate is outside the admitted observations. A site
with no declarations in a completed sample shows "none observed".
An unread page remains unknown. One current observation can show
a malformed declaration. Reciprocity, canonical contradictions or a
missing target need comparable observations for both ends in the
same site and admitted Audit sample. Unread or expired evidence
gives "cannot confirm".

Example: one already-read page names another as an alternate. The
other page has no return link. Show one structural finding with the
observed addresses and sources.

Language links from the observed sample: pages are nodes and "names as
alternate" are edges. Each edge keeps where it was found (page tag,
header or sitemap), the crawl date and the page read; disagreement
between these is a finding. Checks:

1. Reciprocity: if A names B, B must name A. Missing self-reference.
2. A named alternate that is a 404, a redirect, noindexed or
   non-canonical.
3. A page in two clusters; alternates leaking between clusters.
4. Wrong or malformed language or region codes, against the published
   code list.
5. A missing or wrong default alternate for a cluster.
6. A canonical that crosses a language line.
7. Declared page language against the language its text reads, only if
   research shows the existing post-language detector can be reused;
   otherwise this check is left out.

Recompute changed clusters daily and all observed clusters weekly,
using the current site's existing Audit facts. Add no fetch. Missing
or expired facts and a redirect outside the current site give "cannot
confirm". A proposal writes out the correct full set of
alternates as text for a person to apply. There is no export file for a
content system. Link-graph figures name the population, such as
"of 420 language pages read".

Feature 43 would add alternate-language links to multilingual pages.
DR-081 puts that action after version 1. The read-only audit of
existing declarations in feature 564 stays in version 1. Audit
evidence gives no authority to update markup or sitemaps.

#### F-0564 protected observation binding

`REQ-0047-W2-COUNTRY-TARGETING-AUDIT` uses
`DES-0047-W2-OBSERVED-LANGUAGE-LINKS` and
`VAR-0047-W2-OWN-SITE-LANGUAGE-SAMPLE`. Use only current recorded
0044/0115 facts for the active site's admitted sample. The sample
identifies its scope, observation dates and declaration sources.
Both ends need comparable facts for a reciprocity finding.

`FX-0047-W2-COUNTRY-TARGETING-AUDIT` uses `IT-STRUCT-LANG`.
Compare a current reciprocal declaration, a proved missing return
link, no declaration in a completed sample, a malformed declaration
and an unread or expired alternate. Show “none observed” only
for the completed absence, and “cannot confirm” for the unread,
expired or out-of-scope alternate. Bind the exact Audit profile
under QR-147. F-0043 retains its separate deferred refusal. No new
fetch, cross-site expansion or markup and sitemap edit follows.

### Market match and currency or unit checks after version 1

DR-081 defers market declarations, market pages, per-market business
matching and market currency and unit checks. Version 1 has no settings,
jobs, screens or stored market records for them. Spec 0095 owns its
separate local listing checks and supplied evidence.

Markets:

1. The admin declares each market of a site: a country and language pair,
   and how its pages are told apart (a path prefix, subdomain or host).
   A host must be on the site's allowed-host list or be another
   registered site of the same owner.
2. A language version on a host the crawler may not read shows "cannot
   confirm: host not allowed", never "broken", and nothing is fetched.
3. One language only shows "not applicable". No markets declared gives
   "not enough evidence: no markets declared". The language-link graph
   needs no declaration.

Market pages: for each market, the key pages that exist in other markets
but not this one ("Router guide has en and de versions; no fr version").
The key-page selection rule, including whether it uses an admin list or
most-visited darshak totals, is a readiness blocker owned by the language-and-
market research record; the market-gap report cannot build until one rule and
its fixtures are admitted. Also near-identical local pages competing for
the same queries (ganit jobs 265 and 286, with layout groups), and
structured data that contradicts the market (a currency or language tag
that does not match). Report only; copy quality is never graded.

### Own-site branch details (344)

The local check compares the active site's own indexed pages.
DR-088 preserves this route under the DR-081 site scope. Market
declarations and per-market matching remain after version 1.
QR-069 and QR-135 hold the branch cap, confirmation age, skew
window, rights and lineage proof. Deferred market settings supply
no defaults for this route.

1. The admin types business name, address, phone and opening hours
   with the branch time zone. Keep one record per site and stable
   branch ID with confirmed revision, optional retirement and the
   exact authorised pages. The report compares indexed structured
   data, address and phone evidence for those pages. It treats
   the entered record as one source of evidence.
   Allocate an older single record once in the same site transaction,
   unique by original record ID and subject to the branch cap.
   Check the admitted format and capacity at allocation. If another
   branch takes the last slot first, leave the old record unallocated.
   Show “Branch limit reached” in the authorised view. If the exact
   migration reader is unsupported, hold allocation and show “These
   older details cannot be read” to the authorised requester.
   Historic reading requires an admitted reader for that record and
   the current grant. Unproved reading withholds its values. An
   unallocated record gives 0095 no branch binding or active totals.
   New totals require confirmation of the exact branch and page-map
   version. QR-069 and QR-135 retain reader and capacity qualification.
   Allow one active branch per page. A retarget checks the expected
   map revision and owner confirmation. Invalidate earlier plans,
   findings and markup grants. Never remap old snapshots.
   Retirement records a durable generation tombstone. Restore
   cannot revive a retired ID, stale map or revoked grant.
2. Per field, both values: "phone: page says A, you entered B, page X,
   read 3 Nov". A difference, with no verdict on which is right. It never
   writes back or changes the typed fact.
3. "Not found on page" and "could not read" are not differences; each has
   its own label.
4. Typed details carry a confirmed-on date. Past the confirm age they
   show "please confirm" and the match still runs, marked stale.
5. Compared only if the page was read within the skew window; otherwise
   "recheck due" and a re-read is offered, never forced.
6. No details entered gives "not enough evidence: no business details for
   this branch" with a button to enter them.
7. Matching folds accents, case, number formats and address
   abbreviations. No outside map or address service is called.
8. Local pack position, map ranking and review pages are shown as not
   observable, with no fetch and no connect button.

#### F-0344 verification binding

`REQ-0047-W2-BRANCH-DETAILS` uses `DES-0047-W2-BRANCH-EVIDENCE`
and `VAR-0047-W2-OWN-PAGE-DETAILS`. In planned
`IT-0047-W2-BRANCH-LINEAGE`, `FX-0047-W2-BRANCH-LINEAGE` replays
allocation of one legacy record under the same site. It keeps
one branch ID, the original lineage and old snapshot bindings.
With one slot free under the fixture's admitted cap, commit a
different authorised branch before allocating the older record.
The losing allocation commits no branch, page binding or active
totals. An otherwise eligible record remains readable as history
through its admitted reader. Give a separate unsupported reader
case and withhold the record's values and all dependent outputs.
Retry under admitted capacity and confirm the exact branch and
page map before new totals. Identify the same original record
on replay and keep one allocation. For the held cases, 0095
F-0347 keeps independently eligible rows and neither case admits
a branch comparison, draft or markup proposal or effect. A similar
address creates no match. A stale page-map retarget fails.
An authorised synthetic retarget invalidates old plans,
findings and markup grants. Restore cannot revive a retired branch. Compare
current, missing, unreadable and stale page facts separately.
No fetch, native write or provider action follows.

### Market totals after version 1

A market with too little evidence is counted: "2 markets
match, 1 differs, 3 not enough evidence". It is never dropped or counted
as zero.

### Shop details

A shop tells the same story in several places. This report lays them side
by side and reports where they differ, naming each source. It reports
mismatch, never money.

Example: the page and its markup say 18.00; the shop record says 21.00,
both looks within the allowed gap. Report: "Price differs on 31 products:
page 18.00, markup 18.00, shop record 21.00." A product missing its stock
status gets "not enough evidence: availability missing".

1. Applies to WordPress sites with WooCommerce active. A site without a
   shop shows "no shop found", not an error or zero. XenForo shops are not
   covered in version 1.
2. The views of an offer: the page (what the visitor sees), the markup
   (product data declared for search engines), the shop record (read
   through the read-only product operation in the WordPress plugin,
   which needs its own approval) and an optional product feed at one
   admin-named address on an allowed host. Accepted feed formats, schemas,
   size limits and parser fixtures are readiness blockers owned by the shop
   integration record; no feed can build or run until they are admitted.
3. A missing view shows as missing. If the shop record cannot be read,
   page and markup are still compared and neither is picked as right.
   Each fact keeps its source, read time and page version. Old values are
   kept for the history days and never overwritten.
4. Six material facts are needed for a verdict: price, currency,
   availability, shipping, returns and seller identity.

| Verdict | Shown when |
| --- | --- |
| All agree | Every present view agrees and the six facts exist |
| Differs | Views disagree within the allowed gap; all values shown |
| Recheck due | The looks were too far apart to compare |
| Not enough evidence (missing ...) | A material fact is missing or too old |

5. No offer score. Four plain facts show side by side: how new, how
   complete, how many views agree, how many independent views exist.
6. Each fact has its own freshness rule. Admins may tighten one but never
   go below the safe bottom.
7. Two looks compare prices only within the allowed gap and in the
   same currency. Outside the gap show "recheck due" with an offer
   to re-read those pages. Currency remains an observed offer fact.
   Currency conversion, unit rules and market currency verdicts
   are after version 1 (feature 560, spec 0102).
8. A stale fact keeps its last value, dated: "current price: not enough
   evidence; last seen 21.00, 17 hours ago".
9. If a record or feed has not changed for far longer than its usual
   rhythm (the median gap between its changes), that is one "shop record
   stale" finding with a product count.
10. Checks: price differs across views; unavailable or removed products
    still in the sitemap or marked in stock; a variant page whose
    canonical points at its parent while it carries its own price;
    inconsistent currency labels between views of the same offer;
    missing shipping or returns facts; stale availability. Findings group
    by product layout or by the wrong record field.
11. The product operation and this report never calculate revenue,
    order value, conversion, ROI or expected value, suggest a price,
    read orders or customers, read rival prices, marketplaces or
    shopping engines, or grade reviews. A
    person-set sales label belongs to link ranking (spec 0029). Visits and
    impressions to affected pages may show as plain counts.
12. Live-shop safety: the record reader runs off-peak, in small pages, at
    a rate setting, under the slowing rule (slows at 10% above normal,
    pauses at 25%), with a product cap per run. It reads the owner's own
    shop only and writes nothing to the shop.

The same integration hosts `ReadShopSalesDay` under specs 0021 and
0103. Its separate sales approval and aggregate contract never extend
the product operation's fields. Sales work shares the live-shop
slowdown guard, with its own admitted admin-side footprint bounds.
Refuse sales work when there is no budget or when the guard pauses.
Keep product checks available under their own guard and approval.

#### Wave 2 own-shop price and availability evidence

F-0585 · REQ-0047-W2-SAME-OFFER-CONSISTENCY ·
DES-0047-W2-ADMITTED-OFFER-OBSERVATIONS. Use admitted same-site
WooCommerce page, markup and independently approved read-only shop
facts. Compare only the exact product and variant, same currency
for price comparison, current evidence and the admitted time gap.
Keep the six material-fact gates above for the full verdict. Missing
records do not stop an evidenced page-and-markup comparison or prove
which source is right. A stale value is last seen, never current.

FX-0047-W2-SAME-OFFER-CONSISTENCY · IT-0047-W2-SAME-OFFER-CONSISTENCY
reuses Shop details' missing-fact, comparison-gap, staleness,
disagreement and read-only record checks. Give one comparable
disagreement, a repeated observation, different variants, expired
price and missing stock, plus absent shop-operation approval. Show
one scoped owning finding for the proved disagreement with the
values and sources, and hold any unsupported verdict. Use stand-ins
for all source reads. No product write, sales data, currency
conversion or public effect. The native reader and optional feed
keep their own approvals, profile, measured bounds and footprint.

### Weak pages and falling pages

Two lists and one switch. "Weak pages": pages few pages link to, and pages
that lead nowhere. "Falling pages": pages whose search demand has fallen
for long enough against their own past. The switch says whether a run may
use search numbers.

Run mode:

1. Each run is "structure only" (the default) or "with search data",
   chosen by the admin and stored on the run and every finding.
2. Structure only uses the crawl's link graph alone and claims nothing
   about visits, search demand or forecasts.
3. "With search data" adds impressions and visit totals (no money) for
   ordering and the filter. If the numbers are missing or incomplete, the
   run is refused with the reason; it never turns into structure-only by
   itself.
4. Falling pages always need search data; in structure-only runs they
   show "not measured".

Weak pages:

1. Few links in: how many guest-visible pages link to it in their
   content, apart from menus and lists. Zero links in, few links in,
   great click depth and low internal PageRank are four facts; each row
   names the one that applies. None is called "unreachable".
2. No way on: no content link to another guest page, or only menu links.
   Pages pointing only to unlike topics are shown apart, not as dead
   ends.
3. A page may be on both lists as two findings. Counts are against the
   named crawl.
4. Pages not meant to be found (noindex, canonical elsewhere, a later
   page of a list) appear in their own labelled group and stay in the
   graph. Only guest-visible pages exist here.
5. Order: by one counted fact the admin picks (fewest links in, deepest,
   most impressions with search data). No combined score. Until research
   proves an order, the screen says "not ranked; sorted by the fact you
   picked".
6. "Show link suggestions" on each row opens the existing suggestions
   screen (spec 0028) for that page, using the existing relevance gate
   and topic area; a moderator approves each link. No second suggestion
   engine and no new input to job 324.
7. With search data only: a page is hidden from the default list only if
   its impressions and human visits are both verified zero over a whole
   period. The screen shows the hidden count and "show all". Missing,
   partial or unauthorised numbers never count as zero. The filter
   deletes nothing and stops no check.
8. After a fix: when a person approves a link from a row, the page and
   rule version are recorded. After the next crawl, check my fix re-reads
   just those pages. "Solved" means the rule no longer fires, not that a
   count rose. Search demand afterwards is compared with the page's own
   earlier level; missing data gives "inconclusive". It is never called
   caused by the link.

Falling pages:

1. One metric per run (impressions by default), from one source at a
   time, never mixed. Only whole weeks count; a part week, missing day or
   late copy is not zero.
2. Own baseline, frozen at the run's cut-off. Never a site average,
   another page or one universal level. A changed baseline makes a new
   comparison with a visible seam. A redirect, merge or canonical change
   restarts the page's history and says so.
3. Support gate: too little volume or history gives "not enough evidence:
   too few impressions", never a slope.
4. The robust trend fit is a readiness blocker owned by the falling-pages
   research record. It must name one deterministic method, parameters and
   fixtures proving that one spike cannot raise a finding before this report
   can build. Odd points stay visible with reasons.
5. A finding needs the fall, the support and persistence together:
   several whole windows in a row. A gap breaks the run.
6. Units stay apart: observed counts; slope (counts per week); rate
   (slope against the own baseline, per week); change over a stated
   length. Weekly counts 50, 45, 40, 35 with baseline 50 give slope -5 a
   week, rate -0.1 a week and three-week change -0.3. A zero or negative
   baseline gives "rate unavailable".
7. Many pages falling together form one group, "many pages falling", with
   every page row kept; overlapping groups use unique page totals.
8. Context, never cause: link changes between crawls, a new layout
   revision, a page edit, a competing page (job 265), a search-update
   note. A finding with no supported fix stays a diagnostic; a fix goes
   to the Coach or suggestions for a person to approve. "Report" is the
   only action.
9. A sustained fall stays a report until research shows few false alarms.

#### F-0003 verification binding

`REQ-0047-ORPHAN-EVIDENCE` uses `DES-0047-ZERO-CONTENT-INLINKS`
in Weak pages. An authorised site admin reads a finding for a
current guest-visible page from the named crawl generation. Count
qualifying source pages with content links to that destination.
Exclude menus and lists from this count. A page is eligible for the
zero-inlinks fact only when the admitted coverage proves zero over the
named population. Unknown coverage does not prove zero, and zero
content inlinks does not establish that the page is unreachable.

- `VAR-0047-ORPHAN-STRUCTURE` uses the stored crawl graph and its
  declared population, with the existing structure-only report.
- `VAR-0047-ORPHAN-SEARCH` requires compatible, complete admitted
  search and human-visit inputs. Use the existing fact sort and the
  two-verified-zero display filter only. A missing required measure
  refuses this mode with its reason. Do not fall back.

Use the existing 0047 site-scoped report and 0028 suggestions screen.
The report opens no connection and adds no public-page work. The
existing relevance gate and moderator decision handle any proposed
link. Keep the crawl's zero fact separate from few links, high depth,
low internal PageRank, demand and later remediation. Record approved
remediation and check it against the next crawl as specified above.
Neither a report filter nor a missing measure changes ranking inputs.

`BDD-0047-ORPHAN-REVIEW` binds the existing Weak and falling pages
checks. Give one page proved zero content inlinks, one menu-only page,
one unread page with an observed inbound edge and one incomplete
source inventory. Check the labelled population, unknown handling and
the exact handoff to existing suggestions. In search mode, test two
verified zeroes, absent demand, and Show all. Approve a fixture link
through 0028, re-read via the next crawl and require "Solved" only
when the original rule no longer fires.

Both modes require admitted native crawl coverage and report memory
proof. The exact zero-coverage predicate and native projection fixtures
must be admitted before deriving work. The blocked few-links threshold
applies to the distinct few-links fact. This design does not pick it or
bypass admission if the report couples the two facts today.

#### Wave 2 navigation-only inlinks

F-0140 · REQ-0047-W2-NAVIGATION-ONLY-INLINKS ·
DES-0047-W2-COVERED-INLINK-COMPARISON. Use the 0044 observed graph
and F-0003's admitted source population. For the exact guest-visible
destination, require an observed navigation inlink and proved zero
eligible body inlinks across complete comparable coverage. The outbound
“No way on” report keeps its own meaning.

FX-0047-W2-NAVIGATION-ONLY-INLINKS specialises BDD-0047-ORPHAN-REVIEW.
Compare a navigation-only destination, one with an observed body inlink,
one with no observed navigation inlink and one with an unread source.
Only the first yields this finding. Incomplete coverage stays unknown.
Placement advice uses current relevant evidence and the existing 0028
suggestion flow. The report starts no fetch and inserts no link.

### Additional technical checks (635–645)

Run on retained observations under the existing report caps.
A finding carries site, feature, subject address or object ID,
variant, exact evidence IDs and time, profile revision, coverage,
measured value where available, and the proposed manual step.
Show generations and source scopes separately. Require guest-visible
evidence and current actor and 0108 site scope before release. Use
privacy-safe projections. Exact query values used in a protected
comparison never appear in the report. Recheck rights and 0032
erasure before showing active or historic evidence. The methods
below do not change crawler admission, robots, indexing or ranking.

- **635, duplicate articles, tags and archives.** Compare admitted
  own-site pages using visible main text after boilerplate removal.
  Exclude thread and WordPress comment variants. Allow cross-class
  pairs only within equivalent language and extraction profiles.
  Declare skipped classes and pairs as coverage limits. Profile v1
  uses NFC, default full case folding, NFC again and collapses runs
  of Unicode whitespace. Exact comparison needs non-empty matching
  normalised text at the admitted length and complete main content.
  Similarity is the intersection size divided by union size for
  sets of contiguous three-word groups. Use [Unicode 17.0.0 word
  boundaries][structure-words], keeping segments with at least
  one letter or number. An empty union is unknown. Near-duplicate
  output waits for admitted minimum text, threshold and bounded
  candidate pool against the 0116 fixtures. Unread or insufficient
  content stays unknown. Keep similar pairs as
  pairs. Overlapping pairs cannot prove that every member of a
  larger group is equivalent. Show canonical and index hints as
  separate evidence. No consolidation recommendation for genuinely
  distinct purposes without human review.
- **636, insecure outbound anchors.** Resolve the observed anchor
  through 0107 against the final document and its effective
  admitted base. A scheme-relative anchor inherits that base's
  scheme. Invalid, unadmitted or conflicting base evidence gives
  unknown when resolution depends on it. Flag resolved external
  HTTP targets using the active site's admitted origin and alias
  list. Show the local occurrence and a redacted target without
  credentials, tracking or private query values. Exclude non-web
  schemes. Do not open the target, infer a redirect or replace it.
  An HTTPS alternative requires separately supplied evidence.
- **637, page structure size.** Use 0115's observed DOM element
  count for a comparable completed render profile. Exclude text
  nodes and inert template contents. Count visited open shadow
  elements once by node identity, including each slotted element.
  Report closed and unvisited roots as coverage gaps. Count the
  frame element in the parent and its document separately. A
  stopped run has an interim observation with the limiter and
  capture stage. Its tree may later grow or shrink. A threshold
  finding needs complete coverage for the stated settled profile.
- **638, request count.** Use only broker request facts from one
  0115 render, grouped by device, cache and consent mode and
  settle window. Deduplicate actual outbound attempts by broker
  attempt ID. Count the main request, admitted frame requests,
  sent redirects, retries and conditional revalidation including
  304 responses. Count cache hits, offline replays and policy
  refusals before send separately. No extra render or request.
  A capped run shows the lower bound for attempts by capture time
  and why the comparable settled count is unavailable. For 637
  and 638, “large” or “many” needs an admitted report threshold. A render
  safety cap alone cannot define a performance threshold.
- **639, main content inside a frame.** Use the parent reading
  landmarks, visibility and admitted frame observations. Distinguish
  a visible primary region containing frame-hosted main content
  from decorative, hidden or supplemental embeds. Require a
  visible substantial parent reading landmark and admitted child
  main-content semantics for the same snapshot. Geometry alone
  cannot prove this. A mismatched or unavailable child is present
  and unread or unknown. No frame fetch or bypass to inspect it,
  and no conclusion about search indexing.
- **640, blocked internal link.** Join the observed request to the
  existing valid 0082 decision. Match scheme, host, effective port,
  encoded path, applicable query and crawler product token and
  group. Use exact [robots protocol semantics][site-robots-standard].
  Do not substitute a canonical or deduplicated address. An
  explicit rule disallow is distinct from local deny on 401,
  oversize or unreachable policy. Unknown or stale policy stays
  unknown. A redirect destination needs its own admitted evidence.
  Never infer index state, noindex or a blocked target's status.
- **641, blocked address in a sitemap.** Use the same robots
  decision for each observed sitemap page entry from 0044. Show
  the file and entry plus the observed disallowing rule and agent.
  Keep sitemap index entries separate from listed page URLs. A
  conflict calls for owner review of the intended visibility.
- **642, sitemap without a known discovery route.** Compare the
  existing sitemap inventory against its observed 0044 robots
  declarations, configured seeds, sitemap index references and
  exact site and property submissions from admitted 0040 evidence.
  Show Sutradhar seeds, public declaration paths, provider
  submissions and provider processing status independently. An
  index cycle cannot establish reachability from a root. A complete
  local sample without a public root path can show “No link found
  in this local sample” even when provider evidence is unavailable.
  Incomplete local or provider coverage is unknown for that source.
  An out-of-scope file is neither fetched nor labelled orphaned.
- **643, important text visible only in an image.** Use a current
  visible image, readable owner-supplied transcription or separate
  admitted local extractor. Match image digest, extractor version
  or evidence revision, and exact render landmark. Report text
  visible only as pixels separately from whether nearby text,
  alternative text or accessible name gives an equivalent. Scope
  absence to the complete checked region. An unread image stays
  unknown. A declared decorative image with material text goes
  to review. Keep transcriptions within existing protected evidence.
  No upload, automatic rewrite or generic alt-text replacement.
- **644, WordPress comment pagination.** Use native page class,
  post ID, install route revision and the observed native page
  identity, including `cpage` or its pretty permalink form. Keep
  stored canonical and reply destination and anchor distinct from
  comment page identity. Compare article repetition and comment
  content separately. An exact duplicate requires equal complete
  guest-visible article and comment content. A complete empty
  comment region differs from an unread one. Distinct comments
  are retained as distinct evidence. No private or pending reads.
  Preserve permalink and reply accessibility in a manual proposal.
- **645, WordPress crawl shaping.** Provide a site-owner review
  showing observed reply variants, attachment pages, feeds and
  internal search pages as separate types. Bind WordPress, theme
  and plugin revisions plus the owner's intended crawl, indexing
  and access for each class. Show observed index, crawl, canonical
  and sitemap facts separately and explain admitted manual choices.
  Propose preserving guest reply and comment links, media rights
  and useful feeds. No broad rule for every parameter or attachment.
  A crawler block cannot stand in for a readable noindex, and
  no rule substitutes for access control. Include before/after
  checks for headers and permalinks and a restore step against
  the original settings or template version. Incomplete or conflicting
  plugin/theme
  evidence gives a review task without a ready change. The
  owner applies any chosen change in WordPress. No apply path
  for crawl or index settings and no Search Console submission.

The baseline navigation evidence for 644 comes from the published
[WordPress comment pagination contract][wp-comment-pages].
The [Google sitemap guide][google-sitemap-build] distinguishes
publication, discovery hints and acceptance. The [Google noindex
guide][google-noindex-rule] distinguishes readable indexing
directives from robots blocking. These sources were checked on
20 November 2026. No source warrants a prediction of indexing.

#### Wave 2 stored technical observation bindings

Use synthetic retained own-site observations under the exact 0047
site, actor, evidence and capture scope. QR-158 still owns report
thresholds and native evidence. QR-159 platform and manual-plan
qualification supplies no extra read or effect. Preserve the 0044,
0040, 0082 and 0115 source contracts. These checks use no new
platform scan, network read, submission or native change.

| Feature | Owning requirement and design | Planned fixture and expected evidence |
|---|---|---|
| F-0635 | REQ-0047-W2-NON-THREAD-DUPLICATES · DES-0047-W2-SCOPED-MAIN-TEXT | FX-0047-W2-NON-THREAD-DUPLICATES · IT-0047-W2-NON-THREAD-DUPLICATES. Compare eligible article and tag or archive records under the same admitted extraction and language profile. Reuse 0115 RN-27 and UT-RN-23 for main-text separation. Include boilerplate-only overlap, a related but distinct pair, an excluded thread and incomplete coverage. Group supported complete non-empty matches only. Similarity requires its admitted minimum, threshold and pool and remains a pair. Unknown never creates a match. |
| F-0636 | REQ-0047-W2-INSECURE-OUTBOUND-OBSERVATION · DES-0047-W2-EXACT-ANCHOR-RESOLUTION | FX-0047-W2-INSECURE-OUTBOUND-OBSERVATION · IT-0047-W2-INSECURE-OUTBOUND-OBSERVATION. Use stored anchors and 0107 resolution with HTTP and HTTPS effective bases. Try an invalid base, same-site alias, protected target field and no separately proved HTTPS alternative. Flag only an admitted resolved external HTTP address. Display safe source occurrence and redacted target. Unknown stays unknown. No follow-up read, redirect inference or replacement. |
| F-0637 | REQ-0047-W2-SETTLED-STRUCTURE-SIZE · DES-0047-W2-EXACT-RENDER-COVERAGE | FX-0047-W2-SETTLED-STRUCTURE-SIZE · IT-0047-W2-SETTLED-STRUCTURE-SIZE. Count open shadow and slotted elements once by node identity, excluding text and inert templates. Distinguish the parent frame element and its document. Compare closed or unread roots, a shrinking capped tree and a settled count at the admitted threshold. Show the partial stage and limiter without a final count claim. Reuse RN-05/28 and UT-RN-04/24. |
| F-0638 | REQ-0047-W2-OBSERVED-REQUEST-COUNT · DES-0047-W2-EXACT-RENDER-COVERAGE | FX-0047-W2-OBSERVED-REQUEST-COUNT · IT-0047-W2-OBSERVED-REQUEST-COUNT. Use stored 0115 broker evidence in one exact render and device, cache, consent and settle profile. Count sent distinct attempts, including revalidation with 304. Separate cache hits, pre-send refusals and offline replay. Deduplicate the same attempt ID. Interrupted capture shows observed attempts and a missing settled total. No “many” finding without an admitted report threshold. Reuse RN-03/05 and RN-26. |
| F-0639 | REQ-0047-W2-FRAME-MAIN-CONTENT · DES-0047-W2-EXACT-RENDER-COVERAGE | FX-0047-W2-FRAME-MAIN-CONTENT · IT-0047-W2-FRAME-MAIN-CONTENT. Give a supplemental embed, a visible parent reading landmark with admitted child main content, a covered frame and an unread foreign child. Require compatible complete parent and child observations for the supported result. Geometry alone proves no main content. Unknown child evidence yields unknown. Reuse RN-09/34 and UT-RN-07/27. |
| F-0640 | REQ-0047-W2-EXACT-ANCHOR-ROBOTS-FACT · DES-0047-W2-OBSERVED-ROBOTS-COMPARE | FX-0047-W2-EXACT-ANCHOR-ROBOTS-FACT · IT-0047-W2-EXACT-ANCHOR-ROBOTS-FACT. Reuse 0082 LV-R01 and UT-LV-01 with stored decisions for the exact scheme, host, port, encoded path, applicable query and crawler group. Compare admitted precedence, a port change, local 401 hold and stale policy. Only an explicit current rule gives a disallow finding. An unknown or expired policy and a local hold retain their separate meanings. No raw query, canonical substitution or index verdict. |
| F-0641 | REQ-0047-W2-EXACT-SITEMAP-ROBOTS-FACT · DES-0047-W2-OBSERVED-ROBOTS-COMPARE | FX-0047-W2-EXACT-SITEMAP-ROBOTS-FACT · IT-0047-W2-EXACT-SITEMAP-ROBOTS-FACT. Use exact eligible 0044 observed page entries with allowed, explicit disallow and ambiguous 0082 decisions and a sitemap index entry. Reuse the same rule and agent decision as F-0640. Show file, entry and disallow evidence only for the supported conflict. An index-file reference or unknown policy supplies no page finding or indexing outcome. |
| F-0642 | REQ-0047-W2-SITEMAP-DISCOVERY-FACTS · DES-0047-W2-OBSERVED-SITEMAP-SCOPE | FX-0047-W2-SITEMAP-DISCOVERY-FACTS · IT-0047-W2-SITEMAP-DISCOVERY-FACTS. Compare stored public declaration, configured seed, internal index cycle and no eligible public root path in a complete local sample. Keep 0040 exact-property submission and processing evidence separate from 0044 observed inventory. Missing provider scope stays unknown. No out-of-scope orphan label, fetch or submission. Reuse 0115 RN-23 and F-0117 source coverage. |
| F-0643 | REQ-0047-W2-IMAGE-ONLY-TEXT-EVIDENCE · DES-0047-W2-ELIGIBLE-IMAGE-EQUIVALENTS | FX-0047-W2-IMAGE-ONLY-TEXT-EVIDENCE · IT-0047-W2-IMAGE-ONLY-TEXT-EVIDENCE. Compare a decorative image, visible material text evidenced by eligible supplied transcription or separately admitted local extractor, a proved adjacent or accessible equivalent and an unread image. Match the exact digest, evidence version and rendered landmark. Scope missing equivalent to a complete checked region. Keep unread evidence unknown. Reuse 0115 RN-09/12/39. No invented text, upload or alt-text edit. |

[structure-words]: https://www.unicode.org/reports/tr29/tr29-47.html
[site-robots-standard]: https://www.rfc-editor.org/rfc/rfc9309.html
[wp-comment-pages]: https://developer.wordpress.org/reference/functions/paginate_comments_links/
[google-sitemap-build]: https://developers.google.com/search/docs/crawling-indexing/sitemaps/build-sitemap
[google-noindex-rule]: https://developers.google.com/search/docs/crawling-indexing/block-indexing

### Platform reports and review plans (646–655)

Read existing same-site records and authorised supplied observations.
Bind site, platform, native object IDs, source revisions, comparison
window, guest scope, coverage and permitted use to each report.
A new theme, plugin, permission or evidence revision holds affected
plans for review. Apply 0020 actor and action rules, 0108 site
isolation, 0078 limits and current 0032/0118 rights and erasure
generations before preview, copy, export or restore. Retire prior
plans on revocation even after a policy rollback. Reports are
observations or manual plans, with zero public-page footprint.

- **646, views against search traffic.** Compare a XenForo thread's
  0021 native view-counter increase between two comparable captures
  with admitted 0040 final Web Search click observations for the
  exact authorised property, page aggregation and Google canonical
  mapped to the native thread. Pin matching native counter boundaries
  at the start and end of whole Search Console reporting dates in
  America/Los_Angeles, with local clock rules and measured native
  flush delay admitted. Any delayed batch, reset, import, migration,
  changed counter policy or uncertain mapping holds classification.
  Final Search Console data may still omit pages or days. Use one
  qualified source grain, never combine query/device/appearance
  slices or fill gaps. Show the observed subtotal and coverage when
  complete counts are unproved. Only a qualified disclosed cohort
  sample can classify. Label native events and provider clicks
  separately, with window and scope. There is no person count or
  subtraction or ratio across the two sources. The [Search Analytics
  contract][gsc-query] describes reporting dates and dataset limits.
  With enough eligible threads under an admitted cohort
  profile, use inclusive lower and upper quartiles for each count
  and report high views with low clicks and the reverse. Use
  the values at ranks ceil(n/4) and ceil(3n/4) in each ascending
  cohort vector, with ranks starting at one. Equality at a bound
  is included. If bounds tie, classify neither tail for that
  metric. An undersized cohort has no classification. Show scope
  and thresholds. No blanks
  replaced by zero, cross-section comparison or ranking input.
- **647, Search Console per forum section.** Start from the admitted
  0040 property and the 0021 public section-to-thread membership.
  Filter locally by stable section ID and the approved nested-section
  policy under current actor and 0040 connection generation. Admit
  only proved guest membership throughout the period. Label moved,
  hidden and undetermined membership separately without revealing
  hidden objects. A manual URL-prefix plan needs proved literal
  HTTPS scheme, host and slash-bounded path for the intended routes.
  Show exclusions, overlaps, nested sections and global thread
  canonicals. A prefix cannot stand in for membership. If coverage
  fails, use the local report. Show the eligible literal prefix
  and the [provider's Add property steps][gsc-add]. The owner adds
  and verifies it in their account. A later read needs a separately
  selected and admitted 0040 binding. This feature adds no
  multi-property support, OAuth scope, account write, verification
  or sitemap submission. Never sum overlapping property totals.
- **648, XenForo thread prefixes.** Require the native current
  per-thread edit decision and allowed prefix ID under forum and
  user-scope policy. Use decisions without group membership lists.
  Keep the stored title, rendered document title and visible
  heading separate. Show how the prefix renders in the current
  theme, including omission or existing repetition. Propose only
  a permitted useful topic or type label with direct evidence.
  Show the exact proposed fields and retain other fields. Staff
  make any change through the native editor. Unknown authority,
  stale evidence or traffic alone cannot justify a prefix. The [XenForo prefix
  guide][xf-prefix]
  describes native availability and display.
- **649, WordPress category and tag planner.** Review object IDs,
  hierarchy, post membership and exact archive paths under the
  active taxonomy. Keep hierarchical categories and flat tags
  distinct. Propose candidate moves or merges with unique affected
  posts, slugs, archive and link changes and redirect risks. Use
  only current supplied evidence and eligible local observations.
  Bind the complete changed scope and public membership snapshots.
  Block cycles, reserved or occupied paths, ambiguous mappings,
  hidden memberships and changes in guest visibility. Avoid
  disclosing blocked objects in counts or previews. Show exact
  supported before and after trees and archive mappings locally.
  Bind the map and any supplied membership receipt to the current
  admitted taxonomy revision and source scope. Recheck before
  showing a saved proposal as current. If the map changed, show
  “The archive mapping changed. Review the current map before
  planning the move”. A matching term ID and slug cannot carry
  the old preview forward. An unproved map or membership gives
  unknown current effects. Only independently eligible dated
  history may still show.
- **650, permalink structure.** Compare the exact native current
  and proposed route profiles, including trailing slash, taxonomy
  bases, localisation and supported rewrite variants. Produce a
  locally reviewable old-to-new mapping keyed by native object,
  guest scope and revision. Use 0107 for exact route checks. Hold
  on unsupported locale routes, a reserved path, private target,
  slash/host/query alias collision, unresolved redirect chain or
  unknown important source. Respect DR-081 scope. Report required
  redirect, canonical, sitemap and post-change checks. Give exact
  preconditions and a guarded manual restoration of the original
  settings if the plan fails. Generate no configuration. The owner applies and
  checks changes on
  the site using [WordPress permalink settings][wp-permalink].
For 651–653, reuse only the typed 0044 and DR-110 protected staging
projection already admitted to the exact install, project and scope.
A new scrubbed field needs QR-159 admission and a matching active
owner manifest before opening the file. A denied or expired grant,
wrong install, unknown alias, secret or failed scrub withholds the
comparison. No raw staging copy, authentication or connection.

- **651, classic editor to blocks.** Compare the same WordPress
  page and intended variant before and after conversion. Use an
  approved local staging export per 0044 and DR-110 for protected
  previews. Check admitted findings for headings, links, image
  equivalents, structured facts, metadata, canonical, public
  visibility and unsupported shortcodes. Unadmitted fields are
  unavailable. Do not read raw shortcode or protected markup. Distinguish
  changed from unavailable. Show the
  original revision and a restore instruction. No conversion,
  write, or authenticated crawl.
- **652, XenForo upgrade.** Compare a known production baseline
  with an approved supplied test-installation capture, including
  admitted staff declarations of platform, add-on, theme and
  route versions. Check public page classes, permissions, links,
  canonical, robots, sitemap, redirects and structured findings.
  For the add-on accept only an independently attested, redacted
  offline handshake receipt bound to the test install and versions.
  No credentials or live handshake are read. Show prerequisites, backup receipt,
  tested restore and unresolved release stops. The [XenForo upgrade
  guide][xf-upgrade] calls for a backup and restore if the upgrade
  needs to be reversed. Never present an in-place downgrade.
  The owner operates the native upgrade tools.
- **653, WordPress theme switch.** Compare matched guest render
  and native data from the live baseline and approved supplied
  preview. Include page classes, templates, mobile profile,
  menus, headings, visible content, links, metadata, structured
  markup, accessibility and render coverage. A preview is protected
  under DR-110. A material missing variant, activation hook or
  plugin dependency requires review. Show a manual restore plan
  for the old theme, settings and tested backup. No activation
  or external request.
- **654, possible image reuse.** The paid reverse image provider
  and automatic monitoring are outside the admitted 0100 routes.
  Offer a local board of authorised supplied observations for
  owner-owned or licensed assets under 0097. Each record binds
  asset digest, inert scrubbed occurrence URL, source, observation
  date, owner asset rights and separate evidence-use permission.
  A hash match or independently verified matching rendition
  establishes only an observed match. Similarity goes to review.
  An occurrence cannot establish a licence or infringement.
  Show observation and rights separately. Recheck both grants
  before release and retire revoked evidence across reports.
  No URL resolution, submission, charge, scheduling or outreach.
- **655, image context.** Use the 0115 visible figure, caption,
  accessible name and text in the same eligible reading region.
  Show whether the same rendered figure is referenced and whether
  its caption or nearby text explains the observed role. Report
  visible association separately from the accessible name and
  equivalent. Unread accessibility facts stay unknown. Check
  asset and region revisions and 0097 permission. Private metadata
  cannot supply person identity or other unsupported facts. An unread figure
  or ambiguous purpose needs review.
  Offer a local caption or context edit suggestion only. No
  unproved identification, image upload, or application.

Official sources reviewed on 20 November 2026 are pinned by
their links for the first supported profiles. Use qualification
to tie relevant platform behaviour to each admitted build.

[gsc-query]: https://developers.google.com/webmaster-tools/v1/searchanalytics/query
[gsc-add]: https://support.google.com/webmasters/answer/34592
[xf-prefix]: https://docs.xenforo.com/manual/forums/threads/thread-prefixes
[wp-permalink]: https://wordpress.org/documentation/article/settings-permalinks-screen/
[xf-upgrade]: https://docs.xenforo.com/manual/installing-and-upgrading/upgrade

#### Named platform report and manual review fixtures

These planned spec 0076 cases reuse synthetic existing records
and owner-approved source profiles. QR-158 and QR-159 hold
exact evidence, permissions and platform proof. The named
companion tests retain their own scope.

- `DES-0047-W2-WP-MANUAL-REVIEW` and
  `DES-0047-W2-NATIVE-MANUAL-PLAN` compare exact native
  identities, visibility and current grants. Reports and
  manual plans give no route, setting or publication effect.
- `DES-0047-W2-LOCAL-SEARCH-REPORT` keeps the native 0021
  counter and admitted 0040 Search Console rows separate.
  Reports do not add OAuth, verification, polling or a write.
- `DES-0047-W2-SUPPLIED-STAGING-REVIEW` uses only the typed
  0044 export with the current owner manifest and DR-110
  binding. An unavailable field or denied install gives no
  authenticated read. A restore plan needs exact local proof.
- `DES-0047-W2-LOCAL-IMAGE-EVIDENCE` uses only authorised
  supplied observations or the existing 0115 figure evidence.
  The paid image route stays refused. An observed match gives
  no legal conclusion and no image or URL acquisition.

| Feature | Requirement | Design and variant | Fixture and planned test | Named fixture assertion |
|---|---|---|---|---|
| F-0644 | REQ-0047-W2-COMMENT-PAGES | DES-0047-W2-WP-MANUAL-REVIEW, VAR-0047-W2-COMPLETE-COMMENT-REGION | FX-0047-W2-COMMENT-PAGES, IT-0047-W2-COMMENT-PAGES | Match exact WordPress post and comment-page identity. Only equal complete article and comment regions establish duplication. Keep complete empty and unread distinct. XenForo and pending or private comments give no comparison. |
| F-0645 | REQ-0047-W2-CRAWL-SHAPING | DES-0047-W2-WP-MANUAL-REVIEW, VAR-0047-W2-NATIVE-VISIBILITY-PLAN | FX-0047-W2-CRAWL-SHAPING. The shared check is BDD-0047-W2-CRAWL-SHAPING | Compare reply, attachment, feed and search variants separately. A conflicting plugin or changed baseline holds the plan. A robots block cannot stand in for readable noindex or access control. No native setting or submission changes. |
| F-0646 | REQ-0047-W2-VIEWS-CLICKS | DES-0047-W2-LOCAL-SEARCH-REPORT, VAR-0047-W2-XF-WEB-PAIR | FX-0047-W2-VIEWS-CLICKS, IT-0047-W2-VIEWS-CLICKS | Compare exact final Web reporting dates with native counter boundaries, including Pacific clock changes. Compute inclusive tails at ceil(n/4) and ceil(3n/4) on each eligible vector. Equality to a distinct bound qualifies. Equal bounds, delayed counters or partial coverage hold classification. No cross-source ratio. |
| F-0647 | REQ-0047-W2-SECTION-GSC | DES-0047-W2-LOCAL-SEARCH-REPORT, VAR-0047-W2-PROVED-SECTION-SCOPE | FX-0047-W2-SECTION-GSC, IT-0047-W2-SECTION-GSC | Compare one admitted section plus its selected nested policy against an overlapping or stale property. Match literal HTTPS slash boundaries for the manual plan only. A moved thread without complete period membership stays outside the section total. Hidden identities stay withheld. |
| F-0648 | REQ-0047-W2-XF-PREFIX | DES-0047-W2-NATIVE-MANUAL-PLAN, VAR-0047-W2-CURRENT-PREFIX-PREVIEW | FX-0047-W2-XF-PREFIX, BDD-0047-W2-XF-PREFIX | Pin the allowed native prefix for the exact thread and actor. Show stored title, rendered title and visible heading separately. Show a theme omission as observed. A stale title or revoked authority holds review. No title write. |
| F-0649 | REQ-0047-W2-TAXONOMY | DES-0047-W2-NATIVE-MANUAL-PLAN, VAR-0047-W2-SCOPED-TAXONOMY-TREE | FX-0047-W2-TAXONOMY, BDD-0047-W2-TAXONOMY. Reuse 0044 IT-0044-W2-CRAWL-SEGMENTS for exact scope, generation and coverage of any supplied membership evidence. | Compare a valid category reparent with a flat tag, a cycle, occupied slug and hidden member. With matching current evidence, show the scoped before and after map. Change the native archive-map revision under the same category ID and slug. Hold the earlier preview from current use and keep only eligible dated history. A missing current membership receipt gives unknown effects. Block visibility changes without exposing private counts. |
| F-0650 | REQ-0047-W2-PERMALINK | DES-0047-W2-NATIVE-MANUAL-PLAN, VAR-0047-W2-CURRENT-ROUTE-MAP | FX-0047-W2-PERMALINK, IT-0047-W2-PERMALINK | Map exact current native objects to distinct eligible target routes. A stale redirect, reserved path or slash/query collision holds the map. Give guarded manual restore steps from the original revision only. No configuration or write. |
| F-0651 | REQ-0047-W2-BLOCK-MIGRATION | DES-0047-W2-SUPPLIED-STAGING-REVIEW, VAR-0047-W2-SCRUBBED-EDITOR-COMPARE | FX-0047-W2-BLOCK-MIGRATION, IT-0047-W2-BLOCK-MIGRATION. Reuse 0044 BDD-0044-STAGING for exact protected export and mapping admission. | Use the exact owner-bound scrubbed WordPress export and a matching classic baseline. Show loss of admitted alt evidence separately from an unavailable field. Wrong install, planted secret or incomplete capture gives no unsupported verdict. |
| F-0652 | REQ-0047-W2-XF-UPGRADE | DES-0047-W2-SUPPLIED-STAGING-REVIEW, VAR-0047-W2-OFFLINE-UPGRADE-CHECK | FX-0047-W2-XF-UPGRADE, IT-0047-W2-XF-UPGRADE. Reuse 0044 BDD-0044-STAGING for protected export admission. Test the offline handshake and restore evidence in the local review. | Compare versioned eligible test facts with the live baseline and a redacted exact-install offline handshake receipt. Wrong binding or absent tested restore holds the review. No credentials, handshake, upgrade or downgrade action. |
| F-0653 | REQ-0047-W2-THEME-SWITCH | DES-0047-W2-SUPPLIED-STAGING-REVIEW, VAR-0047-W2-MATCHED-THEME-PREVIEW | FX-0047-W2-THEME-SWITCH, IT-0047-W2-THEME-SWITCH. Reuse 0044 BDD-0044-STAGING for protected export admission. Reuse 0115 UT-RN-06 for mobile and optional desktop profile controls and UT-RN-16 for script-dependent differences. | Compare matched guest mobile and separately admitted desktop captures. Report proved navigation loss. Unknown activation hook, missing variant or expired manifest holds the manual plan. No theme activation or fresh render. |
| F-0654 | REQ-0047-W2-IMAGE-REUSE | DES-0047-W2-LOCAL-IMAGE-EVIDENCE, VAR-0047-W2-SUPPLIED-OCCURRENCE | FX-0047-W2-IMAGE-REUSE, BDD-0047-W2-IMAGE-REUSE. Reuse 0097 IT-0097-ASSETS for exact asset rights and expiry and 0100 SD-SCOPE for the excluded-route boundary. Test the paid image request in this fixture. | Give a verified copy, uncertain resemblance and withdrawn licence. Show dated observed match only under both asset and evidence grants. A paid monitor request, unsafe URL or revoked rights gives refusal. No infringement claim. |
| F-0655 | REQ-0047-W2-IMAGE-CONTEXT | DES-0047-W2-LOCAL-IMAGE-EVIDENCE, VAR-0047-W2-VISIBLE-CAPTION-CONTEXT | FX-0047-W2-IMAGE-CONTEXT, BDD-0047-W2-IMAGE-CONTEXT. Reuse 0097 IT-0097-ASSETS for exact asset rights and 0115 UT-RN-10 for accessible-name evidence. Test visible caption association in this fixture. | Compare an observed visible caption, hidden caption and separately proved accessible name on one figure revision. Reject ambiguous identity and withdraw stale context after a rights change. No image acquisition or edit. |

### Wave 1 site structure verification bindings

Use the current authorised site, exact admitted sample, native
revision and dated evidence. Show the observed population and
coverage. Missing, unread, withdrawn or incompatible facts stay
unknown. Report findings and supported local advice. The bindings
add no collection, native write, setting change or public footprint.

For F-0039, 0044 owns the observed response and 0115 owns the
rendered finding. Report the finding only when its response and
finished evidence share the exact admitted capture and page revision.
Two captures of the same page do not join merely because the status,
visible bytes or claimed revision match. If an eligible response
stands alone, show its observed status and say “We can’t confirm
whether this response showed an error page.” Show a recorded 0115
reason only from a separately eligible attempt. An unadmitted engine
gives SUT-1500 without a verdict. An observed broker refusal gives
SUT-1504 and only the permitted refusal details. An otherwise supported
partial finding stays at low confidence or below under RN-35. The
report and its recovery view start no fetch or render and add no
public-page work.

| Feature and requirement | Owning design and exact planned fixture |
|---|---|
| F-0037 · REQ-0047-W1-TAG-REVIEW | DES-0047-W1-TAG-EVIDENCE, FX-0047-W1-TAG-REVIEW, BDD-0047-W1-TAG-REVIEW. Compare exact duplicate tags, an ambiguous near-match, a sparse tag with unknown demand and a tag changed after preview. Show dated evidence and supported local merge or noindex advice. Do not infer low demand or choose a near-match rule without admission. QR-104 retains the separate 0052 draft and any platform merge authority; stale proposals cannot apply. |
| F-0038 · REQ-0047-W1-TITLE-COLLISION | DES-0047-W1-LAYOUT-EVIDENCE, FX-0047-W1-TITLE-COLLISION, BDD-0047-W1-TITLE-COLLISION. Compare observed exact and admitted near collisions, a distinct title and a page not read. Attribute evidence to the exact read layout revision. Reuse 0044 title facts. Hold unsupported similarity results and unread pages. No title copy or write; the 0041 evidence-backed wording exception keeps its separate owner and route. |
| F-0039 · REQ-0047-W1-SOFT-404-REPORT | DES-0047-W1-RENDER-EVIDENCE, FX-0047-W1-SOFT-404, shared 0115 RN-19/RN-35 and UT-RN-17. Contrast an observed missing-page message with HTTP 200, legitimate sparse content and a partial render. Report a supported finding with its source and confidence. Partial renders cannot exceed low confidence. Reuse the 0044 response facts and 0115 detector; no new classifier, fetch, status or indexing change. Keep the same eligible HTTP 200 response, and supply identical finished error text from a second capture of the same page. The report still shows the independently eligible observed status and withholds the joined finding. In the matching-capture control, show the detector's supported finding with its evidence and confidence. In separate current observations hold an unadmitted engine under RN-01/SUT-1500 and show a recorded blocked dependency under RN-03/SUT-1504. The first has no verdict. The second cannot raise supported partial evidence above low confidence. Opening or repeating the report causes no broker or render dispatch, status change or public-page work. |
| F-0040 · REQ-0047-W1-CANONICAL-CONFLICT | DES-0047-W1-LAYOUT-EVIDENCE, FX-0047-W1-CANONICAL-CONFLICT, IT-0047-W1-CANONICAL-CONFLICT. Compare admitted 0044 canonical, redirect, sitemap and internal-link observations for the same exact page. Omit one comparison on repeat. Show dated, supported contradictions and mark missing facts unavailable. With eligible redirect B disagreeing with canonical A, stage a report and withdraw only the redirect observation’s grant before release. In a separate replay, release the report with the grant current, withdraw it and reopen the saved report. In both cases leave the redirect evidence in storage for the test. Recheck current authority under 0082 before release or replay. Withhold B, its evidence reference and each contradiction that needs it. Keep independently eligible dated facts and any contradiction they still prove. Mark the affected comparison unavailable even when the remaining evidence shows no contradiction. No early or follow-up crawl, retry, canonical choice, redirect or write. |
| F-0041 · REQ-0047-W1-PAGINATION-HEALTH | DES-0047-W1-LAYOUT-EVIDENCE, FX-0047-W1-PAGINATION, IT-0047-W1-PAGINATION. Compare a supported paginated series, an observed contradictory canonical and an unread next page. Report the observed issue with the exact pair and preserve unknown on the unread endpoint. Use existing 0044 crawl facts. No follow-up crawl, markup or native change. |
| F-0042 · REQ-0047-W1-BROKEN-URL-DRAFT | DES-0047-W1-REDIRECT-EVIDENCE, FX-0047-W1-BROKEN-URL, BDD-0047-W1-BROKEN-URL. Compare an observed old URL with traffic or link evidence, one independently supported replacement, an ambiguous replacement and a loop. Use current allowed 0046 evidence. Show a traceable local draft only for the proved route. The feature gives no live redirect change. QR-104 retains native authority and public footprint qualification. |
| F-0043 · REQ-0047-W1-HREFLANG-DEFER | DES-0047-W1-LANGUAGE-REFUSAL, FX-0047-W1-HREFLANG-DEFER, existing IT-STRUCT-LANG boundary. Attempt to create a declaration or sitemap alternate under DR-081 and refuse. The same fixture's existing same-site observation and unread alternate belong to F-0564; their structural finding or “cannot confirm” is no F-0043 positive result. No fetching, adding markup or sitemap mutation. |
| F-0044 · REQ-0047-W1-LAYOUT-SHIFT-ADVICE | DES-0047-W1-RENDER-EVIDENCE, FX-0047-W1-LAYOUT-SHIFT, shared 0115 RN-10/UT-RN-08. Compare an observed resizing image, a stable reserved box and incomplete render evidence. Use the admitted render and label simulations. Show the finding and bounded manual advice where supported. Neither missing dimensions alone nor a partial render proves cause. No markup edit. |
| F-0045 · REQ-0047-W1-SPEED-PAGE-TYPE | DES-0047-W1-PAGE-TYPE-EVIDENCE, FX-0047-W1-SPEED-PAGE-TYPE, BDD-0047-W1-SPEED-PAGE-TYPE. Compare admitted thread and archive groups using the same device and compatible 0034 field profile, an unread page and a labelled 0115 simulation. Bind the captured native template revision. Keep coverage and sources distinct. Report observed group variation and no causal claim, blended values or additional measurement work. |
| F-0046 · REQ-0047-W1-GOOGLEBOT-REPORT | DES-0047-W1-LOG-EVIDENCE, FX-0047-W1-GOOGLEBOT, shared 0044 AT-SEO-LOG and 0034 AT-AN03/07. Distinguish a confirmed Googlebot line, a spoof and uncheckable evidence in a permitted sample. Stop consent before commit and vary the coverage boundary. Read only the already-admitted log classification and show counts for its actual sample and supported waste facts. A new live hook stays held under separate privacy, expiry and measured public limits in QR-106. |
| F-0047 · REQ-0047-W1-OUTBOUND-LABEL-REPORT | DES-0047-W1-LINK-EVIDENCE, FX-0047-W1-OUTBOUND-LABELS, BDD-0047-W1-OUTBOUND-LABELS. Compare supported sponsored, user-generated and editorial contexts with their observed marks and an ambiguous context. Report a mismatch only with exact context and mark evidence; a hostname proves no sponsorship. Do not alter attributes. Preserve 0029's distinct rule for links Sutradhar places. |
| F-0048 · REQ-0047-W1-SIGNATURE-PROFILE-AUDIT | DES-0047-W1-LINK-EVIDENCE, FX-0047-W1-SIGNATURE-PROFILE, IT-0047-W1-SIGNATURE-PROFILE. Use admitted visible signature, profile or author-link evidence, an eligible harmful-pattern fixture, a benign link and an unread profile. Withdraw visibility before delivery. Show protected staff evidence only for a current supported finding. No identity exposure, causal spam label, public check or member-link effect. |

The existing 0044, 0034, 0046 and 0115 checks retain their owners.
A shared fixture link gives no separate collector or detector
credit. QR-104 and QR-106 retain the exact sample, platform,
rights, source threshold and public-effect qualifications. These
fixture identities record planned structural evidence and refusals.

### Wave 1 protected audit bindings

Bind the active owned site, exact page and native revision, dated
source and capture profile, population, coverage and use rights.
These findings and local review proposals do not admit a new
render, request, provider check, public markup or native effect.
An incomplete or refused source is unknown within its missing scope.

| Feature and owning design | Exact planned fixture and boundary |
|---|---|
| F-0081 · REQ-0047-W1-REGRESSION · DES-0047-W1-DATED-LAYOUT-COMPARE | FX-0047-W1-REGRESSION, IT-0047-W1-REGRESSION. Compare eligible before-and-after observations of titles, canonical, noindex, robots and structured data on a fixed own-site sample after an admitted theme, plugin or platform event. Reuse 0044 facts and the F-1316/F-1327 owners. Show the dated observation and coverage, hold unread pages, and reconcile a replay of the same event and regression as one local notice. The trigger, approved sample and hours-scale notice bound need exact admission. No automatic remediation. |
| F-0083 · REQ-0047-W1-MIXED-CONTENT · DES-0047-W1-RESOURCE-OBSERVATION | FX-0047-W1-MIXED-CONTENT, IT-0047-W1-MIXED-CONTENT. Compare insecure and secure resource evidence on an HTTPS page, a blocked redirect and partial capture under the existing 0115 broker profile. Keep the observed resource reference, refused attempt and actual delivery distinct. Do not follow a forbidden route or treat a blocked read as known delivery. No transport change. DR-109's separate supplied HTTPS evidence rule for F-0128 remains intact. |
| F-0084 · REQ-0047-W1-OBSTRUCTION · DES-0047-W1-RENDER-OBSERVATION | FX-0047-W1-OBSTRUCTION, BDD-0047-W1-OBSTRUCTION. Compare a measured blocking overlay, a dismissible consent panel and missing geometry at the exact admitted viewport and state. Reuse 0115 visibility and paint evidence under RN-09. Apply only the admitted obstruction rule and keep intent unknown. No dialog, consent, form or other interaction follows from this diagnosis. |
| F-0085 · REQ-0047-W1-IMAGE-WEIGHT · DES-0047-W1-RESOURCE-OBSERVATION | FX-0047-W1-IMAGE-WEIGHT, BDD-0047-W1-IMAGE-WEIGHT. Compare confirmed image bytes and format, an eligible alternative format and unknown size. Apply only the independently admitted weight and format profile. Reuse eligible 0115 resource and decoder evidence, including RN-21 and QR-047 where applicable. Show evidence and safe advice; no claim from a failed read, conversion, fetch or replacement. |
| F-0086 · REQ-0047-W1-EMBED-REVIEW · DES-0047-W1-RESOURCE-OBSERVATION | FX-0047-W1-EMBED-REVIEW, BDD-0047-W1-EMBED-REVIEW. Compare a verified broken embed in an admitted source, refusal and one eligible rights-checked replacement. Report a refused or partial lookup as unknown. Link a specific proposal to the exact current native revision and source evidence; withdrawal or edit invalidates it. No extra provider request or automatic repair. |
| F-0087 · REQ-0047-W1-DATE-CONSISTENCY · DES-0047-W1-DATED-LAYOUT-COMPARE | FX-0047-W1-DATE-CONSISTENCY, IT-0047-W1-DATE-CONSISTENCY. Compare matching date types, a same-field contradiction, absent modified time and differently scoped sitemap metadata. Reuse 0044 date and 0115 structured-data evidence under the admitted date basis. Explain compatible observed differences, retain missing facts as unknown and never treat a lastmod field as proof of original publication. No date rewrite. |
| F-0088 · REQ-0047-W1-SITE-IDENTITY-AUDIT · DES-0047-W1-RENDER-OBSERVATION | FX-0047-W1-SITE-IDENTITY, BDD-0047-W1-SITE-IDENTITY. Compare dated exact own-site name sources, a conflicting declaration and an unread favicon against a supported matching sample. Show actual conflicts and eligible native guidance, preserving unread asset coverage. Reuse 0044 structured-data boundaries and 0115 evidence. Any markup requires its exact separate exception, native approval and footprint. |

QR-108 retains the source/renderer and native qualification for
these cases. The existing QR-103 and QR-104 authority boundaries
stay in force. These fixture links describe planned evidence only.

### Settings

| Setting | Default | Allowed range |
| --- | --- | --- |
| `seo.structure.enabled` | on | on or off |
| `seo.structure.history_crawls` | blocked: no admitted value | 2 to a safe top |
| `seo.structure.memory_share` | blocked: no admitted value | fixed share of the service slice |
| `seo.structure.finding_cap` | blocked: no admitted value | above zero, safe top |
| `seo.layout.min_pages` | blocked: no admitted value | 2 to a safe top |
| `seo.layout.min_read_share` | blocked: no admitted value | above 0 to 1 |
| `seo.layout.coverage_floor` | blocked: no admitted value | above 0 to 1 |
| `seo.layout.noise_tolerance` | blocked: no admitted value | above 0, safe top |
| `seo.layout.relevant_marks` | blocked: no admitted value | fixed list, none removed |
| `seo.layout.blessed_classes` | empty | per site, safe top |
| `seo.layout.variant_marker` | empty (no variants) | one marker per site |
| `seo.layout.history_days` | blocked: no admitted value | 1 to a safe top |
| `seo.locale.markets` (after version 1) | none declared | per site, safe top |
| `seo.locale.read_share_floor` (after version 1) | blocked: no admitted value | above 0 to 1 |
| `seo.locale.skew_hours` (after version 1) | blocked: no admitted value | safe bottom and top |
| `seo.locale.confirm_days` (after version 1) | blocked: no admitted value | 1 to a safe top |
| `seo.locale.key_pages` (after version 1) | blocked: no admitted value | per market, safe top |
| `seo.locale.detail_pages` (after version 1) | none | per market, safe top |
| `seo.locale.competing_similarity` (after version 1) | blocked: no admitted value | above 0 to 1 |
| `seo.shop.price_age_hours` | blocked: no admitted value | safe bottom and top |
| `seo.shop.stock_age_hours` | blocked: no admitted value | safe bottom and top |
| `seo.shop.look_gap_hours` | blocked: no admitted value | safe bottom and top |
| `seo.shop.record_rate` | blocked: no admitted value | above zero, safe top |
| `seo.shop.off_peak_window` | blocked: no admitted value | admin hours, safe length |
| `seo.shop.feed_address` | empty | one allowed-host address |
| `seo.shop.run_product_cap` | blocked: no admitted value | above zero, safe top |
| `seo.shop.history_days` | blocked: no admitted value | 1 to a safe top |
| `seo.weak.few_links_in` | blocked: no admitted value | 1 to a safe top |
| `seo.weak.content_links_out` | blocked: no admitted value | 1 to a safe top |
| `seo.weak.zero_filter` | on (search-data runs) | on or off |
| `seo.fall.metric` | impressions | impressions, clicks, visits |
| `seo.fall.min_volume` | blocked: no admitted value | above zero, safe top |
| `seo.fall.min_history_weeks` | blocked: no admitted value | safe bottom and top |
| `seo.fall.persist_windows` | blocked: no admitted value | 2 to a safe top |
| `seo.fall.site_wide_min_pages` | blocked: no admitted value | above 1, safe top |
| `seo.fall.fit` | blocked: no admitted value | admitted method list only |
| `seo.mode.default` | structure only | structure only, with search data |

Every value marked "blocked" is a readiness blocker owned by the relevant
Ganit Vikas research record below. The record must supply its value, safe
bounds, fixtures, evidence and approval before the dependent feature can build
or run. No implicit default, unbounded parser or fallback is permitted.

### Build-blocking research (Ganit Vikas)

1. Layouts: normal build-to-build noise in XenForo and WordPress themes;
   the least group size and read share; which marks matter to search; the
   coverage floor; whether a learned similarity step beats the plain rule
   on made-up sites with known layouts; memory fit in the budgets.
2. Market research after version 1: key page choice; the similarity level
   for competing local pages; safe address and hours matching across scripts
   and formats; the reuse of the language detector; skew and confirm ages;
   the least pages read before a market is judged.
3. Shops: freshness and gap per fact on real shop rhythms; reading a
   visible price across formats; whether WooCommerce exposes variant
   stock and price safely and cheaply; the usual-rhythm test; feed
   formats. Nothing ships until a rule passes on made-up worlds.
4. Weak and falling pages: a trend fit that ignores one spike, on made-up
   pages with known falls and gaps; the volume, history and persistence
   floors; whether ganit's per-page changepoint job does as well as a
   fit; "few links in" per site size; whether any order beats plain sorts.

### Not part of it

Sharing structure between installs or with other products; proof tokens
for outsiders; release-safety verdicts, simulation and causal claims;
money values, order values and conversion guesses; local rank tracking,
map-pack and review scraping; any second crawler; market scores and
weights; translated-but-not-localised detection; per-market shop
availability; category gaps; review summaries; multiplied priority
formulas; site-total or other-page baselines; one-click apply; learned
recrawl timing; a separate learning pipeline (rejections go through
feedback learning, spec 0053).

## Limits

- All report jobs run within the crawler's budgets of spec 0044 (96 MiB
  for crawl-time marks, 64 MiB for report building) and, once admitted,
  `seo.structure.memory_share` of the service slice. While that resource
  bound remains blocked, report jobs fail closed. They stop first under
  any pressure.
- `seo.structure.finding_cap` bounds findings per run.
- The shop reader runs off-peak, at `seo.shop.record_rate`, with
  `seo.shop.run_product_cap` per run, slowing at 10% and pausing at 25%
  above the shop's normal response time.
- No network connection is opened by any report.

## Footprint budget

The checks add nothing to public pages. The shop record reader adds
read-only admin-side requests, off-peak, under the slowing rule; its
per-request time, query and byte budgets are readiness blockers owned by the
shop-reader footprint record. The reader cannot build or run until measured
bounds and fixtures are admitted.

## How it is tested

Rust tests on made-up sites with known answers, plus Angular screen tests.

Shared:

- Below its floor a report shows "not enough evidence" and the missing
  item, with no number, styled as neither success nor failure.
- No report opens a network connection; only the crawler does.
- No screen offers an apply, insert or title-change control.
- A bare percentage fails a screen test; every count names its
  population.
- One site's findings never appear for another site or client.
- Sources and kinds are never added; no combined score exists.
- The jobs stop first under pressure and stay inside their memory share.
- A last-seen value shows as stale and dated, never blank or zero.

Layout groups:

- The same page and rules give the same signature; changing text, images
  or counts of repeated blocks never changes the family.
- A hashed class change alone changes nothing; a blessed class does.
- A declared kind cannot override two clear structures; with no declared
  kind a group still forms.
- A test marker makes variants, not families; none set, none exist.
- The five counts stay apart; unread pages never show as faulty; "fixed"
  waits for the re-read.
- Below the group or coverage floor: "not enough evidence", no claim.
- A rule change marks history "older rules" and joins no line.
- A screen test lists the banned word "caused".
- A similarity candidate cannot create or change an identity.
- No page text or markup is stored, only hashes and mark kinds.

Version 1 language-link audit (IT-STRUCT-LANG):

- The structural audit works with an admitted own-site sample.
  A market record, deferred key or restored market settings start
  no report or query. With an alternate on another registered site,
  an out-of-scope redirect or expired facts, report "cannot confirm".
  No recompute adds a fetch. Local listings stay in spec 0095.
- A names B without B naming A gives one reciprocity finding naming both.
- A version on a host not allowed shows "cannot confirm", never "broken",
  and nothing is fetched from it.
- Report bad region codes and canonical or path contradictions
  only with the required observations. Check no declarations, an
  unread page and a confirmed fault as separate states.
- Only changed clusters are rechecked in the daily pass.
- A missing sample gives "cannot confirm". No finding proves that a
  page was translated or suitable for a market.

Market match after version 1:

- One language or one market shows "not applicable", no perfect score.
- No markets or no details gives "not enough evidence" and the missing
  item; link-graph findings still show.
- The two screens use different types; one cannot fill or be shown as the
  other.
- A difference shows both values and sources; nothing is written back.
- "Not found" and "could not read" never show as differences.
- Pages read too far apart give "recheck due".
- Totals count unevidenced markets; no bare percentage appears.
- Local pack, map and review data show as not observable, with no fetch
  and no connect button.
- One site's details never appear for another.

Shop details:

- A product missing a material fact gets no verdict and no score, and the
  fact is named.
- Looks beyond the gap give "recheck due", never "differs".
- A stale fact shows its last value, dated, never blank or zero.
- A quiet record gives one stale-source finding with a product count.
- A difference shows every value and source; none is picked as right.
- No revenue, price advice, order or customer field is stored or shown
  (test of stored fields and screen words).
- Rival, marketplace and shopping-engine addresses are never fetched.
- The reader slows at 10% and pauses at 25% on a test shop.
- A site with no shop shows "no shop found".
- An admin cannot set a rule looser than the safe bottom.
- One site's shop data never appears for another.
- A PHP and SQL test proves the record reader writes nothing (read-only
  role).

Weak and falling pages:

- A page with both problems is on both lists as two findings.
- Each row names one applicable fact; none is called unreachable.
- No combined score exists; the order is the fact picked.
- Missing search numbers refuse a with-search-data run; none falls back
  by itself; structure-only claims nothing about visits.
- Only both-verified-zero pages hide; unknown never hides; show-all
  restores; nothing is deleted.
- A link approved from a row is re-read; a higher count alone is not
  "solved"; missing demand gives "inconclusive".
- Link suggestions come only from the existing screen and gate.
- One spike among steady history raises no finding.
- Low volume, short history or a gap gives "not enough evidence".
- The weekly fixture gives -5, -0.1 and -0.3 as separate units.
- No site-average or other-page baseline is used.
- A merge or redirect restarts history with a note.
- Copy never says a link or change "caused" a fall.
- A many-pages group keeps every row; overlaps use unique totals.
- One site's findings never appear for another.

## Changes from the source

- Signatures, identities, verdicts, floors, modes and states are decided
  in Rust, with Kani proofs where they fit.
- Similarity, trend fits, rhythm and changepoint maths run in ganit, the
  Rust numerical engine.
- All screens and forms are a desktop app built from SDS components.

Source: `docs/specifications/signals/site-structure-checks.md`,
`site-structure-checks-2.md`, `site-structure-checks-3.md`,
`site-structure-checks-4.md` and `site-structure-checks-5.md` (all in
`docs/specifications/signals/`) in the archive
`archive-2026-11-08-old-repo`.

## Performance Lab

[Spec 0142](0142-local-performance-lab.md) presents compatible
measurements under the existing performance budget and
comparison rules. An admitted lab observation must identify its
capture method, profile, clock and scope before it can support a
regression. Keep supplied, local and field observations distinct.
DR-135's separate scripted duration is never field INP.
