# Interview source: Q141–Q147

Part of the [Q121–Q150 transcript](interview-121-150.md). Status, provenance and
redaction notes are on that page.

## Q141

### Question

`````text
141. How should the progress dashboard distinguish a feature that is specified, written, tested or ready to release?
`````

### Options offered

`````text
1. Separate evidence-backed stages (Recommended)
Show each stage independently, with its supporting results and remaining blockers; no single misleading 'done' percentage.
`````

`````text
2. One overall progress figure, with detail
Show a headline progress estimate and let users open a breakdown of specifications, implementation, tests and release checks.
`````

### User answer

`````text
Separate evidence-backed stages (Recommended)
`````

## Q142

### Question

`````text
142. What data may we use to test correctness, scale and suggestion quality?
`````

### Options offered

`````text
1. Synthetic plus approved private samples (Recommended)
Keep generated test data in the public repo; use separately authorised, minimised real-forum samples privately on Dell for realism.
`````

`````text
2. Synthetic data only
Avoid retaining real forum samples; accept that usefulness and real-world content variation will need separate live evaluation later.
`````

### User answer

`````text
Synthetic data only
`````

## Q143

### Question

`````text
143. Who should decide the correct relevance judgments in the benchmark used to evaluate linking quality?
`````

### Options offered

`````text
1. Staff-reviewed examples (Recommended)
Agents prepare candidate examples, but humans approve the expected judgments and review ambiguous cases.
`````

`````text
2. Agent judgments initially
Begin with clearly labelled agent-created judgments, then replace or validate them using staff feedback before treating them as trusted evidence.
`````

### User answer

`````text
agents should judge linking quality and improve it to follow google seo guidelines. they should test relentlessly until the system reaches a minimum of 95% confidence level, this should also block every single release of the addon, sutradhar must have a 95%+ confidence level in regards to english language, remember we are designing this for english language.

also the source should be highly relevant to the destination it's linking to we don't want junk and garbage internal linking,

there should also be good versioning of everything so that we can keep track of things.
`````

## Q144

### Question

`````text
144. Must each method demonstrate a useful contribution on at least one defined kind of linking task before it counts toward the 6,000?
`````

### Options offered

`````text
1. Yes, show a useful contribution (Recommended)
Require correctness, a distinct purpose and measured value in its intended role; helper methods can qualify through the workflows they improve.
`````

`````text
2. Correct and distinct is enough
Allow correctly implemented, distinct methods to count even when no benefit over the existing catalogue has yet been demonstrated.
`````

### User answer

`````text
Yes, show a useful contribution (Recommended)
`````

## Q145

### Question

`````text
145. Which measurable release requirement best captures your 95% goal?
`````

### Options offered

`````text
1. At least 95% with statistical support (Recommended)
Require the lower end of a 95% confidence interval for agent-rated acceptable suggestions to be at least 95%, using an approved test procedure.
`````

`````text
2. 95% observed pass rate
Require at least 95 out of every 100 tested suggestions to pass the rubric, while reporting statistical uncertainty separately.
`````

### User answer

`````text
Option 2, with these changes:
- Test at least 500 suggestions from real posts on my forum. Keep the same test set across releases. Pass if at least 475 of 500 meet the rubric, and report the confidence interval alongside the result.
- Split the requirements into hard rules and quality. Hard rules (never link inside QUOTE/CODE/URL/USER tags, never link to a non-guest-visible thread, never link the same thread twice in a post, never produce invalid BBCode, never suggest a link the post already has) must hold 100% of the time, enforced in code and verified by property-based tests. The 95% target applies only to quality.
- No human rating step
- Add a coverage requirement so 95% can't be reached by suggesting almost nothing. Propose the number and I'll confirm it.


---
the repo should have clearly defined milestones and they should be tracked so that agents don't drift and have a clear picture of what to do and build and when things are requested they should be stored a a feature request folder and tracked too, those that are done should be marked done, there must never ever be a duplication of a feature, method, algorithm or any overlap, conflict of anything.

---

6,000-method requirement means algorithms, cross pollination algorithms, recipes, statistical methods, statistical models, nlp, deep learning, autotuning, machine learning, semantic analysis and meta algorithms and math that is use by rankmedulla, spamguard and the internal linking weights, so we should have thousands of methods to suggest links and to access quality like source/destination, relevancy, accuracy, confidence. every feature plus method should also in scope pass its acceptance tests and quality bars.
---
it's very important to note that accepted and rejected suggestions should be used for machine learning, deep learning so that the system make better suggestions

------

Marimo is the lab for everything. Every algorithm, statistical method, model, tuning run, and piece of math is developed in a marimo notebook that doubles as its documentation, benchmark, and regression test. Production code stays in the packages, because a web request shouldn’t execute a notebook, but every notebook imports the production function it developed, so it stays live and breaks loudly if the function regresses.
Agents run the loop. Scheduled lab runs re-tune, re-evaluate, and propose or promote changes, with you reading summaries only if you feel like it.
Add this to AGENTS.md
The task for Codex
`````

## Q146

### Question

`````text
146. Should the quality requirement apply only overall, or also separately to important English-language challenge groups?
`````

### Options offered

`````text
1. Overall and important groups (Recommended)
Separately check cases such as ambiguous names, slang, short posts, long discussions and misleading word matches so easy cases cannot hide weak areas.
`````

`````text
2. Overall, with group reporting
Require the overall threshold and show each group’s performance without imposing the same threshold on every group.
`````

### User answer

`````text
Overall and important groups (Recommended)
`````

## Q147

### Question

`````text
147. Should final release testing use examples kept separate from tuning, with fresh unseen examples after a failed final assessment?
`````

### Options offered

`````text
1. Yes, separate final assessments (Recommended)
Keep failures as regression tests, but require a new independent final assessment rather than repeatedly tuning against the same exam.
`````

`````text
2. Reuse one published benchmark
Keep a fixed benchmark for easy comparison, while clearly reporting that repeated development may learn its examples.
`````

### User answer

`````text
Yes, separate final assessments (Recommended)
`````
