# 0097: Brand monitoring, assets and surveys

Keep a site brand kit, review what supplied evidence says about the
brand, and plan surveys that people can answer through their site.
DR-088 limits excluded services to local checks, supplied evidence
and manual drafts. There is no outside polling or account connection.
Unsplash, Pexels and Wikimedia Commons connectors are not built
(spec 0100). Use owner-supplied assets with current rights evidence.

Spec 0095 owns review imports and protected rating aggregates. Spec
0096 uses approved assets in distribution packs, spec 0098 owns
sponsor records, and spec 0094 owns report exports. Site accounts
and permissions come from specs 0020 and 0101. Spec 0093 separately
measures mentions in supplied AI answers.

Fictional example. A forum imports a dated mention file. Staff flag
a complaint for review but send any reply themselves. A survey asks
members which guide to improve. The owner sees protected group
results and chooses a licensed image for the next guide.

## Words used here

- **Mention** means an observed reference in a named source.
- **Brand kit** means approved versions of brand assets and rules.
- **Use grant** means documented rights for one asset and purpose.
- **Survey wave** means one version and time window of a survey.
- **NPS** means Net Promoter Score, a measure of recommendation.

## What this spec designs

| Feature | Version 1 treatment | Requirement |
|---|---|---|
| 384 Mention monitoring on free sources | Supplied mention snapshots | REQ-0097-SOURCES |
| 385 News monitoring through GDELT | Supplied dated news records | REQ-0097-SOURCES |
| 386 Google Alerts feeds | Supplied normalised alert records | REQ-0097-SOURCES |
| 387 Hacker News monitoring | Supplied dated records | REQ-0097-SOURCES |
| 388 Podcast mentions | Supplied, authorised evidence and timestamps | REQ-0097-SOURCES |
| 389 Wikipedia edit monitoring | Compare supplied dated revision evidence | REQ-0097-SOURCES |
| 390 Reddit monitoring | Supplied dated records | REQ-0097-SOURCES |
| 391 YouTube mentions | Supplied dated records | REQ-0097-SOURCES |
| 392 Mention sentiment | Evidence-backed local tone labels | REQ-0097-LISTEN |
| 393 Mention spike alerts | Local finding when admitted evidence is analysed | REQ-0097-LISTEN |
| 394 Influencer identification | Public author evidence for manual review | REQ-0097-REVIEW |
| 395 Topic and hashtag tracking | Match named terms in the admitted evidence | REQ-0097-LISTEN |
| 396 Share of voice in mentions | Shares in one comparable sample | REQ-0097-LISTEN |
| 397 Review rating over time | Protected periods per review provider | REQ-0097-RATINGS |
| 398 Crisis mode | Local review checklist and hand-off | REQ-0097-REVIEW |
| 399 Sponsor and partner mentions in forum | Guest-visible mentions of named partners | REQ-0097-LISTEN |
| 400 Boolean query builder | Bounded AND, OR and NOT filters | REQ-0097-LISTEN |
| 401 Exclusion filters | Display filtered and eligible counts | REQ-0097-LISTEN |
| 402 Mention tagging and assignment | Review states and staff assignment | REQ-0097-REVIEW |
| 403 Reply suggestions | Local, sourced draft | REQ-0097-REVIEW |
| 404 Multilingual mentions | After version 1 under DR-081 | REQ-0097-SCOPE |
| 405 Mention reports and exports | Protected local report and safe export | REQ-0097-REPORT |
| 406 Helpful-reply opportunities | Draft advice with relevant site evidence | REQ-0097-REVIEW |
| 407 Brand hub | Versioned assets and usage rules | REQ-0097-KIT |
| 408 Digital asset library | Local, searchable versioned assets | REQ-0097-KIT |
| 409 Asset usage tracking | Observed use in indexed pages and local packs | REQ-0097-KIT |
| 410 Brand guideline documents | Export approved tokens and rules | REQ-0097-KIT |
| 411 Off-brand colours and fonts | Compare supported observed tokens | REQ-0097-KIT |
| 412 Brand voice guide | Approved examples and voice rules | REQ-0097-KIT |
| 413 Branded social image templates | Approved layout and licensed assets | REQ-0097-ASSETS |
| 414 Asset request and approval workflow | Local request, approval and expiry | REQ-0097-ASSETS |
| 415 Expiring licensed asset alerts | Local notice with licence evidence | REQ-0097-ASSETS |
| 416 Image rights registry | Documented permitted use and expiry | REQ-0097-ASSETS |
| 417 Brand kit export | Safe local export of approved material | REQ-0097-ASSETS |
| 418 Duplicate asset detection | Exact hash match and sourced similarity proposal | REQ-0097-KIT |
| 419 Format and size conversions | Bounded, licensed local conversion | REQ-0097-ASSETS |
| 420 Outdated logos in content | Report observed superseded versions | REQ-0097-KIT |
| 421 Sponsor co-branding rules | Require both brands' approved rules | REQ-0097-ASSETS |
| 422 Font licence tracker | Separate display, embedding and redistribution rights | REQ-0097-ASSETS |
| 423 Member survey pages | Admitted signed-in site survey route | REQ-0097-SURVEY |
| 424 Survey distribution | Existing approved notices and manual links | REQ-0097-SURVEY |
| 425 Survey analytics | Protected cross-tabs and uncertainty | REQ-0097-ANALYSE |
| 426 Open-text answer themes | Scrubbed, protected local themes | REQ-0097-ANALYSE |
| 427 Satisfaction and NPS tracking | Comparable measures by wave | REQ-0097-ANALYSE |
| 428 Concept tests | Approved and fixed survey variants | REQ-0097-TEST |
| 429 Survey-based segments | Admitted aggregate groups | REQ-0097-ANALYSE |
| 430 Survey templates | Versioned questions and consent wording | REQ-0097-SURVEY |
| 431 Response quotas and weighting | Predeclared caps and weights | REQ-0097-TEST |
| 432 Survey results in reports | Shared protected results | REQ-0097-REPORT |
| 433 XenForo poll analytics | Admitted XenForo aggregate poll evidence | REQ-0097-ANALYSE |
| 434 Question wording tests | Fixed assignment across approved versions | REQ-0097-TEST |
| 435 Long-running tracker surveys | Comparable versions and periods | REQ-0097-TEST |
| 670 Audience personas from forum data | Fictional roles from protected aggregates | REQ-0097-W2-AUDIENCE-SKETCH |
| 674 Mention geography | Supplied coarse aggregate geography | REQ-0097-LISTEN |
| 682 Content similarity map | Local index map and equivalent list | REQ-0097-KIT |
| 684 Store or club locator | WordPress-only draft from approved public records | REQ-0097-KIT |
| 702 Product feedback mining | Sourced local feedback themes | REQ-0097-LISTEN |
| 703 Competitor mentions in the forum | Guest-visible matches to owner-named terms | REQ-0097-LISTEN |

## Mention evidence and review

1. Use guest-visible active-site evidence or an owner-picked
   bounded file under spec 0100. Record rights, date, window,
   scope, source ID and hash. Scrub or reject private fields,
   free text and media metadata before preview, persistence,
   local drafting or hand-off. Use inert, admitted formats.
   Reject conflicts atomically. Follow no references or timers.
   QR-071 admits formats, rights, coverage and deduplication.
   Supplied files use the byte, row, entry-count and path limits of
   spec 0100 (`import.max_mib`, `import.max_rows`) and an entry count
   cap per archive (proposed 1,000 entries until 0100 names it). A
   ZIP entry path must stay inside the archive; an absolute path or a
   `..` part refuses the file. Escape values on output. A rights or
   permission claim in a file is labelled "owner says". Scan review,
   survey and mention text for names, addresses, phone numbers and
   order IDs. If the scan cannot run, reject the text (fail closed).
2. Reuse exact owner-approved names and aliases. Search the admitted
   population with bounded AND, OR and NOT expressions. Show counts
   before and after explicit exclusions. Preserve unknown tone,
   location and identity. Show source span and method version for
   labels. Compare shares only within a declared, comparable sample.
   For spikes, require comparable windows and the admitted method.
   Until QR-071 admits its floor and method, show observations only.
3. Tag and assign evidence locally or open a crisis checklist.
   Show public authors only with permitted attributable evidence.
   Do not infer identities, contacts or location. Geography uses
   supplied coarse aggregates only. Local Coach replies have no
   external fallback. Pin site, actor, evidence and rights versions.
   Recheck the grant at copy, export or replay under spec 0032.
   Staff send any reply by hand.
4. Rating history consumes the protected 0095 mean of new reviews
   for Google, Yelp and Trustpilot separately (340, 349 and 350).
   Compare like preceding periods, then the same period a year
   earlier once coverage allows. Show scope, scale and count.
   Suppress unknown or revealing cells and totals. QR-071 admits
   the interface. No second archive or blended rating is added.

### Supplied mention contracts (384–406)

