# 0123: Writing analysis reports

Reports show a writer where a draft may need attention and what the
report measured. A writer can run one report, combine several, or
open a summary and jump to the passages that support it.

Spec 0121 owns writing requests, revisions, authorisation and edit
approval. Spec 0122 owns language selection, local text processing
and corrections. This spec owns full-document measurements and
read-only report views. It consumes evidence through those public
contracts. It makes no claim about live search rankings.

## Words used here

- **Report**: one named, read-only measurement of a whole document.
- **Measure**: one exact count or ratio with its units and coverage.
- **Window**: a bounded slice of the document that one job step reads.
- **Seal**: the fixed, immutable result of one report request.
- **Reference profile**: an admitted body of texts that a measure can
  be compared with.
- **Preset**: a saved, ordered set of report IDs.

Example: Asha runs a saved set of three reports on one chapter:
R03 writing counts, R12 repeats in the draft and R13 nearby echoes
(illustrative figures). R03 shows the counts with their units. R12
groups a phrase that occurs in two sections, and an occurrence inside
an excluded quotation is left out and never joins the group. R13 shows
the phrase twice within the admitted distance, each hit linked to its
text. R13 fails on a withdrawn pack. The other two stay readable and
the set records R13 as failed. Asha edits the chapter and opens the
old seal. It still names the old revision. A new run gets a new ID
that links to the old seal.

## Report catalogue

The catalogue below defines separate outcomes based on public
[writing reports][reports], [report help][help], [style categories][style],
[summary help][summary] and [writing preferences][settings]. The suite
coverage record holds the exact source, plan and surface for every
outcome. Source observations date to 20 November 2026.

Use one admitted locale and genre policy per measurement group.
Resolve each selected range's language under spec 0122. A mixed
document may show more than one group. A report policy applies only
to a group whose resolved language it supports. An unknown language
or missing policy leaves the affected locale-dependent measure
unavailable. Keep eligible measures for the other groups and label
the coverage. For each table row below, the acceptance fixture uses
original synthetic text and has
both a matching and a non-matching case. Report IDs stay stable
across changes to settings. Font, colour and sort settings cannot
create new report capabilities.

