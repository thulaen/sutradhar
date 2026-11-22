# Writing suite coverage

This register connects public reference behaviours to the writing
suite designed in [spec 0121](../specs/0121-clean-room-writing-suite.md).
DR-112 to DR-114 set the version 1 scope. Use original rules, text
and screens. Apply spec 0076 to each conformance obligation.

## Meaning of a row

An outcome is a distinct thing a person can do or observe. A
source row identifies its dated reference, user role and surface.
Several rows can use the same Sutradhar behaviour. A test must
show the behaviour holds for each relevant variant. Pricing,
marketing wording and identifiers identify source scope and cannot
establish the operation or implementation of a capability.

For each row of the signed scope record, retain source section,
observation date, source plan and surface, an independently written
statement of the outcome, local behaviour and scope, owner spec, applicable
variant
IDs, requirement and test IDs, and any outside effect with its
exact owner decision. Keep separately proved synthetic fixtures,
measured operating bounds and rights records. A row represents a
requirement before it supplies proof. A shared design or test that
exercises only one case cannot establish every variant.

Check completeness both ways. Every documented outcome needs a
row. Every design and test that claims a reference outcome needs
its exact row. Missing source details, conflicting claims and
questions about reach remain explicit. A confirmed host alias
or renamed tool can share a row only when its inputs, visible
effects and conditions match. Evidence for the local substitute
and for any missing effect are separate release obligations.

## Owning specifications

| Spec | Main contract | Proof record |
|---|---|---|
| [0121](../specs/0121-clean-room-writing-suite.md) | Shared scope, revisions, authority and coverage | QR-114 |
| [0122](../specs/0122-multilingual-writing-checks.md) | Language and proofreading | QR-115 |
| [0123](../specs/0123-writing-analysis-reports.md) | Measures, document reports and comparisons | QR-116 |
| [0124](../specs/0124-writing-revision-and-translation.md) | Rewriting, local drafting and translation | QR-117 |
| [0125](../specs/0125-story-planning-and-critique.md) | Story planning, long critique and publication drafts | QR-118 |
| [0126](../specs/0126-writing-sessions-and-learning.md) | Sessions, markers and local learning | QR-119 |
| [0127](../specs/0127-writing-sources-and-origin-records.md) | Sources, citations, similarity and origin | QR-120 |
| [0128](../specs/0128-writing-policies-and-team-controls.md) | Personal and shared writing policy | QR-121 |
| [0129](../specs/0129-writing-files-and-review-exchange.md) | File profiles and offline review | QR-122 |
| [0130](../specs/0130-writing-surfaces-and-local-assistants.md) | Exact host routes and local assistants | QR-123 |

## LanguageTool

Sources were observed on 20 November 2026. Source tags refer to
[Premium plans][lt-plans], [languages][lt-language],
[local variants][lt-variants], [editor][lt-editor],
[advanced extension][lt-extension],
[team plan][lt-team], [dictionary import][lt-dictionary],
[installation surfaces][lt-hosts], [host limits][lt-host-limits],
[Windows editor][lt-windows], [access guide][lt-access],
[Outlook][lt-outlook], [Overleaf][lt-overleaf],
[macOS][lt-mac], [paraphrasing modes][lt-rephrase] and
[paraphrasing languages][lt-rephrase-locales],
[API access][lt-api] and [public API fields][lt-fields].

The native client, file handling and check-engine outcomes remain
separate. Use spec 0130 for host rows and DR-114 for additional
outside effects. The relevant spec's fixture must enforce the exact
conditions from the source. A source plan or entitlement becomes a
scope field. The rows show the closest named test and any
missing behaviour. QR-124 owns the separate exact variant
IDs, test instantiations and evidence not yet admitted. No subscription or
provider request is implied by a row.

