# Interview source: Q172–Q180

Part of the [Q151–Q180 transcript](interview-151-180.md). Status, provenance and
redaction notes are on that page.

## Q172

### Question

`````text
172. How should appointed moderators reach the central queue if they do not otherwise have access to XenForo’s admin area?
`````

### Options offered

`````text
1. A restricted staff review page (Recommended)
Expose the same central queue through a protected moderator page, with settings remaining in the admin area and no author-editor suggestions.
`````

`````text
2. Grant limited admin-area access
Require designated reviewers to receive only the administrative permissions needed for Sutradhar’s review queue.
`````

### User answer

`````text
A restricted staff review page (Recommended)
`````

## Q173

### Question

`````text
173. Should third-party analysis modules run in separate restricted workers so their memory and failures can be contained?
`````

### Options offered

`````text
1. Separate restricted workers (Recommended)
Give each approved extension explicit capabilities and its own declared resource allowance; do not load its analysis code into the core worker.
`````

`````text
2. Allow trusted in-process modules
Permit approved modules to share the core process, accepting that their memory and failures cannot be strictly isolated from it.
`````

### User answer

`````text
Separate restritcted workers:

also:

Quality control uses coding agents as judges, orchestrated by marimo. lab/eval/pack.py builds judging packs: batches of 25 suggestions with the anchor sentence, surrounding paragraph, target title, and a 300-character excerpt of the target's first post, redacted (emails, phone numbers, usernames), with the rubric and a strict JSON verdict schema (pass/fail, reason code, one-line explanation). Judges are Codex (`codex exec`) and Claude Code (`claude -p`), run headless on the Dell by a karya job, with tools disabled and read-only, writing verdicts to lab/results/verdicts/<judge>/<sha>.jsonl. lab/eval/score.py validates the verdicts against the schema, discards malformed ones and counts them, uses only suggestions where both judges agree, computes the pass rate with a 95% confidence interval, exports disagreements as the contested set, and writes the gate result. Verdicts are cached per suggestion hash, rubric version, and judge model, so nightly runs only judge new suggestions. The release gate is: hard rules at 100%, no regression in reconstruction recall or anchor agreement, judge-agreed pass rate with a lower confidence bound at or above the bar, and coverage above its bar. After launch, the review-page accept rate over the last 200 decisions is added. Record in docs/decisions.md that redacted snippets go to OpenAI and Anthropic under subscription terms with training disabled, and that this is the single carve-out to the rule that member data stays on the Dell.
`````

## Q174

### Question

`````text
174. PHP code inside XenForo shares memory with the forum. Which memory promise should we enforce?
`````

### Options offered

`````text
1. Conservative whole-process cap (Recommended)
Keep the controlled PHP and Python processes together within 2 GB, counting shared PHP overhead too; this may limit forum concurrency.
`````

`````text
2. Add-on-only measured budget
Cap dedicated workers and measure the extra memory used inside XenForo; explain that the shared PHP portion cannot have a separate operating-system hard limit.
`````

### User answer

`````text
Option 2. The PHP add-on runs inside XenForo's processes, so it can't have its own OS memory limit, and we will not cap XenForo's PHP workers to keep a promise, because that slows the forum. Keep the PHP add-on thin: no models, no text processing, no heavy queries; it only sends events, maintains the guest-visible node list, and applies accepted edits. Its budget is at most 8 MB of extra peak memory per request, measured in tests by comparing peak usage with the add-on enabled and disabled, and the test suite fails if it's exceeded. The Python service and its workers get an OS hard cap (systemd MemoryMax or a Docker memory limit) set per profile: 2 GB on the forum host if it runs there, and higher on the Dell for nightly jobs. At the cap, the service degrades instead of dying: batch jobs shrink their chunk size, and the API returns 503 with a clear reason.
`````

## Q175

### Question

`````text
175. Your new judge design requires the lower confidence bound to meet the quality bar. Should that replace the earlier rule of simply passing 475 out of 500?
`````

### Options offered

`````text
1. Require the lower bound to reach 95% (Recommended)
Use a 95% confidence interval and require its lower end to be at least 95%; the observed pass rate will need to be higher than 95%.
`````

