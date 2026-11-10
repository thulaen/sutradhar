# 0031: Shared-platform installation

This is how one Sutradhar installation serves XenForo forums and
WordPress sites side by side: installing, pairing, running, updating,
repairing, backing up and removing it, and the safety contract at each
site's edge. One shared service keeps every site's data, permissions
and work apart. Removing one site's connection leaves the others
running, with the shared models they still need. It is roadmap Phase 5
item 1, with the site contracts of Phase 4 items 3 and 4.

## What it does

### Shape of an installation

1. The owner installs one signed `.deb` on the Ubuntu 26.04 PC, which
   is the home: the one place that holds authoritative state. The
   Dioxus desktop app (the window) is the only owner screen. It drives
   the service over the owner-only local socket.
2. Each site gets a thin signed add-on (XenForo) or plugin (WordPress).
   The app hands the owner the signed zips to install through the
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

Events and pause:

1. Save, update, delete and status or visibility events carry the site,
   a stable event ID, object and version, cause, and a deletion
   tombstone where needed. Event data is only a hint to reread facts.
2. Delivery is at least once, with durable acknowledgement,
   de-duplication, bounded retry and a dead-letter list. Out-of-order
   events never bring back deleted or newly private content. Lost
   events are found by checkpointed reconciliation with native state.
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
   never a blind retry.

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
   old whole revision is never restored over later edits. A lost
   journal or expired evidence blocks undo. Retention follows the
   privacy policy, not a copy of XenForo's receipt expiry.

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
   XenForo forum and gives no right to update the installation.
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
7. Links join only source and target in the same content language,
   from WPML or Polylang facts checked again at apply. Browser locale
   or URL shape is not enough. Unknown or changed language holds the
   operation. A monolingual profile may declare its language. No
   translation edits and no cross-site fallback.
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
   2. Check OS and architecture, systemd, PostgreSQL, disk, memory
      actually free, outgoing connectivity and enforceable slice limits.
   3. Confirm the plan still matches the PC and the approved authority.
   4. Create only the approved service account, storage and slice.
      Leave every other service and application alone.
   5. Fetch exact models and data, check their rights and verify each.
   6. Start only registered Sutradhar parts in the budget. Create
      PostgreSQL roles and forced RLS through forward-only migrations,
      with separate schema-owner and app roles.
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
   still sees the PC's public address. An outage or expired metadata
   blocks new installs and updates, never the running service.
7. Notices start on; automatic install starts off. A setting filters
   notices to security releases or all releases. Installing needs the
   owner's approval of the exact plan; unattended install would need a
   new approval.
8. The notice shows what changed, each site's compatibility, the
   expected interruption, resource and disk needs and the recovery
   plan. The plan pins sites, releases, schema and state versions,
   artefacts, backup evidence and permissions; any change voids it.
9. Applying: take the durable operation lease, fence stale actors,
   verify the whole release, check sites, resources and backups, pause
   affected writes and checkpoint. A failed check keeps the old state
   or a clearly reported paused state, never partial success.
10. Old and new run together only if their measured combined peak fits;
    otherwise the owner is offered a maintenance window. Limits are
    never lifted for an update.
11. Schema changes are forward-only expansion, then self-tests of
    content, permission, queue, observability and recovery. Only a full
    pass resumes writes; approvals and changed content are rechecked.
    Contraction needs its own compatibility window and evidence.
12. An older app restarts automatically only when current schema,
    state, journal and every site are proven compatible; the
    three-minute return target applies only then. Otherwise the service
    pauses and asks. Migrations never run backwards, and an old backup
    is never restored automatically. The owner-only backup-check
    override stays, with reason and audit, but never overrides release
    trust, required checks, limits, data safety or compatibility.
13. The lifecycle part updates as its own verified artefact: staged,
    checked, with the previous copy kept and the journal preserved.
    Only one copy holds authority. Every interruption leaves either a
    working lifecycle part or a verified recovery route. Metadata
    versions are recorded to refuse rollback, freeze, mix-and-match and
    substitution. A bad clock blocks updates; the installer never
    changes system time.
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
   confirmation; the lifecycle part goes last and keeps its journal
   until the final receipt is durable. Interruptions stay resumable.
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

