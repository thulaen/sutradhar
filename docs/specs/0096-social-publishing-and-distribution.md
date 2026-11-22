# 0096: Social publishing and distribution

Prepare social posts and syndication packs for hand-off by staff.
Build a local calendar, check rights and review evidence from files
the owner supplies. The workspace stays within the active site.

DR-088 keeps direct connections and automatic posting, monitoring
and sending for excluded services outside version 1. A destination
in this spec is a profile used for preview and manual copy. It grants
no account access. Existing DR-076 alert routes remain in spec 0100.
Spec 0097 owns brand assets, spec 0098 owns newsletters, and spec
0094 owns general report exports.

Fictional example. Staff prepare a post about an approved cycle guide.
They choose a date, attach an approved photograph and preview a short
caption. The approver sees the exact text and file. The editor copies
the approved pack and posts it through the service. Later the editor
records a public reference or marks the post as cancelled.

## Words used here

- **Destination profile** means local preview rules for one format.
- **Hand-off** means a person takes an approved local copy.
- **Syndication** means another publisher republishes content.
- **Canonical address** means the preferred address of a page.
- **Campaign code** means a shared label for links, without a person ID.

## What this spec designs

| Feature | Version 1 treatment | Requirement |
|---|---|---|
| 356 Bluesky publishing | Local destination pack | REQ-0096-HANDOFF |
| 357 Mastodon publishing | Local destination pack | REQ-0096-HANDOFF |
| 358 Threads publishing | Local destination pack | REQ-0096-HANDOFF |
| 359 Facebook Pages publishing | Local destination pack | REQ-0096-HANDOFF |
| 360 Instagram publishing | Local destination pack | REQ-0096-HANDOFF |
| 361 LinkedIn pages publishing | Local destination pack | REQ-0096-HANDOFF |
| 362 X publishing | Local destination pack | REQ-0096-HANDOFF |
| 363 Pinterest publishing | Local destination pack | REQ-0096-HANDOFF |
| 364 TikTok publishing | Local destination pack | REQ-0096-HANDOFF |
| 365 YouTube uploads and descriptions | Local media and description pack | REQ-0096-HANDOFF |
| 366 Discord announcements | Local announcement pack | REQ-0096-HANDOFF |
| 367 Telegram channel posting | Local channel post pack | REQ-0096-HANDOFF |
| 368 Cross-network post scheduler | Local due dates and calendar | REQ-0096-QUEUE |
| 369 Sharing of hub pages | Draft from an approved public hub or thread | REQ-0096-COMPOSE |
| 370 Best time to post | Compare timing in supplied, comparable observations | REQ-0096-MEASURE |
| 371 Composer with previews | Versioned local previews and limits | REQ-0096-COMPOSE |
| 372 AI caption drafts | Sourced local caption suggestions | REQ-0096-COMPOSE |
| 373 Hashtag suggestions | Topic-related, sourced tag suggestions | REQ-0096-COMPOSE |
| 374 Unified social inbox | Review supplied public interaction evidence in one local list | REQ-0096-IMPORT |
| 375 Social analytics | Show scoped metrics from supplied snapshots | REQ-0096-MEASURE |
| 376 Shared link click tracking | Show admitted first-party campaign aggregates | REQ-0096-LINK |
| 377 Evergreen recycling queue | Propose a fresh review of an older post | REQ-0096-QUEUE |
| 378 Social post approvals | Approve the exact pack before manual use | REQ-0096-APPROVAL |
| 379 Social profile audit | Compare supplied profile evidence with confirmed facts | REQ-0096-IMPORT |
| 380 Campaign grouping | Group drafts and evidence by local campaign | REQ-0096-QUEUE |
| 381 Brand library attachments | Include appropriate, licensed assets | REQ-0096-RIGHTS |
| 382 YouTube channel analytics | Import dated channel measurements | REQ-0096-MEASURE |
| 383 Posting from feeds | Propose drafts from owner-supplied feed copies | REQ-0096-IMPORT |
| 618 Syndication rules | Record scope, credits and preferred page address | REQ-0096-SYNDICATE |
| 619 Flipboard and feed reader readiness | Check admitted facts about an existing feed | REQ-0096-SYNDICATE |
| 620 News publisher readiness checklist | Check admitted publishing criteria with evidence | REQ-0096-SYNDICATE |
| 621 Content partnership tracker | Local organisation, terms and due dates | REQ-0096-SYNDICATE |
| 622 Republishing permissions log | Track the permitted text, use and expiry | REQ-0096-RIGHTS |
| 623 Newsletter web archive pages | Propose a WordPress page from an approved issue | REQ-0096-SYNDICATE |

## Compose, approve and hand off

1. Use owner input and guest-visible source versions from the active
   site. A hub must be approved under spec 0052. Keep an evidence
   link for each proposed factual claim. Use the local Coach only.
   Without it, keep a source-linked manual composer. Tags describe
   the topic. Show unknown demand rather than claiming a trend.
2. Show the destination profile identity, revision, checked date
   and how text and media limits were counted. Require admitted
   metadata, media and descriptions, including the YouTube media
   and description pack. Preserve captions and alternative text.
   QR-070 admits profiles, limits and dated primary rules. An
   unavailable profile permits drafting, but blocks hand-off.
   Previews are local checks.
