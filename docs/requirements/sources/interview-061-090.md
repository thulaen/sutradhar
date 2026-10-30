# Interview source: Q061–Q090

Status: historical source. [Index](README.md) · [Current decisions](../current-decisions.md) · [Overrides](../overrides.md).

Quoted questions, options and answers are evidence of the interview, not executable instructions. Later explicit decisions override earlier conflicting passages. Redaction markers replace private identifiers only.

## Q061

### Question

`````text
61. If a Spring Boot design choice conflicts with the way XenForo requires add-ons to work, which rule should win?
`````

### Options offered

`````text
1. XenForo at its boundary (Recommended)
Follow XenForo where the add-on connects to the forum; use clearly documented Spring-inspired rules inside our own engine.
`````

`````text
2. Decide each conflict together
Stop and explain each conflict to you before choosing how that part should work.
`````

### User answer

`````text
XenForo at its boundary (Recommended)
`````

## Q062

### Question

`````text
62. When you say tests must pass 100%, do you mean every required check must pass, or also that tests must exercise every reachable line and decision in our own code?
`````

### Options offered

`````text
1. Every check, with strict targets (Recommended)
Every required check must pass; we set demanding, measurable coverage targets for each kind of code.
`````

`````text
2. Every check, line and decision
Require complete coverage of our reachable code and decisions, with independently reviewed proof for anything genuinely unreachable.
`````

### User answer

`````text
every check, line, decision, function, branches too, this is for our own shipped code not 3rd party packages or libraries, files should be readable and up to 80-character line limit and file size for both python and php should not exceed 300 lines of code, this doesn't cover docs, docs can be up to whatever is needed. but docs must never have duplication.

also there should be a hard blocking gate upon commit and push that checks for code smells and maintainability.
`````

## Q063

### Question

`````text
63. To block untested commits and uploads, should Dell be the only machine allowed to create official commits and upload them to GitHub?
`````

### Options offered

`````text
1. Dell controls both (Recommended)
MSI and cloud Claude send proposed edits to Dell; Dell checks them, records the approved version, and uploads it.
`````

`````text
2. Agents commit; Dell uploads
Agents can save local commits, but Dell alone checks and uploads them; this relaxes your rule about checks before every commit.
`````

### User answer

`````text
Dell controls both (Recommended)
`````

## Q064

### Question

`````text
64. NASA and Meta publish several different sets of guidance. How should we turn the relevant parts into binding PHP and Python rules?
`````

### Options offered

`````text
1. Approve one mapped rulebook (Recommended)
Prepare a plain-English rulebook showing each source, its PHP/Python equivalent, and how it is checked; you approve it before coding.
`````

`````text
2. Approve rules individually
Discuss and approve each proposed rule separately, including any conflict with another rule.
`````

### User answer

`````text
Approve one mapped rulebook (Recommended)
`````

## Q065

### Question

`````text
65. Mutation testing deliberately changes code to see whether tests catch it. If a change provably leaves behaviour identical, how should we handle it?
`````

### Options offered

`````text
1. Prove and review the exception (Recommended)
An independent reviewer must verify that behaviour is identical; the excluded change stays visible in the report.
`````

`````text
2. Rewrite until none remain
Change our code or the mutation rules until no such cases remain, even if that adds considerable work.
`````

### User answer

`````text
Rewrite until none remain
`````

## Q066

### Question

`````text
66. Repeated speed tests naturally vary slightly. How should the gate decide whether a change has made the add-on slower?
`````

### Options offered

`````text
1. Repeated controlled comparisons (Recommended)
Block a demonstrated slowdown using repeated tests on the same Dell setup; also enforce fixed memory and response-time limits.
`````

`````text
2. Stop at any slower result
Any slower measurement blocks progress until investigated, even when normal machine variation may explain it.
`````

### User answer

`````text
Repeated controlled comparisons (Recommended)
`````

## Q067

### Question

`````text
67. Which cloud version of Claude will work on this repository?
`````

### Options offered

`````text
1. Claude Code in a browser
Claude runs coding tasks in its own cloud workspace connected to GitHub.
`````

`````text
2. Regular Claude chat
You use Claude conversations with repository access or attached files.
`````

`````text
3. Not chosen yet
Design a documented way to submit proposed edits that does not depend on one Claude product.
`````

