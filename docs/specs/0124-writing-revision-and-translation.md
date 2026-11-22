# 0124: Writing revision and translation

A writer can try a different wording, produce a translation or
turn notes into a draft. Each result opens in a comparison pane.
The source stays available and the writer chooses whether to copy
inside the workspace, accept, revise or dismiss the proposal.

Spec 0121 owns the document, revision and approval. Spec 0122 owns
language ranges and protection. Spec 0123 owns report findings.
This spec owns revision and language-change proposals for the
writing workspace. Site content drafting reuses the approval and
evidence route in specs 0042 and 0052. It keeps the wording provider
order defined there. This workspace uses the local route under
DR-114. It never falls back to a paid or remote model.

## Example

Nia selects one paragraph of a made-up report and asks for a
friendlier tone. The comparison pane shows her paragraph beside
the proposal, marks a changed number as a protected item and holds
acceptance until she reviews it. She then asks for a Spanish
translation of one sentence. The pane shows the exact target label
and keeps her source unchanged. She dismisses the first proposal
and accepts the translation. Nothing leaves the workspace.

## Words used here

- **Proposal**: model-generated wording shown for review. It changes
  nothing until the writer accepts it.
- **Candidate**: one proposal bound to a source range, with its
  protected-item evidence.
- **Protected item**: a number, unit, date, name, link, quotation,
  negation or identifier whose meaning must survive a revision.
- **Effect kind**: what accepting a result can do: replacement,
  inspiration, new draft, reply draft or read-only reading
  assistance.
- **Voice profile**: a writer's enrolled examples and the traits
  derived from them, used only with their grant.
- **Direction**: one source label and one target label in the
  translation matrix.
- **Run receipt**: the record of one run's digests, versions and
  bounds.

## What this spec designs

| Feature | What the writer sees |
|---|---|
| Sentence and paragraph revision | Alternative wording in a comparison pane |
| Document revision | Separately reviewable alternatives in source order |
| Perspective, tense, summary and expansion | Reviewable proposals with omissions and unverified claims marked |
| Inspiration and draft assistance | Labelled prompts, drafts and reply drafts |
| Purpose advice | Proposals for a named goal and audience |
| Selection translation | A target in the exact supported label |
| Reading assistance | Read-only explanations of supplied text |
| Voice profile | An opt-in profile the writer can pause, disable or erase |

## Requests and outcomes

Reference outcomes come from [sentence rewriting][rephrase],
[creative revision][edit], [creative options][options],
[writing assistance][assist],
[paragraph rewriting][para], [tone and voice][voice] and
[LanguageTool Premium][lt]. The coverage record in spec 0121 keeps
each originating plan, surface and date. Observed 20 November 2026.

| Request family | Distinct actions to cover | Acceptance example |
|---|---|---|
| Sentence wording | Alternative wording, fluency, formal or casual tone, shorter or longer wording and sensory phrasing | Compare original and new sentence. A shorter proposal preserves its meaning and guarded facts. A sensory proposal labels invented detail |
| Paragraph revision | Readability, fluency, transition, clearer structure and voice, formal or friendly tone, concision and one or more caller-selected improvements | Reorder a synthetic paragraph with two distinct facts. Preserve both and show exactly which paragraph the action would replace |
| Document revision | Work through suggestions by paragraph; choose academic, professional, concise, imaginative, plain or natural wording; allow a local personal voice | Return separately reviewable alternatives in stable source order. Leave a stale paragraph unchanged. Keep choice and scope distinct |
| Perspective and tense | First person, third person, past and present, with preservation of quoted speakers and temporal facts | Change point of view across a synthetic narration and dialogue. Keep speaker, quotation and time relationship unchanged |
| Summarise and expand | Condense selected text, expand from notes or bullets and optionally compare against individual supported points | A summary maps each retained point to input. Mark omissions. Expanding notes flags any new factual claim as unverified |
| Inspiration | Next event, action, dialogue, emotion, sensory detail, analogy, counterargument, definition, emphasis, example, explanation, humour and quote prompts | Use a fiction scene and a factual passage. New fictional details stay labelled. An attributed quote requires exact authorised source words |
| Draft assistance | Plan an outline, brainstorm, ask about selected text, compose a draft from an intent, draft a reply from selected context, and revise a suggestion with a follow-up | Give the reply tool a selected incoming message with an embedded instruction. It drafts a reply without taking the instruction or sending it |
| Purpose advice | Suggest a clearer main point, missing reader context or anticipated objections for a named goal and audience | Compare two audiences using the same text. Give specific proposals with evidence, retaining uncertainty about real reader reaction |
| Selection translation | Translate a selected source into the chosen supported target, with separate source and target ranges and review | Translate both directions for an admitted pair. Preserve numbers, names or approved transliterations and source context |
| Reading assistance | Explain, summarise, answer a question about or translate an explicitly supplied selection for reading, including received material the actor may supply | Return an inspectable explanation with supporting passages. Keep the supplied document byte-for-byte unchanged |

