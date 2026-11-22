# 0138: Empirical Update Lab

Figures: every numeric limit, threshold and budget in this spec is proposed
until measured on the Dell or settled by a QR.

## Purpose and authority

The Lab helps an owner inspect which page traits travelled with
measured gains or losses on their own site after confirmed Google
updates. It supplies evidence and local Editorial Coach advice. A
result describes observed resemblance or association in a named
sample. It cannot identify Google's private rules or promise a
ranking result.

[ADR-0024](../adr/0024-own-site-update-lab-evidence.md) and DR-124
authorise only eligible settled Search Console comparison labels
from the active owned site and the declared matching controls.
Before access, verify that the current protected decision and
[spec 0072](0072-ranking-and-spam.md) policy admit this purpose.
An ordinary setting, site consent or tracker grant cannot stand in
for study approval. All study profiles need source coverage,
current authority and protected isolation proof.

DR-119 applies to the full dependency graph. Exclude other sites and
rivals, including public reports, files, captures, labels, pre-trained
site models, embeddings and derived facts sourced from them. Do not
read DataForSEO, imported result rankings, GA4, Matomo, Bing, browser
referrers, scraped Google results, analytics joins or Buddhi and
Rakshak output, even as controls. Keep those timelines separate.
The Lab receives no connector credentials and starts no pulls,
crawls, outside inference, notifications or native writes. The
local fitter and scenario worker have typed inputs and outputs.
Nothing from the Lab enters ranking, relevance, trust, spam detection,
model promotion, calibration, ranking research, Optuna, Ganit Vikas,
candidate or anchor selection, or release qualification.

## Words used here

These words have one meaning in this spec.

| Word | Meaning |
|---|---|
| Study | One frozen, owner-approved comparison run on the active owned site |
| Eligible comparison | A settled spec 0137 before-and-after result from the same site |
| Feature card | The record of one page's measured traits, taken before any gain or loss is read |
| Label | "Gained" or "lost", taken from a settled spec 0137 comparison |
| Label-blind freeze | Fixing every input before any label is opened |
| Negative control | A check that should find nothing, to catch false findings |
| Winner Match | How closely a page resembles pages that gained in this site's own study. It describes resemblance, never Google's rules |

Example: Ravi's forum has 40 settled pages after a confirmed core update.
He approves a study. The Lab freezes each page's traits, then opens the
labels, and a negative control with shuffled labels finds nothing. The
card reads "Pages that gained here tended to answer in the first 100
words (observed, 40 pages, low confidence)". It never says why Google
moved anything. All figures are illustrative.

## Scope and applicability

The owner reviews feature cards, studies, Winner Match, simulations
and advisory rules for one active owned XenForo or WordPress site.
The Rust service owns each effect. Alekh shows the protected result.
An eligible signed native publication or major edit may queue its
exact feature card. A card event cannot start or approve a study.
An unsupported platform, native content subtype or projection holds
the affected operation using its existing safe reason and message.
Google Search Update Tracker 0137 owns incident timing and labels.
Spec 0138 owns local comparisons and guidance. It contributes no
signal, feedback, edit or action to ranking, release qualification
or a native site.

## What this spec designs

| Owner | Interface and boundary |
|---|---|
| Rust service | Plans studies, validates site and source authority, owns the case set and stores, evaluates bounded results and confirms every durable transition. |
| Spec 0137 tracker | Supplies a typed eligible generation and native event facts from the same owned property. The Lab holds when the projection or proof is absent. |
| Spec 0019 scheduler | Admits frozen jobs using spec 0078 memory and CPU budgets. Checks cancellation, pause, replay and fair turn-taking. |
| Spec 0104 Python fence | Permits reviewed local feature and scenario calculations only on typed data. Rust controls admission, the declared rule and resulting authority. |
| Tauri and Angular | Render the owner plan, scoped status, Alekh timeline, Winner Match and evidence receipts through SDS. |
| Local MCP | Prepares scoped requests and reads authorised typed results. The trusted desktop owner confirmation owns every heavy action. |
| File ledger and protected storage | Specs 0030, 0032, 0038 and 0118 bind temporary files, protected results, recipient grants, copies and erasure. |

All work runs on the owner's PC except a separately approved
sealed archive move under the existing storage contract. Nothing
in the Lab adds public-page queries, bytes or processing.

## Route and frozen inputs

Register owner-only local desktop actions
`update_lab.preview`, `update_lab.run` and `update_lab.read` under
0020. Preview accepts a site, verified property and date span and
returns the exact plan and expiry after checking policy and scope.
It reads only permitted selection and coverage metadata and never
opens feature values or labels. Run accepts that plan digest and
one current exact live owner confirmation. Read accepts a study
ID and rechecks current site and source permissions before every
view. The separate typed Rust job runs through 0019
using 0078 resource admission. The job may read only the approved
immutable per-study projections via registered site-scoped interfaces.
A scoped local owner view presents the results. The separately registered
Alekh view and MCP projections have their own grants and schemas.
There is no public, connector, command-line or plaintext export route.

Every plan binds site, exact owned and verified property, policy and
owner grant generations, five incident identities and current
revisions, allowed time span, time-zone database revision, input
manifests, cohort and matching protocol versions, and a canonical
plan digest. Review shows the periods, all support and refusal
thresholds, limits, retention and expected local result prior to
confirmation. A changed binding requires fresh confirmation. Read
and commit both check the current grant, scope, pause, corrections
and erasure fence. Never start a provider request to fill a gap.

Eligible input has three kinds:

| Input | Exact source and allowed projection |
|---|---|
| Incident and labels | Current settled 0137 JSON-confirmed eligible Ranking incident and unfiltered page-day comparison generation for its admitted surface. Retain exact before and after dates, incident revision, label, support summary, compatible projection and receipt identities. Read no raw query, feed text or other provider facts. |
| Own content identity | Signed 0021 guest-visible page and canonical/native identity, 0137 native revision evidence, historic site section and content type, complete-reading and feature snapshot receipts. Remain inside the active site's exact property mapping. |
| Match controls | Approved before-window Search Console page aggregates, calendar, coverage and expected measures for the exact surface, plus native platform, section, content type and language. Keep these separate from the candidate feature vector. |

Do not derive page totals by adding query, country, device or
appearance projections. Require a current complete page aggregate
profile evidenced by 0040 and 0043. The current 0040 projections
give no proof of complete comparable page-day rows. Keep real
study intake held with SUT-4501 on those projections. A final API
day or completed pagination gives no assurance of provider coverage.
Partial, capped, withheld, disconnected without current archive
grant, unverified, corrected or incomparable
observations cannot become zero, absence, a label or proof of rank.

## Cohort and labels

The primary profile is Web Search, whole property, with the same
authoritative unfiltered page-level grain in all periods. Use
the site's single admitted native language profile and content
classes with verified identities and snapshots. Only pure Ranking
Core rollouts of confirmed compatible scope may enter that profile.
Admit other eligible 0137 surfaces and incident classes under
their exact compatible measure and incident profiles. Hold mixed
product events, uncertain interval or scope, overlapping applicable
rollouts or operational incidents, and events without a valid current
settling and finality receipt.

Use exactly 28 whole Search Console native days before the start and
28 starting at or after the 0137 settling deadline. Use 0137's
current zone revision, final page-day coverage, full-window and
split-half support and exact applicable measure and label rules.
A shorter tracker comparison or unproved page aggregate waits for
a compatible 28-day generation. No reconfiguration or extra pull
follows. A stronger
Lab exclusion cannot widen the tracker cohort. A row with only a
query projection or a missing day has no Lab outcome. Record each
excluded row once at the first failing check: scope/grant, event
validity/overlap, page eligibility and lineage, finality/coverage,
support, then label eligibility.

A page must have an unambiguous native identity and canonical family,
be continuously public and guest-visible, and keep the same admitted
revision, canonical mapping, section, locale and content class from
feature cutoff through the end of the after window. New pages and
changes during either window, rollout or settling are reported by the
tracker alone. An observation interval that can cross a boundary is
insufficient. Family identity binds all eligible URL aliases and
revisions of the same site page. Hold merges, splits or ambiguous
joins. Keep unsupported and refused counts visible in the owner's
protected report.

Use the exact 0137 sustained-winner label as positive and hard-drop
as negative. A survivor marker adds no row. Small changes, a
nonqualifying measured change and unknowns do not become negatives.
This deliberately compares outcome extremes. Counts show the whole
admitted input population, eligible labelled rows, unmatched rows
and matched pairs. A matched score is not a population outcome rate.

## Feature snapshots

Use the latest valid already-held snapshot at or before the start of
the before window, captured no more than 28 days earlier. Both its
source observation time and its accepted storage time must be at or
before that cutoff. Require unchanged exact native revision and
canonical family from snapshot through both windows. Never
reconstruct missing features from a later snapshot, even if its
reported content hash is identical. Pin the typed field inventory,
parser and authoring projection versions. Prefer no feature over
inventing a neutral value.

The baseline vector uses the existing complete 0026 editorial body
projection and the corresponding authoritative link inventory. Use
the native article body or forum opening post. Exclude replies,
quotes, signatures, hidden zones, widgets, navigation and previews. Require
complete coverage of the exact range and known editorial-word count
between 100 and 20,000 inclusive. Use each projection's named word
count and canonical structural boundaries. Count authored logical
objects once by position inside that range, independent of piece
overlap or chunk boundaries.

