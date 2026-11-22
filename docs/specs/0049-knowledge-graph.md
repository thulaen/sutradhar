# 0049: Knowledge graph

The knowledge graph is a map of the things a site talks about (people,
products, places, organisations, events) and how they connect: which
threads mention them, which tags group them, and what Wikidata, the free
encyclopaedia of facts, says about them. Sutradhar uses it to judge
whether two threads are about the same thing, and shows that reason apart
from the final ranking. An optional extra, off by default, drafts short
descriptions for XenForo tag pages. The module is `graph`. It is part of
roadmap Phase 6.

Example: threads mentioning "Pixel 9" and "Google Pixel 9 Pro" both
connect to the Wikidata item for the Pixel 9 series, so a link between
them gets a graph reason: "both about the Pixel 9 series".

## What it does

### Scope

1. In scope: building the graph from shodhan's recognised entities and
   keyphrases (spec 0026), links and tags; matching entities to
   Wikidata; relevance features with explanations; freshness and
   ambiguity checks; tag-description drafts, preview, batch approval,
   applying and undo.
2. There is no linking to XenForo tag pages.
3. The graph maths runs in ganit (specs 0064 and 0066). Pagdandi-style walks
   stay
   a lab candidate until the method lab admits them (spec 0056).
   Anchors are in spec 0023 and drafting is in spec 0052.

### Modules and design

1. Modules: `build` (nodes and edges, stored in PostgreSQL), `export`
   (Arrow snapshots for ganit), `apply` (versioned ganit results),
   `wikidata` (lookup and cache), `relevance`, `fresh`, `tagdesc`.
2. There is no graph library in the service. All graph maths (PageRank,
   communities, k-cores, link prediction, spectral features) runs
   once, in ganit's graph engines. The `graph` module stores nodes and
   edges, exports them and applies the results.
3. Nodes: entity (name, kind, optional Wikidata item id), thread, tag,
   forum.
4. Edges: thread mentions entity (with count and zone); thread links to
   thread (existing links); thread has tag; entity related to entity
   (Wikidata "instance of", "subclass of", "part of", "manufacturer" and
   similar, a fixed list); co-mention within a thread.
5. ganit receives node ids, edge types and weights only, never names.
6. Callers depend on traits with fakes: `EntitySource` (shodhan's
   output), `KnowledgeBase` (Wikidata over reqwest; fake `FixtureKb`),
   `GraphStore` (PostgreSQL with forced row-level security; in-memory
   fake), `GanitResults` (versioned result reader; fake with golden
   files), `TagWriter` (the add-on's endpoint; fake that records calls)
   and `Clock`.
7. One entity key (normalised name plus kind) everywhere. Wikidata
   answers are cached once and shared by relevance and tag drafts.
8. Edges update incrementally from changed threads, and relevance is
   answered by indexed lookups. The whole graph is never held in memory.

### Matching to Wikidata

1. Only entity names from guest-visible text are sent, never post text.
   Lookups are on, and the administrator is told.
2. Requests go to `www.wikidata.org` only (allow-listed), with a
   descriptive user agent, at most 1 request a second, cached for 30
   days.
3. Ambiguity: when two or more candidate items score within 10% of each
   other, the entity is "ambiguous" and gets no item id until a person
   chooses. The graph then uses only forum evidence for it.
4. Freshness: each matched person or organisation is re-checked every 30
   days for a date of death, dissolution or end time. A change marks
   dependent tag descriptions "needs review". Against vandalism, a
   change must hold for two checks before it is flagged.
5. Wikipedia pages are read only as evidence shown to the reviewer, by
   link. Their text is never stored or copied (CC BY-SA terms).
6. ganit job 295 (nicknames and synonyms) feeds the alias table used by
   entity matching.

### Relevance

1. `GraphRelevance::score(source, target)` gives a value from 0 to 1,
   symmetric, and an explanation.
2. It combines, with positive weights in the active spec 0077 champion:
   shared entities and tags (indexed lookups), the same topic community
   (ganit job 278), the
   link-prediction score (job 256) and the similarity of the two
   threads' spectral features (job 276).
   As a feature-table method (spec 0068) it calls this per-pair score
   for each row and adds the column. The weighted combination is a
   method of its own, and so is each part, used only as a step of a
   method chain (specs 0068 and 0069); the live service never fuses a
   part on its own. In a method chain of spec 0068 the link-prediction
   part is one of job 256's three scores (Adamic–Adar, resource
   allocation or Katz). The selected score is fixed in the versioned
   candidate configuration. The combination weights follow the same
   champion contract; spec 0014 stores only their exactly activated values.
   Spec 0077 alone evaluates and promotes a
   change, never an independent per-site live-method switch.