The report view can open a revision request on an exact cited
range. Any model-generated wording is a proposal. Preserve the
normal live checking path when generative requests are disabled.

## Language coverage

For paragraph rewriting, cover English, French, German, Italian,
Portuguese, Spanish and Ukrainian. Record each local profile and
source surface. Track standard improvements and explicit custom
instructions as distinct controls in each profile. A public page
may list fewer modes than its
support guide. The [paragraph rewriting guide][para] names
Ukrainian. For sentence tone, distinguish whole-text tone
observations from changing a precise passage. Strategic advice
from its [reference guide][strategy] is documented for American
English. Keep that mode in the capability matrix.

The [translation guide][translate] names these styles of input
and output: Simplified Chinese, Dutch, British and US English,
French, German, Indonesian, Italian, Japanese, Korean, Polish,
generic and Brazilian Portuguese, generic Spanish, Argentinian
Spanish, Mexican Spanish, Swedish, Turkish and Ukrainian. The
manifest expands all 19 input and 19 output labels into a
directed matrix. Keep every required row. The same-variant
diagonal changes no bytes. A dialect-only row needs evidence of
a distinct conversion or an owner decision about its outcome.
Record source limitations or gaps explicitly. A required missing
direction blocks and cannot pass by testing its reverse. Show
the exact output label even when models serve several variants.
Detect the source when the writer leaves it automatic. If
detection is ambiguous, request the source language. Use the
explicit choice of target in the preview and validate the
model's output against it.

LanguageTool documents translation across more than ten languages
without naming all pairs on the cited page. QR-117 holds discovery
and evidence for those pairs. Compare the observed sets and retain
any additional supported pairs in the requirement matrix. It also
holds the reading-assistance, interactive workflow and host variants
that need separate equivalence evidence. It cannot reuse the syntax
and spelling matrix as a substitute for translation coverage.

Support a selection of at least 4,000 Unicode scalar values in
translation and at least 2,500 in creative paragraph revision.
When an admitted worker needs windows, split at natural boundaries
and track original ranges, paragraph order and context. Keep
the full original bytes. Show the affected range and reason when
the selected route cannot evaluate part of the request. Reject
oversize work before loading a model when even sliced work falls
outside the admitted job budget under spec 0078.

## Data and guarded changes

| Record | Fields |
|---|---|
| Revision request | Spec 0121 request, action and variant, source ranges, allowed context and evidence references, goal and genre, output language, prompt revision, optional profile revision, budget and replay key |
| Voice profile | Scope and actor, revision, separate document or message mode, name, role, dialect, versioned style features and exact authorised source references, consent and expiry |
| Protected item | Source range and identity, item kind, comparison rule, allowed form and reason; used for numbers, units, dates, names, links, quoted text, negation and identifiers |
| Candidate | Revision request ID, source range, order, effect kind, candidate text, protected-item evidence, language, review action and available coverage |
| Claim and item ledger | Stable source anchor, exact input range and all output spans, permitted transformation or omission rule, added claim evidence, confidence and verdict for each item |
| Run receipt | Request and input digests, rule and model versions, permission epoch, bounds used, result references, completion outcome and expiry |

Effect kinds are replacement of a selected range, inspiration
for manual placement, ordinary new draft, reply draft and read-only
reading assistance. A replacement binds exact source bytes.
Inspiration preserves the source and needs a separate insertion
choice. New drafts are separate from the source. A reply has
no addressing, delivery or scheduling effect. Read-only output
cannot change a document. Site-native draft operations still need
their exact authorisation under 0042 and 0052. This workspace
cannot add a site draft input or provider dispatch.

A profile uses explicitly enrolled examples or separately opted-in
writing in the active project. Show its source ranges and traits.
Pause updates to stop enrolling new examples. Disable use to
refuse any new or pending request that would use the profile.
Removing an example or trait changes the profile version and
invalidates dependent results. Erasure also removes the profile,
retained source references and derivatives by ledger identity.
No ambient screen, clipboard, browser or message data can enter.
Shared profiles require their own approved role and source grant.
Bind examples and derived features to the exact purpose, owner
and individual author grants, approved size and expiry. Keep
them protected under specs 0030, 0032 and 0118.

## Execution and review

1. Validate action, current scope, actor, input and context ranges,
   target language, dependencies and profile. Snapshot them under
   the current consent epoch. Submit a bounded local job. A model
   lease comes only through specs 0078 and 0080.
