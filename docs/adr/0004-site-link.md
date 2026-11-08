# ADR-0004: How the service and the sites talk

Decided by the owner on 8 November 2026.

## Decision

- A thin PHP add-on for XenForo and a thin plugin for WordPress, for the
  versions in [AGENTS.md section 5.4](../../AGENTS.md#54-the-site-link).
- The service on the owner's PC calls the site over HTTPS. The site
  never calls the owner's PC.
- Every message is signed and protected against replay, as set out in
  [AGENTS.md section 5.4](../../AGENTS.md#54-the-site-link).
- Moderators accept or refuse suggestions in the site's own admin pages.
- Each feature's public-page footprint stays within the budget in its
  specification.

## Why

- Shared hosts cannot reach a home PC, and home routers block incoming
  connections. When the service calls out, nothing needs opening at
  home.
- A thin add-on keeps the site fast and the risk small.

## Trade-offs

- Changes reach the site only when the service calls again, not
  instantly.
- Only the PHP version in AGENTS.md section 4 is supported, so hosts on
  older PHP are left out.

## How we check it

- Contract tests on both sides use the same signed test messages,
  including old, repeated and wrongly signed ones that must be refused.
- Footprint tests compare each feature against its budget.
- End-to-end tests on test sites in containers on Dell: first read,
  suggest, accept and undo.