| Feature ID | Exact observation |
|---|---|
| `body_words` | Eligible editorial word count |
| `heading_density` | 1,000 times the count of approved non-empty editorial heading blocks, divided by editorial words |
| `internal_link_density` | 1,000 times visible authored links to an eligible same-site canonical target, divided by editorial words. Exclude Sutradhar-applied links. A missing target-type or authorship fact makes this feature unavailable. |
| `list_density` | 1,000 times the count of non-empty authored list items, divided by editorial words. Count a nested item once at its own position. |

Store ratios as exact nonnegative numerator and positive denominator.
Use checked widened cross products for comparisons. If a parser
cannot distinguish a required boundary or count, mark the feature
unavailable. Zero is eligible only when complete coverage proves
that the count is zero. Keep no raw text, addresses, query terms,
member identities or provider samples in the Lab store or its
diagnostics. Typed snapshot references never grant raw content access.

### Trait registry

A versioned registry entry binds a stable trait ID, the six-group
assignment or no fit group, field type and unit, the owning source and
version, exact extraction and exclusions, required scope, support and
denominator, bounds, availability reasons and permitted consumers.
Hash the registry and approved entry versions into the plan and
pre-label freeze. Each field carries its exact revision, observation
and accepted storage times, source receipt, coverage and value or safe
reason. A card can show typed observations with a partial label.
Fitting and Winner Match use only exact admitted comparable fields
with complete coverage of their declared scope. Adding a fit field,
or changing its meaning, creates a new plan and comparable profile.

| Group and trait IDs | Typed observation and admission |
|---|---|
| Answer and structure · `opening_words`, `passage_count`, `words_before_answer` | Count words in the first eligible editorial prose block, non-empty eligible authored prose blocks, and words preceding the first admitted exact answer span in source order. Count using the same 0026 word and block projection. The profile or owner mark must be bound to the revision and information need before cutoff. Give separate reasons for no admitted answer span and incomplete extraction. An answer at the first eligible word yields zero distance. |
| Answer and structure · `heading_density`, `heading_level_counts`, `skipped_heading_level_count`, `repeat_heading_count` | Keep the existing exact density. Count non-empty eligible authored headings by observed level in document order. Use admitted 0115 skipped-level and duplicate observations with their exact definitions. If those facts are absent, hold those fields. A body with no headings has zero heading density only when the body and heading extraction are complete. |
| Entity and intent coverage · `covered_entity_count`, `covered_intent_count`, `assessed_intent_count`, `unassessed_intent_count` | Count distinct admitted matches to already-held own-site entity and owner-approved intent keys. Pin the checklist version and eligible denominator. An unassessed checklist item is separate from an assessed missing match. No provider, cited page, personal identity or query text enters the registry. |
| Originality and freshness · `eligible_block_count`, `repeated_block_count`, `publish_age_days`, `edit_age_days` | Count authored blocks and positions with a match under the pinned own-site fingerprint profile. Freeze the comparison inventory and its coverage at cutoff. Count complete 86,400-second days from the verified native event to the observation UTC. Keep crawler observations as a separate card fact. No smoothed age or guessed publish date. |
| Readable and usable form · `sentence_count`, `readability_measure`, `viewport_measure` | Reuse exact 0026 counts and named language-specific measure where admitted. A rendering field needs its exact measured unit, versioned 0044/0115/0017 source, viewport, profile and scope. A structural count is a distinct field. A missing or unproved measure stays unavailable. |
| Media and tables · `editorial_image_count`, `eligible_alt_image_count`, `editorial_table_header_count` | Count eligible authored logical images, the subset with admitted eligible alt text and non-empty authored editorial tables with a verified row or column header. Keep uncertain assets and layout tables separate. Count logical objects once across chunks. |
| Internal links and attribution · `internal_link_density`, `outgoing_editorial_link_count`, `incoming_editorial_link_count`, `referring_family_count`, `byline_presence` | Keep existing exact link density. Measure only allowed own-site editorial edges with an eligible canonical endpoint and independently covered graph scope. Count distinct allowed referring families by protected key. Reuse the exact displayed-attribution presence fact for that revision. Exclude names, account or identity matches, scores, ratings and protected attributes. |
| Existing baseline · `body_words`, `list_density` | Keep the formulas in Feature snapshots. A weighted plan states the group assignment for each enabled scalar before any outcomes. |

An owning projection must publish an exact definition for each field
it supplies. A registry entry naming a missing adapter, unknown unit,
partial denominator or unsupported comparison does not activate the
field. Category counts and booleans are display fields until a
versioned fit definition declares their exact scalar treatment.
A visible trait is an observed fact with coverage. It gives no
global originality, truth, author reputation or accessibility grade.

## Label-blind freeze

Before the first after-window read, record the selected plan and
incidents, typed registry and extraction profile, ordered pre-window
candidate and sampled family keys, partition and earliest-event
assignments, exact trait values and refusal reasons, graph/checklist
and revision digests, and canonical receipt hash. The pre-freeze
reader can access only the declared pre-window identity and feature
manifests, approved source-level metadata and policy. It receives no
label, after-window metric or support, changed-page exclusion,
survivor flag, post-cutoff snapshot, coach response or model score.

Every trait dependency used for a frozen value must have been
observed and accepted by the pre-window cutoff. This includes answer
marks, intent checklists, duplicate inventories, link graphs, render
receipts and author facts. Where the feature or a dependency arrived
later, preserve the frozen unavailable reason. An identical body hash,
a backdated source timestamp or a prior CMS revision cannot backfill
a cutoff. Use post-update snapshots only for separately dated cards.

After freeze, open only the fitting partitions' labels and support
under the declared eligibility order. An exclusion keeps its frozen
slot and never promotes another candidate. Freeze choices, ranges,
weights, baseline, support rules and the fitting receipt before
opening evaluation labels. A real correction to pre-cutoff evidence
invalidates the study through the existing fence. It cannot rewrite
the old freeze or retain the old approval. Changing only sealed
outcomes cannot change the freeze hash, sampling or feature vector.

### Subtype and temporal negative cases

Extend UT-0138-FEATURES, UT-0138-WINDOWS and PRP-0138-STATE.

- At exact cutoff accept a complete already-stored value. Move source
  observation, acceptance or any dependent checklist, byline or
  render receipt one tick later in separate fixtures. Each becomes
  unavailable in the frozen vector even if body bytes are identical.
  The later card may show only an observation with its own later date.
- Use a valid literal WP or XF heading correctly inside the editorial
  range, an identical heading in a quoted block and the same eligible
  heading repeated in overlapping source chunks. Count only the one
  authored object. Keep an unavailable heading parser distinct from a
  complete heading-free body. With an incomplete lineage, a repeated
  heading or skipped level cannot become zero.
- Swap sealed winner and drop labels, after support and held-out page
  lists while preserving the pre-freeze inputs. Assert identical
  sample and freeze hash. Instrument a pre-freeze attempt to read
  each sealed field and the early held-out read. Refuse each access.
  Swap only an outcome after approval and assert no candidate
  replacement or movement across split. A real generation correction
  holds the stale study under the correction fence.
- Give a Ranking-only core, spam and Discover event the same dates.
  A subtype mismatch with the chosen study refuses before labels.
  For the admitted Discover click-only variant, refuse a Web Search,
  Google News or News-tab label and never borrow position. The Web
  variant falls short if its expected position is absent. A US
  English-only incident and a whole-property broader or unknown
  scope remain held unless exact source evidence is separately admitted.
- Give two families identical metric controls but different native
  platform, content class, section or language in separate pairs.
  Each pair is incomparable. Use a WP revision from another native
  content class or an XF reply at the opening-post position. Refuse
  that typed trait source. An alias with ambiguous lineage or a
  later move between sections cannot repair the pair.

## Split before reading labels

Require exactly five consecutive eligible incidents of one admitted
profile within the confirmed plan. Order by start and canonical
incident ID. Require
non-overlapping full observation intervals, including rollout and
settling. A gap with an intervening possibly applicable incident, unresolved
correction or changing scope holds the study. Revalidate older
incidents through 0137 before selecting them. The
plan chooses dates and site before results. Within this bounded span,
use the oldest three incidents for fitting and the newest two as
held out. Require all five. An incident correction invalidates the
whole study rather than silently replacing one interval.

Assign each canonical page family to a stable bucket before reading
outcomes. Hash the frames for the literal `0138/family-split/1`, site ID,
profile version and family key in that order with SHA-256. Each
frame is exact UTF-8 text prefixed by its unsigned 64-bit big-endian
byte length, as in 0064. Represent the profile version as canonical
decimal text with no leading zeros. Interpret
the first eight digest bytes as an unsigned big-endian integer.
Its remainder modulo five assigns buckets 0–2 to fitting and 3–4
to evaluation. Reject conflicting or unresolved family mappings.
Pin pre-window candidate keys and input digests at split time.
Within each partition retain only the family's first event with
admitted pre-window identity and scope, regardless of its later
coverage, outcome or revision. One family can appear at most once.
Merges or corrections affecting assignment hold the old study.

