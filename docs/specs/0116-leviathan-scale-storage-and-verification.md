# 0116: Leviathan scale, storage and verification

This specification owns three things about Leviathan beyond its legal wall and
its page understanding: how it holds a very large audit (the frontier, the
vault, the archive, the journal, the diff, the link graph and fingerprints),
how a person watches and steers it, and how it is proved. Spec 0082 owns the
legal wall, identity, politeness, modes and fences and keeps only a summary of
the audit storage; spec 0115 owns page understanding; spec 0105 owns link
evidence; spec 0044 owns the site audit that uses the facts; spec 0078 owns
memory and disk caps; spec 0030 and 0038 own the home folder and the storage
wizard; spec 0114 owns the verification of Behemoth Boot and the farm and
chaos machinery that Leviathan reuses.

This spec also owns the two verification rules shared by both crown jewels,
which spec 0114 left to it: the identical Linux and Windows verification
matrix (N-1725) and the reproducible source-to-binary build verifier (N-1726).
They apply to Behemoth Boot and Leviathan alike.

Leviathan is built for one user's PC (spec 0082, ADR-0022). "Enterprise" and
"billion-URL" in the pillar lists are aims: the real limit of every structure
here is a disk or memory cap that the user sets, and capacity is reported as a
measured number, never promised. Repeat crawls and freshness learning work
only on the user's own registered sites. A rival is read once and not tracked
over time (DR-062).

Example: Sam's shop has 400,000 pages. A pre-crawl estimate samples 50 pages
politely and says "about 380,000 to 450,000 pages, 9 to 12 hours at the polite
rate, 6 to 9 GiB of storage, light load on your server". The crawl starts. At
hour 5 the PC loses power. On restart the journal keeps its verified
flushed records and drops a half-written tail. One page check is
unconfirmed. Sam sees the last verified progress and reviews the
original result. The crawl resumes only from work proved safe. Sam
pauses the `/forum/` folder and lowers the speed from the steering
panel. Both changes
are recorded. The next day a second crawl runs, and the diff lists 212 changed
titles, 9 lost canonical tags and one template change that touched 1,900
pages. The vault stores the unchanged files once, and the archive file opens
in a standard WARC reader.

Values marked "proposed" are design choices to be checked by measurement.

## What it does

### Words used here

- **Frontier**: the list of addresses waiting to be fetched.
- **Vault**: the store of fetched files, named by a fingerprint of their
  content.
- **Journal**: a crash-safe log of every crawl step.
- **Fingerprint**: a short number made from content so copies can be found.
- **Tier**: fast local storage (hot) or slow, cheap storage (cold).
- **Project**: one registered site, kept separate from every other.
- **Lab**: the offline test web of simulated sites.
- **Model**: a small formal description of rules, checked by Quint.
- **Matrix**: the same checks run on both operating systems.

### What this spec designs

| Feature | What the user or program sees |
|---|---|
| N-1691 Disk-backed billion-URL crawl frontier | A large queue that lives on disk with a small memory window |
| N-1692 Adaptive freshness scheduler | Sections that change often are revisited sooner (own sites only) |
| N-1693 Multi-node distributed crawl coordinator | Deferred, not built in version 1 (ledger state `scheduled`, QR-048) |
| N-1694 Content-addressed deduplicated vault | Each file stored once per project |
| N-1695 Standard WARC archival recorder | Portable, protected archive files for approved recipients |
| N-1696 Instant historical crawl-diff engine | Changes between two crawls of a site |
| N-1697 Full-site link-graph calculator | Click depth, orphans, link flow and broken hubs |
| N-1698 Power-loss-proof recovery journal | Recover verified progress and hold uncertain fetches for review |
| N-1660 Bit-for-bit offline replay proxy | Recorded exchanges replayed byte for byte for tests |
| N-1699 Automatic hot-and-cold storage tiering | Fast summaries local, bulky snapshots on an external drive or S3 |
| N-1700 Strict multi-project isolation vaults | Projects never mix storage, keys, memory or queues |
| N-1690 Near-duplicate content fingerprinting | Copies and thin pages found across a site |
| N-1709 Live flight-recorder telemetry | Live speed, status, queue and memory view |
| N-1711 Live mid-crawl steering controls | Pause, speed, priority and exclusions during a crawl |
| N-1712 Interactive site-discovery visual map | How the crawl moved through the site |
| N-1713 Pre-crawl impact estimator | A forecast from a tiny polite sample |
| N-1714 Server-log correlation engine | Crawl structure beside what real bots hit |
| N-1715 Open MCP diagnostic interface | Read-only crawl diagnostics over local MCP |
| N-1717 Full Web Platform Tests conformance suite | Browser-standard tests of the page engine |
| N-1718 Continuous 24/7 grammar-guided fuzzing farm | Structure-aware fuzzing of every parser |
| N-1719 Formal Quint discipline proofs | Models of the scheduling and compliance rules |
| N-1720 Self-contained synthetic mini-internet lab | An offline web of simulated sites |
| N-1721 Automated reference-browser visual oracle | Comparison with a mainstream browser (proposed, undecided) |
| N-1722 Network chaos and fault-injection simulator | Broken networks and dying workers |
| N-1724 Multi-day soak and memory-endurance lab | Long runs that find slow leaks |
| N-1725 Identical Linux and Windows verification matrix | The same checks and the same results on both systems |
| N-1726 Reproducible source-to-binary build verifier | Two builds of one source give the same bytes |

### Frontier

1. The frontier is a log-structured merge store: new addresses go to an in-
   memory table (`crawl.frontier_mem_mib`) and are written to sorted,
   compressed runs on disk, which are merged in the background under a write
   budget. A small Bloom filter in front of the runs is used only as a
   negative shortcut ("surely not seen"); a positive answer is always checked
   in the exact index.
2. Invariant: an address is in the in-memory table or in a sorted run before
   it is handed to a fetcher, and the "already seen" test looks in both. So a
   merge or a crash between two lookups cannot cause a second fetch of the
   same address. After a crash the memory table is rebuilt from the journal,
   which is the source of truth for what was claimed.
3. Each host has its own queue so politeness (spec 0082) is applied per host.
   Order inside a host follows priority (steering, importance, depth) and then
   discovery order.
4. Size is bounded by `crawl.frontier_max_gib`. At the bound, new addresses of
   the lowest priority are dropped with a counted note, never silently, and
   the report says the audit is incomplete (SUT-1600). "Billions of URLs" is
   an aim; capacity is the measured number of addresses per GiB (MS-LS-01).
5. Memory stays inside the audit-mode share of spec 0078 whatever the queue
   size. Compaction has a stated write-amplification budget so it cannot
   starve fetching.

### Freshness scheduler

1. For a registered site only, the scheduler learns how often each section
   changes, from the content fingerprints of earlier crawls and from
   conditional requests (entity tag and last-modified), and suggests a revisit
   interval per section between `crawl.freshness_min_h` and
   `crawl.freshness_max_d`.
2. It never raises the request rate above the caps of spec 0082; it only chooses
   which pages to fetch within them. A rival or any site not registered is never
   revisited on a schedule (DR-062).
3. A section the learner cannot judge (too few samples) keeps the default
   interval and is marked "not enough history".
4. The user can see the learned intervals and override any of them.

### Multi-node coordination (deferred)

Splitting one crawl across several machines needs the machines to talk to each
other, which ADR-0021 forbids (no network port, no remote bridge). Version 1
therefore has one machine. Several processes on one machine are allowed.
Whether a later version may coordinate the owner's own machines is an owner
decision (QR-048), and the feature stays `scheduled` in the ledger until then.

### Vault and archive

