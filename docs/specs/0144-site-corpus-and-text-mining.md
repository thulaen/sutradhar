# 0144: Local corpus and text mining

## Purpose and boundary

This sub-spec belongs to the
[Unified Site Audit suite](0140-unified-site-audit-suite.md).

The owner can inspect language, wording and changes across a fixed set
of eligible pages from one owned site. The Text Mining Explorer shows
what each operation covered, the version it used and why a result is
missing. Suggestions and scores are advisory. The interface and live
link-ranking language stay British English. This suite creates no
ranking input, training set for ranking, site edit, new crawl, provider
lookup, cloud job or network route.

One current owner approval enables count-safe corpus analysis for the
site. It names the site, eligible stored source classes and admitted
operations. A job still binds an exact inventory and current authority.
The approval grants no text access, classifier admission, export,
individual profiling or permission to recover suppressed values.
Read and export use the stricter current source, recipient, privacy and
role rules. An approved locale snapshot is a supplied file with exact
own-site, source, collection, revision, capture and rights evidence.
The file gives no live locale request, staging login or whole-site
coverage. The existing 0044/0115/0116 producers own crawl observations.
Use 0026 for text features, 0024/0080 for model admission, 0104 for
fenced workers and 0043 for typed tables. 0123 owns writing reports.

## Ingress and language

Use the admitted local scrubbed editorial-text projection of a page
revision. A supplied file uses a bounded, versioned, inert manifest and
projection. Verify its exact approved bytes and canonical identity
before admission. Decode strict UTF-8. Reject invalid bytes, ambiguous
identity, an unknown manifest version or a digest mismatch. Treat every
string as data. HTML extraction, boilerplate and layout coverage come
from 0115. An unsupported or incomplete projection stays distinct
from observed empty text. Respect visibility, reuse rights, opt-outs,
and 0082 and 0118. Exclude protected spans before deriving anything.

Keep immutable original projected UTF-8 bytes and separate analysis
keys. Offsets are zero-based half-open byte ranges on the exact
projection, never DOM indices or offsets into normalised strings.
The primary exact query uses NFC and case-sensitive matching. Treat a
case-folded, accent-insensitive, stemmed or lemma query as a different
admitted operator. Retain reversible source-range mapping for any
admitted transformation. Hold a hit without an unambiguous range.

Store declared and detected language, detector profile, confidence,
script, and resolved language separately per bounded range. Resolve
language only by an admitted policy. A CLDR likely subtag supplies a
resource lookup hint. It does not identify the author, market, or text.
Unknown or conflicting ranges give explicit unknown coverage. A page
can enter multiple language panes through disjoint ranges. No measure
silently substitutes another language or joins across range seams.

Bind each supported operation to exact language tag, script, Unicode
and segmentation profile, locale data, text projection, matching
policy, component versions, immutable digests and test manifest.
The first profile to qualify is British-English text. Each additional
profile qualifies operations independently. Default Unicode breaks
alone do not qualify dictionary-based languages. Run the exact
versioned Unicode conformance files and locale gold fixtures, and
measure Windows and supported Linux on the same synthetic corpus.
Packaged spaCy may provide tokens, sentences, lemmas and labels only
for capabilities proved under 0024/0026. No unqualified package or
model is assumed. Store and display unknown or unavailable.

## Records and read state

Each record has explicit type/schema version, site and project.
Store protected records per site under 0108 and 0118. Identifiers
in telemetry, progress and ordinary audit are opaque and allowed by
their owning safe schema. No page text, terms, names, URLs or excerpt
digests in telemetry. Register each managed spill, index, manifest and
copy under 0030. Use 0032 for erasure and 0019 for job receipts.