| ID and report | Evidence and output | Report-specific acceptance |
|---|---|---|
| R01 Overview | Run summary with section coverage, admitted scores by aspect, key findings, positive findings, policy goal bands and links to evidence | Use two different genre policies. Show measured strengths and weaknesses, drill down to each finding and give the reason for an inapplicable goal |
| R02 Saved set of reports | Run listed report IDs against one sealed revision. Keep each report's findings, status and limits | Make one report fail and rerun. Other successes remain readable under their grants. The old seal stays fixed and a changed draft cannot enter its set |
| R03 Writing counts | Scalar, grapheme, word, sentence and paragraph counts, with blanks, headings and exclusions accounted for | Use two paragraphs with emoji, combining text and an excluded block. Show exact counts and named units |
| R04 Vocabulary | Distinct eligible forms; uncommon forms only against an admitted frequency reference, with coverage and reference label | Use a term with repeated forms, a known common word and a word absent from the reference. Unknown frequency cannot become a rare-word claim |
| R05 Readability | Estimate reading time using an admitted locale-specific rate. Show the document overview, paragraph difficulty bands and exact passage links; show the four separately admitted published measures | Use three band fixtures and too little text. Check reading time and each measure against its own independent vectors. Distinguish missing evidence from a low score |
| R06 Grammar and mechanics | Fold spec 0122's captured corrections into a whole-document list by category and scope | Inspect two valid and two invalid spans with mixed language, then make one category unavailable. Preserve coverage and exact check revisions |
| R07 Sentence length | Length per eligible sentence and number in each versioned display band, plus covered sentence count | Place one case at a band edge and one just below. Exercise abbreviations and a block spanning two batches |
| R08 Sentence variety | Count structure categories from admitted parse facts, with first-word patterns and repeated opener groups | Use repeated openers with and without matching structure. Show the two measures separately |
| R09 Readable phrasing | List original 0122 clarity, fluency and concision findings with sentence and paragraph context | Show one awkward sentence and one concise alternative. Keep an intact quote and unsupported language range distinguishable |
| R10 Function-word density | Give eligible function-word share for each sentence and for the whole document, with numerator and denominator. Flag spans only by admitted locale and genre policy | Build an exact marked fixture, allow one intentionally repetitive phrase and give unknown when the denominator is absent |
| R11 Overused choices | Count words or fixed phrases from an admitted list and the owner's term list with rate and occurrence links | Add and remove an owner phrase. Exercise word boundaries, an ignored quote and unrelated homographs |
| R12 Repeats in the draft | Group matching normalised phrases across the selected scope. Preserve exact occurrences, gaps and excluded spans | Repeat a phrase across sections and within one section. Add a phrase split by an excluded quotation. No occurrence bridges the excluded range |
| R13 Nearby echoes | Group repeats within the admitted maximum distance in Unicode scalar positions and the selected minimum length and count | Use equal phrases within, at and outside the distance, repeated across a batch seam and split by an excluded block. No loss at the seam or duplicate matches from context |
| R14 Repeated initial sounds | Preserve adjacent words with shared initial sound under the admitted local pronunciation table | Include same letter with a different sound, different letter with a matching sound and unknown pronunciation |
| R15 Familiar phrases and redundancies | Flag matched, independently sourced phrase types and explain the wording overlap. Allow genre and quotation exemptions | Use one intentional catchphrase, one exempt dialogue span and one eligible redundant combination |
| R16 Diction | Flag vague, abstract, inflated and strongly context-dependent expressions with checkable local evidence | Compare same expression in a vague and in a precise technical passage. Show a weak signal as advice |
| R17 Easily confused words | List eligible homophones, shared spellings and related confusion sets. Keep likely misuse separate from possible ambiguity | Use two words with matching pronunciation and a valid term sharing spelling. Show only evidence-supported correction |
| R18 Pronouns | Count pronoun type, flag unsupported reference, and compute per-sentence and whole-document proportions using eligible words | Use a clear antecedent, a missing antecedent and dialogue-only text. Do not flag a named speaker as absent |
| R19 Abbreviations | List occurrences, first explanation and later variations in source order using word and case rules | Use the same short form with two expansions and an accepted abbreviation that needs no explanation |
| R20 Consistency | Show conflicting spelling, capitalisation, hyphenation, numbers, dates, named terms and typography under one policy; separate variation by language or quote | Mix two legitimate dialects in marked spans, then mix spellings in one scope. Only the latter can be a consistency finding |
| R21 Transitions | Show eligible linking phrases and positions. Flag missing or repetitive transitions under the declared genre | Compare linked and unlinked adjacent passages and a deliberate break. Give covered range and deciding evidence |
| R22 Dialogue | Show marked speech, attribution tags, alternative tags, tag modifiers and counts with supporting text | Use nested quotes, indirect speech and narrative between two lines. Do not treat prose quotes as dialogue without evidence |
| R23 Pace | Map scene and paragraph ranges with explicitly described pace indicators such as action, dialogue or reflection. Surface candidate slow passages in context | Use an intentional quiet section and repetitive exposition. Show evidence and advice, without a verdict about reader engagement |
| R24 Sensory language | Categorise explicit sight, sound, touch, taste and smell with spans; separate figurative or uncertain evidence | Use examples from all five senses, quoted metaphor and ambiguous language. Keep uncertainty separate |
| R25 Style patterns | View passive and nominalised verbs, applicable adverbs outside dialogue, long clauses, emotional exposition, repeated starts and inclusive-language findings. Report passive clauses divided by eligible classified clauses | Compare an intentional passive sentence and a policy warning. Show passive and eligible counts and their proportion. Test adverbs in speech and narrative. Turning off one overlay keeps the data consistent |
| R26 House style | Show independently defined violations of effective personal and shared rule versions, with policy and scope provenance | Use a shared spelling rule and a lower-priority conflicting preference. Show which rule applies and the reason. Try changed policy while running |
| R27 Reference comparison | Compare like metrics for the selected writing and separate admitted author and genre references. Choose either or both. Show each coverage, unit and difference separately | Test author only, genre only and both. An incompatible genre or locale disables that comparison only. Support a readable data table for each result |
| R28 Tone and purpose | Give overall and local tone observations and degree of fit to the writer's audience, formality, intent and document type. Explain any disagreement between whole and local views | Use a formal document with one informal quoted line. Show both tone scopes. Do not infer a reader's reaction as fact |
| R29 Word inventory by role | Group eligible words by noun, verb, adjective and adverb, with counts and ordered occurrence links. Request 0122 alternatives for the exact selected occurrence | Use the same spelling in two roles. Navigate previous and next occurrence, then show alternatives for the right sense and span. Unsupported roles stay unknown |

