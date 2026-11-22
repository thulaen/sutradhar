# 0099: Jobs and workflow screen

The Jobs screen is where an owner sees, understands and steers everything
Sutradhar does in the background: crawls, indexing, model downloads, reports,
backups, test runs and the workflows the owner builds. It replaces what
Airflow gave (dependency views, run history, retries) with a screen written
by us, as ADR-0021 decided. The same spec owns the workflow builder: the
blocks, triggers, actions, versions, dry runs, approval steps and the alert
centre. It owns what a person sees and does. The scheduler that runs the work
belongs to spec 0019 (our own Axum and Tokio scheduler),
the memory numbers to spec 0078, scripts to spec 0104 and the action list
behind the buttons to spec 0020.

Example: Asha opens Jobs on Monday. The top line says "Running 2 jobs, 1
waiting for memory, machine running cool." The link check is amber. She
hovers the "Why is this waiting?" pill and reads "Waiting for 2 GB of memory
to free up". She clicks "Cut to the front"; the nightly report pauses without
losing progress and the link check runs. Later a crawl fails. The card shows
"Try 2 of 3, trying again in 45 seconds because the other website was busy",
and the "What changed?" view shows that the crawl had 30% more pages than
yesterday. She clicks "Dry run" on a new workflow, sees what it would do,
approves one step with a thumbs-up and leaves it to run at 02:00.

Values marked "proposed" are design choices to be checked by measurement.

## What it does

### Words used here

