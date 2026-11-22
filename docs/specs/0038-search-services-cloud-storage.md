# 0038: Embedded search, cloud storage and provisioning

Tantivy keeps a local keyword index of a site's guest-visible posts.
Spec 0022 owns ingestion and the vector index. Spec 0023 owns BM25 and
the ranking contract. PostgreSQL remains the source of truth and the
local full-text facility of ADR-0021 keeps its own uses.

Storage is separate. The owner may approve their own Amazon S3 or Google Cloud
Storage bucket for permitted analytics archives. Backups use the encrypted
route and target list of spec 0030. The storage screen shows the provider,
region, scope and cost estimate before approval. This is Phase 5 item 6.

Example: an administrator picks "Amazon S3, London region". The screen
shows "about £0.40 a month for your current 18 GB" and the access it
needs. After approval Sutradhar creates a private bucket, uploads
yesterday's analytics, checks each file's checksum, and only then
deletes the local copies.

## What it does

### Terms

- Keyword search: finding text by its words, scored by how rare and
  frequent they are (BM25). Different from the vector search that finds
  similar meaning.
- Built-in index: Tantivy, a Rust keyword-search library inside
  the service. Each site has its own index and active generation.
- Bucket: a named storage area at a cloud provider.
- Provisioning: creating the approved private bucket for the owner.
- Region: the provider's location where the data is kept.

### Scope

In scope: the embedded keyword index, rebuild and coverage, and the
local and approved cloud object stores. Include bounded upload,
verified move, bucket creation, cost warnings and the owner controls.
The storage route supplies evidence only for its approved provider,
account, destination and object operation. Cloud helper execution has
its own [0031 helper boundary][storage-helper-boundary]
and needs separate qualification under QR-242. A verified bucket or
object move gives no helper entitlement or job receipt.

[storage-helper-boundary]:
  0031-shared-platform-installation.md#shape-of-an-installation

Managed and self-hosted search backends have no product configuration,
credentials, provisioner or migration path in version 1 (ADR-0021).
The framework compatibility components of spec 0108 do not enter
the product profile.

Out of scope: the vector index, which stays in the service's own folder
(spec 0022); analytics collection (spec 0034); the content and schedule
of backups (spec 0030), which use the `ObjectStore` here; paid
embedding providers (spec 0025).

### Design

`sutradhar-bhandar` (the storage crate) gains these modules, each
behind a trait with a fake in development-only `sutradhar-test-support`
(spec 0014):

- `SearchIndex`: `upsert`, `delete`, `query`, `begin_generation`,
  `switch_to` and `health`.
- `ObjectStore`: `put` (returns a receipt), `head`, `get`, `list` and
  `delete`.
- `Provisioner`: `quote` and `create` (from an approved quote only).

Modules:

- `bhandar::search`: the embedded `TantivyIndex`. Reuse the canonical
  item and revision identifiers from spec 0022. Search has no
  network route. One lexical store and active pointer per site
  serve spec 0023's mandatory BM25 and the admin search projection.
  Store safe site, item and revision identities and the title, body,
  heading and phrase fields that spec 0023 requires. Spec 0022
  produces current source records. PostgreSQL full-text search
  retains its separately owned database uses.
- `bhandar::objects`: `LocalStore`, `S3Store` and `GcsStore`. Each cloud
  adapter uses its own admitted signing and checksum contract.
- `bhandar::provision`: creates only the private bucket in an
  approved quote, using the exact provider and region.
- `bhandar::archive`: the bounded upload queue, verification and
  guided move. Store limits and retries apply only to storage work.

### Keeping search in step

1. What is indexed: guest-visible thread titles, post chunks (the same
   1,024-token chunks with character offsets), tags and forum names.
   Nothing else. Staff-only and private content never.
2. New, edited, hidden and deleted posts enter through spec 0021's
   site-side change feed and ingestion contract. The registered index
   consumer reads committed spec 0019 outbox events. It starts bounded
   idempotent batches with its inbox receipt. A stale site or source
   revision cannot populate the active generation.
3. A post that stops being guest-visible leaves the active index within
   60 seconds of Sutradhar seeing the change. A thread moved from a
   public to a private forum leaves in one batch.
4. Rebuild in a staged site generation. Track the source checkpoint
   and reconcile later committed edits and tombstones before switching.
   Verify a complete manifest at its reconciled source high-water
   mark. Compare the expected pointer, lease and source and erasure
   fences together in the switch transaction. Reconcile again if any
   moved. Keep the old generation for pinned readers only while
   permitted. Recheck visibility and erasure before delivering
   results even when a reader started before the switch.
5. The Search screen shows which local generations are ready, their
   coverage and missing evidence. Spec 0023 owns BM25, fusion and
   readiness. An unavailable required channel blocks suggestions.
6. A legacy remote search setting is unavailable on upgrade or restore.
   Rebuild locally from admitted source facts. Do not connect to,
   copy from, or delete a legacy service. Show the required recovery
   in the app. Quarantine retired settings, jobs, outbox events and
   credentials inert. Preserve separately admitted bucket keys.
   No typed action or restored record can re-enable a search endpoint.

### Cloud storage

1. Local storage is the default. A cloud archive or backup needs
   separate owner approval of the site, kind, region and bucket.
   Use only the fields and retention admitted under specs 0030
   and 0034. Encrypt approved objects on the PC before transfer.
   The owner holds the backup key. There is no standalone cloud
   search or index upload route. Explicitly approved encrypted
   backups can carry the declared contents of spec 0030. That
   spec owns eligible backup targets and any optional index bytes.
2. Compress and encrypt the object before hashing the stored
   bytes. Use an opaque site prefix and a digest-based object
   name. Object names carry no readable member or site identity.
