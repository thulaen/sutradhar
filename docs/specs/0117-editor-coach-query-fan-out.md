# 0117: Editorial Coach Query Fan-Out Simulator

The Coach checks a draft against smaller questions about its topic.
In a made-up map guide, it flags a buried answer about damp storage
and offers a link to the site's published page on drying paper.

This is a practice run with no provider calls or citation prediction.
It never edits or publishes, and supplies no ranking or training data.

## Words used here

A **fan-out** is a set of smaller questions about the main topic.
A **passage** is a short text range. A **fence** is a saved version or
permission marker checked again before releasing any result.

## What this spec designs

| Behaviour | This spec owns | Shared design |
|---|---|---|
| Quick Search and Deep Research | Angle and expansion rules, offline fallback | Models 0024 and 0080, jobs 0019 and budget 0078 |
| Passage X-ray and coaching | Draft segmentation, scoring, gap routing and advice | Coach 0041, sources 0084, local site index 0038 |
| Citation rehearsal | A voluntary comparison of admitted snapshots | Leviathan 0082 and 0115, rival scope DR-062 |
| Custom rules and tool access | Typed, bounded inputs and outputs | Python 0104, local MCP 0020, permissions 0101 |
| History and cleanup | Derived run records and ledger groups | Files 0030, erasure 0032, storage wizard 0038 |

DR-101 admits typing-time checks and rivals for this Coach.
Spec 0132 owns the separate paid keyword lookup. Its cached
evidence can be used locally. A fan-out run cannot start it.
Spec 0133 may supply admitted dated search evidence and one-off
comparison excerpts. Keep them labelled and apply the existing
local scoring, consent and storage boundaries. Feature 238 in
spec 0131 is separate. Its protected evidence is unavailable to
this simulator. Spec 0093 retains actual answer file analysis and
DR-086's provider exclusions.

## Components and permissions

1. Live typing runs in the Tauri and Angular desktop editor. A local
   request carries a typed text tree, actor, draft ID and revision.
   Native staff editors use approved service-polled saved snapshots
   or local import. Sites never call the PC. Spec 0094 keeps its own
   native typing-time map and does not carry draft text for fan-out.
2. The Rust Coach owns expansion, segmentation, scoring, decisions,
   task requests and validation. It takes an admitted model lease from
   the scheduler for each model stage. It invokes no local runtime or
   native library outside Model Hub. Fan-out data stays out of ranking.
3. A read-only adapter queries the active site's published index under
   its own permission marker. Allow only the caller's own unsaved
   revision or an authorised saved revision.
4. Fence Python hooks with spec 0104. Rust validates the result,
   owns all permissions and uses the built-in result on hook failure.

| Hook | Input | Output |
|---|---|---|
| expand | topic, mode, locale, enumerated angle | candidate questions with angle and parent ordinal |
| route | angle, core-intent flag, eligible page IDs, coverage state | one allowed action and optional eligible page ID |
| rubric | pair score and evidence-present flags | advice warning codes and format enum |

Unknown fields, invalid references and raw advice text are refused.
Hooks get no network, disk, rival bytes or write handles.

## Data contracts

IDs are UUIDs, times UTC and hashes SHA-256 over versioned input.
Fields are required unless marked optional or nullable. Use the
authenticated site and forced row rules of spec 0108. Recheck site,
source and actor fences before every delivery or write.

