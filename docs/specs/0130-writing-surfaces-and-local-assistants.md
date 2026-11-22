# 0130: Writing surfaces and local assistants

A writer can use Sutradhar's checks on the text they open in the
editor, run a saved writing assistant and bring a draft from
another tool. The assistant uses only the sources shown before
the run. It gives advice and proposals for the writer to review.

Spec 0121 owns revisions and approval. Specs 0122 to 0129 own
each writing action, report, policy and file boundary. This spec
owns the scope of local assistant routines and the route record for
each writing surface. The local MCP door and settings confirmations
stay with spec 0020.

Example: Asha saves an assistant named "Tidy headings" from the
form. It may check and propose, nothing more, and it starts inert.
She reviews its action, scope, trigger and data in the preview and
approves it. While she types, the routine takes the latest
committed revision and shows advice only. Later she pastes the
same instructions as plain text. The natural-language route
produces the same preview and needs the same approval. She dictates
a note with a live in-app gesture, and the cleanup is reviewed
apart from the plain transcript. When she locks her screen the
capture closes and the audio is erased. An imported template that
asks for a network action is refused.

## What it does

### Words used here

- **Surface**: a place where writing happens, such as the editor, a
  file, a browser field or a mobile app.
- **Route**: the recorded way one outcome reaches one surface, with
  its local substitute or its named owner decision.
- **Assistant template**: a saved, versioned set of instructions,
  allowed read-only actions and routines. It is data, not a grant.
- **Routine**: a template bound to a trigger, inputs and limits.
- **Input approval**: the exact selected source, purpose and expiry
  the author allows for one use.
- **Host route**: the record of one outside product surface and
  what Sutradhar does locally in its place.

### What this spec designs

| Feature | What the writer or program sees |
|---|---|
| Route record (S01 to S15) | One row per outcome and surface, with its local substitute or gap |
| Assistant templates and routines | A form or text request that gives one review preview |
| Routine run and stop | Manual, typing-time or scheduled advice that never edits |
| Dictation and formats | Reviewed transcript and reviewed number or unit changes |
| Check route over MCP | Typed, read-only check requests with exact 0122 offsets |

## Public outcomes and routes

Keep each role, input, surface and outside effect in the source
matrix. Reference outcomes were observed on 20 November 2026
through the public [assistant builder][builder], [sharing guide][agent-share],
[connectors][connectors], [keyboard guide][keyboard],
[dictation guide][dictation], [browser guide][browser],
[compatible hosts][pwa], [writer surfaces][lt] and [proofreading API][api].
Design original screens and rules.

| ID and outcome | Local route and required observable test |
|---|---|
| S01 Full writing editor | Provide the workspace in the version 1 desktop app. Test selection and whole-document checks, version review and keyboard access on Ubuntu 26.04, Debian 13 and Windows 11 |
| S02 Site editor | Use only admitted WordPress or XenForo staff and member workflows. Test saved or approved local snapshots and ordinary site moderation through their current native path. No browser callback to the owner's PC |
| S03 Outside desktop writing | Supply exact supported files or a deliberately selected local copy. Test word processor, notes and writing project examples separately. Record deep in-place work and ambient window context as separate gaps |
| S04 Outside browser writing | Offer local file or deliberate input into the protected workspace. Test web editor text and separately a restricted field. Record native extension, per-site restrictions and live inline suggestions on the outside page independently |
| S05 Mail, messaging and calendar | Draft from explicitly supplied context. Test a thread summary, reply draft, selected message explanation and calendar event proposal. Sending, scheduling and event monitoring have separate owner decisions |
| S06 Mobile writing | Use admitted file exchange for separately recorded mobile editor and project workflows. Test approved source and return copies without background capture or sync. Mobile apps, system keyboards, notifications and shortcuts remain distinct route rows |
| S07 Completion and number formats | Offer a reviewed local completion. Propose arithmetic, unit, number, ratio, phone, date and time display changes for supported locales. Do not execute formulas from a document. One exact gesture confirms and testable unknown values stay unchanged |
| S08 Dictation | Offer local transcription from an explicitly selected in-app microphone with an admitted live approval or approved audio file. Show exact draft and purpose. Review plain transcript and any cleanup separately. Test stop and permission loss, exact review and no retained audio after removal |
| S09 Local reading help | Search, translate, explain or summarise a deliberately supplied selection. Test read-only use on selected received text and distinguish the same actions on a draft. Capture of active pages or screens has separate authority |
| S10 Proofreading automation | Expose approved typed check requests and read-only results through protected local MCP. Prove 0122 offsets, scope and bounded limits. Track public, paid hosted and self-hosted HTTP service separately under DR-114 |
| S11 Assistant setup | Configure name, description, instructions, exact supported action list and routine schedules from a form or a natural-language suggestion that produces the same review preview. Both routes need the same owner approval |
| S12 Routine execution | Offer manual, approved desktop typing-time and approved local scheduled advice. Exercise three trigger paths, pause, cancel, expiry and replay. Inbox, external app and mention triggers have separate route rows |
| S13 Local directory and sharing | Save, find, copy and review versioned assistant templates. Send a protected signed read-only template or independent copy using 0129. Test both with a rejected foreign scope or embedded capability grant |
| S14 Assistance administration | Set contextual restrictions, permitted actions and available modes through 0128. Test disablement mid-run. A template cannot enlarge a setting, scope or linked identity |
| S15 Support and workshops | Show local instructions, diagnostics, help and authorised prepared material. Test a diagnostic package and a manual support handoff. Hosted support, human experts, live presence, community and event payment are independent outside outcomes |

