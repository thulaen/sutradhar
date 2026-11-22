# 0132: DataForSEO keyword research

The Editorial Coach works offline by default. An owner can request
keyword estimates for selected terms or expand one seed, after
seeing the exact terms, market and maximum estimated charge.
Saved evidence can help group topics while the writer edits.

DR-117 permits these two DataForSEO keyword routes. There is no
file import into this intake, live typing request, draft-save
request, scheduled keyword refresh or ranking input. DR-085's
existing supplied-file research remains separate. DR-118 gives
the wider live evidence routes in
[spec 0133](0133-dataforseo-live-coach-evidence.md) their own
contract and uses this
same connector, evidence store, spend authority and vault.
DR-134's separately approved Local Search Preview in
[spec 0143](0143-audit-grid-findings-and-local-preview.md) also
shares these components. Reserve and reconcile spend across all
admitted purposes. Keep request plans and result grants distinct.

Example: Priya types three short phrases into "Selected terms" and
chooses "Review lookup". The preview shows the exact normalised
terms, the market "United Kingdom, English" and a maximum charge of
USD $0.01236 (12,000 + 120 x 3 microdollars), with her remaining
budget. She presses "Approve this lookup" in the app within five
minutes. One POST goes to `api.dataforseo.com` with no retry. The
reply is validated and saved as a snapshot labelled "DataForSEO
keyword estimate" with its fetch date. A later same-key request
within 30 days reuses it and says nothing is charged. At 30 days
the app says "Saved data may be out of date". A local MCP agent can
only prepare a plan. It cannot approve or send.

## What it does

### Words used here

- **Lookup plan**: the prepared, exact request with its maximum
  charge, waiting for the owner to approve.
- **Snapshot**: an immutable saved reply, tied to its request key.
- **Hold**: an amount reserved against the caps before sending.
- **Microdollar**: one millionth of a US dollar, the unit of the
  tariff arithmetic.
- **Sandbox**: the provider's free test host, used only for the
  fixed synthetic test.
- **Evidence selection**: the exact snapshot rows an owner links to
  a brief.

### What this spec designs

| Feature | What the owner or program sees |
|---|---|
| Selected terms | Estimates for 1 to 20 reviewed terms |
| Expand one seed | Up to 50 suggested phrases for one reviewed seed |
| Approval and spend | An exact preview, caps, holds and settlement |
| Saved evidence | Cached snapshots with separate freshness labels |
| Local MCP actions | Prepare, status, results and brief, never approve |

## Boundaries and endpoint contract

Rust owns the allow-listed HTTP client, parsing, price manifests,
approval and spend ledger. Jobs use spec 0019 and its pause gate.
Tauri and the Sutradhar Design System own the UI. Specs 0020 and
0118 own permission and protected local transport. Tijori owns
the API login and password. The site-scoped store uses forced
row-level security under spec 0108. No script, add-on or UI code
holds network or vault authority.

The [Overview][overview] and [Suggestions][suggestions] documents
and [Labs Google tariff][price] were checked on 20 November 2026.
Both requests are POST to the fixed verified HTTPS origin
`api.dataforseo.com`, with one UTF-8 JSON task inside an array.

| Operation | Path and fields | Sutradhar bounds |
|---|---|---|
| Selected terms | `/v3/dataforseo_labs/google/keyword_overview/live`. `keywords`, `location_code`, `language_code`, `include_serp_info=false`, `include_clickstream_data=false` | 1–20 distinct terms. Provider permits up to 700, with up to 80 characters and 10 words each |
| Expand a seed | `/v3/dataforseo_labs/google/keyword_suggestions/live`. `keyword`, same targeting and false flags, `include_seed_keyword=false`, `exact_match=false`, `limit=50` | One seed and at most 50 returned items. Provider limit is 1,000. No pagination, offsets, filters, add-ons or user-supplied flags |

Both endpoints use Google, the admitted United Kingdom code
`location_code=2826` and `language_code="en"`. Show the target.
There is no device parameter. DR-118 owns other market controls.
Both endpoint contracts document at most 10,000 calls a minute
and 30 simultaneous calls, one task per call. The connector
allows at most one keyword call in flight per billing account
and six keyword dispatches in any rolling minute. Four active calls
per installation are shared with spec 0133, and both limits apply.
Count through restart. A shared limiter also accounts for any wider authorised
route using the same credentials. Use no automatic POST retry,
redirect, proxy credential forwarding or queue API for Labs.

