# 0152: Live backlink crawl profiles and crawler names

Give Leviathan a name, see what a backlink run will read and set
its budget. The crawler records links it sees on permitted public
pages and explains when a page could not be checked. Review the
results in the [Link study](0046-web-link-study.md).

Figures: every numeric limit, threshold and budget in this spec is proposed
until measured on the Dell or settled by a QR.

## Scope and admission

[0045](0045-backlink-crawl.md) owns backlink leads and observations.
[0082](0082-leviathan-crawler.md) owns the legal wall, network
identity and fetcher. This spec owns live approval, owner labels,
per-run controls and limits for the ordinary outside backlink route.
Register each setting once through 0014. Keep the current site
and actor, declared types, bounds and apply classes in the safe
settings registry. Use the existing 0020 owner confirmation and
0014 history. The safe description is metadata only.
[0153](0153-public-web-backlink-index.md) designs a separate index.

DR-154 records what to design. A real run needs admission of its
exact profile under QR-237, the QR-010 legal and QR-025 crawler
dossiers and the relevant host proof. The numbers below are the
qualification profile. Owner approval of a run uses values within
the admitted profile and gives no ability to override a host rule.

0045 uses named leads for the enrolled site. A manual outside check
covers the exact eligible lead and approved redirect scope. Turn on
eligible non-rival background rechecks separately. Each rival check
still needs an unused one-off owner approval for its exact public
URL. Keep that approval separate from any ordinary recurring plan.
Keep protected 0120 leads in their own route and store. A map
or file may supply a lead. Owning the destination or buying a
report supplies no authority to fetch its referring pages.

## Crawler name and public identity

**Crawler name** is a site-scoped `CrawlerDisplayName` under 0014.
Default to “Leviathan”. Decode valid Unicode and reject controls,
format characters, surrogate input and line or paragraph separators
before normalising, including at the ends. Normalise to NFC, trim
Unicode whitespace at both ends, then require 1–80 Unicode scalar
values. Store and compare that canonical value. Display it as plain
text. Apply locally at once. A local name change leaves current run
authority and the separately confirmed card unchanged.

An owner may separately choose **Show my crawler name on its public
identity page**, default off. Preview the exact public fields before
confirming. Verify the HTTPS card on a domain the operator controls.
It gives the actual operator, purpose, opt-out route, canonical
software identity and verification instructions. Publish a
fingerprint only under an expressly admitted card profile. Refuse
any public label that claims another operator, bot or browser.
Local labels are private unless the owner approves publication.
A setting change cannot publish or replace an external card.
Verify the exact approved site, card and public text before using
the card. The admitted card proof binds the exact site, card digest,
key, verification UTC, expiry UTC and label-consent generation.
For qualification, expiry is no more than 86,400 seconds after
verification. Shorter admitted proof bounds win. At or after
expiry, or without trusted time, hold the next send. If the card is
shared across sites without a separate
verified label, hold label use for that site. Off fences the old
labelled card reference. Store the raised withdrawal generation
outside restorable card and settings data. Recheck the floor at
claim and dispatch. An unknown floor or old consent holds. Before
resuming, verify a current card without the withdrawn label.
Hold new requests if the card or
monitored contact loses its required proof.

Every live request uses 0082's canonical fetch identity
`Leviathan/<version>` and the verified card link. The canonical
`Leviathan-Browser/<version>` identity stays with Audit rendering.
Show the saved **Crawler name** and the read-only **HTTP crawler
identity** as separate fields. The name appears in the app and,
when approved, the card. Show “Websites see the Leviathan software
identity when it visits.” The name never changes the User-Agent
product token, robots group,
fingerprint, host waits, stop list or spent approvals. After a public
name change, require the confirmed replacement card before dispatch.
Custom wire product tokens would need a separate change to ADR-0022,
DR-041 and the 0082 verification profile.

Use the 0082 admitted signature profile with request method, target
and identity fields covered. Keep private keys in Tijori. Public
key and contact detail carry no app credential or membership data.
[RFC 9309][rep] defines robots product matching.
[RFC 9421][sig] defines HTTP Message Signatures.

[rep]: https://www.rfc-editor.org/rfc/rfc9309.html
[sig]: https://www.rfc-editor.org/rfc/rfc9421.html

## Live rate and storage profile

The installation has no active outside backlink profile by default.
All endpoints are inclusive. The lower admitted limit in any
applicable profile wins. This table supplies no permission for
own-site Audit, protected 0120 leads or public-web index dispatch.