| Record | Fields and constraints |
|---|---|
| Run | run ID, site ID, actor ID, draft ID, draft revision hash, topic (1–240 Unicode characters), mode (quick or deep), locale (`en-GB`), stage, status, generation u64, created time, last update time, optional expiry, query/profile/model version IDs, site index generation and authority generation, comparison snapshot IDs, optional failure code |
| Query | run ID, ordinal u8 starting at 1, angle enum, text (1–240 Unicode characters), parent ordinal or null, hop u8, normalised question hash, generator (GGUF, local MCP or heuristic), rule version. Unique canonical question per run. Parent precedes child |
| Passage | run ID, passage ID, kind (structure or meaning), block ID, start and exclusive end UTF-8 byte offset, content hash, optional heading ID, revision hash, token count u16. Ranges end on character boundaries in the unchanged editor snapshot and are disjoint within a kind |
| Pair score | run ID, query ordinal, passage ID, embedding and reranker keys, cosine and calibrated relevance (signed and unsigned fixed-point at scale 10,000), direct-answer and standalone booleans, evidence spans, warning codes, optional refusal reason. One outcome per matrix cell |
| Angle result | run ID, query ordinal, status (quotable, weak, missing or unassessed), optional best passage ID, coverage counts, issue codes, optional advice reference. Ties sort by content start then passage ID |
| Gap | run ID, query ordinal, action (answer_here, link_existing or cluster_topic), reason code, optional same-site page ID and canonical URL for link_existing, optional deduplicated cluster ID for cluster_topic, evidence IDs and source generation. One action per eligible missing query |
| Rival comparison | run ID, query ordinal, local passage ID or null, eligible rival snapshot and passage IDs or null, outcome (draft, rival, tie or insufficient), scoring profile, fetch date and provenance. Never label this a provider citation |
| Ledger pointer | run ID, storage ledger entry ID, record class, site ID, digest, bytes u64, retention generation, optional approved destination and proof receipt. Object names contain opaque IDs only |

Stages are queued, generating, segmenting, embedding, reranking, routing,
comparing and finishing. Status is active, paused, cancelled, held,
complete, failed or deleting. Only a complete generation offers current
findings. Held coverage is explicit. Empty, missing and unassessed are
different. Live passage and scoring records refer to the active
snapshot. Durable Coach history contains only the approved derived
fields in Security and privacy. The ledger pointer keeps its own rules.

## Execution and outcomes

1. Quick Search emits 8–15 total, including one follow-up round.
   Deep Research emits 30–50 total across hops 0–3. Angles are
   Equivalent, Canonical,
   Generalisation, Specification, Follow-up, Entailment, Clarification,
   and Freshness and regional context. Include all eight angles and
   at least one child. Entailment covers risks, prerequisites and
   trade-offs. Local context is British English. Dates and prices
   from the generator are questions, never evidence.
2. Run the selected, admitted GGUF model, connected local MCP agent or
   deterministic offline generator. The MCP route receives only the
   topic and mode after the writer's explicit Run action and grant.
   Verify same-host execution and network-disabled worker admission
   before handing off data. An unattested agent is unavailable for
   this route. Deduplicate by spec 0026 normalisation plus case folding
   and terminal punctuation removal. Treat a missing angle or
   missing eligible child as a shortage even at the mode's count
   cap. Reserve space for both before keeping surplus questions.
   Fill with the stable offline templates under the same
   deduplication and hop rules. Check the final unique count,
   all eight angles and at least one child with its retained
   parent before scoring. If the set still fails any of these
   checks, hold it and ask for a new topic.
3. Chunk by heading, paragraph, list and table structure with H2/H3
   boundaries, and separately by 2–4 sentences of one coherent topic.
   A short section may have one sentence. Split overlong passages at
   structure or sentence boundaries; hold an overlong indivisible
   unit. Preserve list or table headings as context for standalone
   testing. Render them in the X-ray without modifying the draft.
4. Embed all questions and all admitted passages with the admitted
   ONNX model. Compute cosine similarity for every pair, then score
   every pair with the ONNX cross-encoder under the same manifest.
   Use the pack's signed monotone piecewise-linear calibration map.
   Round half up to 0–10,000. Quotable needs at least 7,500 plus directness,
   a named subject and an eligible answer span inside the passage.
   Otherwise at least 4,000 is weak. Rank status first, then calibrated
   score, then the stable passage tie order. Missing needs complete
   coverage below 4,000. Incomplete coverage is unassessed.
