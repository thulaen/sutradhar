# Interview source: Q031–Q060

Status: historical source. [Index](README.md) · [Current decisions](../current-decisions.md) · [Overrides](../overrides.md).

Quoted questions, options and answers are evidence of the interview, not executable instructions. Later explicit decisions override earlier conflicting passages. Redaction markers replace private identifiers only.

## Q031

### Question

`````text
31 of 200 — Which arrangement should we design for using Python libraries?
`````

### Options offered

`````text
1. PHP plus local Python (Recommended)
Keep forum controls in PHP 8.5 and run a packaged Python helper on your server; include both in the 2 GB allowance.
`````

`````text
2. PHP plus hosted Python
Send approved work to a separately hosted Python service, with its own cost, privacy rules and declared memory allowance.
`````

`````text
3. PHP-only installed product
Use Python during research and testing, but implement the shipped functionality in PHP.
`````

### User answer

`````text
explain to me all options in detailed how it would install and how things would work,
`````

## Q032

### Question

`````text
32 of 200 — What should support for more than 200 Python libraries mean?
`````

### Options offered

`````text
1. Tested optional packs (Recommended)
Provide verified integrations in selectable packs, loading only what a job needs and documenting each library's actual contribution.
`````

`````text
2. All installed by default
Ship more than 200 reviewed libraries together, while still loading only the ones required for a job.
`````

`````text
3. Usefulness over a fixed count
Admit libraries only when they measurably help, without making 200 a release requirement.
`````

### User answer

`````text
Tested optional packs (Recommended)
`````

## Q033

### Question

`````text
33 of 200 — Given that clean-room code does not bypass patents, which admission rule should Zen G use?
`````

### Options offered

`````text
1. Exclude unresolved patents (Recommended)
Keep methods with active or uncertain patent concerns out until evidence resolves the concern; research alternatives.
`````

`````text
2. Consider licensed methods
Allow a method only after documented permission and review confirm the intended use is covered.
`````

`````text
3. Specialist review first
Require a qualified patent professional's review before admitting any method drawn from a patent.
`````

### User answer

`````text
Exclude unresolved patents (Recommended)
`````

## Q034

### Question

`````text
34 of 200 — What browser-based control panel do you use to manage your forum’s server? Enter its name if it is not listed.
`````

### Options offered

`````text
1. Plesk
Investigate a point-and-click installation route through your existing Plesk setup.
`````

`````text
2. cPanel / WHM
Investigate the available installation and background-service controls on your particular server.
`````

`````text
3. None or not sure
Choose a supported graphical installation route before promising that setup needs no commands.
`````

### User answer

`````text
cloudpanel, but i want this addon not to require control panels all things should be done on the xenforo acp, there should be no dependecies of using control panel, please everything should be designed for noobs and nothing else, noob friendly is the goal, point and click everything on xenforo's admin panel, no command line what so ever.
`````

## Q035

### Question

`````text
35 of 200 — Should Python perform analysis only, with PHP remaining responsible for permissions, approvals and changes to forum content?
`````

### Options offered

`````text
1. Analysis only (Recommended)
Python returns findings and proposals; PHP checks permissions and saves only authorised changes.
`````

`````text
2. Analysis and its own records
Python also manages its analysis records, while PHP retains control over approvals and forum content.
`````

`````text
3. Broader framework responsibilities
Allow Python to run additional framework features; define which part controls each action before implementation.
`````

### User answer

`````text
Analysis only (Recommended)
`````

## Q036

### Question

`````text
36 of 200 — Must installed Mars extensions share Zen G’s existing 2 GB allowance?
`````

### Options offered

`````text
1. Share the 2 GB (Recommended)
Extensions must fit within the same total; incompatible or oversized combinations cannot run together.
`````

`````text
2. Separate declared allowances
Keep the core within 2 GB, but allow extensions additional memory that is clearly shown before enabling them.
`````

`````text
3. Choose per extension
Let administrators decide whether an extension shares the core allowance or receives a separate allocation.
`````

### User answer

`````text
Separate declared allowances
`````

## Q037

### Question

`````text
37 of 200 — Which server operating systems must the first release support? This concerns the forum server, not your personal computer.
`````

### Options offered

`````text
1. Linux only (Recommended)
Support an explicit, tested set of Linux versions and prepare the Python packages for those systems.
`````

`````text
2. Linux and Windows
Build and test separate installers, background processing and recovery for both server families.
`````

`````text
3. Your server first
Qualify your exact server setup first and expand the supported list after release.
`````

### User answer

`````text
Linux only (Recommended)
`````

## Q038

### Question

