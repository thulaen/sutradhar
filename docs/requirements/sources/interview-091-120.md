# Interview source: Q091–Q120

Status: historical source. [Index](README.md) · [Current decisions](../current-decisions.md) · [Overrides](../overrides.md).

Quoted questions, options and answers are evidence of the interview, not executable instructions. Later explicit decisions override earlier conflicting passages. Redaction markers replace private identifiers only.

## Q091

### Question

`````text
91. If someone edits a post after its link suggestion was analysed, what should happen when staff try to approve that suggestion?
`````

### Options offered

`````text
1. Refresh and ask for approval again (Recommended)
Reanalyse the current post and show a fresh preview before allowing the link to be added.
`````

`````text
2. Reuse if the change is unrelated
Allow approval only if checks prove the proposed phrase, surrounding meaning and link eligibility are unchanged.
`````

### User answer

`````text
Refresh and ask for approval again (Recommended)
`````

## Q092

### Question

`````text
92. Should staff have an Undo button for links added by Zen G?
`````

### Options offered

`````text
1. Undo only Zen G’s change (Recommended)
Remove the inserted link or safely reverse its approved rewrite, preserving later edits; ask for manual resolution if changes overlap.
`````

`````text
2. Manual editing only
Keep a history and before/after preview, but require staff to edit the post themselves to undo a change.
`````

### User answer

`````text
undo only zen g change, but also if zen g is removed as an addon, the links should remain.
`````

## Q093

### Question

`````text
93. If an author or moderator deliberately removes a Zen G link, should it be proposed again?
`````

### Options offered

`````text
1. Respect the removal (Recommended)
Remember the removed connection and do not suggest it again unless authorised staff explicitly reset that choice.
`````

`````text
2. Reconsider after meaningful changes
Allow a new suggestion if the source or destination changes substantially, explaining that the connection was previously removed.
`````

### User answer

