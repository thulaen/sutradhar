# 0044: SEO signals, crawler and health reports

This part checks each registered site's health for search engines. It
visits the site's own pages as a guest would (the crawler), reads the
site's database, the copied Search Console, Bing and GA4 figures
(spec 0040) and, if allowed, the web server's access logs. It turns these
into a health report per site: what is wrong, why it matters, how many
pages it affects, and a proposed fix that a person carries out. It never
changes titles, redirects, canonical addresses or indexing instructions
by itself. It is roadmap Phase 6 item 2. The module is `sanket`.

Example: "212 thread pages 2+ point their canonical address to page 1
(signal SIG-017). Search engines may ignore the later pages' replies.
Proposed fix: in XenForo's options, let each page name itself as
canonical." The administrator decides.

## What it does

### Terms

- Signal: one check from the owner's list of health items, with a result
  per page or per site. A starred signal is one the owner marked; starred
  signals are built first.
- Crawler: the program that fetches the site's own pages as a guest,
  politely. Frontier: the list of addresses still to fetch.
- Click depth: how many clicks from the home page a page needs.
- Soft 404: an error or no-permission page that answers "200 OK".
- Canonical address: the address a page says is its main one.
- Proposal: a suggested fix, written for a person to carry out.

### Scope

1. The crawler reads only sites registered in Sutradhar: XenForo and
   WordPress. Each site is crawled on its own and its data stays apart.
