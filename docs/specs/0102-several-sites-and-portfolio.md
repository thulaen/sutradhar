# 0102: Several sites and the portfolio view

This specification owns how one Sutradhar installation serves more than one
registered site: how sites are kept apart, how work is shared fairly between
them, the read-only portfolio dashboard that shows them side by side, and the
opt-in check for duplicate content across the owner's own sites. It also lists
the language and market features that the owner deferred past version 1.

The owner decided on 20 November 2026 that version 1 supports several sites
for one owner (DR-082) and handles sites written in British English only, so
the language and market features are after version 1 (DR-081). Sutradhar is
still the owner's own desktop app: a "site" here is a registered XenForo or
WordPress site, and every site belongs to the same owner. Other people reach a
site through that site's own sign-in (spec 0101); they never see another site.

Site data has a site column and forced row-level security. Each
site transaction binds exactly one site under spec 0108. The separate
installation scope under DR-150 is for its named lifecycle records.
A token sees one site only (spec 0020);
each project has its own keys, vault, queues and memory slice (spec 0116);
work is shared by the turn-taking rules (spec 0078); the home folder layout
and removal are spec 0030's.

Example: Omar runs a cycling forum, a sister blog and a walking forum. The
portfolio view shows three tiles. The blog's tile says "Attention: connection
expired" because its Search Console sign-in needs renewing; the walking
forum's tile shows 4 open high-priority fixes; the cycling forum's tile is
calm. A crawl of the walking forum runs while the blog waits its turn, and the
screen says "Waiting for the walking forum's crawl to finish". Omar clicks the
blog's tile and the app switches to that site; the header now shows the blog's
name in its own colour. Later he switches on "look for duplicate content
across my sites" for the two forums. The result lists 14 page pairs, each with
a similarity figure and a proposal to choose one canonical page; nothing is
changed anywhere.

Values marked "proposed" are design choices to be checked by measurement.

## What it does

### Words used here

- **Site**: one registered XenForo or WordPress site with its own add-on link.
- **Current site**: the site the app is acting on now.
- **Site context**: the site a database transaction, job or token is limited to.
- **Portfolio**: all of the owner's registered sites together.
- **Site group**: sites the owner has chosen to compare with each other.
- **Tile**: one site's summary on the portfolio view.

### What this spec designs

| Feature | What the user sees |
|---|---|
| 557 Portfolio dashboard | One screen with a tile for every site |
| 558 Cross-site duplicate content | Duplicate pages across the owner's own sites, found only on request |

### Several sites in one installation

1. Registration, the site link, keys and removal are those of specs 0021 and
   0030; this spec adds only the rules below. Up to `sites.max` sites may be
   registered (default 10, range 1 to 50, both proposed and measured by MS-
   SS-02). Names and site links must be unique; registering the same site
   twice, or two sites with one link or one site id, is refused (SUT-1907). A
   site has a name, a platform, a status (active, paused, disconnected) and
   its own settings profile (spec 0014). The training sandbox of spec 0101 is
   not a site: it does not count towards the limit and cannot be in a
   portfolio or a group. A rival cannot be registered as a site, because
   registration needs the add-on's keys.
2. The site register is the file in the home folder that spec 0030
   owns. It lists sites without opening a database transaction.
   Each site summary still requires its own site context and forced
   row-level security (spec 0108).
3. Separation. Every site has its own rows, keys, vault, journal, queues,
   memory slice and files. Every site job, site transaction, token, MCP
   session, workflow, script and `sutra` command binds one site when it
   starts. A query that spans sites is refused (SUT-1901). An
   installation-only binding cannot list or mutate site data. Spec 0119's
   owner coordinator has no database
   handle. Each of its reads remains a separately authorised one-site
   job. Test stale pooled context and revocation with IT-0119-SCOPE.
4. Only the owner works across sites, and only in the desktop app. The app
   uses an owner session. Every site-changing call carries an explicit
   site id. The service compares it with the window's current site and
   refuses a mismatch (SUT-1904). Site accounts, agent tokens and MCP
   sessions each see one site (specs 0020 and 0101).
5. Removing a site follows spec 0030, and this checklist is tested item by
   item: the site's rows, keys, vault, archive and files; its portfolio-cache
   and group-fingerprint rows; its outside-service connections (tokens
   revoked, spec 0100) and ticket links (marked disconnected); its place in
   any group (the group re-checks without it); its backups and snapshots (kept
   only if the owner chooses; an erasure list is applied on restore); and its
   registry entry. The audit entries of the registration and the removal are
   kept in the install's audit area under spec 0032's retention. Rebuild
   fingerprints only for restored sites and groups still admitted after
   the restore checks in spec 0030.
