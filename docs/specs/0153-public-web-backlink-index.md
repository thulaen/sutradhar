# 0153: Full public-web backlink index design

The index works back from a destination page to the public pages
observed linking to it. A wide crawl discovers pages independently
of the owner's enrolled sites. A site owner can search the eligible
observations for their verified destinations and see the source,
dates and gaps in coverage. Each generation describes its own
eligible crawl window. Undiscovered, changing, blocked, restricted
and excluded pages remain outside any completeness claim.

Figures: every numeric limit, threshold and budget in this spec is proposed
until measured on the Dell or settled by a QR.

## Boundary and adoption

This separate design uses the admitted 0082 fetcher and legal wall.
[0152][live] owns the ordinary 0045 named-lead live profile. That
profile gives no authority to the public-web index. [0154][slices]
owns the offline site-slice hand-off.
[0046](0046-web-link-study.md) owns the site's Link study. Index
adoption needs an exact scope decision under QR-238. ADR-0022
excludes a hosted web index, DR-074 and spec 0116
LS-04 exclude multi-machine crawl in version 1, and DR-028 excludes
a remote bridge. DR-062 and DR-085 protect one-off rival scope. The
current exclusions govern until the protected scope adopts this
design. No crawl, monitoring, bridge, connector, spend or disclosure
authority follows from this document.

A private single-machine index is the reference deployment. A future
privately operated worker pool needs a separate adoption of that topology.
The backlog can include permitted public hosts beyond the enrolled sites.
It grows through approved seed expansion and scheduled fresh generations
within the adopted rules for each address. Expected coverage and refresh
time follow measured legal and capacity limits. For scale context,
Common Crawl's March 2026
archive reports about 1.97 billion pages and 345 TiB uncompressed.
This is a sizing reference. It supplies no input or retention right.
[Common Crawl release][scale-source].

[live]: 0152-live-backlink-crawler-profile.md
[slices]: 0154-backlink-index-site-slices.md
[scale-source]: https://www.commoncrawl.org/blog/march-2026-crawl-archive-now-available


## Crawl admission and scheduling

Pin the signed policy digest, owner, purpose, admitted discovery sources,
public host and address rules, time window, data classes, retention,
identity, resource profile, expiry and revocation generation. Record
which legal evidence supports each permitted scope. Open host discovery
needs an express scope in that policy. Links and eligible map rows may
create inert leads. Each lead needs exact scope proof and 0082's legal
wall before its first network request and at every redirect.
A periodic rival schedule needs an explicit superseding decision.
Pause, expiry or unknown authority blocks the next request. No discovery
or benchmark result can renew its own authority.

### Operator plan and controls

Open **Public-web index → Plan** on the operator's approved machine.
Show scope, regions, discovery inputs, eligibility and refresh rules,
the fixed declared crawler identity, storage destinations and ceilings.
Show no live start control when adoption or qualification is unproved.
Use **Preview and approve** to bind the exact plan to collection
authority. Show request, time, retained data and cost estimates, and
which estimate lacks evidence. Display frontier depth, the oldest
eligible lead, last completed generation and observed coverage once
running. **Pause collection** holds new sends. **Cancel this run**
releases proven unused reservations and retains actual and uncertain
charges. The latest eligible published generation remains readable
under its own current grant. **Clear index data** fences reads and
jobs before the 0032 erasure acknowledgement.

The signed `IndexOperatingProfile` binds the schema, index instance,
operator, jurisdiction, fetch identity, scope and legal policy,
admitted seed classes, refresh rules, exact platform and storage
placements, and evidence digest. It fixes inclusive maxima for
frontier rows, origin and address claims, attempts per UTC day,
rolling received and decoded bytes, per-response bytes, distinct
eligible targets per source capture, elapsed deadline, in-flight
attempts, working memory, retained physical
bytes, temporary staging, managed copies and spend per charge period.
It fixes minimum host and address gaps, quiet hours, observation and
receipt retention and approval expiry. Absent or unverified fields
hold collection. Changing scope, identity, placements, qualified
ceilings or refresh rules requires re-admission of the exact profile.
The 0152 named-lead limits grant no public-web collection.

