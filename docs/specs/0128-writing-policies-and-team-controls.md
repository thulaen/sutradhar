# 0128: Writing policies and team controls

An editor can use a shared vocabulary and writing guide while
keeping personal preferences for their own work. The owner can
see which rules apply to a site or workspace, who may edit them,
and which writing actions are available there.

Spec 0121 owns writing workspace access and edits. Spec 0101 owns
site team membership, grants and role limits. Specs 0014 and 0020
own settings approval, changes and the local permission door.
Spec 0118 owns protected data and key custody. This spec owns
writing rule records, their application and local policy reporting.

Example: Dev owns a site with two editors. He adds a shared term
"Sutradhar" with its alias and a shared rule that expands an
abbreviation. One editor also keeps a personal rule that conflicts. The
preview shows the owner, precedence and reason for each rule, and the
shared restriction applies first. Dev disables generation for the
reader-notice group. A draft opened from a blocked host reaches no
check and no capture. A synthetic card number in an export is masked in
the preview and redacted as the signed plan says.

## Words used here

- **Policy revision**: one versioned, owner-approved set of ordered
  writing rules for a site or workspace.
- **Writing rule**: a bounded text predicate with an expected form,
  priority, severity and reason.
- **Group**: an owner-defined set of eligible accounts. A group is not
  a site grant.
- **Applicable context**: the verified app, host, device and label a
  request comes from.
- **Policy receipt**: the record of an action, the rules that won and
  the effect or refusal.
- **Insight snapshot**: a bounded group measure released under a
  privacy rule, with its denominators.
- **Snippet**: saved plain text that is inserted only after an exact
  preview.

## What this spec designs

| Feature | What the owner or editor sees |
|---|---|
| T01 to T07 | Personal and shared dictionaries, style guides, tones, snippets and terms |
| T04, T08, T12 | Preference policy, feature controls and group roles |
| T09 to T11, T13 | Confidential labels, sensitive data checks, context limits and re-authentication |
| T14 to T16 | Writing insights, use and cost figures, policy history |
| T17 | A map of enterprise outcomes to separate route rows |

## Source outcomes

Use independently authored rules, interaction and reporting. Record
the original surface and plan for each outcome observed on
20 November 2026. See the public [team controls][team],
[feature plans][plans], [usage analytics][analytics],
[term library][terms], [access controls][access],
[sensitive data guide][dlp], [session expiry guide][expiry] and
[key custody guide][keys].

