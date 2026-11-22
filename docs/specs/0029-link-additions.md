# 0029: Link additions

This part adds rules to the links Sutradhar suggests and how they are
ordered, more ways for moderators to approve them, link help while a writer
drafts, a member-text option, and a few small imports, limits and flags.
It builds on staff suggestions (spec 0028) and the ranking rules, and adds
no new scorer, crawler, queue, matcher or approval screen. It is roadmap
Phase 4 item 3. Every link is still approved by a moderator.

[Spec 0119](0119-cross-site-link-suggestions.md) owns sister-site
suggestions (65). Its editor uses the source site's native link rules.

## What it does

### 1. Sutradhar never marks its own links

- A link Sutradhar places carries no `nofollow`, `sponsored` or `ugc`
  mark, and Sutradhar never asks XenForo or WordPress to add one. The links
  are editorial, approved by a moderator and inside the site.
- There is no setting. It covers links placed by suggestions, hubs and the
  Editorial Coach.
- Marks that the platform or a theme adds by its own rules are left alone.
  The record shows them as seen, never changed. The platform-integration
  research record must establish whether XenForo or WordPress marks an
  internal link itself; until admitted for the exact pinned platform, the
  add-on assumes no automatic marking and must not invent a markup change.
- The backlink crawl still reads other sites' marks as data.

### 2. Pages with few links in: a small lift and a report

- Need (ganit job 324) already rises when a page has few inbound links,
  with falling returns (job 49). No new scorer is added.
- A ceiling on need's part of the order key keeps the lift small. Need
  still reorders only candidates that passed the relevance gate.
- A "few links in" tag shows on the suggestion card.
- A report, a filter on the site's link counts and click-depth list,
  shows page, links in, click depth and "suggestions waiting". It is
  advice only. The share defining "few" is a readiness blocker owned by the
  link-report evaluation record; the tag and filter cannot activate until its
  value and fixtures are admitted. With no link data or no admitted share it
  says "not measured", never zero.

### 3. A hub and its own threads may link both ways

- The reciprocity rule (rule 8) refuses a pair when the destination
  already links back and neither has a third inbound link. It gains one
  exception: a hub and a thread of that hub (its one primary hub) may link
  each way.
- Both links still pass the gate, rules 1 to 7 and a moderator's approval.
- Thread-to-thread pairs stay refused. Loops of two or three threads stay
  refused unless every step is a hub pair.
- The hub list sets the exception, never Sutradhar's own guess.

### 4. A page's sales value may reorder slightly

- Staff mark a page, section or tag "leads to sales": none, some or
  strong. It is a label set by a person. No money, orders, prices or
  customers are stored anywhere.
- It acts only in the order stage, after the gate. It may swap two
  candidates whose relevance is within `linking.sales.margin`. That margin
  is blocked as the settings table states, so this ordering stays off until
  admitted. It never lifts one over a failed gate or beats a clearly more
  relevant one.
- The card says "ordered slightly by sales label" when it mattered.
- The site-structure checks read no sales field; the label belongs to
  ranking. A business-weighted mode does not exist.

### 5. Site owners may write reject-only link rules

- A form, not a formula language: pick a kind (section, tag, page, address
  pattern, anchor word), choose "never link to" or "never link from", and
  give a value. Example: "never link into the Sponsored section".
- Rules join `equity::check` as extra refusals with the same reason
  display, at suggestion time and again at approval. One function, two
  callers.
- A rule can only refuse. It cannot add a candidate, raise a score or relax
  a built-in rule or the gate.
- Each rule shows how many suggestions it has refused. Edits are versioned
  with one-step undo. Rules are per site and per agency client.

### 6. Tri-Rank is tested in the lab as a rival

- The idea: score a link by how much link strength a page receives, how
  close in meaning the two pages are, and whether the link moves a reader
  one step toward acting.
- It is a sealed challenger to the chosen ranker. Spec 0077 alone compares
  a complete challenger with the verified champion and promotes it; only the
  promoted champion is on. Passing research does not create a selectable
  production alternative, and a failed challenger is not promoted.
- No fixed damping or spread numbers, fixed preset weights, four-stage buying
  funnel or checkout sink rule is invented. Every required value is a
  readiness blocker owned by the Tri-Rank research record; the challenger
  cannot train or evaluate until each value and its evidence are admitted.
  Brand-name terms stay off screens.
- It is written fresh from this description, as ideas only.

### 7. More visitor signals in link ranking

