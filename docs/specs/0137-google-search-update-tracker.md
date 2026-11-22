# 0137: Google Search Update Tracker and content timeline

See official Google updates, your own page changes and measured search
movement on one timeline. A page published during a rollout appears
in its own group. The board gives observations without causal verdicts.

Spec 0043 owns charts and Alekh. Specs 0040 and 0136 own connectors.
DR-119 and DR-124 govern the separate advisory Lab in spec 0138.
The tracker adds no rival data, ranking input, site write or
public-page work.

Example: Ravi runs a forum. Google posts a core update with begin and
end dates, and the feed lists only the Ranking product. The board shows a
"rolling" band and a desktop notice, and the first import raises no ping.
A guide he published during the rollout sits in its own group, labelled
"Published during rollout", with no verdict. Google sets the end at
07:00 UTC on 10 March. The phase is "settling" until 07:00 UTC on 17 March
(end + 604800 seconds), then "analysis_window_open". With the default
N of 28, the board compares the 28 whole days before the rollout with
the 28 whole days after the settling deadline, once every day is final.
If Search Console cannot prove page-day coverage, the page shows
SUT-4002 and no label. The dates above are illustrative.

## Words used here

| Word | Meaning |
|---|---|
| Surface | The Search Console result type, such as Web Search or Discover |
| Settling period | Seven full days after the official rollout end |
| Final day | A source day marked final by its connector, with coverage evidence |
| Cohort | Pages grouped by when they were published or changed |

## What this spec designs

| Component | Responsibility |
|---|---|
| Rust tracker | Fixed HTTPS feed client, incident revisions, notices and clock fence |
| Rust timeline | Site-scoped CMS events, connector projections and comparisons |
| Spec 0019 scheduler | Durable poll, comparison and notice jobs, fair turns and pause |
| Tauri and Alekh | Shared chart bands, native-day points, markers and board evidence |
| Local MCP | Read-only 0020 tool for a single site's timeline and board |

## Official feed and notifications

After the owner enables tracking, schedule one shared public poll. Each
install uses one small job per slot, however many sites opt in. The Rust
client sends only a public HTTPS GET to
`status.search.google.com/incidents.json`. Admit the public address, verify
certificate and hostname, and refuse redirects. The request carries no
cookies, IDs, tokens, referrers, search terms or site data. Send `Cache-
Control: no-cache` and the saved validators when present. Bind them to the
endpoint, the accepted feed-body digest and the committed projection
generation. Accept 304 only if the validator sent with that request still
matches all three in the current committed feed receipt and its protected
projection is present. Keep the projection and digest. Renew only incidents in
that projection that meet the same-response freshness rules below. On timeout,
service error or invalid JSON, try `/en/feed.atom` once in the same slot. A
429 or valid Retry-After holds both routes until the stated deadline, clamped
to at most 24 hours (proposed). An invalid, past or missing value means the
next slot. Never forward feed data or prompts.

Accept HTTP 200 with the expected JSON or Atom media type, up to 2 MiB
of decoded data and at most 1,024 incident or entry items. Refuse XML
DTDs, entities and remote resolution, duplicate JSON keys and nesting
beyond 16. Limit JSON incident IDs to 128 ASCII alphanumerics,
titles to 512 bytes, notes to 100 of 4 KiB per incident, and
validators to 512 bytes. Pin the [JSON schema][schema] and
[product IDs][products] in the release. These admission bounds
belong to Sutradhar. A refused body keeps the accepted history
and supplies no current confirmation. Never truncate into a pass.

An Atom entry needs one valid absolute [Atom identifier][atom] of
at most 512 UTF-8 bytes, one title and one RFC 3339 update time.
Keep the ID as inert case-sensitive text in a separate namespace.
Use the optional summary up to 4 KiB. Render both text fields as
plain text. Discard links and resolve no identifier. A provisional
entry supplies no JSON revision, rollout, phase notice or Lab fact.

