# Interview source: Q061–Q070

Part of the [Q061–Q090 transcript](interview-061-090.md). Status, provenance and
redaction notes are on that page.

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
