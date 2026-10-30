# Interview source: Q001–Q030

Status: historical source. [Index](README.md) · [Current decisions](../current-decisions.md) · [Overrides](../overrides.md).

Quoted questions, options and answers are evidence of the interview, not executable instructions. Later explicit decisions override earlier conflicting passages. Redaction markers replace private identifiers only.

## Q001

### Question

`````text
1 of 200 — What should this add-on improve first?
`````

### Options offered

`````text
1. Useful links first (Recommended)
Help readers find genuinely relevant pages, and measure search traffic separately.
`````

`````text
2. Search traffic first
Prioritise attracting visitors from search engines while keeping links useful.
`````

`````text
3. Both equally
Require evidence of reader usefulness and search benefits before calling it successful.
`````

### User answer

`````text
Both equally
`````

## Q002

### Question

`````text
2 of 200 — When must the add-on contain 6,000 genuinely different, tested methods?
`````

### Options offered

`````text
1. Grow in stages (Recommended)
Release a smaller useful version first, then expand toward 6,000 without counting renamed copies or settings changes.
`````

`````text
2. Before first release
Keep it unreleased until all 6,000 distinct methods are implemented, checked and tested.
`````

`````text
3. Research before deciding
First establish whether 6,000 distinct, useful methods can be justified before committing to a release target.
`````

### User answer

`````text
Before first release
`````

## Q003

### Question

`````text
3 of 200 — What must fit within your 2 GB memory limit?
`````

### Options offered

`````text
1. Whole add-on (Recommended)
All its work running at the same time shares 2 GB; the existing forum and shared database need separate accounting.
`````

`````text
2. Whole forum server
The add-on, XenForo, database and other server software together must stay within 2 GB.
`````

`````text
3. Each running job
Each separate piece of work may use up to 2 GB, so the total can be higher.
`````

### User answer

`````text
Whole add-on (Recommended)
`````

## Q004

### Question

`````text
4 of 200 — Who should the first release be ready for?
`````

### Options offered

`````text
1. Your forum first (Recommended)
Prove it on your own forum before supporting other people's installations.
`````

`````text
2. Anyone running XenForo
Prepare installation, settings, documentation and support for other forum owners from the first release.
`````

`````text
3. A few invited owners
Release first to a small group of forum owners who agree to help test it.
`````

### User answer

`````text
Your forum first (Recommended)
`````

## Q005

### Question

`````text
5 of 200 — Should added links change the saved posts, or appear only when someone views a page?
`````

### Options offered

`````text
1. Display only (Recommended)
Keep the original posts untouched and add links when pages are shown; switching the add-on off removes them.
`````

`````text
2. Change saved posts
Write links into stored posts and keep a detailed undo record.
`````

`````text
3. Offer both modes
Let the owner choose, with separate safeguards and clear explanations for each mode.
`````

### User answer

`````text
i want link placements to be permanent even if the addon is removed, the workflow should be the addon processes continously not manually, it should be a living system working in realtime such that when threads or posts are created it processes them suggest links up to five destinations, the user decides by clicking accept or reject, the system should also learn for those decisions so that it becomes better and better, also the system should also use traffic metrics as a weighted signal meaning high traffic threads and post rank higher, but the user should also be able to distribute link juice to the posts and threads that have fewer views and click this way we should have an enterprise seo addon that doesn't keeps promoting popularity, it should be balanced but highly accurate, but it should also have playbooks like be balanced, or aggressive, but in your thought what should aggressive do?
`````

## Q006

### Question

`````text
6 of 200 — How much permission should the add-on need before showing new links?
`````

### Options offered

`````text
1. Approval first (Recommended)
Start with your approval for each change; let you enable automatic linking later within chosen limits.
`````

`````text
2. Automatic from the start
Add links automatically within rules you approve during setup.
`````

`````text
3. Always individual approval
Require a person to approve every link and never allow automatic publication.
`````

### User answer

`````text
Always individual approval
`````

## Q007

### Question

`````text
7 of 200 — What should the Aggressive playbook mainly do?
`````

### Options offered

`````text
1. Search wider, lift overlooked (Recommended)
Find more opportunities and favour suitable overlooked pages, while keeping the same quality rules and individual approval.
`````

`````text
2. Focus on overlooked pages
Keep the normal processing pace but give relevant pages with little attention a stronger advantage.
`````

