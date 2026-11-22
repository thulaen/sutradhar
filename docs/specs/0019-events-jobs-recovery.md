# 0019: Events, jobs and durable recovery

## Purpose and ownership

This specification owns platform event delivery, the product job queue,
leases, checkpoints, scheduling, retries, backpressure, replay and
reconciliation. It is roadmap Phase 3 item 6.

ADR-0021 selects a PostgreSQL queue and transactional product outbox,
driven by the Rust scheduler on Axum and Tokio. Spec 0110 owns the shared
primitives. This spec owns product use and one path per job class.
Specialist workers use their owner's admitted typed interfaces.

Spec 0021 owns the separate site-side change feed `xf_sutradhar_outbox`.

## Definitions

- **Event:** an immutable, versioned fact naming ids, versions and counts,
  never unrestricted post text.
- **Job:** one admitted execution of a registered job class against an
  immutable input manifest.
- **Workflow:** the versioned dependency graph for a job class.
- **Lane:** a priority and resource class: `live`, `maintenance` or
  `backfill`.
- **Attempt:** one worker execution of a job, with a unique attempt id and
  fencing token.
- **Fence:** a monotonically increasing token checked at every durable or
  external side effect so an expired worker cannot commit.
- **Checkpoint:** a cursor and progress snapshot committed atomically with
  the business work it describes.
- **Dead letter:** an input or attempt quarantined with its error and
  evidence after retry exhaustion or a permanent failure.
- **Reconciliation:** comparison of authoritative inputs, execution
  receipts and externally visible effects, followed by a defined repair.

## Job-class registry

Every background job class has exactly one entry in the versioned job
registry. A class without an entry cannot run. The entry contains:

| Field | Required contract |
|---|---|
| Identity | Stable dotted kind, schema version, owning specification and module |
| Orchestration | One versioned Rust workflow and typed job contract |
| Lane and tenant | `live`, `maintenance` or `backfill`; tenant-key and RLS rules |
| Inputs | Immutable payload schema, maximum bytes, source and manifest digests |
| Idempotency | Key derivation, duplicate result and conflicting-key behaviour |
| Ordering | Site-qualified ordering key and cross-key parallelism |
| Execution | Worker language, admission profile, concurrency group, timeout and cancellation safe points |
| Retry | Error classes, maximum attempts, base/max backoff and seeded jitter |
| Side effects | Business writes, files, model activation and site calls, each with its fence/idempotency check |
| Checkpoint | Cursor type, commit boundary and maximum repeated work |
| Failure | Dead-letter schema, quarantine, alert and authorised replay procedure |
| Retention | Event, checkpoint, result and audit retention classes |
| Verification | Unit, BDD, integration, chaos, replay and reconciliation test IDs |

The platform job inventory is:

| Job class | Lane | Owning execution contract |
|---|---|---|
| New/edit/delete/erasure ingestion and visibility reconciliation | `live` | Specs 0021 and 0022 |
| Suggestion generation, moderator-approved apply and undo | `live` | Specs 0027 to 0029 |
| Index rebuild and historical backfill | `backfill` | Spec 0022 |
| Model/BGE/spaCy feature rebuild | `backfill` | Specs 0024 and 0026 |
| Search, analytics and connector refresh | `maintenance` | Specs 0034, 0038, 0040 and 0044 to 0048 |
| Retention, backup, restore and destructive deletion | `maintenance`, priority escalation where required | Specs 0030, 0032 and 0033 |
| Research and numerical batch work | `maintenance` | Specs 0012, 0013, 0055 and 0056 |
| Feedback preparation and training | `maintenance` | Specs 0053 and 0077 |
| Batch link-value preview | `maintenance` | Spec 0027 SIM-01 to SIM-05. One fixed-site advisory lease. Typed preview-only output. No ranking or native effect. |
| Ranking study, evaluation, promotion and rollback | `maintenance` | Spec 0077 registers the ranking job kinds. Spec 0057 owns protected evaluation evidence. |
| Nightly, weekly and monthly integrity work | `maintenance` | Specs 0057, 0064 and 0077 |

A feature adds a job class through its one registered workflow and owner.

