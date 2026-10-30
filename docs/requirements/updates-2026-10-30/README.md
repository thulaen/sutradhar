# Requirements update: 30 October 2026

Owner: project owner. Recorded by the continuing Claude coordinator.
Status: owner requirement decisions. These approve requirements, not the
implementation designs that still have to be written and approved.

The owner's handover instruction to Claude, and a follow-up message asking for
trunk-based development in the agent rules, change several earlier decisions.
The earlier rules stay in the record; the
[supersession table](supersession.md) links each one to its replacement.
The detailed AutoIssues request is itemised in the
[AutoIssues source register](autoissues-source.md).

## New requirements

### R083: trunk-based development on main only

Use strict trunk-based development. `main` is the only branch. Do not create
feature, fix, temporary or release branches, additional Git worktrees, pull
requests or a second branch named `master`. Bring the foundation branch's
checked work onto `main` through a verified, non-destructive transition and
preserve that branch and its history. Keep unfinished user-visible behaviour
unavailable rather than holding it on a branch.

### R084: protected main-only publisher

A protected publisher alone advances `main`. Before it does, the replacement
must keep: independent code, performance and security reviews; the required
AI review through a qualified main-only process; separate owner-approval and
check-reporting authority; exact candidate identification and fresh checks;
refusal of stale updates and unexpected remote changes; complete audit and
recovery evidence. Explain every GitHub setting or App permission change and
obtain approval of the concrete revision before any `main` update. Never
disable protection temporarily, use the owner's personal login, expose
credentials, force-push, or claim incompatible PR-only and main-only rules
are both satisfied.

### R085: one Git coordinator

With several agents, one coordinator alone moves shared Git state. Agents own
non-overlapping files and never overwrite another agent's work.

### R086: meaningful milestones

Make small, coherent commits that each deliver a reviewable result, and
publish each checked milestone promptly. For each milestone: finish its
behaviour and acceptance cases; run the full pre-commit profile and reviews;
commit exactly the checked content; recheck the candidate and run push
checks; publish through the main-only publisher; verify the remote commit;
update the completion ledger and report in plain English. Do not commit
single sentences, empty scaffolding, repeated status notes, superficial
rearrangements or unfinished fragments. Do not rewrite published history.
If publication is blocked, preserve the work and explain the exact blocker.
The foundation commit remains an identified historical candidate; newer notes
are not silently added to it.

### R087: mandatory test-driven development

New behaviour and bug fixes follow red, green, refactor: write a meaningful
test, observe it fail for the intended behavioural reason, make the smallest
correct change, pass, then refactor with the tests passing. Do not manufacture
failing tests or describe existing code as test-first. Unit tests are
mandatory for executable behaviour; add integration, property, security,
recovery and other tests the boundary needs. Run fast checks (syntax, static
analysis, formatting, affected unit tests) after meaningful edits; they never
replace the commit and push profiles.

### R088: engineering principles and mapped rulebook

Apply DRY, KISS and SOLID where they help, and appropriate NASA/JPL and Meta
engineering practices. Research those practices from primary public sources
and document a PHP/Python mapping of each rule's purpose, enforcement and
applicability. Do not transplant C-specific rules blindly or claim NASA or
Meta certification. Keep the existing coverage, mutation and review
requirements, qualifying each tool per language.

### R089: file, function and line limits

Every first-party text file, including implementation, tests, tooling,
generated source and documentation, has at most 300 lines and 80 characters
per line. Every function, method and closure has at most 60 lines. Count
physical lines, including blank and comment lines, under one documented rule.
Split long documentation into coherent linked documents without losing
requirements, history or provenance. Inventory data files, lockfiles,
third-party material and binary formats and present genuine incompatibilities
for an explicit decision. Do not silently exclude files, alter third-party
code, minify or encode content to evade the limits. Violations are hard
failures in the commit and push profiles.

### R090: 20× performance gate

