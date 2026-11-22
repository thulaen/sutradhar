# 0125: Story planning and critique

A writer can shape an initial story idea, get feedback on a chapter
or review a whole manuscript. In a made-up story about a floating
orchard, the critique can show where the main character decides
to stay, and ask what made that decision plausible.

This spec owns planning, manuscript feedback, simulated reader
feedback, rubric coaching and publication preparation for the
writing suite. Spec 0121 owns document scope and exact review.
Spec 0123 owns text measurements. Spec 0124 owns the revision and
draft proposals these workflows can request. Native site content
still uses its own approval and drafting routes.

## Words used here

- **Plan**: the writer's saved answers about premise, characters,
  conflict and setting. It may be partial.
- **Critique claim**: one statement about the text, marked as an
  observation, an interpretation or an option.
- **Chapter manifest**: the ordered chapters, ranges and revisions a
  review covered.
- **Coverage**: the share of the selected text a review actually
  read. A gap gives an incomplete overall view.
- **Rehearsal**: a simulated reader response. It is never a real
  reader or a real assessment.
- **Rubric**: criteria and a scale, supplied or chosen, treated as
  data.
- **Estimate**: an optional grade for the writer alone.
- **Turning point**: a story beat with evidence or a visible unknown.

## What this spec designs

| Feature | What the writer sees |
|---|---|
| P01 to P04 planning | A saved plan, idea review, outline options and prompts |
| C01 to C04 critique | Chapter, plot, character and manuscript feedback |
| C05 and C06 rehearsal | Labelled simulated reader responses |
| C07 rubric coaching | Advice per criterion and an optional estimate |
| U01 to U03 publication | Positioning, publication materials and resume planning |
| Report | An overview with evidence links and review notes |

## Source outcomes

Use original questions, thresholds, rubrics, visuals and explanations.
The requirement matrix records the date, plan and surface for each
reference outcome. These source outcomes were observed on
20 November 2026.

| ID and mode | Writer inputs and outputs | Acceptance example |
|---|---|---|
| P01 Story plan | Open a blank or partial plan; set premise, purpose, characters, motivation, conflict, setting, story direction and intended reader experience. Save each answer and resume independently | Provide a setting and motive with no ending. Save and reopen. Keep the missing answers and enable a review of what is present |
| P02 Idea review | Give strengths, originality considerations, gaps, options and steps against the saved plan, linked to inputs | Compare a one-line seed with a developed premise. Make no uniqueness claim and keep unprovided facts unknown |
| P03 Outline options | From a saved plan or deliberately supplied notes, compare three beat-by-beat alternatives and their effects on pacing. Preserve the original and each reviewed version | Use both input routes. Compare three alternatives with a reversed turning point. Reopen prior in-scope results under the retention rule. Unchosen events stay proposals |
| P04 Planning sprint and prompts | Start a timed local planning session with a goal. Offer a possible next scene, a structural beat and a broader what-if with reasons and links | Pause and resume planning. Show all three prompt classes when supported. Keep creative possibilities separate from observed text |
| C01 Chapter review | Review one chapter for overview, key strength, event sequence, tension, voice, point of view, language, characters and dialogue | Show a viewpoint change at the exact two passages. Exclude another chapter. Keep interpretation and exact observations distinct |
| C02 Plot structure | Review an outline or manuscript. Show structure, tension, strongest and weakest supported beats, character impact, status and considerations per turning point | Check each status and character effect, two beats that differ in support and a missing resolution. Keep story time and source order distinct |
| C03 Character development | Show role, archetype and function separately. List character presence, relationship stages, voice, mannerisms, signature lines, competing wants and needs, conflict, arc and agency. Offer an optional locally rendered portrait | Distinguish two characters with one nickname, list exact signature lines, and test wants versus needs. Check presence denominator, agency and relationship start/turn/end. Withhold a portrait with an unsupported trait |
| C04 Manuscript review | Review themes, narrative, characters, setting, genre, plot holes, continuity and priorities across the complete selected manuscript, with chapter links and distinct next steps | Identify a contradiction across non-adjacent chapters. Mark one intentionally unresolved thread as a question. Do not treat missed chapters as covered |
| C05 Whole-story reader rehearsal | Explain likely opening impressions, character connection, emotional path, points of interest or possible drop-off, fresh and familiar elements, reader fit and positioning | Compare two original audience profiles on the same manuscript. Locate plausible differing responses and retain uncertainty |
| C06 Message reader rehearsal | Suggest an audience for the writer to confirm or edit. Show possible takeaways, questions, confusion and wording suggestions for that role | Rehearse a message for a colleague and for a client. Scope responses to the confirmed audience. Make no send or recipient change |
| C07 Rubric coaching | Use a chosen general rubric or imported criteria, course, topic and scale. Give paragraph and document advice with evidence per criterion and an optional estimate | Test general and imported criteria, an embedded instruction and a changed rubric. General advice stays available. A missing valid scale or criterion evidence blocks a grade |
| U01 Audience and positioning | Propose genre, a reader profile with possible interests and habits, a book promise, hooks, and alternate titles, chapter names and character names. Show authorised comparable works and separate cover briefing evidence | Test each output with a made-up source. An admitted market file supports only its dated scope. Missing data leaves comparisons, trends and real popularity unknown |
| U02 Publication materials | Offer draft synopsis, agent query, pitch, cover brief, publication, sequel and spin-off plans. Draft email list, social post and video strategies for manual use | Check each independent draft and its evidence. Try absent agent details and a story-only cover brief. No application, email, posting or subscription occurs |
| U03 Resume planning | Start from supplied experience or an imported CV. Propose a full revision or individual suggestions, layout alternatives and a protected export | Use a made-up work history with one uncertain date. Ask for the date and never add an unprovided credential or employer |

