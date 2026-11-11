# 0069: Wide link ranking shape

This spec adds one chain shape of spec 0067 for the scope area "Link
ranking and protection" of spec 0064, marked as ranking. Spec 0067
freezes an admitted shape, so spec 0068's link ranking shape is never
widened; this is a second shape beside it. It runs inside
`shodhan.rank` exactly as spec 0068's link ranking shape does, and adds
two slots: text normalisation before text relevance, and score
normalisation before fusion. Every step is a published method with its
own catalogue entry (spec 0055). It is roadmap Phase 7 item 1.

Example: a post mentions "spare power supplies". One chain of this shape
takes the reranker relevance, stems both sides with the Porter stemmer,
and scores the pair by query likelihood with Dirichlet smoothing on the
stemmed words. It adds YAKE keyphrase agreement, Adamic–Adar closeness,
a Wilson lower bound on past acceptance, the three protection columns
and pointwise mutual information as the anchor step. It puts every
relevance column but the three cautions on one scale by z-score, fuses
them by ListMLE and calibrates by beta calibration. The relevance gate
keeps only clearly relevant destinations, and the order key puts them in
order. Each other choice of step in a slot is another chain of the same
shape. A chain counts only as spec 0067 says.

## What it does

### Words used here

- Feature table and candidate table: as spec 0068 defines them.
- Relevance column: a column that a relevance method of spec 0027
  writes. Prefix caution, spam risk and self-promotion caution are
  relevance methods there, so their columns are fused too.
- View: a derived copy one step adds to each pair's evidence, tagged
  with its method id: a text view normalises the source phrase and the
  destination text; a score view normalises one column within a probe
  batch. A view is never a column and is never gated or ordered.
- Word step: a step whose catalogue entry declares that it reads words
  of the source phrase or the destination. Every other step reads
  vectors, stored scores or ganit outputs.
- Prefix: a chain's steps before its fuse slot.

### Output kinds, views and checks

1. The shape reads a candidate table and every step up to fusion
   writes a feature table, so each step reads the kind of the slot
   before it (spec 0067, "Shapes", item 2). Fusion, calibration and the
   order key write scores and an ordering, as spec 0068 sets.
2. The steps of slots 2 and 11 are the only feature-table steps of this
   shape that add no column. Each copies every column unchanged and adds
   one view per pair (slot 2) or one view per column other than the
   three cautions (slot 11). Spec 0068's checks on a new column ("Output
   kinds and probe sets", item 4) do not apply to them; these do:
   - a step that changes or adds a column, or adds more or fewer views
     than its slot asks, fails;
   - a text view that equals the raw text on every pair of every probe
     batch fails: it added nothing;
   - a slot 11 step whose every score view equals its column on every
     row of every probe batch fails, since it added nothing; a view
     that equals its column, such as min–max on a column of 0 and 1,
     is fine on its own; a view value that is not a finite number
     fails;
   - a text view depends only on the text it normalises, and a score
     view only on its own column within the batch and, where the
     step's entry names one, the fit its owning ganit job gives for
     that batch.
3. Every other feature-table step adds exactly one column and keeps spec
   0068's checks. A word step reads the latest text view in the pair's
   evidence when one is present, and the raw text when it runs alone.
   Word statistics a word step reads, such as term and document
   frequencies, are built by their owning ganit job once for each text
   view of slot 2 and once for the raw text, and a step reads the
   statistics built for the text it reads, so query likelihood on
   stemmed words reads stemmed statistics. Two kinds of step always read
   the raw text and never a text view: the protection steps of slots 7
   to 9, so a caution such as a "Closed" prefix is never hidden by
   stemming; and any step that reads a model built from the raw text
   only, such as LSI, LDA or soft cosine. A step that reads only
   vectors, stored scores or ganit outputs reads no text and ignores any
   text view.
