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
"Language links" with "Market match", "Shop details", and "Weak pages"
with "Falling pages".

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
4. Starts on with researched values. Every setting has a safe bottom and
   top.
5. Language versions on another registered site of the same owner are
   read under the per-pair read-and-report rule, not by a new route.
6. Each finding shows what, where, how many pages, evidence kind, crawl
   date and a written proposal for a person. Findings group into one
   alert per cause.

### The evidence floor (everywhere)

Every report has a floor: the least evidence it needs before it may
speak. The floor is a researched setting. Below it the report says "not
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
   A score appears only if GanitEvolve research proves it on made-up
   worlds with known answers.
4. A report that cannot run says so; it is never shown as passed.
5. No fallback to a weaker score when evidence is missing.

### Shared plumbing

1. The crawl pass writes a small `seo_structure` record per page and
   per-kind detail tables. No page text is kept beyond the page copies
   of spec 0044.
2. A report job runs after each full crawl and each changed-page pass,
   within the report memory budget (raised only by a researched setting).

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
   headings. What "materially" means is set by research.
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

### Language links and market match

Two screens, never merged, with different types: "Language links"
(counted from the crawl) and "Market match" (which may abstain).

Example: the German thread page names the English one as its alternate,
but the English page does not name it back. Report: "Link broken one
way: de names en, en does not name de. 14 language pairs." It says
nothing about whether the street address on those pages is right.

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

Language links (always computable): pages are nodes and "names as
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

Only clusters that changed between crawls are rechecked daily; the weekly
pass checks all. A proposal writes out the correct full set of
alternates as text for a person to apply. There is no export file for a
content system.

Market pages: for each market, the key pages that exist in other markets
but not this one ("Router guide has en and de versions; no fr version").
How key pages are chosen (an admin list, or most visited by darshak
totals) is set by research. Also near-identical local pages competing for
the same queries (ganit jobs 265 and 286, with layout groups), and
structured data that contradicts the market (a currency or language tag
that does not match). Report only; copy quality is never graded.

Business details:

1. The admin types, per site and location: business name, address,
   phone, and opening hours with the location's time zone. One record per
   site, never shared, and never treated as the truth. The report
   compares it with what pages in that market state (structured data,
   address and phone elements, on pages the admin lists).
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
   this market" with a button to enter them.
7. Matching folds accents, case, number formats and address
   abbreviations. No outside map or address service is called.
8. Local pack position, map ranking and review pages are shown as not
   observable, with no fetch and no connect button.

Totals: a market with too little evidence is counted as such: "2 markets
match, 1 differs, 3 not enough evidence". It is never dropped or counted
as zero. Link-graph figures name their population ("of 420 language
pages read").

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
   read-only through one read-only shop reader in the WordPress plugin,
   which needs its own approval) and an optional product feed at one
   admin-named address on an allowed host (formats set by research).
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
7. Two looks compare only if read within the allowed gap; otherwise
   "recheck due" with an offer to re-read those pages.
8. A stale fact keeps its last value, dated: "current price: not enough
   evidence; last seen 21.00, 17 hours ago".
9. If a record or feed has not changed for far longer than its usual
   rhythm (the median gap between its changes), that is one "shop record
   stale" finding with a product count.
10. Checks: price differs across views; unavailable or removed products
    still in the sitemap or marked in stock; a variant page whose
    canonical points at its parent while it carries its own price;
    currency differing between views or from the market's currency;
    missing shipping or returns facts; stale availability. Findings group
    by product layout or by the wrong record field.
11. It never calculates revenue, order value, conversion, ROI or expected
    value, suggests a price, reads orders or customers, reads rival
    prices, marketplaces or shopping engines, or grades reviews. A
    person-set sales label belongs to link ranking (spec 0029). Visits and
    impressions to affected pages may show as plain counts.
12. Live-shop safety: the record reader runs off-peak, in small pages, at
    a rate setting, under the slowing rule (slows at 10% above normal,
    pauses at 25%), with a product cap per run. It reads the owner's own
    shop only and writes nothing.

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
4. A robust trend fit chosen by research, so one spike cannot raise a
   finding. Odd points stay visible with reasons.
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

### Settings

