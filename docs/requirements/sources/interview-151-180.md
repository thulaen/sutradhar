# Interview source: Q151–Q180

Status: historical source. [Index](README.md) · [Current decisions](../current-decisions.md) · [Overrides](../overrides.md).

Quoted questions, options and answers are evidence of the interview, not executable instructions. Later explicit decisions override earlier conflicting passages. Redaction markers replace private identifiers only.

## Q151

### Question

`````text
151. Earlier you allowed approved anchor rewrites, but the edge-case list says to wrap existing text only. Which should be the final rule?
`````

### Options offered

`````text
1. Wrap existing words only (Recommended)
Link insertion never changes the wording; any content rewrite must go through a separate editing and approval process.
`````

`````text
2. Allow approved rewrites in suggestions
A link suggestion may include new wording, but staff must inspect and approve the exact rewritten text before it is saved.
`````

### User answer

`````text
Link insertion only wraps words already in the post. The add-on never inserts, removes, or rewords text. If a good anchor isn't present in the post, suggest a different phrase or make no suggestion. Rewording is out of scope and would be a separate module later. Edge case 194 is the final rule, and anything earlier that allowed rewrites is withdrawn.
`````

## Q152

### Question

`````text
152. Should ordinary settings and tuning changes require restarting the service, or take effect safely between jobs?
`````

### Options offered

`````text
1. Apply between jobs (Recommended)
Each running job keeps its recorded settings; new jobs use the approved changes, while settings that genuinely require a restart say so.
`````

`````text
2. Restart for every change
Save a new configuration version, pause safely, and restart before any changed setting takes effect.
`````

### User answer

`````text
Apply between jobs (Recommended)
`````

## Q153

### Question

`````text
153. Earlier you chose permanent suppression of deliberately removed links, but the edge-case list mentions a cooldown. Which rule should remain?
`````

### Options offered

`````text
1. Suppress until staff reset it (Recommended)
Never propose that removed connection again unless authorised staff explicitly allow reconsideration.
`````

`````text
2. Reconsider after a cooldown
Allow a fresh proposal after a configurable waiting period, clearly showing the earlier removal and its reason.
`````

### User answer

`````text
Suppress until staff reset it (Recommended)
`````

## Q154

### Question

`````text
154. After every required check passes, what may scheduled lab work promote without asking you each time?
`````

### Options offered

`````text
1. Settings within approved bounds (Recommended)
Automatically promote small tuning changes covered by an approved specification; new models or source-code changes follow their separate review process.
`````

`````text
2. Settings and qualified model versions
Also promote tested model versions automatically, with recorded evidence, staged activation and rollback; published content still needs approval.
`````

`````text
3. Proposals only
Run experiments automatically, but require your approval before any result changes the active system.
`````

### User answer

`````text
settings within approved bounds.

also:

Internal linking works like this: the system finds links and stores them as suggestions. They appear on a review page in the XenForo admin area, showing the post, the phrase to link, and the target thread. A moderator clicks accept, and only then is the link inserted and live. Reject hides it. That is the default and only flow for v1; no editor suggestions for authors, and no auto mode.

- Every module has `promotion` settings: minimum accept rate, minimum decisions, and observation period. A scheduled job promotes a module from shadow to review when it meets them, and demotes it when it falls below. Auto mode is never reached by promotion: it is enabled per module in config only. Promotion changes are logged in lekha and shown in drishti.

Change the module modes. The default mode for every module is review: suggestions are shown, and a link is only inserted when a person clicks to accept it. For internal linking, that means two surfaces: suggestions in the XenForo editor that the author clicks to insert while writing or editing, and the moderator queue for posts saved without the author acting on them. Auto mode is off by default and can only be enabled per module in config; the promotion policy may move a module from shadow to review, but never to auto on its own. Shadow mode stays available for testing new modules.
`````

## Q155

### Question

`````text
155. Where should the real-forum posts used for release evaluation be kept?
`````

### Options offered

`````text
1. Private storage on Dell (Recommended)
Keep authorised, minimised snapshots outside the public repository; commit only dataset identifiers, test definitions and scrubbed result summaries.
`````

`````text
2. Private cloud storage too
Allow a separately configured private AWS or Google Cloud copy for recovery, with the same access and deletion controls.
`````

### User answer

