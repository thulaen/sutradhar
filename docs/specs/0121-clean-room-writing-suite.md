# 0121: Clean-room writing suite

The writing suite helps people plan, write, check and review their
own work. A writer can check one sentence, ask for feedback on a
chapter, or work through a full manuscript. Each proposed change
shows what would change. The writer chooses what to keep.

DR-112, DR-113 and DR-114 define the version 1 scope. The public
products describe the outcomes to cover. Sutradhar defines its own
algorithms, rules, scores, example text, screen layouts and wording.

## Words used here

- **Writing workspace**: the local place that holds writing projects.
  It is separate from every site.
- **Revision**: one immutable saved version of a document.
- **Check request**: one ask to examine chosen text with chosen checks.
- **Finding**: one piece of advice about a range of text, with a
  reason. It may carry a proposal.
- **Edit proposal**: an exact change that the writer can compare,
  apply or keep out. It is never a site write grant.
- **Apply receipt**: the record that one reviewed change was made.
- **Grapheme**: one character as a writer sees and edits it.
- **Coverage row**: one dated entry for one observable outcome in the
  coverage register.
- **Clean room**: working from public descriptions of outcomes only,
  with our own algorithms, rules, scores and wording.

## Ownership

| Behaviour | Owner | Shared contract |
|---|---|---|
| Writing project, analysis request, suggestion and release coverage | 0121 | Editorial Coach 0041, drafts 0052, budget 0078 |
| Feature 186 spelling and grammar for eligible site text | 0121 | Language and correction 0122, page eligibility 0044, permissions 0101 |
| Evidence that supports an SEO editorial tip | 0041 and 0042 | Reuse accepted writing findings where authorised |
| Query fan-out and citation rehearsal for search visibility | 0117 | Separate run, question and comparison semantics |
| Writing measurements and report views | 0123 | Shared revisions 0121, language and correction 0122 |
| Writing revision, translation and locally grounded drafts | 0124 | Shared revision approval 0121, site content drafting 0042 and 0052 |
| Story planning, critique and publication preparation | 0125 | Shared revisions 0121, report measures 0123, proposal drafting 0124 |
| Writing sessions, progress and local learning | 0126 | Workspace scope 0121, text counts 0123, story prompt content 0125 |
| Writing sources, citations, overlap and origin | 0127 | Shared revisions 0121, writing activity 0126, file intake 0100 |
| Writing policies and team controls | 0128 | Team site grants 0101, settings 0014 and 0020, protection 0118 |
| Writing document tree, file formats and protected review exchange | 0129 | Shared revisions 0121, file intake 0100, permissions 0101 and 0128, protection 0118 |
| Local writing surfaces and assistants | 0130 | Local MCP 0020, model admission 0080, policy 0128 and interchange 0129 |
| Link ranking and site analytics | 0027 and 0034 | Writing scores and grades supply neither path |

The site audit gives the writing suite only the requested eligible
text and its site, actor and source revision. Member checks are off
until the site's owner grants that purpose. Member permission must
also cover the exact text and purpose. Check both grants, or staff
eligibility where appropriate, at dispatch, result commit and use.
A public copy of a member post cannot replace either grant.
Expose an audit finding by reference to the writing result. Site
public response allowance is zero extra queries, time and bytes.

### F-0186 writing audit binding

`REQ-0121-STAFF-MEMBER-PAGES` and `REQ-0121-LANGUAGE` use
`DES-0121-W2-LOCAL-WRITING-AUDIT` and
`VAR-0121-W2-ADMITTED-WRITING-RANGE` for the exact 0044 saved audit
revision and 0122 pack, capability and text range. Keep the existing
site and interface language rules. The writing range can use another
admitted language or dialect under DR-113.

`FX-0121-W2-SPELLING-GRAMMAR` links
`IT-0121-STAFF-MEMBER-PAGES`, `UT-0121-LANGUAGE` and
`E2E-0122-OFFLINE`. Use synthetic eligible staff and member text
on each required platform. Revoke either required member grant
before dispatch, commit or display. A missing or withdrawn pack is
unavailable, separate from a clean result. Expose the Audit finding
by reference to the writing result. No vendor runtime, outbound work
or site edit follows. QR-114/115 hold unproved packs, hosts and bounds.

