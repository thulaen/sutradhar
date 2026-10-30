# Interview source: Q161–Q171

Part of the [Q151–Q180 transcript](interview-151-180.md). Status, provenance and
redaction notes are on that page.

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
