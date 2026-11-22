# 0092: Keyword, topic and competitive research

The research workspace shows what people search for, groups similar
searches and proposes pages or topics for the active site. It uses the
Search Console and Bing data copied by [spec 0040](0040-search-connectors.md)
and guest-visible pages indexed under [spec 0022](0022-reading-indexing.md).

DR-085 permits files supplied by the owner and one-off public rival
checks under [spec 0082](0082-leviathan-crawler.md). Live provider
feeds and rival monitoring are outside version 1. Spec 0041 owns
writing advice, spec 0046 owns backlink overlap (264), and specs 0093
and 0094 own AI visibility and general reporting. Spec 0043 owns
own-site charts. The combined rival comparison is local to this
workspace.

DR-117 permits the separately approved
[keyword lookup](0132-dataforseo-keyword-research.md). Its exact API
evidence stays labelled. This spec keeps its local file intake.

Fictional example. A cycle forum sees searches about winter tyre
pressure. The owner chooses an existing guide for one group. Another
group has no suitable page, so the board proposes a new topic. A
complete supplied ranking sample shows a rival and no result for
the forum within its measured depth. The gap card names the source,
date and result depth.

## Words used here

- **Query** means what someone searched for.
- **Cluster** means queries about the same subject and need.
- **Intent** means what someone wants to do, such as learn or buy.
- **Impression** means a recorded appearance in search results.
- **Snapshot** means evidence dated to when the source measured it.
- **Gap** means a difference within a named set of comparable evidence.

## What this spec designs

| Feature | Version 1 treatment | Requirement |
|---|---|---|
| 206 Keyword workspace | Search Console and Bing observations side by side | REQ-0092-WORKSPACE |
| 207 Keyword clustering | Local groups by meaning | REQ-0092-GROUPING |
| 208 Search volumes | Supplied estimates with dates and scope | REQ-0092-IMPORT |
| 209 Keyword difficulty | Supplied estimates on the source's named scale | REQ-0092-IMPORT |
| 210 Competitor keyword gap | Compare equivalent ranking observations | REQ-0092-COMPARE |
| 211 Question finder | Find question-shaped searches | REQ-0092-QUESTIONS |
| 212 Keyword-to-page mapping | Choose one eligible target per cluster | REQ-0092-TARGET |
| 213 Wikipedia interest trends | Supplied dated article page views | REQ-0092-IMPORT |
| 214 Forum long-tail ideas | Confirm precise phrases with recorded searches | REQ-0092-QUESTIONS |
| 215 Owner keyword lists | Lists and tags for the active site | REQ-0092-LISTS |
| 216 Business value tagging | Owner priority tags | REQ-0092-LISTS |
| 217 Opportunity score | Score evidence-backed planning ideas | REQ-0092-SCORE |
| 218 Zero-click queries | Show few or no recorded clicks to this site | REQ-0092-LOW-CLICK |
| 219 Irrelevant query filter | Keep the owner's exclusions off the board | REQ-0092-LISTS |
| 220 Intent shift detector | Compare result types in matched snapshots | REQ-0092-RESULT-SHIFT |
| 221 Semantic keyword map | Show the same groups as a visual map and a list | REQ-0092-GROUPING |
| 222 Content idea board | Show ideas with reasons and evidence | REQ-0092-SCORE |
| 259 Competitor registry | A list of named rivals per site | REQ-0092-RIVALS |
| 260 Competitor sitemap monitoring | After version 1 | REQ-0092-SCOPE |
| 261 Competitor page change alerts | After version 1 | REQ-0092-SCOPE |
| 262 Competitor speed comparison | Compare supplied field measurements | REQ-0092-COMPARE and REQ-0092-RIVAL-READOUTS |
| 263 Competitor ranking overlap | Compare supplied rankings in one sample | REQ-0092-COMPARE and REQ-0092-RIVAL-READOUTS |
| 264 Competitor backlink overlap | Local result from spec 0046 | REQ-0046-RIVAL-OVERLAP (test IT-0046-RIVAL-OVERLAP) |
| 265 Competitor social activity | After version 1 | REQ-0092-SCOPE |
| 266 Competitor structured data | Supplied structured-data facts | REQ-0092-STRUCTURED-DATA and REQ-0092-RIVAL-READOUTS |
| 267 Competitor publishing pace | Compare supplied publication evidence | REQ-0092-COMPARE and REQ-0092-RIVAL-READOUTS |
| 268 Competitor new topic alerts | After version 1 | REQ-0092-SCOPE |
| 269 Combined competitive gap report | Show supported findings and missing evidence | REQ-0092-REPORT and REQ-0092-RIVAL-REPORT |

## Workspace and ideas

1. Read the active site's admitted bound search copies under spec
   0040. Keep provider, exact property and active binding epoch,
   report profile, surface, dimensions, filters, native date and
   compatible clock, source revision and coverage with each row.
   Match space and case variants for display. Keep original rows
   and site and page totals distinct. Exact replay keeps the
   existing rows, coverage and observation date unchanged. A data
   correction needs 0040's verified current receipt for the exact
   bound report partition. Use its committed rows and coverage
   together. Keep the existing withdrawal and erasure fences.
   A superseded binding, wrong property or conflicting receipt
   cannot replace current rows. Reselecting the same property
   leaves old rows historical if the owner kept them and the
   archive grant still allows them. Show their original scope
   and dates. An archived row gives no current coverage or count.
   Mark coverage as complete for exposed rows, partial, missing or
   expired. Even a complete pull covers only the rows the source
   exposes. Recheck scope before showing a saved group or result.
   This view opens no provider session and starts no fetch.