| Record | Required identity and facts |
| --- | --- |
| Corpus manifest | Site, project, corpus revision, approved source snapshot set and sorted inventory digest, scope and purpose, approval reference, source/authority/erasure generations, eligible projection/locale policy digests, supported operation profile digests, limits, expiry |
| Inventory item | Ordinal, opaque canonical page ID, source revision and projection digest, evidence and rights references, exclusion/partial/eligible disposition, safe reason and approved section ID |
| Language range | Item and stable range ID, byte bounds, declared/detected/resolved tag and script, resolution evidence/profile and coverage |
| Operation profile | Operation and algorithm version, input projection, locale, dictionaries and model artefact digest, case and boundary rules, options, minimum support, estimates and constraints, quality evidence, profile hash |
| Shard receipt | Corpus, operation, attempt, authority/erasure/job fence, half-open ordinal range, accepted and omitted counts, sorted key range, schema, digest, verified sealed ledger pointer and checkpoint |
| Result seal | Corpus and operation profile, ordered input identities, canonical logical result digest and ordered physical shard lineage digest as separate fields, coverage digest, units, exact numerator/denominator, scope, rights, erasure generation and expiry |
| Dependency index | Opaque source and derivative IDs, record and generation, every shard/index/release/export that depends on the source, erasure target and safe reconciliation receipt |

States are exact, empty, partial coverage, unsupported, held, or
withdrawn. Empty means a complete eligible population was inspected
and no eligible item or token exists. A measured zero requires positive
known coverage and a valid denominator for rates. Partial means a
required source or outcome was unobserved or incomplete. An exact
whole-corpus statistic is unavailable if required evidence is missing.
An owner-approved smaller inventory gets a new manifest and labelled
denominator. An unsupported operation never returns zero. Hold
stale, unauthorised or irreconcilable data. Use one safe inaccessible
response for foreign, hidden, erased and unknown objects.

## Corpus arithmetic

A document is one approved canonical page for a run and language
pane. Reject conflicting mappings or two revisions of a page in the
same inventory. Let N be the number of complete eligible pages with
at least one eligible token in that pane and operation. Page length L
is the eligible token count after the operation's declared exclusions.
Let tf be count in the page, cf its corpus sum, df distinct matching
pages, and avgL the sum of lengths divided by N. Sum contributions
once per page across its disjoint ranges. Display coverage by source
and language where safe. A count across unlike profiles is separate.
Use checked integer counts and rational values until the formula needs
a real calculation. Use a pinned deterministic numeric profile. Seal
scores to six decimal places, round half to even, and order sealed ties
by opaque page ID then original start byte. Admission thresholds use
the actual value and its error bound, before display rounding.

