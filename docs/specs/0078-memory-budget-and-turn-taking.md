# 0078: Memory budget and turn-taking

This specification owns how much memory Sutradhar may use on a PC, how
that budget is shared, and who waits when it runs short. It replaces the
fixed memory ceilings of AGENTS.md section 5.6 and ADR-0020. It builds on
the promise check, shed order and watchdog of spec 0033 and the jobs of
spec 0019; it does not repeat them. It is the owner's decision of 19
November 2026 (ADR-0021): an automatic budget, 6 GB by default, a slider,
and fair turn-taking so every job gets its chance.

Example: Maya has an 8 GB laptop. Sutradhar finds 8 GB of memory, keeps
2 GB for her system, and sets a budget of 6 GB. While she edits a post,
a crawl and a model download wait with the reason "waiting for 2 GB of
memory to free up". When she closes her browser they start, one after the
other. The ranking engine and her review page never wait.

Values marked "proposed" are design choices to be checked by the
measurements in "How it is tested". They are not measurements.

## What it does

### Words used here

- **Machine memory**: the PC's total memory, read at start-up.
- **Headroom**: memory kept for the operating system and other programs.
- **Ceiling**: machine memory minus headroom. Sutradhar never plans above
  it, even when the user overrides a warning.
- **Budget**: the memory Sutradhar plans to use, set by the formula below
  or by the user's slider. It is never above the ceiling.
- **Window share**: the part of the budget for the desktop window.
- **Resident core**: parts that stay loaded: PostgreSQL, the engine
  process and the search cache.
- **Burst pool**: the budget left after the window share and the resident
  core. Jobs, models and the Python sidecar draw from it through
  promises (spec 0033).
- **Heavy model**: a model that needs more than 1 GiB when loaded.

### Reading the machine

At start-up, and again every 60 seconds (proposed), Sutradhar reads
machine memory, available memory, free disk and, when a Vulkan graphics
card is present, its dedicated graphics memory. A failed read keeps the
last good figures and shows "memory check unavailable" on the Jobs screen.
It never guesses a larger figure. After 3 missed reads in a row
(proposed) the last good figures are stale: they give no new capacity,
and new admissions are held until a read succeeds. Keep the original
observation time when retaining its figures. Re-reading that record or
receiving it again cannot clear the stale hold. Recovery needs a new
successful observation from the current admitted reader and host.
The new observation may report the same values.

### The budget formula

All sizes are in GiB (proposed values).

1. Headroom `H = max(2, 0.25 x machine memory)`.
2. Ceiling `C = machine memory - H`.
3. Default budget `D = min(C, max(6, 0.4 x machine memory))` for machines
   of 8 GiB or more.
4. Presets: Safe is `0.75 x D`, Balanced is `D`, Custom is the user's
   value between 2 and `C`.
5. The window share is 0.5 when `D` is below 12 and 1 otherwise.

| Machine memory | Headroom | Ceiling | Default D | Safe |
|---:|---:|---:|---:|---:|
| 8 | 2 | 6 | 6 | 4.5 |
| 12 | 3 | 9 | 6 | 4.5 |
| 16 | 4 | 12 | 6.4 | 4.8 |
| 32 | 8 | 24 | 12.8 | 9.6 |
| 64 | 16 | 48 | 25.6 | 19.2 |

A machine below 8 GiB is outside the supported baseline. QR-013
owns the installer outcome on those machines. One proposal is
Safe only with budget `max(2, C)`. When `C` is below 2 GiB this
value exceeds the ceiling and fails admission.
Require an owner decision and exact host evidence before
qualifying any below-minimum profile.

### Shares inside the budget

The resident core has these proposed caps, each enforced as a limit on the
part: PostgreSQL 1.0, engine process 0.6 and search cache 0.3. At
`D = 6` the resident core is 1.9, the window share 0.5 and the burst pool
3.6. At other budgets the core stays the same and the extra goes to the
burst pool. The core caps may be raised in the Engine Room view up to the
ceiling; they are never lowered below a floor of 0.25 each (proposed).

Embedded DuckDB, the Python sidecar, models and Leviathan jobs are not in
the core. Each takes a promise from the burst pool before it starts.

### Who never waits

These never wait for memory and are never shed: live ranking in Rust, the
review pages, the desktop window, and a backup that is already running.
A live job stops only if it passes its own cap (spec 0033). New live work
may be refused with a plain reason when even the core is over its caps,
but it is never queued behind a background job.

### Who waits, and in what order

