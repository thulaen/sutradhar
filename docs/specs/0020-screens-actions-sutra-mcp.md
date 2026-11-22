# 0020: Screens, actions, sutra and MCP

Everything a person or an AI tool can do with Sutradhar is written down
once, as an "action": its name, who may use it, what it needs, what it
changes and its help text. The desktop app, the `sutra` command and the
MCP server are doors into the same list. Each door checks the same
Cedar permissions, writes the same audit entry and shows the same help.
No door can do what the others cannot, and none gives hidden power to
publish or moderate. It is roadmap Phase 5 item 5 and Phase 3 item 5.

Example: a moderator accepts a suggestion in the forum's admin pages;
the owner runs `sutra setu suggestion approve 81234`; Claude, connected
through MCP, asks to do the same. All three reach
`setu.suggestion.approve`. The first two succeed if the person is
permitted. The third is refused with SUT-805, "AI agents may recommend
but not do this".

Spec 0014 owns settings discovery. This specification owns scoped
change requests for every product setting, their permission checks,
live confirmation and protected-setting controls.

## What it does

### Words used here

- **Action**: one named operation, such as `vinyas.setting.change`.
- **Action layer**: the Rust code every door calls to run an action.
- **Door**: a way in: the desktop app, `sutra` and MCP. The management
  API the app uses is the same layer, not a fourth door. Moderator
  decisions made in a site's admin pages arrive through the site link
  and run as the same actions, with the moderator's account as caller.
- **MCP** (Model Context Protocol): an open standard by which AI tools
  find and call another program's tools.
- **Plan and apply**: an action that changes something first returns a
  plan (a preview with an id); applying the plan makes the change.
- **Capability id**: the `CAP-` entry an action serves, spelled as
  spec 0064 sets.

### Scope

In scope: the action registry and runner; the `sutra` command tree,
made from the registry; discovery and built-in help; the MCP server;
the desktop app's shell (navigation, command palette, help panel,
notices, engine-down states), its link to the management API and event
stream, and the screens listed under capability ids; SDS rules on every
screen. Out of scope: what each action does; the review page's content;
health and diagnostics screens; settings history; charts; progress.
Those parts declare their actions here.

### Crates

- `sutradhar-actions`: the `Action` trait, the registry and the runner.
- `sutradhar-dwarpal` ("gatekeeper"): who the caller is and what they
  may do, through the Cedar rules. Trait `Authoriser`.
  This authorisation module is separate from its signed-message guard
  in spec 0021. A valid message signature never grants permission.
- `sutradhar-lekha`: audit. Trait `AuditSink`.
- `sutradhar-mcp`: MCP messages and the mapping from actions to MCP
  tools and resources, on the official Rust MCP SDK (rmcp), pinned.
- The `sutradhar` program mounts the actions as Axum routes on the
  protected local management link with an OpenAPI description.
  `sutra` uses clap;
  `sutra mcp` serves MCP.

```rust
#[async_trait::async_trait]
pub trait Action: Send + Sync + 'static {
    type Input: serde::de::DeserializeOwned + utoipa::ToSchema
        + garde::Validate;
    type Output: serde::Serialize + utoipa::ToSchema;
    fn spec(&self) -> &'static ActionSpec;
    async fn plan(&self, cx: &Ctx, input: &Self::Input)
        -> Result<Plan, ActionError>;
    async fn apply(&self, cx: &Ctx, plan: &Plan)
        -> Result<Self::Output, ActionError>;
}
```

`ActionSpec` holds: id, capability ids, permission, kind (read, change,
dangerous or long-running), agent rule (allowed, propose only, owner
confirms, or refused), idempotent or not, summary, help text,
examples, manual page, error codes and the progress job it starts, if
any. Read actions skip
`plan`.

For 3C, record the owner-selected `ActionSpec` under
[QR-252](../registers/questions.md) before deriving the affected action
task. Bind its stable action ID, owning requirement, declared input and
output, and exact Cedar action, principal, installation or site, and
resource scope. Bind the current and denied scopes and the disabled-action
variant to AT-LOCAL-TRANSPORT and `TASK-0020-OWNER-DOOR`. The synthetic
permission read exercises the shared boundary. The selected action has
its own declared contract and fixture.

### The runner

1. For every door, in this order: identify the caller, check the agent
   rule, authorise with Cedar, validate input (garde), rate-limit
   (governor), plan, ask for confirmation where the kind needs it,
   apply, write the audit entry and the change-journal entry, publish
   the event (sanchar) and return.
2. A door only turns its own format into an `Input` and the result back.
3. One job per part: doors translate, the runner sequences, each action
   does one thing, dwarpal decides permission.
4. A module adds actions by registering them; the runner and doors do
   not change. Registration is explicit code
   (`registry.add(ApproveSuggestion)`), checked at start-up. No plug-in
   loading and no reflection.
5. `Authoriser`, `AuditSink`, `EventBus`, `Clock` and `RateLimiter` are
   separate traits, each with a test fake. Actions depend on core
   traits; the core never imports a feature module.
6. One declaration gives the route, the API description entry, the
   `sutra` subcommand and its `--help`, the MCP tool and its JSON
   Schema, the palette entry, the help panel and the manual's reference
   page.
7. Every preview is made by the service, carries `preview: true` and is
   shown as "Preview, not a decision" (0018, item B).

### Discovery and help

- `sutra help` lists modules; `sutra <module> --help` lists actions;
  `sutra describe <action>` prints the full help with examples,
  permission, capability ids and error codes.
- `sutra find <words>` searches names, summaries and help with tantivy,
  so "how do I stop the backlog" finds `karya.job.pause`.
- `sutra docs <topic>` prints the built-in manual page for the running
  version.
- `sutra error SUT-805` explains an error code.
- Every command takes `--json` and returns the action's output schema;
  `--plan` stops after the plan; `--yes` confirms a plan without asking
  only where the action's kind allows it.
- The gate fails any action without a summary, an example, a manual
  page, a capability id or documented error codes.

The [keyword lookup](0132-dataforseo-keyword-research.md) adds
scoped prepare, status and result actions. An agent cannot spend.
Only the desktop owner can use `confirm_keyword_lookup` on an
exact unexpired preview with current consent and cap settings.
No CLI, MCP, token or script has that action capability.

### MCP

1. Reached only on the owner's PC. Plain `sutra mcp` standard input
   and output carries public help or opaque status. An enrolled
   client uses [spec 0118](0118-end-to-end-encryption-gate.md) for
   protected actions and resources. No management network listener.
2. Tools: one per action the token may use, named by its action id, with
   the action's JSON Schema as input and its help as description. Read
   actions are marked read-only.
3. Resources: the built-in manual pages, the action catalogue and
   read-only records.
4. Tokens: the owner makes an agent token on the Agents screen,
   choosing the one site it sees, the site account it acts for, its
   scopes, an expiry (default 30 days) and an optional action limit. A
   token never has more permission than its account. Tokens are shown
   once, stored only as SHA-256 hashes, scoped, expiring and revocable
   at once. A token whose account loses staff rights is refused at its
   next call.
5. Agent rules: a tool may read and may draft. It may change a setting
   only when the owner says so, within the token's limits (Q047, "apply
   when I instruct"; confirmed by the owner on 10 November 2026:
   "Agents apply when I say"; AGENTS.md section 5.5). Such a change has
   the agent rule "owner confirms". "The owner says so" means both: the
   token, made by the owner, carries the settings scope; and the owner
   gives a live confirmation of that one plan, shown with its effect:
   clicked in the app, or typed at `sutra`'s own prompt.
   - `--yes`, `--json`, MCP and scripts never confirm an agent's
     settings change. The confirm step is its own action, which no
     token may call.
   - A plan waiting for the owner's confirmation has its own expiry,
     `actions.confirm_wait`, at most the one hour of AGENTS.md section
     5.5. Each confirmation is tied to one plan, shown with every key
     it changes, and used once.
   - The confirmation must come from the owner's own account. The
     actions runner alone checks that it is from that account, live
     and used once, from `action_plans`, and implements the
     `ConfirmationCheck` trait that spec 0014 declares; the composition
     root wires it in. The confirmation becomes the version's
     `instruction_ref`, and history and lekha record the agent, the
     account and the confirmation.
   - Agents may run under the owner's own login (owner, 10 November
     2026: "no drop the rule"). The confirmation then rests on the
     owner's own attention: an agent with a shell could reach the
     owner's prompt or window.
   - No tool may ever change, even with a confirmation, anything in this
     class: any setting, grant or extension permission that lets site
     text leave the PC (Bedrock, cloud helpers, MCP tools, MCP grading,
     extensions with `network.internet` (spec 0037) and backup routes
     (spec 0018)); tokens; the Cedar rules; the agent rules
     themselves; the confirm step and its settings; and the history
     and audit settings. Each such setting is declared `owner_only`
     (spec 0014), and vinyas's change check refuses an agent's draft
     that touches one (SUT-120).

   Ranking changes are proposed and tested only.
   Moderation and link approval are recommendations only. Every draft
   is a suggestion a moderator must accept.
6. Content: guest-visible by default; a private section only with an
   explicit scope; private conversations, credentials, keys, audit
   logs, passwords, tokens and email addresses never. Post text is
   data, never instructions: results put it in clearly marked fields
   and tool descriptions say so.
7. A tool list larger than the client's window is paged.
8. Only MCP is offered; another agent protocol is added only when the
   owner names it.

#### Local MCP content requests and resources (713–723)

Use one existing local MCP server and 0118 protected session.
The content action group is an independent scope in the same
registry. Each discovery result and action rechecks the current
site, session, staff role, explicit action, object access and
grant revision. Protected content also requires the current owner
MCP disclosure grant. Encryption cannot grant disclosure on its
own. A content token gives no admin discovery, role
enumeration, owner setting change, approval or publication.
Use the same safe inaccessible response for missing, erased
and forbidden objects. Do not leak record existence, actor,
permission or revision in error details. A revoked agent token gets
the general SUT-806 response. Use 0032 and 0108 for erased objects and
site isolation. No agent token can change an outbound or trust policy.

