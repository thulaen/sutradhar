# 0115: Leviathan rendering and content engine

This specification owns what Leviathan can understand about a page beyond its
raw bytes: building the page tree, styling and layout, working out what a
visitor can see, running page scripts safely, reading documents, feeds,
images, media headers and structured data, and explaining how the finished
page differs from the first response. It is the content half of the crawler.
Spec 0082 owns the legal wall, identity, politeness, modes and fences, and the
receipts and niche mechanics; spec 0105 owns link evidence; spec 0116 owns the
audit storage (vault, archive, offline replay), the steering and the
verification of the crawler, including the browser standards tests, the
reference-browser oracle and the shared build checks; spec 0044 owns the site
audit that uses this engine's facts.

Receipts mode (spec 0082) runs no scripts and does no rendering. In Receipts
mode only the raw level runs, only on pages within the 512 KiB page ceiling,
and the only readers are sitemaps, feeds and headers; PDF, office, image and
media readers run in Audit mode. Everything else in this spec applies to Audit
mode on the user's registered sites. Specs 0044 and 0082 use the engine
admitted by EX-02 under this spec. Rivals are read once and are not tracked
over time (DR-062). A check that needs two reads works only on the user's
own sites.

The pillar list asked for clean-room engines for HTML, CSS, layout, fonts,
painting, JavaScript, WebAssembly and every decoder. ADR-0022 and AGENTS.md
section 6 rule 14 say mature components sit behind our layer, and that the
rendering engine is chosen by a research experiment (RQ-011, EX-02, QR-003).
This spec therefore defines the behaviour, the interfaces, the limits and the
safety cage, and leaves the choice of component to that experiment. Until the
engine is chosen, Audit mode reads raw HTML and headers only (spec 0082), and
every rendering level below is "not available".

Example: Dana's shop page returns status 200 and a script-built shell. At level
"script", Leviathan opens the page in its cage as `Leviathan-Browser`, with a
phone-sized screen and a virtual clock. The page asks the broker for two data
files; both pass the robots rules and private-address block. After 3 virtual
seconds the clock stops. The report says: "The product list and the price are
added by a script; the first response had none of them." The visible-content
check says that a cookie banner covers the page and that a block of keywords is
white on white. The soft-404 check says "Likely an error page: it shows 'No
results' with status 200". Nothing was clicked except the open-and-close buttons
of two FAQ sections, and every request was a GET.

Values marked "proposed" are design choices to be checked by measurement.

## What it does

### Words used here

- **Level**: how much of a page Leviathan builds: raw, static, script or paint.
- **Cage**: the locked operating-system sandbox a page runs in.
- **Broker**: Leviathan's own fetcher; it is the only way a caged page reaches
  the network.
- **Fuel**: a count of script steps allowed, so no script can run forever.
- **Virtual clock**: a clock the cage controls, so waiting costs no real time.
- **Main content**: the part of a page that is not repeated navigation,
  footers or notices.
- **Visible**: shown to a person looking at the page at a stated screen size.
- **Soft-404**: a page that says "not found" or is empty but returns status 200.
- **Hit testing**: asking which box is on top at one point of the screen.
- **Registrable domain**: the part of a host name that an owner can register,
  such as `example.com`.
- **Shadow root**: a hidden page tree inside a web component.
- **JIT**: a compiler that turns hot script code into machine code while it
  runs.
- **WOFF2**: a compressed web font file format.
- **Range request**: a GET that asks for only part of a file.
- **Tree-change record**: the list of changes to the page tree that the page's
  scripts make, in order.
- **Content-addressed vault, WARC**: defined in spec 0082 and owned by spec
  0116.

### What this spec designs

| Feature | What the user sees |
|---|---|
| N-1661 Clean-room HTML5 parser and page-tree builder | A page tree that matches what browsers build from messy markup |
| N-1662 Clean-room CSS stylesheet and cascade engine | The final style of every element |
| N-1663 Full visual page-geometry engine | Position and size of every box and word |
| N-1664 Simultaneous mobile and desktop rendering | Phone layout first, with an optional desktop comparison |
| N-1665 Shadow DOM and modern component reader | Content inside web components is read, not skipped |
| N-1666 Real human visibility calculator | Hidden and covered content told apart from visible content |
| N-1667 Visual stability and load-speed analyser | Simulated loading metrics and layout jumps |
| N-1668 Clean-room font parser and global text shaper | Text wrapping that matches real devices for all scripts |
| N-1669 Clean-room headless page painter | Software screenshots with no external browser |
| N-1670 Accessibility tree builder | The screen-reader outline and its quality checks |
| N-1671 Adaptive display-mode simulator | Dark mode, reduced motion and high contrast checks |
| N-1672 Process-level rendering cages | Every page runs in a sealed sandbox |
| N-1673 Clean-room ECMAScript parser and virtual machine | Page scripts run safely |
| N-1674 Fenced just-in-time speed optimiser | Faster scripts, only inside tight fences |
| N-1675 Clean-room WebAssembly execution engine | Page WebAssembly runs with step limits |
| N-1676 Virtual clock fast-forwarder | Timers finish without real waiting |
| N-1677 Deterministic clock and randomness controls | The same page renders the same way twice |
| N-1678 Live client-side data-call interceptor | Every background request, tied to what it builds |
| N-1837 Initial HTML vs finished-page diff viewer | What depends on scripts |
| N-1679 Smart soft-404 and broken-app detector | Error pages that say 200 |
| N-1680 Per-script CPU blame meter | Which script costs the most time |
| N-1681 Safe interactive-reveal inspector | Tabs and FAQs opened without risky clicks |
| N-1682 Clean-room PDF layout and link reader | Text, tables, metadata and links from PDFs |
| N-1683 Streaming XML sitemap and feed engine | Huge sitemaps and feeds read in a stream |
| N-1684 Strict structured-data verifier | Declared facts checked against what shows |
| N-1685 Clean-room image format decoder suite | Image size and health checks |
| N-1686 Streaming audio and video manifest reader | Media facts from headers only |
| N-1687 Clean-room office document extractor | Text and metadata from office files |
| N-1688 Social-card and preview simulator | Share previews checked |
| N-1689 Global language and regional-link auditor | Real language and hreflang pairs checked |
| N-1838 Editorial content vs boilerplate separator | Main content apart from repeated parts |
| N-1657 Clean-room compression decoders with bomb shields (register name kept; mature decoders are used) | Safe decompression with expansion limits |


### Levels and the engine choice

| Level | What is built | Runs scripts | Needs |
|---|---|---|---|
| Raw | Parsed HTML tree, headers, text | No | A mature HTML parser |
| Static | Styles, layout, visibility, accessibility tree | No | CSS, layout and font components |
| Script | The page after its scripts have run | Yes, in the cage | A script engine (RQ-011) |
| Paint | A screenshot | Optional | A software rasteriser |

1. Each level builds on the one before. A level that has no admitted component
   is "not available", and the report says so; it never invents a result.
2. The engine for the script and paint levels is chosen by experiment EX-02 on
   build effort, memory in the budget of spec 0078, build on both systems
   without our compiling C or C++, fidelity against a reference browser (spec
   0116) and safety. Candidates in the dossier are an embeddable Rust web
   engine, a Rust script engine, a prebuilt V8 binding and a managed headless
   browser. Their licences and maturity are not yet confirmed.
3. All components sit behind traits with test fakes, so the choice can change
   without changing the rules here. We write no first-party script virtual
   machine, no first-party JIT and no first-party decoders where mature ones
   exist (rule 14). The pillar list's "fenced JIT" is available only if the
   chosen engine provides one, and only inside the cage and its limits.
4. The first-party parts are: the cage and broker, the limits, the virtual
   clock and determinism controls, the visibility calculator, the data-call
   tracker, the diff, the soft-404 detector, the script cost meter, the
   interaction rules, the verifiers and the boilerplate separator.
5. A change of engine needs a new decision record and a fidelity run, not a
   setting.
6. Admission rules for an engine. Every engine must support a host-supplied
   fetch hook so that it opens no socket of its own. An engine that opens its
   own sockets (for DNS, QUIC or WebRTC, as a managed headless browser does)
   may be admitted only if the cage blocks all network access, leaving one
   pipe to the broker, and a test proves it. An engine that needs a page tree
   and layout of its own must provide them; a script engine without a page
   tree (the dossier notes this of the Rust script engine) qualifies only as
   part of a pair with a tree builder, and that pairing is judged by EX-02
   too.
