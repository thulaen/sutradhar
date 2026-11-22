# 0133: Live search evidence in the Editorial Coach

The free local Coach works without an account, internet or spend.
The owner can choose a live lookup for a short topic and see what
the provider observed, when and where. A live result enriches an
existing local run. It cannot reproduce a search engine's hidden
queries, promise a citation, modify content or affect live ranking.

Feature 516 uses only the DR-118 named routes below. Use the one Rust
connector, Tijori binding, site store, pause authority and budget
in spec 0132. The Tauri Coach uses the SDS, with no vault or network
authority in the UI. DR-117's keyword actions retain their own
request contract. DR-085 and DR-086 apply outside these exceptions.
No live call follows a keystroke, save, panel open or cache miss.
DR-134 adds the separate Toolbox preview in
[spec 0143](0143-audit-grid-findings-and-local-preview.md) through
the shared connector and spend gate. Coach approvals and evidence
grants give no preview authority.

Example: Mei is editing a short topic in the Editorial Coach. She opens
"Review a live lookup", chooses Google live results, priority queue,
depth 20, an asynchronous Overview and two People Also Ask clicks. The
preview shows the exact term, country, language and device, the tariff
date and a total maximum of 3,900 microdollars (2 x 1,200 for depth,
1,200 for the Overview, 2 x 150 for the clicks). She approves once. The
result arrives by queue and is saved with the dated "Live Search Data"
badge beside her local run. Typing in the editor buys nothing, and
reopening the panel reuses the saved snapshot. Only "Refresh live data"
can buy a new observation. If the provider times out, she sees "Live data
is unavailable. The local Coach is still available."

## Words used here

| Word | Meaning |
|---|---|
| Live lookup | One owner-approved paid request to a named provider route |
| Instant | An admitted Live result returned in the same request |
| Budget queue | Organic or AI Mode work posted as a task and collected later, at Normal queue or Priority |
| Snapshot | One saved provider observation, with its header, route, market and times |
| Lite | The documented flat-value Top Domains and Top Pages routes |
| Citation | An explicit provider reference. A search link alone is not one |
| Hold | The part of a charge reserved and not yet settled |
| Sandbox | The provider's free test host, used only for setup tests |
| Scheduled batch | A one-shot set of up to 10 approved tasks |

### What this spec designs

| Component | Responsibility |
|---|---|
| Route profiles | The named DR-118 endpoints, tariffs and request fields |
| Setup and approval | Credentials, Sandbox test, exact-byte approval, tag use |
| Queue and batch | Task binding, bounded polling, one-shot batch grant |
| Evidence records | Typed snapshots, results, questions, answers and demand rows |
| Local use | Dated badges, reuse, rights, five-URL passage selection |
| Provider parsing | Owner-selected alternative gated by 0082 proof |
| Extension and MCP | Fenced Python and three scoped local tools |
| Verification | The extended 0132 Quint machine and four acceptance tests |

## Endpoint and tariff profiles