- Only darshak aggregates are used, and consent rules stay.
- Every signal is a rate with its visit count for shrinkage, from human
  visits only, frozen when the bot share trips. Counts are refused as
  signals.
- In use: engaged-visit rate, click-through on added links, sign-up rate;
  the hub-pair rate comes with the Coach additions.
- Onward-click rate, reading-depth rate and search-entry share are sealed
  candidates, each blocked until Ganit Vikas admits its usefulness, privacy
  and weight evidence and spec 0077 promotes a complete champion containing
  it. Only fields darshak already records may be used; a new field needs the
  privacy page revised first.

### 8. Approving a viewed list in safe-sized batches

- A suggestion can be ticked only after it has been on screen with its
  sentence, anchor and destination. "Tick all" is not offered.
- Each item is rechecked on its own at approval. One that fails (stale,
  moved, over a cap) is skipped with its reason; the rest go on.
- A batch is idempotent and logged as separate approvals. Undo is per link.
- Batch size has a safe bottom and top. The existing cap of 50 is the
  conservative admitted maximum. The batch-evaluation record must judge review
  time per item (job 50), later-discovered wrong approvals and undo load before
  any change. A lower value uses the ordinary confirmed settings route;
  raising the maximum additionally needs an owner decision. No automatic
  increase is permitted.
- Reject-only rules and the "never marked" rule apply to each item.

### 9. Approving a link for a set date

- The moderator picks a date; the link is placed then, not now.
- To schedule, the full recheck must pass now: post unchanged at the
  anchor, destination guest-visible, caps and rules clear.
- On the date the whole recheck runs again, with the equity caps counted
  that day, and the moderator's permission checked again. If any test
  fails, nothing is written, the card returns to review with the reason,
  and the moderator is told. It is never retried in silence.
- The date can be cancelled or moved until it runs. A past date is
  refused. `suggest.schedule.max_days` is the blocked furthest-date bound;
  dated approval cannot build or run until the scheduling research record
  admits its value and fixtures.
- The audit trail names who approved and who it ran for. Placement and
  undo are unchanged. It uses the job queue and the load governor and
  yields to live work.

### 10. Link suggestions while drafting

- A run starts only when the writer presses "Suggest links" or when the
  platform itself saves the draft. Never from a key press or a timer.
- The add-on sends the saved draft once, with a minimum gap per draft and
  a daily cap per member. It sends nothing while typing.
- The writer sees proposed anchors and destinations beside the draft, read
  only. Sutradhar puts nothing into the draft.
- After the post is saved, the usual pipeline runs again on the real post,
  drops any phrase that is gone and queues the rest. A moderator approves
  each with the normal recheck. The card shows "asked for while drafting".
- Only the author and moderators see them, as the platform's visibility
  allows. Guests never do. Other writers' drafts are never compared.
- This differs from typing-time checks, which the Coach does not do.

### 11. The small hold-back test of links

The aim is to learn whether suggested links help, by leaving a few pages
without new links for a while and comparing them with similar pages that
got them.

- Off by default: this design starts it off, because it holds links
  back from real pages. It runs only once a site's owner agrees, per
  site only.
- The cost is shown in plain words before the switch and while it runs:
  how many suggestions wait, for how many days, and the link benefit
  missed, as a range with its uncertainty, never a promise.
- `linking.holdback.share` and `linking.holdback.days` are readiness blockers
  with safe tops, owned by the hold-back evaluation record. The test cannot
  build or start until both are admitted. A page is eligible only when the
  test is fair (enough similar pages and traffic); otherwise it says "not
  enough data".
- A held page shows "held for test" on its cards, so nothing is hidden from
  a moderator. A moderator may release a page at once; that ends its part
  and is counted.
- Groups are chosen at random and logged so the result can be checked. The
  analysis is a ganit batch run shown only to staff. Search Console totals
  from the connector supply the outcome; figures from different sources
  are never added together.
- Never used on pages with a safety, legal or breakage flag.
- Research must answer: the smallest share that can show a gain, the
  length, the cost estimate and what the test cannot tell.

### 12. The member-text option

On by default. A moderator approves each change. AI never rewords a post.

- What it allows, each with its own approval:
  - a spelling fix to a word of five letters or more;
  - a link on a misspelt word, offered as the link alone or link plus fix;
    with the option off, it is refused with that reason;
  - a reworded link text, such as "power supply unit" to "PSU", changing
    only the link's own words.
