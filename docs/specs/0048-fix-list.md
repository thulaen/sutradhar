# 0048: Fix list

The Fix list is a product feature. It answers one question for a site
owner: what is wrong with my site, who is on it, and is it really fixed?
Each finding Sutradhar makes about a registered site becomes one item on
that site's Fix list. An item closes only when a recheck of the live
pages proves the fix, and reopens if the fault returns. A "What to do
first" view says which items to take on this cycle. The module is
`work_graph`; the screen label is "Fix list". It is part of roadmap
Phase 6; "Which spec designs each feature" in [features.md](features.md)
lists the features it designs in whole or in part. They include tasks,
approvals, deadlines, playbooks, freeze windows and backlog by impact,
served as one list.

Example: the health report finds 212 thread pages with a wrong canonical
address. One item opens. The owner says "I fixed it"; it reads "fix
claimed" until the crawler re-reads those pages: "212 of 212 right".
Weeks later a theme update undoes it. The next crawl finds 40 wrong:
"came back once".

## What it does

### Three lists, never mixed

| List | Holds | Where |
| --- | --- | --- |
| Fix list (this part) | Faults in a registered site | Product, per site |
| autoissues | Faults in Sutradhar itself | Scrubbed and held on Dell (spec 0063) |
| Forgejo | The project's own plans and tasks | Dell |

1. One question sorts them: is it wrong with the owner's site, or with
   Sutradhar? Site faults are never copied to Forgejo or autoissues.
2. A Sutradhar fault, such as a crashing check, never goes on the Fix
   list. The item shows "check could not run" and its error code; the
   fault goes to autoissues, scrubbed and held.
3. The Fix list follows the same lifecycle rules as autoissues: a repeat
   updates one record, only the checker verifies, a return reopens, a
   flood cap, and "won't fix" needs a reason. Whether one Rust rule
   engine serves both is decided at build. The stores stay apart, and
   autoissues is left out of product packages.

### What becomes an item

1. A health-report finding (spec 0044), once per cause, not per page.
2. A site alert, such as robots.txt blocking everything.
3. A link-health finding, such as a stranded or dead page.
4. A block page on a registered site (spec 0045).
5. One a person types, an imported spreadsheet row, or a rule action. An
   editor may add a Coach tip with one "add to Fix list" click; the Coach
   stays advice only.
6. The key is site, check and cause. A repeat while open updates that
   item. A fault returning after "verified" reopens the same item.
7. A run files at most `work.flood.items_per_run` new items. Past that
   the screen says how many wait, and they get items on later runs. The
   health report still lists every finding.
8. An item holds ids, counts, dates and the check's name: no post text,
   no member names. Addresses are built when shown, for guest-visible
   pages only. History is append-only.
9. Only staff with permission for that site see the list. Guests see
   nothing.

### States

| State | Meaning | Set by |
| --- | --- | --- |
| Open | Found; nobody on it | Sutradhar |
| Taken | A staff member took it | That person |
| Fix claimed | Person says fixed; recheck due | That person |
| Verified | A recheck found every named page right | Recheck only |
| Closed | Verified, then right for N clean crawls | Sutradhar |
| Came back | Fault returned; shown as open, with a count | Recheck |
| Marked fixed, not checked | Person says fixed; no recheck possible | That person |
| Won't fix | Accepted on purpose; written reason | Admin |
| Page gone | Every named page gone or not guest-visible | Recheck |

1. Open work is open, taken, fix claimed and came back.
2. "Marked fixed, not checked" needs a written reason, is flagged
   everywhere, and is never counted as fixed or used in any
   before-and-after test.
3. Verified is not "helped". Later the Coach's before-and-after test may
   add "helped", "not shown" or "not enough data". It never decides
   closing.
4. Only the recheck sets verified, came back and page gone. A person, an
   MCP agent or an outside tool asking for them is refused, and the
   refusal is logged. An agent may comment, assign, claim and start a
   recheck.

### The recheck

| Kind | Proves the fix by |
| --- | --- |
| Page facts | The crawler re-reads the item's pages |
| Site setting | A read-only database read of the option |
| Search data | Search Console, GA4 or darshak after enough days |
| Placed link | The link check that staff suggestions (spec 0028) runs |
| None | Nothing can be checked: "marked fixed, not checked" only |

1. Page facts use check my fix (spec 0044): only the pages named in the
   item, guest view, allowed hosts, the crawler's rate limit and slowing
   rule, a fresh copy, never a stored one. It changes nothing. Each result
   is stamped with time and page version; one older than the claim does
   not count.
2. The item shows "211 of 212 right" and names the page left. Verified
   needs every named page right. A page the person excludes is listed
   with a reason.
3. An unreadable page (timeout, 429, 5xx, robots denial, block page,
   paused crawl) proves nothing. It neither verifies nor reopens:
   "recheck due".