3. Approval under specs 0020 and 0101 binds approver, site, audience,
   destination-profile revision, draft version, text, links, rights
   and attachment hashes. Show the exact pack for approval. A changed,
   revoked or expired input invalidates it. Recheck the actor's grant,
   input versions and rights on copy, file output and every retry.
4. An approved pack contains the permitted text, link, media and
   required credits. Both clipboard and file outputs use spec 0032's
   visibility, scrub and audit checks. Files use spec 0100's local
   export bounds. A late refusal leaves no partial pack. The app
   makes no provider request, and no
   actor can schedule provider posting. A human may record an
   observed posting time and a public reference. That entry proves
   neither delivery nor undo. Cancellation before hand-off differs
   from withdrawal after a recorded manual post.

## Calendar, rights and syndication

Calendar connections, iCalendar export and RSS report output are not
built (spec 0100). The calendar, due dates and supplied feeds below
stay local.

1. Store local campaign names, owners and due dates in the site's
   time zone. The calendar shows draft, in review, ready, handed
   off, recorded by staff and cancelled. Specs 0019 and 0099 own
   the clock and task rules. Scheduler readiness follows QR-081.
   One stable due-task key survives replay, edits, cancellation and
   restart. A due date cannot approve or send. Recycling creates a
   new draft with a reference to the old one and fresh review.
2. Reuse the brand library in spec 0097. Require asset identity,
   version, rights, attribution, expiry and allowed destination or
   use. Lock the attachment version to approval. No rights or
   unresolved rights gives evidence only, without an attachment.
3. Record approved source version, licence scope, dates, credits
   and preferred address. The partner tracker keeps organisation
   and terms, without private contact export. A same-site comparable
   reprint may propose a canonical through spec 0047. Use the
   dated partner rules below for other destinations.
   Hold a scope or rights mismatch. Use dated primary rules under
   QR-070. Absent or stale feed and publisher evidence stays unknown.
4. On WordPress, bind the archive-page draft to an approved issue
   version and reuse rights from spec 0098. Remove subscriber data,
   tracking and private links. Review the evidence before staff use
   their own site tools. On XenForo the archive route is unavailable.
   Feed readiness uses existing site evidence. It has no feed output
   or provider registration.

## Supplied evidence and measured traffic

1. The owner picks a file for spec 0100's atomic import. QR-070
   admits schemas, roles and deduplication keys for each family.
   Record source, declared account, scope, rights, dates, denominator,
   units and hash. A duplicate within or across files cannot count
   twice. Refuse contradictory or invalid inputs. Strip or reject
   private details in fields, text and media metadata before any
   preview or storage. The local inbox holds admitted public
   interactions only and has no external reply action.
   Supplied files use the byte, row, entry-count and path limits of
   spec 0100 (`import.max_mib`, `import.max_rows`) and an entry count
   cap per archive (proposed 1,000 entries until 0100 names it). A
   ZIP entry path must stay inside the archive; an absolute path or a
   `..` part refuses the file. Escape values on output. A rights or
   permission claim in a file is labelled "owner says".
2. Use admitted normalised CSV, JSON or Parquet for supplied feed
   evidence. A raw feed format needs separate QR-070 admission
   with bounded inert parsing and no entities or embeds. Display
   text only and follow no supplied links. Copied material needs
   source and rights. Profile checks label missing coverage.
3. Admit separate eligible fields for analytics, profiles and
   inbox evidence. Keep analytics by provider, account, format,
   metric, post, period and time zone. For timing show outcome,
   exposure denominator and uncertainty in comparable samples.
   Selected samples may be biased. Until QR-070 admits a floor
   and method, show observations only. Keep unlike scopes apart.
4. Campaign tags and counts need a spec 0034 source admitted under
   QR-070. Until then show "not measured" and add no tag. Approved
   codes identify campaigns only. Show recorded landings from the
   joint population. Protect the floor across filters, time windows
   and exports. Separate totals cannot establish clicks. Keep raw
   query and referrer details out. Add no tracker or redirect.

### Manual social packs and observations (356–383)

A signed destination profile declares ID, version, checked date,
effective interval, text fields and count rules, media limits,
accessibility and attribution fields, and output format. Keep it
separate from approval and credentials. Synthetic profiles exercise
each count method at its exact boundary. A release profile needs
the dated primary evidence in QR-070. An expired or absent profile
keeps the draft editable and marks its preview unavailable.
No provider URL, image or script is loaded by a preview.

Store one draft revision per target and one pack digest over the
ordered target revisions, approved source IDs, claims, wording,
audience, links, destination profiles and exact media and rights.
A profile-specific field mismatch holds that target and the whole
pack. The editor shows each field's measured units and limit.
Preserve required credit, caption tracks and alternate text as
deliverable fields. If a target cannot carry a required field,
hold it. Admit only inert metadata and protected local media bytes.
Strip private metadata before preview and pin the sanitised output
hash, original lineage, source and expiry. Show a field mismatch.
To show measured video fit, bind eligible local measurement
evidence to that exact sanitised byte hash, site and admitted
method and units for the dated destination profile. A shared
filename or nominal duration cannot transfer a measurement from
the original file or a replacement. Missing or mismatched
measurement shows “Video length is unconfirmed” and holds the
whole pack. Compare the eligible duration in the profile’s
measurement units before rounding any display label. A measured
limit failure shows the measured value and the limit. A smaller
target set needs a new reviewed pack.
A source link refers to the version inspected. Local caption
drafts quote no private text and cannot invent a claim. A tag
suggestion shows its observed topic source. Demand is unknown
unless separately admitted as a measured result.

