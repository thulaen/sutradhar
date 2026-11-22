# 0108: Behemoth Boot data access

This is group PB-D of Behemoth Boot (spec 0081). It owns how a Behemoth Boot
program stores and reads data: repositories, the entity mapping layer, SQL
drivers and connection pools, transactions, schema migrations, Redis and
Valkey, other database families, search-engine adapters and caching. Spec
0081 owns the ledger and clean-room rules, spec 0106 the core (wiring,
settings, lifecycle) and spec 0107 the web layer. Sutradhar's own data rules
(PostgreSQL 18 with row-level security, AGENTS.md section 5.2) bind every
starter here.

Sutradhar uses PostgreSQL 18 as its system of record, embedded Tantivy for
search, embedded DuckDB for analytics and an embedded Rust vector index
(ADR-0021). The other drivers and adapters in this spec are framework parity:
they are built, tested and kept in step with Spring, but the Sutradhar
product profile does not enable them. Nothing in Sutradhar talks to Kafka,
OpenSearch or Elasticsearch (ADR-0021).

Spring facts here come from the owner's pillar list (19 November 2026) and
are "unverified" until the scout confirms each against official notes (spec
0081). The behaviour description, not the Spring wording, is the contract.
The pillar list asked for native wire-protocol drivers for every database;
that is replaced by mature crates behind our starters (ADR-0022, AGENTS.md
section 6 rule 14).

Example: an orders repository declares `find_by_status_and_created_after`. At
build time the method name becomes a typed query, checked against the schema;
a typo such as `find_by_statu_and_created_after` stops the build with "no
field `statu` on Order; did you mean `status`?". At run time the call needs a
site. The transaction first binds that site through the setter below and
returns only that site's rows. A call with no site returns no rows and logs
SUT-972. A save edited elsewhere in the meantime fails with an "edited by
someone else" error and preserves the newer data.

Values marked "proposed" are design choices to be checked by measurement.

## What it does

### Words used here

- **Entity**: a Rust type that maps to one table or document.
- **Repository**: a typed interface that finds and saves entities.
- **Derived query**: a query worked out from a method name.
- **Dirty tracking**: noticing which fields of a loaded entity changed.
- **Optimistic locking**: a version number that rejects a save made from a
  stale copy.
- **Site context**: the site that every transaction is set to (AGENTS.md 5.2).
- **Pool**: a reusable set of open database connections.
- **Migration**: a numbered change to a schema, applied once.
- **Cache tier**: one level of caching (local memory, then a shared store).
- **Row-level security (RLS)**: the database rule that shows each role only the
  rows its policy allows (AGENTS.md section 5.2).
- **Fetch plan**: an explicit list of relationships to load with an entity.
- **Projection**: a smaller type that holds only some fields of an entity.
- **Savepoint**: a marker inside a transaction that can be rolled back alone.
- **Isolation level**: how far concurrent transactions can see each other.
- **TTL**: time to live, how long a cached value stays valid.
- **Stampede**: many callers loading one missing value at the same moment.
- **Dry run**: a run that shows what would change and changes nothing.
- **Valkey, sentinel, wide-column**: a Redis-compatible store, Redis's
  failover helper, and a database family of wide rows (Cassandra).
- **Migrator role / app role**: the schema owner that runs migrations, and the
  limited role the running app uses (spec 0030).

### What this spec designs

| Feature | What the user or program sees |
|---|---|
| N-1841 Declarative repository macro engine | A trait method name becomes a typed query at build time |
| N-1754 Clean-room ORM and change-tracking engine | Entity mapping, relationships, dirty tracking, optimistic locking, cascades, auditing and query builders |
| N-1755 Clean-room SQL drivers and connection pools | Drivers and pools for PostgreSQL, MySQL, MariaDB, SQLite, DuckDB, Oracle and SQL Server |
| N-1756 Lazy datasource connection startup | An option to open real connections on first use, so the app starts before the database is ready |
| N-1757 Declarative transaction manager | Transaction rules on functions: propagation, isolation, read-only, savepoints and rollback rules |
| N-1758 Built-in database schema migration engine | Checksummed migrations, rollback plans and safe start-up locking |
| N-1759 Redis and Valkey engine with listener macros | Cluster, sentinel and primary-replica routing, and listener macros for publish and subscribe |
| N-1760 NoSQL, graph and wide-column repository engines | Repositories for MongoDB, Neo4j, Cassandra and Couchbase |
| N-1761 Search-engine repository adapters | Typed clients for Elasticsearch and OpenSearch with templates and highlight mapping |
| N-1762 Multi-tier caching abstraction | Cache, update and evict attributes over a local cache and a shared store |

### Repositories and derived queries

1. A repository is a trait. A derive macro implements it. The method name is
   parsed at build time into a query: `find_by`, `read_by`, `get_by`,
   `count_by`, `exists_by` and `delete_by`, with `And`, `Or`, comparison words
   (`Between`, `LessThan`, `After`, `Before`, `In`, `Like`, `IsNull`), `OrderBy`
   with direction, and `First` or `Top` with a number. The exact keyword list
   is taken from the official reference by the scout (unverified).
2. Results may be a single entity, an optional entity, a list, a page (with
   total) or a slice (without total), and a projection (a smaller type holding
   only some fields).