| Operation | Deterministic initial profile or applicability condition |
| --- | --- |
| TF-IDF | Report tf, cf, df and N alongside page weight `tf × (1 + ln((N+1)/(df+1)))`. Site term view reports corpus counts and mean page weight over all N pages, including known non-hits. No score at N=0. |
| BM25 prominence | For an exact single term and eligible page, use `ln(1+(N-df+0.5)/(df+0.5)) × tf × (k1+1)/(tf+k1×(1-b+b×L/avgL))`. Proposed k1=1.2, b=0.75, with the same page projection for all N. A proved non-hit in a nonempty corpus scores zero. Site view reports mean per-page prominence over N. This operation is separate from retrieval and its settings. |
| N-grams | Count overlapping consecutive windows of 1 to 5 eligible word tokens within one sentence, paragraph and language range. No window crosses removed or covered-but-excluded text. Version rules for punctuation, stop words and stemming. Initial counts use NFC surface words; existing 0026's one-to-four-word keyphrase and stemmed gram profiles retain their separate rules. Five-word mode needs its own fixture and resource proof. |
| Collocation | Count ordered adjacent token positions in the same eligible sentence and range. For the pair x,y form O11 both, O12 only x first, O21 only y second and O22 neither. Compute `G²=2×sum(Oij×ln(Oij/Eij))`, with `Eij=row_i×column_j/total`, zero observations contributing zero. Hold zero marginals, fewer than 100 opportunities or fewer than three joint hits (proposed). Show counts and whether O11 exceeds E11. No significance or causal claim. |
| Keyphrases | Reuse the admitted 0026 YAKE one-to-four-word profile (top 20, ties by original position). Aggregate distinct pages emitting a keyphrase and eligible coverage. Show independent page evidence. Other languages require their own stop-list, segmentation and validation. |
| Concordance and KWIC | Find every eligible exact occurrence over cursor pages of at most 100 matches and 256 KiB per reply (proposed). Return page and byte range with up to eight eligible words on each side within the same authorised sentence and range. Recheck each page and reconstruct context on read. Do not bridge or expose masked text. Include grammatical usage only with the exact admitted POS/dependency span; otherwise label unavailable. |
| Main text and ratios | Use 0115 RN-27's admitted editorial region. Show component coverage. Content-to-HTML ratio uses projected editorial UTF-8 byte count divided by original eligible decoded HTML UTF-8 byte count from the same capture. Show unavailable for missing raw or incomplete projection, or an incomparable transformation whose editorial byte count can exceed its source. |
| Readability and style | Pin exact language, sentence and syllable definitions, source standard, sample policy and formula. With W words, S eligible sentences, Y syllables, C approved letters, F three-or-more syllable words with the Fog profile's documented exclusions, Reading Ease is `206.835−1.015 W/S−84.6 Y/W`; Flesch–Kincaid is `0.39 W/S+11.8 Y/W−15.59`; Coleman–Liau is `0.0588(100 C/W)−0.296(100 S/W)−15.8`; ARI is `4.71 A/W+0.5 W/S−21.43` with A the admitted character count. The candidate Fog word/sentence variant is `0.4(W/S+100F/W)`, with its exact source/exclusions qualification held until verified. Original SMOG approximation uses 10 non-overlapping consecutive sentences at the beginning, middle and end, counts words of at least three syllables and adds 3 to the integer square root of the nearest perfect square, ties upwards. For S<30 withhold that SMOG profile. Each grade keeps its source's unit; no combined grade. Other languages require a separate proved measure. |
| Lexical diversity and sentence facts | Report distinct surface word types divided by eligible token count in fixed consecutive 100-token windows (proposed). Average complete windows only and disclose discarded tail; fewer than 100 tokens gives unavailable. Keep windows within one language/range. Sentence length uses original eligible token counts with median and fixed admitted band counts. Passive ratio uses admitted passive clauses divided by eligible classified clauses; label undecided clauses and coverage. |

## Entity, topic and classifier projections

For admitted entity kinds, use exact 0026 typed spans and versioned
identity evidence. A homonym with no confirmed identity stays
unresolved. 0049 owns existing graph entities and scope. Propose
salience within a section as distinct eligible pages with that entity
divided by N for the section and language; graph edges use distinct
pages with both entities. Show denominators, eligibility and
suppression. Persons and potentially identifying attributes are
count-only in the corpus. Apply current privacy floors and
complementary suppression to all terms, snippets, filters, charts
and counts. Reuse only already admitted, count-safe 0026 entity
facts from an eligible source projection. New corpus models and
Python receive only proved safe projected ranges. Derive neither a
private name nor an unproved person label from a masked range. When
the existing producer cannot supply permitted facts or a safe span,
hold that view or operation. No new name-discovery or knowledge-base
route and no cross-site graph.

### Private name view

This is the separately approved, owner-only exception to 0026's
count-only reporting. It runs offline in the authenticated desktop
window, for an explicit owner-confirmed selection on an approved
staff-published article or staff-published own-site page with current
editorial use and rights. User contributions, replies and comments,
private material, member handles and IDs, contacts, inferred names
and guessed identities are ineligible. Public occurrence by itself
gives no grant. A disputed or ambiguous match is unavailable.

The admitted existing own-site published source must attest the
exact selected staff-authored span in its permitted projection. Hold
if source scrubbing removed the text or staff provenance is
unavailable. Never recover it from raw archive bytes. Exclude text
quoted from comments or other members even inside a staff article.
The preview shows only safe page and selection facts.

