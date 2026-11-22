# 0110: Behemoth Boot messaging, events and resilience

This is group PB-F of Behemoth Boot (spec 0081). It owns how a Behemoth Boot
program sends and receives messages and events, keeps a database change and a
message in step, schedules work, survives flaky calls, models state, and takes
locks. It covers: message brokers (Kafka, AMQP and RabbitMQ, Pulsar, NATS and
queue adapters), an integration patterns engine, the transactional outbox and
inbox, the module wall engine, async context propagation, the scheduler, the
resilience layer, the event bus, state machines, and locks with leader
election. Spec 0081 owns the ledger and clean-room rules, spec 0106 the core,
spec 0107 the web layer, spec 0108 data access (it owns the site column, row-
level security and migration conventions; this spec owns the DDL of the tables
listed under Data), and spec 0109 security. Batch jobs are group PB-G; health
and metrics are PB-H.

Sutradhar's own use follows ADR-0021: jobs and events use a PostgreSQL queue
with an outbox in the same transaction as the change. Delivery is at least
once and every consumer is idempotent. Our own scheduler in Rust, on Axum and
Tokio, replaces Airflow, and there is no Kafka. The job classes, leases and
recovery rules are owned by spec 0019 and the Jobs screen by spec 0099.
This spec provides the shared framework. The broker adapters here are framework
parity only:
built, tested and kept in step with Spring, and off in the Sutradhar profile.

Spring facts here come from the owner's pillar list (19 November 2026) and are
"unverified" until the scout confirms each against official notes (spec 0081).
The behaviour description, not the Spring wording, is the contract. The
pillar list asked for clean-room engines for every broker; they are mature
crates behind our starters (ADR-0022, AGENTS.md section 6 rule 14).

Example: an editor saves a report. In one database transaction the report
row and an "report saved" message row are written. The app is killed straight
after the commit. On restart the relay finds the unsent message and sends it.
A listener marked "after commit" runs once; had the transaction rolled back,
neither the message nor the listener would exist. The relay sends the message
a second time after a lost acknowledgement, and the consumer's inbox
recognises the duplicate and ignores it. A call to a slow outside service
retries twice with growing waits, then the circuit opens and later calls fail
fast with a plain message instead of piling up.

Values marked "proposed" are design choices to be checked by measurement.

## What it does

### Words used here

- **Event**: a fact that something happened, with a type and a payload.
- **Outbox**: a table of messages written in the same transaction as the
  change they describe, sent later by a relay.
- **Inbox**: a table of message ids already handled, so a repeat is ignored.
- **Relay**: the loop that reads the outbox and sends messages.
- **At-least-once**: a message may arrive more than once, never zero times.
- **Idempotent**: doing it twice has the same effect as doing it once.
- **Lease**: a lock that expires unless it is renewed.
- **Fencing token**: a number that rises with each lease so an old holder can be
  refused.
- **Circuit breaker**: a switch that stops calls to a failing service for a
  while.
- **Bulkhead**: a limit on how many calls to one service run at once.
- **Misfire**: a scheduled run that was missed, for example while the app
  was off.
- **Back-pressure**: a full receiver slowing its sender.
- **Jitter**: a small random delay added so many callers do not act together.
- **Fencing**: refusing a write that carries an older lease token.
- **Starter, composition root**: defined in specs 0081 and 0106.
- **Dedupe key**: the id that tells a consumer a message is a repeat.

### What this spec designs

| Feature | What the user or program sees |
|---|---|
| N-1777 Unified event bus with typed listeners | Local and durable events through one interface; listeners that run only after a commit |
| N-1772 Transactional outbox and inbox guarantee engine | A save and its message succeed or fail together, and repeats are ignored |
| N-1773 Modular monolith architecture engine | Module walls enforced at build and test time; durable record of cross-module events |
| N-1774 Automatic async context propagation | Identity, trace and request markers follow work onto other tasks |
| N-1775 Distributed scheduler and cron engine | Fixed-rate, fixed-delay and cron tasks that never run twice by mistake |
| N-1776 Built-in resilience and fault-tolerance layer | Retries, backoff, circuit breakers, rate limits, timeouts and bulkheads |
| N-1784 Declarative state-machine engine | States, events, guards and actions, with persisted state |
| N-1785 Distributed locks and leader election engine | Leases with fencing tokens, and one elected leader |
| N-1771 Full Enterprise Integration Patterns engine | Channels, routers, splitters, aggregators, claim checks and file watchers |
| N-1845 Clean-room Apache Kafka engine | Producers, consumers, streams and share consumers, with dead-letter recovery |
| N-1846 Generic AMQP 1.0 and RabbitMQ clients | Classic and generic AMQP clients, streams and virtual-host set-up |
| N-1770 Apache Pulsar, NATS and queue adapters | Listeners, schema registries and acknowledgement control for other brokers |