The report catalogue also links to 0122's live line checks and word
exploration, and uses the [word inventory guide][wordroles] for R29. It links to
the writing-suite contracts for rephrasing,
chapter critique and originality checks. These consume the same
revision. They keep their own distinct inputs, effects and tests.
A similarity check needs an admitted reference collection and
cannot be inferred from repeated phrases inside one draft.

The public [readability guide][readability] names Flesch Reading
Ease, Flesch–Kincaid Grade, Coleman–Liau Index and the Automated
Readability Index. Implement each from its independent published
source and verify it against its own vectors under spec 0076.
Label the grade outputs as US school grades. The app's British
English cannot change their units. Show the method conditions and
withhold a measure where language or sample conditions fail.
Explain the inputs and boundaries of the three difficulty bands.
A reading-time estimate shows its count and assumed eligible rate.

## Records and comparison rules

| Record | Fields |
|---|---|
| Report definition | Stable ID, version, locale and genre applicability, prompt or measure identity, required facts, units, inclusion and exclusion rules, risk and resource class |
| Report request | Exact spec 0121 request, report IDs and definition versions, optional author and genre comparison digests, range and manifest digest |
| Preset | Scope, owner, label, ordered unique report IDs and version |
| Measure | Report ID, metric version, rational numerator/denominator or exact count, units, optional range, coverage and availability reason |
| Finding | Spec 0121 finding, report ID, evidence source and applicable genre or locale policy |
| Reference profile | Name, attribution and rights record, admitted collection digest, locale, genre, measurement versions, number of eligible texts and coverage |
| Dependency manifest | Approved range set and document content hash, report and measure versions, locale and genre policy versions, pack and model versions, comparison and corpus digests, authorisation epochs, effective limits |
| Sealed report | Request and manifest digests, requested and completed items, immutable result digest, coverage, dataset identities and expiry |

Every count shows units and coverage. Every rate also needs a
positive denominator and its exact numerator. Zero is a measured
result. A missing input has an explicit reason. Store exact values
and round only for display under the report's versioned rule.
Never conflate different metrics, writing types or locales. A
reference requires the same metric version, named units, admitted
sampling frame and measurement scope. Decline comparisons when
those differ. Display a reference as evidence to consider. It
cannot prescribe the writer's voice or certify an outcome.

Use the original document bytes and ranges from 0122. Keep case,
quotation and region policies attached to the evidence. A phrase's
position is its first scalar position in the sealed document. The
report may show each overlapping occurrence once and carries its
exact ranges. A match never crosses an excluded range. Assign each
match to the window that contains its start. Supply sufficient
declared context at seams and never count the same match twice.
Pin and admit different window or token rules as separate policy
versions. Never use another product's thresholds, weighting or
reference corpus as the default.

## Run and review

1. The writer chooses a report or a saved set. Show what it needs
   and its selected scope. Snapshot source, genre and ranges.
   Check current actor, workspace, document and reference grants,
   consent and language/pack availability. Bind the exact request
   and dependency manifest to each job, window and result.
2. The scheduler takes a resource lease and produces report facts
   in bounded windows. Read no other document without its own grant.
   Check resource use and cancellation between windows. Each window
   returns exact coverage and never edits a document.
3. Fold the window facts in stable text order. Global measures need
   exact covered scope. If part of it is missing, show supported
   local findings and label the global measure incomplete. Seal one
   terminal result per request identity. Pending windows use
   replaceable checkpoints keyed by exact input and window ID.
   Duplicate delivery replaces the same checkpoint. A run after
   terminal failure creates a new ID linked to the prior seal.
   Reuse a successful report only when its complete identity and
   current grants match. Preserve the earlier seal. Recheck scope,
   every grant, consent, expiry and dependency before display.
4. The panel gives the report list, current scope, findings in
   text order, a summary and a data table for every chart, grouping
   and word display. The
   writer can toggle highlights, jump to a finding, listen to its
   explanation or compare an associated proposal. Resolve a range
   against the sealed revision. If the draft changed, open that
   revision for inspection or run the report again. Changes use
   spec 0121 review.
