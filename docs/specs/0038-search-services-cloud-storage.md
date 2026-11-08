# 0038: Search services, cloud storage and provisioning

Sutradhar keeps a word index of a site's guest-visible posts, so it can
find threads by their words as well as by meaning. By default the index
is built in, with nothing extra to run. A site that already has
Elasticsearch or OpenSearch, on its own server or hosted by Amazon, can
use that instead. Separately, raw analytics and backups can be kept in
the owner's own Amazon S3 or Google Cloud Storage bucket. Sutradhar can
create that bucket after showing the provider, region and expected
monthly charge, and can move everything from one place to another,
checking every batch. All of this is optional, and cost warnings never
silently stop authorised work. It is roadmap Phase 5 item 6.

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
- Search service: Elasticsearch or OpenSearch, separate programs that
  hold a keyword index.
- Built-in index: tantivy, a Rust keyword-search library, inside the
  service.
- Bucket: a named storage area at a cloud provider.
- Provisioning: creating the bucket or search service for the owner.
- Region: the provider's location where the data is kept.

### Scope

In scope: the `SearchIndex` trait with built-in, Elasticsearch and
OpenSearch versions; keeping the index in step with guest-visible
content; rebuilding without a gap; the `ObjectStore` trait with local,
S3 and Google Cloud Storage versions; the bounded local queue; verified
upload and the move between stores; costs and budget warnings; creating
buckets and hosted search after approval; the Storage and Search screens
and `sutra` commands.

Out of scope: the vector index, which stays in the service's own folder
(spec 0022); analytics collection (spec 0034); the content and schedule
of backups (spec 0030), which use the `ObjectStore` here; paid
embedding providers (spec 0025).

### Design

`sutradhar-bhandar` (the storage crate) gains these modules, each
behind a trait with a fake in `sutradhar-pariksha`:

- `SearchIndex`: `upsert`, `delete`, `query`, `begin_generation`,
  `switch_to` and `health`.
- `ObjectStore`: `put` (returns a receipt), `head`, `get`, `list` and
  `delete`.
- `Provisioner`: `quote` and `create` (from an approved quote only).

Modules:

- `bhandar::search`: `TantivyIndex` (default) and one `RestSearch`
  client with an Elasticsearch and an OpenSearch dialect, since they
  share most of their REST interface. Hosted Amazon OpenSearch Service
  uses the same client with AWS request signing.
- `bhandar::objects`: `LocalStore`, `S3Store` and `GcsStore`. Google
  Cloud Storage's XML interface accepts the same style of signed request
  with HMAC keys, so one signer (`hmac` and `sha2`) serves both.
- `bhandar::provision`: `AwsProvisioner` and `GcpProvisioner`, each
  only creating a private bucket (and, for AWS, an OpenSearch domain)
  after an approved quote.
- `bhandar::archive`: the bounded local queue, verified upload, removal
  of confirmed local copies and the guided move.
- One active search backend at a time. Plain REST over `reqwest`, with
  `backon` retries and `governor` limits; no extra client libraries.
  The dialect, signer and price table are separate parts.

### Keeping search in step

1. What is indexed: guest-visible thread titles, post chunks (the same
   1,024-token chunks with character offsets), tags and forum names.
   Nothing else. Staff-only and private content never.
2. Changes arrive through the outbox: a karya job reads new, edited,
   hidden and deleted posts and applies them in batches.
3. A post that stops being guest-visible leaves the active index within
   60 seconds of Sutradhar seeing the change. A thread moved from a
   public to a private forum leaves in one batch.
4. Rebuild: a new generation fills in the background, with progress
   (spec 0016), while the old one answers. The switch is one step (an
   alias for Elasticsearch and OpenSearch, a pointer file for tantivy).
   An edit during a rebuild is applied to both generations.
5. Coverage: the Search screen shows how many guest-visible posts are
   indexed, which are missing and why.
6. Keyword hits are one source of candidates for matching; spec 0023
   decides how they are weighed.
7. When a connected search service is down, suggestions continue from
   meaning alone, marked "made without keyword search".
8. XenForo's own Enhanced Search index on the same cluster is never read
   or written; Sutradhar's indexes use their own prefix.

### Cloud storage