5. For missing in-scope angles first check the current own-site index.
   Reuse a published, readable, canonical same-site answer when one
   passes the Coach's qualification check, showing its exact link.
   With a complete index check and no match, keep the core intent on
   this page. Suggest a cluster topic only for a distinct substantial
   intent that can stand alone. Merge synonyms and overlapping intents
   into one proposal. Show unrelated angles as held with no action.
   A stale, unavailable or incomplete index withholds routing, including
   any suggestion to create a separate page. A cluster suggestion
   remains available for review while its own evidence is current.
   QR-240 binds the private destination,
   exact owner confirmation, data fields and lifecycle before admitting
   the separate Save write or its implementation task. Coach history
   approval supplies no plan authority. A reviewed suggestion cannot
   create a page.
6. When the writer requests comparison, accept a current scrubbed
   Leviathan Receipts excerpt or an owner-supplied static UTF-8 text
   excerpt with URL, capture time, digest and reuse grant. An optional
   supplied search observation names date, query and scope. Compare
   both sides under the same rubric. A margin below 500 is a tie.
   Incomplete or unsupported evidence is insufficient. Never retain
   a rival crawl page, fetch from the panel, or schedule rival work.
   Switch-off or source revocation invalidates pending and saved
   comparison results before delivery, reload or storage move.
7. On typing after opt-in, debounce the latest editor revision and
   rescore only changed passages against the pinned question matrix.
   Reuse cells only for identical input bytes and model/rule keys.
   Invalidate stale cards immediately. Re-expand only when the writer
   changes topic, mode or generator and requests a fresh run. Never
   rerun the MCP route or rival fetch while typing.

## Screen and words

Editor on the left, Coach on the right, with a Coach tab on narrow
screens. Run and Pause sit below topic and mode. Angle cards show
“Quotable”, “Weak or buried”, “Missing” or “Still checking”. A card
focuses its passage. X-ray marks both views and has a keyboard list
with angle tags. Keep focus and announce only settled changes.

The standalone check names an unclear reference and shows the phrase.
The answer-first check covers the opening 100 words and first sentence
after each eligible heading. Literal heading advice shows an example
question matched to the angle. Format advice maps Canonical to a short
definition, Specification and Entailment to a comparison table,
Follow-up to a checklist and Clarification to a compact FAQ. Other
angles default to a short answer with a cited example. Specificity
marks visible numbers, steps, named entities or concrete examples as
checkable details. It never treats a number as independently verified.

Banner: “This is a practice run. It shows how clear your answers are.
It does not tell you what a search service actually asks or cites.”
For held angles: “This question belongs to a different topic. Leave it
out unless it helps your reader.” Advice and links are copyable.
At clipboard dispatch the desktop checks the active editor site, current
actor and Cedar grant, draft and exact revision, run generation and
permitted source against the selected card. Serialise the final check
and clipboard hand-off with active editor changes. Cached results or
identical text cannot satisfy a different scope or revision. Refuse a
foreign site or denied grant through 0020 before resolving the card.
Use SUT-3604 for a stale card on the authorised site. Refusal leaves
the clipboard as it was. A fresh eligible card copies only its current
permitted advice or link. Copying grants no editing, storage or outside
request. A person applies changes in the normal editor.

## Settings and bounded work

Defaults are proposed design targets. Admit exact packs and measure
on the supported systems before offering live mode.

| Setting | Type | Default | Allowed | Takes effect |
|---|---|---|---|---|
| `fanout.mode` | enum | quick | quick, deep | next run |
| `fanout.generator` | enum | GGUF if admitted, otherwise heuristic | GGUF, local MCP, heuristic | next run |
| `fanout.live` | bool per editor | off | explicit on or off | next revision |
| `fanout.rivals` | bool per site, owner only | off | on or off | disabling invalidates access immediately |
| `fanout.keep_days` | whole days | 7 after owner approval of local history | 0–30 | next cleanup, revocation immediate |

`fanout.rivals` is owner only. The CLI, MCP and agents cannot change
it.

Cap an admitted run at 12,000 words, 512 passages combined, 256
model tokens per passage and 128 per question, including model
prefixes. Hold above caps rather than silently truncating coverage.
Compare at most five snapshots and at most 128 rival passages total.
Use a 250 ms debounce, one active and one latest queued revision
per editor, and batches admitted by the scheduler. A hook has
250 ms wall time and 64 MiB within the system sidecar cap. Full
Quick and Deep runs time out at 30 and 90 seconds on the 16 GiB
reference CPU. Comparison has its own 30-second deadline. A missed
deadline reports held coverage, never a green result from partial
work. Target warm workstation p95 at most one second from key-up
for a one-passage edit in a 1,500-word Quick fixture, inclusive of
debounce and ordinary desktop work. Show progress on larger jobs.