- The placement checks stay: nothing inside quotes, code, spoilers,
  existing links or mentions. A name labelled as a person is never fixed.
- The recheck runs at apply time. Undo keeps any later edit by the author.
  Each change is a before-and-after pair in the journal, and is one step in
  the undo list. The thread is not bumped and nobody is alerted. The XenForo
  integration record must establish how a moderator edit is recorded before
  member-text changes can build; missing evidence blocks that integration
  rather than permitting an audit-trail guess.
- Words come only from the site's own words (titles, tags, glossary and
  abbreviations it already uses, found by the language layer) or from a
  moderator typing them. Never from a language model, MCP agent, paid
  service or local model. The moderator picks or types; Sutradhar never
  chooses.
- The language layer is shodhan's close-spelling matching (it matches,
  never corrects), keyphrases, a name finder and a word tagger chosen in
  the lab, plus a spelling candidate list, an abbreviation pairing and
  word types. It is clean room: built only from published papers and the
  owner's own text, with no copied code or models. That line covers the
  rules and word lists; admitted share-alike data may teach its models
  (the owner, 10 November 2026: "Yes, for teaching models"; spec 0026,
  "Clean room, rights and data"). The old toolkit's name
  stays off screens. A fuller language-layer design may replace this set
  once the owner approves it.
- Research, through Ganit Vikas under the one proof standard, must answer
  which tasks the option needs, which public methods, the wrong-suggestion
  rate by language, and the clean-room proof. Passing rivals remain sealed
  candidates; spec 0077 alone promotes a complete champion. Moderator
  acceptance feeds the bounded tuning (spec 0053).

### 13. Upload of the "crawled, not indexed" list

- A person downloads the list from Search Console and uploads it.
  Sutradhar does not call Google.
- Only addresses on a registered site are kept, with the report date and
  "from upload". Others are dropped and their count shown.
- Size and row limits have safe tops. A wrong file gets a plain reason.
- It shows as a tag on pages and in the few-links report. It is advice
  and changes no score.

### 14. Imports of visitor numbers

- Search Console, GA4, Matomo, Plausible and Fathom files. No raw server
  logs, which hold visitor addresses.
- Totals only. A file with address, user or device columns is refused
  whole.
- Each import is its own source with tool, range and upload date. It is
  never added to darshak's counts or another source. Missing data shows
  "not measured".
- Used in reports and the Coach's evidence, never in ranking. Per site and
  per client. Each tool-and-version file layout, schema, size bound and parser
  fixture is a readiness blocker owned by the import-integration record; that
  importer is refused until admitted, never parsed by guessing columns.

#### Wave 2 supplied visitor-file bindings

REQ-0029-W2-SCOPED-VISITOR-FILES uses the existing totals-only rule
and spec 0100 SD-17/18, UT-SD-15/16 and FI-SD-03 for bounded
staging and atomic intake. Bind each declared tool, version, range,
source type, approved site and client. Reject a file with address,
user or device columns or an unadmitted layout. No
combined source total, ranking input or inherited provider access.
QR-143 retains source-profile approval.

| Feature | Design and planned fixture |
|---|---|
| F-0484 | DES-0029-W2-OFFLINE-MATOMO-FILE · FX-0029-W2-OFFLINE-MATOMO-FILE · IT-0029-W2-OFFLINE-MATOMO-FILE. Compare a protected admissible totals file, a wrong-site or non-totals file and a direct credential offered to file intake. Refuse the latter cases and keep the valid data source-labelled and offline. Use IT-0136-READ and IT-0136-SCOPE separately for the DR-120 timeline-only read and refusal of reuse by other reports, MCP, Coach or ranking. A local file grants no 0136 authority. |
| F-0485 | DES-0029-W2-OFFLINE-PLAUSIBLE-FATHOM · FX-0029-W2-OFFLINE-PLAUSIBLE-FATHOM · IT-0029-W2-OFFLINE-PLAUSIBLE-FATHOM. Use independent Plausible and Fathom totals samples from their respective admitted layouts, with different dates and source labels. Reject an unadmitted schema, wrong site and provider token. Keep numbers separate and missing dates unknown. No direct connection or refresh. |

### 15. Site limits may be loosened, up to a safe maximum

- Platform limits are defaults. A site may move a limit within its own
  bounds, never past a safe maximum measured on Dell, and never to zero.
- Moving toward the maximum shows what could go wrong, in plain words, and
  how far from the top it is. One-step undo shows the old value first.
