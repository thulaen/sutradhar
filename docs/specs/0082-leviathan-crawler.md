# 0082: Leviathan, the crawler

Leviathan is Sutradhar's one crawler. It reads pages for the site audits of
spec 0044 and the backlink and notice reads of spec 0045, and it is built to
do what a Googlebot-class crawler does, at the scale of one person's PC, and to
stay inside the law and every site's locks (ADR-0022). This specification owns
the crawler platform: its identity, its legal wall, its politeness, its modes
and its resource fences. Specs 0044 and 0045 own what the audits and the
backlink reads do with the pages. Pattern Clues and the other Leviathan
features are written in later specifications on this base.

Example: Rana points Leviathan at a public fashion blog to find who links to
her shop. Leviathan fetches robots.txt first and finds the blog allows it. It
reads each page in small chunks, finds one link to her shop, keeps a short
receipt and throws the page away. Halfway through, Rana joins a video call.
The memory brake slows the crawl from 20 pages a second to 2. Another site
shows a "prove you are human" page; Leviathan stops on that site, tells her
why, and does not try to get past it.

Values marked "proposed" are design choices to be checked by measurement.

## What it does

### Words used here

- **Receipt**: the small record kept from a page in Receipts mode.
- **Frontier**: the list of addresses waiting to be fetched.
- **Block page**: a page that blocks or challenges a visitor, such as a robots
  refusal, a rate-limit page or a "prove you are human" check.
- **Compliance ledger**: the tamper-proof record of why each fetch was allowed.
- **Opt-out signal**: any machine-readable request not to crawl, index, quote,
  archive or use a page for text mining or AI training.

### Identities

1. Leviathan has two declared identities: `Leviathan/<version>` for fetching,
   and `Leviathan-Browser/<version>` for pages it renders in Audit mode. Every
   request carries one of them and an address of the public page that says
   who runs the crawler, why it visits, and how to stop it.
2. It never uses another crawler's name or another company's identity, and it
   never poses as a person's browser without the Browser-Mode label. A
   two-faced check uses the paired own-site Audit facts of spec 0115.
   It cannot show how a page treats another crawler.
   DR-132 admits a separate owner-confirmed pair under
   [spec 0141](0141-audit-crawl-logs-and-technical-toolbox.md).
   Use the existing fetch and admitted Audit render identities,
   exact owned public page and current serving-generation proof.
   Run the legal wall independently for every request.
3. Every install has its own signing key, and requests are signed with it
   (HTTP message signatures in the Web Bot Auth style). The identity page
   explains how to check a signature, how to allow Leviathan and how to
   block it. The exact scheme is checked against its published standard in
   the research dossier before the first signed request (QR-025).
4. A site owner can tell that two visits came from the same installation by
   the key's thumbprint, and can allow or block that thumbprint. The
   signature does not say who the user is, and a home PC has no fixed address,
   so no address list is published. For a site the user owns, the app shows
   the thumbprint so the owner can allow it.

Spec [0152][backlink-profile] owns the local name and separately
approved public identity-card label. The canonical wire identities,
operator truth, signing and stop behaviour remain here.

[backlink-profile]: 0152-live-backlink-crawler-profile.md

### The legal wall

The wall is built into the fetcher, not left to features. A feature cannot call
the network except through it.

1. **robots.txt.** It is fetched first for every host and obeyed under
   Leviathan's name, to the finest wildcard, by the pinned parser (fuzz tested).
   A missing answer from an error, 401 or 403 counts as deny, which is stricter
   than some crawlers and is kept on purpose. A file over 500 KiB is reported
   and only its first 500 KiB is used. If the file cannot be reached, Leviathan
   holds back from the host and retries later.
2. **Page and header signals.** `X-Robots-Tag` headers and page meta tags such
   as noindex, nofollow, nosnippet and noarchive are read before anything is
   stored, and obeyed.
3. **AI and text-mining opt-outs.** Machine-readable reservations (robots
   directives for AI crawlers, text-and-data-mining reservation files, a site's
   `ai.txt` and emerging content-preference headers) are detected and obeyed.
   The list of supported signals and their exact syntax is verified in the
   research dossier before release (QR-025). An unrecognised but plausible
   signal is logged and the page is treated as reserved.
4. **No bypass, by construction.** The fetcher has no code path that solves a
   CAPTCHA, guesses or reuses a login, passes a paywall, replays a challenge or
   avoids a block. A page marked as paid, or answering 401, 402 or 403, is left
   unread and recorded as "could not read". Guest view only: no sign-in, no
   forms, no cookies kept between pages.
5. **Block pages.** On a block page Leviathan stops on that host and tells the
   user. On a registered site it pauses that site's whole crawl and raises an
   issue for a person. It never retries past a block, and reports it as a block,
   not as an ordinary error.
6. **Material it avoids.** It does not fetch addresses on the shipped lists of
   known piracy, warez and adult sites, and it stops on a host whose content
   type signals match. The lists are curated and reviewed by the project, and
   a false match is reported and corrected. This is a best-effort control, not a
   promise that no such page is ever touched.
7. **Read-only.** Only GET and HEAD requests are sent. It never submits a form,
   adds an item to a cart or triggers an action.
8. **Address safety.** Private, loopback, link-local and metadata addresses are
   refused at the socket, including after a redirect, so a public page cannot
   point Leviathan at a home router or a cloud service.
9. **Stop list.** Any site that asks goes on a stop list and stays there.

### Privacy gate and scrubber

1. Before fetching, collection rules are checked: directories known to hold
   personal accounts or financial records, and sites aimed mainly at children,
   are skipped.
2. At download time, personal identifiers (phone numbers, email addresses,
   government ID numbers, home addresses) are removed from any text before it
   is written to disk. The patterns and their tests are in the specification of
   the scrubber; a miss is a defect, not a feature choice.
3. Regional rules (GDPR, UK rules, US state rules) adjust retention, opt-outs
   and scrubbing, from a rulebook the project maintains. The rulebook needs
   legal review before release (QR-010).
4. Erasure: a verified request to delete a person's or site's records wipes
   every snapshot, cache, backup and index, and produces a deletion receipt.
5. Receipts, clue cards and snapshots hold no more personal data than the
   scrubbed text.

### The compliance ledger

For every fetch the ledger records the time, the address, the version of the
robots file that allowed it, the opt-out signals seen and the legal basis. It is
append-only with chained hashes, held in the file ledger's protection (spec
0030), and survives the page being discarded. A fetch with no ledger entry is
refused: if the ledger cannot be written, the crawl stops.

For the separate spec 0120 and 0131 routes, the chain may hold an opaque
reference to a sealed erasable record with the exact address.
Commit both pieces before a fetch. On verified erasure append a
safe outcome and delete the protected address. The chain retains
no path or reusable address digest. Deny a new fetch without both
pieces and current permission.

### Politeness

These rules are carried from specs 0044 and 0045 and now live here.

