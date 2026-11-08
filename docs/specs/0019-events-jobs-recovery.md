# 0019: Events, jobs and durable recovery

Most of Sutradhar's work runs in the background: reading a new post,
rebuilding an index, nightly checks, the weekly cache sweep. This part
says how that work is queued, shared between workers, saved as it goes
and picked up again after a crash, without being lost or repeated.
It is the events and jobs starters of Sutradhar Boot: sanchar
("communication") for events and karya ("work") for the queue. It is
roadmap Phase 3 item 6.

Example: a worker is reading post 41,200 of 120,000 when the service is
killed. Within a minute a worker takes the job again, starts from the
restart point saved at post 41,000, and the progress bar says "resumed
from checkpoint at 41,000; about 200 posts repeated". No post enters
the index twice, because each write is keyed by the post and its
version.

## What it does

### Words used here

- **Event**: a short record that something happened, such as "post 123
  was saved". It carries ids and counts, never post text.
- **Job**: one unit of queued work. A **handler** is the code for one
  kind of job.
- **Lane**: a queue with its own share of workers: `live` (new and
  edited content, always first), `backfill` (the old catalogue, lowest
  priority) and `maintenance` (scheduled jobs and sweeps).
- **Lease**: a worker's time-limited claim on a job, renewed by a
  heartbeat. If the heartbeat stops, the lease runs out and another
  worker may take the job.
- **Fence**: a number that changes at each claim. A worker whose lease
  ran out holds an old fence, and its writes are refused.
- **Checkpoint**: a restart point, saved in the same transaction as the
  work it covers.
- **Dead letter**: a job put aside after a failure that retrying will
  not fix. It waits for a person.

### Scope

In scope: events, the queue, leases, checkpoints, retries, dead
letters, schedules, batch work, lanes and yielding, the memory
watchdog, graceful shutdown, the trigger for ganit's batch jobs, their
management API routes, the `sutra karya` and `sutra sanchar` commands
and their tables. Out of scope: the progress handle and its views (the
progress part, which this feeds); what each job does; how forum changes
become events (the site link part); ganit's own engines, parameter
registry, numerical guards and the shadow test and activation of its
outputs.

### Shape

1. One queue serves everything. An event is delivered by turning it into
   one job per subscriber, in the same transaction that saves the event.
   Scheduled work is a job made by the scheduler. Batch work is a job
   that checkpoints. So leases, retries, dead letters, progress and
   recovery are written once.
2. sanchar depends on the module registry; karya depends on sanchar and
   the registry; the extension broker depends on karya. Neither knows
   any feature.
3. Traits, each with a test fake:
   - `EventStore` and `JobStore`: PostgreSQL through sqlx; in-memory
     fakes with failure injection (lost connection, slow statement,
     deadlock).
   - `JobHandler`: one per job kind, registered in the composition
     root. `spec()` gives the kind's name (prefixed by its module),
     lane, role, capability id, whether it can be cancelled, its time
     limit, its retry rule and its needs; `run(cx)` does the work.
   - `Clock`: real or moved by hand, for leases, backoff and schedules.
   - `MemoryGauge`: reads the slice's memory use and limit.
   - `YieldSignal`: whether background work should step aside, and
     why; built from the live lane's age, pressure readings and the
     forum database load reported by the site link.
   - `GanitBatch`: exports a job's snapshot, starts the run, follows it
     and collects its versioned output; the fake is a scripted ganit.
4. A handler's `JobContext` holds: the payload; its settings snapshot,
   taken at claim and kept to the end; its progress handle, restored
   from the checkpoint; the last cursor; `commit`, the only way to save
   work; the cancellation token; the current chunk size, which the
   watchdog may shrink; and its fence.
5. Roles: `worker` runs live, backfill and maintenance jobs that need no
   models; `inference` runs jobs that need models (only it loads
   models; others send it work); `scheduler` makes scheduled jobs,
   re-queues expired leases and runs nothing heavy; `serve` publishes
   events and makes jobs but runs none. The desktop app runs no jobs.

### Events (sanchar)

1. An event has: `id` (UUID version 7, ordered by time), `kind` (an
   English dotted name such as `content.post_saved`), `schema_version`,
   `key` (such as `post:123`), `occurred_at`, `source` (`forum` or
   `service`), the trace context, and a JSON payload of ids, versions
   and counts, at most 4 KiB.
