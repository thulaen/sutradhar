# 0033: Safe running

Safe running stops one bad job, one full disk or one wrong change from
hurting a site. It adds build-time bans and attack tests, a "promise"
check before work starts, a shed order, stop-one-job, jobs that wait for
other jobs, low-disk notices, one change journal with undo and trash for
all of Sutradhar's own changes, and a safe mode that stops Sutradhar's
own writes. It builds on the jobs and watchdog of spec 0019. It is
roadmap Phase 5 item 2.

Example: an index rebuild grows past its memory limit while members
post. Only that rebuild is stopped, with a plain reason, and posting
carries on. The owner gets one low-disk notice, not one a day. A
moderator's wrong bulk tag is undone in one click.

## What it does

### Build-time bans

A ban is a check the gate enforces, so the unsafe thing cannot reach
`main`. Bans are data in the gate catalogue, added after the existing
checks without moving them. Each ban is confirmed by research. The
candidates:

- a background task nobody owns (it cannot be named, stopped or
  counted);
- a thread or task started without a name and a time limit;
- a queue or channel without a size limit;
- stopping the whole process from inside a job;
- a network call outside the admitted network client;
- a wait with no time limit;
- the system clock read inside scheduling logic;
- a large memory request not made through the promise check.

Rules:

1. Each ban has two test files. One uses the banned thing and must fail
   with the named message. One uses the allowed route and must pass.
2. A check fails if any ban lacks its pair.
3. The message names the allowed route.
4. There are no exceptions, timed or otherwise. A real need goes through
   an admitted crate.

### Attack tests

Five attack tests run nightly with the chaos tests. Each is a small
program that obeys the rules and tries to take everything:

| Attack | Must happen |
| --- | --- |
| Memory balloon | the promise is refused; no kernel out-of-memory kill |
| Spawn bomb | the job cap holds; surplus is refused fast |
| Retry storm | the retry budget runs out; calls stay under the bound |
| Cron stampede | many same-minute schedules do not starve live work |
| Queue flood | surplus is refused; memory stays bounded |

- Throughout, a made-up live workload runs. Its slowest 1% of responses
  must stay inside a band set by research. Surviving is not enough.
- A longer soak run must show no kernel out-of-memory kill, and the
  pressure steps firing in their written order. Its length is set by
  research.

### The promise check

Before a job starts, or a part loads a model or starts background work,
it asks whether there is room.

1. The check reads the memory gauge and the job's declared need. There
   is room when use now, plus promises already made, plus this job's
   need, fits the part's budget and the memory limits in AGENTS.md
   section 5.6 with 15% headroom and no swap. It looks forward, not
   only at use now.