Use spec 0078's detected physical memory and RAM and graphics budgets.
On the 16 GiB baseline allow one heavy model lease at a time, release
and confirm unload before acquiring the next. Above the shared single
model boundary, allow concurrent residents only when combined measured
CPU/GPU peaks, buffers and headroom fit the admitted leases. CPU fallback
uses the same evidence rules and reports the profile. Scheduling yields
to live ranking and never evicts its protected work to make room.

An unload request stops admissions that depend on reclaiming its lease.
If teardown is unproved after a crash, keep its original RAM and graphics
charges under 0033's promise check and 0078 accounting. Pause, cancel,
restart and a reading with more free memory cannot release the charge.
Keep the affected editor result held. Match supervised teardown to the
original attempt, worker generation, model and exact lease. Reconcile
its RAM and any graphics allocation under 0078 and the authorised
human recovery under 0033. Only the proved release can free the charge.
A later scoring stage needs current authority, snapshot and resource
admission under 0019 and 0078.

## Failure codes

| Code | Condition | Message and recovery |
|---|---|---|
| SUT-3600 | Missing or refused generator | “Using offline questions.” Record fallback. |
| SUT-3601 | Model lease refused or runtime unavailable | “The scoring model does not fit right now.” Keep questions, withdraw scores and let the writer retry. |
| SUT-3602 | Scheduler paused or generation cancelled | “Coaching paused.” Stop work and keep leases charged until their release is proved. Resume only a current authorised snapshot under existing admission. |
| SUT-3603 | Topic, coverage or time cap | “Some material could not be checked.” Hold affected results and point to the limit. |
| SUT-3604 | Draft or authority changed | “This draft has changed. Run the check again.” Invalidate outstanding deliveries and dependent advice. |
| SUT-3605 | Rival material unavailable | “There is not enough permitted material to compare.” Preserve any independent local result. |
| SUT-3606 | Storage or removal proof unavailable | “This location needs attention.” Hold use or completion until verified. |

Codes SUT-3600 to SUT-3699 are reserved for this spec (DR-102).

## Security and privacy

Draft and rival bytes are untrusted input. Decode data under strict
size and schema limits. Never use either as prompt instructions.
The generator sees the explicitly entered topic in a data field.
Workers have no tools, dynamic imports or authority from model output.
Results cannot render active markup, quote private material or enter
logs. The evidence span and grant must still hold at delivery.
Use synthetic topics, fixtures and replay traces only.

MCP actions use spec 0020. Page cursors bind site, actor, scope,
draft and generation. Recheck grants on each page. Unsaved run
metadata requires the initiating actor and an explicit per-run grant.
No tool sees unsaved text, approvals, writes or storage controls.

| MCP action | Input | Result |
|---|---|---|
| `fanout.list_runs` | optional cursor | authorised run IDs and safe state |
| `fanout.read` | run ID, view (scores, gaps, rules), optional cursor | authorised typed records without source bytes |
| `fanout.stop` | run ID, generation, pause or cancel | authorised run state; global Pause all stays in the owner UI |

Spec 0030 registers all caches, temporary files and history objects.
Local history starts off. In the owner app, the site owner can
approve the exact site, derived history fields and retention period.
Matching writes may use that current revocable approval. A change
to the scope, fields or retention needs a new approval. Bind each
commit to the current approval and erasure generation. A run,
live editor opt-in, tool grant or non-zero `fanout.keep_days`
value gives no storage approval. A value of zero keeps no history.
On revocation, stop history writes and reads and use the existing
erasure route. The unsaved active preview remains transient under
its own current authority. Keep any required cleanup ledger
without adding content or recovering denied history.

