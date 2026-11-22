# Spec 0143 - Site Health, audit views and local search preview

Site Health shows what was checked, what changed and where to start.
The Local Search Preview shows either a free layout example or a dated,
paid sample of public listings in a market the owner selected.

## Ownership

Use [0044][seo] and [0047][structure] audit producers, [0141][crawl]
tool and log findings, [0142][lab] performance evidence and [0144][text]
corpus facts. [0145][catalogue] defines versioned checks and typed fields.
[0043][alekh] owns saved views and safe expressions. [0048][fix] owns
Fix List identity, proposals, priority, effort and verification. Reuse
0132/0133 for DataForSEO custody, tariff, transport and accounting.
None of these reads changes a live page, publishing plan or ranking.

## AG-01 Evidence, snapshots and filters

Use one immutable result manifest per site, project, evidence scope,
canonical inventory and profile set. Pin check and field catalogue
versions, each source snapshot, comparable clock, coverage and rights.
Per-page records follow the canonical identity of the producer and
retain the observed address separately. An external edge can be
inspected only to the extent already covered by the source. A redirect,
canonical or language alternate is an observation and confers no
merging, redirect, fetch or other site permission.

| Record | Fields beyond the shared 0140 envelope |
|---|---|
| `field_definition` | `field_id, catalogue_version, producer_profile_kind, grain, scalar_type, unit, projection_class, nullable_conditions, filter_operators, cardinality`. Bind the exact 0145 URL, occurrence or selected drawer subject. A drawer can select a request, phase, frame, fact, change or corpus result without adding a joined URL row. Bind the subject, data type, unit and field UUID to the versioned producer schema. |
| `audit_fact` | `site_id, project_id, audit_snapshot_id, row_kind, row_id, field_id, definition_version, input_generation, value?, availability, evidence_ref?, observed_utc?, coverage_ref`. Primary key includes the manifest identity and field ID. Every measured value carries a definition and evidence. Restricted, unsupported, missing, partial and stale are distinct internal states. |
| `audit_check_result` | `site_id, audit_snapshot_id, rule_id, rule_revision, cause_key, scope_key, subject_row_id, result, reason, evidence_refs[], category, severity, versioned_template_ref, first_seen_utc, observed_utc`. Result is `found, clear, unknown, not_applicable`. Severity is Error, Warning or Notice for found rows only. |
| `audit_view` | `view_id, site_id, snapshot_id, definition_versions, field_ids[], filter_ast, order, rights_revision, erasure_epoch`. A cursor also binds the session, last typed sort value, row ID and 10-minute expiry. |
| `grid_source_join` | `site_id, source_kind, authority_ref, source_snapshot_id, canonical_join_version, page_id, metric_id, grain, unit, window_start_utc, window_end_utc, source_zone, freshness, coverage_ref, evidence_ref, value?`. Missing proof or denial carries a typed reason. |

Select a saved audit, section and view, then choose field, operator
and value from the registry. Support equal, different, ordered
comparison, bounded prefix for approved text, known or unknown and
membership. Sort chooser names by pinned British English collation
and then stable field UUID. Sort complete comparable row values by
type, nulls last in both directions, then safe display name and row
UUID under 0145. The default row sort uses name and UUID. Omit value
sort for pointers or incompatible measures. Limit to 32 displayed
columns, 16 predicate leaves, 64 membership items per leaf, 200 rows
per page and two nested Boolean groups. Wrong grain, field version,
cursor or site fails before scanning. Each typed source projects
eligible fields before applying a filter to prevent inference.
Visible row-level counts observe 0034 where they touch people.

For multiple segment membership, a row appears within each
matching segment. Label overlap and compute site counts from
distinct canonical identities. An empty segment reports coverage
and no rate. Rates divide found rows by check-eligible measured
rows, with their scope and measured denominator. Show unknown and
not-applicable separately. Page and link rows use separate grains.
No blended health or AI discoverability score.

Custom field expressions use the admitted 0104 worker and 0043
typed inputs. Owner approves the exact script, field allowlist,
schema and budget. Precompute on saved local material in a job.
Return only a typed scalar or unknown. No live query, raw SQL,
credential, network, file, unrestricted clock or private field.
Stop on error or budget at each batch, fence its release and retain
the independent built-in fields. No custom field inherits export
or MCP permission.

### Source enrichment

- **Search Console and GA4.** Read existing 0040 admitted same-site
  projections at the exact page, metric grain, window and zone.
  Preserve property, analytical surface, unit and freshness.
  Search Console impressions and GA4 visits remain distinct.
