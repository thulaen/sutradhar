# 0043: Search charts

Search charts answer one question for a site owner: how are my search
numbers moving over time, and is this move unusual for them? One chart
shows one number across days. On it sit indicators (lines worked out
from the numbers, such as a 14-day average), alerts, a filtering tool
that finds every series matching a test, and short chart rules the admin
writes. It is part of roadmap Phase 6; "Which spec designs each feature"
in [features.md](features.md) lists the features it designs in whole or
in part. In code it is the `quarry` module.

Example: an admin opens Search Console clicks for one forum section over
180 days. A 14-day average runs through them. A band on 3 March says
"search update noted". The last three days are shaded "still arriving".
The admin sets an alert, "clicks fell more than usual for this section",
which can fire only on days Search Console has finished.

## What it does

### Rules that hold throughout

- Staff-only screens, in the desktop app. Guests see nothing new.
- It reads copies of numbers and changes nothing: no page, post, link,
  setting or ranking. Monitors only alert. A chart rule can never change
  a result or a page.
- Each site and agency client keeps its own rows. A chart never mixes two
  sites. Nothing is shared between installs.
- It starts on with researched values. Anything not measured shows "not
  measured", never zero.
- Numbers come only from the approved metric sources: Search Console,
  Bing, GA4 and darshak. Sources are never added together.
- Rival and link numbers come only from services the owner connects and
  from backlink records ([0045](0045-backlink-crawl.md)), charted as one
  labelled source, never mixed with another.
- Member text and search words never enter a chart rule. Rules see ids
  and numbers only.

### What it reuses

The visual layer's charts, loader and "as of" stamp
([0017](0017-visual-layer.md)); the connectors' daily copies and coverage
([0040](0040-search-connectors.md)); ganit job 214's late-day estimate;
darshak's visit numbers and "fewer than 10" rule
([0034](0034-analytics.md)); ganit jobs 204, 212, 161 and 176 for
forecasts, changepoints and anomalies; the backlink crawl's outside
notices; the audit log and undo history (180 days); the falling test,
support gate and own baseline of the site structure checks
([0047](0047-site-structure-checks.md)); the one filter form of the
action layer ([0020](0020-screens-actions-sutra-mcp.md)); saved layouts
from the fix list ([0048](0048-fix-list.md)); locked sandbox rules
([0037](0037-extension-isolation.md)); grouped alerts and quiet hours;
durable jobs and `karya.shed_order` ([0019](0019-events-jobs-recovery.md),
[0033](0033-safe-running.md)); the metric catalogue
(`catalogue/metrics.toml`); row-level security; GanitEvolve.

New here: series state and polarity rules, the chart screen, the
indicator set, chart alert rules, the filtering screen and the chart
rule engine. No new chart library, query language, job system, sandbox,
alert list or number store.

### The chart screen

- A search box picks a series: a query, a page, a section (a forum or
  category) or the whole site. Then a number: clicks, impressions, click
  rate or average position. Then a source, if more than one holds it.
- One or two panes. Indicators and event bands sit on or under the main
  line. A table of the same points sits beside it and loads first.
- Compare with an earlier period is on by default: the same days of the
  period before (`quarry.chart.compare_days`, about 28).
- Saved views per admin keep series, dates, indicators and rules. A
  view's link rebuilds it for a signed-in staff member of that site only,
  showing data "as of" the save date, with a switch to "as of now".
- Daily, weekly and monthly views. The service rolls up, never the
  window.
- Keyboard use for every action. High-contrast theme. Lines differ by
  label and dash style as well as colour.
- Loading, empty, error, retry, offline and unsaved changes are each
  shown in words. Errors appear on the chart, never as a pop-up.

### Series and sources

| Series | Source | Note |
| --- | --- | --- |
| Clicks, impressions | Search Console | Daily. Bing: its own lines |
| Average position | Search Console | A daily weighted average |
| Click rate | Same source | Clicks over impressions, same day |
| Sign-ups from search | GA4 | By landing section |
| Human sign-up rate | darshak | Humans only |