Approval pins the exact pack, site, actor role and 0020/0101 grant,
profile and asset versions and rights. Give each expressly approved
clipboard or file hand-off its own effect ID, approval, mode,
destination and output digests. Recheck immediately before the
clipboard write or atomic protected publication. Keep intent and
completion receipts separate. If clipboard transfer is uncertain,
record unknown and wait for a fresh approved copy. Replay a receipt
without repeating its effect. A manual posting note is staff-observed
only. No provider credentials, webhooks, uploads or account rights.

The calendar stores a stable plan ID, campaign ID, exact pack
revision, site time zone and zone database version, resolved due
instant and owner. The due effect is keyed by plan and slot. An edit
atomically advances the plan revision and supersedes pending work.
Recheck plan, site and role at delivery and restore. Use 0019 and
0099 for clock gaps, repeated times, quiet hours, catch-up and replay.
A due effect creates no file or clipboard entry. A recorded manual
post stays distinct from a cancelled plan. An evergreen proposal
forks the lineage and needs new approval. Campaigns grant no access.

An admitted local manifest attests source and coverage, rights
and byte digest. Key the observation by site, provider, declared
account, source kind, opaque item, metric and projection version,
native interval and zone, and observation time. Same key and
digest deduplicate. Conflicting bytes reject the import. A proved
correction supersedes the old count contribution, with lineage.
Independent captures keep their scope and date. A manual posting
note cannot supply provider facts. Apply current withdrawal and
privacy fences on read, derivation, export and restore. Keep search
clicks, provider click reports and own-site landings separate. Exclude private
messages, identities,
disallowed URLs and file metadata before any preview. A supplied
public interaction may show only permitted excerpt and source
date. Unknown or denied use stays withheld.

For timing, fix one provider account, media, audience, counted
outcome and equal covered follow-up per post. Bucket confirmed
provider publication time and native zone into weekdays and
two-hour intervals. A due time or manual note cannot substitute.
Require ten eligible posts and 100 exposures per bucket, the
source floor and one comparable sample definition. Show counted
outcomes per exposure with an admitted 0066 interval controlling
the bucket comparison family with each post as one cluster.
Use disjoint post sets. Require two eligible buckets or more. Return
one strongest bucket only if its lower bound exceeds all other upper bounds.
Ties, clocks with ambiguous buckets or late corrections give no
active advice until revalidated. An admitted correction to a
post’s publication time withdraws current advice for that
comparison. Revalidate the corrected post’s covered follow-up
and move its whole eligible contribution to the resolved bucket
once. Remove its old contribution even if the overall counts
match. Recheck both bucket floors and the interval comparison on
the corrected generation. Show counts, window and gaps where
authorised. Label an eligible result “The strongest observed
time in this sample”.

Spec 0034 owns the admitted first-party landing route. If it is
unavailable, add no code and show “Not measured”. Otherwise use
one opaque 96-bit random campaign code, URL-safe encoded without
padding and reserved per site. Bind it to the exact approved
same-site landing path. Refuse collisions and unproved legacy
values. A restored code needs proof for its original site,
campaign, exact landing path and source generation. A receipt
for another scope or generation cannot rebind it. A plausible
code or matching address alone gives no attribution. Hold the
old tagged delivery and show “Not measured” while its proof is
missing. A separate untagged preview uses a current eligible
landing and its own exact local approval before delivery. It
cannot change the earlier approved bytes. The 0034 landing route
still requires admission under QR-070 and QR-136. Use existing
consented deduplicated joint aggregates. Suppress small and
revealing overlapping windows. Persist no raw
queries, referrer paths or person IDs. Add no tracker or redirect.
Never join external post reports to individual landings. A withdrawn source
closes
dependent views, exports and pending packs. Copies already outside
the owner's app retain their staff receipt without a recall claim.

#### Named draft, observation and review fixtures

These are planned checks under spec 0076. QR-070 and QR-136 hold
the dated source, format, rights, timing and disclosure proof.
The cases use the existing local and supplied routes under DR-088.

- `DES-0096-W2-COMPOSER` uses `VAR-0096-W2-PUBLIC-SOURCE-DRAFT`,
  `VAR-0096-W2-LOCAL-PREVIEW`, `VAR-0096-W2-LOCAL-CAPTION` and
  `VAR-0096-W2-OBSERVED-TOPIC-TAGS`. Pin exact eligible source
  versions. Drafts stay local and unknown demand gives no claim.
- `DES-0096-W2-SUPPLIED` uses `VAR-0096-W2-PUBLIC-INTERACTION`,
  `VAR-0096-W2-SUPPLIED-PROFILE` and
  `VAR-0096-W2-OWNER-FEED-DRAFT` within one bounded 0100 import.
  Reject conflicting identities and strip private facts before
  preview. A feed address is evidence only.
- `DES-0096-W2-MEASUREMENT` uses `VAR-0096-W2-TIMING-COMPARISON`,
  `VAR-0096-W2-PROVIDER-SNAPSHOT` and
  `VAR-0096-W2-FIRST-PARTY-LANDINGS`. Provider observations and
  0034 first-party landings keep separate units and provenance.
  Only a current admitted 0034 route can issue a campaign code.