| Key or fixed bound | Qualification default and allowed values | Scope |
|---|---|---|
| seo.backlink.enabled | Boolean, off. On needs exact desktop-owner confirmation | Site-scoped, owner-only. New ordinary outside dispatch |
| seo.backlink.pages_per_day | 300, from 1 through 1,000 | Page attempts per site per UTC day, including redirects |
| seo.backlink.host_gap_seconds | 30, from 15 through 300 seconds | Adjacent starts per host and shared address |
| seo.backlink.bytes_per_hour | 67,108,864, from 1,048,576 through 268,435,456 bytes | Each of received and decoded bytes per site in the preceding 3,600 seconds |
| seo.backlink.memory_share | 500, from 100 through 1,000 basis points | Share of the exact service slice byte cap, rounded down |
| seo.backlink.store_mib | 512, from 128 through 4,096 MiB | Ordinary queue, receipts, attributable indexes, journals, staging and managed copies |
| seo.backlink.crawler_name | `CrawlerDisplayName`, Leviathan | Site-scoped, owner-only. Live local display |
| seo.backlink.public_label | Boolean, off. On needs separate desktop-owner confirmation | Site-scoped, owner-only outbound label on the verified card |
| Installation request limits | One request in flight per host and shared address, two total, at least one second between all starts | All outside backlink attempts, including policy reads and failed sends |
| Installation accounting | 2,000 attempts per UTC day, 128 MiB per rolling hour per byte counter | Shared across all enrolled sites on the installation |
| Frontier and response | 20,000 pending leads per site, up to three redirects, 15 s per attempt, 512 KiB each received and decoded per page | Apply stricter 0082 type and mode caps |
| Stored evidence | At most 128 safe Unicode scalar values per anchor, optional context under the 0082 ceiling, 30 days for unfinished leads and 90 days for observations | Earlier privacy or rights expiry wins |

All eight settings are site-scoped and `owner_only`. The three
identity and enable controls apply `live` as described above. The five
numeric controls have the registry apply class `live`, with a per-run
limit fence at claim and dispatch. Store `pages_per_day` as an integer
count, `host_gap_seconds` as a duration in whole seconds,
`bytes_per_hour` as a byte size, `memory_share` as integer basis
points and `store_mib` as a size in whole MiB. Their JSON values are
integers in those units. Convert to service units with checked
arithmetic before saving or charging. An increase in allowed work needs
a new approved run even though the desired value is saved immediately.
For all eight controls, require the current owner role for the exact
enrolled site in the protected desktop app.
A staff or MCP token may receive the safe registry description
within its scope. It cannot read the current private value, draft
a change or confirm one. Require the same site and current Cedar
grant on view, preview, confirmation, delivery and history reads.
Keep values and history in the 0014 site store under 0108 and 0118.
Use the existing protected local connection. Logs and safe
diagnostics carry no private label. Mark the public label switch
as `route_out`. The required consent is separate from authorising
outside dispatch. Apply both switches as live fences. On admits
no existing run. Require an exact run approval under the current
qualified profile on a new claim. Off fences the next send even
if a running job holds an older settings snapshot. A settings import,
restore or preset cannot renew either approval.

A MiB is 1,048,576 bytes. Apply the least of the configured memory
share, 256 MiB, the 0082 mode cap and current scheduler grant to the
entire parser, decoder, queue and writer working set. Below 32 MiB
of admitted working memory start no attempt. Use 0082's privacy
scrubber and discard page bodies after bounded extraction.
An unsafe or over-limit anchor is unknown and cannot establish a
different link identity.

The 0082 Receipts disk cap also limits receipt data itself. Check
whole reservation sizes before dispatch. Include temporary managed
compaction copies and each separately retained copy in 0030's file
ledger. An exhausted bound, unproved encryption or unavailable
volume holds the run. Default storage is local. Moves to an
admitted encrypted external volume follow 0038. No cloud move
follows from enabling a crawl.

Reserve each attempt and the maximum admissible receive and decode
bytes before transport. Release unused reservations only on receipt
or proof of no dispatch. Stop at the cap if completion is unproved.
Reject an advertised or
decoded response above the cap. Count bytes actually received under
the 0082 reader limit before termination. Keep incomplete evidence.
Count attempted policy
reads, redirects and failures in the shared online allowance. A policy
read for a lead also needs a possible reservation for its page request.
Robots and other policy bytes also count against the applicable
byte budgets. Preserve the shared-host gate, stop lists, back-off, quiet
hours, server-health and scheduler limits. Keep waits through
restart and UTC midnight. An unproved counter holds. Take the
earliest applicable 0107 or 0019 timeout and the profile bound.
One 0045 heavy job may own these slots under the 0078 scheduler.
The 0082 shared claims apply even if a legacy sharing switch is off.
Use the same host and group clock for requests from other admitted
Leviathan routes. Count only 0045 outside-backlink attempts in the
0152 installation and site budgets. Record a policy read once
under its owning route even if another route later uses the valid
cached policy without a network request.
A profile may lower a ceiling before the next attempt. Higher
limits need new measured admission.

