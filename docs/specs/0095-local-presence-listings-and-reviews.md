# 0095: Local presence, listings and reviews

Compare the details on your own location pages with supplied listing
records. Review dated review evidence and prepare corrections or
replies for staff to use by hand. A site may describe more than one
physical location. The active site and British English scope still
apply (DR-081 and DR-082).

DR-088 permits local checks, supplied files and drafts for manual
use in specs 0095 to 0098. Outside services excluded by DR-076
have no connector, polling, automatic publishing or sending here.
Spec 0047 owns the name, address and phone check (344). Spec 0097
owns the wider brand and review-rating history. Spec 0094 owns
general report exports.

Fictional example. A cycle shop has two locations. A supplied listing
shows Saturday hours at the wrong branch. The app shows the two
versions and their dates. The owner confirms the right hours and
previews a correction. A staff member changes the outside listing
in that service using their own account.

## Words used here

- **Listing** means a record describing a business or location.
- **Location key** means the active site's identifier for one branch.
- **Snapshot** means supplied evidence dated to when it was observed.
- **Structured data** means labelled facts in a page's markup.
- **Citation** here means a directory entry for a business location.

## What this spec designs

| Feature | Version 1 treatment | Requirement |
|---|---|---|
| 338 Google Business Profile health | Check supplied listing facts against active-site evidence | REQ-0095-COMPARE |
| 339 Google Business Profile post scheduling | Plan dates and prepare local drafts for manual posting | REQ-0095-PLAN |
| 340 Google review monitoring | Compare dated supplied review snapshots | REQ-0095-REVIEWS |
| 341 Review reply drafts | Prepare a local reply for staff | REQ-0095-REPLY |
| 342 Review sentiment and themes | Show supported themes and uncertainty within one sample | REQ-0095-THEMES |
| 343 Review request campaigns | Prepare wording and a consent checklist for manual use | REQ-0095-PLAN |
| 345 LocalBusiness structured data | Propose labelled facts from confirmed page and location evidence | REQ-0095-MARKUP |
| 346 Location pages manager | Plan or revise WordPress location pages through normal review | REQ-0095-PAGES |
| 347 Multi-location dashboard | Compare branch coverage and findings within one site | REQ-0095-COMPARE |
| 348 Apple Business Connect sync | Prepare a manual correction from supplied listing facts | REQ-0095-COMPARE |
| 349 Yelp review monitoring | Compare dated supplied review snapshots | REQ-0095-REVIEWS |
| 350 Trustpilot review monitoring | Compare dated supplied review snapshots | REQ-0095-REVIEWS |
| 351 Facebook recommendations monitoring | Compare dated supplied review snapshots | REQ-0095-REVIEWS |
| 352 Opening and holiday hours sync | Compare and prepare manual corrections | REQ-0095-HOURS |
| 353 Local landing page audit | Show covered location-page problems and evidence | REQ-0095-PAGES |
| 354 Local event posting | Prepare a dated event draft for manual posting | REQ-0095-PLAN |
| 355 Local citation checklist | Track evidence and local review tasks for named directories | REQ-0095-CITATIONS |

## Location records and listing checks

1. Reuse the site's business-detail record and evidence checks from
   spec 0047, including its confirmation age, time gap and
   normalisation. One branch can use the existing record. QR-069
   requires the 0047 amendment for stable site and branch keys,
   adoption of older records, and removal or retargeting of branches
   before multi-location work. The owner confirms branch matches.
   An ambiguous or unassigned listing stays out of branch totals.
   When 0047 allocates no branch for an older record, keep eligible
   history in its owning view. Until 0047 confirms the exact branch
   and page map, the older record supplies no current branch total,
   comparison, draft or markup proposal. An unsupported reader
   supplies no old values. Preserve independently eligible branches
   and unassigned listings under their existing grants.
2. Use spec 0100's bounded import. Record source, rights, checksum,
   observation time and a branch or explicit unassigned state.
   Strip or reject private data before preview, storage, model
   input or logs. Preview inert text and admit the entire file
   atomically. Invalid schema, rights or scope leave no partial
   rows. Match identity within the site, provider, branch, snapshot
   and source. Identical evidence replays safely. Conflicting
   identities are refused. QR-069 admits file formats and rights.