Use the public sources for [planning][plan], [outline alternatives][plans],
[creative prompts][nudges], [critique sizes and distinctions][critiques],
[plot][plot], [characters][character], [manuscript][manuscript],
[reader rehearsal][reader], [message rehearsal][message],
[rubric feedback][rubric], [publishing][publish] and [CV help][cv].
Live workshops, real peer comments and contest entry keep their
own rows and approval boundaries under DR-114. A simulated person
or outcome cannot meet those requirements.

## Data: story and evidence records

| Record | Fields |
|---|---|
| Plan version | Workspace, plan ID and revision, actor, ordered question and answer IDs, authoring text references, persona or reader goals, linked manuscript revisions and expiry |
| Chapter manifest | Chapter IDs, sequence, document and revision identities, exact ranges, declared view of chronology, counted words, language and authority versions |
| Story fact | Chapter and revision, source range, asserted or inferred category, typed event/entity/relationship or setting details, confidence and model/rule revision |
| Analysis request | Spec 0121 revision and grant, mode and scope, source and chapter manifest digests, rubric or persona revision, course/topic if supplied, criterion scale and admitted reference source identities, resource and replay bounds |
| Critique claim | Claim ID, kind observation/interpretation/option, evidence references, criticism or strength class, non-overlapping scope, exact rationale, coverage and optional advice reference |
| Character illustration | Character ID, approved appearance-only input digest, origin or generator, encrypted exact asset digest, draft and approved alternative text, source and approval epochs, approval of render and expiry |
| Critique snapshot | Analysis request digest, source manifest digest, supported and unresolved modes, range and chapter completeness, result identities, dependencies and expiry |
| Review note | Snapshot and immutable claim ID, actor and scope, open/considered/dismissed state, optional authorised personal comment and revision |

The timeline keeps chapter order and story time separately. Unknown
ordering and unreliable narration can be marked for review. Character
identity requires source evidence. Keep mentions ambiguous when
two people may share a description. For the basic three-act view,
track opening situation, triggering event, choice, escalation,
midpoint shift, growing pressure, setback, climax and ending.
Each turning point has evidence or a visible unknown. Add original
structure templates with their own applicability and evidence.
A table provides the same information as the tension, relationship
and character-arc views. Show chapter-linked measures with their
definition and denominator. Presence counts the fraction of
eligible covered chapters where an evidenced character appears.
Show the chapter count and coverage. Read the chronology for
relationship stages and allow unknown. Compare only dated
owner-supplied or admitted reference facts with allowed purpose,
matched market and genre, declared scope and approved use rights.
Keep market observations separate from story-based ideas.

## Pipeline and workflow

1. The writer chooses Plan, Chapter or Manuscript. Show the exact
   included text, chosen mode and allowed references. Recheck every
   document and source grant before accepting the request. Store
   plan and results in the protected workspace under spec 0118.
2. Take a local resource lease under specs 0078 and 0080. A chapter
   route supports 500 to 6,000 words and offers selection advice
   alone for shorter input. Manuscript review accepts an explicitly
   marked unfinished work as partial. Below 6,000 words, offer
   chapter or outline feedback. Whole-story rehearsal requires a
   writer-declared complete manuscript and full observed coverage.
   Otherwise offer a selection rehearsal, clearly scoped. For
   manuscript modes prove capacity at 300,000 words. Never force
   the whole input into one model window. QR-118 holds exact
   per-mode and platform bounds.