2. Group with the admitted local text stack of specs 0024 and 0026.
   Pin the model revision, evidence set and stable input order. Show
   why each term was grouped. Leave unclear terms ungrouped. The
   group rule needs repeatability and topic-separation fixtures
   before use (QR-063). The map and keyboard-accessible list allow
   the same selection and inspection.
3. Find question-shaped queries in the workspace and show their
   counts. Guest-visible discussions on the active site may suggest
   a specific search phrase. A phrase has search confirmation only
   when a matching normalised query has recorded impressions.
4. Use the active site's guest-visible index under spec 0022.
   A local subject and intent test for research alone needs admission
   under QR-063. Propose at most one qualifying target and let the
   owner confirm an eligible page. Bind the exact site, group and
   membership revision, current mapping revision, eligible native
   page and page revision, admitted test and its evidence scope.
   Recheck group, page and authority at confirmation and when reading
   the mapping. If visibility or access is lost, the page disappears,
   or the admitted group or page revision changes, show “Target
   needs review” where current access permits. Do not carry the old
   confirmation to a different group or page version. A competing
   confirmation against an older mapping revision cannot overwrite
   the already committed target. Use the existing local owner route
   to review and confirm again.

   Show “No suitable page” only after a complete check of the
   eligible same-site candidate population under that exact
   subject and intent profile. An incomplete index, bounded
   shortlist or stale decision yields “Not enough evidence”.
   An unresolved mapping gives no evidence of absence and no
   opportunity point for lack of a page. No outside check, site
   change or ranking input follows.
5. The owner creates lists, tags, priorities and exclusions for the
   active site. A term entered by the owner keeps that source
   label. Excluded terms stay in the workspace and off the idea board.
6. The proposed opportunity score is 0 to 3. Give one point each
   for at least one recorded query meeting the impression floor,
   an explicit owner priority tag, and a confirmed lack of a suitable
   page. Record the engine, search type and window. Score only
   groups with an eligible, non-excluded member. Excluded members
   supply no evidence or priority. Suppress all-excluded groups.
   Show reasons and break ties by stable cluster identity. Missing
   demand or an incomplete page check gives "not enough evidence".
   The board offers ideas and outline prompts.
7. A low-click finding uses unique, non-overlapping rows for one
   query, engine, search type, period and level of detail. Require
   the proposed impression floor and positive impressions. Flag
   when 10,000 times the clicks is at most the ceiling in basis
   points times the impressions. Show the counts and period. People
   may have clicked another site. Incomplete or incompatible counts
   give "not enough evidence".

## Supplied evidence and intent

1. Use the bounded import in spec 0100. Show the source, permission
   to use it, measurement date, coverage, units, missing-value
   meaning and file hash. Preview before accepting an upload.
   Reject an incomplete or contradictory manifest. Treat values as
   data. Imports have no outbound fetch or execution. Use only
   schemas and rights admitted per source under QR-063. Refuse
   unadmitted sources and keep missing fields unknown.
   Supplied files use the byte and row limits of spec 0100
   (`import.max_mib`, `import.max_rows`) and an entry count and path
   length cap per archive (proposed 1,000 entries, 255 characters
   until 0100 names them). A ZIP entry path must stay inside the
   archive; an absolute path or a `..` part refuses the file. Escape
   every value on output. A permission claim in a file is shown as
   "owner says" and is never proof.
2. Show search volume as an estimate with the supplied scope.
   Explain difficulty on the source's own scale. Show Wikipedia
   article views as their own series. Keep unlike scales separate
   and show gaps in data.
3. Compare observed result types for the same query in two eligible
   supplied snapshots from distinct, ordered times. Match engine,
   device, scope, method, result depth and exact eligible position
   keys under the admitted source. Keep each result family's
   position namespace.
   Reuse 0043 F-0229's admitted type and family profile. Hold a
   changed taxonomy or result scope even if the labels and positions
   look the same. Each counted position must have exactly one
   admitted type and each sample must satisfy the admitted minimum
   under QR-063. An empty sample or missing, duplicated or unlabelled positions
   hold the comparison.
   Recheck both source grants before showing the comparison.

   Use the positive common complete position count as the denominator.
   For each type subtract its earlier count from its later count.
   Compare the absolute count difference times 10,000 against
   the configured basis-point threshold times the denominator,
   using checked integers before any display rounding. At threshold
   equality the result qualifies. Show all types tied for the
   largest qualifying absolute change, ordered by canonical type
   identifier in byte order. For each, give dates, counts, shares
   and the signed share change. A rise or fall of observed result
   types supplies no finding about personal intent.

## Rival comparisons

1. The owner keeps unique public domains for the active site up
   to the registry limit. An explicit owner action names one public
   page. A replay or concurrent duplicate creates no new fetch.
   Use spec 0082's opt-out, fetch and privacy controls. Record the
   observation's scope, method and time. Use existing evidence for
   a repeated request where allowed, or refuse. The action never
   follows links, accepts a batch or schedules a recheck.
2. For rankings, require a match on query, engine, device,
   measurement scope, sample date and result depth. Overlap
   counts queries seen for both domains. Show the denominator of
   queries with comparable complete evidence. A gap requires an
   explicitly complete sample at its stated depth, with a rival
   result and verified absence of the active site at that depth.
   A missing or truncated sample gives unknown.