6. Fair turn-taking is spec 0078's: each site has `site.memory_weight` (1 to
   10) and takes turns by weighted fair queueing with aging, never more than
   half the burst pool while another site waits (MB-08). A site's priority in
   this spec is that weight; there is no second algorithm. A paused site has
   no share. A waiting job shows the site it waits for in plain English. A
   cage of a site's crawl counts against that site's slice.
7. Per-site rules stay per site: quiet hours, politeness caps of spec 0082,
   content levels and destinations of spec 0100, grants of spec 0101 and
   retention.
8. No wrong-site mistakes. Every screen shows the current site's name, with
   its own colour, in the header. Every confirm dialog for a change names the
   site. There is no "apply to all sites" for changes in version 1, in any
   entry point: screens, workflows, scripts, scheduled jobs, the command line
   and agent tokens. The portfolio, duplicate check and
   [spec 0119](0119-cross-site-link-suggestions.md) each own a specific
   read path. No path grants a general multi-site query or write.
9. Each window remembers its own current site as an app preference (not a
   setting of the service). Two windows may show two sites, and a change in
   one carries that window's site id (item 4). A window whose site was removed
   or paused returns to the portfolio view. A new window starts on the
   portfolio if there is more than one site, otherwise on the only site.

### The portfolio dashboard

1. The portfolio view is owner-only and is not exposed through MCP, because an
   MCP session sees one site (spec 0020). The service builds it by listing the
   sites from the site register and, for each, opening a transaction under
   that site's own context (spec 0108, with the pooled connection reset each
   time) with a small pool of workers (`sites.portfolio_workers`, default 4).
   No single query crosses sites.
2. Each tile reads one summary row of `portfolio_cache` for its site (a site-
   scoped table of counts, refreshed by the service) and shows: the site's
   status; open fixes by the severity names of spec 0048; the last crawl's
   time and result (spec 0044); suggestions waiting for review (spec 0028);
   trends of visits (spec 0034) and search clicks (spec 0040) where connected;
   open alerts (spec 0099); and any connection that needs renewing (spec 0040)
   or failing destination (spec 0100). The tile carries counts, statuses and
   times only: never a page title, address, name or alert text. Each of these
   specs exposes the summary read below. A figure that is not
   connected shows "not connected". Each figure carries its source
   time. The tile shows "last updated" for its cache generation.
3. Tiles are ordered by an exact rule, shown on the screen. First sites with a
   failing connection or a stalled job (a job that has made no progress for
   `sites.stalled_min` minutes), then sites with open fixes of the highest
   severity, then the rest. Within each class sort by site name and
   then stable site id. Show "no attention items" only when all
   relevant flags are current and clear. Otherwise label the missing
   or stale checks. Sites are never ranked against each other by quality, and
   no total or average across sites is shown as a score.
4. One site's failure never breaks the screen. If a site's data cannot be
   read, its tile says "could not read this site" with the reason, and the
   others load (SUT-1903, information).
5. The cache is refreshed every `sites.portfolio_refresh_s` seconds and on
   demand. A disconnected or paused site keeps its last permitted
   figures, marked stale with its status. Proposed warm-cache targets
   on the 8 GiB reference: ten sites in under two seconds and fifty
   in under six (MS-SS-01 and MS-SS-02).
6. The owner may export one site's tile data as a snapshot (spec 0100). There
   is no cross-site export in version 1.
7. A tile opens the site: the window's current site switches and the site's own
   overview shows.

### Summary reads and refresh

1. Within one site's refresh, use one bounded summary read per owning
   module. Each returns at most one fixed-shape row for the selected
   site. It carries the module's counts or times, source version and
   time, and coverage state. No read scans a history, a queue, page
   rows, or all the site's connectors. A module maintains its summary
   from its own committed records. Use the same transaction for
   synchronous updates. A scheduled rebuild checks the source
   version before publishing and remains unavailable until complete.
