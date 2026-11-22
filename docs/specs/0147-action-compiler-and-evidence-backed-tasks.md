# 0147: Action Compiler and evidence-backed tasks

Every finding Sutradhar makes ends as a plain-English task with proof, a
way to do it and a way to check it worked. One Action Compiler in the
Rust core sits between every analysis part and the screens. It turns
typed observations into one card per root cause, attaches what Search
Console, GA4 and darshak say about the pages involved, and hands the card
to the Fix list (spec 0048). Nothing here edits, publishes or touches
live ranking.

Spec 0048 owns the Fix list: items, states, rechecks, the return rule,
"What to do first", templates and outside task tools. This spec adds the
front door and the proof: the observation contract for all ten angles,
root-cause roll-up, the proof receipt, Focus 3, cooldown, the rollout and
data holds, and the 28-day outcome receipt. Where 0048 already decides
something, this spec cites it. It extends 0048 without changing its rules,
except four small additions listed under "Additions to 0048".

Example: Ravi's forum has Search Console and darshak but no GA4. The audit
finds a missing meta description on 400 thread pages that share one theme
template. One card opens: "Fix the shared theme header that hides the
description on 400 pages". Its receipt reads "Search Console: 15,000
impressions, click rate (CTR) 1.9% where your site normally gets 3.4% at this
position. darshak: 2,100 visits, readers stop at block 2. GA4: not connected,
so conversions are not shown." It sits in "Focus 3". After Ravi taps "I
finished this", the crawler re-reads the pages: "400 of 400 right", so the
card is Verified. 32 days later it shows "CTR 1.9% to 3.1%: helped (observed,
not proven)". All figures are illustrative.

## Words used here

These words have one meaning in this spec.

| Word | Meaning |
|---|---|
| Observation event | One typed finding sent by an analysis part, with ids and numbers only |
| Root cause | The shared thing behind many findings: a template, asset, rule or setting |
| Cluster | All observations with one root cause, kept as one record with its page list |
| Action card | The plain-English task a person sees; one Fix list item (spec 0048) |
| Proof receipt | The card's evidence from Search Console, GA4 and darshak, per source |
| Focus 3 | The daily view of the three cards to do first |
| Hold | A reason a card waits: update rollout, or data that cannot be trusted |
| Cooldown | A quiet window after a verified fix, with no repeat suggestion |
| Outcome receipt | The observed change over 28 days, starting 4 days after the edit |
| Measured gap, measured risk, structure only | The three kinds spec 0048 ranks inside: a gap shown by search data, a fault with no traffic value, and real scope with no outcome value |
| Compare-and-set | Saving a change only if the record has not changed since it was read |
| Bootstrap | Re-drawing the data many times to see how much a result could vary |
| Sanity status | Healthy, Caution, Broken or Unchecked, from spec 0146 FC-32 |
| Source | One of Search Console (GSC), GA4 or darshak (the local collector) |
| FC-nn | A numbered check in spec 0146, such as FC-32 |
| Heavy-lane job | Big background work run through the scheduler under the memory budget |
| Look-alike pages | Unchanged pages of the same category and similar level, used as a comparison |
| Outcome interval | The range the observed change likely falls in, after allowing for many cards |

## What this spec designs

- One observation contract (`ObservationEvent`) and the catalogue of ten
  angles that must use it, each mapped to its owner spec and recheck.
- Roll-up by root cause, the proof receipt and the partial-source rules.
- How cards are ordered, the Focus 3 view and the task board views.
- Holds, cooldown, live verification and the 28-day outcome receipt.
- Local MCP tools, fenced Python hooks, exports and failure codes.

Non-goals, all fixed:

- No live ranking input, no ranking research, no site write, no auto
  edit, no auto link and no auto publish. Every change is a person's.
- No promise of results. An outcome says "observed", never "caused".
- No new connector and no model call. Syncs belong to spec 0040 and need
  owner approval; this spec only reads stored copies.
- No combined money score and no summing of sources (DR-164).
- No outside Notion connector. DR-076 builds only Slack, Teams, email,
  Jira, Linear and Asana. A clean-room workspace is inside version 1
  (DR-165) but is a separate series and is not designed here.
- No visitor profile, raw address, member name or private forum identity.

## Additions to 0048

This spec asks spec 0048 to take four additions, recorded in DR-140: a
"data cannot be trusted" Freeze reason; a limit of the running-update
Freeze reason to reactive cards, with a 7-day settling tail; a
`snoozed_until` date on a Defer call; and an exemption from "weak
evidence is never Fund" for the Fix-tracking card only. It also adds
Markdown and JSON to 0048's CSV export. It also asks 0044 for a `card`, a
`reactive` and a `cooldown_class` field per signal, and 0048 for the
`work.*` keys below. The `ObservationEvent` contract and the three 0146
events are to be registered with spec 0019 and named by each emitter
(TASK-0019-OBSERVATION-EVENTS in the implementation catalogue).

## Who owns what

This table shows which spec owns a topic and what 0147 adds.

| Topic | Owner | What 0147 adds |
|---|---|---|
| Items, states, owners, comments, labels, groups, waits | 0048 | Nothing. Cards are 0048 items with extra fields |
| Recheck, return rule, "marked fixed, not checked" | 0048 | The verification run record and the offline rule |
| "What to do first": Fund, Look into first, Freeze, Defer | 0048 | The data-hold Freeze reason, the narrowed rollout reason and the Focus 3 pick |
| Findings and the signal registry | 0044 | A `card` field per signal, checked by the gate |
| Crawl, logs, Toolbox, Performance Lab, audit grid | 0141, 0142, 0143, 0140, 0145 | Emit observation events |
| Text mining, Coach tips, fan-out gaps | 0144, 0041, 0042, 0117 | Emit observation events |
| Update windows and settled comparisons | 0137, 0138 | Rollout hold; settled shifts become cards |
| Backlink receipts | 0045, 0105 | Emit observation events |
| AI fetches, AI visibility | 0135, 0093 | Emit observation events |
| Forecasts, decay, cannibalisation, sanity status, lift | 0146 | Consumed as evidence; sanity gate reused |
| DataForSEO lookups | 0132, 0133 | Optional evidence, owner-approved only |
| Search Console and GA4 copies | 0040 | Read through its public API |
| darshak aggregates and privacy | 0034 | Read through its public API |
| Python scripts | 0104 | Hooks below |
| MCP tools, approvals | 0020 | Tool names below |
| Outside task tools (Jira, Linear, Asana) | 0048, 0100 | Export templates only; no Notion template (spec 0100 F-0495 refuses any Notion intent) |
| Memory, jobs, file ledger | 0078, 0019 and 0099, 0030 | Compile and verify are heavy-lane jobs |

