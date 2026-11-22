# 0141 Crawl, log and Technical Toolbox

## What it does

A site owner can inspect an existing site crawl, compare two eligible
captures, import access logs and run individual checks from the Technical
Toolbox. Each finding shows its source, what was checked and where evidence
is incomplete. The parent audit spec owns shared packaging, protected
storage, resource leases, export, UI conventions and the separately approved
paid Local Finder or Maps contract. Spec 0141 owns the evidence projections
and checks below. Specs 0044, 0082, 0107, 0115 and 0116 keep authority over
acquisition, permissions, transport and storage.

## Route, authority and limits

The desktop opens the job preview for work that reads a site or imports a
file. It shows site, purpose, source, profile, scope and applicable bounds.
The current site role, read grant, purpose and pause generation are checked
before work, at dispatch and before delivery. MCP lists or reads only
allowed projections through 0020. A tool cannot grant live acquisition,
choose an identity, approve a provider or publish a result. File imports
remain local. Existing authorised own-site crawl schedules keep their scope.
Every new live comparison needs one human approval and has no schedule.

Offline checks consume supplied bytes and existing evidence. Guest work uses
the current broker and scope rules, GET/HEAD, public HTTPS where required
below, declared identities only, per-host robots, resolved and connected
address checks, redirect fences, budgets and durable compliance intent. Stop
on blocks and challenges. A draft robots file cannot affect a live decision.
The owner-approved staging route accepts a mapped local export under DR-110.
Locale comparison accepts supplied captures only. No staging credentials,
remote locale switch, third-party identity or rival monitoring route exists
here.

Use the minimum of the parent job lease and each owning parser, transport,
storage and rendering cap. 0044 F-0113/F-0114/F-0116 and 0082 set crawl
scope, speed, concurrency and holds. 0044 already caps a response at 5 MiB
decompressed. The live pair reserves two main-page admissions under the
existing limits and a positive browser-site cap before either leg. Robots
checks, redirects, timeout, total bytes and optional already admitted
subresources use the existing broker caps. Use one pair in flight per site,
both legs sequentially. Decline before dispatch when reservation fails. Use
an actual admitted 0115 render path for the Browser leg. Capture and fence
both main observations before any script executes. This pair uses the
admitted static stage with scripts off and grants no screenshot or
subresource dispatch. If that path cannot consume the exact brokered main
response without extra requests, hold the pair. Raw content and derivative
surfaces appear only at the level admitted by 0082/0115. Script, paint and
browser-dependent observations require exact EX-02 and cage proof. Never
relabel raw or partial evidence as a completed render.

The first offline Toolbox profile accepts at most 1,000 targets or source
rows per operation, 256 KiB per text rule document, 5 MiB decompressed per
XML/feed document and 4 KiB per URL/path field, subject to lower owning
limits. Stop before processing an over-limit input and make the missing
scope explicit. For streamed access logs use at most 64 KiB per line, one
line buffer plus bounded parser state, and the parent byte, row, memory,
time and disk leases. Discard and count an over-limit line without buffering
the excess. A file or dialect with unrecoverable framing stops at the last
valid checkpoint. This is a design profile and requires measurement on each
admitted machine class.

## Capability and rulebook