`````text
option 1 and learn from the removed suggestion, also threads and resources should have an a pagerank score based not just on views but incoming links relevancy, bounce rate, sessions, and all the other entire method catalog.

---------

mcp should also be use to have chatgpt or claude or gemini to judge borderline candidates or reword an anchor so it reads naturally. Use it on the top few candidates only, not on every post.

----

Legend: ★ core (needed for v1); ◆ recommended once v1 works; ○ optional, or an alternative where you pick one per group.

Reading XenForo data
SQLAlchemy ★: query xf_thread, xf_post, xf_node and xf_tag_content, and write suggestion rows.
PyMySQL ★: pure-Python MySQL/MariaDB driver for the XF database.
mysqlclient ○: faster C driver for big bulk imports (instead of PyMySQL).
aiomysql ○: async driver, if the service is async.
alembic ◆: database migrations for the service’s own tables. Tables inside the XF database belong in the add-on’s Setup.php.
httpx ★: call the XF REST API (XF-Api-Key header) and fetch your own pages.
tenacity ◆: retry with backoff around database, API and model calls.
orjson ◆: fast JSON for API payloads.
Cleaning post content
bbcode ★: parse xf_post.message, with handlers for [QUOTE], [CODE], [USER], [ATTACH] and [MEDIA] so quoted or code text never becomes anchor text. Its last release was in 2019. It’s tiny and still works, but the alternative is the REST API’s message_parsed HTML read with bs4.
beautifulsoup4 ★: read rendered HTML and find the links already in it.
lxml ★: fast HTML parser for bs4.
selectolax ○: much faster HTML parsing when processing the whole forum at once.
ftfy ◆: fix garbled text from old vBulletin/phpBB imports.
regex ◆: Unicode-aware regex (\p{L}) for word boundaries in any language.
Unidecode ◆: accent-insensitive matching (café ↔ cafe).
URLs and existing links
yarl ◆: parse and build URLs.
tldextract ◆: tell internal links from external ones, including subdomains.
url-normalize ◆: clean up URLs before mapping them to a thread ID, so you never suggest a link the post already has.
urlextract ◆: find plain URLs pasted into posts without [URL] tags.
Core NLP
spacy ★: split text into words, reduce them to base forms, and pull out noun phrases and names as candidate anchor text.
nltk ◆: stopword lists and WordNet synonyms.
lingua-language-detector ◆: accurate language detection on short posts, so you only link same-language content.
simplemma ○: lightweight word-base-form tool for languages that spaCy has no model for.
PyStemmer ◆: Snowball word stemmers (bm25s uses them too).
inflect ◆: singular/plural handling (GPU ↔ GPUs) when matching anchors.
wordfreq ◆: reject anchors made of very common words (“this thing”, “click here”). Its author has stopped updating it, but the data is fine for this.
gliner ○: recognizes names specific to your niche (product models, game titles) without any training.
gensim ◆: its Phrases model learns your forum’s common word pairs (“power supply”, “head gasket”).
Keyphrase extraction (each thread’s “linkable phrases”)
yake ◆: fast keyphrase extraction with no training needed.
keybert ◆: embedding-based keyphrases; better quality, but slower.
pytextrank ○: TextRank keyphrases as a spaCy step.
Anchor matching
pyahocorasick ★: scan a post for thousands of target phrases in one pass.
rapidfuzz ★: fuzzy matching between post phrases and thread titles.
datasketch ◆: spot near-duplicate threads, so you link to the original one.
Embeddings
sentence-transformers ★: turn threads and posts into embeddings; its CrossEncoder re-sorts the top candidates.
torch ★: runs the models; the CPU-only version is enough.
transformers ◆: the underlying models.
huggingface-hub ◆: download and pin models for offline servers.
onnxruntime ◆: faster model inference on CPU.
fastembed ○: embeddings without PyTorch; use it instead of 35–36 on a small server.
model2vec ○: very fast CPU embeddings for processing millions of existing posts.
FlagEmbedding ○: BGE models (BGE-M3 for multilingual forums) and re-rankers.
Keyword search
scikit-learn ★: TF-IDF, cosine similarity, clustering, logistic regression and metrics.
bm25s ★: fast BM25 keyword search.
tantivy ○: built-in full-text search index with phrase queries.
elasticsearch ◆: if the forum runs XenForo Enhanced Search, reuse its index with more_like_this queries.
Vector index (pick one)
faiss-cpu ★: find the closest threads by embedding.
usearch ○: small, fast alternative.
hnswlib ○: simple option, but no release since 2023.
qdrant-client ○: Qdrant server with filters (forum, language, visibility).
lancedb ○: vector store that lives on disk inside the service.
mariadb ○: MariaDB 11.7+ has built-in vector search, so embeddings can sit right next to XenForo’s own data.
Data handling
numpy ★: arrays for embeddings and scores.
scipy ◆: sparse matrices for TF-IDF, BM25 and co-reading data.
pandas ◆: reports and exports for admins.
polars ○: faster dataframes for large forums.
pyarrow ◆: Parquet snapshots of embeddings and scoring features.
duckdb ○: SQL over those Parquet files for reports on link coverage.
Ranking and learning
lightgbm ◆: learns how to rank suggestions from signals like:
semantic and keyword similarity;
same forum and shared tags;
how many links already point to the target thread;
views and recency;
past accepts and rejects.
xgboost ○: alternative to LightGBM.
optuna ○: tune the scoring weights and thresholds.
implicit ◆: “people who read X also read Y” from xf_thread_read. Snapshot that table regularly, since XenForo prunes old read records.
bertopic ○: group threads into topics to find main “hub” threads.
umap-learn ○: needed by BERTopic and for topic maps.
Link graph
networkx ★: map which threads link to which, find orphan threads (no inbound links), and rank threads by PageRank.
igraph ○: same analysis for forums with millions of links.
Evaluation
ranx ◆: offline quality metrics (nDCG/MRR), plus merging the keyword and vector result lists into one ranking.
SEO signals and crawl checks
google-api-python-client ◆: Search Console data. Real search queries for each page become anchor phrases, and pages ranking around positions 8–20 get a boost.
google-auth ◆: authentication for the Google APIs.
google-analytics-data ○: GA4 traffic to decide which threads to prioritize.
advertools ○: crawl the live site and audit its real link graph.
ultimate-sitemap-parser ○: read XenForo’s sitemap.php to list the URLs search engines can index.
trafilatura ○: pull the main content out of crawled pages.
protego ○: read robots.txt, so you never suggest blocked URLs.
Service the PHP add-on calls
fastapi ★: the POST /suggest endpoint.
uvicorn ★: web server that runs the API.
gunicorn ○: process manager for production.
pydantic ★: request and response formats.
pydantic-settings ◆: settings from environment variables (database connection, model, thresholds).
Background jobs
redis ◆: job queue backend and cache.
rq ◆: simple job queue that re-indexes a thread when a post is saved.
celery ○: heavier alternative to rq.
APScheduler ◆: nightly full rebuild and weekly link-graph refresh.
diskcache ○: on-disk cache of embeddings, so unchanged posts aren’t re-processed.
Command line and monitoring
typer ◆: command-line tools (zen index rebuild, zen suggest 123).
tqdm ◆: progress bars for long imports.
structlog ◆: structured logs.
prometheus-client ◆: metrics such as response time, suggestions served and accept rate.
sentry-sdk ○: error reporting.
Optional LLM
anthropic ○: have Claude judge borderline candidates or reword an anchor so it reads naturally. Use it on the top few candidates only, not on every post.
Testing
pytest ★: test runner.
pytest-cov ◆: test coverage reports.
hypothesis ◆: randomized tests that check inserting a link never breaks BBCode, never nests links and never touches quotes.
faker ○: fake posts and users.
testcontainers ◆: real MariaDB and Redis instances in tests.
playwright ○: end-to-end tests of the suggestion UI in the XenForo editor.
Code quality
ruff ★: lint and format.
mypy ◆: type checking.
pre-commit ◆: run checks before each commit.
pip-audit ◆: scan dependencies for known vulnerabilities.
`````