Search Console's position is an average across searches, weighted by
impressions. It is not a rank watched each day, and the screen says so.
Search Console leaves some rare searches out of query-level numbers, so
those charts carry a note. Each series needs a catalogue entry (source,
the decision it serves, what it is compared with). No entry, no line.

### Honest chart rules

- HC01: each day is closed (final), still arriving (a late source's
  newest days) or not measured. Each looks different and is named in
  the table.
- HC02: not measured is a gap, never zero and never joined across.
- HC03: alerts, filters, indicators and rules use closed days only.
- HC04: job 214's estimate is a shaded band labelled "estimate", never
  in a line of copied numbers. With no result: "not yet available".
- HC05: daily stays daily. Weekly and monthly bars use closed days only
  and skip a part period. No intraday detail and no candles.
- HC06: for position, lower is better and 1 is at the top. Words say
  "better" or "worse", never "up" or "down". Polarity travels with the
  series through every indicator, alert, filter and rule.
- HC07: a day with too few impressions shows "too few impressions" and
  no position line. The floor is `quarry.chart.min_impressions`.
- HC08: two sources are two labelled lines. A sum or ratio needs one
  source and one day grid. Unlike settings or dates: "do not compare".
- HC09: fewer than 10 visits or clicks from darshak shows "fewer than
  10".
- HC10: a forecast range shows only when a ganit result exists and
  history is long enough. Otherwise nothing is drawn.
- HC11: zooming never changes a peak. Drawing thinner may only keep
  highs and lows; the table keeps every point.
- HC12: an event band says what happened and when, never why. Its note
  reads "association, not cause".

### Event bands

Three kinds: search update notices from the backlink crawl's outside
notices; Sutradhar's own changes from the audit log (an approved link
batch, an undo); notes an admin types. Events close together merge into
one band with a count; none is dropped. A band opens its record. Nothing
is read from other people's sites for this.

### Indicators

An indicator helps one decision: is this move normal for this series,
or worth a look?

- One definition of each indicator lives in ganit. The chart, alerts,
  the filtering tool and chart rules all call it. A second copy anywhere
  is a defect.
- Closed days only. A window with too many days not measured (share set
  by `quarry.chart.min_measured_share`) gives no value and says "not
  enough measured days". Missing days are never zero or filled in.
- Position follows HC06, impression-weighted. The falling test's support
  gate and own-baseline rules apply: too few impressions gives "not
  enough evidence", never a line.
- Every indicator starts on, shown as "usefulness not yet measured"
  until research scores it on real history. One that stops helping is
  demoted from the default view, visibly and reversibly. Among rival
  methods the winner is on and others can be picked.
- Each indicator needs a catalogue entry. Windows are researched per
  indicator and set as defaults.

| Family (plain name) | Question it answers |
| --- | --- |
| Averages and crossings | Is the trend turning? |
| Straight-line trend channel | How strong is the trend, and is a break real? |
| Best and worst of recent days | Did this series just beat its last best? |
| Unusual-move bands, squeeze | Moved more than usual for this series? |
| Typical daily range | How big is a normal day for this series? |
| Pace of change | How fast is it moving, and is the pace fading? |
| Place in its recent range | Near its best or worst of the window? |
| Impression flow | Is demand building or draining behind the clicks? |
| Weighted average position | The honest headline position, by impressions |
| Position against impressions | Demand shifting behind a steady position? |
| Clicks against position | Fewer clicks at the same position? |
| Climb with fading pace | An improvement that may not hold |
| Spike with no clicks | Impressions jumped, clicks did not follow |
| Share improving or worsening | Is growth narrowing across my series? |
| New bests and worsts | Breadth of change across my series |
| Section against site | Is one section moving apart from the rest? |
| Sections moving together | Unrelated sections changing alike? |
| Fall from peak, days below it | How deep and how long was a fall? |
| Concentration | Does a few pages' share of clicks make me fragile? |
| Trending or settling | Act or wait? (may be set aside by research) |