3. Speed comparisons use the same metric, observation window,
   measured scope and device. Show the coverage and units. For
   publishing pace, count distinct pages with supplied evidence
   that they were published in the shared interval. Mark partial
   samples. A sitemap date alone cannot prove a publication date.
4. Show structured-data facts from an admitted, scrubbed file.
   Include the source and observation time. One-off checks use
   Receipts mode. Spec 0115 limits its readers to sitemaps, feeds
   and headers in that mode, so this check uses no live markup
   reader. The combined report names the missing evidence for
   each unavailable comparison.

### Evidence for the competitive report (259–269)

Keep DR-085 and the 0082 legal wall in force. Saving a rival
creates one canonical public domain in the active site's
registry. Require a current owner grant. Use the admitted offline
domain grammar and identity profile. Reject non-public and
ambiguous names without DNS or transport. Enforce uniqueness
and the site cap together in the registry transaction, including
under concurrent writes. Aliases stay separate until an authorised
identity migration proves equivalence and updates dependent
references atomically. Erasure removes and invalidates references.
Registration grants no visit, review, disclosure or import. The explicit one-off
Receipts route has its own plan and legal decision. Feature
238 in spec 0131 is independent and supplies no research grant.

| Readout | Comparable evidence and calculation |
|---|---|
| Speed (262) | Use owner-supplied Chrome UX Report observations under an admitted file profile. Bind exact page or origin IDs to the selected own site and rival versions, using admitted aliases only. Match metric, units, statistic, device, observation window and calendar, scope and dataset version. Show both values and rival minus own in original units. An unsupported, suppressed or mismatched observation gives no difference |
| Rank overlap (263) | Pin a canonical query cohort and one capture batch, depth, result family, taxonomy, engine, device, scope and method. Choose one immutable 0043 sample per query with complete eligible scope for both domains. Use 0043 attribution, deduplication and conflict rules. Count queries with both domains seen and divide by the count with comparable complete evidence. Show the numerator, denominator, selected and unknown counts. Zero comparable queries gives no fraction |
| Backlink overlap (264) | Spec 0046 owns authoritative source and link comparison. Show its scoped result by reference and recheck the evidence before display. Do not recount or blend its totals with queries or speed |
| Structured data (266) | Read only admitted file fields bound to the canonical rival page and evidence revision. Show entity ID, admitted property, scrubbed typed scalar value, unit where applicable, capture time and provenance. Render linked entity IDs as inert text. Fields may be unavailable or withheld. Drop over-limit or secret values and show no unadmitted property. Treat all input as data with no executable or linked output and no live markup reader |
| Publication evidence (267) | Bind exact own and rival scopes, canonical page IDs and identity version. Count one verified original publication per page using a declared original-publication field in an admitted owner-supplied file. Match calendar, time zone and closed period on both sides. A conflict leaves that page's date unknown. Edits, sitemap dates and republishes establish no first publication. Show observed counts and coverage for both. With fully covered matched scopes show rival minus own count and per-day rate for each. With partial evidence show observed counts only |
| Combined report (269) | Show each card with its source, scope, date, coverage and allowed status. Missing evidence leaves that card unavailable and has no effect on independent cards. A non-comparable result names the mismatch. No joint score, ranking input or running monitor |

The combined view contains fixed speed, rank, backlink,
structure and publication cards. A card is comparable,
observed partial, unavailable, not comparable or withdrawn.
List the four deferred functions separately as outside version 1.
Each card binds site, actor, rival version, both inputs' IDs,
source revisions, evidence epochs and grant and policy versions.
Bind the selected range and cohort by reference. Never copy
query text to snapshots or logs. Stage card evidence at one
read generation. When permitted evidence has different ages,
label the separate source times. Recheck the full card vector
before returning it, including on reload or restore. Withdrawn
inputs invalidate only their dependent cards. Actor revocation
withholds the report. Never sign, send or publish this view.

Features 260, 261, 265 and 268 are outside version 1. No action,
imported settings, scheduler migration, retry or stale approval
may start sitemap monitoring, named page change alerts, rival
social tracking or rival new-topic alerts. Refuse that effect
with an unavailable result. An owner may still use a current
explicitly permitted one-off scope where its own contract
authorises it. A saved receipt never starts follow-up work.

| Requirement | Verification contract |
|---|---|
| REQ-0092-RIVAL-READOUTS | IT-0092-RIVAL-READOUTS: use synthetic complete, partial, suppressed and revoked sources. Test own and rival speed target identity, scope, unit and polarity, rank overlap at matched depth and family, duplicate and conflicting samples, structured fields for the wrong rival, over-long values and sensitive literals, first publication versus last edit, aliases, conflicting original dates, matched calendars and partial coverage. Keep all denominators and source labels |
| REQ-0092-RIVAL-REPORT | BDD-0092-RIVAL-REPORT: open all cards, withdraw a link reference during assembly, then revoke the actor, replay and restore. Hold dependent cards or the whole actor scope as appropriate. Attempt queued, migrated, retried and restored jobs for all four deferred functions. Observe no network, job, alert or ranking effect |

QR-129 owns exact source, evidence and qualification requirements.

## Data