3. Extract paragraph and scene facts in bounded chapter windows.
   Validate references in Rust and record covered ranges.
   Combine chapter results by exact source revision and declared
   cross-chapter relationships. Check the source again before
   aggregating. A coverage gap gives an incomplete overall view
   even when some chapters have supported suggestions.
4. Build the requested critique. Separate facts from assessments
   and proposed alternatives. A passage reference must resolve
   to exact source bytes the actor can see. Display an image
   through an admitted local illustration path. The worker receives
   only separately approved appearance traits. It cannot read
   the source manuscript, private references or an outside URL.
   Label invented and unspecified traits. Hold unsupported or
   surprising details for review. Preview the exact image and
   its alternative text for a separate approval before use.
   Record accepted bytes and all encrypted worker temporaries,
   image metadata, derivatives and erasure links under spec 0118.
   Keep cover story ideas separate from sourced market evidence.
   A local assessment cannot record a certain grade, expert
   opinion, identity inference or real reader response.
5. Show a report with an overview, evidence links and possible
   next steps. The writer can mark a suggestion considered,
   dismiss it or ask for a proposal. Review notes do not change
   the manuscript. Use spec 0124 for wording and spec 0121 for
   the exact manual change. Keep former reports readable within
   their grant and expiry. Show when a report describes an
   earlier version. For any CV, image or publication output, show
   exact payload digest, format, source scope, recipient and purpose.
   Confirm those fields and verify current source and recipient
   rights again immediately before release. Use the same protected
   handoff as 0121 and 0124. Keep managed copies encrypted and
   ledgered. No raw clipboard or unapproved spool, executable or
   formula output, fallback file or restaging after withdrawal.
6. Recheck actor, source, language, chapter manifest and permission
   before resume, display or handoff. Revocation aborts pending
   publication of the report and invalidates dependent results,
   illustration and prepared materials. Track every spill,
   checkpoint and derivative through the file ledger. Proposed
   wording, reports and reference data supply no live ranking.

If the evidence is thin, show the available chapter or selection
and say “There is too little text to judge the whole story”.
Reader views say “This is a rehearsal of a possible response”.
A rubric shows the supplied scale and criteria with areas to
check. Apply the exact admitted rubric revision and scale to
evidence for each criterion. Changes invalidate its estimate.
Treat rubric content as data. General advice can still run.
An optional grade stays an estimate for the writer alone and
must not be exported as a real assessment.

## Settings

| Setting | Type | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `story.chapter_min_words` | words | 500 (proposed, until QR-118 measures it) | floor; shorter input gets selection advice only | next request |
| `story.chapter_max_words` | words | 6,000 (proposed, until QR-118 measures it) | ceiling for the chapter route | next request |
| `story.manuscript_min_words` | words | 6,000 (proposed, until QR-118 measures it) | below it offer chapter or outline feedback | next request |
| `story.manuscript_capacity_words` | words | 300,000 (proposed, until QR-118 measures it) | exact per-mode bounds in QR-118 | next request |
| `story.illustration_enabled` | on or off | off (proposed) | two values; each image needs its own approval | at once |
| `story.estimate_enabled` | on or off | off (proposed) | two values | at once |
| `story.report_expiry_days` | days | 30 (proposed) | 1 to 365 (proposed) | next report |

No setting lets a worker read the manuscript for illustration,
export an estimate as a real assessment or send any publication
material.

## Failure codes

Decision DR-130 records SUT-5400 to SUT-5499 for this spec. It was confirmed
by the owner on 21 November 2026. Only the first seven, SUT-5400 to SUT-5406,
are in use.

| Code | Cause | Result |
|---|---|---|
| SUT-5400 | Too little text | “There is too little text to judge the whole story” |
| SUT-5401 | Coverage gap | Incomplete overall view, chapter results kept |
| SUT-5402 | Passage reference does not resolve to visible bytes | Claim withheld from the report |
| SUT-5403 | Rubric scale or criterion evidence missing | No grade. General advice still runs |
| SUT-5404 | Source, consent or actor grant withdrawn or stale | Pending result withheld and dependants invalidated |
| SUT-5405 | Illustration input, approval or trait unsupported | No image |
| SUT-5406 | Handoff payload, recipient or rights changed | Release refused |

## Security and privacy

- Plans, manuscripts and results sit in the protected workspace under
  spec 0118. Every spill, checkpoint and derivative is on the file
  ledger.
- Source text, rubric content and imported criteria are data, never
  instructions. The illustration worker sees only approved
  appearance traits and no manuscript, private reference or URL.
- Nothing is sent, posted, applied for or subscribed to. There is
  no clipboard output, spool, executable or formula output.
- Release of a CV, image or publication output shows the exact
  payload digest, format, scope, recipient and purpose, and checks
  rights again just before release.