## Architecture

```
 emitters (0044 0141 0142 0143 0144 0041 0137 0045 0135 0146 ...)
        |  ObservationEvent via typed outbox events (0019)
        v
   validate --> key (site, check, cause) --> cluster by root cause
        |                                        |
        |                                        v
        |                evidence joiner: 0146 results, 0040 and 0034 reads
        |                                        |
        |                         sanity gate (0146 FC-32 status)
        |                                        |
        '--------> action card = 0048 item <-----'
                           |
        .------------------+-------------------.
        v                  v                    v
   What to do first    Focus 3 and board    MCP tools, exports,
   (0048)              (Alekh, Tauri)       Python snapshots
```

The compiler is the intake stage of the `work_graph` module (spec 0048);
it adds no new module wall. It reads other modules only through their
public APIs, writes records and outbox events in one transaction, and its
consumers are safe to repeat (AGENTS.md section 5). Its code has no way
to fetch, write a site, send or spend; a gate walk of its dependencies
proves it. The crawler fetches for a recheck (spec 0044,
"check my fix") on a person's click.

This table lists the events the compiler sends and receives.

| Event | From, to |
|---|---|
| `ObservationEmitted` | Any analysis part to the compiler, carrying one `ObservationEvent` |
| `CardOpened`, `CardUpdated` | Compiler to the screens and MCP |
| `HoldChanged` | The compiler's hold stage to the screens: a card moved into or out of a hold |
| `VerificationFinished` | Crawler to the compiler; the item's rule decides (0048) |
| `OutcomeReady` | Outcome job to the screens and the ganit accuracy tracker |

Health and audit: the module reports queue depth, the last event time per
emitter and the sanity state to the health page, and writes one audit
line per card change, approval request, recheck request and script
change (spec 0032).

## The ten angles

Every analysis part emits `ObservationEvent` for these angles. The signal
registry (spec 0044) names, for each signal, its `recheck` kind (0048) and
now its `card` rule: a rule id, or `evidence_only` with a reason. A signal
with neither fails the gate, so no finding can sit in a dead report.
`evidence_only` facts appear only inside a card's receipt, never as a
list of their own.

This table lists each angle, who emits it, how its root cause is keyed,
what evidence it needs and how its fix is proved.

| Angle | Emitters | Root-cause key | Evidence fields | Recheck (0048 kind) | Example headline |
|---|---|---|---|---|---|
| A1 Technical and crawlability | 0044, 0141, 0082 Leviathan, 0140 | Template, rule file line, redirect rule, shared link element | GSC impressions and clicks on the affected URLs; bot hits from logs (0141); GA4 and darshak landing visits | Page facts | "Fix 1 redirect chain on your top 8 guide pages" |
| A2 Speed and mobile | 0142, 0143 | Template, shared asset hash, script | darshak phone and desktop quick-bounce; GA4 mobile gap; GSC mobile clicks | Page facts | "Shrink the 3.4 MB hero image on the product template" |
| A3 Titles, meta, markup | 0044, 0145, 0143 | Template field, shared markup block | GSC CTR against expected CTR at that position (0146 FC-04, FC-19); GA4 engagement after the click | Page facts | "Rewrite the title on 4 guides with half your normal click rate" |
| A4 Content quality | 0041, 0042, 0117, 0144 | Page and block; shared template block | GSC impressions on unaddressed queries (FC-17); darshak drop-off block (FC-09); short bounces (FC-13) | Page facts, then Search data | "Answer the top 3 missing questions under Heading 2" |
| A5 New content and forecasting | 0146, 0132, 0133 | Topic cluster | Rising queries (FC-02), seasonal peak (FC-01), site-search misses (FC-11), forum goldmine (FC-31); GA4 conversion on related topics | None until published; then Search data | "Publish a guide on this topic before week 14" |
| A6 Decay, cannibalisation, pruning | 0146, 0044 | Topic cluster, query pair | Slide (FC-03), tug-of-war (FC-21), prune audit (FC-26) with assist safety (FC-30) | Search data; Page facts for merge and redirect | "Merge page B into page A and redirect it" |
| A7 Internal linking | 0023, 0029, 0119, 0146 | Hub or orphan set | darshak anchor click rates (FC-12); next-page paths (FC-25); striking distance (FC-18) | Placed link | "Add 4 reviewed links from your hub posts into this guide" |
| A8 Google update shifts | 0137, 0138 | Update, surface, trait gap | Settled before and after Search Console shift by surface; trait gaps against survivors | Search data | "Refresh these 6 guides hit by the latest core update" |
| A9 AI search readiness | 0093, 0135, 0117, 0144 | Page and block; bot rule | Informational clicks; darshak copy hotspots (FC-10); optional DataForSEO AI answers (owner-approved) | Page facts | "Make the opening sentences under each heading stand alone" |
| A10 Backlinks and site health | 0045, 0105, 0146 | Target URL, tracking tag | Backlink receipts pointing at 404s; 404 hits in darshak; GA4 and GSC gap flags; FC-32 tracking status | Page facts; Site setting for tags; Search data (FC-32 status) for tracking cards | "Redirect 5 dead URLs that still have live backlinks" |

DataForSEO inputs (A5, A9) stay owner-approved, one exact lookup at a
time (DR-117, DR-118) and are never fetched by the compiler. Spec 0142
and 0143 fields that name a provider keep their own owner-only switches.

