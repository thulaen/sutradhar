# 0135: AI fetches, referrals and ranking evidence

The WordPress plugin and XenForo add-on show verified AI bot
fetches and separately count human visits referred by an AI
assistant. Both are built in. Reports identify what the site
observed and when its coverage was incomplete.

DR-122 permits an additional post-relevance signal. A verified fetch can raise
the order score of eligible content by a configurable amount. The default
weight is 0 (no boost) until an evaluation supports a value (DR-160); the
range is 0 to 500 basis points (0 to 5%). Referrals never enter that signal.
Nothing changes an AI provider's results.

Example: a forum's XenForo add-on sees OpenAI's GPTBot fetch a public thread.
The request matches the published ranges in the signed verifier, so one record
is appended after the page has been sent. The owner's "AI traffic" page later
shows
"Verified AI fetches: 3 this week (coverage complete)", because the
privacy release allows a count that small; otherwise it reads "Not
measured". The thread's
order score rises by 5% (an owner-set weight; the default is 0) for 28
days, then reverts. A
visit from an AI assistant's link is counted separately and never raises any
score. All figures are illustrative.

## Words used here

| Word | Meaning |
|---|---|
| Verified fetch | A served public revision with proof of the AI agent and its role |
| Human referral | A confirmed human visit that came from an admitted assistant host |
| Fetch fact | The exact rank-facing proof: one positive flag for one revision |
| Verifier | The signed list of agent names and published address ranges used to prove a fetch |
| Coverage | Whether the day's observations were fully collected |
| Spool | The site-owned file where records wait for the service to collect them |
| Basis point | One hundredth of one percent (0.01%) |
| Generation | A counter that rises each time a list or setting changes |
| Fence | A stored minimum that stops older data being accepted |
| Projection | A prepared view of data for one purpose |
| Differencing | Working out a hidden number by subtracting visible ones |

## What this spec designs

Native observations and signed exchange, site-scoped report views, and
one verified-fetch fact for post-relevance ordering. Darshak owns human
classification and referral counts. Spec 0027 owns ordering admission.

## Ownership and request flow

Use the existing guest visibility and native content-identity
contracts in 0021. Darshak in 0034 owns human classification
and all referrer counts. The PHP add-ons collect only the
minimal site evidence described below. Rust owns verification
admission, protected aggregation and ranking. The desktop
owner approves site enablement, ranking settings and proofs.
The built-in native admin pages show the admitted site's
cached results. No extra plugin, extension, cookie, external
script, callback or public control endpoint is needed.

Collection starts off until the site's notice, relevant
consent and recording mode are confirmed. No protected,
staff-only, conversation, draft or preview request qualifies.
Respect 0034's opt-outs and privacy mode. A denied or missing
policy refuses collection. Never use evidence of a fetch to
relax an access decision or change a robots rule.

### Verify an observed fetch

Observe an ordinary native guest GET that successfully served
a supported public content revision. Bind native completion,
HTTP 200, returned resource, exact revision and active
guest-visibility generation already resolved by that handler.
An unproved or cancelled response completion cannot qualify. A redirect,
HEAD, failed lookup, 304, prefetch, robots request or block
page cannot create a verified content fetch. WordPress binds
the canonical published post or page. XenForo binds the
thread view or a separately proved, directly served public
post revision. A thread view alone cannot prove that a given
reply was fetched. Unmapped, stale and ambiguous IDs yield no
positive fact. Do not copy a content body to verify identity.

Match a recognised agent token with its operator proof.
Use an admitted versioned copy of official IP ranges or an
equivalent admitted network verification under 0034. A name
alone establishes only an unverified claim. Use the actual peer or the
admitted exact proxy chain in 0034.
Reject forged, ambiguous or stale forwarding evidence. Use
only the local signed verifier in the request. No DNS or
network verification in the native hook. Analyse
the address in memory and discard it before any persistence.
No IP, full user agent, URL query, header value, visitor ID
or draft content leaves this collection path.

| Operator evidence | Handling |
|---|---|
| [OpenAI][openai] | Match the published ranges of OAI-SearchBot, GPTBot or ChatGPT-User to that exact token. Keep search, training and user-directed fetch roles explicit |
| [Perplexity][perplexity] | Match PerplexityBot or Perplexity-User to the separate published ranges. Keep crawler and user fetches distinct |
| [Anthropic][claude] | Admit each named agent only with current operator identity proof. A documented ClaudeBot, Claude-SearchBot or Claude-User name alone stays unverified |
| [Google][google] | Google-Extended is a robots policy token with no separate HTTP agent. An ordinary Googlebot visit cannot establish an AI fetch |

Reuse 0034's daily verifier refresh with a sealed, signed manifest of agent
token, operator, role, proof origins, effective and expiry instants, allowed
ranges, digest and verification generation. Search, training and user-directed
roles qualify only if operator proof and the observed token bind the exact
admitted role. Unknown roles do not qualify. Cap the admitted list at 16,384
prefixes total (proposed); a manifest with more is rejected whole and the
previous generation stays active. Normalise IPv4-mapped IPv6 addresses to IPv4
before matching. Ship the admitted verifier as a read-only compiled array
(opcache or APCu) so a worker does not parse it per request, and look up an
address by binary search over the sorted prefixes only after a recognised
agent token matches. Check signature and digest when the cache loads, and
compare the manifest expiry, last refresh and revocation with the clock on
every lookup; key the cache to the active pointer's digest. A clock that goes
backwards makes the verifier stale and stops positive classification. Every
time limit in this spec uses that host clock on the request path and the
service clock elsewhere. Reject ambiguity, overlapping conflicting agent
claims and expiry. Stop positive classification after 48 hours without a
current valid refresh. An explicitly withdrawn verification generation cannot
support a fact.

### Attribute a human referral

Use the existing 0034 start/end, opt-out and deduplication rules. Require a
completed `human` classification, with the first-party referrer input bound to
its exact consented start, site and served resource. In the admitted 0034
collector reject a referring address longer than 2,048 UTF-8 bytes (never
truncate it), and parse the rest in memory. Reject user information, control
and bidirectional characters. Require a safe HTTPS origin and retain the
canonical host (lower-case ASCII, international names in punycode) only for an
exact match in the versioned 0093 list. Ignore the beacon request's own page
origin or caller-defined host labels. No path, query or fragment is read into
a durable record. Missing, hidden or unrecognised referrer stays unknown. Bot
traffic, invalid consent, unknown classifier and unconfirmed sessions do not
count as human referrals. A visit label and a referral source describe
different facts.

Use host-by-content joint aggregates only when 0034 admits
that projection and suppression. Suppress groups below ten qualifying visits
and apply a
stricter admitted policy where required. Qualify equivalent
privacy protections for narrow or user-directed fetch cells. Protect
overlapping groups, totals,
filtering and stable release windows from differencing or
re-identification. A report lacking that proof offers only
an admitted coarser aggregate or “Not measured”. This feature
does not grant the opt-in referrer path collection in 0120.

