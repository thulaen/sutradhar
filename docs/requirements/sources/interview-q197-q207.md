# Interview source: Q197–Q207

Part of the [Q181–Q209 transcript](interview-181-209.md). Status, provenance and
redaction notes are on that page.

## Q197

### Question

`````text
197. With cloud agents joining, may GitHub and cloud workspaces run checks using public code and made-up forum data, while private forum data and final performance tests stay on Dell?
`````

### Options offered

`````text
1. Allow public-data cloud checks (Recommended)
Cloud agents and isolated GitHub runners may test synthetic data; licensed XenForo files, private datasets, credentials and Dell services remain protected.
`````

`````text
2. Keep every check on Dell
Cloud agents may propose changes, but testing and manual builds wait for Dell; outside submissions need your approval before Dell runs them.
`````

### User answer

`````text
allow public data cloud checks but don't put my forum data on the repo, and don't mention my forum in the repo, my website shouldn't be known on the repo, all things about it should be anonymized. i don't want to get doxxed. you may ask further questions until all conflicts are resolved and only then you tell me zero conflicts and the question stops, i release plan mode and you start working.
`````

## Q198

### Question

`````text
198. If Dell is offline, may agents continue research, documentation and proposed code changes, leaving any Dell-dependent checks and merges waiting?
`````

### Options offered

`````text
1. Continue proposals (Recommended)
Agents can make useful progress elsewhere; no required Dell result is skipped or treated as passed.
`````

`````text
2. Pause all project work
Keep the earlier rule that development stops completely until Dell is available again.
`````

### User answer

`````text
Pause all project work
`````

## Q199

### Question

`````text
199. Should changes to approved requirements, privacy rules, required checks and release rules still need your explicit approval, even though ordinary implementation changes merge automatically?
`````

### Options offered

`````text
1. Yes, protect those decisions (Recommended)
Agents work unattended within approved specifications; changing the rules or approving a new feature specification still requires you.
`````

`````text
2. Let agents approve those too
Agents may change requirements and governing rules through the same automatic review-and-test process, without a separate decision from you.
`````

### User answer

`````text
Yes, protect those decisions (Recommended)
`````

## Q200

### Question

`````text
200. What may the public repository and manual show about tests run on your real forum data?
`````

### Options offered

`````text
1. Synthetic examples and safe pass/fail (Recommended)
Publish made-up examples and scrubbed check outcomes; keep real counts, excerpts, timing details and dataset descriptions private.
`````

`````text
2. Also anonymous summary statistics
Allow reviewed aggregate results after removing identifying details; still prohibit names, domains, member records and real post text.
`````

### User answer

`````text
Also anonymous summary statistics
`````

## Q201

### Question

`````text
201. To make your approvals distinguishable from agent actions, which separate GitHub identity should automation use?
`````

### Options offered

`````text
1. A dedicated GitHub App (Recommended)
Give automation its own restricted credentials; keep your personal account for approving specifications and changes to protected rules.
`````

`````text
2. A dedicated bot account
Use a separate GitHub account for automation, with its own login and permissions; you retain your personal account for protected decisions.
`````

### User answer

`````text
A dedicated GitHub App (Recommended)
`````

## Q202

### Question

`````text
202. Which checks must finish before agents save and push a change to a working branch, before it is eligible to enter main?
`````

### Options offered

`````text
1. Full checks before each commit
Retain your earlier rule: all applicable pre-commit checks finish first; GitHub's additional AI review and merge checks follow after the branch is pushed.
`````

`````text
2. Fast branch checks; full merge checks (Recommended)
Require fast tests, formatting, types and secret checks before branch commits and pushes; require the complete applicable suite and AI review before merging into main.
`````

### User answer

`````text
Full checks before each commit
`````

## Q203

### Question

`````text
203. Should the five-minute manual publishing target pause when Dell or a required hosting service is unavailable?
`````

### Options offered

`````text
1. Yes, pause and report it (Recommended)
Keep your all-work-pauses rule; show the delay honestly and resume the queued update when the required services return.
`````

`````text
2. Let manual publishing continue
Create an exception allowing cloud manual builds and publication while Dell is offline; other project work remains paused.
`````

### User answer

`````text
Yes, pause and report it (Recommended)
`````

## Q204

### Question

`````text
204. May Dependabot and Release Please create unchecked proposal commits, provided those commits cannot enter main until every required check and review passes?
`````

### Options offered

`````text
1. Allow this narrow exception (Recommended)
Only recognised proposal bots get the exception; ordinary agent commits still require full pre-commit checks, and bot changes receive the full checks before merging.
`````

`````text
2. No unchecked bot commits
Replace automatic bot-written branches with a process that prepares patches, runs the checks first, and only then creates commits; this changes the requested standard bot workflow.
`````

### User answer

`````text
Allow this narrow exception (Recommended)
`````

## Q205

### Question

`````text
205. You requested both a fixed benchmark and fresh release examples; how should the 500-suggestion judging requirement apply?
`````

### Options offered

`````text
1. At least 500 in each (Recommended)
Judge at least 500 fixed and 500 fresh suggestions per release candidate; require both groups to pass, with more time and subscription usage.
`````

`````text
2. 500 total, split equally
Judge 250 fixed and 250 fresh suggestions per candidate; require each group to pass, replacing the earlier 500-example fixed judging requirement.
`````

`````text
3. 500 fresh; fixed tests without AI
Run AI judging on 500 fresh suggestions; retain the fixed benchmark for repeatable non-AI checks, replacing the earlier requirement to judge both sets.
`````

### User answer

`````text
At least 500 in each (Recommended)
`````

## Q206

### Question

`````text
206. For the rule that more than 2% invalid judge answers blocks release, should an answer still count as initially invalid when the one allowed repair fixes it?
`````

### Options offered

`````text
1. Yes, count the initial failure (Recommended)
Measure reliability using first attempts from each judge in each judging round; record repair success separately so repairs cannot hide unreliable output.
`````

`````text
2. Count only failures after repair
Treat repaired answers as valid for reliability; keep the initial error rate visible, but block only when unrepaired answers exceed 2%.
`````

### User answer

`````text
Yes, count the initial failure (Recommended)
`````

## Q207

### Question

`````text
207. Which installation path should the product and its beginner manual promise?
`````

### Options offered

`````text
1. Point-and-click setup (Recommended)
Keep guided setup as the main path, with an agent or host administrator handling necessary server preparation; commands remain optional advanced reference.
`````

`````text
2. One-time setup commands
Allow a short copy-and-paste command sequence for installation, then provide point-and-click controls for everyday use.
`````

### User answer

`````text
Point-and-click setup (Recommended)
`````