1. Uses: raw analytics archives and, if the administrator chooses,
   backups, in their own prefix and lifecycle. Backups are encrypted on
   the owner's PC first, with a key held by the owner.
2. Every object is compressed with zstd and named by its content's
   BLAKE3 hash plus day and kind. Nothing in object names identifies a
   member or the site.
3. Verified upload: the provider's own checksum must match ours before a
   local copy is deleted. A mismatch keeps the local copy and retries.
4. Bounded local queue: during an outage, files wait in a local queue
   with a size limit (default 2 GiB). When full, that collection stops,
   the gap is reported with start and end times, and the site carries
   on unaffected.
5. Retention: detailed analytics 90 days, anonymous aggregates two
   years. Buckets get matching lifecycle rules at creation, checked
   daily.
6. Erasure: detailed analytics objects are split by day and by one of
   256 groups of visitor identifiers, so erasing one visitor rewrites at
   most 90 small objects.
7. The guided move: copy in batches of at most 1,000 objects or 1 GiB,
   verify each, record a checkpoint, switch the active store in one
   step, then remove old copies after the administrator confirms. It
   resumes after a restart and reports progress. A region the provider
   retires gets a move offered.
8. Two installs pointed at one bucket are refused by a marker object
   holding the install's id.
9. A bucket deleted by hand: uploads stop, the queue fills, and the
   screen says what happened.

### What ganit decides here

From ids and numbers only; each answer is shadow-tested and the last
good one kept if a job is late:

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

### Creating resources

1. The administrator chooses the provider, region and kind (bucket, or
   hosted OpenSearch on AWS; no hosted Google Cloud search in version
   1).
2. Sutradhar shows a quote: the exact resources, region, settings
   (private, no public access, the provider's encryption at rest,
   lifecycle rules) and the expected monthly charge with its basis.
3. The administrator pastes a limited key, with the exact policy shown
   (one bucket, no delete of other buckets), and approves the quote.
   Approval is audited. No sign-in library is added.
4. Sutradhar creates exactly what was quoted, tags it "sutradhar",
   checks it and stores the connection.
5. Connecting an existing resource skips steps 2 to 4 and runs the same
   checks: reachable, private, writable, checksums returned.
6. No cloud resource is ever created without an approved quote.

### Data

PostgreSQL, every table with row-level security on and forced:

- `stores`: id, kind, provider, region, bucket, active flag, created or
  connected, lifecycle state, last check.
- `store_objects`: key, store, size, BLAKE3, provider checksum, verified
  at, local copy removed at.
- `store_moves`: id, from, to, checkpoint, counts, state.
- `search_backends`: kind, endpoint, active generation, coverage.
- `cloud_budgets`: monthly budget, prices with their source and date.
- Credentials are held by tijori in files only the service account can
  read, never in the database, logs or exports (`secrecy`).

### API and events

- Actions, through the action layer: `bhandar.search.choose`,
  `.search.rebuild`, `.search.coverage`, `.store.connect`,
  `.store.quote`, `.store.create`, `.store.move`, `.store.verify`,
  `.budget.set`.
- Events: `search.generation_switched`, `search.backend_down`,
  `store.upload_verified`, `store.queue_full`, `store.move_progress`,
  `budget.warning`.

### Failures (SUT-820 to SUT-829)

- SUT-820 search service unreachable.
- SUT-821 search index behind by more than 10 minutes.
- SUT-822 bucket unreachable or credentials refused.
- SUT-823 checksum mismatch after upload; local copy kept.
- SUT-824 local queue full; collection stopped, gap recorded.
- SUT-825 provider refused creation; nothing was created, or exactly
  what was created is listed.