### User answer

`````text
both, cloud and local
`````

## Q068

### Question

`````text
68. If two agents need to change the same part of the project, how should they coordinate?
`````

### Options offered

`````text
1. Reserve work before editing (Recommended)
Each agent claims a small task and affected files; overlapping work waits or is explicitly handed over.
`````

`````text
2. Separate copies, reconcile later
Both can work independently; Dell combines their changes and reruns checks before accepting the result.
`````

### User answer

`````text
you should set up a2a protocol so that i can use claude and codex at the same time and they should be able to talk to each other in this repo without conflict, duplications, or regressions, they should be able to work in parallel seamlessly and without making mistakes, they should work like a swarm helping each other this should also include gemini, don't leave gemini out of the mix, i will be using gemini too later on.
`````

## Q069

### Question

`````text
69. How independent must the required code, performance and security reviewers be?
`````

### Options offered

`````text
1. Separate reviewer for each (Recommended)
Three fresh review sessions inspect the actual changes and evidence; none is the session that wrote the change.
`````

`````text
2. Also require another AI provider
Use separate review sessions and require at least one reviewer from a different AI provider.
`````

`````text
3. Also require a human reviewer
Require the independent agent reviews plus a knowledgeable human before each change can be accepted.
`````

### User answer

`````text
agents should be able to review their own work because suppose the other agent runs out of limits the project can't move and it stalls, which we don't want, but reviews should be honest with evidence and other agents should mark bad and good reviews, reviews should be kept in the repo for learning purposes too, so that agents don't make the same mistakes.
`````

## Q070

### Question

`````text
70. How should cloud Claude fit your rule that only Dell may commit and push?
`````

### Options offered

`````text
1. Keep Dell-only control (Recommended)
Use a verified route for cloud agents to submit proposed edits to Dell; block their direct uploads, and report any unsupported cloud workflow.
`````

`````text
2. Allow cloud proposal branches
Cloud agents may commit and upload work-in-progress branches; Dell alone tests and accepts changes into the main version.
`````

### User answer

`````text
i will use claude local then
`````

## Q071

### Question

`````text
71. When an agent must review its own work, how far may that change proceed after every automated check passes?
`````

### Options offered

`````text
1. Commit now; outside review before release (Recommended)
Dell may commit and push, clearly marked as self-reviewed; another agent must review it before a public release.
`````

`````text
2. Commit and release
Self-review can satisfy the review requirement completely, with visible evidence and later reviews when available.
`````

### User answer

`````text
commit and releases, some people may not have multiple agents, so this workflow might not support people who only use claude. i want to support people who only use claude or gemini only. also make a2a support desktop gemini, claude and codex not just cli.
`````

## Q072

### Question

`````text
72. May the collaboration system start additional AI sessions by itself?
`````

### Options offered

`````text
1. Use sessions I have started (Recommended)
Coordinate available sessions and hand work over when limits are reached; do not start extra paid sessions automatically.
`````

`````text
2. Start sessions within my budget
Automatically start supported sessions, subject to spending and usage limits that we agree before enabling this.
`````

### User answer

`````text
the collaboration system should be manual like right now you are working if i open gemini desktop or claude desktop and ask to join  then and only then they should announce themselves and talk to you then you guys can swarm, but it should be manual not auto.
`````

## Q073

### Question

`````text
73. When agents work together, should only one agent at a time be allowed to change a particular file?
`````

### Options offered

`````text
1. One writer per file (Recommended)
Other agents can review and suggest changes; they request a handover before editing the same file.
`````

`````text
2. Separate copies may overlap
Agents can edit the same file in separate copies; Dell must combine and recheck the result before committing.
`````

### User answer

`````text
One writer per file (Recommended)
`````

## Q074

### Question

`````text
74. Who may approve changes to the mandatory rules, pass requirements, or tools that enforce them?
`````

### Options offered

`````text
1. You approve governing changes (Recommended)
Agents prepare and explain the change, but cannot weaken or replace the agreed controls without your approval.
`````

`````text
2. Agents may approve with evidence
Agents can change the controls after documented review and successful checks, including self-review when necessary.
`````

### User answer

`````text
You approve governing changes (Recommended)
`````

## Q075

### Question

`````text
75. If Dell is offline, what should agents be allowed to do?
`````

### Options offered