## Data

Every limit and threshold in this spec (200 page ids, 32 facts, 5-page
clusters, counts of 5, 4 and 28 days, 32 days, 8 outcomes, 120 characters, 100
rows, 60 calls, 64 MiB, 20 minutes, 10,000 rows) is proposed until measured on
the Dell, as are the caps named below (5,000 rows a batch, 100,000 members,
500,000 clusters, 5,000 custom events a day, 10%, 35 days, 200 outcome jobs,
1,000 resamples, 500 matched pages, 20 minutes, 3 open requests, 90 days and 7
days). Counts are 64-bit with checked arithmetic. Any zero denominator, or
fewer than 5 look-alike pages, gives "not enough data" (SUT-6809) and no
division runs. At most 2,000,000 member rows are kept per site, then the
oldest fixed members are compacted (proposed). Every window and date is a UTC
instant in whole days of 86,400 seconds; the site time zone is for display
only.

### ObservationEvent

This table lists the fields of each event. Events are typed, versioned
and small. Every field is an id, number, enum or short
fixed-vocabulary word. Text from outside is never copied in.

| Field | Meaning |
|---|---|
| `event_id` | Unique id of the event |
| `schema_version` | Contract version |
| `site_id` | The registered site |
| `angle` | A1 to A10 |
| `signal` | Registry signal id (spec 0044) |
| `rule_key` | The check's id |
| `cause_key` | Template id, asset hash, rule id or setting key; or `per_page` |
| `subject` | Page id, query id, topic id or site |
| `page_ids` | Up to 200 affected page ids per event |
| `chunk_no`, `chunk_total` | Position when an emitter splits a large cluster across events |
| `facts` | At most 32 typed values; a string value must be an enum allowed for that signal, else the event is rejected |
| `measured_at` | UTC time the part observed it |
| `recheck` | The 0048 recheck kind |
| `platform` | `wordpress`, `xenforo`, `raw` or `unknown` |
| `origin` | The emitting spec, or `custom` for a Python generator |

Idempotency: an event is identified by site, origin, signal, cause key,
subject, rule key, chunk number and the UTC day of `measured_at`, so a repeat
replaces facts and unions page ids in one record; chunks of one cluster are
merged by set union and ordered by `measured_at` then `event_id`. A
`cause_key` is accepted only if it names a template, asset, rule or setting
the crawler or registry has registered; otherwise it is treated as `per_page`,
counted, and after 100 such events in one run shown as SUT-6811 for that
emitter. The per-run card cap is applied per origin and per angle, so one
noisy emitter cannot starve others. Events come only from registered in-
process emitters, and `custom` events only through the Python worker; over-cap
custom events are dropped and counted before storage. A `custom` origin uses a
`custom:` prefix, cannot emit A10 tracking signals, and is capped at 10% of
the per-run card budget (proposed). An event over the size or field limits is
rejected whole (SUT-6811).

### Cluster and card

This table lists the stored records and their columns.

| Table | Columns |
|---|---|
| `root_cause_cluster` | cluster_id, site_id, rule_key, cause_key, cause_kind (shared_template, shared_asset, shared_rule, shared_setting, single), affected_count, first_seen, last_seen, item_id |
| `cluster_member` | cluster_id, page_id, first_seen, last_seen, state (open, fixed, gone) |
| `action_card` | item_id (spec 0048), headline_rule, effort_band, effort_minutes, impact_tag, platform, steps_ref, deep_link (kind and id), hold (none, rollout, data), snoozed_until, cooldown_until, receipt_id, outcome_id |
| `proof_receipt` | receipt_id, item_id, generation, gsc (below), ga4 (below), darshak (below), sanity_status (one per source pair used), grade, sample_label, data_through, plain_text |
| `verification_run` | run_id, item_id, kind, requested_by, approved_at, pages_checked, pages_right, result per page, finished_at |
| `outcome_28d` | outcome_id, item_id, verified_at, window_anchor, before and after windows, per-source change with interval, verdict, confounded reason |

A cluster groups pages when 5 or more share one cause key
(`work.cluster.min_pages`, proposed). Members arrive in chunks and are written
in batches of 5,000 rows; `affected_count` is kept incrementally and equals the
number of `cluster_member` rows up to the cap (beyond it the counter keeps
counting and rows stop) (indexed by cluster and state), up
to 100,000 members per cluster; beyond that the card says "more than 100,000
pages (counted, not listed)", and a page that leaves the cluster is marked
`fixed` or `gone`, never deleted. One cluster is one 0048 item; its "What to
do first" call, comments and owner apply to all its pages.

The card holds ids, counts, dates and the rule's name only (spec 0048
rule 8). Headlines are built from a fixed template and counts. Query
words, term words and page titles are resolved when shown, under the
spec 0146 scan and redaction rules, and are never stored on the card.

### Proof receipt

Every source block has a status: `used`, `not_connected`,
`not_enough_data`, `withheld` or `stale`. A missing source is never read
as zero. Numbers come from stored copies and spec 0146 results through
their public APIs, with each result's own confidence object.

| Source | Fields |
|---|---|
| GSC | Impressions, clicks, CTR, expected CTR at that position (FC-04), average position, top query ids, branded and non-branded clicks, surface (web, discover, image, video, news). Page totals come from page rows, never summed from page-and-query rows, so withheld rare queries cannot distort them |
| GA4 | Organic sessions, engaged sessions, key events, assist status (FC-30: unknown, known zero or a count of 5 or more), thresholded flag |
| darshak | Visits, active reading seconds, drop-off block reference (FC-09), copy hotspot block, anchor click rate, site-search term ids, phone and desktop gap, counts under 5 suppressed |

The receipt also carries the 0146 sanity status, the grade and the sample
label, and a one-sentence `plain_text` such as "Based on Search Console
and darshak. Thin data: 11 weeks. GA4 not connected." A receipt with no
source block at all is refused: a card with no evidence is "Look into
first" (spec 0048 rule 5), never "Fund".

