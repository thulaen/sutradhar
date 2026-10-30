# Current decisions: R053–R082

Part of [current decisions](current-decisions.md); read its status and the
[30 October 2026 update](current-decisions.md#30-october-2026-update) first.

## Quality evaluation and release evidence

- R053
  - Current requirement: Preserve every hard safety case in Q148, including
    BBCode boundaries, visibility changes, stale writes, Unicode offsets,
    event duplication/order, job leases, signatures, injection, privacy,
    migrations and vector-version compatibility. Starred cases block any
    relevant release; all cases are covered as their modules are built.
  - Source: Q145, Q148
- R054
  - Current requirement: First-party code targets 100% line, function, branch,
    decision and MC/DC coverage. Required checks include unit, property,
    mutation, regression, performance, stress, chaos, security, dependencies,
    faults, memory, concurrency, database, maintainability, duplicate-code and
    documentation/specification checks. Soak and full quality judging are
    release-only.
  - Source: Q060, Q062, Q148, Q183, Q186
- R055
  - Current requirement: Use controlled repeated performance comparisons
    against established equivalent-work baselines; noise is not a proven
    regression or improvement. Equivalent mutations require reviewed code or
    rule changes rather than unexplained exclusions. Map appropriate NASA/Meta
    guidance into one owner-approved PHP/Python rulebook; do not claim
    certification.
  - Source: Q064–Q066
- R056
  - Current requirement: PHP and Python first-party source uses at most 80
    columns and at most 300 code lines per file. Avoid duplicated explanations
    and duplicated implementations without hiding logic or suppressing
    meaningful checks. Docs have no line-count cap.
  - Source: Q062
- R057
  - Current requirement: Require code, performance and security reviews with
    durable evidence. Honest self-review may satisfy engineering-review roles;
    it is labelled as self-review. Required GitHub AI review is additional and
    must cover the current candidate. Review silence, a stale result or a
    skipped job is not a pass.
  - Source: Q060, Q069, Q071, Q193
- R058
  - Current requirement: Unit tests need no corpus; property tests generate
    content; a committed synthetic fixture of about 300 threads covers edge
    cases. Private development and nightly datasets use the specified
    2,000- and 50,000-thread tiers on the validation host. They never enter
    public Git, hosted runners or logs.
  - Source: Q142 as replaced by Q155, Q197
- R059
  - Current requirement: Release judging uses at least 500 fixed and at least
    500 fresh valid suggestions, assessed separately. Maintain unseen
    final-assessment data, split by thread/time to prevent leakage, and check
    overall and important groups. Independently identified valid linking
    opportunities must reach 95% coverage.
  - Source: Q143–Q149, Q205
- R060
  - Current requirement: Each required quality result must have a lower 95%
    confidence bound of at least 95%, not merely 95% observed passing. Hard
    rules remain 100%; reconstruction recall and anchor agreement must not
    regress at release. After launch, add the review-page accept rate over the
    latest 200 decisions; its decision threshold must be specified and
    approved rather than invented here. Freeze sampling, intervals and
    subgroup rules before scoring; never drop difficult cases or repeatedly
    sample until a pass appears.
  - Source: Q145–Q148, Q173, Q175–Q176
- R061
  - Current requirement: Use two providers for release quality judging: the
    approved OpenAI and Anthropic judge route on the validation host. A single
    explicit exception permits bounded redacted packs to leave that host.
    Verify account terms, training controls, actual service locations,
    no-tools/read-only enforcement and credential isolation before use. Keep
    no prompt transcripts.
  - Source: Q156, Q159 as amended by Q173 and Q176, Q208
- R062
  - Current requirement: Judge packs contain the anchor sentence with bounded
    surrounding context, target title and a bounded excerpt; reply targets
    include their actual reply context. Redact identifying data and treat post
    text as untrusted data. Enforce the strict verdict schema.
  - Source: Q173, Q177
- R063
  - Current requirement: Round one is blind. For disagreements, both judges
    see the other's anonymous verdict/reason in round two. In round three, a
    fail cites rubric criteria and a pass answers them. Unresolved cases are
    failures in the overall score and remain visible as contested; later
    moderator decisions affect future learning, not completed gate results.
  - Source: Q176, Q178
- R064
  - Current requirement: Permit one repair of an invalid verdict. Count initial
    invalid answers in the per-judge/per-round reliability rate even if
    repaired; more than 2% invalid blocks release. First-round agreement
    below 95% also blocks release until approved rubric/judge correction and
    a fresh assessment. Track flip rates and suspicious deference.
  - Source: Q176, Q179, Q206
- R065
  - Current requirement: Full judging starts on an exact release-candidate
    tag, resumes across usage windows and reuses valid context-specific
    cached verdicts. Cache keys distinguish content, model, rubric,
    prompt/schema/redaction versions, round and opposing reasoning. Changed
    rubric/model or relevant content invalidates results.
  - Source: Q173, Q176, Q180, Q183
- R066
  - Current requirement: Nightly work performs non-judge checks only. A
    configurable, optional 50-suggestion merge warning may have a zero budget.
    Drift sampling runs only when judges are enabled. A 72-hour supervised
    uninterrupted soak is a release gate; it survives the editing computer
    switching off and records durable progress.
  - Source: Q161, Q180, Q183, Q186

## Repository, privacy and operations

- R067
  - Current requirement: The public project is MIT-licensed. Independently
    implement first-party logic from approved specifications, with source and
    rights records. Admit reviewed permissive dependencies and models/data
    under their own reviewed terms. Exclude unresolved patent concerns; public
    availability and clean-room coding do not establish patent clearance.
  - Source: Initial request, Q025–Q027, Q033, Q166
- R068
  - Current requirement: Agents may prepare branches, commit, push and open
    pull requests in local or cloud environments. They do not push directly
    to main or create release tags. Use a separate GitHub App identity and
    restrict credentials so an agent action cannot masquerade as the owner's
    protected approval.
  - Source: Q193, Q196, Q201
- R069
  - Current requirement: Require full applicable checks before ordinary agent
    commits, then fast push rechecks and current-evidence verification.
    Required GitHub reviews and merge checks follow after pushing. Only
    recognised Dependabot and Release Please proposal commits have the narrow
    pre-commit exception; they still need the complete applicable merge
    checks.
  - Source: Q158, Q202, Q204
- R070
  - Current requirement: Protect main from direct pushes, force pushes and
    deletion, including ordinary owner pushes. Require current branches,
    squash merges, tests and reviews. Trusted ordinary changes may auto-merge
    after checks. Protected rules/specifications need owner approval;
    untrusted contributors need the owner's merge decision and approval before
    running their code on the validation host.
  - Source: Q181, Q193, Q196, Q199
- R071
  - Current requirement: Weekly release preparation may run unattended, but
    cannot bypass incomplete scope or any gate. Candidate tags trigger checks;
    final tags/releases use the same qualified code only after success. Keep
    the release draft while gates are incomplete. A broken main requires an
    immediate revert PR rather than fix-forward as the first response.
  - Source: Q140, Q187, Q193–Q194, Q196
- R072
  - Current requirement: The editing computer is for source and Git
    synchronisation only: no builds, tests, dependency installs, caches,
    installers or backup/evidence stores. Public synthetic-data cloud checks
    are allowed. Private datasets, licensed XenForo and final
    environment-specific measurements stay on the validation host.
  - Source: Q045, Q155, Q197
- R073
  - Current requirement: All project work pauses when the validation host is
    unavailable. Publishing deadlines pause during required-service outages
    and the delay is reported honestly. Preserve checkpointed work and resume;
    do not manufacture success.
  - Source: Q075, Q198, Q203
- R074
  - Current requirement: Use the owner's specifically authorised project
    storage. The later 30 October decision replaces the failed drive with a
    dedicated sutradhar directory on the existing data SSD, excluding the
    operating-system disk and neighbouring project directories. Protect the
    project directory and preserved contents against deletion or destructive
    changes without the owner's explicit instruction. Verify device identity
    and dependencies; the new decision does not authorise formatting. Rebuild
    a lost private corpus from an authorised export and requalify it; do not
    create unapproved local or other-disk backups. Public documentation
    contains no private hardware identifiers.
  - Source: Q045–Q046, Q188, Q197; later storage replacement decision
- R075
  - Current requirement: No public repository material may reveal the owner's
    forum, domain, private data or identifying paths. This includes historical
    quotations, artifacts, CI output, previews and screenshots. Reviewed
    anonymous summary statistics may be public.
  - Source: Q138, Q192, Q197, Q200

## Toolchain and manual

- R076
  - Current requirement: Use a justfile as the
    check/test/lint/typecheck/eval/run/db entry point. Adopt import-linter
    boundaries, pytest-xdist/randomly/timeout, deptry, Ruff security rules,
    gitleaks, conventional commits/Commitizen, Release Please, weekly grouped
    Dependabot and a CI-matching devcontainer. Do not add Fossil, Sapling,
    Buck2, Bazel, Pants or Nix.
  - Source: Q192
- R077
  - Current requirement: Provide JSON Schemas in contracts with generated
    Pydantic models, Schemathesis, Syrupy extraction snapshots, mutation
    checks for hard rules and extraction/scoring benchmarks when the core
    exists. A named tool is not installed or admitted merely because this
    requirement lists it.
  - Source: Q192
- R078
  - Current requirement: Publish a separate beginner manual on the project's
    GitHub Pages site, using MkDocs Material and the requested plugins. Keep
    architecture, internals and module-building material in developer docs.
    Every user page identifies its audience and outcome; generated references
    and synthetic screenshots must remain current. The build, generators and
    screenshot jobs cannot read private corpora, data directories or raw lab
    results; only an explicitly admitted, privacy-reviewed metrics export may
    cross that input boundary.
  - Source: Q192
- R079
  - Current requirement: Preserve the full Q192 manual structure, design,
    writing rules, generation and CI requirements. Public pages describe
    current functionality and show Not yet available where needed; they do
    not present planned features as working. The primary installation path
    follows Q207, and automatic link insertion remains unavailable in v1.
  - Source: Q157, Q192, Q207
- R080
  - Current requirement: Publish dev after main updates and previews for PRs,
    with removal on close. Preserve every full release version; latest points
    only to an approved release. Target a sub-four-minute build and
    publication within five minutes when required services are available. Use
    a weekly manual gardener and per-page audience/review metadata.
  - Source: Q192, Q195, Q203
- R081
  - Current requirement: The first documentation-only checkpoint may mark
    code-only checks not applicable only with proof that no implementation is
    present. Documentation, secrets, rights and consistency checks still
    apply. Adding executable code ends that limited profile.
  - Source: Q190
- R082
  - Current requirement: The first internal implementation milestone is the
    method research catalogue, supported by the necessary approved
    foundations. It is not permission to release a small substitute for the
    full product.
  - Source: Q191
