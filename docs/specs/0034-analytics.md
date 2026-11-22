# 0034: darshak, first-party analytics

darshak ("audience") counts how people use a site, first-party, with no
outside analytics service. Its first job is to tell people from
programs. It feeds link ranking with rates from people's visits only,
never with raw popularity counts. It is roadmap Phase 5 item 4.

Example: a person opens a thread, reads for 40 seconds and scrolls. The
counting script confirms the page was visible and touched, the page's
signed token checks, and the visit is labelled "human". A program
fetching the same page every second from a data centre is labelled
"suspected automation", reason DATACENTRE, and never reaches ranking.

[Spec 0135](0135-ai-fetches-referrals-and-ranking.md) reuses crawler
verification and human aggregate rules. Its separate verified
fetch fact is an order-stage input under DR-122. Human referral
figures stay in analytics.

## What it does

### Terms

- Visit: one page view and what followed on that page.
- Label: the one class a visit gets. Reason code: why.
- Population: a dashboard group.
- Page token: a short ticket from the site, signed with a shared secret
  (HMAC, a keyed fingerprint), so only pages the site really served can
  be counted.
- Daily hash: an anonymous code made from the visitor's address and
  browser name with a random secret ("salt") destroyed every day, so
  yesterday's codes can never be matched again.
- Reverse DNS with forward confirmation: asking what name an address
  has, checking it ends in the crawler company's domain, then asking
  that name's address and checking it matches.
- Data centre: a network of rented servers, where most automated traffic
  comes from and few readers do.
- Rate: a share or average (such as engaged visits per visit), as
  opposed to a count (such as total views).

### Scope

In scope: the counting script and endpoint; the counting modes; the six
layers and labels; the page token; the daily hash; country by DB-IP
Lite; the four populations; reports and the Adobe capability matrix;
GA4 merging; rate-only ranking inputs; storage, retention and erasure.

Out of scope: object storage (spec 0038); the connectors (spec 0040);
crawling and log reading (spec 0044); the selector that places links and
its ranking and link-equity rules (spec 0027).

### Design

- Crate `sutradhar-darshak`, with modules `collect`, `token`, `hash`,
  `layers` (one per layer), `label`, `logs`, `geo` (DB-IP Lite through
  `maxminddb`), `verify` (`hickory-resolver` and `ipnet`), `sessionise`,
  `aggregate`, `report`, `merge` (GA4), `matrix`, `export` (Arrow for
  ganit) and `features` (rates for ranking).
- `hickory-resolver`, `ipnet` and `maxminddb` each join the dependency
  list for that single role.
- Each layer is a type behind trait `Layer` (`judge(&Visit) ->
  Verdict`), so a layer can be added or tested alone. Other traits with
  fakes: `Resolver`, `GeoDb`, `CrawlerLists`, `EventStore` (PostgreSQL
  with row-level security), `ArchiveStore`, `TrafficSource` (GA4),
  `Clock` and `Rng`.
- One visit schema from script to archive; one labelling pipeline for
  script hits and server-log lines; one rate calculator for reports and
  ranking. The label is the first rule that matches in a fixed order; no
  score blending.
- The counting script is small strict TypeScript, about 2 KB, added by
  the add-on to guest-visible pages only.