Treat text as a short independently entered search phrase. Trim,
collapse ASCII spacing, NFC-normalise and lowercase using the
admitted 0026 literal match key. Preview the actual sent form.
Require at most 80 scalar values, 320 UTF-8 bytes and ten words
after normalisation. Reject controls, private identifiers,
addresses, URLs, contact details, secrets, long passages and
draft, forum, member or raw connector-text provenance. Owner
entries and agent proposals use dedicated plain keyword fields.
A client source label cannot bless a draft or scoped content
reference. An agent proposal is untrusted until the owner reviews
the exact terms as independent public search phrases. A denied or
unproved source, unknown sensitive content or failed privacy
check refuses the term. The request accepts no editor buffer,
finding, citation, note, log or job attachment.
No site ID, domain, tag, account name or draft context goes in
the body. The vault credential is confined to the approved
provider's authentication header. See [Basic authentication][auth].

## Data and freshness

| Record | Contract |
|---|---|
| Provider binding | Site, opaque billing account key, Tijori credential generation, enabled bit, approved scopes, disclosure policy and tariff profile. One billing binding per installation, retained across credential rotation. Site grants reuse it without duplicating balances. Secrets are never returned |
| Lookup plan | Site, prepared by, exact request type and canonical terms, market, response limit, request digest, policy, grant and credential generations, tariff version, maximum charge in integer microdollars, preview ID, expiry and refresh or cache intent |
| Accounting fence | Private installation billing binding and epoch, day/month counters and opaque outstanding holds. No terms, result digests or other-site membership in site projections |
| Spend entry | Site, billing key, attempt and plan IDs, request digest, approved maximum, tariff version, UTC period keys, status, claimed send time, provider task ID if verified, actual cost or unknown, reconciliation evidence ID and route reason |
| Snapshot | Site, immutable snapshot ID, route profile, request key and revision, provider task ID, fetched time, schema version, tariff/receipt ref, status and first-page coverage, omitted requested term IDs, item IDs and owning ledger refs |
| Keyword item | Snapshot and row ID, term, target, exact source path, separate provider update dates for keyword and intent or unknown, nullable non-negative monthly average, up to 12 monthly observations, paid competition 0–1, organic difficulty integer 0–100 and admitted intent enum. No scraped result or raw JSON |
| Evidence selection | Site, actor, prompt or brief context ID, exact snapshot and row IDs, source and version, current effect grant and approved display scope |

Use one site-and-profile request key over sorted distinct terms
or the single seed and every fixed targeting and result flag.
Cache matching immutable snapshots. A same-key hit still needs
current site, provider binding, retention rights, live actor and
context grants. Opt-out disables requests immediately. Existing
evidence remains readable only under the retained owner-approved
scope. A narrowed or revoked right invalidates derived groups and
briefs before delivery. A prepare call may return eligible evidence
instead of a paid plan. Reopening or typing
never prepares a request. Before 30 UTC days since fetch, a
snapshot may be reused. Mark metric freshness separately from
request freshness. A field's own update timestamp older than 45 days or unknown
carries its own warning. Never reuse another field's date. At 30 days
show “Saved data may be out of date”. Stale evidence remains
read-only and labelled. Only explicit refresh makes an eligible
cached scope payable again. Hold same-key attempts while sent
or billing is uncertain. Exclude expired or erased rows from
all reads. Keep managed data for 90 days from fetch by default.

For a successful reply require top and task status 20000,
exactly one task and one result, consistent counts and the
expected task ID shape, endpoint and request echo. Only
`tasks[0].result[0].items[]` supplies rows. Bind the Suggestions
`seed_keyword` to the exact approved seed and ignore seed
metadata. Check result and item scopes against code 2826 and
`en`. Refuse a different scope. A documented null scope
without items means no returned data. A non-empty reply with
unknown scope supplies no evidence. Use `total_count` or an
offset token only to label bounded coverage. Never paginate.

Validate each returned term as a short untrusted plain phrase.
A term has 1 to 80 Unicode scalar values and at most 10 words (proposed,
matching the provider limits). It holds letters, digits, spaces and
common punctuation only. Reject control, bidirectional and markup
characters and any line break.
Refuse private, structural or active content. Carry no prompt
instructions, markup or raw response into the Coach or MCP.
Map `keyword_info.search_volume`, `monthly_searches` and
`competition`, `keyword_properties.keyword_difficulty` and
`search_intent_info.main_intent`. Use `keyword_info.last_updated_time`
for the volume and paid competition dataset. Each monthly row's
`year` and `month` identify its observation period. Use
`search_intent_info.last_updated_time` for intent alone.
The admitted keyword property projection has no difficulty update
timestamp. Keep its valid value and show “Date unknown”
with its own freshness warning. Missing or invalid dates stay unknown.
The Coach and local MCP keep these labels and dates when reusing
an authorised item in a brief. Order monthly records by year
and month, with no inferred month or zero. Drop conflicted
month or term identities and identical duplicates. Reject
Overview items outside requested terms. Competition is 0–1
and difficulty 0–100. Intent is informational, navigational,
commercial or transactional. Unsupported fields stay unknown.

