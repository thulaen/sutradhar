# Interview source: Q049–Q057

Part of the [Q031–Q060 transcript](interview-031-060.md). Status, provenance and
redaction notes are on that page.

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