## Coverage and clean-room evidence

Read the [writing suite coverage register][coverage] for the source
outcomes, the owning specs and the evidence required for each route.

Use public functional descriptions. Capture a dated entry per
observable outcome. A coverage row has a stable ID, public source
and section, observation date, plan or surface, original one-sentence
outcome, named owning spec, supported variants, permitted data route,
requirement IDs and acceptance test IDs. Keep even the rows for routes
that need a separate approval or where the public description needs
clarification. Map equivalent vendor outcomes to one behaviour and
prove all stated user-visible variants. A receipt that verifies one
language, report, file type or host cannot stand for the others.

| Family | Outcomes to enumerate and design |
|---|---|
| Text correction | Spelling, grammar, punctuation, word choice, contextual confusion, consistency, vocabulary, and stricter optional checks |
| Language coverage | Detection/manual selection, all documented language and regional variants, script, direction, multilingual passages, and capabilities per language |
| Line editing | Clarity, tone, inclusive phrasing, concision, fluency, variety, context-aware word exploration and rewriting |
| Analysis reports | Statistics, combined views, style, structure, readability, document-wide repeats, nearby echoes, alliteration, clichés, overuse, lengths, transitions, diction, pronouns, homonyms, acronyms, dialogue, pace and sensory detail |
| Style and team policy | Personal and shared rules, dictionaries, reusable text, brand voice, team knowledge, quotation exemptions, settings controls and writing analytics |
| Planning and writing | Ideas, outlines, scene and character planning, document types, prompts, focus mode, sessions, counts, goals, pauses, placeholders, reflections, sprints and challenges |
| Story critique | Plot, character, chapter and manuscript feedback, plot beats, voice, pacing, setting, themes, and simulated reader reactions |
| Publication preparation | Audience and market fit, query letters, synopsis, pitch, resume and other document drafts |
| Reading and revision | Summaries, explanations, translation, text questions, rubric feedback, rehearsal of reader responses, review and rewrite |
| Attribution and originality | Provenance recording, optional authorship evidence, similarity evidence, source discovery, citation formatting and verification, and estimated origin signals |
| Editing surfaces | Local editor, native site workflows, maker-approved file interchange, browser, desktop, mail, office and mobile workflows |
| Collaboration and services | Review, comments, sharing, cross-device work, workshops, expert feedback, peer critique, live events, contest entry and networking |
| Managed deployments | Group and role management, sign-in and provisioning, domain capture, data loss controls, group and client policy, audit, key custody, session expiry, and cost and return reporting |

Use the public [LanguageTool plans][lt] and [language list][locales],
[Grammarly features][gr], [plans][plans], [languages][gr-lang] and
[agent guide][agents], and [ProWritingAid features][pwa],
[reports][reports], [languages][pwa-lang] and [plans][tiers].
Source pages can change. Save the source identity and observation
date in the coverage record, with an original statement of the
observed behaviour. Use additional primary support pages for limits
and variants. A reviewer checks the coverage against those pages.

Keep product assets and unpublished details out of the design inputs.
Examples, training evaluation and release fixtures use suitable
synthetic inputs. Check rights and provenance of every shipped
dictionary, model, language corpus, rule pack and licensed reference
collection separately. A rule or test stays attributable to its
independent derivation. Under spec 0076, classify public behaviour
descriptions as permitted observations and the new language rules
as original designs. Classify published standards and licensed
dependencies under their own rules. Where strict separation applies,
record distinct research and implementation identities, access,
functional-only transfer, attestation and independent comparison.
Reject copied assets, improper access, unexplained provenance and
unlicensed data. The suite has no LanguageTool runtime or Java
package dependency.

## Layout and workflow

The Writing entry opens a local project list. The project opens with
an outline and file list, the current document and a checks panel.
Focus mode hides the outline and checks without deleting state.
The checks panel switches between checks for the current selection,
whole-document reports and review proposals. Results jump to the
affected text. The keyboard can choose a document, inspect an
explanation and compare an edit without using colour or a pointer.

