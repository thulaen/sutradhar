# Sutradhar

Sutradhar suggests useful internal links for XenForo forums and WordPress
sites. A service on the owner's Ubuntu PC reads the site's posts and
finds good links between them. A moderator reviews each suggestion in the
site's admin pages, and only accepted links are added. For example, a post
that mentions "replacing a cracked screen" could be offered a link to the
thread with the full repair guide, on those exact words.

## How it is built

- Rust for the app, the background service and the tools, on our own
  framework, Sutradhar Boot.
- A Dioxus desktop app with our own accessible design system.
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

The app is under the GNU Affero General Public License, version 3 or
later ([LICENSE](LICENSE)). The XenForo add-on and WordPress plugin are
under the GNU General Public License, version 2 or later.