| Rule and scope | Required behaviour and outputs | Synthetic acceptance design |
|---|---|---|
| CR-01 Seed crawl | Under 0044/0082, use registered own exact host/subdomain/path scope, first-party eligible seeds and approved sitemap origins. Normalise by the 0044 URL rules. Intersect bounded depth, include/exclude and admitted parameter policy with the registered scope. Display omission and stop reasons. | CR-01-A seed inside folder, outside sibling, depth boundary, denied host and duplicate normalised key. Only eligible frontier claims occur. |
| CR-02 List crawl | Accept pasted/local URL list, eligible XML sitemap or approved local backlink receipt URL references. Validate site and lease per seed. Only listed targets and separately authorised redirect hops may be read. Reject unadmitted file kinds. | CR-02-A repeat a URL, add wrong-site receipt and discovery href. Claimed keys are unique and no discovery expands the list. |
| CR-03 Saved comparison | Compare two saved, still-readable own-site captures of compatible source, configuration, fields and coverage. Full outer merge on the owning URL key. Emit added/removed only when opposite coverage proves absence, changed fields by same stage/rule version, issue deltas with each denominator and unknowns otherwise. DR-110 staging uses the approved mapping. | CR-03-A complete and partial twins with status, metadata and render changes, disappeared row and staging mismatch. Separate proved changes from unknown. |
| CR-04 Segments | Use versioned, bounded URL and evidence predicates with explicit membership and overlaps. Unknown membership stays unknown. Report unique affected URLs and assessed denominators per rule and category, with explained priorities from the owning view. | CR-04-A overlapping predicates, zero denominator and missing HTML. Emit scope and counts without an overall, segment or category composite health score. |
| CR-05 HTTP and links | Keep observed status family and response class, inlink/outlink direction, source/destination, occurrence and observed redirect hops. Prove loops by repeated normalised hop. An external link can be listed from observed HTML without fetching it. Unobserved remote status remains unknown. | CR-05-A eligible chain, proved loop, blocked hop, duplicate href, external target and uncrawled destination. Preserve observed edges and hold remote verdicts. |
| CR-06 Page metadata | Show title, description, H1 and H2 counts, character lengths, missing/multiple, same-as-H1 and duplicates with exact rule version. Pixel widths require an admitted viewport/font projection. Capture HTML and HTTP canonical and robots independently. Report conflict, observed chain, non-indexable target, pagination and scope with linked evidence. | CR-06-A header/HTML conflict, empty heading, near-identical titles, refused destination and unavailable font proof. Keep contradictory sources and width unavailable. |
| CR-07 Site structure checks | Resolve hreflang, XML sitemap and pagination only through eligible sources. Distinguish indexable pages absent from a complete sitemap view, present but ineligible URLs, and possible orphans. Absence of links requires closed matching site-graph coverage. | CR-07-A complete sitemap and graph plus partial graph. Same omitted URL has proved sitemap absence and unknown graph status in the partial capture. |
| CR-08 Raw and render | Compare compatible approved raw, static, script and paint evidence for one page revision. Flag main content visible after scripts only when both relevant stages completed with the same profile. Retain observed blocked-resource reasons and permitted client redirects, with fragments as evidence rather than new HTTP requests. | CR-08-A static shell and synthetic completed script projection, blocked stylesheet, missing renderer and hash-only navigation. No substituted render or extra fetch. |
| CR-09 Text quality and accessibility | Use qualified canonical text for exact duplicate groups and qualified same-version near-duplicate threshold and grouping. Run only admitted language-specific spelling/grammar rules. Report atomic accessibility observations for alternative text, roles, heading order, labels and landmarks. Contrast needs qualified style/paint evidence. Advertise covered rules, never full WCAG conformance. | CR-09-A same and near variants, unsupported language, labelled fixture and paint missing. Same-version groups and rule-specific unknowns remain distinct. |
| CR-10 Capture extraction and saved search | Allow bounded pre-capture CSS, qualified XPath subset or bounded regular expressions under the owning parser. Post-capture search is over the exact separately retained scrubbed text or inert code surface under 0044 F-0184. Raw, rendered and visible projections stay distinct. Return stable occurrence coordinates and capped local results. Missing surfaces require a new separately authorised capture. | CR-10-A each eligible surface, unsupported selector, malformed expression, redacted region and unavailable raw. No further network use on search. |
| CR-11 Structure views | Draw crawl shortest-path and directory trees and the retained internal-edge view using 0116/Alekh. Root is the admitted seed, ties order by canonical URL key, member counts and filters describe the reviewed graph only. The internal-edge view uses the capped force-directed layout below. | CR-11-A cycle, two equally short paths, dangling node, boundary and one over each layout cap. Compare exact output coordinates and omission counts across admitted profiles. |
| CR-12 Enrichment | Join current eligible own-site GSC, GA4 and CrUX projections only through their owning scopes, matching surface, site, source grain, dated window and version. Use Matomo only from an approved supplied aggregate file, scoped to its declared grain and period. No inferred item-level match, provider call or source fallback on a missing join. | CR-12-A wrong period, unresolved aggregate grain and unavailable provider. Keep each metric and reason separate. |
| CR-13 Local prompt drafts | Per-segment apply authorised local Model Hub or separately fenced local worker template to minimum eligible text. Pin recipe, model, source revision and rules. Retain only the permitted reviewed draft in project scope. No prompt export, site publication, ranking path or external inference. | CR-13-A supported draft vs opted-out page, ambiguous data rights and unavailable model. Check local draft review and absence of side effects. |
| LG-01 Log intake | Use the exact offline log profiles below. Qualify each separately under 0044/0034. Profile choice grants no connection. | LG-01-A accept the exact synthetic Combined record and its dash-for-zero variant. Try changed order, ambiguous timezone, ambiguous site, overlong line, invalid escape and truncated tail. Never guess an unsupported dialect. |
| LG-02 Streaming and custody | Use bounded sequential parser, bounded rows/field lengths and disk range indexes under 0116 and shared lease. Pin exact file identity, parser, offset and parser state at each durable checkpoint. Discard identifying data before release or local analytic storage. The source stays owner-controlled. Resume only from proven checkpoint and refuse changed bytes. | LG-02-A huge generated fixture under both admitted baselines, oversized line, disk-full, pause during a partial record and file replacement. Peak and offsets have proof, no duplicate record after valid resume. |
| LG-03 Bot and coverage | Project bot label as verified, claimed, unknown or disproved only from qualified provenance, public lists or owner-supplied verification evidence. Preserve trusted-origin and proxy caveats. Aggregate by site, date, URL/segment, bot class and status using a dated denominator, with errors, known discoveries and crawl frequency. Distinguish unvisited within window from never visited and candidate waste from a finding. | LG-03-A spoofed claim, trusted supplied proof, proxy ambiguity, 4xx/5xx and a partial capture. Show measured counts and withhold absent-window claims. |
| TB-01 Robots | Test supplied draft/live-capture bytes against declared profile tokens in memory. When a live check is explicitly eligible, use the broker's existing live decision and resource inventory. Evaluate only known admitted target and CSS/JS/image/font paths. Show source, exact eligible matching line and rule precedence; retain incomplete resource discovery as coverage unknown. | TB-01-A allow, disallow, equal precedence fixture, unavailable live source, draft different from live and resources omitted. Draft cannot alter any transport state. |
| TB-02 AI access | A versioned sourced bot-token registry drives offline robots-policy simulation only. Show distinct policy result, observed honest-identity status/header, and eligible raw main-text visibility versus saved comparable render. A simulated decision says nothing about service response to that bot. No identity impersonation or unsupported coverage count. | TB-02-A token group allow with legitimate blocked live observation, unrecognised token and script-only visible fixture. Keep stages and their unknowns separate. |
| TB-03 Rewrite simulation | Parse declared offline dialect into ordered literal or bounded regular-pattern path rules, deterministic fixed captures and local target substitution. Apache, NGINX and Caddy each need an exact qualified subset manifest. Reject the whole input for an unknown directive, flag, conditional, map or evaluation context. Emit simulated status/hops, an eight-hop stop or a proved earlier loop. Only declared rules can run. Resolve no address and deploy no config. | TB-03-A ordinary path match, loop, invalid expression, unsupported rule and large bulk request. No transport action exists. |
| TB-04 Fetch and render | Show separate saved HTTP/source, DOM, paint, console summaries and blocked resources using CR-08. Start an eligible existing own-site Audit fetch only with a scoped owner grant and exact admitted profile. Keep browser capacity, screenshot and source custody checks. User-agent input offers no free text. | TB-04-A supported saved raw, script-off path, approved paint fixture, capture refusal and unsupported stage request. Each stage shows its own reason. |
| TB-05 Declared response pair | One owned public HTTPS canonical page and one job. Compare two complete pre-script main observations. The Leviathan-Browser leg obtains its main response through the actual admitted 0115 render path. Apply the signed generation and acquisition contract below. Show eligible field matches or observed differences. Never infer concealment, search treatment or cause from a partial or different-profile observation. | TB-05-A equal pair, changed eligible field, third-party identity, changed footer settings, changed generation, closed robots on second identity and withdrawn approval. No claimed comparable pair on refusal. |
| TB-06 Mobile | Compare compatible owned saved mobile/desktop captures only under 0115 mobile-first profile and admitted optional desktop. Show main text/links/metadata/canonical/hreflang/structured data and qualified viewport, target size, font legibility and overflow measurements. Bulk uses the same admission per row, without converting a simulated view to a search verdict. | TB-06-A compatible twins, unsupported viewport and changed revision. Field diff only where comparable. |
| TB-07 International | Validate eligible saved HTML, header and XML hreflang declarations with 0047 roles. Show language/region syntax, duplicate/canonical conflicts and observed return-tag edges. Derive a local graph from admitted edges only. Compare locale-labelled supplied snapshots with exact origin/page, compatible captures and declared locale provenance. | TB-07-A return pair, missing target coverage, wrong locale source and revision drift. No header change or remote request follows. |
| TB-08 Structured data | Use the single 0044 F-1316 validator and 0115 RN-22 visible evidence. Offer the exact local Event, Article and BreadcrumbList builder profiles below. Emit only inert draft JSON-LD. | TB-08-A covers each admitted profile, absent optional and required fields, boundary and one over for each limit, malformed input, visible mismatch and withdrawn evidence. Keep syntax, vocabulary, policy and visibility separate. No route contact, context resolution or publication. |
| TB-09 XML and feeds | Build offline XML sitemaps and explicit locale alternates from approved CSV/Alekh owned rows. Parse allowed RSS/Atom subsets in networkless streaming mode. Keep output deterministic by canonical URL/locale key and do not resolve entities, links or external resources. | TB-09-A canonical alternates, duplicate rows, invalid dates, unsupported entity and truncated feed. Emit only accepted local output with coverage. |
| TB-10 Snippet and local preview | Draw local desktop/mobile model previews under dated qualified font/width assumptions; supported date, breadcrumb, FAQ and review fields are illustrative. No search result prediction. Paid Local Finder and Maps use only the exact 0143 provider, grant, purpose, charge and result contract. | TB-10-A clipping with pinned font, conditional rich field, unsupported size and unpaid/local-only path. No optional provider invocation follows from local preview. |
| TB-11 AMP and tracker | Use an offline qualified AMP or legacy rule profile for supported saved HTML. List its coverage and partial outcomes. Link to eligible 0137 tracker records by date/source only, without source contact or causal claim. | TB-11-A valid, invalid, unsupported dialect and tracker unavailable. No live diagnostic or rank interpretation follows. |

