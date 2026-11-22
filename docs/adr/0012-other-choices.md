# ADR-0012: Other version 1 choices

Decided by the owner on 8 November 2026. The rules for each are in
[AGENTS.md](../../AGENTS.md); this record keeps the reasons. Model,
provenance and ranking-signal rows are revised by
[ADR-0020](0020-version-1-documentation-and-architecture-baseline.md).

| Topic | Why |
|---|---|
| App under AGPL, add-ons under GPL | Replaced by Apache-2.0 for the app per ADR-0021 and DR-037; add-ons stay GPL. WordPress needs GPL-compatible plugins; AGPL was meant to keep the app open |
| BGE-M3 and BGE reranker through admitted data-platform runtimes | One multilingual retrieval and cross-encoder direction, with release evidence and no unsafe production weight loading |
| Broad spaCy NLP pipeline | Shared typed linguistic features without duplicating entity and classification producers |
| Optional paid embeddings | An explicitly enabled outside route and the sole paid production exception for dense retrieval under ADR-0020; local sparse, multi-vector and rollback operation remain mandatory |
| Screen text in translation files | More languages can be added without code changes |
| Free public bot lists | No paid service needed |
| mdBook manual, published and built in | One source for both |
| Test sites in containers with made-up data | Safe to break; the XenForo licence stays private |
| Cedar and Rego policy languages | Typed permission and build-policy contracts |
| Indian module names, plain English screens | Short, unique names; screens stay clear |
| GitHub as the only off-site copy of the repository | Owner's choice; see the risk in [ADR-0008](0008-forgejo-and-github.md) |
| Release key on Dell | Keeps releases automatic; every use is logged |
| Agents mark ready tasks in roadmap order | Agents keep moving without waiting |
| Agents close phases after a signed check | Work never waits at the end of a phase |
| Editorial Coach wording through approved local models, optional MCP and paid routes | Private by default; the owner controls outside routes |
| Quality judges may use Bedrock or MCP, off by default | The owner decides when site text may leave the PC |
| A 12 GB service-stack admission ceiling and a separate 1 GB desktop ceiling | Admission uses measured working sets on each declared hardware profile; physical memory size never proves that the service stack or desktop fits |
| Extensions sealed off with Linux's own controls, no Docker on the PC | Nothing extra to install; still kept apart |
| Provenance classes with targeted strict clean room | Licensed reuse, public standards and interoperability evidence remain distinct from strict clean-room work |
| Ranking signals admitted by relevance hypothesis and held-out evidence | Useful signals can reorder eligible links without coupling every feature to ranking |
