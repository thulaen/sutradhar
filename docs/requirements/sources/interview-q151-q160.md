# Interview source: Q151–Q160

Part of the [Q151–Q180 transcript](interview-151-180.md). Status, provenance and
redaction notes are on that page.

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