| ID | Outcome to provide or verify | Acceptance |
|---|---|---|
| T01 Personal dictionary | Add, edit, remove, export and import word exceptions by writer and locale | Confirm an import diff for a synthetic lexicon. One exception cannot suppress another language or another writer's spelling result |
| T02 Shared dictionary | Manage authorised terms with separate use, add, edit, remove, import and export grants and exact change history | Compare staff and ineligible readers. Refuse duplicate or wrong-scope entries and a revocation mid-import |
| T03 Personal and shared style guides | Match original word, case, typography, abbreviation and formatting rules with optional proposal and explanation. Group applicable guides | Try conflicting personal and shared rules and an active shared rule with an exemption. Show rule owner, precedence and exception rationale |
| T04 Preference policy | Choose document and dialect defaults, suggestion categories, generation, advanced checks, recording and approved route visibility by eligible group | Try a denied group and a conflicting personal preference. A higher-precedence deny survives approved changes elsewhere |
| T05 Tone profiles | Define shared intended tone by use and group. Preview examples and choose which named profile applies | Compare a reader notice and a formal mail draft. Choosing tone does not change authority, claims or send anything |
| T06 Snippets | Save and find personal or shared plain-text reusable passages, including preview and deliberate insertion | Try two snippets with the same label. Review exact expanded text and refuse placeholder injection and use after a revoked grant |
| T07 Term library | Save definitions, aliases, purpose, permitted contacts and document references. Keep read-only term use and separate add, edit, delete, file or preset import and export grants | Resolve two aliases in a permitted text. Hide an out-of-scope reference. Offer a suggested new term for human review. Refuse automatic URL actions and ambient external highlighting |
| T08 Feature controls | Enable allowed assistive and generative actions globally or by local group on an admitted surface | Disable a feature during an active job. Previously authorised outputs follow the current grant. A token cannot approve a setting change |
| T09 Confidential work | Apply an admitted local confidentiality label or verified imported restriction before reading, staging or capturing content | Test public and confidential drafts with exact label evidence and a forged label. Fail closed for unknown admitted label integrity |
| T10 Sensitive data check | Before an allowed export or processing stage, detect email, web URLs, telephone, payment cards, US SSN, UK National Insurance, Canadian SIN and international bank account numbers. Allow a deliberate local test of a synthetic sample | Match one synthetic item per declared class, give a masked preview and block or redact according to the signed plan. A local scan cannot grant a recipient access |
| T11 Application, domain and client policy | Restrict admitted contexts by signed app identity, normalised host/subdomain policy or device role | Try an exact blocked host, a permitted sibling, subdomain mode, unverified host, disallowed app and wrong device. No capture or processing on denial |
| T12 Group and role administration | Bind writing capabilities to current 0101 grants, owner-defined groups and eligible scoped roles. Show applicable controls and history | Expire a role and revoke a group. An inherited local allow cannot create a site grant or owner setting authority |
| T13 Re-authentication and keys | Lock relevant views and work on expiry. Show current device and status. Renew through the admitted local or native identity proof. Reuse owner key custody, rotation and recovery | Try stale session, wrong actor, copied ciphertext and revoked device. Never reveal unwrapping keys to writing tools |
| T14 Writing insights | Offer personal progress and authorised group measures for correctness, clarity, inclusivity, style guide consistency and brand tone consistency. Count seen, accepted and unaddressed suggestions separately | Compare two exact reporting windows and show numerators, denominators, coverage and missing facts. Withhold small groups, individual text and private fields |
| T15 Use and cost | Show activation, eligible use by feature and cost-centre figures as distinct metrics. Export a permitted snapshot. Estimate returns only from supplied assumptions and matched before and after evidence | Separate measured activity from entered costs and an absence of benefit data. No invented return figure, live spend, employment scoring or provider upload |
| T16 Logging and review | Present approved writing policy history and redacted audit export through existing administrator controls | Compare before and after policy revisions. A normal member or agent cannot read audit records. Protected reports stay in the ledger |
| T17 Enterprise boundary | Map vendor SAML, SCIM, domain capture, managed external clients, external key custody, remote audit API and account invitation outcomes to separate route rows | A local session or encrypted backup cannot satisfy external identity, provisioning, remote capture, dashboard or key-custody requirements without a separate owner decision |

There is one owner management surface. Staff actions may use their
admitted native site roles or local file routes. Site grants and policy
groups are distinct and meet only in the current authorisation check.
Any additional remote bridge, provider, account login, connector or
ambient context collection needs its own decision under DR-114. A
known outside document label cannot grant read authority. Record
Gmail and Word label feeds, domain claims, managed device controls,
account join/leave effects, recurring external login, key providers and remote
support separately. QR-121 holds those boundaries and exact proof.

## Records and rule evaluation

| Record | Fields |
|---|---|
| Policy revision | Site or workspace scope, ID, actor and authority, ordered rules, supported modes and context IDs, allowed groups and version, expiry, digest |
| Writing rule | Stable ID, kind, language and orthography, allowed input category, bounded literal/token predicate, optional expected form, exemption endpoints, priority, severity, reason and version |
| Workspace dictionary | Scope and actor or named group, language, version, exact text entries and independent provenance. Use 0122 word-exception semantics |
| Tone and term | Scope and version, display description, taxonomy or aliases, signed allowed audiences, supporting examples or references and rights |
| Snippet | Scope, actor, version, typed bounded placeholders, plain-text fields, source rights and range policy |
| Applicable context | Site/workspace, actor, admitted client and device identity, verified host or native context, label and provenance, document revision and grant epoch |
| Group membership | Exact scope and actor, group and version, current site or local identity proof, effective time, expiry, assignment and revocation epoch |
| Policy receipt | Exact requested action and document, policy, authority and membership epochs, winning rule IDs, reason and effect or refusal |
| Insight snapshot | Scope and cohort, measurement definitions, time and denominator, privacy release ID, exact input digests and expiry |