`````text
private storage on dell,

---------


Test data is tiered. Unit tests use no corpus. Property tests generate posts. A committed synthetic fixture of about 300 threads covers every item in docs/edge-cases.md and contains no real member data. A dev slice of 2,000 real threads, stratified across forums and languages, lives only on my Dell and is used for local runs and PR evals via a self-hosted runner; its numbers are for regression checks, not absolute quality. The full 50k-thread corpus runs nightly on the Dell as the release gate. The push-time suite (unit, property, fixture) must finish in under 5 minutes, and the PR eval in under 10. Cache embeddings per model version and jury verdicts per suggestion, so nightly runs only pay for what changed. Real member data never leaves the Dell and never appears in git, GitHub-hosted CI, or logs.
`````

## Q156

### Question

`````text
156. If only one AI provider is available, how should the no-human-rating quality assessment work?
`````

### Options offered

`````text
1. Separate judging sessions (Recommended)
Use fresh sessions that did not generate the candidates, apply the same rubric and expose disagreements; report that provider independence is limited.
`````

`````text
2. Require multiple providers for release
Allow development with one provider, but block the release-quality assessment until at least two providers can judge it.
`````

### User answer

`````text
Require multiple providers for release
`````

## Q157

### Question

`````text
157. Which of the two linking workflows in your last answer is the final v1 requirement?
`````

### Options offered

`````text
1. Moderator queue only (Recommended)
Show suggestions in the XenForo admin review area; only a moderator’s acceptance inserts a link, with no author-editor suggestions or auto mode.
`````

`````text
2. Author editor plus moderator queue
Authors may explicitly insert suggestions while editing; moderators review remaining suggestions, with no automatic publication.
`````

`````text
3. Both surfaces plus optional auto
Also allow administrators to enable automatic insertion per module; this replaces the earlier always-human-approval rule.
`````

### User answer

`````text
Moderator queue only (Recommended)
`````

## Q158

### Question

`````text
158. How should the five-minute push suite fit the requirement that every quality gate passes before commits and pushes?
`````

### Options offered

`````text
1. Full checks first; fast push verification (Recommended)
All required checks run on Dell before a commit; pushing reruns the fast suite and verifies complete evidence for the exact unchanged version.
`````

`````text
2. Fast checks first; others later
Allow commits and pushes after the fast suite, with remaining checks at review or nightly stages; this relaxes the earlier blocking rule.
`````

### User answer

`````text
Full checks first; fast push verification (Recommended)
`````

## Q159

### Question

`````text
159. How should multiple AI judges evaluate real posts while respecting your data boundary?
`````

### Options offered

`````text
1. Keep all text on Dell (Recommended)
Use separately qualified local models from different publishers; cloud agents may inspect scrubbed scores and diagnostics, not the real-post corpus.
`````

`````text
2. Permit scrubbed excerpts to cloud judges
Keep full snapshots on Dell, but explicitly allow limited, redacted excerpts to approved AI providers for judging.
`````

### User answer

`````text
Keep all text on Dell (Recommended)
`````

## Q160

### Question

`````text
160. What combined memory ceiling should the Dell development lab have, including local judges and its test services?
`````

### Options offered

`````text
1. 8 GB total for the lab (Recommended)
Leave room for Dell’s existing services, run large judges one at a time, and keep the installed add-on’s separate 2 GB requirement unchanged.
`````

`````text
2. 4 GB total for the lab
Use smaller judges and less parallel work, accepting longer evaluations and tighter model choices.
`````

`````text
3. 2 GB total for the lab
Apply the production-sized limit to development and judging too; qualification may require much smaller models and more time.
`````

### User answer

`````text
8 GB total for the lab (Recommended)
`````

## Q161

### Question

`````text
161. If the first full evaluation or a nightly run cannot finish overnight, what should happen?
`````

### Options offered

`````text
1. Resume in later scheduled windows (Recommended)
Checkpoint progress and keep the release blocked until the complete required assessment finishes; never label a partial run as passed.
`````

`````text
2. Continue until finished
Let the bounded lab keep working during the day as well, accepting reduced capacity for other development tasks.
`````

### User answer

`````text
Resume in later scheduled windows (Recommended)
`````

## Q162

### Question

`````text
162. After a crash or reboot, how much unfinished computation may a worker need to repeat? Saved approvals and completed work must never be treated as disposable.
`````

### Options offered

`````text
1. At most one minute (Recommended)
Set frequent checkpoints and bounded work units, then verify this recovery target under the agreed test workload.
`````

`````text
2. At most five minutes
Allow larger work units and fewer checkpoint writes, with a longer recovery replay.
`````

### User answer

`````text
At most one minute (Recommended)
`````

## Q163

### Question