Use site-scoped records and forced row-level security (spec 0108).
Keep source manifests, admitted observations, owner lists, mappings
and research notes. Raw connector terms stay in spec 0040's stores.
Derived views reference those rows. Keep copied connector terms
out of new imports, notes, logs and exports. Keep scrubbed keywords
from separate owner files with their own source and permission.
Follow the keep-or-delete choice on connector disconnect under
spec 0040. Retained evidence stays labelled as disconnected.
Source withdrawal or erasure clears related derived labels and
caches under specs 0032 and 0040. Withdrawing an actor's grant
blocks that actor's access and pending work. Stale jobs and
restores cannot revive withdrawn or erased evidence.

## Settings

Values below are proposed. Reuse specs 0078 and 0100 for job, memory
and import limits. If a job exceeds an admitted limit, show the
limit and keep prior valid results.

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `research.impression_floor` | whole impressions | 100 | 1 to 100,000 | next analysis |
| `research.click_ceiling_bp` | basis points (100 per percentage point) | 100 | 0 to 10,000 | next analysis |
| `research.intent_min_results` | results in each sample | 10 | 2 to 100 | next analysis |
| `research.intent_shift_bp` | basis points of absolute change | 3,000 | 1 to 10,000 | next analysis |
| `research.rival_limit` | domains per site | 20 | 1 to 100 | next registry write |

## Failure codes

DR-085 reserves SUT-2100 to SUT-2199. SUT-2100 means missing or
insufficient evidence. SUT-2101 means an import was refused.
SUT-2102 means the evidence cannot be compared. SUT-2103 means
spec 0082 refused a rival read. SUT-2104 means a mapping needs
review. SUT-2105 means a research limit was reached. Use spec 0020's
permission response when an action is refused.

## Security and privacy

The owner manages data imports, rivals, priorities and mappings. Read
actions check the active site and the caller's grant under specs 0020
and 0101. Research evidence, suggestions and scores stay local. They
do not publish content or supply ranking signals. The feature adds
zero queries, time or bytes to public-page responses. The isolated
rival request uses the existing crawler's limits. Audit source
decisions and erasure under spec 0032 without raw terms.

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| REQ-0092-ACCESS | Actions MUST check the actor, site and resource limits. | IT-0092-ACCESS |
| REQ-0092-WORKSPACE | The workspace MUST keep observations at their original level of detail and distinguish missing data from zero. | UT-0092-WORKSPACE |
| REQ-0092-GROUPING | Grouping MUST be local and repeatable and the list MUST offer the same selection as the map. | UT-0092-GROUPING, BDD-0092-GROUPING |
| REQ-0092-QUESTIONS | Search confirmation MUST require a recorded search and question views MUST show their evidence. | UT-0092-QUESTIONS |
| REQ-0092-TARGET | A mapping MUST name at most one eligible page and MUST recheck visibility. | UT-0092-TARGET, BDD-0092-TARGET |
| REQ-0092-LISTS | The owner MUST be able to save lists and tags and exclude terms from the board. | BDD-0092-LISTS |
| REQ-0092-SCORE | The board MUST show score components, reasons and missing evidence. | UT-0092-SCORE, BDD-0092-TARGET |
| REQ-0092-IMPORT | Imports MUST be bounded and approved, keep source scope and units, and pass provenance validation. | IT-0092-IMPORT |
| REQ-0092-LOW-CLICK | Low-click reports MUST use comparable counts and label what they measure. | UT-0092-LOW-CLICK |
| REQ-0092-RESULT-SHIFT | Intent change flags MUST require comparable result samples and admitted labels. | UT-0092-RESULT-SHIFT |
| REQ-0092-RIVALS | The owner MUST control the bounded rival registry. Reads MUST be owner-initiated one-off checks under spec 0082. | IT-0092-RIVALS |
| REQ-0092-COMPARE | Rival comparisons MUST use matching measurements and show coverage and unknowns. | UT-0092-COMPARE |
| REQ-0092-REPORT | The combined report MUST name the sources and missing inputs for each finding. | BDD-0092-REPORT |
| REQ-0092-SCOPE | This research module MUST NOT schedule rival monitoring, dispatch provider calls or send monitored rival alerts. Only separately admitted DR-117 and DR-118 clients can call their own endpoints. | E2E-0092-SCOPE |
| REQ-0092-ERASURE | Erasure and source withdrawal MUST invalidate affected evidence and results. Removing an actor’s grant MUST stop that actor’s access and pending work. | IT-0092-ERASURE |
| REQ-0092-FOOTPRINT | Research MUST NOT publish, alter link ranking or add work to public-page responses. | E2E-0092-FOOTPRINT |
| REQ-0092-STRUCTURED-DATA | Rival structured data MUST show only scrubbed, admitted fields with evidence and MUST treat embedded text as data. | UT-0092-STRUCTURED-DATA |
| REQ-0092-SETTINGS | Settings MUST enforce the ranges in this spec with a clear error outside them. | UT-0092-SETTINGS |

## How it is tested

Designed, not run. Spec 0076 governs readiness. Import schemas,
research rules and measured bounds in QR-063 need admission before
implementation can be authorised.

