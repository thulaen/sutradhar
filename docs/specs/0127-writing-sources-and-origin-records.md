# 0127: Writing sources and origin records

A writer can see where a passage came from, check it against a
declared collection and prepare a citation. A separate writing
record can explain how text in an opted-in document arrived.
Indeterminate text has an explicit unknown label.

Spec 0121 owns revisions, reader permissions and changes to drafts.
This spec owns writing-workspace reference collections, citation
proposals, overlap reports, origin estimates and optional recording
and replay. Specs 0042 and 0052 retain own-site near-copy advice.
The link engine and site analytics never use the suite's reports.

Example: Asha opens a draft with three claims. "Check sources" reads
only the two files in her admitted collection. One claim is supported,
one meets a conflicting source and one has no coverage, so it shows
unknown. "Format citation" previews the APA 7 in-text and reference
effects. She approves them and the exact reviewed effect set commits.
She then starts recording for this document only. Later she pastes a
paragraph with no origin supplied, and the record shows it as unknown.

## Words used here

- **Collection**: a declared set of local reference files with a
  stated purpose, licence and expiry.
- **Coverage**: the part of the requested text or sources that a
  check really examined.
- **Citation proposal**: the exact in-text, note and reference-list
  effects a writer reviews before any commit.
- **Capture grant**: an actor's approval to record events for one
  document and one capture epoch.
- **Capture epoch**: one recording period. Pause, stop and erasure end
  it, and resume starts a new one.
- **Origin event**: a committed editor change with its admitted
  category, such as direct input, paste or generated insertion.
- **Origin estimate**: an offline detector's approximate signal for
  a passage. It is not provenance and not a verdict.
- **Unknown**: the explicit label for text or coverage the system
  cannot establish.

## What this spec designs

| Feature | What the writer sees |
|---|---|
| E01 to E05, E12 | Claim review, source finding, citations, consistency and similarity checks against declared local collections |
| E06 | An optional origin estimate with coverage and uncertainty |
| E07 to E09 | Opt-in writing record, origin breakdown and private replay |
| E10, E11, E13 | Approved sharing, group aggregates and bounded history |

## Outcomes and scope

Use independently designed rules and original fixtures. The source
matrix preserves the exact capability, plan and surface observed
on 20 November 2026. See the public [source finder][finder],
[similarity checker][similarity], [origin help][origin],
[estimate guide][estimate] and [report reference][reports].

| ID | Outcome | Required case |
|---|---|---|
| E01 Claim review | Identify possible factual claims, quotations, missing citations and references that disagree with a claim | Compare a supported claim, a conflicting source, a quotation and an uncovered claim. Keep unknown and adverse evidence distinct |
| E02 Finding a source | Find candidate local sources with citation metadata, exact passages and relevance reasons | Show a nearby subject match that gives no support. Choose an evidenced passage without treating lexical match as proof |
| E03 Citation generation | Generate the exact in-text citation, note or reference effects required by the chosen edition and source type. Preview placement and order | Try author name or date missing, conflicting IDs and two styles. Reuse an existing matching reference or apply a style-valid in-text-only citation. Only the exact reviewed effect set commits |
| E04 Consistency check | Resolve all references in a selected section against one style edition and report missing, inconsistent or ambiguous links | Include two works of one author in one year, one missing reference and repeated citation. A format pass does not certify a claim |
| E05 Similarity report | Locate overlapping wording against named permitted collections, with scope, references and exact ranges; show matched, quoted, correctly cited and unresolved overlap | Try common wording, quoted text and an exact repeated passage. Unknown coverage cannot justify a clean or misconduct finding |
| E06 Origin estimate | On request, show approximate origin signals for eligible passages, confidence, uncertainties and coverage under an admitted detector | Use human, generated, edited, translated, mixed and short synthetic text. Unsupported language or uncertain input gives unknown. No identity or misconduct inference |
| E07 Writing activity | Record committed in-editor text events for a chosen revision series after the author opts in. Distinguish direct input, paste, import, ordinary correction, newly generated insertion, generated revision and unknown | Mix typing, unmarked and explicitly labelled paste, accepted correction, a new generated insertion and a generated rewrite. Never guess the app or origin of an unmarked paste |
| E08 Origin breakdown | Display origin spans in text and a table with covered counts, typed words, changed words, active sessions and admitted elapsed time | One part predates recording and another is paused. Each displays unknown. Session activity stays distinct from origin |
| E09 Replay | Privately step through the admitted document events and show insertions, removals and suggestion application in order | Replay after an undo and after a capture gap. A removed snippet can appear only where the writer authorised retaining deleted text and it remains in scope |
| E10 Handoff | Share an approved encrypted report with a named recipient. Start with summary only and offer separate switches for full text, source labels, timings and replay | Swap recipient, change evidence, or withdraw replay after preview. Refuse handoff or rebuild the exact permitted payload |
| E11 Aggregates | Generate only approved group indicators from deliberately scoped summaries. Bind cohort, purpose and group privacy evidence | Suppress a small group and private source labels. Keep explicit unknown and refuse cross-site totals and adjacent slices that would reveal a suppressed category |
| E12 Internal knowledge | Search a scoped local reference set for definitions, approved terminology and evidence relevant to a selected passage | Compare two disagreeing approved sources and one source from another workspace. Give eligible dated evidence and flag disagreement |
| E13 Draft history | Keep private analysis and reference findings under an approved bounded retention window and show them only under current document and source rights | Retry, restore and then remove a corpus. Keep withdrawal effective for old reports and omit invalid findings and quotations |