Reserve the physical peak on each admitted volume before stage,
compaction or copy. The reservation covers retained bytes plus all
unwritten reservations and this operation's incremental worst case.
Require that sum to fit the
profile cap. Require unwritten reservations plus the new reservation
to fit verified free bytes on the exact volume. Apply checked integer
arithmetic and the 0030 ledger. Insufficient capacity holds
new claims while eligible generations remain readable. Claim expiry
does not reclaim a potentially spent network reservation. A confirmed
owner resume rechecks current deadlines, counters, profile and legal
policy before any claim can dispatch.

### Frontier and evidence

Use only discovery inputs named in the admitted operating profile.
A versioned local manifest of operator seeds, a separately admitted
offline public link map, a link from an admitted public capture or
a row from an admitted sitemap may supply a candidate. Each source
needs its own rights, codec, decoded bounds, and current scope.
The map or sitemap supplies a lead only. Protected 0120 referrer
material and site- or client-private rows cannot enter the frontier.

| Record | Required fields and authority |
|---|---|
| `DiscoveryInput` | Index instance, policy epoch, input ID, source class, immutable digest, source and rights revision, capture or receipt reference, canonical URL profile, decoded bytes, observed time and admission result. |
| `SeedCandidate` | Input ID and stable ordinal, exact 0152 canonical source key, source scope, discovery time and eligibility reason. Store no raw payload. |
| `FrontierEntry` | Index instance, scope and source key, first admitted discovery receipt, approved due instant or none, exact current policy and rights generations, and current state and lease reference. |

Verify input, rights and bounds before the durable input commit.
Replaying the same input ID and digest returns its receipt and adds
no rows. The same ID with different bytes refuses. Resolve canonical
identity before reserving frontier storage. Deduplicate by instance,
scope and canonical source key. An address admitted under a different
scope cannot supply permission or coverage here. When multiple
eligible inputs offer the same key, preserve the first committed
eligible discovery receipt. Retention or rights withdrawal applies
to that lineage before further use. A newly reviewed source can
supply later eligible provenance through its own admission.

The operating profile fixes admission limits for encoded and decoded
seed bytes, candidates per input, and aggregate queued frontier rows.
Charge every decoded candidate to the input limit before deduplication.
Refuse an oversized input before adding any of its candidates.
Reserve the exact new frontier slots and staging bytes before
committing the input and its candidate rows together. If they
cannot fit, keep prior rows, record `limit_reached` and hold the
whole new intake until its capacity is proved. A rejected input
or omitted lead supplies no source-absence evidence.

Give each candidate a canonical key for its exact public address and
policy scope under 0152. Verify canonical identity before deduplication.
Use the existing durable scheduler and a disk-backed frontier with one
bounded memory window. A record moves through Discovered, Held, Ready,
Claimed, Committed or Refused. Removal fences the old scope immediately
and marks the record Erased after verified cleanup. A source correction
or new approval is a fresh scope decision with preserved lineage.

A claim binds the exact authority, source version, host queue, lease and
reserved request, byte and storage budget. Commit the claim and compliance
intent before sending. Recheck current authority under that lease before
request, spill and publication. Atomically bind the result to the original
claim, current authority and coverage. An exact retry reconciles existing
evidence. An uncertain dispatch holds its reservation and cannot start a
second fetch. A gap or refused read gives unknown coverage. A missing
edge from an incomplete read gives no evidence of link removal.

Keep capped queues and budget counters across the whole local crawl and
each host. Group shared servers under 0082's reservation and wait
rule with the admitted index profile. Apply the strictest host gap,
global rate, decoded-byte, disk,
memory and elapsed bound. Wait for admitted backoff and stop for blocks.
When a limit is reached, hold or stop new work and record the resulting
coverage gap. Workers cannot widen scope or allocate a second allowance.

Stream page bodies only through protected, bounded memory. Require a
bounded response deadline and release buffers on commit, refusal, pause
or cancellation. A stuck reader is cancelled within its admitted bound.
Keep the scrubbed link receipt and permitted coverage only. Keep raw
page bodies outside durable storage, traces, exports and replay bundles.
Unproved cleanup leaves that worker unavailable and triggers reconciliation.

