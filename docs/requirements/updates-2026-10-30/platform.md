# Requirements update: community models, progress and visuals

Owner: project owner. Recorded by the Claude coordinator from requests the
owner gave on 30 October 2026 while answering the model questions. The
owner's exact words are in the
[platform requests](../sources/platform-requests-2026-10-30.md).
Status: owner requirement decisions. They approve requirements, not the
feature specifications that must implement them, which still need drafting
and approval. Related: the [models update](models.md).

## R109: community models from Hugging Face

A point-and-click Hugging Face browser lets a forum's administrators add
embedding and reranker models beyond the pinned ones, keeping the models in
`models.lock` as the Verified tier, as recorded in
[ADR-0003](../../adr/0003-community-models.md): search with filters, a
compatibility check before any download (safetensors weights, or ONNX
weights through fastembed, no custom code, supported architecture, memory
budget, vector size, gated models, licence acceptance), pinned and
verified download into a per-site
overlay lock, self-test, a side-by-side trial on the forum's own links,
background index rebuild with one-click rollback, audit logging, health
reporting with a community badge, settings, command-line equivalents and
fixture-based tests. `trust_remote_code` is never enabled. Follow-up
answers: the owner approves the shipped defaults while each forum's
administrator chooses community models for their own site; ONNX through
fastembed is also accepted; community models with input windows smaller
than the piece size (1,024 tokens) or the reranker window (8,192) are
refused (the question asked about community models).

## R110: one progress system

Everything that can take longer than two seconds reports progress through
one system, as recorded in [ADR-0004](../../adr/0004-progress.md): a core
handle with persisted state and honest ETAs, command-line, web, API, log,
metric and notification views of the same state, use in every listed
long-running operation, and tests that enforce it for every registered job
and long command.

## R111: a visual layer on real data

The admin surfaces gain a visual layer built only on real data (any real
system data, following the owner's follow-up answer), with
reduced-motion support, text equivalents, lazy-loaded WebGL and free
libraries, and member text only on admin pages. The visuals (the owner's
56 plus two follow-up items), each with
its data source and status, are in the
[visual layer register](../../dev/visuals.md). The first ten are built
first, then the rest in the listed order. The owner's follow-up answer
("any real system data, plus an error log that is deduped, plus a live
feed that is deduped.") is recorded as the data rule and as register items
57 and 58; reading the error log and live feed as new visuals rather than
as data sources is the recorder's interpretation, for the owner to
confirm.

## Points the feature specifications must resolve

These are the recorder's notes, not owner decisions.

- Network access from the forum host to the Hugging Face Hub, the Hub
  client and the licence-accept record need admission and privacy review.
- Community models must respect the 6 GB service cap (R021 as amended)
  through the server's model budget.
- New libraries named by these requests each need an admission record
  before use (`huggingface_hub`, fastembed, `rich`, `sse-starlette`, the
  Prometheus client, marimo, pytest-sugar, Apache ECharts, sigma.js, D3, a
  WebGL scatter library, OpenTelemetry).
- Visual items 25, 26, 48, 54 and 56 touch undecided components or
  existing rules (R009, R010, R013); see the register's notes.
- R111 places visuals on admin surfaces, but item 56 is shown to forum
  members and item 54 goes out by ntfy or email; the specification states
  what member-facing and outgoing content may contain.
- Items 16 and 17 rely on click and traffic analytics and their consent
  rules (R029–R033).
- Live bars through server-sent events need a route into XenForo admin
  pages within R001 (XenForo integration rules) and R022 (thin PHP).
