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

- Throughout, a made-up live workload runs. The slowest-1% response band,
  workload profile and measurement fixtures are readiness blockers owned by
  the safe-running performance record. The attack-test gate cannot pass until
  they are admitted; surviving without a latency verdict is not enough.
- The soak length, pressure schedule and expected observations are readiness
  blockers owned by the same record. Until admitted, the soak cannot qualify
  a release; it must ultimately show no kernel out-of-memory kill and the
  pressure steps firing in their written order.

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
5. A job whose outcome is unknown after a crash stays charged once
   under the exact original attempt, fence and resource scope. An
   expired lease, apparent spare capacity or another attempt's
   receipt cannot release the charge. Restart or restore cannot
   refund or duplicate it. The current authorised human recovery
   and exact release proof must match the original record. If the
   actor, action or proof is unproved, hold the charge and work
   that depends on its release. Use 0078 SUT-4610. QR-081 binds the
   recovery authority and proof for the affected profile.
   For a fenced worker, verify release of the exact charged resource
   scope through the admitted 0037 or 0104 stop evidence. An accepted
   stop request, matching worker exit or partial process-tree check
   cannot prove release of the whole original charge. Keep it held
   when any charged part remains or its release is unverified. Use
   the owning platform's fence checks. This creates no stop or
   recovery action.
6. Steps that free space (finish, cancel, retention sweeps) are never
   blocked by the check.
7. Recovery after a restart never grants a promise the job did not have.
8. On Dell, the same record and check wrap each gate check.

For crash reconciliation, count one charge for each original
attempt, fence and resource scope until the authorised release is
proved committed under 0019. Recovery reads that disposition. It
does not add capacity from the same receipt twice. A stop proof
or approved release that has yet to commit leaves the charge in
the admission total. Keep a later attempt's promise separate
even if its resource request is identical. An unreadable or
unreconciled restored state supplies no free capacity.

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
   own cap. The complete deterministic tie-break and starvation rules are
   readiness blockers owned by the memory-pressure research record. The
   victim-selection behaviour cannot build until the record and its adverse
   tests are admitted; the written lane order still takes precedence.

Inside one worker several jobs share a process, so stopping one job is
cooperative. Only a hung job costs a whole worker restart.

Any separate forced-stop profile proposed by spec 0078 needs
the trigger and affected scope qualified under QR-249. Keep the
ordinary cooperative stop and the hung-worker restart under
their existing conditions.

`karya.shed_order` holds the shed list. The list and its order are
written once, in spec 0078 (memory budget and turn-taking), so that no
second copy can drift: crawls pause first, then training, then the test
room, chart jobs, maintenance and backfill, then idle models and caches.
`live` jobs are never shed; they stop only if over their own cap. Step 1
goes first; each later step only if pressure stays high.

### Job waits for job

A job may name earlier jobs it must wait for (for example, rebuild the
index after the import). The wait is checked on claim, with the
existing rules.

- The waiting job can be claimed only when every awaited job has
  succeeded.
- If an awaited job goes dead or is cancelled, the waiting job is held
  with the reason, and a person releases or dismisses it.
  If the owning site later confirms a committed effect, reconcile
  its exact receipt under 0019 and 0021. This keeps the cancelled
  job's final state. A receipt alone cannot qualify the held
  dependent or give it a new attempt. Existing person and
  dependency checks still apply.
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

The gate's jobs on Dell keep 150 GB free. An installed Sutradhar's protected
disk-floor share is a readiness blocker owned by the disk-safety research
record; new promises stop until an admitted value is available. Both floors
are protected, not caches.

1. A disk is unsafe when promises already made plus the floor exceed
   free space. It is judged on figures already read, never on a guess.
2. One notice when it becomes unsafe, one when it recovers. Not daily.
   A disk sitting at 82% for a month sends nothing new.
3. Recovery needs a clearly safe reading held for `disk.recover_hold_minutes`.
   That duration is blocked until the disk-safety research record admits it;
   no recovery is announced without it. A late or tied reading never announces
   recovery; unsafe wins a tie.
4. One delivery per recipient per spell. The authoritative notice-spell and
   recipient-delivery records commit with the disk-state transition and spec
   0019 outbox event. Keep one notice effect per recipient and spell.
   Reconcile an unknown delivery before retrying. If the durable notice
   journal or event reconciliation is unavailable, new promises stop and the
   unsafe state stays visible; no warning is discarded or marked delivered.
