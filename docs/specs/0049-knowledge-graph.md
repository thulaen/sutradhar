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
3. The graph maths runs in ganit (specs 0064 and 0066). Pixie-style walks stay
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
2. It combines, with weights in settings: shared entities and tags
   (indexed lookups), the same topic community (ganit job 278), the
   link-prediction score (job 256) and the similarity of the two
   threads' spectral features (job 276).
   As a feature-table method (spec 0068) it calls this per-pair score
   for each row and adds the column. The weighted combination is a
   method of its own, and so is each part, used only as a step of a
   method chain (specs 0068 and 0069); the live service never fuses a
   part on its own. In a method chain of spec 0068 the link-prediction
   part is one of job 256's three scores (Adamic–Adar, resource
   allocation or Katz); which of the three the live method uses is a
   setting.
3. The explanation names the strongest part, such as "both mention X",
   "X is part of Y" or "same topic: home routers".
4. It feeds the relevance side only: it is one of the scorer features
   from which candidates and ranking (spec 0027) calibrate the
   acceptance probability that the relevance gate checks. It is not a
   need or quality input, so it never reorders candidates after the gate
   and cannot pass the gate on its own.
5. It is reported apart from the overall score.

### ganit jobs used

Each result is versioned. The service shadow-tests it against the active
one, then activates it, and keeps the last good version if a job is late.
A result holding a NaN is refused and the previous one stays active.

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

### Facts to check when built

Whether XenForo 2.3 tags have a description or meta description field of
their own, or the add-on must keep and show it; Wikidata's current
user-agent and rate rules; Wikipedia's reuse terms; the Wikidata
properties for death, dissolution and end time.

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
XenForo's own mechanism; any further budget is set when the feature is
built.

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
  previous PageRank stays active.
- Benchmark: `score` stays within 5 ms per pair under the 5% median
  rule.
- Depth: 100% line and branch coverage. The private-content guard, the
  mention rule, the 120-character check, the originality check and the
  "off means nothing happens" rule have no surviving mutants and have
  property tests. Kani proves the length check refuses every draft over
  120 user-perceived characters, and that no apply call is reachable for
  a line that is not approved.

## Changes from the source

- The graph maths runs in ganit, the Rust numerical engine.
- The preview page is a Dioxus app screen built from SDS components.
- Speed follows the 5% median rule; the 20x goal is kept (spec 0007).

Source: `docs/specifications/signals/graph.md` in the archive
`archive-2026-11-08-old-repo`.
