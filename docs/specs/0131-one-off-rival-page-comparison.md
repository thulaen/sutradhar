# 0131: One-off rival page comparison

An owner can compare one saved draft with one named public page.
The owner sees the exact address and confirms a single read.
The Coach then shows similarities in headings and where the
comparison is incomplete. It uses literal headings from the initial
reply, whose on-screen visibility is unknown. An editor can use
the comparison to decide which topics to explain in their own words.

DR-115 gives this route sole ownership of feature 238. Spec 0117
owns the separate fan-out rehearsal and its stored evidence. This
one-off comparison does no search, rival discovery, tracking,
general crawling, rewriting, automatic publishing or live link
ranking. It adds no time, queries or bytes to public-page responses.

Example: Meera has a saved draft, "Winter boiler servicing", and
wants to see how one rival's page is headed. She opens the draft in
the Coach, enters one HTTPS address with no query and chooses
"Review one visit". The preview shows the exact origin and path and
expires after five minutes. She confirms with the focused button.
One robots check and one page read follow, with no redirect and no
draft content sent. The Coach lists heading pairs that share at
least two words, the headings left unpaired on each side and the
evidence limits. It says nothing about the rest of the rival's
page. After 15 minutes, or when she presses "Clear comparison", the
view is erased. A second plan for the same page is refused during
the repeat suppression window.

## What it does

### Words used here

- **Plan**: the immutable, owner-confirmed record of one exact
  address, draft revision and set of limits.
- **Slot**: the single robots or page request a plan may claim, once.
- **Heading**: the title, H2 or H3 text of the initial response.
- **Paired**: a draft heading and a source heading that share enough
  words under the match rule.
- **Evidence limits**: what could not be read or proved, with a reason.
- **Spent target**: an address whose slot was claimed. It cannot be
  read again within the repeat suppression window.

### What this spec designs

| Feature | What the owner sees |
|---|---|
| Review and confirm | An exact address preview and one confirmed read |
| One public read | One robots check and one page GET under the 0082 wall |
| Local comparison | Paired and unpaired headings with evidence limits |
| Clear and expiry | A view that is erased on close, clear or after 15 minutes |

## Authority and layout

Open a saved, guest-visible-intended draft in the desktop Coach.
Use spec 0042's saved draft eligibility. Resolve the exact revision
through spec 0052's content store or the paired site's signed
read contract in 0021. Require positive evidence of the declared
guest-visible destination, with no conversation draft, private
preview or protected or unreadable portion. Missing or mixed
eligibility withholds the comparison. On revision, visibility,
session or grant change, stop further work and discard the view.
Only the current site owner with live draft and evidence grants
can initiate the read or retrieve the result.

```text
Compare with a public page
Saved draft      [title and revision]
Public page      [exact HTTPS address]
[Review one visit]
This sends the address to the public site. Your draft stays here.
[Read once] [Cancel]

Comparison       [draft version] [source and time]
Similar words    [heading pairs and exact references]
In your draft    [headings left unpaired]
In the sample    [headings left unpaired]
Evidence limits  [scope, omissions and reason]
[Clear comparison]
```

Use spec 0020's action runner with a dedicated app-only owner
capability. Show the exact origin and path in an immutable preview.
The preview expires five minutes from creation on the lifecycle
clock. Confirmation consumes that preview. Dispatch must start
before the earlier of preview expiry and the end of the owner's
authenticated session. Bind the exact route, URL, site, saved
revision, owner session and live grant generations to the plan.
An authorised owner can confirm with the focused button by
keyboard or pointer. No shortcut, CLI, MCP, token, script or
site add-on can confirm. Audit the opaque plan and actor IDs and
the outcome. Seal the exact address in the purpose-specific
ledger entry. The public request carries no draft content.

## Records and public read

| Record | Fields |
|---|---|
| Review plan | Version, opaque plan and actor IDs, site ID, saved draft ID and revision, current grant and policy generations, sealed exact target, canonical origin, preview digest, creation/confirmation and expiry, owner session, request kind and limits |
| Spend claim | Plan ID, keyed private site-scoped target identity, route ID, actor and scope, state approved/spent/refused, expiration and budget buckets. Bind previously known aliases to one identity. Budget and target claim share one durable transaction |
| Dispatch slot | Plan ID, robots or page purpose, sealed exact address reference, pre-send compliance ID, status unused/claimed/finished and opaque post-response outcome. Atomic claims are one-way |
| Live source evidence | Opaque evidence ID, final allowed address reference, read time, complete flag, crawler policy and compliance receipt IDs, scrubber/parser versions, eligible headings in source order and bounded evidence ranges. All source content and references to text are transient |
| Comparison row | Result generation, draft and source heading references as applicable, matching term count and total distinct term count, status paired/draft_heading/sample_heading, reason code where evidence is insufficient. A paired row binds both exact versions |

