# ADR-0022: Behemoth Boot and Leviathan

Decided by the owner on 19 November 2026, answer by answer, through the
question tool. This record supplements ADR-0021. It is a documentation
and architecture decision. It does not authorise application code,
executable tests, build files, deployment configuration or
infrastructure changes.

## Two crown jewels

Behemoth Boot and Leviathan are the two flagship components of
Sutradhar. Each is a complex, clean-room system the project wants to
show to the open-source community. Each has high priority and must never
drift from its stated purpose. Both gate Sutradhar 1.0.0: version 1 does
not ship until each passes its own parity or conformance ledger. The
framework layer and the crawler's own logic are built directly, with no
throwaway wrappers.

## Behemoth Boot

[ADR-0025](0025-refactor-and-boot-first.md) governs delivery order.
Finish the repository-wide refactor and qualify all of Boot before
starting downstream product implementation.

Behemoth Boot is the new name of Sutradhar Boot, our framework inspired
by Spring Boot. Every part of the shipped product is built on it, as
AGENTS.md section 5.1 says.

- **Scope.** All features of Spring Boot and of the Spring projects in its
  release train are written in Rust inside Behemoth Boot, whether or not
  Sutradhar uses them. The owner names Spring Framework, Security, Data,
  Batch, Integration, GraphQL, Kafka, AMQP, Session, LDAP, Web Services,
  gRPC, AI, Modulith and Cloud. Each feature is a clean-room design.
- **Own code and reuse.** We write the framework layer that makes it
  Spring-like: the container and wiring, auto-configuration, conditions,
  profiles, property binding, starters, the actuator, lifecycle,
  repository and transaction semantics, test slices, and the lockstep and
  parity ledger. Mature, maintained Rust crates sit underneath behind
  Behemoth Boot starters and are never reimplemented: HTTP servers, SQL
  and NoSQL drivers, serialisation, messaging clients, TLS and similar
  (AGENTS.md section 6 rule 14). Starters are final, not throwaway
  wrappers. Clean-room means no copying of Spring's Java code.
- **Version lockstep.** Behemoth Boot version numbers mirror Spring Boot's,
  so a version says which Spring Boot release it matches. The owner cites
  Spring Boot 4.1.2 as stable and 4.2.0-RC2 as the candidate; these are
  unverified and the research dossier must confirm them.
- **Monthly scout.** An agent runs on the first of each month. It reads
  Spring's public release notes, milestones, release candidates and
  configuration changelogs and lists every addition, change, deprecation
  and removal across the release train. Each item becomes a tracked
  clean-room ticket with acceptance tests, whether or not Sutradhar uses
  it. Scouts read only public documentation and observable behaviour,
  never Java source, and hand plain behaviour specifications to separate
  Rust workers.
- **Parity ledger.** A live ledger lists every Spring Boot module, property
  key and feature with its Behemoth Boot status. The categories are
  `matched`, `in progress` and `scheduled`. A release is blocked if parity slips
  behind the matching
  Spring Boot release. A configuration changelog diff flags missing,
  renamed or changed defaults. Deprecations and removals follow Spring's
  schedule. A migration switch refuses to start an app that still uses
  outdated or renamed keys. Spring security advisories are watched and
  matching fixes are checked in the Rust equivalent.
- **Pillars.** The owner's list of about 100 pillars is the input for the
  specifications: version lockstep and automation; wiring, auto
  configuration and compile-time start-up; web servers, routing and
  protocols; data access, repositories and transactions; security and
  identity; messaging, events and integration; batch, cloud and AI and MCP;
  Actuator, metrics and structured telemetry; developer tooling,
  packaging and testing; and clean-room discipline with standards proofs.
  The pillars are saved with this decision and become specifications in
  later slices, each with one owning spec and no overlap.
- **Assurance.** Behaviour-parity contract tests, Quint models of start-up
  order, shutdown, transactions, security filter order and batch restart,
  a continuous fuzzing farm for every parser, an identical Linux and
  Windows matrix, Bazel slicing, head-to-head benchmarks against the
  matching Spring Boot release, a zero-unsafe core gate, fault injection,
  and a compatibility matrix against external servers.

