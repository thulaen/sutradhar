# 0129: Writing files and review exchange

A writer can bring an existing document into Sutradhar, review it
and prepare a protected copy for a named person. The writer can
choose whether that person may read it, return comments or propose
changes. Sutradhar shows the exact content any approved copy holds.

Spec 0121 owns the document and revisions. Specs 0101 and 0128 own
site-team permissions and writing policy. Spec 0100 owns bounded
file intake and output. Spec 0118 owns protected transport and
storage. This spec owns the writing document tree, interchange and
offline review flow. The existing site editor keeps its own apply
and publish route.

Example: Ravi imports a formatted DOCX. The preview names one
unsupported table shape and shows what would be lost. He accepts the
conversion and the original stays untouched. He seals a copy for a
named reviewer with the comment role and replay off. The reviewer
returns a signed response with one suggested wording. It appears as
inert feedback until Ravi confirms an exact local revision. The source
changes only then.

## Words used here

- **Document tree**: the ordered typed blocks and style runs a file
  becomes after parsing.
- **Conversion preview**: the exact losses, differences, destination
  and rights shown before a conversion is accepted.
- **Exchange grant**: the approval naming a package, revision,
  recipient and role.
- **Role**: view, comment or propose. Each is separate.
- **Review response**: a signed return of anchored comments,
  inert suggestions or typed edit proposals.
- **Handoff receipt**: the record of one delivered package.
- **Round trip**: export and re-import with exact positions kept.

## What this spec designs

| Feature | What the writer sees |
|---|---|
| F01 to F03 | A protected document library, editing and deliberate paste |
| F04 to F06 | Import, comparison, preserve or convert, and versioned export |
| F07 to F09 | Sealed copies by role, signed responses and exact local apply |

## Source outcomes and permitted routes

Use independent workflow and format designs. The primary sources
for the known outcomes are [document editing][grdocs],
[format preservation][grformats], [file export][grexport],
[mobile files][mobile], [sharing][share], [writing formats][pwa],
[native file handling][scriv] and [LanguageTool editor][lt].
Record exact source surfaces and plans as observed 20 November 2026.

| ID | Outcome | Local contract and fixture |
|---|---|---|
| F01 Document library | Create, rename, search, organise, duplicate, archive and recover protected versions | Recover a deleted draft still inside the approved retention window, then erase it and try restore. Ownership and site scope stay fixed |
| F02 Keyboard and formatting | Plain and styled text, headings 1 to 3, bold, italic, underline, links, bullets, numbers, checkboxes, quote, inline or block code, tables, images and nested sections. An accessible command menu changes a declared selection | Modify several mixed blocks and a table by keyboard. Reparse the same source and keep linked ranges. Undo preserves other sections. Images have alt text and never trigger fetch |
| F03 Paste or selection | Admit only the text or supported formatting the person deliberately chooses to bring in | Paste rich text, a styled list and an unsafe link. Show permitted styles and an exact preview. No ambient clipboard monitoring or authority over another app |
| F04 Import and compare | Read a chosen document, retain its original only under the chosen grant, parse supported structures and show limits and differences | Import a formatted original with an unsupported shape. Preview losses before commit. Refuse active content and do not replace the original |
| F05 Preserve or convert | Produce an exact local round-trip or an explicitly chosen converted copy, with the visible difference and file destination | Change one permitted text range and compare positions and formatting. An unsafe or unsupported feature is never silently dropped or returned active |
| F06 Versioned export | Export an admitted document or report, including approved Word, PDF and text routes. A PDF or printer needs its own release evidence under QR-039 | Cancel after a changed destination or source content. No raw fallback file. Repeat approved delivery exactly once or show the prior receipt |
| F07 Read, comment and propose | Send fixed recipient-scoped copies. A reader can see allowed text. A commenter can return signed feedback with optional suggested wording. A proposer can return a typed signed edit proposal | Give three recipients differing scopes. Refuse a viewer's feedback write. A commenter's wording appears as inert feedback and cannot replace text. The local source stays unchanged |
| F08 Review round trip | Import a permitted signed response and review anchors and a change comparison. Apply only an exact confirmed local revision | Bring back feedback for an older revision and a wrong-site draft. Unknown anchor needs review. No remote edit or auto-merge |
| F09 Package delegation | Reuse current ownership and grants for author choice of eligible publisher, reviewer or named custodian | Change reviewer or custodian during preview. A one-time package's recipient role does not grant ownership of a workspace or local management access |

