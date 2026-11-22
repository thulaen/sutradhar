# 0113: Behemoth Boot developer tools

This is group PB-I of Behemoth Boot (spec 0081). It owns the tools that make
building on the framework fast and safe: live reload while coding, development
services through Docker Compose, container-based integration tests, test
slices that start only one layer, in-memory test harnesses, the project
generator, the container image packager, single-binary and layered packages,
the configuration metadata catalogue, and the upgrade and migration doctor.
Spec 0081 owns the ledger and clean-room rules; specs 0106 to 0112 own the
framework behaviour these tools exercise; spec 0003 owns the toolchain and
check runner, and ADR-0021 makes Bazel the only build system.

These are developer tools. Sutradhar's owner never uses them: the owner's
experience is point and click with no terminal (ADR-0021). Sutradhar's own
programmers use the slices, harnesses, metadata and doctor every day. Several
items are framework parity only: they are built, tested and kept in step with
Spring, but Sutradhar does not use them. Sutradhar ships a `.deb` on Linux and
an unsigned installer on Windows, not container images, and the product and
its owner use no Docker. Its own tests start a real PostgreSQL 18 from the
test harness with no Docker (specs 0108 to 0111). The gate still pins and
scans every tool image it uses, in `gate/tools.toml` with Grype (AGENTS.md,
pinned tools and supply chain), and any dev-only container here passes through
those same rules.

Spring facts here come from the owner's pillar list (19 November 2026) and are
"unverified" until the scout confirms each against official notes (spec 0081).
That includes "Spring Boot 4.2's Buildpacks updates" and "4.2's startup
speedups". The behaviour description, not the Spring wording, is the contract.
Mature crates and Bazel rules sit behind our tools (ADR-0022, AGENTS.md
section 6 rule 14).

Example: Dev changes one route handler and saves. The watcher rebuilds only
the affected crate with Bazel and restarts the app process while the
development database and the warm model cache keep running, so the app is
serving again in under the proposed time. Dev then writes a `web_test` slice:
it starts only the web layer with an in-memory transport and a fake service,
checks one route and finishes without opening a port or a database. Later Dev
bumps the framework version; the upgrade doctor reports `cache.ttl` is now
`cache.ttl_seconds`, writes a patch that renames it, and lists two deprecated
calls. Dev reviews and commits the patch through the normal gate.

Values marked "proposed" are design choices to be checked by measurement.

## What it does

### Words used here

- **Slice test**: a test that starts only one layer of the framework.
- **Harness**: a test helper that stands in for something real, such as a
  network.
- **Starter, composition root**: defined in specs 0081 and 0106.
- **Compose file**: a file that lists development services to run in containers.
- **OCI image**: a standard container image.
- **Layered package**: a package split into parts that change at different
  rates.
- **Metadata catalogue**: the generated list of every setting with its type,
  default, range and description (spec 0106).
- **Doctor**: a tool that checks a project after an upgrade and proposes fixes.

### What this spec designs

| Feature | What the developer sees |
|---|---|
| N-1850 Sub-second DevTools live reload (unverified; the speed is a proposed target) | A save rebuilds only what changed and restarts the app fast, keeping heavy local state warm |
| N-1795 Built-in Docker Compose development bridge (unverified) | A compose file at start launches needed services and wires their settings |
| N-1796 Auto-wired container integration testing (Testcontainers wording, unverified) | Tests start real isolated services in containers and receive their addresses |
| N-1797 Fast vertical slice test runner (slice tests, unverified) | Test attributes that start only the layer under test |
| N-1798 In-memory mock web, gRPC and broker harnesses | Controllers, filters, security chains, gRPC and listeners tested with no real port |
| N-1799 Interactive Behemoth Initializr generator (unverified) | A screen and a wizard that scaffold a new app with chosen starters and ready tests |
| N-1800 Clean-room OCI container image packager | Lean, reproducible container images built by the build tool |
| N-1801 Fast-lookup single-binary and layered archives (unverified) | One self-contained binary, or layered bundles with indexed resources |
| N-1802 Auto-generated configuration metadata catalogue | Machine-readable settings metadata for editors, docs and settings forms |
| N-1803 Built-in upgrade and migration doctor | An audit after a version bump that proposes renames and flags deprecated calls |

