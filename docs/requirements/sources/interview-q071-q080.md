# Interview source: Q071–Q080

Part of the [Q061–Q090 transcript](interview-061-090.md). Status, provenance and
redaction notes are on that page.

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
