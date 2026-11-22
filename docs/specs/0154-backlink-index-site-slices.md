# 0154: Backlink index site slices and local lookup

Figures: every numeric limit, threshold and budget in this spec is proposed
until measured on the Dell or settled by a QR.

## Site slice publication

[0153](0153-public-web-backlink-index.md) owns collection and can
freeze a bounded projection from one generation. This spec owns
the signed file and local readout. [0046](0046-web-link-study.md)
reads the eligible slice as a distinct evidence source. It accepts
no raw corpus, bulk archive or live network query. QR-239 requires
admission of the exact recipient, file codec and resource profile.

The owner enrols a verified destination property. A current enrolment receipt
binds site, scope and epoch, recipient, publisher, exact index instance
and source admission. A different instance needs separate enrolment
and an independently verified starting withdrawal floor.
Scope is an allowlist of exact HTTPS origins and optional path roots matched
on segment boundaries. Verify each alias and subdomain separately.

Use the existing 0065 signature framing in the admitted offline codec.
`T` is length-prefixed UTF-8, `U` is an unsigned 64-bit big-endian
integer and `B` is length-prefixed bytes under 0064. The caller bytes
are `T(record_type) || U(1) || B(canonical_typed_json)`. Tijori prepends
`T(purpose)` once. Use [RFC 8785][jcs] canonical UTF-8 for the JSON body.
Verify the exact purpose, record type, format version and body schema.

| Purpose | Record type | Admitted signing role |
|---|---|---|
| `0154-site-slice` | `site-slice-manifest` | Slice publisher |
| `0154-site-withdrawal` | `site-slice-withdrawal` | Custody withdrawal signer |
| `0153-coverage-witness` | `coverage-witness` | Independent coverage verifier |

Each role needs its own purpose-scoped signing key in the protected
profile under QR-238 and QR-239. Accept only the current admitted
key and epoch for each required role at import, activation and read.
Use verified local trust state outside the package and any restored
copy. An old signature's issue date cannot prove current authority.
Recipient sealing under 0118 uses its own key and grants no signing
role. An unadmitted signer, framing or purpose holds the record.
Rotation and revocation never lower verified withdrawal floors.

The canonical signed manifest uses purpose `0154-site-slice` and contains
`schema`, `site_id`, `scope_ref`, `scope_epoch`, `publisher_id`,
`index_instance_id`, `signer_id`, `key_epoch`, `recipient_id`, `snapshot_id`,
`generation`, `census_digest`, `profile_digest`,
`canonicalisation_digest`, `observed_interval`, `issued_at`,
`expires_at`, `erasure_watermark`, `coverage`,
`row_counts`, `payload_length`, `payload_digest`. Use a half-open UTC window.
Apply the signed framing above to the typed manifest. Seal the package
for the exact purpose and recipient under 0118. Refuse duplicate or
unknown fields. Encode 64-bit unsigned
counts, UTC epoch seconds and revisions as decimal strings without
leading zeroes. Digests are 64 lower-case hex characters for SHA-256
or the explicitly typed HMAC-SHA-256 commitments below.
The payload digest covers the exact decoded canonical typed rows.
Seal typed rows under 0118 with site, scope, index instance,
recipient and snapshot as context.
The first qualification profile caps each sealed and decoded file at
32 MiB, total rows of all classes at 100,000, address keys at 2,048
UTF-8 bytes, result pages at 100 rows and import working memory at
64 MiB. Reserve 128 MiB of managed stage space before import or hold.
The signed source profile may lower a bound. Hold unknown schemas,
missing admission and limit breaches. Ledger staging and committed
files. Expiry is at most 604,800 seconds after signed issue time.
Check trusted current time and evidence on every read. Refuse a
future issue time. Earlier scope, licence and protection expiry wins.
Reload, replay and restore cannot extend it.

[jcs]: https://www.rfc-editor.org/rfc/rfc8785.html