1. The vault stores each fetched file once per project. The file's identity is
   the SHA-256 of its plaintext content, used internally; the stored name is a
   keyed hash of that value, so listing the folder reveals nothing about
   content to anyone who lacks the key. A second crawl of an unchanged file
   adds a reference, not a copy. Files are compressed, then encrypted.
2. Deduplication is within a project only. It never crosses projects, so one
   project's storage cannot reveal that another holds a file.
3. Write order is: the file, then its reference, then the journal record, so a
   crash never leaves a reference to a missing file. The sweep (below) can
   only delete files with no reference.
4. Sweep: a mark-and-sweep run under the project lock deletes only files that
   have been unreferenced for at least `crawl.vault_grace_h` hours (default
   24), so a crawl that adds a reference during the sweep cannot lose its
   file.
5. The archive recorder writes standard web archive files (WARC, ISO 28500)
   with a record per request and response, record digests and per-record
   compression, split at `crawl.warc_max_mib`. Only the user's own sites and
   the lab are archived (spec 0082). Archive and replay use only the
   scrubbed exchange from spec 0082, with all `Set-Cookie` fields
   removed. Digests cover the scrubbed records.
6. Keys. Each project has a data key, made when the project is registered and
   wrapped by the install's key held by tijori (spec 0065). Vault files and
   archive files are encrypted at rest with an authenticated cipher chosen in
   the build-or-reuse record. "Export archive" puts a standard WARC inside
   a sealed export for the authorised recipient, under spec 0118. LS-07
   checks the WARC after the recipient has opened it.
   Backups (spec 0030) carry the wrapped key. Losing the install's key means
   the vaults cannot be read; the screen says so before a project is created
   and in the backup guide. Key rotation re-wraps the data key; a full re-
   encryption is optional and runs as a job.
7. Every snapshot and report is signed with the install's signing key (spec
   0082, spec 0065) so later tampering is detectable. Verification is an
   action of spec 0020.

### Journal and recovery

1. Every crawl step (address claimed, fetched, parsed, stored, links added) is
   appended to a journal with a length and a checksum per record. "Recorded"
   means flushed: the journal is flushed at least every
   `crawl.journal_fsync_ms` (default 100, at most 1,000). The commit primitive
   is a data sync of the file and its folder on Linux and `FlushFileBuffers`
   on a file opened write-through on Windows; each is tested on its own
   system. Publish the checkpoint only after verified durable flush.
   An uncertain checkpoint write reconciles the original record and
   publishes no progress or new dispatch until its result is verified.
2. Under 0019 and 0082, the sole outbound adapter commits
   `TransportPossible` before handing an HTTP attempt to transport.
   Use the durable compliance intent, exact project, logical
   request, attempt, fence, storage generation, target, deadline
   and reservations. Require current permission and pacing. If
   that commit cannot be verified, do not hand off the request.
3. On restart verify the current compliance intent, project fence
   and storage generation. Read the monotone checkpoint from the
   protected project execution ledger outside the restorable journal.
   It binds the project, storage
   generation, committed sequence and record digest. Verify the
   record chain through that point before replay. Discard only a
   proved incomplete final append beyond that boundary (SUT-1601).
   A later physical record supplies no commit or no-send proof.
   Damage within the boundary or inability to verify it holds the
   affected crawl with SUT-1602. Do not truncate when the boundary
   is unproved. A verified fresh journal has an admitted empty
   checkpoint. Rebuild local state only from verified records.
4. Reconcile an eligible completed effect against its exact receipt
   and retain its result once without sending. Reclaim an ordinary
   visit only when the current adapter proves no handoff was possible
   and the old dispatch fence is closed against late sends. Record
   the adapter's terminal no-handoff proof with the closed fence
   against the original attempt, from the intact current intent
   ledger. A missing row, stale backup, expired lease or lack of
   reply cannot prove it. A possible handoff retains the longest
   applicable host or shared-address wait. Requeue through the
   existing scope, scheduler, policy and reservation checks. Keep one logical
   request and close the old attempt before opening another.
5. A possible handoff without verified outcome stays held with the
   original reservation, waits and unknown coverage. A backed-up
   journal cannot clear the current receipt or charge. The flush
   window bounds unfinished local work and grants no retry. Proved
   retryable failures still follow their owning route. Preserve
   0082's spent rival claim, 0120's consumed one-off approval and
   0131's spent slot and crash-terminal plan. A proved unsent result
   cannot revive those grants. Existing 0110 storage is idempotent.
6. When the disk is full (the disk cap of spec 0078 or the drive itself), the
   crawl pauses at its last verified checkpoint. Show SUT-1615 and
   preserve the journal. A failed flush advances no checkpoint.
   Keep new claims held and delete nothing to make room.

### Storage tiers

1. Hot data (summary tables, the frontier, the journal, indexes) stays on
   local fast storage. Cold data (raw snapshots, archive files) may move to an
   external drive or an S3-compatible bucket the user owns, through the
   storage wizard of spec 0038 and the file ledger of spec 0030.
2. Moving is a background job that can pause (spec 0099). A moved file keeps
   its fingerprint. It is verified by reading it back and comparing its hash
   with the manifest; a storage service's own checksum may be an extra check
   but is not trusted alone (a multipart upload's entity tag is not a content
   hash). The local copy is deleted only after a verified read-back.
   Credentials for a bucket come from tijori (spec 0065). A failed move pauses
   the move job and the crawl continues (SUT-1605). The file is fetched back
   on demand. With the drive missing the audit says "Drive not connected" and
   the jobs that need it pause (spec 0038).
3. Nothing is sent to the outside without the user's switch for that
   destination (ADR-0021); a test checks that a move to an S3 destination that
   is switched off is refused.

### Project isolation

1. Each project has its own vault folder, encryption key (from the secret
   store of spec 0065), memory budget slice, queue, journal and archive files.
2. Mechanism: a crawl of one project runs in its own process (or cage),
   started with only that project's key handle and paths. Its memory slice is
   a hard limit on that process (a control group on Linux, a job object on
   Windows), so a crawl that exceeds its slice is stopped alone and no other
   project is touched.
3. Every table carries the site column and forced row-level security (spec
   0108). A crawl task opens the site transaction of its own project (spec
   0110).
4. A test that tries to read another project's key, files or queue from a
   project process must fail.
5. Removing a project removes its vault, key and files (spec 0030).

### Fingerprints and near-duplicates

1. For each page's main content (spec 0115) the engine makes a 64-bit
   similarity fingerprint from word groups. To find all pairs that differ by
   at most `d` bits (`crawl.simhash_hamming`, proposed 3, allowed 0 to 6)
   without comparing every pair, each fingerprint is split into `d + 1` parts;
   two fingerprints within `d` bits must agree exactly on at least one part,
   so one sorted on-disk table per part gives the candidates.
2. Candidates are confirmed by a second comparison, the overlap of word groups
   (Jaccard) at or above `crawl.confirm_jaccard` (proposed 0.8). Only
   confirmed pairs are reported.
3. Proposed targets on a labelled fixture: recall at least 95 percent and
   precision at least 90 percent (MS-LS-02).
4. Memory stays bounded by working one table at a time across millions of pages.
5. Reports say "near-duplicate" with the similarity value and the sample size
   and never claim a page was copied or by whom. It works within one project's
   own audit crawl. The one exception is the opt-in site group of spec 0102:
   derived, encrypted records of guest-visible main content may be compared
   across the sites of a group chosen by the owner, and nothing from the
   vault, the archive or the keys crosses.

### Crawl diff and link graph

1. The diff compares two crawls of one project by merging their sorted on-disk
   tables, so memory stays under `crawl.diff_mem_mib` for any size. It lists
   changed titles, headings, canonical addresses, status codes, meta tags,
   added and lost links, and template shifts (many pages whose structure
   fingerprints changed together).