4. A fresh eligible 404, 410 or independently proved loss of guest
   visibility is "page gone" for that exact named page. A blocked or
   unread attempt gives no such proof. A 404 after a redirect to a
   different page does not establish that the named page is gone.
   Keep the approved page set and its per-page results for the
   whole recheck. The item becomes "page gone" only when the exact
   current evidence proves that outcome for every named page.
   When some pages are right and others gone, keep a claimed fix
   due for recheck and show both results. An unread page in that
   scope also holds item closure. No gone or unread row counts as
   right or silently leaves the denominator.
5. Search-data rechecks say "not enough evidence" below their admitted
   floor; a recheck whose floor is blocked cannot run.
6. Every signal in the registry (spec 0044) names its recheck kind, or
   "none" and why. An entry without one fails the gate.

### The return rule

1. After each routine crawl and changed-page pass, each rule runs again
   on the stored facts, with no new fetching. The crawler reports; the
   item's rule decides. The crawler never closes work.
2. Any page in scope failing again reopens the item as "came back",
   showing old pages and new ones (for example 40 pages, 12 created since
   the fix). The count rises by one and the earlier proof stays.
3. After `work.return.flag_after` returns the item is flagged: "keeps
   coming back; look at the cause".

### Default order

The Coach's order of checks: permission, freshness, sample size and
safety first (failures held back, with the reason); then gap size,
support, effort, and risk to pages doing well. No money value is
invented. A gap not measured shows "gap not measured", never zero.
Recency is a choice.

### Fields and teams

1. An item has: site, check, cause, pages, owner, watchers, labels, due
   date, effort estimate, comments and history. The owner is one staff
   account of that site, or "unowned, for team X". No item is ownerless.
2. Staff accounts, roles and access rules are the platform's. No second
   user system.
3. A comment can point at a page or a report, built from ids.
4. Labels are admin-made words. Typed custom fields wait for research on
   need.
5. Groups: an item may hold child items, to `work.max_depth` levels
   (research asks whether two are enough). A parent cannot be verified
   while a child is not. A parent's gap size adds only items of one kind
   and source, and counts a page named twice once.
6. Waits: "item B waits for item A", as for jobs. A loop of any length is
   refused when made, with the path named. A wait advises people; for a
   rule it binds, and no rule starts a blocked item.

### Import and export

1. An admin picks a site, drops a CSV file, checks the guessed columns
   and saves the mapping. A saved mapping (per site, never shared) makes
   the next import of the same shape one click. XLSX needs a pinned,
   admitted Rust parser first.
2. Every row must name a page on that site's allowed hosts. Any other row
   is refused in the per-row report with its reason. No placeholder is
   made for an unknown address, so other clients' and rivals' pages stay
   out.
3. A partial import shows how many rows passed and each failure with its
   reason. The admin chooses to go on; the choice is recorded. No row is
   dropped silently.
4. A row becomes an item sourced "import", with file and row number. The
   mapping must give each item a recheck kind, or it is "marked fixed,
   not checked".
5. Files are limited by size, rows and cell length. Cells starting `=`,
   `+`, `-` or `@` are kept as text and neutralised on export, as in the
   safe exports of spec 0032. The parser is fuzzed.
6. Export is CSV first; a client-ready XLSX needs an admitted writer.
   Each row links back to the live item and needs a staff sign-in. Export
   leaves out what the exporter may not see.

### Saved audits

1. An audit is a saved run of a chosen check list on one site. A
   template is the list, its limits and its weights. Playbooks for
   migration, launch and recovery are templates.
2. A run is sealed: template version, page list, crawl run and per-check
   results are kept and never change. Page copies are not kept beyond
   their 30-day rule.
3. Comparing two runs of one template gives fixed, got worse, new and
   unchanged from the sealed results. Nothing is re-run, so a diff stays
   true after check code changes. Page-level changes use compare two
   crawls.
4. A failed check makes an item by the usual key. A check that cannot
   name a recheck cannot ship in a template.
5. Counted facts only; no single audit score unless research shows it
   changes a decision, and then always with its parts and spread. Each
   number shown is listed in the metrics catalogue
   (`catalogue/metrics.toml`) with its source, decision and comparison,
   or it is not shown.

### Goals

1. A goal is a listed metric, a target and a date. Sources are Search
   Console, Bing, GA4, darshak and own analytics, never added together.
   No actual is typed in.
2. Status comes from a ganit forecast with a range: on track, at risk,
   off track, or "unforecastable" with the reason. An abstaining forecast
   shows that, never a green tick from one guess.
3. One grouped alert when the forecast says the target will be missed
   while time remains. Quiet hours apply.
4. Driver tree: a goal split into child goals with weights. Declared
   weights show as declared; fitted ones show their uncertainty.
5. Links to work, kept apart: "items expected to move this goal", by gap
   size, labelled an expectation; and "items that did", from verified
   items with a before-and-after result.
6. Listed metrics and fixed operators only. There is no free formula
   language.

