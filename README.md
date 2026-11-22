# Sutradhar

Sutradhar suggests useful internal links for XenForo forums and WordPress
sites. A service on the owner's Ubuntu PC reads the site's posts and
finds good links between them. A moderator reviews each suggestion in the
site's admin pages, and only accepted links are added. For example, a post
that mentions "replacing a cracked screen" could be offered a link to the
thread with the full repair guide, on those exact words.

## How it is built

- Rust for the app and the background service, on our own framework,
  Behemoth Boot; Rust also for operations and developer tools
  ([ADR-0019](docs/adr/0019-rust-for-tools.md)).
- Leviathan sits beside Behemoth Boot as set out in
  [ADR-0022](docs/adr/0022-behemoth-boot-and-leviathan.md).
- A Tauri desktop app with Angular 22 screens and our own accessible
  design system.
- Thin PHP add-ons for XenForo and WordPress that work on shared
  hosting.
- PostgreSQL with row-level security.
- Every check blocks every commit, push and release.

## Read more

- [Rules for contributors and agents](AGENTS.md)
- [Roadmap to version 1](docs/roadmap.md)
- [Decisions](docs/adr/)
- [Contributing](CONTRIBUTING.md)
- [Security](SECURITY.md)

## Licence

The app is under the Apache License, version 2.0. The XenForo add-on and
WordPress plugin are under the GNU General Public License, version 2 or
later. The name Sutradhar is covered by a trademark policy.
