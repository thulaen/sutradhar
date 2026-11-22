# 0101: Site accounts, writers, governance and compliance helpers

This specification owns how people other than the owner take part in Sutradhar's
work, and the plain governance and compliance tools around it. It covers: the
permission templates, expiring and agency access, trusted member writers and
offboarding; each writer's personal space (assistant profile, style memory,
dashboard, private drafts and source packs); sponsor content intake; the team
activity feed, decisions knowledge base, training sandbox, access and retention
policies, local feature-use statistics, the quarterly planning board and
stakeholder digests; and six compliance helpers (affiliate disclosure, sponsored
labels, accessibility statement, cookie policy, data request log and advertising
standards).

How people reach Sutradhar is settled by earlier specs and is not changed
here. The owner uses the desktop app on the owner's own PC (spec 0020).
Everyone else works in the sites' own pages with the sites' own sign-in, and
what they do arrives through the site link as signed messages (spec 0020,
"Changes from the source"). Sutradhar has no web sign-in, no credentials of
its own for other people and no network port; it only keeps records keyed by a
site account id. This spec therefore adds no login. It adds how Sutradhar
decides what a site account may do, and what Sutradhar remembers per account.

Spec 0020 owns tokens, actions and the confirm step; spec 0028 the staff
review of suggestions; spec 0048 the Fix list with its comments, impact
ranking and effort estimates (features 541, 549 and 550 stay there); spec 0032
audit and erasure; spec 0014 settings and history; spec 0034 visit counting;
spec 0039 the test room; spec 0094 reports; spec 0100 outside services; spec
0041 the Editorial Coach.

Example: Priya is a freelance writer for a forum's blog. The editor gives her
the "Agency writer" template for 60 days. She opens the Coach in the blog
editor on the site, signs in with her site account, and gets coaching on her
draft. The draft stays in her private space; other site accounts and the
editor cannot read it unless Priya shares it. (The owner, who runs Sutradhar
on their own PC, can technically read the database, and Priya's notice says
so.) On day 61 the grant has expired, and the next request is refused with a
plain message while the editor's settings stay as they were. When the
engagement ends, the editor presses "Offboard writer": Priya's grants and
tokens stop at once, her drafts are exported for her, her style memory and
source packs are kept 90 days and then deleted, and the audit log records
every step. Separately, the site sponsors post a brief through the sponsor
form; it waits in a review queue with a "Sponsored" label and a disclosure
check, and nothing is published until a human accepts it.

Rules of the sites and of the law named here (disclosure rules, accessibility
duties, privacy deadlines, advertising codes) are drawn from the owner's
feature list and are "unverified" until the dossier confirms each with an
official source (QR-054). Everything the compliance helpers produce is a draft
or a check for a human; it is not legal advice and needs the legal review of
QR-010.

Values marked "proposed" are design choices to be checked by measurement.

## What it does

### Words used here

- **Site account**: a person's account on a site (a forum or WordPress user),
  known to Sutradhar by its site account id.
- **Note**: a message the site's add-on stores in its outbox for Sutradhar to
  collect (spec 0021).
- **Rights claim**: what the add-on says a site account may do, carried in a
  note.
- **Template**: a named set of permissions for a site account.
- **Grant**: a template given to one site account, with a start, an optional end
  and who gave it.
- **Writer space**: what Sutradhar keeps for one site account: profile, style
  memory, drafts and source packs.
- **Owner**: the person who runs Sutradhar on their own PC.
- **Feed**: a list of recent events people may see.
- **Sandbox**: a practice copy with made-up data.

### What this spec designs

| Feature | What people see |
|---|---|
| 556 Permission templates | Ready-made sets of permissions to give to people |
| 552 Team access and retention policies | Who may see what, and how long things are kept |
| 1309 Expiring guest access | Access that ends on a date |
| 1311 Agency writer access | A limited template for outside writers |
| 1308 Trusted member writers | Members allowed lighter checks on their own posts |
| 1312 Writer offboarding | One step that ends a person's access and handles their data |
| 1303 Assistant profile per writer | The coach's settings for one writer |
| 1304 Personal style memory | What the coach has learned from a writer's own accepted edits |
| 1305 Writer dashboards | A writer's own numbers and progress |
| 1306 Personal drafts space | Private drafts |
| 1307 Writer source packs | Saved sets of sources for a piece |
| 1310 Sponsor content intake | A form for sponsors, a queue and a label |
| 543 Team activity feed | A feed of what happened |
| 547 Decisions knowledge base | A record of decisions and their reasons |
| 551 Training sandbox | A practice site with made-up data |
| 553 Feature usage analytics | Which features are used, kept on this PC |
| 554 Quarterly planning board | Goals and the work linked to them |
| 555 Stakeholder digests | A short summary for people who do not use Sutradhar |
| 629 Affiliate disclosure checker | A check that affiliate links are disclosed |
| 630 Sponsored content labels | Clear labels and link marks on sponsored content |
| 631 Accessibility statement draft | A draft statement from what Sutradhar finds |
| 632 Cookie policy draft | A draft from what the site is found to set |
| 633 Data request log | A log of privacy requests and their deadlines |
| 634 Advertising-standards check | A check of claims in paid content |

### Who can do what

1. The owner has every permission, through the desktop app on the owner's own
   account.
2. A site account acts through the site's pages. The site never calls the
   owner's PC (spec 0021). The add-on stores what the person did, such as
   "this writer asked for coaching on this draft", as a note in its own
   outbox. Sutradhar collects notes with its signed poll; the whole reply is
   signed by the add-on's Ed25519 key and checked by the signed-message guard
   of spec 0021 (contract, nonce, timestamp, body hash). Each note carries the
   site id, the site account id, a note id, the time it was made and a rights
   claim. The site derives the actor from its authenticated action with
   its native protection against forged browser requests. The claim
   binds the native decision and check time to the site, actor, action,
   typed target and contract version. Refuse a note with invalid or
   missing bindings. Sutradhar answers through the signed routes of
   spec 0021. The writer sees the answer the next time the page asks
   after Sutradhar's next poll.
3. Replay and order: notes have increasing ids; Sutradhar records the highest id
   handled per site and ignores an id it has already handled. A note older than
   `team.note_max_age_s` is refused (SUT-1802).
4. An agent acts through a token the owner made (spec 0020). The token
   binds its site and acting account at issue. Take that identity from
   the validated token. A request cannot select its own actor.
5. Permissions are Cedar rules (AGENTS.md). A request is allowed only if the
   rights claim says the account has the right on the site, a grant gives it
   in Sutradhar, and the rules allow it; all three must agree. A grant can
   never give more than the site account holds.
6. Freshness. A rights claim is accepted for `team.claim_max_age_s` (default
   300). When a claim is older, or an agent token acts for the account,
   Sutradhar first asks the site for the account's current rights through a
   signed `ReadAccountRight` route in spec 0021, for the exact action and
   target. If the site cannot be reached or the signed answer cannot
   be verified, refuse the request (SUT-1811). A current native deny
   takes effect even if a prior claim allowed the action (SUT-1801).
7. The add-on determines the site right using its native account and
   target checks. A fresh allow or valid note claim is only the site
   input to the three-way decision. Check the current local grant and
   Cedar policy before applying that decision. A site claim or reply
   never grants a setting change or an owner-only action.
8. Grant end dates are enforced from Sutradhar's own clock, never from a
   claim, so an expired grant is refused even when the site link is down.

### Account Right Actions