| Product ID | Timeline track |
|---|---|
| `rGHU1u87FJnkP6W2GwMi` | Ranking |
| `QAVfsAEBQ159b2mEWBYF` | Crawling |
| `DRyTdKyPd41QXD2hnncp` | Indexing |
| `pKUD9XkLn3TBLquSpQMD` | Serving |

Use only the nonempty deduplicated `affected_products[].id` set. Show
records on each known track with one incident identity. The ranking
comparison requires the complete set to equal the single Ranking ID. Unknown
products remain unclassified and block a comparison even in a record with
the Ranking ID. Use `external_desc` as the displayed official title, `begin`
and optional `end` as the interval and `modified` as the revision timestamp.
Save updates from `updates[]` with `created, modified, when, status, text`,
sort by that tuple in UTC. `status_impact` is an inert label. Require ID,
title, begin, modified and product IDs. Omitted end means open. Omitted
notes mean empty. Reject invalid RFC 3339 time, future time, end before
begin, duplicate incident IDs or equal-revision changed projection. Hash the
canonical projected record in field order with sorted product IDs. Ignore
`uri`. The owner can open a link built from the validated ID and fixed
origin.

Classify whole phrases “Discover update”, “core update” or “spam update”
after case-folding and whitespace collapse. If there is no unique match,
show “Ongoing ranking issue” or “Past ranking issue” according to the end
only when provider disruption status supports that classification. Otherwise
show “Ranking notice”, with open or ended as a separate date state. A dated
scope record binds the official note digest, products, surfaces, markets,
languages and other qualifiers to the exact incident revision. Only compare
if the projected data proves all qualifiers. Prefer Web Search for globally
scoped Core and Spam and Discover for Discover updates. Unconfirmed scope
holds verdicts. A limited market or language needs independently admitted
proof for that exact scope. A broader series keeps its own pane and supplies
no verdict for that incident. An unchanged scope note can carry forward only
across metadata-only edits with the same verified scope.

Bind the immutable scope record to the exact accepted incident revision
and the feed receipt for its accepted projection. Use that receipt's UTC.
A later 304 may refresh phase confirmation under the existing rules. It
cannot alter the scope record. An unknown qualifier leaves the scope
unconfirmed. Carrying forward an unchanged note creates a record for the
new revision under its current receipt. The old record stays historical.
A changed or unconfirmed scope makes the earlier comparison unavailable
as current evidence. Check the exact scope record and its supporting
aggregate coverage before presenting a current comparison or giving
typed evidence to the existing 0138 reader. A complete wider series does
not fill a narrower coverage gap. Reuse only current admitted source
coverage. A retained old revision without this proof remains
eligible history only under current read authority. Its title and
dates supply no scope proof. A correction starts no connector pull.

### Phase rules

For each Ranking-only rollout with an admitted Core, Spam or Discover
category, derive `rolling` without a current end, `settling` until
`end + 604800` UTC seconds, then `analysis_window_open`. An ongoing
ranking issue is displayed with source dates and has no rollout verdict.
Keep accepted incident revisions immutable. A newer revision with an
altered interval or missing end fences all older pending notices and
comparisons. A missing item in an otherwise good feed is unconfirmed.
Keep omitted items as historical blockers until revalidated. An empty
or reduced feed cannot re-confirm them, complete them or purge them.
An older body cannot supersede a newer accepted incident.

Require a current per-incident JSON confirmation within two hours
and a UTC lower bound at least the deadline. Anchor the lower bound
to a single valid HTTPS Date from that same revalidated response
minus 120 seconds. A nonzero, malformed or repeated Age refuses
phase freshness. Do not merge in a saved Date on 304. An absent
Age alone gives no proof. Advance the accepted bound on monotonic
elapsed time for up to two hours. Reject rollback or drift over
120 seconds. A missing or invalid Date,
restart or drift requires a fresh anchor. Show “Waiting for Google’s
status feed” and the last confirmation. Resume on the next valid poll.

| Stage | What the owner sees |
|---|---|
| Rolling | Desktop notice and banner “Google is rolling out an update. Charts may move during the rollout. Give it time before making big changes.” |
| End confirmed | Mark the official end on linked charts and show “Google says the rollout has finished. We’ll wait seven full days before comparing.” |
| Analysis window open | “The seven-day wait is over. We’ll compare the periods once enough final data arrives.” |

