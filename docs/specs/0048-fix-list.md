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
4. A 404, a 410 or a page that left guest view is "page gone". With no
   page left, the item is "page gone", not "verified".
5. Search-data rechecks say "not enough evidence" below a researched
   floor.
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
   when and if parts are declarative; a then step may be a Rhai user
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
4. Metering, the client portal and scheduled reports belong to the
   agency edition's own design. Rival data comes only from connected
   services. Own-site alerts still become items; competitor alerts are
   dropped.

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
   freeze window, a running search update (the outside notices of spec
   0045), or a moderator approval still awaited.
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
| `work.verify.clean_crawls_to_close` | researched | 1 to a safe top |
| `work.recheck.max_pages` | researched | above zero, safe top |
| `work.flood.items_per_run` | researched | above zero, safe top |
| `work.return.flag_after` | researched | 2 to a safe top |
| `work.wontfix.review_days` | researched | safe bottom and top |
| `work.memory_share` | researched | fixed share of the service slice |
| `work.max_depth` | researched | 1 to a safe top |
| `work.import.max_rows` | researched | above zero, safe top |
| `work.import.max_cell_chars` | researched | above zero, safe top |
| `work.audit.keep_runs` | researched | safe bottom and top |
| `work.goal.min_days_history` | researched | safe bottom and top |
| `work.sla.target_days` | researched | per state, safe bottom, top |
| `work.workload.capacity_hours` | researched | above zero, per person |
| `work.rule.runs_per_hour` | researched | above zero, safe top |
| `work.rule.actions_per_run` | researched | above zero, safe top |
| `work.rule.versions_kept` | researched | 1 to a safe top |
| `work.rule.history_days` | researched | safe bottom and top |
| `work.checklist.weight.<check>` | researched | above zero, safe top |
| `work.impact.min_days` | researched | safe bottom and top |
| `work.impact.min_pages` | researched | above zero, safe top |
| `work.change.window_max_days` | researched | safe bottom and top |
| `work.next.cycle_days` | researched | safe bottom and top |
| `work.next.share.<kind>` | researched | above zero; all sum to 1 |
| `work.next.look_floor` | researched | above zero, safe top |
| `work.next.min_sample` | researched | above zero, safe top |
| `work.next.capacity_hours` | researched | above zero, per person |
| `work.template.max_bytes` | researched | above zero, safe top |

"Researched" values are fixed by GanitEvolve research before the build.

### Research first (GanitEvolve)

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
fetching "pages outranking it"; cross-site portfolios, brand and market
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

## How it is tested

Rust tests on made-up sites with known answers, Dioxus screen tests, and
property tests where noted:

- A repeat finding updates its open item; no second item is made.
- Only the recheck sets verified, came back and page gone; a person, an
  MCP agent and an outside call each get a logged refusal.
- An unreadable page verifies and reopens nothing ("recheck due"); a 404,
  a 410 or a page out of guest view gives "page gone".
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
- Screens, heavy visuals and the rule canvas are a Dioxus app built from
  SDS components, not browser-side code.
- Owner-written rule steps are Rhai user scripts (feature 467), limited
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