Preview exact verified site, scope, source, observation dates, safe coverage
and size. Match independent publisher trust, recipient, rights and current
scope before the owner confirms this exact package. At activation,
under the withdrawal scope fence, recheck current trust, rights,
expiry, scope and watermark. Commit the staged ledger reference,
immutable generation binding, admission receipt and active pointer
in one transaction. A failed check leaves the pointer unchanged
and clears staging. Keep serving the prior slice only while it is
eligible. Import refresh requires a newly selected package and the
same approval. Retry reads the exact committed receipt.
Same snapshot ID and digest reconciles once with no freshness change.
Key the active pointer, generation bindings and withdrawal floors by
site, scope reference and epoch, publisher, index instance and
recipient. Use the stable receiving identity apart from the seal
key. Key rotation preserves the lineage and floors. Bind one
snapshot ID and digest to each generation.
Refuse an older generation or conflicting same-generation package.
A different instance cannot use the active pointer or floor.
Recheck authority and expiry before each read.

## Offline withdrawal and recovery

Custody signs a versioned withdrawal using the admitted framing and
`0154-site-withdrawal` purpose, with `schema`, `index_instance_id`,
`site_id`, `scope_ref`, `scope_epoch`, `publisher_id`, `signer_id`,
`key_epoch`, `recipient_id`, `withdrawal_id`, `issued_at`,
`withdrawn_through_generation` and `erasure_watermark`.
Match the full pointer key before applying either floor. Both floors can
only increase.
A different
payload under the same withdrawal ID refuses. Seal it to the same
recipient. An independently verified change to local authority or a
new verified withdrawal raises the durable fence before any further
read, export, cache serve or restore. A same or older watermark
cannot undo it. A newer clean slice may restore eligible rows only
with current grant, a generation above the withdrawn-through
floor and at least the locally held watermark. The publisher
proves the slice was rebuilt with all applicable withdrawals.
No fallback exposes an old affected slice. Reconcile an exact
replayed receipt without lowering the current floor.

The offline app cannot see a withdrawal it has not received. It
shows the original expiry and “Last verified on [date]”. Import the
withdrawal file or new generation to apply publisher changes. An
expired or time-unproved slice holds until replaced. There is no
network check. Record protected cleanup and unreachable copies under
0032. Restore remains held without the current local floor.

## Identity and coverage

- `EdgeObservation` contains `source_key`, `target_key`, `source_group`,
  `capture_id`, `observed_at`, `rel_set`, `proof_ref`.
- `PageObservation` contains `source_key`, `target_key`, `capture_id`,
  `observed_at`, `finding`, `proof_ref`.
- `ScopeCoverage` contains `target_scope_ref`, `start`, `end`, `profile_ref`,
  `state`, `eligible_sources`, `assessed_sources`, `reasons`, `proof_ref`.
- `SliceReceipt` binds the full pointer key, `generation`, `snapshot_id`,
  `package_digest`, `admitted_at`, `expires_at`, `coverage_ref`, `ledger_ref`.

Use the exact 0152 canonical URL profile and verified scope for `target_key`.
Drop fragments. Keep distinct paths and query identities unless a checked
alias proves equivalence. A foreign canonical claim cannot join or expand
scope. Withhold unsafe or ambiguous identities and record the coverage gap.
Deduplicate observations on `(source_key,target_key,capture_id)`. Within a
capture, repeated links yield one edge with the sorted union of allowed rel
marks. Conflicting evidence for the same directed pair and capture
refuses. Admitted seen and complete absent observations jointly
order by `(observed_at,capture_id)`. The latest eligible state is
authoritative within that snapshot and selected window. Retain the
last-seen and last complete absence dates. Group sources by the
pinned public suffix profile. An unknown group leaves the page
eligible and its domain count unproved.

`finding` is `seen` or `absent_in_complete_read`. The latter needs a
complete dated read of the exact source and complete eligible
projection for that target under 0153. It needs no destination fetch.
If it follows
a seen observation, keep the history and report “Not seen since”
and its separate complete-read date. A later eligible positive read can
show “Back again” under 0046. A block, failed or truncated read
or omitted file row cannot change that state. Record any proven
attempt and its coverage separately. Keep last checked, last read
and last seen distinct. Never borrow history from another snapshot.

`state` is `complete_for_slice`, `partial`, `unavailable` or `withheld`.
Complete requires that all eligible sources in the declared exact slice were
assessed with no unresolved gap or cap. A broader query cannot borrow narrow
coverage. Complete zero means no matching edge in that scope and window.
Show observed partial evidence with coverage. Use null for unsupported or
suppressed counts. Apply 0046 disclosure before any row or aggregate release.

### Signed coverage attestation