Label “DataForSEO keyword estimate”, fetch date, metric date,
market and exact source. Search volume estimates market searches.
Search Console counts appearances of the owner's site. Paid ad
competition concerns advertisers. Organic difficulty estimates
difficulty entering organic results. Do not add or substitute
any of these figures. Treat the volume series as estimates.

## Approval and spend

1. The owner turns the route on in the desktop app, sets daily
   and monthly caps and enters credentials through Tijori. Both
   site and installation caps default to zero. Store each in USD
   cents from 0 to 100,000. The installation owner sets the global
   caps. A site scope cannot expose their remaining balance.
   Local simulation and evidence remain
   available if setup is skipped. The separate “Test in free
   Sandbox” action sends one fixed synthetic term to the same
   Overview path on `sandbox.dataforseo.com`, after showing
   that host and term and asking the owner to confirm. Label
   replies as fixed test data. Check only authenticated test
   transport and a separately admitted dummy fixture shape.
   No paid validation, research storage or fallback. [Sandbox][sandbox]
2. Prepare locally. Compute a maximum with the admitted current
   tariff. Use integer microdollars. The checked tariff is
   `12,000 + 120 × maximum billed items`. Use the number of
   normalised terms for Overview and 50 for Suggestions. The
   maximum is USD $0.01440 for 20 terms and $0.01800 for 50
   items. Return fewer rows at the provider's actual charge.
   The UI displays all significant digits and the tariff date.
   A signed release tariff expires after 30 days or immediately
   on trusted change evidence. Refuse paid work if tariff and
   account billing compatibility cannot be proved.
3. Show the immutable exact request and caps in the owner app.
   Its five-minute monotone expiry starts at preparation.
   App-only `confirm_keyword_lookup` consumes its exact digest
   and binds owner, scope, cap, tariff, credential and consent
   generations. Pending previews die on restart. Agents may prepare
   only. Pause-all stops both approval dispatch and scheduled
   execution. No tool can approve, set caps or unwrap secrets.
4. In one durable transaction, recheck grants, pause, tariff,
   budget, in-flight key and UTC period. Reserve the estimate
   against site and installation day and month caps. The private
   accounting authority locks billing first, then site budget and
   send state, in the same transaction. Callers see only their
   permitted site receipt and cap decision. Count booked charges
   in the dispatch period and all outstanding holds once, including
   earlier periods. Clock rollback or missing budget lineage
   withholds new dispatch. Rotation cannot reset accounting. Lowering
   a cap cannot release an earlier hold. Failed reserves
   dispatch nothing. Before handing bytes to transport atomically
   claim the reservation once and check the current authority
   and deadline. A single fenced Rust dispatcher linearises
   pause, revocation and send. Pause after a send claim cannot
   recall bytes already handed to transport.
5. Only provably unsent cancellations release the full hold.
   Validated provider cost settles to the original dispatch
   periods. Verify task, path, response echo and cost before
   settling. If sent status or billing is uncertain, retain
   the full hold and block the same request key across restart.
   Match the held request using its approved site and canonical
   route, profile and request fields. A new preview, attempt,
   reference tag or credential generation does not create a
   different request key. No replay, retry or fresh approval
   resends it. Reconcile only
   when a verified provider receipt binds the known task to
   that attempt. With no task ID keep the hold until exact
   trusted binding exists or the account is decommissioned.
   A timestamp or balance change cannot settle a request.
   A confirmed over-estimate charge freezes paid dispatch for
   the billing binding for review. Caps bound admitted dispatch.
   Provider repricing and outside account use are external.

Use tightly bounded jobs. Connection timeout five seconds, total
request time 15 seconds and response body at most 2 MiB decoded.
Use the smaller active scheduler memory limit, with a maximum
16 MiB lease for request and parse. Stop on offline, pause,
missing credentials, refusal, cap or limit. Keep all draft work
and show local advice and any currently eligible saved evidence.

## User interface and MCP