## Leviathan

Leviathan is the shared crawler. It does everything a Googlebot-class
crawler does, fetching as mobile or desktop, and never breaks the law or
a site's locks.

- **Legal wall.** It obeys robots.txt to the finest wildcard, backs off
  when the file cannot be reached, honours X-Robots-Tag and page meta
  tags, and honours AI and text-mining opt-out signals. It cannot solve
  CAPTCHAs, guess logins, pass paywalls or bypass access controls, and it
  avoids piracy, warez and adult material. It sends read-only GET and HEAD
  requests only. It checks collection rules before fetching, strips
  personal identifiers before writing text, applies regional legal rules,
  supports verified erasure with a receipt, and keeps a tamper-proof
  compliance ledger.
- **Identity.** Every request is signed with Web Bot Auth keys and carries
  an honest identity card and a published address list with opt-out
  instructions. It never pretends to be Googlebot or another company's
  bot. It has two declared identities: Leviathan, and Leviathan
  Browser-Mode for rendering. Cloaking checks say that sites which cloak
  only against verified Googlebot cannot be tested.
- **Politeness.** Live server-health brake, exact Retry-After obedience,
  per-address sharing for hosts that share a server, byte caps by file
  type, loop and trap stoppers, conditional refreshes, a site-wide asset
  cache, quiet hours, a domain circuit breaker and a crawl-budget
  simulator.
- **Own code and reuse.** The crawler's own logic is ours: the legal
  engine and compliance ledger, politeness and the frontier, receipts,
  the link graph, the vault and WARC output, crawl diffs, Pattern Clues,
  telemetry and the plain-English explainers. Commodity layers come from
  mature, maintained crates: HTTP/1.1, HTTP/2, HTTP/3 and QUIC, TLS and its
  cryptographic primitives (audited, pinned and in the SBOM), DNS with
  DNSSEC, compression decoders, image, PDF, font, XML and office decoders,
  and an existing embeddable engine for JavaScript, WebAssembly and
  rendering. The engine choice is a build-or-reuse decision in the
  research dossier. We do not write our own protocol stacks, decoders or
  cryptography. Wherever a crate is used we add the limits and checks
  below (bomb shields, quotas, process cages).
- **Storage and scheduling.** A disk-backed frontier, an adaptive
  freshness scheduler, a multi-node mode on the user's own workers, a
  content-addressed vault, WARC output, a crawl-diff engine, a link graph,
  a crash-safe journal, hot and cold tiering, and project isolation.
- **Safety.** Expansion-bomb and drive-by quarantine, trap avoidance,
  signed artefacts, zero-trust parsing workers, per-page resource quotas,
  and process cages for untrusted pages.
- **Transparency.** Live telemetry, a plain-English "why was this page not
  crawled" explainer, live steering, a discovery map, a pre-crawl impact
  estimate, log correlation, a local MCP interface and a post-crawl
  briefing.
- **Conformance.** Web Platform Tests, a fuzzing farm, Quint proofs of the
  compliance and scheduling rules, a synthetic mini-internet, a reference
  browser oracle, chaos and compliance suites, multi-day soak runs, an
  identical Linux and Windows matrix and reproducible builds.

### Scale and modes

Leviathan runs at the scale of one user's PC. The user points it at their
own sites or at any public site, at polite rates, within the memory
budget. An optional multi-machine mode uses the user's own workers. There
is no hosted index of the web.

Crash recovery replays verified local work. A network attempt with
possible handoff and no verified outcome stays held with its old
charge and wait. Reclaim only a visit proved unsent by the current
adapter after fencing the old sender. An old journal cannot revive
a spent permit. DR-156 and spec 0116 own the recovery contract.

It has two modes. Receipts mode streams a page in small chunks, scans for
links, keeps only a small receipt and discards the page, with a page size
ceiling, a hard memory fence, compact visited filters, a disk-backed
queue, a speed brake that follows free memory, and plain presets. A
receipt holds the linking address, target, anchor words, rel values, date
and a short snippet. Audit mode keeps snapshots, the vault and WARC,
rendering and screenshots, within a disk cap. Each job picks its mode.