Show the selected text language near the document name. Allow explicit per-
range language selection after showing the detected ranges. British English is
the interface language. A command or check with incomplete language support
shows the affected range and the capability that needs it. Page language and
market analytics follow DR-081. Right-to-left content, combining characters
and mixed direction need their own cursor, span, selection and accessibility
tests. List the exact admitted languages and dialects by behaviour in [spec
0122](0122-multilingual-writing-checks.md). Freeze the union of the three
dated source lists. Record the language, script, region, dialect or
orthography, and the exact capabilities and limits evidenced for
that variant. For example, one source can list a language without
spelling correction while another covers it.

Show “Check selection”, “Check document” and “Review changes”.
Typing-time checks can be paused. Expensive whole-document work
shows scope and progress before leaving the document usable.
“Compare change” shows the original and proposed text. “Apply this
change” acts on those exact bytes only, with undo. “Keep original”
dismisses that suggestion. Any external share starts from an
explicit preview of the exact data and recipient.

## Shared records and authority

Use opaque IDs scoped by site or local writing workspace. A local
writing workspace cannot read site text without the site's separate
grant. Every revision is immutable. Content addresses refer to bytes
already admitted to that scope. UUIDs and preview hashes identify
records and never confer authority.

| Record | Required fields and rules |
|---|---|
| Document revision | Scope, document ID, revision ID, optional parent, canonical UTF-8 digest, block IDs, language ranges and source kind. Preserve import/source coordinates with a reversible offset map |
| Check request | Scope, actor, document revision, selected blocks or whole document, check IDs, language/policy/evidence revisions, approval epoch, deadline and replay key |
| Result | Request ID and exact input identities, status, completed and unavailable checks, engine and rules revisions, bounded findings and coverage |
| Finding | Stable rule ID and version, category, confidence class, explanation key, cited ranges and optional proposal. Record unavailable as a separate outcome |
| Range | Block ID and start-inclusive/end-exclusive UTF-8 byte offsets in the named revision. Both ends match whole grapheme boundaries. Convert to UI offsets through the map |
| Edit proposal | Source revision, ordered non-overlapping ranges, expected bytes digests, replacement text and proposed effect. Never a site write grant |
| Apply receipt | Actor, proposal ID, base and new revisions, idempotency key, exact reviewed effect and undo reference |
| Ledger record | Logical group, scope, class, encryption/key reference, dependency revisions, retention, state and erasure reference under specs 0030, 0032 and 0118 |

Rust owns validation, policy and writes. Tauri and Angular use the
protected local platform door. The scheduler grants model leases
through specs 0078 and 0080. Bind each result to permission, source,
rules, language and model revisions. Before display, apply or
export, recheck the authority and the revision. A stale result
cannot edit a newer draft. A repeated accepted request has the
same receipt and produces at most one effect. An undo writes a
new revision and leaves intervening edits intact or asks for review.

## Data and outside workflows

Check locally on admitted hardware and keep member consent, role
checks, protected transport and encrypted managed storage in force.
Use bounded parsing for user files and treat text and collaborator
comments as data. Keep raw text out of logs. Cache and temporary
work follow the same scope and ledger deletion rules as saved work.

Use DR-114 local equivalents for outside workflows. Design
recipient-bound encrypted review files and manual exchange, a local
library of admitted reference material, local session timers and
owner-supplied review or workshop materials. Keep review comments
distinct from approved edits. Show which collection a similarity
search covered. A match is evidence to inspect. Report attribution
events only when captured under the authorised recording mode.
Any estimate must show uncertainty and cannot make a misconduct or
identity verdict. Grade feedback uses the writer's named rubric and
remains a revision aid.

Record each gap in reach or effect, including live presence,
real human expert review, corpus reach, direct host integration and
cross-device changes. Track active-window or screen capture, global
clipboard access, browser and mail context, and external identity
provisioning separately. A local editor starts with no authority
over those sources or a remote identity service. An approved route
would need its own bounded, current source and scope grant.
A local substitute may cover part of an
outcome. Require a separate owner decision before counting a gap as
covered or adding any outside or privileged route. Keep DR-028 and
DR-076 and the transport, consent and site boundaries in force.
MCP checks the actor and scope and returns advice. It cannot accept
a proposed edit, publish, open capture or confirm a share.