| ID | Documented outcome and reference | Owning design and verification |
|---|---|---|
| LT01 | Basic spelling checks for eligible languages (languages, editor) | 0122 RULES and UNION. Test exact regional accepted forms and category-specific limits Explicit trace REQ-0122-RULES and UT-0122-RULES |
| LT02 | Basic contextual grammar checks (editor, plans) | 0122 RULES. Test valid and invalid agreement within a supported language Explicit trace REQ-0122-RULES and UT-0122-RULES |
| LT03 | Basic punctuation (editor, plans) | 0122 RULES. Test exact ranges and locale punctuation Explicit trace REQ-0122-RULES and UT-0122-RULES |
| LT04 | Detection and manual writing language selection (fields, hosts) | 0122 CHOICE. Compare detected, explicit and uncertain choices Explicit trace REQ-0122-CHOICE and UT-0122-CHOICE |
| LT05 | Regional spelling profiles and Oxford English (languages, variants) | 0122 UNION and CHOICE. Preserve profiles independently Explicit trace REQ-0122-UNION and IT-0122-UNION |
| LT06 | Advanced language checks in English, French, German, Spanish, Dutch, Polish and Portuguese (languages, plans) | 0122 UNION and RULES. Bind the source language and exact category Explicit trace REQ-0122-RULES and UT-0122-RULES |
| LT07 | Optional more detailed style review (extension, plans, fields) | 0122 RULES. Separate detailed advice from ordinary corrections Explicit trace REQ-0122-RULES and UT-0122-RULES |
| LT08 | Misspelled known names and terms (extension, plans) | 0122 RULES. Distinguish supported correction from unknown identity Explicit trace REQ-0122-RULES and UT-0122-RULES |
| LT09 | Time and number consistency (extension, plans) | 0123 R20 and 0122 RULES. Keep formatting and factual value separate Explicit trace REQ-0123-MEASURES and UT-0123-MEASURES |
| LT10 | Typography and formatting issues (extension, editor) | 0122 RULES and 0123 R20. Exercise stated rules by locale and layout Explicit trace REQ-0122-RULES and UT-0122-RULES |
| LT11 | Contextual vocabulary, meanings and foreign terms (extension, fields) | 0122 RULES. Test alternate senses and supported false-friend context Explicit trace REQ-0122-RULES and UT-0122-RULES |
| LT12 | Synonym and chosen-range alternative (hosts, Windows app) | 0122 word exploration. Use appropriate senses for the selected occurrence Explicit trace REQ-0122-RULES and UT-0122-RULES |
| LT13 | Tone advice (editor, plans) | 0123 R28. Contrast whole-document and passage evidence Explicit trace REQ-0123-CATALOGUE and IT-0123-CATALOGUE |
| LT14 | Suggest while typing (hosts, extension) | 0122 BOUNDS and 0130 RUN. Check only the eligible latest revision Explicit trace REQ-0122-BOUNDS and E2E-0122-BOUNDS |
| LT15 | Review and accept a correction in one action (extension) | 0122 APPLY. Exact preview and consent fence per edit Explicit trace REQ-0122-APPLY and IT-0122-APPLY |
| LT16 | Enable, disable and filter individual rules or groups (fields) | 0122 RULES and 0128 RULES. Test applicable policy, exemption and precedence Explicit trace REQ-0128-RULES and UT-0128-RULES |
| LT17 | Strict versus ordinary checks and preferred regional variants (fields, extension) | 0122 CHOICE and RULES. Prove availability by detailed category, not a generic level flag Explicit trace REQ-0122-CHOICE and UT-0122-CHOICE |
| LT18 | Personal dictionary (editor, plans) | 0128 T01. Preserve its version and language Explicit trace REQ-0128-RULES and UT-0128-RULES |
| LT19 | Team dictionary (team, dictionary) | 0128 T02. Separate personal edits and team CSV replacement. Check first-column entries, the 1,000-word source boundary, removal preview, export and undo. Confirm exact role where the source is ambiguous Explicit trace REQ-0128-APPROVAL and IT-0128-APPROVAL |
| LT20 | Personal writing rules and style guide (editor, extension) | 0128 T03. Test single-actor rule and independent version Explicit trace REQ-0128-RULES and UT-0128-RULES |
| LT21 | Shared consistency rules (team, plans) | 0128 T03 and T04. Test group policy and observed scope Explicit trace REQ-0128-RULES and BDD-0128-RULES |
| LT22 | Focused writing mode (editor) | 0126 H01. Restore focus and chosen text without lost work Explicit trace REQ-0126-COVER and IT-0126-COVER |
| LT23 | Writing goals (editor) | 0126 H04. Check the target and observed progress Explicit trace REQ-0126-GOAL and UT-0126-GOAL |
| LT24 | Writing and productivity statistics (editor, plans) | 0123 R03 and 0126 H03. Keep live text measure and session record distinct Explicit trace REQ-0123-MEASURES and UT-0123-MEASURES |
| LT25 | Detailed writing tips (editor) | 0123 R09 and R25. Show actual supporting ranges Explicit trace REQ-0123-CATALOGUE and IT-0123-CATALOGUE |
| LT26 | Longer text check in editor and extension (plans, extension) | 0122 BOUNDS. Record 100,000 characters in the extension guide and 150,000 per text field in the Premium and team plans. Test separately measured local bounds Explicit trace REQ-0122-BOUNDS and E2E-0122-BOUNDS |
| LT27 | Rewrite a sentence in general, formal, concise, fluent or simple modes (paraphrasing modes and languages) | 0124 MODES. Record English in US, British, Australian, New Zealand, Canadian and South African varieties, plus French, German, Dutch, Spanish and Portuguese. Verify each mode, scoped plan and preserved fact Explicit trace REQ-0124-MODES and IT-0124-MODES |
| LT28 | Translate selected text in the extension (extension) | 0124 LANG and QR-117. Retain unresolved published pair list before coverage claims Explicit trace REQ-0124-LANG and IT-0124-LANG |
| LT29 | Open Word text and return a formatting-preserving copy (editor, hosts) | 0129 F04/F05 and FIDELITY. Test same-format and selected conversion separately Explicit trace REQ-0129-FIDELITY and UT-0129-FIDELITY |
| LT30 | Browser as writing surface (hosts) | 0130 S04. Split Chrome, Firefox, Edge, Safari and Opera and test the local equivalent. Each native route keeps its own gap Explicit trace REQ-0130-ROUTE and IT-0130-ROUTE |
| LT31 | Windows and macOS desktop, including documented offline use (access guide) | 0130 S01/S03. Separate standalone offline and cross-app modes. Keep macOS accessibility capture and Windows and macOS host cases distinct. A version 1 desktop cannot prove macOS support Explicit trace REQ-0130-ROUTE and IT-0130-ROUTE |
| LT32 | iPhone or iPad writing surface (hosts) | 0130 S06. Prove the safe file route; native app and device workflow keep separate gaps Explicit trace REQ-0130-ROUTE and IT-0130-ROUTE |
| LT33 | Word, Google Docs, LibreOffice and Pages compatibility (hosts, plans) | 0129 format list and 0130 S03/S04. Keep each host's native editing and file exchange separate Explicit trace REQ-0129-MATRIX and IT-0129-MATRIX |
| LT34 | Gmail, Apple Mail, Thunderbird and Outlook (hosts, plans, Outlook) | 0130 S05. Separate Windows Outlook desktop and Windows or macOS selection shortcut cases from any native mail add-on. Keep a local draft route and each unresolved host effect Explicit trace REQ-0130-ROUTE and IT-0130-ROUTE |
| LT35 | WordPress and general web editor writing (hosts, host limits) | 0130 S02 and S04, 0044 owner-staff eligibility with 0121. Native WordPress effect and extension-only workflow keep distinct cases Explicit trace REQ-0121-STAFF-MEMBER-PAGES |
| LT36 | Partial browser coverage for Notion, possible extension behaviour in Trello and Asana, and no supported native route in the cited cases (host limits) | 0130 ROUTE. Keep the documented degree and surface. Do not infer native integration or add unsupported apps Explicit trace REQ-0130-ROUTE and IT-0130-ROUTE |
| LT37 | Structured markup or text request to a proofreading service (fields) | 0130 S10, QR-123. Test plain text and marked-up text with a reversible source-range map. Keep the public input shape distinct Explicit trace REQ-0130-MCP and IT-0130-MCP |
| LT38 | Report supported languages to a client (fields) | 0122 UNION and 0130 MCP. Exact response identity and variant list need client-delivery proof in QR-124. UNION covers the documented languages only Explicit trace REQ-0122-UNION and IT-0122-UNION |
| LT39 | Public hosted proofreading API (API, fields) | 0130 S10. Record public networking, request bounds and any access condition as separate effects under DR-114 Explicit trace REQ-0130-ROUTE and IT-0130-ROUTE |
| LT40 | Paid hosted proofreading API (API, fields) | 0130 S10. Track authenticated mode and distinct published quotas. Public/premium guide lists 20/80 requests per minute, 75,000/300,000 characters per minute and 20,000/60,000 per request. Local bounds need their own proof Explicit trace REQ-0130-ROUTE and IT-0130-ROUTE |
| LT41 | Self-hosted proofreading API (API) | 0130 S10. Record the distinct standalone service, owner control and transport route. Review local-only equivalence separately Explicit trace REQ-0130-ROUTE and IT-0130-ROUTE |
| LT42 | Cross-device saved writing and preferences (editor, plans) | 0129 F01 and S06 in 0130. Local recovery and deliberate exchange leave live sync as a gap Explicit trace REQ-0129-ERASE and E2E-0129-ERASE |
| LT43 | Editor browser support in Overleaf legacy source mode (Overleaf) | 0130 ROUTE. Keep the exact source editing mode separate from rich text Explicit trace REQ-0130-ROUTE and IT-0130-ROUTE |
| LT44 | Editor browser support in Overleaf rich text mode (Overleaf) | 0130 ROUTE and 0129 FIDELITY. Local formats cannot prove native use Explicit trace REQ-0130-ROUTE and IT-0130-ROUTE |
| LT45 | Correct IBAN and ISBN input in the team plan (team) | Original Sutradhar selection and field routes follow public standards. The source does not specify exact field surfaces or validation rules. REQ-0122-IBAN / UT-0122-IBAN / IT-0122-IBAN and REQ-0122-ISBN / UT-0122-ISBN / IT-0122-ISBN define distinct offline checks. BDD-0122-STRUCTURE covers selection and typed fields. QR-124 requires exact tested field and surface evidence. Number formatting and sensitive-data classification have different effects |
| LT46 | Consistency of names and titles across authorised team writing (team) | 0123 R20 and 0128 T03. Test names and titles independently Explicit trace REQ-0123-CATALOGUE and IT-0123-CATALOGUE |
| LT47 | Team owner, delegated rights, invitations and member edits or removal (team) | 0101 and 0128 T12/T17. Separate site group membership from hosted invitations, seats, billing or owner reassignment Explicit trace REQ-0128-APPROVAL and IT-0128-APPROVAL |
| LT48 | Draft assistance in the Premium browser extension (extension) | 0124 MODES. Test draft suggestions independently from rewriting, with native effect in QR-124 Explicit trace REQ-0124-MODES and IT-0124-MODES |
| LT49 | Match classification, source offset, explanation and possible replacements returned to a client (fields) | 0130 MCP uses 0122 offsets and bounded results. Exact rule and replacement metadata and markup-to-source proof need QR-124 Explicit trace REQ-0130-MCP and IT-0130-MCP |
| LT50 | Rule-filtered service request, category and stricter level controls, preferred regional variants and first-language hints (fields) | 0122 CHOICE and RULES, 0130 MCP. Verify each allowed typed input and keep service-specific shape distinct Explicit trace REQ-0122-CHOICE and UT-0122-CHOICE, REQ-0130-MCP and IT-0130-MCP |
| LT51 | Windows Notepad and named macOS applications and shortcuts (Windows editor, macOS, Outlook) | 0130 S03 and ROUTE. Keep local-file alternatives and every direct app access route distinct Explicit trace REQ-0130-ROUTE and IT-0130-ROUTE |

[lt-plans]: https://languagetool.org/premium
[lt-language]: https://help.languagetool.org/hc/en-us/articles/39254526141463-What-languages-does-LanguageTool-support
[lt-variants]: https://languagetool.org/insights/post/product-languages-supported/
[lt-editor]: https://help.languagetool.org/hc/en-us/articles/39254517614999-What-is-the-LanguageTool-Editor
[lt-extension]: https://help.languagetool.org/hc/en-us/articles/39254502438551-What-features-are-available-in-the-new-premium-only-version-of-the-LanguageTool-browser-extension
[lt-team]: https://help.languagetool.org/hc/en-us/articles/39254472101655-How-does-the-Team-Plan-work
[lt-dictionary]: https://help.languagetool.org/hc/en-us/articles/39254502745239-How-to-import-an-existing-Dictionary-into-LanguageTool
[lt-hosts]: https://help.languagetool.org/hc/en-us/articles/39254499177623-How-to-use-and-install-LanguageTool
[lt-host-limits]: https://help.languagetool.org/hc/en-us/articles/39254502620183-Which-websites-and-applications-are-supported-by-LanguageTool
[lt-api]: https://help.languagetool.org/hc/en-us/articles/39254488835095-Does-LanguageTool-offer-an-API
[lt-fields]: https://languagetool.org/http-api/swagger-ui/
[lt-windows]: https://languagetool.org/windows
[lt-access]: https://help.languagetool.org/hc/en-us/articles/39254499343383-Where-can-I-access-the-LanguageTool-Writing-Assistant
[lt-outlook]: https://help.languagetool.org/hc/en-us/articles/39254511533591-Do-you-offer-an-add-on-for-Outlook
[lt-overleaf]: https://help.languagetool.org/hc/de/articles/39254502720663-Kann-ich-das-LanguageTool-Add-on-in-Overleaf-verwenden
[lt-mac]: https://languagetool.org/mac-desktop
[lt-rephrase]: https://languagetool.org/paraphrasing-tool
[lt-rephrase-locales]: https://languagetool.org/insights/post/product-rephrase-feature/

## Grammarly

The public reference set was observed on 20 November 2026. Use
[features][g-features], [plans][g-plans], [languages][g-languages],
[multilingual checks][g-checks], [keyboard controls][g-keyboard],
[Android writing][g-android], [iPhone dictation][g-audio],
[Android dictation][g-android-audio], [generation][g-compose],
[rewrites][g-rewrite], [voice][g-voice], [natural wording][g-natural],
[goals][g-goals], [purpose advice][g-purpose], [translation][g-translation],
[reading help][g-read], [reader responses][g-reactions],
[rubrics][g-rubric], [CVs][g-cv], [citations][g-citations],
[source finder][g-finder], [similarity][g-similarity],
[origin estimates][g-origin], [recorded origin][g-record],
[editor][g-docs], [classic editor][g-classic],
[format preservation][g-preserve],
[file download][g-download], [mobile files][g-mobile],
[sharing][g-share], [team settings][g-team], [roles][g-roles],
[groups][g-groups], [term library][g-terms], [sensitive data][g-dlp],
[managed controls][g-managed], [data labels][g-labels],
[key custody][g-keys], [linked devices][g-devices], [session expiry][g-expiry],
[personal progress][g-progress], [weekly feedback][g-weekly],
[team measures][g-measures], [individual usage][g-individual],
[team export][g-export], [dialect policy][g-dialect],
[team access][g-team-access], [shared dictionary][g-dictionary],
[assistant builder][g-builder],
[assistant sharing][g-ashare], [connected assistants][g-connectors],
[browser controls][g-browser], [desktop workflows][g-desktop],
[phrasing review][g-phrase], [currently offered services][g-about] and
[expert service notice][g-expert].

