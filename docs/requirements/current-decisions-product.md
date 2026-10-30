# Current decisions: R001–R028

Part of [current decisions](current-decisions.md); read its status and the
[30 October 2026 update](current-decisions.md#30-october-2026-update) first.

## Product and delivery

- R001
  - Current requirement: Sutradhar is a Linux-hosted, Spring Boot-inspired
    modular SEO framework integrated with XenForo 2.3. Its PHP integration
    targets PHP 8.5; heavy analysis runs in a packaged Python service.
    XenForo's supported integration rules win at the forum boundary.
  - Source: Q010, Q018, Q031–Q037,
    [Python follow-up](sources/follow-up-decisions.md), Q061, Q116
- R002
  - Current requirement: Prove the system on an authorised forum first,
    initially in English, for workloads exceeding one million posts. Larger
    bulk-work scenarios remain in the requested tests. Point-and-click
    operation is the primary user experience and must not depend on a hosting
    control panel.
  - Source: Q004, Q012–Q014, Q034, Q124, Q207
- R003
  - Current requirement: Reader usefulness and search benefits both matter.
    Relevance remains essential. Balanced and Aggressive playbooks explore
    useful opportunities, including overlooked content; aggressive does not
    weaken safety or approval.
  - Source: Q001, Q005, Q007
- R004
  - Current requirement: A complete first product release requires at least
    6,000 genuinely distinct, implemented and qualified methods and all agreed
    release scope. Incremental development commits are allowed; they must not
    be described as a complete release. No renamed copies, parameter tweaks or
    superficial recipe rearrangements pad the count.
  - Source: Q002, Q085, Q140, Q144–Q145
- R005
  - Current requirement: The method scope includes linking, relevance, spam,
    quality, confidence, statistical models, mathematical procedures, NLP,
    machine learning, deep learning, ensembles, wrappers, meta-methods and
    meaningful research variants. Count a distinct method once even when
    several features use it.
  - Source: Q019 as replaced by Q145
- R006
  - Current requirement: Provide more than 10,000 distinct, tested playbook
    situations. Every counted case must have a meaningful difference and an
    executable expected outcome; labels alone do not count.
  - Source: Q018, Q020
- R007
  - Current requirement: Every feature is specified before coding, with
    behaviour examples, a test-first plan and acceptance evidence. The owner
    approves each feature specification and changes to governing requirements.
    Use G3Doc-style documentation, plain English and British spelling.
  - Source: Initial request, Q054, Q060, Q139, Q192, Q199

## Internal linking

- R008
  - Current requirement: Analyse existing eligible content with visible
    progress and durable checkpoints, then continuously handle new and edited
    threads and resources. Do not claim literal sentience or impossible
    perpetual progress through outages.
  - Source: Q005, Q014, Q018, Q051, Q087
- R009
  - Current requirement: V1 creates suggestions for a restricted
    moderator/staff review page. An authorised person approves each link
    before insertion. There are no author-editor suggestions and no automatic
    insertion mode in v1. Shadow work remains available for testing.
  - Source: Q006, Q009, Q103, Q154, Q157, Q172
- R010
  - Current requirement: Link insertion wraps only existing source words.
    It never inserts, removes or rewrites source prose. If no safe existing
    phrase is available, choose another existing phrase or abstain.
  - Source: Q151
- R011
  - Current requirement: Offer up to five alternative destinations per proposed
    phrase. Default to at most five existing content-link placements per whole
    thread, including replies, or resource, including updates. This limit is
    configurable; quotes, interface navigation and automatic mentions do not
    consume this content-link allowance.
  - Source: Q008, Q049, Q052–Q053, Q101
- R012
  - Current requirement: Do not create repeated source-thread to
    destination-thread connections. A useful reply may be the exact
    destination, but use the canonical parent thread for duplicate checks.
    Never link the source to itself or nest a link inside another link.
  - Source: Q053, Q098, Q145, Q148
- R013
  - Current requirement: Only guest-visible content is eligible for link
    indexing, suggestions and judging. Check source and target visibility
    again immediately before an accepted change. Private messages, drafts and
    staff-only content must not enter those pipelines.
  - Source: Q021, Q100, Q145, Q148
- R014
  - Current requirement: Threads and resources are eligible destinations;
    approved public hub pages become eligible when that optional module is
    enabled. XenForo tags are not link destinations. Resource features require
    compatible XenForo Resource Manager to be present.
  - Source: Q054, Q097, Q109, Q165
- R015
  - Current requirement: Show the first useful suggestion within the qualified
    30-second target, with live progress. Approval waits until all methods
    applicable to currently available evidence complete and all hard safety
    rules pass. Missing evidence is visible and is never fabricated or treated
    as zero.
  - Source: Q015, Q041, Q086, Q089–Q090, Q126
- R016
  - Current requirement: Source edits invalidate old approval and require
    fresh analysis and approval. Pending suggestions expire after 30 days and
    earlier when content or visibility makes them stale. Apply accepted
    changes through XenForo's supported services and preserve edit/history
    integrity.
  - Source: Q091, Q148, Q182
- R017
  - Current requirement: Undo only the add-on's change, preserving later
    independent edits; conflicts require review. Published links remain when
    the add-on is uninstalled. A deliberately removed connection stays
    suppressed until staff reset it. A rejected connection may return only
    after meaningful content changes.
  - Source: Q092–Q093, Q153, Q183
- R018
  - Current requirement: Learn from accepted, rejected and removed suggestions,
    with optional reason buttons. There is no author opt-out from
    moderator-approved linking; this does not remove analytics consent rights.
  - Source: Q030, Q093, Q145, Q169

## Analysis, tuning and resource limits

- R019
  - Current requirement: Run every applicable method rather than silently
    selecting a smaller convenient subset. Missing-input methods remain
    visibly waiting, with their settings and positive weights preserved.
    Faulty methods are isolated, reported with a plain-English error and code,
    and dependent outputs are withheld.
  - Source: Q040, Q045, Q086, Q117
- R020
  - Current requirement: Use bounded chunks, reserved capacity, durable
    checkpoints, idempotent work and recovery after restarts. A qualified
    failure-recovery target permits at most one minute of repeated unfinished
    computation per worker; committed completed work is retained.
  - Source: Q051, Q087–Q088, Q148, Q162
- R021
  - Current requirement: The Python service and all workers belonging to that
    service share an operating-system cap of 2 GB on a forum host. The lab
    profile has an 8 GB combined ceiling. Separately admitted extension
    services need explicit declared allowances; they cannot silently be
    omitted from the published resource accounting.
  - Source: Q036, Q160, Q173–Q174
- R022
  - Current requirement: Keep PHP thin and cap the add-on's incremental peak
    at 8 MB per request, measured enabled versus disabled. Do not cap
    XenForo's PHP workers to satisfy this promise. Essential bounded safety
    validation is still required. Search, database and other shared services
    have separately stated allowances.
  - Source: Q016, Q174
- R023
  - Current requirement: Slow, shrink batches or refuse new work before
    reaching the hard memory ceiling; a refusal explains why. Test recovery
    from operating-system termination because a hard cap cannot itself
    guarantee graceful behaviour.
  - Source: Q174, Q176
- R024
  - Current requirement: Tune only settings inside approved bounds. Continuous
    weight changes are limited to 5% of the current value per step; discrete
    settings need approved bounded steps. Preserve positive method weights.
    Compare changes in the background before activation and reverse only the
    failing change or linked group.
  - Source: Q042, Q045, Q048–Q050, Q154, Q167–Q168
- R025
  - Current requirement: Record evidence, benefits, drawbacks, before/after
    results and configuration history. Apply settings atomically between jobs,
    with each job retaining an immutable settings version. Mark settings that
    genuinely require a restart. Agents apply requested configuration changes
    only when explicitly instructed.
  - Source: Q022, Q047–Q050, Q133, Q152
- R026
  - Current requirement: Marimo notebooks are the live home for method
    explanations, experiments, benchmarks and regression examples. They import
    production functions; production never imports lab code and requests never
    execute notebooks. Architecture and feature rules live in linked written
    specifications.
  - Source: Q145, Q150, Q192
- R027
  - Current requirement: Support more than 200 reviewed Python libraries,
    organised into tested packs; install all approved packs and load only what
    is needed. Named replacements need the owner's approval. Source
    suggestions are candidates, not evidence of compatibility, rights
    clearance or admission.
  - Source: Q028, Q032, Q038, Q093, Q096
- R028
  - Current requirement: Accept approved data-only model formats, preferring
    safetensors for tensors where applicable. Reviewed JSON and other
    non-executable formats are allowed; executable downloaded model payloads
    and pickle-based loading are not. Pin models and track their versions.
  - Source: Q148, Q163