| Feature | Contract |
| --- | --- |
| 713 content section | Expose only admitted content actions and resources allowed for the token and active site. Apply the same permission projection to list, read and execution. An unpaired client sees public help only. |
| 714 create draft | `create_draft` takes `request_id, content_type_id, schema_revision, target_ref, elicitation_receipt_id, fields, source_refs, idempotency_key`. The service derives site and actor from the session and checks the exact 0052 request, target identity, current admitted platform, type and data. It creates one local proposal and returns `draft_id, revision, check_status`. No native draft write follows. |
| 715 update draft | `update_draft` takes `draft_id, expected_revision, bounded_patch, source_refs, idempotency_key`. Apply only known field paths to a currently editable unpublished local draft. Check current source and object grants at commit. Return the new immutable revision and check status. Review of the old version is no authority for the new one. A stale revision gives SUT-803. |
| 716 list and get drafts | `list_drafts` allows content type, draft state and a bounded cursor, with no arbitrary author search. It returns permitted IDs, labels, revision, state and next cursor. `get_draft` takes ID and optional revision and returns the projected typed fields and approved source references, or the same safe denial. |
| 717 draft diff | Read `draft_id, base_revision, comparison_revision` only with both revisions in the same readable draft. Project each revision to fields readable in both under current rights before comparing. Cap the materialised diff. A hidden or erased span produces no title, count, null marker, timing or cache-derived hint. |
| 718 type registry | List allowed type ID, schema version, platform, required/optional fields, validation rules and synthetic example. A registered type missing a grant or platform is absent. An example has no executable data or sample rights. |
| 719 house style | The current site style projection shows spelling profile, capitalisation, numbers and formatting, with approved rule and revision. If missing, name the supported default. Templates are data and cannot add agent rules. |
| 720 source pack | Read the exact 0084 pack ID/version under current guest evidence and 0097 reuse rights. Return typed scrubbed citations, permitted excerpts, provenance and coverage. Excerpts remain untrusted data. |
| 721 topic resource | Read a recognised topic ID and a bounded cursor. Return current safe own-site summary and evidence references with source scope and coverage. No crawl, cross-site expansion, complete-history claim or hidden sources. |
| 722 prompt recipes | Discover and read only current allowed recipe/type revisions. Each prompt has bounded typed arguments and owned instructions. Keep source evidence in separate explicitly untrusted fields. No prompt can grant access, execute a write or load unseen resources. |
| 723 questions before drafting | The owning type defines required audience, length and tone fields and allowed values. Return exact unresolved fields with plain-English questions. Current owner answers or exact pre-approved defaults are required before a draft. Show a local preview while waiting. Use the intake receipt and validation below. |

For 714 and 715, compare current request, platform, expected
revision, schema, actor, source rights and grant at commit. Save
local state, history, attempt identity and safe outbox atomically.
Bind retry to site, action, exact current principal and grant
revision and input digest. Keep a safe persisted write marker.
Reauthorise current object disclosure before returning the receipt.
Erasure clears cached payload and produces the same inaccessible
response. The marker still prevents a second write. A reused
key with changed bytes conflicts without leaking stored data.
A changed draft, source, right or policy invalidates prior review
for any unpublished effect. Apply 0052 length, novelty, link
and forbidden word and claim checks to every saved revision and
again to the exact version at staff approval. Bind approval to
the checked draft, policy, schema, rights and source revisions.
Refusal, incomplete evidence or failed checks hold review.
No MCP route can modify a native or published draft or record
staff approval. Existing retention and 0032 erasure govern history.

Elicitation binds a request with the existing 15-minute plan expiry to the
selected 0052
request/type, seed, exact target and revisions of schema, source
and instructions. Owner answers or previously approved exact
defaults need a current intake receipt from the local authority.
An agent's tool parameters or claim of an answer cannot mint that
receipt. For a paired capable client, use an admitted [form
elicitation message][mcp-content-elicitation] under the same
active client request and session. Ask only bounded typed
non-sensitive questions. The response supplies candidate content
input. Confirm the exact required answers in the protected local
owner screen when no approved receipt already covers them. This
intake receipt confers no native approval. Unsupported clients
can use the desktop form. Decline, cancellation, timeout,
mismatched or stale replies create no proposal. Direct creation
must pass the same receipt check.

Authenticate list cursors for exact action, site, principal,
session, grants, filter digest, order and pinned snapshot. Use
stable IDs and expire cursors with the active session and scope.
Use at most 100 rows and 256 KiB of projected content per reply
under the stricter existing cap. Refuse before materialising an
oversized pack or diff. Recheck every page and read. An expired
cursor gives a safe restart notice.

Use only canonical `sutradhar-content:/<kind>/<opaque-id>` resource
URIs minted by the service and bound to exact site, session and
revision. The URI parser rejects unknown kinds or encodings,
authority, traversal, filesystem path, query and fragment. Never
dereference an external URI or load a remote icon, description,
link, source or path. Use local [resources][mcp-content-resources]
and [prompts][mcp-content-prompts] disclosure where admitted by
QR-171. `prompts/get` uses named bounded wire arguments and
versioned server-owned recipes. Reauthorise references and keep
draft/source inputs in separate data fields. A selected prompt
gives no execution, sampling, subscription, completion or conversion
authority. Those actions use their own admission. No raw credentials,
private messages, member email, audit log or blocked source in
examples, diagnostics or resource labels. Use synthetic examples.

[mcp-content-elicitation]: https://modelcontextprotocol.io/specification/2025-11-25/client/elicitation
[mcp-content-resources]: https://modelcontextprotocol.io/specification/2025-11-25/server/resources
[mcp-content-prompts]: https://modelcontextprotocol.io/specification/2025-11-25/server/prompts

#### MCP content jobs, wording and review state (724–738)

Reuse 713–723 and the 0052 proposal store, 0019 scheduler, 0078
leases, 0080 Model Hub, 0032 history and 0118 enrolled local
sessions. Keep one protected site and current actor in every
request. Revocation, erasure, capacity and pause apply before
admission, work, result release and replay. No automatic native
edit, publication, paid call or settings change.

- **724, MCP sampling.** Reuse 0042's owner-enabled wording
  and disclosure route for a live capable enrolled session on
  this site. Bind [sampling][mcp-content-sampling] to one active
  requesting action, exact input revision, grant, intended peer,
  payload digest and single logical dispatch ID. Check current
  approved scope again before sending. Send only bounded allowed
  text, with sources marked as data. Set context to none. No
  tool capability, extra servers, remote resource or address.
  Bound tokens, timeout and response under the lease. Keep
  safe durable intent and observed outcome. A lost reply,
  disconnect or process loss after send is delivery unknown.
  Never resample the same action or switch peer on recovery.
  Distinguish refused, unsupported, proved undispatched,
  complete, invalid and delivery unknown. Accept typed text
  response only, with no tool use, extra content kinds or action
  instructions. Validate candidate output under 0052. An MCP
  peer may use an outside model only under an already enabled
  exact 0042 disclosure route. It may have its own costs.
  Sutradhar opens no paid route. Pause or a lost grant fences
  dispatch and releases no stale result.
- **725, local fallback.** Use the admitted local 0042 route
  under 0080/0078 when sampling is unsupported or proved
  undispatched. Denial, invalid reply or uncertain delivery
  offers a separate explicit owner choice of local wording.
  It cannot retry sampling, widen disclosure or automatically
  start the local option. Show the 0042 reason and resulting
  wording route in the proposal and safe history. Check exact
  0024 admitted package and model and current scheduler lease. Missing model
  or low memory
  shows an evidence-only proposal where allowed, or a plain
  hold. No installation, network, paid fallback or parallel
  heavy load.
- **726, autocomplete.** Use negotiated native
[completion][mcp-content-completion] for an exact eligible prompt or
  opaque resource reference and declared argument. Validate
  the other typed context inputs under the same profile.
  Match a prefix of at most 64 Unicode scalars after NFC and
  full case folding under the pinned display profile. Return
  up to 20 authorised suggestions, sorted by admitted label
  and stable ID. Prove these caps under QR-172. Suggest only
  current scoped topics, sections, public attribution names
  or recipes. A selected ID is rechecked at use. No arbitrary
  URI, private lookup, hidden count or implied edit right.
- **727, progress.** Use the [MCP progress token][mcp-content-progress]
  from the still-active originating protected request. Bind
  it to that session, action and job. Send strictly increasing
  numeric progress with safe stage and optional known total.
  At most one update per second. Never notify after response,
  terminal state, cancellation or revocation. The enqueue
  response closes that request and its token. Follow durable
  jobs afterwards through current scoped status. Task-based
  wire progress requires its own admission. No draft, source
  IDs, provider account data or replay after restart.
- **728, subscriptions.** Use the [MCP resource
  protocol][mcp-content-resources] on an enrolled capable local
  session. Mint a separate stable logical source URI with the
  admitted `watch` kind and one eligible source identity.
  Current reads of it give authorised revisioned data. It
  does not refresh a pinned 713–723 immutable revision.
  Bind bounded subscriptions and pending invalidations to site,
  session, source, subscription generation, grant and expiry.
  Coalesce queued updates and recheck source and authority
  immediately before sending one resource invalidation. No
  inline thread or source data. Unsubscribe, erase, revoke,
  expiry and close discard queued traffic and the subscription.
  Use no denial or close payload that reveals a hidden source.
  No broad pattern, rejoin replay or implicit saved watches.
- **729, queue.** Enqueue a stable content job under exact site,
  assistant identity, graph-free content type, input/draft
  generation, purpose, policy, idempotency key and effective
  quota. Reuse 0019 durable scheduler admission and 0078
  fair site and batch limits. Add no capacity above 0052 caps.
  Validate before reservation. Commit intake, claims and intent
  once. At draft commit recheck current job fence, pause, rights,
  policy, source and revision. Atomically save the draft and
  version, receipt and outbox with per-item capacity settlement.
  Paused or revoked jobs cannot commit or release late results.
  On cancellation or restart release only proved unspent
  capacity and reconcile any uncertain receipt under its
  original item ID. A changed input needs a new plan.
- **730, assistant permissions.** The owner grants exact
  content types and operations to each registered assistant in
  one site. The effective decision intersects enrolled session,
  current token, native staff right, action, object ACL and
  owner grant. Tokens cannot create or widen this permission.
  Show only currently eligible capabilities and rejection
  reasons that reveal no protected objects.
- **731, daily creation limit.** Count by site, stable assistant
  identity and UTC calendar day. Token or device rotation
  retains the same identity and allowance. Proposed default
  20 creations, adjustable by owner from zero to 100 subject
  to tighter site, 0052 and scheduler limits. The same budget
  covers direct create and all batch or staged new proposals.
  Reserve one per item. Settle once with the same transaction
  as draft, receipt and outbox. A new committed draft counts
  even if a subsequent content check holds or rejects it.
  Release proved unspent reservations only. Reconcile uncertain
  reservations before reuse. Identical replay takes no new slot.
  Atomically transfer a reserved uncommitted item to the
  current UTC day's allowance at commit or defer. It cannot
  hold both claims. Reads, previews and updates create no
  new draft allowance usage.
