# 0104: Python scripting and the sidecar

Python is how a user customises Sutradhar and how heavy analysis runs outside
the Rust engine. This specification owns the Python sidecar (the helper
process that hosts Python), the fenced workers that run scripts, the script
library with its history, the Python views of Alekh (formerly QuarryView),
custom buttons, extractors, hooks and link scorers, package installs, and the
way AI agents write scripts through MCP. It follows ADR-0021: Python never
touches live ranking or a reply to a site, a script only creates proposals,
and any PyPI package may be installed by the user with the owner's accepted
risk. Spec 0078 owns the memory numbers, spec 0099 the screens that run and
show jobs, spec 0020 the action list and MCP, spec 0037 the isolation of
third-party extensions, and spec 0043 the charts and tables that Python
views feed.

Example: Ravi wants a "Revenue at Risk" column in the Alekh table. He opens
"New view", picks a starter card and types three lines. The form for its two
settings (`min_words: int = 300`, `include_pdfs: bool = False`) is built from
the type hints. The script shows its needs as badges: "Read crawl data. No
internet." The first run uses a read-only snapshot of one million URLs,
streamed in chunks, and shows a speed bar. He saves it; the library records
version 2 with a readable diff. Later he asks an AI agent for a second view;
the agent writes it and its tests, the tests pass, and Ravi reads a plain
summary and approves before it goes live. At no point could either script
change a site or reach the internet.

Values marked "proposed" are design choices to be checked by measurement.

## What it does

### Words used here

- **Sidecar**: the first-party Python process that hosts workers and runs
  background analysis. One per installation.
- **Worker**: a fenced Python process or interpreter that runs one script at a
  time.
- **Script**: a Python file the user, an agent or a starter card provides,
  with a manifest.
- **Manifest**: the script's declared kind, settings, needs and tests.
- **Need**: one thing a script asks for, such as "Read crawl data".
- **Fence**: the limits on a worker: time, memory, folders, network and calls.
- **Snapshot**: a read-only, frozen copy of data handed to a worker.
- **Signal**: a plain number or tag a script saves for Rust to read.

### What runs where

| Runs in the sidecar | Never in Python |
|---|---|
| Optuna tuning, neural-network training, spaCy text work (ADR-0021) | Live ranking, any reply to a site |
| User scripts: Alekh views, buttons, extractors, hooks, link scorers | Writing to a site or its database |
| First-party helper code that is small, tested, pinned and within the limits | Anything outside the fence of its script |

A script returns data. Rust reads it, checks it and decides.

Ranking optimisation uses the registered Rust workflows in spec
0077 with the spec 0019 scheduler. The sidecar takes bounded immutable
inputs and returns package results and artefact references. Rust owns
the objective, checkpoints, lease, retries and publication fence.
A worker cannot enqueue or approve a transition or receive protected
evaluation labels. Stop or lease loss fences its output and invokes
the admitted process cleanup. Spec 0077 admits the exact package
bridge and resource profile before starting work.

### The sidecar and its workers

1. The sidecar uses one bundled, pinned Python (licence PSF-2.0, ADR-0021)
   inside the home folder. The user installs nothing and sees no terminal.
2. It sleeps when no job needs it and shows "Waking up" while it starts (spec
   0078 holds the budget of warm workers).
3. A small pool of fenced workers is kept warm so a script starts in under 100
   ms (proposed, measured). A pre-loaded toolkit (page parsers, tables, maths,
   pattern matching) is imported once when the sidecar starts and shared by
   workers.
4. Proposed, unproven: several isolated interpreters may share one process
   where the pinned Python supports separate interpreters with their own
   locks, so small hooks run side by side without extra memory. Where it does
   not, or where measurement (QR-028) shows no gain, a worker is a process.
   A worker is never shared by two scripts at the same time.
5. Proposed, unproven: large tables travel from Rust to Python through
   shared memory in a column format with no row copy or conversion, as a
   read-only view. The format and library are chosen and the claim is proved
   by measurement (QR-028, test IT-PY-01). If it fails, tables travel in chunks
   through a pipe.
6. Calls are batched: a hook that runs on 5,000 pages receives them in chunks,
   not 5,000 calls. A giant audit is streamed in chunks and never holds the
   whole table in memory.
7. A result cache remembers what a script returned for an unchanged page,
   keyed by the script version, its settings and the page hash. RS30
   additionally requires its site, target, package, admission and expiry
   checks.
8. Only the function a user edited is reloaded in a running worker. A reload
   that fails keeps the old version running.
9. Heavy maths may use a user-installed package that speeds up loops. We ship
   none and compile no C or C++ (ADR-0021).

### The fence

Every worker has a fence the script cannot widen.

| Limit | Rule |
|---|---|
| Time | A per-run time limit, `script.run_timeout_s` (proposed: default 60, range 1 to 900), checked by the sidecar, which stops the worker at its next safe point and, if it has not stopped after `script.grace_s` (proposed: default 5, range 1 to 30), ends the process |
| Memory | A share of the promise of spec 0078; a worker past its promise is stopped |
| Folders | Its own folder, the read-only snapshot and the exports folder only |
| Network | None by default; named hosts only if the manifest needs them and the user allows |
| Calls | A list of Sutradhar functions its needs allow |
| Processes | No new programs, no shells, no foreign libraries loaded from outside |

1. A script declares its needs in the manifest. The screen shows them as
   badges. Anything not declared is blocked at the fence, not just discouraged.
2. By default a script sees a protected, read-only snapshot of core data, so a
   buggy script cannot damage project history.
3. Writing goes through two doors only: the exports folder and the proposals
   list. Nothing else is writable.
4. The mechanism of the fence differs by system (QR-029): on Linux it may
   reuse the controls described in spec 0037 for extensions; on Windows it
   needs its own design, researched and tested before a script may run there.
5. A script that tries to cross the fence is stopped, the attempt is logged
   with its name, and the user is told in plain words.

### Packages

1. A user may install any PyPI package into one script's own environment with
   one click. Two scripts may need different versions; each has its own
   environment inside the home folder, so one never breaks another.
2. Only built packages (binary wheels) are installed. A package that would
   need a compiler is refused with a plain message, because we compile no C or
   C++ (ADR-0021). Before the install the screen shows the package, version,
   licence and file hash, and what the fence will still block. The install
   runs inside the fence, with network allowed only to the package index.
3. The owner accepted the supply-chain risk of arbitrary packages (risk
   register). We therefore add hash pinning, a record in the software bill of
   materials and a one-click removal.
4. Packages never enter the app's own bill of materials as bundled code; they
   are the user's own installs, listed separately.
5. Removing a script removes its environment and ledger entries (spec 0030).

### Script kinds

| Kind | What it does | Result |
|---|---|---|
| View column or formula | Adds a calculated column to an Alekh table | Values in the view |
| View lens | A saved multi-condition segment shown as a one-click tab | A row set |
| Grouper and pivot | Groups URLs into templates, pivots and roll-ups | A table |
| Colour rule | Decides row colours | Style hints |
| Chart | Returns a bar, scatter or trend chart above a table | A chart spec |
| Extractor | Pulls data from a page the crawler read | Rows |
| Hook | Runs on an event such as "after crawl" | Proposals or signals |
| Link scorer | Computes a custom score in the background | Signals saved as plain numbers |
| Button | Adds a button that runs a script on a selection | A proposal or export |
| Sidebar panel | A small panel such as a list or form | Display only |

Only an admitted link scorer can propose the named
`script_quality_rate` of spec 0072 through its background route.
It has no network or outside evidence grant. Hooks cannot emit
a ranking record or write one directly. Rust validates its range and
provenance under RS30 and consumes it only when the owner switch and
the admission record allow it. A script cannot set a weight, enter the
relevance gate or return a live ranking decision. Other script data is
not a ranking input. Leviathan's link scoring (spec 0082) is separate.

### The script library

1. Every script has a manifest: kind, version, settings with type hints, needs,
   sample inputs and tests.
2. Settings forms are built from the type hints: whole numbers become sliders
   with the declared range, booleans become toggles, text becomes a box.
3. Every save is a version in a local history with a readable diff and
   one-click restore. The history is automatic; the user needs no Git program
   and no terminal.
4. A script can be shared as a recipe file holding the script, columns, chart
   settings and manifest, with no data and no secrets.
5. Colour and a badge say whether a script is official (shipped with
   Sutradhar), the user's own or from an agent.
6. A folder can be watched, so the user may edit a script in an outside editor
   and see it reload; the same checks apply.
7. Starter cards (ready scripts for common jobs) are shipped; each is a script
   like any other, with tests.

### Writing and testing

1. **Editor.** A built-in editor with a plain-language error box and a live
   output preview on a sample.
2. **Console.** An interactive scratchpad inside the app window (not a
   terminal), fenced like any worker and connected to a read-only snapshot,
   for one-liners.
3. **Notebook mode.** A script can be shown as cells; the file stays a script.
4. **Macro recorder.** Click through filters and exports and get tidy,
   readable Python, not boilerplate.
5. **Profiler.** A millisecond breakdown of start-up, data transfer and work,
   and an optional line-by-line speed heat map using Python's low-overhead
   profiler.
6. **Replay tests.** Use the frozen synthetic fixtures and pinned expected
   output defined in N-1597. The separate N-1384 owner-approved one-item
   preview supplies no replay test evidence.
7. **Test before activation.** A script must pass its own tests before the
   sidecar activates it.
8. **One-click proposals.** A tweak shows "What this tweak does" in plain
   words every time live code is changed.

### AI agents writing scripts

1. An agent reaches scripts through MCP actions (spec 0020). It may read the
   exact data structures, columns and helper functions (a schema listing),
   so it need not guess names.
2. An agent-written script is a proposal. It must come with generated tests
   and pass them before it can be activated; a person then approves it after
   a plain-English review card with a diff and the needs it asks for.
3. An agent may propose extractor repair from already held eligible own-site
   page evidence. Verify the proposed selector on synthetic samples. It
   cannot fetch new remote evidence, reload or resume work on its own.
4. An agent may optimise a slow script from its speed heat map, with the same
   tests and approval.
5. An agent cannot widen a fence, install a package without the user's click
   or raise its own permissions.
