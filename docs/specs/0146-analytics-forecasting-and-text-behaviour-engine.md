# 0146: Analytics forecasting and text-behaviour engine

Tell a writer when to publish, what to refresh and where readers quit, in
plain English, from the site's own Search Console, GA4 and darshak
numbers. Heavy maths runs in Rust on the owner's PC. Every forecast
carries a range and a "how sure" label. Nothing here edits, publishes or
touches live ranking.

Spec 0040 owns the Search Console and GA4 connectors, 0034 owns darshak
and its privacy modes, 0043 owns Alekh, 0041 and 0042 own the Editorial
Coach, 0094 owns reports and experiments, 0066 owns the maths jobs, and
0104 owns Python scripts. This spec adds the engine that joins them and
the 32 capabilities below (FC-01 to FC-32). Where an older spec already
owns a result, "Who owns what" says so and this spec only adds to it.

Example: Meera runs a cooking forum with Search Console and darshak, but no
GA4. Her board shows "Publish your slow-cooker guide in the week of 3
September: demand rose from that week in both of the last two years (range: 27
August to 10 September, low confidence: only two years are saved)". The drop-
off overlay says "Blocks 7 to 9 lose 31% of readers, about twice your normal."
The GA4 cards show "Connect GA4 to see conversions". Then darshak's counts
fall to a fifth of normal for 3 days while Search Console stays steady, and
the banner reads "Your counter looks broken since 12 March; forecasts for this
site are paused". All figures are illustrative.

## Words used here

These words have one meaning in this spec.

| Word | Meaning |
|---|---|
| Capability (FC-nn) | One of the 32 numbered results this spec defines |
| Source | One of Search Console (GSC), GA4 or darshak (the local collector) |
| Final day | A source day its connector marks final (spec 0137 sense) |
| Block | One heading, paragraph, list, table or quote of a page's body |
| Anchor | A block's page id, position number and short text hash |
| Reach | The share of visits that got at least as far as a block |
| Interval | The range a result likely falls in, 80% by default (proposed) |
| Confidence object | The record every result carries: value, range, sample label, sanity status and grade |
| Sample label | "Enough data", "Thin" or "Too little", from the minimums below |
| Sanity status | "Healthy", "Caution", "Broken" or "Unchecked", from FC-32 |
| Grade | High, medium, low or withheld: how far a result may be trusted |
| Rollup | A small table of totals kept so screens never scan raw rows |
| Snapshot | A read-only file of final rows given to a Python script |
| Look-alike pages | Unchanged pages of the same category with similar position and demand |
| Edit ledger | The log of human-approved title, intro and structure edits |
| Bootstrap | Re-drawing the data many times to see how much a result could vary |
| False-alarm control | Benjamini-Hochberg: limits how many flags are chance when many are tested |
| MAD | Median absolute deviation: a spread measure that outliers cannot bend |
| Count floor | The smallest count that may be stored or shown: 5 (proposed) |
| Heavy lane | The job lane for big background work (spec 0033) |

If the owner trims scope, FC-27, FC-28 and FC-31 are the first to move to
Later.

## What this spec designs

- A Rust module `forecast` (proposed name) that stores source rows as
  per-site DuckDB tables (ADR-0021), builds rollups and runs the 32
  capabilities.
- The partial-source rules: what runs with one, two or three sources.
- The shared confidence object, small-sample rules and sanity gate.
- Screens: the Forecast and Refresh Calendar, the Paragraph Drop-Off
  Overlay, the Striking-Distance Queue, the Rewrite Lift Ledger and the
  Three-Source Sanity Banner.
- Local-only MCP tools, fenced Python hooks and the failure codes.

Non-goals, all fixed:

- No live ranking input, no ranking research, no site write, no auto
  edit and no auto publish. Every edit is a human-approved diff.
- No promise to rank. Words such as "guaranteed" never appear.
- No visitor profile, raw address, member name or private forum identity.
- No new connector. Specs 0040 and 0136 fetch; this spec reads their
  stored copies. It starts no sync without owner approval, and syncs
  follow the 0040 and DR-079 rules for what leaves the PC.
- No rival or DataForSEO inputs (DR-119: own-site evidence only).
- No model call. Everything here is statistics and text matching.
- No Matomo rows in MCP, exports or scripts (spec 0137 rule).

## Who owns what

This table shows which spec owns a topic and what 0146 adds.

| Topic | Owner | What 0146 adds |
|---|---|---|
| Striking-distance finder (feature 11, positions 8 to 20) | 0040 | FC-18 ranks positions 4 to 15 by fitted upside and missing subtopics. Its band differs on purpose (positions 4 to 7 need the FC-04 curve to price); 0040's list stays as it is |
| Cannibalised queries | 0044 | FC-21: swap history over time and the page to keep |
| 90-day traffic forecast with and without changes | 0094 | FC-01, FC-04, FC-07 supply the site's own curve and ranges. 0094 keeps the report |
| Scroll bands and exit advice (F-0053, F-0054) | 0034 | FC-09 builds on them with block-level reach, once QR-221 is answered |
| Beacon fields, privacy modes, retention | 0034 | Proposed new fields, off until the owner approves (QR-221) |
| Before-and-after edit studies in the Lab | 0138 | FC-20 and FC-24 are the plain site-owner view. The Lab keeps its settled-update studies |
| Google update windows | 0137 | Edits inside a window or its 7-day settling period are "confounded" |
| Text parsing, stems, blocks, similarity index | 0026 and 0144 | Reused as they are |
| Seasonal and bootstrap maths | 0066 (jobs 212, 129) | Callers only; thresholds and wording stay here |
| Memory, turn-taking | 0078 | The engine asks for admission; it sets no cap of its own |
| Jobs screen | 0099 | Heavy runs are ordinary jobs |
| File ledger | 0030 | Entries go through `LedgerWriter` |

## Sources and partial-source rules

Each capability lists the sources it needs. A capability runs when all
its needed sources are connected and have final data. Optional sources
add detail. A missing source is never read as zero: the card says which
source is missing, why it matters and the wizard step to connect it.

| Connected | Runs |
|---|---|
| GSC only | FC-01 to FC-06, FC-08, FC-17 to FC-20, FC-22 to FC-24, FC-29 |
| GA4 only | FC-11 (after QR-222), FC-16 (page level) |
| darshak only | FC-09, FC-10 and FC-12 (all after QR-221), FC-11 (after QR-222), FC-14, FC-15, FC-16 (block view after QR-221), FC-25, FC-30 (consented visitors only), FC-31 |
| GSC and GA4 | GSC only and GA4 only, plus FC-07, FC-13, FC-21, FC-26, FC-27, FC-28, FC-32 |
| GSC and darshak | GSC only and darshak only, plus FC-07, FC-13, FC-21, FC-26, FC-27, FC-28, FC-32 |
| GA4 and darshak | GA4 only and darshak only, plus FC-32 |
| All three | Every capability |

FC-32 compares two sources; with GSC and darshak it compares their visit
counts directly. With fewer than two sources it cannot compare anything; the
sanity status is "Unchecked" and no result may be graded above "medium".
FC-09, FC-10, FC-12, FC-16's block view and FC-14's section view need the
proposed darshak block signals (QR-221). Until the owner approves them, those
cards show "Needs block signals" (SUT-6707).

The [0150 experiment gate](0150-experiment-source-and-sensor-gates.md)
separately requires a qualified receipt from Search Console, GA4
and darshak at every scientific verdict checkpoint. FC-32 stays
an advisory diagnostic. Its ratios cannot replace experiment
coverage, impute a metric, identify the cause of measurement
skew or establish a causal outcome.

### DES-0146-SAVED-GSC-EVIDENCE

