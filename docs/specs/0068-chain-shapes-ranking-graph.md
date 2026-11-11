# 0068: Chain shapes for link ranking and graph relevance

This spec writes down the first chain shapes of spec 0067, for two
scope areas of spec 0064: "Link ranking and protection" and
"Knowledge and graph analysis". Both shapes run inside `shodhan.rank`,
the ranking pipeline of spec 0027, and every step is a method that
spec 0027 or spec 0049 describes, each with its own catalogue entry
(spec 0055). It is roadmap Phase 7 item 1.

Example: a post mentions "spare power supply". One chain of the link
ranking shape scores each reranked candidate by reranker relevance,
then adds lexical match, entity agreement, graph relevance and
acceptance prior. It adds the three protection columns every chain has
(prefix caution, spam risk and self-promotion caution) and anchor
quality. It fuses that table by logistic fusion and calibrates it. The
relevance gate keeps only clearly relevant destinations, and the order
key puts them in order. Each other choice of step in a slot is another
chain of the same shape. A chain counts only as spec 0067 says.

## What it does

### Words used here

- Feature table: one row per candidate pair and one column per feature
  score, each from 0 to 1, with the method id that wrote it and that
  method's direction (spec 0027), carried with each pair's evidence.
  This spec adds it as an output kind in spec 0055's sense.
- Candidate table: a feature table with one row per candidate and no
  columns yet. It is the input of both shapes below.

### Output kinds and probe sets

1. Both shapes read a candidate table. In the link ranking shape every
   step up to fusion writes a feature table; fusion and calibration
   write scores and the order key writes an ordering, which for spec
   0067 are one output kind. Each score and ordering travels with the
   feature table and each pair's evidence it was made from, so the gate
   and the order key read the columns and evidence they need. Every
   step of the graph relevance shape writes a feature table.
2. Logistic fusion reads a feature table and writes a score, and every
   relevance method reads and writes a feature table, so spec 0055 has
   a probe set for each of those pairs of kinds. The link ranking shape
   runs on the first and the graph relevance shape on the second. Each
   is built as spec 0055 builds every probe set: seeded made-up forums
   whose posts, threads and links cover every evidence kind the steps
   below read.
3. A relevance method of spec 0027, and each part of spec 0049's graph
   relevance, reads the feature table so far, with each pair's
   evidence, and writes the same table with exactly one column of its
   own added. Every column it read is copied unchanged. These are the
   method's own kinds, alone or in a chain (spec 0027, "Words used
   here"), so it has one probe set and one seed either way (spec 0067,
   "Chain identity", item 4).
4. A step whose new column is exactly equal to a column already
   present, or orders the rows the same way as one, on every probe
   batch of two or more rows, fails its checks: it added nothing. So
   does a step that changes a column it read, whose new column has the
   same value in every row of every probe batch, or whose new column
   holds a value that is not a finite number from 0 to 1. Columns are
   compared after each is turned by its direction, and these checks
   run on every step of every chain run as well as on each step run
   alone.
5. A feature table's behaviour digest covers every column. Its order
   digest, coarse digest and rank correlation (spec 0067) are taken on
   the column its last step added, with ties ranked as spec 0067 sets,
   so a rescaled copy of a method matches it and a chain is compared
   with a single method by the one column each adds.
6. Every probe set for a feature table holds batches of two or more
   rows. Each batch starts from a seeded table: one made-up column for
   each step that writes a feature table and that an earlier slot of
   any shape on that pair of kinds can hold. Each such column has a
   seeded id, never a method id, and values drawn from the probe seed,
   never a method's real output. So every single method gets the same
   inputs, and a step run alone, such as fusion or the weighted
   combination, has columns to read and combine. Item 4 compares a new
   column with every column present, seeded ones included; the probe
   seed varies each seeded column on its own, so an honest step never
   matches one on every batch.
7. A chain runs on the same batches from the rows alone, with every
   seeded column removed, so its fusion reads only its own columns.
   So a chain is compared with a single method only where both read the
   same rows and evidence: a relevance method that adds one column is
   compared with a chain by that column, since that column does not
   depend on the seeded ones; fusion and the weighted combination run
   alone give a result that depends on seeded columns, so `qualify`
   never pairs a chain with them (spec 0067, "Words used here", probe set).