- The memory limits in AGENTS.md section 5.6, the 15% headroom of spec
  0024 and the crawler's 1 to 8 pages a second cannot be loosened. No
  site setting overrides the load governor.

### 16. Harassment flags

- A hint in the moderator hint queue, with its evidence, like alt-account
  hints. It never warns, hides, locks or restricts anyone. One signal alone
  is not enough.
- Flags about a member are kept apart from their public profile and
  covered by the privacy page. The member is not told. A moderator can
  dismiss a flag, and dismissals tune the weights within bounds.
- On by default, with an off switch.
- Research must answer which signals count, false alarms by language, and
  whether XenForo's report centre can carry the flag safely.

### Settings

| Setting | Default | Allowed range |
| --- | --- | --- |
| `linking.fewlinks.boost_cap` | blocked: no admitted value | above zero, safe top |
| `linking.fewlinks.share` | blocked: no admitted value | above zero, below one |
| `linking.hubpair.enabled` | off: prerequisite values blocked | on only after admission |
| `linking.sales.margin` | blocked: no admitted value | above zero, safe top |
| `linking.sales.enabled` | off: prerequisite margin blocked | on only after admission |
| `linking.owner_rules.max` | blocked: no admitted value | safe bottom and top |
| `ranking.darshak.<signal>.weight` | blocked: no admitted value | above zero, safe top |
| `suggest.batch.max` | 50 | 1 to 50; higher needs owner approval |
| `suggest.schedule.max_days` | blocked: no admitted value | above zero, safe top |
| `suggest.draft.enabled` | off: prerequisite limits blocked | on only after admission |
| `suggest.draft.min_gap_seconds` | blocked: no admitted value | safe bottom |
| `suggest.draft.per_member_day` | blocked: no admitted value | above zero, safe top |
| `linking.holdback.enabled` | off | off until the site owner agrees and values are admitted |
| `linking.holdback.share` | blocked: no admitted value | above zero, safe top |
| `linking.holdback.days` | blocked: no admitted value | above zero, safe top |
| `text.member_text.enabled` | on | on or off |
| `text.member_text.min_word_letters` | 5 | admitted safe bottom and top |
| `import.crawled_list.max_rows` | blocked: no admitted value | above zero, safe top |
| `import.visitors.max_bytes` | blocked: no admitted value | above zero, safe top |
| `moderation.harassment.enabled` | off: prerequisite threshold blocked | on only after admission |
| `moderation.harassment.min_signals` | blocked: no admitted value | two or more, safe top |

Every "blocked" value is a readiness blocker owned by this feature's
research card and Ganit Vikas evidence record. The affected setting and
every behaviour depending on it cannot build or activate until the
record supplies the value, safe bounds, evidence and approval. There is
no implicit default or guessed fallback. Every signal and the few-links
lift also has its own off switch.

### Not offered

Sponsored or nofollow marks on our links; a revenue or offer score; a
formula language for owner rules; fixed Tri-Rank constants and presets;
popularity counts as signals; any signal that follows one visitor across
visits; tick-all approval; unlimited batches; dated links with no second
recheck; drafting runs from key presses or timers; inserting into a draft;
hold-back on by default; a hidden control group; AI rewording of posts;
auto-approved text changes; copied toolkit code or models; raw server log
imports; any limit with no top.

## Limits

- Every setting above has a safe top, and no limit can be zero.
- Drafting runs have a minimum gap per draft and a daily cap per member.
- Batches have the admitted top of 50. Dated approvals use only an admitted
  `suggest.schedule.max_days`, which remains blocked as stated above.
- Uploads and imports have row and byte tops.
- The memory limits in AGENTS.md section 5.6, the 15% headroom of spec
  0024 and the crawler's 1 to 8 pages a second cannot be loosened by
  any setting.
- Dated approvals and the hold-back analysis run through the job queue and
  the load governor and yield to live work.

## Footprint budget

The "Suggest links" button in the site's editor sends one request only on
a press or a platform draft save, and none while typing. Its time, query and
byte budgets are readiness blockers owned by the editor-footprint research
record; the drafting integration cannot build or run until measured bounds
and fixtures are admitted. The other parts add nothing to public pages.

## Wave 1 link review bindings

Use the current exact same-site source and target, eligible native
context, revision and ordinary permission rules. These are review
proposals. Any separately admitted native effect keeps its owning
moderator approval, recheck, footprint and receipt.