## Settings

| Setting | Type | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `writing.member_checks` | on or off, per site | off | two values | next check request |
| `writing.typing_checks` | on or off, per editor | on (proposed) | two values | at once |
| `writing.focus_mode` | on or off, per project | off (proposed) | two values | at once |
| `writing.request_deadline_s` | seconds | 60 (proposed) | 1 to 300 (proposed) | next check request |

As-you-type checks keep a short fixed deadline of 2 seconds
(proposed), and `writing.request_deadline_s` does not change it.

The site's owner alone turns `writing.member_checks` on, and member
permission must still cover the exact text and purpose. No setting
adds a vendor runtime, a background read of the active window,
clipboard, browser or mail, a remote login, or an outside route. No
setting lets MCP accept an edit, publish, open capture or confirm a
share. No setting gives the site's public response any extra query,
time or bytes. Admitted sizes and budgets come from spec 0078 and
the language limits from spec 0122.

## Failure codes

Decision DR-130 records SUT-5000 to SUT-5099 for this spec. DR-130 was
confirmed by the owner on 21 November 2026.

| Code | Meaning |
|---|---|
| SUT-5000 | A grant, scope or permission is missing, withdrawn or wrong. The message is scope-safe and names no text or range |
| SUT-5001 | The revision or result is stale. Nothing is shown or applied |
| SUT-5002 | A proposal is refused: wrong expected bytes, overlap, protected range, replay mismatch or concurrent edit |
| SUT-5003 | Coverage is incomplete or a capability is unavailable. Never shown as a clean result |
| SUT-5004 | An export or share is refused: wrong recipient, stale preview, revoked grant or corrupt review file |
| SUT-5005 | An admitted size, budget or deadline limit was reached |
| SUT-5006 | A coverage row lacks its requirement, owner, test proof or gap decision, so the release gate blocks |
| SUT-5007 | An outside or privileged route has no owner decision and is refused |

Spec 0122 and spec 0123 refuse a scope or permission failure with
SUT-5000 of this spec.

## Security and privacy

- Check both grants (site owner purpose and member permission), or
  staff eligibility, at dispatch, result commit and use. A public copy
  of a member post replaces neither grant.
- The site audit passes only the requested eligible text and its site,
  actor and source revision. Audit findings are exposed by reference.
- A local workspace cannot read site text without the site's separate
  grant. Opaque IDs, UUIDs and preview hashes never confer authority.
- Keep raw text out of logs. Cache and temporary work follow the same
  scope and ledger deletion rules as saved work.
- Managed storage is encrypted under specs 0030, 0032 and 0118.
  Review files are bound to one recipient.
- Treat user files, text and collaborator comments as data. Embedded
  instructions are never followed.
- MCP returns advice only. It cannot accept an edit, publish, open
  capture or confirm a share.
- Activating the local editor never reads the active window,
  clipboard, browser or mail, and never opens a remote login.
- Examples, training evaluation and release fixtures are synthetic.
  Shipped data must pass the rights and provenance check of spec 0076.
- Writing scores and grades supply neither link-ranking nor analytics
  paths.

## Requirements