2. Fix list, crawl, review, visits, search, jobs and destinations use
   the reads in specs 0048, 0044, 0028, 0034, 0040, 0099 and 0100.
   A completed source may report zero. A disabled, incomplete,
   expired or unreadable source reports its state separately. A
   failure reason is a safe code. At display, classify each flag as
   current, not applicable or unknown. A flag goes unknown after one
   configured refresh interval without a successful refresh. Use
   only current flags to sort and label missing checks. Compare the
   earliest running-job progress time returned by 0099 with the
   current service clock at display. A current summary gives a stall
   when elapsed time reaches `sites.stalled_min` minutes. A lease renewal
   leaves the committed progress time unchanged. A current summary
   with no running jobs clears the flag. A stale or unavailable
   jobs summary leaves the check unknown. An unproved service clock
   or progress timestamp later than that clock also leaves it unknown.
   A job can become stalled without a new event. Stall checks for
   paused sites are not applicable.
3. Commit one site-scoped cache generation. At refresh, bind each
   field to the site lifecycle generation and the owning module's
   current authority generation. The owner module advances its
   authority fence when erasure, scope replacement or withdrawal
   takes effect. Check these fixed-size fences at publication and
   again at delivery of a view or a prepared export. Order the final
   check and hand-off against invalidation. Invalidate affected
   figures in open views and discard any undelivered export. If a
   fence cannot be read, withhold the figure. Paused caches use the
   same checks. Older refreshes cannot replace newer data.
4. A refresh reads local summaries only. No site or provider fetch,
   crawl, raw query result, or job dispatch is part of a tile read.
   The source repair or rebuild stays with its module. Admission in
   QR-090 requires a query cost independent of source history size.

### Duplicate content across the owner's own sites

1. The check is off by default (`sites.dup_enabled`) and works only on a site
   group of two or more sites that the owner chooses (at most
   `sites.groups_max` groups of at most `sites.group_members_max` sites).
   Choosing a group is an action only the owner can take. The group is
   recorded as a group id in each member site's own rows, so there is no table
   that spans sites.
2. Only guest-visible, indexable main content may be compared. A site flagged
   private or staff-only cannot join a group (SUT-1906), and any page that is
   not guest-visible is left out, so a pair can never reveal that private text
   exists (specs 0021, 0041 and 0022).
3. Each site's own crawl makes, as spec 0116 describes, a similarity
   fingerprint of each page's main content (spec 0115). For a group check a
   small derived record per page is kept in that site's own rows
   (`group_fingerprints`, site column, forced row-level security): the page
   id, the 64-bit fingerprint, its comparison parts and a sketch of the page's
   word groups (64 values of 32 bits, 256 bytes). The sketch lets the
   confirming overlap be estimated without any text. A record is about 300
   bytes, so the default limit costs about 600 MB. It is encrypted with that
   site's own project key (specs 0065 and 0116).
4. Matching runs in the service over each site's sorted fingerprint stream,
   one site at a time under its own context, with the part-table scheme of
   spec 0116; candidates are confirmed by the estimated word-group overlap at
   or above `crawl.confirm_jaccard` (spec 0116). No query crosses sites, no
   page text, address list, vault file or key crosses a site boundary, and
   spec 0116's isolation rule is amended to allow only this derived exchange
   (spec 0116, item on fingerprints).
5. A pair shows both addresses, the similarity figure and the sample size, and
   says "near-duplicate", never that one page was copied or by whom.
6. For each pair the check proposes advice (choose one page as the main one
   and point the other to it, or rewrite one) in the wording of the Editor
   Coach (spec 0041, which owns the advice wording of feature 558). A proposal
   changes nothing; a human accepts any change on the site through the review
   page.
7. Each site's `group_fingerprints` is limited (`sites.dup_max_rows`, default
   2,000,000). Past the limit the check stops adding records for that site and
   the report says it is incomplete, naming the number of rows and the disk
   they use (SUT-1905); it does not silently drop old pages.
8. Removing a site, removing it from the group, or an erasure request (spec
   0032) deletes that site's group rows at once, and a subject erased on one
   site is erased from every site's records that hold it.
9. Only registered sites are compared. A rival or any other site cannot be in
   a group, so Leviathan's one-read rule (DR-062) and the legal wall of spec
   0082 stand.

### Portfolio and market acceptance (557–568)

The owner opens one portfolio view generation. Each tile pins
site, admitted source and summary generations, read time, coverage
and owner grant. Fetch each via a fixed-site reader and reset
the pooled connection. Apply the summary bounds and field fences
above. An unreachable but still granted site can show a dated
cache only if local authority and field generations remain
provable. Cache reads start no network. An unproved grant or
generation withholds its values. A removed or revoked tile
invalidates open views, cursors and pending exports. Recheck
the exact fields and site at display, file publication and copy.
The portfolio has no staff, MCP, CLI or workflow entry. The
owner can export one eligible tile under 0100.