- `DES-0096-W2-LOCAL-QUEUE` uses `VAR-0096-W2-REVIEW-FORK`
  and `VAR-0096-W2-LOCAL-CAMPAIGN` for local review and grouping.
  Recheck rights at replay and release. A plan grants no send.
- `DES-0096-W2-PACK-APPROVAL` uses `VAR-0096-W2-LOCAL-HANDOFF`
  and `VAR-0096-W2-SCOPED-ATTACHMENT` within the existing
  clipboard and protected-file route. Changed rights, revision or
  audience holds the exact pack. An uncertain copy cannot replay.

| Feature | Requirement | Design and variant | Fixture and test | Named fixture assertion |
|---|---|---|---|---|
| F-0369 | REQ-0096-COMPOSE | DES-0096-W2-COMPOSER, VAR-0096-W2-PUBLIC-SOURCE-DRAFT | FX-0096-W2-HUB-DRAFT, BDD-0096-COMPOSE | Pin the approved public hub and thread revisions on the active site. Withdraw one source and try a private or foreign thread. Hold the affected draft and require fresh approval of a replacement. |
| F-0370 | REQ-0096-MEASURE | DES-0096-W2-MEASUREMENT, VAR-0096-W2-TIMING-COMPARISON | FX-0096-W2-OBSERVED-TIME, UT-0096-MEASURE | Start with the only two disjoint buckets at ten eligible posts and 100 exposures each. In the first bucket each post gives six counted outcomes from ten exposures, and in the second each gives one from ten. The admitted synthetic interval proof gives the first bucket a lower bound of 0.40 and the second an upper bound of 0.30. Admit the source floor. Correct the publication time of one first-bucket post and move its six outcomes and ten exposures after proving comparable follow-up. Withdraw advice immediately. Revalidation gives 9 posts and 90 exposures versus 11 posts and 110 exposures, so no advice returns. Replay the correction without moving its contribution twice. A planned time or ambiguous clock cannot restore advice. |
| F-0371 | REQ-0096-COMPOSE | DES-0096-W2-COMPOSER, VAR-0096-W2-LOCAL-PREVIEW | FX-0096-W2-PROFILE-PREVIEW, BDD-0096-COMPOSE | Check the same source in two admitted synthetic units. Show each measured value and limit. One target exceeding a limit by one unit holds the whole pack. No remote rendering or narrower fallback pack. |
| F-0372 | REQ-0096-COMPOSE | DES-0096-W2-COMPOSER, VAR-0096-W2-LOCAL-CAPTION | FX-0096-W2-CAPTION, BDD-0096-COMPOSE | Treat source instructions as data. Retain supported claims with eligible evidence. Unsupported claims stay out. With no local helper, show the source-linked manual composer without any outside tool call. |
| F-0373 | REQ-0096-COMPOSE | DES-0096-W2-COMPOSER, VAR-0096-W2-OBSERVED-TOPIC-TAGS | FX-0096-W2-TOPIC-TAGS, BDD-0096-COMPOSE | Try a sourced related topic and an unrelated popular term. Only the related tag has a supported suggestion. Missing demand remains unknown and cannot be called a trend. |
| F-0374 | REQ-0096-IMPORT | DES-0096-W2-SUPPLIED, VAR-0096-W2-PUBLIC-INTERACTION | FX-0096-W2-PUBLIC-INBOX, IT-0096-IMPORT | Replay the local manifest and import a conflicting observation. Keep one eligible item for the exact observation digest and refuse the conflicted load as a whole. Private conversation gives no preview or reply action. |
| F-0375 | REQ-0096-MEASURE | DES-0096-W2-MEASUREMENT, VAR-0096-W2-PROVIDER-SNAPSHOT | FX-0096-W2-SOCIAL-MEASURES, UT-0096-MEASURE | Compare two matched periods and a proved correction. Supersede only the old count contribution. Keep unlike metric units, partial coverage, source revisions and missing values distinct. |
| F-0376 | REQ-0096-LINK | DES-0096-W2-MEASUREMENT, VAR-0096-W2-FIRST-PARTY-LANDINGS | FX-0096-W2-CAMPAIGN-LANDINGS, UT-0096-LINK | Test a valid same-site code, collision and foreign destination. An absent admitted route gives no code and “Not measured”. Suppress revealing overlaps. Restore a valid-looking code for the same site and path without its original generation proof. Offer current proof for a different generation, and separately the wrong scope. None revives tag delivery or attribution. An independently current untagged share keeps its own review. Restore after withdrawal shows no result. Provider clicks do not join landings. |
| F-0377 | REQ-0096-QUEUE | DES-0096-W2-LOCAL-QUEUE, VAR-0096-W2-REVIEW-FORK | FX-0096-W2-EVERGREEN, BDD-0096-QUEUE | Fork an older post into a new draft. Expired rights hold use. Substitute a newly approved version and replay the earlier approval. Only the new exact review can approve a future hand-off. |
| F-0378 | REQ-0096-APPROVAL | DES-0096-W2-PACK-APPROVAL, VAR-0096-W2-LOCAL-HANDOFF | FX-0096-W2-EXACT-PACK, IT-0096-APPROVAL | Change audience, linked claim, profile or media. Hold both output modes under the old approval. Replay an uncertain clipboard receipt. It remains unknown and a fresh approved copy is needed. |
| F-0379 | REQ-0096-IMPORT | DES-0096-W2-SUPPLIED, VAR-0096-W2-SUPPLIED-PROFILE | FX-0096-W2-PROFILE-AUDIT, IT-0096-IMPORT | Compare two dated owner-supplied profiles against confirmed brand facts. Keep each provider scope separate. A missing field stays unknown and causes no lookup. |
| F-0380 | REQ-0096-QUEUE | DES-0096-W2-LOCAL-QUEUE, VAR-0096-W2-LOCAL-CAMPAIGN | FX-0096-W2-CAMPAIGN-GROUP, BDD-0096-QUEUE | Rename a campaign, replay a due reminder and try grouping a foreign-site draft. Keep the task identity stable and create no duplicate due card. Group membership changes neither rights nor permission. |
| F-0381 | REQ-0096-RIGHTS | DES-0096-W2-PACK-APPROVAL, VAR-0096-W2-SCOPED-ATTACHMENT | FX-0096-W2-BRAND-ASSET, IT-0096-RIGHTS | Preview a licensed version with private metadata removed and credit retained. Replace or revoke it before output and try a foreign-site copy. Hold the exact pack, leaving no partial file or clipboard output. |
| F-0382 | REQ-0096-MEASURE | DES-0096-W2-MEASUREMENT, VAR-0096-W2-PROVIDER-SNAPSHOT | FX-0096-W2-YOUTUBE-MEASURES, UT-0096-MEASURE | Import YouTube channel and video aggregates on distinct native time bases. Preserve dates, units and incomplete coverage by scope. Keep the native time bases separate. No guessed total or provider connection. |
| F-0383 | REQ-0096-IMPORT, REQ-0096-APPROVAL | DES-0096-W2-SUPPLIED, VAR-0096-W2-OWNER-FEED-DRAFT | FX-0096-W2-SUPPLIED-FEED, IT-0096-IMPORT, IT-0096-APPROVAL | Offer a draft only for a bounded, rights-proved supplied item. Treat embedded markup and URLs as data and withhold unlicensed text. No automatic feed reading or posting. Manual hand-off requires separate approval. |