## Force-directed internal-link view

Use only directed, deduplicated internal edges of the authorised
capture, ignoring self-edges for the layout. Apply the user's segment
and visibility filter first. Choose at most 256 eligible nodes by
canonical key order, then at most 1,024 directed edges joining two
selected nodes, in source-key and destination-key order. Preserve both
directions if present. Show displayed and omitted counts for the
eligible view, or unknown where the source count is incomplete. A cut
never proves a missing link. Keep original edge direction and attach
only permitted evidence references. Selecting a node opens its saved
URL details; no network action follows.

Use signed integers and one versioned layout. Set node index k from
zero in canonical-key order. Initialise coordinates to
(128 × (k mod 16), 128 × floor(k / 16)). Run exactly 64 synchronous
rounds. For each unordered node pair, take dx and dy from first to
second and let d be max(1, abs(dx) + abs(dy)). Add a repulsion of
(-trunc(256 × dx / d²), -trunc(256 × dy / d²)) to the first and its
opposite to the second. For equal coordinates apply (-1, 0) to the
earlier node and (1, 0) to the other instead. For each unordered
connected pair add attraction (trunc(dx / 16), trunc(dy / 16)) to
the first and its opposite to the second. Sum using signed 64-bit
integers. Clamp each total displacement to ±32 and each resulting
coordinate to ±4,096. Truncation is towards zero. Draw directed arrows
from the original selected edges. The same layout version and inputs
produce identical coordinates. Empty data shows its coverage reason.
A single node stays at the origin. Cancel between rounds and withhold
an incomplete layout. Apply the parent's job lease and erasure checks
to the local projection. Pan and zoom affect only the view.