Persist only opaque IDs, source and model versions, dates, angle and
mode, numeric scores, status, action and warning codes. Topic and
question text, draft and rival bytes, model prompts and full passage
and comparison records are outside this history projection. An
approved SSD or S3 move uses spec 0038 and a site key in tijori. Verify both
versions and erasure fences before switch or restore. Stop new
writes and revoke handles before deletion. Zero trace needs absence
receipts for every selected local and remote version. Report Keep and
unreachable storage. Never erase the source draft or published page.

## Requirements

Each requirement names the test that proves it. The tests are defined
in "How it is tested".

| Requirement | Required behaviour | Acceptance evidence |
|---|---|---|
| REQ-0117-EXPAND | Each run MUST cover all eight angles within its count and hop limits. | UT-0117-EXPAND exercises all eight angles, both hop and count boundaries, deduplication, malformed provider output, offline fallback and empty or unfit topic. |
| REQ-0117-SCORE | Each admitted pair MUST be scored locally and map to the exact status rule. | UT-0117-SCORE checks exact offsets and each matrix cell, ties, thresholds, standalone and answer evidence. Golden fixtures include heading, list, table, pronoun and missed-answer cases. |
| REQ-0117-ROUTE | A gap action MUST have current same-site evidence or complete absence evidence. | IT-0117-ROUTE checks exact own-site target, equivalent gaps, substantial missing intent, unrelated angles, stale index and both balanced and insufficient rival outcomes. Invalidate a reviewed cluster suggestion after a new index generation, erased evidence or changed draft revision, before any candidate Save. No plan write, page creation or ranking mutation. QR-240 governs positive Save admission. |
| REQ-0117-EDITOR | The live editor MUST bind findings and controls to the active revision. | BDD-0117-EDITOR covers mode choice, X-ray, keyboard and screen reader, six coaching checks and stale-card removal and clipboard refusal after a site or draft revision change. A human remains responsible for applying changes. |
| REQ-0117-ACCESS | Agent and hook routes MUST enforce source and actor bounds and MUST NOT apply a change. | IT-0117-ACCESS checks wrong site, unsaved colleague draft, hostile text, invalid Python reply, remote MCP refusal, permission withdrawal and no auto-edit, publishing or storage consent. |
| REQ-0117-BUDGET | Each stage MUST stay within its time and admitted memory lease. | E2E-0117-BUDGET checks full and changed-passage timing, CPU fallback, hard cap, peak RAM and graphics usage, pause, timeout and model unload before swap. |
| REQ-0117-LEDGER | Local history MUST require current exact owner approval and contain only the approved derived fields. All copies MUST be ledgered and removal MUST verify absence before claiming zero trace. | FI-0117-LEDGER checks history off, exact approval, no full records in history, retention zero and withdrawal before commit or replay. Interrupt SSD and S3 move, upload and deletion before and after durable receipts. Restart preserves authority and cannot claim clean removal until all selected objects are absent. |
| REQ-0117-STATE | The invariant suite and implementation trace replay MUST both pass. | QM-0117 and RP-0117 verify the bounded Quint model and replay its trace actions against the Rust job and ledger services through Quint Connect. |
| REQ-0117-PARITY | The same Bazel gates MUST pass on all three supported systems. | PL-0117 runs the same data, state, permissions and outcome gates under Bazel on Ubuntu 26.04, Debian 13 and Windows 11. Public-page budget is zero extra queries, bytes and work. |

## How it is tested

Designed, not run. Model the scheduler, result fence and file ledger as
a finite Quint state machine. Use one owner and two sites, two draft
revisions, two model leases and two stores. Abstract bytes as bounded
counters. States include stage and run status, current and worker
generation, scope and authority epoch, pause flag, resident and
unreconciled leases, in-flight tasks and ledger intents, committed and
deleted entries. Transitions include submit, edit, admit, request and
verify unload, reconcile an unknown release under the 0033 rules,
finish, fail, pause, resume, cancel, revoke, move, require deletion,
verify and restart.


Test ids and where each is defined (all designed, not run):