For each event and its allowed partition, enumerate at most 10,000
pre-window candidates from the complete declared site inventory. Deduplicate
exact replay and refuse conflicting duplicate identities. If the
source reaches the cap with rows still unread, hold the study.
Before reading after-window support or outcomes, order the eligible
pre-window page keys by length-framed SHA-256 over
`0138/page-sample/1`, site ID, profile version, incident ID and
canonical page key. Compare all digest
bytes in unsigned lexicographic order, then the page key. Keep
the first 256 keys, or all if fewer. Report eligible count, retained
count and actual scope. Never replace an excluded or unlabelled key
with one outside the fixed sample. Use observed counts only.

## Match within an event

Join the two 0137 extreme labels only after freezing the split and
sample. Within one incident, match a winner with a hard drop from
the same property, site, native platform, section ID, content class
and locale. Require before-window values from the same page aggregate
profile. Pair eligibility uses only these exact controls:

- Larger before impressions and larger before clicks may each be
  at most twice the corresponding smaller total.
- Absolute difference in before impression-weighted positions is at
  most two position units on a position-enabled profile. Omit this
  term on a separately admitted click-only profile. An unexpectedly
  missing expected value makes the pair incomparable.
- Compare each page's late-to-early before-window click rate across
  its two 14-day halves. Using the equal durations, each ratio is
  late clicks divided by early clicks. Both halves need at least
  five clicks and 40 impressions. Absolute difference between the
  two ratios may be at most one quarter. Hold a missing half or an
  unproved denominator.
- Larger editorial word count may be at most twice the smaller.
  This control leaves within-band variation and is recorded beside
  the word-count finding.

Within each incident, sort eligible edges by absolute before
position difference where applicable, then absolute click-trend
ratio difference, absolute before impressions difference, absolute
before clicks difference, then winner and drop canonical page keys.
Use typed integer and rational comparisons throughout. Traverse this
list once. Keep an edge only if neither page is already matched.
Never retry matching with widened bounds. No page can serve as
control across incidents. Exact ties in all recorded inputs follow
the named canonical page order, with no clock or database row order
influence. Report the omitted counts and each signed pre-match and
post-match control difference distribution by incident. Omit small
disclosure cells under the owning privacy projection.

## Fit and evaluate the local association

For each declared feature and pair with both values, count whether
the winner is larger (P), smaller (N) or equal (T). Record missing
feature pairs separately. Require at least 24 non-tied pairs in each
fitting incident. Learn a direction only if every fitting incident
has the same nonzero sign of P minus N and, in each, the magnitude
of (P minus N) divided by (P plus N) is at least one fifth. Ties and
missing pairs stay in coverage, but supply no direction. There is
no feature selection, interaction or weight optimisation in this
baseline. Freeze the four directions or explicit holds and all fitting evidence
before opening held-out labels.
If no feature qualifies at fitting, record support and leave the
held-out labels sealed.

In each held-out incident count directional agreements A,
disagreements D, ties and missing pairs. Require at least 20
non-tied pairs and A/(A+D) at least three fifths in both incidents.
Show the count and agreement fraction per incident, the unweighted
mean of the two incident fractions and the minimum
and maximum incident fractions. Show a fixed one-half reference
for the majority rule without treating it as a probability model.
Counts and variation describe these pairs and incidents. They
provide no population confidence interval or causal significance.
A result below either gate is reported as unsupported, with no
alternative selected. Apply the same rule to every declared feature.

## Negative controls

1. Screen for pre-existing trends without making a second outcome
   label. In each held-out pair orient the two pages by the higher
   and lower late-to-early click ratios from the before-window
   matching control. Ignore the eventual winner/drop orientation.
   Count how often the learned feature direction agrees with this
   pre-update order, excluding and separately counting exact trend
   ties, feature ties and missing values. Require at least 20
   non-tied pairs and agreement from 40% through 60% inclusive in
   both held-out incidents. The control uses the already-matched
   pairs and existing final rows. No new comparison labels, periods,
   pulls or rematching. A failed or unavailable control holds the
   finding for that feature.
2. In synthetic fixtures reverse winner and drop orientation after
   freezing pairs, without changing identities, split, values or support. Every
   signed feature comparison must reverse and ties must remain ties.
   Independently use a constant feature and an intentionally
   later-dated value. The constant yields ties and no learned
   direction. The later snapshot is refused before fitting.
3. Run the existing rank, eligibility, spam and activation boundary
   checks against the same immutable fixture with the Lab absent,
   held and report-capable. Outputs and control decisions outside
   the Lab must stay identical. Record independent negative route
   evidence for connectors, providers, cross-site data and feature
   transfer. Passing a Lab metric confers no ranking or export right.

The pre-trend check is a screening control, with no claim of independence
or removal of all confounding. Every report states that matching
does not control for all market, site or search system changes.
The owner sees scope, exclusion counts, outcome selection, match
balance, coverage, held-out and pre-trend control results, and safe evidence
references. This baseline report offers no forecast, advice to
apply a treatment, causal conclusion or raw source content. Protected drill-down
requires owning site permission and shows only the declared typed
facts and safe source references.

## Durable state, limits and retention

Use one site job at a time through 0019, below interactive priority.
Freeze a versioned input receipt before reserving work and commit
the entire report and safe event in the same fenced transaction.
Use states planned, frozen, fitted, evaluated, published, held and
revoked. Publication means the protected local report becomes
readable. It is reachable only from evaluated with current
authority. The safe owner view reports a held reason and coverage
for an authorised empirical hold. A revoked result is unreadable
except for its permitted refusal status.
The baseline study checkpoints only at a completed phase or event.
The batch scenario profile has separate bounded block checkpoints.
Either resumes only with exact input and current grant, policy, pause,
correction and erasure epochs. Discard an invalid stage.
Reconcile a lost commit reply using the same plan and input digests,
without duplicate report or logical effect. A correction after
publish invalidates the old result. Reopening it shows only its
safe withheld status until a newly confirmed study is eligible.

| Typed record | Identity and content |
|---|---|
| Plan and input | Site, actor, property, decision and consent generations, cutoffs, incident and projection digests, case manifest, profile and policy |
| Sample and fit | Plan, partition and event IDs, opaque family and pair identities, typed pre-window controls, frozen direction or hold, feature and cohort coverage, match digest |
| Report and receipt | Plan and input digests, algorithm and platform, bounded counts, results and control outcomes, authorised permissions and epochs, exact store and verified commit outcome |

Cap one immutable generation at five primary incidents, 50,000
examined page rows total, 256 sampled pages per event and 16,384
eligible pair edges per event. Use at most 128 MiB resident memory
and 64 MiB protected temporary bytes, or any stricter admitted 0078
limit. Stop after 15 active minutes or 24 elapsed hours; require a
trusted clock. Refuse an unproved fit or storage profile. Stop,
reconcile and clean scratch under 0030 and 0032. No background retry
carries authority across an invalid plan. Hold an unperformed effect
unless exact permission and replay proofs are current.

Use site RLS, 0118 protected stores, the 0030 managed ledger and the
0032 erasure registry. Retain derived studies until the earliest of
180 days from commit, a shorter owning retention, or source expiry
or withdrawal. Revoke pending, published and cached data access
before cleanup, including views and restored copies. A partial purge
stays fenced and reconciliation accounts for every managed target.
An unaffected plan cannot inherit revoked evidence.

## Requirements and baseline verification

| Requirement and proposed design | Exact test obligations |
|---|---|
| `REQ-0138-ADMISSION`, `DES-0138-LOCAL-JOB`, `VAR-0138-WEB-CORE` | `IT-0138-ADMISSION` tests missing authority, changed policy, false owner grant, rival data, added providers and attempted rank, spam and write consumers. `BDD-0138-OWNER-PLAN` checks exact one-use plan confirmation and withheld result. |
| `REQ-0138-COHORT` and `REQ-0138-SNAPSHOT`, `DES-0138-FROZEN-STUDY` | `UT-0138-WINDOWS` covers boundary days, spring and autumn time changes, settling equality, source correction, outages and missing/partial days. `UT-0138-FEATURES` covers duplicate source blocks, zero versus absent, post-cutoff feature, merged aliases, author or target-scope omissions and bounded arithmetic. |
| `REQ-0138-MATCH` and `REQ-0138-VALIDATE`, `DES-0138-PAIRED-CONTRAST` | `UT-0138-MATCH` uses an independently enumerated small graph, reversed input order, shared candidate, equal controls and cap boundary. `UT-0138-CONTRAST` covers all ties, direction reversal, exact threshold equality, pre-trend control failure and held-out incidence. `PRP-0138-ARITHMETIC` checks rational ordering and checked bounds. |
| `REQ-0138-RECOVER` and `REQ-0138-ERASE`, `DES-0138-LOCAL-JOB` | `IT-0138-REPLAY` tests loss before and after report commit, stale worker, corrected cohort and same manifest replay. `E2E-0138-WITHDRAW` tests inaccessible store, partial erasure and old backup fencing. Repeat protected cases on admitted x86-64 Ubuntu 26.04 LTS, Debian 13 and Windows 11 profiles. |

### Worked synthetic fixture

For the matching fixture use winners W1 and W2 at before positions
10 and 8, and drops D1 and D2 at 10 and 12. Give every page the same
controls otherwise, including 20 clicks and 100 impressions in each
before half. Eligible edges are W1–D1, W1–D2 and W2–D1. The frozen
greedy traversal selects only W1–D1 and reports W2 and D2 unmatched.
Shuffling input cannot change the result.