Persist each site's first accepted generation as its quiet import
checkpoint. Initial incidents can show bands and banners without pings.
Use a transactional outbox keyed by site, incident and stage, with a
replaceable revision before delivery. Revalidate current scope, phase,
clock, feed, grant and pause fence at enqueue, commit and send. Quiet
hours defer pings and revalidate on wake. The desktop consumer records
the stable key before acknowledging; retries cannot show it twice.
A correction marks delivered evidence superseded and updates the chart.

## Content timeline and comparisons

Show a shared cursor and time scale with rollout bands, operational
tracks and source-labelled metric panes. Open with Ranking selected.
Separate controls show Crawling, Indexing, Serving and unclassified
notices on their own tracks. Clicking a band opens the
official dates, notes and affected surfaces. Clicking a page marker
shows the exact version, its source and which interval it falls in.
Provide the same data as a keyboard-accessible table. Use the site's
authorised WP or XF native publication and revision times. Calculate
major edits by exact title change or when changed eligible body words
are both at least 100 and at least 20% of the greater revision word
count, using a versioned deterministic word diff. A crawler records
an observed change between two observations, never an exact edit
or publish time. An interval that overlaps a boundary is ambiguous.

Read only admitted aggregate rows under 0040 or 0136. No extra pulls.
The current 0040 query, country/device and appearance projections
cannot establish authoritative unfiltered page-day coverage. A final
API day, a short response and exhaustive local pagination cannot
prove that Google supplied every page row. Preserve chart panes.
Hold the affected comparison and Lab evidence with SUT-4002 until
an independently admitted source proves the exact cohort's coverage.
Do not sum query or segmented rows to supply the missing totals.
Chart Search Console `web`, `discover`, `image`, `video` and `news`
as distinct series. `googleNews` is its own pane if admitted by 0040,
and `webMultimodal` cannot be merged into Web Search. Search Console
supplies native Pacific-time days. GA4 and Matomo use their own source
days and measures, in separate panes with surface unclassified.
Only source days wholly within a window qualify. Use the embedded
time-zone database revision for each native interval, even across
daylight saving. Do not add providers, infer zero for omissions,
estimate position for a source without it, or treat source fetch
completeness as proof that Google returned every observation.

Publication in `[begin,end)` gives “Published during rollout”, without
a verdict for that event. “Between updates” uses adjacent applicable
rollouts on the same surface ordered by begin, then ID. Accept an exact
publish/edit in `[previous_end,next_begin)`. Compare the earliest and
latest seven consecutive whole days after that event and before the
next begin, without overlap. Require unchanged identity and revision
after that event and final compatible rows on all 14 days. Return
mean clicks per seven-day block and the difference, or unknown.

Use `N` final native-day intervals per comparison side, default 28 and
owner configurable from 14 to 28. Before is the last N whole days
ending by rollout start. After is the first N whole days starting at
or after the settling deadline. Wait for every day to become final.
Require one known unchanged page identity and revision across both
windows, rollout and settling. Require explicit final page-day points,
one page aggregate projection, 100 reported impressions and 20
reported clicks per window. Omitted page days or query-only totals give
no verdict. Labels show provider limits. Suppress a verdict when a
rollout or operational incident overlaps the observation or settling
intervals, or when source, property, surface, granularity, canonical
mapping or coverage changed. Show the first applicable reason in this
order: source invalid, still rolling or settling, missing data,
overlap, page change, insufficient support. Unknown survives export.

Calculate relative click change from the sum of clicks against its
non-zero before sum. Average position uses impression weighting
within each window under 0043. Improvement is before minus after.
Split after into floor(N/2) earliest days and the rest. Each half
needs one click and 40 impressions and is compared to the before
daily mean. Compare exact ratios and round for display only.