- **UT-0117-EXPAND, UT-0117-SCORE:** unit tests, defined in the
  acceptance evidence of REQ-0117-EXPAND and REQ-0117-SCORE.
  In UT-0117-EXPAND, give Quick 15 distinct valid roots in one
  angle and Deep 50 candidates missing an angle after
  deduplication. Verify deterministic offline completion within
  the mode's count cap, all eight angles, and parent-before-child
  order and hop limits. When an offline template cannot supply an
  eligible missing angle or child, the run holds before scoring.
- **IT-0117-ROUTE, IT-0117-ACCESS:** integration tests, defined in
  the acceptance evidence of REQ-0117-ROUTE and REQ-0117-ACCESS.
  In IT-0117-ROUTE prepare a substantial cluster suggestion on the
  authorised site with complete absence evidence. Hold its candidate
  Save at the final evidence check. Replace the index generation or
  erase named evidence in separate cases. Observe the evidence refusal,
  the withdrawn route and zero calls to a stand-in destination writer.
  Leave no plan record, draft, job or native effect. A new complete
  check can offer fresh review. In a separate case change the draft
  revision while keeping the selected text byte-identical. Refuse
  on the authorised site with SUT-3604. These refusal checks
  supply no positive Save authority under QR-240.
- **BDD-0117-EDITOR:** behaviour test of the live editor. Prepare a
  current link card on site A and hold its Copy before clipboard
  dispatch. Switch the same actor to site B with identical topic
  and passage text and a different eligible same-site link. Deliver
  the old Copy. Refuse with the existing 0020 scope outcome and
  leave the clipboard unchanged. On an authorised site, repeat
  after an edit changes the revision. Refuse with SUT-3604. Run a
  fresh eligible local check on B. Copy its own current link.
  The actions leave the draft and Coach history unchanged and
  send no provider, MCP or rival request.
- **E2E-0117-BUDGET:** end-to-end timing and memory test. On the 16 GiB
  fixture, interrupt the worker after it requests unload and before
  release is proved. Exercise pause and cancellation in separate runs.
  Restart the service and editor from the durable state. Supply a
  fresh reading with apparent headroom and a late result for the old
  lease. In a separate run lower the applicable budget. While release
  is unknown, keep the original charge. Withhold the affected score
  and refuse a second heavy lease, including a CPU fallback request.
  Try a teardown proof for a different worker generation. Require
  the original release proof and reconcile it under the existing
  guards. With a current authorised snapshot, admit the next stage
  only if its own lease fits.
- **FI-0117-LEDGER:** fault injection on the file ledger.
- **QM-0117, RP-0117:** the bounded Quint model and its trace replay.
- **PL-0117:** the same Bazel gates on the three supported systems.

Invariants: admission counts resident and unreconciled leases and
buffers against both current caps. Count an unknown unload as
resident for the single-heavy limit until verified recovery. If a
cap shrinks, keep all commitments charged, refuse over-cap admission
and use existing pressure handling under 0078. Pause, withdrawal, edit,
cancel and uninstall invalidate every stale publication attempt.
No transition
writes a draft, affects ranking or approves a plan. A visible object
has a committed ledger entry. A completed erase has no selected object
or usable handle. The Rust replay driver injects the model's action at
each simulated lease, transaction and I/O boundary and compares state
after every step, including late completion and recovery. Changed models
gate commit; all models gate push; release runs the deeper matrix.
Admit the pinned [Quint Connect](https://github.com/quint-co/quint-connect)
toolchain under the shared Bazel rules before using replay evidence.

Acceptance evidence includes the full test trace, admitted model packs,
calibration and latency receipts, review and all platform gates.

## Changes from the source

Owner decision DR-101 admits typing-time checks and rival rehearsals
for this Coach, with Quick Search at 8 to 15 questions and Deep Research
at 30 to 50. It stays a practice run: no provider calls, no citation
prediction and no ranking or training data. Paid keyword lookup stays
with spec 0132. Admitted dated evidence and one-off excerpts may
come from spec 0133. Spec 0131 retains its separate feature 238
evidence. Spec 0093 keeps actual answer file analysis under DR-086.
Failure codes SUT-3600 to SUT-3699 are reserved for this spec
(DR-102). Numeric defaults are proposed design targets until measured.