3. Verify the exact encrypted bytes on upload. If a provider
   checksum does not prove byte identity, use the admitted
   read-back check. Bind the exact owned destination object version
   and ciphertext digest to the receipt's site, store and approval
   generation before any cleanup. At move cutover, recheck the
   destination and its current key and erasure authority. At local
   cleanup, recheck those, the receipt and the separate removal approval.
   On mismatch or lost authority, keep the verified local copy under
   its policy and apply any current erasure fence.
4. Bounded local queue: during an outage, files wait in a local queue
   with a size limit (default 2 GiB). When full, that collection stops,
   the gap is reported with start and end times, and the site carries
   on unaffected.
5. Apply 0034 and DR-029: default raw events 395 days, linkable
   journeys at most 90 days, eligible aggregates two years. Remove
   daily hashes at day close and apply journey expiry to each version. Set and
   check bucket lifecycle against the current policy. Erasure
   fences remain effective before provider cleanup.
6. Erasure follows specs 0030 and 0032. Detailed analytics still use
   the day and visitor partitions. Fence queued uploads and both
   stores during a move. Reconcile every accessible object version
   against the tombstone before making moved or restored data
   available. Unreachable or unverified storage stays held.
7. The guided move: copy whole sealed revisions. Each batch has at
   most 1,000 objects and at most 1 GiB of protected bytes. Apply
   any stricter admitted route bound. Verify each revision, record
   a checkpoint and switch the active store in one step. Remove old
   copies after the administrator confirms. It
   resumes after a restart and reports progress. A region the provider
   retires gets a move offered.

   Check the verified protected byte length of each exact source
   revision at preview and before dispatch. Bind its fenced
   revision ID and proved length to the approved plan. A changed
   revision or length mismatches the plan under 0020 SUT-803.
   A revision must fit one batch. Refuse the whole plan if any
   selected revision exceeds the byte limit. Hold if exact size or
   identity is unproved. Keep the authorised source placement.
   Refusal starts no copy, switch or source cleanup. Streaming
   buffers do not split a revision into Move objects. Backup
   creation, ordinary upload and restore keep their own admitted
   formats, routes and limits.

   Commit the new placement pointer and its switch checkpoint in
   the same transaction under spec 0019. Bind the site, move ID,
   old and new placement generations, and the exact verified
   destination receipts. On restart or a lost reply, resolve that
   checkpoint and compare the current pointer before acting. An
   earlier completed switch never overrides a later placement.

   For this move, a verified copy binds the exact approved sealed
   revision and stored byte digest to the admitted destination
   volume and managed ledger identity, or to the exact owned
   object version. Use the approved checksum or read-back proof
   for those bytes and the admitted durable write receipt. A
   transport success or an eject indication supplies no copy or
   deletion proof.

   When the current destination becomes unavailable after a
   verified switch, hold its affected use. The retained source
   does not become active again. Use the existing exact relink
   for a returning managed volume. A change of active location
   needs its own approved move or restore, with current revisions,
   keys and erasure checks.

   Reconcile a lost source cleanup reply against the original
   approved source identity and the exact 0030 ledger or owned
   object version receipt. An unproved deletion stays held.
   Before any remaining ordinary cleanup, verify current access
   to the active destination and its copy proof. Recheck the
   selected source identity, separate removal approval and
   current erasure and key authority. A detach or a changed drive
   letter provides no absence proof. Preserve existing erasure
   obligations under 0032 independently of ordinary cleanup.
8. After approval, claim the bucket with a conditional marker bound
   to this installation. Verify an unknown result before any retry.
   A different installed marker refuses use. Bind the admitted site
   prefix to each connection and recheck authority before deletion.
9. A bucket deleted by hand: uploads stop, the queue fills, and the
   screen says what happened.

Coach history (spec 0117) is an owner-approved derived-data class.
Only its listed fields may move through the wizard. Use site keys,
encrypted bytes and versioned read-back and erasure receipts.

### What ganit decides here

From ids and numbers only, each answer is inert candidate evidence. Spec
0077 alone evaluates, promotes, activates and rolls it back; a late or failed
candidate leaves the verified champion unchanged:

- Job 123: the keyword index's merge policy, giving tantivy's settings.
- Job 201: disk-use forecast, giving "storage full in N weeks" on the
  Storage screen and a warning before the queue or disk fills.
- Job 241: index and cache disk size, for the settings estimator before
  a rebuild or move.

### Costs and budgets

- Monthly cost is stored size and operations times the provider's
  price for that region, plain arithmetic. The closed-period books for
  it are hisaab's (spec 0035).
- Prices no program can read are entered by the administrator from the
  provider's page and confirmed every 180 days.
- Budget warnings at 50%, 80% and 100% of the monthly budget, on the
  dashboard and as staff alerts. Work continues; the warning says
  provider fees continue.

For the N-1475 estimate, bind the owner-supplied tariff and last
confirmation to the exact site, provider, region, storage class,
currency and billed units. Bind the selected local workload revision
and its usage assumptions. Recheck scope and the existing
reconfirmation interval before displaying or replaying the estimate.
Identical numbers on a tariff for another region or currency cannot
prove the selected cost.

QR-202 must settle the exact cutoff. For approval, use the durable
confirmation instant for that tariff, distinct from its source date.
Under the proposed admitted UTC elapsed-time profile, a confirmation
is current through age 180 × 24 hours, inclusive. Require
reconfirmation at the first representable instant after that cutoff.
A date-only record supplies no exact instant. Future or unverified
confirmation gives no current proof. This proposal does not set an
active boundary.

