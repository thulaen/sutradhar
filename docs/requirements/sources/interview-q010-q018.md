# Interview source: Q010–Q018

Part of the [Q001–Q030 transcript](interview-001-030.md). Status, provenance and
redaction notes are on that page.

## Q010

### Question

`````text
10 of 200 — Which XenForo version will the first installation use? If you know the full version number, enter it in your own words.
`````

### Options offered

`````text
1. XenForo 2.3
Plan around the 2.3 series and verify the exact installed release before integration testing.
`````

`````text
2. XenForo 2.2
Check the older series separately before promising PHP 8.5 compatibility.
`````

`````text
3. Not chosen yet
Select a supported release after checking PHP 8.5 compatibility and your other add-ons.
`````

### User answer

`````text
XenForo 2.3
`````

## Q011

### Question

`````text
11 of 200 — Where will your forum run?
`````

### Options offered

`````text
1. A server you control
You or your administrator can install software and run background processing continuously.
`````

`````text
2. Shared web hosting
Your hosting company limits background jobs, memory controls and installed software.
`````

`````text
3. XenForo Cloud
The hosting service controls the environment, so we must check its add-on and processing restrictions.
`````

### User answer

`````text
A server you control
`````

## Q012

### Question

`````text
12 of 200 — Roughly how many posts must the add-on handle at the first release? Enter a closer estimate if you have one.
`````

### Options offered

`````text
1. Under 100,000
Design and measure against a smaller forum, including future growth.
`````

`````text
2. 100,000 to 1 million
Plan for a substantial content collection that must be processed in manageable pieces.
`````

`````text
3. Over 1 million
Treat large-scale searching, processing time and storage use as central design requirements.
`````

### User answer

`````text
Over 1 million
`````

## Q013

### Question

`````text
13 of 200 — Which languages must the first release understand well enough to suggest links?
`````

### Options offered

`````text
1. English only initially
Prove relevance and learning in English before adding other languages.
`````

`````text
2. English plus named languages
Enter the additional languages so each gets its own quality checks.
`````

`````text
3. Many languages from release
Make multilingual accuracy a first-release requirement; we will need an explicit supported-language list.
`````

### User answer

`````text
English only initially
`````

## Q014

### Question

`````text
14 of 200 — What should happen to the million-plus posts that already exist when you install it?
`````

### Options offered

`````text
1. Gradual background review (Recommended)
Work through older posts steadily while giving new and edited posts priority.
`````

`````text
2. Finish old content first
Complete an initial review of existing content before starting normal continuous suggestions.
`````

`````text
3. Start with new activity
Process new and edited posts first; review older posts only when you choose sections to include.
`````

### User answer

`````text
it should process all threads and there should be a progress bar with checkpoint, the addon should also be powered by elasticsearch when availalbe. so it should support elasticsearch or opensearch straight from the jump and everything about this addon should be point and click without ever need the users to touch the command line and that should be one of the rules in the repo
`````

## Q015

### Question

`````text
15 of 200 — In normal conditions, how soon after a new post should suggestions appear?
`````

### Options offered

`````text
1. Within a few minutes (Recommended)
Aim for prompt background results while protecting forum speed and the memory limit.
`````

`````text
2. Within 30 seconds
Make a much faster response a measured requirement, potentially needing more CPU or simpler initial processing.
`````

`````text
3. Adapt to server pressure
Let suggestions take longer when the server is busy, with the current delay clearly shown.
`````

### User answer

`````text
Within 30 seconds
`````

## Q016

### Question

`````text
16 of 200 — Should the 2 GB limit also include Elasticsearch or OpenSearch when it serves this add-on?
`````

### Options offered

`````text
1. Separate search allowance (Recommended)
Keep all add-on processing within 2 GB; clearly report that the search service needs its own agreed memory allowance.
`````

`````text
2. Include search in 2 GB
Require the add-on and its search service together to fit within 2 GB, and prove feasibility before building further.
`````

`````text
3. Use existing shared search only
Allow search only through a service the forum already runs, and measure the extra load caused by the add-on.
`````

### User answer

`````text
Separate search allowance (Recommended)
`````

## Q017

### Question

`````text
17 of 200 — What should happen if neither Elasticsearch nor OpenSearch is available?
`````

### Options offered

`````text
1. Built-in search mode (Recommended)
Continue using the forum database, clearly explaining any slower processing or reduced capabilities.
`````

`````text
2. Require a search service
Keep linking unavailable until an administrator connects a supported search service through setup.
`````

`````text
3. Suggestions pause temporarily
Preserve existing links and queued work, but wait for a search service before producing suggestions.
`````

### User answer

`````text
Require a search service
`````

## Q018

### Question

`````text
18 of 200 — While the first review of all existing threads is running, what should take priority?
`````

### Options offered

`````text
1. New activity first (Recommended)
Aim for the 30-second target on new and edited posts while older threads progress in the remaining capacity.
`````

`````text
2. A fixed share for each
Reserve processing capacity for both old and new content, so neither can indefinitely delay the other.
`````

`````text
3. Finish existing threads first
Complete the initial review sooner, accepting that new-post suggestions may miss the 30-second target during that period.
`````

### User answer

`````text
finish existing threads first then the system continues to work and fire on every new thread, and when changes are made to a thread or resource the system should update itself. it should be sentient, the addon should also support mcp in version 1 this should let allow agents to read all content make suggestions and to tweak the algorithms, the addon should also support gsc, this would should make use of gsc data to make suggestions to threads and posts that are crawled by not indexed plus discovered but not indexed and content that has low impressions plus content that has high impressions but zero clicks, in a nutshell zen g should ship with a playbook that also have over 10 000 use cases and scenarios for seo, but the should be modular monolith, also this gen g addon should be built like a framework, in fact let's make it an seo framework, let's make it like this it ships with internal linking, it also ships with mcp, it ships with access to gsc for internal linking, it ships with mcp for moderation, it ships with mcp for content creation such that agents like claude and codex or chatgpt can create content that is zero duplication that what already exists. the content editor we call it "Saturn" so the workflow will be like this user writes asks claude via mcp to create content claude connects to zen g and writes content based on the instructions given and claude puts internal linking were appropiate if any and the content shows up on xf acp and the user approve where it should go and which forum node or sub forum or whatever. or whether it's a resource item, zen g should be built in the spirit of spring boot framework and should be opionated with the vibes of modular monolith and should be extensible, it should have a sub addon system called "Mars" to allow other devs and contributors to add other things, so gen g should be and seo framework inspired by spring boot and you should figure out how this should work.
`````