### Refresh and progress

Only a separately adopted recurring policy can create a later
refresh generation. Bind its occurrence to the exact index instance,
profile and policy revisions, schedule ID, 0019 occurrence and UTC
window. Reserve at most one run per occurrence. Replay reconciles that
run. Routine reads, slice imports and a missing edge open no run.
A held, missed or expired occurrence releases no catch-up request
unless the active schedule and current grant independently admit it.
A replacement policy cannot revive an old occurrence.

Show counts from committed records for queued, held and assessed
eligible sources, and separately show attempts and uncertain sends.
Count a source once per frozen census. Do not count retries as new
sources. Show the observation window, current budget and oldest
eligible pending discovery with provenance and metric freshness.
These measures describe the index run. An empty frontier says
nothing about undiscovered addresses. Show an estimate only from a
current measured operating profile over the same scope and units.
Missing or stale throughput leaves duration unknown.

### Cost and synthetic reference

The preflight shows the current signed caps and estimates requests,
decoded bytes, disk, memory, elapsed time and operating cost separately.
Use measured throughput from the exact profile, report ranges and list
unpriced inputs as unknown. Reserve capacity before execution. If actual
use exceeds an admitted bound or cost proof expires, hold new claims.
The signed benchmark binds build, scheduler, policy, hardware, workload
generator and observed peak usage. A synthetic run proves behaviour for
that profile. A capacity estimate supplies no live permission.

A finite refresh promise needs a plan assigning each selected source
to one UTC day and one disjoint stratum, a group that uses the same
qualified operating conditions. Pin the stratum rules, matching
measurement windows and their expiry to the profile. Count each
distinct eligible complete source at most once per window. Attribute
each charged attempt to its owning stratum once, including policy
reads, retries, redirects, and unresolved sends. Include the whole
measured elapsed time, with waits, stalls and downtime.

For each day and stratum, let N be the planned source count. Across
current matching measurement windows, take the lowest ratio of
completed sources to elapsed seconds and the highest ratio of charged
attempts to completed sources. Keep zero-completion windows in the
comparison. N equal to zero contributes zero. For N greater than zero,
zero completions or absent proof
makes the estimate unknown. Otherwise compute the time estimate as N
divided by the completion rate, and the attempt estimate as N times
the attempt ratio. Round each up to a whole unit, using checked integer
arithmetic on the underlying counts. Sum the estimates for the day.
Compare time with the intersection of that UTC day and the permitted
schedule window, excluding known periods when dispatch cannot occur.
Compare attempts with that day's verified cap less spent and
outstanding reserved attempts, counting any uncertain attempt once.
Equality fits. An unknown counter or exceeded bound holds the finite
refresh promise. Live reservations and per-request fences still apply.

Show “Time to cover this scope is unknown” when a required estimate
or finite census is missing. Separately admitted bounded collection can
still report partial coverage and the observed count and age of assessed
pages. If the measured plan cannot fit, show “This refresh needs more
time or a smaller scope”. Show physical storage required for the
active generation, successor staging and the peak of each managed
copy on its own placement. A new host, worker or volume adds no
capacity without its own admission.

For the synthetic reference, use four fake hosts and a fixed set of
200 eligible leads. Allow four global workers, one request per host,
a minimum 30-second host gap, 256 KiB decoded per response, 16 MiB
decoded in total and 64 MiB for worker and queue memory together. These
are fixture values. Test the host gap on a synthetic clock and verify
the response and total-byte boundaries independently at and one over.
Race duplicates and a crash around send and commit. Hold a lost reply,
a cross-scope redirect and revoked authority. Verify no raw bytes in
stores, no released record for an unproved read and no unsupported
site or whole-web coverage claim. Require measured results before
choosing live caps.

## Graph and generation contract

One crawl controller holds the authoritative policy, ledger and budget.
Partition the frontier by the verified shared-server group. A worker
gets a bounded lease with fence, expiry and immutable exact target.
Serialise request claims for the same host and shared address at the
controller. Stop new work when that authority is unavailable. Reclaim
only work proved unsent. Cancel and hold an uncertain dispatch.
Workers receive no site privacy data or shared vault root. Reusing a
lease or adding a worker cannot enlarge the quotas. A deployment on
more than one machine still needs separate adoption.

