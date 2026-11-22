# 0103: Shop SEO for WooCommerce

This specification owns fourteen shop features for WordPress sites that run
WooCommerce, including the product page audit, out-of-stock handling and
faceted-navigation crawl traps, category content suggestions,
copied manufacturer descriptions, product structured data, review snippet
eligibility, free merchant listing readiness, the product image audit,
guide-to-product internal links, product questions and answers from forum
discussions, seasonal demand, sales attribution and sales event landing pages.

Spec 0047 owns the "Shop details" consistency report (585). The audit
and out-of-stock handling (579 and 580) use the checks documented in
that spec and the same read-only shop reader. Spec 0103 owns the
acceptance of those two features in the feature register.

The owner decided on 20 November 2026 that version 1 builds the WooCommerce
checks and proposals for WordPress sites and nothing for other shop platforms
(DR-083), and that sales attribution may use order counts and revenue per
landing page and origin class, with no customer data (DR-084). XenForo has no
shop in version 1, and Shopify is out of scope (ADR-0021). A WordPress site
without WooCommerce shows "no shop found", never an error or a zero.

The shop checks, drafts and proposals have no shop write action. Link
suggestions use the existing moderator review and undo in specs 0028
and 0029. The dossier must confirm the search engine rules against
dated official sources under QR-058 before an eligibility verdict uses
them. A verdict cannot promise an appearance in search.

Fictional example: Lena's shop has 3,200 products. The audit says "A filter on
the Shoes category makes about 41,000 address combinations from 6 filters;
38,400 are already set to 'noindex', 2,600 are not. They use 22 percent of the
crawl." It proposes a rule and shows its effect. Lena reviews the advice and
sets the rule in her site's own tools. The structured data check says "On 112
products the page and shop record agree on a price of £18.00. The markup says
£21.00." Lena reviews the proposed correction. The sales view shows, for the
shoe category's landing page last month, 38 orders and 2,140.00 pounds that
the shop recorded as arriving from search; no customer is named, and the view
groups small cells as "Other" when it can show them safely.

Values marked "proposed" are design choices to be checked by measurement.

## What it does

### Words used here

- **Shop reader**: the one read-only reader in the WordPress plugin that gives
  Sutradhar shop facts (spec 0047); it needs its own approval.
- **Faceted navigation**: filter and sort links that make many addresses.
- **Crawl trap**: a set of addresses with no natural end.
- **Product page, category page**: pages of the shop.
- **Landing page**: the page a visit starts on.
- **Origin class**: how the shop recorded that an order arrived (search,
  direct, social, other: proposed classes; the shop's own origin fields and
  their mapping are open under QR-058).

### What this spec designs

| Feature | What the user sees |
|---|---|
| 579 Product page audit | The shared product checks from spec 0047 |
| 580 Out-of-stock product handling | Stock findings from spec 0047 and unavailable products excluded from link suggestions |
| 581 Faceted navigation crawl traps | Filter combinations that waste crawling, with a proposed rule |
| 582 Category page content suggestions | Draft ideas for thin category pages |
| 583 Duplicate manufacturer descriptions | Product text that is the same as other text |
| 584 Product structured data | Proposed or corrected product markup |
| 586 Review snippet eligibility | A checklist for review markup |
| 587 Free merchant listings readiness | A checklist for free listings |
| 588 Product image audit | Image problems per product |
| 589 Guide-to-product internal links | Proposed links from guides to products |
| 590 Product Q&A drafts | Draft questions and answers from forum discussions |
| 591 Seasonal product demand | How the shop's own demand moves through the year |
| 592 Search traffic sales attribution | Orders and revenue per landing page, aggregated |
| 593 Sales event landing pages | Draft landing pages for sales |

### Shared rules

1. Scope. Only WordPress sites with WooCommerce active (spec 0047). Facts come
   from the crawl (spec 0044), the page engine (spec 0115), the site's own
   search and visit data (specs 0040 and 0034) and the shop reader. No shop
   platform other than WooCommerce, and no merchant account of any kind, is
   read.
2. Use the active site's guest-visible products, categories, guides,
   relevant discussions and public shop policy pages. The comparison
   in feature 583 also accepts the authorised inputs named there.
   Cart, checkout and account pages and customer and order identities
   are excluded. Feature 592 accepts only its approved aggregates.
3. Every finding names its source, observation time and page version
   where it refers to a page. Missing facts show "not enough evidence".
   Show a zero only when the source establishes a zero.
4. A shop proposal shows the evidence and a preview for review. Staff
   make shop changes in the site's own tools. Insertable links follow
   the permission, freshness, preview and undo rules in specs 0028
   and 0029.
5. The Editorial Coach (specs 0041 and 0042) uses the local route for
   these drafts. Ground each claim in a named source. Take prices and
   stock from the current shop record. With no admitted local route,
   show the sourced outline where available or say drafting is
   unavailable. Drafts and their evidence cannot use outbound fallback.
6. Shop alerts use the count-only content of spec 0100's Headline level.
   Keep sales counts, revenue, product facts and draft evidence local.
7. Read findings and make proposals through site-scoped actions under
   spec 0020. Enforce the active site and the actor's permission on
   every action. The owner enables sales and sees its protected figures.

### Product page audit and out-of-stock handling (579 and 580)

1. Reuse the product, variant and stock checks and freshness rules in
   spec 0047's "Shop details" report. The audit shows those findings on
   the WooCommerce products, with their evidence and missing views.
2. Show when unavailable or removed products remain in a sitemap or
   appear in stock. Exclude products without confirmed current stock
   from guide-to-product suggestions. Use the existing shop details
   verdict and link review routes.

