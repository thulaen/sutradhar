# Current decisions: R029–R052

Part of [current decisions](current-decisions.md); read its status and the
[30 October 2026 update](current-decisions.md#30-october-2026-update) first.

## Analytics, SEO and related features

- R029
  - Current requirement: Preserve the requested Adobe Analytics-like capability
    scope and build a source-backed capability matrix. Do not claim parity
    until every claimed capability has evidence. Combine first-party analytics
    with Google Search Console, Bing and GA4; paid backlink-service
    dependencies are removed.
  - Source: Q042–Q044, Q115, Q118
- R030
  - Current requirement: Use separate visitor identifiers, not named
    forum-account browsing histories. Detailed histories require explicit
    consent and stop on withdrawal. Pseudonymous data is still treated as
    personal data rather than assumed anonymous.
  - Source: Q076–Q079
- R031
  - Current requirement: Count one page view for a page per session under the
    chosen deduplication rule. Retain detailed events for 90 days and
    anonymous aggregate reports for two years. Keep operational state locally;
    upload compressed raw analytics to verified object storage, then remove
    confirmed local copies.
  - Source: Q078, Q080–Q084
- R032
  - Current requirement: Use a bounded local queue during storage outages.
    If full, stop that collection, report the gap and preserve forum
    availability. Support optional AWS S3 and Google Cloud Storage and
    verified, resumable migration. Explain costs and warn at budgets without
    silently stopping otherwise authorised cloud work.
  - Source: Q081–Q083, Q125, Q127–Q129
- R033
  - Current requirement: Prefer GA4 for supported overlapping traffic
    metrics; first-party data supplies other metrics and gaps without
    double-counting. Prioritise new, changed and important pages for
    constrained inspection APIs, with rotating samples. Missing or incomplete
    source coverage is visible.
  - Source: Q118, Q122–Q123
- R034
  - Current requirement: Preserve the full SEO-signal request in Q115 for
    review and traceability. Crawl the forum and check referenced external
    destinations, not the wider web. Do not change titles. Other site SEO
    remedies are reports and proposals, without automatic or one-click
    canonical, redirect or indexing changes.
  - Source: Q115, Q119–Q121
- R035
  - Current requirement: Build a knowledge graph for relevance and an optional
    tag-description feature, off by default, with a 120-character maximum.
    Use reviewed Wikidata, Wikipedia and forum evidence, check ambiguity and
    freshness, write original descriptions, and require preview then batch
    approval. Pixie is a research candidate, not automatically cleared or
    implemented.
  - Source: Q054, Q056–Q057
- R036
  - Current requirement: Provide Hot, Rising, Top, Best and Controversial
    sorting on relevant lists and replies; chronological replies remain the
    default. Explain graph relevance separately from overall ranking.
    Independently specify Google-inspired relevance and spam concepts without
    claiming proprietary equivalence.
  - Source: Q049, Q054–Q055, Q059, Q110, Q116
- R037
  - Current requirement: Provide usefulness voting for members and guests,
    displaying the viewer's vote, counts and a summary. Accepted guest votes
    have equal influence after checks. Feed all reaction types into learning;
    Sad and Angry are negative even when their XenForo mapping is neutral.
    Suspicious voting is flagged for review, not silently discarded.
  - Source: Q050, Q058, Q060, Q112–Q116, Q170
- R038
  - Current requirement: Public karma reflects public content and updates
    when reactions or content change. Alternative-account hints require
    supporting independent signals and moderator review; no automatic bans.
    Expert routing needs moderator-approved invitations, member preferences
    and rate limits.
  - Source: Q102, Q105, Q110, Q113–Q114, Q148
- R039
  - Current requirement: Optional dedicated hub pages group useful public
    material and may include solved answers. Their links need individual
    approval and follow the five-link default. Staff may request long-thread
    summaries; approved summaries show their date and covered reply, and are
    hidden if deletion or privacy changes make them unsafe.
  - Source: Q102, Q106–Q111
- R040
  - Current requirement: Agents may create drafts through authorised
    interfaces; staff choose destination and an authorised publishing account
    in the GUI and approve publication. Require useful new contribution and
    novelty checks, not an unprovable promise of zero textual overlap.
    Existing-post rewriting is outside v1 linking.
  - Source: Q018, Q029, Q151, Q171

## Framework, interfaces and experience

- R041
  - Current requirement: Use the approved glossary for system parts and plain
    English inside them. Preserve the exact Q116 glossary, including kavach.
    Module class names may follow the demonstrated Setu class form. English
    UI labels lead, with the glossary name alongside. Missing names use
    English placeholders until approved.
  - Source: Q116, Q130
- R042
  - Current requirement: Provide configuration profiles, dependency
    coordination, module discovery/lifecycle, events, scheduled/batch jobs,
    repositories, review, audit, diagnostics, signing and a testing kit.
    Modules depend on core contracts; core must not import feature modules.
  - Source: Q018, Q102, Q104, Q116, Q192
- R043
  - Current requirement: PHP supplies approved forum data to Python. Python
    does not directly access the XenForo database. Changes use PHP/XenForo
    services. Third-party execution runs in separately restricted workers
    with declared capabilities and resource allowances.
  - Source: Q035, Q094, Q104, Q173
- R044
  - Current requirement: GUI, the sutra command line and MCP expose documented
    capabilities through the same permission-checked action layer. Provide
    thorough discovery, help and embedded documentation, including the
    requested broad command coverage. No interface gains hidden publication or
    moderation authority.
  - Source: Q018, Q021–Q024, Q047
- R045
  - Current requirement: Support Codex, Claude and Gemini collaboration
    through qualified adapters and A2A-style coordination. A protocol alone is
    not conflict prevention. Require manual session joining except explicitly
    authorised scheduled jobs, one writer per file and recorded safe handover.
  - Source: Q068, Q072–Q073, Q189
- R046
  - Current requirement: Keep a rolling SQLite run ledger on the validation
    host: ownership, leases, checkpoints, evidence references and handover
    history. Preserve abandoned drafts and revoke previous ownership before
    transfer. The database itself stays out of public Git.
  - Source: Q073, Q189
- R047
  - Current requirement: Use native XenForo UI foundations plus reviewed chart
    libraries. Provide interactive statistics, drill-downs, before/after
    views, responsive layouts and light/dark themes. Settings remain
    discoverable with collapsible detail; users may choose suitable mobile
    views.
  - Source: Q050, Q131–Q132, Q164
- R048
  - Current requirement: Use the requested Kraken palette research, BBC GEL
    typography/spacing and GOV.UK interaction patterns without protected
    branding, fonts or imagery. Verify WCAG 2.2 AA and actual supported
    mainstream browser environments, including mobile and real Safari where
    claimed.
  - Source: Q050, Q184–Q185, Q192
- R049
  - Current requirement: Provide a point-and-click setup path with an agent
    or authorised host administrator handling necessary server preparation.
    Advanced commands remain optional reference. Installation preflight must
    verify the supported host; optional cloud creation needs supported login
    and approval of cost and region.
  - Source: Q124, Q127, Q207
- R050
  - Current requirement: Notify before an installed-product upgrade and obtain
    approval, then pause affected features. An explicit owner override may
    waive a backup precheck only, not code gates. Uninstall offers export and
    an explicit retain/delete choice for owned data while retaining published
    links.
  - Source: Q039, Q134–Q136
- R051
  - Current requirement: Group recurring operational alerts on the dashboard
    and in XenForo staff alerts. Explain faults in plain English with a
    central error code and a manual link. Diagnostic bundles are scrubbed and
    previewed before export.
  - Source: Q041, Q117, Q137–Q138, Q192
- R052
  - Current requirement: Separate permanent minimal audit events from
    identifying details and post text. Erase the separate private data when
    required without rewriting the minimal event sequence. Purge or
    regenerate affected indexes, caches, snapshots, suggestions and lab data
    after deletion/erasure.
  - Source: Q102, Q148, Q209