## Ordering, Focus 3 and the task board

Ordering stays with spec 0048. "What to do first" decides Fund, Look into
first, Freeze or Defer, ranks only inside one kind, shows reasons and limits,
and never adds sources or units. This spec adds no score. A page that earns
15,000 impressions and conversions lands in "Measured gap" and ranks high
there by its click-gain range; the same fault on an old page with no clicks
and no readers is "Structure only": it ranks inside its own kind, and 0048
keeps it out of Fund unless it fits, so it usually waits in the low-priority
clean-up view (Defer). Conversions and reader numbers sit beside the click
gain on the card, each in its own unit.

Impact tag: it comes from the card's own kind and unit, never from a mix, and
always shows its unit. For a measured gap, High is a low-end click gain of 5%
or more of the site's weekly organic clicks (`work.impact.high_pct`), Medium
1% or more (`work.impact.med_pct`), else Low; a zero or missing weekly total
gives "not enough data". For a measured risk the tag reads "Risk: high, medium
or low", by severity, with the label "no traffic value". For structure only it
reads "Tidy-up, n pages". A range for the top card of a kind is labelled "one
of N cards", because picking the best of many flatters it. Effort: each rule's
template names a minutes band per page and per template, so "15 min" is a band
from the rule, not a guess about the person. Both show as tags such as "15
min, High impact, WordPress".

**Focus 3** shows at most `work.focus.count` cards (3, proposed). It takes
cards called Fund, in this order: one open "Fix tracking first" card if any
(taken even without a Fund call; at most one, with its causes merged; others
stay on the board); then the top card of Measured gap; then the top of
Measured risk; then the next Measured gap; with more slots, Measured gap,
Measured risk and Measured gap again, repeating. An empty kind is skipped.
Structure only never appears in Focus 3. Ties break (for a risk, by higher
severity first) by larger low-end click gain, then smaller effort minutes,
then older `first_seen`, then `item_id`. With fewer cards it shows fewer and
says "Nothing else to fund", never padding. Cards on hold, snoozed, cooling
down or from a `custom` origin never appear until the owner promotes the
custom card. "Quick wins under 15 minutes" filters Fund cards whose effort
band is 15 minutes or less (proposed).

**Task board.** Columns are views over 0048 states and calls, not new
states:

| Board column | Means in spec 0048 terms |
|---|---|
| Ready | Open or Came back, call Fund, no hold |
| Look into first | Open or Came back, call Look into first |
| Later | Defer without a date |
| `In Progress` | Taken |
| Waiting for Verification | Fix claimed |
| Cooling Down | Verified or Closed, with `cooldown_until` in the future |
| Verified, waiting to close | Verified, cooldown over |
| Done | Closed, cooldown over |
| Snoozed | Defer with a `snoozed_until` date the owner set |
| Held, rollout active | Freeze, reason "update running or settling" |
| Held, other reason | Freeze for another 0048 reason (paused crawl, freeze window, moderator wait) |
| Blocked, data check | Freeze, reason "data cannot be trusted" |

"Won't fix", "Marked fixed, not checked" and "Page gone" are filters, as
in 0048. A Taken or Fix-claimed card on hold keeps its column and shows a
hold banner. Alekh (spec 0043) offers the same table with filters.

## Holds

1. **Rollout hold.** While spec 0137 reports a rollout as rolling, or
   settling for 7 full days after its end, reactive cards stay Held. This
   narrows the 0048 freeze: it applies to reactive cards only. A reactive
   card is one whose signal has `reactive` set in the registry (spec 0044),
   which an emitter or script cannot change: traffic or CTR moved (decay in
   FC-03, CTR drops, settled update shifts). If 0137 data is stale or
   unreadable the hold stays (fail closed; SUT-6802). A hold lasts at most
   90 days (`work.hold.max_days`, proposed); then the card says "held too
   long" and a person decides. Fixes for a broken redirect,
   a slow image or a missing tag are not reactive and are not held. A
   held card re-enters when 0137 reaches `analysis_window_open`.
2. **Data hold.** The compiler reads the spec 0146 FC-32 sanity status,
   which is one status per source pair. A card is Blocked (SUT-6803) when
   any pair its evidence uses is "Broken". A "Caution" pair lets the card in
   with grade "low" and a badge (SUT-6804); "Unchecked" (including spec
   0034 mode 2) shows "can't cross-check". A person may choose "show
   anyway, grade low" with a reason; such a card can be Look into first,
   never Fund. Cards that need
   only the crawl (a broken link) stay Ready.
3. **Fix tracking first.** When any pair FC-32 checks is Broken, one A10 card
   is opened per root cause (tag break, missing days, bot burst). These cards
   are exempt from the data hold and from 0048's "weak evidence is never
   Fund". It is verified when FC-32 is Healthy for `work.tracking.clean_days`
   (7, proposed) final days; a flap inside that time reopens the same item
   (0048 return rule).
4. **Precedence and safety.** For one card: Page gone; then Came back (reopens
   at once, ignoring cooldown and snooze); then data hold; then rollout hold;
   then snooze; then cooldown. No waiver (spec 0114 BJ-29 covers gate waivers,
   not holds), offline sign-off, MCP tool or Python hook can lift or skip a
   hold (the Fix tracking first card is exempt because it is never held); the
   only override is a person's per-card "show anyway" on a data hold, which
   never makes a card Fund, and "held too long" does not lift a hold. A
   tracking card that reopens more than 3 times in 30 days stays in one slot,
   labelled "unstable tracking". A recheck run stops after 20 active minutes
   (SUT-6813). An active run expires after 20 minutes without a heartbeat and
   is marked failed. The run's page list is frozen at its start; a page that
   joins waits for the next run. Each card carries a version, changes by
   compare-and-set, and has at most one active `verification_run`.

## Verification, cooldown and outcome