Background work is shed in this order when pressure rises. This list
replaces the table in spec 0033 and is the single place it is written.

| Step | Paused or unloaded |
|---|---|
| 1 | Leviathan crawls (all modes) |
| 2 | Model training and Optuna in the sidecar |
| 3 | `test_room` jobs (spec 0039) |
| 4 | `chart_filter` and `chart_script` jobs (spec 0043) |
| 5 | `maintenance` and `backfill` jobs |
| 6 | Idle models, warm sidecar workers and caches unloaded |
| never | The "never wait" list above |

Within one step, the larger promise pauses first, then the later start.
Paused work is not re-queued by the system; it resumes when pressure
clears (spec 0033).

Use the existing 0019 pause and resume record. A repeated request
or delayed pressure signal does not add an entry, start a second
attempt or grant an extra turn. Competing work keeps its own
eligible wait and share under the current fairness rules.

### Fair turn-taking

Each job class has a lane (spec 0033). Waiting jobs in the same lane take
turns by weighted fair queueing with aging:

1. Each waiting job has a weight, 1 by default. Its turn score is its
   weight times `1 + minutes waiting / 30` (proposed aging period).
2. When room appears, the highest score that fits starts first. A job that
   does not fit is skipped, but keeps its score.
3. No job that fits in an empty burst pool waits for more than 12 hours of
   eligible time (proposed) while smaller or later jobs keep starting. The
   scheduler refuses to start a new small job ahead of an old one that
   would fit, once the old one has waited 12 hours.
4. A job that cannot ever fit the budget is refused at once with the size
   it needs and the setting that would allow it.

With several sites, each site has a fair share of the burst pool: equal by
default, changeable by weight. A site that has used its share steps aside
only when another site is waiting. One site never holds more than half the
burst pool while another site waits (proposed).

Only eligible jobs enter turn-taking. A cancelled job, an
unresolved native retry or a job held on a cancelled prerequisite
cannot gain a turn by aging. Other eligible work uses the
existing rules once its exact admission fits. The release of a
resource promise does not decide any native effect outcome.

### Loading models

Every model load is a promise from the burst pool (spec 0033).

- The need is the file size times 1.2 plus the context estimate (proposed).
- Badges: "Fits easily" when the need is at most 40% of the free burst
  pool, "Tight fit" when it fits, "Too big for this PC" when it does not.
- A power user may override "Too big" up to the ceiling. The screen says
  what may slow down or stop, and the override ends when the model unloads.
- On a budget below 10 GiB at most one heavy model is loaded. On a larger
  budget any combination that fits may stay loaded.
- A model unloads after 5 idle minutes (proposed) or when a higher step
  needs the room. Embedding and reranker models are unloaded before a
  language model.
- Local language models never serve live ranking.

### The Python sidecar

The sidecar sleeps when no job needs it and shows "waking up" when it starts.
It keeps `sidecar.warm_workers` light workers awake (default 1 only when `D`
is at least 6 GiB, otherwise 0). On a budget below 10 GiB at most one heavy
worker runs. A worker that grows past its promise is stopped at its next safe
point; one that leaks is restarted between jobs.

### Graphics memory and disk

- With a Vulkan card, graphics memory is a separate budget of 80% of
  dedicated memory (proposed). Shared system memory is never counted as
  graphics memory. With no card, or `gpu.offload` off, models run on the
  processor.
- A disk cap, `disk.cap_gb`, defaults to the smaller of 50 and 25% of free
  disk at install (proposed). A job's disk need is a promise like memory,
  and the low-disk rules of spec 0033 apply.

### Pressure levels

Pressure is used memory divided by the budget, not by machine memory. The
thresholds of spec 0033 stand: chunks halve at 80%, no new claims at 90%,
and work resumes below 75%. At 80% the scheduler also stops starting new
heavy work. At 90% it walks the shed order, one step at a time, waiting 10
seconds (proposed) between steps. A machine whose available memory falls
below the headroom triggers the same walk even if Sutradhar is inside its
budget, because the user's other programs come first.

### What the user sees

- The Jobs screen shows a memory tank, a pill on each waiting job such as
  "Waiting for 2 GB of memory to free up", and the one-sentence headline
  "Running 2 jobs, 1 waiting for memory".
- The memory screen has three choices, Safe, Balanced and Custom, and a
  slider with the figures in plain English. The slider is keyboard
  operable, announces its value and never goes past the ceiling.
- Changing the budget takes effect between jobs. A lower budget does not
  kill running jobs; it stops new starts until usage fits.

