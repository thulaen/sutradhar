# Interview source: Q181–Q190

Part of the [Q181–Q209 transcript](interview-181-209.md). Status, provenance and
redaction notes are on that page.

## Q181

### Question

`````text
181. Before code from an outside contributor may run on Dell, should a maintainer approve the exact version being tested?
`````

### Options offered

`````text
1. Yes, approval before Dell execution (Recommended)
Do not automatically run public submissions on your machine; isolate approved tests from private data, judge credentials and other projects.
`````

`````text
2. Automatic synthetic-only isolation
Automatically test outside submissions only in an isolated environment with synthetic data, no credentials and no access to private evaluations.
`````

### User answer

`````text
Yes, approval before Dell execution (Recommended)
`````

## Q182

### Question

`````text
182. How long should an unreviewed suggestion remain in the queue before it expires and needs fresh analysis?
`````

### Options offered

`````text
1. 30 days (Recommended)
Give staff time to review while limiting stale backlog; edits or visibility changes can invalidate it sooner.
`````

`````text
2. 7 days
Keep the queue fresher, at the cost of more repeated analysis.
`````

`````text
3. Until the content changes
Retain suggestions without an age limit, but recheck their current validity before approval.
`````

### User answer

`````text
30 days (Recommended)
`````

## Q183

### Question

`````text
183. After a moderator rejects a proposed source-to-destination connection, when may it be suggested again?
`````

### Options offered

`````text
1. Only after meaningful content changes (Recommended)
Suppress repeated proposals for unchanged content; a materially changed source or destination may justify a clearly explained new suggestion.
`````

`````text
2. Only after staff reset it
Treat rejection like deliberate removal and suppress that connection indefinitely until an explicit reset.
`````

### User answer

`````text
only after meaningful content changes, also:

Change when judges run. The full judge run (500 valid suggestions, two judges, consensus rounds) is a release-time step: it runs when a release candidate is tagged, on that exact commit, as a resumable job that survives usage windows and reports when complete. It reuses cached verdicts for any suggestion unchanged since the last judged run. The nightly job no longer runs judges; it runs only the free checks (hard rules, reconstruction recall, anchor agreement, coverage, report). Optionally, a merge to main triggers an early-warning judge run on a sample of 50 suggestions, with a configurable budget that can be set to zero. Everything else in the judge design (consensus protocol, pending status, usage-limit handling, caching) stays as specified.
`````

## Q184

### Question

`````text
184. Which accessibility target should block release of Sutradhar’s own screens?
`````

### Options offered

`````text
1. WCAG 2.2 AA (Recommended)
Require the recognised AA criteria, including keyboard use, readable contrast, accessible forms and alternatives to visual-only charts.
`````

`````text
2. WCAG 2.2 AAA where applicable
Pursue stricter applicable criteria as well, with more demanding design and verification work and explicit reporting of any unmet requirements.
`````

### User answer

`````text
WCAG 2.2 AA (Recommended)
`````

## Q185

### Question

`````text
185. Which browser support policy should the first release follow?
`````

### Options offered

`````text
1. Current mainstream browsers (Recommended)
Qualify recent Chrome, Edge, Firefox and Safari releases, including mobile layouts; publish exactly which environments were actually tested.
`````

`````text
2. Include older supported browsers
Also qualify a wider browser range aligned with XenForo’s supported audience, accepting more compatibility work and fewer modern interface features.
`````

### User answer

`````text
Current mainstream browsers (Recommended)
`````

## Q186

### Question

`````text
186. How long should the uninterrupted release soak test run under a representative changing workload?
`````

### Options offered

`````text
1. 72 hours (Recommended)
Exercise several daily cycles, queue growth, cleanup and memory stability, with controlled recovery checks recorded separately.
`````

`````text
2. 24 hours
Cover one full daily cycle with a shorter release delay.
`````

`````text
3. Seven days
Cover a full weekly cycle, accepting a substantially longer wait for each qualifying release.
`````

### User answer

`````text
72 hours but there should be logging just in case MSI windows switches off, but dell runs 24/7 because it's a server machine. this doesn't msi to run right?
`````

## Q187

### Question

`````text
187. After the exact release candidate passes every required check, who should authorise publishing the downloadable release?
`````

### Options offered

`````text
1. You make the final release decision (Recommended)
Show the completed evidence, changes and known limitations, then publish only after your explicit approval.
`````

`````text
2. Publish automatically after all gates pass
Treat creation of the release-candidate tag as authorisation to publish once every required result is complete and passing.
`````

### User answer

`````text
Publish automatically after all gates pass
`````

## Q188

### Question

`````text
188. If the Seagate fails, how should private evaluation data be recovered while keeping the full corpus on Dell?
`````

### Options offered

`````text
1. Rebuild from an authorised forum export
Recover source from GitHub, create a new private dataset version on Dell and rerun qualification; old local evidence may be lost.
`````

`````text
2. Add a separate Dell-attached backup (Recommended)
Plan an encrypted backup on another explicitly approved drive attached to Dell; do not use the Samsung or MSI without a new instruction.
`````

### User answer

`````text
Rebuild from an authorised forum export
`````

## Q189

### Question

`````text
189. If an agent stops responding or runs out of usage, may another joined agent take over its reserved task?
`````

### Options offered

`````text
1. Yes, with a recorded safe handover (Recommended)
Preserve its draft, invalidate the old write reservation, record the last known state and give the replacement a new reservation.
`````

`````text
2. Wait for your instruction
Keep the reservation blocked until you explicitly choose a replacement or release it.
`````

### User answer

`````text
yes with a recorded safe handover, but there should be a rolling ledger of runs will should keep this ledger in sqlite
`````

## Q190

### Question

`````text
190. For the first documentation-only commit, may code-only checks report 'not applicable' with proof that no implementation exists, while documentation, secrets, rights and consistency checks must pass?
`````

### Options offered

`````text
1. Yes, a strictly limited docs profile (Recommended)
Allow the documentation checkpoint without fake test passes; adding any executable code activates the full required code checks.
`````

`````text
2. Build the full checking system first
Delay the first documentation commit until the enforcement tools exist and are qualified; this requires allowing tooling work before that commit.
`````

### User answer

`````text
Yes, a strictly limited docs profile (Recommended)
`````