- **Matomo.** Ingest an owner-approved local closed aggregate only.
  Require a declared same-site property and producer, matching
  eligible period, units and source coverage. Apply 0034 disclosure.
  Never share a grid tab with the 0136 direct connection's scope or
  query per-visit, URL referrer-path or member material. Show
  “Supplied Matomo figures” with date and measured window.
- **CrUX.** Read entitled 0044/0142 fields for admitted eligible
  URL, form factor and field window. Origin evidence stays labelled
  in an origin panel. A missing page record gives no URL value.
- **Paid local preview.** Keep results in their own dated card.
  This route creates no crawl facts or free view joins.

Use the exact admitted address-to-page map for all joins. An
ambiguous alias, property, scope, clock, unit or protected overlap
holds the enrichment and supplies neither zero nor copied totals.
A source-specific clock or destination copied into a report keeps
its label and coverage. Never combine overlapping sources as totals.

## AG-02 Review and progress

Show eight category panels for Crawlability, HTTPS and Security,
Performance, Internal Linking, International, Structured Data,
AI Bot Access and Content Quality. Put qualified accessibility
observations in Content Quality with their own filter. Render an
independent panel for each category with measured scope, severity
counts, unknown count and an inspectable list of findings. Use
“AI bot access” for observed or simulated permissions only.
The panel cannot infer selection, discoverability or rankings.

In the inspection view, sort admissible findings by severity band
and comparable evidence within the category. “What to do first”
shows 0048’s exact calls and orders measured gaps by size and range,
measured risk by severity, structure by pages affected and evidence
needs by information gain per cost. Show its deciding reasons,
limits, weakest input and revisit trigger. Show observed affected pages, unit,
eligibility, uncertainty, scope and the measured review date.
Within an equal impact band, show the sourced effort range if it
is compatible. Unavailable effort goes in “Needs an estimate”.
Use rule ID and cause key as the final stable order. The impact
and effort matrix is a view of separate values with labelled
unknowns. Never estimate time from severity, raw counts, ratings
or another site. A saved owner severity override maps an exact
rule revision to one band within its category. Preview affected
counts and sorting, use live confirmation, pin the template
version and keep the source severity visible. A later policy or
band change does not rewrite old audits or create a recurrence.
The gate for a new severity template is 0048's review workflow.

Use 0048's logical Fix List key for the site, check and cause.
Check eligible comparable snapshots only. A currently found cause
with earlier measured clear is “New regression”. Same cause
measured found in both is “Still open”. Earlier found with
current clear earns “No longer observed” until the 0048 verification
route confirms the exact repair; call it “Fixed” only then. With
scope changed, page missing or current result unread, use
“Comparison unavailable” and preserve the Fix List item.
These comparison labels are evidence. They do not set Fix List
state. No recheck, assignment, repair or publication follows a read.

### UI and exports

Site Health opens on audit date and coverage, then category
panels and “Review suggested work”. The next view puts the issue
reason and evidence first, then separate WordPress and XenForo
manual instructions and review action. Page Explorer and Link
Explorer open a common drawer with "Overview", “Documents”,
“Links”, “Markup”, “Speed” and “Text” tabs. The Documents tab
reads only current safe raw and render projections. It shows
headers, redirect hops and read-only bounded plain text. It
does not run or load markup. A speed tab can open the 0142
waterfall and frame evidence, with the same source and permission.
Offer saved view grouping and drill-down with keyboard links;
render charts and evidence notes with equivalent tabular text.

Register exports through 0100, 0118 and the 0030 file ledger. Use
the exact owner-approved wizard destination and an owner-only file.
Project the permitted fields and recipient, neutralise text cells
under SD-32 and preserve typed numbers. Seal the staged output and
manifest. Recheck destination, rights and erasure at publication.
Release one complete file with a verified ledger receipt. Reconcile
a lost receipt before any retry. Use local PDF only after QR-039
and exact safe renderer admission under 0094/0100. The
local developer ZIP has only UTF-8 fixed names `manifest.json`,
`findings.csv`, `pages.csv` and `fix-steps.md`, in that order.
Manifest pins schema, snapshot digest, field catalogue, counts,
files and SHA-256 digests. Use inert owner-written instructions
and bounded scrubbed data. No scripts, active links, raw body,
path names, images, secrets, support dumps or personal fields.
Allow up to 25 MiB decoded total and 10,000 data rows per file,
with stricter existing caps taking precedence. An over-limit
request offers a smaller view and creates no partial archive.
The ZIP itself needs exact export, licence, parser and cleanup
admission. No archive opens, sends or publishes automatically.

