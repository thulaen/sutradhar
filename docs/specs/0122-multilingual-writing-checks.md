# 0122: Multilingual writing checks

A writer can check a selection or a whole document for spelling,
grammar, punctuation and style. They can choose which language and
local spelling to use, including different languages in one text.
Each check shows what it examined and where its advice applies.

Spec 0121 owns the shared writing and permission records. This spec
owns language selection, correction, vocabulary advice and the line
check used by feature 186. Rewriting, translation, whole-document
reports and shared policy administration have separate contracts.

In a made-up example, Asha writes a note that mixes English and
French. She picks British English for the English parts and French
for the rest. One check shows a spelling finding in each language,
and the coverage receipt names any part it could not check.

## Words used here

- **Language key**: a BCP 47 tag, with an orthography ID where needed,
  that names a language, script and region.
- **Check pack**: signed, immutable data for one language and its
  declared variants.
- **Rule**: one stable, versioned check that can raise a finding.
- **Grapheme**: one character as a writer sees and edits it.
- **Coverage receipt**: the record of which ranges were requested,
  evaluated, excluded or left incomplete.
- **Generation**: a counter that rises on each committed edit or
  relevant change, so older work can be dropped.
- **Structure check**: the length, character and check-digit test of
  an IBAN or ISBN. It says nothing about ownership or assignment.

## Required language coverage

Source facts below were observed on 20 November 2026. LT means the
[LanguageTool language guide][lt], except where shown. G means
[Grammarly multilingual suggestions][g], its [dialect guide][gd]
and its [public languages page][gm].
P means the [ProWritingAid language guide][p]. An ordinary entry adds
the language without a region. A dialect or orthography adds a
distinct variant of the base. Keep differences in capability visible.

| Language and required variants | Sources | Required proof beyond general correctness |
|---|---|---|
| Arabic, Persian | LT | Right-to-left selection and mixed direction. Persian has no LT spelling promise |
| Asturian, Belarusian, Breton, Crimean Tatar | LT | Accented forms, script choice where documented, and no merge with a neighbouring language |
| Catalan, Balearic and Valencian | LT | Pronoun and word-form differences remain local to the variant |
| Chinese, Japanese | LT, G | Language-aware word boundaries. No LT spelling claim. G lists spelling, punctuation and grammar for both. Its help page names finished-sentence clarity, fluency and concision for Japanese on Windows and Mac |
| Czech, Finnish, Hindi, Hungarian, Indonesian, Korean, Turkish, Vietnamese | G | Script-appropriate segmentation, morphology and punctuation. Admit a fixture collection per check and variant, including finished-sentence advice |
| Danish, Norwegian, Swedish | G; LT also lists Danish and Swedish and its separate guide lists Norwegian | Keep exact public Norwegian scope and use native examples for shared forms. Apply distinct punctuation and word forms |
| Dutch and Belgian Dutch | LT, G | Regional accepted forms and the advanced Dutch check set |
| English, British, American, Australian, Canadian, Indian, New Zealand and South African English | LT, G, P | Keep each spelling and grammar profile distinct. P separately names General English. LT documents an Oxford spelling option for British English |
| Esperanto, Galician, Greek, Irish, Khmer | LT | Language-specific word boundaries and valid language forms |
| French, Belgian French, Canadian French and Swiss French | LT, G | Regional punctuation, spelling, numbers and typography |
| German, Austrian German, German as used in Germany and Swiss German orthography | LT, G | Distinct standard written variants, such as acceptance of ss at the relevant word in the Swiss fixture |
| Italian | LT, G | Grammar, spelling and language-specific punctuation |
| Polish, Romanian, Slovak, Tagalog and Ukrainian | LT, G | Locale-specific forms and finished-sentence advice. Advanced Polish check set stays distinct |
| Portuguese, Brazilian Portuguese, pre-agreement Angolan and Mozambican Portuguese, and Portuguese as used in Portugal | LT, G | Regional and orthographic differences remain explicit. Keep the advanced Portuguese check set |
| Russian, Slovenian, Tamil | LT | Tamil needs a dedicated segmentation fixture and carries no LT spelling claim |
| Spanish, Argentinian, Mexican and Spanish as used in Spain, and voseo | LT, G | Keep the second-person variant and corresponding verb agreement. Keep the advanced Spanish check set. Keep the three G regional choices explicit. Do not assume every regional choice has its own source rule set |