| Board filter | Decision |
|---|---|
| Sustained winners | At least 20% more clicks, both halves above the before mean, and improvement at least 3 if the surface supplies position |
| Hard drops | At least 20% fewer clicks, both halves below the before mean, and improvement at most -5 if the surface supplies position |
| Small wiggles | Absolute move at most 2 positions where supplied, click change strictly within 10%, and the post daily mean inside the admitted same-page 0043 forecast band, with no active drop alert. “Performing well. Leave it alone.” |
| Multi-update survivors | Sustained winner on at least three consecutive applicable official updates on the same surface, including this one. Missing or held verdicts break the run |
| No verdict | Every remaining case. Show the comparison measures and the first unmet condition where available |

For Discover and Google News, when the source omits position use the
click-only winners and drops with the same support and persistence.
A missing position on a surface expected to have it gives no verdict.
A board label is an observed comparison. It cannot identify a penalty
or prove an edit worked. Show windows, counts, source, receipt and
coverage beside each label. Default sort is absolute click change
descending, ties by stable site page ID. A board row can be both a
sustained winner and a survivor. Export uses 0094's protected snapshots.

## Data and access

| Record | Required fields |
|---|---|
| Feed receipt | Source enum, generation, SHA-256, bounded validators, outcome, accepted UTC and ledger ID |
| Incident revision | Google ID, provider revision UTC, body digest, title, product IDs, category, start/end UTC, source status and note tuples |
| Incident scope | Version 1, keyed by exact incident revision and accepted feed receipt ID. Copy the accepted UTC instant from that receipt, the existing SHA-256 digest of the projected official note if present, sorted deduplicated product IDs, surface, market and language scopes, any additional typed scope qualifiers admitted by the pinned extraction profile, and state `confirmed` or `unconfirmed`. Each scope axis is explicitly all, a nonempty sorted set of admitted values, or unknown. A missing or unsupported qualifier cannot supply an all-scope value. Only the current verified note and profile prove scope. |
| Site phase | Site, incident revision, policy version, phase, deadline, freshness receipt, state and delivery keys |
| Content event | Site, native page ID, revision pair, event kind, instant or observed interval, source, diff version and receipt |
| Daily point | Site, page, source, property and surface, native day and zone revision, UTC interval, clicks/impressions/position or provider visit measure, coverage, finality and input generation |
| Board row | Site, page, incident revision, incident scope reference, surface, source generation, exact-scope coverage evidence reference when proved, policy hash, bounds, measures, support, cohort, labels or reason and evidence IDs |
| Ledger entry | Site or public feed owner, artefact ID and kind, digest, encryption key ref, store, retention and erasure generation |

Site rows enforce RLS. `search_update.read_timeline` takes date bounds
(up to 180 days), source, surface, optional page and cursor.
`search_update.read_board` takes incident ID, surface, optional label and
cursor. Both return at most 100 authorised rows and a next cursor
bound to the site, filters and evidence generation. Both require local
MCP site read permission and stop on a changed grant or pause-all.
MCP and exports expose only Search Console and authorised GA4 points,
plus typed incident and board fields. Matomo stays in the owner UI.
No external text enters MCP, models or scripts. Only the separate
protected spec 0138 reader can receive typed Lab evidence. It reads
the current eligible settled generation for the active owned site's
verified Search Console property. Require authoritative, complete
page-level coverage for that exact surface and window. Lab access
cannot change the tracker filters, fill a gap with another source
or start a connector pull. The timeline API, exports and Matomo
carry no rights for this reader.

Store protected accepted projections, not raw feed bodies. Register
temps and projections under the 0030 ledger and 0032/0118 protection.
Retain 1,095 days of final evidence, plus unresolved blocker metadata.
Register one reference per project and remove the cache at zero, or
on uninstall. Revoke source generations and cursors before erasure.
Use 0118-approved SSD or S3 archives. Unreachable copies stay pending
until proved absent. A backup cannot bypass an erasure generation.

### Settings (proposed, admit by measurement)

| Setting | Type, default and range | Takes effect |
|---|---|---|
| `update_tracker.enabled` | Owner Boolean, off. Owner only. The CLI, MCP and agents cannot change it | Before next dispatch and read |
| `update_tracker.poll_minutes` | Install integer, 45, 30–60 | Next slot, one slot at a time |
| `update_tracker.window_days` | Site integer, 28, 14–28 | Fresh comparison generation |