| Test | Fixture and expected result |
|---|---|
| IT-0092-ACCESS | Try empty, one, maximum and over-limit jobs, a dependency failure, a wrong site and a caller without the grant. |
| UT-0092-WORKSPACE | Exact replay is inert. Only an eligible committed 0040 correction replaces the exact source partition. Changing its revision or property binding cannot make older rows current. Engines and mixed levels of detail stay apart. Unknown, truncated, hidden or expired coverage differs from zero. A disconnected source keeps permitted dated rows or removes them according to owner choice. |
| UT-0092-GROUPING, BDD-0092-GROUPING | The same input and model repeat the groups. Given mixed topics, the keyboard-accessible list and the map show the same members. |
| UT-0092-QUESTIONS | Show a question query and a confirmed forum phrase with their evidence. Keep a phrase without a matching search unconfirmed. |
| UT-0092-TARGET, BDD-0092-TARGET | Given the cycle forum, map one exact group and offer a topic for the other only with its required evidence. Change the confirmed page's revision while it stays public, then try a split or merged group, a private page and two confirmations against the same previous mapping revision. A stale confirmation cannot follow the changed version or replace a concurrently committed target. Show a complete no-match result distinctly from a shortlist or an unresolved mapping. |
| BDD-0092-LISTS | Given saved priorities and exclusions, explain a priority point and keep the excluded term off the board. |
| UT-0092-SCORE | Exercise all score points, ties, mixed exclusions, all-excluded groups and waiting versus complete target checks. |
| IT-0092-IMPORT | Repeat a valid import. Give SUT-2101 for unadmitted schemas, unknown units, refused rights and files outside bounds. Zip-slip case: an entry named `../x`, one named `a/../../x`, an absolute path, a backslash or drive-letter path and a symbolic link are each refused with no file written outside the work folder and nothing committed; an archive of 1,000 entries (proposed cap) is read and one of 1,001 is refused whole before any entry is read. |
| UT-0092-LOW-CLICK | Exercise values below, at and above both limits, a zero denominator, missing counts, and incompatible engines or periods. Show SUT-2100 for insufficient evidence. |
| UT-0092-RESULT-SHIFT | Compare ordered complete samples with exact positions. At the current qualification defaults, ten positions split 5 and 5, then 8 and 2, give tied changes of +3,000 and -3,000 basis points. Show both in canonical type order, even with shuffled input. A shift of 2,000 basis points does not meet the 3,000-basis-point threshold. Hold a changed taxonomy, incomplete positions, untyped result, insufficient count or withdrawn source. |
| IT-0092-RIVALS | Try rival counts below, at and over the cap, with colliding concurrent names, unknown aliases, migration and erasure. Saving uses no DNS or transport. Permit one separate approved scoped read. Inspect fetch records for blocked, repeat, concurrent and replayed actions. |
| UT-0092-COMPARE | Match rank, speed and date evidence. Give SUT-2102 for unlike scopes. Check the overlap denominator, verified absence and unknown from a truncated sample. |
| BDD-0092-REPORT | Given a valid rank comparison and no speed file, show the ranking gap and explain why speed is unavailable. |
| E2E-0092-SCOPE | With a saved rival, restart and run background jobs. Observe zero repeat reads, live feeds and monitored alerts. |
| IT-0092-ERASURE | Erase a term and withdraw a file during a job. Replay the job and restore a backup. The evidence stays withdrawn. Withdraw one actor’s grant and verify only that actor loses access and pending work. |
| E2E-0092-FOOTPRINT | Check site writes and ranking inputs and compare the public-page request footprint before and after. |
| UT-0092-STRUCTURED-DATA | Display admitted file fields with their source. Omit private, unadmitted and embedded instruction fields. Confirm the rival crawl invokes no markup reader. |
| UT-0092-SETTINGS | For all five settings, accept each lower and upper bound and reject the values just outside. Preserve the prior valid setting. |

## Wave 2 keyword research verification

These owner bindings trace the current workspace and supplied
evidence routes under spec 0076. Reuse the existing requirement and
test identities. Each fixture is synthetic and exercises its named
feature assertion in the existing case. It is not an observed run.

| Owning design | Applicable source variants |
| --- | --- |
| DES-0092-W2-WORKSPACE | VAR-0092-W2-BOUND-SEARCH uses existing admitted active-site 0040 search copies with their own dimensions and coverage. VAR-0092-W2-LOCAL-GROUPS uses the pinned admitted local model and the same groups for the map and list. |
| DES-0092-W2-SUPPLIED | VAR-0092-W2-SUPPLIED-VOLUME, VAR-0092-W2-SUPPLIED-DIFFICULTY and VAR-0092-W2-SUPPLIED-ARTICLE-VIEWS each retain the admitted owner-supplied source schema and native units. VAR-0092-W2-SUPPLIED-RANK and VAR-0092-W2-SUPPLIED-RESULT-TYPES require a matched supplied sample for their exact comparison. |
| DES-0092-W2-QUESTIONS | VAR-0092-W2-BOUND-SEARCH supplies the observed questions. VAR-0092-W2-OWN-GUEST-PHRASE requires both current own guest evidence and a matched eligible impression-bearing query for confirmation. |
| DES-0092-W2-TARGET | VAR-0092-W2-OWN-INDEX-TARGET uses the active site's eligible guest-visible index and the exact qualified subject and intent match. |
| DES-0092-W2-BOARD | VAR-0092-W2-OWNER-PLAN uses scoped owner lists, priorities and exclusions with admitted current evidence. It produces only local research views and proposals. |