### Live reload

1. A watcher tracks the files of the workspace and, after a short quiet period
   (`dev.debounce_ms`), asks Bazel (spec 0003) to rebuild only the affected
   targets. It then restarts the app process. Rust code cannot be swapped into
   a running process, so a restart is the mechanism. "Warm" state is kept in
   separate helper processes that do not restart: the development PostgreSQL,
   the model cache and the build server. The watcher supervises them and stops
   them all on exit or crash; a helper left behind is found and stopped at the
   next start.
2. Proposed target, measured and not a promise: from the save to the app
   serving again, one small crate changed, within a few seconds on the
   reference machine, build included; the part after the build finishes aims
   at under one second. The pillar list's "sub-second" is an aim. The build
   server's memory stays inside a stated share of the 8 GiB budget (spec
   0078); the share is set at the first measurement.
3. The watcher ignores generated files, version control folders, files over a
   size limit and any path matching the secret patterns in `dev.ignore` (key
   and certificate files, `.env` files, anywhere in the tree), not only secret
   folders. It never uploads anything.
4. Live reload exists only in a development build, behind a Bazel build
   setting (the crate feature) that the Sutradhar release build leaves out,
   and a gate test checks that no release binary contains it (SUT-1300).
5. A reload that fails to build keeps the previous process running and shows the
   compiler's error in plain English with the file and line.

### Compose bridge

1. At start, a development build looks for a compose file in the project. For
   each service it declares, the bridge starts it, waits until it is healthy,
   generates random credentials where the service needs them, and puts the
   connection settings into the settings layers of spec 0106. On exit it stops
   what it started and nothing it did not start. A service with no health check
   is waited on by a port test with a time limit, then marked "unverified".
2. Compose files are untrusted input. The keys `build`, `extends`, `include` and
   `provider` and similar are refused, exactly as the gate refuses them in its
   own set-up files (AGENTS.md, pinned tools and supply chain), and images must
   be pinned by digest in `gate/tools.toml` and scanned by the gate's Grype.
3. Credentials are random per run and kept in memory. They reach a container
   only through a temporary file with owner-only permission that is removed at
   stop, not through process arguments or a log. Broker virtual hosts or
   databases are created for the run and removed afterwards.