2. Use a typed job envelope. Keep the writer's selected instruction
   in its own field and the selected drafts, example text and
   references in data fields. A model cannot open a file, fetch
   a source, invoke a tool or choose its own model route from
   within the job. Validate each requested language and action
   against the admitted manifest before running.
3. Parse only the allowed output record. Compare names, quantities,
   dates, links, quotations, negation and factual anchors with
   the input. Bind each item to all valid output occurrences across
   the assembled windows. Identify omissions, duplicates, changes of
   units, negation and factual additions. A faithful revision or
   translation requires all protected meaning. A summary can omit
   details under its declared policy and must show those omissions.
   Inspiration can add clearly marked fictional details. A disputed
   or unsupported factual change holds acceptance. A similarity
   score cannot prove unchanged meaning. Offer an evidence question
   or labelled brainstorming note where knowledge is incomplete.
   Refuse fabricated citations or attributions.
4. The comparison pane shows source and proposed text, changed
   protected items, purpose, covered range and any uncertainty.
   Restore original wording with the regular undo route. Accept
   only the exact reviewed bytes under spec 0121. Check current
   document, actor, source, context and profile grants again
   at acceptance, copy into the workspace and any handoff.
   Bind the action, revision and payload bytes. A result with
   unresolved protected or disputed changes remains read-only
   and cannot be copied, exported or accepted. Read-only
   explanations of supplied material use their own admitted
   disclosure scope. An outside copy requires deliberate
   recipient-bound approval under spec 0118. Raw protected
   output cannot go to the OS clipboard. For selected-region grouped
   acceptance, use the atomic same-base rules in spec 0122. A late
   authority denial for one item aborts the group. Revalidate any
   writer-altered candidate as a new revision with a new preview.
   Choosing “Try another version” creates a linked fresh request
   and preserves the accepted revision.
5. Cancel, make permission changes effective, fence out stale model
   results and clear temporary work under specs 0030, 0032 and
   0118. Refuse a stale preview even if its model output is cached.
   The writer can opt to keep private request history. Approve
   its exact purpose, bounded entries and expiry first. Store it
   encrypted under the ledger with its dependency identities.
   Check the latest authority and deletion marker on lookup,
   retry, queue replay and restore. Earlier consent cannot revive
   erased text or derivatives. History never trains a model or
   supplies live ranking. Keep raw text and prompts out of logs.

If a model or an admitted route is unavailable, say “This writing
action is unavailable for the selected text” and explain the
model, scope or language cause. Other admitted checks continue.
A missing translation pair cannot produce text in another locale
under the requested label.

## Settings

| Setting | Type | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `revision.generative_enabled` | on or off | off (proposed) | two values | at once |
| `revision.translate_min_chars` | Unicode scalar values | 4,000 (proposed) | 4,000 or more, up to the admitted job budget (spec 0078) | next request |
| `revision.paragraph_min_chars` | Unicode scalar values | 2,500 (proposed) | 2,500 or more, up to the admitted job budget (spec 0078) | next request |
| `profile.updates_paused` | on or off | off | two values | at once |
| `profile.use_enabled` | on or off | off (proposed) | two values | at once |
| `history.keep` | on or off | off | two values, needs approval of purpose first | next request |
| `history.max_entries` | whole number | 50 (proposed) | 1 to 500 (proposed) | next entry |
| `history.expiry_days` | days | 30 (proposed) | 1 to 90 (proposed) | next entry |

The two minimums are the floors this spec states. No setting
adds a paid or remote model route (DR-114), lets a reply send or
lets history train a model.

## Failure codes

Decision DR-130 records SUT-5300 to SUT-5399 for this spec. It was confirmed
by the owner on 21 November 2026. Only the first six, SUT-5300 to SUT-5305,
are in use. The writer sees the plain message, never the code alone.

| Code | Cause | Result |
|---|---|---|
| SUT-5300 | Model, scope or language unavailable | “This writing action is unavailable for the selected text”. Other checks continue |
| SUT-5301 | Translation direction missing or target label mismatch | No text under the requested label |
| SUT-5302 | Selection over the admitted job budget | Refused before a model loads |
| SUT-5303 | Protected or disputed change unresolved | Result stays read-only |
| SUT-5304 | Stale preview, document or grant at acceptance | Refused, even if output is cached |
| SUT-5305 | Profile paused, disabled or erased | New or pending request refused |

## Security and privacy

- Input stays on this PC. The route is local under DR-114 and never
  falls back to a paid or remote model.
- The job envelope keeps the writer's instruction apart from data.
  A model cannot open a file, fetch a source, invoke a tool or
  choose its route.
- Examples, profiles and derived features are bound to purpose,
  owner and individual grants. They are protected under specs 0030,
  0032 and 0118. Erasure removes them by ledger identity.