#### F-0314 versioned goal alerts

REQ-0048-W2-GOAL-ALERT and DES-0048-W2-FORECAST-RELEASE
use the existing Goals rule and 0019 and 0099 notice route.
FX-0048-W2-GOAL-ALERT and IT-0048-W2-GOAL-ALERT compare
the four supported outcomes using a synthetic admitted
forecast profile. Replay a qualified miss before its target
date. Change the target revision, cross quiet hours and try
a late job after expiry. Send the single eligible local notice
for the exact live goal version. An expired, replaced or
unforecastable goal supplies no alert. An unadmitted forecast
profile keeps the action unavailable.

#### Wave 1 goal progress binding

F-0079 · REQ-0048-W1-GOAL-PROGRESS · DES-0048-W1-GOAL-EVIDENCE.
Show the owner's metric, target and date alongside comparable observed
progress from the goal's selected source. Missing or incompatible
intervals remain unknown. Keep expected contributions apart from measured
changes. Show the strongest supported contributors only within comparable
evidence and carry the uncertainty and limits from 0094. A nearby change
alone supplies no measured contribution.

FX-0048-W1-GOAL-PROGRESS · BDD-0048-W1-GOAL-PROGRESS. Given an owner goal,
one final compatible interval, one missing interval and two separately
recorded possible contributors, when the owner reads progress, then show
the supported observation and distinguish each dated evidence item from
the corresponding expectation. Where 0094 supports a comparison, carry
its interval and limits. Where it does not, show no effect estimate.
Reuse the 0048 no-typed-actual, separate-source and abstaining-forecast
checks and UT-0094-PROOF for evidence admission. The goal changes no site data.

### Workflow, approvals and deadlines

1. An item kind may add its own states. One guard cannot be switched off:
   nothing reaches "verified" except by recheck.
2. Approval steps are optional and set by the admin. The agency
   edition's default: specialist writes, manager reviews, client approves,
   then a person does the work. Steps may be removed. They decide whether
   work should go ahead; the Fix list makes no site change at all.
3. Two limits no chain can remove: a moderator approves each link or
   member-text change, and the Coach stays advice only. Unattended
   irreversible changes are not allowed.
4. A person cannot approve their own item through a second account on one
   identity, or skip a step. Each attempt is refused and logged.
5. Deadlines: a target per state. The clock pauses in "waiting on the
   site's owner" and the reason is kept. A breach notifies the named
   owner. Escalation re-owns the same item; it never makes a second.
6. Freeze windows: dates when change should wait. Items show "frozen
   until".

### Site work acceptance contracts (539–550)

An item binds site, authorised check identity, target, kind,
current evidence revision and logical effect ID. Automatic
issue intake deduplicates by the owning check's item key.
A later occurrence updates or reopens that item under the return
rule. It cannot mark an item verified, assign access or apply a
fix. Show current evidence and audit-safe state history. Reject
wrong-site rechecks and stale action versions.

F-0539 uses REQ-0048-W2-WORK-ITEM and DES-0048-W2-RECHECK-STATE
for the existing local item lifecycle. FX-0048-W2-WORK-ITEM
and IT-0048-W2-WORK-ITEM create and assign one exact authorised
item and replay the same action. Try an older action version,
a foreign site's item and a claimed fix without a valid recheck.
Each unsupported transition refuses. Only a current recheck
of its named pages with the expected evidence may set verified.
The workflow cannot apply a native fix.

Native actions require QR-089 and QR-146 mappings for work
read, comment, mention, item approval, client flag and deadline.
Each maps exact WordPress or XenForo permission and native object
to a current 0101 Cedar target. Unmapped routes remain unavailable.
A comment binds exact report or item revision, actor and audience. A mention
must resolve to a current
eligible account in that same site and object scope. Recheck at
submit, notification and replay. Wrong or revoked scope shows
no comment, title or identity to the recipient. Client read-only
views show no comments. No mention adds a direct delivery route.

An approval chain stores the expected item and chain revisions,
ordered roles, subject digest, next position and expiry.
Commit one step and 0019 receipt in one conditional update on
item, chain, position, actor and grant versions. Recheck native
and Cedar roles. Decline, expiry or withdrawal cannot advance.
Deny self-approval across aliases or unproved identity equivalence.
Keep a safe refusal record. Concurrent acceptance has one winner.
Changed input or lost role invalidates queued decisions for that
effect. A Fix list decision grants no native publish authority.

An agency client view uses one site, one current site sign-in under
0101 and the exact flagged state, checked result and date.
Side-by-side counts are owner-local, read with a separate grant
for each site. A native session sees only its one site.
No portal, cross-client search, combined list, secrets or visibility from
a shared template. A client who can approve needs the separately
admitted action grant. Read-only view approval is never implied.