In the contrast fixture use P=15 and N=10 in each fitting incident,
then A=12 and D=8 in each evaluation incident. Both threshold
equalities pass their fields. For the control use 8 agreements of
20 comparable pairs in one incident and 12 of 20 in the other. Both
control boundaries pass. Change one 12 to 13 with the same
denominator and the affected finding holds. These are test inputs.

For `PRP-0138-STATE` model two sites, two family keys, candidate and
current generations, policy unlocked or locked, two workers,
temporary and published state and one old backup in Quint. Explore
approve, freeze, fit, unseal, commit, lose reply, correct, revoke,
erase, restore and reconcile transitions. Assert no locked ingestion,
no other-site projection, no future feature, no family crossing
the split, no early held-out read, no publication from stale
authority, one commit per plan and no restore resurrection. Replay
model traces through Rust service, job fence and ledger in the
admitted Quint Connect route. Bind model, property, bounds, solver
and test source revisions. The model is evidence only for its
property and bounds. Independent review must inspect the complete
read, execution and restore paths and the absence of consumer edges.

## Authority and profiles

Freeze one variant in the owner-approved plan. Verify each
profile's exact incident and measurement support independently.
Site and property scope cannot widen at run time.

| Profile | Exact evidence and treatment |
|---|---|
| `VAR-0138-WEB-CORE` | Use the full paired position and click profile and the settled Core window under 0137. |
| `VAR-0138-WEB-SPAM` | Use the same evidence only for a confirmed Ranking-only Spam rollout with compatible own-site scope. The incident name provides no spam classification or detector input. |
| `VAR-0138-DISCOVER-CLICK` | Use the exact Discover rollout, bounded settled Discover page aggregates and the 0137 click-only label where position is not supplied. Use the same control order with the position term absent by design. |
| `VAR-0138-GOOGLENEWS-CLICK` | Admit only a specific compatible Google News event and typed 0137 click-only envelope. Keep it separate from the News tab in Web Search. |
| Other Search Console surfaces | Retain the individual tracker views. A Lab variant needs a matching 0137 official incident scope, same-grain page coverage, expected fields and its own independently qualified descriptor. Missing expected measures or scope holds. |

Reuse the seven-day settling and final native-day checks of 0137.
Bind nature of incident, surface, property, language, exact
projection, policy and time; no cross-surface labels, weights,
reference cohorts, aggregation or proxy substitution. The five
incident fitting design applies within
each compatible profile. Report insufficient events or coverage.
Do not widen a profile, shorten settlement or borrow another site's
examples to obtain a result.

## Feature cards on a site timeline

Alekh, historically QuarryView, owns the display. On an eligible
publish or major edit event, queue a feature card for the exact
native current public revision under 0019. Reuse 0137's major-edit
predicate, signed native timestamps and provisional observed-change
handling. A card opens a protected existing projection only. It
does not capture a fresh page, request indexing, pull results, run
a provider or guess a publication date. If a required fact is
unavailable, keep its reason and coverage.

A revision card compares compatible typed traits from an older
observed exact revision and the selected revision. An update card
shows the last eligible pre-cutoff trait snapshot beside the
post-update observed trait snapshot and the separate settled
search comparison, each with its own dates and source revision.
New or edited pages follow the timeline's changed-page cohort.
Keep event notes for them, and withhold the unchanged-page Lab
label. A later snapshot never replaces a fitter's pre-period
evidence. Events can appear during rollout; settled result
labels and coach grading wait for final evidence.

Keep data at the already-admitted exact public revision and source
owner. The following are typed feature profiles, separate from
live rank inputs. Apply complete-source and approved-parser
requirements before reporting each count.

| Trait | Observation and limit |
|---|---|
| Answer-first and passage structure | Count eligible opening and subsequent text blocks, and words before the first explicitly admitted answer passage for an owner-approved information need. Use a versioned structural profile or an exact owner-marked passage, recording which. If a sound passage cannot be identified, leave the distance unavailable. It is not a judgement of truth or intent. |
| Headings | Count non-empty editorial headings per 1,000 body words and report measured nesting/repeated heading observations under 0026/0115. Missing extraction never confirms no headings. |
| Entities and fan-out coverage | Compare existing eligible own-site entity references and an owner-approved typed intent checklist to the exact page. Show covered, missing and unassessed counts and denominators. Use no live search, competitor terms, inferred persona or external model or prompt. |
| Originality | Show distinct visible editorial blocks and matched eligible duplicates within the site's exact captured comparison set, with pinned fingerprint version and coverage. Compare matches before calling a block repeated. The result establishes no global originality or authorship. |
| Freshness | Keep verified native publish/edit dates separate from observed crawler times and displayed dates. Use the exact captured time and age basis. A new date or hash discrepancy gives no quality verdict. |
| Readability | Show sentence and word counts and the admitted language-specific measure from 0026 with its version and reason when unavailable. Do not substitute a score from another language or mix unlike denominators. |
| Media and tables | Count admitted editorial images with and without eligible alt text, and non-empty editorial tables with a declared row or column header. Keep layout, accessibility and unavailable assets distinct. A table or image count gives no usability judgement. |
| Internal authority | Show eligible own-site incoming and outgoing editorial link counts, distinct referring page families and captured graph coverage. Do not use live order, popularity, external backlink data or a named authority grade. An incomplete graph cannot prove absence. |
| Author and attribution | Record only verified displayed byline/attribution presence and consistency as typed fields with source and coverage. Disclose no names, accounts, demographics, protected attributes, identity matches or reputation in evidence. |
| Usability | Show only admitted comparable own-site rendering or structural facts under 0044, 0115 and 0017, with viewport, profile, coverage and measurement time. Static structure and real measurement remain distinct. No new live render or probe follows. |

Each card has site, stable page family, exact revision, observation
time, trigger, evidence generation, feature profile, typed measures
and units, coverage and safe refusal reason. A trigger is not a
measurement. Treat partial coverage, missing values and actual
zero separately. The owner's current consent, source, erasure and
read authority apply both when opening and when paging the card.

## Winner Match and receipts

On a separately approved current card, compare to the distinct
winner pages of the frozen fitting set of one compatible protected
profile. Do not read the evaluation set as reference material. Keep
same site, property, surface, locale, platform, section and
content class, and comparable feature projection version. Exclude
the target's family, erased or revoked rows and profiles or dates
not in the owner-approved plan. Any unavailable selected feature
makes that pair incomparable. Never substitute a rival or adjacent
surface. Show the count of eligible references, comparable
references, selected examples and measured trait dimensions.

For the proposed reference comparison, derive a closed min/max
interval per feature from the eligible fitting reference set.
Freeze the nonconstant feature dimensions from the declared fit
profile before opening the target. Require every frozen dimension
on both pages. Report excluded constant dimensions per fit.
Reject a target outside the supported feature interval or fewer
than three usable dimensions, 20 comparable winner references or
three observed reference incidents. For each compared feature,
divide the absolute same-unit difference by the frozen range.
Average these exact fractions with equal feature weight.
Winner Match uses the smallest exact mean distance to any eligible
reference. Calculate 100 times one minus that distance and round
half to even for display. Report the three nearest
eligible distinct families or fewer if disclosure floors require,
ordered by exact distance and then canonical reference identity.
Display unrounded distance bounds, counts, ranges and feature
drivers. The number is resemblance to observed own-site winners.
Its label and receipt describe resemblance only.

An observed drop flag is a distinct displayed 0137 hard-drop
receipt for the exact unchanged revision and incident. It is not
derived from a low Winner Match. No qualifying label or subsequent
edit gives the supported unknown or historical presentation.
Keep a prior receipt bound to its historical revision.

Each match, card, fit or simulation receipt binds request and plan
digest, current permission and site, target revision and exact
source evidence digests, incident/surface/projection, selected
denominator, supported features, method and profile, result with
uncertainty or hold, and current custody generations. Keep only
scoped references, numeric projections and provenance. Record
separate actual counts for pages, incidents, observed pairs and
simulated executions. A receipt records what the admitted method
observed or computed. Verify exact replay, type, scope and
destination before relying on it. Encode the typed record using the
existing canonical receipt format for the owning producer. Its
identity binds the canonical data digest and exact plan and effect
identity. Where signed proof is required, verify its current owning
purpose and approved signer under 0065. A view record, a plain proof
hash and independent signed run evidence remain different records.

## What-if and simulated alternatives

An owner drafts bounded typed alternatives to the immutable
current feature card. Preview exact local changes and unsupported
dependencies before a separate confirmation. Recompute only
features whose deterministic transformations are admitted for that
particular input. Scenarios are inert. They create neither a new
native revision nor a training label.

| Proposal | Permitted simulated transformation |
|---|---|
| Tighten the intro | Give a typed replacement opening or reorder an owner-declared answer block. Recalculate supported opening length, position and readable structure only. |
| Add a table | Give a local inert schema with header and non-empty typed cells. Evaluate the declared structural and coverage changes only. Empty or malformed structure stays unavailable. |
| Fill a fan-out gap | Add one or more explicitly named owner-approved intent items with a bounded local content block. Recalculate exact local checklist coverage. Unknown intents stay unresolved. |
| Add internal links | Supply an exact eligible same-site destination and native revision for each proposed link. Simulate only the local structural and link counts. No live graph, link rank, candidate proposal or target mutation follows. |