2. Each site has a fixed list of allowed hosts (which may include the
   owner's registered language hosts, spec 0047). The crawler never
   fetches a host outside it, even after a redirect. A post linking to a
   partner's shop is recorded; the shop is never opened.
3. Out of scope: other people's sites, search-engine result pages,
   logged-in pages, rivals, and any second or distributed crawler. The
   only exceptions are two narrow lists, the backlink lead list and a
   fixed list of official status pages (spec 0045). They keep link
   records and a change note only.
4. Reading Search Console and Bing stays read-only. Copying search
   figures is spec 0040, visit statistics spec 0034, and the link graph
   maths is ganit's graph engine.

### The signal registry

1. `signals.toml` in `sanket` holds every item of the owner's list (235
   items, SIG-001 to SIG-235, in the owner's order, in 15 sections) as an
   entry: id, section, the owner's exact words, starred or not, sources
   (crawler, database, Search Console, Bing, GA4, first-party analytics,
   logs, outside service), check kind (per page, per site, trend), state
   (built, waiting for a source, removed by a decision), the acceptance
   test and a `recheck` entry (spec 0048).
2. A gate check fails if an entry disappears or lacks a `recheck` entry,
   so no signal is lost.
3. A signal whose source is removed or not approved stays in the
   registry with its reason and shows on the report as "not measured".
   The rule is "run what has evidence":
   - Section 13, backlinks: "not measured" only while no service is
     connected and no backlink run exists. Otherwise the figures come
     from the web link study (spec 0046), labelled by source and never
     added together.
   - Safe Browsing (SIG-193, SIG-229) and CrUX or PageSpeed (section
     14): each is on once the site's own Google key is set; with no key
     it shows "not measured".
   - Section 14's real-user Core Web Vitals come from darshak's page
     script (spec 0034), as counts only.

### Modules and design

1. `sanket` modules: `registry`, `crawl` (frontier, fetcher,
   politeness), `robots`, `sitemap` (quick-xml), `html` (lol_html),
   `schema` (JSON-LD with serde_json), `links` (internal and outside,
   with url and psl), `logs`, `checks` (one function per signal family),
   `report`, `propose`.
2. Traits with test fakes: `Fetcher` (reqwest; fake `FixtureSite`
   serving a made-up site), `RobotsRules` (fake `AllowAll`),
   `ForumReader` (read-only sqlx through sampark; fake `FixtureForum`),
   `SearchNumbers` (spec 0040's tables), `LogSource` (fake lines) and
   `Resolver` for outside hosts.
3. Each check is a pure function from collected facts to results, so
   checks are tested without a network. A new signal adds one function.
4. One address normaliser, shared with reading and indexing (spec 0022),
   uses thread ids, not slugs. One fetcher serves the crawl and the
   outside-link check. One report model serves every signal.
5. The crawler fetches HTML and headers only and runs none of the page's
   scripts. "Content present in raw HTML" is checked against the stored
   post text.

### The crawler

1. It starts from the home page, the sitemaps and the database's list of
   guest-visible threads, forums and pages. It fetches as a guest with no
   cookies.
2. Identity: it always sends its own declared name,
   `SutradharHealthCheck/<version>` plus the site's address, in every
   request, including the browser check. It never uses another crawler's
   name. In place of a two-name cloaking check, it compares what a guest
   sees with what the declared crawler sees; the report says this cannot
   prove how a page treats a search engine.
3. It obeys robots.txt for its own name (robotstxt, pinned and fuzz
   tested) and records which addresses other crawlers are blocked from,
   without fetching them as those crawlers. A robots file over 500 KiB is
   reported and only its first 500 KiB is used. A robots file answering
   401 or 403 is treated as deny.
4. Rate: `seo.crawl.rate_per_second`, default 2, allowed 1 to 8, with an
   aggressive preset of 4. One request at a time. Above 2 a second the
   app asks for confirmation and shows the site's measured response time.
5. One rule for slowing down: the crawler measures the site's normal
   median response time with no crawl running. It slows when the median
   rises 10% above normal and pauses at 25% above. The load governor can
   also pause it.
6. An hourly limit on requests and bytes per site; going over it is
   SUT-654.
7. It honours `Retry-After`, status 429 and `Crawl-delay`. It stops on
   repeated failure and retries safely later.
8. Schedule: changed pages daily (from the database's edit and reply
   times), the whole site weekly, robots.txt and sitemaps daily. The
   weekly full pass uses `ETag` and `If-Modified-Since`.
9. The frontier and results live in PostgreSQL with checkpoints, so a
   crash or restart resumes with no page fetched twice in that pass.
   Endless spaces (search results, member lists, sort parameters) are
   recognised by pattern and sampled, not followed for ever.
10. The crawler starts on, with balanced values. Features needing an
    owner key or a log file show "not measured" until supplied.

### Control screen

1. Start, pause and resume, rate, schedule, history and progress, through
   the one progress system (spec 0016).
2. List mode crawls a given list of the site's own addresses.
3. Compare two crawls: pages added, removed and changed, from stored
   facts. Example: "37 pages changed title since last week".
4. Check my fix: after a person applies a fix, re-read only the pages
   named in one proposal (or one Fix-list item, spec 0048), within the
   same rate limit, and show how many are now right. A routine crawl
   also feeds the item's return rule. It never changes anything and
   never closes work by itself.

### Outside links

1. Only addresses that appear in guest-visible posts are checked, each at
   most once a week, with `HEAD` then a ranged `GET`. Nothing further is
   followed.
2. Protection against server-side request forgery (starred): an address
   that resolves to a private, loopback, link-local or the host's own
   range is refused, before connecting and after every redirect (at most
   5). The resolved address is pinned for the connection, so a DNS
   rebinding cannot switch it.
3. Per-domain rate: one request every 10 seconds.

### Page copies

1. Copies of fetched guest pages are kept for
   `seo.crawl.page_copy_days` (default 30, allowed 1 to 365), within
   `seo.crawl.page_copy_max_bytes` per site (above zero; the value is set
   by research before the build).
2. Only guest-visible pages from the site's own allowed hosts are kept,
   compressed and held per site, oldest removed first.
3. A copy is removed on expiry, when the page leaves guest view at the
   next crawl (and then also from the test room's frozen sets, spec
   0039), and when the site is removed. Erasure requests cover copies.
4. A copy is never public, never sent to a cloud helper, and never used
   to decide what guests may see. The test room may read one and makes
   its own sealed copy.

### Small browser check

1. Most pages stay raw HTML only. A headless browser opens at most
   `seo.crawl.browser_pages_per_day` pages per site (above zero; set by
   research before the build), off-peak, to find script-built content.
   Example: a menu built by a script shows no links in raw HTML; the
   check records the difference.
2. It only reads: no forms, no sign-in, guest view, allowed hosts only,
   the same declared name and the same slowing rule.
3. It runs in its own sandbox built from Linux's own controls (spec
   0037), with no container program, and its own memory limit, counted
   in the heavy job slot of spec 0024 within the memory limits in
   AGENTS.md section 5.6; it stops first under any pressure.
4. Pages it has not checked show "not measured", never a guess.
5. The browser must first be pinned and admitted as AGENTS.md section 4
   sets out. Until then this check is unavailable and labelled so.

### Server logs

1. Optional. The administrator mounts the web server's access log
   read-only into the service, where darshak's log task holds the day's
   salt, and names its format (common, combined, or a pattern).
2. New lines are read every 5 minutes and follow spec 0034 exactly. A
   line that joins a beacon visit (same daily hash and page within 2
   minutes) adds only a "seen in logs" mark. A line that joins none is
   its own visit, labelled in the same fixed order with the rules that
   apply to log lines (1 to 6, 8 and 10). A verified Googlebot is
   `verified_search_crawler`, a failed claim FAKE_CRAWLER, a claim with
   no check UNVERIFIABLE, and an unverified line with no beacon
   NO_SCRIPT. Crawler claims are checked by published address lists and
   reverse DNS with forward confirmation.
3. AI crawlers get named labels in the log report, for reading only and
   never as ranking inputs. Nothing edits the robots file.
4. Addresses stay in memory only. Per-day counts by label, reason and
   page type are kept. The logs also feed darshak's page-request input
   for job 323's inflation check.
5. The health crawler's own visits come from the site's own server
   address with its own name. darshak labels them `declared_bot`, reason
   OWN. They never count as people and never feed ranking.

### Reports and proposals

1. One health report per site, grouped in the 15 sections. Each signal
   shows its result, the pages affected (by thread id, linking to the
   page), the trend over 90 days, the source and its coverage, and a
   proposal for a person: what to change, where in XenForo, WordPress or
   the server, and what to expect.
2. Never: changing a title; applying or one-click applying a canonical,
   redirect, robots or indexing change. Proposals carry no "apply"
   button. A duplicate-title finding gives a report and no title-change
   proposal.
3. Reports gain a "Layout groups" view with one alert per group, shop
   reports, two weak-page lists and a falling-page list (spec 0047). For
   falling pages, "Report" is the only action.
4. Grouped alerts, with plain-English faults and codes and scrubbed
   diagnostics, for sudden changes: robots.txt unreachable or blocking
   everything, sitemap errors, a spike in 5xx answers to crawlers, and an
   HTTPS certificate expiring within 14 days. ganit's changepoint and
   anomaly jobs flag crawl spikes and drops.
5. A block page on a registered site also becomes a Fix-list item
   (spec 0048).

### Signals given to linking

1. The `SeoSignals` trait gives linking: orphan threads, click depth,
   inbound and outbound links, cannibalised queries, striking-distance
   queries (positions 8 to 20), content gaps, pages with no impressions
   in 90 days, and index status.
2. Inbound links, click depth, impressions at positions 8 to 20 and
   content gaps are the inputs of ganit job 324's need score (spec 0034).
3. Canonical address, noindex, robots.txt blocking and status decide
   which targets the selector may use (the eligible-target rule, spec
   0027).
4. These signals only reorder candidates that passed the relevance gate.
   None is a popularity count.

### ganit jobs used

The service exports the link graph (thread ids and edges), crawl counts,
log counts and Search Console figures keyed by page and query ids.
Search terms never leave the `sanket` module: it marks each query id
with flags such as "contains a brand term" or "starts with a question
word", and ganit sees only those flags. Results show after their shadow
test, and the last good version is kept when a job is late.

- Graph: job 251 internal PageRank; job 253 orphan threads and click
  depth; job 265 pages competing for the same queries
  (cannibalisation); job 258 pairs of threads that only link to each
  other.
- Learning: job 79 query intent; job 80 branded versus non-branded
  share, from the flags; job 81 the crawl-budget model behind crawl-waste
  advice; job 83 thin-content value behind noindex proposals (report
  only).
- Changepoints: jobs 160 to 162, in Googlebot crawl rate, organic clicks
  and many rankings at once. Anomalies: job 179 crawling of
  non-indexable addresses, job 180 server errors to bots. Forecast: job
  205 Googlebot requests outlook.
- Causal effects: jobs 302 to 304, the effect of IndexNow notices, of
  meta-description length and of structured data on click-through, each
  shown as advice with its interval. Survival: job 311, time until Google
  first crawls a new thread.
- Gaps: job 289 content gaps from on-site search and Search Console
  query ids.

### Telling search engines about new pages

1. One plain list of public addresses of the site's own pages, sent as
   IndexNow notices, for the owner's own registered sites only. A
   private, staff-only or logged-in address is never listed.
2. A daily cap, with a safe top, limits how many go out (its value is set
   by research). At the cap the rest wait for the next day and the screen
   says so.
3. A notice only says a page exists. It asks for no indexing change and
   edits no page, robots file or sitemap. It is separate from the Search
   Console and Bing readers. This part measures the notices' effect.
4. The owner may upload Google's "crawled, not indexed" list (small
   version).