5. Show the printable preview and locally export its sealed copy
   under specs 0100 and 0118. Preview names the exact revision,
   included quotes, payload hash, format, destination and any
   recipient. At approval and handoff, bind current authority,
   recipient grant where applicable and exact preview fields. A
   managed copy stays encrypted and ledgered. Print and PDF need
   an admitted protected route under QR-039 and deliberate
   authorisation. Refuse an unadmitted printer, spool, temporary
   path or format. Use only escaped inert output, with no fallback
   delivery. Keep the report if the requested route is unavailable.
6. Recheck current grants and the exact manifest on lookup, commit,
   display, resume, copy and handoff. Use the request and manifest
   digests together for the cache key. Revocation during pause or
   restore invalidates the work and all dependent summaries and
   managed copies. Removal or expiry uses the file ledger and
   restore rules. Keep prose and raw source tokens out of logs.
   Keep report scores out of training and ranking inputs.

## Data

The records above are the data of this spec. Each is scoped by site or
local writing workspace and bound to the spec 0121 revision it read.

- Stored: report definitions, presets, requests, dependency manifests,
  reference profiles, replaceable window checkpoints and sealed
  reports, with exact values and no rounding.
- Managed copies and exports are files under the file ledger and
  specs 0100 and 0118. They stay encrypted and are erased with their
  seal.
- Reports hold ids, ranges and measures. Logs and traces hold no
  prose or raw source tokens. Scores are not training or ranking
  input.

## Settings

| Setting | Type | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `writing.report.preset` | ordered unique report IDs, per scope | none | any of R01 to R29 | next run |
| `writing.report.locale_policy` | admitted locale policy version | policy admitted for each group's resolved language | admitted versions compatible with the selected group | next run |
| `writing.report.genre_policy` | admitted genre policy version | the document's admitted genre | admitted versions | next run |
| `writing.report.reference` | none, author, genre or both | none (proposed) | four values | next run |
| `writing.report.owner_terms` | list of words or phrases | empty | at most 200 terms of at most 100 Unicode scalar values each (proposed) | next run |
| `writing.report.echo_distance` | Unicode scalar positions | policy value | up to the admitted maximum; QR-116 must record it before admission | next run |
| `writing.report.echo_min_length` | Unicode scalar positions | policy value | admitted policy range | next run |
| `writing.report.echo_min_count` | whole number | policy value | admitted policy range | next run |
| `writing.report.highlights` | on or off, per report | on (proposed) | two values | at once |
| `writing.report.seal_expiry_d` | days | 30 (proposed) | 1 to 365 (proposed) | new seals |

Font, colour and sort settings change only the display. They cannot
create a report capability or change a measure. Report IDs stay stable
across settings. The exact policy values, thresholds and resource
limits are held by QR-116. No setting turns on another product's
thresholds, weighting or reference corpus, or a network lookup.

## Failure codes

Codes SUT-5200 to SUT-5299 are reserved for this spec (DR-130). A scope or
permission refusal uses SUT-5000 of spec 0121.

| Code | Meaning |
|---|---|
| SUT-5200 | A measure is withheld: language, sample or method conditions failed, or the input is missing. A reason is given |
| SUT-5201 | Coverage is incomplete. Local findings show and the global measure is labelled incomplete |
| SUT-5202 | The draft, manifest or request digest changed. A cache hit is refused and a new run is needed |
| SUT-5203 | A reference is refused: wrong licence or scope, withdrawn version, or incompatible metric |
| SUT-5204 | A preview, export or print route is unadmitted or its preview fields changed. No fallback file is written |
| SUT-5205 | Authority, consent or a dependency was withdrawn, and dependent facts, summaries and copies are invalidated |
| SUT-5206 | A window failed or the admitted resource limit was reached |

## Security and privacy

- Report only text that the actor may read and check. Read no other
  document without its own grant.
- Recheck current grants, consent, expiry and the exact manifest on
  lookup, commit, display, resume, copy and handoff. Revocation
  during pause or restore invalidates the work and its dependants.
- Reports never edit a document. Changes go through spec 0121 review.
- A reference is evidence only. Check its rights record, licence and
  scope, and never leak its source text.
- Export output is escaped and inert, uses only an admitted protected
  route and writes no fallback delivery. Managed copies stay
  encrypted and ledgered.
- Keep prose and raw source tokens out of logs. Keep report scores out
  of training and ranking inputs. Reports make no claim about live
  search rankings.