VAR-0146-GSC-SAVED-NATIVE-DAY is the existing Search Console saved
read class in the source matrix. The service reads eligible stored
0040 evidence for the authorised own site, current property and
exact projection. Retain the binding to provider-native dates and
clock, canonical source interval, surface, dimensions, aggregation,
filters, source generation and digest, watermark, coverage, current
scope and erasure. The analytic purpose and view permission must
match that projection. Query and segmented facts retain their
original scope and cannot supply an unsegmented total.

The two limits govern different operations. The published 16-month
bound applies to Search Analytics requests and actual available
history may be shorter. Read locally retained final copies within
0040's two-year class on provider dates, subject to earlier rights
and erasure. Keep missing, capped, partial, expired, erased and
disconnected evidence distinct. A short response or reaching the
page limit alone supplies no completeness proof. A zero needs an
eligible explicit value or the owning source's exact zero proof.
A gap holds operations that need complete coverage. Current
admitted native-date reports can still show their permitted facts
and gaps. A retained copy keeps its existing scope and reader
rights. Keep the connector's keep-or-delete and archive grant
checks and the forecast's existing connection prerequisites.

Preserve locally admitted source-native reporting profiles. For a
paired comparison or an operation that requires a site-calendar
day or week, require independently admitted exact common interval
coverage, compatible population and grain. Daily aggregates remain
in their original intervals. Hold the dependent calculation if a
daily value would have to be split, resampled or imputed. Keep
known missing days and the existing safe coverage reason visible.
These checks use eligible saved evidence. Any connector request
follows its existing approved 0040 job and authority.

Spec 0094 owns the 90-day organic-click report. This saved-read
class supplies only its already admitted input contract. The
separate 0150 experiment source gate retains its own profiles and
three-proof requirement.

## Architecture

```
 GSC copy (0040)   GA4 copy (0040)   darshak aggregates (0034)
        \               |                /
         '----> ingest: validate, scan, dedupe, mark final days --.
                                                                  v
   file ledger <---- per-site DuckDB tables ------------- rollup builder
   (0030)                    |                                    |
                             v                                    v
                  capability runners (FC-01..32)           sanity check (FC-32)
                             |                                    |
                 forecast confidence objects  <------- gate ------'
                             |
        .------------+-------+-----------+----------------.
        v            v                   v                v
   Editorial Coach  Alekh tables     local MCP tools   Python snapshots
   cards (0041)     (0043)           (0020, read only) (0104, read only)
```

| Part | Duty |
|---|---|
| Ingest | Reads stored connector copies and darshak aggregate tables through their owners' public APIs; rejects bad rows; never calls a service |
| Tables | One DuckDB file per site, in the site's own encrypted folder, so no cross-site read exists. Source rows are rebuildable from the connector copies |
| Rollup builder | Weekly, page, query, topic, block and daily top-URL tables. Incremental on each new final day; a full rebuild only after a method change or SUT-6716 |
| Runners | Pure Rust functions over rollups; no network, no clock but the passed-in date |
| Sanity gate | Computes FC-32 first; every result carries its status |
| Jobs | Syncs, rebuilds and yearly refits are heavy-lane jobs under 0019, 0033 and 0099 |
| Records | Results, edit ledger and sanity reports sit in PostgreSQL under row-level security |
| Vault | Credentials stay in tijori under 0040; this module never sees them |

### Module boundary and events

The module reads only public APIs: 0040 (stored copies), 0034 (darshak
aggregates and the consent store), 0144 (page text, blocks, similarity
index), 0044 (link graph), 0137 (update windows) and 0066 (maths jobs).
It offers one API: results, ledger, sanity and drop-off reads. It writes
typed, versioned outbox events in the same transaction as its records,
and consumers are safe to repeat (AGENTS.md section 5):

| Event | From, to |
|---|---|
| `EditApproved` | Editorial Coach to forecast: opens an edit-ledger row |
| `ForecastResultReady` | Forecast to Coach and Alekh: a new generation is final |
| `SanityChanged` | Forecast to Coach and Alekh: the banner state changed |

Beacon fields and `collector.block_signals` belong to darshak, not to
this module. Maths comes from ganit (0066): seasonal traffic is job 212
and bootstrap intervals are job 129. Any other fit named below (monotone
fit, survival curve, Holm and Benjamini-Hochberg control, ridge fit, MAD)
is requested from 0066 as a new numbered job with 0146 as its caller;
nothing is copied into this module. Until 0066 assigns those numbers,
FC-04, FC-05, FC-06, FC-16, FC-19, FC-20, FC-24, FC-26, FC-28 and FC-32
are blocked on that task.

Health and audit: the module reports partition integrity, the last final
day per source and the sanity state to the health page, and writes one
audit line per approval request, sync request, script change and erasure.

## Data

### Source tables (DuckDB, dictionary-coded text)

All counts are 64-bit unsigned; sums use checked arithmetic and an
overflow fails the job instead of wrapping. Floating sums use a fixed
order so results match across machines. Store each source day with
its original provider clock and interval. Store a derived
site-calendar label only where DES-0146-SAVED-GSC-EVIDENCE proves
the exact mapping. A site zone change rebuilds derived labels and
eligibility under the new calendar version and keeps the original
source intervals and coverage.

| Table | Columns |
|---|---|
| `page_dim` | site_id, page_id, url, published_on, updated_on, category, topic_id, format, language, status (live, merged, redirected, deleted), ended_on (read from 0144 and 0044; text traits for FC-28 are read from 0144, not stored here) |
| `gsc_row_day` | site_id, day, page_id, query_id, device, search_type, clicks, impressions, position_x_impressions, final |
| `gsc_dict` | query_id, text, scan_status (clean, redacted). Queries Google withheld are absent, never invented |
| `ga4_page_day` | site_id, day, page_id, device, channel, sessions, engaged_sessions, engagement_seconds, key_events, thresholded |
| `ga4_term_day` | site_id, day, page_id, term_id, count, thresholded |
| `dk_visit_day` | site_id, day, page_id, device_class, channel, visits, goal_clicks, short_visits (under 10 active seconds), active_hist and end_active_hist (counts in six bands, all visits and end-band visits), end_band_visits |
| `dk_block_day` | site_id, day, page_id, block_no, block_hash, device_class, visits, reached, copies, copy_len_band |
| `dk_link_day` | site_id, day, page_id, block_no, link_id, target_page_id, seen, clicks |
| `dk_pair_day` | site_id, day, from_page_id, to_page_id, mode (anonymous, mode 1 only, or consented), count. In mode 2 every row is consented |
| `dk_term_day` | site_id, day, from_page_id, term_id, count |
| `dk_thread_day` | site_id, day, thread_id, reads |
| `dk_assist_day` | site_id, day, page_id, assists, status (known_zero, known, unknown) |
| `dk_health_day` | site_id, day, automation_visits, total_visits. Counts only; read by FC-32 alone, never joined to another `dk_*` table |
| `term_dict` | term_id, text, scan_status (clean, redacted) |

Rules for every row: no address, no visitor or member id, no visit id, no
forum identity. Counts below five (proposed) in `dk_*` and `ga4_term_day` are
suppressed at write time, never stored then hidden. Suppression is
complementary: a kept total must not let a reader subtract the dropped cells,
so the module passes every `dk_*` table, rollup and output through the common
privacy ledger of spec 0034 (complements, roll-ups and repeated differences).
`gsc_dict.text` and `term_dict.text` pass the spec 0095 scan for names,
addresses, phone numbers and order ids at ingest, also for email addresses,
addresses with query strings and digit runs of 8 or more. A new scanner
version rescans clean rows. If the scan cannot run, the text is redacted (fail
closed), and redacted text is hidden on every card, in MCP, in exports and in
snapshots. A "forget this query" action deletes one entry everywhere and keeps
a keyed hash of it (HMAC, key in tijori) in a per-site deny list that ingest,
rebuilds and restores skip; erasing the site removes the list. Results hold
`query_id`, never text, and logs hold ids only. `block_hash` is the first 16
hex characters of SHA-256 over the block's normalised text and a per-site
salt, computed in the page script, so page text never leaves the visitor's
browser. The salt is visible to the page, so the hash is for matching only,
never for privacy; block text is public page copy. At most 64 distinct
unmatched hashes are kept per page per day (proposed); the rest go into one
"other" bucket. A hash that matches the anchor map is replaced by the block
number and dropped from the stored row after 28 days (proposed). Assists
(FC-30) and three-step paths are computed on demand from the spec 0034 consent
store. Only per-page counts are kept, rebuilt when a visitor withdraws and
expired after 90 days. A count that crosses the floor because of a withdrawal
changes in the daily batch, never live. Private-category forum threads are
never written to `dk_thread_day`.