6. Natural-language requests ("a button that exports pages with no internal
   links") become scripts through these rules, never directly into a site.

### Failure and recovery

The sidecar is supervised by the engine supervisor (spec 0030). A crashed
worker is replaced; a crashed sidecar restarts and replays only jobs that
checkpointed. A script that fails three times in a row is paused with its
last error and a one-click fix. A failing optional script never blocks the
product (a crash falls back to the last good version or to doing nothing).

#### Starter recipes, recovery and custom blocks (N-1442, N-1446 and N-1450)

Keep script and manifest versions under the existing library, file
ledger and worker fence. Use an eligible local profile with current
source and output rights. The Jobs screen consumes only the admitted
projection and its receipt under 0099.

Keep import and library selection inert. In a sealed synthetic test,
pin the script, manifest, broker and OS fence, fixture and result
digests, exact package versions, hashes and licences. Changes void
the proof. The test cannot download, install, use production
credentials, resolve a real send or produce a product effect.
Activation has independent current human approval. A dropped or
restored file stays inert.

| Feature | Exact form or recovery contract | Owning requirement |
|---|---|---|
| N-1442 One-click recipe library | Present the shipped or otherwise admitted recipe, exact version, declared needs, scope and local preview. Selecting it creates a reviewable draft and never installs a package, activates a job, starts a worker or widens rights. Check current symbolic references and required tests. Current owner review and the relevant 0104/0099 admission come before use. | PY-04, PY-08, PY-09, PY-10 · [FX-0104-W5-RECIPE-EXACT-DEPENDENCIES](../planning/feature-wave-5.md#acceptance-design-for-n-1430-to-n-1453) |
| N-1446 Silent sidecar self-repair | Hold automatic recovery until QR-086 admits the exact failed process, measured profile and 0030 supervisor budget. Existing update, rollback, Quit and portable removal still fence the action. Quit, stop, pause and recovery exhaustion hold restart or replay where applicable. A readiness outage alone cannot restart a healthy instance. Resume only an eligible committed checkpoint through 0019 with current authority and original effect identity. Unknown receipts hold. Show recovered only when both instance and recovery outcome are proved. | PY-12, PY-15, PY-20 and 0030/QR-086 · [FX-0104-W5-FENCED-SIDECAR-RECOVERY](../planning/feature-wave-5.md#acceptance-design-for-n-1430-to-n-1453) |
| N-1450 Drop-in custom job blocks | A versioned 0104 manifest with admitted ports, limits, needs, sample and tests may propose a local 0099 catalogue entry. Visibility and activation follow independent admission, current grants and owner review. Reject unsupported packages, script or output types and requests outside the fence. No directory arrival or detection executes, loads, installs or gives the graph native action authority. | PY-02, PY-03, PY-04, PY-08, PY-10, PY-17 · [FX-0104-W5-TYPED-CUSTOM-BLOCK-ADMISSION](../planning/feature-wave-5.md#acceptance-design-for-n-1430-to-n-1453) |

QR-201 qualifies exact manifest and preview profiles, process identity,
recovery and current fence proof on each admitted platform.

#### Script proof, recovery and presentations (N-1516–N-1525)

Pin site, script, source, manifest, packages, fence, settings and
fixture versions. Separate a draft, successful synthetic proof,
human approval, active version and recorded effect. Each changed
input invalidates the relevant earlier evidence. Run designed
fixtures only on bounded synthetic data with no credentials,
network, package installation, production write or grant access.
Use current host admission, current grants, safe cancellation,
scratch cleanup and the exact owning action for any later
activation. QR-204 qualifies these records.

| Feature | Exact proof, version or recovery rule |
|---|---|
| N-1516 Two-second sandbox test | Qualify two seconds as a target for the bounded synthetic quick check. Pin measured host profile and deadline. Timeout, isolation loss or incomplete checks yield a hold. A quick pass covers only those exact cases. It does not replace required tests and human approval. [FX-0104-W5-QUICK-SYNTHETIC-TEST-NO-PROOF-SUBSTITUTE](../planning/feature-wave-5.md#acceptance-design-for-n-1502-to-n-1525). |
| N-1517 Plain-English code diff card | Bind authorised base and candidate source digests, manifest and setting changes to a protected exact diff. Explain the change and supported effects in short plain English. Mask private paths, embedded secrets and ineligible hunks without claiming a complete review of withheld source. The explanation cannot approve or apply code. [FX-0104-W5-PROTECTED-EXACT-SCRIPT-DIFF](../planning/feature-wave-5.md#acceptance-design-for-n-1502-to-n-1525). |
| N-1518 Automatic script time machine | Retain immutable numbered script versions under existing history bounds. Pin source, manifest, settings and dependency identities and protect stored bytes. A restore offers a new candidate under current grants, fence, erasure and environment checks. Older approval, package access and activation do not survive the restore. [FX-0104-W5-IMMUTABLE-SCRIPT-HISTORY-CURRENT-RIGHTS](../planning/feature-wave-5.md#acceptance-design-for-n-1502-to-n-1525). |
| N-1519 Crash auto-fallback | A failed reload keeps the current admitted version. After a running script fails, use only the currently admitted last-good version for the same exact script and event, or no script. Use 0030/QR-086 and the 0019 checkpoint and effect receipt. Invalidate the old worker generation before a new start. Recheck current permissions, fence, caps and effect receipt. Unknown or partially committed effects cannot be retried under another version. Keep the existing failure streak pause. [FX-0104-W5-LAST-GOOD-SCRIPT-EXACT-EFFECT](../planning/feature-wave-5.md#acceptance-design-for-n-1502-to-n-1525). |
| N-1522 Fenced script permissions | Use the exact owner-admitted script and OS fence with allowlisted folders, host rules, process, memory and time caps. Admit only supported host-specific enforcement. Protect file identity across open and use. Deny unknown or unlisted needs. An approved test or script cannot change the fence, inherit network access or expose raw credentials. [FX-0104-W5-SCRIPT-EXACT-OS-FENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1502-to-n-1525). |
| N-1524 Side-by-side live comparison | Compare two pinned variants on the same eligible synthetic input in isolated runs. Pin expected output or error for each fixture and measure at the same host/profile, scope, units and boundaries. Missing oracle or unlike timings yields unknown. Show runtime, resource and result differences with gaps. Avoid timing claims from unlike conditions. No dual production run, live rank change, activation or reuse of prior approval. [FX-0104-W5-PINNED-SYNTHETIC-COMPARABLE-RUNS](../planning/feature-wave-5.md#acceptance-design-for-n-1502-to-n-1525). |
| N-1525 Auto-updated script cards | Keep a brief plain-English card per exact script proposal and manifest revision with current purpose, predicted scope, limits and tests. A source change marks the previous card stale. Use only authorised details and distinguish proposed effects from observed ones. The card is descriptive data and confers no execution permission. [FX-0104-W5-SCRIPT-CARD-CURRENT-EVIDENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1502-to-n-1525). |

Trace PY-02 to PY-05, PY-08 to PY-12, PY-15, PY-17 to PY-21
as applicable. Synthetic proof design is separate from release,
owner approval and current platform admission.

#### Script authoring, version changes and bounded execution (N-1526 to N-1550)

Use the existing script library, worker fence and local scheduler. Bind
script, manifest, package environment, settings, evidence and tests to
the exact site, actor, input or output scope and current grant. Saving a
draft, reviewing an import or seeing a preview does not activate a
script. Agent-written changes require tests and exact human approval.
Check authority at launch, input read, output release, replay and restore.
Expired or erased evidence withholds dependent results.

Bind one effect identity per item to the old and new manifest,
package hashes, settings, graph, grant and lease. Publish a new
generation atomically at its admitted boundary for new work only.
Use concurrent slots only under proved 0078 capacity. Otherwise
hold or drain. A failed or uncertain result cannot cause automatic
replay of either version. Zero interruption is a source label.
Version-specific evidence controls the actual transition.

A removable screen entry or form references the exact admitted
action and cannot change its admission. Bind script, environment,
host, target action and port, exact schedule occurrence and current
actor or service identity. At commit and dispatch recheck the
0019, 0099 and 0104 fences. Use one durable occurrence and effect.

| Feature | Exact script or worker contract | Existing requirements |
|---|---|---|
| N-1526 Live script stopwatch | Show eligible elapsed work, waiting, memory promise and measured use and their sampling age for the exact worker, run and script version. Pending or missing measures stay labelled. A last sample gives no new capacity or guarantee. This view neither starts a profiler nor changes a worker limit. | PY-12, PY-14 and 0078 MB-14 · [FX-0104-W5-EXACT-WORKER-MEASUREMENTS](../planning/feature-wave-5.md#acceptance-design-for-n-1526-to-n-1550) |
| N-1527 Zero-interruption swap | Accept only a tested and reviewed new version under the current activation fence. Existing eligible items stay pinned to the old version until a safe boundary. New items use one committed version and its exact environment. Revocation and stop outrank completion. Failure holds the candidate and retains the last still-eligible version. A restart cannot repeat an effect or blend generations. | PY-09, PY-10, PY-11 · [FX-0104-W5-PINNED-ENVIRONMENT-SWAP](../planning/feature-wave-5.md#acceptance-design-for-n-1526-to-n-1550) |
| N-1529 Official-vs-custom colour border | Derive source class from the verified shipped manifest or eligible user and agent records. Use text with colour. Unknown or altered origin stays unverified. Import or update cannot promote a script to official or overwrite eligible custom versions. | Script library, PY-09, PY-10 · [FX-0104-W5-VERIFIED-ORIGIN-LABEL](../planning/feature-wave-5.md#acceptance-design-for-n-1526-to-n-1550) |
| N-1530 One-click graduate to permanent tool | Prepare a named, removable local menu entry referencing the exact admitted script and input profile. Show needs and target to the owner. Grants, tests and activation remain separately checked. The entry cannot become a built-in capability, gain broader MCP discovery or dispatch native work. | PY-02, PY-04, PY-09, PY-10 · [FX-0104-W5-SCOPED-LOCAL-TOOL-ENTRY](../planning/feature-wave-5.md#acceptance-design-for-n-1526-to-n-1550) |
| N-1531 Automatic button builder | Map supported type hints and manifest needs to eligible typed controls and range checks. Show required fields, safe empty states and declared folder selection. Validate again at execution. The generated button is a local draft until admitted. It cannot convert unsupported Python or a hidden field into authority. | PY-04, PY-10, PY-13 · [FX-0104-W5-TYPED-BUTTON-DRAFT](../planning/feature-wave-5.md#acceptance-design-for-n-1526-to-n-1550) |
| N-1532 Plain-English trigger menus | Offer supported named local events and manual triggers from the typed 0099 catalogue. Bind exact script revision, allowed input, scope, occurrence identity and approval. Selecting a trigger creates a draft only. A protected store write step cannot gain an intercepting hook. No remote trigger, authority transfer or trigger from source text. | PY-02, PY-04, PY-10 and 0099 · [FX-0104-W5-REGISTERED-LOCAL-TRIGGER](../planning/feature-wave-5.md#acceptance-design-for-n-1526-to-n-1550) |
| N-1533 Friendly built-in script editor | Use the local editor with inert source text, scoped diagnostics and versioned save and diff. Use an isolated synthetic test by default. A one-item real preview requires its separately admitted route and limits. Editing and test success do not install packages, widen a fence or activate an agent change. | PY-03, PY-09, PY-10, PY-13 · [FX-0104-W5-INERT-EDITOR-SYNTHETIC-TEST](../planning/feature-wave-5.md#acceptance-design-for-n-1526-to-n-1550) |
| N-1534 Self-contained Python package store | Use only the existing owner-selected PyPI package step. Show exact compatible wheel and version, pin platform, ABI, hash and licence for every admitted dependency and retain the isolated ledger and environment. Review and preview stay offline. No implicit dependency install. Confirm any required licence review. Only the named package-index route is eligible during this operation. Missing or unbuilt wheel, altered hash or unadmitted dependency holds. No global install, compiler, agent consent or new package source. | PY-07, PY-08, PY-16, PY-23 · [FX-0104-W5-COMPATIBLE-PINNED-WHEEL](../planning/feature-wave-5.md#acceptance-design-for-n-1526-to-n-1550) |
| N-1535 Visual blocks next to real Python | Produce an editable local Python draft from admitted typed blocks and show the equivalent input, order, output and proposed operation. Treat unsupported blocks or ambiguous reverse mapping as unavailable. Code and labels cannot execute in the preview, expand needs or bypass tests and human review. Keep the existing macro recorder as its owning source for recorded actions. | PY-02, PY-04, PY-09, PY-10 · [FX-0104-W5-TYPED-BLOCK-CODE-PROPOSAL](../planning/feature-wave-5.md#acceptance-design-for-n-1526-to-n-1550) |
| N-1536 Ready-to-use Python starter cards | Show the exact shipped card, example, needs, package profile and tests. Duplicate only to a local draft. Storage, export and audit examples show scope and no effect in the synthetic test. Naming an S3 or drive workflow cannot give connection, posting, package or storage authority. | PY-04, PY-08, PY-09, PY-10 · [FX-0104-W5-INERT-PINNED-STARTER-CARD](../planning/feature-wave-5.md#acceptance-design-for-n-1526-to-n-1550) |
| N-1537 Mistake-proof input forms | Check types, bounds and eligible site-scoped destination references both in the form and at use. Keep errors attached to the exact safe field. No guessed path or bucket, credential echo, folder read or provider verification from typing. Changes after validation require a fresh check. | PY-04, PY-13, PY-21 · [FX-0104-W5-CURRENT-TYPED-DESTINATION](../planning/feature-wave-5.md#acceptance-design-for-n-1526-to-n-1550) |
| N-1538 Drag-and-drop script sharing | Export only an approved local version, script and eligible recipe metadata with exact final source and metadata scrubbing and no site data, secrets, effective grants or package credentials. A drop resolves no paths or URLs. Treat an import as inert untrusted text. Validate bounded safe names, declared needs and versions and show a local draft diff. Rebind each dependency and require current tests and review before any use. A drop cannot run code or install a package. [FX-0104-W5-SCRIPT-SHARING-INERT-EXACT-VERSION](../planning/feature-wave-5.md#acceptance-design-for-n-1526-to-n-1550). | PY-03, PY-04, PY-08, PY-09, PY-10, PY-21 |
| N-1539 Plain-English safety x-ray | Show declared needs, actually enforced boundaries and any observed sandbox result separately. An inspection cannot certify all future behaviour. Unknown or unresolved needs hold review. An imported statement cannot become a permission and a clean inspection cannot replace the enforced fence. [FX-0104-W5-SAFETY-VIEW-ENFORCED-LIMITS](../planning/feature-wave-5.md#acceptance-design-for-n-1526-to-n-1550). | PY-03, PY-04, PY-10, PY-18 |
| N-1540 Live output preview window | Render the exact bounded typed sample output for the pinned draft, preferably sealed synthetic input. Show current, pending, held or stale. Use a separately admitted real-item preview only with its existing scope and cap. Mask fields on the completed projection. No durable output, external request, filesystem write or production job. [FX-0104-W5-VALID-PREVIEW-MASKED-SAMPLE](../planning/feature-wave-5.md#acceptance-design-for-n-1526-to-n-1550). | PY-02, PY-03, PY-05, PY-10, PY-21 |
| N-1541 Point-and-click script scheduling | Use the 0019 and 0099 local schedule contract and a named eligible script and environment version. Confirm its scope, site clock, bounded work and precise permitted outputs through existing owner authority. Recheck at dispatch, including stop and changed grants. Use the original occurrence key on retry. A schedule cannot install, send, approve an action or enable native writes. [FX-0104-W5-SCRIPT-SCHEDULE-ONCE-CURRENT-GRANT](../planning/feature-wave-5.md#acceptance-design-for-n-1526-to-n-1550). | PY-02, PY-04, PY-10 and 0099 |
| N-1543 Built-in plain-English recipe book | Describe only the current admitted typed script kinds, capabilities and constraints. Copy safe examples without live paths, site data or keys. Examples are inert and demonstrate synthetic or separately admitted inputs, not checks of current storage or provider access. No documentation entry makes an action scriptable. [FX-0104-W5-RECIPE-EXAMPLE-INERT-LOCAL](../planning/feature-wave-5.md#acceptance-design-for-n-1526-to-n-1550). | PY-02, PY-04, PY-21 |
| N-1544 Automatic upgrade check | After signed admission of the exact new release, pin old and new package environment, manifest, complete synthetic tests and target platform. Check schema compatibility and effect fences separately. A fixture grants no automatic update or rollback. Then use the existing local job route for bounded synthetic compatibility tests against eligible installed script and environment revisions. Mark untested or incompatible scripts accordingly. Keep the previous still-compatible version or hold that script. An agent update is a new proposal with tests and exact review. No package retrieval, rewrite, activation or repair follows automatically. [FX-0104-W5-SIGNED-UPGRADE-EXACT-COMPATIBILITY](../planning/feature-wave-5.md#acceptance-design-for-n-1526-to-n-1550). | PY-08, PY-09, PY-10, PY-11, PY-15 and 0030 update trust |
| N-1545 Pre-loaded SEO toolkit | Warm only the pinned and admitted bundled library set within 0078 promises. Keep private state, user packages and site data outside the shared warm template. Check per-platform resource and isolation proofs. An unavailable warm profile falls back only to the admitted cold worker. No new fetch or startup guarantee. [FX-0104-W5-WARM-BUNDLED-CLEAN-PROFILE](../planning/feature-wave-5.md#acceptance-design-for-n-1526-to-n-1550). | PY-03, PY-07, PY-14, PY-17 |
| N-1546 Zero-copy table sharing | QR-028 must prove the exact transfer profile on each supported Linux or Windows target where it is offered, custody, read-only semantics, memory savings and absence of a row copy. Bind immutable projected bytes, schema, byte and offset bounds, recipient worker, run, lease, lifetime, exact site and erasure generation. No alias into mutable source rows. Tear down the mapping before reuse even after crash or expiry. No fallback copy following loss of rights. Compare whole-batch memory with the bounded pipe path on each exact target. Keep the bounded local chunk transfer when zero-copy is unavailable. Revoke access before reuse and apply 0118 to protected transfers and spill. No unrestricted shared handle or raw private row. [FX-0104-W5-ZERO-COPY-IMMUTABLE-LEASE-LIFETIME](../planning/feature-wave-5.md#acceptance-design-for-n-1526-to-n-1550). | PY-03, PY-05, PY-06, PY-17, PY-21 |
| N-1547 Isolated sub-workers inside one process | Treat this as an optional profile until QR-028 and QR-029 prove exact target OS fencing, per-worker teardown and cancellation, isolated package and native extension state and 0078 whole-process caps. Interpreter separation alone gives no proof. Stop an invalid shared process with no publishable output. A process arrangement cannot share mutable script state, secret material, or another script's lease. Use separate admitted workers if unproved, or hold when no compatible worker exists. [FX-0104-W5-OPTIONAL-WORKER-EQUIVALENT-ISOLATION](../planning/feature-wave-5.md#acceptance-design-for-n-1526-to-n-1550). | PY-03, PY-07, PY-12, PY-14, PY-17 |
| N-1548 Pre-computed scores that never touch live ranking | The fenced background script writes eligible advisory output. A separately admitted link scorer may propose only the named RS30 rate. Rust alone validates the QR-082 admission under 0072, 0027 and 0077, current default-off owner switch, frozen eligible guest evidence, confirmed numerator, positive denominator, finite value from 0 to 1, exact site, target and expiry. Missing, stale or off omits the optional component and preserves baseline order and weights. No substitute zero, calibration, candidate or training input or general score import. Nothing runs Python in the live ranking path or consumes unadmitted, stale or arbitrary scores. Relevance and existing weights remain protected. [FX-0104-W5-ADVISORY-SCORE-AND-RS30-FENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1526-to-n-1550). | PY-01, PY-02, PY-22 |
| N-1549 Smart file-watcher hot-reload | A permitted local watcher observes only an admitted source location and creates a pinned candidate for validation. Coalesce duplicate observations of the exact version. A replaced path, invalid source, changed dependency or failed test leaves the previous still-eligible version active. A qualified function-level reload must preserve the immutable old run and new dependency state. Otherwise use the existing admitted whole-version promotion. No file arrival alone executes source. [FX-0104-W5-WATCHER-TESTED-EXACT-VERSION](../planning/feature-wave-5.md#acceptance-design-for-n-1526-to-n-1550). | PY-03, PY-09, PY-10, PY-11 |
| N-1550 Batch-instead-of-one-by-one execution | Use admitted ordered bounded input chunks under the existing chunk and memory profile. Pin batch identity, input generation and version and use the existing checkpoint boundary. A replay cannot duplicate eligible committed output. Refuse oversize and mixed-site input before processing, and hold after grant loss or incompatible checkpoint. No unbounded materialisation or guarantee that all batches fit. | PY-03, PY-05, PY-06, PY-12, PY-17 · [FX-0104-W5-BATCH-PINNED-BOUNDED-IDENTITIES](../planning/feature-wave-5.md#acceptance-design-for-n-1526-to-n-1550) |

QR-205 qualifies exact manifests, schema and limits, synthetic test
profiles, atomic version changes, package custody and per-platform
optional worker proofs. This slice creates no new effect route.

#### Local performance, Alekh views and audit scripts (N-1551 to N-1574)

Use the bundled pinned CPython worker and the existing 0104 admission.
Bind site, actor, script and environment revision, declared needs,
producer scope, snapshot and erasure generation, resource lease and
allowed output. Treat source text, regular expressions, recipe files
and rendered labels as data. Only the current fenced worker may
process the admitted projection. Test profiles use sealed synthetic
inputs. Live observations are limited to evidence the owning producer
has already admitted for this purpose. No script gains a provider
credential, direct connection, editorial or native write, or a live
ranking decision. Test and review remain separate from activation.

| Feature | Exact local performance or view contract | Existing requirements |
|---|---|---|
| N-1551 Auto-compiled fast math lanes | Use an explicitly chosen and admitted prebuilt package operation under the existing package profile. Pin types, precision, supported platform and measured resource accounting. A tag cannot install, compile, load a foreign library or bypass the worker fence. An unsupported operation can use its already admitted ordinary path where equivalent and within limits. | PY-03, PY-07, PY-08, PY-12, PY-17, PY-23 · [FX-0104-W5-PREBUILT-MATH-EXACT-PACKAGE](../planning/feature-wave-5.md#acceptance-design-for-n-1551-to-n-1574) |
| N-1552 Idle worker hibernation | Keep only the admitted warm set under 0078 and release eligible idle workers at safe boundaries. Pending receipts and active version leases retain their own recovery rules. A wake needs a fresh admitted resource promise. Never count a suspended or measured-stale worker as ready capacity. Reuse N-1390 and the existing lifecycle. | PY-12, PY-14, PY-15 · [FX-0104-W5-IDLE-WORKER-NEW-WAKE-PROMISE](../planning/feature-wave-5.md#acceptance-design-for-n-1551-to-n-1574) |
| N-1553 Instant result caching | Reuse only eligible deterministic results from the same site, data and source revision, input, script and environment version, settings and allowed output profile. Bind grant, retention and erasure generation and recheck them before a hit. A cached result cannot repeat an effect, inherit approval or supply a missing current result. | PY-05, PY-19, PY-21, PY-22 · [FX-0104-W5-EXACT-CACHE-CURRENT-GRANT](../planning/feature-wave-5.md#acceptance-design-for-n-1551-to-n-1574) |
| N-1554 Live-ranking safety wall | Keep Python outside retrieval gates, live ranking, signed site replies and protected release decisions. Scripts emit typed local reports, tags or eligible candidate outputs. The separately admitted RS30 rate uses only the 0072 and 0027 Rust checks and owner switch. Other computed fields or scores cannot enter ranking. | PY-01, PY-02, PY-22 · [FX-0104-W5-RUST-ONLY-RANKING-BOUNDARY](../planning/feature-wave-5.md#acceptance-design-for-n-1551-to-n-1574) |
| N-1555 Streaming row pipeline | Read a pinned eligible source through bounded ordered chunks under PY-06. Keep memory promises for held and copied rows and stop on expired lease or changed scope. Close or reconcile the original checkpoint across interruption. A repeated chunk cannot duplicate released output. Use the same batch contract as N-1550. | PY-03, PY-05, PY-06, PY-12 · [FX-0104-W5-ORDERED-BOUNDED-STREAM](../planning/feature-wave-5.md#acceptance-design-for-n-1551-to-n-1574) |
| N-1556 One-second startup profiler | Time the admitted startup, load, input and script phases separately on the exact worker, model of measurement and warm or cold state. Show clock precision and missing phases. One second is a target to qualify. Mask private paths and inputs or output. Profiling cannot run a job or widen its resource promise. | PY-12, PY-14, PY-17, PY-21 · [FX-0104-W5-MEASURED-STARTUP-PHASES](../planning/feature-wave-5.md#acceptance-design-for-n-1551-to-n-1574) |
| N-1557 Live calculated table columns | Bind a reviewed typed formula and eligible input columns to the exact Alekh view revision and derived column type and units. Show missing or incompatible inputs and keep originals. Apply row and output limits. The column changes no source field and is not an order-stage ranking input. | PY-02, PY-03, PY-05, PY-13 · [FX-0104-W5-DERIVED-COLUMN-TYPED-VIEW](../planning/feature-wave-5.md#acceptance-design-for-n-1551-to-n-1574) |
| N-1558 Reusable Python view lenses | Store the named script version, filter shape, eligible input fields and scope as a local view reference. Recheck producer grants and suppression before each use and after restore. One filter cannot expose hidden rows, cross-site references, site data from another owner or underlying protected counts. | PY-03, PY-04, PY-05, PY-09, PY-21 · [FX-0104-W5-VIEW-LENS-CURRENT-SAFE-PROJECTION](../planning/feature-wave-5.md#acceptance-design-for-n-1551-to-n-1574) |
| N-1559 Cross-table Python joins | Join only authorised same-site typed projections with proved compatible page or other explicitly admitted keys, source revisions, units, grain and windows. Keep provider, native and protected joint measures distinct. Missing or ambiguous keys hold a combined result. Never join anonymous data to account records or reconstruct a protected row or count. Reuse 0034, 0040 and 0043 producer scope. | PY-03, PY-05, PY-06, PY-21 · [FX-0104-W5-JOIN-EXACT-COMPATIBLE-OWNED-KEYS](../planning/feature-wave-5.md#acceptance-design-for-n-1551-to-n-1574) |
| N-1560 Instant pivot and rollup scripts | Use the exact admitted dimensions, compatible measures, eligible population, intervals and denominator. Hold overlapping totals, mixed sources or unsafe small groups under the 0034 and 0043 rules. Materialise a bounded local view with coverage; a missing group cannot count as zero. | PY-03, PY-05, PY-06, PY-12 · [FX-0104-W5-PIVOT-DISJOINT-DISCLOSED-GROUPS](../planning/feature-wave-5.md#acceptance-design-for-n-1551-to-n-1574) |
| N-1561 Custom conditional colour rules | Return bounded typed style hints for eligible values in an admitted SDS rendering profile. Keep equivalent text, meaning and accessible contrast. Styling cannot hide warnings or missing values, expose a protected cell, inject markup or change the measure. | PY-02, PY-03, PY-13 · [FX-0104-W5-CONDITIONAL-STYLE-EQUIVALENT-TEXT](../planning/feature-wave-5.md#acceptance-design-for-n-1551-to-n-1574) |
| N-1562 Live regex tester on real crawl columns | Use a qualified bounded pattern and eligible same-site field profile. Default to synthetic examples. An expressly admitted real preview uses a bounded protected snapshot and exact pattern, row and time caps. Invalid or over-budget matches hold safely. No new crawl, raw query or hidden field. | PY-03, PY-05, PY-12, PY-13 · [FX-0104-W5-BOUNDED-TYPED-REGEX-PREVIEW](../planning/feature-wave-5.md#acceptance-design-for-n-1551-to-n-1574) |
| N-1563 Smart URL pattern grouping | Group eligible normalised same-site URLs using a versioned reviewed rule and safe labels. Keep ambiguous or uncovered cases separate. No private query text, credentials or unproved aliases may reach the group or display. Grouping cannot rewrite a URL, canonical, rank or crawl frontier. | PY-02, PY-04, PY-05, PY-21 · [FX-0104-W5-SAFE-CANONICAL-URL-GROUPING](../planning/feature-wave-5.md#acceptance-design-for-n-1551-to-n-1574) |
| N-1564 Time-series diff columns | Compare exact stored revisions and compatible closed windows, units, scope and field semantics. Retain source coverage, polarity and unknowns under 0043. A renamed page or missing historical mapping holds the affected comparison. No inferred date or live rank check. | PY-05, PY-06, PY-19 · [FX-0104-W5-COMPATIBLE-CLOSED-SNAPSHOT-DIFF](../planning/feature-wave-5.md#acceptance-design-for-n-1551-to-n-1574) |
| N-1565 Custom export shapers | Shape only authorised output fields within the admitted local recipe and file type. Recheck recipient, field provenance, disclosure and bounds on the final protected 0100/0118 file. Sanitise formula-like values and unsafe strings under the format rules. A shaping script cannot restore withheld fields, split a protected count or create executable workbook objects, delivery or cloud upload. | PY-02, PY-03, PY-04, PY-21 · [FX-0104-W5-PROTECTED-DISCLOSURE-SAFE-EXPORT](../planning/feature-wave-5.md#acceptance-design-for-n-1551-to-n-1574) |
| N-1566 Saved Python mini-charts | Return bounded inert marked values and a typed chart description with unit, source, coverage, data alternative and accessible labels. The admitted 0043 view renders it. Preserve masking and suppression in every form. Reject script, executable link and remote asset requests. | PY-02, PY-03, PY-05, PY-13 · [FX-0104-W5-INERT-CHART-DATA-ALTERNATIVE](../planning/feature-wave-5.md#acceptance-design-for-n-1551-to-n-1574) |
| N-1567 Shareable QuarryView recipe files | Treat the historical QuarryView label as Alekh. Reuse the reviewable local recipe file contract for source, column, view and chart metadata. Keep all site data, credentials, paths and live grants out. Import only an inert version. Rebind dependencies, run current approved tests and require separate activation. A file does not run on drop. | PY-03, PY-04, PY-08, PY-09, PY-10, PY-21 · [FX-0104-W5-INERT-ALEKH-RECIPE-TRANSFER](../planning/feature-wave-5.md#acceptance-design-for-n-1551-to-n-1574) |

| Feature | Exact audit scope | Existing requirements |
|---|---|---|
| N-1568 Strict schema and JSON-LD validators | Extend the eligible local 0044 structured-data facts, exact visible page revision and admitted rule profiles with reviewed advisory checks. Parse bounded typed projections and distinguish malformed, inapplicable, unproved and failed observations. Use 0115 exact guest visibility where required. Keep syntax and vocabulary separate. Hold remote JSON-LD context, recursive entities and unapproved grammar without remote resolution or raw input echo. No script execution, external validator, markup change or indexing guarantee. | PY-02, PY-03, PY-05 and 0044 feature 1316 · [FX-0104-W5-BOUNDED-STRUCTURED-DATA-VALIDATION](../planning/feature-wave-5.md#acceptance-design-for-n-1551-to-n-1574) |
| N-1569 Custom hreflang matrix checker | Use only feature 564's admitted read-only observations of existing same-site declarations under 0047. Pin observed declaration and current guard versions and window. Foreign return targets are never fetched. An unresolved return path, outside host or expired scope stays unproved. DR-081 keeps multilingual placement, market expansion, changes, regeneration and cross-domain acquisition outside this route. | PY-02, PY-03, PY-05 and 0047 feature 564 · [FX-0104-W5-ADMITTED-HREFLANG-RETURN-SCOPE](../planning/feature-wave-5.md#acceptance-design-for-n-1551-to-n-1574) |
| N-1570 Pagination and faceted-navigation trap detector | Use approved bounded 0044 and 0082 stored path, link and coverage evidence to propose a scoped diagnostic. Pin the exact pattern and observed route revision. A suspected loop, weak boundary or incomplete crawl cannot prove the entire space was sampled. No probe, new crawl, robots edit or automatic frontier change. | PY-02, PY-03, PY-05, PY-12 · [FX-0104-W5-STORED-PATH-TRAP-DIAGNOSTIC](../planning/feature-wave-5.md#acceptance-design-for-n-1551-to-n-1574) |
| N-1571 Custom Core Web Vitals root-cause scripts | Use qualified 0034 real-user aggregates or separately admitted 0115 or supplied trace projections under the owning profile. Pin origin, capture, viewport, page and trace versions. Bind rendering mode, archived profile, device, origin, page, capture date and trace versions. Field aggregates keep exact consent and aggregation. Label 0115 approximations simulated. Missing interaction timing stays unknown. Separate lab evidence, observed aggregates and hypotheses. Unknown protected detail remains unavailable. No trace collection, new provider or unsupported causal verdict. | PY-02, PY-03, PY-05, PY-21 · [FX-0104-W5-SCOPED-CWV-EVIDENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1551-to-n-1574) |
| N-1572 Redirect chain and migration checker | Use eligible own-site observed chain facts or the exact DR-110 scoped staging export under 0044 and 0047. For signed owner-supplied staging scope bind active site, approved staging project, old-to-new mapping, exact allowed sections, digest, grant/expiry and extraction time, plus comparable live audit revision. Scrub before disk or spill. Treat supplied addresses as inert data. No settings or authentication hand-off. A missing, stale or out-of-scope hop cannot prove arrival. No staging log-in, arbitrary fetch, redirect change or repair. | PY-02, PY-03, PY-05 and REQ-0044-STAGING (test BDD-0044-STAGING in spec 0044) · [FX-0104-W5-EXACT-CHAIN-STAGING-COMPARISON](../planning/feature-wave-5.md#acceptance-design-for-n-1551-to-n-1574) |
| N-1573 AI-search and answer-engine readiness scorer | Use eligible own-site feature and document observations to produce versioned advisory findings with evidence and coverage. Distinguish content properties from measured AI visibility. Existing supplied answer analysis remains under 0093 and DR-086. If the advisory view cites a supplied observation, attribute a page citation only to the exact eligible canonical page in the 0093 evidence. A brand mention or citation of a sibling page does not prove a citation of the page under review. Incomplete citation coverage cannot prove its absence. This Python route has no live query, spend, monitoring, factual-answer promotion or ranking effect. Separate approved DR-118/0133 Coach calls grant no broker access here. | PY-01, PY-02, PY-05, PY-22 and 0093 · [FX-0104-W5-ADVISORY-AI-READINESS-EVIDENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1551-to-n-1574) |
| N-1574 Log-file crawl-waste analyser | Use only current approved 0044 and 0034 log projections scrubbed before Python. Bind site, mapping, exact policy and closed source window. Suppress revealing joined aggregates across releases and restore. Separate verified bots, claimed or unknown identities, status, workload and admissible page groups. Report covered intervals and source denominators. No raw addresses or access logs, no identity or conversion join, and no control of crawler visits or site rules. [FX-0104-W5-PROTECTED-CRAWL-LOG-AGGREGATES](../planning/feature-wave-5.md#acceptance-design-for-n-1551-to-n-1574). | PY-02, PY-03, PY-05, PY-21 and QR-088 |

Use only the exact compatible prebuilt wheel and its pinned transitive
hashes, licences, ABI, OS and broker profile and admitted numeric
precision. Hold JIT, AOT, source builds or implicit installation in
N-1551. An ordinary fallback keeps the same limits and meaning.
Pin cache provenance and current erasure generation before reuse.
The Rust-owned RS30 path stays default-off under 0072. Missing, off
or invalid keeps the baseline order and weights without a zero score.

For table joins and rollups scope and bound at the authorised producer
before worker materialisation and again at release. Bind grain, exact
keys, cardinality, closed windows and compatible units. Hold unproved
fan-out and hidden totals inferred from across views or files.

For 1565 to 1567 bind exact active site, source snapshot, recipient,
profile and rendered bytes. Parsing and shaping stay inert. Hold
formulas, macros, active objects, external relationships and hidden
metadata. Keep quote and credit constraints and joint disclosure.
Decode and validate the final protected file before one guarded
release. Remove scratch after failure, expiry or revoked grant.

QR-206 qualifies exact package, cache, schedule, output, comparison and
audit profiles, including bounded replay and erasure. These script
forms give no new collection, ranking, transport or approval route.

#### Scripted evidence, editorial checks and worker controls (N-1575–N-1598)

Run these as declared 0104 capabilities. Pin the site, script, package,
manifest, settings, source revision, approved fields, effect class and
current worker lease. Use immutable eligible input through its owning
specification and keep provenance, freshness, bounds and coverage in
typed outputs. An absent source, partial cover or unproved comparison
yields an explicit gap. Before accepting or exposing an output, Rust
checks its schema, current rights, lease, erasure and applicable
owning rules. Expired work cannot hand back results or retain scratch.
Use 0019, 0078 and 0118 for bounded recovery, isolation and custody.
No script creates a fetch, standing watch, native change, send,
publish action or new live ranking input by returning data.

| Owner catalogue entry | Exact evidence, effect and limit |
| --- | --- |
| N-1575 Visual-layout change alerts | Compare eligible own-site screenshots of the same page and viewport profile from 0115 or an admitted supplied source. Record viewport, capture mode, version, mask and comparison coverage. Check shifts in the main content and supported mobile navigation states. A comparable change is a local review finding under the owning alert policy. A mask, pending asset or capture mismatch gives a gap. No new render, visits or outside notices. [FX-0104-W5-COMPARABLE-MASKED-LAYOUT-PAIR](../planning/feature-wave-5.md#acceptance-design-for-n-1575-to-n-1598). |
| N-1576 Custom security and compliance scanners | Flag evidence of broken outbound links, mixed-content references, staging-domain references and absent expected privacy UI in current admitted 0044/0115 observations. Distinguish detected, absent within complete comparable observed scope and unknown. Missing or partial privacy UI coverage stays unknown. Store safe redacted findings with source coverage. No live security probes, new fetches, exploit tests, private response disclosure or legal compliance certification. [FX-0104-W5-SECURITY-FINDINGS-OBSERVED-SCOPE](../planning/feature-wave-5.md#acceptance-design-for-n-1575-to-n-1598). |
| N-1577 Staging-to-production pre-flight suite | Compare approved, separate snapshots of the intended release against a matching baseline and explicit expected page set. Evaluate indexing directives and title templates under 0044, including conflicts and missing pages. Bind the exact DR-110 owner-approved local staging file to active site, project, mapping and retained scope before open. Use the signed matching live audit on the production side where required. Recheck erasure and decline stale or mismatched scope. No new audit trigger or sign-in. A pass applies only to compared evidence and does not release or deploy a site. [FX-0104-W5-SUPPLIED-STAGING-MATCHED-SCOPE](../planning/feature-wave-5.md#acceptance-design-for-n-1575-to-n-1598). |
| N-1578 Topic-cluster and semantic-gap scripts | Propose local meaning-based clusters and candidate gaps from comparable eligible records under 0092. Pin the admitted analysis profile, model and language and retain evidence for thin or competing coverage. Sparse pages and ambiguous topics stay uncertain. No inferred search demand, rival watch or new scoring signal. [FX-0104-W5-SEMANTIC-GAP-CURRENT-LOCAL-CORPUS](../planning/feature-wave-5.md#acceptance-design-for-n-1575-to-n-1598). |
| N-1579 Custom internal-link candidate scorers | Restrict experimental candidate comparisons to isolated 0104 data and typed review evidence. For shipped order, only the separately admitted background `script_quality_rate` under RS30 and 0072 may be consumed by Rust, using N-1548 and RM06 with the current default-off owner switch. Missing, off, stale or invalid omits the component. Preserve frozen pre-RS30 order and weights without substitute zero or weight renormalisation. Keep semantic eligibility and the current rank profile. Commercial priorities and conversion values provide no independent script input to ordering, live eligibility or rank weights. [FX-0104-W5-EXACT-RS30-ADMISSION-BASELINE](../planning/feature-wave-5.md#acceptance-design-for-n-1575-to-n-1598). |
| N-1580 Anchor-text diversity guardrails | Use a scoped 0027 graph snapshot and eligible comparable anchors. Explain observed concentration, counted placements and coverage. Propose exact wording alternatives under 0041 with meaning and destination evidence. Omit private text and count uncertainty. No rewriting, insertion or ranking effect. [FX-0104-W5-ANCHOR-CONCENTRATION-SCOPED-ADVICE](../planning/feature-wave-5.md#acceptance-design-for-n-1575-to-n-1598). |
| N-1581 Orphan-page rescue matcher | Declare the bounded measured graph and apparent orphan status in that scope. Unmeasured paths stay unknown. Produce at most five sourced related candidates when evidence supports them. Include existing typed relevance and authority evidence. Search potential does not qualify an off-topic source. Pass proposals to the existing candidate review path with no placement or new search demand measurement. [FX-0104-W5-MEASURED-ORPHAN-ELIGIBLE-PROPOSALS](../planning/feature-wave-5.md#acceptance-design-for-n-1575-to-n-1598). |
| N-1582 Link-placement heatmap rules | Classify verified placements as editorial body, navigation, footer, sidebar or unknown using admitted 0044/0115 evidence. Retain page, placement and origin scope, respect hidden and protected regions and provide equivalent accessible tabulation. Keep Python classes advisory or in approved local files only, under PY-02/PY-22 and 0072. Native Rust placement producers retain separate authority. The candidate set, weights and order stay invariant under script output. [FX-0104-W5-PLACEMENT-PROJECTION-RANK-INVARIANCE](../planning/feature-wave-5.md#acceptance-design-for-n-1575-to-n-1598). |
| N-1583 Content decay early-warning scripts | Compare aligned closed own-site performance windows and verified publication or revision dates under 0040/0043. Optional rival freshness may use only dated evidence already admitted to 0092 under current rights. Show contrary evidence and coverage. A decline finding is a review cue. No predicted rank collapse, new source calls or repeated rival checks. [FX-0104-W5-DECLINE-COMPARABLE-CLOSED-WINDOWS](../planning/feature-wave-5.md#acceptance-design-for-n-1575-to-n-1598). |
| N-1584 Custom readability and tone checkers | Return issue spans and short suggestions for an eligible revision from a versioned local style profile. Check clarity, jargon, prohibited terms and required product mentions only where the language and evidence profile supports them. Use 0041/0122 for checks and review. No silent edits, copied provider material or claims about unsupported languages. [FX-0104-W5-VERSIONED-LOCAL-STYLE-SPANS](../planning/feature-wave-5.md#acceptance-design-for-n-1575-to-n-1598). |
| N-1585 Keyword cannibalisation resolver | Group sites' pages separately using exact within-site query and landing-page evidence under 0040/0092. Pin query scope, dates and source coverage. Suggest keep, merge or redirect for human review where evidence supports each consequence. Query omission or partial coverage is unknown. Create no redirect, canonical or content change. [FX-0104-W5-CANNIBALISATION-COMPLETE-SITE-PROJECTION](../planning/feature-wave-5.md#acceptance-design-for-n-1575-to-n-1598). |
| N-1586 Entity coverage and fact-completeness checks | Compare eligible page text against admitted, dated references and distinguish uncovered topic, disputed assertion and insufficient support. Record source role, permission, locale and exact evidence. Reference ranking is stated only with matching supplied proof. Use 0041/0092 review and no new external fetch, fact invention or publication. [FX-0104-W5-FACT-COVERAGE-CURRENT-CITATION](../planning/feature-wave-5.md#acceptance-design-for-n-1575-to-n-1598). |
| N-1587 What-if link-graph simulator | Let a script propose an exact typed graph delta and render the eligible result from the existing 0027 feature 1 batch simulator with 0066 job 251 advisory entry. Preserve the pinned profile, graph scope, revision, numerical tolerances, limits and review. Graph reorganisation and evidence stay in a disjoint advisory projection. A result describes only the model. No candidate publisher, 0077 promotion, ranking forecast or persistent live graph follows. The script cannot run its own live ranking, operate on an unknown graph or change links or templates. [FX-0104-W5-GRAPH-DELTA-ADVISORY-SIMULATION](../planning/feature-wave-5.md#acceptance-design-for-n-1575-to-n-1598). |
| N-1588 Safe white-hat compliance gates | Evaluate a current suggestion against declared editorial, link-count and partner-policy evidence. Report advisory violation, no match in the checked scope or unknown. Any binding block or native refusal remains with the existing admitted Rust or native action and rule. No Python rejection, approval, partner exception or external disclosure. [FX-0104-W5-COMPLIANCE-ADVISORY-NATIVE-GATE-SEPARATE](../planning/feature-wave-5.md#acceptance-design-for-n-1575-to-n-1598). |
| N-1589 Automated content-brief generator | Build a bounded local 0041/0092 brief draft from approved queries, heading summaries, eligible link targets and sourced entity gaps. Record revision, coverage and rights, with no copied source wording or title inference. Give reviewable citations and safe unknowns. No outbound prompt, fresh lookup, publishing or automatic insertion. [FX-0104-W5-LOCAL-BRIEF-CURRENT-SOURCE-RIGHTS](../planning/feature-wave-5.md#acceptance-design-for-n-1575-to-n-1598). |
| N-1590 Capability permission badges | Present declared needs, currently effective scope and denials separately from the manifest and actual broker and OS fence. Show folder, data, export, network and relevant limits in plain words. An unchecked or unrecognised need stays denied. Update unread or stale badges on settings and permission changes. Badge display cannot admit a need. [FX-0104-W5-PERMISSION-BADGES-CURRENT-DECLARED-AND-EFFECTIVE](../planning/feature-wave-5.md#acceptance-design-for-n-1575-to-n-1598). |
| N-1591 Read-only live snapshots | Give the worker a bounded current-generation frozen view of eligible rows and fields, never a live database handle. Bind site, purpose, source revision, field roles, expiry and worker lease. Check protection and erasure before every chunk and fence cached results on changes. Row writes, arbitrary reads, another site's rows and secrets are unavailable. [FX-0104-W5-SNAPSHOT-FROZEN-SCOPE-FENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1575-to-n-1598). |
| N-1592 Zero-overhead live speed heatmaps | Offer opt-in admitted line profiling in a separate bounded diagnostic run with matching input, versions and calibration. Use a baseline to measure and report overhead, byte limits, coverage and unavailable line mapping. Keep source and values within their owner grants and show accessible equivalent data. No zero-overhead claim or mandatory hot-path instrumentation. [FX-0104-W5-SEPARATE-PROFILED-OVERHEAD](../planning/feature-wave-5.md#acceptance-design-for-n-1575-to-n-1598). |
| N-1593 Safe template strings for queries and HTML | Bind values through the owning typed read interface, which selects admitted queries and parameters. Output HTML-like snippets as inert text or an admitted sanitised presentation shape with context-specific escaping. Mark prompt text as untrusted data. Check schema, escaping, limits and sink before delivery. Reject raw SQL, executable markup, unsupported contexts and prompt authority changes. [FX-0104-W5-TYPED-TEMPLATE-APPROVED-SINK](../planning/feature-wave-5.md#acceptance-design-for-n-1575-to-n-1598). |
| N-1594 Shadow-mode staging before going live | Pair incumbent and candidate only on the same sealed, bounded, synthetic fixture with versioned expected outputs, resource envelope and loss-aware comparison. Sidecar gives neither run production callbacks, real credentials, queue effects, rank input or publishing. A pass produces isolated evidence for independent approval. A diff, timeout or escaped fence holds. [FX-0104-W5-SEALED-SYNTHETIC-SHADOW-COMPARISON](../planning/feature-wave-5.md#acceptance-design-for-n-1575-to-n-1598). |
| N-1595 Automatic rollback on crash | Reuse N-1519, 0030 and exact QR-086 restart admission. Stop, pause, Quit and revocation fence recovery at actual start. Persist exact failed version, worker, job identity and observed failure, fence late results, and choose a currently eligible last-good version or no script under the existing failure policy. Preserve PY-20 pause and unchanged effect identity. Hold retry, fallback and new effects until ambiguous original intent and receipt reconcile. A different version cannot repeat them. [FX-0104-W5-RECOVERY-EXACT-EFFECT-FENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1575-to-n-1598). |
| N-1596 Isolated per-script environments | Pin the exact script, approved environment version, wheels, hashes, licence and dependencies within the registered home. Retain 0104 package installation, compiler refusal and owner click. Separate private packages and writes across scripts. Use a proved isolation profile or hold. Restore, switching and cleanup cannot share grants, widen the fence or remove a needed environment. [FX-0104-W5-PINNED-ISOLATED-SCRIPT-ENVIRONMENTS](../planning/feature-wave-5.md#acceptance-design-for-n-1575-to-n-1598). |
| N-1597 Deterministic replay testing | Compare an exact synthetic frozen set of 100 page fixtures with a bounded proof profile. Pin clock, seed, order, script, settings, wheels, fixture and expected output digests. Compare the owning canonical result or declare a supported tolerance separately. Repeat runs with network off and no durable effects. Reject a substituted real page or one-item preview result before dispatch, even if its projection matches the expected values. Differences, partial replay and unsupported determinism remain visible. [FX-0104-W5-EXACT-DETERMINISTIC-SYNTHETIC-REPLAY](../planning/feature-wave-5.md#acceptance-design-for-n-1575-to-n-1598). |
| N-1598 Outbound domain allowlists | Retain network-off by default. Intersect the declared host need with exact current owner approval and the owning connector's site, operation, destination and data scope and the active DR-079 outbound level. A host grants none of those permissions and cannot enable a broader level or site-text switch. No credential or scope change from the script. Mediate delivery through the admitted native fence and connector. Recheck resolved identity and any redirect before disclosure. Unproved routes, raw networking and implicit host or payload widening hold. A listed host gives no general egress, paid lookup or new API grant. [FX-0104-W5-BROKERED-EGRESS-SCOPE-FENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1575-to-n-1598) |

QR-207 qualifies exact same-site evidence, schema and rights, measured
limits, stale or revoked results and the safe handling of unavailable
sources. Keep worker test suites synthetic and bounded. Existing local
output and export gates apply independently of successful script proof.

#### Script previews, authoring and local review (N-1599–N-1622)

Spec 0104 owns these script forms. Use the existing admitted
worker, script kinds, calls and file paths. Pin the site, actor,
purpose, current grant, script and manifest revisions, settings,
package hashes, host fence, input and output profiles and fixture.
Read only eligible projected data through the owning API.
Treat source, file contents, print output and model text as inert
data. A descriptor, example, result or signed script pack cannot
add authority. Keep proposed, tested, reviewed and active
versions distinct. Recheck current rights, erasure, job fence and
scope before result or hand-off, retry and restore.

All acceptance fixtures for this slice are synthetic. Use bounded
sealed scratch with no production records, credential handles,
network, live page capture, package installation or durable
effects. Destroy the scratch after completion, stop or recovery.
The production worker routes keep their separately admitted
permissions and ceilings. A hypothetical effect remains a proposal.
Publishing, native writes, connector changes, spending and
ranking decisions stay with their existing owners.

| Feature | Exact local contract | Existing requirements |
|---|---|---|
| N-1599 Instant dry-run diff preview | Pin the exact script, scoped revision and requested input. Show only admitted proposed effects with typed target, before and after when eligible, and safe unresolved or refused reasons. The bounded synthetic preview cannot mark an action ready, write tags or queues, reserve quota or dispatch a webhook. | PY-02, PY-05, PY-10; 0099 JW-05 · [FX-0104-W5-INERT-DRY-RUN-EFFECT-DIFF](../planning/feature-wave-5.md#acceptance-design-for-n-1599-to-n-1622) |
| N-1600 Secret-free credential vault | Keep secret values in 0065. An admitted script declares an exact brokered action need. The approved owning route resolves a short-lived opaque handle bound to site, account, operation, destination, exact script and manifest, worker lease, grant, request schema and payload and response scopes. Resolve only inside the broker. Recheck at resolution, dispatch and delivery for expiry, scope and erasure. Keep prior egress, spend and uncertain-receipt checks. Expose no handle in chain, output or error. No general API token, raw value, credential lookup or new host follows from an alias in source. Synthetic tests use no real handles. | PY-03, PY-04, PY-18, PY-21 · [FX-0104-W5-EXACT-BROKER-HANDLE-SCOPE](../planning/feature-wave-5.md#acceptance-design-for-n-1599-to-n-1622) |
| N-1601 Automatic memory leak janitor | Use recorded resource pressure and exact worker generation under 0078 and the 0030 supervisor. Retire an eligible idle worker only after its work, leases and returns are reconciled. Keep state and unknown outcomes in the owning durable record. A suspected leak cannot restart protected work, extend a cap or retry an uncertain effect. | PY-12, PY-14, PY-15; 0078 MB-15 · [FX-0104-W5-RECONCILED-IDLE-WORKER-RETIREMENT](../planning/feature-wave-5.md#acceptance-design-for-n-1599-to-n-1622) |
| N-1602 Live sidecar scratchpad | An owner-started local session gets a bounded fenced scratch worker and the current permitted immutable snapshot. Bind scope and output profile per request. No shell, live database, inherited handles, cross-site input, save, scheduling or native write. Clear scratch and process authority at exit or stop. | PY-02 to PY-05, PY-12, PY-17 · [FX-0104-W5-FENCED-SNAPSHOT-SCRATCHPAD](../planning/feature-wave-5.md#acceptance-design-for-n-1599-to-n-1622) |
| N-1603 Instant notebook mode | Show one exact local script as ordered cells with typed bounded output and inert notes. Pin cell identities, dependencies and source version. Editing a cell makes a new draft and invalidates dependent outputs. Deliberate owner-started evaluation may run the exact pinned cell and allowed input prefix in bounded scratch under declared needs. Inherit no credentials. Stop clears work. Unsupported conversion stays unavailable. A saved output from an older run is labelled stale and never executes on open. No state shared with a production worker. | PY-02, PY-05, PY-09, PY-10 · [FX-0104-W5-PINNED-NOTEBOOK-CELL](../planning/feature-wave-5.md#acceptance-design-for-n-1599-to-n-1622) |
| N-1604 Zero-boilerplate hook decorators | Read only admitted declarative annotations into a typed manifest proposal. Validate trigger, inputs, allowed output and exact 0099 action versions without invoking a decorator or importing the script for discovery. Unsupported metadata holds. A decorator does not register, activate or grant a hook. | PY-03, PY-04, PY-10; 0099 JW-16 · [FX-0104-W5-DECLARATIVE-HOOK-PROPOSAL](../planning/feature-wave-5.md#acceptance-design-for-n-1599-to-n-1622) |
| N-1605 Auto-generated settings forms from type hints | Parse only the admitted declarations and constants as data. Show correct types, bounded values, permitted defaults and accessible errors. Hold dynamic expressions, unsupported or ambiguous types and stale manifests. Recheck current value, settings authority and revision at save and dispatch. A form cannot widen a fence. | PY-04, PY-09, PY-13 · [FX-0104-W5-INERT-TYPED-SETTINGS-FORM](../planning/feature-wave-5.md#acceptance-design-for-n-1599-to-n-1622) |
| N-1606 External IDE live sync | Watch only the current owner-selected file in the verified local source root under the existing path and receipt rules. Save an immutable draft and show exact diff. Recheck on rename, path replacement, restart and activation. No IDE plug-in, network sync, external file fetch or execution from Save. | PY-03, PY-09, PY-10, PY-11 · [FX-0104-W5-EXACT-LOCAL-SOURCE-WATCH](../planning/feature-wave-5.md#acceptance-design-for-n-1599-to-n-1622) |
| N-1607 Built-in Git version history for scripts | Use 0104's bounded immutable local history, readable diff and restore proposal. A branch label can represent a local draft lineage only when admitted. Restore creates a new candidate and needs current grants and checks. No Git remote, shell, hook, repository rewrite or revived approval. | PY-03, PY-09, PY-10, PY-19 · [FX-0104-W5-VERSIONED-LOCAL-HISTORY-RESTORE](../planning/feature-wave-5.md#acceptance-design-for-n-1599-to-n-1622) |
| N-1608 Custom sidebar mini-apps | Accept only versioned display data in admitted panel, table, chart or form schemas. Escape output and bind the current script and view scope. A control can present an existing typed proposal action only within its own grant. No HTML injection, remote frame, executable script UI, native change or hidden input. | PY-02, PY-04, PY-10, PY-13 · [FX-0104-W5-INERT-SIDEBAR-SCHEMA](../planning/feature-wave-5.md#acceptance-design-for-n-1599-to-n-1622) |
| N-1609 One-click virtual-environment package installer | Use an existing owner-approved per-script environment. Bind the exact compatible built wheel and dependency set, versions, ABI, licence, hashes and ledger receipts. Local catalogue discovery creates no install. The existing owner-click index action may use only official PyPI with exact pinned compatible wheels including transitives. Recheck artefact bytes, licence and hash before installation. Refuse source distributions and compiler. Record user-package inventory and provenance separately from the bundled release. The actual installation uses only the separately approved package route and no compiler or global environment. | PY-07, PY-08, PY-16, PY-23 · [FX-0104-W5-EXACT-APPROVED-WHEEL-ENVIRONMENT](../planning/feature-wave-5.md#acceptance-design-for-n-1599-to-n-1622) |
| N-1610 Script-to-script chaining | Bind a finite typed sequence to the exact 0099 graph, individual script and environment revisions, site and original effect identities. Each child gets its own current authority with bounded inputs and resources. Reject unresolved ports, cycles and inherited needs. Only a proved eligible failure uses its owning fallback policy. Hold ambiguous effects until the exact original receipt reconciles. Never propagate handles or wider projections to children. No chained step grants a send. [FX-0104-W5-SCRIPT-CHAIN-EXACT-CHILD-RECEIPT](../planning/feature-wave-5.md#acceptance-design-for-n-1599-to-n-1622). | PY-02 to PY-06, PY-12; 0099 JW-04/08/16 |
| N-1611 Rich live print outputs | Render eligible bounded typed values as inert tables, labelled progress or safe address text under the exact output profile. Keep missing or truncated content visible. Use links only through the separately admitted preview and user action. Do not auto-resolve, open external content, execute markup or print hidden fields or credentials. [FX-0104-W5-INERT-RICH-OUTPUT-SAFE-PROJECTION](../planning/feature-wave-5.md#acceptance-design-for-n-1599-to-n-1622). | PY-02, PY-05, PY-21 |
| N-1612 Community and agency script packs | Validate bounded pack encoding and safe paths before unpack. Refuse duplicate or escaping names, external references and embedded grants. Keep one site scope. Separate unsigned local import from publisher-authenticated admission. Verify publisher and digest of signed packs. Every script stays inactive. Record each script, need and dependency with permitted provenance and licence. Check current site and scope, versions and package identity before a separate local draft. A signature gives no install, approval, client access, network or scoring right. [FX-0104-W5-SIGNED-SCRIPT-PACK-INERT-IMPORT](../planning/feature-wave-5.md#acceptance-design-for-n-1599-to-n-1622). | PY-03, PY-04, PY-07 to PY-10, PY-21 |
| N-1613 Plain-English to QuarryView Python | Use the current Alekh name and admitted typed view and filter schema. A permitted agent may propose bounded filter code using allowed fields, measures, time windows and missing-value semantics. Unknown definitions hold. Review the exact draft on synthetic inputs. No invented column, live application, new query scope or causal claim. [FX-0104-W5-ALEKH-EXACT-TYPED-FILTER-PROPOSAL](../planning/feature-wave-5.md#acceptance-design-for-n-1599-to-n-1622). | PY-02, PY-04, PY-05, PY-10; 0043 |
| N-1614 Auto-generated test suites before hot-reload | Bind generated test proposals to the exact candidate, source, manifest, dependencies, fence and synthetic fixtures. Validate expected results, error cases and effects independently of script claims. A failed, incomplete or stale run holds. Passing generated checks alone cannot activate, waive other checks or replace human approval. [FX-0104-W5-GENERATED-TEST-CURRENT-CANDIDATE](../planning/feature-wave-5.md#acceptance-design-for-n-1599-to-n-1622). | PY-03, PY-09 to PY-11, PY-17 |
| N-1615 Self-healing CSS and XPath extractors | Propose a user-script selector change from eligible evidence and prove it on bounded inert synthetic page fixtures. Use the pinned admitted parser and selector subset of the exact owning read contract. Unknown syntax, unsafe expansion and changed source hold. No new crawl, browser launch, external entity or script execution, arbitrary URL fetch, product parser patch or automatic reload. [FX-0104-W5-SELECTOR-CHANGE-SYNTHETIC-ONLY](../planning/feature-wave-5.md#acceptance-design-for-n-1599-to-n-1622). | PY-02 to PY-05, PY-09 to PY-11; 0082/0115 |
| N-1616 AI-guided script optimisation | Use current authorised profiling and measured scopes for the pinned version. A permitted assistant may suggest a local diff with explicit counterexamples and synthetic equivalence and speed cases. Keep units and uncertainty visible. No speed guarantee, compiler, changed package, activation or ranking decision follows from a suggestion. [FX-0104-W5-OPTIMISATION-COMPARABLE-SYNTHETIC-DIFF](../planning/feature-wave-5.md#acceptance-design-for-n-1599-to-n-1622). | PY-03, PY-09 to PY-12 |
| N-1617 Natural-language custom buttons on demand | Translate eligible wording to a local typed button proposal with declared data inputs, script need, expected result and existing approval path. Revenue, sitemap and other facts follow their own scoped source contracts. A request cannot discover private data, add an action, mount a button, send or change content. [FX-0104-W5-TYPED-BUTTON-WITHOUT-EFFECT](../planning/feature-wave-5.md#acceptance-design-for-n-1599-to-n-1622). | PY-02, PY-04, PY-10, PY-13 |
| N-1618 Full live schema introspection over MCP | Expose only documented eligible 0104 type, field and helper metadata through the enrolled local 0020 route. Bind bounded continuation to enrolled peer, current site, grant, schema, filter and generation. Recheck after revoke or cached read. Refuse raw records, private table or column names, arbitrary reflection, files, live variables and inferred methods. Schema text is inert and is not a callable capability. [FX-0104-W5-SCHEMA-METADATA-CURRENT-PEER-CURSOR](../planning/feature-wave-5.md#acceptance-design-for-n-1599-to-n-1622). | PY-03, PY-04, PY-21; 0020/0118 |
| N-1619 Interactive AI code review before approval | Present the exact script diff, declared and admitted reads and outputs, source, dependencies, fence and verified tests. Distinguish observed results from predictions. Hold unknown effects or unsupported policy. A reviewer may record safe comments, not the owner's approval. Changed scope or revision invalidates review and approval. [FX-0104-W5-REVIEW-EXACT-DRAFT-NO-CONSENT](../planning/feature-wave-5.md#acceptance-design-for-n-1599-to-n-1622). | PY-03, PY-04, PY-09 to PY-11 |
| N-1620 Automated bug reproduction and patching | Use the eligible safe failure classification and protected scoped diagnostic references to propose a local synthetic regression fixture and candidate diff for ordinary product behaviour. A vulnerability report stays in the separate defensive triage route. This feature cannot reproduce or exercise a vulnerability or generate an exploit. Avoid exposing the failing URL, raw traceback or production records. If synthetic evidence cannot reproduce it, report the gap. No overnight agent dispatch, remote probe, automatic patch or resume. [FX-0104-W5-SYNTHETIC-REGRESSION-DEFENSIVE-TRIAGE](../planning/feature-wave-5.md#acceptance-design-for-n-1599-to-n-1622). | PY-03, PY-09 to PY-12, PY-15; 0020/0032 |
| N-1621 Plain-English documentation generator | Produce a bounded local draft description of a currently authorised script and exact revision. Include eligible settings, need and tested limitations. No authorisation to read a colleague's private script, reveal a source path or alter its metadata follows from generation. Stale documentation remains labelled until reviewed. [FX-0104-W5-DOCUMENTATION-CURRENT-AUTHORISED-REVISION](../planning/feature-wave-5.md#acceptance-design-for-n-1599-to-n-1622). | PY-04, PY-09, PY-10, PY-21 |
| N-1622 Conversational A/B formula tuning | Compare proposed formulas only through bounded synthetic offline fixtures under the admitted research and version rules. Pin baseline and three variants with comparable synthetic inputs, exact expected result or oracle, metric, coverage and resource scope. Report ties and insufficient evidence. The proposal cannot run a live test, choose a champion, reach protected evaluation sets, pass a score to live ranking or widen the separate default-off RS30 route. | PY-01, PY-02, PY-10, PY-22; 0072/0077/0094 · [FX-0104-W5-OFFLINE-FORMULA-ORACLE-AND-TIES](../planning/feature-wave-5.md#acceptance-design-for-n-1599-to-n-1622) |

QR-208 qualifies the exact profiles, supported hosts, parsers and
evidence. No external account, parser, model route or production
effect is newly admitted by this section.

#### Business evidence and outside-tool proposals (N-1623 to N-1631)

A fenced local script receives only the eligible typed projection named
by its current manifest. Bind site, source revision, intended purpose,
script and environment versions, grant and erasure generation before
reading. Recheck on output, replay and restore. A proposal cannot grant
connection, credentials, live lookup, confirmation, spending or a native
effect. External actions remain under their owning specs and separate
approval. Test with bounded synthetic records.

| Feature | Exact script scope | Existing requirements |
|---|---|---|
| N-1623 Profit-and-margin prioritisation | Use permitted owner-supplied business files with admitted same-site product and variant mapping. Bind margin, currency, unit, effective interval and exact source revision. Distinguish unknown, missing and proved zero. Show the eligible local audit order and its scope. Keep protected amounts and unauthorised joins out of exports. No ERP or live inventory request, shop action or link-ranking input. | PY-01, PY-02, PY-05, PY-21 and 0100/0103 · [FX-0104-W5-SUPPLIED-COMPATIBLE-MARGIN-SCOPE](../planning/feature-wave-5.md#acceptance-design-for-n-1623-to-n-1646) |
| N-1624 Direct CMS publish and draft hooks | Return an exact local wording or change proposal with target, fields, source and scope. Preserve XenForo and WordPress native admission and separate human review. A script cannot create even a native draft, edit canonicals or insert approved links. Shopify and headless CMS write hooks have no route. Owner review of the script or its proposal cannot authorise a native action through Python. | PY-01, PY-02, PY-03 and 0020/0090 · [FX-0104-W5-CMS-LOCAL-PROPOSAL-NO-NATIVE-WRITE](../planning/feature-wave-5.md#acceptance-design-for-n-1623-to-n-1646) |
| N-1625 Smart issue-tracker routing | Prepare an eligible local candidate from permitted fields, exact Fix list observation and scoped dedupe key. Attach only admitted non-sensitive evidence. The existing 0100 tracker route separately owns eligible destination, exact preview and approval, current DR-079 outbound level including addresses, owner review, idempotent receipt and dedupe. Approval of a proposal confers no issue write or dispatch. A post-crawl script cannot dispatch, import credentials or open GitHub Issues. | PY-02, PY-03, PY-21 and SD-03/04/05/06/21 · [FX-0104-W5-TRACKER-PROPOSAL-CURRENT-OUTBOUND-SCOPE](../planning/feature-wave-5.md#acceptance-design-for-n-1623-to-n-1646) |
| N-1626 Live alert webhooks for SEO emergencies | Prepare a safe local alert proposal from approved observation and current rule evidence. Distinguish a proved event from a partial crawl or stale state. Existing 0099/0100 keep their own route, destination, owner approval, outbound level and idempotent receipt. No automatic route or dispatch by script or proposal. A script cannot open a webhook or listener, call a provider, send URLs or trigger remote work. | PY-02, PY-03, PY-18 and 0099/0100 · [FX-0104-W5-SAFE-LOCAL-ALERT-NO-LISTENER](../planning/feature-wave-5.md#acceptance-design-for-n-1623-to-n-1646) |
| N-1627 Automated client deck and report builders | Shape an authorised local report proposal from exact 0094 snapshot, sources and protected coverage. Use only the separately admitted inert Markdown, print or office-file profile and final recipient checks. PDF needs QR-039 qualification where applicable. DR-087 keeps Slides and comparable destinations as files for manual use. No cloud connection, send, presentation automation or fabricated metrics. | PY-02, PY-03, PY-05, PY-21 and 0094/0100 · [FX-0104-W5-DECK-PROPOSAL-LOCAL-CUSTODY](../planning/feature-wave-5.md#acceptance-design-for-n-1623-to-n-1646) |
| N-1628 Custom AWS S3 and external SSD pipelines | Propose an exact approved export class or storage intent, with site, bounded data, retention, destination profile and evidence. A script's own admitted export folder remains its only file output for this case. The 0038 verified store, 0030 ledger and 0118 protection own any later storage move or cloud copy. No direct cloud call, arbitrary drive path, plaintext transfer, credential, activation or deletion. | PY-02, PY-03, PY-04, PY-21 and 0030/0038 · [FX-0104-W5-STORAGE-INTENT-EXACT-SITE-DESTINATION](../planning/feature-wave-5.md#acceptance-design-for-n-1623-to-n-1646) |
| N-1629 Competitor SERP and changelog monitors | Compare current eligible owner-supplied research or an already separately approved one-off observation under 0082 and 0092. Bind already-obtained rights-checked site, source revision, lawful scope, dates and coverage. An external address or periodic callback cannot acquire evidence on the script's behalf. Unknown stays unknown. DR-085 keeps scheduled rival, price and SERP monitoring outside this route. The distinct DR-115, DR-117 and DR-118 actions grant no script access or repetition. | PY-02, PY-03, PY-05 and DR-085/115/117/118 · [FX-0104-W5-EXISTING-RIVAL-EVIDENCE-NO-REACQUISITION](../planning/feature-wave-5.md#acceptance-design-for-n-1623-to-n-1646) |
| N-1630 Paid-search and SEO synergy scripts | Compare compatible supplied paid and organic metrics for the exact site, query scope, population, interval and unit. Use the protected 0098 report profile and show sources, eligible comparable basis and uncertainty. A difference does not prove a saving or causal result. No provider refresh, bid, spend, audience change or ranking signal. | PY-01, PY-02, PY-05 and 0098/0100 · [FX-0104-W5-SUPPLIED-PAID-ORGANIC-EXACT-COMPARISON](../planning/feature-wave-5.md#acceptance-design-for-n-1623-to-n-1646) |
| N-1631 Open-ended AI model bridges | Keep a script's typed local input and output separate from any model invocation. A model request is a typed local proposal only. Its separately authorised owning route checks scope, fence and approval. Returned language is inert and confers no follow-up or signing authority. Unknown profiles hold. No arbitrary endpoint, cloud key, proxy, spend, confirmation, automatic fallback or native delivery. | PY-01, PY-02, PY-03, PY-18, PY-21 and 0020/0080/0091 · [FX-0104-W5-TYPED-MODEL-PROPOSAL-NO-FOLLOW-UP](../planning/feature-wave-5.md#acceptance-design-for-n-1623-to-n-1646) |

QR-209 qualifies eligible projections, compatibility, receipt and
refusal evidence. These local script forms do not add collection,
tracking or public-page work.

#### Bounded Python views, observations and authoring (N-1853–N-1862)

Pin one authorised site, script, environment, manifest, host fence,
admitted inputs, immutable revisions and output schema. Recheck
grants and erasure at dispatch, delivery, replay and restore. Reuse
0078 resource admission, 0019 job and checkpoint authority and 0118
custody. Any Python output remains data, a review draft or an exact
eligible export under the owning path. It cannot execute in live
ranking or in replies to sites, open a network route or import a
native authority.

| Owner catalogue entry | Exact script or worker contract |
| --- | --- |
| N-1853 Always-warm Python worker pool | Use only the measured 0104 warm pool and current 0078 lease. Warm state is pre-admitted and contains no previous job's records, secret handles or mutable grants. An empty scratch folder does not prove the worker is clean. Hold warm admission before dispatch if in-memory state from a previous job remains or its absence is unproved. Use the existing safe retirement boundary. Under pressure, idle timeout, revocation and stop, release the instance safely. Qualify the proposed under-100-ms warm start plus idle and working memory on the 8 GiB reference and each supported host profile. No unconditional residency, unproved speed or start of a script. [FX-0104-W5-ADMITTED-EMPTY-WARM-WORKER](../planning/feature-wave-5.md#acceptance-design-for-n-1839-to-n-1862) |
| N-1854 Python-powered QuarryView filters | Use Alekh's typed view and filter schemas with an admitted bounded worker over an immutable eligible snapshot. Pin fields, scopes, windows, missing-value semantics and result limit. Hold unknown columns or out-of-scope reads. No arbitrary SQL, new query authority, rank feed or update. [FX-0104-W5-IMMUTABLE-SCOPED-ALEKH-FILTER](../planning/feature-wave-5.md#acceptance-design-for-n-1839-to-n-1862) |
| N-1855 One-click Open this view in Python | Translate the exact current Alekh filter, sort, scope and input schema to a versioned inert editable script draft. Omit raw records, private paths, keys and grants. A non-translatable control gives an explicit gap. Opening, saving or restoring the draft cannot execute or widen access. [FX-0104-W5-INERT-ALEKH-TO-PYTHON-DRAFT](../planning/feature-wave-5.md#acceptance-design-for-n-1839-to-n-1862) |
| N-1856 Custom HTML and DOM extractors | Use the admitted parser, selector subset and already approved raw or rendered evidence for the exact revision. Return only typed eligible fields with source, coverage and missing reasons. Receipts mode remains raw and limited. No fresh request, arbitrary document access, protected personal extraction, parser execution side effect or automatic update. [FX-0104-W5-SCOPED-TYPED-DOM-EXTRACTION](../planning/feature-wave-5.md#acceptance-design-for-n-1839-to-n-1862) |
| N-1857 Raw vs. rendered HTML diff scripts | Consume only the current comparable typed 0115 diff or admitted input projection. Keep exact profiles, changes, dependency limits and unsupported facts distinct. Missing or incompatible render is unavailable. Refused required dependency makes affected admitted facts partial with lower confidence under RN-34. No new capture. No independent authority to read hidden material or label observed change as intent. [FX-0104-W5-COMPARABLE-RAW-RENDERED-DIFF](../planning/feature-wave-5.md#acceptance-design-for-n-1839-to-n-1862) |
| N-1858 Bot vs. browser comparison hooks | Use only comparable paired own-site Audit observations already admitted under 0115 and QR-050. Return the exact observed differences with scope and unknowns. No impersonation of a search crawler, repeat rival check, new renderer run or claim about what a named bot sees. [FX-0104-W5-PAIRED-OWN-SITE-AUDIT-OBSERVATION](../planning/feature-wave-5.md#acceptance-design-for-n-1839-to-n-1862) |
| N-1859 Robots.txt and meta-tag conflict auditor | Compare exact captured rule, page directive, header and sitemap evidence. Keep scope, version and admitted precedence with 0082 and 0044. Conflicts and unknowns are historical findings. No override to a deny, new fetch, policy edit or automatic indexing action. [FX-0104-W5-CAPTURED-DIRECTIVE-CONFLICT](../planning/feature-wave-5.md#acceptance-design-for-n-1839-to-n-1862) |
| N-1860 Custom internal PageRank and authority models | Run bounded isolated research or typed review over admitted same-site graphs, with exact profiles and result evidence. Where relevant use the existing 0027 simulator. Output is inert comparison, not live rank, graph mutation or new weight. The separately admitted RS30 rate remains the only allowed script-derived live order input under 0072. [FX-0104-W5-ISOLATED-SAME-SITE-GRAPH-COMPARISON](../planning/feature-wave-5.md#acceptance-design-for-n-1839-to-n-1862) |
| N-1861 Strict per-script time and memory caps | Require current host fence, supported process limits and 0078 promise before work. Enforce outside the script, stop at the admitted bound, clean up the worker and fence late returns. Prove CPU, elapsed time and memory claims in the owning platform profile. Unknown enforcement holds and no retry inherits a stale lease. [FX-0104-W5-EXTERNAL-TO-SCRIPT-RESOURCE-STOP](../planning/feature-wave-5.md#acceptance-design-for-n-1839-to-n-1862) |
| N-1862 Record clicks into clean Python | Record selected eligible actions only during an explicit bounded local session. Store typed action IDs and already authorised safe arguments, without keystrokes, credentials, opaque confirmations, private records or grants. Compile only an inert editable draft, including gaps for unsupported actions. Review and approval, then any execution, use the owning 0104 and 0020 paths. Automation cannot reproduce native writes, sends or confirms. [FX-0104-W5-BOUNDED-TYPED-INTENT-RECORDING](../planning/feature-wave-5.md#acceptance-design-for-n-1839-to-n-1862) |

QR-218 qualifies the exact projection, fence, replay and processor
limits without granting runtime or phase admission.

#### Authoring and combined evidence (N-1863–N-1864)

- **N-1863, one universal language every AI already masters.**
  Python is the admitted user-script language for this sidecar.
  A local enrolled agent sees only the allowed typed script
  profile, known inputs and safe diagnostics. It may propose
  a version and synthetic cases. Familiarity with a language
  proves neither correctness nor support. Require the exact
  platform fence, package and fixture evidence and human approval
  under PY-03, PY-09, PY-10 and PY-17. No first-try guarantee,
  native edit, core Python hook or live ranking input.
  See [FX-0104-W5-FENCED-LOCAL-PYTHON-PROPOSAL][w5-python-local-evidence-cases].
- **N-1864, Search Console and analytics fusion scripts.**
  Use only eligible same-site projections already admitted by
  0034, 0040 and 0043, or separately approved supplied files.
  Bind canonical property and page revisions, measurement
  definitions, unit, native time basis, cohort, coverage and
  eligible join keys. A shared date label does not prove a shared
  interval. If native day boundaries differ without exact eligible
  common coverage, show the source reads only in their own scopes.
  Hold the joined result without splitting or resampling native totals.
  Supply a file only as the bounded read-only
  projection from its approved intake. Keep its own source label,
  revision, coverage and current grant. Check that grant at input
  and before releasing the result or reusing a cache. A missing
  projection stays unknown. An incompatible projection gives an
  unavailable comparison. Suppressed or revoked inputs stay
  withheld. A zero requires explicit eligible source evidence.
  A file or cached output cannot silently replace a selected
  source that lost evidence or rights. No identity joins or
  re-creation of withheld counts.
  The output is local audit advice only. Direct connector actions
  are outside Python, including DR-120's Matomo timeline route.
  No native refresh, extra credential, provider request or
  ranking effect. Trace PY-01, PY-02, PY-05, PY-21 and QR-219.
  See [FX-0104-W5-SCOPED-ANALYTICS-FUSION][w5-python-local-evidence-cases].

[w5-python-local-evidence-cases]:
  ../planning/feature-wave-5.md#acceptance-design-for-n-1863-to-n-1880

### Data

- `scripts`: id, name, kind, current version, owner kind (official, user,
  agent), state.
- `script_versions`: script, version, source hash, manifest, saved by, time.
- `script_runs`: script, version, inputs hash, state, time, memory peak,
  worker, fence outcome.
- `script_cache`: script version, settings hash, page hash, result hash.
- `script_environments`: script, package, version, licence, hash, installed.
- `fence_events`: script, rule crossed, time, action taken.

Script source and manifests are files in the home folder; the ledger of spec
0030 lists them.

### Settings

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `script.max_run_seconds` | seconds (replaces `quarry.script.max_seconds`) | 60 | 1 to 3,600 | next run |
| `script.chunk_rows` | whole number | 5,000 | 100 to 100,000 | next run |
| `script.cache_enabled` | on or off | on | two values | at once |
| `script.network_allowed` | on or off | off | two values | next run |
| `script.history_keep` | whole number | 50 versions | 5 to 1,000 | next save |
| `script.agent_requires_tests` | on or off | on, locked | on only | never off |
| `script.profiler_enabled` | on or off | off | two values | next run |

### Failure codes

SUT-910 script refused by the fence; SUT-911 time limit reached; SUT-912
memory promise exceeded; SUT-913 need not declared; SUT-914 manifest
invalid; SUT-915 tests failed, not activated; SUT-916 package hash mismatch;
SUT-917 package licence needs confirmation; SUT-918 sidecar not ready; SUT-919
script tried to write outside its doors. A failure leaves the previous version
active.

### Security and privacy

- All script input is untrusted data. A page's text can never become a
  command to a script or an agent.
- Folder, network and process limits are enforced by the operating system,
  and time and memory limits by the sidecar with a forced stop, never by the
  script's good behaviour.
- Secrets are never in a snapshot; a script that needs one asks through the
  vault of spec 0065 and gets a short-lived handle, never the value.
- No script or package sends data out unless the user allowed that host.
- Script output shown on screen or in a log is masked for secrets.

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| PY-01 | Python MUST NOT run in the live ranking path or build a reply to a site. | UT-PY-01, BD-PY-01 |
| PY-02 | A script MUST only create proposals, signals or exports. | UT-PY-02 |
| PY-03 | The folder, network, process and library limits of a fence MUST be enforced by the operating system and MUST NOT be widenable by the script. Time and memory limits MUST be enforced by the sidecar with a forced stop as the last step. | PL-PY-01 to 07 |
| PY-04 | A need not declared in the manifest MUST be blocked. | UT-PY-03 |
| PY-05 | A worker MUST receive a read-only snapshot by default. | UT-PY-04 |
| PY-06 | Large tables MUST be processed in chunks within the memory promise; they MUST be passed without copying rows only if QR-028 proves it. | IT-PY-01, FI-PY-01 |
| PY-07 | Each script MUST have its own package environment. | IT-PY-02 |
| PY-08 | A package install MUST show licence and hash and be recorded, and be removable. | UT-PY-05 |
| PY-09 | Every save MUST be a restorable version with a readable diff. | UT-PY-06 |
| PY-10 | An agent-written script MUST pass generated tests and human approval before activation. | BD-PY-02 |
| PY-11 | A reload that fails MUST keep the old version running. | FI-PY-02 |
| PY-12 | A worker over its time or memory limit MUST be stopped. | FI-PY-03 |
| PY-13 | Settings forms MUST be built from type hints and validated. | UT-PY-07 |
| PY-14 | The sidecar MUST sleep when idle and wake on demand under the sidecar rules of spec 0078 (warm workers and heavy-worker limit). | IT-PY-03 |
| PY-15 | A crashed or failing script MUST never stop the product. | FI-PY-04 |
| PY-16 | Removing a script MUST remove its environment and ledger entries. | IT-PY-04 |
| PY-17 | The fence MUST work on Windows 11 and Ubuntu and Debian before scripts are enabled there. | PL-PY-01 to 07 on each system |
| PY-18 | A script MUST have no network unless its manifest names hosts and the user allows them. | UT-PY-08, PL-PY-02 |
| PY-19 | A cached result MUST be used only for the same script version, settings and page hash. | UT-PY-09 |
| PY-20 | A script that fails three runs in a row MUST pause with its last error and a fix button. | BD-PY-03 |
| PY-21 | A snapshot MUST hold no secret; a secret MUST reach a script only as a short-lived handle. | UT-PY-10 |
| PY-22 | Rust MUST consume only the current admitted RS30 rate, using the owner switch and spec 0072 validation. | IT-PY-05, RM06 |
| PY-23 | A package that needs a compiler MUST be refused with a plain message. | UT-PY-11 |

## How it is tested

Designed, not run.

- **Unit (UT-PY):** manifest checks, need blocking, type-hint forms (with a
  value just inside and just outside each range), version and diff, hash check
  of a package, refusal of a package that needs a compiler (UT-PY-11), network
  off unless allowed (UT-PY-08), cache keys (UT-PY-09), no secret in a snapshot
  (UT-PY-10).
- **Planted faults (PL-PY):** one script per escape trick that must be stopped
  with the named message: reading outside its folder, opening a network
  socket, starting a shell, loading a foreign library, writing outside the
  two doors, trying to widen its own needs, and an honest script that must
  pass.
- **Behaviour (BD-PY):** the Ravi scenario; an agent writing a view with tests;
  a broken extractor repaired by an agent and approved.
- **Integration (IT-PY):** a one-million-row table streamed in chunks inside
  the memory promise; two scripts with conflicting package versions;
  sidecar wake and sleep; removal clean-up; a script signal refused until
  admission and the owner switch allow RS30 (IT-PY-05); three failed
  runs pause a script (BD-PY-03).
- **Fault injection (FI-PY):** a worker killed mid-run, a memory spike, a
  failed hot-reload, a crashing optional script.
- **Measurement:** start time of a warm worker; handoff cost of a million
  rows; memory of the sidecar idle and at work on the 8 GiB reference.

## Changes from the source

ADR-0020's rule "no first-party Python" is replaced by ADR-0021: small boxed
first-party Python, never in ranking. Spec 0043's declarative TOML script
rules and its `quarry.script.*` settings are superseded by Python scripts
under this spec; a banner in spec 0043 says so, and its full pass is queued.
Spec 0037's fence uses Linux controls only and needs a Windows design
(QR-029). Features of the pasted Python catalogue and the sidecar and script
features of the Jobs paste now have this one home.