Derive a stable per-install slot offset within the interval; never
dispatch two polls in parallel or sooner than 30 minutes apart
(proposed). A request has ten seconds (proposed). Use 128 MiB below
16 GB, 256 MiB on the 16 GB baseline or 512 MiB on an admitted
workstation under 0078 (proposed). Each comparison freezes one
generation, at most 10,000 pages and 56 measured days (proposed).
Checkpoint every 500 pages under 0019 and release the turn
(proposed). Stop at 15 active minutes or 24 hours elapsed
(proposed). Publish the
whole generation atomically; mark a capped scope explicitly. Pause,
withdrawal or lost lease stops dispatch and publication.

## Failure codes

Codes SUT-4000 to SUT-4099 are reserved for this spec (DR-123).

| Code | Owner message |
|---|---|
| SUT-4000 | “Google’s update feed is unavailable. Your saved timeline is still here.” |
| SUT-4001 | “We couldn’t confirm the update dates. Comparison is paused.” |
| SUT-4002 | “The two periods need more comparable Search Console data.” |
| SUT-4003 | “Tracking is paused. We’ll check again when it resumes.” |

## Requirements

| Requirement | Acceptance |
|---|---|
| REQ-0137-FEED | IT-0137-FEED MUST pin hosts and parse official shapes from synthetic examples. Check 200/304 against the retained projection and feed-body digest, missing projections, stale or mismatched receipt bindings, Atom-only with an inert tag identifier and optional summary, changed dates, singleton Ranking versus mixed or unknown products, hostile XML, oversize and no outbound site data. A copied Date on 304 and an aged cached response cannot renew confirmation. Test two opted-in sites, one fallback at most and no fallback on Retry-After |
| REQ-0137-TIME | IT-0137-TIME MUST cover end + 604799 and +604800 seconds, DST, rollback, stale confirmation, restart, removal, concurrent revision, quiet hours and deduplicated notices |
| REQ-0137-BOARD | UT-0137-BOARD MUST prove N boundaries, late and omitted points, all cohorts, weighted positions, equality at thresholds, unavailable positions, overlap, interrupted survivor sequence and the incident scope correction case below |
| REQ-0137-SAFE | IT-0137-SAFE MUST cover site role, pause, wrong cursor, encryption, shared feed reference counts, interrupted deletion and stale restore. No ranking or site write |
| REQ-0137-REPLAY | QM-0137 and RP-0137 MUST check the bounded Quint model and replay its traces through the Rust scheduler, timeline and file ledger with Quint Connect |

## Security and privacy

- The public feed request is a GET with no cookie, ID, token, referrer,
  search term or site data. Redirects are refused and the certificate and
  hostname are verified.
- Feed bodies are bounded and parsed safely: no XML DTD, entity or remote
  resolution, no duplicate JSON keys, nesting at most 16. A refused body
  keeps accepted history and never counts as a confirmation.
- Feed text, Atom identifiers and notes are inert plain text. Links are
  discarded and no identifier is resolved. Feed data is never forwarded
  and never becomes a prompt.
- Site rows enforce row-level security. The two MCP tools need local MCP
  site read permission and stop on a changed grant or pause-all.
- MCP and exports carry only Search Console and authorised GA4 points
  and typed incident and board fields. Matomo stays in the owner UI.
- Only the protected spec 0138 reader receives typed Lab evidence, and
  only under the conditions in "Data and access". No external text
  reaches MCP, models or scripts.
- Raw feed bodies are not stored. Accepted projections are protected
  under 0030, 0032 and 0118. Erasure revokes generations and cursors
  first, and a backup cannot bypass an erasure generation.
- The tracker adds no ranking input and no write to a site.

## How it is tested

Designed, not run. Every test uses synthetic fixtures. None calls
Google, Search Console, GA4 or Matomo.