For 3C, record the owner-selected class under
[QR-252](../registers/questions.md) before deriving the affected job task.
Bind its full registry entry, authorised trigger and exact producer,
class version, tenant and logical effect key to `TASK-0019-DURABLE-PLATFORM`.
Use IT-0019-SCHEDULER-RESTART and IT-0019-DELIVERY-DUPLICATE for its
fenced restart and duplicate-delivery evidence. Where the first outbox
event triggers this class, bind the typed 0110 producer and consumer,
committed event, ordered key and original job effect. Record the owning
trigger and its fixture separately when the event and job are independent.
QR-081 owns the scheduler qualification.

## Event contract

Every event envelope contains:

```text
schema_version, event_id, event_type, occurred_at, producer, tenant_id,
ordering_key, source_version, trace_id, idempotency_key, payload_digest,
payload
```

Payloads carry ids, versions, counts and bounded typed values required by
the schema. They do not carry secrets, credentials or unrestricted post
text. The producer and consumer register the schema and maximum size.
Authenticate site before persisting. Reject wrong-site or unauthenticated
input and record only scrubbed rejection evidence. An authorised invalid
envelope is quarantined without executing its payload.

Serialise the monotone source sequence per site-qualified key in the business
transaction. Strict keys use spec 0110's relay order. Cross-key work may run
in parallel. A timestamp from another host cannot order changes.

Save the business write and product-outbox row in the same PostgreSQL
transaction. Delivery is at least once. Spec 0110 owns relay claims,
ordering locks and consumer inbox deduplication. Commit a consumer's
inbox acknowledgement with its own local effects. If a consumer starts a
durable job, insert the job in that transaction using its logical key.
No claim or acknowledgement can expose an uncommitted change.
Settings activation and role status use spec 0014's strict keys.
Their inbox receipts record delivery only. A role's effective
snapshot follows that spec's separate process-instance check.

## Product scheduler

Use spec 0110's database clock, typed schedule, execution limits and
`schedule_runs` record. Insert the scheduled fire and its queued job
atomically. A fire key binds site, schedule ID, version and canonical planned
occurrence. A mapped time gap can share a physical instant with another
occurrence without merging the keys. Replaced versions admit no new fires and
reconcile their queued work. Reject a changed digest. Authorised manual runs
use their own explicit request ID.

A registered product schedule describes its owning class, dependency
versions, time zone, concurrency group, retry and stop limits, inputs,
artefacts, recovery and evidence. Validate its graph before admission.
A non-overlapping group admits only one active run. A scheduler restart
must rediscover queued jobs and resume according to their durable state.

Product schedules catch up at most once unless the class declares skip.
Use UTC unless the class needs local quiet hours. A missing wall-clock
instant moves to the first valid instant after the gap. A repeated instant
fires once at its first occurrence. Spec 0110 supplies the framework's
timing and misfire mechanism.

When the admitted database clock is unavailable or unproved, hold the
affected schedule and lease decisions. Jobs reports “Waiting:
scheduler time could not be verified.” Keep the existing database
error when its cause is proved. Retain the durable occurrence,
original deadlines, attempts and fences. Do not fire, skip or retry
an occurrence, expire a slot or reclaim a lease based on the workstation
clock. A worker that cannot renew stops before its next effect. The
independent stop and safe-point rules in 0033 and 0110 still apply.
When authoritative time returns, reconcile the original records and
apply the same occurrence, catch-up and admission rules.

## Worker claims, leases and fences

Claim eligible PostgreSQL jobs in a short transaction under the site's
scope. Skip rows already claimed by another worker. Acquire the registered
concurrency slot and write the attempt, input digest, lease expiry and
higher fence in the same transaction as the claim. A claim never spans a
network call. The execution ledger preserves all attempts.

Queued work can become running or held. A hold records a reason and can
be released to the queue. A running attempt completes, fails, cancels
at a safe point or becomes interrupted. Expiry uses the database clock.
Before the recorded expiry a lease is still current for its time check.
At equality it has expired and the old worker can no longer renew or
act on it. Reclaim atomically checks current database time and the old
lease and fence. Once the earlier attempt is reconciled, it releases
its slot and admits one eligible replacement fence under the original
deadline. Terminal jobs cannot be re-claimed. Replay uses the dead-letter
rules. Spec 0099 reads these states.

The worker renews its lease at the registered heartbeat interval. A new
attempt obtains a higher fence. Every business commit, file rename,
artefact-pointer update and site request checks the current fence and
idempotency key. A stale attempt cannot commit, publish a success receipt
or activate an artefact.