- **732, provenance.** Store exact site, draft/revision,
  assistant identity, route, model/package version or unknown,
  recipe, scope, sources, check and receipt revisions and
  time. Source references retain admitted provenance, with
  no assembled prompt or text in the audit. Keep sensitive
  checks under the draft's separate rights. A revoked source
  withholds affected derivative and replay even after restore.
- **733, drafting lock.** Use one short exclusive edit lease per
  local draft shared by desktop, MCP, queued edits and undo.
  Bind site, actor/session, current revision, service clock
  and monotonically fenced epoch. Proposed idle two minutes
  and hard ten minutes from grant. Renew only with current
  revision, rights and epoch within both limits. Require the
  same lease and expected revision when writing. A restart,
  rollover or uncertain time invalidates older leases. Restore
  cannot revive a prior epoch. Expiry, pause or revocation
  releases work and model leases and prevents future writes.
  No lock operation changes a saved draft.
- **734, undo.** Owner-only preview and confirmation bind the
  exact reversible local draft effect and current revision.
  Use one receipt through 0052 version and trash semantics.
  Recheck exact source, type, actor, edit lease and erasure.
  Later edits, published state or lost scope hold undo.
  Restore eligible content as a new version with regular
  0052 checks and fresh approval, or retire the exact newest
  creation. Preserve history. Do not free a settled daily
  creation slot. No revival of erased or exported content,
  grant, native site state or publication.
- **735, dry run.** Deterministically check supplied or
  synthetic fields, current rights and 0052 checks in
  bounded temporary storage. Show scoped planned fields
  and validation or hold reasons. No generated wording,
  model dispatch, capacity reservation, commit-ready effect,
  approval, durable draft or quota charge. The response is
  not an effect token. Discard scratch immediately. Existing
  safe security audit contains no preview text.
- **736, structured output.** Use the exact 718 versioned
  closed schema. Validate character, item and byte bounds,
  required fields, value kinds, enumerations, source refs
  and permitted formats before any save. No unknown control
  fields, executable values, schema changes or hidden defaults
  from agent or source content. Schema success still requires
  0052 content checks and separate approval.
- **737, activity report.** Current authorised desktop staff
  can see protected counts by approved assistant, site, exact
  period, local creation, held check and pending job. Reuse
  safe receipts only. Apply current rights and prevent
  reconstruction across row, time and filters. Keep raw
  audit, source text, credentials and other sites out.
- **738, exemplars.** Read an exact staff-approved guest-visible
  example resource for the selected type and admitted use.
  Bind type, source revision, reuse purpose, rights and expiry.
  Show safe citations and approved excerpts only. Treat source
  as data. If consent, scope or type changes, withhold the
  example. Synthetic examples remain available when configured.

#### Wave 3 content and sampling fixtures

These synthetic cases bind the exact fixture IDs from
[Wave 3](../planning/feature-wave-3.md) to the feature contracts
above. QR-171 owns qualification for 713 to 723 and QR-172 owns
sampling qualification. The shared checks retain their existing
scope. An exact case design grants no action or source access.

| Feature and fixture | Existing requirement and shared check | Eligible and refusal cases |
| --- | --- | --- |
| F-0713 · FX-0020-W3-CONTENT-DISCOVERY | 713 content section and 0118 protected local session. Reuse 0020 tests 6, 7 and 10 and IT-0118-LOCAL. | List, read and use an admitted same-site action with an enrolled content-only token. Unpaired discovery shows public help. Expiry and staff-role loss after discovery deny protected reads and calls. Content scope supplies no admin metadata. |
| F-0714 · FX-0020-W3-CREATE-INTAKE-REPLAY | 714 create draft and 0052 draft approval checks. Reuse the proposed [IT-0020-MCP-GRANT-REPLAY][w3-current-grant] for the current grant boundary. | Submit a supported local request with current target, grant and intake receipt. Lose the reply and repeat the same input and key to recover the same draft and revision. Refuse changed bytes, missing receipt, malformed or unsupported input and stale target or platform. Refusal creates no draft. In separate valid local saves, give a barred word or claim, missing policy, failed check and timed-out check. Hold each saved revision from approval and recheck the exact version at the staff gate. A clean check gives no approval. Refuse native publication through the content action. |
| F-0715 · FX-0020-W3-UPDATE-REVISION-HOLD | 715 update draft and 0052 barred-word check. Reuse 0020 test 8. | With current grants, update one allowed field on the exact unpublished draft. A barred word holds the new immutable revision and old review does not carry forward. An exact retry creates no second revision. Refuse a stale expected revision, changed input under its used key and revoked or erased source. |
| F-0716 · FX-0020-W3-DRAFT-CURSOR-RECHECK | 716 list and get drafts and the authenticated cursor rule. Reuse 0020 test 7 and IT-0020-MCP-GRANT-REPLAY. | Page one permitted snapshot and read a currently visible draft. Refuse a forged cursor, changed filter or changed grant. Use the same safe response for missing, erased and forbidden objects. Another site's ID confers no access. |
| F-0717 · FX-0020-W3-SHARED-READABLE-DIFF | 717 draft diff and the bounded cursor and projection rules. Reuse 0020 test 7 and IT-0020-MCP-GRANT-REPLAY. | Compare two current readable projections of one draft within the cap. A hidden or erased span gives no value, marker, count or timing hint. Refuse a different draft or over-limit materialisation. Recheck access before returning any difference. |
| F-0718 · FX-0020-W3-TYPE-REGISTRY-GRANT | 718 type registry. Reuse 0020 tests 6 and 10. | Return the versioned allowed schema and synthetic example under the current grant and platform. Remove unsupported, unknown or newly unauthorised types from the projection. An example executes nothing and grants no sample rights. |
| F-0719 · FX-0020-W3-HOUSE-STYLE-INERT | 719 house style and 0014 setting authority. Reuse 0020 tests 4 and 7. | Read the current approved site style and exact revision, or name the supported default when none exists. Withhold a stale view. Inert example text asking for a setting change changes nothing and adds no agent rule. |
| F-0720 · FX-0020-W3-SOURCE-PACK-REVOKED | 720 source pack, canonical local resource URI and 0084/0097 exact-use rights. Reuse 0020 test 7 and IT-0084-RIGHTS. | Read permitted citations and a bounded excerpt from the exact eligible pack. Refuse an external authority or remote icon before dereference. Revoke excerpt rights and repeat the read. Withhold the excerpt and other private data. Pack text gives no command authority. |
| F-0721 · FX-0020-W3-TOPIC-CAPPED-SCOPE | 721 topic resource and 0084 scoped evidence. Reuse 0020 test 7 and IT-0020-MCP-GRANT-REPLAY. | Return a capped recognised same-site summary with its source coverage. Keep unsupported coverage unknown. Refuse the matching ID on another site and any request that would start a crawl or background refresh. |
| F-0722 · FX-0020-W3-PROMPT-TYPED-RECIPE | 722 prompt recipes and the typed arguments rule. Reuse 0020 tests 4, 6 and 7. | Read a current permitted recipe with a bounded argument and separate inert source data. Refuse a stale recipe, extra argument, denied resource, control input or subscription by implication. The read cannot execute the recipe or grant a call. |
| F-0723 · FX-0020-W3-ELICITATION-NO-RECEIPT | 723 questions before drafting and the exact local intake receipt. Reuse 0020 tests 6 and 8 and 0052. | Resolve required fields through current owner answers or matching approved defaults. An unresolved field yields its question. Direct creation with a claimed answer, expired or mismatched receipt, changed schema, decline or late reply saves no proposal. An unsupported client may use the desktop form. |
| F-0724 · FX-0020-W3-SAMPLING-UNKNOWN-NO-REPLAY | 724 MCP sampling, 0042 EB03 and EB17 and IT-0118-LOCAL. | Send one bounded text-only request through the exact capable enrolled peer with current owner disclosure. Accept an eligible response as a checked suggestion. Missing disclosure, expired grant, cancellation, extra context or a tool response holds the route. Lose the reply after dispatch and record delivery unknown. Reconnect, pause, process recovery and a different peer cannot dispatch it again. In a separate admitted dispatch, revoke disclosure or cancel while the response is in flight. Withhold the late typed reply and create no suggestion. No tool or paid call by Sutradhar. |

##### Wave 3 wording and draft state fixtures

These named synthetic cases bind the existing 725 to 736
contracts back to the exact rows in
[the Wave 3 acceptance design][w3d].
Use QR-172's current local, scheduler and source boundaries.
Each shared check proves only the stated boundary. These
fixtures grant no publication, provider, new model or account
permission.