1. **Own sites.** Rate `leviathan.rate_per_second`, default 2, allowed 1 to 8,
   with an aggressive preset of 4. One request at a time per host. Above 2 a
   second the app asks for confirmation and shows the site's measured response
   time.
2. **Outside sites.** One request at a time per host and the applicable
   shared server, with the admitted gap or longer host wait. Ordinary
   0045 runs use the separately admitted [0152 profile][backlink-profile].
   Unadmitted outside profiles stay held.
3. **One slowing rule.** Leviathan measures a site's normal median response time
   with no crawl running. It slows when the median rises 10% above normal and
   pauses at 25% above. The memory scheduler of spec 0078 can pause it too.
4. **Waits.** It honours `Retry-After`, 429, `Crawl-delay` and
   `cf-mitigated`; it stops on repeated failure and retries later.
5. **Budgets.** An hourly limit on requests and bytes per site; going over it
   is SUT-654.
6. **Shared servers.** All Leviathan fetch routes on one installation
   use one scheduler reservation for each exact request. The reservation
   claims both the canonical host and current verified shared-server
   group. This includes policy reads, redirects, Audit requests and
   backlink requests. Claim both keys atomically or claim neither. Bind
   both to the same possible handoff record under 0116 LS-09/31.
   Another route, site, mode, worker or identity card cannot acquire
   a second claim on either key. Each route proves its own authority
   and limits.

   At claim and before handoff, take the latest of the stored host and
   group deadlines, the last proved start on either key plus the
   candidate's applicable minimum gap, and current slower deadlines.
   Use a trusted upper bound if the start is unknown. Admit at equality
   when the other checks pass. A looser route cannot shorten a wait.
   Hold dispatch without current address group proof or trusted time.
   Changed address proof needs a new claim for the same logical
   request. Fence the sender and close its proved-unsent attempt
   under 0116 before opening the next one. Preserve the host wait
   across the change and apply the new group's wait.

   A possible handoff keeps its request charge and published wait.
   Before releasing either claim, prove the adapter and stream have
   stopped and bind their actual start or a trusted upper bound, such
   as a qualified closure time. The earlier intent time alone cannot
   bound a delayed send. At release atomically record the start bound
   under both keys. Set each deadline to the latest of its old deadline,
   the bound plus that attempt's minimum gap, and later required waits
   including Retry-After. If closure or the time bound is unproved,
   hold both claims. A missing result continues to hold the original
   attempt under 0116. Share safety state only inside the scheduler.
   The explanation exposes no other site's work or permissions.
7. **Byte caps.** Responses are cut at a cap by type, proposed 2 MiB for HTML
   and 64 MiB for PDF, and decompressed size is capped.
8. **Traps.** Endless spaces (search results, member lists, sort parameters,
   calendars) are recognised by pattern and sampled, not followed for ever.
9. **Repeat visits.** Conditional requests (`ETag`, `If-Modified-Since`) are
   used, and shared assets are fetched once per site.
10. **Circuit breaker.** All requests to a host stop at once if errors spike,
    block pages appear or latency passes a danger threshold.
11. **Quiet hours.** Heavy site-wide audits default to the site's local
    overnight hours.

### Modes

| Mode | Reads | Keeps | Used by |
|---|---|---|---|
| Receipts | A lead list or a niche | Small receipts only; the page is discarded | Backlink reads (spec 0045), niche scans |
| Audit | The user's registered sites | Facts, snapshots, vault, archive files, rendering output | Site audits (spec 0044) |
| Notices | A fixed list of status pages | A dated note of any change | Spec 0045 |

Each crawl job chooses one mode, and the Jobs screen shows which. A mode cannot
be changed during a job.

### Receipts mode

1. A page is read as a stream in 16 KiB chunks (proposed). Links are found on
   the fly and each chunk is dropped when done. Reading stops at a page ceiling,
   default 512 KiB, allowed 64 to 512 KiB. The ceiling covers decoded
   bytes, including when the server compresses a response.
2. Images, video, fonts, scripts and other heavy files are not fetched,
   even for an empty script-built shell. Use the raw level and only
   the sitemap, feed and header readers of spec 0115.
3. A receipt is kept only when the page links to a target the job cares about.
   It holds the linking address, the target address, the link words, the
   relation marks (nofollow, sponsored, user-generated), the date, and one short
   optional sentence around the link, scrubbed of personal data.
   Under DR-059 keep at most 160 Unicode scalar values after scrubbing.
   A setting of zero omits the sentence. If the whole safe sentence exceeds
   the selected limit, omit it. Older over-limit stored context is withheld
   on read, export and restore. Independently eligible receipt fields stay
   available under their original authority and retention. DR-059
   admits the optional context under this receipt profile.
4. Fixed memory fence: the crawler's working memory stays inside
   `leviathan.receipts.memory_mib`, default 300, allowed 100 to 500 (proposed),
   and never grows past it.
