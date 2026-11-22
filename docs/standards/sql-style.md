# SQL authoring guide

Use this guide for PostgreSQL migrations and database tests under
[AGENTS.md](../../AGENTS.md#52-data) and the first-party store
contracts in [spec 0108](../specs/0108-behemoth-data-access.md).

## Migration and query boundaries

- Keep schema changes in the owning numbered migration directory.
  Preserve each applied file and its checksum. Match the owning
  migration contract for forward changes and any declared rollback.
- Bind values through the typed database interface. Make the owning
  operation and transaction boundary explicit in the design and in
  review. R19 and H39 in
  [spec 0061](../specs/0061-nirikshak.md) cover their named classes
  of query construction and write sequences.
- Use forced row-level security for the tables in scope, including
  migration metadata. Preserve separate migrator and restricted app
  roles. Verify that the site context is bound at the start of the
  transaction and that absent context and pooled reuse cannot disclose
  another site's rows. Spec 0108 owns connection and migration details.
- Keep site-native database access in its own adapter scope. Spec
  0021 owns the site-side allow-list and read-only query contract.
  Product migration authority grants no site write.

## Review and checks

Trace the schema and query to its owner and test the migration
checksum, role grants, absent site context and transaction boundary
where applicable. Spec 0108 names squawk migration safety evidence
and the role and isolation checks. A formatting or syntax check
cannot establish tenant isolation or migration recovery.