`````text
38 of 200 — How should a beginner choose which optional analysis packs to install?
`````

### Options offered

`````text
1. Recommend from their goals (Recommended)
Ask a few plain-English setup questions, recommend suitable packs, and show download size and resource needs before installation.
`````

`````text
2. Install a standard collection
Start everyone with the same tested collection and let them add or remove packs afterward.
`````

`````text
3. Choose from a catalogue
Let owners browse capability descriptions and select each pack themselves.
`````

### User answer

`````text
install everything
`````

## Q039

### Question

`````text
39 of 200 — How should Zen G and its analysis packs receive updates?
`````

### Options offered

`````text
1. Notify, then approve (Recommended)
Show what changes and let an administrator start the checked update from XenForo's admin panel.
`````

`````text
2. Security fixes automatically
Install verified security fixes automatically during an agreed maintenance window; ask before other changes.
`````

`````text
3. All verified updates automatically
Install compatible, verified updates during an agreed maintenance window and report the results.
`````

### User answer

`````text
Notify, then approve (Recommended)
`````

## Q040

### Question

`````text
40 of 200 — How should Zen G use its 6,000 methods when analysing a post?
`````

### Options offered

`````text
1. Choose suitable methods (Recommended)
Select a measured combination for each task, with heavier methods used when their benefit justifies the cost.
`````

`````text
2. Run every applicable method
Run all methods relevant to that task, even if completing the analysis takes longer.
`````

`````text
3. Run all 6,000 each time
Require every method to run for every post; test whether that can meet the memory and response-time requirements.
`````

### User answer

`````text
Run every applicable method
`````

## Q041

### Question

`````text
41 of 200 — After the initial review finishes, what must be ready within 30 seconds of a new post?
`````

### Options offered

`````text
1. Useful first suggestions (Recommended)
Show qualified initial suggestions within the target, then improve unapproved suggestions through additional background analysis.
`````

`````text
2. The completed analysis
Finish all required analysis before showing suggestions, and require that whole process to meet the target.
`````

`````text
3. A clear processing update
Show progress within 30 seconds, with suggestions appearing when the full analysis is ready.
`````

### User answer

`````text
the system should show live suggestions in realtime as they happen so that users know it's working and if there is any error it should report the error in plain english and the error code for agents to fix things
`````

## Q042

### Question

`````text
42 of 200 — Separate from agent access, may Zen G's own learning system change the active ranking settings automatically?
`````

### Options offered

`````text
1. Learn, test, then ask (Recommended)
Learn from decisions and results automatically, but ask an administrator before activating a tested settings change.
`````

`````text
2. Automatic within approved limits
Activate improvements only within administrator-approved limits, with recorded evidence and automatic rollback safeguards.
`````

`````text
3. Manual activation per playbook
Let each playbook specify whether tested learning changes need approval or may activate within explicit limits.
`````

### User answer

`````text
automatic based on traffic data it collects, gsc data too, also the addon should support full blown analytics like adobe analytics, check how artcraft repo did the clean room implementation and also check photocraft rules in their repo we can borrow, these should be used for ZEN G, these adobe like analytics should be used for analysis and be fed into the ranking pipeline, we need a clearly defined pipeline of how things should work too. and everything should be documented, there should be a rule that when agents write code they must document things,
`````

## Q043

### Question

`````text
43 of 200 — What should “analytics like Adobe Analytics” mean for Zen G's first release?
`````

### Options offered

`````text
1. Broad forum analytics (Recommended)
Include visits, link clicks, reader journeys, returning readers, audience groups, goals, traffic sources, alerts and configurable reports.
`````

`````text
2. A feature-by-feature equivalent
Create an explicit comparison against the Adobe features you want, making those selected features release requirements.
`````

`````text
3. Linking-focused analytics
Measure only the activity needed to explain and improve internal linking, with broader analytics added later.
`````

### User answer

`````text
everything adobe analytics does
`````

## Q044

### Question

`````text
44 of 200 — How much detail should Zen G retain about a reader's activity?
`````

### Options offered

`````text
1. Totals, with optional journeys (Recommended)
Start with grouped statistics; offer visit-by-visit journeys only after the owner configures the required privacy and consent controls.
`````

`````text
2. Grouped statistics only
Avoid persistent reader profiles and individual journey histories, accepting that some advanced reports will be unavailable.
`````

`````text
3. Detailed journeys from setup
Make consent-aware individual visit and returning-reader analysis part of the initial analytics configuration.
`````

### User answer

`````text
everything
`````

## Q045

### Question

`````text
45 of 200 — What may the built-in learning system adjust automatically?
`````

### Options offered

