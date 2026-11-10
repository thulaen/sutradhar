# 0035: hisaab, books and reports

hisaab (हिसाब, "the reckoning") keeps Sutradhar's books. It runs only at
night, as a batch: one large job processed in a single run, never item
by item while people use the site. It owns four areas: exact decimal
money ledgers, reconciliations with control totals, retention
decisions, and formal reports and receipts. It decides and reports; it
never acts. It is roadmap Phase 5 item 4.

## What it does

### Terms

- Ledger: a book of money entries where every amount is recorded twice,
  as a debit and a credit, so the two totals must agree.
- Reconciliation: comparing two records of the same fact, kept by
  different parts, and listing every difference.
- Control total: a count or sum written at the end of a file, so the
  reader can prove nothing was lost or added.
- Retention decision: which stored items have reached the end of their
  allowed life and may now be deleted.

Example: the night after 2 March, Sutradhar exports the 41,200 paid
embedding requests of 2 March. hisaab posts them to the books: USD
12.348120 for site 1. It compares that figure with the live spending
counter used for the caps during the day, and they agree to the
millionth. The output file ends "41,200 records, total 12.348120" and
is signed. The consuming module checks the signature and totals and
stores the day's books; on 1 April the March statement is ready to
download. Had the counter disagreed, the Diagnostics screen would show
the difference and its size the next morning.

### Scope: the four areas

1. Exact decimal money ledgers, one per cost source: paid embeddings
   (spec 0025), cloud object storage and hosted search (spec 0038).
2. Reconciliation with control totals, one per pair of independent
   records that must agree exactly.
3. Retention decisions, one per class of stored data with a retention
   rule.
4. Formal reports and receipts, one per named audience and period.

Out of scope: live spending caps and every deletion (the consuming
modules); statistics, forecasts and cost models (ganit); screens (the
Dioxus app); metrics, traces and alerts (observability); search
signals (spec 0044).

The catalogue holds 6 programs, 40 jobs (H1 to H40) and 61 platform
pieces (HP1 to HP61), with 8 more jobs depending on details not yet
settled. There is no padding and no duplicate.

### The six programs

- Bahi ("account book"): the ledger.
- Dar ("rate"): rating requests against the price table.
- Milan ("matching"): matching two sorted files.
- Avadhi ("term"): retention decisions.
- Vivaran ("statement"): paged statements.
- Raseed ("receipt"): numbered receipts.

On screen they carry plain English names: ledger, rating, matching,
retention, statement and receipt.

### Boundaries

- Batch only. karya (the job system) starts hisaab in the nightly
  window. No page, request, review card or live suggestion ever waits
  for it.
- hisaab never writes Sutradhar's database. Its batch worker has no
  database login and writes only files in its exchange folder. The
  consuming module's ingest step writes the rows.
- hisaab never receives post text, credentials or personal data beyond
  ids, amounts, counts, dates and the minimum each job needs. Each
  record contract declares its fields, and a test refuses any other
  kind.
- The consuming module validates every result before using it, and
  carries out every action, deletions included. Validation checks form,
  totals and guards; it does not redo the decision, except that a guard
  such as the 90-day trash rule repeats the condition as a refusal, on
  purpose.
- The batch programs read no clock, no randomness and no environment,
  open no database or network, call only hisaab's own code, and take no
  file name or threshold as a literal.

### Records and contracts

1. Record contracts live in `contracts/hisaab/`, one per contract,
   named `<contract>-v<N>`, never edited once released. A change is a
   new version, read beside the old one until both sides move.
2. The reader and writer are generated from the contracts at build time
   by a Cargo build step. The generator refuses anything outside the
   allowed subset; a gate check fails when generated code is stale.
3. Allowed subset: fixed-length records; numbers as text digits with a
   separate leading sign; no binary, packed, floating or variable-length
   fields. A program may use other forms inside itself.
4. Files are record-sequential, one fixed record length per file,
   ASCII, with no line delimiters. Each record's first character is its
   type: `H` header, `D` detail, `T` trailer.