The host evidence names each documented product surface. Record
browser, desktop OS, mail, office, chat, note, manuscript, native
site and mobile app cases individually, including partial support.
An app with no documented native surface is not inferred from a
vendor logo. A local file's readability cannot prove direct
integration. Mark every direct effect still needing a decision
under DR-114. Keep DR-028 and DR-076 effective. QR-123 holds the
exact equivalent and proof per host or outside action.

## Assistant data and permissions

| Record | Fields |
|---|---|
| Assistant template | Scope and owner, name, version, description, instruction fields interpreted as data, allowed local writing implementation and read-only effect IDs, routine IDs and grant and expiry references |
| Routine | Template and version, manual/local generation/local schedule trigger, scoped inputs, bounded policy conditions, rate and deadline limits, action ID and result destination within the workspace |
| Routine run | Actor and scope, exact template and input digest, current approval and dependency epochs, trigger identity and unique attempt key, scheduler receipt, expiry and result reference |
| Host route | Exact host/platform, outcome, public source and observation, required scope, local substitute or named owner decision, admissible effects and acceptance references |
| Input approval | Actor, workspace and document revisions, explicit selected source or device/audio file, purpose and available result effects, expiry and consent epoch |

A template uses specific admitted local implementations of answer,
explanation, check, translate and draft-for-review. Each carries
a read-only advice or proposal effect. A template action ID,
natural-language instruction or model result cannot call a shared
provider-enabled draft route or any settings, connector, network,
file, window, clipboard, shell or send action. Bind current
authenticated actor and exact source grants independently of the
template at every stage. Bind policy under 0128 and approval under
0121. Preview action, scope, triggers and data before importing a
new version. Signature verifies identity and bytes. The owner
grants current permission separately.

## Run and stop

1. Resolve the active action and author's admitted input. Check
   actor, scope, consent and output restriction before reading.
   A natural-language request produces a template proposal only.
   Hold template instructions in the authorised instruction slot.
   Treat documents, excerpts and connector-derived material as
   data. No automatic fallback to another model or tool.
2. Bind each routine to its approved inputs, effects, conditions,
   cadence and actor. A typing trigger takes the latest committed
   revision under spec 0122. Newer committed input supersedes any
   pending older revision. A schedule uses the next authorised
   interval and never scans or backfills earlier windows. Bind the
   trigger and retry key to scope, actor, exact revision and
   routine version. Restart cannot duplicate a trigger effect.
   Use bounded jobs under 0019 and 0078 and recheck consent and
   order against a pause or cancellation before commit.
3. Give the model its typed allowed inputs under 0080. Arm a
   microphone only through a live in-app gesture and admitted OS
   proof for the exact actor, device, draft and purpose. Scheduled,
   background, cross-app and MCP triggers cannot arm it. Show a
   capture indicator and stop control. Close capture on stop, lock,
   focus policy change, suspension, device or user change, grant
   loss, time budget, scope change or worker loss. An intentional
   stop can prepare a transcript for review. Cancel or revoke
   discards it. Protect audio and spills in temporary ledger work,
   close streams and erase audio on every terminal path. Keep no
   capture history. Preserve meaningful negation and self-correction.
   Give text cleanup its own review and no identity inference.
4. Validate results using each action's owning spec. Use bounded,
   typed arithmetic and admitted unit and locale rules. Each of
   arithmetic, unit, number, ratio, phone, date and time formatting
   has its own switch. An ambiguous value, overflow or malformed
   expression leaves the source unchanged and provides a reason.
   A free-form expression cannot execute. Exact review is required
   for a completion, shortcut insertion, revision or formatting.
   Native keyboard effects and automatic replacement have separate
   decisions under DR-114.