2. Each job kind declares a memory need and a disk need in its `spec()`.
3. With no room, the job stays waiting with a plain reason ("waiting:
   not enough memory"). It is never failed and never counted as an
   attempt.
4. A job's memory promise is its cap. Asking for more is refused.
5. A job whose outcome is unknown after a crash stays charged until a
   person resolves it, so a ghost cannot free room by accident.
6. Steps that free space (finish, cancel, retention sweeps) are never
   blocked by the check.
7. Recovery after a restart never grants a promise the job did not have.
8. On Dell, the same record and check wrap each gate check.

The promise check has no off switch.

### Shed order and stop-one-job

The watchdog of spec 0019 halves chunks at 80% memory, claims nothing
at 90% and resumes below 75%. Those figures stand. Added:

1. Shed order, as a setting. When pressure rises, work is paused and
   caches unloaded in the order of the table below. Shed work is not
   re-queued by the system; it resumes when pressure clears.
2. Stop one job, not the service. A job that goes past its own cap is
   asked to stop at its next safe point. It is then waiting again, with
   an interruption counted and a reason. Other jobs carry on.
3. Hung is not slow. A job reaching safe points on time is healthy,
   even if slow. One that misses its safe-point limit is hung. Then only
   its worker is restarted, and its leases run out as usual.
4. Incident bundle: kind, cap, highest use seen and promise history.
   AutoIssues files it, scrubbed and held. A job stopped five times in
   an hour is held aside (quarantined).
5. Victim choice, when a whole part is short of memory: the lowest lane
   first, then the largest promise. A `live` job stops only if over its
   own cap. The exact rule is set by research and tested.

Inside one worker several jobs share a process, so stopping one job is
cooperative. Only a hung job costs a whole worker restart.

`karya.shed_order` holds exactly this list, in this order. Step 1 goes
first; each later step only if pressure stays high.

| Step | Paused or unloaded |
| --- | --- |
| 1 | `test_room` jobs (spec 0039) |
| 2 | `chart_filter` and `chart_script` jobs (spec 0043) |
| 3 | `maintenance` jobs |
| 4 | `backfill` jobs |
| 5 | idle caches and models unloaded |
| never | `live` jobs: never shed; stopped only if over their own cap |

### Job waits for job

A job may name earlier jobs it must wait for (for example, rebuild the
index after the import). The wait is checked on claim, with the
existing rules.

- The waiting job can be claimed only when every awaited job has
  succeeded.
- If an awaited job goes dead or is cancelled, the waiting job is held
  with the reason, and a person releases or dismisses it.
- Loops are refused when the wait is made. The job and its waits are
  walked; a wait that leads back to itself, or a job waiting on itself,
  is refused with a plain error and nothing is queued.
- Both jobs belong to one site. Waits never cross sites or installs.
- A job may have only so many waits, and a chain may be only so deep.
- The waiting job keeps its own lane and priority. A wait never raises
  it.
- If the awaited job's record has moved to `job_run`, its final state
  counts.
- This is not `concurrency_group`, which means one at a time, with no
  order.

### Low-disk notices

The gate's jobs on Dell keep 150 GB free. An installed Sutradhar keeps a
researched share of its own disk. Both are protected floors, not caches.

1. A disk is unsafe when promises already made plus the floor exceed
   free space. It is judged on figures already read, never on a guess.
2. One notice when it becomes unsafe, one when it recovers. Not daily.
   A disk sitting at 82% for a month sends nothing new.
3. Recovery needs a clearly safe reading held for a researched time. A
   late or tied reading never announces recovery; unsafe wins a tie.
4. One delivery per recipient per spell. A full outbox refuses new
   spells rather than drop a warning.
5. Quiet hours: the unsafe notice is a fault and gets through. The
   recovery notice waits.
6. No automatic deletion. The notice only tells. The nightly retention
   and the owner-directed emergency order (caches, debug logs, old
   images) stand unchanged.
7. A third failed retention run in a row raises one "clean-up is
   failing" notice. A clean run resets the count.
8. While unsafe, new promises are refused and `backfill` pauses first.
9. Dashboard bands by category show; they do not alert. Their values
   are set by research.

### Job record

- In the product, `job` and `job_run` gain what was promised (memory
  and disk), the cap used and why the job stopped.
- On Dell, each gate check's result gains the same, beside the readings
  the heavy-job queue already records.
- A job's original deadline is fixed at first start, with a cap. A
  restart keeps the charge.

### Change journal, undo and trash

Scope: only what Sutradhar itself changed. Member posts are never
trashed or rewritten by this part. XenForo and WordPress keep their own
rules.

1. Change journal. Every change made through the action layer (a link
   applied, a tag, a setting, a bulk action) records who, when, the old
   value and the new value. This generalises the link and WordPress undo
   journals. Audit stays separate and is never cut short.
2. No record, no change. A change that cannot be journalled does not
   happen.
3. Undo restores only what nobody has touched since. Anything a person
   changed afterwards is left alone and listed as "changed since, not
   undone". The undo is itself recorded; nothing is erased.
4. History is kept 180 days; owners may raise it. Older entries expire
   as stated, not as a cache.
5. Add-on settings go one step back, and the old value is shown before
   it is restored.
6. Trash. What Sutradhar removes (a rejected suggestion, a deleted rule
   or list, a retired item) goes to the trash and can be restored.
   Everything in the trash keeps 90 days. Erasure and uninstall rules
   still apply.
7. One site's journal, trash and waits never reach another site.

### Safe mode

One state, two parts.

- Visitor-facing additions switch off.
- Sutradhar makes no changes, runs no helpers and spends nothing on the
  cloud. Review, doctor, repair, backups and undo still work.
- It says why, and what to do.
- It starts by itself only if the start-up self-check cannot repair
  itself, or a change cannot be journalled. Otherwise monitors only
  alert, and an admin presses the switch.
- Leaving needs an admin and a passed check (`safe.exit_check`, which
  cannot be switched off).

### Settings

Values marked "by research" are measured before the build.

| Setting | Default | Allowed range |
| --- | --- | --- |
| `harness.disk_floor_gb` | 150 (Dell only) | not below 150, safe top |
| `install.disk_floor_share` | by research | safe bottom and top |
| `karya.shed_order` | the shed table | `live` never listed |
| `karya.job.cap_bytes` per kind | by research | above zero, safe top |
| `karya.stop.safe_point_secs` | by research | safe bottom and top |
| `karya.wait.max_per_job`, `max_depth` | by research | 1 up, safe top |
| `disk.recover_hold_minutes` | by research | safe bottom and top |
| `disk.cleanup_fail_runs` | by research | 2 up, safe top |
| `job.deadline_cap_days` | by research | above zero, safe top |
| `undo.keep_days` | 180 | safe bottom, safe top |
| `trash.keep_days` | 90, for all | fixed at 90 |
| `safe.exit_check` | on | cannot be switched off |

### What other parts gain

- Jobs (spec 0019): `job` gains waits and promises; `spec()` gains
  needs; claim runs the promise and wait checks; the watchdog gains the
  shed order and stop-one-job; `maintenance` yields too; new error codes
  and disk-full text; new metrics and the disk alerts; attack tests join
  the test order; quarantine counts stops.
- The gate: bans after the existing checks; attack tests nightly; the
  job record, the 150 GB floor and the memory limits.
- Undo, trash and safe mode (specs 0028 and 0018): generalised, not cut.
- The action layer (spec 0020): each change writes a journal entry.

## Limits

- Memory: room is judged against each part's budget and the memory
  limits in AGENTS.md section 5.6, with 15% headroom and no swap. Part
  budgets stay as set until measured.
- Disk: 150 GB free on Dell for gate jobs; a researched share on an
  installed PC.
- Undo history 180 days (may be raised); trash 90 days, fixed.
- Waits per job and chain depth have safe tops.
- The promise check, the bans and the exit check have no off switch.

## Footprint budget

This part adds nothing to public pages. Safe mode only switches
visitor-facing additions off.

## How it is tested

Rust tests, unless named otherwise:

- SR01: every ban has a failing and a passing test file; a missing pair
  fails the gate.
- SR02: each attack test ends with no kernel out-of-memory kill and
  live responses in band; the soak shows pressure steps in order.
- SR03: no room means waiting with a reason and no attempt counted.
- SR04: one overspending job stops; its neighbours finish.
- SR05: a slow job reaching safe points is not stopped; a hung one is.
- SR06: the shed order is read from settings; `live` cannot be listed.
- SR07: a wait loop of any length is refused and nothing is queued
  (property test).
- SR08: a failed awaited job holds the waiter; a person decides.
- SR09: one unsafe notice and one recovery notice per spell; none daily.
- SR10: a tied or late reading never announces recovery.
- SR11: low disk deletes nothing; freeing steps still run.
- SR12: undo spares a later human edit and says so; the undo is
  recorded.
- SR13: entries older than `undo.keep_days` expire; audit is untouched.
- SR14: a trashed item restores; erasure and uninstall rules still hold.
- SR15: safe mode blocks writes and helpers; undo and backups still
  work.
- SR16: a change that cannot be journalled does not happen.
- SR17: one site's journal, trash and waits never reach another site
  (SQL test under row-level security).
- The room decision, victim choice, wait-loop refusal, the notice state
  machine and undo eligibility are proved with Kani or model-checked
  with stateright. The victim rule is tested under memory shortage.

## Changes from the source

- The deciding logic (room, victims, wait loops, notices, undo) is
  rewritten as Rust, proved with Kani, Verus or Creusot.
- Thresholds, hysteresis and band maths are worked out in ganit.
- Admin screens are the Dioxus app built from SDS components.
- Bans are gate catalogue entries with planted-fault tests, not rules
  of the old checker.
- "Restart its container or worker" becomes restart its worker.
- The memory total follows AGENTS.md section 5.6.

Source: `docs/specifications/platform/safe-running.md` and
`docs/specifications/platform/safe-running-2.md` in the archive
`archive-2026-11-08-old-repo`.