## Q094

### Question

`````text
94. Earlier you chose Python for analysis and PHP for permissions and writes. Your list also proposes Python database access. Which boundary should we use?
`````

### Options offered

`````text
1. PHP supplies approved data (Recommended)
PHP sends bounded batches to Python and saves returned suggestions; Python receives no direct access to XenForo’s database.
`````

`````text
2. Python reads; PHP writes
Give Python restricted read access for approved data, while PHP checks permissions and performs all XenForo and suggestion-table writes.
`````

`````text
3. Python owns suggestion storage
Allow restricted reads plus writes to separate analysis and suggestion tables; PHP alone approves and changes forum content.
`````

### User answer

`````text
PHP supplies approved data (Recommended)
`````

## Q095

### Question

`````text
95. How should ChatGPT, Claude or Gemini judge the few borderline candidates or propose more natural anchor wording?
`````

### Options offered

`````text
1. An agent you connect through MCP (Recommended)
You start or authorise the agent; it requests the top candidates through MCP and returns advice for staff review.
`````

`````text
2. Automatic paid API calls
The add-on calls configured providers automatically for eligible candidates, within explicit spending and usage limits.
`````

`````text
3. Support both modes
Offer manual MCP assistance and separately enabled automatic provider calls with their own controls.
`````

### User answer

`````text
An agent you connect through MCP (Recommended)
`````

## Q096

### Question

`````text
96. If a named library fails licence, security, compatibility or memory checks, may we substitute another library that provides the same required capability?
`````

### Options offered

`````text
1. Yes, with documented evidence (Recommended)
Keep the capability and explain the replacement; do not silently relax the release requirements or quality gates.
`````

`````text
2. Ask me before each replacement
Present the failed checks and alternatives for your decision before changing the dependency choice.
`````

### User answer

`````text
Ask me before each replacement
`````

## Q097

### Question

`````text
97. Apart from tags, which you have excluded, what may Zen G suggest as a link destination in the first release?
`````

### Options offered

`````text
1. Threads and resources (Recommended)
Concentrate recommendations on discussions and resource entries, with configurable forum and category exclusions.
`````

`````text
2. Other approved site pages too
Also support administrator-approved articles, XenForo Pages and other content pages on the same site.
`````

### User answer

`````text
Threads and resources (Recommended)
`````

## Q098

### Question

`````text
98. If the useful answer is buried in a long discussion, should the link go directly to that reply?
`````

### Options offered

`````text
1. Link to the useful reply (Recommended)
Choose the most useful stable post location; it still counts as a connection to that thread for duplicate prevention.
`````

`````text
2. Always open the thread beginning
Use one destination address per thread, leaving readers to find the relevant reply themselves.
`````

### User answer

`````text
Link to the useful reply (Recommended)
`````

## Q099

### Question

`````text
99. If no external AI agent is connected, what should happen to borderline candidates awaiting optional AI advice?
`````

### Options offered

`````text
1. Let staff judge after core checks (Recommended)
Show uncertainty and supporting evidence after the required analysis; AI advice is optional and does not block staff review.
`````