### The event bus

1. One interface publishes and listens, in two kinds: local (in memory, inside
   one process) and durable (through the outbox, surviving a crash). Events
   and listeners are typed; a listener names the event type it handles. The
   composition root wires listeners explicitly (spec 0106); there is no
   scanning.
2. A listener may be marked "after commit". Mechanism: the transaction wrapper
   of spec 0108 collects after-commit callbacks and runs them only after a
   successful commit; on rollback they are dropped. After-commit listeners are
   best effort and local: if the process dies between the commit and the
   callback, the callback is lost, and only a durable event survives. An
   after-commit listener is therefore forbidden for work that must happen
   (use the outbox). An error or panic in an after-commit listener is
   reported (SUT-1013) and cannot undo the commit. Without a transaction such
   a listener is refused and does not run, and the log says so (SUT-1013).
3. Local listeners run in a declared order, on the publisher's task or on a
   named worker pool. A listener that fails does not stop later listeners and is
   reported with its error; it is never swallowed (AGENTS.md 5.1).
4. Durable events carry an id, a type, a version, a site, a time, a trace id, an
   optional ordering key and a payload. Ordering is kept per ordering key and
   is not promised across keys.
5. Delivery of durable events is at least once; Sutradhar does not claim
   exactly-once delivery. Consumers are idempotent through the inbox below.
   Buffers are bounded by entries and bytes (`event.buffer`,
   `event.buffer_mib`) and exert back-pressure on the relay, never unbounded
   memory (spec 0078).

### Outbox and inbox

1. The outbox and inbox are tables with a site column and forced row-level
   security (spec 0108). Writing an outbox row happens inside the caller's
   transaction through the same connection, so the row exists only if the change
   committed (SUT-1000 if there is no transaction).
2. Claiming and order. The relay claims rows by ordering key: it takes a
   transaction-level advisory lock for (site, key) with a try-lock, and a row
   is never sent while an earlier unsent row of the same key exists. `FOR
   UPDATE SKIP LOCKED` is used only to pick different keys for different
   relays. A key marked `strict` (the default) stalls when its first row fails
   until the row is sent, dead-lettered by `outbox.max_attempts`, or handled
   by a person (retry or skip); a key marked `unordered` lets later rows pass.
3. Attempts are counted before the send, so a message that crashes the relay
   still counts. Between attempts the wait grows exponentially up to
   `outbox.retry_max_delay_s`, and a relay that cannot reach the database backs
   off the same way and starts with a random delay (`outbox.relay_jitter_ms`)
   so many relays do not start together after an outage (SUT-1014, information).
4. A message that fails `outbox.max_attempts` times moves to the dead-letter
   table with its last error. The Jobs screen (spec 0099) lists dead letters; a
   person may retry or discard them. Dead letters are kept
   `dead_letter.retention_days`.
5. Inbox: the handler's transaction runs `INSERT ... ON CONFLICT DO NOTHING` on
   (consumer, dedupe key), then the handler's own writes, then commits. A
   conflict means "already handled": the message is acknowledged and skipped
   (SUT-1002, information). The dedupe key is the outbox row id for durable
   events, or a producer-supplied key when the event declares one. Effects are
   therefore applied once per consumer even though delivery is at least once.
   A local settings watcher wakes only after the durable consumer
   commits. The inbox records delivery. Spec 0014 owns application
   and its separate process acknowledgement. For settings consumers,
   compare the canonical digest on a duplicate identity. Changed
   payload bytes are a conflict to quarantine, not a successful retry.
6. Sent rows are kept `outbox.retention_days` and then removed. The inbox is
   kept `inbox.retention_days`, which must be at least the outbox retention
   plus the longest replay window; a start with a smaller value fails.
7. A payload is capped at `outbox.max_payload_kb`. A larger one is refused at
   write time, and the claim check pattern below is the way to send bulk data.
   The table is capped at `outbox.max_rows` (proposed); at the cap the
   writer refuses a new row with a clear error (SUT-1013) and the alarm at
   `outbox.backlog_alarm` has already fired. Polling is one shared loop for
   all sites, every `outbox.poll_ms`, never one loop per site.
8. Background tasks have no user. The relay and the scheduler build a system
   principal for each site and open a site transaction themselves (`set_config`
   as in spec 0108) before they read or write anything.
9. Metrics for group PB-H: relay lag (oldest unsent age), backlog size, dead
   letter count. An alarm fires at `outbox.backlog_alarm`.
10. At shutdown the relay stops claiming, finishes sends in flight and marks
    them inside the drain budget of spec 0106.