| Feature | Existing requirement and owning design | Existing test and exact fixture assertion |
| --- | --- | --- |
| F-0206 | REQ-0092-WORKSPACE, DES-0092-W2-WORKSPACE, VAR-0092-W2-BOUND-SEARCH | UT-0092-WORKSPACE, FX-0092-W2-WORKSPACE and 0040 AT-CN25. Group a permitted Search Console row and Bing row for one normalised phrase, with hidden and missing rows and unlike report levels. Replay the exact receipt. Offer conflicting same-revision data, then a verified correction for the Search Console partition. Confirm one replacement of those rows and coverage only. Reselect the same property with a new binding epoch and replay the old data. Show only permitted dated history or unknown until separately eligible current rows arrive. No duplicated total, hidden count, fresh observation on replay or provider call. |
| F-0207 | REQ-0092-GROUPING, DES-0092-W2-WORKSPACE, VAR-0092-W2-LOCAL-GROUPS | UT-0092-GROUPING, FX-0092-W2-GROUPING. Use the exact admitted local model and evidence for the same bound site. Permute the arrival order of related, unrelated and ambiguous phrases. Groups and reasons repeat on the frozen stable ordering, with uncertain terms ungrouped. |
| F-0208 | REQ-0092-IMPORT, DES-0092-W2-SUPPLIED, VAR-0092-W2-SUPPLIED-VOLUME | IT-0092-IMPORT, FX-0092-W2-SUPPLIED-VOLUME. Import an admitted, rights-checked volume estimate from an owner-supplied file. Bind date, exact source scope, units and manifest. Keep a missing figure unknown and refuse a malformed manifest. No transport or paid lookup follows. |
| F-0209 | REQ-0092-IMPORT, DES-0092-W2-SUPPLIED, VAR-0092-W2-SUPPLIED-DIFFICULTY | IT-0092-IMPORT, FX-0092-W2-SUPPLIED-DIFFICULTY. Use two admitted files with unlike named difficulty scales and one missing value. Keep the figures and scale descriptions separate. Withhold an unproved rights claim and preserve missing data as unknown. No conversion or outside request. |
| F-0210 | REQ-0092-COMPARE, DES-0092-W2-SUPPLIED, VAR-0092-W2-SUPPLIED-RANK | UT-0092-COMPARE, FX-0092-W2-KEYWORD-GAP. Match complete supplied query, engine, device, scope, date, method and depth. A rival present and verified site absence gives a gap limited to that sample. Truncation or unmatched scope gives unknown or the existing refusal. No gap-fill fetch. |
| F-0211 | REQ-0092-QUESTIONS, DES-0092-W2-QUESTIONS, VAR-0092-W2-BOUND-SEARCH | UT-0092-QUESTIONS, FX-0092-W2-QUESTION-VIEW. Give observed eligible question queries, ordinary queries, hidden and ambiguous rows. Show the qualifying questions with source counts and topic group evidence. Hidden query text cannot be reconstructed from metadata. |
| F-0212 | REQ-0092-TARGET, DES-0092-W2-TARGET, VAR-0092-W2-OWN-INDEX-TARGET | UT-0092-TARGET, BDD-0092-TARGET, FX-0092-W2-OWN-TARGET. Bind one eligible own-site target to the exact group, membership revision, page revision and admitted intent evidence. Change the still-public page, split the group and race two confirmations. Hold stale confirmation and expose no withdrawn source. Compare a complete no-match proof with an incomplete index or shortlist and one saved private page. Only complete current coverage supports absence. An unresolved mapping stays reviewable only within current scope and gives no lack-of-page opportunity point. |
| F-0213 | REQ-0092-IMPORT, DES-0092-W2-SUPPLIED, VAR-0092-W2-SUPPLIED-ARTICLE-VIEWS | IT-0092-IMPORT, FX-0092-W2-ARTICLE-VIEWS. Give two dated permitted owner-supplied Wikipedia article-view series with one missing day. Keep native article, date, unit and gaps. Neither the views nor their absence supplies a search volume estimate or a live feed. |
| F-0214 | REQ-0092-QUESTIONS, REQ-0092-ERASURE, DES-0092-W2-QUESTIONS, VAR-0092-W2-OWN-GUEST-PHRASE | UT-0092-QUESTIONS, IT-0092-ERASURE, FX-0092-W2-FORUM-PHRASE. Derive two suggested phrases from current guest evidence and match one to an eligible impression-bearing query. Keep the unmatched phrase unconfirmed. Withdraw source or rights during rebuild, repeat a late result and restore. No stale confirmation or private raw text is shown. |
| F-0215 | REQ-0092-LISTS, DES-0092-W2-BOARD, VAR-0092-W2-OWNER-PLAN | BDD-0092-LISTS, FX-0092-W2-OWNER-LISTS. Save a scoped owner list and tag including one owner-entered term. Retry the same save and switch the active site. Preserve one entry, its owner-source label and exact site access. |
| F-0216 | REQ-0092-LISTS, REQ-0092-SCORE, DES-0092-W2-BOARD, VAR-0092-W2-OWNER-PLAN | BDD-0092-LISTS, UT-0092-SCORE, FX-0092-W2-OWNER-PRIORITY. Give eligible topics in two independent sites different owner priority tags. Add and remove one authorised tag. Only its current site and eligible score explanation change; priority never supplies observed search evidence. |
| F-0217 | REQ-0092-SCORE, DES-0092-W2-BOARD, VAR-0092-W2-OWNER-PLAN | UT-0092-SCORE, FX-0092-W2-OPPORTUNITY-SCORE. Give cases covering zero through three supported components, same-score groups, exclusions, missing demand and incomplete page coverage. Explain each supported component and use stable cluster identity for ties. Hold insufficient evidence rather than inferring demand or target absence. |
| F-0218 | REQ-0092-LOW-CLICK, DES-0092-W2-WORKSPACE, VAR-0092-W2-BOUND-SEARCH | UT-0092-LOW-CLICK, FX-0092-W2-LOW-CLICK. Exercise both proposed bounds below, at and above, with zero or missing impressions, overlap and a wrong window. Use unique comparable counts and the exact basis-point inequality. Give SUT-2100 for insufficient evidence. Only describe clicks to this site. |
| F-0219 | REQ-0092-LISTS, DES-0092-W2-BOARD, VAR-0092-W2-OWNER-PLAN | BDD-0092-LISTS, FX-0092-W2-OWNER-EXCLUSIONS. Exclude an eligible term in one group and all members of another, then remove one exclusion under current scope. Keep the terms in the workspace. The board uses neither evidence nor priority from excluded terms and omits the all-excluded group. |
| F-0220 | REQ-0092-RESULT-SHIFT, DES-0092-W2-SUPPLIED, VAR-0092-W2-SUPPLIED-RESULT-TYPES | UT-0092-RESULT-SHIFT, FX-0092-W2-RESULT-SHIFT and 0043 F-0229. Compare two ordered complete samples at the proposed minimum and shift threshold. With ten positions and two types, counts of 5 and 5 then 8 and 2 give two equally large changes in opposite directions. Show both with counts, dates and denominators in canonical type order. Reorder input and refuse another taxonomy with the same visible names and positions, wrong scope, duplicate position key or a withdrawn source. This is an observed result comparison, with no inference of personal intent. |
| F-0221 | REQ-0092-GROUPING, DES-0092-W2-WORKSPACE, VAR-0092-W2-LOCAL-GROUPS | BDD-0092-GROUPING, FX-0092-W2-KEYBOARD-MAP. Select the same groups and an ungrouped term on the map and keyboard list. Both views select the same permitted records and expose equivalent reasons and evidence in text. A colour-only or omitted state fails. |
| F-0222 | REQ-0092-SCORE, REQ-0092-ERASURE, DES-0092-W2-BOARD, VAR-0092-W2-OWNER-PLAN | UT-0092-SCORE, IT-0092-ERASURE, FX-0092-W2-IDEA-BOARD. Build ordered eligible ideas and draft prompts with ties, exclusions and a missing source. Withdraw a file or actor grant during the run, replay and restore. Invalidate that evidence, preserve only current authorised results and keep connector text local by reference. No write or provider fallback. |