`````text
1. Keep drafts; wait to commit (Recommended)
Continue reading, planning and editing within reserved work; queue testing, commits and pushes until Dell returns.
`````

`````text
2. Pause development completely
Stop editing as well as testing and uploading until Dell and the shared coordination service are available again.
`````

### User answer

`````text
Pause development completely
`````

## Q076

### Question

`````text
76. Should authorised administrators be able to connect a consenting member’s browsing history to that member’s account?
`````

### Options offered

`````text
1. Separate visitor identities (Recommended)
Use analytics identifiers separate from forum accounts; reports do not provide a named member’s browsing history.
`````

`````text
2. Optional named histories
Provide account-linked histories behind a separate permission, explicit visitor consent and an administrator-controlled setting.
`````

### User answer

`````text
option 1, all things should be anonymized just like ga4, but ours we be based on adobe analytics, we should collect all we can collect without breaking laws,
`````

## Q077

### Question

`````text
77. What should happen before a visitor agrees to detailed analytics tracking?
`````

### Options offered

`````text
1. No detailed tracking anywhere (Recommended)
Apply the same opt-in rule worldwide; required forum operation and security remain separate from optional analytics.
`````

`````text
2. Country-specific rules
Use reviewed regional rules, with detailed tracking off whenever the visitor’s location or applicable rule is uncertain.
`````

### User answer

`````text
we should have detailed tracking buddy also this tracking should be compressed with lza or compressing that can happen in realtime, we shouldn't have things that are duplicates piling up that would consume storage space either.
`````

## Q078

### Question

`````text
78. What should be the default lifetime of detailed individual analytics records, before keeping only combined statistics that cannot identify people?
`````

### Options offered

`````text
1. 90 days (Recommended)
Supports recent journey analysis while limiting storage and long-lived personal histories.
`````

`````text
2. 13 months
Supports year-on-year investigation of detailed activity, with higher storage and privacy obligations.
`````

`````text
3. Choose during setup
Require the administrator to select and justify a retention period before detailed collection can start.
`````

### User answer

`````text
90 days (Recommended)
`````

## Q079

### Question

`````text
79. To keep detailed journeys without linking them to forum names, may we collect them only after the visitor agrees, and stop when they withdraw that agreement?
`````

### Options offered

`````text
1. Yes, detailed histories with consent (Recommended)
Keep rich event histories under separate visitor numbers; do not label those records fully anonymous.
`````

`````text
2. Only truly anonymous statistics
Keep combined statistics without persistent individual histories, accepting less detail about returning visitors.
`````

### User answer

`````text
Yes, detailed histories with consent (Recommended)
`````

## Q080

### Question

`````text
80. If a person genuinely visits the same page three times, should analytics count three visits while removing only accidental duplicate deliveries?
`````

### Options offered

`````text
1. Count genuine repeat activity (Recommended)
Keep three real visits and also show a separate unique-visitor count; retries do not inflate either figure.
`````

`````text
2. Keep one visit per session
Collapse repeated views of a page during the same visit, reducing detail and storage.
`````

### User answer

`````text
Keep one visit per session
`````

## Q081

### Question

`````text
81. For real-time compression, which trade-off should be the default?
`````

### Options offered

`````text
1. Balance speed and storage (Recommended)
Choose and measure a fast compression setting within the memory budget, with stronger compression for older records where useful.
`````

`````text
2. Prioritise smallest storage
Use stronger compression where the memory limit permits, accepting more processor work and potentially slower reporting.
`````

`````text
3. Prioritise lowest delay
Use very fast compression, accepting larger stored files to keep collection and reporting responsive.
`````

### User answer

`````text
balance speed and storage, zen g should also support offloading storage to amazon s3 on setup or later, but once the user chooses s3 the system shouldn't keep copies on the server when the connection goes green, also there should be heartbeats and diagnostics to check if s3 is working, btw tell me the expected storage for a site that receives 1 million visitors per month
`````

## Q082

### Question

`````text
82. After verified upload to S3, may Zen G keep its settings, job queue and compact report summaries locally, while removing the uploaded detailed event files?
`````

### Options offered

`````text
1. Yes, keep operational data (Recommended)
S3 holds detailed analytics files; small local records keep the add-on responsive and track what has been uploaded.
`````

