# SPEC-010 part 7: format check and the owner's decisions

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in the
[specification decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Part of [SPEC-010](models.md). Requirements changed by the answers:
R120, R121, R122 and R125. Proposals and recorder's notes are in
[part 8](models-8.md).

## Format check of the shipped models

Checked on Dell from the Hugging Face API on 30 October 2026 by the
coordinator, and recorded here. `sutra models pin` repeats the check
for every revision it writes.

- `BAAI/bge-m3` at revision `5617a9f61b028005a4858fdac845db406aefb181`:
  PyTorch weights only as `pytorch_model.bin`, a pickle file of 2.27 GB.
  An official ONNX export, `onnx/model.onnx` with `onnx/model.onnx_data`,
  2.27 GB at full precision. No safetensors file and no other branch.
  `colbert_linear.pt` and `sparse_linear.pt` are also pickle, but
  dense-only use (R098) does not need them.
- `spacy/en_core_web_trf` at `58223163ebdcfdff182d526cc8deb1c73cd8d5a3`:
  `transformer/model` is a msgpack container (a compact data format
  used by Thinc, spaCy's machine-learning layer) that embeds a PyTorch
  `torch.save` ZIP archive (`archive/data.pkl`, pickle protocol 2,
  rebuilding tensors through `torch._utils._rebuild_tensor_v2`) about
  1 MB into the file. Loading it means unpickling. spaCy calls
  `torch.load` itself, so Sutradhar's static checks cannot confine that
  call. PyTorch's restricted "weights only" reader had a remote-code
  flaw (CVE-2025-32434, fixed in PyTorch 2.6).
- `spacy/en_core_web_lg` at `557bf758b8b070251eef3a62dd5442e72e72e2ce`:
  its model files (`tok2vec`, `ner`, `parser`, `tagger`, `senter`) are
  msgpack with no pickle markers; `vocab/vectors` is 617 MB.
- `Alibaba-NLP/gte-reranker-modernbert-base` (revision beginning
  `f7481e60`) and `minishlab/potion-retrieval-32M` (revision beginning
  `6fc8051f`) ship `model.safetensors`; they raise no conflict.

This check stays as the record of what was found. The owner's answers
to MD-01 and MD-02 resolved both conflicts (R120): bge-m3 is pinned as
its official ONNX export, and `en_core_web_lg` replaces
`en_core_web_trf`.

## Decisions (answered by the owner)

Ten choices that needed the owner's judgement, kept with their options
as history. The first option of each was the drafter's
recommendation. The owner answered all ten on 30 October 2026; each
answer follows its question. The technical choices, which the owner
accepted as a group, are in part 8.

### MD-01: bge-m3's PyTorch weights exist only as a pickle file

R098 chose bge-m3 run by PyTorch; R106, R028 and ADR-0002 allow only
data-only formats and never load pickle. Both cannot hold.

- Use bge-m3's official ONNX export at full precision, run through ONNX
  Runtime (recommended). Same model and publisher, no pickle. R106's
  rule that pickle is never loaded stays word for word; the owner would
  add this export to the formats R106 lists (then ONNX for community
  models only) and change R098's runtime for this model. Costs: ONNX
  Runtime needs an admission record; Sutradhar adds the dense-vector
  step the model card describes (first token's vector, normalised) if
  the export does not give it; outputs are checked against the example
  scores on the publisher's model card, since the pickle cannot be
  opened to compare. Tests that change: the lock entry (format, runtime,
  two files); loader tests gain a Verified ONNX path; ONNX operator and
  external-weight checks join the Verified tier and the nightly run;
  self-test and SPEC-008's recorded vectors come from the ONNX run, plus
  a model-card score test; memory and speed baselines are re-measured.
- Convert the pickle once into safetensors inside a sealed Dell lab
  sandbox (no network, a throwaway account, nothing else readable) and
  pin the converted file beside the original's checksum. Costs: a
  pickle is opened once in the lab, which R106 as written forbids, so
  it needs an explicit one-time exception; the converted file is not on
  the official source, so forum hosts need a signed Sutradhar source or
  bundle. Keeps PyTorch as R098 says. Tests: converter, output
  equivalence, provenance fields and the signed distribution route.
- Choose another embedder that ships safetensors and reads at least
  1,024 tokens. Costs: R098 changes; a named replacement needs the
  owner's approval and a new SPEC-009 comparison (R027, R100). Tests:
  every embedder fixture and SPEC-008 and SPEC-009 expectations.

Owner's answer: use the official ONNX copy, run through ONNX Runtime
(R120). The costs and test changes listed under that option apply and
are written into parts 1, 2, 4, 5 and 6.

### MD-02: `en_core_web_trf` keeps its transformer weights as a pickle

R098 chose `en_core_web_trf`; R106 never loads pickle. Both cannot hold.

- Use spaCy's large non-transformer English pipeline, `en_core_web_lg`
  (recommended): data-only, verified above, and run without PyTorch.
  R106 stays word for word; R098 changes, as a named replacement
  (R027). Costs: lower entity accuracy than the transformer pipeline
  by spaCy's own published figures, so fewer or weaker entity anchors
  (R104); its 617 MB word-vector table, though it needs far less
  working memory and processor time than a transformer. Tests that
  change: the lock entry and self-test entities; SPEC-009's entity
  anchor fixtures and its label mapping (P-MA04); SPEC-008's entity
  reading tests; the memory row; and a fixture copying the trf layout
  that must be refused.
- Convert the transformer weights once in a sealed Dell lab sandbox.
  Costs: a pickle is opened once in the lab (a one-time R106
  exception); spaCy cannot read safetensors for that component, so
  Sutradhar would maintain its own loading code inside spaCy's loading
  path across spaCy versions; and a signed Sutradhar source is needed.
  Keeps the transformer's accuracy. Tests: converter, equivalence,
  the custom loader and the distribution route.
- A different entity model that ships safetensors, to be chosen,
  admitted and compared; a data-only spaCy pipeline would still supply
  noun phrases and sentences. Costs: another model in memory, a new
  comparison and an R098 change. Tests: the new model's fixtures and
  SPEC-009's label mapping.
- Rejected: reading the embedded pickle with PyTorch's "weights only"
  reader. spaCy calls `torch.load` itself, so the restricted mode cannot
  be enforced; that reader had a remote-code flaw (CVE-2025-32434); and
  it still loads a pickle, against R106.

Owner's answer: use `en_core_web_lg` (R120). It runs without PyTorch;
the test changes listed under that option apply.

### MD-03: By estimate, everyday use does not fit the 6 GB cap

The memory table (part 5) puts everyday use at about 5,980 MB, past
the 95% refuse line, and about 7,150 MB with SPEC-012's map projection.

- Raise the service cap to 8 GB (an R021 change) if Dell measurements
  confirm the estimate (recommended). Everything runs side by side, the
  largest total stays under the 90% line, and rebuilds still take
  turns (MD-04).
- Keep 6 GB with stage turns: only bge-m3 stays loaded; the reranker
  and spaCy pipeline load for their stage of each batch; the map
  projection runs only when no stage model is loaded. Peak about 88%.
  Costs: slower backlog and suggestions (each swap's time measured),
  the 30-second target re-measured, and maps refreshed less often.

Owner's answer: raise to 8 GB if Dell measurements confirm the
estimate; until then 6 GB stands (R121). The option's "rebuilds still
take turns" was replaced by MD-04's answer: new suggestions pause and
the new embedder runs alone (R122). Asked later, the owner also
gave the bundled monitoring services a separate declared allowance on
top of the service's cap (R121, part 5).

### MD-04: How does a rebuild with a second bge-m3-sized model fit?

- Take turns in the one model-host process: new and edited posts first
  with the serving model, rebuild batches with the new one in between,
  with each swap's measured cost in the rebuild estimate (recommended).
  Suggestions continue; the rebuild takes longer than the first run.
- Pause new suggestions during the rebuild, so the new model runs alone
  and faster. Honest cost: a full bge-m3 rebuild takes about as long as
  the first run, one to several weeks per million posts (SPEC-008), and
  new and edited posts get no suggestions for that whole time.

Owner's answer: pause suggestions during the rebuild (R122). The owner
first chose taking turns, asked for the question again and chose the
pause, then kept the pause when asked to reconcile it with SPEC-009's
Q-MA06. From Q-MA06, suggestions already made stay approvable during
the rebuild, and the switch to the new index is made in one step.

### MD-05: Community model slower than the 30-second first suggestion?

May an administrator activate a community model whose trial projects a
first suggestion slower than 30 seconds on this host, where the active
model meets it (R015)?

- No: activation is refused; the administrator keeps the current model
  or tries another. R015 stays intact (recommended).
- Yes, after a warning and the administrator's confirmation, recorded
  in the audit log; that forum then runs outside the qualified target,
  and the health page says so. This waives R015 for that choice.

Owner's answer: allow after a warning (R125).

### MD-06: How does `models.lock` count under the file limits?

- As a tool-written lockfile: written only by `sutra models pin`,
  checked by `sutra models lock-check`, exempt from the 300-line limit,
  with lines still within 80 characters (recommended).
- As ordinary first-party text, split into one linked lock file per
  model, giving up the single file the owner described.

Owner's answer: exempt, tool-written.

### MD-07: How does an approved, lab-trained model reach use?

- As a "site-trained" model in that forum's overlay, delivered by a
  lab-signed bundle and never published (recommended). This adds a tier
  and badge name for the owner to approve.
- As a Verified model in `models.lock` for every forum, after a privacy
  review of publishing weights trained on the forum's content.

Owner's answer: site-only, never published.

### MD-08: What must a retrained model show before it goes to you?

- The method of SPEC-009's challenger rule (Q-MA03), with a lower bar
  for gain (recommended): recall at 5 better, with the lower end of its
  95% interval above zero; anchor agreement and reconstruction recall
  with a measured change of at least zero and the interval's lower end
  no lower than minus 1 point; no subgroup worse; hard rules 100%;
  budgets met.
- The challengers' bar: recall at 5 better by at least 2 points at the
  lower end, with the same other conditions.

Why the bars differ: a challenger would replace the model the owner
chose, so it must clearly win; a retrained version is the same model
refined on the forum's own decisions. Why minus 1 point is allowed: it
is measurement tolerance, not permission to get worse. Even an unchanged
model's interval reaches below zero by chance, so demanding zero there
would reject it. A model whose measured change is below zero still
fails, because R060 forbids these two measures to regress, and R060's
release rules still apply at release. Both rules share the 1-point
tolerance; this one adds the measured-change condition.

Owner's answer: better, with small tolerance.

### MD-09: Training content later deleted or erased?

- The model is flagged and replaced by the next retraining, within
  30 days; until then it stays in use (recommended).
- It is withdrawn at once, rolling back to the previous approved model.

Owner's answer: flag, replace within 30 days.

### MD-10: A community model states no licence?

- Refuse it, because no permission to use it is shown (recommended).
- Allow it after a stronger warning that the administrator accepts.

Owner's answer: allow after a stronger warning (R125).