- SUT-826 lifecycle rule missing or changed at the provider.
- SUT-827 budget passed; work continues.
- SUT-828 move paused after repeated failures; it resumes.
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
  move uses the accessible progress bar with text ("41,200 of 120,000
  files, about 12 minutes left"); budget warnings use the SDS warning
  component with icon and words.
- Admin search box and `sutra find --content`: CAP-006. Search size
  limit and warning: CAP-030. Storage screen and `sutra store
  connect|create|verify`: CAP-032. Move and `sutra store move`: CAP-033.
  Budget and `sutra budget set`: CAP-034. Search screen and `sutra
  search choose|coverage`: NEW-033-A. `sutra search rebuild`:
  NEW-033-B, "Rebuild the search index without stopping suggestions".

### Observability

`sutradhar_search_query_seconds{backend}`, `_search_lag_seconds`,
`_search_coverage_ratio`,
`sutradhar_store_upload_bytes_total{provider,outcome}`,
`_store_queue_bytes`, `_store_move_objects_total`,
`sutradhar_cloud_cost_estimate{provider}` (a number, no names).
Last-success gauges for the indexer, uploader and daily lifecycle check.
Alerts: search behind 10 minutes; queue over 80%; checksum mismatch;
budget passed (a warning, not a stop).

### Facts checked before building

Google Cloud Storage's acceptance of AWS-style version 4 signatures
with HMAC keys; the Elasticsearch and OpenSearch versions to support and
where their REST calls differ; S3 and Google Cloud checksum headers and
lifecycle-rule formats; how the slice counts memory-mapped file pages;
prices per region, recorded with their page and date.

## Limits

- Inside the service slice, within the memory limits in AGENTS.md section 5.6:
  tantivy writer 64 MiB heap; readers 64 MiB of caches; REST client and
  uploads 32 MiB of buffers. Uploads stream and never hold a whole file
  in memory. These are estimates, measured on Dell.
- The index files are memory-mapped. The slice counts the page cache of
  mapped files, though the kernel can reclaim it; the measured peak must
  include it.
- Elasticsearch and OpenSearch never run inside Sutradhar's slice. A
  connected one is the owner's own server or a hosted service.
- Local queue 2 GiB by default. Move batches at most 1,000 objects or
  1 GiB.
- Speed targets on Dell with the 50,000-thread tier: built-in keyword
  query p95 under 50 ms; connected service p95 under 20 ms of
  Sutradhar's own work; a new post searchable p95 under 5 s; a hidden or
  deleted post gone under 60 s; admin search p95 under 300 ms end to
  end; verified upload at least 20 MB/s; a built-in rebuild at least
  2,000 posts a second, yielding to live work.

## Footprint budget

This part adds nothing to public pages. An outside failure never blocks
a site page.

## How it is tested

Rust tests with fakes unless named:

- SC-01: with public, staff-only and private posts, only guest-visible
  posts are searchable on each backend.
- SC-02 (test site): a post hidden in XenForo leaves the active index
  within 60 s.
- SC-03: the fixture corpus gives the expected threads in the top ten on
  tantivy and on Elasticsearch and OpenSearch test containers on Dell.
- SC-04 (and chaos): a rebuild with queries and edits running answers
  every query and loses no edit; killing it mid-way is safe.
- SC-05 (property): with a store corrupting one upload in ten, no local
  copy is deleted unless its checksum matched.
- SC-06: an outage longer than the queue stops collection at the
  limit, shows the gap with its times and leaves pages unaffected.
- SC-07 (chaos and property): a move of 10,000 objects killed at random
  points finishes with every object verified once and the switch made
  once.
- SC-08: an unapproved quote sends no create call to the provider fake.
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
- SC-16: Kani proofs hold and cargo-mutants leaves no surviving mutant
  for the guest-visibility filter, "delete local only after a verified
  checksum" and "create only what was approved", for example "no path
  deletes a local copy whose receipt does not match".
- SC-17: with job 201 overdue, the Storage screen shows the last good
  forecast with its date and an AutoIssue is filed.
- Coverage is 100% of lines and branches; speed targets are held by the
  5% median rule of AGENTS.md 6.8.

## Changes from the source

- Credentials are held by tijori in service-only files, not container
  secrets.
- The built-in index runs in the service in the service slice, not a
  worker container; the old 5.6 GB figure gives way to the limits in
  AGENTS.md section 5.6.
- Post text or archives go to an outside search service or bucket only
  when the owner has turned that cloud route on (AGENTS.md 5.5).
- Month-to-date cost books move to hisaab (spec 0035).
- Screens are the Dioxus app built from SDS components.
- Speed: the 5% median rule replaces the old ratchet.

Source: `docs/specifications/operations/search-storage.md` and
`docs/specifications/operations/search-storage-2.md` in the archive
`archive-2026-11-08-old-repo`.
