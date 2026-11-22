# 0118: End-to-end encryption gate

The desktop app, service, XenForo add-on and WordPress plugin protect
Sutradhar data in transit and at rest. The gate stops a candidate when
a data route, store or supported variant lacks proof. Example: a
hosting proxy can relay a request to review a made-up forum post. Only
the paired add-on can open the request, and only the paired service
can open the reply. The proxy still sees where the request went.

## Words used here

An **endpoint** is a named process or person authorised to open data.
A **protected record** includes site content, identities, drafts,
analytics, tokens, secrets and their reversible or sensitive
derivatives. A **sealed record** is encrypted and has an integrity
check that fails if the bytes or their bound context change.

## Scope and boundaries

| Boundary | Required protection |
|---|---|
| Service to XenForo and WordPress, including replies and errors | Verified HTTPS plus a separately sealed, signed application message |
| Desktop, `sutra`, service, local workers and authorised local MCP clients | Mutually authenticated protected channels on owner-only local transports |
| PostgreSQL, DuckDB, text and vector indexes, caches, logs, queues, backups, temporary work, swap and crash material | Encrypt protected durable bytes with keys kept away from the storage location, or stop the route before writing |
| Add-on tables, files, job payloads and effect receipts | Seal protected records before the database or filesystem receives them |
| Browser pages created by an add-on | Site-native authorisation and HTTPS to the declared authorised endpoint. Deliver only fields the viewer can read |
| Approved external stores, backups and downloads | Encrypt on the sending endpoint. Only the declared recipient has the private key or secret needed to open it |
| Approved integrations, content helpers and incoming public reads | Encrypt transport to the named destination. Send no protected records to a relay or provider unless the recipient is explicitly approved to open them |

The inventory includes public-origin content copied into a Sutradhar
store. Public pages and their original CMS rows stay under the site's
management. A Sutradhar add-on never rewrites the CMS's storage scheme.
Recovery files, diagnostics and exports are in scope until handed to
an authorised recipient. Public release packages, signatures, protocol
versions and necessary routing data may be read openly. The inventory
names every such field and may contain no member or content detail.

An endpoint needs authorised plaintext in memory to process or show it.
The design protects against interception, copied storage, unexpected
relays and wrong-site access. An authorised process taken over while
it can decrypt is outside that protection, as are traffic timing and
copies recipients make after delivery. Shared hosting qualifies only
where it passes the add-on preflight and declares its trusted origin.

## Paired site protocol

1. Keep the service-initiated direction, Cedar checks, guest visibility,
   native review and effect rules of spec 0021. Move protected query
   parameters and operation names into the sealed body. Contract 2.0
   exposes one fixed POST route per adapter with an empty query.
   Signed pairing names the canonical path, and the media type is
   `application/vnd.sutradhar.sealed`. Refuse any other version than
   2.0, redirects, compression and caching.