Every commit and push runs or validates the applicable performance gate.
Changed executable code aims for at least a 20× speed-up against a
trustworthy, fixed baseline for the same behaviour and workload. A pass is
either repeatable measurements showing the speed-up, or a reviewed evidence
report showing that 20× is not reasonably achievable under the declared
workload, platform and correctness constraints. The full evidence rules are
in [SPEC-007](../../specifications/performance-gate.md). Documentation-only
candidates may receive an explicit non-applicability result from the actual
inventory; missing tooling is never non-applicability.

### R091: AutoIssues

Implement the complete attached AutoIssues design through an approved
Sutradhar-specific specification, under `tools/`, excluded from forum-owner
packages. Preserve every substantive item in the
[source register](autoissues-source.md). Verify that integrations support the
chosen PHP and Python versions and admit dependencies before adding them.

### R092: QA integrations and the Dell lab

Test every adapter with controlled fixtures, including fingerprint stability,
deduplication, verification, reopening, leases, migrations, crashes,
concurrency, quota boundaries and flood behaviour. Check what instrumentation
actually exists. Product instrumentation stays off by default. Sequence heavy
QA services on Dell within the approved combined 8 GB lab budget; do not raise
it silently. The production Python service keeps its 2 GB limit and PHP its
8 MB per-request increment.

### R093: commit and push quota

Design Sutradhar's own quota: explicit amounts and escalation; candidate-bound
evidence from one authoritative store; stable cutoffs; no duplicated failure
charges; credit only for genuine, evidenced fixes with recorded lessons; an
availability limit when fewer real issues exist; verified zero debt when
nothing is owed, hidden or deferred; no fabricated issues, manufactured
resolutions or manual resets; infrastructure outages kept distinct from
quality failures; missing or unavailable evidence blocks rather than passes.

### R094: AutoIssues on every commit and push

AutoIssues queue and quota checks run on every commit and push, including in
the protected publisher; local hooks alone are not enough. Inspect the queue
before work. An open P0 blocks unrelated work; work needed to fix it stays
allowed. Handle blocking P1/P2 issues in the affected component under the
approved policy. The design must remove bootstrap and repair deadlocks: a gate
must not need a fix published before it permits that checked fix to be
published. Separate independently checked candidate fixes from final
post-publication verification; an agent never marks its own work verified.
Keep repeated-run verification and reopening. Suppression needs an authorised,
justified, traceable change and cannot avoid mandatory tests, safety rules or
quota debt.

### R095: snapshots without a bot exception

Main-only and milestone rules replace the attachment's fix branches and
nightly snapshot pull requests. Keep durable nightly exports on Dell. Publish
reviewed, privacy-safe snapshots with meaningful milestones through the same
checked main-only route. No unchecked AutoIssues bot commit is allowed.

### R096: separate live and preserved storage

Keep live mutable databases separate from preserved immutable snapshots,
under an explicitly authorised storage design. Never unlock, overwrite or
delete preserved work to make a database or test run succeed.

### R097: completion ledger and reports

Keep a concise, evidence-backed
[completion ledger](../../dev/completion-ledger.md) with five states:
finished and verified; implemented awaiting checks; awaiting design
approval; blocked by an external dependency; not started.
Each milestone report says what works, what was checked, what the evidence
does and does not prove, local and verified GitHub commit identifiers, and
the next milestone.

## Unchanged requirements restated

All checks run in authorised environments; nothing runs on Windows (R072).
The full product scope (R001–R082) is unchanged, including at least 6,000
qualified methods. Later the same day the owner added R098–R108 in the
[models update](models.md), which also amends R021, R050 and R092, and
R109–R111 in the [platform update](platform.md), then R112–R118 in the
[publication update](publication.md), which replaces the GitHub App
publisher with a gated repository on Dell and amends R070, R083, R084
and R086. Signal Controller and
Buganizer are behavioural inspiration only; nothing is copied from their
implementations, databases, private configuration, credentials, approvals
or branding (R067).