5. Recheck current actor, source, policy and exact approval epoch
   at display or copying under 0121 and 0118. If scope or bytes
   changed, refuse. Keep a bounded local receipt with identities
   and reasons, without raw text, audio or model prompt. Pause-all, expiry,
   member or owner revocation, removal
   and erasure stop queued results and protected copies. Restore
   obeys the current grants and deletion state.

Record received, sent and labelled mail, and mention and watched
channel message events as distinct outside triggers. Keep directory
subscriptions and linked updates separate from independent copies.
Hosted shared use, edit and manage under another person's
connected accounts have separate permission and identity effects.
A local template provides a read-only or independent version and
starts inert until its local approval and grants take effect.

The check route verifies the enrolled installation or approved
local launcher, authenticated channel and actor token/session under
0020 and 0118. Plain program streams show public help and status
only. Require a typed versioned selection, locale, policy and
size/rate/deadline bounds. Return exact 0122 finding offsets and
coverage. Refuse an unproved locale, over-limit batch or stale
grant. Local IPC supports an admitted check. A hosted or standalone
HTTP service needs its own authority and release evidence.

## Settings

All switches are per workspace, owner-controlled through spec
0020 confirmations, and off until the owner turns them on
(proposed). No setting widens a template's scope, an action list or
a linked identity (S14), and none enables a network, file, window,
clipboard, shell or send action from a template.

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `assist.format_arithmetic` | on or off | off (proposed) | two values | next request |
| `assist.format_unit` | on or off | off (proposed) | two values | next request |
| `assist.format_number` | on or off | off (proposed) | two values | next request |
| `assist.format_ratio` | on or off | off (proposed) | two values | next request |
| `assist.format_phone` | on or off | off (proposed) | two values | next request |
| `assist.format_date` | on or off | off (proposed) | two values | next request |
| `assist.format_time` | on or off | off (proposed) | two values | next request |
| `assist.typing_routine` | on or off | off (proposed) | two values | next committed revision |
| `assist.schedule_routine` | on or off | off (proposed) | two values | next interval |
| `assist.dictation` | on or off | off (proposed) | two values | next capture |
| `assist.mcp_check` | on or off | off (proposed) | two values | next request |
| `assist.capture_max_s` | seconds | 120 (proposed) | 10 to 600 (proposed) | next capture |
| `assist.input_approval_min` | minutes | 5 (proposed) | 1 to 60 (proposed) | next approval |

Other bounds are set by admission and measured under QR-123: the
check route's size, rate and deadline, and each routine's rate and
deadline. A value above its admitted bound is refused, never
clamped silently. Pause-all under 0128 overrides every switch.

## Failure codes

SUT-6600 route record missing for a host or surface; SUT-6601
template action not on the allowed list; SUT-6602 imported template
needs local approval; SUT-6603 template scope, signature or
embedded grant refused; SUT-6604 routine trigger superseded or
duplicate; SUT-6605 consent, scope or approval epoch stale; SUT-6606
capture refused or closed (arming route, device, lock, suspension,
budget or grant); SUT-6607 format change refused, source unchanged;
SUT-6608 check request refused (peer, actor, locale, size, rate or
deadline); SUT-6609 result refused at display or copy because scope
or bytes changed; SUT-6610 paused or revoked, queued result dropped.
Codes SUT-6600 to SUT-6619 are reserved for this spec (DR-130, under DR-064).

## Security and privacy

- Template instructions are data in the authorised instruction slot.
  Documents, excerpts and connector material are data, never
  instructions.
- A template, natural-language request or model result cannot call a
  settings, connector, network, file, window, clipboard, shell or
  send action, or a shared provider-enabled draft route.
- Actor and source grants are bound afresh at every stage. A
  signature proves identity and bytes, not permission.
- Audio is protected, closed on every terminal path and erased. No
  capture history is kept.
- Receipts keep identities and reasons, never raw text, audio or
  model prompt.
