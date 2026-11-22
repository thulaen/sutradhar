# 0136: Read-only Matomo timeline connector

A site owner can connect their existing Matomo instance and
see its counted search arrivals beside other measures on the
Search Update Tracker timeline. The connector is off by default. It reads
one authorised site's aggregate reports and installs nothing
on Matomo or on the owner’s site.

DR-120 permits this separate read route. Spec 0040 retains
its own connector store and measures. Only the [Update Tracker][tracker]
can read these aggregates through a typed, site-scoped route.
Editorial Coach, MCP, other reports and ranking have no access.
The connector adds no trackers, writes to Matomo, account
enumeration, individual-visit reads or third-party transfers.

Example: Asha runs a WordPress guide site and a Matomo 6 instance on
another host. She creates a dedicated View-only account and an expiring,
secure-only View token, enters the HTTPS origin and `idSite`, and approves
the mapping in the owner app. At 05:04 Matomo time the daily job pulls the
day three days earlier. The report holds 120 rows, so coverage is full.
`/guides/seo-basics/` has 14 consented search entries and is released.
`/guides/tags/` has 6 and is withheld, because it is under the floor of
ten. On the timeline pane she reads "Matomo search entries" for that
native day. The pane is never added to Search Console clicks. If Matomo
later returns an error envelope on HTTP 200, the day stays unknown.

Figures: every numeric limit, threshold and budget in this spec is proposed
until measured on the Dell or settled by a QR.

## Words used here

| Word | Meaning |
|---|---|
| Matomo search entries | Consenting visits Matomo attributed to a search engine, counted by the first page of the visit |
| Native day | A calendar day in the Matomo site's own time zone |
| Binding | The owner-approved link between one Matomo site and one Sutradhar site |
| Pull | One read of one day's report through the fixed route |
| Coverage | Whether the pulled day is full, partial or unknown, with evidence |
| Surface unclassified | The series does not say which Google surface sent the visit |
| Erasure fence | A live check that blocks reads of erased or revoked source data |

## What this spec designs

| Component | Responsibility |
|---|---|
| Setup and attestation | Owner approval, scope evidence, fixed endpoint, sealed token |
| Wire client | Three POST methods, shape checks and projection of two fields |
| Archive and coverage rules | Proved pre-archived reports, row bounds, partial and unknown days |
| Scheduler | One daily pull, bounded backfill, shared rate limit, retry once |
| Privacy release | Disclosure floor, consent invalidation, retention and erasure |
| Timeline pane | "Matomo search entries" for the Update Tracker only |

## Setup and request authority

Use the local Tauri owner app, existing 0020 action registry,
Rust client, spec 0019 jobs and Tijori sealed credential store.
Use a Matomo 6 adapter where its exact profile has been
admitted. Matomo 6 supports restricted API tokens. Require a
separate owner-created, expiring, secure-only token with
View access. Require a dedicated non-admin account with only
View on the mapped site and a separate restricted View token.
Locally attest the account's site grants, token restrictions
and absence of anonymous View. Successful reads prove neither
permission. Refuse setup if any evidence is missing or if the
profile changes. See [tokens][tokens] and [roles][roles].

The owner enters a fixed HTTPS origin and optional install path, selects one
declared Matomo `idSite` and reviews the mapped Sutradhar site, source time
zone and fields. The setup test uses only the fixed route and allowed methods
below. Require live owner approval binding the mapping, endpoint, profile,
credential, scope attestation and consent generations. Check those bindings
before dispatch, projection and display. Discard a reply from a disabled,
changed or revoked generation. No MCP tool, script or site add-on can enable,
test or remap this connector.

Permit only a verified public HTTPS endpoint controlled by
the owner, with explicit install path followed by `/index.php`.
Check DNS and actual socket address, HTTPS certificate, exact
hostname and port 443 at each request. No redirect, arbitrary
port, embedded credential, inline query, proxy delegation,
private or special address, or post-approval host change.
The paired site and Matomo installation may be different
hosts. Require current owner evidence linking both to the
approved report site. The endpoint preflight must protect
the POST body and token from request logging.