### Data

- `seo_crawl_page`: site, address hash, normalised address, thread or
  node id, status, redirect target, fetch time, response time, content
  hash, canonical, robots directives, title hash and length, description
  hash and length, headings summary, structured data summary, link
  counts, structure marks (spec 0047).
- `seo_frontier`: address, priority, reason, next due, attempts.
- `seo_link`: source page, target (internal id or outside address hash),
  kind (post, navigation, listing), rel attributes.
- `seo_external`: address hash, last status, redirect chain length,
  checked at.
- `seo_log_day`: day, crawler, page type, requests, 4xx, 5xx, 429,
  median response time.
- `seo_result`: site, signal id, scope (page or site), page id, value,
  severity, first seen, last seen.
- `seo_structure`, the group tables, alternates, markets and business
  facts belong to spec 0047.
- Titles and descriptions are stored as lengths and hashes for duplicate
  checks, not as text, except for the 500 shown on the report at a time.
- Every table has forced row-level security, per site.

### API, events and commands

- Management API: `GET /api/seo/report`, `GET /api/seo/signals/{id}`,
  `GET /api/seo/pages/{id}`, `POST /api/seo/crawl` (start a crawl now, at
  most one a day), `GET /api/seo/registry`.
- Events: `seo.crawl_started`, `seo.crawl_completed`,
  `seo.signal_changed`, `seo.alert_raised`.