| Feature and design binding | Exact planned fixture and expected boundary |
|---|---|
| F-0057 · REQ-0029-W1-SECTION-PROPOSAL · DES-0029-W1-NATIVE-SECTION-EVIDENCE | FX-0029-W1-SECTION-PROPOSAL, BDD-0029-W1-SECTION-PROPOSAL. Compare a clear eligible section, ambiguous topic and inaccessible section. Show the current topic and native section evidence or hold. The 0085 moderation boundaries still apply. No move, redirect, actor substitution or native notice. |
| F-0058 · REQ-0029-W1-QUERY-ANCHOR · DES-0029-W1-CURRENT-ANCHOR-EVIDENCE | FX-0029-W1-QUERY-ANCHOR, IT-0029-W1-QUERY-ANCHOR. Compare a natural eligible on-page span with proven 0040 support, a heavily used exact-match phrase and suppressed query data. Only an exact 0023 span can use admitted support subject to F-0005 and 0027 checks. Suppressed or unproved terms give no lift and cannot be disclosed. Reuse the existing anchor guard and approval checks. No replacement wording or new rank input. |
| F-0059 · REQ-0029-W1-HUB-DIRECTIONS · DES-0029-W1-HUB-EVIDENCE | FX-0029-W1-HUB-DIRECTIONS, shared 0029 test 3 and 0052 hub membership and link review cases. Compare one missing outbound link, one missing return link and a removed member. Use the exact current primary hub and eligible member relation. Each direction has its own review and effect. No inferred membership or reciprocity exception for an ordinary thread pair. |
| F-0060 · REQ-0029-W1-BODY-PLACEMENT · DES-0029-W1-CURRENT-ANCHOR-EVIDENCE | FX-0029-W1-BODY-PLACEMENT, BDD-0029-W1-BODY-PLACEMENT. Compare equally relevant eligible main-text and closing spans, a quoted span and a changed revision. Prefer the eligible main-text span near the supporting passage, within existing 0023/0027 checks. Quotes and stale spans do not qualify. Show the exact span and keep the 0028 moderator's right to decline. |

One planned fixture per feature preserves the existing scope and
test owners. These bindings give no new hook, scorer, provider
request or effect permission.

### F-0061 to F-0068

Bind the active site, current actor, exact source and target
revisions and evidence coverage. Keep proposals in ordinary
review. The records below add no native, public, capture or
outreach route. A supported change still uses its separate owner,
approval, exact profile and effect receipt.

| Feature and owning design | Exact fixture and planned boundary |
|---|---|
| F-0061 · REQ-0029-W1-DENSITY · DES-0029-W1-DENSITY-REVIEW | FX-0029-W1-DENSITY, IT-0029-W1-DENSITY. Run the declared post and thread caps just below, at and above their existing bounds and race two eligible approvals. Reuse 0027 equity and the 0028 approval and equity-race checks. Show density advice only for supported readable content. Recheck placed and pending links before any admitted apply so concurrent approval cannot exceed the cap. |
| F-0062 · REQ-0029-W1-NOFOLLOW-AUDIT · DES-0029-W1-NATIVE-MARK-EVIDENCE | FX-0029-W1-NOFOLLOW, BDD-0029-W1-NOFOLLOW. Compare an observed internal nofollow mark, a mark justified by its native context and a page whose rendered state is unavailable. Use read-only 0047 evidence and the exact admitted platform rule. Report supported mismatch or unknown. Preserve platform marks and refuse automatic edits. |
| F-0063 · REQ-0029-W1-NAVIGATION-ADVICE · DES-0029-W1-NATIVE-PLACEMENT-EVIDENCE | FX-0029-W1-NAVIGATION, BDD-0029-W1-NAVIGATION. Compare a section with approved comparable 0040 demand, an exact admitted 0047 observed menu lacking it, an unread menu and a menu that already lists it. Report the supported omission or unknown. Suggest a supported native placement without changing menus, navigation or theme. |
| F-0064 · REQ-0029-W1-STICKY-REVIEW · DES-0029-W1-NATIVE-PLACEMENT-EVIDENCE | FX-0029-W1-STICKY, BDD-0029-W1-STICKY. Compare a useful pin, a disproportionately visible pin and missing reader evidence against a current eligible replacement. Bind compatible 0047 link and admitted reader evidence for each and show uncertainties. Hold a rotation suggestion when the comparison is unproved. No pin, unpin or weight change. |
| F-0066 · REQ-0029-W1-WOO-BRIDGE · DES-0029-W1-WOO-DIRECTION | FX-0029-W1-WOO-BRIDGE, IT-0029-W1-WOO-BRIDGE; reuse 0103 SH-12 and UT-SH-11 for stock and target admission. Independently compare forum-to-product and product-to-forum in the same WordPress site. Require an admitted forum adapter, exact eligible product identity with current confirmed stock, guest-visible relevant discussion and current source moderation. Other sites, lost stock or visibility and missing adapter hold. Each direction requires separate native effect and footprint evidence. |
| F-0067 · REQ-0029-W1-RESOURCE-READ · DES-0029-W1-RESOURCE-EVIDENCE | FX-0029-W1-RESOURCE-READ, IT-0029-W1-RESOURCE-READ; reuse 0021's exact Resource Manager capability and guest visibility checks. Compare admitted installed, absent and denied Resource Manager with an unadmitted Media Gallery reader. Only an eligible current XenForo resource or independently admitted gallery item can enter the graph. No inferred visibility, additional probe or write. |
| F-0068 · REQ-0029-W1-PLACEMENT-CLICK-MAP · DES-0029-W1-PLACEMENT-AGGREGATE | FX-0029-W1-PLACEMENT-CLICKS, IT-0029-W1-PLACEMENT-CLICKS. Match only the 0034 admitted human added-link count to its exact eligible placement receipt and source revision under the common privacy release. Two current placements, an old revision, bots and suppressed support must stay distinct. Unproved position stays unknown and the view exposes no reader trail or referrer path. No new capture or rank input. |