An admitted source profile names provider, use rights, exact
supported input fields, identity, time and coverage conventions.
Pin each observation to site, profile revision, account or
population, opaque item ID and version, observed UTC, interval,
rights and content digest. The same observation key and bytes
deduplicate. A collision refuses the file atomically. A proved
correction supersedes the active record, preserving lineage.
Independent captures keep their dates. Within one count scope,
resolve one canonical item by provider, account or population,
source family and native item ID. Keep the exact native occurrence
time and active item version. Repeated or overlapping files add
no extra item. If occurrence time, identity or cross-provider
syndication cannot be proved, withhold that combined count.
Mark partial scope explicitly. Omission proves neither deletion
nor silence. Rights withdrawal revokes old and corrected rows.

For news and public discussions, keep provider scope and record
identity distinct from the supplied page's title or text. Podcast
evidence needs episode ID, approved text or supplied timestamps and
the source rights. Bind each displayed offset to the exact episode
version under the admitted time convention. An offset shown beside
a quote also needs a supplied binding to that exact text span. An
older episode version or another span cannot supply a missing
offset even if the episode ID and wording match. Show “Time in
episode unknown” while keeping independently eligible text. A
proved zero offset means the start of the supplied version. No
stream follows a supplied reference.

A Wikipedia comparison needs two explicitly supplied revision IDs
in order for the same page and language, with readable evidence.
Require proved predecessor order and guest visibility for both.
A missing or unordered revision gives unknown. Follow no provider
address or tag. No credentials, remote jobs or watched accounts.

The Boolean builder accepts quoted terms, approved names and tags
and grouped AND, OR and NOT. Case-fold and normalise whitespace by
the admitted text profile. Match whole tokens or exact token phrases.
Require valid UTF-8, 4,096 bytes overall and 256 per term.
Allow depth eight, 32 terms and 128 tokens. Evaluate at most
10,000 items and 100 output rows in one cancellable page turn.
Use NOT before AND before OR. Source data supplies no operators
or instructions. Disallow regex and terms with private identifiers.
A term is false only against a covered field. NOT swaps true
and false and keeps unknown. AND is false if any side is false,
true if both true, and unknown otherwise. OR is true if any side
is true, false if both false, and unknown otherwise. Keep actor,
scope and expression version. Apply disclosure to all counts,
quotes and results across filters, replay and restore. Show
unknown separately. Suppress revealing counts and complements.

Label tone for an exact target and approved evidence span under
the admitted local classifier. Keep mixed, ambiguous or weak
tone unknown. Unsupported language stays outside this corpus.
Pin item, target, span and method versions. Recheck excerpt rights
and the eligible count release before displaying either. Topic and hashtag views
show actual approved matches and missing scope. Share of voice
uses one fixed version of brands and approved aliases in one
compatible licensed sample.
Denominator is distinct eligible observations matching any set
member. Each numerator counts distinct observations matching that
brand. One observation can match multiple names, so shares need not
sum to one. Report the sample scope and suppress unsafe groups.

Evaluate spikes only after approved local intake or a local
analysis action. Use two adjacent closed seven-day date grids
in the pinned native zone, matched canonical items, coverage,
query and scope. No provider schedule. Require at least ten eligible baseline
matches.
The current count must be at least five higher and at least 1.5
times baseline, using exact arithmetic. Require the source
disclosure floor for both intervals and an admitted comparison
profile. Partial imports, incomplete deduplication or overlapping
reporting populations give no notice. Deduplicate by site, query
and profile revisions, source scope, release cohort and current
interval. Commit the 0099 outbox and evidence in one step. A
correction replaces pending evidence or marks delivered evidence
superseded without re-alerting. Enforce idempotent consumption.
Recheck source, freshness, scope, privacy and destination after
quiet hours. Hold an uncertain delivery. No suppressed counts
appear in the notice. State only change in the measured sample.

Use only the protected 0095 rating projection, keyed by site,
provider, branch, published native interval, zone and scale.
Require current coverage, release decision, aggregate eligible
sum and count, and source version. The admitted producer must
prove original native publication date and one current rating
per eligible review ID. Edits replace the original period's
contribution. Observation date cannot substitute. Compare the
immediately preceding equal-length complete period until a
year of comparable coverage exists. Then align native month and
day endpoints to the prior year and require exact day counts
and comparable coverage. Missing endpoints or leap mismatch give
unknown. Divide released sum by count. No raw reviews, blended
average or new archive. Revoked evidence is unavailable everywhere.

A queue action pins site, item revision, actor, requested tags,
assignee and state. Use one logical effect ID and atomic check and
write. Assign only active permitted staff. Recheck roles, grants,
evidence and version at assign, access, hand-off and replay.
Crisis review offers local prompts to verify evidence, name a
responsible reviewer, choose an approved response or abstain, and
record a hand-off. It never sends or changes site operations.
Staff-only author cards show permitted public attribution with
exact source and count under current rights. No public profile,
contact details or inferred identity, reach or influence.
A mention reply draft in 403 addresses one admitted observation.
For helpful replies in 406, pair one supplied discussion with
current guest-visible own-site evidence about the same topic.
Explain the match and uncertainty. A local draft discloses the
site relationship and contains no personal detail or mass template.
Keep one proposal per exact discussion and site evidence revision.
Cap pairing work at 50 pairs per run and 5 seconds of active time, then
stop and report the rest as not compared (proposed, admit by measurement).
Both draft routes use local-only helpers. Copy requires review of
the exact evidence, rights, wording and current policy. Reject
off-topic or foreign evidence. No model egress or network effect.

Reports freeze query, source coverage, periods, label versions,
rights and suppression policy under 0094 and 0100. Recheck current
grants and lineage before local export, replay or restore. Omit
private data, ambiguous author identities and unlicensed excerpts.
Protected managed copies and jobs follow 0032 and 0118 on erasure.
Existing British-English scope and the 404 refusal still apply.
DR-113's writing-suite languages do not enable mention translation.

#### Named mention intake and review fixtures

These are planned spec 0076 cases. QR-071 and QR-137 retain
format, use rights, source coverage and disclosure admission.
Under DR-088 use a bounded owner-supplied file or existing
guest-visible evidence for the authorised active site. A file,
address, provider profile or staff action grants no login,
polling, referrer-path collection or provider read.

- `DES-0097-W2-SUPPLIED` uses `VAR-0097-W2-SCOPED-INTAKE`,
  `VAR-0097-W2-EPISODE-EVIDENCE` and `VAR-0097-W2-REVISION-PAIR`
  through the same bounded import. Use rights and identity
  must apply to the exact supplied content or comparison.
- `DES-0097-W2-MENTION-COMPARE` uses
  `VAR-0097-W2-TARGET-TONE`, `VAR-0097-W2-LOCAL-SPIKE-NOTICE`,
  `VAR-0097-W2-APPROVED-TOPIC-MATCH` and
  `VAR-0097-W2-SCOPE-SHARE`. Keep unknowns and disclosure
  fences across scope changes, review and replay.
- `DES-0097-W2-RATING-COMPARE` uses
  `VAR-0097-W2-0095-RATING-PROJECTION` for the protected
  producer's released per-provider totals and current lineage.
- `DES-0097-W2-LOCAL-REVIEW` uses
  `VAR-0097-W2-STAFF-AUTHOR-CARD` and
  `VAR-0097-W2-STAFF-CRISIS-CHECKLIST` with exact site and staff
  grants. Assignment and a recorded hand-off share review work
  within the permitted staff queue. It gives no file
  or copy approval and transfers nothing to an outside service.
  The spike fixture targets the existing local 0099 notice.

