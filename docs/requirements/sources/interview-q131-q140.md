# Interview source: Q131–Q140

Part of the [Q121–Q150 transcript](interview-121-150.md). Status, provenance and
redaction notes are on that page.

## Q131

### Question

`````text
131. How should the large number of settings be presented?
`````

### Options offered

`````text
1. Simple view plus advanced details (Recommended)
Start with safe defaults and a few understandable choices; reveal detailed controls and explanations when requested.
`````

`````text
2. All settings visible together
Show the complete configuration in searchable sections, with explanations beside every setting.
`````

### User answer

`````text
all setting vible together but collapasable and the choice for the user to decide how they should be shown, there must be mutliple ways to show a large number of settings that are mobile friendly too, think like an expert frontend designer and cover all edge cases.

-------------

there should also be a hard blocking gates that covers edge cases, memory issues, race conditions, databases issues, sutradhar should never corrupt peoples production servers, there should be seperation of concerns.
`````

## Q132

### Question

`````text
132. Which appearance modes should the first release support?
`````

### Options offered

`````text
1. Light and dark, follow preference (Recommended)
Provide tested light and dark themes, follow the visitor’s chosen preference and allow a manual switch.
`````

`````text
2. Light mode only
Concentrate the first release on one fully tested accessible appearance.
`````

`````text
3. Follow the XenForo theme only
Adapt to the installed forum style without offering a separate Sutradhar appearance switch.
`````

### User answer

`````text
Light and dark, follow preference (Recommended)
`````

## Q133

### Question

`````text
133. When an administrator changes several settings, when should they take effect?
`````

### Options offered

`````text
1. Review changes, then apply together (Recommended)
Show one clear before-and-after summary, validate the combined settings and save them as one reversible version.
`````

`````text
2. Save each change immediately
Validate and apply each setting as it changes, with an obvious undo action and a warning before disruptive changes.
`````

### User answer

`````text
Review changes, then apply together (Recommended)
`````

## Q134

### Question

`````text
134. Before an upgrade changes stored data, should Sutradhar require a verified recovery backup?
`````

### Options offered

`````text
1. Yes, block without recovery proof (Recommended)
Check that the required backup exists and the recovery procedure has been tested before allowing the upgrade.
`````

`````text
2. Allow an explicit owner override
Strongly recommend the backup, but let the owner accept the risk and continue with a recorded override.
`````

### User answer

`````text
Allow an explicit owner override
`````

## Q135

### Question

`````text
135. If a safe upgrade requires pausing Sutradhar briefly, how should that happen?
`````

### Options offered

`````text
1. Owner-approved maintenance window (Recommended)
Explain the pause, let the owner choose when it happens, and keep the forum available wherever the change permits.
`````

`````text
2. Pause immediately after approval
Once the owner approves the upgrade, pause affected features and complete the change straight away.
`````

### User answer

`````text
Pause immediately after approval
`````

## Q136

### Question

`````text
136. When Sutradhar is uninstalled, what should happen to its analytics, settings, suggestions and learned models? Approved links will remain in either case.
`````

### Options offered

`````text
1. Offer export and an explicit choice (Recommended)
Show what will remain locally or in cloud storage, then let the owner retain or delete Sutradhar-owned data.
`````

`````text
2. Retain data by default
Keep data for a later reinstall until the owner runs a separate cleanup action.
`````

`````text
3. Delete Sutradhar-owned data
Remove its stored data after clear confirmation, while preserving published content and approved links.
`````

### User answer

`````text
Offer export and an explicit choice (Recommended)
`````

## Q137

### Question

`````text
137. How should owners hear about serious faults such as failed storage, stalled jobs or a broken data connection?
`````

### Options offered

`````text
1. Dashboard and XenForo staff alerts (Recommended)
Show persistent diagnostics and notify the responsible staff through XenForo, grouping repeated reports of the same problem.
`````

`````text
2. Also offer email alerts
Provide optional email delivery in addition to the dashboard and XenForo alerts, with configurable recipients and frequency.
`````

### User answer

`````text
Dashboard and XenForo staff alerts (Recommended)
`````

## Q138

### Question

`````text
138. Should the diagnostics page provide a shareable report for an agent or support person to investigate a fault?
`````

### Options offered

`````text
1. Yes, previewed and scrubbed (Recommended)
Export versions, error codes and relevant checks after removing credentials and private content; let the owner inspect it before sharing.
`````

`````text
2. Keep diagnostics on the server
Provide readable on-screen details, but no packaged support export in the first release.
`````

### User answer

`````text
Yes, previewed and scrubbed (Recommended)
`````

## Q139

### Question

`````text
139. Before agents implement a feature, how should its written specification be approved?
`````

### Options offered

`````text
1. You approve each feature specification (Recommended)
Present a plain-English description, examples and pass requirements before coding that feature.
`````

`````text
2. Approve the master specification once
Let agents develop detailed feature specifications within the approved master plan, returning to you only for new decisions or scope changes.
`````

### User answer

`````text
You approve each feature specification (Recommended)
`````

## Q140

### Question

`````text
140. May development commits contain completed, fully checked parts of the system before all 6,000 methods and all required features are finished?
`````

### Options offered

`````text
1. Yes; block release until complete (Recommended)
Allow incremental work with every applicable quality check passing; clearly mark unfinished capabilities and prohibit counting placeholders as implemented.
`````

`````text
2. No incomplete-system commits
Keep proposed work outside the official history until the entire first-release scope is implemented and passes all required checks.
`````

### User answer

`````text
we should allow commits to happen and pushes to github to happen but actual addon release should be marked as incomplete until 6000 methods are done and all things are built.

these doesn't mean skipping our hard blocking gates like quality checks, every quality check should run.
`````