QR-106 retains native authority, source coverage, profile and
measured footprint requirements. Shared cases retain their owners.
These fixture bindings are planned evidence only.

## How it is tested

Rust tests unless stated; add-on tests are PHP tests.

1. No placed text or stored record carries a `nofollow`, `sponsored` or
   `ugc` mark, for suggestions, hubs and Coach links.
2. Raising need to its ceiling changes order only among candidates that
   passed the gate; a failed one stays refused.
3. A hub and its thread link both ways; two plain threads still cannot.
4. The sales label swaps only candidates inside the margin and never beats
   a gate failure; no stored field holds money, orders or customers.
5. A reject rule removes a candidate at suggestion and at approval, and no
   rule ever adds or raises one.
6. Tri-Rank remains a sealed challenger; if it fails the proof standard it
   cannot enter spec 0077 promotion, and even a pass creates no independent
   production selection.
7. A count-kind signal is refused by the registry; a frozen thread keeps
   its last good rates.
8. A suggestion never shown cannot be ticked; "tick all" is not offered.
9. In a batch, one stale item is skipped and the others apply; a batch
   above the safe top is refused.
10. A link set for a date writes nothing before it, and writes only if
    every recheck passes on the day; a failed check returns the card to
    review with its reason.
11. Add-on test: typing in the editor causes no request and no work.
12. Drafting suggestions never change the draft, and reach a moderator only
    after the post exists.
13. The hold-back test cannot start without the site owner's agreement; a
    held page shows "held for test" and one click releases it; the cost is
    on screen before and during the test.
14. No post text is written without a moderator's approval of that change.
15. No language model or AI service is called to produce candidate words.
16. With the option off, a misspelt-word link is refused with the reason.
17. A later author edit survives undo of a text change.
18. A name labelled as a person is never offered a spelling fix.
19. An upload keeps only addresses of the registered site.
20. An import with address columns is refused whole; imported numbers never
    join darshak or another source's figures.
21. No limit can be set to zero or above its safe maximum.
22. A harassment flag never hides or changes a post.
23. The decision logic (never-marked check, hub-pair exception, reject
    rules, order caps, batch rules, date checks, hold-back eligibility and
    member-text span checks) follows ADR-0020. Components explicitly
    assigned to its strict core require 100% line, branch and MC/DC
    coverage and 100% of killable mutants; equivalent mutants need proof
    and independent review. Other components use the owning risk matrix's
    required coverage and mutation analysis. Kani proofs apply where
    tractable, with assumptions and trusted computing base recorded.

## Changes from the source

- The decision logic and the language layer are rewritten as Rust, with
  Kani, Verus or Creusot proofs, not a separate proof language.
- The maths and the hold-back analysis run in ganit, the Rust numerical
  engine.
- The admin screens are in the site's admin pages and the desktop app.

Source: `docs/specifications/linking/link-additions.md`,
`linking/link-additions-2.md` and `linking/link-additions-3.md` in the
archive `archive-2026-11-08-old-repo`.