- Commands: `sutra sanket crawl|report|signal <id>|registry`.

### Failures

- SUT-650: robots.txt unreachable. The crawl uses the last good copy for
  a day, then stops and alerts.
- SUT-651: the site answers too slowly. The crawl pauses.
- SUT-652: an outside address refused as private. Recorded, never
  fetched.
- SUT-653: log format not understood. Log reading stops, with an alert.
- SUT-654: the crawl is over its hourly limit. It resumes from its
  checkpoint.
- SUT-655: a source is missing. The signal shows "not measured" with the
  reason.

### Security and privacy

1. Guest view only: no cookies and no staff session, so nothing private
   is fetched.
2. Responses are capped at 5 MiB, and decompressed size is capped too.
   HTML parsing is bounded in time and depth. Redirects are re-checked.
3. No visitor addresses are stored from logs. Member names on profile
   pages are not stored, only page facts.
4. Deleting a thread removes its rows at once.

### Edge cases to cover

Friendly addresses off (`index.php?threads/...`); the board in a
subfolder; old domains and CDN hosts; percent-encoded slugs; `/goto/post`
links; maintenance mode answering 503; a robots.txt that changes during a
crawl; sitemap indexes of many files; gzip sitemaps; hreflang only on
multilingual sites; very large threads with hundreds of pages.

### Facts to check when built

Google's and Microsoft's published crawler address lists; whether the
Search Console API has any links data; Safe Browsing's terms for
commercial use (the Web Risk API may be the commercial route); CrUX and
PageSpeed quotas; robots.txt's 500 KiB limit in Google's current
documentation.

## Limits

- Crawling runs one fetch at a time with a streaming parse, at most
  96 MiB above idle. The frontier stays in PostgreSQL, never in memory.
  Report building uses at most 64 MiB. All within the service slice.
- The browser check runs in its own Linux sandbox with its own memory
  cap, counted in the heavy job slot, and is stopped first under
  pressure.
- Rate 1 to 8 requests a second (default 2), one at a time; hourly
  request and byte limits per site; outside links one request per domain
  every 10 seconds, each link at most weekly; at most 5 redirects.
- Responses at most 5 MiB, decompressed size capped; robots.txt read to
  500 KiB.
