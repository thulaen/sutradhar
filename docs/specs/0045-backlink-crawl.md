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

1. Use admitted owner-supplied evidence (spec 0046) and permitted
   free public link maps as lead lists (ADR-0022). Each record names
   its source. Figures from different sources stay separate.
2. The crawl fills gaps by checking named leads. Live backlink and
   search-result provider feeds are not built (DR-076 and DR-085).
3. With no evidence or crawl result, show "not measured".

### How it finds pages

It fetches only from a lead list. A lead is a page some evidence says
may link to a registered site:

1. a page in an admitted owner-supplied report;
2. a page in a public web archive's index;
3. a non-rival page known to hold a link and due for a recheck;
4. one protected referrer lead under spec 0120. Use its individually
   approved read and scope. Exclude it from regular lead lists and
   follow-up reads. The protected receipt remains in that scope.
   Generic scheduling, link following, archives, retained snippets,
   detectors and exports cannot ingest or copy it;
5. a page an admin pastes in.

It never follows every link on a page, except in a niche scan, a separate
job of spec 0082 that follows links by niche score. It follows a link only when
the
page is a lead and the link goes to another page on that same site
listing the same target (for example "more resources, page 2").

The desktop owner may confirm an eligible manual run under the current
0152 scope. The separately authorised background pass works through
eligible non-rival leads, as set out under "Starting state".
Reject a rival lead from that pass, including a saved due record
after restart or an alias. Spec 0082 owns its one-off admission.

### What it keeps

One link record, and nothing else from the page:

- the page address and the registered page it points to;
- the link's own words, cut to the admitted short limit, which remains a
  build blocker as the settings contract states;
- the link's marks, such as nofollow or sponsored;
- first seen, last seen and the source of the evidence;
- `last_checked`, `last_read` and, for a link not seen, the reason, with
  the meanings set by the link study's states.

The rest of the page is read in memory and thrown away: no page copies,
images or author names, and no text but the one short scrubbed sentence
of spec 0082. A link not found on a later visit shows
"not seen since" with a date. It is never called removed, and never
guessed at.

### Paywalls and copyright

- Guest view only: no sign-in, no forms, no cookies kept between pages.
- A page marked as paid, or answering 401, 402 or 403, is left unread and
  recorded as "could not read". Nothing is taken from it.
- Nothing gets past a block. The crawl stops at any block page and tells
  the admin.
- A rival's public pages may be read once, in a niche scan or as a lead,
  within spec 0082's legal wall. Rival research has no repeat job.
  Separate comparisons use evidence the owner supplies (DR-085).
- Only a receipt leaves the page: the link words and one short scrubbed
  sentence, as spec 0082 sets out.

### Being a good visitor, and block pages

The declared identity, robots.txt, waits, host gaps, the stop list, address
checks and block-page handling are owned by spec 0082 (Leviathan). This crawl
follows them on every read, own site or outside, and adds nothing of its own.

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

An eligible manual run uses the admitted [0152 live profile][profile]
and the desktop owner's exact confirmation. The background pass stays
off until the site has a current verified contact and the desktop
owner separately confirms an eligible schedule under 0152. Contact
and rule acceptance supply no schedule or run approval. The schedule
covers only its approved set of eligible non-rival leads and dates.
It cannot schedule rival monitoring or read from a live provider.

### Logic and research

Which address may be fetched, the lead rules and retention are Rust,
proved with Kani. Network and page-reading adapters are Rust. Maths is
in ganit. The lead rules go through Ganit Vikas research first.

### Settings

[Spec 0152][profile] owns the proposed defaults, ranges and live
approval for the ordinary backlink route. Use its admitted receipt
and link-text, site, host, shared budget, memory and storage bounds.
Until the signed profile proves each bound and the 0082 identity and
legal conditions under QR-237, show “Crawler not ready”. A manual
plan can be prepared without starting a run.

[profile]: 0152-live-backlink-crawler-profile.md

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