## Records, exchange and lifecycle

| Record | Fields |
|---|---|
| Fetch observation | Site, native content key and exact revision, observed UTC date in YYYY-MM-DD form, verified agent ID and role, verifier generation, signed origin collection generation, opaque event sequence and no raw request material |
| Native verifier receipt | Site, verb, exact control revision and target verifier generation, kind and schema, manifest digest, admitted proof-set ID, generation, effective and expiry UTC, last valid refresh UTC, authority and key epochs and native commit outcome |
| Fetch report aggregate | Site, UTC day, admitted population and site or exact content scope, provider and role bucket when permitted, observed count or withheld reason, coverage source and verifier generations, privacy release ID and retention expiry |
| Referral aggregate | Site, eligible native content scope or site total, date basis and window, admitted host ID, human count or suppressed, source and coverage generations and privacy release ID |
| Fetch fact | Site, destination and exact public revision, verified positive observed flag, first admitted verified observation UTC date, fixed expiry UTC, proof ID, scope and expiry generation. No count or human fields |
| Site view receipt | Site, actor scope, UTC window, eligible content keys, approved report cells, coverage, methodology and freshness. Sealed and signed |
| Owner inspection | Capability and schema IDs, active setting descriptors, allowed safe values, effective versions, input and output lineage refs, source categories, redaction reason and authorised exact ranking trace refs |

Use the actual in-memory time for the native verifier checks.
Persist and send only the observation’s UTC date. The signed
origin epoch and sequence identify its exact occurrence. Use
the existing ordered service commit for first proof. If a gap or
ambiguous origin leaves that proof unverified, keep base order.
An older source without an admitted UTC-date and first-proof
binding cannot produce a ranking fact.