Web-wide similarity, live candidate searches, links hosted for school
submission, cross-device capture and background access to external apps
or the global clipboard need separate approval under DR-114. Track
host-specific origin attribution and institutional reporting separately.
Give each its own row. A local collection, supplied file or manual handoff
covers
only its declared scope. QR-120 holds remaining source and reach gaps.

## Reference and report records

| Record | Fields |
|---|---|
| Collection | Scope and actor, permitted purpose and licence, source and import identities, sealed manifest and document digests, index version, read and output grants, coverage and expiry |
| Citation metadata | Collection or supplied reference ID, kind, authors and roles, title, publisher or container, edition or date, locale, publication identifier or verified address, page or other locator, stated unknown fields |
| Claim and finding | Source revision, exact range, claim or overlap kind, proposed source IDs and evidence spans, confidence and assessment, completed scope and unmet evidence |
| Citation proposal | Base revision and ranges, reference identity, style and edition, ordered optional effects for inline text, note and reference list, and exact effect digest |
| Capture grant | Actor, document and purpose, granted modes and evidence retention, start and expiry, current epoch, visibility and revocation rights |
| Origin event | Scope, actor and revision chain, event number and replay key, committed operation, admitted category, optional checked source, exact ranges, authorised time data, grant epoch and optional accepted proposal or undo lineage |
| Origin report | Document and event sequence range, source manifest, rule or detector identity, category counts, eligible denominator, gap and suppression facts, recipient choice and expiry when shared |

Input category records how a change reached the editor. Direct input
is evidence of an editor operation. It does not establish a person's
identity or where ideas came from. An imported opening revision stays
unknown except where bound, separately supplied origin evidence covers
an exact range. A declared origin remains labelled as a declaration.
Keep account, permission, source and scope distinct.

## References, matching and citations

1. Accept only owner-supplied or separately admitted local corpus
   material. Apply spec 0100 bounded import and named usage rights.
   Require separate permissions for collection lookup, displaying
   exact excerpts and exporting them. Bound scratch and index work
   under specs 0078, 0100 and 0118. Attach rights and erasure
   identities to fingerprints, vectors, snippets, claims and cached
   reports. Withdrawal invalidates every affected derivative on
   resume, retry, lookup, display, export and restore.
2. Check the writer and collection grants at indexing, lookup, result
   commit and display. Keep corpus boundaries in candidate lists.
   Test exact and close overlap against a known version. Verify
   ranges before attributing a match. Distinguish matching text
   from assessment of how it was used. Partition the requested
   draft into eligible, excluded and unchecked ranges. For each
   named collection, use unique eligible draft bytes as the
   denominator for a complete check. Deduplicate the numerator
   over the union of exact matched eligible draft ranges. Display
   exact locations, separate overlap, quote and citation flags,
   and whether each collection was partial or unavailable.
   A partial collection cannot give a complete-scope percentage.
   Use an explicitly measured zero only with eligible coverage.