- **Job**: one unit of background work with a state, an owner module and
  a cost (spec 0078's job classes).
- **Workflow**: a saved graph of steps (blocks) joined by arrows, started by a
  trigger. A run of a workflow creates jobs.
- **Block**: one step: a trigger, an action, a script step, an AI step, a
  human approval, a branch, a loop, a sub-workflow or an error handler.
- **Run**: one execution of a workflow, with its inputs, outputs and timing.
- **Dry run**: a run that shows what each step would do and changes nothing.
- **Proposal**: a change a workflow wants to make to a site; only a human
  accepts it (AGENTS.md section 5).
- **Engine Room**: the settings view with every deep control (ADR-0021).

### Screens

WCAG 2.2 AA throughout; every colour has text; every graph has a list view.

1. **Overview.** One live plain-English headline pinned at the top. Below it:
   running, waiting, failed and finished jobs, each with a progress bar split
   into named steps ("1. Gathering pages, 2. Analysing links 64%, 3. Saving
   report"). The window title carries the current percentage.
2. **Why is this waiting?** Every waiting job has a pill that names the cause
   (memory, a higher priority job, a quiet period, a blocked dependency, a
   drive that is not connected, low battery, a site asking us to slow down).
3. **Workflow map.** The graph of steps, drawn as a transit map: moving dots
   are real tasks. Zoom out for a three-word summary of the workflow and in
   for single pages and live logs. Hovering a slow or failed step lights every
   step that depends on it. Boxes glow amber or red when a step takes longer
   or uses more memory than normal. A corner mini-map and a one-click tidy
   keep large graphs readable. Clicking an arrow shows a small, masked sample
   of the data that flows between two steps.
4. **Builder.** Plain-English blocks snap together, with no config file. A
   text box can draw a first graph from a sentence (an AI step, proposal only).
   Sticky notes can be pinned to any box. Safe-latch permissions let a
   beginner view and "Run now" while deeper switches stay behind the Engine
   Room unlock.
5. **Run history.** Filter eligible retained runs in plain words
   ("only failed jobs", "only overnight"). The 24-hour timeline shows
   the available history. Show each permitted result with its
   original workflow version. "Resume from here" and "Replay with
   the same inputs" require the current recovery checks for that
   run. Show the safe reason when a recovery action is held.
   Checkpoint bookmarks use their own current read checks.
6. **Fix and explain.** Common failures carry a direct button that opens the
   exact setting (an expired key, a full drive). Hovering a technical word
   shows a five-word explanation. The blame splitter says whether a slowdown
   came from this PC, the Python sidecar, the internet or an outside site.
   "Ask for help" bundles the graph, the plain-English error and scrubbed
   details into one summary to paste into support or an AI chat.
7. **Scheduling.** A traffic-jam calendar shows load by hour. "Auto-spread"
   moves jobs to quiet hours; "Finish by 07:00" plans backwards from a
   deadline; blackout dates and quiet hours stop work; a deadline badge turns
   amber when a job drifts behind. Estimates are honest: they show a range
   and say how many past runs they are based on.
8. **Alert centre.** All alerts in one list with severity, routing rules per
   alert type, quiet hours, a digest, and snooze and acknowledge.
9. **Reports.** A reliability card with grades A to F per workflow, a speed
   trend per job, a one-page monthly health report, and a resource receipt
   after each run (time, memory, disk, pages, outside calls).

#### Workflow views, explanations and recovery (N-1358–N-1381)

Use the existing Jobs views, immutable workflow graph and durable
records from spec 0019. Bind the exact site, actor, workflow,
graph and run revisions and the view's scope and as-of time.
Check authority before reading detail, following an edge,
comparing runs, returning a sample or accepting an action.
The owner still uses spec 0020's approval route for effects.
Changing a view does not dispatch work. Show a safe unknown or
unavailable state when a required fact is stale or missing.

The canvas, keyboard list and accessible labels use the same
admitted visible nodes, edges, events and status. Protect scope
before counts, links, focus, logs, samples, snapshots, exports
and replay. Use bounded reads and update rates under specs 0078
and 0099. Honour reduced motion. Do not reveal a hidden task by
animation, totals, layout or elapsed time. Use spec 0032's scrub
and erasure rules. Recheck protected data on restore. QR-198
qualifies the exact visual and transition profiles.

| Feature | Exact view or action | Requirements and planned fixture |
|---|---|---|
| N-1358 Living subway map | Show admitted dependency edges and animate only recorded eligible task movement for the exact run. Paused, disconnected and stale views have named states. The accessible list presents equivalent activity with reduced or absent motion. | JW-02, JW-03 · [FX-0099-W5-RECORDED-ELIGIBLE-MOVEMENT](../planning/feature-wave-5.md#acceptance-design-for-n-1358-to-n-1381) |
| N-1359 Zoom from summary to detail | Pin the bounded plain-English summary, progress, steps and authorised page or log detail to the same snapshot. Use three words for the overview where the admitted summary can remain accurate. Detail is read on demand and grants no extra rights or remote reads. | JW-02, JW-03, JW-15 · [FX-0099-W5-PINNED-SUMMARY-DETAIL](../planning/feature-wave-5.md#acceptance-design-for-n-1358-to-n-1381) |
| N-1360 Click-to-trace dominoes | From an eligible slow or failed step, show reachable eligible dependants from the stored graph, with keyboard focus or selection as well as hover. Distinguish dependency from execution and actual blocking. Keep traversal bounded and never reveal hidden nodes. | JW-03, JW-08 · [FX-0099-W5-VISIBLE-DEPENDENCY-TRACE](../planning/feature-wave-5.md#acceptance-design-for-n-1358-to-n-1381) |
| N-1361 Drag-and-drop detours | Suggest a typed replacement path around a declared optional step in a new local graph revision. Preserve required steps, approvals and downstream input definitions. Show the exact change and use ordinary validation and authorisation before a later run. An active run keeps its pinned graph and may only use its existing admitted fallback. | JW-04, JW-06, JW-08, JW-16 · [FX-0099-W5-TYPED-OPTIONAL-DETOUR](../planning/feature-wave-5.md#acceptance-design-for-n-1358-to-n-1381) |
| N-1362 Colour-coded traffic lanes | Draw the admitted 0019 and 0078 lane, wait reason and eligible progress. Use text and accessible order for each class. Explain fair turn-taking without promising service from the image. No change to priority, a lease or admission. | JW-01, JW-03, JW-10 · [FX-0099-W5-SCOPED-LANE-WAIT-REASON](../planning/feature-wave-5.md#acceptance-design-for-n-1358-to-n-1381) |
| N-1363 X-ray data peek | Show a bounded typed masked projection from the exact authorised edge, run and recorded sample. Permit only allowed fields and scrub combinations that identify people or expose secrets. If no safe sample exists, show that state. No live hook, new upstream fetch, raw vault value or replay leak. | JW-03, JW-12, JW-15 · [FX-0099-W5-MASKED-EDGE-SAMPLE](../planning/feature-wave-5.md#acceptance-design-for-n-1358-to-n-1381) |
| N-1364 Heat-glow bottlenecks | Compare current admitted timing or memory measurements with the exact comparable normal profile. Label the measured reason with the visual cue. Unknown baseline, incomplete observations or incompatible class stay unknown. The display neither sets a threshold nor changes job priority. | JW-02, JW-03, JW-17 · [FX-0099-W5-COMPARABLE-BOTTLENECK-REASON](../planning/feature-wave-5.md#acceptance-design-for-n-1358-to-n-1381) |
| N-1365 Sidecar bridge indicator | Show start, hand-off, reply or failure only from the admitted 0104 work receipt for the exact run, step and admitted attempt under 0019. A fenced attempt cannot update the current indicator with a late reply. Only the owning current record or an eligible original effect receipt reconciled under 0019 can change the displayed outcome. Distinguish queued, handed over and uncertain. No script call, new worker, payload disclosure or fallback follows from opening this view. | JW-02, JW-03, JW-15 · [FX-0099-W5-SIDECAR-EXACT-RECEIPT](../planning/feature-wave-5.md#acceptance-design-for-n-1358-to-n-1381) |
| N-1366 Snap-together workflow blocks | Assemble local draft blocks from the admitted catalogue. Check typed port matches, exact action versions, scope, graph bounds and allowed topology before save. Preserve keyboard authoring and validation parity. Saving creates a draft revision with no activation or effect. | JW-04, JW-06, JW-16 · [FX-0099-W5-TYPED-PORT-AUTHORING](../planning/feature-wave-5.md#acceptance-design-for-n-1358-to-n-1381) |
| N-1367 Corner radar mini-map | Derive the viewport and position controls from the same authorised graph. Show equivalent keyboard location and navigation. A viewport move changes only the view. Hidden labels and nodes do not leak through geometry or counts. | JW-03, JW-15 · [FX-0099-W5-AUTHORISED-MINI-MAP](../planning/feature-wave-5.md#acceptance-design-for-n-1358-to-n-1381) |
| N-1368 One-click tidy canvas | Propose a deterministic readable layout of the same graph and keep focus stable. Reduce crossings where possible. Layout alone cannot change dependencies, order of execution, approvals or the pinned run graph. | JW-03, JW-06 · [FX-0099-W5-LAYOUT-ONLY-TIDY](../planning/feature-wave-5.md#acceptance-design-for-n-1358-to-n-1381) |
| N-1369 Crash translator | Map an admitted error code and safe context to a short plain-English reason and indicate if an authorised person needs to act. Unknown errors need a safe reason and code. Raw dumps and guessed causes never appear. | JW-02, JW-15, JW-18 · [FX-0099-W5-SAFE-ERROR-TRANSLATION](../planning/feature-wave-5.md#acceptance-design-for-n-1358-to-n-1381) |
| N-1370 One-click fix buttons | Open the exact eligible existing screen or setting for an admitted failure and show its intended scope. Check current grant and route. Opening the screen cannot write the setting, rotate a key, bypass confirmation or retry the job. | JW-16, JW-20 · [FX-0099-W5-GRANT-CHECKED-REMEDY-VIEW](../planning/feature-wave-5.md#acceptance-design-for-n-1358-to-n-1381) |
| N-1371 Reliability report card | Use the admitted A-to-F rule, exact workflow and comparable versioned run populations. State the period, denominator, coverage and treatment of retries, cancellation and missing evidence. Unknown or sparse coverage has no grade. A grade changes no work. | JW-17, JW-22 · [FX-0099-W5-RELIABILITY-COMPARABLE-RUNS](../planning/feature-wave-5.md#acceptance-design-for-n-1358-to-n-1381) |
| N-1372 Plain-English retry countdown | Show current attempt, admitted limit, known scheduler due instant and safe cause from the same retry record. Use the scheduler clock and distinguish held, due and waiting. Clock uncertainty holds the countdown. A restart or clock change cannot dispatch a retry from the countdown. | JW-08, JW-17 · [FX-0099-W5-RETRY-COUNTDOWN-CURRENT-CLOCK](../planning/feature-wave-5.md#acceptance-design-for-n-1358-to-n-1381) |
| N-1373 Blame splitter | Show supported evidence for a local resource, sidecar, connection or outside response and its confidence or gap. Untested or coincident facts remain unattributed. Opening the explanation starts no probe and cannot establish that a site blocked traffic. | JW-02, JW-15, JW-17 · [FX-0099-W5-SUPPORTED-CAUSE-OR-UNKNOWN](../planning/feature-wave-5.md#acceptance-design-for-n-1358-to-n-1381) |
| N-1374 Instant word tooltips | Render a short approved glossary explanation in the current context. Match the visible term and provide keyboard and touch access. Where five words would lose meaning, use a short label with an accessible fuller explanation. No model or outside lookup. | JW-03, JW-18 · [FX-0099-W5-APPROVED-ACCESSIBLE-GLOSSARY](../planning/feature-wave-5.md#acceptance-design-for-n-1358-to-n-1381) |
| N-1375 Noise-cancelling logs | Fold admitted repeated low-priority diagnostic messages inside the authorised run and show safe counts and ordering. Preserve warning, failure and required audit visibility. Folding affects only the local display. It never removes stored records or exposes masked fields. | JW-15, JW-22 · [FX-0099-W5-DISPLAY-ONLY-LOG-FOLDING](../planning/feature-wave-5.md#acceptance-design-for-n-1358-to-n-1381) |
| N-1376 Calm amber retries | Use the existing retry and severity evidence. Distinguish an active automatic retry from an exhausted or intervention-required failure with text, icon and colour. Ambiguous state stays identified as unknown. Colour cannot suppress an alert, change retry policy or send a message. | JW-03, JW-08, JW-13 · [FX-0099-W5-AMBER-RETRY-SAME-SEVERITY](../planning/feature-wave-5.md#acceptance-design-for-n-1358-to-n-1381) |
| N-1377 24-hour timeline scrubber | The 24-hour viewport shows only the intersection of eligible retained events and samples under jobs.history_days and jobs.failed_history_months. Opening it adds no collection or retention. Show exact observation boundaries, missing intervals and current access. Browsing history uses a read-only reconstruction, cannot run a job and never recreates erased input. | JW-03, JW-15, JW-22 · [FX-0099-W5-TIMELINE-RETAINED-READ-ONLY](../planning/feature-wave-5.md#acceptance-design-for-n-1358-to-n-1381) |
| N-1378 Resume from right here | Offer the last compatible committed checkpoint for the exact run, graph and input. Recheck active site, exact lease generation, current pause and cancellation, class limits, erasure, grants and effect receipts. Preserve run, node and loop effect identities. Reconcile uncertain receipts through the owning route. Unknown stays held without retry or fallback. Resume only through 0019's fenced path with stable effect IDs. An arbitrary node, stale checkpoint or unknown receipt holds. | JW-06, JW-08, JW-09, JW-16 · [FX-0099-W5-RESUME-EXACT-COMMITTED-CHECKPOINT](../planning/feature-wave-5.md#acceptance-design-for-n-1358-to-n-1381) |
| N-1379 No-risk dry run | Preview the exact local graph against eligible bounded scratch inputs and show proposed effects and refusal reasons. Use synthetic data by default. JW-19 owns the exact scrubbed send preview. N-1384 and 0038 own a separate admitted real-item cap. A dry run cannot borrow real inputs or reserve an outbound action. No site or product write, live effect, send, credential reveal, confirmation or quota reservation. Discard scratch after the test. | JW-04, JW-05, JW-15, JW-23 · [FX-0099-W5-DRY-RUN-NO-AUTHORITY-OR-EFFECT](../planning/feature-wave-5.md#acceptance-design-for-n-1358-to-n-1381) |
| N-1380 Ghost-car speed race | Overlay two authorised comparable run histories by exact version, step IDs and measured clock basis. Label changed graph, inputs, capacity or gaps. Provide an accessible ordered comparison and avoid a causal claim when the runs differ. No rerun or ranking input. | JW-03, JW-17, JW-22 · [FX-0099-W5-RECORDED-RUN-COMPARABILITY](../planning/feature-wave-5.md#acceptance-design-for-n-1358-to-n-1381) |
| N-1381 One-click undo | Show the exact already admitted reversible local effect, recoverable snapshot, current target and proposed result. Recheck grant, version, expiry, erasure and receipt and require the existing owner confirmation. Bind the original local effect and exact current base. Use the owning compare-and-set and one stable undo receipt. Reconcile a lost reply on that receipt. Never retry with a fresh key. Later edits, uncertain outcome, native changes or irreversible effects are held. Cancel does not roll back work. | JW-04, JW-06, JW-16, JW-20 · [FX-0099-W5-UNDO-EXACT-LOCAL-EFFECT](../planning/feature-wave-5.md#acceptance-design-for-n-1358-to-n-1381) |

Keep the source-established labels, IDs and ownership shown above.

#### Job control, explanations and live views (N-1406–N-1429)

Use the current site and actor, eligible job, graph and run
revisions and a protected snapshot. Apply access and erasure
before returning entries, labels, totals, logs, search hits,
cursor state or notifications. Use the same admitted projection
for the visual and accessible text views. Label stale, offline
and unknown evidence. Bound reads, sampling, saved projections and
update rates under specs 0078 and 0099. Use existing secure local
session and audit rules under specs 0020, 0032 and 0118.

Use one current access and projection generation across display,
buffer and replay. Bind bounded streams to monotonic sequence,
current cursor and safe fan-out. Scope before grouping, counting
or export to an OS surface. On lock, denial or scope change, clear
wall, taskbar, title and audio state. Expose gaps from pressure
without replaying hidden events. Do not infer global next work
from scoped data or leak hidden jobs through omitted positions.

These forms reuse the scheduler, action and alert authorities.
Usage, duration, resource and any admitted cost already recorded
by an owning route may be displayed within scope. Keep measured
values, estimates and missing evidence distinct. An explanation,
preview, forecast or usage display cannot call a provider, obtain
prices, make or resize a reservation, spend, or enable delivery.
Paid lookups and approved sends retain their own exact routes,
limits and approvals. QR-200 qualifies these forms.

| Feature | Exact local view or action | Owning requirement |
|---|---|---|
| N-1406 Double-click twin blocker | Deduplicate on the service against exact site, actor, action, request intent, graph and input digest and current eligibility. Commit run, lease and receipt together. Same key with different bytes conflicts. Explicitly admit each genuinely new run. A second click offers the active run only if the actor may see it. Reconcile an uncertain receipt. A stale or repeated start cannot create a second run. A genuinely different job still needs its own admission. [FX-0099-W5-ONE-RUN-INTENT-TWICE](../planning/feature-wave-5.md#acceptance-design-for-n-1406-to-n-1429). | JW-09, JW-16 |
| N-1407 Early trouble forecasting | Use an admitted set of comparable exact job classes, versions, workload and resource observations against the current budget. Pin method, units, minimum comparable sample, freshness and exclusions including waits or failed runs. Show coverage and calibrated horizon and interval, or unknown when the trend is unsupported. A predicted limit cannot rewrite budgets, reserve capacity, prefetch a remote page or start work. [FX-0099-W5-FORECAST-CURRENT-HOST-BUDGET](../planning/feature-wave-5.md#acceptance-design-for-n-1406-to-n-1429). | JW-17, JW-21 |
| N-1408 Universal command bar | Parse typed words to bounded typed 0020 plans with exact ordered affected actions and jobs, site, graph and action revisions, requested and canonical times and authority expiry. Ambiguous time or scope holds. Show the resolved set and use exact one-use 0020 owner confirmation where required. Recheck at commit and show each permitted outcome including refusal and partial groups under the existing bulk contract. A phrase, model or preview cannot confirm, expand scope, spend or retry an uncertain effect. [FX-0099-W5-COMMAND-EXACT-SCOPED-CONFIRMATION](../planning/feature-wave-5.md#acceptance-design-for-n-1406-to-n-1429). | JW-08, JW-16, JW-20 |
| N-1409 Highlight-to-explain | Give a short explanation of the selected eligible graph region using its pinned nodes, boundaries, inputs and dependencies. Use protected typed data and the current view. Missing or hidden relationships stay unknown. Selection cannot run a block, open private data or dispatch a model outside the admitted route. [FX-0099-W5-SCOPED-REGION-EXPLANATION](../planning/feature-wave-5.md#acceptance-design-for-n-1406-to-n-1429). | JW-03, JW-15, JW-16 |
| N-1410 Speed coach | Compare eligible like-for-like timing, wait and resource evidence for the exact graph. Cite the measured bottleneck and identify any proposed parallel work that fits current dependencies, capacity and job profiles. Unknown support yields an explanation only. The proposal uses separate 0019 graph and resource admission, exact review and grants. It gives no lease, start or confirmation. [FX-0099-W5-SPEED-COACH-DEPENDENCY-FENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1406-to-n-1429). | JW-02, JW-17, JW-20 |
| N-1411 Text-to-workflow drawing | Convert typed text through the admitted local wording route into a proposed graph of allowed catalogue blocks. Check bounded typed ports, effects, scopes, loops and resource needs. Unknown or protected actions hold. Generated text cannot activate, call a provider or confirm. A saved candidate remains an inert draft. [FX-0099-W5-INERT-LOCAL-GRAPH-DRAWING](../planning/feature-wave-5.md#acceptance-design-for-n-1406-to-n-1429). | JW-04, JW-06, JW-16, JW-20 |
| N-1412 Morning briefing card | Use the admitted local morning boundary and exact eligible overnight job results. Show a short count of finished work, scoped measured outcomes and any evidenced item needing attention. Distinguish cancelled, partial, unknown and completed. Another day's results or hidden scopes cannot fill gaps. Refresh only from existing authorised local records, without starting wording, recovery or scheduling. No send. [FX-0099-W5-BRIEFING-LOCAL-MORNING-SCOPE](../planning/feature-wave-5.md#acceptance-design-for-n-1406-to-n-1429). | JW-02, JW-13, JW-18 |
| N-1413 Smart outage bundling | Use one safe correlation identity and bounded alert group under the existing 0032/0099 incident ledger. Group only recorded shared cause, scope and time. Keep unrelated protected outcomes and no new watch or dispatch. Preserve different causes, current severity and each permitted outcome. An unknown cause cannot prove a shared outage. Replays and corrections produce one eligible summary, with no send or acknowledgement by implication. [FX-0099-W5-OUTAGE-BUNDLE-EXACT-CAUSE](../planning/feature-wave-5.md#acceptance-design-for-n-1406-to-n-1429). | JW-13, JW-15, JW-22 |
| N-1414 Plain-English history search | Compile text to a bounded, inert filter with exact time basis, site, list scope and supported fields. Confirm ambiguous date or query meaning through the local view. Apply grants before searching, paginating or counting. No new crawl, command or broader source read follows from a search. [FX-0099-W5-HISTORY-FILTER-SCOPE-BEFORE-PAGE](../planning/feature-wave-5.md#acceptance-design-for-n-1406-to-n-1429). | JW-15, JW-16, JW-22 |
| N-1415 Dusty-job cleanup nudges | Show a local pause proposal only from independently permitted complete local evidence for the configured period and schedule. No extra usage watch or store. Missing telemetry alone cannot prove disuse. The source's 60-day example needs an admitted profile. Missing evidence is unknown. Recheck current use and scope before a separate authorised pause. No automatic pause, delete or data change. [FX-0099-W5-DISUSE-PAUSE-PROPOSAL-RECHECK](../planning/feature-wave-5.md#acceptance-design-for-n-1406-to-n-1429). | JW-16, JW-17, JW-20 |
| N-1416 Voice job controls | The spoken route is unavailable pending the owner decision under QR-027. Keep typed controls under their own current grant and preview. Do not request microphone permission, record audio, start speech recognition or execute a restored voice request. [FX-0099-W5-UNAVAILABLE-SPOKEN-PATH](../planning/feature-wave-5.md#acceptance-design-for-n-1406-to-n-1429). | JW-16, JW-20; QR-027 |
| N-1417 Auto-written playbook | Refresh a bounded deterministic projection of authorised local records for the exact site, graph and rule revision. A model-written alternative requires an explicit existing action with current grant and late-result fence. Graph or UI events start no model work, scheduled job, reservation or recovery. Keep exact sources and show stale wording. Treat all instructions as inert text. Reading or regenerating the guide runs no step. [FX-0099-W5-PLAYBOOK-PINNED-REVISION](../planning/feature-wave-5.md#acceptance-design-for-n-1406-to-n-1429). | JW-04, JW-06, JW-15 |
| N-1418 Safety confidence tag | Label the exact proposed effects, proved reversibility, prerequisites, policy version and residual effects after undo before confirmation. Bind the evidence to the exact shown plan. State uncertainty or hold when proof is missing. The label cannot promise zero risk, substitute for an undo proof or grant authority. Changed action, scope or target voids the label. | JW-04, JW-16, JW-20 · [FX-0099-W5-EFFECT-REVERSIBILITY-EVIDENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1406-to-n-1429) |
| N-1419 Mission-control wall mode | Present the same local eligible Jobs projection full-screen on an owner display. Preserve equivalent accessible controls and exit. Recheck visibility on scope change, hide private detail and show stale or disconnected status. No live share, remote display link, service control or new session. | JW-02, JW-03, JW-12, JW-18 · [FX-0099-W5-SCOPED-LOCAL-WALL](../planning/feature-wave-5.md#acceptance-design-for-n-1406-to-n-1429) |
| N-1420 Optional gentle soundscape | Off by default. Give a bounded local audio cue only from an admitted current completion or needs-input state. Honour mute, reduced interruption and duplicate-event handling. Convey the status in text as well. No device pairing, audio fetch, message or remote route. | JW-03, JW-13, JW-18 · [FX-0099-W5-LOCAL-CUE-DEDUPLICATION](../planning/feature-wave-5.md#acceptance-design-for-n-1406-to-n-1429) |
| N-1421 Live work ticker | Show an ordered bounded stream of current eligible work events from the existing local view. Use safe item labels and deduplicate a replay. Show a gap, pause or disconnection when supported by the current authorised view. A protected event outside that view cannot create a missing row, visible sequence gap or count. Do not invent progress or display private page, member, credential or payload data. | JW-02, JW-15, JW-18 · [FX-0099-W5-ORDERED-SAFE-WORK-TICKER](../planning/feature-wave-5.md#acceptance-design-for-n-1406-to-n-1429) |
| N-1422 Taskbar pulse ring | Off by default. Show only the admitted desktop profile and safe aggregate progress for the chosen scope. Clear or mark it on lock, site switch, unknown progress or disconnect. Provide equivalent in-app text where unavailable. No protected label or off-PC update. | JW-02, JW-03, JW-18 · [FX-0099-W5-CLEARED-STALE-TASKBAR](../planning/feature-wave-5.md#acceptance-design-for-n-1406-to-n-1429) |
| N-1423 Engine heartbeat line | Read 0112/0106 exact process-instance heartbeat, 0104 admitted sidecar lifecycle and 0019 scheduler state separately. A work lease cannot prove process liveness. Asleep and failed are distinct. Label unavailable, idle or stale separately. A long silent job or healthy database cannot prove both processes live. This indicator never restarts anything or changes readiness. | JW-02, JW-03, JW-18 · [FX-0099-W5-EXACT-INSTANCE-HEARTBEAT](../planning/feature-wave-5.md#acceptance-design-for-n-1406-to-n-1429) |
| N-1424 Checklist progress bars | Render named milestones from the exact job profile with observed completed and current steps, denominator and any unknown estimates. Fix the milestone denominator to the exact graph. Repeats and retries do not advance it. A complete result needs a success receipt. Hold a guessed percentage and preserve accessible text. The display does not advance a checkpoint. | JW-02, JW-03, JW-17 · [FX-0099-W5-OBSERVED-MILESTONE-PROGRESS](../planning/feature-wave-5.md#acceptance-design-for-n-1406-to-n-1429) |
| N-1425 Live window-title counter | Put the current admitted safe progress projection in the local window title. Bind window, site and active job context and throttle changes. On unknown, stale or changed scope, clear or use safe status text. Never include sensitive job labels or claim completion from a hidden run. | JW-02, JW-15, JW-18 · [FX-0099-W5-SAFE-WINDOW-TITLE](../planning/feature-wave-5.md#acceptance-design-for-n-1406-to-n-1429) |
| N-1426 One-sentence system headline | Summarise current authorised jobs, chief supported wait reason and available machine state in one clear line. Treat disconnected or missing sensor evidence as unknown. Use the same snapshot as the overview. A count or description cannot reveal another site's work. | JW-01, JW-02, JW-18 · [FX-0099-W5-SCOPED-SYSTEM-HEADLINE](../planning/feature-wave-5.md#acceptance-design-for-n-1406-to-n-1429) |
| N-1427 Phone ping for big finishes | Phone push remains unavailable pending QR-027. Use exact admitted major-job and failure predicates and existing safe alert identity. Apply current route, scope, quiet hours, digest and snooze and reconcile uncertain delivery under the owning contract. Slack, Teams or email may deliver only under their separate 0099 and 0100 route, grants, rule and current approval. This form does not create a device route, new recipient, send or other approval. | JW-13, JW-19; QR-027 · [FX-0099-W5-PHONE-PUSH-UNAVAILABLE](../planning/feature-wave-5.md#acceptance-design-for-n-1406-to-n-1429) |
| N-1428 Conveyor-belt queue view | Render the authorised recorded queue, lane, wait reason and fitting-work state as a bounded accessible view. Preserve the scheduler's actual ordering and explain pauses or indeterminate next work. Movement is presentation only. No new priority, reservation or start. | JW-01, JW-03, JW-10 · [FX-0099-W5-READ-ONLY-QUEUE-VIEW](../planning/feature-wave-5.md#acceptance-design-for-n-1406-to-n-1429) |
| N-1429 Smart-scroll live feed | Follow bounded eligible events until the reader scrolls or selects a row. Keep the anchor and keyboard focus while buffered updates arrive. Resume only by a clear user action. Mark any missed or evicted events and keep scope checks on resume. No replay effect or unwanted movement. | JW-03, JW-15, JW-18 · [FX-0099-W5-SCOPED-SCROLL-RESUME](../planning/feature-wave-5.md#acceptance-design-for-n-1406-to-n-1429) |

N-1408 uses the source label Universal command bar. N-1419 uses
Mission-control wall mode. Use the source labels shown above.

#### Outcome, trust and job controls (N-1430 to N-1449)

Use the existing site-scoped Jobs screen, typed action registry and
0019 scheduler. Pin visible state to its exact workflow, run, graph,
measurement and grant generations. A view, saved filter or presentation
choice cannot grant a new action. Apply 0032 and 0118 to scrub, retain,
export and restore only the eligible projection. At a confirmed action
recheck the underlying grant, affected objects, current state and
expected revision, then use its original effect and recovery contract.

| Feature | Exact view or action | Owning requirement or contract |
|---|---|---|
| N-1430 Outcome-first cards | Read the committed eligible outcome and resource receipt of the exact run. Show the measured completed work and findings with scope and coverage. Distinguish successful, partial, uncertain and failed work. An old receipt or expected count cannot supply a completion claim. | JW-02, JW-15 · [FX-0099-W5-COMMITTED-OUTCOME-AND-COVERAGE](../planning/feature-wave-5.md#acceptance-design-for-n-1430-to-n-1453) |
| N-1431 Live cost and API meter | Show the owning route's measured charges, estimates, reservations and unresolved capacity separately by admitted unit, exact connector account/product/route, timestamp, policy revision, window and stable effect or receipt ID. Keep account identity masked to the viewer. Reconcile corrections and charge unresolved reservations against the original cap across reset or restore. Forecasts start no price fetch. Check current cap and approval at dispatch. Keep tokens, request counts and currency units distinct. The existing 0099 reservation and owning connector cap control dispatch. At insufficient or unproved capacity, hold and report the safe reason. Opening the meter cannot spend, reserve, widen a cap or grant provider access. | Workflow execution and connector limits, JW-15, JW-16, JW-17 · [FX-0099-W5-COST-METER-ORIGINAL-RESERVATION](../planning/feature-wave-5.md#acceptance-design-for-n-1430-to-n-1453) |
| N-1432 Human action trail | Read the eligible 0032 audit projection for each exact action, actor class, device scope where permitted, outcome and related run or setting revision. Distinguish a deliberate owner action from an automatic transition. Keep identifiers beyond the viewer's grant, secret fields and raw job data out. The view cannot edit audit history or approve another action. | Helpers, JW-15 and 0032 audit · [FX-0099-W5-HUMAN-OR-AUTOMATED-AUDIT-PROJECTION](../planning/feature-wave-5.md#acceptance-design-for-n-1430-to-n-1453) |
| N-1433 Read-only client progress link | Apply DR-028. Offer one frozen, recipient-approved local snapshot of the eligible run version under 0032, 0100 and 0118. Bind exact final bytes, recipient, site, purpose, observation time and expiry. Recheck rights immediately before hand-off and after replay. Refuse remote assets and URLs, file and action links, callbacks, active HTML and copied controls. Shared copies are outside managed recall. Scrub internal addresses, controls, hidden nodes and secrets from the final file and bound its lifetime while managed. Unknown ownership or changed rights holds release. No live link, remote bridge, new client login or dispatch. | JW-12, JW-15 · [FX-0099-W5-CLIENT-FROZEN-LOCAL-SNAPSHOT](../planning/feature-wave-5.md#acceptance-design-for-n-1430-to-n-1453) |
| N-1434 Deadline countdown badge | Read the approved schedule, target instant, site time zone and current ledger of progress. Display the deadline separately from the estimated finish range, sample count and confidence limits. Show amber with text only on admitted drift evidence. Unknown or stale estimates cannot show the run on track. Viewing the badge changes no schedule or priority. | JW-03, JW-14, JW-17 · [FX-0099-W5-DEADLINE-AND-PROVED-DRIFT](../planning/feature-wave-5.md#acceptance-design-for-n-1430-to-n-1453) |
| N-1435 Pause impact preview | Preview the current eligible dependent reports and schedules, what will wait or become stale, and the difference between pause and cancellation under 0019. Protect never-wait work and name known unknowns. At confirmation recheck the same affected scope and state. Neither preview nor cancellation undoes an effect already committed. | JW-06, JW-10, JW-20 · [FX-0099-W5-PAUSE-PREVIEW-PROTECTED-AND-COMMITTED-WORK](../planning/feature-wave-5.md#acceptance-design-for-n-1430-to-n-1453) |
| N-1436 One-click project lenses | Save versioned local view filters within the eligible fields of the active site. Recheck the current grants before listing or counting runs through a lens. An owner selecting another registered site uses the existing desktop site switch under 0102 and a separate site query. A restored lens gives no new scope, joint query or hidden counts. | Screens and 0102, JW-15, JW-16 · [FX-0099-W5-PROJECT-LENS-CURRENT-SITE](../planning/feature-wave-5.md#acceptance-design-for-n-1430-to-n-1453) |
| N-1437 Human thumbs-up gates | Insert an approved typed gate into a local draft graph and bind plan, exact graph/node/loop identity, protected effect and input digests, actor class, site, destination, grant and expiry. The run releases its step lease and holds no memory while waiting. Consume one current 0020 confirmation fenced with eligible dispatch through the owning action. Reconcile lost outcomes with the same receipt. Edits and restoration cannot transfer approval. Decline, expiry, pause, changed scope or a tool confirmation releases no effect. A gate cannot create a new permission or send route. | JW-07, JW-16, JW-19, JW-20 · [FX-0099-W5-HUMAN-GATE-EXACT-RECEIPT](../planning/feature-wave-5.md#acceptance-design-for-n-1430-to-n-1453) |
| N-1438 One-page monthly health report | Bind the closed site-calendar month and eligible run-history revisions. Show measured reliability, duration and completed work with scope and coverage. Missing history stays visible. Qualify the exact local page, accessibility and file output under 0100 and 0118. Hold a version that clips required information. No scheduled mailing or public report. | Reports, JW-15, JW-17, JW-22 · [FX-0099-W5-MONTHLY-REPORT-EXACT-LOCAL-PAGE](../planning/feature-wave-5.md#acceptance-design-for-n-1430-to-n-1453) |
| N-1439 Safety-latch permissions | Separate the visible view and eligible Run now action from advanced settings. Opening the Engine Room never adds a permission. Current actor and action grants, owner-only live confirmation and protected memory, disk and secret rules apply behind every entry point. Unsupported staff or assistant operations stay unavailable. | JW-04, JW-16, JW-20 · [FX-0099-W5-SAFETY-LATCH-VIEW-AND-AUTHORITY](../planning/feature-wave-5.md#acceptance-design-for-n-1430-to-n-1453) |
| N-1440 Sticky notes on the graph | Pin a bounded inert note to the exact graph revision and node, with author and eligible scope. Treat note contents as data. Show an orphaned anchor only within current rights. A note gives no action, schedule condition, approval, external preview or tool instruction. | JW-03, JW-06, JW-15, JW-16 · [FX-0099-W5-GRAPH-NOTE-INERT-SCOPED-ANCHOR](../planning/feature-wave-5.md#acceptance-design-for-n-1430-to-n-1453) |
| N-1441 Calm vs. Engine Room toggle | Switch between the approved Jobs presentation levels with the same visible scope and safe status. Keep dangerous or protected controls under the current independent grants. A switch or saved preference changes no flag, timeout, worker limit, action grant or product settings, and never resumes a held job. | Screens and ADR-0021, JW-03, JW-16 · [FX-0099-W5-PRESENTATION-PREFERENCE-ONLY](../planning/feature-wave-5.md#acceptance-design-for-n-1430-to-n-1453) |
| N-1443 Portable workflow cards | Use the existing closed 0100 local file profile. Bind the graph version and safe symbolic references. Refuse live authority, credentials, site data, system paths, remote addresses and executable content. Rebind every reference through the current local catalogue and grant. A Python reference requires separate 0104 admission. Imported cards become unactivated local drafts. | Workflow authoring, JW-06, JW-15, JW-16 · [FX-0099-W5-INERT-PORTABLE-WORKFLOW](../planning/feature-wave-5.md#acceptance-design-for-n-1430-to-n-1453) |
| N-1447 Checkbox bulk actions | Use one bounded selection preview with per-item site, task, current action eligibility and expected revision. Map pause, retry or clear to their existing typed actions. Never restart a completed or unknown effect and never erase required audit or retention records. Show exact per-item results or safe holds, preserving original effect receipts and refusing unauthorised or stale items. | JW-08, JW-09, JW-15, JW-16, JW-20, JW-22 · [FX-0099-W5-BULK-ITEM-EFFECT-FENCES](../planning/feature-wave-5.md#acceptance-design-for-n-1430-to-n-1453) |
| N-1449 Single-key speed shortcuts | Offer discoverable keyboard shortcuts in the eligible Jobs view. Ignore shortcuts in editable text and other protected focus contexts. Send any Run now or Pause request through the identical visible action and required confirmation. Search remains a scoped view. Repeated keys share the original action identity where applicable. | JW-03, JW-09, JW-16, JW-20 · [FX-0099-W5-FOCUS-BOUND-SHORTCUT](../planning/feature-wave-5.md#acceptance-design-for-n-1430-to-n-1453) |

QR-201 qualifies exact run fields, comparisons, scope, projections,
file profiles and refusal cases. Preserve the existing spend,
retention, scheduler and confirmation limits.

For an eligible weekly ranking run, project its committed
KEEP_CHAMPION only as the historical decision of that run. Use
0077 for the outcome and show release assessment separately under
0057. After restart recheck the run's current read scope and each
source's disclosure rules. A retained Jobs view cannot supply
assessment detail or its read permission. See 0057
BDD-0057-KEEP-NOT-RELEASE for the restricted-history case.

### Alert centre contracts (602–607)

Project 0032's canonical drishti alert, grouping, resolution and
replay receipts. One site-bound source event has registered type,
stable ID, monotone generation, time and reason. Map its approved
severity to information, warning or critical by typed producer
profile. No title from raw data. Equal event ID and generation
with different digest refuses. A corrected or withdrawn record
fences older notices. Display text, icon and colour. A critical
label changes no grant. Spec 0137 alone validates official notice
dates and stages. Provisional or revoked updates cannot alert.

Show eligible alerts in one site-scoped list with filters, source
time and availability. Bound each page to 100 rows. A cursor pins
site, actor and session, scope and grant generation, source high-water,
filter and sort. Check current access at detail and each page.
Unknown or withdrawn scope reveals no title, type, reason or count.
Use the same safe projection in desktop and admitted local commands.
Revoke open pages and queued effects at scope loss.
The alert centre reads no remote store and exposes no raw
exception, secret, draft, member name or private address.

A routing revision binds alert type, site, severity, exact
destination or local view, approval, template and expiry.
The default is local. Route changes use the existing 0020
settings rules. External delivery needs the separately admitted
0100/0111 action and current destination and content approval.
Severity, template, copy or retry cannot add a destination.
Native XenForo staff alerts use the separately approved 0032
route and exact grant. It creates no WordPress or external send
grant. Connector failures stay local and cannot route through
the failing effect. Protected sources cannot become outward counts.

Quiet intervals are half-open in the owner's approved time zone
and zone database version. Resolve each boundary under 0019,
union overlapping and overnight intervals, and stop dispatch
when the clock or scope cannot be proved. A local eligible view
remains visible. On catch-up recheck current state, rule, clock,
disclosure and expiry. Expired items stay visible as history only.
Receipt and generation prevent repeats after restart or clock
rollback. Quiet hours delay messages and never stop safety work.

Digest none, hourly or daily uses the versioned 0019 fire key.
One logical digest effect binds site, route lineage and bounded
window. Freeze projection, destination and alert IDs at claim.
Compare the pending generation and commit one outbox effect.
Edits can replace unclaimed scopes and cannot reopen an occurrence
held or delivered. Do not redigest released events after cadence
changes. Share disclosure history across source types, filters,
recipients, files, prior releases and frequencies. Headline
counts obey source restrictions and suppress revealing groups.
Atomic local files and separate 0100/0111 recipient receipts
govern delivery. An unknown outcome stays held after restart.
It cannot resend or reroute.

Source state stays with 0032. Acknowledgement marks one viewer's
exact alert revision seen and never changes pending delivery.
A scoped snooze holds only its approved personal or route
notifications. Staff cannot hold another recipient's notifications.
Bind site, actor, scope, current source state and expected view
revision. Commit the conditional change, audit and outbox once.
Snooze is an exact future UTC time under the current cap. After a
lowered cap, stop holding at the earlier applicable deadline.
Expiry is durable and never reopens on clock rollback or restore.
New source generation needs its own state. Revalidate at dispatch.
Portfolio counts keep their approved owner-only definition.

#### Wave 2 alert-centre fixture bindings

Use synthetic events, current grants, trusted clocks and stand-ins
for previously admitted delivery. QR-151 still owns source, clock,
disclosure and delivery evidence. Source status stays with 0032.
No fixture configures or calls a destination.

| Feature | Owning requirement and design | Planned fixture and expected evidence |
|---|---|---|
| F-0602 | JW-13 and JW-15 · DES-0099-W2-GRANTED-ALERT-VIEW | FX-0099-W2-GRANTED-ALERT-VIEW · BD-JW-05 and UT-JW-10. Give an eligible source, duplicate event, same-key same-generation conflicting digest, foreign site, revoked cursor and provisional 0137 notice. Only the admitted current 0032 projection appears. Recheck site, actor, session and grant on each page and detail. Refusal reveals no hidden count, reason or title. |
| F-0603 | JW-03 and JW-13 · DES-0099-W2-TYPED-ALERT-SEVERITY | FX-0099-W2-TYPED-ALERT-SEVERITY · AC-JW-01 and BD-JW-05. Show information, warning and critical from registered producer profiles in the keyboard view and high contrast. Use text and icon as well as colour. Hold an unknown label or source mapping. Severity grants no broader destination, action or disclosure. |
| F-0604 | JW-13, JW-16 and JW-19 · DES-0099-W2-CURRENT-ALERT-ROUTE | FX-0099-W2-CURRENT-ALERT-ROUTE · BD-JW-05, UT-JW-11 and IT-SD-01. Compare a stand-in eligible route with a changed recipient, content level and grant, a connector fault and an unadmitted native staff route. Recheck identical current scope at dispatch. Hold outside effects while keeping eligible local notices. Only 0100/0111 owns approved connector delivery and only the separately admitted 0032 XenForo staff path can send natively. |
| F-0605 | JW-13 and JW-24 · DES-0099-W2-EXACT-QUIET-INTERVAL | FX-0099-W2-EXACT-QUIET-INTERVAL · BD-JW-05 and UT-JW-15. Simulate overlapping half-open quiet intervals across repeated local times, expiry, clock rollback, pause and current grant withdrawal. Use the pinned zone and 0019 occurrence and clock rules. Keep the local view and safety work. Release an eligible delayed effect only once after revalidation. An unproved clock or expired or revoked scope holds it. |
| F-0606 | JW-09, JW-13 and JW-19 · DES-0099-W2-SCOPED-DIGEST-SLOT | FX-0099-W2-SCOPED-DIGEST-SLOT · UT-JW-06, BD-JW-05 and IT-SD-01. Compare none, hourly and daily using exact 0019 fire keys, a missed or replaced occurrence, cadence change and suppressed groups. Freeze the eligible projection at claim. Simulate separate accepted, rejected and unknown SMTP recipient outcomes through the owning route. Preserve one authorised effect per scope, and hold unknown without duplicate or widened disclosure. |
| F-0607 | JW-13 and JW-16 · DES-0099-W2-EXACT-ALERT-VIEWER-STATE | FX-0099-W2-EXACT-ALERT-VIEWER-STATE · BD-JW-05 and UT-JW-11. Race two viewers and check acknowledgement of one exact revision, eligible scoped snooze, cap reduction, lost role, new source generation and stale restore. Acknowledgement changes seen state only. Snooze holds only its authorised effect until the earlier applicable deadline. Commit one conditional view-state and audit change. No source resolution, repeat or held effect revival. |

### Controls on the Overview

- **Pause and quiet mode:** one button pauses or slows background work for a
  chosen time, for example during a video call. The Python sidecar sleeps when
  no job needs it and shows "Waking up" when it returns (spec 0104).
- **Cut to the front:** moves one job to the head of the queue and pauses
  lower jobs without losing progress. It never overrides the never-wait list
  of spec 0078.
- **Pause impact preview:** before pausing, the screen says what will be late.
- **Memory slider and mode dial:** the user's pick of spec 0078 (Safe,
  Balanced, Custom); this screen shows it and changes nothing else.
- **Battery and heat:** heavy jobs pause on low battery and ease off when the
  machine is hot, each with a plain note.
- **Bulk actions:** checkbox selection and single-key shortcuts for pause,
  resume and retry.
- **Command bar:** a typed sentence (spoken waits for QR-027) such as "Pause
  all crawls for 2 hours" becomes an action of spec 0020, shown first as a
  preview the user confirms. It never reaches past the permissions of the
  user.

### Workflows

1. **Triggers:** a schedule, an incoming signed message from a program on this
   PC (it arrives only through the local socket or named pipe of ADR-0021,
   never a network port; messages from the internet wait for QR-030), a file
   arriving in a folder, a site change, an alert, a button, or another
   workflow. A schedule uses plain words ("every weekday at 7", "right after
   an article is published"), never cron codes.
2. **Actions:** every action of spec 0020 the user may use, plus reports,
   exports and messages to connected services that the owner has switched on.
3. **Logic:** branching, loops, data mapping, workflow variables, data tables
   and input forms that reject a wrong entry before the run starts.
4. **Script and AI steps:** a script step runs a Python script in the
   fenced sidecar (spec 0104). An AI step calls a model through the Model Hub
   (spec 0080). Both may only create proposals.
5. **Human approval:** a thumbs-up step stops the run until a person decides.
   A run waiting for approval holds no memory and expires after a chosen time.
6. **Errors:** a retry policy per step, a fallback step, an error workflow
   that runs on failure, and a "twin blocker" that stops two copies of the
   same run from running at once.
7. **Sub-workflows and templates:** a workflow can call another. Templates and
   one-click recipes are files, shared as portable workflow cards.
8. **Versions:** every save is a version; any version can be restored; a run
   records the version it used.
9. **Test runs:** run on sample data, on one real item for five seconds
   ("single-page test drive"), or simulate a failure of any step. A sandbox
   clone runs a copy of a workflow with no side effects.
10. **Credentials and quotas:** secrets live in the vault of spec 0065 and are
    masked everywhere. Each connected service has a quota shown on its block.

#### Workflow authoring and execution (458–477)

Store an immutable graph revision with stable node and edge IDs,
typed input and output ports, trigger, owner, site, scope, action
catalogue versions and hash. Show the same graph in the canvas
and keyboard list. Reject missing nodes, incompatible ports,
ungranted actions, dynamic action names and implicit cycles before
approval. Allow bounded explicit loops only, up to 100 items per
loop, 128 nodes per workflow, three sub-workflow levels and 10,000
step invocations per run. Compute and check worst-case expansion
before activation, including children, retries and error handlers.
The stricter admitted job limit wins. Persist branch decisions,
ordinals and checkpoints in visit order. Runtime stops leave
earlier committed effects visible with a partial outcome. A
cancellation is not rollback. A saved draft has no effects.

Preserve the admitted local triggers: schedule, button, file
arrival from a scoped folder, signed local message, site change,
alert and sub-workflow. Bind ingress identity, event ID, origin,
site and exact grant. A folder capture needs a current folder
grant and a file resolved inside it without link traversal.
Use separately authorised service identity for recurring work.
Intersect current caller, workflow, child and action grants.
A trigger never inherits the creator's permissions. Reject
unproved origin and outside events relayed by local wrappers.
Feature 462 stays unavailable until QR-030 and the relevant ADR
admit the exact remote route. Current ingress stays local only.

The catalogue lists only workflow-admitted 0020 operations:
authorised local reads, bounded workflow state, proposals and
separately approved 0100 sends under JW-04. Display typed inputs,
scope, approval and effects. Registry, credential, site mutation,
spend and protected setting actions outside this list remain
standalone. An approval cannot add to the workflow catalogue.

A schedule preview gives the native zone, zone database version,
next UTC slots and the same clock and missed-run rules as 0019.
Use the 0019 fire key (site, schedule ID and version, canonical
planned occurrence), mapped to one logical workflow occurrence.
A schedule edit fences pending claims and replaces unclaimed
slots atomically. Commit accepted inbox, run and outbox together.
Started runs keep their graph revision and recheck current rights.
Pause or withdrawal blocks dispatch and publish. An acknowledged
trigger cannot create a second logical run.

Each step uses typed immutable inputs and 0106's settings
generation. Flow variables have declared type, bounds, provenance
and lifetime (run or scoped protected table). Persist classification,
site grant, permitted consumer and source versions for all values,
including child outputs. Reject untyped fields and every unauthorised
map. Recheck current readability, durable authority and erasure
on use and replay. Release only the approved typed projection. Never
interpret data as an instruction, script, template or permission.
Python steps use the 0104 fence. Local AI uses the 0080 Model Hub
and 0078 leases. Both return typed proposals through validators.
Neither route can confirm, spend, execute an effect or pass secrets.

Before a protected step show the exact intended effect and request
the actor class required by 0020. Hold no lease while waiting.
Bind plan ID, approval class, exact site, graph and step,
effect and data hashes, destination, actor, scope and expiry.
Recheck and consume once at the protected effect. Test, dry-run,
scripts and MCP cannot confirm. Approved sends still need 0100
route authority and current exact confirmation if required.
Owner-only confirmation uses the live owner route. A hook, send
approval or older grant cannot authorise writes or spending.

Use stable step effect IDs qualified by run, node and loop path.
Keep input digest, attempts, failure reason, quota reservation,
receipt and outcome in protected history. Mark durable intent
before dispatch. Retry only a proved safe transient under current route, cap and
grant. On lost reply reconcile the original exact receipt. Unknown
holds repeats, equivalent fallback and external failure actions.
Show a safe alert under 0099. Recheck source, settings, grants
and erasure when reading history. History inspection and synthetic
replay have no effects. Recovery follows the owning route's
CAS and idempotency under the original ID. A genuinely new
business action starts separately and needs current authority.
Neither replay nor new approval can repeat a completed or unknown
effect. Every permitted fallback has independent route approval.

Use every applicable owning-connector limit, including provider
account across sites and tokens, route, site and workflow. Key
reservations by non-secret account, unit, window and effect ID.
Commit reservation with dispatch intent. Reconcile against the
typed receipt. Keep unknown capacity committed under the
connector's policy across restart, reset, graph edit or rotation.
Pause, refusal or revoke fences new dispatch. Only the approved
Rust action resolves a Tijori handle bound to site, account,
action, destination, scope and grant generation. Revalidate at
resolve and dispatch. No raw credentials or handle IDs in editor,
samples, exports, portable references, Python or model data.

Offer original local starter graphs for a freshness report and
review reminder. Preview and clone to a local draft. Portable
cards use a bounded closed 0100 schema and local file picker.
Carry graph and symbolic safe references only. Reject executable
content, credential refs, URLs to resolve, system paths, live
grants and site data. A hash protects integrity only. Map each
reference to a current local catalogue and grant. A Python ref
needs separate 0104 admission. Restore creates a new revision
requiring current authority. No saved record gives activation.

Synthetic test mode reads synthetic fixtures only. Dry-run uses
local input by default with the five-second cap for a real-item
preview. A labelled remote read needs a separately admitted free,
non-mutating route with current grant, approved egress and brokered
credential only. Test or dry-run writes no durable workflow value,
product file, quota reservation, approval token or outside effect.
Keep ephemeral diagnostics and existing scrubbed audit rules.
No package install or remote Model Hub fallback. Inject failures,
quotas and worker expiry against offline fixtures.

##### Named workflow authoring, ingress and approval fixtures

These planned spec 0076 cases use synthetic actors, graphs,
events, clocks and receipts. QR-142 retains admission. QR-081
holds scheduler proof and QR-030 still holds the remote route.

- `DES-0099-W2-GRAPH-AUTHORING` uses `VAR-0099-W2-TYPED-GRAPH`,
  `VAR-0099-W2-TYPED-ACTION`, `VAR-0099-W2-STARTER-CLONE` and
  `VAR-0099-W2-TYPED-BOUNDED-LOOP`. Inputs and actions come from
  the existing closed 0020 catalogue. A draft or mapped value
  grants no execution or permission.
- `DES-0099-W2-INGRESS-SCHEDULE` uses `VAR-0099-W2-LOCAL-TRIGGER`
  and `VAR-0099-W2-NATIVE-OCCURRENCE`. Use the signed local
  source or 0019 occurrence and intersect current grants.
  `DES-0099-W2-REMOTE-REFUSAL` binds only the F-0462 denial.
- `DES-0099-W2-LOCAL-RECOVERY` uses `VAR-0099-W2-PROTECTED-HISTORY`
  and `VAR-0099-W2-ACCOUNTED-RETRY`. Inspect or reconcile the
  same exact receipt. History and synthetic replay have no effects.
- `DES-0099-W2-LOCAL-PROCESSOR` uses `VAR-0099-W2-FENCED-PYTHON`
  and `VAR-0099-W2-MODEL-HUB-PROPOSAL` within the existing
  0104, 0080 and 0078 fences. Both yield typed proposals only.
  No arbitrary language, package or provider action is admitted.
- `DES-0099-W2-EFFECT-APPROVAL` uses
  `VAR-0099-W2-EXACT-HUMAN-CONFIRM` under the existing 0020
  actor and action authority. Approving cannot expand the
  catalogue. Any existing 0100 send retains its separate route,
  approval and current grant.

| Feature | Requirement | Design and variant | Fixture and test | Named fixture assertion |
|---|---|---|---|---|
| F-0458 | JW-03, JW-06 | DES-0099-W2-GRAPH-AUTHORING, VAR-0099-W2-TYPED-GRAPH | FX-0099-W2-GRAPH, AC-JW-01, UT-JW-04 | Build the same typed graph by keyboard list and canvas. Compare stable nodes and edges after save. A missing node, implicit cycle, invalid port or ungranted action cannot activate. |
| F-0459 | JW-11, JW-25 | DES-0099-W2-INGRESS-SCHEDULE, VAR-0099-W2-LOCAL-TRIGGER | FX-0099-W2-TRIGGER, UT-JW-07, IT-JW-03 | Replay one eligible signed local event and keep one logical run. Deny wrong-site or stale signatures, an escaped folder reference and an outside event wrapped in a local message. |
| F-0460 | JW-04, JW-16 | DES-0099-W2-GRAPH-AUTHORING, VAR-0099-W2-TYPED-ACTION | FX-0099-W2-ACTION-CATALOGUE, UT-JW-02, UT-JW-11 | Show registered action types and their required grants. Deny an invalid mapped field, a disabled route and an unlisted write. Browsing the catalogue creates no approval, provider access or effect. |
| F-0461 | JW-09, JW-14, JW-24 | DES-0099-W2-INGRESS-SCHEDULE, VAR-0099-W2-NATIVE-OCCURRENCE | FX-0099-W2-SCHEDULE, UT-JW-06, UT-JW-09, UT-JW-15 | Resolve a repeated hour and missed slot under 0019. Race a prior claim with an edited schedule and then pause before dispatch. Only a current accepted occurrence creates one logical run. |
| F-0462 | JW-25 | DES-0099-W2-REMOTE-REFUSAL | FX-0099-W2-REMOTE-REFUSAL, IT-JW-03 | Try a public webhook, a cloud relay and inbound network event. Refuse each without creating a run or listener. The separate eligible local control belongs to F-0459 and creates no remote authority. |
| F-0463 | JW-06, JW-16 | DES-0099-W2-GRAPH-AUTHORING, VAR-0099-W2-STARTER-CLONE | FX-0099-W2-STARTER-TEMPLATE, UT-JW-04, UT-JW-11 | Clone an approved local starter into an inert draft. Use unknown typed input and a removed grant. Reject the inputs or activation, and copy no credential, authority or live schedule. |
| F-0464 | JW-09, JW-22 | DES-0099-W2-LOCAL-RECOVERY, VAR-0099-W2-PROTECTED-HISTORY | FX-0099-W2-HISTORY-REPLAY, UT-JW-06, UT-JW-14 | Inspect eligible history and synthetic replay across a pre-dispatch and post-dispatch crash. An unknown or completed effect never repeats. Expired evidence is withheld and old approval cannot authorise a new effect. For a restored historical 0070 journal without a current graph or run, use 0020 IT-0020-HISTORICAL-CW-RESTORE. |
| F-0465 | JW-08, JW-13 | DES-0099-W2-LOCAL-RECOVERY, VAR-0099-W2-ACCOUNTED-RETRY | FX-0099-W2-SAFE-RETRY, BD-JW-04, BD-JW-05 | Retry only a proved transient in route bounds and with current grant. An unknown synthetic send holds retry and equivalent fallback. Recheck quiet hours and produce only the eligible local alert. |
| F-0466 | JW-06, JW-16 | DES-0099-W2-GRAPH-AUTHORING, VAR-0099-W2-TYPED-BOUNDED-LOOP | FX-0099-W2-BOUNDED-LOGIC, UT-JW-04, UT-JW-11 | Try a typed branch and explicit loop at the admitted expansion limit and one over. Include child, retry and error branches. Refuse overflow, stale input or recursion. Retain earlier committed effects on a later stop. |
| F-0467 | JW-04, JW-15 | DES-0099-W2-LOCAL-PROCESSOR, VAR-0099-W2-FENCED-PYTHON | FX-0099-W2-PYTHON-PROPOSAL, UT-JW-02, UT-JW-10, UT-PY-02, UT-PY-03 | Use the admitted fenced Python route on synthetic allowed data. A secret, unapproved purpose or other language gets no execution. Valid output must pass the typed proposal validator without effects. |
| F-0468 | JW-04, JW-15 | DES-0099-W2-LOCAL-PROCESSOR, VAR-0099-W2-MODEL-HUB-PROPOSAL | FX-0099-W2-AI-PROPOSAL, UT-JW-02, UT-JW-10 | Treat untrusted text as data under the exact approved local model and lease. Revoked evidence or an unavailable lease yields no proposal. An accepted result is typed and creates no effect or outside fallback. |
| F-0469 | JW-07, JW-16 | DES-0099-W2-EFFECT-APPROVAL, VAR-0099-W2-EXACT-HUMAN-CONFIRM | FX-0099-W2-HUMAN-APPROVAL, UT-JW-05, UT-JW-11 | Wait with no worker lease. Change the input digest and try an expired or MCP-supplied confirmation. Only the required current human actor can approve the unchanged exact effect for one consumption. |

#### Workflow table, form and child contracts (676–680)

Each graph revision pins the exact table, form, error-handler and
child revisions it can use. Bind site, resource or run identity,
actor, action, source generation, scope and lifetime to each read
or mutation. Recheck grants and 0032 erasure at use and release.
Apply the active 0078 budget and 0108 site boundary even during
restore. Samples and dry runs use a private scratch projection
with no durable table or workflow effect.

- **676, workflow data tables.** A table definition has a stable
  ID, immutable revision, owner, grant, column list and purpose.
  Each column has an ID, kind, optional flag and bound. A data
  generation is separate from the schema revision. Pin both and
  the exact row manifest at read, resume and replay. Initially
  allow bounded plain text, boolean, 64-bit signed integer,
  versioned enumeration, UTC timestamp and typed same-site
  reference. Keep row ID, previous row generation, source revision
  and erasure scope separate from user values. Qualification
  must test 32 fields per definition, 2,000 live rows and 1 MiB
  canonical value bytes per table as proposed bounds. Share a
  separate retention and total byte cap across historical and live
  generations. No new write if the admitted cap cannot be met.
  Apply the stricter existing lease, time and storage limits.
  Owner-edited rows use a typed screen and expected revisions.
  Runs may read the pinned validated data and use bounded
  run-local rows. A persistent change is a local proposal
  for separate human review. Bind exact table, schema, data
  generation, rows and changes to the reviewed digest. Commit
  once with compare-and-swap, audit and receipt. A mismatch holds
  for review. No raw SQL, dynamic table names, secrets, hidden
  conversions or cross-site joins. Type changes require review
  of dependent graphs. Old revisions observe current rights,
  expiry and erasure. After removal keep only safe receipts.
- **677, workflow input forms.** Bind stable form and field IDs
  to exact input ports for the same graph revision. The form is
  protected local app UI. Each field carries plain label, help,
  kind, size or value range, optional flag and explicit default. Treat missing,
  empty and invalid differently. Form input never contains raw
  credentials, executable types or permissions. Bound parse
  size and reject duplicate or unknown keys, stale versions and
  invalid inputs before reserving a run. Error and replay views
  show safe field labels and reason, never raw rejected values.
  Focus the error summary and show one actionable error per
  field. The server issues an ingress ID tied to site, actor,
  session, form and graph revision and expiry. Check the typed
  canonical input digest and current grants. Commit input,
  inbox and run once under 0019. A retry returns the existing
  run only if the identity and digest match and authority is
  current. Recheck expiry before dispatch. Required fields have
  no implicit defaults. Local automation uses its own typed
  trigger mapping and never forwards a public form or inherits
  the owner's defaults or grant.
- **678, workflow diagrams.** Render the exact saved or working
  revision from stable node and edge IDs. Show start, normal,
  condition, bounded loop, child, error, and end paths with plain
  labels. An accessible ordered list exposes the same visible
  steps, branch order, decisions and warnings. Require keyboard
  navigation, visible focus, textual errors and a held state for
  stale detail. Graph edits and publication are distinct. The
  diagram loads no step values, secret handles or remote assets.
  Apply one authorised projection and generation to canvas,
  list and export. Escape labels and keep destinations inert.
  Omit hidden labels, nodes and identifying edges. Show a safe
  scope limit. Views cannot activate a workflow. Unsaved or
  unapproved revisions stay local. Export only the approved
  frozen digest through the existing protected snapshot after
  current rights and erasure checks.
- **679, error workflows.** Select a named exact handler
  revision and approval at graph review. Atomically commit one
  stable handler intent with the terminal failure, keyed by
  source run, graph revision and step path. Claim it under the
  current site, actor or explicitly permitted service identity
  and intersected grants. Recheck at claim and use after restart.
  Refusal, pause, cancellation and uncertain original effects
  cannot start a handler. Show only the safe hold or local alert
  for uncertainty. Pass a typed fault code, safe reason, time
  and trace reference. All descendants inherit the local-only
  diagnostics and proposals mode. It forbids raw inputs, secrets,
  original-effect retries, native writes, sends, spending and
  generic workflow triggers. A handler failure stops and emits
  at most one local alert that cannot retrigger its own chain.
  A failing step has one terminal occurrence ID across the
  child and parent path. Assign the nearest admitted enclosing
  handler once and carry its receipt on propagation. Ancestors
  can stop or continue only along a declared branch and never
  run a second handler for the occurrence. Unknown or expired
  receipts hold dependent work and preserve committed receipts.
  No replay can manufacture a new terminal occurrence.
- **680, sub-workflows.** Bind the child graph and exact version
  to typed mappings and pin the full transitive call graph and
  its approval generations. Root is level zero and the maximum
  child level is three. Validate 128 nodes per graph, 100 items
  per loop and no more than 10,000 expanded invocations across
  the root. Compute worst case including retries and error
  paths before activation. Refuse cycles through call and error
  edges. Keep a shared durable root allowance across concurrent
  descendants and recovery. A stricter lease wins. At admit,
  dispatch, input read and release, intersect caller, all path
  graph and current action grants, source and erasure generations.
  Bind the child key to root run, full node path and the vector
  of nested-loop ordinals. Commit its digest, root allowance,
  admission and outbox together under the pause/cancel fence.
  Recover with the original key. A changed digest for a used key
  is refused. No duplicate charge against the allowance for the
  same admitted dispatch. New invocations debit the same shared
  allowance. Pause and cancel also fence steps within children.
  Completed effects retain original receipts. Release only the
  approved typed output for that invocation. Hold on changed
  approval or missing version. A newer release cannot substitute.

Map form/graph validation to SUT-900 or SUT-904, stale or
unavailable named versions to SUT-906, and denied scopes to
SUT-907. Describe what the user can review in plain English.
Use the existing 0099 durable execution and 0032 safe history.
Erasure and expiry also affect dependent table and form snapshots,
in-flight handlers and child projections.

#### Wave 2 typed workflow fixture bindings

Use synthetic graphs, tables, scoped grants and scheduler receipts.
QR-164 owns exact graph and execution proof. These fixtures operate
inside the existing local door and grant, storage and lease limits.

| Feature | Owning requirement and design | Planned fixture and expected evidence |
|---|---|---|
| F-0676 | JW-05, JW-06 and JW-16 · DES-0099-W2-PINNED-TYPED-TABLE | FX-0099-W2-PINNED-TYPED-TABLE · UT-JW-03, UT-JW-04 and UT-JW-11. Use empty and one-row tables and test the proposed 32-field, 2,000-live-row and 1 MiB canonical-value limits and one over each. Race incompatible owner updates, resume at the pinned schema and data generations and revoke an older version. Hold wrong-site, wrong-type, over-history-cap and stale reads. The run can prepare an inert persistent-edit proposal only. A sample writes nothing. |
| F-0677 | JW-09, JW-15 and JW-16 · DES-0099-W2-EXACT-FORM-INGRESS | FX-0099-W2-EXACT-FORM-INGRESS · UT-JW-06, UT-JW-10 and UT-JW-11. Test current, stale and expired local form revisions with missing, empty, invalid, duplicate and unknown inputs and planted secret or instructions. Reject before reserving a run. Give safe field errors without rejected values. Only the matching site, actor, session, form and graph scope with the same typed digest can replay its accepted 0019 inbox/run identity. No public form or inherited default or grant. |
| F-0678 | JW-03 and JW-15 · DES-0099-W2-ONE-VISIBLE-GRAPH-PROJECTION | FX-0099-W2-ONE-VISIBLE-GRAPH-PROJECTION · AC-JW-01 and UT-JW-10. Compare canvas and accessible list for one saved revision with loop, child and error edges, a restricted label and old revision. Check text, focus, selection, warnings and consistent safe omission in both views. Protected labels, step values and identifying edges stay hidden. Nothing loads an outside asset or activates the graph. |
| F-0679 | JW-04, JW-08, JW-09 and JW-16 · DES-0099-W2-ONE-LOCAL-TERMINAL-HANDLER | FX-0099-W2-ONE-LOCAL-TERMINAL-HANDLER · BD-JW-04, UT-JW-06 and UT-JW-11. Model exhausted retries, crash at terminal failure commit and child failure propagation using the original occurrence. Only the nearest admitted scoped handler can run once. Cancellation, denial, pause or uncertain original effect does not start one. Fail the handler and try external send through a descendant and alert re-entry. Preserve original receipts and at most one safe non-reentrant local alert. |
| F-0680 | JW-06, JW-09 and JW-16 · DES-0099-W2-PINNED-CHILD-INVOCATION | FX-0099-W2-PINNED-CHILD-INVOCATION · UT-JW-04, UT-JW-06 and UT-JW-11. Compare child depth three and four, cycles through error edges and worst-case expanded work including retries against the existing shared 10,000-invocation root cap. Race contenders for the final allowance, change a used key's digest and revoke while paused after a committed child. Hold a stale grant, missing pinned child or changed digest. Preserve original receipts and debit the allowance once per exact invocation. No newer child or wider right by implication. |

### Workflow inspection, testing and scheduling (owner job catalogue)

Use 0019's registered scheduler, graph revisions, leases, checkpoints
and receipts. Use 0020's exact local owner action for any change, with
the current site, actor, allowed action and effects. A view, bookmark,
test, receipt or schedule suggestion gives no additional authority.
Check rights, policy, graph revision and expiry before work or output,
and on replay and restore. Apply 0032, 0108 and 0118 to protected
inputs, history and outputs. Use 0078's admitted limits. These controls
admit no external provider, webhook, outreach, native site change or
message send. Separately approved 0100 actions retain their own scope.

| Owner job | Exact view or proposal |
| --- | --- |
| N-1382 Mid-flight freeze frame | Pause the exact eligible run through the existing action and wait for its admitted safe checkpoint or a declared hold. Show only the acknowledged safe checkpoint and authoritative effect fence. Show waiting or hold if no safe boundary is acknowledged. Filter queue and progress to the current actor and site. A settings change uses its own 0020 preview and authority. Resume with current permissions and compatible versions; an already committed or uncertain effect never repeats. No pause of protected work by implication. [FX-0099-W5-ACKNOWLEDGED-CHECKPOINT](../planning/feature-wave-5.md#acceptance-design-for-n-1382-to-n-1405) |
| N-1383 Pretend-it-broke simulator | Use an inert synthetic run of the pinned graph and one named step failure. Show declared branches, bounded repeats, dependency holds and final result from the existing dry-run engine. Simulate in memory or admitted temporary scratch, with no real data, credentials, model dispatch, script execution, sends or handler effects. Clear scratch after completion, failure or cancellation. [FX-0099-W5-INERT-FAILURE-SIMULATION](../planning/feature-wave-5.md#acceptance-design-for-n-1382-to-n-1405) |
| N-1385 Checkpoint bookmarks | Bookmark the exact eligible saved graph layout and optional valid checkpoint reference under the current site and actor. Keep authorised references and layout only, without checkpoint data, grants or progress state. Reopen or prepare a new inert draft after checking current graph and evidence. It grants no resume; the existing 0019 recovery path checks checkpoint and effect compatibility separately. An expired, inaccessible or erased reference stays unavailable. [FX-0099-W5-BOOKMARK-CURRENT-REFERENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1382-to-n-1405) |
| N-1386 Speed trend mini-charts | Use bounded completed observations for comparable job class, revision, workload, units and host profile. Separate total elapsed time, active work, waits and failed or incomplete runs where measured. Show the exact period, eligible run count, unknowns and accessible table. Never treat the chart as a release speed receipt or tune a job from it. [FX-0099-W5-COMPARABLE-ELAPSED-TREND](../planning/feature-wave-5.md#acceptance-design-for-n-1382-to-n-1405) |
| N-1387 Clone-to-sandbox | Keep a hashed immutable local sandbox clone with only the admitted inert symbolic references, distinct from test scratch. Strip model/vault references, imported paths, schedules, credentials, callbacks and grants including child grants. Replay and restore stay inert. Promotion creates a new ordinary draft under current authority. Resolve safe fixture inputs only under the sandbox profile. Activation, external calls and writes stay disabled. A workflow draft can enter an ordinary later approval only through its existing separate route. [FX-0099-W5-INERT-SANDBOX-CLONE](../planning/feature-wave-5.md#acceptance-design-for-n-1382-to-n-1405) |
| N-1397 Plain-English schedules | Parse text with an admitted bounded local grammar, or offer a typed draft for review. Show the exact schedule, zone and zone version, next UTC occurrences and any permitted native event mapping. Ambiguous wording, DST gap or unresolved fold, unknown source or day scope holds the proposal. Only the normal 0020 owner-confirmed scheduler action changes a schedule. No automatic replan or connector. [FX-0099-W5-LOCAL-SCHEDULE-PREVIEW-AND-CONFIRM](../planning/feature-wave-5.md#acceptance-design-for-n-1382-to-n-1405). |
| N-1398 Traffic-jam calendar | Project current authorised planned runs and measured resource promises in declared site-calendar slots. Distinguish planned load, active job, uncertainty and an admitted capacity conflict. Show exact offsets for repeated hours and an accessible list. No schedule change, competing site's identity or promised completion from the display. [FX-0099-W5-CALENDAR-SCOPED-RESOURCE-CONFLICT](../planning/feature-wave-5.md#acceptance-design-for-n-1382-to-n-1405). |
| N-1399 Auto-spread | Prepare one bounded local plan moving eligible unclaimed jobs within their allowed windows and current dependency and resource rules. Keep deadlines, blackout, fairness, quiet periods and protected work intact. Show old and new occurrences, holds and expected limits. Exact owner confirmation and current schedule comparisons are required before the existing scheduler changes any slot. No action from the preview. [FX-0099-W5-AUTO-SPREAD-UNCLAIMED-ONLY](../planning/feature-wave-5.md#acceptance-design-for-n-1382-to-n-1405). |
| N-1400 Finish-by-time | Propose an eligible start range from the exact deadline, zone, compatible run-duration evidence, capacity, dependencies and safe windows. A missed or unproved bound gives a reason and no promised completion. Require the existing reviewed schedule action before changing a due time. Never claim a lease, bypass an upstream hold or reprioritise protected work to meet an estimate. [FX-0099-W5-START-RANGE-WITH-DEADLINE-AND-PROOF](../planning/feature-wave-5.md#acceptance-design-for-n-1382-to-n-1405). |
| N-1401 Honest clock ETAs | Estimate a bounded finish range from compatible measured runs and current work, declared clock and known waits. Show coverage, sample size, freshness and major constraints. Paused work, unknown dependency or stale evidence can show unavailable. An estimate guarantees no finish time and changes no schedule. [FX-0099-W5-ETA-COMPATIBLE-MEASURED-RUNS](../planning/feature-wave-5.md#acceptance-design-for-n-1382-to-n-1405). |
| N-1402 Polite auto-slowdown | Show and follow the existing 0082 response to an admitted host's wait or limit signal, including current ceilings and recovery state. Apply the shared host and site cap across concurrent runs and replay. A missing or stale signal cannot accelerate work. Never switch hosts, identities or providers to avoid a pause. [FX-0099-W5-SLOWDOWN-SHARED-HOST-CAP](../planning/feature-wave-5.md#acceptance-design-for-n-1382-to-n-1405). |
| N-1403 Trigger-on-anything | Register existing typed local trigger sources only. Authenticate local ingress and native inbox event, subscription, version and digest. Intersect caller or recurring service identity, workflow, child and action grants. Use a scoped folder open that refuses links and replacement. An outside event wrapped by a local source remains outside scope. Recheck at dispatch. Conflicting bytes with a reused event ID hold. Bind exact source and site, event identity, action grant and trigger revision. Deduplicate through 0019 before a run starts; consumer retries and restore cannot revive revoked triggers. A new run uses its ordinary effect approvals. No internet listener, polling expansion, file instruction, remote hook or new service connection. [FX-0099-W5-LOCAL-TRIGGER-EXACT-EVENT-FENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1382-to-n-1405). |
| N-1404 Blackout dates | Preview an owner-approved exact local time range and its affected schedule classes. Store the zone rules and use the current 0019 occurrence fence. Admissions stop during the blackout for eligible classes; already-running work follows its admitted safe stop and protected work rules. Blackout does not grant a send, suppress required safety evidence or authorise retroactive catch-up. [FX-0099-W5-BLACKOUT-CURRENT-OCCURRENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1382-to-n-1405). |
| N-1405 Graceful upstream waiting | Hold the exact dependent job for its admitted upstream event, status or existing connector receipt with a bounded wait and deadline. Keep no worker or resource lease while waiting. Require exact current site, source, job or receipt, expected version, state and deadline. Refuse self or dependency cycles and stale or mixed proof. Race wake and expiry through one durable occurrence. Wait without worker lease or poll. Timeout produces the proper local outcome and reason. An uncertain effect stays reconciled under its own receipt. No new provider checks, repeated sends or automatic fallback. [FX-0099-W5-UPSTREAM-WAKE-OR-EXPIRE](../planning/feature-wave-5.md#acceptance-design-for-n-1382-to-n-1405). |

Use the existing 0019 event, occurrence and effect identities.
Check current plan, site, actor and grant at commit; the same key and
different inputs conflict. Commit eligible schedule changes and their
receipt together. Preserve completed, missed and cancelled occurrences.
After pause, restart or restore, compare current approval and clock
authority before making work eligible. QR-199 qualifies exact cases,
including changed deadlines, compounding waits and stale history.

### Safety rules that always apply

1. A workflow, script or agent creates proposals only. Nothing changes a
   site, deletes data or spends outside money without a human accept. A step
   that sends a message or export to a connected service is a "send step". It
   works only for a service the owner has switched on, only when the owner has
   approved that workflow's send steps, and it shows the exact content in a
   dry run. It never carries site text off the PC unless that switch is on.
2. Secret masking, hard memory and disk caps and undo snapshots cannot be
   switched off by a workflow (ADR-0021 never-off core).
3. A workflow cannot raise its own permissions. A change to who may run or edit
   it is a recorded action of the owner.
4. Dry runs perform no write, no outside call that changes state and no
   message. Read-only calls are allowed and marked.
5. Progress is shared by an exported snapshot file, never a live link. The
   export has no buttons, no secrets and no internal addresses.

### Helpers

- **Plain-English status:** a one-sentence headline, a retry countdown, calm
  amber for retries that are working, and logs with repeated noise folded away.
- **Early trouble forecast:** a steady trend warns ahead ("this crawl grows 10%
  a week and will reach your memory limit in three runs").
- **Dusty-job nudges:** jobs that have not run or helped for a long time are
  suggested for removal, never removed.
- **Morning card and playbook:** a short summary of what happened overnight,
  and an auto-written playbook per workflow (what it does, what to do when it
  fails), saved as text.
- **Speed coach and speed race:** a hint that names the slowest step, and a
  comparison of this run against the last run or a chosen run.
- **Cost meter:** outside calls and any money are shown live; there is never a
  hidden spend.
- **Ambient signals:** a taskbar ring, an engine heartbeat line and an
  optional gentle sound, all off by default.
- **Action trail:** who or what did what, kept in the audit log (spec 0032).

### Not built here

- Phone push messages (feature 1427) and spoken job control (feature 1416)
  need a decision (QR-027). Until then neither is built.
- A read-only live progress link (feature 1433) is replaced by the exported
  snapshot above (DR-028).
- Installing, moving and removing files and drives are spec 0030 and 0038.

#### Wait explanations, help and priority (N-1865–N-1868)

- **N-1865, Why is this waiting? pill.** Show the current admitted
  wait record for the exact job, site and lease generation. Name
  the safe constraint and amount where available. Distinguish
  several holds, stale evidence and unknown cause. Preserve
  accessibility and avoid exposing another site's identity or
  queue. The view does not release a wait. Trace JW-01, JW-15.
  See [FX-0099-W5-SCOPED-ADMITTED-WAIT-REASON][w5-local-jobs-cases].
- **N-1866, side-by-side What changed? view.** Pin exact eligible
  run revisions, field definitions and observation scope. Show
  changes in admitted settings, work, waits and resource evidence.
  Compare only compatible measurements and label missing,
  withheld and estimated values. A change is not proof of
  causation. No new run, settings read outside scope or replay.
  Trace JW-15, JW-17, JW-22.
  See [FX-0099-W5-COMPARABLE-AUTHORISED-RUNS][w5-local-jobs-cases].
- **N-1867, one-click Ask for help card.** Preview a bounded local
  projection of the current eligible graph, error reason and
  safe observations. Pin rendered bytes and approved recipient
  and purpose before any eligible manual copy or export. Remove
  private source, secrets, identifying addresses and executable
  links from the final representation. Recheck rights and erasure at
  delivery and restore. Bind recovery to the original site's eligible
  run, evidence revision, rendered bytes, recipient, purpose and
  original approval expiry. Current Jobs access alone cannot revive
  an old card. If the evidence revision is stale, show “Help details
  have changed. Review a new preview” with SUT-906. At approval
  expiry show “Help approval expired. Review a new preview” with
  SUT-902. If its source is erased, show “Help evidence is no longer
  available” with SUT-906. Withhold the old preview and bytes in
  each case. Restore or reopening gives no new approval, copy or
  export. A fresh approved preview delivers its exact scrubbed
  bytes only on the existing manual action. The card
  cannot send, launch an agent or give repair authority. Trace JW-12,
  JW-15 and 0032. Reuse AU-13 for the shared delivery fence.
  See [FX-0099-W5-SAFE-MANUAL-HELP-SNAPSHOT][w5-local-jobs-cases].
- **N-1868, Cut to the front button.** Pin the current queue revision,
affected eligible waiting lower lanes,
  safe checkpoint, head and leases. Preview an eligible priority
  change for the exact site, job and affected scope. Apply only
  the owning 0019/0078 action with current authority and queue
  revision. Respect the never-wait list, protected leases,
  admitted slot budget and fair turn-taking. Eligible lower
  work yields only at its safe boundary. No forced kill, stale
  resume, scheduler bypass or duplicate claim. Trace JW-10,
  JW-16, JW-20 and QR-219.
  See [FX-0099-W5-CURRENT-SAFE-PRIORITY-PREVIEW][w5-local-jobs-cases].

[w5-local-jobs-cases]:
  ../planning/feature-wave-5.md#acceptance-design-for-n-1863-to-n-1880

### Data

- `workflows`: id, name, owner, trigger, state, current version.
- `workflow_versions`: workflow, version, graph, saved by, time, note.
- `workflow_runs`: run, workflow, version, trigger, inputs hash, state, start,
  end, resource receipt.
- `workflow_steps`: run, block, state, attempts, start, end, checkpoint.
- `workflow_notes`: workflow, block, text, author.
- `alert_rules`: alert type, severity, route, quiet hours, digest, snooze.
- `alert_view_state`: site, canonical 0032 alert ID and revision,
  viewer acknowledgement and authorised snooze scope or deadline.
  Digest and route receipts use their existing owning records.

The job records themselves belong to the scheduler (spec 0019). Run history is
kept 90 days by default; a run that failed is kept 13 months.

### Settings

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `jobs.history_days` | whole number, days | 90 | 7 to 730 | next night |
| `jobs.failed_history_months` | whole number | 13 | 1 to 36 | next night |
| `jobs.approval_expiry_h` | hours | 72 | 1 to 720 | for new runs |
| `jobs.quiet_hours` | time range | none | any | at once |
| `jobs.blackout_dates` | list of dates | none | any | at once |
| `jobs.sound` | on or off | off | two values | at once |
| `jobs.taskbar_ring` | on or off | off | two values | at once |
| `jobs.dry_run_default` | on or off | on for new workflows | two values | for new workflows |
| `alerts.digest` | none, hourly, daily | daily | three values | next digest |
| `alerts.snooze_max_h` | hours | 168 | 1 to 720 | at once |

### Failure codes

SUT-900 workflow graph invalid (loop with no exit, missing block); SUT-901
step failed after all retries; SUT-902 approval expired; SUT-903 duplicate run
blocked; SUT-904 input form rejected; SUT-905 trigger source unreachable;
SUT-906 version not found; SUT-907 permission refused; SUT-908 dry run
attempted a write (a defect, the run stops); SUT-4400 incoming message rejected
(bad signature, age or size); SUT-4401 send step refused (service off or not
approved); SUT-909 snapshot export refused (contains a secret). A failure
never leaves a half-applied change.

### Security and privacy

- Sample data on an arrow is masked and size-limited; it is never stored in a
  run unless the owner switches that on for a test run.
- Incoming signed messages are checked for signature, age and size before any
  step starts; an unsigned message starts nothing.
- A workflow card shared as a file holds no secrets, no personal data and no
  site addresses unless the owner adds them.
- Data in a run is untrusted input to every later step, never an instruction.

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| JW-01 | Every waiting job MUST show the named cause of its wait. | UT-JW-01, BD-JW-01 |
| JW-02 | The Overview MUST show one plain-English headline and per-step progress. | BD-JW-02 |
| JW-03 | The workflow map MUST have a list view and text for every colour. | AC-JW-01 |
| JW-04 | A workflow MUST create proposals only and MUST NOT change a site. The one exception is a send step the owner approved for that workflow (JW-19). | UT-JW-02, BD-JW-03 |
| JW-05 | A dry run MUST perform no write and send no message. | UT-JW-03, FI-JW-01 |
| JW-06 | Every save MUST be a version, restorable, and a run MUST record its version. | UT-JW-04 |
| JW-07 | A human approval step MUST hold no memory and MUST expire. | UT-JW-05 |
| JW-08 | A retry, fallback and error workflow MUST work per step and be shown plainly. | BD-JW-04 |
| JW-09 | A duplicate run of one workflow MUST be blocked. | UT-JW-06 |
| JW-10 | "Cut to the front" MUST NOT pause the never-wait list of spec 0078. | IT-JW-01 |
| JW-11 | An incoming message MUST be checked for signature, age and size first. | UT-JW-07, FZ-JW-01 |
| JW-12 | Progress MUST be shared only as an exported snapshot with no secret. | UT-JW-08 |
| JW-13 | The alert centre MUST apply severity, routing, quiet hours, digest and snooze. | BD-JW-05 |
| JW-14 | Schedules MUST accept plain words and MUST NOT require cron codes. | UT-JW-09 |
| JW-15 | A run's secrets MUST be masked in every screen, log, sample and export. | UT-JW-10, FZ-JW-02 |
| JW-16 | A workflow MUST NOT raise its own permissions. | UT-JW-11 |
| JW-17 | Estimates MUST show a range and the number of past runs they use. | UT-JW-12 |
| JW-18 | The screen MUST show its state in the no-job, offline and failed cases. | AC-JW-02 |
| JW-19 | A send step MUST work only for a switched-on service with approved send steps, and a dry run MUST show its exact content. | UT-JW-13, BD-JW-06 |
| JW-20 | The command bar MUST show a preview and need a confirm before any action. | BD-JW-07 |
| JW-21 | Heavy jobs MUST pause on low battery and ease off when hot, with a plain note. | IT-JW-02 |
| JW-22 | Run history MUST follow `jobs.history_days` and `jobs.failed_history_months`. | UT-JW-14 |
| JW-23 | A single-item test drive MUST run one real item for a short, capped time and change nothing. | BD-JW-08 |
| JW-24 | Quiet hours and blackout dates MUST stop scheduled work and say why. | UT-JW-15 |
| JW-25 | A trigger message MUST arrive only through the local door, never a network port. | IT-JW-03 |

### Portfolio summary read

Return the count of unresolved, unacknowledged alerts for this site
and the earliest last-progress time among running jobs. A snoozed
alert remains open. Use the job start time until its first committed
progress under 0019. A lease renewal alone leaves that time unchanged.
Queued, waiting, paused and finished jobs do not enter that time.
For a complete summary with no running jobs, return no progress
time with complete coverage. An unavailable summary carries its
failure state and cannot establish that no job is running.
Spec 0102 uses this summary to detect a stall without listing runs.

Expose one bounded, site-scoped summary read under spec 0102. Its
source and failure cases run in IT-SS-06 with the owning module.

### Named workflow scope, recovery and preview fixtures

These planned spec 0076 cases use synthetic receipt, quota and
credential fixtures. A real-item preview uses an eligible local
item within the existing cap. QR-142 and QR-081 hold admission.

- `DES-0099-W2-LOCAL-RECOVERY` uses
  `VAR-0099-W2-RESTORED-REVISION` through the existing 0019
  checkpoint and effect fence. A graph restore creates a draft
  revision. It gives no older run an approval or undo.
- `DES-0099-W2-EFFECT-LIMITS` uses
  `VAR-0099-W2-SHARED-RESERVATION` and
  `VAR-0099-W2-SCOPED-VAULT-HANDLE`. Reuse existing 0100 owning
  route caps, exact effect IDs and approved 0065 resolution.
- `DES-0099-W2-LOCAL-PREVIEW` uses
  `VAR-0099-W2-BOUNDED-TEST-MODES` and
  `VAR-0099-W2-READ-ONLY-PREVIEW`. The fixture for a labelled
  read makes no request. An actual non-local read would need
  its separate admitted free read route, current grant and
  approved data transfer. A preview or confirmation grants
  no provider action, public transfer or spend by itself.
- `DES-0099-W2-TYPED-SCOPE` uses
  `VAR-0099-W2-CURRENT-ROLE-CHECK` and
  `VAR-0099-W2-CHECKPOINT-VALUE`. Apply current grants and
  agreed projections on every step and restore.
- `DES-0099-W2-GRAPH-AUTHORING` uses
  `VAR-0099-W2-INERT-WORKFLOW-CARD` within the existing 0100
  local file profile. A local file export grants no publication.
  Imported templates stay inert until their own scoped approval.

| Feature | Requirement | Design and variant | Fixture and test | Named fixture assertion |
|---|---|---|---|---|
| F-0470 | JW-06, JW-16 | DES-0099-W2-LOCAL-RECOVERY, VAR-0099-W2-RESTORED-REVISION | FX-0099-W2-RESTORE-GRAPH, UT-JW-04, UT-JW-11 | Pause an earlier run with one committed effect. Withdraw its action grant and keep separate same-site read access to its history. Restore a prior graph as a new draft and hold activation. Show the permitted earlier result and checkpoint reference against their original version. Resume and replay stay held with the safe permission reason. No checkpoint is reattached and no committed effect is repeated or undone. |
| F-0471 | JW-08, JW-09, JW-16 | DES-0099-W2-EFFECT-LIMITS, VAR-0099-W2-SHARED-RESERVATION | FX-0099-W2-ACCOUNT-QUOTA, BD-JW-04, UT-JW-06, UT-JW-11 | Race synthetic callers across sites for the last unit of a mocked account cap. Commit one intent and reservation. Uncertain use stays reserved across restart, rotation and graph edit. Revoke to fence new dispatch. |
| F-0472 | JW-15, JW-16 | DES-0099-W2-EFFECT-LIMITS, VAR-0099-W2-SCOPED-VAULT-HANDLE | FX-0099-W2-VAULT-HANDLE, UT-JW-10, FZ-JW-02, UT-JW-11 | Use an inert handle stub with exact site, account, action, destination and grant. Only the owning Rust action can resolve it. Refuse raw or mismatched inputs. Expose no secret or handle ID in editor or files. |
| F-0473 | JW-05, JW-23 | DES-0099-W2-LOCAL-PREVIEW, VAR-0099-W2-BOUNDED-TEST-MODES | FX-0099-W2-SAMPLE-PREVIEW, FI-JW-01, BD-JW-08 | Inject a failure in a synthetic sample. Separately test an eligible local item at the five-second cap and just over. Distinguish both modes and stop the over-cap preview without a hidden provider read or side effect. |
| F-0474 | JW-05, JW-19 | DES-0099-W2-LOCAL-PREVIEW, VAR-0099-W2-READ-ONLY-PREVIEW | FX-0099-W2-DRY-RUN, UT-JW-03, UT-JW-13, BD-JW-06 | Use a local read and an offline fixture for a separately approved labelled read. Show the permitted effect preview. Reject mutation, paid lookup, actual send, secret disclosure and any durable reservation or confirmation. |
| F-0475 | JW-16 | DES-0099-W2-TYPED-SCOPE, VAR-0099-W2-CURRENT-ROLE-CHECK | FX-0099-W2-RUN-SCOPE, UT-JW-11 | Give run-only scope and try edits or a foreign-site trigger. Revoke while waiting and resume. Cedar rechecks current actor, site and step rights, and stale or wider authority makes no effect. |
| F-0476 | JW-06, JW-16 | DES-0099-W2-TYPED-SCOPE, VAR-0099-W2-CHECKPOINT-VALUE | FX-0099-W2-VARIABLE-SCOPE, UT-JW-04, UT-JW-11 | Compare run and protected-table scopes after checkpoint restore. Refuse a wrong type or unapproved projection. Revalidate source, consumer and erasure state before use or release. Hold an ineligible restored value. |
| F-0477 | JW-06, JW-15, JW-16 | DES-0099-W2-GRAPH-AUTHORING, VAR-0099-W2-INERT-WORKFLOW-CARD | FX-0099-W2-PORTABLE-CARD, UT-JW-04, UT-JW-10, UT-JW-11 | Round-trip a bounded inert card through the local picker. Executable input, unknown action, secret or saved grant is refused. A valid import creates a local draft pending current reference mapping and separate approval. |

## How it is tested

Designed, not run.

- **Unit (UT-JW):** the wait-cause mapper with one case per cause; version
  save and restore; the duplicate-run blocker; schedule words to times (with
  one value just inside and one just outside quiet hours); masking.
- **Behaviour (BD-JW):** the Asha scenario above step by step; a retry that
  succeeds; an error workflow that runs; the alert flow with a snooze.
- **Send and trigger cases:** a send step to a switched-off service is refused
  and its dry run shows the exact content (UT-JW-13, BD-JW-06); the command
  bar shows a preview and needs a confirm (BD-JW-07); a trigger message sent
  to a network port is refused and one through the local door starts the run
  (IT-JW-03); history is pruned at the setting values (UT-JW-14); the
  five-second test drive runs one real item and changes nothing (BD-JW-08);
  heavy jobs pause on low battery and ease off when hot (IT-JW-02).
- **Integration (IT-JW):** cut-to-front while a never-wait job runs; a
  workflow that calls the sidecar and the Model Hub; a replay from a
  checkpoint after a restart.
- **Fault injection (FI-JW):** the app is killed mid-run and the run resumes
  from the last checkpoint; a dry run is made to try a write and stops.
- **Fuzz (FZ-JW):** malformed incoming messages and graphs never start a step
  or crash the screen.
- **Accessibility (AC-JW):** every screen on both systems, with a keyboard
  and a screen reader, in all states.
- **Measurement:** time to draw a graph of 500 steps; memory of the screen
  with 10,000 runs in history (inside the budget of spec 0078).

## Changes from the source

This spec is new. It takes the owner's job-screen and workflow answer
(ADR-0021) and the pasted catalogue of 19 November 2026. Features 458 to 477,
602 to 607 and 676 to 680 of the old list now have this one home. Spec 0019
owns product scheduling and recovery. Feature 1433 uses a snapshot. Features
1416 and 1427 need QR-027 evidence. Scheduler readiness follows QR-081.
