# 0120: Opt-in referral backlink discovery

This feature helps an owner find a public page that links to one of
their threads. It uses a separate, optional route for the referring
page address under DR-107. The route needs a qualified privacy review,
exact owner approval and the visitor's permission before it can
collect a path. It starts off for every site.

In a made-up example, an independent map club links to a forum's
paper-map guide. A visitor allows this feature and the browser
supplies the club's page address. Sutradhar can stage a lead and ask
the owner to check it. The report names a backlink only after the
admitted check finds the link on the public page.

## Words used here

A **referrer path** identifies a page on the referring host. A **lead**
is an observed candidate, which might be wrong. An **approval
generation** is one exact reviewed configuration and its authority.
A **fence** is a version checked again before access or delivery.

## What this spec designs

| Behaviour | Owner and shared contract |
|---|---|
| Feature 106 | This spec owns consent, collection, staging, verified output and erasure |
| Ordinary visits, hosts and consent | Spec 0034 keeps host-only analytics. Reuse its trusted consent and opt-out evidence, with a separate purpose |
| Public backlink confirmation | Spec 0045's named lead route and spec 0082's legal and technical controls. Spec 0046 owns ordinary backlink study |
| Source site and platform authority | Specs 0021, 0101 and 0031, and encrypted custody under spec 0118 |
| Audit and removal | Specs 0030 and 0032, with the approved erasure rules below |