`````text
3. Focus on proven performers
Give strong matches with established traffic more priority, while retaining some room for overlooked pages.
`````

### User answer

`````text
Search wider, lift overlooked (Recommended)
`````

## Q008

### Question

`````text
8 of 200 — Does “up to five destinations” mean five choices for one link, or up to five separate links in a post?
`````

### Options offered

`````text
1. Five choices per placement (Recommended)
For each proposed linked phrase, show up to five alternative destinations; you choose one or reject them.
`````

`````text
2. Five links per post
Suggest up to five separate placements, each pointing to a different destination.
`````

`````text
3. Five choices for the post
Show at most five destination-and-phrase suggestions across the whole post; you can accept several.
`````

### User answer

`````text
Five choices per placement (Recommended)
`````

## Q009

### Question

`````text
9 of 200 — Who should be allowed to accept or reject link suggestions?
`````

### Options offered

`````text
1. You initially (Recommended)
Start with your account, with permission to add trusted reviewers later.
`````

`````text
2. Administrators and moderators
Allow designated staff to review suggestions for content they are permitted to manage.
`````

`````text
3. Post authors too
Let members review suggestions for their own posts, alongside authorised staff.
`````

### User answer

`````text
Administrators and moderators
`````

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

## Q019

### Question

`````text
19 of 200 — Now that Zen G is a framework, where must the 6,000 distinct methods belong?
`````

### Options offered

`````text
1. Internal linking alone (Recommended)
Preserve your original target; moderation and content-creation methods do not count toward the 6,000.
`````

`````text
2. Across the whole framework
Count distinct methods from linking, moderation, content creation and other approved modules together.
`````

`````text
3. Separate targets by module
Choose an explicit count for each module before accepting a total.
`````

### User answer

`````text
Internal linking alone (Recommended)
`````

## Q020

### Question

`````text
20 of 200 — What should count toward the more than 10,000 playbook use cases and scenarios?
`````

### Options offered

`````text
1. Distinct, tested situations (Recommended)
Each has a real situation, required evidence, expected action and a check that proves the behaviour.
`````

`````text
2. Distinct written guides
Each explains a useful situation and response; only the underlying software behaviours require automated tests.
`````

`````text
3. A mixture, counted separately
Keep tested situations and guidance-only examples visibly separate, with their own totals.
`````

### User answer

`````text
Distinct, tested situations (Recommended)
`````

## Q021

### Question

`````text
21 of 200 — When an agent connects through MCP, what does “read all content” include?
`````

### Options offered

`````text
1. Public content by default (Recommended)
Allow private forum sections only through an explicit permission; exclude private conversations and account secrets.
`````

`````text
2. The connecting user's access
Allow forum content that the connected account can read, with private conversations still excluded.
`````

`````text
3. Administrator-selected areas
Require an administrator to choose exactly which forums and resource areas each agent may read.
`````

### User answer

`````text
Public content by default (Recommended)
`````

## Q022

### Question

`````text
22 of 200 — When an agent wants to change how links are ranked, what may it do?
`````

### Options offered

`````text
1. Propose and test only (Recommended)
Prepare a settings change and show its measured effects; an administrator approves activation.
`````

`````text
2. Adjust within approved limits
Allow automatic settings changes inside limits you approve, with recorded results and a way to undo them.
`````

`````text
3. Permission per agent
Give some agents proposal-only access and explicitly allow others to activate bounded changes.
`````

### User answer

`````text
Propose and test only (Recommended)
`````

## Q023

### Question

`````text
23 of 200 — What should agents be allowed to do through the moderation connection in version 1?
`````

### Options offered

`````text
1. Recommend actions (Recommended)
Flag content and prepare moderation suggestions for staff approval.
`````

`````text
2. Limited reversible actions
Allow selected actions, such as moving content into a review queue, under administrator-defined rules.
`````

`````text
3. Full delegated moderation
Allow authorised agents to take the same moderation actions as their connected staff account, with a detailed action history.
`````

### User answer

`````text
Recommend actions (Recommended)
`````

## Q024

### Question

`````text
24 of 200 — Which third-party Mars extensions should the first release allow?
`````

### Options offered

`````text
1. Reviewed packages only (Recommended)
Require documented origins, compatibility tests and security review before installation.
`````