### Wire contract

Use POST form fields under the [Reporting API][reporting]. Always send
`module=API`, `format=JSON`, the exact `method` and Tijori-resolved
`token_auth` in the protected body. Accept only HTTPS success and
`application/json`. Reject a top-level `result=error` envelope even on HTTP
200. Never persist its message or any non-projected property. Use no cookies,
credential headers, free-form method names, callback, batch API or arbitrary
segment.

| Method | Request fields beyond the shared fields | Accepted fields and effect |
|---|---|---|
| `API.getMatomoVersion` | None | Single object with a string `value` equal to the profile version, checked during setup and version revalidation |
| `SitesManager.getSiteFromId` | Exact single `idSite` | Single object with `idsite`, `main_url`, `timezone`. Verify the approved mapping before activation and each pull |
| `Actions.getEntryPageUrls` | Exact `idSite`, `period=day`, exact ISO date, `flat=1`, `filter_limit=1001`, `filter_offset=0`, fixed `segment=referrerType==search;consentState==consented` | Array of flat row objects. Each released row requires `url` and `entry_nb_visits`. Scan at most 1,001 source rows with no paging |

The admitted profile must pin these exact wire shapes against a
synthetic installation of its pinned patch, and fail closed on
shape drift. Integers accept JSON integers or digit-only strings,
within unsigned 32-bit range. Reject fractions, signs, blanks,
duplicate keys, non-scalar required properties and overflow.
Project only the fields above. Recognise archive summary rows
in the admitted profile and never treat a label as a URL.

Match Matomo's [entry metric][entry] and [segment contract][segments].
This projection counts consenting visits that Matomo attributed
to a search engine, by first page of the visit. It does not
identify a Google search surface. Rows named “Others”, hierarchy
totals, unknown scope and unmapped URLs do not become page counts.
Discard provider text and raw payload after validation. Use
safe exact canonical own-site mappings only. Drop URL query and
fragments before any storage if that preserves the exact identity.
Otherwise refuse that row. Never fetch a URL supplied in the output.

Admit only a proved pre-archived report for the exact fixed
segment. Require configuration evidence that serving it cannot
trigger archive generation or other writes. Keep the archiver
outside the connector. If a profile cannot prove this for a
day, do not query it. See [Matomo archiving][archive].

Accept up to 1,000 source rows in returned order, without
filling holes after filtering. A pull of 1,001 or more rows is
rejected whole (SUT-6302) and never shown as complete or partial.
Summary rows or source archive caps mark coverage partial.
Hierarchy drift, conflicting duplicate IDs or ambiguous canonical
aliases also reject the snapshot.
Identical duplicate rows yield one count. A full day needs current archive
coverage evidence, including the effective generation and caps.
A small or empty response cannot prove coverage. A missing row,
failed pull or incomplete day yields unknown for absent pages.

## Scheduling, records and visibility

| Record | Required fields |
|---|---|
| Binding | Site, connection ID, exact endpoint ref, Matomo site ID, profile and source time zone and tzdb revisions, Tijori handle, scope attestation, owner decision and consent generations, enabled/paused/revoked state |
| Pull receipt | Site, connection and credential generation, admitted method and day, start/finish, profile/version, response hash, coverage, row counts and safe reason. No token or response content |
| Timeline aggregate | Site, connection and generation, native page ID, Matomo native date, time zone and tzdb revision, UTC interval, archive generation, metric ID, value or suppressed/unknown, observed time, coverage and privacy release |
| Erasure fence | Site, connection or source scope, source generation and protected erasure epoch. Live on every read, replay and restore |

