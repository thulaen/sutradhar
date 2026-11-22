# Spec 0140 - Unified site audit suite

Use Site Audit to see what a guest can reach, what a page contains,
where it is slow and what to review first. Open a finding to see
the evidence, its limits and steps for WordPress or XenForo.
The free local tools work on saved evidence when the PC is offline.

## Authority and scope

The suite has five parts, with one owner for each effect.

| Part | Design | Existing effect owner |
|---|---|---|
| Crawl and log review | [0141][crawl], CR/LG | 0044 for crawl facts, 0082 for requests, 0116 for sealed comparisons, 0034 for log disclosure |
| Technical Toolbox | [0141][crawl], TB, [0143][grid], LP, and [0145 structured-data and bot inventories](0145-audit-rule-and-field-catalogue.md#structured-data-rules-and-bot-policy-coverage) | 0044/0047 for findings, 0115 for renders and markup facts, 0132 for provider budget and transport, and 0145 for exact inventory and case evidence |
| Performance Lab | [0142][lab], PLAB | 0115 for the renderer and broker, 0047 for comparable speed evidence and regression notices |
| Site Health and URL Explorer | [0143][grid], AG and [0145][rules], AUD-0145 | 0043 for Alekh typed views, 0048 for the Fix List, 0100 for supported export formats |
| Text Mining | [0144][text], A144 | 0026 for existing text projections, 0024/0080 for packages and 0049 for existing graph contracts |

DR-110 keeps staging snapshot-only. The owner can approve one
live pair for an exact owned public page under 0141. Both reads are
guest reads with declared Leviathan identities and separate legal
checks, including robots.txt for each request. The Browser leg goes
through an admitted 0115 render path. Compare the pre-script main
observations only. A named outside
crawler can appear in stored or supplied evidence or an offline
robots rule evaluation. Neither path proves what that crawler saw.

Market and regional comparisons use supplied evidence. The two 0115
live viewport profiles and fixed locale remain the only live
rendering choices. Site Audit can read separately qualified
languages of stored own-site content. British English remains the
interface and live link-ranking language. The private names view in
0144 has its own admission and owner confirmation. Ordinary results
keep person counts under the existing disclosure rules.

Use supplied Matomo aggregates for an approved grid join. Spec 0136
owns direct Matomo access for its timeline. Specs 0040 and 0044
own Search Console, GA4 and eligible field-speed evidence. A paid
local search preview in 0143 has its own scope, maximum charge and
approval. The suite never schedules competitor or regional crawls,
rewrites a live page, adjusts ranking, deploys configuration, or
sends site data as part of a report. It shows coverage and category
priorities. Overall letter grades and health or discoverability
scores are outside this scope.

Use maintained components admitted by the owning specs. Reuse
Leviathan's raw and gated render levels, Behemoth Boot modules,
Alekh and the local Model Hub. The suite supplies first-party
orchestration, evidence, checks and screens. It ships no external
audit executable or vendor service dependency for its core.
Appearance and copy are Sutradhar's own. Public vendor documents
describe behaviours for comparison and never authorise source,
artwork, formula or screen reuse.

## System boundaries

- **Rust core.** Use typed Boot APIs for the Audit planner, source
  resolver, rule evaluation and report publication. Resolve rights
  before existence. Pass only permitted evidence projections between
  modules. Use 0048 for findings and suggested work and 0043 for
  Alekh views. Keep authority, settings, consent, jobs, results
  and evidence metadata in PostgreSQL. A permitted Alekh cache
  has no authority and uses 0118 custody.
- **Fetch and render.** Use the 0082 legal broker and 0115 cage.
  Resolve each next hop and downgrade only to an explicitly
  supported lower fact level. Report a level, failure, or scope cap
  next to the fact. Keep external link destinations inert unless
  already allowed by the owning acquisition contract.
- **Model Hub and Python.** Invoke model and 0104 workers only
  when an exact operation and profile is admitted. Bind operation,
  package/script digest, current grant, schema, time, memory and
  output caps. Declared input columns are the only data. Code/data
  channels are distinct. Python cannot contact the network or run
  site actions. Rules and model outputs are advisory.
- **Desktop.** Use Tauri, Angular 22 and the existing design system.
  A site picker and source banner persist from Site Health to the
  drawer, job receipt, export and Fix List. Show the same results
  in a keyboard-accessible list and graph. Show units, origin,
  level, date and completeness in both. Action labels say
  what happens, including when an operation costs money.
- **Local MCP.** Expose admitted typed summaries through 0020
  and 0118. Separate permission to plan, read and use a protected
  projection. Resolve all inputs within the session's single site.
  Owner approval is in the app. No raw page body, retained log
  line, image, credential, private name view, arbitrary SQL,
  external path or audit log disclosure.

## Shared data and release rules

| Record | Exact members |
|---|---|
| `audit_request` | `site_id, project_id, run_id`: opaque IDs. `kind`: crawl, log, toolbox, lab, grid, corpus or local_preview. `inputs_digest, profile_digest, settings_digest, scope_digest`: 32-byte digests. `plan_revision, grant_generation, source_generation, erasure_epoch, pause_epoch`: monotone counters. `snapshot_ids[]`, `approved_limits_ref`, nullable `approval_id`, `created_utc, expires_utc`. |
| `audit_artifact` | `site_id, project_id, run_id, artifact_id, format_id, schema_version, source_refs, manifest_digest, ledger_id, key_scope_ref, policy_revision, grant_generation, erasure_epoch, expires_utc`. `publish_state`: staging, sealed or withdrawn. Source references bind exact generations and applicability. Format-specific sizes and multiplicity follow the owner. |
| `audit_evidence_ref` | `site_id, snapshot_id, evidence_kind, producer_id, producer_profile, source_ref, source_revision, captured_utc, completeness, reason_code, rights_ref, erasure_epoch, projection_digest`. `completeness`: complete, partial, refused, unavailable or withdrawn. No raw text or vault handle. |
| `audit_outcome` | `site_id, project_id, run_id, attempt_id, request_digest, input_generation, stage, safe_reason, progress_numerator, progress_denominator?, committed_artifact_ids[], usage_receipt_ref?, updated_utc`. Unknown total has no percentage. A completed publication binds one immutable manifest. |

Dates use UTC instants and display the selected site's zone. IDs
and typed formats use existing Boot contracts. Compare wire
digests only with identical serializer versions. Use a new
revision for corrected facts. Read the prior immutable result
while eligible or show the withdrawal reason. The UI never
substitutes a result from another site, scope, profile or date.

Pin scope, inventory and policy at planning. Authorise and reserve
cost, resource promises and storage before a dispatch. Recheck
current site, approval and fences before a network boundary,
each checkpoint, commit and delivery. Do work in 0019 scheduler
turns with 0078 admission. One run cannot hold a turn while waiting
on another. Record a durable checkpoint and reservation receipt
before releasing ownership. An uncertain external effect keeps
the original identity and the reservation until reconciled.
Publish against the pinned input and current generations in one
transaction with its outbox event, or withhold the entire
publication. Read-only repeat returns the same eligibility-checked
result and does not start another job.

### Limits, offline and custody

Run the 8 GiB supported floor, 16 GiB baseline and 32 GiB
workstation fixture. Respect the lower of each owner's measured
promise and the 0078 budget. Reserve complete concurrent peak,
including helpers, IPC, spill buffers, cache and old snapshots
until released. Start conservatively with one unit of heavy work
per site, then add only admitted independent turns. Work that
cannot meet the cap waits with its specific reason. A larger
machine may run more admitted work without changing results,
privilege or workload caps. Never consume the live engine's
protected reserve. There is no new default public-page footprint.

Register a temporary file or object before writing any protected
bytes. Use 0032, 0038 and 0118 for encryption, exact destination
and recipient approval, outbox, restore and cleanup. Register raw
source copies only where a producer expressly permits them.
Keep references to owner-supplied originals where possible.
Moving managed history to an external SSD uses protected storage.
An AWS S3 archive also requires the exact owner-approved scope
and storage mode. Selected deletion removes ledger-managed copies
and handles their keys under the owning contracts. Report residual
objects or retained owner choices and retry through the ledger.
Owner originals and uncontrolled exported copies remain outside
managed erasure. Advancing the erasure fence removes old reads
before physical cleanup completes.

| Situation | Screen copy |
|---|---|
| Offline with eligible saved facts | “Showing saved evidence from {date}.” |
| Unread, incomplete or incompatible facts | “There is too little comparable evidence for this check.” Show the field or source holding it. |
| Memory or disk hold | “This job is waiting for space.” Open Jobs to see the requested amount and cap. |
| Pause or withdrawal | “Work stopped. Saved results still need permission to open.” |
| Site or access failure | “This result is unavailable in this site.” |
| Source or rule unavailable | “This check needs a source or component that is unavailable.” |
| Cleanup incomplete | “Some managed copies still need removal.” List only the owner's safe ledger projection. |

## Screen map

The site and evidence selection stay visible at the top. A wide
window keeps navigation beside the selected view. A narrow
window uses the same focus order in one column.

~~~text
Site selector     Evidence and date     Jobs    Pause AI

Site Health       Categories and evidence coverage
                  Review suggested work > Fix List
Crawls            Sources > Runs > Select two to compare
Technical Toolbox Tool and source > Exact scope > Run or Approve
Performance Lab   Scope and profile > Metrics > Waterfall > Frames
Access logs       Import preview > Coverage > Trends and exceptions
Text Mining       Corpus > Language > Terms > Context or compare
                  Row selected > Page or link details drawer
~~~

The paid preview lives beside the free snippet example.
The charge shows in the exact plan dialog before confirmation.
The optional private name view has a separate owner action
inside an eligible source page. It clears when that view closes.

## Verification design

The suite adds no executable model or tests in this document.
Define an `AuditSuite.qnt` plan with finite sites, two projects,
two source generations, two workers, two artefacts, one job of
each kind per site and logical time. Compose the existing scheduler,
broker, budget and file-ledger transitions as interfaces. Use
bounded interleavings with nondeterministic pause, revoke, retry,
late delivery, crash, restore and cleanup failure. Model network,
filesystem and payment as external effects with separate durable
claim and acknowledgement. Unbounded liveness is outside the
finite check. Test eventual eligible turns under 0019's fairness
assumption separately from the safety properties.

| Invariant | Quint Connect Rust replay outcome |
|---|---|
| INV-0140-AU-01 Exact site and source | Cross-site ID requests, swapped grants and stale scope fail before reading rows or paths. Stored outputs carry one site and the exact eligible input. |
| INV-0140-AU-02 Guarded acquisition | Every page request has current owned route or owning-spec legal authority, declared identity, endpoint and robot decision, allowed method, public socket and live lease. Inject a forbidden hop and stop between pair legs. A paid provider POST is only the separate 0143 connector effect. |
| INV-0140-AU-03 Pause and budget | No new productive work, dispatch or result release after the relevant work fence. Existing authorised erasure, scratch cleanup, stop and receipt reconciliation may finish on their owning route. Prove that cleanup cannot restart productive work and active promises remain under the admitted caps. |
| INV-0140-AU-04 Visibility | No sealed manifest is reachable before an authorised atomic commit. Cancellation, lost reply and duplicate outbox delivery never create a second result or a mixed-generation result. |
| INV-0140-AU-05 Advisory isolation | No trace updates a draft, deploys a rule or reaches a live ranking API. Admit one exact owner-approved, recipient-sealed local export only under the existing custody and authorisation rules. Refuse changed rights, recipient, destination, raw protected output, plaintext and private names, including after revoke before publication. Prove with negative Cedar tests and dependency edges. |
| INV-0140-AU-06 Erasure | An advanced erasure fence removes read, restore and replay access immediately. Every owned artefact is tracked from allocation to deletion or named cleanup hold. A successful Delete receipt means every selected managed object and key was reconciled. |

Replay selected minimal negative traces and longer success and
failure traces through typed Rust adapters and fake time, broker,
filesystem, provider and workers. Compare operation order, state
projection, usage and fence effects at every transition. Add
synthetic mini-internet, log, locale, regional, Unicode and metric
goldens at all limits and adjacent cases. Fuzz parsing before
staging and include multiplication/size overflow, namespace,
decompression and malformed encoding. Exercise keyboard order,
status text, scope expiry and accessible graph/table equivalence.

Bazel verification on Linux and Windows covers identical result
schemas, fixture bytes, statuses, authority, stop, cleanup and
replay. Platform-specific capture measurements may differ within
an admitted method's tolerance. Run exact-platform cage, broker
and native path tests before enabling a profile. Zero real pages,
vendor calls, secrets or personal data enter fixtures. Qualify Quint Connect
separately under QR-006 and EX-06. Use the
normal release gate and the applicable receipts of each owning
spec. An unavailable component does not acquire permission by
passing mock tests.

The [Quint Connect source][quint-connect] describes Rust trace
replay and [Quint CLI documentation][quint-cli] describes the
trace format and witness fields. RQ-010, QR-006 and EX-06 still
require exact version, hash, Bazel adapter and successful
platform evidence before native proof can count. The same logical
test traces run through any separately admitted pinned back end.

The audit grid's manual local CSV uses 0143 AT-AG-04 with the
0100 export and 0118 protection evidence for the same destination
profile. Join the [local export recovery trace][audit-csv-recovery]
and keep separate ZIP and PDF admission.

[audit-csv-recovery]:
  ../planning/test-and-traceability.md#audit-grid-manual-csv-recovery

## Definition of done

Each admitted route has an exact catalogue ID, owning module,
typed evidence and limits, another route for source absence where
specified, a deterministic acceptance case, a permission and
erasure case and named platform proof. Cross-link the specs, owner
decisions and task evidence. Check the configured documentation
and link rules. Building the product starts under the existing
phase and approval gates.

[crawl]: 0141-audit-crawl-logs-and-technical-toolbox.md
[lab]: 0142-local-performance-lab.md
[grid]: 0143-audit-grid-findings-and-local-preview.md
[text]: 0144-site-corpus-and-text-mining.md
[rules]: 0145-audit-rule-and-field-catalogue.md
[quint-connect]: https://github.com/quint-co/quint-connect
[quint-cli]: https://quint.sh/docs/quint