### Reviewing and changing limits

In **Settings → Crawler → Limits and storage**, show desired values,
current qualified ceilings, and the effective limits after narrower
policy and host limits. Show bytes used and reserved separately for
received data, decoded data and managed disk. Include charge windows,
the current storage target, retention, and the earliest known eligible
start. Mark an unknown counter or unavailable volume as held. Hide
specific outside URLs from installation-wide totals.

The owner's exact settings preview binds the site, current settings
revision, admitted profile digest, current verified volume and
placement generation, and requested values.
Reject zero, out-of-range and non-integral values before confirmation.
Show “This limit is outside the verified crawler profile” when the
admitted envelope is narrower. Leave the saved values unchanged.
An owner saves a lower or higher bound by confirming that exact
preview. Reject a stale preview and require a new one. Saving never
starts, resumes or widens a run.

After a tightening, apply it to the next claim and recheck at dispatch.
Raise a host gap against the actual last start and preserve any
later wait already recorded. Apply new lower request and byte caps
to usage already charged or reserved in their current windows. Hold
future claims if the current total is over the new limit. A limit
change never clears counters, claims or waits. A proved sent attempt
keeps its original charge and response deadline unless another fence
stops the read. An unsent reservation rechecks at dispatch and is
released only with proof of no send. An uncertain original send
stays charged and held. A settings change, new preview, volume move
or refresh cannot replay it or reuse a spent one-off permit.
A smaller disk bound holds new writes that need a
reservation. Keep eligible history available and show **Free up
crawler storage**. Apply approved retention or removal through 0032.

A higher maximum or shorter minimum host gap relaxes a limit.
Use it only for a newly confirmed run, after measured profile
admission where needed. Persist a
per-run limit fence. Each tighter revision updates it atomically with
the settings change. For each maximum take the minimum, and for
each minimum host gap take the maximum, of initial approval and
all later fences. Replay stored fences before a later claim or send.
Use the separately approved [0038 storage move][storage-move].
Hold new claims until verified cutover. Reconcile any pending write
only against its original revision and volume. A missing target
holds the run. Retained history follows the verified ledger. The
old copy is inactive after cutover and cleanup has its own approval.
Require fresh run approval on the new placement. A newly approved
run still shares existing waits and charges.

[storage-move]: 0038-search-services-cloud-storage.md

`seo.backlink.enabled` and `seo.backlink.public_label` are in the
never-changeable class (spec 0020 rule 5, spec 0014 `owner_only`; DR-079
for outbound content). The CLI, MCP, scripts and agents cannot change them,
even with a confirmation prompt for an agent; the owner's own desktop
confirmation stays allowed.

## Lifecycle and permission

1. In **Settings → Crawler**, verify identity and storage. In
   **Link study → Check named pages**, show exact site, public leads,
   profile, earliest start, finite expiry, request and byte bounds,
   policy reads and reasons a run may stop. The desktop owner confirms
   the exact plan. Local MCP and scripts can prepare a plan within
   their current site grants. The 0020 preview expiry governs the
   confirmation, and the confirmed expiry governs the run itself.
2. Save a CrawlProfile with site ID, profile ID, revision, authority
   epoch, fixed purpose, route, scope, identity reference, policy
   and settings digests, scheduler digest, volume reference, approval
   time and expiry. Each Run binds run ID, site, the exact profile
   revision, lead manifest digest, start time and state. Attempts
   bind the run, opaque attempt ID, target, current policy expiry,
   host/address reservation, charge window, send state and result.

   A manual confirmation binds one request ID and one exact lead
   manifest. Store its allowed start and immutable end as UTC instants
   under the 0019 authoritative clock. The approved end must be after
   the start and within the exact qualified route's time bounds. Check
   at claim and dispatch. At equality with the end, hold new work.
   A shorter current prerequisite, consent, card, one-off permit or
   scheduler deadline wins. An unknown clock or missing end holds.
   A restart, new preview or renewed proof cannot extend the original
   run. Pause and expiry keep any already-started effect subject to its
   existing reconciliation, custody and release rules.

   For non-rival background rechecks, the owner separately confirms
   one 0019 schedule ID and revision, exact site and qualified profile,
   known non-rival lead manifest, approved redirect scope, cadence,
   time zone, allowed occurrence window and finite end. The preview
   shows each bound. A missing or unqualified entry leaves it off.
   A scheduled occurrence binds that approval and the 0019 occurrence
   key. Commit at most one run with its due subset and the earlier
   of the occurrence window end and approval end. Recheck that cutoff
   at claim and dispatch. The original 0019 run deadline may bring
   it earlier. Skip missed occurrences. A replaced schedule,
   expired or revoked approval, or a lead that is now rival or unproved
   cannot admit the saved occurrence or the affected lead. A newly
   eligible lead outside the approved manifest needs fresh confirmation.