3. Show entered facts, current page observations and supplied listings
   with their dates. Preserve missing, unreadable, stale and differing
   as separate states. A difference does not decide which is right.
   The branch dashboard shows each branch and unknown coverage.
4. Keep weekly hours and dated exceptions in the location's time zone.
   A dated closure or replacement hours overrides the normal day.
   Treat missing hours as unknown. Show split and overnight ranges
   within their branch. QR-069 needs fixtures for clock jumps,
   repeated times, holiday crossover and overlapping exceptions.
   Stale or ambiguous times need confirmation before a correction.
5. Show supplied directory evidence with source, date, access scope
   and notes. Distinguish never checked, stale and unreadable.
   The checklist follows no supplied address and estimates no map
   position or provider coverage. It has no outside listing action.

## Reviews and manual plans

1. Admit only scrubbed review snapshots that the owner may use. Record
   provider, location, interval, stated coverage, rating scale and
   approved opaque source IDs when available. Keep author names,
   profiles, contact details and booking or order IDs out of both
   fields and free text. With no reliable identity, mark newness
   unknown. Compare snapshots only
   within the same provider, location and scope. Show newly observed
   entries as such. An absent entry cannot establish deletion or a
   period without reviews.
   Scan review text for names, addresses, phone numbers and order IDs.
   If the scan cannot run, reject the text (fail closed).
2. Show sentiment and themes in comparable samples, with a
   deduplicated denominator, provider, scope, scale, window and
   method version. Preserve unknown. Withhold sparse groups,
   revealing totals and differences across dates or filters.
   Aggregates contain no review excerpts. Do not combine scales.
   Spec 0097 can use those protected aggregates only after its
   local interface and disclosure tests are admitted. It creates
   no second archive. QR-069 admits the grouping and tone rules.
3. Use a named review, approved business facts and local brand voice
   to propose a reply. Use the local Coach route only. If it is
   unavailable, show evidence for staff to write the reply.
   Exclude private customer details and invented commitments.
   Pin the fact, policy and review versions to approval. If one
   changes, recheck before manual hand-off.
4. A local plan records a venue, due date, owner and draft for staff
   to copy or use manually. Confirm event and offer details before
   marking it ready. Review request plans show consent and provider
   policy checks, with no audience list, messages or contact export.
   They cannot offer incentives or select only favourable reviewers.
   Use QR-069's dated primary provider rules for reply, event,
   offer and request plans. Missing or expired rules keep the
   affected plan in review. The app records manual hand-off only.

## Own-site pages and markup

1. Use the existing crawler and index to audit guest-visible location
   pages. Show which pages contain the branch's confirmed name,
   address, phone, opening hours and useful directions, with citations
   to the observations. Refer consistency findings to spec 0047.
   Missing or stale evidence shows an audit finding and the next
   check, without a ready draft.
2. On WordPress the location-page manager prepares a new-page or
   update draft with existing editorial approval. Pin target identity
   and version and recheck these and source grants before staff use
   it. Staff publish in the site's own tools. XenForo uses audit only.
3. Propose LocalBusiness markup from agreeing, current guest-visible
   page facts and confirmed branch details. Keep unsupported fields
   out. Reuse the checker in spec 0115. A proposal includes a preview
   and sources. Any apply route under QR-069 needs moderator
   authority and any applicable owner confirmation. Spec 0021's
   signed conditional action binds the site, page revision and
   branch evidence version. Apply and undo are idempotent. Stale
   or conflicting revisions and missing platform routes leave
   proposal text only. Admission needs a measured public-page
   exception and cache parity for publish and undo.

### Typed branch and provider evidence (338–355)

A snapshot identity binds site, provider profile, opaque listing
scope, branch binding revision, observation UTC, rights revision
and source coverage. Its protected manifest binds the byte hash.
Same identity and bytes are one observation. Same identity with
changed bytes rejects the whole import. Independent admitted
captures retain their dates and scope even if bytes match.
Multi-branch input keeps separate branch projections and explicit
unassigned records. Recheck rights and mapping at import, reassignment
and hand-off. Conflicts across dates remain open for review.

