# Interview source: Q121–Q130

Part of the [Q121–Q150 transcript](interview-121-150.md). Status, provenance and
redaction notes are on that page.

## Q121

### Question

`````text
121. For redirects, canonical addresses and instructions that keep pages out of search results, should Sutradhar only report issues or also apply an administrator-approved fix?
`````

### Options offered

`````text
1. Report and propose only (Recommended)
Explain the issue and recommended action; make no such site changes from Sutradhar.
`````

`````text
2. Apply individually approved fixes
An administrator reviews the exact affected pages and approves each supported change before Sutradhar applies it.
`````

### User answer

`````text
Report and propose only (Recommended)
`````

## Q122

### Question

`````text
122. When GA4 and Sutradhar report different traffic numbers, how should the learning engine use them?
`````

### Options offered

`````text
1. Prefer Sutradhar; compare separately (Recommended)
Use our defined metrics as the primary evidence and show GA4 alongside them; do not add overlapping counts together.
`````

`````text
2. Prefer GA4 when available
Use GA4 as the primary source for supported traffic metrics and our analytics for additional behaviour and gaps.
`````

`````text
3. Let the owner choose per metric
Require an explicit source choice for each overlapping measure, with the differences explained.
`````

### User answer

`````text
Prefer GA4 when available
`````

## Q123

### Question

`````text
123. Google currently limits index-status inspections to 2,000 URLs per site per day. On a large forum, which pages should get checked first?
`````

### Options offered

`````text
1. Priority pages plus a rotating sample (Recommended)
Check new, changed and important pages sooner while steadily covering the rest; show when each result was last checked.
`````

`````text
2. Equal rotation across all pages
Give each eligible page a turn, accepting slower detection of issues on important or newly changed pages.
`````

### User answer

`````text
Priority pages plus a rotating sample (Recommended)
`````

## Q124

### Question

`````text
124. If background services or hard memory controls need server-level preparation, which installation promise should we make?
`````

### Options offered

`````text
1. Agent prepares server once (Recommended)
An authorised agent or host prepares the supported server; installation and normal operation then use the XenForo control panel.
`````

`````text
2. Control-panel-only on compatible hosts
Require the necessary server facilities to exist already, and refuse installation with a plain explanation when they do not.
`````

### User answer

`````text
agent prepares server once, but please make it easy to just point and click for all preparations and installations.
`````

## Q125

### Question

`````text
125. Which search service should be required and fully supported for the first release?
`````

### Options offered

`````text
1. OpenSearch (Recommended)
Choose one documented search platform and test its supported version thoroughly; add other providers later.
`````

`````text
2. Elasticsearch
Use Elasticsearch as the required platform, including a compatibility review if reusing XenForo Enhanced Search.
`````

`````text
3. Both from the first release
Support and qualify both platforms, with the same capability and recovery tests for each.
`````

### User answer

`````text
both from the first release with the option to use hosted opensearch from amazon on gcp, the same applies to storage, aws or gcp
`````

## Q126

### Question

`````text
126. May staff approve a suggestion after every method with available evidence finishes, even while other methods are visibly waiting for missing data?
`````

### Options offered

`````text
1. Yes, show the missing evidence (Recommended)
Distinguish finished analysis from unavailable inputs; required safety and permission checks must still pass.
`````

`````text
2. No, wait for every required input
Hold approval until all required methods have their evidence and finish, even if an external connection or history is missing.
`````

### User answer

`````text
Yes, show the missing evidence (Recommended)
`````

## Q127

### Question

`````text
127. Should the setup assistant create supported cloud storage and search resources, or only connect to resources the owner already created?
`````

### Options offered

`````text
1. Create or connect, with approval (Recommended)
Offer guided setup that shows the provider, region and expected charges before the owner authorises resource creation.
`````

`````text
2. Connect existing resources only
Provide connection checks and clear instructions, but leave creation of paid cloud resources to the owner.
`````

### User answer

`````text
create connect with approval and with supported logins, however all this stuff should be optional
`````

## Q128

### Question

`````text
128. If the owner changes from local storage to cloud storage, or between AWS and Google Cloud, should Sutradhar handle the move?
`````

### Options offered

`````text
1. Yes, a resumable guided move (Recommended)
Copy in bounded batches, verify each batch, switch the active location safely, then remove old copies under the approved migration plan.
`````

`````text
2. Export and reconnect only
Provide an export and connection checks, but require the owner or agent to perform the actual transfer separately.
`````

### User answer

`````text
Yes, a resumable guided move (Recommended)
`````

## Q129

### Question

`````text
129. When cloud usage approaches an owner-set budget, what should Sutradhar do?
`````

### Options offered

`````text
1. Warn and limit optional work (Recommended)
Warn early and stop optional rebuilds or expensive analysis at configured usage limits; explain that provider fees may still continue.
`````

`````text
2. Warn but keep working
Show escalating cost alerts while allowing all configured jobs to continue.
`````

### User answer

`````text
Warn but keep working
`````

## Q130

### Question

`````text
130. How should the Indian glossary names appear in the user interface?
`````

### Options offered

`````text
1. English label, glossary name alongside (Recommended)
Use labels such as “Internal linking · Setu” and “Health checks · Drishti,” with meanings available in help text.
`````

`````text
2. Glossary name first
Use names such as “Setu” and “Drishti” as the main labels, with English explanations underneath.
`````

### User answer

`````text
English label, glossary name alongside (Recommended)
`````