Accept one canonical absolute HTTPS URL within 2,048 UTF-8
bytes. Refuse credentials, query, fragment, IP literal, nondefault
port or an ambiguous path. The shared 0082 URL, DNS, address and
socket checks are authoritative. Refuse destinations on the active
site or another registered own-site origin. Use the optional rival
register only as an input list. Registry membership is no grant.

Reuse the 0082 Fetcher with the admitted Receipts profile.
Before transport, atomically consume the confirmed plan and
reserve the site and shared per-origin budgets, once per target.
A plan can have one `robots` slot and one `page` slot. Commit the
sealed address, pre-send compliance entry and irreversible slot
claim before opening that socket. The robots entry records the
control check purpose. Its response can update only the legal
wall cache under the crawler’s protected retention rules. The
page entry binds the current robots decision and other legal
evidence. Recheck the epochs, URL, approval, ownership and
cancellation at each dispatch.
A missing or uncertain commit or any crash stops the plan for good.
A completed request appends its result separately. No recovery path
retries a claimed slot or advances to another slot after restart.

Use one page GET. The optional robots check must complete and
allow access before the page slot is claimed. Reuse current
verified evidence for other prerequisites. When that evidence is
missing, stop without additional fetches. Apply the outside-host
gap and shared host limits. Each GET uses the declared identity
and installation signature under 0082. Verify the certificate
and the resolved socket destination against the exact authorised
origin on each request. Follow no redirect. Send no credential,
cookie, referrer or prefetch. Use verified public HTTPS on both
GETs. A public origin needs no pairing or application envelope.
Use spec 0118 protected local channels for service, app and
worker traffic and its sealed store for all durable records.
No insecure fallback, remote connector or analytics beacon.

Apply the 0082 guest-only legal wall and privacy gate, including
robots, stop list, text-mining, snippet and archive directives,
region and disallowed content decisions. A pause, pending legal
decision, unsupported scope or missing compliance receipt gives
no dispatch. After a reply, reject blocked, login, unsupported,
truncated, opted-out or unsafely scrubbed pages. A blocked or
failed attempt has no automatic retry. An owner cannot reuse a
spent target through a fresh plan, alias, worker restart or reuse
of its prior reply during the admitted repeat suppression window.
The key is private and scoped to the active site and this route.
Keep 0082's existing host controls across sites. A different page
needs its own approval and admission. Spent or erased plan IDs
cannot be revived. On target erasure, remove its address and
deduplication key. Keep a path-free pause for this whole site's
comparison route for the remainder of the affected window. A
current origin stop or privacy hold still denies every relevant
request under 0082. A restore missing the live enforcement epoch
or the route pause state denies new approvals and dispatch.

Use only the bounded raw scan already admitted by 0082 and 0115.
For HTML, consume only the existing raw facts interface for the
literal title, H2 and H3. Accept facts only if its admitted profile
excludes scripts, styles, templates, comments, hidden-marked and
reserved context, without opening a new reader. Attribute facts
to initial-response text with unknown on-screen visibility. For
plain UTF-8 text use the first non-empty line as its only heading.
Never build a page tree, render, run scripts, load CSS, fetch
subresources or follow links. Release no facts before the whole
response passes the scrubber and legal-signal checks. If raw
heading fields or complete scope cannot be proved, return
“This page could not be compared from the public text we can read.”

## Compare locally

Use H2 and H3 text and the title from the saved draft.
Exclude unchecked scopes before any matching. Keep source order
and the first 24 eligible headings per side, each truncated at a
grapheme boundary to at most 200 Unicode scalar values. Mark
truncation. If either side has no eligible heading, produce an
insufficient result and no missing-topic classification.

Use the same admitted British English token split and literal
match key from spec 0026 for both. Keep a set of distinct
tokens with at least one letter, excluding the admitted profile's
versioned stop words. Omit empty sets. A match needs at least two
shared tokens and intersection divided by union of at least one
half. Order candidate pairs by that fraction descending, shared
terms descending, then each side's source ordinal ascending.
Take each pair in order only if neither heading is already paired.
Show remaining headings in original order. Keep draft-only and
sample-only distinct. Strip or escape bidirectional and control characters
(proposed rule) before showing any rival heading, and show it as plain
text only. Say “Headings with similar words” and
“These are headings from the part we could read”. A mismatch
cannot assert that the full page omitted a topic. There is no
generated text or comparison to a search engine.