Use the [separate LT language guide][lt-other] for Norwegian and
Oxford English. LT names advanced checks for English, French, German,
Spanish, Dutch, Polish and Portuguese. G describes basic multilingual
correction for its list, and finished-sentence clarity, fluency and
concision for those languages except Chinese. P names English only.
A narrower source promise cannot erase a broader one. Track genuine
source disagreements in QR-115 and keep every public variant in scope.

| Capability | Exact source conditions to record in the coverage manifest |
|---|---|
| Basic spelling, punctuation and grammar | Per category and variant. G's multilingual guide distinguishes its desktop, web and mobile surfaces. Its public site gives different counts for iOS and Android. LT publishes its spelling exclusions |
| Finished-sentence clarity, fluency and concision | G help lists the non-English set above except Chinese, with Japanese limited to Windows and Mac. Preserve the completed-sentence condition |
| Advanced grammar and tone | G's public page names English, Spanish, French, German, Portuguese and Italian. LT's advanced checks cover its named seven languages. Classify each exact check instead of marking every advanced category present |
| Regional writing modes | G's public page lists some regional names and also says its non-English models do not tailor to those dialects. Its dialect help separately documents tailored English. Keep regional selection, underlying language-level checks and specific dialect rules distinct for Argentina, Mexico, Spain, Brazil and Portugal |
| Automatic correction and completion | G documents optional mobile automatic correction and completion in its [keyboard guide][mobile]. Track those outcomes separately under DR-114. A typing-time suggestion with human review cannot prove an automatic effect |

Record each capability, locale, source, plan, surface and use condition
as separate manifest fields. Source surfaces describe evidence about
the reference. They cannot grant access to another product on our PC.
Route native integrations and any missing action to the owner under
DR-114. Spec 0121 requires exact human confirmation for our edits.
QR-115 holds exact evidence where correction and translation lists
disagree or refer to different kinds of regional support.

Translation has its own directional coverage and test matrix. A
language listed only for translation does not itself add source
evidence for a correction check. This distinction preserves, for
example, Spanish regional translation output while keeping the
correction variant explicit where independently documented.
Do not invent a source promise for an unspecified dialect. Treat
the generic entry as its own profile with only admitted rules.

## Check packs and records

A check pack contains signed, immutable data for one language and
its declared variants. Bind every typed record and result to its
version and platform admission. Rust owns the output checks and
uses the admitted tokenisation and language pipeline under spec
0026 or an original Rust rule. Use Model Hub and the scheduler
for any model stage. Pack data never runs as a script. A licensed
payload must pass spec 0076 provenance and product licence checks.

| Record | Fields and allowed values |
|---|---|
| Language key | Canonical BCP 47 tag and explicit orthography ID where needed. Use the registered tag for the underlying language, script and region. Store dialect or spelling policy separately when no registered subtag expresses it. The generic key is explicit |
| Language range | Spec 0121 range, language key, source override/document preference/detected/unknown, optional model revision and confidence class |
| Check pack manifest | Pack ID, version, language keys, supported capability IDs, limited capabilities, tokenizer/rule artefact identities and digests, checked licence and provenance record IDs, platform evidence and signature |
| Rule | Stable rule ID and version, variants, applicability conditions, trigger and exceptions, evidence range, category, certainty class, protection rules, explanation ID and optional proposed edit builder |
| Correction | Spec 0121 finding plus basic/advanced profile, exception context, detected language key, source pack version and local action proposal |
| Scoped word exception | Workspace or site, optional actor, language key, normalised form and original display form, creator and active version. Used for spelling only |
| Word exploration | Exact selected source range, language, source ID, possible senses and optional alternatives with usage or register labels. No recommendation where the sense is uncertain |
| Coverage receipt | Requested and evaluated ranges per category and language, excluded ranges and reason, required pack revisions and terminal outcome |

