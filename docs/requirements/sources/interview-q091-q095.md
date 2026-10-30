# Interview source: Q091–Q095

Part of the [Q091–Q120 transcript](interview-091-120.md). Status, provenance and
redaction notes are on that page.

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
