# 0105: Leviathan link evidence and pattern clues

Leviathan (spec 0082) reads pages in passing and keeps tiny receipts. This
specification owns what Sutradhar does with that evidence: the Evidence
screen for backlink receipts, the small "site clue cards", the clusters of
look-alike sites, the playful "Conspiracy Corkboard", and the opt-in "Web
Weirdness and Black-Hat Radar". Spec 0082 owns how a page is read, the receipt
format, the memory fence, the niche scan and the leaderboard count. Spec 0045
owns the backlink job and 0046 the link study that scores trust. This
specification adds no fetching of its own and no ranking.

Everything here is a pattern guess from public clues. It is never proof and
never an accusation. It stays on the user's PC, is off until the user opts
in, and is always stamped "Allegedly" (DR-041, DR-062 and the legal wall
of spec 0082). The text and screens of this spec need legal review before
release (QR-010).

Example: Mei runs a niche scan of fashion sites and opens Evidence. She sees
receipts: "fashion-mag.example links to your boots page with the words
'waterproof boots', followed, spotted 3 Nov". She switches on clue cards,
the Radar and the playful view. Eight lifestyle blogs share a theme, a
plugin list and the same mattress-shop link phrase. The Corkboard draws red
string between them under a big "ALLEGEDLY" stamp and says: "Automated pattern
guess from public clues. Not proof of anything." Mei can export the list or
delete it with one click.

Values marked "proposed" are design choices to be checked by measurement.

## What it does

### Words used here

- **Receipt**: the small record of one link found on a page (spec 0082).
- **Evidence**: receipts about a site the user cares about, shown as a list.
- **Clue card**: a record of about 110 bytes (at most 256) per domain, made
  while a niche scan passes through it, holding public technical traits.
- **Clue**: one shared trait between sites, such as the same server block.
- **Cluster**: a set of sites that share enough clues.
- **Corkboard**: the picture of a cluster, with red string for shared clues.
- **Radar finding**: one oddity on one page or site, such as hidden text.
- **Confidence**: for a cluster, low (3 clues agree), medium (4) or high (5
  or more). For a Radar finding, low when one signal fits, medium when two
  independent signals fit, high when three do. It is not a probability.

### Three layers, kept apart

| Layer | Needs | Shown as |
|---|---|---|
| Evidence | Receipts from backlink reads or a niche scan | Evidence list and counts |
| Clues and clusters | Clue cards from a niche scan, opt-in | Corkboard, "Allegedly" |
| Radar | A single read of a page, opt-in | Findings list with one proof line each |

Turning on one layer does not turn on the others. Each has its own opt-in.

### Evidence screen

1. A table of receipts for one target site: linking page, target page, link
   words, relation marks (nofollow, sponsored, user-generated, none), date
   spotted, and the one short sentence of proof.
2. Filters in plain words: "followed only", "new this week", "link words
   containing...", "from sites in the niche". The link-words filter reads
   plain text only: it is cut at 128 characters (proposed), it is matched as
   a literal phrase and never as a pattern, and an over-long or malformed
   entry is refused with a plain message.
3. Counts by linking site, with the leaderboard of spec 0082 shown as "most
   linked in this niche" and labelled "observed links, not quality".
4. Each row says "This is a record of what was seen on that date. The link
   may have changed since." A receipt is never refreshed by a repeat crawl of
   a rival (owner, 20 November 2026); the user's own sites may be re-read.
5. Receipts feed the web link study (spec 0046), which does the scoring.
   Evidence is the raw view of what was seen; spec 0046's "Links to my pages"
   report is the study view. Both read the same receipts and neither copies
   them. Evidence shows no score and no ranking. Dated provenance and
   eligible receipt fields remain evidence without optional context.
   Withhold over-limit legacy context under 0082 in both views and exports.
6. Export to a file the user can open (CSV and JSON), scrubbed as in spec 0082.
   A cell that starts with `=`, `+`, `-` or `@` (after any leading space or
   control character) is neutralised with a leading apostrophe in the CSV
   and in the JSON text fields, as spec 0094 does, so a spreadsheet never
   runs it as a formula.
7. Delete one receipt, all receipts of a site or all of them; the file ledger
   of spec 0030 and the erasure rules of spec 0082 apply.

### Clue cards

1. A card is made only when `evidence.clues_enabled` is on and a niche scan is
   running. It is made while the page streams past; no page is stored.