If a required term is missing, retain still-permitted supported lines
and label any subtotal as known costs only. If the tariff cannot
support the selected current estimate, say “Current storage estimate
unavailable.” Previously recorded eligible evidence keeps its date and
scope. The estimate cannot issue or suppress a booked-cost warning.
Use SC-10 and the [0035 S04 books figure][booked-budget-figure] under
their own authority. Neither an estimate nor its matching amount is
the separately approved, scope-bound provisioning quote in SC-08.
The estimator starts no provider request, purchase or booked charge.

[booked-budget-figure]: 0035-hisaab.md#changes-from-the-source

### Creating resources

1. The owner chooses the provider and region for one private bucket.
2. Sutradhar shows a quote: the exact resources, region, settings
   (private, no public access, the provider's encryption at rest,
   lifecycle rules) and the expected monthly charge with its basis.
3. The administrator pastes a limited key, with the exact policy shown
   (one bucket, no delete of other buckets), and approves the quote.
   Approval is audited. No sign-in library is added.
4. Bind the exact quote, site and bucket intent to the approval.
   Create only that resource. If the result is unknown, reconcile
   the provider state under spec 0019 before retrying. Recheck
   the approval before the connection can be used.
5. Connecting an existing bucket needs separate owner approval of
   exact site, kind, region, prefix and scoped rights. It gives no
   create permission. Run reachability, privacy and verification
   checks only within that approved scope.
6. No cloud resource is ever created without an approved quote.

### Data

PostgreSQL, every table with row-level security on and forced:

- `stores`: id, kind, provider, region, bucket, active flag, created or
  connected, lifecycle state, last check.
- `store_objects`: key, store, size, BLAKE3, provider checksum, verified
  at, local copy removed at.
- `store_moves`: id, from, to, checkpoint, counts, state.
- `search_generations`: site, generation, verified manifest,
  source checkpoint, active state and coverage.
- `cloud_budgets`: monthly budget, prices with their source and date.
- Credentials are held by tijori in files only the service account can
  read, never in the database, logs or exports (`secrecy`).

### API and events

- Cedar-checked actions: `bhandar.search.rebuild`,
  `.search.coverage`, `.store.connect`,
  `.store.quote`, `.store.create`, `.store.move`, `.store.verify`,
  `.budget.set`.
- Events: `search.generation_switched`, `search.index_unavailable`,
  `store.upload_verified`, `store.queue_full`, `store.move_progress`,
  `budget.warning`.

### Failures (SUT-820 to SUT-829)

- SUT-820 local index unavailable.
- SUT-821 search index behind by more than 10 minutes.
- SUT-822 bucket unreachable or credentials refused.
- SUT-823 checksum mismatch after upload; local copy kept.
- SUT-824 local queue full; collection stopped, gap recorded.
- SUT-825 provider refused creation; nothing was created, or exactly
  what was created is listed.
- SUT-826 lifecycle rule missing or changed at the provider.
- SUT-827 budget passed; work continues.
- SUT-828 move paused after repeated failures. Show the reason and last
  verified checkpoint. Resume from that checkpoint only after the
  existing scope, approval, space and verification checks pass.
- SUT-829 bucket became public; uploads stop until it is private.

All outside calls have timeouts, retries with back-off and a circuit
breaker. Clock skew with the provider over 5 minutes makes signed
requests fail; the screen says to fix the computer's time.

### Privacy

- Only guest-visible content is indexed or stored. Detailed analytics
  are pseudonymous personal data and stay in the chosen region.
- A bucket that becomes public stops uploads (SUT-829).

### Screens and capabilities

