# SPEC-011 part 3: operations, audiences, enforcement and safety

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in the
[specification decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Part of [SPEC-011](progress.md). Requirements: R110 and ADR-0004;
R009, R020, R022, R023, R027, R044, R051, R058, R067, R072, R087,
R119, R123, R127, R128 and R129; ADR-0006.

## Operations that must use it

| Operation | Unit and total | Cancel | Owner |
| --- | --- | --- | --- |
| Model downloads | bytes, from the lock files | yes | SPEC-010 |
| Model verification | bytes hashed | yes | SPEC-010 |
| Self-test | steps, one per model | yes | SPEC-010 |
| Model trials | items: sample links; stage per model | yes | SPEC-010 |
| Thread-level index (R123) | items: threads | yes | SPEC-008 |
| Index builds and rebuilds | items: posts | yes | SPEC-008 |
| Incremental embedding | items: queued posts | yes | SPEC-008 |
| Initial import (R107) | items: eligible posts; pauses | yes | SPEC-008 |
| Per-post analysis | items: pairs reranked, of 50 | no | SPEC-009 |
| Link-graph rebuilds | items: threads | yes | SPEC-009 |
| Other batch jobs | their natural count | declared | its spec |
| Database migrations | steps, one per migration | no | F021 |
| Backups | bytes | yes | F021 |
| Restores | bytes | no | F021 |
| Export | items or bytes | yes | F022 |
| Uninstall | steps | no | F021 |
| Evaluation runs | items judged; batch stages | yes | F023 |
| Judge job | steps: batches; pauses at limits | yes | F023 |
| AutoIssues ingestion | steps, one per adapter | yes | SPEC-005 |
| Snapshot export | items | yes | SPEC-005 |
| Review bulk actions | items: suggestions | yes | F009 |
| `sutra doctor` | steps: checks | yes | F022 |
| Nightly run | staged: overall and stage bars | per stage | F003 |
| `just qa` | staged: overall and stage bars | per stage | SPEC-005 |

Notes on the table:

- Totals are determinate wherever a count or byte size is knowable.
  Posts that arrive during incremental embedding raise the total and
  mark it revised. Skipped posts are counted by reason (SPEC-008).
- The thread-level index, read by bge-m3 from each thread's title and
  summary, is built first; the initial import (the backlog run) then
  fills the per-post index in the background in order of thread views
  and recency (R123; SPEC-008 and SPEC-009 own both).
- A model trial runs each sample link through both models at once, so
  its two child handles, one per model, advance side by side
  (SPEC-010; SPEC-012 item 40).
- Per-post analysis is SPEC-009's reranking of up to 50 pairs for one
  post; the row shows "k of 50 reranked" and the methods still pending.
  It stops only with its post's work, never by a cancel button.
- Partial downloads are kept for resuming; uninstall removes them with
  the model files (R108). A cancelled backup is marked incomplete and
  is never offered for restore.
- The judge job pauses with the time it will resume when a usage limit
  is hit, then continues (R065).
- "Nightly" covers both the product's scheduled nightly run and the
  Dell nightly checks (R066); both are staged operations.
- `just qa`, the Dell nightly and AutoIssues run on Dell and use the
  installed, published copy of this module, never the candidate's
  ([trust boundaries](../trust-boundaries.md)). They have command-line
  and log views only, with history in the tools' own store on Dell.
- Review bulk actions belong to F009, which decides how they run; they
  report here in the unit shown.

## Who sees what

Each job registration declares its audience, stored in the record:

- Administrators with Sutradhar's progress permission (the scope
  `progress.view`, granted from XenForo administrator permissions) see
  every operation.
- Anyone with SPEC-010's "View models" permission sees model
  operations: downloads, verification, self-test, trials, activation
  and rollback.
- Anyone with permission to use the review page (R009) sees per-post
  analysis rows, including analysis the system started by itself, and
  bulk actions on the review page.
- The person who started an operation always sees it.
- Item identifiers in failure lists are shown to administrators only.

## Enforcement

1. Every job registered with karya runs with a handle that karya
   creates. Registration requires the unit, whether the total is
   knowable, whether cancelling is safe and the audience.
2. Every `sutra` command declares whether it can run longer than two
   seconds (P-PG07). Such a command must create a handle. During the
   integration suite a timing guard fails any command that ran longer
   than two seconds without a handle, and any declared long command
   that created none.
3. A test runs each registered job on the synthetic fixture forum
   (about 300 threads, R058) with a recording listener. It asserts a
   start event, at least one advance, a finish event and `done` equal
   to `total` at the end. No job is skipped. Jobs that use the network
   or an outside provider (model downloads, the Hugging Face Hub, judge
   providers, AutoIssues adapters) run against recorded fakes: stored
   responses replayed without a connection. A job with no work on the
   fixture is a gap in the fixture and fails the test.
4. Fake-clock tests cover monotonic progress, the indeterminate
   fallback, seeding from history and resuming from a checkpoint, plus
   the update limit, stage weights, side-by-side child handles and
   paused time.
