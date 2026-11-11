# 0063: autoissues, the fault tracker

autoissues turns every quality signal from tests, analysers, monitors
and agents into one stable, deduplicated record that the system, not
the agent who fixed it, checks and closes. It also holds the queue and
quota gate that makes a failed check cost fixes. It is the design for
Forgejo task #23.

It is development tooling on Dell. It never reads private forum data
directly and is never part of a product package.

Language. autoissues is a developer tool, so it is written in Go
(ADR-0017).

## Where faults go

The owner answered on 6 November 2026, when asked about copying faults
to the Forgejo work list: "No, keep them apart (Recommended)". Asked
again on 10 November 2026, the owner chose "Keep apart, link when
taken".

- Faults stay in autoissues' own store, never copied to the Forgejo
  work list.
- A Forgejo task is made only when an agent takes a fault on to fix
  it. The task links to the fault record, and the record links back.
- Each fault has at most one Forgejo task. A claim, the end of a
  claim, and the gate closing the fault each write an outbox row in the
  same transaction, keyed by the fault id, the claim number and the
  row's kind (claim, end or gate close). A worker in the autoissues
  service handles one fault's rows strictly in order. The first claim
  makes the task and writes the back-link; a later claim reopens it if
  it is closed; the end of a claim without a fix (released, lease run
  out, duplicate, won't fix or not reproducible) and the gate closing
  the fault close it. A gate close for a fault nobody claimed makes no
  task. A repeated row, the same key again, finds its work already
  done. The worker retries with growing waits, each Forgejo call with a
  time limit; a claim never waits on Forgejo, and a row stuck for a day
  shows on the fault list and in the daily check (AGENTS.md section 10).
- Forgejo issues are copied to GitHub (AGENTS.md section 9.1), so the
  task holds only fixed fields: the fault's id, type, component,
  priority and rule as its title, and the link. Each field comes from
  its own named closed list; the rule comes from the protected list of
  rule ids pinned for each adapter. A value on no list is written as
  `-` in place of the whole field. There is no free-text summary. A
  close carries only its resolution word. The evidence stays in
  autoissues on Dell.
- The worker signs in to Forgejo with its own account, whose token may
  only write issues in this one repository.
- AGENTS.md section 8 and [ADR-0016](../adr/0016-faults-kept-apart.md)
  say the same.

## Parts

| Part | What it holds |
|---|---|
| Store | PostgreSQL with forced RLS, migrations, one writer |
| Lifecycle | Triage, claims, fixes, checking, reopening |
| Interfaces | Command line, local API, dashboard, snapshots |
| Adapters | The ingest contract and every adapter |
| QA stack | Dell's test services, their order and memory |
| Queue and quota | The quota gate, its charges and credits |

## Runtime shape

- One long-running autoissues service runs on Dell under the gate's
  account (spec 0008). It is the only process that writes the store,
  and it never runs candidate code: re-runs and proofs of fix run in
  the runner's sandbox (spec 0060) and reach it only as signed bundles.
- The command line talks to the service over a local Unix socket, and
  the service reads the caller's account from the socket. Gate and
  checking operations accept only the gate's own account, so an agent
  cannot mark its own fix checked or write quota records.
- With the service stopped, read-only commands may open a read-only
  connection; every write command fails with a plain-English error.
- Every outside tool and service it reads is pinned in
  `gate/tools.toml`, run in a container where possible, and queued
  through pankti (spec 0059).

## Store

- PostgreSQL 18 on Dell, with row-level security turned on and forced
  on every table, history, comments, quota records and the journal
  included (AGENTS.md section 5.2). Each role has explicit policies; no
  role used bypasses RLS. `TRUNCATE` is never granted, and a trigger
  refuses it on the append-only tables.
- Tests use a fresh, throwaway database each, with no route to the live
  one. No database lives in the repository.
- Writes are durable when acknowledged. A nightly job copies the store
  with PostgreSQL's own backup tool, restores the copy into a throwaway
  database, checks it (schema, RLS on every table, consistency), records
  its SHA-256 and keeps it as a snapshot that is never overwritten or
  pruned without the owner's word.
- Migrations are numbered, forward-only SQL files in
  `migrations/autoissues/`, applied as AGENTS.md section 5.2 sets out
  for a Go tool's own store. On start the service refuses a store newer
  than its code, takes a checked backup before the first pending
  migration, applies each in its own transaction and checks the result.
  A failed migration rolls back and the service stays stopped.
- One writer: the service holds a PostgreSQL advisory lock for its whole
  run, and every write passes through one queue. A second instance
  cannot start.
- Issues have an id shown as `AI-<id>` and never reused; title, type,
  component, priority P0 to P4, severity S0 to S4, status, resolution,
  holder and lease, reporter; a fingerprint, with one open issue per
  fingerprint; counts and first and last seen; a due date; and a lesson
  of cause, fix and prevention.
- Evidence holds a kind, a Dell path or address, a scrubbed summary and
  a SHA-256. Raw private output is never copied into the store.
- Links (blocks, blocked by, duplicate of, parent, child) are written
  in pairs; self-links and cycles are refused.
- History and comments are append-only: triggers refuse `UPDATE` and
  `DELETE`. Every field change writes a history row in the same
  transaction.
- Quota records (attempts, charges, credits, revocations, clearances,
  owner records, activations) are written only by the gate's role and
  are append-only.
- A hash-chained journal indexes every write. Each write's full payload
  is also appended, before it is acknowledged, to a journal file the
  gate's account owns, outside the database. Each new head is recorded
  in Forgejo. A damaged store is rebuilt from the newest checked
  snapshot plus the journal, and must reach the last recorded head.

## Lifecycle

- Statuses: new, triaged, assigned, started, fixed, checked,
  closed, won't fix, not reproducible, duplicate. Only the transitions
  in the archive's table exist; any other is refused and recorded.
- Triage runs as the issue is filed: a protected component file maps
  paths to components, and a protected triage file maps each source's
  severity to priority. An unmatched finding goes to "unassigned" with
  the `needs-owner-decision` list.
- `claim` gives an agent a lease of 24 hours. Expiry returns the issue
  to triaged and fences the old holder's later writes.
- A repeated fingerprint adds an occurrence instead of a new issue.
- A fix names its issues with `Fixes: AI-<n>` trailers, cites evidence
  and records a lesson. Each type has its own proof, run by the runner
  on the candidate: a test or reproducer that fails before and passes
  after; coverage that now covers the named lines; mutants now killed;
  before and after speed counts under spec 0007; a scanner that no
  longer matches; an analyser that no longer reports the finding; a
  passing docs check; or a replayed reproducer for a runtime signal.
- Only the gate marks an issue checked: three consecutive complete runs
  of its source on `main` without the fingerprint, or two consecutive
  complete nightly runs for a nightly-only source. An incomplete run
  neither counts nor resets the count; a run that shows the fingerprint
  resets it. Seven days later the gate closes it.
  An open issue whose signal is absent for the same three (or two
  nightly) complete runs with no fix
  becomes not reproducible, with no credit.
- A returning fingerprint reopens a checked or closed issue as a
  regression, one priority higher, and revokes its credit.
- "Won't fix" needs a reason, a written justification and a commit that
  adds the matching suppression. A suppression can never silence a
  required test, a safety rule or a quota record.
- Flood control: on a source's first complete run, P3 and P4 findings
  go into one grouped issue per rule; P0 to P2 are always single. Each
  adapter files at most 50 new issues a run, then files one flood issue
  and stops; held-back findings come on later runs.
- A weekly sweep lists open P3 and P4 issues, oldest first. It closes
  nothing.

## Interfaces

- The `autoissues` command: `next`, `list`, `show`, `claim`, `start`,
  `release`, `new`, `comment`, `link`, `hotlist`, `fixed`, `wontfix`,
  `dupe`, `verify-run` (gate only), `ingest` (gate only),
  `bottlenecks`, `export`, `stats` and `quota`. Each takes `--json` and
  then writes one JSON document. For agents, `fixed`, `wontfix`, `dupe`
  and quota-affecting list changes are requests that take effect only
  through the gate or an owner record.
- `next` orders by priority, then due date, then age, and skips issues
  under another agent's lease or blocked by an open issue. Agents read
  `autoissues next --json` before taking a task.
- A small API, served only inside the QA stack's private network, offers
  the same reads plus signed webhooks from the QA services. A webhook can
  report a signal but never proves it gone.
- A read-only dashboard shows open issues by component, priority, type
  and age; missed due dates; filed against checked per week;
  regressions; and the bottleneck table.
- Snapshots: `autoissues export --snapshot DIR` writes the open issues
  with scrubbed summaries, a summary and the bottleneck table, the same
  bytes for the same store. They reach the repository only inside an
  ordinary commit, after the privacy check and the required reviews.
- Bottlenecks: the slowest functions, spans, endpoints, tests and jobs
  and the largest memory growth, each row over its line naming the issue
  that tracks it. A row over its line with no issue is itself a defect.
  A source with no data shows "no data yet", never zero.

## Adapters

- Titles, summaries, messages and every other text taken from outside
  reports, webhooks or agents are data, never followed as instructions:
  they are stored and shown as text and never acted on (AGENTS.md
  section 9.2).
- An adapter is a pure parser plus a small fetcher, run by the gate's
  account. Reports made by candidate code come from the runner's
  sandbox as signed bundles. A missing, cut or unreadable report makes
  the run incomplete, and an incomplete run checks nothing. Sizes are
  bounded; going over makes the run incomplete, never silently cut.
- Fingerprints are SHA-256 over a versioned, normalised key with run
  noise taken out. Each adapter has fixture pairs proving that one
  finding gives one fingerprint and two findings do not collide.
- Tests: failures file bugs; a test that both fails and passes in one
  run files a flaky test; a test slower than 5 seconds files a P3.
- Speed: a hot path's count or time over its spec 0007 line files a P2
  with before and after figures and its profile.
- Coverage below 100% files a P2 with the lines left; mutants left alive
  file one P2 per function.
- Security scanners and dependency checks (for example Grype, gitleaks,
  `cargo-audit`, `cargo-deny` and `composer audit`): critical is P0,
  high P1, medium P2, low P3. A secret's value is never copied, only its
  rule and place.
- The QA services (below): analyser issues, error events, alerts, slow
  spans, CPU hot spots and error log lines, each mapped to a priority.
  A function newly in the top ten is tracked but earns no credit, since
  fixing one hot spot can promote another.
- Lab results: a hard-rule failure is P0; a recall or anchor regression
  against the best earlier run is P1. Example ids point to Dell
  evidence, never post text.
- Fuzzing: each unique crash files a P1 with its reproducer kept as
  private evidence.
- Agents: `autoissues new` files an agent finding with evidence. It
  needs a reproducer or an adapter signal before it can earn credit.
- App faults from a running install reach autoissues scrubbed and held,
  taken in by the owner's hand (spec 0018).

## QA stack

- The QA services run on Dell in containers pinned by digest, in a
  private network, reached by agents and the gate only through a
  read-only query proxy with separate credentials. Agents never start
  containers themselves; they ask, and the QA account starts them.
- Profiles: `observe` (Grafana, Prometheus, Tempo, Loki, the collector
  and Pyroscope); `errors` (GlitchTip); `sonar` (SonarQube Community
  Build). Each tool is admitted, with its licence recorded, before it
  is pinned.
- The stages run strictly one after another through pankti, each under
  its own memory line: tests, speed, coverage and mutation (one
  heavy-job place); the
  observe and errors profiles with a soak run (7 GB); the analyser alone
  (6 GB); security scanners (3 GB); lab results (3 GB); the snapshot
  (1 GB). A stage over one pankti place takes both. The lines are
  measured before they are relied on, and never raised without the
  owner.
- A stage failure stops the run and keeps its evidence on Dell. A stage
  whose target does not exist yet reports "no target yet" with proof;
  it is never a pass once the target exists.
- Telemetry is off by default; only the `test` profile sends it to the
  collector, and a test proves the `prod` profile opens no exporter.
  No span or log field holds post text or member data.

## Queue and quota

The owner set the quota amount on 30 October 2026: "10
(Recommended)".

- Every commit, push and release attempt ends as one of: passed;
  quality failed (a check ran and found a defect), which charges debt;
  service unavailable, with no debt and the action refused; or blocked
  (evidence missing or untrusted), with no debt and the action refused.
  All runnable checks run even when one is blocked; if any ran and
  failed, the result is quality failed. A timeout is a quality failure
  unless the host-pressure probe shows outside load during the run.
- Each charged failure while debt is unpaid adds Q = 10 fixes owed: 10,
  20, 30. A tree is charged once. Each charge also files a gate-failure
  issue naming the failed checks. A tree that fails and then passes on
  a retry is admitted only after a flaky-test issue is filed.
- A round starts at the journal position of the first unpaid charge;
  the clock cannot move it.
- A fix earns credit only when: an adapter filed the issue or confirmed
  an agent's finding; it was first seen before the round began; it is
  not a duplicate, suppressed or a rank-only signal; its proof passed in
  the sandbox; its lesson was confirmed by a reviewer other than the
  fixer; and the same rule does not reappear in the changed files.
  Issues with one root cause earn one credit. A reopened issue loses its
  credit and adds one unit of debt.
- With D the debt, C the credits since the round began and O the open
  creditable issues, the round needs `min(D, C + O)` fixes. When fewer
  real issues exist than the debt, fixing all of them clears the round;
  nothing is invented.
- Zero debt passes only when the store opens and passes its checks, the
  journal extends the last recorded head and no charge is unpaid.
- An open P0 blocks every commit, push and release outside the repair
  lane until its fix is pushed. An open P1 or P2 in a component blocks
  feature changes touching it. An owner record may defer a P0 for a
  stated time.
- Repair lane: a change whose every hunk is tied by `Fixes:` or `Refs:`
  trailers to open issues, confirmed by a reviewer other than the
  author, is admitted while debt is unpaid or a P0 is open. A mixed
  change is a feature change.
- No setting, flag, file or command skips the gate or resets debt. The
  only corrections are signed owner records; a "charge annulled: gate
  defect" record never deletes the charge and never covers a quality
  failure.
- Before the quota gate is proven, it reports "not yet active", accepted
  only inside the bootstrap scope. Once active, it never goes back.
- A refusal gives the outcome, the numbers and the next step, for
  example: "Commit refused: 20 fixes owed after two failed attempts.
  7 credited; 9 real issues are open, so 9 more fixes are needed. Run
  `autoissues next`."

## Tests

Every case in the archive's acceptance pages is kept, rewritten for
Go: packaging (no autoissues path or dependency in any product
package), store (RLS forced on every table, one writer, append-only
history), lifecycle (every transition in the table and no other),
interfaces (one JSON document, exit codes, signed webhooks), adapters
(fingerprint pairs, incomplete runs, flood cap), QA stack (stages never
overlap, the memory lines hold) and quota (cases AT-G01 to AT-G25).
The Forgejo link adds these: with Forgejo down, a claim still succeeds
and its task is made once Forgejo is back; a crash between the claim
and the task, and a repeated worker run, make exactly one task; a
failed close is retried until it lands; claim, release, claim again
gives exactly one task, open; a claim quickly followed by its release
closes the task; a close row retried after a later claim leaves the
task open; a gate close for a fault nobody claimed makes no task;
and a fault whose title, summary or rule holds a secret, a private
name, a Dell path or post text, including a rule shaped like a cloud
access key or a person's login name that is on no list, gives a task
holding none of them.
Each runs on Dell with made-up data, never on the owner's PC.

## Changes from the source

- Python, FastAPI, marimo and the Python test tools become Go and the
  tools pinned in `gate/tools.toml`.
- The store was already PostgreSQL with forced RLS (the owner's
  rewrite of 7 November 2026); it becomes PostgreSQL 18 on Dell under
  AGENTS.md section 5.2, and the old storage area goes.
- Podman, the QA user's slice and the 8 GB lab line become containers
  on Dell queued through pankti; the stage lines are kept.
- The GitHub publisher and its anchor log become Forgejo, with commits
  on `main` only (AGENTS.md section 9).
- Faults stay apart from Forgejo, linked to a task only when one is
  taken, by the owner's answers of 6 and 10 November 2026.

Source: `docs/specifications/autoissues/README.md`, `storage.md`,
`lifecycle.md`, `interfaces.md`, `adapters.md`, `qa-stack.md`,
`quota.md`, `quota-recovery.md`, `tool-support.md`, `acceptance.md`
and `acceptance-2.md` in the archive `archive-2026-11-08-old-repo`.