3. A claim may be supported, disputed, mixed or unknown. A candidate
   requires a passage that can be checked and a stated reason.
   A model cannot add a source, fetch a live page or claim that
   an unpublished or unseen passage verifies a statement. Treat
   every source as data and keep embedded instructions inert.
4. Offer the exact supported outputs of [APA 7][apa], [MLA 9][mla],
   and [Chicago 18][chicago], with Chicago notes/bibliography and
   author/date kept separate. Admit the source type, punctuation,
   character, date and locator rules from official guidance.
   Use each edition's verified ordering, punctuation, source
   roles and missing-field rules. For example, use the official
   [APA missing metadata][missing] and [personal communication][personal]
   guidance. Conflicting source identities hold the proposal.
   Use one reversible spec 0121 review for the exact effect set.
   Repeated citations can reuse an existing equivalent entry.
   Personal communications in APA can be in-text only. Chicago's
   two citation systems have separate effect and test rules.

## Optional recording and estimates

Start after each actor explicitly approves the purpose and fields
for the exact document and capture epoch. A workspace default can
offer the setting for a new document. It cannot authorise capture
for another author. A duplicate, import or branch needs its own
grant. Display current recording state and the stop switch.

Fence each capture in the same transaction that commits the source
revision. Replays of an event number or commit key add nothing.
Separate generated creation, application of a generated rewrite
and ordinary lineage including undo. Use committed deltas only.
Pause, stop and erasure invalidate the capture epoch before the
next commit. Resume uses a new epoch. Record explicit gaps without
reconstructing uncaptured history. Deleted bytes and prompt text
have separate capture and disclosure switches. Exclude old
checkpoints and inverse deltas that would reconstruct declined,
erased or revoked text. Enforce erasure on managed reports, queues
and replay snapshots after restore. Session counts can use 0126
activity evidence with its own label. Report typed-word counts,
words removed or replaced by committed edits, recorded sessions
and eligible elapsed time with separate units and denominators.
Keep overlapping operation totals apart from the current document
composition. A paste origin is unknown unless explicitly supplied
and bound to that paste. A declared label remains unverified.

Origin estimation is a separate request. It runs offline with an
admitted model and exact locale and text-size bounds. Train and
evaluate with synthetic data under the project evidence rules.
Require frozen tests for false positives, false negatives, calibrated
uncertainty, quoted and mixed text and language variants. Report
whole-selection coverage and per-passage uncertainty separately.
Record the admitted eligible denominator and minimum coverage.
Below that threshold, give no overall estimate. Display the
model revision and what it evaluated. Let a writer declare known
AI assistance, request a supported citation or disclosure and
mark a citation as added. Scores never set those fields. Do not
present a detector score as event provenance, a verdict or an assessment of
an author.

## Layout and release checks

The panel offers “Check sources”, “Format citation”, “Check
similarity” and “Writing record”. Show collection and coverage in
each check. “Start recording” names the document and protected
fields. “Share a copy” prepares exact approved sections, with
replay off by default. Require the exact payload, current grant,
format, recipient and purpose at preview and handoff under 0118.
Allowed editions bind current source and subject grants for each
included field independently. Managed results stay encrypted and
use the file ledger. A revoked managed handoff cannot finish.
An authorised copy already handed to a recipient has a receipt.
Keep it separate from pending and managed copies. The app cannot
recall recipients' retained files. Refuse raw clipboard, active
export, public report link and remote submission. Show historical
observations while authority, source rights and expiry allow.

Only release a group report after its cohort, purpose and category
or source denominators pass the admitted privacy rule. Keep source
filenames or labels out unless the view independently allows them.
If a difference between permitted exports can reveal a suppressed
group or category, coarsen or refuse the later request under the
signed release policy. Keep a minimal scope receipt to apply the
policy across periods, filters and exports.

## Data

- Records are those in "Reference and report records": collection,
  citation metadata, claim and finding, citation proposal, capture
  grant, origin event and origin report.
- Managed results stay encrypted and are listed in the file ledger
  under spec 0118. Fingerprints, vectors, snippets, claims and cached
  reports carry rights and erasure identities.
- Origin events are fenced to a capture epoch and committed with the
  source revision. Pause, stop and erasure invalidate the epoch.