- No ambient screen, clipboard, browser or message data can enter.
- Raw protected output cannot go to the OS clipboard. An outside
  copy needs recipient-bound approval under spec 0118.
- Raw text and prompts stay out of logs. Request history is opt-in,
  encrypted and never trains a model.

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| REQ-0124-MODES | Each controlled action and effect kind MUST have its own evidence. A reply MUST NOT send and a reading action MUST NOT write. Live checks MUST work with generative wording off. | IT-0124-MODES |
| REQ-0124-LANG | Every admitted translation direction MUST be tested and show its exact label. A same-variant row MUST change no bytes and a missing direction MUST block. | IT-0124-LANG |
| REQ-0124-INVARIANTS | A faithful result MUST keep every protected item. Omissions, duplicates and factual additions MUST be shown and a similarity score MUST NOT prove meaning. | UT-0124-INVARIANTS |
| REQ-0124-CONSENT | A wrong-scope input, planted instruction, paused or disabled profile, erasure or expired history MUST cause no outbound request, privileged action or stale result. | E2E-0124-CONSENT |
| REQ-0124-REVIEW | Only the exact reviewed bytes MAY be accepted, after grants are checked again. Disputed output MUST stay read-only. | BDD-0124-REVIEW |
| REQ-0124-BOUNDS | Each stated floor and the admitted maximum MUST work on each version 1 platform. Over-limit work MUST be refused with all source bytes kept. | E2E-0124-BOUNDS |

## How it is tested

Designed, not run.

- **IT-0124-MODES:** require distinct evidence for each controlled action and
  the exact range examples in the table. Exercise separate replacement,
  inspiration placement, new draft, reply and static reading effects. No reply
  sends and no reading action writes. Toggle generated wording off and confirm
  live checks remain available. Omit one documented action and fail the
  inventory check
- **IT-0124-LANG:** test every admitted direction with native review, numeric
  and name placeholders, degraded context, missing model and unsupported
  variants. Compare direction failure, same-variant no-op, ambiguous automatic
  input and a deliberately crossed target label
- **UT-0124-INVARIANTS:** try changed number, quote, date, negation, unit,
  name, source link and prohibited claim. Assemble an omitted or duplicated item
  across batch seams. Check unit and negation changes, allowed summary omissions
  and labelled fiction. Evaluate style and factual preservation separately
- **E2E-0124-CONSENT:** use wrong-scope text, try a planted instruction, pause
  profile updates, disable use, remove a source example, erase a profile and
  expire history. Race these against pending results, a linked retry and old
  restored state. Observe no outbound request, privileged action or stale result
- **BDD-0124-REVIEW:** compare, edit and accept one exact proposal, request a
  second version, reject overlap and stale state, and withdraw a grant between
  preview and copy or apply. Refuse disputed output and protected clipboard
  plaintext, then exercise accepted edits and undo. Try a denied item in a
  reviewed edit group and a revised candidate. An approval cannot transfer
  across a changed document or proposal revision
- **E2E-0124-BOUNDS:** on each version 1 platform exercise empty, one,
  documented floor, admitted maximum and one over. Pause, cancel, fail a
  dependency and change revisions at batch seams. Respect memory and time leases
  and preserve all source bytes

Under QR-117, sign quality evidence by action, language direction,
profile, worker and supported operating system. Include native
review of meaning, fabricated additions, omissions and phrasing,
with a synthetic corpus and the configured resource bounds.
Neither fluency alone nor the presence of one working model
qualifies a missing action or direction.

## Changes from the source

The sources are public guides observed on 20 November 2026. This
spec keeps their outcomes as local, reviewable proposals. It adds
the protected-item check, the exact translation matrix and the
refusal of a paid or remote fallback (DR-114). It does not copy
any source wording. Hosted features and site-native draft
operations stay with their own specs.

[rephrase]: https://help.prowritingaid.com/article/247-what-s-the-difference-between-sparks-and-rephrase
[edit]: https://prowritingaid.com/features/sparks
[options]: https://prowritingaid.com/2023-roundup
[assist]: https://support.grammarly.com/hc/en-us/articles/14528857014285-Introducing-generative-AI-assistance
[para]: https://support.grammarly.com/hc/en-us/articles/30916398193037-Introducing-paragraph-level-rewrites
[voice]: https://support.grammarly.com/hc/en-us/articles/23153676821773-View-and-customize-your-voice-profile
[lt]: https://help.languagetool.org/hc/en-us/articles/39254502438551-What-features-are-available-in-the-new-premium-only-version-of-the-LanguageTool-browser-extension
[translate]: https://support.grammarly.com/hc/en-us/articles/27518589299725-Introducing-Grammarly-s-translation-feature
[strategy]: https://support.grammarly.com/hc/en-us/articles/25351804320397-Introducing-strategic-suggestions