5. Quiet hours: the unsafe notice is a fault and gets through. The
   recovery notice waits.
6. A low-disk notice cannot delete anything. Retention and each step in
   the owner-directed emergency order for caches, debug logs and old
   images use their own authority. The [Dell legacy sweep][dell-sweep]
   has its separate current-rule check.
7. The [record-retention job in 0018][record-retention] raises one
   "clean-up is failing" notice on its third consecutive failed run.
   Further failures in that streak give no new notice. A successful
   run resets the count for later notices under 0018. Existing alerts
   keep their resolution route under 0032.
   Any optional override or threshold for another cleanup route needs
   its own admission. This notice gives no trigger or cleanup authority
   for Dell's legacy timer.
8. While unsafe, new promises are refused and `backfill` pauses first.
9. Dashboard bands by category show only after the disk-safety research
   record admits their values and fixtures. Until then each affected band
   shows "not measured" and does not alert; no guessed band is displayed.

[record-retention]:
  0018-platform-additions.md#c-stored-records-new-versions-only
[dell-sweep]: 0059-pankti.md#legacy-nightly-sweep-and-current-permission

### Job record

- In the product, spec 0019's execution-ledger record holds what was
  promised (memory and disk), the admitted cap and why the attempt
  stopped. It is an audit and recovery record, not a PostgreSQL queue.
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

Every value marked "blocked" is a readiness blocker owned by the safe-running
research record. It must supply the measured value, safe bounds, adverse
fixtures and approval before the dependent behaviour can build or run. A
missing disk, stop, wait or deadline safety value fails closed.

| Setting | Default | Allowed range |
| --- | --- | --- |
| `harness.disk_floor_gb` | 150 (Dell only) | not below 150, safe top |
| `install.disk_floor_share` | blocked: no admitted value | safe bottom and top |
| `karya.shed_order` | the shed table | `live` never listed |
| `karya.job.cap_bytes` per kind | blocked: no admitted value | above zero, safe top |
| `karya.stop.safe_point_secs` | blocked: no admitted value | safe bottom and top |
| `karya.wait.max_per_job`, `max_depth` | blocked: no admitted value | 1 up, safe top |
| `disk.recover_hold_minutes` | blocked: no admitted value | safe bottom and top |
| `disk.cleanup_fail_runs` | blocked: no admitted optional override or threshold for another route | 2 up, safe top, only after admission. The fixed 0018 notice uses its owning rule |
| `job.deadline_cap_days` | blocked: no admitted value | above zero, safe top |
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

### Not built

These were turned down when the owner chose this design, and are not
built here; a later page may ask the owner again:

- From the old runtime candidate: a separate worker tier in another
  language; our own async runtime; an allocator shim and a
  zero-allocation core; separate settings files (spec 0014 holds
  settings); time-limited exceptions and a switch to turn off jitter;
  deadlines that may only shorten, jitter, single-flight, a retry
  budget stack, a full graph runner, a simulation harness and a browser
  layer; rows for several machines; and its build-first order.
- From the old storage candidate: automatic deletion jobs, guessing
  what is a cache, a fixed cache cap, disk-image and seed-failure
  rules, byte layouts and the copy census.
- From the old memory candidate: its baseline figure and its start and
  stop duties (spec 0019).
- Also: percentage feature switches, automatic rollback beyond the
  existing undo, and a standby database.

## Limits

- Memory: room is judged against each part's budget and the memory
  limits in AGENTS.md section 5.6, with 15% headroom and no swap. Part
  budgets stay as set until measured.
- Disk: 150 GB free on Dell for gate jobs; an installed PC uses only the
  admitted `install.disk_floor_share`, which remains blocked as stated above.
- Undo history 180 days (may be raised); trash 90 days, fixed.
- Waits per job and chain depth have safe tops.
- The promise check, the bans and the exit check have no off switch.

## Footprint budget

This part adds nothing to public pages. Safe mode only switches
visitor-facing additions off.

## Promise accounting trace