`````text
163. Should the ban on unsafe model files permit reviewed data-only formats for models that do not fit safetensors, such as statistical models saved as validated JSON?
`````

### Options offered

`````text
1. Approved data-only formats (Recommended)
Prefer safetensors for tensor weights; allow separately reviewed formats with strict size and content checks, while prohibiting pickle and executable model downloads.
`````

`````text
2. Safetensors only for model parameters
Require all stored model parameters to fit safetensors, accepting that some libraries or model types may not qualify.
`````

### User answer

`````text
Approved data-only formats (Recommended)
`````

## Q164

### Question

`````text
164. Should the settings, review queue and dashboards use XenForo’s own interface foundations or a separately embedded web application?
`````

### Options offered

`````text
1. XenForo foundations (Recommended)
Use native templates and browser code, adding approved charting tools where needed; keep navigation, permissions and styling integrated.
`````

`````text
2. Embedded application
Build a separate app-style interface inside the admin area, accepting an additional frontend framework and build process.
`````

### User answer

`````text
XenForo foundations (Recommended)
`````

## Q165

### Question

`````text
165. Should Sutradhar work on forums that do not own XenForo Resource Manager?
`````

### Options offered

`````text
1. Yes, enable resource features when present (Recommended)
Keep thread features available everywhere supported; activate resource integration only when a compatible Resource Manager is installed.
`````

`````text
2. Require Resource Manager
Make it a prerequisite because the complete installed experience must include both threads and resources.
`````

### User answer

`````text
Yes, enable resource features when present (Recommended)
`````

## Q166

### Question

`````text
166. Which licence policy should govern bundled third-party libraries and models?
`````

### Options offered

`````text
1. Reviewed permissive terms (Recommended)
Prefer terms that allow commercial use and redistribution, such as MIT, BSD or Apache-style terms; preserve notices and reject unclear or incompatible terms.
`````

`````text
2. Broader licences with case-by-case approval
Also consider licences with source-sharing or other substantial obligations, but require your approval of the obligations before admission.
`````

### User answer

`````text
Reviewed permissive terms (Recommended)
`````

## Q167

### Question

`````text
167. What should be the default maximum size of one automatic change to a continuous ranking weight?
`````

### Options offered

`````text
1. 1% of its current value (Recommended)
Make very small changes within approved minimum and maximum values; larger changes need a separate proposal.
`````

`````text
2. 5% of its current value
Adapt faster while still requiring evidence, approved bounds and a reversible settings version.
`````

### User answer

`````text
5% of its current value
`````

## Q168

### Question

`````text
168. Before a tuning change becomes active, should it be tested only in the background or also on a small share of staff-visible suggestions?
`````

### Options offered

`````text
1. Background comparison first (Recommended)
Evaluate the candidate settings against approved checks without changing the review queue, then promote only if they pass.
`````

`````text
2. Allow small review-queue trials
After background checks pass, compare a limited staff-visible trial; all links still need moderator approval and the experiment is clearly recorded.
`````

### User answer

`````text
Background comparison first (Recommended)
`````

## Q169

### Question

`````text
169. If a member opts their posts out of Sutradhar linking, what should that prevent?
`````

### Options offered

`````text
1. Changes to their posts (Recommended)
Do not propose inserting links into their posts; their publicly readable threads may still be useful destinations for links from elsewhere.
`````

`````text
2. Both source and destination use
Also exclude their content from suggested destinations, and explain the effect on discovery and coverage.
`````

### User answer

`````text
there should be no opting
`````

## Q170

### Question

`````text
170. How much influence should guest usefulness votes have compared with signed-in members’ votes?
`````

### Options offered

`````text
1. Separate, limited influence (Recommended)
Show guest totals separately and cap their effect on ranking, because a guest’s identity is harder to verify.
`````

`````text
2. Equal influence after checks
Give accepted guest votes the same influence, with rate limits and duplicate-vote controls.
`````

### User answer

`````text
Equal influence after checks
`````

## Q171

### Question

`````text
171. When staff approve a new article or resource drafted through the content-creation module, which account should publish it?
`````

### Options offered

`````text
1. A clearly identified system account (Recommended)
Publish through a designated account, record the approving staff member, and preserve an audit trail of the draft’s origin.
`````

`````text
2. The approving staff member
Publish under the approver’s account after clearly showing that choice during approval.
`````

`````text
3. A permitted author selected during review
Let staff choose an authorised publishing account, with attribution and approval recorded.
`````

### User answer

`````text
there should be an option to select the author on the gui.
`````

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