## Inert JSON-LD builder profiles

Each shape uses the fixed Schema.org context identifier and the exact
admitted vocabulary edition. The builder emits one locally escaped,
inert JSON-LD object with deterministic field order, at most 64 KiB
UTF-8. It accepts no manual JSON, arbitrary types or property names.
Trim and normalise text to NFC before measuring Unicode scalar counts,
reject control or bidi control characters, and omit blank optional
fields. Counts below include the endpoints. Date-time fields use a
valid Gregorian ISO date and time to whole seconds and an explicit
numeric UTC offset or Z, with no implied timezone. Compare dates as UTC
instants. URLs must be eligible, previously approved owned HTTPS
references, without user info, query or fragment and at most 2,048 UTF-8
bytes. Selecting a URL does not fetch it. If the site, rights, shape,
source or exact profile cannot be verified, withhold the draft.

| Object | Exact fields and bounds |
|---|---|
| Event | Required name 1–200 scalars, startDate and location. Optional endDate cannot precede startDate. Location is Place with name 1–200 scalars and optional typed address, or PostalAddress directly. A present address requires streetAddress 1–200, addressLocality 1–100 and addressCountry as an admitted 2-letter ISO country code. Optional addressRegion 1–100 and postalCode 1–32. Accept no other field. |
| Article | Require one headline of 1–200 scalars in the local form. Accept zero to eight author nodes, each a Person or Organization with a permitted public name of 1–120 scalars and optional eligible URL. Accept zero to eight distinct eligible image URLs in supplied order. Optional datePublished and dateModified; when both are present dateModified cannot precede datePublished. Omit absent optional fields. Do not infer a name, date, image or article from a missing source. |
| BreadcrumbList | Require 2–20 ordered ListItem rows and a name of 1–120 scalars per row. Assign position 1 to n from the supplied order. Each nonterminal row needs its distinct eligible item URL. The last row may omit item or use its distinct eligible URL. Reject duplicates, user-supplied positions and blank or unsupported required fields. |

The field limits are local form constraints. Article headline is a
local form requirement. Google's Article page treats its documented
fields as recommendations. The selected BreadcrumbList subset uses
positional items and the documented allowance for the final item URL
to be omitted. A local draft or pass carries no search-display promise.
Compare finished-page facts only through separately admitted RN-22
evidence. On TB-08-A, test an Article with only its headline and a
BreadcrumbList with the second item URL absent. Both can produce a
draft. Fail closed on a first breadcrumb with missing item, 21 items,
a repeated item URL, a ninth author or image, an inverted date pair
and a blank required name. A withheld visible comparison does not
rewrite the draft. Keep field-local reasons and exports within the
existing approval and custody rules.

[Schema.org Article][builder-article] ·
[Schema.org BreadcrumbList][builder-breadcrumb] ·
[Google Article guidance][builder-garticle] ·
[Google Breadcrumb guidance][builder-gbreadcrumb]

[builder-article]: https://schema.org/Article
[builder-breadcrumb]: https://schema.org/BreadcrumbList
[builder-garticle]: https://developers.google.com/search/docs/appearance/structured-data/article
[builder-gbreadcrumb]: https://developers.google.com/search/docs/appearance/structured-data/breadcrumb

## First offline access log profile