A deadline pins site, item, state-entry generation, assignee,
policy revision and entered instant. Calculate active elapsed
time excluding exactly the stored waiting-on-owner intervals.
A reassignment keeps accrued time and refreshes the approved
recipient. Commit state and a single 0019/0099 breach key and
outbox together. Recheck recipient and policy after quiet hours
or restart. Escalation updates the same item. Keep unknown
delivery safe without a duplicate effect.

A freeze binds action scope, current policy generation, half-open
UTC interval and original zone. Recheck at display, review and
before any queued native action in the owning gate. A running
0137 band can advise waiting only when confirmed and current.
It cannot create a freeze or cancel a completed effect. Apply
current native recovery authority under 0033 even during a freeze.
No freeze grants a site effect.

Playbooks are versioned local checklists with site-specific
source evidence, expiry and current grants. Preview and clone an
approved migration, launch or recovery checklist. Previewing or
marking a step does not execute that operation. Impact order
stays within the existing four calls and evidence kinds. Display
range, unit, named limits and weakest evidence. Effort combines
approved task estimates only in compatible units and separates
time, money and uncertainty. Missing estimates stay unknown.

#### Wave 2 Fix list and local workflow bindings

Use synthetic native roles, item revisions, grants, clocks and
receipts. QR-089 and QR-146 still govern native mappings and
scope. Existing setting and evidence admissions govern limits.
F-0541, F-0549 and F-0550 share the governing workflow or backlog
checks. Their fixtures identify separate observations without
creating separate actions, roles or ranking measures.

| Feature | Owning requirement and design | Planned fixture and expected evidence |
|---|---|---|
| F-0540 | REQ-0048-W2-AUTOMATIC-ISSUE-INTAKE · DES-0048-W2-LOGICAL-FIX-ITEM | FX-0048-W2-AUTOMATIC-ISSUE-INTAKE · IT-0048-W2-AUTOMATIC-ISSUE-INTAKE. Reuse the repeat-finding and return-after-verification checks. Replay one eligible broken-link finding by exact site, check and cause. Update its evidence while open, then compare a genuine recurrence after an admitted verified recheck with an unread page. Only the supported occurrence updates or reopens the same item once. Preserve safe proof and flood bounds. No automatic verification, assignment grant or repair. |
| F-0541 | REQ-0048-W2-SCOPED-COMMENTS · DES-0048-W2-LOCAL-COMMENT-AUDIENCE | FX-0048-W2-SCOPED-COMMENTS · IT-0048-W2-SCOPED-COMMENTS. Use the existing item or report comment and mention route. Submit against the exact revision and mention one eligible account, one foreign account and one revoked account. Recheck at submit, notification and replay. A read-only client sees no comment. Refused recipients receive no text, title or identity. No direct delivery route. |
| F-0542 | REQ-0048-W2-APPROVAL-CHAIN · DES-0048-W2-CONDITIONAL-APPROVAL-STEP | FX-0048-W2-APPROVAL-CHAIN · IT-0048-W2-APPROVAL-CHAIN. Race two synthetic approvers at the same next position. Reuse the self-approval and moderator-boundary checks. Bind exact subject, item and chain revisions, actor, native and Cedar grants and expiry. Keep one current decision and receipt. An expired or revoked grant, stale replay or alias with unproved equivalence cannot advance. Native approval and publication keep their own gates. |
| F-0544 | REQ-0048-W2-NATIVE-CLIENT-WORKSPACE · DES-0048-W2-SEPARATE-CLIENT-SCOPES | FX-0048-W2-NATIVE-CLIENT-WORKSPACE · IT-0048-W2-NATIVE-CLIENT-WORKSPACE. Use two separately granted owner-local sites and one client's native session under DR-080. Compare a flagged item and an unflagged item. The client sees only the flagged item's safe state, checked result and date for its one site. No comments, internal labels, combined search or remote portal. A flag gives no approval. |
| F-0545 | REQ-0048-W2-DEADLINE-CLOCK · DES-0048-W2-DURABLE-BREACH-IDENTITY | FX-0048-W2-DEADLINE-CLOCK · IT-0048-W2-DEADLINE-CLOCK. With a synthetic admitted service-level profile, enter and replay a waiting-on-owner interval, resume, then race reassignment with the breach notice. Subtract that interval once and keep accrued time on reassignment. Recheck policy and recipient after restart or quiet hours. One breach identity and outbox outcome under 0019/0099. A missing target profile or revoked recipient holds its effect. |
| F-0546 | REQ-0048-W2-LOCAL-PLAYBOOK-CLONE · DES-0048-W2-INERT-CHECKLIST-VERSION | FX-0048-W2-LOCAL-PLAYBOOK-CLONE · BDD-0048-W2-LOCAL-PLAYBOOK-CLONE. Preview and clone exact current approved migration, launch and recovery checklists in the authorised site. Expire a source and try a foreign template. Refuse unsupported inputs. Marking a step updates only the eligible local checklist and executes no checklist operation. |
| F-0548 | REQ-0048-W2-SCOPED-FREEZE · DES-0048-W2-HALF-OPEN-FREEZE-FENCE | FX-0048-W2-SCOPED-FREEZE · IT-0048-W2-SCOPED-FREEZE. Use a scoped half-open UTC interval and pinned original zone across a clock shift. Check the start, end and a policy or native grant change between approval and a queued effect. Recheck the current gate and show the deciding reason. An official update band may advise waiting only. Preserve separately governed 0033 recovery and never authorise site work through the freeze. |
| F-0549 | REQ-0048-W2-EXPLAINED-BACKLOG-ORDER · DES-0048-W2-EXISTING-PRIORITY-KINDS | FX-0048-W2-EXPLAINED-BACKLOG-ORDER · IT-0048-W2-EXPLAINED-BACKLOG-ORDER. Use the existing “What to do first” decision with compatible items of one kind, sparse evidence, unlike units and an allowed owner override. State the call, deciding reasons, weakest input, limits and revisit trigger. Order only within the existing kind and supported units. Weak evidence cannot Fund. Keep an override labelled and apply no action. |
| F-0550 | REQ-0048-W2-SOURCED-EFFORT · DES-0048-W2-EXISTING-PRIORITY-KINDS | FX-0048-W2-SOURCED-EFFORT · IT-0048-W2-EXPLAINED-BACKLOG-ORDER. Reuse the same backlog case and vary current evidence. Compare compatible approved time ranges, an unsupported cost and one changed estimate. Keep source, units, range and uncertainty. Separate time from money and withhold missing effort. No invented conversion, effort or priority score. |