Encode the manifest's `coverage` as the canonical typed
`ScopeCoverage` with the 0153 proof projection. Bind `proof_ref`
to the SHA-256 digest of all exact bytes signed for the coverage
descriptor, including the purpose field. Carry the descriptor and
its signature inside the recipient-sealed package. Coverage belongs
in the manifest, outside the rows covered by `payload_digest`.
Include `source_scope_digest`,
`projection_digest`, `assessed_set_commitment` and `basis`. The
`census_digest` field in the manifest is the HMAC-SHA-256 frozen
census commitment. The assessed-set commitment uses its own
domain label under 0153. `basis` is `frozen_finite` or
`open_discovery`. The approved projection digest binds the
destination scope, view set, allowed fields and disclosure policy.
Both commitments bind the same generation, scope and window.
Never deliver the operator key, membership lists or hidden counts.

The descriptor uses purpose `0153-coverage-witness`. Its canonical
record contains schema, issuer and key epoch, index instance,
generation, generation manifest digest, approved witness digest,
source and projection digests, target scope and epoch, profile
digest, `recipient_id`, observation window, basis, both commitments,
disclosed counts, coverage state, gap codes, erasure watermark,
and the slice `payload_digest` and `row_counts`. Use the signed framing
above for the exact canonical descriptor. Include the descriptor and
signature in the 32 MiB package bounds. QR-238 and QR-239
require the exact independent verifier trust and purpose profile.
A key admitted only to publish slices cannot sign this descriptor.

At activation verify both signatures, issuer roles, both key epochs
and the descriptor digest offline. Require exact equality on fields
shared with the signed slice, enrolled scope and approved profile.
Match the recipient ID, decoded payload digest and counts by row
class before any projection is eligible. Refuse missing,
unauthorised or mismatched proof. Fetch no source
or proof on import.
For `complete_for_slice` require `frozen_finite`, equal disclosed
counts that are both present, no gaps, and a
verified complete-projection attestation from that exact generation.
Recheck the current disclosure and withdrawal floor at read. A
valid partial package cannot report complete zero. Refuse a signed
package whose fields claim incompatible coverage, such as complete
with a cap or a different proof scope. Suppressed evidence cannot
support complete zero. Neither a count nor an empty result can
substitute for the attestation. Resolve a stale prior package under
the existing trust, expiry and withdrawal rules.

## Local lookup and owner view

`lookup_site_slice` takes `site_id`, `snapshot_ref`, `target`,
`view`, `page_size` and `cursor`. The snapshot reference binds the
exact pointer key, snapshot ID and approved digest. Resolve it
against the active site scope on the first page and later pages.
Target is the enrolled `Property` or one canonical
`Destination` within its scope. Views are `ReferringPages` and
`TopReferringDomains`. The read requires the current site and disclosure
rights. Missing or mismatched scope returns the existing scoped refusal.

It returns `snapshot_ref`, `source`, `observed_interval`, `admitted_at`,
`expires_at`, `coverage`, `rows`, `edge_count`, `domain_count`, `next_cursor`.
Page rows return safe source and target keys, dated state and
permitted rel marks. Both current totals and top-domain rows count
only projected pairs with latest eligible state seen in the selected
snapshot and scope. A dated absence stays available as history under
current permission. It adds no page, edge or domain to those totals.
Domain rows return group, distinct visible current edges and pages. Build
totals before pagination from that projection.
Sort visible page rows by `(source_key,target_key)` and domain rows by edge
count descending and group key ascending, using canonical byte key order.
Authenticated cursors bind the full pointer key, snapshot digest,
target, view, page size, last sort key, authority and disclosure
epochs, and expiry. Null
starts or ends the series. A next page begins strictly after the last key.
After a scope, authority or active snapshot change, ask to reopen the view.
A withdrawn snapshot exposes neither its surviving rows nor totals.
Require a verified successor for the affected scope before reopening.
A separately eligible snapshot outside the withdrawal remains readable.

Show **Import snapshot** and **Import newer snapshot** under **Backlinks**.
Show source, dates and covered scope beside both views. Use **No
current links in this snapshot** for exact complete zero and
**Coverage is incomplete**
for partial results and **Not measured for this selection** for unavailable
results. Keep withheld cells hidden. **Save a copy** requires the current
owner export permission, licence, disclosure and exact local-file preview.
Recheck before writing. Keep credit, dates and scope on the saved report.

Keep this source separate, including its totals. It adds no score or rank
signal, staff or MCP grant, network request or follow-up monitoring job. A
0045 lead check still requires its own admission. Source addresses are text.