- The check route uses the protected local channel under 0020 and
  0118. It opens no network listener and holds no secret.

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| REQ-0130-ROUTE | The route record MUST map each host and surface exactly, omit any documented effect not admitted, and require separate admission for automatic edits, network clients, remote requests, live co-writing and third-party monitoring. | IT-0130-ROUTE |
| REQ-0130-TEMPLATE | A form and a text request MUST give the same review preview and approval. An imported template MUST start inert and carry no capture, permission or account, and MUST be refused if changed, forged or overbroad. | BDD-0130-TEMPLATE |
| REQ-0130-RUN | Routines MUST run only on approved inputs, supersede older revisions, never backfill, never duplicate a trigger effect after restart, and never edit or publish. | IT-0130-RUN |
| REQ-0130-INPUT | A microphone MUST be armed only by a live in-app gesture with admitted proof, close on every stop condition, and leave no audio, clipboard input or stale text. | E2E-0130-INPUT |
| REQ-0130-MCP | The check route MUST return exact 0122 offsets only to the enrolled peer and actor over the protected channel, within typed bounds, and MUST NOT confirm anything. | IT-0130-MCP |
| REQ-0130-PARITY | The same effects MUST be enforced on all version 1 release platforms, with no ambient access from a denied route. | E2E-0130-PARITY |
| REQ-0130-FORMAT | Each format switch MUST be enforced, malformed or ambiguous input MUST leave the source unchanged, and a disabled or unknown transformation MUST NOT edit. | UT-0130-FORMAT |
| REQ-0130-SETTINGS | The settings of the table MUST be enforced as stated, and an out-of-range value MUST be refused. | UT-0130-SETTINGS |
| REQ-0130-FAIL | Each refusal MUST carry its named code from the reserved range and no raw text, audio or prompt. | UT-0130-FAIL |

## How it is tested

Designed, not run.

- **IT-0130-ROUTE:** test the exact host and surface map and an
  explicitly omitted documented effect. Require separate admission
  for automatic edits, network clients, remote requests, live
  co-writing and third-party monitoring.
- **BDD-0130-TEMPLATE:** create an equivalent proposal via form and
  text. Review its data and effects. Import a changed, forged or
  overbroad template. No inherited capture, permissions or
  accounts. Try a shared drafting action that has a different
  provider route. Refuse use of an imported template until
  current local approval.
- **IT-0130-RUN:** test manual, typing-time and scheduled advice,
  superseded revision, delayed retry, restarted trigger, missing
  scope, corrupt input and a race between pause or revocation and
  commit. No stale result, automatic edit or publication.
- **E2E-0130-INPUT:** try schedule and MCP arming, stop versus
  cancel, device or user change, suspend, lock, revoked consent, a
  changed draft, audio bounds and a lost worker. Check no other
  audio or clipboard input, stale text or untracked copy.
- **IT-0130-MCP:** test exact offsets over the protected channel and
  a wrong peer or actor. Try public standard streams, a paused
  worker, a malformed format, over-limit size or rate, an
  unsupported locale, expired time, a wrong-site session and a
  settings request. No network listener, secret, external post or
  ability to confirm.
- **E2E-0130-PARITY:** enforce the same effects on all version 1
  release platforms. Prove no ambient access or output from a
  denied route, including recovery and portable run.
- **UT-0130-FORMAT:** exercise each switch, bounded arithmetic,
  malformed or over-limit input, incompatible units, ambiguous
  phone or date and exact acceptance. A disabled or unknown
  transformation cannot edit.
- **UT-0130-SETTINGS:** one value just inside and one just outside
  each range of the settings table, and each default.
- **UT-0130-FAIL:** trigger each named refusal and check its code,
  its range and that no raw text, audio or prompt is carried.

## Changes from the source

The public products document hosted agents, connectors that act on
other accounts, ambient window context, native keyboards, live
inline suggestions on outside pages and hosted support. This spec
keeps the observable outcome of each as a local route and records
every direct effect still needing a decision as a gap under DR-114
(QR-123). Templates start inert and are read-only. Hosted shared
use and edit or manage under another person's accounts are not
built.

QR-123 records each source variant and exact policy and platform
proof. Preserve unmatched hosted and automation outcomes as gaps
for the owner's separate approval.

[builder]: https://support.grammarly.com/hc/en-us/articles/47453002811021-Create-and-configure-agents-with-the-agent-builder
[agent-share]: https://support.grammarly.com/hc/en-us/articles/48322812077325-Share-and-publish-your-agent
[connectors]: https://support.grammarly.com/hc/en-us/articles/40642362241293-About-Superhuman-Go-agents-and-connectors
[keyboard]: https://support.grammarly.com/hc/en-us/articles/360041391992-Manage-your-keyboard-settings-in-Grammarly-for-iPhone
[dictation]: https://support.grammarly.com/hc/en-us/articles/46245126716941-Introducing-speech-to-text
[browser]: https://support.grammarly.com/hc/en-us/articles/115000091592-Grammarly-s-browser-extension-user-guide
[pwa]: https://help.prowritingaid.com/article/201-compatible-apps-websites-prowritingaid
[lt]: https://help.languagetool.org/hc/en-us/articles/39254499343383-Where-can-I-access-the-LanguageTool-Writing-Assistant
[api]: https://help.languagetool.org/hc/en-us/articles/39254488835095-Does-LanguageTool-offer-an-API