By default schedule one local daily pull at 05:00 in the Matomo site time
zone, with a stable per-site spread of up to ten minutes. Pull the day three
calendar days earlier. The site scope can explicitly request a bounded 30-day
backfill. Share one request in flight and at most six calls per minute across
all connections to the same endpoint. Apply ten seconds per request and a 4
MiB decoded reply cap. On a proved transient read failure retry once after
five minutes. Pause, disconnect or lowered limits stop new requests. Store the
date key and digest before advancing. Atomically replace the whole day;
omitted rows disappear. Treat a duplicate as a no-op only when the
response digest, current binding, native day and zone, archive evidence,
coverage and privacy release all match the committed projection. A
matching response body cannot preserve a stale coverage or disclosure
decision. Recheck these bindings at replacement. If one changed during
the pull, refuse the stale projection and withhold the old day where
its proof no longer holds. Use the admitted time-zone database for both
local midnights, including 23- and 25-hour days. A
zone or roll-up change starts a separate series. Mark missing slots and prior
evidence stale until revalidated. Revoke on uncertain token or site identity.

Privacy releases use the admitted 0034 disclosure floor of
at least ten visits where applicable, no unsafe totals,
complements, overlap queries or user-defined subsets. Bind
consent and source rounding to the profile. Withhold a count
unless its underlying lower bound clears the floor. A rounded
ten alone is insufficient. If aggregate evidence cannot resolve
withdrawn consent, invalidate the entire affected aggregate.
Display “Matomo search entries”, the exact date basis, coverage and freshness,
in its own pane. Never sum entries with Search Console
impressions, clicks or positions, or with GA4 sessions.
Mark the series as search referrals with surface unclassified.
Align its native day to the timeline by interval, without
splitting or guessing its volume across calendar boundaries.

Protect connection data, receipts and admitted aggregates
under 0032, 0108 and 0118. Retain eligible aggregates for at
most 365 days. The owner can withdraw consent, disconnect or
erase. Revocation cancels jobs, deletes the local token and
invalidates derived views. Restore must use the current
source and erasure epochs. After the one permitted transient retry
fails, keep the requested native day unknown. Show the most recent
eligible saved day of the same active binding and series if current
scope, consent, disclosure, retention and erasure checks still allow
it. Keep its original date and coverage and mark it stale. If no
eligible day exists, show the no-saved-day message under SUT-6305.
A stale day cannot fill the requested day or give a zero. The owner
removes the provider token in Matomo. There is no provider write.

## Data

The records are the four in the table under "Scheduling, records and
visibility": Binding, Pull receipt, Timeline aggregate and Erasure
fence. All are per site and per connection. The token lives only in
the Tijori sealed store, and no record holds it. Receipts keep a response
hash, never response content. Aggregates keep native page ID, Matomo
native date, time zone and tzdb revision, UTC interval and archive
generation. Aggregates are kept at most 365 days. Protection follows
0032, 0108 and 0118, as set out under "Security and privacy" below.

## Settings

The connector is off by default. The owner controls whether it is
enabled and which Matomo site is mapped. The other values are fixed by
this spec and are not owner settings. Setting names are (proposed).
All numbers here are proposed until measured on Dell.

| Setting | Type | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `matomo.enabled` (proposed) | Owner Boolean. Owner only. The CLI, MCP and agents cannot change it | Off | On or off | Before the next request, projection and display |
| Mapped Matomo `idSite` | One declared site | None | One site per binding | After live owner approval and attestation |
| Backfill | Site-scope request. Needs owner approval before any pull | Off | 30 days at most, one bounded run | Next scheduled pull |
| Pull time | Fixed | 05:00 Matomo time, spread of up to 10 minutes | Not settable | Not applicable |
| Day pulled | Fixed | Three calendar days earlier | Not settable | Not applicable |
| Rate | Fixed | One request in flight, six calls a minute per endpoint | Not settable. Lowered limits stop new requests | Next request |
| Request and reply | Fixed | 10 seconds, 4 MiB decoded | Not settable | Next request |
| Source rows | Fixed | Up to 1,000 accepted, 1,001 scanned to detect overflow | Not settable | Next pull |
| Retry | Fixed | Once, after five minutes, on a proved transient failure | Not settable | Next failure |
| Disclosure floor | Fixed | At least ten visits (lower bound) | Not settable | Next release |
| Retention | Fixed | 365 days at most | Not settable | Next sweep |