If heartbeat renewal fails, the worker stops before its next side effect.
The scheduler or recovery reconciler marks the attempt interrupted
when authoritative database time reaches the lease expiry. Reclaim
still requires reconciliation and 0033 and 0078 admission before
any replacement attempt.
An unresolved resource promise remains charged under 0033.
Lease release and resource release are separate checks. Reconcile the
exact original attempt, fence and resource scope against its durable
release disposition and existing action outcome. For the admitted
QR-081 recovery route, commit the charge disposition and scoped result
as one owning execution update. Another claim can count that capacity
only after verifying the commit. If the disposition is missing or
uncertain, keep the charge and hold work that depends on its release.
Replay of a committed release returns its earlier result only under
current result rights and cannot release another promise.
An infrastructure interruption is not counted as a product test
failure, but its full attempt history is retained.

## Idempotent side effects and checkpoints

A checkpoint commits in the same PostgreSQL transaction as the business
rows and execution-ledger update it covers. It includes job, attempt,
fence, cursor, chunk number, source high-water version, progress and
committed time. The cursor uses a stable key, never a mutable row offset.

Stage immutable content-addressed files and verify hash and durability. Their
visibility pointer changes with the current fence check in one database
transaction. Stale staging cannot publish a file. Index identities include
tenant, item, source version, feature/model version and generation. A repeated
write of identical bytes returns the original receipt; different bytes under
one idempotency key are quarantined.

Use spec 0021's signed stable request ID for each site effect and bind the
approved target version. Duplicates use the original site receipt. A timeout
after sending is an unknown outcome: the worker reconciles before retrying and
never assumes the site did not apply the change. Keep the logical effect held
until its outcome is reconciled. A later attempt cannot issue a different
request for that effect while the outcome is unknown. After the site receipt
window expires, refuse blind retry and escalate the unknown result for
reconciliation.

Model and champion transitions use the atomic manifest and
compare-and-set contract in spec 0077. No generic worker can activate a
model outside that controller.

## Retry, unknown outcomes and dead letters

Errors are classified as:

1. **Transient:** lost connection, timeout or rate limit. Retry with
   bounded exponential backoff and seeded jitter up to the class's attempt
   and elapsed-time limits. Respect a provider's longer explicit retry
   window without exceeding the authorised job deadline.
2. **Permanent:** malformed input, unsupported schema, permission refusal
   or digest mismatch. Do not retry execution; dead-letter or quarantine.
3. **Unknown outcome:** a side effect may have succeeded but its receipt
   was lost. Reconcile first; retry only through the idempotent recovery
   path.
4. **Defect:** invariant violation or unexpected internal failure. Stop
   the affected work, dead-letter immediately and file a fault record.
5. **Resource refusal:** admission or hard budget failure. Stop at a safe
   point and retain evidence; never reduce mandatory evaluation or test
   scope to obtain a pass.

A dead letter retains the original immutable input, all attempts, error code,
trace and last valid checkpoint. Replay creates a new attempt under the same
key and digest only after reconciling effects. A corrected payload needs a
linked job with a fresh key after reconciliation. Replay, release, dismissal
and cancellation require Cedar permission and an audit record.

## Priority, yielding and resource admission

Live ingestion, moderation-driven apply/undo and erasure take precedence
over maintenance and backfill. Use admitted lane limits and spec 0078
turn-taking so backfill cannot occupy every worker. A ready lane that fits
must receive its admitted fair share. Priority does not reorder one key.

Backfill and maintenance workers yield at checkpoint boundaries when live
work exceeds its registered queue-age threshold, site load is unsafe, or
CPU/memory/I/O pressure crosses the admitted resource profile. They resume
only after the registered stable interval. Yielding never abandons a
half-committed chunk.

For a backfill with an existing committed `backfill.paused` record,
bind a resume to that exact pause and the current scoped run,
checkpoint and effect fence. Resolve the pause identity through
the ordered source event and the owning job state. Hold when
those records cannot prove the current pause. A checkpoint can
stay identical across two different pauses. Current pressure
recovery and a permitted Resume request compete at the existing
fenced resumption. Only one may advance that pause. Replays of a
committed Resume can show its original result under current
result rights. An unperformed request for a superseded pause
holds. Neither a stale request nor a repeated recovery signal
can clear a later pause or terminal cancellation.