An on-demand pair check pins group generation, exact registered
site IDs, grants, indexability, source and fingerprint revisions.
An owner-only coordinator has no database handle. Request
bounded protected fingerprint streams and page-reference
reads from separate fixed-site jobs. Reject polluted pooled
context, partial group restore or an unproved member. Do not
exchange raw text, URLs, keys or role context between stores.
Use one compatible 0116 tokenizer and normalisation profile
and its signed `crawl.simhash_hamming` and `crawl.confirm_jaccard`.
An incompatible profile or cap gives an explicit incomplete
result. Show “No match” only for a complete comparison. Give
sample and thresholds without identifying withheld pages.

Recheck membership, page refs, rights, current indexability
and source generation via the separate readers immediately
before delivering or reusing a pair. A current fence orders
delivery and any saved cache against withdrawal. One changed
or unproved member invalidates dependent pairs and cursors.
Hold at most 256 MiB RAM and 128 MiB encrypted scratch per
coordinator under 0078. Cap active work at 15 minutes. Ledger
all scratch and protected copies under 0030/0032/0118. On stop
or restart erase incomplete scratch before reuse. Rebuild from
the current signed group only. Never restore obsolete results.

The duplicate check is distinct from 0119, which admits a
separate directed cross-site link suggestion under DR-100.
A duplicate result or proposed primary page creates no pair
grant, ranking input, canonical change, redirect or link effect.
Use 0041 wording only in local review. Keep the current site's
own review path for any separately approved native effect.

Feature 564 inspects already admitted own-site declarations
through 0047. Report an observed malformed own-page declaration.
A foreign, unread or stale target has unknown status. Reciprocity,
canonical or absent-target findings need comparable complete
same-site observations. Absence is distinct from unread coverage.
No unauthorised fetch, market recommendation or group match. Features 559–563
and 565–568 remain after
version 1 in their own scope. DR-113's multilingual writing and
DR-118's named Coach market lookups use their own exact approvals.
DR-081 governs site SEO, so approved writing runs under 0121/0124
retain their languages. Keep 0040 country evidence, 0095 local
listing checks and 0096 local plans in scope. No deferred site
action, setting, quota or job through file, template, MCP or restore.
These exceptions grant no translation memory or market comparison.

#### Wave 2 portfolio and deferred market-route bindings

Use synthetic owner sites and observations. QR-147 still governs
owner isolation, proofs of source and the separation from separately
admitted writing, shop and named Coach routes. SS-17 remains a
refusal boundary for the deferred market features.

| Feature | Owning requirement and design | Planned fixture and expected evidence |
|---|---|---|
| F-0557 | SS-02, SS-03, SS-07, SS-08, SS-09, SS-10, SS-25, SS-26, SS-28 and SS-29 · DES-0102-W2-FIXED-SITE-TILES | FX-0102-W2-FIXED-SITE-TILES · IT-SS-06, BD-SS-04 and PL-SS-02. Use two owner windows and separately scoped bounded local summaries with source dates, stale facts, unavailable data and one removed site. Refuse mixed transaction context. Keep only independently authorised current or dated stale figures. A removed grant fences its tile and undelivered one-site export. No broad export, combined score, MCP or other multi-site route. Use IT-SS-06 for the clock-driven Jobs attention tier, current-empty and unknown cases. Reuse FI-SS-01 for independent tile failures. |
| F-0558 | SS-12, SS-13, SS-14, SS-15, SS-16, SS-22 and SS-23 · DES-0102-W2-EXACT-OWNER-DUPLICATE-GROUP | FX-0102-W2-EXACT-OWNER-DUPLICATE-GROUP · UT-SS-09, UT-SS-10, UT-SS-11, UT-SS-12, PL-SS-03 and IT-SS-04. Use an exact owner-approved own-site group, bounded derived records and matching signed comparison profiles. Try a near-match boundary, incomplete cap, a changed or private page, partial restore and revocation before delivery. Keep safe pair, sample and similarity evidence only while current. A complete no-match differs from an incomplete check. No new fetch, raw cross-store exchange or 0119 grant. Use 0041 wording in local advice only. |
| F-0559 | SS-17 · DES-0102-W2-MARKET-ROUTE-REFUSAL | FX-0102-W2-MARKET-CHOICE-REFUSAL · AU-SS-01. Attempt market selection from a portfolio and again after a separately admitted Coach observation. Require no market setting, choice, task or inferred recommendation. Existing observations retain their exact source and scope under DR-081. |
| F-0560 | SS-17 · DES-0102-W2-MARKET-ROUTE-REFUSAL | FX-0102-W2-MARKET-CURRENCY-REFUSAL · AU-SS-01. Try an international currency and unit audit from a shop or writing task. Deny the market route. Existing own-site shop evidence and independently authorised local text checks keep their own scope. No conversion, product edit or market effect. |
| F-0561 | SS-17 · DES-0102-W2-MARKET-ROUTE-REFUSAL | FX-0102-W2-SITE-TRANSLATION-MEMORY-REFUSAL · AU-SS-01. Request cross-site glossary and translation memory, then observe a synthetic separately admitted local writing check. Refuse the site memory, shared vocabulary store and related grants. DR-113 writing permissions leave the market boundary unchanged. |
| F-0562 | SS-17 · DES-0102-W2-MARKET-ROUTE-REFUSAL | FX-0102-W2-SITE-TRANSLATION-WORKFLOW-REFUSAL · AU-SS-01. Try to create a site-wide translation queue and attach an eligible local 0124 revision as prior authority. Deny the market workflow. Local writing proposals keep their own review and provide no translation publication permission. |
| F-0563 | SS-17 · DES-0102-W2-MARKET-ROUTE-REFUSAL | FX-0102-W2-COUNTRY-SERP-REFUSAL · AU-SS-01. Refuse a portfolio or generic country search-result comparison, including reuse of a current Coach approval. Verify only by stand-in that the independent DR-118/0133 named Coach plan keeps its exact approved target, source and budget. No automatic, widened or background request. |