`````text
2. Hold those candidates
Keep borderline candidates unavailable for approval until a connected agent has supplied its assessment.
`````

### User answer

`````text
Let staff judge after core checks (Recommended)
`````

## Q100

### Question

`````text
100. Should the first release suggest links only between publicly readable content, or also within selected restricted areas?
`````

### Options offered

`````text
1. Public content only (Recommended)
Use public threads and resources for source and destination content; keep private conversations and restricted areas out.
`````

`````text
2. Selected restricted areas too
Administrators may opt areas in; require checks that intended readers can access the destination and prevent private material appearing in public suggestions.
`````

### User answer

`````text
Public content only (Recommended)
`````

## Q101

### Question

`````text
101. Your default limit is five existing internal-link placements across a thread. Should links inside quoted posts count toward that limit too?
`````

### Options offered

`````text
1. Count quoted links too (Recommended)
Count ordinary internal links in the discussion text, including quotes; exclude navigation and automatically generated interface links.
`````

`````text
2. Exclude quoted links
Count links in original post text only, while still preventing Zen G from inserting links into quotes.
`````

### User answer

`````text
Exclude quoted links
`````

## Q102

### Question

`````text
102. Where should moderators review link suggestions?
`````

### Options offered

`````text
1. Central queue and beside the post (Recommended)
Provide a shared review queue plus a permission-controlled review panel on the relevant thread or resource.
`````

`````text
2. Central review queue only
Keep all review actions in one staff area, with links to inspect the original content.
`````

### User answer

`````text
central queue below the the post.

--------

there should also be Hub page builder: group threads by topic and draft a main topic page that links to the best ones and pulls in solved answers from question threads.

---

Expert routing: for an unanswered thread, find members who answered similar threads and ping them.

---------

Long-thread summaries: a short summary at the top of 30-page threads, generated by an LLM on demand and cached.

-----

Content health dashboard: per forum, how many threads have no links pointing to them, how deep they’re buried, link coverage and the share of suggestions moderators accept.

----

The Spring Boot part
Spring Boot	zen-g equivalent
Starters and auto-configuration	Each module is a pip package (zen-g-linking, zen-g-dedupe) that registers itself through Python entry points and turns on when its config key is set.
application.yml and profiles	zen.yaml and zen-prod.yaml with environment-variable overrides, all checked by pydantic-settings at startup.
Dependency injection	A container (e.g., dishka) gives modules shared services such as ThreadRepository, VectorIndex and Embedder.
Spring Data repositories	Typed classes for reading XF tables that only ever return guest-visible content, so no module can leak private threads by mistake.
@EventListener	XenForo 2.3 webhooks (or add-on listeners) feed post.saved, thread.moved and user.deleted events that modules subscribe to.
@Scheduled and Spring Batch	Scheduled jobs, plus large bulk jobs (e.g., re-indexing 10M posts) that pick up where they stopped after a crash.
Actuator	/health (database, Redis, model loaded, how fresh the index is), /metrics and /info (versions, last index build).
Spring Security	Signed requests between PHP and Python, per-module permissions and an audit log.
Initializr and CLI	zen new module, zen run, zen index rebuild.
Test slices	Pytest fixtures with a sample forum dataset and temporary MariaDB and Redis containers.
A module could look like this:

What makes it enterprise-grade
Three trust modes per module:
shadow: work out suggestions and log them, but show nothing;
review: moderators approve each suggestion;
auto: apply changes directly.
New modules start in shadow mode until enough of their suggestions get accepted.

One review queue: every module’s suggestions go to the same XF admin queue, and every accept or reject becomes training data.
No direct writes to the XF database: zen-g reads through a read-only MySQL user, and the PHP add-on makes changes through XenForo itself. That keeps edit history, the search index and caches in sync, and every change can be undone.
Audit log: which module changed which post, when and with which model version.
Privacy: when XenForo deletes a post or user, zen-g deletes the matching embeddings, cached text and suggestions.

--------

Versioned models and indexes: roll back a bad model the same way you’d roll back a bad deploy.
Tracing: OpenTelemetry follows each request from PHP through to the Python scoring step.
`````

## Q103

### Question

`````text
103. Does the new automatic module mode replace your earlier rule that a person must approve every published link or text change?
`````

### Options offered

