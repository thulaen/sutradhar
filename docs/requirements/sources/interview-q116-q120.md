# Interview source: Q116–Q120

Part of the [Q091–Q120 transcript](interview-091-120.md). Status, provenance and
redaction notes are on that page.

## Q116

### Question

`````text
116. If Zen G suspects coordinated voting by several accounts, what may it do before a moderator investigates?
`````

### Options offered

`````text
1. Flag it without changing votes (Recommended)
Show the evidence and affected scores; require moderator approval before excluding those votes from ranking or karma.
`````

`````text
2. Temporarily exclude suspect votes
Apply a reversible, logged exclusion from Zen G scores while awaiting review; preserve the original XenForo reactions.
`````

### User answer

`````text
option 1, also i change the repo name to Sutradhar, so going forward it should be named Sutradhar.

----

## Naming
Indian names are for the parts of the system. English is for the code inside them.

Use glossary names for:
- packages and subpackages (`sutradhar.vinyas`, `sutradhar.sanchar`)
- feature modules: their pip package, registered name, config section, and router prefix (`sutradhar-setu`, `sutradhar.modules.setu`, `/modules/setu/`)
- the class that defines each feature module (`class Setu(Module)`)
- CLI command groups (`sutra mandali list`, `sutra karya run`)

Use plain English for everything else:
- functions, methods, variables, and parameters (`suggest_links`, `max_links_per_post`)
- domain classes (`Suggestion`, `Thread`, `Post`, `PostSaved`)
- config keys inside a section, log messages, error messages, metric names, database tables and columns, and HTTP paths outside module routers
- anything defined by a library or standard (FastAPI, SQLAlchemy, HTTP, Prometheus)

Rules:
- Only use names from the glossary. If you need a new one, use an English placeholder, and propose an Indian name with its meaning in your summary. Never invent one in code.
- Spell names exactly as in the glossary: lowercase ASCII, no diacritics.
- The docstring or help text of each Indian-named package, module class, and CLI group starts with its meaning, e.g. `"""Setu (bridge): internal link suggestions."""`
- Don't propose names of deities, religious texts, political parties, or government programs.

### Glossary
| Concept | Name | Meaning |
|---|---|---|
| Framework (package `sutradhar`, CLI `sutra`) | sutradhar | holder of the threads |
| Configuration and profiles | vinyas | arrangement |
| Module registry and lifecycle | mandali | troupe |
| Dependency injection | sanyojan | coordination |
| Events | sanchar | communication |
| Scheduled and batch jobs | karya | work |
| Database and repositories | bhandar | storehouse |
| Suggestions and review | sujhav | suggestion |
| Audit log | lekha | record |
| Health checks, metrics, actuator | drishti | sight |
| Auth and request signing | dwarpal | gatekeeper |
| Testing kit | pariksha | test |
| XenForo integration | sampark | connection |
| BBCode text extraction | shodhan | refining |
| Example module | namaste | hello |
| Internal linking | setu | bridge |
| SEO signals | sanket | signal |
| AutoModerator | panchayat | village council |
| Hot/Best ranking | shikhar | peak |
| Karma | karma | deeds |
| Spam protection | kavach | armor |
| Ask the forum | uttar | answer |
`````

## Q117

### Question

`````text
117. If a previously approved method develops a serious fault, should Zen G isolate that method or pause the entire analysis engine?
`````

### Options offered

`````text
1. Isolate and report the fault (Recommended)
Keep its settings and positive weight, stop executing the faulty method, and visibly withhold results that require it until repaired.
`````

`````text
2. Pause the whole analysis engine
Stop all new analysis and approvals until the fault is repaired and the complete required catalogue can run safely again.
`````

### User answer

`````text
isolate and report the fault in plain english, also there should be an error code, there should be a health/diagnostics page for things that have problems or issues
`````

## Q118

### Question

`````text
118. If an owner has not connected Search Console, Bing or a paid backlink service, should Sutradhar still run its other checks?
`````

### Options offered

`````text
1. Run what has evidence (Recommended)
Ship the requested capabilities, but clearly mark disconnected, limited or unavailable sources; never report missing data as zero.
`````

`````text
2. Require every selected source first
Let owners select their required sources during setup, then hold the affected analysis until all selected connections work.
`````

### User answer

`````text
run what has evidence this should in gsc, bing api, ga4; remove the need for paid backlink service we should focus on gsc, bing, and ga4 as external connectors and our bespoke adobe analytics module
`````

## Q119

### Question

`````text
119. When a health check suggests changing a title, redirect, canonical address or search-indexing instruction, who should approve the actual change?
`````

### Options offered

`````text
1. An administrator approves each change (Recommended)
Show the affected pages, proposed change and expected consequences before applying it.
`````

`````text
2. Allow administrator-approved rules
Administrators can authorise specific automatic fixes in advance; other changes still need individual approval.
`````

### User answer

`````text
remove the need to change titles.
`````

## Q120

### Question

`````text
120. How far may the built-in crawler go outside your own site?
`````

### Options offered

`````text
1. Check only referenced external links (Recommended)
Audit the forum and inspect its outbound destinations with limits; do not explore unrelated pages on those external sites.
`````

`````text
2. Also crawl approved outside sites
Allow separate, explicitly configured competitor or reference-site crawls with their own permissions, limits and schedules.
`````

### User answer

`````text
Check only referenced external links (Recommended)
`````