Plain-text term, rule and snippet content stays data. Validate
syntax, Unicode ranges, locale and bounds before showing a diff.
Use bounded parsers under 0100. Imports cannot contain executable
rules or external links that run. Any approved custom runtime uses
spec 0104's existing boundary only after separate policy admission.
A dictionary exception list can affect spelling, never an access
decision. An expanded snippet is a new exact text proposal under
0121. A glossary contact or document reference needs independent
read permission and cannot become an automatic contact action.
This spec owns curated term and rule changes. Spec 0127 owns
claim and evidence search. Project a permitted term into that
search only with a typed use grant for its exact purpose and
revision. Deduplicate repeats and make withdrawal effective before
another lookup or result. Using a term grants no edit or export.

## Apply, revoke and report

1. Bind each policy preview to base revision, actor, scope,
   rules, membership, authority and exact effect. Use specs 0014,
   0020 and 0101 for owner-only delegation and confirmation. Keep
   settings that can widen confidentiality, capture, key access
   or egress outside token and role-template powers. A read-only
   import preview needs current owner approval for the approved
   replace or merge. Check the latest epoch at commit and record
   any resulting event in the same transaction.
2. Authenticate claim, app, installation, document revision,
   device identity and context before reading or staging text.
   Use a trusted native app identity, admitted normalised DNS
   host or explicitly enrolled local client proof as appropriate.
   Host checks use exact canonical name or exact label-bound
   descendants only when selected. A known imported label may
   tighten the current classification. A stale, ambiguous or
   forged label cannot widen it or establish authority.
3. Require the live Cedar grant, any required native site grant
   and applicable local permission and membership together.
   An explicit applicable denial from any group wins. Apply
   effective shared restrictions before enabled preferences or
   defaults. If two equal-precedence rules conflict, withhold
   their affected check and ask the owner to resolve the tie.
   Expired local sessions lock protected views and actions until
   the admitted local proof is renewed. Native actions need the
   current site sign-in proof. Jobs and results lose release
   authority on a relevant expired or revoked grant.
4. Check current authority, policy, classification, purpose and
   context before dispatch, durable commit, resume or handoff.
   Scan sensitive data in bounded memory before staging. Collapse
   overlapping matches to one protected range and keep raw matches
   out of persistent copies and logs. Show only masked preview
   facts allowed by the signed rule. Match the outgoing bytes to
   the exact approved redaction or refusal. Missing classes or
   partial scans block the route. A scan cannot authorise a use.
   Superseded policies cannot return through a queued job or
   restored settings.
5. Publish approved local views. Bind allowed consent events,
   versioned measure and coverage, contributor caps, membership
   and time windows. Activation uses admitted eligible accounts
   and a deliberate activation event. Feature use identifies its
   own eligible denominator. Distinguish suggestions seen,
   accepted and left unaddressed using one event identity.
   Compare before and after only for matched cohorts, definitions
   and complete windows with admitted membership history. Display
   measured counts and user-entered assumptions independently.
   A local baseline cannot satisfy peer or industry benchmarking
   without separately admitted comparable evidence. Prevent
   differencing under spec 0127. Exclude private reflections.
6. Confirm and release any permitted report only using 0118 and 0100
   for the exact recipient-bound copy and ledger. An owner-side
   style permission does not approve data export. Keep personal
   writing text, tokens and keys out of shared reports. A
   revoked grant stops pending copies and erasure invalidates
   managed versions and derivatives after restore.