A pull of 1,001 or more rows is rejected whole (SUT-6302) and never
shown as complete or partial.

## Failure codes

DR-155 reserves SUT-6300 to SUT-6399 for 0136 under DR-064.
Use only the defined codes below. Link each to the same release's
[Matomo recovery help][matomo-recovery] through the 0032 catalogue.
The help and diagnostics expose only the current permitted site and
safe reason. A held or revoked source has no current read authority.

[matomo-recovery]:
  ../../manual/troubleshooting.md#recover-a-matomo-timeline-connection

| Code | Owner message |
|---|---|
| SUT-6300 | "Matomo setup was refused. Check the account, token and site." Evidence of scope, token restrictions or profile is missing or changed |
| SUT-6301 | "The Matomo address was refused." DNS, socket address, certificate, host, port, redirect or path check failed |
| SUT-6302 | "Matomo returned data we cannot use." Wrong shape, error envelope on HTTP 200, 1,001 or more rows, hierarchy drift or conflicting duplicates |
| SUT-6303 | "This day is not ready to read." No proved pre-archived report or archive coverage evidence. The day is not queried |
| SUT-6304 | "This Matomo connection was revoked." Uncertain token or site identity, withdrawn consent or erasure. Jobs are cancelled and views invalidated |
| SUT-6305 | After the one permitted transient retry fails, show "Matomo data is out of date. Showing the last observed day." only with an eligible saved day. Otherwise show "No saved Matomo day is available." The failed day stays unknown in both cases |
| SUT-6306 | "The Matomo connector is paused." Pause, disconnect or lowered limits stopped new requests |

## Security and privacy

- The connector is read-only. It sends only the three POST methods and
  installs, writes and enumerates nothing on Matomo or on the owner's site.
- The token is restricted, expiring, secure-only and View-only. It is
  sealed in Tijori and never stored in a record, receipt or log.
- Each request checks DNS, socket address, certificate, hostname and
  port 443. There is no redirect, proxy, private address or later host
  change.
- No visit-level read, free-form method, callback, batch API or arbitrary
  segment is allowed. No URL in a reply is ever fetched.
- Provider text and raw payload are discarded after validation.
- Only the Update Tracker can read the aggregates. Editorial Coach, MCP,
  other reports and ranking cannot. MCP and exports do not carry them.
- Counts below the disclosure floor are withheld, and an aggregate that
  cannot resolve withdrawn consent is invalidated whole.
- Connection data, receipts and aggregates are protected under 0032, 0108
  and 0118. Restore uses the current source and erasure epochs.

## Requirements

| Requirement | Acceptance |
|---|---|
| REQ-0136-READ | IT-0136-READ uses a simulated Matomo 6 API to prove POST-only token scope and exact methods and report site. Try non-View, expired or public anonymous access, malformed mapping, redirects, unsafe DNS and route injection. No write or raw visit request |
| REQ-0136-METRIC | UT-0136-METRIC distinguishes native day, fixed consent segment, asset and URL identity, flat row, Others, cap, conflicting or missing counts and correct privacy-safe values. Unknown never becomes zero or a Google surface |
| REQ-0136-LIFE | IT-0136-LIFE covers time-zone shifts, missed and replayed slot, bounded backfill, pause and retry, token rotation, erasure and stale restore. No crossing into another site or mode |
| REQ-0136-ARCHIVE | IT-0136-ARCHIVE refuses unproved archive, unsafe account grants, rounding, nested rows, wrong shapes and HTTP 200 errors. Covers 999/1000/1001 rows (1,001 rejected whole), short upstream truncation and removal of prior rows on replacement |
| REQ-0136-SCOPE | IT-0136-SCOPE covers remap during read, consent withdrawal, shared-origin scheduling, 23/25 hour days, and denial to MCP, Coach and other report readers |
| REQ-0136-PARITY | E2E-0136-PARITY uses only synthetic simulated endpoints, fake tokens, sites and dates on Ubuntu 26.04, Debian 13 and Windows 11 under Bazel. Prove protected transport, no public-page footprint and no ranking input |

