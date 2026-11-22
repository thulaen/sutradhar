# 0044: SEO signals, crawler and health reports

This part checks each registered site's health for search engines. It
visits the site's own pages as a guest would (the crawler), reads the
site's database, the copied Search Console, Bing and GA4 figures (spec
0040) and, if allowed, the web server's access logs. It turns these into
a health report per site: what is wrong, why it matters, how many pages
it affects, and a proposed fix that a person carries out. It never
changes titles, redirects, canonical addresses or indexing instructions
by itself. It is part of roadmap Phase 6; "Which spec designs each
feature" in [features.md](features.md) lists the features it designs in
whole or in part. The module is `sanket`.

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
2. Each site has a fixed list of allowed hosts. A language link never
   extends that list or the site's scope (spec 0047). The crawler never
   fetches outside it, even after a redirect. A post linking to a partner's
   shop is recorded; the shop is never opened.
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
2. Identity, robots.txt, rate, the slowing rule, waits and the hourly
   limit are owned by spec 0082 (Leviathan). The own-site crawl follows
   them and adds nothing of its own.
3. Schedule: changed pages daily (from the database's edit and reply
   times), the whole site weekly, robots.txt and sitemaps daily. The
   weekly full pass uses `ETag` and `If-Modified-Since`.
4. The frontier and results live in PostgreSQL with checkpoints, so a
   crash or restart resumes with no page fetched twice in that pass.
   Endless spaces (search results, member lists, sort parameters) are
   recognised by pattern and sampled, not followed for ever.
5. The crawler starts on, with balanced values. Features needing an
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

### Wave 2 crawler intake, scheduling and list mode

These synthetic control fixtures use the existing own-site guest
crawler, eligible address policy and durable queue. A decision by a
staff member or local command gives no signed-in site access. Use
current grants, the pinned Leviathan identity and all existing site,
address, robots, stop and budget checks. Use a stand-in fetcher.

