# 0045: Backlink crawl

The backlink crawl answers one question for a site owner: which other
sites link to my pages? A backlink is a link on someone else's page that
points to your site. Example: a blog post links to your router guide,
and Sutradhar keeps that one link. It is part of roadmap Phase 6; "Which
spec designs each feature" in [features.md](features.md) lists the
features it designs in whole or in part. It widens the crawler of
[0044](0044-seo-signals-crawler.md), and its records are used by the web
link study ([0046](0046-web-link-study.md)), which does no fetching.

## What it does

### One crawler, three kinds of reading

No second crawler. The crawler gains two narrow lists.

| Kind | Reads | Keeps |
| --- | --- | --- |
| Own-site crawl | Registered sites, as in 0044 | Facts, short copies |
| Backlink crawl | Pages likely to link to a registered site | Link records |
| Outside notices | A fixed list of status pages | A note of any change |

The crawler's rule that other people's sites are out of scope is lifted
for these two lists only. The rest of 0044 stands.

### Where backlink data comes from

1. First, the services the owner connects with their own key:
   DataForSEO, Semrush and Common Crawl
   ([0040](0040-search-connectors.md)). Each record names its source.
   Figures from different sources are never added together.
2. The crawl fills gaps. It checks leads; it does not wander the web.
3. With no source connected and no crawl run, the screen says "not
   measured", never zero.

### How it finds pages

It fetches only from a lead list. A lead is a page some evidence says
may link to a registered site:

1. a page a connected service reports;
2. a page in a public web archive's index;
3. a page already known to hold a link, due for a recheck;
4. a public referring page in the site's own approved analytics totals;
5. a page an admin pastes in.

It never follows every link on a page. It follows a link only when the
page is a lead and the link goes to another page on that same site
listing the same target (for example "more resources, page 2").

A person may start a run at any time. A modest background pass works
through the list, as set out under "Starting state".

### What it keeps

One link record, and nothing else from the page:

- the page address and the registered page it points to;
- the link's own words, cut to a researched short limit;
- the link's marks, such as nofollow or sponsored;
- first seen, last seen and the source of the evidence;
- `last_checked`, `last_read` and, for a link not seen, the reason, with
  the meanings set by the link study's states.

The rest of the page is read in memory and thrown away: no page copies,
quotes, images or author names. A link not found on a later visit shows
"not seen since" with a date. It is never called removed, and never
guessed at.

### Paywalls and copyright

- Guest view only: no sign-in, no forms, no cookies kept between pages.
- A page marked as paid, or answering 401, 402 or 403, is left unread and
  recorded as "could not read". Nothing is taken from it.
- Nothing gets past a block. The crawl stops at any block page and tells
  the admin.
- Rival sites are never crawled to track them. Rival data comes only
  from connected services. A rival's page is read only if it is a lead
  for a link to a registered site.
- Only the short link words leave the page.

### Being a good visitor

- The same declared name in every request, with a public page saying
  what the crawler does and how to stop it. It never poses as another
  crawler.
- `robots.txt` is obeyed under that name. A missing answer, an error,
  401 or 403 counts as deny.
- `Retry-After`, status 429, `Crawl-delay` and `cf-mitigated` are
  honoured.
- One request at a time per host, slowly, with researched gaps. The
  own-site speed setting does not apply here.
- Any site that asks goes on a stop list, and stays on it.
- Addresses are checked first: no private addresses, and no redirect to
  one.

### Block pages, on every crawl

If a page is a block or challenge, such as "prove you are human", the
crawler stops on that host and tells the admin. On a registered site it
pauses that whole site's crawl and raises an issue for a person. It
never solves, avoids or retries past a block. A block is reported as a
block, not as an ordinary error or page. This holds for the own-site
crawl too.

### Outside notices

A short fixed list of official status and documentation pages, read a
few times a day, politely. Only the date and a note that the page
changed are kept. Example: "A search update is running, so this week's
dip may not be your fault." Rank-tracking sites are not on the list. A
notice is context, never proof of cause. Search charts show these as
event bands ([0043](0043-search-charts.md)).

### Telling search engines about new pages

The crawler ([0044](0044-seo-signals-crawler.md)) owns this: one plain
list with a daily cap, for own sites and public addresses only.

### Starting state

A person-started run works straight away. The background pass stays off
until the admin gives a contact address and accepts the rules; then it
runs. Off is this design's own default, because the pass fetches pages
from other people's sites in the background.

### Logic and research

Which address may be fetched, the lead rules and retention are Rust,
proved with Kani. Network and page-reading adapters are Rust. Maths is
in ganit. The lead rules go through GanitEvolve research first.

### Settings

| Setting | Default | Allowed range |
| --- | --- | --- |
| `seo.backlink.pages_per_day` | set by research | above zero, safe top |
| `seo.backlink.host_gap_seconds` | set by research | safe floor |
| `seo.backlink.memory_share` | set by research | fixed share of the slice |
| `seo.backlink.bytes_per_hour` | set by research | above zero, safe top |

Researched values, the link-words limit and the published crawler name
are set before the build.

## Limits

- The crawl stays inside its fixed share of the service slice
  (`seo.backlink.memory_share`), within the memory limits in AGENTS.md
  section 5.6.
- It stops first under any pressure, is one heavy job at a time, and
  never slows a live site.
- Pages per day, bytes per hour and the gap between requests to one host
  are capped by the settings above. One request at a time per host.
- Page bodies are never stored; only link records are kept.
- Each site or agency client keeps its own records; nothing is shared
  between installs.
- Legal review: reading pages this way differs by country, and link
  words raise a small copyright question. Both are reviewed before the
  build.

## Footprint budget

This part adds nothing to public pages of registered sites. It reads
other sites' public pages from the owner's PC.

## How it is tested

Rust tests, with fake hosts serving planted pages:

- BL01: a page's body is absent from every store after it is read.
- BL02: a record holds only the listed fields, and link words stay
  within the limit.
- BL03: a sign-in page, a paid page and 401, 402 and 403 are left
  unread.
- BL04: a block page stops that host and tells the admin, with no retry
  past it; on a registered site it pauses that site's crawl and raises
  an issue.
- BL05: a `robots.txt` deny, a robots error and a stop-listed site are
  never fetched.
- BL06: a link missing on a recheck shows "not seen since" with a date.
- BL07: no source and no run shows "not measured".
- BL08: a page that is not a lead is never fetched.
- BL09: the name is the same in every request, and matches the public
  page.
- BL10: the crawl stops under pressure and stays inside its memory
  share.
- BL11: one client's records never appear for another client.
- BL12: figures from different sources are never added together.

## Changes from the source

- Decision logic and adapters are rewritten as Rust, proved with Kani;
  maths uses ganit.
- The memory share is taken from the service slice, not a flat 8 GB.

Source: `docs/specifications/signals/backlink-crawl.md` in the archive
`archive-2026-11-08-old-repo`.