7. The limits split in two. Required of every engine, enforced from outside the
   page: a real-time kill, a memory cap, output caps on nodes, bytes and
   sub-resources. Engine-dependent: script fuel, the virtual clock and
   determinism. An engine that cannot meet an engine-dependent limit lowers
   that capability to "not available" (for example "not deterministic"), or is
   rejected in EX-02. These criteria are added to EX-02 and the dossier.
8. The paint level needs a software rasteriser, which is a separate mature
   component and does not come from the script engine.

### The cage and the broker

1. Every page that is parsed with scripts or painted, and every document,
   image or media reader, runs in its own process in an operating-system
   sandbox with no access to local files, the clipboard, printers, cameras,
   microphones, location, other processes or the network. Level raw and static
   run in the same kind of cage because they read untrusted bytes too. On
   Linux the cage uses a network namespace that holds only a pipe to the
   broker, a system-call filter and a file-access restriction (as in spec
   0037). On Windows a job object does not block files or the network, so the
   cage uses a restricted low-privilege process (an AppContainer without
   network capability, or an equivalent) plus a job object; this is researched
   and tested as the Windows render cage in QR-029. Render levels
   are "not available" on Windows until that test passes.
2. The cage has no socket. Every sub-resource request (style, script, data
   call, image, font, frame) goes to the broker, which applies the full legal
   wall of spec 0082 to it: robots rules (a robots file for each host is
   fetched once and cached by the broker), opt-out signals, politeness per
   host (a third-party host such as a content network is a host like any
   other), the private-address block (`is_blocked` of spec 0107, including
   after redirects) and the compliance ledger. A request the wall refuses is
   not made, and the page sees a network error.
3. A fact that depends on a refused sub-resource (a missing style, script or
   data file) is marked "partial: blocked sub-resource", and the visibility
   result, the soft-404 verdict and the diff carry lower confidence. The
   report names the refused address and the rule (SUT-1504), so a broken page
   that Leviathan itself broke is never mistaken for a site fault.
4. The broker makes only GET and HEAD requests (and GET with a byte range for
   media headers). Anything else, including a form post or a script's POST, is
   refused (SUT-1505) and listed in the report.
5. Limits per page, all enforced from outside the page: virtual time
   (`render.page_ms`), real time (`render.real_ms`), script fuel (`render.fuel`,
   where the engine supports it), memory (`render.memory_mib`), page-tree nodes
   (`render.max_nodes`), sub-resources (`render.max_subresources`), frame depth
   and count (`render.max_frame_depth`, `render.max_frames`), total bytes
   (`render.max_bytes_mib`), stylesheet size and selector count, and screenshot
   size. A page that reaches a limit is stopped and reported with the limit
   (SUT-1501), and the partial facts it produced stay marked "partial".
6. Browser dialogues, pop-ups, downloads, new windows, notifications, permission
   requests and storage prompts are denied or ignored. Storage, service
   workers and cookies are unavailable; a page sees them as denied and
   Leviathan behaves like a first-time visitor that accepts nothing.
   Navigation away from the page is blocked.
7. A cage that crashes or hangs is killed and its page marked "render failed"
   (SUT-1502); the crawl goes on (spec 0082).
8. The cage and broker are the only code that handles untrusted pages;
   everything downstream receives plain data.
9. How many cages run at once is `render.parallel` (default 1), and the
   product of it and `render.memory_mib` must fit the audit-mode share of spec
   0078; a start with a larger product fails.
10. Not covered here: hostile-script and drive-by download quarantine and bot-
    trap and honeypot avoidance are owned by spec 0082; the legal exposure of
    running third-party scripts goes to the legal review (QR-010).

### Identity, viewport and mobile first

1. The cage's requests carry the `Leviathan-Browser/<version>` identity of spec
   0082, never another browser's name. A page may therefore treat Leviathan
   differently from a person; the reports say they cannot prove how a page
   treats a search engine's own crawler (spec 0082).
2. The default screen is a phone (proposed 390 by 844 CSS pixels, with a phone
   device pixel ratio) because search engines index mobile first. An optional
   desktop pass (proposed 1366 by 768) renders the same page for a side-by-
   side comparison, and the report lists what differs: content, links,
   structured data and layout.
3. Both passes count against the same render budget of the site (`seo.crawl.
   browser_pages_per_day`, spec 0044), so desktop doubles the cost and is off
   by default.

### Page tree, style, layout and text

1. The HTML parser follows the HTML standard's own error-recovery rules, so
   broken tags, misnested formatting and tables give the same tree as modern
   browsers. A standards conformance run is in spec 0116.
2. The style engine computes the final style of every element from author
   stylesheets, the browser's default styles, custom properties, media and
   container queries and the display-mode settings below, within limits on
   stylesheet size and selector count.
3. The layout engine places every box and every word for blocks, lines,
   flexible boxes, grids and tables, and reports position and size. Text is
   shaped by a mature shaper with right-to-left, vertical and complex scripts,
   using the font files the page loads (OpenType and WOFF2) or the bundled
   fallback fonts, so wraps and page heights match real devices. Font decoding
   has size and table limits.
4. Web components are read through their shadow trees, including open shadow
   roots and slotted content. Closed roots that cannot be read are reported as
   "closed, not read".
5. Determinism: with `render.deterministic` on, bundled fonts, a fixed screen
   and a controlled clock and random source make two renders of the same bytes
   identical (below).
6. Layout does not need a real display. A page whose layout would exceed the
   limits above is stopped, not truncated silently.

### What a visitor can see

1. The visibility calculator marks each element visible or not visible, with
   the reason: zero or near-zero opacity, hidden by display or visibility,
   clipped, text smaller than a threshold (`render.min_font_px`, proposed 4
   px), colour matching the background (contrast ratio near 1), pushed off
   screen, or covered by another box at the point a person would see it.
2. Covering is decided by hit testing at the centre and corners of each box in
   the visible area, using painted order. Consent banners and overlays that
   cover main content are reported as such.
3. The result is evidence for the checks of spec 0044 and the radar of spec 0105
   (hidden text); it states what the layout says and never claims intent.
4. Main content is the visible text left after the boilerplate separator below.

### Loading metrics

1. A loading simulation reports, from the broker's recorded timings and the
   layout, the time at which the largest visible image or text block painted
   and the total layout shift as boxes move while loading. These are layout-
   based approximations of the published Core Web Vitals definitions for the
   largest contentful paint and the cumulative layout shift. Total blocking
   time needs long-task timing and a real clock, which a virtual clock does
   not give, so it is reported only for an engine that provides long-task
   timing and is otherwise "not available".