Resource admission measures the scheduler, PostgreSQL, Rust services and
admitted workers and model processes together. Training
and promotion evaluation do not overlap production serving unless the
combined profile passes. Apply spec 0078 and Dell's job limits. Work that
does not fit waits or refuses without reducing mandatory evidence.

## Graceful shutdown

Shutdown stops new claims globally. A cancellation stops only the target. At
the next safe point, commit or roll back the chunk and record interruption or
cancellation with atomic lease release. Hold any unresolved external effect
for reconciliation. A forced kill leaves the fence and checkpoint for
recovery. The registered stop timeout and maximum repeated chunk are measured
in chaos tests.

When cancellation commits after an authorised native send, keep
the cancellation and the site effect outcome separate. A late
worker response cannot commit success after lease loss. If the
original link-edit receipt needs recovery, use the existing
authorised 0021 ReadLinkEffect route. A verified committed
receipt settles that effect once under its owner. It does not
change the cancelled job into a completed job, run a successor or
undo the site edit. Missing current receipt proof leaves the
effect unresolved. No case releases a new send by age, pause,
memory clearance or expiry alone.

## Replay and reconciliation

Recovery has four ordered stages:

1. **Freeze inputs:** identify the exact business versions, event
   checkpoints, manifests and policy digests.
2. **Reconstruct in isolation:** replay from the approved checkpoint into
   quarantined derived state without external writes.
3. **Reconcile:** compare authoritative business state, event log,
   execution receipts, file/artefact identities and site idempotency
   records. List missing, duplicate and conflicting effects.
4. **Repair and resume:** apply only the registered idempotent repair,
   verify it and then resume live consumption from a recorded position.

Coordinated restore includes the PostgreSQL business, outbox, queue,
inbox, scheduler and execution records, artefact stores and registry
history. Check all generations before writing and replay only from a
reconciled point. Retention cannot stand in for backup.

## Version changes and migration

A fresh installation creates one PostgreSQL path. Inventory earlier
layouts and map producers, consumers, schedules, keys, jobs and effects
to registered contracts. Prove compatibility by isolated replay. A
shadow cannot perform side effects.

Drain or park the class, record its high-water mark and fence old
workers. Migrate with checksums and one restore manifest. Switch one
admission path atomically. Reconcile all job dispositions before
reopening. A gap blocks that class. Retain evidence.

Rollback uses compatible backup and reconciled keys, with exactly one
live path. After retirement, use forward recovery under the same rules.
ADR-0021 supersedes the earlier broker migration.

## Management API and commands

The local owner-only API, through Cedar-checked actions, provides:

- list classes, workflows, runs, attempts, progress and last error;
- show queue age and relay lag by admitted scope without private content;
- cancel, pause, resume and resource-admit a class;
- list, reconcile, replay and dismiss dead letters;
- list schedules and request an audited manual run;
- show event and attempt lineage by id and input digest; and
- show migration wave, drain, cutover and reconciliation evidence.

`sutra karya` and `sutra sanchar` remain API clients; their command names
access the same queue and event contracts. Write operations use typed action
requests, live permission checks and audit. No command accepts shell text.

Existing event names such as `job.dead_lettered`, `job.held`,
`job.lease_lost`, `schedule.skipped`, `backfill.paused` and
`backfill.resumed` use versioned product schemas. Progress uses the
shared progress contract rather than a second status store.

## Error contract

Existing codes retain their meanings where applicable:

| Code | Meaning |
|---|---|
| `SUT-201` | Event payload violates its schema, size or content restrictions |
| `SUT-202` | Event kind is not registered |
| `SUT-203` | Consumer requires a newer schema; work waits or dead-letters under its compatibility rule |
| `SUT-231` | Job payload failed its schema |
| `SUT-232` | Lease lost; attempt stopped before another side effect |
| `SUT-233` | Fence check failed; chunk or activation rolled back |
| `SUT-234` | Job time limit passed |
| `SUT-235` | PostgreSQL unreachable |
| `SUT-236` | Database statement exceeded its limit |
| `SUT-237` | Resource admission refused new work |
| `SUT-238` | Backfill paused with its reason |
| `SUT-239` | Handler defect; dead-lettered |
| `SUT-240` | Work held after repeated interruptions |
| `SUT-241` | Scheduled fire skipped because the previous non-overlapping run is active |
| `SUT-242` | Numerical or data batch failed, timed out or produced invalid output |
| `SUT-243` | Cancellation cannot yet stop safely |
| `SUT-244` | Owning module or worker is unavailable |
| `SUT-245` | Retired broker-specific error; never emitted by product jobs |
| `SUT-246` | Retired orchestrator-specific error; never emitted by product jobs |
| `SUT-247` | Reconciliation gap; affected derived work blocks |
| `SUT-248` | Duplicate idempotency key carries a different payload; quarantine |
| `SUT-249` | Migration drain, cutover or rollback reconciliation failed; dual live execution remains forbidden |