Apply only a declared transform whose changes are in observed joint support. The
proposed support check requires at least 20 comparable distinct fitting families
within one quarter of the frozen range of every changed numeric trait
simultaneously, with exact supported classes for categorical traits. It does not
combine different relatives to fill separate traits. Use the same frozen
reference pool and gate for the baseline and hypothetical inputs. Use the same
projection and frozen model or reference fit for both. Return exact changed
fields, local resemblance and model score differences where supported, support
counts, disagreement and known missing effects. Report hypothetical feature and
model comparisons only. Exceeding feature range, joint support, bound or version
yields “Outside the evidence we have” with no extrapolated result. Do not add a
direct apply action. An author can independently make an edit through the owning
native editorial workflow.

## Self-grading on subsequent observed outcomes

At advice confirmation freeze its receipt, current coach rule
and hypothesis, matched comparison specification, expected
direction, primary observable metric and eligible window. Record an
actual edit only from the platform's signed revision evidence and
an exact local adoption link approved by the owner. A suggestion,
preview, text similarity or human claim of use cannot make a
coached edit. Record discarded advice and missed implementation
separately, without treating them as failed outcomes.

After the next applicable settled official update on the same
surface, evaluate only a real applied revision with complete
authorised windows and no incompatible additional change. Do not
skip an inconvenient next update. If the approved edit is too late
for a complete pre-window, or controls, visibility, stable revision
or coverage fail, record ungradable with the reason. Require the
exact unchanged revision test throughout the comparison. The Lab
cannot guess what would have happened without that action.

In the grading record bind the site, surface, adoption receipt and
first applicable accepted incident's stable Google ID. Select from
0137's applicable timeline in begin-time and ID order. Keep that
binding when a newer feed omits the incident. Withhold the current
grade and new guidance from it. A later settled incident cannot
replace it. Resume evaluation of the same incident only after 0137
confirms its current revision, exact scope and settled phase. Apply
the existing complete-window and rights checks. If the confirmed
scope or evidence cannot qualify the comparison, record ungradable
with its reason. Keep a former grade as dated history only under
its current grant. The hold starts no new feed or connector request.

Match the coached units to prospectively frozen, eligible uncoached units in the
same owned site, scope and exact same event. Before seeing outcomes, exclude
exposure to the same guidance, disputed adoption and simultaneous prior
experiments. Preserve the exact pre-window match controls above and require
publish ages within 14 complete native days of one another. Sort eligible edges
by those declared controls and then canonical page keys, without reading either
outcome. Match each family at most once. Use the same primary metric, windows
and coverage bounds on both arms. For the proposed first grade, use final daily
mean clicks and compute each page's after mean less before mean. Report the
exact paired difference between coached and uncoached changes and the count and
median of complete paired differences. For an even count take the exact mean of
the two central values. Require at least 20 distinct usable pairs to give an
observed grade. Show exact windows, minimum and maximum paired difference,
missingness, conflicts, withdrawals and exclusions. The interval is observed
variation in the admitted sample. Do not call these observational controls
randomised. Prior approved 0094 experiments retain their own separate write and
assignment authority. Differences show observed association and possible
confounding, and may be affected by other site changes. Use no personal
demographics or inferred group attributes. Preserve the admitted
site, section and language, with no device-level grade.

A grading receipt appends to the immutable originating rule and study
generation. It cannot rewrite, activate or demote a rule or supply ranking, spam
or release inputs. A new coaching rule version follows the advisory decision
route.

## Learned advisory weights and comparisons

Keep the fixed paired-direction contrast above
as a primary interpretable baseline. A separate local comparison
can evaluate immutable weighted variants. Weights describe the
Lab's advisory comparison only. They can neither become ranking
weights nor be described as Google's preferences.

Before opening fit outcomes, declare the feature registry, six
ordered groups, exact projection and units, missing-value policy,
baseline, evaluation measure and stop bounds. Derive ranges from
fitting data and freeze them before any held-out read. Keep a single
complete comparable pair set per partition. A missing common feature
excludes the pair from every candidate and the baseline, with its
reason in coverage. The six groups
are answer and structure, entity and intent coverage, originality
and freshness, readable and usable form, media and tables, and
internal links and attribution. Display counts, support and signs
per feature even when a group contributes no evidence.

Use the existing frozen per-feature direction from the admissible
fitting set. For each compared feature, calculate the signed
normalised difference using the frozen nonzero fitting range and
its exact learned direction. Within each group take the exact mean
of those differences. Do not silently drop a missing active group
or clamp an unsupported value. The model score is the exact mean
of active group means weighted by the declared candidate weights.
An exact zero difference means a tie. Record it separately.
This score has no population probability meaning.

Compare the equal-group baseline against a bounded candidate
enumeration. Use the Cartesian product of integer weights
0, 1, 2 and 4 across the six groups. Exclude all-zero and canonical
duplicates after dividing nonzero weights by their greatest
common divisor. The weighted variant requires all six groups. Hold
when a group has no admitted comparable measure. In frozen group
order, enumerate the 3,367 distinct tuples. The owner
previews the exact comparator, candidate count, tie rules and
search bounds. Pick a candidate on the fitting incidents only
using pooled directional agreement on the fixed comparable
fitting pair set, counting exact score ties as half an agreement.
Break score ties by fewer active groups, then the canonical weight
tuple. Keep the omitted counts on that fixed common comparison set.
A candidate score uses the sum of group weights as its denominator.

Freeze the single chosen tuple, baseline and all fit receipts before opening
held-out evidence. Require at least 20 non-tied pairs and the declared control
independently on each held-out incident. Report agreement over the fixed
complete pair set with ties worth one half. Require at least three fifths
agreement and no less than the frozen baseline on each incident before marking
the weighted candidate supported. Report exact denominators, feature weights,
fitting and held-out differences, trial and phase counts, tied or missing
results and the admitted pre-trend control. Keep unsupported, worse and tied
variants as such. The holdout cannot select another model or threshold. If the
model and evidence assumptions cannot describe an alternative, keep its
comparison unavailable.

## Batched local pair and scenario work

Use 0019 registration for exact immutable job class, per-site
intent, job checkpoint and fair turn. Work units are bounded local
comparisons of frozen page pairs, admitted weight vectors or
admitted what-if variants. A pair in copied observations counts
once in sample support. Repeated simulation is computational work,
without new observation or feature credit.

The standard profile uses the baseline study's small-job caps.
A separate owner-approved batch has a maximum of
4,194,304 comparator or scenario evaluations per frozen plan,
at most 65,536 per checkpoint, 60 active minutes and 24 elapsed
hours. A fit and the later holdout remain distinct phases. The
enumerator uses a canonical input ordering and a computed upper
bound from exact pair, model and variant counts. A scenario batch
can contain at most 256 approved target revisions, 64 distinct
typed alternatives per target and 256 preselected eligible fitting
references for each comparison. Choose reference keys in the
frozen canonical sample order, before computing a score. Their
product is the maximum scenario comparison count, within the common
cap that also includes any weight comparisons. Preview and reject
a plan beyond its bound before claiming resources. No padding,
replay or repeated random draw earns more evidence. On an eligible
larger site, a measured admitted profile may reach millions of
comparisons across variants. An ordinary smaller dataset can stop
with much less work and retains its actual support.

Require exact 0078 combined budget and memory/CPU measurements.
Admit the small interactive profile from the 8 GiB baseline.
Admit the low-priority batch from the 16 GiB baseline with at most
1 GiB worker allocation and 512 MiB protected scratch,
subject to the stricter measured headroom. Any workstation tier
needs its own versioned resource profile. An 8 GiB profile or a
failed current reading cannot borrow a larger cap. High available
memory never bypasses job, retention, evidence or total system
limits. Checkpoint and yield on current memory pressure and live
work under 0078, with no automatic restart under a changed grant.

Use current approved finalisation events when the owner has confirmed a
processing policy for that exact site, profile and revision. The Lab asks no
upstream provider for refresh. A native save event keys a card by site, page,
revision pair, event kind and diff version. A tracker event keys settled
evidence by provider incident ID, revision and accepted source generation. Each
event may enqueue one card for its exact scope. An old native event is
historical or held when the current revision has moved. A crawler observation
that spans a boundary remains an interval. No card trigger starts a study or
assigns an outcome label. Confirmation of one study gives no recurring study or
altered settings grant. Pause fences further claims, commits and report
publication. An already published authorised report remains readable on an
ordinary pause. Source expiry, revocation or erasure withdraws existing reads,
cursors and dependent active rules before cleanup. Resume checks exact current
consent, inputs, clocks and effects. Persist the exact enumerator cursor, plan,
source digests, visited prefix and counters before yielding. Resume remaining
work with the same inputs. Count checkpointed work once and retain a
conservative reservation for uncertain work until reconciliation. No rank or
site operation shares its effects.

## Version the coaching rules

A candidate rule contains a bounded typed advisory predicate and
wording, its versioned source model, admitted feature and scope
manifest, hypothesis and validation contract. Store an immutable
revision and parent. A new draft starts inactive. A model result or
a self-grade alone cannot change the active advisory rule.