Admit Apache HTTP Server 2.4 Combined, one owner-attested single
owned virtual host per file. The manifest pins the site, file digest,
byte length, exact producer and format revision, grammar and field
manifest digest, escaping, timestamp format and basis, byte metric,
source window, rights and capture date. Repeat the pinned profile in
the import receipt, cursor and each aggregate. Resume requires the same
file and profile. An absent optional metric is unknown; an unsupported
or ambiguous input format is refused.
Use the Apache Combined record order: remote host, remote logname,
remote user, bracketed time, quoted request, final status, response
size, quoted referrer and quoted user agent. Fields have a single
space between them. Require LF or CRLF framing and a terminated line.
Parse the English month names and the exact numeric UTC offset from
the request-received timestamp. Require a valid date, clock and offset.
Store UTC, the timestamp basis and the declared source window.
A missing time or offset rejects the row.

Use the declared Apache escaping of quote, backslash, hex byte and
C-style whitespace for opaque quoted fields. Refuse invalid escapes,
unterminated fields, decoded controls in the retained request target,
extra fields and physical framing controls. A decoded escape never starts a new
record or alters field framing. Within a request,
accept only one method token, an origin-form target and a printed HTTP
version with no extra tokens. Persist only a proved eligible canonical
page key or a safe unresolved reason. Strip query and identifying data
before persistence. Accept final status 100 to 599. Treat a decimal
response size as reported body bytes without HTTP headers. The
Combined size field uses Apache %b. A dash represents zero bytes
sent. A missing mandatory field rejects the row. Never present this
metric as measured transfer bytes.
Do not persist remote host, user, logname, referrer or user agent.
Without separately qualified trusted bot evidence, classification is
unknown or claimed, according to the permitted label projection.

The synthetic combined record uses TEST-NET address 192.0.2.4, dashes
for logname and user, timestamp 10/Jan/2026:10:00:00 +0000, request
GET /guide HTTP/1.1, status 200, size 17 and quoted dashes for the two
header fields. Check exactly one covered request for the eligible
page and 17 reported body bytes. Replace size with a dash and check
zero reported body bytes. Escape a quote
in an ignored referrer, append a query, and remove the final newline
in separate variants. The parser preserves only permitted projected
facts, rejects an incomplete tail, and counts refusals distinctly.
Read-only record processing never resolves a logged address.

[apache-combined]: https://httpd.apache.org/docs/2.4/mod/mod_log_config.html#examples

The [Apache Combined and escaping reference][apache-combined] names
the producer grammar. Qualify parser framing, offsets, byte units,
source scope and privacy on the exact synthetic profile.

### Other offline log profiles

The same dispatcher accepts the explicit profiles below after
per-profile admission. Pin the field and escaping schema and
validate a source header or signed intake manifest before any
event. Accept a single owner-mapped scope, known clock, close
window and authorised input. A missing supported optional metric
remains unknown. Unknown source version, ambiguous framing or
reused period holds the scope. Report observed coverage even
where a producer can omit requests. All example values below
are synthetic.

| Profile | Exact input fields and projection | Acceptance |
|---|---|---|
| NGINX Combined default | Use its predefined Combined order, with the second token serving as a literal dash and default escapes as hex bytes. Pin offset-bearing local time. Use log-write local time with explicit offset and basis. Read request, status and body_bytes_sent, excluding headers. Discard identity, referrer and agent after allowed ephemeral classification. Cookie extensions are outside this profile. | A 200 fixture with 17 body bytes and an escaped ignored field counts 17 reported body bytes. Apache-only escape and different log format give a safe refusal. |
| IIS W3C Extended | One initial Fields declaration gives unique space-separated known tokens. Require date, time, cs-method, cs-uri-stem and sc-status. Use the W3C UTC logged-event basis. Keep its exact meaning profile-bound. Admit optional sc-bytes only with proved producer unit, otherwise unknown. Ignore extra declared approved columns and every identity column. Refuse a mid-file field change. | A reordered approved header still maps a synthetic 200. A dash in required time, missing Fields or background format change gives no eligible row. |
| Cloudflare HTTP Requests Logpush | Newline JSON with unique keys and approved scope manifest. Pin Logpush output timestamp_format=rfc3339 and the selected field set. Require EdgeStartTimestamp as RFC 3339 UTC edge-request start, ClientRequestHost, ClientRequestPath, ClientRequestMethod and EdgeResponseStatus. Optional EdgeResponseBytes is labelled bytes returned by the edge to the client. Select full-day aggregate scope. Drop keys holding identities, bodies or query strings before checkpoints. | A JSON object with an eligible host and integer status counts once. Duplicate key, wrong host, unknown timestamp unit and an embedded fake line fail safely. |
| AWS ALB HTTP(S) access file | Pin the documented profile field count; parse positional fields with space and quoted spans. Require type 1 http/https/h2, response-generation instant time (2) in the admitted ISO 8601 UTC shape, elb_status_code (9), sent_bytes (12) and request_line (13). For the HTTP profile sent_bytes includes response headers and body, excluding TCP headers and TLS handshake payload. Map absolute request target to an already authorised hostname and strip query and fragment before admission. Use 100–599 or distinct 000 status unavailable. Unsupported trailing format holds until admitted. | An eligible 200 status with source bytes remains labelled load-balancer response bytes. Test target-versus-balancer status mismatch, 000, unknown host and partial delivery. Nothing treats best-effort delivery as complete. |
| CloudFront standard W3C | Pin the exact standard W3C export edition and field order. Require date, time, cs-method, x-host-header, cs-uri-stem and sc-status. Use UTC at response end for supported HTTP rows. HTTP sc-bytes includes viewer response headers. Refuse unknown semantics and non-HTTP profiles. Use 000 as status unavailable. | Scope-select host before mapping. Separate a proved 200, an observed 000, another host and absent file coverage, including oversize-unlogged limitations. |
| Custom delimited log | Choose exactly one versioned declared UTF-8 single-line CSV or TSV format, unique field keys, bounded known quoting, explicit RFC 3339 offset and declared timestamp basis, method, owned origin-form path and integer HTTP status. Confirm scope, known optional byte unit, and digest. Treat the remaining fields as discard-only. | Try an admitted reordered schema, multiline record, duplicate header, spoofed path and an unknown byte unit. Read eligible typed values only. No script or format detection. |