- Collection files, supplied files and manual handoffs hold only their
  declared scope. No data from this spec reaches the link engine or
  site analytics.

## Settings

All keys and numbers below are proposed until QR-120 admits them.

| Setting | Type | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `writing.record.enabled` | on or off, per document | off | two values | next capture epoch |
| `writing.record.keep_deleted_text` (owner-only) | on or off | off (proposed) | two values | next capture epoch |
| `writing.record.keep_prompt_text` (owner-only) | on or off | off (proposed) | two values | next capture epoch |
| `writing.share.replay` (owner-only) | on or off | off | two values | next preview |
| `writing.origin.min_coverage` | ratio | 0.5 (proposed) | 0.1 to 1 (proposed) | next estimate |
| `writing.origin.max_chars` | characters | 50,000 (proposed) | 100 to 200,000 (proposed) | next estimate |
| `writing.collection.max_mib` | MiB | 256 (proposed) | 1 to 1,024 (proposed) | next index |
| `writing.history.keep_days` | days | 30 (proposed) | 1 to 365 (proposed) | next night |
| `writing.share.expiry_h` | hours | 72 (proposed) | 1 to 720 (proposed) | next preview |

Indexing a collection is chunked, checkpointed and limited by a
deadline of 60 seconds per chunk (proposed). A missed deadline
saves the checkpoint and resumes later. Spec 0078 sets the
resource budget, and a collection over that budget is refused.

The settings marked owner-only (`keep_deleted_text`, `keep_prompt_text`
and `share.replay`) can be changed only by the owner in the app. The CLI,
MCP and agents cannot change them.

No setting turns on web-wide search, a live fetch, cross-device
capture, a public report link or a workspace-wide recording default
that authorises another author.

## Failure codes

Decision DR-130 records SUT-5600 to SUT-5699 for this spec. It was confirmed
by the owner on 21 November 2026. Only the first ten, SUT-5600 to SUT-5609,
are in use, and REQ-0127-FAIL tests them.

| Code | Plain-English result |
|---|---|
| SUT-5600 | "This source is not in an admitted collection." Refuse the lookup. |
| SUT-5601 | "Coverage is unknown or partial." Give no complete percentage and no clean finding. |
| SUT-5602 | "The reference details conflict." Hold the citation proposal. |
| SUT-5603 | "This style or source type is not supported." Offer no citation. |
| SUT-5604 | "Recording is not approved for this document." Capture nothing. |
| SUT-5605 | "There is not enough text to estimate." Show unknown. |
| SUT-5606 | "This copy cannot be shared as shown." Refuse or rebuild the exact permitted payload. |
| SUT-5607 | "This group view is withheld." Small group, suppressed category or differencing risk. |
| SUT-5608 | "Rights for this source were withdrawn." Invalidate every derivative. |
| SUT-5609 | "This document or collection is over the limit." Refuse before work starts. |

## Security and privacy

- Treat every source, citation and embedded instruction as data.
  A model cannot add a source or fetch a live page.
- Recording starts only after explicit approval by each actor for the
  exact document and epoch. A default cannot authorise another author.
- Deleted bytes and prompt text have separate capture and disclosure
  switches. Erased or revoked text is not rebuilt from checkpoints.
- A detector score is never provenance, identity or misconduct
  evidence. A declared origin stays a declaration.
- Handoff needs the exact payload, grant, format, recipient and purpose
  under spec 0118. Raw clipboard, active export, public link and remote
  submission are refused.
- Group reports apply the admitted privacy rule and a scope receipt
  that blocks differencing across periods, filters and exports.
- Outbound access is off. The suite sends no source or draft text out.

## Requirements