11. The relay and the consumers never send site content outside the PC unless
    the switch for that destination is on (ADR-0021).

### Modular monolith engine

1. Each module declares its public API, its allowed dependencies and the events
   it publishes and handles, in its manifest. This spec defines the manifest
   format. The check itself is run by nirikshak (specs 0061 and 0003), which
   already owns module walls; there is no second engine.
2. At build and test time the check reads the real dependency graph from the
   build tool and fails on an undeclared edge, a cycle, or a use of another
   module's internals. Event publication is checked statically: the publish
   function is generic over a declared event type, so publishing an undeclared
   event does not compile. Tests also check it at run time (SUT-1003).
3. Cross-module events are recorded durably in the outbox with the publishing
   module and the handling module. A test can read them back and assert, for
   example, "saving a report publishes `report.saved` and the index module
   handles it".
4. Event visibility across threads is a tested rule (BF-25): a handler on
   another task sees the committed data of the publisher.
5. The engine generates a plain module map from the manifests into the
   documentation (BF-26).

### Async context propagation

1. A task-local context holds the principal (spec 0109), the trace and span ids,
   the site, the request id and the locale. The runtime's own spawn does not
   carry task-locals, so the framework's spawn functions copy the context
   explicitly. They cover async tasks, blocking tasks and threads.
2. A database transaction is never carried to another task. A new task starts
   with a marker "no transaction", and the data starter (spec 0108) refuses a
   statement without a site transaction there.
3. The child's context is a copy and cannot raise privileges. A context copy
   holds no secret (a token is never in the context).