3. The explanation names the strongest part, such as "both mention X",
   "X is part of Y" or "same topic: home routers".
4. It feeds the relevance side only: it is one of the scorer features
   from which candidates and ranking (spec 0027) calibrate the
   acceptance probability that the relevance gate checks. It is not a
   need or quality input, so it never reorders candidates after the gate
   and cannot pass the gate on its own.
5. It is reported apart from the overall score.

### ganit jobs used

Each result is a versioned, inert candidate. The service records shadow
evidence; spec 0077 alone evaluates, promotes, activates and rolls it back.
A late or failed candidate leaves the verified champion unchanged, and a
result holding a NaN is refused.

- Job 251 internal PageRank and job 255 core knowledge threads (k-core),
  stored per thread.
- Job 254 link communities and bridging threads.
- Job 256 link-prediction scores.
- Job 264 tag co-occurrence, for tag synonyms and hierarchy shown to
  staff.
- Job 262 topic cohesion per forum, for the dashboard.
- Job 276 spectral graph embeddings, and job 278 the clustering of the
  combined link and meaning graph: the single topic definition shared
  with hubs (spec 0052).
- Job 284 topic names for job 278's clusters (from ids and keyphrase
  ids; the service maps them back to words).
- Job 286 near-duplicate clusters and their canonical thread.
- Job 295 nicknames and synonyms.

### Tag descriptions (optional, off by default)

1. Short SEO meta descriptions for XenForo tags, at most 120 characters.
   When the feature is off, no draft, preview or write happens.
2. Staff with the "Manage tag descriptions" permission choose tags, or
   "all without a description".