3. Mechanism: a checked-in schema snapshot (a generated file listing tables,
   columns and types, produced from the migrations and marked generated,
   AGENTS.md 8.5) is the macro's input. The derive macro reads it, turns the
   method name into SQL text, and fails the build with the nearest names when
   a field, type or column does not exist (SUT-974, raised at build time
   only). The generated SQL is then checked a second time with sqlx's offline
   query metadata, which a gate step refreshes and verifies is not stale;
   schema drift fails the build. No query is built from a method name at run
   time and there is no reflection (AGENTS.md 5.1).
4. A query that cannot be derived is written as explicit SQL in an attribute
   and is checked the same way.
5. Values are always bound as parameters, never joined into the text.
6. A page request has a maximum size (`data.max_page_size`) so a client cannot
   ask for everything. An unpaged read has a row cap (`data.max_rows`), and
   `delete_by` is capped at `data.max_delete_rows`. Sort and filter fields from
   a client come only from an allow-list of derived column names, never from
   text, so no identifier is ever joined into a query.

### Entity mapping layer

1. Entities carry derive attributes for table, columns, keys and
   relationships: one to one, one to many, many to one and many to many.
2. There is no hidden loading. A relationship is loaded by an explicit fetch
   plan ("load orders with their lines"), which removes the surprise
   one-query-per-item pattern (the "N+1" problem). Reading an unloaded
   relationship is a compile-time error, not a lazy proxy.
3. Dirty tracking compares the loaded snapshot with the current value and
   writes only the changed columns.
4. Optimistic locking uses a version column. A save with a stale version fails
   with SUT-973 and changes nothing.
5. Cascades are explicit per relationship: save, delete or none. A delete
   cascade that would remove more than `data.max_cascade_rows` rows (proposed
   1,000) needs an explicit larger limit in code.
6. Auditing fills created and modified time and the acting user from the
   request context. The acting user is personal data and follows the retention
   rules of spec 0032. The audit record never holds secret values. (Audit of
   access decisions belongs to group PB-E.)
7. A typed query builder builds queries for what a method name cannot say.
8. An entity records its owning scope. Site entities carry the exact
   site ID. Only the installation-owned metadata below can use the
   installation scope.

### Site context and row-level security (Sutradhar rule)

1. Ordinary service connections use the restricted app role. The
   migrator owns the schema and runs only the admitted install and
   upgrade migrations. It is unavailable to ordinary service work.
   Keep their existing Tijori-backed credentials separate. Spec 0030
   owns the restricted setup role. Spec 0032 confines the erasure
   role to deletion on `audit_details` and `subjects`. Spec 0040 confines
   connector secret-table grants to `sutradhar_connector`. Each of those
   existing grants stays with its owning operation. Ordinary app
   connections do not inherit the narrow grants or the migrator role.
   Every runtime role is a non-owner, is not a superuser and has no
   `BYPASSRLS`. The setup role meets those same restrictions.

2. A protected transaction binds exactly one typed scope. Retain
   DR-148's site statement for `SiteId`. The separate `InstallationId` is
   the canonical value in the current verified native register
   format under 0030. Equal text in the two types gives no shared
   authority. Refuse a format with no admitted encoding. The first statement
   after opening the transaction binds the authorised value into the
   applicable fixed statement.

   | Scope | Exact bound statement |
   | --- | --- |
   | Site | `SELECT set_config('app.site', $1, true)` |
   | Installation | `SELECT set_config('app.installation', $1, true)` |

   The binder selects the namespace from the action and authenticated
   role. No caller input chooses it. Validate the identity and check
   that the other setting is absent before protected SQL or a
   savepoint. The transaction keeps that one scope until commit or
   rollback. A missing, malformed or changed identity, two contexts,
   or a binding failure blocks the transaction before application
   work under SUT-972. Treat an empty setting as absent. Refuse any
   protected statement outside its scoped transaction.

3. Each protected table keeps row-level security enabled and forced.
   Site policies require a canonical match to the exact site and no
   installation context. Installation policies require a canonical
   match to the exact installation and no site context. A row carries
   only one owning scope. Where a 0030 table has both kinds of row,
   its constraint accepts exactly one of the two typed identities.
   Apply the same scope and role conditions to reads and writes.
   With neither context, or with both, policies expose no rows and
   allow no writes. A setup or migration request cannot mint a site
   or obtain site, connector, report, erasure or ordinary runtime
   authority through the installation context.

4. DR-150 admits the installation context for verified first setup,
   approved pre-site migration and an exact current owner-approved
   app update under DR-149. The setup role can reach only the allowed
   installation-owned checkpoints in 0030. The migrator can use its
   own migration history. A table name alone grants no operation.
   Site-owned rows require site scope. The setup role
   cannot read or write SQLx history. The migrator has no setup
   metadata write or runtime authority. An ordinary app, connector
   or erasure role cannot use the installation context. Migration
   schema work remains under its reviewed plan and never turns this
   scope into site-data access.

5. Reuse the migration history table `_sqlx_migrations` and its owning
   SQLx version, checksum and outcome records. It has forced RLS and
   only the migrator's installation policy and grants. Each record
   binds the canonical installation ID in an `installation_id` column.
   Require its non-null value and default it from the bound installation
   context so SQLx history inserts need no extra argument. Preserve
   its columns and their meanings. Refuse a history row owned by
   another installation.
   The app starter checks the role, table ownership, RLS, forced RLS
   and required policies before starting with SUT-971 on failure.