Use a reviewable succession of draft, shadow candidate, reviewed,
active, paused, retired or superseded. The Lab shows the exact
before and proposed version, support, stale conditions, scoped
effect on local guidance and independent review receipt.
For promotion require the admitted held-out and control proof,
and the version's governing comparison with the previous rule.
Where prospective adoption outcomes form part of the proposed
rule's qualification, require the predeclared self-grading test
and observed comparison or hold. No missing outcome is a pass.
An explicit owner decision can pause or retire an advisory rule
without granting authority to activate a replacement.

The owner confirms an exact eligible promotion, deactivation or rollback shown
locally under 0020. Check current version, scope, model evidence, policy and
erasure generation. Update the active pointer and append the transition receipt
atomically. A stale confirmation, concurrent replacement, revoked rule evidence
or unqualified predecessor holds. Roll back only to a still-admitted previous
advisory revision. Rule transitions occur only at that owner effect boundary.
Historical grades are immutable and the product champion controller has no Lab
caller. The active pointer governs only the Lab's local coaching advice.

Before showing current guidance, recheck the exact fit and receipt
dependencies, including on cached reads. If a correction to the
rule's cohort, source projection or incident invalidates its evidence,
or current permission or the admitted profile no longer qualifies,
hold the guidance and dependent output even when the pointer and
draft revision still match. Apply the same eligibility check at a
rollback commit. The hold leaves the active pointer in place. Apply
existing authority and erasure rules to grade history.

## Alekh, approvals and local MCP

Alekh is the current name for QuarryView. Embed a typed,
site-scoped read projection under 0043 and the Lab's explicit
permissions. Use the same approved dates, dimensions, units,
coverage threshold, filters and immutable result reference in
charts, tables and accessible text. Show open or settled intervals,
source identity, measured and simulated values separately, match
support, known confounds and current policy refusal reasons.
Recheck current site, owner grant, view generation and erasure on
open and every page or cursor read. Use no dynamic query,
arbitrary Python filter, provider call, third-party tile, external
export or public widget. Alekh retains its own ordinary charts.

Register local MCP tools to prepare a study, what-if or pause
request and read typed run state or approved summaries, each
under its own exact grant. Use 0020 and 0118
enrolment on the protected local channel and one exact site.
No raw contents, credentials, query terms, audit logs or
protected receipts are return values. Protected output over
unenrolled ordinary standard input and output is refused.

MCP can present a pending request and follow its eligible status.
A separate site-scoped read grant permits bounded weight diffs and
self-grading summaries from the current verified generation.
Return only incident and variant identities, supported fit or
grading window, group weight changes, observed comparison
denominators, approved aggregate metrics and safe exclusion
reasons. Apply grant, generation, erasure and approved minimum
cell checks at every read. Hold the summary if the disclosure
profile lacks a minimum cell or overlap rule. Suppress individual
records, small cells and overlapping cuts. A 0137 grant cannot open the Lab
summary. Tracker timelines and winner/drop boards keep their
existing 0137 tools.
MCP cannot supply live approval, activate a rule, request a
provider, dispatch or defeat a pause. The owner sees and confirms
the exact typed plan on the existing trusted local surface.
The owner-only Pause acts immediately on the admitted scheduler
fence. An MCP pause request reaches only that existing owner route
for approval and cannot impersonate its confirmation. A separate
approved typed fail-closed policy may hold on revocation or risk.
No durable pause or cancelled task can be resumed by a stale
token or replayed approval.

## Storage, erasure and platform evidence

Register manifests, jobs, typed snapshots, drafts, receipts and
cache generations per site in the 0030 ledger. Store no raw
page body, query or credential. Keep the default local under
0118 sealed custody. An independently approved 0038 move to a
managed encrypted external SSD is allowed only after the exact
volume, rights, scope and read-back are verified. An approved
private S3 profile admits sealed archive bytes only if 0038
and 0118 admit that exact Lab data class, owner recipient and
managed version and erasure route. Nothing in the Lab can call
an S3 endpoint, decrypt a copy for an operator, provision a
bucket or export a free-form file. Use the separate owner-approved
storage route with exact destination/read-back and no raw egress.
Without that approval the archive stays local.

Track every managed copy, replica and version and current erasure
generation in 0032/0118. Stop reads, writes, deliveries and worker
results before declaring scope withdrawn. Delete or reconcile each
managed target and its admitted key path; offline drive or
unreachable old object versions remain pending until proved
absent. Restore applies erasure before making bytes or views
available. No completed deletion claim follows from revocation
alone. Record a safe erasure receipt with precise dispositions.

Use exact x86-64 Ubuntu 26.04 LTS and Debian 13 detached .deb
profiles and the Windows 11 installer profile with its independent
pre-launch trust path and signed underlying files under 0030.
Use the same Rust implementation and pinned Bazel graph only
after each target is admitted. Pin the Tauri/Angular local view,
per-platform protected transport and supported own XenForo or
WordPress native fixture. Bind manifest bytes, capabilities,
clock, model, profile and exact tests. Linux execution cannot
qualify the Windows profile. Missing package, hook, storage or
worker protection remains a visible unsupported case.

## Additional acceptance anchors

| Requirement and case | Required result |
|---|---|
| `REQ-0138-CARDS`, `IT-0138-CARD-BOUNDARY` | Publish and major edit, ambiguous crawl observation, proof of unchanged cohort, separate supported surfaces and stale cursor. No provisional or cross-surface result, stale fit or native effect. |
| `REQ-0138-MATCH-RECEIPT`, `UT-0138-WINNER-MATCH` | Assert exact distance, supported range, actual reference denominators, tie, cross-site refusal, unknown feature and stale drop flag. Verify exact receipt replay and scope. |
| `REQ-0138-SCENARIOS`, `IT-0138-WHATIF-QUOTA` | Exercise each typed alternative, combined scenario, cap equality/overflow, unsupported joint shape, quota pause/resume and effect isolation. Compare observed and computed numbers separately. |
| `REQ-0138-COACH-GRADE`, `BDD-0138-GRADE-NEXT` | Match a signed applied edit and frozen uncoached cohort through the next settled event. Refuse missing adoption or complete window, confounded revision and withdrawal. Check the versioned rule decision separately. |
| `REQ-0138-RULE-DECISION`, `IT-0138-RULE-REPLAY` | Race pause and owner promotion, reject stale version, conflict and invalid evidence, then lose the reply. Exactly one scoped receipt or hold, with no promotion outside the Lab. |
| `REQ-0138-VIEW-SCOPE`, `IT-0138-ALEKH-MCP` | Local keyboard and typed view parity, wrong site, revoked token, held-out reference, forbidden confirmation, stale cursor and blocked egress. No protected recovery through fallback. |
| `REQ-0138-ARCHIVE-PARITY`, `E2E-0138-STORE-PARITY` | Repeat bounded local, external and independently approved archive cases, offline erasure and stale restore on each admitted platform. Verify matching deterministic receipts and protection. |

Extend the Quint state model with two advisory versions, two
selection scopes, one study batch, a lost reply, an owner pause,
one native adoption and an erased restored copy. Add invariants
for snapshot cutoff, one coach decision, no MCP confirmation, no
active result after revocation, no cross-surface pairing and no
live ranking or site writer. Use exact integer arithmetic and
bounded checks under the existing formal toolchain, including
overflow and deterministic selection. Replay each counterexample
class and the admitted paths through the Rust job, ledger and
owner view on the exact pinned Linux and Windows Bazel profiles.
Independent review checks actual evidence and consumer isolation.

## Source projection and phase access

`API-0138-SOURCE` uses two typed projections. The pre-window
inventory response contains authorised page-family and revision
references, pre-window facts and source manifest. The outcome
response contains only the exact settled 0137 labels, proof of
comparable page-day support and receipt references for the
requested sealed sample. The scheduler grants outcome access to
the exact site, property, variant, study, phase and selected
keys only after its prerequisite seal. Fitting has no held-out
projection. Ordinary query rows and 0137 chart display cannot
supply missing projection proof. Hold if the authoritative
producer or coverage is unavailable. No connector work follows.

## Stable actions, messages and records

All 0138 requests use an active owner site binding, interpreted
by 0020 from authenticated state. No request grants its own site,
role, permit, signer or storage destination. Opaque references
are versioned and site-bound. Reject unknown fields, changed
plan bytes, stale revision, excessive length and wrong scope
before protected access.

| Contract | Typed request → bounded response |
|---|---|
| `API-0138-PLAN`, `update_lab.preview` | Owned property reference, variant, inclusive date span and allowed local operation; the preview verifies selection metadata only → plan reference, exact digest, proposed effects, required evidence, expiry and allowed approval path. |
| `API-0138-RUN`, `update_lab.run` | Plan reference, expected digest and idempotency key → current run reference and safe status. Consume the exact live owner approval from the owning 0020 verifier in the plan transition. No caller-supplied confirmation field. |
| `API-0138-READ`, `update_lab.read` | Run reference, typed view, expected view generation, opaque cursor and page limit (integer 1–100, default 50) → a typed page from one immutable authorised result. Scope and erasure are checked at open and each page. |
| `API-0138-PREPARE` | Local MCP `update_lab.prepare_study`, `update_lab.prepare_what_if` and `update_lab.prepare_pause` accept exact typed draft intent or a run reference → pending owner-review reference, visible reason and expiry. `update_lab.read_status` takes a pending request reference and returns current stage, expiry and safe reason under its own grant. No dispatch, labels, protected receipts or approval path through MCP. |
| `API-0138-MCP-SUMMARY` | `update_lab.read_summary` takes run reference, summary kind `weight-diff` or `self-grade`, opaque cursor and page limit 1–100, default 50 → typed covered counts and permitted measures with next cursor. Independent site grant required. |
| `UI-0138-OWNER` | Prepare, review, run, pause, read evidence and manage advisory rules through the trusted owner UI. Keep current calls read-only until their separately permitted effect. Display observed, simulated, held, stale and ungradable results distinctly with accessible table parity. |
| `DB-0138-RECORDS`, `EVT-0138-LOCAL-OUTCOME` | Bind site and RLS, immutable schema and manifest digests, plan and phase, current policy, approval and erasure generations and the exact effect identity. Commit the local projection pointer, result receipt and one safe event together. The event carries only scoped reference, stage and reason. No new recipient or destination. |