- **IT-0137-FEED (REQ-0137-FEED):** pinned hosts and official shapes
  from synthetic examples. Covers 200 and 304, old validators, Atom-only
  entries, changed dates, singleton Ranking against mixed or unknown
  products, hostile XML, oversize bodies and no outbound site data.
  With the accepted projection and feed-body digest still current,
  a matching 304 with a fresh same-response Date can renew only
  incidents present in that projection. A missing projection, stale
  or mismatched receipt binding, copied Date or aged cached response
  cannot renew confirmation. Two opted-in sites, one fallback at
  most, and no fallback on Retry-After. Checks SUT-4000 and SUT-4001.
- **IT-0137-TIME (REQ-0137-TIME):** end + 604799 and + 604800 seconds,
  DST, rollback, stale confirmation, restart, removal, concurrent
  revision, quiet hours and deduplicated notices. Checks SUT-4001
  and SUT-4003.
- **UT-0137-BOARD (REQ-0137-BOARD):** N boundaries, late and omitted
  points, all cohorts, weighted positions, equality at thresholds,
  unavailable positions, overlap and an interrupted survivor sequence.
  Checks SUT-4002 and the first-reason order. For the incident scope
  correction case, use a synthetic Ranking-only Core incident after its
  settling deadline and a complete eligible global Web Search series.
  Accept a metadata-only revision with the same official scope note and
  qualifiers. Bind the scope record and current comparison to that revision.
  Then apply a correction that narrows the note to one market and
  leaves the reporting data unchanged. Keep the wider series visible
  in its own pane. Hold the current comparison for missing exact-scope
  coverage and show SUT-4002. An older comparison or cursor cannot
  return a current verdict. Pair with 0138 IT-0138-SEALED-SOURCE to
  check that the protected reader receives no settled evidence for the
  corrected incident from the broader series. Observe no connector pull.
- **IT-0137-SAFE (REQ-0137-SAFE):** site role, pause, wrong cursor,
  encryption, shared feed reference counts, interrupted deletion and
  stale restore. No ranking input and no site write.
- **QM-0137 and RP-0137 (REQ-0137-REPLAY):** the bounded Quint model,
  and replay of its traces through the Rust scheduler, timeline and file
  ledger with Quint Connect.
- **Setting limits:** `update_tracker.poll_minutes` at 29, 30, 60 and 61,
  `update_tracker.window_days` at 13, 14, 28 and 29, and the caps of
  10,000 pages and 56 measured days, in IT-0137-TIME and UT-0137-BOARD.

The Quint model has two sites, two incident revisions and products,
two feed kinds, two stores, and time before, at and after the deadline.
Represent two accepted projection generations and the request's bound
endpoint, validator and digest using opaque fixture identities.
Actions are enable, poll, accept, correct, pause, tick, notify, compare,
delete and restore. Assert `NoEarlyNotice`, `AtomCannotSettle`,
`RankingRevisionCurrent`, `OutageTrackIsolated`, `NoMidRolloutVerdict`
and `NoStaleRestore`. Model and replay a 304 bound to an earlier
generation after the stored projection advances. Even with
its own valid Date, it cannot renew any incident, replace the current
projection or advance a phase. A matching current response can renew
only present incidents when its own freshness evidence qualifies.
Replay correction racing a notice, pause racing a commit, and shared
cache deletion with interrupted cleanup. Use only
synthetic fixtures; Bazel must prove identical behaviour, clocks,
memory and erasure on Ubuntu 26.04, Debian 13 and Windows 11. Product
checks are designed in this spec and run with its implementation.

## Changes from the source

Atom notices remain provisional when JSON is unreachable.
Seven full days after a confirmed end opens the measurement window.
[Google's guidance][core] informs the delay. The [Search Console
contract][gsc] defines the underlying days and search types.

[schema]: https://status.search.google.com/incidents.schema.json
[products]: https://status.search.google.com/products.json
[core]: https://developers.google.com/search/docs/appearance/core-updates
[gsc]: https://developers.google.com/webmaster-tools/v1/searchanalytics/query
[atom]: https://www.rfc-editor.org/rfc/rfc4287#section-4.2.6