2. `publish(tx, event)` writes the event and one job for each switched-on
   subscriber, inside the caller's transaction. A subscriber whose
   module is waiting or faulty still gets its job, which waits
   unclaimed, so nothing is lost.
3. Events are hints; the database is the truth. A handler always reads
   the current state of what the event names before acting. So repeats
   and out-of-order events are harmless: a "post saved" handled after
   the post was deleted finds it deleted and acts on that.
4. Inside one role, a Tokio broadcast channel passes events to
   listeners such as the progress stream. It is a convenience, never
   the record. Other roles read the `event` table by id.
5. Modules declare event kinds with their schema version. A newer
   schema than a handler knows makes its job wait as "needs upgrade",
   never fail.

### Claiming and leases

1. Each worker role runs `karya.worker.slots` slots (default 4), of
   which `karya.worker.live_slots` (default 1) take only live jobs, so
   a long backfill never holds every slot.
2. A free slot claims in one short transaction: the oldest claimable
   job of the best lane it may take (live, then maintenance, then
   backfill), by priority then id, with `FOR UPDATE SKIP LOCKED` so
   workers never wait for each other. It sets state `claimed`, the
   lease owner, a new fence and a lease expiry 30 seconds ahead.
3. Slots wake on PostgreSQL `NOTIFY`, sent in the transaction that makes
   a job, and otherwise poll every second, so a lost notice delays work
   by at most a second.
4. A job is claimable only if its `not_before` has passed, its module is
   runnable, no other job with the same key is claimed, its role
   matches, and the safe-running promise and wait checks pass.
5. A heartbeat every 10 seconds extends the lease. A heartbeat refused
   because the fence moved means the lease is lost: the job is
   cancelled at once and writes nothing more.
6. If a worker cannot renew for 20 seconds, it cancels the job itself
   before the lease runs out.
7. The scheduler re-queues expired leases every 5 seconds. That counts
   an interruption, not a failed attempt: infrastructure trouble never
   leads to a dead letter.
8. Each worker has a stable name and a boot number. On start it frees at
   once any lease held by an earlier boot of the same name.

### Saving work

1. `cx.commit(|tx| async { ...; Ok(cursor) })` is the only way to save
   work. In one transaction karya runs the handler's writes; writes the
   checkpoint (cursor, chunk number, progress snapshot); checks the
   fence still matches (`UPDATE job ... WHERE id = ? AND fence = ?`),
   rolling everything back and stopping the job if not; and commits.
2. So a checkpoint is never ahead of its work, and a worker that lost
   its lease never overwrites the new owner's work.
3. Work outside the database is idempotent and keyed: index files are
   written by item id, content version and model key; calls to a site
   add-on carry an idempotency key of job id and chunk number, which the
   add-on records.

### Retries and dead letters

1. Handlers return one of three error kinds, each with an SUT code:
   - **Transient** (time-out, lost connection, provider rate limit):
     retried with backoff from 1 second, doubling, with jitter, capped
     at 10 minutes, for as long as it takes. Never a dead letter. A
     provider's own wait time is used as the delay.
   - **Permanent** (bad input that will not change): retried at most
     `max_attempts` times (default 5) with backoff, then dead-lettered.
   - **Defect** (an internal error that should never happen):
     dead-lettered at once, with a fault record written to the held
     faults file (0018, item G).
2. A dead job keeps its payload, attempts, last error and trace.
   `replay` makes a new queued job with the same payload and key,
   marked `replay_of`. Since handlers are idempotent and read current
   state first, replaying twice is safe. Replays go through the action
   layer and are audited.
3. A job that keeps killing its worker never reaches a failed attempt.
   After 5 such crashes within one hour it is held with SUT-240, a fault
   record is written, and an administrator may release it. Safe-running
   stops count towards this.

### Bounded chunks and recovery

1. Batch handlers work in chunks and commit after each. karya sizes
   chunks from measured throughput to take about 10 seconds
   (`karya.chunk.target`) and never more than 30 (`karya.chunk.max`).
   After any crash, at most one chunk per worker is repeated, inside the
   one-minute limit.
2. Batches over a changing table page by id cursor, never by offset; the
   cursor is the checkpoint.
