# SPEC-009 part 3: Matching and anchors

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in [the specification
decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Requirements: as in [part 1](matching.md). Part 2 is [reranking,
suggestions, the two queues and the 30-second target](matching-2.md);
part 4 is [failure, privacy, performance and testing](matching-4.md);
part 5 is [acceptance](matching-5.md); part 6 is [the owner's
decisions](matching-6.md).

## Design, continued

### The comparison test before release (R100)

Purpose: before release, on Dell, the chosen set (bge-m3 with
gte-reranker-modernbert-base) is compared with the challengers the owner
named: all-MiniLM-L6-v2, bge-small-en-v1.5 and the ms-marco-MiniLM
reranker, tested in both its 6-layer and 12-layer sizes because the
owner did not name one (P-MA02). The owner's answer to Q-MA04 adds the
earlier choices that load safely, from data-only files without code
from their repositories: arctic-embed-s, and potion-base-8M in a
first-pass stage. jina-reranker-v1-tiny-en joins only if it loads
safely; its own instructions appear to need repository code, so it is
expected to be excluded, with the reason recorded. Every model in the
test runs in the production forms where they apply: bge-m3 through
ONNX Runtime and spaCy's `en_core_web_lg` (R120). How challengers are
pinned is SPEC-010's MP-02.

Data (P-MA09):

- Link reconstruction on the forum's own guest-visible English content
  on Dell. Internal links that members wrote are hidden in a test copy
  (the words stay, the link is removed), and each set of models tries
  to recover the destination and the anchor.
- At least 2,000 links, sampled at random with a fixed seed from the
  50,000-thread tier and spread across the subgroups below. They are
  kept apart, by thread and by time, from any data used for tuning; the
  unseen final-assessment data of R059 is never used.
- Subgroups: thread type, post length, destination popularity (inbound
  links) and forum section. Reconstruction favours popular threads
  (Q148), so popularity is reported separately.

Frozen protocol: before any score is computed, a protocol file records
the models and revisions, the sampling, the metrics, the subgroups, the
interval method and the winning rule, and its SHA-256 checksum is
recorded. A run whose protocol checksum differs is invalid. Difficult
cases are never dropped, and the sample is never redrawn to get a pass
(R060).

Metrics: recall at 5 is the main one. Also reported: recall at 1 and 3,
mean reciprocal rank (how high the right answer sits on average),
retrieval recall at 50 (whether the right destination reaches the
reranker), anchor agreement (the suggested anchor overlaps the member's
own link text), 95th- and 99th-percentile latency (the service level
is 99% within 30 seconds; R124), peak memory and throughput.

Comparisons by stage:

- Embedding: each embedding model, with the chosen reranker on top.
- Reranker: each reranker on the same bge-m3 top-50 lists.
- First pass, with the earlier set (Q-MA04): potion-base-8M against
  potion-retrieval-32M on shortlist recall at 100 (whether the right
  destination is among the first pass's 100 nearest threads) and sweep
  speed.
- A line with no reranker is reported for information.

Shorter windows (P-MA01): each challenger runs at its own window, which
is 512 tokens or fewer. Pieces are re-split to fit it under the same
rules, and reranker inputs keep the same order and proportions: an
eighth of the window for the query side, three-eighths for pieces and
the rest for summary text. R109's refusal of short windows applies to
community models, not to this test.

"Clearly wins", fixed before scoring (Q-MA03, as the owner chose: at
least +2 points). A challenger clearly wins only if all of these hold:

1. the lower end of the paired 95% interval for its recall at 5 minus
   the chosen model's is at least +2 percentage points (the interval
   comes from re-sampling the test cases many times);
2. no subgroup is worse with 95% confidence;
3. anchor agreement does not get worse: the measured difference is zero
   or better, and the lower end of its 95% interval is no lower than
   −1 point;
4. it keeps within the service's memory cap (6 GB, or 8 GB once Dell
   measurements confirm the estimate; R121) and the 30-second target
   on the reference hardware of 4 cores and a SATA solid-state disk
   (R124).

Why condition 3 allows −1 point: R060 forbids anchor agreement to
regress. With about 2,000 test links, the interval around a measured
difference is roughly 1 to 2 points wide on each side even when two
models are equally good, so demanding that the whole interval sit above
zero would reject a challenger that is truly no worse. The rule
therefore requires the measured difference itself to be zero or better
and lets only the interval's lower end dip, by at most 1 point. That is
a measurement tolerance, not permission to get worse. SPEC-010's MD-08
uses the same test for retrained models. The bar for recall differs: a
retrained version of the chosen model needs only to be better (lower
end above zero), whereas a challenger must clearly win (at least +2
points), because R100 replaces the owner's chosen model only on a clear
win.