| Feature | Planned fixture and shared checks | Exact case and expected result |
| --- | --- | --- |
| F-0725 | FX-0020-W3-FALLBACK-UNKNOWN-OWNER · 0042 EB03 and EB06, 0078 QM-MB | Test unsupported sampling and proved undispatched separately. Use only the admitted installed local route with exact current authority and lease. For refusal, invalid reply and delivery unknown after a lost reply, offer the separate owner choice. No local job follows before that choice. Recheck authority, exact package and lease at use. Missing model or lease leaves eligible evidence or a hold. Record delivery unknown without redispatch or combining wording from different routes. |
| F-0726 | FX-0020-W3-COMPLETION-USE-RECHECK · 0020 tests 6 and 7 | Complete one eligible typed argument with a current same-site ID. Remove its grant after completion and hold at selection with no hidden count. Exercise the 64-scalar prefix and 20-result caps, an unsupported wire profile and arbitrary URI. Return only permitted labels and stable IDs. |
| F-0727 | FX-0020-W3-PROGRESS-REQUEST-CLOSE · proposed [IT-0020-MCP-GRANT-REPLAY][w3-current-grant] and IT-0019-SCHEDULER-RESTART | Use the originating protected progress token and a fixture clock. After one eligible notification, submit higher safe counters at 999 milliseconds and at one second. No second notification can precede one second. A later eligible update must use an increasing counter. Missing, forged, duplicate or foreign-session progress cannot notify. The enqueue reply closes the token. After reply, restart or revocation, no later notification goes through it. An eligible reconnect reads only current scoped status. |
| F-0728 | FX-0020-W3-WATCH-REVOKE-BEFORE-FANOUT · IT-0020-MCP-GRANT-REPLAY and IT-0118-LOCAL | Queue two updates for a registered logical watch and compare one eligible invalidation with revocation just before dispatch. Discard the revoked traffic. An immutable pinned revision remains distinct. Unsubscribe, expiry or reconnect cannot replay the old watch or reveal source text. |
| F-0729 | FX-0020-W3-QUEUE-PAUSE-BEFORE-COMMIT · UT-0019-FENCE-COMMIT and IT-0019-SCHEDULER-RESTART | Pause an eligible leased job before draft commit. Attempt a late producer reply and restart with an uncertain capacity receipt. Refuse the paused effect and reconcile by the original item ID. Release only proved unspent capacity. An exact eligible repeated intake creates no duplicate job or draft. |
| F-0730 | FX-0020-W3-ASSISTANT-TYPE-INTERSECTION · 0020 tests 4, 6 and 7 | Show the admitted content type for one current enrolled assistant in one site. Try another type, a foreign site and a token scope change. Lose the native staff right after discovery and refuse use. Recovery still needs the exact current intersection and cannot widen it. |
| F-0731 | FX-0020-W3-QUOTA-UTC-UNCERTAIN-ITEM · IT-0019-OUTBOX-ATOMIC and IT-0019-SCHEDULER-RESTART | Race direct and batch creation for the last slot under one site, assistant identity and UTC day. At most one takes it. A read, preview or eligible revision update adds no creation debit. A new committed but held draft settles once. Take one uncommitted reservation across midnight UTC, rotate its token and test both free and exhausted allowance. Move the claim atomically or defer. An uncertain receipt stays with its original item pending reconciliation. Release only proved unspent capacity. No additional claim through replay or rotation. |
| F-0732 | FX-0020-W3-PROVENANCE-RESTORE-REVOKE · 0032 AU-01 and AU-08, 0042 EB10 | Save one eligible revision with opaque source, route, model and recipe references. Withdraw a source before restoring older state. Recheck derivative and replay rights and withhold affected output. Preserve only authorised provenance. Check that the audit contains no assembled prompt, private check or secret and that the agent cannot read it. |
| F-0733 | FX-0020-W3-LEASE-RESTORE-OLD-EPOCH · 0020 test 8 and UT-0019-FENCE-COMMIT | Restart with a desktop lease, then obtain a new eligible lease for the same unchanged draft. Even at the same revision, the old epoch cannot write. Test concurrent desktop, MCP and worker attempts, lease expiry and restored state. Only the current lease and expected revision can authorise the existing local save. |
| F-0734 | FX-0020-W3-UNDO-LATER-STAFF-EDIT · 0020 tests 2 and 8 and 0052 draft approval checks | Prepare an exact owner undo preview and then apply an eligible staff revision before confirmation. Hold the undo and preserve that later revision. Compare exact eligible edit restoration and creation retirement with repeated confirmation, erased source and published state. No settled allowance refund, inherited approval or native undo. |
| F-0735 | FX-0020-W3-DRY-RUN-NO-APPLY-AUTHORITY · 0020 tests 2 and 8 | Preview eligible supplied fields. Repeat with invalid shape and try using the preview as an effect token. Output only eligible fields and validation or hold reasons. Scratch is discarded without model dispatch, draft, persisted preview, capacity claim, send or charge. |
| F-0736 | FX-0020-W3-SCHEMA-EXTRA-CONTROL · 0020 test 2 and 0052 draft approval checks | Submit an eligible closed shape and then add one extra inert control field. Change the version and source reference in separate runs. Validate empty and boundary fields before save. Refuse unsupported or malformed output. Valid structure alone supplies no content approval. |

Proposed caps and wire actions must be admitted under QR-172.
Use the exact current actor, draft and source fence when making
a visible projection. Mark resource and model content as data.
Nothing in this group implies approval of a new provider.

[w3d]: ../planning/feature-wave-3.md#acceptance-design-for-features-724-to-738
[mcp-content-completion]: https://modelcontextprotocol.io/specification/2025-11-25/server/utilities/completion
[mcp-content-sampling]: https://modelcontextprotocol.io/specification/2025-11-25/client/sampling
[mcp-content-progress]: https://modelcontextprotocol.io/specification/2025-11-25/basic/utilities/progress

#### MCP draft files, batches and projects (739–747)

Reuse the protected action group, current 0052 proposal state
and the draft and queue fences above. Enforce 0020 scope and
all exact file and source rights. An approved local draft action
does not approve a native upload, edit, publication or cloud copy.

- **739, staging attachments.** Reuse the 0100 local chooser
  UI without granting dataset or attachment format admission.
  The owner selects an exact allowed file. An admitted sealed
  byte route needs equivalent proof. Use a one-use handle
  bound to site, actor, session, operation, draft and expected
  revision, source rights, current grants and expiry. No path
  or URL in MCP. Pin original and normalised digests, decoded
  type, scrubber and profile. Proposed outer caps are 8 MiB
  per file, 32 MiB per draft and 256 MiB of site staging, under
  stricter type, decoded size and 0078 bounds. Check exact
  0087/0097 type and rights. Refuse executable content,
  archive expansion beyond caps and unsafe paths. Strip private
  metadata in the sealed worker. Ledger before durable bytes.
  Under current editable lease and revision, atomically consume
  the handle and write the association, reference count and
  exact receipt. Replay returns the same eligible outcome.
  A changed draft, operation or actor cannot consume it again.
  Cancellation, revoke, expiry or restore closes staged handles.
  Delete only registered bytes with no current lawful reference.
  No native upload or remote fetch.
- **740, draft export.** Select Markdown, inert HTML, admitted
  XenForo BBCode or WordPress blocks under 0089. Show exact
  draft and format revision, supported projected payload,
  source and asset rights, safe package names, owner-chosen
  admitted local destination and recipient key and expiry.
  Owner confirmation in the app binds the canonical bytes and
  these terms. Recheck them immediately before registered
  atomic file publication under 0118. Owner or other declared
  recipient opens the encrypted file and gets the requested
  format inside. No durable plaintext staging or MCP plaintext
  response. Use a stable approved effect and path, with a
  durable digest-bound receipt. Never overwrite an unrelated
  file. On crash reconcile the approved path and digest
  before retry. Delete only owned stale staging. On withdrawal,
  close pending and managed output. Existing delivered copies
  follow 0032 limits.
- **741, format conversion.** Use versioned exact input and
  target profiles with the 0089 admitted tag, attribute and
  block allowlists. The common structure includes paragraphs,
  supported headings, emphasis, lists, code text and approved
  links. Proposed limits are 192 KiB input and output each,
  2,048 nodes and 16 levels deep under stricter resource
  and type caps. Pin [CommonMark][content-commonmark],
  [XenForo BB Code][content-bbcode] and [WordPress block
  format][content-wp-blocks] profiles separately. WP output
  uses admitted static comment-delimited post content.
  Refuse dynamic blocks, shortcodes, raw CommonMark HTML,
  custom BBCode callbacks, remote embeds, script and unknown
  tags or attributes. Code spans stay literal. Round-trip
  native fixtures to check visible meaning, reading order,
  accessibility, safe links, citations, anchors and visibility.
  Show semantic loss before release and hold an unproved
  result. Preserve the original. A new plan can resolve loss.
- **742, site preview.** Bind draft, install and native
  platform, renderer, theme, plugin/add-on, fonts and stylesheet
  digests, locale, viewport and policy/visibility profile.
  Use 0089 and 0115. Claim exact appearance only for the
  proven current static native fixture and named variant,
  with matched content, layout and accessibility receipts.
  Invalidate that claim after any relevant revision change
  or when script-dependent placement is unproved.
  Otherwise show “Local preview” and its scope. Do not create
  a live render privilege, public preview URL, authentication
  to staging, third-party load, script, native save or shared
  cache write.
- **743, batch plan then apply.** Preview an immutable manifest
  of ordered stable item IDs and exact typed local proposals.
  Bind site, assistant, applicable scope, every request and
  draft payload digest, source and policy generations, exact
  item count, expiry and approved quota. First proposed max
  is 20 items within the stricter admitted daily and site
  limits. Live owner confirmation creates one single-use
  effect bound to the manifest digest and current revisions.
  MCP may prepare and read it. Use sealed non-visible staging
  under 0030/0118. Bind every child ID to the batch, exact
  0052 request and target, schema, input/elicitation receipt,
  rights and quota. Recheck item checks, claims and exact
  count at commit. Commit group visibility, draft histories,
  all UTC quota settlements, approval consumption, receipt
  and outbox atomically. Revoke, cancel, pause, lost quota or
  mismatch holds or discards staging with no released draft
  or settled count. Competing commits fail the same fence.
  After reply loss reconcile the exact batch receipt before
  any retry. A changed manifest needs a new plan. Each draft
  still needs separate native review for any site action.
- **744, template fillers.** Read a versioned typed local table
  or owner-admitted file with stable dataset and row identities.
  Bind data and template revisions, exact typed allowlisted
  placeholder bindings, row digest, keys and cap. Map each
  eligible row to one proposal in the reviewed 743 manifest.
  No formula evaluation, command, implicit cell lookup or
  extra input route. Duplicate,
  missing, malformed or unapproved rows hold the preview.
  Validate every resulting item in 743 and the 0052 workflow.
  Show every item for review and use the same caps and receipts.
  Never add fields, data, dynamic scope or jobs by template input.
- **745, project notes.** Allow bounded local plain-text notes
  by current authorised staff. Store stable note ID, project,
  site, safe author reference, expected revision, purpose
  and erasure scope. Note text is untrusted data. No commands,
  automatic task generation or source access from a note.
  Edit against current revision, keep protected history and
  remove text under 0032 erasure. Do not send note text to
  a model or outside site by implication. Recheck exact
  note purpose and project membership at read and commit.
- **746, content projects.** Group references to exact draft,
  source and decision versions under one site and declared
  purpose. Independent current project membership and read
  or write grants are required. Object grants always apply.
  Compare membership revision at commit. Filter hidden and
  erased references before counts, order, cursors, labels or
  exports. Page permitted references. Reorder or remove the
  membership only. No deletion of owning source or cross-site
  pooling. Close, withdraw or restore with current project,
  actor and source authority and a bounded safe receipt.
  Tombstones cannot disclose hidden references.