The local proof names a concrete action, not a reference brand.
The source matrix retains variant, plan, surface and exact source
conditions. For outside effects, the final column identifies the
admissible local outcome or the need for a separate decision
under DR-114. Attach exact requirement and test evidence for
every expanded variant in the signed scope record.

| ID | Documented outcome and reference | Owning design and verification |
|---|---|---|
| GR01 | Correct spelling, grammar and punctuation separately (features, languages, checks) | 0122 RULES and UNION. Test each check family and locale Local trace REQ-0122-RULES / UT-0122-RULES |
| GR02 | Recognise contextually confused words and word choice (features, browser) | 0122 RULES and 0123 R17. Separate likely misuse and possible ambiguity Local trace REQ-0122-RULES / UT-0122-RULES |
| GR03 | Suggest clarity, concision and fluency improvements (features, checks) | 0122 RULES and 0123 R09. Keep the completed sentence and locale conditions Local trace REQ-0122-RULES / UT-0122-RULES |
| GR04 | Detect and adjust tone at different scopes (features, voice, rewrites) | 0123 R28 and REQ-0123-CATALOGUE / IT-0123-CATALOGUE for detection. REQ-0124-MODES / IT-0124-MODES for exact wording adjustment |
| GR05 | Inclusive language and passive voice (features, plans) | 0122 RULES and 0123 R25. Prove opt-in categories and applicable exceptions Local trace REQ-0122-RULES / UT-0122-RULES |
| GR06 | Select English dialect and detect writing language (languages, checks) | REQ-0122-CHOICE / UT-0122-CHOICE and REQ-0122-RANGE / BDD-0122-RANGE. Preserve the five dialect choices and mixed input |
| GR07 | More than one language in a document and advanced checking per surface (languages, checks) | 0122 UNION and BOUNDS. Preserve exact separate surface and mode rows including advanced Japanese Local trace REQ-0122-RULES / UT-0122-RULES |
| GR08 | Get short definitions and synonyms from a selected word (browser) | 0122 word exploration. Local selected input and exact range advice; native browser effect is separate Local trace REQ-0122-RULES / UT-0122-RULES |
| GR09 | Review, accept, dismiss, undo or flag a suggestion (features, classic editor, browser) | REQ-0122-APPLY / IT-0122-APPLY and REQ-0124-REVIEW / BDD-0124-REVIEW for review, undo and apply. Suggestion feedback flag and any delivery need exact QR-124 evidence |
| GR10 | Resume after disabling or ignoring a category (browser, team) | 0122 RULES and 0128 RULES. Keep one dismissal apart from policy-level disablement Local trace REQ-0122-RULES / UT-0122-RULES |
| GR11 | Document goals for audience, purpose, formality and domain (goals) | 0123 R28 and 0124 MODES. Test distinct declared goal fields Local trace REQ-0123-CATALOGUE / IT-0123-CATALOGUE |
| GR12 | Sentence rewrites and tone or length changes (features, rewrites) | 0124 MODES and INVARIANTS. Preserve supported facts Local trace REQ-0124-MODES / IT-0124-MODES |
| GR13 | Paragraph rewrite with standard action or custom request and grouped acceptance (rewrites) | 0124 MODES and 0122 APPLY. Test all seven documented language families Local trace REQ-0124-MODES / IT-0124-MODES |
| GR14 | Whole-document paragraph proposals and natural wording in distinct styles (natural, features) | REQ-0124-MODES / IT-0124-MODES with REQ-0124-REVIEW / BDD-0124-REVIEW for paragraph proposals and each style. Keep original meaning and separate voice profile source |
| GR15 | Personal voice learned from approved examples with document and message profiles (voice) | REQ-0124-CONSENT / E2E-0124-CONSENT. Test selected manual examples and opted-in ongoing updates independently. The Humanizer guide asks for at least 200 words for a custom profile. Track same-profile use in the Paraphraser and cross-host sync separately |
| GR16 | Proactive audience and purpose advice for supported surfaces (purpose) | 0124 MODES. Preserve documented US-English condition and separate live host permission Local trace REQ-0124-MODES / IT-0124-MODES |
| GR17 | Compose, ideate, outline, revise and make a reply from supplied context (generation) | 0124 MODES and CONSENT. Each proposal effect needs its own test. No reply sends Local trace REQ-0124-MODES / IT-0124-MODES |
| GR18 | Chat follow-ups, prompt history and regenerate (generation, editor) | REQ-0124-CONSENT / E2E-0124-CONSENT for bounded opt-in history and REQ-0124-REVIEW / BDD-0124-REVIEW for re-run. Attach separate chat-follow-up evidence in QR-124 |
| GR19 | Translate selected written text to or from supported variants (translation) | 0124 LANG. Keep exact source/target matrix and locale validation Local trace REQ-0124-LANG / IT-0124-LANG |
| GR20 | Ask, explain and translate selected received or reading text (reading help) | 0124 reading-assistance mode and 0130 S09. Do not conflate this with selection in an authored draft Local trace REQ-0130-ROUTE / IT-0130-ROUTE |
| GR21 | Simulate a message reader and revise against their likely concerns (reader responses) | 0125 C06 and SIM. Compare two audience choices without sending Local trace REQ-0125-SIM / BDD-0125-SIM |
| GR22 | Coach against a supplied or built-in grading rubric (rubrics) | 0125 C07 and SIM. Evidence per criterion and an optional estimated grade Local trace REQ-0125-SIM / BDD-0125-SIM |
| GR23 | Improve a supplied CV, layout alternatives and protected export (CVs) | 0125 U03 and REQ-0125-COVER / IT-0125-COVER for draft and layout. REQ-0125-FENCE / IT-0125-FENCE for export. Prove all three separately |
| GR24 | Find supporting or conflicting reference passages (source finder) | 0127 E01/E02. A permitted collection is the local scope. Web reach needs separate decisions Local trace REQ-0127-LOCAL / E2E-0127-LOCAL |
| GR25 | Generate a full and in-text citation using supplied metadata (citations) | REQ-0127-CITE / UT-0127-CITE. Use supplied metadata in the local route. Test complete and in-text or note output with each style |
| GR26 | Flag citation format and consistency across the document (citations) | REQ-0127-CITE / BDD-0127-CITE. Separate classic paid Academic-domain formatting with selected style from current Docs chat-assisted checks and the format tag or citation review effect |
| GR27 | Generate citations from the current page on a compatible journal site (citations) | REQ-0127-CITE / UT-0127-CITE proves local supplied metadata only. GR-C01 to GR-C20 record the distinct browser cases. Each needs live host permission and exact source evidence |
| GR28 | Review similarity with documented scope and citation suggestions (similarity) | 0127 E05 and MATCH. Track each corpus reach and verified absence separately Local trace REQ-0127-MATCH / IT-0127-MATCH |
| GR29 | Estimate AI-origin signals and disclose known use separately (origin estimates) | 0127 E06 and ESTIMATE. No inferred identity or automatically added citation Local trace REQ-0127-ESTIMATE / E2E-0127-ESTIMATE |
| GR30 | Start or stop per-document origin recording and source categories (recorded origin) | REQ-0127-RECORD / BDD-0127-RECORD. Test per-session, per-document choice and suggested opt-in for future documents. Keep automatic activation in the reference Docs surface separate |
| GR31 | Distinguish direct, pasted, generated and ordinarily corrected input (recorded origin) | 0127 E07/E08 and REQ-0127-RECORD / BDD-0127-RECORD. Keep direct, pasted, conventional correction, generated insertion and AI rewrite distinct. Verify declared, observed and unknown source cases separately |
| GR32 | Show origin breakdown, eligible time and activity counts (recorded origin) | 0127 E08 and 0126 H11. REQ-0127-RECORD / BDD-0127-RECORD covers recorded categories. Timing and count variants need exact 0126 event and proof without surveillance |
| GR33 | Replay history and optionally include replay in a shared report (recorded origin) | 0127 E09/E10 and REQ-0127-SHARE / IT-0127-SHARE. Record pause, local report, erased content and optional replay independently. Synchronised cross-device reporting needs its own route |
| GR34 | Receive school submission via hosted link or institutional aggregates (recorded origin) | 0127 E10/E11 and REQ-0127-SHARE / IT-0127-SHARE for exact local copies. Hosted school submission, group-only counts by origin category and commonly used source information are distinct protected outcomes. No teacher access from local consent |
| GR35 | Create, search, organise, label and edit a writing file (editor) | 0129 F01 is the library contract. REQ-0129-ERASE / E2E-0129-ERASE tests recovery and erase. Create, rename, search, label and ordinary lifecycle require their own fixture instances in QR-124 |
| GR36 | Format text, tables, outline and keyboard commands (editor) | 0129 F02 and REQ-0129-FIDELITY / BDD-0129-FIDELITY for structured editing. Each keyboard interaction has its own effect |
| GR37 | Classic document import and same-format download with formatting (format preservation, file download) | 0129 F04/F05. REQ-0129-FIDELITY / UT-0129-FIDELITY for exact preserved format and TXT-to-DOCX conversion. Include import refusal cases |
| GR38 | Current Docs print, PDF and Word export with known losses (editor) | 0129 F06 and REQ-0129-FIDELITY / UT-0129-FIDELITY for exact output features. QR-039 holds PDF and printer route proof |
| GR39 | Mobile document input and output profiles (mobile files) | 0129 format table and 0130 S06. Track direct iOS effects separately Local trace REQ-0129-MATRIX / IT-0129-MATRIX |
| GR40 | View, comment and edit roles (sharing) | 0129 F07/F08. Prove read, inert suggested wording and signed proposal separately Local trace REQ-0129-SHARE / IT-0129-SHARE |
| GR41 | Email invite, shared URL and transfer ownership of a hosted document (sharing) | 0129 F09. Live role assignment, link revocation, real-time editing and ownership demotion need independent decisions Local trace REQ-0129-SHARE / IT-0129-SHARE |
| GR42 | Personal dictionary, word dismissals and dialect preferences (features, team settings, browser) | 0128 T01 and REQ-0122-RULES / UT-0122-RULES for language-scoped use. Track individual word constraints and 100-symbol source limit separately from team multiword input |
| GR43 | Shared dictionary and organisation-wide dialect (team settings, roles) | 0128 T02/T04 and REQ-0128-RULES / UT-0128-RULES. Support team CSV and multiword cases with admitted punctuation. Verify global and group dialect, and personal fallback when no preference applies |
| GR44 | Create style rules and properties, scope and exceptions (team settings, roles) | 0128 T03. Distinct create/edit/delete permissions and exact rule use Local trace REQ-0128-RULES / UT-0128-RULES |
| GR45 | Shared brand-tone profiles and category preferences (team settings) | 0128 T04/T05. Keep named contextual profiles and member preference scope Local trace REQ-0128-RULES / UT-0128-RULES |
| GR46 | Create, insert, manage and suggest reusable snippets (team settings) | 0128 T06. Insertion and proposed snippet creation are separate approved effects Local trace REQ-0128-RULES / UT-0128-RULES |
| GR47 | Definitions, aliases, contacts and document references (term library) | 0128 T07 and 0127 E12. REQ-0128-RULES / BDD-0128-RULES for terms. Separate admin edit and optional personal suggested-new-term action |
| GR48 | Import, assign or export members and groups (groups, roles) | 0128 T12. Distinguish approved local role policy from hosted invitations and provisioning Local trace REQ-0128-APPROVAL / IT-0128-APPROVAL |
| GR49 | Group managers and delegated roles (groups, roles) | 0128 T12. Exact allowed and refused scope, including custom authority Local trace REQ-0128-APPROVAL / IT-0128-APPROVAL |
| GR50 | Feature, generation and agent controls (team settings) | 0128 T08 and REQ-0128-APPROVAL / IT-0128-APPROVAL plus REQ-0130-RUN / IT-0130-RUN for active disable. Separate host background checks, agent controls and preservation of existing words when adding is disabled |
| GR51 | Restrict use on a domain, in an app or by client (managed controls) | 0128 T11. Bind verified native route and scope. Keep outside deployments distinct Local trace REQ-0128-CONTEXT / IT-0128-CONTEXT |
| GR52 | Confidential source restrictions and trusted labels (data labels) | 0128 T09. Imported labels can add restrictions. Live Word and Gmail labels have separate source rows Local trace REQ-0128-CONTEXT / IT-0128-CONTEXT |
| GR53 | Sensitive data masking before protected processing (sensitive data) | 0128 T10, SENSITIVE. Prove all documented data classes and refusal on incomplete check Local trace REQ-0128-SENSITIVE / E2E-0128-SENSITIVE |
| GR54 | Managed, external sign-in, domain capture and provisioning (plans, managed controls, groups) | 0128 T17. Keep managed mode, SAML, SCIM and domain join separate. A local setting cannot prove them Local trace REQ-0128-CONTEXT / IT-0128-CONTEXT |
| GR55 | Session expiry, re-authentication and linked device revocation (linked devices, session expiry) | 0128 T13. Prove protected local and native effects independently Local trace REQ-0128-CONTEXT / IT-0128-CONTEXT |
| GR56 | Organisation keys and independent customer key custody (key custody) | 0128 T13/T17 and 0118. Scope encrypted asset classes. Hosted KMS, provider-managed and owner-managed routes are distinct Local trace REQ-0128-PLATFORM / E2E-0128-PLATFORM |
| GR57 | Audit logs and API, regulated account support (plans) | 0128 T16/T17. Protect local audit. Remote audit, dedicated support and access controls need separate routes Local trace REQ-0128-PLATFORM / E2E-0128-PLATFORM |
| GR58 | Personal progress, learning suggestions and writing reports (personal progress, weekly feedback) | 0123 R01/R03/R04/R28, 0126 H04, 0128 T14. Keep private trends and weekly email delivery distinct Local trace REQ-0128-INSIGHTS / IT-0128-INSIGHTS |
| GR59 | Compare performance with organisation and other users (personal progress, weekly feedback, team measures) | 0128 T14 and QR-121. Compare only with an admitted matched source and versioned metric Local trace REQ-0128-INSIGHTS / IT-0128-INSIGHTS |
| GR60 | Team quality dimensions, suggestions seen, accepted and unresolved (team measures, team export) | 0128 T14. Preserve correctness, clarity, inclusivity, style and brand tone as separate measures Local trace REQ-0128-INSIGHTS / IT-0128-INSIGHTS |
| GR61 | Activation, account use, group and app views (team export, individual usage) | 0128 T15. Distinct eligibility and denominator. Ambient or individual capture needs separate approval Local trace REQ-0128-INSIGHTS / IT-0128-INSIGHTS |
| GR62 | Group and individual writing sessions and improved sessions (individual usage, team export) | 0128 T14/T15 and 0126 activity. A share of improved eligible sessions requires admitted before-and-after evidence. Individual organisation export retains its own gap Local trace REQ-0128-INSIGHTS / IT-0128-INSIGHTS |
| GR63 | Usage and team reports for controlled export (team export) | 0128 T14–T16. Keep export category, compatible format, roles and repeat-disclosure floor distinct Local trace REQ-0128-INSIGHTS / IT-0128-INSIGHTS |
| GR64 | Report estimated return and cost-centre use (plans) | 0128 T15. Admitted assumptions and evidence are separate from measured use Local trace REQ-0128-INSIGHTS / IT-0128-INSIGHTS |
| GR65 | Browser and site-specific controls, writing suggestions and switch between native surfaces (browser) | 0130 S03/S04 and REQ-0130-ROUTE / IT-0130-ROUTE. Separate Chrome, Firefox, Edge and Safari, plus narrowly scoped host controls. See GR82 to GR84 for native desktop |
| GR66 | Android widget, automatic correction and drafts (Android writing) | 0130 S06/S07. Keep native suggestion, automatic correction, manual draft and mobile file flow distinct Local trace REQ-0130-ROUTE / IT-0130-ROUTE |
| GR67 | iPhone and iPad keyboard suggestions and supported locale modes (keyboard controls) | 0130 S06/S07 and 0122 UNION. Recognise narrower keyboard variants; local advice does not automatically replace Local trace REQ-0130-ROUTE / IT-0130-ROUTE |
| GR68 | Smart arithmetic, units, ratio, phone, date and time display (keyboard controls) | 0130 FORMAT. Test separate settings and typed proposals with ambiguity handling Local trace REQ-0130-FORMAT / UT-0130-FORMAT |
| GR69 | Word deletion gesture, selected completion and native shortcuts (keyboard controls, browser) | 0130 S06/S07. QR-124 needs exact keyboard gesture and shortcut proof. A typed formatting test cannot establish delete, completion or native auto-replace |
| GR70 | iPhone dictation and cleanup (iPhone dictation) | 0130 S08 and INPUT. Distinct platform permission, final transcript and review Local trace REQ-0130-INPUT / E2E-0130-INPUT |
| GR71 | Android dictation and cleanup (Android dictation) | 0130 S08 and INPUT. Track native widget and no presumed offline provider Local trace REQ-0130-INPUT / E2E-0130-INPUT |
| GR72 | Configure assistants by form or conversation (assistant builder) | 0130 S11/TEMPLATE. Equal authorised proposal types with local approval Local trace REQ-0130-TEMPLATE / BDD-0130-TEMPLATE |
| GR73 | Manual, timed and proactive writing routines (assistant builder) | 0130 S12/RUN. Test each trigger, exact input, scope and cancel/replay Local trace REQ-0130-RUN / IT-0130-RUN |
| GR74 | Activate after mail received, sent or labelled (assistant builder, connected assistants) | 0130 S05/S12. Record each trigger independently. A manually supplied exchange cannot prove a live effect Local trace REQ-0130-RUN / IT-0130-RUN |
| GR75 | Activate after chat mention or watched message (assistant builder, connected assistants) | 0130 S12. Test separate local equivalents and keep watches and cross-service sending as gaps Local trace REQ-0130-RUN / IT-0130-RUN |
| GR76 | Connected research, calendar, documents and mail actions (connected assistants) | 0130 S05/S09/S14 and REQ-0130-ROUTE / IT-0130-ROUTE confirm the local scope only. GR85 to GR91 name distinct outside effects |
| GR77 | Share live use, edit or management under the creator's connected account (assistant sharing) | 0130 S13. Each inherited-account effect needs its own decision. Read-only local templates grant only current local permissions Local trace REQ-0130-TEMPLATE / BDD-0130-TEMPLATE |
| GR78 | Organisation directory subscription or independent assistant copy (assistant sharing, assistant builder) | 0130 S13. Keep subscribed updates and local template copies independently scoped Local trace REQ-0130-TEMPLATE / BDD-0130-TEMPLATE |
| GR79 | Support routes and historical expert review offering (currently offered services, plans) | Current local support is 0130 S15. Hosted support and priority routes are independent. The expert service notice dates discontinuation to 31 January 2025. The historical feature page listing stays recorded under QR-124 |
| GR80 | Rewriter reviews frequently generated expressions and offers wording alternatives (phrasing review) | 0124 MODES is the local wording anchor. Exact frequency, uncertainty and review evidence need QR-124. An expression alone cannot prove origin or override 0127 |
| GR81 | Refresh private voice or keep a profile available across different writing tools (voice, natural wording) | REQ-0124-CONSENT / E2E-0124-CONSENT verifies local grants and erasure. Reuse within a compatible local scope needs evidence. Live outside profile sync is a separate effect |
| GR82 | Windows writing widget, application-specific position and documented window context (desktop workflows) | 0130 S03. REQ-0130-ROUTE / IT-0130-ROUTE for explicit selections only. Direct checking and contextual window use retain separate decisions |
| GR83 | macOS writing widget, sidebar, app block and positioning (desktop workflows) | 0130 S03. Local selection can be reviewed. macOS deployment, capture and native editing are separate gaps |
| GR84 | macOS similarity and origin checks in Apple Pages and Word (desktop workflows) | 0127 E05/E06. REQ-0127-LOCAL / E2E-0127-LOCAL for supplied text. Preserve each host and checked effect |
| GR85 | Drive file and metadata search, access information and recent changes (connected assistants) | 0130 S09 and 0127 E12 for approved supplied files. Live Drive search, permissions lookup and change feeds require separate decisions |
| GR86 | Gmail and Outlook search and summaries of supplied mail, attachments and calendar context (connected assistants) | 0130 S05 and 0124 draft assistance. REQ-0124-CONSENT / E2E-0124-CONSENT. Track connected search and inbox reading separately |
| GR87 | Draft and send a message, pick send time, undo recent send, label and move inbox items (connected assistants) | 0130 S05 only for explicit local draft. Each native send, schedule, undo and triage effect needs its own decision |
| GR88 | Find an available time and create or update a meeting (connected assistants) | 0130 S05 for a local proposed time with supplied evidence. Live availability and each calendar write remain separate |
| GR89 | Jira find, summarise, see issues and blockers and report workload (connected assistants) | 0130 S09 or approved file reporting. Each live access and reporting variant needs evidence within DR-076 scope |
| GR90 | Jira comment, reassign, attach, move or bulk-update work (connected assistants) | 0130 S14 and REQ-0130-ROUTE / IT-0130-ROUTE retain the exact outside effects. Local advice cannot perform those actions |
| GR91 | Active window and deliberate additional window context (connected assistants, desktop workflows) | 0130 S03/S09 for explicit selected data only. Each outside capture mode needs separate permission and an admitted host |