Results: the report names exact revisions and stays on Dell; a public
summary needs a privacy review (R075). A clear win is only a proposal:
replacing a model needs the owner's approval (R027, R100), and a
challenger whose window is under 1,024 or 8,192 tokens would also need
the owner to change R101 or R103. The test runs within the lab's 9 GB
ceiling, and the chosen set is also measured under the production cap
(6 GB, or 8 GB once confirmed; R121).

### MS MARCO-trained models (R099)

Each model's admission record states whether its published training
data includes MS MARCO and quotes that data's terms. The owner allows
such models, so MS MARCO use is recorded and is never a reason to
refuse. This is the owner's allowance, not legal clearance; the rights
record (R067) keeps it.

### Data stored, and uninstall

- Anchor candidates per source post, with spans, sources and scores,
  rebuilt when the post changes; phrase vectors, kept until it changes.
- Suggestion records with their reranker scores, model and index
  versions, coverage and any re-score or replacement, and the reasons
  candidates were rejected (part 2).
- The comparison protocol, results and evidence, on Dell only.

All of it is derived from guest-visible content and purged with that
content (R052). SPEC-010 owns the uninstall table under R050 as
amended; matching's stores fall as follows:

- Always deleted, with no keep option or export step, as embeddings:
  phrase vectors.
- Keeping R050's export offer and keep-or-delete choice, as in
  SPEC-010's table: anchor-candidate records, suggestion records (with
  their scores, states and model keys) and rejection reasons.
- Never on a forum host: the comparison evidence.

### Interfaces between PHP and Python

Transport, signing and schemas belong to F004 (R043, R077).

- `SuggestionUpdate` (Python to PHP): one source post's new suggestion
  records, sent the moment that post completes, and later state
  updates, re-scores and replacements (R123). It was `SuggestionBatch`
  in the draft; suggestions are never sent per batch. Once PHP has
  stored them, the `suggestions` topic of SPEC-011's live stream tells
  open review pages (part 2). Progress goes through SPEC-011.
- `VisibilityCheck` (Python to PHP): source and destination ids to
  confirm guest visibility just before display; PHP answers with their
  current state (R013).
- `MemberNameCheck` (Python to PHP): candidate people's names,
  normalised; PHP answers which equal a member's user name, and those
  are dropped. PHP never sends Python the member list.
- `Readiness` (Python, read by PHP and the health page): ready, and
  ready or not for suggestions, with reason and code, including "model
  update in progress" while new suggestions pause (R106, R122;
  SPEC-010).
- Queue figures (Python, read by the review page and the health page):
  live and backfill queue depth and oldest age, processing rate, and
  thread-level and per-post index coverage, as metrics (R127) and on
  SPEC-011's live stream (R123).
- `ReviewDecision` (PHP to Python): approve, reject or remove with any
  reason, for learning (R018) and retraining (SPEC-010).
- At approval, F009 re-checks content, offsets and visibility in PHP
  (R013, R016).

### Settings and defaults

Fixed by the owner's decisions, not settings: exact search; the top 50
reranked; 3 best pieces; the 8,192-token window; the four anchor
sources; the allowed entity types; merged keyphrases; moderator
approval of every link; suggestions never held, the thread-level index
first, the two queues and the pause during an embedder rebuild (R122,
R123).

Settings, with proposed defaults (tuned only within approved bounds,
R024):

- `matching.search.hits_per_piece` (1,000), and
  `matching.score.piece_weight` (0.7) with
  `matching.score.summary_weight` (0.3) (P-MA06);
- `matching.target.reply_margin` (0.05; P-MA03);
- `matching.rerank.min_score` and `anchors.min_score` (calibrated on
  Dell before release; shadow only until then);
- `matching.rerank.first_post_share` (0.5; P-MA08);
- `anchors.score.weights` (similarity 0.6, title words 0.3, keyphrase
  0.1), `anchors.keyphrase.weights` (statistical 0.5, embedder 0.5),
  `anchors.keyphrase.max_per_post` (20) and `anchors.length` (3 to 80
  characters, at most 8 words) (P-MA07);
- `anchors.alternatives_per_phrase` (5, never above R011's five);
- `anchors.people.enabled` (true; Q-MA05, strict safeguards);
- `matching.reverse.sources_per_destination` (200; P-MA10);
- `matching.rejections.keep_days` (30; P-MA11);
- `matching.rescore.replace_margin` (calibrated on Dell before release
  with the minimum scores; part 2 and the recorder's notes in part 6).