The supplied variants make no provider call. Any DR-117 or DR-118
action remains under its separate owning scope and live approval.
These links admit neither an extra connector, data source,
automatic refresh, rival monitor, site write nor ranking input.

QR-063 still owns exact schemas, rights, research and model proof,
proposed bounds, coverage and supported cases. Bind the fixture
data, named executable assertion and evidence producer before
deriving a build task. Review exact erasure, access and footprint
under the existing owning tests. No synthetic case admits a
real source or qualifies a runtime profile.

## Wave 2 competitive evidence verification

These fixture assertions use the existing requirement and test
identities. They bind the exact 0092 behaviours under spec 0076.
Feature F-0264 stays with spec 0046; the combined report may read
its current scoped result by reference.

| Owning design | Applicable variant and condition |
| --- | --- |
| DES-0092-W2-RIVAL-REGISTRY | VAR-0092-W2-RIVAL-REGISTRY uses the existing site cap, admitted offline grammar, canonical uniqueness and current owner grant. VAR-0092-W2-ONE-OFF-RECEIPT requires an independent current plan and qualified 0082 legal and 0115 Receipts profile. |
| DES-0092-W2-RIVAL-READOUTS | VAR-0092-W2-SUPPLIED-RIVAL-SPEED requires an admitted matched CrUX file profile. VAR-0092-W2-SUPPLIED-RIVAL-RANK requires complete, comparable admitted 0043 samples. VAR-0092-W2-SUPPLIED-RIVAL-STRUCTURE admits only matching scrubbed typed fields. VAR-0092-W2-SUPPLIED-RIVAL-PUBLICATION uses verified original dates and distinct canonical pages. |
| DES-0092-W2-RIVAL-REPORT | VAR-0092-W2-LOCAL-RIVAL-CARDS requires each card's source, identity, disclosure and current evidence. A card can be held while independent eligible cards remain visible. |
| DES-0092-W2-RIVAL-REFUSAL | The four deferred rows have refusal fixtures only and confer no supported version 1 variant. |