- Facts checked before building: DB-IP Lite's download address,
  checksums, sizes and CC BY 4.0 terms; the community bot list, whose
  licence must suit Apache-2.0; the data-centre network list and
  its licence; the crawlers' published address lists and reverse-DNS
  domains (Google, Bing, the main AI crawlers), with dates; XenForo
  2.3's guest page cache lifetime; and the privacy-notice template
  (the site owner's responsibility).

### Counting modes and consent

1. Anonymous (default where the site's law allows): nothing stored on
   the device (no cookies, no local or session storage); the address
   used in memory only; the daily hash; no journeys across days. Page
   pairs inside one anonymous visit are counted for hub choice, as
   aggregates only. A daily count by referring site is kept, with no
   path.
2. Require consent (the owner's switch): nothing is counted, not even
   anonymously, until the visitor agrees. When the site's law is not
   known, the site fails closed to this mode.
3. Detailed histories, in either mode, require consent and use a
   random first-party visitor number. Linked journeys and their
   identifying links expire within 90 days. Withdrawal and “Delete
   my history” stop and erase that evidence. DR-029's raw event
   retention never extends that journey or its identifiers.
4. Opt-outs: a browser sending `Sec-GPC: 1` or `DNT: 1` (Global Privacy
   Control or Do Not Track) is not counted at all; the script also
   checks `navigator.globalPrivacyControl`.
5. Notice: setup gives a privacy-policy paragraph to add. darshak shows
   a "not yet confirmed" warning until the administrator confirms it is
   in place.

### Collection

1. The script fetches the page token from an uncached endpoint, never
   from the cached page. It makes a visit id in the page's memory and
   sends a start beacon with `navigator.sendBeacon`: token, thread or
   forum id, visit id and `navigator.webdriver`.
2. When the page is hidden or left, it sends an end beacon: visit id,
   visible seconds, interaction flag, scroll band, clicks on added links
   and the real-user speed fields.
3. Both beacons are always sent, whatever the visit looks like, so a
   visit that fails the human checks is labelled, never lost. The script
   never blocks the page and never retries.
4. `collect` checks size (4 KiB), schema, opt-outs and the mode. On a
   start beacon it runs layers 1, 2, 3, 5 and 6 with the address in
   memory, computes the daily hash and drops the address. On the end
   beacon, or 30 minutes after the start, it adds layer 4 and gives the
   visit its one label. It answers `204` at once.
5. A flusher writes batches to PostgreSQL every second or 1,000 events.
   Raw day files are zstd-compressed for archiving.
6. One counted view per anonymous visitor per thread per day, which
   also meets one view per page per session. Repeats are kept as
   repeats.
7. Prefetch and prerender send no beacon until the page is really
   visible. A page restored from the back and forward cache sends no
   second start beacon.

### One label per visit

Labels: `human`, `suspected_automation`, `verified_search_crawler`,
`ai_crawler`, `declared_bot`. A visit gets the label of the first rule
that matches, so it has exactly one label and one reason. Every visit
darshak sees, from a beacon or a log line, is stored with its label.

| Order | Rule | Label | Reason |
| --- | --- | --- | --- |
| 1 | Sutradhar's own health crawler, verified | declared_bot | OWN |
| 2 | Claims a search crawler, verified | verified_search_crawler | VERIFIED |
| 3 | Claims an AI crawler, verified | ai_crawler | VERIFIED |
| 4 | Claims a crawler, no check offered | suspected_automation | UNVERIFIABLE |
| 5 | Claims a crawler, check failed | suspected_automation | FAKE_CRAWLER |
| 6 | On the community bot list | declared_bot | UA_LIST |
| 7 | Beacon token missing, wrong or expired | suspected_automation | TOKEN |
| 8 | Data-centre network, not verified | suspected_automation | DATACENTRE |
| 9 | `navigator.webdriver` true | suspected_automation | WEBDRIVER |
| 10 | Log line with no joined beacon | suspected_automation | NO_SCRIPT |
| 11 | Under 5 s visible, no interaction | suspected_automation | UNCONFIRMED |
| 12 | Over a rate limit | suspected_automation | RATE |
| 13 | Over the network-share cap | suspected_automation | NETWORK_SHARE |
| 14 | ganit job 322 says likely automated | suspected_automation | MODEL |
| 15 | None of the above | human | HUMAN |

- Rule 4: a crawler whose operator offers no way to check it is treated
  like a fake ("suspected unless verified").
- Rules 10 and 11 also catch people who block scripts or leave within
  5 seconds. They keep their own reasons, are counted on the dashboard
  and never feed ranking.
- Readers on a VPN or cloud browser fall under DATACENTRE: counted and
  shown, never lost.

### The six layers

1. Crawler verification. A browser name claiming a known crawler is
   checked against the operator's published address ranges (refreshed
   daily; a failed refresh keeps the last good list), else by reverse
   DNS with forward confirmation. Results are cached for the day, keyed
   by the daily hash, not the address. If DNS is down, the claim is
   rechecked after 10 minutes, then labelled UNVERIFIABLE, never
   guessed.
2. Community bot list. A pinned list of bot browser names, matched in
   one pass with `aho-corasick`, with `regex` for the few patterns that
   need it.
3. Network type and country, read locally with `maxminddb`; no address
   leaves the machine. Network type comes from DB-IP's IP-to-ASN Lite
   file matched against a local list of data-centre networks.
   Data-centre traffic is suspected unless layer 1 verified it. Country
   comes from DB-IP's Country Lite file, for display only: the layers
   get a visit record with no country field, so no rule can filter by
   it.
4. Human checks from the script: visible at least 5 seconds (setting),
   an interaction, and `navigator.webdriver` false.
5. The page token, on beacons only. It holds `site id, page id, window,
   key id, signature`. The window is the current 5-minute period; the
   signature is HMAC-SHA256 of the rest with the shared key (made by the
   add-on in PHP, checked in Rust with `hmac` and `sha2`). Valid if the
   signature checks in constant time, the page id matches, and the
   window lies between the guest page cache lifetime plus 10 minutes ago
   (default 15 minutes) and one window ahead. The 10 minutes cover a
   slow clock, the window ahead a fast one. Start-up refuses an
   allowance shorter than the cache lifetime. Two keys are accepted
   while a key is changed.
6. Limits. Rate limits per daily hash and per network (`governor`); one
   counted view per anonymous visitor per thread per day; and no single
   network may give more than 20% (setting) of a thread's human views in
   a day, the excess labelled NETWORK_SHARE. The cap is per thread, not
   per site, and the hash includes the browser name, for offices and
   mobile networks behind one address.

### Beacons, log lines and the visit id

- A visit is labelled when its end beacon arrives, or 30 minutes after
  its start if none does (then from the start beacon, usually
  UNCONFIRMED).
- The end beacon joins its visit by the visit id, never by time or hash,
  so a visit crossing midnight keeps the day and hash it started with.
- Server-log lines (optional, spec 0044) are read every 5 minutes by
  darshak's log task, which holds the day's salt. First check approval
  under spec 0044 and the counting rules above. The source must preserve
  the opt-out decision for the request. In consent mode it must also
  prove consent was active for that request and remains valid when the
  batch commits. The evidence must come from an admitted site-owned
  source, bound to the exact site and request. Consent mode also needs
  a withdrawal check fenced at commit. Until the format and its
  evidence are admitted under QR-088, refuse it. A common or combined
  log alone cannot prove either decision. A missing, untrusted or
  opted-out decision adds no visit, mark, report count, inflation input
  or ranking input.
- An admitted line joins a beacon visit when its daily hash and page id
  match and its time is within 2 minutes of the start beacon. It adds
  only a "seen in logs" mark. An admitted line that joins nothing is
  its own visit, labelled by rules 1 to 6, 8 and 10. Discard a late line
  after its day closes. The cursor and results commit together so a
  retried batch cannot count a line twice.
- Keep log contributions keyed to their approval version, separately
  from beacon counts. This includes joined marks and derived inputs.
  Read or publish only contributions permitted by the current version
  and retention. Stop, change or site removal makes the old version
  unreadable at once and queues it for removal. If an output used that
  version, withhold it until rebuilt without the old contributions.
  Keep beacon-only counts. Use the site's deletion and backup rules in
  spec 0032. Expiry removes its effects by the same route.

### The daily hash

- `hash = HMAC-SHA256(salt, site id + address + browser name)`, cut to
  128 bits, computed in memory.
- The salt is 32 random bytes, held only in memory as a secret value.
- A new salt starts at midnight in the site's time zone. The old one is
  kept only until the day closes at 02:00 (setting), for late beacons
  and log lines, and is then wiped.
- At day close the hash is removed from that day's stored visits, so no
  one, Sutradhar included, can link them to an address, to each other or
  to the next day.
- A restart makes a new salt; that day is marked "counting restarted",
  since a reader may be counted twice. Losing the salt on restart
  splits only that day; earlier days are not touched.

### Country figures

- From DB-IP Country Lite (CC BY 4.0). Every screen showing country
  carries "IP Geolocation by DB-IP" with a link to db-ip.com.
- The files update monthly by a karya job; a failed or unverified
  download keeps the previous files.

### The four populations

People (`human`); verified search crawlers; AI crawlers; and other
automation (`suspected_automation` and `declared_bot`, each still shown
by reason). They are shown side by side and never added together. Only
`human` visits feed reports of reader behaviour and ranking.

### ganit jobs used

Inputs are Arrow snapshots of ids, labels, reasons, counts, seconds and
network-type flags; never addresses, hashes, browser names, text or
country. A candidate result enters, remains in or leaves service only
through spec 0077's champion/challenger controller. Shadow evidence has no
serving authority. A late or failed job leaves the verified champion
unchanged; rollback restores the complete prior champion under spec 0077.

- Job 322: bot probability per session, from behaviour, network and
  verification signals (rule 14).
- Job 323: inflation check comparing script counts, server logs and
  Search Console clicks, shown as the trust report.
- Job 324: need score per thread, from inbound links, click depth,
  impressions at positions 8 to 20 and useful but unfound content.
  Any production weight change uses spec 0077's authorisation and
  evidence. Job 298 is link-impact evaluation. Report or experiment
  outputs from spec 0094 do not tune this job. It takes no darshak
  counts.
- Job 325: quality floor per target, from rates with shrinkage for
  low-traffic threads (engaged-visit rate, solved, reports, outdated).
- Also: 204 and 212 (forecasts), 215 (missing days, shown as estimates),
  161 (changepoints in organic clicks), 176 (request-rate anomalies),
  198 (traffic-source shift), 298 and 308 (effect of added links), 56
  (which added links get clicked).

### Wave 1 protected reader evidence

Use only the current eligible human population inside the admitted
0034 producer. Bind the active site, exact page or link revision,
counting mode, observation window, supported denominator, consent
or other applicable authority, and release and erasure generations.
Use the common disclosure history. Do not expose join keys or
infer individual order from aggregate totals. Missing, withdrawn,
partial or unsafe evidence stays unknown.

| Feature and owning design | Exact planned fixture and outcome |
|---|---|
| F-0052 · REQ-0034-W1-LINK-SATISFACTION · DES-0034-W1-JOINT-LINK-QUALITY | FX-0034-W1-LINK-SATISFACTION, IT-0034-W1-LINK-SATISFACTION. Compare an exact same-site link click and independently admitted follow-on reading, a bot, revoked follow-on evidence, sparse support and separate click and dwell marginals. Use only the admitted joint producer, dwell rule, window and matching eligible denominator. A missing profile, inadequate support or invalid consent holds. Reuse 0034 consent and human-rate checks. Only the existing 0027 RS18/job 325 and 0077 admission path can make a qualified human rate available to post-relevance ordering. |
| F-0053 · REQ-0034-W1-DROP-OFF · DES-0034-W1-SCROLL-BANDS | FX-0034-W1-DROP-OFF, BDD-0034-W1-DROP-OFF. Compare eligible aggregate scroll bands on one exact thread revision, a missing end beacon, revealing overlapping samples and reordered posts. Show only supported bands and coverage under the disclosed population. A coarse band cannot locate a post or reconstruct a visit trail. Hold any exact answer-move suggestion without the separately admitted mapping. No native move or extra capture. |
| F-0054 · REQ-0034-W1-EXIT-ADVICE · DES-0034-W1-EXIT-POPULATION | FX-0034-W1-EXIT-ADVICE, BDD-0034-W1-EXIT-ADVICE. Compare a proved exit and eligible onward visit in one admitted completed window, incomplete or revoked evidence, sparse support and a popular off-topic target. An open or missing observation is unknown, and an anonymous co-occurrence does not prove direction or exit. Only eligible relevant next-step links may reach ordinary review. No direct rank input, new capture or native change. |

Reuse AT-AN02 and AT-AN22 to AT-AN24 for human, opt-out and
consent authority; AT-AN33/34 for ranking-rate boundaries and
AT-AN37 for an unfinished beacon. These cases retain their owned
scope and do not establish a new joint, scroll-to-post or exit
profile. The fixture identities bind planned proof only.

### The freeze rule

- If a thread's bot share (`suspected_automation` and `declared_bot`
  visits as a share of all its visits; verified crawlers excluded)
  passes 50% in a day, or job 176 flags an anomaly, its ranking features
  retain only the last clean values of admitted traffic rates. With
  no last good rate those inputs are unavailable. The freeze performs
  no direct demotion and leaves non-traffic inputs current.
- Raw counts are still recorded and shown, marked "frozen for ranking"
  with the reason.
- It unfreezes after 7 clean days in a row. Both thresholds are
  settings.

### What darshak gives ranking

The rules for using these inputs live in spec 0027 only.

- Only `human` visits feed ranking.
- `GET /api/darshak/features` offers, per thread and day, rates only:
  engaged-visit rate, click-through rate on added links and sign-up rate
  after a visit, each with its visit count for shrinkage, never alone.
  Each carries kind `rate`; the feature registry refuses counts.
- Pair rate, for hubs only: for pages A and B, the share of visits to A
  that go on to B, with its visit count (spec 0042), sent to ganit as a
  `rate`.
- A frozen thread's features are its last good values, marked frozen.
- Example: target A has 40,000 views and B 900. darshak offers neither
  number, only A's engaged-visit rate of 0.21 and B's of 0.48.

### Storage and retention

- `darshak_visit`, partitioned by day: visit id, site id, page id,
  label, reason, daily hash (removed at day close), network-type flag,
  country code, visible seconds, interaction flag, scroll band,
  added-link clicks, repeat count, "seen in logs", referrer class,
  device class and the speed fields. No address, browser name or site
  user id.
- `darshak_referrer_day`: day, site id, referring host and count of
  human visits. This store has no path or query field. The separate
  [spec 0120](0120-opt-in-referral-backlink-discovery.md) route
  cannot join its data to darshak. The privacy record owns the visit
  floor. Rows without an admitted floor are never shown.
- `darshak_day_*`: aggregates by population. `darshak_feature`: per
  thread and day, rates, frozen flag and reason. `darshak_salt_event`:
  rotations and restarts, never a salt.
- Speed fields (for spec 0044's Core Web Vitals counts): Largest
  Contentful Paint, Interaction to Next Paint, Cumulative Layout Shift,
  time to first byte and First Contentful Paint, from the browser's own
  performance interface, with the page type.
- Raw visitor events use the owner-set 0100 period, default 395 days
  under DR-029. Strip daily hashes at day close from every working and
  archived copy. Keep journey links and first-party visitor numbers
  in a separate consent store for at most 90 days. Withdrawal removes
  associated lawful rows and updates or withholds derivatives under
  0032 before unlinking. Retained event rows and backups cannot be
  relinked after expiry or through a supplied identity or external
  source. Delete expired data. Keep eligible aggregates two years.
  Apply the same fences to protected storage and verify moved bytes.
- Thread merges and moves: aggregates follow the merge.
- Signed-in members are counted like guests, with no link to their
  account.

### Reports, GA4 and the matrix

- Reports read `human` aggregates unless about crawlers: freeform
  tables, segments, calculated metrics, flows and fallout, cohorts
  (consenting visitors only), sign-up attribution to added links,
  real-time (last 30 minutes), alerts, scheduled emails, scrubbed and
  previewed exports, before-and-after views, the four populations and
  the trust report.
- GA4's figure is shown where both measure the same thing and GA4's day
  is complete. Numbers are never added together.
- `catalogue/adobe-matrix.toml` lists each Adobe Analytics capability
  from Adobe's public pages, with its date and Sutradhar's evidence.
  Nothing is called "like Adobe" beyond it.

### Breakdown table and segment rule profiles (1336 and 1337)

Use the current protected 0034 producer and disclosure history. Bind the
active site, actor, purpose and counting mode, source and measure version,
population and data grain, interval and zone, coverage and rights and
erasure generations. Never expose row-level evidence, identifiers or
suppressed values through preview, filter labels, totals, export,
repeated comparisons or restore. Specs 0100 and 0118 protect the exact
approved local projection.

| Feature | Exact report or rule contract |
|---|---|
| 1336 Breakdown tables | Choose measure and ordered dimensions from the admitted compatible field profiles. Use their historic assignments and declared units and denominators. Keep unknown, unassigned and suppressed distinct. Do not sum overlapping groups or mix provider and native populations. Apply the common privacy ledger to each level, complement, roll-up and filter before release. A split that would disclose a protected row is held. |
| 1337 Visit segment builder | Own the typed rule format for page-view, visit and admitted member aggregate populations. Pin allowed bounded fields, grain, order, operators, typed true/false/unknown shape, current consent and rule revision. Reject incompatible grain, fields or unsupported predicate order. Evaluate within the admitted producer preprojection. Never infer membership or intersections from marginal totals. Evaluate rules only within their owning producer and expose eligible counts. Never link anonymous visits to member records or across the anonymous site-day boundary. Reuse the rule format for 450, 187 and 226 only in their own data and permission profiles. A result cannot grant audience access, send, track or create a ranking input. |

QR-196 qualifies field compatibility, predicate semantics, source scopes,
privacy and coverage. Exercise empty, unsupported, wrong-site, expired,
withdrawn, sparse, repeated-filter and restoration cases. These views add
no beacon, cross-device identity or source collection. Hold a cursor,
filter or export that could reveal a rare group. Use 0078 bounds.

The existing aggregate API also owns the collector evidence in
TASK-0034-SCIENCE-DARSHAK under [0150][experiment-consumer].
Bind 0034's consent and day closure checks to IT-0150-DAR-READER
for the exact current human denominator and approved metric. Give
the consumer only the scoped aggregate and authorised receipt.
No visitor record, new collector or once-valid denominator serves
as current scientific evidence.

[experiment-consumer]: 0150-experiment-source-and-sensor-gates.md

### Protected audience measures (524–538)

Each measure pins site, exact approved source and privacy cohort,
metric and unit, population, page and revision basis, native
interval and zone, coverage, grant and erasure generation.
Read guest-visible page evidence for author, section and tag views.
Use recorded historic assignments if proved for the interval.
Existing merge and move totals keep their own declared basis.
A current assignment cannot fabricate an old dimension. Show
unassigned, unknown and suppressed separately. A person label
needs authorised public attribution. Multi-tag groups may overlap,
so their rows cannot add to a site total.

Keep the existing anonymous mode. The members-versus-guests view
requires either the admitted site-native status aggregate or an
admitted supplier file that proves an equivalent aggregate.
Darshak does not link accounts or sign-in to visits. Never match
anonymous visits to membership or outcomes. For conversions use
only admitted native consented joint aggregates or the existing
0034 consented journey producer where its exact rules cover the
joint event. A profile pins source, rights, consent epoch and
projection. All member IDs, visit tokens and anonymous hashes
stay in their current owning boundary. The report, PC projection
and MCP receive no joint identifiers. Use no marginal-count join.
Unproved evidence shows “Not measured”.

The live view covers the six most recent closed five-minute UTC
slots and shows source and watermark. Each slot is half-open
and ends on a UTC five-minute boundary. Release a site aggregate
only once the source watermark proves complete classification,
including allowed delayed beacons. Partial and open slots show
“Still collecting”. Delayed classification can leave all recent
values unknown. Never issue a provisional count or update a released
bucket on refresh. Late correction invalidates affected rows and
requires a new safe release. Suppress revealing totals and repeated
differences across views and exports. Pause, stale input and clock
gaps show a reason. The existing private site management SSE
stream carries approved aggregates only, with current grants.

A page engagement view uses the existing 0034 eligible rate and
floor. It compares the page with an unchanged page-kind and
section definition in one window. Exclude the chosen page from
the peer pool. Form the peer reference from the same admitted
projection. Use the admitted privacy release for both rates. Keep numerator,
denominator, sample detail and support only inside the producer.
Return a fixed five-point rate band or unknown. Withhold the rate
if another view, peer set, tooltip, diagnostic, prior export or
filter can reveal its visit count or small support. Apply the
common release history across MCP, reports and UI. No blended score.

Registration and upgrade cards count one eligible native outcome
ID once, linked to the earliest eligible entry page in its proved
consented journey. Pin the native page revision, original cohort
interval and approved journey limit from one to 30 elapsed days.
An opted-out, late outside-window or unmatched event cannot count.
Cross-day links need that exact consented profile. Partial,
dropped or incompatible evidence remains unknown.

A funnel pins ordered visit, register, first qualifying post and
upgrade stages and the same entry cohort. Each later stage's base
is the proven eligible immediately preceding stage with completed
follow-up. Count distinct eligible subjects once per stage within
the native producer. Match ordering, expiry and consent. Missing
intermediate evidence cannot prove a downstream stage. Release
a typed count and rate pair, or unknown, with stage, coverage and
precision. Keep the base private when required. Apply disclosure
jointly across every stage, filter and recipient.

For traffic source, use a single mutually exclusive provider
source mapping with its native unknown category. Divide eligible
human visits in the bucket by the same closed population of
eligible attributable and unknown visits. Keep reports by
distinct providers separate. Context can show a covered source
domain where allowed. Never include a referring path or query
in these views. Respect any separate route's exact purpose.

For publishing time and format, use native publish instant,
time-zone definition, exact format and comparable closed
follow-up windows. Match within site, page kind and source.
Require at least ten eligible pages per compared group and
admitted coverage. Use the matching outcome and denominator
with the qualified 0066 comparison. Show sample scope and
uncertainty; unreliable or sparse groups give no preferred time.
Sponsor views show exact approved campaign and eligible placement
period alone. Distinguish visits, eligible engagement, and
proved conversion. Never use one as evidence of another, merge
with provider clicks or infer a sponsor's total from site totals.

Use fixed attention-time bands with seconds `[0,10)`, `[10,30)`,
`[30,60)`, `[60,180)` and `[180,infinity)`. The current visible
seconds and interaction flag do not prove active duration. An
admitted source must prove disjoint monotonic foreground intervals
with activity, timestamps, deduplicated start and stop and valid
consent. Clip each interval at the earliest hide, session end or
admitted inactivity deadline. Refuse incomplete or unproved timing.
No beacon change without its own approved footprint.

Depth bands are 1, 2, 3, 4, 5–9 and 10 or more distinct eligible
pages per proved visit. Use its original site day and salt rule.
Never join anonymous visits across that day boundary. Use one
eligible entry source per visit under the 531 mapping, including
unknown. Keep provider scopes and source-specific floors separate.
Apply common disclosure across time and depth bands and totals.

Exports use the exact protected 0100 snapshot projection. Check
site, actor, source freshness, scope and erasure at publication,
copy, replay and restore. Freeze definition, method, source dates
and suppression under the shared privacy record. Recheck retention,
rights and erasure epochs. On source withdrawal revoke dependent
outputs, then recompute or withhold. The owner privacy view shows
safe mode, consent, scope, retention and destination summaries
and erasure actions. Keep secrets out. A policy draft uses approved
facts and owner wording. A change to mode, scope, source, retention
or destination invalidates it. No cookie
switch, processing or page publication follows from a draft.

Paywall markup requires an index-intended existing CreativeWork
page and proved protected regions. Use [Google's profile][paywall]:
set `isAccessibleForFree` to false on the page and each `hasPart`
WebPageElement. Only exact class selectors for non-nested regions.
A private or noindex page gives no proposal. Do not include
protected text or send it to a validator. Prove locally for guest,
authorised and cached responses. A 0021/0028 effect binds page,
revision, content and policy, canonical and robots invariants
and current moderator and owner authority. Admit cache and public
footprint, one receipt and guarded undo. Keep later staff edits.
Without proof show a proposal only. No access or indexing change.

Wall advice is advisory, scoped to approved kinds and observed
evidence. No causal or conversion guarantee. None of these new
projections can register a ranking input. The already admitted
Darshak rate interface remains the separate ranking path.

[paywall]:
https://developers.google.com/search/docs/appearance/structured-data/paywalled-content

#### F-0538 registration wall advice

REQ-0034-W2-WALL-ADVICE and DES-0034-W2-OBSERVED-WALL
use authorised own-site evidence for the exact wall kind,
page revision and observed audience coverage. The local
review shows limitations and source evidence.

FX-0034-W2-WALL-ADVICE and BDD-0034-W2-WALL-ADVICE use one
supported observation, changed coverage, an unsupported
proposal, conflicting observations and revoked rights. Show
bounded advice only where the evidence remains eligible.
Refuse any native access change or new ranking input. Keep
the case distinct from the approved markup path under F-0537.

#### Named audience measure and disclosure fixtures

These planned spec 0076 cases reuse existing privacy, consent,
classification and reset controls. QR-145 holds the exact source,
joint producer, privacy floor, native and release proof. The
listed AT cases cover their existing boundaries. Each FX adds
the feature-specific synthetic assertion, with no timer, account
join, public page work or ranking input.

- `DES-0034-W2-AUDIENCE-PROJECTION` uses the named source
  variants below. Match the exact native population, time basis
  and current permission. Apply shared suppression to retained
  reports, UI, MCP and export. Missing or unauthorised evidence gives no count.
- `DES-0034-W2-CONSENTED-OUTCOMES` uses
  `VAR-0034-W2-ATTESTED-JOINT-OUTCOMES` and
  `VAR-0034-W2-COMPLETE-PRIOR-STAGE`. The admitted native joint
  or eligible existing consented journey producer retains
  identifiers. Separate totals cannot prove joint evidence.
- `DES-0034-W2-NATIVE-PROPOSAL` uses
  `VAR-0034-W2-TYPED-PROTECTED-REGION`. The fixture exercises
  local proposal and refusal. Any native effect still needs
  exact 0021/0028 authority and site, visibility and budget proof.
- `DES-0034-W2-PROTECTED-RELEASE` uses
  `VAR-0034-W2-EXACT-LOCAL-SNAPSHOT` and
  `VAR-0034-W2-CURRENT-PRIVACY-DRAFT`. The existing local
  file and draft routes preserve current actor, exact source
  scope and erasure. A draft gives no setting or publish action.

| Feature | Requirement | Design and variant | Named fixture and existing AT boundary | Named fixture assertion |
|---|---|---|---|---|
| F-0524 | REQ-0034-W2-HISTORIC-CONTENT | DES-0034-W2-AUDIENCE-PROJECTION, VAR-0034-W2-HISTORIC-DIMENSIONS | FX-0034-W2-HISTORIC-CONTENT, AT-AN27 to AT-AN30, AT-AN48 | Change a page’s current author or section after a closed interval. The eligible row keeps the proved historic assignment. Hold revoked attribution and show unassigned separately. Overlapping tags cannot total the site. |
| F-0525 | REQ-0034-W2-MEMBERS-GUESTS | DES-0034-W2-AUDIENCE-PROJECTION, VAR-0034-W2-NATIVE-STATUS-AGGREGATE | FX-0034-W2-MEMBERS-GUESTS, AT-AN21, AT-AN24, AT-AN27 to AT-AN30 | Give admitted separate member and guest aggregates and an anonymous-only control. The latter shows “Not measured”. No attempted account join can turn anonymous visits into member counts. |
| F-0526 | REQ-0034-W2-CLOSED-LIVE-SLOTS | DES-0034-W2-AUDIENCE-PROJECTION, VAR-0034-W2-WATERMARKED-SLOTS | FX-0034-W2-CLOSED-LIVE-SLOTS, AT-AN37, AT-AN38 | Show six closed five-minute UTC slots. Hold a slot whose classifier watermark misses a late beacon. Repeated refresh returns the same authorised release. Correction invalidates it before a new eligible release. |
| F-0527 | REQ-0034-W2-ENGAGEMENT-BAND | DES-0034-W2-AUDIENCE-PROJECTION, VAR-0034-W2-PRIVATE-PAGE-RATE | FX-0034-W2-ENGAGEMENT-BAND, AT-AN33, AT-AN34 | Use an eligible same-kind section peer pool excluding the selected page. Show its five-point rate band only when safe. A prior tooltip or peer set that reconstructs support holds the band. |
| F-0528 | REQ-0034-W2-JOINT-OUTCOME | DES-0034-W2-CONSENTED-OUTCOMES, VAR-0034-W2-ATTESTED-JOINT-OUTCOMES | FX-0034-W2-JOINT-OUTCOME, AT-AN22, AT-AN23, AT-AN24 | Count an eligible native outcome ID once internally despite two receipts. The earliest proved entry revision gets credit. An outside-window or revoked event cannot count. Marginal totals provide no attribution. |
| F-0529 | REQ-0034-W2-PAYWALL-PREVIEW | DES-0034-W2-NATIVE-PROPOSAL, VAR-0034-W2-TYPED-PROTECTED-REGION | FX-0034-W2-PAYWALL-PREVIEW, AT-AN17, AT-AN48 | Given an index-intended page and one exact non-nested protected region, show only the typed proposal. Private or noindex pages hold it. Stale approval or unsafe guest-cache evidence yields no native effect or content disclosure. |
| F-0530 | REQ-0034-W2-ORDERED-FUNNEL | DES-0034-W2-CONSENTED-OUTCOMES, VAR-0034-W2-COMPLETE-PRIOR-STAGE | FX-0034-W2-ORDERED-FUNNEL, AT-AN23, AT-AN24 | Give a complete ordered journey, a repeated stage and a missing prior stage. Count each eligible stage once internally against its preceding completed base. A missing or reversed stage gives no later count. |
| F-0531 | REQ-0034-W2-SOURCE-SHARE | DES-0034-W2-AUDIENCE-PROJECTION, VAR-0034-W2-SOURCE-NATIVE-SHARE | FX-0034-W2-SOURCE-SHARE, AT-AN25, AT-AN27 to AT-AN30, AT-AN45 | Give 30 known-source and 10 unknown eligible human visits in one synthetic complete provider population. The internal share is 75%, subject to release. Refuse a provider sum or display of any referring path. |
| F-0532 | REQ-0034-W2-PUBLISHING-COMPARISON | DES-0034-W2-AUDIENCE-PROJECTION, VAR-0034-W2-MATCHED-PUBLISHING-COHORT | FX-0034-W2-PUBLISHING-COMPARISON, AT-AN34 | Compare equal closed follow-up for two same-site page-kind groups. A nine-page group, changed zone or unequal coverage yields no preferred time. An eligible group still needs the admitted comparison and uncertainty. |
| F-0533 | REQ-0034-W2-SPONSOR-MEASURES | DES-0034-W2-AUDIENCE-PROJECTION, VAR-0034-W2-SCOPED-CAMPAIGN-MEASURES | FX-0034-W2-SPONSOR-MEASURES, AT-AN24, AT-AN45 | Show only exact permitted placement-window measures. Keep eligible visits, engagement, proved conversion and missing coverage distinct. Other sponsors, provider clicks and revealing site totals do not fill the projection. |
| F-0534 | REQ-0034-W2-SAFE-ANALYTICS-FILE | DES-0034-W2-PROTECTED-RELEASE, VAR-0034-W2-EXACT-LOCAL-SNAPSHOT | FX-0034-W2-SAFE-ANALYTICS-FILE, AT-AN25, AT-AN47 | Stage the allowed projection, then revoke before publication. Publish nothing and hold the restored copy. A current allowed output is one atomic file with withheld cells absent and no secret or visitor field. |
| F-0535 | REQ-0034-W2-PRIVACY-NOTICE | DES-0034-W2-PROTECTED-RELEASE, VAR-0034-W2-CURRENT-PRIVACY-DRAFT | FX-0034-W2-PRIVACY-NOTICE, AT-AN21, AT-AN22, AT-AN23, AT-AN24, AT-AN41 | Build a local draft from current consent mode, retention and permitted destination summaries. Edit one input before approval. Show the old draft as stale. No setting change or policy publication follows. |
| F-0536 | REQ-0034-W2-ATTENTION-BANDS | DES-0034-W2-AUDIENCE-PROJECTION, VAR-0034-W2-FOREGROUND-INTERVALS | FX-0034-W2-ATTENTION-BANDS, AT-AN11, AT-AN24, AT-AN36 | Prove 9, 10 and 180 eligible foreground seconds with consent. Classify into [0,10), [10,30) and [180,infinity). An unmatched stop or passive visibility alone proves no duration. Hold a revealing band complement. |
| F-0537 | REQ-0034-W2-DEPTH-BANDS | DES-0034-W2-AUDIENCE-PROJECTION, VAR-0034-W2-NATIVE-VISIT-DEPTH | FX-0034-W2-DEPTH-BANDS, AT-AN13, AT-AN19, AT-AN39, AT-AN40 | Count pages A, A, B as two distinct pages within one proved visit. Keep the original site day for its own joined beacons. A separate next-day anonymous visit is never linked. Unknown source stays distinct. |

### External comparisons and public offer proposals (671, 673, 675)

These views carry site, purpose, current 0020 actor and rights,
exact source and subject revisions, date, population, coverage and
metric definition. Hold an output when those conditions change. Use shared
0032 erasure, 0108 site scope and 0118 protected storage. No additional
network, tracking, ranking or automatic publication route.

- **671, rival traffic estimates.** Under DR-085, an owner-supplied,
  licensed and bounded file can provide an estimate. Reuse the 0100
  safe intake and the current site-scoped 0092 rival identity.
  Bind exact report and estimate model version, native metric,
  unit, channel, rival version, scope, device and market. Preserve
  source collection date and effective period separately. An
  admitted profile gives exact grain, bounds and expiry. Refuse
  negative counts, inverted intervals and unsupported methods or
  fields. Pin corrections to the same manifest scope. Replace or
  withdraw earlier records atomically and deduplicate exact replay.
  Compare only estimates with compatible definitions, windows
  and evidence. Unknown or absent precision and values stay
  unknown. Never rank by presumed precision. Label first-party
  observations separately. They cannot fill or calibrate a rival
  estimate. DataForSEO or citation rehearsal cannot supply this
  measure. Rights follow the exact file grant and intended use.
  No provider call, spending, monitoring or estimate from a crawl.
- **673, review benchmarking.** Current owner approval binds
  exact counterpart, version, rights, intended use and scope.
  Use 0095 scrubbed evidence from own authorised and supplied
  rival sources only. Map location, provider, native metric and
  scale, interval, sampling, denominator and coverage on both
  sides. Keep stock of visible reviews separate from changes.
  “First observed” requires a stable review identity and adequate
  matched previous coverage. It gives no published date or proof
  of deletion. Calculate a mean only from eligible native numeric
  ratings, divided by their eligible count and with the same
  weighting definition. Do not average branch means or mix
  recommendations and rating scales. Release protected sample
  size and mean only together when safe. Show scope and sample
  limits, including non-representativeness. Missing, suppressed
  or withdrawn evidence stays unknown. No text, account ID,
  subtraction leakage, public check, affiliation or provider call.
- **675, membership offer pages.** Draft a local page from owner
  approved offer facts. Bind programme and public plan revision,
  benefits, total price and currency, qualifying conditions,
  start and expiry, supported public link and terms source.
  Unknown or changed public terms hold the draft. Use one guest
  projection of identical offer copy, links and terms for all
  visitors and crawlers on the same route and revision. No
  referrer, cohort, device or crawler-specific commercial variant.
  A responsive layout may rearrange the same public content.
  Do not copy member-only material, create a checkout or change
  access. Keep gated member facts, pricing experiments and
  membership-level variants out of this route. Staff approve
  the exact local hand-off. Native draft creation requires a
  separately admitted signed 0021/0028 draft-only effect binding
  site, page, expected revision, offer digest, current rights,
  staff permission and approval. Commit once against the expected
  revision and keep the exact receipt. Staff operate publication
  separately under current authority. Require parity evidence
  for the specific native template, add-on and host cache in
  guest, verified crawler, device and warm/cold responses.
  Synthetic tests qualify the method and cannot prove a live
  host. Reject a mismatch or missing proof. Public delivery
  needs verified expiry, revoke and cache invalidation so changed
  public terms cannot leave an old approved offer in circulation.
  Existing analytics or experiments give no variant authority.

### Phrase evidence, crawler contrast and photo packs (681, 683, 685)

Use the active site, exact scope, known evidence coverage and
current policy and rights. Bind input and method versions to
each dated result. Recheck disclosure and erasure at read, copy,
export and restore. Each feature adds zero public-page footprint.

- **681, phrase counts.** Use retained eligible own-site visible
  primary text under the current 0044/0115 reuse grant. Honour
  source opt-out, revocation and pending erasure before extraction.
  Exclude private regions, boilerplate and duplicate representations
  of the same page revision.
  Use 0047's admitted normalisation and Unicode word segmentation.
  Count contiguous one-, two- and three-word phrases within each
  eligible content block. Include overlapping positions. Never
  join across block, language or page boundaries. For a fixed
  snapshot report phrase occurrences and distinct eligible
  canonical pages independently. Label checked and missing
  populations, method and language. An unread page is outside
  the measured denominator and never gives zero. Apply admitted
  phrase disclosures, minimum distinct canonical page count and
  anti-differencing across scope, region, group, dates and outputs.
  Suppress literal sensitive phrases, including contact or identity
  fragments found in public text, under a versioned rule. Withhold
  when no complete release profile or candidate evidence exists.
  No author joins or raw text exports. Recheck source rights and
  erasure at result commit. Bound and ledger derivatives and
  caches under 0032/0118. Use 0078 memory and file caps.
  A capped pass shows partial coverage and publishes no full-site
  claim. No ranking signal or new indexing job.
- **683, crawler response contrast.** Spec 0082 admits only its
  declared Leviathan and Leviathan-Browser identities. Never
  offer a custom header or persona through any tool, UI, API,
  adapter, replay or restore. No new request or legal or robots
  bypass follows from a report. Compare paired own-site Audit
  evidence under 0115. Bind exact URL, request and source snapshot,
  revision, identity, robots scope, time, method, device and
  render profile, geographic scope, cache and consent state.
  Split served HTTP facts from computed render facts. Label
  differences with confounds. Show unmatched pairs separately.
  A render difference cannot prove identity-based discrimination.
  Use owner-supplied edge observations only in the admitted
  scrubbed 0044 form. Label observed or unverified identity
  and time separately. Exclude IP, query and trace values.
  Local synthetic fixtures can demonstrate agent-dependent
  behaviour without network requests. Never claim what another
  crawler received from these comparisons.
  The user-facing name in spec 0152 is separate from the wire
  identity and the recorded Audit capture. Saving that name
  changes neither the stored pair nor its report outcome.
  A backlink-check result supplies no Audit counterpart merely
  because it used the same declared identity and URL.
- **685, Business Profile photos.** Use 0097 permitted assets and
  0095 exact approved public location record. Let the owner
  review proposed additions, replacement, and withdrawal notes
  for logo, cover or business photo. Use the [Google Business
  Profile photo guide][gbp-photo] to qualify a dated checklist.
  Verify inert JPEG or PNG bytes, physical dimensions and size
  under the admitted policy. Label the documented 250 by 250
  minimum and 720 by 720 recommendation separately. Google
  lists 10 KB to 5 MB. Pin validated units in the policy
  profile before claiming a boundary verdict. Check decoded
  output bytes after stripping location and identity metadata
  under 0097. Review focus, lighting, realistic depiction and
  significant alteration separately. Bind the role-specific
  checklist, use rights, credit, people and location consent.
  Show local preview and staff-approved export with source,
  scrubbed digest and per-photo intent. Staff upload or remove
  their own photos in their account. A customer photo only
  gets a separately reviewed manual report/removal request if
  supported. The local hand-off note proves no provider state.
  Display pending, not approved or live only as independently
  supplied dated provider evidence. File conformance cannot
  verify the profile or guarantee acceptance or placement. DR-076
  authorises no direct account, upload, removal or monitoring.

#### Wave 2 comparison and protected hand-off fixtures

Use synthetic supplied records and existing own-site evidence. QR-163
and QR-165 still govern exact source, privacy and native proof. Use
stand-ins for separately admitted effects. These reports add no
collection, paid lookup, third-party action or ranking input.

| Feature | Owning requirement and design | Planned fixture and expected evidence |
|---|---|---|
| F-0671 | REQ-0034-W2-SUPPLIED-RIVAL-ESTIMATE · DES-0034-W2-SOURCE-BOUND-COMPARISON | FX-0034-W2-SUPPLIED-RIVAL-ESTIMATE · IT-0034-W2-SUPPLIED-RIVAL-ESTIMATE. Use a licensed owner-supplied estimate bound to the active site, exact 0092 rival, report model, source period, channel, unit and bounds. Compare a compatible correction with stale, unproved model and incompatible unit. Keep estimates and first-party measurements separate. An unknown or withdrawn record cannot be filled from citations, a crawl or live provider lookup. |
| F-0673 | REQ-0034-W2-PROTECTED-REVIEW-BENCHMARK · DES-0034-W2-SOURCE-BOUND-COMPARISON | FX-0034-W2-PROTECTED-REVIEW-BENCHMARK · IT-0095-REVIEWS. Compare exact authorised counterpart, location and comparable provider scale, coverage, denominator and weighting. Distinguish observed review stock from first observed under complete eligible prior coverage and stable identity. Try sparse or unproved branch, unlike scale and a revealing complement. Release count and mean together only when safe. No review text, identities, live pull or claim of publication or removal. |
| F-0675 | REQ-0034-W2-PUBLIC-OFFER-PARITY · DES-0034-W2-EXACT-PUBLIC-OFFER-DRAFT | FX-0034-W2-PUBLIC-OFFER-PARITY · IT-0034-W2-PUBLIC-OFFER-PARITY. Use an exact approved public offer and compare stand-in guest and crawler views, device layouts and warm and cold cache paths for one native template and revision. Try referrer-specific copy, unknown cache proof, foreign site, expired terms and withdrawal. Hold a mismatch or missing proof. Preserve one public offer meaning and distinct staff review. Until the exact 0021/0028 profile is admitted stay local. No checkout, access or publish effect. |
| F-0681 | REQ-0034-W2-PROTECTED-PHRASE-COUNTS · DES-0034-W2-EXACT-PHRASE-PROJECTION | FX-0034-W2-PROTECTED-PHRASE-COUNTS · IT-0034-W2-PROTECTED-PHRASE-COUNTS. Use the admitted 0047 segmentation and retained 0044/0115 primary text. Count eligible overlapping occurrences within blocks separately from distinct canonical pages. Exclude private regions, duplicate variants and identity fragments even from public text. Hold sparse or revealing cells and label partial pass coverage. Erasure before commit withdraws dependent output. No author join, raw export or ranking. For shared A144-02 evidence, compare only the same eligible one- to three-word windows with equal token and seam rules. A different boundary or language profile cannot share the result. Keep this report’s own occurrence and canonical page populations and release checks. |
| F-0683 | REQ-0034-W2-DECLARED-CRAWLER-CONTRAST · DES-0034-W2-OWNED-COMPARABLE-AUDIT | FX-0034-W2-DECLARED-CRAWLER-CONTRAST · IT-0034-W2-DECLARED-CRAWLER-CONTRAST. Use paired stored own-site Audit records with declared admitted identities and compatible revision, cache, consent, viewport and other capture scope. Keep served response, computed render, scrubbed supplied edge evidence and offline simulation separate. Refuse custom identity through UI, API, adapter or restore. Drop unapproved edge fields before output. Label confounds and claim only what the admitted captures show. The 0034 report reads passive evidence. An exactly approved live pair belongs to 0141 under DR-132 and uses only the two declared 0082 identities. Offer a live-pair receipt when opening, replaying or restoring the report. Only the existing admitted 0115 projection can supply capture facts. No report action can dispatch a leg or reuse the live approval. Rename the local 0152 crawler from “North map” to “South map” with the original accepted Audit pair fixed. The report stays unchanged. Replace one member with an eligible same-URL backlink-check result bearing the same declared wire token. Hold the incomplete comparison for missing Audit capture proof. No additional request. |
| F-0685 | REQ-0034-W2-SCRUBBED-BUSINESS-PHOTO-PACK · DES-0034-W2-EXACT-LOCAL-PHOTO-HANDOFF | FX-0034-W2-SCRUBBED-BUSINESS-PHOTO-PACK · IT-0097-ASSETS. Use eligible role-specific logo and cover assets, a stale location, mislabeled role, invalid size, missing people rights and planted private metadata. Use only the admitted dated 0095/0097 and format profile, exact public location and scrubbed output bytes. Withhold unsafe media or expired rights. Separate manual customer-photo review and dated supplied provider status. No provider account, upload, removal or placement claim. |

[gbp-photo]: https://support.google.com/business/answer/6103862

### Newsroom, media contacts and launch evidence (691, 693, 695)

Bind each record to the active site, staff action, exact source
revision and purpose. Use 0020, 0032, 0108 and 0118 for access,
withdrawal, storage and restore. Keep private records out of the
newsroom and view statistics. These designs add no ranking input.

- **691, WordPress newsroom.** Show a local proposal for an
  index page with dated public announcements, an approved press
  kit and a public contact endpoint nominated by staff. Each
  item binds exact public sources, desired page or slug, section
  order, metadata, owner and asset rights. Preview the same
  public projection for every eligible guest and crawler.
  Staff confirm that the sources, dates, images, credit and
  links remain current. The public contact endpoint needs
  separate explicit publication evidence. It cannot come from
  the private contact list. Keep drafts and restricted archives
  outside the projection. Stay local until exact WordPress
  create or revise draft-only authority is admitted under
  0021/0028 and 0020. Bind actor and native right, site, expected
  absence or page revision, reserved slug, approved fields,
  source and rights generations, projection digest, expiry
  and signed request ID and nonce. Commit only the private
  draft and its effect receipt in the same native operation.
  Check replay, autosave, preview permissions, caches, sitemap
  and feed exclusion and measured footprint. Rights loss holds
  old previews and pending drafts. Undo preserves later staff
  work. Publishing requires separate current staff approval.
  No XenForo page action, taxonomy rewrite, remote wire,
  automatic release or permission change.
- **693, media contacts.** A local protected record has site,
  opaque ID, staff owner, approved display name, organisation,
  role, one consented contact field and optional approved
  public reference. Bind each field to explicit current
  permission with media-reference purpose, source, version,
  date and expiry. The authorised staff user enters the local
  consented record. An existing importer gains no new scope.
  Suspend expired, conflicting or uncertain consent. Withdrawal
  or expiry commits a durable use fence with the record change.
  An old import or pending edit cannot reinstate it. Recheck
  current actor, exact fields and purpose at each read, edit,
  replay and restore. Retain minimum lawful withdrawal proof
  in the protected owning store. Safe audit has no names or
  contact values. Apply 0032 erasure before restoring data.
  No identity join with CRM or follow-up lists, support-bundle
  copy, 0100 recipient selection, address harvesting, public
  directory, marketing list, export or outreach email.
  DR-116 and the 0100 sender scope continue.
- **695, rival launch notes.** DR-085 excludes scheduled rival
  and social monitoring. Analyse only the owner's authorised
  supplied public announcement evidence under 0100 and 0092.
  Bind site, exact approved rival revision, publisher channel,
  native opaque item and event or subject ID, first-observed
  date, reported launch date, coverage, rights and admission.
  No title-based identity. If item or event identity is unproved,
  show an unclassified observation. Confirm only with eligible
  provenance and staff review. Keep reported and verified time
  separate. Distinguish new launch, correction, teaser, unrelated
  mention and unknown. Repeated bytes, syndication and corrections
  update the same event when proved. They create no extra notice.
  Commit the confirmed version and at most one exact local
  0019/0032 outbox intent together. Recheck grant, source and rule
  revisions at delivery. 0099 delivers a local notice only,
  including fallback, digest and replay. 0131/0133 grants do not
  feed this view. No timer, provider pull, social app, live alert
  claim, scrape or outbound recipient. Absent evidence stays
  unknown.

### Thread sharing lists and link pages (697 and 699)

- **697, Bluesky best threads.** Under DR-076 and DR-088 use
  the local 0096 publishing pack for a staff-curated ordered
  list of guest-visible same-site threads. The native Bluesky
  custom feed route is outside version 1. [Bluesky's custom
  feed description][bsky-custom] names a separate feed generator.
  A local share list does not qualify that outside capability.
  Bind exact public thread IDs, revisions, display labels,
  approved excerpt and asset rights, and per-item staff choice.
  Freeze the ordered snapshots and exact 0096 destination,
  actor, use, approval and expiry for each pack.
  A subject-safe order from current admitted aggregate evidence
  may inform review, without affecting live ranking. No count
  implies an endorsement or verified social performance.
  Recheck content visibility and permissions per item at
  preview, copy, export and replay. A changed item holds the
  pack. Staff may post only through their own manual workflow.
  No service account, API publish, feed generator, public
  service, external polling or automatic ranking on Bluesky.
- **699, WordPress link-in-bio.** Build an editable local
  draft with approved public profile details, labelled links,
  exact ordering and optional authorised images. Keep exact
  safe HTTPS targets and use 0107 address handling without
  following links. Use 0101 for visible sponsor disclosure
  and appropriate relationship attributes where applicable.
  Same route, content version and guest scope give the same
  links and copy to every visitor and crawler. Responsive
  layout may reorder the visual placement without changing
  link meaning or reading order. No redirects, short-link
  service, profile targeting, tracking pixel or private links.
  Bind staff approval to actor, native right, exact site,
  permitted route and slug, expected draft revision or absence,
  payload digest, expiry and source rights. Keep the proposal
  local until the exact signed 0021/0028 WordPress private
  draft action is admitted. Use a conditional revision with
  one effect ID and receipt. No rewrite of a live page. Site
  staff publish separately under fresh authority. Hold on lost
  rights, expiry, stale queues, cache or restore. Keep later
  staff work. A managed public output needs exact host and
  route evidence, warm and cold cache parity, expiry proof
  and measured budget. XenForo has no link page effect.
  There is zero unrelated page footprint.

[bsky-custom]: https://bsky.social/about/blog/5-19-2023-user-faq

#### Wave 2 Bing archive alias and local media hand-off bindings

Use synthetic current site, source and rights evidence. QR-166 through
QR-168 retain exact archive, disclosure and native verification.
These fixtures grant no provider intake, polling, contact acquisition
or public write. Any separately admitted native draft uses a stand-in.

| Feature | Owning requirement and design | Planned fixture and expected evidence |
|---|---|---|
| F-0687 | REQ-0034-W2-BING-ARCHIVE-ALIAS · DES-0034-W2-EXISTING-BING-ARCHIVE-READ | FX-0034-W2-BING-ARCHIVE-ALIAS · AT-CN19, AT-CN22, AT-CN30, AT-CN35 and AT-CN37. Read only admitted copied figures from the single archive in 0040. Compare Bing surfaces, provider day and report grains with an initial gap and a moved property. Old kept data needs current archive approval for the exact former identity. Ordinary transport access, retention or restore cannot fill gaps or join old plots to the new property. The 0034 view owns the read projection. Spec 0040 owns the single archive. No new archive, file intake, provider request or assumed historical coverage. |
| F-0691 | REQ-0034-W2-EXACT-NEWSROOM-DRAFT · DES-0034-W2-PUBLIC-SOURCE-PRIVATE-DRAFT | FX-0034-W2-EXACT-NEWSROOM-DRAFT · IT-0034-W2-EXACT-NEWSROOM-DRAFT. Use exact eligible public announcements, credited assets and a separately approved synthetic public-role contact point. Exclude the private contact list. Model a reserved-slug conflict, changed native revision, lost reply, cache and sitemap exclusion and rights loss after private draft creation. Stay local until the separate signed 0021/0028 WordPress draft capability is admitted. Its receipt covers one private draft only. Preserve later staff edits. No XenForo, taxonomy, permission or public effect. |
| F-0693 | REQ-0034-W2-PRIVATE-MEDIA-CONTACT · DES-0034-W2-EXACT-MEDIA-PURPOSE | FX-0034-W2-PRIVATE-MEDIA-CONTACT · IT-0034-W2-PRIVATE-MEDIA-CONTACT. An authorised staff user enters synthetic consented media-purpose fields in one active site. Try a wrong role, conflicting consent, expired evidence, withdrawal during edit or copy and stale restore. Fence affected use and retain only permitted protected proof. Reject CRM joins, export, bundle copy and sender selection. No acquisition, newsroom publication, marketing list or outreach under DR-116. |
| F-0695 | REQ-0034-W2-SUPPLIED-RIVAL-LAUNCH · DES-0034-W2-PROVED-LOCAL-LAUNCH-NOTICE | FX-0034-W2-SUPPLIED-RIVAL-LAUNCH · IT-0034-W2-SUPPLIED-RIVAL-LAUNCH. Use an authorised supplied public announcement and separately labelled launch, teaser, repeated bytes, syndication and correction. Exact event provenance and review qualify one site-scoped local notice. Missing event identity or time stays unknown. Withdrawal fences pending delivery and dependent view. Refuse background poll and outward digest. DR-085 prevents scheduled rival monitoring. Named 0131/0133 grants do not supply this route. |
| F-0697 | REQ-0034-W2-MANUAL-THREAD-SHARE · DES-0034-W2-LOCAL-THREAD-PACK | FX-0034-W2-MANUAL-THREAD-SHARE · IT-0096-APPROVAL and IT-0096-RIGHTS. Review an ordered local pack of eligible guest-visible same-site threads. Freeze exact revisions, selected excerpts, credits, asset rights and approved use. Withdraw one item before copy, export or replay. Hold the whole affected pack. Try native Bluesky feed registration and live reload, both deferred under DR-076/088. No ranking, provider publication, follow-up or outside job. |
| F-0699 | REQ-0034-W2-EXACT-LINK-PAGE-DRAFT · DES-0034-W2-PUBLIC-SOURCE-PRIVATE-DRAFT | FX-0034-W2-EXACT-LINK-PAGE-DRAFT · IT-0034-W2-EXACT-LINK-PAGE-DRAFT. Prepare an eligible WordPress proposal from approved public fields, labelled safe links and authorised media. Compare one guest projection with paid relation evidence, changed actor, revision, cache proof and expired consent. Hold private URLs, referrer variation, stale rights and a public overwrite attempt. Stand-in 0021/0028 authority gives only one exact conditional private draft receipt while current. No XenForo write, redirect, tracking or publication. |

### Community tone and licensing agreements (701 and 705)

- **701, community sentiment by topic.** Compare the most recent
  two closed site-calendar months using approved own-forum
  guest-visible source items and unchanged topic definition,
  eligibility, method, language and disclosure profile. Pin
  the site zone and date rules. Assign a post by native publish
  instant and its exact site and stable post ID. Use one
  authorised revision in the frozen snapshot for each post
  and topic. Move, edit, hide and delete invalidate affected
  comparisons and require a new qualified correction. Use
  0097 admitted tone for the exact topic. One disjoint label
  per eligible item per topic and month. Count unread or
  ambiguous tone as unknown if eligibility is proved.
  Tone share uses the whole eligible measured sample including
  unknown classifications. Release counts, changes and
  denominators only when the joint privacy evidence admits
  them. Keep dates, labels, coverage and unknowns visible. Show each safe
  component
  separately, with no blended sentiment index, member map or
  claim to measure what people feel. Changed topic assignment,
  missing history, different sampling or sparse volumes can hold
  comparison. Apply the shared 0034 disclosure across both months,
  filters, exports and prior releases. Recheck source authority
  before use or replay. No effect on ranking or moderation.
- **705, licensing agreement tracker.** Use an immutable site
  and agreement ID under the 0098 identity contract, independent
  of counterpart names. One set of parties can have several
  agreements. Bind exact parties, evidence and term revisions,
  asset, scope, permitted purpose, channels, duration and expiry,
  territories, attribution and admitted fees. Distinguish staff
  proposal, signed, active, superseded, expired, withdrawn and
  disputed states. Keep current revision and evidence proof.
  Concurrent changes compare expected version. Resolve a
  partial asset or use withdrawal on that scope alone.
  Store the source as inert or use an approved reference.
  Separate discussion from signed terms. A recorded agreement
  cannot grant access, transfer content, approve model training
  or accept a deal. A current exact-use grant under 0097 controls
  any separately approved hand-off. Use 0098's local renewal
  reminders with exact agreement and cycle, current
  rights, evidence revisions and replay receipt.
  Withdrawal, later evidence or expiry cancels pending use.
  Apply 0032/0118 storage and erasure. No contacts or terms in
  normal reports or ranking, and no billing or third-party send.

### Exact community tone and agreement cases (701 and 705)

| Feature and design | Fixture and expected result |
|---|---|
| F-0701 · REQ-0034-W2-TOPIC-TONE · DES-0034-W2-CLOSED-COMMUNITY-TONE | FX-0034-W2-PROTECTED-MONTH-TONE and IT-0034-W2-TOPIC-TONE. Compare two closed months in the exact site clock and frozen topic profile. Place a post next to the local boundary, repeat a revision, move a post, give an ambiguous item and revoke one source. Count one disjoint label or unknown per eligible post in each month's denominator. Hold a joint release that exposes a suppressed cell across filters, prior dates or exports. No member map, blended index or ranking effect. |
| F-0705 · REQ-0034-W2-CURRENT-AGREEMENT · DES-0034-W2-EXACT-AGREEMENT-SCOPE | FX-0034-W2-AGREEMENT-WITHDRAWAL and IT-0034-W2-CURRENT-AGREEMENT. Open two agreements for the same parties under the existing 0098 agreement identities. Race edits and withdraw one asset use. The 0034 licensing scope keeps the exact agreement and term revision. Hold stale expected versions. Recheck the single 0098 renewal per current agreement, term revision and cycle on replay and restore. No second agreement identity or renewal follows from the licensing record. Review each use against its separate current 0097 rights. Hold the withdrawn scope and keep independently authorised scopes. Recording terms grants no use or hand-off. |

### Visitor analysis, report views and files (1338–1355)

Use the owner-approved active site and permitted source only. Pin
the source, source and approval versions, metric, unit, population,
dimensions, interval, time basis and zone, coverage watermark,
classification revision, scope, audience and erasure generation.
Registered metric and field combinations govern which results a
view can read. Apply typed input, output, scan and run limits before
work under 0078. A view gives no raw access, new data collection,
third-party request, provider refresh, alert send or ranking input.

Use final source partitions. Bind each event to its native UTC
instant, proved ordering and accepted source version. Group by its
declared report clock with the site zone where applicable. Date-only
or tied evidence does not prove event order. Use half-open windows
and the same exact time basis for every part of a result. A
registration, visit, page event and member are different units.
Define the eligible count denominator before any rate. Measured
zero requires complete eligible evidence. Missing, pending,
suppressed and not applicable stay distinct. Corrections supersede
the affected release and cannot introduce a second counted event.

Anonymous counts still lose their daily hash at day close. Keep the
existing within-day aggregate boundary and do not reconstruct a
visitor path, extend an anonymous visit across days, or infer a
member from a visit, address, browser or referrer. Use consented
journey evidence only while admitted and still eligible within its
existing 90-day maximum. Preserve DR-029's separate raw event
retention. Any signed-in member measure needs an independently
admitted native producer that groups eligible activity with current
consent and releases aggregates only. Member identifiers and
event-level joins remain at that producer. None may enter reports,
exports or MCP. Consent withdrawal, revocation and erasure fence
work, caches, receipts and restore under 0032 and 0118.

Apply one source disclosure record to jointly released rows,
denominators, changes, complements, alternatives and past releases.
Enforce the admitted minimum group, noise and repeated-release rules
before a chart, text, filter, file or other output becomes visible.
Suppress the affected group and derived rows where safe release is
unproved. A requested overlap, narrower filter or new presentation is
not authority to repeat noise, disclose a hidden denominator or
reconstruct suppressed data.

| Feature | Exact scope and calculation |
| --- | --- |
| 1338 Sequential segments | Match the ordered admitted page-event revisions inside one eligible proved visit, first strictly before second. Count each matching visit once, with all complete eligible visits in the chosen base. Document repeated-page and adjacency criteria. An anonymous page-pair total proves co-occurrence only. An ordered result needs a separately admitted ordered producer, consent and within-day bounds where relevant. Never join daily hashes or guess order from dates. |
| 1339 Segment comparison | Compare the same metric definition, grain, interval, source, mode and coverage across two exact visit segments. Compute each rate against its native eligible base, show absolute or relative change only where valid, and use 0066 ganit only under an admitted method for standardised differences, minimum support, variance, overlap and the multiplicity correction family. Refuse largest or significant claims from raw rates. Pin decision, thresholds and coverage. A zero reference prevents a relative change. Label overlapping bases and suppress revealing comparisons. |
| 1340 Segment overlap | For exactly two or three approved visit cohorts, form disjoint membership combinations within the admitted producer from one closed eligible base. Count each visit once in a combination. Show overlap against the declared base or eligible union and identify which basis was used. Release combinations, totals and complements together only if safe. No person or cross-source join. |
| 1341 Attribution models | Extend the existing earliest-entry report only via an admitted attribution projection. Keep native registration ID and consented touchpoints strictly inside the owning producer. Pin the same exact source and eligible base and the declared one-to-30 elapsed-day attribution horizon, independently of the 90-day retention limit. No raw ID or touchpoint appears outside the producer. First and last award one unit to the proved earliest or latest eligible touch. Even shares the unit over the same admitted touches. Time decay normalises the admitted decreasing weights with a pinned half-life and rounding rule. Each matched outcome has one total unit per model. Define ties and incomplete-order refusal before release. Report unmatched separately where safe. Modelled credit is not a causal result or a new join to accounts, orders or anonymous visits. |
| 1342 Contribution analysis | Pin two closed comparable periods, signed-off metric definition and partition. For an additive total, report each disjoint section, source or page's current minus prior value, with unknown or unassigned in the same basis. Compare alternative breakdowns separately, rather than adding overlapping dimensions. A rate needs an independently admitted decomposition from its native numerator and denominator. No inferred cause or reporting by subtraction of suppressed cells. |
| 1343 Reader flow | Show bounded aggregate predecessor or successor steps around the exact eligible page revision from the admitted producer. A node or edge uses the named anchored visits and covered step as its base. Separate an observed end from missing coverage or withheld edges. Only an exact edge proved by the admitted producer can contribute to a projected edge. A gap in coverage or a withheld edge cannot prove a substitute edge or an end. Count only the visits with admitted proof for each reported edge and keep the existing joint disclosure check. Within-day anonymous pairs prove co-occurrence only. Either flow edge needs a separately admitted ordered producer. No individual path. A saved or displayed result contains no individual path or identifiers. |
| 1344 Custom fallout | Approve a bounded ordered list of typed steps, one entry cohort, follow-up and eligible source. Count each eligible visit or consented subject once per step inside the corresponding producer. Later steps require earlier proved matches and complete follow-up. Each step's denominator is the eligible preceding stage with completed follow-up. Missing intermediate proof gives unknown, not a dropout. Registrations need the existing native joint projection. |
| 1345 Retention cohorts | Own the admitted signed-in, consented native member cohort aggregate. Views 315 and 1353 use their exact eligible projections, with 315 retaining its separate search scope. Assign first qualifying action once within proved history and exact cohort rules. Define return as a later eligible action in a closed follow-up band. Use the eligible same-cohort members with full follow-up as the denominator. Show returned and no observed return for that covered band, never a permanent departure; report time-to-action with an admitted grain and censoring treatment. Unknown first history or open follow-up cannot count as non-return. |
| 1346 Classification tables | Use a versioned owner-approved typed mapping of canonical page addresses, referring hosts or eligible source classes, with explicit priority, conflict rejection and unknown default. Apply the exact active revision consistently to every eligible report. Reclassifying prior observations requires their still-permitted fields, a separately approved rebuild, and a new release. No raw address, full referrer or personal identifier becomes a report dimension. |
| 1347 Log processing rules | Reuse 0044's exact approved log-source and QR-088 intake, opt-out and consent proof. Use closed ordered operations to clean or rename permitted fields before aggregation. Check types, collision, required provenance, field sizes and rejected rows. Rules cannot widen raw fields, alter counted time, waive proof, turn unknown into zero, or execute supplied expressions. Hold unsafe batches and checkpoint accepted input with the rule revision; retry cannot duplicate counts or revive retired approval. |
| 1348 Channel rules | Version one mutually exclusive ordered classification per admitted provider or source, with an explicit unknown category. Rules may group proved search, AI assistant, social or email facts but cannot promote a hostname or guess to verified crawler or referral evidence. Reuse 0135 and existing 0034/0040 source definitions. Each channel uses the same closed eligible attributable-plus-unknown visit base within its source. Source totals cannot be pooled or affect ranking. |
| 1349 Filtered views | Save a bounded site-scoped allowlisted rule and field projection. An owner-approved timeout applies only to a separately admitted reproducible visit projection from eligible ordered events still within their source limits. Pin timeout version, boundary and effective window. Do not re-sessionise saved aggregates, carry anonymous identities across days or change global counting or member permissions. Compare only equivalent view semantics. |
| 1350 Live spreadsheet workbook | Reuse 0100's owner-approved local file action and 0032/0118 rules. Admit an open spreadsheet format and exact schema of protected materialised tables, current scope and recipient, verified reader, size and refresh limits. Use the approved stable workbook ID, exact target generation and digest, current authorised reader and sealed receipt under bounded local 0019 execution. Stage and verify the entire replacement, recheck destination, rights and erasure and replace atomically. Hold a changed target, stale replay or unknown receipt. The open-format payload stays in admitted protected storage or sealed export. No macros, executable formulas, external links or data connections, private cells, cloud service, or plaintext fallback. Revocation or failed admission holds refresh and closes affected managed output. The previous managed file remains eligible only while its authority permits. A file already taken stays outside managed recall. |
| 1351 Time of day | Use an admitted source with the actual time precision for each metric. Mark its zone and version, and report hourly buckets or a declared weekday and weekend calendar. Distinguish repeated offset hours and gaps. Add counts only for disjoint supported bases and recompute a rate from its eligible numerator and denominator. Do not spread a daily aggregate across hours or treat a missing hour as zero. |
| 1352 Period comparison | Pin current and comparison windows and the alignment rule before reading them. An earlier or same-period last-year comparison needs comparable metric, unit, base, calendar, coverage and classified source. Define leap-day, incomplete-day and calendar-boundary treatment and display effective bounds. Report change or unknown, including a zero denominator or incompatible view. Preserve eligible absolute delta when the reference is zero and relative change is unavailable. No prorating missing coverage. |
| 1353 Return frequency bands | For the exact admitted 1345 native consented cohort projection, use a declared closed window, first eligible visit definition and disjoint bands of subsequent eligible visits. Count each member once in the appropriate band and use all eligible members with complete follow-up as its denominator. A member with no observed returns needs proven coverage. Release totals or rates only. No return history, persistent identifiers or audience lists. |
| 1354 Embedded media plays | Require an admitted own-site player and first-party counter contract, eligible item, guest-visible page and player, native and cache revision, covered interval, actual counting mode, consent, opt-outs and approved test of added queries, time and bytes. Add zero work if unsupported or over budget. Count admitted play-start events under exact start, deduplication and replay rules. Distinguish actual eligible starts from impressions, autoplay, rescans, buffering and repeated state signals. State eligible player coverage and only show a play rate with an admitted exposure denominator. An unsupported player is not measured. No remote-player fetch, tracking hook or extra beacon before admission. |
| 1355 Content traits | Bind each visible page revision and its admitted trait method to the matching eligible engagement window and page population. Compare compatible categories and use the exact released engaged-visit rate or approved bands and floor. Withhold any underlying base or numerator hidden by 0034, also across cumulative and overlapping views. Keep sections, observation times, possible confounds and overlapping traits visible. Sparse or incompatible groups show no comparison. A trait association establishes no causal effect and supplies no ranking signal. |

QR-197 qualifies limits, units, consent and disclosure on each
projection. For 1354, measure and approve the exact supported public
route, consent checks, added queries, time and bytes before any
footprint is allowed. Every other form adds zero public-page work.
Use existing 0030 file registration and 0019 receipts where applicable.
An unadmitted source or route stays unavailable.

#### Wave 4 protected analytics verification

These links bind the existing feature rules and their planned
verification under spec 0076. They reuse the current consent,
site, erasure and disclosure checks. All rows use exact fixture
scope. None adds public-page work, rank effects or route authority.

| Shared design | Applicable variant and condition |
| --- | --- |
| DES-0034-W4-AGGREGATE | VAR-0034-W4-FINAL-AGGREGATE compares or releases only the declared complete eligible final aggregate. It keeps measure, source, grain, coverage and native denominator. |
| DES-0034-W4-ORDERED | VAR-0034-W4-ORDERED-VISIT requires a separately admitted ordered visit producer. VAR-0034-W4-NATIVE-REGISTRATION and VAR-0034-W4-NATIVE-COHORT use only their respective admitted native consented producer and keep identifiers inside it. A registration stage in fallout also requires its native joint projection. |
| DES-0034-W4-PROFILE | VAR-0034-W4-TYPED-RULE applies the current bounded source and grain profile within the owning producer. None can infer order, change global counting or admit fields. VAR-0034-W4-SUPPLIED-LOG additionally requires the current approved 0044 log intake, opt-out and consent proof. |

| Feature | Requirement, design and variant | Named verification and expected result |
| --- | --- | --- |
| F-1336 | REQ-0034-W4-BREAKDOWN, DES-0034-W4-AGGREGATE, VAR-0034-W4-FINAL-AGGREGATE | [FX-0034-W4-BREAKDOWN](../planning/feature-wave-4.md#acceptance-design-for-features-1313-to-1337) · BDD-0034-W4-BREAKDOWN. Group a protected total by two compatible dimensions as assigned in the observed interval, then change one current assignment and try an overlapping roll-up or a complement. Keep the historic grouping, unknown and unassigned distinct. Hold the revealing result under the existing disclosure record. |
| F-1337 | REQ-0034-W4-SEGMENT-RULE, DES-0034-W4-PROFILE, VAR-0034-W4-TYPED-RULE | [FX-0034-W4-SEGMENT-RULE](../planning/feature-wave-4.md#acceptance-design-for-features-1313-to-1337) · IT-0034-W4-SEGMENT-RULE. Apply an eligible typed predicate inside its declared page-view, visit or admitted member-aggregate producer. Preserve a null comparison as unknown. Refuse wrong grain, a cross-day anonymous or member join, and an expired or revoked rule/cursor after restore. Return only permitted counts. |
| F-1338 | REQ-0034-W4-SEQUENCE, DES-0034-W4-ORDERED, VAR-0034-W4-ORDERED-VISIT | [FX-0034-W4-SEQUENCE](../planning/feature-wave-4.md#acceptance-design-for-features-1338-to-1357) · IT-0034-W4-SEQUENCE. Give complete visits with A before B, reversed order, equal unresolved times and repeated A. Count an eligible matching visit once in the complete eligible base. An unordered anonymous co-occurrence or absent ordered producer cannot yield an ordered count. |
| F-1339 | REQ-0034-W4-SEGMENT-COMPARE, DES-0034-W4-AGGREGATE, VAR-0034-W4-FINAL-AGGREGATE | [FX-0034-W4-SEGMENT-COMPARE](../planning/feature-wave-4.md#acceptance-design-for-features-1338-to-1357) · BDD-0034-W4-SEGMENT-COMPARE. Compare the same metric definition in two exact segments with different eligible bases. Use each native denominator. Hold unlike or revealing comparisons. A zero reference blocks relative change. Overlap is labelled, and significance waits for the exact admitted method and correction family. |
| F-1340 | REQ-0034-W4-OVERLAP, DES-0034-W4-AGGREGATE, VAR-0034-W4-FINAL-AGGREGATE | [FX-0034-W4-OVERLAP](../planning/feature-wave-4.md#acceptance-design-for-features-1338-to-1357) · IT-0034-W4-OVERLAP. Compare two and three admitted cohorts from one closed synthetic visit base with at least one overlapping visit and one visit outside all named cohorts. Assign each eligible visit to one disjoint combination within the producer. Verify the exact declared full base and eligible union as separate supported denominators where independently safe to release. The outside visit contributes to the full base only. Refuse a single cohort, fourth cohort or incompatible base. Change denominator or presentation and try combining a total or complement with a prior release to recover a protected count. Apply the same disclosure history and withhold each revealing projection before release. No person or cross-source join. |
| F-1341 | REQ-0034-W4-REGISTRATION-CREDIT, DES-0034-W4-ORDERED, VAR-0034-W4-NATIVE-REGISTRATION | [FX-0034-W4-REGISTRATION-CREDIT](../planning/feature-wave-4.md#acceptance-design-for-features-1338-to-1357) · IT-0034-W4-REGISTRATION-CREDIT. Give one eligible registration with two proved ordered touches. First credit is 1 and 0, last is 0 and 1, and even is one half each. Then try an unmatched registration, out-of-window touch, unresolved order and withdrawn consent. Each supported matched model awards one unit in total. No native ID or touchpoint leaves the producer. Hold unadmitted tie or decay rules. |
| F-1342 | REQ-0034-W4-CONTRIBUTION, DES-0034-W4-AGGREGATE, VAR-0034-W4-FINAL-AGGREGATE | [FX-0034-W4-CONTRIBUTION](../planning/feature-wave-4.md#acceptance-design-for-features-1338-to-1357) · BDD-0034-W4-CONTRIBUTION. Compare two closed additive totals with a disjoint breakdown. Give each eligible component its signed current-minus-prior delta. Hold a request to combine overlapping dimensions, a rate lacking an admitted decomposition, or subtraction that reconstructs a protected cell. |
| F-1343 | REQ-0034-W4-READER-FLOW, DES-0034-W4-ORDERED, VAR-0034-W4-ORDERED-VISIT | [FX-0034-W4-READER-FLOW](../planning/feature-wave-4.md#acceptance-design-for-features-1338-to-1357) · BDD-0034-W4-READER-FLOW. For one eligible page revision compare proved predecessor or successor observations, a proved end and missing coverage. Display bounded aggregate edges with their anchored base. In a synthetic cohort the producer proves A to B and B to C but the existing policy withholds an edge. Keep a separate branch where A to C is proved directly. Withholding must create no substitute A to C edge or ending and must not add visits to the direct A to C count. Apply joint disclosure to its eligible base. Co-occurrence alone establishes neither order nor ending. Never release an individual path or the withheld detail. |
| F-1344 | REQ-0034-W4-FALLOUT, DES-0034-W4-ORDERED, VAR-0034-W4-ORDERED-VISIT, VAR-0034-W4-NATIVE-REGISTRATION, VAR-0034-W4-NATIVE-COHORT | [FX-0034-W4-FALLOUT](../planning/feature-wave-4.md#acceptance-design-for-features-1338-to-1357) · IT-0034-W4-FALLOUT. Observe A, B and C for a fixed entry cohort with repeat, missing intermediate event and incomplete follow-up. Count each eligible unit once per step against the eligible preceding stage with completed follow-up. Missing proof gives unknown. A registration stage needs the owned native joint projection. |
| F-1345 | REQ-0034-W4-RETENTION, DES-0034-W4-ORDERED, VAR-0034-W4-NATIVE-COHORT | [FX-0034-W4-RETENTION](../planning/feature-wave-4.md#acceptance-design-for-features-1338-to-1357) · BDD-0034-W4-RETENTION. Compare a consented native cohort with proved first action and closed follow-up against missing prior history, open follow-up and a revoked grant. Use the same eligible complete-follow-up cohort as denominator. Show observed return or no observed return for its band only. |
| F-1346 | REQ-0034-W4-CLASSIFICATION, DES-0034-W4-PROFILE, VAR-0034-W4-TYPED-RULE | [FX-0034-W4-CLASSIFICATION](../planning/feature-wave-4.md#acceptance-design-for-features-1338-to-1357) · IT-0034-W4-CLASSIFICATION. Apply a current owner-approved typed rule revision, including an unclassified item. Refuse overlapping same-priority rules and any full referrer or identifier dimension. An unapproved rebuild cannot reclassify old observations. |
| F-1347 | REQ-0034-W4-LOG-RULE, DES-0034-W4-PROFILE, VAR-0034-W4-SUPPLIED-LOG | [FX-0034-W4-LOG-RULE](../planning/feature-wave-4.md#acceptance-design-for-features-1338-to-1357) · IT-0034-W4-LOG-RULE. Process one approved local batch through the typed ordered clean or rename steps. Refuse collisions, missing opt-out or consent proof, unapproved fields and executable rules. Reconcile a lost commit using the accepted input and rule revision without double counting. |
| F-1348 | REQ-0034-W4-CHANNEL, DES-0034-W4-PROFILE, VAR-0034-W4-TYPED-RULE | [FX-0034-W4-CHANNEL](../planning/feature-wave-4.md#acceptance-design-for-features-1338-to-1357) · BDD-0034-W4-CHANNEL. Classify an admitted source with proved search, AI, social or email evidence and an ambiguous host. Put each eligible source row in one category, including unknown. Do not infer a verified bot or human AI referral from a hostname, pool sources or feed ranking. |
| F-1349 | REQ-0034-W4-FILTERED-VIEW, DES-0034-W4-PROFILE, VAR-0034-W4-TYPED-RULE | [FX-0034-W4-FILTERED-VIEW](../planning/feature-wave-4.md#acceptance-design-for-features-1338-to-1357) · IT-0034-W4-FILTERED-VIEW. Use a bounded allowlisted site rule and revoke its scope before read. Hold expired permission or an unadmitted timeout projection. A version change cannot re-sessionise a saved aggregate, alter member permissions or change global counting. |
| F-1351 | REQ-0034-W4-CLOCK-VIEW, DES-0034-W4-AGGREGATE, VAR-0034-W4-FINAL-AGGREGATE | [FX-0034-W4-CLOCK-VIEW](../planning/feature-wave-4.md#acceptance-design-for-features-1338-to-1357) · BDD-0034-W4-CLOCK-VIEW. Compare an admitted hourly source with a daily-only total, repeated clock hours with different offsets and a gap. Preserve actual buckets and unknowns. Recompute rates from eligible counts. Do not split the daily total or average rates. |
| F-1352 | REQ-0034-W4-PERIOD, DES-0034-W4-AGGREGATE, VAR-0034-W4-FINAL-AGGREGATE | [FX-0034-W4-PERIOD](../planning/feature-wave-4.md#acceptance-design-for-features-1338-to-1357) · BDD-0034-W4-PERIOD. Compare closed aligned periods, a leap-day or partial-day mismatch and a complete zero reference. Show effective bounds and eligible absolute change for the comparable case. Hold the mismatches. Relative change is unavailable with a zero reference. Do not prorate missing coverage. |
| F-1353 | REQ-0034-W4-RETURN-BANDS, DES-0034-W4-ORDERED, VAR-0034-W4-NATIVE-COHORT | [FX-0034-W4-RETURN-BANDS](../planning/feature-wave-4.md#acceptance-design-for-features-1338-to-1357) · BDD-0034-W4-RETURN-BANDS. Classify each eligible member of the exact 1345 projection once in disjoint return bands. Use the complete-follow-up cohort denominator. Hold unproved no-return, revoked consent and any small-band disclosure. Return no member history. |
| F-1355 | REQ-0034-W4-TRAITS, DES-0034-W4-AGGREGATE, VAR-0034-W4-FINAL-AGGREGATE | [FX-0034-W4-TRAITS](../planning/feature-wave-4.md#acceptance-design-for-features-1338-to-1357) · BDD-0034-W4-TRAITS. Match visible page revisions and their admitted traits to compatible closed engagement observations. Hold sparse or stale input, a revealing overlap or a reconstruction across cumulative bands. Show association and coverage without causal, recommendation or ranking output. |

QR-196 retains the field and producer profile gates for 1336 and 1337.
QR-197 retains the time, source, consent and release gates for the
remaining rows. The relevant 0044 and QR-088 intake checks still apply.
Before deriving tasks, bind approved producer and profile identities,
data and UI shapes, supported limits, consent and disclosure rules
and exact fixtures. These are planned cases. The later observed
and signed evidence belongs to the exact implementation candidate.
A held fixture condition exercises only its refusal.

#### Workbook and media admission checks (1350 and 1354)

| Feature, requirement and design | Existing Wave 4 fixture and refusal boundary |
|---|---|
| F-1350 · REQ-0034-W4-WORKBOOK-REFRESH · DES-0034-W4-PROTECTED-REPLACE | [FX-0034-W4-WORKBOOK-HOLD](../planning/feature-wave-4.md#acceptance-design-for-features-1338-to-1357) · IT-0034-W4-WORKBOOK-HOLD binds the existing QR-197 fixture. Try a changed target, wrong reader or site, stale digest, revoked grant, unknown receipt, active cell content and a cloud link. Interrupt before and after replacement and restore an older generation. Keep the old managed file only while still authorised. Refuse an unadmitted refresh or reconcile the original effect without a second replacement. |
| F-1354 · REQ-0034-W4-MEDIA-PLAY · DES-0034-W4-PLAYER-COUNTER | [FX-0034-W4-MEDIA-HOLD](../planning/feature-wave-4.md#acceptance-design-for-features-1338-to-1357) · IT-0034-W4-MEDIA-HOLD binds the existing QR-197 fixture. Try an unsupported player or revision, missing applicable consent, opt-out, stale cache, repeated state and over-budget path. An unadmitted public path adds zero work and provides no observed play count. The existing positive play-start fixture needs an admitted profile. Without a compatible exposure denominator show no rate. |

The workbook stays local under DR-087. Reuse 0100's approved file
action and 0032/0118 custody and erasure. SD-32 and SD-33 establish
only their own CSV-text and destination controls. They do not
qualify an open workbook format, protected spreadsheet reader or
atomic replacement profile. Keep formulas and other active content
out under the existing workbook contract. A separate copy already
handed to an authorised recipient is outside managed recall.

The media public path stays unavailable until QR-197 admits the
exact own-site player, native and cache revision, play-start and
deduplication profile, declared counting mode and privacy checks,
and measured query, time and byte footprint. The existing
AT-AN12, AT-AN17 and AT-AN22 through AT-AN24 cases keep their
token, cache and counting authority. They do not qualify a player
or admit a new public hook.

These are structural bindings to planned refusal fixtures. Their
identities supply no admitted positive variant, implementation
readiness or observed result. Positive fixture qualification
remains under the exact QR-197 source, protection and host profiles.

### API, events and commands

- `POST /sutradhar/collect` (public, rate-limited); `POST
  /api/darshak/report`; `GET /api/darshak/realtime` (server-sent
  events); `GET /api/darshak/populations`; `GET /api/darshak/trust`;
  `POST /api/darshak/erase` (consenting visitors only); `GET
  /api/darshak/features`.
- Events: `darshak.day_aggregated`, `.day_closed`, `.thread_frozen`,
  `.thread_unfrozen`, `.salt_rotated`, `.gap_started`, `.gap_ended`.
- Commands: `sutra darshak report|populations|explain <visit>|erase`.

### Failures

- SUT-610 queue full. SUT-611 archive upload unconfirmed. SUT-612
  aggregation behind. SUT-613 report too large.
- SUT-614 ganit candidate late or refused; the verified champion remains
  unchanged and no consumer activates the candidate.
- SUT-615 erasure only partly carried out; retried.
- SUT-616 DB-IP files missing or stale: country unknown, the
  data-centre rule skipped and the gap shown.
- SUT-617 DNS unavailable: rechecked after 10 minutes, then
  UNVERIFIABLE.
- SUT-618 token key missing: counting paused, alert.
- SUT-619 privacy notice not confirmed: warning only.

### Privacy

- Anonymous visits cannot be linked to anyone once their day closes,
  so there is nothing in them to erase; darshak's privacy page says so.
  Erasure applies to consenting visitors' histories, by visitor number,
  and regenerates the affected aggregates.
- Page pairs: two pages opened in the same anonymous visit. Never linked
  across visits, days or people. Only a count per pair and day, with its
  visit count. The pair-formation rule and minimum visit floor are readiness
  blockers owned by the darshak privacy record; until both are admitted the
  pair rate shows "not measured" and no pair rows are emitted. Working data
  is dropped with the hash at day close. Only human visits count, and the
  opt-outs apply.
- Small groups: the small-group size, privacy budget, noise mechanism and
  utility evidence are readiness blockers owned by the darshak privacy record.
  Until admitted, affected reader-behaviour figures are suppressed. Once
  admitted, figures for small groups carry differential-privacy noise from
  the pinned OpenDP library so no one visitor shows; nothing under a floor
  is ever given.
- An individual path never leaves darshak. The privacy page names the
  pair count and the referring-site count.
- Proxy headers are trusted only from the configured reverse proxy.
- Only guest-visible pages carry the script and tokens; beacons naming
  any other page are refused.

### Observability

`sutradhar_darshak_visits_total{label,reason}`, `_queue_depth`,
`_open_visits`, `_frozen_threads`, `_token_failures_total{kind}`,
`_dns_checks_total{outcome}`, `_log_joins_total{outcome}` and
`_aggregation_lag_seconds`. No page, hash, address or country labels.

## Limits

- Outage queue: up to 512 MiB on disk. When full, collection stops, the
  gap is shown, and the site is never slowed.
- In the service (inside the service slice, within the memory limits in
  AGENTS.md section 5.6): event queue at most 10,000 events (about 8 MiB); DB-IP
  files about 40 to 80 MiB (to measure); data-centre and bot list automatons
  under 4 MiB; open visits under 16 MiB; the day's verification cache under 4
  MiB; the log task under 8 MiB. Aggregation streams day partitions, at most 128
  MiB above idle. ganit's jobs run in ganit's batch work.
- Beacon size 4 KiB. Raw events 395 days by default; linked journeys up
  to 90 days; eligible aggregates two years.
- Speed targets on Dell: `collect` answers within 10 ms for 99% of
  beacons at 500 a second, DNS excluded; a 90-day report for 1 million
  pages within 2 s at the 95th percentile; nightly aggregation of 5
  million visits within 20 minutes; labelling one visit within 50
  microseconds, DNS excluded.

## Footprint budget

The source's budget for readers' pages:

- Bytes: the counting script is at most 2 KB compressed, on
  guest-visible pages only; staff-only pages carry nothing.
- Time: the script loads after the page, never blocks it and never
  retries, so readers' pages get no slower.
- Requests: one uncached token fetch and two beacons per visit, each
  beacon at most 4 KiB.

Extra database queries and bytes on the site per page are readiness blockers
owned by the darshak footprint research record. The affected add-on cannot be
admitted for implementation or release until measured bounds and fixtures
exist; the missing source value is not an unlimited allowance.

### Portfolio summary read

Return human visit counts for the two most recent closed calendar
days in the site's time zone. Use admitted, final labels under the
site's counting mode. Do not substitute older dates. Exclude invalid
log approval versions and withhold the trend while their effects are
removed. A missing or incomplete day cannot support a trend.

Expose one bounded, site-scoped summary read under spec 0102. Its
source and failure cases run in IT-SS-06 with the owning module.

## How it is tested

All runs use synthetic visits, fake DNS, fake DB-IP files, fake crawler
lists, a test PostgreSQL, a fixed clock and seeded randomness. Browser
tests drive a headless browser and check the pages with an
accessibility checker.

Labels:

- AT-AN01 (property): any visit gets exactly one of the five labels and
  one reason.
- AT-AN02: a visit passing every layer is `human`, HUMAN.
- AT-AN03: Googlebot from a published Google range is
  `verified_search_crawler`, VERIFIED.
- AT-AN04: GPTBot verified by the operator's list is `ai_crawler`,
  VERIFIED; an uncheckable claim is UNVERIFIABLE.
- AT-AN05: a name on the bot list is `declared_bot`, UA_LIST.
- AT-AN06: a visit failing layer 4 only is UNCONFIRMED and stored.

Layers:

- AT-AN07: a fake Googlebot (wrong reverse name, or name resolves
  elsewhere) is FAKE_CRAWLER; both checks passing is VERIFIED, cached by
  daily hash, not address.
- AT-AN08: on the pinned list and 10,000 names, matches equal a naive
  check's.
- AT-AN09: unverified data-centre visit is DATACENTRE; a verified
  crawler from the same network is VERIFIED.
- AT-AN10 (security): visits differing only in country get the same
  label, and the layer input type has no country field (compile-time).
- AT-AN11 (browser): visible 4 seconds, or no interaction, or
  `navigator.webdriver` true, is reported and not `human`.
- AT-AN12: no token, forged signature, another page's token or an
  expired token each give TOKEN.
- AT-AN13: 3 views by one hash in a day give 1 view and 2 repeats; one
  network at 30% of a thread's human views has its excess labelled
  NETWORK_SHARE.
- AT-AN14: a burst over the rate limit labels only the excess RATE.
- AT-AN15 (contract): job 322 over its threshold gives MODEL; with no
  active result, rules 1 to 13 decide alone.

Page token:

- AT-AN16 (PHP and Rust contract): a token made by the add-on is
  accepted by Rust; one byte changed is refused.
- AT-AN17: a page from the guest cache has no token embedded; the token
  is fetched uncached; cached delivery and bad lifetime or clock windows
  are refused; page-window reuse and clock checks still hold.
- AT-AN18: during a key change, old and new keys are accepted until the
  old is retired, then only the new.

Hash, modes and privacy:

- AT-AN19: same address and browser on one day give one hash; across
  midnight, different hashes with no stored link.
- AT-AN20 (security): after rotation the old salt's memory is wiped and
  no salt is on disk, in logs or in the database; a mid-day restart
  marks the day "counting restarted".
- AT-AN21 (browser): anonymous mode writes no cookie, local or session
  storage.
- AT-AN22: `Sec-GPC: 1` or `DNT: 1` records nothing.
- AT-AN23: "require consent" on and no consent records nothing.
- AT-AN24: with consent a journey is kept; withdrawal stops it; "Delete
  my history" erases it and regenerates aggregates.
- AT-AN25 (security): no address or browser name is stored, logged,
  traced or exported.
- AT-AN26 (desktop app test): a screen showing country has the DB-IP
  attribution link.

Populations, freeze and features:

- AT-AN27 to AT-AN30: each population holds only its labels; other
  automation is broken down by reason; no screen or export adds two
  groups.
- AT-AN31: 60% bot share freezes features at the last clean day's
  values, marked with the reason; 60% from verified crawlers only does
  not freeze.
- AT-AN32: 7 clean days unfreeze; a job 176 anomaly freezes.
- AT-AN33 (contract): every offered feature is kind `rate`; no total is
  offered alone.
- AT-AN34: rates come from `human` visits only.
- AT-AN35: a frozen thread's features are its last good values.

Beacons, joins, storage and the rest:

- AT-AN36 (browser): hidden after 2 seconds with no interaction, both
  beacons are sent and the visit is stored as UNCONFIRMED.
- AT-AN37: a start beacon with no end is labelled 30 minutes later.
- AT-AN38: an admitted log line 40 seconds after a beacon gives one
  visit marked "seen in logs"; an admitted line with no beacon is its
  own visit. An opted-out, withdrawn, unknown or closed-day line adds
  no mark or count. Check both counting modes, missing source evidence,
  and retry with the same cursor after a commit. Stop, replace the
  source and shorten retention on separate runs with committed data.
  Check immediate refusal of stale derived outputs, removal and a
  rebuild that retains beacon-only visits.
- AT-AN39: started 23:59, ended 00:01, joins by visit id and keeps the
  earlier day and hash.
- AT-AN40: at the 02:00 close the salt is wiped and no stored visit of
  that day holds a hash.
- AT-AN41: an anonymous erasure request is answered "nothing to erase";
  a consenting visitor's history is erased.
- AT-AN42: the five speed fields are stored with the page type and
  appear in spec 0044's counts.
- AT-AN43: use the 90-day cap for consenting linked journeys and the
  default 395-day raw event policy in 0100. At 91 days, the journey
  link and first-party visitor number are gone. An otherwise eligible
  unlinked raw event can remain. Verify daily hashes are absent from
  both working and archived copies after day close. At the applicable
  raw or aggregate cutoff, apply the retention mask before cleanup.
  After cleanup and restore, expired rows and journeys stay unavailable.
  An eligible aggregate two years and a day old is gone. An erasure
  request still fences the associated data before routine expiry.
- AT-AN44 (chaos): with PostgreSQL stopped, the queue fills to 512 MiB,
  then SUT-610 and the gap show, and `collect` still meets its time.
- AT-AN45: a complete GA4 day shows GA4's figure, labelled, never a sum.
- AT-AN46 (gate): a matrix entry marked built without a passing test
  fails the check.
- AT-AN47 (contract): refuse a ganit result with an infinite value
  and retain the last eligible result. In a separate synthetic job 325
  case, pin its current 0066 registration and site, prepared snapshot
  and current 0019 claim. Keep the 0077 verified champion active.
  Replace the expected snapshot and invalidate the old claim before
  the finite, in-range, correctly stamped old result arrives. Return
  SUT-614 on refusal. The active champion digest stays the same and
  the feature read uses only independently eligible current serving
  inputs. The refused result supplies no new rate or series value.
  Repeat with a compatible current candidate. It stays inert until
  0077 authorises activation. Exports hold no address, hash, browser
  name or country.
- AT-AN48 (security): a page guests cannot see carries no script or
  token, and beacons naming it are refused.
- AT-AN49 (benchmark): the `collect`, aggregation and labelling times
  hold, under the 5% median rule of AGENTS.md 6.8.
- Footprint test: script bytes and requests per page stay in budget.
- Coverage follows ADR-0020. Strict-core members require 100% line, branch
  and MC/DC coverage and every killable mutant killed, with equivalent
  mutants proved and independently reviewed; other code follows the owning
  risk matrix. Property tests remain mandatory for the label order, token
  check, visit-id and log joins, hash, salt rotation and day close, opt-outs
  and consent, "country never filters", the freeze and rate-only features.
- Kani proofs where tractable: every visit gets exactly one label; a
  joined log line never adds a second view; a feature offered to ranking
  is always kind `rate`.

## Changes from the source

- Beacons reach the add-on on the site, since the site never calls the
  owner's PC; the service collects them over the signed link. The
  address is still used in memory only and dropped after labelling.
- Speed: the 5% median rule of AGENTS.md 6.8 replaces the old ratchet;
  the 20x goal is kept (spec 0007, "The 20x goal").
- Screens are the desktop app built from SDS components.
- Browser tests drive a headless browser; screen tests are Rust tests.
- The old serve and worker roles are parts of the one service in the service
  slice.

Source: `docs/specifications/signals/analytics.md`,
`docs/specifications/signals/analytics-2.md`,
`docs/specifications/signals/analytics-3.md`,
`docs/specifications/signals/analytics-4.md` and
`docs/specifications/signals/analytics-5.md` in the archive
`archive-2026-11-08-old-repo`.
