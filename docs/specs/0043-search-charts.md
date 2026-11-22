# 0043: Search charts

> **Superseded in part (20 November 2026).** Chart rules written in TOML and
> the `quarry.script.*` settings are replaced by Python view and chart scripts
> (spec 0104; ADR-0021). Read those parts, and the settings table rows for
> `quarry.script.*`, as void until this spec is updated. Alekh is the new name
> of QuarryView.

Search charts answer one question for a site owner: how are my search
numbers moving over time, and is this move unusual for them? One chart
shows one number across days. On it sit comparisons (lines worked out
from the numbers, such as this week against last week), alerts, a filtering tool
that finds every series matching a test, and short chart rules the admin
writes. It is part of roadmap Phase 6; "Which spec designs each feature"
in [features.md](features.md) lists the features it designs in whole or
in part. In code it is the `quarry` module.

Example: an admin opens Search Console clicks for one forum section over
180 days. The same days of the period before run beside them as a
dashed line. A band on 3 March says "search update noted". The last
three days are shaded "still arriving". The admin sets an alert, "clicks
for a calendar week fell by more than 30% against the week before",
which is checked only once Search Console has finished every day of the
week.

## What it does

### Rules that hold throughout

- Staff-only screens, in the desktop app. Guests see nothing new.
- It reads copies of numbers and changes nothing: no page, post, link,
  setting or ranking. Monitors only alert. A chart rule can never change
  a result or a page.
- Each site and agency client keeps its own rows. A chart never mixes two
  sites. Nothing is shared between installs.
- A dependent chart starts only after every required value is admitted;
  blocked script, filter, alert or retention behaviour stays off. Anything
  not measured shows "not measured", never zero.
- Connector chart numbers come from Search Console, Bing, GA4 and
  darshak. Sources are never added together.
- With the owning source admitted, link charts may show own-site
  backlink records or permitted supplied figures from spec 0046.
  Label and separate each source. Spec 0092 owns rival sample admission.
  This screen can show manual samples through its authorised API.
  It cannot create rival chart series, monitors or alerts.
- Member text and search words never enter a chart rule. Rules see ids
  and numbers only.

The international demand map (feature 74) is after version 1
under DR-081. Existing approved country aggregates remain source
evidence. They do not start language analysis or translated editions.

### What it reuses

The visual layer's charts, loader and "as of" stamp
([0017](0017-visual-layer.md)); the connectors' daily copies and coverage
([0040](0040-search-connectors.md)); ganit job 214's late-day estimate;
darshak's visit numbers and "fewer than 10" rule
([0034](0034-analytics.md)); ganit jobs 204, 212, 161 and 176 for
forecasts, changepoints and anomalies; official update events
[0137][updates]; the audit log and undo history (180 days); the falling test,
support gate and own baseline of the site structure checks
([0047](0047-site-structure-checks.md)); the one filter form of the
action layer ([0020](0020-screens-actions-sutra-mcp.md)); saved layouts
from the fix list ([0048](0048-fix-list.md)); locked sandbox rules
([0037](0037-extension-isolation.md)); grouped alerts and quiet hours;
durable jobs and `karya.shed_order` ([0019](0019-events-jobs-recovery.md),
[0033](0033-safe-running.md)); the metric catalogue
(`catalogue/metrics.toml`); row-level security; Ganit Vikas.

New here: series state and polarity rules, the chart screen, the
comparison set, chart alert rules, the filtering screen and the chart
rule engine. No new chart library, query language, job system, sandbox,
alert list or number store.

### The chart screen

- A search box picks a series: a query, a page, a section (a forum or
  category) or the whole site. Then a number: clicks, impressions, click
  rate, average position, sign-ups from search or the human sign-up
  rate. Then a source, if more than one holds it.
- One or two panes. The earlier period, event bands and any forecast
  range sit on or under the main line. A table of the same points sits
  beside it and loads first.
- Compare with an earlier period is on by default: the chosen dates
  against the same number of days just before them, or the last
  `quarry.chart.compare_days` (about 28) closed days when no dates are
  chosen.
- Saved views per admin keep series, dates and rules. A
  view's link rebuilds it for a signed-in staff member of that site only,
  showing data "as of" the save date, with a switch to "as of now".
- Daily, weekly and monthly views. The service rolls up, never the
  window.
- Keyboard use for every action. High-contrast theme. Lines differ by
  label and dash style as well as colour.
- Loading, empty, error, retry, offline and unsaved changes are each
  shown in words. Errors appear on the chart, never as a pop-up.

### Series and sources

| Series | Source | Note |
| --- | --- | --- |
| Clicks, impressions | Search Console | Daily. Bing: its own lines |
| Average position | Search Console or admitted Bing position profile | Daily documented position semantics and weighting, shown separately |
| Click rate | Same source | Clicks over impressions, same day |
| Sign-ups from search | GA4 | By landing section |
| Human sign-up rate | darshak | Humans only |

Search Console's position is an average across searches, weighted by
impressions. It is not a rank watched each day, and the screen says so.
Search Console leaves some rare searches out of query-level numbers, so
those charts carry a note. Each series needs a catalogue entry (source,
the decision it serves, what it is compared with). No entry, no line.

### Search Console setup guardian (80)

DR-106 keeps the connector read-only. Show the selected site and
exact Search Console property, the last completed verification,
ownership and sitemap observations, and when they were read.
A stale, missing or wrong-property result says “Cannot confirm”.
A missing submitted sitemap entry only means it was not seen in
these reports. It does not prove that Google never found the file.

Address verification or ownership first when the mapped property
or owner role is unconfirmed. For the confirmed owner of that
property, guide these steps in Search Console:

1. Confirm the sitemap is published in a supported format. Use Live
   URL Inspection on its exact URL and check that **Page fetch**
   shows **Successful** without a login. A local fetch cannot prove
   what Googlebot can read.
2. Select the named property and open **Sitemaps**. Paste the exact
   URL into **Add a new sitemap**, then choose **Submit** there.
3. Check the sitemap's processing status and open its row for
   details if Google reports an error.