| Feature | Requirement | Design and variant | Fixture and test | Named fixture assertion |
|---|---|---|---|---|
| F-0384 | REQ-0097-SOURCES | DES-0097-W2-SUPPLIED, VAR-0097-W2-SCOPED-INTAKE | FX-0097-W2-FREE-SOURCES, IT-0097-SOURCES | Give dated, authorised Bluesky, Mastodon and feed-like records. Deduplicate exact observations in each scope and withhold unlicensed excerpts. A repeated item adds no count. No polling. |
| F-0385 | REQ-0097-SOURCES | DES-0097-W2-SUPPLIED, VAR-0097-W2-SCOPED-INTAKE | FX-0097-W2-GDELT, IT-0097-SOURCES | Keep independent licensed news captures and their dates. Dedupe one proved article ID within the sample. Unknown native occurrence time cannot enter a combined count. No GDELT call. |
| F-0386 | REQ-0097-SOURCES | DES-0097-W2-SUPPLIED, VAR-0097-W2-SCOPED-INTAKE | FX-0097-W2-GOOGLE-ALERTS, IT-0097-SOURCES | Give a normalised Alerts file with an address and a contradictory identity. Reject the conflicted import as a whole. In the valid control, keep the address inert and open no connection. |
| F-0387 | REQ-0097-SOURCES | DES-0097-W2-SUPPLIED, VAR-0097-W2-SCOPED-INTAKE | FX-0097-W2-HACKER-NEWS, IT-0097-SOURCES | Use a dated public item and proved title correction. Keep one active contribution and its lineage. Exclude private actor data and infer no author or deleted item. |
| F-0388 | REQ-0097-SOURCES | DES-0097-W2-SUPPLIED, VAR-0097-W2-EPISODE-EVIDENCE | FX-0097-W2-PODCAST, IT-0097-SOURCES | Bind licensed supplied text and a zero offset to the exact episode version and span. Show the offset as the episode start. Give a replacement version with the same ID and wording, but only the old offset. Also give the current version with a timestamp for a different span of the same wording. Both current quotes may show if independently eligible, with time unknown. No stream fetch or implied playback. |
| F-0389 | REQ-0097-SOURCES | DES-0097-W2-SUPPLIED, VAR-0097-W2-REVISION-PAIR | FX-0097-W2-WIKIPEDIA, IT-0097-SOURCES | Compare two readable, ordered and eligible supplied IDs for one page and language. An unread revision, mismatched page or unproved predecessor yields unknown. No revision lookup. |
| F-0390 | REQ-0097-SOURCES | DES-0097-W2-SUPPLIED, VAR-0097-W2-SCOPED-INTAKE | FX-0097-W2-REDDIT, IT-0097-SOURCES | Refuse a missing rights manifest or wrong source family. An embedded instruction in eligible content stays inert. No account call or inferred identity. |
| F-0391 | REQ-0097-SOURCES | DES-0097-W2-SUPPLIED, VAR-0097-W2-SCOPED-INTAKE | FX-0097-W2-YOUTUBE, IT-0097-SOURCES | Keep mention item and channel capture keys separate when visible titles match. Deduplicate only a proved matching key and bytes. No account connection, video access or merged count. |
| F-0392 | REQ-0097-LISTEN | DES-0097-W2-MENTION-COMPARE, VAR-0097-W2-TARGET-TONE | FX-0097-W2-TONE, UT-0097-LISTEN | Apply an admitted local label to a supported exact target and span. Keep ambiguous or weak tone unknown and unsupported language outside the corpus. Loss of excerpt rights holds display. |
| F-0393 | REQ-0097-LISTEN | DES-0097-W2-MENTION-COMPARE, VAR-0097-W2-LOCAL-SPIKE-NOTICE | FX-0097-W2-SPIKE, UT-0097-LISTEN | Compare two closed adjacent seven-day windows with ten baseline and fifteen current matches and proof of both floors. Replay adds no notice. Hold partial evidence or withdrawal. A correction cannot re-alert. |
| F-0394 | REQ-0097-REVIEW | DES-0097-W2-LOCAL-REVIEW, VAR-0097-W2-STAFF-AUTHOR-CARD | FX-0097-W2-ATTRIBUTION, BDD-0097-REVIEW | Show a permitted public byline and a permitted pseudonym with exact source to authorised staff. Withhold the private name. No contact, identity linking or inferred influence. |
| F-0395 | REQ-0097-LISTEN | DES-0097-W2-MENTION-COMPARE, VAR-0097-W2-APPROVED-TOPIC-MATCH | FX-0097-W2-TOPIC-TAG, UT-0097-LISTEN | Match approved token or phrase and tag using the admitted normalisation. Reject the unrelated substring and retain unknown on uncovered fields. No tag lookup. |
| F-0396 | REQ-0097-LISTEN | DES-0097-W2-MENTION-COMPARE, VAR-0097-W2-SCOPE-SHARE | FX-0097-W2-MENTION-SHARE, UT-0097-LISTEN | Deduplicate the eligible sample. Count a dual-brand observation once in the denominator and once in each matching numerator. Hold sparse and incompatible comparisons rather than forcing shares to sum to one. |
| F-0397 | REQ-0097-RATINGS | DES-0097-W2-RATING-COMPARE, VAR-0097-W2-0095-RATING-PROJECTION | FX-0097-W2-RATING-PERIODS, IT-0097-RATINGS | Use only released same-provider, branch, scale and native-period sums and counts. Compare eligible preceding periods or the eligible prior year. A leap mismatch or withdrawn correction gives unknown for the affected comparison. No raw reviews. |
| F-0398 | REQ-0097-REVIEW | DES-0097-W2-LOCAL-REVIEW, VAR-0097-W2-STAFF-CRISIS-CHECKLIST | FX-0097-W2-CRISIS-REVIEW, BDD-0097-REVIEW | Give low-confidence evidence, assign permitted staff and record a local hand-off. Withdrawal holds later access and replay. The checklist offers verification or abstention and performs no site change or communication. |

## Brand kit and assets

1. Store immutable kit versions, approved rules, tokens and examples.
   Use the existing index and crawler evidence for observed usage,
   off-brand comparisons and outdated logos. Show unavailable
   coverage. Exact hash equality marks duplicates. Similarity and
   content maps need admitted methods and a keyboard-accessible
   list. A WordPress-only locator draft uses owner-approved public
   places. Staff publish with existing site approval. An add-on
   route would need signed authority, undo and measured limits
   under QR-071. XenForo shows unavailable. No external map service
   or member location is used.
2. Version each grant with issuing evidence, permitted operations,
   audience, channel, expiry and revocation state. For co-branding
   require both owners' terms. Separate font display, embedding and
   redistribution. Pin exact source and derivative hashes. Recheck
   grants on preview, conversion, export, hand-off and replay.
3. Use approved local requests and sandboxed inert readers. Strip
   private metadata. Refusal leaves no usable bytes or partial
   conversion. Deduplication never merges grants. Kit and guideline
   files use spec 0032 and 0100. Local notices show expiry and the
   affected uses. No asset is sent automatically.

### Versioned brand evidence and permitted uses (407–422)

A kit revision pins site, approved token profile, colour, type and
spacing values, guides, examples, exact asset revision IDs and
approval. Change creates a new revision. A revocation immediately
closes affected previews, proposals and legacy kit access. Keep an
audit-safe history without delivering revoked bytes. The library
indexes approved titles and tags only within the active site.
Search skips withdrawn assets. A version change never changes the
asset's identity or the grants on another revision.

Record asset revision, kind, raw and sanitised hashes, format
and bounds. Each grant has immutable ID and version, source and
derivative revisions, evidence, purpose, operation, audience,
channel, destination, attribution, effective and expiry UTC,
and current revocation generation. A request grants no licence.
Require every applicable grant and the earliest expiry for any
combined output, in `[effective,expires)`. Recheck at claim,
preview, approval, finalisation and copy. Erasure removes managed
bytes. Expiry keeps only allowed history and closes use. Other
unaffected grants stay valid. Exact raw hash
equality means identical supplied bytes within the same site.
Sanitised equality can report identical eligible output separately.
Keep raw and sanitised match namespaces separate. An authorised
actor must be able to compare both exact records before equality,
candidate selection, counts or replies. Compute similarity only
as a bounded background proposal under an admitted profile.
Merge no rights or asset identity. No hidden candidate can affect
visible counts, order or reported timing. Recheck on restore.

Use the 0044 and 0115 own-site observation store for usage. Record
site, page ID, revision, guest scope, read receipt and time,
coverage, component locator and role, asset identity namespace,
digest and evidence kind (declared, resolved or painted). Include admitted
local packs as
a separate evidence kind. A matching name, missing read or obsolete
URL is insufficient. A superseded logo finding needs an exact
old asset match and a currently approved rule for its location.
Match each digest inside its admitted identity namespace and
capture. A painted finding needs paint evidence tied to that
capture and exact asset representation. The same page revision,
component and URL cannot carry paint evidence forward to a later
capture. Label the dated observation as declared, resolved or
painted. When only resolution is proved, show “Appearance
unconfirmed”. Keep any permitted earlier painted observation
dated. Evidence gives no full site coverage, page change or new
asset fetch.

For off-brand checks compare observed and expected values using the
same signed token profile, element role, member or guest scope,
viewport, media, theme and observation interval. Accept known opaque sRGB
values, units
and resolved font identifiers only where 0115 proves them. Alpha,
indirect styles or nested fallback remain unknown unless the exact
profile resolves them. A declared font and a rendered glyph are
different evidence. Show exact expected token, observation kind
and coverage per finding. No page style or word change. The 0041
Coach may read the exact approved voice profile and example rights
for current scope. Reuse spec 0128's protected local guide route.
Revocation stops wording and review consumers.

Guidelines and branded social templates bind the kit revision,
template revision, destination profile and asset grants. A
template uses one admitted inert typed local renderer with
pixel dimensions, colour and font tokens, text bounds, layout,
caption or alt field, credit and final export format. Block
missing required fields in the final format. No arbitrary markup,
active embeds or remote fonts. The rendered bytes and manifest
are the approved 0096/0100 local pack. Sponsor co-branding requires
current compatible grants for both sets of marks and exact
approved placement, relative size, minimum space and credit
rules. Conflicting or absent terms leave a review item only.
Font grants distinguish local file view, naming, renderer use,
preview, binary embedding or subsetting, and redistribution.
A guideline names a font only under its naming and document
rights. This gives no authority to attach or render its bytes.

Requests bind site, actor, requested asset, operation, purpose,
destination, audience and expiry. Staff review the exact version.
Current grants and capability apply at approval and every use.
A withdrawn request cannot create a new approval. Expiry
notice due times are expiry UTC minus 30 or seven elapsed days.
Use one 0099 outbox key per site, grant version, threshold and
approved local recipient. No retroactive alert if the due time has
already passed when approved. Revalidate version, expiry and
current recipient after quiet hours and restore. Renewed or revoked
grants replace pending work. Show permitted asset label and date.
A sent key is never repeated. A notice neither renews nor sends
the licence or asset outside the site owner's app.