Write each EdgeObservation with source and target keys, exact capture
ID, observed UTC time, admitted rel flags, authority and proof refs.
This index has its own multi-target projection from the 0082 stream.
An ordinary target-only Receipts run cannot supply this projection.
Within one exact capture, count distinct eligible canonical target
keys after policy checks. Qualify only targets proved by the pinned
parser and 0152 key rules. Reserve a bounded map and stage space for
at most the profile's per-source cap. Fold repeated target keys into
one pair using the 0154 edge semantics. The cap includes eligible
external links whether or not they match an enrolled property. A
link does not authorise fetching its target. A clipped projection
can retain verified positive observations inside its bounds with
`projection_clipped` coverage.

Commit a complete read only on proven end of the exact eligible
response, a successful full parse and complete collection for the
scope. Equality with the cap can complete if the stream adds no
further distinct eligible key. On a further distinct eligible key,
close the projection as clipped. A truncated response, failed parse
or unproved source revision yields incomplete coverage. A cap or
incomplete read never marks a previously known pair absent. Keep the
raw response in bounded volatile memory. Use the 0152 canonical key
and 0154 edge and coverage semantics.
Obey noindex, nofollow, nosnippet, noarchive, current opt-outs and
expiry according to the admitted legal profile.

Sort immutable edge partitions by target key, source key and capture
ID. Build a target lookup for incoming links and a private source
lookup for withdrawals. Deduplicate exact observations before
aggregation. Resolve each directed pair over its eligible seen and
complete absent observations in the 0154 capture order. A later
complete absence changes the dated state. A blocked or incomplete
read without a verified positive changes only checked-date and
coverage evidence. It cannot establish a link's absence. Publish a
signed generation manifest only after validating all
partition content digests, measured coverage and the erasure floor.
A read observes one generation. Withhold an affected old or staged
generation during verified withdrawal. A source can affect a
projection through census or assessed-set membership even when it
has no edge there. Do not answer from a mixture of partitions.
Keep eligibility and coverage lineage in rebuilds.

Estimate footprint as queued rows, both indexes, WAL or equivalent
journal, compaction peak, replicated partitions, staged generations
and each managed recovery copy. Count physical bytes as well as
logical result rows. Measure each class and the full peak before
signing a live capacity profile. Report throughput, oldest pending
eligible address, attempt cost, catch-up time and measured query
latency with scope. Missing throughput gives no completion date.
Compaction failure keeps the former eligible generation and holds
new work at the storage bound. A capacity profile fixes storage and
cash ceilings, and a budget lapse starts no new claim.

### Coverage census and publication

Each generation freezes a `FrozenSourceCensus`. It binds the index
instance, generation, source-scope policy, target projection digest,
policy generation, UTC half-open observation window, frozen time,
and the exact sorted set of eligible canonical source keys. Sort by
canonical UTF-8 bytes and deduplicate. Record the reason for each
exclusion and unresolved lead under the protected operator policy.
The fence seals membership and scope at freeze. Leads admitted
after freeze may enter a later generation. Missing frozen members
remain gaps in this one. An opened web frontier alone cannot bound
all sources that might link to the registered destination.

The verifier builds the assessed set independently from complete
eligible read receipts for the frozen set in the same window and
policy. Assess at most once per source for coverage. An admitted
complete read can contribute to both the assessed set and edge
evidence. A blocked, truncated, expired, erased or skipped read
cannot contribute to the assessed set. A read outside the window
also cannot contribute. A clipped edge
projection cannot claim a complete target view, even if all source
reads completed. A failed generation receipt cannot prove zero.