Pin the draft and source evidence throughout. Before the preview,
transport, result commit and delivery, resolve live site, actor,
source, legal and policy authority. Missing or changed evidence
withholds the result. If a new owner request or selection changes
the generation, online work can finish only within the original
transport fence. Suppress stale output, dispose of the bytes and
retain the spent claim. No old result is silently relabelled.

## Protection and limits

Keep raw replies, normalised headings, draft passages and derived
rows in admitted protected memory. Never feed them into a model
or extension, persist them, expose them through MCP, or send them
to another service, history, export or support bundle. Remove
them on close, clear, pause, cancellation, revocation, erasure,
restart or after 15 minutes. A clear action changes the result
generation and cancels pending work. Recheck generation, scope
and authority before painting an async result. Specs 0117, 0092
and other Coach routes cannot read this evidence. Keep the
source draft unchanged.

Seal exact address and spend evidence in the file ledger under
0118. Use a content-free compliance chain with erasable sealed
address reference as in 0082 and 0120. An erasure removes both
address and reusable identity tags and records an opaque outcome.
Suppress restored results and claims with missing erasure state.
Expose no path, query, excerpt or draft terms to telemetry,
common errors or technical logs. The review and comparison
only show those fields to the admitted owner.

| Fence | Maximum before applying any lower shared limit |
|---|---|
| Page and control | One page and one robots resource. No retry or redirect. Owner may spend three plans per site per UTC day and one per origin per hour |
| Fetch | 512 KiB decoded page, 500 KiB decoded robots, 15 seconds total after claim. Cap before decoding output enters the raw scan |
| Local work | 24 headings per side, 200 scalar values per heading and 576 pair comparisons. Give the comparison stage a 16 MiB memory lease and two seconds, within 0078 |
| State | Approval five minutes, view 15 minutes. A signed 0082 policy under QR-126 sets the repeat suppression and spent-claim retention intervals. Closed results cannot be reopened |

Use existing common errors and the corresponding 0082 policy,
transport or bounds reason. Show “This visit is no longer
approved” on expiry or stale plan, “The public site did not allow
this visit” on legal refusal, or “The comparison was cancelled”
on pause or cancellation. Display a coverage reason for refusal
without showing third-party text. If any limit, protection or
Receipts profile is unadmitted, show “This comparison is
unavailable” and make no request. QR-126 owns admission evidence.

## Data

The records are the review plan, spend claim, dispatch slot, live
source evidence and comparison row, in the table under "Records and
public read". The plan, the sealed address and the spend evidence
are durable, sealed in the file ledger under 0118. Raw replies,
headings, draft passages and derived rows are transient, held in
protected memory only and never stored, exported or sent on.

## Settings

The maxima below are the fences of "Protection and limits". A lower
shared 0082 or 0078 limit applies first. No setting raises a
maximum, and the route cannot be switched on if any limit,
protection or Receipts profile is unadmitted.

All numbers here are proposed until measured on Dell.

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `rival.compare_enabled` | on or off, owner only | off (proposed) | two values | next plan |
| `rival.plans_per_site_day` | whole number | 3 | 0 to 3 | next plan |
| `rival.plans_per_origin_hour` | whole number | 1 | 0 to 1 | next plan |
| `rival.preview_expiry_min` | minutes | 5 | 1 to 5 | next plan |
| `rival.view_expiry_min` | minutes | 15 | 1 to 15 | next result |
| `rival.page_max_kib` | KiB decoded | 512 | 1 to 512 | next plan |
| `rival.robots_max_kib` | KiB decoded | 500 | 1 to 500 | next plan |
| `rival.fetch_deadline_s` | seconds after claim | 15 | 1 to 15 | next plan |
| `rival.headings_per_side` | whole number | 24 | 1 to 24 | next comparison |
| `rival.heading_max_chars` | scalar values | 200 | 1 to 200 | next comparison |
| `rival.match_min_terms` | whole number | 2 | fixed | not settable |
| `rival.match_min_ratio` | ratio | 0.5 | fixed | not settable |

`rival.compare_enabled` is owner only. The CLI, MCP and agents cannot
change it.

The repeat suppression and spent-claim retention intervals come from
the signed 0082 policy under QR-126 and are not settings here. The
comparison stage holds a 16 MiB lease and two seconds, within 0078.

## Failure codes