### Views and filters

1. List (default order), board by state, timeline for waits, calendar
   (due dates, deadlines, freeze windows, recheck dates), "my items" for
   the signed-in person on the current site, and workload against
   declared capacity with an over-allocation flag.
2. A board drag that a guard blocks is refused with the reason shown.
3. Saved views are per admin.
4. Sprints: time-boxed scope and close. Velocity counts verified items
   only; "claimed fixed" is a second, labelled line.
5. One filter form, defined once in the action layer (spec 0020), serves
   the app, `sutra`, MCP, saved views and rules, and gives the same items
   in each. No second query language.

### Rules

1. A rule reads: when X, then Y, if Z. X is a finding appearing, an item
   coming back, a recheck finishing, a deadline passing, a goal changing
   status, or a schedule. Admins make rules; staff cannot.
2. Y may only be: create, assign or label an item; add a comment; send a
   staff notice; start a recheck; set "frozen until"; ask a person. Y may
   never change the site, place or remove a link, change member text,
   change a setting, publish or call outside. Saving a rule that asks for
   more is refused.
3. Rules are entries run on the durable jobs system (spec 0019). The
   when and if parts are declarative; a then step may be a Python user
   script (feature 467). A script can call only the actions in item 2;
   the runner refuses any other call while the script runs. Rules use
   the jobs system's waits, limits and load governor. No second
   scheduler. A rule cannot be saved without its caps: runs an hour and
   actions a run.
4. The first save is a dry run that shows what the rule would do and
   does nothing. Rules have versions with differences shown. A run in
   flight finishes on the version it started with.
5. Run history keeps, per step: inputs, outputs, time, error and retries.
6. Errors are typed and routable: "page not found", "shed under load",
   "denied by permission", "check could not run". A shed rule is never
   reported as a failed page.
7. A visual canvas that draws the same rules comes after the typed rule
   list has run for a while. Anything it draws, a rule can say.
8. Rule sets can be saved as templates.

### Page checklists

1. Page kinds come from the platform: thread, forum, resource, hub,
   guide, shop product, and WordPress post types (spec 0022). An admin
   may mark a WordPress page as landing or service. A person's correction
   of a kind survives later crawls. No learned classifier at first.
2. Each kind has a checklist. Every check ships with its proposal text
   and a recheck, or the gate fails.

| Family | Compared with | Notes |
| --- | --- | --- |
| Title, description | Own pages' click rate, same position | Titles never changed |
| Headings | This site's own best pages of the kind | No rival fetched |
| Structured data | Completeness for the kind | JSON-LD as text only |
| Internal links | The site's own cluster norms (spec 0049) | Via suggestions |
| Competing own pages | Coach cannibalisation radar | Run first |
| Speed | Real-user vitals by template (darshak counts) | One per template |
| Conversion | Landing, service and shop pages | Labelled apart |

3. A title finding appears only where the Coach's narrow title exception
   allows it. Click-rate curves come from the site's own Search Console
   data, never a published curve. Schema is never proposed because a
   rival uses it. An internal-link fix goes through staff suggestions and
   a moderator.
4. Conversion checks are labelled "conversion factor", counted apart
   from ranking checks, and no text says they affect ranking.
5. Weights are visible and an admin may change them; the shipped set
   resets in one step and every change is in the settings history. A
   page's result is recomputed from the crawl, cannot be typed, and shows
   its parts and spread.