3. A draft is written for each by the drafting route of spec 0052 (staff
   members' own connected AI assistants, through the MCP route), from an
   evidence pack: the Wikidata facts, the forum's top threads under the
   tag (titles only) and the freshness result.
4. Checks on every draft:
   - at most 120 user-perceived characters (counted with icu_segmenter,
     so an emoji counts as one);
   - no 6-word run shared with the Wikidata or Wikipedia description
     (original wording);
   - no claim about a living person beyond the evidence;
   - plain text only.
5. Preview: a table of tag, current description, draft, evidence and
   check results. Staff edit or remove lines, then approve the batch.
6. Approved lines are applied through the add-on's endpoint, which uses
   XenForo's own save mechanism. Each batch can be undone as a whole,
   restoring every previous text.

### Brand entity evidence (624–628)

Keep the owner's organisation evidence separate from the entity
graph's existing ranking inputs and general Wikidata matching.
The brand view creates no new connector or scheduled rival
watch. Read eligible own-site observations or authorised supplied
files. A separately approved Wikidata presence read uses the exact
one-off public page admission of 0082 and address checks of 0107.
The owner approves one known public HTTPS item address with its QID,
legal profile, rights, limits, expiry and grant revision. Claim it
before a single request. Redirects give an unknown result. No search,
API discovery, related links, retries, automatic refresh or private
names, drafts or member records in requests. The feature has its own
site-scoped evidence read and store. No credentials, scheduled
matcher snapshots, generic cache fallback or graph output are shared.
Keep third-party observations dated. Absent proof gives unknown.

An organisation version binds site, owner-approved entity ID,
official name, approved aliases, public evidence revisions,
scope and active date. Store the original visible spelling. Name
profile v1 applies Unicode NFC, default full case folding, NFC again,
and trims and collapses Unicode White_Space to one ASCII space.
Compare exact normalised names and approved aliases. Similar
spellings, confusable characters and collisions require review.
Matching names cannot prove identity or cause a merge. The report
separates verified identity from names observed in dated samples.
It cannot claim to cover the web or silently rename anything.

A knowledge panel checklist examines the current eligible
own-site schema.org `Organization` markup against [Google's
Organization guidance][organization]. Google lists recommended
properties. The checklist shows each applicable name, public URL,
logo and `sameAs` as supported, missing, mismatched or unverified.
For `sameAs`, check valid public HTTPS syntax, then the exact entity
against the current official profile register. Independent control
evidence plus exact public identity may prove an official social
profile. A channel, alias or one matching name alone cannot. Show
local markup coverage and gaps. No remote panel read, inferred panel
status, ranking change or markup write.

For Wikidata presence record an exact item ID, revision,
observed date, item evidence and identifier or redirect
provenance. Match the active site organisation to the admitted
item type and independently supported identity. A disambiguation,
unknown redirect or identical name alone cannot prove a match.
Label proved, ambiguous, not observed in this sample, or unknown
for unread or stale evidence. Use [Wikidata notability][wd-notability]
as dated policy guidance. Record evidence of the stated paths:
qualifying sitelink, serious public references or structural need.
A proved identity does not establish any path. Label the
[self-promotion essay][wd-essay] as an essay. No edit, account
operation, item creation or promotional recommendation.

The local profiles register records site, entity and profile
versions, stable provider profile ID and role, exact public HTTPS
address, allowed use, ownership evidence, date, rights and expiry.
Use 0107 for published address syntax. Refuse credentials, token
parameters and ambiguous addresses. Treat it as inert. The owner
may add, replace or revoke a row against its current revision.
Independent current control and identity evidence must agree.
A redirect may be recorded as evidence. It grants no new address
or write right. A changed, expired or revoked profile invalidates
dependent checks, draft and copy actions before they are shown.
Restore must apply current erasure and revocations first.

A founder or team page is a local WordPress proposal with
exact owner-approved entity and public profile facts and active
person-specific 0098 consent covering each fact, link, image and
credit used. Strip private contact and role data. Check claim, template and
source revisions
at review. Spec 0052 owns the local proposal, 0020/0028 own
approval and 0100 owns intake boundaries. Native creation requires
an admitted 0021 signed WordPress draft-only action.
Bind site, native author and subject, approved fields and asset
grants, current consent and policy revisions, expected WordPress
revision and expiry to the staff approval. Compare current state
before the one write and store an idempotent receipt. Revocation
withholds further action and flags an admitted draft for staff.
Later human edits stay intact. The action cannot publish, index or run a
public cache effect. Site staff publish only under current approval.
No XenForo page action or outside profile write. Force the site's
0108 data scope. Register inputs and derivatives under 0032/0118.

#### Wave 2 brand entity and organisation fixture bindings

Use synthetic eligible own-site observations and authorised supplied
records. QR-156 still owns dated source, rights, identity and native
review evidence. The fixtures use stand-ins where a separate native
or read admission is required. They grant no new request, identity
profiling, graph signal, public change or cross-site scope.

| Feature | Owning requirement and design | Planned fixture and expected evidence |
|---|---|---|
| F-0624 | REQ-0049-W2-ORGANISATION-CHECKLIST · DES-0049-W2-PROVED-BRAND-EVIDENCE | FX-0049-W2-ORGANISATION-CHECKLIST · IT-0049-W2-ORGANISATION-CHECKLIST. Observe eligible schema.org Organisation fields under the pinned 0115 and dated guidance profiles. Compare a current independently confirmed official profile, misleading sameAs, a complete missing field and unread evidence. Distinguish supported, missing, mismatched and unverified fields with coverage. No external panel observation, appearance claim or markup write. |
| F-0625 | REQ-0049-W2-DATED-WIKIDATA-PRESENCE · DES-0049-W2-EXACT-ITEM-EVIDENCE | FX-0049-W2-DATED-WIKIDATA-PRESENCE · IT-0049-W2-DATED-WIKIDATA-PRESENCE. Use an eligible supplied item revision, a same-name ambiguous record, a declined redirect outcome and a spent one-off approval. Bind exact QID, item type, observed date and independent organisation evidence. A spent approval cannot resolve another item or repeat. Identity and each notability-evidence path stay distinct. A missing or stale source stays unknown. No credential, lookup, account action or item creation in the fixture. |
| F-0626 | REQ-0049-W2-BRAND-NAME-COMPARISON · DES-0049-W2-PROVED-BRAND-EVIDENCE | FX-0049-W2-BRAND-NAME-COMPARISON · IT-0049-W2-BRAND-NAME-COMPARISON. Compare existing own-site and authorised sample spellings against the current approved name and aliases using the existing name profile v1. Preserve original spelling and dated source. Hold confusables, stale samples and incomplete evidence. A name match proves no entity identity, profile ownership or authority to rename. |
| F-0627 | REQ-0049-W2-FOUNDER-TEAM-DRAFT · DES-0049-W2-EXACT-CONSENTED-DRAFT | FX-0049-W2-FOUNDER-TEAM-DRAFT · IT-0049-W2-FOUNDER-TEAM-DRAFT. Build a local 0052 WordPress proposal from eligible public organisation facts and current person-specific 0098 consent for each used fact, link, image and credit. Expire one grant or change a source revision before review. For the separately admitted 0021 draft-only capability use a stand-in receipt, replay and revoke after draft creation. Retain only the exact admitted effect; flag it for staff and preserve later edits on withdrawal. Without that profile stay local. No XenForo native route, publication, indexing, public cache effect or outside profile update. |
| F-0628 | REQ-0049-W2-OFFICIAL-PROFILE-REGISTRY · DES-0049-W2-VERIFIED-INERT-PROFILE | FX-0049-W2-OFFICIAL-PROFILE-REGISTRY · IT-0049-W2-OFFICIAL-PROFILE-REGISTRY. Add an exact authorised public HTTPS profile to the site-local organisation registry with current independent control and identity evidence. Refuse an ambiguous or unsafe address, stale evidence or changed revision. Revocation and restore withhold dependent checks, drafts and copies while retaining permitted safe lineage. An alias, redirect or profile row gives no new network or markup authority. |

[organization]: https://developers.google.com/search/docs/appearance/structured-data/organization
[wd-notability]: https://www.wikidata.org/wiki/Wikidata:Notability
[wd-essay]: https://www.wikidata.org/wiki/Wikidata:Self-promotion/en

### Wave 1 topic and graph evidence

Bind the active site, exact eligible graph, source revisions, covered
population, rights and freshness. Use only admitted own-site graph
inputs and the separately allowed evidence below. An unresolved
identity or incomplete sample supports no inferred topic, gap or rank
fact. These views open no new provider, native or publication route.

| Feature and owning design | Exact planned fixture and outcome |
|---|---|
| F-0049 · REQ-0049-W1-AUTHORITY-MAP · DES-0049-W1-TOPIC-VIEW | FX-0049-W1-AUTHORITY-MAP. Reuse the 0049 graph building, private-thread, ambiguity, member-mention and Arrow export cases. Compare two qualified clusters, a public homonym and an ambiguous outside identity. Keep distinct proved entities; show graph source, observed population and the approved basis for strong or weak areas. An unmeasured authority basis stays unknown. No private thread, member mention or unresolved entity gives a rank fact or new Wikidata lookup. |
| F-0050 · REQ-0049-W1-HUB-COVERAGE · DES-0049-W1-OWN-GAP | FX-0049-W1-HUB-COVERAGE, BDD-0049-W1-HUB-COVERAGE. Compare a current eligible hub with two covered topics, a supported demanded topic absent from its complete admitted membership, and a missing query period. Require compatible 0092 demand evidence and the exact 0029/0052 hub scope. A missing term or metric is unknown, not zero or a gap. Show only independently supported additions or link proposals for ordinary review. |
| F-0051 · REQ-0049-W1-RIVAL-TOPIC-GAP · DES-0049-W1-RIVAL-SITEMAP-EVIDENCE | FX-0049-W1-RIVAL-TOPIC-GAP, IT-0049-W1-RIVAL-TOPIC-GAP, reusing IT-0092-RIVALS and E2E-0092-SCOPE. Use an owner-supplied permitted snapshot or one exact owner-selected public sitemap check through the existing DR-085 and 0092/0082 Receipts route. Bind source and coverage to the observed sample. A proved topic may support an observed scoped gap with admitted own-site evidence. Slug-only hints, unread children and denied sources prove none. A replay, sitemap index or saved rival cannot start another request, child traversal or background monitoring. |

Shared source checks retain their owners. No binding admits a new
coverage floor, authority model, link effect or ranking signal.

### Data

`graph_entity`, `graph_edge` (typed, weighted, per site), `graph_kb_cache`
(item id, facts, fetched at), `tagdesc_batch`, `tagdesc_item` (tag id,
previous text, draft, checks, state, approver, applied at). Every table
has forced row-level security, per site.

### API, events and commands

- Management API: `GET /api/graph/thread/{id}`,
  `GET /api/graph/explain?source&target`, `POST /api/tagdesc/batches`,
  `POST /api/tagdesc/batches/{id}/approve`,
  `POST /api/tagdesc/batches/{id}/undo`.
- Events: `graph.updated`, `tagdesc.batch_ready`, `tagdesc.applied`,
  `tagdesc.undone`, `graph.entity_stale`.
- Commands: `sutra graph explain`, `sutra graph rebuild`,
  `sutra tagdesc preview|approve|undo`.

### Failures

- SUT-670: Wikidata unreachable. Forum evidence only, retried with
  backoff.
- SUT-671: a draft fails its checks. Kept, marked, never applied.
- SUT-672: the tag changed since the preview. That line is skipped and
  shown.
- SUT-673: applying failed at XenForo. The batch stops; the applied part
  can be undone.
- SUT-674: the graph rebuild is more than a day behind.

### Security and privacy

1. Only guest-visible content. A private thread adds no node, edge or
   lookup.
2. Only entity names are sent out.
3. Members' names (mentions such as `[USER]`) never become graph
   entities.
4. Erasing a thread removes its edges and regenerates the affected
   relevance.
5. Tag descriptions are public text reviewed by staff.

### Edge cases to cover

Entities with the same name in different kinds; renamed products; merged
Wikidata items (redirects); entities in other languages; tags that are
not entities ("help", "beginner"); tags with thousands of threads; a tag
deleted during a batch; Wikidata vandalism.

### Build-blocking platform and source evidence

The graph integration record must establish whether XenForo 2.3 tags have a
description or meta-description field or the add-on must provide one, current
Wikidata user-agent and rate rules, Wikipedia reuse terms, and the Wikidata
properties for death, dissolution and end time. Each dependent integration is
blocked until official sources, dates, licence review and isolated fixtures
are admitted; no platform field or external property is guessed.

## Limits

- The whole graph is never held in memory. Building edges and the Arrow
  export stream in pages, at most 64 MiB above idle in the worker. The
  relevance cache (moka) is at most 32 MiB in the service. ganit's batch
  runs keep their own budget. All within the service slice, measured
  nightly.
- Wikidata: at most 1 request a second, cached for 30 days.
- Graph relevance adds at most 5 ms to scoring one candidate pair.
- An incremental update after a new post is visible within 60 seconds.
- A preview of 500 tags opens within 2 seconds.

## Footprint budget

The graph adds nothing to public pages. Tag descriptions, when turned on,
add at most 120 characters of text to each tag page, saved through
XenForo's own mechanism. Any further time, query or byte budget is a
readiness blocker owned by the tag-description footprint research record;
the optional feature cannot build or activate until measured bounds and
fixtures are admitted. This blocker does not remove the graph's own limits.

## How it is tested

Rust tests in the `graph` module:

- Building: fixture threads with entities give the fixture's nodes and
  edges.
- Privacy: a private thread adds no node, edge or lookup.
- Ambiguity: two candidates within 10% leave the entity ambiguous with no
  item id.
- Freshness: an item that gains a date of death turns its tag
  descriptions to "needs review".
- Explanations: two threads sharing an item get an explanation naming
  it.
- Property test: for any pair the score is between 0 and 1 and
  symmetric.
- Length: a 121-character draft, counting an emoji as one, fails.
- Originality: a draft sharing 6 words in a row with the Wikidata
  description fails.
- Off: with the feature off, no draft, preview or write happens.
- Flow: an approved batch is applied line by line through the fake
  add-on, and undo restores every previous text.
- A tag edited after the preview has its line skipped with SUT-672.
- Outage: Wikidata answering 503 triggers backoff and forum-only
  evidence.
- Mentions: a `[USER]` mention makes no entity from the member's name.
- Contract: a fixture graph exported in Arrow holds ids, edge types and
  weights only, no names; a job 251 result with a NaN is refused and the
  verified champion remains unchanged.
- Benchmark: `score` stays within 5 ms per pair under the 5% median
  rule.
- Depth follows ADR-0020. Only admitted strict-core components require
  100% line, branch and MC/DC coverage and zero surviving killable mutants,
  with equivalent mutants proved and independently reviewed; other code
  follows the owning risk matrix. The private-content guard, mention rule,
  120-character check, originality check and "off means nothing happens"
  rule retain their assigned mutation gates and property tests. Kani proves
  the length check refuses every draft over 120 user-perceived characters,
  and that no apply call is reachable for a line that is not approved.

## Changes from the source

- The graph maths runs in ganit, the Rust numerical engine.
- The preview page is a desktop app screen built from SDS components.
- Speed follows the 5% median rule; the 20x goal is kept (spec 0007).

Source: `docs/specifications/signals/graph.md` in the archive
`archive-2026-11-08-old-repo`.