### Shape: link ranking

Pipeline step: `shodhan.rank` (spec 0027, "Where ranking sits", steps
2 and 3). Input kind: the candidate table of one source phrase after
spec 0023's reranking. Scope row: link ranking and protection, marked
as ranking.

| Slot | Writes | Steps (spec 0027 names) |
|---|---|---|
| 1. Reranker | feature table | reranker relevance |
| 2. Text | feature table | retrieval similarity; lexical match (BM25); title overlap |
| 3. Agreement | feature table | entity agreement; keyphrase agreement |
| 4. Graph | feature table | graph relevance |
| 5. Prior | feature table | acceptance prior; rejection-prone target |
| 6. Prefix | feature table | prefix caution |
| 7. Spam | feature table | spam risk |
| 8. Same author | feature table | self-promotion caution |
| 9. Anchor | feature table | anchor quality |
| 10. Fuse | score (relevance) | logistic fusion; LambdaMART |
| 11. Calibrate | score (calibrated probability) | calibration (job 33) |
| 12. Order | ordering, one kind with score | the order key (spec 0027, "The relevance gate", item 2) |

- Between slots 11 and 12 the shipped step applies spec 0027's
  relevance gate. It is part of the step, never a slot: it reads the
  calibrated probability and the reranker relevance column as slot 1
  wrote it, from the copy the shipped step keeps, never as a later step
  passed it on. Slot 12 puts the destinations that passed in order by
  the order key, so nothing in it can buy relevance. Its output still
  covers the whole probe batch: the destinations that passed come
  first, in order-key order, and those refused follow in the order of
  slot 10's relevance score, marked refused. The live service shows
  only those that passed.
- Position weight (spec 0027) sits in no slot: it depends only on the
  source, so its column has one value in every row and item 4 refuses
  it.
- Slots 6 to 8 each hold one step, so every chain carries prefix
  caution, spam risk and self-promotion caution into the gate, as spec
  0027 does. No chain can leave a protection out. The probe set varies
  each of the three on its own, so none of them orders the rows the
  same way as another on every batch.
- Slot 12's order key uses, as its relevance part, the relevance score
  slot 10 wrote, which travels with the calibrated probability. The
  order-stage outputs it combines (need, quality, freshness and trust)
  are worked out by the shipped step after the gate, as the gate is,
  never by a slot, and travel with each pair's evidence (spec 0027,
  "The relevance gate", item 2).
- Logistic fusion and LambdaMART are two methods of spec 0027. Which
  one the live service serves is job 45's setting; in a chain each is
  its own step.
- Slot 4's step is spec 0049's graph relevance method as the live
  service runs it. The chains of the graph relevance shape below
  count as members of their own (spec 0064) and never feed a link
  ranking chain.