"Typical daily range" uses day-to-day closes, as no intraday highs
exist. The trend-fit family uses a robust method chosen by research.
"Sections moving together" is association only.

Set aside: link quality scores (link counts chart as the one labelled
source); a rival index with no connected source; an index of other
sites' turbulence (needs sharing between installs); effort sizing;
value-at-risk and steady-growth ratios, unless research ties one to a
decision. Market-style indicators are rebuilt against the owner's own
sections, or not at all, and named for what they measure.

### Alerts

An alert watches one series, or a saved filter's result, and tells staff
when a condition becomes true. It is a notice and nothing else.

- Conditions from a menu: crosses its average; leaves its usual band;
  falls from its recent best by more than a size; indicator better or
  worse than a level; new best or worst in N days; newly matching a
  saved filter; data went missing. A chart rule can also return yes or
  no.
- "Usual" means the series' own spread by default; a fixed level is
  allowed.
- Checked after each pull, on closed days only. A move into or out of
  "not measured" never fires, except by "data went missing".
- "Test on history" shows the days the rule would have fired. It sends
  nothing. A new rule starts only after that has been seen.
- The notice carries rule name, series, days, source, size of the move
  and "association, not cause", and links to the chart with the rule
  drawn on it.
- It joins the grouped alert list. Notices from one rule in one window
  merge ("12 series matched") with every row kept. Staff mark one seen
  or closed. A repeat updates the same notice;
  `quarry.alert.repeat_gap_hours` limits repeats.