### Grammarly automatic source citation surfaces

The browser guide for citations names the host and path scopes
below. For each, preserve the source-page condition, user-enabled
citation button and style switch. Verify local supplied metadata
under REQ-0127-CITE and UT-0127-CITE. That check grants no browser
host access. Record each in-page effect under DR-114.

| ID | Host or documented path |
|---|---|
| GR-C01 | journals.plos.org/plosone |
| GR-C02 | sciencedirect.com/science/article/pii/ |
| GR-C03 | journals.sagepub.com/doi |
| GR-C04 | ncbi.nlm.nih.gov/pmc/articles |
| GR-C05 | doaj.org/article |
| GR-C06 | arxiv.org/abs |
| GR-C07 | SpringerOpen subdomains with an articles path |
| GR-C08 | frontiersin.org/articles/ |
| GR-C09 | onlinelibrary.wiley.com |
| GR-C10 | journals.physiology.org |
| GR-C11 | science.org |
| GR-C12 | nature.com |
| GR-C13 | pnas.org |
| GR-C14 | annualreviews.org |
| GR-C15 | pubs.asha.org |
| GR-C16 | academic.oup.com |
| GR-C17 | mdpi.com |
| GR-C18 | muse.jhu.edu |
| GR-C19 | online.ucpress.edu/journals |
| GR-C20 | journals.uchicago.edu |