### Records (PostgreSQL)

| Table | Columns |
|---|---|
| `forecast_result` | result_id, site_id, capability (FC-nn), subject, generation, confidence (below), inputs (table ids), data_through, sanity_id |
| `sanity_report` | sanity_id, site_id, from_day, to_day, per-source status, flags (code, day, source), reasons |
| `edit_ledger` | revision_id, site_id, page_id, kind (title, meta, intro, structure), approved_at, published_at, words_added, words_removed, blocks_moved, before_hash, after_hash, control_set_id, approver (staff id) |
| `anchor_map` | page_id, block_no, block_hash, valid_from, valid_to, status (matched, moved, orphaned) |

File ledger entries use the spec 0030 schema through `LedgerWriter`:
kind `forecast_table`, `forecast_snapshot`, `forecast_script_cache`,
`forecast_offload` or `forecast_export`; group `forecast:<site_id>`;
creator spec 0146. Erasing a site or page removes every file of these
kinds and every record above.

### Confidence object

Every forecast, estimate or verdict is stored and shown as:

| Field | Meaning |
|---|---|
| `value`, `low`, `high` | Best estimate and the interval ends (finite numbers only; NaN and infinity are refused) |
| `level` | Interval level, default 0.80; shown in `plain_text` when not default |
| `method` | Capability id and version |
| `n` | Rows, pages or weeks behind it |
| `sample_label` | enough, thin or too_little |
| `sanity_status` | healthy, caution, broken or unchecked |
| `grade` | high, medium, low or withheld |
| `sources_used` | Which of GSC, GA4, darshak fed it |
| `data_through` | Last final day used |
| `seed` | Hash of site, capability and generation, so reruns match |
| `plain_text` | The one sentence shown, built from the fields |

Grade rule: broken sanity gives "withheld" (no number is shown as
advice); caution caps the grade at "low"; unchecked caps it at "medium";
"too_little" caps it at "low" and shows the small-sample line; "thin"
caps it at "medium". There is no waiver or override of the gate or of
"withheld". A result with no interval is refused by the writer, and so
is `plain_text` with "guaranteed", "will rank" or similar wording.

## Common rules for the 32 capabilities

- Site-calendar weeks run Monday to Sunday where
  DES-0146-SAVED-GSC-EVIDENCE proves exact alignment. Only final
  days count. Use the passed-in date and pinned calendar for day
  and week bounds. Keep native-source periods under their admitted
  profiles.
- Branded queries (owner-listed brand terms and the domain name) are
  removed from CTR work (FC-04, FC-19, FC-29).
- Stems and stop words come from spec 0026 and the page's language
  pack; a page in a language with no pack shows SUT-6713 for text
  capabilities.
- Samples: the minimum each definition states is the "too_little" line; "thin"
  runs from that line up to, but not including, twice it. A definition with no
  stated minimum uses 8 final weeks and 100 rows.
- Ratios: a ratio or logarithm is never taken of zero. Use the +5
  smoothing of FC-02, or show "no data". A series that is zero for the
  whole window is "no data", not a result.
- Intervals: each definition names its method. Where it names none, use a
  seeded block bootstrap over weekly rollups (block of 4 weeks, 1,000
  resamples, proposed), for at most the 5,000 subjects with the most
  impressions per run; others are not scored and their card says "showing the
  top 5,000". FC-04 uses its 20,000 highest-impression pages. FC-06 uses at
  most 200 topics and 5,000 queries across the run. FC-20 uses the 5,000 most
  recent approved edits, FC-24 at most 1,000 drafts and FC-16 the top 5,000
  pages by visits per run. FC-28 uses 200 resamples of at most 5,000 pages and
  chooses its penalty once on the full data. Arithmetic: 5,000 subjects x
  1,000 resamples x about 104 weeks is about 5 x 10^8 additions, about a
  second per capability; if the bootstrap share passes a quarter of a rebuild,
  lower the subject cap, never the resamples. The cost counts against the
  job's budget.
- Counts and ranked lists (FC-09, FC-10, FC-12, FC-17, FC-22, FC-23,
  FC-25, FC-31) are exact. They carry low = high = value, take their
  sample label from their stated minimum and need no bootstrap. A share
  of visits shows a Wilson interval.
- False alarms: when a capability flags many pages or queries, apply
  Benjamini-Hochberg control at 0.10 across that run's candidates.
  Where tests share look-alike pages, use Benjamini-Yekutieli instead.
  Verdict words "likely lift" and "likely drop" need a 95% interval
  after control, whatever `forecast.interval_level` is.
- Maths is deterministic: a named seedable generator with a pinned
  version, results independent of thread count, and values rounded to a
  fixed precision before storing, so Linux and Windows match.
- Differences between counts (drops, gaps) are computed with checked
  signed arithmetic; a negative drop is clamped to 0 and marked
  "suppressed".
- Weeks are ISO weeks. Week 53 folds into week 52 only in the week-of-year key
  of the seasonal index (FC-01, FC-03, FC-06); rolling windows use real
  Monday-to-Sunday weeks.
- A flagged capability may never mix sources in one score: GA4 and
  darshak measures are never added together (ADR-0013).
- A planted-null fixture of 1,000 unchanged pages must yield about zero
  flags for every capability that flags items (UT-0146-MATHS).

All thresholds are "(proposed, admit by measurement)".

## The 32 capabilities

All thresholds in these tables are proposed; see the common rules.

### FC-01 seasonal admission

The proposed 104-week threshold counts eligible saved week buckets.
An observed partial week at either edge can count towards that
history threshold. Retain its exact native days and coverage.
Missing or unproved weeks remain gaps. Reaching 104 clears the
history count only. Before using any partial edge, require an
independently admitted treatment of its weekly impressions in the
yearly mean, seasonal index and advice range. Preserve the metric,
scope and compatible calendar. Withhold the seasonal index, target
week and range when the exact edge treatment or seasonal method
lacks qualification. Show the existing safe history or coverage
reason. Use only the admitted method's results.

Ganit job 212 supplies the seasonal maths identity in spec 0066.
The FC-01 evidence must separately establish the exact partial-edge
treatment, stability at the available cycle count, and held-back
performance under its scope. Keep the existing grade cap below
three cycles. The 0094 90-day report retains its own QR-067 backtest.

### Pillar 1: demand and timing

This table lists each capability, the sources it needs and how it is worked out.

