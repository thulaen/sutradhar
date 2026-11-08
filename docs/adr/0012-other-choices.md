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
| Qwen3-Embedding-0.6B for text matching | Small and strong; runs in pure Rust |
| Our own entity model in pure Rust | No outside model to trust; fits the memory limits |
| Quality judges may use Bedrock or MCP, off by default | The owner decides when site text may leave the PC |
| Memory limits scale with the PC: 5 GB below 16 GB, 8 GB at 16 GB or more | Works on an 8 GB PC; uses more when there is room |
| Extensions sealed off with Linux's own controls, no Docker on the PC | Nothing extra to install; still kept apart |
| A tiny first-pass model, potion-retrieval-32M, before Qwen3 | Fast first pick on an 8 GB PC; Qwen3 makes every final match |
| Every feature is a clean-room design | We learn ideas from public pages only and copy nothing |
| Every feature feeds the link ranking where it can, behind the relevance gate | More signals, while off-topic links can never rise |