2. Use single-shot [RFC 9180 HPKE](https://www.rfc-editor.org/rfc/rfc9180.html)
   base mode with X25519/HKDF-SHA256, HKDF-SHA256 and ChaCha20-Poly1305
   (suite IDs 0x0020, 0x0001, 0x0003). Fresh sender encapsulation per
   message. Use independent signing and encryption keys per endpoint
   and pairing. Keep spec 0021's Ed25519 signature over the exact sealed
   body hash and original routing, freshness and request bindings. The
   signature authenticates the sender. Require TLS 1.3 with peer
   verification. Append one line to the spec 0021 signature input:
   the answered attempt ID or empty, followed by a line feed.
3. HPKE info is the ASCII label `sutradhar-site-envelope-v2`. Associated
   data is the ordered header below. Encode text as UTF-8 with a
   big-endian u16 byte length before each field. Opaque IDs use the
   existing canonical form from spec 0021. Time and generation are
   decimal with no leading zeroes. Direction is `request` or `reply`.
   `X-Sutradhar-Request-Id` carries the unique attempt ID in each
   direction. `X-Sutradhar-Answers` carries the answered attempt ID
   on replies and is absent on requests. Its request frame field
   is empty. Seal UTF-8 JSON under the exact operation schema.

| Frame | Ordered fields |
|---|---|
| Associated data | contract version, site ID, pairing generation, direction, sender signing key ID, recipient encryption key ID, method, canonical path, time, nonce, message ID, answered request ID |
| Body bytes | same associated data, 32-byte encapsulated key, sealed bytes including authentication tag |
| Protected JSON | Operation ID and typed payload. Effects bind the immutable logical effect ID, site, actor, operation, both revisions, approval and expiry. The reply echoes logical ID and intent digest with the outcome |

4. Bound the wire frame before allocating and subtract its overhead
   from spec 0021's operation limit. Reject disagreement between
   signed headers and frame fields before decryption. Verify site,
   version, signature, time, nonce and key generation before opening.
   Recheck auth, scope and effect fences after opening. Unknown or
   unauthenticated requests receive one fixed empty error response.
   After valid authentication, return sealed HTTP 200 for every
   application outcome. Other failures expose a fixed empty 400.
5. Each attempt uses a fresh nonce, attempt ID and encapsulation.
   A retry keeps the logical effect ID and expiry. In the effect
   transaction compare the keyed digest of the canonical typed
   intent by site and logical ID. Apply once. Conflict, expiry or
   lost authority refuses before effects. The sealed reply binds
   the attempt and intent.
6. Pair in the native admin view. Confirm site identity, protocol
   version, both signing and encryption key fingerprints and fresh
   possession challenges on the owner PC, using spec 0021's pairing
   approval and matching fingerprints outside the network transcript.
   Pairing may expose public keys, versions and random challenges.
   It carries no site data or credential. An endpoint with a changed
   unapproved key, missing algorithm or failed self-test stays held.

HPKE is a standard envelope, not a new cipher. A later compromise of a
recipient key can open captured messages to that key. Use short approved
exchange epochs and retire decryption keys when old messages have expired. TLS
follows [RFC 8446](https://www.rfc-editor.org/rfc/rfc8446.html). Choose pinned
Rust and packaged PHP libraries through the normal admission gate. Test the
claimed PHP extensions, both add-ons and shared hosting. Availability of a
library alone proves no admission.

### Paired AI observation coverage

The [0021 registry](0021-forum-contracts-ingestion.md)
names the four 0135 operation IDs for this same sealed 2.0 route.
The pull is a bounded read. Acknowledgement, site-view delivery and
verifier control use the protected effect frame and their exact
owning scopes. Apply the existing site, recipient, approval, expiry,
generation and replay checks. Transport authentication supplies no
native read, display or control grant.

Classify each operation's protected schema, receipt and storage
sinks in the existing surface inventory. Require the exact approved
adapter, request and reply profile with IT-0118-LINK and the 0135
IT-0135-FETCH, IT-0135-SPOOL and E2E-0135-NATIVE cases. Hold an
unproved profile under the existing protection gate.

## Other channels and storage

- The Unix socket, named pipe and worker streams use mutual TLS 1.3
  with enrolled install peers. Deliver one-launch worker credentials
  through inherited handles and revoke them at stop. The WebView
  renderer has sealed IPC tied to the declared app origin, window
  and native launch grant. Unverified IPC carries no protected data.
- Protected MCP ends in an enrolled owner-started client or its
  in-process adapter. Require the same process grant and scoped
  token on each request. Plain stdio carries public help or status.
- Bundled PostgreSQL uses mutual TLS 1.3 on loopback only, with the
  installation CA and a named role per client. Refuse local sockets,
  plain TCP and non-loopback peers. No management network listener.
  Prove embedded store calls remain in process and scoped.
- Native staff views use TLS to the admitted origin, with pass-through
  allowed. If an intermediary terminates browser TLS, hold protected
  views. Refuse mixed content, redirects and caches. Prove the
  origin and hosting path with the same runtime checks.
- Protect structured stores with an admitted encrypted volume or
  filesystem. Authenticate storage context at the block or record
  layer. Include WAL, indexes, dumps, replicas, exports, temporary
  work, swap and crash paths. Seal add-on fields before driver
  calls. Raw snapshots carry no unwrapping root. Before the affected
  process opens protected stores on the owner PC at start or restart,
  verify the current native crash capture profile and each
  destination. Use read-only native evidence. A disabled sink uses
  the same profile's sealed absence proof only while its enforced
  boundary still holds. If a changed or unproved sink could capture
  protected memory before an owning protection check can stop it, keep
  the affected route held with SUT-3700. An earlier profile receipt
  grants no start.

- Use XChaCha20-Poly1305 for new sealed records, with a fresh 192-bit
  operating-system nonce, and associated data binding site, store,
  record ID, schema version, kind and key epoch. Existing equally
  protected formats keep their admitted readers under the same proofs.
  For out-of-process or externally retained copies, seal to the
  intended recipient with a distinct key. Use the existing encrypted
  backup formats and recovery rules of specs 0018 and 0030.
- Store ciphertext and public frame fields only. Independent data keys
  are wrapped per site and purpose. Tijori holds the owner PC's
  wrapping roots under OS-backed custody. If a protected key store
  cannot unlock, the app needs the owner's unlock before opening
  stores. An add-on wraps its keys with a site-scoped root kept outside
  its storage database under the native site's private configuration.
  Root custody, recovery, readiness and key access are verified.
- Where an owner has approved an external service to process data,
  the verified provider endpoint is the named recipient. Admission
  documents which fields it will open. A broker, storage operator or
  transit relay is never a recipient by default. Support bundles and
  audit exports carry scrubbed data only and are encrypted if they
  hold any protected record. Mark an incompatible route unavailable.

The [PHP Sodium API](https://www.php.net/manual/en/book.sodium.php)
provides authenticated encryption primitives. Verify the chosen formats
interoperate with the accepted Rust libraries using published vectors
and synthetic application fixtures.

## Rotation, legacy upgrade and deletion

Tijori and the add-on key store expose opaque handles for sign, open,
seal and wrap. Data, link, transport, review and release keys are
separate. States are staged, active, decrypt-only, revoked and
destroyed. Send with active keys only. Planned decrypt-only grace
applies to an authorised set with a bounded expiry. Compromise
revokes immediately. Keep the signed minimum epoch outside restores
and refuse when its current proof is unavailable. Recovery in
isolation needs separate authorisation and verified resealing to new
roots. Record which captured messages and copies may be exposed.

An established connection still needs current transport trust at each
protected request and reply. After verified revocation or when current
trust cannot be proved, refuse the affected boundary before any further
protected effect or delivery. The earlier connection and its handshake
cannot authorise either boundary. Use the owning route's existing safe
refusal and exact receipt reconciliation under current authority.

Mark legacy contracts and stores held until the supported migration
finishes. Freeze reads, writes and uploads to legacy paths. Re-pair
without sending protected traffic on the old site contract. Copy,
seal, verify with the new key, publish a generation and only then
remove legacy managed copies under the owner-approved plan. Keep
recovery copies encrypted. After cutover, rollback may restore only
a build and store that pass the same encryption floor. An unreachable
store or unproved deletion stays explicitly held. Tombstones block
resurrection on restore. Verify removal of every selected version,
external object and managed key, with no success receipt for leftovers.

For an interrupted app update after encryption cutover, recheck the
current signed minimum key epoch from its authority outside restored
state at the guarded previous-payload start under 0030 and 0031.
Bind it to the exact protected store and reader, verified key handles
and current lifecycle checkpoint. Recheck before granting protected
work or resuming writes. Earlier proof cannot pass the current check
after restart or a floor change. Stale or missing proof gives
SUT-3700. A verified current floor with no compatible protected key
path gives SUT-3701. Both hold before the previous payload starts or
takes protected work. Keep current work and recovery evidence sealed.
Recovery does not lower the floor.

For the previous-payload start, keep protection proof separate
from compatibility with the current erasure history. A valid
key path and readable encrypted data grant no fallback if the
required history format cannot be enforced by the exact previous
payload and its admitted read route. Use the current 0030 history
outside the fallback generation and 0032's erasure fence. Keep
protected data sealed when the fallback route cannot prove both.
Apply the existing paused compatibility outcome under 0030 and
0031 for an unsupported history format. Check missing protection
proof and unavailable protected key paths under their own reasons.

## Data

This gate keeps two records, both defined under "Gate contract": the
protection inventory and the gate result. The inventory names every
surface, its data class, protector and key purpose. The gate result
binds the candidate, the artefact hashes and the signed runner evidence.
Use only the typed, safe metadata and signed evidence fields listed
in Gate contract. Use safe identifiers for endpoints and surfaces.
Keep key material, credentials, protected content and identifying
site or member detail out of both records. Key states are staged,
active, decrypt-only, revoked and destroyed.

## Settings

This spec has no owner setting. There is no unencrypted compatibility
mode, so protection cannot be switched off or relaxed. Exchange epochs,
decrypt-only grace and the signed minimum epoch follow the key rules in
"Rotation, legacy upgrade and deletion", and the bounds of each surface
stay with the spec that owns it.

## Gate contract

| Inventory or receipt | Required shape |
|---|---|
| Protection inventory | Stable surface ID, data class, source and sink, intended endpoint, owner spec, platform and route, persisted replicas, protector, key purpose, bounds, allowed metadata, lifecycle and evidence IDs |
| Gate result | Candidate tree and built artefact hashes, target and runtime identity, inventory digest, fixture hashes, coverage by surface, named check IDs, pass or failure codes and signed runner evidence |

1. Aegis owns `security.encryption-coverage` in the gate catalogue.
   Use the admitted graph of spec 0003. Match inventory and observed
   OS sockets, processes, routes, storage writes and package contents
   in both directions. Include schemas, jobs, exporters and clients.
   Unknown, duplicate or bypassed paths fail. A claimed absent path
   needs a sealed negative capability test.
2. Commit checks affected surfaces and their dependants. Push checks
   the whole inventory and known-answer, missing-key, policy and
   lifecycle cases. Release checks the exact packages on Ubuntu 26.04,
   Debian 13 and Windows 11, XenForo 2.3 and both supported WordPress
   majors, with PHP and hosting shapes admitted under their own specs.
   Repeat against proxied and direct HTTPS, offline restore and
   low-resource fixtures, including upgrades. A missing runner, proof
   or dependency fails. There is no unencrypted compatibility mode.
3. The gate checks the cryptographic path with known-answer vectors,
   spies at trust boundaries, raw storage and proxy captures, and
   negative planted faults. Absence of sample text alone cannot pass.
   Check key custody and verify correct decryption, encryption at
   every durable write and failure on wrong site, key, context or tag.
   A signed independent security review checks surfaces beyond static
   analysis, key separation and the exact candidate's complete
   inventory. Sign the exact candidate digest and unsigned payload
   manifest digest under
   [0005](0005-release-coordination.md#exact-candidate-manifest-and-record),
   with the separate protection inventory digest and exact part and
   profile coverage. Verify those bindings before approval and verify
   the signed packaging at publication and install. The detached
   profiles in spec 0005 preserve package bytes and bind the same
   verified bytes at native use. Windows uses spec 0030's independent
   first trust route. Repeat checks on install, upgrade and pair.

## Failure codes

| Code | Condition | Message |
|---|---|---|
| SUT-3700 | Missing or stale evidence, or unclassified surface | “We cannot verify protection for this part.” |
| SUT-3701 | Key store or compatible protected path unavailable | “Unlock the key store or repair this connection.” |
| SUT-3702 | An encrypted message or record cannot be verified | “The protected data could not be opened.” |
| SUT-3703 | Migration or deletion is waiting for proof | “Protection is still unverified. Check the named location.” |

Codes SUT-3700 to SUT-3799 are reserved for this spec (DR-104).

## Security and privacy

The gate records a safe code and surface ID without raw secrets. All
fixtures use invented content, identities and keys. The inventory may
contain no member or content detail, and only public release packages,
signatures, protocol versions and necessary routing data may be read
openly. Support bundles and audit exports carry scrubbed data only. The
design does not protect against an authorised process taken over while
it can decrypt, traffic timing or copies that recipients make after
delivery. Ordinary public pages get zero extra queries, bytes or work.

## Requirements

| Requirement | Acceptance evidence |
|---|---|
| REQ-0118-COVER | UT-0118-COVER adds an unclassified endpoint, store, job and exporter. Each mutation MUST fail. A signed inventory from another artefact MUST fail. |
| REQ-0118-LINK | IT-0118-LINK checks cross-language vectors, proxy captures, header and ciphertext mismatch, replay, wrong-site key, downgrade and encrypted failure on both add-ons. Use [0021 test 32](0021-forum-contracts-ingestion.md#how-it-is-tested) for the authenticated `AcknowledgeEvents` request with an unsupported field. The sealed refusal MUST leave the acknowledged cursor and native prune boundary in place. Repeated refusal MUST leave it unchanged. Retries of a separately eligible effect MUST reconcile once under 0021. |
| REQ-0118-LOCAL | IT-0118-LOCAL tries a wrong pipe peer, old worker identity, plaintext protected tool output and an unofficial client. The app MUST refuse each. Test the legitimate authenticated flows. |
| REQ-0118-HOST | IT-0118-HOST checks successful endpoint TLS and refuses protected browser output behind a terminating proxy, absent host proof, HTTP fallback or browser caching. Both add-ons MUST hold protected routes when preflight fails. |
| REQ-0118-STORE | FI-0118-STORE proves protected writes at every admitted durable sink, including OS swap and crash capture, portable or relocated managed storage, and add-on database and file writes. Bind the result to the exact sink, runtime profile and lifecycle. The raw sink MUST contain no protected plaintext. Wrong-key and unauthorised copied-volume reads MUST fail. |
| REQ-0118-KEY | FI-0118-KEY interrupts rotation, revokes a key while work waits, then restarts. Old work MUST respect the current epoch. Sign and encryption handles MUST not substitute for each other. |
| REQ-0118-MOVE | E2E-0118-MOVE interrupts upgrade, rollback, move, restore and erasure. No write, activation or completion record may bypass a verified generation or revive an erased object. |
| REQ-0118-RELEASE | PL-0118 proves exact artefact and supported variant coverage. Missing runners, side paths, invalid remediation and stale receipts MUST block signing and publication. |

Crypto work on management routes must meet the owning surface's
admitted bounds with the protected protocol enabled.

## How it is tested

Designed, not run. All fixtures use invented content, identities and
keys. Each test id is defined in the acceptance evidence above:

- **UT-0118-COVER:** unit test of the inventory against added surfaces.
- **IT-0118-LINK, IT-0118-LOCAL, IT-0118-HOST:** integration tests of
  the paired site protocol, local channels and hosting paths.
- **FI-0118-STORE:** use the boundary evidence in the gate contract
  for each admitted PC and add-on profile. Include the database, WAL,
  indexes, queue, spill and temporary work, OS swap and crash capture,
  managed exports and backups, and add-on database and file writes.
  On admitted portable, moved and relinked storage, bind proof for
  each destination that can hold protected bytes. Use spec 0030's
  permitted storage profiles and current key custody. A disabled sink
  needs the same profile's sealed absence proof. A result for the
  primary volume or a different host cannot cover that sink. Reuse
  the verified package, keys and protected store on each applicable
  synthetic PC profile. Restart with an earlier crash-sink absence
  receipt and a changed synthetic reading for the native capture state
  or destination. Hold with SUT-3700 before opening the protected
  store or making its data available to the affected process.
  Leave the store sealed. In the control, verify the current sink
  and its protection or enforced
  absence, then apply the remaining start checks. The fixture changes
  only supplied native evidence and leaves host configuration untouched.
- **FI-0118-KEY:** fault injection on key rotation and revocation.
  Reuse 0109 IT-BE-02 and FI-BE-02 where the protected transport uses
  bundle rotation. Keep the same connection open. Revoke transport
  trust before a new request in one run and before releasing a staged
  reply in another. The new request has no protected effect and the
  staged body stays withheld. Verify the separate healthy rotation
  control. Prove any already committed effect only through an eligible
  connection and current authority, with no repeat execution.
- **E2E-0118-MOVE:** end-to-end test of upgrade, rollback, move,
  restore and erasure.
  Join 0031 IP-26 for failed update self-test after cutover. Use two
  copies of one synthetic paused fallback, with current encrypted
  store, compatible previous reader and valid key proof at epoch E.
  Keep the previous live service stopped and interrupt before its
  launch. At restart keep the independent current floor at E in the
  control. Verify exact protected serving once, then record SUT-835
  and the one completed rollback event under 0030. Prove timing only
  through 0030's separate admitted observer.
  After another passing preflight advance the independent floor
  before the guarded start. Present the earlier authentic proof
  unchanged at restart and, in a separate run, without a restart.
  SUT-3700 refuses stale evidence. An unavailable current proof
  gives the same hold. In another branch verify the higher floor
  but show that the fixed previous reader and store have no protected
  key path under it. This gives SUT-3701. Keep payload, store, schema,
  journal, scope and site proofs otherwise fixed. Each refusal leaves
  no launch, key unwrap for the previous reader, protected data
  hand-off, write resume, automatic restore or rollback success.
  Keep the current data and receipts. Join the independent 0031
  IP-26 unsupported erasure-history branch. Keep the exact current
  key floor and key path eligible. A current authentic history
  stored in an unsupported format and an older readable proof
  each fail to qualify that reader. No previously erased data or
  pending export becomes readable. The control proves current
  erasure enforcement before the protected return. Use local
  fixture evidence only with no site query.
- **PL-0118:** planted-fault and artefact coverage check for release.

## Changes from the source

Owner decision DR-103 asks for a gate that enforces encryption end to
end for Sutradhar and both add-ons, not one that proves only signatures
or TLS. This design adds recipient encryption to the signed site
contract and verifies the whole custody path across endpoints and
stores. Failure codes SUT-3700 to SUT-3799 are reserved for it (DR-104).