2. The link graph is stored as compressed adjacency lists on disk. It computes
   click depth from the start page, orphan pages (known to the site, for
   example from its sitemap, but with no inbound internal link), broken hubs
   (pages with many broken outgoing links), and an internal link-flow score
   from a damped iterative calculation.
3. The flow score is called "internal link flow" and is labelled as our own
   measure, not a search engine's ranking factor.
4. Both work in streaming passes and report their memory use.

### Offline replay

1. The broker can record raw exchanges of a page (the request, the response
   bytes as received after TLS and before decoding, and the timings) into the
   archive format above, in test and in the user's own audit crawls, including
   the sub-resources the page fetched.
2. The replay proxy serves a recording back byte for byte and opens no outside
   connection. The only difference allowed between a recording and the
   original is the privacy scrub of personal data (spec 0082 privacy gate);
   the scrub is applied once when recording, and a replay is exact to the
   scrubbed record.
3. Recordings are limited to the user's own sites and the lab. Third-party
   sub-resources fetched for a page of those sites are part of the recording,
   are marked third-party and are removed with the crawl.
4. Recordings are listed in the file ledger (spec 0030).

### Erasure and retention

1. A verified erasure request (spec 0032, and the erasure rule of spec 0082,
   SUT-4305) reaches every structure here: vault files (a file is deleted when
   its last reference goes), archive files (the record is removed by rewriting
   the file and signing it again, with a note in the audit log), journals
   (records hold addresses and hashes and are rewritten at the next
   compaction), the diff, link graph, fingerprint and log tables (rows
   deleted), recordings and cold-tier copies.
2. A legal hold set by the owner postpones deletion of the held project's data
   and is recorded; it never overrides a verified erasure of personal data
   without the owner's decision.
3. Crawls are kept for `crawl.keep_crawls` (default 10, oldest removed first).
   Content from third parties is archived only as part of the user's own pages
   and is removed with its crawl.
4. A test erases one subject across all structures and checks that nothing
   remains.

### Live view and steering



1. **Telemetry:** requests per second, status codes, render times, queue depths,
   journal lag and memory are recorded in the metric registry of spec 0112 with
   `sutradhar_leviathan_` names, at near-zero cost, and shown on the Jobs screen
   (spec 0099).
2. **Steering:** while a crawl runs the user may change speed, pause or resume
   a folder, raise a section's priority and add exclusions. Each is an action
   of spec 0020, applied at a safe point (between two fetches of one host),
   audited, and visible in the report. Steering can only stay inside the caps
   and the legal wall of spec 0082: it cannot raise a rate above them, add a
   target outside the registered site, or remove a rule of the wall.
   If a send is unconfirmed after recovery, Jobs shows “Page check
   paused. We could not confirm whether the request was sent.”
   Show the last verified progress within the owner's current scope.
   Keep the original attempt and affected coverage on hold. Use its
   recorded result to reconcile it. The existing Resume control
   continues only work proved safe under current permission, storage
   and pacing. Show separate reasons for corrupt records and full disk.
3. **Discovery map:** a picture of how the crawl moved from the start page
   through sections to deep pages, with depth and status colours, and always a
   list view with the same facts. Very large sites are drawn from a sample and
   say so.
4. **Impact estimator:** before a large audit a small polite sample (default
   50 pages, within the caps) is fetched. The estimate gives ranges for total
   addresses (from the sitemap and the sample), finish time, storage and
   server load, with the sample size, and states that it is an estimate. A
   host whose median answer in the sample is over `crawl.slow_host_ms`
   (default 2,000) lowers the planned rate. A sample under 10 pages gives no
   range (SUT-1608). Proposed acceptance: on the lab, 90 percent of estimates
   contain the true value.
5. **Server-log correlation:** read only the admitted site aggregates
   from darshak (specs 0034 and 0044). Match by site and page id, using
   a site-checked path when no page id is known. This shows which pages
   bots request and which the crawl found. The view opens no log file,
   reads no individual request or hash and keeps no separate copy or
   retention period. On a refused import, lost approval or unavailable
   aggregate, show "not available" (SUT-1609).
6. **Diagnostics over MCP:** four read-only resources over local MCP (spec
   0020): `leviathan/queue`, `leviathan/rules`, `leviathan/timings` and
   `leviathan/exclusions`. They carry plain-English text from the explainers
   of spec 0082, no page content and no secret, and need an agent token with
   the `leviathan.read` scope. A test checks that no resource returns page
   text or a secret.

### Verification: models, lab and standards

1. **Lab.** An offline test web of thousands of simulated sites runs on this
   machine from fixtures, served in process with its own name resolver and the
   reserved `.test` top-level name so no address can be a real site. It
   includes slow and stalling servers, broken certificates, redirect loops,
   script-built pages, tricky robots files and opt-out headers, traps, bombs
   and cloaking. It is deterministic by seed and needs no outside network.
2. **Quint models (bounded model checks).** Models describe the scheduler and
   the compliance rules: a page is never fetched when robots rules or opt-out
   signals forbid it; a host's delay is never shorter than its backoff or rate
   cap; the broker never sends a request other than GET or HEAD; and a
   steering change never lowers a cap. Models are checked within stated bounds
   and time is modelled in bounded steps; they are a check, not a proof of the
   Rust code (as in spec 0114). The simulator runs at commit and push and the
   model checker in the long lane. The link rule between a model property and
   a test is the one of spec 0114 (`QP-nnn` ids, SUT-1403); it is not repeated
   here. Rust-level proofs run on Linux only.
3. **Standards tests.** The browser standards test suite is run against the
   page engine levels that exist (spec 0115): tests are grouped by feature and
   level; each failure is fixed or listed in `gate/known-failures.toml` with a
   reason; the gate fails on a new failure or a crash (the engine process
   exits abnormally or a limit trips), not on a count. The suite is mostly
   under the BSD 3-clause licence with some directories under others, so each
   directory's licence is checked; its size and run time are measured before
   it is admitted. The suite's licence is checked before it enters the
   repository. Until an engine is admitted, this suite does not run and the
   report says so.
4. **Reference browser (proposed).** Comparing the engine's page tree, layout
   and screenshots with a mainstream browser run as a black box is not used
   until the owner decides, because the owner chose that no Spring program is
   ever run (DR-070) and the same caution may apply (QR-049). Until then
   standards tests and fixtures are the oracle.
5. **Grammar-guided fuzzing.** Every parser and decoder of Leviathan has a
   structure-aware fuzz target built from a grammar of its input: HTML, CSS,
   scripts, TLS and QUIC records, PDF, images, fonts, XML, archives and
   compressed bodies. Machines, caps, de-duplication, corpus integrity and the
   zero-open-findings release rule are those of spec 0114. Each finding
   becomes a regression test.
6. **Chaos and soak.** The chaos lab of spec 0114 runs Leviathan scenarios in
   the lab: dropped packets, a connection frozen mid-byte, a storm of 503
   answers, a killed cage process, a full disk, a missing drive. A soak runs a
   simulated crawl of a million lab pages (a site model generated from a seed,
   with a mix of sizes, depths and error rates) for `verify.soak_hours` (spec
   0114) and fails when resident memory grows faster than
   `verify.soak_mem_slope_mib_h`, when open handles grow at all
   (`verify.soak_handles_slope_h`, default 0), or when disk grows faster than
   the crawl's own output (`verify.soak_disk_slope_gib_h`).

### The Linux and Windows matrix (shared)

1. Every check of both crown jewels (unit, slice, contract, integration,
   formal, chaos, conformance, fuzz regression, benchmark) runs on Linux and
   Windows 11. The "result" that must match is the pass or fail status and the
   hash of the normalised output. Timings are not compared between systems;
   each system is compared with its own stored baseline (spec 0114).