- A manual crawl at most once a day. IndexNow notices under a daily cap.
- Speed needs: the site's median response time during a crawl stays
  within 10% of normal; a weekly full crawl of 1 million pages finishes
  within 7 days at 2 a second (about 5.8 days of fetching); the report
  opens within 1 second at the 95th percentile for 1 million pages; one
  page's HTML checks run within 2 ms.

## Footprint budget

The crawler adds guest page requests at the set rate and slows at 10%
above the site's normal median response time, pausing at 25%. It adds
nothing to the pages themselves. Any per-page budget is set when the
feature is built.

## How it is tested

Rust tests in `sanket`, with a made-up site served by wiremock:

- Registry (gate): `signals.toml` holds all 235 items with the owner's
  words; removing one, or a `recheck` entry, fails the check.
- Crawl: every guest-visible thread is fetched and a seeded private node
  is never fetched or reported.
- Redirects: a three-hop chain and a loop are both reported.
- Soft 404: a no-permission page answering 200 is reported.
- Canonical: page 2 pointing to page 1 gives SIG-017 with its proposal.
- Robots: a file over 500 KiB is reported and only 500 KiB is used.
- Noindex: a noindexed page that robots.txt blocks gives the "never
  seen" case.
- Sitemap: a listed redirected address and a noindexed one are reported.
- Schema: DiscussionForumPosting missing its author gives a validation
  error.
- Graph contract: a fixture link graph exported in Arrow and a golden
  ganit job 253 result give click depth and orphans that match hand
  counts; the export holds no address or text.
- SSRF: post links to `127.0.0.1`, an address in `10.0.0.0/8`, a host
  resolving to a private address and a redirect to one: none is fetched
  and each gets SUT-652.
- Outside links: only referenced links are fetched.
- Politeness: at 10% above normal the crawler slows; at 25% it pauses,
  and it resumes on recovery.
- Resume (failure injection): the worker killed mid-crawl resumes from
  its checkpoint with no page fetched twice.
- Logs: a combined-format "Googlebot" line whose reverse name is not
  Google's is `suspected_automation`, FAKE_CRAWLER, through darshak, and
  no visitor address is stored.
- Own crawler: every health-crawl visit is `declared_bot`, OWN, never a
  person, never a ranking input.
- Eligibility: non-canonical, noindexed or robots-blocked pages are left
  out of the list given to the selector.
- Search Console: cannibalised and striking-distance queries match the
  fixture.
- Screens: a Dioxus app test walks every report screen and finds no
  apply or one-click fix.
- Titles: duplicate titles give a report and no title-change proposal.
- Not measured: with no backlink source and no run, section 13 shows
  "not measured" with its reason.
- Query privacy: the ganit export holds query ids and flags only.
- Hosts: a redirect to a host outside the list is refused and recorded.
- Sites: a WordPress and a XenForo site crawl apart with no mixed data.
- Rate: a value above 8 or below 1 is refused, with the reason shown.
- `Retry-After` and 429 are honoured against a test server.
- Page copies: a copy expires on the set day and on site removal; a copy
  of a page no longer guest-visible is removed at the next crawl.
- Compare: two crawls give the right added, removed and changed counts.
- Check my fix: reads only the named pages and changes nothing.
- Browser check: stays within its daily and memory limits, stops when
  the site slows, uses the same name, and says "not measured" when the
  browser is not admitted.
- Benchmark: one page's HTML checks within 2 ms, under the 5% median
  rule.
- Proofs and depth: 100% line and branch coverage. The address
  classifier, the redirect re-check, the guest-only fetch, the
  "no apply path" rule, the slowing rule, retention and the registry
  check have no surviving mutants and have property tests. Kani proves
  the classifier refuses every private, loopback and link-local range of
  IPv4 and IPv6. robotstxt parsing is fuzz tested.

## Changes from the source

- The crawler's decision logic (which address may be fetched, the slowing
  rule, retention and checks) is rewritten as Rust, proved with Kani.
- The maths jobs run in ganit, the Rust numerical engine.
- Report screens are a Dioxus app built from SDS components.
- The 20x speed goal is replaced by the 5% median rule.
- "Forum" now reads "registered site" throughout.
- The browser's admission follows AGENTS.md section 4 pinning.

Source: `docs/specifications/signals/seo.md`,
`docs/specifications/signals/seo-2.md` and
`docs/specifications/signals/seo-crawler-amendment.md` in the archive
`archive-2026-11-08-old-repo`.