| Requirement | Acceptance |
|---|---|
| REQ-0121-STAFF-MEMBER-PAGES | IT-0121-STAFF-MEMBER-PAGES: on all three release platforms, check synthetic staff and member pages through 0044 and trace the result reference. Prove correct and incorrect cases, offline operation, no public-page footprint and no vendor runtime. Try each missing grant, both grants and withdrawal before dispatch, commit and use |
| REQ-0121-COVERAGE | IT-0121-COVERAGE: review every supported outcome and variant against dated public evidence. Refuse a missing requirement, owner, test proof or gap decision, a duplicate owner, an omitted documented outcome, or a local result passing for an unproved outside workflow |
| REQ-0121-LANGUAGE | UT-0121-LANGUAGE: test each declared variant with correct, incorrect and ambiguous examples. Exercise mixed language, bidirectional text, diacritics, edits near non-Latin span edges, an omitted documented locale or capability, and unavailable capabilities |
| REQ-0121-APPLY | IT-0121-APPLY: try new revision, multiple proposals, incorrect expected bytes, replay, cancel, loss of permission and concurrent edit. Only the exact confirmed change commits. Preview, MCP and analysis alone perform no write |
| REQ-0121-BOUND | E2E-0121-BOUND: on Ubuntu 26.04, Debian 13 and Windows 11, require the same visible outcomes. Check admitted sizes and budgets at minimum, maximum and just over; failed dependency, pressure, pause and loss of device; inspect ledger cleanup and erasure through restore |
| REQ-0121-EXPORT | IT-0121-EXPORT: try wrong recipient, stale preview, revoked grant, corrupt review file and replay. Preserve the read-only import preview, accepted recipient and exactly approved payload or refuse |
| REQ-0121-ISOLATION | E2E-0121-ISOLATION: watch analysis, import, export and retry under disabled outbound access. Check wrong-scope requests, embedded instructions and revoked source across pause and restore. Expect no onward copy, ranking input, credentials or stale result. Check that activating the local editor never reads the active window, clipboard, browser or mail in the background or opens remote login |
| REQ-0121-SETTINGS | UT-0121-SETTINGS: each setting at its default and each allowed value, with one value outside range refused. Member checks stay off until the owner grants the purpose. No setting reaches a forbidden route |
| REQ-0121-ERRORS | UT-0121-ERRORS: provoke each failure code once. The message names no text or range, and SUT-5000 is the only scope refusal other specs use |

Each test id above is one named suite under Bazel. IT is an integration
test, UT a unit test and E2E an end-to-end test on the three platforms.

Proof for each family covers exact algorithm and data admission,
accuracy and false positives, valid and invalid input, failures and
platform limits. The shared design cannot supply a release pass
without its per-behaviour contracts and tests. QR-114 holds the
coverage, bounds and open route decisions.

## How it is tested

Designed, not run.

- **Integration (IT-0121):** STAFF-MEMBER-PAGES, COVERAGE, APPLY and
  EXPORT, as described in the requirements table, with synthetic
  pages, proposals and review files.
- **Unit (UT-0121):** LANGUAGE for every declared variant. SETTINGS
  for each setting at its default, each allowed value and one value
  outside range. ERRORS for each failure code and its scope-safe
  wording.
- **End to end (E2E-0121):** BOUND and ISOLATION on Ubuntu 26.04,
  Debian 13 and Windows 11.
- Each family also needs its per-behaviour tests in the owning specs
  before the shared design can pass a release.

## Changes from the source

DR-111 first chose to ship LanguageTool for feature 186. DR-112
replaced it with a clean-room suite that covers the public outcomes
of LanguageTool Premium, Grammarly and ProWritingAid, and so there is
no LanguageTool runtime or Java dependency. DR-113 admits all
documented writing languages while the interface stays British
English. DR-114 turns outside workflows into local equivalents and
takes each gap to the owner. The vendors' wording, assets and
thresholds are not copied. QR-114 holds the open coverage decisions.

[lt]: https://languagetool.org/premium
[locales]: https://help.languagetool.org/hc/en-us/articles/39254526141463-What-languages-does-LanguageTool-support
[gr]: https://www.grammarly.com/features
[plans]: https://www.grammarly.com/plans
[gr-lang]: https://support.grammarly.com/hc/en-us/articles/115000090971-Does-Grammarly-support-languages-other-than-English
[agents]: https://support.grammarly.com/hc/en-us/sections/38552317725325
[pwa]: https://prowritingaid.com/features
[reports]: https://prowritingaid.com/features/writing-reports
[pwa-lang]: https://help.prowritingaid.com/article/61-which-languages-does-prowritingaid-support
[tiers]: https://prowritingaid.com/pricing
[coverage]: ../planning/writing-suite-coverage.md