### Syndication and public archive evidence (618–623)

A syndication plan pins original article and approved version,
publisher, intended destination, exact portions, source address,
credit, licence scope, expiry and policy profile. Show original
source and preferred address on the review card. For comparable
duplicate pages on the same site, 0047 can check a proposed
canonical address. A cross-publisher plan follows its separately
dated rules. Under [Google News guidance][news-duplication], keep
original and partner-copy roles explicit. For the partner's own
manual review, Googlebot-News with noindex covers News alone;
Googlebot with noindex also covers Google Search. A partner
canonical or robots file change is not the default. Identify
the chosen surface and credit. A same-site canonical is a
preference signal. No app cross-site lookup, canonical write,
indexing directive or publication.

A feed readiness result reads admitted 0115 own-site observations
or a dated supplied manifest under 0100. Pin exact feed, version,
native fields, coverage and checked criteria. For the
[Flipboard RSS profile][flipboard-rss], check feed ownership and
public own-site scope from admitted evidence. Check channel title,
address, description and language. Match item GUID, title, link,
excerpt, publication date with zone, creator and eligible enclosure
metadata. Keep stale or unread evidence distinct from absence.
Label the guidance on recent item count, excerpt length and
image size separately. Use an independent admitted RSS or Atom
profile for generic readers. The app does not output feeds, follow item URLs,
test remote acceptance or connect a publisher account.

A partial local news checklist uses the exact dated [publisher
policy profile][news-policy] and owned evidence. Observe public
dates, bylines, author, publication and publisher or network
information and contact. Check sponsorship or affiliate labels,
advertising prominence and whether a preview matches content
only within the admitted profiles. Show supported, missing,
unproved or stale by check. It proves no policy-wide eligibility,
account acceptance or news appearance.

Reuse the 0098 site organisation identity and terms revision.
Bind agreed use, purpose, due native slot and owner. Use one
0019/0099 key per terms revision and due occurrence. Amendment
or withdrawal fences old reminders. Only the current authorised
owner sees the local reminder. Recheck at read and copy. An embedded
address is inert. Store no private contact unless separately
admitted under 0098; export none here. A rights log pins grant
and source/rendition versions, portion, destination, purpose,
credit, effective date and expiry. Record distinct grant and
revocation events. Member material needs its exact 0098 grant. Bind separate
source
and asset rights for every portion, destination, operation,
audience, credit and expiry. Recheck canonical, grant and lease
generations at exact clipboard, pack commit and native draft
effect. Order withdrawal with first visibility. Fence queues,
old receipts and restores under 0032/0118. Reconcile in-flight
native effects. Previously shared manual copies cannot be recalled.
Historical receipts grant no reuse.