For a lost reply on a local CSV export, reopen the original
operation. Reconcile its exact site, snapshot, field projection,
approved destination identity and sealed output and manifest
digests through the 0100 export and 0030 file ledger. Use the
0118 protection and current rights and erasure checks again before
showing evidence. If the ledger and exact destination prove one
publication, show its original receipt and write nothing. A prior
verified receipt remains history while access permits it. A
currently unreadable destination leaves current file presence
unverified.

When the original publication remains unproved, show “Export could
not be confirmed. Check the original export.” Keep the unresolved
operation held if its destination is unreadable, replaced or
unverified.
Leave existing bytes and other destinations untouched. A later
publication uses the existing 0100 approval and exact destination
checks only after reconciliation proves that the original file
effect did not occur. No evidence is released after its rights
or erasure check fails. If 0032 identifies this export as an
affected managed copy, keep its erasure receipt separate. A
detached destination or a changed drive letter supplies no
absence proof. Use the exact approved 0038 volume identity.
Keep the receipt waiting while the exact managed copy cannot
be checked. Use 0032's erased outcome only for verified removal.
Its nothing-found outcome needs verified absence at the exact
managed identity. Copies already in the recipient's custody lie
outside that proof.

## LP-01 Paid Local Search Preview

Free snippet layout remains local, fast and available with no
credentials. Show the preview profile, selected title, description,
breadcrumb, date and supported eligible rich-result examples.
Label it “Example search snippet”. It is a local rendering
estimate. Display paint bounds under its pinned font, locale,
device and width. Keep uncertain facts unknown. Never promise
which snippet, result or rich feature a provider will show.

The separate paid path uses one exact Google Local Finder or
Google Maps result request at a time. Disabled by default. The
owner-only switch `local_preview.paid_enabled` (proposed name; off;
never-changeable class, DR-079, so the CLI, MCP and agents cannot change
it) enables the route, and a local credential grant in Tijori is also
needed. Each run sends the exact place query outside the PC.
No auto-lookup from an audit, typing, save, link click, cache
miss, expired receipt, device toggle or reopen. Version 1 has
no queued task, recurring job, automatic retry, provider map
embed, listing contact, external asset request or result-linked
crawl. Display only the dated returned sample after validation.

Official contracts inspected 21 November 2026. The following
prices are reference amounts. Only the current signed tariff
and pinned compatible request profile can quote a paid run.
Recheck the route, limits, licence and commercial terms before
admission and hold on tariff expiry or pricing drift.

| Surface | HTTPS POST on fixed host `api.dataforseo.com` | Fixed depth | Max quote per task at inspected rate |
|---|---|---|---|
| Google Local Finder | `/v3/serp/google/local_finder/live/advanced` | 10, one billable result page | 2,000 USD microdollars, $0.002 |
| Google Maps | `/v3/serp/google/maps/live/advanced` | 10, within the 100-result billing block | 2,000 USD microdollars, $0.002 |

The wire array has one task with the allowlisted exact fields
`keyword, location_code, language_code, device, depth, tag`.
`device` is desktop or mobile. Pin the documented operating-system
defaults, Windows on desktop and Android on mobile. The Maps profile
pins both search_this_area and search_places at their documented
true defaults. Show the requested market as such. A result may
represent another scope. Refuse a conflicting response scope. A
sample cannot establish exact locality or complete market coverage.
Choose a city or broader area from an admitted code catalogue. Bind
its version and resolved place label in the owner preview. No coordinates,
home location, postcode, origin URL or private site content.
One manually entered public keyword of 1 to 80 Unicode scalars
and at most 160 UTF-8 bytes after NFC and single-space folding.
Allow letter, mark, digit, single spaces, apostrophe and hyphen
under the named gate. Reject all escapes, colon, operators,
URLs, contact, personal or unproved private material using the
0132 source and privacy gate. Neither draft values nor audit
text can enter this field automatically. Fill `tag` with a
random opaque 0132 task label devoid of site or actor data.
Require allowlisted market, resolved device and exact outbound
confirmation. Reject raw keys, arbitrary endpoints, redirects,
billing multipliers, URL resolution, and returned links in
outbound or automatic request fields.

Keep one outstanding LP request per installation, at most one
handoff per 60 seconds per billing account and at most five paid
LP handoffs per account per UTC day. Reuse the stricter existing
account/installation limits shared across 0132/0133. Include
LP reservations in the common exact daily and monthly site and
account spend transaction. Cap expiry and UTC rollover are
coordinated under 0132. The maximum is one base unit,
2,000 microdollars at the inspected tariff. Depth 10 uses one
billable unit. Forbid pixel ranking and operator surcharges.
Fixture pricing tests include zero remaining, exact cap, concurrent
other route reservation, and one microdollar short. No estimate
relies on an expected refund. One owner approval for an exact
plan expires after five minutes or the earlier tariff or scope
expiry. Consume it at the dispatch claim. Non-auth DNS and
socket retry settings cannot resend the paid POST.