- Quiet hours hold notices for one digest. Faults ("this rule could not
  be checked") still get through at once.
- Outside the app, a notification holds a count and a link, never a
  search word. Existing channels are used.
- A failing rule raises one fault notice. It never fails silently and
  never fires a data alert.
- No alert edits, hides, links or redirects anything.

### Filtering tool

It finds every series that passes a test, such as "pages in the guides
section whose clicks fell more than usual over the last 28 complete
days, with enough impressions".

1. Step one chooses series with the action layer's one filter form:
   site, section, source, page or query text, minimum impressions. It
   runs in the database, in pages, in one pass, on that site's rows.
2. Step two is the test: the alert menu, or a chart rule returning yes
   or no per series, run only on series that passed step one.

- A rule test runs on at most `quarry.filter.universe_cap` series. Over
  that, the screen says "too many to test: narrow the filter" with the
  count. It never tests only the first few.
- Results are a ranked, sortable table: series, evidence, days used and
  a small trend line. Rows with too little evidence are listed apart as
  "not enough evidence" and never ranked. A row opens the chart.
- A saved filter reruns on a schedule, per admin and site, showing
  "newly matching" and "no longer matching" with dates. It is a
  low-priority job, shed first under pressure, and can raise an alert.
- "Falling, as in the report" calls the site structure report's own
  function, so a list and the report agree. Filter rows are not
  findings.
- Result lists hold search words, so they are staff-only and never in a
  notification or export without the export preview.

### Chart rules

A chart rule is a short declarative rule in TOML that an admin writes in
a plain text box, run by a Rust rule engine. It does one of three jobs:

- Line: draws one extra line or a few marks on a chart.
- Test: yes or no for each series, for the filtering tool.
- Alert condition: yes or no for a series on each closed day.

Nothing else. A rule cannot suggest or make a change. Its output is
labelled "your rule", in its own style, with the sources it read. It
never feeds a ganit result, a ranking, a link suggestion, a Coach tip or
anything else that decides.

Shape only (names settle in the build):

    api = 1
    kind = "alert"
    series = ["clicks"]

    [[let]]
    name = "usual"
    call = "spread"
    of = "clicks"
    days = 90

    [when]
    worse = { of = "clicks", days = 14 }
    more_than = { times = 2.0, of = "usual" }

What a rule sees and returns:

- Only the series named in its header; naming is the permission. The
  engine sends those series for a fixed run of days and nothing more.
- Ids and numbers only, never search words, addresses, member text,
  other sites, settings or secrets.
- Closed days only; asking for days still arriving is refused.
- Each value is closed or not measured. Not measured stays not measured
  in every sum, ratio and average; a rule cannot turn it into zero.
- Position keeps its polarity; the rule language speaks of "better" and
  "worse".
- Built-in indicators are calls answered from ganit, so there is still
  one definition of each.
- It returns a fixed shape: a list of (day, number), or marks (day, a
  label from a fixed list or short plain text), or yes or no. A checker
  validates it: days inside the range, finite numbers, size caps, no
  markup. Output that fails is refused whole, with the reason shown.
- Same rule, same input, same output. No clock, no random number, no
  unordered walk of data. The day being examined is an input.
- A header naming two sources is refused unless the rule only shows
  them side by side. HC01 to HC12 apply unchanged.

### How a rule runs safely

No layer is trusted alone.

1. Before running, a check refuses a rule that is too long, names an
   unknown interface version, or uses an unknown key. The box shows the
   line.
2. The engine knows only the small rule vocabulary: no import, no
   evaluation of text as code, no clock. Limits apply to steps, nesting
   depth and data size.
3. Rules run in a locked sandbox made under the extension isolation
   rules (Linux namespaces, seccomp and Landlock, no container program):
   non-root, read-only root, all capabilities dropped, no new
   privileges, the reviewed seccomp list, memory limit equal to swap
   limit, process and processor limits, no host folder, no extra
   socket, no own store and no network at all.
4. One pipe in and one out: data and indicator answers in, the checked
   result out. No environment secret, no other door.
5. Steps are the first limit and are repeatable on any machine. A
   wall-clock time is the backstop. A run that hits it says "stopped:
   time", is never kept as a result and never fires an alert.
6. A rule that fails or is stopped repeatedly is paused and the admin is
   told. Runs per admin per hour are limited.
7. The sandbox starts for each run and exits; no warm pool. If start-up
   is too slow, a preview runs when the admin presses Run or pauses
   typing, never on every keystroke.

### Keeping rules

- Stored in PostgreSQL per site, with row-level security. Saves are new
  versions only, each with a note. Delete goes to trash for 90 days.
  Undo history follows the platform rule (180 days).
- Each rule names an interface version. Released versions keep working;
  a change to the interface is a new version.
- Each run writes a `job_run` record: rule version, interface version,
  engine version, input digest, limits, output digest and outcome.
- Saving a rule needs the permission "Write chart rules", off for
  moderators. Anyone who may view the chart may see a rule's output.
- A rule is plain text. An admin may export it to a file and import one
  by hand; imports are rechecked and never trusted. Sutradhar sends
  rules nowhere. No library, no ratings, no hidden source.
- One generic catalogue entry covers "admin rule output".

### Left out on purpose

A rule compiler, language server or rule library; a second chart
library; sums across series; rival series, competitor snapshots and
reverse content gaps (rivals are never crawled for tracking); a
fleet-wide turbulence index or "alpha against the market"; a community
library, marketplace or hidden rules; strategies, backtests and
recommendations sent to a change queue; candlesticks and intraday
detail; drawing tools, bar replay, linked multi-chart workspaces and a
window-side data cache; signed webhooks and push digests; a matcher for
thousands of standing alerts; crawl rate and publishing velocity as
charted numbers. Rank tracking is not assumed.

### Settings

Every setting has a safe floor and top, and a setting may loosen a limit
only up to the researched safe top. There is no off switch for the
honest rules, the rule limits as a group, or the "no network" rule. Each
indicator also has its own window setting.

| Setting | Default | Allowed range |
| --- | --- | --- |
| `quarry.script.max_steps` | set by research | above zero, safe top |
| `quarry.script.max_seconds` | set by research | safe floor and top |
| `quarry.script.memory_limit` | set by research | fixed share of the slice |
| `quarry.script.max_source_bytes` | set by research | above zero, safe top |
| `quarry.script.max_output_points` | set by research | above zero, safe top |
| `quarry.script.indicator_calls` | set by research | above zero, safe top |
| `quarry.script.runs_per_admin_hour` | set by research | above zero, safe top |
| `quarry.script.pause_after_failures` | set by research | safe floor and top |
| `quarry.filter.universe_cap` | set by research | above zero, safe top |
| `quarry.filter.page_rows` | set by research | safe floor and top |
| `quarry.filter.memory_share` | set by research | fixed share of the slice |
| `quarry.alert.rules_per_site` | set by research | above zero, safe top |
| `quarry.alert.repeat_gap_hours` | set by research | safe floor and top |
| `quarry.chart.min_impressions` | set by research | at least darshak's floor |
| `quarry.chart.min_measured_share` | set by research | safe floor and top |
| `quarry.chart.max_series` | set by research | above zero, safe top |
| `quarry.chart.max_points` | set by research | above zero, safe top |
| `quarry.chart.compare_days` | set by research | safe floor and top |
| `quarry.run.keep_days` | set by research | follows the undo history rule |

### Research before the build

GanitEvolve, on real history and made-up data with planted moves: which
indicators earn a place (false alarms against misses, against a plain
average), which families are set aside, windows per indicator; how many
measured days an indicator needs and what share of gaps spoils it; what
"usual" should mean, with a robust spread, and repeat gaps that keep
false alarms low; how late Search Console days arrive, whether copied
days are revised, and so when a day is closed; typical steps and memory
of useful rules, sandbox start cost on Dell, whether step counts are
identical across machines and versions, fuzzing of the pre-check and
output checker, and whether ganit calls are fast enough; filter sizes by
site size and what the first scan can read in its share; a thinning
method that keeps highs and lows; points and series per chart.

### Changes to other parts

- Visual layer: a Search charts screen and its data rows; no new
  library.
- Metric catalogue: entries for each series and indicator, plus one for
  admin rule output; connected-service and backlink figures as one
  labelled source.
- Safe running: `karya.shed_order` gains `chart_filter` and
  `chart_script`, after `test_room`. `live` is never listed.
- Extension isolation: the sandbox allow-list accepts a first-party rule
  sandbox with no network.
- Settings register: the `quarry.*` keys.
- Grouped alerts: a "chart alert" notice kind and a "rule could not be
  checked" fault.
- Memory record: rows for the parts below, once measured.
- Site structure checks: the falling test is callable by name, with no
  change in behaviour.
- The manual gains a Search charts page for site owners.

### Logic and maths

Decisions are Rust, proved with Kani where they decide: day-state and
polarity rules, honest-chart checks, whether an alert fires, merges or
waits, the rule pre-check and output checker, run admission against
limits, and "too many to test". All maths is in ganit, one definition
per indicator. A faster version may be added only with ganit's version
as its safe twin and a test that they agree. No PHP change, as guests
see nothing.

## Limits

Everything counts inside AGENTS.md 5.6: the service parts in the service slice,
the chart window in the window's cap, no swap.

| Part | What it adds | Guard |
| --- | --- | --- |
| Rule sandbox | Peak while a run lasts, none when idle | Sandbox memory limit |
| Filter run | A scan of many daily rows | Paged, one at a time, a share |
| Indicator calls | More work for ganit | Calls per run capped |
| Chart data | Small snapshots from the service | The visual layer's 1 MB and 2 MB caps |
| Alert checks | A small pass after each pull | Per-site rule cap |
| Records | Disk: runs, versions | Kept for set days |

Risks, biggest first:

1. The filter scan. It reads in pages inside the database, keeps no full
   copy, runs one at a time at low priority, asks the promise check
   before any large request, and is shed first. A big filter is slow,
   not dangerous.
2. The rule sandbox. Its memory limit stops one run ("stopped: used too
   much memory"), not the service. Worst case to test: the limit times
   the runs allowed together.
3. Everything at the same hour. Pulls, saved filters and alert checks
   run in karya's order with spread start times; alert checks come after
   pulls.
4. ganit load. Calls per run are capped and answered from ganit's cache
   for the same series and window where it holds one.
5. The window. A cap on points and series per chart keeps drawing light.

Not added: no headless browser, second chart server, language server,
code-editor package, warm worker pool or new number store.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Rust tests, with SQL tests for grants and row-level security:

- QV01: a day still arriving is drawn apart and never used by an alert,
  filter, indicator or rule.
- QV02: a day not measured is a gap in chart and table, and never zero in
  any indicator or rule sum.
- QV03: no estimate band appears without a ganit result; when shown it
  is labelled and never inside a line of copied numbers.
- QV04: two sources are never summed; a ratio across sources is refused.
- QV05: on a position series every indicator, alert, filter and rule
  says "better" or "worse" correctly; a sign test covers each indicator.
- QV06: too few impressions gives that message and no position line.
- QV07: darshak counts under 10 show "fewer than 10".
- QV08: no weekly or monthly bar is drawn for a part period.
- QV09: a series or indicator with no catalogue entry is not drawn.
- QV10: chart, alert, filter and rule give one value for one indicator.
- QV11: no alert fires on a move into or out of not measured, except
  "data went missing".
- QV12: "test on history" sends nothing.
- QV13: notices merge with rows kept; a repeat updates one notice; the
  repeat gap holds; quiet hours make one digest; a fault still gets
  through.
- QV14: a failing rule gives one fault notice and no data alert.
- QV15 (SQL): the alert code and the rule engine hold no write grant on
  any page, post, link, setting or ranking table.
- QV16: a rule test over too many series is refused with the count; no
  subset is tested silently.
- QV17: rows with too little evidence are listed apart and never ranked.
- QV18: a saved filter shows newly and no longer matching, with dates,
  and is shed first under pressure.
- QV19: in the sandbox, a network call, file read or write, clock read
  and random number each fail.
- QV20: hostile rules (endless loop, deep nesting, memory balloon, huge
  output, nested lists, many indicator calls) stop inside their limits;
  the screen stays live, other admins are unaffected, memory stays in
  its share.
- QV21: the same rule and input give the same output and stop at the
  same step. A run stopped by the time backstop is never kept and never
  alerts.
- QV22: a rule sees only its declared series, never search words,
  addresses or member text; an undeclared series is absent.
- QV23: output of the wrong shape is refused whole, with the reason.
- QV24: no rule output reaches ranking, linking, the Coach or a ganit
  input.
- QV25: saves are new versions; delete goes to trash for 90 days; every
  run has a complete record.
- QV26: with two sites, one site's rules, views, alerts, filters and
  runs never appear for the other.
- QV27: guests and members see nothing; staff without the permission
  cannot save a rule.
- QV28: the table loads before the picture; keyboard use, high contrast
  and reduced motion work; every screen state shows in words.
- QV29: a saved view reopens "as of" its date, with a switch to now.
- QV30: no rival, link or crawl number appears until the owner extends
  the source list.
- QV31: with chart, rule and filter at their shares and all else
  running, memory stays inside the service slice and the window's cap.
- QV32: if the rule engine is switched off or fails its checks, screens
  say "rules unavailable" and the menus still work.
- QV33: event bands merge and never drop; none uses a cause word.
- QV34: an imported rule is rechecked first; Sutradhar sends none
  anywhere.

The pre-check and the output checker are fuzzed.

## Changes from the source

- The user script language is replaced by declarative TOML rules run by
  a Rust rule engine, with the same jobs, limits and safety layers; its
  crate admission record is no longer needed.
- Decision logic and adapters are rewritten as Rust, proved with Kani;
  maths uses ganit.
- Admin pages become screens in the Dioxus app built from SDS; charts
  come from the visual layer.
- Memory follows the limits in AGENTS.md section 5.6, not a flat 8 GB
  with 15% headroom.
- The source's speed claims are not carried; speed follows the 5%
  median rule.

Source: `docs/specifications/signals/quarryview.md`,
`docs/specifications/signals/quarryview-2.md`,
`docs/specifications/signals/quarryview-3.md`,
`docs/specifications/signals/quarryview-4.md` and
`docs/specifications/signals/quarryview-5.md` in the archive
`archive-2026-11-08-old-repo`.
