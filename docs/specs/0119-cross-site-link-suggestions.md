# 0119: Cross-site link suggestions

An owner can ask Sutradhar to suggest relevant links between two of
their registered sites. In a made-up example, a forum discussion about
caring for paper maps could link to the owner's map maintenance guide
on WordPress. A moderator on the forum sees the exact change and the
destination before agreeing to it. A link in the reverse direction
has its own approval.

DR-100 admits this separate workflow. It does not give a normal
one-site search, ranker, MCP token, crawler or portfolio view
permission to read or edit a second site.

## Words used here

A **pair** is two registered sites and their directions approved by
the owner. A **direction** gives one site the source role and the
other the destination role. A **fence** is a current version or grant
that must still match at delivery. An **effect** is one native edit
with its durable result receipt.

## What this spec designs

| Feature | Supported pair | Variants to test |
|---|---|---|
| 10 Forum and blog bridge | One XenForo forum and one WordPress site | Each direction, with each supported WordPress major |
| 65 Sister-site network | Either two XenForo forums or two WordPress sites | Each XenForo direction and same- and mixed-major WordPress pairs |

The existing site register and native permissions belong to specs
0102 and 0101. Specs 0023 and 0027 own reused matching and relevance
mechanisms. Specs 0028, 0021 and 0031 own native review, edit safety
and platform receipts. Spec 0118 governs encrypted custody.
This spec owns pair admission, the restricted two-site read, link
decisions and the cross-site effect type. The portfolio, duplicate
check and WooCommerce bridge keep their own contracts.

## Pair authority

1. Off by default. The owner opens the desktop pair screen, picks
   two active sites, chooses eligible source sections and destination
   sections for each direction, and confirms the exact names and
   canonical origins. The service verifies the same authenticated
   owner holds both registrations and both sites' grants are current.
   Same registrable domain or login name alone proves no authority.
2. A direction is a distinct grant. Per-site rows record the pair ID,
   peer ID, direction, selected scopes, expiry and their acknowledged
   grant generations. Each write uses one site's transaction and row
   policy. Serialise owner updates by the ordered installation
   and site IDs. Confirm both projection digests. Activate only after
   both receipts match a signed monotone marker in the install ledger,
   outside site backups. Enrol the same epoch on the source before
   activating. Missing, old or mixed markers are inert. Enforce
   pair-count and uniqueness limits under that lock.
3. An owner-only coordinator has no database handle or generic API.
   It starts two independent, typed single-site reads with their own
   grant, job and fresh context. Compare bounded public results in
   memory. Clear pooled state before the next context. No joined
   table, shared index, site key, CLI, MCP or portfolio access.
4. Disable, removal, grant expiry, new origin, role loss, lack of
   encryption or a scope change closes the owner's local grant at
   once. Hide cards, cancel unsent work and block effect claim.
   Until the source fences any sent attempts, show “Revoking”.
   A fresh grant uses new epochs. Restore applies the revocation
   ledger before it can open any direction.

## Finding and reviewing a link

1. The owner requests a bounded run for one grant. Use locally
   indexed guest-visible, indexable British-English source pages and
   target pages inside the chosen sections. Neither a locked,
   sensitive, deleted, held or stale item nor private text can enter
   the search. Reject unknown language, incomplete coverage and
   stale visibility. Name the portion checked, never treat a partial
   scan as a complete absence of opportunities.
2. Use source passage features as a temporary query against the
   destination index. Read the target under its own grant and site
   context. The broker may briefly hold only admitted public
   candidate evidence. Do not save the target excerpt, embeddings
   or scores in either site's normal indexes, training or ranking.
3. Use spec 0023's anchor admissibility and retrieval building blocks,
   followed by the spec 0027 two-threshold relevance gate. Admit a
   pair-mode calibration against synthetic site pairs with fixed
   high and low relevance examples. Both sites' effective relevance
   floors and all exclusions apply. A missing calibration, model
   mismatch or unavailable candidate view yields no approvable card.
   Sort only passing candidates by pair-mode relevance, then stable
   source and destination IDs. Visits and money give no lift.
4. A card binds the pair generation, exact versions, source anchor
   range, target canonical HTTPS URL and eligibility evidence. It
   names both sites, direction and the reason the passage helps.
   Show it in the source site's native review with a cross-site
   label. Retain only the target address and evidence references
   needed for review, application and audit under the source grant.
   No other destination content goes to the source add-on.