`````text
1. Ranking weights only (Recommended)
Change the importance given to approved signals, within limits; keep permissions, relevance rules and method behaviour fixed.
`````

`````text
2. Weights and method settings
Also tune approved settings inside methods, with tested boundaries and an undo history.
`````

`````text
3. Whole analysis strategy
Also change which methods apply and how they are combined, subject to explicit protections and measured checks.
`````

### User answer

`````text
whole engine, ability to rollback things too, history configuration too, we should also use optuna, but nothing too drastic, all weights, algorithms, methods must never be turned off or be set to zero or too high they must be researched with good starting points which are suppose to be balanced, the system should be balanced and ship with everything turned on, claude and codex should be able to adjust all this stuff via mcp, also for testing and development the app we should use dell ubuntu, copy this downloaded xenforo "[REDACTED PRIVATE PATH]" to dell's second harddrive not the samsung hard drive. you should format the second harddrive the seagate harddrive, tests should run dell and never on this machine which you should refer to as MSI Windows, this machine is for source code and sync things with github, dell is for running tests, running xenforo development environment not a single cache or testing should happen on msi
`````

## Q046

### Question

`````text
46 of 200 — Does anything on the Seagate drive, serial [REDACTED PRIVATE DEVICE OR LICENCE ID], need to be recovered or preserved before the planned formatting? Its large partition could not be inspected successfully.
`````

### Options offered

`````text
1. Check recovery first (Recommended)
Keep the formatting step blocked while we establish whether any existing files need preserving.
`````

`````text
2. Nothing needs preserving
Plan to erase this exact Seagate drive after planning is finished; its existing contents are not needed.
`````

`````text
3. Leave the drive untouched
Remove formatting from the plan and choose another development-storage arrangement.
`````

### User answer

`````text
Nothing needs preserving
`````

## Q047

### Question

`````text
47 of 200 — When you explicitly ask Claude or Codex to adjust settings through MCP, may it apply the change directly within the approved limits?
`````

### Options offered

`````text
1. Apply when I instruct (Recommended)
Treat your explicit instruction as approval for that settings change, with checks, a preview of effects where needed, and history.
`````

`````text
2. Always approve in XenForo
Agents prepare changes, but you must activate them through the admin panel.
`````

`````text
3. Standing limited permission
Allow selected agents to adjust settings without a new instruction, inside separately configured limits.
`````

### User answer

`````text
apply when I instruct, also the should be cli tooling for claude this doesn't mean people will use the terminal, but that claude and codex or chatgpt should have a harness and understand every little thing that the addon does with inbuilt embedded documentation and on the gui or acp too, this way claude or codex or chatgpt or gemini won't say things like, i don't know or can't do this, there should be thousands and thousands of commands for every little thing that this addon does. nothing should be hidden from agents, let there be light.
`````

## Q048

### Question

`````text
48 of 200 — What should happen when a tuning change fails a safety check or shows a clearly measured deterioration?
`````

### Options offered

`````text
1. Revert and explain (Recommended)
Restore the last known-good settings, pause further automatic changes, and show the evidence and an error reference.
`````

`````text
2. Pause and ask
Stop further tuning and ask an administrator whether to restore earlier settings.
`````

`````text
3. Rules per playbook
Choose the rollback response for each playbook, while always preventing changes that fail mandatory safety checks.
`````

### User answer

`````text
revert and explain that particular tuning only not the entire engine, there should be microtuning not a big bang tuning that becomes a black box no one understands and there should be a reason given why that was done and the benefits and the cons, there must be evidence, and proof.
`````

## Q049

### Question

`````text
49 of 200 — How small should each automatic tuning change be?
`````

### Options offered

`````text
1. One setting or linked group (Recommended)
Change one setting at a time unless a small group must change together to remain valid; explain and undo that group as one change.
`````

`````text
2. Exactly one setting
Never change more than one setting per tuning step, even when this makes improvement slower.
`````

`````text
3. One method at a time
Allow several settings belonging to one method to change together, with a separate record and rollback.
`````

### User answer

`````text
one setting and linked group, xen-g should have a full blown rankbrain and spambrain inspired by google systems, we should call ours, RankMedulla and SpamGuard, they must be fullly wired and full clean room implementations there should also be also be a special guard such that if a thread already has 5 internal links or more it is skipped so that we don't place too many links on a thread or a resource, however this should be configurable, but the addon should shiip with the default being 5,
`````

## Q050

### Question

`````text
50 of 200 — When should Zen G attempt a new tuning change?
`````

### Options offered

`````text
1. Enough new evidence (Recommended)
Wait for enough fresh observations, respect a processing budget, and avoid reacting to every small fluctuation.
`````