**Verify.** "I finished this" moves the card to Fix claimed (spec 0048). The
crawler's "check my fix" (spec 0044) re-reads only the card's pages (at most
`work.recheck.max_pages` per run; a retry re-reads only the pages that failed,
and only failures are stored with a count of passes), as a heavy-lane job
through the scheduler, and records a `verification_run`. "211 of 212 right"
keeps the card open and names the page left, with the reason in plain English.
An unreadable page proves nothing: the card shows "recheck due" and offers a
one-click "Try again" (never an unattended retry; spec 0048 makes a recheck a
person's click). After `work.recheck.retries` (3, proposed) failed attempts it
says "We could not read these pages. Check they are public, then try again"
(SUT-6807). If the owner is offline or no recheck is possible, the only path
is 0048's "Marked fixed, not checked", open to the owner or a named staff
role only (never an MCP tool or script): a written reason, flagged everywhere,
never counted as fixed (SUT-6814). A local MCP agent may only request a
recheck; a person approves it, and it stops at pause-all.

**Cooldown.** When 0048 sets a card to Verified, `cooldown_until` is set to 28
days later (`work.cooldown_days`, proposed). During it the compiler records,
but does not open, a new card for the same page, rule and cause key when the
registry marks the signal `cooldown_class: content`, and the board shows
"Cooling Down (n suppressed)". Cooldown never hides a fault the recheck
proves: the 0048 return rule still reopens the item as "Came back" at once.
Technical faults, and faults with another cause key on the same page, are
unaffected. A page that joins a cooling cluster shows "new page joined"; if it
tops its own kind it opens its own card. Repeat records are kept as one
counter per page and rule.

**Outcome receipt.** The outcome job runs only after the card is Verified. The
anchor is the later of the edit's publish date and the claim time (the claim
time when there is no publish record), + 4 days + `work.outcome.days` final
days, the outcome job writes `outcome_28d`, extending the spec 0146 FC-20
method to the card's primary metric (named by its rule: clicks, CTR, reading
depth or sessions). It compares the days before the fix with the days after,
minus the change in 5 or more look-alike pages: same category, matched on
their level before the fix, not on the same template, frozen when the person
claims the fix, within 20% of the pre-fix metric (else "not enough data"),
seasonally adjusted, with a bootstrap interval and false-alarm control. The
seed is a hash of `outcome_id` for a named, pinned PRNG; look-alike sampling
and resampling use that stream; sums run in ascending `page_id` order in
integer or fixed-order arithmetic; Linux and Windows must give identical bytes
(golden test in IT-0147-OUTCOME). The floor is 100 impressions per Search
Console window, 100 visits for darshak and 100 sessions for GA4 (proposed).
The other sources are shown as context, each in its own unit, and a stale or
withheld source shows "not available". Words follow 0048: "helped", "not
shown", "not enough data" or "confounded" (a rollout, a second edit or a "Came
back" inside the window, a same-cause repeat card (a suppressed repeat record
counts), or a missing primary source; SUT-6809 for not enough data). "Helped"
needs the interval, after control, to exclude zero for the primary metric; it
says "observed", never "caused". A source that is missing shows "GA4 not
connected", never zero. Outcomes feed the ganit accuracy tracker only through
the route in 0048 ("before-and-after tests judge the allocator"); they tune
nothing directly. A scoreboard shows, per angle and rule, how many cards were
opened, fixed, verified, measured and helped, counting confounded and not-
shown outcomes in the rates and saying that many cells were compared; a cell
needs 8 outcomes (proposed), else "not enough results yet".

### Start experiment

An eligible current card can open a separate
[0148 experiment](0148-scientific-experimentation-engine.md)
with its exact site and revision reference. Claiming, verifying
or reporting the card creates no study, registration or lock.
A scientific receipt can appear beside the ordinary observed
outcome. A 0151 guidance proposal changes only an eligible
advisory effect after exact owner confirmation. The 0048 fixing
and ordering rules keep their existing inputs and effects.

Keep the ordinary 0146 FC-32 diagnostic on the card. The
[0150 sensor gate](0150-experiment-source-and-sensor-gates.md)
owns the three independent source proofs for a verdict. In
BDD-0148-START and BDD-0150-THREE-SOURCE, use an eligible card with
a Healthy diagnostic from two sources and no darshak proof.
Claiming the card creates no experiment. A separately authorised
Start experiment opens only its exact draft. The diagnostic stays
available and the verdict stays held until all required proofs
qualify under 0150.

## Customisation, MCP and exports

**Python (spec 0104, fenced).** A script may add a prioritisation view, a
grouping rule or a card generator. It works only when `work.python_hooks` is
on (owner-only, never-changeable class of spec 0020 rule 5; DR-140; QR-225 for
snapshots). It reads a read-only snapshot of cards and receipt numbers with no
secrets and no resolved query, term or page text, deleted after the run and
pinned to one generation, under 0104's caps and codes (SUT-910 to SUT-916). A
custom order only reorders the owner's own "Custom order" view and is labelled
"custom"; it never changes "What to do first". A generator emits
`ObservationEvent` with origin `custom`, re-validated like any event
(SUT-6811), at most 5,000 a day; its cards need the same evidence as any other
and a person still approves every change. Output is typed, at most 10,000
rows, with non-finite numbers refused; hot-reload is atomic with a dry run,
and a failure keeps the previous version.

**MCP (local only, spec 0020).** Read tools: `tasks.focus`, `tasks.board`,
`tasks.receipt`, `tasks.verifications`, `tasks.outcomes`. Request tools
(`tasks.request_recheck`, `tasks.request_sync`, `tasks.request_status`) need a
site "request" grant above read, can name only a card's stored page ids, and
only create an approval request (SUT-6808): at most 3 open per site and tool,
expiring after 7 days (proposed), duplicates merged; a request over the cap is
refused (SUT-6808). No tool can set Verified, Closed, Came back, Page gone,
Won't fix or Marked fixed not checked (0048 rule 4), start a sync or fetch
without approval, or edit a page. Every tool stops at pause-all, needs site
read permission, returns at most 100 rows a call under a rate limit of 60
calls a minute per site and agent session (proposed), and returns text from
outside only in fenced "untrusted" fields cut at 120 characters.