## Security and privacy

Each workflow, relay and worker has a named identity and least privilege.
Derive site scope from the authenticated contract, never free payload.
Use forced RLS and a non-owner role under spec 0108 on every queue,
outbox, inbox, schedule and execution transaction. Ordinary workers
cannot read signing keys, release policy or owner credentials.

Payloads, checkpoints and logs exclude post text, member names, anchors
and secrets unless a specific specification expressly requires bounded
content in a protected artefact. Audit actors are pseudonymous.
Destructive replay, restore, deletion, scheduler fairness and memory
caps belong to the strict core. The scheduler needs a Quint model.

## Observability

Metrics include relay backlog and lag, queued and running time, workflow
outcomes, worker attempts, retries,
interruptions, fence refusal, checkpoint age, dead letters,
reconciliation differences, migration dispositions and resource high-water
marks. Labels use registered lane, class, state, outcome and reason, never
raw tenant or content identifiers.

Traces link producer event, schedule or workflow run, job and attempt,
checkpoint and external idempotency receipt. Logs exclude payload content.
Alerts fire for live lag beyond its service objective, any new dead
letter, repeated lease loss, schema/digest failure, missed schedule,
resource pressure and unreconciled migration or replay.

## Requirement, design and task traceability

These identities group the normative contracts above. They do not show
execution state or admit an implementation task. Derived tasks still
follow spec 0076's readiness gate.

| Requirement | Normative obligation | Design and derived task | Unit evidence | BDD evidence |
|---|---|---|---|---|
| `REQ-0019-DURABLE-EXECUTION` | Registered event and job schemas, per-key ordering, fences, idempotent effects, bounded retries, schedules, dead-letter replay and live priority preserve one logical effect | `DES-0019-DURABLE-PLATFORM`; `TASK-0019-DURABLE-PLATFORM` | `UT-0019-REGISTRY-UNIQUE`, `UT-0019-EVENT-SCHEMA`, `UT-0019-PER-KEY-ORDER`, `UT-0019-FENCE-COMMIT`, `UT-0019-IDEMPOTENT-FILE`, `UT-0019-SITE-UNKNOWN`, `UT-0019-RETRY-BOUND`, `UT-0019-DST`, `UT-0019-DEAD-LETTER-REPLAY`, `IT-0019-OUTBOX-ATOMIC`, `IT-0019-DELIVERY-DUPLICATE`, `IT-0019-SCHEDULER-RESTART`, `PRP-0019-NO-DOUBLE-COMMIT` | `BDD-0019-LIVE-PRIORITY` |
| `REQ-0019-RESOURCE-ADMISSION` | Combined admission and fair turn-taking protect live work, avoid starvation of fitting work and refuse excess without reducing evidence | `DES-0019-DURABLE-PLATFORM`; `TASK-0019-DURABLE-PLATFORM` | `UT-0019-RESOURCE-BUDGET` | `BDD-0019-RESOURCE-REFUSAL` |
| `REQ-0019-COORDINATED-RECOVERY` | Migration, replay, reconciliation and coordinated restore preserve immutable evidence, fences, idempotency and compatible recovery points | `DES-0019-DURABLE-PLATFORM`; `TASK-0019-DURABLE-PLATFORM` | `UT-0019-RESTORE-COMPATIBILITY` | `BDD-0019-RESTORE` |

| Design | Owning contract | Requirements and derived task |
|---|---|---|
| `DES-0019-DURABLE-PLATFORM` | The job-class registry through the migration, recovery and observability contracts above | `REQ-0019-DURABLE-EXECUTION`, `REQ-0019-RESOURCE-ADMISSION`, `REQ-0019-COORDINATED-RECOVERY`; `TASK-0019-DURABLE-PLATFORM` |