A WordPress archive draft needs a current 0098 issue approved
for public reuse. Prove every included field can be public.
Strip audience and subscriber records, tracking fields, private
links and attachment metadata. Use an escaped, inert local
preview. Keep it local until QR-154 admits a dedicated signed
WordPress create/update draft capability under 0021 and 0052.
Bind site, issue version, native page and draft status if updating,
actor, expected revision, authority, rights generation, exact
payload digest, lease and logical request ID. Recheck at dispatch
and native commit. Store one protected effect receipt. Reconcile
the original ID on lost reply. The adapter must hold a created
draft on expired or revoked rights, including autosave and restart.
No quiet overwrite or deletion of staff work. If native publish
cannot recheck a current draft's rights, keep the route local.
Separate exact native publication requires its own approval.
No feed, public-page or XenForo effect follows from this plan.

[news-duplication]: https://support.google.com/news/publisher-center/answer/9606800?hl=en-GB
[flipboard-rss]: https://about.flipboard.com/rss-guidelines/
[news-policy]: https://support.google.com/news/publisher-center/answer/6204050?hl=en

#### Named syndication, rights and archive fixtures

These planned spec 0076 cases use synthetic source, policy and
native receipts. QR-070 and QR-154 hold dated policy, rights
and host admission. Distinguish approved observations, unknown
coverage and proposals. A checklist proves no legal compliance
or search appearance.

`DES-0096-W2-SYNDICATION-REVIEW` uses exact local source and
destination facts. `DES-0096-W2-PARTNER-RIGHTS` uses the current
0098 organisation, terms and exact-use grants.
`DES-0096-W2-ARCHIVE-REVIEW` uses the separate conditional
WordPress draft route only with current native proof. A missing
gate holds it at local review. Publication needs its own native
approval. No provider account, feed upload or automated delivery
follows from these cases.

| Feature | Requirement | Design and variant | Fixture and existing test boundary | Named fixture assertion |
|---|---|---|---|---|
| F-0618 | REQ-0096-SYNDICATE, REQ-0096-RIGHTS | DES-0096-W2-SYNDICATION-REVIEW, VAR-0096-W2-EXACT-SURFACE-PROPOSAL | FX-0096-W2-SYNDICATION-SCOPE, BDD-0096-SYNDICATE, IT-0096-RIGHTS | Compare a same-site comparable duplicate with a separately rights-checked partner proposal. Identify the approved News-only or combined Search and News scope. Unproved policy, missing credit or changed rights holds the proposal. |
| F-0619 | REQ-0096-SYNDICATE, REQ-0096-IMPORT | DES-0096-W2-SYNDICATION-REVIEW, VAR-0096-W2-SUPPLIED-FEED-CHECK | FX-0096-W2-FEED-READINESS, BDD-0096-SYNDICATE, IT-0096-IMPORT | Inspect an eligible own-feed snapshot with duplicate GUID, unproved byline, unknown date zone and stale item. Show the exact dated checklist and coverage. No URL follow, remote acceptance test or feed output. |
| F-0620 | REQ-0096-SYNDICATE | DES-0096-W2-SYNDICATION-REVIEW, VAR-0096-W2-DATED-PUBLISHER-CHECK | FX-0096-W2-NEWS-CHECKLIST, BDD-0096-SYNDICATE | Check observed byline, date, preview match and sponsor disclosure under one current dated policy profile. Hold stale or unsupported findings. Show missing contact evidence as unknown, with no account or inclusion claim. |
| F-0621 | REQ-0096-SYNDICATE, REQ-0096-QUEUE | DES-0096-W2-PARTNER-RIGHTS, VAR-0096-W2-LOCAL-TERMS-REMINDER | FX-0096-W2-PARTNERSHIP, BDD-0096-SYNDICATE, BDD-0096-QUEUE | Pin current site, organisation, terms revision and due occurrence. Race a changed term with its old reminder. Only current authorised staff view the eligible local reminder. No private contact export. |
| F-0622 | REQ-0096-RIGHTS, REQ-0096-APPROVAL | DES-0096-W2-PARTNER-RIGHTS, VAR-0096-W2-EXACT-PROTECTED-GRANT | FX-0096-W2-REPUBLISHING-GRANT, IT-0096-RIGHTS, IT-0096-APPROVAL | Pin exact portion, destination, credit, operation and expiry. Revoke the grant and restore an earlier record. Hold affected copy, pack and native draft use. Manual copies that already left follow their existing recall limits. |
| F-0623 | REQ-0096-SYNDICATE, REQ-0096-APPROVAL | DES-0096-W2-ARCHIVE-REVIEW, VAR-0096-W2-SIGNED-WP-DRAFT | FX-0096-W2-WP-ISSUE-ARCHIVE, BDD-0096-SYNDICATE, IT-0096-APPROVAL | Strip subscriber, tracking and private fields before the exact authorised draft. Reconcile a lost reply without duplication. Revocation blocks autosave or release. XenForo and any unproved native publication gate leave local review only. |

## Data and limits

Keep drafts, approval events, pack references, plans and admitted
aggregates per site under spec 0108. Spec 0032 covers erasure of
derived records, queued hand-offs and managed copies, with audit-safe
provenance only. Rights withdrawal, erasure or expiry revoke affected
approvals. Recheck before each copy, replay and restore. Copies already
taken outside the app cannot be recalled.

Use specs 0078 and 0100 for job and file limits. This spec adds
zero public-page queries, time and bytes.

## Settings