3. Disable → Hold → Ready → Run. Running can pause, finish or
   cancel. A blocked host holds its own leads. Check permissions,
   scope, stop list and resolution before each dispatch. Recheck
   the actual destination against 0082 at the socket. A redirect
   needs fresh target policy and budget proof and cannot widen a
   one-off approval. All network reads use the owning fetcher.
4. Pause, revoke, expiry or cancel blocks the next dispatch and
   invalidates a late result if authority is gone. Cancel an active
   read where possible. Recheck current grants and quota on resume.
   A restored legacy run without profile proof can show eligible
   dated history and cannot resume fetching.
5. On lost reply show “Check outcome unknown”. A rival or protected
   one-off is spent once dispatch may have begun. No automatic
   retry. Use no automatic transport retry. For a proved retryable
   non-rival failure, the scheduler may retry once with a fresh
   attempt after at least 15 minutes and every later required wait.
   A block or unproved permission ends retries for that host. An
   uncertain dispatch stays charged and held under its original
   identity. A fresh approval cannot replay that unresolved check
   or reuse a spent one-off permit. Store an exact observation once.
6. Erasure fences display, scheduling and replay before deleting
   eligible data and managed copies under 0032 and 0118. Apply
   latest stop, authority and deletion fences before any restore.
   Show which prerequisite failed when admission, storage, consent,
   scope or policy cannot be verified. “Crawler not ready” names
   a failed prerequisite before confirmation. “Could not check this
   page” means its host blocked a ready crawler.

## Public address keys

Use one verified HTTPS canonical URL profile. Parse by the pinned URL
reader. Lowercase the punycode host, omit port 443 and fragments,
and keep the exact parsed path and query in the key. Refuse user
information, IP literals, other ports, controls, ambiguous escapes,
and addresses or query fields outside the admitted privacy profile.
Never delete a query field and then fetch or claim a different page.
Compare same-page aliases only after the owning scope verifies them.
A canonical hint on another page gives no approval or equivalence.
Cap the encoded address at 2,048 UTF-8 bytes before the frontier.

## Acceptance and formal verification

| Requirement | Planned checks |
|---|---|
| REQ-0152-IDENTITY | UT-0152-DISPLAY-NAME and IT-0152-NAME below. Keep the same robots and stop decision across name changes. Refuse a changed fingerprint and lost identity proof. |
| REQ-0152-LIMITS | IT-0152-LIMITS admits the 300th eligible page attempt and refuses the 301st. Race two hosts sharing one address, keep a longer 429 wait over midnight, test 2,000 and 2,001 installation attempts and two versus three simultaneous slots. Boundary-test site and installation received and decoded byte counters separately, a 15-second deadline, full ledger, lost mount and low memory. |
| REQ-0152-APPROVAL | BDD-0152-PUBLIC-LABEL and E2E-0152-APPROVAL. Start off. Deny MCP confirmation, contact alone and changed preview bytes. At the confirmed expiry and after restart, hold new dispatch. Put a scheduled claim just before expiry with its first transport slot at equality. Send nothing and start no substitute occurrence. Reconcile only earlier effects. A lost rival reply spends its permit. For non-rival rechecks, prove one run for the original authorised occurrence and none for a missed window, replaced schedule or expired approval. A lead reclassified as rival cannot be selected by a saved occurrence. A restored or revoked profile cannot dispatch. |
| REQ-0152-SCOPE | IT-0152-URL-SCOPE gives a checked lead for site A and a foreign site B report. Deny the foreign lead, a cross-origin redirect outside the plan and private final resolution before sending to them. Two admitted queries on the same path remain distinct. Reject ambiguous escaping. An eligible complete read without the exact target link supplies no seen result. |
| REQ-0152-PRIVACY | SEC-0152-RECORD scrubs synthetic material, withholds unsafe text, and tests same-site storage, erasure and restore. Prove zero extra public page queries, processing or bytes in both native add-ons. |

