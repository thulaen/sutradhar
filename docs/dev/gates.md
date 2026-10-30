# Required checks and blocking rules

Audience: agents and reviewers. Outcome: determine which evidence is required at
each development stage and refuse unsupported passes.

Status: enforcement contract. Check names below are reserved names, not proof that
the checks are installed. Implementation requires an approved specification.

## Stages

| Stage | Requirement |
| --- | --- |
| Ordinary agent commit | Full applicable pre-commit profile, including engineering reviews |
| Push | Recheck unchanged candidate evidence and rerun the fast suite |
| Trusted proposal-bot commit | Only Dependabot and Release Please may defer project checks |
| Merge | Complete applicable suite, current-change AI review, up-to-date base and authority checks |
| Nightly | Full private corpus deterministic checks; no AI judges by default |
| Release candidate | Full scope acceptance, release judging, 72-hour soak and release-only checks |
| Publication | All results valid for the exact qualified candidate; no unresolved required condition |

The [documentation bootstrap](bootstrap.md) defines the only initial docs-only
profile. A release-only check is not pretended to have run before each commit.
Likewise, the extra GitHub AI review naturally follows the branch push; earlier
engineering reviews still happen before the ordinary agent commit.

The push fast suite includes unit, property and approximately 300-thread synthetic
fixture tests, with a qualified target under five minutes. The private development
evaluation uses a stratified 2,000-thread slice, with a qualified target under ten
minutes; it measures regression, not absolute quality. The 50,000-thread nightly
dataset runs deterministic hard-rule, reconstruction-recall, anchor-agreement and
coverage checks. Real corpora never enter the public repository or cloud runner.

## Required GitHub check names

| Exact context | Required evidence |
| --- | --- |
| `just check` | Complete applicable profile manifest and valid results for the candidate |
| `tests` | Required behavioural, coverage, property, mutation and regression checks |
| `edge-case suite` | Applicable documented edge cases, with all critical release cases covered |
| `import-linter` | Approved Python module-boundary contracts |
| `security scans` | Security review, secrets, dependency and relevant static/dynamic checks |
| `manual checks` | Applicable manual correctness, privacy, accessibility and change-coverage checks |
| `AI code review` | Completed review for the current proposal; findings handled under the approved policy |
| `owner approval` | Required owner decisions, or explicit non-applicability for ordinary approved-scope work |
| `Dell availability` | Live availability and authority check, refreshed before the protected action |

Bind expected check sources to the trusted verifier identity where GitHub supports
that restriction. A candidate must not define its own successful evidence by
changing a workflow, posting a similarly named status or forging a review comment.
The protected verifier must use trusted policy and identify the exact current
candidate and base. A queued, cancelled, timed-out, skipped, stale or unavailable
required check blocks its stage.

Do not set broad path filters that leave a required check permanently absent.
Produce a truthful explicit applicability result from a trusted inventory instead.
Do not treat GitHub's presentation of a skipped job as proof that a required scan
actually ran. [GitHub ruleset reference](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/available-rules-for-rulesets)

## Engineering checks

First-party code requires 100% line, function, branch, decision and modified
condition/decision coverage. Modified condition/decision coverage means proving
that each Boolean condition can independently change a decision's outcome. Tool
support must be demonstrated for each relevant language and compiled/native
boundary; a line-coverage percentage is not a substitute. Third-party source is
not silently included in a first-party percentage or mislabelled as covered.

The applicable profile includes unit and behaviour tests, property tests,
regressions, mutation testing, specification conformance, documentation consistency,
dependency/rights admission, security analysis, measured performance, memory,
stress, races, database interactions, chaos and fault injection, maintainability
and code-smell checks. Qualify equivalent mutant handling by rewriting the code
or reviewed mutation rules; do not waive equivalent mutants or weaken rules to
inflate the score.

All first-party PHP and Python files obey the approved 80-column and 300-lines-of-
code limits. Generated code remains first-party generated output and must be
covered by an explicit, approved policy; generation is not a blanket exemption.
Documentation has no arbitrary length cap, but explanations must not be duplicated.

Required tests must prove boundaries, not mirror the implementation. Use the
synthetic edge-case fixture, generated adversarial cases and relevant integration
tests. A critical leak/corruption/data-loss case cannot be deferred past release
because its triggering path is inconvenient to exercise.

## Performance, memory and recovery