## How it is tested

Designed, not run. Every test uses synthetic simulated Matomo 6
endpoints, fake tokens, sites and dates. None calls a real Matomo.

- **IT-0136-READ (REQ-0136-READ):** a simulated API proves POST-only
  access with the token in the body, exact methods and the report site.
  Tries a non-View, expired and anonymous-View setup, a malformed
  mapping, redirects, unsafe DNS and route injection. Asserts no write
  and no raw visit request.
- **UT-0136-METRIC (REQ-0136-METRIC):** table tests of native day, fixed
  consent segment, asset and URL identity, flat rows, "Others", the row
  cap, conflicting or missing counts and privacy-safe values. Unknown
  never becomes zero or a Google surface.
- **IT-0136-LIFE (REQ-0136-LIFE):** time-zone shifts, a missed and a
  replayed slot, bounded backfill, pause and retry, token rotation,
  erasure and a stale restore. Nothing crosses into another site.
  For a newly approved synthetic binding, exhaust its first pull with
  two proved transient failures, using the single retry after five
  minutes. SUT-6305 shows the no-saved-day message. The failed native
  day is unknown, and no third request is sent for that pull. Repeat
  with an eligible earlier day in the same active binding and series.
  Show that day with its original date and coverage, marked stale.
  Keep the failed day unknown. In separate runs offer a saved day from
  another site or binding, or one whose consent, privacy release,
  retention or erasure proof now refuses use. Withhold those rows and
  show the no-saved-day message when nothing eligible remains.
- **IT-0136-ARCHIVE (REQ-0136-ARCHIVE):** refuses an unproved archive,
  unsafe grants, rounding, nested rows, wrong shapes and HTTP 200 errors.
  Covers 999, 1,000 and 1,001 rows (1,001 rejected whole), short upstream
  truncation and removal of prior rows on replacement. Replay an
  identical response body after archive coverage becomes partial.
  The owner pane must update coverage, and a late reply with the old
  proof cannot restore the full-day label. A duplicate with matching
  current proof leaves one day and one count per native page.
- **IT-0136-SCOPE (REQ-0136-SCOPE):** remap during a read, consent
  withdrawal, shared-origin scheduling, 23 and 25 hour days and denial to
  MCP, Coach and other report readers.
- **E2E-0136-PARITY (REQ-0136-PARITY):** the synthetic flows on Ubuntu
  26.04, Debian 13 and Windows 11 under Bazel. Proves protected
  transport, no public-page footprint and no ranking input.
- **Limit checks:** each fixed limit in Settings is tested one value
  inside and one outside, in IT-0136-ARCHIVE and IT-0136-LIFE.

## Changes from the source

DR-120 permits this read route. It is new and separate: spec 0040 keeps
its own connector store and measures and is not changed. The source
idea of showing Matomo beside other measures is narrowed to one
aggregate report (entry pages from search), one fixed segment and the
Update Tracker pane. Entries are never summed with Search Console or GA4.
The profile admits Matomo 6 only, where its exact wire shapes are
proved. DR-155 owns the error range allocation for this design.
The connector still needs its own exact profile admission.

[tokens]: https://matomo.org/faq/general/faq_114/
[reporting]: https://developer.matomo.org/guides/reporting-api
[entry]: https://developer.matomo.org/api-reference/reporting-api-apiModule_apiAction-combinations
[segments]: https://developer.matomo.org/api-reference/reporting-api-segmentation
[roles]: https://developer.matomo.org/guides/permissions
[archive]: https://developer.matomo.org/guides/archiving-behavior-specification
[tracker]: 0137-google-search-update-tracker.md