6. Check both contexts are unusable after commit and rollback.
   Ordinary pooled connections still use `DISCARD ALL` before reuse
   to clear session state, prepared statements, listeners and advisory
   locks. Close a connection returned in an open transaction.
   Installation work uses a separately checked-out lifecycle or
   migration connection that cannot return to the ordinary app pool.
   The transaction pooler restriction stays in force.

7. This is a Behemoth Boot addition for Sutradhar's profile. Framework
   qualification uses synthetic installations and sites within the
   exact covered profile. The addition leaves matched Spring
   behaviour in its own profile.

### Drivers and pools

1. Drivers are mature crates behind one starter per database: PostgreSQL,
   MySQL, MariaDB, SQLite, DuckDB, Oracle and SQL Server. We do not write wire
   protocols. We compile no C or C++ (ADR-0021), so a driver whose engine is a
   C or C++ library is used only as a prebuilt binary or, if that is not
   possible, as a child process. SQLite and DuckDB follow that rule (DuckDB:
   linked as a prebuilt library if the crate allows it, otherwise run as a
   child process). Oracle and Couchbase need vendor client libraries and are
   likely blocked until a prebuilt, licence-clean build exists (QR-034).
2. A platform matrix (Windows 11, Ubuntu 26.04, Debian 13) is kept in the
   dossier for every starter. A starter is enabled on a system only after its
   tests pass there; SQL Server on Windows also needs certificate-checked TLS.
3. A pool has a minimum and maximum size, an acquire timeout, a maximum
   connection age, a maximum idle time, a validation check on borrow and a
   leak warning for a connection held too long. The default maximum is set so
   that the connections' memory plus the database's own limit stays inside
   spec 0078's PostgreSQL share, and it never exceeds the server's
   `max_connections` minus a reserve; the starter checks this at start.
4. The pool provides metrics (`db_pool_size`, `db_pool_in_use`,
   `db_pool_waiting`, `db_pool_timeouts`) to group PB-H.
5. A connection that fails validation is dropped and replaced, never handed out.
6. Timeouts apply to every query: `db.statement_timeout_s`,
   `db.lock_timeout_s` and `db.idle_in_tx_timeout_s`.
7. Connections to a database that is not on this machine require TLS with
   certificate checking (`db.tls`). The bundled PostgreSQL uses mutually
   authenticated loopback TLS under spec 0118 on all supported systems.
   Secrets come from spec 0065. The pool applies the current key epoch.

### Lazy start

1. With `db.lazy_connect` on, the pool opens no real connection at start. The
   first query opens one. Start-up then does not wait for the database.
2. With it off (the default), start-up opens the minimum connections and a
   failure stops the start with a plain message.
3. A health check never counts a lazy pool as down before its first use; it
   reports "not yet used".

### Transactions