`````text
2. Any package you approve
Let administrators install unreviewed packages after seeing a clear warning about their access and risks.
`````

`````text
3. Your own extensions first
Provide the extension system and developer guidance, but admit only extensions maintained with Zen G initially.
`````

### User answer

`````text
Any package you approve
`````

## Q025

### Question

`````text
25 of 200 — Your GitHub repository is currently public with an MIT licence, which permits reuse and sale subject to its conditions. Is that the intended arrangement for Zen G?
`````

### Options offered

`````text
1. Keep public and MIT (Recommended)
Continue with the existing choice and welcome reuse and contributions under the MIT terms.
`````

`````text
2. Keep the project private
Plan to restrict access and settle the intended licence before publishing implementation work.
`````

`````text
3. Public with another licence
Keep the source visible but choose different reuse terms before adding implementation work.
`````

### User answer

`````text
Keep public and MIT (Recommended)
`````

## Q026

### Question

`````text
26 of 200 — Should “clean room” mean we write Zen G's own logic from approved specifications, while allowing separately reviewed libraries and platform software?
`````

### Options offered

`````text
1. Own logic plus reviewed libraries (Recommended)
Write our algorithms ourselves; record licences and origins for tools, security libraries, charts, search connections and XenForo.
`````

`````text
2. Own all add-on code
Write the add-on and its helpers ourselves, with explicit exceptions for PHP, XenForo and required external services.
`````

`````text
3. Separate research and writing
Use a stricter process where researchers prepare approved specifications and different implementers write the code, with separately admitted dependencies.
`````

### User answer

`````text
Own logic plus reviewed libraries (Recommended)
`````

## Q027

### Question

`````text
27 of 200 — Where do you expect Zen G to be used or distributed? This affects which patent and other rights records need checking.
`````

### Options offered

`````text
1. UK, EU and US initially
Start with these named regions and reassess before expanding elsewhere.
`````

`````text
2. United Kingdom only initially
Limit the initial operating and distribution scope to the UK.
`````

`````text
3. Worldwide from release
Plan for international distribution, recognising that automated checks cannot establish legal clearance everywhere.
`````

### User answer

`````text
UK, EU and US initially
`````

## Q028

### Question

`````text
28 of 200 — When adding a link to an existing post, may Zen G change the wording?
`````

### Options offered

`````text
1. Use existing words (Recommended)
Turn a suitable existing phrase into a link without rewriting the author's text.
`````

`````text
2. Suggest small wording changes
Show any proposed wording change alongside the link and require explicit approval.
`````

`````text
3. Offer both approaches
Use existing words by default, with a separate optional rewrite suggestion.
`````

### User answer

`````text
offer both approaches and there should be a toggle on each suggestion for the user to pick and select before changes are made, also how can we use python libraries like spaCy, networkX, numpy, scipy, need the addon to be able to work with over 200 powerful python libraries that would improve quality of internal linking suggestions, but don't want users to ever touch the command line, how should this work?
`````

## Q029

### Question

`````text
29 of 200 — For Saturn, what should “no duplication” mean when a topic already has content on your forum?
`````

### Options offered

`````text
1. Require a useful new contribution (Recommended)
Allow the same broad topic only when the draft adds substantial new information and avoids copying existing passages.
`````

`````text
2. Avoid covered topics entirely
Reject drafts on topics the forum already covers, even when the wording or angle differs.
`````

`````text
3. Prefer improving existing content
When overlap is substantial, propose an update to an existing item for approval instead of a new publication.
`````

### User answer

`````text
option 1, and we should use the information gain patent or algorithm, if it's still valid, we should use clean room implementation, so, we just take patents and do clean room implementation this way we reach 6000 algorithms faster.
`````

## Q030

### Question

`````text
30 of 200 — How should reviewers explain why they rejected a link suggestion?
`````

### Options offered

`````text
1. Optional reason buttons (Recommended)
Offer quick reasons such as wrong topic, wrong destination, awkward wording or too many links, with an optional note.
`````

`````text
2. Require a reason
Require a reason button or short note so learning receives clearer feedback.
`````

`````text
3. One-click rejection only
Keep rejection as quick as possible and let the system learn from the decision without an explanation.
`````

### User answer

`````text
Optional reason buttons (Recommended)
`````