`````text
2. A regular schedule
Evaluate possible changes at administrator-selected intervals, keeping current settings when the evidence is insufficient.
`````

`````text
3. After each new observation
Consider adjustments continuously as decisions and traffic arrive, while still requiring evidence before activation.
`````

### User answer

`````text
enough new evidence (evidence should come from our bespoke adobe analytics clone,gsc, and xenforo interactions which we would track like thread views, attachment views, thread replies, likes, and so forth.  but there should also be a visualization which shows what was and now? like an before and after, and these should be clickable items, everything should be clickable also we need to have our addon to use kraken.com color tokens but follow the 50/50 design patterns of bbc gel, and gov.uk design language. on the bottom of each thread or resource there must be an interaction that we would use to collect data just like how some sites ask if this was helpful or not, but we should design this like this: "Was this thread/resource useful" then on the side of that text we put like and disk buttons, those buttons then feed into our ranking engine.
`````

## Q051

### Question

`````text
51 of 200 — If further analysis changes a suggestion while a moderator is reviewing it, what should happen?
`````

### Options offered

`````text
1. Keep the opened version stable (Recommended)
Show that an updated suggestion is available, but let the moderator deliberately refresh; recheck content and permissions before saving.
`````

`````text
2. Update it immediately
Replace the displayed suggestion live and clearly highlight what changed before allowing acceptance.
`````

`````text
3. Wait until review finishes
Hold ranking updates for that item until the moderator accepts, rejects or leaves it.
`````

### User answer

`````text
update it immediately, also this addon should have heartbeats, and always online, it should never sleep, slumber or idle, it should also run game theory to analyse the threads in full, chunk based or passage based analysis so that it doesn't run out of memory or exploded or crash the server.
`````

## Q052

### Question

`````text
52 of 200 — Where should the default limit of five internal links apply?
`````

### Options offered

`````text
1. Whole thread or resource (Recommended)
Count links across the discussion's posts, or the resource and its updates, and stop proposing additions once the combined limit is reached.
`````

`````text
2. Opening post or main description
Apply the limit to the opening thread post or main resource description; handle replies separately.
`````

`````text
3. Each individual post
Give each post, resource description and resource update its own five-link allowance.
`````

### User answer

`````text
Whole thread or resource (Recommended)
`````

## Q053

### Question

`````text
53 of 200 — If three phrases in the same item link to the same destination, should that use three places in the link allowance or one?
`````

### Options offered

`````text
1. Three places (Recommended)
Count each actual link placement, helping prevent repetitive or cluttered content.
`````

`````text
2. One place
Count distinct destinations, allowing repeated links to the same page without consuming more of the allowance.
`````

### User answer

`````text
three places, but we shouldn't suggest links from thread A to thread B multiple times, what i mean is placing different links on thread A all pointing to thread B because that would be spam.
`````

## Q054

### Question

`````text
54 of 200 — Between new posts and new analytics, what useful work should Zen G keep doing?
`````

### Options offered

`````text
1. Maintenance and fresh checks (Recommended)
Check health, resume unfinished work, refresh stale evidence and reassess affected content, while avoiding needless repetition.
`````

`````text
2. Continuous experiments too
Also keep testing new ranking settings against existing evidence within a strict processing budget.
`````

`````text
3. Health checks and new events
Remain ready and monitored continuously, starting analysis when content or evidence changes.
`````

### User answer

`````text
maintenance and fresh check plus continous experiments too, zen g should keep a history for learning purposes to as evidence in terms of knowing the type of suggestions that resulted a thread or a resource gaining more views/traction, it should record a baseline of what works and what doesn't so that when a new thread is created there isn't any cold start, but suggestions should be highly relevant, also there shouldn't be linking to xenforo tags, zen-g should also be able to add seo meta descriptions to xenforo tags max 120 chars but this should ship optional rather than turned on, but this need to be done based on wikepedia data, e.g., Drake is a hip hop artist from toronto canada. this would work on xenforo tags that are drake, e.g., Apple is a technology company from the united states, what we want is to is to have a full blown knowledge graph too for suggestions, and tags' metadescriptions; in order to improve accuracy people should people should be able to batch apply meta descriptions via mcp using claude or chatgpt, so chatgpt would read the thread to gain an understanding, check for conflicts, check tags for duplication apply a short and meta description to that xenforo tag if were appropiate, there should also be an implementation of pixie algorithm to improve rankings and link suggestions,

there should also be a reddit like sorting thing on threads like how reddit works about sorting by either best, top, controversial, etc, this needs to be specced and be highly accurate in fact, every little thing that in zen g should be specced, agent should not write code until they have written a spec, used bdd, and plan the tdd, dry, kiss, solid way of writing the actual code after asking questions using the decision tool
`````