5. The source moderator must hold that site's edit right and approve
   one exact card. Destination owner approval comes from the active
   direction grant. Approving a card cannot approve the reverse link,
   open a new pair, approve a batch or make future edits automatic.
   Enforce the source page's link and anchor limits, existing-link
   exclusion and destination-domain policy. Never remove native
   link marks or weaken rel, moderation or visibility rules.

## Applying, revoking and undoing

1. Create one `ApplyCrossSiteLink` intent, encrypted and signed
   under specs 0021 and 0118. Bind the logical effect ID, source
   site, post and revision, body and anchor digest, byte range,
   source moderator and single approval, expected result digest,
   exact target site, platform, page, revision and canonical URL,
   plus both grants, pairing epochs, proof digest and expiry.
   No replacement body, script or markup travels. Ordinary
   `ApplyLinkEdit` cannot accept this external target.
2. Call `ReadCrossSiteLinkTarget` through the target pairing.
   The request binds the direction, target ID and grant generation.
   Its signed reply attests the target revision, canonical HTTPS
   URL, origin, guest visibility and indexability. Refuse a new
   origin or redirect, unverified URL or any user information in
   the address. The engine verifies that the reply answers the
   exact target read for this direction and pending source effect.
   Check both current grant projections and the active pair marker
   before signing the one-use assertion. Its expiry is at most
   15 seconds from observation. Recheck those same generations
   when claiming the source effect for dispatch. If withdrawal or
   replacement wins before that claim, discard the late reply and
   any unused assertion. A new grant cannot adopt the old proof.
3. Check the assertion and expiry in the source native transaction.
   The same commit fence checks the source grant epoch, approval,
   edit right, old body and post version, reapplies anchor limits,
   compares the native result hash and stores the effect receipt.
   `FenceCrossSiteDirection` conditionally advances the source epoch,
   from expected to new, with a signed receipt. It serialises with
   native commit and never edits content.
   Reconcile any preceding effect before marking revocation effective.
   If unreachable, hold the revocation until all issued deadlines
   have expired with clock bounds proved. Report unknown past edits
   separately. Revocation does not recall an already committed edit.
   The destination observation is time-bounded. The hosts share no
   transaction. A later permitted check that finds a target change
   flags the source moderator and blocks new intents for that link.
4. Reuse spec 0021's bounded receipt lookup and the appropriate
   0031 save path, including Prahari on WordPress. Commit the
   insertion, terminal state and notice in the fenced job store.
   A known commit stays historical after revocation. Retry only
   a proved pre-save refusal with current authority and the same
   proof bytes. An expired or unknown outcome goes to review.
5. A source moderator can undo an applied link through spec 0028,
   including after revocation. Bind the retained, eligible source
   insertion receipt and recheck the current source edit right and
   exact bytes at the link. Use only this source proof for the
   selective undo. It can proceed when the target is unreachable
   or its grant is withdrawn. The undo opens no target request
   and restores no withdrawn target evidence. The target
   administrator's authority cannot substitute for source edit
   permission. Remove only the recorded link tags and preserve
   later source changes. A conflict needs a person. There is no
   second-site write or automatic deletion.

## Data

All rows use the current site's identifier and forced row rules.
Keep protected copies sealed under spec 0118, separate from the normal
same-site suggestion, ranker and training stores.

| Record | Required fields |
|---|---|
| Grant projection | UUID pair and local/peer site IDs, direction, local scope IDs and origin fingerprint, paired generation counters, owner approval digest, grant expiry, state |
| Run | UUID and source site ID, grant generation, profile digest, current source/destination index generations, cursor, pages inspected, cap used, state and reason |
| Card | UUID and source site ID, pair generation, source revision and anchor range/digest, destination opaque ID and canonical URL, proof and profile digests, expiry and state |
| Eligibility proof | site and page IDs, canonical URL, page revision, guest and indexability facts, grant and pairing generation, signed time and response binding |
| Decision and effect | card ID, source site and native moderator, effect ID, decision and intent digests, proof reference, expiry, state and native receipt reference |

Runtime is owner-requested only. Run through the scheduler and yield
to live work. Limits below are proposed acceptance budgets. Tighten
in the owner review if platform measurements require changes.
All numbers here are proposed until measured.

## Settings

| Setting or fixed limit | Type | Default or value | Allowed range | Takes effect |
|---|---|---|---|---|
| `bridge.enabled` | on or off | off | on after owner confirmation, or off | off fences work at once |
| `bridge.max_pairs` | whole number | 10 | 1 to 20 | next confirmation |
| Direction lifetime | days | 30 | at most 30 | each confirmation |
| Source coverage and candidates | pages, candidates per page | 100 pages, 20 per page | at most those values for one run | before admission |
| Run deadline | seconds | 60 | fixed | each run |
| Target observation age at native apply | seconds | at most 15 | fixed | at apply |
| Card retention | days | up to 7 | grant expiry or source withdrawal wins | before showing a card |
| Inserted link size and count | rendered bytes, links per page | at most 2,304 extra bytes per link; up to 3 links per page | each platform's smaller admitted limit wins | before approval and apply |