For Google listing health and Apple listing corrections, compare the
latest admitted snapshot against confirmed name, public address,
business phone, canonical location page and hours. Each field shows
observed value and date, confirmed value and date, and comparison
reason. Missing fields give unknown. A manually copied correction
pins the provider, listing, branch, fact revision and intended
field. A changed or withdrawn item closes the prepared hand-off.
No listing account authorisation is necessary for reading the file.

Store half-open civil intervals in the branch zone and pinned time
zone database revision. Separate known closure, all-day open and
unknown. Reject a zero-duration open range or conflicting overlaps.
Deduplicate exact duplicates and keep disjoint intervals. Split
overnight projections at midnight. A dated closure removes every
portion for that date, including carry-in from earlier exceptions.
A dated replacement supplies that date's intervals. Its overnight
portion carries over only if the following day has no exception.
Conflicting exceptions fail. Resolve clock gaps and folds with
explicit valid offsets and civil times or withhold that interval.

Review snapshots bind provider, listing, branch, covered time,
native scale, approved stable review ID, observed date and rights.
Compare with a selected scope-matched baseline. “First observed”
needs stable identity and sufficient prior coverage. A return of a
previously seen ID is “Seen again”. An edit changes version only.
A partial baseline, absent identity or omitted ID gives no proof
of new, published or deleted reviews. Preserve provider-native
units, including recommendations separately from stars. Scrub before
preview and keep data off every externally capable helper route.
Count deduplicated eligible IDs in one approved local theme profile
and state its scope and unknowns. Apply disclosure across filters,
earlier managed files and 0097, suppressing small groups and unsafe
complements. Withdraw affected aggregates when rights expire.

Pin purpose, destination profile, branch, zone, policy date, facts
and wording digest. State runs from draft through review to ready
and copied, or stale on invalidation. A 0019/0099 reminder binds
site, branch, plan version, resolved native slot in UTC and zone
database revision. Apply existing quiet-hours and catch-up rules.
One receipt per slot. A reminder never copies or sends. An expired
plan needs a new staff-confirmed time. A reply uses one scrubbed
observation and one current voice guide. The draft excludes
unsupported commitments, prices, personal details and guesses
about a customer. A review request includes a truthful opt-in checklist
and one generic draft. It has no audience, destination credential
or contact list. “Ready to copy” needs exact approved wording,
current facts and dated provider-plus-purpose policy. Recheck all
grants, source and policy digests at the explicit copy click.
An outdated fact can be compared but cannot authorise a copy.
Record the copy as local hand-off only. Repeat clicks have no
additional effect receipt.

### Location page evidence and structured facts

The landing page audit compares indexed guest-visible revisions to
the signed branch, opening hours and direction facts. Define an
eligible page by exact native identity and branch membership.
Mark closed, unassigned, stale, unreadable and confirmed cases
separately. Do not infer location from address similarity. The
dashboard reports eligible, observed and unresolved branches as
distinct counts. It cannot silently omit a branch with no page.