**Exports.** Three file formats (CSV, Markdown, JSON) use spec 0048's import
and export rules and its safe CSV test: every CSV text cell whose first
visible character, after any space or bidi mark, is `=`, `+`, `-`, `@`, tab or
carriage return is neutralised, and Markdown and JSON string values are
escaped with control and bidi characters stripped. An export holds at most
10,000 cards (SUT-6815). Query and term words are resolved only after the
scan. Three copyable templates (Jira, Linear and Asana) are plain text the
person pastes or uploads. There is no Notion template or connection:
DR-076 and F-0495 of spec 0100 refuse it, and any Notion-equivalent
workspace is a separate series (DR-165). A card's deep link never
carries a page address to an outside provider. Jira, Linear and Asana
connections stay as 0048 "Templates and outside task tools" defines them: one-
way first, owner-keyed, and the outside "finished" mark never closes a card.
An export for the owner's own use may include guest-visible page addresses; a
template shared between installs holds none (0048).

## Budgets and speed

Designed, not run; every number is proposed until measured on the Dell.
The compile pass runs per audit run as a heavy-lane job under 0019, 0033
and 0099 and asks spec 0078 for admission. Screens read stored cards, not
raw events.

| Observation events in one run | Compile pass (proposed) | Peak memory (proposed) | Focus 3 or board read, slowest 1 in 20 (proposed) |
|---|---|---|---|
| 100,000 | 10 s | 128 MiB | 100 ms |
| 1,000,000 | 2 min | 128 MiB | 100 ms |
| 10,000,000 | Several slices of 1,000,000 events, about 2 min each, not measured | 128 MiB | 100 ms |

Memory is a 64 MiB chunk plus at most 64 MiB of cluster state, so it does not
grow with events; the cluster map spills to the store past 500,000 open
clusters (about 128 bytes each). The compiler asks spec 0078 for 128 MiB; a
refused admission defers the run with the usual waiting-for-memory badge. A
resume adds under 5% to a slice. The Focus 3 row and board columns are also
refreshed at the earliest `cooldown_until`, `snoozed_until` or hold end, after
each sync and on a settings change. Raw events are capped by a store budget
agreed with spec 0099, compacting the oldest first. Outcome jobs ask for 128
MiB in the heavy lane and run daily. Receipts are rebuilt only when their
input watermark changes, from one grouped query per source, reading at most
10,000 member pages per receipt (sampled by seed and labelled). The read path
uses indexes on site, board column, call, kind and hold, a materialised Focus
3 row updated on each `CardUpdated`, and a board paged by key, 100 rows a
page, for 10,000 open cards. Outcome jobs run as one batch in the heavy lane,
at most 200 per run by due date, with at most 1,000 resamples and 500 look-
alike pages sampled by seed. Raw events are kept 35 days and then compacted
into cluster rows.

Per run the compiler files at most `work.flood.items_per_run` new cards
(spec 0048); the rest wait and the screen says how many. Events stream in
64 MiB chunks. A cluster's member list is stored as ids, never loaded
whole into the screens. A slice stops after 20 active minutes and resumes
with a hash and erasure check, as in spec 0146. Only the release long
lane (DR-069) runs the 10-million-event test.

## Settings (proposed, blocked until admitted like the 0048 keys)

This table lists the settings this spec adds. Like the 0048 keys, each
is blocked until its value, bounds and evidence are admitted.

| Setting | Type, default and range | Takes effect |
|---|---|---|
| `work.focus.count` | Site integer, 3, 1 to 5 | Next view |
| `work.quick_minutes` | Site integer, 15, 5 to 30 | Next view |
| `work.cooldown_days` | Site integer, 28, 14 to 90 | New verifications |
| `work.outcome.days` | Site integer, 28, 14 to 56 | New verifications |
| `work.cluster.min_pages` | Site integer, 5, 2 to 50 | Next compile |
| `work.recheck.retries` | Site integer, 3, 1 to 5 | Next recheck |
| `work.impact.high_pct` | Site integer percent, 5, 2 to 20 | Next view |
| `work.impact.med_pct` | Site integer percent, 1, 1 to 5, and below `work.impact.high_pct` | Next view |
| `work.tracking.clean_days` | Site integer, 7, 3 to 14 | Next check |
| `work.hold.max_days` | Site integer, 90, 30 to 180 | Next view |
| `work.python_hooks` | Owner Boolean, off. Never-changeable class (spec 0020 rule 5; spec 0014 `owner_only`): the CLI, MCP and agents cannot change it | Next script run |

## Screens

All screens use plain English first. "Why?" opens the proof receipt.

1. **Focus 3.** Up to `work.focus.count` cards with a verb headline, tags, one
   line of proof and a "Do this" button that opens the exact place: a Coach
   block, a link in Internal Link Review, an asset in the Performance Lab or
   the audit row. A "Quick wins" toggle and "Show all" open the board.
2. **Task board.** The board columns above, with filters by angle, page
   group, owner, effort and source; Alekh table view; export menu.
3. **Proof receipt drawer.** Three blocks (Search Console, GA4,
   darshak), each with its own badge: "used", "not connected, connect",
   "not enough data", "stale". Sanity banner (spec 0146), grade and
   sample label, and "what would change this".
4. **Fix guide.** Step-by-step instructions for WordPress, XenForo or raw
   templates, built from the rule's template; nothing is applied for the
   person.
5. **Verification and outcomes.** The recheck result ("400 of 400 right"),
   the cooldown date, and the 28-day outcome receipt with the scoreboard.

A card with no connected source says "We only have your crawl for this
one" (SUT-6800), never an empty proof. Offline, screens show saved cards
with "Last updated".

## Failure codes

Codes SUT-6800 to SUT-6899 are reserved for this spec (DR-142).