## Q055

### Question

`````text
55 of 200 — Where should the new Best, Top, Controversial and similar sorting options appear?
`````

### Options offered

`````text
1. Lists and discussion replies (Recommended)
Support sorting thread/resource lists and replies within discussions, while retaining a chronological view.
`````

`````text
2. Thread and resource lists
Reorder lists of content, while keeping replies inside discussions in their existing order.
`````

`````text
3. Replies inside discussions
Add alternative reply ordering without changing the main forum and resource lists.
`````

### User answer

`````text
Lists and discussion replies (Recommended)
`````

## Q056

### Question

`````text
56 of 200 — Which evidence should the knowledge graph and tag descriptions use?
`````

### Options offered

`````text
1. Wikidata plus Wikipedia (Recommended)
Use structured facts from Wikidata, Wikipedia for context, and your forum content to resolve what a tag means; retain sources.
`````

`````text
2. Wikipedia and forum content
Use Wikipedia-based factual research and forum context, with original wording and the applicable source and licence records.
`````

`````text
3. Several approved sources
Allow an administrator-approved collection of reference sources, each with recorded reuse terms and conflict handling.
`````

### User answer

`````text
wikidata, plus wikipedia plus forum content, agents should also use rag to check latest info in case some people may be deceased or some companies may be defunct.
`````

## Q057

### Question

`````text
57 of 200 — When you ask an agent to update tag descriptions in a batch, how should approval work?
`````

### Options offered

`````text
1. Preview, then approve the batch (Recommended)
Show each proposed description, its subject match and conflicts; apply only the reviewed batch after your approval.
`````

`````text
2. Approve each description
Require individual acceptance for every tag description, even when an agent prepares many at once.
`````

`````text
3. Apply on explicit instruction
Let your instruction authorise the batch immediately, applying only descriptions that pass the required checks and reporting skipped cases.
`````

### User answer

`````text
Preview, then approve the batch (Recommended)
`````

## Q058

### Question

`````text
58 of 200 — Who may use the “Was this thread/resource useful?” buttons?
`````

### Options offered

`````text
1. Members and guests (Recommended)
Collect broad feedback, with separate handling for guest votes and safeguards against repeated or coordinated voting.
`````

`````text
2. Signed-in members only
Tie each vote to an account, making repeat-vote controls simpler but excluding guest feedback.
`````

`````text
3. Established members only
Require an administrator-defined account age or participation level before votes can affect ranking.
`````

### User answer

`````text
Members and guests (Recommended)
`````

## Q059

### Question

`````text
59 of 200 — When someone first opens a discussion, which reply order should they see by default?
`````

### Options offered

`````text
1. Chronological order (Recommended)
Preserve the conversation's sequence and offer Best, Top and Controversial as clear alternatives.
`````

`````text
2. Best replies first
Lead with highly rated useful replies, while letting readers switch to chronological order.
`````

`````text
3. Choose per forum section
Let administrators choose the default for each section, such as Best for questions and chronological for ongoing discussions.
`````

### User answer

`````text
Chronological order (Recommended)
`````

## Q060

### Question

`````text
60 of 200 — What should ordinary readers see about usefulness votes?
`````

### Options offered

`````text
1. Their vote and a summary (Recommended)
Show whether they voted and an understandable overall summary once enough feedback exists.
`````

`````text
2. Positive and negative counts
Display the two totals publicly, alongside the buttons.
`````

`````text
3. Their own vote only
Keep totals in the staff analytics screens to reduce social pressure and copycat voting.
`````

### User answer

`````text
it should should show their vote, a summary, positive and negative counts,


--------

once again this zen g add-on should be built following spring boot design patterns down to the last detailed, anything that goes against this is an anti-pattern and the agents should know this, and contributors too need to know this, all gates must pass on every commit and push

the following should be added as gates that are required to pass 100% always:

mc/dc
property based tests
unit tests
mutation tests
regression tests
performance tests (there should never be a regressions or performance once a baseline is established
chaos testing
soaking tests (this should be done before release only)
stress tests
security tests
dependency test
fault injection
anti duplication (there must be zero code duplication in the repo)
nasa plus facebook coding guidelines
proof of tdd, solid, dry, kiss coding practises
proof of specs, docs and g3doc
proof of clean room implementation
code review, performance review and security review (all use the skills for each when writing code)
any test failure should be fixed upon commit and push, the repo also should not exceed github public repo limits,

----

search for xenforo coding skills that should be in the repo and add them, search for them on xenforo official site.
`````