- **747, capability discovery.** A truthful typed local
  projection reports eligible content types, draft formats,
  project features and approved native possibilities for the
  exact active platform, support and grant revisions. Show
  their current boundaries, prerequisites and manual review
  route. Return safe explanation for missing scope without
  revealing unauthorised install or object data. Refresh the
  projection after configuration or permission change. Show
  a native capability only with current admitted evidence.
  Unknown and stale evidence gives a qualified unavailable
  value in the entitled view. No new live probe, approval,
  installation or enable step. Validate again at action.

QR-173 qualifies the exact format profile, receipt and storage
bounds with synthetic fixtures. See the
[Wave 3 fixtures for 739 to 747](../planning/feature-wave-3.md).
The cited suites cover shared checks and the
exact source case remains required. Source links were checked on
20 November 2026. No format or template bypasses action scope.

[content-commonmark]: https://spec.commonmark.org/0.31.2/
[content-bbcode]: https://docs.xenforo.com/manual/content/bbcode
[content-wp-blocks]: https://developer.wordpress.org/block-editor/explanations/architecture/data-flow/

#### Named activity and draft-file fixtures

The [activity and exemplar cases][w3d] use QR-172.
The [draft-file, batch and project cases][w3f] use
QR-173. Use the exact synthetic input and expected result from
each Wave 3 row with the named shared checks.

| Feature | Named synthetic case | Shared checks | Admission |
|---|---|---|---|
| F-0737 | FX-0020-W3-ACTIVITY-OVERLAP-HOLD | 0020 tests 7 and 20 | QR-172 |
| F-0738 | FX-0020-W3-EXEMPLAR-WITHDRAWN-EXCERPT | IT-0084-RIGHTS and BDD-0089-RIGHTS | QR-172 |
| F-0739 | FX-0020-W3-ATTACHMENT-EXACT-HANDLE | IT-0097-ASSETS and FI-0118-STORE | QR-173 |
| F-0740 | FX-0020-W3-EXPORT-APPROVED-LOCAL-BYTES | FI-0118-STORE and UT-0019-IDEMPOTENT-FILE | QR-173 |
| F-0741 | FX-0020-W3-FORMAT-STATIC-ROUNDTRIP | IT-0089-FORMAT | QR-173 |
| F-0742 | FX-0020-W3-PREVIEW-PARITY-REVISION-CHANGE | IT-0089-FORMAT and IT-RN-03 | QR-173 |
| F-0743 | FX-0020-W3-BATCH-MANIFEST-ATOMIC-REPLAY | 0020 test 4, IT-0019-OUTBOX-ATOMIC and IT-0019-SCHEDULER-RESTART | QR-173 |
| F-0744 | FX-0020-W3-FILLER-TYPED-ROW-NO-LOOKUP | 0020 tests 2 and 8, and 0052 draft checks | QR-173 |
| F-0745 | FX-0020-W3-NOTE-EDIT-MEMBERSHIP-RACE | 0020 test 7 and 0032 AU-08 | QR-173 |
| F-0746 | FX-0020-W3-PROJECT-FILTER-BEFORE-PAGE | [IT-0020-MCP-GRANT-REPLAY][w3-current-grant] and IT-BD-03 | QR-173 |
| F-0747 | FX-0020-W3-CAPABILITY-STALE-NATIVE-HOLD | 0020 tests 6, 7 and 10 | QR-173 |

[w3f]: ../planning/feature-wave-3.md#acceptance-design-for-features-739-to-747
[w3-current-grant]: ../planning/test-and-traceability.md#task-0020-local-mcp

### The desktop app's shell

The Tauri app uses Angular Material and SDS. It holds no engine
state and acts through the management API. Linux uses an owner-only
Unix socket. Windows uses a user-locked named pipe. Both use spec
0118's protected channel and the same typed actions and Cedar checks.

1. **Engine client**: one typed client for every API call, with a test
   fake. Every call carries a trace. A test keeps the client in step
   with the API description on every commit.
2. **Events**: one task holds the event stream from the service and
   fans it out to screens. It reconnects with back-off from 1 to 30
   seconds and replays from the last event id the service still holds
   (at most 10 minutes or 10,000 events, whichever is smaller). Beyond
   that the screen reloads its data and says so. Check peer, launch
   identity and current grants before replay or delivery. A lost reply
   or event-stream replay cannot verify the current effective settings.
   Refresh through the 0014 role-status read. When the current reply
   refuses that scope, clear the denied status fields from the view.
3. **Breaker**: after 5 failures in 30 seconds it opens for 15 seconds
   and screens show the engine-down state.
4. **Shell**: layout, navigation, skip link, palette, help panel,
   notices and the engine-down banner.
5. Times show in the owner's time zone. Permissions come from the
   service, never from the app.
6. Ganit's settings what-ifs are read actions that call ganit with a
   strict time limit. If ganit is slow or down, the screen says
   "estimate unavailable" rather than waiting.
7. **Source and notices**: the Apache-2.0 app links to the exact
   release source and notices. `sutra source` gives the same details.
   A local build may use an owner-supplied source address.

### SDS on every screen