Project timestamp basis explicitly with UTC time and day. Apache
request-received, NGINX log-write, IIS logged event, Cloudflare edge
request start, ALB response-generated, CloudFront response-end and
a declared custom basis are distinct. UTC normalisation does not make
the bases equal. Keep aggregates separate by basis and byte metric.
Cross-source comparisons needing a shared window or byte definition
return unknown where equivalence is unproved. Preserve source and unit
in an eligible export. A bucket with uncovered periods or an absent
metric cannot turn them into zero.

Accept plain files initially. A compressed source needs an exact
approved streamed decoder and expansion/checkpoint contract under
0115. Page absence becomes a finding only with qualified
source coverage and scope. Duplicate file digest, overlapping
range and uncertain commit reconcile against original receipt
before reading a new window. A correction uses its own approved
source and replaces the exact prior projection.

[nginx-log]: https://nginx.org/en/docs/http/ngx_http_log_module.html
[iis-log]: https://learn.microsoft.com/en-us/windows/win32/http/w3c-logging
[cf-log]: https://developers.cloudflare.com/logs/logpush/logpush-job/datasets/zone/http_requests/
[cf-log-config]: https://developers.cloudflare.com/logs/logpush/logpush-job/api-configuration/
[alb-log]: https://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-access-logs.html
[cfnt-log]: https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/standard-logs-reference.html

Producer field sources checked 21 November 2026.
[NGINX][nginx-log] · [IIS][iis-log] · [Cloudflare fields][cf-log] ·
[Cloudflare output format][cf-log-config] ·
[AWS ALB][alb-log] · [CloudFront][cfnt-log]

## Offline rewrite profiles

The initial version accepts only exact tested text subsets. Mode
is explicit. File size is at most 256 KiB and a run has at most
128 rules and 1,000 synthetically tested target paths under the
tighter owner limit. Interpret LF and CRLF, ASCII tokens and
inert quoted path strings. Preserve source line numbers.

| Profile | Accepted statements | Match and action |
|---|---|---|
| Apache local redirect | An isolated per-directory RewriteRule with an anchored ASCII path pattern, one local path substitution and exact flags R=301,L or R=302,L. Also permit a fixed Redirect 301 or Redirect 302 under its own literal prefix rule. | Remove one leading slash for RewriteRule at the declared root. Reattach it for output. Allow at most four captures and decimal references 1 to 4 to participating groups. Preserve an unmatched path. |
| NGINX local rewrite | One supplied, bounded sequence of standalone server-context rewrite rows with anchored safe path pattern, local path substitution and redirect or permanent flag. | Match a leading-slash path. Permit the same bounded capture substitution. Map redirect to 302 and permanent to 301. |
| Caddy local redirect | An isolated sequence of static redir rows with literal absolute local path, local destination and explicit status 301 or 302. | Compare the whole escaped canonical path. No templates or imported matcher state. |

Restrict regex to literals, bounded character classes, grouping,
alternation, anchors and bounded repeats understood by the
admitted parser. Refuse regex backreferences, lookarounds, ambiguous
escaping, unbound capture, query/fragment substitution, absolute
destinations, directives with side effects, includes, variables and
modes that require surrounding server context. Treat unsupported
complete input as refused, without a speculative partial result.
A target may have a query. Report scope as path-only and do not
echo query fields. Use first matching rule in order. Follow produced
local paths in the same offline plan up to eight transitions. A
repeat proves a simulated loop and one additional transition after
the limit keeps completeness unknown. Results and source line are
inert. Qualify each dialect against its official description and
independent pure synthetic cases, including root, percent encoding,
semantics outside the subset, path expansion bounds and rule order.
No default promise of exact server behaviour follows.

## Typed data and processing