Use the existing 0020 approval lifetime for applicable proposals.
Automatically derived CLI or MCP discovery refuses the owner-only
desktop actions. Only the separately registered MCP tools are visible.
An expired or used approval cannot reserve a new run. The desktop
may show an eligible existing outcome after a lost reply only
when current scope still permits it. A changed request with the
same key holds before effect. Bind each cursor to site, owner
grants, plan, typed view, filters, evidence and erasure generations.
Never widen a cursor to fill an empty view. Tie each interface, UI path,
store and event to the existing REQ-0138 rows and their tests.

## Data

Use schema major 1. Site, page, revision, run, property and
incident references are opaque UTF-8 values of at most 128 bytes
with exact owning-source equality. Use native ISO dates for source
days and UTC seconds for study events. A digest holds 32 raw SHA-256
bytes. Site authority comes from authenticated context. Counters are
unsigned 64-bit values within the profile bounds. A fraction has a
signed numerator and positive denominator within the checked bound.
Null carries one typed unavailable reason. Unknown fields, version,
out-of-bounds values and absent required fields refuse before a
source read. A version change needs a fresh plan digest.

| Record | Fields required |
|---|---|
| `study_plan` | Plan ID, site, property, variant, operation, incident revisions, each native interval and zone revision, source and registry digests, approval and expiry context, input and plan digests |
| `feature_card` | Site, family, page and exact revision, observation and acceptance UTC, native trigger or observed interval, field registry digest, up to 64 typed measures with unit and coverage, version and source digest |
| `pair_fit` | Study ID, exact variant, partition, incident revision, frozen ordered pair identity list digest, feature and controls digest, direction or hold per declared trait and schema version |
| `study_result` | Study and exact generation, state, current scope proof, plan, input, fit and report digests, coverage, observed and simulated counts separately, method, evaluations and local evidence references |
| `coach_transition` | Site, rule and prior/current version, verb, current protected approval, evidence and erasure generations, effect identity and checked result |
| `ledger_ref` | Site, project, artefact and store identity, byte length, digest, current key purpose reference, retention cutoff, replica and erasure generation |

The immutable result identity is unique within site and plan effect.
Keep rows site-scoped under forced RLS. Commit the protected local
projection, one receipt and one safe outbox event together. Cancelled
or stale work cannot release a partial result. Pause fences new
effects. A revoked source makes dependent results and rules
unreadable and closes their open cursors.

## Failures and recovery

| Stable error and code | Trigger and result | Owner message and recovery |
|---|---|---|
| `ERR-0138-AUTHORITY`, SUT-4500 | Wrong or expired grant, unapproved plan, locked policy or scope mismatch. Refuse before opening evidence. | “This study is unavailable.” Review the site's current permission and prepare a fresh plan where allowed. |
| `ERR-0138-COVERAGE`, SUT-4501 | Unproved, missing or incomplete source rows, cohort or features. Hold with safe counts only. | “More comparable data is needed.” Show the exact missing date, source grain or field when authorised. |
| `ERR-0138-PROFILE`, SUT-4502 | Invalid input, unadmitted variant, bound exceeded or missing resource profile. Refuse the affected work. | “This study profile is unavailable.” Show the failed check and allow a fresh approved plan within its supported profile. |
| `ERR-0138-CONTROL`, SUT-4503 | Complete comparable inputs supplied but measured support, holdout or control fails. Hold the finding. | “The comparison is inconclusive.” Show the measured support and failed control. |
| `ERR-0138-RECONCILE`, SUT-4504 | A durable transition may have taken effect and its current result is missing. No second effect. | “The last result needs to be checked.” Read or reconcile the original operation under current authority. |

Return the common 0020 safe error envelope and current run stage where
authorised. It includes no foreign site identity, credential, query
or raw draft. A worker time or memory stop uses the scheduler hold,
or SUT-4502 for an unadmitted profile. Erased results remain fenced.

## Owner screens

### Owner interface and action states

Use **Empirical Update Lab** for spec 0138, **Google Search Update Tracker**
for spec 0137, and **Alekh** for charts and tables. QuarryView is the former
name. **Editorial Coach** (spec 0041; "Editor Coach" in quoted decisions) is
the local advisory surface. Follow ADR-0024 and DR-119/124. Setup, approvals
and evidence views are for the current authorised site owner.

### Entry from the timeline

In the 0137 timeline, open **Plan a study from this update** from an eligible
incident. Bind site, incident revision, surface and source
generation. The link opens the local preview. The preview lists the exact
five compatible incidents for a baseline study or the reason it
cannot select them. Use no live outcome to expand that selection.
Return through **Open update timeline** with the same authorised
scope. On permission loss clear the linked evidence and show only
the safe refusal. When only the verified rollout dates change,
show **Update dates changed** and offer **Review current dates**.
When only the verified affected scope changes, show
**Update coverage changed**. If both change, show
**Update dates and coverage changed**. For either coverage message,
use **Open update timeline** to show the current authorised incident
details. Preserve the linked old receipt only where current policy
permits historical access.

| Tracker state | Lab text and action |
|---|---|
| Official rollout still open | **Update underway.** Show the official start. Study action is unavailable. |
| Official end confirmed, wait underway | **Waiting seven full days after the update.** Show official end and earliest comparison time, with time zone. Study action is unavailable. |
| Date or feed uncertain | **Waiting for Google’s status feed.** Show last verified confirmation if allowed. Keep the study held. |
| Wait complete, final compatible days missing | **Waiting for final Search Console data.** Show measured days and required days for that exact surface. Study action is unavailable. The wait alone confers no evidence. |
| Confirmed scope narrower than the eligible chart series | **This chart covers more search traffic than the update. The study is on hold.** Keep the official dates and authorised broader chart visible under their own scope. Keep the comparison and study unavailable until exact matching coverage is proved. |
| Settling complete, coverage profile unproved | **Page coverage could not be verified.** Show the authorised source and affected window. Keep the study action unavailable. |
| Final eligible comparison available | **Review study plan.** Show search surface, windows, source and available cohort before any approval. |
| Page published or edited during rollout | **Published during rollout** or **Changed during rollout** as the native facts allow. Keep the revision marker and feature card. Withhold an unchanged-page result. |

Show source day basis beside each date range. Use the existing tracker
clock and settling phases. A local clock, stale link or historical
window cannot open an early comparison. Render keyboard links,
status text and table equivalents through SDS. Use text and focus
as well as colour. Buttons held by a prerequisite show a plain
reason beside the control. An unauthorised user sees a safe
unavailable state and no private labels, counts or receipts.

### Review, approve and follow a study

**Review study plan** displays the owned site and verified property,
surface and incident revisions, closed dates, type of comparison,
included data and controls, scope exclusions, resource and time
limits, retention, and exact proposed effect. Separate normal local
storage from any independently approved protected archive. The
owner selects **Approve this study** or **Cancel** on the trusted
local surface. Protect the exact plan digest. A changed plan,
expired approval, changed site or lost authority requires fresh
review with **Review the new plan**. Cancel creates no run.

After confirmation use **Queued**, **Checking evidence**,
**Comparing pages**, **Checking the comparison**, **Ready to review**,
**Paused**, or **Study unavailable**, according to verified state.
Show actual examined, included and held counts when known. Show no
estimated finish. **Pause study** fences new work and result
publication. Show **Pausing** while an in-flight effect is
unreconciled. **Review resume** requires current evidence and
owner authority before work can continue.
**Checking the last result** is the uncertain reply state.
Keep another start unavailable until the exact run is reconciled.

### Review evidence, resemblance and alternatives

- **Feature card** shows the exact revision, whether publication
  or a major edit triggered it, source, time, coverage and typed
  measures. **Compare revisions** separates the two dated snapshots
  and any eligible settled search changes. **Not measured**,
  **Not enough comparable data** and a measured zero are distinct.
- **Winner Match** shows **Similarity to observed winners**, the
  supported resemblance score, actual comparable pages and
  incidents, and what was compared. **Not enough comparable winners**
  replaces an unsupported score. **Observed drop** is a separate
  settled receipt for the exact unchanged revision. A later revision
  shows older evidence as **Earlier revision**, only if still allowed.
- **View evidence** opens the current scoped receipt. Show study,
  windows, source and revision, figures and units, match coverage,
  controls, uncertainty and owning evidence references. Mark
  **Evidence withdrawn** or **This result needs checking** when
  appropriate. No clipboard, export or public share is enabled.