### Faceted-navigation crawl traps (581)

1. Group addresses observed in the bounded crawl by site, path and
   parameter names. Estimate combinations as the product of distinct
   observed value counts per name. Use the crawl bounds of specs 0044
   and 0082 and show an open-ended estimate if it exceeds the proposed
   upper setting bound. Flag a group at
   `shop.trap_min_urls` (proposed 500), or when the crawl records an
   expanding pattern under specs 0044 and 0082. This check uses the
   finished crawl evidence and opens no extra crawl.
2. Show combinations as an estimate, observed addresses as a count,
   and crawl share as the group's distinct observed addresses divided
   by distinct addresses visited in the same crawl. With no valid
   denominator, show "not enough evidence". Keep indexability,
   "noindex", robots rules and canonical status as separate facts.
3. The proposal is a change for the review page: a robots rule, a "noindex"
   setting, a canonical setting or a change to a filter link, whichever the
   site's plugins can do. Each proposal shows which pages the change would
   affect and which receive search traffic (spec 0040). Warn when a
   blocking rule covers a page with clicks or traffic evidence is missing.
4. Sutradhar never edits the robots file or a setting itself.

### Category content suggestions (582)

1. Report a short or missing introduction using the page engine's main
   content and `shop.thin_category_words` (proposed 150). Use a short
   product list as evidence only if a comparable site baseline has been
   admitted. Show missing extraction or baseline evidence separately.
2. Propose a sourced outline for the introduction and questions. Use
   the site's own search queries (spec 0040) and eligible discussions as
   described for product Q&A. Include an existing guide as an example
   only when it belongs to the active site.
3. The draft cites its sources, avoids claims about performance, health or
   safety that it cannot source, and is a proposal only.

### Copied manufacturer descriptions (583)

1. Compare shop descriptions using spec 0116's fingerprints. Across
   sites, reuse only owner-approved group comparison under spec 0102
   and use its fingerprints and sketches. For manufacturer evidence,
   use a user-supplied file under spec 0100's import limits or one read
   of an owner-named address under spec 0082. Keep that evidence local
   to the approved comparison and out of drafts and outbound routes.
2. A result shows the products, the similarity figure and the sample size and
   says "very similar to", never that text was copied or by whom.
3. The proposal is a short outline for a rewrite with the facts to keep.
   Nothing is rewritten for the user.

### Product structured data (584) and review snippets (586)

1. Use spec 0115's structured data verifier and the product comparison
   in spec 0047. Match the same product and variant in each view. Check
   the name, price, currency, availability, brand, identifiers, images
   and review figures against the available evidence.
2. Use spec 0047's freshness and missing-fact verdicts. Show "recheck
   due" or "not enough evidence" and withhold corrective markup when
   facts are missing, too old or disagree on the proposed value.
3. Build any proposed markup from confirmed page and shop facts. Only
   propose a count or rating supported by real, visible reviews for
   that product. Flag a rating found in markup alone.
   Escaping rule, inheriting spec 0095's typed-markup rule: every shop
   name, description, brand and identifier placed in JSON-LD is data.
   Encode it as JSON string content, escape quotes, backslashes and
   control characters, and escape `<`, `>`, `&` and U+2028/U+2029 as
   `\uXXXX` so text such as `</script>` or an HTML comment opener
   cannot close the script element. Build the object from typed
   fields, never by joining strings, and show the exact escaped
   result in the preview. A value that cannot be escaped safely is
   refused with "not enough evidence".
4. Show verifiable facts while QR-058 remains open. Show "cannot tell"
   for an eligibility verdict without admitted rules. When the dated
   rules are admitted, show "meets", "does not meet" or "cannot tell"
   with the reason and the version of the rules used.

### Free merchant listing readiness (587)

1. Show observed product and policy facts, such as price, currency,
   availability, image, identifiers, link, contact, shipping and returns.
   Check the product and site policy pages allowed by the shared rules.
2. Show "cannot tell" for listing readiness until QR-058 admits a dated
   rule set. Count products as ready only where the admitted rules and
   all required facts support that verdict.
3. The checklist uses public site pages. It has no merchant account
   access, account writes or feed upload (DR-076).

### Product image audit (588)

1. Using the image readers of spec 0115, each product's images are checked for
   dimensions (`shop.image_min_px`, proposed 800 on the long side), file size
   (`shop.image_max_kib`, proposed 300 for a normal image), format, broken
   files, missing description text, the same image used on several products,
   and a large first image that is loaded lazily (which slows the main
   picture).
2. Findings group by product and cause, with a count. Proposals (compress,
   resize, add a description) are for the review page.

### Guide-to-product links (589)

1. The link engine (specs 0023, 0027 and 0029) may take a product page as the
   destination of a link from a guide, a blog post or a forum article, using its
   usual relevance, anchor and link-density rules. The source and
   destination belong to the same active site.