3. After a kill, a restarted worker frees its old leases at once;
   otherwise the scheduler re-queues them within 35 seconds. Every job
   resumes within 60 seconds, with nothing lost and nothing repeated.
4. A resumed job's progress says how many items will be repeated.

### Scheduled work

1. The scheduler wakes at each schedule's time. A module declares a
   schedule: name, cron expression, time zone, lane, overlap rule and
   catch-up rule; settings may change it within its declared limits.
2. On waking it inserts the fire into `schedule_fire`, keyed by schedule
   and due minute in UTC. Only the first insert succeeds, so two
   schedulers (as during an upgrade) fire once between them.
3. If the previous run has not finished, the fire is recorded as
   "skipped: previous run still running" (SUT-241).
4. Fires missed while the scheduler was down give at most one catch-up
   run on start; a schedule may declare "skip" instead.
5. Schedules use UTC unless they name the owner's time zone
   (`karya.time_zone`, for quiet hours). A local time that does not
   exist when clocks go forward fires at the first minute after it; a
   local time that happens twice fires once, at the first.

### Yielding to live work

1. Backfill and maintenance executors check `YieldSignal` between chunks
   and pause, with a plain reason on their progress, while any of these
   holds: a live job has waited more than 2 seconds; CPU, memory or
   input-output pressure is over its setting (read from the slice's
   control group); the site link reports the forum database slow; an
   administrator paused the backlog (`karya.backfill.paused`, a live
   setting).
2. They resume when the signal has been clear for 60 seconds. Pausing
   only at chunk boundaries never leaves half-saved work.

### The memory watchdog

1. Every second each role reads its memory use against its own budget:
   its row in the memory table of spec 0024. These steps are for one
   role. The service slice as a whole has its own soft line and steps,
   also in spec 0024, and both apply.
2. At 80%, chunk sizes halve, and halve again at each further 5%, down
   to one item.
3. At 90%, the role claims no new work in any lane; running jobs finish
   their chunk; progress and health say "paused: memory at 91% of
   budget" (SUT-237).
4. Below 75%, it resumes; the gap stops it flapping.
5. The watchdog follows the one shed and stop-one-job order in the
   safe-running part.
6. A watchdog cannot prove graceful behaviour alone, so tests also kill
   the service at its hard limit and check recovery.

### Graceful shutdown

On SIGTERM the root cancellation token is cancelled: slots stop
claiming; running jobs see it at their next safe point, commit or roll
back the current chunk and hand the job back as queued with no attempt
counted; the task tracker waits for them. karya aims to stop within 20
seconds, inside a 30-second systemd stop timeout. A job that cannot stop
safely finishes its chunk.

### Ganit's batch jobs

1. ganit never schedules itself. Every ganit batch job is a karya job of
   kind `ganit.<job>` in the maintenance lane, declared in ganit's job
   registry with its inputs, schedule, consumer and capability id, and
   run one at a time (a concurrency group of one).
2. Batch runs happen only in the nightly window. At its start karya
   pauses backfill and rebuilds, shrinks the inference worker for the
   night and starts ganit's batch runner; at its end, or when no
   `ganit.*` jobs are left, karya stops the runner and resumes the
   paused work. A job due outside the window waits for the next one; one
   still running when the window closes pauses at its checkpoint and
   resumes the next night.
3. Each job runs in steps:
   1. **Export**: write the job's snapshot to the shared data folder in
      streamed batches of at most 8 MiB. Snapshots hold ids, numbers and
      word counts only, never post text.
   2. **Start**: ask the runner through its versioned interface, naming
      the job, snapshot, settings version and a run id, with the trace
      context.
   3. **Follow**: poll its status, feed its progress into the job's
      progress handle, and pass on pause and resume when `YieldSignal`
      says live work needs room. The job heartbeats its own lease;
      ganit keeps its own checkpoints, so a resumed run continues.
   4. **Collect**: ganit writes its versioned output (parameters,
      models, scores or forecasts, with its reproducibility stamp) to
      the folder; it never writes Sutradhar's database. karya checks it
      against its schema and the numerical guards (no NaN, infinity or
      out-of-range value), then hands it to the consumer, a job of the
      consuming module, which records it, shadow-tests it, and
      activates it or keeps it in shadow. Activations and rollbacks are
      audited.
   5. **Fail**: a time-out, failed run, or schema or guard failure is a
      transient or permanent error with SUT-242. Consumers keep the last
      good output; an overdue job writes a fault record. Live
      suggestions never wait for a batch run.