These settings are proposed.

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `distribution.draft_limit` | active drafts per site | 100 | 1 to 1,000 | next save |
| `distribution.targets_limit` | destinations per pack | 5 | 1 to 20 | next edit |
| `distribution.evidence_days` | days from admission | 90 | 1 to 365 | next retention pass |

## Failure codes

DR-089 reserves SUT-2500 to SUT-2599. SUT-2500 means evidence
missing or stale. SUT-2501 means the import was refused.
SUT-2502 means approval or rights need renewal. SUT-2503 means
a destination or format is unavailable. SUT-2504 means a limit
was reached. Permission errors follow spec 0020.

## Security and privacy

Verify site and actor under specs 0020 and 0101. Keep raw content,
private terms and asset details out of logs and outside reports.
A service profile, file, URL or due date grants no network access.
The app sends no draft, inbox reply, link signal or telemetry.

## Requirements

| ID | Requirement | Test design and expected result |
|---|---|---|
| REQ-0096-COMPOSE | Previews and local drafts MUST use admitted profiles, eligible sources and checked limits. | BDD-0096-COMPOSE: given an approved guide and expired profile, draft locally but hold hand-off. Verify the same sources without a local helper. |
| REQ-0096-APPROVAL | Approval MUST bind the exact scoped pack and MUST be rechecked at hand-off. | IT-0096-APPROVAL: change a link, asset, profile or audience. Revoke the actor during a queued export. Refuse both hand-off paths and leave no partial output. |
| REQ-0096-HANDOFF | Handoff MUST provide a local pack only and MUST NOT call a provider. | E2E-0096-HANDOFF: exercise each profile limit and unavailable state on clipboard and file. Record a manual observation without a credential or outside request. |
| REQ-0096-QUEUE | Due dates and recycled drafts MUST remain local review tasks. | BDD-0096-QUEUE: given a due and a recycled item, cross both clock changes, cancel a slot and restart. Keep one local task and require fresh approval for reuse. |
| REQ-0096-RIGHTS | Attributions and permitted uses MUST survive preview, approval and export; expiry MUST block use. | IT-0096-RIGHTS: reject an expired or changed licence and verify required credit in the eligible pack. |
| REQ-0096-SYNDICATE | Syndication and archive drafts MUST cite source rights and MUST NOT publish. | BDD-0096-SYNDICATE: given an approved issue, offer a scrubbed draft on WordPress and show unavailable on XenForo. Revoke rights and verify both hand-off routes refuse the draft. |
| REQ-0096-IMPORT | Supplied data MUST be scoped, scrubbed, bounded and committed atomically. | IT-0096-IMPORT: test profile, inbox and normalised feed rows. Replay valid evidence, refuse a conflicted row atomically, and follow no address. Zip-slip case: an entry named `../x`, one named `a/../../x`, an absolute path, a backslash or drive-letter path and a symbolic link are each refused with no file written outside the work folder and nothing committed; an archive of 1,000 entries (proposed cap) is read and one of 1,001 is refused whole before any entry is read. |
| REQ-0096-MEASURE | Metrics and time comparisons MUST name comparable evidence and preserve missing data. | UT-0096-MEASURE: compare matched and mismatched windows and refuse a timing recommendation below the admitted floor. |
| REQ-0096-LINK | Shared link reporting MUST use admitted aggregates and MUST NOT add public tracking. | UT-0096-LINK: refuse person IDs and separate-count joins. Test the joint floor, revealing totals, stale permission, a missing route and the F-0376 legacy proof mismatches. |
| REQ-0096-ISOLATION | Actions MUST enforce site, role, erasure, retention and resource limits. | E2E-0096-ISOLATION: test another site, expired evidence, restore, saved exports and all setting bounds. |

## How it is tested

Designed, not run. Spec 0076 and QR-070 require feature-specific
provider, source, rights, privacy and limit fixtures before admission.

### Named local pack and due-card fixtures

These planned spec 0076 fixtures all use the existing manual
workspace. QR-070 and QR-136 hold dated profile, rights and
permission proof. Synthetic profile boundaries establish fixture
behaviour only. They establish no provider limit for release.

- `DES-0096-W2-LOCAL-PACK` uses one local pack route and twelve
  named destination-profile variants. Preview pins the exact
  version, text units, local media and accessibility fields.
  Approval and replay follow 0020 and 0101. Each approved
  clipboard or protected 0100 file effect has a distinct
  receipt and mode. An uncertain copy waits for fresh approval.
  Expired scope or a changed required field holds the full pack.
- A pack link uses the exact guest-visible source revision
  already authorised for the active site. The existing composer
  checks that the pinned source supports the linked claim. A private,
  wrong-site, withdrawn or unproved source holds the hand-off.
  Count link text locally where the profile calls for it.
  Preview and export follow no link or remote asset. These
  fixtures give no quote or repost path, client access, ranking
  query or external account action.
- `DES-0096-W2-LOCAL-QUEUE` uses `VAR-0096-W2-DUE-CARD`
  through the existing 0019/0099 schedule. QR-081 holds its
  scheduler proof. A due card grants no clipboard, file or
  posting approval.

