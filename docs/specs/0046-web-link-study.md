# 0046: Web link study

The link study answers one question for a site owner: how good are the
links to my site, and how sure are we? It reads link records and the
reports of services the owner connects, and fetches nothing itself.
Every number says what kind it is. The screen label is "Link study"; the
module is `subramanian`. It is part of roadmap Phase 6; "Which spec
designs each feature" in [features.md](features.md) lists the features
it designs in whole or in part.

Example: "Service A reports 412 pages linking here (service figure, read
2 Nov). Our crawl has checked 57. 3 links: not seen since 14 Oct."

## What it does

### Ground rules

1. It only reads and reports. It never inserts, edits or removes a link,
   so moderator approval and member text are untouched.
2. Staff-only screens in the Dioxus app; guests see nothing new.
3. Each site and each agency client keeps its own records. Nothing is
   shared between installs.
4. It starts on. With no source it shows "not measured", never zero.
5. The study code opens no network connection. Only the crawler and the
   service adapters do.

### What it reuses

| Part | Reuses |
| --- | --- |
| Link records, leads, block pages | Backlink crawl (spec 0045) |
| "Not seen since", "not measured", no mixing | Backlink crawl rules |
| Service keys, quotas, health | Connectors (spec 0040) |
| Own page facts; visit totals | Crawler (spec 0044); darshak (spec 0034) |
| Before and after tests | ganit jobs 298, 299, 308; Coach items 20, 21 |
| Survival of links | ganit jobs 309, 310; feedback learning (spec 0053) |
| "Not enough evidence" wording | Editorial Coach (spec 0041) |
| Research; settings | GanitEvolve (spec 0012); settings (spec 0014) |

New in this part: one label rule and a refusal to compare unlike
numbers; link states with a reason; six reports; a "needs more evidence"
box; connectors for DataForSEO, Semrush and Common Crawl; made-up test
worlds.

Sources: crawl records; services the owner connects with their own key;
Search Console and Bing link exports; pasted pages. What each service
lets Sutradhar store is set by research.

### Labels on every number

Each figure carries a kind, source, date, sample size and range.

| Kind | Meaning |
| --- | --- |
| Seen | Our crawl read the page and found the link |
| Own data | The owner's own Search Console, GA4 or darshak |
| Service figure | A connected service's number, named and dated |
| Modelled | Our calculation, with a range and its inputs |
| Not measured | No source, or the data cannot support it |

1. Kinds and sources are never added together. Two services sit side by
   side.
2. Numbers compare only if source, dates and settings match. Otherwise
   the screen says "do not compare", gives the reason and shows no
   difference.
3. Other answers: "not enough evidence", "out of date", "sources
   disagree". No vendor score is copied or implied.
4. Source names come from what the administrator connected. The "no
   brand names" rule allows services the owner connects, and legal
   credits.
5. A connected service's numbers stay labelled and never feed Sutradhar's
   modelled ones.

### Link states

A link is never called lost, removed or "probably gone".

| State | Shown when |
| --- | --- |
| Seen | The last read found the link |
| Not seen since (date), page read | Page read, link absent |
| Not seen since (date), not read | Page could not be read |
| Page gone | 404 or 410: the page went, not the link |
| Back again | Seen after a gap; both dates kept |

1. Timeouts, 429, 5xx, DNS errors, robots denial, 401 to 403, block pages
   and a paused crawl never change "seen". They show "due for recheck".
2. A service dropping a link shows "service no longer lists it".
3. Dates kept: first seen, last seen, last checked, last read, and the
   reason a link was not seen.
4. Rechecks use the backlink crawl's lead list (kind 3), ordered by
   change rate and link value (ordering set by research).

### The six reports

1. Links to my pages: new links, not seen since, referring sites, link
   words as kept, and growth. Growth marks each date the checking
   changed, so more looking is not shown as more links.
2. Links to dead pages, so a redirect can be set. Advice only.
3. Link value: visits from each linking site, from darshak totals and
   imported visitor numbers. Too few visits shows "too few".
4. Survival: how long inserted links lasted (ganit job 310). Outside
   links get a range only after enough history; otherwise "not enough
   history".
5. Before and after: matched untouched pages beside changed ones (ganit
   jobs 298, 299, 308). If they moved alike: "correlation, not cause".
6. Rival overlap: sites a connected service says link to a named rival
   and not to the owner's site. A reading list only, never an outreach
   sender. A rival's pages are never fetched, except as a lead for a link
   to a registered site.

### Evidence box