2. Product links follow the owner's marks: a link that is paid or affiliate
   carries the right relation marks (spec 0101's checker), and a product that
   lacks confirmed current stock is excluded (feature 580).
3. Link proposals use the existing review, insertion and undo contract
   in specs 0028 and 0029.

### Product questions and answers (590)

1. Use approved guest-visible discussions on the active site and name
   the source thread for each answer. Without a qualifying discussion,
   show "no sourced answer". Cross-site sources are open under QR-060.
2. A source answer must have an accepted reply or a staff reply. Strip
   names and other member identifiers before drafting. Source links
   and the draft must carry no identifying fields or quoted identities.
3. Staff review the draft locally. The site publishes by its own means.
   Propose at most `shop.qa_max` pairs for one product (proposed 10).

### Seasonal demand (591)

1. Use the active site's search and visit series (specs 0040 and 0034)
   by category and calendar week. Keep sources separate and show their
   coverage and sample sizes.
2. Below `shop.demand_min_years` years (proposed 2), show "not enough
   history". Mark missing weeks as gaps. Show the observed series for
   planning. A seasonal verdict needs a defined comparison of the same
   weeks across years and fixtures for partial history, the year
   boundary and recurring versus one-off changes (QR-061). Until then,
   show "seasonality not established".
3. The chart describes this shop. It does not change the site or supply
   a popularity signal to link ranking.

### Search traffic sales attribution (592)

1. Sales starts off. The owner sees and confirms the site, period,
   purpose, returned fields, coverage, group floor and expiry.
   Bind this approval to the exact sales contract and paired add-on.
   Product approval under spec 0047 never gives sales approval.
   The add-on and service both verify the grant before work,
   publication, receipt and display. A changed scope needs fresh
   approval. A failed verification leaves the operation unavailable.
   Use the signed grant, revocation and lease protocol of spec 0021.
2. `ReadShopSalesDay` of spec 0021 serves one bounded day release.
   Build it on the site from WooCommerce's native order API.
   The reader examines payment, state, totals, refunds, currency
   and admitted grouping facts only. It sends and keeps no order
   records or identities. Direct order-table queries and customer
   fields are outside the reader's contract. [WooCommerce order API][shop-api]
3. Assign each qualifying order once to the shop calendar day of
   its recorded payment, under the zone recorded in the approval.
   Use successive local midnights as the inclusive start and
   exclusive end, even on days of other lengths. An unpaid, draft,
   failed or cancelled order does not qualify. A paid refunded
   order still counts as one. Value is the final paid total after
   discounts, including tax and shipping, less recorded refunds
   against that order at the release cutoff. A fully refunded order
   contributes zero. Reject contradictory records and unsupported
   currencies. Never sum unlike currencies or convert rates.
4. Only QR-058-admitted native grouping facts may provide an origin
   or public landing group. Place each order in exactly one cell
   per day and currency. Use "unknown" when the native evidence is
   missing or the landing page is not admitted for this site. Never
   join orders to visits or cookies. Do not enable tracking in the
   shop. No raw origin string or landing URL crosses the route.
5. Before the reply, remove all cells below `shop.sales_min_orders` from the
   named partition. Merge their disjoint orders into one "Other" row for that
   day and currency only if the merged count meets the same floor. Give that
   row reserved group and origin codes, with no member list, breakdown or
   count of source cells. Otherwise omit the whole remainder. Publish no
   independent total or overlapping cuts. Sales cells stay per site and never
   join group or portfolio views (spec 0102). Show disclosed rows alongside
   visits and search clicks, with the cutoff and "recorded by your shop". The
   report describes recorded attribution.
   Dominance rule: a cell whose revenue is mostly one order (proposed:
   over 80% from one order) is merged into "Other" or hidden, even above
   the floor. Campaign tags come from the shop's own links and can be
   spoofed, so show them as "tag as recorded", never as proof.
6. Publish the whole day atomically across all currencies. Each
   repeat returns the same release under the current signed envelope.
   Seal empty releases too. Use the same complete coverage state for
   a day with zero orders and one with every cell suppressed. Use an
   admitted consistent native snapshot or a verified source fence.
   If it cannot prove a complete input at the cutoff, publish nothing.
   Retry a failed scan only before any release was published and
   within the work bounds. Reuse no partial scan data.
7. A native correction or erasure withdraws the affected day and
   rollups. Pass affected intervals only, without an order or person
   id. Use an admitted native change source with no public-page
   hook. If the interval cannot be proved, withdraw all potentially
   affected releases. Persist the withdrawal marker first. Check
   the current signed withdrawal generation before the service
   accepts or displays sales results, even from cache. Withhold
   them if verification fails. A generation change fences local
   in-flight work. Keep value-free interval and generation fences
   while a reread or restore could reopen a cohort. Seal the exact
   time interval across grouping, floor, currency, zone, pairing and
   grant changes. Do not release any overlapping interval again.
   Apply the lifecycle rules of specs 0030 and 0032.
8. Keep disclosed daily cells up to `shop.sales_keep_days`. Seal a
   monthly rollup once from that month's disclosed day releases,
   by approved group, origin and currency, before pruning those
   releases. A later release cannot revise it. Describe the
   rollup as a sum of disclosed days and show missing coverage.
   Keep rollups up to `shop.sales_rollup_keep_days` from month end.
   Withdraw affected rollups together with their source releases.
   Sales values stay in the local owner report, with no SQL
   console, custom metric, export, snapshot or outside alert path.
9. On disable, revoke or site removal, fence service reads, late
   replies, cached results and display immediately. Send the
   signed revoke. The add-on stops on receipt or at its work
   lease expiry, whichever comes first. Verify the grant and
   lease again when the worker commits. Delete sales records
   and derived copies on each side. Retry the revoke and cleanup
   at the next signed contact if the add-on is unreachable.
   Local expiry and erasure still apply. The service permits
   no new use while cleanup is pending. Apply retirement and
   erasure fences before restore.
10. Run the builder only through an admitted off-peak host or
    admin runner. Public requests, page-driven WP-Cron and the
    signed read never start it. QR-091 must admit both storage
    modes, source consistency under concurrent payment, refund
    or erasure, pagination and cache consistency, withdrawal
    and measured limits. A host that cannot provide these
    checks has no sales reader.

[shop-api]:
  https://developer.woocommerce.com/docs/features/orders/wc-get-orders/

### Sales event landing pages (593)

1. For a sales event the owner names (dates, the products or categories
   chosen) the Coach drafts a landing page outline and text: a heading, the
   offer in plain words, links to the chosen products and categories, and the
   dates.
2. Prices and stock come only from the shop record at the time of drafting and
   are marked "to be checked before publishing". The draft never states a
   discount, a "was" price or a deadline that is not in the shop record or
   given by the owner.
3. The draft goes through the advertising and disclosure checks of spec 0101
   before it is shown for review. It is a proposal; the site publishes it by
   its own means.

### Shop acceptance contracts (579–593)

Pin each finding to active WooCommerce site, product or variant,
page and shop revision, evidence projection and observed UTC.
Compare only current native shop facts and comparable guest page
facts under 0047. A missing reader or incomplete scope cannot
produce zero or a stock decision. Use the shared 0047 price and
availability findings for 585. Do not add a second price store
or a provider route. A removed or unconfirmed-stock product
is ineligible for a new guide-to-product suggestion.

A facet report reads one completed permitted crawl generation.
Count actually visited distinct parameterised addresses, grouped
by site, observed path and query keys. Canonical is a separate
observed field. It cannot collapse distinct observed paths.
Report the combination estimate separately. Use only the same
crawl denominator and show cap, blocked and sampled coverage. A missing
denominator gives unknown. A proposed rule
shows the scoped page impact, observed canonical, robots,
indexability and search evidence. Preserve all incomplete cases.
No extra visits, robots-file edit or indexing effect.

A category word count needs proved main-content extraction.
Compare with the exact configured threshold. Unread content is
unknown. Short product-list verdicts stay unavailable until
QR-061 proves comparable complete pages, product count, taxonomy,
sample minimum and freshness. Keep unsupported product facts out
of copy. In 583 use the current 0116 profile and thresholds.
Keep own-site, approved 0102 group and manufacturer inputs
distinct. Manufacturer evidence uses only a licensed local
file or a separately approved one-read named address under
0082. No refetch or broader use. Show source name only within
its grant. Never claim
copying from similarity. A manufacturer comparison grants no
right to quote, rewrite or send source material. Withdrawal
revokes the underlying proposal.

Keep Product snippets, review snippets and free listings separate.
Use dated [Product snippet rules][product-snippets] and the
[ReviewSnippet profile][review-snippets]. Bind exact product,
variant, visible evidence, native currency and 0115 check profile.
Check the selected snippet's required fields. Reviews need
eligible visible first-party product ratings with scale and
count. Markup alone cannot prove them. Free listing readiness
checks local sale page and offer facts under [merchant listing
markup][merchant-listings] and dated [free listing rules][free-list].
Check applicable public price, stock, shipping, returns and
policy evidence. A snippet does not establish a sale offer or
account readiness. No current profile or source gives unknown.
Render only confirmed fields. No Google appearance guarantee.
No app markup write, account access, crawler validator send
or feed upload. Staff use their normal approved site tools.

Image findings use existing 0115 evidence and the named shop
settings. Keep dimensions, decoded status, payload size, visible
description and observation coverage separate. Missing asset
evidence cannot prove a broken file. Propose a local correction
only. The link engine checks current stock, same active site,
relevance, link density, purchasability, exact destination and
relation marks before review. Recheck admitted native stock,
destination and policy at insertion through 0028/0029. Preserve
the existing undo and check the exact source version. Positive signals do
not override stock or relevance. A Q&A draft uses only a current
eligible accepted or staff answer in an authorised same-site
discussion. Pin native thread, revision, rights and guest scope
at save and review. Strip identities, including in source
citations. Distinguish no eligible answer from unread evidence.
A cross-site grant gives no Q&A evidence. Use QR-060.
Forum thread text drafted into a Q&A answer is data, never an
instruction: it is quoted or summarised as source text, never
obeyed, and it never changes the draft's rules, tools or targets.
Cap each source excerpt at 2,000 characters (proposed) and the whole
source set for one draft at 8,000 characters (proposed), cutting on
a character boundary and saying so. Escape the text for the place it
lands: plain text in the review view, and markup-safe text in any
proposed page output.

The QR-061 candidate seasonal profile (every number below is proposed and
fixed only when QR-061 closes) fixes category taxonomy, stock context, source,
metric and zone before analysis. Use the most recent max(2,
`shop.demand_min_years`) (proposed) complete comparable ISO week years.
Preselect the same four-week block and its preceding four weeks in each year,
without overlaps or crossing year boundaries. Use native-zone week anchors and
refuse unmatched week 53. Require full coverage and at least 50 eligible
baseline events (proposed) in each year. A recurring increase needs an exact
ratio of at least 1.20 (proposed) in every compared year. Show covered
declines and one-year spikes as observations only. Incomplete history, changed
taxonomy or unlike stock basis gives unknown. Show source and sample. The rule
needs QR-061 evidence before use. No causal, sales or ranking inference.

The 592 report consumes only the immutable ReadShopSalesDay
release. Keep recorded native attribution separate from visit
or click evidence. Consent, receipt, currency, refund cutoff,
independent withdrawal and approved privacy cells still apply.
Cover both admitted storage modes and prove no worker or query
on an unsupported host. Race payment and refund with sealing
one indivisible all-currency day. Exact replay returns its receipt.
Suppressed and empty days share the same coverage state. No
independent total or overlapping release after zone, group or
grant edits. Recheck current approval at final display, including
after native correction or restore. No console, MCP, export or
alert projection.

A sales-event draft freezes authorised event interval, product
set, sources, prices, stock and 0101 disclosure evidence.
Pin the claimed discount basis, dates, product variant,
currency, stock and price evidence as of approval. Recheck the
exact protected projection before review, copy or local export.
Expired or changed offers, rights and unsupported amounts hold
the claim and require fresh review. No campaign, price, ad,
catalogue or publishing effect.

[product-snippets]: https://developers.google.com/search/docs/appearance/structured-data/product-snippet
[merchant-listings]: https://developers.google.com/search/docs/appearance/structured-data/merchant-listing
[review-snippets]: https://developers.google.com/search/docs/appearance/structured-data/review-snippet
[free-list]: https://support.google.com/merchants/answer/13889434?hl=en

### Data

- Findings, proposals and drafts: the shop tables of the audit (spec 0044)
  with the site column and forced row-level security (spec 0108).
  Use the existing report and job admission limits (specs 0047 and
  0078).
- Both sides keep one immutable day release record with site, approved
  interval and zone, grant and release generations, cutoff, coverage and
  expiry. Store disclosed cells with the six fields of spec 0021's
  `ReadShopSalesDay` row (day, landing group id, origin class, count, minor-
  unit total, currency) and release key. Counts and values are sensitive.
- The service monthly record names the sealed month, group and
  currency scope, source release generations and expiry. A backup
  cannot bypass expiry, withdrawal or approval checks.
- Settings and the approvals of the shop reader: the settings registry (spec
  0014).

### Settings

Numeric defaults and ranges below are proposed.

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `shop.trap_min_urls` | whole number | 500 | 50 to 1,000,000 | next audit |
| `shop.thin_category_words` | whole number | 150 | 20 to 2,000 | next audit |
| `shop.image_min_px` | pixels | 800 | 100 to 8,000 | next audit |
| `shop.image_max_kib` | KiB | 300 | 20 to 10,240 | next audit |
| `shop.qa_max` | whole number | 10 | 1 to 50 | next draft |
| `shop.demand_min_years` | whole number | 2 | 1 to 10 | next view |
| `shop.sales_enabled` | on or off | off | two values | at once |
| `shop.sales_min_orders` | whole number | 5 | 5 to 1,000 (proposed) | next confirmed grant, undisclosed periods only |
| `shop.sales_keep_days` | days | 395 | 30 to 1,095 | next night |
| `shop.sales_rollup_keep_days` | days from month end | 730 | 30 to 1,095 | next night |

There is no setting that lets Sutradhar change a shop page, rule or markup
itself, read customer or order identities, link an order to a person or send
shop money data to an outside service.

### Failure codes

SUT-2000 no shop found; SUT-2001 shop reader not approved; SUT-2002 shop
record not readable (information); SUT-2003 not enough history for seasonal
demand; SUT-2004 sales origin not recorded by the shop; SUT-2005 sales group
too small (information); SUT-2006 draft refused, a price or stock fact would
be invented; SUT-2007 reader returned a field outside the aggregate list or
named the wrong site. Codes SUT-2000 to SUT-2099 are reserved for this spec.

### Security and privacy

- Sales approval, reads and proposals are audited under spec 0032,
  without identities, raw sales cells or revenue in audit entries.
- The shop reports have no new public-page queries, time or bytes.
  The reader in spec 0047 uses off-peak, measured admin-side budgets.
  Admit separate limits for the materialiser and signed read under QR-091.
  The record pins concurrent sales workers, request and job time, lease
  lifetime, withdrawal check age, request and reply bytes, native queries,
  scanned orders and stored cells. A cap or slowdown stop returns a gap,
  with no partial release. Run off-peak under spec 0047's guard. Prove the
  guard on both admitted WooCommerce storage modes and shared hosting.
- The only cross-site comparison is the approved fingerprint route in
  spec 0102 that feature 583 reuses. The action checks the active site
  and applies the permissions and visibility rules of that route.

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| SH-01 | Every feature MUST work only on a WordPress site with WooCommerce active and MUST show "no shop found" otherwise (SUT-2000). | UT-SH-01 |
| SH-02 | The checks MUST use the shared input allow-list. Sales MUST use only approved aggregates under SH-14. | UT-SH-02, PL-SH-01 |
| SH-03 | Every change MUST be a proposal on the review page with a preview. A shop proposal MUST NOT change pages, rules, markup or settings. Shop link insertion and undo MUST use specs 0028 and 0029. | IT-SH-01, AU-SH-01 |
| SH-04 | A fact that cannot be read MUST show "not enough evidence", never zero, and each finding MUST name its source and time and the page version where applicable. | UT-SH-03 |
| SH-05 | A trap candidate MUST be reported with its parameters, combinations, crawl share and state, and warn about blocking pages with known or unknown search traffic. | UT-SH-04 |
| SH-06 | Category content proposals MUST be outlines with sources, and drafts MUST NOT invent a product fact, price, stock, rating or date (SUT-2006). | UT-SH-05, PL-SH-02 |
| SH-07 | Duplicate-description checks MUST use only shop text, the owner's own group, a user-supplied file or a once-read listed address, and MUST say "very similar", never "copied". | UT-SH-06 |
| SH-08 | Product markup MUST be checked against the page and shop record, proposed markup MUST come only from them, and a rating MUST NOT be proposed without real visible reviews. | UT-SH-07, PL-SH-03 |
| SH-09 | The review snippet check MUST give "meets", "does not meet" or "cannot tell" with reasons and MUST show "cannot tell" without admitted rules or sufficient evidence. | UT-SH-08 |
| SH-10 | The merchant listing check MUST withhold a readiness count without admitted rules and MUST NOT access merchant accounts. | UT-SH-09, AU-SH-02 |
| SH-11 | The image audit MUST report the stated problems and group them by product and cause. | UT-SH-10 |
| SH-12 | Product link proposals MUST use the link engine's rules and relation marks and MUST exclude products without confirmed current stock. | UT-SH-11 |
| SH-13 | Product Q&A MUST use guest-visible sources only, name the source thread, show no member name and mark an unsourced question. | UT-SH-12, PL-SH-04 |
| SH-14 | Sales attribution MUST be off by default, require the owner's approval after notice, and MUST stay unavailable pending verified admission under QR-091. | UT-SH-13, BD-SH-01 |
| SH-15 | A sales reply MUST be rejected if it uses the wrong site or includes a field outside the aggregate list (SUT-2007). | UT-SH-14, PL-SH-05 |
| SH-16 | Sutradhar MUST NOT link an order to a visit or a person, and MUST show "origin not recorded" when the shop records none (SUT-2004). | UT-SH-15 |
| SH-17 | Sales releases MUST merge or suppress small cells before transfer and MUST preserve disjointness across time, settings changes and rollups (SUT-2005). | UT-SH-16, UT-SH-16A |
| SH-18 | Raw counts and revenue MUST be kept out of the console, custom metrics, exports, shared snapshots and outside alerts. Sales cells MUST NOT join group or portfolio views (spec 0102). | UT-SH-17 |
| SH-19 | Seasonal demand MUST say "not enough history" below the minimum years (SUT-2003) and name its source and sample size. | UT-SH-18 |
| SH-20 | A sales landing page draft MUST take prices and stock only from the shop record, mark them for checking and pass the checks of spec 0101. | UT-SH-19 |
| SH-21 | Headline shop alerts MUST carry finding counts only. Drafts and their evidence MUST use the local route. | UT-SH-20 |
| SH-22 | The settings of the table MUST be enforced at their bounds. | UT-SH-21 |
| SH-23 | The audit and stock handling MUST reuse spec 0047 and surface findings for the active site's products. | IT-SH-02 |
| SH-24 | Sales revocation, native correction and erasure MUST stop reads, discard late replies and withdraw the affected intervals. Removal, disablement and erasure MUST clear daily and monthly derived records under the lifecycle rules. | IT-SH-03 |
| SH-25 | Actions MUST enforce site and actor permission. Shop checks MUST add zero public-page queries, time and bytes. | IT-SH-04, AU-SH-03 |
| SH-26 | Markup corrections MUST use current, agreed facts for the same product and variant, or show the reason the proposal is unavailable. | UT-SH-07 |
| SH-27 | Seasonal verdicts MUST be withheld until the comparison rule and its fixtures are admitted. Missing weeks MUST remain gaps. | UT-SH-18 |
| SH-28 | JSON-LD built from shop names, descriptions or other shop text MUST escape script-closing sequences, quotes and markup, and MUST be built from typed fields. | UT-SH-07A |
| SH-29 | Forum thread text used in a Q&A draft MUST be treated as data, never as an instruction, MUST be length-capped and MUST be escaped for where it lands. | UT-SH-12A |

| Feature | Specific requirements |
|---|---|
| 579, 580 | SH-23, SH-12 |
| 581 | SH-05 |
| 582 | SH-06 |
| 583 | SH-07 |
| 584 | SH-08, SH-26 |
| 586 | SH-09 |
| 587 | SH-10 |
| 588 | SH-11 |
| 589 | SH-03, SH-12 |
| 590 | SH-13, SH-21 |
| 591 | SH-19, SH-27 |
| 592 | SH-14, SH-15, SH-16, SH-17, SH-18, SH-24 |
| 593 | SH-20, SH-21 |

SH-01–04, SH-22 and SH-25 apply wherever their rules govern an action.

### Named shop evidence and proposal fixtures

These planned spec 0076 cases use synthetic WooCommerce facts,
approval and observation receipts. QR-149 and the source-specific
questions hold profile and host admission. DR-083 keeps the
active shop scope. No case creates a new public data route.

- `DES-0103-W2-SHOP-EVIDENCE` uses the named observation
  variants below. Reuse 0047, 0044 and 0115 as applicable.
  F-0585 remains a 0047 finding. Product, review and free listing
  guidance keep separate dated profiles. Unknown facts stay unknown.
- `DES-0103-W2-LOCAL-SHOP-DRAFT` uses the exact evidence and
  existing permission rules for the named local proposals.
  An eligible link enters 0028/0029 review. Other proposals
  confer no markup, product or publishing write.
- `DES-0103-W2-PROTECTED-SALES` uses
  `VAR-0103-W2-NATIVE-DAY-RELEASE` only for the existing
  `ReadShopSalesDay` under DR-084. Use separate exact owner
  approval and existing signed native lease and withdrawal checks.
  Unadmitted storage starts no worker. The PC display gets only
  approved aggregate rows, with no order, person, marginal join,
  console, MCP, export or alert projection.

| Feature | Requirement | Design and variant | Fixture and test | Named fixture assertion |
|---|---|---|---|---|
| F-0579 | SH-01, SH-04, SH-23 | DES-0103-W2-SHOP-EVIDENCE, VAR-0103-W2-CURRENT-PRODUCT-FACTS | FX-0103-W2-PRODUCT-AUDIT, UT-SH-01, UT-SH-03, IT-SH-02 | Match current WooCommerce product and variant facts to one observed page revision. Incomplete crawl or unsupported host gives unknown or no shop. Reuse the owning check and no duplicate price finding. |
| F-0580 | SH-12, SH-23 | DES-0103-W2-SHOP-EVIDENCE, VAR-0103-W2-CONFIRMED-STOCK | FX-0103-W2-STOCK-SITEMAP, UT-SH-11, IT-SH-02 | Find a proved unavailable product still in the sitemap. Distinguish withdrawn variant and stale stock. Only confirmed current availability permits a new link candidate. No native change. |
| F-0581 | SH-03, SH-05 | DES-0103-W2-SHOP-EVIDENCE, VAR-0103-W2-BOUNDED-FACET-REPORT | FX-0103-W2-FACET-TRAP, IT-SH-01, UT-SH-04 | Count distinct visited parameterised addresses from one crawl. Two distinct paths sharing one canonical stay distinct. Show the value-product estimate separately. A blocked or capped observation cannot invent crawl share. No extra request or rule write. |
| F-0582 | SH-06 | DES-0103-W2-LOCAL-SHOP-DRAFT, VAR-0103-W2-THIN-CATEGORY-OUTLINE | FX-0103-W2-CATEGORY-OUTLINE, UT-SH-05, PL-SH-02 | Use a proved extracted introduction at the exact configured threshold and an unread control. Missing product-list baseline holds its verdict. Offer only an eligible sourced outline, with no invented price or stock. |
| F-0583 | SH-07 | DES-0103-W2-SHOP-EVIDENCE, VAR-0103-W2-BOUNDED-TEXT-COMPARE | FX-0103-W2-MANUFACTURER-SIMILARITY, UT-SH-06 | Compare exact and close fingerprints on compatible samples. Use a licensed local file or already admitted one-read evidence only in its own scope. Withdrawal holds the comparison. No claim of copying, source quote or refetch. |
| F-0584 | SH-08, SH-26, SH-28 | DES-0103-W2-LOCAL-SHOP-DRAFT, VAR-0103-W2-TYPED-PRODUCT-FACTS | FX-0103-W2-MARKUP-PROPOSAL, UT-SH-07, PL-SH-03, UT-SH-07A | Compare exact product and variant under current visible and native facts. Conflicting currency or markup-only price blocks proposed values. Escape the eligible typed preview. No markup write or validator send. |
| F-0586 | SH-08, SH-09 | DES-0103-W2-SHOP-EVIDENCE, VAR-0103-W2-DATED-REVIEW-PROFILE | FX-0103-W2-REVIEW-SNIPPET, UT-SH-07, UT-SH-08 | Check a current visible eligible first-party rating with its scale and count. Stale or markup-only ratings and an unadmitted rules version give no eligibility proof. No search appearance promise. |
| F-0587 | SH-10 | DES-0103-W2-SHOP-EVIDENCE, VAR-0103-W2-LOCAL-LISTING-CHECKLIST | FX-0103-W2-FREE-LISTING, UT-SH-09, AU-SH-02 | Show dated own-site price and policy facts. Missing shipping or a stale profile withholds readiness and any ready count. Refuse a merchant login, remote request or feed upload. |
| F-0588 | SH-11 | DES-0103-W2-SHOP-EVIDENCE, VAR-0103-W2-OBSERVED-IMAGE-FACTS | FX-0103-W2-PRODUCT-IMAGE, UT-SH-10 | Use existing observations for dimensions, decode and description. An observed broken decode and inaccessible evidence show separate states. Local guidance adds no fetch or media effect. |
| F-0589 | SH-03, SH-12 | DES-0103-W2-LOCAL-SHOP-DRAFT, VAR-0103-W2-ELIGIBLE-LINK-REVIEW | FX-0103-W2-GUIDE-LINK, IT-SH-01, AU-SH-01, UT-SH-11 | Allow a relevant, in-stock same-site destination with current relation marks to enter review. Off-topic popularity, missing marks, unknown stock and foreign scope cannot qualify. No new insertion authority. |
| F-0590 | SH-13, SH-21, SH-29 | DES-0103-W2-LOCAL-SHOP-DRAFT, VAR-0103-W2-SCRUBBED-LOCAL-ANSWER | FX-0103-W2-PRODUCT-QA, UT-SH-12, PL-SH-04, UT-SH-12A, UT-SH-20 | Pin an eligible same-site staff or accepted reply. Strip identity and treat source text as data. Unaccepted or foreign discussion gives no sourced answer. Loss of local helper cannot cause an outside call. |
| F-0591 | SH-19, SH-27 | DES-0103-W2-SHOP-EVIDENCE, VAR-0103-W2-MATCHED-WEEK-BLOCKS | FX-0103-W2-SEASONAL-DEMAND, UT-SH-18 | In the proposed profile compare matched closed blocks for two years, 49 versus 50 baseline events, and exact ratio 1.20 versus below. Incomplete weeks, changed scope or missing admission give no recurrence verdict. |
| F-0592 | SH-14, SH-15, SH-16, SH-17, SH-18, SH-24 | DES-0103-W2-PROTECTED-SALES, VAR-0103-W2-NATIVE-DAY-RELEASE | FX-0103-W2-SEALED-SALES-DAY, UT-SH-13, UT-SH-14, UT-SH-15, UT-SH-16, UT-SH-16A, UT-SH-17, BD-SH-01, PL-SH-05, IT-SH-03, IT-SH-04 | Simulate sealed native payment and refund races in each admitted profile. One release covers all currencies and exact replay returns its receipt. Empty and suppressed days share coverage. Late revoked display or restore gives no values. |
| F-0593 | SH-20, SH-21 | DES-0103-W2-LOCAL-SHOP-DRAFT, VAR-0103-W2-PINNED-EVENT-PROPOSAL | FX-0103-W2-SALES-EVENT, UT-SH-19, UT-SH-20 | Pin a local event draft to exact native price, stock, interval and disclosure. Withdraw evidence or change the offer before copy. Hold the claim for review with no price change or publication. |

## How it is tested

Designed, not run.

| Test | Designed case |
|---|---|
| UT-SH-01 | Active WooCommerce, inactive WooCommerce, other WordPress and XenForo. |
| UT-SH-02, PL-SH-01 | An allowed public policy page passes. Cart, account and unauthorised documents are refused. |
| UT-SH-03 | Compare true zero, unavailable and stale observations with their source records. |
| UT-SH-04 | Try a group below, at and over the proposed threshold, a zero denominator, an expanding path and a search-traffic warning. |
| UT-SH-05, PL-SH-02 | Thin, full and missing introductions. Refuse invented prices and accept a sourced outline. |
| UT-SH-06 | Compare a shop duplicate, approved fingerprint, authorised file and once-read page. Refuse an unapproved group. |
| UT-SH-07, PL-SH-03 | Agreeing, stale, wrong-variant and conflicting facts. Refuse rating markup without visible reviews. |
| UT-SH-07A | Shop names, descriptions and brands holding `</script>`, `<!--`, double quotes, backslashes, `&`, U+2028 and a control character. The proposed JSON-LD parses as JSON, equals the typed input after decoding, and holds no raw `<`, `>` or `&`; embedded in a page it leaves exactly one script element. A value that cannot be escaped is refused. |
| UT-SH-08 | Each snippet verdict and the unavailable verdict before dated rules are admitted. |
| UT-SH-09, AU-SH-02 | A checklist without admitted rules shows no readiness count. Verify zero merchant-account requests. |
| UT-SH-10 | Exercise each image finding and verify grouping by product and cause. |
| UT-SH-11 | A relevant, available product passes. Out-of-stock, unknown and cross-site targets are excluded. |
| UT-SH-12, PL-SH-04 | A guest-visible staff answer passes. Strip member names. Show no sourced answer with no qualifying reply. |
| UT-SH-12A | A hostile forum post: it says "ignore previous instructions", asks to link an outside site, claims to be staff, and holds markup and a `</script>` string, at, below and above the 2,000-character excerpt cap (proposed). The draft follows its normal rules, gains no link, tool or target, is cut at the cap with a notice, and shows the text escaped in review and in any proposed output. |
| UT-SH-13, BD-SH-01 | Given approval or no approval, show sales as unavailable before admission. After admission, only the owner can enable the read after notice. |
| UT-SH-14, PL-SH-05 | Check the allowed site-bound reply and indistinguishable empty and suppressed coverage. Reject wrong site, duplicate key, invalid Other pair, below-floor count, invalid money, extra or customer field. Logs contain no rejected values. |
| UT-SH-15 | Unknown groups, paid and unpaid states, cancellations, partial and full refunds, duplicate order records, currency separation, and both clock-change days. |
| UT-SH-16 | Check cells and the disjoint Other row below, at and above the floor. Try totals, overlapping changes, repeated reads, rollups, and withdrawal after display. |
| UT-SH-16A | Dominance: a cell with revenue 80% from one order (at the proposed limit), one with 81% from one order and one with 79%, each with counts above the floor. The 81% cell is merged into Other or hidden, 80% and 79% show, and Other stays disjoint and above the floor, or the whole remainder is omitted. A refund or a second order that moves a cell across the limit changes the verdict on the next release only. |
| UT-SH-17 | Try a sales count or value through each blocked export, query, metric, snapshot and outside alert route. |
| UT-SH-18 | Compare incomplete and full years, missing weeks, the year boundary, one-off and repeated changes and a rule pending admission. |
| UT-SH-19 | Check the source, stale price wording and disclosure checks for a sales landing page. |
| UT-SH-20 | Verify count-only alerts. With the local Coach absent, drafting cannot send evidence to an outside helper. |
| UT-SH-21 | Exercise each proposed setting at its bounds and just outside. |
| IT-SH-01, AU-SH-01 | Review a shop change without writing. On a separate eligible link, verify moderator insertion and undo through the existing route. |
| IT-SH-02 | Use the same fixtures as spec 0047 for variant and unavailable-product findings. |
| IT-SH-03 | Revoke approval while a read is pending. Cut the link and expire the lease. Retry revoke, then re-pair. Withdraw a release after daily expiry. Check local view, rollup, restore and replay never disclose retired values. |
| IT-SH-04, AU-SH-03 | Try a wrong site and actor. Interrupt the paged scan with payment, a late currency, refund, erasure, timeout and slowdown. Measure both storage modes and compare public-page queries, time and bytes with checks off and on. |

In the fictional Lena example, a moderator reviews the filter proposal
and sees the matching product facts. The site stays as it was until
staff edit it in the site's own tools. With the sales reader admitted
and the owner opted in, only privacy-checked sales values appear.

## Changes from the source

The feature register gives the consistency report (585) to spec 0047. Spec
0103 owns the other shop features and reuses spec 0047 checks for 579 and 580.
By the owner's decisions of 20 November 2026, version 1 builds only
WooCommerce checks and proposals (DR-083) and allows sales attribution with
counts and revenue per landing page, aggregated, with no customer data and
with safeguards (DR-084). The merchant feed integration is not built (DR-076);
the readiness check is a checklist only.
