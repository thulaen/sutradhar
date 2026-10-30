# SPEC-005 interfaces

Status: APPROVED. Part of [SPEC-005](README.md).
Approved by the owner on 30 October 2026 (see the approval record in
`docs/specifications/approvals-2026-10-30.md`); it supersedes any
"awaiting approval" wording below.

## CLI

`autoissues` is the console entry point of the importable `autoissues`
package (AT-I01). Every command accepts `--json` and then writes exactly one
JSON document to standard output, with a versioned schema in
`tools/autoissues/schemas/`; human text goes to standard error. Exit codes:
0 success, 2 refused by policy, 3 not found, 4 service unavailable, 5
invalid input (AT-I02).

| Command | Purpose |
| --- | --- |
| `next` | Highest-priority actionable issue |
| `list`, `show` | Query and inspect issues |
| `claim`, `start`, `release` | Take, begin or return a lease |
| `new` | File an agent finding |
| `comment`, `link`, `hotlist` | Annotate and relate issues |
| `fixed` | Submit a fix with lesson and evidence |
| `wontfix`, `dupe` | Request suppression or duplicate |
| `verify-run` | Verification pass, gate account only |
| `ingest` | Ingest one signed report set, gate only |
| `bottlenecks` | Bottleneck table |
| `export` | Data export and snapshots |
| `stats` | Counts and trends |
| `quota` | Read the current quota position |

`next` orders by priority, then SLA due date, then age, and filters by
component and type; it skips issues under someone else's live lease and
issues blocked by an open issue (AT-I03). Each command validates arguments
before contacting the service (AT-I04). Commands that change status follow
the [transition table](lifecycle.md#transition-table): for agents,
`fixed`, `wontfix`, `dupe` and quota-affecting `hotlist` changes are
requests that take effect only through the gate or an owner decision record.

## API

`tools/autoissues/api` is a small FastAPI application served by the
AutoIssues service only inside the QA network namespace; binding elsewhere
needs an explicit approved configuration change (AT-I05). It offers the
same read operations as the CLI and these webhooks:

- `POST /hooks/glitchtip`, `POST /hooks/alertmanager`,
  `POST /hooks/sonarqube`.

Each webhook needs a secret held only in gate configuration and an HMAC
signature where the sending tool supports one; it verifies the payload
schema and enqueues an ingest run; it never writes outside the single
writer. Unknown, oversized or unsigned payloads are refused and counted
(AT-I06). A webhook can report a signal but never proves its absence;
verification queries the source directly. Write operations other than
webhooks are not exposed over HTTP; agents use the CLI socket, which carries
operating-system identity.

## Dashboard

`tools/autoissues/dashboard.py` is a marimo notebook that reads through a
read-only connection. Views: open issues by component, priority, type and
age; SLO breaches; filed versus verified per week; regression count; the
bottlenecks table (AT-I07). It runs interactively with `marimo run` bound to
localhost, and headless with `marimo export html` for the nightly report;
a test renders both from a fixture database (AT-I08). It follows the
project's chart accessibility rules (R047, R048).

## Snapshots

`autoissues export --snapshot DIR` writes:

- `open.json`: every open issue with scrubbed evidence summaries;
- `summary.md`: counts by priority, type and component, SLO breaches and
  the needs-owner-decision hotlist;
- `bottlenecks.md`: the bottlenecks table.

Output is deterministic for a given database state, so unchanged data gives
identical files (AT-I09). The nightly job writes snapshots to a new
immutable directory on Dell. Publication into the repository's
`autoissues/` directory happens only inside a meaningful milestone commit,
after the privacy gate and review, through the main-only publisher; there is
no bot commit and no pull request (AT-I10, R095).

## Bottlenecks

`autoissues bottlenecks` and `bottlenecks.md` show:

- top 20 CPU functions from Pyroscope for nightly jobs and the soak, with
  change since the previous nightly;
- slowest 20 Tempo spans by p95, with trace links;
- slowest endpoints and transactions from GlitchTip and Prometheus with
  p50, p95 and p99;
- slowest 20 tests and slowest jobs;
- largest memory growth during the soak.

Each row over its threshold shows the issue identifier that already tracks
it; a threshold-crossing row without an issue is itself a defect
(AT-I11). Sources with no data yet show "no data yet", never zero.