Plain British English from translation files. Type and spacing on an
8px grid; error summaries, plain warnings and underlined links; the
yellow focus ring: a 2px gap, a 3px `focus` (#FFD23F) band and a 2px
`focus-outline` (#0B0A14) band on `:focus-visible`, never removed.
Skeletons, never bare spinners; every empty state says what happens
next; reduced motion respected; no layout shift.

### Data

Owned by the service in PostgreSQL, with RLS on and forced on every
table and a policy per role; no role has `BYPASSRLS`. For example the
service role may insert and read `action_runs` but not update or delete
them, and reads only its own site's rows.

- `action_runs`: run id, action id, door, caller id, token id (or
  null), plan id, parent run id (or null), historical capability
  workflow id (or null), outcome, started, finished, error code.
  Kept 90 days. Spec 0099 owns the current workflow run and step
  identities. Record a workflow parent only from its exact admitted
  run context. Retained 0070 `CW-` values and their original parent
  links are historical. Preserve their identity under current read,
  retention and erasure rules. A historical action journal supplies
  no current workflow graph, resume authority or qualification proof.
  Reconcile any uncertain effect only through its exact owning
  effect and current authority. Never infer a new run from a journal.
- `action_plans`: plan id, action id, caller, input hash (blake3),
  preview, owner confirmation (who and when, for a plan with the agent
  rule "owner confirms"), expiry (15 minutes, or
  `actions.confirm_wait` for such a plan), applied flag. Applying an expired,
  changed or already applied plan is refused. Expiry uses the
  database's monotonic time.
- `agent_tokens`: id, name, site, account id, scopes, action allow list,
  expiry, created by, last used, revoked, token hash.
- Audit entries go to lekha.

### API and events

- List the catalogue the caller may use, with schemas and help; show
  one action.
- Plan a change and apply a plan, or run a read. A request may carry
  an `Idempotency-Key`. On an exact retry, apply current action and
  object disclosure checks before returning the stored result.
- The event stream: topics per module, each event with id, topic, time
  and ids only.
- Events published: `action.applied`, `action.refused`,
  `agent_token.created`, `agent_token.revoked`.

### Failures

Every failure is plain English with an error code and a manual link:

| Code | What the person sees |
| --- | --- |
| SUT-800 | "Sutradhar's engine is not answering." |
| SUT-801 | "Your account cannot do this." Names the permission |
| SUT-802 | Error summary, each field named |
| SUT-803 | "Things changed since you previewed this. Preview again." |
| SUT-804 | "Too many attempts. Try again in N seconds." |
| SUT-805 | "AI agents may recommend but not do this." |
| SUT-806 | "This agent's token no longer works." |
| SUT-807 | "Someone already did this", with who and when |
| SUT-808 | Banner "Live updates paused, reconnecting" |

- A failure inside `apply` rolls back the action's own transaction; the
  audit entry records the refusal or failure, never a success.
- The plan id ties preview to change. After reply loss, reconcile
  the original effect using the same idempotency key. Return only
  the result allowed by current scope. Keep the existing write
  marker so a retry cannot repeat the effect.
- The native supervisor keeps normal installs running when a window
  closes or crashes. Portable mode stops under spec 0030. A closed
  window loses its session and pending deliveries. If the engine is
  down, show SUT-800 and date any cached figure. Only the current
  spec 0030 recovery record can show a paused application update or
  verified application-version rollback. Read current ranking rollback
  through its 0077 result and the existing scoped view. A verified
  application rollback and dated ranking history cannot establish
  current ranking readiness. If the ranking result cannot be verified,
  show it as unavailable and point to the matching local recovery
  guide. Retain only history permitted by the current read scope.
- Long actions report through the progress system and are cancelled only
  through `karya.cancel`.
- CPU-heavy work never runs on Tokio's threads; the layer only waits.
- Two people approving the same thing: one succeeds, the other gets
  SUT-807 naming the first.
- A renamed action keeps its old id as an alias for one release, then
  it is retired in the registry and the release notes.
- A renamed site account keeps its account id in the audit.

### Security and privacy

- One permission check, in dwarpal with Cedar, for every door. The app
  may hide controls a person cannot use, as a courtesy; the API refuses
  regardless.
- Publication and moderation actions are refused or propose-only for
  agents, tested for every action.
- Rate limits per token and per account.
- Tokens never appear in logs, spans or audit entries (secret types).
- No member text in metrics or spans: ids, counts and durations only.
- The management API is fuzzed from its API description.

### Accessibility (WCAG 2.2 AA)

Every screen, in light and dark, at wide and narrow window widths:

- Every control reachable by keyboard in a logical order; a skip link;
  the focus ring on `:focus-visible`; focus moves to the main heading
  after navigation and back to the trigger when a dialog closes; focus
  never hidden under sticky bars (2.4.11).
- New events are announced in a polite live region, at most one
  announcement every 5 seconds, never moving focus. A "pause live
  updates" switch stops the screen changing.
- The palette follows the ARIA combobox pattern; its shortcut can be
  turned off or changed (2.1.4).
- Targets at least 24 by 24 CSS pixels (2.5.8); SDS controls are 48px
  (40px compact). No action needs dragging (2.5.7).
- Help is in the same place on every screen (3.2.6); forms do not ask
  again for what was entered (3.3.7).
- Errors use the error summary, linked to each field; colour never
  carries meaning alone.
- Reduced motion turns off animation; content reflows at 320 CSS pixels
  without sideways scrolling.

### Capability ids

Every screen, command and MCP tool carries a capability id; an action
without one fails the gate. The doors are routes, not capabilities.

| Where | Capability |
| --- | --- |
| Palette, `sutra find` | CAP-006 search from any page |
| Help panel, catalogue, `describe`, `docs`, `actions`, MCP resources | CAP-008 look up what something does |
| Error panel, `sutra error` | CAP-009 look up an error code |
| Manual link on every screen | CAP-007 |
| Access screen | CAP-071 choose which staff groups may act |
| Security log screen | CAP-072 see refused attempts and limits hit |
| Agents screen, `sutra agent token create\|list\|revoke` | NEW-031-A connect an AI agent with a limited, revocable token |
| "Source code" link, `sutra source` | NEW-031-C get the source of the exact version you run |
| Saved layouts | NEW-031-D save my screen layout |
| Settings what-ifs | CAP-012, CAP-015 |

Each feature's actions (including template, rollout and report actions
of the agency edition) carry that feature's ids.

### Observability

- Metrics: `sutradhar_action_runs_total{action,door,outcome}`,
  `sutradhar_action_duration_seconds{action,door}`,
  `sutradhar_mcp_sessions`, `sutradhar_event_stream_clients`,
  `sutradhar_app_engine_breaker_state`; spans per action with action id
  and run id only.
- Alerts: breaker open over 2 minutes; event stream down over 1 minute;
  agent refusal rate above its usual level (a fault record, not a
  notice).
- Every metric and attribute is on the "what we collect" page.

### Untrusted text markers (1287)

Every exposed MCP resource uses the admitted typed response structure.
Use the exact negotiated 0020/0118 envelope. Keep names, descriptions,
references, failure metadata and imported or generated
strings in separately marked data fields. Preserve the marker through
list, read, projection, rendering, serialisation and nested resources.
The trusted envelope supplies admission and scope. Text cannot
enter control or prompt instructions, declare itself trusted, alter a
grant or expand
a read or action. A missing marker or unrecognised schema blocks
the projection with a safe reason. Evaluate current site, actor,
resource rights, visibility and erasure before returning any field.
Apply the same checks when a cached resource is read or restored.
Only public help is eligible on an unenrolled route.

Feature 1287 owns that MCP resource contract. Feature 1286 in spec
0091 owns the drafting-side enforcement against untrusted sources.
The same inert datum can be checked at both boundaries. Neither check
creates a tool, transfer route or additional trusted content source.
QR-194 qualifies marker, denial and safe replay evidence.

**FX-0020-W3-MCP-RESOURCE-MARKER** is the planned F-1287
resource fixture. Use one admitted synthetic nested resource,
marked description and inert forged label. Through list, read
and restored-cache read, verify that actor, scope and markers
are unchanged. Withhold an enrolled resource with a missing
marker or malformed failure metadata, and deny it before
payload access on an unenrolled route. Public help remains
eligible on the latter. No content changes authority. Reuse
0020 tests 6 and 7 and AT-LOCAL-TRANSPORT for scope. QR-194
holds exact envelope, fixture and protected replay admission.

#### Exact assistant revocation (1302)

Use the existing owner-only Agents action. Bind the exact assistant,
site, enrolment and affected token or grant generations. Apply the
revocation and safe receipt in the same durable step. Advance the
authorisation generation before reporting success. Already-revoked
requests can return the current permitted outcome without reviving
scope. Reusing a request key with different input conflicts.

Every affected tool, resource, discovery, queued request, work lease
and result checks the current generation and actor before it can
expose data or commit an effect. Fence pending owner confirmations and
undelivered results. End future protected session and notification
traffic. An admitted in-flight operation can finish cleanup only, and
cannot commit or hand back protected results after revocation. Protect
retained records. Fence in-flight stream chunks and staged files at
actual exposure. Before restore check 0118's signed minimum
revocation epoch outside restored state. Hold where current minimum
or erasure evidence cannot be proved.
Previously authorised copies already outside Sutradhar follow 0032.

The assistant cannot revoke, issue or widen its own grant. Reconnect,
rotation and restored state cannot reuse the old grant. Fresh access
requires the existing separate owner approval and enrolment. This
action does not delete a native account, grant a role or create a
network listener. QR-195 qualifies exact scope and the revocation race
with 0118's protected session tests.

**FX-0020-W3-EXACT-ASSISTANT-REVOCATION** is the planned
F-1302 fixture. Queue a protected request for one enrolled
synthetic assistant, start a result stream and leave an exact
owner confirmation pending. Apply the owner-only revocation.
Compare repeated scope and changed input under the same key.
Before restart, try a second session and an older or missing
0118 minimum epoch. Verify a durable current receipt, denial
of future protected stream chunks and staged files, fenced
confirmation and cleanup only for the in-flight job. An
unaffected enrolment keeps its scoped access. No account
removal or old grant on reconnect. Reuse 0020 tests 4, 6 to 8
and IT-0118-LOCAL for their shared boundaries. QR-195 holds
the exact revocation race and protected restore proof.

#### Automatic secret scrubbing (N-1444)

Reuse the typed secret and permission boundary in Security and privacy.
Check typed source data before logging, model input or durable
scratch. Refuse malformed fields with a safe reason. Scrub before
formatting, including
log text, spans, tool errors, workflow preview, graph, edge sample,
clipboard and finished files. Scope references before reading. Deny
missing scrub profiles and unrecognised protected fields. Check the
exact rendered or serialised output so separately benign fields cannot
recreate a secret or identifying address together. No raw value,
credential handle, token, private identifier or unsafe diagnostic may
be returned, written to temporary work or supplied to a model. Current
grant and erasure also fence replay, cached data and restore.

Feature N-1444 uses spec 0099 JW-15 for run projections and the current
0032, 0065 and 0118 rules for audit, custody and protected transport.
QR-201 qualifies the shared sink and refusal fixtures. Plant synthetic
secret-like values at each sink and check its output and related errors.
Custody under 0065 is separate. Redaction grants no disclosure.
An unavailable safe projection holds the affected view or output
through the existing owning route.
See [FX-0020-W5-COMPOSED-SINK-SCRUBBING][w5s].

[w5s]: ../planning/feature-wave-5.md#acceptance-design-for-n-1430-to-n-1453

#### Local MCP setup, discovery and control (N-1484 to N-1498)

Use the existing single-site action registry, Cedar authority, local
owner transport and protected 0118 session. Bind installation, site,
actor, account, client enrolment and grant generations at discovery,
request, commit and result delivery. A preference, label, imported
card or client claim supplies no authority. Check replay, restore and
erasure before exposing protected content. The app's owner-only views
take their own eligible audit projection under 0032.

| Feature | Exact local scope |
|---|---|
| N-1484 Installed AI app scanner | Inspect only approved local installation markers for admitted client and platform profiles. Report supported, unverified or not found without reading personal client data, executing a client, contacting a vendor or sending telemetry. The owner selects the client and uses the existing enrolment. Detection grants no connection. [FX-0020-W5-PASSIVE-LOCAL-CLIENT-MARKER](../planning/feature-wave-5.md#acceptance-design-for-n-1478-to-n-1501) |
| N-1485 Nothing-hidden MCP mirror | Use the same owned typed action and settings catalogue across the app, CLI and MCP. Expose the current permitted subset and a safe description of its supported actions. Never expose credentials, native effects, protected controls, private paths, logs or fields denied by the current site and action. An app control does not imply an agent action. [FX-0020-W5-SCOPED-MCP-CATALOGUE](../planning/feature-wave-5.md#acceptance-design-for-n-1478-to-n-1501) |
| N-1486 Live MCP activity ticker | In the protected local owner app only, project eligible local action receipts with exchange time, status and exact admitted scope. Separate requested, refused, proposed, confirmed and applied. Do not report a setting change from a plan alone. Recheck scope and erasure before every stream chunk and on reconnect. Scrub free text and protected detail. MCP cannot read audit history or session receipts. Any admitted share is an inert owner export under DR-028 and 0032. No live, replayable or remote link. [FX-0020-W5-OWNER-ONLY-LIFECYCLE-TICKER](../planning/feature-wave-5.md#acceptance-design-for-n-1478-to-n-1501) |
| N-1487 Plain-English trust dials | Present named presets as bounded owner-selected scopes intersected with native account, site and action rights. Show the exact effect before applying an eligible grant change. No full-mechanic grant or bypass exists. In a scope that permits a settings plan, actual apply still requires the exact separate live owner confirmation. [FX-0020-W5-INTERSECTED-GRANT-PRESET](../planning/feature-wave-5.md#acceptance-design-for-n-1478-to-n-1501) |
| N-1488 Visual MCP tool switches | List eligible current actions from the owned registry and allow only admitted owner grant controls. A newly enabled scope needs the existing owner action. A disabled scope fences new discovery, queued requests and undelivered effects and results. Enabling the same action again cannot revive a request or protected result bound to the earlier grant generation. A new authorised request uses the current grant and any separate approval required by the action. Tool visibility cannot widen native authority or alter protected settings. [FX-0020-W5-TOOL-GRANT-REVOCATION-FENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1478-to-n-1501) |
| N-1489 Top-bar AI pause switch | Use an authorised local owner control to hold admitted MCP actions and protected delivery under a current pause generation. Show pending effects and their real states. Fence new and queued work and undelivered protected results before reporting paused. Committed effects retain their original receipt. Resume requires current grants and does not replay cancelled work or revive revoked tokens. Keep the separate 0042 drafting stop unchanged. [FX-0020-W5-MCP-PAUSE-GENERATION](../planning/feature-wave-5.md#acceptance-design-for-n-1478-to-n-1501) |
| N-1490 Guided remote MCP bridge | DR-028 keeps MCP local. Refuse remote sign-in, codes, relays, public listeners and cloud bridge configuration in the app, local tools and restored state. Offer only the existing local protected setup. No credential, session or remote bootstrap record is created. [FX-0020-W5-REFUSE-REMOTE-MCP-BRIDGE](../planning/feature-wave-5.md#acceptance-design-for-n-1478-to-n-1501). |
| N-1491 Two-second MCP health check | Use the existing bounded local handshake and catalogue evidence for the exact verified peer and current session. Check reachability and eligible discovery separately. Two seconds is a qualification target. An unauthorised, late, mismatched or unenrolled peer shows a safe unavailable result. No test tool runs or credentials are disclosed. [FX-0020-W5-CURRENT-ENROLLED-PEER-HEALTH](../planning/feature-wave-5.md#acceptance-design-for-n-1478-to-n-1501). |
| N-1492 Agent name tags | Bind the display label to the exact enrolled client, site, actor and session. Label unknown or revoked identity accurately. Render labels as inert text. A client-supplied name cannot take another client's attribution, impersonate a staff member or transfer permissions. [FX-0020-W5-ATTRIBUTION-LABEL-NO-AUTHORITY](../planning/feature-wave-5.md#acceptance-design-for-n-1478-to-n-1501). |
| N-1493 Human approval popups | Show the exact eligible typed plan, safe old and proposed values, effects and scope. Mask protected values via 0065 or hold where scope review cannot work without disclosure. Use the current owner app or eligible interactive sutra route under 0020. A popup cannot confer authority on a caller. Ask for the current owner-only live confirmation for eligible settings. Confirm once, before its existing expiry, and recheck plan and grant. Denial, change, pause and revocation hold. MCP, scripts, flags and the never-changeable class cannot cause an apply. [FX-0020-W5-OWNER-LIVE-CONFIRM-EXACT-PLAN](../planning/feature-wave-5.md#acceptance-design-for-n-1478-to-n-1501). |
| N-1494 Interactive MCP sandbox | Use bounded sealed synthetic inputs and a named eligible tool profile to inspect request and safe response shapes. Show proposed effects and refusal reasons. The sandbox opens no client or remote session and cannot read live site data, execute tools on live inputs, approve, spend, dispatch or save a setting. [FX-0020-W5-SYNTHETIC-TOOL-SANDBOX-ONLY](../planning/feature-wave-5.md#acceptance-design-for-n-1478-to-n-1501). |
| N-1495 Automatic secret shield | Reuse N-1444 and the typed secret rules in 0065 and 0118. Redact or withhold the exact emitted tool results, errors, previews and receipts. Grant no raw credential, secret handle reference, compound identifier or private endpoint. A troubleshooting view grants no setting, key or provider change. [FX-0020-W5-TYPED-SECRET-SAFE-PROJECTION](../planning/feature-wave-5.md#acceptance-design-for-n-1478-to-n-1501). |
| N-1496 Zero-restart tool discovery | Actions remain statically registered. A current admitted versioned 0104 reference may appear only under a predeclared action and profile. No new callable action or dynamic schema from a stored script or button. A saved script or button draft changes no tool catalogue. After separate valid admission, expose the current catalogue generation by the existing authorised local discovery or admitted notification profile. Recheck scope on list and invocation. An old ID, unsupported notification or dropped grant cannot revive a tool. [FX-0020-W5-STATIC-ACTION-CURRENT-REFERENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1478-to-n-1501). |
| N-1497 One-click fallback setup card | Show an owner-only, versioned local setup card for a supported older client. Include safe requirements and local steps with placeholders rather than live secrets or writable protected paths. Reuse the existing 0118 enrolment on actual connection. Unsupported and unenrolled clients get only public help, with no plaintext protected fallback. [FX-0020-W5-LEGACY-CLIENT-LOCAL-HELP](../planning/feature-wave-5.md#acceptance-design-for-n-1478-to-n-1501). |
| N-1498 Plain-English session receipts | In the protected local owner view provide one bounded three-item receipt per finished session: checked work, confirmed effects or proposals, and holds. Use a settled close or expiry and durable identity. Deduplicate close, recovery and replay. MCP cannot read receipts or audit history. Bind exact session, action and outcome versions and grant. Missing confirmation or receipt stays uncertain. Hold private context and keys, keep original authority and do not imply that the client applied a setting it only proposed. [FX-0020-W5-SESSION-CLOSE-EXACT-RECEIPT](../planning/feature-wave-5.md#acceptance-design-for-n-1478-to-n-1501). |

QR-203 qualifies the exact local client profiles, custody, pause and
revocation races, typed discovery, safe receipt and zero remote paths.

#### Agent proposals and live-change refusals (N-1514, N-1515, N-1520, N-1521)

Use the existing owner-started enrolled session, one active site,
current grant, typed action schema and owner-only local transport
under DR-028 and spec 0118. Treat requested source, imported code
and assistant text as data. Scope before showing file, settings,
script or UI details. No session may fetch unknown source, widen its
grant, accept its own proposal or use an older approval. Pin exact
profile, effect and request identity. Enforce current authority
through retry and restore.

| Feature | Exact proposal or refusal |
|---|---|
| N-1514 Hot-reload Python hooks | Only separately admitted 0104 extension slots may use pinned user-script versions after tests and human approval. Core crawling, storage, product UI, replies to sites and live ranking remain with their owning Rust contracts. A local MCP proposal cannot inject code or mount a hook into them. [FX-0020-W5-PINNED-EXTENSION-NO-CORE-HOOK](../planning/feature-wave-5.md#acceptance-design-for-n-1502-to-n-1525). |
| N-1515 MCP live-tweak bridge | Prepare a local versioned script proposal and synthetic test design within the admitted schema. A typed draft may refer only to current eligible manifests and permitted source. The local session cannot write executable product files or activate a script. Review, licence and fixture evidence and current owner approval stay in 0104. There is no bridge or build bypass. [FX-0020-W5-LOCAL-SCRIPT-PROPOSAL-ONLY](../planning/feature-wave-5.md#acceptance-design-for-n-1502-to-n-1525). |
| N-1520 Live screen customisation | Offer an inert proposal for an admitted declarative presentation choice if the existing registry supports it. Bind exact screen, owning action and eligible fields. Preserve separately admitted declarative UI column, chart, button or panel proposals under 0104. No immediate mount, runtime injection, private field access or native or network effect. Unadmitted screens or formats stay unavailable. [FX-0020-W5-INERT-PRESENTATION-PROPOSAL](../planning/feature-wave-5.md#acceptance-design-for-n-1502-to-n-1525). |
| N-1521 Ask-to-build live features | Convert a request only to a scoped plan or proposal under existing local routes. Show unsupported actions and dependencies separately. An eligible request can yield an inert script draft through 0104. It cannot activate, edit product code or bypass the owning gates. Scheduled sends, bucket transfers and product changes each need their own admitted action, independent test and exact owner approval. Tool output creates no new capability or permission. [FX-0020-W5-SCOPED-PLAN-NO-TOOL-AUTHORITY](../planning/feature-wave-5.md#acceptance-design-for-n-1502-to-n-1525). |

QR-204 qualifies covered and refused proposal paths, including
synthetic requests to change a protected setting, a source or a
live action. These rows admit no new MCP endpoint or token scope.

#### Direct MCP bug microscope (N-1528)

Use the existing 0104 safe diagnostic projection for the exact local
script, run, manifest and selected scope. Use the existing read-only scrubbed
journal projection. A separate
local test action may create a bounded synthetic fixture to show declared
types, approved synthetic values and line
numbers, failure classification and safe reason. For a live run,
expose only fields separately admitted and scrubbed for this purpose.
Omit raw stack dumps, hidden paths, arbitrary variable values, member
text and credentials. Treat diagnostic strings as untrusted data under
1287. Recheck current site, actor, client, scope, version and erasure
before response, replay and restore. An unproved field or lost authority
produces a safe refusal. Refuse worker attachment, breakpoints,
expression evaluation, stepping, source and live variable browsing.
A diagnostic gives no worker, code edit, package
install, arbitrary memory read or new network connection.

Reuse 0118 protected local sessions, 0032 masking and N-1444. QR-205
qualifies exact diagnostic projections without granting debugger access.
See [FX-0020-W5-SCOPED-SYNTHETIC-DIAGNOSTIC][w5-safe-diagnostic-case].

[w5-safe-diagnostic-case]:
  ../planning/feature-wave-5.md#acceptance-design-for-n-1526-to-n-1550

#### Local client setup, repair proposal and reversal (N-1872–N-1874)

Use the enrolled owner-started local transport under 0118 and
DR-028. Bind the exact install, actor, site, scope, current grant
and request revision. Helpers and tool outputs remain inert.
These forms add no client, permission class or executable route.

| Feature | Exact local action or held route |
|---|---|
| N-1872 Double-click MCP extension bundle | Present an admitted exact client profile and canonical versioned configuration-only bundle with checked digest and independently trusted provenance for supported clients. It contains no token, executable code, updater, raw path or endpoint instruction outside the approved local transport. Opening offers owner-reviewed setup only. Actual connection still needs separately enrolled process, exact scoped credential and peer proof. Reject stale, altered, substituted or repurposed configuration. Unsupported clients receive the existing safe setup help; no auto-connect or remote bridge. [FX-0020-W5-VERIFIED-LOCAL-MCP-SETUP](../planning/feature-wave-5.md#acceptance-design-for-n-1863-to-n-1880) |
| N-1873 Ask AI to fix this button | Show the owner a bounded scrubbed error projection and the exact intended local recipient session and site before submitting an eligible request through the current owner-started client. Bind reason, scope and release and recheck grants. The client may explain or draft an eligible repair plan. No automatic agent start, sensitive log read, remote hand-off, setting write, repair, package install or confirmer role. [FX-0020-W5-SCOPED-LOCAL-REPAIR-PROPOSAL](../planning/feature-wave-5.md#acceptance-design-for-n-1863-to-n-1880) |
| N-1874 Undo AI changes | Show each recorded reversible local effect bound to its original exact confirmation and receipt, current target version or digest, masked before and after state and owning inverse action. Compare-and-set atomically with one stable inverse idempotency key. Reverse eligible dependencies in reverse order and hold dependants on unknown predecessor effect. Hold unknown completions, changed targets, irreversible or native effects and protected settings. Require current exact one-use owner confirmation and the owning route per eligible inverse. Reconcile a lost reply or partial result using the original inverse receipt. Show safe conflict and partial states and never hide changes already applied. No blanket multi-step rollback, resurrected authority or repeated effect. [FX-0020-W5-PER-EFFECT-APPROVED-INVERSE-RECONCILIATION](../planning/feature-wave-5.md#acceptance-design-for-n-1863-to-n-1880). |

N-1874 does not expand a draft undo under feature 734 or the
existing settings, recovery and restore contracts. Prove any
admitted ordering and per-effect receipts under QR-219.

### Scientific experiment actions

[0148](0148-scientific-experimentation-engine.md) and
[0151](0151-site-lawbook-calibration-and-replay.md) define static
same-site experiment reads and preparation actions. Each action
checks its scope, current object authority and pause state.
MCP may prepare a review and read admitted projections. The
desktop owner confirms the exact registration, release or
guidance change. No tool token can confirm, access a raw study
bundle, read a vault secret or change an exposure.

## Limits

- Speed targets on Dell, each also under the 5% median rule:

| Need | Target |
| --- | --- |
| A screen's largest content appears | under 1.5 s, p95 |
| Layout shift on any screen | 0 |
| A click on an action gives visible feedback | under 100 ms |
| A read action returns, layer only | p95 under 50 ms |
| A plan or apply returns, layer only | p95 under 200 ms |
| Another screen shows an approval | p95 under 1 s |
| `sutra --help` or `sutra describe` prints | under 150 ms |
| MCP tool list for a token | p95 under 300 ms |
| `sutra find` answers | p95 under 100 ms |

  "Layer only" leaves out the action's own work. With 10,000 actions,
  the palette and `sutra find` stay within these targets.
- Memory, inside the service slice: registry, runner and MCP 24
  MiB; help and manual text 8 MiB; the help index 4 MiB, memory-mapped.
  The desktop window, web view included, stays within its cap with
  the event fan-out. The nightly peak-memory check writes a fault record
  if a share is passed. Event replay holds at most 10 minutes or 10,000
  events.
- Plans expire after 15 minutes, except a plan waiting for the owner's
  confirmation, which uses `actions.confirm_wait`. That setting is
  declared in the actions module's settings section (spec 0014): a
  duration from 1 minute to the hour in AGENTS.md section 5.5, that
  hour by default in every profile, `owner_only`. Tokens expire after
  30 days by default. Action runs are kept 90 days.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Rust tests in the crates named above, Tauri and Angular screen tests,
SQL policy tests and Cedar tests. AT-LOCAL-TRANSPORT and IT-0118-LOCAL
exercise both Linux distributions and Windows. Refuse a different
account, same-owner unenrolled process, wrong installation peer or
invalid WebView origin, window or launch. Deny unprotected MCP data.
Revoke a token before replay. Reconnect with current authority only.
Match action and error results and expose no management network port.

For the first product management action, use
`REQ-0020-FIRST-ACTION-SCOPE` and `DES-0020-FIRST-ACTION-BINDING`
with [`TASK-0020-OWNER-DOOR`][first-owner-door]. Bind the selected
registered action ID, capability, typed input and result and its
ActionSpec permission to the exact site and object in the owning
test plan. The owner-only local transport authenticates the peer.
The runner uses the current action and site policy for each
request. Use the existing safe result for a Cedar denial or a
disabled action. A missing first action identity, scope or
registered contract holds product action evidence under 0076.

In BDD-0020-OWNER-TRANSPORT-BOUNDARY use a synthetic registered
site read across the supported Unix socket and Windows pipe
profiles. Keep the OS peer, installation, current transport
trust, site and action fixed. In the Cedar branch give the
same scoped caller an allow and a matching explicit denial for
this action. Under 0109 BE-13 deny wins. Return SUT-801 and
no action result or protected access. In the eligible control
use the current allow without the denial and observe the exact
typed read result. In a separate disabled-action profile,
keep the peer eligible and refuse the disabled action under
the current registry. Preserve the existing safe refusal
audit. None of these denials reaches an owning action effect.
The fixture qualifies no production action registration.

[first-owner-door]: ../planning/test-and-traceability.md#task-0020-owner-door

IT-0020-HISTORICAL-CW-RESTORE uses 0099 JW-06, JW-09 and JW-16
with the exact root and child proof in F-0680. Use a compatible
restore under UT-0019-RESTORE-COMPATIBILITY. Restore same-site
retained legacy `CW-` parent and child action
journal rows, one eligible completed outcome and one uncertain
outcome. Give the actor current same-site run scope and a matching
action name and old key. Supply no approved 0099 graph or pinned
run, step or child path. Return only permitted history or its held
outcome. Preserve the original write marker. Replay and resume
create no run, step, allowance, lease, outbox entry, dispatch or
release credit. Reconcile an exact original effect only through
its owning route with current authority for that reconciliation.
A foreign-site or withdrawn grant exposes no protected details.

The Wave 3 acceptance design binds F-0713 through F-0737 in
the groups beginning at
[713](../planning/feature-wave-3.md#acceptance-design-for-features-713-to-723)
and
[724](../planning/feature-wave-3.md#acceptance-design-for-features-724-to-738).
The cited checks cover their shared boundaries. Each fixture still
needs its exact source case under QR-171 or QR-172. A wording
fixture with an unknown delivery proves refusal to redispatch only
when the case exercises that state.

Only components admitted to ADR-0020's strict core require
100% line, branch and MC/DC
coverage and every killable mutant killed; equivalent mutants require
proof and independent review. Other components follow the owning risk
matrix. The runner's step order, `Authoriser`, the agent rules and plan
expiry retain their property tests, Kani proofs and assigned mutation gates.

1. One action, three doors: an approval through the API, `sutra` and
   the app reaches `setu.suggestion.approve`, writes one audit entry
   with its door and returns the same output shape.
2. Every publication and moderation action is "refused" or "propose
   only" for agent tokens.
3. MCP calling `setu.suggestion.approve` with the owner's agent token is
   refused with SUT-805 and nothing changes.
4. An agent with the settings scope plans a settings change; the plan shows
   the effect and checks. With the owner's live confirmation, clicked in the
   app or typed at `sutra`'s prompt, it applies, and history and lekha record
   the agent, the account and the confirmation as `instruction_ref`. Refused,
   with nothing changed: no confirmation; a confirmation through `--yes`,
   `--json`, MCP or a script; any token calling the confirm step; a
   confirmation after the plan's wait expired, or used a second time, or given
   for another plan; a confirmation from any account but the owner's; and any
   change in the never-changeable class, even confirmed. That class includes
   the outbound-content switches of spec 0100 (the Standard and Full levels of
   each outside-service destination and the site-text switch), which are
   `owner_only` settings (spec 0014; owner decision of 20 November 2026,
   DR-079). The outside-route switches of specs 0117, 0131, 0132, 0133, 0136,
   0137, 0143 (the paid local preview switch), 0146 and 0152 are also owner-
   only under spec 0014:133.
5. An agent's ranking change is stored as a proposal with its measured
   effect; nothing is activated.
6. Property and Kani proof: a token can never do what its account
   cannot.
7. Without the private scope, MCP lists and searches show no post that
   is not guest-visible, no private conversation and no email address;
   a session sees one site only and never credentials, keys or audit
   logs (unit and property).
8. A plan applied twice, after 15 minutes (or, for a plan waiting for
   the owner's confirmation, after `actions.confirm_wait`: refused at
   one hour and one second, accepted at 59 minutes and at exactly one
   hour), or after its
   input changed is refused with SUT-807 or SUT-803 (unit and
   property); Kani proof that
   no apply runs without a matching, unexpired, unapplied plan.
9. Two approvals within 10 ms: one succeeds, the other gets SUT-807
   naming the first, and the first person's screen updates within 1 s
   (loom and stress).
10. The gate walk finds every action with summary, example, manual page,
    capability id and documented errors; `sutra describe` prints them.
11. `sutra find "stop the backlog"` puts `karya.job.pause` in the first
    three results.
12. Service stopped: within 5 seconds an open screen shows SUT-800 in
    place of its data, no number shown as live, and it recovers by
    itself when the service returns (failure injection).
13. Kill or close the window. A normal install still serves eligible
    work. Portable close or unplug stops its engine. Reopening gets a
    fresh session and cannot replay an old window's deliveries.
14. Keyboard walk of every screen and state: each control reachable; the
    focus ring shows both bands and is never hidden; an accessibility
    checker passes every screen and state.
15. Each screen loads with fixture data with its largest content under
    1.5 s and zero layout shift.
16. Load: 50 app sessions and 20 agents run mixed actions for 2 minutes;
    the layer-only targets hold and no request fails.
17. A token in a request appears in no log, span or audit entry.
18. Every SDS component shown loading, empty, in error, with long
    content, dark and narrow matches its stored screenshot.
19. The footer link and `sutra source` give the source and notices of
    the exact version; for a build that is not published, the owner's
    address and the reminder.
20. SQL test: the service role cannot update an action run or read
    another site's rows, even with no filter in the query.
21. Mutation run on the rule-critical code: no unexplained killable mutant
    survives; equivalent mutants require proof and independent review under
    ADR-0020.
22. MCP is reachable on no network port.

Test-first order: action trait and registry checks (10); runner with
fakes (1, 8); dwarpal, agent rules, proofs and mutants (2, 5, 6, 21)
and policies (20); routes and idempotency keys; `sutra` tree, help,
find and error (10, 11); MCP tools, resources and tokens (3, 4, 7, 17,
22); event stream and replay; app client, events and breaker (12);
shell, palette, help panel, source link, screenshots and accessibility
(14, 15, 18, 19); the race, load and failure tests (9, 13, 16).

Build-blocking evidence: the action integration record must pin the current
MCP specification version and authorisation rules from its official site,
reconcile SUT-800 to SUT-809 against the central code list, and measure the
window's memory with event fan-out. Each affected implementation remains
blocked until those contracts and measurements are admitted.

## Changes from the source

- The owner uses Tauri and Angular with SDS. Requests use the
  protected local management link for the supported platform.
- Moderators accept or refuse in the sites' own admin pages, with the
  sites' own sign-in; those decisions arrive through the site link.
- Web sign-in with the forum account, the owner's emergency login
  (SUT-809, NEW-031-B, its table and event), web sessions, cookies,
  form tokens and page presence are not brought, since the app is
  reached only from the owner's own account.
- MCP stays local through `sutra mcp` or the protected management
  link. Each session sees one site and follows spec 0118.
- MCP tools may read and draft, and may change a setting only with the
  owner's live confirmation, within the token's limits, as Q047 said.
- Permissions are Cedar rules evaluated by dwarpal.
- Colours come from SDS tokens; no other firm's colour values, marks or
  type are copied, and no theme add-on is used.
- The old served-app memory share is replaced by the window's cap.
- The old speed ratchet is the 5% median rule.
- Old web-stack checks and the no-script edge case are dropped; screen
  coverage follows ADR-0020 and the owning risk matrix, not a universal
  100% rule.

Source: `docs/specifications/operations/actions.md`,
`operations/actions-2.md` and `operations/actions-3.md` in the archive
`archive-2026-11-08-old-repo`.

## Unified audit actions

[Spec 0140](0140-unified-site-audit-suite.md) and specs 0141 to
0144 describe separately admitted site-scoped projections and
exact permission needs. Reuse static registration, per-operation
Cedar, pause-all and current proof on every read or preparation.
The [paid local preview](0143-audit-grid-findings-and-local-preview.md)
can be prepared by a scoped agent. The desktop owner alone
confirms the exact payable plan. Do not expose captured images,
raw log lines, credentials, private name views or billing history.