Use [BCP 47][bcp] for language tags and [Unicode UAX #29][uax] for
extended grapheme clusters, the characters a writer sees and edits.
Pin the exact revisions of segmentation data and dependency in the
pack receipt. Apply language tailoring to token and sentence discovery.
The display and editing range still uses whole grapheme boundaries.
Keep original bytes. Analyse a normalised view with a reversible map
to source ranges. If a correction cannot map to whole graphemes in the
same document revision, give an explanation without an edit action.

## Execution

1. Take a request snapshot and validate the scope, selected ranges,
   exact revision, permissions, consent and requested checks. Exclude
   code, command examples, tokens and destination addresses from
   ordinary prose changes. Treat quoted dialogue, citations and marked
   language examples using the selected document policy. Exclusion
   needs a visible reason when it removes a selected range.
2. Choose the most specific explicit range setting first, then the
   document's fixed language if selected. In automatic mode use
   independently admitted detection and the document's allowed
   variant choices. Never infer a regional spelling dialect from a
   short ambiguous sentence. Use an admitted generic profile or
   label the range uncertain. Equal automatic candidates yield unknown.
3. Run the requested categories from the exact pack. Basic checks
   cover correctness. Detailed checks add advisory style and
   selected advanced rules. Use an admitted lexical candidate
   generator for spelling and constrain an inflection or agreement
   proposal to an exact source span. Casing and spelling exceptions
   use the pack's language and orthography rules. One dialect's
   exception cannot affect any other. A word in a dictionary cannot
   suppress a valid grammar or house-style finding.
4. For word exploration, read admitted dictionaries, thesauri and
   inflection data for the selected language. List senses first when
   several meanings qualify. Present alternatives by sense and usage
   and keep names or specialist terms when the context is uncertain.
   Choosing a replacement asks for the same exact range review.
5. Treat models as sources of candidates. Validate output against
   the input range, applicable rule and pack. Protect names, URLs,
   identifiers, numbers, dates, measurements, negation and quotation
   from silent changes. Where an existing name or number is itself
   suspect, explain the evidence and require a separate exact review
   before offering an edit. Do not invent replacements as fact.
6. Deduplicate by rule, language, revision and source range. Order by
   position, correctness before style, shortest range and stable
   rule ID. Keep distinct overlapping advice visible. Show that
   two proposals overlap and recheck after applying either one.
   Accept or dismiss each suggestion through spec 0121. A reviewed
   group has one base revision and a sorted list of disjoint ranges.
   Recheck editability, applied range boundaries and current scope
   for every range and their union immediately before commit.
   One overlap, protected range, stale or revoked item rejects the
   whole group. Undo uses one reviewed inverse group and refuses a
   conflict with an intervening edit rather than losing that edit.
7. Check current permissions and source, pack and policy revisions
   before publishing. The coverage receipt accounts for all requested
   ranges, including unsupported, timed out or paused work. No
   findings plus incomplete coverage is never a clean result.

## Structure checks for account and book numbers

“Check a number” offers “Bank account number (IBAN)” and “Book
number (ISBN)”. It checks one exact selected range. A workspace or
site policy may enable either check independently on an admitted
typed field. The service resolves a stable field ID and its range
from the authoritative source document and selected revision. A
native request can name a field only through an admitted mapping.
Never infer a field from a caller-supplied range or scan unmarked
text. Enforce spec 0128 T04, T11 and T12 for group, context and role
and the spec 0121 read and check permissions on both entry routes.
A selected range must be readable and checkable in its own right.
A denied field cannot be reopened through the selection route.

| Record | Contract |
|---|---|
| Structure check request | Spec 0121 scope, actor, revision and one grapheme-aligned source range, identifier kind `iban` or `isbn`, source `selection` or `typed_field`, policy revision and check data version. Typed fields also bind field ID, field mapping admission revision and resolver result. Only the service assigns this source type |
| Structure check result | Request ID, exact source range, rule revision, and `matches_structure`, `format_problem`, `check_digit_problem` or `unavailable`. For an available check, include subtype `iban`, `isbn_10`, `isbn_13` or `undetermined`. For a refusal, use the scope-safe common error. No field returns the selected value or a replacement |
| Check data | Signed spec 0076 receipt binds source edition and digest, extracted national template data digest, ISBN rule revision, derivation and review IDs, synthetic fixture digest and platform admission. Templates list only exact length and character classes per position. No example account numbers |

Trim outer ASCII spaces. Before trimming, a candidate over 80
bytes is a format problem. An empty result after trimming is also
a format problem. For an IBAN accept either a plain
ASCII token or space-separated blocks of four characters with a
one-to-four-character final block. ASCII letters are matched without
case. Reject hyphens, tabs, newlines, punctuation and Unicode
lookalikes. After parsing, require two letters, two digits and
one to thirty account characters. Invalid shape is a format problem.
An unavailable admitted country table or missing country code then
gives “This country format is unavailable”. For a known country,
a length or character-class mismatch is a format problem. Once the
template matches, check digits outside 02 to 98 fail the check digit
test. Move the first four characters to the end. Map A to 10 through
Z to 35 and take the remainder after division by 97. Only remainder
1 passes. Use the [Swift register][iban] and [EBS204 §6][iban-check].
No network lookup occurs.

For ISBN accept ASCII digits with optional single ASCII spaces or
hyphens between characters. Allow ASCII X or x only as the last
character of a ten-character candidate. In the ten-character form
treat terminal X as 10 and multiply each position by the weights
10 down to 1. Require the sum to be divisible by 11. In the
thirteen-digit form require 978 or 979, excluding [the 9790 prefix for music
identifiers][ismn]. Starting at the first digit, multiply the
thirteen positions by alternating weights 1 and 3. Require the
sum to be divisible by 10. Mark other lengths or prefixes as a
format problem. Resolve shape and length first,
then select the matching rule. Use the [ISBN Agency guide][isbn]
and the [legacy ISBN guide][isbn-10]. Group separators in the input are
accepted for readability. This check has no publisher-range lookup.

After checking, show “The number follows the expected format” or
“The check digit does not match”. A malformed input says “Check the
length and characters for this kind of number”. In all three cases
also show “This checks the number’s structure only”. A matching
check neither establishes account ownership or a payment destination
nor confirms that an ISBN was assigned. No replacement, correction,
conversion, link or upload follows a result.

Run the deterministic check in the Rust service using authorised
text from the bound revision. Do not send the selected number to
a model, extension, MCP client, analytics, logs, durable findings,
policy review queue or a report export. Keep temporary raw and
normalised bytes in the protected job scope. Drop both when the
job exits, including on error or cancel. Recovery never restores
this transient job or its result. Render an inline marker and an
accessible status without repeating the input. Before reading and
before publishing, recheck the exact revision,
generation, live grants, field mapping, policy and data admission.
On pause discard the temporary candidate and withhold a result.
After resume a new request resolves the current input. A stale,
cancelled or denied request ends with the common error and exposes
no range or classification. A permitted request with an unavailable
rule or country template shows incomplete coverage. Publish only to
the checking actor on the authorised editor surface. Recheck the
bound generation and authority before showing a queued result.
Discard queued replies and clear visible markers on pause, cancel,
supersession, erasure, authority loss and process restart. Clear
only the check state. Preserve the source document under its own
retention and revision rules.

## Data

The records of the tables above are the data of this spec. Each is
scoped by site or local writing workspace and bound to its pack,
rule, policy and source revisions.

- Stored: check pack manifests and their signed payloads, scoped
  word exceptions, dismissals (rule and text fingerprint within the
  exact document scope), coverage receipts and findings of spec 0121.
- Held only for the running job: the selected number and its
  normalised form in a structure check. Neither is stored, logged,
  exported or restored.
- Files and temporary work follow the file ledger and erasure rules
  of specs 0030, 0032 and 0118. Source documents keep their own
  retention and revision rules.

## User controls and limits

Select “Check as I type”, “Check selection” or “Check document”.
Choose “Correct mistakes” or “Detailed review”. The detailed view
shows advice about tone and style separately from a grammatical
mistake. Offer enabled check categories, ignore for the current
document, and add to my dictionary for a spelling suggestion.
The bulk action selects only fully reviewed proposals for one
revision. Store dismissal as a rule and text fingerprint within
the exact authorised document scope. A changed range must be
rechecked. Increment the input generation on each committed edit
and on a relevant document, scope, language, pack, policy or grant
change. Retain at most the current job and one request for the
latest generation per editor. Cancel superseded work and discard
its result. While an input method is composing text, wait for its
commit. A blocked pack shows “This check is unavailable for this
text” with the reason and the coverage that did finish.

A saved document check checks at least 150,000 Unicode scalar values
without asking the writer to split it. The ceiling is 1,000,000
Unicode scalar values (proposed), and each check has a deadline of
120 seconds (proposed). A check over the ceiling or the deadline is
reported as incomplete for the affected range. Chunk at paragraph and sentence
boundaries, with context windows and shared whole-document
consistency state where the category needs it. Each span is owned by
one batch. A range too large to fit one pass is evaluated in scheduled
slices or reported as incomplete for the affected check. For each
requested category, the non-overlapping primary ranges must exactly
cover the requested UTF-8 byte set as checked, explicitly excluded
or incomplete. Context crossing slice boundaries counts as context
only. Preserve the document on pause. Traces carry ids and reason
codes without document text, quoted snippets or raw tokens.
Use spec 0078 for per-machine resource ceilings and competing jobs.
QR-115 records exact admitted per-pack bounds and the workload proof.

## Settings

| Setting | Type | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `writing.check.trigger` | as I type, selection, document | selection (proposed) | three values | next request |
| `writing.check.depth` | correct mistakes or detailed review | correct mistakes (proposed) | two values | next request |
| `writing.language.mode` | automatic or fixed | automatic (proposed) | two values | next request |
| `writing.language.document` | BCP 47 language key | none | a key of an admitted pack | next request |
| `writing.language.range` | language key per range | none | a key of an admitted pack | next request |
| `writing.structure.iban` | on or off, per workspace or site | off | two values | next request |
| `writing.structure.isbn` | on or off, per workspace or site | off | two values | next request |

Fixed limits, which are not settings: a saved document check covers at
least 150,000 Unicode scalar values; a number candidate over 80 bytes
is a format problem; an IBAN has 5 to 34 normalised characters (two
letters, two digits and one to thirty account characters); at most
the current job and one request for the latest generation are kept per
editor. Per-machine ceilings come from spec 0078, and QR-115 records
the exact per-pack bounds. A range setting beats the document setting.
Group, context and role limits for the structure checks come from
spec 0128. No setting sends a selected number to a model, extension,
MCP client, analytics or log, or lets a check reach the network.

## Failure codes

Codes SUT-5100 to SUT-5199 are reserved for this spec (DR-130). A scope or
permission refusal uses SUT-5000 of spec 0121 and exposes no range or
classification.

| Code | Meaning |
|---|---|
| SUT-5100 | “This check is unavailable for this text.” A pack is blocked, missing, withdrawn or fails admission, with the coverage that did finish |
| SUT-5101 | The language of a range is unknown or the automatic candidates are equal. The range is labelled uncertain |
| SUT-5102 | A correction cannot map to whole graphemes in the same revision. An explanation is shown without an edit |
| SUT-5103 | Input generation, revision, grant, policy or pack changed. The result is withheld |
| SUT-5104 | “This country format is unavailable.” No admitted country table or country code |
| SUT-5105 | Coverage is incomplete because of an unsupported, timed out or paused range. Never a clean result |
| SUT-5106 | A reviewed group is rejected whole: overlap, protected range, stale or revoked item, or an intervening edit at undo |

## Security and privacy

- Checks run locally on admitted hardware. Member consent, role
  checks, protected transport and encrypted managed storage stay in
  force.
- Check both grants again before reading and before publishing a
  result. Show a result only to the checking actor on the authorised
  editor surface.
- A selected number never goes to a model, extension, MCP client,
  analytics, log, durable finding, policy review queue or report
  export. Raw and normalised bytes are dropped when the job exits,
  including on error or cancel. Recovery never restores them.
- Pack data never runs as a script. A licensed payload must pass the
  provenance and product licence checks of spec 0076.
- Treat text, models and their output as data. Validate model output
  against the input range, rule and pack, and protect names, URLs,
  identifiers, numbers, dates, measurements, negation and quotation.
- Traces carry ids and reason codes only, never document text, quoted
  snippets or raw tokens.
- A structure check does not confirm account ownership, a payment
  destination or that an ISBN was assigned. It makes no network
  lookup.
- Fixtures contain original synthetic text and no example account
  numbers from any source.

## Requirements

| Requirement | Acceptance test |
|---|---|
| REQ-0122-UNION | IT-0122-UNION: remove each source variant and each supported capability in turn from a synthetic coverage manifest. Missing promises block. Distinguish Persian spelling absent in the reference from Japanese and Chinese spelling documented by another reference. Add an unsupported guessed dialect and refuse unproved admission |
| REQ-0122-RANGE | UT-0122-RANGE and BDD-0122-RANGE: mix a right-to-left paragraph, combining letters, unrelated scripts, emoji and quoted text. Exercise insertion, removal, undo, keyboard navigation and screen reader announcement; an offset inside a grapheme cannot change text |
| REQ-0122-CHOICE | UT-0122-CHOICE: explicit span versus fixed document language, generic versus ambiguous regional evidence and a confidently detected language. No inference of national dialect from ambiguous spelling. Change a range preference mid-job and withhold stale output |
| REQ-0122-RULES | UT-0122-RULES: for every promised check and variant use original correct, incorrect and ambiguous fixtures, and an adversarial exception. Include false friends, proper names, consistency of times and numbers, typography, passive voice, unclear pronouns, concise wording inclusive language and alternatives for two distinct senses where applicable. Record false positives and missed findings by class |
| REQ-0122-APPLY | IT-0122-APPLY: two overlapping suggestions, a protected literal, a stale source revision, a rule-pack replacement and a repeated confirmed request. Try a late protected item and grouped undo after an intervening edit. Only the exact still-valid change set may apply, once. A stale item aborts every action in the group |
| REQ-0122-IBAN | UT-0122-IBAN and IT-0122-IBAN: generate original synthetic account strings from test-only national templates. Test digit-only and mixed letter templates, grouped and plain text, lowercase, empty, 80 and 81 source bytes, 34 and 35 normalised characters, canonical check digits 02 and 98, unknown country, malformed shape with unknown country, false national shape with valid remainder, check digits 00/01/99 even with remainder 1, mutated digit and forbidden Unicode. A missing table reports incomplete; a withdrawn admission between read and publish suppresses output |
| REQ-0122-ISBN | UT-0122-ISBN and IT-0122-ISBN: make original synthetic candidates with ISBN-10 terminal X and digits, 978 and 9791–9799 cases with zero and nonzero check digits, single and doubled separators, wrong prefix, checksum-valid 9790, wrong length, whitespace only and misplaced X. Mutate a digit and check the error classification. A correct checksum shows only a structure result |
| REQ-0122-STRUCTURE | BDD-0122-STRUCTURE and E2E-0122-STRUCTURE: select a synthetic number and check it without outbound traffic. Repeat through a typed workspace or site field with its exact active permission. Test policy-off, an unmarked number, forged field identity, wrong kind, missing or stale mapping, an actor with read but no check grant, and a selection inside a denied field. After pause resume on a changed revision. Denial, cancellation, stale input and permission loss never expose a classification or source span. Withhold a delayed IPC reply after erasure or a permission or policy change. Crash mid-check and reopen the editor; no result resumes and the draft stays intact. Repeat on each v1 platform and try another site. Inspect IPC traces, logs, files, metrics and exports for leaked characters or replacements |
| REQ-0122-BOUNDS | E2E-0122-BOUNDS: on each v1 platform check empty, one, the supported document floor and admitted cap, then one over. Pause, resume, fail a dependency and revoke access. Detect overlap, missing coverage and context counted twice. Exercise rapid input, composition and a switch of generation. Keep typing usable and show neither revoked nor superseded results |
| REQ-0122-OFFLINE | E2E-0122-OFFLINE: provide an eligible saved audit snapshot from 0044, disable outbound network, then check staff or consented member text supplied by the audit. Try every grant failure, synthetic prompt injection, corrupt manifest and withdrawn pack. Observe no outbound transfer, unauthorised result, site effect or search rank input |
| REQ-0122-SETTINGS | UT-0122-SETTINGS: each setting at its default and each allowed value, one value outside range refused, and the range setting beating the document setting. Structure checks stay off unless a policy enables them |
| REQ-0122-ERRORS | UT-0122-ERRORS: provoke each failure code once. A scope refusal returns SUT-5000 with no range or classification. Messages contain no document text |

Release results show correctness, false positive and missed-finding
rates per category and language/dialect, repeatability, range safety
and latency and memory use on admitted profiles. Rates from English
or another well-supported language cannot clear a different variant.
Bind the accuracy and resource thresholds, fixed evaluation fixture
digest, adjudication and exact pack/platform to signed evidence
before a variant can pass. Keep every required capability and
variant in the release matrix. An unavailable pack has an explicit
refusal path and cannot satisfy the release gate.

## How it is tested

Designed, not run.

- **Unit (UT-0122):** RANGE, CHOICE, RULES, IBAN, ISBN and SETTINGS,
  each with original synthetic fixtures and one value just inside and
  one just outside each limit. ERRORS covers every failure code.
- **Behaviour (BDD-0122):** RANGE and STRUCTURE through the editor,
  with keyboard and screen reader.
- **Integration (IT-0122):** UNION, APPLY, IBAN and ISBN against a
  synthetic coverage manifest, pack and proposal group.
- **End to end (E2E-0122):** STRUCTURE, BOUNDS and OFFLINE on each v1
  platform, with outbound access disabled.
- Release results use the signed accuracy and resource evidence
  described above. Each test id is one named suite under Bazel.

## Changes from the source

The public LanguageTool, Grammarly and ProWritingAid language lists
differ. This spec freezes their union as dated on 20 November 2026
and keeps every variant in scope, so a narrower source cannot erase
a broader one. Translation-only languages add no correction
evidence. Where the sources disagree, QR-115 holds the evidence.
DR-113 admits all documented writing languages while the interface
stays British English under DR-081. The IBAN and ISBN structure
checks are added to feature 186 as local, deterministic checks with
no lookup. Native integrations and automatic correction are routed
to the owner under DR-114.

[lt]: https://help.languagetool.org/hc/en-us/articles/39254526141463-What-languages-does-LanguageTool-support
[lt-other]: https://languagetool.org/insights/post/product-languages-supported/
[g]: https://support.grammarly.com/hc/en-us/articles/39345737251469-Introducing-Multilingual-Suggestions
[gd]: https://support.grammarly.com/hc/en-us/articles/115000090971-Does-Grammarly-support-languages-other-than-English
[gm]: https://www.grammarly.com/languages
[mobile]: https://support.grammarly.com/hc/en-us/articles/360041391992-Manage-your-keyboard-settings-in-Grammarly-for-iPhone
[p]: https://help.prowritingaid.com/article/61-which-languages-does-prowritingaid-support
[bcp]: https://www.rfc-editor.org/info/rfc5646/
[uax]: https://www.unicode.org/reports/tr29/

[iban]: https://www.swift.com/standards/data-standards/iban-international-bank-account-number
[iban-check]: https://www.cnb.cz/export/sites/cnb/cs/platebni-styk/.galleries/iban/download/EBS204.pdf
[ismn]: https://ismn-international.org/ismn/ranges/
[isbn]: https://www.isbn-international.org/content/isbn-users-manual/29
[isbn-10]: https://publications.gc.ca/collections/collection_2024/bac-lac/SN3-207-1984-eng.pdf