- The Search, Storage, Create, Move and Budget screens meet WCAG 2.2 AA:
  the quote is a plain table with a caption and the charge in words as
  well as figures; region choice is a labelled select, not a map; the
  move uses the accessible progress bar with text ("41,200 files
  verified"); budget warnings use the SDS warning
  component with icon and words. Move progress counts only verified
  revisions committed at a checkpoint. When Pause blocks new claims
  and a started write still needs verification, show "Pausing. Waiting
  to verify a storage write". Hold an uncertain outcome for the 0019
  reconciliation path before retry. Show "Move paused" when the in-flight
  effects have a known outcome and the checkpoint is safe.
- If no measured route profile is admitted, say "Waiting for a
  verified move profile." Dispatch no move work. An independently
  eligible preview remains available.
- If a selected item exceeds the admitted Move size limit, show
  0020 SUT-802 for the selected item and say "One item is too large
  for this move. It stays in its current location." Show its
  verified size and the applicable limit. If size proof is unavailable,
  say "We could not verify the size of one item." Keep the plan
  held until its existing checks pass.
- If a storage write is unverified before a pause is requested, say
  "Waiting to verify a storage write." Show the last verified
  checkpoint. Reconcile the exact pending write in the original
  move before offering retry. Until a verified switch, the source
  remains selected, subject to its current access checks.
- For repeated failures, show SUT-828, the recorded reason and last
  verified checkpoint. Use "Move paused" only after in-flight
  effects and the checkpoint are known. If the admitted run
  deadline has passed, say "This move run reached its time limit."
  If approval expired, say "The move approval has expired." Keep
  the waiting state while an effect remains unverified and show
  its recorded reason separately.
  A restart keeps the original deadline. Offer continuation or a
  fresh review only through the owning action with current proof
  and authority. A held move supplies no switch or ordinary
  source removal approval.
- "Move paused" confirms the move checkpoint only. For a removable
  drive, use N-1471 to say "Sutradhar has stopped using this drive"
  after verifying its current identity, flushed writes, resolved
  receipts and no open Sutradhar handles. If native eject readiness
  is unavailable, show that narrower result and direct the owner
  to the system's safe removal action. Only claim native readiness
  with verified native evidence. Recheck when the volume, mount or
  lease changes.
- Admin search box and `sutra find --content`: CAP-006. Search size
  limit and warning: CAP-030. Storage screen and `sutra store
  connect|create|verify`: CAP-032. Move and `sutra store move`: CAP-033.
  Budget and `sutra budget set`: CAP-034. Search screen and `sutra
  search coverage`: NEW-033-A. `sutra search rebuild`:
  NEW-033-B, "Rebuild the search index without stopping suggestions".

### Observability

`sutradhar_search_query_seconds`, `_search_lag_seconds`,
`_search_coverage_ratio`,
`sutradhar_store_upload_bytes_total{provider,outcome}`,
`_store_queue_bytes`, `_store_move_objects_total`,
`sutradhar_cloud_cost_estimate{provider}` (a number, no names).
Last-success gauges for the indexer, uploader and daily lifecycle check.
Alerts: search behind 10 minutes; queue over 80%; checksum mismatch;
budget passed (a warning, not a stop).

### Facts checked before building

Admit each storage adapter's signing, verified upload and lifecycle
fixtures from dated provider sources. Keep the index's resident and
memory-mapped page cost in spec 0078's resource evidence. Quotes use
the dated price inputs shown to the owner.

Register the version 1 Rust maintenance class
`bhandar.store.move` in spec 0019's job registry. Bind the move ID,
site, immutable plan digest, approval generation, route profile and
placement pointer to its durable identity. Order by site and stable
placement identity across pointer changes.
A repeat with the same authorised intent and digest returns the same
run. A different digest for the same intent is a conflict.

- Pin the original deadline at the first eligible claim using the
  database clock and the stricter of the qualified class elapsed limit
  and the admitted 0033 cap. A wait before the first claim spends no
  attempt. Each claim that creates a new lease and higher fence
  records one attempt against the qualified class
  limit, including recovery. A checkpoint yield under the same valid
  lease records the yield without starting another attempt. Pause,
  yield, replacement, restart and replay retain the original deadline
  and attempt history. Apply the 0019 stable interval before resuming
  after a scheduler yield. Keep infrastructure interruption distinct
  from a product test failure.
- Use one 0019 retry policy for move effects. Retry only an eligible
  transient failure within the attempt and elapsed bounds, with seeded
  back-off and the provider's applicable retry delay. Reconcile an
  uncertain effect against its original intent before authorised
  replay. An expired deadline fences new dispatch and pointer switch.
  Keep the 0033 resource promise charged for an unknown crash outcome
  until a person resolves it. Record any earlier completed effect
  through its existing reconciliation path.
- Bind profile evidence to the adapter and host, the maximum
  entries and protected bytes in a plan manifest and per batch,
  attempts as a count,
  elapsed limit in seconds, lease, heartbeat, back-off and stable
  interval in milliseconds, the memory promise and each volume
  promise in bytes, maximum seconds to the next safe stop point,
  and maximum repeat work in revisions, protected bytes and seconds.
  Use the 0033 cap, safe-running record and disk checks, and the
  0078 resource profile. Measure checkpoint and stop behaviour with
  live work, an interruption and the largest admitted revision.
  The shared 0110 lease and retry settings apply only to primitives
  that use that policy and meet the admitted bound. A short-call
  timeout supplies no whole-move or streaming qualification. A
  missing or unproved profile holds dispatch.

Before the first claim, match the exact job schema, adapter route and
host profile to their protected QR-202 qualification record. Verify
the measured bounds and receipt identity through the owning profile
record, and bind its digest in the run. A route label, a profile from
another host or a caller's timing value cannot substitute. Recheck
current admission before any new execution on a later claim. Keep an
independently eligible preview available while dispatch is held.

An older move record may contain a placement and cursor without the
exact approved plan, original profile proof, deadline, attempts or
fence history. Recover such a record for authorised inspection and
reconcile recorded effects under the original identity. Verify an
existing result through the same scoped ledger or destination proof.
Resume only when authoritative records prove the same admitted run
and its original bounds. Missing history cannot start a new clock,
clear an attempt, authorise a write or switch a placement. Offer a
fresh move preview only through the current owner route, after any
unresolved original effect is accounted for. Preserve the confirmed
placement and the existing source cleanup boundary.

#### Decision and proof for any future part representation

Before admitting any move of a revision larger than one batch,
record a separate owner decision for the exact data classes, routes,
and bounded representation before qualification under QR-202. The
profile must define a canonical manifest pinned to the exact site,
source revision, protected byte length and whole-revision digest.
It must specify the order, byte ranges, lengths and authenticated
proof for every part, with limits on count, bytes, resource promises
and repeat work. Admit the actual protection and destination
representation with its app and storage readers.

Require exact durable part receipts and checkpoints. Prove safe
restart and lost-reply reconciliation, rejection of missing, repeated,
reordered or substituted parts, verification of the complete
protected revision before switch, and erasure across every managed
part and the assembled revision. Hold partial output and leave the
authorised source intact until verified cutover. Measure the boundary
cases and stop behaviour on each admitted route. This decision and
qualification are required before any part-based Move. The current
upload buffer, backup and restore routes supply no part admission.

### One-item preview and storage forecast (owner job catalogue)

- **N-1384, single-page test drive.** Use the existing 0099 dry-run
  owner action and 0038 storage admission for one eligible site item
  selected by the owner. Pin an admitted local copy, or the exact
  separately admitted read-only path already permitted for that
  preview. Label a qualified brokered read and keep it free and
  non-mutating. No redirect or extra fetch. Default to local.
  Apply one monotonic five-second budget from start through fetch
  and preview, or the stricter qualified limit. Process only the
  eligible item through the dry-run projection. Show unmeasured or
  unfinished steps explicitly. No whole crawl, native write, production
  save, token transfer, provider spend or new fetch path. Clean
  all scratch after success, cancellation, timeout or failure.
  A late stage result cannot become eligible after timeout.
  See [FX-0038-W5-CAPPED-ONE-ITEM-PREVIEW][w5j].
- **N-1393, hard-drive space guard.** Use the exact destination volume,
  latest measured free space, current promises, admitted floor and job
  forecast from the existing 0038/0033 contracts. Include all admitted
  temporary and permanent work on each relevant volume. Reserve against
  the measured scope at claim. A stale or unavailable reading, missing
  floor, or incompatible destination holds work. Use the existing
  low-space state and recovery hold. Show the affected job and reason
  without guessing a deadline. No automatic deletion, relocation,
  provider call or change to storage approval.
  See [FX-0038-W5-SAME-VOLUME-PROMISE-HOLD][w5j].

QR-199 exercises exact preview timeouts, stale volume readings,
concurrent reservations, rights loss and cleanup before hand-off.

[w5j]: ../planning/feature-wave-5.md#acceptance-design-for-n-1382-to-n-1405

### Removable storage and the approved S3 wizard (N-1469–N-1477)

Reuse the 0038 storage registry, guided move, verification receipt,
queue and fields. Keep the 0030 eligible archive classes, the 0034
retention and 0032 erasure rules, and 0118 encryption and recovery
fences. Pin the owner, site, volume or store, exact operations, data
class, current approval, retention, policy and configuration
generation. The runtime and worker gates of 0078/0104 and the 0019
durable job own execution and stop. Changed rights or unavailable
proof hold new work and delivery. No feature grants storage access
through an agent, restored setting, job or inactive project.

| Owner catalogue entry | Exact scope |
| --- | --- |
| N-1469 Plug-in SSD auto-detector | When enabled, use the admitted native volume event and safe drive metadata to offer an eligible move. Identify the volume independently of its label and mount path. Show compatibility, space and unknowns. Never mount a device, browse media contents or move data on detection alone. [FX-0038-W5-VOLUME-DETECTION-NO-ACCESS](../planning/feature-wave-5.md#acceptance-design-for-n-1454-to-n-1477). |
| N-1470 Background data mover with pause | Move the exact approved data classes in bounded batches while unaffected work continues. Bind each lease to verified source revision, exact volume or store, current approval and erasure and immutable destination byte receipt. Recheck those and free space before resume. Write encrypted destination bytes, verify and checkpoint each revision. At cutover, reconcile accessible revisions against current tombstones and intervening changes or keep the verified source active. Switch the pointer once. Pausing fences new claims and drains in-flight effects to known state. Old copies need separate removal approval. [FX-0038-W5-MOVE-PAUSE-VERIFIED-SINGLE-SWITCH](../planning/feature-wave-5.md#acceptance-design-for-n-1454-to-n-1477). |
| N-1471 Safe-unplug green light | For the exact managed volume and move generation, close new leases, drain and prove write flush, resolve receipts and verify zero owned handles before showing safe. Expire the indication on a new lease, remount or volume change. A late native result from before that change cannot restore the indication, even for the same named volume and path. If current Sutradhar release is proved but native eject readiness is unavailable, show the narrower Sutradhar state and direct the owner to native safe removal. [FX-0038-W5-UNPLUG-DRAINED-VERIFIED-VOLUME](../planning/feature-wave-5.md#acceptance-design-for-n-1454-to-n-1477). |
| N-1472 Missing-drive calm mode | Show the missing exact volume and which projects or actions depend on it. Hold affected paths at the last verified checkpoint. Keep unrelated projects available under their scope. Rebind only after verifying the returning volume, data and current rights; a similarly named device at the same letter is insufficient. Never silently copy to a new drive or the cloud. [FX-0038-W5-MISSING-DRIVE-EXACT-VOLUME-RESUME](../planning/feature-wave-5.md#acceptance-design-for-n-1454-to-n-1477). |
| N-1473 AWS S3 connection tester | Preview the exact owner-approved AWS account context, region, bucket, isolated probe prefix, bounded bytes, operations and dated cost risk. Bind one unique never-reused reserved object key, exact digest and resulting version to the exact permitted synthetic write, read-back and cleanup receipt. Check current rights and cleanup policy before the write and every effect. Reconcile an uncertain PUT or cleanup against that exact key and proved version. Never delete by prefix or current key or overwrite foreign content. Unproved residue holds setup. At expiry or revocation, hold and show the affected version, with only the existing owner-approved cleanup route available. Use no project content. Detect versioning and holds before writing. Unknown create or cleanup results stay held and are reconciled before retry. Show separate read, write and cleanup proof. A test grants no later upload or bucket-create right. [FX-0038-W5-PROBE-EXACT-VERSION-RECONCILIATION](../planning/feature-wave-5.md#acceptance-design-for-n-1454-to-n-1477). |
| N-1474 Automatic hot-and-cold splitting | Use the approved placement policy and recorded activity of the exact project and eligible object classes. Leave unknown or active-use data where it safely is. The policy can queue bounded verified moves between already approved stores. Keep active search local. Cloud placement accepts only eligible owner-approved encrypted archives and backups. Any new store, data class or remote operation needs its own approval. [FX-0038-W5-COLD-MOVE-APPROVED-PLACEMENT](../planning/feature-wave-5.md#acceptance-design-for-n-1454-to-n-1477). |
| N-1475 Cloud cost estimator | Use the existing local volume and usage model with a dated, owner-supplied regional tariff and currency. List retention, class, rounded units, request, retrieval and transfer assumptions and unknowns. Apply the existing estimate and warning rules. No pricing feed, cloud inventory scan, billing permission, account charge or purchase from an estimate. Stage the estimate before changing only the selected region or currency in separate cases. Identical old tariff numbers supply no current total. Compare confirmation at 179 and 181 days. Only under the proposed QR-202 profile, compare one represented instant before the cutoff, equality and its first successor. The first two remain eligible and the successor needs reconfirmation. A missing transfer term leaves a labelled supported subtotal. In separate SC-10 cases pair a stale estimate with current authorised books above an existing threshold, then a high current estimate with missing or incompatible books. Only the valid books give a booked warning, and authorised work continues. Offer an eligible estimate with the same amount and scope as a real provisioning quote in SC-08. The estimate alone refuses before any remote write. The separately approved exact quote follows its own tests. No provider call, booked charge or purchase follows this fixture. [FX-0038-W5-LOCAL-DATED-CLOUD-ESTIMATE](../planning/feature-wave-5.md#acceptance-design-for-n-1454-to-n-1477). |
| N-1476 One-click private S3 bucket maker | Use the existing exact quote, approved region and single-bucket setup intent under 0038. Preview effective data access, encryption and lifecycle and provision only the approved bucket and safeguards. Verify exact region, owner, all approved public-access blocks, effective policy and object ownership, compatible private access and exact claim marker before activation. Do not depend on a legacy object ACL when bucket-owner-enforced ownership disables ACLs. Distinguish observed existing bucket from original creation proof. A lost create reply supplies no claim. Keep partial setup staged and unusable. Reconcile the original effect without deleting or recreating by name. Reconcile the exact configured account, bucket and region and conditional SC-08 claim marker, with generation and current approval at claim and activation. A different or competing claim, unknown ownership, policy mismatch or uncertain receipt holds. A bucket approval alone allows no unrelated resources, IAM expansion or project uploads. [FX-0038-W5-PRIVATE-BUCKET-EXACT-CLAIM](../planning/feature-wave-5.md#acceptance-design-for-n-1454-to-n-1477). |
| N-1477 Offline queue for AWS S3 | Use the local encrypted outbox for eligible approved objects, with the existing 2 GiB default. Reserve exact bytes before enqueue, deduplicate, report a timed collection gap at the cap and leave site pages unaffected. Apply the current verified store, site, approval, source revision and tombstone fences to each release lease. Reconcile an uncertain upload and verify exact encrypted bytes before any local cleanup. Network return, restart or restore cannot open a suspended grant. [FX-0038-W5-OFFLINE-OUTBOX-QUOTA-RECHECK](../planning/feature-wave-5.md#acceptance-design-for-n-1454-to-n-1477). |

Use the verified provider semantics for ACLs, ownership and actual
access. A versioned probe cleanup must reconcile the exact version
written. A current-key deletion marker leaves underlying object versions
intact. Verify permitted deletion of the exact version and read-back
under the scoped probe authority. Keep the least-privilege probe isolated from
archive traffic. Use synthetic provider fixtures to qualify failure,
expiry, versioned cleanup and uncertain receipts under QR-202.
[Amazon S3 access control](https://docs.aws.amazon.com/AmazonS3/latest/userguide/access-management.html)
[Amazon S3 versioning](https://docs.aws.amazon.com/AmazonS3/latest/userguide/versioning-workflows.html)

#### Storage previews and verified movement (N-1478 to N-1499)

Use the existing owner-approved storage connection, current scope,
0030 ledger and 0118 storage custody. Preserve separate authority to
preview, move, provision or delete. A stale receipt or approval
cannot move a pointer or remove an old copy.

| Feature | Exact storage or move contract | Existing proof anchor |
|---|---|---|
| N-1478 Built-in file shrinker | Apply only the admitted bounded lossless format to eligible captures or logs through existing archive or move work. Compress before sealing and hash the stored protected bytes. Verify decode and identity within the admitted profile. An unverified, interrupted or over-budget result cannot replace the intact eligible copy. No new collection, cost guarantee or provider call. | Cloud storage, SC-05, SC-07, SC-16 · [FX-0038-W5-BOUNDED-LOSSLESS-PROTECTED-COPY](../planning/feature-wave-5.md#acceptance-design-for-n-1478-to-n-1501) |
| N-1480 Drag-and-drop drive mover | Make drag and keyboard selection produce the same reviewed move plan, with exact source, target volume or approved bucket, site, size, protected object and erasure generations. Recheck current authority and destination before staging and switch. Reuse verified copy, one atomic switch and separately approved cleanup. Pause, disconnection or a stale target leaves the last verified location usable where authorised. | SC-07, SC-08, SC-14 · [FX-0038-W5-VERIFIED-VOLUME-SWITCH](../planning/feature-wave-5.md#acceptance-design-for-n-1478-to-n-1501) |
| N-1482 S3-compatible cloud presets | Provide setup profiles only for already admitted storage adapters, endpoints and retention or protection contracts. Names in the source, including R2, B2 and MinIO, do not admit a provider. A preset is inert until the exact owner-approved storage setup. Bucket provision, claim and use keep their separate authority. Incompatible or unadmitted endpoints stay unavailable. | Cloud storage, SC-08, SC-09, SC-11, SC-12 · [FX-0038-W5-INERT-STORAGE-PRESET](../planning/feature-wave-5.md#acceptance-design-for-n-1478-to-n-1501) |
| N-1483 Side-by-side sync preview | Compare current eligible manifests or admitted scoped observations of each exact storage side. Show only safe identities, versions, verified hashes, missing and conflicting entries and coverage. Bind each side's read and release to its exact current storage scope and generation. If one side loses authority after partial consumption, hold its pending reads and withhold buffered or late results. Show “Comparison unavailable. One location cannot be checked.” Keep only the other side's separately permitted view. Release no aggregate, difference, equality or readiness derived from the held side. Restart or restored scratch cannot resume that preview. A fresh comparison needs both exact current scopes. A stale or unreachable side cannot imply equality or readiness. No upload, overwrite, new remote listing route or deletion follows from viewing the plan. | SC-05, SC-07, SC-08, SC-15 · [FX-0038-W5-MANIFEST-DIFFERENCE-PREVIEW](../planning/feature-wave-5.md#acceptance-design-for-n-1478-to-n-1501) |
| N-1499 Drive-letter repairman | Propose a relink only after proving the same eligible volume and durable ledger identity at a new local path. Bind one exact site and its allowed managed groups, confirm through the existing owner route and verify every affected reference. A matching path or label alone gives no proof. A wrong, substituted or unavailable volume stays disconnected. MCP can report or draft only. | 0030 test 24, SC-07, SC-08 · [FX-0038-W5-RELINK-EXACT-VOLUME-AND-SITE](../planning/feature-wave-5.md#acceptance-design-for-n-1478-to-n-1501) |

QR-203 qualifies format expansion, exact local and remote manifests,
provider and volume identity, signed confirmation, single switch and
restore. Use sealed synthetic fixtures for every acceptance case.

Use 0038's admitted Amazon S3 and separately admitted Google Cloud
Storage paths only. R2, B2 and MinIO presets stay unavailable.
An unavailable hint supplies no endpoint or credential field.
Refuse endpoint or header overrides and changes by restored data.

#### S3 access refusal explanation (N-1502)

Bind the exact site, owner-approved store, operation, region,
credential generation and protected request receipt. Show the
safe observed refusal or uncertainty. Distinguish unproved
credentials, missing access, clock error, unreachable store and
public bucket using current admitted evidence. A scoped local
agent may read only the authorised scrubbed projection. Remove
credentials, provider policy text, private object names and
account identifiers from any wider output. A suggested remedy
identifies the existing Storage screen or the exact owner step.
The suggestion cannot edit IAM, bucket policy, keys or provider
accounts, test an arbitrary bucket or expand access. The owner
may separately use the existing admitted connect or verify
action with a current grant. A lost effect receipt stays
unresolved until the owning route verifies it. Keep local data
under the existing hold and cleanup rules.

Trace SUT-822, SUT-829 and the existing storage refusal fixtures.
[N-1502](../planning/feature-wave-5.md#acceptance-design-for-n-1502-to-n-1525)
uses fixture `FX-0038-W5-SAFE-S3-REFUSAL-PROJECTION`.
QR-204 qualifies the exact protected explanation and authority
checks using synthetic provider outcomes.

#### Storage setup preview (N-1871)

Use three local wizard steps for the existing storage actions.
First choose an eligible managed local destination, verified
removable drive or separately approved own AWS S3 destination.
Then show exact volume or store identity, site, scope, archive
class, rights, estimated costs or unknowns and the permitted
verification or move. Finally request the existing exact owner
confirmation for the particular action and use its owning
receipt. Show a safe hold when a check, destination or approval
changes.

Detecting a drive, editing a path or previewing a bucket gives no
mount, file grant, network call, provisioning, spend or data
movement. Pin the relevant protected path and source version
and recalculate if scope changes. An S3 verification, creation
or copy remains distinct under the admitted N-1473/N-1476 and
0038 contracts. The approved encrypted transfer needs its own authority before
bytes leave. Before activating the pointer, verify exact read-back
and protected receipt. Removing the local copy needs separate
cleanup approval. Trace SC-05, SC-07 to SC-09
and QR-219.
See [FX-0038-W5-EXACT-STORAGE-PREVIEW-AUTHORITY][w5-storage-preview-case].

[w5-storage-preview-case]:
  ../planning/feature-wave-5.md#acceptance-design-for-n-1863-to-n-1880

## Limits

- Inside the service slice, within the memory limits in AGENTS.md section 5.6:
  tantivy writer 64 MiB heap; readers 64 MiB of caches; REST client and
  uploads 32 MiB of buffers. Uploads stream and never hold a whole file
  in memory. These are estimates, measured on Dell.
- The index files are memory-mapped. The slice counts the page cache of
  mapped files, though the kernel can reclaim it; the measured peak must
  include it.
- Local queue 2 GiB by default. Move batches at most 1,000 objects and
  1 GiB of protected bytes, subject to stricter admitted route bounds.
- Speed targets on Dell with the 50,000-thread tier: built-in keyword
  query p95 under 50 ms; a new post searchable p95 under 5 s; a hidden or
  deleted post gone under 60 s; admin search p95 under 300 ms end to
  end; verified upload at least 20 MB/s; a built-in rebuild at least
  2,000 posts a second, yielding to live work.

## Footprint budget

This part adds nothing to public pages. An outside failure never blocks
a site page.

## How it is tested

Rust tests with fakes unless named:

- SC-01: with public, staff-only and private posts, only guest-visible
  posts are searchable in the one site's local index.
- SC-02 (test site): a post hidden in XenForo leaves the active index
  within 60 s.
- SC-03: Tantivy returns the expected fixture threads. Upgrade and
  restore a legacy search setting with a synthetic saved credential,
  a pending job and an undelivered outbox event. Restart and exercise
  ordinary recovery of jobs and events with those records present.
  The Search screen shows the retired route is unavailable and
  offers a local rebuild. The retired records stay quarantined and
  their credential stays hidden. Assert no runnable remote job,
  remote call or create action. Rebuild from admitted local source
  facts and preserve the independently approved bucket from the
  mixed backup.
- SC-04 (and chaos): a rebuild with queries and edits running answers
  every query and loses no edit. Kill it before and after the
  pointer switch. Erase between staged verification and switch
  and while a reader holds the old pointer. Replay an older edit
  and verify no stale hit or answer.
- SC-05 (property): corrupt one upload in ten. Separately give exact
  ciphertext proof from a different object version at the approved
  store, or replace the approved removable volume with a different
  volume at the same path. Hold cutover and local-copy cleanup in
  each case. Revoke the destination key or move grant after matching
  read-back, before cutover or cleanup. The previous receipt cannot
  authorise removing the source. Any separately required erasure
  stays pending until absence is proved for selected copies and keys.
  Separately lose the switch reply after its commit and detach the
  destination before source cleanup. Verify the recorded switch
  once, hold affected use and leave the retained source inactive.
  Reconnect the proved destination and start only the separately
  approved source cleanup. Lose its reply and restart. Use the
  exact source ledger or version proof to settle that effect once.
  In another run substitute a volume at the old source path. Leave
  it untouched and withhold deletion proof.
- SC-06: an outage longer than the queue stops collection at the
  limit, shows the gap with its times and leaves pages unaffected.
- SC-07 (chaos and property): a move of 10,000 objects killed at random
  points finishes with every object verified once and the switch made
  once. In a route that admits the full byte limit, test a whole
  sealed revision of 1,073,741,824 protected bytes on its own.
  It passes the size check and still needs all other Move evidence.
  Test 1,073,741,825 bytes separately. Show the size refusal without
  a move write, a switch or source cleanup, even with eligible
  items in the same plan. After a valid preview, change the
  fenced revision and make it one byte over. Refuse the stale
  plan under SUT-803 and require a fresh preview. With an unproved
  length, hold before dispatch. Backup and restore checks retain
  their existing limits. Pause a separate move after a destination
  write is dispatched
  and race the next batch claim. Lose the write receipt. Check that
  no new claim succeeds, the screen shows pending verification, the
  verified count stays at its checkpoint and no retry repeats the
  unresolved write. Reconcile the exact destination revision. Show
  "Move paused" once the effects and checkpoint are known. Check
  exact resume with current scope and authority, then exercise a
  revoked approval and a substituted destination in separate runs.
  Each invalid resume holds the switch and any ordinary cleanup.
  In a scheduler case, wait for resources before the first claim,
  yield at a verified checkpoint and then lose a lease. Check no
  pre-claim attempt, the same deadline and cursor after yield, and
  a higher recorded fence and attempt after reclaim. Lose a write
  receipt during an interruption and keep its original resource
  promise charged until a person resolves it. In separate cases,
  exhaust the admitted attempt bound, schedule a retry past the deadline
  and omit the measured profile. Check that each case holds new
  dispatch and pointer switch while any earlier effect reconciles.
  In separate cases let the run deadline or approval expire after a
  verified checkpoint. Keep the checkpoint and existing source
  pointer. Show the recorded time-limit or approval reason with any
  still-unverified write. Restarting cannot extend that run. In the
  removable-drive pause case, show only the proved Sutradhar stop
  while native readiness is unavailable. A changed volume or new
  lease clears the earlier stop indication.
  Try the first claim with a valid measured record for a different
  host or adapter profile under the same route label. Refuse the
  claim before any write, deadline or attempt is recorded. Preserve
  the eligible preview. Separately restart a legacy move with a
  saved cursor and unresolved write but missing original bounds.
  Reconcile only the exact earlier effect whose identity and current
  read authority are proved. Supply no new claim, switch, deletion
  or replacement deadline. With complete original evidence and
  current scope, resume the existing run under its next fence.
- SC-08: an unapproved quote or use request sends no remote write.
  Race two marker claims, lose the winner's reply, revoke approval
  before use and deletion, and apply erasure during a move or
  restore. Verify only authorised object versions can be served.
- SC-09: an approved bucket is private, in the quoted region, with the
  lifecycle rules, and nothing else is created.
- SC-10: passing the budget warns and every authorised job keeps
  running.
- SC-11: a bucket turning public stops uploads with SUT-829 until
  private again.
- SC-12: the signer matches the providers' published signing examples
  byte for byte.
- SC-13 (load): 2 minutes of mixed queries and edits on the
  50,000-thread tier hold the query and freshness targets.
- SC-14: every screen and state passes the accessibility checker, and
  screenshots match.
- SC-15: credentials appear in no log, span, export or screen.
  In N-1483, compare two admitted synthetic local volumes. Consume A
  and buffer part of B's manifest before rendering. Revoke B's read
  scope and deliver the remaining old reply. Verify the display keeps
  only A's separately permitted state and the safe unavailable reason.
  B's entries, versions, hashes, counts and any comparison result
  stay withheld, including on restart with the old scratch. Restored
  rights can support a fresh preview under both current scopes. No
  preview writes an object, switches a pointer or removes a copy.
- SC-16: Kani proofs hold and cargo-mutants leaves no surviving mutant
  for the guest-visibility filter, "delete local only after a verified
  checksum" and "create only what was approved", for example "no path
  deletes a local copy whose receipt does not match".
- SC-17: with job 201 overdue, the Storage screen shows the last good
  forecast with its date and an AutoIssue is filed.
- Coverage follows ADR-0020: only admitted strict-core components require
  its 100% line, branch, MC/DC and killable-mutant targets, with equivalent
  mutants proved and independently reviewed; other code follows the owning
  risk matrix. Speed targets are held by the 5% median rule of AGENTS.md 6.8.

## Changes from the source

- Credentials are held by tijori in service-only files, not container
  secrets.
- The built-in index runs in the service in the service slice, not a
  worker container; the old 5.6 GB figure gives way to the limits in
  AGENTS.md section 5.6.
- Search stays embedded under ADR-0021. Only approved encrypted
  archives and backups use the optional bucket route.
- Month-to-date cost books move to hisaab (spec 0035).
- Search and Storage use the desktop SDS components.
- Speed: the 5% median rule replaces the old ratchet.

Source: `docs/specifications/operations/search-storage.md` and
`docs/specifications/operations/search-storage-2.md` in the archive
`archive-2026-11-08-old-repo`.