4. ganit's live calls (such as settings what-ifs) are not karya jobs.

### Data

In Sutradhar's own PostgreSQL, never the forum's database: schemas
`sanchar` and `karya`, English names, times as `timestamptz` in UTC,
JSON as `jsonb`, short fixed sets as checked `text`. Every statement
has `statement_timeout` and `lock_timeout`. Every row carries its site.

- `event`: `id`, `kind`, `schema_version`, `event_key`, `source`,
  `occurred_at`, `traceparent`, `payload` (at most 4 KiB, checked).
  Indexes on (`kind`, `id`) and (`event_key`, `id`).
- `job`: `id` (identity); `kind`; `module`; `lane`; `role`;
  `concurrency_group` (null, or a name allowing one running job);
  `priority` (0 first; deletion and erasure use 0); `job_key`; `state`
  (`queued`, `claimed`, `succeeded`, `cancelled`, `dead`, `held`);
  `held_reason`; `payload` (at most 64 KiB); `event_id`; `replay_of`;
  `attempts`, `max_attempts`, `interruptions`; `not_before`;
  `lease_owner`, `lease_boot`, `fence`, `lease_expires_at`,
  `heartbeat_at`; `settings_version_id`; `capability_id`;
  `error_code`, `error_class`; `created_at`, `claimed_at`,
  `finished_at`; `traceparent`; plus the safe-running waits and
  promises.
- A partial unique index on `job_key` where `state = 'queued'`. A
  second queued job for the same key updates the first one's payload
  (`ON CONFLICT ... DO UPDATE`, keeping the newest version), so a storm
  about one post makes one job. Claim indexes: (`state`, `lane`,
  `role`, `priority`, `id`) and (`job_key`, `state`).
- `job_checkpoint`: `job_id`, `chunk`, `cursor`, `progress`,
  `committed_at`; key (`job_id`, `chunk`). Older rows are removed in the
  same transaction, keeping one per job.
- `job_run`: one row per finished job: `job_id`, `kind`, final state,
  attempts, interruptions, durations, settings version, error code and,
  for ganit jobs, the output version. Kept 30 days and at least 200 runs
  per kind.
- `schedule`: `name`, `cron`, `time_zone`, `lane`, `overlap` (`skip`),
  `catch_up` (`once` or `skip`), `enabled`, `last_due_at`.
  `schedule_fire`: `name`, `due_at`, `job_id`, `outcome` (`queued`,
  `skipped_overlap`, `skipped_disabled`); key (`name`, `due_at`).
- `worker_boot`: `worker_name`, `boot`, `started_at`, `seen_at`.
- Retention settings: finished jobs move to `job_run` and are deleted
  after 7 days; events are deleted 7 days after every job made from
  them finishes; dead and held jobs stay until a person replays,
  releases or dismisses them; fires are kept 90 days.

### Row-level security

Every table has RLS on and forced, queue and outbox included. Each role
is a database login that owns no table.

- `event`: serve, worker, inference and scheduler may insert and read;
  none may update; only the scheduler may delete, and only rows past
  retention.
- `job`: every role may insert and read; worker and inference may update
  only rows of their own role; the scheduler may update expired leases
  and retention fields; serve may only cancel, replay, dismiss or
  release, as checked by the action layer.
- `job_checkpoint`: insert, read and delete by the role holding the job.
- `job_run`, `schedule_fire`: append-only.
- `schedule`: the scheduler; `worker_boot`: each worker for its name.

Migrations are forward-only numbered SQL files, tested by upgrading
every earlier released schema with queued, claimed and dead jobs
present. An upgrade never loses a queued job: migrations run while the
workers are stopped, and a change to `job` keeps every queued row.

### Management API and commands

Routes on the management API, Cedar-checked, through the action layer,
every write audited:

- list jobs by state, lane and kind with an id cursor, flagging stalled
  ones (no heartbeat for 60 seconds);
- show one job with its checkpoint position, progress link and last
  error in plain English;
- cancel a job;
- list, replay (by ids, or by kind and error code) and dismiss dead
  jobs;