1. Each register row pins OS and architecture, kernel and cgroup mode,
   systemd, PostgreSQL, hosting and PHP, the add-on, platform, site
   database, service release and digests, models and topology. Evidence
   names the exact tuple, data, limits, tool and rule versions and
   results. No vendor name or nearby version stands in; a missing field
   blocks rather than meaning "all versions".
2. Rows: the Ubuntu 26.04 x86-64 PC; XenForo and WordPress together
   (memory and tenancy to qualify); separate installations (extra cost
   to measure and approve); WordPress multisite, custom types and
   plugin modes (each qualified on its own); untrusted sites on one
   hostname (cannot qualify); separate hostnames with isolated accounts
   (needed). ARM needs its own approval.
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

- Memory: Sutradhar stays within the memory limits in AGENTS.md section
  5.6: the service slice (service, engine, lifecycle part, models,
  ganit, MCP server, PostgreSQL, workers, backups and overlapping
  updates) and the desktop window. No swap. The 15% headroom applies
  to the service slice: the soft line is 85% of it, 3,825,000,000 bytes
  of 4.5 GB on a PC under 16 GB and 5,950,000,000 bytes of 7 GB on a
  larger one (spec 0024), using decimal bytes. The desktop window keeps
  its own cap of 0.5 GB or 1 GB. Native
  allocation peaks and restore and rebuild work are measured.
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
- Extra time per page: the owner's target is a few milliseconds. The
  exact statistic, ceiling, load profile and noise tolerance, and the
  extra queries and bytes per page, are set when the feature is built.
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
- IP-03: power, process or network loss at each checkpoint, then
  resume: no duplicate resources, no lost receipts.
- IP-04: untrusted bootstrap, wrong root, forged signature, swapped
  digest or incomplete offline bundle stops before anything runs.
- IP-05: existing applications, services and site files are unchanged
  through install and its failures, apart from approved route changes.
- IP-06: hosting that cannot run the add-on, or slice limits that
  cannot be enforced, stay unsupported; vendor claims never pass.
- IP-07: a busy PC with too little free memory waits or refuses safely,
  even when Sutradhar's own caps fit.
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
  current scoped authority reaches the right site's review.
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
- IP-23: notice filters never install without the exact-plan action;
  update requests carry no site, member, content or installation IDs;
  the app explains address visibility.
- IP-24: an incompatible site, too few resources or a failed backup
  check stops the update; the backup override is kept and audited.
- IP-25: kill or fail each update checkpoint under load: stale actors
  fenced, no lost or doubled work, only a compatible app returns.
- IP-26: incompatible schema with later work refuses automatic
  fallback and old-backup restore, keeps evidence and pauses.
- IP-27: racing update, repair, restore and removal never share
  authority; a crashed holder needs reconciliation.
- IP-28: interrupting a lifecycle-part update at each hand-over, with
  engine and window down, never gives two holders and always leaves a
  route; an unknown journal never launches the old program.
- IP-29: transient failure recovery stops at its bounds, keeps work and
  shows repeated failure instead of looping.
- IP-30: corrupt backups, missing keys or missing storage never report
  success; an isolated restore checks data, roles, forced RLS and app
  state before a switch; later work is never erased.
- IP-31: lost signing or pairing keys, stale trust or a hardware key on
  another machine follow only the defined recovery path.

### Removal, reporting and resources

- IP-32: one site disconnecting or removing its integration leaves the
  other's data, keys, models and queues intact.
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
- CO-10: a language change before apply lets only same-language targets
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

- The service runs on the owner's Ubuntu 26.04 PC from a signed `.deb`
  under systemd, not in containers beside the website on a hosting
  server.
- Host preparation by an agent with server access, Docker, rootless
  limits and hosting-panel recipes are replaced by the `.deb` installer
  and the systemd slice.
- Shared hosting is supported for the add-ons, as AGENTS.md 5.4 requires.
- The first test target is Ubuntu 26.04, replacing the earlier 24.04.
- The live review and settings screens become the Dioxus app built from
  SDS; moderators review in the site's admin pages.
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