Bind the confirmation to exact site, owner session and grant,
page ID, publication and editorial rights revisions, source digest,
UTF-8 byte span, purpose and erasure generation. Verify that the
confirmed span is within the currently eligible published staff
content. No background prefetch, enumeration or automatic selection.
Recheck exact scope before showing it. A changed range or revision
requires a new confirmation. Show the approved name only in the
on-device visual view and accessibility text of that authorised
desktop session. Never put it in a file, cache, log, audit event,
export, MCP response, Python input, model prompt, public
accessibility projection or ranking input. Do not copy it to the
clipboard. Existing source custody stays with the source owner.
Clear the displayed and transient value on window lock or close,
withdrawal, erasure, grant loss or session end. Reopening never
restores the name. Only safe non-name confirmation metadata can
remain under the existing audit rules. Admission requires the
authorised-owner and refusal fixtures on each supported desktop
platform with no name in any forbidden sink.

### Local grouping and labelling

Semantic grouping candidate uses only independently admitted local
ONNX embeddings catalogued under 0024/0080. Pin context, pooling,
normalisation, dimension, numerics and exact corpus input. Each valid
page has one L2-normalised embedding per language pane; zero vectors
or truncated incompatible input are unknown. Proposed bounded profile:
2,000 pages, 768 dimensions, 8 centroids and at most 20 passes. Seed
first centroid with least page ID, next with farthest point from its
nearest seed using cosine distance and page ID ties. Assign to greatest
cosine similarity, break ties by seed ordinal, and re-normalise the
arithmetic mean of assigned embeddings on each pass. Hold empty
clusters and unstable assignments at the pass limit. Label clusters
from the top five distinct terms by summed corpus TF-IDF within the
cluster, with lexical ties. Keep input coverage and unknown members.

Statistical topic candidate fits non-negative factors W,H over the
same kind of versioned non-negative term-by-document matrix V.
Proposed limit: 2,000 documents, top 1,024 terms by df then term key,
8 factors, exactly 50 alternating Euclidean multiplicative updates.
Use positive initial factors deterministically derived from the
profile's pinned seed and cell coordinates. In ascending order and
fixed numeric precision, multiply H elementwise by
`(WᵀV)/max(WᵀWH,10⁻¹²)`, then multiply W elementwise by
`(VHᵀ)/max(WHHᵀ,10⁻¹²)` using the updated H.
Record residual norm, coverage, factors' relative weights and top five
eligible terms. Hold non-finite, zero or unqualified fits. The model
profile must pass a frozen residual, stability and interpretation
protocol. “Thin” is an observed eligible occupancy, “possible gap”
requires named reviewed reference evidence, and cluster membership
never proves query competition. Eligible observed query overlap uses
the existing 0044 F-0016 source and can remain unavailable.

Intent labels are informational, commercial comparison,
transactional and navigational. Tone uses the versioned approved
taxonomies and sentiment positive, neutral and negative only where
qualified. Hold out-of-taxonomy, ambiguous and mixed input with a
typed unknown or mixed reason. Page labels do not describe readers.
No labels for individuals or claims of user demand. Aggregate tone
and sentiment under the current suppression rules. Freeze exact
artefacts, dependencies, calibration, preprocessing, label map,
metric, threshold, populations and disjoint synthetic evaluation by
canonical or template family before scoring. Proposed initial gate:
100 labelled cases per class, macro F1 at least 0.80 including
abstentions as false negatives for their actual class, each class
recall at least 0.65 and eligible coverage at least 0.80. Admit
subgroups only with separately measured support and quality. Freeze
confidence and ambiguity thresholds per selected language/model
profile before evaluating its untouched holdout. Independent review
approves the exact profile and checks the resource release tests.
This proposal admits no model or taxonomy by itself.

## Duplicate and cross-crawl views

Reuse 0116 LS-13/36 SimHash and exact confirming word-group Jaccard
within the same eligible project. Jaccard is `|A∩B| / |A∪B|`
over distinct compatible shingles; empty union gives unavailable.
Keep the existing word-group, Hamming and Jaccard thresholds.
An optional MinHash candidate requires separate admission. Proposed
profile uses five-word shingles and 128 seeded, independently
specified hash permutations. Equal minima divided by 128 estimates
Jaccard. Record seeds/profile, sample and confidence. Confirm every
reported pair with exact Jaccard. Hold insufficient coverage or work
cap overflow and measure candidate recall against an exhaustive
synthetic oracle. No transitive inference or accusation of copying.