Run a pinned, privilege-free 0019/0078 converter with no network.
The budget below is proposed and is admitted by measurement.
Use at most 32 MiB input, 16 million total decoded pixels, 64
frames, four 8-bit channels and 64 KiB metadata. Bound complete
output to the same limits, aggregate scratch to 128 MiB, active
time to 30 seconds and memory to 256 MiB, or lower profile caps.
Count all surfaces and cumulative decode, including rejected
frames. Refuse nested inputs, references, SVG and embedded fonts
without an admitted inert profile. Check dimensions and expansion
during decode. Strip hidden metadata before preview or output. Keep aspect
ratio by default,
make any alternative crop explicit, and show exact dimensions,
format, licence carry-over and required credit. Bind input,
parameters, converter profile and output hash. Work only in
protected scratch. A timeout, resource failure, cancellation or
revocation leaves no usable output.

A kit export pins site, kit, purpose, audience, destination,
approver, profile, manifest, output digests and grant generations.
Preview omissions before approval. Recheck each grant and stage
content plus manifest under the ledger. Publish all as one 0100
effect. Recovery discards an uncommitted bundle and repeats only
the exact committed receipt. Recheck at copy and show omissions
only within the viewer's grant. No site or provider publication. Remove
affected managed copies
and invalidate running jobs on erasure or withdrawal. External
copies already taken by staff cannot be recalled. Store manifests,
temporaries, conversions and managed exports through 0030 and
0118. No public-page work or ranking input is added.

#### Named asset rights and output fixtures

These planned spec 0076 checks use synthetic grants and eligible
local bytes or existing authorised native observations. QR-071
and QR-138 hold the exact formats, measured limits, rights and
source proof. No case acquires media or adds a network route.

- `DES-0097-W2-ASSET-RIGHTS` uses `VAR-0097-W2-EXACT-REQUEST`,
  `VAR-0097-W2-LOCAL-EXPIRY-NOTICE`, `VAR-0097-W2-OPERATION-GRANT`,
  `VAR-0097-W2-DUAL-GRANT-PREVIEW` and
  `VAR-0097-W2-FONT-OPERATION-GRANT`. Validate the exact
  operation and current grant at use. An approved request adds
  no licence or delivery. Expiry notices stay local under 0099.
- `DES-0097-W2-LOCAL-OUTPUT` uses
  `VAR-0097-W2-TYPED-SOCIAL-LAYOUT`, `VAR-0097-W2-ATOMIC-KIT-FILE`
  and `VAR-0097-W2-BOUNDED-CONVERTER`. Use the admitted inert
  local renderer or protected converter and the existing local
  file route. Review exact output and manifest. Replay a
  receipt without another delivery. Failure or revoked rights
  exposes no partial output.
- `DES-0097-W2-BRAND-KIT` uses
  `VAR-0097-W2-PROTECTED-DUPLICATES` and
  `VAR-0097-W2-NATIVE-LOGO-EVIDENCE`. Compare only authorised
  records and reuse existing 0044/0115 observations. A finding
  grants no permission to fetch, change or redistribute an asset.

| Feature | Requirement | Design and variant | Fixture and test | Named fixture assertion |
|---|---|---|---|---|
| F-0413 | REQ-0097-ASSETS | DES-0097-W2-LOCAL-OUTPUT, VAR-0097-W2-TYPED-SOCIAL-LAYOUT | FX-0097-W2-SOCIAL-TEMPLATE, IT-0097-ASSETS | Pin kit, template and destination profile plus asset rights. A valid local preview retains required credit and text access. Missing required fields, revoked rights or an oversized image yields no usable output. |
| F-0414 | REQ-0097-ASSETS | DES-0097-W2-ASSET-RIGHTS, VAR-0097-W2-EXACT-REQUEST | FX-0097-W2-ASSET-REQUEST, IT-0097-ASSETS | Bind asset version, operation, purpose, destination, audience and actor. Change destination or withdraw the original request. Hold approval and use until current exact authority. No automatic delivery. |
| F-0415 | REQ-0097-ASSETS | DES-0097-W2-ASSET-RIGHTS, VAR-0097-W2-LOCAL-EXPIRY-NOTICE | FX-0097-W2-RIGHTS-EXPIRY, IT-0097-ASSETS | Check expiry minus 30 and seven elapsed days. Replaying a current local threshold gives one notice. Quiet hours followed by renewal, revocation, expiry or loss of recipient authority holds old work. |
| F-0416 | REQ-0097-ASSETS | DES-0097-W2-ASSET-RIGHTS, VAR-0097-W2-OPERATION-GRANT | FX-0097-W2-IMAGE-LICENCE, IT-0097-ASSETS | Verify the evidenced crop and required credit within its effective interval. The expiry instant, a prohibited derivative and a public URL without use rights grant no use. Keep separate operation history. |
| F-0417 | REQ-0097-ASSETS | DES-0097-W2-LOCAL-OUTPUT, VAR-0097-W2-ATOMIC-KIT-FILE | FX-0097-W2-KIT-BUNDLE, IT-0097-ASSETS | Interrupt the content and manifest commit or revoke a grant before finalisation. No partial bundle is visible. Restore preserves withdrawal. A committed effect replays its exact receipt without republishing. |
| F-0418 | REQ-0097-KIT | DES-0097-W2-BRAND-KIT, VAR-0097-W2-PROTECTED-DUPLICATES | FX-0097-W2-ASSET-MATCH, UT-0097-KIT | Distinguish raw equality, sanitised equality and admitted similarity. A hidden candidate changes no visible counts, order or reported timing. Replay after restore preserves separate licences and identities. |
| F-0419 | REQ-0097-ASSETS | DES-0097-W2-LOCAL-OUTPUT, VAR-0097-W2-BOUNDED-CONVERTER | FX-0097-W2-LOCAL-CONVERSION, IT-0097-ASSETS | Try multi-frame expansion over the admitted profile, decoder failure, pause, revocation and hidden metadata. Hold failed work and expose no usable scratch or partial file. Eligible replay returns the exact approved receipt. |
| F-0420 | REQ-0097-KIT | DES-0097-W2-BRAND-KIT, VAR-0097-W2-NATIVE-LOGO-EVIDENCE | FX-0097-W2-SUPERSEDED-LOGO, UT-0097-KIT | Use the dated native observation, exact old digest and current location rule. Flag the proven violation only. A permitted historical use or URL-only match yields no violation or new read. |
| F-0421 | REQ-0097-ASSETS | DES-0097-W2-ASSET-RIGHTS, VAR-0097-W2-DUAL-GRANT-PREVIEW | FX-0097-W2-DUAL-BRAND, IT-0097-ASSETS | Render a local preview with both current compatible grants and approved placement and credit. Withhold output when sponsor proof is missing, terms conflict or a prior approval names different terms. |
| F-0422 | REQ-0097-ASSETS | DES-0097-W2-ASSET-RIGHTS, VAR-0097-W2-FONT-OPERATION-GRANT | FX-0097-W2-FONT-RIGHTS, IT-0097-ASSETS | Keep naming, display, rendering, embedding and redistribution rights distinct. A display-only grant cannot embed or copy font bytes. Expired or unproved rights hold the requested output. |

## Surveys and protected results

1. Review the exact wave, consent text, eligibility and close time
   before activation. The add-on rechecks current membership and
   consent at submission. Commit one answer per site, wave and
   eligible person atomically. Use an expiring site-only reference
   bound to its wave and assigned variant. Reject replay, revoked
   eligibility and wrong scope. Provide site-local erasure without
   sending member IDs to the PC. The service pulls signed evidence
   under spec 0021. Until QR-071 admits the route and budgets,
   show plans and eligible imports only.
2. Prepare manual survey links. A site notice or approved email
   needs a distinct opt-in invitation purpose, current consent and
   authorised launch through specs 0099 and 0100. No alert contacts
   or member lists supply recipients. Hold email until that use is
   admitted. No survey action adds work to unrelated public pages.
3. Site-held answers stay within the survey route. The PC receives
   only admitted protected aggregates and authorised scrubbed
   excerpts for themes. Strip identifying text at intake. Apply joint
   floors and anti-deduction rules across filters, waves, totals, NPS,
   themes and exports. XenForo poll input must be eligible aggregate
   evidence. Show sample, scale, exclusions, method and intervals.
   NPS is percent promoters minus percent detractors on the admitted
   scale. Personas use protected results. Never create individual profiles.
4. Pin consent, eligibility, survey and assigned variant versions.
   Predeclare quotas, bounded weights and analysis before opening.
   Admit concurrent filling of quotas and consent withdrawal in the
   plan. A withdrawn response cannot remain in a released cell.
   Changed wording, scale, weights or sampling can break comparison.
   Show coverage and denominators. A weighted opt-in sample cannot
   establish representativeness. Tracker waves never join respondents.
5. Hand only protected aggregates to reporting under spec 0094.
   Match definitions across waves and preserve unknown. QR-071 must
   admit methods, sampling, intervals, disclosure and privacy tests
   before use. An inconclusive survey cannot prove a claim.