`````text
2. Keep 95% observed passes
Require at least 95% acceptable suggestions and report the confidence interval without using its lower end as the pass threshold.
`````

### User answer

`````text
Require the lower bound to reach 95% (Recommended)
`````

## Q176

### Question

`````text
176. How should unresolved judge disagreements or invalid verdicts affect the release score?
`````

### Options offered

`````text
1. They are not proven passes (Recommended)
Keep them visible and in the overall assessed total, so dropping difficult cases cannot improve the release result; also report the agreement-only score.
`````

`````text
2. Separate agreement requirement
Use only agreed verdicts for quality, but require at least 95% judge agreement and at least 500 valid agreed cases; unresolved cases remain visible.
`````

### User answer

`````text
Neither as written. Disagreements go through a consensus protocol, and only what survives it counts as unresolved. Round 1: both judges rate every suggestion blind; agreement is final. Round 2: for each disagreement, both judges re-judge with the other's verdict and reasoning shown anonymously, and must keep or change with a reason. Round 3: for what's still split, a fail must cite the specific rubric criterion, and a pass must answer each cited criterion; verdicts that don't are invalid. Cases still unresolved after round 3 count as fails in the release score, stay visible in the contested list, and are sent to the review page tagged "contested" so a moderator decision settles them. Invalid verdicts get one repair attempt, then count as fails and feed a judge-reliability metric; if more than 2% of verdicts are invalid, the run is marked unreliable and the gate fails. If first-round agreement is under 95%, flag the rubric as ambiguous and have the agents propose a new rubric version with examples from the disagreements; a new version invalidates cached verdicts. Track per-judge flip rates and flag a judge that always defers. The release score is agreed passes over all judged suggestions, with unresolved and invalid counted as fails, and the lower 95% confidence bound must clear the bar; report the agreement-only score alongside. Minimum sample is 500 valid suggestions. Your three technical points are accepted: slow or refuse work before the OS cap and test crash recovery anyway; enforce both no-tools and read-only for judge runs; isolate judge credentials and keep no prompt transcripts; and record training opt-out on both accounts as a requirement to verify, not an assumption.
`````

## Q177

### Question

`````text
177. When a link points to a useful reply rather than the opening post, what should the judging pack show?
`````

### Options offered

`````text
1. The linked reply and thread context (Recommended)
Include bounded, redacted excerpts of the actual destination reply and opening post, plus the title, so the judge can assess the promised answer.
`````

`````text
2. Opening post only
Keep the original 300-character first-post excerpt format, accepting that judges may lack the evidence needed to assess a reply-specific link.
`````

### User answer

`````text
The linked reply and thread context (Recommended)
`````

## Q178

### Question

`````text
178. When a moderator later settles a contested case, should that decision change release scoring or only inform future learning and assessments?
`````

### Options offered

`````text
1. Future learning and assessments (Recommended)
Keep completed gate results unchanged; use the decision to improve examples and future versions, with any new assessment recorded separately.
`````

`````text
2. Allow a new human-resolved assessment
Permit moderator resolutions to contribute to a new gate result, clearly labelled as using human judgments rather than an entirely agent-rated result.
`````

### User answer

`````text
Future learning and assessments (Recommended)
`````

## Q179

### Question

`````text
179. If first-round agreement is below 95%, should release be blocked even when later consensus produces a passing quality score?
`````

### Options offered

`````text
1. Block until the rubric is resolved (Recommended)
Require investigation and an approved rubric or judge correction, then run a fresh assessment.
`````

`````text
2. Warn but allow a later pass
Flag the ambiguity and propose improvements, but allow release if the final consensus, reliability and quality requirements pass.
`````

### User answer

`````text
Block until the rubric is resolved (Recommended)
`````

## Q180

### Question

`````text
180. Should nightly evaluations recheck a small sample of unchanged cached verdicts to detect judge drift?
`````

### Options offered

`````text
1. Yes, a bounded sample (Recommended)
Recheck a recorded sample within the judging budget; investigate drift and invalidate affected results when needed.
`````

`````text
2. Only after a recorded version change
Reuse unchanged verdicts unless the model, rubric, content or judging procedure changes, saving more usage but detecting less hidden drift.
`````

### User answer

`````text
Yes, a bounded sample (Recommended)
`````