Same-own-site historical comparisons pin canonical mapping, source
revisions, profile/locale, availability, exact comparable scope and
extraction versions. Use 0116's streaming alignment and bounded
shortest insert/delete token diff with fixed earlier-old then
earlier-new ties. Compare exact original surface tokens plus their
offsets and show adds/removes without rewriting either snapshot.
Entity change lists require identical entity-resolution profiles and
exact evidence on both sides. Sentiment drift is the difference of
same-label proportions in percentage points, within identical
qualified profile and compatible population; show both denominators.
Model, language or baseline changes give unavailable. An absent or
erased capture never proves removal. A capped operand yields partial
coverage, never a complete diff.

## Resources, stop and erasure

Proposed design defaults for resource admission, measured before
release: 100,000 canonical pages or 2 GiB projected UTF-8 per corpus,
512 KiB or 64,000 eligible tokens per page, 64 pages or 4 MiB per
chunk, whichever limit arrives first. Read whole pages within chunk
bounds. When the next page cannot fit the current chunk, defer it
to the next. A single page over its cap yields incomplete coverage.
Use one active worker per site with 0019/0078 turn-taking. Request a
192 MiB CPU fold promise including memory-mapped working sets and
buffers. Reserve model and matrix working memory separately through
the same budget. Use 8 MiB sorted encrypted runs, at most eight
inputs per merge, and at most 2 GiB new and temporary managed bytes
per run. Charge old live data separately while still retained.
Wait for the current installation's tighter disk or memory promise.
No promise equates to a measured fit. Check on the admitted 8 and
16 GiB host profiles and all v1 supported platforms.

Poll stop at least every 256 tokens and before every bounded write
of up to 64 KiB. Target a safe yield within two seconds, pending
measurement under the existing watchdog. Only complete pages
contribute. Register exact owned staging before bytes. Write sealed
sorted shards and verify file identity and digest. Adopt each
verified pointer with the checkpoint, counts, intent and safe outbox
in one 0019 transaction under current job, rights, stop and erasure
fences. No visibility until the final complete seal. An uncertain
acknowledgement reconciles the original receipt before retry.
Canonical result hashing uses stable sorted logical rows under the
same fixed serializer and excludes chunk, attempt and worker IDs.
Keep actual physical shard identities in execution lineage.
Retain the resource promise while cleanup or ownership is uncertain.
Pause or disk-full preserves the last durable checkpoint. Resume
only on the same permitted manifest. Cancellation fences future
commits and reconciles exact staged files.

An erasure immediately revokes the affected generation at read,
dispatch, commit, export and restore boundaries. Walk the reverse
dependency index with 0032. Remove affected managed rows, postings,
model results, shards, caches and exports, or rebuild a new
generation from a current allowed inventory. Old receipts hold
only approved safe evidence. An unreadable store or missing current
erasure proof blocks release. Apply authoritative erasure before any
restored reader or worker opens an old generation. Follow existing
retention, cloud-version and delivered-copy limits. This suite
grants no new storage route.

## Verification and owner flow

The owner enables the site scope, selects an eligible stored corpus
or supplied snapshot, reviews coverage and the resource estimate,
and starts analysis. The Explorer shows one language pane at a time,
scope and omission reasons, then ranked term tables, safe occurrence
views, accessible graphs, duplicate pairs and historical comparisons.
Pause or stop uses existing job controls. The result displays all
applicable holds. An export is a separately eligible local release
with current rights, exact projection and recipient.

The static local action `audit_corpus_query` requires
`audit.corpus.read` and a current exact result projection grant.
Input is the same-site corpus seal ID, one admitted operation,
up to 16 allowed filter leaves and an optional bound cursor.
Return up to 50 typed aggregate, cluster or overlap rows,
source, population, coverage and reasons. Never return word
context, protected exact offsets, person names, original text,
raw entity provenance or private pseudonyms through this route.
Owner-only context stays in its eligible desktop view. No
action creates a new corpus, source read, model run or publish
effect. Refuse prohibited operation, scope or expired cursor
before reading protected data.