File, managed service and device variants are separate. The
following public file outcomes require individual admission
and proof. Extensions are labels. Detect the actual bytes and
structure before trusting one.

| Formats | Direction and proof |
|---|---|
| Classic Grammarly DOC, DOCX, ODT and RTF | Separate same-format export for each admitted input. Preserve eligible structures under the classic profile |
| Classic Grammarly TXT and styled paste | Test explicit TXT-to-DOCX. Styled paste supports bold, italic, headings 1 and 2, lists and links where safe |
| Current Grammarly Docs | PDF and documented text routes for input; PDF and DOCX for output. Test known omissions for images, charts, graphs, citations, formulas, bookmarks, comments and columns. Show per-file losses before approval |
| Grammarly iPhone input | PDF, DOC, DOCX, ODT, TXT, RTF, Markdown, TextPack, TextBundle and OPML. Track other documented file variants separately. A scanned PDF or bundled file keeps its own proof |
| Grammarly iPhone output | PDF, RTF, TXT, Markdown, DOCX and HTML. Test each output separately from its input list |
| ProWritingAid text input | DOC, DOCX, ODT, TXT, TEXT, HTML, HTM and RTF as documented by host and plan. Keep differences between its web and desktop profiles |
| ProWritingAid Markdown input | .markdown, .md, .mdown, .mdtext, .mdtxt, .mdwn, .mkd and .mkdn, by documented profile. Use an admitted inert parser |
| ProWritingAid projects | SCRIV, SCRIVX and FDX input and approved copies. Preserve eligible section, scene and speaker structure. Record original Scrivener save-back and format retention as a separate DR-114 route |
| LanguageTool editor Word import | Preserve documented Word formatting and track native editor and Word add-on effects separately |
| Protected print and PDF | Admit input text and source page ranges with limits. Prove exact print and export effects separately under QR-039 |

The source matrix separates same-format round-trip, export to a
new format, and formatting-preserving versus text-only import.
No promise of layout fidelity is made until exact synthetic proof
exists per format, version and platform. A product integration
claim has its own row even where a file can carry the same text.

## Shared records and formats

| Record | Fields |
|---|---|
| Input manifest | Workspace, actor, format ID and version, pinned opened source handle and file identity, immutable byte digest, strategy, rights and consent epoch, enabled features, size limits, attached parts and expiry |
| Document tree | Revision, stable ordered typed blocks, headings, list and checkbox state and admitted assets. Style runs and references bind exact spans or source member and format IDs and their revision. Record what lacks a safe round-trip map |
| Conversion preview | Import manifest and target scope, original and intended tree hashes, admitted and omitted features, coverage, format version, opened destination anchor and filename or recipient, exact projection rules and current grant epoch |
| Exchange grant | Package ID, exact base revision and document scope, sender, named recipient and verified key fingerprint, view/comment/propose role, allowed fields, expiry, display copy and return authority |
| Review response | Package and revision, responder, signature, response ID, ordered anchored comments and inert suggestions or typed edit proposals, signed base bytes and exact affected ranges and scope |
| Handoff receipt | Exact payload and base digests, format, sender, recipient and purpose, approvals and epochs, operation identity and committed/unknown/refused state |

The tree stores source ranges using spec 0122 boundaries.
Code examples stay inert. Only document data can enter the tree.
Workers never follow package relationships or resolve external
templates on their own. A rewrite or review response cannot
modify the original source except through the current exact
approval.