Write a protected `CoverageWitness` with source-scope digest,
projection digest, frozen census commitment, assessed-set
commitment, eligible and assessed counts, coverage class, and
bounded gap codes. Allow `blocked`, `incomplete_read`, `expired`,
`withdrawn`, `unread`, `limit_reached`, `open_scope` and
`projection_clipped`. Store distinct codes in canonical byte order.
Commit each set with the index's generation-scoped HMAC-SHA-256 key.
Use canonical JSON per RFC 8785 for a typed record with
`kind` ("census" or "assessed"), instance, generation,
source-scope digest, projection digest, window and sorted keys.
Keep the key and source sets in the operator's protected store. Select
`complete_for_slice` only for an expressly admitted finite,
independently enumerated census, after exact set equality and
complete authorised projection have been verified. A moving
discovery scope is partial even if its current queue drains. Expose
only the 0154 disclosure-safe attestation. The independent
generation verifier authorises that exact descriptor after checking
partition digests, source receipts, the projected payload digest,
row counts, recipient, scope and the current floor. The
generation manifest binds the SHA-256 digest of each canonical
protected witness to its projection digest. Refuse a substituted
witness before signing, even for the same generation and projection.
Its signing grant is distinct from the slice publisher’s.
Withdrawals fence the old witness and require re-verification.

## Security, authorisation and custody

### Ingest, outbound requests and publication

Bind each signed index authorisation to the operator, index instance,
collection purpose, admitted crawl profile and identity, scope and
exclusions, policy and key epochs, budget reference and expiry. Keep
separate grants for collection, generation publication and destination
reads. Check issuer, purpose, exact scope and currency at the owning
boundary. Reuse Tijori custody and the admitted signature verifier.
A grant for site management, an earlier run or another recipient
cannot authorise this index operation.

Before every fetch, redirect and retry, revalidate the current grant,
pause and stop generations, allowed address and transport, budget and
the 0082 crawler identity and legal wall under the admitted index
profile. Bind the durable 0082 request intent to those decisions.
A missing, ambiguous or superseded intent
holds dispatch. Send only the admitted guest request and declared
crawler identity. Keep site grants, private seed lists, raw credentials,
referring paths and user or member identifiers out of that request.
Recheck directives after each response before retaining any row.
Web content remains untrusted data and cannot control any worker.

Stage observations privately. A protected generation manifest binds
the instance, schema, source and policy generations, deletion watermark,
partition digests and the allowed projection profile. Publish by swapping
one verified generation at a time after rechecking grants, disclosure
and deletion state at commit. Fencing an affected generation withholds
it at read time, including cached and in-flight replies. A late fetch
or stale staged generation cannot restore access. Hold all reads of an
affected projection, including its apparently unaffected surviving
rows, until a freshly verified successor passes the current floor.
Independently eligible projections outside the withdrawal can continue.

A destination publication checks the current site grant and verified
registered destination before projection and again before release.
Restrict fields, counts, pagination and cursors to the projection and
generation. A cursor binds the destination scope and grant generation.
Use one safe refusal shape for missing and unauthorised scope. No
hidden totals, alternate filters, response identifiers or diagnostic
details reveal a restricted source row. A cold query reads approved
data already held. Missing local evidence causes no crawl or outbound
query, even when a caller presents a saved cursor.

### Encryption, withdrawal and restore

Apply [0118](0118-end-to-end-encryption-gate.md) to each protected store,
approved transport and replica. Use opaque Tijori handles and distinct
purpose keys. Separate ingest, publication and read identities. Bind
seals to the index and intended store or approved recipient. An
unproved protection profile keeps the route unavailable. Ledger every
managed partition, projection, cache, temporary file and approved copy.
Use [0032](0032-audit-erasure-diagnostics.md) for removal receipts and
[0038](0038-search-services-cloud-storage.md) for any separately approved
managed storage movement.

Apply verified erasure, withdrawal or a relevant policy denial to the
exact affected scope. Fence access before rebuilding projections and
counts. Purge affected frontier entries, source rows, derivatives and
managed copies under the owning retention rule. Evidence of a mere
fetch denial does not attest completed erasure. Verify each selected
copy's result. Unreachable copies or uncertain acknowledgements stay
pending or failed. At restore, keep the backup isolated until current
tombstones, grants and key epochs have been applied and verified outside
the backup. A restored reader exposes only rebuilt eligible projections.

### Formal and synthetic acceptance