- list and release held jobs;
- list schedules and run one now (recorded as a fire);
- show which running jobs wait on which;
- list events after an id, for the app to catch up after a reconnect.

Pausing the backlog is the `karya.backfill.paused` setting. Jobs started
by hand use their own feature's routes. `sutra karya`: `list`, `show`,
`cancel`, `dead`, `replay <ids>|--kind <k> --code <c>`, `held`,
`release`, `schedules`, `run <schedule>`, `backfill pause|resume`.
`sutra sanchar`: `tail [--kind]`, `show`. Both are API clients.

Events published here: `job.dead_lettered`, `job.held`,
`job.lease_lost`, `schedule.skipped`, `backfill.paused`,
`backfill.resumed`. Job completion goes through progress.

### Error codes

SUT-200 to SUT-229 for sanchar, SUT-230 to SUT-299 for karya:

| Code | Meaning |
| --- | --- |
| SUT-201 | Event payload over 4 KiB or not ids and counts only |
| SUT-202 | Event kind not declared |
| SUT-203 | Handler needs a newer event schema; job held |
| SUT-231 | Job payload failed its schema (permanent) |
| SUT-232 | Lease lost; job stopped and wrote nothing |
| SUT-233 | Fence check failed at commit; chunk rolled back |
| SUT-234 | Job time limit passed (transient) |
| SUT-235 | Database unreachable (transient) |
| SUT-236 | Statement passed its time limit (transient) |
| SUT-237 | No new work: memory at or over 90% of budget |
| SUT-238 | Backfill paused, with the reason |
| SUT-239 | Handler reported a defect; dead-lettered |
| SUT-240 | Job held after repeated interruptions |
| SUT-241 | Fire skipped; previous run still running |
| SUT-242 | Ganit batch job failed, timed out or gave bad output |
| SUT-243 | Cancel asked for a job that cannot stop safely |
| SUT-244 | Job's module is faulty or waiting; job waits |

### When parts fail

- Database unreachable: claiming stops; running jobs keep their lease
  while they can and cancel before it runs out; forum events stay in
  the add-on's outbox until it returns; nothing is dropped.
- Pool full: callers wait up to their time limit, then get a transient
  error. All pools together stay under `max_connections`; the installer
  checks the sum and health shows it.
- Deadlock or lock wait time-out: retried as transient.
- Disk full: writes fail as transient; the disk watchdog alerts;
  backfill pauses first.

### Security and privacy

- Payloads, events, checkpoints and logs carry ids, versions, counts and
  codes only, never post text, names or anchors.
- Deletion and erasure jobs run in the live lane at priority 0.
- Replay, dismiss, release and cancel are permission-checked and
  audited, with pseudonymous actors.
- The batch runner gets only snapshots of ids, numbers and word counts,
  has no database login and writes only its outputs.
- RLS keeps each role to its own jobs.

### Edge cases

- An event for a post that is gone: treated as deleted.
- Events during an index rebuild: queued; the index parts decide which
  generation they update.
- A handler that partly succeeds then crashes: its chunk rolls back, or
  outside work is repeated harmlessly by key.
- Clock difference with the forum host: ordering uses ids and versions,
  never the other host's times.
- The other side upgraded first: a newer schema holds the job (SUT-203);
  nothing is lost.
- A job running for days heartbeats and checkpoints throughout; its
  settings version stays fixed.
- Many failures at once: one alert per dead-letter kind and code.
- A module switched off: its jobs stay queued and run if it is switched
  on again; uninstall removes them.
- A million backfill jobs take disk, not memory; claiming stays fast
  through the lane, priority and id index.

### Observability

Labels: `lane`, `state`, `kind` (about 100, fixed by code), `class`,
`outcome`, `reason`; never ids. Metrics:
`sutradhar_job_queue_depth{lane,state}`,
`sutradhar_job_queue_age_seconds{lane}`,
`sutradhar_job_duration_seconds{kind,outcome}`,
`sutradhar_job_last_success_timestamp_seconds{kind}`,
`sutradhar_job_retries_total{kind,class}`,
`sutradhar_job_dead_total{kind}`,
`sutradhar_job_interruptions_total{kind}`,
`sutradhar_job_lease_lost_total{kind}`,
`sutradhar_job_chunk_items{kind}`,
`sutradhar_schedule_fires_total{outcome}`,
`sutradhar_event_published_total{kind}`,
`sutradhar_event_coalesced_total{kind}`,
`sutradhar_memory_used_ratio`, `sutradhar_backfill_paused{reason}`,
and Tokio runtime figures with an alert on scheduling delay.