5. pytest shows progress with pytest-sugar in interactive terminal runs
   on Dell, and plain output in CI, in the gate and in every run that
   does not write to a terminal (P-PG08). All work is on Dell; the
   Windows computer only runs the app, and nothing runs on it (R072,
   R119).

## Failure behaviour

- Reporting never breaks a job. Outside strict tests, an error in the
  progress system is logged once per operation with an error code and
  the job carries on.
- State store unavailable: the latest update is kept in memory and
  retried with growing pauses; a warning is logged at most once a
  minute. Progress inside a checkpoint shares the checkpoint's
  transaction, so the two never disagree; a failed checkpoint is
  handled by karya (F003).
- Worker killed or service restarted (R023): when the service starts,
  records whose worker is gone become paused with the reason
  "interrupted, resumes from checkpoint" if the job is resumable, or
  failed with a reason if not. No record stays "running" for ever.
- Work refused before it starts, for example under memory pressure
  (R023): a record is still created, failed, with the plain reason, so
  the person who asked can see why.
- Stall: no change for five minutes shows "no progress for 5 minutes"
  and a health warning. The system does not kill the job by itself.
- Clock changes: estimates use the monotonic clock, so a change of
  system time does not distort them.
- Live channel problems: the page falls back to polling; if polling
  also fails, the bar shows when it last updated and is marked stale,
  never shown as current.
- Expired viewer token: the stream closes; a visible, active page
  renews the cookie and reconnects. If renewal fails, for example
  after signing out, the page says "sign in again to see live
  progress".
- Too many streams, or no HTTP/2: pages poll.
- Cancel not finished within ten minutes: a warning, never a forced
  kill that skips recovery.
- Notification not delivered: retried with growing pauses for up to 24
  hours and listed on the health page; the job is unaffected.
- Bundled Prometheus server or OpenTelemetry collector stopped, or at
  its memory cap (R127, R121): jobs and progress views carry on, since
  they read the state record; scrapes are missed, the service's
  bounded span queue drops spans rather than grow, and the health page
  says monitoring is unavailable.

## Privacy and security

- Records, logs, gauges, spans and notifications hold job names,
  counts, durations, error and reason codes and item identifiers; never
  post text, titles or member names. `fail_item` messages pass through
  the same scrubber as logs.
- Audiences are enforced by the service from the token's scopes, not
  by the page.
- Viewer tokens: an HttpOnly cookie limited to the live path; a key
  used for nothing else; five-minute expiry; renewal only while the
  page is visible and in use; compared in constant time (a comparison
  that does not leak timing). They never appear in addresses or logs.
- The forwarded path reaches only the allow-listed GET routes; it
  strips signature headers sent by browsers; the service refuses an
  unsigned request before any work; responses are never cached.
- The live channel is same-origin and adds no cross-origin access.
- Cancelling uses XenForo forms with CSRF protection, the action
  layer's permission check and an audit record.
- Gauge labels carry only job keys, operation ids and units. Metrics
  and tracing are on by default (R127); they carry only IDs, counts and
  durations, which ADR-0006's test enforces, and every gauge appears on
  its "what we collect" page. `/actuator/metrics` is served only on the
  private address and is not on the live path's allow-list.
- Dell-side tools keep their progress history in Dell storage; nothing
  is stored or run on the Windows computer, which only runs the app
  (R119).

## Admission records needed

Each library is used only after its admission record is approved (R027,
R067). If one fails its review, the feature that needs it is blocked
and a replacement is put to the owner (D-PG06). A record names the
pinned version, the licence as read
from the project's own files, the rights and patent review, where the
library runs, its memory cost (for SPEC-010's memory table) and support
for the chosen Python version. Licences below are expectations to
confirm.

- `rich` (expected MIT): terminal bars in the `sutra` command.
- `sse-starlette` (expected BSD-3-Clause): the stream route. It builds
  on Starlette, which the service framework's own admission covers.
- The Prometheus client, `prometheus-client` (expected Apache-2.0):
  gauges in the production service, always on (R127).
- The Prometheus server (expected Apache-2.0), bundled as a pinned
  container image in the default docker compose file (R129) with its
  own memory cap (R121): scrapes the gauges and keeps them 15 days
  (ADR-0006). The OpenTelemetry collector is admitted under SPEC-012.
- marimo (expected Apache-2.0): the notebook bar, in the lab only and
  never in production (R026).
- pytest-sugar (expected BSD-3-Clause): test progress in Dell developer
  runs only, never shipped. Its behaviour with pytest-xdist, which R076
  adopts, is checked before admission.
- Browser test tools (P-PG06), used only on Dell and in cloud workers
  and never shipped: Node.js with its built-in test runner and
  coverage; Playwright; and axe-core, whose licence is MPL-2.0 (a
  file-level copyleft licence), recorded explicitly as a test-only tool
  that is never copied into the product.