[g-features]: https://www.grammarly.com/features
[g-plans]: https://www.grammarly.com/plans
[g-languages]: https://www.grammarly.com/languages
[g-checks]: https://support.grammarly.com/hc/en-us/articles/39345737251469-Introducing-Multilingual-Suggestions
[g-keyboard]: https://support.grammarly.com/hc/en-us/articles/360041391992-Manage-your-keyboard-settings-in-Grammarly-for-iPhone
[g-android]: https://support.grammarly.com/hc/en-us/articles/15606282682637-Grammarly-for-Android-user-guide
[g-audio]: https://support.grammarly.com/hc/en-us/articles/46245126716941-Introducing-speech-to-text
[g-android-audio]: https://support.grammarly.com/hc/en-us/articles/50343460281101-Introducing-speech-to-text-on-Android
[g-compose]: https://support.grammarly.com/hc/en-us/articles/14528857014285-Introducing-generative-AI-assistance
[g-rewrite]: https://support.grammarly.com/hc/en-us/articles/30916398193037-Introducing-paragraph-level-rewrites
[g-voice]: https://support.grammarly.com/hc/en-us/articles/23153676821773-View-and-customize-your-voice-profile
[g-natural]: https://support.grammarly.com/hc/en-us/articles/38552339652109-Humanizer-user-guide
[g-goals]: https://support.grammarly.com/hc/en-us/articles/360054679292-What-are-Goals
[g-purpose]: https://support.grammarly.com/hc/en-us/articles/25351804320397-Introducing-strategic-suggestions
[g-translation]: https://support.grammarly.com/hc/en-us/articles/27518589299725-Introducing-Grammarly-s-translation-feature
[g-read]: https://support.grammarly.com/hc/en-us/articles/45119057068173-Reading-assistance-for-Superhuman-Go-Ask-Go-Translate-Explain
[g-reactions]: https://support.grammarly.com/hc/en-us/articles/38552519578893-Reader-Reactions-user-guide
[g-rubric]: https://support.grammarly.com/hc/en-us/articles/38552360525837-AI-Grader-user-guide
[g-cv]: https://support.grammarly.com/hc/en-us/articles/45441636399117-Resume-Builder-user-guide
[g-citations]: https://support.grammarly.com/hc/en-us/articles/9182359307917-How-to-generate-citations-and-a-bibliography
[g-finder]: https://support.grammarly.com/hc/en-us/articles/38552451211533-Citation-Finder-user-guide
[g-similarity]: https://support.grammarly.com/hc/en-us/articles/115000091452-Plagiarism-Checker-user-guide
[g-origin]: https://support.grammarly.com/hc/en-us/articles/28936304999949-AI-Detector-user-guide
[g-record]: https://support.grammarly.com/hc/en-us/articles/29548735595405-Introducing-Authorship
[g-docs]: https://support.grammarly.com/hc/en-us/articles/38552281546765-Docs-Grammarly-s-new-AI-writing-surface
[g-preserve]: https://support.grammarly.com/hc/en-us/articles/115000091512-Preserve-text-formatting
[g-download]: https://support.grammarly.com/hc/en-us/articles/115000091352-Create-upload-and-download-your-documents
[g-mobile]: https://support.grammarly.com/hc/en-us/articles/4412638077709-How-to-create-or-edit-documents-on-iPhones
[g-share]: https://support.grammarly.com/hc/en-us/articles/40852724499853-Introducing-document-sharing-in-docs
[g-team]: https://support.grammarly.com/hc/en-us/sections/115000249011-Team-Features
[g-roles]: https://support.grammarly.com/hc/en-us/articles/27540336731149-About-custom-roles
[g-groups]: https://support.grammarly.com/hc/en-us/articles/4415031489037-Organize-team-members-into-groups
[g-terms]: https://support.grammarly.com/hc/en-us/articles/16664924710797-Add-terms-to-Knowledge-Share
[g-dlp]: https://support.grammarly.com/hc/en-us/articles/27458670736525-Enable-Data-Loss-Prevention-DLP
[g-managed]: https://support.grammarly.com/hc/en-us/sections/27479182375693-Configure-Access-Security
[g-labels]: https://support.grammarly.com/hc/en-us/articles/27455819234317-Enable-confidential-mode
[g-keys]: https://support.grammarly.com/hc/en-us/articles/27895143707405-Bring-Your-Own-Key-FAQs
[g-devices]: https://support.grammarly.com/hc/en-us/articles/4418264113165-How-do-I-check-what-devices-are-linked-to-my-Grammarly-account
[g-expiry]: https://support.grammarly.com/hc/en-us/articles/26230680413325-Session-Timeout-FAQs
[g-progress]: https://support.grammarly.com/hc/en-us/articles/21940617172877-Track-your-writing-progress
[g-weekly]: https://support.grammarly.com/hc/en-us/articles/115000090892-Common-questions-about-weekly-Grammarly-Insights-reports
[g-measures]: https://support.grammarly.com/hc/en-us/articles/32478363601549-Review-your-organization-s-Effective-Communication-Score
[g-individual]: https://support.grammarly.com/hc/en-us/articles/11226802974477-Review-member-usage-analytics
[g-export]: https://support.grammarly.com/hc/en-us/articles/4403095501453-Organize-exported-team-overview-analytics-data
[g-builder]: https://support.grammarly.com/hc/en-us/articles/47453002811021-Create-and-configure-agents-with-the-agent-builder
[g-ashare]: https://support.grammarly.com/hc/en-us/articles/48322812077325-Share-and-publish-your-agent
[g-connectors]: https://support.grammarly.com/hc/en-us/articles/40642362241293-About-Superhuman-Go-agents-and-connectors
[g-browser]: https://support.grammarly.com/hc/en-us/articles/115000091592-Grammarly-s-browser-extension-user-guide
[g-about]: https://support.grammarly.com/hc/en-us/sections/115000022371-About-Grammarly
[g-classic]: https://support.grammarly.com/hc/en-us/articles/360003474732-Grammarly-Editor-user-guide
[g-dialect]: https://support.grammarly.com/hc/en-us/articles/15858322185357-Set-an-organization-wide-dialect-preference
[g-team-access]: https://support.grammarly.com/hc/en-us/articles/32752750261517-Manage-your-team-s-feature-access
[g-dictionary]: https://support.grammarly.com/hc/en-us/articles/14441067751309-Create-organizational-dictionary
[g-desktop]: https://support.grammarly.com/hc/en-us/articles/4412816078349-Grammarly-for-Windows-and-Grammarly-for-Mac-user-guide
[g-phrase]: https://support.grammarly.com/hc/en-us/articles/42141090078861-AI-Rewriter-user-guide
[g-expert]: https://support.grammarly.com/hc/en-us/articles/34083794138381-Grammarly-s-expert-writing-service-has-been-discontinued