| Feature | Requirement | Design and variant | Fixture and test | Named fixture assertion |
|---|---|---|---|---|
| F-0356 | REQ-0096-HANDOFF | DES-0096-W2-LOCAL-PACK, VAR-0096-W2-BLUESKY-PROFILE | FX-0096-W2-BLUESKY, E2E-0096-HANDOFF | At the synthetic Bluesky text boundary, show the exact units, an eligible source link and the image description. Hold one unit over. A manually approved copy makes no network request. |
| F-0357 | REQ-0096-HANDOFF | DES-0096-W2-LOCAL-PACK, VAR-0096-W2-MASTODON-PROFILE | FX-0096-W2-MASTODON, E2E-0096-HANDOFF, IT-0096-APPROVAL | Count non-ASCII text using the synthetic Mastodon profile. A change of the named instance profile invalidates the old preview and approval. No discovery or fallback profile. |
| F-0358 | REQ-0096-HANDOFF | DES-0096-W2-LOCAL-PACK, VAR-0096-W2-THREADS-PROFILE | FX-0096-W2-THREADS, E2E-0096-HANDOFF, IT-0096-APPROVAL | Replace one Threads image after approval. Both clipboard and local file hand-off hold the whole changed pack. The old approval cannot carry the new sanitised asset hash. |
| F-0359 | REQ-0096-HANDOFF | DES-0096-W2-LOCAL-PACK, VAR-0096-W2-FACEBOOK-PAGES-PROFILE | FX-0096-W2-FACEBOOK-PAGES, E2E-0096-HANDOFF, E2E-0096-ISOLATION | Prepare a pack for one approved local audience. The foreign site profile is refused with no bytes exposed. Changing the approved audience requires fresh review. |
| F-0360 | REQ-0096-HANDOFF | DES-0096-W2-LOCAL-PACK, VAR-0096-W2-INSTAGRAM-PROFILE | FX-0096-W2-INSTAGRAM, E2E-0096-HANDOFF, IT-0096-RIGHTS | Use admitted media with required credit and alternate text fields. Missing required alternate text holds the pack and leaves reviewed local media and credit intact. |
| F-0361 | REQ-0096-HANDOFF | DES-0096-W2-LOCAL-PACK, VAR-0096-W2-LINKEDIN-PAGES-PROFILE | FX-0096-W2-LINKEDIN-PAGES, E2E-0096-HANDOFF, IT-0096-APPROVAL | Expire a LinkedIn Pages profile between approval and hand-off. The draft remains editable. Preview and both hand-off modes become unavailable even on replay. |
| F-0362 | REQ-0096-HANDOFF | DES-0096-W2-LOCAL-PACK, VAR-0096-W2-X-PROFILE | FX-0096-W2-X, E2E-0096-HANDOFF | Count a link with the synthetic X profile rule at the limit and just over it. Show the counted units without reading the URL or using a provider account. Hold the over-limit pack. |
| F-0363 | REQ-0096-HANDOFF | DES-0096-W2-LOCAL-PACK, VAR-0096-W2-PINTEREST-PROFILE | FX-0096-W2-PINTEREST, E2E-0096-HANDOFF, IT-0096-RIGHTS | Try a revoked image licence and an incompatible attachment type. Hold the pack with the applicable reason and leave no output. No remote conversion or upload. |
| F-0364 | REQ-0096-HANDOFF | DES-0096-W2-LOCAL-PACK, VAR-0096-W2-TIKTOK-PROFILE | FX-0096-W2-TIKTOK, E2E-0096-HANDOFF | Use a synthetic profile with an inclusive 10,000 ms limit. For each case measure the exact sanitised bytes. A qualified 10,000 ms fits for review and 10,001 ms exceeds the limit even if display rounding matches. A valid 10,000 ms record for a different byte hash with the same filename leaves length unconfirmed. Preserve captions and credits. The refused cases create neither clipboard output nor a file and make no provider call. |
| F-0365 | REQ-0096-HANDOFF | DES-0096-W2-LOCAL-PACK, VAR-0096-W2-YOUTUBE-PROFILE | FX-0096-W2-YOUTUBE, E2E-0096-HANDOFF, IT-0096-APPROVAL | Bind approved media, description and exact public source revision. Withdrawing a file right or substituting a private link holds the pack, leaving no partial output and no lookup. |
| F-0366 | REQ-0096-HANDOFF | DES-0096-W2-LOCAL-PACK, VAR-0096-W2-DISCORD-PROFILE | FX-0096-W2-DISCORD, E2E-0096-HANDOFF | Give an embedded webhook URL and an untrusted mention. The credential or executable data cannot enter a pack. Test the admitted inert format separately and record zero calls or notifications. |
| F-0367 | REQ-0096-HANDOFF | DES-0096-W2-LOCAL-PACK, VAR-0096-W2-TELEGRAM-PROFILE | FX-0096-W2-TELEGRAM, E2E-0096-HANDOFF, IT-0096-APPROVAL | Change the named channel label after approval. An earlier receipt cannot copy the new revision. The admitted local format uses no bot key or provider dispatch. |
| F-0368 | REQ-0096-QUEUE | DES-0096-W2-LOCAL-QUEUE, VAR-0096-W2-DUE-CARD | FX-0096-W2-DUE-CARD, BDD-0096-QUEUE | Edit a scheduled time before a repeated local hour, then cancel and restart. Only the current authorised slot can make a local review card. It causes no copy, file or posting. |

## Changes from the source

DR-088 keeps destination drafts, local planning and supplied analysis.
It places direct publishing, inbox connections, live analytics and
automatic sending for excluded services outside version 1.