2. A card is a fixed record of at most 256 bytes (proposed) with these
   fields: a domain hash (8 bytes) and a scan number (4); the server block as
   a network prefix (6); the name-server group hash (8); theme and layout
   fingerprints (16); a plugin-and-robots-oddity hash (8); the main outbound
   destination hash, its link-phrase hash and the outbound link count (20);
   the posting-minute pattern, comment-presence flag and author-bio flag (8);
   up to 6 topic-word hashes (24); the date (4); and the commercial-link flag
   (1). The readable domain text is kept once per scan in a separate table, not
   in each card. A card holds no page text and no personal data (spec 0082,
   privacy gate).
3. A scan stops adding cards at the disk cap `evidence.card_cap_mb` (default
   20 MB, proposed). Ten thousand cards, with their domain table, must fit in
   about 3 MB.
4. A card is made once per domain per scan. Cards from an earlier scan are not
   compared with a later one to track a site over time; each scan stands on
   its own.
5. Cards of a finished scan are kept 30 days by default and then removed, or
   removed at once on request. Clusters, findings and their proof lines are
   removed at `evidence.findings_keep_days` or on erasure, whichever is first,
   whether or not cards exist.

### Clues and clusters

Five clues, each with a plain name, a plain meaning and a fixed test.

| Clue | Meaning | Test (proposed) |
|---|---|---|
| Same Building | Many "independent" sites on one server block, or name servers | At least 5 domains in one block with at least 1 other shared trait; a block alone never counts, and shared providers (big hosts, CDNs, cloud ranges) on the exclusion list never count |
| Identical Wardrobe | The same starter theme, layout, plugins and even the same oddity in robots.txt | Theme and layout fingerprints equal, plus one more equal trait |
| Suspiciously Loyal Fan Club | Unrelated topics all linking to one destination with the same stiff phrase | At least 4 topic-distant sites, same destination, same phrase, few other outbound links |
| Ghost Town Schedule | Posts always at the same minute, no real comments, stock author bios, one commercial link each | At least 3 of the 4 traits on at least 4 sites |
| Zombie Domain | A domain whose name suggests one business but whose page is about another (one read; no registration or owner data is used) | Domain words and page topic words disagree |

1. A cluster needs at least 3 distinct clues across at least 4 domains
   (proposed) to be shown on the Corkboard. Red string joins sites that share 3
   or more clues.
2. A cluster shows its clues in plain words, how many sites, and the stamp
   "ALLEGEDLY" with the text: "This is an automated pattern guess from public
   clues. It is not an accusation and not proof of wrongdoing."
3. A cluster never names a person, an owner or a company. No record holds a
   registrant, WHOIS or name-matched field. It lists domains only, as the
   user's own PC sees them.
4. The Corkboard is for the user's own PC. There is no share button for a
   cluster. The export has the same stamp on every line.
5. A domain the user marks "this is mine" or "this is fine" is removed from
   clusters, and the choice is remembered locally.
6. A false match is expected sometimes. Every finding has a "Why this?" link
   that shows the exact clues and the numbers.

### Radar findings

The Radar looks for the shady tricks beginners should recognise. Each finding
is a short evidence line, never a verdict. Comparison findings need the
admitted own-site evidence named below.

| Finding | What is compared | Proof kept |
|---|---|---|
| Invisible ink | Spec 0115 visibility evidence at an admitted render level | Scrubbed evidence reference and reason |
| Two-faced pages | Spec 0115 paired Audit observations of the same own-site page | Evidence references and scoped difference |
| Leftover AI prompt slips | Stray phrases of a chatbot reply inside a live article | The phrase and 60 characters around it |
| Fake five-star factories | Structured data claiming a rating with no visible reviews | The claimed numbers and the count of visible reviews |
| Find-and-replace city clones | Many pages identical except a place name | Count of pages and the changed words |
| Fake freshness | A "Last updated" stamp that moves while the text does not | Two fingerprints, only for the user's own sites |
| Parasite subfolders | A respected site hosting unrelated commercial pages | The path and the topic mismatch |
| Keyword salad | One phrase repeated unnaturally | The phrase and its count per 100 words |
| Circular hype rings | A closed loop of sites linking only to each other | The loop of domains |
| Redirect roulette | A long chain of redirects ending somewhere unrelated | The hops, up to 10 |
| Comment-spam graveyards | Old pages buried under spam comments | The count and a ratio |
| Sneaky footer freeloaders | Hidden commercial links in templates or widgets | The link, its position and a count from admitted own-site Audit evidence |

