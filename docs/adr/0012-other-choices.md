# ADR-0012: Other version 1 choices

Decided by the owner on 8 November 2026. The rules for each are in
[AGENTS.md](../../AGENTS.md); this record keeps the reasons.

| Topic | Why |
|---|---|
| App under AGPL, add-ons under GPL | WordPress needs GPL-compatible plugins; AGPL keeps the app open |
| Models in pure Rust | No outside native library to trust |
| Optional paid embeddings | Better matches when wanted; works without them |
| Screen text in translation files | More languages can be added without code changes |
| Free public bot lists | No paid service needed |
| mdBook manual, published and built in | One source for both |
| Test sites in containers with made-up data | Safe to break; the XenForo licence stays private |
| Cedar and Rego policy languages | Both run by Rust libraries |
| Indian module names, plain English screens | Short, unique names; screens stay clear |
| GitHub as the only off-site copy of the repository | Owner's choice; see the risk in [ADR-0008](0008-forgejo-and-github.md) |
| Release key on Dell | Keeps releases automatic; every use is logged |
| Agents mark ready tasks in roadmap order | Agents keep moving without waiting |
| Agents close phases after a signed check | Work never waits at the end of a phase |
| Editorial Coach wording by pure-Rust models, plus an MCP server | Private by default; the owner may plug in their own AI tools |