| Identity | Existing contract |
|---|---|
| `REQ-0033-PROMISE-ACCOUNTING` | The promise check keeps one charge for the original attempt, fence and resource scope until its authorised release is proved. |
| `DES-0033-PROMISE-CHECK` | The promise check owns charge accounting. Use the existing 0019 scheduler and execution disposition and 0078 memory and Jobs presentation under their own contracts. |
| `VAR-0033-INTERRUPTED-PROMISE` | An admitted product worker can lose its outcome across crash, restart or isolated restore. Apply the same accounting on each supported host and resource profile. The original attempt and its later replacement keep distinct promises. |

TASK-0033-SAFE-RECOVERY derives the accounting case from these
identities. SR03 proves hold and preservation of the original
charge. FI-MB observes the dependent admission and its reason.
IT-0019-SCHEDULER-RESTART covers the owning durable result on
both sides of the release commit. Use PL-0037-WORKER-STOP where
the admitted profile needs process-scope proof. The existing
protected gate still needs exact runner and profile evidence.

QR-081 qualifies the existing authorised recovery action and
Cedar scope, original attempt and fence, resource identity,
release evidence and audited outcome. QR-029 owns any required
worker fence evidence. Where the recovery action or proof
cannot be established, keep the charge held. A positive
clearance task needs an admitted route and the spec 0076
readiness record.

## How it is tested

Rust tests, unless named otherwise:

- SR01: every ban has a failing and a passing test file; a missing pair
  fails the gate.
- SR02: each attack test ends with no kernel out-of-memory kill and
  live responses in band; the soak shows pressure steps in order.
- SR03: no room means waiting with a reason and no attempt counted.
  Compare the measured-fit but currently occupied case in
  BDD-0019-RESOURCE-REFUSAL. A current admitted promise makes
  the otherwise eligible job wait. Verify the reason and no
  started attempt.
  A later verified release affects only the capacity check.
  The scheduler still checks eligibility and the fair turn.
  Crash an attempt with an unresolved promise. Restart at lease
  expiry, give a fresh reading that appears to leave room and a
  receipt for a different attempt. Hold a candidate that fits only
  if the original charge is removed. Keep one original charge,
  show SUT-4610 and count no new attempt. Repeat after old-state
  restore and with missing or refused recovery authority. Preserve
  independently eligible work under its existing admission rules.
  In a separate fenced-worker case, keep the original attempt,
  fence, resource identity and synthetic recovery authority exact.
  Supply a valid observation that the worker's parent exited, while
  the admitted process scope still contains one synthetic fixture
  member or its closure cannot be proved. A fresh reading appears to
  have room. Keep the charge once with SUT-4610, and count no new
  attempt for dependent work. Compare with complete scope release
  evidence. That evidence clears only the resource proof check.
  Current recovery authority and audited reconciliation still apply
  under QR-081. The fixture grants no real reconciliation.
- SR04: one overspending job stops; its neighbours finish.
- SR05: a slow job reaching safe points is not stopped; a hung one is.
- SR06: the shed order is read from settings; `live` cannot be listed.
- SR07: a wait loop of any length is refused and nothing is queued
  (property test).
- SR08: a failed awaited job holds the waiter; a person decides.
- SR09: one unsafe notice and one recovery notice per spell; none daily.
- SR10: a tied or late reading never announces recovery.
- SR11: low disk deletes nothing. Eligible freeing steps still run
  under their own authority. In a separate synthetic 0018 retention
  part, two consecutive failed runs give no notice, the third gives
  one and the fourth gives no new notice. A successful run resets the
  count with the prior alert and audit revision unchanged.
  The third failure in the next streak gives one new notice.
  An unadmitted threshold for another route supplies no alternate
  notice and no evidence for the 0018 streak. No timer or service is
  started or changed and the fixture deletes nothing.
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
- Admin screens are the desktop app built from SDS components.
- Bans are gate catalogue entries with planted-fault tests, not rules
  of the old checker.
- "Restart its container or worker" becomes restart its worker.
- The memory total follows AGENTS.md section 5.6.
- The old candidates' project names are left out of the "Not built"
  list; its 12 GB baseline and 60 GB cache cap are given as "a
  baseline figure" and "a fixed cache cap".

Source: `docs/specifications/platform/safe-running.md` and
`docs/specifications/platform/safe-running-2.md` in the archive
`archive-2026-11-08-old-repo`.