Design a bounded Quint model using two sites, two languages, three
items, two attempts, two profile revisions and at least two shard
slots. Model intake, claim, stage, atomic checkpoint, unknown
acknowledgement, pause, cancel, revoke, erasure, restore and release.
Check site and generation isolation, current epoch at publication,
one logical contribution per item, no checkpoint gaps, no partial
or unauthorised release, owner-only on-demand name visibility with
no forbidden sink, and resource promises within admitted caps.
State fairness assumptions separately from safety invariants. Feed
the predetermined failing and successful trace sequences into
matching Rust replay tests with a deterministic fixture clock,
fenced fake storage and pinned profile. Follow QR-006/EX-06 before
claiming Quint Connect qualification. Keep model and replay evidence
distinct and bind exact candidate, models, fixtures, traces, bounds,
platform pins, results and proof of any applicable non-run case.

### Evidence shared with Wave 2 reports

The owning profiles and their thresholds remain separate under
QR-165 and DR-133. Record a shared input and the exact assertion it
supports. Match the site, source revisions, admitted text projection,
operation profile and current rights. Reuse a checked assertion only
for the same eligible inputs and effect. A shared assertion gives no
additional method or capability credit. Each report and corpus
projection keeps its own scope, release and proof.

| Wave 2 obligation | 0144 operation and admissible shared assertion | Different scope and required proof |
|---|---|---|
| F-0681 in [0034](0034-analytics.md), FX-0034-W2-PROTECTED-PHRASE-COUNTS | A144-02, the overlapping one-, two- and three-word window count. Reuse an occurrence assertion only where the admitted normalisation, segmentation, eligible token positions and window bounds match in both profiles. Reuse a distinct-page count only over the same eligible canonical page population and deduplication. | Compare a window inside one eligible sentence and a candidate crossing a sentence boundary within a block. The 0144 window must stay within one sentence, paragraph and language range. F-0681 keeps its existing block, language and page boundaries. A differing seam, token profile or coverage cannot reuse the assertion. Use the owning 0144 evidence for four- and five-word windows and additional language profiles. Shared counts cannot supply a protected release. |
| F-0682 in [0097](0097-brand-monitoring-assets-and-surveys.md), FX-0097-W2-SAFE-SIMILARITY-GRAPH | A144-07, exact overlap confirmation of one eligible pair. Share only the exact overlap assertion for the same two page revisions, extraction, language, normalisation, word-group sets and confirming comparison. A shared acceptance assertion also needs the same applicable admitted threshold, candidate and coverage conditions. | F-0682 consumes qualified 0047 equality and three-word-group similarity, then proves its graph and equivalent list. 0144 uses the 0116 LS-13/36 candidate and confirming comparison envelope. Compare the same operands under a different group profile or admission rule. A matching score alone cannot supply profile or threshold evidence or prove exact text equality. The graph and corpus view each need current scope, coverage and release proof. |