### Protected survey contracts (423–435)

Offer original local templates for satisfaction, site improvement
and concept feedback. Staff can select, preview and clone a
template to a new site-owned ID. Each version holds question IDs,
prompts, field kinds, choice IDs, required fields and consent. An
approved wave pins the template version, eligibility, purpose,
site, time zone, UTC open and close instants, retention, analysed
populations, quota and weighting plans, exact variants and
disclosure profile. Changing contents requires a new approval.
Use only admitted question kinds. Bound one wave to 30 questions,
16 options per question and 1,000 Unicode scalar values per free
answer, within the lower admitted host limit. Unknown choices or
unsupported question kinds cannot enter a live wave.

A tracker schedule pins exact planned UTC slots, native zone and
zone database version, scope, wave template and approval. Key a
wave by site, schedule revision and slot. Check current launch
authority and adapter proof before opening each wave. A paused,
expired or missed slot cannot auto-open later. Site close and
resume use the native recorded bounds even when the PC is offline.
Any explicit replacement slot needs an approved revision.

Only the paired site's signed-in native route serves an admitted
wave. Live time is `[open,close)`. The site creates one opaque
reference for the member and wave, never another site's ID.
Bind the assignment and consent revisions. Show the exact
purpose, answers used, optional fields and withdrawal route.
Serialise native membership, eligibility, consent and wave epoch
checks with the form token, assignment, unique member-wave key,
quota count, answer and safe receipt. The same native transaction
must order submission against pause, close, role loss or withdrawal.
A rejected or interrupted change leaves no partial answer or quota.
A committed receipt replays without a second effect. No member ID,
invite recipient or raw answer leaves the site. Only an expressly
approved protected projection can be pulled. Unadmitted routes
remain local plans or authorised imports.

A manual invitation links to the exact site wave. It carries no
member identifier, credential or answer. An admitted 0099 survey invitation
action uses 0101 site approval,
current capability and an exact recipient purpose grant. The
separate launch requires the QR-139 native action mapping and
refuses unsupported hosts. Native
delivery records one key per wave, recipient and purpose inside
the site. The PC receives no identifiers. Revalidate open wave,
actor, recipient eligibility and consent immediately before each
release. Reconcile exact native receipts after interruption and
replay. Keep committed, refused before release and unresolved
invitation outcomes distinct inside the site. A committed receipt
proves only its exact effect. A missing or mismatched receipt
leaves the effect unresolved. Hold it for native reconciliation
under 0099, without a repeat or replacement send. Withdrawal
proved before release refuses that invitation. Never treat a
partly reconciled batch as fully sent. Report only status permitted
for the actor and disclosure scope. No individual outcome or
recipient identifier leaves the site. A local due reminder grants
no delivery.
Survey email has no admitted purpose in 0100. Keep it held unless
the owner separately admits the invitation route. No use of
alert recipients, an exported list or an outside service.

For concept and wording tests, approve all exact variants, target
metric, assignment units, seed commitment and quotas before
opening. Sign allocation ratio, preapproved strata and unit
order. A site-held deterministic assignment binds an eligible
member to one variant for that wave. Browser changes and retries
do not reassign. Reserve quota within the same accepted response
transaction; when the exact group is full return “This group is
closed”. No transfer or reopened quota on withdrawal. A permitted
minimal site-only refusal marker lasts only through wave close
to block repeat. If it cannot lawfully remain, close the affected
admission scope. Remove withdrawn data from every denominator where
applicable. Report assigned, eligible, answered and item-valid
bases separately under disclosure, with achieved balance, coverage
and possible non-response or quota bias. Close serially before
reading a final result.

For each released cohort, record wave and question revision,
eligible and answered denominators, native interval, sampling,
weight profile, omissions, release epoch and aggregate units.
A cross-tab uses only approved answer/category dimensions.
Require at least ten distinct consented respondents per shown
cell and the stricter native floor. Authorise each release in
one atomic site disclosure ledger covering every report, export,
theme, count, comparison, complement and prior manual hand-off.
A missing history, concurrent conflict or revealing combination
withholds the affected cells and derived outputs. Test sparse
deltas after withdrawal using prior release receipts. No individual
members or segments become target lists.
Predeclare positive weights from 0.25 to 4 with exact basis and
strata. Never renormalise only the displayed rows. Show raw
and weighted aggregates separately with valid bases. Use admitted
0066 intervals matching the assignment and sampling design.
Weighted opt-in results describe the sample. Missing data,
withdrawal or changing bases makes a comparison unavailable.

Use the [published NPS definition][nps]. For an admitted 0–10
question, the ordinary unweighted score is 100 times (promoters
minus detractors) divided by all valid integer answers, including
passives at 7–8. Promoters answer 9–10. Detractors answer 0–6.
Report points from -100 to +100. A zero base or unreleasable group
gives unknown. Missing, fractional or out-of-scale answers are
invalid. A separately admitted weighted estimate uses valid
weight sums in numerator and denominator and a weighted label. Satisfaction
uses its own
fixed question and scale. Never turn a different scale into NPS.
Tracker waves compare only exact eligible definitions, consent,
sampling, interval definition, duration, zone, coverage,
wording and weighting. Different observation dates are expected. Never join
respondents
across waves. A wording change is a versioned test, without
merging the trend until comparison is independently admitted.

For a local text theme, accept only an expressly authorised,
scrubbed source projection. If native scrubbing cannot establish
an excerpt is safe, drop it before it reaches the PC. The model
uses local-only analysis and exact approved fields. Purpose-bound expiring
excerpt references
remain protected and follow erasure. Theme labels and results
contain no excerpts or identifiable wording. A result names
method version, scope, denominator and uncertainty.
XenForo polls use the admitted signed aggregate only after the
native subject can see results for the exact node, thread, poll
and option revision. Recheck visibility on every read. Hidden or
interim undisclosed results stay withheld. Label selections and
distinct voters separately. Per-option voter share divides by
all distinct eligible valid voters in that snapshot, when proved.
Unknown basis withholds the share. No NPS or person data.
WordPress has no XenForo poll route.

A withdrawal advances the site survey erasure epoch, invalidates
cached cells, jobs and managed reports, and recomputes or withholds
affected aggregates. Purge protected excerpts and derivatives under 0032.
Preserve
only safe lineage. Each 0021 projection release has a single
use receipt with active site, audience, exact scope and native
epoch. Require a fresh signed check immediately before rendering
or making a managed output visible, with a receipt lifetime of
30 seconds. A changed epoch, unreachable site or missing history
withholds cached views, numbers, exports and restored copies.
Each release is dated to its verified native snapshot. On
learning withdrawal, revoke affected caches and jobs before reuse.
Earlier manual copies are tracked for disclosure without a recall. Admission
under QR-071
needs exact host routes, public footprint and privacy proof.

[nps]:
https://prod.netpromotersystem.com/about/measuring-your-net-promoter-score/

#### Named survey and protected-release fixtures

These planned spec 0076 cases use synthetic actors, consent,
answers and approvals. QR-071 and QR-139 hold exact native
routes, invitation authority, disclosure proof and footprint.
A route without admission stays at local planning or authorised
import. The existing native invitation route grants no survey
email or outside communications.

- `DES-0097-W2-SURVEY-NATIVE` uses `VAR-0097-W2-CONSENTED-WAVE`
  and `VAR-0097-W2-PURPOSE-BOUND-INVITE`. Launch and native
  invitation have separate approvals. No member ID, recipient
  or raw answer goes to the PC. An expressly approved protected
  projection follows 0021. The 0099 invitation uses only an
  admitted native notice and the exact 0101 purpose grant.
- `DES-0097-W2-SURVEY-ANALYSIS` uses
  `VAR-0097-W2-PROTECTED-CROSS-TAB`, `VAR-0097-W2-SCRUBBED-THEME`,
  `VAR-0097-W2-NATIVE-SCALE-NPS`, `VAR-0097-W2-NON-IDENTIFYING-GROUP`
  and `VAR-0097-W2-XENFORO-POLL-AGGREGATE`. Apply current
  native visibility and disclosure before releasing a result.
  Incoming theme excerpts stay protected and released labels
  contain no excerpt or identifying wording.
- `DES-0097-W2-SURVEY-DESIGN` uses `VAR-0097-W2-PINNED-CONCEPT`,
  `VAR-0097-W2-LOCAL-TEMPLATE`, `VAR-0097-W2-ATOMIC-QUOTA`,
  `VAR-0097-W2-PINNED-WORDING` and `VAR-0097-W2-COMPARABLE-TRACKER`.
  Exact wave approval precedes exposure. Release only a
  comparable admitted result.
- `DES-0097-W2-SURVEY-RELEASE` uses
  `VAR-0097-W2-EXACT-AGGREGATE-FILE` for the approved 0094/0100
  local file route. Recheck the single-use release receipt, its
  30-second lifetime, current native epoch and prior disclosure
  immediately before visibility. A missing or expired check
  holds the report. The fixture grants no new export authority.