| Record and key | Required fields and constraints |
|---|---|
| Capture (site, capture) | Source kind, canonical scope, plan/rule/profile version, source and permission generations, exact time and timezone, stable coverage flags by field and stage, lease reference, status, manifested custody references. Immutable once sealed. |
| Page (site, capture, canonical URL, stage) | Source/request receipt reference, site generation status, observed URL when permitted, typed status and redirect/links references, metadata and source slots, content/structure references, coverage, scrub profile and field-level outcome. Stages are raw, static, script and paint, with distinct readiness. |
| Edge (site, capture, source, stage, relation, ordinal) | Destination key if eligible, safe display text where permitted, direction, source occurrence, observed decision and eligible hop receipt. Dangling or omitted destination has its own state. |
| Check (site, capture or local inspection, rule, subject) | Manifest, safe source refs and rule location, observation stage, outcome, reason, checked count and known denominator or reason. Unknown denominator is never zero. Outcomes are observed, partial, unknown, refused, unavailable or withdrawn. |
| Log import and cursor (site, import) | Source digest, dialect/profile digest, pinned timestamp basis, byte metric and source period, file bytes, next confirmed offset, last accepted ordinal, site, current grants, erasure epoch and original receipt. Cursor contains no raw line. |
| Log observation (volatile) | UTC instant and source timestamp basis, eligible canonical key, HTTP method and status, admitted bot class and proof disposition, typed byte metric with value or unknown. Exists only within the owning parser turn. Release a permitted day's aggregate through 0034/0044, never a stored per-request row. |
| Log aggregate (site, source, timestamp basis, UTC day, page or segment, class) | Segment version, exact source coverage, admitted coarse status or metric dimension, counts and exact producer byte metric/units, eligible denominator, rights and safe proof reference. Retain only the admitted projection, and prevent overlapping batches from double-counting. |
| Pair (site, job, canonical page) | Canonical plan digest, one-use owner approval receipt and expiry, immutable crawl and Browser leg IDs, signed generation observations before, between and after, lease and compliance refs, declared identities, exact admitted render profile, same guest context and parser comparison fingerprints, two completion reasons and check refs. One final immutable outcome per work identity. No cross-site join. |
| Offline receipt (site, inspection, input digest, rule version) | Preview/use purpose, matched local schema/profile, scrubbed evidence references and deterministic output digest. Refer to parent custody and protected receipts. Plain draft output cannot carry an authority or release receipt. |

Request a 128 MiB memory promise for offline log work and up to
512 MiB of encrypted scratch. Propose a 256 GiB read cap per file
on either an approved local drive or external SSD, subject to the
site lease. Process at most 16 MiB or 25,000 records per scheduler
turn. Check stop between lines and every 64 KiB of a long rejected
line. Release the turn at the earlier of this cap or two seconds
of processing at the next line boundary. Stage only an eligible
aggregate checkpoint. If the source period, offset proof, disclosure
or local free space fails, hold the remaining job and show incomplete
coverage. Check current rights again before committing any aggregate.

The job validates the manifest and lease, writes protected admission,
performs bounded processing, commits an authorised checkpoint and seals a
scrubbed result. Every stored row keys by site. Before preview, copy, export
or MCP, check live read rights and erasure. Alekh uses typed read
projections. Unknown, withheld and withdrawn fields stay typed when filtered
or exported. Sort output by owner URL key, stage and rule ID, and use stable
ordering for ties. Do not retain cookies, session credentials, unneeded
request details or original logs in the project.

## Exact owned-page acquisition and generation

The current signed 0021 site transport and applicable 0028/0031 add-on
boundary may carry a separate read capability after its own admission. The
owner-confirmed plan names the installation, exact HTTPS canonical page and
the two declared profiles. The add-on adapter must provide a signed
page-generation observation through that paired channel, before, between and
after the two public reads. Identity-independent page generation must cover
published content and all reviewed settings that can change the eligible
main response. Use an atomic generation proof tied to serving, with every
relevant publish and settings change invalidating the previous generation.
An adapter that cannot attest this coverage or relies on body hashes or
timestamps is unavailable. The default public footprint budget is zero extra
queries, zero extra time and zero extra bytes per page. Do not add a public
header, route, query or callback by default. An admitted adapter may expose
metadata only through its separately qualified signed capability. Existing
page and site trust is checked at the actual peer, never by a user-entered
claim.

The signed observation has fixed typed fields: installation ID, site ID,
canonical URL key, content revision, response configuration generation,
serving proof version and broker challenge/request ID, nonce, issue time and
signing key ID under the existing signed request/reply envelope. The adapter
binds coverage across both declared identities and all served instances
eligible for the plan. Admit an adapter only if protected installation
evidence proves that this URL is served from one coherent current generation
under both identities, including its cache and routing policy. The paired
channel may return a protected serving observation for each leg where the
exact admitted hosting profile supports one. Compare its response reference
with the broker receipt and the unchanged generation. Otherwise require
qualification of a coherent direct-serving profile that serves that
generation at the actual peer and cannot return stale, mixed or
variant-specific generations. A signed control-plane counter alone cannot
admit either profile. Stale, missing, replayed, withdrawn, cache-uncertain,
mixed-instance or mismatched observations withhold pair completion. Consume
the exact current owner approval once, after both reservations and before
the first dispatch. Bind each leg to its distinct original attempt identity,
intent and receipt under the same plan. Resume between legs only when the
first receipt and current scope are known. The two legs keep the same guest
context apart from the declared identity and the owning render path. They
are attempted at most once per approved logical pair. Hold any crash or
lost-reply uncertain attempt with zero additional dispatch under QR-212. A
refused, mismatched or partial leg yields a partial or refused job and an
unavailable comparison. A render held before admission cannot send the
Browser identity as a stand-alone fetch. A fresh, explicitly approved job
may acquire later under current rights and budgets.

