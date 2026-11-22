# 0031: Shared-platform installation

This is how one Sutradhar installation serves XenForo forums and
WordPress sites side by side: installing, pairing, running, updating,
repairing, backing up and removing it, and the safety contract at each
site's edge. One shared service keeps every site's data, permissions
and work apart. Removing one site's connection leaves the others
running, with the shared models they still need. It is roadmap Phase 5
item 1, with the site contracts of Phase 4 items 3 and 4.

## Verified native add-on handoff

This boundary applies before archive extraction, registration or
activation. Bind the exact manifest part and immutable ZIP bytes
to the selected site, native account and currently authorised
action. Either use the same verified upload stream end to end or
prove the private staged upload independently at the consumer.
An ordinary file chooser, filename or post-install check provides
no byte binding. Use the existing native screens and rights. On a
hosting profile with no qualified verification-to-use handoff,
keep installation through this route unavailable. The add-on
cannot verify its own trust after its installer starts.

### Exact native ZIP acceptance

Run `IT-0031-ZIP-EXACT-UPLOAD` under the [0005 detached delivery
profile](0005-release-coordination.md#platform-acceptance-design).
Start without Sutradhar's add-on or plugin installed on each
applicable shared-hosting fixture. Run XenForo 2.3 and both
supported WordPress majors separately, using each declared host
profile and its exact native installer and permission contract.
An update or a preinstalled add-on cannot prove first installation.

Provide both supported WordPress profiles from this fixture inventory
for [0003 WordPress Plugin Check][wp-check]. Use fresh sealed test
instances. The first WordPress source candidate needs an admitted
profile and the complete exact plugin projection. The same profile
identities govern the release ZIP check. The checker result is a
separate receipt. It supplies no native install, update, permission
or live-footprint proof.

[wp-check]: 0003-toolchain-and-check-runner.md#wordpress-plugin-check

Use an unchanged synthetic release ZIP and its detached manifest.
Bind the owner verification to the current site, host, release,
archive length and digest. Independently check the signed-in native
administrator's approval for that exact site and action. Before
extraction, the native consumer either receives the same immutable
verified stream or independently matches the private staged upload
against that release binding, with identity preserved through use.
Expect one native receipt bound to the exact upload, actor and action.
After a lost response, reconcile the original result before a retry.
Use the current native read rights for that site and receipt.

In a separate lost-response case, commit the original native
installation and receipt, then remove the actor's native management
access to that site's receipt. Keep the release, package and native
effect fixed. Refuse to disclose the receipt on the old actor's
read-back. Keep the native files and original receipt, with no
repeat extraction, registration or activation. Public release help
retains its ordinary read scope. The separate control keeps the
original actor's exact native read rights and recovers only the
one committed receipt under the existing route.

Keep the selected release and requested operation fixed. Separately
substitute a staged upload, switch the site or actor, revoke the
native approval, withhold the host handoff proof, and offer an
ambiguous archive. Refuse before extraction, scripts, registration
or activation. For archive ambiguity, sign a separate synthetic ZIP
with two members at the same decoded path. Match the verified
length and digest so only the archive check can refuse it. A
picker or ordinary native upload with no immutable
handoff proof, or verification first attempted from inside an
unpacked archive, supplies no proof. Check that the refused action
leaves the native files and installation state untouched. Record
only authorised evidence under 0005. In separate synthetic cases,
keep the exact verified upload, approved native administrator and
selected host. Offer an additional outside recipient for the ZIP
or its proof. Refuse the altered route before either reaches the
recipient or the native consumer starts extraction, registration or
activation. Retain the exact approved owner-to-host binding. In the
control assert that only the approved native host consumes the
handoff and its receipt matches the original actor and upload.
Use a synthetic recipient, with no outside request or disclosure.

In a separate WordPress update branch, keep the selected site,
release, WordPress ZIP, actor and verified upload fixed. Change
the native site database version after the earlier check. Provide
no compatible current host profile for that database. The valid
ZIP leaves the update held before extraction, scripts, native
file changes, database writes or activation. Keep the previous
plugin files and native action records. Report the WordPress
compatibility reason separately from ZIP verification. A control
with the exact compatible current profile may use the already
approved update route after its remaining checks pass. This
branch supplies no first-install proof.

In a separate shared-hosting update case, verify the exact WordPress
ZIP and immutable handoff under the current native approval. Keep
the selected WordPress site, host, artefacts and actor unchanged.
At native preflight substitute only the compatibility result with a
genuine XenForo result for a native update of the same release. Hold
the WordPress action before extraction, migration or activation.
Report the missing exact WordPress compatibility proof and keep the
current plugin files, native state and prior action receipts. Emit
no new native success or rollback receipt. A separate control with
the exact current WordPress result can proceed under the existing
checks. Keep the XenForo result scoped to its own tuple. The
compatibility check supplies no per-site fallback proof for IP-26.

For the separate WordPress topology branch of IP-08, start with a
qualified single-site native ZIP fixture and its verified lifecycle
result. Keep the exact signed ZIP, WordPress core and PHP build bytes
and package proof. Use a second synthetic target in multisite mode
without its own exact lifecycle and isolation result. Supply current
native authority and exact-action approval in the fixture. Offer the
genuine single-site result for that target. Hold the unqualified mode
before extraction, scripts, registration or activation, and label the
reason. Keep the single-site result scoped to its original tuple.
It yields no native success or pairing for the other target. The
exact original single-site control retains its own eligible handoff.

IP-08 still needs the rest of the lifecycle and compatibility tuple.

### Native ZIP handoff trace

`DES-0031-NATIVE-ZIP-HANDOFF` owns verification at native use.
0005 retains the release-verifier and recipient boundary. Every
branch uses `IT-0031-ZIP-EXACT-UPLOAD` on its admitted host under
`TASK-0031-SITE-FIXTURES`. The Phase 4 demonstration consumes its
native result only for that exact host and action.

| Owning requirement | Exact clause and case within the owning design |
| --- | --- |
| `REQ-0031-ZIP-NATIVE-AUTHORITY` | Match the exact selected site, scoped actor and current native permission for the action independently of owner release proof. Refuse site or actor swap and revoked native approval. |
| `REQ-0031-ZIP-EXACT-USE` | Use the same immutable upload stream or the independently verified private stage before extraction, registration or activation. Refuse substituted or ambiguous archive bytes. |
| `REQ-0031-ZIP-UNPROVED-HANDOFF` | An unproved picker, stock upload or verifier invoked after extraction cannot establish the handoff. Hold with no archive or native side effect. |
| `REQ-0031-ZIP-NATIVE-RECEIPT` | One exact approved upload, actor and action give one native receipt. Reconcile a lost reply only to that exact effect before retry. |

Use `REQ-0005-ZIP-RECIPIENT` for the separate no-extra-recipient
case in the same fixture. Current website rights and the release
proof both remain required.

| Variant identity | Applicability and exact evidence |
| --- | --- |
| `VAR-0031-XENFORO-NATIVE-ZIP` | XenForo 2.3 and its admitted native host/installer and administrator contract. Run the clean first-install and all refusals for this exact profile. |
| `VAR-0031-WORDPRESS-NATIVE-ZIP` | The WordPress native plugin host and admitted installer and capability contract. Check both supported majors independently on their exact host profiles. The major version identifies a fixture cell. |
| `VAR-0031-NATIVE-ZIP-STREAM` | The admitted native consumer retains the same immutable verified upload stream through use. Verify stream identity and the exact native effect. |
| `VAR-0031-NATIVE-ZIP-PRIVATE-STAGE` | The admitted native consumer independently matches the private staged upload before effects. Verify exact length, digest and identity through use. |

Apply only the verification path proven for the host and operation.
A named variant supplies no hosting qualification or new route.

## What it does

### Shape of an installation

1. The owner's PC keeps authoritative state. Spec 0030 owns the
   signed Linux package and Windows first trust check. The native
   supervisor controls the engine. The Tauri and Angular window uses
   spec 0020's protected Unix socket or named pipe.
2. Each site gets a thin signed add-on (XenForo) or plugin (WordPress).
   The app hands the owner the ZIPs verified under the
   [release profile](0005-release-coordination.md#detached-delivery-profile)
   to install through the
   site's own admin pages.
3. Helpers may process public chunks of work only. One PC can be both
   window and helper; the helper role never gains the window's private
   data or keys. Every role uses the same engine package.
4. No incoming network access is needed. The service calls each site
   over HTTPS; the site never calls the PC. Cloud helpers on the
   owner's own accounts exchange encrypted messages through the owner's
   cloud mailbox. Without a cloud account, the one PC does everything.
5. A site keeps working when the PC is asleep. For example, the forum
   can still apply a suggestion that was checked earlier, while an
   edited post waits for a fresh safety check.
6. Settings have one registry with public English keys. Cedar decides
   app permissions; Rego judges agents and builds. The 32 approved
   roles keep their scope.
7. The add-ons expose signed platform routes, current platform facts and
   supported native saves. They cannot start the service, run commands
   or give a site administrator installation authority. Public
   visibility, native permissions and independent approvals always
   apply. XenForo OAuth and outside connector OAuth are kept.

The [Phase 5 helper evidence][phase-five-helper]
needs its exact job, recipient and mailbox profile under QR-242.
Keep cloud dispatch unavailable until that contract qualifies. An eligible
helper can open only its approved public projection and return a
typed result for local validation. The mailbox operator has no
plaintext role. Use existing owner-only egress controls, the admitted
engine package, scheduler and measured resource bounds. Verify and
reconcile work on the owner's PC, including pause, revoked approval,
late or unknown delivery and cleanup. Preserve the existing limits
on outside services, remote management and multi-machine crawling.
A 0038 storage receipt proves only the object operation it names.

[phase-five-helper]:
  ../planning/implementation-catalogue.md#phase-5-exit-evidence

#### Proposed cloud helper base preview

For review under QR-242, consider only the base PageRank calculation
from the [0027 batch link-value preview][cloud-base-preview].
It reuses the 0066 Jaal job 251 calculation through the existing
0019 advisory class. It has no route to the candidate publisher.
The current 0027 contract keeps the preview local. An owner-approved
exception to its no-external-request boundary and an independently
qualified helper profile would be required before any cloud send.
The local preview stays available under its own unchanged contract.

If adopted, register the site-scoped Boolean
`cloud_helper.base_preview` as `owner_only` and `route_out` in 0014.
Default it off. Show **Allow cloud help for link-value previews**
in Engine Room. Off keeps the ordinary action local. On allows
preparing one exact job for the verified helper and mailbox. The
desktop owner confirms the public projection, destination, time
limit and maximum charge for that job. Recheck both approvals
at send and result release. Off fences pending sends and late
output. Scripts and MCP can see safe setting help and cannot
change or approve the route. There is no automatic paid fallback.
Export only
the frozen complete graph of already public links with opaque
job-local node labels, directed edges and admitted public-derived
numeric weights. Verify the QR-153 numerical profile and send only
its admitted non-sensitive fields. Keep URLs,
native identities, revisions and their map, permission records, site
analytics, drafts and all proposed edit and scenario inputs on the
PC. An opaque label does not make topology anonymous. Require
explicit release of the whole proposed graph to that helper and
independent evidence that every edge, weight and teleport input
comes from permitted public material. Withhold the projection on
unknown coverage or private derivation. Use no truncated graph,
perturbed weights or replacement model.

Use the 0118 recipient and key-epoch rules. Require a separately
admitted bounded mailbox encoding under QR-242. The signed site
and local worker profiles give no mailbox authority. Bind these
records before a send or before opening a reply.

| Record | Required payload under the proposed profile |
| --- | --- |
| Local plan | Site, actor and current grant generations, native graph and rights digest, graph-to-label map, exact approved projected digest, model, recipient and mailbox proof refs, job and attempt IDs, current fence, deadline, quote and reserved budget. Keep this record on the PC. |
| Sealed work | Exact route and protection profile, opaque job and attempt IDs, fence, expiry, sender and recipient key epochs, digest of the projected bytes, model profile, ordered node labels and directed edges with exact approved weight data. Include no native identity or scenario. |
| Sealed result | Same job and attempt, fence, original expiry, work digest and model profile, worker identity, allowed numeric evidence, ordered base mass and resource receipt. Exactly one row per node in order. Bind the reply to the receiving PC. |

Check current provider account, mailbox namespace and helper key
epoch before sealing or uploading. The compute worker takes no
site credentials or network access and runs no scripts. The
mailbox operator can move sealed records and sees no plaintext.
Return only the approved numeric result and bounded evidence.
The PC verifies the receipt and current context, and independently
checks vector shape, finite nonnegative mass and total. Recompute
the outward 0027 residual against the exact pinned base graph
and model before using the vector. Compute every scenario and
publish the existing advisory view on the PC under current rights.
The result gives no ranking, training, crawl or native write authority.

Use the admitted 0027 SIM profile, including its existing bounds for
one run, under the 0019 lease and 0078 resource checks. Qualify the
actual helper build and host. Count its base work, local scenarios,
mailbox time, protected scratch and result transport in one run.
The split cannot renew the deadline or add a second allowance.
Keep a dated maximum in one approved currency for the whole attempt,
including helper compute, mailbox operations, transfer and cleanup.
A storage quote alone covers no compute. Before dispatch atomically
reserve the exact approved amount under the admitted account cap.
Hold on unknown cost, expired approval or insufficient budget.
On unknown delivery retain the
original attempt and charge reservation. Release them only on
exact settlement or proof of no send after fencing the old sender.
On pause, expiry, cancellation, lost scope or erasure, fence the
pending result and track every owned copy under 0030/0032. A late
receipt can reconcile only the original effect. It cannot admit
late output, show that remote work never started or allow another
send. Report uncertain delivery, cost and deletion separately
until each is verified.

| Outcome | Owner message |
| --- | --- |
| Off or profile unavailable | “Cloud help is unavailable. Run this preview on your PC.” |
| Maximum unproved and no send | “We can’t verify the maximum charge. This request was not sent.” |
| Delivery uncertain | “We can’t verify delivery. The approved amount is reserved.” |
| Billing uncertain | “We can’t verify the charge. The approved amount is reserved.” |
| Result proof fails | “We couldn’t verify the helper’s result. You can start a local preview.” |

A new local preview uses its own ordinary authority and neither
settles nor reopens the original paid attempt.

**Synthetic qualification.** Use SIM-01 with a local mailbox stand-in,
no network and no paid service. For the three-node cycle require
base mass `1/3` at each node within the proved bounds. Calculate the
removal scenario locally with mass `4/17`, `6/17` and `7/17` within
its proved bounds. Check that the delta intervals include `-5/51`,
`1/51` and `4/51`, respectively. Require loss, gain and gain
with intervals excluding zero. Compare the local-only and split
runs and record combined resource results under the test profile. Separately
plant a private or wrong-site input, an unproved profile, a wrong
recipient, a valid reply with another input digest, a wrong vector,
and a result after revocation. Each affected case withholds output.
Set the remaining admitted budget to the approved maximum, then
one smallest approved currency unit below it in a separate run.
Allow one reservation at equality and hold the lower-budget run
before dispatch. Change the account or key after preview in another
run and prove no send. Lose a send reply, cancel and restart. Retain the
original charge hold and reject a late result. Reconcile the exact
transport receipt once. Verify scratch cleanup and prove no
scenario inputs leave the PC. No site
write, release or ranking effect follows. This fixture cannot approve
a provider, recipient, live route or product adoption.

[cloud-base-preview]:
  0027-candidates-ranking.md#batch-link-value-simulator-feature-1

### Site identity and isolation

1. One installation may register several authorised sites of either
   platform. Each site has its own fixed internal identity, platform
   identity, credential scope, permissions, queue partition, index
   namespace, backup catalogue and audit stream. Owning the
   installation is a separate authority.
2. Site identity is never taken from a submitted URL, host header,
   numeric content ID or any field the caller picks. Trusted dispatch
   sets it.
3. Every Sutradhar table, including management, queue and audit tables,
   has forced row-level security. Pooled connections clear and set the
   site again before reuse. Object paths, cache keys, vector lookups,
   events, exports and failure reports keep the same separation outside
   SQL.
4. Public, fixed model weights may share one loaded process. Site
   content, embeddings, learned values and private fine-tuning are
   never pooled. Cross-site batches keep separate authority, outputs,
   errors and accounting; if separation is uncertain the batch is
   refused. No site can learn that another exists from resource names
   or diagnostics.
5. Separate installations, if chosen, isolate credentials, storage,
   queues and model processes. Their extra memory and disk are measured
   and shown to the owner first, and they get no second budget by
   default. Work is never moved elsewhere to hide its cost.
6. Site credentials must not be readable by another site's PHP process.
   Qualification inspects PHP users, file access and key storage. If
   one hosting account can read several sites' keys, protection from a
   compromised site is not claimed; an approved isolation change or a
   separate qualified set-up is needed. Hosting pools and permissions
   are never changed silently.

### The XenForo edge

1. The approved XenForo rules stay: guest visibility is cross-checked
   against XenForo; writes use XenForo's supported mechanisms; a link
   wraps existing words and by default never rewrites text.
2. The member-text option, on from the start, lets a moderator approve
   each change to post text. Undo and recheck still apply.
3. At save, current visibility, content and permission are checked
   again. Protected BBCode, duplicate-target, signature and undo rules
   stay. Adding WordPress changes none of this and never allows direct
   writes to XenForo's database. No site database is moved.

### The WordPress edge

Eligibility:

1. Input: the full stored body, installation, site, platform, post ID,
   type and version, publication and password state, visibility facts,
   actor and object capabilities, target facts and the qualified
   block and plugin profile. Output: a site-scoped eligible record, or
   a refusal with a safe reason and version. Private text never appears
   in refusals or logs.
2. Only truly public, published, password-free items are indexed or
   linked. Actual anonymous visibility is checked, including registered
   restrictions; a `publish` flag alone is not enough. Drafts,
   scheduled, waiting-for-review, private, trashed, revision and
   password items are left out. Source and target both meet the rules.
3. Posts and pages have their own test fixtures. A custom post type
   needs explicit registration, its real capability mapping and its own
   qualification; an unknown type is refused.
4. Multisite maps each blog to its own Sutradhar site. Every operation,
   cache, key, event and index is bound to it; equal numeric IDs never
   cross. Network super-administration grants no other site's keys or
   installation control. Multisite stays unqualified until its own
   isolation and lifecycle cases pass; this never shrinks release scope.
5. A change of status, password, visibility, restriction, capability or
   content invalidates affected work: indexes, caches and queued
   approvals are reconciled, and a once-public item cannot be seen in a
   preview. If state is unknown or invalidation unfinished, work holds.

Validation (a separate Rust HTML and block validator):

1. It wraps only existing visible words, keeping their exact bytes and
   order. No nested links, rewriting, added or removed text, or changes
   to existing anchors. Duplicate-target and existing-link limits stay.
2. It never touches attributes, comments, scripts, styles, code, pre,
   blockquote, anchors, shortcodes, protected regions, block delimiters
   or JSON block attributes. Rendered shortcode or dynamic output is
   not editable text. An anchor never crosses a protected or structural
   boundary.
3. Block types are admitted from a versioned allowlist, each only after
   an editor round trip passes. An unknown, dynamic or plugin-defined
   block refuses linking in the whole item, including ordinary
   paragraphs beside it. Failed parsing, bad nesting or uncertain
   visible text also refuses the item. There is no best-effort repair.
4. Example: a post with a normal paragraph and an unknown plugin block
   gets no Sutradhar links at all. The owner can still edit it; the app
   explains why it is held.
5. A Rust text diff alone never qualifies editor compatibility: the
   real pinned block editor and plugins, with their stored-markup
   checks, are tested too.

[paired-outbox]:
  0021-forum-contracts-ingestion.md#noticing-changes-the-outbox

Events and pause:

1. Save, update, delete and status or visibility events carry the site,
   a stable event ID, object and version, cause, and a deletion
   tombstone where needed. Event data is only a hint to reread facts.
2. Delivery is at least once, with durable acknowledgement,
   de-duplication, bounded retry and a dead-letter list. Out-of-order
   events never bring back deleted or newly private content. Lost
   events are found by checkpointed reconciliation with native state.
   Use [spec 0021's outbox rules][paired-outbox] for a present saved
   event the reader cannot interpret. Keep the
   affected acknowledgement at its proved boundary. The missing-event
   timer cannot drop a present unsupported event or admit its reply.
3. Capture is qualified across crashes before and after the native save
   and the acknowledgement. Human saves are never lost while the
   service is away. A possible capture gap holds affected work.
4. Recursion guards name the Sutradhar operation and version; required
   hooks and revisions stay on. Hooks are never suppressed globally.
5. Pause is a durable per-site decision, checked by the add-on before
   every apply and undo, including after restarts, delayed callbacks
   and old approvals. Resume needs current authority and fresh checks
   and never revives expired approvals. An unreadable pause state
   refuses Sutradhar writes. Installation maintenance is separate.

Conditional save:

1. Input: the signed Rust plan, current source and target facts, actor
   capabilities, authority version, pause and operation ID. Right before
   applying, all are checked with the full content version and hash.
   The saved result must match Rust's approved hash.
2. Every approval binds installation, site, platform, item, content
   version and hash, actor, action, target visibility and expiry. No
   authority comes from a route name or a caller-supplied capability.
   An administrator role or approval token never skips the object
   capability check.
3. Prahari, an independent content check, also signs each change, bound
   to the same site, platform, item, operation and before-and-after
   bytes. A missing, stale or negative Prahari verdict refuses the save
   and returns it to review. Two signatures never override current
   native permission or a human edit.
4. `wp_update_post` alone does not prove atomic compare-and-save. A
   supported protocol must be qualified against every admitted writer
   and hook, including edits between read and save. A plugin-local lock
   is not enough. Without it, writes stay off: no direct SQL, silent
   overwrite or disabled hooks.
5. Saving keeps unrelated fields, escaping, Unicode, revisions and hooks.
   A hook that changes the approved result fails qualification.
6. A journal record comes before any ambiguous effect. A retry
   reconciles the native receipt; the same operation never applies two
   links, and a reused ID with a different body is refused. An
   uncertain outcome reports "reconciliation needed", never success and
   never a blind retry. The admitted protocol must prove the link
   effect and receipt together. Use the signed bounded lookup of
   spec 0021. Prove both outcomes before enabling apply or undo:
   one exact committed effect, and a verified pre-save refusal.
   An unknown outcome holds. The WordPress receipt lifetime is
   part of the same privacy and recovery admission.

Selective undo:

1. Input: a scoped journal entry plus full current content,
   permissions, visibility, pause and version. Output: the exact
   inverse change, an already-undone receipt, or a conflict with no
   write.
2. The journal records site, platform, item, actor, operation ID,
   before and after hashes and versions, the exact anchor bytes and
   target, structural location, native receipt and validation profile.
   Its bytes follow retention, erasure and encrypted-backup rules;
   whole posts never go into audit messages.
3. Undo removes only that operation's still-matching markup and keeps
   every later human word and unrelated link. Ambiguous matches, edited
   anchors, overlapping operations or broken blocks stop for a person.
   The result is validated and saved under the same conditional
   protocol.
4. WordPress revisions help recovery but never prove selective undo. An
   old whole revision is never restored over later edits. If a person
   later recreates identical markup, the exact committed undo receipt
   still belongs to its original native revision. Replay retrieves the
   eligible historical receipt without saving again. Read current link
   presence separately. A lost journal or expired evidence blocks
   undo. Retention follows the privacy policy, not a copy of XenForo's
   receipt expiry.

For an eligible cross-site undo, apply the same WordPress source
checks and native save contract. Use only the source receipt and
current source edit permission under
[0119's existing undo route][cross-site-undo]. IT-0119-UNDO checks
an unreachable target after pair revocation and a separate source
permission refusal. The result covers the source effect only.

[cross-site-undo]:
  0119-cross-site-link-suggestions.md#applying-revoking-and-undoing

### sampark: the shared adapter interface

sampark ("contact") owns versioned platform identity, fact-reading,
event and conditional-apply interfaces, with separate XenForo and
WordPress implementations. Shared code takes typed facts; it never
erases platform differences or treats a missing fact as public.

| Contract | Inputs | Outputs and authority |
| --- | --- | --- |
| Read facts | Bound site, object, version, native context | Typed facts |
| Deliver event | Site, event ID, version, cause, tombstone | Durable receipt |
| Plan change | Exact source and target, actor, policy | Rust-approved plan |
| Apply change | Signed plan, current facts, pause | Native save receipt |
| Undo change | Journal ID, current facts and authority | Receipt or conflict |

1. Facts name installation, site, platform and version, object type and
   ID, source bytes and hash, visibility, permission version, parse
   format and native save behaviour. Sensitive bytes stay on the engine
   path, never in telemetry or ganit.
2. XenForo keeps its BBCode validator and supported writes; WordPress
   has its own HTML and block validator. Neither is turned into the
   other. Scheduling, signing, journalling and validation orchestration
   are shared where safe and never reimplemented in PHP.
3. Facts, events, plans and receipts are versioned. Old and new add-on
   versions are tested together; unknown formats, or unknown fields that
   could widen authority, are refused. Missing facts, unsupported
   versions, stale content, pause, bad signatures, lost journals and
   unsuited hooks refuse writes. An outage never becomes an empty
   success; stale or missing state is shown honestly.
4. Small Rust interfaces cover site identity, platform facts,
   conditional apply, the release feed, the backup store and registered
   resource control. Each has a behaviour contract and an independent
   fake. The existing action, settings and history, progress, queue
   and audit systems are reused; there is no second management stack.
   Each rule lives in one owning module.

### Who owns each task

| Task | Where | Authority |
| --- | --- | --- |
| Install, update, backup, repair, removal | Desktop app | Rust lifecycle |
| Engine settings and setting history | Desktop app | vinyas Rust API |
| Dashboards and resource summary | Desktop app | Shared Rust facts |
| Add-on setup, pairing confirm, site pause | Site admin pages | Rust actions |
| Accept or refuse suggestions | Site admin pages | Rust actions |
| Connection state and last contact | Site admin pages | Shared Rust facts |

1. Each task has one editor. Site pages may show read-only engine
   summaries but never a second editor or settings store. Neither side
   widens the other's permissions. The add-on has no engine logic.
2. Site pages stay useful when the service is unreachable: they show
   the last known state and keep pause working. For example, a site
   administrator pausing their WordPress site changes nothing on the
   XenForo forum and gives no right to update the installation. For
   XenForo, show last signed contact separately from the heartbeat
   condition in spec 0021's heartbeat contract.
   A current contact can accompany the existing SUT-340 notice.
3. Screens use SDS components. Site pages keep the platform's own
   navigation and permission checks.

### Site routes and browser trust

1. The add-on's pages and signed routes sit under the site's existing
   HTTPS hostname, preferably beneath `/sutradhar/`, subject to
   qualification. Preflight lists existing routes, rewrites, redirects,
   subfolder and multisite mappings, cookies, caches, proxy prefixes,
   origin checks and timeouts. Any hosting change is shown for separate
   approval, and the original set-up is kept. A clash blocks that set-up
   with a clear reason. No new domain and no site move is needed.
2. Subfolder installs keep their real base path; the site never is
   assumed to own `/`.
3. Mutually untrusted sites need separate qualified hostnames. Folders
   on one hostname do not isolate page scripts. A site and its add-on
   pages are one browser trust boundary. Distinct hostnames alone do not
   prove cookie or server isolation: parent-domain cookie injection,
   host-header swaps and redirects are tested.

### Identity, review grants and sessions

1. XenForo keeps OAuth2, with its existing assent step, to establish the
   moderator's identity. WordPress uses its native signed-in identity.
   After that step, the add-on issues a scoped, single-use review grant.
   A shared domain or holding a link never proves identity.
2. Input: fresh native identity, installation and site, current
   capabilities and the intended recipient. Output: a restricted
   Sutradhar grant for that actor and site only, never platform admin
   or installation ownership from a role name. Review permission never
   needs unrelated admin rights.
3. The grant binds issuer, audience, installation, site, platform,
   actor, session, purpose, operation, expiry, authority version and a
   unique ID. A versioned policy sets lifetime, clock tolerance, sizes
   and rates before it is switched on. If expiry or replay state is
   unreliable, sign-in is refused.
4. dwarpal verifies native authority and consumes the grant atomically.
   Two submissions give at most one grant. A lost-reply retry may fetch
   the same receipt only for the same exchange. Used, expired, revoked,
   malformed, stale-epoch, wrong-purpose, wrong-recipient or cross-site
   material is refused and audited. A WordPress nonce is never one-use
   authentication.
5. Secrets never go in query strings, reusable links, logs, referrers,
   analytics or support packs. No open redirect. The pairing master key
   never reaches page scripts.
6. A review grant lasts at most 12 hours and ends after 30 minutes idle.
   The five-minute progress-viewer token keeps its separate purpose.
   Cookies are HTTPS-only, Secure, HttpOnly and host-only, with an
   explicit SameSite and path policy; a narrow path is not a boundary.
7. Origin and CSRF checks cover forms and every state-changing request.
   Browser-supplied scope and capability IDs are untrusted.
8. Logout ends the grant. Native logout, role loss, site revocation and
   a pairing-epoch change end related grants and queued approvals. The
   save boundary rechecks authority even before that news arrives.
   Unrelated sites and outside OAuth connectors are not signed out.
9. If identity, permission, replay, audit or revocation state cannot be
   read, new consequential authority is refused with an explanation.
   The owner's emergency sign-in grants no content permission the
   platform cannot verify.

### Pairing, keys and revocation

1. The installation owner (in the app) and the site administrator (in
   the site's admin pages) both confirm the site, installation and
   requested permissions. Pairing approval is short-lived, single-use,
   consumed atomically and bound to both. A lost reply returns the same
   receipt; it never issues a second credential or pairs another site.
2. Key material moves only through the signed pairing exchange, never a
   prompt, URL query, analytics, log or support bundle.
3. Keys are separated by purpose and direction: sign-in grant, session
   authority, content approval and management, each with site,
   installation and key-version separation. Viewer tokens, signed
   messages and pairing approvals never stand in for another purpose.
4. Every management request binds installation and site, actor,
   action, object, body hash, fresh request ID, validity and authority
   epoch. Replay and repeat state are checked before any effect. A
   reused ID with different content is refused. A timeout never becomes
   permission.
5. WordPress registers only its own versioned REST namespace, with
   native permission callbacks, plain and subfolder permalinks, and a
   signature over the exact method, path, query and body. No
   application passwords, user listing, core users route, browser
   cookies or open route.
6. Revocation stops queued work and grants on the old epoch. Rotation
   is checkpointed and audited and keeps its approved overlap; revoked
   keys stay revoked during overlap and after an old backup is
   restored. Re-pairing, replacing trust or changing ownership needs
   explicit administrator approval and is never a repair fallback.
   Knowing a site URL or holding a diagnostic report grants nothing.

### Traffic, blocks and retries

1. Per-site and installation-wide rate, burst and concurrency limits,
   bounded batch bytes and items, connect and read deadlines and fair
   scheduling between sites. All values are vinyas settings, measured
   before traffic is switched on.
2. On 403 or 429: bounded exponential back-off with jitter. A valid
   Retry-After is honoured; if it passes a deadline, the work holds as
   blocked. A bad header never causes a retry loop. Attempts and due
   times survive restarts. Unfinished work is never deleted. An
   uncertain save is reconciled first, then retried with the same
   operation ID and a fresh signature. A retry never revives expired
   approval or revoked authority.
3. Wordfence, Sucuri, Cloudflare, ModSecurity and Imunify360 blocks and
   challenges are recognised, even inside 200 or redirect replies, with
   bounded bytes and CPU and versioned rules. The state is `blocked`
   with a safe reason, confidence, next check and guide. Unknown
   replies stay "unavailable", with no guessed vendor. Signature, TLS,
   schema and code faults keep their own reasons.
4. No origin-address fallback, challenge solving, identity rotation,
   TLS bypass, global allowlist or turning off a security plugin.

For cross-site suggestions, both adapters admit the distinct spec
0119 typed source operation only with an atomic native epoch, edit
and receipt fence. An unsupported adapter holds that operation.

### Instruction-only edits

1. A write request holds platform, installation and site, object ID,
   operation ID, source version and SHA-256, actor and approval
   reference, authority epoch, validation profile, the range and
   target, the expected SHA-256 of the full result, and expiry.
2. Ranges use Unicode code-point offsets bound to the exact stored
   source. Mixed units and implicit normalisation are refused.
3. A write request never carries an HTML fragment, BBCode string, new
   post body, SQL, code or attribute map. Targets are validated
   references under the public-visibility and outbound-address rules.
4. Rust picks the words, target and validation result. The add-on
   checks the request, object authority, pause, visibility, language and
   source version, then builds the native markup mechanically, keeping
   every byte outside the wrap. It never picks other words or repairs
   content. The whole proposed body must hash to Rust's approval.
5. A filter that would change the body refuses the save before it is
   stored. No save-then-repair. The stored result is rechecked against
   the approved hash.
6. Both ends negotiate the edit-contract version. Unknown versions,
   old markup fields and weaker profiles refuse writes.

### Non-invasive integration

1. XenForo uses official code event listeners and class extensions
   that call the original. WordPress uses hooks, filters and registered
   REST routes. Call order is qualified. No core edits, replaced core
   services or direct changes to host tables; content writes use host
   functions. Widgets and template hooks are preferred; any template
   change is version-specific and tested.
2. Outputs are normal native save events, edit history, index updates
   and targeted cache invalidation; other add-ons see a normal edit.
3. Page rendering never calls the service. It may only queue bounded
   background work. A missing service never breaks visitor pages.
4. Host cache interfaces with namespaced, expiring keys; never a global
   flush. After a write, only affected objects are invalidated (with
   `clean_post_cache` where the save flow needs it). Downstream plugin
   and CDN behaviour is qualified per version; missing evidence blocks
   that write combination.
5. Cacheable HTML holds no personal state. Per-user data comes from
   scoped uncached routes. Platform file interfaces only; nothing
   depends on one web server's local files. Several web servers see
   repeat-safe events and the current pause.
6. Tables, options, routes, templates, CSS and page scripts are
   namespaced. CSS is scoped to owned roots; scripts are deferred, add
   no browser globals and respect the content security policy. No other
   plugin's settings are touched.
7. Every hook has an error boundary with bounded, redacted logs.
   Visitors never see raw errors. Display additions fail open; content
   writes fail closed. Programming errors never become a success.
8. Safe mode turns off Sutradhar's visitor-facing additions when the
   add-on's error threshold is passed or the host version is untested.
   Admin pages, pause, evidence and alerts stay, and the admin page
   says why and what to do. Threshold, window, hysteresis, persistence
   and reactivation are governed settings. A further safe-mode part
   makes no changes, runs no helpers and spends nothing on the cloud,
   entered by itself in only two cases. The trash keeps items 90 days.

### Plugin coexistence

1. Setup and scheduled health checks build a site-scoped report from
   the native read-only inventory: identity, profile fingerprint, time,
   evidence, affected operations, safe reason, guide and next check.
   Absent, unknown, blocked and qualified are kept apart. Users routes
   are never probed. Detection alone is not compatibility.
2. Detected: Wordfence, Sucuri, Solid Security, All-In-One WP Security,
   WP Rocket, LiteSpeed Cache, W3 Total Cache, Autoptimize, Yoast, Rank
   Math, Link Whisper, Elementor, Divi, WPBakery, WPML and Polylang.
   Cloudflare, ModSecurity and Imunify360 are outside protection
   profiles. Renames, editions, must-use and network plugins and Divi
   modes need versioned fixtures.
3. Checks never save another plugin's options, call its management
   interface, edit its files, flush its caches or change hosting or
   security settings. Health checks share the traffic limits.
4. Inventory, upgrade, settings, language or ownership changes
   invalidate affected qualification and queued work.
5. Builder-managed items (Elementor, Divi, WPBakery) are skipped with a
   plain reason, never flattened. If ordinary content cannot be told
   apart from builder content, the item holds.
6. Existing links, including Link Whisper's, are never removed,
   replaced, nested or recreated. SEO plugins keep their structured
   data, canonical and language tags; Sutradhar never duplicates or
   overrides them.
7. Both platforms apply spec 0018's British-English eligibility
   rule and verify its current facts at apply (spec 0021).
   A native language plugin may supply qualifying evidence under
   the same rule. This guard adds no translation or cross-site path.
8. The counting asset: cached HTML holds only the script reference and
   non-sensitive page identity. The page's script fetches a fresh
   page-bound darshak token from a narrow uncached route that grants no
   write or staff authority, checks guest visibility, origin and
   eligibility, limits issue rate and keeps consent, GPC and DNT rules.
   Replies are no-store and private, and the whole cache path is
   tested. A challenge, cached reply or failed fetch is never a
   confirmed human visit. Token lifetime, signature and rotation stay.
9. Only Sutradhar's own asset is marked with vendor-supported exclusion
   hints; the owner is guided where a manual exclusion is needed.
10. Scheduled work runs from a server-triggered runner (Action
    Scheduler driven by server cron, or another native runner) and
    must progress with zero page visits. WP-Cron is never disabled,
    other plugins' actions never run, force flags are never used and no
    second Action Scheduler copy is installed. Server cron set-up is an
    owner step. Queue entries bind site, operation, source version, due
    time, attempt, lease and state. A missing runner reports blocked
    and never falls back to page visits.

### Installing

1. The app explains requirements, expected resource use and what the
   owner will approve. It obtains an exact qualified release and shows
   compatibility and source and licence notices.
2. The installer's steps are checkpointed and safe to restart:
   1. Establish release trust independently; verify bootstrap material
      before running it, or stop with a trust set-up action.
   2. Check OS, architecture, admitted process controls, PostgreSQL,
      disk, available memory, outgoing connectivity and protected link.
   3. Confirm the plan still matches the PC and the approved authority.
   4. Create approved owner-private storage, ledger and process
      budgets without altering other services or applications.
   5. Fetch exact models and data, check their rights and verify each.
   6. Start only registered Sutradhar parts in the budget. Use 0030
      Installation step 6 to check current memory and enforced process
      limits before starting PostgreSQL. Recheck current capacity and
      verify the existing migration reservation before any migration
      SQL. Do both under the verified native setup intent. Create
      PostgreSQL roles and forced RLS through forward-only migrations,
      with separate schema-owner and app roles. Use only the verified
      installation checkpoint scope of specs 0030 and 0108 before
      pairing. A later site check needs its own exact site authority.
   7. Pair each site through the two-party confirmation.
   8. Run real connection, permission, write-boundary, pause, recovery
      and made-up-data checks. No real content is written as a test.
   9. Show the steps finished, evidence, the remaining work and the
      next useful action. Set-up succeeds only when all checks pass.
3. Each resource is recorded before its effect could be ambiguous. A
   retry reconciles the real resource and receipt, never recreates it.
   An ownership mismatch or unknown state pauses for diagnosis.
4. The add-on's own preflight checks only facts its platform can
   report; it never runs shell commands. Unknown facts show as unknown.
   No command or download is offered for a release that does not exist.
5. No unverified download-and-run, silent package install, firewall
   change, proxy swap, service shutdown or public management port.
6. The installer checks free capacity as well as Sutradhar's cap. A
   shortage is shown; it waits or refuses without stopping anything.
   No number of simultaneous sites is advertised before it is measured.

### Release trust and updates

1. Release metadata follows TUF 1.0.33: root metadata names keys and
   thresholds; targets list artefacts; snapshot binds versions;
   timestamp gives freshness. Downloads are bounded.
2. The owner's offline root, the online distribution metadata and the
   Dell qualification authority are kept apart. Thresholds, storage,
   rotation and recovery are set by review. Root recovery material never
   sits with the online publisher or agent clones.
3. Two claims stay distinct: the Dell release verdict (exact source,
   checks, tools, rules, scope and evidence limits) and the signed
   distribution metadata (manifest, hashes, sizes, release identity).
   The client needs both. A checksum proves neither. Public receipts
   expose no private data or site IDs.
4. First install needs a trust seed set up independently: an earlier
   trusted release or an owner record checked out of band. A new
   download never vouches for itself. Online and offline installs use
   the same manifest and rules; paths, sizes, links and decompression
   are bounded. An offline bundle missing an artefact is incomplete.
5. Releases come only from the Dell publisher, from exact versioned
   artefacts; never from `main` or a "latest" tag. Dell takes no
   incoming internet connection: CI and publishing state are polled
   outward, with the event saved before the cursor moves and run IDs
   used to avoid duplicate issues. Failures keep the cursor for retry
   and show a stalled-publisher issue.
6. Update checks send no site URL, content, member data, installation
   ID, token or site count. The app explains that the release server
   still sees the PC's public address. Loss of a source required by
   the chosen route or expiry of required metadata blocks that install
   or update. The current service can keep running. An independently
   verified offline bundle uses its own admitted proof and expiry.
7. Notices start on and automatic install starts off. A setting filters
   notices to security releases or all releases. Under DR-149 only the
   installation owner can approve the exact app update plan. The same
   confirmation applies to a downloaded bundle. Unattended install
   would need a new approval.
8. The notice shows what changed, each site's compatibility, the
   expected interruption, resource and disk needs and the recovery
   plan. The plan pins sites, releases, schema and state versions,
   artefacts, backup evidence and permissions; any change voids it.
9. Applying: take the durable operation lease, fence stale actors,
   verify the whole release and check sites, resources and backups.
   Use the exact per-volume workspace admission in spec 0030 and
   0033's disk rules before pausing affected writes. Checkpoint the
   pause. Recheck disk proof before a space-consuming stage or
   fallback. A failed check keeps the old state or a clearly recorded
   paused state. Unproved journal state holds continuation.
10. Old and new run together only if their measured combined peak fits;
    otherwise the owner is offered a maintenance window. Limits are
    never lifted for an update.
11. Schema changes are forward-only expansion, then self-tests of
    content, permission, queue, observability and recovery. After a full
    pass, recheck current approvals and changed content. Verify the
    0030 per-volume reservation and durable journal at the final switch
    and write resume. Hold the affected step on an unproved check.
    Contraction needs its own compatibility window and evidence.
12. Under the current operation lease, check the exact fallback payload
    against current trust, schema, persisted state formats, journal and
    every site's compatibility. Immediately before launching the
    previous app, recheck that each registered site's exact native
    compatibility and pairing proof is current under the same lease.
    An earlier checkpoint or unchanged version label supplies no
    current proof. Verify the exact persisted effect-journal and receipt
    formats against the previous payload's admitted reader profile.
    An unsupported journal remains held before launch. Retain its bytes
    and original receipt. A compatible reader reconciles a verified
    terminal effect under 0021 without issuing it again. After
    encryption cutover the build and store must pass spec 0118.
    Under the same lease, verify the current protected retirement
    and erasure checkpoints from 0030 and 0032 outside the fallback
    data. Prove that the exact previous payload and its admitted
    read route can enforce the current history for each registered
    site. Bind the required stored history format and checkpoints
    to the reader proof. Recheck before starting the previous
    payload and before admitting protected work or resuming writes.
    Hold a missing or incompatible required history format before
    launch. A verified previous-app start records its own effect only.
    Before write resume and a verified return, match the current
    database schema and state formats to the exact fallback plan and
    previous app again. Keep any later incompatibility on the same
    paused update. An older app restarts automatically only with all that
    proof. The three-minute return target applies only then.
    Otherwise keep writes paused, send no native effect from the
    paused update and show the failed or unproved check. Migrations
    never run backwards, and an old backup is never
    restored automatically. The owner-only backup-check
    override stays, with reason and audit, but never overrides release
    trust, required checks, limits, data safety or compatibility.
13. The lifecycle part updates as its own verified artefact: staged,
    checked, with the previous copy kept and the journal preserved.
    Only one copy holds authority. Every interruption leaves either a
    working lifecycle part or a verified recovery route. Metadata
    versions are recorded to refuse rollback, freeze, mix-and-match and
    substitution. A verified staged-file effect supplies no proof of
    the current lifecycle holder. Before either copy can act after
    interruption, reconcile the original hand-over under the current
    lease and admitted native profile. Keep an unproved holder on
    the existing verified recovery route without starting either
    copy with authority. A bad clock blocks updates; the installer
    never changes system time.
14. Signing-root recovery and site-pairing recovery are separate. An old
    backup never brings back revoked keys or older trust.

### Running, pause and repair

1. One durable installation lease excludes competing update, repair,
   restore and removal. Site operations take narrower leases and never
   override global maintenance. A crash does not free the lease until
   the journal and owned resources are reconciled.
2. Automatic recovery covers only qualified restart, interrupted-work
   and passing-dependency cases, with bounded attempts and back-off.
   Repeated failure stops, keeps queued and failed work, and raises an
   issue. Recovery never deletes work to empty a queue.
3. App actions: Check installation, Restart processing, Rebuild a
   derived index, Retry a failed operation, Download diagnostics. Each
   explains its effect, interruption and risk first. Destructive or
   trust changes need explicit approval. Broad root repair is never the
   default.
4. The lifecycle part is a separate Rust program from the engine, with
   fixed typed verbs only: no shell text, file paths or names chosen by
   a site. Global verbs: plan and apply install, plan and apply update,
   bounded restart, plan and apply restore, lifecycle update, plan and
   apply removal. Each plan binds release, resources, sites, settings
   and expiry; each verb has its own permission, inputs, effects,
   bounds, failure and recovery contract.

| Principal | Allowed scope | Never allowed |
| --- | --- | --- |
| Reviewer | Their site's review | Host, trust, other sites |
| Site administrator | Their site's management | Global removal |
| Installation owner | Reviewed installation actions | Other applications |
| Authorised agent | An approved plan | Publishing keys |
| Lifecycle part | Registered resources, fixed verbs | Arbitrary shell |

5. Site actions: status, pause and resume, scoped diagnostics, export,
   site backup request, retry of authorised work and disconnect. A site
   credential can never ask for a global verb.

### Backups and restore

1. A site backup holds its durable state, schema, roles, policies,
   receipts, change and undo history and settings. Installation backups
   add registered-resource and recovery journals. What can be rebuilt
   (indexes, public models) is listed and never called a backup.
2. Backups are encrypted with owner-held recovery material, stored
   locally or optionally off the PC with clear privacy and retention
   settings. Protected evidence is never swept up by expiry. Integrity
   is verified before success is reported; restore drills prove it.
3. Signing keys and pairing secrets have their own protected backup and
   never enter a support bundle. A hardware-bound key is not promised
   portable: replace, revoke and re-pair with approval. The owner is
   told which recovery material to keep and what loss each covers.
4. Restore goes into an isolated target, checks integrity, site
   separation, RLS, compatibility and app checks, then proposes a
   switch. Later work is never lost by an automatic restore.
5. Report restart compatibility and restore compatibility separately.
   A previous app proved against the current state under item 12
   supplies only restart evidence. For restore, match the selected
   backup's manifest, archive format, site and recovery authority to
   its admitted reader and exact isolated target. Apply the current
   protected history of spec 0030, minimum key epoch of spec 0118,
   and exact storage proof of spec 0038 where applicable. A missing
   or incompatible reader, authority or proof holds the affected
   restore before activation. Keep eligible live state and later work.
   Name the failed check on Restore. Let the owner select an eligible
   backup or use an existing separately authorised isolation route
   when one is available. If restart is held on schema compatibility,
   a separate restore must still pass its own plan, lease and checks.
   Reconcile the held update before taking a restore lease. Keep its
   compatibility outcome. An isolated restore supplies evidence for
   that restore only and cannot reopen live writes or certify rollback.
6. For an already admitted live switch, audit the proposed exact
   effect before publication. The audit records pending work and
   cannot prove live success. If the audit write fails, refuse with
   SUT-848 and keep the live pointer and existing holds. After a
   successful audit, still compare current history under 0030's
   guarded publication. A newly advanced history fence holds that
   stage. At the switch effect, prove that the native route still
   targets the backing object checked in the exact isolated restore.
   Use its matching volume and managed ledger evidence under 0038
   and the exact owner-approved switch plan and current lease.
   Qualify the at-use root identity in the admitted platform profile
   under QR-202. Reopening a path after the final check needs the
   same proof. A readable substitute, even on the same volume,
   cannot inherit the isolated result or pending audit. Hold such a
   switch. Keep the live pointer, later work and isolated evidence.
   A verified effect receipt must bind the exact object switched,
   independently of the pending audit and isolated result. After
   interruption, reconcile that original effect under current read
   authority. Recheck the exact backing object and native volume
   before opening the restored data. If either is unavailable or
   differs, hold affected access and keep the recorded placement
   and receipt. A historical receipt cannot admit a substitute
   or authorise another switch.

### Removal

1. Three choices: Disconnect this site, Remove this integration, Remove
   the whole installation. The app shows the scope, affected
   resources, exports, kept and deleted categories and shared parts.
   Kept categories default to Keep. Backup or export is offered first;
   a failed export or unclear ownership blocks deletion.
2. Writes are paused and fenced, in-flight work settled, site keys and
   grants revoked, and an interruption-safe plan recorded. Only
   registered resources are removed, each verified.
3. Disconnecting one site leaves every other site intact. Models,
   indexes and embeddings are deleted only within the authorised scope;
   shared models stay while another site uses them.
4. Removing an integration never silently removes the service. Whole
   removal needs installation authority and each affected site's
   confirmation. The lifecycle part stays with its journal while
   a required deletion or stop is unproved. It goes last after the
   verified completion result is durable. Choosing no final receipt
   file still leaves the journal available during any hold.
   Interruptions stay resumable under the exact removal plan.
5. Approved links stay unless undo is asked for and can keep later
   edits. Shared system parts, hosting files, other applications and
   protected storage are never removed. Retained backups, audit and
   evidence are listed with their reason.

### Diagnostics and audit

1. Diagnostics come from an allowlist: release and digest, redacted
   checks, bounded resource summaries, step and error codes and safe
   correlation IDs. Content, member details, keys, pairing material,
   arbitrary files and private paths are left out by construction. The
   exact bundle is previewed and its bytes scanned before sharing.
2. Consequential approvals, refusals, settings and trust changes and
   state changes are audited with site, actor, operation and evidence.
   A failed audit write refuses the action. Error data is bounded and
   attacker input is never echoed into logs. A failed notice stays
   queued and shows as a notifier fault.

### Compatibility register, manuals and assistance packs

1. Each row pins OS, architecture, native process controller and
   limits, local transport and key custody, PostgreSQL, hosting, PHP,
   add-on, site database, release digests, models and topology. For an
   update or rollback, bind those rows to the exact schema and state
   format versions in the plan. Evidence names the exact tuple, data,
   limits, tool and rule versions and results. A vendor name or nearby
   version gives no evidence. A missing field blocks qualification.
   For first setup or an approved update, name the qualified
   installation database profile and its checkpoint and refusal cases.
   After enrolment, bind the current exact owner approval to any update
   using this scope. Site and connector work use their own authority.
2. Rows cover Ubuntu 26.04, Debian 13 and Windows 11 on x86-64,
   with XenForo 2.3 and both supported WordPress majors. Qualify
   shared and separate installs, custom types, multisite and plugin
   modes independently. Untrusted sites need separate hostnames and
   isolated accounts. ARM needs separate approval.
3. The manual has usable guides for: requirements, hosting and resource
   use, shared versus separate installs, rights and notices; installing
   and pairing from both admin panels; daily admin, review permissions,
   pause and resume and each action's effect on queues and other sites;
   update notices, interruption, compatibility, backup-override limits
   and conditional fallback; encrypted backups, restore tests, recovery
   points and key recovery; diagnosing service, network and database
   faults; and the three removal choices. It also has guides for 16
   named plugins and 3 outside protection systems.
4. Guides lead with plain words and an example, explain terms at first
   use, and never need Git or container knowledge to approve a plan.
   Screens show steps finished, work left, errors and the next action.
   Commands are reference only. The recovery guide covers the case with
   no engine and no window, the receipts to keep, and never says to
   delete data and start again.
5. Assistance packs, made from the chosen release and plan, are given to
   an AI helper the owner chooses. Each holds release and digests,
   official links, the environment, the outcome asked for, permitted
   steps, approval limits, checkpoints, stop conditions and how to fetch
   diagnostics. No passwords, keys, pairing secrets or content. A pack
   grants no permission; the software checks scope itself. A helper
   never opens ports, installs remote-access software, resets queues,
   weakens a gate or gets publishing keys. Each pack ends with what
   changed, checks, limits, work left and owner decisions, and stops
   safely when trust or authority is unclear. Six packs: first install,
   failed-install diagnosis, updating, repair of a stopped service,
   restore and removal, each following the rules above.
6. The register, manual, recovery guide, packs, notices and offline
   inventory ship with the same release. Links, version pins, secret
   exclusion, accessibility and parity with the real screens are
   tested. A stale pack says it is out of date and never installs from
   `main` or "latest".

## Limits

- Memory: use spec 0078's measured process and window budgets and
  headroom. Count the service, lifecycle, models, ganit, MCP, database,
  workers, backups and overlapping updates. Measure native allocation
  peaks, restore and rebuild. Swap cannot extend a supported budget.
- The agency edition adds a measured per-client allowance,
  `agency.memory_per_client`.
- WordPress: at most 8,388,608 bytes (8 MiB) extra peak memory per
  request, against the same site with the plugin off. XenForo keeps its
  8 MB rule. PHP allocation and resident peaks are both measured, with
  identical PHP, opcode cache, content and load; persistent overhead
  and concurrency are reported apart. No unlimited PHP work.
- Large posts: WordPress does not copy XenForo's large-post cutoff
  without its own approved, measured limit. Running out of bounded work
  space refuses the operation; content is never truncated.
- Every message, queue, retry, deadline, grant store, clean-up and
  health check is bounded in the release profile; a missing bound
  blocks it. Replay records live for their validity window; no
  unbounded nonce set.
- Review grant: at most 12 hours, 30 minutes idle. Progress-viewer
  token: five minutes. Download links and pairing approval are short
  and single-use.
- The memory limit is shared: sign-in, extra sites and separate installs
  bring no automatic extra allowance.

## Footprint budget

- No call to the service during page rendering; at most bounded
  background work is queued.
- Extra memory per request: 8 MiB for WordPress, 8 MB for XenForo,
  measured against the same site with the add-on off.
- Extra time per page: the owner's target is a few milliseconds. The exact
  statistic, ceiling, load profile, noise tolerance, extra queries and
  bytes per page are readiness blockers owned by the add-on footprint
  research record. Until that record supplies measured values, fixtures
  and approval, the affected add-on cannot be admitted for implementation
  or release; "a few milliseconds" is not an acceptance threshold.
- Cached public HTML carries only the counting script reference and
  non-sensitive page identity; the token comes from one uncached
  request.

## How it is tested

Tests are written first, with normal, adverse and missing-service
cases, and property tests that vary site IDs, roles, versions and fault
points. Hard-rule Rust code (validators, site binding, grant consumption,
journal matching) is proved with Kani within stated bounds; mutation,
fuzz and concurrency tests add to the proofs. Test sites run in
containers on Dell with made-up data through the heavy-job queue; memory,
load and soak tests run on the pinned test machine. Each case records its
inputs, expected result or refusal, permission, faults, tuple, bounds and
measured outcome.

### Installation and environment (Rust lifecycle tests)

- IP-01: each missing prerequisite gives the real missing or unknown
  fact and the next step; the add-on runs no shell; no false success.
- IP-02: swapping account, path, package, route or release in an
  approved plan fails before any effect; only registered resources made.
- IP-03: power, process or network loss at each checkpoint, then resume
  only the exact verified intent. For the 0030 pre-site checkpoint,
  try a foreign installation, stale register or partial migration
  receipt. Refuse site rows and connector work while only installation
  scope is present. No duplicate resources, site effects or lost receipts.
- IP-04: untrusted bootstrap, wrong root, forged signature, swapped
  digest or incomplete offline bundle stops before anything runs.
- IP-05: existing applications, services and site files are unchanged
  through install and its failures, apart from approved route changes.
- IP-06: unproved hosting, process budgets or local protection hold
  that profile. A vendor claim alone cannot qualify it.
- IP-07: reuse 0030 tests 2, 3 and 5. After passing preflight with
  a budget within the ceiling, separately hold on lost or insufficient
  memory evidence before PostgreSQL launch and before migration SQL.
  SUT-830 names the current check. Keep SUT-837 for the separate
  proved cap excess. With the same verified instance and launch
  receipt, restore current memory proof and continue the exact
  migration after the hold without starting another process.
  Reconcile a lost migration reply to the original receipt.
- IP-08: only an exact passing tuple, run through the whole lifecycle,
  becomes qualified in the register.

### Authority, isolation and content (Rust, SQL, Cedar and PHP tests)

- IP-09: two sites with equal item, user and job IDs each get only their
  own SQL rows, objects, vectors, caches, logs, backups and diagnostics,
  including pooled connections, stale scopes, errors and guessed paths.
- IP-10: shared weights leak no content, embedding, learned value or
  existence between sites; unsafe batches are refused.
- IP-11 (Cedar): reviewer or site admin asking for a global verb,
  another site, shell or an unregistered resource is refused and
  audited.
- IP-12: duplicated, replayed, expired, raced or rebound pairing gives
  at most one credential; native CSRF nonces alone cannot pass.
- IP-13: revoked or rotated keys and changed permission refuse old
  grants and queued work; prompts and reports grant nothing.
- IP-14: a shared PHP account that reads several sites' keys is refused
  as unqualified.
- IP-15: native sign-in, role loss, cross-site grant and CSRF: only
  current scoped authority reaches the right site's review. Join
  the 0028 IT-REVIEW-EFFECT open-view branch on both native profiles.
  Recheck the current review grant at server release. After the
  open view observes a denial through the existing refresh, an
  earlier released response or a restored browser view cannot
  show details or enable an action.
- IP-16: a paused site refuses writes after restarts and old approvals
  until a fresh authorised resume.
- IP-17: XenForo's oracle, approved-write, original-text and undo cases
  pass unchanged with WordPress switched on.
- IP-18: every WordPress state, type and permission change: only public,
  password-free content and authorised actions pass; unknown types and
  unqualified modes are blocked and labelled.
- IP-19: HTML, Unicode, delimiters, attributes, shortcodes, dynamic
  blocks and links: only eligible words are wrapped, protected bytes
  stay, and the real pinned editor opens, saves and reopens with no loss
  or invalid block (driven by a headless browser).
- IP-20: a human edit or transforming hook at each save point refuses
  stale approval and overwrites nothing; no conditional protocol means
  writes stay off.
- IP-21: repeated delivery and recursion apply a change at most once,
  keep hooks and revisions and audit correctly.

### Releases, updates and recovery (Rust tests)

- IP-22: stale or expired metadata, rollback, mix-and-match, swaps,
  wrong keys or a bad clock refuse the update; the service keeps running.
- IP-23: notice filters never install without the exact-plan action.
  For the channel and a downloaded bundle, try one valid plan as a site
  administrator, as a non-owner with the old app update permission and
  as the installation owner. Both non-owners get SUT-801 and start no
  update or write pause. Change one pinned site or
  payload identity after the owner's preview. SUT-803 requires another
  preview and approval. A matching approval enters the remaining
  update checks. Remote update requests carry no site, member, content
  or installation IDs. The app explains address visibility. IP-25 uses
  this exact owner check after a committed update step.
- IP-24: an incompatible site, too few resources or a failed backup
  check stops the update; the backup override is kept and audited.
  Use 0030 test 29 for the exact-volume workspace, later unsafe disk
  and unproved fallback cases. Include loss of space after a passing
  self-test and before the final switch. Separately substitute the
  approved backing root on the same volume with enough space. Test
  29 owns that root check and refusal. A backup override gives no
  disk exception. Only verified recovery proof supplies a rollback
  result.
- IP-25: kill or fail each update checkpoint under load: stale actors
  fenced, no lost or doubled work, only a compatible app returns.
  Use an exact owner-approved offline update with a verified package
  and one committed forward migration. Before activation withdraw
  only the installation owner's current update authority. Preserve
  package, backup, pairing and compatibility proof. Restart and replay
  the old approval. Keep the schema and exact migration receipt, show
  the 0030 owner-approval hold and keep writes paused. No new payload start,
  switch, fallback or repeated migration follows. Where the existing
  route offers current owner confirmation, bind the unchanged update
  intent and remaining work to that committed checkpoint. With all
  current checks passing, continue only those remaining effects.
  Use 0030 test 29 with IP-24 to separate an unsafe disk before switch
  from unsafe disk after a committed switch with a lost reply. The
  first holds before switch. In the second, verify and return only the
  original permitted effect. Recheck the disk, current lease and plan
  before any unperformed step. Reconciliation creates no second switch.
  In the compatible timed case, use spec 0030's release fixture clock
  across product process restarts. If an interruption breaks the
  qualified observer clock, verify the functional outcome and record
  the timing as unavailable.
- IP-26: hold current committed work constant. In the local effect
  journal fixture, commit one authorised native apply and its exact
  receipt under spec 0021. Then make a later human edit. Prepare two
  disposable fallback states for the same effect, one with a journal
  format the previous payload can read and one with a well-formed newer
  format it cannot read. Verify the format against the previous
  payload’s admitted reader proof. Keep the native receipt, post, schema,
  pairing and all other proof fixed. Fail the candidate self-test in
  both cases. The supported reader reconciles one terminal effect and
  retains the later edit. Replay adds no native write. The unsupported
  format holds before the previous app starts, keeps writes paused and
  preserves the journal, receipt and post. It cannot treat the effect
  as absent or retry it. Report the paused outcome without a verified
  rollback receipt, SUT-835 or timing pass. These fixture encodings add
  no product format or migration.
  On each admitted Linux and Windows IP-26 profile, keep the
  verified previous payload, committed forward migration, current
  owner plan, lease and matching version and checksum history.
  Pass the earlier compatibility preflight. At the previous app's
  database starter, use synthetic database metadata with RLS
  enabled but no longer forced on `_sqlx_migrations`. Keep the role
  and other policies eligible. The current 0108 guard reports
  SUT-971. Keep writes paused and show the failed database check.
  The previous app never becomes ready or takes the active pointer.
  Preserve committed history, current work and the original
  migration receipt. Ordinary app work never reads protected
  migration-history rows. Record no automatic restore, down
  migration, verified rollback, SUT-835 or timing success. The
  matching forced-RLS control can continue only under all existing
  current compatibility checks.
  Independently vary the fallback schema, persisted state format,
  journal, one registered site's
  compatibility and the build or store's encryption floor after cutover.
  A mismatch or missing proof keeps writes paused before the older app
  starts. Preserve later work and recovery evidence. No automatic
  backup restore, downgrade or verified rollback receipt. Include a
  passing exact compatible fallback as in spec 0030 test 9.
  In the interrupted fallback fixture, verify both synthetic paired
  sites and checkpoint before launching the previous app. Change the
  installed native add-on version for one site so it is incompatible.
  In a separate case keep that version fixed and revoke the native
  pairing used by its compatibility proof. In a third case keep that
  site's installed version, current keys and pairing generation.
  Present a restored copy of its receiver replay store without proof
  of current continuity under 0021 signing test 3. The old checkpoint
  does not qualify current pairing proof in that case. Keep the other
  site's proof, schema, state, journal and previous payload fixed.
  Resume with the earlier proof still in the checkpoint. Each case
  holds before the previous app starts. Preserve all later work and
  original receipts. Keep writes paused and emit the paused outcome,
  with no protected request, cursor advance, native effect, automatic
  restore, verified rollback receipt, SUT-835 or timing pass. In the
  continuity control, independently prove the current store for the
  same exact pairing. Fallback may continue only under all existing
  checks. The separate 0021 receiver test still refuses an accepted
  request repeated before nonce expiry. Use only synthetic proof on
  the admitted XenForo and WordPress fixtures, with no live site query.
  In a separate interrupted fallback, verify the retained previous
  payload under an admitted QR-202 profile. Keep its exact bytes,
  recorded manifest, earlier trust result, current plan and lease
  fixed. Keep schema, state, journals, pairing, erasure and the
  0118 key floor eligible. Before the previous app starts, use a
  current verified release policy that excludes its version from
  native use under the applicable 0005 acceptance floor. Resume
  the original update from its checkpoint. The earlier trust result
  remains evidence for its original context. Hold this fallback
  before launch, keep writes paused and show the release-trust check
  in Update paused. Preserve the current data and receipts. Record
  no resumed work, automatic restore, verified rollback, SUT-835
  or timing success. In the control the exact previous release
  remains eligible under the current verified trust profile.
  Join 0118 E2E-0118-MOVE for the separate key-floor restart case.
  Keep the previous package and encrypted store fixed. Require the
  current independent floor alongside the other fallback checks.
  Stale or unavailable proof shows Update paused with SUT-3700. An
  exact verified floor with no compatible protected key path shows
  SUT-3701. Resume and Rollback stay unavailable on each hold.
  Preserve the current data and work. Record no SUT-835, verified
  rollback or timing pass. No earlier archive supplies the floor.
  In a separate E2E-0118-MOVE branch, hold the minimum key epoch,
  encrypted store, schema, effect journal, pairing, owner-approved
  update and lease fixed. Use the current 0030/0032 history with one
  committed erasure. Supply authentic current history in a
  synthetic format outside the exact previous payload's admitted
  reader profile. In a second run give the same incompatibility
  together with an earlier readable checkpoint that predates the
  erasure. Neither proves the current fallback read route. Keep
  the previous payload stopped and show Update paused with the
  unsupported history check. Keep current work, erasure history
  and receipts. No protected data, prepared export, resumed writes,
  automatic restore or rollback success follows. The supported
  control uses the same current erasure, verifies one protected
  return and leaves that subject's erased derived data unavailable.
  In the post-start branch, let the exact previous payload start with
  current compatible proof and keep its verified startup checkpoint.
  Before allowing protected work or recording a verified return, present
  a current database schema revision outside that payload's admitted
  compatibility. Keep current data, committed migration and receipts.
  Use Update paused with the database compatibility reason. Assert no
  resumed writes, protected work, repeat launch or migration, automatic
  restore, rollback receipt, SUT-835 or timing pass. The control retains
  the compatible current revision and verifies one return under the
  original eligible plan and lease. Use synthetic revision evidence on
  admitted Linux and Windows profiles. These fixtures add no schema or
  stored format.
- IP-27: racing update, repair, restore and removal never share
  authority; a crashed holder needs reconciliation.
- IP-28: interrupting a lifecycle-part update at each hand-over, with
  engine and window down, never gives two holders and always leaves a
  route; an unknown journal never launches the old program.
  On each admitted Linux and Windows profile, commit the verified
  staged-file effect for the new lifecycle part and lose its reply
  before the holder hand-over can be proved. Keep both exact
  verified copies, the original plan and lease, the staged-file
  result and the earlier holder proof. Restart with the current
  holder proof missing. The earlier proof names the holder before
  the interrupted step only. Reconcile the staged-file result once
  under current read authority and preserve both copies and existing
  data. Use the existing verified recovery route. Grant no new
  authority, repeat no file effect, and make no fallback launch,
  write resume or successful rollback receipt. In the control,
  prove one exact current holder under the same lease. Only that
  holder may carry out remaining approved work after its other
  checks. Keep the engine and window down throughout this fixture.
- IP-29: transient failure recovery stops at its bounds, keeps work and
  shows repeated failure instead of looping.
- IP-30: corrupt backups, missing keys or missing storage never report
  success; an isolated restore checks data, roles, forced RLS and app
  state before a switch; later work is never erased.
  In a separate matching fixture, prove the previous app can restart
  on the current schema, state, journal and paired sites. Keep that
  proof and later work fixed. Offer in turn a correctly checksummed
  older backup with an archive format without a current admitted
  reader, and a supported backup with a recovery key below the
  signed minimum epoch. Each restore is held for its own reason.
  The permitted restart still succeeds. The restored generation
  stays inactive, no restore success or switch follows, and
  current records and later work stay intact.
  In the current history format branch, pass isolated restore with
  a supported archive, current key epoch and exact reader admitted
  under the current release policy. Before live publication, advance
  the authoritative history to a verified checkpoint whose format
  the selected reader cannot enforce. It contains a committed
  erasure fence with one outside target still waiting. Offer the
  earlier readable checkpoint. Hold the prepared stage and report
  the current history check before any new staged data lookup or
  live effect. Keep the live pointer, later work, backup and waiting
  receipt. Issue no switch, restore success or erasure completion.
  In the control, use a separate eligible plan with its exact reader
  for that current history. Apply the fence in isolation and check
  current owner approval and lease before any live switch. A changed
  scope still needs a new preview and confirmation. The waiting target
  needs its own result. This fixture adds no format, minimum release
  version or recovery route.
  In the inverse case keep the exact verified previous payload and an
  eligible encrypted backup. Make only the current schema incompatible
  with the previous payload. It never starts. Restart with the same
  paused update record and refuse restore while its lease is
  unreconciled. After exact reconciliation, confirm a separate
  isolated restore whose reader, key epoch, history and row-level
  security pass. Require its own test result. The live pointer,
  current and later work, and paused compatibility outcome remain.
  No verified rollback receipt or automatic restore switch follows.
  In a separate fixture for an admitted portable restore, keep the
  exact selected portable plan and lease, package, archive, isolated
  result, volume and protection proofs fixed. Withhold the current
  portable activation evidence. Offer an authentic normal-install
  restore and launch result under its own profile for matching
  package and archive bytes. Hold before portable activation or
  resuming writes. Preserve the isolated evidence, current pointer
  and later work. Do not switch to a normal install or register a
  start-up entry. Record no live restore or rollback receipt. In
  the exact portable control, verify its own current at-use
  profile and all existing checks. Only that route can switch.
  This fixture admits no new restore or mixed-mode archive route.
  For an already admitted synthetic switch, pass the isolated restore
  and retain its verified test result. Give the switch its own exact
  approved plan and current lease. Verify [0030's protected restore
  history][restore-history] and
  [0118's minimum key epoch][restore-key-floor], and retain current
  reader, manifest, authority and applicable storage proof. Fail
  only the required audit write under
  [Diagnostics and audit](#diagnostics-and-audit) item 2 before
  live activation. Refuse the switch with SUT-848. Preserve the live
  pointer, current records and later work. The isolated target stays
  inactive, with its test result limited to that target. Keep existing
  write and access holds. Emit no live restore success, switch or
  rollback receipt. After interruption reconcile the original plan
  and recorded lease. Any unperformed retry requires current exact
  authority and a current lease. This fixture admits no switch route.
  In a separate synthetic Windows 11 restore variant, verify the
  isolated target on native volume A. Keep the exact approved switch
  plan and lease, verified backup, and 0030 history and audit proof.
  At the final publication check substitute a different writable
  volume B with the same drive letter and label. Apply the current
  volume and ledger identity check in 0038. Show that the selected
  restore destination could not be verified. The isolated result
  still describes A only. Keep the live pointer and later work, give
  no live success or rollback receipt, and do no write, activation,
  relink or cleanup on B or any substitute. The existing pending
  audit stays distinct from an effect receipt. In the control keep
  the exact eligible volume A, ledger, isolated result, current
  approval and lease. The fixture records one switch to A only
  after all current publication checks pass.
  In another run pass the final check against backing root A. Then
  at effect hand-off supply backing root B on the same volume and
  path, with matching visible ledger markers and restored bytes.
  The isolated result and pending audit remain evidence for A.
  Hold on changed or unproved at-use identity and show the existing
  destination verification reason. Keep the live pointer and later
  work. Refuse any write, relink, cleanup or activation on B and
  any live success or rollback receipt. The control keeps the exact
  checked object through publication and verifies only its own
  committed switch. No live target is used.
  In a separate admitted synthetic portable restore, commit the exact
  switch to volume A with the matching backing root and verified
  effect receipt, then lose the reply. On restart present volume B
  at the same path with the same label and restored byte hashes. Keep
  the original receipt and the pointer to A. Under current read
  authority recover only the historical switch outcome. Hold current
  access with the destination verification reason. Do no read of
  restored data, write, cleanup, relink or launch on B. There is no
  new switch or rollback receipt. When exact A and its backing root
  return in the control, recheck current access under the existing
  profile. Any permitted work uses A and the switch count stays one.
- IP-31: lost signing or pairing keys, stale trust or a hardware key on
  another machine follow only the defined recovery path.
  Use synthetic receipts from the existing release-root and site-pairing
  recovery routes. After an eligible isolated restore on another machine,
  the saved site's fingerprint is present and the old hardware-bound
  private key is unavailable. Offer a genuine current release-root
  recovery receipt as pairing proof. The restored site stays disconnected.
  It sends no protected request, reactivates no key or grant, and resumes
  no pending site effect. Current release evidence is still eligible
  for its own package checks only. In the fresh pairing control, obtain
  the existing local owner and native administrator approvals and prove
  both peers' signing and encryption possession for the exact site,
  installation and current pairing generation under specs 0021 and
  0118. Only that link becomes eligible. Independently withhold current
  release-root proof and offer this genuine site pairing receipt to
  the release check. Refuse package use with no launch or install effect.
  Neither refusal changes the original proofs or restores old authority.
  In the same-installation restore branch, reuse 0021 signing test
  10 for an exact recorded native effect after link key rotation.
  Verify current pairing and replay-store continuity first. A
  retained local journal or old reply supplies no current proof.
  A fresh exact read reconciles the original receipt only. An
  unavailable native record stays unknown without another write.

[restore-history]: 0030-install-upgrade-removal.md#the-site-register
[restore-key-floor]:
  0118-end-to-end-encryption-gate.md#rotation-legacy-upgrade-and-deletion

### Removal, reporting and resources

- IP-32: one site disconnecting or removing its integration leaves the
  other's data, keys, models and queues intact. In a synthetic WordPress
  case complete the scoped disconnect with the plugin still installed.
  Old pairing keys can no longer act. Give no removal or erasure proof
  for the plugin or its records. In a separate integration removal
  case select the registered native cleanup and lose its result after
  local access is revoked. Keep that cleanup unresolved and preserve
  the verified local result until IP-34 reconciles the original effect.
  Restore a site fixture from before removal with the old pairing key.
  It stays revoked under the current 0021 and 0118 trust records.
  No native effect or renewed access follows. The backup supplies
  no cleanup proof.
  Approved links in the site’s posts and the other site stay intact.
- IP-33: only selected, authorised resources are deleted; failed
  export, protected storage, unknown ownership or another site's use
  prevents deletion; defaults keep data.
- IP-34: interrupting each removal step: revoked keys cannot write,
  results are honest, the lifecycle part stays until last, approved
  links stay.
- IP-35: planted secret values never appear in diagnostics, metrics,
  logs, previews or packs; sharing needs a preview.
- IP-36: GitHub, database and notice outages and duplicate CI events:
  cursors skip nothing, issues are de-duplicated, failures stay visible,
  and Dell has no incoming route.
- IP-37 (release tier): both platforms at load, with migration, backup,
  restore and overlapping versions, fit the memory limits with headroom
  and no swap, on both sizes of PC.
- IP-38: paired add-on-off and add-on-on request runs hold the XenForo
  and WordPress memory limits, with concurrency and persistent overhead
  reported; missing measurements block.
- IP-39: every app and site page in loading, empty, failed, retry,
  offline and unsaved states passes keyboard, translation, visual,
  responsive and WCAG 2.2 AA checks.
- IP-40: manifest, register, manuals, recovery guide and the six packs
  agree with the exact release; nothing unbuilt is labelled shipped.

### Interfaces, WordPress and sign-in (WX cases)

- WX-01: each task has one editor; both sides call the same Rust
  authority; no ranking in PHP. With engine or window stopped, site
  pause and scoped recovery still report useful state.
- WX-02: root and subfolder sites, clashing routes, proxy prefixes and
  cookie clashes: only a qualified existing-HTTPS mapping is offered and
  originals survive failure.
- WX-03: untrusted sites on one hostname cannot qualify by path; with
  distinct hostnames, origin, parent-domain cookies, redirects, host
  spoofing, PHP accounts and server data are each tested.
- WX-04: identical numeric IDs on both platforms reach only the right
  facts, validator, permissions and save protocol.
- WX-05: every publication, password, type and restriction state; posts,
  pages, custom types and multisite apart; a mid-job visibility change
  invalidates index, cache and approvals.
- WX-06: every protected region, malformed nesting, delimiters and JSON
  attributes: only visible words wrapped, bytes and order kept; fail on
  ambiguity.
- WX-07: a qualified paragraph beside an unknown block refuses the whole
  item; every admitted editor tuple passes open, save and reopen.
- WX-08: racing, crashing and retrying hooks: receipts de-duplicate,
  tombstones stop resurrection, reconciliation closes gaps, nothing
  unsaved is acknowledged.
- WX-09: restarts, late approvals, undo retries and unreadable pause
  all refuse writes; resume touches only that site.
- WX-10: edits, permission loss and target privacy at every save point:
  stale writes never commit; `wp_update_post` with a local lock fails.
- WX-11: undo with later edits, duplicate anchors, overlaps, a corrupt
  or lost journal and an interrupted save stops without text loss; a
  whole revision restore fails the case.
- WX-12: the grant binds every field; duplicate use, lost replies,
  replay, expiry, swaps, open redirects, stale epochs, unsigned data and
  CSRF fail; viewer and pairing tokens and nonces cannot pass.
- WX-13: logout, role loss, revocation or a restored old epoch refuses
  authority at request and save; emergency sign-in never skips object
  permission; site admins never act on shared resources.
- WX-14: all screen states pass keyboard, translation, visual and
  accessibility checks; PHP, parsing, queue and two-platform peaks are
  measured; missing bounds block.
- WX-15: feature and capability IDs keep their meaning, manual pages
  match the actions, links resolve and nothing unbuilt is called shipped.

### Coexistence (CO cases, Docker matrix on Dell)

The matrix runs a baseline without the plugins, each required row on
its own, then qualified combinations. Each row has a positive control:
refusing everything never passes. Other protections stay on. Rows:
Wordfence, Sucuri, Solid Security, All-In-One WP Security, WP Rocket,
LiteSpeed Cache (with its real cache runtime), W3 Total Cache, Yoast,
Rank Math, Elementor, WPML and Polylang; Autoptimize, Link Whisper, Divi
and WPBakery get their own rows before support is claimed. Outside
firewalls need classifier and denial tests. The release tier runs
install, pair, suggest, approve, update and uninstall on each row from
scratch; the nightly run does smoke coverage of all rows. Licensed
copies come only from official sources.

The release and nightly campaigns also cover the supported XenForo and
WordPress versions and their latest beta candidates, Xon's Redis cache,
XenForo Enhanced Search, the major theme vendors and Redis Object
Cache. Each vendor group needs named editions and versions before it is
admitted. Each campaign records the exact beta it ran; a beta that
passes is never turned on for real sites without its own
qualification. A missing licensed copy or test setting blocks its
campaign (archive `installation-platforms/non-invasive.md`).

- CO-01: engine and health traffic stay within rates, batches, CPU,
  memory, timeouts and concurrency on a busy site.
- CO-02: 403, 429, Retry-After, clock changes and restarts: back-off
  persists, no storm, expired or revoked work stays refused.
- CO-03: each firewall's block page, even with 200, reads as blocked
  with guidance; unknown pages and real faults stay distinct; no leak.
- CO-04: multibyte ranges in both formats match Rust's hashes; patches,
  code, wrong versions, stale source, bad ranges and drift refuse.
- CO-05: racing saves, retries and undo lose no human edit or link;
  without conditional-save evidence writes stay off.
- CO-06: with passwords and users routes off, signed namespace requests
  work; wrong site, key, purpose, nonce, epoch or signature refuses.
- CO-07: behind page caches and stacked optimisers each token is
  fetched uncached; the asset survives exclusions; no double count.
- CO-08: every detected identity and an unknown revision show the right
  guide without changing other plugins; updates void old evidence.
- CO-09: builder items, dynamic links and SEO output are skipped or
  left untouched.
- CO-10: a language change before apply lets only eligible English targets
  pass.
- CO-11: with zero visits the runner progresses within bounds; killed
  runners, duplicate callbacks, pause and revoke cause no doubled write.
- CO-12: the six lifecycle steps on each row keep permissions, content
  and plugin settings; uninstall removes only Sutradhar's parts.
- CO-13: a missing licence, tool or row, or a failed run, blocks its
  stage and raises one de-duplicated issue.
- CO-14: manual steps work by keyboard, touch and assistive technology,
  match the version, expose no secrets and never say to turn off
  protection.

### Non-invasive cases (NI)

- NI-01: install, update and remove through supported extension points;
  core files, schema and original callbacks are unchanged.
- NI-02: a missing or slow service during visitor load causes no
  service traffic, visitor error or budget overrun.
- NI-03: editing one item under each cache stack refreshes affected
  pages only, with no global flush or private state in cached HTML,
  across several web servers.
- NI-04: varied load order, theme, CSP and optimisation cause no CSS or
  global clashes, lost behaviour or permission leak.
- NI-05: planted callback and logging faults and untested versions:
  display falls back, writes refuse, pause survives, admin recovery and
  redacted alerts work.
- NI-06: every pinned supported, beta and plugin row runs install, use,
  update and removal; deprecations block (PHPStan deprecation rules on
  every push); ordinary host behaviour is identical while paused and
  after removal unless the owner approves an exception.

### Analyser and planted faults

The project's Rust analyser extracts facts for site and platform
ownership, validator, capability, current-version check, pause, journal
and save receipt, grant purpose, recipient, expiry, epoch, logout and
revocation paths and action IDs, each with provenance. An unresolved call
or hook widens the checks and blocks the claim. Planted faults in every
relevant language must fail: forbidden writes, wrong validators,
cross-site grants, altered protected bytes and fake journal matches.
Speed is checked by the 5% median rule on Dell baselines.

## Changes from the source

- The owner app starts and controls its engine on the supported
  Linux and Windows profiles. Spec 0030 owns package trust.
- The owner's Tauri window uses Angular and SDS. Shared-host add-ons
  deliver native moderator review and status under their own grants.
- The one-use handoff into a live screen is kept as a one-use review
  grant issued after native sign-in; live-socket rules are dropped.
- Install, update, backup, repair and removal move from site admin
  pages to the app (AGENTS.md 5.7); site pages keep pairing, pause,
  review and status.
- The independent recovery controller becomes a separate Rust lifecycle
  program on the PC.
- The `sutra` host command is rewritten as Rust.
- Prahari is rewritten as Rust checks proved with Kani, Verus or Creusot.
- Numeric work is in ganit, the Rust numerical engine.
- The analyser is rewritten as Rust.
- A cloud home and remote windows are dropped: the home is the PC and
  the only window is the app on it; helpers may be cloud helpers.
- Message signing uses Ed25519 per AGENTS.md 5.4, with keys split by
  purpose and direction, replacing HMAC-SHA256 derivation.
- XenForo facts come through the add-on's read-only reads over the
  signed HTTPS contract, not a direct database link from the service.
- The editor round trip runs in a headless browser.
- Speed uses the 5% median rule in place of ratchets; the 20x goal is kept.
- Heavy Dell runs use the heavy-job queue (AGENTS.md 11), replacing the
  old 3.5 GB governor.
- Old milestone, requirement and capability mapping tables are dropped;
  tests map to the gate catalogue.

Source: `docs/specifications/installation-platforms/` in the archive
`archive-2026-11-08-old-repo`: `README.md`, `architecture.md`,
`lifecycle.md`, `security.md`, `updates.md`, `amendments.md`,
`acceptance.md`, `delivery.md`, `references.md`, `interfaces.md`,
`wordpress.md`, `authentication.md`, `acceptance-wxf.md`, `desktop.md`,
`coexistence.md`, `coexistence-acceptance.md`,
`coexistence-references.md` and `non-invasive.md`.