4. The fuse step reads each column's score view when one is present,
   except the three caution columns, which it always reads as written
   and turned by their direction, never as a view, so a caution set on
   every row of a batch is never erased by normalising. The relevance
   gate reads the reranker relevance column as slot 1 wrote it, from the
   copy the shipped step keeps, never a view.
5. A feature table's behaviour digest covers every column and view.
   Where its last step added no column, its order and coarse digests
   equal its behaviour digest; otherwise spec 0068 ("Output kinds and
   probe sets", item 5) sets them. A step of slot 2 or 11 uses the
   relevance methods' probe set, with the same seed whether run alone or
   in a chain.

### Shape: wide link ranking

Pipeline step: `shodhan.rank` (spec 0027, "Where ranking sits", steps
2 and 3). Input kind: the candidate table of one source phrase after
spec 0023's reranking. Scope row: link ranking and protection, marked
as ranking.

| Slot | Writes | Steps | Rows listed |
|---|---|---|---|
| 1. Reranker | feature table | reranker relevance | 1 |
| 2. Normalise text | feature table, text view | see below | 7 |
| 3. Text | feature table | see below | 20 |
| 4. Agreement | feature table | see below | 14 |
| 5. Graph | feature table | see below | 34 |
| 6. Prior | feature table | see below | 11 |
| 7. Prefix | feature table | prefix caution | 1 |
| 8. Spam | feature table | spam risk | 1 |
| 9. Same author | feature table | self-promotion caution | 1 |
| 10. Anchor | feature table | see below | 8 |
| 11. Normalise scores | feature table, score views | see below | 5 |
| 12. Fuse | score (relevance) | see below | 22 |
| 13. Calibrate | score (calibrated probability) | see below | 9 |
| 14. Order | ordering, one kind with score | the order key | 1 |

- Between slots 13 and 14 the shipped step applies spec 0027's
  relevance gate, as spec 0068 sets: part of the step, never a slot.
  Those refused follow those passed, in slot 12's score order, marked
  refused; the live service shows only those that passed.
- Slots 7 to 9 each hold one step, so every chain carries prefix
  caution, spam risk and self-promotion caution into the gate.
- Slot 14 is as spec 0068 sets for its slot 12. Slot 5's source is the
  thread the source phrase sits in; chains of spec 0068's graph
  relevance shape never feed it.

### Steps by slot

Each row is one method with its public source; slots 1, 7, 8, 9 and
14 hold the one step the shape table names. Rows that spec 0027, 0049
or 0068 already names keep their catalogue entries. Fusers are marked
linear, non-linear or unsupervised.

| Slot | Method | Source |
|---|---|---|
| 2 | Porter stemmer | Porter, 1980 |
| 2 | Lovins stemmer | Lovins, 1968 |
| 2 | Paice/Husk (Lancaster) stemmer | Paice, 1990 |
| 2 | Krovetz inflectional stemmer with dictionary | Krovetz, 1993 |
| 2 | S-stemmer (plural removal only) | Harman, 1991 |
| 2 | Dictionary lemmatisation (WordNet morphology) | Miller, 1995 |
| 2 | Character n-gram tokenisation | Cavnar and Trenkle, 1994 |
| 3 | Retrieval similarity (dense bi-encoder) | spec 0027; Reimers and Gurevych, 2019 |
| 3 | Lexical match (BM25) | Robertson and Zaragoza, 2009 |
| 3 | Title overlap (Jaccard of word sets) | spec 0027; Jaccard, 1912 |
| 3 | TF-IDF cosine | Salton and Buckley, 1988 |
| 3 | Pivoted length normalisation | Singhal, Buckley and Mitra, 1996 |
| 3 | BM25F over title, first post and replies | Robertson, Zaragoza and Taylor, 2004 |
| 3 | Query likelihood, Dirichlet smoothing | Ponte and Croft, 1998; Zhai and Lafferty, 2001 |
| 3 | Divergence from randomness (PL2) | Amati and van Rijsbergen, 2002 |
| 3 | Information-based model (log-logistic) | Clinchant and Gaussier, 2010 |
| 3 | Axiomatic retrieval (F2EXP) | Fang and Zhai, 2005 |
| 3 | Sequential dependence model | Metzler and Croft, 2005 |
| 3 | Term proximity scoring | Tao and Zhai, 2007 |
| 3 | Character n-gram profile similarity | Damashek, 1995 |
| 3 | Longest common subsequence overlap (ROUGE-L) | Lin, 2004 |
| 3 | Latent semantic indexing | Deerwester et al., 1990 |
| 3 | Topic model similarity (LDA) | Blei, Ng and Jordan, 2003 |
| 3 | Explicit semantic analysis | Gabrilovich and Markovitch, 2007 |
| 3 | Soft cosine | Sidorov et al., 2014 |
| 3 | Smooth inverse frequency embeddings | Arora, Liang and Ma, 2017 |
| 3 | Word Mover's Distance | Kusner et al., 2015 |
| 4 | Entity agreement | spec 0027 |
| 4 | Keyphrase agreement | spec 0027 |
| 4 | TextRank | Mihalcea and Tarau, 2004 |
| 4 | TopicRank | Bougouin, Boudin and Daille, 2013 |
| 4 | MultipartiteRank | Boudin, 2018 |
| 4 | PositionRank | Florescu and Caragea, 2017 |
| 4 | Topical PageRank | Liu et al., 2010 |
| 4 | RAKE | Rose et al., 2010 |
| 4 | YAKE | Campos et al., 2020 |
| 4 | KP-Miner | El-Beltagy and Rafea, 2009 |
| 4 | KEA | Witten et al., 1999 |
| 4 | C-value / NC-value termhood | Frantzi, Ananiadou and Mima, 2000 |
| 4 | Entity linking overlap | Ferragina and Scaiella, 2010 |
| 4 | Link-based entity relatedness | Milne and Witten, 2008 |
| 5 | Graph relevance | spec 0049 |
| 5 | Personalised PageRank from the source thread | Haveliwala, 2002; Jeh and Widom, 2003 |
| 5 | SimRank | Jeh and Widom, 2002 |
| 5 | Common neighbours | Newman, 2001 |
| 5 | Jaccard of neighbourhoods | Liben-Nowell and Kleinberg, 2007 |
| 5 | Salton (cosine) index | Salton and McGill, 1983 |
| 5 | Hub promoted index | Ravasz et al., 2002 |
| 5 | Leicht–Holme–Newman index | Leicht, Holme and Newman, 2006 |
| 5 | Preferential attachment | Barabási and Albert, 1999 |
| 5 | Adamic–Adar | Adamic and Adar, 2003 |
| 5 | Resource allocation | Zhou, Lü and Zhang, 2009 |
| 5 | Local path index | Lü, Jin and Zhou, 2009 |
| 5 | Katz | Katz, 1953 |
| 5 | Graph distance (negated shortest path) | Liben-Nowell and Kleinberg, 2007 |
| 5 | Average commute time | Fouss et al., 2007 |
| 5 | Local random walk | Liu and Lü, 2010 |
| 5 | Matrix forest index | Chebotarev and Shamis, 1997 |
| 5 | CAR index | Cannistraci, Alanis-Lobato and Ravasi, 2013 |
| 5 | Hierarchical random graph | Clauset, Moore and Newman, 2008 |
| 5 | Stochastic block model reliability | Guimerà and Sales-Pardo, 2009 |
| 5 | Structural perturbation method | Lü et al., 2015 |
| 5 | Matrix factorisation for links | Menon and Elkan, 2011 |
| 5 | Spectral similarity (Laplacian eigenmaps, job 276) | Belkin and Niyogi, 2003 |
| 5 | Diffusion maps | Coifman and Lafon, 2006 |
| 5 | DeepWalk | Perozzi, Al-Rfou and Skiena, 2014 |
| 5 | LINE | Tang et al., 2015 |
| 5 | GraRep | Cao, Lu and Xu, 2015 |
| 5 | HOPE | Ou et al., 2016 |
| 5 | Structural identity embedding | Ribeiro, Saverese and Figueiredo, 2017 |
| 5 | GraphWave | Donnat et al., 2018 |
| 5 | VERSE | Tsitsulin et al., 2018 |
| 5 | ProNE | Zhang et al., 2019 |
| 5 | FastRP | Chen et al., 2019 |
| 5 | Poincaré embeddings | Nickel and Kiela, 2017 |
| 6 | Acceptance prior (job 94) | spec 0027 |
| 6 | Rejection-prone target (job 97) | spec 0027 |
| 6 | Beta–binomial empirical Bayes | Casella, 1985 |
| 6 | Hierarchical partial pooling (forum, sub-forum, target) | Gelman et al., 2013 |
| 6 | Wilson score lower bound | Wilson, 1927 |
| 6 | Inverse propensity weighted acceptance | Joachims, Swaminathan and Schnabel, 2017 |
| 6 | Cascade model of decisions | Craswell et al., 2008 |
| 6 | User browsing model | Dupret and Piwowarski, 2008 |
| 6 | Dynamic Bayesian network click model | Chapelle and Zhang, 2009 |
| 6 | Bradley–Terry strength | Bradley and Terry, 1952 |
| 6 | Matrix factorisation of decisions | Koren, Bell and Volinsky, 2009 |
| 10 | Anchor quality (jobs 51, 296) | spec 0027 |
| 10 | Agreement with existing anchors | Craswell, Hawking and Robertson, 2001 |
| 10 | Keyphraseness | Mihalcea and Csomai, 2007 |
| 10 | Commonness | Milne and Witten, 2008 |
| 10 | Query clarity of the anchor | Cronen-Townsend, Zhou and Croft, 2002 |
| 10 | Simplified clarity and average IDF | He and Ounis, 2004 |
| 10 | Pointwise mutual information | Church and Hanks, 1990 |
| 10 | Log-likelihood ratio collocation strength | Dunning, 1993 |
| 11 | Min–max per batch | Lee, 1997 |
| 11 | Z-score per batch | Montague and Aslam, 2001 |
| 11 | Sum-to-one per batch | Montague and Aslam, 2001 |
| 11 | Rank-based normalisation | Renda and Straccia, 2003 |
| 11 | Mixture-model CDF normalisation | Manmatha, Rath and Feng, 2001 |
| 12 | Logistic fusion (linear) | spec 0027 |
| 12 | Ranking SVM (linear) | Joachims, 2002 |
| 12 | AdaRank (linear) | Xu and Li, 2007 |
| 12 | ListNet (linear) | Cao et al., 2007 |
| 12 | ListMLE (linear) | Xia et al., 2008 |
| 12 | Coordinate ascent (linear) | Metzler and Croft, 2007 |
| 12 | SVM-MAP (linear) | Yue et al., 2007 |
| 12 | PRank (linear) | Crammer and Singer, 2001 |
| 12 | SoftRank (linear) | Taylor et al., 2008 |
| 12 | LambdaMART (non-linear) | spec 0027; Burges, 2010 |
| 12 | RankNet (non-linear) | Burges et al., 2005 |
| 12 | RankBoost (non-linear) | Freund et al., 2003 |
| 12 | MART, pointwise (non-linear) | Friedman, 2001 |
| 12 | Random forest, pointwise (non-linear) | Breiman, 2001 |
| 12 | McRank (non-linear) | Li, Burges and Wu, 2007 |
| 12 | Additive model with pairwise terms (non-linear) | Lou, Caruana, Gehrke and Hooker, 2013 |
| 12 | Factorisation machines (non-linear) | Rendle, 2010 |
| 12 | CombSUM (unsupervised) | Fox and Shaw, 1994 |
| 12 | CombMNZ (unsupervised) | Fox and Shaw, 1994 |
| 12 | Reciprocal rank fusion (unsupervised) | Cormack, Clarke and Büttcher, 2009 |
| 12 | Borda fusion (unsupervised) | Aslam and Montague, 2001 |
| 12 | Condorcet fusion, scored by Copeland's count of pairwise wins with ties broken by destination id (unsupervised) | Montague and Aslam, 2002; Copeland, 1951 |
| 13 | Platt scaling | Platt, 1999 |
| 13 | Isotonic regression | Zadrozny and Elkan, 2002 |
| 13 | Histogram binning | Zadrozny and Elkan, 2001 |
| 13 | Bayesian binning into quantiles | Naeini, Cooper and Hauskrecht, 2015 |
| 13 | Ensemble of near-isotonic regressions | Naeini and Cooper, 2016 |
| 13 | Beta calibration | Kull, Silva Filho and Flach, 2017 |
| 13 | Venn–Abers predictors | Vovk and Petej, 2014 |
| 13 | Scaling-binning calibrator | Kumar, Liang and Ma, 2019 |
| 13 | Spline calibration | Gupta et al., 2021 |

- Rank-based steps are rank-based normalisation and the rank-based
  fusers: reciprocal rank fusion, Borda fusion and Condorcet fusion.
  The first three give equal values equal rank, the lowest rank of
  their tied group, as spec 0067 sets; Condorcet breaks ties by
  destination id.
- Each score normaliser's catalogue entry states the value it gives
  when a batch's column has no spread, such as one row or one value in
  every row, and when its sum is not above zero.
- Each agreement step (slot 4) measures agreement between the two
  sides by the measure its entry names; that measure is part of the
  procedure, and a changed measure is a setting.
- Every graph step (slot 5) reads a stored ganit output (spec 0049);
  a randomly started embedding takes its seed from its ganit job.
- Job 33's calibration (spec 0027) is one of the slot 13 procedures,
  and its catalogue entry is that row's entry. The live service keeps
  serving job 33's active calibration (spec 0027); changing which slot
  13 procedure job 33 serves live needs an amendment to spec 0027,
  never a setting.

### Fusion and training

1. Every fuser training and calibration fit of slots 12 and 13 learns
   from one source of labels: seeded moderator decisions that are part
   of the probe set, drawn from the probe seed and versioned with it.
   None of them reads real decisions. Steps of slots 5 and 6 read their
   own ganit outputs, as their entries name; in a qualified run, the
   outputs that slot 6 reads are fitted on the same seeded decisions. In
   a qualified run every ganit output a step reads, graphs included, is
   built from the seeded probe forum, which holds no accepted
   suggestions, so no step reads a real decision through a link.
2. `qualify` asks for each fit, including any fit a slot 11
   normaliser makes, such as the mixture model, through the public
   interface of its owning ganit job: job 45 (Shiksha) for fusers, job
   33 for calibration, and the normaliser's own job for its fit; plain
   per-batch arithmetic, such as min–max or ranks, is not a fit.
   `shodhan.rank` only applies the results; the ranking module computes
   no statistics itself (spec 0027).
3. Job 45 trains each linear fuser once for each score normaliser of
   slot 11, on spec 0027's terms, with every weight positive and inside
   its bound: 45 trainings in all. Each training reads the three caution
   columns as written, as the fuse step does, and the score views of
   every other relevance column any step of this shape writes, as
   written from the raw text with no text normaliser. Every chain of
   this shape has a text normaliser and uses those weights, not weights
   trained on its own normalised text. A chain's fuse step uses only its
   own columns' weights from the training for its normaliser,
   renormalised to sum to one; a chain with any renormalised weight
   outside its spec 0027 bound gives no result and never falls back. A
   chain whose own weights sum to a value that is not a finite number
   above zero gives no result and never falls back. No chain has a
   training of its own for a linear fuser.
4. A non-linear fuser is trained once for each column set, where a
   column set is named by a chain's prefix: its normalisers change the
   values the fuser learns from, so they are part of the set. These
   trainings run only inside the qualified run, within spec 0064's
   execution time limit. A run whose trainings, with every other cost,
   would not fit is refused before it starts (spec 0067, "Performance").
   Its trainings read the caution columns as written, as the fuse step
   does.
5. An unsupervised fuser reads no weights and is never trained. Every
   fuser and score normaliser reads each column turned by its direction
   (spec 0068, "Output kinds and probe sets", item 4). Each fuser is
   trained, or built, to be monotone in each caution column: raising one
   caution on a row never raises its fused score. Each calibrator's
   fitted map never decreases as the score rises. No score normaliser's
   view rises as its turned column falls, including the values each
   entry states for a column with no spread or a sum not above zero. So
   a caution only ever lowers a destination. A fuser that is not
   monotone in a caution column on a probe batch, trained or not, such
   as CombMNZ on signed score views, gives no result for that chain. A
   fit that cannot meet this counts as not fitted, and so does a slot 11
   normaliser fit, such as the mixture model, that fails or meets an
   empty batch. A pairwise or listwise fuser whose seeded decisions give
   it no pairs counts as not fitted. So does any fuser training whose
   seeded decisions hold only one class.
6. Each calibrator in slot 13 is fitted for each chain, inside the run's
   time limit, since each chain's fused score has its own spread. A
   calibration fitted on rows of only one class counts as not fitted. A
   chain whose weights, calibration or slot 11 fit are not fitted gives
   no result and never falls back to other weights, another calibration
   or another fit (spec 0068). A chain with a rank-based fuser
   (reciprocal rank fusion, Borda, Condorcet) gives no result for a
   batch where a caution is set on every row, since ranking would erase
   that caution; a caution column that is clear on every row erases
   nothing and gives a result. A caution counts as set on a row when its
   value is above zero. Every chain gives no result for an empty probe
   batch.
7. A fuser's procedure is its training objective and how it scores;
   two linear fusers that both score by a weighted sum are two steps
   because they learn by different objectives. Weights a fuser learns,
   or that are changed by hand, and fitted calibrations are settings,
   never steps: other weights, same chain. Logistic fusion keeps its
   trained intercept; renormalising touches only the weights.

### What can make chains behave the same

Many chains will be removed as behaving the same (spec 0067, "Proving
chains differ"):

- The calibrated probability is read only by the gate, so a calibrator
  changes a chain only through which rows pass; calibrators that pass
  the same rows give one output ordering.
- A score normaliser that keeps the order of its column and adds no
  ties changes a chain only through how the fuse step combines
  columns. With a rank-based fuser (reciprocal rank fusion, Borda,
  Condorcet) every such normaliser gives the same output, and only one
  of those chains counts. Sum-to-one on a column whose sum is not above
  zero, and a mixture-model normaliser that rounds values into ties,
  do not keep that order, so they are judged like any other step.
- A text normaliser changes only word steps. Where it moves no word
  step's column order, its chains match their neighbours.
- A chain whose other steps are all steps of spec 0068's link ranking
  shape, and whose two normaliser steps change little in that chain,
  is a near member of that shape's chain with those steps left out (spec
  0067, "Proving chains differ", item 5). The two fuse and calibrate
  differently, so it is removed only when the rank-correlation check
  finds them alike.

So the probe set for a feature table and a score, which this shape
shares with spec 0068's link ranking shape, must hold batches of each
of these, so that every slot can matter:

- rows whose calibrated probability falls near the gate's thresholds;
- near-duplicate destinations;
- misspelt and inflected source phrases and titles;
- long threads with many replies, for length and proximity steps;
- columns with outliers and skew, so the score normalisers differ.

### Slot sizes and their product

With every row admitted, as plain arithmetic:
1 × 7 × 20 × 14 × 34 × 11 × 1 × 1 × 1 × 8 × 5 × 22 × 9 × 1 =
5,805,676,800 chain ids. This only bounds the indexes (spec 0067,
"Chain identity", item 2). No product, estimate or formula over shape
sizes supplies a count (spec 0067, "Proving chains differ", item 6):
the count comes only from `qualify`, from each chain's own
runner-signed result.

### Shape sheets and admission

1. The shape is one entry in `catalogue/shapes/` (spec 0067), naming
   each step by method id, admitted only when every step it names has
   a catalogue entry, and approved with its scope area's batch.
2. Every row above needs its own catalogue entry under spec 0055, with
   all four conditions of "One identity per procedure", item 5: a
   sourced difference, a small example, a useful role measured on its
   own and a reviewer's note. A row that fails admission is left out of
   the sheet, so its slot is smaller; the shape lists only admitted
   methods, and its product shrinks with it.
3. One method sits in one slot only (spec 0067, "Shapes", item 5):
   n-gram tokenisation (slot 2) and n-gram profile similarity (slot 3)
   are two entries.
4. `shodhan.rank` declares prefix caution, spam risk and self-promotion
   caution, as spec 0068 sets; admission refuses this shape if any
   chain does not carry all three.
5. The shape is frozen once admitted. A method admitted later makes a
   newer shape, and this spec is then amended with that shape.

## Limits

- The shape lists no setting, preset or playbook as a step.
- Steps that need a pretrained neural model beyond spec 0023's embedder
  and reranker are left out; small models trained in the run on the
  probe set, such as RankNet, or by a ganit job on the forum's own
  data that reads no decisions, such as DeepWalk, are allowed.
  Methods a reviewer would likely call a setting of a row above, such
  as BM25+, are left out.
- Steps that read outside data (WordNet, the Krovetz dictionary, an
  encyclopaedia for explicit semantic analysis, entity linking and
  link-based relatedness) pass spec 0055's rights area or fail
  admission. Steps that need word vectors or topic models read ones a
  ganit job builds from the forum's own text, never outside weights.
- Phrase expansion, passage aggregation, near-duplicate removal and
  diversification are left to later shapes.
- The live service never runs a chain of this shape: `shodhan.rank`
  fuses every ready relevance method, as spec 0027 sets.
- Every row this spec adds that no earlier spec serves live runs only as
  a step of a method chain of this shape; the live service never fuses
  it on its own. Making one live needs an amendment to spec 0027, never
  a setting.

## Performance

- Chains run in index order, sharing work on shared first slots (spec
  0067); graph and prior steps read stored ganit outputs.
- With every row admitted, the prefixes number
  1 × 7 × 20 × 14 × 34 × 11 × 1 × 1 × 1 × 8 × 5 = 29,321,600. Each of
  the eight non-linear fusers trains once per prefix: 234,572,800
  trainings. The nine linear fusers train once per score normaliser,
  45 trainings, and the five unsupervised fusers not at all.
- With every row admitted, those trainings and the calibration fits
  will almost surely exceed spec 0064's execution time limit, so a run
  over the whole shape is refused before it starts, and none of its
  chains, linear or not, gets a result from that run. Chains of this
  shape count only from runs that fit: a shape with smaller
  admitted slots, or a later, narrower shape for the non-linear
  fusers.
- Each chain also fits its calibrator once (Fusion and training, item
  6): with every row admitted, 5,805,676,800 fits, the most fits of
  any kind in the shape.
- A run keeps at most one training per non-linear fuser and one fit
  at a time. Chains that share a prefix sit next to each other in
  index order (spec 0067), so each training and fit is dropped once its
  chains finish and none is stored; anything kept for replay counts in
  spec 0067's scratch-space refusal.
- Slots 12 to 14 run once per chain. Their measured cost, the
  measured cost of one training per fuser and one calibration fit, any
  fit a slot 11 normaliser asks for, the 45 linear trainings kept for
  the whole run, and the keys this shape writes, including its
  placings of spec 0068's link ranking slots, go into spec 0067's time
  and scratch-space refusals, worked out with checked arithmetic.
- The run waits in Dell's one heavy-job queue (AGENTS.md section 11).

## How it is tested

Rust tests in `sutradhar-methods`, `shodhan` and `qualify`, with
made-up forums:

- `wide_shape_sheet_passes_admission`: each step reads the kind of the
  slot before it, no slot lists a setting, no method is in two slots.
- `sheet_naming_unadmitted_method_is_refused` and
  `sheet_without_a_protection_is_refused`.
- `text_normaliser_adds_one_view_and_no_column` and
  `text_normaliser_changing_a_column_fails`.
- `text_view_equal_to_raw_text_fails`,
  `score_step_with_every_view_equal_fails`,
  `score_view_equal_on_one_column_passes` (min–max on a column of 0
  and 1) and `score_view_not_finite_fails`.
- `word_step_reads_latest_text_view`, `vector_step_ignores_text_view`,
  `fuse_reads_score_views` and `gate_reads_reranker_column_not_view`.
- `pairwise_fuser_with_no_pairs_is_not_fitted`,
  `linear_fuser_trains_on_raw_text_score_views` and
  `fuser_and_normaliser_read_columns_turned_by_direction`.
- `protection_steps_read_raw_text`,
  `raw_text_model_step_reads_raw_text` and
  `word_step_reads_statistics_built_for_its_view`.
- `rank_fuser_with_caution_set_on_every_row_gives_no_result`, and the
  same chain with that caution clear on every row gives a result.
- `training_and_fusion_read_cautions_alike`,
  `caution_on_every_row_reaches_the_gate` and
  `one_class_fuser_training_is_not_fitted`.
- `live_calibration_procedure_is_not_a_setting` and
  `new_rows_are_never_fused_live`.
- `caution_never_raises_fused_score`: raising one caution on a probe
  row never raises the fused score or the calibrated probability, for
  every score normaliser, fuser and calibrator.
- `renormalised_weight_outside_bound_gives_no_result` and
  `empty_probe_batch_gives_no_result`, the second also for a chain
  whose mixture-model normaliser meets the empty batch.
- `linear_fuser_zero_or_non_finite_weight_sum_gives_no_result` and
  `unfitted_calibration_gives_no_result`: neither falls back.
- `fits_read_only_seeded_probe_decisions`,
  `logistic_fusion_keeps_intercept` and `calibrator_fitted_per_chain`.
- `linear_fuser_uses_own_weights_renormalised`,
  `linear_fuser_never_trains_per_chain` and
  `unsupervised_fuser_reads_no_weights`.
- `nonlinear_fuser_trains_once_per_prefix`: chains with one prefix
  share a training; chains whose normalisers differ do not.
- `run_refused_when_trainings_exceed_time_limit`, before any step runs.
- `rank_fuser_chains_differing_in_order_keeping_normaliser_match`,
  `sum_to_one_with_sum_not_above_zero_is_judged_alone` and
  `calibrators_with_same_pass_set_match` keep only the lowest; a batch
  near the gate keeps two calibrators apart.
- `inert_normaliser_chain_compared_with_0068_chain`: the two are
  paired, and one is removed only when the check finds them alike.
- `normaliser_value_used_for_column_with_no_spread`: a one-row batch
  through min–max and z-score gives the value each entry states.
- `wide_chain_id_round_trips_fourteen_steps`; index 0 and an index
  above the chain count are refused.
- `count_comes_only_from_signed_results`: a shape with no results
  counts no chain, whatever its size.
- `probe_set_holds_each_batch_kind` and
  `ranking_subtotal_counts_wide_chains`.

## Changes from the source

Written fresh from specs 0027, 0049, 0055, 0067 and 0068 and a
research list of published methods for each slot.