### Deferred past version 1

By the owner's decision of 20 November 2026 (DR-081) these 24 features are not
built in version 1, and the register marks them "after-v1". Version 1 handles
sites in British English only, and the Sutradhar screens are British English
(ADR-0021). No code, setting or screen for them is added now:

- Market and number handling: 559 Market selection advice, 560 Currency and
  unit checks (the part that spec 0047 shares waits too), 565 Per-market
  reports, 563 Search results by country, 567 Regional content calendars, 568
  Cross-market link opportunities, 566 Local-language query clustering.
- Translation: 561 Translation memory and glossary, 562 Translation review
  workflow, 1215 Human editing levels, 1216 Style guide per language, 1220
  Language coverage tracker, 1222 Native-speaker reviewers, 1219 Name
  transliteration rules.
- Local adaptation: 1208 Cultural adaptation, 1209 Local formats, 1210 Local
  regulation notes, 1211 Local examples, 1212 Right-to-left checks, 1213
  Localised images, 1214 Local search terms, 1217 Localised calls to action,
  1218 Local holidays and events, 1221 Locale disclaimers.

Feature 564 stays as the country-targeting structure audit of spec
0047. It uses own-site hreflang and related observations from spec
0115, the reader. Spec 0047 owns the report and uses existing
same-site observations. Country aggregates and offer currency
remain evidence only. Spec 0095 owns local listing checks. Other
specs treat deferred languages and market features as after version 1
(QR-056). The old list had 26 features here: 24 deferred, plus 557 and 558.

#### Wave 2 remaining deferred market checks

These fixtures apply SS-17 and AU-SS-01 under DR-081 and QR-147.
No refusal changes authorised own-site evidence or extends the
F-0564 structural read. It cannot grant language placement.

| Feature | Design and planned fixture |
|---|---|
| F-0565 | DES-0102-W2-MARKET-ROUTE-REFUSAL · FX-0102-W2-MARKET-REPORT-REFUSAL · AU-SS-01. Request a per-market report and try a separately admitted 0040 country aggregate as authority. No market template, grouping or scheduled report is created. Show the source aggregate only under its original purpose and labels. |
| F-0566 | DES-0102-W2-MARKET-ROUTE-REFUSAL · FX-0102-W2-LOCAL-LANGUAGE-CLUSTER-REFUSAL · AU-SS-01. Use a synthetic admitted 0092 topic view, then request query clustering in another site language or across languages. The approved local source scope persists. No new language, market or terms intake. |
| F-0567 | DES-0102-W2-MARKET-ROUTE-REFUSAL · FX-0102-W2-REGIONAL-CALENDAR-REFUSAL · AU-SS-01. Open an existing approved 0096 site-local schedule, then request automatic creation of a regional calendar. Keep the local schedule in its own scope. No market calendar, provider operation, queue or posting. |
| F-0568 | DES-0102-W2-MARKET-ROUTE-REFUSAL · FX-0102-W2-CROSS-MARKET-LINK-REFUSAL · AU-SS-01. Try discovery from a portfolio and offer a separately admitted 0119 directed same-topic fixture as authority. Refuse market discovery, implied group grant, reverse direction or automatic application. Only the independently approved directed suggestion enters its own native review under DR-100 and IT-0119-SCOPE. |