The signed contract covers viewing site-scoped suggestions or reports,
using the Coach on an owned or shared draft, reviewing a suggestion,
viewing moderation hints and submitting through the sponsor form.
Each action names its typed resource and, where used, its section.
Admission under QR-089 requires one native predicate and a conformance
fixture per action on each platform that supports it. Unknown or
unmapped actions are denied. Templates, owner-only controls and local
grant administration use their own local rules.

### Permission templates

1. Templates are rows in Sutradhar's own `templates` table, not settings of
   the registry. Each template compiles to Cedar permit rules, which are
   regenerated whenever the template changes. The built-in ones are: -
   **Viewer**: sees suggestions and reports the site account may already see.
   - **Writer**: uses the Coach on their own drafts and sees their own space.
   - **Editor**: a Writer plus seeing coach results of drafts shared with
   them. - **Reviewer**: accepts or refuses suggestions in the site's pages
   (spec 0028). - **Moderator**: sees moderation hints (spec 0051); never
   bans. - **Agency writer**: a Writer limited to named sections and to an end
   date. - **Sponsor contributor**: may submit through the sponsor form only.
2. The owner may make new templates from the list of permissions. A template
   has a name, the permissions and an optional default end.
3. A template cannot hold a permission in the never-changeable class, an
   owner-only setting, or any permission to change a site directly. People
   propose; a human accepts (AGENTS.md).
4. Two permissions are special. `grants.manage` (give or end grants) and
   `trusted.manage` (mark or unmark trusted writers) belong to the owner only
   and are in no built-in template; the owner may add one to a template on
   purpose.
5. Changing a template changes all its grants at once, with a preview of who is
   affected, and is audited.

### Grants, guests and agencies

1. A grant gives a template to one site account for one site, with an optional
   end date (`team.max_grant_days` is the longest allowed). The grant, who
   gave it and why are recorded. A site account that the site marks as under
   its minimum age cannot get a grant.
2. Expiring guest access is a grant with an end date. After the end the
   account is refused at its next note or call and the screen says why; the
   grant is kept in history. A reminder goes to the giver
   `team.expiry_notice_days` before the end as an in-app alert and, if the
   owner has enabled it, through the alert centre's routes of spec 0099.
3. Agency writer access is the Agency writer template. It limits the account to
   named sections, hides other people's drafts and suggestions, and shows no
   figures about the site beyond the writer's own pieces.
4. Trusted member writers. Staff with `trusted.manage` may mark a member as
   "trusted writer" when the member meets the criteria
   (`team.trusted_min_age_days`, default 90, and `team.trusted_min_accepted`,
   default 10 accepted posts). For a trusted writer: the hard checks (safety,
   privacy, disclosure and policy) always run; the soft quality checks still
   run but never block; and their submissions go to the front of the review
   queue ("fast lane"). Their posts are never published by Sutradhar, and a
   human still accepts anything beyond what the site already lets them post.
   Staff may revoke at any time, a mark or revoke by one account on itself is
   refused, and every mark and revoke is audited and listed to the owner
   weekly. A trusted mark is not a permission to change a site.
5. Only an account with `grants.manage` may give or take away a grant, and no
   one may give a grant to themselves.

### Offboarding

1. "Offboard writer" is one action made of steps that are recorded one by one,
   so an interrupted run resumes at the next step and shows "incomplete" until
   the end (SUT-1808): (a) mark the account "offboarding" so new notes are
   refused; (b) revoke the account's agent tokens (spec 0020: at once); (c)
   end its grants; (d) export the writer's private drafts and source packs for
   them, in plain text or Markdown with a JSON index; (e) move shared work to
   an editor-chosen account; (f) schedule the deletion below. Notes already
   received but not yet handled from the account are dropped and listed.
2. The writer's style memory, profile, dashboard data and source packs are
   kept `team.offboard_keep_days` (default 90) in case of return, then
   deleted. Drafts follow the draft rule of spec 0042
   (`coach.draft.keep_days`) and the writer's own wish to delete them. An
   erasure request (spec 0032) deletes everything at once.
3. Every step is audited. Audit entries keep the opaque account id, not the
   person's private content, as spec 0032 requires.

### Writer space

1. Roles and notice. The owner is the person responsible for the data. Before
   a writer uses the Coach for the first time the add-on shows a notice, in
   plain English, that says what Sutradhar keeps, for how long, that drafts
   stay on the owner's PC and never go to a route that leaves it (spec 0041:
   other people's drafts never leave the PC), who can see what, and that the
   owner can read the database. A writer space is private to the account from
   other site accounts and staff. Staff with `writer.read_all` see that drafts
   exist, with titles and dates, and can read a draft only if the writer
   shares it.
2. Drafts use the draft store of spec 0042 (the pasted or uploaded draft kept
   "for the editor alone"); this spec adds account-keyed access, sharing,
   versions and offboarding to it and creates no second store. Versions are
   kept `team.draft_versions` with an autosave every `team.autosave_s`
   seconds. Drafts are stored with row-level security (spec 0108) and
   encrypted at rest with the install's key (spec 0065). The writer keeps the
   rights to their text.
3. **Assistant profile.** A writer's settings for the Coach: tone, topics, which
   checks to show first, reading-level target. It changes how the Coach talks to
   this writer, never what is checked.
4. **Personal style memory.** Derived features only, never raw text: counts
   and ratios (sentence and paragraph length, heading use, readability band),
   up to 200 preferred terms taken from the writer's own accepted edits, and
   tone labels. It is capped at 64 KiB per writer, versioned, encrypted,
   included in backups and registered as an erasure target (spec 0032). It
   uses only that writer's own accepted text, is shown on a screen, can be
   edited, has a "forget my style" button, and is never used for ranking or to
   judge the writer.
5. **Dashboard.** The writer sees their own numbers: drafts started, coach
   suggestions accepted, sources checked, publication progress of their own
   pieces. No comparison with other writers is shown unless staff switch on a
   leaderboard, which is off by default.
6. **Source packs.** A source pack is a named set of sources for one piece:
   web addresses, notes and, when the research toolkit (spec 0084) exists, its
   results with their provenance; until then a pack holds addresses and notes
   only. A pack can be shared with an editor. When the writer opens a pack,
   each address may be checked once on request ("check these links now")
   through the single-read rules of spec 0082 and the address protection of
   spec 0107; the result is shown but not stored, so nothing is tracked over
   time (DR-062).

### Sponsor content intake