### Local resource controls and receipts (owner job catalogue)

Use the single 0078 budget, fair admission and pressure policy.
Do not create a second worker, queue, threshold or resource pool
for the Jobs screen. Pin the exact admitted profile, job class,
host reading scope, freshness and current lease. Distinguish steady
measurements, promises and missing readings. A stale, unsupported
or incompatible host cannot gain capacity. Keep protected work
under its existing priority and safety rules.

| Owner job | Exact resource rule or display |
| --- | --- |
| N-1388 Visual memory tank | Show used, reserved and remaining memory against the same current admitted budget, with headroom and measured versus estimated figures distinct. Indicate freshness and why eligible work waits. Provide keyboard-readable text with the same values. A last good reading is labelled stale and gives no new capacity. [FX-0078-W5-CURRENT-MEMORY-HEADROOM](../planning/feature-wave-5.md#acceptance-design-for-n-1382-to-n-1405) |
| N-1389 Video-call Quiet Mode | The owner can use the existing bounded quiet action for an hour as a local choice. Scope it to eligible background work and the existing sidecar pressure or idle control. Show what may be delayed and remaining time. No meeting, microphone, calendar or provider detection. Ending quiet mode rechecks current pressure and never resumes protected or cancelled work by implication. [FX-0078-W5-QUIET-EXPIRY-CURRENT-PRESSURE](../planning/feature-wave-5.md#acceptance-design-for-n-1382-to-n-1405) |
| N-1390 Auto-napping sidecar | Use 0104's admitted worker lifecycle and the existing warm-worker profile. Release eligible idle processes and their resource promises at a safe boundary with no pending lease or result. Make the next eligible start obtain one new admitted lease and show the wake state. An expired or superseded process cannot return data. No installation, download or hidden workload on wake. [FX-0078-W5-CURRENT-WORKER-WAKE-LEASE](../planning/feature-wave-5.md#acceptance-design-for-n-1382-to-n-1405) |
| N-1391 Battery-aware pausing | Apply only the admitted host power and low-battery policy for eligible heavy work. Show unplugged, low, recovered and unavailable readings distinctly. Pause at the measured safe boundary and resume only under the approved power and pressure conditions. Lost telemetry cannot prove recovery. No new pause threshold or change to protected work. [FX-0078-W5-QUALIFIED-BATTERY-HOLD](../planning/feature-wave-5.md#acceptance-design-for-n-1382-to-n-1405) |
| N-1392 Resource receipt | Project one safe receipt for each exact run or admitted attempt scope. State measured peak memory, elapsed time, disk change or amount, and existing approved work counts, with source and units. Distinguish reservations, estimates, retries and missing readings. Apply 0032 and current site rights before display or export. No file paths, raw payload, cost inferred from resource use or deployment authority. [FX-0078-W5-SCOPED-RESOURCE-RECEIPT](../planning/feature-wave-5.md#acceptance-design-for-n-1382-to-n-1405) |
| N-1394 Fairness balance bar | Show the current lane and permitted site share, eligible wait age, admitted resource budget and why a candidate waits or fits, using 0019's exact fairness decision. Show own-site or approved aggregate scope only. Reveal no other site's identity or protected job details. The display cannot reorder work or change weight or priority. [FX-0078-W5-FAIRNESS-PROJECTION-NO-PRIORITY-CHANGE](../planning/feature-wave-5.md#acceptance-design-for-n-1382-to-n-1405). |
| N-1395 Cool-down auto-brake | Use the exact admitted thermal reading and the existing pressure and shedding order. Bound changes and any recovery by the current host's approved temperature, stability and lease profile. An unknown reading means recovery is unproved. Report a safe reason without inventing temperature or changing protected work. No separate cooling controller or host update. [FX-0078-W5-THERMAL-HOLD-FRESHNESS](../planning/feature-wave-5.md#acceptance-design-for-n-1382-to-n-1405). |
| N-1396 One-slider memory cap | Display Safe, Balanced and Custom through the existing memory setting and its admitted ceiling and headroom. Give a keyboard-accessible value and a preview of eligible starts affected. Only current owner confirmation changes the registered budget. Lowering it affects new admissions under the owning policy and does not kill running or protected work. A percentage example does not set a product threshold. [FX-0078-W5-MEMORY-CAP-EXACT-OWNER-CONFIRM](../planning/feature-wave-5.md#acceptance-design-for-n-1382-to-n-1405). |

Keep quiet, battery, thermal and memory holds independent. Each
has a generation and current eligibility fence. Under JW-21 low
battery pauses heavy work and excess heat slows it. Unplugging
alone supplies no low battery proof. Qualify current sensor freshness
and separate hysteresis profiles under QR-199. Resume only after all
applicable holds clear with a current lease and eligibility.

QR-199 qualifies source freshness, supported host profiles, idempotent
receipts and pause or wake races. Keep pause, recovery and scheduler
authority with their existing owning actions.

#### Hardware speed tuner (N-1500)

Use eligible measured resource receipts and local machine observations
to propose bounded trade-offs for memory, disk caches and crawl pace.
Pin the install or exact site and purpose, workload and hardware
profile, the current approved
settings and evidence provenance. Distinguish stale estimates and
current measurements. Keep the 0078 ceiling and headroom, never-wait
set and promise checks. Cache and crawl proposals use only their owning
registered profiles and limits. Record the exact affected keys and
expected consequences. MCP can prepare an eligible settings plan under
0020. Only a current owner confirmation through the existing route can
apply that exact plan, and protected choices stay outside agent
authority. Viewing or restoring a proposal cannot speed up a crawl or
raise a budget.

Use MB-01, MB-02, MB-03, MB-04, MB-09 and MB-14 for existing resource
checks. QR-203 qualifies comparable measurements, unavailable
readings, current settings and exact worst-case admission. The tuner
allocates no separate scheduler or permission. An eligible site plan
cannot alter installation-wide budget.* or disk.cap_gb or inspect
other sites. Use a distinct owner-only install plan where admitted.
Apply each key at its registered boundary, including between jobs or
restart where required. Preserve MB-08/13 fairness and active work.

[FX-0078-W5-BOUNDED-HARDWARE-TUNER][w5t] binds this case to its
exact Wave 5 fixture and retained resource checks.

[w5t]: ../planning/feature-wave-5.md#acceptance-design-for-n-1478-to-n-1501

#### Tuning slider scope (N-1523)

Show only admitted typed script settings and resource choices from
specs 0014, 0020 and 0104. Bind actor, site, script and manifest
version, exact setting, unit, old value and proposed value.
Show its admitted interval and effect timing and provide equivalent
keyboard entry. A slider movement is a preview. Apply a permitted
change only through the existing owner-confirmed action, with a
fresh exact setting and resource check. Owner-only and protected
settings stay under their separate authority.

Keep script settings separate from install-wide resource settings.
Apply each at its registered boundary, including restart if required.
Preserve running work under MB-13. A script or site grant gives no
install-wide change. Do not resize a live reservation, revive a held job or
grant
headroom above the admitted ceiling. Unsupported units or values,
stale manifests, changed scope and unresolved authority hold.
Use existing MB-03, MB-13, MB-14 and MB-16 checks. QR-204 qualifies
stale plan, scope, bounds and current approval fixtures.

[FX-0078-W5-TUNING-SETTING-OWNER-BOUNDARY][w5s] links the
exact Wave 5 fixture and owning settings boundary.

[w5s]: ../planning/feature-wave-5.md#acceptance-design-for-n-1502-to-n-1525

#### Per-script speed and memory fence (N-1542)

Use 0104's per-run time bound and the existing 0078 memory promise,
with OS enforcement where the owning fence requires it. Bind the
promise and timeout to the exact admitted host, workload, worker,
script and run generation. Account for warm workers, shared and
copied data and the whole admitted process scope. Stop at the existing
safe boundary or invoke the owning forced-stop contract. Preserve
the never-wait work and original receipts. A worker cannot borrow
another lease, widen its own limits or publish late output after
expiry or a replacement. Unsupported platforms and unproved caps
hold admission.

Use MB-02, MB-04, MB-07, MB-10, MB-12, MB-14 and MB-15 with 0104
PY-03, PY-12 and PY-17. QR-205 qualifies exact supported platform
profiling and bounded synthetic stop and recovery fixtures.
[N-1542](../planning/feature-wave-5.md#acceptance-design-for-n-1526-to-n-1550)
uses fixture `FX-0078-W5-SCRIPT-LEASE-LIMIT-AND-LATE-STOP`.

#### Resource mode labels (N-1869)

The source offers Eco, Balanced and Turbo. The admitted product
presets remain Safe, Balanced and Custom. Show their verified
allowed budgets and timing through the existing settings action
and provide clear labels and equivalent keyboard controls.
Any display mapping needs compatible scope. Turbo has no extra
budget, speed guarantee or override route. Do not add a preset
from the source title. Preview effects using the exact current
profile, budget, headroom and owning settings revision. An
eligible owner-confirmed change takes effect between jobs.
Never kill a current worker, resize a lease or queue never-wait
work because the dial changed. Trace MB-02, MB-03, MB-04,
MB-13 and MB-16. QR-219 verifies preview, bounds and races.
See [FX-0078-W5-ADMITTED-BUDGET-MODE-BOUNDS][w5-budget-mode-case].

[w5-budget-mode-case]:
  ../planning/feature-wave-5.md#acceptance-design-for-n-1863-to-n-1880

## Security and privacy

- Readings come from the owner's own PC. Quiet Mode uses no meeting,
  microphone or calendar detection (N-1389).
- Receipts, the fairness bar and the memory tank carry no file path, raw
  payload, other site's identity or protected job detail (N-1392, N-1394).
- Only a current owner confirmation changes the budget. A script, site
  grant or MCP plan cannot raise install-wide `budget.*` or `disk.cap_gb`,
  widen its own limits or borrow another lease (N-1500, N-1523, N-1542).
- A woken sidecar installs, downloads and runs nothing hidden (N-1390).
- A model, job or worker may not load a private copy to evade accounting.

## Requirements

Each requirement has an identifier, a priority (MUST, SHOULD, MAY) and its
tests. Test families are in "How it is tested".

| ID | Requirement | Tests |
|---|---|---|
| MB-01 | The system MUST read machine memory, available memory, free disk and graphics memory at start-up and every 60 seconds, and keep the last good figures on a failed read, for at most 3 missed reads (proposed), after which figures are stale and new admissions are held. | UT-MB-01, FI-MB-02 |
| MB-02 | The budget MUST follow the formula above for every machine of 8 GiB or more and MUST NOT exceed the ceiling. | UT-MB-03, PT-MB-01 |
| MB-03 | Safe, Balanced and Custom MUST work; Custom MUST be refused above the ceiling and below 2 GiB with a plain reason. | UT-MB-04, BD-MB-01 |
| MB-04 | The never-wait list MUST never be shed, queued or paused for memory. | UT-MB-05, QM-MB-01 |
| MB-05 | The shed order table MUST be the only written copy and MUST be applied in order. | UT-MB-06, BD-MB-02 |
| MB-06 | Waiting jobs MUST take turns by the aging rule; no eligible job may starve beyond the starvation bound. | PT-MB-02, QM-MB-02 |
| MB-07 | A job that can never fit MUST be refused at once with the need and the setting that would allow it. | UT-MB-07 |
| MB-08 | Each site MUST get a fair share while another site waits. | UT-MB-08, QM-MB-03 |
| MB-09 | Every model load MUST pass the promise check and show the right badge; the override MUST stop at the ceiling. | UT-MB-09, BD-MB-03 |
| MB-10 | On a budget below 10 GiB at most one heavy model and one heavy sidecar worker MUST be loaded. | UT-MB-10, QM-MB-04 |
| MB-11 | Local language models and Python MUST NOT take part in live ranking. | IT-MB-01 |
| MB-12 | Pressure MUST be computed against the budget and also react to low available machine memory. | UT-MB-11, FI-MB-03 |
| MB-13 | A budget change MUST take effect between jobs and MUST NOT kill a running job. | BD-MB-04 |
| MB-14 | Memory, disk and graphics promises MUST be visible on the Jobs screen with plain reasons. | UI-MB-01 |
| MB-15 | When a job passes its own promise, it MUST be asked to stop at the next safe point under 0033. The proposed 125% forced-stop boundary, its equality rule and process scope require an admitted profile under QR-249. | FI-MB-01 |
| MB-16 | Every setting below MUST be validated, with the allowed range named in the error. | UT-MB-12 |

### Promise admission trace

DES-0078-PROMISE-CONSUMER connects existing MB-09 and MB-14
to [the 0033 accounting requirement and shared variant][promise-owner].
The memory admission check uses the 0033 total. The Jobs view
shows the matching wait reason in Failure behaviour. A lease
release or memory reading does not create released capacity.
Capacity and presentation must agree for the same disposition.

TASK-0078-PINNED-PROFILES names the supported machine and
workload profiles. TASK-0078-CORE-RESOURCE-PROOF maps FI-MB
to SR03 and IT-0019-SCHEDULER-RESTART on each applicable
profile. Spec 0019 owns commit and replay. The 0078 view and
tests cannot release a charge or decide recovery authority.
QR-081 owns admission of that action and proof.

For TASK-0078-CORE-RESOURCE-PROOF, MB-07 and MB-09 use
UT-0019-RESOURCE-BUDGET and BDD-0019-RESOURCE-REFUSAL under
REQ-0019-RESOURCE-ADMISSION. Spec 0019 owns the SUT-237
refusal when the exact required measured profile cannot fit.
For a fitting profile with only temporary contention, use
the 0033 SR03 wait. Bind each branch to the exact host,
workload and current promise evidence under QR-081.

[promise-owner]: 0033-safe-running.md#promise-accounting-trace

## Data

- Settings are rows of the settings registry of spec 0014 (the table under
  "Settings"). This spec adds no table of its own (proposed; confirm at
  the data review).
- Promises, ghost promises and lanes are owned by spec 0033; the jobs that
  hold them are owned by spec 0019.
- Machine readings (memory, available memory, free disk, graphics memory)
  are taken at start-up and every 60 seconds; the last good figures are the
  only ones used when a read fails.
- A resource receipt (N-1392) is one safe projection per run or admitted
  attempt. It is subject to spec 0032 and the current site rights.
- Nothing here stores page content, file paths or raw payloads.

## Settings

Names follow the settings registry of spec 0014. All are per install
unless the scope says site. All defaults and ranges here are proposed
until measured on Dell.

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `budget.mode` | safe, balanced or custom | balanced | three values | between jobs |
| `budget.custom_gb` | number, GiB | `D` | 2 to ceiling | between jobs |
| `budget.headroom_gb` | number, GiB | formula | 2 to half of machine memory | restart |
| `budget.core.postgres_gb` | number, GiB | 1.0 | 0.25 to ceiling | restart |
| `budget.core.engine_gb` | number, GiB | 0.6 | 0.25 to ceiling | restart |
| `budget.core.search_gb` | number, GiB | 0.3 | 0.25 to ceiling | between jobs |
| `budget.model_idle_unload_s` | seconds | 300 | 30 to 86,400 | between jobs |
| `budget.aging_minutes` | minutes | 30 | 5 to 1,440 | between jobs |
| `budget.starvation_hours` | hours | 12 | 1 to 72 | between jobs |
| `sidecar.warm_workers` | whole number | 1 if `D` is 6 GiB or more, else 0 | 0 to 4 | between jobs |
| `gpu.offload` | on or off | on if a card is found | two values | between jobs |
| `gpu.memory_share` | percent | 80 | 25 to 95 | between jobs |
| `disk.cap_gb` | number, GiB | formula | 5 to free disk | between jobs |
| `site.memory_weight` | whole number, per site | 1 | 1 to 10 | between jobs |

A value out of range is refused whole with the range. The Engine Room view
shows these; Basic shows only `budget.mode` and the slider.

Compare size settings in the explicit units defined by spec 0014.
The installed key's spelling does not choose its unit. For a plan
affecting both Custom budget and restart-only headroom, distinguish
the desired version from the running role's effective snapshot.
At a between-jobs boundary validate the budget against headroom
from the running snapshot. Preserve the last complete effective
snapshot and show the restart reason if that mix fails. Use the
0014 checks before and after the existing authorised restart.
The 0033 over-promise stop rule evaluates existing jobs against
their own promises.

## Failure behaviour

| Failure | Detected by | Effect | Recovery |
|---|---|---|---|
| Memory read fails | Reader error | Last good figures kept; headline notes it; labelled stale, they give no new capacity | Next read succeeds |
| 3 reads missed in a row (proposed) | Missed-read count | Figures stale and give no new capacity; new admissions held | Next read succeeds |
| Budget formula gives less than 2 GiB | Formula | Safe only, "unsupported machine" notice | User adds memory or accepts |
| A model load is refused | Promise check | Job waits or is refused with the reason | Room appears or override |
| Process passes its promise | Gauge with exact job scope | Ask the job to stop at the next safe point. Apply a forced stop only under a qualified QR-249 profile | Keep the owning interruption record and apply the 0033 cleanup and reconciliation rules |
| User opens heavy programs | Available memory below headroom | Shed order walked | Resumes when memory returns |
| Crash during a promise | Restart | Keep one original unresolved charge. Work that fits only after its release waits with SUT-4610 | Exact current human recovery and resource release proof under 0033 and QR-081 |

### Failure codes

The codes below are proposed. Codes SUT-4600 to SUT-4699 are proposed as the
range reserved for this spec; the owner or a decision record must confirm
the range before it is used.

| Code | Meaning |
|---|---|
| SUT-4600 | Memory read failed; last good figures kept (information) |
| SUT-4601 | Budget formula below 2 GiB; Safe only, "unsupported machine" |
| SUT-4602 | Model load refused by the promise check |
| SUT-4603 | Job can never fit the budget; refused with need and setting |
| SUT-4604 | Custom budget refused (above ceiling or below 2 GiB) |
| SUT-4605 | Process past its promise, stopped at its next safe point |
| SUT-4606 | Forced stop under the admitted QR-249 profile for the proposed 125% boundary |
| SUT-4607 | Shed order walked because of pressure or low available memory |
| SUT-4608 | Setting refused whole, with the allowed range named |
| SUT-4609 | New live work refused because the core is over its caps |
| SUT-4610 | Ghost promise after a crash, waiting for a person (spec 0033) |

## Interactions and limits

- Spec 0033 owns the promise check, watchdog thresholds, lanes and victim
  choice; this spec supplies the budget and the shed table it uses.
- Spec 0019 owns the jobs and their retries. Spec 0024 owns the models and
  their files; the Model Hub shows the badges of this spec.
- Alekh (spec 0043) and Leviathan (specs 0044 and 0045) declare memory
  needs through their job `spec()`.
- The strict core (ADR-0020) gains item 11, the scheduler's fairness and
  memory caps, proved with Quint and Kani.
- A model, job or worker may not load a private copy to evade accounting.

## How it is tested

Designed, not run. All tests are specified here and are written before the
code. None has run.

- **Unit (UT-MB)**: formula rows for 4, 8, 12, 16, 32 and 64 GiB; Safe and
  Custom edges (2, ceiling, just over); shed order walk; badge boundaries
  at 40% and 100%; every setting range with one value just inside and one
  just outside.
- **Property (PT-MB)**: budget never exceeds the ceiling for any machine
  size from 8 to 1,024 GiB; the sum of promises never exceeds the budget;
  turn order is stable for equal scores.
- **Quint models (QM-MB)**: never-wait list is never shed; no starvation
  within the bound; site fair share; one heavy model on small budgets.
- **Fault injection (FI-MB)**: the memory read fails mid-walk and
  available memory collapses suddenly. In FI-MB-02 keep the exact
  host and profile. After the admitted MB-01 stale boundary, replay
  the last good observation. Keep its original observation time and
  unavailable reason and hold new admission. Then take a fresh successful
  reading whose values match the old one. Use that new observation
  for the ordinary 0033 promise and 0019 claim checks. Neither
  observation releases a prior promised charge or qualifies a profile.
  In FI-MB-01 use a synthetic
  1,024 MiB promise and three exact readings for its own scope.
  Test one byte below 1,280 MiB, equality and one byte above.
  All three exceed the original promise and enter the 0033
  cooperative stop contract. Under the 125% proposal the first
  case is below the forced-stop boundary. The second
  distinguishes inclusive and strict triggers. The third
  satisfies both proposed comparisons. Claim no forced-stop
  pass until the owner-approved trigger and current process
  scope have measured qualification under QR-249. Separately
  offer a combined reading from a shared process with no exact
  job-scope proof. It gives no forced stop or resource release
  on that evidence. Keep the separate hung-worker fixture.
  Join 0033 SR03 for an unresolved charge after crash and restart.
  A new reading, expired lease or different attempt's receipt gives
  no release. Keep exactly one charge through old-state restore.
  Join the SR03 same-attempt partial stop case to the exact
  platform's PL-0037-WORKER-STOP evidence. Keep one unresolved
  charge even when the parent exit is genuine and free memory
  appears sufficient. Apply the same admission hold until the
  original charged scope is proved released and QR-081's recovery
  authority and reconciliation are satisfied.
  Check the Jobs reason and the same current refusal at admission.
  In the ranking branch of IT-0077-QUEUE-RECOVERY, keep one
  unresolved charge for an interrupted training attempt when the
  scheduler receives an authorised protective rollback. The exact
  prior set, host and measured profile match. Set capacity so the
  set would fit only if the original charge were cleared. At
  restart and lease expiry,
  offer a new reading and another attempt's valid release receipt.
  Show the unresolved-promise reason and hold the rollback before
  a new model load or pointer change. Keep candidate admission
  paused and record no rollback success or new load attempt.
  The exact current authorised release clears its own charge
  once. The same rollback can proceed after fresh admission and
  reconciliation if every existing check still passes.
  In the durable-count branch of IT-0019-SCHEDULER-RESTART and SR03,
  prove full worker cleanup and current synthetic recovery authority.
  Stop immediately before the release disposition commits. Keep the
  original charge until the owning record proves the commit. In a
  separate run lose the reply just after commit. Resume and reconcile
  with current evidence, then reserve the same amount for a later
  eligible attempt. Replaying the first result leaves the new charge
  intact and gives no second credit. Repeat with current committed
  evidence unavailable during an isolated old-state restore. Hold work
  that needs the uncertain capacity. The old state supplies no refund.
- **Behaviour (BD-MB)**: Maya's laptop scenario; moving the slider down
  while jobs run; overriding "Too big for this PC"; a second site waits.
- **Integration (IT-MB)**: a crawl, training and a review page run together
  on a simulated 8 GiB machine; ranking latency is not affected.
- **Combined-peak coverage:** bind the native memory observation
  to every required process scope in the matching admitted 0031
  profile, including the app window. Use AGENTS.md section 5.6 and
  the profile's existing measurement evidence. Keep one complete
  synthetic run and its live window fixed. Offer a valid observation
  scoped to the service and workers, with a peak below the budget.
  Hold combined-peak proof because the required window has no
  measured coverage. Keep the observation scoped. The control
  binds the full-scope observation under the same profile. Missing
  coverage supplies no combined result or profile admission.
- **Interface (UI-MB)**: the pill text, headline and keyboard slider.
- **Measurement protocol**: on a real 8 GiB and a real 16 GiB machine, run
  the declared workload profile and record peak resident memory, p95
  review-page latency and headroom. The proposed shares and the 125% kill
  rule are accepted or changed only from these measurements.
  In BDD-0077-RESOURCE-NONOVERLAP bind each measurement to the
  exact concurrent workload set and its batch and model keys.
  Use a synthetic 1,024 MiB budget and promises of 200 MiB for
  serving, 300 MiB for training and 250 MiB for evaluation. Give
  their exact paired profiles peaks of 520, 470 and 570 MiB for
  serving with training, serving with evaluation and training with
  evaluation, respectively. Admit the measured serving latency
  where applicable. Leave the three-way combination unmeasured.
  Keep the 0077 wait and exact smaller-combination control.
  The synthetic case supplies no real admission evidence.
  For Phase 5 item 9, bind each required complete test to its
  admitted host and workload recipe, and to the observed run and
  required windows under TASK-0078-CORE-RESOURCE-PROOF. Use the
  0019 full-backfill measurements and 0033 SR02 ordered pressure
  observations where the admitted recipe requires them. In a
  synthetic completeness case use the same build, host, profile and
  recipe for two authentic incomplete attempts. One has the initial
  required observations and the other has the remaining observations.
  Their union cannot prove a complete run. Keep each observation on
  its original run and hold the affected measurement. A complete
  matching run may clear that evidence obligation. Any authorised
  resume must preserve the original run and prove the required
  coverage under its controlling recipe. Use the existing evidence
  references and hold an unproved recipe, run or reader.

### Unresolved resource boundary fixtures

Use synthetic inputs. These cases check the existing ceiling and
resource validation while the owning question determines the
eligible product behaviour.

- **Small host, QR-013.** Give the installer 3 GiB. The proposed
  headroom is 2 GiB, leaving a 1 GiB ceiling. The proposed Safe
  input gives 2 GiB and is outside that ceiling. The test must
  observe no allocation from that value. Bind any permitted
  Safe profile or refusal to the owner's decision and the
  exact installer and host evidence.
- **Two sites and one pool, QR-081.** Two sites each wait for
  5 GiB in an 8 GiB burst pool. Both requests fit alone.
  Neither fits the proposed half-pool share while the other
  waits. Apply PT-MB-02 and QM-MB-02 at the proposed wait bound
  with this same input. Qualify eligibility and the deterministic
  decision before claiming the wait bound is proved. No start
  may exceed the admitted resource limits or site-share policy.
- **Empty or invalid disk range, QR-248.** Give setup 16 GiB free.
  The proposed default is 4 GiB, below the proposed 5 GiB setting
  minimum. With 4 GiB free the proposed range is empty. Join
  UT-MB-12 to 0033's protected disk-floor proof. Neither an
  out-of-range default nor an empty interval admits disk work.
  Qualify the exact installer and safe wait reason only against
  the approved default, range and current disk-floor profile.

## Changes from the source

AGENTS.md section 5.6's fixed ceilings and ADR-0020's 12 GB admission
rule are replaced by the automatic budget; the shed table of spec 0033
gains two steps at the top (crawls, then training) as the owner decided.