| Feature | Owning requirement and design | Planned fixture and expected evidence |
|---|---|---|
| F-0113 | REQ-0044-W2-GUEST-CRAWL-INTAKE · DES-0044-W2-EXISTING-CRAWL-CONTROLS | FX-0044-W2-GUEST-CRAWL-INTAKE · IT-0044-W2-GUEST-CRAWL-INTAKE. Use one registered fixture home, eligible sitemap and public page seed and an allowed guest response. Add unread and denied robots decisions, a private seed, an off-list or refused host and a spent budget. Allow only eligible guest requests under the existing identity and scope. Require zero requests for the held cases. Record an eligible refusal reason and incomplete coverage. Reuse the existing crawl and F-1318 guest fence, and 0082 LV-R01–06/09/10. |
| F-0114 | REQ-0044-W2-OWN-CRAWL-SCHEDULE · DES-0044-W2-FENCED-CRAWL-TRIGGERS | FX-0044-W2-OWN-CRAWL-SCHEDULE · IT-0044-W2-OWN-CRAWL-SCHEDULE. Use the existing daily and weekly schedules and stand-in admitted 0032 own-site theme, plugin and platform events, each with its own equivalent replay. Require one trigger identity and logical outcome per qualified request within existing rate and work limits. A wrong-site, unadmitted or revoked event starts none. On restart recheck the current authority and durable ledger head under [0116](0116-leviathan-scale-storage-and-verification.md#journal-and-recovery) LS-09 and LS-31. Recover an exact completed result once. Continue eligible scheduled work after durable proof the original request never reached transport. Hold the uncertain effect, original reservation and coverage gap with no second send. Do not create a new event source, schedule or rival read. Reuse FI-LS-02 and IT-LS-02 for durable history. |
| F-0116 | REQ-0044-W2-EXPLICIT-CRAWL-LIST · DES-0044-W2-EXISTING-CRAWL-CONTROLS | FX-0044-W2-EXPLICIT-CRAWL-LIST · IT-0044-W2-EXPLICIT-CRAWL-LIST. Paste or import a synthetic own-site list with a duplicate exact request identity, a refused entry and an independently eligible hop from a listed source. Add an off-list discovered href and deny another redirect hop. Deduplicate by the existing admission rules and retain safe rejection and coverage states. Read only eligible listed entries and separately permitted hops within the same existing crawl limits. Never seed from links found on the page. Reuse 0082 LV-R01 and LV-R05. |

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
   `seo.crawl.page_copy_max_bytes` per site. This byte cap is a readiness
   blocker owned by the crawler storage record; page copies cannot build or
   run until its value and eviction fixtures are admitted.
2. Only guest-visible pages from the site's own allowed hosts are kept,
   compressed and held per site, oldest removed first.
3. A copy is removed on expiry, when the page leaves guest view at the
   next crawl (and then also from the test room's frozen sets, spec
   0039), and when the site is removed. Erasure requests cover copies.
4. A copy is never public, never sent to a cloud helper, and never used
   to decide what guests may see. The test room may read one and makes
   its own sealed copy.

### Cookie observations

Read `Set-Cookie` only from the final own-origin main guest response
already admitted for an own-site Audit. Inspect fields separately.
Until QR-089 admits bounded header bytes, field count and parser time,
show "not available" and collect no observation. Collect only safe
names and the allow-listed attributes
`Secure`, `HttpOnly`, `SameSite`, `Domain`, `Path`, `Max-Age`,
`Expires` and `Partitioned`. Store flags, recognised choices and
validated scope or lifetime. Scrub names and scope against the privacy
gate. If unsafe, omit the detail and mark coverage partial. Unrecognised
attribute values and malformed fields are discarded and mark coverage
partial. The crawler strips the raw fields at ingress (spec 0082).
Cookie values never reach storage, a cookie jar or another request.

Record the site, page reference, origin, audit, time, parser version
and coverage reason. Keep the facts with the source audit report and erase them
when it expires, the page is erased or the site is removed. Separate
no observation, a complete sample with no cookie fields and partial
coverage, even when partial coverage yields no names. A cached or 304
response gives no new negative evidence and cannot make old facts
current. Redirects, subresources and script actions give no evidence.
Add no fetch or interaction. Spec 0101 uses the scoped sample.
[HTTP cookie fields](https://datatracker.ietf.org/doc/html/rfc6265)

### Small render check

1. Raw HTML is the fallback. Use only the engine and render level
   admitted by EX-02 under spec 0115, within the cap in
   `seo.crawl.browser_pages_per_day`. The cap and per-platform cage
   proof are required before render work. Until the settings registry
   admits a positive cap for this site, render is unavailable. Use the
   existing site slowing rule and off-peak window.
2. Use the spec 0115 cage and broker with spec 0082's legal wall,
   allowed identities and guest access. Keep its admitted memory and
   job limits. Without the required engine, cage, cap or evidence
   show "not measured" or "not available" with the reason.

### Wave 2 link facts, timings and metadata

Use synthetic records from eligible own-site guest captures. Bind
the registered site, admitted source, page or resource revision and
coverage. Use 0115 only with an independently admitted render profile.
Missing evidence stays unknown. These Audit reports start no extra
fetch, rank input, title, description, robots or other site change.

| Feature | Owning requirement and design | Planned fixture and expected evidence |
|---|---|---|
| F-0141 | REQ-0044-W2-URL-LINK-REPORT · DES-0044-W2-LINK-OCCURRENCES | FX-0044-W2-URL-LINK-REPORT · IT-0044-W2-URL-LINK-REPORT. Use exact 0022 identities and one site capture with inbound and outbound links in body and navigation, repeated occurrences at distinct positions and one unread or hidden referrer. Show safe anchors, positions, capture dates and direction. Preserve distinct occurrences without duplicating one replay. Missing edges remain unknown. Reuse the 0116 LS-14 bounded graph rule. F-0140 navigation-only judgement remains with 0047. |
| F-0142 | REQ-0044-W2-URL-RESPONSE-TIME · DES-0044-W2-SCOPED-CRAWL-METRICS | FX-0044-W2-URL-RESPONSE-TIME · IT-0044-W2-URL-RESPONSE-TIME. Compare two dated section observations using the same admitted timing method. Give a timeout, a denied response and a cached observation. Show measured crawl durations with their basis, coverage and source. Keep unlike or unavailable timings separate. Do not merge with visitor timing, infer a duration or start a timing probe. |
| F-0143 | REQ-0044-W2-URL-PAGE-WEIGHT · DES-0044-W2-SCOPED-CRAWL-METRICS | FX-0044-W2-URL-PAGE-WEIGHT · IT-0044-W2-URL-PAGE-WEIGHT. Use stored HTML, script, stylesheet and image byte facts from one admitted complete 0115 capture. Show per-type values, transfer or decoded basis and cache/resource coverage. Deduplicate replayed receipts and count a shared resource at its exact recorded attribution without adding it twice. Preserve actual distinct transfers. A refused download or raw HTML-only capture shows missing coverage, never a zero. No new resource read. |
| F-0144 | REQ-0044-W2-TITLE-PRESENCE · DES-0044-W2-METADATA-STAGES | FX-0044-W2-TITLE-PRESENCE · IT-0044-W2-TITLE-PRESENCE. Compare complete head observations for zero, one and multiple titles. Include a script replacement at an admitted completed stage and a partial read. Show the relevant raw or rendered stage for each fact. No absence fault from a partial head. Reuse F-1320 and 0115 RN-18 for capture boundaries. No title proposal or edit. |
| F-0145 | REQ-0044-W2-BOILERPLATE-TITLE · DES-0044-W2-TITLE-DIAGNOSTICS | FX-0044-W2-BOILERPLATE-TITLE · IT-0044-W2-BOILERPLATE-TITLE. Use a complete observed title matching the admitted site-name-only rule, an admitted untitled placeholder and a specific title containing the site name. Report only supported whole-title matches with 0047 layout and coverage. An absent profile or title stays unknown. No inferred template intent or title replacement. |
| F-0146 | REQ-0044-W2-LOCAL-TITLE-WIDTH · DES-0044-W2-TITLE-DIAGNOSTICS | FX-0044-W2-LOCAL-TITLE-WIDTH · IT-0044-W2-LOCAL-TITLE-WIDTH. Use a local title and pinned admitted glyph metrics and viewport profile. Compare widths at the preview boundary and missing font or viewport evidence. Show a local estimate within that profile or unavailable. No provider call, claim about an actual search result or title rewrite. |
| F-0147 | REQ-0044-W2-REPETITION-EVIDENCE · DES-0044-W2-TITLE-DIAGNOSTICS | FX-0044-W2-REPETITION-EVIDENCE · IT-0044-W2-REPETITION-EVIDENCE. Use an admitted rule and exact 0026 passage facts for repeated wording in a title and description, a legitimate repeated product term and incomplete evidence. Show only supported staff warnings with ranges and rule version. An absent profile or ambiguous use holds. No intent claim, ranking effect, policy change or rewrite. |
| F-0148 | REQ-0044-W2-DESCRIPTION-FINDINGS · DES-0044-W2-METADATA-STAGES | FX-0044-W2-DESCRIPTION-FINDINGS · IT-0044-W2-DESCRIPTION-FINDINGS. Compare complete captures with zero descriptions, more than one on a page and a duplicate across eligible pages. Exercise a separate admitted width preview and a partial head. Keep observed missing, multiple and duplicate results separate from the preview estimate. A partial record proves no absence. Preserve date and source stage. |
| F-0149 | REQ-0044-W2-ROBOTS-DIRECTIVE-CONFLICT · DES-0044-W2-METADATA-STAGES | FX-0044-W2-ROBOTS-DIRECTIVE-CONFLICT · IT-0044-W2-ROBOTS-DIRECTIVE-CONFLICT. Compare stored header and meta directives, agreeing and conflicting observations and an unread or incomplete response. Pin crawler identity, applicable group and rule version. Show exact eligible directives and a resolved result only under the admitted precedence. Reuse 0082 LV-R01/02 and UT-LV-01/02. Robots access stays separate and a denial remains a denial. No provider-indexing claim or policy change. |

### Server logs

1. Optional, off by default. In the app the owner chooses a readable
   access-log source for this site and confirms read-only use, format,
   purpose, fields, retention and the current counting and consent mode.
   Show which fields are discarded and the stop control. Retention
   cannot exceed spec 0034's limit for each data class. Only formats
   and request evidence admitted under QR-088 can be selected. Bind the
   confirmation to the site and a version of every choice shown. A
   change, stop or site removal invalidates that version. The owner
   must confirm again before any further read. An unavailable approval
   has the same effect as a stop. Test this contract with AT-SEO-LOG.
2. Darshak reads new lines every 5 minutes. Check the active approval
   before opening the source and atomically when committing records or
   the cursor. Stop closes the source and discards uncommitted work.
   Spec 0034 decides which lines may reach joins, counts and reports.
   Only darshak's local log task holds the day's salt. An admitted
   unmatched line uses rules 1 to 6, 8 and 10. A verified Googlebot is
   `verified_search_crawler`, a failed claim FAKE_CRAWLER, a claim with
   no check UNVERIFIABLE, and an unverified line with no beacon
   NO_SCRIPT. Crawler claims are checked by published address lists and
   reverse DNS with forward confirmation.
3. AI crawlers get named labels in the log report, for reading only and
   never as ranking inputs. Nothing edits the robots file.
4. Addresses stay in memory only. Per-day counts by label, reason and
   page type are kept. Site-scoped bot counts by page are available to
   spec 0116 through darshak, within the approved fields and retention.
   The admitted logs feed darshak's page-request input for job 323's
   inflation check.
5. The health crawler's own visits come from the site's own server
   address with its own name. darshak labels them `declared_bot`, reason
   OWN. They never count as people and never feed ranking.

### First verified Googlebot request (201)

Use 0091 feature 1268's admitted original native publication proof
for the exact own-site page. Keep the source time and precision.
A recrawl, later edit, republish of the same page or first log
appearance does not reset that instant. An unproved old URL or
different page at the same address supplies no history.

Under QR-088 and QR-113, require an already permitted event-time
projection that proves the named Googlebot identity, the exact
page and eligible URL at that time, the request time and source
generation. A general verified crawler label cannot supply the
identity. Use the current source admission, privacy checks and
coverage watermark. Daily totals and a first event in an open
file cannot prove the first request. A missing profile leaves the
feature unavailable. No new event intake, retention or remote query
follows from this report.

For the first elapsed value, require continuous proved eligible
coverage starting at original publication and ending at the
candidate request, including that request's instant. Under the
admitted clock, granularity and ordering, pick the earliest proved
eligible event in that prefix. Duplicate delivery of one event
does not change it. Hold an ambiguous order, gap or untrusted
clock. An earlier event admitted before coverage closes can change
the candidate. A late line refused by the 0034 AT-AN38 closed
interval rule adds nothing. An authorised source replacement
fences the earlier result and needs its own approved coverage.
Read the result only while both proofs remain current.

Show the elapsed value as “First verified Googlebot request
observed” with source precision and coverage. When a request
is independently eligible but the preceding interval is unproved,
show only the dated observation where disclosure permits. Say
“First request time unavailable.” No observation is evidence of
successful content delivery or indexing on its own. Missing
proof gives no zero, first observation or additional evidence for
ganit job 311. That job keeps its separate source qualification.

### Wave 2 local log admission and report bindings

The server-log intake above and 0034 own collection and labelling.
QR-088 governs the exact source and request evidence. QR-113 governs
report measures, comparable windows, privacy and coverage. These
fixtures use synthetic local files, stand-in verifier decisions and
approved stored projections. They create no remote log source,
account, DNS check, crawl or external action.

| Feature | Owning requirement and design | Planned fixture and expected evidence |
|---|---|---|
| F-0196 | REQ-0044-W2-LOG-INTAKE · DES-0044-W2-APPROVAL-BOUND-BATCH | FX-0044-W2-LOG-INTAKE · AT-SEO-LOG and AT-AN38. Open an owner-supplied local fixture only after exact current approval for this site, source, format, purpose, fields, retention and counting mode. A common-only file cannot prove request opt-out or consent. Try an admitted matched beacon, unmatched line, late line, unknown format, replay, stop and withdrawal at commit and after release. Commit results and cursor atomically. Unknown format yields SUT-653. Closed-day, denied or withdrawn evidence contributes nothing. Fence stale derivative outputs, including on restore, and preserve beacon-only counts. |
| F-0197 | REQ-0044-W2-VERIFIED-BOT-LABEL · DES-0044-W2-OWNED-BOT-CLASSIFICATION | FX-0044-W2-VERIFIED-BOT-LABEL · AT-AN03 and AT-AN07. Use the admitted standalone classification producer with synthetic published range proof or reverse and forward confirmation. Compare a verified claim, failed or unavailable check and the Audit crawler's own labelled observation. Apply 0034 precedence once per observation. Publish the eligible label and reason only and retain no address in the report. |
| F-0198 | REQ-0044-W2-SECTION-CRAWL-SHARE · DES-0044-W2-COMPARABLE-LOG-COUNTS | FX-0044-W2-SECTION-CRAWL-SHARE · IT-0044-W2-SECTION-CRAWL-SHARE. Compare two exact historical section mappings for one approved closed interval. Count eligible verified search requests from the same complete comparison scope. Name the denominator and coverage. Separate OWN, other automation and incomplete intervals. Missing or zero denominator withholds a share. These observations do not measure a search engine's assigned budget. |
| F-0199 | REQ-0044-W2-CRAWL-TRAFFIC-COMPARE · DES-0044-W2-COMPARABLE-LOG-COUNTS | FX-0044-W2-CRAWL-TRAFFIC-COMPARE · IT-0044-W2-CRAWL-TRAFFIC-COMPARE. Match exact eligible page and closed interval for verified crawler requests and separately disclosable human visits. Compare a proved zero, a low crawler count under the admitted value rule, and suppressed, unmatched or incomplete data. Show frequent or rare only under the admitted thresholds. Never combine the populations or infer a zero from suppression. |
| F-0200 | REQ-0044-W2-DATED-BOT-STATUS · DES-0044-W2-COMPARABLE-LOG-COUNTS | FX-0044-W2-DATED-BOT-STATUS · IT-0044-W2-DATED-BOT-STATUS. Use two verified crawler groups over comparable closed days. Show stored request, 4xx and 5xx totals with 429 a subset of 4xx. Respect scope, counts and suppression. An absent day remains unknown. Do not infer individual response codes from class totals or count 429 twice. |
| F-0201 | REQ-0044-W2-FIRST-OBSERVED-GOOGLEBOT · DES-0044-W2-EXACT-PUBLISHED-REQUEST-EVENTS | FX-0044-W2-FIRST-OBSERVED-GOOGLEBOT · IT-0044-W2-FIRST-OBSERVED-GOOGLEBOT. Give exact publication at 10:00:00 UTC and a verified same-page Googlebot request at 10:00:12 UTC with closed complete prefix. Expect 12 seconds. Separately admit a valid request at 10:00:05 before closure and expect 5 seconds once. In a separate run deliver that earlier line only after the AT-AN38 closed-day fence. Keep 12 seconds without counting another event. Try pre-publication traffic, another verified crawler, republish, a same-address different page, day totals, an open prefix, a missing interval and withdrawn proof. Show an independently eligible dated request if first elapsed is unproved. No new event retention or source read. |
| F-0202 | REQ-0044-W2-BOT-HUMAN-SERVER-WORK · DES-0044-W2-COMPARABLE-SERVER-WORK | FX-0044-W2-BOT-HUMAN-SERVER-WORK · IT-0044-W2-BOT-HUMAN-SERVER-WORK. Use two separately disclosable populations, one approved server-work measure, matching units, scope and coverage. Compare with request counts only, response medians only and a suppressed human group. Show measured work only with comparable proof. Keep counts as labelled context when permitted and otherwise show work unknown. No zero or inferred estimate for a suppressed group. |
| F-0203 | REQ-0044-W2-GOOGLEBOT-CLAIM-STATES · DES-0044-W2-OWNED-BOT-CLASSIFICATION | FX-0044-W2-GOOGLEBOT-CLAIM-STATES · AT-AN07. Reuse 0034's exact proof, failure and uncheckable cases. Given a verified claim, failed claim and unchecked claim, show VERIFIED, FAKE_CRAWLER and UNVERIFIABLE separately in permitted aggregates. Missing proof cannot imply fraud. No raw address, blocking action or other traffic control. |
| F-0204 | REQ-0044-W2-OBSERVED-AI-CRAWLER-COUNTS · DES-0044-W2-OWNED-BOT-CLASSIFICATION | FX-0044-W2-OBSERVED-AI-CRAWLER-COUNTS · AT-AN04. Use one eligible verified named AI crawler, an unverified claim, the Audit crawler labelled OWN and a separately classified human referral. Report only the verified crawler's eligible named activity in this view. Keep claims, OWN and referrals out of that count. A log count supplies no human referral or 0135 protected native fetch fact and no ranking input, robots change or follow-up. |
| F-0205 | REQ-0044-W2-CRAWL-CHANGE-ALERT · DES-0044-W2-COMPARABLE-LOG-COUNTS | FX-0044-W2-CRAWL-CHANGE-ALERT · IT-0044-W2-CRAWL-CHANGE-ALERT. Use closed comparable windows with ordinary counts, a qualified spike and drop and an import gap. Require one authorised local alert per supported transition, with dates, counts and admitted rule. Replay must not duplicate it. Missing coverage or unsupported rule yields no spike or drop and starts no extra crawl. |

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

### Wave 2 crawl comparison and response evidence

These Audit reports use stored guest observations or an explicitly
supplied offline robots draft. Bind the registered site, admitted
hosts, capture, rule version, coverage and current grant. Keep
XenForo and WordPress separate. Protected addresses and fields never
reach display, storage or proof. Use only synthetic fixtures here.
The report starts no acquisition, retry, navigation or site edit.

| Feature | Owning requirement and design | Planned fixture and expected evidence |
|---|---|---|
| F-0115 | REQ-0044-W2-CRAWL-DIFF · DES-0044-W2-SNAPSHOT-COMPARE | FX-0044-W2-CRAWL-DIFF · UT-LS-10. Use two valid signed own-site snapshots with compatible scope, stable page mappings and coverage. Show supported added, removed and changed pages and issues. Refuse a foreign or unverifiable capture. Erased, unread, truncated and unmapped records prove no removal. Use the existing bounded streaming pass. LS-09 supplies capture lineage only. A comparison opens no network connection. |
| F-0117 | REQ-0044-W2-SITEMAP-DIFFERENCE · DES-0044-W2-SITEMAP-OBSERVATIONS | FX-0044-W2-SITEMAP-DIFFERENCE · IT-0044-W2-SITEMAP-DIFFERENCE. Compare stored native sitemap sets, observed links and response facts of matching site, identity and comparable capture. Give a listed unlinked address, an observed link missing from a complete eligible set, a listed redirect and a listed error. An unread child leaves absence unknown in affected coverage. No submission, repair or remote read. Reuse the existing sitemap check and RN-23. |
| F-0118 | REQ-0044-W2-SITEMAP-NOINDEX · DES-0044-W2-SITEMAP-OBSERVATIONS | FX-0044-W2-SITEMAP-NOINDEX · IT-0044-W2-SITEMAP-NOINDEX. Match an observed sitemap entry to current noindex evidence for its exact eligible resolved version. Include a blocked or unseen version and an observed indexable page. A crawl refusal gives no noindex fact. Reuse the sitemap and noindex never-seen cases. Never remove an entry. |
| F-0119 | REQ-0044-W2-SITEMAP-LIMITS · DES-0044-W2-BOUNDED-SITEMAP-READER | FX-0044-W2-SITEMAP-LIMITS · UT-RN-19 and UT-RN-21. For decoded sitemap and index fixtures, exercise 50,000 entries and 52,428,800 bytes separately, then one beyond each. Use plain and compressed fixtures and malformed declared dates. These are the existing Wave protocol-check boundaries. If an admitted reader or acquisition cap stops earlier, name the cap and report partial evidence. Never raise an acquisition, expansion, depth, memory or time limit to reach a protocol boundary. No external entities, tail guesses or completeness claim. |
| F-0120 | REQ-0044-W2-ROBOTS-SYNTAX · DES-0044-W2-ROBOTS-OFFLINE-ANALYSIS | FX-0044-W2-ROBOTS-SYNTAX · UT-LV-01 and FZ-LV-01. Use valid and malformed synthetic directives, a comment and an overlong file with a directive beyond the 500 KiB cut. Separate admitted-prefix defects from truncation. Do not report knowledge of the unread tail or turn the diagnostic into an effective admission decision. The bound is the project's existing bound. |
| F-0121 | REQ-0044-W2-ROBOTS-SIMULATION · DES-0044-W2-ROBOTS-OFFLINE-ANALYSIS | FX-0044-W2-ROBOTS-SIMULATION · IT-0044-W2-ROBOTS-SIMULATION. Compare a current eligible stored policy and a supplied draft against one synthetic same-site address and the admitted crawler identity. Show each deciding rule and group with current or draft provenance. An unavailable, stale or unread current policy stays unknown in the simulation and follows 0082's hold for actual admission. The draft never changes policy or permits a fetch. Reuse UT-LV-01 for the rule decision. |
| F-0122 | REQ-0044-W2-RESPONSE-CODES · DES-0044-W2-OBSERVED-RESPONSES | FX-0044-W2-RESPONSE-CODES · IT-0044-W2-RESPONSE-CODES. Use dated redirect and error responses with eligible source-page observations and safe references. Include a timed-out attempt and a denied destination. Report the recorded status only for a response that occurred. Preserve referring pages, dates and coverage. A timeout or refusal never becomes a synthetic HTTP status. |
| F-0123 | REQ-0044-W2-INTERMITTENT-FAILURE · DES-0044-W2-OBSERVED-RESPONSES | FX-0044-W2-INTERMITTENT-FAILURE · IT-0044-W2-INTERMITTENT-FAILURE. Compare compatible saved observations of an alternating response outcome with a single failure, a denied observation and a missing interval. Show eligible response history, counts and dates. A diagnosis requires support from its admitted comparison profile. One isolated event or missing observation cannot establish recurrence. This report starts no retry or rival check. |
| F-0124 | REQ-0044-W2-REDIRECT-CHAIN · DES-0044-W2-OBSERVED-REDIRECTS | FX-0044-W2-REDIRECT-CHAIN · IT-0044-W2-REDIRECT-CHAIN. Reuse the saved three-hop and loop cases. Add a chain ending at the existing hop cap and one with a refused offsite hop. Show observed admitted hops, proved loops and the safe stop reason. Mark the unobserved tail unknown. Use 0082 LV-R05 and UT-LV-04 for per-hop refusal evidence. No request from the report. |
| F-0125 | REQ-0044-W2-TEMPORARY-MOVE-MISMATCH · DES-0044-W2-OBSERVED-REDIRECTS | FX-0044-W2-TEMPORARY-MOVE-MISMATCH · IT-0044-W2-TEMPORARY-MOVE-MISMATCH. Match a stored temporary response to independently admitted own-site move intent binding source and destination. Compare an old observation without move evidence and a stale or mismatched record. Report only a supported mismatch. Hold when the move-evidence profile is absent. Time observed supplies no permanent intent. |
| F-0126 | REQ-0044-W2-REDIRECT-TRIGGER · DES-0044-W2-OBSERVED-REDIRECTS | FX-0044-W2-REDIRECT-TRIGGER · IT-0044-W2-REDIRECT-TRIGGER. Compare a stored server response, static meta refresh and script-trigger observation from an admitted 0115 capture. Separate the recorded trigger or target from proof that the destination was reached. Include a partial or unavailable capture and a refused offsite target. Reuse RN-01, RN-03 and RN-18. The report does not execute or follow anything. |
| F-0127 | REQ-0044-W2-UNEXPECTED-HOME-ROUTING · DES-0044-W2-OBSERVED-REDIRECTS | FX-0044-W2-UNEXPECTED-HOME-ROUTING · IT-0044-W2-UNEXPECTED-HOME-ROUTING. Match a known content source and eligible stored hop to a proven registered home destination under the current site mapping. Compare a legitimate observed root alias and an unread or refused target. Flag only the supported unexpected home route. Do not infer a reached page from a location alone, or edit routing. |

### HTTPS redirect evidence (128)

F-0128 belongs to the 0044 Audit. Under DR-109, the owner supplies
purpose-approved edge or access records for the registered site. The
report uses the existing site-scoped Audit permission. Reuse spec
0100's bounded picker
and parser. The versioned import admits source, site, observation
time, deployment edge, scheme, approved path identity, status and
scrubbed location, plus scope and coverage. Validate and scrub
in memory before preview, staging, spill or logging. No raw copy.
Reject visitor identities, credentials, unapproved header data or
query values. Recheck site, scope and grant before committing safe
observations to the audit store, separate from ordinary analytics.

Show “HTTPS redirect recorded” for a comparable HTTP observation
with a redirect to a scope-approved HTTPS address. The recorded
hop does not prove a later hop or TLS success. Show a gap when an
eligible observation directly serves the page over HTTP or redirects
to HTTP. For stale, ambiguous, denied or incomplete evidence show
“Cannot confirm”. Keep edge, time and coverage in the result.
Incomplete coverage cannot hide a supported local gap or prove a
whole-site result. This report fetches neither scheme and edits nothing.

| Identity | Feature contract |
|---|---|
| REQ-0044-HTTPS-EVIDENCE | The Audit reports redirect evidence, a supported HTTP gap or “Cannot confirm” from the approved supplied records alone. |
| DES-0044-SUPPLIED-HTTPS | The local Audit uses the approved picker and source mapping, scrubs before any preview or storage, rechecks current site and grant at commit, and shows only the scoped result. |
| VAR-0044-HTTPS-EDGE | Owner-supplied edge record with an eligible deployment edge and an admitted source profile for the exact site. |
| VAR-0044-HTTPS-ACCESS | Owner-supplied access record with all required facts and an admitted source profile for the exact site. No missing field is inferred from its file type. |
| UI-0044-HTTPS-EVIDENCE | The scoped Audit result keeps edge, observation time and coverage. A report gives no authority to import again or open either URL. |

Both input variants use the same evidence rule for registered XenForo
and WordPress sites. They use no native site call. An unproved source
profile stays unavailable. The feature supplies no ranking signal.
Ordinary public pages gain zero queries, time or bytes.

| Requirement and design | Planned verification |
|---|---|
| REQ-0044-HTTPS-EVIDENCE and DES-0044-SUPPLIED-HTTPS | FX-0044-SUPPLIED-HTTPS and BDD-0044-HTTPS-EVIDENCE: given synthetic exact-site records for each admitted input variant, show a recorded HTTPS redirect, direct HTTP or HTTP redirect gap, and “Cannot confirm” for missing, stale or ambiguous evidence. Mixed coverage retains a supported local gap. Wrong site and mid-import revocation disclose no result. Planted visitor identifiers, credentials and query values reach no preview, store, spill, log or receipt. Observe no network request or public-page work. |

QR-109 owns the remaining source admission and evidence. Bind the
exact input profile and expiry, typed local interface and errors,
audit retention and deletion, and the scoped unit and integration
cases before deriving a build task. This binding records the feature
scope and planned case under spec 0076.

### Wave 2 URL and link observation bindings

Use stored eligible guest observations for the exact registered site,
profile, capture and source or destination revision. Keep XenForo and
WordPress evidence apart. An unread, refused, stale or out-of-scope
record cannot establish identity, absence or reachability. Use the
existing site-scoped Audit read and safe display rules. Scrub protected
URL fields before any result or proof. A fixture uses synthetic records
only. These report cases add no reads, redirects, rank input or effects.

| Feature | Owning requirement and design | Planned fixture and expected evidence |
|---|---|---|
| F-0129 | REQ-0044-W2-HOST-DUPLICATES · DES-0044-W2-EXACT-URL-IDENTITY | FX-0044-W2-HOST-DUPLICATES · IT-0044-W2-HOST-DUPLICATES. Compare independently admitted www and non-www observations with exact 0022 identity proof, distinct destinations and an unadmitted alias. Report only the evidenced duplicate with its observations. A guessed host is never admitted or opened. Reuse the existing refused-host case. |
| F-0130 | REQ-0044-W2-SLASH-VARIANTS · DES-0044-W2-EXACT-URL-IDENTITY | FX-0044-W2-SLASH-VARIANTS · IT-0044-W2-SLASH-VARIANTS. Compare a slash pair resolving to one proved canonical identity, a pair with separate identities and an unread variant. Label observed inconsistent referring links only for the supported pair. Preserve exact paths and the unknown result. |
| F-0131 | REQ-0044-W2-CASE-VARIANTS · DES-0044-W2-EXACT-URL-IDENTITY | FX-0044-W2-CASE-VARIANTS · IT-0044-W2-CASE-VARIANTS. Use one case-only pair with distinct content identities and one verified duplicate pair. Preserve path case in eligible evidence. Never join from a case-folded path or claim duplicate content from spelling alone. |
| F-0132 | REQ-0044-W2-URL-STRUCTURE · DES-0044-W2-SAFE-URL-REPORT | FX-0044-W2-URL-STRUCTURE · IT-0044-W2-URL-STRUCTURE. Exercise parameter, underscore, length and depth dimensions independently. Use synthetic admitted units and boundary values, a missing profile, and escaped and protected fields. Keep literal and decoded meanings distinct in the parser. Show safe evidence for each supported dimension, hold unadmitted limits and disclose no protected value. No change to the request identity. |
| F-0133 | REQ-0044-W2-MIXED-LINK-VARIANTS · DES-0044-W2-POSITIONED-LINK-EVIDENCE | FX-0044-W2-MIXED-LINK-VARIANTS · IT-0044-W2-MIXED-LINK-VARIANTS. Give two observed internal href variants resolving to the same proven 0022 identity. Retain distinct eligible source positions and native relation marks. An unresolved target is unknown and never joins the group. No link is rewritten. |
| F-0134 | REQ-0044-W2-TRACKING-PARAMETER · DES-0044-W2-SAFE-URL-REPORT | FX-0044-W2-TRACKING-PARAMETER · IT-0044-W2-TRACKING-PARAMETER. Observe an internal href with a key admitted as tracking, another with a functional or unclassified key and a protected value. Show only the authorised finding and scrubbed location. Apply 0032 before display, storage or proof. Unclassified keys are left alone. No stripping or redirect. |
| F-0135 | REQ-0044-W2-LINKED-NOINDEX · DES-0044-W2-POSITIONED-LINK-EVIDENCE | FX-0044-W2-LINKED-NOINDEX · IT-0044-W2-LINKED-NOINDEX. Match a current observed link and exact resolved revision to the observed noindex case. Compare a blocked target, stale noindex on an older revision and an observed indexable page. The blocked or stale record is unknown. Reuse the noindex never-seen test. No indexing or link action. |
| F-0136 | REQ-0044-W2-HTTP-LITERAL-LINK · DES-0044-W2-SAFE-URL-REPORT | FX-0044-W2-HTTP-LITERAL-LINK · IT-0044-W2-HTTP-LITERAL-LINK. Supply stored internal HTTP and HTTPS href text and one unresolved target. Report supported literal scheme evidence and unknown resolution separately. No request to either address, assumed HTTPS equivalent or automatic rewrite. F-0128 and DR-109 alone govern the separate supplied redirect evidence. |
| F-0137 | REQ-0044-W2-SCRIPT-LINK-STAGE · DES-0044-W2-RENDERED-LINK-EVIDENCE | FX-0044-W2-SCRIPT-LINK-STAGE · IT-0044-W2-SCRIPT-LINK-STAGE. Compare exact raw, completed load-time and permitted passive-reveal captures under the existing 0115 Audit. Show raw href, load insertion and observed reveal-only href as separate stages. A refused action does not run, and a partial capture cannot show absence or crawler reachability. Reuse RN-18, RN-20, UT-RN-16 and UT-RN-18. |
| F-0138 | REQ-0044-W2-ANCHOR-TEXT · DES-0044-W2-RENDERED-LINK-EVIDENCE | FX-0044-W2-ANCHOR-TEXT · IT-0044-W2-ANCHOR-TEXT. Compare an empty, an admitted generic and a descriptive visible label with separate accessibility-tree names. A descriptive accessible name does not change the visible result. Keep source position, template and covered render stage. Missing visible or accessibility evidence is unknown for that field. Reuse 0115 RN-12 and UT-RN-10. |
| F-0139 | REQ-0044-W2-TEMPLATE-OUTBOUND · DES-0044-W2-TEMPLATE-LINK-EVIDENCE | FX-0044-W2-TEMPLATE-OUTBOUND · IT-0044-W2-TEMPLATE-OUTBOUND. Use compatible 0047 layout-family and revision evidence, an admitted count rule and bound, a consistently excessive observed sample and a comparable reference. Keep unread pages separate. Withhold the warning when the profile or floor is missing. Name the population and rule. Count observed outbound hrefs without opening destinations, inferring intent or editing links. |

### Migration comparison and protected staging (193 and 194)

Under DR-110, compare an approved local staging export to a signed
live audit. Before opening a file, the owner binds the install, active
site, staging project, migration mapping, allowed sections and grant
in the app. The file cannot supply that authority.

The bounded, versioned capture has source identity, time, coverage
and page records. Admit only mapped page IDs, scrubbed canonical
addresses, response class, page fingerprint, metadata findings and
visibility observations. Reuse spec 0100's safe picker and parser.
QR-111 admits the exact source and format. A general site backup,
raw archive, location pasted into the app or file extension alone
gives no profile.

Reuse 0100's F-0599 exact chosen file and mapping proof. The selected
handle supplies one stable bounded read. Use only the protected
scrubbed projection for preview and commit. Pin the admitted profile,
protected input and projection digests, approved mapping revision
and current grant to the receipt. If a live audit was chosen for
comparison, bind that signed audit separately. The file cannot
choose or replace it. Never resolve the pathname again to recover
content. Refuse substituted bytes. An unproved change to the opened
file, mapping or access holds commit. The owner can select and review a new file
through the same approved picker. A proved lost-reply replay
returns the original eligible receipt without opening a file again.

Refuse duplicate or ambiguous mappings, foreign records, private
values, credentials and executable fields. Scrub before any preview,
spill, write or log. Do not make a raw staging copy or open a
connection. Retain accepted facts sealed under spec 0118 for at
most seven days, as a proposed bound. Recheck grant at commit and
every read. Loss of scope or erasure withdraws derived results.

Live and staging facts need comparable scope, dates and versions.
A complete, mapped capture may prove an item absent. Treat stale,
unmatched or incomplete records as unknown. A replacement site can
be the staging project if its mapping and scope are approved. Display
source and coverage alongside supported changes. Any move advice
stays in review. The crawler never logs into the staging site.

REQ-0044-STAGING requires scoped supplied facts and no staging calls.
BDD-0044-STAGING compares valid export and audit, then tests mixed
sites, a stale mapping, incomplete data, planted sensitive fields,
revocation during parsing and an attempted authenticated crawl.
Replace the selected file at the same path after preview, change
its bytes or mapping and try an unadmitted owner-supplied backup.
The protected result uses the exact eligible reviewed projection
or holds with no new partial result. Lose the reply to a successful
commit and reconcile the same eligible receipt. No file is reopened.

| Requirement | Verification contract |
|---|---|
| REQ-0044-STAGING | BDD-0044-STAGING: compare exact protected file, scope and signed audit. Check wrong or changed inputs during preview and commit and one known commit after a lost reply. No staging call is made |

### Wave 2 observed changes and stored-copy checks

Use synthetic records of eligible own-site guest evidence and current
site grants. Bind the precise source, revision, profile and coverage.
A check introduces no new acquisition, attack sample, public effect
or authority to edit. An unavailable source or profile holds the
affected finding. QR-110 still owns source and profile admission.

#### Indexed URLs and exact content revisions

Read only 0040's eligible saved inspection for the same site, current
property binding and exact inspected URL. Keep the provider verdict,
Google-selected canonical, result observation time and provider crawl
time distinct. Use 0040's dated status mapping. Where its evidence
supports it, show “Google reported this URL indexed” and the result
date. Report an alternate canonical, an uninspected URL and an unknown
or unavailable result separately. Qualification and current access
apply to each source. A historical result remains dated if access
still permits it.

F-0175 shows local PDF or document facts for the exact eligible
capture, document identity and admitted reader. Name the byte basis
and date. If the eligible URL result reports indexed and the exact
version is unproved, show “We could not verify which version Google
indexed.” Preserve the distinct dated URL result.
Withhold derived findings from a withdrawn source at read or release.

QR-110 must independently admit an exact positive revision binding
before either feature makes a combined claim. It binds site and
current property generation, inspected URL and confirmed canonical,
retained inspection receipt, provider crawl observation, exact local
capture and document or page revision, applicable representation,
and independent evidence with its own scope, expiry and digest.
The profile must prove the link between the provider's indexed
observation and the specific eligible version. A matching URL, a
valid signature, a close timestamp or a fresh inspection alone
supplies no such link. Withhold a join from an earlier crawl to a
later edited version. Missing or conflicting identity holds the join.

F-0179 uses that binding and the current eligible suspicious-content
finding to prepare a private review proposal. When the binding is
missing or held, show any independent eligible findings separately.
Say “The page was flagged, but we could not verify that Google
indexed this version” when the separate eligible URL result
reports that URL indexed. Withhold the indexed-spam proposal. Before
display or hand-off recheck both grants, exact revisions and admission.
The separate F-0180 observation remains available under its own
scope. Neither path opens the page or provider, requests indexing,
changes ranking or edits the site.

| Feature | Owning requirement and design | Planned fixture and expected evidence |
|---|---|---|
| F-0174 | REQ-0044-W2-SCRIPT-COST-EVIDENCE · DES-0044-W2-SCRIPT-OBSERVATION | FX-0044-W2-SCRIPT-COST-EVIDENCE · IT-0044-W2-SCRIPT-COST-EVIDENCE. Read a synthetic eligible exact-script processing sample of 40 ms CPU. Separately use the qualified 0115 offline receipt under QR-110. A fixed largest visible block paints 740 ms with the script and 620 ms with it omitted. Show +120 ms at that simulated milestone and retain 40 ms CPU as its own value. Use 620 ms for both legs to show 0 ms. With the script at 600 ms and the omitted leg at 620 ms, show -20 ms. Hold for a changed viewport, largest block or dependency, incomplete recording, missing timing or grant withdrawal. Only complete comparable pairs supply a difference. Preserve independent eligible processing evidence. Reuse RN-10/16/42 and UT-RN-08/14 and N-1677. No report replay, live profiling or real-visitor claim. |
| F-0175 | REQ-0044-W2-DOCUMENT-OBSERVATIONS · DES-0044-W2-INERT-DOCUMENT-EVIDENCE | FX-0044-W2-DOCUMENT-OBSERVATIONS · IT-0044-W2-DOCUMENT-OBSERVATIONS. Match admitted 0115 PDF and document metadata and separately eligible 0040 URL status. Google crawls revision A. The same URL serves revision B before a later inspection with matching canonical. Show B's observed title and byte basis and the dated URL status, with indexed version unverified. Try an alternate canonical, a wrong property, a denied document and source withdrawal. Only an independently admitted exact positive binding joins the content and index facts. Reuse RN-21/24. No opening, execution or provider request. |
| F-0176 | REQ-0044-W2-PAIRED-VISIBILITY · DES-0044-W2-QUALIFIED-GUEST-COMPARISON | FX-0044-W2-PAIRED-VISIBILITY · IT-LE-09. First use a single eligible RN-09 Audit observation with proved zero opacity in one region and a proved closed help control in another. Show scoped visibility and the reason with no paired result. Test raw Receipts, incomplete CSS or layout, outside-viewport evidence and grant withdrawal. Separately use an eligible RN-45 pre-script pair with a supported response difference. Withhold that comparison for different serving generation, viewport, script timing, partial coverage or change of scope. Keep any independently qualified visibility fact. Reuse 0105 N-1825/N-1826. Starts no render and gives no search-treatment or intent claim. |
| F-0177 | REQ-0044-W2-NEAR-DUPLICATE-PAGES · DES-0044-W2-QUALIFIED-GUEST-COMPARISON | FX-0044-W2-NEAR-DUPLICATE-PAGES · UT-LS-09 and MS-LS-02. Compare repeated layout with distinct main content, confirmed near duplicates and a partial sample. Use the admitted LS-13/36 read-only fingerprint and confirmation profile. Report supported similarity and sample size; an unread page proves no duplicate or absence. No intent claim, canonical change, merge, redirect or noindex. |
| F-0178 | REQ-0044-W2-VISIBLE-STRUCTURED-DATA · DES-0044-W4-OBSERVE | FX-0044-W2-VISIBLE-STRUCTURED-DATA · UT-0044-W4-STRUCTURED-DATA and UT-RN-20. Reuse F-1316's one validator and RN-22. Compare a declared field at an exact entity and revision with matching, contradicting and incomplete visible evidence. Show supported inconsistency with field and provenance. An unobserved region yields unknown. No second parser, indexing verdict, markup edit or external context resolution. |
| F-0179 | REQ-0044-W2-INDEXED-SPAM-REVIEW · DES-0044-W2-LOCAL-REMEDIATION-EVIDENCE | FX-0044-W2-INDEXED-SPAM-REVIEW · IT-0044-W2-INDEXED-SPAM-REVIEW. Google crawls synthetic clean A. A later same-URL capture flags B and an inspection still reports the URL indexed. Show separate eligible facts and unavailable indexed-version evidence. Prepare no indexed-spam proposal. Test a compatible independent positive binding to the flagged revision under QR-110, then change its canonical, current revision, source grant or proof. Only the eligible bound version supports a private review proposal. F-0180 retains its separate observation. No deletion, indexing effect, external query or rank input. |
| F-0180 | REQ-0044-W2-SUSPICIOUS-CONTENT-CHANGE · DES-0044-W2-SIGNED-CHANGE-EVIDENCE | FX-0044-W2-SUSPICIOUS-CONTENT-CHANGE · IT-0044-W2-SUSPICIOUS-CONTENT-CHANGE. Compare a synthetic clean capture and changed eligible observation with a scrubbed unexpected content category. Include a known authorised update and a partial capture. Show dated evidence privately, differentiate known change and unknown, and make no compromise diagnosis solely from an observed difference. No active input, command, intrusion or automatic repair. |
| F-0181 | REQ-0044-W2-UNEXPECTED-PAGE-CHANGE · DES-0044-W2-SIGNED-CHANGE-EVIDENCE | FX-0044-W2-UNEXPECTED-PAGE-CHANGE · IT-0044-W2-UNEXPECTED-PAGE-CHANGE. Compare signed eligible observations under F-1327 and 0116 LS-08/14. A separately proved title update explains the title change only. Add an unrelated body change in the same later capture and keep one review notice for the unexplained change. Compare a general deployment marker, a different native revision or site, missing prior sample and withdrawn proof. Keep dates and change evidence by scope. Replay the exact unexplained transition through restart. Produce one eligible local notice, no warning on a missing comparison and no confirmed-defacement label. No added recurrence, acquisition or repair. |
| F-0182 | REQ-0044-W2-SCOPED-SITE-HEALTH-READOUT · DES-0044-W2-STORED-SITE-HEALTH-EVIDENCE | FX-0044-W2-SCOPED-SITE-HEALTH-READOUT · IT-0044-W2-SCOPED-SITE-HEALTH-READOUT. Use synthetic stored availability, DNS and admitted F-0164 leaf expiry for the exact own-site origin. A current availability row does not renew an ineligible TLS fact. Distinguish an observed expiry date, a failed observation and missing proof. Old same-site evidence remains dated only while authorised. Keep the F-0164 alert identity and current cooldown. 0112's local readiness is no site uptime evidence. Withhold missing DNS or uptime profiles. Starts no DNS lookup, probe, route or repair. |
| F-0183 | REQ-0044-W2-BOUNDED-STORED-EXTRACTION · DES-0044-W2-SCRUBBED-COPY-READ | FX-0044-W2-BOUNDED-STORED-EXTRACTION · IT-0044-W2-BOUNDED-STORED-EXTRACTION. Compare independently admitted bounded CSS, XPath and regular-expression fixtures on scrubbed own-site copies. Require typed projected fields, source and filter version. Refuse unsupported syntax, forbidden projection, unadmitted processor and exceeded row, memory or time limits. No live execution, new fetch or raw private field. Reuse 0115 reader bounds and the admitted 0104 fence where applicable. |
| F-0184 | REQ-0044-W2-SCOPED-STORED-SEARCH · DES-0044-W2-SCRUBBED-COPY-READ | FX-0044-W2-SCOPED-STORED-SEARCH · IT-0044-W2-SCOPED-STORED-SEARCH. Test exact text and inert code strings against a current permitted stored projection. Show eligible matches with age and scope. Refuse a different site and recheck withdrawals before response. Unavailable or stale evidence cannot support a current result. Apply 0038 custody without treating it as an index admission. No code execution or acquisition. |
| F-0185 | REQ-0044-W2-FENCED-CRAWL-CHECK · DES-0044-W2-APPROVED-PYTHON-CHECK | FX-0044-W2-FENCED-CRAWL-CHECK · IT-0044-W2-FENCED-CRAWL-CHECK. Use only the DR-019 and 0104 exact approved advisory Python check, immutable eligible own-site snapshot and bounded typed results. Model a held page, unread page, an over-limit result and a forbidden permission. Give one result or safe hold per eligible page with no hidden gaps or duplicates. A denied or revoked check releases no output. No Rhai, network, work on a live page, public write or rank override. |

### Expected changes and key-page notices (181)

Under QR-110 use F-1327 and 0116 LS-08/14 to compare eligible signed
old and new observations of the same own-site page with compatible
revision lineage, profile and coverage. Keep the two dates and
actual observed differences. An unread or incomplete observation
gives “We could not compare this page.” It supplies no notice of an
unexpected change.

A dated deployment or edit may explain a difference only where an
independently admitted existing effect receipt binds the site, page,
exact old and new revisions or their proved lineage, affected
fields or regions, committed change, current scope and immutable
evidence. A notice or generic annotation supplies context. It
cannot make an observed change expected on its own. If a receipt
explains only a changed title, keep other observed changes subject
to the ordinary notice rule. A signed capture proves the dated
observation. It supplies no edit authorisation.

Show observed changes with the exact eligible explanation or
“Change observed. We could not verify that it was expected.”
An observed change remains visible when its expected status is
proved. Publish one local review notice for an eligible unexplained
transition under the admitted existing rule. Use 0032 and 0099,
with a stable source event bound to the site, page, profile and
exact comparison lineage and unexplained change. Reconcile replay
or restart to that event. Recheck scope at display and dispatch.
Withdrawal can remove the explanation. Keep the dated change under
its own rights. Withdrawal creates no alert for a previously assessed
transition and cannot reopen a prior dispatch. A superseded source
observation holds its pending notice under 0099. This report provides
no new deployment producer, change, site check or schedule.

### Custom and spelling checks (183 to 186)

Features 183 to 185 read stored, scrubbed, eligible own-site facts.
Feature 185 uses spec 0104's versioned fenced Python workers under
DR-019. The runner cannot change a page or ranking policy.
DR-112 gives feature 186 to the clean-room writing suite in spec
0121. This audit supplies eligible own-site text and reads named
findings by reference. Member checks need both the separate
default-off owner grant and member permission for that purpose.
Staff checks require the current staff grant. A stopped or
unavailable check holds its findings. The route works offline.

### Wave 2 crawl report controls and supplied comparisons

Use synthetic cases with the current authorised site, exact capture
generation, coverage and admitted source. Unknown or withdrawn facts
cannot pass a report check. The fixtures simulate effects only where
the owning route already permits them and add no calls or authority.
QR-110 retains exact report and input qualifications.

| Feature | Owning requirement and design | Planned fixture and expected evidence |
|---|---|---|
| F-0187 | REQ-0044-W2-CRAWL-SEGMENTS · DES-0044-W2-SHARED-REPORT-SCOPE | FX-0044-W2-CRAWL-SEGMENTS · IT-0044-W2-CRAWL-SEGMENTS. Apply exact eligible section and 0047 layout segments plus a separately authorised author projection to the same synthetic reports. Bind the definition, site, generation and coverage to the count and page list. Unproved membership remains unknown. Reject a cross-site or unapproved author segment, and withhold revoked or unsafe results before filtering or counting. No new author lookup. |
| F-0188 | REQ-0044-W2-CRAWL-GRAPH-VIEWS · DES-0044-W2-SHARED-REPORT-SCOPE | FX-0044-W2-CRAWL-GRAPH-VIEWS · BDD-0044-W2-CRAWL-GRAPH-VIEWS. Use one bounded eligible 0116 graph, with a cycle and unread edges. Render force-directed and tree views in the 0017 accessible profile. Retain the same nodes, edges and coverage in both, including a labelled non-tree edge or cycle reference, and in the equivalent table or text. Missing coverage or a display limit gives a safe reason, never invented topology. |
| F-0189 | REQ-0044-W2-ISSUE-TRAFFIC-ORDER · DES-0044-W2-ADMITTED-ISSUE-ORDER | FX-0044-W2-ISSUE-TRAFFIC-ORDER · IT-0044-W2-ISSUE-TRAFFIC-ORDER. Compare equal-severity findings supported by disclosable 0034 human traffic for the same compatible scope and window, missing or suppressed counts and a critical finding with few observed views. Show traffic context and order only under an admitted Audit rule. Preserve 0048's own kind and evidence ordering. Never make an unknown count zero, hide a critical finding or feed link ranking. |
| F-0190 | REQ-0044-W2-SCOPED-FIX-RECHECK · DES-0044-W2-EXISTING-GUEST-RECHECK | FX-0044-W2-SCOPED-FIX-RECHECK · IT-0044-W2-SCOPED-FIX-RECHECK. Reuse 0048's Check my fix route and its admitted caps, current claim and named pages. Compare two right, two proved gone, one gone plus right and one gone plus unread. Include a later redirect's 404, blocked response, stale observation and revoked grant. A proved gone result applies to its exact page; mixed outcomes keep the full approved denominator and hold closure. Every page must be right to verify or proved gone for the item to become Page gone. No broadened scope or dispatch without the existing cap and grant. |
| F-0191 | REQ-0044-W2-GROUPED-CRAWL-ALERT · DES-0044-W2-SCOPED-CRITICAL-TRANSITIONS | FX-0044-W2-GROUPED-CRAWL-ALERT · IT-0044-W2-GROUPED-CRAWL-ALERT. Compare a new proved critical finding, equivalent replay, admitted failure after verified recovery and missing evidence. Record one scoped grouped local alert per supported transition, with safe evidence and time. A partial finding cannot raise or clear it. No extra collection, external destination or automatic resolution. |
| F-0192 | REQ-0044-W2-OFFLINE-REDIRECT-MAP · DES-0044-W2-APPROVED-MIGRATION-INPUT | FX-0044-W2-OFFLINE-REDIRECT-MAP · IT-0044-W2-OFFLINE-REDIRECT-MAP. Use an approved inert map with a supported pair, duplicate conflict, loop and denied target. Distinguish declared topology from exact eligible stored route observations. An unread destination stays unknown even if a proposed path looks valid. Report only scope-checked safe facts. No probing, executing redirects or writing routes. |
| F-0193 | REQ-0044-STAGING · DES-0044-W2-SUPPLIED-STAGING-COMPARE | FX-0044-W2-SUPPLIED-STAGING-COMPARE · BDD-0044-STAGING and UT-LS-10. Compare a DR-110 approved local export against a signed live audit for the same mapped compatible observations. Show observed absence only in complete, current mapped scope. Include one unmatched item and one wrong mapping. Hold unknown or refuse mismatched scope. Reuse the bounded comparison. The result initiates no acquisition, switch-over or site edit. |
| F-0194 | REQ-0044-STAGING · DES-0044-W2-SUPPLIED-STAGING-INGRESS | FX-0044-W2-SUPPLIED-STAGING-INGRESS · BDD-0044-STAGING and 0100 F-0599 picker and mapping proof. Bind the active site, install, project and approved mapping before opening the owner-picked export. Try a same-path replacement after preview, changed bytes or mapping, planted unapproved fields, wrong site, withdrawn grant, an unadmitted backup and an authenticated crawl request. Keep only the exact eligible reviewed projection or hold. Recheck before commit and every read. A proved lost reply recovers one eligible receipt. No raw copy, credential, connection or exception to guest-only crawling. |
| F-0195 | REQ-0044-W2-APPROVED-CRAWL-EXPORT · DES-0044-W2-SCOPED-REPORT-HANDOFF | FX-0044-W2-APPROVED-CRAWL-EXPORT · IT-0044-W2-APPROVED-CRAWL-EXPORT. Test one eligible projection with CSV formula-starting text and a true negative number, typed Parquet, and a separately enrolled 0020 read-only MCP view. Reuse 0100 SD-32 and UT-SD-13 for CSV neutralisation. Preserve Parquet types and seal the owner-only local files and manifest under existing grant, path and custody rules. Wrong recipient, incomplete proof or a revoked grant holds the affected hand-off. No raw secrets, remote delivery or file write through a read-only view. |

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

### Wave 1 competing pages and brand traffic

| Feature | Requirement and design | Planned fixture and expected evidence |
|---|---|---|
| F-0016 | REQ-0044-W1-CANNIBALISATION, DES-0044-W1-QUERY-COMPETITION. Compare eligible own canonical pages for one query only on matching 0040 property, surface, projection and final dates. Job 265 provides candidate evidence. Any merge, differentiation or link proposal needs its own evidence and stays advice. | FX-0044-W1-CANNIBALISATION, IT-0044-W1-CANNIBALISATION. Extend the existing Search Console cannibalisation fixture with a supported pair, a third page missing a final day and an unsupported remedy. Verify the pair, the reason for each supported proposal and the missing coverage. A missing day supplies no zero. The unsupported remedy is absent. No page, metadata, link or ranking write. Reuse IT-0043-OWN for source compatibility. |
| F-0070 | REQ-0044-W1-BRANDED-SPLIT, DES-0044-W1-BRAND-CLASSIFICATION. Keep the site's name and approved aliases in a versioned classification rule. Show separately the eligible branded and non-branded search clicks from the same admitted 0040 query projection and final window. Divide by the eligible classified clicks in that projection, with the basis and excluded coverage visible where disclosure allows. A missing or zero denominator withholds shares. Job 80 receives only allowed flags and query IDs. | FX-0044-W1-BRANDED-SPLIT, IT-0044-W1-BRANDED-SPLIT. Supply an exact brand, an approved alias, a clear non-brand, a homonym and an unavailable query row. Check numerator, denominator, rule version and source. The homonym and suppressed row do not enter either class. Apply 0040 term privacy to views, logs and files. Reuse IT-0043-OWN and the existing query-privacy test. No property-wide extrapolation or ranking admission. |

### ganit jobs used

The service exports the link graph (thread ids and edges), crawl counts,
log counts and Search Console figures keyed by page and query ids.
Search terms never leave the `sanket` module: it marks each query id
with flags such as "contains a brand term" or "starts with a question
word", and ganit sees only those flags. Each result is inert candidate
evidence; spec 0077 alone evaluates, promotes, activates and rolls it back.
A late or failed candidate leaves the verified champion unchanged.

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

### Wave 2 visible content and render evidence

Use the current registered site's eligible guest observations and
pin the exact page, source, capture profile, revision and coverage.
Keep raw HTML, admitted rendered facts and protected captures
distinct. Unread, outside-scope and stale records stay unknown.
Synthetic fixtures do not admit a new renderer, network request,
sitemap submission, search action, ranking input or native change.

| Feature | Owning requirement and design | Planned fixture and expected evidence |
|---|---|---|
| F-0150 | REQ-0044-W2-WORD-AND-TEXT-RATIO · DES-0044-W2-COVERED-TEXT-METRICS | FX-0044-W2-WORD-AND-TEXT-RATIO · IT-0044-W2-WORD-AND-TEXT-RATIO. Compare full and thin synthetic pages from one confirmed layout using 0026's eligible word boundaries and the admitted 0115 text and source extraction. Show the exact observed numerator, denominator, unit, rule and template scope. A zero or unknown denominator, unread page or unsupported ratio profile yields no ratio. A thin flag needs its own admitted floor. |
| F-0151 | REQ-0044-W2-BOILERPLATE-RATIO · DES-0044-W2-COVERED-TEXT-METRICS | FX-0044-W2-BOILERPLATE-RATIO · UT-RN-23 and MS-RN-02. Within one eligible layout sample, distinguish confirmed repeated navigation and footer regions from unique text and a legitimate shared editorial passage. Name the observed population and coverage. Remove supporting observations and hold unreliable shares. Use RN-27 rather than a new separator. |
| F-0152 | REQ-0044-W2-VISIBLE-PLACEHOLDER · DES-0044-W2-VISIBLE-CONTENT | FX-0044-W2-VISIBLE-PLACEHOLDER · IT-0044-W2-VISIBLE-PLACEHOLDER. Compare an admitted placeholder in observed main content with the same wording in a genuine quotation and an inaccessible region. Show only supported visible findings and safe positions under the admitted profile. Missing visibility holds. Reuse 0115 RN-09 and UT-RN-07. |
| F-0153 | REQ-0044-W2-BROKEN-IMAGE-REFERENCES · DES-0044-W2-OBSERVED-MEDIA | FX-0044-W2-BROKEN-IMAGE-REFERENCES · IT-0044-W2-BROKEN-IMAGE-REFERENCES. Match a proved missing image to exact eligible references on two source pages. Compare a refusal, incomplete coverage and an image updated since observation. Check current source and media revisions before showing supported advice. An unavailable decoder or denied read does not prove breakage. Reuse 0115 RN-34/39. |
| F-0154 | REQ-0044-W2-IMAGE-FILENAME-ADVICE · DES-0044-W2-OBSERVED-MEDIA | FX-0044-W2-IMAGE-FILENAME-ADVICE · IT-0044-W2-IMAGE-FILENAME-ADVICE. Compare a generic camera name and descriptive original against a generated CDN alias and unresolved reference. Report admitted naming evidence only for the known original or exact proven alias under the current rule. Do not turn an arbitrary CDN path into an original name, fetch an address or rename media. |
| F-0155 | REQ-0044-W2-VIEWPORT-DECLARATION · DES-0044-W2-EXACT-DECLARATIONS | FX-0044-W2-VIEWPORT-DECLARATION · IT-0044-W2-VIEWPORT-DECLARATION. Use eligible complete heads with absent, valid and invalid viewport declarations and a partially observed head. Separate absent and invalid facts under the admitted parser. A partial head supplies no proved absence. No claim about computed layout or edit. Reuse F-1320 for head coverage. |
| F-0156 | REQ-0044-W2-LANGUAGE-ENCODING-FACTS · DES-0044-W2-EXACT-DECLARATIONS | FX-0044-W2-LANGUAGE-ENCODING-FACTS · IT-0044-W2-LANGUAGE-ENCODING-FACTS. Compare observed language and encoding declarations with eligible 0026 language evidence and the admitted 0115 decode result. Include a mismatch and an unread header. Keep declaration, decoder outcome and language estimate separate. Missing scope stays unknown. No declaration, hreflang or market change under DR-081. |
| F-0157 | REQ-0044-W2-SOCIAL-CARD-AUDIT · DES-0044-W2-LOCAL-CARD-OBSERVATION | FX-0044-W2-SOCIAL-CARD-AUDIT · IT-0044-W2-SOCIAL-CARD-AUDIT. Use valid and conflicting metadata, a refused image and a foreign host. Paint inert local text and eligible already-loaded own-site media with the scope visible. Mark missing or refused fields explicitly. Reuse 0115 RN-38 and UT-RN-29. No image acquisition, active preview or social provider action. |
| F-0158 | REQ-0044-W2-FEED-AUDIT · DES-0044-W2-BOUNDED-FEED-OBSERVATION | FX-0044-W2-FEED-AUDIT · UT-RN-19 and UT-RN-21. Use stored own-site RSS and Atom records with discoverable valid feeds, malformed or oversized input and incomplete discovery. Separate parsed syntax from discovery evidence and give the exact earlier stop if a cap applies. Stream within RN-21/23, disable external entities and never fetch a feed to prove absence. No syndication or connector action. |
| F-0159 | REQ-0044-W2-GUEST-404-OPTIONS · DES-0044-W2-VISIBLE-CONTENT | FX-0044-W2-GUEST-404-OPTIONS · IT-0044-W2-GUEST-404-OPTIONS. Compare a captured 404 with eligible visible suggestions and a search form, a thin 404 and a supported soft-404 observation. State the actual response code separately from visible options and soft-404 confidence. Missing render facts cannot establish quality. Reuse RN-19/20 and UT-RN-17. Do not submit the form or follow any option. |
| F-0160 | REQ-0044-W2-VIEWPORT-PARITY · DES-0044-W2-PAIRED-RENDER-EVIDENCE | FX-0044-W2-VIEWPORT-PARITY · IT-0044-W2-VIEWPORT-PARITY. Compare matched mobile and independently admitted desktop records for one exact eligible revision with one visible-content gap and one link gap. Use the same admitted scope, engine, revision and comparison method apart from intended device differences. Desktop remains off unless set and counts within the existing budget. Unmatched or partial captures prove no gap. Reuse RN-08/18 and F-0082's capture lineage. |
| F-0161 | REQ-0044-W2-PROTECTED-SCREENSHOTS · DES-0044-W2-PAIRED-RENDER-EVIDENCE | FX-0044-W2-PROTECTED-SCREENSHOTS · UT-RN-09. Model mobile and independently admitted desktop captures under the existing opt-in, engine and cage. The default, insufficient disk, stale or revoked scope and failed renderer yield no released image. A passing fixture binds resolution, exact source revision, device profile and protected ledger reference before release. Keep incomplete scratch private and clean it up on failure. Reuse RN-08/11/32 and vault bounds. |

### Telling search engines about new pages

1. One plain list of public addresses of the site's own pages, sent as
   IndexNow notices, for the owner's own registered sites only. A
   private, staff-only or logged-in address is never listed.
2. The IndexNow daily cap and safe top are readiness blockers owned by the
   crawler notification research record. No notice can be sent until they are
   admitted; at the cap the rest wait for the next day and the screen says so.
3. A notice only says a page exists. It asks for no indexing change and
   edits no page, robots file or sitemap. It is separate from the Search
   Console and Bing readers. This part measures the notices' effect.
4. The owner may upload Google's "crawled, not indexed" list (small
   version).

### Wave 4 crawler findings and local controls (1313 to 1335)

Use the active registered site, site-scoped grants and existing crawl,
render, log and report routes. Bind each view to the exact source,
observation or revision, registered address, coverage and admitted rule
profile. An absent or unqualified source shows the existing unavailable
or not measured state. Do not turn missing facts into a finding, a zero
or a reason to retry outside the existing schedule. Check the current
grant, visibility, expiry and erasure generation through review, copy,
export, replay and restore. Apply 0032 and 0118 to every stored copy and
derived result. Keep any facts eligible for existing ranking checks in
the owning registry and 0027 policy. The findings below add no new
ranking path or public-page code.

| Feature | Exact observation, proposal or control |
|---|---|
| 1313 Sitemap file draft | Use current eligible canonical, indexable guest pages from the registered crawl, removing duplicate addresses and holding stale or ambiguous candidates. Report coverage and exclusions. Validate the final bounded local XML and compare with the observed native sitemap for the same scope. Apply F-0117 and 0115 RN-23 to the native membership comparison. Mark a draft URL missing from the native sitemap only with complete comparable native evidence for that URL. If an unread child leaves its membership uncertain, show it as unknown and keep an independently eligible URL in the local draft. Never submit, replace or claim coverage for unseen pages. Search Console stays read-only under DR-106. |
| 1314 Mobile usability audit | Use the admitted 0115 render level, engine, viewport, platform cage and site cap. Pin the rendered page revision and measure exact visible target, text and overflow features with our qualified rules. Record unknown rendering or unsupported templates as not measured. Static HTML alone cannot prove a rendered dimension. |
| 1315 AMP validation | Require WordPress, current admitted AMP capability and matched same-site guest AMP and canonical revisions. Use a dated parser and eligibility profile. Compare the explicit canonical relation and its eligible target. Missing capability or incomplete evidence holds. Recommend manual review without enabling a plugin, fetching an unknown destination or writing a link. |
| 1316 Structured data validation | Keep one check family for 107, 178 and the 1181 draft preview's eligible rule use. Parse bounded inert source facts. Distinguish syntax, vocabulary and the admitted rich-result profile. Compare visible context where required. An unavailable rule or incomplete visibility withholds that verdict. Validation proves neither search appearance nor indexing. |
| 1317 Owner prompts per page | Bind the owner's approved eligible page set and revisions, bounded prompt, minimum source projection, purpose, admitted model, transport and output profile. A named enrolled MCP process needs separate owner disclosure approval. Send one sealed no-tools request per eligible item. Fence rights and stop at prompt, result, replay and hand-off. Reconcile unknown dispatch before retry. Treat page text as data. Allow an admitted local model or separately permitted MCP scope only under 0020, 0042 and 0091. Keep raw archived copies and private content on the PC, reject disallowed disclosure, and apply the existing AI stop at queue, prompt and result. No fallback, tool effect or promotion of generated wording to measured fact. |
| 1318 Signed-in test crawl | Keep the source feature identifiable. The current 0044 and 0082 crawl admits guest reads only. Refuse a test-account session, credentials or authenticated follow-up in the desktop, local tool, queue, retry and restore. A protected address cannot become a crawl result, archive, export, index or link target through this route. A separate signed-in contract would require the owner's decision and explicit scope and isolation. |
| 1319 Command-line crawls | Use spec 0020's local authenticated command entry and 0044's existing start, stored comparison and admitted local export. Check the same site, actor, rate, job and export bounds without a desktop window. Preserve the existing start identity on retry. No alternate fetcher, bearer disclosure, remote listener or settings confirmation path. |
| 1320 Head HTML validity | Use the admitted bounded source parser, completed coverage and response revision. Report malformed head structure with safe evidence. Keep source markup and effective parsed title, canonical and robots observations distinguishable. A parser repair does not erase the source finding or authorise an edit. |
| 1321 Page carbon estimate | Use an admitted openly licensed published method and exact version, named on the report. State transfer scope, weight basis, assumptions, time and uncertainty. Different cache and resource coverage must remain visible. No estimate from insufficient bytes and no actual emissions measurement, compliance claim or vendor rating. |
| 1322 Off-topic page finder | Use eligible same-site public material and the existing 0047 section mapping. Pin page and section revisions, approved embedding model, centre construction profile, sample coverage and threshold. Keep distant matches as review findings with context. Missing or sparse section evidence withholds the finding. No cross-site comparison, automatic recategorisation or ranking exception. |
| 1323 Crawl settings file | Use the existing settings registry and admitted inert file profile. Export permitted crawl keys and schema revisions only under 0014, 0030, 0032 and 0118, to the approved recipient. Pin retention, destination and manifest. Import proposes a merge of named settings against the active site generation. Hold stale generation and commit exact approved changes and receipt together. Refuse wrong types, unknown or protected keys, credential fields, and unsafe or mismatched destinations as applicable. Only the existing owner confirmation path can apply an eligible plan. Files and replay cannot confirm changes. |
| 1324 Tunable issue rules | Keep changes to the exact admitted issue catalogue, site and threshold or priority schema. Show defaults, scope and effects in an owner-confirmed diff. An advisory rule can be tuned only within its admitted range. Protected checks and hard crawl, disclosure and ranking limits cannot be switched off or weakened. The old Rhai reference does not authorise code execution. User scripts, where admitted elsewhere, follow ADR-0021 and 0104. |
| 1325 Adaptive crawl pace | Reuse 0082's one host and site admission and slowing rule. Bind measured latency, errors, cooldown and current limits. Slow or pause under the existing thresholds and recover only within current ceilings and the admitted recovery profile. No missing or stale measurement may increase capacity. Restart, replay or another entry point cannot bypass a pause or budget. |
| 1326 Checks by page importance | Use one admitted 0066 job 324 priority and cadence profile. Pin the exact need inputs and eligible within-site search share as scheduler advice. Persist next due-time and bounded recurrence per group. Preserve baseline slots, a qualified maximum wait outcome and fair turn-taking per host. Pause, replay and restore preserve caps and due-work identity. Keep input provenance, due times and coverage. An unknown priority uses the admitted ordinary profile. Preserve fairness and existing caps, off-peak rules and adaptive pace. Feature 15 consumes this one schedule contract. Priority cannot add crawl entitlement. |
| 1327 Page change history | Keep the single dated own-site observation history used by feature 81 and other consumers. Pin page identity, capture time, fetch status, source profile and revision. Compare title, headings, content fingerprint, canonical and status only across eligible comparable observations. Use the current approved observation projection. Keep page copy only under admitted byte and retention caps. When the exact original is absent show the fingerprint or changed flag without reconstructing text. Seal immutable comparable diffs under 0032 and 0118. Missing coverage or a temporary failure cannot prove removal. Withdraw managed history or copies when current rights require it. |
| 1328 CDN log streams | Use existing authorised local access-log sources or bounded owner-supplied CDN records admitted under QR-088 and 0100. Bind approved own-source evidence under QR-088 and 0116, retention, source epoch, exact field projection, cursor and live approval. Compare at open and commit cursor with the batch atomically. No identity rejoin. Stop discards uncommitted work. Raw addresses, credentials and unapproved fields never enter views. DR-076 admits no direct CDN account, provider API or live feed. |
| 1329 Counter coverage check | Compare an exact supported template and guest observation with eligible 0034 counter presence and admitted observation evidence. Separate structural presence, eligible counted observations, partial scope and missing proof. Opt-out or blocked measurement is not proof of a broken counter. Add no probe event, tag, counter script or new footprint. |
| 1330 IndexNow pings | Use solely the existing own-site public IndexNow notice contract. Bind registered public host, current canonical address, revision and event, verified owner and approved endpoint, trigger, daily cap and safe top before dispatch. Fence removal, privacy, grant or verification loss. Persist quota and receipt with exact event identity. Reconcile ambiguous submission without a duplicate. Show wait at cap, received or verification pending separately. The feature never places a host key file or enables a connector. Do not send removed or private addresses under the current page-exists contract or claim an indexing result. This route grants no Search Console or Bing write. |
| 1331 Browser page check | Show existing own-site observations for the exact current eligible address in the owner-only app. Use an owner-selected or supplied address without inherited browser credentials. Any closer browser hand-off needs its own admitted local profile. No browser extension permission, page script, remote listener, active download or browser-history intake is granted. Stale or private address evidence cannot produce a current report. |
| 1332 Journey stage labels | Extend the current eligible feature 71 intent evidence with separately qualified content or query stage, evidence and rule revision. Report different or mixed intent and stage distinctly. Unknown stays unknown. Stage describes the item, with no identification or profiling of an actual reader. No market, provider or ranking change. |
| 1333 Search funnel per section | Pin an eligible canonical page base, exact historic section, event-stage definitions and proved interval ordering. Use 0034/0040 compatible bot and human scope. Require joint coverage before contingent conversion. Report observed crawled, eligible to index, verified crawler-visited and measured organic visit stages on compatible coverage. Require producer evidence for any nested rates. Show denominators, non-overlap or partial scope. Never use clicks as visits, unknown coverage as zero or a known share as proof of indexing. |
| 1334 Bot crawl by device and file type | Reuse approved 0034 and 0044 log facts. Separate verified, claimed and unknown crawler identities and admitted device and file classes. Show source scope, valid timing source, sample denominator, units and coverage for latency. Do not infer device from a name or an absent field, expose raw addresses or use counts as a ranking input. |
| 1335 Agent-readable site guide | Draft a bounded inert local llms.txt and plain map from currently eligible public canonical hubs on the active site. Attribute evidence and hold private, unproved or stale targets. Review exact text and links. The proposal is a single version for every visitor. It adds no agent-specific delivery, network fetch, upload, indexing authority or auto-publication. |

QR-196 refines exact profiles, rights checks, bounds and planted refusals.
Feature 1318 and the removal request in feature 1330 need an owner
scope decision and decision record before admission. Keep the exact
unavailable route visible. IndexNow supports deletion notices. The
current own-site page-exists contract limits Sutradhar's submission
scope. A received or key-pending reply carries no indexing verdict.
[IndexNow protocol](https://www.indexnow.org/documentation)

#### Wave 4 feature verification

These owner bindings retain the exact observation, scope and refusal
rules above under spec 0076. They use the existing source, permission,
erasure, evidence custody and availability conditions.

| Owning design | Applicable variants and evidence |
| --- | --- |
| DES-0044-W4-OBSERVE | VAR-0044-W4-SOURCE-PROJECTION uses the admitted current own-site facts for the exact named observation. VAR-0044-W4-MOBILE-RENDER and VAR-0044-W4-WP-AMP require their separate rendering or native capability proof. |
| DES-0044-W4-ARTEFACT | VAR-0044-W4-LOCAL-DRAFT produces only the exact bounded inert local file and review projection. |
| DES-0044-W4-ADVICE | VAR-0044-W4-LOCAL-MODEL uses the current admitted local route. VAR-0044-W4-ENROLLED-MCP needs independent enrolment, owner disclosure and protected transport for the exact projection. |
| DES-0044-W4-CONTROL | VAR-0044-W4-GUEST-CRAWL retains the 0082 guest fences and host budgets. VAR-0044-W4-LOCAL-COMMAND uses the existing authenticated local entry. VAR-0044-W4-ADMITTED-PRIORITY uses only the admitted job 324 profile and common due-work identity. |
| DES-0044-W4-SETTINGS | VAR-0044-W4-INERT-SETTINGS and VAR-0044-W4-ADVISORY-RULE use the existing registry and live owner confirmation. They do not admit new keys or rule changes. |
| DES-0044-W4-LOG | VAR-0044-W4-LOCAL-LOG uses the approved local log or supplied-file profile. QR-088 and current source authority still apply. |
| DES-0044-W4-NOTICE | VAR-0044-W4-CURRENT-PUBLIC-NOTICE is the existing eligible page-exists route only. It needs a qualified profile, trigger, daily cap and safe top before any actual dispatch. |

| Feature | Requirement, design and variants | Named verification and expected result |
| --- | --- | --- |
| F-1313 | REQ-0044-W4-SITEMAP-DRAFT, DES-0044-W4-ARTEFACT, VAR-0044-W4-LOCAL-DRAFT | [FX-0044-W4-SITEMAP-DRAFT](../planning/feature-wave-4.md#acceptance-design-for-features-1313-to-1337) · BDD-0044-W4-SITEMAP-DRAFT. Give a registered guest-crawl snapshot with canonical, duplicate, redirected, noindex and stale rows. Emit bounded valid local XML for eligible known URLs and a scoped comparison to the observed native sitemap. Keep exclusions and coverage. Compare the same eligible draft URL absent from the visible native entries in two fixtures. Complete native scope can prove absence. An unread child that could cover the URL leaves native membership unknown without removing the eligible draft URL. Reuse the F-0117 and RN-23 coverage boundary. A submit or replace request gives no site write under DR-106. |
| F-1314 | REQ-0044-W4-MOBILE, DES-0044-W4-OBSERVE, VAR-0044-W4-MOBILE-RENDER | [FX-0044-W4-MOBILE](../planning/feature-wave-4.md#acceptance-design-for-features-1313-to-1337) · IT-0044-W4-MOBILE. Use one eligible rendered revision with small targets and overflow and the pinned mobile viewport. Repeat the admitted cage, budget and revision proof per platform under 0115 RN-01, RN-05 and RN-08. Raw-only HTML, unproved viewport, missing cage or an unadmitted cap leaves rendered dimensions not measured. |
| F-1315 | REQ-0044-W4-AMP, DES-0044-W4-OBSERVE, VAR-0044-W4-WP-AMP | [FX-0044-W4-AMP](../planning/feature-wave-4.md#acceptance-design-for-features-1313-to-1337) · BDD-0044-W4-AMP. Use an eligible public WordPress AMP/main pair and the admitted dated rule, then change canonical scope or the main revision. A disabled WordPress capability or a XenForo site shows unavailable. No unknown target is fetched and no plugin or link changes. |
| F-1316 | REQ-0044-W4-STRUCTURED-DATA, DES-0044-W4-OBSERVE, VAR-0044-W4-SOURCE-PROJECTION | [FX-0044-W4-STRUCTURED-DATA](../planning/feature-wave-4.md#acceptance-design-for-features-1313-to-1337) · UT-0044-W4-STRUCTURED-DATA. Give bounded malformed syntax, valid vocabulary with a failed rich-result rule, mismatched required visible context and an unavailable profile. Syntax, vocabulary and rich-result outcomes stay distinct. Keep rule reuse for features 107, 178 and 1181. No indexing or search appearance claim. |
| F-1317 | REQ-0044-W4-PAGE-PROMPT, DES-0044-W4-ADVICE, VAR-0044-W4-LOCAL-MODEL, VAR-0044-W4-ENROLLED-MCP | [FX-0044-W4-PAGE-PROMPT](../planning/feature-wave-4.md#acceptance-design-for-features-1313-to-1337) · IT-0044-W4-PAGE-PROMPT. Use an eligible synthetic local model request and separately an enrolled MCP process with exact owner-approved disclosure for the pinned page revision. Check one sealed request without tools per eligible item. Inspect the MCP input for only its permitted minimal projection, with no raw archive or private text. Treat a harmless planted instruction as data. Withheld disclosure, stale revision or revoked scope before prompt yields no dispatch. Lose the response to a synthetic attempt and retry before reconciliation. Observe zero repeat dispatch while its state is unknown. Deliver a late response after Stop and reject it on result, replay and hand-off. Keep eligible wording as advisory evidence only. No outside fallback, tool effect or measured-fact claim. |
| F-1318 | REQ-0044-W4-GUEST-FENCE, DES-0044-W4-CONTROL, VAR-0044-W4-GUEST-CRAWL | [FX-0044-W4-GUEST-FENCE](../planning/feature-wave-4.md#acceptance-design-for-features-1313-to-1337) · IT-0044-W4-GUEST-FENCE. Offer a signed-in request through desktop, local tool, queue, retry and old restore, then an independently eligible guest request. Refuse the signed-in request without reading a private page or creating results, archives, exports, index entries or link candidates. Preserve existing 0082 guest admission. |
| F-1319 | REQ-0044-W4-LOCAL-COMMAND, DES-0044-W4-CONTROL, VAR-0044-W4-LOCAL-COMMAND | [FX-0044-W4-LOCAL-COMMAND](../planning/feature-wave-4.md#acceptance-design-for-features-1313-to-1337) · IT-0044-W4-LOCAL-COMMAND. Use the existing 0020 authenticated local command while the window is closed. Start once, compare stored results and request an admitted local export. Replay the start and try wrong site, exhausted rate, a remote listener and scripted confirmation. Preserve existing local authority and job identity. |
| F-1320 | REQ-0044-W4-HEAD-SOURCE, DES-0044-W4-OBSERVE, VAR-0044-W4-SOURCE-PROJECTION | [FX-0044-W4-HEAD-SOURCE](../planning/feature-wave-4.md#acceptance-design-for-features-1313-to-1337) · UT-0044-W4-HEAD-SOURCE. Compare a valid complete head, misplaced body markup and parser recovery on the same response revision. Keep the source finding separate from effective title, canonical and robots facts. A truncated head cannot prove that metadata is absent. No edit follows. |
| F-1321 | REQ-0044-W4-CARBON, DES-0044-W4-OBSERVE, VAR-0044-W4-SOURCE-PROJECTION | [FX-0044-W4-CARBON](../planning/feature-wave-4.md#acceptance-design-for-features-1313-to-1337) · BDD-0044-W4-CARBON. With an admitted dated method, compare measured transferred bytes on compatible cache and resource bases. Display the exact assumptions, uncertainty and labelled estimate. Missing scope or method, incomparable cache basis or a vendor score alone cannot establish it. No measured-emissions or compliance claim. |
| F-1322 | REQ-0044-W4-OFF-TOPIC, DES-0044-W4-OBSERVE, VAR-0044-W4-SOURCE-PROJECTION | [FX-0044-W4-OFF-TOPIC](../planning/feature-wave-4.md#acceptance-design-for-features-1313-to-1337) · BDD-0044-W4-OFF-TOPIC. Compare a same-site page and admitted section revision with eligible distant material, sparse evidence and a moved page with stale membership. Show a scoped review finding only with the admitted model, centre profile and threshold. No recategorisation, ranking exception or cross-site comparison. |
| F-1323 | REQ-0044-W4-SETTINGS-FILE, DES-0044-W4-SETTINGS, VAR-0044-W4-INERT-SETTINGS | [FX-0044-W4-SETTINGS-FILE](../planning/feature-wave-4.md#acceptance-design-for-features-1313-to-1337) · IT-0044-W4-SETTINGS-FILE. Preview one exact eligible settings file and resulting scope. Refuse credentials, unknown or protected keys, wrong types, stale merge and changed destination. A file or replay cannot confirm. Only the existing live owner route can commit the authorised diff and receipt together. |
| F-1324 | REQ-0044-W4-ISSUE-RULE, DES-0044-W4-SETTINGS, VAR-0044-W4-ADVISORY-RULE | [FX-0044-W4-ISSUE-RULE](../planning/feature-wave-4.md#acceptance-design-for-features-1313-to-1337) · BDD-0044-W4-ISSUE-RULE. Give one permitted advisory priority change with the existing owner diff. Try a protected check disable, an out-of-range threshold and legacy script text. Allow only the exact admitted confirmed priority change. No script execution or protected-rule change. |
| F-1325 | REQ-0044-W4-PACE, DES-0044-W4-CONTROL, VAR-0044-W4-GUEST-CRAWL | [FX-0044-W4-PACE](../planning/feature-wave-4.md#acceptance-design-for-features-1313-to-1337) · IT-0044-W4-PACE. After qualifying latency or errors apply the existing 0082 slowing or pause to the exact host/site scope. Replay with lost measures and a competing crawl. Recovery remains within the current admitted ceilings and shared budget. Stale measurements cannot raise capacity. |
| F-1326 | REQ-0044-W4-PRIORITY, DES-0044-W4-CONTROL, VAR-0044-W4-ADMITTED-PRIORITY | [FX-0044-W4-PRIORITY](../planning/feature-wave-4.md#acceptance-design-for-features-1313-to-1337) · IT-0044-W4-PRIORITY. Compare admitted higher and ordinary priority, stale evidence, deferred ordinary work and competing same-host work after pause or restore. Preserve eligible due identity and baseline fair turns within the existing caps. Unknown priority uses ordinary admission. Feature 15 gains no additional crawl entitlement. |
| F-1327 | REQ-0044-W4-HISTORY, DES-0044-W4-OBSERVE, VAR-0044-W4-SOURCE-PROJECTION | [FX-0044-W4-HISTORY](../planning/feature-wave-4.md#acceptance-design-for-features-1313-to-1337) · BDD-0044-W4-HISTORY. Compare dated, comparable revisions including a temporary error and changed address. Use a fingerprint when the permitted original is absent. Missing coverage cannot prove removal. Revoked copies and erased older observations stay withheld on replay and restore. Reuse feature 81 history. |
| F-1328 | REQ-0044-W4-CDN-LOG, DES-0044-W4-LOG, VAR-0044-W4-LOCAL-LOG | [FX-0044-W4-CDN-LOG](../planning/feature-wave-4.md#acceptance-design-for-features-1313-to-1337) · IT-0044-W4-CDN-LOG. Provide eligible local access-log or owner-supplied CDN fixtures with planted unapproved fields. Race grant expiry and stop against batch/cursor commit. Reuse AT-SEO-LOG with the exact source epoch. Keep one current scrubbed batch or hold. Attempts to use a direct account, named API, live stream or identity rejoin are refused. |
| F-1329 | REQ-0044-W4-COUNTER-COVERAGE, DES-0044-W4-OBSERVE, VAR-0044-W4-SOURCE-PROJECTION | [FX-0044-W4-COUNTER-COVERAGE](../planning/feature-wave-4.md#acceptance-design-for-features-1313-to-1337) · BDD-0044-W4-COUNTER-COVERAGE. Give compatible own-site template evidence with an eligible counted observation, observed presence only, opt-out, missing proof and a proved mismatch. Keep all states distinct. A withheld measurement cannot establish a broken counter. No extra probe, script, tag or public work. |
| F-1330 | REQ-0044-W4-INDEXNOW, DES-0044-W4-NOTICE, VAR-0044-W4-CURRENT-PUBLIC-NOTICE | [FX-0044-W4-INDEXNOW](../planning/feature-wave-4.md#acceptance-design-for-features-1313-to-1337) · IT-0044-W4-INDEXNOW. Use a synthetic endpoint and independently admitted current-public notice profile, binding owner, registered host, exact canonical revision, event and endpoint. Remove daily-cap or safe-top admission in separate runs and observe zero sends. At the admitted cap send nothing and report wait for the next day. In a separate eligible run record one synthetic dispatch, lose its reply and replay the same identity. Reconcile the original quota and receipt without repeating the notice while its outcome is unknown. Distinguish received and verification pending without an indexing verdict. Revoke visibility or authority after queueing and hold the event. Refuse removed or private addresses, key setup and Search Console or Bing writes. |
| F-1331 | REQ-0044-W4-PAGE-EVIDENCE, DES-0044-W4-OBSERVE, VAR-0044-W4-SOURCE-PROJECTION | [FX-0044-W4-PAGE-EVIDENCE](../planning/feature-wave-4.md#acceptance-design-for-features-1313-to-1337) · BDD-0044-W4-PAGE-EVIDENCE. Read the exact existing current-public own-site observation through the owner app. Try an ineligible or stale address, browser-held cookie, extension or remote hand-off. Refuse the new route and withheld evidence. Owner address selection grants no new capture or browser authority. |
| F-1332 | REQ-0044-W4-JOURNEY-STAGE, DES-0044-W4-OBSERVE, VAR-0044-W4-SOURCE-PROJECTION | [FX-0044-W4-JOURNEY-STAGE](../planning/feature-wave-4.md#acceptance-design-for-features-1313-to-1337) · BDD-0044-W4-JOURNEY-STAGE. Compare admitted learn and compare stage evidence on current items, conflicting intent and stale labels. Preserve separate evidence, mixed stages and unknowns. A reader history cannot enter the classifier. No new provider, audience or ranking effect. |
| F-1333 | REQ-0044-W4-SEARCH-FUNNEL, DES-0044-W4-OBSERVE, VAR-0044-W4-SOURCE-PROJECTION | [FX-0044-W4-SEARCH-FUNNEL](../planning/feature-wave-4.md#acceptance-design-for-features-1313-to-1337) · IT-0044-W4-SEARCH-FUNNEL. Compare actual compatible nested stages for one historic section, matched window and canonical page base. Try mixed intervals, non-nested coverage, changed section and provider clicks. Show exact supported denominators only. Missing evidence or clicks cannot stand in for visits, coverage or indexing proof. |
| F-1334 | REQ-0044-W4-BOT-LOG, DES-0044-W4-LOG, VAR-0044-W4-LOCAL-LOG | [FX-0044-W4-BOT-LOG](../planning/feature-wave-4.md#acceptance-design-for-features-1313-to-1337) · BDD-0044-W4-BOT-LOG. Use current approved log projections with verified, merely claimed and unknown crawler evidence. Compare known and unknown device/file classes, distinct latency units and absent timing coverage. Show only supported scoped aggregates and timing with units and denominator. No address disclosure or inferred device. |
| F-1335 | REQ-0044-W4-SITE-GUIDE, DES-0044-W4-ARTEFACT, VAR-0044-W4-LOCAL-DRAFT | [FX-0044-W4-SITE-GUIDE](../planning/feature-wave-4.md#acceptance-design-for-features-1313-to-1337) · BDD-0044-W4-SITE-GUIDE. Supply current public canonical hubs, private or stale candidates and planted unsafe markup. Produce one bounded inert local guide and plain map for review. Refuse upload and alternate agent-only versions. No network request, publication, indexing or extra delivery authority. |

QR-196 and the named dependencies still own the exact profiles,
authority, fixtures and observed evidence. A refusal can be verified
while a positive route stays unavailable. The signed-in crawl, removed
page notice and direct CDN live feed remain held. Rendering, native AMP,
MCP, estimates and other conditional routes use their own exact admission
proof. No fixture admits a live call or replaces the required records.
Bind schema, limits, permissions and observed proof before task use.

### Wave 2 passive technical and privacy observations

Use stored guest evidence for the exact registered site, eligible
origin, capture, revision, rule and coverage. Any owner-supplied
evidence must fit an already admitted scrubbed input profile.
Unknown, refused, stale or incompatible facts stay unknown. Use
synthetic fixtures only. No report starts a request, tries an attack,
changes production settings or enables a search connector.

#### Certificate expiry evidence and recurrence

Use the 0082 `PeerLeafExpiry` record for the exact registered own
HTTPS origin. Recheck its source and custody, current grant, allowed
origin and admitted trust profile before evaluating, publishing or
reading a current finding. Require the latest
authoritative observation under the admitted source ordering and
freshness rule. A missing clock, future observation, stale
observation or superseding refusal holds the current alert even
when an older receipt still verifies. When the validity end
arrives, hold the approaching-expiry alert. Keep dated evidence
only while authorised. Scope withdrawal
removes the live listing, detail, count and pending publication under
0032 and 0099. A renewed grant does not revive a prior queued alert.

QR-110 must qualify the exact time comparison for the existing
14-day rule. The proposed test profile uses the verified observation
instant. A positive difference up to and including 14 elapsed UTC
days qualifies. The first representable instant beyond that limit
does not. This compares time remaining for the alert. The
[certificate validity period][leaf-validity-standard] includes its
end instant under the peer validation profile. A proof failure
or zero remaining time supplies no approaching-expiry alert.
A later eligible capture can enter the window.
Replay, restore and refresh of a view use the original observation
instant and cannot create a fresh observation.

Within the existing 0032 grouping, use site ID, exact origin, leaf
digest and the applicable threshold rule as the one expiry-alert
identity. Multiple requests or replay of the same eligible leaf and
threshold produce one logical alert. Honour existing cooldown and
current alert routing. A newly observed eligible leaf gets its own
identity and can alert only if it qualifies. A single observed leaf
cannot certify renewal for every connection to the origin. Display
the origin, observed expiry and observation date under eligible
access. With valid viewing rights but no current usable evidence,
say “We could not verify this site's current certificate expiry.”
This uses no new send route or site-side work.

[leaf-validity-standard]: https://www.rfc-editor.org/rfc/rfc5280.html#section-4.1.2.5

| Feature | Owning requirement and design | Planned fixture and expected evidence |
|---|---|---|
| F-0162 | REQ-0044-W2-BLOCKED-RESOURCES · DES-0044-W2-SCOPED-RENDER-FACTS | FX-0044-W2-BLOCKED-RESOURCES · IT-0044-W2-BLOCKED-RESOURCES. Compare stored broker decisions about a denied script and stylesheet with one eligible observed visible difference and one partial render. Show safe refusal category and observed difference separately. Reuse 0115 RN-03/05/34 and UT-RN-27. Partial data cannot prove which resource caused a change. Never bypass the policy or retry for a result. |
| F-0163 | REQ-0044-W2-CACHE-ENCODING-AUDIT · DES-0044-W2-OBSERVED-RESPONSE-PROFILE | FX-0044-W2-CACHE-ENCODING-AUDIT · IT-0044-W2-CACHE-ENCODING-AUDIT. At the first synthetic capture measure 200 encoded and 800 decoded body bytes. A later eligible 304 for the same retained representation keeps those counts at the first capture time and records the later status separately. Compare a fresh identity response, the same URL and validator with a different negotiated representation, an erased body, a withdrawn read and a stopped decoder. Never show zero body bytes or a fresh size from the 304. Reuse 0082 N-1648 and RN-21/40 and UT-RN-19. No request or live configuration change. |
| F-0164 | REQ-0044-W2-TLS-EXPIRY · DES-0044-W2-ELIGIBLE-CERTIFICATE-FACTS | FX-0044-W2-TLS-EXPIRY · IT-0044-W2-TLS-EXPIRY. Join the 0082 N-1836 peer fixture through the qualified 14-day comparison and existing 0032/0099 alert. A synthetic intermediate expires first and a second origin shares a valid leaf. Neither provides expiry evidence for the requested origin. Try an authentic receipt after freshness ends, after a newer refusal and at the validity end. None supplies a current alert. Replay a qualifying same-origin leaf through two captures and restart, then observe a different qualifying leaf. Keep one logical alert per eligible leaf and threshold. Withhold a newer refused or no-longer-trusted peer and prove that withdrawing read scope removes current rows, details, counts and pending publication. Starts no probe, trust fallback or new send. |
| F-0165 | REQ-0044-W2-SECURITY-HEADER-OBSERVATIONS · DES-0044-W2-OBSERVED-RESPONSE-PROFILE | FX-0044-W2-SECURITY-HEADER-OBSERVATIONS · IT-0044-W2-SECURITY-HEADER-OBSERVATIONS. Compare synthetic complete eligible responses with presence, absence and conflict under the admitted header profile. Withhold findings for a foreign origin or incomplete response. State only observed facts, applicable policy and manual owner guidance. No configuration edit or claim of compliance. |
| F-0166 | REQ-0044-W2-REDIRECT-SCOPE-REVIEW · DES-0044-W2-SUPPLIED-PROTECTED-FACTS | FX-0044-W2-REDIRECT-SCOPE-REVIEW · IT-0044-W2-REDIRECT-SCOPE-REVIEW. Compare a supplied scrubbed destination-category finding against an admitted current own-site routing profile and matching dated passive evidence. Include a scope mismatch, stale policy and unknown category. Expose only the potential scope defect and evidence for staff review. No crafted address, navigation, active replay, vulnerability verification or repair. DR-109 still governs F-0128 separately. |
| F-0167 | REQ-0044-W2-VERSION-DISCLOSURE · DES-0044-W2-SUPPLIED-PROTECTED-FACTS | FX-0044-W2-VERSION-DISCLOSURE · IT-0044-W2-VERSION-DISCLOSURE. Match an approved scrubbed public disclosure observation to current approved version evidence where available. Distinguish disclosure, a supported mismatch and an unavailable comparison. Drop unapproved raw evidence. Make no exploitability claim or software change. |
| F-0168 | REQ-0044-W2-DIRECTORY-LISTING-OBSERVATION · DES-0044-W2-SUPPLIED-PROTECTED-FACTS | FX-0044-W2-DIRECTORY-LISTING-OBSERVATION · IT-0044-W2-DIRECTORY-LISTING-OBSERVATION. Compare stored eligible evidence of a directory index with a similar normal page and unread evidence. Report an admitted listing finding scoped to a safe page reference. No guessed address, copied listing, collection of entries or attempt to open one. |
| F-0169 | REQ-0044-W2-FILE-EXPOSURE-OBSERVATION · DES-0044-W2-SUPPLIED-PROTECTED-FACTS | FX-0044-W2-FILE-EXPOSURE-OBSERVATION · IT-0044-W2-FILE-EXPOSURE-OBSERVATION. Give synthetic owner-supplied exposure categories under an already admitted scrubbed 0032/0118 profile, an ambiguous report and surplus planted secret fields. Keep only eligible category and remediation context. Refuse unadmitted input and show no raw path, name, secret or content. No attempt to locate or retrieve a file. |
| F-0170 | REQ-0044-W2-EMAIL-EXPOSURE-COUNTS · DES-0044-W2-SUPPLIED-PROTECTED-FACTS | FX-0044-W2-EMAIL-EXPOSURE-COUNTS · IT-0044-W2-EMAIL-EXPOSURE-COUNTS. Compare an eligible synthetic visible address, an already scrubbed observation and incomplete page coverage. Check the 0082 ingress redaction and safe aggregate count, category and page scope only. No address in runtime storage, logs, report, copy or export. Incomplete coverage supplies no zero. |
| F-0171 | REQ-0044-W2-COOKIE-OBSERVATION · DES-0044-W2-FINAL-GUEST-COOKIE-FACTS | FX-0044-W2-COOKIE-OBSERVATION · IT-0044-W2-COOKIE-OBSERVATION. Under a synthetic QR-089-admitted header profile, compare an eligible final main same-origin response, a complete no-field response and redacted or malformed attributes, plus a cached response, 304, redirect and subresource. Distinguish complete absence, no new observation and partial facts. Show only existing allow-listed scrubbed facts, never raw values or jar state. Without exact QR-089 admission collect nothing. |
| F-0172 | REQ-0044-W2-AXE-AUDIT · DES-0044-W2-SCOPED-RENDER-FACTS | FX-0044-W2-AXE-AUDIT · IT-0044-W2-AXE-AUDIT. Use synthetic admitted checker and cage capabilities for supported rule IDs, current guest page and exact render coverage. Missing package pin, licence proof, cage, resource admission or profile withholds that route. Only compatible own-site traffic can order existing findings. Missing traffic does not erase a supported rule finding. Reuse RN-12, without substituting it for independent axe-core admission. No conformance or public change. |
| F-0173 | REQ-0044-W2-SCRIPT-INVENTORY · DES-0044-W2-SCOPED-RENDER-FACTS | FX-0044-W2-SCRIPT-INVENTORY · IT-0044-W2-SCRIPT-INVENTORY. Give a stored same-origin script, approved third-party script observation, duplicated delivery and refused origin. Report one identity per observed script with separate safe origin class and coverage. A refused dependency gives no loaded-script result. Reuse 0115 RN-03/16 and UT-RN-14. No acquisition for inventory alone. |

#### Cache revalidation and compression evidence

Under QR-110, use 0082 N-1648's stored validation proof for the same
site, origin, guest request profile, retained representation, body
revision and validator, with current scope and rights. A same-address
response or a repeated validator string alone cannot bind a body.
Use the existing bounded response and decoder evidence only.

Keep the time and status of the conditional exchange separate from
the original body observation. For an exact eligible 304 match, show
the earlier measured encoded and decoded body bytes with their
original time. Label any separately measured 304 exchange bytes as
that exchange, with its own units. A 304 supplies no new measured
body or compression result. A missing, mismatched or withdrawn
body proof gives no reused body counts or successful revalidation
finding. Where admission permits, show the observed response status
on its own. A capped decoder gives no complete decoded size. Keep
the body count unknown in those cases. This report uses no new fetch,
fallback, cache write or raw validator disclosure. F-0333's separate
edge-cache proof remains confined to 0047.

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
- `seo_structure`, layout groups and structural alternate facts
  belong to spec 0047. Its market and business-match records
  are after version 1 under DR-081.
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

- Log import (AT-SEO-LOG): the app shows all choices and gives a
  preview of the planned field names before confirmation. It reads no
  source lines. Missing approval leaves the source unopened. Change
  each choice, stop during a batch and fail the approval lookup.
  Check that the stale batch and cursor are never committed. Check
  consent and opt-outs against AT-AN38 of spec 0034.

### Edge cases to cover

Friendly addresses off (`index.php?threads/...`); the board in a
subfolder; old domains and CDN hosts; percent-encoded slugs; `/goto/post`
links; maintenance mode answering 503; a robots.txt that changes during a
crawl; sitemap indexes of many files; gzip sitemaps; hreflang with an
unobserved alternate; very large threads with hundreds of pages.

### Build-blocking external-contract evidence

The crawler external-contract record must pin and review Google's and
Microsoft's published crawler address lists, whether the Search Console API
has links data, Safe Browsing's commercial-use terms and any Web Risk route,
CrUX and PageSpeed quotas, and the current documented robots.txt limit. Each
dependent integration is blocked until official sources, dates, licence or
terms review and compatibility fixtures are admitted; the existing 500 KiB
implementation bound remains conservative and is not represented as a newly
verified external fact.

## Limits

- Crawling runs one fetch at a time with a streaming parse, at most
  96 MiB above idle. The frontier stays in PostgreSQL, never in memory.
  Report building uses at most 64 MiB. All within the service slice.
- The render check uses the admitted OS cage with its own memory
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
nothing to the pages themselves. Any additional per-page time, query or
byte budget is a readiness blocker owned by the crawler footprint research
record. The affected crawl route cannot build or run until that record
supplies measured limits and fixtures; no unlimited budget is implied.

### Portfolio summary read

Return the completion time and result of the latest terminal own-site
Audit, including failure or cancellation. Do not count in-flight jobs
or Receipts reads. Where there has been no terminal Audit, return no
observation. Use the report or failure record's own completion time.

Expose one bounded, site-scoped summary read under spec 0102. Its
source and failure cases run in IT-SS-06 with the owning module.

### F-0082 script visibility evidence binding

REQ-0044-W1-SCRIPT-VISIBILITY and DES-0044-W1-SAME-RENDER-DIFF
use FX-0044-W1-SCRIPT-VISIBILITY and the existing 0115 RN-18,
UT-RN-16 and BD-RN-02 cases. Bind the initial and completed
views of the exact own-site, guest-visible revision, URL, Audit
profile and admitted capture lineage. Compare a changed title,
inserted link and hidden content with a partial render. Show
only the proved differences and the relevant capture stage.
Missing support stays unknown; an observed difference neither
proves indexing treatment nor makes a script effect causal.
The existing EX-02 and 0115 renderer and broker gates apply.
No separate headless path, additional read or effect follows.

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
- Screens: a desktop app test walks every report screen and finds no
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
- AT-SEO-COOKIE: separate two fields with the same cookie name and
  different validated paths. Check an Expires value containing a
  comma, a secret value, a malformed field, unsafe names and paths,
  a partial response, a 304, a redirect and a subresource. Confirm
  no added request, no sent cookie and erasure with the report.
  Check the archive and replay scrub in IT-LS-01 (spec 0116).
- Render check: uses the declared render identity, stays within its
  admitted daily and memory limits and stops when the site slows.
  Without a cap, admitted engine or platform cage, show "not measured"
  and start no render worker. Check the raw fallback separately.
- Benchmark: one page's HTML checks within 2 ms, under the 5% median
  rule.
- Proofs and depth follow ADR-0020. Only admitted strict-core components
  require 100% line, branch and MC/DC coverage and zero surviving killable
  mutants, with equivalent mutants proved and independently reviewed;
  other code follows the owning risk matrix. The address classifier,
  redirect re-check, guest-only fetch, "no apply path" rule, slowing rule,
  retention and registry check retain their assigned mutation gates and
  property tests. Kani proves the classifier refuses every private,
  loopback and link-local range of IPv4 and IPv6. robotstxt parsing is fuzz
  tested.

## Wave 1 rich results and public metadata bindings

Use current eligible own-site guest-visible evidence and the exact
native revision. Compare before and after only in the admitted
profile. Validate approved shape, rights and current approval.
These bindings supply local review or refusal. Any native effect
requires the distinct approved 0021 adapter, precondition and
receipt, plus the exact measured public footprint under QR-108.
Missing or withdrawn proof adds no public work or metadata.

| Feature and owning design | Exact planned fixture and boundary |
|---|---|
| F-0107 · REQ-0044-W1-EVENT-SHAPE · DES-0044-W1-VISIBLE-MARKUP-EVIDENCE | FX-0044-W1-EVENT-SHAPE, IT-0044-W1-EVENT-SHAPE and shared F-1316 UT-0044-W4-STRUCTURED-DATA. Compare an eligible visible event, cancellation and unproved date or venue, then replay against a changed revision. Use exact 0115 visible facts and admitted Event shape. Hold stale or unsupported facts. No automatic Event output or search appearance claim. |
| F-0108 · REQ-0044-W1-KEY-MOMENTS · DES-0044-W1-VISIBLE-MARKUP-EVIDENCE | FX-0044-W1-KEY-MOMENTS with shared check IT-0044-W1-KEY-MOMENTS. Compare a verified video and visible exact timestamp with an unsupported time and repeated provider claim. Bind admitted 0115 media facts to the current page, visible reference, rights and VideoObject profile. Keep unproved facts out. No provider fetch or markup from this review. |
| F-0109 · REQ-0044-W1-IMAGE-LICENCE · DES-0044-W1-VISIBLE-MARKUP-EVIDENCE | FX-0044-W1-IMAGE-LICENCE, IT-0044-W1-IMAGE-LICENCE and shared IT-0084-RIGHTS. Compare an owner-original image with valid terms, a third-party licenced image, withdrawn terms and existing duplicate markup. Only the exact current owner-original item with proved rights can enter review. Third-party licence does not grant this public effect. No duplicate or unapproved output. |
| F-0110 · REQ-0044-W1-MEDIA-SITEMAP · DES-0044-W1-NATIVE-MEDIA-INVENTORY | FX-0044-W1-MEDIA-SITEMAP, IT-0044-W1-MEDIA-SITEMAP. Compare current eligible media entries, a native plugin already serving them, a duplicate and withdrawn media. Use admitted 0040 sitemap and 0044 visibility observations. Unknown plugin state or route holds. No second sitemap or endpoint. The positive fixture needs the exact approved native bounded route. DR-106 continues to bar Search Console submission. |
| F-0111 · REQ-0044-W1-TDM-RESERVATION · DES-0044-W1-POLICY-PREVIEW | FX-0044-W1-TDM-RESERVATION, IT-0044-W1-TDM-RESERVATION. Preview an exact owner reservation against an admitted versioned syntax profile under QR-025 and the existing 0082 policy. Compare unaffected directives, malformed input, stale version and replay. An unproved profile stays held. Any public change requires separate native approval and measured footprint. No change to unrelated rules, private access or external crawler compliance claim. |
| F-0112 · REQ-0044-W1-EXPERT-VERIFICATION · DES-0044-W1-EXACT-AUTHORITY-LABEL | FX-0044-W1-EXPERT-VERIFICATION, IT-0044-W1-EXPERT-VERIFICATION. Compare an eligible exact visible answer with a later edit, revoked owner designation and an undesignated moderator. Check the actual current expert identity, current native 0101 role and exact revision at approval and render. Hold stale or unauthorised labels. No impersonation or edit of the answer; any public label needs its admitted route and budget. |

The existing Wave 4 crawler and validation contracts keep their
requirements and case identities. The planned fixtures here grant
no renderer admission, analytics release or public effect.

## Changes from the source

- The crawler's decision logic (which address may be fetched, the slowing
  rule, retention and checks) is rewritten as Rust, proved with Kani.
- The maths jobs run in ganit, the Rust numerical engine.
- Report screens are a desktop app built from SDS components.
- Speed follows the 5% median rule; the 20x goal is kept (spec 0007).
- "Forum" now reads "registered site" throughout.
- The render route needs EX-02 admission and the platform cage proof.

Source: `docs/specifications/signals/seo.md`,
`docs/specifications/signals/seo-2.md` and
`docs/specifications/signals/seo-crawler-amendment.md` in the archive
`archive-2026-11-08-old-repo`.

## Unified suite readers

[Spec 0141](0141-audit-crawl-logs-and-technical-toolbox.md) uses
these facts for the unified crawl and Toolbox. It separately owns
DR-132 pair acquisition through 0082 and 0115. The
[audit grid][audit-grid-0143] may join eligible 0040 and field-speed
facts and supplied Matomo aggregates. It gives no provider, fetch
or page scope authority.

### Saved visibility and declared-identity observations

Under QR-110 and QR-050, F-0176 reads the already admitted 0115
visibility fact for one exact own-site Audit capture and reuses
0105 N-1825. Bind site, eligible page and revision, capture,
render profile, viewport, scoped region, measured reason, coverage
and current grant. Use the scrubbed evidence reference. A proved
visibility result remains available when no pair qualifies.

Report the obscured region with its measured reason and profile. Show a
proved closed help control as collapsed. An outside-viewport
observation is limited to that viewport. Missing geometry, a
blocked dependency affecting the region or raw Receipts-only
evidence gives unavailable. Do not read the concealed text, open a
control or render again to fill the gap.

Report a served-response difference only for a saved RN-45 pair
with the exact two admitted declared identities, current scope and
proved common serving generation. Compare the main responses before
scripts under the existing rule. Script timing, a changed viewport
or mismatched render scope gives no result for the pair. Label each
supported result with its own source, date and evidence. When one
qualified view has no valid comparison, say “Visibility observed.
No comparable second view.” Keep both reads subject to current
grants. No detection proves intent, a visitor's actual experience
or how Google sees the page. Reading never acquires a pair.

[audit-grid-0143]: 0143-audit-grid-findings-and-local-preview.md