## ProWritingAid

The public reference set was observed on 20 November 2026. Use the
[product features][p-features], [plans][p-plans], [languages][p-lang],
[reports][p-reports], [report help][p-help], [report details][p-details],
[setup and preferences][p-settings], [writing styles][p-styles],
[summary help][p-summary], [readability guide][p-readable],
[style guide][p-line], [word exploration][p-words],
[rewrites][p-rewrite], [revision presets][p-presets],
[rewrite controls][p-controls], [inspiration help][p-inspire],
[creative options][p-creative],
[planning][p-plan], [prompts][p-prompts], [timers][p-timer],
[sessions][p-session],
[markers][p-mark], [reflections][p-reflect], [breaks][p-break],
[streaks][p-streak], [critique modes][p-critique], [word ranges][p-limits],
[plot][p-plot],
[character][p-character], [manuscript][p-ms], [reader][p-reader],
[publishing][p-market], [similarity][p-sim], [report history][p-history],
[used checks][p-used], [focused critique][p-focused],
[custom rules][p-rules], [creative names][p-names],
[file formats][p-files], [host compatibility][p-hosts],
[project save-back][p-scrivener], [community][p-community],
[challenge][p-challenge], [preparation][p-prep], [contest][p-contest]
and [contest terms][p-terms].

The local proof names distinct test cases. Source size and plan
conditions describe the source. A working local mode supplies no
evidence of an outside purchase, host action or community result.
Take unsupported outside effects to the owner under DR-114.