| Setting | Default | Allowed range |
| --- | --- | --- |
| `seo.structure.enabled` | on | on or off |
| `seo.structure.history_crawls` | researched | 2 to a safe top |
| `seo.structure.memory_share` | researched | fixed share of the service slice |
| `seo.structure.finding_cap` | researched | above zero, safe top |
| `seo.layout.min_pages` | researched | 2 to a safe top |
| `seo.layout.min_read_share` | researched | above 0 to 1 |
| `seo.layout.coverage_floor` | researched | above 0 to 1 |
| `seo.layout.noise_tolerance` | researched | above 0, safe top |
| `seo.layout.relevant_marks` | researched | fixed list, none removed |
| `seo.layout.blessed_classes` | empty | per site, safe top |
| `seo.layout.variant_marker` | empty (no variants) | one marker per site |
| `seo.layout.history_days` | researched | 1 to a safe top |
| `seo.locale.markets` | none declared | per site, safe top |
| `seo.locale.read_share_floor` | researched | above 0 to 1 |
| `seo.locale.skew_hours` | researched | safe bottom and top |
| `seo.locale.confirm_days` | researched | 1 to a safe top |
| `seo.locale.key_pages` | researched | per market, safe top |
| `seo.locale.detail_pages` | none | per market, safe top |
| `seo.locale.competing_similarity` | researched | above 0 to 1 |
| `seo.shop.price_age_hours` | researched | safe bottom and top |
| `seo.shop.stock_age_hours` | researched | safe bottom and top |
| `seo.shop.look_gap_hours` | researched | safe bottom and top |
| `seo.shop.record_rate` | researched | above zero, safe top |
| `seo.shop.off_peak_window` | researched | admin hours, safe length |
| `seo.shop.feed_address` | empty | one allowed-host address |
| `seo.shop.run_product_cap` | researched | above zero, safe top |
| `seo.shop.history_days` | researched | 1 to a safe top |
| `seo.weak.few_links_in` | researched | 1 to a safe top |
| `seo.weak.content_links_out` | researched | 1 to a safe top |
| `seo.weak.zero_filter` | on (search-data runs) | on or off |
| `seo.fall.metric` | impressions | impressions, clicks, visits |
| `seo.fall.min_volume` | researched | above zero, safe top |
| `seo.fall.min_history_weeks` | researched | safe bottom and top |
| `seo.fall.persist_windows` | researched | 2 to a safe top |
| `seo.fall.site_wide_min_pages` | researched | above 1, safe top |
| `seo.fall.fit` | researched | a researched list |
| `seo.mode.default` | structure only | structure only, with search data |

"Researched" values are fixed by GanitEvolve research before the build.

### Research first (GanitEvolve)

1. Layouts: normal build-to-build noise in XenForo and WordPress themes;
   the least group size and read share; which marks matter to search; the
   coverage floor; whether a learned similarity step beats the plain rule
   on made-up sites with known layouts; memory fit in the budgets.
2. Languages: key page choice; the similarity level for competing local
   pages; safe address and hours matching across scripts and formats; the
   reuse of the language detector; skew and confirm ages; the least pages
   read before a market is judged.
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
  for crawl-time marks, 64 MiB for report building) and the fixed
  `seo.structure.memory_share` of the service slice. They stop
  first under any pressure.
- `seo.structure.finding_cap` bounds findings per run.
- The shop reader runs off-peak, at `seo.shop.record_rate`, with
  `seo.shop.run_product_cap` per run, slowing at 10% and pausing at 25%
  above the shop's normal response time.
- No network connection is opened by any report.

## Footprint budget

The checks add nothing to public pages. The shop record reader adds
read-only admin-side requests, off-peak, under the slowing rule; its
per-request budget is set when the reader is built.

## How it is tested

Rust tests on made-up sites with known answers, plus Dioxus screen tests.

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
- The four counts stay apart; unread pages never show as faulty; "fixed"
  waits for the re-read.
- Below the group or coverage floor: "not enough evidence", no claim.
- A rule change marks history "older rules" and joins no line.
- A screen test lists the banned word "caused".
- A similarity candidate cannot create or change an identity.
- No page text or markup is stored, only hashes and mark kinds.

Language links and market match:

- A names B without B naming A gives one reciprocity finding naming both.
- A version on a host not allowed shows "cannot confirm", never "broken",
  and nothing is fetched from it.
- A canonical crossing a language line and a bad code are reported.
- Only changed clusters are rechecked in the daily pass.
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
- All screens and forms are a Dioxus app built from SDS components.

Source: `docs/specifications/signals/site-structure-checks.md`,
`site-structure-checks-2.md`, `site-structure-checks-3.md`,
`site-structure-checks-4.md` and `site-structure-checks-5.md` (all in
`docs/specifications/signals/`) in the archive
`archive-2026-11-08-old-repo`.