2. A difference is a defect and blocks the release, unless the check is on the
   allowed-difference list `gate/allowed-differences.toml` (spec 0003). Every
   entry names its reason, has the owner's approval, carries a hard expiry
   of 30 days or less (matching BJ-29 of spec 0114), and is tracked like a
   waiver (AGENTS.md section 7.2). It also needs an equivalent check on the
   other systems, or else an owner waiver. At the time of writing it holds:
   proofs with Kani, Verus and Creusot (Linux only); memory sanitisers (Linux
   only); the cage mechanism, which differs by
   system but passes the same tests; and servers with no Windows build in the
   compatibility matrix.
3. Windows runs on the GitHub Actions Windows runners (ADR-0021). A check that
   cannot run on a system is reported "not run there"; it is never counted as
   passed.
4. The result of both runs is stored together, so a regression on one system
   shows at once. The farm machines of spec 0114 run fuzzing only and are not
   a crawl coordinator.

### Reproducible builds (shared)

1. Spec 0113 owns the Bazel rules that make builds reproducible; this spec
   owns the check of the result. The verifier builds each release artefact
   twice in clean workspaces on the same pinned runner image and toolchain,
   and once more on a second pinned runner image, and compares the bytes. Any
   difference fails the release. The runner image versions are recorded in the
   report.
2. Sources of difference are pinned: timestamps, build paths, archive order,
   build ids, locale, toolchain and dependency versions (spec 0003,
   `gate/tools.toml` and lock files). The verifier itself is pinned and
   reproducible.
3. Every prebuilt binary that ships or builds the product (for example the
   database, the analytics engine, the model runtimes) is pinned by hash,
   checked for its licence, scanned by the supply-chain checks and listed in
   the bill of materials (spec 0113). The pillar list's "zero opaque prebuilt
   blobs" means exactly this: no unpinned, unverified or unlisted binary.
4. Before release approval, the trusted runner supervisor puts the report
   digest in the required release result's evidence references. Use the
   existing 0003 `evidence-bundle` record and 0065 runner key for that
   purpose. The report binds the exact 0005 candidate `H(C)`, unsigned
   payload inventory `H(I)`, approved packaging profile and each required
   target, plus all three build records and their input and output digests.
   Verify the bundle and report against the protected release request before
   using either. After approval, 0005 owns the distinct detached manifest
   and release record. Leave the Windows wrapper unsigned. Rotate the
   release key yearly and after suspected compromise through 0065's
   owner-signed policy route. Refuse revoked keys under 0065.
5. On Linux this runs from the first release. On Windows it is a release gate
   after the project's own fix for the Rust rules on Windows (ADR-0021,
   RK-023), the same condition as BI-13 of spec 0113.

### Reuse

Our own code is the frontier merge logic, the journal, the vault and its
sweep, the archive recorder's glue, the diff and graph passes, fingerprint
matching, the estimator, the steering rules, the lab, the models and the
matrix and build verifiers. Mature crates sit behind them: compression,
hashing, sorted storage and merge, archive format readers and writers, graph
and text algorithms, the fuzz engine (QR-042), and the standards test suite.
Each is a build-or-reuse record.

#### Recorded exchange verification (N-1660)