| ID | Documented outcome and reference | Owning design and verification |
|---|---|---|
| PW01 | Live grammar, spelling, punctuation and consistency feedback (features, reports, languages) | REQ-0122-RULES / UT-0122-RULES. Test General, British, US, Australian and Canadian English separately |
| PW02 | Style and genre or document type selection (writing styles, plans) | 0123 R01/R28 and REQ-0123-CATALOGUE / IT-0123-CATALOGUE. Separate General, Academic, Business, Technical, Creative, Casual, Web and Script, generic fallback, named document types and rule applicability |
| PW03 | Configure specific check categories and rule preferences (setup and preferences, report details) | REQ-0128-RULES / UT-0128-RULES. Separate ignore once, scoped disable, deliberate removal and rule exemptions |
| PW04 | Set quote and dialogue exemptions (setup and preferences) | REQ-0128-RULES / BDD-0128-RULES. Test dialogue exclusions without changing narrative evidence |
| PW05 | Personal vocabulary and company/product name preferences (setup and preferences) | 0128 T01/T03. REQ-0128-RULES / UT-0128-RULES. Keep name consistency separate from word exceptions |
| PW06 | Report overview and exact evidence view (summary help) | 0123 R01. REQ-0123-CATALOGUE / IT-0123-CATALOGUE. Test genres, goal bands, report strengths and scope |
| PW07 | Custom report set (reports, report details) | 0123 R02. REQ-0123-BATCH / FI-0123-BATCH. Keep a shared revision and distinguish retry from a new seal |
| PW08 | Text measures and vocabulary comparisons (summary help, setup and preferences) | 0123 R03/R04. REQ-0123-MEASURES / UT-0123-MEASURES. Check scope and exact denominator |
| PW09 | Readability measures and grade labels (readability guide) | 0123 R05 and REQ-0123-MEASURES / UT-0123-MEASURES. Distinguish Flesch Reading Ease from the three US grade-style indices. Use independent method vectors |
| PW10 | Estimated reading time and paragraph difficulty ranges (readability guide) | 0123 R05. REQ-0123-CATALOGUE / IT-0123-CATALOGUE. Test three bins, exact jump and count and estimated time separately |
| PW11 | Grammar report for the selected text (reports, report help) | 0123 R06. REQ-0123-CATALOGUE / IT-0123-CATALOGUE. Reuse exact 0122 output ranges |
| PW12 | Sentence-length counts and distributions (report details) | 0123 R07. REQ-0123-MEASURES / UT-0123-MEASURES. Check band edges and sentence boundaries |
| PW13 | Sentence structure and variety (report help) | 0123 R08. REQ-0123-CATALOGUE / IT-0123-CATALOGUE. Test different starts and syntax independently |
| PW14 | Clarity, fluency, concision and actionable wording (reports, style guide) | 0123 R09. REQ-0123-CATALOGUE / IT-0123-CATALOGUE. Show supported and uncertain advice separately |
| PW15 | Function-word density by sentence and document (report help) | 0123 R10. REQ-0123-MEASURES / UT-0123-MEASURES. Keep genre boundaries and whole-document context |
| PW16 | Overused words and phrases including custom terms (report help, setup and preferences) | 0123 R11. REQ-0123-CATALOGUE / IT-0123-CATALOGUE. Scope to exact term policy |
| PW17 | Repeats across the document (reports, report details) | 0123 R12. REQ-0123-BATCH / FI-0123-BATCH. Deduplicate exact occurrences |
| PW18 | Nearby repeats with adjustable limits (reports, setup and preferences) | 0123 R13. REQ-0123-CATALOGUE / IT-0123-CATALOGUE. Test distance, length and minimum occurrences separately |
| PW19 | Shared nearby initial sounds (reports, report details) | 0123 R14. REQ-0123-CATALOGUE / IT-0123-CATALOGUE. Use actual pronunciation evidence |
| PW20 | Familiar expressions and redundancies (reports, setup and preferences) | 0123 R15. REQ-0123-CATALOGUE / IT-0123-CATALOGUE. Keep intentional and exempt spans distinct |
| PW21 | Diction and vague wording (reports, report details) | 0123 R16. REQ-0123-CATALOGUE / IT-0123-CATALOGUE. Validate technical words in context |
| PW22 | Shared spelling or sound and word confusion lists (reports, report details) | 0123 R17. REQ-0123-CATALOGUE / IT-0123-CATALOGUE. Show alternatives without forcing corrections |
| PW23 | Pronouns and balance (reports, report details) | 0123 R18. REQ-0123-MEASURES / UT-0123-MEASURES. State counts and evidence conditions |
| PW24 | Abbreviation consistency and explanations (reports, report details) | 0123 R19. REQ-0123-CATALOGUE / IT-0123-CATALOGUE. Test explicit exceptions and later variations |
| PW25 | Style, spelling, punctuation and formatting consistency (reports, setup and preferences) | 0123 R20. REQ-0123-CATALOGUE / IT-0123-CATALOGUE. Compare one policy and mixed dialects |
| PW26 | Linking words and transitions (reports, report details) | 0123 R21. REQ-0123-CATALOGUE / IT-0123-CATALOGUE. Scope breaks and connectors |
| PW27 | Dialogue and speech tags (reports, report details) | 0123 R22. REQ-0123-CATALOGUE / IT-0123-CATALOGUE. Distinct speech, tags and non-dialogue quotes |
| PW28 | Scene pace checks (reports, report details) | 0123 R23. REQ-0123-CATALOGUE / IT-0123-CATALOGUE. Separate each evidenced pace factor |
| PW29 | Five senses and figurative evidence (reports, report details) | 0123 R24. REQ-0123-CATALOGUE / IT-0123-CATALOGUE. Keep each category and unknown observations |
| PW30 | Passive, hidden verbs, adverbs, subordinate clauses, emotion description and repeated starts (style guide) | 0123 R25. REQ-0123-CATALOGUE / IT-0123-CATALOGUE. Prove each category and applicable ratio separately |
| PW31 | House rules and writing guide report (reports, setup and preferences) | 0123 R26 and 0128 T03. REQ-0128-RULES / UT-0128-RULES. Test conflicting policy and displayed evidence |
| PW32 | Compare with an author and with genre (summary help, setup and preferences) | 0123 R27. REQ-0123-REFERENCE / IT-0123-REFERENCE. Independently licensed, matched comparators only |
| PW33 | View and navigate word classes and get alternatives in context (word exploration) | 0123 R29 and 0122 word exploration. REQ-0123-CATALOGUE / IT-0123-CATALOGUE. Test each role, occurrence and exact target. The word-ranges source lists a 2,000-word selection for its separate check |
| PW34 | Summary and critique export, print and same facts in different views (summary help, report help) | REQ-0123-VIEW / BDD-0123-VIEW and 0129 F06. Document PDF/print and exact scope approval |
| PW35 | Rephrase whole sentences in seven disclosed modes (revision presets) | REQ-0124-MODES / IT-0124-MODES. Standard, fluent, formal, informal, sensory, shorten and lengthen have separate cases |
| PW36 | Edit longer selections for readability, transitions, senses, summary, expansion, tense and perspective (creative options, rewrites) | REQ-0124-MODES / IT-0124-MODES. Test each allowed mode and both the documented minimum sentence input and 2,500-character limit from rewrite controls. Keep local admission independently measured |
| PW37 | Offer independent creative prompts for story action, dialogue, feelings, sensory cues, analogy, argument, meaning, emphasis, example, explanation, humour and quotations (creative options) | REQ-0124-MODES / IT-0124-MODES. Each output effect stays a proposal. The 2023 roundup includes an example mode. The separate 2025 inspiration help documents the mode and example interaction. Confirm each current option and surface separately. Attributed words require evidence |
| PW38 | Start again, copy a permissible example and accept wording (rewrites, creative options) | REQ-0124-REVIEW / BDD-0124-REVIEW. Compare exact effects and approved handoff |
| PW39 | Save a partial story plan and receive feedback (planning) | 0125 P01/P02. REQ-0125-PLAN / BDD-0125-PLAN. Retain missing fields |
| PW40 | Try alternative beat-by-beat outlines from plan or notes (planning) | 0125 P03. REQ-0125-PLAN / BDD-0125-PLAN. Check three distinct alternatives |
| PW41 | Give story prompts by next action, structural beat or broad possibility (prompts) | 0125 P04. REQ-0125-COVER / IT-0125-COVER. Test each prompt class and appropriate source word counts |
| PW42 | Timer for drafting or planning (timers, plans) | 0126 H02. REQ-0126-TIME / FI-0126-TIME. Test extend, hide, expire, pause and timely retry |
| PW43 | Record and amend sessions and words (timers, sessions) | 0126 H03/H04. REQ-0126-GOAL / UT-0126-GOAL. Keep diary corrections separate from observed time |
| PW44 | Writing focus mode (features, plans) | 0126 H01. REQ-0126-COVER / IT-0126-COVER. Preserve editor state |
| PW45 | Add searchable placeholders during writing (markers) | 0126 H05. REQ-0126-MARK / BDD-0126-MARK. Test deliberate insertion and exact anchor retention |
| PW46 | Private confidence, mood and reflective notes (reflections) | 0126 H06. REQ-0126-PRIVATE / E2E-0126-PRIVATE. No share or author substitutions |
| PW47 | Repeat a guided pause and return to the draft (breaks) | 0126 H07. REQ-0126-COVER / IT-0126-COVER. Own pacing and silent accessible controls |
| PW48 | Consecutive writing days, milestones and progress (sessions, challenge) | 0126 H04/H09. REQ-0126-GOAL / UT-0126-GOAL. Exact local eligible events |
| PW49 | Weekly product-use streak regardless of recorded writing (streaks) | 0126 timing rule. REQ-0126-COVER / IT-0126-COVER. No incidental capture. Separate equivalent or owner decision required |
| PW50 | Critique a single chapter with navigable evidence (critique modes) | 0125 C01 and REQ-0125-TRACE / UT-0125-TRACE. The mode comparison gives 6,000 as maximum with 500 recommended. The separate word-ranges guide says 4,000 maximum. Preserve both dated claims. Spec 0125 chooses its own local range. Test overview, strengths, beat outline, tension, protagonist arc, viewpoint, structure, pace, style, characters and dialogue |
| PW51 | Analyse outline or story beats, tension and arc (plot) | 0125 C02 and REQ-0125-TRACE / UT-0125-TRACE. Record each of nine beats, high and low points, structure, arc consequence and tension display. Use documented outline or full input mode |
| PW52 | Character cast, portrait, relationships, wants, agency and arc (character) | 0125 C03 and REQ-0125-COVER / IT-0125-COVER. Distinguish cast, role and function, counted appearance, relationship stages, voice, wants, needs and agency. Separate portrait permission, generated asset and alt text |
| PW53 | Full-manuscript review, chapter-specific issues and priorities (manuscript) | 0125 C04 and REQ-0125-TRACE / UT-0125-TRACE. Record full-document reference input up to 300,000 words and recommended minimum 6,000. Test themes, plot, character, setting, continuity, priorities and coverage loss |
| PW54 | Simulated reader impressions, engagement and marketing ideas (reader) | 0125 C05 and REQ-0125-SIM / BDD-0125-SIM. Reference prefers a complete edited 6,000–300,000-word manuscript. Test impressions, engagement clues, possible reader reactions, emotional trajectory and marketing ideas. Local proof remains a rehearsal |
| PW55 | Reader fit, hook, title and market comparisons (publishing) | 0125 U01. REQ-0125-COVER / IT-0125-COVER. Distinguish supplied market evidence from ideas |
| PW56 | Synopsis, query letter, cover brief and publishing plans (publishing) | 0125 U02 and REQ-0125-COVER / IT-0125-COVER for each artefact. REQ-0125-FENCE / IT-0125-FENCE for exact handoff. A fence alone proves no generated material |
| PW57 | Outreach plans, sequels and spin-offs (publishing) | 0125 U02. REQ-0125-COVER / IT-0125-COVER. Separate email, social and video draft effects from sending |
| PW58 | Review past feedback and change its review state (report help, past reports) | 0125 review record. Report history confirms save, find, rename, view, print and PDF across six critique modes. Use REQ-0125-FENCE / IT-0125-FENCE for access and erasure and QR-124 for independent history and print proof. Keep cross-host sync distinct |
| PW59 | Similarity checks against published sources (similarity) | 0127 E05 and REQ-0127-MATCH / IT-0127-MATCH. Reference has 2,000 words per check and admitted Web Editor, Windows Word and Google Docs surfaces. Mac Word, Desktop Everywhere and browser extension checks are excluded in the source. Web corpus reach and credit accounting have separate effects |
| PW60 | Safe file intake and conversion for each documented format (file formats) | 0129 format table. REQ-0129-MATRIX / IT-0129-MATRIX. Separate host-specific directions and optional loss |
| PW61 | Scrivener source-project save-back with formatting (project save-back) | 0129 F05. REQ-0129-FIDELITY / UT-0129-FIDELITY proves a qualified copy only. Original write-back needs separate authority |
| PW62 | Share a draft and comments (plans) | 0129 F07/F08. REQ-0129-SHARE / IT-0129-SHARE. Safe recipient-scoped exchange. Hosted interaction stays separate |
| PW63 | Desktop Everywhere and specific host integration (host compatibility) | 0130 S03/S04 and REQ-0130-ROUTE / IT-0130-ROUTE. The exact host list below fixes the documented scope and proof class for each row |
| PW64 | Browser, native Word, Google Docs and writing app routes (features, host compatibility) | REQ-0130-ROUTE / IT-0130-ROUTE. Preserve each Chrome, Firefox and Edge website, native Word, Google Docs and app role separately. Local proofs cannot establish native integration |
| PW65 | Standalone web editor with saved documents and separate mobile capture, writing, editing and progress (plans) | 0129 F01/F02 and 0130 S01/S06. REQ-0130-ROUTE / IT-0130-ROUTE. Web editor access and mobile beta are distinct. The compatibility guide excludes Linux, iOS and Android toolkit integration and names browser access separately |
| PW66 | Local course and recorded expert material (community) | 0126 H08. E2E-0126-PRIVATE verifies refused media and privacy. QR-124 holds successful playback, captions, progress and replay fixtures for admitted supplied material. A hosted expert catalogue has its own effect |
| PW67 | Live workshops, replays and query sessions (features, community) | 0126 H08/H10. REQ-0126-LOCAL / IT-0126-LOCAL. Separate live attendance, questions and recordings |
| PW68 | Daily co-writing, guided practice and themed challenges (community, challenge) | 0126 H09/H10. REQ-0126-LOCAL / IT-0126-LOCAL. Local sprint and real multi-person presence have distinct effects |
| PW69 | Community help, networking and peer or live expert critique (community) | 0126 H10 and 0129 F07. REQ-0130-ROUTE / IT-0130-ROUTE. Local material and comments do not satisfy hosted group participation |
| PW70 | Themed challenge dashboard, plan, goal, daily word log and awards (challenge, preparation) | 0126 H09 and REQ-0126-GOAL / UT-0126-GOAL. Keep title, goal, recorded daily count and milestone evidence. PW78–PW83 define split local and hosted review obligations |
| PW71 | Challenge organiser entry, event RSVP and sponsor offers (challenge, community) | 0126 H10. REQ-0126-COVER / IT-0126-COVER. Keep each documented outside effect separately eligible for decision |
| PW72 | Contest entry package, fees, term checks, outcome and prizes (contest, contest terms) | 0126 H10 and 0129 F06 with REQ-0129-FIDELITY / UT-0129-FIDELITY for a local prepared copy. Record 2026 entry as a closed source edition and capture intended limits and disclosure rules. Submit, payment, panel evaluation and award need separate decisions |
| PW73 | Feature walkthrough, help and beta participation (community, plans) | 0130 S15. REQ-0130-ROUTE / IT-0130-ROUTE. Local material, live support and hosted testing kept separate |
| PW74 | Custom literal or contextual rule with example, speech tags, proposed replacement and explanation (custom rules) | 0128 T03 with REQ-0128-RULES / UT-0128-RULES for applicable rule. Test example analysis and exact rule preview individually |
| PW75 | Configure manuscript feedback goals and review state (focused critique) | 0125 request and review note. Request local focus, category status, navigation and reviewed note separately. QR-124 holds explicit fixture evidence per state |
| PW76 | Invent candidate names for fictional people, places and objects (creative names) | 0125 publication and planning options. Keep names labelled as proposals and prove distinct case types under QR-124 |
| PW77 | An accidentally used similarity check is unavailable to recover (used checks) | Keep the reference paid-credit rule separate from local analysis retry and history. 0127 E13 and QR-124 record exact local lifecycle and outside entitlement behaviour |
| PW78 | Challenge session tag and independent daily word entry (challenge) | 0126 H03/H09. Test each route, duplicate, corrections and common account for eligible words. A hosted challenge retains its own route |
| PW79 | Challenge day boundary and different entry and correction windows (challenge) | 0126 H09 and REQ-0126-GOAL / UT-0126-GOAL. Use distinct synthetic dates and UTC rollover for the reference variant. Local configured dates stay explicit |
| PW80 | Show same daily progress in table, line and bar views, plus goal and average (challenge) | 0126 H09. Verify consistent data and accessible alternative on all views. QR-124 holds exact variants |
| PW81 | Challenge word, habit and participation awards (challenge) | 0126 H09 for local evidenced word and routine milestones. Actual group participation and sponsor reward are separate |
| PW82 | Short timed planning sessions and a staged preparation course (preparation) | 0125 P04 and 0126 H08. Local sessions and admitted lessons need exact input, playback and completion fixtures. Real cohort remains a separate effect |
| PW83 | Kickoff with two planning periods, peer breakouts, check-in and live expert workshop (preparation) | 0126 H10 for a schedule and manual preparation. Track real attendance, RSVP, meeting, breakout and workshop each under DR-114 |