An unapplied card carries no extra work, queries or bytes on public
pages. An approved edit uses normal rendering and adds no query,
external call, script or per-view eligibility check. The per-page
addition is at most 6,912 bytes from this feature. Footprint tests
also require a proposed p95 increase of at most 1 ms against the
same native render fixture before enabling the effect.

## Failure codes

| Code | Behaviour and plain-English result |
|---|---|
| SUT-3800 | “Choose two sites you own.” Reject an ineligible pair. |
| SUT-3801 | “This direction needs approval.” Hold a disabled, expired or incomplete grant. |
| SUT-3802 | “The link could not be checked.” Hold missing evidence or an incomplete run. |
| SUT-3803 | “A page or approval has changed.” Withdraw the card and refuse the stale effect. |
| SUT-3804 | “We could not confirm the edit.” Hold the effect for native receipt reconciliation. |

Codes SUT-3800 to SUT-3899 are reserved for this spec (DR-105).

## Erasure

Use spec 0032 to remove projections and derived cards when a site,
source, target or subject is withdrawn. Persist the revocation
before serving the next result. Keep only the audit and native
receipt fields required to explain past edits or let a permitted
source moderator undo. Replicas and restores follow the same fence.
Logs contain safe codes and opaque IDs, with no content or URL.

## Security and privacy

Each direction is a distinct, expiring grant, and the owner coordinator
holds no database handle. Protected copies are sealed under spec 0118
and kept apart from same-site suggestion, ranker and training stores.
Only public candidate evidence is briefly held. No target excerpt,
embedding or score is saved in a normal index. Revocation closes the
owner's local grant at once. Logs contain safe codes and opaque IDs,
with no content or URL. Use synthetic fixtures only.

## Requirements

| Requirement | Test and expected result |
|---|---|
| REQ-0119-PAIR | UT-0119-PAIR: one missing acknowledgement, mixed restore, wrong owner or origin and third site MUST refuse. A duplicate confirm cannot spend another pair slot. An absent reverse grant permits the selected direction and refuses the reverse. |
| REQ-0119-SCOPE | IT-0119-SCOPE: ordinary ranker, MCP, CLI, duplicate check, same-site pipeline and portfolio MUST remain unable to read the pair. Poison pooled context and revoke while separate reads wait. No wrong-site row or stale result may be delivered. |
| REQ-0119-RELEVANCE | UT-0119-RELEVANCE: below and exactly at both strict floors, above both, highly visited but off-topic, duplicate URL, private page and mismatched model. Only the eligible above-floor pair MUST yield a card. |
| REQ-0119-REVIEW | BDD-0119-REVIEW: show both sites and exact anchor. A target moderator cannot approve a source edit. Approving one link MUST affect only that source, with no reciprocal or batch approval. |
| REQ-0119-EFFECT | IT-0119-EFFECT: wrong or expired pair proof, target redirect, source edit, altered retry, markup payload and ordinary apply bypass. Race revocation before and after native commit and unreachable recovery. Check known commit, proved pre-save failure and expired or unknown receipt. Only a valid ordered effect commits once. |
| REQ-0119-UNDO | IT-0119-UNDO: revoke the pair, then undo with and without a source conflict. Undo MUST remove only the recorded wrapper or change nothing, preserving later WordPress edits. |
| REQ-0119-ERASE | FI-0119-ERASE: race two reads with scope withdrawal and restore older projections. Stale cards, jobs and plaintext evidence MUST remain unavailable. |
| REQ-0119-PARITY | PL-0119 checks all pair directions and supported CMS combinations, page byte/time/query budgets, native link marks, encryption and restart recovery. Any unsupported edit route MUST stay held. |

Implementation admission requires a complete spec 0076 record for
each platform effect, pair-scoring qualification, signed site
contracts and measured footprint. The gates hold the feature until
each required fixture has proof.

### F-0010 verification binding

Feature 10 uses the separately admitted forum and blog pair workflow
under DR-100. It covers one XenForo site and one WordPress site with
one independent grant per direction. Feature 65 uses the same pair
mechanism for its own supported pairs. Share the applicable signed
case result only when it proves the exact declared profile and
direction. A case listed for both features is still one case.