Model authorise, stage, publish, read, withdraw, erase and restore in
Quint with two destinations, different grants, two generations and a
concurrent withdrawal. Require the live grant, current index profile
and 0082 decision at send, a verified manifest at publish, exact destination
scope at reply, and no exposure below the current deletion watermark.
Replay these outcomes in Rust through the qualified Quint Connect path.

- `IT-0153-FROZEN-CENSUS` compares three empty edge projections.
  Finish every source in one admitted finite set, cap the last read
  in an otherwise identical set, and drain one open discovery queue.
  Only the first has verified complete zero. Substitute a receipt from
  a different scope or outside the window. Admit a new lead after
  freeze and verify it enters only a later generation. Withdraw a
  counted member before projection and require a current witness.
  Substitute a different witness for the same projection after the
  manifest was sealed. Refuse the descriptor before signing.
- IT-0153-DISCOVERY-PROGRESS uses the capped synthetic crawl.
  Replay a valid seed input, offer an equivocal digest, repeat a
  canonical source under two scopes and try a protected 0120 lead.
  Apply the qualified limits at equality and one candidate or byte
  above. Verify atomic refusal, one row per admitted key, no fetch
  from rejected input, no eviction of existing queue records and
  a coverage gap when admission reaches the bound.
- IT-0153-EDGE-CAP uses a synthetic cap of three distinct targets
  for one source. Repeat an admitted target and complete the stream
  with exactly three eligible keys. Confirm full enumeration. Add a
  fourth key and verify partial coverage. Truncate the tail or
  substitute an earlier revision before recording completion. Keep
  the dated seen state of a prior pair missing from each partial
  capture. A later eligible complete read without that pair can mark
  it absent. An ordinary target-only receipt supplies no complete
  index capture, even when it contains all currently known links.
- IT-0153-REFRESH-OCCURRENCE proves that no open discovery or
  recurring policy means no dispatch from the corresponding route.
  In the separately admitted synthetic branch, replay one scheduled
  occurrence across restart, revoke its grant before claim and
  restore older state. Require at most one run, no stale dispatch
  and honest counts after a repeated attempt and unknown reply.
  Draining the open queue gives neither whole-web completion nor
  verified zero. Missing throughput shows no finish estimate.
- IT-0153-REFRESH-CAPACITY uses a finite census with two strata at
  qualified rates of one complete read per two and three seconds.
  Leave ten eligible sources in each. Include charged policy reads
  and stalled reads in the windows without counting them as complete.
  Verify the 50-second estimate and an admitted window of exactly
  that duration, then hold the plan with a window one second shorter.
  Replace one rate with zero or unknown and withhold the estimate.
  Repeat with an open scope. A second volume with free space cannot
  fill a shortfall on the approved stage volume.
- IT-0153-COLD-QUERY uses a registered destination, another owner and
  an unregistered destination. Plant restricted rows for both denials.
  Compare cold and warm queries, counts, cursors and safe diagnostics.
  Confirm no restricted data escapes and no request leaves the process.
- IT-0153-WITHDRAW-RESTORE withdraws a source during publication and
  during reply formation. Replay an older copy and lose one deletion
  acknowledgement. Put a withdrawn link and a surviving link in the
  same affected projection. Hold both, its totals and its cursors.
  An independently eligible projection stays readable. Rebuild above
  the current floor and publish a verified successor. Resume only
  eligible rows. Require no resurrection and a pending receipt for
  the unproved copy.
  Repeat after publishing complete zero for a finite census in which
  the withdrawn source has no edge. Hold its coverage proof and zero
  label even though no visible edge needs removing.

Run release fixtures on the admitted platform and smallest memory
profile. Compare against a synthetic graph with known incoming edges.
Cross-check two publishers, frozen partitions, targeted withdrawal,
a timed-out worker and stale-concurrent claims. Verify complete zero
only within the exact admitted source coverage and 0154 disclosure.
An unproved build, privacy or legal result grants no live use.

Public native add-on footprint is zero. This index reads outside
public pages and exchanges only the separately approved offline
projection. It adds no code, queries, bytes or requests to an own
site's public page.