4. The gate bans the runtime's raw spawn functions and `std::thread::spawn` in
   product code with a lint list (clippy's disallowed methods). The lint cannot
   see a macro or a dependency that spawns; those are covered by the dependency
   review. A raw spawn found by the gate fails the gate with SUT-1010.
5. Relay and scheduler tasks create a system principal per site (see Outbox
   and inbox).

### Scheduler

1. Three kinds of task: fixed rate (start every N seconds), fixed delay (wait N
   seconds after the last run ends) and cron. Cron has five fields or six with
   seconds first, and a time zone. For fixed-time cron, our design choice (not a
   standard; cron programs differ) is: a time that does not exist because
   clocks jump forward runs at the next valid moment, and a time that happens
   twice runs once. Expressions of the form "every N minutes or hours" run by
   elapsed time and ignore the change. The cron parser is a mature crate; its
   behaviour is a build-or-reuse item.
2. Misfire policy per task: skip, run once now, or run missed runs up to
   `scheduler.max_catchup`. The default is `scheduler.default_misfire`, run
   once now. Runs beyond the limit are dropped and logged (SUT-1007).
3. Overlap policy per task: forbid (the default), allow, or queue one. A refused
   overlap is logged (SUT-1006).
4. Only one node runs a task at a time. The task takes a lease and a fencing
   token (below) before it runs and renews it while running. On a single
   machine, which is Sutradhar's case, the lease is a database row and costs
   one write per run. Two app instances on one data directory are blocked by
   an instance lock in the database.
5. The scheduler reads time from the database clock (`now()`) and uses monotonic
   time for in-process timers, so a wall-clock jump on the PC cannot skip or
   repeat runs. Hold decisions that require database time when that time is
   unavailable or unproved. Apply the product occurrence and lease recovery
   rules of 0019. A task running longer than `scheduler.max_run_s` (or its own
   limit) is cancelled at its next safe point and reported. Its in-process
   timer does not authorise a scheduled fire or lease reclaim. Work is
   admitted by the turn-taking rules of spec 0078.
6. Every run is recorded in `schedule_runs`: start, end, result, node and lease
   token. The scheduler owns this record. Specs 0099 and 0019 use its run
   history. Spec 0019 caps product catch-up at one or skip.

### Resilience layer

1. Policies are small, typed and composable: retry (count, backoff, jitter,
   which errors), timeout, circuit breaker (failure rate, window, open time,
   half-open probes), rate limiter, bulkhead (maximum concurrent calls and queue
   length) and an overall deadline.
2. The default order, outermost first, is: rate limiter, bulkhead, overall
   deadline, retry, circuit breaker, per-attempt timeout, call. The breaker
   counts every attempt. An open-breaker rejection is never retried. Hedging
   (sending a second request early) is not supported.
3. The build refuses these unsafe orders: retry with no overall deadline; a
   bulkhead inside retry (each retry would take a new slot); and any retry of a
   call not marked idempotent.
4. Retry applies only to errors marked retryable and calls marked idempotent.
   Backoff is exponential with jitter, capped by
   `resilience.retry_max_delay_ms`.
5. An open breaker fails fast with a stable error (SUT-1004) that says which
   service and when the next probe is. A retry limit reached is SUT-1005.
6. Every policy publishes metrics (calls, retries, rejections, open time) for
   group PB-H.

### State machines

1. A machine is declared with states, events, transitions, guards (conditions)
   and actions. The build checks the declared graph: every state is reachable
   and a final state has no outgoing transition. Two transitions for one state
   and event are accepted only if each has a named guard and a declared priority
   or a declared "exclusive" pair. The engine cannot prove that two guards are
   exclusive, because guards are ordinary Rust; the declaration is a promise
   that tests check.
2. A guard returning false refuses the transition with a plain reason
   (SUT-1009). Actions run inside the transaction of the transition, and a
   failing action rolls the transition back.
3. State is stored through a repository (spec 0108) with a version column, so
   two writers cannot both apply a transition from the same state.
4. A machine can be exported as a model for the Quint checker (ADR-0021). The
   export covers the declared graph; guards and actions appear as free
   predicates, not code. It is generated from the declaration, never written by
   hand.
5. Sutradhar uses machines for job and review lifecycles; the job states
   themselves are defined in spec 0019.

### Locks and leader election

1. A lock is a lease on a named resource held by an owner with an expiry. It is
   taken with one atomic database statement and renewed before it expires.
2. Each grant returns a fencing token that rises every time. The resource that
   accepts a write checks the token inside the same transaction as the write
   (for example `UPDATE ... WHERE fence <= $token`), so a holder that was
   paused past its expiry cannot damage anything. This `fence` is the same
   counter that spec 0019 uses for its worker fences; one counter, one
   meaning.
3. A job attempt takes a new grant. A class or task may set its own lease and
   renew times; the settings below are the defaults.
4. Leader election uses a lock. A node that cannot renew stops acting as leader
   at once.
5. PostgreSQL is the default. Valkey or another store needs separate
   framework admission under DR-071. Optional stores stay off in the
   product. The admitted store's clock determines expiry.
6. A lock held when the app shuts down is released as part of the drain (spec
   0106); one left by a crash expires.

### Integration patterns

1. An in-process engine with typed message channels (direct, queue, publish and
   subscribe), content-based routers, splitters, aggregators with a completion
   rule and timeout, filters, transformers, claim checks (the payload is stored
   aside and a reference travels), file watchers and gRPC handlers.
2. Every channel is bounded. A full channel applies back-pressure to the
   sender or rejects according to its policy, and never grows without limit.
3. An aggregator has a maximum number of open groups and a time limit per group;
   a group that times out is released with what it has or discarded by policy.
4. Error channels carry failed messages with their cause. No message is dropped
   without a record.

### Broker adapters

1. Kafka (producers, consumers, stream processing and, unverified, share
   consumers with asynchronous commits, and dead-letter recovery), AMQP 0-9-1
   and 1.0 with RabbitMQ streams and virtual-host set-up, Pulsar, NATS and
   JMS-style point to point queues each get a starter. Clients are mature
   crates. We compile no C or C++ (ADR-0021), so a broker whose only client
   needs a native library is used only if a prebuilt binary is allowed,
   otherwise it is blocked and listed (QR-037).
2. Each adapter provides: typed listeners, acknowledgement control (manual or
   after the handler succeeds), retry and dead-letter handling that use the
   resilience layer, and schema checks where the broker has a registry.
3. All adapters are excluded from the Sutradhar build by a Cargo feature, and a
   gate test checks that the product binary contains none of them. A message
   off the PC needs the owner's switch for that destination (ADR-0021).

### Reuse

Our own code is the event bus, the outbox and inbox logic, the module wall
checks, the context rules, the scheduler's policies, the resilience composition
and its order check, the state-machine declaration and checks, and the lock and
lease protocol. Mature crates sit behind them: the async runtime, broker
clients, a cron parser, and metrics. Each choice is a build-or-reuse record in
the research dossier.

#### Message delivery, scheduling and fenced transitions (N-1770–N-1785)

Define each parity item through spec 0081. Use the phase and
product profile that own the work. Sutradhar uses one local
PostgreSQL job, outbox and inbox path under specs 0019 and 0108.
Optional broker and clustered framework fixtures confer no
product adapter, connector, port, second scheduler or deployment.
Pin site, principal, schema, scope, versions, effect identity and
current grant at every boundary. Use synthetic fixture traffic,
not a Spring program, live external target or real credential.

| Feature | Exact framework behaviour and product boundary | Existing requirements |
|---|---|---|
| N-1770 Apache Pulsar, NATS and queue adapters | Qualify only admitted mature clients and named typed protocol profiles in the framework. Bind listener schema, consumer identity, acknowledgement mode, retry and dead-letter receipt. An unproved client stays blocked. The product binary contains no broker adapter. A broker or schema does not grant outbound access. [FX-0110-W5-FRAMEWORK-ADAPTER-NO-PRODUCT-ROUTE](../planning/feature-wave-5.md#acceptance-design-for-n-1767-to-n-1790). | BF-20, BF-21, BF-22, BF-24, BF-40 |
| N-1771 Full Enterprise Integration Patterns engine | Use bounded typed channels, routes, splits, joins and claim references with declared ownership, scope and completion rules. Refuse untyped or cross-site claims and record safe reject or timeout outcomes. Distinguish a complete group, a timed-out partial release and a discarded group under the declared policy. A late part cannot turn the recorded timeout into completion. Record the failed or dropped part under BF-39. Route, file watch, gRPC shape and claim reference give no new listener, file permission, send or data read. [FX-0110-W5-BOUNDED-TYPED-CHANNEL-AND-CLAIMS](../planning/feature-wave-5.md#acceptance-design-for-n-1767-to-n-1790). | BF-07, BF-19, BF-39 |
| N-1772 Transactional outbox and inbox guarantee engine | Commit the scoped business change and exact outbox intent together. Keep stable event, consumer and ordering identities. The handler commits the inbox receipt with its eligible effect and acknowledges under the admitted protocol. An exact repeat causes no second effect. Bind payload digest and hold a conflict or uncertain receipt under the owning reconciliation path. Delivery remains at least once. [FX-0110-W5-ATOMIC-INTENT-EXACT-REPLAY](../planning/feature-wave-5.md#acceptance-design-for-n-1767-to-n-1790). | BF-01, BF-02, BF-06, BF-27 to BF-30, BF-32 to BF-34 |
| N-1773 Modular monolith architecture engine | The existing nirikshak check compares actual build dependencies with declared public APIs, module permissions and event types. Reject undeclared edges, internals or cycles. Bind declared events to the admitted typed route and prove consumers see committed records. Do not create a second module check or separate product service. [FX-0110-W5-DECLARED-MODULE-AND-EVENT-WALLS](../planning/feature-wave-5.md#acceptance-design-for-n-1767-to-n-1790). | BF-07, BF-25, BF-26; 0003/0061 |
| N-1774 Automatic async context propagation | Copy only admitted principal, site, trace, request and locale fields. A child inherits no transaction, secret or extra privilege, and opens its own admitted site transaction. Revalidate current authority at protected use. Lint direct spawn and review indirect task starts under the existing proof. [FX-0110-W5-CHILD-CONTEXT-NO-TRANSACTION-INHERITANCE](../planning/feature-wave-5.md#acceptance-design-for-n-1767-to-n-1790). | BF-08, BF-09, BF-32 |
| N-1775 Distributed scheduler and cron engine | Pin the admitted clock, occurrence, schedule revision, overlap and missed-run rules and exact run and owner identity. Use one lease and fence per eligible run. Reconcile duplicate delivery and stop stale attempts. The product uses only its local scheduler under 0019 with its stricter catch-up rules. No cluster or second queue follows from the framework form. [FX-0110-W5-CURRENT-OCCURRENCE-SCHEDULER-FENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1767-to-n-1790). | BF-10, BF-11, BF-32, BF-34, BF-35 |
| N-1776 Built-in resilience and fault-tolerance layer | Use the exact admitted order of timeout, deadline, retry, breaker, limiter and bulkhead policies. Retry only a currently eligible idempotent attempt under the original effect identity and budget. An unknown response reconciles first. Open circuits, lost grants and stop fences cannot be bypassed by retry or fallback. No new connector or hedged request. [FX-0110-W5-RETRY-ORIGINAL-EFFECT-WITHIN-BOUNDS](../planning/feature-wave-5.md#acceptance-design-for-n-1767-to-n-1790). | BF-12, BF-13, BF-14, BF-19 |
| N-1777 Unified event bus with typed listeners | Register typed publishers and listeners through the existing composition. Distinguish volatile local work from durable outbox work. An after-commit callback is best effort and cannot run for a rolled-back change or reverse a commit on failure. Check scope, declared ordering key, capacity and idempotency for durable delivery. [FX-0110-W5-VOLATILE-CALLBACK-DURABLE-REPLAY](../planning/feature-wave-5.md#acceptance-design-for-n-1767-to-n-1790). | BF-04, BF-05, BF-06, BF-25, BF-26, BF-31 |
| N-1784 Declarative state-machine engine | Validate the exact declared graph, guards and terminal states. Bind site, expected state and revision and one transition identity. Update the eligible action and state together inside the owning transaction. Fail or hold without an external effect on a guard or version conflict. Generated Quint graph proof does not prove arbitrary Rust actions. | BF-15, BF-16, BF-34, BF-36, BF-37 · [FX-0110-W5-EXACT-SITE-STATE-TRANSITION](../planning/feature-wave-5.md#acceptance-design-for-n-1767-to-n-1790) |
| N-1785 Distributed locks and leader election engine | Grant an exact scoped resource lease with database-clock expiry and monotonic fence. The owning resource verifies the current fence in the same transaction as its protected write. Stop acting at lost renewal or scope. Restart and restore cannot revive a superseded fence. Product keeps one local PostgreSQL authority and admits no alternative lock store or cluster. | BF-17, BF-18, BF-23, BF-34, BF-38 · [FX-0110-W5-CURRENT-LEASE-FENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1767-to-n-1790) |

#### Optional broker starters (N-1845–N-1846)

| Owner catalogue entry | Exact messaging contract |
| --- | --- |
| N-1845 Clean-room Apache Kafka engine | Qualify the declared producer, consumer, stream and optional unverified share-consumer profiles independently through admitted components. Bind cluster, topic, partition, schema, actor and effect identity. Preserve acknowledgement and checkpoint rules, bounded retry, dead-letter ownership and current scope after interruption. Unknown settlement holds. The product binary excludes brokers. [FX-0110-W5-KAFKA-SYNTHETIC-SETTLEMENT-ONLY](../planning/feature-wave-5.md#acceptance-design-for-n-1839-to-n-1862). |
| N-1846 Generic AMQP 1.0 and RabbitMQ clients | Keep AMQP 0-9-1, AMQP 1.0 and RabbitMQ Stream evidence separate. Apply exact exchange, route, virtual-host and message authority. Acknowledgement, lost reply, retry and dead-letter stay under 0110. Virtual-host administration requires its own action and proof. No product client, automatic provisioning, broker call or outbound send from a fixture. [FX-0110-W5-AMQP-EXACT-PROFILE-AND-SETTLEMENT](../planning/feature-wave-5.md#acceptance-design-for-n-1839-to-n-1862). |

BF-20 blocks absent usable admitted clients. A stub or skipped
fixture cannot claim an adapter profile.

### Data

- Tables, defined by this spec (with site column and forced row-level security
  under spec 0108's conventions): `outbox`, `inbox`, `dead_letters`,
  `schedule_runs`, `locks`, `state_machines`. Spec 0019 uses these
  primitives and owns its product job queue and attempt history.
- Ledger and behaviour descriptions: repository files (spec 0081).
- Module manifests: files in each module (AGENTS.md section 5).

### Settings

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `outbox.batch` | whole number | 100 | 1 to 10,000 | next start |
| `outbox.max_payload_kb` | KiB | 256 | 1 to 4,096 | next start |
| `outbox.retry_max_delay_s` | seconds | 300 | 1 to 3,600 (proposed) | next start |
| `outbox.relay_jitter_ms` | milliseconds | 1,000 | 0 to 60,000 | next start |
| `outbox.backlog_alarm` | whole number | 10,000 | 1 to 10,000,000 | next start |
| `outbox.max_rows` | whole number | 1,000,000 (proposed) | 1,000 to 100,000,000 (proposed) | next start |
| `inbox.retention_days` | days | 14 | 1 to 730 | next night |
| `dead_letter.retention_days` | days | 30 | 1 to 730 | next night |
| `outbox.poll_ms` | milliseconds | 500 | 50 to 60,000 | next start |
| `outbox.max_attempts` | whole number | 10 | 1 to 1,000 | next start |
| `outbox.retention_days` | days | 7 | 1 to 365 | next night |
| `event.buffer` | whole number | 1,000 | 10 to 100,000 | next start |
| `event.buffer_mib` | MiB | 16 | 1 to 1,024 | next start |
| `scheduler.max_catchup` | whole number | 10 | 0 to 1,000 | next start |
| `scheduler.default_misfire` | skip, run_once, run_missed | run_once | three values | next start |
| `scheduler.max_run_s` | seconds | 3,600 | 1 to 86,400 | next start |
| `scheduler.default_overlap` | forbid, allow, queue_one | forbid | three values | next start |
| `lock.lease_s` | seconds | 30 | 5 to 3,600 | next start |
| `lock.renew_s` | seconds | 9 | 1 to 1,200 | next start |
| `resilience.retry_max_attempts` | whole number | 3 | 1 to 20 | next start |
| `resilience.retry_base_ms` | milliseconds | 200 | 10 to 60,000 | next start |
| `resilience.retry_max_delay_ms` | milliseconds | 10,000 | 100 to 600,000 | next start |
| `resilience.breaker_failure_pct` | percent | 50 | 1 to 100 | next start |
| `resilience.breaker_window` | whole number | 20 | 5 to 1,000 | next start |
| `resilience.breaker_open_s` | seconds | 30 | 1 to 3,600 | next start |
| `resilience.bulkhead_max` | whole number | 10 | 1 to 10,000 | next start |
| `resilience.bulkhead_queue` | whole number | 20 | 0 to 10,000 | next start |
| `resilience.timeout_ms` | milliseconds (per attempt) | 5,000 | 100 to 600,000 | next start |
| `resilience.deadline_ms` | milliseconds (overall) | 30,000 | 100 to 3,600,000 | next start |
| `resilience.rate_per_s` | whole number | 50 | 1 to 100,000 | next start |

`lock.renew_s` must be strictly less than a third of `lock.lease_s`; a start
with another value fails. There is no setting that turns off the outbox
pairing, the inbox check or the fencing token.

### Failure codes

SUT-1000 outbox write outside a transaction; SUT-1001 message moved to dead
letters; SUT-1002 duplicate message ignored (information, not an error);
SUT-1003 module wall breach; SUT-1004 circuit open; SUT-1005 retry limit
reached; SUT-1006 task overlap refused; SUT-1007 missed run skipped by policy;
SUT-1008 lock lost or fence refused; SUT-1009 state transition refused;
SUT-1010 raw task spawn without context; SUT-1011 channel full; SUT-1012
broker adapter blocked (no usable client); SUT-1013 after-commit listener
failed; SUT-1014 relay backing off (information). Codes from SUT-1000 on use
four digits, and the range 1000 to 1099 is reserved for this spec.

### Security and privacy

- Payloads of events and messages follow the secret-masking rules: no secret
  appears in a log, a dead letter or a metric label.
- A message to a destination outside the PC is refused unless the owner's switch
  for it is on.
- A listener, script or workflow that receives an event can only create
  proposals (spec 0099).
- Event payloads that hold personal data are kept only as long as the outbox
  retention and are removed by erasure requests (spec 0032).

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| BF-01 | An outbox row MUST be written in the caller's transaction and MUST exist only if the change committed. | UT-BF-01, FI-BF-01 |
| BF-02 | The relay MUST NOT send one row from two relays at once, and a repeat after a crash MUST be absorbed by the inbox. | IT-BF-01, FI-BF-02 |
| BF-03 | A message failing the attempt limit MUST move to dead letters with its error. | UT-BF-02 |
| BF-04 | An after-commit listener MUST NOT run if the transaction rolls back. | UT-BF-03 |
| BF-05 | A failing listener MUST be reported and MUST NOT stop later listeners. | UT-BF-04 |
| BF-06 | Durable events MUST keep order per ordering key and MUST apply back-pressure rather than grow without bound. | IT-BF-02, FI-BF-03 |
| BF-07 | An undeclared dependency, a cycle, use of another module's internals or an undeclared event MUST fail the build or test. | UT-BF-05, PL-BF-01 |
| BF-08 | A child task MUST inherit the context, MUST NOT inherit a transaction, and MUST NOT raise privileges. | UT-BF-06 |
| BF-09 | Raw task spawn in product code MUST be refused by the gate. | PL-BF-02 |
| BF-10 | The scheduler MUST run a task on one node at a time, with a lease and fencing token, and apply the overlap and misfire policies. | UT-BF-07, IT-BF-03 |
| BF-11 | Cron next-run times MUST follow the stated daylight-saving rule. | UT-BF-08 |
| BF-12 | Retry MUST apply only to retryable errors on idempotent calls, with backoff and jitter. | UT-BF-09 |
| BF-13 | A circuit breaker MUST open on the stated failure rate, fail fast with SUT-1004 and probe before closing. | UT-BF-10 |
| BF-14 | An unsafe policy order MUST be refused at build time. | UT-BF-11 |
| BF-15 | A state machine with an unreachable state, an ambiguous transition or a final-state exit MUST fail the build. | UT-BF-12 |
| BF-16 | A transition MUST apply only from the stored state and version, and a failing action MUST roll it back. | IT-BF-04 |
| BF-17 | A lock MUST return a rising fencing token, and an older token MUST be refused by the resource. | UT-BF-13, FI-BF-04 |
| BF-18 | A holder that cannot renew MUST stop acting at once. | FI-BF-05 |
| BF-19 | Every channel MUST be bounded and MUST apply back-pressure or a stated rejection. | UT-BF-14 |
| BF-20 | A broker adapter with no usable client MUST be marked blocked, not faked. | AU-BF-01 |
| BF-21 | Each feature MUST have parity rows, behaviour descriptions and independent contract tests. | CT-BF-01 |
| BF-22 | No adapter MUST send a message off the PC without the owner's switch. | UT-BF-15 |
| BF-23 | `lock.renew_s` MUST be strictly less than a third of `lock.lease_s`. | UT-BF-16 |
| BF-24 | The Sutradhar build MUST contain no broker adapter. | UT-BF-17 |
| BF-25 | A handler on another task MUST see the committed data of the publisher. | IT-BF-05 |
| BF-26 | The module map MUST be generated from the manifests, and an event published by a module that did not declare it MUST NOT compile, and MUST fail at run time in tests with SUT-1003. | UT-BF-18, PL-BF-01 |
| BF-27 | Two relays working one ordering key MUST NOT send a later row before an earlier one, and a strict key MUST stall on a failing first row. | IT-BF-06, FI-BF-06 |
| BF-28 | An inbox conflict MUST acknowledge and skip with SUT-1002, and the dedupe key MUST be the outbox row id or the declared key. | UT-BF-19 |
| BF-29 | Attempts MUST be counted before the send, with exponential backoff and a relay start jitter. | UT-BF-20, FI-BF-07 |
| BF-30 | A payload over `outbox.max_payload_kb` MUST be refused at write time. | UT-BF-21 |
| BF-31 | An after-commit listener failure MUST be reported with SUT-1013 and MUST NOT undo the commit. | UT-BF-22 |
| BF-32 | The relay and scheduler MUST open a site transaction for their system principal before any read or write. | IT-BF-07 |
| BF-33 | At shutdown the relay MUST stop claiming and finish and mark sends in flight inside the drain budget. | FI-BF-08 |
| BF-34 | A fence MUST be checked in the same transaction as the write it guards. | IT-BF-08 |
| BF-35 | The scheduler MUST use the database clock, apply the misfire and overlap policies (SUT-1006, SUT-1007), and block a second instance. | UT-BF-23, FI-BF-09 |
| BF-36 | A state machine with two transitions for one state and event MUST need named guards and a priority or exclusive declaration. | UT-BF-24 |
| BF-37 | The Quint export MUST cover the declared graph and be generated, not written by hand. | UT-BF-25 |
| BF-38 | A leader that cannot renew MUST stop acting, and an election MUST give one leader. | IT-BF-09 |
| BF-39 | Integration channels, aggregators and claim checks MUST be bounded and MUST record every dropped or failed message. | UT-BF-26 |
| BF-40 | Broker adapters MUST support manual or after-success acknowledgement, retry and dead-letter handling through the resilience layer. | CT-BF-02 |
| BF-41 | Each limited setting, including `event.buffer`, `outbox.retention_days` and `scheduler.default_overlap`, MUST be enforced. | UT-BF-27 |

## How it is tested

Designed, not run.

- **Unit (UT-BF):** the outbox write, listener rules, module checks, context
  rules, scheduler arithmetic (including a clock change and daylight-saving
  cases, with one value just inside and one just outside), retry and breaker
  state tables, state machine checks, lock tokens. UT-BF-23 also holds
  database-clock decisions while local time changes. Join its missing-time
  case to 0019 IT-0019-SCHEDULER-RESTART for the durable occurrence, lease
  expiry boundary and safe recovery. Exercise the independent monotonic
  run limit during the fault. Stop the task at its safe point without
  treating that stop as proof that the database lease expired.
- **Planted faults (PL-BF):** a module that reads another's internals; a raw
  task spawn, a blocking-task spawn and a thread spawn; each must be refused
  with its named message while an honest example passes.
- **Integration (IT-BF):** a real PostgreSQL 18 started by the test harness,
  with no Docker: two relays, three schedulers, a lock contest, a transition
  race.
- **Fault injection (FI-BF):** the app killed between commit and send, a lost
  acknowledgement, a paused lock holder, a slow consumer.
- **Behaviour (BD-BF):** the example above, step by step.
- **Contract (CT-BF):** one test per parity item, written from the behaviour
  description by someone other than the implementer.
- **Audit (AU-BF):** each blocked broker is listed with its reason.
- **Measurement (MS-BF-01):** outbox throughput and relay lag on the 8 GiB
  reference at the default settings; scheduler cost per run.

## Changes from the source

The pillar list asked for clean-room Kafka, AMQP and other engines. Behemoth
Boot wraps mature crates instead and keeps only the guarantees, policies and
checks as our own code (AGENTS.md section 6 rule 14). Spring Modulith's
checks are matched through manifests and the real dependency graph, not by
reading annotations. ADR-0021 and AGENTS.md section 5.1 define the product
queue and outbox. Broker adapters are framework parity only.