Backlink work may use free public link maps such as Common Crawl's to
shortlist pages, then visits only those pages. Niche crawling starts from
a small set of public hubs, scores links by how well they match the niche,
and keeps a small fixed-size sketch to find the most-linked sites, with a
switch to hide very large platforms.

Pattern Clues is opt-in and local only. It keeps small clue cards and
reports public clues such as shared hosting, themes, outbound link overlap,
domain change and publishing rhythm, plus hidden text, cloaking checks,
generated-content slips, review schema that does not match the page,
city-swap doorway pages, fake freshness, parasite subfolders, keyword
stuffing, link rings, redirect chains, comment-spam graveyards and
injected footer links. A playful tone is allowed. It never accuses: every
finding shows the clues, carries an "allegedly" stamp and a disclaimer,
and cannot be exported or shared with names attached unless the user
confirms.

## Where the rules live

Spec 0081 owns Behemoth Boot's lockstep, scout and parity ledger. Spec 0082
owns Leviathan's identity, legal wall, politeness, modes and resource fences;
specs 0044 and 0045 keep the audit and backlink behaviour built on it. Spec
0083 owns the meaningful-line count and the anti-padding checks.

## Line targets and anti-padding

The tracked target is four million meaningful lines for Behemoth Boot and
four million for Leviathan, enforced by a hard-blocking gate. This is
narrower than the owner's earlier ten million proposal; ADR-0013's ban on
vanity numbers is narrowed to allow this one tracked measure. A
meaningful line is non-generated, not duplicated, reached by tests, and
survives mutation checks. The coding harness and every gate enforce
anti-padding rules: no copied or near-copied code, no dead code, no
generated bulk, no formatting inflation, and every capability traces to a
parity or conformance item. A gate counts only meaningful lines.
Completion is judged by the ledgers, tests and proofs; line counts alone
never complete anything.

## Delivery

The work is thousands of slices, each about 150k tokens, within the rules
of ADR-0021's Delivery section. Agents hand over with a prompt when close
to their limits.

## Explicit supersession map

| Earlier decision | Replaced or narrowed rule | Successor |
|---|---|---|
| ADR-0002, AGENTS.md 5.1 "Sutradhar Boot", a small Spring-Boot-inspired core | Behemoth Boot: full Spring Boot and release-train parity, lockstep versions | Behemoth Boot section |
| ADR-0013 no vanity numbers | One tracked measure: meaningful lines, with anti-padding | Line targets section |
| Specs 0044 and 0045 Leviathan as a small shared crawler | Googlebot-class crawler with its own stack, two modes and a legal wall | Leviathan section |
| The pillar lists' own HTTP, TLS, QUIC, DNS, decoder, browser and JavaScript engines, and clean-room drivers and ORM | Mature crates underneath our framework layer and crawler logic; no reinvented wheels | The two "Own code and reuse" bullets; AGENTS.md section 6 rule 14 |

Any older statement covered by this map has no normative force.

## Open items and risks

- The embeddable engine for JavaScript and rendering is not yet chosen.
- The Spring Boot version numbers and the listed Spring features are the
  owner's statements and are unverified.
- Both scales are expectations to be tested against the meaningful-line
  gate; a few million real lines each is the planning figure.
- Legal review is needed for Pattern Clues wording and for crawling any
  public site, including terms of service.

## Consequences

Two new specification families are needed, one for Behemoth Boot and one
for Leviathan, each with a parity or conformance ledger. Specs 0004, 0012,
0014 to 0017, 0044, 0045 and the roadmap change to agree. The harness and
gates gain the anti-padding rules.

## How this decision is checked

- The documentation checker finds no "Sutradhar Boot" in current rules.
- Spec 0076's traceability covers every pillar once it has a spec.
- The meaningful-line gate and its fixtures are named in the harness spec.