## Screens, failure feedback and proof

Open **Crawls**, **Access logs** or **Technical Toolbox** in the parent UI.
Select site and known source. Choose saved capture, local file, or an
eligible live own-site operation. Preview scope and constraints, show **Run
check** or **Approve this comparison** as applicable, then show progress,
pause/cancel and coverage. Show eligibility before confirmation. Pair
details show the exact profiles, stable revision, approved differences and
each incomplete reason. The result bar offers **See evidence** for readable
rows and **Export local file** where authorised. Disabled operations explain
the exact prerequisite, such as **Site revision could not be verified**,
**Robots rules prevent this request**, **Rendering is unavailable for this
site**, or **Results are incomplete**. Do not show raw sensitive data in
explanations.

### Local tool projections

All use static schemas and 0020 site and projection grants. A stale
session or foreign source fails before enumeration. Page range names
bind the same opaque cursor contract as 0143. No tool can read original
files, add rules, rerun a job automatically or approve a live request.

| Action and permission | Input | Output |
|---|---|---|
| audit_toolbox_results, audit.toolbox.read | Exact inspection ID and optional admitted rule ID and cursor | At most 50 approved typed outcomes, source and coverage. Robot tests may show the safe matched rule and line only in the granted projection. |
| audit_log_summary, audit.logs.read | Exact accepted import or daily aggregate ID, filter by proved aggregate dimension, cursor | At most 50 protected aggregate rows, period, source coverage and suppression. No raw log or source reference with path. |
| audit_compare_summary, audit.compare.read | Exact sealed comparison ID, field subset and cursor | At most 50 compatible safe changes or unavailable reasons. No pending leg, cached page contents or authority to dispatch. |

Use a synthetic mini-internet and dated generated logs. Unit designs cover
empty, single, boundary, one over, malformed source, unknown version and
dependency failure for each table rule. Fuzz typed URL input, metadata,
robots, CSV, XML, rewrites, log lines and selector parsing with inert
fixtures. Shared scope and input-bounds tests must assert zero network
actions outside the broker. Integration runs assert deterministic sealed
projections under admitted Linux and Windows Bazel profiles and measure
worst-case memory, disk and cancellation against the parent 8, 16 and 32 GiB
host profiles. Repeat source, policy and manifest versions in proof refs.
Tests are designed here and require actual per-profile evidence before
admission.

Quint models transitions Preview, Approve, Reserve, DispatchOne, Checkpoint,
Pause, Resume, Cancel, Seal and Erase. Model the signer, owner grant, pause,
budget, robots/site status, claim generation, stage completeness and erasure
separately. Plan separate Rust replay fixtures using deterministic fake
time, transport, signer, file offsets and leases. Bind TB-05 to
INV-0140-AU-02 through INV-0140-AU-04. Qualify Quint Connect under QR-006
and EX-06 before accepting the evidence. Assert admitted guest
method/identity/site and durable intent before dispatch, budget bounds and
fair queue turns under the parent scheduler, no second dispatch after
revocation or uncertainty, no concluded pair from incompatible or incomplete
evidence, and no readable output after erasure. Include exactly-at-limit
versus one-over, pause between pair legs, failed intent flush, cached old
content under an unchanged control-plane counter, mixed generation and lost
terminal reply. Add an adapter fixture with a valid signature on the wrong
canonical page and a replayed valid envelope. Refuse all three and preserve
the eligible local compliance record. Require identical canonical local
receipts or identical typed refusals across eligible profiles.

## Provenance

Vendor documents checked 21 November 2026: [SEO Spider][vendor-spider],
[Configuration][vendor-config], [Crawl comparison][vendor-compare],
[Log File Analyser][vendor-logs], [Log format guide][vendor-log-guide],
[TechnicalSEO.com index][vendor-tools] and
[Public descriptions][vendor-tools-info]. Their capability descriptions
give comparison context only. Tool UI paths, advertised counts without
complete catalogues, exact parser dialects and proprietary formulas
supply no Sutradhar admission or conformance proof.

[vendor-spider]: https://www.screamingfrog.co.uk/seo-spider/
[vendor-config]: https://www.screamingfrog.co.uk/seo-spider/user-guide/configuration/
[vendor-compare]: https://www.screamingfrog.co.uk/seo-spider/tutorials/how-to-compare-crawls/
[vendor-logs]: https://www.screamingfrog.co.uk/log-file-analyser/
[vendor-log-guide]: https://www.screamingfrog.co.uk/log-file-analyser/tutorials/introduction-to-log-files/
[vendor-tools]: https://technicalseo.com/tools/
[vendor-tools-info]: https://technicalseo.com/tools/pages/home.php