5. Header: contract and version, job, run id, site id (0 for all
   sites), business date, creation time (UTC), settings version and
   rule-table version.
6. Trailer: detail count, a hash total of the key ids (their sum modulo
   10^18), the signed sum of every amount field and the sum of every
   count field. Output trailers also repeat the input totals and carry a
   balance code.
7. Money: six decimal places (millionths), exact decimal arithmetic,
   never binary floating point, with the ISO 4217 currency code beside
   every amount and never converted between currencies. Each rounding
   point is named in its contract with its mode; rounding differences
   post to a rounding account, so totals still balance.
8. Rounding: each paid request is settled rounded up to the millionth,
   so a cap is never under-counted. Statements show the currency's
   smallest unit, rounded to the nearest with exact halves to the even
   digit, plus a rounding line so totals agree.
9. Dates: UTC timestamps and, where a rule needs one, the site's
   business date (its local calendar day). hisaab reads no clock and
   converts no time zone.
10. Field kinds are limited to id, amount, currency, count, quantity,
    date, timestamp, code, version and hash. Free text appears only in
    the release defects report's text record (H38), which holds no
    personal data.
11. No truncation: a value longer than its field is refused at export
    (SUT-867); a short record read in is a failure, never treated as
    complete (SUT-861).

### The night's run

1. Window: when ganit's night work has finished, or as ganit job 11's plan
   orders it, karya starts hisaab's batch worker. Backfill and rebuilds
   stay paused, as for ganit. hisaab never runs while ganit's batch work
   runs.
2. Export: each karya job of kind `hisaab.<job>`, in the maintenance
   lane with a concurrency group of one, writes its input files and a
   control record (job, business date, settings and rule-table
   versions, limits) to `runs/<run>/in/`, with a BLAKE3 hash per file.