Use [Google help](https://support.google.com/webmasters/answer/7451001?hl=en).
Report processing errors with observation time. The existing read
connector can recheck after the owner's action. It never adds a
provider write scope, action or job.

REQ-0043-SETUP requires property-matched evidence and accurate
manual guidance. BDD-0043-SETUP tests a verified unsubmitted sitemap,
unsupported format, inaccessible file, wrong property, missing
ownership, stale results and provider error. Observe zero provider
writes, including on legacy replay. Ordinary pages gain no work.

| Requirement | Verification contract |
|---|---|
| REQ-0043-SETUP | BDD-0043-SETUP: test a verified unsubmitted sitemap, unsupported format, inaccessible file, wrong property, missing ownership, stale results and provider error. Observe zero provider writes, including on legacy replay. Ordinary pages gain no work |

#### F-0080 verification binding

Feature 80 belongs to 0043. DR-106 supersedes the catalogue's
sitemap submission clause. 0040 owns read-only property,
permission and sitemap observations. `REQ-0043-SETUP` and
`DES-0043-SETUP-GUARDIAN` cover the local diagnosis and owner
guidance. `BDD-0043-SETUP` is the governing behaviour case.

Bind the active site, selected exact Search Console property ID,
current authorised owner, 0040 connection and observation generation,
and exact sitemap URL. Display each verified observation with its
date. On a wrong property, missing ownership or expired grant,
show “Cannot confirm”. Never show foreign property or sitemap
details. Missing, stale, partial and provider-error observations
cannot establish a negative fact. When the exact sitemap is absent
from a comparable report, say only that the report did not show it.

The owner resolves permissions first. For the verified property, the
guidance explains supported format and requires the owner to confirm
**Successful** under **Page fetch** in Live URL Inspection for the
exact unauthenticated sitemap URL. A local fetch or historical
inspection supplies no live confirmation. The owner selects the named
property, opens **Sitemaps**, pastes that URL in **Add a new sitemap**
and selects **Submit** there. Show the processing observation, date
and available error details through the existing read connector.

VAR-0043-SETUP-VERIFIED concerns the named verified property
and matching current owner scope. Absent proof exposes no foreign
details and offers only relevant verification or permission help.

Reuse `BDD-0043-SETUP` with mocked observations only. Exercise a
confirmed owner and verified sitemap, a different site or alternate
property, missing and revoked ownership, an omitted entry in a
partial response, stale reads, unsupported format and failed live
fetch. Prove correct guidance and zero provider writes on current
and legacy replay. No new provider scope, action, job or public-page
work follows. QR-108 retains the applicable source and platform proof.

### Wave 1 search and change evidence bindings

Use the current authorised site, source, exact native dates and
projection, historical page or section identity and coverage. Keep
source, surface, measure and denominator visible. Incomplete,
suppressed, wrongly mapped or stale observations stay unknown.
Do not add overlapping projections or join separate totals to
invent a combined population.

| Feature and owning design | Exact fixture and planned boundary |
|---|---|
| F-0069 · REQ-0043-W1-RICH-APPEARANCE · DES-0043-W1-NATIVE-SEARCH-EVIDENCE | FX-0043-W1-RICH-APPEARANCE, IT-0043-W1-RICH-APPEARANCE and shared IT-0043-OWN. Match exact 0040 search-appearance type, page, final source window and population. Compare an observed gain, a complete proved loss and a suppressed result. Only compatible full coverage can prove a loss. Display the observed transition and support. Structured data or a chart supplies no rich-result eligibility guarantee. |
| F-0071 · REQ-0043-W1-INTENT · DES-0043-W1-TYPED-INTENT | FX-0043-W1-INTENT, BDD-0043-W1-INTENT. Compare supported informational, commercial and navigational evidence for separate page and query identities, a mixed item and an unseen query. Use only exact admitted 0044/0092 evidence and the matching item classifier and profile. A query label cannot assign a page label or describe an actual reader. No supported evidence gives unknown or ambiguous. |
| F-0072 · REQ-0043-W1-QUERY-NOTICE · DES-0043-W1-CLOSED-QUERY-RULE | FX-0043-W1-QUERY-NOTICE, IT-0043-W1-QUERY-NOTICE and shared QV01/QV11/QV13. Compare a newly observed and a rising eligible query, partial history and restart. Require an independently admitted same-scope history, rise rule and support profile. Hold without them. Under that profile count one logical notice per site, rule, query, source and period. Say “first observed” within the measured evidence. No raw term in logs or shared notices and no new delivery route. |
| F-0073 · REQ-0043-W1-DEVICE-GAP · DES-0043-W1-NATIVE-SEARCH-EVIDENCE | FX-0043-W1-DEVICE-GAP, IT-0043-W1-DEVICE-GAP and shared IT-0043-OWN. Compare mobile and desktop weighted positions for one admitted matched query and date population. Withhold when device and query appear only in separate projections, coverage is missing or the mix differs. Report the supported gap and any separately evidenced hypothesis. Device difference alone supplies no cause. In a synthetic admitted-profile control compare one page and visible query on one closed native day, with matching device scope and twenty impressions per device. The recorded averages 5 for mobile and 8 for desktop give a three-point difference with mobile better under HC06. Add a separate joint fixture on one day with queries A and B at positions 1 and 9 on both devices. Use mobile impression weights 18 and 2 and desktop weights 2 and 18. Both totals are 20 and the raw averages are 1.8 and 8.2. Refuse their gap because the query weights differ despite equal keys and totals. Keep the connection and unrelated eligible report current. Offer equal-looking separate device and query projections under that same current connection. The comparison stays unavailable. Repeat with an individually admitted earlier joint receipt after reselection of the same 0040 property. Its archive remains separately eligible under current archive rights. The current comparison stays unavailable on display, replay and restore. In a separate branch withdraw the viewer's grant while the result is staged and refuse protected data under the current scope check. Another branch keeps the eligible joint observations but omits the material-difference rule. No material flag follows. The synthetic profile supplies no threshold, real source or new request. |
| F-0074 · REQ-0043-W1-MARKET-MAP-DEFER · DES-0043-W1-MARKET-REFUSAL | FX-0043-W1-MARKET-MAP-DEFER, E2E-0043-W1-MARKET-MAP-DEFER. Preserve approved own-site country observations under 0040. Try a restored map preference and schedule, staff and agent requests and the translated-edition trigger. DR-081 holds the separate international demand map and all language and market actions. No target, view, job, query or role exposure for the deferred feature. |
| F-0075 · REQ-0043-W1-INDEXING-INTERVAL · DES-0043-W1-INSPECTION-EVIDENCE | FX-0043-W1-INDEXING-INTERVAL, IT-0043-W1-INDEXING-INTERVAL and shared 0040 AT-CN26/28. Use a trusted local publication time and exact unchanged canonical page with comparable negative then positive retained inspections. Use their observation times as interval bounds. A first positive gives only an upper bound from that origin time. Missed samples leave the wider interval. Changed revision, missing origin, reindexing or wrong property cannot establish original time to index. No added inspection, sitemap submission or indexing request. |
| F-0076 · REQ-0043-W1-DEMAND-ACTIVITY · DES-0043-W1-SECTION-COMPARISON | FX-0043-W1-DEMAND-ACTIVITY, BDD-0043-W1-DEMAND-ACTIVITY and shared IT-0043-OWN. Compare admitted impression and posting counts for two sections over matched closed dates. Hold affected results on missing coverage or a historical move without a proved section mapping. Show the two eligible measures separately in the protected comparison evidence. No automatic action, causal result or charted publishing-velocity series. |
| F-0077 · REQ-0043-W1-EVERGREEN-LABEL · DES-0043-W1-DATED-CLASSIFICATION | FX-0043-W1-EVERGREEN-LABEL, BDD-0043-W1-EVERGREEN-LABEL. Compare an eligible stable guide, a date-bound offer and contradictory dated evidence. Require the exact 0049 source and an admitted label rule and population. Show the supported category and dates or unknown. No expiry decision or rank change follows directly. RS19 and 0077 retain their separate input and promotion requirements. |
| F-0078 · REQ-0043-W1-SITE-CHANGE-BANDS · DES-0043-W1-OWN-AUDIT-EVENTS | FX-0043-W1-SITE-CHANGE-BANDS, BDD-0043-W1-SITE-CHANGE-BANDS and shared QV33. Use the entitled same site's dated theme, plugin, platform, setting and Sutradhar changes only where an admitted 0032 event and its safe scope prove the change. Replay one event and revoke another. Keep one identity per eligible event, distinguish missing evidence and prevent stale detail display. A nearby traffic change proves no cause. No new audit event capture. |

#### F-0073 device comparison evidence

0043 owns the report. Use only a current, stored comparable joint
projection from the 0040 profile required by QR-106. Require the same
owned property, source scope generation, native dates, surface,
selected page and matched query and date population for both devices.
Use HC06 polarity and the 0040 one-based API basis for all cells.
For each device, total its cell impressions as I. Sum each cell’s
recorded mean position times its impressions as P. Compare P/I
only for positive I. Refuse mixed measurement bases and URL-bulk
substitution. Use no offset conversion or unweighted mean of cell
averages. Prove complete coverage of the selected query-day cells
on both devices. Each cell must have the same
share of eligible impressions on both devices. Equal keys or total
impressions alone supply no weight-mix proof. Hold a different or
unknown mix. Do not manufacture common weights or fill absent cells.
Keep the existing comparison evidence gates. QR-106 must also bind
a material-difference rule and evidence. A chart evidence minimum or
the separate position alert rule supplies no material-gap rule.

For a currently authorised source and viewer, an unproved joint
profile, coverage or weight mix shows “Comparable device data is
unavailable.” Leave source status and separately eligible reports
under 0040. A confirmed connection or final source day cannot make
the comparison known. A missing or withdrawn grant instead uses
the current scoped refusal and discloses no protected row. Show any
independently supported hypothesis with its own source and scope.
After reselection of the exact same property, apply 0040 AT-CN25.
A separately authorised older report keeps its original dates and
generation. Its equal values cannot support a current comparison.
Recheck before display, replay and restore. The report adds no
provider fetch, crawl or public-page work.

The named fixtures describe existing scenarios or protected refusal
cases. Source profiles, comparison rules and any protected notice
scope still need their exact admission under QR-106 and QR-108.
F-0080 stays under DR-106 and its separate manual sitemap guidance.

### Rank and visibility views (223–234)

Open Search charts, then choose “Search performance” or “Supplied
search results”. Each view shows the site, source, observation
scope, date basis, covered dates and any omitted evidence before
the chart or table. The evidence table has the same data and
filter state as the chart. A source switch clears filters that
would change meaning. No public-page queries, bytes or processing
are added by these staff views.

#### Search performance

Use only the admitted read copies in spec 0040. A projection is
the set of dimensions present in a source row. Key a daily row by
site, connection, property, source, native day, search type,
projection ID and the typed values of every projected dimension.
An atomic import generation binds rows to a coverage receipt for
that projection and scope. Never add projections, mix generations
or treat a row as evidence for an absent dimension. A complete
import of exposed query rows can still omit rare queries. A closed
connection day alone establishes no projection's completeness.
For complete page or section aggregates, require a source profile
with authoritative page aggregates and receipt coverage for the
full intended scope and window. Otherwise the aggregate is
unavailable. Show partial samples only where the view permits them.

- **Daily position history (223).** Show Search Console and Bing
  on separate labelled series, in each source's native reporting
  days. Use the position and its denominator as admitted by that
  source. Apply HC01–HC08. If either connector omits a compatible
  position field or weighting count, leave the affected value
  unavailable. Each needs an admitted metric catalogue entry.
  Use “Average recorded position” in the chart.
- **Section visibility (224).** Use Search Console only and the
  historical section membership from the site's own page index.
  Its section lookup returns section, page ID, effective interval
  and mapping generation for the entire native day. A same-day
  move has no unique attribution where daily totals cannot be
  split. Compare the 28 whole source days ending on the chosen
  closed day with the 28 before them. Both need complete coverage.
  Use ganit's existing period calculation. Show both impression
  totals and their signed current-minus-prior change. Show both
  weighted positions and their point change with HC06 polarity.
  Keep the position evidence gate. A measured zero can be shown
  and gives no position. The view serves a section only, with no
  blended score.
- **Position bands (225).** On one comparable source, dimension
  and day, count each admitted position row once in `[1,4)`,
  `[4,11)`, `[11,21)` or `[21,+∞)`. Use source decimals
  before rounding. Show the number of usable rows and disclose
  omissions under the source contract. The chart counts reported
  averages, not individual searches or visits. Values below 1,
  non-finite values and unsupported observations are excluded with
  reasons. “No evidence for these bands” differs from zero rows in
  a band of an available chart.
- **Query segment (226).** Apply a regular expression, a text
  matching pattern, inside the Rust service's own authorised
  source filter. The author may choose literal text or regular
  expression. Pattern matching is case-sensitive Unicode search on
  the full source query text, with anchors available for exact
  matching. No backreferences, look-around, executable code or
  cross-source joins. Cap the expression at 256 UTF-8 bytes and
  use a linear-time matcher with a 512 KiB compiled size cap.
  QR-125 must qualify those bounds and the existing universe cap.
  A bad or oversized expression fails as a whole. An unavailable
  source row or one exceeding the admitted scan bound cannot
  silently become a non-match. QR-125 admits total scan rows,
  source bytes, time and retention before a profile can run.
  A saved filter binds staff actor, site, connector, property,
  projection, matcher version and mode to its exact scope. Use
  protected saved filter storage and the file ledger. Load, edit
  and scheduled runs check that actor's current authority.
  On grant loss or source erasure withhold matches and invalidate
  pending and cached results under specs 0032 and 0040. Scripts,
  notices, logs and outside exports receive no patterns or terms.
- **Position alerts (232).** Use the existing position alert for
  an absolute point move beyond the reviewed threshold over two
  adjacent whole calendar weeks or months. Equality does not
  trigger. The existing evidence gate, history preview, grouping
  and repeat suppression apply. Smaller position means “better”.
  Use ganit's single period comparator. Require every day in both
  native periods to be closed and measured. A gap cannot trigger
  a swing. Replaying the same rule, series and window updates its
  notice without a second alert.
- **Weekly gains and losses (233).** Compare adjacent native
  weeks from Monday through Sunday, one source and one metric.
  Use the same ganit comparator. Require all days in both weeks
  to be closed and measured. Sort gains and losses by magnitude,
  then stable series ID. List rows with no move apart. Withhold
  insufficient rows with a reason. Full-period requirements for
  224, 232 and 233 leave the general measured-share rule in force
  for other charts and alerts.
- **Landing page trends (234).** Select one section and one
  admitted connector metric measured at the landing page or
  source page. The view names which of these each source measures.
  Use one same-source projection, with dates and page-to-section
  mapping proved for that observation. The existing chart rules
  govern days and comparison windows. Display separate labelled
  panes for different sources and never infer conversions from
  Search Console or Bing clicks or copy GA4 counts into their lines.

An unresolved historical join invalidates every section it might
affect. An authoritative interval proving that page outside the
selected section for the entire native day permits exclusion. Pin
one mapping generation for each calculation. Never use the current
section to backfill source dates. Query privacy keeps the
rare-term caveat and erasure requirements even after filtering.
Require live authority before resolving source rows and before
returning results. Invalidate pending and cached derived views
when membership, source, source grant or evidence changes.

#### Supplied search results

Use DR-085 owner-supplied files admitted through
spec 0092. Use that spec's exact revision, provenance, import
preview and owner confirmation. This surface reads those local
records by authorised API and cannot fetch search results, start
a provider connector, build a monitored series or send a rival
alert. An optional spec 0082 one-off page read is distinct
evidence and cannot establish a search position.

Spec 0092 owns the sample manifest and import generation. Scope
includes query, engine, market and location, device, result depth,
type taxonomy and measurement method. Each sample binds that scope
to an immutable measurement ID and observed time. The slot key is
sample ID, result family and position in that family's namespace.
Visual positions have their own namespace. Carry the result ID and
domain from the admitted typed normalisation. Keep repeated result
IDs in distinct slots. An identical same-slot replay collapses.
A conflicting slot invalidates its sample. An unmapped slot makes
any required domain or result figure unavailable. A declaration of
completeness names covered families and depths.

Two-time comparisons match all scope fields. Only sample ID and
actual observation time differ. A query-set summary may vary the
query across its explicitly selected set. Match the other scope
fields and require one declared capture batch. Different import
times cannot establish distinct observation times. A brand view
maps each canonical result ID to its set of family and slot pairs.
In both samples with a different set means “Changed”. Only in the
later sample means “Appeared”. Only in the earlier sample means
“No longer seen” at the stated depth.

| Feature | Local analysis and output |
|---|---|
| 227 Observed positions | Show each sampled position, result family and observation date. A complete sample without the domain at the stated depth says “Not seen in these results”. Withhold an absence claim for partial evidence. No interpolation between observations |
| 228 Visibility share in a supplied sample | Select an admitted query set and one complete sample per query in the same declared scope. Count each domain-attributed organic slot once. The denominator is all such slots, including other domains. Use the exact canonical domain mapping and show each selected domain's count and share plus “Other domains”. A missing query, refused mapping, conflicting slot assignment or zero denominator makes this share unavailable |
| 229 Search result types | Show the presence and count of admitted types in each complete sample. Include type taxonomy version, depth and query count. An unlabelled or not measured type stays unknown. Keep map, shopping and other types distinct from organic slots |
| 230 Map result positions | Render only eligible supplied cells for the exact selected listing and grid scope. Use the same rows for a local grid and accessible table. Give the observed map position, “Not seen” with complete mapped cell evidence, or “Unknown” for an unproved result. No outside tiles or interpolation |
| 231 Brand search snapshots | The owner chooses brand queries in admitted supplied files. Show which bounded results appear in each sample and a manually requested comparison of two matching complete snapshots. Changed, appeared and no longer seen apply only to the compared scope. Repeated automatic capture, visits or alerts are outside version 1 |

For feature 230, QR-125 qualifies the selected listing key under
0092's admitted source and result identity profile. Pin its exact
engine, listing identity and applicable alias proof to the supplied
grid and measurement scope. A display name or bare location label
cannot resolve a listing. Each requested cell must match the exact
coordinate key and location scale, query, device and observation
time. Keep the map result family and its measured depth distinct
from organic positions.

In a complete eligible cell, a unique proved position of that
listing gives the observed map position. “Not seen” requires
attribution of every admitted map slot, with none for the selected
listing. A different mapped branch can share the display name
without counting as the target. An ambiguous row proves no target
position or absence. Return “Unknown” if identities or positions
conflict or if the cell is ambiguous, missing or partial. Keep
other independently eligible cells within the same grid scope.
Use the same authorised rows and reason in the grid and table.
Recheck the exact source grant at read. Do not look up a listing
or acquire more observations to resolve an unknown.

The share uses measured slots as its denominator. It carries
no estimate of traffic or total search demand. No data flows into
link ranking, an action queue or an editor. Deletion or withdrawal
of a file, evidence item or source grant invalidates dependent
views, including pending work and restored state. Protected
temporary data, result caches and local receipts follow specs
0032 and 0118. If no profile is admitted, leave the corresponding
view unavailable. QR-125 lists the admission and proof obligations.

### Coverage requirements for 223–234

| Requirement | Verification contract |
|---|---|
| REQ-0043-OWN | IT-0043-OWN: daily and landing positions use the correct native day and one projection. Compare section-only 28-day views, exact weighted positions, signed increases and falls, same-day moves, unrelated unmapped pages, unlike dimension values, overlapping projections, mixed generations, omitted rare terms, measured zero, gaps and late data. Prevent whole-site section comparison and no mixed provider total |
| REQ-0043-SEGMENT | IT-0043-SEGMENT: test band boundaries and labelled omissions, literal and anchored Unicode matches, bad pattern, unsupported features, byte, compile, universe and combined scan caps. Test saved filters, wrong actor, erasure and revocation on a scheduled rerun. Sparse and partial scopes disclose coverage. No script or other site sees the query |
| REQ-0043-SAMPLES | E2E-0043-SAMPLES: offline approved imports test all five observed-result views with complete, partial, contradictory and withdrawn data. Try repeated results, identical and conflicting slot replay, unmapped results, unlike depth, a changed taxonomy and scoped absence. Replay after restore. Attempt provider and rival schedule creation, map tile calls and alert delivery. Every such route remains unavailable |
| REQ-0043-MOVES | IT-0043-MOVES: exercise paired closed calendars, ties, tiny counts, a move equal to the threshold, a late revision, a gap above the general measured-share floor and repeated notice delivery. Other alerts still apply the general floor. A data gap cannot become a gain, loss or position swing. Show the actual polarity |
| REQ-0043-FOOTPRINT | E2E-0043-VIEWS: synthetic cases on each admitted v1 platform, with socket denial, cross-site data, lost grants, cancelled work and current scope checks. Compare normal page work and ranking input before and after all views. Zero added page work and no ranking effect |

#### Wave 2 position and supplied search verification

These feature bindings retain the existing chart, disclosure and
source contracts. Each synthetic fixture selects one exact assertion
within the existing verification case. Keep observed results separate
from these planned fixtures under spec 0076.

| Owning design | Applicable variants |
| --- | --- |
| DES-0043-W2-NATIVE | VAR-0043-W2-POSITION-DAYS uses separate native position and day series. VAR-0043-W2-SECTION-POSITION requires the complete historical section and Search Console window receipt. VAR-0043-W2-SECTION-LANDING uses one admitted page or landing-page projection per source. |
| DES-0043-W2-FILTERS | VAR-0043-W2-POSITION-BANDS counts exact comparable source averages. VAR-0043-W2-QUERY-FILTER uses the existing authorised literal or regular-expression mode in the protected source projection. |
| DES-0043-W2-SAMPLES | VAR-0043-W2-SUPPLIED-POSITION, VAR-0043-W2-SUPPLIED-SLOTS and VAR-0043-W2-SUPPLIED-TYPES separately use the exact admitted position, organic slot or taxonomy evidence. VAR-0043-W2-SUPPLIED-GRID and VAR-0043-W2-SUPPLIED-BRAND use the corresponding supplied scope and complete cell or comparison contract. |
| DES-0043-W2-PERIODS | VAR-0043-W2-OWN-POSITION-ALERT uses the existing own-site reviewed position alert. VAR-0043-W2-NATIVE-WEEK-MOVES shows comparable closed native weeks through the same period comparator. |

| Feature | Existing requirement, design and variant | Existing test and exact fixture assertion |
| --- | --- | --- |
| F-0223 | REQ-0043-OWN, DES-0043-W2-NATIVE, VAR-0043-W2-POSITION-DAYS | IT-0043-OWN, FX-0043-W2-DAILY-POSITION. Give Search Console and Bing observations on unlike native date bases. Withhold a compatible weighting count in one series and add a still-arriving point in the other. Keep separate labelled lines, the unavailable observation and the late state under HC01–HC08. No copied day, pooled provider value or fabricated zero. |
| F-0224 | REQ-0043-OWN, DES-0043-W2-NATIVE, VAR-0043-W2-SECTION-POSITION | IT-0043-OWN, FX-0043-W2-SECTION-VISIBILITY. Use two adjacent closed 28-day Search Console windows and exact historical memberships. For comparable section evidence, show both impression totals, signed current-minus-prior change and weighted positions with lower-is-better polarity. A move within one native day or a missing mapping holds affected section results. No site-wide aggregate. |
| F-0225 | REQ-0043-SEGMENT, DES-0043-W2-FILTERS, VAR-0043-W2-POSITION-BANDS | IT-0043-SEGMENT, FX-0043-W2-POSITION-BANDS. Use exact admitted averages of 1, 3.9, 4, 11 and 21 and one unsupported row in a single source/day/measure. Show band counts 2, 1, 1 and 1, with five usable rows and one omission. Distinguish zero in an available band from unavailable evidence. |
| F-0226 | REQ-0043-SEGMENT, DES-0043-W2-FILTERS, VAR-0043-W2-QUERY-FILTER | IT-0043-SEGMENT, FX-0043-W2-QUERY-FILTER. Compare literal and anchored case-sensitive Unicode matching in the exact saved actor/site/source projection. Reject unsupported syntax or a byte, compile, universe or scan overrun as a whole. In interactive and scheduled cases revoke the grant or erase the source. No partial, restored, cross-site, script or notification match follows. |
| F-0227 | REQ-0043-SAMPLES, DES-0043-W2-SAMPLES, VAR-0043-W2-SUPPLIED-POSITION | E2E-0043-SAMPLES, FX-0043-W2-OBSERVED-POSITION. Give compatible observed positions across samples, a complete sample without the domain at its declared depth and a truncated sample. Show the exact sampled dates and bounded absence only for complete evidence. No interpolation or substitution from a native-series average, including after withdrawal or restore. |
| F-0228 | REQ-0043-SAMPLES, DES-0043-W2-SAMPLES, VAR-0043-W2-SUPPLIED-SLOTS | E2E-0043-SAMPLES, FX-0043-W2-SLOT-SHARE. Use one eligible complete query set with three mapped organic slots: two attributed to one named domain and one to Other domains. An exact same-slot replay leaves three slots and shares of two thirds and one third. A conflicting slot, missing query, unmapped domain or zero denominator gives unavailable. |
| F-0229 | REQ-0043-SAMPLES, DES-0043-W2-SAMPLES, VAR-0043-W2-SUPPLIED-TYPES | E2E-0043-SAMPLES, FX-0043-W2-RESULT-TYPES. Show admitted organic, map and shopping types in one complete sample using their own family and taxonomy version. An unlabelled result stays unknown. A changed taxonomy refuses cross-sample comparison even if every result position is equal. |
| F-0230 | REQ-0043-SAMPLES, DES-0043-W2-SAMPLES, VAR-0043-W2-SUPPLIED-GRID | E2E-0043-SAMPLES, FX-0043-W2-MAP-GRID. Use one exact supplied grid with the selected listing at a recorded map position. A complete cell contains only a mapped same-name different branch and gives Not seen. An otherwise identical cell with unresolved listing identity gives Unknown, as does a missing cell. Try different location scale, query, device or observation time and withdrawn source. The table and grid agree on authorised values and reasons. No provider lookup, outside tiles or interpolation. |
| F-0231 | REQ-0043-SAMPLES, DES-0043-W2-SAMPLES, VAR-0043-W2-SUPPLIED-BRAND | E2E-0043-SAMPLES, FX-0043-W2-BRAND-SNAPSHOTS. In two matching complete samples taken at distinct observed times, move one canonical result to a different slot, add one and omit one. A manual comparison shows Changed, Appeared and No longer seen for the exact measured depth. Unlike depths or different import times alone cannot yield a comparison. Restart starts no capture or rival alert. |
| F-0232 | REQ-0043-MOVES, DES-0043-W2-PERIODS, VAR-0043-W2-OWN-POSITION-ALERT | IT-0043-MOVES, FX-0043-W2-POSITION-SWING. Compare two closed and fully measured adjacent native weeks or months for the same authorised series. An absolute point move strictly beyond the reviewed limit yields one grouped local notice with correct position polarity. Equality or a missing day gives none. Replay cannot emit a second logical notice or disclose query text. |
| F-0233 | REQ-0043-MOVES, DES-0043-W2-PERIODS, VAR-0043-W2-NATIVE-WEEK-MOVES | IT-0043-MOVES, FX-0043-W2-WEEKLY-MOVES. Use two closed Monday–Sunday weeks, one native source and one metric. Give two equal gains, a loss, an unchanged series and a missing day. Sort by magnitude within direction then stable series identity. Keep unchanged apart and withhold the incomplete series with reason. |
| F-0234 | REQ-0043-OWN, DES-0043-W2-NATIVE, VAR-0043-W2-SECTION-LANDING | IT-0043-OWN, FX-0043-W2-LANDING-TRENDS. Choose one exact section. Supply an admitted source-page metric and a separately admitted landing-page metric with their native dates. Keep the panes labelled and require historical section mapping for each observation. Unresolved mapping withholds the affected section result. Separate eligible GA4 evidence cannot be injected into search lines or prove conversion from a search click. |

DR-085 keeps 227 to 231 on the supplied-results route. An eligible
0040 read copy, a separate one-off public page read or local rule
setting cannot supply a native sampled rank, feed or rival tracker.
The existing 232 own-site alert adds no rival alert route.

QR-125 still owns exact source coverage, historical mapping,
admitted profile, thresholds and bounds, lifecycle and measured
proof. Bind each fixture to its exact source shape, permission,
executable assertion and evidence producer before task derivation.
All views retain the existing site, erasure and zero public-page
footprint rules.

### Honest chart rules

- HC01: each day is closed (final), still arriving (a late source's
  newest days) or not measured. Each looks different and is named in
  the table.
- HC02: not measured is a gap, never zero and never joined across.
- HC03: alerts, filters, comparisons and rules use closed days only.
- HC04: job 214's estimate is a shaded band labelled "estimate", never
  in a line of copied numbers. With no result: "not yet available".
- HC05: daily stays daily. Weekly and monthly bars use closed days only
  and skip a part period. No intraday detail.
- HC06: for position, lower is better and 1 is at the top. Words say
  "better" or "worse", never "up" or "down". Polarity travels with the
  series through every comparison, alert, filter and rule.
- HC07: a day with too few impressions shows "too few impressions" and
  no position line. The floor is `quarry.chart.min_impressions`.
- HC08: two sources are two labelled lines. A sum or ratio needs one
  source and one day grid. Unlike settings or dates: "do not compare".
- HC09: fewer than 10 visits, clicks or sign-ups from darshak shows
  "fewer than 10".
- HC10: a forecast range shows only when a ganit result exists and
  history is long enough. Otherwise nothing is drawn.
- HC11: zooming never changes a peak. Drawing thinner may only keep
  highs and lows; the table keeps every point.
- HC12: an event band says what happened and when, never why. Its note
  reads "association, not cause".

### Event bands

The update study in spec 0072 uses these bands and the separate metric
series here; the spam-view estimate there uses the separate metric
series only.

Three kinds: confirmed incidents from [0137][updates];
Sutradhar's own changes from the audit log (an approved link
batch, an undo); notes an admin types. Events close together merge into
one band with a count; none is dropped. A band opens its record. Nothing
is read from other people's sites for this.