1. Receipts mode supports only findings backed by the facts that
   specs 0082 and 0115 permit there. It cannot measure hidden
   text, hidden placement, two-faced behaviour or structured data
   that needs Audit. Do not fetch, render or compare a second page
   to fill a gap. Missing capability or evidence gives unavailable.
2. Use the visibility output of spec 0115 for hidden text and
   placement and its matched Audit evidence for two-faced findings.
   Two-faced and fake-freshness comparisons use the owner's own
   sites only. A rival is read once (DR-062). A two-faced finding
   cannot show how a search engine's crawler was treated.

3. Findings use words like "looks like" and "may be". The names above are
   labels for the pattern, not statements about a person.
4. The user may switch each finding type off.
5. Radar never reads a page that the legal wall of spec 0082 would not allow.

### Legal and tone rules

1. Every cluster and finding carries the "Allegedly" stamp and the explanatory
   text. It cannot be removed by a setting.
2. Nothing here is sent anywhere. No cluster or finding leaves the PC unless
   the user exports it, and only after a confirm step that names the domains
   in the export. The export keeps the stamps. A receipt export needs the
   same confirm whenever it names a domain other than the user's own sites
   (for example a niche export); an export limited to receipts about the
   user's own targets needs none.
3. The playful look is an option: a plain table view shows the same data with
   no jokes and no pictures, and uses neutral names ("Hidden text", "Rating
   with no visible reviews", "Template links in many sites"). The playful
   names in this spec are labels for the playful view only. The default for a
   new install is the plain view. Final names wait for legal review (QR-010).
4. A person or business that asks to be removed (spec 0082 erasure) is removed
   from every card, cluster and finding.
5. Findings about named sites are never used by ranking or by any automatic
   action. They are information for the user only.

### Neutral Radar evidence summaries (N-1811 to N-1814)

These historical feature titles identify report ideas. Use the neutral
display names below and apply the immutable `Allegedly` marking. LE-02,
LE-05 to LE-07, LE-10 to LE-15 and LE-18 apply as appropriate. Respect
the independent, default-off Radar choice, provenance, export controls
for named domains, erasure and the ban on rival histories. These views
carry no rank weight or action authority.

| Feature | Neutral finding and admitted evidence |
|---|---|
| N-1811 | **Domain and page-topic difference.** Use the existing single eligible observation comparing domain words and topic from one read. Do not join snapshots or acquire new evidence. Describe observed content without identifying a registrant or asserting an expired registration, transfer, hijack or intent. Reject a rival history join. [FX-0105-W5-NEUTRAL-DOMAIN-TOPIC-OBSERVATION](../planning/feature-wave-5.md#acceptance-design-for-n-1791-to-n-1814). |
| N-1812 | **Comment-area link observation.** Show count and share of eligible distinct observed comments carrying links over all visible eligible sampled comments, and observed placements separately. Hold denominator for withheld or unproved coverage. Duplicate placements cannot add a comment. No moderation or outreach. Missing field coverage means unknown. Keep only the minimised counts and provenance, without commenter names, account identifiers or quoted text. [FX-0105-W5-SCOPED-COMMENT-DENOMINATOR](../planning/feature-wave-5.md#acceptance-design-for-n-1791-to-n-1814). |
| N-1813 | **Date and content comparison.** On an admitted own site compare exact date observations and compatible main-content fingerprints. Report a recorded date change with unchanged observed content if supported. Incomparable evidence remains unknown. [FX-0105-W5-OWN-SITE-DATE-AND-CONTENT-COMPARISON](../planning/feature-wave-5.md#acceptance-design-for-n-1791-to-n-1814). |
| N-1814 | **Recorded redirect hops.** Show the admitted ordered hop evidence up to the smaller of the existing route cap and the Radar display cap of ten. Record gaps, blocked observations or truncation without filling them in. Keep destinations minimised and apply existing disclosure controls. This view consumes existing evidence and starts no follow-up reads. [FX-0105-W5-RECORDED-MINIMISED-REDIRECT-HOPS](../planning/feature-wave-5.md#acceptance-design-for-n-1791-to-n-1814) |

At export check the exact named-domain set and current LE-14
confirmation, immutable Allegedly marking, and rights and erasure
at final file release.

#### Passive Radar evidence profiles (N-1825–N-1833)

The canonical feature names trace the owner catalogue. Use the
existing default neutral labels, separate Radar opt-in, type switches
and mandatory Allegedly stamp and explanation. Bind site, project,
exact eligible observations, source revisions, coverage, safe reason
and erasure to every finding. Keep bounded scrubbed proof lines,
no registrant or inferred person, owner or company identity, and no
comparison that tracks a rival across scans. Raw-only Receipts cannot
produce facts needing Audit or rendering. Every output is local
historical review information. It gives no automatic action, ranking
input, outreach, public disclosure or attribution. Keep exact named
domain confirmation for an eligible export under LE-14.

| Owner catalogue entry | Exact evidence and conclusion |
| --- | --- |
| N-1825 Invisible ink (hidden text) | Use only the admitted 0115 visible-layout evidence, with exact viewport, profile and reason. Distinguish actual non-visibility at that profile from missing assets, controls or partial rendering. Use a scrubbed evidence reference. No intent or live visitor claim. [FX-0105-W5-SCOPED-LAYOUT-VISIBILITY](../planning/feature-wave-5.md#acceptance-design-for-n-1815-to-n-1838) |
| N-1826 Two-faced pages (alleged cloaking) | Use paired comparable own-site Audit observations under 0115 and QR-050 only. Bind the two declared identities to exact page and complete comparable content, settings and viewport generation, or report a gap, and state profile, time, coverage and specific observed difference. No identity spoofing, rival repeat, new render or inference about search-engine treatment or motive. [FX-0105-W5-PAIRED-OWN-SITE-OBSERVATION-SCOPE](../planning/feature-wave-5.md#acceptance-design-for-n-1815-to-n-1838). |
| N-1827 Leftover AI prompt slips | Match eligible plain visible text under the admitted mode and retain only the scrubbed phrase and at most the existing 60 characters around it. Distinguish quotation, discussion and ambiguous context. A match cannot prove authorship, origin or deliberate automation. Text never becomes an instruction. [FX-0105-W5-VISIBLE-PHRASE-BOUNDED-REDACTED-CONTEXT](../planning/feature-wave-5.md#acceptance-design-for-n-1815-to-n-1838). |
| N-1828 Fake five-star factories | Compare the eligible structured-data claim about an exact entity with proved visible review evidence for the same entity and revision. Keep claimed and observed counts separate. Missing display, blocked resources, sampling or a mismatch in entity yields unknown. A discrepancy cannot prove false reviews, fraud or an owner's intent. [FX-0105-W5-EXACT-ENTITY-DECLARED-AND-VISIBLE-REVIEWS](../planning/feature-wave-5.md#acceptance-design-for-n-1815-to-n-1838). |
| N-1829 Find-and-replace city clones (doorway pages) | Compare eligible same-scan pages with bounded fingerprints and a confirming comparison. Separate exact common editorial text, changed place terms and covered sample. Repeated navigation alone is insufficient and legitimate variants are reported with uncertainty. No fetch to expand the sample or index action. [FX-0105-W5-SAME-SCAN-CONFIRMED-EDITORIAL-REPETITION](../planning/feature-wave-5.md#acceptance-design-for-n-1815-to-n-1838). |
| N-1830 Parasite subfolders | Describe an eligible same-scope path group's observed topic relationship to comparable published site sections. Record the sample, source and uncertainty. Topic mismatch alone gives no claim of reputation, control, business relationship or breach. No discovery of unauthorised paths or tracking a rival's changes over time. [FX-0105-W5-PATH-TOPIC-MISMATCH-EVIDENCE-GAP](../planning/feature-wave-5.md#acceptance-design-for-n-1815-to-n-1838). |
| N-1831 Keyword salad meter | Count frequency of an eligible phrase using a pinned eligible language and token profile in the covered editorial text. Show phrase count and words covered, with the existing per-100-words unit. Very short, ambiguous or partially covered text stays unknown when it cannot support the finding. No inferred intent, search penalty or score. [FX-0105-W5-PHRASE-COUNT-EDITORIAL-DENOMINATOR](../planning/feature-wave-5.md#acceptance-design-for-n-1815-to-n-1838). |
| N-1832 Circular hype rings | Describe a bounded cycle in eligible observed links from the exact same scan, with proof per observed edge. Missing outward evidence is unknown and does not prove a closed group. Keep clusters separately opted in and apply existing exclusions. No inferred coordination, ownership, link placement proposal or map of ways to manipulate links. [FX-0105-W5-SAME-SCAN-CYCLE-AND-COVERAGE](../planning/feature-wave-5.md#acceptance-design-for-n-1815-to-n-1838). |
| N-1833 Sneaky footer freeloaders | Use admitted own-site Audit visibility and exact placement evidence for repeated footer or widget links. Record eligible link, position and distinct covered placements with the current site and profile. A footer occurrence or unknown visibility alone does not prove a hidden commercial link. No attribution of who inserted it or automatic removal. [FX-0105-W5-VERIFIED-VISIBLE-FOOTER-PLACEMENT](../planning/feature-wave-5.md#acceptance-design-for-n-1815-to-n-1838). |

Raw Receipts reports visibility, placement and Audit-dependent
structured evidence unavailable. No render starts here. Apply
DR-079, 0020 and 0100 outbound boundaries. LE-14 exact owner
confirmation covers only local hand-off of the named domain set.
Mark every released line Allegedly. Erase managed cards, clusters
and findings and refuse replay of their stale copies.

#### Passive fingerprint and clue projections (N-1875–N-1879)

Use only the admitted opt-in local niche scan under specs 0082
and 0105. Derive eligible bounded facts during the one authorised
guest pass. The owner starts the eligible read under 0082. LE-02 independently
gates clues, clusters and Radar. Do not add a receipt opt-in. No repeat rival
acquisition, new DNS or
ownership investigation, account data, raw IP publication or
tracking identifiers. The plain view and qualified legal review
remain required under DR-063 and QR-010. Clusters, findings and
any eligible export with current LE-14 named-domain confirmation retain their
Allegedly
stamp and explanatory text. No accusation, ranking, outreach
or automated site action. Apply current retention and erasure
before local view, export, cache, retry and restore.

| Feature | Exact eligible observation |
|---|---|
| N-1875 Tiny site fingerprint cards | Use the declared 0105 card fields, 256-byte proposed cap, scope-specific key, exact active scan generation and one card per domain. Erase derived cards, clues, clusters, findings, caches and separate readable-domain record under the same fence. No stored page or cross-scan tracking. Keep eligible domain text only in its separate protected scope. Exclude raw addresses, analytics identifiers, personal data and owner clues that have no admitted field. A card cap stops cards under LE-16. [FX-0105-W5-BOUNDED-SAME-SCAN-CARD-ERASURE](../planning/feature-wave-5.md#acceptance-design-for-n-1863-to-n-1880). |
| N-1876 Same Building clue | Use only the approved observed provider grouping and independent shared trait from the same scan. Under the existing proposed rule require at least five distinct eligible domains and one other shared trait. Aliases add no domain. A shared block or CDN alone cannot count and excluded providers stay excluded. Unknown provider or missing trait is unresolved. Do not retain or reveal a publicly reusable address or query again to confirm identity. [FX-0105-W5-DISTINCT-DOMAINS-AND-ADDITIONAL-TRAIT](../planning/feature-wave-5.md#acceptance-design-for-n-1863-to-n-1880). |
| N-1877 Copy-Paste Closet clue | This source label maps to the existing Identical Wardrobe clue, not a second clue or vote. Compare only the admitted theme, layout and one additional eligible equal trait on same-scan cards, as proposed. Shared template use or an incomplete fingerprint does not establish copying. [FX-0105-W5-ONE-WARDROBE-CLUE-WITH-CORROBORATION](../planning/feature-wave-5.md#acceptance-design-for-n-1863-to-n-1880). |
| N-1878 Suspiciously Loyal Fan Club clue | Compare the existing bounded same-scan destination, phrase and distinct topic and outbound-count evidence under 0105. Keep the exact eligible coverage and proposed minimum of four distinct topic-distant sites with matching destination and phrase and qualified low outbound count. No phrase or destination proves common ownership. Do not fetch the destination or revisit a source for the clue. [FX-0105-W5-EXACT-PHRASE-DESTINATION-AND-TOPIC-COVERAGE](../planning/feature-wave-5.md#acceptance-design-for-n-1863-to-n-1880). |
| N-1879 Ghost Town clue | Use only the admitted Ghost Town Schedule traits from the same scan, with present, absent and unknown facts distinct. Under the proposed rule require at least three of four admitted traits on at least four distinct eligible sites. Use the separate cluster minima under LE-04. Do not infer a posting schedule from a single ambiguous date or identify an author. A missing comment or bio alone proves no pattern. [FX-0105-W5-SCOPED-SCHEDULE-TRAITS-AND-EXPORT-ERASURE](../planning/feature-wave-5.md#acceptance-design-for-n-1863-to-n-1880). |

Trace LE-02 to LE-17 as applicable. QR-219 qualifies exact
same-scan matching, missing provenance, plain view, scrubbed
exports and withdrawal. No source title creates a second clue.

### Data

- `evidence_views`: target site, filters, saved by.
- `clue_cards`: scan, domain hash, server prefix, name-server hash, theme and
  layout fingerprints, plugin-and-robots hash, destination hash, phrase hash,
  outbound count, posting pattern, comment and bio flags, topic hashes, date,
  commercial flag. A separate `scan_domains` table holds the readable domain.
- `clusters`: scan, id, clues, domains, confidence, created.
- `radar_findings`: scan, page, type, proof line, confidence, created.
- `evidence_marks`: domain, "mine" or "fine", by, time.

Receipts themselves belong to spec 0082's receipt store.

### Settings

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `evidence.clues_enabled` | on or off (cards) | off | two values | next scan |
| `evidence.clusters_enabled` | on or off | off | two values | next scan |
| `radar.enabled` | on or off | off | two values | next scan |
| `radar.types` | list of finding types | all | any subset | next scan |
| `evidence.card_cap_mb` | MB | 20 | 1 to 200 | next scan |
| `evidence.card_keep_days` | days | 30 | 1 to 365 | next night |
| `evidence.cluster_min_clues` | whole number | 3 | 3 to 5 | next scan |
| `evidence.cluster_min_domains` | whole number | 4 | 4 to 50 | next scan |
| `evidence.findings_keep_days` | days | 30 | 1 to 365 | next night |
| `evidence.playful_view` | on or off | off | two values | at once |

There is no setting that removes the "Allegedly" stamp or makes clusters
public.

### Failure codes

SUT-920 clue cards not enabled; SUT-921 card cap reached, scan continues
without cards; SUT-922 reserved (page refusals use spec 0082's codes);
SUT-926 owner field refused; SUT-927 cross-scan comparison refused; SUT-928
repeat read of a rival refused; SUT-923 export
refused (a line would lose its stamp); SUT-924 finding withdrawn (domain
marked fine); SUT-925 receipt store unreadable. A failure never stops the
scan itself.

### Security and privacy

- Cards and findings hold public technical traits only, with no personal data.
- Page text is data, never an instruction. A phrase such as "Sure! Here is an
  article" is stored as text and never obeyed.
- Everything stays in the home folder and in the file ledger of spec 0030.
- An export is scrubbed again before it is written.

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| LE-01 | Evidence MUST show receipts with date and proof, and MUST show no score. | UT-LE-01, BD-LE-01 |
| LE-02 | Clue cards, clusters and Radar MUST each need their own opt-in (`evidence.clues_enabled`, `evidence.clusters_enabled`, `radar.enabled`) and be off by default. | UT-LE-02 |
| LE-03 | A clue card MUST be at most 256 bytes, hold no personal data and be made from a streaming read. | UT-LE-03, FZ-LE-01 |
| LE-04 | A cluster MUST need the configured minimum clues and domains. | UT-LE-04 |
| LE-05 | Every cluster, finding and export line MUST carry the Allegedly stamp, and the stamp MUST NOT be removable. | UT-LE-05, PL-LE-01 |
| LE-06 | A cluster or finding MUST NOT name a person, owner or company, and no record MUST hold a registrant or name-matched field. | UT-LE-06, PL-LE-02 |
| LE-07 | Cards from different scans MUST NOT be compared to track a site over time. | UT-LE-07, PL-LE-03 |
| LE-08 | A cluster's confidence MUST follow the stated mapping, and a Radar finding's confidence MUST follow its own. | UT-LE-11 |
| LE-09 | A site marked mine or fine MUST be removed from clusters. | BD-LE-02 |
| LE-10 | Findings MUST NOT feed ranking or any automatic action. | IT-LE-01 |
| LE-11 | A verified erasure request MUST remove the domain from every card, cluster and finding. | IT-LE-02 |
| LE-12 | Every finding MUST have a "Why this?" view with its exact clues. | BD-LE-03 |
| LE-13 | The plain view MUST be the default, use neutral names and show the same data. | AC-LE-01 |
| LE-14 | A cluster or finding export, and a receipt export naming a domain other than the user's own, MUST need a confirm step naming its domains. | BD-LE-04 |
| LE-15 | Every stored proof line and domain text MUST be scrubbed and size-capped. | UT-LE-09, FZ-LE-02 |
| LE-16 | Reaching the card cap MUST stop card making, not the scan; deleting receipts MUST remove their ledger entries. | IT-LE-03 |
| LE-17 | Same Building MUST ignore shared providers and a block alone. | UT-LE-10 |
| LE-18 | Fake-freshness and two-faced checks MUST run only on the user's own sites; a rival MUST be read once. | UT-LE-08, PL-LE-04 |
| LE-19 | Radar MUST use spec 0115 visibility and paired Audit facts where required and MUST report unavailable in Receipts mode. | IT-LE-09 |
| LE-20 | An export cell or JSON text field starting with `=`, `+`, `-`, `@`, a space or a control character MUST be neutralised so a spreadsheet never runs it. | UT-LE-12 |

## How it is tested

Designed, not run.

- **Unit (UT-LE):** one fixture per clue and per Radar finding with a positive
  case, a case just below the threshold and an honest look-alike that must not
  match (for example two real sites on one big host); card size; cluster
  thresholds; stamp present on every output line.
- **Planted faults (PL-LE):** a build that drops the stamp, one that names an
  owner (PL-LE-02), one that compares two scans (PL-LE-03), and one that
  re-reads a rival for freshness (PL-LE-04); each must be refused with its
  named message (SUT-926 owner field refused, SUT-927 cross-scan comparison
  refused, SUT-928 repeat read of a rival refused).
- **Behaviour (BD-LE):** the Mei scenario; marking a site "mine"; the "Why
  this?" view.
- **Integration (IT-LE):** a niche scan of a small fixture web with seeded
  clusters; erasure of a domain; findings kept out of ranking.
- **Fuzz (FZ-LE):** malformed headers and pages never crash card making or
  create a card with a wrong size; proof lines and domain text with personal
  identifiers are scrubbed and capped (FZ-LE-02).
- **Accessibility (AC-LE):** the Corkboard has a list view and text for all
  string colours; both views on both systems.
- **Measurement:** time and memory of making 10,000 cards in a streaming scan.

- **UT-LE-12:** export formula safety. Run every CSV and JSON export
  (receipts, clusters, findings) with a text field starting in turn with
  `=`, `+`, `-`, `@`, a space then `=`, a tab then `+`, a carriage return,
  a line feed, a null and another control character, in a domain, a proof
  line and a note. Each CSV cell and JSON text field comes out with a
  leading apostrophe, the Allegedly stamp is still present, and a plain
  value starting with none of these is unchanged. Re-reading the export
  gives back the original text after removing the apostrophe.
- **IT-LE-09 (numbers 04 to 08 are unused).** Use only Receipts
  facts. Visibility and paired findings show unavailable. On an admitted
  own-site Audit, use the exact versioned render facts. For F-0176,
  give one supported RN-09 obscured region and one proved collapsed
  control without a valid pair. Show scoped layout facts with safe
  reasons and no served-response comparison. Test a region with an
  unproved dependency and a region outside the measured viewport.
  Independently use an admitted RN-45 pair. Reject a mismatched job,
  site, revision or identity, a partial or stale fact, and a difference
  caused by script timing or viewport. Withhold only dependent facts
  while an independent source remains eligible. Neither branch makes
  a follow-up fetch.

## Changes from the source

This spec is new. It takes the owner's pastes of 19 November 2026 on
backlink receipts, niche scans, "allegedly PBN" clues and black-hat checks.
Receipts, the memory fence, presets and the niche leaderboard stay in spec
0082, where they already live; their duplicate pasted entries were merged
there. The pasted "fake freshness" and "zombie domain" checks are narrowed so
that they need no repeat read of a rival, following the owner's rule of 20
November 2026. The default is the plain view, not the playful one, until legal
review (QR-010).