| Requirement | Acceptance |
|---|---|
| REQ-0127-LOCAL | E2E-0127-LOCAL: disable outbound access, try unadmitted collections and planted citation instructions. Allow admitted local reading only. Show unknown coverage and no fetch |
| REQ-0127-CITE | UT-0127-CITE and BDD-0127-CITE: exercise independent synthetic vectors per style, source type and unknown or conflicting metadata. Test insertion with duplicate or existing entry, APA in-text-only case, both Chicago forms, changed reference and undo. The exact permitted effect set commits together |
| REQ-0127-MATCH | IT-0127-MATCH: compare two sources sharing one overlap, a common phrase, exact quoted and cited text, and partial or missing coverage. Test revoked excerpt-only rights, cross-corpus, different source version and linked derivatives. Show exact ranges when eligible and no implied finding of misconduct |
| REQ-0127-RECORD | BDD-0127-RECORD: try a second author, default setting, changed branch, pause racing a commit, accepted suggestion lineage, duplicate event, distinct prompt and inverse-delta grants, erase and restored state. Replay only allowed ranges. Unknown stays unknown |
| REQ-0127-ESTIMATE | E2E-0127-ESTIMATE: test null, short, edited, quoted, human-only, generated-only and mixed fixtures per admitted locale. Check passage and overall scopes and exact coverage bounds. Only a user's declared event can set a citation or disclosure |
| REQ-0127-SHARE | IT-0127-SHARE: try recipient substitution, authority loss after preview, replay off, removed source names and timings, failed export, differencing across periods, stale revision and deleted prompt. Refuse unapproved raw output, restore and later disclosure |
| REQ-0127-LOAD | E2E-0127-LOAD: prove empty, one, exact admitted maximum and one over for documents and collections on all three version 1 platforms. Race cancellation, erasure and reference removal at storage boundaries |
| REQ-0127-SETTING | UT-0127-SETTING: for each setting, test the default, one value just inside and one just outside the range, and the stated effect time. Refuse any setting that would enable web search, live fetch or another author's capture |
| REQ-0127-FAIL | UT-0127-FAIL: reach each SUT-5600 to SUT-5609 code once. Check the plain message, that no source or draft text appears in it, and that no code is reused from another spec |

QR-120 covers corpus use rights and representativeness, all exact
style and source types, method qualification, bounded profiles
and the local equivalent evidence for each outside workflow.

## How it is tested

Designed, not run. Use invented authors, sources and synthetic text.

- **E2E-0127-LOCAL**: outbound access off, with unadmitted collections
  and planted citation instructions. Admitted local reading only.
- **UT-0127-CITE**: independent vectors per style, source type and
  unknown or conflicting metadata.
- **BDD-0127-CITE**: preview, approve, change a reference and undo.
  The exact effect set commits together.
- **IT-0127-MATCH**: overlap, quote and cited text across two sources,
  with partial coverage, revoked rights and another corpus.
- **BDD-0127-RECORD**: grants, epochs, duplicate events, erase and
  restore. Unknown stays unknown.
- **E2E-0127-ESTIMATE**: null, short, edited, quoted, human, generated
  and mixed fixtures per admitted locale, with coverage bounds.
- **IT-0127-SHARE**: recipient swap, lost authority, replay off,
  differencing, stale revision and failed export.
- **E2E-0127-LOAD**: empty, one, maximum and one over, on all three
  version 1 platforms, with races at storage boundaries.
- **UT-0127-SETTING** and **UT-0127-FAIL**: as in the requirements.

## Changes from the source

The source tools are web services with live search, host-specific
attribution and institutional reports. Here every check reads only a
declared local collection or supplied file. Web-wide similarity, live
candidate search, hosted submission links, cross-device capture and
background app access are separate DR-114 rows, held in QR-120. The
source detector and activity record become offline, opt-in features
that never claim identity, provenance or misconduct.

[finder]: https://support.grammarly.com/hc/en-us/articles/38552451211533-Citation-Finder-user-guide
[similarity]: https://support.grammarly.com/hc/en-us/articles/115000091452-Plagiarism-Checker-user-guide
[origin]: https://support.grammarly.com/hc/en-us/articles/29548735595405-Introducing-Authorship
[estimate]: https://support.grammarly.com/hc/en-us/articles/28936304999949-AI-Detector-user-guide
[reports]: https://prowritingaid.com/features/writing-reports
[apa]: https://apastyle.apa.org/style-grammar-guidelines/references/examples
[mla]: https://style.mla.org/in-text-citations-overview/
[chicago]: https://www.chicagomanualofstyle.org/tools_citationguide.html
[missing]: https://apastyle.apa.org/style-grammar-guidelines/references/missing-information
[personal]: https://apastyle.apa.org/style-grammar-guidelines/citations/personal-communications