Use the existing pinned v3 validator and bounded TLS transport
authority. Adopt a 90-second wall timeout, one active request
and a 4 MiB decoded JSON cap, subject to a stricter shared limit.
Validate exact envelope, one task and the one documented result
array slot or documented empty response. Check task path and data
echo, target, tag, task time, status and exact top/task cost agreement.
Parse provider cost from JSON decimal into exact integer USD
microdollars under 0132. Reject overflow, negative or fractional
microdollars and any cost whose provenance cannot be verified.
Once the billing envelope is proved, a charge over the approved
maximum freezes paid dispatch even if item validation fails.
An absent scope with returned items supplies no result. Refuse anomalies
and record the billing state separately even when no result remains.
Project up to ten permitted result rows in provider absolute
position order. These ranks are provider sample positions, including
other element types. In Local Finder, use a boolean is_paid from a
local_pack item only. In Maps, derive the classification solely from
maps_search or maps_paid_item. Withhold a conflicted classification.
Group position is within the item type. Received count is the
validated items array length. Eligible count is the number of
items permitted after route, type, scope and privacy checks,
before the display cap. Provider-reported count is the optional
result items_count, labelled separately. se_results_count does
not supply a market estimate. Only a documented
user-rating object can supply value, max and votes; never use hotel
class, rating distribution or nearby data. Missing values stay
unknown. Preserve gaps in absolute positions and withhold duplicates
with conflicting evidence. Omission does not prove absence.

| Record | Fields beyond the 0132 spend and site evidence records |
|---|---|
| `local_preview_plan` | `site_id, preview_id, surface, canonical_keyword, location_code, language_code, device, target_catalogue_revision, fixed_depth, request_fingerprint, approval_digest, tariff_revision, maximum_microdollars, credentials_generation, grant_generation, created_utc, expires_utc, explicit_refresh`. |
| `local_preview_snapshot` | `site_id, snapshot_id, task_ref, licence_profile, wire_profile, payload_digest, source_surface, market_and_device, provider_result_utc, received_utc, complete_or_partial, received_items_count, eligible_items_count, reported_items_count?, returned_rows[], freshness_deadline, deletion_deadline, ledger_id, erasure_epoch`. |
| `local_preview_row` | `rank_absolute u16, group_rank u16?, paid bool?, safe_listing_label UTF-8 ≤160 bytes?, rating_kind enum?, rating_value Decimal?, rating_max Decimal?, rating_votes u32?`. Accept rating only if exact documented scale is known and values in range. A missing or unapproved value has no default. |

Only display business labels after rights, privacy, redaction and
safe-text checks. Drop phone, address, images, coordinates, place
and client IDs, profile links, user comments, snippets, query
echoes not required for matching and raw JSON before storage.
No snapshot forwards instructions or automatic website links.
Cache sealed permitted evidence under exact route/site/fingerprint
for 24 hours of freshness and keep eligible stale results for
30 days maximum. Reopening uses saved evidence and date. Only
explicit Refresh can start a new plan; a selected run never
reuses approval. On revocation withdraw reads and fence jobs.
Apply 0132's uncertain-billing identity across fresh tags,
keys, credential rotation, pause and restore. A sent request
with lost response retains the full uncertainty hold and cannot
be retried. Cost beyond the allowed maximum freezes paid
dispatch and uses the common accounting recovery. Expose the
owner's “Billing is uncertain” notice with the retained amount.
A lost, denied or capped run falls back to the free layout and
current allowed cache. No stale result can become a fresh claim.

[Local Finder contract][finder] · [Local Finder prices][finder-price]
· [Maps contract][maps] · [Maps prices][maps-price]

## MCP and acceptance

All actions are static 0020 registrations after exact permission
and projection admission. Scope the laws in 0140. Return safe
failure before scope lookup. The owner-only name view and
provider billing receipts remain outside MCP.