5. Visited addresses are remembered in a compact filter (Bloom or Cuckoo), about
   12 MiB for ten million addresses (the owner's figure, to be measured), with a
   stated false-positive rate. A rare false positive skips a page; it never
   repeats one.
6. The frontier lives in a compressed queue file on disk (an internal or
   external drive). Only the next few hundred addresses are in memory.
7. A speed brake follows spec 0078's pressure levels: if free memory falls, the
   crawl slows by steps and pauses before anything else is shed.
8. Presets: Quick scout (5 minutes), Niche sweep (20 minutes) and Stop at 50 MB
   of receipts. A preset sets a time or a size cap; it never raises a limit.

Free public link maps, such as Common Crawl's lists (spec 0040), may supply a
shortlist of pages that link to a site; Leviathan then visits only those pages.

Spec 0131 uses a separately authorised single-page Receipts profile.
It takes only admitted raw heading facts, never the page or a new
reader. Its route does not broaden any other crawl or draft feature.

### Niche (focused) crawling

1. A niche scan starts from a small starter set of public hubs, 20 to 50.
2. Each outgoing link is scored for the niche from its words, its address words
   and its sentence. High scores go to the front; off-topic links are dropped.
3. A fixed-size sketch (Count-Min or top-k) counts how many distinct niche
   sites link to each destination, in a few megabytes. A switch hides very large
   platforms. The leaderboard is a count of observed links, never a ranking of
   quality.
4. A public site may be read within the legal wall and the admitted
   lead or niche scope. For a registered rival, each admitted URL
   needs an owner-started one-off check under DR-062 and DR-085.
   Do not admit it to background discovery, queued rechecks or a
   chart refresh. Claim the one-off read in the site ledger before
   opening the page. A restart, duplicate job, redirect or canonical
   alias cannot spend the same admission twice. Recheck the target
   at each scope change. An uncertain or spent claim gives no read.
   The existing approved non-rival backlink route stays separate.

### Audit mode

Audit mode keeps what site audits need: page facts and short copies (spec 0044),
full snapshots in a content-addressed vault, archive files for each crawl, a
crash-safe journal, a link graph, and a crawl-diff between two crawls. It stays
inside the disk cap of spec 0078 and spec 0030's ledger. Raw HTML and headers
only until EX-02 admits an engine and the host's cage passes spec 0115.
Strip every `Set-Cookie` field at response ingress, including redirects
and brokered subresources. Only the bounded Audit observer of spec 0044
can inspect a field before stripping. Drop the raw field before sending
a response to workers, archives, cache, replay, telemetry or traces.
No fetch sends a cookie. A scrub failure refuses the response.
Spec 0044's small render check uses that route for at most
`seo.crawl.browser_pages_per_day` pages per site, off-peak. An
unavailable route stays raw only and reports why.

### Layers and reuse

Leviathan's own code is the legal wall, the compliance ledger, politeness, the
frontier and filters, receipts, the vault and archive output, the link graph,
the diff and the explainers. Commodity layers are mature crates: HTTP/1.1,
HTTP/2, HTTP/3 and QUIC; TLS with audited cryptographic primitives; DNS with
DNSSEC; compression decoders with expansion limits; the HTML, XML, image, PDF
and font decoders; and, later, a rendering engine. Each choice is a build-or-
reuse record in the research dossier. Every untrusted byte goes through a
memory-safe parser worker with no privileges, with per-page limits on time,
memory, connections and sub-resources.

### Plain-English explanations

A page that was not crawled has one clear reason, in a sentence: the exact
robots line, the tag, the depth limit, the block page or the time-out. A
pre-crawl estimate (pages, time, disk, load on the site) is shown from a small
polite sample. Live controls change speed, pause a folder or add an exclusion
while a crawl runs.

#### Legal wall, privacy, identity and polite fetching (N-1632 to N-1646)

Use the existing guest-only fetcher, legal wall and site or one-off
authority. Bind purpose, identity, mode, host, current robots policy,
scope, retention and resource limits. Check the applicable rules
before dispatch, before following an admitted next destination and
before storing or releasing any fact. An unknown required rule holds
the affected operation. The user, a signed identity, a rendering
request and a queued job remain subject to the same policy.

| Feature | Exact boundary and refusal | Existing requirements |
|---|---|---|
| N-1632 Page-level tag and header obedience | Read eligible response headers and metadata only in bounded transient privilege-free parsing before retention. Resolve per-purpose conflicts under the restrictive rule and keep no raw disk, log or spill. Unsupported or partial directives cannot permit retention or link traversal. Apply the qualified rule profile to the exact response and destination and use its restrictive resolution. A denied index, follow, snippet or archive effect cannot be created from the denied page data. Hold ambiguous admission and preserve independent stop controls. | LV-R01, LV-R02 · [FX-0082-W5-RESTRICTIVE-RESPONSE-DIRECTIVE](../planning/feature-wave-5.md#acceptance-design-for-n-1623-to-n-1646) |
| N-1633 Modern AI and text-mining opt-out compliance | Use only QR-025's versioned and dated signal and syntax profile. Bind publication and storage decisions to the actual purpose. Treat a plausible unknown reservation conservatively. A new or unclear signal cannot authorise crawling, retention, quotation, AI use or a fallback identity. Keep safe resolution evidence without copying reserved text. | LV-R02, LV-R10, QR-025 · [FX-0082-W5-SCOPED-TEXT-MINING-RESERVATION](../planning/feature-wave-5.md#acceptance-design-for-n-1623-to-n-1646) |
| N-1634 Hard architectural wall against bypassing locks | Apply the existing guest-only restriction equally to every owner, renderer, import reference and restored job. On authentication, payment, challenge or block evidence use the owning stop outcome and hold further access under policy. The crawler has no override for an owner's authorisation claim, no alternate identity to pass a block and no authenticated crawl. | LV-R01, LV-R03, LV-R04, LV-R05 · [FX-0082-W5-GUEST-WALL-REFUSAL](../planning/feature-wave-5.md#acceptance-design-for-n-1623-to-n-1646) |
| N-1635 Pre-crawl GDPR data-minimisation gate | Check the admitted purpose, data minimisation policy, allowed category and exact current source boundary before any fetch. Disallow known personal-account, financial or children-focused scopes under the owning route. Review unknown regulated categories without broadening collection. Record only the safe decision. Qualification under QR-010 provides the policy, with no claim of automatic legal clearance. | LV-R03, LV-R06, LV-R07 and QR-010 · [FX-0082-W5-CURRENT-MINIMUM-DATA-SCOPE](../planning/feature-wave-5.md#acceptance-design-for-n-1623-to-n-1646) |
| N-1636 Live personal-data scrubber at download time | Allow only bounded transient privilege-free parsing needed for directives and scrubbing. Strip cookies and scrub before cache, disk, spill, other parser projection, logs, snapshots or release. Check the complete emitted projection, including boundaries between fragments and compound fields. A scrubber failure or unsupported response withholds the affected content. Keep safe aggregate reasons instead of revisiting raw protected bytes. | LV-R07, LV-R08, LV-R13 · [FX-0082-W5-SCRUB-BEFORE-SINKS](../planning/feature-wave-5.md#acceptance-design-for-n-1623-to-n-1646) |
| N-1637 Cryptographic crawler identity | Use the declared Leviathan/<version> or Leviathan-Browser/<version> and exact admitted signed peer verification profile and per-install thumbprint under DR-061 and QR-025. Protect the signing key and distinguish verifiable installation identity from user identity. Hold unqualified or invalid identity. A signature is evidence of sender only and cannot override robots, a block, a wait or storage policy. | LV-R01, LV-R03, LV-R10 · [FX-0082-W5-BOUND-SIGNED-IDENTITY](../planning/feature-wave-5.md#acceptance-design-for-n-1623-to-n-1646) |
| N-1638 Transparent identity card and published IP list | Display the declared crawler and purpose, approved verification information and existing public stop route only. On a registered own site the owner can inspect the allowed key thumbprint where eligible. DR-061 replaces the source's public IP list. No identifying address, account or raw key is published. | LV-R10 and DR-061 · [FX-0082-W5-IDENTITY-CARD-DISCLOSURE](../planning/feature-wave-5.md#acceptance-design-for-n-1623-to-n-1646) |
| N-1639 Cascading erasure engine | Accept only the verified scope and current authority under 0032. Fence reads, queued work, cache, snapshots, indices and restore immediately using the current erasure generation. Apply the owning protected backup and compliance-ledger treatment. Record verifiable completion only when all applicable stores are reconciled. On partial physical cleanup hold access and keep an unfinished receipt. No completed result, restore or rollback of the durable erasure fence. Report only deletion proved by each owned store. Never let a stale backup restore availability. | LV-R07, LV-R12 and 0032/0118 · [FX-0082-W5-ERASURE-FENCE-RESTORE](../planning/feature-wave-5.md#acceptance-design-for-n-1623-to-n-1646) |
| N-1640 Region-aware legal rulebook | Use QR-010's dated qualified rule profile, exact scope and effective version for eligible source, purpose and authority. Hold unknown applicability or disallowed regional collection, retention or use. A location estimate cannot admit a region and no rule creates a market setting, extra host call or legal verdict. | LV-R01, LV-R02, LV-R07 and QR-010 · [FX-0082-W5-CURRENT-QUALIFIED-REGION-RULE](../planning/feature-wave-5.md#acceptance-design-for-n-1623-to-n-1646) |
| N-1641 Tamper-proof compliance ledger | Use the existing protected append-only hash chain and durable intent before each permitted outbound attempt. Before each actual eligible request commit an intent with request and attempt ID, site, recipient, declared identity, purpose and mode, robots and stop generation, applicable wait, retention, budget and safe target reference. Check policy immediately before each separately admitted bootstrap, redirect or retry. Unknown intent acknowledgement requires reconciliation before dispatch. Under DR-156 and 0116 LS-09/31, the sole adapter commits the exact possible-handoff state before transport. Reclaim an ordinary visit only using a current terminal no-handoff record after fencing its former sender. An uncertain handoff preserves the original attempt, charge and waits. Reconcile the exact outcome without sending again. Append response status, directives, bytes and hold separately. For 0120/0131 commit the chain and sealed erasable address together. Erasure retains no raw address or reusable digest in the chain. A missing or invalid predecessor, intent or write result holds a request. Keep the separately admitted erasable record where required by the owning route. The chain detects tampering and carries no private payload. | LV-R06, LV-R12 and 0030/0118 · [FX-0082-W5-DURABLE-INTENT-CHAIN](../planning/feature-wave-5.md#acceptance-design-for-n-1623-to-n-1646) |
| N-1642 Read-only browsing discipline | Admit only the existing eligible GET or HEAD request shapes, for the exact guest route and passive read purpose. Keep destinations, rendered scripts and links subordinate to the broker and legal wall. A safe method label alone cannot authorise an action URL, form, session or credential. A redirect must re-enter the existing address and policy checks. | LV-R03, LV-R04, LV-R05, LV-R13 · [FX-0082-W5-PASSIVE-GUEST-READ-ONLY](../planning/feature-wave-5.md#acceptance-design-for-n-1623-to-n-1646) |
| N-1643 Live server-health brake | Use the existing site or host scope, current verified normal median and qualified observations. Apply the 0082 slowing and pause thresholds without an independent override. Stale or absent readings cannot increase rate. A block holds independently of measured latency. Resume only where the admitted scheduler and all more restrictive timers allow. | LV-R09 · [FX-0082-W5-QUALIFIED-HEALTH-BRAKE](../planning/feature-wave-5.md#acceptance-design-for-n-1623-to-n-1646) |
| N-1644 Exact wait-header obedience | Use the admitted valid wait and response profile and trusted scheduler clock. Pin stamped observation, exact host and scope, trusted clock and persisted deadline. Preserve all stricter host waits, shared limits, robots and stop. A hard opt-out holds independently of transient waits. Unknown clock or unqualified header cannot accelerate a retry. Restart or a second source cannot shorten a recorded wait. A block is a block, not a timed retry. | LV-R01, LV-R03, LV-R09 · [FX-0082-W5-STRICTEST-RESTART-WAIT](../planning/feature-wave-5.md#acceptance-design-for-n-1623-to-n-1646) |
| N-1645 Shared-server IP awareness | Use the existing verified address and shared-server admission rule to enforce the shared polite limit across eligible hosts. Site caps and individual waits still apply. Share only the stricter cap from already admitted current address evidence. Unknown or changed grouping preserves reservations and cannot reset overlapping holds or trigger probes. No extra primary-source lookup, origin probing, IP publication or restriction evasion. | LV-R05, LV-R09 and DR-061 · [FX-0082-W5-CURRENT-SHARED-SERVER-CAP](../planning/feature-wave-5.md#acceptance-design-for-n-1623-to-n-1646) |
| N-1646 Clean per-file-type byte caps | Pin the admitted mode and response class and its restrictive type, compressed and decompressed limits, with cumulative streaming admission for both compressed and decoded bytes and the host and site budgets. Refuse an inconsistent response type. At the exact cap prove EOF before declaring completeness. Otherwise mark truncation and withhold full directive coverage. A short or cut response cannot become complete evidence or be handed to a more permissive path. Source example sizes remain proposed. | LV-R08, LV-R11, LV-R13 · [FX-0082-W5-TYPED-BYTE-CAPS-WITH-EOF](../planning/feature-wave-5.md#acceptance-design-for-n-1623-to-n-1646) |

QR-209 qualifies the protected or synthetic proofs for directives,
identity, privacy, ledger, stops and type and mode bounds. The one-off
public routes keep their own authority and caps. No part introduces
authenticated acquisition, vulnerability reproduction, offensive
automation or an additional testing route on a live host.

#### Crawl bounds and ordinary fetch evidence (N-1647–N-1659)

Use the existing crawler identity, locked mode, guest-only method
profile, legal wall, compliance ledger and request and byte limits.
Recheck the current site, project, host, rights, stop and ledger
generations at queue, dispatch and delivery. Keep the 0019 scheduler
and 0116 frontier and storage contracts. Report safe partial coverage
where evidence is missing. The following routes offer ordinary SEO
evidence from admitted fetches. They provide no additional authority
to visit a host, broaden retention or reopen a stopped crawl.

| Owner catalogue entry | Exact contract |
| --- | --- |
| N-1647 Infinite-loop and calendar trap stopper | Recognise repeated bounded frontier patterns from eligible observations. Report the sampled branch, exclusion reason, coverage and count or unknown. Apply existing exact seen rules and audit budgets; no repeated traversal or removal of a manual exclude. Distinct legitimate pages cannot be silently merged merely because they share a URL pattern. [FX-0082-W5-BOUNDED-FRONTIER-TRAP-COVERAGE](../planning/feature-wave-5.md#acceptance-design-for-n-1647-to-n-1670). |
| N-1648 Bandwidth-saving conditional refreshes | Use a currently eligible validator for the exact retained guest representation and request profile. A not-modified response can reuse only the verified version for that same scoped request after fresh rules and rights checks. If the body, validators, representation or current rights are unproved, leave coverage unknown and use a full request only through existing authorisation and budget. No body retention in Receipts. [FX-0082-W5-CONDITIONAL-REUSE-EXACT-REPRESENTATION](../planning/feature-wave-5.md#acceptance-design-for-n-1647-to-n-1670). |
| N-1649 Site-wide asset deduplication cache | Reuse shared asset bytes only within the same admitted Audit project and exact request and variant scope. Recheck grant, opt-outs, validators, policy, resource type and retention; keep matching bytes in the existing 0116 vault. Expired, changed or inaccessible evidence cannot satisfy a read. No cross-project cache or heavy asset fetch in Receipts. [FX-0082-W5-AUDIT-ASSET-EXACT-PROJECT-AND-VARIANT](../planning/feature-wave-5.md#acceptance-design-for-n-1647-to-n-1670). |
| N-1650 Quiet-hours scheduling | Use the existing local overnight setting for registered own-site heavy audits. Require a verified site time zone and explicit policy for ambiguous or skipped local time under 0019. Commit a bounded occurrence once. Require overlap with the allowed own-site off-peak window. Respect 0099 quiet, blackout and other no-work controls separately. Recheck window, approval, cap and stop at actual dispatch. Missed occurrences do not grant extra work. No claim about least traffic from the clock alone. [FX-0082-W5-LOCAL-OFF-PEAK-AND-QUIET-INTERSECTION](../planning/feature-wave-5.md#acceptance-design-for-n-1647-to-n-1670). |
| N-1651 Instant domain circuit breaker | Use the existing host stop, back-off and shared-server rules. Advance the authoritative stop generation before further dispatch. Bound and reconcile in-flight work; do not assume an issued request was cancelled. Report why the host or own-site crawl stopped. A block holds without retry. An eligible transient fault resumes only under the current owning rule. [FX-0082-W5-HOST-STOP-LATE-REPLY-FENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1647-to-n-1670). |
| N-1652 Crawl-budget simulator | Use the existing polite-sample estimator and approved local graph or eligible aggregate data. Pin estimate scope, host, sample size, windows and caps. Show bounded scenarios for observed traversal and possible duplicate or thin sections. The output is the crawler's model, with unknowns, and cannot assert a search engine's allocation, change a crawl budget or schedule extra traffic. [FX-0082-W5-SCOPED-POLITE-SAMPLE-ESTIMATE](../planning/feature-wave-5.md#acceptance-design-for-n-1647-to-n-1670). |
| N-1653 Independent DNS resolver with cryptographic verification | Use the existing admitted resolver and DNSSEC evidence profile. Record a safe validation class and scope, and distinguish proof of verification from unsigned, stale, invalid or unavailable evidence. Follow the owning resolver policy and refuse ambiguous addresses where it requires a hold. Every candidate still goes through the shared socket exclusion check. No new query destination or direct resolver access from a renderer. [FX-0082-W5-RESOLVER-PROOF-SEPARATE-ADDRESS-CHECK](../planning/feature-wave-5.md#acceptance-design-for-n-1647-to-n-1670). |
| N-1654 Dual-stack IPv4 and IPv6 auditing engine | Compare only approved own-site Audit connections for the same canonical URL, request and evidence revision and eligible window. No second-family read under Receipts or a one-off rival scope. Each family contributes its own evidence and must independently pass the fetch wall and count in the same budget. An unavailable family gives unknown or the exact observed failure. A host block never triggers another-family retry. [FX-0082-W5-OWN-SITE-DUAL-STACK-SHARED-BUDGET](../planning/feature-wave-5.md#acceptance-design-for-n-1647-to-n-1670). |
| N-1655 Internal-network safety firewall | Apply the shared 0107 address exclusion policy at the connection boundary for every candidate, redirect and broker request. Connect only to the checked public address for that exact request and site. An absent check, prohibited or changed destination, or unknown classification holds. Record the safe refusal category and dispatch no request to the blocked address. [FX-0082-W5-PUBLIC-ADDRESS-EXACT-CONNECTION-FENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1647-to-n-1670). |
| N-1656 Full redirect-chain forensic tracer | Record a bounded ordered chain of observed status, timing and safe allowed headers under the existing guest audit. Preserve the last permitted hop and stop reason. Page-level refresh is passive evidence and cannot force navigation. DR-109 supplied-only redirects and DR-110 protected staging remain separate. Follow a target only through normal per-hop method, site, address, robots and opt-out checks. Strip cookie and identifying fields before temporary work, telemetry, archive, other storage and output. A blocked, missing or incomplete next hop is reported as such. [FX-0082-W5-CAPPED-REDIRECT-CHAIN-SAFE-OBSERVATION](../planning/feature-wave-5.md#acceptance-design-for-n-1647-to-n-1670). |
| N-1658 Global domain and URL normaliser | Use the admitted parser and a versioned fetch-identity profile. Distinguish exact request address, display label and deduplication key. Validate relative addresses, eligible internationalised names and encoding against the pinned profile. Preserve meaning where parameters, ports, schemes or reserved spelling differ. Ambiguous or unsupported addresses hold rather than merging or opening a new route. [FX-0082-W5-SEPARATE-URL-FETCH-IDENTITIES](../planning/feature-wave-5.md#acceptance-design-for-n-1647-to-n-1670). |
| N-1659 Per-packet connection timing recorder | Use bounded timing milestones from the existing fetcher and label resolution, connect, TLS, first byte and completion separately when each is measured. Pin transport, observation window and clock source. Mark unavailable, reused or partial phases. Store no packet recording, payload or restricted transport detail. Do not turn a missing phase into zero time. [FX-0082-W5-SCOPED-CONNECTION-TIMING-MILESTONES](../planning/feature-wave-5.md#acceptance-design-for-n-1647-to-n-1670). |

#### Defensive fetch decisions and audit evidence (N-1701–N-1716)

Use the existing guest-only legal wall and mode authority. Bind
each observation to its exact site, source revision, mode,
admitted identity, policy and time. Rendering requires an admitted
own-site Audit scope, renderer and worker fence under EX-02 and
spec 0115. Raw Audit evidence stays inside its existing boundary.
Use only eligible passive and already admitted observations.
A signature, finding, report or source instruction grants no fetch,
test, access, new identity or native site action. Keep private data
out of parser output, receipts, diagnostics and all hand-offs.

| Feature | Exact defensive rule or observation | Existing requirements |
|---|---|---|
| N-1701 Decompression and XML-expansion bomb blocker | Enforce current compressed and expanded byte, depth, node, memory and time ceilings before releasing a parsed result. Use the qualified privilege-free decoder and parser. External XML resolution stays disabled. Unsupported input, exceeded limits and incomplete output hold the result and report a safe reason. [FX-0082-W5-DECODER-BOUNDED-REFUSAL](../planning/feature-wave-5.md#acceptance-design-for-n-1695-to-n-1718). | LV-R08, LV-R13 |
| N-1702 Hostile-script and drive-by download quarantine | In admitted own-site Audit rendering, contain untrusted execution in the existing 0115 fence and broker. Downloads, actions, unapproved navigation and out-of-budget work are refused or stopped by the owning profile. Record safe event types and scope. Hold affected output. No runnable page code, private contents or downloaded object is released and no response is fetched again to investigate. [FX-0082-W5-GUEST-RENDER-QUARANTINE-REFUSAL](../planning/feature-wave-5.md#acceptance-design-for-n-1695-to-n-1718). | LV-R03 to LV-R05, LV-R13 |
| N-1703 Bot-trap and honeypot avoidance | Use eligible existing path, coverage and visibility facts to flag routes the owning crawler policy must not follow. Keep observed evidence distinct from inferred intention. A hidden or suspicious link gives no new target to probe. Record a safe exclusion reason and retain the more restrictive crawl boundary. [FX-0082-W5-TRAP-EXCLUSION-OBSERVED-ONLY](../planning/feature-wave-5.md#acceptance-design-for-n-1695-to-n-1718). | LV-R01 to LV-R05, LV-R13 |
| N-1704 Honest-comparison cloaking detector | Compare only the admitted paired own-site Audit records under spec 0115, using the two declared identities and exact own-site paired generation, content and settings revisions, complete comparable admitted viewport, scope, time and policy. Refused or missing dependencies count as partial and cannot prove the pair. Show a supported difference or unknown. A difference alone proves no deliberate concealment. No impersonated crawler or visitor identity, rival probe, extra route or repeated acquisition follows. [FX-0082-W5-PAIRED-DECLARED-AUDIT-COMPARABILITY](../planning/feature-wave-5.md#acceptance-design-for-n-1695-to-n-1718). | LV-R03, LV-R04, LV-R10 |
| N-1705 Cryptographically signed audit artefacts | Bind each eligible scrubbed snapshot or report to its exact bytes, admitted install signer and purpose, site, capture, policy and source versions, observation time and coverage. Reuse 0065, 0116 and 0118 for signing, protected custody and current verification. A signature verifies recorded evidence and cannot approve data release or certify that a finding is true or current. [FX-0082-W5-SIGNED-AUDIT-CURRENT-SCOPE](../planning/feature-wave-5.md#acceptance-design-for-n-1695-to-n-1718). | LV-R06, LV-R07; 0116 LS-08 |
| N-1706 Compromised-site and spam-injection detector | Flag possible injected content from already admitted, scrubbed own-site Audit differences and eligible visible-page or structure observations. Separate evidence, coverage and hypothesis. Use the owning safe local finding and existing alert review. No active security probe, vulnerable-route reproduction, cleanup, site write or confirmed compromise by inference. | LV-R03 to LV-R05, LV-R07, LV-R13 · [FX-0082-W5-PASSIVE-COMPROMISE-INDICATOR](../planning/feature-wave-5.md#acceptance-design-for-n-1695-to-n-1718) |
| N-1707 Zero-trust parsing boundaries | Pass untrusted network bytes only through the admitted bounded worker for the chosen input class. Bind exact parser, version, scope and fence. Pass only typed scrubbed output across the boundary. Worker refusal, crash or unverified result withholds dependent evidence. No added permission, connection, code execution outside the admitted renderer or alternate parser path. | LV-R07, LV-R08, LV-R13 · [FX-0082-W5-VERIFIED-PARSER-PROJECTION](../planning/feature-wave-5.md#acceptance-design-for-n-1695-to-n-1718) |
| N-1708 Hard per-page resource quotas | Bind current admitted mode and page budgets, including nested work and brokered subresources. The stricter time, memory, byte, connection or subresource limit wins. Stop and fence further work out of scope and release safe partial status. A timeout cannot grant budget or allow a later projection to claim completeness. | LV-R08, LV-R09, LV-R11, LV-R13 · [FX-0082-W5-SHARED-PAGE-QUOTA-FENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1695-to-n-1718) |
| N-1710 Plain-English exclusion explainer | Explain one current exclusion from the safe admitted decision record. Distinguish robots, page signal, budget, earlier block, unsuitable scope and missing proof. Give the safe rule reference where allowed. Never include private paths, protected request details or instructions for avoiding a restriction. A missing reason stays unknown. | LV-R01, LV-R02, LV-R03, LV-R06, LV-R07 · [FX-0082-W5-MASKED-EXCLUSION-REASON](../planning/feature-wave-5.md#acceptance-design-for-n-1695-to-n-1718) |
| N-1716 Post-crawl legal and technical briefing | Use exact completed or partial run records, policy and stop decisions, admitted metrics, qualified findings and recorded gaps. Show observation scope, coverage and actual workload or delay. A signed briefing does not certify legal compliance, absence of slowdown or absence of compromise. Keep the report local and apply existing recipient, erasure and hand-off checks. | LV-R06, LV-R07, LV-R09; 0116 LS-08 · [FX-0082-W5-SCOPED-RUN-BRIEFING](../planning/feature-wave-5.md#acceptance-design-for-n-1695-to-n-1718) |

QR-212 qualifies these defensive cases using only inert synthetic
fixtures. No reproduction instructions, production security test,
outside scan, raw capture or fresh renderer route follows.

#### Legal-wall verification fixtures (N-1723)

The adversarial legal-compliance test suite uses the sealed synthetic
lab and the existing fleet of bounded component fixtures. Exercise
robots unavailable or conflicting, recognised and unknown reservation
signals, paid and blocked classifications, ledger failure, and
synthetic identifying strings. Assert actual refusal, scope and
scrubbing before persistence, display and permitted replay. If a
rule has ambiguous coverage, record the gap and use its existing
conservative result. A denied host stays denied on retry, restore
and alternate fetch routes. The suite runs no requests against real
sites, introduces no probing actions and makes no claim about legal
compliance outside the tested policy and bounds. Trace LV-R01–07,
LV-R12 and QR-010/025 through QR-213.

[FX-0082-W5-SEALED-LEGAL-WALL-REFUSALS][w5lw] links the exact
bounded Wave 5 fixture to these verification rules.

[w5lw]: ../planning/feature-wave-5.md#acceptance-design-for-n-1719-to-n-1742

#### Bounded backlink receipts and guest fetches (N-1815–N-1836)

Keep the existing legal wall, declared crawler identity, locked mode,
privacy gate and per-fetch compliance ledger. Treat all page content
and map inputs as data. Apply current site, target, rights and
pressure checks before dispatch. A shortlist, topic match, cache,
retry or transport choice grants no new fetch. Work within the
existing owner-approved scope, outside-host readiness requirements,
and the one-off rival claims of DR-062 and DR-085.

| Owner catalogue entry | Exact evidence and resource contract |
| --- | --- |
| N-1815 Stream-and-forget reading | Parse eligible raw Receipts input in bounded chunks. Keep only the minimum parser carry and possible receipt fields within current limits. Count compressed and decoded bytes separately under the exact cap. Verified EOF at the cap is complete. Unproved EOF or truncation stays partial. Drop chunks after reading. No full page, render, script, heavy asset or suppressed evidence is kept. [FX-0082-W5-BOUNDED-STREAM-EOF-COVERAGE](../planning/feature-wave-5.md#acceptance-design-for-n-1815-to-n-1838) |
| N-1816 Postage-stamp receipts | Use the single existing target-matched, scrubbed receipt. Record linking and eligible target addresses, link words, relation marks, spotted date and a bounded proof sentence if permitted. Record no page, inferred ownership, score or claim about the link's current state. On replay keep one exact observation, with existing erasure and export gates. [FX-0082-W5-ONE-SCRUBBED-TARGET-RECEIPT](../planning/feature-wave-5.md#acceptance-design-for-n-1815-to-n-1838) |
| N-1817 Cheat-code shortcut using open web maps | Use existing 0040 licensed or supplied owner-approved link-map evidence only for a bounded, dated lead list. No hosted address submission. Record its scope and source. A map row is not a confirmed link or authority. Visit only an exact eligible shortlisted page under fresh guest permission and budget. Rejected leads and unsupported map access give a gap, with no arbitrary scan or provider route. [FX-0082-W5-LICENSED-LEAD-SCOPE](../planning/feature-wave-5.md#acceptance-design-for-n-1815-to-n-1838) |
| N-1818 Tiny math checklists instead of huge URL lists | Use the admitted bounded probabilistic seen filter for Receipts, with the current visit or reservation identity. A possible false positive is a coverage loss, not proof that the page was read. Qualify its size and false-positive rate. Preserve exact restart and one-off rival claims. Audit continues to use the exact frontier of 0116. [FX-0082-W5-BOUNDED-RECEIPTS-SEEN-FILTER](../planning/feature-wave-5.md#acceptance-design-for-n-1815-to-n-1838) |
| N-1819 Waiting line lives on the drive, not in RAM | Use the registered own-project disk frontier and bounded working window. Bind admitted volume, scope, current counters and checkpoint. On restart reconcile the durable queue before claiming new work. Full or missing storage holds safely. No spill into another project, unmanaged disk, network storage or an unlimited memory queue. [FX-0082-W5-DURABLE-SCOPED-FRONTIER](../planning/feature-wave-5.md#acceptance-design-for-n-1815-to-n-1838) |
| N-1820 Hard memory fence and polite speed brake | Apply the Receipts ceiling and spec 0078 pressure controls to the complete crawl working set. Slow or pause before admitting unsafe work. Keep host politeness, shared-server cap, back-off, stop list and block independent of the PC's free memory. Recovery resumes only after all current authority and resource checks pass. [FX-0082-W5-INDEPENDENT-MEMORY-AND-HOST-HOLDS](../planning/feature-wave-5.md#acceptance-design-for-n-1815-to-n-1838) |
| N-1821 Simple time and space presets | Map the existing Quick scout, Niche sweep and receipt-space presets to the existing 5-minute, 20-minute and 50 MB defaults and exact contract units. The first applicable time, byte, disk, memory or legal limit wins. Show the stop reason and coverage. Changing a preset cannot expand the target set or lift existing limits. [FX-0082-W5-FIRST-APPLICABLE-PRESET-LIMIT](../planning/feature-wave-5.md#acceptance-design-for-n-1815-to-n-1838) |
| N-1822 Start inside the neighbourhood | Start a scoped niche scan from the existing bounded seed set of eligible public hubs. Record seed provenance and inclusion rationale. Recheck each host and distinguish approved own or non-rival scope from a registered rival. Rival checks require owner-started exact URL and durable pre-open one-off claim. Unknown or spent holds. Redirects and canonical aliases cannot repeat an observation. No queued or repeated rival inspection. Starting from an eligible hub never authorises an off-topic or unauthorised destination. [FX-0082-W5-ELIGIBLE-ONE-OFF-SEED-CLAIM](../planning/feature-wave-5.md#acceptance-design-for-n-1815-to-n-1838) |
| N-1823 Follow the scent, drop the clutter | Prioritise only already eligible niche leads using admitted anchor, address and nearby-sentence cues. Treat unknown topic or incomplete coverage explicitly and record safe reasons for exclusions. A topic score changes queue order within existing politeness and budget. It cannot confirm link quality, bypass robots, or start broader discovery. [FX-0082-W5-BOUNDED-ELIGIBLE-LEAD-PRIORITY](../planning/feature-wave-5.md#acceptance-design-for-n-1815-to-n-1838) |
| N-1824 Finding the most-linked site in a niche with tiny memory | Use the existing bounded distinct-referring-site estimation profile over eligible observations. Pin normalised source-site and destination identities, sampling and approximate evidence. Repeated links from one site do not count as independent sites. Label coverage, uncertainty, ties and optional large-platform filter. Report only observed link evidence, with no rank input, outreach, quality or wider-web leadership claim. [FX-0082-W5-DISTINCT-REFERRING-SITE-SAMPLE](../planning/feature-wave-5.md#acceptance-design-for-n-1815-to-n-1838) |
| N-1834 Strict robots.txt rule engine | Apply the pinned parser and existing conservative fetch and cache policy for the exact host, declared identity and current rule version. Honour qualified wildcard decisions. For overlong robots read the first admitted 500 KiB and report truncation. Preserve distinct 401/403 and unreachable or error holds under the existing rule. If rules are unavailable, stale beyond admission or an error counts as deny, hold affected dispatch. Preserve robots evidence in the compliance ledger, including each later redirected host. [FX-0082-W5-EXACT-HOST-ROBOTS-AND-REDIRECT-POLICY](../planning/feature-wave-5.md#acceptance-design-for-n-1815-to-n-1838). |
| N-1835 Clean-room HTTP/1.1, HTTP/2, and HTTP/3 engine | Qualify mature admitted transport components separately with pinned synthetic fixtures. Preserve the same guest method, legal wall, identity, accounting, stop and trust checks across supported versions. Unsupported or unproved variants report their scope. Apply LV-R05 and 0107 separately at selected connection and each redirect. No fallback or alternate route after destination denial. Other protocol fallback follows only the existing eligible policy. [FX-0082-W5-ADMITTED-TRANSPORT-SAME-LEGAL-WALL](../planning/feature-wave-5.md#acceptance-design-for-n-1815-to-n-1838). |
| N-1836 Clean-room TLS 1.3 encryption and certificate engine | Use admitted audited cryptographic and transport components. Bind exact peer, expected name, trust profile and certificate outcome to the protected connection. Hold on invalid or unproved required trust or revocation evidence and show a safe reason. Header observations are separate facts. Recheck 0107 and LV-R05 at actual socket and each redirect. No unverified downgrade, trust override or alternate route after deny. [FX-0082-W5-TLS-EXACT-PEER-AND-SOCKET-POLICY](../planning/feature-wave-5.md#acceptance-design-for-n-1815-to-n-1838). |

#### Peer leaf expiry for existing own-site Audit captures

For F-0164, QR-110 must admit a bounded projection from N-1836's
verified connection in an already permitted registered own-site
Audit capture. Apply the existing address and policy checks at the
socket and each redirect. Match the exact HTTPS origin, including
effective port, registered site and verified peer name. A redirect
target supplies evidence only for its own admitted origin.

`PeerLeafExpiry` carries site ID, origin, source capture ID, connection
proof digest, transport and trust profile revisions, grant generation,
verified observation instant, leaf certificate SHA-256 over its DER
bytes, leaf validity end instant and protected custody reference.
Instants use UTC and preserve the source precision. The accepted
server leaf supplies both digest and expiry. Trusted intermediates,
response headers, cached bodies and refused handshakes supply neither.
A reused connection without exact current proof of the accepted leaf
cannot refresh its observation. Replay preserves the original time.
Scope, trust, revocation, parsing or storage proof failure withholds
the record. Keep only the bounded projection under the existing
protected crawl retention and erasure rules. Export or log no raw
certificate, chain, address of the peer socket or trust-store data.
No fresh connection, follow-up probe or alternate trust route.

QR-217 qualifies frontier and sampling bounds, exact scope and budgets,
safe network and retention behaviour, and guarded receipts and replay.

Apply DR-079 through 0020 and 0100. Page addresses are outbound
content and cannot leave through Headline, an agent token or a
workflow. Standard and Full are owner-only, and Full needs the
site-text switch. A deliberate LE-14 local export grants no hosted
upload, default egress or provider map query with page addresses.

### Data

Receipts and the frontier are files in the home folder (spec 0030) with ledger
entries. Audit facts stay in PostgreSQL as in spec 0044. The compliance ledger
is a file with chained hashes plus a searchable mirror table.

### Settings

All numbers here are proposed until measured on Dell.

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `leviathan.rate_per_second` | requests per second | 2 | 1 to 8 | between requests |
| `leviathan.receipts.memory_mib` | MiB | 300 | 100 to 500 | next job |
| `leviathan.receipts.page_ceiling_kib` | KiB | 512 | 64 to 512 | next job |
| `leviathan.receipts.snippet_chars` | Unicode scalar values | 160 | 0 to 160 | next job |
| `leviathan.receipts.disk_cap_mb` | MB | 50 | 1 to disk cap | next job |
| `leviathan.quiet_hours` | local hours | 00:00 to 06:00 | any window | at once |
| `leviathan.hide_mega_platforms` | on or off | on | two values | next scan |
| `leviathan.shared_host_limit` | on or off | on | two values | at once |

DR-059 sets the context ceiling. Refuse a new setting above it.
A previously saved explicit value above it needs the normal 0014
invalid-setting repair before this receipt path can resume. Keep
the original settings history. No silent clamp or new crawl follows.

There is no setting that turns off robots.txt, opt-out signals, the no-bypass
wall, the private-address block or the compliance ledger.

### Failure codes

This crawler reuses SUT-650, SUT-651, SUT-652 and SUT-654 from spec 0044.
SUT-652 (private address refused) is owned by spec 0044. SUT-653 (log
format not understood) does not apply to crawling and is not reused.

| Code | Meaning |
|---|---|
| SUT-4300 | Refused by robots rules |
| SUT-4301 | Refused by an opt-out signal |
| SUT-4302 | Block page received and host stopped |
| SUT-4303 | Signing key problem |
| SUT-4304 | Not used: a private address is refused with SUT-652 |
| SUT-4305 | Erasure failed and nothing was partly deleted |
| SUT-4306 | Compliance ledger could not be written and crawl stopped |
| SUT-4307 | Resource fence reached |
| SUT-4308 | Mode cannot change during a job |

Any failure leaves the crawl stopped or paused without a partial write.

### Security and privacy

- Every network call goes through the fetcher and the legal wall. A build-time
  ban refuses any other network call (spec 0033).
- Page content is untrusted data and never instructions, including any text
  that looks like an instruction to the crawler.
- No secrets are used: guest view only.
- Private data never reaches disk unscrubbed.

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| LV-R01 | Leviathan MUST fetch and obey robots.txt for every host, and MUST hold back when it cannot be reached. | UT-LV-01, FZ-LV-01 |
| LV-R02 | It MUST obey page and header signals and AI and text-mining opt-outs before storing anything. | UT-LV-02 |
| LV-R03 | It MUST have no code path that solves a CAPTCHA, guesses a login, passes a paywall or avoids a block. | AR-LV-01, BD-LV-01 |
| LV-R04 | It MUST send only GET and HEAD, and MUST NOT keep cookies between pages. | UT-LV-03 |
| LV-R05 | It MUST refuse private, loopback, link-local and metadata addresses, including after redirects, using the shared `is_blocked` function and range table of spec 0107. | UT-LV-04, FZ-LV-02 |
| LV-R06 | Every fetch MUST have a compliance ledger entry; if it cannot be written the crawl MUST stop. | UT-LV-05, FI-LV-01 |
| LV-R07 | Personal identifiers MUST be removed before text is written to disk. | UT-LV-06 |
| LV-R08 | Receipts mode MUST stay inside its memory fence, keep no page, and cut a page at the ceiling. | PT-LV-01, FI-LV-02, UT-RN-26 (spec 0115) |
| LV-R09 | It MUST honour the politeness rules, including the 10% slow and 25% pause rule. Share host claims and waits between routes and sites. | UT-LV-07, BD-LV-02, IT-0152-SHARED-WAIT |
| LV-R10 | It MUST use its declared identities and never another company's name. | UT-LV-08 |
| LV-R11 | A mode MUST NOT change during a job. | UT-LV-09 |
| LV-R12 | Erasure MUST fence access atomically. Applicable store removal needs verified completion and a receipt. Partial cleanup MUST keep the fence and record that completion is unknown. | IT-LV-01 |
| LV-R13 | Every untrusted byte MUST pass a privilege-free parser with limits. | FZ-LV-03, FI-LV-03 |

## How it is tested

Designed, not run. Fixtures are a synthetic mini-internet of made-up sites
served locally.

- **Unit (UT-LV):** robots rules with wildcards, 401/403 as deny, the 500 KiB
  cut, signal precedence, GET and HEAD only, address filters with redirects to
  private ranges, ledger entry per fetch, scrubbers for each identifier type,
  the slowing thresholds at exactly 10% and 25%, identity strings, mode lock.
- **Fuzz (FZ-LV):** robots files, headers, HTML, compressed bodies and URLs from
  mangled inputs never crash, hang or escape limits.
- **Fault injection (FI-LV):** the ledger cannot be written, memory pressure
  rises mid-crawl, the connection drops mid-chunk, a parser worker is killed.
- **Property (PT-LV):** receipts mode never exceeds its memory fence; the
  visited filter never repeats a page; the frontier survives a crash.
- **IT-LV-RIVAL:** restart with a saved rival already due for recheck.
  Check background, retry, duplicate and alias paths make no new read.
  A still-unused owner-started one-off check may proceed once.
- **Behaviour (BD-LV):** Rana's crawl above; a block page stops a host; a site
  that asks goes on the stop list.
- **Adversarial compliance (AR-LV):** conflicting opt-out headers, malformed
  robots files, hidden paywalls, disguised personal data, and a source tree
  check that no bypass code path exists.
- **Integration (IT-LV):** erasure across snapshots, caches, backups and
  indexes; a crash during an audit crawl resumes without fetching a page twice.
- **Measurement:** memory, speed and receipt size on 8 GiB and 16 GiB machines;
  the filter's real false-positive rate.

## Changes from the source

Specs 0044 and 0045 held the crawler's identity, robots, politeness and block
rules; those paragraphs now point here. The identity name
`SutradharHealthCheck` becomes Leviathan's two identities. The ban on keeping
anything but link words is replaced by the scrubbed snippet. Receipts mode,
niche scans, the compliance ledger, the privacy gate and the legal wall are
new. The no-script rule of spec 0044 stays until the rendering engine is chosen.