| Feature | Existing requirement, design and variant | Existing test and exact fixture assertion |
| --- | --- | --- |
| F-0259 | REQ-0092-RIVALS, DES-0092-W2-RIVAL-REGISTRY, VAR-0092-W2-RIVAL-REGISTRY, VAR-0092-W2-ONE-OFF-RECEIPT | IT-0092-RIVALS, FX-0092-W2-RIVAL-REGISTRY. Give an authorised owner one free registry slot. Save a valid named public domain once, refusing its normalised duplicate and a competing insert that would exceed the cap. The save makes zero DNS or transport calls. Separately use one approved page plan and a recording fake under the current 0082 legal decision and 0115 Receipts profile. One approved claim gives one dispatch; replay, a concurrent duplicate and a denied plan give none. Profile or rights gaps hold the one-off step without changing the registry. |
| F-0260 | REQ-0092-SCOPE, DES-0092-W2-RIVAL-REFUSAL | E2E-0092-SCOPE, FX-0092-W2-RIVAL-SITEMAP-REFUSAL. Restore an old sitemap-watch flag and its former due time, then restart the scheduler and try its manual resume. The rival list stays visible within its grant. No sitemap watch, timer, request, derived report or alert starts. |
| F-0261 | REQ-0092-SCOPE, DES-0092-W2-RIVAL-REFUSAL | E2E-0092-SCOPE, FX-0092-W2-RIVAL-PAGE-ALERT-REFUSAL. Supply a reachable named rival page with an old page-change schedule. Try to migrate or retry that schedule. Refuse the page alert with zero fetches, changed-page comparisons or notices from it. |
| F-0262 | REQ-0092-COMPARE, REQ-0092-RIVAL-READOUTS, DES-0092-W2-RIVAL-READOUTS, VAR-0092-W2-SUPPLIED-RIVAL-SPEED | IT-0092-RIVAL-READOUTS, FX-0092-W2-RIVAL-SPEED. Give admitted owner-supplied measurements of 2,100 and 2,400 milliseconds with the same metric, statistic, scope, device, dataset and closed window, mapped to exact own and rival targets. Show both and rival minus own as 300 milliseconds. Withhold the difference if the target, unit, window or page versus origin scope cannot be matched or either value is suppressed. |
| F-0263 | REQ-0092-COMPARE, REQ-0092-RIVAL-READOUTS, DES-0092-W2-RIVAL-READOUTS, VAR-0092-W2-SUPPLIED-RIVAL-RANK | IT-0092-RIVAL-READOUTS, FX-0092-W2-RIVAL-RANK-OVERLAP. Give four selected queries from one supplied cohort. Both domains appear in one complete sample and the rival appears with proved own-site absence in another complete sample. The other two are truncated and unlike-family. Report one overlap in two comparable queries, four selected and two unknown. An identical slot replay adds no query or slot. Refuse a conflicting sample or unlike depth and give no fraction for zero comparable queries. |
| F-0265 | REQ-0092-SCOPE, DES-0092-W2-RIVAL-REFUSAL | E2E-0092-SCOPE, FX-0092-W2-RIVAL-SOCIAL-REFUSAL. Import retired rival social tracker settings and try to reuse their former approval or token on restore. No social connector, credential lookup, repeating check, job or alert is admitted. |
| F-0266 | REQ-0092-STRUCTURED-DATA, REQ-0092-RIVAL-READOUTS, DES-0092-W2-RIVAL-READOUTS, VAR-0092-W2-SUPPLIED-RIVAL-STRUCTURE | IT-0092-RIVAL-READOUTS, UT-0092-STRUCTURED-DATA, FX-0092-W2-RIVAL-STRUCTURED-FIELDS. Give an admitted file for the precise rival page and capture version with one approved typed scalar and bounded entity reference. Show the scrubbed scalar, its applicable unit and dated provenance, and render the entity reference as inert text. Refuse or withhold a wrong rival, secret or over-limit value, unadmitted field and embedded instructions. Neither the readout nor a separate Receipts observation starts a markup reader. |
| F-0267 | REQ-0092-COMPARE, REQ-0092-RIVAL-READOUTS, DES-0092-W2-RIVAL-READOUTS, VAR-0092-W2-SUPPLIED-RIVAL-PUBLICATION | IT-0092-RIVAL-READOUTS, FX-0092-W2-RIVAL-PUBLICATION. Give seven verified own originals and fourteen distinct rival originals in matching complete seven-day windows. Show a difference of seven and rates of one and two per day in the declared calendar. A duplicate alias, edit or sitemap date adds no verified original. Conflicting dates stay unknown. Partial scope yields observed counts only. |
| F-0268 | REQ-0092-SCOPE, DES-0092-W2-RIVAL-REFUSAL | E2E-0092-SCOPE, FX-0092-W2-RIVAL-TOPIC-ALERT-REFUSAL. Give one eligible one-off observation mentioning a new subject. Replay its receipt, restore an old topic detector and start the background scheduler. Retain eligible original evidence only. No new read, topic-monitor job or alert follows. |
| F-0269 | REQ-0092-REPORT, REQ-0092-RIVAL-REPORT, DES-0092-W2-RIVAL-REPORT, VAR-0092-W2-LOCAL-RIVAL-CARDS | BDD-0092-RIVAL-REPORT, IT-0092-ERASURE, FX-0092-W2-RIVAL-CARD-PROJECTION. Give one complete comparable speed pair, missing publication evidence, a scoped 0046 reference and a rank source revoked during assembly. Keep the independent speed result with its sources and show the precise unavailable or withdrawn cards. Recheck the vector and current grant on retry and restore. An expired actor sees no report. No blended score, rank input or external refresh. |

DR-085 keeps these comparisons on eligible supplied or separately
approved one-off evidence. DR-118's named paid routes keep their
own spec 0133 contract and exact approval. These fixtures supply
no authority for a paid request, new source or repeat monitoring.

QR-129 and the owning profiles still require proof of source
rights, exact manifests, canonical identity, comparable coverage,
current actor, limits, withdrawal and the real evidence path.
A positive synthetic result on an admitted fixture is a planned
assertion. Real-source and task admission need their own evidence.

## Changes from the source

Features 208 to 210 and 262 to 263 use supplied evidence in place of live
feeds. Features 213, 266 and 267 use supplied article views, structured data
and publication evidence respectively. Features 260, 261, 265 and 268 are
after version 1 (DR-085).