When 0137 cannot confirm an incident again, show its permitted
saved band and last confirmation time in the chart and keyboard
table under the existing hold. A confirmed correction updates the
current band. The metric pane keeps its own HC01 day states and
source dates. An official date confirmation gives no proof of
measurement coverage. FX-0040-W1-UPDATE checks this join under QV33
with the existing 0137 rules.

#### F-0308 dated site notes

REQ-0043-W2-DATE-NOTE and DES-0043-W2-NATIVE-NOTE use the
existing admin note and event-band contract. A note binds its
authorised site, actor, source date grid, native day and zone.
Keep its own event identity when a band groups it with an
admitted incident or audited undo.

FX-0043-W2-DATE-NOTE extends QV33 with HC12. Place a valid
native-day note on a shortened daylight-saving day beside the
two distinct event kinds. Replay the saved note and try an
unmapped clock and a withdrawn note. Chart and keyboard table
show the same current permitted records, with supported dates
and the original category. An unavailable note neither creates
a band member nor supplies evidence of cause.

### Comparisons

A comparison helps one decision: is this move normal for this series,
or worth a look? There are only these:

- A period against the period before it (see "Periods and the period
  before" below).
- Two numbers of one series and one source side by side, such as
  impressions against position, or clicks against position.
- ganit's forecast range for organic clicks (HC10; jobs 204 and 212)
  and its changepoints in organic clicks (job 161).
- Request-rate anomalies flagged by ganit job 176 (Vichalan), which
  watches darshak's request rate for the whole site, bots included.
  They are shown only as markers labelled "request-rate anomaly" on the
  time axis of whole-site charts, never as a count or a series.
- One value for each whole fixed period: the chosen dates, a calendar
  week (Monday to Sunday) or a calendar month, built from each source's
  own daily dates (Search Console's, GA4's property time zone,
  darshak's), never re-split.

The ganit results cover only these series: the forecast range and
changepoints cover organic clicks (Search Console clicks for the whole
site or a section), and request-rate anomaly markers appear on
whole-site charts only. Other series never offer them. On a covered series
with no ganit result for a day, nothing is drawn and the chart says "no
ganit result yet".

Periods and the period before:

- A period's value is a sum for summed series (clicks, impressions,
  sign-ups from search) and a weighted mean for the rest:
  position weighted by impressions, click rate as the period's clicks
  over its impressions, and the human sign-up rate as its sign-ups over
  its visits.
- A summed series is compared by its average per measured day (the
  period's sum divided by its number of measured days), so missing days
  and months of different length do not skew the comparison.
- Summed series compare as a change in number and in share. Position
  and the two rates compare as a difference in the period values only,
  never as a share; position follows HC06, so lower is better.
- If the earlier period's sum is below `quarry.chart.min_compare_count`,
  or for a rate or position the sum either period is weighted by
  (impressions, or visits for the human sign-up rate) is below it,
  whatever the source, the pair gives no result and says "not enough
  evidence". This is the one evidence gate for comparisons, position
  included. A summed series' later period has no floor, so a fall to
  nothing still shows and can alert, except where HC09 withholds it.
- HC09 covers comparisons and alerts too: when any darshak count behind
  a period's value (a sum, or a rate's sign-ups or visits) is fewer
  than 10, that period shows "fewer than 10" and no value, and the pair
  shows no change, difference or share and has no result.
- If either period has too many days not measured (see "Rules for all
  of them"), the pair gives no result.
- If the series' history is shorter than the two periods together, the
  pair gives no result and says "not enough history".
- A pair with no result, or a single period with no value, is shown as
  such and never fires an alert.

Rules for all of them:

- One definition of each sum and comparison lives in ganit. The chart,
  alerts, the filtering tool and chart rules all call it. A second copy
  anywhere is a defect.
- Closed days only. A period with too many days not measured (share set
  by `quarry.chart.min_measured_share`) gives no value and says "not
  enough measured days". Missing days are never zero or filled in.
- Position follows HC06 and HC07; comparing it uses the one evidence
  gate above.

Set aside: link quality scores (link counts chart as the one labelled
source); a rival index; an index of
other sites' turbulence (needs sharing between installs); effort sizing;
steady-growth ratios.

### Alerts

An alert watches one series, a saved filter's result, or the whole
site's request-rate anomalies from ganit job 176, and tells staff
when a condition becomes true. It is a notice and nothing else.

- Conditions from a menu: for a whole calendar week or month against
  the one before, better or worse by more than a set share (summed
  series only) or a set difference (position and the rates); for a
  whole calendar week or month, its value (the average per measured
  day for summed series) better or worse than a fixed level, where
  position and the rates need that period's impressions or visits to
  reach `quarry.chart.min_compare_count`; outside ganit's
  forecast range or a changepoint marked by ganit (organic clicks
  only); an anomaly flagged by ganit job 176 (request rate, whole site
  only); newly matching a saved filter whose test is from this menu
  and is not itself a saved filter; data went missing. A chart rule can
  also return yes or no.
- A period condition is checked once, when every day of the period is
  closed or marked not measured. A ganit condition with no ganit
  result stays silent, and the alert shows "no ganit result yet".
- Checked after each pull, on closed days only; a ganit condition is
  checked when ganit's result for that day lands. A move into or out of
  "not measured" never fires, except by "data went missing".
- "Test on history" shows the days the rule would have fired. It sends
  nothing. A new rule starts only after that has been seen.
- The notice carries rule name, series, days, source, size of the move
  and "association, not cause", and links to the chart with the rule
  drawn on it.
- It joins the grouped alert list. Notices from one rule in one window
  merge ("12 series matched") with every row kept. Staff mark one seen
  or closed. A repeat updates the same notice;
  `quarry.alert.repeat_gap_hours` limits repeats.
- Quiet hours hold notices for one digest. Faults ("this rule could not
  be checked") still get through at once.
- Outside the app, a notification holds a count and a link, never a
  search word. Existing channels are used.
- A failing rule raises one fault notice. It never fails silently and
  never fires a data alert.
- No alert edits, hides, links or redirects anything.

### Filtering tool

It finds every series that passes a test, such as "pages in the guides
section whose clicks fell more than usual over the last 28 complete
days, with enough impressions".

1. Step one chooses series with the action layer's one filter form:
   site, section, source, page or query text, minimum impressions. It
   runs in the database, in pages, in one pass, on that site's rows.
2. Step two is the test: the alert menu, or a chart rule returning yes
   or no per series, run only on series that passed step one.

- A rule test runs on at most `quarry.filter.universe_cap` series. Over
  that, the screen says "too many to test: narrow the filter" with the
  count. It never tests only the first few.
- Results are a ranked, sortable table: series, evidence, days used and
  a small trend line. Rows with too little evidence are listed apart as
  "not enough evidence" and never ranked. A row opens the chart.
- A saved filter reruns on a schedule, per admin and site, showing
  "newly matching" and "no longer matching" with dates. It is a
  low-priority job, shed first under pressure, and can raise an alert.
- "Falling, as in the report" calls the site structure report's own
  function, so a list and the report agree. Filter rows are not
  findings.
- Result lists hold search words, so they are staff-only and never in a
  notification or export without the export preview.

### Chart rules

A chart rule is a short declarative rule in TOML that an admin writes in
a plain text box, run by a Rust rule engine. It does one of three jobs:

- Line: draws one extra line or a few marks on a chart.
- Test: yes or no for each series, for the filtering tool.
- Alert condition: yes or no for a series on each closed day.

Nothing else. A rule cannot suggest or make a change. Its output is
labelled "your rule", in its own style, with the sources it read. It
never feeds a ganit result, a ranking, a link suggestion, a Coach tip or
anything else that decides.

Shape only (names settle in the build):

    api = 1
    kind = "alert"
    series = ["clicks"]

    [[let]]
    name = "usual"
    call = "spread"
    of = "clicks"
    days = 90

    [when]
    worse = { of = "clicks", days = 14 }
    more_than = { times = 2.0, of = "usual" }

What a rule sees and returns:

- Only the series named in its header; naming is the permission. The
  engine sends those series for a fixed run of days and nothing more.
- Ids and numbers only, never search words, addresses, member text,
  other sites, settings or secrets.
- Closed days only; asking for days still arriving is refused.
- Each value is closed or not measured. Not measured stays not measured
  in every sum, ratio and average; a rule cannot turn it into zero.
- Position keeps its polarity; the rule language speaks of "better" and
  "worse".
- Built-in comparisons are calls answered from ganit, so there is still
  one definition of each.
- It returns a fixed shape: a list of (day, number), or marks (day, a
  label from a fixed list or short plain text), or yes or no. A checker
  validates it: days inside the range, finite numbers, size caps, no
  markup. Output that fails is refused whole, with the reason shown.
- Same rule, same input, same output. No clock, no random number, no
  unordered walk of data. The day being examined is an input.
- A header naming two sources is refused unless the rule only shows
  them side by side. HC01 to HC12 apply unchanged.

### How a rule runs safely

No layer is trusted alone.

1. Before running, a check refuses a rule that is too long, names an
   unknown interface version, or uses an unknown key. The box shows the
   line.
2. The engine knows only the small rule vocabulary: no import, no
   evaluation of text as code, no clock. Limits apply to steps, nesting
   depth and data size.
3. Rules run in a locked sandbox made under the extension isolation
   rules (Linux namespaces, seccomp and Landlock, no container program):
   non-root, read-only root, all capabilities dropped, no new
   privileges, the reviewed seccomp list, memory limit equal to swap
   limit, process and processor limits, no host folder, no extra
   socket, no own store and no network at all.
4. One pipe in and one out: data and comparison answers in, the checked
   result out. No environment secret, no other door.
5. Steps are the first limit and are repeatable on any machine. A
   wall-clock time is the backstop. A run that hits it says "stopped:
   time", is never kept as a result and never fires an alert.
6. A rule that fails or is stopped repeatedly is paused and the admin is
   told. Runs per admin per hour are limited.
7. The sandbox starts for each run and exits; no warm pool. If start-up
   is too slow, a preview runs when the admin presses Run or pauses
   typing, never on every keystroke.

### Keeping rules

- Stored in PostgreSQL per site, with row-level security. Saves are new
  versions only, each with a note. Delete goes to trash for 90 days.
  Undo history follows the platform rule (180 days).
- Each rule names an interface version. Released versions keep working;
  a change to the interface is a new version.
- Each run writes a `job_run` record: rule version, interface version,
  engine version, input digest, limits, output digest and outcome.
- Saving a rule needs the permission "Write chart rules", off for
  moderators. Anyone who may view the chart may see a rule's output.
- A rule is plain text. An admin may export it to a file and import one
  by hand; imports are rechecked and never trusted. Sutradhar sends
  rules nowhere. No library, no ratings, no hidden source.
- One generic catalogue entry covers "admin rule output".

### Left out on purpose

A rule compiler, language server or rule library; a second chart
library; sums across series; rival series, competitor snapshots and
reverse content gaps (rivals are never crawled for tracking); a
fleet-wide turbulence index; a community library, marketplace or hidden
rules; recommendations sent to a change queue; intraday detail; drawing
tools, linked multi-chart workspaces and a window-side data cache;
signed webhooks and push digests; a matcher for thousands of standing
alerts; crawl rate and publishing velocity as charted numbers. Actual
search placements are read only from the sections on supplied samples.

### Settings

Every setting has a safe floor and top, and a setting may loosen a limit
only up to its admitted safe top; an unadmitted top blocks the change. There is
no off switch for the
honest rules, the rule limits as a group, or the "no network" rule. Each
comparison also has its own window setting.

| Setting | Default | Allowed range |
| --- | --- | --- |
| `quarry.script.max_steps` | blocked: no admitted value | above zero, safe top |
| `quarry.script.max_seconds` | blocked: no admitted value | safe floor and top |
| `quarry.script.memory_limit` | blocked: no admitted value | fixed share of the slice |
| `quarry.script.max_source_bytes` | blocked: no admitted value | above zero, safe top |
| `quarry.script.max_output_points` | blocked: no admitted value | above zero, safe top |
| `quarry.script.comparison_calls` | blocked: no admitted value | above zero, safe top |
| `quarry.script.runs_per_admin_hour` | blocked: no admitted value | above zero, safe top |
| `quarry.script.pause_after_failures` | blocked: no admitted value | safe floor and top |
| `quarry.filter.universe_cap` | blocked: no admitted value | above zero, safe top |
| `quarry.filter.page_rows` | blocked: no admitted value | safe floor and top |
| `quarry.filter.memory_share` | blocked: no admitted value | fixed share of the slice |
| `quarry.alert.rules_per_site` | blocked: no admitted value | above zero, safe top |
| `quarry.alert.repeat_gap_hours` | blocked: no admitted value | safe floor and top |
| `quarry.chart.min_impressions` | blocked: no admitted value | at least darshak's floor |
| `quarry.chart.min_measured_share` | blocked: no admitted value | safe floor and top |
| `quarry.chart.max_series` | blocked: no admitted value | above zero, safe top |
| `quarry.chart.max_points` | blocked: no admitted value | above zero, safe top |
| `quarry.chart.compare_days` | blocked: no admitted value | safe floor and top |
| `quarry.chart.min_compare_count` | blocked: no admitted value | at least darshak's floor |
| `quarry.run.keep_days` | blocked: no admitted value | follows the undo history rule |

Every value marked "blocked" is a readiness blocker owned by the Alekh
Ganit Vikas research record below. The record must supply the value, safe
bounds, fixtures, supporting evidence and approval before its setting or any
dependent script, filter, alert, chart or retention behaviour can build or
run. There is no implicit default or unbounded fallback.

### Build-blocking research

Ganit Vikas, on real history and made-up data with planted moves: which
comparisons earn a place (false alarms against misses, against a plain
average), which families are set aside, windows per comparison; how many
measured days a comparison needs and what share of gaps spoils it; what
"usual" should mean, with a robust spread, and repeat gaps that keep
false alarms low; how late Search Console days arrive, whether copied
days are revised, and so when a day is closed; typical steps and memory
of useful rules, sandbox start cost on Dell, whether step counts are
identical across machines and versions, fuzzing of the pre-check and
output checker, and whether ganit calls are fast enough; filter sizes by
site size and what the first scan can read in its share; a thinning
method that keeps highs and lows; points and series per chart.

### Changes to other parts

- Visual layer: a Search charts screen and its data rows; no new
  library.
- Metric catalogue: entries for each series and comparison, plus one for
  admin rule output; connected-service and backlink figures as one
  labelled source.
- Safe running: `karya.shed_order` gains `chart_filter` and
  `chart_script`, after `test_room`. `live` is never listed.
- Extension isolation: the sandbox allow-list accepts a first-party rule
  sandbox with no network.
- Settings register: the `quarry.*` keys.
- Grouped alerts: a "chart alert" notice kind and a "rule could not be
  checked" fault.
- Memory record: rows for the parts below, once measured.
- Site structure checks: the falling test is callable by name, with no
  change in behaviour.
- The manual gains a Search charts page for site owners.

### Logic and maths

Decisions are Rust, proved with Kani where they decide: day-state and
polarity rules, honest-chart checks, whether an alert fires, merges or
waits, the rule pre-check and output checker, run admission against
limits, and "too many to test". All maths is in ganit, one definition
per comparison. A faster version may be added only with ganit's version
as its safe twin and a test that they agree. No PHP change, as guests
see nothing.

## Limits

Everything counts inside AGENTS.md 5.6: the service parts in the service slice,
the chart window in the window's cap, no swap.

| Part | What it adds | Guard |
| --- | --- | --- |
| Rule sandbox | Peak while a run lasts, none when idle | Sandbox memory limit |
| Filter run | A scan of many daily rows | Paged, one at a time, a share |
| Comparison calls | More work for ganit | Calls per run capped |
| Chart data | Small snapshots from the service | The visual layer's 1 MB and 2 MB caps |
| Alert checks | A small pass after each pull | Per-site rule cap |
| Records | Disk: runs, versions | Kept for set days |

Risks, biggest first:

1. The filter scan. It reads in pages inside the database, keeps no full
   copy, runs one at a time at low priority, asks the promise check
   before any large request, and is shed first. A big filter is slow,
   not dangerous.
2. The rule sandbox. Its memory limit stops one run ("stopped: used too
   much memory"), not the service. Worst case to test: the limit times
   the runs allowed together.
3. Everything at the same hour. Pulls, saved filters and alert checks
   run in karya's order with spread start times; alert checks come after
   pulls.
4. ganit load. Calls per run are capped and answered from ganit's cache
   for the same series and window where it holds one.
5. The window. A cap on points and series per chart keeps drawing light.

Not added: no headless browser, second chart server, language server,
code-editor package, warm worker pool or new number store.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Rust tests, with SQL tests for grants and row-level security:

- QV01: a day still arriving is drawn apart and never used by an alert,
  filter, comparison or rule.
- QV02: a day not measured is a gap in chart and table, and never zero in
  any comparison or rule sum.
- QV03: no estimate band appears without a ganit result; when shown it
  is labelled and never inside a line of copied numbers.
- QV04: two sources are never summed; a ratio across sources is refused.
- QV05: on a position series every comparison, alert, filter and rule
  says "better" or "worse" correctly; a sign test covers each comparison.
- QV06: too few impressions gives that message and no position line.
- QV07: darshak counts under 10 show "fewer than 10".
- QV08: no weekly or monthly bar is drawn for a part period.
- QV09: a series or comparison with no catalogue entry is not drawn.
- QV10: chart, alert, filter and rule give one value for one comparison.
- QV11: no alert fires on a move into or out of not measured, except
  "data went missing".
- QV12: "test on history" sends nothing.
- QV13: notices merge with rows kept; a repeat updates one notice; the
  repeat gap holds; quiet hours make one digest; a fault still gets
  through.
- QV14: a failing rule gives one fault notice and no data alert.
- QV15 (SQL): the alert code and the rule engine hold no write grant on
  any page, post, link, setting or ranking table.
- QV16: a rule test over too many series is refused with the count; no
  subset is tested silently.
- QV17: rows with too little evidence are listed apart and never ranked.
- QV18: a saved filter shows newly and no longer matching, with dates,
  and is shed first under pressure.
- QV19: in the sandbox, a network call, file read or write, clock read
  and random number each fail.
- QV20: hostile rules (endless loop, deep nesting, memory balloon, huge
  output, nested lists, many comparison calls) stop inside their limits;
  the screen stays live, other admins are unaffected, memory stays in
  its share.
- QV21: the same rule and input give the same output and stop at the
  same step. A run stopped by the time backstop is never kept and never
  alerts.
- QV22: a rule sees only its declared series, never search words,
  addresses or member text; an undeclared series is absent.
- QV23: output of the wrong shape is refused whole, with the reason.
- QV24: no rule output reaches ranking, linking, the Coach or a ganit
  input.
- QV25: saves are new versions; delete goes to trash for 90 days; every
  run has a complete record.
- QV26: with two sites, one site's rules, views, alerts, filters and
  runs never appear for the other.
- QV27: guests and members see nothing; staff without the permission
  cannot save a rule.
- QV28: the table loads before the picture; keyboard use, high contrast
  and reduced motion work; every screen state shows in words.
- QV29: a saved view reopens "as of" its date, with a switch to now.
- QV30: an admitted link or crawl source is labelled. A missing source
  shows "not measured". A saved rival cannot create a series or alert.
- QV31: with chart, rule and filter at their shares and all else
  running, memory stays inside the service slice and the window's cap.
- QV32: if the rule engine is switched off or fails its checks, screens
  say "rules unavailable" and the menus still work.
- QV33: event bands merge and never drop; none uses a cause word.
- QV34: an imported rule is rechecked first; Sutradhar sends none
  anywhere.

The pre-check and the output checker are fuzzed.

## Changes from the source

- The user script language is replaced by declarative TOML rules run by
  a Rust rule engine, with the same jobs, limits and safety layers; its
  crate admission record is no longer needed.
- Decision logic and adapters are rewritten as Rust, proved with Kani;
  maths uses ganit.
- Admin pages become screens in the desktop app built from SDS; charts
  come from the visual layer.
- Memory follows the limits in AGENTS.md section 5.6, not a flat 8 GB
  with 15% headroom.
- The source's speed claims are not carried; speed follows the 5%
  median rule.

Source: `docs/specifications/signals/quarryview.md`,
`docs/specifications/signals/quarryview-2.md`,
`docs/specifications/signals/quarryview-3.md`,
`docs/specifications/signals/quarryview-4.md` and
`docs/specifications/signals/quarryview-5.md` in the archive
`archive-2026-11-08-old-repo`.

[updates]: 0137-google-search-update-tracker.md