| Feature | Requirement | Design and variant | Fixture and test | Named fixture assertion |
|---|---|---|---|---|
| F-0423 | REQ-0097-SURVEY | DES-0097-W2-SURVEY-NATIVE, VAR-0097-W2-CONSENTED-WAVE | FX-0097-W2-NATIVE-ANSWER, BDD-0097-SURVEY | Accept one eligible answer at open and reject at close. Race a duplicate and a withdrawal in the site transaction. A lost reply replays the committed receipt. Guests and foreign sessions fail without data egress. |
| F-0424 | REQ-0097-SURVEY | DES-0097-W2-SURVEY-NATIVE, VAR-0097-W2-PURPOSE-BOUND-INVITE | FX-0097-W2-NATIVE-INVITE, BDD-0097-SURVEY | Prepare a manual link with no member ID and a separately approved synthetic site invitation batch. For one lost reply, read the exact committed native receipt. On a second lost reply, give an absent or mismatched receipt. For a third invitation, prove consent withdrawal before release. Replay and preserve one confirmed effect, one unresolved effect with no repeat, and one refusal with no release. Do not mark the batch fully sent. No individual outcome leaves the site. Refuse survey email and contact export. |
| F-0425 | REQ-0097-ANALYSE | DES-0097-W2-SURVEY-ANALYSIS, VAR-0097-W2-PROTECTED-CROSS-TAB | FX-0097-W2-CROSS-TAB, UT-0097-ANALYSE | Compare an eligible cross-tab, a sparse cell and a revealing complement in two release attempts. Show an interval only for the admitted sampling and weighting basis. Unknown remains unavailable on export. |
| F-0426 | REQ-0097-ANALYSE | DES-0097-W2-SURVEY-ANALYSIS, VAR-0097-W2-SCRUBBED-THEME | FX-0097-W2-TEXT-THEMES, UT-0097-ANALYSE | Submit identifying and ambiguous text beside a permitted theme source. Native scrubbing proves safe excerpts before protected intake or drops them. Withdraw a contributor and hold dependent results until recomputed. |
| F-0427 | REQ-0097-ANALYSE | DES-0097-W2-SURVEY-ANALYSIS, VAR-0097-W2-NATIVE-SCALE-NPS | FX-0097-W2-NPS, UT-0097-ANALYSE | Use 0, 6, 7, 8, 9 and 10. The score before the disclosure check is zero points with six valid answers. Add one 9 and one 10 from different eligible respondents. Four promoters, two detractors and two passives over eight valid answers give 25 points. Missing, fractional and out-of-scale answers add nothing to that base. The disclosure check still holds sparse results. Incompatible scales give no NPS and satisfaction stays separate. |
| F-0428 | REQ-0097-TEST | DES-0097-W2-SURVEY-DESIGN, VAR-0097-W2-PINNED-CONCEPT | FX-0097-W2-CONCEPT, IT-0097-TEST | Approve two exact synthetic variants and one allocation. Changing a browser or retrying keeps the same assignment. A revised input needs new approval and withdrawn data stays out of a release. |
| F-0429 | REQ-0097-ANALYSE | DES-0097-W2-SURVEY-ANALYSIS, VAR-0097-W2-NON-IDENTIFYING-GROUP | FX-0097-W2-AGGREGATE-GROUP, UT-0097-ANALYSE | Propose aggregate groups with an unsafe intersection. Deny the revealing release, member-list export and cross-wave identity matching. An eligible cell never becomes a targeting list. |
| F-0430 | REQ-0097-SURVEY | DES-0097-W2-SURVEY-DESIGN, VAR-0097-W2-LOCAL-TEMPLATE | FX-0097-W2-WAVE-TEMPLATE, BDD-0097-SURVEY | Clone a local preset under a fresh site ID. Keep prior question and choice IDs pinned to their saved version. Over-limit and unsupported questions, or changed consent, cannot open a wave under old approval. |
| F-0431 | REQ-0097-TEST | DES-0097-W2-SURVEY-DESIGN, VAR-0097-W2-ATOMIC-QUOTA | FX-0097-W2-QUOTA-WEIGHTS, IT-0097-TEST | Race answers and withdrawal for the last quota place. One committed receipt consumes one place and replay consumes none. Withdrawal never reopens it. Hold unsupported weights and show raw and admitted weighted bases separately. |
| F-0432 | REQ-0097-REPORT | DES-0097-W2-SURVEY-RELEASE, VAR-0097-W2-EXACT-AGGREGATE-FILE | FX-0097-W2-SURVEY-REPORT, E2E-0097-REPORT | Release an eligible aggregate with a current exact signed single-use receipt. Foreign scope, withdrawal, stale receipt or unreachable site holds a managed export or replay. Previously copied files are outside recall. |
| F-0433 | REQ-0097-ANALYSE | DES-0097-W2-SURVEY-ANALYSIS, VAR-0097-W2-XENFORO-POLL-AGGREGATE | FX-0097-W2-XF-POLL, UT-0097-ANALYSE | Compare signed single- and multiple-choice aggregate snapshots. Preserve selections and distinct voters as separate units. Hold share without proved voters and withhold hidden or revoked results. No WordPress route. |
| F-0434 | REQ-0097-TEST | DES-0097-W2-SURVEY-DESIGN, VAR-0097-W2-PINNED-WORDING | FX-0097-W2-QUESTION-WORDING, IT-0097-TEST | Renew one eligible member session after assignment. The assigned wording stays fixed within its wave, submission replay adds no answer, and combining changed wording or different waves cannot prove an effect. |
| F-0435 | REQ-0097-TEST | DES-0097-W2-SURVEY-DESIGN, VAR-0097-W2-COMPARABLE-TRACKER | FX-0097-W2-TRACKER-WAVE, IT-0097-TEST | Compare equal complete native wave definitions. Changed scale, partial coverage or lost consent holds the comparison. A missed or paused slot cannot auto-open after restart. Do not join respondents across waves. |

### Audience themes and mention geography (670 and 674)

Feature 670 creates fictional audience sketches from already
released eligible own-forum aggregates. Admit only purpose and
topic-level counts from the existing protected category, survey
or cohort profile, with site, coverage, date, granularity and
release version. A sketch may show recurring public topics and
planning questions. A stated goal needs an eligible purpose
aggregate that explicitly measured it. Topic counts cannot
explain motives. Derive themes only from released fields.
Any local wording step receives only the protected projection,
without raw terms, segmentation keys, rows or identifiers. Never build records
per person, join
post authors to visits, expose quotes or infer sensitive traits.
No name, demographic inference, contact or linkable hash enters
a sketch. If compared cells could identify someone by subtraction
or narrow scope, suppress them using the shared release rule.
Label the sketches as fictional descriptions of the observed
sample. Selection bias and missing topics remain visible.

Feature 674 uses an owner-authorised coarse supplied region count
by admitted provider and topic scope. Preserve the source's
region taxonomy and level, method and version, count unit,
interval, coverage and unknown region. Use 0097 source identity
and deduplication. If a post or count belongs to overlapping
regions, label them overlapping and withhold additive totals.
A share needs disjoint or separately qualified linked
denominator evidence. Unknown region remains distinct. A name, post, IP, URL
suffix or
language cannot fill missing geography. Never reverse-geocode
or add a market profile. Compare only like source panels.
Use the 0034 shared privacy release across map, table, filters,
tooltips, exports and prior snapshots. Withhold a cell, linked
totals and narrower output when disclosure checks fail.
Rights or scope loss withholds all queued and released views.
Re-import cannot restore a revoked result or bypass suppression.

### Content similarity and public location drafts (682 and 684)

Feature 682 is a local map of comparable guest-visible pages.
Index findings carry site, exact page and index revisions,
language, extraction method, coverage and permitted use. Use
the 0047 duplicate text profile and threshold admission. Distinguish
normalised text equality from bounded shingle similarity. Show
the latter on its zero-to-one scale. Neither establishes
semantic relevance. No chains of equivalence or merge proposal.
Pin the compared sample, pool, graph digest and layout. Show
indexed versus compared eligible coverage. Proposed maximum
2,000 nodes, 20,000 edges and 20 edges per node need admission,
within 0078 worker and storage caps. Clip only with a labelled
partial result. Unauthorised or unread nodes and edges produce
safe scope limits. Use one authorised graph revision for map,
list, filters, tooltips and exports. Recheck actor, site,
rights and erasure as views are released. A stale or revoked
input invalidates nodes and edges before display or restore.
Register derived graphs and retained projections for erasure. No new crawl,
site-link effects or ranking input.

Feature 684 drafts a store or club locator on WordPress from
approved 0095 branch facts and explicit public place records.
Each place binds local ID, label, record version, address and
optional published opening hours, approved same-site link,
source, rights and expiry. Show unknown facts or omit the
unproved place, with an explanation in review. Render an
accessible list sorted by approved label and stable ID, with
search over public place names or locality labels in the
admitted static view. Search reads only the already served
public records and gives no query data to the site. Provide
keyboard access. No geolocation, tracking, remote map, external
fetch, member address or hidden-record count. Generate one guest
projection across referrers and cache paths. Use a local proposal
until QR-069/071 admit the exact signed WordPress draft-only
create or update route. Bind actor and native capability, rights,
expected revision, exact fields, digest, expiry and current
approval. Compare the expected revision before the one write.
Keep an idempotent receipt and preserve later editor changes.
Site staff publish separately. Withdraw expired or revoked
branch, address and rights evidence before display or hand-off,
including draft restore. An admitted managed view proves current
rights and revision across all cache paths or hides stale
content. Qualify footprint and expiry independently. XenForo
has no locator effect.