## Local import and export

1. The writer selects the input, purpose and workspace. For an
   external file, check identity and allowed rights before parse.
   Reject unadmitted content with a clear reason. Use a bounded
   reader and encrypted ledgered work where a buffer must spill.
2. Parse with the exact format profile. Limit expanded bytes,
   expansion ratio, members, depth, table size and images. A parse
   that misses `writing.file.parse_deadline_s` stops and holds the
   import with SUT-5801. The original stays untouched. Refuse macros,
   active rules, program references, executable attachments and external
   relationships. Validate container structure and path identity
   before opening children. Reject path traversal, symlinks,
   reparse points and names that collide after admitted case
   or Unicode canonicalisation. Charge every parsed member and
   duplicate candidate to bounds. An unadmitted format cannot
   fall back to a less restrictive reader.
3. Compare the original and the document tree. Explain unsupported
   elements and provide an inspectable preview. The writer chooses
   whether to accept a permitted conversion. Work from the exact
   opened file and immutable digest, or an encrypted snapshot of
   those bytes. Recheck source identity and bytes at acceptance.
   An input changed after preview is refused. Preserve the
   incoming file. A future Scrivener save-back route must prove
   exact authority, locked and concurrently changed project
   handling and no write outside the approved tree.
4. Before export, seal the exact document and render profile. Check
   byte and layout evidence under the chosen output mode. Bind
   writer approval to the exact payload, format, destination and
   current rights. Recheck the pinned destination anchor and
   path identity before an atomic publish. Refuse path changes,
   aliases and unintended replacement. Recipient packages use
   only the selected projection. Rebuild authorised structure
   deliberately and verify no hidden text, comments, metadata,
   relationship or unselected attachment travelled with it.
   Output only through the approved 0118 endpoint. Keep managed
   staging encrypted. Refuse untracked cache, spill or spool
   paths and protected clipboard plaintext.
5. Keep recovery points, report receipts and erase requests in
   the ledger. Recovery preserves the current restrictions. A
   deleted document or revocation cannot reappear through import
   retry, pending export or a previous backup.

## Offline review

Use a signed recipient-sealed package. The approval names the
exact revision, attachment and excerpt digests, scope and role.
Keep reading, commenting and proposing separate. The admitted
recipient client requires the correct decryption key and verifies
expiry against a trustworthy local clock before opening or
returning a review. Copies still under the sender's control
and queued deliveries follow revocation. Record a delivered
copy separately. Departed bytes and copies the recipient makes cannot
be recalled. Stop new local handoffs immediately on revocation.

The return package contains the response parent identity,
scope, grant epoch, responder identity and checked signing
purpose. Validate scope, role, response digest, expiry, exact
source ranges and live authority at import and apply. Deduplicate
by response identity and digest. Refuse a changed replay.
A still-authorised response to an older revision can use an
admitted exact edit map. Otherwise open authorised feedback as
a manual placement preview. Expired or revoked responses are
refused. Comments and suggested wording use no edit grant.
Conflicting proposals need exact review under 0121. A reviewer cannot become
a site or
product owner through a package.

Hosted view/comment/edit links, register-by-email, transfer of
hosted ownership with demotion of the previous owner, live co-author
presence, cross-device sync,
deep host editing and sending via mail require separate rows
under DR-114. A delivered package supports deliberate offline
review. When it cannot deliver a hosted effect, keep that gap
visible. QR-122 records each affected host, file or role boundary.

## Data

- Records are those in "Shared records and formats": input manifest,
  document tree, conversion preview, exchange grant, review response
  and handoff receipt.
- The incoming file is preserved. Managed staging stays encrypted and
  recovery points, receipts and erase requests live in the ledger.
- A package holds only the selected projection. No hidden text,
  comment, metadata, relationship or unselected attachment travels
  with it.