| Code | Owner message |
|---|---|
| SUT-6800 | “We only have your crawl for this one. Connect a source to see its effect.” |
| SUT-6801 | “This card needs {source}. Connect it to see the numbers.” |
| SUT-6802 | “Your {source} data is out of date. We’ll catch up on the next sync.” |
| SUT-6803 | “Your tracking looks broken, so tasks that rely on it are paused.” |
| SUT-6804 | “Your sources disagree a little. Treat these numbers as rough.” |
| SUT-6805 | “A Google update is still settling, so this waits until {date}.” |
| SUT-6806 | “You fixed this recently. We’ll stay quiet until {date}.” |
| SUT-6807 | “We could not read these pages. Check they are public, then try again.” |
| SUT-6808 | “A person has to approve this check before it runs.” |
| SUT-6809 | “There isn’t enough data yet to say whether this helped.” |
| SUT-6811 | “A finding was rejected because it was not in the expected form.” |
| SUT-6812 | “More tasks are waiting. We’ll add them on the next run.” |
| SUT-6813 | “This check is taking too long. Try again.” |
| SUT-6814 | “To mark this fixed without a check, write a short reason.” |
| SUT-6815 | “This export is too large. Choose fewer tasks.” |

SUT-6810 is not used: quota exhaustion uses SUT-632 of spec 0040. Each code
fires where stated above: 6800 and 6801 when sources are missing; 6802 on
stale data; 6803, 6804 in the data hold; 6805 in the rollout hold; 6806
in cooldown; 6807 and 6813 in verification; 6808 on a request tool; 6809
in outcomes; 6811 on a rejected event; 6812 at the card cap; 6814 on an
offline sign-off; 6815 on an export.

## Requirements

Each row names the test that proves it.

| Requirement | Acceptance |
|---|---|
| REQ-0147-CATALOG | UT-0147-CATALOG and FX-0147-ANGLES MUST show every registry signal (spec 0044) names a `card` rule or `evidence_only` with a reason, and an unnamed one fails the gate. IT-0147-ANGLES emits one synthetic event for each of A1 to A10 and expects a card with the right recheck kind |
| REQ-0147-ROLLUP | UT-0147-ROLLUP MUST cover 4, 5 and 400 pages on one cause key, `per_page` fallback, two causes on one page, a page leaving a cluster, and `affected_count` always equal to member rows |
| REQ-0147-EVIDENCE | UT-0147-EVIDENCE MUST show page totals not summed from page-and-query rows, branded and non-branded split, each surface separate, missing source never zero, all seven source combinations badged, and no receipt with no source block ranking as Fund |
| REQ-0147-GATES | IT-0147-GATES (with FX-0147-SANITY and FX-0147-ROLLOUT) MUST cover the tracking card's Fund and hold exemptions, stale 0137 data, a hold on a Taken card, no waiver, sign-off or script lifting a hold, flapping tracking status, a forged non-reactive flag, and FC-32 Broken, Caution, Healthy and Unchecked, the "Fix tracking first" card and its 7-day close, reactive versus non-reactive cards in a rollout and its settling week, and the hold ending at `analysis_window_open` |
| REQ-0147-ORDER | UT-0147-ORDER MUST prove the tie-break order, an empty kind, fewer than three cards, one tracking card at most, Structure only never in Focus 3, a zero weekly total, no combined score and no summed sources, Focus 3 order, ties, the Quick wins filter and impact tag equality at 1% and 5% |
| REQ-0147-LIFECYCLE | IT-0147-LIFECYCLE (with FX-0147-COOLDOWN) MUST cover one active run per item, concurrent claim and recheck, same page with a different cause key, a return during snooze, offline and stale-source screens (SUT-6802), and claim, recheck, "211 of 212", unreadable pages, retries, offline sign-off with a reason, cooldown suppression counted, a return reopening during cooldown, and "marked fixed, not checked" never counted |
| REQ-0147-OUTCOME | IT-0147-OUTCOME MUST cover the FC-20 windows, look-alike set frozen at the fix, confounded cases, each missing source, interval equal and excluding zero, the scoreboard floor of 8, and "observed" wording |
| REQ-0147-EXPORT | UT-0147-EXPORT MUST cover the three file formats and three templates, the 10,000-card limit (SUT-6815), leading-space and bidi-prefixed formulas, formula-cell neutralisation, no secret or private data, and no shared template holding an address |
| REQ-0147-PY | IT-0147-PY MUST run a good, a slow, a hanging-dry-run, a crashing, an oversize-output, a NaN-output and a secret-seeking synthetic script, check the "custom" label, that "What to do first" is unchanged, and the off state |
| REQ-0147-MCP | IT-0147-MCP MUST check every tool against pause-all, a wrong grant, the 100-row and 60-call limits, approval-only request tools, and that no tool can set a recheck-only state |
| REQ-0147-SAFE | IT-0147-SAFE MUST cover a forged cause key, a free string in a fact (including from a custom origin), a custom flood and a custom event merging into a real cluster, cross-site reads and writes on every table, and site role, vault secrets absent from cards, exports, snapshots and MCP, hostile and personal text in events (rejected or redacted), bidi and control stripping, instruction-like text never acting, and erasure with an interrupted cleanup |
| REQ-0147-WALLS | UT-0147-GATEWALK MUST show the compiler has no dependency that can fetch, write a site, send or spend; UT-0147-NORANK that no import path reaches ranking; UT-0147-CODES that every SUT code fires with its message |
| REQ-0147-LIMITS | UT-0147-LIMITS MUST test every setting above at its bounds and one step outside, including `work.impact.high_pct` at 1, 2, 20 and 21, `work.impact.med_pct` at 0, 1, 5 and 6 and above `high_pct`, `work.tracking.clean_days` at 2, 3, 14 and 15, `work.hold.max_days` at 29, 30, 180 and 181, and `work.recheck.max_pages` of spec 0048 |
| REQ-0147-EVENTS | IT-0147-EVENTS MUST send each outbox event twice and expect one record, with record and event in one transaction, and an event over the limits rejected whole (SUT-6811) |
| REQ-0147-SCALE | PF-0147-SCALE MUST also time the board and Focus 3 reads for 10,000 open cards at the push tier and a 100,000-member cluster, and the cluster-map spill in the long lane. It MUST time 10,000 events at the push tier (30 runs, median, within 5% of the last committed run) and 100,000, 1,000,000 and 10,000,000 only in the release long lane, with flat memory and a resumed checkpoint |
| REQ-0147-EXAMPLE | FX-0147-RAVI MUST run the worked example as a golden fixture so its sentences cannot drift from the rules |
| REQ-0147-REPLAY | QM-0147-LIFECYCLE, QM-0147-GATES and QM-0147-ERASE with RP-0147 replay through the Rust jobs, compiler and file ledger with Quint Connect |