#### Wave 2 protected audience views and location fixtures

Use synthetic released aggregates, stored same-site observations and
current authorised public records. QR-163 and QR-165 retain exact
disclosure, source and native profile proof. No new capture,
identity profile, external connection or public effect follows.

| Feature | Owning requirement and design | Planned fixture and expected evidence |
|---|---|---|
| F-0670 | REQ-0097-W2-AUDIENCE-SKETCH · DES-0097-W2-FICTIONAL-AUDIENCE-SKETCH | FX-0097-W2-FICTIONAL-AUDIENCE-SKETCH · UT-0097-W2-AUDIENCE-SKETCH. Reuse UT-0097-ANALYSE for shared small-group disclosure. Use a current eligible own-forum topic aggregate and an independently measured eligible purpose aggregate from the lawful release profile. A fictional description names eligible scope and coverage. Remove the purpose evidence and withhold the goal. Try a sparse intersection, person row, foreign aggregate and a revoked source on replay. Suppress the revealing cells and refuse unapproved or stale inputs before any wording step. No identity, private trait, unmeasured motive or reconstructed total follows. |
| F-0674 | REQ-0097-LISTEN · DES-0097-W2-PROTECTED-REGION-COUNTS | FX-0097-W2-PROTECTED-REGION-COUNTS · UT-0097-LISTEN and E2E-0097-REPORT. Compare exact compatible coarse supplied region counts, overlap, mixed providers and unknown regions. Refuse coordinates as an inferred location and a subtraction across releases that reveals a cell. Keep all views under the same 0034 suppression and fence release after revocation. No geocoding, market or provider action. |
| F-0682 | REQ-0097-KIT · DES-0097-W2-SAFE-SIMILARITY-GRAPH | FX-0097-W2-SAFE-SIMILARITY-GRAPH · UT-0097-KIT. Use one current authorised graph whose exact and near pairs use qualified 0047 text profiles. Compare unrelated nodes, an overlapping near-pair chain, unread node and over-cap sample. Match the same visible graph, list, filters and coverage revision. Hold revoked nodes and edges before release. Near links establish no transitive group, relevance or merge. No new fetch. Share the exact 0144 A144-07 overlap assertion only for identical eligible pair inputs, word groups and confirming comparison. A shared acceptance assertion also needs the same admitted threshold and candidate and coverage conditions. The graph keeps its own equality, map/list parity and withdrawal proof. |
| F-0684 | REQ-0097-KIT · DES-0097-W2-EXACT-PUBLIC-LOCATOR-DRAFT | FX-0097-W2-EXACT-PUBLIC-LOCATOR-DRAFT · UT-0097-KIT. Draft an accessible local WordPress list from exact approved 0095 branch and public place records. Include a vanished branch, unproved address, stale cached view and revoked scope after restore. In a stand-in admitted draft-only capability race a changed native revision. Preserve later staff edits and hold stale input. Without QR-069/071 admit local proposal only. No XenForo effect, member location, geolocation, map service or publication. |

### Own product feedback and rival mentions (702 and 703)

Feature 702 makes local evidence groups for explicitly registered
own products. Bind site, guest-visible source items, product
version, match and tone method versions, interval, scope and
rights. Separate direct requests, reported failures, general
complaints, praise and ambiguity. A phrase alone cannot prove
ownership or identify the product version. Where a source spans
categories, label overlap and avoid summing buckets. Give each
theme its eligible evidence and uncertainty. Show a quote only
under precise 0097 reuse permission and current safe disclosure.
No member profiling, contact, ticket creation or product priority
change follows. A staff member may draft an approved local follow-up.

Feature 703 matches exact active 0092 rival aliases in the same
site's eligible public forum corpus. Use the admitted profile's
literal token boundaries and alias version. Match text independently
of a linked host. Unknown ownership, alias collision or unread
context gives a review item. Treat quotations separately from
original statements. Deduplicate the same observed source and
contribution identity. Show eligible dates, scope and evidence
under the shared 0034 release. Do not claim off-site coverage,
sentiment or endorsement from a name match.

Both features release protected counts separately from source
drill-down. Source review needs current actor, staff action,
native ACL, exact revision and span, purpose, rights and expiry.
Deduplicate the same contribution across quote wrappers and
topic views before counting or proposing copy. Hide snippets,
member IDs, author-bearing address fragments and the existence
of withheld sources from unauthorised views. Recheck grants
and erasure on reads, queued release, preview, copy, export
and restore. No alias lookup, monitoring, external collection
or ranking input.

### Exact listening cases (702 and 703)

| Feature and design | Fixture through UT-0097-LISTEN |
|---|---|
| F-0702 · REQ-0097-LISTEN · DES-0097-W2-EXACT-OWN-PRODUCT | FX-0097-W2-VERSIONED-PRODUCT-SAMPLE. Give two similarly named products and one approved version, an ambiguous request, an eligible observation in two labels and a revoked quote. Hold unproved product identity, keep category overlap visible and avoid summing it, and withhold the quote after withdrawal. No ticket, provider request or product decision. |
| F-0703 · REQ-0097-LISTEN · DES-0097-W2-SCOPED-RIVAL-ALIAS | FX-0097-W2-DEDUPLICATED-MENTION. Give exact and lookalike tokens, repeat a quoted contribution, attach a different host, add protected evidence and revoke the alias. Use the current alias version and literal token boundaries, count the supported contribution once with its quote distinction and hold ambiguous or stale material. No outside lookup or ranking effect. |

## Data, settings and limits

Bind snapshots, brand versions, rights, survey waves and results to the active
site under spec 0108. Spec 0032 covers audit, withdrawal and restore,
including site response references, PC intake, replies, assignments, derived
evidence, queues and managed exports. Withdraw affected access on erasure or
revoked rights and recheck on replay or restore. Shared copies cannot be
recalled. Follow specs 0078 and 0100 for job and file limits. Report and draft
routes must add zero public-page work. QR-071 needs separate measured query,
time and byte budgets for each site action.

| Proposed setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `brand.asset_limit` | assets per site | 1,000 | 1 to 10,000 | next save |
| `brand.evidence_days` | days from admission | 90 | 1 to 365 | next retention pass |
| `survey.active_limit` | waves per site | 3 | 1 to 20 | next activation |

## Failure codes

DR-090 reserves SUT-2600 to SUT-2699. SUT-2600 means evidence is
unavailable. SUT-2601 means import refused. SUT-2602 means rights
need review. SUT-2603 means disclosure is unsafe. SUT-2604 means
the platform route is unavailable. SUT-2605 means a limit was reached.

## Security and privacy

Use site and actor grants under specs 0020 and 0101. Keep raw
evidence, receipts and personal details out of audit output and
external reports. All proposals stay local pending staff action.
No outputs provide a link-ranking signal.

## Requirements

| ID | Requirement | Test design and expected result |
|---|---|---|
| REQ-0097-SOURCES | Evidence MUST be site-scoped, bounded, rights-checked and committed atomically. | IT-0097-SOURCES: exercise each admitted source, replay, conflict, expired rights and a supplied URL without fetching it. Zip-slip case: an entry named `../x`, one named `a/../../x`, an absolute path, a backslash or drive-letter path and a symbolic link are each refused with no file written outside the work folder and nothing committed; an archive of 1,000 entries (proposed cap) is read and one of 1,001 is refused whole before any entry is read. |
| REQ-0097-LISTEN | Local comparisons MUST show scope, coverage and unknowns. | UT-0097-LISTEN: exercise bounded filters, unrecognised names, mixed windows and an ineligible spike. |
| REQ-0097-REVIEW | Review actions MUST use authorised local evidence and MUST NOT contact anyone. | BDD-0097-REVIEW: given a complaint, assign it and draft a reply; no destination receives it. |
| REQ-0097-RATINGS | Ratings MUST stay separate per provider and protected across periods. | IT-0097-RATINGS: check eligible new-review periods, refuse unlike or unsupported providers and verify suppression through the 0095 hand-off. |
| REQ-0097-KIT | Kit checks MUST show versions, sources and missing coverage. | UT-0097-KIT: check duplicate bytes, superseded usage, map/list parity, and the admitted WordPress locator and unavailable XenForo route. |
| REQ-0097-ASSETS | Asset hand-off MUST enforce versioned use rights and expiry. | IT-0097-ASSETS: reject missing font embedding permission, revoked co-branding, malformed media and a changed derivative without partial output. |
| REQ-0097-SURVEY | A wave MUST require eligible signed-in participants and an admitted site route before activation. | BDD-0097-SURVEY: given an admitted survey on each supported platform, accept one eligible reference under concurrency. Refuse wrong wave, replay and expired or withdrawn consent. Recheck after restore. |
| REQ-0097-ANALYSE | Survey views MUST protect small groups and MUST NOT create individual profiles. | UT-0097-ANALYSE: test NPS on an admitted scale, unknown answers, sparse segments and revealing subtotals. |
| REQ-0097-W2-AUDIENCE-SKETCH | A fictional audience sketch MUST use only current eligible released purpose and topic aggregates from the same own-forum scope under QR-163 and the existing 0034 disclosure contract. A stated goal needs independently measured eligible purpose evidence. No individual profile or revealing group may be released. | UT-0097-W2-AUDIENCE-SKETCH uses FX-0097-W2-FICTIONAL-AUDIENCE-SKETCH and the shared UT-0097-ANALYSE disclosure cases. |
| REQ-0097-TEST | Variants and weights MUST follow an approved, comparable plan. | IT-0097-TEST: check stable assignment, quota races, changed wording or weight, and an affected cell after withdrawal. |
| REQ-0097-REPORT | Exports MUST use only protected results under the shared safe export rules. | E2E-0097-REPORT: export permitted aggregates; refuse an erased source and a combination that reveals a small cell. |
| REQ-0097-SCOPE | All actions MUST enforce site, role and limits and MUST NOT poll excluded services. | E2E-0097-SCOPE: try a wrong site, multilingual request, restart and limit neighbours. Observe no remote monitoring. |