### ProWritingAid host evidence

The compatibility guide (updated 19 November 2026) gives the
following distinct cases. For each, compare the deliberate file
or selection route under REQ-0130-ROUTE and IT-0130-ROUTE.
A local proof covers an explicit copy. Native live effects,
format coverage and app permissions need separate proof or
an owner decision. All named versions and partial cases are
source scope. Do not infer unnamed versions.

| ID | Host and source mode |
|---|---|
| PW-W01 | Adobe RoboHelp 2020, Windows desktop |
| PW-W02 | Dabble, Windows desktop |
| PW-W03 | Discord, Windows desktop |
| PW-W04 | Evernote, Windows desktop |
| PW-W05 | Figma, Windows desktop |
| PW-W06 | FocusWriter, Windows desktop |
| PW-W07 | LibreOffice Writer, Windows desktop |
| PW-W08 | LivingWriter, Windows desktop |
| PW-W09 | Microsoft Mail and Calendar, Windows desktop |
| PW-W10 | Microsoft OneNote, Windows desktop |
| PW-W11 | Microsoft Outlook, Windows desktop |
| PW-W12 | Microsoft PowerPoint 2016 or newer, Windows desktop |
| PW-W13 | Microsoft Teams, Windows desktop |
| PW-W14 | Microsoft Word 2016 or newer and Microsoft 365, with Word add-in alongside Desktop Everywhere, Windows desktop |
| PW-W15 | Milanote, Windows desktop |
| PW-W16 | Notion, partial, Windows desktop |
| PW-W17 | Obsidian, partial, Windows desktop |
| PW-W18 | Plottr, Windows desktop |
| PW-W19 | Quip, Windows desktop |
| PW-W20 | Scrivener, Windows desktop |
| PW-W21 | Skype, Windows desktop |
| PW-W22 | Slack, Windows desktop |
| PW-W23 | Slite, Windows desktop |
| PW-W24 | Telegram, Windows desktop |
| PW-W25 | Thunderbird, Windows desktop |
| PW-W26 | Vellum, Windows desktop |
| PW-W27 | WhatsApp, Windows desktop |
| PW-W28 | Windows 11 Notepad, Windows desktop |
| PW-W29 | WordPad, Windows desktop |
| PW-W30 | yWriter, Windows desktop |
| PW-M01 | Discord, macOS desktop |
| PW-M02 | Drafts, macOS desktop |
| PW-M03 | Evernote, macOS desktop |
| PW-M04 | iA Writer, macOS desktop |
| PW-M05 | Mail, macOS desktop |
| PW-M06 | Microsoft Outlook, macOS desktop |
| PW-M07 | Microsoft Word 2016 or newer, macOS desktop |
| PW-M08 | Notes, macOS desktop |
| PW-M09 | Obsidian, partial, macOS desktop |
| PW-M10 | Pages, macOS desktop |
| PW-M11 | Scrivener, macOS desktop |
| PW-M12 | Slack, macOS desktop |
| PW-M13 | Slite, macOS desktop |
| PW-M14 | Spark Desktop, macOS desktop |
| PW-M15 | Storyist, macOS desktop |
| PW-M16 | TextEdit, macOS desktop |
| PW-M17 | Vellum, macOS desktop |
| PW-M18 | WhatsApp, macOS desktop |
| PW-B01 | Atticus, Chrome route also listed from Windows and macOS, browser extension |
| PW-B02 | Campfire Write, browser extension |
| PW-B03 | Discord, browser extension |
| PW-B04 | Facebook, browser extension |
| PW-B05 | Google Chat, browser extension |
| PW-B06 | Google Docs, browser extension |
| PW-B07 | Google Mail including Gmail, browser extension |
| PW-B08 | Google Search, browser extension |
| PW-B09 | Google Translate, browser extension |
| PW-B10 | LinkedIn, browser extension |
| PW-B11 | LivingWriter, browser extension |
| PW-B12 | Milanote, browser extension |
| PW-B13 | Microsoft Outlook, browser extension |
| PW-B14 | Notion, browser extension |
| PW-B15 | Quip, browser extension |
| PW-B16 | Wattpad, browser extension |
| PW-B17 | WordPress, browser extension |
| PW-B18 | World Anvil, browser extension |
| PW-B19 | X, browser extension |
| PW-X01 | Linux, standalone browser editor, native toolkit integration excluded in the source |
| PW-X02 | Android, standalone browser editor, native toolkit integration excluded in the source |
| PW-X03 | iOS, standalone browser editor, native toolkit integration excluded in the compatibility guide |
| PW-X04 | Mobile beta in the plan guide, independent from the older native toolkit integration claim |

[p-features]: https://prowritingaid.com/features
[p-plans]: https://prowritingaid.com/pricing
[p-lang]: https://help.prowritingaid.com/article/61-which-languages-does-prowritingaid-support
[p-reports]: https://prowritingaid.com/features/writing-reports
[p-help]: https://help.prowritingaid.com/category/53-the-reports
[p-details]: https://help.prowritingaid.com/category/53-the-reports/3
[p-settings]: https://help.prowritingaid.com/article/296-getting-started-with-prowritingaid
[p-styles]: https://help.prowritingaid.com/article/312-what-are-the-different-writing-styles-available
[p-summary]: https://help.prowritingaid.com/article/352-how-to-use-the-summary-report
[p-readable]: https://help.prowritingaid.com/article/41-how-to-use-the-readability-report
[p-line]: https://help.prowritingaid.com/article/31-what-does-the-writing-style-report-do
[p-words]: https://help.prowritingaid.com/article/39-how-to-use-the-thesaurus-check
[p-rewrite]: https://prowritingaid.com/features/sparks
[p-presets]: https://help.prowritingaid.com/article/247-what-s-the-difference-between-sparks-and-rephrase
[p-creative]: https://prowritingaid.com/2023-roundup
[p-plan]: https://help.prowritingaid.com/article/378-what-is-story-canvas
[p-prompts]: https://help.prowritingaid.com/article/433-what-are-creative-nudges
[p-timer]: https://help.prowritingaid.com/article/417-how-to-use-write-mode
[p-session]: https://prowritingaid.com/features/writing-sessions
[p-mark]: https://help.prowritingaid.com/article/436-how-to-use-placeholders
[p-reflect]: https://prowritingaid.com/features/reflections
[p-break]: https://help.prowritingaid.com/article/438-what-is-the-resets-feature
[p-streak]: https://help.prowritingaid.com/article/443-what-are-streaks
[p-critique]: https://help.prowritingaid.com/article/322-chapter-critique-vs-manuscript-analysis-vs-virtual-beta-reader
[p-plot]: https://help.prowritingaid.com/article/420-what-type-of-feedback-does-plot-analysis-provide
[p-character]: https://help.prowritingaid.com/article/428-what-type-of-feedback-does-character-analysis-provide
[p-ms]: https://help.prowritingaid.com/article/318-what-is-manuscript-analysis
[p-reader]: https://help.prowritingaid.com/article/337-what-type-of-feedback-does-virtual-beta-reader-provide
[p-market]: https://help.prowritingaid.com/article/375-what-type-of-feedback-does-marketability-analysis-provide
[p-sim]: https://help.prowritingaid.com/article/29-how-does-the-plagiarism-check-work
[p-history]: https://help.prowritingaid.com/article/429-how-to-save-view-print-or-download-reports-in-critique
[p-used]: https://help.prowritingaid.com/article/76-can-i-recover-past-plagiarism-checks
[p-files]: https://help.prowritingaid.com/article/187-which-files-are-compatible-with-prowritingaid
[p-hosts]: https://help.prowritingaid.com/article/201-compatible-apps-websites-prowritingaid
[p-scrivener]: https://help.prowritingaid.com/article/228-why-are-my-changes-not-saving-to-scrivener
[p-community]: https://prowritingaid.com/features/community
[p-challenge]: https://help.prowritingaid.com/article/380-how-to-use-the-novel-november-dashboard
[p-prep]: https://help.prowritingaid.com/article/448-what-is-preptober
[p-contest]: https://prowritingaid.com/novel-beginnings
[p-terms]: https://prowritingaid.com/novel-beginnings/terms
[p-controls]: https://help.prowritingaid.com/article/244-what-is-sparks
[p-focused]: https://help.prowritingaid.com/article/320-what-type-of-feedback-does-manuscript-analysis-provide
[p-rules]: https://help.prowritingaid.com/article/388-how-to-create-a-custom-style-guide
[p-names]: https://prowritingaid.com/fantasy
[p-inspire]: https://help.prowritingaid.com/article/251-what-s-the-difference-between-sparks-edit-and-sparks-inspire
[p-limits]: https://help.prowritingaid.com/article/161-does-prowritingaid-have-a-word-limit