6. Measured impact: when verified items have before-and-after results
   over enough days, check types whose effect fades are lowered in the
   order, never removed. Shipped weights change only by the ganit route:
   fit, shadow test, owner approval, one-step rollback. "Marked fixed,
   not checked" items are left out.

### Change tracking

1. Two kinds of record, never merged. "Sutradhar made it": who approved,
   which proposal, when, with undo, from the audit trail and link
   journal. "Seen on the site": found by a crawl, shown as "changed
   outside, actor unknown", with an interval from the previous crawl to
   this one. The day it was seen is never shown as the day it happened.
2. A before-and-after result on an outside change says "associated
   with", never "caused", and names what is missing. A change inside a
   running test's window marks that test "muddied".
3. A fix that disappears is an outside change with a known earlier
   state, handled by the return rule.

### Agency edition

1. One client is one site with its own store. No item, template,
   mapping, rule, goal, saved view or outside connection crosses
   clients.
2. An agency screen shows each client's counts side by side. It never
   merges them, ranks clients together or searches across them.
3. A "client may see" flag, off for every item by default. A client sees
   only flagged items, read only: state, checked result and date. Never
   comments, owners or internal labels. A client approval step needs a
   client login.
4. Metering and scheduled reports use their own designs. Client
   access follows DR-080 and 0101 native sign-in. Existing own-site
   alert rules apply. Rival alerts stay outside the agency work list.

### What to do first

Of everything on the Fix list, what do I do first, look into before
acting, leave alone for now, and what can wait? One list per site with
four calls. It decides and never acts. Screen label: "What to do first".

Example: of 4,000 findings marked important: do these 11 this cycle;
look into these 3 first, because there is too little data to know if the
pages lost clicks; freeze these 6, because a search update is running;
defer the rest to next cycle. Each line gives its reasons and what would
change the call.

| Call | Means |
| --- | --- |
| Fund | Worth doing this cycle: enough evidence, safe, and it fits |
| Look into first | Too uncertain; the next step is named evidence |
| Freeze | Evidence weak or contradicted, or something blocks action |
| Defer | Small gain next to the rest; looked at again each cycle |

1. Freeze reasons are real constraints: a paused crawl, a block page, a
   freeze window, a confirmed running update from 0137 through 0043,
   or a moderator approval still awaited.
2. Calls cover open and came-back items. Items someone has taken are
   left alone.
3. Reasons, never one score. Each call shows its deciding reasons and the
   limits beside it: time, crawl capacity, freezes, waits. A ranked list
   with no limits shown is a defect.
4. Every call carries a revisit trigger, names its weakest input and what
   evidence would change it.
5. Weak evidence can only be Look into first or Freeze, never Fund.
   Missing evidence means Look into first, never a guess or zero. A gap
   is shown in its source's own unit; units and sources are never added.
6. Its code has no way to fetch, write, send or spend; a gate walk of
   its dependencies proves it. A person turns a call into work.
7. A person may fund against the call. The reason is recorded and the row
   is labelled "override". Overrides tune the rules only by the ganit
   route. Later before-and-after tests judge the allocator.
8. Ranking happens inside a kind only:

| Kind | Evidence | Ranked by |
| --- | --- | --- |
| Measured gap | Search data shows a gap on enough sample | Gap size, range |
| Measured risk | Crawl or platform shows risk, no traffic value | Severity |
| Structure only | Real scope, no outcome value | Pages affected |
| Needs evidence | Which unknown to settle first | Information gain per cost |

9. Each kind has a share of the cycle's capacity (hours the admin
   declares, and crawl time within the crawler's limit). "Needs evidence"
   has a floor an admin may raise but never remove. An empty kind's share
   goes to the others.
10. It uses ganit's budget-split, weekly-action and value-of-information
    jobs (5, 28 and 31) and the freeze windows and waits above.

### Templates and outside task tools

1. Templates move between installs by file only. A file holding a site's
   address or name is refused, and nothing goes online. Project-vetted
   templates ship in updates. An imported template is inert: a new
   version the admin chooses to run, never changing defaults or
   re-scoring sealed audits. No performance claim without a measured
   record, or "no measured record yet".
2. Outside task tools (such as Jira or Asana): one way first, one
   connection per site with the admin's own key (kept like connector
   keys), with pause and revoke at once. Two-way sync comes later. The
   outside "finished" mark is only a claim and never closes an item. The recheck
   kind, gap size, links and verified state never come back in.

### Links to other parts

1. The signal registry (spec 0044) gives each signal a `recheck` entry.
2. Check my fix may start from an item.
3. The Editorial Coach (spec 0041) gains the "add to Fix list" click.
4. The action layer (spec 0020) gains actions and MCP tools; none sets
   verified.
5. Jobs (spec 0019) gain recheck, return-check and rule-run kinds. Audit
   (spec 0032) gains events for item changes; its safe CSV test is
   reused.