Use repeated controlled comparisons and report variability. A new known real
slowdown fails the relevant regression gate; noise is not automatically a failure
or a reason to dismiss a repeatable regression. Record comparable workloads,
hardware, runtime, configuration and dependencies.

Measure no more than 8 MB additional PHP peak memory per request against a matched
enabled/disabled comparison. The installed Python service and all its workers,
including extension workers, share the 2 GB operating-system budget. Libraries'
native allocations, child processes and relevant operating-system accounting must
be included. Work must shrink, wait or refuse safely before the hard cap; separately
test unexpected worker termination and recovery. The Dell laboratory has its own
8 GB combined limit, including its test services and judge orchestration.

Test bounded chunks, durable restart, duplicate and out-of-order events, deletion,
permission changes, lease replacement, partial writes, full storage and interrupted
network access. Under the qualified workload, worker recovery repeats no more than
one minute of unfinished computation. Completed committed work is not disposable.
The first useful suggestion target is 30 seconds under a declared qualified
workload; it does not permit approval before all methods applicable to available
evidence finish.

The release soak is 72 uninterrupted hours on the supervised Dell host, with
durable progress and logs. The editing machine and chat may be closed. A deliberate
interruption test is separate recovery evidence, not a substitute for uninterrupted
soak time. An infrastructure outage pauses development and invalidates the
uninterrupted run; preserve evidence before beginning its replacement.

## Reviews

Record code, performance and security reviews with findings, fixes, source
identity, evidence and review limitations. A self-review is allowed but must be
labelled as such. Review-quality checking must evaluate whether the reasoning and
coverage support the verdict. Counting review files is not a review-quality test.

The additional Codex GitHub review requires integration qualification. Official
documentation describes GitHub reviews, not a universally available status-check
name. Verify actual completion for the current content and report it through the
protected `AI code review` check. An eyes reaction, silence, an obsolete review or
an unrelated comment cannot pass it. [Codex GitHub review documentation](https://learn.chatgpt.com/docs/third-party/github)

Claude Code review is an optional second reviewer with its own qualified adapter.
Do not confuse an engineering reviewer with the two-provider suggestion-quality
judges described below. A provider quota problem leaves the required result
pending; it does not authorise a fabricated pass or paid fallback.

## Release suggestion assessment

Run judging on the exact release candidate, not every nightly run. Use at least
500 fixed and 500 fresh valid suggestions, with the two sets assessed separately.
Preserve held-out data and predefined sampling. Both sets and important declared
groups must meet the approved quality and coverage conditions. The lower end of
the 95% confidence interval must be at least 95%; observed 95% alone is insufficient.
The opportunity-coverage target is separately 95%, and hard rules must pass 100%.
Document the interval and sampling assumptions, including repeated or related
threads, rather than pretending every correlated example is independent.

Two approved providers judge blind first. Disagreements receive a second round
with the opposing verdict and explanation shown anonymously. In round three, a
failure must cite a rubric criterion and a passing judgement must answer each
objection. Unresolved cases count as failures in the all-suggestions score and
remain visibly contested for staff. Later staff decisions inform future learning;
they do not rewrite a completed assessment.

Allow one repair of an invalid judge answer. Calculate each judge's initial-invalid
rate per judging round before repair; more than 2% blocks release. Record repair
success separately. Count unresolved and invalid failures in the overall
denominator; never discard them to improve the result. First-round agreement below
95% blocks release. A proposed rubric correction needs approval and a fresh
assessment; changed rubrics invalidate old verdicts. Track flip rates and warnings
for a judge that always defers.

Version cache keys by content, source/target excerpts, model, rubric, prompt,
schema, redaction and judging round, including the opposing reasoning when used.
Unchanged valid results may be reused only within their qualified scope. A bounded
drift check runs where judging is enabled. Jobs checkpoint and resume through
usage windows. An optional 50-suggestion post-merge warning run has a configurable
budget, which may be zero; it is not the release assessment.

## Complete release

No count of catalogue rows substitutes for 6,000 distinct, correct, useful and
qualified methods. All requested release features, approved library admissions,
required playbook situations, critical edge cases, operational and UI acceptance,
WCAG 2.2 AA evidence, actual browser qualification, installation, upgrade, rollback,
privacy and packaging checks must be complete. Keep an item-by-item evidence
ledger. Missing or indirect evidence means the full release is not achieved.