1. A function may be marked transactional with propagation, isolation, a
   read-only hint, a timeout, and rollback rules ("roll back on these error
   types, commit on those"). The propagation modes are those of the official
   reference (unverified): required, requires new, nested, supports, not
   supported, mandatory and never.
2. Protected statements need a scoped transaction. Apply these modes as follows.
   "supports" and "not supported" allow a path with no scoped
   transaction only when it touches no protected table. The authorised
   site or installation scope is bound before the first savepoint.
   Rolling back to that savepoint preserves the original scope.
3. Nested transactions use savepoints.
4. By default an error that is not listed as commit-on-error rolls the
   transaction back. An error is never swallowed (AGENTS.md 5.1).
5. A read-only transaction is started read-only before its first statement, so
   the database itself refuses writes.
6. A deadlock or serialisation failure (SQLSTATE 40P01, 40001) is retried a
   bounded number of times (`db.tx_retries`, default 2) with backoff, and only
   for a function marked idempotent. Otherwise it fails with its code.
7. A transaction never spans an outside call (a web request or a message
   send) unless the code names it. A long transaction raises a warning at
   `db.tx_warn_s` and is ended by the database at `db.idle_in_tx_timeout_s`.
8. After-commit hooks run only after a successful commit.

### Schema migrations

1. Migrations are numbered plain SQL files in `migrations/` (AGENTS.md 5.2).
   The engine builds on the sqlx migrator (rule 14) and runs as the migrator
   role at install and upgrade (spec 0030 owns when and the pause budget). The
   extra features here are our additions beyond sqlx: a validate-only mode, a
   dry run, repeatable migrations and a rollback pairing. That they match what
   Flyway and Liquibase users expect is unverified until the scout confirms.
2. An applied file never changes. A changed checksum stops the start (SUT-975).
3. Locking uses a PostgreSQL advisory lock on a dedicated session connection
   that is not behind a pooler. A second process waits up to
   `migrate.lock_timeout_s`, then fails (SUT-976).
4. A rollback plan is a paired file named `NNNN_name.down.sql` beside
   `NNNN_name.up.sql` (sqlx's reversible naming). A migration with only an up
   file is "forward only" and cannot be rolled back. App update and
   previous-payload recovery use specs 0030 and 0031. Keep committed
   production migrations forward-only for those routes. A paired file
   cannot supply schema compatibility or authorise a down migration
   against a live installation. Hold an incompatible previous payload
   before launch.
5. Before a numbered migration commits, check enabled and forced
   RLS, required policies and role grants for each affected table and
   its migration history. Run the check after the migration's SQL on
   the same locked, installation-bound transaction under the migrator
   role. Commit the verified schema and that version's SQLx success
   history together. A failed check returns SUT-971 and rolls back
   that transaction. Earlier committed versions remain. Hold the
   adapter profile or file before any migration effect if it cannot
   keep the SQL, check and history in that transaction. This includes
   a file requiring execution outside it. Squawk still checks the
   migration before admission. If commit is uncertain, reconcile exact
   schema, role, policies and history under the original intent and
   lock before publishing one native success checkpoint or retrying.
   An unproved outcome stays held.
6. Another tool's migrations live in `migrations/<tool>/` and that tool applies
   them with checksums, as AGENTS.md 5.2 says; this engine only checks the
   result.
#### Initial migration history and recovery

Before the first PostgreSQL scope, verify the exact 0030 package and
its first trust requirements. Under the home lock, use the confirmed
native installation identity, even when the site register is empty.
Reuse the existing LedgerWriter intent and durable checkpoint. Pin
the installation ID, register format and generation, verified home
and database root, migration input hashes and the approved operation
and checkpoint identity. Tijori supplies the existing lifecycle
credential to its exact component. On an established installation the
checkpoint carries the prior approved migration evidence reference.
PostgreSQL cannot supply the first scoped transaction's identity.

Use the existing dedicated migration session for the advisory lock.
Acquire it under the bound installation scope and keep the session
while the history and migration work runs. Release it before closing
the session or let connection close release it. Keep the first
history operation under the same verified installation binding.

For a proved empty PostgreSQL setup, use one initial transaction
under the bound installation scope to create SQLx's empty migration
history table, its installation ID column, policies and grants.
Set enabled and forced RLS, verify those settings and commit them
together before any history row is read or written by SQLx. Ordinary
connections remain unavailable until the owning schema and role
checks pass. The initial metadata setup records its native
checkpoint and stores no migration success row. An existing history
table is verified under its current scope and policy. Missing or
inaccessible history on an established installation holds without
recreating it. A legacy history without verified installation ownership
or forced policy fails SUT-971 before row access or migration. Compare
the prior admitted applied checkpoint
before starting a later migration. A filtered history with no visible
rows cannot recast an established install as a new one.

Before migration, verify a current DR-150 setup or update intent.
An update with enrolled sites needs the exact DR-149 approval. Run
its numbered, checksummed migrations on the verified session.
Qualify the SQLx and PostgreSQL driver versions, enabled features,
dependency lock digest and execution mode for the release platform.
Match that evidence to the migration profile in the verified 0030
intent before running the adapter. Exercise its history relation and
column contract, dirty or pending reads, checksums, migration SQL and
history writes. Preserve the original versions, checksums and rollback
semantics.

Each new transaction on the locked session, including one opened by
the adapter, binds the installation as its first statement. Scope
checks precede protected SQL and savepoints. An inner savepoint keeps
the checked outer scope. Commit and rollback end that scope. Refuse
the path before migration or checkpoint if a boundary cannot guarantee
that order, if the adapter uses a different session or if a migration
needs protected SQL outside a bound transaction. Keep SUT-975 for
checksum failure and SUT-976 for lock timeout.

If work stops before the initial transaction commits, resume only
the original native intent for the exact verified installation.
After commit and before native checkpoint publication, reconcile
the exact history shape, role and policy and any admitted SQLx
version and checksum result on the original installation. Write
the matching native receipt once. An absent or stale checkpoint,
changed root or installation identity, unproved outcome or
incompatible restored ledger holds the operation. Use 0030's
existing repair and restore authority. An authorised move or
restore uses the exact identity in its verified native plan. In a
new home verify its new root binding under that same authority.
A restored database cannot choose the identity or convert installation
context into site context.

### Redis and Valkey

1. Clients for a single node, a primary with replicas, sentinel and cluster,
   with read routing (writes to the primary, reads to replicas by choice).
2. Listener macros subscribe a function to a channel or pattern. Messages are
   delivered in order per channel, with a bounded buffer: a slow listener loses
   its subscription with SUT-977 rather than growing memory.
3. Redis is off in the Sutradhar profile. Valkey is treated the same way.

### Other database families and search adapters

1. MongoDB (including bulk writes), Neo4j, Cassandra and Couchbase each get a
   starter and a repository layer that follows the repository rules above where
   the database allows. Drivers are mature crates; a family with no usable
   crate is blocked and listed in the audit (QR-034, AU-BD-01).
2. Elasticsearch and OpenSearch get typed clients, a query builder, index
   template management and highlight mapping. These are framework parity only.
   Spec 0038 owns Sutradhar's search index trait and its connection to an
   owner's own search service; it may reuse these clients, and the product's
   own keyword search is the embedded Tantivy store of spec 0038.
3. Not in this spec: the keyword store, ingestion and text engine
   (specs 0038, 0022 and 0026), the
   embedded vector index (spec 0027 and ADR-0021), the durable job queue and
   outbox (spec 0019 and group PB-F), and the DuckDB analytics tables (spec
   0034). This spec owns only the DuckDB SQL starter.

### Caching

1. Attributes mark a function as cacheable, as updating the cache, or as
   evicting it. The key is built from the arguments by a typed key function, not
   by reflection.
2. A cache key always includes the site, so a cached value can never cross a
   site and bypass row-level security. A cache without a site in its key is
   refused at build or start.
3. Tiers: a local async in-memory cache first, and an optional shared store
   (Redis or another) second. A read checks the tiers in order and fills the
   upper tiers.
4. Each cache has maximum entries and bytes, a time to live and an eviction
   rule. The local caches together stay under `cache.local.max_bytes`, a share
   of the resident budget of spec 0078 (set after measurement), and are shed at
   pressure level 6 of spec 0078's table.
5. A cache never stores a value that is marked secret, and a cache key never
   contains one.
6. Stampede control: concurrent misses for one key run the load once.

### Reuse

Our own code is the repository and query derivation, the entity mapping and
change tracking, the site context, the transaction manager, the migration
rules, the listener macros and the cache attributes. Mature crates sit behind
them: database drivers, connection pooling, the migrator, Redis and search
clients, and a local cache. Each choice is a build-or-reuse record in the
research dossier.

#### Repositories, transactions and cache boundaries (N-1754–N-1762)

Use exact typed contracts and current site context. Framework
adapter parity does not choose a product backend or grant a network
profile. PostgreSQL and forced RLS remain the product's database
boundary. Keep app and migration roles separate and use bounded,
synthetic fixtures for independent proof.

| Owner catalogue entry | Exact data contract |
| --- | --- |
| N-1754 Clean-room ORM and change-tracking engine | Use an explicit typed entity and fetch plan. Bind a version, site and exact changed fields to the save. Enforce current row and cascade limits, site fences and owning audit. A stale version, denied related row or incomplete effect holds atomically. No inferred load, broad cascade or cross-site relationship. [FX-0108-W5-TYPED-ENTITY-SITE-AND-VERSION-SAVE](../planning/feature-wave-5.md#acceptance-design-for-n-1743-to-n-1766). |
| N-1755 Clean-room SQL drivers and connection pools | Qualify each named adapter using its admitted driver and typed contract. Check connection identity, bounds, timeouts, eligibility, reset and safe close before reuse. Product role never owns tables, migrates or bypasses RLS. Bind the exact site through the transaction-local setter before any application statement or savepoint. Close a connection returned with an open transaction. Recheck fail-closed before reuse. External drivers and destinations need separate admission, and unsupported profiles stay unavailable. [FX-0108-W5-DRIVER-ADMISSION-AND-POOL-RESET](../planning/feature-wave-5.md#acceptance-design-for-n-1743-to-n-1766). |
| N-1756 Lazy datasource connection startup | Open no connection for the exact admitted lazy datasource before first use. Report its unused state separately from verified readiness. First use checks the current database, role, site, schema and resource availability. BD-11/30 and 0112 keep the initial not yet used state and their readiness policy. Work waits on the proved current connection. Readiness and health reads do not become a first-use query. Once first use records a failure for this pool, its earlier unused status cannot supply current readiness under 0112. Liveness remains separate. No implicit mode change or bypass. [FX-0108-W5-LAZY-FIRST-USE-READINESS](../planning/feature-wave-5.md#acceptance-design-for-n-1743-to-n-1766). |
| N-1757 Declarative transaction manager | Bind propagation, isolation, read-only intent and savepoints to the exact site and function contract. Enforce read-only in the database, reconcile outcomes under the owning effect identity and retry only the existing bounded, eligible idempotent cases. Set the site locally with a bound value before the first statement or savepoint. Refuse nested cross-site scope or connection reuse with unresolved state. Retry eligible deadlock or serialisation failure only. Keep the original identity and reconcile an uncertain commit. [FX-0108-W5-SCOPED-SAVEPOINT-AND-COMMIT-RECONCILIATION](../planning/feature-wave-5.md#acceptance-design-for-n-1743-to-n-1766). |
| N-1758 Built-in database schema migration engine | Use the existing reviewed, numbered and checksummed migration route with the separate migrator role and startup lock. Ordinary migrations go forward. Verify the exact qualified pair for a requested isolated rollback drill. Check checksums, compatibility, migrator role and forced RLS including migration records after each change before product access. A dry run uses a disposable model and cannot mutate the real schema. A live forward migration requires the owning route and exact approved candidate. App rollback uses 0030 and 0031 and cannot run a down migration on the live database. No schema change under a test role or ordinary app role. [FX-0108-W5-CHECKSUM-MIGRATOR-AND-ROLLBACK-PAIR](../planning/feature-wave-5.md#acceptance-design-for-n-1743-to-n-1766). |
| N-1759 Redis and Valkey engine with listener macros | Qualify only the admitted topology, message, buffer and reconnect cases through framework profiles. A declared listener has an exact current grant, data and channel scope. On lag, revoked grant or unknown state close the affected subscription under the owning limit. No product connection, queue effect or native send from declaration. [FX-0108-W5-SUBSCRIPTION-SCOPE-LAG-CLOSE](../planning/feature-wave-5.md#acceptance-design-for-n-1743-to-n-1766). |
| N-1760 NoSQL, graph, and wide-column repository engines | Qualify each named store and typed read or write form independently under the existing parity and licensing routes. Apply explicit scope, operation, caps and receipt semantics, including partial batch outcomes. A missing driver or unsafe data boundary is blocked. No backend switch, extra connector or import follows. [FX-0108-W5-TYPED-BACKEND-EXACT-RECEIPTS](../planning/feature-wave-5.md#acceptance-design-for-n-1743-to-n-1766). |
| N-1761 Search-engine repository adapters | Qualify separately admitted framework search queries, templates and highlight projections on bounded fixtures. Check exact actor and index scope, typed fields and supported limits. Protected output and templates retain their own permissions. The product retains its existing index and no local query can install a remote search service. [FX-0108-W5-SCOPED-SYNTHETIC-INDEX-PROJECTION](../planning/feature-wave-5.md#acceptance-design-for-n-1743-to-n-1766). |
| N-1762 Multi-tier caching abstraction | Use exact site, cache class, profile version and current rights. Bound size, lifetime, in-flight loads and coalescing by scope. Apply exact site, current policy key, grant and erasure generations at read, coalescing, refresh, eviction and release. Pin admitted backend and DR-071 matrix. No unadmitted adapter or authority from shared-tier recovery. Never cache secrets or session authority. Invalidation, loss or a shared-tier outage cannot reveal stale data or grant new access. [FX-0108-W5-CACHE-CURRENT-SITE-GRANT-AND-ERASURE](../planning/feature-wave-5.md#acceptance-design-for-n-1743-to-n-1766). |

#### N-1841 Declarative repository macro engine

Derive and validate the exact typed query against the pinned schema
snapshot and offline metadata at build time. Reject unknown fields,
ambiguous method names, disallowed sort or filter, unbound values
and schema drift. Runtime uses only the admitted prepared shape and
current site transaction with forced row-level security. Preserve
explicit scope, result and mutation caps. No string-to-query
compiler, SQL concatenation or authority from a method name.
[N-1841](../planning/feature-wave-5.md#acceptance-design-for-n-1839-to-n-1862)
uses fixture `FX-0108-W5-PINNED-QUERY-SCHEMA-AND-SITE-SCOPE`.

### Data

- Ledger and behaviour descriptions: repository files (spec 0081).
- Migration history table: in each database, with row-level security.
- Query metadata for build-time checks: generated files, marked generated
  (AGENTS.md section 8.5) and checked against the schema in the gate.

### Settings

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `db.pool.max` | whole number | 8 | 1 to 200 | next start |
| `db.pool.min_idle` | whole number | 1 | 0 to 200 | next start |
| `db.pool.acquire_timeout_s` | seconds | 5 | 1 to 300 | next start |
| `db.pool.max_age_s` | seconds | 1,800 | 60 to 86,400 | next start |
| `db.pool.leak_warn_s` | seconds | 60 | 5 to 3,600 | next start |
| `db.lazy_connect` | on or off | off | two values | next start |
| `db.migrator_secret` | secret reference | none | a secret id | next start |
| `db.app_secret` | secret reference | none | a secret id | next start |
| `db.tls` | required, local only | required for remote | two values | next start |
| `db.lock_timeout_s` | seconds | 5 | 1 to 600 | next start |
| `db.idle_in_tx_timeout_s` | seconds | 60 | 1 to 3,600 | next start |
| `db.tx_retries` | whole number | 2 | 0 to 10 | next start |
| `db.statement_timeout_s` | seconds | 30 | 1 to 3,600 | next start |
| `db.tx_warn_s` | seconds | 10 | 1 to 600 | next start |
| `data.max_page_size` | whole number | 200 | 1 to 10,000 | next start |
| `data.max_rows` | whole number | 10,000 | 1 to 1,000,000 | next start |
| `data.max_delete_rows` | whole number | 1,000 | 1 to 1,000,000 | next start |
| `data.max_cascade_rows` | whole number | 1,000 | 1 to 100,000 | next start |
| `migrate.lock_timeout_s` | seconds | 60 | 1 to 3,600 | next start |
| `migrate.validate_only` | on or off | off | two values | next start |
| `cache.local.max_entries` | whole number | 10,000 | 10 to 10,000,000 | next start |
| `cache.local.max_bytes` | bytes | 67,108,864 | 1 MiB to 4 GiB | next start |
| `cache.local.ttl_s` | seconds | 600 | 1 to 86,400 | next start |
| `redis.enabled` | on or off | off | two values | next start |
| `redis.url_secret` | secret reference | none | a secret id | next start |
| `redis.tls` | on or off | on | two values | next start |

There is no setting that turns off row-level security checks, the site context,
parameter binding or checksum validation.

### Failure codes

SUT-970 database not reachable at start; SUT-971 required row-level security,
role or migration-history ownership proof failed; SUT-972 query without its
valid transaction scope; SUT-973 stale
version, save refused; SUT-974 query derivation failed at build; SUT-975
migration checksum mismatch; SUT-976 migration lock timeout; SUT-977 listener
dropped for lag; SUT-978 cascade too large; SUT-979 database family blocked
(no usable driver); SUT-980 migrator or app role missing or wrong; SUT-981
unretried deadlock; SUT-982 cache key without a site; SUT-983 row cap exceeded;
SUT-984 sort or filter field not allowed.

### Security and privacy

- Values are always bound as parameters. No text is joined into a query.
- Secrets never appear in logs, cache keys, audit records or error text.
- The database role is least-privilege; the starter checks it.
- Connection strings and passwords are secret settings (spec 0106).

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| BD-01 | A derived query MUST be built and checked at build time, with a wrong field or type stopping the build and naming the nearest names. | UT-BD-01, PL-BD-01 |
| BD-02 | Values MUST be bound as parameters and never joined into query text. | UT-BD-02, FZ-BD-01 |
| BD-03 | A page request MUST be capped at the maximum page size. | UT-BD-03 |
| BD-04 | A relationship MUST be loaded by an explicit fetch plan, and reading an unloaded one MUST NOT compile. | UT-BD-04, PL-BD-02 |
| BD-05 | A save from a stale version MUST fail and change nothing. | UT-BD-05, BD-BD-01 |
| BD-06 | A cascade over the row limit MUST be refused. | UT-BD-06 |
| BD-07 | Every site transaction MUST bind its site first. A query with no site scope MUST return no site rows and be logged. | UT-BD-07, IT-BD-01 |
| BD-08 | The starter MUST refuse to start with a role that owns tables, is a superuser or can bypass row-level security. | IT-BD-02 |
| BD-09 | A pooled connection MUST be reset on return so no site context leaks. | IT-BD-03, PL-BD-03 |
| BD-10 | Pools MUST validate, replace bad connections and warn on leaks, and report metrics. | UT-BD-08 |
| BD-11 | Lazy start MUST open no connection until first use and report "not yet used". | UT-BD-09 |
| BD-12 | Transactions MUST support the stated propagation, isolation, read-only, savepoint and rollback rules. | UT-BD-10, IT-BD-04 |
| BD-13 | A read-only transaction MUST refuse writes in the database. | IT-BD-05 |
| BD-14 | A changed migration checksum MUST stop the start, and only one process MUST migrate at a time. | UT-BD-11, FI-BD-01 |
| BD-15 | A migration that creates a table without forced row-level security MUST fail. | UT-BD-12 |
| BD-16 | A listener MUST have a bounded buffer and lose its subscription on lag. | FI-BD-02 |
| BD-17 | Caches MUST have limits, shed under memory pressure, never store secrets and run one load per concurrent miss. | UT-BD-13, FI-BD-03 |
| BD-18 | A database family with no usable driver MUST be marked blocked, not faked. | AU-BD-01 |
| BD-19 | Each feature MUST have parity rows, behaviour descriptions and independent contract tests. | CT-BD-01 |
| BD-20 | Migrations MUST use the migrator role and forced RLS history under its installation scope. Ordinary service connections use the restricted app role. Setup, erasure and connector roles retain only their exact owning grants. | IT-BD-06, UT-BD-14 |
| BD-21 | The first statement MUST bind exactly one authorised site or installation value with its fixed transaction-local setter. A second context, bad identity or protected statement outside its transaction MUST refuse. | UT-BD-15, PL-BD-04 |
| BD-22 | An open transaction returned to a pool MUST close. Neither context nor a session-level setting may survive connection reuse. | PL-BD-05, IT-BD-03 |
| BD-23 | A schema change that no longer matches a derived query or the offline metadata MUST fail the build. | UT-BD-16, CT-BD-02 |
| BD-24 | A deadlock or serialisation failure MUST be retried at most the stated times, and only for idempotent functions. | FI-BD-04 |
| BD-25 | Statement, lock and idle-in-transaction timeouts MUST apply to every connection. | IT-BD-07 |
| BD-26 | A remote database connection MUST use TLS with certificate checking. | UT-BD-17 |
| BD-27 | A cache key MUST include the site, and two sites using one key MUST NOT see each other's values. | UT-BD-18, PL-BD-06 |
| BD-28 | Sort and filter fields from a client MUST come only from the allow-list, and unpaged reads and deletes MUST be capped. | UT-BD-19, FZ-BD-02 |
| BD-29 | A migration with a changed checksum, a missing paired file in a requested isolated rollback drill, or a table without forced RLS MUST fail with its code. | UT-BD-11, UT-BD-12 |
| BD-30 | A database unreachable at start MUST fail with SUT-970 and a plain message; with lazy start it MUST not fail until first use. | UT-BD-09, IT-BD-08 |
| BD-31 | Redis, search adapters and each other database family MUST have a starter that starts, and a contract test per parity item, or be marked blocked. | CT-BD-03, AU-BD-01 |
| BD-32 | Pool metrics MUST be provided under the stated names. | UT-BD-20 |

## How it is tested

Designed, not run.

- **Unit (UT-BD):** the method-name grammar with a table of valid and invalid
  names; parameter binding; page and cascade limits; stale-version save;
  pool rules with a fake driver; lazy start; transaction rules with a fake
  database; checksum and lock behaviour; cache rules.
- **Planted faults (PL-BD):** a method name with a wrong field; reading an
  unloaded relationship; a connection that returns to the pool still carrying
  a site or inside an open transaction; a statement outside a transaction; a
  cache key with no site; each must be refused with its named message, while
  an honest example passes.
- **Integration (IT-BD):** a real PostgreSQL 18 started by the test harness
  with no Docker; two sites using one pool and never seeing each other's rows;
  a role that can bypass row-level security refused; read-only enforced by the
  database.
- **Site binding (UT-BD-15, PL-BD-04, PL-BD-05, IT-BD-03):** start a
  transaction for synthetic site alpha and verify constant statement text
  and a separate bound site value. Reject a planted session-level setting
  or a data statement before binding. On binding failure, observe zero
  application statements and no change. In separate commit and rollback
  cases, verify that the same physical connection has no usable site
  context before pool reset. Verify reset, check zero visible rows without
  a site and then bind beta on that connection. Only beta rows are
  visible. Separately, return a connection with an open transaction and
  verify that it closes without reuse.
- **App role changes before first borrow (IT-BD-02 and IT-BD-03).**
  In the 3C synthetic product profile start with the restricted app
  role and a clean pooled connection. Verify startup and the consumed
  PB-D profile. Before the first ordinary site-action checkout, change
  only that same named test role's ability to bypass RLS. Keep the
  database, schema, TLS identity, 0020 caller and exact requested site
  fixed. Drive the admitted synthetic action from the local door. The
  borrow checks under BD-10 and N-1755 reject the ineligible connection
  before the site binder or any application SQL. Return the existing
  safe SUT-971 role-proof failure with no site result, mutation or
  business outbox event. The earlier startup proof cannot clear this
  checkout, and no setup or migrator substitute can run the action.
  In the eligible control take a freshly checked restricted checkout
  on the same profile and verify that only the bound site's rows are
  visible. Keep this first-use check separate from a migration or
  restore failure.
- **Installation scope and SQLx history (IT-BD-06, UT-BD-14,
  UT-BD-15, PL-BD-04 and spec 0030 tests 5 and 17).** Use disposable
  PostgreSQL 18 with a verified native installation ID and confirmed
  empty site register. Run the native checkpoint before any history
  read. Observe forced RLS and exact role and scope policies before
  the first history row. Apply and resume the same numbered SQLx
  migrations with their expected version, checksum and rollback
  results. Observe the exact pinned path on the dedicated PostgreSQL
  session. For each start, including one managed by the adapter,
  observe the fixed bound statement first and the scope check before
  history, migration SQL or a savepoint. Observe scope ending after
  commit and rollback and the lock held across those boundaries.
  Hold a mismatched pin and a path that cannot bind before protected
  work, with no migration or success checkpoint. Confirm that the
  setup operation sees only its exact installation record and the
  migrator sees only its permitted history under that scope.
- In separate cases substitute a different install identity,
  replace the admitted database root, restore an obsolete native
  checkpoint, return a setup connection to the ordinary pool,
  submit a malformed scope or plant both contexts. Verify
  SUT-972 on a bad scope before application SQL. Force a binding
  failure and require no protected statement or durable change.
  Use matching synthetic identity bytes in the two admitted encodings.
  Each scope reaches only its own authorised rows. Deny ordinary
  app or connector installation access and a setup attempt to
  write migration history. Keep erasure and connector tests AU-19
  and AT-CN35 under their respective owners.
- **Initial history fault cases (FI-BD-01, IT-BD-06).** Interrupt
  initial metadata creation before commit, then separately after
  commit and before the native checkpoint. Try a second migrator
  while the first owns its lock. Verify atomic rollback or one
  recorded valid setup and reconcile a lost completion by the
  original intent. Omit forced RLS, replace the metadata history
  on an established installation and change one previously applied
  checksum in separate cases. Offer valid old checksums without the
  history ownership binding. Plant an unbound adapter history write
  after commit and a stale owner approval after site enrolment.
  Hold before migration or checkpoint. A matching approved update
  can reach only its scoped installation records. No test grants
  site access.
- **Numbered migration atomicity (UT-BD-12, FI-BD-01, IT-BD-06).**
  Keep an earlier admitted version and plant a later migration that
  creates a table with RLS enabled and without forced RLS. Observe
  SUT-971, no new table or success row, no advanced setup checkpoint
  and the earlier committed state intact. Repeat with a missing
  required policy or overbroad grant. Offer an adapter that cannot run
  the check before commit, then a file needing execution outside the
  bound transaction. Neither starts migration SQL or writes history.
  With a valid migration, lose the reply after commit. Read back the
  exact version, checksum, schema and role policies under the original
  scoped lock. Publish one matching native checkpoint without rerunning
  the migration. Unproved read-back keeps the checkpoint held.
- **Fault injection (FI-BD):** a database that drops mid-transaction; two
  processes migrating at once; a slow listener; memory pressure on the local
  cache.
- **Fuzz (FZ-BD):** hostile values in every bound position never change the
  query shape.
- **Contract (CT-BD):** one test per parity item, written from the behaviour
  description by someone other than the implementer.
- **Audit (AU-BD):** each blocked database family is listed with its reason.
- **Measurement (MS-BD-01):** query overhead of a derived query against
  hand-written SQL; pool memory at the default size on the 8 GiB reference;
  the proposed numbers (page size, cascade rows, pool size, timeouts) are
  checked against a representative load and revised.

## Changes from the source

The pillar list asked for clean-room wire drivers, a full JPA-style mapper
with lazy loading, and Flyway and Liquibase clones. Behemoth Boot instead
wraps mature drivers, makes loading explicit (no lazy proxies, per AGENTS.md
5.1) and builds its migration engine on sqlx, adding the checks users expect.
The site and authorised installation contexts and row-level
security checks are Sutradhar additions, switched on by its profile.
Use the [0081 group map][pb-d-group] for their framework test scope.
The register keeps "Clean-room ORM" and "Clean-room SQL drivers" for
traceability. The build-or-reuse record explains the change.
Spec 0038 keeps the connection to an owner's own search service.

[pb-d-group]: 0081-behemoth-boot-parity.md