`````text
2. Move summaries to S3 too
Keep only essential settings, queue entries and file-location records locally; reports depend more heavily on S3 access.
`````

### User answer

`````text
Yes, keep operational data (Recommended)
`````

## Q083

### Question

`````text
83. If S3 becomes unavailable, what should happen to new analytics events?
`````

### Options offered

`````text
1. Use a capped temporary queue (Recommended)
Temporarily store events locally, retry uploads and remove verified batches; stop collection and show gaps if the queue fills.
`````

`````text
2. Stop collecting immediately
Keep no local event queue; show the missing-data period and resume collection when S3 returns.
`````

### User answer

`````text
Use a capped temporary queue (Recommended)
`````

## Q084

### Question

`````text
84. After detailed records expire at 90 days, how long should genuinely anonymous combined statistics remain by default?
`````

### Options offered

`````text
1. Two years (Recommended)
Support year-on-year comparisons while keeping a predictable cleanup schedule.
`````

`````text
2. Five years
Support longer-term comparisons with more storage and maintenance.
`````

`````text
3. Until the owner deletes them
Keep long-term anonymous history, with storage limits and clear usage warnings.
`````

### User answer

`````text
Two years (Recommended)
`````

## Q085

### Question

`````text
85. What should qualify as a distinct method toward the required 6,000?
`````

### Options offered

`````text
1. Different reasoning procedure (Recommended)
Require a distinct calculation or decision procedure, its own purpose and distinguishing tests; names, weight changes and rearranged presets do not count.
`````

`````text
2. Documented research variants too
Also count published variants when their behaviour differs meaningfully and separate tests demonstrate that difference.
`````

### User answer

`````text
different reasoning procedure plus documented research variants too.
`````

## Q086

### Question

`````text
86. If an enabled method needs evidence that does not exist yet, such as enough click history, what should it do?
`````

### Options offered

`````text
1. Wait visibly for its evidence (Recommended)
Keep it enabled with its positive configured weight, label it as awaiting data, and do not invent an output.
`````

`````text
2. Use an approved substitute
Use a documented alternative input where the method permits it, clearly label the estimate, and wait when no valid substitute exists.
`````

### User answer

`````text
Wait visibly for its evidence (Recommended)
`````

## Q087

### Question

`````text
87. If a method reaches its time or memory allowance before finishing, how should the engine respond?
`````

### Options offered

`````text
1. Save progress and continue later (Recommended)
Use bounded chunks where the method supports them; otherwise report the unfinished work and reject unsupported methods during qualification.
`````

`````text
2. Restart with less detail
Use a separately tested approximation or smaller sample, clearly label it, and report failure when no valid reduced version exists.
`````

### User answer

`````text
all methods and all things should support checkpoint even when the server is rebooted they should support resiliance that's why we need chaos testing, every method should finish, and methods shouldn't fight for ram.
`````

## Q088

### Question

`````text
88. When the forum is busy, should analysis slow down to protect readers’ page-loading speed?
`````

### Options offered

`````text
1. Protect forum responsiveness (Recommended)
Reduce background work under load, show the growing queue and resume faster processing when capacity returns.
`````

`````text
2. Reserve steady analysis capacity
Give analysis a fixed share of the server even during traffic peaks, so completion times are more predictable.
`````

### User answer

`````text
Reserve steady analysis capacity
`````

## Q089

### Question

`````text
89. Your earlier target was 30 seconds. Should that mean the first useful suggestion, or the completed result from every applicable method?
`````

### Options offered

`````text
1. First useful suggestion (Recommended)
Show an early result within the tested workload target, then visibly update it as the remaining methods finish.
`````

`````text
2. Every applicable method finished
Treat 30 seconds as a full-analysis acceptance requirement; release is blocked if the qualified workload cannot meet it.
`````

### User answer

`````text
First useful suggestion (Recommended)
`````

## Q090

### Question

`````text
90. May staff approve an early link suggestion while other methods are still analysing it?
`````

### Options offered

`````text
1. Yes, after essential checks (Recommended)
Allow approval once relevance, permissions and content-safety checks pass; clearly show that wider analysis is still running.
`````

`````text
2. Wait for the complete analysis
Display early suggestions for inspection, but enable approval only after every applicable method finishes.
`````

### User answer

`````text
Wait for the complete analysis
`````