Write the record after the response has been sent (a shutdown step,
never before the visitor's page is served). The write is one append of
at most 512 bytes to the site's spool file, with no database call, no
network and no lock held across it. It has a time limit of 5 ms
(proposed); a slower, failed or refused write is dropped and counted as
a coverage gap. The spool is the bounded sealed host buffer below.

Spool file rules: keep it outside the document root in a directory owned by
the web server user at mode 0700, create it once with create-exclusive and
no-follow flags, opened through its parent directory's descriptor. Every later
open uses no-follow and append, and each open is checked on the descriptor: a
regular file, one link only, and the expected owner and mode, else refuse.
Append in a single write of at most 512 bytes on a local filesystem (network
filesystems are unsupported). The 4 MiB cap (proposed) covers the rotated and
the current file together. Check it from a size stamp each worker refreshes at
most once a second and from the open descriptor's size before each append
batch, and re-check that the path still names the same inode, reopening when
it does not. The add-on rotates the current file at 1.5 MiB (proposed) and
keeps one rotated file, which `ai_observation_pull` reads before the current
file; unread lines in it older than seven days are dropped and counted as a
gap. If the rotated file is still unread and under seven days old when the
next rotation is due, appends are dropped and counted as a gap instead of
rotating. Overshoot is at most one size-stamp interval of appends: workers x
512 bytes x appends per second x 1 second (for example 8 workers x 50 appends
a second x 512 bytes is 200 KiB). Two files of at most 1.5 MiB plus that
overshoot come to about 3.4 MiB, under the 4 MiB cap (all proposed). A host
that cannot open files through a parent directory descriptor cannot collect,
and the report says "Not measured". Rotation is one atomic rename under the
parent descriptor, and the new file gets the same descriptor checks. The
expected owner is the web server user and the mode is 0600; any mismatch is
refused. The service never opens the file: lines reach it through
`ai_observation_pull`. A host that cannot send the response before running the
shutdown step cannot collect, and the report says "Not measured". Rust treats
every line as untrusted when it pulls: it assigns the sequence (file offset
plus a random line ID) and rejects, counting a gap, any line that is
malformed, over 512 bytes, not UTF-8, names an unknown agent, role or verifier
generation, names a revision that is not current, is dated in the future or
over seven days old, or repeats a line ID. A partial last record is discarded.
Coverage comes from the service's own pull record and the dropped-write counts
in the add-on's signed status marker, not from gapless sequences.

### The four paired operation bodies

Use only the 0021 paired operation registry and the fixed 0118 sealed
2.0 POST route. The service starts each attempt. Operation names and
payloads stay in the sealed body. Apply the existing wire and frame
limits before allocation. Refuse legacy envelopes and unadmitted
adapter schemas. Public page requests perform no exchange or ranking.

The fields below form closed JSON records in the 0118 typed payload.
Names in backticks are exact. Preserve the admitted canonical codec
for existing site, content, revision and receipt identities. Missing
codec proof holds the profile. Encode each new spool line ID as
32 lower-case hex digits from 16 random bytes. Reuse that ID for
observing the same line. Digests are lower-case SHA-256 hex. Encode
a generation, revision counter, cursor offset or UTC second as
an unsigned 64-bit integer using
decimal text with no leading zeroes. A row count is a JSON integer.
Require every field of the selected case, including a field that
allows a null value. Refuse fields outside that case, duplicate names
and omitted fields. Use the existing canonical typed intent digest
for effects. The 0118 effect
frame supplies its logical identity, site, actor, source and target
revisions, approval and expiry. Effect replies echo its logical ID
and intent digest as well as the operation result below.

Each reply gives the operation's success variant or the closed
0135 refusal payload with just `outcome=refused`. Never echo the
rejected field or record. The 0118 effect echo still applies when
it can be authenticated. A missing, lost or unverifiable reply
stays unknown at the caller. It gives no committed effect or coverage
proof. Exact retries of a committed effect return its saved receipt
within current authority and the owning receipt window. The expiry
remains the original expiry.

#### `ai_observation_pull`

The request contains `site_id`, `origin_epoch`, `cursor` and
`max_rows`. The row limit is 1 to 512. A cursor contains `origin_epoch` and
`offset` at a complete record boundary. A null request cursor
selects the first retained record boundary of the named origin.
Refuse a changed origin or unproved boundary. Offsets stay bound to that origin
and cannot move to another record during pruning or restore.

The `outcome=batch` reply contains `site_id`, `origin_epoch`,
`requested_cursor`, `next_cursor`, `end`, `count`,
`first_sequence`, `last_sequence`, `verifier_generation`,
`coverage` and `rows`. Each sequence contains the native record
`offset` and opaque `line_id`. Each row contains `site_id`,
`content_key`, `content_revision`, `observed_utc_date`,
`verified_agent_id`, `role`, `verifier_generation`, `origin_epoch`
and `sequence`. Use the admitted site content key and bot role
types. The observation date is a valid UTC calendar date in
yyyy-mm-dd form. Carry no fine-grained observation time or raw
request material. Rows follow spool order in the requested origin.
Match the request, echoed reply and every row to the authenticated
paired site and requested origin. Both cursors, when present, must
carry that origin. Refuse a mismatched reply before updating any row,
coverage, cursor or acknowledgement evidence. This differs from a
malformed record rejected before sealing an otherwise valid batch.
Revalidate current native visibility before committing each row.
Ordering uses the first admitted date under the later ranking rules.

##### Native spool coverage

A pull's `coverage` has exactly `scan_from`, `scanned_through`,
`capture_end` and `status_markers`. The first three are logical byte
offsets in the reply's origin, as decimal text. They describe the
half-open extent examined and the capture end. Require
`scan_from <= scanned_through <= capture_end` and consistency with
the resolved request and next cursors. `scanned_through` equals
`next_cursor.offset`. Return up to eight markers, one current
revision per UTC date in the seven-day retained span and the day
at its boundary. Order by date. A capped or missing marker gives
no completeness for the omitted date. A marker identity is site, origin,
verifier, profile, date and revision. An empty list supplies no
marker proof. Repeated identity and bytes add no count. Changed
bytes under the same identity refuse.

Each marker is a closed record with `site_id`, `origin_epoch`,
`verifier_generation`, `coverage_profile_ref`, `utc_date`,
`marker_revision`, `extent_start`, `extent_end`,
`observation_state`, `start_boundary_ref`, `end_boundary_ref`,
`continuity_ref` and `dropped_writes`. Use `closed` only when every
boundary and continuity reference is verified. An `open` marker
has a null end and continuity reference. Use null for any boundary
not proved by the profile. `unproved` carries null proof references
and no complete count. The dropped-write count is
unsigned decimal text for the marker's exact UTC date, origin
and collection scope. Use null for `unproved`. Apply the pull's
date and counter encodings. Require `extent_start <= extent_end
<= capture_end`. Authenticate the exact marker in the existing
native 0118 signed reply. Reject a marker for a different site,
origin, verifier or coverage profile. Duplicate dates or an older
marker replacing a later one supply no current proof.

The start and end references jointly bind midnight at `utc_date`
and midnight at the next UTC date, the exact marker scope and
logical extent. `continuity_ref` proves the admitted after-response
path was continuously eligible for that projection between those
boundaries and every in-flight append disposition for that date is
resolved. A signed host pause, lost verifier or capture gap prevents
that proof. Never derive continuity from offsets or zero drops.
A zero-length extent supports full-day coverage only when both
proved day boundaries and uninterrupted capture resolve to it.
Do not join across changed origin, verifier or coverage profile
without an independently admitted complete span for that projection.

Assign the UTC date when the native after-response observation
is made, not when it is pulled or when its write finishes.
A late write can fall after an offset for the next day. Conservative
day extents may overlap. Group each accepted row by its bound date.
An older day with an unresolved in-flight write stays open or
unproved. On a date boundary, hold a new first-ranking fact if
earlier eligible proof for that revision may still arrive. An
already bound exact first proof keeps its original deadline.

The signed marker can cover only the supported after-response
path. An observed failed or refused append contributes to the
owning drop evidence and creates no fake spool sequence. Where
the native host cannot prove the after-response boundary or its
drop evidence, return unproved coverage. Preserve proved gaps
on restart. A missed response or absent callback is no verified
fetch. Persist no invented count for an event the host could
not measure. Use the existing bounded append and storage path.
Apply the existing 4 MiB stop and in-flight bound to retained
spool bytes. A logical offset can be larger. A size stamp or
missing record gives no completeness proof.

Sequence identity binds the origin, logical record offset and
line ID. Retention or acknowledgement pruning cannot reassign
an offset within the origin. A replacement that cannot preserve
positions needs the owning new origin fence. Invalidate old
cursors rather than joining them to new bytes. Reconcile proved
already committed positions through the saved pull and effect
records. An uncovered pruned or expired extent is a gap. An
erasure or restore does not reset a gap or authorise old proof.

After current validation, the service binds coverage to the
exact committed pull interval and compatible marker scopes.
Use the current eligible marker revision for each scope and date.
A later marker for it replaces the count in that projection.
A closed marker, fully proved extent and complete eligible
observations are all required for complete coverage.
A gap, rejection, nonzero drop count or unresolved partial
record prevents that result for the affected scope. Cursor,
sequence count and `end` describe the scan only. An empty
completed pull can support zero only with independent complete
coverage for the exact report scope. When the proof covers only
some days, report only an admitted covered interval. A host-side
collector cannot prove traffic the covered path never observes.

For IT-0135-SPOOL and IT-0135-FETCH, use a synthetic closed
empty origin with a proved zero drop count, then omit or stale
the marker with the cursor and end unchanged. Plant one cap
refusal and a pre-send failure without an after-response record.
Neither can produce a fetch or falsely complete the interval.
Prune committed rows, then age out an unproved interval and
restart or replace the spool. Keep verified receipts, refuse
stale origin and crossed offsets, and preserve the gap. Repeat
exact marker delivery and check it cannot count a drop twice.
Compare a full empty UTC day with a host enabled at 23:59 in an
empty origin and with a mismatched claimed date. Only the full
eligible day can support measured zero. Reserve a write on the
older date. Append the later-date row first, then resolve the old
write. Before resolution, the old date stays open and a first rank
fact on that revision waits. On resolution, use the earlier date
if the eligible old observation arrives. Exercise an origin or
verifier change at midnight and missing continuity after restart.

Echo the request cursor exactly, including null. Resolve a null
cursor to the first retained record boundary in this origin. If
the captured origin is empty, use its current append boundary.
Scan at most `max_rows` whole records in that extent. A rejected
record counts towards the scan limit. Use only whole typed records
that fit the 0021 operation limit after 0118 frame overhead.

The count equals the number of returned rows. Empty batches have
`rows=[]`, `count=0` and both sequence endpoints null. Nonempty
endpoints equal the first and last returned identities. Always
return a non-null `next_cursor` at the first unscanned record
boundary in the same origin. If no scan advanced, return the
resolved start boundary. Set `end=true` only if no further complete
record remains in the captured extent. Leaving a record unread at
the size or row bound gives `end=false` and a cursor at that record.
If the bound prevents any lawful progress with a record remaining,
return the existing bounded refusal. End gives no coverage,
measured-zero or acknowledgement proof. No acknowledgement follows
an empty batch. Use only a verified reply and its committed pull
record to acknowledge a returned sequence.

#### `ai_observation_ack`

The effect payload contains `origin_epoch`, `pull_reply_digest`,
`service_commit_ref`, `service_commit_digest` and `sequences`.
The last field lists the exact nonempty, ordered, unique subset of
sequences committed by the service from that one verified batch.
It cannot exceed the admitted pull bound. The effect source
revision identifies the exact committed service selection. The
target revision binds its current native origin. The service
checks the saved batch and flow commit before dispatch. The native
receiver verifies current origin and membership of the exact
sequences and refuses an unproved or changed selection. No range
between two selected sequences is implicit.

The `outcome=acknowledged` reply contains `origin_epoch`,
`pull_reply_digest`, `service_commit_ref`, `service_commit_digest`,
`acknowledged_sequences` and `native_receipt_ref`. Commit the exact
acknowledgement and native receipt together before pruning eligible
acknowledged entries.

Each retry uses a fresh 0118 attempt and the original logical
effect ID, typed intent and expiry. After checking current site,
authority and expiry, match the stored receipt by site, logical
ID and canonical intent digest before checking live spool
membership. A current exact committed receipt returns its saved
outcome in the new attempt's sealed reply, including after the
covered records are pruned. Do not apply the effect again.
A changed intent under the same ID or an expired or unauthorised
retry refuses. For an unsettled attempt with no exact current
receipt, require the owning proof of an unperformed effect and
the complete committed selection before another effect. Missing
spool entries alone prove neither commitment nor safe retry.
Do not acknowledge a partial subset or replace the logical ID.

#### `ai_traffic_view_put`

The effect payload contains `view_generation`, `view_receipt`
and `view_receipt_digest`. The receipt is the exact authenticated
0135 Site view receipt for the admitted reporting profile. It binds
the site and native role audience, content scope, UTC window, approved
cells, coverage, methodology, freshness and current authority epochs.
The effect source revision identifies that exact receipt. The target
revision identifies the current native view generation in its
authorised scope, including the absent state at first delivery.

Verify current site, recipient, generation, visibility, rights and
expiry before storing. A newer receipt replaces only the current view
at the same authorised scope. The same generation with a different
digest, a different scope or an older generation is refused. Check the
existing view bound and current native rights independently on display.

The `outcome=stored` reply contains `view_generation`,
`view_receipt_digest` and `native_receipt_ref`. Store the accepted view and
receipt atomically. Returning the original receipt on exact replay
cannot renew the stored view's expiry or change its audience. Current
display authority stays separate from the outcome of its delivery.

##### Native site-view receipt

The closed `view_receipt` holds `receipt_id`, `site_id`, `actor_scope_ref`,
`audience_ref`, `view_profile_ref`, `scope_ref`, `view_generation`,
`window_days`, `start_utc_date`, `end_utc_date`, `issued_utc`,
`expires_utc`, `grant_epoch`, `visibility_epoch`, `erasure_epoch`,
`disclosure_epoch`, `cells` and `fetch_evidence`. Reuse the
admitted identity codecs and
0118 signing and sealing. The outer generation and receipt digest
must bind these exact receipt bytes. Grant, visibility and erasure
bindings name the current authority for this audience and content scope.

Use half-open UTC-date windows of exactly 7, 28 or 90 days. Dates have
the observation's YYYY-MM-DD form. Verify the original lease of at most
one hour and any earlier bound on its disclosures. The reporting window
gives no retention. The replacement key is the exact admitted site,
actor scope, native role audience, profile, site-total or eligible
content scope, and window choice. The rolling dates belong to the
receipt. An equal or newer generation cannot select another
replacement key. The stored view has a monotone generation floor
for that key.
Resolve each scope against the authorised native view. No arbitrary
key or age selector gives access. At store, display and restore,
check the site, admitted scope and latest locally verified
disclosure fence for that replacement key. Fence a service
withdrawal from the next paired exchange before the next render.
Keep the disclosure floor across restore. Offline, a withdrawal
the add-on has yet to receive cannot extend the original lease.

Each entry in `cells` has `cell_key`, `measure` and `state`. Use the
admitted profile's unique cell keys in canonical key order.
Combined `cells` and `fetch_evidence` entries are at most 512
and must also fit the outer wire budget. An oversized proposed
view cannot replace an eligible current one. A smaller view needs
its own admitted scope and disclosure. Measures
are `verified_ai_fetches` and `human_ai_referrals`. Fetch coverage
belongs to the admitted 0135 origin and verifier scope. Referral
coverage belongs to the exact
[0034 disclosure profile](0034-analytics.md) and
[0093 host and landing mapping][host-mapping].
The latter requires QR-065. Neither measure gives evidence for the
other. Apply the same disclosure history to all cells and complements.

[host-mapping]: 0093-ai-search-visibility.md

| State | Fields in addition to the common fields |
|---|---|
| `measured` | `count`, `proof`. Use a non-negative count with complete eligible coverage for that cell. Zero needs affirmative complete coverage. |
| `partial` | `covered_start_utc_date`, `covered_end_utc_date`, `count`, `proof`. The covered interval is within the receipt window. Use null for `count` if only coverage can be disclosed. Never release partial zero as a measured zero. |
| `suppressed` or `not_measured` | `reason` from the profile's safe reason set. Omit the entire cell if disclosure of its key or existence is forbidden. |

The closed `proof` holds `source_profile_ref`, `source_generation`,
`coverage_ref`, `release_ref`, `release_generation`,
`methodology_ref` and `last_covered_utc_date`. These are safe
references and dates for that authorised cell only. The exact
release binds the source, purpose and view profile, cell key,
measure, population, scope, audience, basis, interval, coverage
and disclosure lifetime. Apply current release eligibility for
each cell and related evidence. It grants no raw-evidence access.

`fetch_evidence` is an array in canonical order of at most one entry
per separately admitted visible content key and exact revision. An
entry holds `content_key`, `content_revision`, `proof_ref`,
`release_ref`, `release_generation` and `rank_expires_utc`.
Bind the positive fact and badge disclosure to that exact
release, scope and revision. Use an empty array without a permitted positive
fact or where disclosure is withheld. Preserve the first eligible
fact's fixed deadline when retained evidence is allowed. A suppressed
cell cannot leak count, provenance, badge, expiry or complement through
the receipt or its display. Exclude raw observations, URLs or referral
paths, addresses, agents' request strings, member or visitor IDs,
credentials and applied ranking weights.

In E2E-0135-NATIVE, compare complete measured zero, a permitted partial
range and a withheld cell. Inspect the protected receipt and both
native views for the allowed fields. Replay the original receipt
after a newer generation has committed, after role loss and offline
after expiry. No replay can replace the new current generation,
change scope or renew expiry. Withdraw just one disclosure while
role, visibility and erasure are unchanged. Apply the new fence
without withholding an independently eligible different scope.
Try exact delivery replay and old restore. Replay can return a
saved delivery receipt only under the current exchange grant.
It supplies no renewed right to any cell, badge or complement.
Keep IT-0118-LINK and each adapter's
profile admission as the owning protection checks.

#### `ai_verifier_put`

The effect payload has exactly one `verb`, either `install` or `revoke`.
Both forms contain `verifier_kind`, `schema_version`,
`control_revision`, `target_generation`, `manifest_digest`,
`authority_epoch` and `key_epoch`. The source revision binds the exact
admitted control plan. The target revision binds the expected native
control revision. The initiating service uses the current owner-enabled
control approval.

The install form also contains `manifest`, `proof_set_ref`,
`effective_utc`, `expires_utc` and `verified_refresh_utc`.
The manifest must be the complete admitted typed verifier manifest
whose digest, proof set and validity interval match the intent.
The revoke form targets the exact bound identity with the shared
fields. It has no install-only fields or allow-list copy.

The `outcome=installed` or `outcome=revoked` reply echoes `verb`,
`control_revision`, `target_generation`, `manifest_digest` and
the exact typed `native_verifier_receipt`. Persist the control
revision, minimum allowed generation,
active pointer or revoke tombstone, and native receipt atomically.
Exact replay of the current control returns the saved receipt
without renewing freshness. Refuse any changed signed intent at
the same control revision and any earlier control record. A revoke
raises the minimum allowed generation above its target. A newer
control cannot install below that minimum. Preserve both minimums
outside restorable verifier data. Hold restored data that cannot
prove the current authority and both fences.

##### Verifier manifest and native receipt

`manifest` is the closed install record containing `site_id`,
`verifier_kind`, `schema_version`, `target_generation`, `proof_set_ref`,
`effective_utc`, `expires_utc`, `verified_refresh_utc`, `authority_epoch`,
`key_epoch` and `entries`. Match repeated fields to the exact sealed intent
and authenticated site. `manifest_digest` hashes the exact manifest bytes
encoded by the admitted canonical codec. The sealed effect authorises
only the exact verified native control plan.

Each entry holds `verified_agent_id`, `agent_token`, `operator_ref`,
`role`, `proof_ref` and `prefixes`. Use the admitted observed-token
matcher and role type. The current proof set pins the exact operator
source, revision and digest, covered prefix ranges, observed token,
agent, role, scope, validity, and legal collection and use basis.
Verify each entry against that registered proof and current authority.
A policy token, operator name or offline checklist supplies no proof.
Keep only safe evidence references in the manifest. It has no raw
request, visitor or credential fields.

A prefix is a closed three-item array of family, network bytes and
prefix length. Use `ipv4` with eight or `ipv6` with 32 lower-case hex
digits for the network bytes. The length is a JSON integer from zero
through 32 or 128 respectively. Host bits must be zero. Encode mapped
IPv4 networks as IPv4. Sort prefixes by family, unsigned network
bytes and numeric length. Sort entries by canonical byte order of
`verified_agent_id`, `agent_token`, `operator_ref`, `role`, `proof_ref`.
Require strict increase of entry keys and prefix tuple keys in each
array. Refuse duplicate keys and tuples. Sum submitted tuple counts
before coalescing. Apply the 16,384-prefix ceiling and the independent
0021 one-MiB body cap. The body count includes the complete protected
JSON, encoded 0118 associated data, 32-byte encapsulated key and
16-byte authentication tag. Equality passes either bound. Refuse the
whole install on excess. No split, cut or partial mutation.

Check intersections before compiling the active lookup. Compare
the resolved observed-token identity, including admitted aliases.
Refuse an overlap for one token and peer with a different agent,
role, operator or proof, even if some of those fields match.
Coalesce overlapping ranges only for the same token, agent, role,
operator and proof. Use their exact union in the compiled lookup.
Keep the signed source digest and exact classification. Match the
recognised token before the in-memory prefix check. Use no
proof lookup or network access on request.

At commit recheck the current control revision, site and owner
approval, proof scope, authority and key epochs. Install requires
`effective_utc` at or before trusted now, trusted now before
`expires_utc`, and a verified refresh no later than trusted now and
less than 48 hours old. Each proof must still allow the scope.
The manifest must expire by the earliest expiry of any entry
proof or use right. Refuse an overlong manifest in full. At
that boundary, the local worker stops positive classification.
Revoke uses the exact known manifest identity and current control
authority.
It needs no still-live proof set or copy of the manifest. Raise the
generation floor above the target without lowering an existing floor.
Clear an active pointer made ineligible by that raise. Keep an
independent eligible newer pointer. Refuse an unrepresentable next
floor and any install that would make such revocation impossible.
Refuse one verifier generation with a different manifest digest.

The closed `native_verifier_receipt` always contains `site_id`, `verb`,
`verifier_kind`, `schema_version`, `control_revision`, `target_generation`,
`manifest_digest`, `authority_epoch`, `key_epoch`,
`minimum_allowed_generation`, `outcome` and `native_commit_ref`.
Match its outcome and identity to the outer reply and bound intent.
A committed install adds `proof_set_ref`, `effective_utc`, `expires_utc`
and `verified_refresh_utc`. A committed revoke adds `tombstone_ref` only.
The assertion of a committed update comes from the protected receipt.
A refusal has no receipt. Commit the manifest pointer or tombstone,
floors and saved receipt together. A valid retry returns that receipt
without reinstalling or extending either timestamp. Binding the receipt
does not authorise a reader to use revoked evidence.

On receipt consumption, the service compares the current protected
site, authority, native control floor, generation and manifest
identity. An authenticated delayed receipt may settle its exact
historical attempt. It cannot roll back the current pointer,
proof, refresh time, expiry or tombstone. Preserve that order
and floors across restore. When current state cannot be proved,
hold dependent effects and positive evidence.

In IT-0135-FETCH, test reversed and duplicate prefixes, mapped IPv4,
host bits, ambiguous overlap, wrong token, wrong role, expired rights
and wrong site with synthetic proof. Try distinct nested prefixes
for one classification, an exact duplicate, and a resolved token
alias with an overlapping different proof. Use 16,384 and 16,385
distinct synthetic prefixes in frames both under the byte bound.
Accept the first if otherwise admitted and refuse the second.
With a legal prefix count, test a valid complete frame at 1 MiB
and one byte over. Exercise both preflight and receiver checks.
Refuse a manifest lasting beyond an entry's use right. Keep the
prior eligible verifier on any refusal. In IT-0135-SPOOL and E2E-0135-NATIVE,
test exact freshness expiry in a running worker, lost install and
revoke replies, altered same-revision intent, old install after
revocation and restore with missing floors. Deliver an old accepted
install receipt after the service has settled a newer revoke, then
an old revoke receipt after a distinct newer install. Exercise
receipt consumption after authentication. Preserve current floors
and release state. Reuse the warm worker and test expiry equality
under the newer admitted manifest and an earlier proof deadline.
No replay renews freshness. Check through IT-0118-LINK on both
adapters.

Use IT-0135-FETCH, IT-0135-SPOOL and E2E-0135-NATIVE with IT-0118-LINK
for the exact protected adapter profiles and their existing fail-closed
cases. The closed schemas and outcomes need their own admitted
WordPress and XenForo evidence before dispatch.

The native request reads only the local admitted verifier. Positive
classification ends at the earlier of manifest expiry and 48 hours
after its last valid signed refresh, or on known revocation or
disablement. The service fences revoked results immediately. The
add-on learns a service revocation on its next admitted exchange or
stops classification at the bound expiry. Unproved delivery or a
missing admitted provider, agent, role or paired-operation profile
gives unknown. A report delivery, fixture or matching name cannot
admit a verifier.

Limit one append per eligible request, at most 512 bytes, 4 MiB of queued data
per site and seven days of unacknowledged retention (all proposed). No public-
path SQL or network operations, response bytes or extra beacon. Cap added time
on the request path (verifier lookup and the after-response hand-off) at 1 ms
p95 and 5 ms p99 on each native adapter (proposed, measured per supported
host). Stop collection on exhausted or unsafe storage. Report dropped
coverage, absent edge-cache observations and stale or unverifiable verifier
data. A failed seal or missing status marker fails closed. The summary cannot
present an incomplete interval as complete or turn a missing interval into
zero.

Count each admitted fetch sequence once in its site and origin
collection generation. Produce report aggregates in Rust from committed
observations and coverage under the admitted privacy profile. Exact
content counts require current visibility and a privacy-approved release
for the selected scope, role and window. Apply the same admitted
suppression to table, chart and any combined view. A completed UTC day
with affirmative coverage for the selected projection and no qualifying
observations can show zero. A gap, expired data, uncovered edge-cache
traffic, missing manifest admission or an unproved report projection
makes the affected coverage unknown. Show an admitted partial result
with its covered range where permitted. Full-window results require
complete, compatible coverage through the whole window.

The 7-, 28- and 90-day choices are reporting windows. Hold aggregate
cells, coverage and privacy-release records only for their admitted
retention period. A 90-day view may therefore be partial or show
“Not measured”. Selecting it does not extend raw or aggregate
storage. The ranking fact's 28-day expiry and the native buffer's
seven-day limit remain separate. Human referrals still use 0034 and
0093. Site-level or landing results need the approved QR-065 host
mapping and visit floor (the floor of ten is proposed until QR-065 is
answered). A landing result also needs the admitted
joint host-and-content projection. Do not derive it from separate
totals. No report aggregate is a ranking input.

Source revocation, changed visibility and erasure invalidate
raw observations, aggregates, native summaries and rank facts
at the owning gates. Recheck visibility and expiry on view and
rank. Recheck native permission, epoch and visibility on every render.
Service-only withdrawal stops renewal and marks old summaries
held on the next paired exchange. A native view lease lasts
at most one hour. An expired offline receipt is unavailable.
Report the last approved coverage date.
Remove acknowledged buffer entries and expired managed copies.
Use 0032 and 0118 ledger, restore and key deletion.

## The ordering signal

Use the new 0027 fetch evidence door after eligibility and safety.
`rank.ai_verified_fetch` is one 0055 fact entry. The protected Rust fetch
evidence reader accepts site, destination, exact revision and one pinned as-of
generation per ranking batch. It returns only the positive fact or absence. No
graph join on events, counts, referrals or native payloads. Bind only the
positive fact for the exact current public revision. Require a
search or automated crawler role expressly admitted for ranking.
A training role needs that same proof. Unknown and user-directed
roles supply no fact. Use the first admitted verified observation’s
recorded UTC date and original proof. Expiry is UTC midnight at
the start of that date plus 2,419,200 seconds. At equality or later
use the base order. A subsequent receipt, replay or restore cannot
move the deadline. Conflicting, erased or missing first-observation
lineage keeps the same revision at base order. A new revision needs
its own exact proof. Keep the existing retention and erasure rules.

Use DR-153’s batch ceiling. Count N distinct canonical same-site
destinations eligible for the 0027 order stage after hard eligibility,
relevance and applicable safety checks, before this adjustment and
final selection. Count those with or without a positive fact.
Duplicate anchors, repeated receipts and split delivery leave N
unchanged. The maximum recipient count is floor(N / 10). Below
ten it is zero. In unboosted order, including the existing stable
tie-break, take at most that many with a positive base key, a
current exact fact and no active trust caution. The rule adds
no rotation, new seed or another allocation input.

Pin the eligible population, selected identities, settings snapshot
and base key profile for the attempt. Before release recheck current
eligibility, authority and expiry. An invalidated fact loses its
adjustment and leaves the slot unused. A changed or unproved
population or scalar profile withholds every adjustment in the
attempt. Preserve the ordinary current safety and final selection.

Future, unproved, stale or removed evidence has no effect. Source feeds from
the Coach, DataForSEO, the Update Lab or editorial surveys cannot provide this
fact.

Use the 0014 declaration for `ranking.ai_fetch_boost_bps` and the
0055 input entry for `rank.ai_verified_fetch`. Use the 0021 and
0118 paired registry for `ai_observation_pull`, `ai_observation_ack`,
`ai_traffic_view_put` and `ai_verifier_put`. QR-236 still requires
exact operation profiles, receipts, native role and footprint
proof and 0077 admission before serving the adjustment.

Use `ranking.ai_fetch_boost_bps`, integer 0–500 basis points.
One basis point is 0.01%. The default is 0 (DR-160): no boost
applies until an evaluation against the ADR-0021 floors supports a value
and the owner sets one. An owner may change it or switch off.
Spec 0014 records the old and new values and requires the
existing 0020 confirmation. Native and MCP views can explain
the setting but cannot silently activate or edit it.

A weight that is not a whole number from 0 to 500 keeps the base order
for the batch, and the numerators use 128-bit checked integers.

For an admitted non-negative scalar order key B set the
comparison key to B × (10,000 + weight) / 10,000 once for each
selected eligible target under DR-153. An unselected target,
missing fact, disabled feature, active trust caution or unadmitted
scalar profile leaves B unchanged. Compare widened checked integer
numerators using the same
denominator 10,000 and existing base-key units. Unboosted keys
use B × 10,000. A tie keeps the existing tie-break. On overflow
or unadmitted precision, keep the base order for the batch.
Recheck current fact and gate before release. On expiry, fall
back to base order. Preserve normal link-equity, diversity and
moderator safeguards after adjustment.
Fetch counts, agents, duplicates, repeated requests and
human referrals cannot stack the boost. The setting cannot
change relevance, ranking thresholds, moderator approval or
the order for another site. Admit through 0055 and 0077 with
the 0027 quality and safety proofs before serving any boost.

## Native interface and owner visibility

In WordPress admin or XenForo admin open Sutradhar, then “AI
traffic”. Require the native site's view capability, active
site ID and a current authorised report. Show 7-, 28- or
90-day UTC views. The first panel says “Verified AI fetches”
and can explain crawler roles. The second says “Human visits
from AI services”. Show date and coverage, a list and optional
chart with the same values, and exact reasons for unknowns.
The ranking panel labels the configured boost as a setting.
Show recent positive evidence and its expiry only where the
sealed site view receipt explicitly approves those fields for the
current native role and exact visible content scope. If that
disclosure is withheld, omit the proof and expiry. The panel
cannot reveal a suppressed observation through an evidence badge,
an applied weight or a per-target ranking trace. When the receipt
permits the retained proof and its fixed deadline, show
“The fetch window for this version has ended” at or after expiry.
A later eligible fetch may still count in the report. It leaves the
original deadline in force. A newer revision needs its own first
qualifying fetch. If the proof is missing or withheld, show only
a permitted unavailable reason. A surviving proof describes
eligibility. An applied weight needs its separate authorised trace.
Report coverage
and privacy release do not decide whether the protected rank reader
has a qualifying fact. Reuse native PHP templates, local SDS
assets and the sealed view. Add no hand-written browser
script, network lookup or public control. Table and chart use
one authorised projection. Send private no-store on admin
responses. Editing opens the owner app.

Use GOV.UK form, table and error patterns and BBC GEL's consistent information
grouping, realised in original SDS components within native admin frames. Take
every colour from the SDS tokens (spec 0017); this spec fixes no colour
values. The Kraken reference below informs mood only. Use the tokens' on-
primary and on-surface pairs for actions and text. Use a clear double focus
outline on light and dark surfaces. Status always has text. Follow native
scale and high contrast, and prove WCAG 2.2 AA with keyboard, reflow and
assistive technology. Do not copy product layouts, typefaces, marks or assets.
See the [GOV.UK design system][gov], [BBC approach][bbc] and [Kraken design
reference][kraken].

The owner app's Basic, Advanced and Engine Room surfaces
enumerate registered capabilities, effective settings and
configuration, supported inputs, outputs, ranking methods
and operation reasons. Reuse the 0014 registry, 0020 scoped
discovery and 0027 ranking explanation. The signed release inventory maps
every shipped action,
setting and pipeline stage to an owner-visible descriptor,
provenance, scope, allowed values and trace type. Native
admin grants admit only the present site's view projection.
Desktop owner inspection still checks Cedar for each value. Display exact
authorised safe values and
per-candidate prior key, applied weight, adjusted key, gate
decision and evidence where the owner may read them. Omit
forbidden evidence IDs, with a scope-safe reason. Secrets,
forbidden audit records and private member data have safe
presence or access indicators only. No unsafe raw process,
credential or private request is copied into native GUI,
MCP, receipts, exports or other sites. Refuse a release
inventory with a shipped capability missing its descriptor.

For an authorised owner view, read the current effective setting and
its source from the 0014 registry. For an exact ranking result, use
the settings version recorded by 0027 and the weight applied to that
result. Keep the configured weight, the permitted current fetch
evidence and the recorded adjustment separate. When the authorised
trace proves that the fetch adjustment was zero for this result,
show “This result kept its base score.” Use the exact authorised
result trace to show any applied weight. If the used version or
trace is unavailable to the viewer, show only the permitted reason.
A displayed fetch fact gives no proof of adjustment for an earlier
result. This status adds no batch size, slot or selection fields and
grants no further evidence access.

## Data

Use the records and retention rules in Records, exchange and lifecycle.
Keep observations, aggregates and ranking facts separate.

The 0135 native buffer retains at most seven days of unacknowledged
observations. The separate Rust ranking fact expires 28 days from
its bound first admitted verified observation of the exact revision.
For a fetch aggregate, bind the exact
admitted site release profile, data class
and expiry for its cell, coverage and disclosure record. Hold that
aggregate route until the profile names the retention and 0032/0118
erasure rules. The 7-, 28- and 90-day view options give no retention
grant. Human referral aggregates remain under 0034 ownership and
retention, with their existing privacy-release and host-mapping gates.

## Settings

Bind site collection to the current owner-enabled 0135 collection
intent, signed native capability and the exact owning 0034 recording,
privacy and opt-out policy references. Keep collection off if the
required policy is missing, stale or refused. Collection enablement
grants no approval to activate the independent 0014/0020 ranking
setting or release a report.
`ranking.ai_fetch_boost_bps` uses the 0014 registry and 0020 confirmation.

## Failure codes

Use separate typed outcomes. Missing or stale fetch proof is unknown.
An authorised report distinguishes measured, partial, suppressed and
not measured. Return a measured zero only with affirmative complete
coverage for that projection. Partial results identify the covered
range. Suppressed results expose no hidden cell or complement. The
ranking reader returns only a current eligible positive fact or
absence. Absence keeps the base order. No new public SUT code.

## Security and privacy

Require current guest visibility, site and actor scope and privacy release.
The public hook keeps no raw request material. Use 0118 for paired records.

## Requirements

| Requirement | Acceptance design |
|---|---|
| REQ-0135-FETCH | IT-0135-SPOOL and IT-0135-FETCH use synthetic verified and forged agents, exact peer and proxy rules, no browser referrer dependence, unsupported bot proof, private/ambiguous pages, direct post and thread, repeated and stale source, and over-cap storage. Exercise no verifier, wrong site, stale restore, expiry/revocation, same-control conflicts, older install after revoke, replay without freshness renewal and known-zero versus missing or suppressed coverage |
| REQ-0135-REFERRAL | IT-0135-REFERRAL uses confirmed human, automation, missing host, opt-out, small/overlapping groups and erased metadata. No path readback or referral ranking input |
| REQ-0135-RANK | E2E-0135-RANK tests the default 0, an owner-set 5% and over-cap, evidence freshness and revoke, no stacked boost, failed relevance and caution, layer isolation from the Coach, mixed thread/post revisions, tie crossing, zero, overflow, final-epoch recheck, no candidate/gate/model changes and exact equity |
| REQ-0135-NATIVE | E2E-0135-NATIVE uses supported WordPress and XenForo versions and all v1 Bazel hosts. Test signed batch replay, sequence gaps, rollback, role loss, offline expiry and service-side immediate withdrawal, cached-origin gaps, erasure and public footprint. Prove keyboard, focus, screen-reader labels and parity. Exercise the partial 90-day report and independent 28-day rank fact expiry |
| REQ-0135-INSPECT | IT-0135-INSPECT plants a missing action, setting, input, output and result descriptor. The release check refuses each. A wrong site, forbidden data class or view cannot disclose protected details or change settings |

## Limits and footprint

Apply the public-path budget under Records, exchange and lifecycle.
Every figure in this spec (512 rows and bytes, 4 MiB, seven days, 48
hours, 16,384 prefixes, 2,048 bytes, ten visits, one hour, 28 days, 1 ms
and 5 ms) is proposed until measured on each supported host.
Bind each adapter's measurement to its exact supported host and
storage profile. Missed edge-cache traffic stays outside coverage.
When a request cannot be recorded within the bound, serve the page
normally and report incomplete coverage on the admitted view.

## How it is tested

IT-0135-FETCH and IT-0135-REFERRAL distinguish the two evidence sources.
IT-0135-SPOOL tests a race at the rotation size, a rotation under load, a
second rotation while the rotated file is unread, a wrong owner or mode, a
symbolic link, wrong permissions, forged, oversized and non-UTF-8 lines,
concurrent appends at the cap, tampering, replayed line IDs, rotation and a
long-lived worker that passes verifier expiry or revocation. With an empty
origin, pull from a null cursor. Return an empty end batch with a non-null
same-origin cursor at the append boundary and no coverage or acknowledgement
from end alone. Append a record and pull again from that exact cursor. Return
the new row once. For a bounded page with a rejected row followed by a valid
row, advance over the rejected row within the scan budget and keep coverage
incomplete. Compare an end batch with a body limit leaving a full record
unread. An authenticated reply with one wrong-site row or wrong- origin cursor
changes no committed row, coverage or cursor. Acknowledge the second and
fourth of four admitted rows from one committed batch. Prune only those
eligible rows and lose the reply. Retry in a fresh sealed attempt with the
same logical ID and intent before expiry. The saved native receipt names that
same subset and the unacknowledged rows remain. Repeat with a changed subset,
after authority loss and after removing the original receipt proof. These
cases give no further acknowledgement or pruning. E2E-0135-RANK also farms the
boost with repeated user-directed fetches and checks the 10% cap. In the
expiry case use ten eligible items and one current fact for an exact public
revision. Set the admitted weight to 500 basis points in the synthetic
fixture. Re-fetch that revision on day 27. At the last admitted clock tick
before the original deadline, use the boost once. At the deadline and after
it, use the base order. Include a new fetch after expiry for the same revision
and a replay of the old proof. Neither changes the first observation or its
deadline. Restore the same apparent current revision without its properly
bound first proof after the original expiry. Give a later verified fetch and
independently eligible report scope. Keep base order and any independently
eligible report under current coverage. A replay does not mint a new ranking
window. Repeat after erasure with current authority and no retained first
proof. Refuse the adjustment. Then edit and re-fetch the new revision. Only
its own eligible exact proof can start its window. An offline bot policy
result under 0145, alone or alongside a later record with unproved origin,
supplies no fetch fact. For the DR-153 guard use separately frozen batches of
9, 10, 19 and 20 exact destinations. Their slot caps are 0, 1, 1 and 2. In the
ten-target fixture give A and B valid facts and base key 200. The existing
tie-break orders A before B. Give C no fact and key 206. At 500 basis points A
compares as 210, before C and B. Change arrival order and verify the same
result. A higher-key foreign, duplicate or failed-relevance fact cannot
enlarge the pool. In separate runs put A under caution before selection,
revoke A’s selected fact before release, or change the eligible pool from ten
to nine. The first selects B, the second leaves the slot unused, and the third
releases no boost. For an admitted observation at 2026-01-08 23:59:59 UTC,
record 2026-01-08 and expire at 2026-02-05 00:00 UTC. Join two eligible same-
date occurrences for one revision, preserving one fixed deadline. Give a
second signed occurrence for the same origin and revision just after that UTC
day boundary. Deliver the newer sequence first. An unproved gap holds the
ranking fact. After reconciliation bind the first exact occurrence and its
original expiry. Verify one fact in both delivery orders. A knowingly
contradictory older result fences the existing fact, with no later deadline.
An unproved old profile gives no current boost.

IT-0135-FETCH also tests a manifest of 16,384 and of 16,385 prefixes, the
verifier lookup time, and a slow, failing and refused spool write (the page is
still served and a gap is counted). IT-0135-REFERRAL also sends hostile
referrers: 2,049 bytes, control and bidi characters, user information, non-
HTTPS, look-alike international host names, and path, query or fragment text.
Each of the first five yields "unknown" with nothing durable. Path, query or
fragment text on a listed host keeps the host and stores nothing else.
E2E-0135-RANK tests the ordering fence. E2E-0135-NATIVE and IT-0135-INSPECT
test authorised views and their descriptors. In IT-0135-INSPECT, change the
effective boost after a synthetic ranking result. Check that the authorised
owner view shows the current setting and source, and the applied weight and
used version for the older result. Remove permission to read that trace and
check that its weight and version are withheld. Native and MCP views keep
their existing scope and never gain that trace by reading the setting
descriptor. For the proposed bounded selection profile, join
IT-0135-INSPECT to its E2E-0135-RANK case with nine eligible targets
and one current fact. With the weight enabled and an authorised
exact result trace, show the configured weight, permitted evidence
and base-score result separately. At ten eligible targets, show the
selected target’s actual adjustment only from its own trace. Remove
trace access and show only the permitted unavailable reason for that
result. Neither status adds a batch size, slot count or hidden evidence.
This candidate fixture gives no authority to use the profile.

Extend IT-0135-FETCH, IT-0135-REFERRAL and E2E-0135-RANK with missing
recording policy, unadmitted retention, expiry under the 90-day view,
suppression, a verified zero and a missing fact. Check the distinct
outcomes, withdrawal before cleanup and the unchanged base order.
In E2E-0135-NATIVE and E2E-0135-RANK, pair a current eligible rank
fact for an exact public revision with a native report whose
content-level evidence is suppressed. Check that the approved
ordering can use the fact, while the native panel gives no evidence
presence, expiry or applied-weight trace for that content. The
configured setting can still appear if its read is authorised.

In E2E-0135-NATIVE, use the original exact public revision and
the fixed deadline from the E2E-0135-RANK expiry case. Admit a
repeat fetch after that deadline with complete authorised report
coverage. The visible count increases once. An authorised panel
with the retained proof shows the ended window and its original
deadline, with no applied-weight claim. Open a newer revision.
The old proof supplies no evidence for it. Give it its own first
admitted qualifying fetch in a separate step and check its own
deadline. Withhold proof when it is missing or when the current
role loses the exact disclosure. Keep any independently released
report count and setting visible. The content panel gives only the
permitted unavailable reason, with no evidence badge, expiry or
applied-weight trace.

## Changes from the source

DR-122 adds the native views and one bounded ordering fact. Operator
lists prove only their exact agent role under the admission rules above.

[openai]: https://developers.openai.com/api/docs/bots
[perplexity]: https://docs.perplexity.ai/docs/resources/perplexity-crawlers
[claude]: https://support.claude.com/en/articles/8896518-does-anthropic-crawl-data-from-the-web-and-how-can-site-owners-block-the-crawler
[google]: https://developers.google.com/crawling/docs/crawlers-fetchers/google-common-crawlers
[gov]: https://design-system.service.gov.uk/
[bbc]: https://downloads.bbc.co.uk/diversity/pdf/BBC_Accessibility.pdf
[kraken]: https://www.kraken.com/