## Synthetic acceptance

- In IT-0154-SLICE-IMPORT, IT-0154-OFFLINE-WITHDRAW and
  IT-0154-CENSUS-ZERO, verify each correctly framed record with its
  purpose-scoped synthetic publisher, custody or verifier key. For
  each record, offer authentic test-signed bytes under a different
  authorised test purpose. First verify that signature within its
  own test scope, then refuse it in the required slot. Separately
  substitute a key from another role and try legacy dotted names
  and raw unframed JSON. No refusal changes the pointer, raises a
  withdrawal floor or supplies coverage or disclosure evidence.
  Repeat with a valid signature from a retired key. Revoke a required
  role between activation and read, then restore old trust data.
  Keep the withdrawal floors and hold the read without a network call.
- IT-0154-OFFLINE-WITHDRAW freezes a 7-day synthetic expiry, opens a
  view offline, imports newer withdrawal proof, then restores an old
  cache. The affected view holds before the next release. A valid
  file from the same publisher's other instance changes no floor.
  Without
  known withdrawal proof, nothing extends the original expiry.
  A clean slice at the current fence exposes only eligible rows.
  A fresh install with no slice shows “Not measured”. Refresh opens
  the local file chooser and performs no network work.

- `IT-0154-CENSUS-ZERO` imports equal empty projections from
  0153's complete, capped and open-discovery fixtures. Only the
  verified finite one shows the scoped zero label. Re-sign a
  mismatched generation, target projection or window with the
  test publisher. Refuse it despite a valid slice signature. Use the
  publisher key in the verifier slot and refuse that descriptor.
  Use a separate valid one-edge package. Re-sign it for another
  recipient, remove its edge row, or alter the row counts. In each
  case keep the original verifier proof and refuse import. A coverage
  field hidden by disclosure stays hidden and cannot justify zero.
  Race withdrawal against a page reply and withhold the old proof.
- `IT-0154-SLICE-IMPORT` repeats one receipt after a lost reply. Swap
  site, scope, index instance, recipient or signature, try stale and
  conflicting generations, and
  exercise equality and one over each cap. A failed or revoked import leaves
  the eligible pointer unchanged and clears staging. Race a withdrawal
  between confirmation and activation. In a separate case lose the
  reply after commit and recover the one receipt. A confirmed refresh
  switches once. Reuse a generation with a different ID and prove it cannot
  replace the active slice. In a synthetic UTC timeline issue
  slice A at 10:00 with signed expiry 10:10 on the same date.
  Commit it at 10:01 and lose the reply. Replay the exact
  package at 10:10:01. Hold A's rows, counts and export, keeping
  its original expiry and commit count. In a separate fixture
  activate independently approved successor B at 10:09 with
  issue time 10:08 and expiry 10:20. Bind B's new snapshot ID
  and generation to the same eligible payload bytes. The replay
  keeps B's pointer, dates and current eligible output. A's
  commit count stays one. The app performs no fetch. Expiry and
  erasure remove access through the ledger.
- `IT-0154-SLICE-IDENTITY` reorders duplicates and tests checked aliases,
  distinct queries, a foreign canonical and a sensitive address. Contrast
  complete dated absence with a blocked read and an omitted row.
  In successive eligible snapshots, see one pair at t1, confirm
  absence at t2, get a blocked read at t3, then see it again at t4.
  Each slice carries the needed eligible observations in its own
  window. For t3 include t1 and t2 with the t3 coverage gap.
  For t4 include t2 and t4. With a known eligible domain, expect
  both edge_count and domain_count to be 1, 0, null and 1.
  Show “Back again” at t4 with the absence and return dates.
  A blocked read directly after the first positive keeps its dated
  seen state and adds a coverage gap. Show the one observed pair
  with partial coverage. A clipped or broader slice cannot show
  complete zero.
- `IT-0154-SLICE-PAGE` pages both views with tied counts, then varies target,
  snapshot, site, page size or disclosure epoch. Each mismatch refuses the
  cursor. A revoked grant or licence refusal creates no local export. An
  unentitled staff or MCP caller sees no protected rows or counts.
- `E2E-0154-SLICE-READ` proves the three coverage labels, source credit and
  permitted export with synthetic sites. Opening, paging, refreshing from a
  selected file and exporting perform no outbound call or ranking effect.
