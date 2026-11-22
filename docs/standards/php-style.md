# PHP authoring guide

Use this guide for the PHP 8.5 site adapters. The language and
engineering rules are in [AGENTS.md](../../AGENTS.md#4-languages-and-files).

## Site-side code

- Keep typed requests, narrow responsibilities and exact failures
  visible in the adapter. Use the distinct native paths in
  [spec 0021](../specs/0021-forum-contracts-ingestion.md) and
  [spec 0031](../specs/0031-shared-platform-installation.md).
- Keep authentication, site binding, native authorisation and the
  approved effect separate in the request flow. Use the signed and
  sealed messages in
  [spec 0118](../specs/0118-end-to-end-encryption-gate.md).
- Put the platform's own namespaced hooks, routes and other declared
  interfaces at the boundary. Respect the native save and undo
  contracts, including conditional save, revision and conflict checks.
  Keep credentials and protected content out of diagnostic output.

## Review and checks

Use the owning PHP unit, failure and platform cases. Apply PHPStan at
its strictest and taint checks where the owning contract calls for
them. Review typed inputs and outputs, native permission, guest
visibility, protected storage and the exact bytes the operation is
allowed to change. Use the pinned Composer inputs and the gate
and review requirements in AGENTS.md. A clean analyser response
covers only the rules and routes it exercised.