| Action | Scope and request | Result |
|---|---|---|
| `audit_findings` | `audit.read`, site implied, snapshot, eligible categories, cursor | Rule, category, restricted evidence summary, disposition, date and cursor, max 50. No aid to mutate or rerun. |
| `audit_url_summary` | `audit.read` plus exact projection grant, snapshot, opaque page ID, up to 16 eligible field IDs | Typed values, known/absent status, source class, coverage. No raw material or native IDs. |
| `prepare_local_search_preview` | `audit.local_preview.plan`, same normal typed request and refresh flag | Opaque plan state or cached snapshot ID. Agent terms are untrusted and do not dispatch. Owner reviews and may confirm only in app. |
| `read_local_search_preview` | `audit.local_preview.read`, same-site approved opaque plan or snapshot ID | Safe state, source, date, permitted row count and approved scrubbed rows only with separate projection grant. No charge history. |

| ID | Synthetic acceptance |
|---|---|
| AT-AG-01 | Same coverage, repeated aliases, missing, denied and unknown facts; counted once per grain, unknowns separate. Invalid predicate, wrong site, over-limit and revoked cursor read no facts. |
| AT-AG-02 | Compare found-to-clear, recurrence, changed scope, changed severity and unread page. Show the stated labels, one Fix List key and no inferred repair or rerun. |
| AT-AG-03 | Join property-matched supplied Matomo plus a wrong property, overlapping sample and open period. Hold the latter and record no Matomo request. Verify CrUX URL and origin separation. |
| AT-AG-04 | Export safe CSV/ZIP, PII, formula-like labels, boundary rows/bytes, unsupported PDF and erasure between stage and rename. No partial archive, active field or late publish. Lose the CSV reply after atomic publication. Restart and verify the exact ledger operation, destination and sealed output and manifest. Show one original receipt and one file. Withhold completion when the first publication is unproved and the destination is unreadable or replaced. No duplicate file, overwrite, alternate destination or release of withdrawn evidence. Keep a previously verified receipt only as permitted history when its current destination cannot be checked. An affected managed CSV on a detached volume leaves its 0032 erasure receipt waiting. Verify removal or absence only at the exact managed identity. An earlier owner copy is outside that proof. |
| AT-LP-01 | Fixture exact path, depth, device, market, price, keyword, approved bytes and one call with each endpoint. Reject an operator, coordinate, injected secret, second task, sandbox-to-paid fallback and changed value. |
| AT-LP-02 | Decline, expired quote, pause, wrong role/site, cap equality/one over, and account race with 0132/0133. Observe zero unauthorised dispatch. |
| AT-LP-03 | Lose acknowledgement, restart, rotate secret and approve a fresh tag. One paid POST in total, reservation remains until verified reconciliation. |
| AT-LP-04 | Parse valid, partial, empty, changed echo, >4 MiB, unexpected status, compensated charge, cost over estimate, planted URL and private details. Admit only eligible values and keep missing unknown. |
| AT-LP-05 | Reopen fresh and stale Toolbox samples. The original date stays and the stale sample is labelled. Keep a separately authorised 0133 snapshot for the same site, billing binding, term, market and device. Withdraw only the Toolbox evidence grant. Retain its bytes and the MCP action scopes. The free layout stays and the withdrawn paid card disappears. MCP prepare returns no withdrawn cache ID. MCP read by known ID returns only the safe refusal with no source, date, count or rows. 0133 refuses that ID and still reads its own allowed snapshot. Reopening buys nothing. Try MCP approval and stop during a response. No paid retry or revoked delivery. |

Quint composes the 0132 budget claim with INV-0140-AU-01 through
INV-0140-AU-06 in [0140][suite]. Model view, plan, approval, hold,
dispatch, unknown, reconcile, sealed and erase with two billing
scopes. A trace
may contain a single paid handoff per claim only when exact
approval and pooled money authorise it. Replay returned costs,
pauses, no-op cache reads and races with fake transport on Linux
and Windows. Release requires all applicable 0140 tests,
agreement between app and exported eligible values, and proof
both routes obey independent site scope and the single spend gate.

[seo]: 0044-seo-signals-crawler.md
[structure]: 0047-site-structure-checks.md
[alekh]: 0043-search-charts.md
[fix]: 0048-fix-list.md
[crawl]: 0141-audit-crawl-logs-and-technical-toolbox.md
[lab]: 0142-local-performance-lab.md
[text]: 0144-site-corpus-and-text-mining.md
[catalogue]: 0145-audit-rule-and-field-catalogue.md
[finder]: https://docs.dataforseo.com/v3/serp/google/local_finder/live/advanced/
[finder-price]: https://dataforseo.com/pricing/google-serp/google-local-finder-serp-api
[maps]: https://docs.dataforseo.com/v3/serp/google/maps/live/advanced/
[maps-price]: https://dataforseo.com/pricing/google-serp/google-maps-serp-api

[suite]: 0140-unified-site-audit-suite.md#verification-design