### Localisation refusal checks (1208–1222)

The register assigns these fifteen localisation forms to spec
0102 after version 1 under DR-081. Apply SS-17 and QR-056 to
each action, discovery list, command, setting, queue, restore
and hand-off. No version 1 implementation, control, provider
call or job for any of the forms can be enabled by an import,
preset, restored record or edited request. Exclude deprecated
requests from discovery and fail safely if one reaches a
supported entry point.

DR-113's admitted writing-suite text languages and dialects in
specs 0121 and 0122 do not change site or market eligibility.
Independent suite operations, including the separately admitted
selected-text translation in 0124, retain their exact text scope. Own-site
structure checks in feature 564, native
site data, local dates, evidenced native currencies, existing jobs and records
keep their existing contracts. None grants the deferred
market, translation review or site localisation action.
The Wave 3 acceptance rows trace each refusal separately.

### Data

- The site register: the home-folder file of spec 0030. Site weight
  comes from its existing setting.
- `site_group_links`: site, group id (in each member site's own rows).
- `portfolio_cache`: one summary row per site, with counts, times,
  coverage and source generations.
- `group_fingerprints`: site, page id, fingerprint, comparison parts and word-
  group sketch, encrypted with the site's key.
- Every database table here has the site column and forced row-level security.
  Window state is an app preference.

### Settings

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `sites.max` | whole number | 10 | 1 to 50 | next registration |
| `sites.<site>.weight` (spec 0078's `site.memory_weight`) | whole number | 1 | 1 to 10 | between jobs |
| `sites.<site>.paused` | on or off | off | two values | at once |
| `sites.portfolio_refresh_s` | seconds | 60 | 10 to 3,600 | next refresh |
| `sites.portfolio_workers` | whole number | 4 | 1 to 16 | next refresh |
| `sites.stalled_min` | minutes | 15 | 1 to 1,440 | next refresh |
| `sites.groups_max` | whole number | 5 | 1 to 50 | next group |
| `sites.group_members_max` | whole number | 10 | 2 to 50 | next group |
| `sites.dup_enabled` | on or off | off | two values | next run |
| `sites.dup_max_rows` | whole number per site | 2,000,000 | 10,000 to 50,000,000 | next run |

There is no setting that lets one query span sites, shows a site's data to
another site's account, applies a change to several sites at once or compares
a site that is not registered.

### Failure codes

SUT-1900 site limit reached; SUT-1901 cross-site query refused; SUT-1902 site
not in the group; SUT-1903 portfolio tile could not be read (information);
SUT-1904 change refused, the named site is not the current site; SUT-1905 a
site's group records over their limit, check incomplete; SUT-1906 a private or
staff-only site cannot join a group; SUT-1907 site already registered or name
or link in use. Codes SUT-1900 to SUT-1999 are reserved for this spec.

### Security and privacy

- Sites are separated by row-level security, keys and process limits; no page
  text crosses between sites. Private and staff-only content never enters a
  group.
- The portfolio shows counts and statuses only, never page text, names of
  people or secrets.
- Group fingerprints are derived, encrypted and removed with their site or on
  erasure.
- Every registration, removal, pause, group change and duplicate-check run is
  audited (spec 0032).

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| SS-01 | The number of registered sites MUST NOT exceed `sites.max` (SUT-1900). | UT-SS-01 |
| SS-02 | Each site job, site transaction, token and session MUST have exactly one site context. Refuse a mixed or cross-site query (SUT-1901). An installation binding MUST NOT resolve a site, portfolio or removal grant. | UT-SS-02, PL-SS-01, UT-SS-16 |
| SS-03 | Only the owner, in the desktop app, MUST be able to work across sites, and no site account, token or MCP session MUST see more than one. | UT-SS-03, IT-SS-01 |
| SS-04 | Removing a site MUST follow the removal checklist (rows, keys, vault, archive, files, cache and group rows, outside connections, ticket links, group membership, backups and registry entry) after a confirm naming the site. | IT-SS-02, IT-SS-05 |
| SS-05 | Heavy jobs MUST take turns between sites under spec 0078's weighted fair queueing (MB-08) with each site's weight, and a paused site MUST start no new work. | UT-SS-04, IT-SS-03 |
| SS-06 | Every screen MUST show the current site's name, every change confirm MUST name the site, and a change MUST NOT apply to several sites at once (SUT-1904). | UT-SS-05, BD-SS-01 |
| SS-07 | The portfolio MUST be built one site at a time under each site's own context, and MUST NOT be exposed through MCP. | UT-SS-06, PL-SS-02 |
| SS-08 | A figure that is not connected MUST show "not connected", never zero. | UT-SS-07 |
| SS-09 | Tiles MUST be ordered by the stated attention rule, sites MUST NOT be ranked by quality, and no cross-site score MUST be shown. | UT-SS-08 |
| SS-10 | One site's failure MUST NOT stop the other tiles from loading (SUT-1903). | FI-SS-01 |
| SS-11 | Ten sites MUST load on the portfolio view within the stated target on the 8 GiB reference. | MS-SS-01 |
| SS-12 | The duplicate check MUST be off by default, run only on a group chosen by the owner, and copy only derived, encrypted records between sites. | UT-SS-09, PL-SS-03 |
| SS-13 | A site not in the group MUST NOT be compared (SUT-1902), and an unregistered or rival site cannot be registered. | UT-SS-10 |
| SS-14 | Duplicate results MUST be confirmed pairs with similarity and sample size, and any advice MUST be a proposal that changes nothing. | UT-SS-11, BD-SS-02 |
| SS-15 | Removing a site or leaving a group, or an erasure request, MUST delete that site's group rows at once. | IT-SS-04 |
| SS-16 | The group table MUST respect `sites.dup_max_rows` and report an incomplete check (SUT-1905). | UT-SS-12 |
| SS-17 | The deferred language and market features MUST have no code, setting or screen in version 1. | AU-SS-01 |
| SS-18 | Each setting of the table MUST be enforced at its bounds, one test per setting. | UT-SS-13 |
| SS-19 | Every site-changing call MUST carry an explicit site id equal to the window's current site, and a mismatch MUST be refused (SUT-1904). | UT-SS-14, BD-SS-03 |
| SS-20 | A workflow, script, scheduled job, command and agent token MUST each have one site context fixed at start, and a cross-site call MUST be refused (SUT-1901). | UT-SS-15, PL-SS-04 |
| SS-21 | The 0030 native site register MUST be readable by its authorised owner route before opening site transactions. The separate DR-150 database scope serves only the admitted 0030 and 0108 lifecycle records under their restricted roles. It gives no authority for site removal. | UT-SS-16, IT-BD-06 |
| SS-22 | A private or staff-only site MUST NOT join a group (SUT-1906), and a non-guest-visible page MUST NOT yield a fingerprint. | UT-SS-17, PL-SS-05 |
| SS-23 | The group records MUST be per site, encrypted with that site's key, hold no text, and be compared in the service one site at a time. | UT-SS-18 |
| SS-24 | A site name, link or id used twice MUST be refused (SUT-1907), and the training sandbox MUST NOT count, join a portfolio or a group. | UT-SS-19 |
| SS-25 | Two windows MUST keep their own current site, and a removed or paused site's window MUST return to the portfolio. | BD-SS-04 |
| SS-26 | Tiles MUST carry counts, statuses and times only, with "last updated", ordered by the stated rule. | UT-SS-08, UT-SS-20 |
| SS-27 | Fifty sites MUST load on the portfolio within the stated target, and the per-site resident cost MUST be measured. | MS-SS-02 |
| SS-28 | Disconnected or paused sites MUST mark admitted cached figures stale and show their status. | UT-SS-21, IT-SS-06 |
| SS-29 | Each source MUST provide one local bounded summary read in a single site scope and stale source versions MUST be fenced. | IT-SS-06, MS-SS-01, MS-SS-02 |
| SS-30 | Registry changes and restores MUST follow spec 0030 before sites become visible or can be used. | IT-SS-07 |

## How it is tested

Designed, not run.

- **IT-SS-06:** compare each source summary with its owning screen or
  eligible records. Cover the empty, missing, stale and refused cases
  and test the exact populations of all seven sources. Race a refresh
  against property replacement, approval withdrawal, removal and
  erasure. Reload or export figures only while their fences hold.
  Keep a running jobs summary current and advance the service clock
  from just under the configured duration to equality. Renew the
  lease without committing progress. The tile enters the stalled
  attention tier without a new progress event or site fetch. Commit
  newer progress within the duration and clear that flag. A complete
  summary with only queued, waiting, paused or finished jobs also
  has no stall. Withhold the flag on a stale or unavailable jobs
  summary and do not show “no attention items”. Repeat with a
  progress timestamp later than the service clock. For a paused site,
  mark the stall check not applicable. Other tiles remain readable.
  Check fixed result size and a bounded query plan on long histories.
- **UT-SS-16:** read the verified native register as the owner before
  site access. Supply a valid installation-only DB context and try a
  portfolio query, site-row read and site removal. No site rows or
  effects follow. Check that the admitted setup or migration checkpoint
  still works through its own role under spec 0108 IT-BD-06. Give an
  exact site context and check that its approved site read still works.
- **IT-SS-07:** interrupt registration, removal and replacement at
  each ledger and register step. Resume twice and compare visible
  sites on both supported systems. Start backup during pending
  setup and removal. Refuse a damaged register and a mismatched
  manifest. Restore or move with newer or missing retirement history.
  Check for no site resurrection. Missing keys leave a site
  disconnected.

- **Unit (UT-SS):** site limit; one-site-context rule; the attention order on
  a table of tile states; "not connected" rendering; group matching with one
  pair just inside and one just outside the similarity limit; settings bounds.
- **Planted faults (PL-SS):** a query that tries to read two sites; an MCP
  call for the portfolio; a group check that copies page text; each must be
  refused with its named message while an honest case passes.
- **Integration (IT-SS):** three sites with a crawl on each and turn-taking;
  removal of one site with the other two untouched; a group check with a site
  leaving the group.
- **Fault injection (FI-SS):** one site's database rows unreadable while the
  portfolio loads.
- **Behaviour (BD-SS):** the Omar scenario above, step by step.
- **Audit (AU-SS):** that nothing exists for a deferred feature.
- **Measurement (MS-SS-01, MS-SS-02):** portfolio load time with a warm cache
  for ten and for fifty fixture sites on the 8 GiB reference (proposed: under
  two and under six seconds), and the per-site resident cost of fifty
  registered sites. The fixtures are generated sites with counts only.

## Changes from the source

The old list had 26 features in "International and multi-site" and
"Localisation". By the owner's decision of 20 November 2026 version 1 supports
several sites for one owner (portfolio view and an opt-in cross-site duplicate
check) and defers the 24 language and market features until after version 1
(DR-081, DR-082). The shared parts of 558 (spec 0041, which gives the advice
wording) and 560 (spec 0047) follow this decision.

## Named site-localisation refusal cases

The named Wave 3 cases use the existing interface, applicability,
authority and recovery rules. Each case has the exact input and
observable outcome in its Wave 3 row.

| Feature | Named synthetic case |
|---|---|
| F-1208 | FX-0102-W3-CULTURAL-ADAPTATION-REFUSAL |
| F-1209 | FX-0102-W3-LOCAL-FORMATS-REFUSAL |
| F-1210 | FX-0102-W3-REGULATION-NOTES-REFUSAL |
| F-1211 | FX-0102-W3-LOCAL-EXAMPLES-REFUSAL |
| F-1212 | FX-0102-W3-RIGHT-TO-LEFT-SITE-CHECK-REFUSAL |
| F-1213 | FX-0102-W3-LOCALISED-IMAGE-REFUSAL |
| F-1214 | FX-0102-W3-LOCAL-SEARCH-TERMS-REFUSAL |
| F-1215 | FX-0102-W3-HUMAN-EDITING-LEVEL-REFUSAL |
| F-1216 | FX-0102-W3-STYLE-GUIDE-PER-LANGUAGE-REFUSAL |
| F-1217 | FX-0102-W3-LOCALISED-CALL-TO-ACTION-REFUSAL |
| F-1218 | FX-0102-W3-LOCAL-HOLIDAYS-REFUSAL |
| F-1219 | FX-0102-W3-NAME-TRANSLITERATION-REFUSAL |
| F-1220 | FX-0102-W3-LANGUAGE-COVERAGE-REFUSAL |
| F-1221 | FX-0102-W3-LOCALE-DISCLAIMER-REFUSAL |
| F-1222 | FX-0102-W3-NATIVE-SPEAKER-REVIEWER-REFUSAL |