SUT-6620 comparison unavailable (limit, protection or Receipts
profile unadmitted); SUT-6621 plan expired or stale (shown as "This
visit is no longer approved"); SUT-6622 target refused (URL, DNS,
address, own-site or repeat suppression); SUT-6623 legal wall
refusal (shown as "The public site did not allow this visit");
SUT-6624 slot claim or ledger write failed, plan stopped; SUT-6625
page rejected after reply (blocked, login, unsupported, truncated,
opted-out or unsafely scrubbed); SUT-6626 raw heading fields or
scope unproved ("This page could not be compared from the public
text we can read"); SUT-6627 comparison cancelled by pause,
clear, edit, revocation or erasure (shown as "The comparison was
cancelled"); SUT-6628 stale result suppressed; SUT-6629 restore
missing enforcement or pause state, approvals denied. Codes
SUT-6620 to SUT-6639 are reserved for this spec (DR-130, under DR-064).

## Security and privacy

- The public request carries no draft content, credential, cookie,
  referrer or prefetch. It follows no redirect and has no retry.
- Only the current site owner can confirm, by the focused button.
  No shortcut, CLI, MCP, token, script or site add-on can.
- The exact address is sealed and erasable. Erasure removes the
  address and reusable identity tags, and suppresses restored claims.
- Common errors and logs hold no path, query, excerpt or draft term,
  and any telemetry the owner opts in to carries none either.
- Rival headings are shown as plain text with bidirectional and
  control characters stripped or escaped (proposed rule).
- No model, extension, MCP resource, export or support bundle reads
  the evidence. Specs 0117 and 0092 cannot read it.

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| REQ-0131-APPROVE | A plan MUST be confirmed by the current owner through the accessible confirm for a saved eligible revision, expire at once on its limit, and never repeat a request after a crash, alias or replay. No other route can approve. | IT-0131-APPROVE |
| REQ-0131-WALL | Every GET MUST pass the 0082 legal wall and use the declared signed identity, with no redirect, credential, cookie, subresource or retry, and no page GET after a prerequisite refusal or uncertainty. | IT-0131-WALL |
| REQ-0131-MATCH | Matching MUST use two shared terms and a one-half ratio, give deterministic rows with explicit incomplete evidence, never claim CSS visibility, and never generate or apply an edit. | UT-0131-MATCH, BDD-0131-MATCH |
| REQ-0131-LIFE | Pause, cancel, clear, edit, revocation or erasure MUST leave no stale result, reusable erase tag, durable text or duplicate fetch. | E2E-0131-LIFE |
| REQ-0131-FOOTPRINT | The route MUST add no public-page footprint, site write, feed, monitoring job or outside connector activity on all version 1 platforms. | E2E-0131-FOOTPRINT |
| REQ-0131-SETTINGS | The settings and fences MUST be enforced as stated and a larger value refused. | UT-0131-SETTINGS |
| REQ-0131-FAIL | Each refusal MUST carry its named code and message and no third-party text, path or draft term. | UT-0131-FAIL |

## F-0238 design and verification binding

DR-115 and the 0041 handoff give this feature one owner. The only
actor who can start the exposed action is the active site's owner
with a current 0042 saved-draft and evidence grant. A successful
read does not create a draft, a ranking signal, a stored excerpt
or an input for a second tool.

| Owning design | Existing requirement and verification |
|---|---|
| DES-0131-OWNER-PLAN | REQ-0131-APPROVE, REQ-0131-SETTINGS and IT-0131-APPROVE, UT-0131-SETTINGS. Bind the exact target, actor, grants, expiry and draft revision before claiming a slot. Test change to each separately. |
| DES-0131-SINGLE-READ | REQ-0131-WALL and IT-0131-WALL. FX-0131-ONE-SIMULATED-READ uses separate fake origins and fresh approvals for the HTML and plain-text variants. Each plan proves pre-send admission, independent slots, repeated-target refusal and zero retry on an uncertain effect. |
| DES-0131-LOCAL-MATCH | REQ-0131-MATCH, UT-0131-MATCH and BDD-0131-MATCH. Bind the initial response and exact draft revision, and check ordered matching and an explicit lack of visibility evidence. |
| DES-0131-PROTECTED-LIFE | REQ-0131-LIFE, REQ-0131-FOOTPRINT, REQ-0131-FAIL and E2E-0131-LIFE, E2E-0131-FOOTPRINT, UT-0131-FAIL. Bind result expiry, withdrawal, one-use claims, zero page effects and retained safe rejection to named receipts. |

| Variant | Predicate and test boundary |
|---|---|
| VAR-0131-RAW-HTML | The admitted 0082 and 0115 raw profile proves complete literal title, H2 and H3 facts after the whole initial HTML reply passes legal and scrub checks. IT-0131-WALL and BDD-0131-MATCH refuse truncated or unsupported facts and preserve unknown on-screen visibility. |
| VAR-0131-PLAIN-TEXT | The admitted Receipts profile supplies a complete plain UTF-8 response and the first non-empty line. An empty or invalid response cannot supply a paired heading. UT-0131-MATCH and BDD-0131-MATCH verify this single-line predicate and the same scrub and custody rules. |
| Refusal fixture | Any unadmitted source, protected work profile or missing eligibility proof holds. UT-0131-FAIL verifies the precise code and safe response. The branch makes no new request to fill an evidence gap. |

Apply each supported response variant to the exact Ubuntu 26.04,
Debian 13 and Windows 11 x86-64 profile in E2E-0131-FOOTPRINT.
Reuse the 0042 and 0052 saved-draft path or the admitted 0021
native path for XenForo and WordPress, with the same grant. The
platform and draft path identities cannot supply missing proof
for either response variant.

UI-0131-REVIEW binds the immutable preview and result screens.
API-0131-OWNER-RUNNER binds the app-only 0020 route and
API-0131-RECEIPTS the internal 0082 raw-facts handoff. Keep both
identities distinct from their pending typed schema and size
qualification. DB-0131-CLAIMS binds the atomic spend and slot
records under forced site scope and the 0030 ledger. It stores
no response body or heading. ERR-0131-SOURCE-UNPROVED binds
SUT-6626 and UT-0131-FAIL. If the raw heading fields or complete
scope are unproved, return the safe existing text and no result. The
claimed slot stays spent and cannot be retried.

QR-126 owns the admitted raw profiles, source rights and
compliance receipts, canonical target identity, repeat window
and exact typed interfaces. Its platform, role and implementation
evidence remain required under spec 0076. Use the existing
case identities above with synthetic inputs.

## How it is tested

Designed, not run. Use synthetic drafts, simulated origins and test
DNS, response and policy fixtures. Implement no live rival requests
in tests.

- **IT-0131-APPROVE:** the owner reviews the exact target for a
  saved eligible revision. Refuse wrong actor or site, private or
  mixed-visibility draft, changed native visibility, expired plan
  and altered scope. Concurrent cross-site attempts, exact preview
  expiry, crashes before and after either slot claim or send, alias
  and replay never repeat a request. The owner uses the accessible
  confirm. Any other route cannot approve.
- **IT-0131-WALL:** exercise denial by stop list, robots, ambiguous
  opt-out, private or changed DNS destination, independent TLS
  failures on page and robots, protected response, redirect,
  missing receipt, scrub failure and both byte caps. Revoke between
  robots and page and fail the pre-send or response ledger write.
  Zero page GET after a prerequisite refusal and zero continuation
  on uncertainty. Each permitted GET uses the declared signed
  crawler identity. No redirects, credentials, cookies, user body,
  subresources or retries.
- **UT-0131-MATCH and BDD-0131-MATCH:** exact two-term and one-half
  boundaries, ties, empty sets, capped Unicode, bidirectional and
  control-character stripping, same-word headings,
  unsupported markup, templates, hidden attributes, CSS hidden
  text, a script-only shell, a truncated response and
  script-injected instructions. For a complete plain UTF-8 reply
  with leading blank lines and two non-empty lines, use the
  first non-empty line only. Empty or invalid text supplies no
  heading. The view never claims CSS
  visibility. Show deterministic paired rows and explicit
  incomplete evidence. No generated or applied edit.
- **E2E-0131-LIFE:** pause, cancel, clear, edit the draft, deny the
  grant, revoke the source or erase the target before transport,
  response or display. Attempt cross-route evidence reuse. Restore
  before and after erasure with a missing live epoch. Check
  same-site pre-expiry refusal and independent cross-site grant
  and host limits. No stale result, reusable erase tag, text in
  durable state or duplicate fetch.
- **E2E-0131-FOOTPRINT:** on Ubuntu 26.04, Debian 13 and Windows 11
  under Bazel, qualify the admitted raw profile and tight
  scheduler caps. Compare public-page footprint and link ranking
  before and after. No site write, feed, monitoring job or outside
  connector activity.
- **UT-0131-SETTINGS:** one value just inside and one just outside
  each range, each default, and refusal of a value above a fence.
- **UT-0131-FAIL:** trigger each named refusal and check its code,
  its message and that no third-party text, path or draft term is
  shown or logged.

## Changes from the source

Under DR-115, feature 238 is one owner-confirmed read of one
named page, with no search, tracking, crawling, rewriting or
ranking, and it does not widen the spec 0117 rehearsal.
Comparison uses only the literal headings of the initial reply,
so it cannot say that a topic is missing from the full page.
Repeat suppression and claim retention intervals
wait for the signed policy under QR-126.