## Security and privacy

- Cards, clusters and receipts hold ids, counts, dates and rule names. No
  post text, member name, visitor id, raw address or private forum
  identity is stored (spec 0048 rule 8; spec 0034 privacy ledger).
- Search Console and GA4 credentials stay in tijori; the compiler reads
  stored copies. Credentials never appear in cards, exports, snapshots,
  Python workers or MCP output.
- Event facts, query words, term words, page text and crawled snippets
  are untrusted data. They are never a prompt and never run. Text is
  scanned under the spec 0146 rules and redacted if the scan fails or
  cannot run; bidi and control characters are stripped; fields are cut at
  120 characters; a log line holds no newline from them.
- An event is checked before it is counted: known signal and site, page
  ids belonging to the site, enum values from the allowed lists, at most
  200 page ids and 32 facts. A bad event is rejected whole.
- Offload keys are per-site data keys held in tijori and never sent out;
  tijori backups exclude them, or tombstone them on restore;
  an object delete is attempted, and a failure is retried and flagged.
- Only staff with permission for the site see its cards. Guests see
  nothing. Row-level security applies to the records.
- Nothing here sends data outside the PC. Syncs and DataForSEO lookups
  follow specs 0040, 0132 and 0133 and DR-117 and DR-118, on owner approval.
- Every card, receipt, verification, outcome, snapshot, export and offload
  file is listed in the file ledger (spec 0030, `LedgerWriter`), kind
  `task_*`, group `tasks:<site_id>`. Erasing a site or the product removes
  them all. A completed-outcome archive may be offloaded to an external
  SSD or S3 only through the storage wizard, encrypted on the PC first,
  with the owner's approval; erasure deletes the key first.
- No output of the compiler is read by live ranking. The module imports
  no ranking crate and the architecture check refuses an import path to
  one.

## How it is tested

Designed, not run. Every test uses synthetic data: fictional sites,
pages, queries and drafts from a seeded generator. None calls Google, GA4
or a real site.

- **Unit and integration:** as in the requirements table. Fixtures:
  FX-0147-ANGLES (one event per angle), FX-0147-RAVI (the example),
  FX-0147-SANITY (a tag break, a missing day, a bot burst, ad-blocker
  skew), FX-0147-ROLLOUT (rolling, settling, open) and FX-0147-COOLDOWN.
- **Setting limits:** `work.focus.count` at 0, 1, 5 and 6;
  `work.quick_minutes` at 4, 5, 30 and 31; `work.cooldown_days` at 13,
  14, 90 and 91; `work.outcome.days` at 13, 14, 56 and 57;
  `work.cluster.min_pages` at 1, 2, 50 and 51; `work.recheck.retries` at
  0, 1, 5 and 6. All in UT-0147-LIMITS.

### Quint

QM-0147-LIFECYCLE: one site, two clusters, states Open, Taken, Fix claimed,
Verified, Closed, Came back and Marked fixed not checked, holds on or off, a
cooldown clock. Actions are emit, roll up, claim, recheck pass, recheck fail,
return, offline sign-off, tick, hold on and hold off. Assert
`NoVerifiedWithoutRecheck` (offline sign-off is a separate labelled state),
`CooldownBlocksRepeatCard`, `ReturnNeverSuppressed`, `NoMemberDropped`,
`NoReadyWhileBlocked` (same exception), `NoReactiveReadyInRollout`,
`NoHoldLiftedByWaiver` (sign-off, tool or script) and `OneActiveVerification`.

QM-0147-GATES: three sources with healthy, caution, broken, missing and stale
states and a rollout phase. Assert `NoFundOnBrokenSource` (except the Fix
tracking first card), `MissingIsNotZero`, `TrackingCardOpenWhileBroken` and
`NoAutoEdit` (nothing changes a draft or site without an approval).

QM-0147-ERASE: two sites, cards, receipts, outcomes, snapshots, exports,
an offload copy and a backup; actions are write, offload, delete site,
uninstall, crash, restore and pause-all. Assert `NoOrphanFile`,
`NoStaleRestore`, `VaultSecretNeverOutside` (cards, exports, snapshots,
MCP), `ErasureReachesOffload`, `PauseHaltsFetches` and
`NoRankingOutput`.

RP-0147 replays each trace through the Rust compiler, scheduler and file
ledger with Quint Connect. Bazel proves identical behaviour, seeds,
clocks, memory and erasure on Ubuntu 26.04, Debian 13 and Windows 11.
Product checks are designed here and run with the implementation.

## Owner items (design on, product off until answered)

This table lists the questions the owner must answer before a feature
turns on or a rule changes.

| Item | Question | Default until answered |
|---|---|---|
| QR-225 | May Python scripts read read-only snapshots (specs 0146 and 0147)? | Off |

Once admitted, these are the owner's own settings: the cooldown, the
Focus 3 count and the impact thresholds; the settle wait belongs to spec
0137. Other numbers are proposed, to be admitted by measurement.

## Changes from the source

This spec uses the repo's names: Alekh, `docs/registers/decisions.md`,
numbered specs and the `work.*` settings of spec 0048. The request asked
for a score that combines sources and for Notion exports; both are held
back as stated above (DR-164, DR-141, DR-165). Its board column names are
views over spec 0048's states, so the Fix list keeps one lifecycle.