Bit-for-bit offline replay proxy uses the existing 0116 archive
and replay route. Use admitted own-site Audit captures or sealed
synthetic fixtures only. Record no session state or raw secret,
and scrub before persistence. Pin the protected project, mode,
request identity, variant, archive version, scrubbed bytes and
eligible recorded timings. Replay exact eligible scrubbed bytes before
decoding, within the owning own-site project, crawl and retention.
Third-party subresources require a recorded eligible dependency.
Check scope and integrity before returning any bytes. Protect
intermediate work and spill under 0118.
A withheld, missing, invalid, expired or erased record returns
the existing safe refusal. It cannot lead to a live fallback.
The internal offline broker cannot resolve misses or create a network
listener. Keep the replay transport offline and apply current vault and
erasure rights before each serve. No external listener, proxy,
credentials, access to another project or promise of indefinite
retention follows from selecting Replay. Trace LS-33/34/39 and
RN-26 with synthetic success, privacy, erasure and stop fixtures.
[N-1660](../planning/feature-wave-5.md#acceptance-design-for-n-1647-to-n-1670)
uses fixture `FX-0116-W5-OFFLINE-REPLAY-EXACT-SCRUBBED-BYTES`.

#### Fingerprints, frontier, scheduling and custody (N-1690 to N-1694)

Use the existing per-project process and resource caps. Bind site,
project, generation, current eligibility and erasure before opening
a plan, claiming work or exposing a result. Validate on sealed
synthetic fixtures and report measured scope. Preserve 0082's
guest-only rules for every admitted acquisition.

| Feature | Exact project and storage contract | Existing requirements |
|---|---|---|
| N-1690 Near-duplicate content fingerprinting | Use eligible main-content fingerprints and the existing confirming comparison. Pin rule and source versions, sample size, scope, confirmation and actual limits. Incomplete or ambiguous evidence yields an observed gap. Only the separate opt-in 0102 derived-data profile can compare eligible sites. Vaults and keys remain isolated. No copying, ownership or causal allegation. | LS-12, LS-13, LS-34, LS-36 · [FX-0116-W5-FINGERPRINT-CONFIRMED-PROJECT-SCOPE](../planning/feature-wave-5.md#acceptance-design-for-n-1671-to-n-1694) |
| N-1691 Disk-backed billion-URL crawl frontier | Use the single-machine frontier, exact seen index, journal and host queue. Preserve exact admission across merge, claim and restore. Separate durable claim and retained result from a network attempt. Apply DR-156 and LS-09/31. A possible handoff with an unknown outcome stays held. Reclaim only ordinary work proved unsent. Recheck current policy and erasure on restart. At the admitted memory or disk bound mark omissions and incomplete work. A full disk pauses under LS-40. Capacity is measured per admitted profile. A billion is a source aim, never a limit proved here. | LS-01, LS-02, LS-09, LS-31, LS-40 · [FX-0116-W5-FRONTIER-RECONCILED-CLAIM-AND-HOLD](../planning/feature-wave-5.md#acceptance-design-for-n-1671-to-n-1694) |
| N-1692 Adaptive freshness scheduler | Use exact eligible registered-site section history, declared revisions and comparable conditional observations. Keep learned intervals within the existing caps and mark inadequate history. An eligible owner override uses existing settings authority and does not raise rate. Recheck current site state at dispatch. Keep feature 1326's due ordering in the one frontier. No scheduled rival visit or changed scope. | LS-02, LS-03, LS-16, LS-22 · [FX-0116-W5-FRESHNESS-BOUNDED-REGISTERED-SITE](../planning/feature-wave-5.md#acceptance-design-for-n-1671-to-n-1694) |
| N-1693 Multi-node distributed crawl coordinator | DR-074 and QR-048 keep coordination outside version 1. Existing local processes use only the one-machine profile. Refuse cross-machine scheduling, restored or imported coordinator settings, remote jobs, ports, identities, tokens and cloud workers. No substitute distributed mode. | LS-04, LS-12, LS-20, AU-LS-01 · [FX-0116-W5-ONE-MACHINE-DISTRIBUTED-REFUSAL](../planning/feature-wave-5.md#acceptance-design-for-n-1671-to-n-1694) |
| N-1694 Content-addressed deduplicated vault | Keep admitted scrubbed bytes and keyed storage names within one project and its own 0118 custody. Bind current per-project key epoch, immutable byte and reference identity. Reconcile the file, reference and journal with durable evidence before exposing verified bytes. Sweep only under the project lock, grace and current retention and erasure, without a live reference, lease or unresolved receipt. A missing key, digest mismatch, incomplete commit or lost scope holds release. Equal bytes never cause cross-project reuse or visible shared evidence. [FX-0116-W5-PROJECT-VAULT-REFERENCE-AND-ERASURE](../planning/feature-wave-5.md#acceptance-design-for-n-1671-to-n-1694). | LS-05, LS-06, LS-12, LS-32, LS-33, LS-34 |

QR-211 qualifies exact measured profiles, interruption and erasure
proof. None of these contracts adds a public-page footprint,
network control plane or a remote evidence path.

#### Archives, operations and verification (N-1695–N-1718)

Use the existing per-project custody, scheduler, journal and
admitted recipient rules. Bind site, actor, project, mode, source,
capture and exact revision before data is read or released.
Keep Audit and offline lab evidence within their own scope.
A public niche, comparison or Receipts observation cannot become
an archive or a wider render input. Enforce current grants,
retention, erasure and key epochs at capture, hand-off, replay and
restore. Withdraw stale derived work under specs 0032 and 0118.
Keep raw private data and credentials out of files, labels, keys,
telemetry, fixture records and public reports.

| Feature | Exact scoped storage, view or verification contract | Existing requirements |
|---|---|---|
| N-1695 Standard WARC archival recorder | Record bounded scrubbed Audit or offline lab exchanges only under the admitted WARC format. Exclude cookies and private or opt-out content before record, digest or spill. Bind request and response records to scope and version and prove the exact opened bytes with an independent reader. Store under project encryption and seal to the approved recipient. A truncated or excluded response stays incomplete. Erasure immediately fences managed export, cold copies and replay, then rewrites the affected archive and signs its new digest. Revoke the old generation. Preserve exact separately scoped retention and legal-hold proof. Already-taken copies are outside managed recall. [FX-0116-W5-WARC-SCRUBBED-RECIPIENT-AND-ERASURE](../planning/feature-wave-5.md#acceptance-design-for-n-1695-to-n-1718). | LS-07, LS-08, LS-33, LS-34, LS-42 |
| N-1696 Instant historical crawl-diff engine | Compare exact eligible same-project captures with compatible scope and coverage via bounded streaming. Use stable canonical version mapping. Separate proved removal from unobserved, erased or partial records. A mapping conflict or unavailable history stays unknown. Timing remains a measured target. No public check or cross-site join follows. [FX-0116-W5-HISTORY-DIFF-COMPARABLE-SCOPE](../planning/feature-wave-5.md#acceptance-design-for-n-1695-to-n-1718). | LS-14, LS-34 |
| N-1697 Full-site link-graph calculator | Use bounded eligible observed arcs and admitted depth and start-page scope. External targets are inert observed endpoints. Distinguish proved orphans and broken hubs from unobserved pages. Label internal link flow as the project's reported measure and keep its coverage. No live change, new outside acquisition or ranking path follows from the graph. [FX-0116-W5-OBSERVED-LINK-GRAPH-COVERAGE](../planning/feature-wave-5.md#acceptance-design-for-n-1695-to-n-1718). | LS-14, LS-15, LS-34 |
| N-1698 Power-loss-proof recovery journal | Use the exact 0116 durable record and 0019 fenced effect identity. Acknowledge only flushed admitted progress. Reconcile torn tail, earlier corruption, current permission, object versions and pending receipts before reclaim. Apply DR-156 and LS-09/31 to the current durable boundary. Keep possible sends with no receipt held and charged. Resume an ordinary claim only with current proof that no handoff was possible. Distinguish attempt from stored result. No duplicate stored output or advance on ambiguous proof. [FX-0116-W5-RECOVERY-UNCERTAIN-UNFLUSHED-HOLD](../planning/feature-wave-5.md#acceptance-design-for-n-1695-to-n-1718). | LS-09, LS-10, LS-31, LS-32, LS-40 |
| N-1699 Automatic hot-and-cold storage tiering | Keep hot summaries, frontier, journal and indexes local. Cold eligible objects may move through the existing 0038 plan only to a current approved destination. Check exact encrypted version and hash on read-back. Keep the verified source until the separately approved cleanup. Missing media, unknown results and erasure hold affected work. [FX-0116-W5-COLD-MOVE-VERIFIED-RETAINED-SOURCE](../planning/feature-wave-5.md#acceptance-design-for-n-1695-to-n-1718). | LS-11, LS-34, LS-37, LS-42 |
| N-1700 Strict multi-project isolation vaults | Bind each worker to its project transaction, vault, key handle, memory slice, queue and archive before use. Scope cache, dedupe, quotas, exports and restore to the same isolation. A copied reference or grant in another project conveys nothing. Exceeding a slice stops only admitted affected work. [FX-0116-W5-PROJECT-VAULT-AND-RESOURCE-ISOLATION](../planning/feature-wave-5.md#acceptance-design-for-n-1695-to-n-1718). | LS-05, LS-12, LS-33, LS-35 |
| N-1709 Live flight-recorder telemetry | Project eligible bounded local samples from the 0112 registry with their units, run and coverage. Label stale, missing and sampled intervals. Watch protected memory and overhead. No raw URLs, IPs, body or private payload. Collection or viewing starts no fetch, off-PC telemetry or extra worker. | 0112 metric registry; LS-12, LS-43 · [FX-0116-W5-SCOPED-FLIGHT-RECORDER-SAMPLES](../planning/feature-wave-5.md#acceptance-design-for-n-1695-to-n-1718) |
| N-1711 Live mid-crawl steering controls | Preview the exact eligible current run, section and new control inside the existing 0020 authority. Apply at its owning safe point and record one receipt. Recheck grants, site, current caps and policy before commit. Pause, rate and exclusion choices cannot admit a new host, weaken any hold or start an authenticated crawl. | LS-02, LS-16, LS-22 · [FX-0116-W5-EXACT-STEERING-SAFE-POINT](../planning/feature-wave-5.md#acceptance-design-for-n-1695-to-n-1718) |
| N-1712 Interactive site-discovery visual map | Present only the eligible current observed graph, discovery root, depth and status. Use the same scope in the keyboard list. Label sampling, gaps and stale facts. Navigation changes the view only. No unseen path discovery, hidden labels or cross-site scope from a map click. | LS-12, LS-14 and 0099 JW-03 · [FX-0116-W5-SCOPED-ACCESSIBLE-DISCOVERY-MAP](../planning/feature-wave-5.md#acceptance-design-for-n-1695-to-n-1718) |
| N-1713 Pre-crawl impact estimator | Use the existing bounded polite sample for a registered site under current owner and guest-only authority. Show measured source, sample size, ranges, planned upper limits and unknowns. LS-41 requires at least ten eligible sample pages. A smaller or held sample gives no range. Preserve the same cap and hold across retry. View or replay starts no sample. A slow host can lower the rate under 0082. Estimation cannot start the full audit, broaden the sample or promise a finish time. | LS-16, LS-17, LS-41 · [FX-0116-W5-QUALIFIED-SAMPLE-RANGE](../planning/feature-wave-5.md#acceptance-design-for-n-1695-to-n-1718) |
| N-1714 Server-log correlation engine | Use only current approved aggregates from 0034 and 0044, matched by approved site and eligible page ID or site-checked path, comparable interval and coverage. Keep an entitled page or path total within its original scope. A revision-specific comparison needs eligible evidence that attributes the aggregate to that exact revision. A bucket spanning revisions gives unknown revision attribution unless the source supplies a proved split. Separate DR-107 referring-path grant still governs any eligible projection. No fallback to raw path, raw requests or wrong grain. Distinguish verified bot evidence from claimed or unknown identity. Apply suppression across charts, exports and overlapping queries. The script or view opens no raw log, follows no address and cannot infer a visitor or retain a separate history. | LS-12, LS-18, LS-34 · [FX-0116-W5-APPROVED-LOG-AGGREGATE-CORRELATION](../planning/feature-wave-5.md#acceptance-design-for-n-1695-to-n-1718) |
| N-1715 Open MCP diagnostic interface | Expose only leviathan/queue, leviathan/rules, leviathan/timings and leviathan/exclusions under the local enrolled 0020/0118 route and current leviathan.read scope. Return bounded typed text data and safe references. No raw target addresses, bodies, secret, global discovery, request execution or underlying debug handle. Each result, restore and replay rechecks revocation and current grant. | LS-19, LS-43 · [FX-0116-W5-FOUR-LOCAL-DIAGNOSTIC-PROJECTIONS](../planning/feature-wave-5.md#acceptance-design-for-n-1695-to-n-1718) |
| N-1717 Full Web Platform Tests conformance suite | Qualify the pinned subset, directory licences and offline dependencies and host maps and exact admitted engine levels and host profiles before use. Use gate/known-failures.toml for admitted exceptions only. Record known failures, applicability, external retrieval refusals, new failures and crashes. No skipped test passes. Test isolated offline fixtures through the existing gate profile. Unsupported or unrun cases cannot pass. No assertion of full browser parity or invocation of an unadmitted reference browser. | LS-20, LS-23, LS-24, LS-27 · [FX-0116-W5-PINNED-OFFLINE-CONFORMANCE](../planning/feature-wave-5.md#acceptance-design-for-n-1695-to-n-1718) |
| N-1718 Continuous 24/7 grammar-guided fuzzing farm | Use 0114's sealed bounded job class with pinned synthetic corpora, grammar and parser versions. Protect synthetic corpora and diagnostic output. Use safe finding references and exact regression evidence under the existing zero-open-findings rule. Scope cases to parser bounds and ordinary synthetic behaviour without vulnerability reproduction. Cover lack of resources, cancellation and missing results. No live target, network input, production corpus or blanket claim of continuous operation or absence of flaws. | LS-20, LS-25 to LS-27 · [FX-0116-W5-SEALED-FUZZ-CORPUS-EXACT-FINDING](../planning/feature-wave-5.md#acceptance-design-for-n-1695-to-n-1718) |

QR-212 qualifies the exact archive, recovery, estimator, supported
platform and gate profiles. Keep existing test catalogue IDs.
DR-156 resolves the repeat-fetch rule across LS-09 and LS-31.
QR-212 still requires proof for the exact recovery profile.

#### Crawler and framework verification evidence (N-1719–N-1726)

Use the existing sealed verification tiers, admitted toolchain and
test catalogue. Pin the candidate tree, run, fixtures, component
profiles, platform and relevant gates. Record the supported outcome,
scope and gaps. A skipped, failed or unsupported run cannot count
as passing. A receipt describes evidence for the exact candidate
and does not sign, release or replace a running component.

| Owner catalogue entry | Exact verification scope |
| --- | --- |
| N-1719 Formal Quint discipline proofs | Check the scheduler and compliance properties within explicit bounds, including deny-before-fetch, back-off and rate caps, GET and HEAD, and steering. Keep the existing model lanes and 0114 model-to-test links. Record model and bound, linked assertions and any counterexample or timeout. A bounded model result is evidence about that model. The Rust verification gates remain separate. [FX-0116-W5-BOUNDED-MODEL-EXACT-TEST-LINK](../planning/feature-wave-5.md#acceptance-design-for-n-1719-to-n-1742). |
| N-1720 Self-contained synthetic mini-internet lab | Use the seeded local 0116 lab of synthetic hosts with fixed clocks, names, data and components. Cover ordinary standards, slow or unavailable responses, markup and opt-out cases without real sites or data. Pin the generator and fixture set and proof of isolation. A name or fixture that cannot resolve in the lab yields a local failure, through the admitted in-process resolver and harness only, without egress, real endpoints or external listener. [FX-0116-W5-SEEDED-OFFLINE-LAB-LOCAL-NAMES](../planning/feature-wave-5.md#acceptance-design-for-n-1719-to-n-1742). |
| N-1721 Automated reference-browser visual oracle | Keep the reference browser unavailable until the owner answers QR-049 and a separate exact profile is admitted. Use pinned standards and synthetic fixtures in the meantime. Any later approved oracle needs isolated identical input, engine version, exact fixed viewport, font digests and comparison tolerances for tree, geometry and paint, and separate evidence of licence and platform compatibility. No real fetch, Spring program execution under DR-070, renderer shipping admission or cutover follows. [FX-0116-W5-REFERENCE-ORACLE-SEPARATE-ADMISSION](../planning/feature-wave-5.md#acceptance-design-for-n-1719-to-n-1742). |
| N-1722 Network chaos and fault-injection simulator | Inject only sealed synthetic transport, timeout, server-error, worker-loss and storage-unavailable events through the existing 0114/0116 lab. Record seeded ordering, exact fault point and resource bounds. Reconcile journal, leases and known outcomes on restart. Model only an owning eligible retry after proved reconciliation. Stop or uncertainty holds even in the simulation. No live retry or fault, and a fixture observation gives no signed release verdict. No production fault or external traffic is generated. [FX-0116-W5-SYNTHETIC-FAULT-CHECKPOINT-RECONCILE](../planning/feature-wave-5.md#acceptance-design-for-n-1719-to-n-1742). |
| N-1724 Multi-day soak and memory-endurance lab | Use the existing seeded million-page synthetic crawl and 0114 soak duration and slope settings. Keep input mix and platform fixed for comparison. Report elapsed coverage, resident memory, handle and disk slopes, interruptions and caps. Fail when the admitted bounds fail, and keep incomplete work distinct. No workload targets real hosts. [FX-0116-W5-SOAK-COMPLETE-COVERAGE-AND-SLOPES](../planning/feature-wave-5.md#acceptance-design-for-n-1719-to-n-1742). |
| N-1725 Identical Linux and Windows verification matrix | Compare pass/fail and normalised result hashes for the exact pinned candidate and profiles. Compare timing separately to each platform's baseline. Preserve the per-check, target and exact candidate allowed differences under LS-38, owner-approved and expiring in 30 days or less (BJ-29 of spec 0114). Keep a Linux-only proof or skipped Windows counterpart distinct from pass. A skipped or unadmitted result remains visible and cannot count as a pass. Evidence cannot enable a Windows cutover. [FX-0116-W5-PARITY-MATCHED-CANDIDATE-AND-EXCEPTIONS](../planning/feature-wave-5.md#acceptance-design-for-n-1719-to-n-1742). |
| N-1726 Reproducible source-to-binary build verifier | Compare each exact unsigned release payload and package manifest across the existing two clean instances of the first pinned runner image and one of the second. Bind source, lock, toolchain, build configuration, runner and artefact digests. Verified prebuilt inputs require hash, licence, scan and SBOM evidence. Apply the existing Windows qualification gate. Bind the exact verifier report digest in the existing supervisor-signed 0003 release `evidence-bundle` before 0005 approval. Use the separate 0005 manifest and release record only after approval. Keep the Windows wrapper unsigned. [FX-0116-W5-REPRODUCIBLE-UNSIGNED-ARTEFACT-IDENTITY](../planning/feature-wave-5.md#acceptance-design-for-n-1719-to-n-1742). |

QR-213 qualifies evidence identity, bounded model and lab coverage,
the dormant reference-browser boundary, platform and build matrices,
and exact failure outcomes. Spec 0082 owns N-1723 legal-wall fixtures.

### Data

- Per project (site column, forced row-level security): `crawls`, `crawl_urls`,
  `crawl_edges`, `crawl_fingerprints`, `crawl_runs`, `freshness`.
- Files in the home folder, recorded in the file ledger: vault folders,
  journals, archive files, frontier files, each under the project's folder.
- Verification data: the lab fixtures, models, known-failures and
  allowed-difference files, in the repository.

### Settings

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `crawl.frontier_mem_mib` | MiB | 128 | 16 to 4,096 | next crawl |
| `crawl.frontier_max_gib` | GiB | 20 | 1 to disk cap | next crawl |
| `crawl.journal_fsync_ms` | milliseconds | 100 | 0 to 1,000 | next crawl |
| `crawl.vault_compress` | zstd or none | zstd | two values | next crawl |
| `crawl.warc_enabled` | on or off | on | two values | next crawl |
| `crawl.warc_max_mib` | MiB | 1,024 | 16 to 8,192 | next crawl |
| `crawl.freshness_min_h` | hours | 1 | 1 to 720 | next crawl |
| `crawl.freshness_max_d` | days | 30 | 1 to 365 | next crawl |
| `crawl.freshness_override` | list of section and hours | none | any | next crawl |
| `crawl.vault_grace_h` | hours | 24 | 1 to 720 | next night |
| `crawl.keep_crawls` | whole number | 10 | 1 to 1,000 | next night |
| `crawl.slow_host_ms` | milliseconds | 2,000 | 100 to 60,000 | next run |
| `crawl.confirm_jaccard` | ratio | 0.8 | 0.5 to 1 | next crawl |
| `crawl.simhash_hamming` | bits | 3 | 0 to 6 | next crawl |
| `crawl.diff_mem_mib` | MiB | 512 | 64 to 8,192 | next run |
| `crawl.cold_tier` | none, external, s3 | none | three values | next run |
| `crawl.estimate_sample` | pages | 50 | 10 to 500 | next run |
| `crawl.steering_enabled` | on or off | on | two values | next crawl |

Pipeline settings for the lab, soak and builds live in `gate/verify.toml` with
those of spec 0114 and take effect at the next run: `verify.lab_sites`
(default 5,000, range 100 to 100,000), `verify.repro_builds` (default 3, range
2 to 10), `verify.soak_mem_slope_mib_h` (default 5, range 0 to 1,000),
`verify.soak_handles_slope_h` (default 0, range 0 to 100) and
`verify.soak_disk_slope_gib_h` (default 1, range 0 to 1,000). There is no
setting that raises a request rate above the caps of spec 0082, lets projects
share storage or keys, or skips the journal.

### Failure codes

SUT-1600 frontier full, audit incomplete; SUT-1601 journal tail dropped
(information); SUT-1602 committed crawl record or flush boundary unproved;
SUT-1603 vault file missing or fingerprint mismatch;
SUT-1604 archive record write failed;
SUT-1605 cold tier unreachable; SUT-1606 cross-project access refused;
SUT-1607 steering change refused by the wall; SUT-1608 estimate sample too
small; SUT-1609 log import refused; SUT-1610 reserved (the model-to-test rule
is SUT-1403 of spec 0114); SUT-1611 standards test newly failing; SUT-1612
matrix difference not on the allowed list; SUT-1613 build not reproducible;
SUT-1614 lab site reachable from outside; SUT-1615 disk full, crawl paused.
Codes SUT-1600 to SUT-1699 are reserved for this spec.

### Security and privacy

- Vaults and keys are per project. Archive files and snapshots are signed.
- The lab is offline and contains no real site, person or credential.
- Server logs and crawl data are scrubbed of personal data and never leave the
  PC without the owner's switch.
- Diagnostics over MCP are read-only and hold no page content or secret.

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| LS-01 | The frontier MUST keep memory within its window whatever its size, use an exact seen-test and report when it drops addresses (SUT-1600). | UT-LS-01, MS-LS-01 |
| LS-02 | Per-host queues MUST apply politeness and priority order. | UT-LS-02 |
| LS-03 | The freshness scheduler MUST work only on registered sites, never exceed the caps of spec 0082 and mark "not enough history". | UT-LS-03, PL-LS-01 |
| LS-04 | Version 1 MUST NOT open any network port for crawl coordination (ADR-0021), and the multi-machine feature MUST stay deferred. | UT-LS-23, AU-LS-01 |
| LS-05 | The vault MUST store a file once per project, never deduplicate across projects, and verify a fingerprint on read (SUT-1603). | UT-LS-04, PL-LS-02 |
| LS-06 | The vault sweep MUST re-check the reference count before it deletes. | UT-LS-05, FI-LS-01 |
| LS-07 | Archive files MUST be standard WARC readable by an independent reader, with record digests. | IT-LS-01 |
| LS-08 | Snapshots and reports MUST be signed and verification MUST detect tampering. | UT-LS-06 |
| LS-09 | The journal MUST carry a checksum per record and drop only a proved torn tail after the verified durable prefix (SUT-1601). Preserve every verified flushed step. The flush window bounds unfinished local work and grants no retry. Reconcile an exact committed effect once, reclaim only with durable proof of no dispatch and hold an unknown send. | FI-LS-02, IT-LS-02 |
| LS-10 | Damage within the committed boundary or lack of trusted boundary MUST hold the affected crawl with SUT-1602 and keep the journal intact. | UT-LS-07 |
| LS-11 | Moved files MUST be verified and a missing drive MUST pause dependent jobs. | IT-LS-03 |
| LS-12 | A project MUST NOT read another project's key, files or queue (SUT-1606). | UT-LS-08, PL-LS-03 |
| LS-13 | Near-duplicate matching MUST use bounded memory and a confirming comparison, and MUST report similarity and sample size. | UT-LS-09, MS-LS-02 |
| LS-14 | The diff and link graph MUST run in streaming passes within their memory settings. | UT-LS-10, MS-LS-03 |
| LS-15 | The flow score MUST be labelled our own measure. | UT-LS-11 |
| LS-16 | Steering MUST stay inside the caps and legal wall, be audited, and refuse anything else (SUT-1607). | UT-LS-12, PL-LS-04 |
| LS-17 | The impact estimator MUST give ranges with the sample size and lower the rate for a slow host. | UT-LS-13 |
| LS-18 | Log correlation MUST read only admitted aggregates for this site and refuse missing approval or evidence (SUT-1609). | UT-LS-14 |
| LS-19 | Diagnostics over MCP MUST be read-only and hold no content or secret. | UT-LS-15 |
| LS-20 | The lab MUST be deterministic, offline and unable to reach a real site (SUT-1614). | UT-LS-16, PL-LS-05 |
| LS-21 | The model-to-test rule of spec 0114 (SUT-1403) MUST apply to the Leviathan models. | UT-LS-17 |
| LS-22 | The models MUST cover robots, backoff, rate caps, GET and HEAD only, and caps never lowered by steering. | UT-LS-18 |
| LS-23 | The standards suite MUST fail the gate on a new failure or a crash and MUST list known failures with reasons (SUT-1611). | UT-LS-19 |
| LS-24 | The reference browser MUST NOT be used until the owner decides. | AU-LS-02 |
| LS-25 | Every parser and decoder MUST have a grammar-guided fuzz target under the farm rules of spec 0114. | UT-LS-20 |
| LS-26 | The soak MUST fail on memory, handle or disk growth beyond the stated slopes. | IT-LS-04 |
| LS-27 | Every check of both crown jewels MUST run on both systems with the same result unless on the allowed-difference list (SUT-1612), and a check not run MUST NOT count as passed. | UT-LS-21, IT-LS-05 |
| LS-28 | Each release artefact MUST be built twice in clean workspaces on the same pinned runner image, and once on a second pinned image, and the bytes MUST match (SUT-1613); on Windows this applies after the rules_rust fix required by ADR-0021. Bind the verified report digest in the release `evidence-bundle` before approval. | IT-LS-06 |
| LS-29 | Every prebuilt binary MUST be pinned by hash, licence-checked, scanned and listed in the bill of materials. | UT-LS-22 |
| LS-30 | The matrix and the build verifier MUST include Behemoth Boot (spec 0114, BJ-21). | AU-LS-03 |
| LS-31 | Before handing out an address, the frontier MUST retain it in the memory table or a sorted run. Before transport, flush the exact claim and dispatch fence. Merge, crash and restore MUST preserve the same identity and claim disposition without an unqualified repeat fetch. | IT-LS-07, FI-LS-03 |
| LS-32 | The vault MUST write the file, then the reference, then the journal, and the sweep MUST delete only files unreferenced for the grace period, under the project lock, tested against a concurrent crawl. | FI-LS-01, IT-LS-08 |
| LS-33 | Vault and archive files MUST be encrypted at rest with the project key, stored names MUST be keyed hashes, and an export action MUST seal a standard WARC to the intended recipient. | UT-LS-24, IT-LS-01 |
| LS-34 | A verified erasure MUST reach the vault, archive, journal, diff, link graph, fingerprint, log and recording data and cold-tier copies, and a legal hold MUST be recorded. | IT-LS-09 |
| LS-35 | A project crawl MUST run in its own process or cage with a hard memory limit, and one that exceeds its slice MUST be stopped alone. | IT-LS-10 |
| LS-36 | Near-duplicate matching MUST use the part-table scheme for the Hamming limit and the confirming comparison, within `crawl.simhash_hamming` 0 to 6. | UT-LS-09, MS-LS-02 |
| LS-37 | A cold-tier move MUST read back and compare the hash, delete the local copy only after it, take credentials from tijori, and refuse a destination that is switched off. | IT-LS-03, UT-LS-25 |
| LS-38 | The matrix result MUST be the pass or fail status and the normalised output hash, timings MUST NOT be compared across systems, and every allowed difference MUST be owner-approved and expire in 30 days or less (BJ-29 of spec 0114). | UT-LS-21 |
| LS-39 | The offline replay MUST serve recorded bytes exactly, differ from the original only by the privacy scrub, and open no outside connection. | IT-LS-11 |
| LS-40 | A full disk MUST pause the crawl, keep the journal flushed, raise an alert and delete nothing (SUT-1615). | FI-LS-04 |
| LS-41 | An estimate from fewer than 10 sample pages MUST give no range (SUT-1608), and a slow host MUST lower the planned rate. | UT-LS-13 |
| LS-42 | A failed cold-tier move MUST pause the move job and let the crawl continue (SUT-1605), and a failed archive write MUST be retried then stop the crawl with SUT-1604. | FI-LS-05 |
| LS-43 | Diagnostics resources MUST be the four named resources with the `leviathan.read` scope and MUST return no page text or secret. | UT-LS-15 |
| LS-44 | The settings of the table, including `crawl.warc_enabled`, `crawl.steering_enabled`, `crawl.vault_compress`, `crawl.vault_grace_h` and `crawl.keep_crawls`, MUST be enforced as stated. | UT-LS-26 |

## How it is tested

Designed, not run.

- **Unit (UT-LS):** frontier merge and bounds, UT-LS-21 (a matrix result of
  status and hash only; an allowed difference with no owner approval, or
  with an expiry over 30 days, refused; one that expires in 30 days or
  less accepted), journal records and tails,
  vault fingerprints and sweeps, fingerprint matching with a table of near and
  far pages, diff and graph on small fixtures, steering rules against the
  wall, the estimator's ranges; with one value just inside and one just
  outside each limit.
- **UT-LS-14:** match admitted counts on two sites with the same path.
  Expose only the active site's aggregates. Run the N-1714 scope case with
  a matching page and a bucket spanning two revisions. Attribute the
  aggregate to a revision only with exact eligible source proof. Stop
  approval mid-read and fail the aggregate lookup. Show unavailable,
  with no raw read or cached result in either case.
- **Planted faults (PL-LS):** a freshness plan for a rival; a cross-project
  read; a steering change that raises a cap; a lab site that reaches outside;
  each must be refused with its named message while an honest case passes.
- **Integration (IT-LS):** a lab crawl with a power cut; a WARC file read by a
  separate reader; a cold-tier move and fetch; a two-runner reproducibility
  check; the matrix on both systems.
- **IT-LS-01:** put distinct cookie values on a main response, redirect
  and subresource. Confirm an independent WARC reader and replay see
  none of them, with record digests still valid.
- **IT-LS-06:** use three clean synthetic build records for each required
  target and a test-signed release `evidence-bundle` naming the report
  digest. Verify exact unsigned bytes, inputs, required targets and the
  protected request. Independently alter the report, substitute proof for
  another inventory or target, and offer a test-signed `release-manifest`
  in the bundle's place. Each fault blocks the build-verifier evidence.
  Join the release refusal to 0005 IT-0005-RELEASE-APPROVAL. A Windows
  wrapper remains unsigned, and missing required Windows evidence stays
  held under LS-27 and LS-28.
- **Fault injection (FI-LS):** kills at random journal points, a torn tail, a
  sweep interrupted, a drive that disappears.
- **FI-LS-02 and IT-LS-02:** Cut power after durable no-handoff
  proof with the former sender fenced, immediately before and after
  handoff with an identical durable possible-handoff record, and
  after the exact result is committed.
  In the pre-handoff uncertain case the server sees zero sends.
  The adapter still lacks proof to reclaim it. Add a
  torn uncommitted tail to each. Resume only the ordinary proved
  unsent claim once. Both identical claim records without a current
  outcome hold with the original charge, wait and gap. Recover the
  committed receipt without another send. Plant a valid-looking
  suffix beyond the checkpoint and offer an older restored prefix.
  Recheck the independent checkpoint and refuse dependent sends.
  Independently corrupt a committed final or earlier record, and
  separately lose the trusted flush boundary. Each holds with SUT-1602 and no
  truncation. On Linux and Windows verify the existing flush
  primitive and the scoped owner status.
- **IT-LS-07 and FI-LS-03:** Merge the frontier during recovery, then
  restore a saved copy. Recheck the original claim and current fence.
  A missing dispatch result, a stale backup or a revoked grant
  cannot start a fetch. A proved unsent ordinary visit cannot
  reopen a spent 0082 rival claim, consumed 0120 approval or 0131
  slot. Keep each exact durable outcome once. Count sends separately
  from recovered local steps and retained pages.
- **Audit (AU-LS):** that no multi-machine code exists, that the reference
  browser is unused, and a sample of features traced from the pillar list to
  the ledger item to the test.
- **Measurement (MS-LS-01 to 04):** frontier capacity per GiB, fingerprint
  memory and recall over a million pages, diff and graph memory over a million
  pages, and crawl speed on the 8 GiB reference. Provisional targets, to be
  confirmed by measurement before release: frontier at least 20 million
  addresses per GiB, fingerprint tables under 512 MiB per million pages, diff
  and graph under `crawl.diff_mem_mib`, and the estimator accuracy above.

## Changes from the source

The pillar lists asked for a billion-URL frontier, a distributed coordinator,
and billions of fuzz inputs. Capacity is a measured, capped number; multi-
machine coordination is deferred because ADR-0021 forbids machines talking
over a network (QR-048); fuzz budgets follow spec 0114. The reference-browser
oracle waits for the owner (QR-049). "Zero opaque prebuilt blobs" is restated
as "every prebuilt binary is pinned, licensed, scanned and listed". The matrix
and the reproducible-build verifier now have their owner here, which unblocks
requirement BJ-21 of spec 0114.