## Data

- Records are those in "Records and rule evaluation": policy revision,
  writing rule, workspace dictionary, tone and term, snippet,
  applicable context, group membership, policy receipt and insight
  snapshot.
- Policy and membership records are scoped to a site or workspace and
  carry an epoch, so a revoke or expiry is seen at commit.
- Reports and exports stay in the 0118 ledger. Raw sensitive matches,
  personal writing text, tokens and keys are never stored in them.
- Audit records are readable only through existing administrator
  controls.

## Settings

All keys and numbers below are proposed until QR-121 admits them.

| Setting | Type | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `writing.policy.feature.<name>` | on or off, per group | off (proposed) | two values | next request |
| `writing.host.match_mode` | exact or subdomain | exact | two values | next request |
| `writing.sensitive.classes` | list of classes | all eight declared | owner only. All eight classes stay on. No list with fewer classes is accepted | next preview |
| `writing.session.expiry_min` | minutes | 30 (proposed) | 5 to 480 (proposed) | next sign-in |
| `writing.insight.min_cohort` | people | 5 (proposed) | 3 to 100 (proposed) | next release |
| `writing.rules.max_per_policy` | whole number | 500 (proposed) | 1 to 5,000 (proposed) | next preview |
| `writing.snippet.max_chars` | characters | 2,000 (proposed) | 1 to 20,000 (proposed) | next preview |

A protected setting that could widen confidentiality, capture, key
access or egress is changed only by the owner, never by a token or a
role template. No setting lets an inherited local allow create a
site grant, or lets a scan authorise a use.

## Failure codes

Codes SUT-5700 to SUT-5799 are proposed for this spec. Spec 0128
needs a decision register row to reserve them under DR-064.

| Code | Plain-English result |
|---|---|
| SUT-5700 | "This app, host or device is not allowed." No capture or processing. |
| SUT-5701 | "Two rules conflict." Withhold the check and ask the owner to resolve the tie. |
| SUT-5702 | "This policy has changed." Refuse the stale effect. |
| SUT-5703 | "This import was refused." Bad syntax, executable rule, external link or over a bound. |
| SUT-5704 | "The sensitive data check did not finish." Block the route. |
| SUT-5705 | "Sign in again to continue." Lock the view or action. |
| SUT-5706 | "This group view is withheld." Small group, differencing or no denominator. |
| SUT-5707 | "Only the owner can change this." Refuse a token or role change. |
| SUT-5708 | "This permission was withdrawn." Role expired or group revoked. |
| SUT-5709 | "This outside route is not approved." Hold it for a separate owner decision. |

## Security and privacy

- Rule, term and snippet text is data. Imports hold no executable
  rule or running link. A custom runtime needs separate admission
  under spec 0104.
- A forged, stale or ambiguous label cannot widen access. An imported
  label can only tighten the classification.
- An explicit denial from any group wins. Equal-precedence conflicts
  withhold the check.
- Sensitive matches are scanned in bounded memory, collapsed to one
  protected range and kept out of copies and logs. The outgoing bytes
  match the approved redaction or refusal.
- Insights withhold small groups, individual text and private fields.
  Differencing is refused under spec 0127. No cost or return figure is
  invented and nothing is uploaded to a provider.
- Keys are never shown to writing tools. Key custody stays with spec
  0118.

## Requirements