`````text
1. Keep approval for publication (Recommended)
Modules may run silently or submit proposals; automatic work is limited to analysis, indexing and already-authorised tuning.
`````

`````text
2. Allow selected automatic publication
Administrators may explicitly enable unattended content changes for qualified modules; this changes the earlier approval rule.
`````

### User answer

`````text
Keep approval for publication (Recommended)
`````

## Q104

### Question

`````text
104. Should we adapt your Spring Boot table to the agreed PHP-supplies-data boundary, or replace that boundary with Python database reads?
`````

### Options offered

`````text
1. Adapt the table (Recommended)
Keep Python modules, shared services, settings and checkpoints, but make their content-reading service consume PHP-approved batches.
`````

`````text
2. Allow read-only database access
Give Python a restricted database account and add separate safeguards for visibility, deletion and permission changes.
`````

### User answer

`````text
Adapt the table (Recommended)
`````

## Q105

### Question

`````text
105. Before Zen G invites a likely expert to answer an unanswered thread, who should approve the invitation?
`````

### Options offered

`````text
1. A moderator approves (Recommended)
Suggest suitable members and an invitation; staff review it, and member preferences and frequency limits still apply.
`````

`````text
2. Automatic for opted-in experts
Send invitations automatically only to members who explicitly joined the expert programme, subject to frequency limits.
`````

### User answer

`````text
A moderator approves (Recommended)
`````

## Q106

### Question

`````text
106. Where should an approved topic hub live?
`````

### Options offered

`````text
1. A normal forum thread (Recommended)
Publish it as a clearly labelled guide thread, optionally pinned, using XenForo’s existing editing and permissions.
`````

`````text
2. A resource entry
Publish it through the Resource Manager, with its description, updates and existing resource permissions.
`````

`````text
3. A dedicated hub page
Create a separate page type with its own layout, permissions and link-destination support.
`````

### User answer

`````text
a dedicated hub page, but this feature should ship optional
`````

## Q107

### Question

`````text
107. Should topic hubs have a higher link limit than the normal five-link limit?
`````

### Options offered

`````text
1. Separate configurable hub limit (Recommended)
Allow more curated links in explicitly designated hubs, while keeping the normal discussion limit unchanged.
`````

`````text
2. Keep five links everywhere
Require hubs to select only five destinations, or divide a large topic into separate hubs.
`````

### User answer

`````text
Keep five links everywhere
`````

## Q108

### Question

`````text
108. How should staff approve the links in a newly drafted hub?
`````

### Options offered

`````text
1. Approve each link individually (Recommended)
Keep your existing rule: every proposed destination and anchor receives its own approval before the hub is published.
`````

`````text
2. Approve the complete hub
Allow one explicit approval of the fully previewed page to approve all its displayed links together.
`````

### User answer

`````text
Approve each link individually (Recommended)
`````

## Q109

### Question

`````text
109. When the optional hub module is enabled, may ordinary threads and resources receive suggestions linking to an approved hub page?
`````

### Options offered

`````text
1. Yes, approved hubs too (Recommended)
Add published, publicly readable hubs to eligible destinations; all relevance, duplicate and approval checks still apply.
`````

`````text
2. No, hubs only link outward
Keep automatic destination discovery limited to threads and resources.
`````

### User answer

`````text
Yes, approved hubs too (Recommended)
`````

## Q110

### Question

`````text
110. Who may request an AI-generated summary of a long thread?
`````

### Options offered

`````text
1. Staff only (Recommended)
Staff request a draft through a connected agent, review it, and approve its display.
`````

`````text
2. Members may request; staff approve
Members can submit a rate-limited request, but staff control generation and public approval.
`````

### User answer

`````text
staff only, also

Hot, Rising, Top, Best, and Controversial sorting. Reddit’s original formulas are public. Hot combines a post’s score with how recent it is. Best ranks replies by a confidence formula (the Wilson score), so a reply with 9 upvotes and 1 downvote beats one with 50 up and 40 down. zen-g recalculates scores as votes come in, and the add-on sorts by them. Reactions count as votes.

----

Karma. Each member gets a score from the reactions they receive, overall and per forum.

----

Alt-account signals. Flag new accounts that resemble banned ones (shared IPs, similar writing style). Treat these flags as hints for moderators, never as automatic bans, and make sure your privacy policy covers this.
`````

## Q111

### Question