| Derived task | Implementation boundary | Upstream identities |
|---|---|---|
| `TASK-0019-DURABLE-PLATFORM` | Implement the Rust scheduler, one PostgreSQL queue and transactional product outbox with registered typed worker boundaries | `DES-0019-DURABLE-PLATFORM`; `REQ-0019-DURABLE-EXECUTION`, `REQ-0019-RESOURCE-ADMISSION`, `REQ-0019-COORDINATED-RECOVERY` |

For the shared unresolved promise variant, use
[VAR-0033-INTERRUPTED-PROMISE][promise-accounting].
REQ-0019-RESOURCE-ADMISSION and REQ-0019-COORDINATED-RECOVERY
reuse DES-0019-DURABLE-PLATFORM and TASK-0019-DURABLE-PLATFORM
for the owning execution commit, exact replay and restored
disposition. IT-0019-SCHEDULER-RESTART uses the SR03 and FI-MB
charge and admission observations. Spec 0033 owns the accounting
and spec 0078 consumes the verified total. QR-081 qualifies the
recovery action, current Cedar scope and complete evidence.

[promise-accounting]: 0033-safe-running.md#promise-accounting-trace

## Verification design

These are test designs, not execution claims. The earlier backend fixtures
`IT-0019-CDC-GAP`, `IT-0019-KAFKA-DUPLICATE` and
`IT-0019-AIRFLOW-RESTART` are retired. Their successors are
`IT-0019-OUTBOX-ATOMIC`, `IT-0019-DELIVERY-DUPLICATE` and
`IT-0019-SCHEDULER-RESTART` respectively.