Use [Schema.org LocalBusiness][localbusiness] for the markup
proposal. Include `@context`, `@type`, stable public branch entity
`@id`, `name` and confirmed public `PostalAddress` fields.
Bind the entity ID when the branch is approved. An ordinary page
move or retarget cannot change it. Admit that
same site's canonical `url`, public `telephone` and
`openingHoursSpecification` only when visible and supported.
Only an admitted lossless profile may encode overnight hours or
exceptions. Test input to markup and back, including dated closure.
The output follows the selected profile rather than serialising the
midnight-split comparison projection. Do not include ratings, review text,
unconfirmed facts or customer details. Write typed markup with
escaping and an exact before/after preview, through one admitted
native adapter. No paste of an arbitrary script or remote context
fetch. Follow [Google's structured data guide][structured] for
supported eligibility fields. The proposal makes no result promise.

A publicly visible apply needs the separate 0021 footprint and
cache evidence under QR-069. Without it the UI keeps proposal text
only. Before applying or undoing, recheck native page, branch
generation, 0020 owner and native moderator grant, privacy and
0021 cache and budget evidence. Admit byte, query and time ceilings;
start at 64 KiB per apply, 20 queries per page and 200 ms added time,
all proposed until measured.
Keep the managed node distinct from manual markup. Test native,
full-page and CDN cache invalidation on apply, undo and erasure.
Refuse a route that can re-serve a withdrawn node. One stable
native effect ID and conditional receipt per action. After a lost
reply, use exact native readback under the admitted markup route.
A committed receipt for that effect proves the earlier apply.
Without an eligible matching receipt, show “Could not confirm
the markup change”. Do not resend on that basis. Current node
ownership needs separate authorised evidence. If staff replaced
the managed node manually, matching bytes and public entity
`@id` give no ownership. Show the earlier confirmed action as
history and hold undo with SUT-2406 when the changed native
version is proved. Undo only the exact still-owned node and
preserve later changes. No merge of manual or duplicate markup.
WordPress native drafts need separate staff publication.
XenForo location page audits add no page drafts. On either host, an
unadmitted markup route keeps proposal text only.

[localbusiness]: https://schema.org/LocalBusiness
[structured]: https://developers.google.com/search/docs/appearance/structured-data/local-business

## Data and limits

Store findings, plans, import manifests and approved aggregate views
per site under spec 0108. Register observation, index, aggregate,
plan, draft and managed-copy erasure under spec 0032. Withdraw
dependent access immediately when rights expire or evidence is
erased. Check eligibility again at hand-off, replay and restore.
Copies already taken to outside services cannot be recalled.
Use the limits of specs 0078 and 0100. The reports and proposal route
use zero public-page queries, time and bytes. Any admitted markup apply
budget names each of those limits.

## Settings

The values below are proposed.

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `local_presence.location_limit` | locations per site | 20 | 1 to 100 | next save |
| `local_presence.snapshot_days` | days from approval | 90 | 1 to 365 | next retention pass |
| `local_presence.review_floor` | admitted reviews per shown group | 10 | 5 to 100 | next analysis |

## Failure codes

SUT-2400 to SUT-2499 are reserved under DR-088. SUT-2400 means evidence
missing or stale. SUT-2401 means unadmitted import. SUT-2402 means an
ambiguous branch. SUT-2403 means a suppressed group. SUT-2404 means an
unavailable platform route. SUT-2405 means a limit was reached.
SUT-2406 means the version changed before apply or undo.

## Security and privacy

Enforce the active site and actor grant under specs 0020 and 0101.
Keep the owner's import, matching and external hand-off approvals
in the audit history without raw review or contact text. Files and
reviews are untrusted. Local drafts never trigger sending, polling,
outside account changes or ranking signals.

## Requirements

| ID | Requirement | Test design and expected result |
|---|---|---|
| REQ-0095-COMPARE | The view MUST bind branch evidence and show coverage and differences. | BDD-0095-COMPARE: given two branches and one ambiguous listing, show two separate branch states and one unassigned row. |
| REQ-0095-HOURS | Corrections MUST use confirmed normal or dated exceptional hours in the branch zone. | UT-0095-HOURS: check a public holiday, overnight range, clock gap and repeat, missing hours and overlapping exceptions. |
| REQ-0095-CITATIONS | The checklist MUST show dates, evidence and unknowns. | UT-0095-CITATIONS: distinguish unchecked, stale and unreadable rows, with no fetch when a file contains an address. |
| REQ-0095-REVIEWS | Review comparisons MUST require scrubbed, comparable evidence and label uncertain newness. | IT-0095-REVIEWS: replay valid evidence. A collision, bad row or missing rights leaves no partial import. Remove personal data from fields and free text. A partial snapshot cannot prove a removal. |
| REQ-0095-THEMES | Themes MUST show coverage and suppress sparse groups. | UT-0095-THEMES: test the floor, unlike scales and subtraction across filtered totals and the 0097 hand-off. Withdraw evidence and confirm it disappears from history. |
| REQ-0095-REPLY | Replies MUST remain local and MUST NOT disclose private facts. | BDD-0095-REPLY: a staff member can review a sourced draft; without a local helper they see evidence and no sent reply. |
| REQ-0095-PLAN | Plans MUST be manual and MUST NOT send or select favourable reviewer lists. | IT-0095-PLAN: due dates and a restart leave provider accounts untouched; a consent or factual gap blocks readiness. |
| REQ-0095-PAGES | Page audits MUST show evidence and page drafts MUST use the WordPress review route. | BDD-0095-PAGES: stale evidence offers a recheck. Changed target or lost rights block draft hand-off. XenForo offers audit only. |
| REQ-0095-MARKUP | Markup MUST use confirmed current facts. Apply MUST require approval and an admitted budget. | IT-0095-MARKUP: try stale facts, absent approval, duplicate apply and conflicting undo. Measure per-platform queries, time and bytes and check cache parity. |
| REQ-0095-ISOLATION | The feature MUST enforce site, role, provenance, erasure, retention and resource limits. | E2E-0095-ISOLATION: refuse a foreign site, exercise limits, withdraw a source mid-job and restore without reviving evidence. |

### Named listing, review and request fixtures

Under DR-088 the planned cases use supplied protected evidence,
local analysis, reminders and reviewed manual drafts. QR-069 and
QR-135 hold dated provider policy, rights and disclosure proof.
A reminder or draft gives no provider access. Keep review text
and private facts away from outside helpers.

- `DES-0095-W2-BRANCH-COMPARE` uses `VAR-0095-W2-LISTING-HEALTH`
  to compare supplied facts with the confirmed 0047 branch.
- `DES-0095-W2-MANUAL-PLAN` uses `VAR-0095-W2-LISTING-POST-DRAFT`
  and `VAR-0095-W2-REVIEW-REQUEST-DRAFT` for manual use with
  current exact wording and approvals. The reminder cannot copy.
- `DES-0095-W2-SUPPLIED-REVIEWS` uses
  `VAR-0095-W2-REVIEW-SNAPSHOT` for the same dated-file route as
  the other providers and `VAR-0095-W2-THEME-PROJECTION` for
  one qualified sample and its suppressed aggregates.
- `DES-0095-W2-REPLY-REVIEW` uses `VAR-0095-W2-LOCAL-VOICE-DRAFT`
  for a checked reply or manual guidance when wording is unavailable.

| Feature | Requirement | Design and variant | Fixture and test | Named fixture assertion |
|---|---|---|---|---|
| F-0338 | REQ-0095-COMPARE | DES-0095-W2-BRANCH-COMPARE, VAR-0095-W2-LISTING-HEALTH | FX-0095-W2-GOOGLE-LISTING, BDD-0095-COMPARE | Compare a dated supplied phone with the confirmed branch phone. Show the difference with both dates, while missing hours and an ambiguous listing stay unknown or unassigned. No provider access. |
| F-0339 | REQ-0095-PLAN | DES-0095-W2-MANUAL-PLAN, VAR-0095-W2-LISTING-POST-DRAFT | FX-0095-W2-LISTING-REMINDER, IT-0095-PLAN | Replay a branch plan slot across restart and quiet hours. Keep one local reminder receipt. Recheck branch, rights, policy and exact wording at copy. A changed or expired draft stays in review. |
| F-0340 | REQ-0095-REVIEWS | DES-0095-W2-SUPPLIED-REVIEWS, VAR-0095-W2-REVIEW-SNAPSHOT | FX-0095-W2-GOOGLE-REVIEW-SNAPSHOT, IT-0095-REVIEWS | Replay identical bytes, then compare overlapping admitted files. An edit changes version, a known ID returns as seen again and only proved identity and prior coverage permit first observed. Missing rows prove no deletion. |
| F-0341 | REQ-0095-REPLY | DES-0095-W2-REPLY-REVIEW, VAR-0095-W2-LOCAL-VOICE-DRAFT | FX-0095-W2-LOCAL-REPLY, BDD-0095-REPLY | Check a scrubbed review and approved voice. Refuse private facts or invented compensation. Revoking a source blocks the prepared draft. Without the local helper, show evidence for staff to write it. |
| F-0342 | REQ-0095-THEMES | DES-0095-W2-SUPPLIED-REVIEWS, VAR-0095-W2-THEME-PROJECTION | FX-0095-W2-SCOPED-THEMES, UT-0095-THEMES | Deduplicate the observed eligible review IDs and keep unknown tone visible. Hold sparse groups and complements across filters, past managed files and hand-off. Keep native scales separate and withdraw affected aggregates. |
| F-0343 | REQ-0095-PLAN | DES-0095-W2-MANUAL-PLAN, VAR-0095-W2-REVIEW-REQUEST-DRAFT | FX-0095-W2-GENERIC-REQUEST, IT-0095-PLAN | Give a generic staff-reviewed request with a consent and dated policy checklist. Refuse incentives, favourable selection and a contact list. A missing policy or consent holds the draft in review. |

## How it is tested

Designed, not run. Spec 0076 and QR-069 require source, rules, platform,
rights and footprint admission with feature-specific fixtures.

### Named local presence verification bindings

These are planned spec 0076 fixtures. QR-069 and QR-135 hold
admission for branch lineage, file rights, dated policy, native
review, measured footprint and cache-safe rollback.

- `DES-0095-W2-NATIVE-REVIEW` uses `VAR-0095-W2-TYPED-MANAGED-NODE`
  for markup and `VAR-0095-W2-WP-DRAFT` for WordPress draft review.
  Each route keeps its own scope and approvals. Markup apply
  and undo need the exact 0020 and native 0021 authorities and
  admitted host proof. Without that proof the output stays a local
  proposal. WordPress staff publish separately in their own tools.
- `DES-0095-W2-BRANCH-COMPARE` uses `VAR-0095-W2-BRANCH-COUNTS`,
  `VAR-0095-W2-LISTING-CORRECTION`, `VAR-0095-W2-CIVIL-HOURS`,
  `VAR-0095-W2-NATIVE-PAGE-AUDIT` and `VAR-0095-W2-DIRECTORY-CHECK`.
  Reuse 0047 branch identity and exact rights. Coverage, observed
  facts and confirmed facts are distinct.
- `DES-0095-W2-SUPPLIED-REVIEWS` uses one
  `VAR-0095-W2-REVIEW-SNAPSHOT` route across the three named
  provider fixtures. Keep scopes and provider-native units apart.
  Treat text as data and keep it away from outside helpers.
- `DES-0095-W2-MANUAL-PLAN` uses `VAR-0095-W2-EVENT-DRAFT`
  for a local reminder and reviewed hand-off only. Under DR-088
  none of these provider cases connects, polls, posts or sends.

| Feature | Requirement | Design and variant | Fixture and test | Named fixture assertion |
|---|---|---|---|---|
| F-0345 | REQ-0095-MARKUP | DES-0095-W2-NATIVE-REVIEW, VAR-0095-W2-TYPED-MANAGED-NODE | FX-0095-W2-BRANCH-MARKUP, IT-0095-MARKUP | Show a typed preview from confirmed public facts. An unadmitted cache or footprint leaves proposal text. Using synthetic approvals, commit a managed node and lose its reply. A matching receipt and an eligible still-owned node permit a separate conditional undo once. In a second run staff replace the node with byte-identical manual markup at a later native version. The matching receipt proves history only. SUT-2406 holds undo and keeps staff markup unchanged. Without an eligible receipt the original result stays unknown, with no repeated apply. |
| F-0346 | REQ-0095-PAGES | DES-0095-W2-NATIVE-REVIEW, VAR-0095-W2-WP-DRAFT | FX-0095-W2-WP-LOCATION-DRAFT, BDD-0095-PAGES | Prepare one new WordPress draft and one pinned update. Changed revision or withdrawn rights holds hand-off. XenForo has audit only, and the WordPress draft creates no publication. |
| F-0347 | REQ-0095-COMPARE | DES-0095-W2-BRANCH-COMPARE, VAR-0095-W2-BRANCH-COUNTS | FX-0095-W2-BRANCH-DASHBOARD, BDD-0095-COMPARE | Show eligible, observed and unresolved counts separately. Retain an eligible branch without a page, exclude an unassigned listing from branch totals and fence foreign-site or withdrawn data. Reuse 0047 IT-0047-W2-BRANCH-LINEAGE for the occupied last slot and unsupported reader. Held legacy evidence supplies no current branch count, comparison, branch draft or markup proposal or effect. Other eligible rows stay visible. |
| F-0348 | REQ-0095-COMPARE | DES-0095-W2-BRANCH-COMPARE, VAR-0095-W2-LISTING-CORRECTION | FX-0095-W2-APPLE-CORRECTION, BDD-0095-COMPARE | Show observed and confirmed values and dates from a supplied Apple listing. Bind the manual correction to the exact branch, provider, listing and field. A modified fact version blocks hand-off from the old preview. |
| F-0349 | REQ-0095-REVIEWS | DES-0095-W2-SUPPLIED-REVIEWS, VAR-0095-W2-REVIEW-SNAPSHOT | FX-0095-W2-YELP-SNAPSHOT, IT-0095-REVIEWS | Refuse a denied Yelp file as a whole. Replaying identical snapshot bytes adds no observation. An unchanged ID in a later capture gives no first-observed claim. Missing stable identity or absent coverage proves neither a first observation nor removal. |
| F-0350 | REQ-0095-REVIEWS | DES-0095-W2-SUPPLIED-REVIEWS, VAR-0095-W2-REVIEW-SNAPSHOT | FX-0095-W2-TRUSTPILOT-SNAPSHOT, IT-0095-REVIEWS | Preserve unlike native scales as separate scopes. A comparable correction changes the observed version. Embedded instructions remain text and incomparable snapshots yield no newness claim. |
| F-0351 | REQ-0095-REVIEWS | DES-0095-W2-SUPPLIED-REVIEWS, VAR-0095-W2-REVIEW-SNAPSHOT | FX-0095-W2-FACEBOOK-RECOMMEND, IT-0095-REVIEWS | Compare recommendations only within the matched listing and native scale. Another location or unknown scale cannot join the sample. Recommendations stay separate from star ratings. |
| F-0352 | REQ-0095-HOURS | DES-0095-W2-BRANCH-COMPARE, VAR-0095-W2-CIVIL-HOURS | FX-0095-W2-HOURS-CORRECTION, UT-0095-HOURS | Use the branch zone. Remove carry-in on the closure date. Hold an ambiguous fold and reject conflicting exceptions. Offer a confirmed manual correction. Round-trip only the admitted lossless markup profile. |
| F-0353 | REQ-0095-PAGES | DES-0095-W2-BRANCH-COMPARE, VAR-0095-W2-NATIVE-PAGE-AUDIT | FX-0095-W2-LANDING-AUDIT, BDD-0095-PAGES | Pin the guest-visible page revision and exact branch. Distinguish missing directions, a mismatched confirmed address and an unreadable page. Show an evidence-linked finding or a recheck, with no rewrite. |
| F-0354 | REQ-0095-PLAN | DES-0095-W2-MANUAL-PLAN, VAR-0095-W2-EVENT-DRAFT | FX-0095-W2-LOCAL-EVENT, IT-0095-PLAN | Bind venue, branch, resolved time, wording and dated policy to the approved plan. An amended event or expired plan needs fresh review. Replayed slots give no second receipt and no posting. |
| F-0355 | REQ-0095-CITATIONS | DES-0095-W2-BRANCH-COMPARE, VAR-0095-W2-DIRECTORY-CHECK | FX-0095-W2-CITATION-CHECKLIST, UT-0095-CITATIONS | Show dated, unassigned, stale, unreadable and never-checked states separately. Treat the supplied address as data. No fetch, provider update or inferred map position. |

## Changes from the source

DR-088 replaces outside-service tracking, sync and publishing with
local checks on supplied evidence and proposals for manual use.
The branch page manager and markup use the existing site approvals.