| Requirement | Acceptance |
|---|---|
| REQ-0128-RULES | UT-0128-RULES and BDD-0128-RULES: cover each rule source, variant, reason, denial, hierarchy tie, dialogue exemption, scoped term and stale policy. Apply a valid term or snippet only after an exact preview |
| REQ-0128-APPROVAL | IT-0128-APPROVAL: try a token, wrong owner, protected token change, edited payload, conflicting groups, stale base and revoke overtaking commit. Restore old settings and refuse a stale outbox effect. Replays return the exact effect and cannot make a second write |
| REQ-0128-CONTEXT | IT-0128-CONTEXT: compare admitted app and device or host, Unicode lookalike and sibling hosts, forged importer, stale confidentiality label and missing context. Lock expired local or native sessions until admitted revalidation. No denied context reaches checks, preview or capture |
| REQ-0128-SENSITIVE | E2E-0128-SENSITIVE: test each admitted sensitive class, overlapping matches, incomplete scans and dependency failure. Verify exact approved output bytes and the absence of unmasked staged data |
| REQ-0128-INSIGHTS | IT-0128-INSIGHTS: empty, small, eligible, changed and overlapping cohorts across dates and filters. Test membership history, opt-out, contribution bounds, absent baseline and explicit assumptions. Refuse disclosure by differencing, falsified cost or an absent measurement denominator |
| REQ-0128-PLATFORM | E2E-0128-PLATFORM: on the three version 1 platforms exercise all admitted policy surfaces, bounds, denied contexts and required failure cases. Prove no remote listener or bypass of 0118 and complete review proof for any admitted variant |
| REQ-0128-SETTING | UT-0128-SETTING: for each setting, test the default, one value just inside and one just outside the range, and the stated effect time. Refuse a token or role-template change to a protected setting |
| REQ-0128-FAIL | UT-0128-FAIL: reach each SUT-5700 to SUT-5709 code once. Check the plain message, that no raw match, text or key appears in it, and that no code is reused from another spec |

QR-121 requires stable settings keys, exact rule and event
limits, an admitted privacy release policy, named key and context
proof, and separate owner decisions for uncovered outside outcomes.

## How it is tested

Designed, not run. Use synthetic lexicons, invented hosts and
synthetic sensitive samples.

- **UT-0128-RULES** and **BDD-0128-RULES**: each rule source, variant,
  denial, tie, exemption, scoped term and stale policy, then apply
  after an exact preview.
- **IT-0128-APPROVAL**: token, wrong owner, edited payload, stale base,
  conflicting groups and revoke racing commit. Replays make no
  second write.
- **IT-0128-CONTEXT**: app, device and host cases, lookalike hosts,
  forged label and expired sessions. Denied contexts reach nothing.
- **E2E-0128-SENSITIVE**: each admitted class, overlaps, partial scans
  and dependency failure, with exact output bytes checked.
- **IT-0128-INSIGHTS**: empty, small, eligible, changed and
  overlapping cohorts, opt-out, bounds, absent baseline and
  differencing.
- **E2E-0128-PLATFORM**: all admitted surfaces and denied cases on the
  three version 1 platforms, with no remote listener.
- **UT-0128-SETTING** and **UT-0128-FAIL**: as in the requirements.

## Changes from the source

The source is a hosted team product with vendor sign-in, directory
sync, managed clients and a remote audit service. Here rules, groups,
reports and re-authentication run on one local owner surface under
specs 0014, 0020, 0101 and 0118. Vendor SAML, SCIM, domain capture,
managed clients, external key custody, remote audit and label feeds
are separate DR-114 rows, held in QR-121. Return-on-investment figures
come only from supplied assumptions and matched evidence.

[team]: https://support.grammarly.com/hc/en-us/sections/115000249011-Team-Features
[plans]: https://www.grammarly.com/plans
[analytics]: https://support.grammarly.com/hc/en-us/articles/32478363601549-Review-your-organization-s-Effective-Communication-Score
[terms]: https://support.grammarly.com/hc/en-us/articles/16664924710797-Add-terms-to-Knowledge-Share
[access]: https://support.grammarly.com/hc/en-us/sections/27479182375693-Configure-Access-Security
[dlp]: https://support.grammarly.com/hc/en-us/articles/27458670736525-Enable-Data-Loss-Prevention-DLP
[expiry]: https://support.grammarly.com/hc/en-us/articles/26230680413325-Session-Timeout-FAQs
[keys]: https://support.grammarly.com/hc/en-us/articles/27895143707405-Bring-Your-Own-Key-FAQs
