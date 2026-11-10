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
  licence must suit AGPL-3.0-or-later; the data-centre network list and
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
   anonymously, until the visitor agrees.
3. Detailed histories, in either mode, only for visitors who agree: a
   random visitor number kept in first-party storage on their device,
   journeys kept 90 days. Withdrawal stops them at once, and "Delete my
   history" erases them.
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
  darshak's log task, which holds the day's salt. A line joins a beacon
  visit when its daily hash and page id match and its time is within 2
  minutes of the start beacon; it then adds only a "seen in logs" mark,
  so no visit counts twice.
- A line that joins nothing is its own visit, labelled by rules 1 to 6,
  8 and 10. A line read after its day has closed is labelled on its own.

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
  since a reader may be counted twice.

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
country. Every result is shadow-tested, activated and can be rolled
back; a late job leaves the last good result in force.

- Job 322: bot probability per session, from behaviour, network and
  verification signals (rule 14).
- Job 323: inflation check comparing script counts, server logs and
  Search Console clicks, shown as the trust report.
- Job 324: need score per thread, from inbound links, click depth,
  impressions at positions 8 to 20 and useful but unfound content, with
  weights tuned by the link-impact job (298). It takes no darshak
  counts.
- Job 325: quality floor per target, from rates with shrinkage for
  low-traffic threads (engaged-visit rate, solved, reports, outdated).
- Also: 204 and 212 (forecasts), 215 (missing days, shown as estimates),
  161 (changepoints in organic clicks), 176 (request-rate anomalies),
  198 (traffic-source shift), 298 and 308 (effect of added links), 56
  (which added links get clicked).

### The freeze rule

- If a thread's bot share (`suspected_automation` and `declared_bot`
  visits as a share of all its visits; verified crawlers excluded)
  passes 50% in a day, or job 176 flags an anomaly, its ranking features
  freeze at the last clean day's values.
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
  human visits. No path or query string; a path only if privacy research
  allows it. Rows below the visit floor (set by research) are never
  shown.
- `darshak_day_*`: aggregates by population. `darshak_feature`: per
  thread and day, rates, frozen flag and reason. `darshak_salt_event`:
  rotations and restarts, never a salt.
- Speed fields (for spec 0044's Core Web Vitals counts): Largest
  Contentful Paint, Interaction to Next Paint, Cumulative Layout Shift,
  time to first byte and First Contentful Paint, from the browser's own
  performance interface, with the page type.
- Visits are deleted at 90 days by dropping day partitions; aggregates
  are kept two years. Raw day files go to verified object storage when
  chosen; the local copy is removed only after the checksum is
  confirmed.
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
- SUT-614 ganit result late or refused; last good kept.
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
  visit count; a pair under the minimum visits (set by research) gives
  nothing. Working data is dropped with the hash at day close. Only
  human visits count, and the opt-outs apply. Until research settles how
  a pair is formed, the pair rate shows "not measured".
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
- Beacon size 4 KiB. Detailed events 90 days; aggregates two years.
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

Extra database queries on the site per page are not set in the source;
they are set when the feature is built.

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
- AT-AN26 (Dioxus app test): a screen showing country has the DB-IP
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
- AT-AN38: a log line 40 seconds after a beacon gives one visit marked
  "seen in logs"; a line with no beacon is its own visit.
- AT-AN39: started 23:59, ended 00:01, joins by visit id and keeps the
  earlier day and hash.
- AT-AN40: at the 02:00 close the salt is wiped and no stored visit of
  that day holds a hash.
- AT-AN41: an anonymous erasure request is answered "nothing to erase";
  a consenting visitor's history is erased.
- AT-AN42: the five speed fields are stored with the page type and
  appear in spec 0044's counts.
- AT-AN43: visits 91 days old and aggregates two years and a day old
  are gone.
- AT-AN44 (chaos): with PostgreSQL stopped, the queue fills to 512 MiB,
  then SUT-610 and the gap show, and `collect` still meets its time.
- AT-AN45: a complete GA4 day shows GA4's figure, labelled, never a sum.
- AT-AN46 (gate): a matrix entry marked built without a passing test
  fails the check.
- AT-AN47 (contract): a ganit result with an infinite value is refused
  and the last good stays; exports hold no address, hash, browser name
  or country.
- AT-AN48 (security): a page guests cannot see carries no script or
  token, and beacons naming it are refused.
- AT-AN49 (benchmark): the `collect`, aggregation and labelling times
  hold, under the 5% median rule of AGENTS.md 6.8.
- Footprint test: script bytes and requests per page stay in budget.
- Coverage 100%. Rule-critical, with every mutant killed and property
  tests: the label order, token check, visit-id and log joins, hash,
  salt rotation and day close, opt-outs and consent, "country never
  filters", the freeze and rate-only features.
- Kani proofs where tractable: every visit gets exactly one label; a
  joined log line never adds a second view; a feature offered to ranking
  is always kind `rate`.

## Changes from the source

- Beacons reach the add-on on the site, since the site never calls the
  owner's PC; the service collects them over the signed link. The
  address is still used in memory only and dropped after labelling.
- Speed: the 5% median rule of AGENTS.md 6.8 replaces the old ratchet;
  the 20x goal is kept (spec 0007, "The 20x goal").
- Screens are the Dioxus app built from SDS components.
- Browser tests drive a headless browser; screen tests are Rust tests.
- The old serve and worker roles are parts of the one service in the service
  slice.

Source: `docs/specifications/signals/analytics.md`,
`docs/specifications/signals/analytics-2.md`,
`docs/specifications/signals/analytics-3.md`,
`docs/specifications/signals/analytics-4.md` and
`docs/specifications/signals/analytics-5.md` in the archive
`archive-2026-11-08-old-repo`.