```text
Editorial Coach > Keyword research
[Selected terms] [Expand one seed]  Market: United Kingdom, English
[Review lookup]  [Use saved evidence]  [Refresh live data]
Review: exact approved terms | date and maximum in USD | spend remaining
[Approve this lookup]  [Cancel]
Results: DataForSEO keyword estimate | scoped date | groups and gaps
[Use selected evidence in brief]
```

Reuse the 0092 topic grouping contract and 0117 same-site gap
router. A supported coverage hint offers a draft update, eligible
existing page link review or one cluster idea. A stale or
incomplete site index withholds absence claims. Paid metrics
never rank links or write drafts. Use existing 0020 read grants.

| Local MCP action | Input and output |
|---|---|
| `prepare_keyword_lookup` | Site implied by session, mode, short terms and explicit refresh flag. `keyword.plan` grant. Returns cached selection or opaque plan ID, market, quote and owner-app review state. Cannot send |
| `keyword_lookup_status` | Opaque plan ID with `keyword.read`. Returns same-site authorised state, safe charge and retry hold. No raw records or credential |
| `keyword_lookup_results` | Snapshot ID, bounded row selection and `keyword.read`. Returns validated inert evidence, source and freshness under the receiving session and context's current grant. At most 50 rows |
| `prepare_writing_brief` | Existing brief action and grant. An optional selection of up to 20 exact item IDs also needs `keyword.read` and current evidence scope. Resolve each opaque item to its original snapshot and row under the saved selection. Returns a local evidence-linked brief. Removed, cross-site or mismatched references are refused. No paid call or external wording fallback |

VAR-0132-SAVED-BRIEF-EVIDENCE uses the existing local brief
action. Resolve each selected reference in the authorised site
to its exact immutable snapshot, row, source and revision.
Require the admitted DR-117 route profile and UK English target
under the receiving brief purpose and scope. A later lookup
for the same term does not replace the selected row. Refuse a
rebound or wrong-market selection even with identical wording.
Keep the original values, nulls, units, coverage and field dates.
Apply the current retention, rights and freshness labels. Recheck
the brief grant, `keyword.read`, receiving scope, selected
references, retention and erasure at delivery and replay.
Refuse an explicitly requested composition if its selection
fails the recheck. An empty selection uses the existing brief
grant and source checks. Paid keyword rows enrich an otherwise
eligible brief. They do not count as same-site source posts or
pages for 0041 F-0036.

Plain copy includes “You have reached your keyword budget”,
“Billing is uncertain. This request will not be sent again” and
“Keyword lookup is unavailable. Local checks are still available”.
No provider error body appears in the app, logs or MCP.

## Protection, verification and admission

Seal snapshots, plans, indices, holds and temporary data under
0118 before durable storage. Register every storage object and
scope in the 0030 file ledger. Deleting the site or uninstalling
removes its managed records and keys under 0032 and 0118.
Deletion cannot erase an outstanding account-level spend fence
while the account remains usable. Keep opaque charge state
without terms or digests until settlement or decommission.
Verified teardown removes installation files and keys. A restored
or re-enabled connector needs current approval, accounting and
erasure epochs. Hold paid dispatch whenever the current accounting
cannot prove that the request is clear of an unresolved prior send.
Do not rebuild erased request data to make that check. Audit
and logs hold only safe IDs, charge, scope and reason.

Define a Quint model with prepare, approve, reserve, claim,
dispatch, settle, pause, revoke, restart, erase and restore.
Check that unauthorised payload shapes cannot dispatch, every
dispatch has an exact current confirmation and sufficient funds,
one claim hands off at most once, unknown holds survive rotation,
both site and global reservations stay within caps across routes,
no pause allows a new hand-off, and unauthorised or erased cached
evidence cannot return. Reuse unless a current approved refresh
exists.
Quint Connect Rust replay uses a fake clock, vault and transport,
failpoints after each durable transition and synthetic results.

## Settings

Caps are in USD cents, and the other values follow the checked
endpoint contract. Both caps default to zero, so no paid request
can be approved until the owner sets them.