2. These are simulated values on the render machine, labelled "simulated", and
   never presented as measurements of real visitors (spec 0040 holds real Search
   Console field data and spec 0034 the site's own counts). Interaction-to-next-
   paint cannot be simulated without a person and is not reported.
3. Sources the scout must confirm before release: the Core Web Vitals
   definitions, the HTML standard, the accessible-name computation
   specification and the contrast rule of the Web Content Accessibility
   Guidelines (4.5 to 1 applies to normal-size text; large text uses 3 to 1).

#### Offline comparison of one script's simulated paint

For F-0174 the current read uses N-1680's scoped script processing
evidence. It shows eligible CPU time or engine fuel with exact
units and separates advisory wall time. It does not infer added
page-load time. The ordinary report starts no replay or capture.

A separate comparison receipt is eligible only after QR-110 admits
its offline profile. Run the pair only in an existing authorised
own-site Audit replay with already retained scrubbed bytes and the
verified N-1677 broker recording. Bind the site, URL, page revision,
snapshot and broker digests, exact target script identity and bytes,
renderer, policy and resource bounds, and both legs' input and
output digests. Pin the mode, viewport, locale, clock, seed and
timing basis. Both legs stay within their admitted render and job
limits and reuse the existing custody checks. No request, cache
repair, added capture or site edit can supply missing evidence.

In one leg the exact recorded third-party script is included. In
the other leg the admitted offline variant omits only that script's
recorded delivery and execution. The receipt records this one
difference. Require complete applicable replay coverage and
matching non-target resources, readiness, visible main content and
milestone identity under the profile. An unresolved dependency,
new request, missing readiness, changed visible main content or
unmatched timing invalidates the pair. A blocked script in an
unrelated capture cannot supply the omitted leg.

Use RN-42's simulated largest visible image or text paint. Measure
from the same replay navigation start to the first paint of the
same profile-selected block in both legs. Match its scoped identity,
content and layout role. Both legs need an eligible complete
milestone on the same pinned clock and precision. Hold if the
largest block changes or the profile cannot establish the match.
Store the two elapsed values, unit and precision, clock and
milestone identity, coverage, variant, profile version and current
scope evidence. Subtract elapsed time with the script omitted from
elapsed time with the script. A positive difference means later
paint, a negative difference means earlier paint, and zero means
the times are equal at that precision.

Show “Simulated paint difference” with both values and their one
offline condition. This result describes that replay and milestone.
The script CPU or fuel sample does not enter the calculation. No
field timing, general attribution or performance promise follows.
A failed, erased or withdrawn receipt cannot provide the comparison.
Recheck both legs and grants before use, then preserve any
independently eligible processing evidence. If only the processing
evidence qualifies, say “Script processing measured. Simulated
paint comparison unavailable.”

### Painting and accessibility

1. A software painter draws a screenshot without a screen or an outside
   browser. Screenshots are off by default (`render.screenshots`), are size
   limited (`render.png_max_kib`), stored in the content-addressed vault of
   spec 0082 and count against the disk cap of spec 0078.
2. The accessibility tree builder derives roles, names, states and the heading
   outline from the page tree and styles, following the published accessible
   name rules. It reports skipped heading levels, buttons and links with no
   name, images with no description, landmarks and form fields without labels.
3. The display-mode simulator renders again with dark mode, reduced motion and
   forced high contrast, and flags text that becomes unreadable (contrast below
   `render.min_contrast`, proposed 4.5 to 1) or layouts that break. Each mode is
   an extra pass counted in the render budget and off by default.

### Scripts, timers and determinism

1. In the script level, page scripts run in the cage's engine with fuel,
   memory and time limits. The pillar list's wording "clean-room JavaScript
   and WebAssembly runtime" is satisfied by a mature engine behind our trait
   (rule 14).
2. WebAssembly modules run in a mature runtime with fuel (a step limit), a
   memory cap and no imports except the ones the page environment provides.
3. The virtual clock replaces the real one. Timers, animation frames and
   delays run in order, and "wait three seconds" costs a fraction of a
   millisecond. The clock stops at `render.page_ms` virtual milliseconds or
   when the page has no pending work (idle), whichever is first.
4. Determinism controls: the date and time start from a fixed value, the
   random source is seeded, and network answers come from the broker's record.
   The same input bytes give the same output; a difference is a defect (spec
   0116 tests it).
5. Script errors are collected with their URL, line and message and are never
   swallowed (AGENTS.md section 5). A script that spins until fuel runs out is
   reported as "script limit reached".
6. The cost meter attributes script time and fuel to each script URL, split into
   first-party and third-party by registrable domain, so a person can see which
   script slows a page. Wall time is reported too but is advisory because it
   varies between machines.

### Data calls and the diff

1. The data-call tracker records every request the page's scripts make after
   load: URL, method, size, timing, and which script started it. It links each
   response to the page-tree changes that follow it using the tree-change
   record: a link is "direct" when a change happens within 100 virtual
   milliseconds of the response being delivered and the changed content
   contains values from that response; otherwise it is "possible". It never
   claims more.
2. The diff compares the first response's tree with the finished tree and
   lists every link, title, meta tag, canonical address, heading, structured-
   data block and block of text that exists only after scripts run, changed by
   them, or removed by them.
3. The soft-404 detector flags a page that returns status 200 and, in the
   finished page, has under `render.soft404_chars` characters of main content,
   or text matching the "not found" or "no results" phrases of the languages
   in `render.soft404_langs`, or a script spinner that never ends within the
   virtual time. It reports a confidence (low, medium or high) and the
   evidence, never changes the page's status, and caps its confidence at "low"
   when the render was partial or a sub-resource was refused. Pages that are
   legitimately sparse (an application page, a search page with no results, a
   login page) cause false alarms; the detector is measured on a labelled
   fixture set with a target false-positive rate (proposed: at most 2
   percent), and the number is reported.

### Safe interaction

1. The inspector may open sections that a page marks as safe to open: details
   and summary elements, tabs, accordions and disclosure buttons with the
   right accessibility state, and content that loads when scrolled into view.
   A scroll counts as an interaction.
2. It never activates a form control, a submit button, a link that leads away,
   a button inside a form, anything whose label or name suggests buying,
   signing in, deleting, subscribing, unsubscribing or changing an account, or
   any control that is not clearly read-only. It never types.
3. The broker refuses non-GET requests from interactions (SUT-1505).
   It also refuses a GET to a same-site path whose name suggests logout,
   delete or unsubscribe. A same-site GET can still have a side effect.
   The accepted risk is recorded in the risk register.
4. Storage is denied and navigation is blocked (above), so a click cannot save
   state or leave the page. The report lists each interaction and its result.
5. Interaction counts per page are capped (`render.max_interactions`), and an
   interaction that issues requests counts against the sub-resource budget.

### Documents, feeds, media and structured data

1. **Compression:** gzip, deflate, Brotli and Zstandard bodies are decoded
   with a maximum output size, the shared expansion ratio
   `web.max_expansion_ratio` of spec 0107 (one setting for every decoder) and
   a time limit (`doc.max_ms`), so a decompression bomb is stopped early (spec
   0082).
2. **Sitemaps and feeds:** XML sitemaps, sitemap indexes, news sitemaps, RSS
   and Atom are read as a stream with bounded memory, however large, with the
   XML limits and no-external-entity rule of spec 0107. A declared "last
   modified" date is compared with the page's real update only for the user's
   own sites, using the second read that audit mode already makes. Hreflang
   return-link checks and the social-card image fetch also run only in Audit
   mode on the user's own sites. Reading an existing feed does not
   enable the RSS output integration excluded in spec 0100.
3. **Structured data:** JSON-LD, Microdata and RDFa are parsed, and declared
   prices, ratings, review counts, breadcrumbs, dates and names are compared
   with the finished page's visible text. A claim with no visible support is
   reported. Offer availability and FAQ comparison require the separately
   qualified visible profiles below.
4. **PDF:** a mature reader in the cage extracts reading-order text, tables,
   author and title metadata and links, within limits (`doc.max_pages`,
   `doc.max_objects`, time). Encrypted or damaged files are reported, never
   guessed.
5. **Office files:** text, headings and metadata are read from Word,
   spreadsheet and presentation files (zip with XML inside) in the cage, with
   zip-bomb limits; macros are never run. The XML parser bans DTDs and
   external entities. Zip entry paths must stay inside the archive and
   nesting depth is capped. Author and title metadata are cleaned (control
   and bidi characters removed, length capped) in PDF and Office files before
   they reach a report or a log line.
6. **Images:** mature decoders read PNG, JPEG, WebP, AVIF and SVG headers and,
   when needed, the pixels, within size limits, to report real dimensions,
   broken files, oversized hero images and missing descriptions. SVG scripts
   are never run. An AVIF decoder that needs native code must be a pinned
   prebuilt binary or is blocked (QR-047); images that were not checked
   because a decoder is blocked are listed as "not checked (decoder blocked)",
   never skipped silently.
7. **Audio and video:** container headers, captions and chapter markers are
   read with byte-range GETs without downloading whole files. Each range is at
   most `doc.media_range_kib`, requests follow the rate and politeness rules
   of spec 0082 (`leviathan.rate_per_second`), and a page's total bytes stay
   under `render.max_bytes_mib`.
8. **Social cards:** Open Graph and similar tags are validated and the preview
   image is fetched once within the size limit and checked for dimensions.
9. **Language and hreflang:** the page's language is detected by a mature
   statistical model. Version 1 uses language detection for scope
   and safety under DR-081. Check hreflang return links only in the
   admitted own-site Audit. Report broken or one-way declarations.
   The structural audit feeds feature 564 in spec 0047 and gives no
   translation, regional variant or market recommendation.
10. **Boilerplate:** repeated navigation, footers, legal text and cookie
    notices are found by comparing blocks across the pages of one crawl, using
    fixed-size fingerprints so memory stays bounded. What remains is the main
    content. For a niche scan or a rival, the fingerprints exist only during
    that one crawl and are discarded when it ends (DR-062).

### Local visible offer and FAQ evidence

Use only a previously admitted completed own-site Audit render. Its
current authority must permit the exact site, canonical page, public
revision, capture receipt, DOM digest and viewport. Bind the markup
projection and visible facts to that same capture and revision. Use the
existing visibility calculator, renderer caps, scheduler and ledger.
Neither a check nor an unavailable result initiates a render, fetch,
click, expansion of a closed panel, context resolution or retry.

Independently qualify a versioned availability profile and a versioned
FAQ profile. Each pins the allowed markup shape, scope and viewport,
closed DOM regions, exact entity-key mapping from the admitted local
template, visibility and text projection revisions, semantic mapping,
bounds and source rights. Both the native Audit render and the exact
producer profile must pass applicable 0076 source and test admission.
Before publishing either value or consuming cached evidence, recheck
capture identity, current rights and erasure. A signed fixture of one
profile provides no authority for the other. An older capture that
lacks the matching profile or complete region proof cannot supply
these facts.

| Record | Contract |
|---|---|
| Visible field header | Site, canonical page and native revision digest, render receipt and DOM digest, viewport, profile ID and revision, opaque exact entity and selected region refs, current authority and coverage refs. Identity and scope use their owning typed records. |
| Availability fact | Header, exact offer and variant keys scoped to this page and pinned profile, and enum `in_stock` or `out_of_stock`. |
| FAQ fact | Header, exact question key and accepted-answer role, and SHA-256 digests of the normalised question and answer strings. |
| Withheld field | Safe reason only: unavailable profile, changed or mismatched scope, withdrawn or erased evidence, partial coverage, unmatched entity, ambiguous value, no visible counterpart, unmapped value or limit reached. No source content or protected identifier in the reason. |

For availability, compare the admitted Offer availability field with
the one visible status span in the exact mapped offer region. The
initial British-English profile maps only the exact normalised labels
“In stock” and “Out of stock” to the two admitted values, paired with
the admitted InStock and OutOfStock declarations. Treat every other
label or declaration as unknown. Require the same exact variant where
the template uses variants. A page-wide banner, a sibling offer, a
hidden label and the state of a cart button supply no fact for this
offer. Two matching visible spans, even when their labels agree,
leave the field ambiguous. Reuse the state only for this capture.

For each FAQ comparison, require one admitted FAQPage mainEntity
of type Question with one acceptedAnswer of type Answer. Match that
exact question to its mapped
visible question and answer regions. A text match alone supplies no
entity binding. A duplicate or missing question
key, two matching answer regions, closed or hidden answer, unsupported
markup-to-text projection or partial answer leaves the field unknown.
Compare complete plain question and answer strings in DOM reading order.
Apply Unicode NFC, trim and collapse whitespace with the admitted
versioned text projection. Keep case, word order and punctuation.
Neither fuzzy matching nor a semantic similarity or contradiction
judgement supplies evidence. A hidden duplicate cannot override visible
facts. Require complete RN-09 layout and paint evidence for
the exact saved region. The signature protects that observation.
Hidden DOM and accessibility text alone supply no visibility. If
the matching visible counterpart is unproved, withhold the comparison.

For each family inspect at most 32 declared entities and 32 mapped DOM
slots per page. Admit only mapped local keys of at most 128 UTF-8 bytes
before conversion to opaque references. A status span is at most 64
UTF-8 bytes, a whole question at most 512 and a whole answer at most
4,096. Apply both text limits before and after normalisation. The
existing stricter render caps, memory and time limits still apply. A
limit that prevents complete inspection withholds the affected family
as unknown with the limit reason. Never use a truncated answer or
publish a prefix as a complete result. Reuse protected capture
references instead of retaining new page text, HTML or screenshots.

Extend UT-RN-20 with the producer portions of 0145 FX-SD-MK13 and
FX-SD-MK16. Check the exact cap and one above for each row, key and text bound.
At the exact projection limit require an otherwise complete
capture and full admitted region. One over withholds that family.
Separately reach the active render cap at equality, including the
lower-memory case under 0078. RN-05 marks the capture partial and
neither family can compare. Use incomplete DOM coverage, a
changed page revision, wrong site, stale profile and grant
withdrawal before release. A fixture authorised only for availability
must leave FAQ unavailable, and the reverse must also hold. Verify no
new render, interaction, file read outside admitted custody or network
request, and only the allowed typed outputs on each admitted profile.

### Offline replay

The offline replay proxy (N-1660) is owned by spec 0116, with the vault and the
archive it replays from. This spec only supplies what is recorded: the broker's
exchanges, including the sub-resources of a page.

### Reuse

Our own code is listed in "Levels and the engine choice". Mature crates and
components sit behind traits: HTML parsing, style and layout, font shaping,
the script engine and WebAssembly runtime, rasterising, PDF, image, XML and
zip reading, compression, and language detection. Each is a build-or-reuse
record in the research dossier, and none is admitted before spec 0003's
pinning and review.

#### Bounded decoding, layout and accessibility evidence (N-1657, N-1661–N-1670)

Use the existing raw, static, script and paint capabilities. Use
admitted mature components and pinned profiles, the 0115 cage and
broker, 0082 fetch rules and 0116 conformance matrix. EX-02 and QR-029
continue to govern platform and engine admission. Missing components,
source, authority, profile or limits produce unavailable or partial
results at the exact affected level. Receipts keeps its 512 KiB raw ceiling
and eligible raw sitemap,
feed or header readers only. Static, script and paint need the
exact EX-02 component and platform profile and positive site cap.
A reference browser needs separate QR-049 admission. No renderer gains
network, local
host or storage access.

| Owner catalogue entry | Exact contract |
| --- | --- |
| N-1657 Clean-room compression decoders with bomb shields | Apply the admitted compatible decoder to supported gzip, deflate, Brotli and Zstandard input where qualified. Enforce the existing outer and inner byte, web.max_expansion_ratio, doc.max_ms, nesting and object caps, including the Receipts ceiling. No spill of unchecked protected bytes. Stop safely at the first exceeded limit and isolate the partial or refused outcome. No parser receives an unverified fallback result. [FX-0115-W5-ADMITTED-DECODER-BOUNDARY-AND-REFUSAL](../planning/feature-wave-5.md#acceptance-design-for-n-1647-to-n-1670). |
| N-1661 Clean-room HTML5 parser and page-tree builder | Use the admitted HTML parser and pinned conformance fixtures for error recovery, misnested markup and table structure. Expose the tree only within the capped, supported level with a clear partial or failed state. Raw parsing executes no script or fetch. [FX-0115-W5-BOUNDED-HTML-TREE-RECOVERY](../planning/feature-wave-5.md#acceptance-design-for-n-1647-to-n-1670). |
| N-1662 Clean-room CSS stylesheet and cascade engine | Use the admitted static or higher render profile to resolve the supported cascade, custom properties, media rules and selectors for the exact viewport, input and applicable stylesheets. Pin profile and coverage. Unknown style or a cap breach does not become a default success. No new stylesheet request path. [FX-0115-W5-PINNED-STYLE-CASCADE-BOUNDS](../planning/feature-wave-5.md#acceptance-design-for-n-1647-to-n-1670). |
| N-1663 Full visual page-geometry engine | Report units, positions and sizes from the admitted engine for supported block, line, flex, grid and table profiles. Preserve measured viewport and font inputs and distinguish partial or unavailable boxes. Report exact positions for this render only, without a claim about every device. [FX-0115-W5-PROFILE-SCOPED-GEOMETRY](../planning/feature-wave-5.md#acceptance-design-for-n-1647-to-n-1670). |
| N-1664 Simultaneous mobile and desktop rendering | Keep mobile first and optional desktop off by default. Compare only equivalent page revisions and declared profiles. Charge both passes to the same positive daily budget and record each pass separately. Side-by-side presentation requires no concurrency or extra allowance. Show an unavailable comparison when one pass lacks support, budget or permission. [FX-0115-W5-MOBILE-FIRST-SHARED-RENDER-BUDGET](../planning/feature-wave-5.md#acceptance-design-for-n-1647-to-n-1670). |
| N-1665 Shadow DOM and modern component reader | Record eligible open component and slot facts under the supported render profile and coverage. Report a closed root as closed and unread. No forcing roots open, hidden-page disclosure, fallback scrape or bypass of script-level admission. [FX-0115-W5-SHADOW-ROOT-EXACT-COVERAGE](../planning/feature-wave-5.md#acceptance-design-for-n-1647-to-n-1670). |
| N-1666 Real human visibility calculator | Label visible, obscured or outside the measured viewport from the exact supported style, geometry and hit-test evidence. Describe the reason for opacity, clipping, small text, contrast, overlays or layout placement where measured. Unknown geometry gives unknown coverage. Report no conclusion about intent or what a given person saw. [FX-0115-W5-MEASURED-VISIBILITY-AND-GAPS](../planning/feature-wave-5.md#acceptance-design-for-n-1647-to-n-1670). |
| N-1667 Visual stability and load-speed analyser | Use existing simulated paint and layout-shift measures, with the exact viewport, versions, clocks and missing phases. Use long-task evidence only where that mode supports it. Keep field data and lab results distinct and omit interaction-to-next-paint. Never label an unsupported or partial profile as measured field performance. [FX-0115-W5-SIMULATED-PAINT-AND-SHIFT](../planning/feature-wave-5.md#acceptance-design-for-n-1647-to-n-1670). |
| N-1668 Clean-room font parser and global text shaper | Decode only supported, eligible files under the existing font caps. Apply the admitted shaper to qualified right-to-left, vertical and complex-script fixtures. Keep typography for existing-page evidence and synthetic conformance. DR-081 deferred RTL site features, translation and market routes remain separate. Record bundled fallback and unsupported cases. No local font discovery, unapproved asset fetch or cross-project reuse. [FX-0115-W5-QUALIFIED-FONT-SHAPING-BOUNDS](../planning/feature-wave-5.md#acceptance-design-for-n-1647-to-n-1670). |
| N-1669 Clean-room headless page painter | Use the qualified headless software paint level and current cage. Keep screenshots off by default. Enforce output size and disk bounds; protect temporary work in the cage, independently check authorised scrubbed pixels and current rights. Bind that check to the immutable final image bytes under the existing 0116 vault identity. Seal only that complete checked image before preview or release. Matching page, capture and dimensions do not permit substituted bytes. Remove interrupted or denied scratch without any released bytes. No active outside browser, unsupported component fallback or storage of a partly authorised screenshot. [FX-0115-W5-PROTECTED-CAPPED-IMAGE-SEAL](../planning/feature-wave-5.md#acceptance-design-for-n-1647-to-n-1670). |
| N-1670 Accessibility tree builder | Derive accessible names, roles, states and heading structure under the pinned admitted profile. Show observed missing names, labels, descriptions or levels separately from unsupported or absent facts. The tree does not prove accessibility conformance and does not interact with forms or controls. [FX-0115-W5-ADMITTED-ACCESSIBLE-TREE](../planning/feature-wave-5.md#acceptance-design-for-n-1647-to-n-1670) |

#### Admitted render and document evidence (N-1671 to N-1689)

Pin each observation to its approved mode, own-site Audit scope,
response revision, engine and component digests, target platform,
cage and broker profile, resource lease and protected generation.
EX-02, QR-003 and QR-029 control component and cage admission.
Without the applicable proof, report unavailable for that level.
Receipts mode stays raw under the existing 512 KiB ceiling and its
sitemap, feed and header reader limits. It runs no page code.
All cases use sealed synthetic fixtures. Reports distinguish scoped
evidence from unknown or partial coverage. No new live probe,
intrusive inspection, vulnerability reproduction or ranking input
follows from these contracts.

| Feature | Exact evidence and required limits | Existing requirements |
|---|---|---|
| N-1671 Adaptive display-mode simulator | Compare admitted own-site Audit views for one exact response and viewport under the requested dark, reduced-motion or high-contrast profile. Count each extra render under the existing site cap. Use text equivalents and mark absent resources or unknown coverage. Show observed layout or readability differences without claiming complete accessibility or changing page settings. | RN-01, RN-05, RN-08, RN-09, RN-12 · [FX-0115-W5-COMPARABLE-DISPLAY-PROFILES](../planning/feature-wave-5.md#acceptance-design-for-n-1671-to-n-1694) |
| N-1672 Process-level rendering cages | Bind exact EX-02 engine and build, separately proved OS cage, launch instance and sole authorised broker transport before parser or render. Each fetch checks current site, policy, robots, reservations, stop and ledger. Stop tears down the whole worker tree and fences late output. Unavailable render fixture or reference browser carries no admission under QR-049. Untrusted content has no direct file, device, clipboard, credential or network access. All permitted guest sub-resource reads use the existing broker controls. Unproved isolation or a stale or failed cage holds the affected work and preserves separate failure evidence. | RN-02, RN-03, RN-05, RN-06, RN-31, RN-32 · [FX-0115-W5-EXACT-CAGE-BROKER-ADMISSION](../planning/feature-wave-5.md#acceptance-design-for-n-1671-to-n-1694) |
| N-1673 Clean-room ECMAScript parser and virtual machine | Keep the source label. Use only a mature EX-02-admitted engine behind the owning interface. Pin level, target, version and limits. An engine without the required cage or supported profile gives unavailable. The project supplies the controls and facts rather than a first-party script virtual machine or a new execution route. | RN-01, RN-02, RN-05, RN-15, RN-31, RN-32 · [FX-0115-W5-ADMITTED-SCRIPT-ENGINE](../planning/feature-wave-5.md#acceptance-design-for-n-1671-to-n-1694) |
| N-1674 Fenced just-in-time speed optimiser | An engine-provided optimisation is optional only under its admitted cage and resource profile. Measure comparable cold and warm work and prove outside-page time and memory stops and unchanged output under the validated profile. If the optimisation lacks proof, keep the eligible unoptimised profile or report unavailable. No first-party JIT or claim of automatic speedup. | RN-01, RN-02, RN-05, RN-15, RN-32 · [FX-0115-W5-BOUNDED-ENGINE-OPTIMISATION](../planning/feature-wave-5.md#acceptance-design-for-n-1671-to-n-1694) |
| N-1675 Clean-room WebAssembly execution engine | An admitted mature runtime runs only within the script-level cage and its named import, memory and step limits. Unsupported imports or exhausted limits hold the module and mark dependent facts partial. A runtime choice adds no file, native execution or socket authority. | RN-01, RN-02, RN-05, RN-15, RN-32, RN-34 · [FX-0115-W5-ADMITTED-MODULE-IMPORT-LIMITS](../planning/feature-wave-5.md#acceptance-design-for-n-1671-to-n-1694) |
| N-1676 Virtual clock fast-forwarder | Apply the admitted page clock to eligible timers and frame order. Stop at idle or the configured virtual cap, with independent real-time, memory and fuel stops. Virtual progress cannot advance broker waits, policy expiry, approval or the scheduler clock. An engine without this capability gives unavailable. | RN-01, RN-05, RN-14, RN-15 · [FX-0115-W5-INDEPENDENT-VIRTUAL-CLOCK-LIMITS](../planning/feature-wave-5.md#acceptance-design-for-n-1671-to-n-1694) |
| N-1677 Deterministic clock and randomness controls | Compare offline runs on identical admitted scrubbed bytes and broker recordings. Pin exact verified scrubbed recording and profile, recipient, per-resource order and engine, settings, capped virtual clock, seed, locale, viewport and bounds. Keep trusted broker and scheduler clocks independent. Missing or erased bytes cannot be acquired or treated as deterministic. A difference is a scoped defect or unsupported result if the profile was unavailable. No external replay request or promise for arbitrary production pages. | RN-13, RN-26, RN-32, RN-44 · [FX-0115-W5-PINNED-OFFLINE-DETERMINISM](../planning/feature-wave-5.md#acceptance-design-for-n-1671-to-n-1694) |
| N-1678 Live client-side data-call interceptor | Correlate only exact admitted broker receipts and tree evidence for the same site and revision. Scrub before persistence and use opaque eligible labels, also in refused request, address and export views. Timing coincidence carries no direct causality. Show blocked sub-resources as partial and retain safe attempt, method, timing and ownership scope. Apply the existing direct-versus-possible linkage rule. No request body, raw credentials, response payload export, extra request or active interception. | RN-03, RN-04, RN-17, RN-29, RN-34, RN-36 · [FX-0115-W5-SCRUBBED-BROKER-TREE-LINKAGE](../planning/feature-wave-5.md#acceptance-design-for-n-1671-to-n-1694) |
| N-1679 Smart soft-404 and broken-app detector | Compare the actual observed response status with the eligible finished-content evidence from its exact render. Preserve confidence and the labelled fixture error rate. Partial or refused sub-resources cap confidence under RN-35. Sparse legitimate pages and timeouts are distinct cases. A finding does not change status, indexing or page state. | RN-05, RN-19, RN-34, RN-35 · [FX-0115-W5-SCOPED-SOFT-404-CONFIDENCE](../planning/feature-wave-5.md#acceptance-design-for-n-1671-to-n-1694) |
| N-1680 Per-script CPU blame meter | Attribute eligible measured CPU or fuel to the approved script observation and origin class, with exact engine profile and coverage. Label advisory wall time separately. Show unknown attribution and scrub source identifiers for output. No claim of causal blame, imported tracking or extra profiling on the ordinary route. | RN-05, RN-16, RN-29, RN-42 · [FX-0115-W5-PROFILE-BOUND-CPU-ATTRIBUTION](../planning/feature-wave-5.md#acceptance-design-for-n-1671-to-n-1694) |
| N-1681 Safe interactive-reveal inspector | Use the admitted bounded own-site Audit interactions only when the control is clearly a passive reveal. Pin exact guest page revision and existing admitted passive interaction class, shared interaction and subresource budget. A safe-looking GET or reveal does not prove no effect. Refuse action-shaped URLs, authentication, navigation, typing and forms. Keep the existing residual-risk record. Ambiguous controls or restricted effects hold. The broker still enforces resource and guest method policy. Store a safe observation of the attempt and no claim that a skipped section was read. | RN-03, RN-04, RN-05, RN-20, RN-37 · [FX-0115-W5-ADMITTED-PASSIVE-REVEAL](../planning/feature-wave-5.md#acceptance-design-for-n-1671-to-n-1694) |
| N-1682 Clean-room PDF layout and link reader | Use only the admitted mature PDF reader in the Audit cage, with its byte, page, object and time limits. Output permitted reading-order text, table and inert link and metadata facts. Hold encrypted, damaged, active or unsupported content under its exact rule. No script, embedded content or destination fetch. | RN-02, RN-21, RN-24, RN-25, RN-40 · [FX-0115-W5-PDF-BOUNDED-INERT-AUDIT-FACTS](../planning/feature-wave-5.md#acceptance-design-for-n-1671-to-n-1694) |
| N-1683 Streaming XML sitemap and feed engine | Stream only already eligible XML and feed formats within the admitted mode, byte, expansion and memory limits. Disable external resources and entities. A truncated index or missing feed gives partial evidence and no coverage claim. Compare modification dates only with existing comparable own-site Audit observations. No extra acquisition or feed output. | RN-21, RN-23, RN-33, RN-38, RN-40 · [FX-0115-W5-XML-FEED-PARTIAL-COVERAGE](../planning/feature-wave-5.md#acceptance-design-for-n-1671-to-n-1694) |
| N-1684 Strict structured-data verifier | Supply the existing feature 1316 checks in 0044 with admitted inert structured fields and matching visible evidence. Pin parser and rule versions, page and observation scope. Separate malformed, unsupported, unproved and unsupported-by-visible-content results. No duplicate check owner, active field script, schema edit or search-appearance guarantee. | RN-21, RN-22, RN-25, RN-29 and feature 1316 · [FX-0115-W5-STRUCTURED-FACT-MATCHING-VISIBLE-REVISION](../planning/feature-wave-5.md#acceptance-design-for-n-1671-to-n-1694) |
| N-1685 Clean-room image format decoder suite | Use the exact admitted mature format and bounded header or pixel profile inside the Audit cage. QR-047 governs native AVIF admission. Unsupported or failed decoders stay visible. Keep dimensions, resource coverage and missing text descriptions scoped. SVG is inert; reading an image cannot fetch referenced assets or expose private metadata. | RN-02, RN-21, RN-24, RN-25, RN-39, RN-40 · [FX-0115-W5-IMAGE-PROFILE-NO-IMPLICIT-ASSET](../planning/feature-wave-5.md#acceptance-design-for-n-1671-to-n-1694) |
| N-1686 Streaming audio and video manifest reader | Use an admitted own-site Audit asset and a bounded brokered range read where its route allows it. Pin media revision and type, exact requested and returned partial range and bytes, time and shared expansion, page and host caps. Hold ignored or mismatched range, full body, redirect or changed media without fallback. Receipts admits only bounded raw, sitemap, feed or header evidence. Hold unsupported or insufficient header, chapter or caption evidence. Never fall back to full download, playback, provider refresh or publishing a transcript. | RN-03, RN-04, RN-05, RN-21, RN-25, RN-33, RN-40 · [FX-0115-W5-AUDIT-MEDIA-EXACT-RETURNED-RANGE](../planning/feature-wave-5.md#acceptance-design-for-n-1671-to-n-1694) |
| N-1687 Clean-room office document extractor | Use an admitted mature reader in the Audit cage and its archive, entry, object and time limits. Show permitted text, headings and scrubbed metadata with coverage. Treat embedded links and formulas as inert. Macros, external resources, encrypted or unsupported formats cannot run or yield guessed content. | RN-02, RN-21, RN-24, RN-25, RN-40 · [FX-0115-W5-OFFICE-READER-INERT-PROJECTION](../planning/feature-wave-5.md#acceptance-design-for-n-1671-to-n-1694) |
| N-1688 Social-card and preview simulator | Use exact current eligible metadata for the page and the reviewed own-site Audit image check where admitted. Bind the broker and size limits. Show verified dimensions, safe metadata and any missing or blocked asset. Do not contact a social provider, post a card or load an additional image outside this route. | RN-03, RN-05, RN-21, RN-25, RN-38, RN-39 · [FX-0115-W5-SOCIAL-CARD-LOCAL-PREVIEW-ONLY](../planning/feature-wave-5.md#acceptance-design-for-n-1671-to-n-1694) |
| N-1689 Global language and regional-link auditor | Use an admitted statistical profile for scope and safety. Compare only feature 564's current eligible declared alternates and return evidence in the own-site Audit route. Unsupported, incomplete or conflicting facts stay unknown. DR-081 gives no regional or translated output, market feature, cross-domain acquisition or markup edits. | RN-01, RN-03, RN-38 and feature 564 · [FX-0115-W5-HREFLANG-EXACT-OWN-SITE-OBSERVATION](../planning/feature-wave-5.md#acceptance-design-for-n-1671-to-n-1694) |

QR-211 qualifies exact admission, passive evidence and refusal on
each declared platform. Retain restricted evidence only under
0032 and 0118. Any change of engine, format or allowable route uses
its own existing approval and validation.

For 1682 to 1685 and 1687 pin the admitted decoder version and format
profile. Check declared and decoded size, objects, depth, time and
expansion. Refuse type mismatch, active script or macro, unscoped
entity or remote context, external resources and incomplete cage
or input. No truncated result can count as complete.

#### Page diff and editorial region evidence (N-1837–N-1838)

Reuse the current EX-02 and QR-029 admission, exact mode, own-site
Audit limits and 0082 broker. Inputs and output are subject to the
current rights, privacy rules and erasure. A missing or blocked
component can only lower the coverage of its dependent facts.

| Owner catalogue entry | Exact rendering contract |
| --- | --- |
| N-1837 Initial HTML vs. finished-page diff viewer | Compare raw and finished observations of the same admitted guest page and comparable revision and capture profile. Report eligible changed links, title, metadata, canonical, headings, structured data and editorial text as typed additions, updates or removals. Where the execution evidence only suggests an association, report it as possible. Missing render, blocked data or incomparable revision is an explicit gap, with no fresh fetch or claimed crawler treatment. [FX-0115-W5-SAME-REVISION-RAW-FINISHED-DIFF](../planning/feature-wave-5.md#acceptance-design-for-n-1815-to-n-1838). |
| N-1838 Editorial content vs. boilerplate separator | Reuse bounded fingerprints and confirmed repeated regions in the admitted mode and sample. Mark navigation, footer, legal and consent regions separately from editorial content. Keep author text and shared introductory or other legitimate content where evidence is insufficient. Record coverage, uncertain extraction and bounded provenance. Offer no hidden material or extra page reads to improve the split. [FX-0115-W5-EDITORIAL-SEPARATION-PRESERVE-AUTHOR-TEXT](../planning/feature-wave-5.md#acceptance-design-for-n-1815-to-n-1838). |

Trace RN-18/27/34 and the existing raw and Audit contracts in QR-217.

### Data

- Per page: facts (visibility, tree summary, accessibility findings, data calls,
  script costs, diffs, soft-404 verdict, structured-data results) in the audit
  tables of spec 0044, each with its level, mode and `partial` flag.
- Paired observations for spec 0105: two complete observations in one
  own-site Audit job, from its two allowed declared identities. Bind
  both to the same site, canonical page and verified revision. Compare
  the main responses before scripts run. Use the same guest request
  conditions apart from the declared identities, and the same parser
  version and comparison rules. Verify the revision with an admitted
  site-owned generation mark that covers both content and settings and
  stays the same across identity variants. A body hash cannot prove
  that. Until its format and tests are admitted, the finding is
  unavailable. Keep evidence references, identities
  and completeness flags, with a version for the pair. A script diff,
  changed viewport, missing revision or incomplete evidence gives no
  pair for this purpose. The pair never uses a third-party identity.
  DR-132 and [spec 0141](0141-audit-crawl-logs-and-technical-toolbox.md)
  own the separate exact-plan approval for acquiring a live pair.
  Keep the Browser leg inside the admitted Audit render path.
- Screenshots and recordings: the vault and archive of spec 0082.
- Nothing here stores page text beyond the short samples spec 0044 allows.

### Settings

Rendering settings live with the crawler's settings; the daily page budget is
`seo.crawl.browser_pages_per_day` of spec 0044 and is not repeated here.

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `render.level` | raw, static, script, paint | raw until an engine is admitted | four values | next crawl |
| `render.viewport` | `mobile` or `desktop` | mobile | two values | next crawl |
| `render.desktop_compare` | on or off | off | two values | next crawl |
| `render.page_ms` | virtual milliseconds | 10,000 | 1,000 to 60,000 | next crawl |
| `render.real_ms` | real milliseconds | 20,000 | 1,000 to 120,000 | next crawl |
| `render.fuel` | script steps | 50,000,000 | 1,000,000 to 1,000,000,000 | next crawl |
| `render.memory_mib` | MiB per page | 512 | 64 to 4,096 | next crawl |
| `render.max_nodes` | whole number | 200,000 | 1,000 to 5,000,000 | next crawl |
| `render.max_subresources` | whole number | 150 | 1 to 2,000 | next crawl |
| `render.max_frames` | whole number | 20 | 0 to 200 | next crawl |
| `render.max_frame_depth` | whole number | 3 | 0 to 10 | next crawl |
| `render.max_css_kib` | KiB of stylesheets | 2,048 | 16 to 65,536 | next crawl |
| `render.max_selectors` | whole number | 100,000 | 1,000 to 5,000,000 | next crawl |
| `render.parallel` | whole number | 1 | 1 to 4 | next crawl |
| `render.dpr` | ratio | 3 (mobile), 1 (desktop) | 1 to 4 | next crawl |
| `render.locale` | fixed tag | `en-GB` | fixed in version 1 | at start |
| `render.timezone` | text | `UTC` | a zone name | next crawl |
| `render.start_date` | date and time | `2026-01-01T00:00:00Z` | a valid date | next crawl |
| `render.screenshot_max_px` | pixels | 4,000 | 100 to 16,000 | next crawl |
| `render.soft404_langs` | fixed list | `en` | fixed in version 1 | at start |
| `render.max_bytes_mib` | MiB per page | 20 | 1 to 512 | next crawl |
| `render.max_interactions` | whole number | 25 | 0 to 500 | next crawl |
| `render.min_font_px` | pixels | 4 | 1 to 12 | next crawl |
| `render.min_contrast` | ratio | 4.5 | 1 to 21 | next crawl |
| `render.soft404_chars` | characters | 200 | 0 to 5,000 | next crawl |
| `render.deterministic` | on or off | on | two values | next crawl |
| `render.screenshots` | on or off | off | two values | next crawl |
| `render.png_max_kib` | KiB | 1,024 | 16 to 16,384 | next crawl |
| `render.display_modes` | list | none | dark, reduced_motion, high_contrast | next crawl |
| `doc.max_ms` | milliseconds per document | 30,000 | 1,000 to 300,000 | next crawl |
| `doc.media_range_kib` | KiB per range | 64 | 4 to 1,024 | next crawl |
| `doc.max_pages` | whole number | 500 | 1 to 10,000 | next crawl |
| `doc.max_objects` | whole number | 500,000 | 1,000 to 10,000,000 | next crawl |

There is no setting that lets the cage reach the network directly, makes the
broker send anything but GET and HEAD, switches off the legal wall for sub-
resources, or lets a page see local files. The limits can be lowered or raised
only inside the ranges above, and a page that exceeds one is stopped.

### Failure codes

SUT-1500 render level not available (no admitted engine); SUT-1501 page limit
reached (the limit is named); SUT-1502 cage failed or killed; SUT-1503 script
error; SUT-1512 script limit reached; SUT-1504 sub-resource refused by the
legal wall (a report code; the page sees a network error); SUT-1505 non-GET
request refused; SUT-1506 document over its limits; SUT-1507 decompression
bomb stopped; SUT-1508 decoder blocked (no usable component); SUT-1509
recording refused (privacy gate); SUT-1510 render not deterministic (a
defect); SUT-1511 closed shadow root not read. Codes SUT-1500 to SUT-1599 are
reserved for this spec.

### Security and privacy

- Untrusted pages, scripts and files run only in the cage, with no local
  access and no network except through the broker and its legal wall.
- No credential, cookie or personal data is kept; samples are scrubbed (spec
  0082).
- Page text is data and never an instruction, including text that looks like a
  command to a script, an agent or a person.
- Nothing in this spec changes a page, submits a form or takes an account
  action.

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| RN-01 | A level with no admitted component MUST report "not available" (SUT-1500) and MUST NOT produce invented results. | UT-RN-01 |
| RN-02 | Untrusted pages, scripts and documents MUST run only in the cage, with no local file, device, clipboard or network access. | PL-RN-01, IT-RN-01 |
| RN-03 | Every sub-resource request MUST go through the broker and the full legal wall of spec 0082, and a refused request MUST appear to the page as a network error (SUT-1504). | UT-RN-02, IT-RN-02 |
| RN-04 | The broker MUST make only GET and HEAD requests (and range GETs), and MUST refuse and list any other (SUT-1505). | UT-RN-03, PL-RN-02 |
| RN-05 | Every per-page limit MUST be enforced from outside the page, and a page at a limit MUST be stopped and its facts marked partial (SUT-1501). | UT-RN-04, FI-RN-01 |
| RN-06 | A cage that crashes or hangs MUST be killed, the page marked failed (SUT-1502) and the crawl MUST continue. | FI-RN-02 |
| RN-07 | Leviathan-Browser MUST be the declared identity of every cage request. | UT-RN-05 |
| RN-08 | The default render MUST be mobile-first, and a desktop pass MUST be off unless set and MUST count in the render budget. | UT-RN-06 |
| RN-09 | The visibility calculator MUST report visible or not with a reason for every element, and MUST state layout facts without claiming intent. | UT-RN-07, BD-RN-01 |
| RN-10 | Loading metrics MUST be labelled simulated, and interaction-to-next-paint MUST NOT be reported. | UT-RN-08 |
| RN-11 | Screenshots MUST be off by default, size limited and stored in the vault under the disk cap. | UT-RN-09 |
| RN-12 | The accessibility tree MUST follow the accessible-name rules and report the listed defects. | UT-RN-10 |
| RN-13 | With determinism on, two renders of identical input MUST be identical, and a difference MUST be reported as a defect (SUT-1510). | UT-RN-11, PR-RN-01 |
| RN-14 | The virtual clock MUST stop at the virtual limit or when the page is idle, and timers MUST run in order. | UT-RN-12 |
| RN-15 | Script errors MUST be collected and never swallowed, and a script that exhausts fuel MUST be reported (SUT-1503). | UT-RN-13 |
| RN-16 | The cost meter MUST attribute script time to script URLs and split first-party from third-party. | UT-RN-14 |
| RN-17 | The data-call tracker MUST record every post-load request and MUST label its link to tree changes "direct" or "possible". | UT-RN-15 |
| RN-18 | The diff MUST list every link, title, meta tag, canonical, heading, structured-data block and text block that depends on scripts. | UT-RN-16, BD-RN-02 |
| RN-19 | The soft-404 detector MUST give a confidence and its evidence and MUST NOT change a page's status. | UT-RN-17 |
| RN-20 | The inspector MUST open only elements marked safe, MUST NOT activate form controls or risky labels or type, and MUST list each interaction. | UT-RN-18, PL-RN-03 |
| RN-21 | Every decoder and reader MUST enforce its size, object, depth and time limits, stop a decompression bomb (SUT-1506, SUT-1507) and refuse the whole file if any archive entry path escapes the work folder. | UT-RN-19, FZ-RN, PL-RN-06, PL-RN-07, PL-RN-08, PL-RN-09 |
| RN-22 | A declared structured-data fact with no visible support MUST be reported. | UT-RN-20 |
| RN-23 | Sitemap and feed reading MUST be a bounded-memory stream, and a "last modified" check MUST run only on the user's own sites. | UT-RN-21, MS-RN-01 |
| RN-24 | Office macros, SVG scripts and PDF scripts MUST NEVER run. | PL-RN-04 |
| RN-25 | A decoder with no usable component MUST be marked blocked (SUT-1508), not faked. | AU-RN-01 |
| RN-26 | Recordings MUST pass the privacy gate, be limited to the user's own sites and the synthetic web, and replay MUST open no outside connection (SUT-1509). | UT-RN-22, IT-RN-03 |
| RN-27 | Boilerplate fingerprints MUST use bounded memory and MUST leave main content. | UT-RN-23, MS-RN-02 |
| RN-28 | A closed shadow root MUST be reported "closed, not read" (SUT-1511). | UT-RN-24 |
| RN-29 | Page text MUST be treated as data and never as an instruction. | PL-RN-05 |
| RN-30 | Each feature MUST have a ledger or conformance item, a contract or conformance test and an independent reviewer (spec 0082, spec 0083). | AU-RN-02 |
| RN-31 | An engine MUST support a host fetch hook or be admitted only with all network blocked except a broker pipe, and a test MUST cover DNS, UDP, WebRTC and a local proxy attempt. | PL-RN-01, IT-RN-04 |
| RN-32 | The render levels MUST be "not available" on a system until its cage test passes, and an engine-dependent limit it cannot meet MUST lower that capability or reject the engine. | UT-RN-25 |
| RN-33 | In Receipts mode only the raw level and the sitemap, feed and header readers MUST run, within the 512 KiB page ceiling. | UT-RN-26 |
| RN-34 | A fact that depends on a refused sub-resource MUST be marked "partial: blocked sub-resource" with lower confidence. | UT-RN-27 |
| RN-35 | Soft-404 confidence MUST be capped at low when the render was partial, and the detector's false-positive rate on its fixture set MUST be reported. | UT-RN-17, MS-RN-03 |
| RN-36 | A data-call link MUST be "direct" only within 100 virtual milliseconds of the response and with its values in the change. | UT-RN-15 |
| RN-37 | A page MUST NOT save storage or navigate away, and a GET to a logout, delete or unsubscribe path MUST be refused. | UT-RN-28, PL-RN-03 |
| RN-38 | Hreflang return links, the social-card fetch and the "last modified" check MUST run only on the user's own sites, and boilerplate fingerprints from a rival or niche crawl MUST be discarded at its end. | UT-RN-29 |
| RN-39 | Images not checked because a decoder is blocked MUST be listed. | UT-RN-30 |
| RN-40 | Decoders MUST use the shared `web.max_expansion_ratio` and the limits `doc.max_ms` and `doc.media_range_kib`. | UT-RN-19 |
| RN-41 | `render.parallel` times `render.memory_mib` MUST fit the audit-mode share of spec 0078, or the start MUST fail. | UT-RN-31 |
| RN-42 | Loading metrics MUST follow the stated approximations and report total blocking time only where long-task timing exists. | UT-RN-08 |
| RN-43 | A crash, a script error and a script limit MUST be reported as separate codes (SUT-1502, SUT-1503, SUT-1512). | UT-RN-13 |
| RN-44 | The settings of the table MUST be enforced as stated, including frame, stylesheet, selector, screenshot, date, locale and time-zone settings. | UT-RN-32 |
| RN-45 | A paired observation MUST meet the own-site Audit, identity, completeness and revision rules in Data. | IT-LE-09 (spec 0105) |

## How it is tested

Designed, not run.

- **Unit (UT-RN):** the limit arithmetic with one value just inside and one just
  outside each limit; visibility on a table of fixture pages (opacity, clipping,
  covering, tiny text, same colour, off screen); accessible names; the diff on
  pairs of first and finished trees; soft-404 on honest empty pages and real
  error pages; the interaction rules on a table of safe and unsafe controls.
- **UT-RN-26:** cut a Receipts read at the configured ceiling, including
  decoded bytes from a compressed response. Refuse a setting over the
  maximum. An empty shell starts no script and fetches no resources.
- **Planted faults (PL-RN):** each must be refused with its named message
  while an honest page passes.
  - **PL-RN-01 (socket):** a page script that opens a socket.
  - **PL-RN-02 (form post):** a page that posts a form.
  - **PL-RN-03 (risky control):** a page with a form control, a risky label
    and a GET to a logout, delete or unsubscribe path.
  - **PL-RN-04 (active content):** a PDF with an embedded script, an Office
    macro and an SVG script.
  - **PL-RN-05 (hostile text):** a page with a hostile "click this button"
    instruction.
  - A zip bomb is covered by the RN-21 fuzz item FZ-RN.
  Office-file faults, each also a requirement RN-21 and RN-24 test:
  - **PL-RN-06 (DTD ban):** a Word, spreadsheet and presentation file whose
    XML holds a DOCTYPE with an internal subset, an external entity naming a
    local file and one naming a URL, and a billion-laughs entity chain. Each
    is refused before expansion, nothing is read from disk or fetched, and
    an honest file passes.
  - **PL-RN-07 (zip-slip path):** archive entries named `../x`,
    `a/../../x`, `/etc/x`, `C:\x`, a name with a backslash separator, a
    name with a null byte and a symbolic-link entry. Each is refused, no
    file is written outside the cage's work folder, and the whole file is
    refused, with no entry read.
  - **PL-RN-08 (nesting depth):** an XML part nested one level inside and
    one level over the depth cap, and an archive inside an archive past the
    cap. The first passes, the others stop with the limit message
    (SUT-1507) and no stack overflow.
  - **PL-RN-09 (metadata cleaning):** a PDF and an Office file whose author
    and title hold control characters, bidi overrides, a newline meant to
    forge a log line and a value over the length cap. The report and log
    carry the cleaned, capped text on one line, and a clean value is
    unchanged.
- **Integration (IT-RN):** a page in the cage over the broker with robots
  rules and a private-address redirect; a replay of a recorded exchange; a
  render in two passes (mobile and desktop).
- **Fault injection (FI-RN):** a cage killed mid-render, a script that never
  ends, a page that grows without bound, a broker that answers slowly.
- **Property (PR-RN):** random pages render identically twice with determinism
  on.
- **Fuzz (FZ-RN):** hostile HTML, CSS, fonts, PDF, images, XML and archives
  never crash the cage or escape it.
- **Behaviour (BD-RN):** the Dana scenario above, step by step.
- **Audit (AU-RN):** each blocked component is listed with its reason, and a
  sample of features is traced from the pillar list to the ledger item to the
  test.
- **Conformance:** the browser standards suite, the reference-browser oracle and
  the synthetic web lab run under spec 0116.
- **Measurement (MS-RN-01 to 03):** memory and time of a sitemap of a million
  entries, boilerplate memory over 10,000 pages, render time and memory of a
  typical page per level on the 8 GiB reference, and the soft-404 false-positive
  rate on the labelled fixtures. Provisional targets, to be confirmed after the
  first run of EX-02: a million-entry sitemap under 256 MiB, boilerplate over
  10,000 pages under 128 MiB, and a typical page at the script level under 512
  MiB and 20 seconds of real time.

## Changes from the source

The pillar lists asked for a clean-room browser engine, script virtual
machine, just-in-time compiler, WebAssembly runtime and decoder suite.
Leviathan uses mature components behind our traits and writes only the cage,
broker, limits and the analysis logic itself (ADR-0022, AGENTS.md section 6
rule 14). The engine choice waits for experiment EX-02 (RQ-011, QR-003); until
then Audit mode reads raw HTML. The "compare against mainstream browsers"
oracle and the standards suites belong to spec 0116. Features that need a
second read of a page work only on the user's own sites (DR-062).