| Test | Requirement | Case | Required result |
|---|---|---|---|
| `UT-0019-REGISTRY-UNIQUE` | `REQ-0019-DURABLE-EXECUTION` | Two workflows claim one job class | Registry validation refuses both execution and admission |
| `UT-0019-EVENT-SCHEMA` | `REQ-0019-DURABLE-EXECUTION` | Valid event, unrestricted text, oversize, unknown schema and wrong site | Accept only the valid event; reject wrong site before persistence and quarantine authorised invalid input |
| `UT-0019-PER-KEY-ORDER` | `REQ-0019-DURABLE-EXECUTION` | Interleaved sites, stalled producer, concurrent commits and repeated versions | Preserve order per key, permit cross-key parallelism and never order by foreign timestamps |
| `UT-0019-FENCE-COMMIT` | `REQ-0019-DURABLE-EXECUTION` | Old and new attempts commit the same chunk or staged-file pointer concurrently | Only the current fence commits; the other receives `SUT-233` |
| `UT-0019-IDEMPOTENT-FILE` | `REQ-0019-DURABLE-EXECUTION` | Same file write repeated and same key with different bytes | First returns original receipt; second quarantines with `SUT-248` |
| `UT-0019-SITE-UNKNOWN` | `REQ-0019-DURABLE-EXECUTION` | Site applies change, response is lost and a later retry exceeds receipt retention | Return the original result while provable; hold after expiry without blind retry. In the cancellation branch, commit the native save, lose its reply, commit local cancellation and expire its worker fence. Give the current reconciler a valid same-effect result through ReadLinkEffect before the receipt window ends. Reconcile one original effect, keep the job cancelled and the dependent held, and issue no retry or undo. Without current receipt proof, keep the effect unresolved. An independent fitting job can claim its own turn after legitimate resource admission. |
| `UT-0019-RETRY-BOUND` | `REQ-0019-DURABLE-EXECUTION` | Transient failures exceed class attempts and elapsed limit | Dead-letter with complete attempts; no infinite retry |
| `UT-0019-DST` | `REQ-0019-DURABLE-EXECUTION` | Two schedules share version and instant; local gap and fold; several missed occurrences and replay | Keep distinct keys; map gaps, use the first fold occurrence and catch up at most once |
| `UT-0019-DEAD-LETTER-REPLAY` | `REQ-0019-DURABLE-EXECUTION` | Retry identical input, then correct a permanent bad payload | Preserve original evidence; replay its same digest or admit a reconciled linked job with a new key |
| `UT-0019-RESOURCE-BUDGET` | `REQ-0019-RESOURCE-ADMISSION` | Combined serving and training measurements exceed the admitted service-memory ceiling | Refuse training before execution, retain serving and mandatory evaluation scope, and emit `SUT-237` |
| `UT-0019-RESTORE-COMPATIBILITY` | `REQ-0019-COORDINATED-RECOVERY` | One compatible coordinated manifest set and one set mixing incompatible business, artefact and scheduler or queue generations | Accept only the compatible set; reject the mixed set before any write and name the incompatible components |
| `IT-0019-OUTBOX-ATOMIC` | `REQ-0019-DURABLE-EXECUTION` | Roll back a business write, then crash after a committed write and before relay | Rollback leaves no event; relay recovers the committed row once logically |
| `IT-0019-DELIVERY-DUPLICATE` | `REQ-0019-DURABLE-EXECUTION` | Relay delivers the same committed event three times around worker crash | One inbox effect, one logical job and one externally visible effect |
| `IT-0019-SCHEDULER-RESTART` | `REQ-0019-DURABLE-EXECUTION` | Kill scheduler before its transaction, worker before commit, and result producer after commit. Separately make database time unavailable at restart with a durable fire and lease. Move the workstation wall clock past the next due instant and old lease expiry. Restore verified database time just before expiry, then at equality with the original job deadline still open and previous effects reconciled. | Resume each case from checked evidence without duplicate effects. Without verified time, keep the fire, lease, fence and deadline and issue no time-dependent transition. Do not reclaim before expiry. At equality refuse the old worker and admit at most one eligible replacement for the original job. |
| `IT-0019-MIGRATION-DRAIN` | `REQ-0019-COORDINATED-RECOVERY` | Old queued, claimed, dead and held jobs exist during class cutover | Each receives one reconciled disposition; no old and new executor runs live together |
| `IT-0019-MIGRATION-ROLLBACK` | `REQ-0019-COORDINATED-RECOVERY` | Cutover finds an unexplained old item | Restore old admission at the high-water mark, fence new workers and verify no duplicate effect |
| `BDD-0019-LIVE-PRIORITY` | `REQ-0019-DURABLE-EXECUTION` | Full backfill and a new erasure event | Backfill yields at a checkpoint; live work meets its objective and fitting work gets its admitted fair share |
| `BDD-0019-RESOURCE-REFUSAL` | `REQ-0019-RESOURCE-ADMISSION` | On one pinned host vary the measured required combined profile and committed resource promises in separate runs. The first profile exceeds the current ceiling even without competing background promises. In the control the measured profile fits, but an eligible worker still holds a current promise. Join UT-0019-RESOURCE-BUDGET, 0033 SR03 and 0078 MB-07/MB-09 for TASK-0078-CORE-RESOURCE-PROOF. | Refuse the first before any training attempt starts with `SUT-237`. The fitting control waits with the current resource reason and starts no attempt while its room is unavailable. After the other promise is released, check current scope, ceiling and fair turn before admission. Serving and mandatory evaluation scope stay intact in both cases. |
| `BDD-0019-RESTORE` | `REQ-0019-COORDINATED-RECOVERY` | Restore coordinated business, artefact, queue, relay and schedule backups | Components resume at one compatible reconciled point |
| `PRP-0019-NO-DOUBLE-COMMIT` | `REQ-0019-DURABLE-EXECUTION` | Generate leases, retries, crashes and concurrent attempts | At most one fence commits each logical side effect |

The IT-0019-SCHEDULER-RESTART resource branch also covers
REQ-0019-RESOURCE-ADMISSION and REQ-0019-COORDINATED-RECOVERY.
Use 0033 SR03 for the original charge and 0078 FI-MB for both
sides of the authorised release commit and old-state restore.
A current claim sees either the one original charge or its
verified release. A replay cannot change a later attempt's
promise even when its amount is identical.

In IT-0019-SCHEDULER-RESTART, race a permitted Resume with
current pressure recovery for one committed backfill pause.
Test both commit orders. Check at most one resume and one
eligible worker. In the manual-first case lose its reply, then
pause again before publishing new checkpoint progress. Keep the
checkpoint bytes identical. Replay the first action. Return
only its permitted history and preserve the newer pause. In
the pressure-first case delay the unperformed request past
the newer pause and separately past terminal cancellation.
Refuse before any new attempt. In every
branch, verify one queue identity and that a separate fitting
job receives its turn under the same 0078 policy.

Load and soak tests measure event-to-claim latency, live queue age,
checkpoint commit time, recovery time, graceful stop and throughput under
full backfill. The performance specification sets the numerical budgets;
no target is claimed from the architecture alone.

This part adds nothing to public pages. Site calls are idempotent and
site footprint remains governed by each feature specification.