- Fixtures and examples are original synthetic text.

## Requirements

| Requirement | Acceptance |
|---|---|
| REQ-0123-CATALOGUE | IT-0123-CATALOGUE: replay every report row with exact synthetic inputs, required positive and negative results, excluded and unknown coverage. Compare a source manifest with an omitted report. It fails even when all present rows pass |
| REQ-0123-MEASURES | UT-0123-MEASURES: exercise empty and one-token inputs, ties, policy boundaries, quoted and bilingual text, and synthetic exact ratios. Use resolved British English and French ranges in one revision. With a compatible policy admitted for one group only, return an unavailable locale-dependent measure for the other and separate coverage for both. Distinguish measured zero, missing and incomparable evidence |
| REQ-0123-BATCH | FI-0123-BATCH: retry a pending batch, cross a report window, lose one window, and pause or cancel mid-fold. Retry a terminal failure and verify the linked run and original seal. No double count, no silently partial score and no folded result from mixed revisions. Change the requested range or evidence digest on replay and refuse a cache hit |
| REQ-0123-REFERENCE | IT-0123-REFERENCE: try a permitted reference, wrong licence or scope, withdrawn version and incompatible metric. Show only admitted comparisons and no source-text leak |
| REQ-0123-VIEW | BDD-0123-VIEW: select reports, jump by keyboard and screen reader, toggle overlays, view underlying table and check a stale draft or failed selected report. Preview the same sealed facts in the report and approved export. Try a changed recipient, format, payload or destination, unauthorised spool and active HTML. Refuse each without writing a fallback file |
| REQ-0123-REVOKE | E2E-0123-REVOKE: withdraw document or reference authority during pause, erase a revision after analysis, retry and restore a backup. Withhold affected facts, summaries and managed copies, and verify ledger handling |
| REQ-0123-LOAD | E2E-0123-LOAD: run empty, one, largest admitted and over-limit reports and sets with concurrent desktop work on each v1 platform. No unbounded spill, lost pause or editor freeze |
| REQ-0123-SETTINGS | UT-0123-SETTINGS: each setting at its default and each allowed value, one value outside range refused. Display settings leave every measure unchanged. Echo distance, length and count behave at, just inside and just outside their bounds |
| REQ-0123-ERRORS | UT-0123-ERRORS: provoke each failure code once. A scope refusal returns SUT-5000. Messages contain no document text |

QR-116 requires the exact measuring rules, genre profiles, signed
accuracy bars, corpus rights and local resource evidence for the
report variants. A documented report needs its own proof before the
suite can count it as covered.

## How it is tested

Designed, not run.

- **Unit (UT-0123):** MEASURES, SETTINGS and ERRORS with original
  synthetic exact ratios and one value just inside and one just
  outside each bound.
- **Integration (IT-0123):** CATALOGUE for every report row and
  REFERENCE for admitted and refused references.
- **Fault injection (FI-0123):** BATCH, with retried, lost, paused and
  cancelled windows.
- **Behaviour (BDD-0123):** VIEW, by keyboard and screen reader, with
  the data table and the export preview.
- **End to end (E2E-0123):** REVOKE and LOAD on each v1 platform.
- Each test id is one named suite under Bazel. QR-116 holds the signed
  accuracy bars and resource evidence that release also needs.

## Changes from the source

The public ProWritingAid report pages describe outcomes that this
spec splits into 29 stable report rows, each with its own matching and
non-matching proof. Our measures, bands and wording are original and
no other product's thresholds, weighting or reference corpus is the
default. Rewriting, chapter critique and originality checks stay in
their own specs, and a similarity check needs an admitted reference
collection. The report views are read-only. Print and PDF wait on
QR-039. QR-116 holds the open measuring rules, genre profiles and
reference rights.

[reports]: https://prowritingaid.com/features/writing-reports
[help]: https://help.prowritingaid.com/category/53-the-reports
[style]: https://help.prowritingaid.com/article/31-what-does-the-writing-style-report-do
[summary]: https://help.prowritingaid.com/article/352-how-to-use-the-summary-report
[wordroles]: https://help.prowritingaid.com/article/39-how-to-use-the-thesaurus-check
[readability]: https://help.prowritingaid.com/article/41-how-to-use-the-readability-report
[settings]: https://help.prowritingaid.com/article/296-getting-started-with-prowritingaid