- **What-if** starts with **Review simulated change**. Show the
  exact typed proposal and its baseline. Offer **Run simulation**
  only after exact owner confirmation. Label results **Observed
  evidence** and **Simulated comparison**. Show changed fields and
  support. Outside the model's scope use **Outside the evidence we
  have** and withhold the score. No apply or publish control.
- **Grading** shows **Edit not yet verified** until signed revision
  and recorded adoption match. **Waiting for the next settled update**
  retains the exact first applicable event when known. If that event
  is missing from the current feed, show **Waiting for Google’s
  status feed** with its last eligible confirmation. Show no grade
  from a later event.
  **Observed comparison with uncoached pages** shows separate changes,
  complete matched pair count, exclusions and uncertainty.
  **Cannot grade this edit** states the actual failed prerequisite.
  Looking at a grade cannot change advice or the site.
- **Coach rules** shows each version, active or paused state and
  evidence. **Activate this guidance**, **Pause guidance** and
  **Retire guidance** preview the exact decision and require
  applicable owner confirmation. Any **Restore earlier guidance**
  is restricted to a still-admitted advisory version. Hold stale
  approval, missing review, expired evidence or conflicting change.
  A rule has no automatic promotion or native edit control.

### Local MCP

Use the approved enrolled site-scoped preparation, status and
separately granted typed summary tools. Return **Ready for owner
review**, **Waiting for owner approval**,
**Paused**, or the safe current stage and reason with an opaque
request reference. **Open owner review** uses an existing protected
local hand-off if allowed. Requests can prepare a study, simulation
or pause for review. They cannot confirm, execute, resume or expose
labels, protected receipts or raw source data. Safe refusals and
expiry remain visible. No protected fallback through plain stdio.

Exercise the 0137 deadline boundary, final-day gap, focus on return,
changed revision, expired approval and wrong site in the owner fixture.
Pair with the UT-0137-BOARD scope correction case and
IT-0138-SEALED-SOURCE. Narrow the verified scope without changing
the official dates. The owner sees **Update coverage changed**,
the unchanged dates and the distinct broader-chart hold. No eligible
label or study opens from the broader pane. In separate branches
change only the official dates or change both dates and scope. Show
**Update dates changed** or **Update dates and coverage changed** as
applicable. A stale link cannot reopen an older study, and losing
access still clears the linked evidence and gives only a safe refusal.
Verify that a simulated change, an ungraded edit and a withdrawn receipt
produce their own states. Repeat pause and MCP expiry without a second
effect or an owner confirmation from a tool.

## Independent acceptance additions

| Case | Observable result |
|---|---|
| `IT-0138-PROFILE-DESCRIPTOR` | Web valid, Discover click-only valid, missing Web position, cross-profile mixing and five-event support shortage. Exact label, sort and hold for each. |
| `IT-0138-SEALED-SOURCE` | Wrong property or page grain, final/capped or exhaustively paginated incomplete API output, unknown response aggregation, held-out read during fit, changed plan and revoked peer. Refuse protected outcomes without an upstream pull. At the source boundary present a complete-looking synthetic bulk-export file for the right property and both 28-day windows. Even with exact study approval, hold with SUT-4501 when the producer is unadmitted. Expose no label and start no import or pull. An explicitly synthetic complete projection can prove admission mechanics only. |
| `UT-0138-COMMON-DENOMINATOR` | Miss a feature used by any fitted variant, tie distances and hide a reference; verify exact frozen support, denominator, nearest score and abstention. |
| `FI-0138-BATCH-CHECKPOINT` | Interrupt at the cap, before and after checkpoint, then lose the reply and lower headroom. No duplicate visible result, budget reset or profile borrowing. |
| `IT-0138-RESULT-BOUNDARY` | Race local result commit, cursor read and rule pointer activation with pause, correction and erasure. Check owner UI and scoped MCP outputs separately. In a synthetic case leave the current draft and active rule pointer unchanged, then correct an incident used by the rule. Hold its cached guidance and affected grade summary. Refuse a pending rollback to a predecessor using that invalid evidence. Keep transition and grade history immutable. No stale content or effect. |
| `BDD-0138-GRADE-FIRST-INCIDENT` | Unknown first applicable settled event followed by a favourable event. The first stays ungradable and the second cannot substitute or promote the rule. In a separate branch omit the recorded first accepted applicable incident from a newer valid JSON projection that still includes the later settled incident. Show “Waiting for Google’s status feed” and the earlier permitted confirmation. Hold the original grade and rule decision. Revalidate the exact first incident through 0137. Accept only its eligible settled evidence, or show “Cannot grade this edit” when its compatible comparison cannot be proved. Start no upstream pull. |

## Settings and limits

Use the existing 0020 site grants and exact-plan approvals, 0019
job limits and 0078 resource profiles. This spec adds no independent
settings default or permission to relax a bound. No study runs until
the owner confirms its exact plan. Study limits, expiry and erasure
apply to the declared site, input and worker version.

| Profile | Existing design limits |
|---|---|
| Baseline study | Five incidents. Two 28-day windows per incident. At most 50,000 examined page rows overall, 256 sampled pages and 16,384 eligible pair edges per event. At most 128 MiB resident and 64 MiB protected temporary bytes. Stop at 15 active minutes or 24 elapsed hours. |
| Owner-approved batch | At most 4,194,304 work units and 65,536 per checkpoint. Stop at 60 active minutes or 24 elapsed hours. At most 1 GiB worker allocation and 512 MiB protected scratch on the admitted 16 GiB profile. The 0078 combined cap may be stricter. |
| Protected read and custody | Pages return at most 100 rows. Derived-study retention ends at 180 days from commit or earlier source expiry, withdrawal or owning retention limit. |

## Public-page footprint

Budget zero added queries, zero added time and zero added bytes per
public request on both native platforms. The Lab adds no public path
handler, tag, beacon, external fetch or synchronous task. It consumes
already admitted event and content projections after their owning
operation. Compare the public request execution paths, query counts
and response bytes with the Lab absent, held and enabled. Prove no
Lab work on the measured path, without assuming that two wall-clock
samples are identical.

## Security, privacy and availability

- No real outcome intake without a current admitted producer for the
  exact own-property, settled, final and complete page-day projection.
  The existing 0040 projections hold with SUT-4501. Final status,
  completed pagination and synthetic coverage do not establish real
  completeness. No bulk export, importer, grant or provider pull follows.
- An exact own-site feature card may show an already-held admitted
  observation with its source time and coverage. Without an admitted
  comparable fit, keep Winner Match, weighted findings, simulated
  scores and grades unavailable. An independently supported tracker
  date or observation remains a separate fact. An unproved field is
  unavailable rather than a numeric zero. No analytics proxy applies.
- Enforce current site, phase, approval and erasure authority. Local
  MCP can prepare permitted requests and return approved typed
  summaries only under its independent protected grant and disclosure
  profile. Otherwise return the safe hold. Keep local sealed custody
  by default. A managed external SSD or private S3 archive requires
  its exact separate 0038/0118 approval, read-back and erasure route.
  The Lab itself initiates no external delivery and exposes no raw
  source, credentials, sealed labels or protected receipts over MCP.

## How it is tested

| Boundary | Existing cases and exact emphasis |
|---|---|
| Authority and input | IT-0138-ADMISSION, IT-0138-SEALED-SOURCE and BDD-0138-OWNER-PLAN cover absent, stale and wrong-site authority, real-source grain and coverage refusal, and the one-use plan. A synthetic complete projection can prove the admission logic only. |
| Temporal, trait and matching | UT-0138-WINDOWS, UT-0138-FEATURES, UT-0138-MATCH and IT-0138-PROFILE-DESCRIPTOR cover empty and one-row cases, settling equality, one late or missing day, exact cutoff and one tick late, 256 and 257 candidates with fixed first-256 selection, 10,000 and 10,001 enumerated candidates, and refusal of a malformed extra edge beyond 16,384, wrong subtype and absent expected position. Assert frozen sample identity and no label access before sealing. |
| Findings and effect isolation | UT-0138-CONTRAST, UT-0138-WINNER-MATCH, IT-0138-WHATIF-QUOTA and FI-0138-BATCH-CHECKPOINT cover constant traits, ties, zero versus missing, cap equality and overflow, lost checkpoint reply, unsupported alternative and separate observed and simulated counts. |
| Permissions, pause, erasure and footprint | IT-0138-ALEKH-MCP, IT-0138-RESULT-BOUNDARY, E2E-0138-WITHDRAW and E2E-0138-STORE-PARITY exercise wrong site, stale cursor, absent disclosure or storage profile, pause, offline erasure and old restore. Include the public-path comparison above on the exact XenForo and WordPress fixtures. |

PRP-0138-STATE and the existing Quint Connect replay bind exact model
and source revisions and exercise the stated invariants on the admitted
Ubuntu, Debian and Windows Bazel profiles. Store fixture and signed
result identities separately. These sections specify proof obligations.

## Changes from the source

Owner requests in DR-119/124, ADR-0024 and the linked 0137 feed and
Search Console references establish the scope and source facts. The
feature registry, deterministic sample, pair comparison, Winner Match,
scenario and grading rules are original Sutradhar designs with frozen
inputs and declared limits. They describe association or similarity in
the admitted sample. They assert no Google ranking mechanism or causal
effect. Use the canonical framing and signing contracts in 0064/0065
with their own versions and conformance evidence. Review the exact
source, intended use and independent evidence under 0076.