### Named mention review and kit fixtures

These planned spec 0076 cases use synthetic evidence and grants.
QR-071, QR-137 and QR-138 hold source, disclosure, exact token
and use-rights admission. DR-088 preserves local work and supplied
evidence. Exclude private community data and foreign-site evidence
from the mention corpus. Staff assignments stay local and confer
no permission to deliver evidence to a team or outside service.

- `DES-0097-W2-MENTION-COMPARE` uses
  `VAR-0097-W2-OWN-PUBLIC-CORPUS`, `VAR-0097-W2-TYPED-FILTER`
  and `VAR-0097-W2-RELEASED-FILTER`. A query uses the eligible
  local sample and the current disclosure decision.
- `DES-0097-W2-LOCAL-REVIEW` uses
  `VAR-0097-W2-STAFF-ASSIGNMENT`, `VAR-0097-W2-SINGLE-MENTION-DRAFT`
  and `VAR-0097-W2-RELEVANT-PAIR-DRAFT`. Drafting gives no
  copy approval, provider read or outreach. An approved copy
  still requires a person to use it outside the app.
- `DES-0097-W2-SCOPE-REFUSAL` covers F-0404 refusal under
  DR-081 and DR-113. There is no translated-mention variant.
  `DES-0097-W2-LOCAL-REPORT` uses
  `VAR-0097-W2-FROZEN-REPORT` through the protected 0094/0100
  file route. Revocation and suppression apply on replay.
- `DES-0097-W2-BRAND-KIT` uses `VAR-0097-W2-IMMUTABLE-KIT`,
  `VAR-0097-W2-LOCAL-ASSET-LIBRARY`, `VAR-0097-W2-PAGE-AND-PACK-USAGE`,
  `VAR-0097-W2-LOCAL-GUIDELINE`, `VAR-0097-W2-MATCHED-STYLE`
  and `VAR-0097-W2-PROTECTED-VOICE-GUIDE`. Pin exact grants and
  evidence. Usage and style findings make no new observation call.

| Feature | Requirement | Design and variant | Fixture and test | Named fixture assertion |
|---|---|---|---|---|
| F-0399 | REQ-0097-LISTEN | DES-0097-W2-MENTION-COMPARE, VAR-0097-W2-OWN-PUBLIC-CORPUS | FX-0097-W2-SPONSOR-MENTION, UT-0097-LISTEN | Use an approved sponsor alias against authorised current guest-visible own-site posts. Private or foreign posts do not enter counts, snippets or matching, even for an otherwise eligible reviewer. |
| F-0400 | REQ-0097-LISTEN | DES-0097-W2-MENTION-COMPARE, VAR-0097-W2-TYPED-FILTER | FX-0097-W2-BOOLEAN, UT-0097-LISTEN | Use synthetic covered and unknown fields to check AND, OR, NOT and parentheses. Double NOT preserves unknown. Accept the admitted depth limit, refuse one level over and reject regex tokens without external evaluation. |
| F-0401 | REQ-0097-LISTEN | DES-0097-W2-MENTION-COMPARE, VAR-0097-W2-RELEASED-FILTER | FX-0097-W2-EXCLUSION, UT-0097-LISTEN | Add and clear an explicit exclusion. Preserve source evidence. Hold before, after or complement counts wherever those comparisons disclose a sparse group. |
| F-0402 | REQ-0097-REVIEW | DES-0097-W2-LOCAL-REVIEW, VAR-0097-W2-STAFF-ASSIGNMENT | FX-0097-W2-ASSIGNMENT, BDD-0097-REVIEW | Pin tag, assignee, actor and current item revision. Replay one effect, revoke the assignee and try a stale revision. Preserve audit-safe history with no delivery and no later protected access. |
| F-0403 | REQ-0097-REVIEW | DES-0097-W2-LOCAL-REVIEW, VAR-0097-W2-SINGLE-MENTION-DRAFT | FX-0097-W2-REPLY-DRAFT, BDD-0097-REVIEW | Draft against one admitted observation. Revoke its rights or omit exact staff review. There is no usable copy. Approved copying stays local, with no automated outreach or remote drafting. |
| F-0404 | REQ-0097-SCOPE | DES-0097-W2-SCOPE-REFUSAL | FX-0097-W2-LANGUAGE-REFUSAL, E2E-0097-SCOPE | Request translated mention monitoring and observe refusal before any data route starts. A separate authorised multilingual writing fixture retains its own scope and supplies no mention input. |
| F-0405 | REQ-0097-REPORT | DES-0097-W2-LOCAL-REPORT, VAR-0097-W2-FROZEN-REPORT | FX-0097-W2-PROTECTED-REPORT, E2E-0097-REPORT | Freeze query, lineage and disclosure in a report. Withdraw an excerpt and replay a queued render and restored copy. Omit withdrawn content and suppress revealing complements. No delivery is started. |
| F-0406 | REQ-0097-REVIEW | DES-0097-W2-LOCAL-REVIEW, VAR-0097-W2-RELEVANT-PAIR-DRAFT | FX-0097-W2-HELPFUL-REPLY, BDD-0097-REVIEW | Use one supplied public discussion and one current relevant guest-visible own-site source. Replaying the exact pair creates no second disclosed proposal. Off-topic, private or revoked context gives no usable draft. No fetch or send. |
| F-0407 | REQ-0097-KIT | DES-0097-W2-BRAND-KIT, VAR-0097-W2-IMMUTABLE-KIT | FX-0097-W2-KIT-REVISION, UT-0097-KIT | Approve one synthetic kit, change its token revision and revoke a contained licence. The older authorised history stays versioned, while revoked bytes and foreign kit access are withheld. |
| F-0408 | REQ-0097-KIT | DES-0097-W2-BRAND-KIT, VAR-0097-W2-LOCAL-ASSET-LIBRARY | FX-0097-W2-ASSET-SEARCH, UT-0097-KIT | Index approved tags of exact active-site asset versions. Hold private metadata, unfinished import and erased assets from results. A grant change removes denied results and their counts. |
| F-0409 | REQ-0097-KIT | DES-0097-W2-BRAND-KIT, VAR-0097-W2-PAGE-AND-PACK-USAGE | FX-0097-W2-OBSERVED-USAGE, UT-0097-KIT | Keep page revision, component and URL fixed across two admitted captures. In the first, paint approved asset A. In the second, resolve a different exact asset B with no eligible paint proof. Show A only at its earlier date if still eligible. Show B as resolved, with appearance unconfirmed. Feeding A’s paint receipt or a digest in the wrong namespace cannot prove B was painted. Keep pack usage separate and stale or unread scope explicit. A renamed URL alone has no asset match. |
| F-0410 | REQ-0097-KIT | DES-0097-W2-BRAND-KIT, VAR-0097-W2-LOCAL-GUIDELINE | FX-0097-W2-GUIDELINE-FILE, UT-0097-KIT | Pin the approved kit and exact tokens in a protected local guideline. Missing font embedding rights hold a file that requires those bytes. Stale kit evidence gives no export. |
| F-0411 | REQ-0097-KIT | DES-0097-W2-BRAND-KIT, VAR-0097-W2-MATCHED-STYLE | FX-0097-W2-TOKEN-CHECK, UT-0097-KIT | Compare a resolved opaque sRGB value and a rendered font under the same admitted theme and role. Unknown alpha, unresolved fallback and a declared but unrendered face give no corresponding verdict. |
| F-0412 | REQ-0097-KIT | DES-0097-W2-BRAND-KIT, VAR-0097-W2-PROTECTED-VOICE-GUIDE | FX-0097-W2-VOICE-GUIDE, UT-0097-KIT | Pin the approved local guide and example rights. Withdraw an example during review. Stop affected wording advice and keep the existing draft intact. No external helper or writing access is added. |

## How it is tested

Designed, not run. Spec 0076 and QR-071 require feature-specific
formats, methods, privacy, rights and platform fixtures before use.

## Changes from the source

DR-088 replaces outside monitoring with supplied evidence and manual
replies. Feature 404 is after version 1. Survey and asset work uses
existing site authority and admitted local routes.