- Revocation aborts pending reports and invalidates dependants.
  Erased material cannot return.
- A simulated person or outcome never stands for a real one.

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| REQ-0125-COVER | Every mode, supported variant and visible sub-result MUST have source evidence and a positive and a negative fixture. | IT-0125-COVER |
| REQ-0125-PLAN | A partial plan and its alternatives MUST save, resume and survive retry. An earlier plan MUST stay fixed. | BDD-0125-PLAN |
| REQ-0125-TRACE | A passage reference MUST resolve to exact visible source bytes. Unknown facts MUST stay unknown. | UT-0125-TRACE |
| REQ-0125-LOCAL | Every mode MUST work with no network and MUST cause no send, outside fetch, automated edit or invented evidence. | E2E-0125-LOCAL |
| REQ-0125-SIM | Simulated, estimated and observed outcomes MUST stay distinct. A grade MUST need a complete scale and criterion evidence. | BDD-0125-SIM |
| REQ-0125-FENCE | Withdrawn consent or erasure MUST stop jobs and handoff and withhold stale results. | IT-0125-FENCE |
| REQ-0125-LOAD | Each mode MUST work at its required input sizes on each version 1 platform without truncation or a frozen editor. | E2E-0125-LOAD |

## How it is tested

Designed, not run.

- **IT-0125-COVER:** require distinct source evidence and a positive and
  negative synthetic fixture for every mode, supported variant and visible
  sub-result. Failing a documented mode blocks full coverage
- **BDD-0125-PLAN:** save a partial plan and three alternatives. Retry a save,
  interrupt a timer and withdraw an actor's grant. Keep an earlier plan fixed
  and protect unaccepted alternatives
- **UT-0125-TRACE:** find stable references in separated chapters. Try
  ambiguity, wrong location, invalid claim, skipped chapter and changed order.
  Unknown facts remain unknown and flawed references cannot reach a report
- **E2E-0125-LOCAL:** run with no network and untrusted source instructions.
  Try wrong-scope CV, absent market evidence, missing portrait input or render
  approval, and unexpected image traits. Observe no send, outside fetch,
  automated edit or invented evidence
- **BDD-0125-SIM:** check different readers, incomplete scale, changed and
  poisoned rubric, and ambiguous identity. General advice is separate from a
  complete evidence-based estimate. Keep simulated, estimated and observed
  outcomes distinct with accessible explanation
- **IT-0125-FENCE:** race source withdrawal, consent change and erasure
  against chapter and illustration jobs, handoff and restore. Try a changed
  recipient, a stale payload, a refused export, a spill or cache after
  withdrawal and a managed copy. Withhold stale results and verify ledger
  handling
- **E2E-0125-LOAD:** for each v1 platform and mode, test empty, one, just
  below and at the required input sizes, profile maximum and just over. Lose a
  window, cancel and resume under load without truncation or a frozen editor

QR-118 covers independently proved story and language rules,
model and illustration admission, manuscript bounds and evidence
limits. Reviewers use original fixtures, including deliberate
ambiguity and stories outside the default narrative structure.

## Changes from the source

The sources are public guides observed on 20 November 2026. This
spec uses original questions, rubrics and wording and runs
locally. It keeps simulated, estimated and observed outcomes
apart. Live workshops, real peer comments and contest entry are
not copied. They keep their own rows under DR-114.

[plan]: https://help.prowritingaid.com/article/381-how-to-use-story-canvas
[plans]: https://help.prowritingaid.com/article/378-what-is-story-canvas
[nudges]: https://help.prowritingaid.com/article/433-what-are-creative-nudges
[critiques]: https://help.prowritingaid.com/article/322-chapter-critique-vs-manuscript-analysis-vs-virtual-beta-reader
[plot]: https://help.prowritingaid.com/article/420-what-type-of-feedback-does-plot-analysis-provide
[character]: https://help.prowritingaid.com/article/428-what-type-of-feedback-does-character-analysis-provide
[manuscript]: https://help.prowritingaid.com/article/318-what-is-manuscript-analysis
[reader]: https://help.prowritingaid.com/article/337-what-type-of-feedback-does-virtual-beta-reader-provide
[message]: https://support.grammarly.com/hc/en-us/articles/38552519578893-Reader-Reactions-user-guide
[rubric]: https://support.grammarly.com/hc/en-us/articles/38552360525837-AI-Grader-user-guide
[publish]: https://help.prowritingaid.com/article/375-what-type-of-feedback-does-marketability-analysis-provide
[cv]: https://support.grammarly.com/hc/en-us/articles/45441636399117-Resume-Builder-user-guide