All paths start `/v3/`. POST a UTF-8 JSON array with one typed
task to the pinned HTTPS API host. Balance and collection use GET.
The [contract sources](#source-documents) were checked on
20 November 2026. Amounts below are integer USD microdollars.
The shared tariff admission in 0132 fixes a maximum per profile
and binds account rate, revision and enabled options.

| Choice | Endpoint and approved request fields | Maximum charge and data |
|---|---|---|
| Google live results | POST `serp/google/organic/live/advanced` or `serp/google/organic/task_post`; queue collects GET `serp/google/organic/task_get/advanced/{id}`. Fields are term, location, language, device, depth 10/20/30, optional authorised Overview loading and PAA clicks 0–4 | Base B 2,000 Live, 600 Normal queue, 1,200 Priority. Reserve B × ceil(depth/10) + A + 150 × clicks. A is zero or one B, with the greater documented surcharge used if different. Keep organic slots, People Also Ask, related searches and AI Overviews |
| Google AI Mode | POST `serp/google/ai_mode/live/advanced` or `serp/google/ai_mode/task_post`; queue collects GET `serp/google/ai_mode/task_get/advanced/{id}`. Term, location, language and device only, for one response | 4,000 Live, 1,200 Normal queue, 2,400 Priority. Keep answer elements and explicit cited references |
| AI keyword demand | POST `ai_optimization/ai_keyword_data/keywords_search_volume/live`; selected terms, location and language | 10,000 + 100 × requested terms, at most 20. Keep estimated current and monthly AI demand. Ordinary volume and intent use 0132 |
| Search Mentions | POST `ai_optimization/llm_mentions/search/live`; one target, platform, location and language, `limit=25` | 100,000 + 1,000 × 25. Keep matched observations, dated mention and source references |
| Top domains or pages | POST `ai_optimization/llm_mentions/{top_domains,top_pages}/live` with `items_list_limit=10` or documented `{top_mentioned_domains_lite,top_mentioned_pages_lite}/live` with `limit=10`. One target, platform, location, language, `internal_list_limit=1` | 100,000 + 1,000 × 10 billable rows. Keep attributed groups and measures. Lite is an option for these two views, with the same tariff |
| Historical and new/lost | POST `ai_optimization/llm_mentions/{historical,timeseries_new_lost}/live`. One target, platform, location, language, exact dates, and `group_range=month` on the new/lost endpoint | 100,000 + 1,000 × number of included months, at most 12. Keep provider period buckets and nullable measures |
| Provider text parsing | POST `on_page/content_parsing/live`. One selected public HTTPS URL. Set `enable_javascript`, `enable_browser_rendering`, `enable_xhr`, `store_raw_html`, `switch_pool` false. Keep resources off. Omit pool, user-agent, custom code and browser options | 150 for one plain Live task. Needs separate source policy proof below. No queued version, raw HTML, browser or control bypass |
| Account balance | GET `appendix/user_data` on owner-requested refresh | Free. Cache last verified `money.balance` for 15 minutes. At most one request per account per 60 seconds, below the documented six-per-minute example. This is an account estimate and cannot settle an individual task |

Terms and targeting in the table mean `keyword` or `keywords`,
`location_code` and `language_code`, plus `device` only on
SERP routes with desktop or mobile. Resolve country-level location
and language from an admitted endpoint-specific catalogue. Never
fall back to another locale. The preview shows exact values.
Reject search operators. Each route permits only its listed fields
and the declared tag. Set `resolve_goto_urls=false` on Organic
when supported, including queue. If a profile cannot control
redirect expansion, refuse it. All other optional fields are
omitted under admitted fixed defaults. Use `load_async_ai_overview`
and omit `people_also_ask_click_depth` for zero. For queue POST use
`priority=1` or `2`. No callbacks or account-wide polling.

Mentions use one `target` entity per request. Allow an approved
short keyword with `match_type=word_match` and one supported
`search_scope` of question or answer, or a public domain in
sources scope with subdomains off. Use include matching.
Select `platform=google` for AI Overview or `chat_gpt` for the
vendor's ChatGPT corpus. Every listed ChatGPT Mentions route uses
US English (`location_code=2840`, `language_code=en`), including
Search, Top Domains and Pages, both Lite routes, Historical and
New-and-Lost. Show the exact codes in the preview. Refuse any
other selected locale before preparing a payable plan.
Use `links_scope=sources` where documented for citation groups.
Never infer the model or an observed citation from a search URL.
History takes 1–12 closed months from 1 August 2025 onward,
with exclusive app end translated to the last inclusive date.
No offsets, user filters or automatic pagination.

Count billable units under the exact admitted tariff. For the
Top and historical routes admit proof that the specified row
bound includes all billable rows, including any nested fields.
An unproved bound blocks dispatch. Example: a priority Organic
depth 20 task with an asynchronous Overview and two PAA clicks
reserves 3,900 microdollars. The actual allowed response can
settle lower. Balance does not raise or release a local cap.
Only one subject per Instant action. Use one task per POST,
at most 10 tasks per approved scheduled batch, at most four
active transport calls per installation shared across 0132 and this
spec, one call in flight per billing account (both limits apply), and
six new paid starts per rolling minute across both. The lower current
provider or account limit also applies. No On-Page batch.
Use a five-second connect timeout and 15 seconds overall,
or 130 seconds overall for Live Mentions. A run holds an
admitted lease of at most 32 MiB through parsing.

## Setup, approval and queue

```text
Editorial Coach > Live search evidence [Off]
[Set up credentials] [Test in free Sandbox]
[Instant] [Budget queue: Normal | Priority]
Topic | country | language | device | evidence types
[Use saved evidence] [Review a live lookup] [Refresh live data]
Exact terms or URL | each task | total maximum | tariff date
[Approve once] [Cancel]    [Pause all live work]
Local Simulation (Free)    Live Search Data (DataForSEO, date)
```

Setup reuses 0132 and optional free, confirmed Sandbox tests.
Use fixed synthetic Organic and AI Keyword Data test requests
to `sandbox.dataforseo.com` only. Check the applicable dummy
result shapes, with no research storage, budget effect or paid
fallback. The spend view shows site charges, held amounts, and
remaining site cap. Only the billing owner can inspect account
balance or installation totals.

Approve exact request bytes, tariff, maximum per task and total
under the 0132 model. For routes with documented `tag`, use
an opaque random project prefix and random attempt suffix
unique within the billing binding. Show that a reference tag
is sent. For a route without `tag`, associate the local tag with
the returned task ID. It proves no provider deduplication.
Use the 0132 request fence with the exact route, delivery mode and
canonical request fields. Exclude the per-attempt tag from that key.
Keep the tag in the exact approved bytes where the route sends it.
No project name, private draft or forum field enters a payload.

Instant returns the admitted Live result. Budget queue is for
Organic and AI Mode only. A scheduled batch is one-shot and
binds exact terms, routes, caps, earliest time and a deadline
at most 24 hours after confirmation. The five-minute preview ends at
confirmation. The separately
confirmed batch grant binds child byte hashes, exact per-child
and total maxima, time range and all authority generations.
Reserve each child at dispatch under the live caps. Commit any
completed child once. A pause delays unclaimed children within
their grant. Cap, altered authority, expiry or uncertainty
holds the rest and shows partial outcomes. No substitutes,
automatic reapproval, recurring schedule or uncertain retry.

Accept queue POST top status 20000 and task 20100. Durably bind
task UUID, site, billing binding, exact path, mode, attempt,
quote and acknowledgement charge before collection. Retain the
unsettled portion of its hold. A free GET is allowed only for
that exact bound task. One durable lease starts at 30 seconds,
then waits 60 seconds between attempts, for at most 60 polls and
60 minutes from acknowledgement. Recheck authority, erasure
and pause before every poll. Task codes 40601 and 40602 mean
waiting. Task 20000 with matching result is terminal. Failures
consume a poll, and any non-pending provider refusal stops it.
Account for the original task separately from its result collection.
The admitted route must verify a typed billing proof for the original
attempt. Its closed cases distinguish a collection fee, a terminal
cumulative task total that includes the acknowledgement, and a terminal
refund against the exact booked charge. A refund must prove the final
task charge after that reduction. These are internal meanings. No
provider field supplies one without exact route evidence and independent
admission. [Organic GET][organicget] and [AI Mode GET][aimodeget]
describe free collection and distinguish response-wide cost from task
cost. A free GET may carry a nonzero reported task cost. Neither field
alone proves the final original-task total or a refund. A collection
fee, including zero for a free GET, gives no original-task settlement.
An unproved basis stays `UNSETTLED` under
SUT-6201, with the existing booking and outstanding hold unchanged.

The proof binds the original billing key and epoch, UTC periods,
attempt, provider task, request digest, profile revision and a
verified evidence reference. Record its kind and amount in integer
USD microdollars. A cumulative total must be non-negative and at
least the verified booking. A refund gives a non-negative credit
and a verified resulting total equal to the booking less that
credit. A difference without admitted refund proof stays
unsettled. No result body or balance delta supplies a refund.

In the shared 0132 ledger, match the proof to the original site,
billing binding, approved request and attempt, route and mode, and
verified task ID. Atomically book the verified cumulative total less
what is already booked, release the remainder of the original hold
and save the terminal receipt. Use the original dispatch UTC day and
month for both cap scopes, even after rollover. Use a downward delta
only for a verified terminal refund up to the amount already booked.
Refuse an unrelated or conflicting proof without changing accounting.
An exact replay returns the saved receipt and changes neither the
balance nor the hold. A confirmed total above the reservation
books the verified actual and
freezes the shared billing binding under 0132's existing overrun rule.
Settlement preserves the original approval and grants no new dispatch,
result use or collection authority.

A lost POST keeps its hold. Recover or reconnect without scheduling
another POST. Timeout or provider
limit shows
“Live data is unavailable. The local Coach is still available.”
While final billing is uncertain, show “Billing is uncertain” and the
amount still reserved from the budget. The final charge is unknown.
If a verified acknowledgement charge is already recorded, show it
separately from the reservation and the final charge.

## Evidence and local use

| Record | Required fields |
|---|---|
| Live snapshot | 0132 snapshot header, exact service and route, market/profile, device where applicable, approved term IDs, provider task and capture/accepted times, coverage and missing-field reasons, typed result references |
| Search result | Snapshot ID, observed type and ordinal, organic rank if present, title, approved public URL reference or unresolved, and source date. Paid, organic and citation positions stay distinct |
| Question or angle | Snapshot, origin PAA/related/AI heading or question, parent question ID or none, observed ordinal, scrubbed short text and optional evidence ID. No reconstructed provider search |
| Answer and citation | Snapshot, surface Organic Overview or AI Mode, typed block/ordinal and safe plain text, explicit reference ID and exact reported URL limited to safe fields. A link without reference evidence is not a citation |
| Demand and mention row | Snapshot, source profile, term or target ID, platform, period, source group, nullable measured/provider-estimated values and exact explicit references. Unknown and observed zero stay distinct |
| Passage evidence | Source URL, capture, legal and scrubber receipts, bounded text chunks and ranges, permission, expiry and score profile references. Draft and provider write capabilities absent |

Validate envelope, task echo, typed route and expiry. Distinguish
full nested and flat Lite values by schema. Bound each reply to
8 MiB decoded, depth 32 and 1,000 typed rows. Cap titles and
questions at 512 characters, URLs at 2,048 and answer text at
64 KiB. Reject active content (scripts, markup, links and embeds)
and mark truncation or missing fields. Strip or escape bidirectional
and control characters from any text shown or logged. Keep vendor
samples out of 0042 and 0093 denominators. Never broaden
a URL by dropping identity
fields. If a credential, secret query or private target prevents
safe storage, retain only an unresolved reference. Exact safe
reported addresses are inert in the interface.

The 0117 local 8–15 and 30–50 query counts exclude imports.
Present imported PAA and related questions alongside them. Derive AI Mode angles
only from admitted explicit headings or questions with evidence.
Show the dated Live badge on every imported item, even when a
local query has identical words. Keep both source references.
Only selected evidence with current role and source rights can
enter local embedding or reranking. No prompt, tool or hook
turns provider content into instructions or a paid plan.

The owner selects up to five distinct public URLs from captured
results or citations. Reuse eligible local evidence when present.
Otherwise elect one new delivery route for an exact URL before
its shared 0082 one-off claim. Default to Leviathan. No result
may fetch automatically. A paid alternative cannot follow a
success, denial or failed attempt on the same target claim.

Provider parsing is a separate owner-selected alternative to a
local read. Review the exact public HTTPS URL and price. Before
giving the address to the provider and before dispatch, require
0082 legal and regional approval for that exact provider crawler
and target. Prove guest-only, declared identity, TLS, address,
redirect, opt-out, no further fetch and bounded scrub rules.
A local allow or header probe proves none of the provider
controls. The public OnPage guide leaves these controls unproved.
Without an admitted capability record the alternative is
unavailable. The [endpoint contract][parse] documents an echo in
`tasks[].data` and nested `page_content`. A request echo or a link
inside the parsed content cannot prove what address was fetched.
Require a protected observation proof admitted under 0082. Bind the
site and approval generation, supplier capability and controls,
request plan, one-off claim, attempt and provider task. Bind the
requested URL, actual final URL, ordered redirect observations,
capture ID and UTC time, and a digest of the exact observed page
item bytes covered by the proof. Use the admitted item encoding
and proof verifier. Verify all addresses in order against the
approved scope. A proved target does not authorise different item
bytes. Missing or conflicting proof leaves the reply unusable.
Billing and result admission stay separate. Keep the original claim.

After current rights and task checks, the conditional profile may
project only admitted `text` leaves of
`tasks[0].result[0].items[0].page_content.main_topic[i]` under
`primary_content[j]`. Require one task, result and finished page
item of the admitted type. Use the existing reply caps and 0117
passage limits. Keep topic and row order. Do not read URLs or
linked objects as passage text. Keep attribution, capture and proof references
on each passage for the 0117 local reranker. A bounded extraction
carries only its observed coverage. Embedded addresses give no
follow, fetch or score authority. Unselected metadata, author
fields and active content stay outside the passage projection.

Cache admitted snapshots by route and site. Recheck rights,
role and current erasure epoch before use or after restore.
Opt-out stops acquisition. Retained rights govern saved use.
Mark SERP/AI Mode after seven days, other data after 30. Keep capture and
vendor dates distinct.
Reuse by exact request and current permission on reopening
and typing. Only deliberate refresh may buy a new observation.
Retain for at most 90 days, or shorter reuse rights. Protect
registered cache files, indices, private URL refs and receipts
under 0030, 0032 and 0118. The storage wizard can move sealed
objects to an external SSD or authorised encrypted S3. Keep
keys local, track every managed replica and require complete
erasure evidence when deleting the project or uninstalling.

## Extension, MCP and verification

Spec 0104 fenced Python workers receive bounded typed evidence
and can return analysis or prepare a named plan through the
Rust broker. No socket, credential, arbitrary domain, raw
response or bypass. Cap one job at two seconds and 64 MiB.
The broker alone selects the pinned host and budgeted request.

Local MCP can `prepare_live_research` from short approved-field
proposals, `read_live_status` by plan and `read_live_evidence`
by scoped snapshot and fields. Require current site grants
`live.plan` or `live.read` as appropriate. Return typed summaries
and site charge receipts only. Exclude private URL refs, terms
outside scope, raw content and secrets. The owner sees a desktop approval
for any proposed
paid run. MCP approval, spend setting changes and remote access
are unavailable. Respect pause-all for paid traffic and job
progress. Refuse another site before resolving IDs.

Extend 0132's Quint machine with queue pending/ready, same-key
reuse, one-shot batch, lawfully sourced rival evidence and
replica erasure. Invariants require exact confirmed prices and
active daily/monthly funds before paid hand-off, disjoint
private payload types, no paused paid hand-off, no duplicate
task claim, reuse except approved refresh, and no surviving
cache after verified deletion. Queue settlement preserves the
original UTC periods. Assert that no collection fee settles a task
and that one original attempt commits at most one terminal receipt.
That receipt books one delta and releases the remaining hold once.
The same receipt or different proof cannot move either amount again.
A lost reply, restart or receipt conflict grants no dispatch.
Quint Connect Rust replay uses
fake transports at every boundary with seeded time and crashes.

## Data

Records live in the site store of spec 0132, which this spec shares.

- The six records in the table under "Evidence and local use": live
  snapshot, search result, question or angle, answer and citation, demand
  and mention row, and passage evidence.
- Approval and spend records: the approved request bytes, tariff, maximum
  per task and total, the tag, site charge receipts and holds.
- Queue records: the durable binding of task UUID, site, billing binding,
  exact path, mode, original request and attempt, quote and original UTC
  period keys, acknowledgement booking, remaining reservation and poll
  lease. Keep terminal proof kind, safe evidence reference, verified
  charge or refund, and the one settlement receipt in the 0132 ledger.
  Retain only its permitted opaque accounting state after erasure.
- Batch grant records: child byte hashes, per-child and total maxima,
  time range and authority generations.
- Cache files, indices, private URL references and receipts, registered
  under 0030, 0032 and 0118, with any managed replica tracked.

## Settings

The owner switch and the fixed limits below belong to this spec. Spend
caps, the pause authority and the credential belong to spec 0132. The
setting name is (proposed).

All numbers here are proposed until measured on Dell.

| Setting | Type | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `live_evidence.enabled` (proposed) | Owner Boolean, owner only | Off | On or off | Before the next dispatch |
| Delivery mode | Per approval | Instant | Instant, Normal queue or Priority queue | At that approval |
| Organic depth | Per request | Chosen at approval | 10, 20 or 30 | At that approval |
| PAA clicks | Per request | 0 | 0 to 4 | At that approval |
| Balance cache | Fixed | 15 minutes, one request per account per 60 seconds | Not settable | Next refresh |
| Transport | Fixed | Four active calls per installation, shared with 0132; one in flight per billing account; six new paid starts a minute | Not settable. Both limits apply; lower provider or account limit also applies | Next dispatch |
| Timeouts | Fixed | Connect 5 s, overall 15 s, or 130 s for Live Mentions | Not settable | Next request |
| Queue polling | Fixed | First at 30 s, then every 60 s | At most 60 polls and 60 minutes | Next task |
| Scheduled batch | Fixed | One shot | At most 10 tasks, deadline at most 24 hours | At confirmation |
| Reply bounds | Fixed | 8 MiB, depth 32, 1,000 rows | Not settable | Next reply |
| Stale marks | Fixed | SERP and AI Mode at 7 days, other data at 30 | Not settable | Next read |
| Retention | Fixed | 90 days, or shorter reuse rights | Not settable | Next sweep |
| Passage URLs | Per run | None | Up to 5 public URLs | At selection |

`live_evidence.enabled` and the spend caps it relies on (spec 0132) are
owner only. The CLI, MCP and agents cannot change them.

## Failure codes

DR-155 reserves SUT-6200 to SUT-6299 for 0133 under DR-064.
Use only the defined codes below. Link each to the same release's
[DataForSEO recovery help][live-recovery] through the 0032 catalogue.
For a denied site or grant, withhold state and accounting details.
For an authorised owner, an unusable result and uncertain billing
can both be present. Keep the result reason and the outstanding
hold distinct. Neither authorises another send.

[live-recovery]:
  ../../manual/troubleshooting.md#recover-an-optional-dataforseo-lookup

| Code | Owner message |
|---|---|
| SUT-6200 | "Live data is unavailable. The local Coach is still available." Timeout or provider limit |
| SUT-6201 | "Billing is uncertain." Show the reserved amount and leave final charge unknown. Show an already verified acknowledgement charge separately. A lost POST keeps its hold |
| SUT-6202 | "This lookup is not available." Route, tariff, locale, row bound or redirect control not admitted. Refused before a payable plan |
| SUT-6203 | "Live work is held." Cap, pause, altered authority or expiry holds the plan or the rest of a batch |
| SUT-6204 | "The provider reply could not be used." Envelope, task echo, exact observed-target proof, bounds or active-content check failed |
| SUT-6205 | "Provider page reading is not available." No admitted 0082 capability record or approval for that URL |
| SUT-6206 | "This request is not allowed for this site." Missing `live.plan` or `live.read` grant, or another site |

## Security and privacy

- Live lookups are off by default and each paid call needs exact owner
  approval. MCP cannot approve, change spend settings or reach remotely.
- The Tauri UI has no vault or network authority. Only the one Rust
  connector uses the Tijori credential and the pinned HTTPS host.
- No project name, private draft or forum field enters a payload.
- An address of the owner's own site goes out only as DR-079 permits
  (owner-only Standard or Full; Full also needs the site-text
  switch); otherwise refuse.
- Replies are bounded and validated. Active content is rejected. Reported
  URLs are inert, and private or secret-bearing targets keep only an
  unresolved reference.
- No prompt, tool or hook turns provider content into instructions or a
  paid plan. Python workers get no socket, credential or raw response.
- Provider parsing needs 0082 legal and regional approval for the exact
  crawler and URL. A local probe proves none of the provider controls.
- MCP returns typed summaries and site charge receipts only, never
  private URL references, secrets or other sites.
- Cache, indices, URL references and receipts are protected under 0030,
  0032 and 0118. Deleting a project or uninstalling needs complete
  erasure evidence for every managed replica.

## Requirements

| Requirement | Acceptance |
|---|---|
| REQ-0133-DISPATCH | IT-0133-DISPATCH proves every endpoint/path, allowance, quote, Sandbox fallback refusal, cap and UTC transition. For each ChatGPT Mentions route, check the exact US English payload and refusal of another country or language before a payable plan. Replay acknowledgement loss and pause before POST or GET, batch expiry, partial charge and simultaneous 0132 work. Prove zero duplicate paid POSTs or charge claims for the same task. Allow repeated free GETs only for an admitted task within its collection limits. CI makes zero paid calls |
| REQ-0133-EVIDENCE | BDD-0133-EVIDENCE verifies local/live badges, observed source and unknowns, citation vs search link, Lite scope, no local ranking input and local scoring only. Test imported items beyond the local run quota, rights loss and no typing or model-triggered paid plan |
| REQ-0133-SAFETY | E2E-0133-SAFETY tries denied robots, unsafe origin, unproved provider parsing, poisoned payload, Python egress, stale MCP grant, encrypted move and erasure after restore |
| REQ-0133-PARITY | E2E-0133-PARITY runs each synthetic flow and Quint replay on Ubuntu, Debian and Windows 11 under Bazel with identical policy and output. No public-page footprint |

## How it is tested

Designed, not run. CI makes zero paid calls. Every test uses fake
transports and synthetic fixtures, with seeded time and crashes.

- **IT-0133-DISPATCH (REQ-0133-DISPATCH):** every endpoint and path,
  allowance, quote, Sandbox fallback refusal, cap and UTC transition.
  For each ChatGPT Mentions route, the exact US English payload and the
  refusal of another country or language before a payable plan. Replays
  acknowledgement loss, pause before POST or GET, batch expiry, partial
  charge and simultaneous 0132 work. Lose the POST reply before any
  charge receipt is verified. The owner sees the full reservation
  and an unknown final charge. In a separate branch, keep a verified
  queue acknowledgement charge and lose the final reply. Show that
  charge apart from the remaining reservation. Final charge stays
  unknown. In the lost POST branch, prepare the same request with a
  new tag. Its hold blocks a second POST. Zero duplicate paid POSTs
  or charge claims for one task. Repeated
  free GETs only for an admitted task within its limits. Use synthetic
  accounting amounts only. Reserve 1,800 microdollars and verify a 600
  acknowledgement booking. Settle after the original UTC day and month
  roll over. A GET-only fee of zero leaves 600 booked and 1,200 held,
  with final billing unknown. A nonzero reported task cost with no
  admitted binding also leaves these values unchanged. In separate runs
  accept a verified cumulative total of 900 and a terminal refund of
  150 against the acknowledgement. The original periods record 900 or
  450 respectively. Release only the outstanding hold, once.
  Replay the exact proof after
  restart and lose its reply after commit. No counter or hold moves
  twice. Change the original task, attempt or billing binding while
  keeping the amount equal. That proof cannot settle either task. A
  smaller cumulative total without refund proof stays unsettled. An
  exact proof of 1,900 books that actual against the original periods
  and freezes further paid sends under 0132. None of these proofs
  approves another call. Check SUT-6200 to SUT-6203 and the fixed
  limits in Settings, one value inside and one outside each.
- **BDD-0133-EVIDENCE (REQ-0133-EVIDENCE):** local and live badges,
  observed source and unknowns, citation against search link, Lite scope,
  no local ranking input and local scoring only. Imported items beyond
  the local run quota, rights loss, and no typing- or model-triggered
  paid plan. Checks SUT-6204.
- **E2E-0133-SAFETY (REQ-0133-SAFETY):** denied robots, unsafe origin,
  unproved provider parsing, poisoned payload, Python egress, stale MCP
  grant, encrypted move and erasure after restore. Checks SUT-6205 and
  SUT-6206.
- **E2E-0133-PARITY (REQ-0133-PARITY):** each synthetic flow and the
  Quint replay on Ubuntu, Debian and Windows 11 under Bazel, with
  identical policy and output and no public-page footprint.
- **Model check:** the Quint machine of 0132 extended as described under
  "Extension, MCP and verification", replayed by IT-0133-DISPATCH and
  E2E-0133-PARITY.

### F-0516 provider evidence

DES-0133-W2-PROVIDER-REQUEST uses the existing 0132 budget and
owner approval. VAR-0133-W2-SERP-LIVE covers the exact admitted
Live search routes in this specification. VAR-0133-W2-SERP-QUEUE
covers the Organic and AI Mode queue only. A free result read
binds the acknowledged task and its authorised collection window.
VAR-0133-W2-ONPAGE-PARSE is the separate conditional supplier
parsing variant. It uses the same approved budget and connector.
Before enabling a payable plan, 0082 and QR-144 must admit the
exact supplier controls, target-binding capability and result
profile. Require the actual observed-target proof after dispatch.

Use FX-0133-W2-PROVIDER-GATES under IT-0133-DISPATCH and
E2E-0133-SAFETY. Prepare a synthetic Live plan and a one-shot
queue batch over local MCP. Confirm each exact quote in the
owner app. A mock response returns a result only for its
authorised site and task. Lose the POST acknowledgement of a
separately approved queue task. Try a fresh tag, another site,
a different endpoint, an expired grant and typing-time or
recurring dispatch. Hold each unauthorised
operation and count no second paid POST. Exercise direct
DR-117 keyword lookups separately under 0132.

Use FX-0133-W2-EXACT-ONPAGE-PASSAGE in IT-0133-DISPATCH,
BDD-0133-EVIDENCE and E2E-0133-SAFETY. A synthetic stand-in supplies
matching admitted capability proof and exact fetched-target
evidence for one owner-selected public URL and confirmed quote.
Observe one allowed Live request on the fake transport, with the
single-route claim and no draft bytes. Return one finished result
with bounded primary text. Show one dated provider passage to the
authorised 0117 rehearsal, with source attribution and no link
follow. In separate runs remove the capability proof before
dispatch or supply a successful task and matching echo with no
observed-target proof after it. Substitute another page's item bytes
while retaining the task, echo and valid original target proof.
Replace the grant after receipt and before admission or a cached
local scoring read. An equally scoped replacement cannot
revive the old release. Use SUT-6205 for missing pre-send
capability and SUT-6204 for unusable post-send target proof.
Settle a valid independent terminal charge once under 0132 even
when the result is refused. Missing billing proof keeps its
original hold. No denied passage, second paid request, local
crawl or widened reader follows. The stand-in qualifies no provider capability.

## Changes from the source

Feature 516 is limited to the DR-118 named routes and shares the 0132
connector, vault, store, pause authority and budget. The live result only
enriches a local run. It does not reproduce hidden search queries, promise
a citation or affect ranking. Every ChatGPT Mentions route is fixed to
US English. Provider parsing is an owner-selected alternative that stays
unavailable until 0082 proof exists. Queue work is Organic and AI Mode
only, with no recurring schedule or callback. The contract sources were
checked on 20 November 2026. DR-155 owns the error range allocation
for this design. Provider routes still use their existing admission.

## Source documents

- [Organic Live][organic], [Organic queue][organicq],
  [Organic result GET][organicget], [Organic tariff][orgprice].
- [AI Mode Live][aimode], [AI Mode queue][aimodeq],
  [AI Mode GET][aimodeget], [AI Mode tariff][aiprice].
- [AI demand][demand], [Demand tariff][demandprice], [Balance][balance].
- [Mentions][mentions], [Top domains][domains], [Top pages][pages],
  [Domains Lite][domainslite], [Pages Lite][pageslite].
- [Historical][history], [New and lost][newlost],
  [Mentions tariff][mentionsprice].
- [OnPage parsing][parse], [OnPage tariff][parseprice],
  [Crawler restrictions][crawl].

[organic]: https://docs.dataforseo.com/v3/serp/google/organic/live/advanced/
[organicq]: https://docs.dataforseo.com/v3/serp/google/organic/task_post/
[organicget]: https://docs.dataforseo.com/v3/serp/google/organic/task_get/advanced/
[orgprice]: https://dataforseo.com/pricing/google-serp/google-organic-serp-api
[aimode]: https://docs.dataforseo.com/v3/serp/google/ai_mode/live/advanced/
[aimodeq]: https://docs.dataforseo.com/v3/serp/google/ai_mode/task_post/
[aimodeget]: https://docs.dataforseo.com/v3/serp/google/ai_mode/task_get/advanced/
[aiprice]: https://dataforseo.com/pricing/google-serp/google-ai-mode-serp-api
[demand]: https://docs.dataforseo.com/v3/ai_optimization-ai_keyword_data-keywords_search_volume-live/
[demandprice]: https://dataforseo.com/pricing/ai-optimization/ai-keyword-search-volume
[balance]: https://docs.dataforseo.com/v3/appendix-user-data/
[mentions]: https://docs.dataforseo.com/v3/ai_optimization-llm_mentions-search-live/
[domains]: https://docs.dataforseo.com/v3/ai_optimization-llm_mentions-top_domains-live/
[pages]: https://docs.dataforseo.com/v3/ai_optimization-llm_mentions-top_pages-live/
[domainslite]: https://docs.dataforseo.com/v3/ai_optimization/llm_mentions/top_mentioned_domains_lite/live/
[pageslite]: https://docs.dataforseo.com/v3/ai_optimization/llm_mentions/top_mentioned_pages_lite/live/
[history]: https://docs.dataforseo.com/v3/ai_optimization-llm_mentions-historical-live/
[newlost]: https://docs.dataforseo.com/v3/ai_optimization-llm_mentions-timeseries_new_lost-live/
[mentionsprice]: https://dataforseo.com/pricing/ai-optimization/llm-mentions
[parse]: https://docs.dataforseo.com/v3/on_page-content_parsing-live/
[parseprice]: https://dataforseo.com/pricing/on-page/onpage-api
[crawl]: https://dataforseo.com/help-center/troubleshooting-onpage-api