- Slot 10 reads the columns its chain's table holds, with the weights
  job 45 trains for that set of columns (spec 0027, "The final score
  and confidence"). Weights are settings, never steps. A chain whose
  weights or calibration are not yet fitted gives no result and never
  falls back to other weights.

### Shape: graph relevance

Pipeline step: `shodhan.rank`, at its graph relevance method (spec
0049, "Relevance"), whose score is one feature of the link ranking
shape. Input kind: the candidate table of one source thread, one row
for each candidate thread paired with it. Scope
row: knowledge and graph analysis, not marked as ranking; graph
relevance counts towards ranking only as a step of the link ranking
shape.

| Slot | Writes | Steps (spec 0049, "Relevance", item 2) |
|---|---|---|
| 1. Shared terms | feature table | shared entities and tags |
| 2. Link prediction | feature table | Adamic–Adar; resource allocation; Katz (job 256) |
| 3. Community | feature table | same topic community (job 278) |
| 4. Embedding | feature table | spectral similarity (job 276) |
| 5. Combine | feature table | weighted combination |

- Adamic–Adar, resource allocation and Katz each need spec 0055's four
  conditions for an identity of their own ("One identity per
  procedure", item 5); their formulas differ as their sources state.
- Slots 2 to 4 read ganit's active outputs, slot 1 reads indexed
  lookups and slot 5 combines the columns already in the table; no
  graph maths runs in the service (spec 0049). For the link ranking
  shape's slot 4, the source is the thread the source phrase sits in.
  Which of job 256's three scores the live method uses is a setting of
  spec 0049.
- Slot 5's step adds the graph relevance column, so its chains end in a
  feature table and run on the same probe set as spec 0049's live
  graph relevance method. The chain whose steps are the live method's
  own parts behaves as that method and does not count (spec 0067,
  "Proving chains differ").
- Slot 5's weights come from settings (spec 0049, "Relevance", item
  2); changing them never makes a new chain.

### Shape sheets

1. Each shape above is one entry in `catalogue/shapes/` (spec 0067),
   naming each step by its method id. A shape is admitted only when
   every step it names has a catalogue entry.
2. The owner approves the sheets of one scope area as one batch, with
   the plain-English summary spec 0067 asks for.
3. A shipped pipeline step declares, when it registers, the steps
   every chain that ends at its relevance gate must carry;
   `shodhan.rank` declares prefix caution, spam risk and self-promotion
   caution. A chain that ends in a feature table, as the graph
   relevance chains do, does not reach the gate. Admission refuses any
   shape whose chains reach the gate but do not each carry every step
   its pipeline step declares, with no method ids written into the
   admission code.
4. Adding a step to a slot later makes a new shape (spec 0067,
   "Shapes", item 4); this spec is then amended with the new shape,
   never by editing an admitted one.

## Limits

- No shape here lists a setting, a preset or a playbook as a step.
- Candidate generators (spec 0027) and spam shapes for `kavach.spam`
  are left to later batches.
- The live service never runs a chain of these shapes: `shodhan.rank`
  fuses every ready relevance method, as spec 0027 sets. The chains
  run in qualified runs and lab runs (spec 0067, "Limits").

## Performance

- The link ranking shape shares the work of its first slots across
  chains, as spec 0067 sets. Its steps read matching's scores,
  tantivy's index or stored ganit outputs. Each step's measured cost
  goes into the time check of spec 0067, "Performance".
- The graph relevance shape reads indexed lookups and ganit's active
  outputs, and runs no graph maths.

## How it is tested

Rust tests in `sutradhar-methods` and `shodhan`, with made-up forums:

- Each shape's sheet passes admission: every step reads the output
  kind of the slot before it, no slot lists a setting, no method sits
  in two slots, and no shape reorders another's slots.
- A feature-table step whose column copies one already present, or
  orders the rows like it on every probe batch of two or more rows,
  fails its checks, and so does one that changes a
  column it read.
- A link ranking chain cannot run its order step on a destination that
  failed the relevance gate. A shape whose chains reach the gate but do
  not each carry every step its pipeline step declares is refused, and
  the graph relevance shape, whose chains end in a feature table, is
  not.
- The gate reads the reranker relevance column as slot 1 wrote it,
  even when a later step passes on a changed copy.
- A step whose column has one value in every row of every batch fails
  its checks, and so does one whose new column holds a value that is
  not a number, an infinity, or a value below 0 or above 1.
- A chain with a step that fails or waits on a batch gives no result
  for that batch.
- A step that copies or rescales a seeded column fails its checks.
- `qualify` never pairs a chain with fusion or the weighted combination
  run alone.
- A chain whose fusion weights or calibration are not yet fitted for
  its set of columns gives no result; it never falls back to the live
  weights, the live calibration or equal weights.
- A step run alone reads the seeded columns; a chain runs on the same
  batches with them removed, so slot 10's fusion and the graph shape's
  slot 5 read only columns their own chain wrote.
- A feature-table step that adds a rescaled copy of another method's
  column gets the same order digest as that method, and a graph
  relevance chain's order digest is taken on the column slot 5 added.
- The live path of `shodhan.rank` fuses every ready relevance method and
  runs no chain, and a graph relevance part is never fused on its own.
- The graph relevance chain built from the live method's own parts does
  not count against that method.
- A link ranking chain id turns back into the same twelve steps, and a
  graph relevance chain id into the same five.
- A link ranking chain with other fusion weights is the same chain, and
  so is a graph relevance chain with other combination weights.
- The ranking subtotal counts link ranking chains and not graph
  relevance chains.

## Changes from the source

Written fresh from specs 0027, 0049, 0055 and 0067.