All numbers here are proposed until measured on Dell.

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `keyword.route_enabled` | on or off, owner only | off (proposed) | two values | next prepare |
| `keyword.site_daily_cap_c` | USD cents, owner only | 0 | 0 to 100,000 | next reservation |
| `keyword.site_monthly_cap_c` | USD cents, owner only | 0 | 0 to 100,000 | next reservation |
| `keyword.global_daily_cap_c` | USD cents, owner only | 0 | 0 to 100,000 | next reservation |
| `keyword.global_monthly_cap_c` | USD cents, owner only | 0 | 0 to 100,000 | next reservation |
| `keyword.terms_per_lookup` | whole number | 20 | 1 to 20 | next prepare |
| `keyword.suggestion_items` | whole number | 50 | 1 to 50 | next prepare |
| `keyword.calls_in_flight` | per billing account | 1 | fixed | not settable |
| `keyword.dispatches_per_min` | whole number | 6 | 1 to 6 | next dispatch |
| `keyword.preview_expiry_min` | minutes | 5 | 1 to 5 | next prepare |
| `keyword.connect_timeout_s` | seconds | 5 | 1 to 5 | next dispatch |
| `keyword.request_timeout_s` | seconds | 15 | 1 to 15 | next dispatch |
| `keyword.response_max_mib` | MiB decoded | 2 | 1 to 2 | next dispatch |
| `keyword.snapshot_reuse_d` | days | 30 | 1 to 30 | next read |
| `keyword.field_stale_d` | days | 45 | 1 to 45 | next read |
| `keyword.retention_d` | days from fetch | 90 | 1 to 90 | next night |

The route switch and the four spend caps are owner only. The CLI, MCP
and agents cannot change them.

Market is fixed at `location_code=2826` and `language_code="en"`.
A signed release tariff expires after 30 days. The installation
owner sets the global caps, and a site cannot see their balance.
There is no setting that adds an endpoint, a retry, pagination, a
file import, a live typing request or a scheduled refresh.

## Failure codes