- `UT-0152-DISPLAY-NAME` checks empty, one, 80 and 81 scalars,
  equivalent NFC input, surrounding whitespace and a newline at
  either end. Refuse the invalid strings without echoing them.
- `IT-0152-NAME` renders “Rana’s map scout” and markup-like text
  as plain text. Reuse 0014 AT-CF41 for site A and B, foreign-site
  read, staff, revoked grant and MCP token. Inspect encrypted
  values and history, safe metadata and protected local delivery.
  Refuse a public impersonation claim and a card for another site.
- `IT-0152-SHARED-WAIT` gives an eligible Audit request and a named
  outside backlink request different hostnames on one admitted group.
  Race the two routes at one instant, then stage a redirect and a
  policy read under the losing route. The first claim serialises
  subsequent starts. First prove the faster Audit route honours the
  original backlink gap without a Retry-After. Then persist a longer
  Retry-After and try again after restart under the Audit route. Refuse one
  tick early and allow equality if scope is still valid. Re-resolve
  the losing target to a new group and carry its host wait. Lose the
  first reply and prove closure separately. Only the original route
  is charged, the wait survives and the unknown attempt cannot retry.
  If closure is unproved, hold both occupied keys. The other site
  receives only its own safe hold reason.
- `IT-0152-LIMIT-CHANGE` tightens the live profile after an attempt
  is reserved and before dispatch, then again after a request has
  started. Check the dispatch fence, original in-flight charge, actual
  last start, later host wait, full disk and retained history. Show
  the later wait to a second site sharing the address. Shorten the
  gap and raise a
  maximum towards its initial value while the tightened older run
  waits. Keep its tighter bound and charged usage,
  refuse a stale preview and use the larger bound only after fresh
  admission and confirmation. Move to a newly verified volume while
  a write result is pending. Reconcile only on the original
  placement, then refuse new work
  under its approval. A retained old copy is inactive and cannot be
  cleaned up by the settings approval. Recheck after restart.
- `IT-0152-LIMIT-SETTINGS` compares the safe registry description to
  all five typed limits. In a synthetic profile admitting the whole
  range, accept each inclusive endpoint. Refuse one below and above
  each range, text and fractional values. Refuse an otherwise valid
  value beyond a narrower profile with no settings revision or fence.
  Verify the exact seconds, bytes and basis points that reach the
  scheduler, including the MiB conversion and memory rounding. Give
  two sites distinct settings. A foreign site, staff user and MCP
  token can see only safe descriptions. A denied or stale change
  commits no revision or fence. Within an active run, saving a
  relaxation changes its desired value only. A confirmed tightening
  updates the run fence in the same settings transaction. Delay the
  snapshot delivery after commit. Deny an attempt above the current
  durable fence even if the claimed job still sees the old limit.
  If the fence cannot be read, hold the attempt.
- `BDD-0152-PUBLIC-LABEL`. Given a verified site's owner and the
  private name with both controls off, when the owner saves a
  local rename, only the local display changes. When the owner
  separately confirms the public preview and the exact approved
  card is verified, that card can show the confirmed label. It
  enables no crawl. A separately enabled crawl retains the fixed
  wire token. Switch the public label off and try to send with
  the old labelled card. Restore the old On setting, verified
  card and its previously granted consent. The current floor
  keeps it held. Verify the unlabelled card and test immediately
  before its expiry and at equality. At equality hold the next
  send. Switching the crawl off fences its next send.

Quint models Disabled, Held, Ready, Claimed, Sent, Resolved and
Unknown. Prove current authority at claim, capped site and shared-host
accounting, one-off consumption, monotone revocation and name changes
that preserve wire identity. Replay the traces against the Rust
scheduler and fetch adapter in the 0116 synthetic mini-internet.
Use the [0116 Linux and Windows matrix][crawler-matrix], LS-38,
with the supported x86-64 preflight of [0030][crawler-platforms].
Qualify each exact Bazel build separately on Ubuntu 26.04 LTS,
Debian 13 and Windows 11 at the required 0078 memory profiles.
Match pass or fail and normalised output hashes. Use each
platform's own resource baseline. Record the source digest,
variant, benchmark and expected boundary outcomes under QR-237.
Missing required proof holds the profile. No test uses a real
site, visitor, crawler contact or provider.

[crawler-matrix]: 0116-leviathan-scale-storage-and-verification.md
[crawler-platforms]: 0030-install-upgrade-removal.md