There is no export, provider feed, visit report, path-level count,
ranking input, automatic follow-up fetch or link edit. Owner and
visitor choices do not override the referring site's policies.
If a browser supplies an empty referrer or only an origin, show
coverage as unavailable. Do not try to recover an absent path.
See the [W3C policy](https://www.w3.org/TR/referrer-policy/).

## Approval and consent

1. The reviewer approves a signed record for the exact site, purpose,
   allowed source origins and public path families, destination
   scope, literal path filters, consent proof, hosts and proxy logs,
   authority, caps, erasure and one-off verification route. Review
   includes the browser, add-on, queue, runner, UI, backup and audit.
   A change or expiry of that record stops the generation.
2. The owner sees and confirms the exact reviewed scope in the desktop
   app. Name who can see a lead, what external requests can be made
   and what revocation removes. Consent copy says “Help this site find
   public pages that link here. This is optional.” Use a separate
   purpose toggle, default off. Deny absent, stale or unverifiable
   consent. Respect Global Privacy Control, Do Not Track and the
   ordinary site opt-out even with explicit consent. Never prompt
   someone again merely because no referrer was supplied.
3. Stage a signed permit through the service-initiated site link.
   Bind it to site, installation, host boot identity, generation,
   current epoch, issue time and expiry. Reject older epochs, stale
   renewals, unknown clock bounds and wrong identities. Keep an
   authenticated epoch floor outside restorable site and PC data.
   An unknown floor, changed host or expired permit holds collection.
4. Only the owner can grant, pause, revoke and verify. Scripts,
   local MCP, CLI, analytics, normal staff and site guests have
   no route to paths. Check the exact scope before browser capture,
   add-on admission, service commit, review, fetch and display.
   Require current guest access to the landing through the existing
   0021 XenForo or 0031 WordPress authority at those boundaries.
   A matching content revision or unexpired permit cannot prove
   guest access. Hold when the native decision is not current or
   cannot be verified. Use only the existing qualified native evidence
   path. The held review or fetch starts no new evidence lookup.
   Owner approval in the app never permits storing a raw URL.

## Capture, filtering and verification

1. On an eligible guest-visible landing page use a small separate
   browser collection route. Read the browser-provided referrer
   only after current owner and visitor consent. The approved
   consent service provides a one-use page proof bound to the
   local page revision and consent generation. Scope and defence
   work without identity from a browser-provided field.
2. Follow the [URL Standard](https://url.spec.whatwg.org/).
   Parse in memory. Accept HTTPS default port and an exact
   reviewed external origin. Remove the query before building
   any payload. Refuse user information, fragments, controls,
   non-public origin, IP literals, encoded separators, traversal,
   unapproved path families and paths that might name a person
   or carry a secret. Refuse ambiguous decoding. Do not read
   history, fetch from the browser, or weaken referrer policy.
3. Send at most one purpose-bound payload to the verified protected
   origin under spec 0118. Send no referer header. The receiver
   revalidates the scrubbed address, trusted consent proof and
   grant. Browser fields cannot establish consent. Refuse unexpected
   fields. Before enabling, prove that initial landing headers and
   collection data never enter proxy, origin, cache, diagnostic or
   crash persistence. A shared host without this evidence stays off.
4. Keep only one candidate per grant, exact source address and
   landing revision. Use a site-keyed digest for deduplication.
   Do not correlate with visit IDs, human rates, IP addresses,
   cookies or separate host and page totals. Duplicate proof or a
   missing page, consent or source match produces no candidate.
   A path supplied by the browser is untrusted and cannot prove
   a visit, traffic size or a link on its own.
5. The protected owner review names the scrubbed canonical URL,
   landing page and one-off check. Bind approval to that exact lead,
   URL, landing revision and reviewed grant and consent generation.
   Recheck them when the owner confirms and when claiming transport
   under specs 0045 and 0082. If withdrawal, expiry or a changed
   generation wins before the claim, the old preview cannot spend an
   approval or send an address. A later grant cannot adopt the
   old lead or approval. Consume one approval per lead and generation
   before transport. A repeated click or ambiguous transport outcome
   cannot spend another request.
   Include at most one required robots refresh in the preview
   and the admitted host budget. Refuse without current robots and
   opt-out proof. Verify HTTPS and public address admission again.
   No follow-up, redirects, subresources, scheduled retry or second
   worker attempt. A partial or refused read proves no backlink.
6. Confirm only an exact public link to the eligible landing page
   or an admitted same-page alias. Return a protected source address,
   landing reference and date of verification, with no referring
   content, link words or visitor facts. Show “Public link found on
   this date.” Never infer that a particular visitor clicked it.
   No automatic copy to a generic lead list or queued recheck.

## Data and loss of authority

| Record | Allowed fields |
|---|---|
| Grant | site, installation and generation IDs, host boot identity, approved scopes, reviewer and owner digests, issued epoch, expiry and safe outcome |
| Browser input | one-use page proof, canonical external host and scrubbed path, source route generation. Proof binds exact page/site/revision and opt-in |
| Site queue and lead | site, grant epoch, random lead ID, site-keyed dedupe digest, encrypted address, landing ID and revision, coarse UTC day and expiry |
| Verification | lead ID, exact authority and proof digest, outcome code, public link-observation time and expiry. The address stays in the sealed lead store |
| Erasure receipt | site and generation IDs, affected stores, counts, last verified step and safe reason. No path or visitor identifier |

No raw referrer, path-bearing evidence or URL derivative leaves the
protected store except to confirm the exact public page with its own
origin, or show approved data in the owner view. Logs and metrics
contain only opaque IDs, aggregate mechanics and safe reason codes.
Before a fetch, commit an opaque ledger reference and the sealed
address record. If their writes cannot be atomic, stage the address,
verify it, then commit the chain entry; no fetch until both match.
After erasure append a signed outcome that holds no path or reusable
address digest. The chain can still verify the approval and erasure.
Admit this interaction under the qualified privacy review.

Use no per-person history or persistent person key in this feature.
A verified individual withdrawal or erasure conservatively retires
the site's affected collection generation and purges all its leads
and derivative observations. Loss of unrelated leads is disclosed
before enabling. Duplicate age cannot extend a lead's expiry.

Revocation fences PC reads, writes and outputs immediately. Send a
signed higher epoch to the add-on. Show “Stopping on the site”
until its durable floor receipt arrives or all issued permits are
proved expired. Quarantine old queue rows and any in-flight reply.
A native visitor withdrawal stops capture at once and invalidates
its outstanding proof. Recheck local and native epoch at queue, claim,
commit and display. Purge site and PC queues, evidence, private ledger
addresses, caches and backups. Restore applies current tombstones and
epoch floors first. Missing proof keeps the receipt pending. Apply the
same rule on expiry, page withdrawal and site removal. A verified loss
of guest access is a page withdrawal even when the landing content
revision is unchanged. Fence the affected lead, confirmed path and
cached or restored copy before review, fetch or display. Apply the
existing withdrawal cleanup and keep completion pending if its proof
is incomplete. Returning guest access does not revive earlier approval.

## Settings and limits

All numbers here are proposed until measured on Dell.

| Setting or bound | Design value | Takes effect |
|---|---|---|
| `referral_links.enabled` | Off until reviewed owner grant | Revocation fences local work immediately |
| Grant duration and site permit | Grant at most 30 days; site permit at most 60 seconds | Current generation only |
| Lead lifetime | 7 days; no extension on duplicate or verification | From first valid capture |
| Candidate cap | 1,000 live distinct leads per site. Admit at most 100 requests to parsing per site per hour, including duplicates | At admission |
| Verification cap | 10 consumed approvals per site per UTC day and 1 per origin per hour. Each allows 1 page plus 1 required robots refresh, at most 10 seconds and 640 KiB combined. Apply smaller 0082 bounds | Before transport; durable across restart |
| Path cap | 512 UTF-8 bytes after cleaning; one body of at most 1 KiB | Before enqueue |

Proposed footprint for admission: disabled adds zero work, queries,
bytes or requests to ordinary public pages. Enabled still adds no
queries or blocking request to page rendering. For an eligible
consenting landing, allow at most 2 KiB compressed browser code
and one optional asynchronous request at most 2 KiB with headers.
Send no cookies on that request. Test proposed p95 added times of
at most 1 ms for rendering and 10 ms for the collector. Test at most
one queue transaction per accepted payload and one added 512-byte
permit per admitted existing poll. Set queue expiry to the earlier
of expiry or 10 minutes. Measure the full supported platform and
hosting matrix. Hold a profile if its existing polls cannot renew
within the lease and budget. Add no second poller.

## Failure codes

| Code | Plain-English result |
|---|---|
| SUT-3900 | “Approval is needed for this site.” Review or owner grant missing or expired. |
| SUT-3901 | “No usable referring page was supplied.” Refuse missing, ambiguous or unsafe address. |
| SUT-3902 | “This link could not be confirmed.” Verification refused or proof missing. |
| SUT-3903 | “Removal is waiting for this location.” Hold the deletion receipt. |
| SUT-3904 | “This site cannot collect this safely.” Host, privacy, footprint or encryption proof missing. |

Codes SUT-3900 to SUT-3999 are reserved for this spec (DR-108).

## Security and privacy

Referrer paths are untrusted browser input and need a qualified privacy
review, exact owner approval and the visitor's permission. No raw
referrer, path-bearing evidence or URL derivative leaves the protected
store except to confirm the exact public page or to show approved data
in the owner view. Logs and metrics hold only opaque IDs, aggregate
mechanics and safe reason codes. Leads expire after 7 days, and
revocation, withdrawal or expiry purges them. Use invented host names,
page content and consent identities only.

## Requirements

Designed, not run.

## How it is tested

Use invented host names, page content and consent identities. Keep
proof fixtures offline or in an isolated sandbox.

| Requirement | Test and expected outcome |
|---|---|
| REQ-0120-GRANT | IT-0120-GRANT compares missing or expired review, wrong site, owner mismatch, no opt-in, Global Privacy Control and Do Not Track. Each MUST refuse before payload creation or storage. |
| REQ-0120-PARSE | UT-0120-PARSE tests origin only, an unsafe address, query removal before transmission and every length edge. A planted secret MUST reach no receiver, queue, log or error. |
| REQ-0120-LEAD | IT-0120-LEAD tries a stale page, replayed proof, forged consent, spoofed referrer and two separate aggregate sources. Only a permitted direct observation can stage one speculative lead. Nothing here enters ranking. |
| REQ-0120-CHECK | BDD-0120-CHECK gives one authorised public source with a link, and separately robots denial, expired proof, moved target and partial read. Only an exact allowed observation MUST be confirmed. Withdraw or replace the grant after the owner sees the preview, and in a separate case after confirmation but before transport claim. Refuse the old approval and send no address. After withdrawal, a late reply from an already claimed send must not confirm or display a result. Separately revoke native guest access before the claim while keeping the landing revision, consent and owner confirmation fixed. Use 0021 guest permission evidence for XenForo and 0031 IP-18 for WordPress. Refuse that claim without spending the one-off approval or sending a request. Crash between address and chain writes or after transport claim. Deny unsafe replay and prove erased chain continuity. |
| REQ-0120-PRIVACY | PL-0120-PRIVACY plants full referrers at initial landing and collector ingress. Try logs, cache, crash, export and unencrypted storage on each host shape. None may retain the planted bytes. Prove isolated address custody. |
| REQ-0120-ERASE | FI-0120-ERASE races withdrawal with queue/verify commits and a late permit. Disconnect the add-on and restore old floors on each side. Reject wrong identity and stale proof. No stale output or resurrected data. Separately revoke native guest access after exact link verification with the same landing revision, consent and approval. Read the saved result and then try a pre-loss restore on each supported native fixture. No referring path is shown or sent. The affected result follows existing page-withdrawal cleanup. No completion before proof. |
| REQ-0120-PARITY | E2E-0120-PARITY measures render, collector, queue, headers, poll and renewal on all host and desktop variants. Try over-quota bursts and dual discovery through a generic crawler. Unsafe or unsupported profiles MUST stay off. |

Spec 0076 readiness needs the signed privacy review, approval
contract, native evidence, release bounds and test trace. The
design adds an opt-in referring-page lane to feature 106.

### F-0106 verification binding

Feature 106 belongs to 0120. DR-107 permits only the separate
default-off route. 0034 continues host-only ordinary analytics.
DR-108 owns the existing outcome codes. QR-107 requires the
qualified review, host and platform proof before enabling.

| Design and existing requirements | Governing planned cases |
|---|---|
| `DES-0120-REFERRAL-ADMISSION` · REQ-0120-GRANT, REQ-0120-PARSE and REQ-0120-LEAD | IT-0120-GRANT, UT-0120-PARSE and IT-0120-LEAD. Require the current privacy review, exact owner approval, trusted visitor opt-in and signed site permit. Bind the permit to the active site and grant generation. Bind the trusted visitor proof to that site, the exact landing revision and consent generation. Check the external origin and path against the reviewed scope. A missing or stale grant, rejected privacy signal, wrong site, wrong revision or replay cannot create or persist a payload. |
| `DES-0120-REFERRAL-PROOF` · REQ-0120-CHECK | BDD-0120-CHECK. Bind the exact staged URL, owner-confirmed one-off intent, the 0045/0082 read authority and the eligible landing identity. A source page must contain the matching public link or an admitted same-page alias in that same target site's scope. Partial read, mismatched site, wrong thread, changed target, expired proof or denied read leaves the lead unconfirmed. |
| `DES-0120-REFERRAL-CUSTODY` · REQ-0120-PRIVACY, REQ-0120-ERASE and REQ-0120-PARITY | PL-0120-PRIVACY, FI-0120-ERASE and E2E-0120-PARITY. Check scrub-before-write and logs, queue, backup and restore, revoked permits, and the measured platform and host bounds. Unsafe or unproved profiles remain off. |

A browser referrer or a successful fetch without matching link
evidence cannot prove a backlink. An origin-only or absent referrer
shows unavailable coverage. Do not recover a missing path, combine
ordinary analytics totals or retain raw referrer material. A valid
consented observation only stages the scrubbed lead. Only its
approved exact verification can show “Public link found on this
date”. Expiry or withdrawal fences later reads, work and display.

Bind VAR-0120-XF-REFERRAL and VAR-0120-WP-REFERRAL independently
to their admitted native and host identities under E2E-0120-PARITY.
Neither native proof gives permission or coverage to the other.

In the same governing cases, use a valid consent proof from one
fixture site with a decoy landing from another, a matching path on
another host, a stale same-site revision, an origin-only referrer,
and a partial read that resembles a positive result. Check the
existing SUT-3900, SUT-3901 or SUT-3902 path as applicable and the
pending erasure rule under SUT-3903 when proof is unavailable.
Apply safe scope checks before reading or writing address material.
Use invented records offline or in the existing isolated fixture.
No new calls or tests are authorised by this binding.

## Changes from the source

Owner decision DR-107 asks for a separate opt-in route for feature 106,
with a privacy review and explicit owner approval before any referring
path is stored. It replaces reliance on independently verified public
links alone. The design adds an opt-in referring-page lane to feature
106. Failure codes SUT-3900 to SUT-3999 are reserved for it (DR-108).
Footprint figures are proposed for admission and need measurement.