6. Settings (spec 0014) gain the `work.*` keys.

### Settings

| Setting | Default | Allowed range |
| --- | --- | --- |
| `work.enabled` | on | on or off |
| `work.verify.clean_crawls_to_close` | blocked: no admitted value | 1 to a safe top |
| `work.recheck.max_pages` | blocked: no admitted value | above zero, safe top |
| `work.flood.items_per_run` | blocked: no admitted value | above zero, safe top |
| `work.return.flag_after` | blocked: no admitted value | 2 to a safe top |
| `work.wontfix.review_days` | blocked: no admitted value | safe bottom and top |
| `work.memory_share` | blocked: no admitted value | fixed share of the service slice |
| `work.max_depth` | blocked: no admitted value | 1 to a safe top |
| `work.import.max_rows` | blocked: no admitted value | above zero, safe top |
| `work.import.max_cell_chars` | blocked: no admitted value | above zero, safe top |
| `work.audit.keep_runs` | blocked: no admitted value | safe bottom and top |
| `work.goal.min_days_history` | blocked: no admitted value | safe bottom and top |
| `work.sla.target_days` | blocked: no admitted value | per state, safe bottom, top |
| `work.workload.capacity_hours` | blocked: no admitted value | above zero, per person |
| `work.rule.runs_per_hour` | blocked: no admitted value | above zero, safe top |
| `work.rule.actions_per_run` | blocked: no admitted value | above zero, safe top |
| `work.rule.versions_kept` | blocked: no admitted value | 1 to a safe top |
| `work.rule.history_days` | blocked: no admitted value | safe bottom and top |
| `work.checklist.weight.<check>` | blocked: no admitted value | above zero, safe top |
| `work.impact.min_days` | blocked: no admitted value | safe bottom and top |
| `work.impact.min_pages` | blocked: no admitted value | above zero, safe top |
| `work.change.window_max_days` | blocked: no admitted value | safe bottom and top |
| `work.next.cycle_days` | blocked: no admitted value | safe bottom and top |
| `work.next.share.<kind>` | blocked: no admitted value | above zero; all sum to 1 |
| `work.next.look_floor` | blocked: no admitted value | above zero, safe top |
| `work.next.min_sample` | blocked: no admitted value | above zero, safe top |
| `work.next.capacity_hours` | blocked: no admitted value | above zero, per person |
| `work.template.max_bytes` | blocked: no admitted value | above zero, safe top |

Every value marked "blocked" is a readiness blocker owned by the relevant
fix-list Ganit Vikas research record below. The value, safe bounds, fixtures,
supporting evidence and approval must be admitted before the dependent rule,
forecast, import, scheduling or retention behaviour can build or run.

### Build-blocking research (Ganit Vikas)

1. Which recheck rule gives no false "verified" or "came back"? Tested on
   made-up sites: fixed, half fixed, fixed then broken, cached, flaky
   host, with answers fixed first. Also clean crawls before closed,
   returns before the flag, one item per cause or per page, and which
   data findings can be rechecked.
2. What history makes a goal forecast worth showing, and when must it
   abstain? Does a single audit score ever change a decision? If not, it
   never ships. How is effort estimated honestly?
3. Can a fading check be told from noise without false demotions? Do
   platform page kinds beat a learned one? What window keeps "associated
   with" honest? Do weights beat the plain order in a blind editor test?
   Safe rule caps; the least history for a click-rate curve.
4. What evidence is strong enough for Fund? What shares and floor never
   starve evidence? Does information gain pick a good next step? Does the
   list beat plain order in a blind editor test?

### Not part of it

Unattended irreversible changes; rival intelligence and alerts, or
fetching "pages outranking it"; cross-site portfolios (spec 0102), brand and
market
optimisation, or one headline number; money values, order values and
spend ceilings; standing delegations and signed tokens (nothing here
acts); user-written code steps; an ontology or its query language.

## Limits

- Runs inside `work.memory_share` of the service slice, and
  stops first under pressure.
- Rechecks read at most `work.recheck.max_pages` pages, within the
  crawler's rate limit and slowing rule.
- At most `work.flood.items_per_run` new items per run.
- Imports are capped by size, `work.import.max_rows` and
  `work.import.max_cell_chars`; template files by
  `work.template.max_bytes`.
- Rules are capped by `work.rule.runs_per_hour` and
  `work.rule.actions_per_run`.

## Footprint budget

This part adds nothing to public pages. Rechecks are crawler reads under
the crawler's limits (spec 0044).

### Portfolio summary read

Return a count of open work for each severity in the Fix list. Use
the four open-work states above. Count once by the site's fix item
identity, including one reopened item. A severity or state change
moves the count in the same transaction as the item.

Expose one bounded, site-scoped summary read under spec 0102. Its
source and failure cases run in IT-SS-06 with the owning module.

## How it is tested

Rust tests on made-up sites with known answers, Angular screen tests, and
property tests where noted:

- A repeat finding updates its open item; no second item is made.
- Only the recheck sets verified, came back and page gone; a person, an
  MCP agent and an outside call each get a logged refusal.
- F-0190 rechecks the exact named pages. A fresh eligible 404 or 410
  for one named page marks that page gone. A 404 for a different
  redirect destination and a denied or timed-out attempt prove no
  named page gone. Two proved gone pages yield the item state
  "page gone". One gone and one right stay "fix claimed" and show
  "1 of 2 right" with the gone page. One gone and one unread also
  stay "fix claimed" with both reasons. Only two right can verify.
  Reconcile outcomes under the exact claim and grant after replay.
- 211 of 212 right stays "fix claimed" and names the page left.
- A return after verified reopens the item, counts once and keeps proof.
- A recheck reads only named pages, in guest view, within the rate limit,
  and changes nothing.
- "Marked fixed, not checked" needs a reason, is flagged and counts as
  no fix.
- A signal with no `recheck` entry fails the gate.
- A crashed check files a fault in autoissues only; the Fix list shows
  the code; no site finding reaches autoissues or Forgejo.
- One site's items never appear for another site or client.
- Past the cap the screen says how many wait; none is lost.
- An item holds no post text or member name; history is append-only.
- Staff without permission see nothing; the report-screen walk finds no
  apply, insert or title-change control; it stops first under pressure
  and stays in its memory share.
- An import row naming a page outside the chosen site is refused with a
  reason; a partial import shows counts, lists failures and records the
  admin's choice; no row vanishes.
- An exported cell starting `=`, `+`, `-` or `@` opens as text.
- A sealed audit never changes; a diff re-runs nothing.
- A wait loop of any length is refused with the path named.
- No goal actual can be typed; sources are never added; an abstaining
  forecast shows "unforecastable" and its reason.
- No approval chain lets a link or member-text change skip a moderator;
  a self-approval attempt is refused and logged.
- Velocity and every shown number is in `metrics.toml`, or not shown.
- The same filter gives the same items in the app, `sutra` and MCP.
- A rule with no caps cannot be saved; a looping or fanning-out rule is
  shed or stopped and screens stay responsive.
- A rule asking for a site change, link, member text, setting or outside
  call is refused at save.
- A first save does nothing live; a run in flight keeps its version.
- "Not found" and "shed under load" take different error paths.
- A check with no proposal or no recheck fails the gate.
- A corrected page kind survives the next crawl.
- A result cannot be typed; it shows parts and spread; conversion checks
  are labelled and counted apart.
- No checklist fetches a rival page or proposes schema for a rival.
- A lowered check still runs; shipped weights change only by the ganit
  route.
- The two change records stay apart; an outside change shows its
  interval and its result says "associated with".
- A change inside a test window marks the test "muddied".
- The client flag is off for every item; a client sees only flagged
  items, read only.
- No screen merges, ranks or searches across clients.
- Property test: weak evidence is never Fund.
- Missing evidence gives Look into first; no zero, guess or money value.
- Every call has reasons, limits and a revisit trigger; a list with no
  limits shown fails a screen test.
- A gate walk proves the decision code reaches no network, database or
  file adapter, refusing a planted three-step dependency fault; no call
  changes the site.
- Ranking is inside a kind only; the evidence floor cannot be removed.
- An override keeps its reason and shows on the list.
- One site's calls use only that site's data.
- A template file with an address or name is refused; an imported
  template changes no default and re-scores no sealed audit.
- An outside connection serves one site; the outside "finished" mark
  never closes an item; revoke stops all traffic at once.
- The import parser is fuzzed.

## Changes from the source

- States, permissions, verdicts, calls and order are decided in Rust,
  with Kani proofs where they fit.
- Forecasts, fits and effects run in ganit, the Rust numerical engine.
- Screens, heavy visuals and the rule canvas are a desktop app built from
  SDS components, not browser-side code.
- Owner-written rule steps are Python user scripts (feature 467), limited
  to the actions in Rules item 2.
- XLSX import and export wait for a Rust library pinned as AGENTS.md
  section 4 sets out.
- Faults in Sutradhar itself go to autoissues (spec 0063); Forgejo
  stays the project's own task list. A fault becomes a Forgejo task
  only when an agent takes it on, linked to it (the owner, 10 November
  2026: "Keep apart, link when taken").

Source: `docs/specifications/signals/work-graph.md`, `work-graph-2.md`,
`work-graph-3.md` and `work-graph-4.md` (all in
`docs/specifications/signals/`) in the archive
`archive-2026-11-08-old-repo`.

## Audit suite intake

[Spec 0143](0143-audit-grid-findings-and-local-preview.md) reads
and proposes work through this Fix List. Use the existing
site, check and cause key and exact recheck authority. An unknown
or incomparable check cannot close a finding. Category views
present individual severity, impact and sourced effort within a
compatible kind. The suite adds no combined score or state write
on opening a view.