| Case | Given, when and expected result |
| --- | --- |
| A144-01 Language and offsets | Given NFC/NFD café, a joined emoji, a Thai dictionary example and mixed-script text, replay independently admitted profiles on each platform. Return equal eligible NFC keys and exact original byte ranges. Unknown ranges supply no trained result. |
| A144-02 Corpus arithmetic | Given `red red gear`, `red bulb`, `green bulb`, red cf=3, df=2, N=3 and total tokens=7. Verify exact weights, tie order, reviewed unsupported French coverage and no grams crossing ranges. Empty corpus gives no rate. |
| A144-03 KWIC and privacy | Given repeated text across two eligible pages, request paged matches, revoke one grant before the second page, then restore. Refuse stale continuation. No old context, identifiable name, small-cell difference or forbidden result count is released. |
| A144-04 Metrics and entities | Given short and 30-sentence corpora and mixed locale, test exact formula vectors and exclusions. Show insufficient SMOG on short input and unknown on missing syllabifier. Withhold a personal name from graph, context and exported view. |
| A144-05 Bounds and replay | Exercise max and one over page, chunk, term projection and disk bounds. Interrupt before receipt commit and after commit before reply. Compare two complete runs with different admitted chunk partitions. One contribution per item, identical sorted logical hashes, distinct shard lineages and no late old-worker commit. |
| A144-06 Frozen classifier | Reject template-family split leakage. Compare confident, tied, unknown, out-of-taxonomy and changed-model rows against exact held-out labels and score floors. A threshold crossed only within platform uncertainty gives unknown or holds the profile. |
| A144-07 Duplicates and changes | A confirmed Jaccard boundary pair passes only when eligible; a lower candidate, shared boilerplate or revoked input never supplies a duplicate claim. Compare one token insertion, erase one operand and change the model profile. Refuse deletion and drift inferences from these gaps. |
| A144-08 Cross-site and erasure | Stage work for one site, race a tenant change and erasure, and restore an old checkpoint and cache. Prove safe inaccessible results, no stale batch or export, and exact authorised cleanup/rebuild receipts. |
| A144-09 Private name selection | Given a fictional person in a staff-published article, a quoted member comment with the same name and a synthetic handle, show only the exact confirmed staff-authored span to the authorised owner. Verify owner accessibility, refusal of other roles and banned fields, no forbidden sink and no view after lock, withdrawal or restore. |
| A144-10 Release model replay | Replay all selected Quint decision traces through production-entry fixtures under pinned Linux and Windows Bazel profiles. Verify no partial publication, budget oversubscription, duplicate counts or epoch rollback. |

## Sources and pins

These references describe standards or algorithms. They do not attest
to an installed runtime, licensed package, calibrated classifier or
qualifying run.

- Unicode 18 reference: [UAX 29 revision 49][uax29], [UAX 15 revision
  58][uax15], [versioned boundary vectors][boundary]. A runtime uses
  its admitted matching data version and vectors.
- [RFC 5646][rfc], [IANA snapshot source][iana] and [UTS 35][uts35]
  for language identifiers. Pin the registry file date and digest.
- [spaCy pipeline versions][spacy] and existing 0024/0026/0080
  package and resource qualification.
- [Salton and Buckley][tfidf], [Robertson and Zaragoza][bm25] and
  [Dunning][dunning] for lexical weighting and association.
- [W3C Text selectors][kwic] for snippet anchoring. The suite uses
  original UTF-8 byte positions, distinct from W3C character offsets.
- [Kincaid report][kincaid] and [original SMOG][smog]. Spec 0123
  retains its individual formula, sample and conformance obligations.
- [Lee and Seung][nmf], [Broder][minhash], [Charikar][simhash] and
  [Myers][diff] for topic fitting, fingerprinting and alignment.

[uax29]: https://www.unicode.org/reports/tr29/tr29-49.html
[uax15]: https://www.unicode.org/reports/tr15/tr15-58.html
[boundary]: https://www.unicode.org/Public/18.0.0/ucd/auxiliary/
[rfc]: https://www.rfc-editor.org/rfc/rfc5646.txt
[iana]: https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry
[uts35]: https://www.unicode.org/reports/tr35/
[spacy]: https://spacy.io/models
[tfidf]: https://hdl.handle.net/1813/6721
[bm25]: https://www.nowpublishers.com/article/DownloadEBook/INR-019
[dunning]: https://aclanthology.org/J93-1003/
[kwic]: https://www.w3.org/TR/annotation-model/#text-quote-selector
[kincaid]: https://stars.library.ucf.edu/istlibrary/56/
[smog]: https://ogg.osu.edu/media/documents/health_lit/WRRSMOG_Readability_Formula_G._Harry_McLaughlin__1969_.pdf
[nmf]: https://proceedings.neurips.cc/paper_files/paper/2000/file/f9d1152547c0bde01830b7e8bd60024c-Paper.pdf
[minhash]: https://www.cs.princeton.edu/courses/archive/spr05/cos598E/bib/broder97resemblance.pdf
[simhash]: https://www.cs.princeton.edu/courses/archive/spring04/cos598B/bib/CharikarEstim.pdf
[diff]: https://www.janelia.org/publication/ond-difference-algorithm-and-its-variations