3. Start: karya calls the runner's versioned internal API (`POST
   /v1/runs`, never on a network port), naming the job, the run id and
   the settings version, with the trace context. The runner derives
   every path from the run id; a caller never names a path or a program.
4. Run: the runner checks the input hashes, looks the job up in its
   fixed job-to-program table and starts the program directly, with no
   shell. The program first checks every input trailer against its
   records, then works, checkpointing after each chunk, and ends by
   writing its trailers.
5. Seal: the runner checks each output ends with a well-formed trailer
   (form, not meaning), hashes each file with BLAKE3 and signs the run
   manifest.
6. Follow: karya polls `GET /v1/runs/{id}` for records processed and total,
   feeding the progress system (spec 0016), and passes pause and resume
   on.
7. Validate: karya checks the signature and hashes, parses every record
   with the generated reader, and checks totals, balancing, that every
   id in an output was in the input, value ranges and the job's guards.
   Any failure refuses the whole run, keeps the last good output and
   files an AutoIssue.
8. Ingest and act: the consuming module records the outputs in
   append-only tables (row-level security, written only by the ingest
   role) and acts: deletes on effective dates, shows statements, serves
   documents. Each step is an audit event.
9. Report back: for decision jobs, the consumer writes what it executed
   and what it refused, with reasons; the next night's run reads it,
   and H40's receipts use it.
10. Window end: karya stops the batch worker. A running job pauses at
    its next checkpoint and resumes the next night.

### Balancing rules and signatures

Every contract states its balancing rule, and both sides check it:

- Ledgers: journal total equals posted plus suspense (records hisaab
  could not post, with reasons); debits equal credits for each site,
  ledger, currency and day.
- Reconciliations: left records equal matched plus left-only; right
  records equal matched plus right-only; every difference shows both
  values.
- Retention: inventory equals due plus not due plus held; the night's
  decisions equal executed plus refused plus deferred.
- Signatures: BLAKE3 per file; the manifest signed with HMAC-SHA-256
  (`hmac` and `sha2`), with a key purpose held by tijori and given only
  to the batch worker and the verifier. A missing or wrong signature
  refuses the run.
- Every amount carries its sign: refunds and credits are negative
  entries, never hidden netting.

### Checkpoint and restart

1. Each program reads its sorted input in chunks
   (`hisaab.chunk_records`). After each chunk it closes that chunk's
   outputs and writes a checkpoint: chunk number, input position, last
   key, running state (such as the open account's balance) and partial
   totals.
2. On restart the runner re-checks the input hashes; a changed input
   refuses the run (SUT-860). The program re-reads to the checkpoint and
   checks the partial totals; a mismatch discards the checkpoint and
   starts again (SUT-868). Otherwise it discards uncommitted chunk files
   and carries on. At most one chunk is repeated.
3. Pause: at the window's end, or when karya's yield signal says live
   work needs room, the runner leaves a stop marker; the program stops
   at its next checkpoint with a "paused" code.
4. A worker killed mid-run loses at most one chunk, and the final
   outputs equal an uninterrupted run's byte for byte.

### What moves to hisaab

Nothing happens twice. Each row moves one responsibility to hisaab.

| Id | Responsibility | From | To |
| --- | --- | --- | --- |
| S01 | Paid spend books, closed days and months | paid embeddings | H1-H3 |
| S02 | Counted against reported tokens, 2% warning | paid embeddings | H9 |
| S03 | Send-log reduction to daily totals | paid embeddings | H28 |
| S04 | Cloud month-to-date cost (size x price) | search and storage | H4-H6 |
| S05 | karya's retention sweeps | jobs | H17-H20 |
| S06 | Progress history retention | progress | H20 |
| S07 | Trash purge selection | undo and trash | H21 |
| S08 | Rejection-reason expiry | matching | H22 |
| S09 | Hint trash stages | moderation hints | H23 |
| S10 | Rejected-draft purge selection | content drafts | H24 |
| S11 | darshak 90-day and two-year selection | darshak | H25-H26 |
| S12 | Connector two-year selection | connectors | H27 |
| S13 | Action-run expiry | action layer | H29 |
| S14 | Extension-event expiry | extensions | H30 |
| S15 | Export-file expiry and orphans | audit and exports | H31 |
| S16 | Erasure-list pruning | audit and exports | H32 |
| S17 | The formal erasure receipt | audit and exports | H39 |
| S18 | Release defects report | release | H38 |

What stays where it is:

- S01, S02: the live journal (formerly called the live "ledger"),
  reservation, settlement, "use the higher count", caps, alerts, the
  hard stop and the fallback stay live. hisaab's books are for closed
  periods.
- S03, S05 to S16: every deletion, its guards (the proved "never under
  90 days" trash rule among them) and the screens showing days left
  stay with their modules. Erasure stays immediate and never waits for
  a night.
- S04: quotes before provisioning stay; the budget warnings of spec
  0038 are raised from the books' figure; work continues as before.
- S17: the audit module keeps the erasure job and the live status of
  every target; hisaab issues the numbered receipt the next night.
- S18: the release gate keeps the check; hisaab produces the report, in
  a nightly window during the release candidate's 72-hour soak on Dell
  (the only time Dell has such a window), and the publisher checks its
  totals.

Not moved: the 30-day recalculation of suggestions (ranking); site,
index and search reconciliations (they repair live state); the audit
chain check and the uninstall receipt; quotes and forecasts; statistics
and cost models (ganit jobs such as 14, 201, 238 to 250, 323, 346 and
351); metrics and trace retention; verified upload before local
deletion and the provider's lifecycle rules; darshak's day close; the
add-ons' own logs and notes; screens; daily mission reports.

### Retention rules and the provider's bill

- For the seven data classes with no retention rule yet (W1 to W7), one
  rule per class is drafted for the owner's approval, starting with
  backups: keep 7 nightly, 4 weekly and 3 monthly, plus each
  pre-upgrade backup until the next successful upgrade. The classes,
  each with the part that keeps it:
  - W1: backups (the backup store, spec 0030);
  - W2: support-bundle files and their previews (spec 0032);
  - W3: visual datasets: weekly snapshots, daily reports, the error log
    and the feed buffer (spec 0017);
  - W4: doctor runs and resolved alerts (spec 0032);
  - W5: the connector pull log (spec 0040);
  - W6: darshak's per-thread features and salt events (spec 0034);
  - W7: expired action plans and agent tokens, whose validity is set but
    not how long they are kept (spec 0020).
- Checking the books against the provider's bill (W8): the
  administrator types each month's bill total from the provider's
  console, and hisaab compares it with the books. No new network call
  or account permission.

### Failures (SUT-860 to SUT-869)

- SUT-860 input refused before the run (hash, totals or schema).
- SUT-861 program failed, or ended without its trailers.
- SUT-862 output refused by validation; the last good one is kept.
- SUT-863 a guard refused a decision; nothing was deleted for it.
- SUT-864 reconciliation break, shown with its size and age.
- SUT-865 job overdue beyond its lookahead; an AutoIssue is filed.
- SUT-866 run paused at the window's end; it resumes next night.
- SUT-867 a value too long for its field; the export was refused.
- SUT-868 checkpoint mismatch on restart; the run starts again.
- SUT-869 hisaab unavailable; the last good books are kept.

## Limits

- hisaab's batch worker has a proposed limit of 256 MiB, measured on
  Dell before admission: the runner (about 16 MiB), the running program,
  sort memory capped at 64 MiB, and file buffers.
- It runs in spec 0024's heavy job slot and never while ganit's batch
  work runs; ganit's share of the slot (at most 0.8 GB on a PC under
  16 GB) is larger, so the night's peak does not rise. hisaab adds
  time, not memory. ganit job 11 places hisaab's jobs in the night; job
  238 counts hisaab's measured peak in the settings estimator.
- All of this sits in the service slice, within the memory limits in
  AGENTS.md section 5.6, with 15% headroom.
- Ingest streams no more than 16 MiB at a time, inside the service's
  budget.
- The exchange folder counts towards the disk budget; run folders are
  removed after verified ingest.
- On Dell, every heavy build, test, coverage, mutation and benchmark
  run goes through the heavy-job queue.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Rust tests unless named. Each test fails before its code, and every new
test passes twenty runs.

Records and contracts:

- AT-HS01 (unit and property): every contract round-trips random valid
  records exactly; a contract using a field form outside the subset
  fails generation, naming it.
- AT-HS02 (contract): an output whose trailer differs from its records
  by one millionth is refused with SUT-862; the last good output stays
  and an AutoIssue is filed.
- AT-HS03: a free-text field outside H38's text record fails the
  data-minimisation test.
- AT-HS04: a value one character too long is refused with SUT-867;
  nothing is truncated.
- AT-HS05: an input with a short record ends with SUT-861 and is never
  treated as complete.

Running:

- AT-HS06: a job due at noon starts only in the nightly window, after
  ganit's batch work has stopped, and the worker stops at the window's
  end (fake clock).
- AT-HS07 (chaos, nightly): a run of 1,000,000 records killed at random
  points and restarted repeats at most one chunk, and the outputs equal
  an uninterrupted run's byte for byte.
- AT-HS08: the window ending mid-run stops at the next checkpoint with
  "paused"; the next night resumes, shown as "resumed from checkpoint".
- AT-HS09: requests naming an unknown job, a path or a program are
  refused and start nothing.
- AT-HS10: an output changed after sealing fails signature checks with
  SUT-862 before any record is parsed.
- AT-HS11 (memory, nightly on Dell): with the largest fixtures the
  worker's peak stays under its limit, and it never runs with ganit's
  batch work.

Programs:

- AT-HS12 (property, Bahi): with random journals, refunds and several
  currencies, debits equal credits for every site, ledger, currency and
  day, and journal total equals posted plus suspense.
- AT-HS13 (Dar): with a price change at 14:00, each request uses the
  price in force at its time, the higher of the read and entered prices,
  and the stated rounding; a request with no price goes to suspense
  with its reason.
- AT-HS14 (property, Milan): matched plus left-only equals the left
  count, matched plus right-only the right count, each difference shows
  both values; unsorted or duplicate keys are refused.
- AT-HS15 (property, Avadhi): no decision names an item younger than
  its rule, a held item or one missing from the inventory; "30 days and
  at least 200 per kind" keeps the newest 200 of every kind.
- AT-HS16 (Kani proof and unit): the trash guard never lets an item
  with under 90 days in the trash be deleted, on this new path too.
- AT-HS17 (Vivaran): page totals, control breaks and footings equal the
  ledger's totals, and the summary record repeats them.
- AT-HS18 (property, Raseed): receipt numbers are unique and gapless
  across restarts, every check digit verifies, and a voided number
  stays listed as void.

Jobs:

- AT-HS19 (H1, H7): posted books equal the live counter for each cap to
  the millionth; a settlement missing from the counter is listed as a
  break and the Diagnostics check fails with SUT-864.
- AT-HS20 (H9): a provider reporting 3% more tokens than counted raises
  the warning once, and the live settlement already used the higher
  figure.
- AT-HS21 (H28): send-log rows older than retention plus 30 days are
  replaced by daily totals equal to their sum, and an erasure receipt
  for an affected item still answers correctly.
- AT-HS22 (shadow, every moved responsibility): the old path and the
  hisaab job give identical results for the set number of nights before
  the old path is removed; any difference stops the move.
- AT-HS23 (H16, H25): a day whose aggregates differ from its visits by
  one is not decided for deletion and the break is reported; other days
  proceed.
- AT-HS24 (H27): only copied rows two years and one day old are
  decided and deleted, and coverage shows those dates as removed by
  retention.
- AT-HS25 (chaos): with hisaab down three nights, deletions already
  decided proceed on their dates, nothing undecided is deleted, and
  SUT-865 files an AutoIssue once past the lookahead.
- AT-HS26 (H39): the night after an erasure, the receipt lists every
  registered target with result and counts, the paid-embedding sending
  record and a gapless number, with no personal data; the live status
  showed at once.
- AT-HS27 (H38, on Dell): the report lists every defect since the last
  release with cause and fix, and its totals equal the publisher's count
  of commits and "Defects:" sections; a commit without the section makes
  the report refuse and the release gate block.
- AT-HS28 (load, nightly): one million paid requests and 50,000
  retention items finish within hisaab's share of the window on Dell and
  under its memory limit.

Also:

- Reproducibility: two builds give identical binaries, and two runs on
  the same inputs give identical outputs and stamps.
- Property tests: balancing, permutation of equal keys, idempotent
  re-runs and restart equivalence.
- Edge cases for every job: empty input, one record, all rejected,
  largest amounts, negative entries, 29 February, month and year ends,
  duplicate ids and unsorted input (refused).
- Oracles: an independent implementation of each program's arithmetic
  and matching, in tests only.
- Fuzzing: the generated reader and the output validator, which read
  untrusted files. Malformed and hostile files go into every program,
  which must refuse them with a code, never crash or write a partial
  trailer.
- Rounding modes have known-answer tests.

## Changes from the source

- The batch programs are rewritten as Rust, in one batch worker under
  the service, not a separate container.
- Record contracts are files read by a Cargo build step; the old
  language's gate support, compiler, test tools and generated-code rules
  are dropped.
- The signing key is held by tijori.
- Memory follows the limits in AGENTS.md section 5.6, not the old 5.6 GB.
- Screens are the Dioxus app.
- The recommended answer to each open question is taken: the six
  names, next-night receipts, the typed bill total, hisaab after ganit,
  the rounding rules, the S01 to S18 moves and the backup rule.

Source: `docs/specifications/hisaab/README.md`,
`docs/specifications/hisaab/hisaab-2.md`,
`docs/specifications/hisaab/hisaab-3.md`,
`docs/specifications/hisaab/hisaab-4.md` and
`docs/specifications/hisaab/hisaab-5.md` in the archive
`archive-2026-11-08-old-repo`.