1. The site hosts a form (an add-on page, with the site's own spam protection)
   where a sponsor can submit a brief or content. The add-on stores it as a
   note; Sutradhar collects it with the signed poll. Sutradhar opens no form
   and no port.
2. Limits: plain text only, at most `team.sponsor_max_chars` characters
   (default 20,000), at most `team.sponsor_per_hour` submissions per sponsor
   (default 10), and no attachments in version 1. Links in a submission are
   shown but never fetched by Sutradhar.
3. A submission is refused if it contains markup that could run or hide content
   (script tags, event attributes, hidden styling) or hidden characters such as
   zero-width text (SUT-1806). The check runs before anything is stored.
4. A submission goes into a queue visible to staff with the Reviewer template.
   It carries the sponsor's name and contact, kept only while the submission
   is open and `team.sponsor_keep_days` after it closes, unless a legal hold
   applies. The default is 90 days; deletion on request wins over it. The
   form shows the sponsor a notice about what is kept and for how long.
5. Every submission is shown with a "Sponsored" label and the checks below
   (disclosure, link marks, advertising standards). Nothing is published by
   Sutradhar; staff accept or refuse, and the site publishes by its own means.
6. Text is stored as plain text and escaped everywhere it is shown (spec 0111,
   escaping rules for templates and messages).

### Governance tools

Boundaries. Comments, mentions, the impact-ranked backlog and effort estimates
are in spec 0048 (features 541, 549 and 550). The decisions record here is not
a second history: settings history is spec 0014's and the audit is spec
0032's; the record holds the reason for a decision and links to them. The
feature-use statistics have nothing to do with the visit analytics of spec
0034. The data request log adds no second request table: a deletion request is
an erasure request of spec 0032, and the log links to it by id.

1. **Activity feed.** A feed built from the audit events of spec 0032 (and,
   for the hash-chained proof, spec 0036): who gave what, which suggestions
   were accepted, which grants were made or ended, and that a comment was
   added (the comment itself stays in spec 0048). The owner sees all of it; a
   site account sees only events about their own account and the work they may
   already see. Entries are kept `team.activity_keep_days`, and a page shows
   `team.feed_page` at a time.
2. **Decisions knowledge base.** A searchable record of decisions (what was
   decided, by whom, when, why and what it replaced), written by the owner or
   by staff with the Editor template and linked to the settings history of
   spec 0014 and the Fix list of spec 0048. Superseded and erased entries may
   keep only the safe lineage
   that 0032 permits. Protected text and references follow erasure.
3. **Training sandbox.** A practice mode on a made-up site from the lab (spec
   0116) and the test room (spec 0039), with made-up members and data. Nothing
   in it reaches a real site or a real table; every proposal in it is
   discarded when the session ends. It shows a banner so no one mistakes it
   for a real site.
4. **Access and retention policies.** One screen lists, for each kind of data
   in this spec, who may see it and how long it is kept, using the settings
   below. The owner may shorten retention but not lengthen it beyond the
   stated maximum, and may not turn off erasure.
5. **Feature-use statistics.** Counts of which screens and actions are used,
   kept only on this PC, never sent anywhere, off by default
   (`team.usage_stats`), shown to the owner as simple charts, and removed by
   the erase-all button. They hold counts and dates, never content or names.
6. **Quarterly planning board.** Goals for a period, each linked to Fix list
   items (spec 0048) and to decisions; shown as a board with progress taken
   from the linked items. It is the owner's local planning aid and changes
   nothing on a site.
7. **Stakeholder digests.** A short plain-English summary of the period's
   progress for people who do not use Sutradhar. It uses the board
   and any admitted 0094 report projection. It is shared as a
   snapshot file (spec 0100) or sent to an
   approved destination at the Headline level.

### Team records, decisions and protected plans (543–556)

Use one site and the exact 0101 grant at every governed read.
Admit native action mappings under QR-089 and QR-146 for feed read,
decision read and authoring. Bind actor, site, native predicate,
Cedar action, target and scope. Unmapped actions are unavailable.
The feed reads eligible 0032 events in site sequence and uses a
cursor bound to exact actor, grant generation, target and filter. Keep
comment body, private draft and credentials out of event text.
A revoked right invalidates queued feed pages and cursors.
A client read-only view uses its restricted 0048 projection.

A decision record holds site, version, actor, decision date,
reason, cited authorised references and superseded ID. Search
only permitted fields. Current settings and audit records remain
in 0014 and 0032. Supersession preserves safe lineage. Erase
sensitive text, references, protected search index and managed
snapshots on withdrawal. Supersession and restore recheck
erasure generations. Ordinary changes create decision revisions,
and never replace the settings history or audit.

Training opens a separate synthetic fixture and role set under
0039 and 0116. Use physically isolated writable storage, jobs and
keys. Reject real connector, event, model or file handles. No
production backup or network target. End, crash or expiry fence
jobs and replay, revoke handles and clear scratch. Allow an inert
reviewed draft export only, without IDs, roles or approval. Reusing
a command or template from practice creates a fresh local draft
with current review. Nothing is carried across as authority.

Access and retention lists use a versioned data kind, actor scope,
current grant, retention policy and actual effect date. Owner
approval cannot exceed row-level or native rights, disable erasure
or raise a retention ceiling. Check native roles again before
effective changes. A shortened retention rule schedules the
existing 0032 erasure path with current policy and evidence.
Closures never revive retired data from backup.

Feature-use counts are off by default. When enabled, record
only the admitted feature, screen or action IDs and site-day
aggregate. Deduplicate exact local events before increasing the
daily count. Keep no person, session, input or content. Recheck
owner scope and opt-in generation. Do not replay pre-opt-in work.
Switch-off stops intake. Erase-all revokes protected views and
queues under the ledger. Counts stay on the PC with no external
export, telemetry, visitor join or ranking use.

Preparation of a quarterly board freezes a site and calendar
interval with approved goal, item and decision IDs and units.
The owner may edit local goals. Progress uses current eligible
item states with unknown or held evidence. No native effect.
Digests freeze exact site, audience, source master, rights,
coverage, profile, body and file digest. Local 0100 copy and an
approved 0099/0100 Headline send are distinct effects. Hide
raw IDs, targets, comments, recipient details and revealing
counts. Recheck disclosure, grant, exact destination and
approval at publish or dispatch. One safe receipt per effect.
Unknown send holds. Restored approval cannot enqueue or reroute.

Display the existing permission templates with their exact
scope, expiry and proposed changes. New or edited templates
need current live owner review of all affected grants.
Precompile the exact Cedar candidate and digest. Compare expected
template, actor and affected grant generations and current native
rights. Atomically commit the policy, generation, audit and grant
changes. Revoke cached decisions and queue authority before reuse.
A failed compile preserves the valid policy without undoing an
independent revocation. Restore and restart recheck current
versions. Clone/import supplies a draft only. No ordinary
template can grant protected settings, keys, site mutation or
rights outside the native account's effective permissions.
Special owner-approved management grants require their own
confirmation and cannot clone or appear by implication.
An old policy replay cannot restore a revoked grant.

#### Wave 2 protected activity and decision bindings

Use the native admission and projection rules above. These synthetic
fixtures reuse the named verification contracts and do not add an
event collection route, identity system or second history.

| Feature | Requirement, design and planned fixture |
|---|---|
| F-0543 | TG-14 · DES-0101-W2-EXACT-ACTIVITY-CURSOR · FX-0101-W2-EXACT-ACTIVITY-CURSOR · UT-TG-12. Read eligible 0032 events for the authorised site account and work it may see. Include a private draft event, a safe comment-added event, a foreign site and a cursor after revocation. Check site, actor, grant generation, target and filter at delivery. Show only safe eligible facts. No draft or comment body or reuse of revoked cursors. |
| F-0547 | TG-15 · DES-0101-W2-SAFE-DECISION-LINEAGE · FX-0101-W2-SAFE-DECISION-LINEAGE · UT-TG-13. Search an eligible decision, create its authorised successor and revoke a cited work item. Erase protected text and restore older evidence. Leave only authorised safe supersession lineage under 0032. Remove protected text and references from search and managed outputs. Do not overwrite the existing settings history or audit. |

### Compliance helpers

These produce checks and drafts only. Each shows the rule it applied, with the
source's name and date and "unverified, not legal advice" until QR-054
confirms it. Every number, deadline and code named below is unverified until
then.

1. **Affiliate disclosure checker.** Finds affiliate links in drafts and
   published pages by a list of tracking patterns (built-in patterns from
   research, QR-054, plus patterns the owner adds) and checks for a visible
   disclosure before the first one, in the same paragraph or within 300
   characters. "Visible" means not hidden by styling or placement, using the
   visibility facts of spec 0115. A missing or hidden disclosure is a finding
   with the page and the rule.
2. **Sponsored content labels.** Checks that sponsored content has a visible
   label and that its links carry the right relation marks (spec 0105's
   receipts use the same marks). It can propose the label text and the link
   marks as a change for a human to accept.
3. **Accessibility statement draft.** Builds a draft statement from the
   accessibility facts of spec 0115 (and the checks of spec 0047), with the
   site's contact point and known gaps, in plain English. The owner edits and
   publishes it.
4. **Cookie policy draft.** Use the guest response facts of spec 0044
   for this site. List the observed names and attributes. Use a purpose
   only when the owner confirms it with evidence for that scope. Mark
   other purposes "purpose unknown". Show when and where the sample
   was taken and whether it was partial. With no observation say "no
   check has run". A sample with no cookie fields says "none observed
   in this sample" only when coverage completed. Show the reason for
   partial coverage, even with no names. The draft describes
   Sutradhar's own cookie-free default (spec 0034) separately.
5. **Data request log.** A log of privacy requests the site receives (access,
   correction, deletion). Record the receipt date, kind, opaque requester
   reference, reviewed due date or internal target, approved changes and
   reasons, state and audit link. Link deletions to the 0032 erasure ID.
   Raise a reminder `compliance.reminder_days` before the current target
   as set out below.
6. **Advertising-standards check.** For paid or sponsored content, flags
   claims that common advertising codes treat carefully (for example health,
   income or price claims, and "best" or "guaranteed" without evidence),
   listing the code section from the rule set the owner enables
   (`compliance.rule_sets`, sources from QR-054). It flags; it never rewrites
   or approves.

### Compliance evidence and review (629–634)

Keep evidence local to the selected site. Draft and published views
use the exact revision the reviewer can read. Display the evidence
scope, coverage, collection time and rule version. Findings are
`observed`, `missing_in_checked_scope`, `needs_review` or
`unknown`. Only a complete applicable observation can show that
a required fact is missing in the checked scope. None of these is a
legal decision. Missing records, partial observations and failed
parsing remain visible. Findings and drafts give no publish
authority, ranking signal or new outbound route.

QR-054 requires legal review before admission of a legal rule set.
An immutable rule set records site, jurisdiction, sector, audience,
rule ID, official source and location, digest, checked and
effective dates, applicability predicate, expiry and legal receipt.
The authorised reviewer and owner each bind the same digest.
Treat withdrawn or superseded sources as inactive until re-admitted.
Label
legal rules, provider technical criteria and editorial heuristics
separately. The owner enables an admitted legal version. If several profiles
or jurisdictions
apply, show separate findings and let a reviewer resolve conflicts.
Missing, expired, changed or unapproved profiles provide source and
evidence review only. Examples below are sources checked on
20 November 2026. Source citations cannot replace legal approval.

- **629, affiliate disclosure.** Classify each link using the
  owner-confirmed commercial relationship or approved tracking
  pattern. Flag a pattern-only match as suspected. Check the visible
  label against the exact first affected segment in reading order
  at the current supported rendered view widths. A wholly affiliate
  piece also needs evidence of the title or teaser seen before
  opening it where the admitted profile applies. For mixed content,
  check disclosure before each affected segment or link as required
  by that profile. The 300-character check is an editorial heuristic.
  If scope, visibility or relationship is unknown, ask for review.
  The [ASA/CAP guidance][cap-affiliate] describes the two cases.
- **630, sponsorship.** Read approved sponsor metadata or typed
  staff declarations. Check the visible label and tokenised
  relationship attributes on paid anchors independently. Keep
  unrelated attribute tokens in a proposed change. For the Google
  Search profile, prefer `sponsored` and recognise existing
  `nofollow` with an explanation. An unverified paid relationship
  creates a review item. A link attribute alone cannot satisfy a
  reader-facing disclosure. Use the [Google outbound links
  guidance][paid-links] and [ASA/CAP guidance][cap-affiliate].
- **631, accessibility statement.** Use dated, scoped WCAG findings,
  version and level, method, sampled pages, criteria, exclusions
  and approved contact details. Show tested, unavailable and untested
  areas. Use the [GOV.UK public-sector model][accessibility-model]
  only with an applicable profile. It includes preparation and
  review dates, known inaccessible content, alternative access,
  feedback and approved enforcement information. A voluntary
  factual draft is available without a public-sector profile.
- **632, cookie policy.** Group observed data by host, path, scan
  context and evidence revision. Use only 0044 admitted metadata,
  including permitted names, host/path scope and security attributes.
  Exclude values, callback parameters and identifiers. Keep
  malformed or conflicting headers as evidence gaps. Label each
  sampled consent state and technique separately. HTTP evidence
  cannot establish what scripts or other contexts store. No
  additional fetch, browser storage probe or consent change.
  Apply any exemption only under its admitted policy and exact
  facts, such as purpose and necessity. Use the [ICO storage and
  access guidance][ico-storage] and its [exceptions][ico-exceptions].
- **633, privacy request log.** Keep an opaque site-local requester
  reference and a single lifecycle record. Link deletion to its
  0032 erasure request with no duplicate erasure queue. For a
  supported request kind and jurisdiction, keep original receipt
  time and the permitted clock start separate. Record site time
  zone, identity or representative check outcome, dates and refs
  for permitted clarification, extension and notice, calculated
  deadline and closure receipt. Never store identity documents in
  this log. Keep original events and compare the current revision
  before adding a change. The admitted calculator follows the
  profile's month-end, weekend and holiday rules. Clarification
  pauses apply only when the exact request type and rule permit
  them. The ICO publishes distinct guidance for [access][ico-access],
  [rectification][ico-rectify] and [erasure][ico-erase]. Never present
  a fixed 30 days as the UK rule. Show any authorised staff
  determination as recorded staff review. If the legal clock
  cannot be proved, show “Check the legal due date” and keep any
  configured internal reminder date separate. Use the 0019 and
  0099 local job rules. Key reminders by site, request, deadline
  revision and reminder type. A new revision or closure cancels
  queued reminders. Notifications contain no case or person data.
- **634, claims.** Bind a claim span, evidence references, rule
  version and applicability to the reviewed content version.
  Flag a potentially verifiable claim when evidence is unread,
  missing or fails to support the exact terms, comparator, currency
  or date. Show material qualifications that need staff review.
  Use the [CAP misleading-advertising rules][cap-claims] for
  admitted UK non-broadcast advertising. A matched reference
  records evidence coverage only.

Store findings as `site_id, content_or_case_id, source_revision,
rule_profile_id?, evidence_id, scope, observed_at, outcome,
reason_code`. Mark previous findings historical if their input,
policy changes. Recheck current actor, object and allowed disclosure
via 0020 and 0108 before preview, copy, replay or reminder delivery.
Fence those effects against the current source, rule and case
revisions. Read native status only through its signed mapped route.
Permission loss hides current and historic findings. Specs 0032
and 0118 govern removal or restriction of cases, inputs, historical
findings and derivatives, including restore and redacted audit.

#### Wave 2 compliance evidence and request fixtures

Use synthetic local evidence and current site, object and role scopes.
QR-157 retains exact fixture and review admission. QR-054 still
governs legal rule version, applicability and legal and owner review.
An unadmitted or withdrawn rule gives evidence review only. These
fixtures add no collection, recipient, native write or legal verdict.

| Feature | Owning requirement and design | Planned fixture and expected evidence |
|---|---|---|
| F-0629 | TG-20, TG-21 and TG-25 · DES-0101-W2-VISIBLE-AFFILIATE-EVIDENCE | FX-0101-W2-VISIBLE-AFFILIATE-EVIDENCE · UT-TG-17, UT-TG-21 and BD-TG-02. Compare a wholly affiliate article and a mixed article with exact approved commercial scope, a pattern-only lead, hidden disclosure and incomplete or stale 0115 viewport evidence. Check the relevant title or teaser and each affected segment only under its applicable admitted profile and exact reading order. Pattern alone stays suspected. Keep an editorial distance check separate from a legal rule. No change or fetch. |
| F-0630 | TG-20 and TG-21 · DES-0101-W2-SEPARATE-SPONSOR-FINDINGS | FX-0101-W2-SEPARATE-SPONSOR-FINDINGS · UT-TG-18 and BD-TG-02. Use an authorised sponsor fact or staff declaration, exact visible label and 0105 relation facts for eligible anchors. Compare existing qualified and unqualified anchors, unrelated relation tokens and an unknown commercial relationship. Show separate disclosure and link findings. Keep unrelated tokens in any inert proposal. An unknown relation remains for review. No unapproved markup or publication. |
| F-0631 | TG-20 and TG-21 · DES-0101-W2-SCOPED-ACCESSIBILITY-DRAFT | FX-0101-W2-SCOPED-ACCESSIBILITY-DRAFT · UT-TG-18 and BD-TG-02. Use a dated 0047/0115 sample with a tested page, blocked page, limited coverage and an approved public contact point represented by a synthetic role address. State exact tested and untested areas, methods and applicable evidence. A stale or absent public-sector profile holds that template and leaves only the existing factual draft path where supported. No personal contact disclosure, legal assertion or public effect. |
| F-0632 | TG-20, TG-21 and TG-26 · DES-0101-W2-SCOPED-COOKIE-DRAFT | FX-0101-W2-SCOPED-COOKIE-DRAFT · UT-TG-22 and UT-TG-18. Compare no observation, complete own-origin main guest sample with no fields and partial 0044 capture with malformed or conflicting fields. Record the exact evidence scope, available purpose proof and separately observed consent context. No browser evidence gives no claim about script or storage behaviour. Withhold unproved purposes and exemptions. Use only admitted scrubbed attributes, never raw values or identifiers. No consent change or browser probe. |
| F-0633 | TG-22 and TG-24 · DES-0101-W2-EXACT-REQUEST-LIFECYCLE | FX-0101-W2-EXACT-REQUEST-LIFECYCLE · UT-TG-19 and IT-TG-04. Use eligible opaque site-local request records of distinct kinds, synthetic admitted clock profiles, calendar boundary and permitted clarification or extension events. Keep original receipt and approved deadline revisions. Without proof show the existing due-date review state and keep the internal target separate. Close or revoke before replay and fence obsolete local reminders. Link deletion once to the existing 0032 erasure ID. No identity document, duplicate queue or disclosure. |
| F-0634 | TG-20 and TG-21 · DES-0101-W2-EXACT-CLAIM-EVIDENCE | FX-0101-W2-EXACT-CLAIM-EVIDENCE · UT-TG-17 and UT-TG-18. Compare an objective price claim and a subjective comment using exact eligible revision and evidence. Include unavailable support, a mismatched time, currency or comparator, omitted qualifications and a revoked profile. Show reviewed source scope and concerns only where supported. A withdrawn profile permits evidence review without a rule verdict. No automatic rewrite, approval or release. |

[cap-affiliate]: https://www.asa.org.uk/advice-online/affiliate-marketing.html
[paid-links]: https://developers.google.com/search/docs/crawling-indexing/qualify-outbound-links
[accessibility-model]: https://www.gov.uk/guidance/model-accessibility-statement
[ico-storage]: https://ico.org.uk/for-organisations/direct-marketing-and-privacy-and-electronic-communications/guidance-on-the-use-of-storage-and-access-technologies/
[ico-exceptions]: https://ico.org.uk/for-organisations/direct-marketing-and-privacy-and-electronic-communications/guidance-on-the-use-of-storage-and-access-technologies/what-are-the-exceptions/
[ico-access]: https://ico.org.uk/for-organisations/uk-gdpr-guidance-and-resources/subject-access-requests/a-guide-to-subject-access/
[ico-rectify]: https://ico.org.uk/for-organisations/uk-gdpr-guidance-and-resources/individual-rights/individual-rights/right-to-rectification/
[ico-erase]: https://ico.org.uk/for-organisations/uk-gdpr-guidance-and-resources/individual-rights/individual-rights/right-to-erasure/
[cap-claims]: https://www.asa.org.uk/type/non_broadcast/code_section/03.html

### Exact writer and contributor scope (1303–1312)

Use existing site-account predicates and signed 0021 projections.
Effective access requires the current native right, the exact local
grant and Cedar approval for the same site, actor, action and object.
A stale claim, unknown mapping or unreachable required current check
holds access. Apply scope before titles, counts, search, pagination or
errors can reveal another person's work. Keep private writer material
on the owner's PC. Use 0032 erasure, 0108 site fencing and 0118 protected
transport and storage. A review, draft or permitted display creates
no new grant, site write, send or publication path.

| Feature | Exact account and evidence rule |
| --- | --- |
| 1303 Assistant profile per writer | The owner pins one site and writer to the approved profile, permitted content types and scope. Tone, topics and reading level affect presentation only. Keep required checks and policy independent. The owner starts each enrolled local assistant session under 0020/0118. A writer profile cannot start a client, issue a token, widen a grant or permit off-PC processing. |
| 1304 Personal style memory | Use only the exact writer's permitted accepted edits, with the existing derived-feature and byte bounds. Keep raw text, private identifiers and other writers' examples out. Apply the admitted brand rules and scope. Edit or forget under current authority. Forgetting advances the erasure generation and holds queued, restored or dependent work that would relearn from old data. No ranking, writer judgement or global training. |
| 1305 Writer dashboard | Show the native site's admitted own-account view of drafts, deadlines and permitted progress. Define each count and source revision, distinguish planned and observed states and apply scope before aggregation. No other writer's data, hidden title or private deadline through totals, filters or replay. Any separately approved team report keeps its own authority. The dashboard makes no assignment or reminder. |
| 1306 Personal drafts | Reuse the 0042 draft store with exact writer, site, draft and revision. A metadata grant permits only its approved fields. Reading private text requires current ownership or explicit sharing for the exact audience and purpose. Check expected version at autosave or edit. Revocation and erasure fence old diffs, exports and restore. No automatic transfer, profiling, provider processing or native edit. |
| 1307 Writer source pack | Bind the exact owned or shared piece, approved sections, eligible 0084 evidence and source revisions, grant, purpose and expiry. Hold wrong-site, stale, private or revoked material and any leak through descriptions, excerpts and counts. Packing or restoring starts no link check. The previously admitted owner-approved one-off link check retains its separate authority. Sharing a pack cannot grant access to the source. |
| 1308 Trusted member writer | Require current `trusted.manage` and the exact applicable criteria for each mark or revocation. No self-mark or self-revoke. Bind the member and current draft-only right. A qualified trust mark can affect the existing local review order and eligible advisory checks under the owning policy. It never waives a required privacy, safety, rights, disclosure, site policy or 0089 submission gate. Recheck at each intake and review. No MCP account or token, self-grant or publication effect. |
| 1309 Expiring guest access | Require current `grants.manage`, no self-grant, native site account and exact draft-only grant. Cap expiry by `team.max_grant_days`. Retain the existing independently scoped expiry reminder. A later grant or extension needs a fresh owner-authorised human Accept. Bind the named human Accept to site, account, scope, start and end and the expiry removal it covers. Deny locally at the current end even if the site cannot be reached. A separately admitted signed expiry action may remove only that grant under the original authority, current original grant ID and version, signed request and one effect receipt. Hold stale or uncertain clock or restore state. Never delete an account, remove newer grants or open MCP. |
| 1310 Sponsor intake | Use the already scoped native sponsor form and signed 0021 pull through the owner's admitted processing session. Bind the exact site, eligible submitter, intake ID, revision, consent and purpose, and enforce signed identity and existing size, rate and plain-text limits before native note storage and independently on owner pull. Hold markup, attachments, hidden content, stale consent or mismatched signed replies. Scope review to the current permitted Reviewer, use existing sponsor contact retention, keep contact details out of exports and require exact rendered sponsor disclosure. No external fetch, CRM reuse, message or automatic publication. |
| 1311 Agency writer access | Use the client's approved native site-account relationship and named writer, section, expiry and draft-only grant. The authorised agency owner starts only their enrolled local session, with one site and the exact current acting account. Require the admitted installation and site pairing proof before any protected view. No direct writer or client MCP enrolment, remote bridge, cross-site draft, portfolio or client-wide disclosure. An unadmitted agency deployment stays unavailable. |
| 1312 Writer offboarding | Use the existing named human offboard action, binding exact account, site, source generations and plan. Fence the account and queue first. Apply TG-09: drop pending notes and record scoped disposition, end exact mapped tokens and grants, offer writer-only encrypted export within current approval, move only authorised shared revisions and schedule existing retention. Keep SUT-1808 until the resumable steps finish and keep an opaque receipt per step. Reconcile restart and discard stale restored authority. Native account or access changes remain conditional on separately admitted existing authority. Erasure takes precedence over export and retention. |

QR-195, QR-055 and QR-089 qualify the typed native and local scope,
grant and erasure races, exact bounded projections and any already
authorised effect. An unavailable platform route gives a safe hold.

#### Wave 3 writer and contributor fixture bindings

Use synthetic staff, site pairs, native right replies and
content. These cases inherit QR-195, QR-055 and QR-089.
ACCT-RIGHT proves the signed native predicates in 0021.
The fixtures add no route, grant, acceptance or native effect.

- `DES-0101-W3-EXACT-WRITER-PROJECTION` separates own-account
  presentation and eligible private content from team or
  foreign-site information before building a result.
- `DES-0101-W3-CURRENT-DRAFT-ROLE` combines exact current
  native rights, local scope and Cedar without exceeding any.
- `DES-0101-W3-SIGNED-PROTECTED-INTAKE` checks the existing
  signed path, bounded input and permitted purpose.
- `DES-0101-W3-FENCED-WRITER-OFFBOARDING` preserves TG-09's
  ordered receipts, first-step access fence and erasure priority.

| Feature | Owning requirement and design | Named fixture and observable check |
| --- | --- | --- |
| F-1303 | TG-02, TG-10 · DES-0101-W3-EXACT-WRITER-PROJECTION | FX-0101-W3-OWNER-SCOPED-WRITER-PROFILE · UT-TG-02 and AT-LOCAL-TRANSPORT. Apply an eligible tone profile to the exact writer, site and content type in an owner-started enrolled local session. Try another writer, site, unenrolled process and profile that removes required checks. Only presentation changes. No wider grant, token, automatic session or off-PC route. |
| F-1304 | TG-11 · DES-0101-W3-EXACT-WRITER-PROJECTION | FX-0101-W3-STYLE-FORGET-PREVENTS-RELEARNING · UT-TG-09 and PL-TG-03. Use only the writer's permitted accepted edits, compare the stated term and byte caps with one over, and try another writer's text and a private identifier. Race Forget my style with queued learning and restore an older snapshot. Old features and dependent learning stay fenced. Keep only permitted bounded derived features, without raw text, ranking or writer judgement. |
| F-1305 | TG-02, TG-10, TG-12 · DES-0101-W3-EXACT-WRITER-PROJECTION | FX-0101-W3-OWN-DASHBOARD-SCOPE-BEFORE-COUNT · UT-TG-02 and UT-TG-10. Use exact native own draft, deadline and expected publication records. Add another writer's title and deadline, a foreign site and a stale observation. Try totals, filters and page boundaries. Only eligible own figures appear. Keep planned and observed progress separate. The view creates no reminder or assignment. |
| F-1306 | TG-10 · DES-0101-W3-EXACT-WRITER-PROJECTION | FX-0101-W3-DRAFT-METADATA-AND-TEXT-AUDIENCE · UT-TG-08 and IT-TG-02. Compare the exact approved metadata-only view with current writer or explicit shared text access in the 0042 store. Submit an old autosave version, revoke the text grant and replay an old diff, file and restored reference. No private text in a metadata response, stale overwrite, renewed grant or native edit. |
| F-1307 | TG-02, TG-10, TG-27 · DES-0101-W3-EXACT-WRITER-PROJECTION | FX-0101-W3-SOURCE-PACK-RECHECK-RIGHTS · UT-TG-23 and IT-0084-RIGHTS. Open an owned or shared pack with exact piece, sections and current source rights. Withdraw a source and try a private annotation and sibling-site reference. Withhold affected fields before counts and excerpts. Opening, sharing and restoring a pack and an already used check request start no read. Separately exercise the existing owner-approved explicit one-off check for one eligible synthetic public address under 0082 and 0107. Show its bounded result for the current request only. Keep no check result or history. Refuse an unapproved or withdrawn address. Replaying the used request starts no second read. Sharing a pack supplies no delegated source right. |
| F-1308 | TG-02, TG-06, TG-08 · DES-0101-W3-CURRENT-DRAFT-ROLE | FX-0101-W3-TRUSTED-WRITER-KEEPS-HARD-GATES · UT-TG-06 and UT-TG-07. Compare the exact current trust criteria at the threshold and just below. Try self-mark, native right loss, revoked or restored trust, failed required check and direct MCP. Only current eligible trust affects the existing draft-only review order and advisory checks. No required gate waiver, token or publishing. |
| F-1309 | TG-03, TG-04, TG-05, TG-06 · DES-0101-W3-CURRENT-DRAFT-ROLE | FX-0101-W3-GUEST-EXPIRY-EXACT-GRANT · UT-TG-04 and FI-TG-02. Bind one named human Accept to the original grant, start, expiry and covered removal. Cross the exact end while disconnected and resume against a replacement grant. Race a stale retry, old restore and unauthorised extension. Deny at expiry even without the reminder. Only an admitted signed effect may remove the exact original grant once. Keep newer access and the native account untouched. No MCP. |
| F-1310 | TG-01, TG-13, TG-21, TG-29 · DES-0101-W3-SIGNED-PROTECTED-INTAKE | FX-0101-W3-SPONSOR-VALIDATE-BOTH-BOUNDARIES · UT-TG-11 and PL-TG-04. Pull one eligible signed plain-text note and replay its receipt. Try inert non-plain markers, attachments, exact size and rate limits and one over before native storage and independently on owner pull. Revoke consent before review and remove rendered disclosure before hand-off. One scoped reviewer draft or refusal. Contact stays out of exports, with no link fetch, message or publication. |
| F-1311 | TG-02, TG-03, TG-05 · DES-0101-W3-CURRENT-DRAFT-ROLE | FX-0101-W3-AGENCY-OWNER-ONE-SITE · UT-TG-02, AT-LOCAL-TRANSPORT and ACCT-RIGHT. Use the client's admitted installation and site pairing, named agency writer, section and current draft-only grant. Swap the client, acting account or section and expire the grant. Try direct writer or client enrolment and a remote relay. Only the agency owner's authorised enrolled local session sees one client's exact scoped draft view. |
| F-1312 | TG-09, TG-10, TG-24 · DES-0101-W3-FENCED-WRITER-OFFBOARDING | FX-0101-W3-OFFBOARD-RESTART-AND-ERASURE · IT-TG-01, FI-TG-01 and IT-0118-LOCAL. Interrupt after each resumable step, change an intervening grant or transfer target revision, erase before the optional export and restore old grants. Fence account, queue and mapped tokens before any further access. Resume exact eligible steps once by opaque receipt, retaining SUT-1808 while incomplete. Only authorised shared work moves and erasure prevents export. Native changes need the separately admitted route. |

### Data

- `grants`, `templates`, `trusted_writers`, `offboardings`: with site column and
  forced row-level security (spec 0108).
- `writer_profiles`, `style_memory`, `drafts`, `draft_versions`, `source_packs`:
  per site and account, encrypted at rest.
- `sponsor_submissions`, `activity_feed`, `decisions`, `planning_items`,
  `usage_counts`, `data_requests`, `compliance_findings`: per site.
- Sandbox data is kept in a separate lab area and never in a real site's tables.

### Settings

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `team.max_grant_days` | days | 365 | 1 to 1,825 (proposed) | next grant |
| `team.claim_max_age_s` | seconds | 300 | 10 to 3,600 | next request |
| `team.note_max_age_s` | seconds | 86,400 | 60 to 604,800 | next poll |
| `team.trusted_min_age_days` | days | 90 | 0 to 3,650 | next mark |
| `team.trusted_min_accepted` | whole number | 10 | 0 to 1,000 | next mark |
| `team.autosave_s` | seconds | 30 | 5 to 600 | next draft |
| `team.feed_page` | whole number | 50 | 10 to 500 | next view |
| `team.sponsor_max_chars` | characters | 20,000 | 100 to 200,000 | next submission |
| `team.sponsor_per_hour` | whole number | 10 | 1 to 1,000 | next submission |
| `team.expiry_notice_days` | days | 7 | 0 to 60 | next night |
| `team.offboard_keep_days` | days | 90 | 0 to 365 | next night |
| `team.draft_versions` | whole number | 20 | 1 to 500 | next save |
| `team.activity_keep_days` | days | 180 | 7 to 1,095 | next night |
| `team.sponsor_keep_days` | days | 90 | 0 to 730 | next night |
| `team.usage_stats` | on or off | off | two values | at once |
| `team.leaderboard` | on or off | off | two values | at once |
| `compliance.request_due_days` | days for internal target | 30 | 1 to 90 | next request |
| `compliance.reminder_days` | days | 7 | 0 to 30 | next night |
| `compliance.rule_sets` | list | none until QR-054 | rule sets with a source | next check |

There is no setting that gives a grant more than the site account holds, lets
a template hold a never-changeable permission, lets Sutradhar accept a
connection from outside, or turns off erasure or the audit of grants.

### Failure codes

SUT-1800 grant expired; SUT-1801 grant revoked or account lost rights;
SUT-1802 request not signed or signature invalid; SUT-1803 grant would exceed
the site account's rights; SUT-1804 template holds a forbidden permission;
SUT-1805 draft not shared with this reader; SUT-1806 sponsor submission
refused (script or hidden text); SUT-1807 data request overdue; SUT-1808
offboarding incomplete; SUT-1809 sandbox action refused (real site); SUT-1810
compliance rule set missing a source; SUT-1811 current site right
unavailable or reply failed validation. Codes SUT-1800 to SUT-1899
are reserved for this spec.

### Security and privacy

- Requests from site accounts are accepted only with a valid signature from the
  site link; nothing is read from an open connection.
- Private drafts, style memory and source packs are visible only to their
  account and what the writer shares; they are encrypted at rest and removed
  by erasure.
- Style memory is never used for ranking, never shared and never trained on
  other people's text.
- Sponsor submissions, feed entries and data requests hold no more personal
  data than needed and have retention limits.
- Everything that changes access is audited with who, when and why.

### Wave 2 team controls and owner planning bindings

Use synthetic sites, role claims, clocks and receipts. Require the
existing native action admission and exact current grant. QR-146
still owns role and profile evidence. A fixture does not introduce
a permission, destination, write route or production storage.

| Feature | Owning requirement and design | Planned fixture and expected evidence |
|---|---|---|
| F-0551 | TG-16 · DES-0101-W2-ISOLATED-PRACTICE | FX-0101-W2-ISOLATED-PRACTICE · IT-TG-03 and PL-TG-05. Open a synthetic 0039/0116 fixture in the separate practice scope with its banner. Try a production model, worker, file and backup handle. Refuse before any usable handle or real action. Interrupt cleanup, then fence practice jobs and replay and clear scratch. An admitted inert draft export carries no real data, reusable grant or approval. |
| F-0552 | TG-23 · DES-0101-W2-RETENTION-FENCE | FX-0101-W2-RETENTION-FENCE · UT-TG-20 and the existing 0032 erasure checks. Preview and confirm one shortening for the exact data kind and site under the current owner and native grants. Refuse above-ceiling retention, disabled erasure and restored expired rows. Apply the existing expiry and erasure fence before physical cleanup. Record safe current scope and actual effect date. |
| F-0553 | TG-17 · DES-0101-W2-LOCAL-FEATURE-COUNTS | FX-0101-W2-LOCAL-FEATURE-COUNTS · UT-TG-14. Start off, enable with owner approval, replay one eligible event, switch off, erase and restore older batches. One site-day count per eligible event for the declared feature ID. No pre-opt-in or erased output returns. Release no person, session, site content, visitor analytics, rank input, external export or telemetry. |
| F-0554 | TG-18 · DES-0101-W2-CURRENT-QUARTER-EVIDENCE | FX-0101-W2-CURRENT-QUARTER-EVIDENCE · UT-TG-15. Pin one site and calendar interval. Use authorised linked goals, decisions and 0048 items. Change an item state, erase a decision and try a foreign goal. Update displayed progress only from current eligible item states, keep safe gaps and reject the foreign link. Board edits are local and cannot verify work or change the site. |
| F-0555 | TG-19 · DES-0101-W2-EXACT-DIGEST-HANDOFF | FX-0101-W2-EXACT-DIGEST-HANDOFF · UT-TG-16 and IT-SD-01. Compare a scoped safe local snapshot with a stand-in for a previously admitted 0099/0100 Headline delivery. Bind source, coverage, audience, body and file digest. Refuse richer content, new recipient and grant loss at hand-off. Simulate a lost acknowledgement and keep the effect held under the original receipt. No blind repeat or restored send approval. |
| F-0556 | TG-03 and TG-07 · DES-0101-W2-EXACT-TEMPLATE-POLICY | FX-0101-W2-EXACT-TEMPLATE-POLICY · UT-TG-03, PL-TG-02 and BD-TG-01. Preview exact affected grants and require current live owner approval. Fail the Cedar precompile and race independent revocation. Preserve the valid policy without reversing the revocation. A valid candidate requires current native rights and conditional atomic policy, grants and audit commit. Restart and old cache cannot revive rights. Clone or import gives a draft only. |

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| TG-01 | Every note from a site account MUST arrive in a reply signed by the add-on and verified under spec 0021, its id MUST be new and it MUST be younger than `team.note_max_age_s`, or it MUST be refused (SUT-1802). | UT-TG-01, PL-TG-01 |
| TG-02 | A request MUST be allowed only if the site says the account holds the right, a grant gives it and the rules allow it. | UT-TG-02 |
| TG-03 | A grant MUST NOT give more than the site account holds (SUT-1803), and a template MUST NOT hold a never-changeable or direct site-change permission (SUT-1804). | UT-TG-03, PL-TG-02 |
| TG-04 | A grant MUST end at its end date by Sutradhar's own clock, even when the site link is down, and refuse the account at the next note or call (SUT-1800); a reminder MUST be sent before the end. | UT-TG-04, FI-TG-02 |
| TG-05 | A rights claim older than `team.claim_max_age_s`, or any token call, MUST first re-read the account's rights from the site, and with the site unreachable the request MUST be refused (SUT-1811); a lost right or ban MUST then stop access (SUT-1801). | UT-TG-05, FI-TG-02 |
| TG-06 | No one MUST be able to give a grant or a trusted mark to themselves, and only `grants.manage` and `trusted.manage` MUST allow them. | UT-TG-06 |
| TG-07 | Changing a template MUST preview who is affected and be audited. | BD-TG-01 |
| TG-08 | A trusted-writer mark MUST need the stated criteria, keep the hard checks, never block on soft checks, be revocable and audited, and MUST NOT carry any permission to change a site. | UT-TG-07 |
| TG-09 | Offboarding MUST run as recorded steps that resume after an interruption, end tokens and grants, drop pending notes, offer an export, move shared work, keep data only for the stated days and audit each step (SUT-1808 while incomplete). | IT-TG-01, FI-TG-01 |
| TG-10 | A writer space MUST be private to its account from other accounts and staff, readable by staff only if shared (SUT-1805), encrypted at rest, use the draft store of spec 0042, show the writer the stated notice, and every table of it MUST be an erasure target of spec 0032. | UT-TG-08, IT-TG-02 |
| TG-11 | Style memory MUST hold derived features only within 64 KiB, use only the writer's own accepted text, be viewable and editable, be deleted by "forget my style", and MUST NOT affect ranking. | UT-TG-09, PL-TG-03 |
| TG-12 | No writer comparison MUST be shown to a writer unless the leaderboard is on. | UT-TG-10 |
| TG-13 | A sponsor submission MUST arrive as a signed note, be plain text within its size and rate limits with no attachments, be refused if it holds runnable markup or hidden characters (SUT-1806), show the Sponsored label, never have its links fetched, and never be published by Sutradhar. | UT-TG-11, PL-TG-04 |
| TG-14 | The activity feed MUST show a site account only events about their own account and the work they may see. | UT-TG-12 |
| TG-15 | Decisions MUST never be deleted, only marked superseded. | UT-TG-13 |
| TG-16 | The training sandbox MUST use only made-up data, show a banner, and MUST refuse any action that would reach a real site (SUT-1809). | IT-TG-03, PL-TG-05 |
| TG-17 | Feature-use statistics MUST be off by default, stay on the PC, hold only counts and dates and be removed by erase-all. | UT-TG-14 |
| TG-18 | The planning board MUST change nothing on a site. | UT-TG-15 |
| TG-19 | A digest MUST be sent only as a snapshot or to an approved destination at the Headline level. | UT-TG-16 |
| TG-20 | Each compliance finding MUST name its rule, source and date and say it is unverified until confirmed, and a rule set with no source MUST be refused (SUT-1810). | UT-TG-17 |
| TG-21 | The affiliate, sponsored-label, accessibility, cookie and advertising checks MUST produce findings or drafts only and MUST NOT publish or rewrite a page. | UT-TG-18, BD-TG-02 |
| TG-22 | A data request MUST have a due date and a reminder, MUST be tracked to completion through the erasure of spec 0032 where it is a deletion, and an overdue one MUST raise SUT-1807. | UT-TG-19, IT-TG-04 |
| TG-23 | The retention settings MUST be enforced, shortening MUST be allowed and lengthening beyond the maximum MUST be refused. | UT-TG-20 |
| TG-24 | Every change to access MUST be audited with who, when and why, and the audit MUST hold no private content. | IT-TG-05 |
| TG-25 | The affiliate disclosure check MUST find a missing or hidden disclosure before the first affiliate link within the stated distance. | UT-TG-21 |
| TG-26 | The cookie draft MUST say "no check has run" when the crawler has recorded no cookie facts. | UT-TG-22 |
| TG-27 | Source pack link checks MUST run only on request, store no result and follow the single-read rules of spec 0082. | UT-TG-23 |
| TG-28 | A site account the site marks under its minimum age MUST NOT receive a grant. | UT-TG-24 |
| TG-29 | The settings of the table, including claim age, note age, trusted criteria, autosave, feed page and sponsor limits, MUST be enforced. | UT-TG-25 |
| TG-30 | A request decision MUST complete within the stated target on the 8 GiB reference. | MS-TG-01 |

## How it is tested

Designed, not run.

- **Unit (UT-TG):** the three-way permission decision on a table of cases;
  grants at their end date with one value just inside and one just outside;
  template checks; draft visibility; style memory scope; the retention
  settings.
- **UT-TG-05:** a stale claim or token call uses ACCT-RIGHT (spec
  0021). Wrong action or target, a lost right and an unavailable
  answer all refuse. A signed allow with an expired local grant or
  a Cedar deny still refuses. Also refuse a fresh note with a
  mismatched actor, target or action, or unsupported claim version.
- **UT-TG-22:** absent, empty, partial and populated cookie samples
  produce different draft text. An unknown purpose stays unknown.
- **Planted faults (PL-TG):** an unsigned request; a grant above the account's
  rights; a style-memory read of another person's text; a sponsor submission
  with hidden text; a sandbox action aimed at a real site; each must be
  refused with its named message while an honest case passes.
- **Integration (IT-TG):** offboarding across drafts, tokens and the feed;
  erasure through private spaces; a data request through to its deletion.
- **Fault injection (FI-TG):** offboarding stopped halfway; a site link that
  goes away during a grant check.
- **Behaviour (BD-TG):** the Priya scenario above, step by step; a compliance
  check on a page with a hidden affiliate disclosure.
- **Measurement (MS-TG-01):** the median time to decide one request on the 8
  GiB reference must stay under 50 ms (proposed), and the 99th percentile
  under 200 ms.

## Changes from the source

The old list assumed a multi-user cloud product. Sutradhar is the owner's
desktop app, so other people act through the sites' own sign-in and signed
messages, and these features become permission templates, grants and per-
account data, not a login system. Teams features that need a live shared web
screen are not built; the feed and board are the owner's screens and read-only
views through the site pages. Features 541, 549 and 550 stay with spec 0048
(the register is corrected). The signed-note flow reuses spec 0021, and its
contract needs a signed route to read an account's current rights (QR-055).
Compliance rules are drafts and checks, not legal advice, and wait for sources
(QR-054) and legal review (QR-010).