| Design and existing requirements | Existing governing cases |
|---|---|
| `DES-0119-DIRECTION` · REQ-0119-PAIR, REQ-0119-SCOPE, REQ-0119-RELEVANCE | UT-0119-PAIR, IT-0119-SCOPE, UT-0119-RELEVANCE. Prove distinct owner-approved scopes, reverse default denial, separate site reads and the two effective relevance floors. |
| `DES-0119-SOURCE-EFFECT` · REQ-0119-REVIEW, REQ-0119-EFFECT, REQ-0119-UNDO | BDD-0119-REVIEW, IT-0119-EFFECT, IT-0119-UNDO. Source moderation and the current target assertion authorise a single ApplyCrossSiteLink. Lost replies and selective undo use the owning signed effect receipts. |
| `DES-0119-PAIR-FENCE` · REQ-0119-ERASE and REQ-0119-PARITY | FI-0119-ERASE and PL-0119. Exercise withdrawal and restore and measure the exact mixed-platform paths. |

`VAR-0119-XF-TO-WP` and `VAR-0119-WP-TO-XF` each bind the exact
admitted XenForo profile, each admitted WordPress major, the tested
direction and both pairing and consent generations. Neither carries a
proof from the reverse direction. Only the pair coordinator combines bounded
public evidence in
memory after two independent single-site reads. The ordinary
ranker, CLI, MCP, duplicate check, single-site pipeline and
portfolio see no pair rows or evidence. The source native review
and effect receive only the eligible target and evidence refs.
A one-use target assertion expires in at most 15 seconds and
grants no second-site write. QR-102 retains the
approval holds for calibration, signed native effects, platform
protection, measured footprint and revocation proof.

### F-0065 same-platform direction binding

Under DR-100, use the existing REQ-0119-PAIR, REQ-0119-SCOPE,
REQ-0119-REVIEW, REQ-0119-EFFECT and REQ-0119-PARITY with
DES-0119-DIRECTION, DES-0119-SOURCE-EFFECT and DES-0119-PAIR-FENCE.
FX-0119-W1-SISTER-DIRECTIONS binds UT-0119-PAIR, IT-0119-SCOPE,
BDD-0119-REVIEW, IT-0119-EFFECT and PL-0119. It needs independent
proof for each authorised direction of a two-XenForo pair and
same-major and mixed-major WordPress pairs under the supported
native profiles. A mixed forum/blog proof belongs to F-0010.

Withhold the reverse grant, then race a valid source approval and
native commit against withdrawal. Bind target proof and moderator to
their exact sites and current roles. Reconcile pending revocation and
lost replies against the same effect receipt. One direction confers no
reverse, third-site, ordinary ranker or generic actor authority. QR-102
retains exact calibration, native protection, revocation and footprint
proof. Reuse shared cases without counting their results twice.

## How it is tested

Designed, not run. Use synthetic two-site fixtures only. The same checks
run under Bazel on the supported desktop platforms. Each receiver
variant also needs its native review and save evidence. Prove both
clocks at cutover. Each test id is defined in the requirements table:

- **UT-0119-PAIR, UT-0119-RELEVANCE:** unit tests of pair admission and
  the relevance floors.
- **IT-0119-SCOPE, IT-0119-EFFECT, IT-0119-UNDO:** integration tests of
  read isolation, the cross-site effect and undo. In IT-0119-EFFECT,
  deliver the signed target reply after withdrawing or replacing
  either grant, and withdraw again between issuing an assertion
  and claiming its source effect in a separate fixture. Neither
  case dispatches an edit. For a source commit that wins the native
  fence, reconcile a lost reply to that exact source receipt with
  zero additional edits. The destination's content stays unchanged.
  Use the same cases for the F-0010 and F-0065 direction profiles.
  In IT-0119-UNDO, confirm one source commit with its eligible
  receipt. Complete pair revocation and make the target unreachable.
  Keep a later unrelated edit in the source. A currently authorised
  source moderator removes only the recorded wrapper. Assert zero
  target reads, writes or grant renewals and no revived target
  projection. In a separate run withdraw the source edit right
  after the undo preview. Give the target administrator current
  rights at that target. Refuse the source undo and preserve the
  recorded link and later edit. Use each applicable admitted source
  profile and the existing source receipt and conflict cases.
- **BDD-0119-REVIEW:** behaviour test of the source moderator's review.
- **FI-0119-ERASE:** fault injection on withdrawal and restore.
- **PL-0119:** the platform and footprint gate for all directions.

## Changes from the source

The two source features share one owner and workflow. Pair approval
covers directions individually. Each accepted proposal edits only
the source site, with a current check of the target.