SUT-6640 route off or lookup unavailable; SUT-6641 term refused (privacy,
source or shape); SUT-6642 market or scope refused; SUT-6643 preview expired
or consumed; SUT-6644 cap reached ("You have reached your keyword budget");
SUT-6645 tariff or billing compatibility not proved; SUT-6646 rate or in-
flight limit; SUT-6647 reply failed validation, no evidence kept; SUT-6648
sent or billing uncertain, same key blocked ("Billing is uncertain. This
request will not be sent again"); SUT-6649 over-estimate charge, paid dispatch
frozen; SUT-6650 clock rollback or missing budget lineage; SUT-6651 credential
missing or rotated; SUT-6652 evidence grant narrowed, revoked or erased;
SUT-6653 restore missing approval, accounting or erasure epoch; SUT-6654
Sandbox test refused or failed. Codes SUT-6640 to SUT-6659 are reserved for
this spec (DR-130, under DR-064). No provider error body is shown.

## Security and privacy

- The credential lives in Tijori and goes only into the provider's
  authentication header. No tool unwraps it.
- The request body holds only normalised public search phrases and
  fixed targeting. No site ID, no domain of another site, no tag,
  account name, draft context or editor text goes out. An address of
  the owner's own site goes out only as DR-079 permits.
- The owner's own-site address needs DR-079 owner-only Standard or
  Full (Full also needs the site-text switch); otherwise refuse.
- Only the owner app can approve, set caps or confirm. Agents may
  prepare only.
- Snapshots, plans, holds and temporary data are sealed under 0118
  and listed in the 0030 file ledger. Logs hold only safe IDs,
  charge, scope and reason.
- A site never sees another site's terms, rows or balances. Site
  deletion does not erase an outstanding account-level hold.
- Returned terms are untrusted plain text and carry no instruction,
  markup or raw response into the Coach or MCP.

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| REQ-0132-REQUEST | A request MUST be one of the two fixed operations, within their term and item limits and market, with no retry, import or live typing traffic, and MUST refuse private, secret or unsourced terms. | IT-0132-REQUEST |
| REQ-0132-SPEND | Every dispatch MUST have an exact current confirmation and an in-cap reservation, hand off at most once, keep unknown holds across restart and rotation, and never be recalled or resent. | IT-0132-SPEND, QM-0132-SPEND |
| REQ-0132-EVIDENCE | Evidence MUST keep missing distinct from zero, label source and each date, check cache authority, and never rank links or write drafts. | UT-0132-EVIDENCE, BDD-0132-COACH |
| REQ-0132-ISOLATION | Records MUST stay site-scoped and sealed, MCP MUST NOT approve, and erasure and restore MUST honour current epochs, with no public-page footprint. | E2E-0132-ISOLATION |
| REQ-0132-SETTINGS | The settings MUST be enforced as stated and an out-of-range value refused. | UT-0132-SETTINGS |
| REQ-0132-FAIL | Each refusal MUST carry its named code and no provider error body. | UT-0132-FAIL |

## How it is tested

Designed, not run. Use synthetic provider fixtures and fake
credentials on Ubuntu 26.04, Debian 13 and Windows 11 under Bazel.
The authenticated Sandbox wizard is an owner action, never a CI
dependency. No production endpoint calls in tests.

- **IT-0132-REQUEST:** test Overview and Suggestion limits, wrong
  market, malformed, private and denied terms, the fixed Sandbox,
  secrets, no retry, no import and no live typing traffic.
- **IT-0132-SPEND and QM-0132-SPEND (the Quint replay):** test
  preview expiry, denial,
  two-site and DR-118 concurrent caps, zero budget, clock rollback
  and month and day rollover, pause before hand-off, restart, lost
  reply, double-charge prevention, bogus settlement and a revoked
  grant. Recreate the preview and rotate or replace the credential.
  The same unresolved request stays blocked. After erase and
  restore, missing spend lineage blocks dispatch without restoring
  the erased request fields.
- **UT-0132-EVIDENCE and BDD-0132-COACH:** distinguish missing and
  zero, paid and organic metrics, source dates, cache authority,
  opt-out, narrowed grant, stale, refresh and exact same-site
  routes, and no live ranking write.
  Use FX-0132-SAVED-BRIEF-BINDING for an eligible 0041 brief
  with one current authorised snapshot and exact row selection.
  Save a newer snapshot with the same term and different values.
  The original selection still resolves to the eligible original
  row. Try its reference against the changed revision, another
  market and an incompatible source profile in separate runs.
  Refuse each substitution. The valid brief preserves original
  dates, units and nulls for the 0084 F-0755 pack. With the brief
  grant current, withdraw `keyword.read` after preparation and
  before delivery. Replay the same selection and refuse it
  without replacing the requested output. In a separate request
  an eligible empty selection needs only the existing brief grant.
  In the F-0036 branch, test an absent minimum and one fewer
  eligible post or page than the admitted minimum, with 20 eligible paid
  rows in each. The brief remains held. At the minimum, the
  selected rows only enrich the otherwise eligible brief. Use
  BDD-0041-W1-BLOG-BRIEF and UT-0084-PACK for those joined cases.
  No reuse, refusal or replay starts a provider request.
  In an exact same-site reuse case, read at synthetic UTC clock T
  with the Overview fetched at T. Give volume a dataset date 46 UTC
  days before T, intent a date 45 UTC days before T and valid
  difficulty no date. Show the volume old-date warning and the
  difficulty unknown-date warning. The intent date has no old-date
  warning at T because it is exactly at the default field limit.
  Use no other field or fetch date for undated difficulty.
  Give one reported zero for a month, omit a different month and
  omit a requested term. Present a dated Search Console impression
  series for the same phrase under its own source. Compare the Coach
  list, authorised MCP results and the prepared local brief for the
  same saved item. Keep the stale volume dataset date, intent date,
  unknown difficulty date, monthly zero, gaps and source units
  distinct. At T plus 30 UTC days show the separate saved-data
  warning and recheck each field's own age. Intent now has its
  old-date warning. The source dates, values and omissions stay
  fixed. Reopening, typing and briefing start no lookup, new
  reservation or dispatch.
- **E2E-0132-ISOLATION:** test wrong site, vault isolation,
  untrusted payload, withdrawal, protected storage and erasure
  after restore, the local MCP approval fence and zero public-page
  footprint.
- **UT-0132-SETTINGS:** one value just inside and one just outside
  each range, and each default.
- **UT-0132-FAIL:** trigger each named refusal and check its code
  and that no provider error body reaches the app, logs or MCP.

Release needs admitted tariff, scope, lifecycle, timeout and exact
replay receipts.

## Changes from the source

DR-117 allows only these two keyword routes. File import into this
intake, live typing requests, draft-save requests, scheduled
refresh and ranking input are not built. DR-085 supplied-file
research stays separate, and DR-118 owns the wider live routes in
spec 0133. Provider limits of up to 700 terms and 1,000 items are
cut to 20 and 50.

[overview]: https://docs.dataforseo.com/v3/dataforseo_labs-google-keyword_overview-live/
[suggestions]: https://docs.dataforseo.com/v3/dataforseo_labs-google-keyword_suggestions-live/
[price]: https://dataforseo.com/pricing/dataforseo-labs/dataforseo-google-api
[auth]: https://docs.dataforseo.com/v3/auth/
[sandbox]: https://docs.dataforseo.com/v3/appendix-sandbox/