1. A coverage estimate from two connected sources' overlap is research
   only. It stays "not measured" until it passes on made-up worlds with
   known truth; if it never passes, it never ships.
2. A report with too little evidence says what would help: start a
   recheck run, connect a service, or paste pages. A click adds leads to
   the crawl's list or opens the connector. Repeat requests merge. An
   evidence request fetches nothing itself.

### Services that disconnect

A disconnected service's figures are marked "no longer connected" and
kept where its terms allow, and deleted where they do not.

### Settings

| Setting | Default | Allowed range |
| --- | --- | --- |
| `link.study.fresh_days` | set by research | safe bottom and top |
| `link.study.min_sample` | set by research | above zero, safe top |
| `link.study.compare_window_days` | set by research | safe bottom and top |
| `link.study.survival_min_days` | set by research | safe bottom and top |
| `link.study.referral_min_visits` | set by research | at least darshak's floor |
| `link.study.service_calls_per_day` | set by research | above zero, quota |
| `link.study.service_keep_days` | set by research | follows service terms |
| `link.study.memory_share` | set by research | fixed share of the service slice |

Researched values are fixed before the build. None is hard-coded.

### Research first (GanitEvolve)

1. What sample and age make a survival range better than "not enough
   history"?
2. How many visits make a per-link value stable and private?
3. How many matched pages make a before and after test fair?
4. Which dates make numbers comparable?
5. Do two services overlap enough for a coverage estimate to be right on
   made-up worlds with known truth?
6. Safe call rates, and what each service lets Sutradhar keep.
7. A link strength score: only counted facts ship at first. GanitEvolve
   researches a score, added only if it passes on made-up worlds with
   known truth.

### Changes this part makes to other parts

1. Backlink crawl (spec 0045): records carry `last_checked`, `last_read`
   and a reason a link was not seen.
2. Connectors (spec 0040): the old "no paid backlink service" rule does
   not hold for services the owner connects. Three link-data connectors
   (DataForSEO, Semrush, Common Crawl) are added there.
3. SEO health (spec 0044): backlinks show "not measured" only with no
   source and no run.
4. darshak (spec 0034): keeps a referrer class and also a daily count by
   referring site; the path only if privacy research allows.

### Not part of it

1. No sharing between installs, peers, relays or cohorts learned across
   sites.
2. The crawl only visits leads. No crawl frontier of its own, and no
   other sites' text is kept, only link records.
3. No web graph is held and no reach, trust or rival rank or traffic
   score is shown, unless a score passes research.
4. AI-answer tracking has its own design.
5. No language-model tier, outside message bus, search cluster, graph
   database or extra deployable programs.

## Limits

- Runs inside its fixed memory share (`link.study.memory_share`) of the
  service slice, within the memory limits in AGENTS.md section 5.6, and
  stops first under pressure.
- Service calls are capped per day within each service's quota.
- Service data is kept only as long as the service's terms allow.
- The study itself makes no network calls.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Rust tests on made-up link worlds:

- Every figure shows kind, source, date, sample and range, or "not
  measured". None appears bare. No source and no run shows no zero.
- Different kinds or sources are never added. Unlike numbers are refused
  with a reason and show no difference.
- A missing link shows "not seen since", a date and a reason, never
  "lost" or "removed". A 404 or 410 shows "page gone", apart from the
  link.
- Each error or block in the state list leaves the link "seen" and due
  for recheck. A returning link shows "back again" with both dates.
- The study code opens no connection; only the crawler and adapters do.
- Rival numbers come only from a connected service, named and dated. One
  site's or client's records never appear for another.
- Too little evidence gives "not enough evidence" and no forecast. An
  evidence request fetches nothing, and repeats merge.
- Survival is hidden until the minimum history. Low visits show "too
  few". Matched pages that moved alike give "correlation, not cause".
- Planted-fault worlds: 10,000 links from one site count as one
  referring site; wider checking is not shown as gain; an unreadable page
  is not "lost". The expected answers are fixed first, and changing them
  needs an independent review.
- A disconnected service's figures are marked, then kept or removed by
  rule. Every setting has a safe bottom and top.
- Under memory pressure it stops first and stays inside its share.

## Changes from the source

- Link states, labels, comparisons and hidden figures are decided in
  Rust, with Kani proofs where they fit.
- The maths runs in ganit, the Rust numerical engine.
- Admin screens are a Dioxus app built from SDS components.

Source: `docs/specifications/signals/link-study.md` in the archive
`archive-2026-11-08-old-repo`.