4. Supported container programs, found on the developer's machine and not
   installed by the framework: Docker and Podman on Linux, and Docker Desktop
   or Podman on Windows 11 (their own licences are the developer's concern).
   If none is found the start continues and says so (SUT-1301).
5. If the bridge itself crashes, the next start finds containers carrying its
   label and removes them.
6. Development only. The bridge is absent from a release build (BI-01), and
   Sutradhar's own tests do not use it.

### Container integration tests

1. A test declares the services it needs (a database, Redis, a broker). The test
   framework starts them in isolated containers, waits for health, and injects
   their addresses and credentials as settings, with no boilerplate in the test.
2. Each test run gets fresh containers or fresh databases inside a shared
   container, and removes them afterwards even when the test fails.
3. Images are pinned by digest. Digests live in `gate/tools.toml`, are scanned
   by Grype, and change only through a reviewed change; a digest that is not
   pinned fails the test. If an image is not already in the local cache and
   the machine is offline, the test is skipped with a clear note, never
   silently passed. Containers have no outward network except what the test
   needs and are limited in memory and time (`testsvc.*`).
4. It is optional and framework parity. Sutradhar's own integration tests start
   the real PostgreSQL 18 and other services as plain processes from the home
   folder layout of spec 0030, with no Docker (specs 0108 to 0111).

### Test slices and harnesses

1. Slice attributes start only one layer: `web_test`, `data_test`,
   `security_test`, `graphql_test` and `modulith_test` (names are our own
   choice, following the owner's list). A slice builds a composition root
   under the rules of spec 0106 from a layer manifest (the list of starters
   that belong to the layer) plus the fakes the test supplies, so a slice test
   of a route does not start the database. The layer manifests are files, one
   per layer.
2. A slice that needs a part that is not in its layer fails at start with a
   plain message naming the part and the layer (SUT-1302).
3. Detection: the container records the parts it started. A test compares that
   list with the layer manifest and fails on any part that is not in it, so a
   slice that "starts more than its layer" is caught when the test runs.
4. In-memory harnesses replace the network but keep the real code above it:
   the real request parser, filter chain and handlers (specs 0107 and 0109)
   are exercised, and only the socket and TLS are replaced by a memory pipe. A
   separate integration test covers real framing and TLS. A gRPC harness and a
   message listener harness work the same way. A harness opens no port, no
   socket and no file outside a temporary folder; this is enforced by a test
   that runs the harness inside the sandbox of the check runner (spec 0003),
   where such an attempt fails.
5. Tests are deterministic: time, random numbers and identifiers come from fakes
   that the harness provides, and parallel tests never share state.
6. A slice test records its start time. The slice must start in a fraction of
   the full app's start time (proposed: under one tenth, measured by MS-
   BI-01).

### Initializr

1. A project generator offers a screen and a command-line wizard that produce
   a new Behemoth Boot application: chosen starters, a composition root, a
   settings file with the three profiles, Bazel build files, a health check, a
   lock file, a software bill of materials and ready-to-run tests for each
   starter.
2. The files it writes are ordinary source that the developer then owns. They
   are not "generated" files in the sense of AGENTS.md section 8.5, carry no
   generated marker, and follow the rules of AGENTS.md (file and function
   limits, doc-tests, licence header). Template licences are recorded in the
   bill of materials.
3. It never overwrites an existing file; a clash is reported (SUT-1308).
4. The command-line form is a developer tool for programmers (ADR-0021 requires
   point and click only for the owner). The screen runs locally in the developer
   tooling and makes no outside network request; its list of starters is built
   from the ledger of spec 0081.

### Container image packager

1. The build tool builds an OCI image from the application binary, with a
   minimal base image, reproducible layers and a software bill of materials
   inside it. Images are Linux only; Windows has binaries and packages
   (below).
2. The base image is named by digest in `gate/tools.toml`, scanned by Grype,
   and its operating system distribution and version labels are checked
   against `image.base_allow` before the image is accepted (unverified: the
   owner's wording refers to the 4.2 buildpacks updates). Fetching a base
   image needs the network once; the result is cached, and a build with no
   cache and no network fails with a plain message.
3. Images are built through Bazel rules with no daemon and no Docker program
   required. Output is byte-identical for identical inputs. The known sources
   of difference are pinned: file times (set to a fixed value), archive order
   (sorted), paths (stripped), build ids and the toolchain version. A
   difference fails the reproducibility check (SUT-1304); a base image not on
   the allowed list fails with SUT-1303.
4. Parity only. Sutradhar does not ship images.

### Single binary and layered packages

1. The build tool can produce one self-contained binary with its resources
   embedded, or a layered bundle in which dependencies, resources and code are
   separate layers. For a program that is not a container the layers serve
   faster updates: an update replaces only the small top layer, so a download
   is small (spec 0030 owns how Sutradhar updates).
2. Resources have an index so a lookup is direct, not a scan (the match to
   Spring's "startup speedups" is unverified). A resource that is missing
   fails at build time (SUT-1307). The index lookup has a measured cost (MS-
   BI-01).
3. Every package carries a software bill of materials in CycloneDX form made
   by the gate's `aegis sbom` and a slice of `gate/sbom.cdx.json`, plus a hash
   manifest. Each component entry carries an `unsafe_count` property: the
   number of unsafe blocks in that component, taken from the unsafe-usage
   scanner of spec 0114 (BI-25). Spec 0030 owns the Sutradhar packages (its
   app, engine and supervisor parts, each with its own hash and signature),
   and the signing keys of release belong to tijori (spec 0065); this spec
   provides only the build rule. The Windows installer itself is unsigned by
   the owner's decision, but its payload parts are still hashed and checked by
   spec 0030.
4. Builds of binaries and packages are reproducible on Linux and Windows. The
   Windows claim depends on the project's own fix for rules_rust on Windows
   (ADR-0021) and is a release gate only after that fix (BI-13).

### Configuration metadata catalogue

1. Spec 0106 owns the field set of a setting's metadata. This spec owns the
   builder: a build rule (a Bazel rule driven by a derive macro on each
   settings struct) that writes the catalogue file as JSON, and a JSON Schema
   derived from it for editor autocomplete. For each setting it records name,
   type, default, allowed range, description, whether it is secret or
   refreshable (spec 0106 and QR-040), the profiles that change it and the
   owning module.
2. The catalogue is a generated file with the generated marker, checked
   against its source by hash (AGENTS.md section 8.5). Descriptions are
   written in the translation files; the generator reads them and the hash
   covers them too.
3. The catalogue feeds editor autocomplete, the generated settings reference,
   the Basic, Advanced and Engine Room settings views (ADR-0021, spec 0014)
   and the configuration changelog diff of spec 0081.
4. A setting without a description or a range fails the build (SUT-1305).

### Upgrade and migration doctor

1. After a framework version change the doctor compares the old and new
   catalogues and the ledger of spec 0081. It lists renamed and removed
   settings and changed defaults. For deprecated calls it uses the compiler's
   own `deprecated` warnings collected from a build (not a grep); it can find
   only what the compiler reports, and the report says so.
2. It writes a patch limited to settings files and the lines the compiler
   pointed at, never edits files itself, and lists every file it would touch.
   The developer or an agent applies it through the normal gate and review.
3. It then checks start-up on settings only: it loads the project's settings
   with `boot.strict_migration` on (spec 0106) and reports every outdated key,
   with no database or other service needed. A result with any warning is
   SUT-1306. `doctor.strict` on means a warning fails the run.
4. It uses only the data of the two versions; it needs no network.

### Reuse

Our own code is the watcher logic, the compose and test service lifecycle, the
slice builders and fakes, the generator templates, the image and package
rules, the metadata catalogue builder and the doctor. Mature crates and Bazel
rules sit behind them: file watching, container clients, archive and image
formats, template rendering and compression. Each choice is a build-or-reuse
record in the research dossier.

### Developer tools and artefact qualification (N-1795 to N-1803)

QR-216 binds these forms to their exact developer or framework profile.
Product installation and its ordinary tests use the native profile in
ADR-0021. Framework parity evidence gives no authority to install a
container tool for the owner or to ship a new product artefact.

| Feature | Owning behaviour and evidence |
|---|---|
| N-1795 | BI-01, BI-04, BI-05 and BI-19. In the admitted developer profile, check the manifest and image pins before starting a service. Generate random credentials in memory and keep any necessary transfer within spec 0118 custody. Report SUT-1301 and continue ordinary startup when the container programme is absent. Remove only the resources created by this run. Refuse disallowed manifest fields, unverified images and production use. [FX-0113-W5-PINNED-DEV-CONTAINER-OWNED-CLEANUP](../planning/feature-wave-5.md#acceptance-design-for-n-1791-to-n-1814). |
| N-1796 | BI-01 and BI-06. Start pinned and scanned fixtures in the optional isolated framework test profile. Check readiness and remove run-owned fixtures on success and failure. Record unavailable optional infrastructure as unavailable. A required lane needs its own passing evidence. [FX-0113-W5-OPTIONAL-CONTAINER-LANE-OWN-EVIDENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1791-to-n-1814). |
| N-1797 | BI-07, BI-17 and BI-18. Run a declared vertical slice and record the started parts. A missing dependency produces SUT-1302. Measure the startup against the proposed fraction on the declared comparable profile. [FX-0113-W5-VERTICAL-SLICE-EXACT-STARTED-PARTS](../planning/feature-wave-5.md#acceptance-design-for-n-1791-to-n-1814). |
| N-1798 | BI-08 and BI-09. Run existing web, gRPC or broker logic against explicitly admitted in-memory adapters and deterministic test inputs. Record which transport was substituted. Network and TLS conformance have their own test lanes. [FX-0113-W5-IN-MEMORY-HARNESS-NO-EXTERNAL-IO](../planning/feature-wave-5.md#acceptance-design-for-n-1791-to-n-1814). |
| N-1799 | BI-10 and BI-22. The local Initializr previews the selected installed template and emits editable project source to a new destination. Refuse unsafe paths and collisions. Include its licence header, lock and bill of materials. The scaffold is ordinary source and must build and pass its ready tests. [FX-0113-W5-INITIALIZR-EXACT-LOCAL-TEMPLATE-FILES](../planning/feature-wave-5.md#acceptance-design-for-n-1791-to-n-1814). |
| N-1800 | BI-11, BI-13, BI-20 and BI-25. Build the admitted Linux OCI artefact from verified local inputs with the declared base digest and operating system identity. Record the resulting digest and the CycloneDX bill of materials with component unsafe counts and its hash manifest. Product installation and other platforms retain their own qualification and release routes. [FX-0113-W5-REPRODUCIBLE-OCI-EXACT-PROFILE](../planning/feature-wave-5.md#acceptance-design-for-n-1791-to-n-1814). |
| N-1801 | BI-12, BI-13, BI-20, BI-23 and BI-25. Bind indexed and layered forms to their exact inputs, resource hashes and bill of materials. Measure the direct index lookup. A missing or unreadable required resource blocks qualification. Apply specs 0030 and 0065 to release custody and require exact host evidence for a Windows claim. [FX-0113-W5-ARCHIVE-VERIFIED-LOOKUP-AND-CUSTODY](../planning/feature-wave-5.md#acceptance-design-for-n-1791-to-n-1814). |
| N-1802 | BI-14. Generate configuration metadata from the typed spec 0106 declarations and their translated descriptions. Bind the result to the source hash and include redaction markers using the declared schema. Each setting needs a description and range. Secret values, sample secrets and current settings values stay outside the catalogue. [FX-0113-W5-METADATA-EXACT-SCHEMA-AND-REDACTION](../planning/feature-wave-5.md#acceptance-design-for-n-1791-to-n-1814). |
| N-1803 | BI-15. Keep any approved version-bound local patch as inert output, with exact source and target schemas, masked values and target collision hold. Patch creation does not write source config. The local doctor compares two exact versions and emits a settings migration diff with verified compatibility findings from a strict migration startup check. A warning fails the doctor run when its strict setting is on. Mark unknown mappings and keep the input intact. Normal setting and release controls own any later change. [FX-0113-W5-MIGRATION-DOCTOR-INERT-DIFF](../planning/feature-wave-5.md#acceptance-design-for-n-1791-to-n-1814). |

#### N-1850 Sub-second DevTools live reload

Keep the development-only watcher and pinned incremental build for
an exact eligible worktree. Debounce and ignore generated, private
and out-of-scope paths. A successful admitted build can restart only
the owned development instance at a safe boundary and reuse only
still eligible helper state. A bad build keeps the verified process.
Measure build and restart phases separately. The target is measured
under the existing profile. No runtime compilation, live code
replacement, release-binary watcher, automatic deployment or signing.

Use BI-03 for bounded save-to-ready time including build. The separate
post-build under-one-second aim excludes build time. Keep the watcher
inside the writable developer target. Exclude secret files by name
even outside secret directories, oversized inputs and version control
paths. On watcher exit or crash clean up only its owned warm helpers.
The runtime binary excludes the watcher.
See [FX-0113-W5-BOUNDED-DEVELOPMENT-RELOAD][w5-dev-watcher-fixture].

[w5-dev-watcher-fixture]:
  ../planning/feature-wave-5.md#acceptance-design-for-n-1839-to-n-1862

### Data

- The metadata catalogue, the package manifests and bills of materials:
  generated files, marked generated.
- The doctor's patch and report: files in the project's build output.
- Ledger and behaviour descriptions: repository files (spec 0081).

### Settings

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `dev.debounce_ms` | milliseconds | 200 | 10 to 10,000 | next start |
| `dev.watch_max_file_mib` | MiB | 5 | 1 to 1,024 | next start |
| `dev.ignore` | list of patterns | `**/generated/**`, `.git/**`, `**/*.pem`, `**/*.key`, `**/.env*`, `**/secrets/**` | any superset | next start |
| `compose.enabled` | on or off (development builds only) | on | two values | next start |
| `compose.file` | path | `compose.yaml` | a file | next start |
| `testsvc.memory_mib` | MiB | 512 | 64 to 16,384 | next test |
| `testsvc.timeout_s` | seconds | 120 | 5 to 3,600 | next test |
| `image.base_allow` | list | none | image names with digests | next build |
| `doctor.strict` | on or off | on | two values | next run |

There is no setting that lets a release build include live reload or the
compose bridge, or that lets the doctor edit files. `testsvc.memory_mib` stays
within the owner-PC limits of AGENTS.md.

### Failure codes

SUT-1300 development feature found in a release build; SUT-1301 container
program not found; SUT-1302 slice needs a part outside its layer; SUT-1303
image base not allowed; SUT-1304 package not reproducible; SUT-1305 setting
without description or range; SUT-1306 doctor found start-up warnings;
SUT-1307 resource missing at build; SUT-1308 generator would overwrite a file.
Codes SUT-1300 to SUT-1399 are reserved for this spec.

### Security and privacy

- The watcher, compose bridge and test containers never send files or settings
  outside the developer's machine.
- Random credentials stay in memory; none are written to a file, log or image.
- Containers and images are pinned by digest and run with limits.
- The doctor and generator never run network requests and never edit existing
  files.

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| BI-01 | A release binary MUST contain no live-reload, compose-bridge or test-container code, and the gate MUST fail otherwise (SUT-1300). | UT-BI-01, PL-BI-01 |
| BI-02 | A failed build during live reload MUST keep the previous process running and show a plain error. | FI-BI-01 |
| BI-03 | The watcher MUST ignore generated files, secret folders, version control and oversize files. | UT-BI-02 |
| BI-04 | The compose bridge MUST generate random credentials kept only in memory and MUST stop only what it started. | UT-BI-03, FI-BI-02 |
| BI-05 | A missing container program MUST NOT stop the start and MUST say so (SUT-1301). | UT-BI-04 |
| BI-06 | Test containers MUST be pinned by digest, limited, isolated and removed after the test even on failure. | IT-BI-01, FI-BI-03 |
| BI-07 | A slice MUST start only its layer and a slice that needs a part outside it MUST fail with SUT-1302. | UT-BI-05 |
| BI-08 | A harness MUST open no network port, socket or file outside a temporary folder. | UT-BI-06, PL-BI-02 |
| BI-09 | Harness time, random numbers and identifiers MUST come from fakes, and parallel tests MUST NOT share state. | UT-BI-07 |
| BI-10 | The generator MUST NOT overwrite a file (SUT-1308) and MUST produce a project that builds and passes its ready tests. | UT-BI-08, IT-BI-02 |
| BI-11 | An image MUST be reproducible (SUT-1304), carry its bill of materials and use only an allowed base by digest (SUT-1303). | UT-BI-09, IT-BI-03 |
| BI-12 | A package MUST carry a bill of materials and a hash manifest, and a missing resource MUST fail at build (SUT-1307). | UT-BI-10 |
| BI-13 | Builds of binaries and packages MUST be reproducible on Linux, and on Windows after the rules_rust fix required by ADR-0021. Images are Linux only. | IT-BI-04 |
| BI-14 | The metadata catalogue MUST be generated, checked by hash, and a setting without description or range MUST fail the build (SUT-1305). | UT-BI-11 |
| BI-15 | The doctor MUST write a patch and never edit files, MUST check settings start-up with strict migration on, and a warning MUST fail the run when `doctor.strict` is on (SUT-1306). | UT-BI-12, BD-BI-01 |
| BI-16 | Each feature MUST have parity rows, behaviour descriptions and independent contract tests. | CT-BI-01 |
| BI-17 | A slice test MUST start in under the proposed fraction of the full app's start time. | MS-BI-01 |
| BI-18 | A slice MUST fail when it starts a part outside its layer manifest, detected from the recorded started parts. | UT-BI-13 |
| BI-19 | A compose file with `build`, `extends`, `include` or `provider`, or an unpinned image, MUST be refused. | UT-BI-14, PL-BI-03 |
| BI-20 | A package MUST carry a CycloneDX bill of materials from `aegis sbom` and a hash manifest. | UT-BI-15 |
| BI-25 | Every component entry in a package's bill of materials MUST carry an `unsafe_count` from the 0114 scanner result for its exact resolved revision and admitted scan profile. Missing, incorrect or unbound counts MUST fail the build. | UT-BI-19 |
| BI-21 | The watcher MUST ignore the secret patterns anywhere in the tree and MUST stop all helper processes on exit or crash. | UT-BI-16, FI-BI-04 |
| BI-22 | Generated scaffolds MUST be ordinary source with no generated marker, and MUST include a lock file, licence header and bill of materials. | UT-BI-17 |
| BI-23 | The resource index lookup MUST be direct and its cost measured. | MS-BI-01 |
| BI-24 | The settings `dev.*`, `compose.*`, `testsvc.*`, `image.base_allow` and `doctor.strict` MUST be enforced as stated. | UT-BI-18 |

## How it is tested

Designed, not run.

- **Unit (UT-BI):** the release-build check on a binary with and without the
  development code; watcher ignore rules; credential handling; slice layer
  rules; harness limits; metadata completeness; doctor renames on a pair of
  fixture catalogues.
- **UT-BI-19:** use a synthetic package with current resolved inputs,
  correct package digests and an exact scanner result per component.
  Check that a previously verified zero still passes when its exact
  identity and current proof match. In separate runs, omit a count,
  change its value, or give a structurally valid count and
  scan result from another revision with the same component label and
  numeric count. Keep the correct manifest for the offered bytes. Each
  fault refuses. Reuse this case in 0005 IT-0005-RELEASE-APPROVAL.
- **Planted faults (PL-BI):** a release build that includes live reload; a
  harness that opens a port; each must be refused with its named message while
  an honest build passes.
- **Integration (IT-BI):** a generated project builds and passes its tests; a
  test container runs and is removed; two builds of one image give identical
  bytes on Linux and on Windows.
- **Fault injection (FI-BI):** a failed rebuild, a container that will not
  start, a crash during a test with a container running.
- **Behaviour (BD-BI):** the Dev scenario above, step by step.
- **Contract (CT-BI):** one test per parity item, written from the behaviour
  description by someone other than the implementer.
- **Measurement (MS-BI-01):** reload time from save to serving for one small
  crate, slice start time against full start time, resource index lookup cost
  and package build time on the 8 GiB reference. The numbers are set after the
  first run; until then only the one-tenth slice fraction is a target.

## Changes from the source

The pillar list described Testcontainers, Docker Compose, buildpacks and a
command-line wizard as built-in (all unverified wording). Behemoth Boot builds
them as optional developer tools, pinned and limited, kept out of release
builds and out of Sutradhar's own gates (ADR-0021: no Docker clutter for the
owner). Images and packages are built through Bazel rules (spec 0003) with no
container daemon. "Sub-second reload" is a proposed target, because Rust needs
a rebuild and restart. The doctor writes patches and does not rewrite files.