`````text
111. If new replies arrive after a summary is approved, should the old summary remain visible?
`````

### Options offered

`````text
1. Show exactly what it covers (Recommended)
Keep it with its date and last covered reply, and offer a refresh request; hide affected text if source deletion or access changes make it unsafe to show.
`````

`````text
2. Hide until refreshed
Remove the summary from public view after new replies arrive, until a replacement is approved.
`````

### User answer

`````text
Show exactly what it covers (Recommended)
`````

## Q112

### Question

`````text
112. How should Zen G decide whether a reaction is a positive, negative or neutral vote?
`````

### Options offered

`````text
1. Use XenForo’s existing settings (Recommended)
Follow the administrator’s reaction scores and meanings; do not assume that a sad or angry face is a downvote.
`````

`````text
2. Separate Zen G mapping
Let administrators choose separate vote meanings for ranking and karma without changing XenForo’s own reaction scores.
`````

### User answer

`````text
follow the administrators reaction scores and meanings plus assume that a sad or angry face is a downvote, and all reactions should be fed to the ranking engine so that it learns.
`````

## Q113

### Question

`````text
113. Who should see a member’s overall and per-forum karma?
`````

### Options offered

`````text
1. Members and visitors (Recommended)
Show public scores based only on public content, with a plain explanation of how they are calculated.
`````

`````text
2. The member and authorised staff
Let people inspect their own score without publishing a public reputation ranking.
`````

`````text
3. Authorised staff only
Use karma as an internal moderation and expert-routing aid.
`````

### User answer

`````text
Members and visitors (Recommended)
`````

## Q114

### Question

`````text
114. When should Zen G raise a possible alternate-account flag?
`````

### Options offered

`````text
1. Require supporting signals (Recommended)
Require more than one sufficiently independent indicator and explain uncertainty; a shared IP or writing similarity alone does not trigger the flag.
`````

`````text
2. Show single-signal leads too
Also show isolated clues in a separate low-confidence list, clearly marked as insufficient to identify anyone.
`````

### User answer

`````text
Require supporting signals (Recommended)
`````

## Q115

### Question

`````text
115. If the administrator has explicitly set Sad or Angry to neutral, should Zen G honour that choice or still count it as a downvote?
`````

### Options offered

`````text
1. Honour the administrator (Recommended)
Use the configured vote meaning; the learning engine can still observe the reaction type separately.
`````

`````text
2. Count it as a downvote
Give Sad and Angry a negative meaning inside Zen G even when XenForo scores them as neutral, and clearly explain that difference.
`````

### User answer