Traces: one per job run, span `karya.job` (`job.kind`, `job.id`,
`job.lane`, `job.attempt`, `error.code`) linked to the publishing
trace; child spans `karya.commit` per chunk; `sanchar.publish` per
event. Logs: `job_claimed`, `job_succeeded`, `job_failed`, `job_dead`,
`job_held`, `lease_lost`, `backfill_paused`, `backfill_resumed`,
`memory_watchdog`, `schedule_skipped`.

Alerts: live queue age over 5 s for 2 minutes (which also tells backfill
to yield); any new dead letter; a scheduled job's last success older
than twice its interval; more than 10 lost leases in an hour; the
watchdog at 90% for over 5 minutes.

## Limits

- Speed targets, measured on Dell with database time included:
  - forum event read to live job claimed: at most 1 s (p99), with the
    backfill lane full;
  - live lane's oldest job waits at most 2 s (p99) during a backlog
    sweep;
  - claim, heartbeat and finish of an empty job: at most 5 ms (p99),
    with RLS on; at least 500 empty jobs a second through one worker
    role with 4 slots;
  - a commit with a 1 KiB cursor and progress: at most 10 ms (p99);
  - after a kill, jobs run again within 45 s (p99), repeating at most
    one chunk (30 s of work);
  - graceful stop within 20 s;
  - 100,000 events in 60 s all kept and coalesced, with an unrelated
    live job claimed within 5 s (p99);
  - a fire queued within 2 s of its due time.
- Memory per role at most 16 MiB: slots under 1 MiB (4 payloads of at
  most 64 KiB); broadcast channel 4 MiB (4,096 events of at most 1 KiB;
  a slow listener catches up from the table); buffers and watchdog
  under 2 MiB; scheduler under 8 MiB. The queue lives in PostgreSQL.
  Snapshot exports stream in batches of at most 8 MiB.
- All of this, ganit's batch runner (in spec 0024's heavy job slot) and
  PostgreSQL sit inside the service slice, within the memory limits in
  AGENTS.md section 5.6.
- Payloads: events 4 KiB, jobs 64 KiB.
- Chunks: about 10 s, never over 30 s. Backoff never over 10 minutes.

## Footprint budget

This part adds nothing to public pages. Calls to the add-ons are
keyed so a repeat changes nothing.

## How it is tested

Rust tests in the sanchar and karya crates, with load and failure
tests in the workspace tests; SQL tests for policies. Coverage is 100%
of lines, branches and MC/DC; the claim, commit and retry code needs
every mutant killed.

1. Publish in a rolled-back transaction leaves no event and no job;
   committed, one event and two jobs for two subscribers.
2. A payload with a text field is refused with SUT-201.
3. Property: streams of events about a few keys leave at most one
   queued job per key per subscriber, holding the newest version.
4. A slot takes a live job before a backfill job; the live slot takes
   no backfill job.
5. Stress: 100,000 jobs, eight workers of four slots on the test
   database; each job finishes once and no two with one key overlap.
6. Model check with loom of heartbeat, commit and re-queuer races:
   either the commit has the current fence or the chunk rolls back;
   never both owners commit.
7. A worker whose lease was taken gets SUT-233; the new owner's
   checkpoint stands.
8. Writes failing after the checkpoint row save neither.
9. 50 transient errors retry 50 times, delays capped at 10 minutes,
   never dead-lettered.
10. Five permanent errors dead-letter with the code; a defect
    dead-letters at once and writes one fault record.
11. Replaying a dead job twice has the effect of one run.
12. A batch over a table with rows added and removed handles every
    steady row once; the cursor only moves forward.
13. Items slowing tenfold never give a chunk over 30 s; chunks shrink.
14. A job claimed on settings version 41 finishes on 41 after 42 is
    activated, and its record says 41.
15. A faulty module's jobs stay queued and run when it runs again.
16. "Post deleted" before an older "post saved": the saved job reads
    the post as deleted and indexes nothing.