- A deleted document or revocation cannot return through import
  retry, pending export or a backup restore.

## Settings

All keys and numbers below are proposed until QR-122 admits them.

| Setting | Type | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `writing.file.max_mib` | MiB | 50 (proposed) | 1 to 500 (proposed) | next import |
| `writing.file.max_members` | whole number | 2,000 (proposed) | 1 to 20,000 (proposed) | next import |
| `writing.file.max_depth` | whole number | 16 (proposed) | 1 to 64 (proposed) | next import |
| `writing.file.max_expanded_mib` | MiB | 200 (proposed) | 1 to 2,000 (proposed) | next import |
| `writing.file.max_expansion_ratio` | ratio of expanded to packed bytes | 20 (proposed) | 1 to 100 (proposed) | next import |
| `writing.file.parse_deadline_s` | seconds | 30 (proposed) | 1 to 120 (proposed) | next import |
| `writing.file.max_images` | whole number | 200 (proposed) | 0 to 2,000 (proposed) | next import |
| `writing.import.keep_original` (owner-only) | on or off | off (proposed) | two values | next import |
| `writing.library.keep_days` | days | 30 (proposed) | 1 to 365 (proposed) | next night |
| `writing.package.expiry_h` | hours | 168 (proposed) | 1 to 2,160 (proposed) | next package |
| `writing.package.default_role` | view, comment or propose | view (proposed) | three values | next package |

The setting `writing.import.keep_original` is owner-only. The CLI, MCP and
agents cannot change it. `writing.share.replay` is in spec 0127 and is
owner-only there.

No setting admits macros, active content, external relationships, a
less restrictive fallback reader or a raw fallback file.

## Failure codes

Codes SUT-5800 to SUT-5899 are proposed for this spec. Spec 0129
needs a decision register row to reserve them under DR-064.

| Code | Plain-English result |
|---|---|
| SUT-5800 | "This file type is not supported." Bytes or structure not admitted. |
| SUT-5801 | "This file is over the limit." Size, expansion, members, depth, table, images or parse time. |
| SUT-5802 | "This file path is not safe." Traversal, symlink, reparse point or name collision. |
| SUT-5803 | "The file or destination changed." Refuse and keep the original. |
| SUT-5804 | "Some content would be lost." Hold until the writer accepts the preview. |
| SUT-5805 | "This copy cannot be opened." Expired, revoked or wrong key. |
| SUT-5806 | "This response was refused." Scope, role, signature, digest or changed replay. |
| SUT-5807 | "This feedback needs manual placement." Older revision or unknown anchor. |
| SUT-5808 | "The export could not be confirmed." Show the prior receipt or repeat delivery exactly once. |
| SUT-5809 | "This document was deleted." Refuse restore or reappearance. |

## Security and privacy

- Detect actual bytes and structure. Extensions are labels only.
- Refuse macros, active rules, program references, executable
  attachments and external relationships. Code samples stay inert.
- Bound every parsed member, including duplicate candidates. An
  unadmitted format cannot fall back to a less restrictive reader.
- Bind approval to the exact payload, format, destination and rights.
  Recheck source identity, bytes and destination at acceptance.
- Packages are signed and recipient-sealed. The recipient needs the
  right key and a trustworthy clock. Copies already delivered cannot
  be recalled and have a separate receipt.
- A reviewer cannot become a site or product owner. Comments and
  suggestions use no edit grant.
- No ambient clipboard monitoring. No fetch from images or links.

## Requirements