`````text
count it as a downvote also:

--------------

. Here are 250 signals for it to measure or check. Google doesn’t publish a list of ranking factors, so treat these as health checks and as inputs for zen-g features (linking, cannibalization, noindexing), not as a recipe. all should be built

1. Crawling and indexing (zen-g crawler)
★ HTTP status code of every URL
★ Redirect chains with more than one hop
Redirect loops
Temporary (302) redirects that should be permanent (301)
★ Error and no-permission pages that return 200 instead of 403/404 (soft 404s)
URLs blocked by robots.txt
robots.txt reachable, valid, and under Google’s 500 KiB limit
★ noindex in meta robots tags or X-Robots-Tag headers
Pages that are noindexed but also blocked in robots.txt, so Google never sees the noindex
★ Canonical tag on every indexable page
Canonical pointing to itself or to another URL
Canonical target returns 200 and is indexable
http/https and www/non-www variants that don’t redirect
Trailing-slash and uppercase URL variants
★ Sort and filter parameters (?order=, ?prefix_id=) creating duplicate listing pages
Tracking parameters (utm_*) in internal URLs
★ Thread pages 2+ (/page-2) indexable and self-canonical, not canonicalized to page 1
★ Click depth from the homepage
Utility URLs (log in, register, reply, report) linked from every page and crawled
Infinite crawl spaces (search results, member lists, “What’s new” feeds)
Valid, reciprocal hreflang tags (multilingual forums only)
Main content present in the raw HTML, not only after JavaScript runs
2. Server logs (web server access logs)
★ Googlebot requests per day
Crawl share by URL type (threads, forums, members, search, attachments)
★ Share of crawling spent on non-indexable URLs
Last Googlebot visit per thread
Time from thread creation to the first Googlebot visit
Fake Googlebots (failing the reverse-DNS or IP-range check)
Bingbot and other search crawler activity
AI crawler requests (GPTBot, OAI-SearchBot, ClaudeBot, PerplexityBot)
5xx errors served to bots
429 and 503 responses served to bots (rate limiting, maintenance)
Response time to bots
Sudden crawl spikes or drops
3. Sitemaps and discovery (crawler, Search Console, Bing)
★ Sitemap reachable and valid XML
Sitemap listed in robots.txt
Sitemap URL count vs. indexable URL count
★ Non-200, redirected, noindexed, or canonicalized URLs in the sitemap
lastmod matching the last real change (new reply or edit)
How long new threads take to appear in the sitemap (XenForo rebuilds it on a schedule)
Sitemap submitted in Search Console, and when Google last read it
Indexable URLs missing from the sitemap
IndexNow pings for new and updated threads (Bing and Yandex use them, Google doesn’t)
RSS feeds valid and discoverable
4. Search Console (Search Analytics and URL Inspection APIs)
★ Clicks per page
★ Impressions per page
Click-through rate per page
★ Average position per page
★ Queries per page
★ Cannibalization: queries where several of your pages rank
★ Striking-distance queries (positions 8–20)
Click-through rate far below normal for the position (weak title or snippet)
Pages losing impressions month over month (content decay)
Pages with zero impressions in 90 days
★ Index status per URL (indexed, crawled but not indexed, discovered but not indexed)
Google-chosen canonical that differs from the declared one
Rich-result status for forum and Q&A markup
Discover clicks and impressions
Image search clicks
Country split per page
Device split per page
Branded vs. non-branded query share
Question queries (how, why, what) per page
New queries that weren’t there last month (emerging topics)
★ Queries with impressions but no good matching page (content gaps)
Week-over-week click changes per forum
Last crawl date reported by URL Inspection
Bing Webmaster Tools clicks, impressions, and queries
5. Analytics and engagement (GA4 and XenForo)
★ Organic visits per landing thread
Engagement rate of organic visitors
Average engagement time per thread
★ Organic visitors who register
Organic visitors who post
Pages per organic visit
Very short organic visits (a rough sign the page didn’t answer the query)
Scroll depth on long threads
Returning-visitor rate
Clicks on internal links zen-g added
On-site search queries (copy XenForo’s search table regularly, because it gets pruned)
★ On-site searches with no results
Referral visits from AI assistants (ChatGPT, Perplexity, Claude, Gemini)
Traffic share by device
Thread views over time (XenForo’s view counter)
6. Thread content quality (XenForo database and zen-g)
★ Vague titles (“Help!”, “Question”, “Problem”)
Titles in all caps or with too much punctuation
Duplicate titles across threads
★ Title contains the main query people search for (from Search Console)
First-post length
Total thread length
★ Reply count
Number of unique participants
★ Accepted solution present (question threads)
Time to first reply
★ Unanswered threads
Total reaction score
Last activity date
Outdated content (old years or product versions in the title or first post)
Thread language
Readability
Spelling error rate
Share of the page made of quoted text
★ Low-value replies (“+1”, “thanks”, “bump”)
Spam posts still live in the thread
★ Near-duplicate threads
How well the thread fits its forum’s topic
Products, brands, and places mentioned
Replies from experts (staff, high-karma members, members with many accepted solutions)
Thread state (locked, moderated, deleted, redirect)
7. Titles and meta tags (crawler)
★ <title> present on every page
Title length (risk of being cut off in results)
★ Duplicate titles across pages
Page number in the titles of pages 2+
Site name in titles, in a consistent format
★ Meta description present
Meta description length
Duplicate meta descriptions
Meta description taken from the first post, not boilerplate
★ One H1 per page, matching the thread title
Logical heading structure (no skipped levels)
Open Graph tags (og:title, og:description, og:image, og:url)
og:image set per thread (first image attachment)
X/Twitter card tags
<html lang> matching the content language
8. Page layout and HTML (crawler)
★ Viewport meta tag (mobile-friendly)
How much navigation and widget HTML comes before the main content
Share of the page taken by sidebars, widgets, and signatures vs. posts
Identical widget content repeated on every page (latest posts, online users)
Visible breadcrumbs
Visible post dates
Visible author names linking to profiles
★ Intrusive pop-ups (cookie walls or sign-up prompts covering content for search visitors)
Ads above the main content
Favicon present and valid (it shows in search results)
Invalid elements in <head> that make Google stop reading it early, so later tags are missed
Empty or broken links (href="#", javascript:)
“Load more” buttons hiding content that has no paginated URL
Helpful 404 page with search and popular threads
★ Bots and guests see the same content (no cloaking)
9. Images and media (crawler and XenForo attachments)
★ Alt text on image attachments
Image width and height set (prevents layout shift)
Lazy loading for images below the fold
Modern image formats (WebP, AVIF)
Image file size
Descriptive image file names
Uploaded images vs. images loaded from other sites
Broken embedded images
Video embeds with thumbnails and titles
Large images (1,200 px wide or more) and max-image-preview:large for Discover
10. Structured data (crawler)
★ DiscussionForumPosting markup on thread pages
Required fields present: author name, publish date, and text or headline
Author URL pointing to the member profile
Like and reply counts (interactionStatistic)
Replies included as comment entries
★ QAPage markup with acceptedAnswer on solved question threads
Accurate dateModified
BreadcrumbList markup
WebSite and Organization markup on the homepage (site name and logo in results)
ProfilePage markup on member profiles
Images from posts included in the markup
Validation errors and warnings
Markup that matches visible content (counts, dates, names)
Deleted and moderated posts left out of the markup
No duplicate or conflicting markup from other add-ons
11. Internal links (zen-g link graph)
★ Internal links pointing to each thread
Internal links going out of each thread
★ Orphan threads (reachable only through forum listings, or not at all)
★ Links inside posts vs. navigation and listing links
Anchor text variety per target
Generic anchors (“here”, “this thread”)
★ Internal PageRank
Links to redirecting URLs (/posts/123/, /goto/post links in quotes, old slugs)
★ Broken internal links
Links to noindexed or robots-blocked pages
Links using non-canonical URL variants
nofollow on internal links
Hub threads linking to their topic cluster
Cluster threads linking back to their hub
Relevance of the related-threads widget
Internal links to your most important threads (by traffic or sign-ups)
Pagination links as real <a href> links
How many listing pages a crawler must go through to reach old threads
Tag pages working as topic hubs
Links per post (too many looks spammy)
zen-g suggestion accept rate per target
Threads linked only from old, rarely crawled pages
Links inside quotes and signatures (repeated and low value)
Links from high-traffic threads to related low-traffic ones (where links help most)
Internal links added and removed over time
12. Outbound links (crawler and XenForo posts)
External links per thread
★ Broken external links
Redirected external links
rel="ugc" or nofollow on member links, with exceptions for trusted domains
★ Links to malware or phishing sites (Google Safe Browsing API)
Affiliate links without rel="sponsored"
Links to expired domains that spammers may have bought
Bursts of links posted by new accounts
Links in signatures and profiles of low-activity accounts
Most-linked external domains
13. Backlinks and mentions (Ahrefs or similar)
Referring domains
New and lost referring domains
★ Backlinks per thread
Most-linked threads
Backlink anchor text
★ Backlinks pointing to 404 pages (redirect them)
Spammy backlink patterns (watch only; Google ignores most of them)
Domain Rating trend (an Ahrefs metric, not a Google signal)
Unlinked brand mentions
Backlinks from sites in your niche
Threads shared or discussed on Reddit and social media
Keywords competitors rank for that you don’t
14. Performance (CrUX API, PageSpeed Insights, real-user monitoring)
★ Largest Contentful Paint (LCP), from real visitors
★ Interaction to Next Paint (INP)
★ Cumulative Layout Shift (CLS)
Time to first byte
First Contentful Paint
Core Web Vitals pass rate per page type (thread, forum list, profile)
Mobile vs. desktop Core Web Vitals
Guest page cache hit rate (XenForo’s page cache)
CDN cache hit rate
Page weight
JavaScript size
Third-party scripts (ads, analytics, embeds)
Render-blocking CSS and JavaScript
Web font loading
Uptime and downtime minutes
15. Security and trust (crawler, Safe Browsing, XenForo)
★ HTTPS everywhere, with no mixed content
HSTS header
TLS certificate expiry
Safe Browsing status of your own site
★ Spam posts left up for more than an hour
Hacked content or injected links
Privacy policy, terms, and contact pages
About page naming who runs the forum
Time taken to act on reported posts
Ad density
`````

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