17. Two schedulers record one fire and queue one job.
18. A still-running hourly job makes the next fire skip with SUT-241.
19. Property: owner time-zone schedules across several years of clock
    changes: missing times fire once just after, repeated times fire
    once, no day fires twice or not at all.
20. Six missed fires of a `catch_up = once` schedule queue one run.
21. A live job waiting 3 s pauses backfill after its chunk, with "live
    work waiting"; it resumes 60 s after the signal clears.
22. Memory at 82%, 92%, then 70%: chunks halve, then SUT-237 stops
    claims, then claims resume.
23. SIGTERM with three jobs, one not cancellable: two commit or roll
    back and go queued with no attempt counted, the third finishes its
    chunk, and the role exits within 20 s.
24. A restarted worker frees its earlier boot's leases at once.
25. Failure test (nightly): 50,000 made-up posts; the service killed
    mid-chunk in turn; every job resumes within 60 s, at most one chunk
    repeated per worker, each post in the index once.
26. Failure test (nightly): driven to the hard memory limit until the
    kernel kills it; jobs resume from checkpoints, nothing lost.
27. Failure test (nightly): PostgreSQL stopped for 90 s; jobs cancel
    before their leases run out, none dead-lettered, all resume.
28. Load (at most 2 minutes at the push tier): full backfill and 20
    live events a second; live oldest wait at most 2 s (p99).
29. Load (nightly full size, 2-minute version at the push tier):
    100,000 events about 20,000 keys in 60 s; none lost, coalesced,
    a new live job claimed within 5 s (p99).
30. Every event and job schema has no free-text field; captured logs
    and spans hold no post text.
31. Upgrades from each earlier schema keep every job in its state;
    claimed ones become queued.
32. `sutra karya list` and `sutra karya dead` match stored snapshots.
33. Scripted ganit: a `ganit.<job>` job exports, starts, follows,
    collects, checks and hands over; a NaN output is refused with
    SUT-242 and the last good output kept; two due jobs run one at a
    time.
34. Every snapshot schema holds only ids, numbers and word counts.
35. Kani proof: for any attempt count, throughput and settings in
    bounds, backoff never exceeds 10 minutes and a chunk never exceeds
    30 s of work.
36. SQL test: the worker cannot claim an inference job and serve cannot
    update a lease; every table has RLS on and forced.
37. The fifth crash within an hour holds a job with SUT-240 and one
    fault record; it is never claimed again until released, then runs
    once more; four crashes in an hour, or five over more than an hour,
    never hold it.
38. Property: over simulated years of sweeps, dead and held jobs stay
    until a person acts, and each action is audited.
39. A `ganit.*` job due at noon runs in the nightly window; one still
    running at the window's end pauses and resumes the next night.
40. Benchmarks `bench_claim_finish` and `bench_commit`, and the load
    tests above, hold their targets and the 5% median rule.
41. Attack tests from the safe-running part join this list.

Facts to check while building: `SKIP LOCKED` with RLS on the pinned
PostgreSQL; the partial unique index coalescing under concurrent
inserts; the cron library's time-zone handling (else due times come
from a time-zone library and the cron library only wakes); the slice's
control-group files are readable by the service; the system-reading
library reads the slice limit (else read the file); UUID version 7
ordering within one millisecond; time-outs on the claim statement are
classed as transient; `NOTIFY` in a transaction arrives only at commit
and the listener reconnects; snapshot files read by the batch runner
without conversion.

## Changes from the source

- Roles run as parts of the one systemd service, not as separate
  containers; memory and pressure come from the service's slice.
- The 30-second stop grace is the systemd stop timeout.
- Routes are on the management API over the local socket, Cedar-checked.
- The desktop app replaces the old live layer; it runs no jobs.
- ganit's batch runner is Rust and runs inside the service slice,
  started by karya, not a container started by a host helper.
- Defects and overdue batch jobs write held fault records (0018 item G)
  instead of filing issues by themselves.
- Every event and job row carries its site, as AGENTS.md 5.2 requires.
- The 20-times speed goal is replaced by the 5% median rule; old per
  container memory estimates are replaced by the service slice.

Source: `docs/specifications/platform/jobs.md`, `platform/jobs-2.md`,
`platform/jobs-3.md`, `platform/jobs-4.md` and `platform/jobs-5.md` in
the archive `archive-2026-11-08-old-repo`.