| Requirement | Acceptance |
|---|---|
| REQ-0129-MATRIX | IT-0129-MATRIX: omit a documented format/profile, permission or external effect from a synthetic coverage record. Block release coverage and preserve the outstanding scope |
| REQ-0129-IMPORT | E2E-0129-IMPORT: exercise empty, one, exact limits and over-limit admitted containers, wrong format, nested active content, symlink or reparse point, path escape and case and Unicode collision. Change source file or destination path and expire approval. No original edit or unsupported render |
| REQ-0129-FIDELITY | UT-0129-FIDELITY and BDD-0129-FIDELITY: check exact structured comparisons for each profile, locale and platform. Include styled paste, a loss preview, stable source positions after reparse, source profiles and denied implicit conversion. Prove no hidden text, metadata, comments or attachments in the selected output |
| REQ-0129-SHARE | IT-0129-SHARE: test each role, inert commenter suggestion, wrong key/site or key purpose, modified and replayed archive, multiple recipients, untrusted clock, delayed handoff, revoked scope and wrong destination. Verify only the exact named package and no raw fallback |
| REQ-0129-REVIEW | BDD-0129-REVIEW: approve one exact response, reject conflicting or stale spans and observe unknown anchor handling and undo. A denied role cannot change any byte |
| REQ-0129-ERASE | E2E-0129-ERASE: cancel after sealing, delete a document, retry and restore. Remove managed derivatives and hold pending delivery. Keep already delivered receipts distinct |
| REQ-0129-SETTING | UT-0129-SETTING: for each setting, test the default, one value just inside and one just outside the range, and the stated effect time. Refuse any setting that admits active content or a raw fallback. Include UT-0129-EXPAND, which tests a zip bomb just over `writing.file.max_expanded_mib` and over the expansion ratio, and UT-0129-DEADLINE, which tests a slow parse held at the deadline |
| REQ-0129-FAIL | UT-0129-FAIL: reach each SUT-5800 to SUT-5809 code once. Check the plain message, that no file content or path appears in it, and that no code is reused from another spec |

## How it is tested

Designed, not run. Use synthetic files and invented recipients.

- **IT-0129-MATRIX**: a coverage record missing a format, permission
  or external effect blocks release and keeps the scope.
- **E2E-0129-IMPORT**: containers at empty, one, exact limit and over,
  wrong format, active content, link tricks and collisions, with
  source or destination changes.
- **UT-0129-FIDELITY** and **BDD-0129-FIDELITY**: structured
  comparison per profile, locale and platform, styled paste, loss
  previews and no hidden output content.
- **IT-0129-SHARE**: each role, wrong key, replayed or modified
  archive, clock and revoked scope.
- **BDD-0129-REVIEW**: approve one exact response, reject stale or
  conflicting spans, handle unknown anchors and undo.
- **E2E-0129-ERASE**: cancel, delete, retry and restore, with pending
  delivery held and delivered receipts distinct.
- **UT-0129-SETTING** and **UT-0129-FAIL**: as in the requirements.

## Changes from the source

The source products edit and share documents through hosted services
and their own import and export profiles. Here one local document tree
and signed offline packages replace hosted links. Hosted view, comment
and edit links, register-by-email, ownership transfer, co-author
presence, sync, deep host editing and mail sending are separate DR-114
rows, held in QR-122. No layout fidelity is promised until synthetic
proof exists per format, version and platform. PDF and print wait for
QR-039.

[grdocs]: https://support.grammarly.com/hc/en-us/articles/38552281546765-Docs-Grammarly-s-new-AI-writing-surface
[grformats]: https://support.grammarly.com/hc/en-us/articles/115000091512-Preserve-text-formatting
[grexport]: https://support.grammarly.com/hc/en-us/articles/115000091352-Create-upload-and-download-your-documents
[mobile]: https://support.grammarly.com/hc/en-us/articles/4412638077709-How-to-create-or-edit-documents-on-iPhones
[share]: https://support.grammarly.com/hc/en-us/articles/40852724499853-Introducing-document-sharing-in-docs
[pwa]: https://help.prowritingaid.com/article/187-which-files-are-compatible-with-prowritingaid
[scriv]: https://help.prowritingaid.com/article/228-why-are-my-changes-not-saving-to-scrivener
[lt]: https://help.languagetool.org/hc/en-us/articles/39254517614999-What-is-the-LanguageTool-Editor