| ID | Needs | Definition |
|---|---|---|
| FC-01 Publish-before-the-peak | GSC | Per topic, weekly impressions over the saved history (24 months at most, spec 0040). The proposed history check needs at least 104 eligible saved weeks, counting partial weeks at the two edges, else SUT-6705. Apply the FC-01 seasonal admission above. A year whose mean is zero is skipped. Seasonal index SI(w) is the median over years of week-w value divided by that year's mean, smoothed over 3 weeks. Peak is the week of highest SI. Rise week is the first week SI reaches 1 + 0.5 x (peak SI - 1). Ramp time R is the 90th percentile of days from publish to first position of 20 or better (FC-08; 28 days if fewer than 10 posts). Target week = rise week - ceil(R / 7). The range is the earliest and latest target over the years. With fewer than 3 cycles the grade is never above "low" (so always, at 24 months)  |
| FC-02 Rising-query scout | GSC | For each query, A is impressions in the last 4 final weeks, B the 4 before. Rising when (A + 5) / (B + 5) is at least 2.0, A is at least 50, at least 3 of the 4 weeks rose, and no single day holds over 60% of A. Rank by A - B. Needs 8 final weeks of site history |
| FC-03 Quiet decay alarm | GSC; GA4 or darshak optional | For clicks and reading seconds per visit: alarm when each of 3 consecutive final weeks is worse than the week before, and the third is worse than the median of the 8 weeks before those 3 by 10%. For impression-weighted position: the same run and 0.5 places. Baseline at least 30 clicks a week; needs 11 final weeks. Clicks and seconds are divided by the FC-01 seasonal index where one exists; position is never divided |
| FC-04 Site click curve | GSC | CTR by rounded position 1 to 20, per device, unbranded, rows with 10 or more impressions. Fit: impression-weighted monotone (never rising with worse position) regression. Interval: 200 bootstrap resamples of pages (at most 20,000) over a page x position x device rollup. A position bucket with under 5,000 impressions borrows from the nearest bucket with data and is labelled "borrowed". Gain = impressions x (CTR(new) - CTR(old)), with the interval. Average position is not a rank; the card says so |
| FC-05 Topic lifespan and refresh clock | GSC | Per page aged 12 weeks or more: plateau is the median weekly clicks of weeks 5 to 12 after publish. Lifespan ends at the start of the first run of 8 final weeks under 70% of the plateau. Pages still holding are censored; deleted, merged and redirected pages count as ended (survival curve). Per category: median and quartiles, 10 pages or more. Refresh date = last publish or refresh date + the category's 25th-percentile lifespan. If survival stays above 0.75 the card says "not enough ended pages yet" |
| FC-06 Category momentum | GSC; GA4 optional | Topic clusters come from spec 0144. Momentum = ln((last 8 final weeks + 5) / (the 8 before + 5)), clicks and impressions (and sessions when GA4 is connected), divided by the FC-01 seasonal index where one exists. Expanding at +0.18 (about +20%) or more, fading at -0.22 or less, each with a bootstrap interval over queries |
| FC-07 Draft payoff | GSC and (GA4 or darshak) | Upside = FC-04 gain for a planned page at its target position over the topic's demand. Value rate = key events per session from 10 or more look-alike published pages, shrunk to the site rate: (events + 200 x site rate) / (sessions + 200). With no GA4, darshak goal clicks per visit stand in and the card says so. Rank drafts by value range, not point value |
| FC-08 Growth-curve benchmark | GSC | For the last 50 posts plus up to 500 retired ones, cumulative impressions and clicks on day 1 to 90, with the median and 25th to 75th percentile band, 12 posts or more. Also days from publish to first position of 20 or better. A new post is "behind" when it is under the 25th percentile at three checkpoints (days 14, 28, 56). First-impression day is the stand-in for "indexed"; Search Console has no index date. The newest 3 days are not final |

FC-08 shows the historical median and middle half of the site's
peer results. Its “behind” flag describes measured checkpoints.
Keep both outside forecast calibration. The 90-day organic-click
forecast report belongs to [spec 0094](
  0094-reporting-forecasts-proof-and-experiments.md#evidence-and-planning).
Use the separate [0151 prediction intake](
 0151-site-lawbook-calibration-and-replay.md#forecast-and-programme-calibration)
only after QR-067 admits a sealed forecast issue and its matched
independent outcome profile.

### Pillar 2: reader behaviour (darshak)

This table lists each capability, the sources it needs and how it is worked out.

| ID | Needs | Definition |
|---|---|---|
| FC-09 Drop-off map | darshak block signals | Reach(b) = visits reaching block b / visits. Drop(b) = Reach(b) - Reach(b+1). Expected drop is the site's median drop for the same block position band across pages of similar length. Flag when Drop is 2 times expected or more and at least 5 points, with 200 visits or more per page and device. Extends F-0053 scroll bands |
| FC-10 Copy tracker | darshak block signals | Copies per 1,000 visits per block. Only the count, block number and a length band (short, medium, long) are stored, never text. Top 5 blocks with 10 copies or more. The Coach shows the draft's own text for that block |
| FC-11 Site-search gap | GA4 or darshak, after QR-222 | Terms searched right after reading page P (5 or more in 28 days). Gap when under half of the term's stems appear in P's block text. Redacted terms and terms under the count floor are not shown (SUT-6709). Off until QR-222 is answered |
| FC-12 Anchor click-magnet | darshak block signals | Click rate = clicks / `seen` (visits that reached the link's block). Compare with the site's median for the same block-position band. Group by normalised anchor wording. Needs 300 visits and 10 clicks per phrasing. Anchor and sentence text come from the page copy, not from visitors |
| FC-13 Quick-bounce autopsy | GSC and (GA4 or darshak) | Page is flagged when it has 100 or more clicks and 100 or more visits, and the share of visits under 10 active seconds (`short_visits` / `visits`) is above the site's 75th percentile, or, with GA4 only, its engaged-session rate is under the site's 25th percentile. Then the top 5 queries are compared with the first 100 words: each query's content stems must appear. Output names the missing words |
| FC-14 Expected vs real reading time | darshak | Expected seconds = words / 238 x 60 (a page with 0 words is "no data") (proposed; the owner may calibrate it). Real = median active seconds of visits that reached the end band, read from `end_active_hist`. Ratio under 0.35 is "skimmed", 0.35 to 1.5 "read", over 1.5 "studied". Section view uses reach only |
| FC-15 Full-read, zero-action | darshak; GA4 optional | Pages where end-band reach is above the site median and next-page clicks (from `dk_pair_day`; GA4 key events per session instead when GA4 is connected, never both in one flag) fall under the site's 25th percentile, 300 visits or more. Output: "weak ending or missing next step" |
| FC-16 Phone vs desktop | darshak block signals or GA4 | Per block (at most 500 blocks per page), phone reach - desktop reach (`device_class` phone and desktop; tablet is left out), two-proportion test, 200 visits or more per device, Holm-corrected over the page's blocks. Flag at 10 points or more and adjusted p under 0.05. Page-level GA4 engaged-session gap is shown when blocks are unavailable |

### Pillar 3: demand in the text

This table lists each capability, the sources it needs and how it is worked out.

| ID | Needs | Definition |
|---|---|---|
| FC-17 Missing-query scanner | GSC | Queries with 20 or more impressions in 90 days. "Covered": all content stems are in the body and in the title or a heading. "Buried": all in the body but not all in the title or a heading. "Missing": any stem absent. Rank by impressions. A page's stem set is computed once per body hash and cached |
| FC-18 Striking distance | GSC | Positions 4 to 15. Upside = impressions x (CTR(3) - CTR(current)) from FC-04, with the interval, paired with FC-17's missing subtopics. Spec 0040 keeps the base list (positions 8 to 20) |
| FC-19 Headline hunter | GSC | z = (observed CTR - expected) / sqrt(expected x (1 - expected) / impressions), expected from FC-04 at the page's position. A page with expected CTR of 0 or 1 is "no data".  Flag z of -2 or lower, 500 impressions or more, position 10 or better, after false-alarm control |
| FC-20 Rewrite lift ledger | GSC | Each approved edit in the edit ledger gets a "before" window (the 28 final days before the publish date) and an "after" window (28 final days from day 4 after publish), each with 100 or more impressions. Clicks and impressions are divided by the FC-01 seasonal index separately before CTR is taken. Observed change is the difference in CTR minus the same difference for 5 or more look-alike pages from the same category. The look-alike set is frozen at `approved_at`; controls edited later in either window are dropped, not replaced, the ids are recorded, and with fewer than 5 left the verdict is "no clear change". The interval comes from 1,000 resamples of the look-alike set. Words: "likely lift" or "likely drop" only when the 95% interval, after false-alarm control, excludes zero; else "no clear change". An edit inside a 0137 rollout or its 7-day settling period, a before window that overlaps one, or another edit to the page within 32 days, is "confounded" (SUT-6714). It says "observed change", never "caused" |
| FC-21 Tug-of-war | GSC and (GA4 or darshak) | Per query, the top URL by clicks per final day, from a daily top-2-URL rollup for queries with 10 or more impressions a day. A pair is flagged when two URLs swap the top place 3 or more times in 28 days and each has 10 impressions a day or more on 10 days. Keep the page with the higher engaged-session rate (GA4) or a higher end-band rate (darshak); the other is the merge candidate. Satisfaction is page-level, not per query, from one source only (GA4 if connected, else darshak),  |
| FC-22 Query-to-heading clarity | GSC | Top 10 queries by clicks. Score 1.0 if all stems are in the title, 0.75 in an H1 to H3, 0.5 in the first 100 words, else 0. Page score is the click-weighted mean; a page with no clicks scores 0 and is marked "no clicks" |
| FC-23 Question harvester | GSC | Queries starting with how, why, what, when, which, can, does, is, or containing vs, versus, cost, price, fix (lists per language pack). Candidates come from an inverted index on stems, capped at the top 10,000 queries. Cluster by Jaccard of stems at 0.6 or more (single link, 200 per cluster). The head is the highest-impression query. Output an FAQ outline ordered by summed impressions |
| FC-24 Revision-to-traffic timeline | GSC | Block-level diffs (word-level inside changed blocks; a page over 50,000 words uses block level only; a linear-space diff, and a cap of 5 million steps before it falls back to block level) between saved drafts that were later published. Each is typed (heading added or removed, block moved, intro rewritten, table added, words +/-20%) and measured as FC-20. Drafts never leave the PC. Revisions within 14 days of each other merge |

### Pillar 4: site-wide strategy

This table lists each capability, the sources it needs and how it is worked out.

| ID | Needs | Definition |
|---|---|---|
| FC-25 Reader-journey miner | darshak | Edge counts A to B per 28 days from page pairs, the top 50 edges per page kept. Pairs mean "read in one visit", with no direction. Anonymous mode (0034 mode 1) holds pairs only; in mode 2 every pair is consented-only; three-step paths use consented journeys only (spec 0034 mode 3, 90 days). Lift = P(B given A) / P(B). Keep edges with 20 or more and lift 2.0 or more. Suggest a link only where the crawl (0044) shows none |
| FC-26 Prune and merge audit | GSC and (GA4 or darshak) | 12 months of clicks, impressions, engagement from one source (GA4 if connected, else darshak), age, last update, overlap (FC-21 or 0144 similarity). Retire candidate (only when 12 final months are saved, else "not enough history"): 0 clicks, under 10 impressions, older than 12 months, assist status known_zero. FC-30 assists are a veto only, never a score input. Never mixes GA4 and darshak figures in one score. Merge: overlap pair per FC-21. Refresh: FC-03 alarm or outdated dates. A page with any clicks, any assist or an unknown assist status is never "retire". Output is advice only |
| FC-27 Format scorecard | GSC and (GA4 or darshak) | One primary format by precedence: step guide (ordered list of 3 or more), comparison (table of 2 or more columns, or "vs" in title), checklist (5 or more short list items), glossary (8 or more definitions), deep dive (twice the category median words and 5 or more H2), else other. Per category and format: median clicks per page-month and reach depth (or engaged rate, GA4), 8 pages or more per cell |
| FC-28 Style profiler | GSC and (GA4 or darshak) | Ridge regression (penalty chosen by 5-fold cross-checking over a fixed grid) of engaged rate (GA4) or end-band rate (`end_band_visits` / `visits`, darshak), never mixed in one fit, on sentence length, English grade level (English only), words per heading, tables, intro length and list share, with category effects. 10 pages per trait or more. Show only standardised effects whose bootstrap interval excludes zero, as "goes with", never "causes", after false-alarm control |
| FC-29 Zero-click filter | GSC; GA4 optional | "Mostly looked at": impressions in the top 20% and CTR under half of FC-04 expected. "Worth writing for": CTR at or above expected and the landing page's engaged-session rate at or above the site median. Engagement is page-level, weighted by the query's share of the page's clicks |
| FC-30 Upper-funnel credit | darshak consented journeys; GA4 key events optional | A page gets an assist when a consented visitor read it 30 or more active seconds, on an earlier day, within 30 days before a key event. 90-day count per page, shown only at 5 or more (the count floor). 5 or more marks it "protected from pruning"; 1 to 4 shows "assists unknown"; consented data that covers the page and shows none is status known_zero ("no assists"), the only status that allows "retire". No consented data: the capability is off and FC-26 shows "assist data missing" |
| FC-31 Forum goldmine | darshak; GSC optional | Threads with reads above the 75th percentile (and impressions when known), at most 5,000 threads per run, and no editorial page among the 5 nearest by TF-IDF cosine (spec 0144 index) at 0.7 or more. Public categories only; private categories are excluded. Thread title and public text only; author names are never read. Staff only (QR-224) |
| FC-32 Three-source sanity check | any two of GSC, GA4, darshak | Per final day: the ratio of GA4 Google-organic sessions to GSC clicks, and of darshak organic visits to GA4 sessions; with no GA4, the ratio of darshak organic visits to GSC clicks. A day with a zero on either side is a missing day (below), never a ratio. The baseline is the trailing 56 final days, flagged days excluded; it needs 28 unflagged days, else the status is Unchecked. After 28 flagged days in a row only an owner-approved, audit-logged reset restarts the baseline (not a waiver of the gate). In spec 0034 mode 2 every darshak count covers consented visitors only, so darshak ratios are Unchecked and only GSC and GA4 are compared. A day flags when ln(ratio) is more than 3.5 x 1.4826 x max(MAD, 0.05) from the baseline median. Missing day: a source has no rows, or a zero, while another does (only where the baseline is 50 clicks a day or more, proposed; on quieter sites it is Caution). With no GSC, only darshak against GA4 is compared. Tag break: GA4 (or, with no GA4, darshak) under 20% of baseline for 2 days while GSC stays within 20% of its baseline. Ad-blocker skew: darshak under 0.7 of GA4 for the whole baseline, shown as "your counter sees about X% of GA4 visits". Bot burst: darshak visits above median + 5 x 1.4826 x max(MAD, 5% of the median), or the automation share in `dk_health_day` above 30%. Slow drift: any ratio more than 15% away from its 365-day median for 28 days (a poisoned or failing source moves slowly). The 365-day median is frozen while the status is not Healthy. With under 365 final days saved, slow drift is not checked and the grade is capped at "medium" (the status stays as computed). Status: Broken if a tag break, missing day or burst touches the last 14 days; Caution on any other flag in the last 14 days, a slow drift or skew; Healthy when there are none; Unchecked with fewer than two sources |

## Budgets and speed

Designed, not run; every number is proposed until measured on the 16 GB
baseline machine. "GB" names the machine's size as in spec 0078; sizes
in the tables are binary (MiB, GiB).

Interactive cards read rollups, never raw rows. Raw rows are read only
by rebuild jobs, in 64 MiB chunks, so memory stays flat. The times below
are ceilings (targets), not measurements.

| Rows in `gsc_row_day` | Interactive card (p95) | Full rebuild (job) | Peak memory |
|---|---|---|---|
| 100,000 | 200 ms | 20 s | 128 MiB |
| 1,000,000 | 1 s | 2 min | 256 MiB |
| 10,000,000 | 5 s | 15 min | 512 MiB |
| Over 10,000,000 | Job only, with a progress bar | Several slices, not measured | 1 GiB |

Arithmetic (estimates): 10 million rows at about 40 bytes raw is about
381 MiB, and about 76 MiB at a 5 to 1 compression ratio. A 64 MiB chunk plus
dictionaries, rollup state and the bootstrap matrix explain the memory
column. Bootstrap work is bounded by the caps in the common rules and runs on
weekly rollups (FC-04 on its page x position x device rollup), never raw
rows. Capability runners run as a final step of the same heavy job and
write `forecast_result`; cards read stored results. Runner time is part
of the "Full rebuild" column, and the bootstrap share stays within a
quarter of it.

The engine asks spec 0078 for admission and sets no cap of its own. Its
request, with a Python script running beside it, is at most 1.5 GiB (proposed;
the 0078 promise, an upper bound of the table above) and is charged to the
burst pool, so it never exceeds the 6 GB default and shares the pool with a
loaded model. Every run waits for memory with a badge (SUT-6710 if it must
stop), pauses on request and takes fair turns. A job slice stops after 20
active minutes (proposed); "active" counts time the job holds a turn, not time
spent waiting. The job checkpoints every 100,000 rows and resumes as a new
slice. On resume, and before every chunk commit and checkpoint write, it
checks (in the same transaction as the commit) the table's hash and erasure
generation and restarts from scratch if either changed. Erasure cancels a
running job and deletes its checkpoint first. It is a heavy-lane job under
0033 and 0099. Only the release long lane (DR-069) runs the 10-million-row
test.

## Settings (proposed, admit by measurement)

This table lists the settings this spec adds.

| Setting | Type, default and range | Takes effect |
|---|---|---|
| `forecast.interval_level` | Site integer percent, 80, 50 to 95 | Next rebuild |
| `forecast.min_count` | Site integer, 5, 5 to 50 (never below 5) | Next write |
| `forecast.brand_terms` | Site text list, the domain name, 20 terms | Next rebuild |
| `forecast.reading_wpm` | Site integer, 238, 150 to 350 | Next rebuild |
| `forecast.search_terms` | Owner Boolean, off (QR-222). Never-changeable class (spec 0020 rule 5): the CLI, MCP and agents cannot change it | Next write |
| `forecast.forum_text` | Owner Boolean, off (QR-224). Never-changeable class (spec 0020 rule 5): the CLI, MCP and agents cannot change it | Next run |
| `forecast.offload` | Owner Boolean, off. Never-changeable class (spec 0020 rule 5): the CLI, MCP and agents cannot change it | Next job |
| `forecast.python_hooks` | Owner Boolean, off. Never-changeable class (spec 0020 rule 5): the CLI, MCP and agents cannot change it | Next script run |
| `collector.block_signals` | Owner Boolean, off. Never-changeable class (spec 0020 rule 5): the CLI, MCP and agents cannot change it | Next page load |

Apply DES-0146-SAVED-GSC-EVIDENCE to the saved history. Use
0040's source bounds and each partition's exact coverage status.
Use SUT-6705 when the yearly capability's required saved evidence
is absent. FC-01 uses its 104-week history check and keeps its
"low" cap at 24 months. Other yearly capabilities keep their
own admitted history requirements.

## Screens

All screens are plain English first. "Why?" opens the confidence object,
the inputs and the chart. SEO users can open the same table in Alekh
(spec 0043). Nothing on a screen has an "apply" button; an edit leaves
as a diff card that a human approves in the Editorial Coach.

1. **Forecast and Refresh Calendar.** A 52-week strip. Each card is one
   page or topic: "Refresh the slow-cooker guide in the week of 3 Sept"
   with the range bar and a grade chip. Cards come from FC-01, FC-05,
   FC-07 and FC-08. A "Thin data" card is greyed and says how many more
   weeks it needs (SUT-6702).
2. **Paragraph Drop-Off Overlay.** In the Editorial Coach, a thin bar
   beside each block: reach as a shade, flagged drops with "Readers leave
   here, about twice your normal", copies as a small clip mark. Phone and
   desktop toggle (FC-09, FC-10, FC-14, FC-16). A block whose text changed
   since the data shows "These numbers are for older wording" (SUT-6708).
3. **Striking-Distance Queue.** A ranked list: page, position, the
   realistic click gain as a range, and "words your page lacks" (FC-17,
   FC-18, FC-19, FC-22). Row actions open a Coach draft, nothing more.
4. **Rewrite Lift Ledger.** One row per approved edit: what changed, the
   28-day before and after, the look-alike comparison and the verdict
   words from FC-20 and FC-24. A "confounded" row says why (SUT-6714).
5. **Three-Source Sanity Banner.** Shown on every forecast screen:
   green "Your numbers agree", amber "Your counter sees about 65% of
   GA4 visits" (SUT-6704), red "Tracking looks broken since 12 March;
   forecasts for this site are paused" (SUT-6703). "Why?" shows the
   daily ratio chart. With fewer than two sources the banner says
   "We can't cross-check your numbers yet".

A source-needed card is a calm box: the missing source, what it unlocks
and a "Connect" button into the wizard of spec 0040 (SUT-6700). Data
older than 3 final days (proposed) shows SUT-6701. Offline, screens show
saved results with "Last updated" (SUT-6706). A visitor-consent gap for
paths and assists shows SUT-6715.

## Local MCP and Python

MCP tools (names proposed; spec 0020 lists them when this spec lands):

| Tool | Access |
|---|---|
| `forecast.list`, `forecast.get` | Read: results with their confidence objects |
| `forecast.dropoff` | Read: block reach, copies and flags |
| `forecast.pairs` | Read: FC-21 cannibalisation pairs |
| `forecast.ledger` | Read: the edit ledger and lift verdicts |
| `forecast.sanity` | Read: FC-32 report |
| `forecast.request_sync`, `forecast.request_rebuild` | Create an owner approval request only; nothing runs until a human approves. At most 3 open requests per site and tool; an identical open request is merged |

Read tools return at most 100 rows a call with a cursor, under a per-site rate
limit of 60 calls a minute (proposed). Every tool needs local MCP site read
permission, stops at pause-all and on a changed grant, and never returns a
secret, address, visit id or forum identity. Text that came from outside
(queries, terms, thread titles, page text) is returned only inside fenced data
fields marked "untrusted", cut at 120 characters, with control and bidi
characters stripped. Redacted text is never returned. There is no remote
relay.

Python (spec 0104, fenced sidecar): a script may add a forecasting model, an
attribution formula, an anomaly detector or an Alekh column. It works only
when `forecast.python_hooks` is on (QR-225). It receives a read-only snapshot
of final rows for one site (at most 1 GiB, else SUT-910 and no run), without
redacted text and without secrets, no network, deleted on every exit path and
listed in the file ledger. Spec 0104's caps and codes apply (time and memory
limits, SUT-910 to SUT-916); the script may not ask for more. Its output is
typed values only, at most 10,000 rows and 1 MiB (proposed), strings fenced,
cut at 120 characters and stripped like MCP text, with NaN and infinite values
refused. Output is a proposal, labelled "custom" with the script hash; the
label cannot be dropped on the way to the Coach, Alekh or MCP. A script change
is hot-reloaded atomically: the new version first passes a dry run on
synthetic rows under the same caps, and the previous version and its hash are
kept. A failed, timed-out or hash-mismatched dry run, or a crash, keeps or
restores the previous version. A rollback never restores a script cache across
an erasure generation.

## Failure codes

Codes SUT-6700 to SUT-6799 are reserved for this spec (DR-139).
SUT-6711 and SUT-6712 are not used; script refusals use SUT-910 to
SUT-916 of spec 0104.

| Code | Owner message |
|---|---|
| SUT-6700 | “This view needs {source}. Connect it to turn this on.” |
| SUT-6701 | “Your {source} data is out of date. We’ll catch up on the next sync.” |
| SUT-6702 | “There isn’t enough data yet for a confident answer. Here is what we have.” |
| SUT-6703 | “Your tracking looks broken, so forecasts are paused.” |
| SUT-6704 | “Your sources disagree a little. Treat these numbers as rough.” |
| SUT-6705 | For FC-01, “This yearly check needs at least 104 weeks of usable saved data.” Show the applicable history or coverage reason. For other yearly capabilities requiring two full years, “We need two full years of your own data to see yearly patterns.” |
| SUT-6706 | “You’re offline. This is your last saved result.” |
| SUT-6707 | “Paragraph numbers are off. Ask the site owner to turn them on.” |
| SUT-6708 | “This paragraph changed since the numbers were collected.” |
| SUT-6709 | “Some search words are hidden or switched off to protect visitors’ privacy.” |
| SUT-6710 | “This rebuild stopped to free memory. It will resume.” |
| SUT-6713 | “Text checks aren’t available for this language yet.” |
| SUT-6714 | “Two edits overlap, so we can’t tell their effects apart.” |
| SUT-6715 | “Reader paths need visitors who agreed to be remembered.” |
| SUT-6716 | “A saved data file is damaged. We’re rebuilding it.” |

Quota exhaustion and reconnect use SUT-632 and the messages of spec 0040.

## Requirements

Each row names the test that proves it.

| Requirement | Acceptance |
|---|---|
| REQ-0146-GATE | UT-0146-GATE MUST show broken sanity gives grade "withheld", caution caps at "low", unchecked caps at "medium", a result with no interval, a non-finite number or "guaranteed" wording is refused, there is no waiver of the gate, and a missing source is never zero |
| REQ-0146-SOURCES | UT-0146-SOURCES MUST run all seven source combinations and match the matrix, with the right "Connect" card for each missing source (SUT-6700) and the "Unchecked" state with one source |
| REQ-0146-MATHS | UT-0146-MATHS MUST reproduce hand-computed values for FC-01 to FC-08, FC-17 to FC-24 and FC-32 on the synthetic multi-year fixture, including threshold equalities, each minimum sample, zero-denominator cases, a zero-MAD series, DST days of 23 and 25 hours, a zone change, the planted-null set of 1,000 pages, and identical output on Linux and Windows |
| REQ-0146-STATS | UT-0146-MATHS2 MUST cover FC-25 to FC-31: edge lift and the 50-edge cap, "any clicks means never retire", format precedence, ridge with correlated traits, a planted-null set with at most 1% flagged, the zero-click classes, assists under and at 5, and forum private-category exclusion |
| REQ-0146-BLOCKS | IT-0146-BLOCKS MUST cover FC-09 to FC-16 on the synthetic block fixture: a moved block, an orphaned block, counts of 4 and 5, complementary suppression (dropped counts not recoverable by subtraction or by sliding windows), copy counts, no text stored, private-category threads never written, and a planted-null block and consent fixture with at most 1% of candidates flagged |
| REQ-0146-FUNNEL | IT-0146-FUNNEL MUST prove anonymous mode yields pairs only, three-step paths and FC-30 need consent and expire at 90 days, withdrawal rebuilds counts, withdrawal changes landing only in the daily batch, a mode-2 case (darshak ratios Unchecked), and no consent gives "off", not zero (SUT-6715) |
| REQ-0146-LEDGER | IT-0146-LEDGER MUST cover FC-20 and FC-24: look-alike choice by category and season, rollout and settling overlap, two edits within 32 days, interval excluding and including zero after false-alarm control, and wording (SUT-6714) |
| REQ-0146-SAFE | IT-0146-SAFE MUST cover site role, pause-all, vault secrets absent from logs, exports, snapshots and MCP, hostile and personal query text (redacted everywhere, "forget this query"), hostile page and thread text, bidi and control stripping, the 120-character cut, newline-free logs, export formula cells, instruction-like text returned through MCP, erasure with an interrupted cleanup, a forgotten query staying out after rebuild and restore, tab and carriage-return formula cells, the 100-row MCP cap with its rate limit, the hostile-file caps and Python string outputs fenced like MCP text |
| REQ-0146-POISON | IT-0146-POISON MUST cover a slow beacon-inflation fixture caught by the 365-day drift check, a per-token cap, automation labels excluded from `dk_*`, the 64-hash unmatched bucket, the per-visit and per-site caps, and hostile beacons (block number out of range, unknown link or page id, unknown device class, over 4 KiB or over 32 entries rejected whole) |
| REQ-0146-OFFLOAD | IT-0146-OFFLOAD MUST prove offload needs the owner switch, files are encrypted before they leave with the key in tijori, the hash is checked after download and before use, a damaged file is quarantined (SUT-6716), the key destroyed before remote deletion, and erasure removes every remote version |
| REQ-0146-SCALE | PF-0146-SCALE MUST time and measure memory at 100,000 and 1,000,000 synthetic rows on the 16 GB baseline and a workstation, and at 10,000,000 rows only in the release long lane (p50 and p95 limits written). The push tier runs only the 100,000-row case (30 runs, median, compared with the last baseline within 5%); 1,000,000 rows and above run in the release long lane (DR-069), showing flat memory and a resumed checkpoint |
| REQ-0146-PY | IT-0146-PY MUST run a good, a slow, a hanging-dry-run, a memory-hungry, a crashing, an oversize-output, a NaN-output and a secret-seeking synthetic script and check caps, rollback, the "custom" label and refusal |
| REQ-0146-OFFSTATE | UT-0146-OFFSTATE MUST prove that with `collector.block_signals`, QR-222 terms, `forecast.python_hooks` and `forecast.offload` off, the dependent cards show SUT-6707 or SUT-6709, store nothing and cannot be switched on by CLI, MCP or agents |
| REQ-0146-LIMITS | UT-0146-LIMITS MUST test `forecast.interval_level` at 49, 50, 95 and 96, `forecast.min_count` at 4, 5, 50 and 51, and `forecast.reading_wpm` at 149, 150, 350 and 351 |
| REQ-0146-EXAMPLE | FX-0146-MEERA MUST run the worked example above as a golden fixture, so the plain-English sentences cannot drift from the maths |
| REQ-0146-EVENTS | IT-0146-EVENTS MUST send each outbox event twice and expect one record, and write the record and event in one transaction |
| REQ-0146-REPLAY | QM-0146-GATE, QM-0146-ERASE and QM-0146-BUDGET, with RP-0146 replay through the Rust jobs, gate and file ledger with Quint Connect |

### Saved Search Console input trace

| Requirement and design | Source or fixture and planned evidence |
| --- | --- |
| REQ-0146-SOURCES, REQ-0146-GATE and REQ-0146-SAFE. DES-0146-SAVED-GSC-EVIDENCE and VAR-0146-GSC-SAVED-NATIVE-DAY. | SRC-0146-GSC-ARCHIVE is 0040 F-0686, REQ-0040-W2-ARCHIVE-PARTITION and DES-0040-W2-OBSERVED-HISTORY. Preserve exact stored property, grain, clock, retention, scope and generation. Reuse AT-CN24, AT-CN37 and IT-0040-W2-GSC-HISTORY for the connector evidence. |
| DES-0146-SAVED-GSC-EVIDENCE and REQ-0146-MATHS. | SRC-0146-GSC-LIMITS is SR-063/064 and RQ-017. These provide published request limits. They leave exact property coverage and the forecast method's admission to the owning records. SRC-0146-FORECAST-OWNER is 0094 F-0023 and QR-067 for the report and its backtests. |
| FX-0146-GSC-HISTORY-VALIDITY. UT-0146-SOURCES, UT-0146-GATE and UT-0146-MATHS. | Give the synthetic property a final Pacific day starting at 08:00 UTC on 5 January and ending at 08:00 UTC on 6 January, with nine eligible clicks. The native view retains nine. A site on the same admitted clock can use that exact day. Switch the site to UTC. The aggregate splits across two UTC dates, and the dependent site-day and paired checks hold. Return to the original calendar and compare the unchanged native interval. Repeat with a partial day, a 50,000-row ceiling and a proved observed zero. The gaps remain unknown and the zero stays zero. |
| FX-0146-GSC-HISTORY-VALIDITY. IT-0146-SAFE and IT-0040-W2-GSC-HISTORY. | Use a qualifying final day outside the retrieval window and within current retained scope. Compare a day past retention, wrong property, exact generation replay, a higher verified correction and a restored erased day. The eligible archived day keeps its original date. Current rights or erasure hold the affected output. Require no extra connector call or native effect from reading the result. |

## Security and privacy

- Only aggregates are stored: no raw address, visit id, visitor or
  member id, or private forum identity. Counts under five are never
  written, and suppression is complementary (see Data). The "Delete my
  history" erasure of spec 0034 reaches every count built from a
  consented journey.
- Search Console and GA4 credentials stay in tijori. The module reads
  stored copies. Credentials never appear in logs, exports, snapshots,
  Python workers or MCP output (REQ-0146-SAFE). A sync sends page
  addresses to Google only under the 0040 and DR-079 rules.
- Query text, on-site search terms, page text and thread text are
  untrusted data. They are never a prompt, never run as code, and are
  escaped on output. Queries and terms are scanned for personal data at
  ingest, and a failed or impossible scan redacts them. Bidi and
  control characters are stripped, each term is cut at 120 characters
  (proposed), and a log line holds no newline from them. Export cells
  starting with `=`, `+`, `-`, `@`, a tab or a carriage return are
  neutralised, in CSV, XLSX and Python exports.
- New collector fields (block reach, copies, link clicks) are owner-only and
  off (QR-221). A beacon is checked before it is counted: block number within
  the page's block count, link and target ids known to the site, device class
  and copy band from the allowed lists, at most 4 KiB and 32 block entries
  (proposed), at most 5 copy entries per beacon, at
  most 1,000 events per visit and at most 10,000 beacons per site and page per
  day, counted by site and page rather than by any token (proposed), and
  rejects any `block_hash` not in the page's `anchor_map` beyond the 64-per-
  day unmatched bucket. A beacon that fails is rejected whole, not trimmed.
  Copy events record a block number and a length band, never the copied text.
  Automation- labelled visits (spec 0034 labels) never enter the `dk_*` tables
  except as the counts in `dk_health_day`.
- Per-site files and keys isolate each site's tables, snapshots, script
  caches and offload files. PostgreSQL records use row-level security.
- Files read from disk, a snapshot or an offload location are hostile input:
  the hash and byte length must match the ledger before opening, the schema
  must match the allow-list, and row, column and dictionary counts and
  decompressed size are capped (at most 50 columns, 100 million rows, a 256
  MiB dictionary, 8 GiB decompressed and a 50 to 1 decompression ratio,
  proposed). A mismatch quarantines the file and raises SUT-6716.
- Offload to an external SSD or S3 is outbound content and needs the owner-
  only `forecast.offload` switch (off). Files are encrypted on the PC before
  they leave, with an authenticated cipher chosen under spec 0030 and the key
  in tijori, and listed in the ledger. The hash covers the ciphertext. Erasure
  destroys the per-site offload key first (crypto-shredding; a single page
  erasure relies on the deletion retries, and keys are never in backups) and
  then retries deletion of every remote object version; an unreachable drive
  shows "pending".
- Erasing a site, a page or the product removes every table, rollup,
  result, ledger row, snapshot, script cache and offload file through
  the file ledger. A backup cannot restore across an erasure generation.
- No output of this module is read by live ranking. The module imports
  no ranking crate and the architecture check (AGENTS.md section 5)
  refuses an import path to one.

## How it is tested

Designed, not run. Every test uses synthetic data: fictional sites,
queries, pages and drafts, built from a seeded generator (fixed seed in
each fixture name). None calls Google, GA4 or a real site.

- **UT-0146-GATE, UT-0146-SOURCES, UT-0146-MATHS, UT-0146-MATHS2,
  UT-0146-OFFSTATE, UT-0146-LIMITS:** as in the requirements table. The MATHS
  fixture has 24 months of weekly data with a planted seasonal peak
  (FC-01). Compare 103 eligible weeks, 104 eligible full weeks and
  104 eligible week buckets including a labelled partial edge. The
  103-week case gives SUT-6705. Both 104-week cases clear the history
  count. An unqualified partial-edge treatment or seasonal method
  withholds the index, target week and range. An admitted shape can
  run only within its qualified profile. Keep the existing low
  grade cap. Also use a rising query (FC-02), a 3-week
  slide (FC-03), a CTR curve with a known shape (FC-04), censored and retired
  lifespans (FC-05), planted tug-of-war, tag-break and drift days (FC-21,
  FC-32), and a planted-null set of 1,000 unchanged pages.
- **IT-0146-BLOCKS, -FUNNEL, -LEDGER, -SAFE, -POISON, -OFFLOAD, -PY,
  -EVENTS:**
  as in the requirements table.
- **PF-0146-SCALE:** fails when memory passes the admitted request or a
  budget is missed.
- **FX-0146-MEERA:** the worked example as a golden fixture.

### Quint

QM-0146-GATE: sources GSC, GA4 and darshak, each with healthy, caution,
broken or missing states; actions are sync, fail, recover, forecast,
approve and edit. Assert `NoConfidentOnBroken` (broken gives only
withheld), `MissingIsNotZero`, `NoDraftChangeWithoutApproval` and
`NoRankingOutput`.

QM-0146-ERASE: two sites, two pages, tables, ledger entries, a backup,
an offload copy and a script cache; actions are write, offload, delete
page, delete site, uninstall, crash and restore. Assert `NoOrphanFile`,
`NoStaleRestore`, `VaultSecretNeverOutside` (logs, exports, snapshots,
MCP) and `ErasureReachesOffload`.

QM-0146-BUDGET: jobs of three sizes, an admitted request, pause, cancel
and turn-taking; assert `NeverOverAdmitted`, `NoStarvation`,
`PauseHaltsBeforeNextChunk` and `AllSyncsNeedApproval`.

RP-0146 replays each trace through the Rust scheduler, gate and file
ledger with Quint Connect. Bazel proves identical behaviour, seeds,
clocks, memory and erasure on Ubuntu 26.04, Debian 13 and Windows 11.
Product checks are designed here and run with the implementation.

## Metrics

Every figure shown is listed in `gate/metrics.toml` with one source, the
decision it informs and the comparison it supports (ADR-0013). A figure
that sets one source beside another (such as the counter-to-GA4 share) is
labelled a comparison and never added to another source's number.

## Owner items (design on, product off until answered)

This table lists the owner decisions for enabling each route.
RQ-017 records the provider limits used in Settings and in 0040.

| Item | Question | Default until answered |
|---|---|---|
| QR-221 | May darshak's page script add block reach, copy counts, link clicks and an in-browser block hash to its beacon (counts only, no text, 4 KiB, 32 block entries)? | Off. FC-09, FC-10, FC-12, FC-16 block view and FC-14 section view show SUT-6707 |
| QR-222 | May GA4 and darshak on-site search terms be stored after the privacy scan and a count floor of 5? | Off. FC-11 is off and shows SUT-6709 |
| QR-223 | Do cross-day reader paths and upper-funnel credit use consented journeys only, kept 90 days? | Consented only |
| QR-224 | May staff see forum and comment thread text in the forum goldmine view, with author names never read? | Off |
| QR-225 | May Python scripts read read-only snapshots of final analytics rows, without secrets? | Off |
| QR-227 | May forecast tables be copied, encrypted, to an external SSD or S3? | Off. Nothing leaves the PC |

The owner may change these defaults without a question: the 238
words-per-minute reading speed (`forecast.reading_wpm`) and the brand
terms (`forecast.brand_terms`). Other numbers are proposed, to be admitted
by measurement.

## Changes from the source

This spec uses the repo's names: Alekh, `docs/registers/decisions.md` and
numbered specs. Multi-year Search Console history is limited to the
site's own saved copies, and cross-day paths to consented journeys, as
stated above. Capabilities are numbered FC-01 to FC-32 to avoid the
1,357-feature list.
