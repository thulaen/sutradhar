# 0098: Sponsors, CRM and community growth

Keep sponsor decisions, prepare newsletters and community plans,
and compare supplied campaign evidence. DR-088 permits local
checks, supplied files and drafts for manual use. Spec 0100 still
owns the approved notices and ticket routes. This spec adds no
marketing sender, direct CRM connection or advertising account.

Spec 0096 owns public distribution packs, manual social hand-off
and syndication. Spec 0097 owns asset rights and surveys. Spec 0094
owns reports and the report master. Spec 0101 owns staff grants,
rights notes and advertising checks. Spec 0034 owns visit data.

Fictional example. A forum owner records a sponsor agreement and
its renewal date. Staff prepare a newsletter with a rights-checked
contributor quote. They review a local copy and send it in their own
tool. The owner later imports permitted totals and compares them
with the campaign's stated costs.

## Words used here

- **CRM** means a record of contacts, agreements and their history.
- **Consent receipt** means evidence of permission for a stated use.
- **Rate card** means a dated list of sponsor prices and terms.
- **Campaign code** means a shared label without a person ID.
- **ROI** means return on investment, compared with stated cost.

## What this spec designs

| Feature | Version 1 treatment | Requirement |
|---|---|---|
| 436 Sponsor and partner CRM | Site-scoped contacts and agreements | REQ-0098-CRM |
| 437 Sponsorship pipeline and forecasts | Evidence-backed ranges by pipeline stage | REQ-0098-SPONSOR |
| 438 Sponsor reports | Approved projection of delivery evidence | REQ-0098-SPONSOR |
| 439 Sponsor media kit generator | Local draft using verified evidence and licensed assets | REQ-0098-SPONSOR |
| 440 Rate card manager | Versioned rates, currency and dates | REQ-0098-CRM |
| 441 Contract renewal reminders | Local tasks with due dates | REQ-0098-CRM |
| 442 Sponsored content approvals | Exact proposal with disclosure and rights checks | REQ-0098-SAFETY |
| 443 Newsletter builder | Local issue draft and safe copy | REQ-0098-NEWSLETTER |
| 444 Newsletter content suggestions | Sourced eligible content choices | REQ-0098-NEWSLETTER |
| 445 Subscriber lists with consent | Local purpose-specific consent records | REQ-0098-CONSENT |
| 446 Click analytics without pixels | Admitted first-party campaign aggregates | REQ-0098-MEASURE |
| 447 Lead forms | WordPress-only admitted site route | REQ-0098-SITE |
| 448 Welcome sequences | Local plans and drafts for manual sending | REQ-0098-NEWSLETTER |
| 449 Sponsor enquiry lead scoring | Local evidence-based organisation score | REQ-0098-SPONSOR |
| 450 Member lifecycle segments | Protected totals only | REQ-0098-CONSENT |
| 451 Re-engagement campaigns | Manual plan with consent and preference checks | REQ-0098-NEWSLETTER |
| 452 Email preferences respected | Existing preferences and local suppression | REQ-0098-CONSENT |
| 453 Email authentication checks | Check supplied domain evidence | REQ-0098-AUTH |
| 454 Partner and sponsor ticketing | Local requests and admitted Fix list route | REQ-0098-CRM |
| 455 CRM sync | Review and reconcile owner-supplied records | REQ-0098-CRM |
| 456 Campaign link builder | Approved page and common campaign label | REQ-0098-MEASURE |
| 457 Campaign return on investment | Labelled estimate with comparable inputs | REQ-0098-MEASURE |
| 656 Ambassador programme tracker | Local programme, tasks and evidence | REQ-0098-COMMUNITY |
| 657 Member invite links | Admitted site links with protected counts | REQ-0098-SITE |
| 658 Post reuse permission requests | Manual request and verified member grant | REQ-0098-CONSENT |
| 659 Contributor spotlight drafts | Draft from consented, attributed sources | REQ-0098-COMMUNITY |
| 660 Community events calendar | Local plan and admitted site display | REQ-0098-SITE |
| 661 Expert Q&A planner | Local plan, evidence and approvals | REQ-0098-COMMUNITY |
| 662 Newcomer first steps | Admitted public content with no visitor profiling | REQ-0098-SITE |
| 663 Sponsor Q&A planner | Local plan with disclosure review | REQ-0098-COMMUNITY |
| 664 Ads and organic overlap | Compare like supplied paid and own organic evidence | REQ-0098-PAID |
| 665 Ad landing page checks | Reuse site audit checks on named destinations | REQ-0098-PAID |
| 666 Paid versus organic share | Use known, comparable traffic populations | REQ-0098-PAID |
| 667 Shopping ads versus free listings | Supplied comparison for WooCommerce | REQ-0098-PAID |
| 672 Staff advocacy | Approved local share packs under spec 0096 | REQ-0098-COMMUNITY |
| 692 Press release drafting | Sourced draft with no wire distribution | REQ-0098-COMMUNITY |
| 694 Press coverage archive | Supplied, rights-checked coverage | REQ-0098-COMMUNITY |
| 696 Social post performance prediction | Conditional estimate from admitted local evidence | REQ-0098-MEASURE |
| 698 Mastodon verification links | Proposal for an approved own-site link | REQ-0098-SITE |
| 700 Print campaign QR codes | Local code for an approved campaign address | REQ-0098-MEASURE |
| 704 Sponsor engagement early warning | Local finding from comparable protected totals | REQ-0098-SPONSOR |
| 706 Brand safety check | Hold proposed placements with unsafe or unknown context | REQ-0098-SAFETY |
| 707 Email accessibility check | Local reading-order, text and asset checks | REQ-0098-NEWSLETTER |
| 708 Email list hygiene | Local suppression from admitted bounce evidence and held inactivity when evidence or profile is unproved | REQ-0098-CONSENT, REQ-0098-W2-INACTIVITY-HOLD |
| 709 Best newsletter send time | Compare supplied like-for-like observations | REQ-0098-MEASURE |
| 710 Newsletter subject line tests | Local plan and supplied comparative results | REQ-0098-MEASURE |
| 711 Newsletter referral programme | Local plan and protected counts | REQ-0098-COMMUNITY |
| 712 Unified sponsor and partner timeline | Versioned local and supplied events | REQ-0098-CRM |

## Sponsor and partner records

1. Keep organisations, contacts with a documented purpose,
   agreements, renewal dates and rate-card versions for one site.
   Record changes as dated events. A timeline combines approved
   local records and supplied events, with source and actor.
   A local renewal task has a stable key and requires no send.
2. The owner uses spec 0100 to import typed, bounded, authorised
   files. Stage a scrubbed preview. Match site, source, object ID
   and revision. Repeat identical inputs safely, refuse same-version
   conflicts atomically and require approval for a disputed merge.
   Fence cancelled and retried imports. No outside account receives
   changes. Use the local Fix list. External tickets need its
   existing spec 0100 approval and may send only admitted findings.
3. Show stage, amount, currency, assumptions and coverage for a
   sponsor forecast range. Spec 0066 owns the method. Show its
   version or unavailable. Separate currencies, estimates and
   measured results. An enquiry score uses declared organisation
   facts, named reasons and a versioned scale. Unknown stays
   unknown. No protected trait, profile or score-triggered contact.
4. Reports and kits use spec 0094's safe projection, spec 0032's
   recipient checks and licensed assets from spec 0097. Recheck
   rights at hand-off. Omit contacts, rates, terms, private text and
   revealing totals. Delivery stays manual. Sponsor warnings need
   comparable protected counts, scope and an admitted rule.
5. Hold sponsored proposals when rights, disclosure or placement
   context are missing. Use spec 0101's advertising checks and
   relevant local safety evidence. Keep a reason and review path.
   Approval binds the exact placement, version, expiry and asset
   rights. Changed page context requires review again. Sponsor
   approval cannot grant access or publish anything.

### Sponsor evidence, agreements and review (436–442)

A sponsor record has site, organisation ID, revision, purpose,
rights and evidence source. CRM contact use requires an independent documented
purpose,
field allowlist, current role, expiry and source retention. A
native sponsor submission under 0101 grants no CRM reuse. Link
agreement, offer, statement or task with exact local IDs. Never
infer the same organisation from a similar name or domain.
An import attests rights, source namespace, object ID, immutable
revision and digest. Preview a scrubbed diff, then compare every
expected record and grant generation. Commit the entire import
and one logical receipt together. Equal revision and bytes
replay. Conflict or stale record refuses the whole batch.
A disputed merge previews inputs, survivor, retained fields and
permissions. Require exact owner approval and current inputs.
Commit the result with opaque tombstones for absorbed aliases.
Old, withdrawn and suppressed inputs cannot restore contacts or
widen their use. Conditional rollback also checks current versions
and keeps later valid changes. Apply source expiry and 0032
erasure on claim, access, export, replay and restore.

An opportunity binds site, organisation, stage, amount in the
minor unit, currency, expected close date and exact input rights.
Report entered amount, signed commitment and observed receipt as
separate measures. Never infer payment from an imported agreement.
A forecast pins as-of time and an explicit close horizon up to
90 days. Use an admitted 0066 profile for exact site, stage
and currency. Freeze comparable prior cohorts with stage at
the pre-outcome cutoff. Use eligible decisions from the past
365 days and retain undecided and missing cases as coverage
gaps. No amended post-outcome stage or duplicate opportunity.
Require at least 20 valid won/lost decisions and proved
profile calibration. Show entered sums and limits otherwise.
Apply the admitted probability to open eligible amounts,
excluding observed receipts and signed commitments from the
scenario. Sum by currency in exact fixed decimal, round once
half-even to that currency's minor unit, and refuse overflow.
Show assumptions, coverage, scenario values and signed method.
Only a validated interval can display interval bounds.
A forecast causes no contact, contract or site change.

An immutable rate-card revision names item, quantity or period,
commercial terms, price, currency and effective dates. Preserve
an exact accepted offer's old card even after an update. Current
selection uses one canonical item, unit and currency, and
half-open UTC effective interval with its original zone.
Reserve it atomically with version approval and offer acceptance.
Overlapping approved intervals require an exact reviewed
resolution. If no current unambiguous price exists, leave blank. No automatic
currency conversion.
Keep personal contacts, private rates and terms out of exports.

A renewal task binds agreement revision, site zone and calendar
data version, the recorded renewal date, owner and local due slot.
Set the default to 09:00 local, 30 calendar days before the
recorded renewal date, or confirmed expiry when none is set.
An explicit approved reminder overrides that default. Resolve
UTC and DST under 0019. Edits supersede pending work atomically.
Revalidate after quiet hours, missed slot and restart. Expired,
completed or cancelled agreements send no notice. One 0099
logical renewal notice and receipt per agreement renewal cycle.
A due-date correction after delivery updates the record without
reissuing a notice. A new cycle has a new approved key. A reminder cannot renew
the agreement, contact a sponsor or send a contract.

A sponsor report projection of the exact protected 0094 master
binds exact sponsor, recipient purpose, campaign, native period,
units, proof, coverage and current rights. Separate promised,
measured and unknown delivery. Suppress other sponsors, private
terms, member data and revealing totals or differences. A media
kit needs approved public figures with evidence and dates, and
exact 0097 operation rights. Use the escaped 0100 file profile.
Bind master, profile, manifest, destination, audience, approval,
asset and output digests, expiry and grant generation. Recheck
current scope at atomic 0094 publication and manual hand-off.
Recover one exact receipt, without exposing a partial bundle.
Revocation removes managed copies and rejects stale retries.
Prior manual outputs retain a receipt without a recall claim.

A sponsored placement proposes exact native page and version,
placement range, paid relationship, required visible wording,
link treatment, asset revisions and licence proof. Follow the
approved 0101 advertising checks and source policy. A missing
or conflicting requirement gives a reason and review, without
a publishable effect. One approval binds exact proposal version,
grants, context, expiry and evidence. Material page changes void
it. An admitted send-to-review uses a signed 0021 conditional
effect with reviewer, native queue, revision, policy and asset
evidence. At the native review effect verify current 0101 role,
lease and evidence. A CRM contact cannot grant a native role.
Reconcile the same effect receipt on replay or a lost reply.
Publish needs a separate native action and approval. The
proposal confers neither publication nor disclosure bypass.

## Newsletters, consent and community

1. Pin issue, audience, source and asset revisions to approval.
   Check reading order, links, alternative text and contrast.
   Recheck rights before manual copy, export and 0096 archive
   intake. Only an approved public issue can feed the archive.
   Subscriber-only issues stay private.
2. Keep admitted purpose-specific contacts separate from aggregate
   lifecycle results. Record consent by site, source, purpose, time
   and expiry. Stale, conflicting or withdrawn permission and
   eligible bounce evidence suppress use. An old imported opt-in
   cannot revive a later unsubscribe. Preserve lawful suppression
   on restore. No addresses or person-level click IDs appear in
   plans or reports. No subscriber export or pixel.
3. Welcome, re-engagement and referral work stays as local plans.
   Recheck purpose and preferences at hand-off. The app cannot
   control manual sends. Staff make reuse requests in the site's
   admitted route. QR-055 supplies current claims only. QR-072 must
   admit a signed grant or revocation note binding site, authority,
   exact content revision, purpose, channel, credit and expiry.
   Recheck on preview, copy, export and replay in all consumers,
   including specs 0084 and 0096.
4. Ambassador, spotlight, press, staff advocacy and Q&A work stays
   as local plans and drafts. A person must confirm contributors'
   consent and published facts. Archive licensed press evidence,
   with source and date. Use spec 0096 for approved packs. No
   public claim or paid wire send follows from a local record.
5. Check dated supplied SPF, DKIM selector and DMARC policy
   evidence under QR-072. Distinguish absent, malformed, stale,
   unsupported and uncertain evidence. Show policy observations
   only. The check makes no DNS change or test send and proves
   neither authenticated mail nor deliverability.

## Site, paid and campaign evidence

1. WordPress general lead intake is separate from spec 0101's
   sponsored-content proposal. Reuse its admitted signed note
   route for bounded plain text where possible. The add-on checks
   purpose, consent, spam limits and retention. The service pulls
   signed notes under spec 0021. No attachments or followed links.
2. Invite links require current eligible site roles, expiry,
   revocation and atomic redemption in a separate site contract.
   The PC sees protected counts without person mappings. Event and
   newcomer pages and verification links use approved static
   content with no first-visit profile. A calendar uses the site
   zone and shows approved dates or cancellations. Each action needs
   platform-specific admission under QR-072, signed conditional
   writes, exact revision, moderation and live owner confirmation
   where required. Replay is harmless. Undo keeps later edits.
3. Use approved landing addresses and campaign labels. A print QR
   code carries that address, without redirects or person IDs.
   Reuse the joint campaign population of specs 0034 and 0096 for
   protected recorded landings only. An email click needs separate
   admitted evidence. Apply floors and anti-deduction checks before
   any output. Without a source, show "not measured".
4. Keep authorised paid and organic inputs by source, coverage,
   metric grain, dimensions and denominator. Compare like
   populations only. Mark missing evidence. Use spec 0047's audit
   and 0103's WooCommerce rules. Ads, feeds and bids have no action.
5. ROI is admitted attributed value minus comparable cost, divided
   by positive cost. Keep campaign, currency, window and basis
   fixed. Show projected and observed inputs apart. Refuse unknown,
   sparse or incompatible inputs. Timing and performance estimates
   need comparable exposure, uncertainty and a sample-selection
   caveat. Subject line plans reuse spec 0094's trial rules with
   supplied results. They select no automatic winner or send.

### Local newsletters, lead intake and campaign measures (443–457)

A newsletter revision pins site, audience purpose, sections,
subject and preview text, source versions, canonical links,
asset rights, credits, alternative text and approval. Show the
local reading order, link destination and contrast checks for
one admitted 0100 format. A profile verifies the output, including
plain text when supported. It loads no external image or pixel.
Suggestions come from approved content in the active site and
the local Coach. Each factual suggestion names the exact source.
Without safe wording, show a source card or manual draft. No
direct subscriber access from drafting. A changed source or
asset holds the hand-off and 0096 archive input. A public label
alone grants no release. Prove each included field independently
public or expressly approved as a public derivative. Escape preview
and output. Pin the exact output digest and recheck all source
and asset operation grants before file commit, clipboard, archive
review and replay. No mail effect.

Store documented subscriber purposes separately from CRM and
survey inputs. Consent binds exact site, protected subject,
purpose, channel, source, original time, expiry and generation.
Check global, site, member, channel and purpose suppression,
including eligible bounce evidence. A narrower grant cannot
override a current broader refusal. An unavailable native
preference is unknown. Use 0032's minimum lawful suppression
marker and exact generations. Deny stale imports and restore.
Fresh consent can only resolve the exact scope it authorises.
Recheck at every use, approval, export and manual hand-off, even
after replay. Templates and external copies receive no addresses.
Welcome and re-engagement steps state purpose, delay, exact
wording, current audience rule and own-site landing link.
These are draft instructions only. No address, copied recipient
list, background campaign or automatic mail.

Use an admitted site-native aggregate or approved supplied
site-level aggregate profile for lifecycle stages. No join
from anonymous 0034 visits to a member. Define new as less than
30 elapsed days since confirmed registration, quiet as older
with no covered consented native activity in the past 30 days,
and established as the remaining covered older population.
Require exact eligibility, source and closed reference UTC.
Missing registration or activity history stays unknown. Show counts and
definitions after disclosure across
filters, cohorts and exports. Individual records stay out of
the report and cannot become targeting lists or ranking input.

The WordPress lead form is a separate 0021 action with exact
native page, form and consent version, preapproved staff purpose,
current expiry and protected source. Its proposed profile allows
organisation name up to 120 scalars, contact address up to 254
bytes and plain enquiry up to 1,000 scalars, under an 8 KiB
request cap. Use an admitted same-site form-session protection
route for guests and members. Shared HTML caches carry no tokens.
The separate 128-bit random submission key expires after ten
minutes and binds site, page, form, consent revision and session.
Bind the canonical payload digest at first use. Atomically
order expiry, limits, pause and consent checks with answer,
single-use key and safe receipt. Exact live replay returns the
receipt; different payload conflicts. No attachments.

Encrypt native intake and the 0021 projection under 0118.
Transport alone grants nothing. Require exact current reviewer,
Cedar purpose, allowed fields, expiry and withdrawal generation
at pull and visibility. Return scoped receipt and erasure lineage.
Unknown current withdrawal holds reads. A 0032 tombstone fences
native and PC managed copies, replays and restores. No reuse for
CRM or newsletters. QR-072 admits exact caps and footprint before
enabling both supported WordPress versions. Start at 16 KiB per enquiry,
10 queries per request and 50 ms added time, all proposed until measured.
XenForo uses no lead route. Unrelated public pages do no extra work.

An enquiry score uses three documented organisation checks.
Give one point each for an exact eligible placement category,
a comparable approved budget meeting its rate card, and a
planned period within confirmed offer dates. Missing, private
or incompatible input gives no total. Show matched checks,
source scope and score version. A score cannot classify the
person, broaden access, trigger contact or enter ranking.

Partner tickets use existing Fix list state and current site
roles. To create an external ticket, require a genuine eligible
0048 finding and exact approved Jira, Linear or Asana action
under 0100. Export only safe finding fields and approved
destination. No contact, private note or reply thread. An uncertain
receipt follows SD-06 and cannot be retried blindly.
CRM reconciliation reads only owned, admitted HubSpot or
Salesforce file versions and schemas under 0100 and QR-141. Apply 436's
protected
preview, scope and merge rules. No polling or provider write.

Read only a dated owner-supplied TXT record set manifest for
approved domains and selectors. Pin exact queried name, source,
digest, authoritative coverage, observation UTC and TTL. Accept
up to 128 records and 8 KiB per reconstructed record. Concatenate
record strings in wire order. Preserve individual candidates.
Distinguish proved NXDOMAIN, no data and indeterminate coverage.
Use admitted freshness at most the supplied TTL or 24 hours,
whichever is lower. Check grammar under [SPF][spf], [DKIM][dkim]
and [DMARC][dmarc]. Report duplicates, invalid syntax, stale or
unsupported values. Empty DKIM key means revoked. Missing
DMARC exact-label evidence does not prove parent policy absent.
Missing referenced or ancestor evidence stays unknown. Strip
private mailboxes and reject private keys before preview.
No DNS access, test mail, live alignment or delivery claim.

Reuse the same 0096 campaign code and 0034 approved landing
measure. Validate the exact same-site path and shared label. Keep only
0096 allowlisted URL parts. Reject conflicting pre-existing codes
and unproved query values. Bind approval and copy receipt to the
current campaign and withdrawal generation. Never carry identifiers, an
outside or redirected target, pixels or raw referrer paths.
Create a local copy with an audit-safe receipt. Without admitted
measurement, keep the preview untagged and show “Not measured”.
Apply the 0096 F-0376 legacy and current scope checks to campaign
links, print QR codes and newsletter referrals. A restored tag
whose original proof cannot be verified stays held. A new
untagged preview can use a current approved public landing.
Obtain its own exact approval before a copy or print export.
Do not replace earlier approved bytes or restore attribution.
An own-site landing is distinct from an email-provider click.
A separately admitted owned file may supply a provider's dated
click aggregate. Pin issue, campaign, covered window, count unit,
deduplication and coverage, including whether scanners were
excluded. Unknown scanner coverage stays unknown. Do not use
opens or join providers to persons. Share 0096 disclosure history
and floors across overlapping counts, reports and exports.

For ROI use one campaign and non-overlapping closed native
window `[start,end)`, with zone, currency, allowed cost basis,
unique source receipts and attribution versions. Show
cost and value in integer minor units, label as projected
or observed, and never mix the two. Use attributed value
net of proved reversals linked to the original attributed row.
A correction replaces that row and issues a new window revision.
A duplicate receipt never adds cost or value. Hold reopened or
partial periods. Measured zero or negative net value is valid
with complete evidence. Refuse non-positive cost, unlike currency,
missing inputs or an unproved privacy floor. Recheck current
rights and attribution under the shared disclosure history. Compute (value
minus cost) divided by
cost as an exact ratio, then display a percentage rounded
half-even to one decimal place. Show input, basis and
uncertainty. It is a local measure, without a bid or send.

[spf]: https://www.rfc-editor.org/rfc/rfc7208.html
[dkim]: https://www.rfc-editor.org/info/rfc6376/
[dmarc]: https://www.rfc-editor.org/rfc/rfc9989.html

### Community programmes and site plans (656–663)

A local plan binds site, kind, ID, staff owner, revision, permitted
sources, next milestone and lifecycle state. A consented contributor
or sponsor has a protected native reference and an explicitly approved
public label. Recipient identity, member permissions and private
discussion remain separate. Check current 0020 actor and action,
0108 site scope, 0097 rights and 0032/0118 withdrawal and erasure
generations before preview, copy, export or a native hand-off.
Local actions add zero work to public pages.

- **656, ambassador programme.** Maintain one site programme with
  versioned enrolments and bounded task records. Each enrolment needs
  its exact purpose, consent revision and named staff owner. Tasks
  record type, due date and completion evidence. Enrolment conveys
  no permission for public attribution, quotes, images or recap.
  Obtain separate exact-use permission for each proposed use. Close, withdraw
  or replace conditionally
  against the current revision. Revocation cancels dependent uses.
  Display permitted progress and outstanding review in staff views.
  These records cannot bestow a site role or trigger a message.
- **657, member invite links.** A separately admitted site-native
  action supports create and revoke for the signed-in eligible
  issuer or authorised staff. Admit the exact per-platform rights
  and 0021 ReadAccountRight mapping before enabling PC actions.
  Bind site, issuer, approved same-site landing, policy revision,
  absolute and idle expiry, use allowance and invite generation.
  Use an opaque random handle, default one use, with any greater
  cap subject to QR-072. The site stores only its keyed digest
  under a purpose-specific site key. Any member mapping stays
  on site. The link only reaches normal landing or signup.
  GET, HEAD and preview never redeem. Eligible acceptance commits
  with the native successful signup or approved acceptance result
  and the invite-use receipt, or with an admitted durable protocol
  proving the same outcome. Bind the claim ID to that completion
  and reject duplicate native acceptors for the invite. At commit
  verify the effective grant, policy, expiry and remaining use
  under the same lock or equivalent conditional transaction.
  Return the recorded result on exact replay. A different request
  for a consumed claim gets a conflict. Failure or uncertainty
  cannot increment the protected total. Revocation and final-use
  claims are serialised at the site.
  Admit only a host whose application, proxy and cache evidence
  proves no raw handle in logs, analytics, cache or referrer.
  The landing has no third-party resources and sets
  `Referrer-Policy: no-referrer`.
  Strip the code from later navigation and keep only bounded
  expiring native claim state. No role elevation or visitor
  tracking. The PC pulls 0034 protected aggregate redemptions
  with no native IDs, handle digest or joins. Suppress sparse
  and reconstructable totals. Link opens use an independent
  admitted aggregate or stay unmeasured.
- **658, post reuse permission.** Prepare a staff request preview
  with exact post revision, excerpt boundary, channels, purpose,
  destination, credit, rights duration and withdrawal terms.
  The separately entitled native contributor reviews those exact
  fields and affirmatively accepts or declines them. Staff can
  initiate the admitted request. They cannot accept for the
  contributor. A separate native withdrawal records the next
  generation. Pull signed receipts through 0021. Bind site, actor,
  authority, exact source and use, request digest and generation.
  QR-055 rights evidence alone grants no consent. Deduplicate
  receipts and enforce generation order. If current contributor
  authority or source revision cannot be proved, hold the scope.
  Subsequent edit, redaction, expiry or withdrawal blocks 0084,
  0096 and other dependent uses on release. Restore applies
  current withdrawal and erasure first. Without a QR-072 native
  admission, provide only a local request preview. External copies
  follow the manual recall limits of the rights contract.
- **659, contributor spotlight.** Create an editable local draft
  from the current public attribution facts and explicitly cleared
  quote, image and link uses. Show exact source and credit per
  span or asset. Confirm separate consent for personal facts and
  named recognition. Bind the reviewed pack hash to current facts,
  asset rights and separate permissions before copy or export.
  Stale or uncertain facts stay in review. Withdrawal holds the
  whole affected pack and its copy/export paths. Staff
  decide any native publication through an already admitted route.
- **660, community events calendar.** A local event stores title,
  description, submitted local times, event and site zones, zone
  database revision, confirmed UTC start and end, state, audience,
  host consent and approved public venue or same-site link.
  Show the event and site zone. A skipped local time needs a
  replacement. A repeated time needs an explicit offset choice.
  Record the staff choice and require end after start under
  0019/0099. Only an admitted native static display may show
  the approved public fields. Keep capacity, registration and
  private details local. Replacement or cancellation retains
  the event ID and creates a new immutable revision. Withdraw
  the prior projection at the native authority change, including
  cached copies. Undo preserves any later staff edit.
  No outside calendar or booking connection.
- **661, expert Q&A.** Plan topic, date, confirmed expert
  participation and scope, question intake plan, moderator,
  source pack, consent and recap rights. Keep suggested questions
  separate from approved, public contributions. Authorise source
  quote, public name, image and recap independently. Bind the
  current approved plan and source revisions to the pack. Hold
  its release if any required authority changes. The plan cannot
  invite, book, stream, open a public form or publish.
- **662, newcomer first steps.** Allow an admitted static page
  or manual proposal with selected guest-visible same-site routes
  and site-approved help. Provide the same approved guide on the
  same route to every guest. A search arrival may find the same
  page through normal links or search or open it manually. No
  source-based redirect, analytics route, referrer path, tracking
  cookie, personalised variant or first-time detection. Check
  visibility and links at publication and on recompilation.
  If an approved destination becomes private, withdraw its link
  before further public serving through the admitted static route.
- **663, sponsor Q&A.** Reuse the local Q&A plan with sponsor,
  deal revision, purpose, contributor consent, explicit
  disclosure preview and sponsor/editor roles. Require separate
  staff editorial review under 0101 for findings, approved
  evidence and brand rights. Bind the released pack to the deal,
  contributor and editorial approvals separately. A commercial
  agreement confers no editorial privilege. Hold the pack for
  changed sponsorship,
  missing label, stale rights or revoked consent. No outside
  send, event booking or automatic publication.

QR-072 admits each native create, revoke, receipt, public read,
write and expiry path separately for WordPress and XenForo.
For 657, 658, 660 and 662 use 0021 signed service-initiated
messages where applicable, exact revision, native staff action
and live approval where 0020 requires it. Public payloads contain
only approved guest fields. Bind static snapshots to the content
hash, revision, policy generation and finite expiry. Verify
current site-local authority at serve or prove equivalent
synchronous invalidation through every cache. Hide affected
content wherever current authority cannot be proved. Exercise
cancel, private target, app offline, cache replay, policy
restore and later staff edits. Prove measured queries, time and
bytes for each route in both cold and warm cache states. Prove
zero extra work on unrelated pages and enforce site, actor and
visibility generations. Keep a stable effect ID and receipt for
conditional native writes. Reconcile uncertain receipts before
retry. Without admission, keep the plan and explain which
native action is unavailable. No site callback to the PC or
community input to live ranking.

#### Named campaign, preference and community fixtures

These planned spec 0076 checks use synthetic evidence, grants
and native receipts. QR-072, QR-141 and QR-160 hold admission.
DR-088 governs local review, supplied files and manual hand-off.

- `DES-0098-W2-CONSENTED-INTAKE` uses
  `VAR-0098-W2-NATIVE-AGGREGATE-STAGES` and
  `VAR-0098-W2-CURRENT-PREFERENCE`. `DES-0098-W2-LOCAL-NEWSLETTER`
  uses `VAR-0098-W2-MANUAL-RE-ENGAGEMENT`. Suppression and
  exact purpose apply before use, restore and hand-off.
- `DES-0098-W2-SUPPLIED-MAIL` uses
  `VAR-0098-W2-INERT-TXT-EVIDENCE` for local checks only.
  `DES-0098-W2-SPONSOR-RECORDS` uses
  `VAR-0098-W2-LOCAL-FIX-REQUEST` and `VAR-0098-W2-OWNED-CRM-DIFF`.
  A local request gives no outside ticket grant. The separate
  0048/0100 finding route still needs exact authority.
- `DES-0098-W2-CAMPAIGN-EVIDENCE` uses
  `VAR-0098-W2-CURRENT-LANDING-CODE` and
  `VAR-0098-W2-CLOSED-WINDOW-ROI`. Keep the 0034/0096
  population and disclosure history. Values are local measures
  and confer no billing, ad, export or sending permission.
- `DES-0098-W2-COMMUNITY-PLAN` uses
  `VAR-0098-W2-STAFF-PROGRAMME` and `VAR-0098-W2-PROTECTED-SPOTLIGHT`.
  Use only current rights on the exact approved source.
- `DES-0098-W2-COMMUNITY-NATIVE` uses
  `VAR-0098-W2-NATIVE-INVITE-CLAIM` and
  `VAR-0098-W2-EXACT-CONTRIBUTOR-USE` with the existing proof
  and platform gates. A request or invite conveys no role,
  licence, publication or new release authority. Hold unsupported
  native actions at their existing refusal or local preview.

| Feature | Requirement | Design and variant | Fixture and test | Named fixture assertion |
|---|---|---|---|---|
| F-0450 | REQ-0098-CONSENT | DES-0098-W2-CONSENTED-INTAKE, VAR-0098-W2-NATIVE-AGGREGATE-STAGES | FX-0098-W2-MEMBER-STAGES, BDD-0098-CONSENT | At one closed reference instant compare 29 and 30 elapsed days since registration. Use covered consented native history. Missing activity or registration evidence stays unknown. Reject anonymous-to-member joins and member exports, and suppress a revealing stage combination. |
| F-0451 | REQ-0098-NEWSLETTER | DES-0098-W2-LOCAL-NEWSLETTER, VAR-0098-W2-MANUAL-RE-ENGAGEMENT | FX-0098-W2-RE-ENGAGEMENT, IT-0098-NEWSLETTER | Build a local draft with a purpose and protected cohort. A stale preference, revoked purpose or unavailable current consent holds the step. No addresses, copied recipients, send or account effect. |
| F-0452 | REQ-0098-CONSENT | DES-0098-W2-CONSENTED-INTAKE, VAR-0098-W2-CURRENT-PREFERENCE | FX-0098-W2-PREFERENCE-FENCE, BDD-0098-CONSENT | Record a broader unsubscribe and a channel preference. Restore an older import and try a narrower or unrelated grant. Clear suppression only under exact current authority. Stale records cannot authorise hand-off. |
| F-0453 | REQ-0098-AUTH | DES-0098-W2-SUPPLIED-MAIL, VAR-0098-W2-INERT-TXT-EVIDENCE | FX-0098-W2-MAIL-AUTH-RECORDS, UT-0098-AUTH | Use bounded dated supplied TXT evidence. Distinguish duplicate SPF, an empty DKIM key and missing DMARC ancestor coverage. Reject private keys before preview. Expired evidence gives no live delivery inference. |
| F-0454 | REQ-0098-CRM | DES-0098-W2-SPONSOR-RECORDS, VAR-0098-W2-LOCAL-FIX-REQUEST | FX-0098-W2-PARTNER-TICKET, IT-0098-CRM | Replay one local partner request. Deny an external request with wrong scope or contact fields. A genuine eligible finding needs the separate exact 0100 action grant. A synthetic unknown receipt cannot start another remote effect. |
| F-0455 | REQ-0098-CRM | DES-0098-W2-SPONSOR-RECORDS, VAR-0098-W2-OWNED-CRM-DIFF | FX-0098-W2-CRM-RECONCILE, IT-0098-CRM | Compare two owned admitted file revisions. Conflict or stale rights refuses the whole import. A contested merge waits for exact current owner approval and cannot revive a withdrawn contact or sync a service. |
| F-0456 | REQ-0098-MEASURE | DES-0098-W2-CAMPAIGN-EVIDENCE, VAR-0098-W2-CURRENT-LANDING-CODE | FX-0098-W2-CAMPAIGN-URL, UT-0098-MEASURE | Use one approved same-site path and the shared code. Refuse redirect, collision, person identifier and unproved query values. Missing admission leaves an untagged preview and “Not measured”. Reuse 0096 FX-0096-W2-CAMPAIGN-LANDINGS and UT-0096-LINK for the restored tag. The old approval cannot copy a replacement untagged link. Any eligible copy is local. |
| F-0457 | REQ-0098-MEASURE | DES-0098-W2-CAMPAIGN-EVIDENCE, VAR-0098-W2-CLOSED-WINDOW-ROI | FX-0098-W2-CLOSED-ROI, UT-0098-MEASURE | Use one closed native period and currency. Value 150 and cost 100 gives 50.0%. A proved reversal replaces its original contribution. Zero cost, unsupported privacy or mixing measured with projected gives unknown. |
| F-0656 | REQ-0098-COMMUNITY | DES-0098-W2-COMMUNITY-PLAN, VAR-0098-W2-STAFF-PROGRAMME | FX-0098-W2-AMBASSADOR, BDD-0098-COMMUNITY | Pin exact enrolment consent, local staff owner and completion evidence. Revoke separate public credit and replay a prior update. Keep only current staff progress. No role, public name or message follows. |
| F-0657 | REQ-0098-SITE | DES-0098-W2-COMMUNITY-NATIVE, VAR-0098-W2-NATIVE-INVITE-CLAIM | FX-0098-W2-MEMBER-INVITE, IT-0098-SITE | GET, HEAD and preview consume nothing. Race final eligible claim and withdrawal under the native grant. Replay the exact committed receipt only. Missing host privacy proof holds the route. The intended link is the only handle output, and receipts and aggregate views carry no member IDs. |
| F-0658 | REQ-0098-CONSENT | DES-0098-W2-COMMUNITY-NATIVE, VAR-0098-W2-EXACT-CONTRIBUTOR-USE | FX-0098-W2-REUSE-GRANT, BDD-0098-CONSENT, IT-0098-SITE | Pin contributor, site, source revision, excerpt and exact use. Silence, staff proxy and expired authority grant nothing. With no admitted adapter show only local preview. Withdrawal blocks copy and dependent hand-off after restore. |
| F-0659 | REQ-0098-COMMUNITY | DES-0098-W2-COMMUNITY-PLAN, VAR-0098-W2-PROTECTED-SPOTLIGHT | FX-0098-W2-SPOTLIGHT, BDD-0098-COMMUNITY | Pin current public facts and exact quote, credit and image permissions. Withdraw one field before hand-off. Hold the affected pack and its outputs. No unproved personal claim or native publication. |

### Paid and organic comparison contracts (664–667)

These reports read owner-authorised files through 0100 and already
admitted own-site facts. The owner binds each file to the active
site and source account or property before opening it. Accept a
registered source profile with exact column mappings and units.
The profile binds `site_id, source_profile, contract_revision,
account_or_property_ref, window_start, window_end, zone,
grain, search_surface, device_scope, market_scope, attribution,
coverage, observed_at, source_digest, grant_revision`. Exact
dimensions and metric definitions must agree within each comparison.
Record defaults explicitly. No automatically chosen dimension or
implicit conversion between source clocks. Use fixed source ranges
and canonical row identity. Sort on profile, account/subscope,
report and metric revision, exact segment and interval, metric,
scope and entity before deduplicating. Reject conflicting keys.
Never accumulate totals with their own slices, cumulative rows or
reissued reports. A valid correction supersedes the exact older
scope and withdraws dependent results under 0032/0118. Every
derived value requires complete admitted dimensions and proved
coverage for its defined population. Partial inputs may show
permitted observed subtotals labelled with their coverage, never
a complete amount or derived metric. Use 0100 bounded protected staging for
exact join keys. Apply 0040 search-term privacy, 0034 shared
floors and anti-differencing, 0032 erasure and current action
rights. Raw identifiers, protected terms, linkable hashes and
unsupported fields stay out of previews, logs, exports, 0094
experiment plans and generated titles.

- **664, Google Search overlap.** Prefer the owner-supplied paired
  text-ad and organic report from an account proven to contain
  only the authorised site within the comparison scope. It may
  include organic data from all linked domains, with no filter by
  linked domain. Reject an unisolated multi-domain report. Paid
  language and location restrictions do not constrain organic.
  Show separately labelled amounts if those populations cannot
  be proved comparable. A second input is allowed only when an
  admitted profile proves matching query identity, surface,
  dates, dimension scope, account binding and metric grain.
  Keep paid ad impressions, paid clicks, organic queries,
  organic clicks and organic listings per query in native units.
  Do not substitute organic queries or listings for impressions. Admit report
  query terms, never
  the purchased keyword or broad-match phrase as a substitute.
  Match exact normalised query keys within the same source profile.
  Do not use fuzzy grouping for joins. Classify paid only, organic
  only, both or unknown. “Only” requires an explicit verified zero
  on the other side. Both means observed in both sides of the
  period, without an assertion of the same search or visitor.
  Show total cost only from a proved same-grain paid report with
  currency and definition. Average cost per click is its own
  measure. Absent rows stay unknown.
  Never assume organic history predates provider linkage.
  A review card can show observed paid and organic activity.
  A separate locally reviewed 0094 measurement proposal has no
  budget authority. The finding cannot infer that organic search
  replaces ads or estimate savings.
  [Google's paired report][search-paired] covers text ads only.
- **665, landing page checks.** Resolve a supplied paid destination
  to an approved own-site page through 0107, using an already
  observed and authorised redirect chain and target mapping.
  Validate effective base and each retained redirect and target
  against the current allowed routes. Redact the displayed
  destination separately. Exact query-bearing keys stay inside
  the allowed protected match. No host or target inferred from
  a tracking parameter.
  Report only current 0047/0115 evidence for status, canonical,
  robots, titles, headings, accessibility and responsive content.
  Reuse admitted page performance evidence under its exact
  profile and name the population. Do not fetch a link, follow
  tracking, open an advertising preview, or modify the ad or page.
  If the address is external, unsupported or ambiguous, withhold
  measurements and say which mapping is missing.
- **666, paid versus organic search traffic.** Use a single
  admitted session-scope acquisition report and versioned
  channel classification. The base profile uses supplied GA4
  Sessions with Session default channel group, Paid Search and
  Organic Search. Bind the view, property, web stream or proved
  hostname scope, reporting zone, dates, segment and filter.
  Exclude first-user and event-scoped channels, apps and other
  sites. Count only valid non-negative integral session values.
  A filtered, thresholded, rolled into (other) or missing cell
  cannot be treated as an observed zero. Exclude Paid Shopping, Organic
  Shopping and
  cross-network or unknown groups. Sum only disjoint compatible
  rows. Paid share = paid sessions / (paid + organic sessions).
  Organic share uses the same denominator. Show each count,
  the denominator, group definitions and admitted reporting
  information. An explicit zero and known positive other channel
  give the corresponding 0% and 100%. Missing, thresholded or
  zero-total input gives unknown share with the reason. Never combine Ads
  clicks,
  Search Console clicks and GA4 sessions. These shares describe
  measured search-channel sessions, with no causal substitution
  claim. The [GA4 channel definitions][session-channels] guide
  the dated profile qualification.
- **667, shopping ads and free product listings.** Require an
  admitted WooCommerce shop and an exact 0103 product or variant
  mapping to the provider offer for the full interval. A current
  SKU alone cannot identify a historic offer. Use comparable
  supplied Merchant Center Products traffic for online Shopping
  ads and online free listings. Bind channel, report profile
  revision, surface, account, offer, destination, market and
  interval. Refuse unseparated campaign totals, ambiguous Ads
  or Organic columns and windows spanning a changed definition.
  YouTube affiliate organic and expanded multi-channel Performance
  Max cannot stand in for the two channels. Pin any backfilled
  report revisions and invalidate earlier incompatible comparisons.
  Show clicks, impressions and clickthrough rate (CTR) separately.
  Divide compatible clicks by positive impressions, or admit the
  provider CTR only with the same grain and proved denominator.
  An absent or suppressed cell stays unknown. A displayed rate
  cannot fill an absent impression count. Recompute permitted
  roll-ups from disjoint counts. Never average rates.
  Keep listings performance separate from local paid or total
  shop sales and eligibility. Never mix impressions of distinct
  variants or count broad Performance Max as Shopping.
  Use the [Merchant Center product traffic guidance][merchant-ads]
  and its [product metrics update][merchant-metrics] to qualify
  the source profile. No feed upload, direct connection,
  bid, budget or stop action.

Show a table with file date, account scope, unit, window, known
coverage or missing evidence, and grant where needed. Recheck rights
before rendering, export or replay. Purge affected views on erasure
or revocation. Use existing 0094 uncertainty and 0078 file and job
bounds. A combination with an unproved population may remain side
by side as labelled amounts without a derived result. No route
adds provider spending, attribution tracking or ranking inputs.

Primary source paths checked on 20 November 2026.

[search-paired]: https://support.google.com/google-ads/answer/3097241
[session-channels]: https://support.google.com/analytics/answer/9756891
[merchant-metrics]: https://support.google.com/merchants/answer/17103877
[merchant-ads]: https://support.google.com/merchants/answer/13287767

#### Named community and paid comparison fixtures

These planned spec 0076 cases use synthetic receipts, approved
source profiles and exact site and actor scope. QR-160 and
QR-161 hold current native, rights, suppression and comparison
proof.

`DES-0098-W2-COMMUNITY-NATIVE` uses existing conditional
native static projections only where the specific host proves
current authority. Otherwise the result stays local. No new
recipient, referrer source or public form is admitted.
`DES-0098-W2-COMMUNITY-PLAN` uses local moderated evidence
packs. A changed deal, time, grant or source returns to review.
`DES-0098-W2-PAID-EVIDENCE` uses only bounded approved files
and existing own-site observations. Preserve each native unit,
current rights and shared suppression. No ad, billing, merchant
or site effect follows from a report.

| Feature | Requirement | Design and variant | Fixture and existing test boundary | Named fixture assertion |
|---|---|---|---|---|
| F-0660 | REQ-0098-SITE | DES-0098-W2-COMMUNITY-NATIVE, VAR-0098-W2-STATIC-EVENT-REVIEW | FX-0098-W2-EVENT-DISPLAY, IT-0098-SITE | Resolve one repeated local time to an explicit offset. Cancel a synthetic admitted static event while the PC is offline. A stale cache hides it, private registration and capacity stay local, and conditional undo preserves a later edit. |
| F-0661 | REQ-0098-COMMUNITY | DES-0098-W2-COMMUNITY-PLAN, VAR-0098-W2-EXACT-EXPERT-PACK | FX-0098-W2-EXPERT-PLAN, BDD-0098-COMMUNITY | Pin confirmed participation, moderator, schedule and current sources. Withdraw a recap, asset or quote permission and hold the affected pack. No invitation, public form, stream or send. |
| F-0662 | REQ-0098-SITE | DES-0098-W2-COMMUNITY-NATIVE, VAR-0098-W2-SAME-GUEST-GUIDE | FX-0098-W2-NEWCOMER-GUIDE, IT-0098-SITE | Compare search, direct and absent source context and show the same approved guide. When a target becomes private, a stale cache or restore cannot serve its link. No referrer lookup, personalised path or unrelated page work. |
| F-0663 | REQ-0098-COMMUNITY | DES-0098-W2-COMMUNITY-PLAN, VAR-0098-W2-DISCLOSED-SPONSOR-PACK | FX-0098-W2-SPONSOR-QA, BDD-0098-COMMUNITY | Pin sponsor deal, contributor consent, credit and separate staff disclosure approval. Amend the deal or revoke a role or right and hold local hand-off. No invitation, booking or publishing effect. |
| F-0664 | REQ-0098-PAID | DES-0098-W2-PAID-EVIDENCE, VAR-0098-W2-EXACT-SEARCH-QUERY | FX-0098-W2-PAID-ORGANIC-OVERLAP, UT-0098-PAID | Match the exact eligible paid and organic query across comparable approved file scopes. Only an explicit verified zero proves one-sided presence. Reject purchased-keyword substitution and unisolated domains. Keep cost separate from CPC. |
| F-0665 | REQ-0098-PAID | DES-0098-W2-PAID-EVIDENCE, VAR-0098-W2-OWN-CACHED-DESTINATION | FX-0098-W2-AD-LANDING-REVIEW, UT-0098-PAID | Use an admitted already observed own-site target and full redirect chain. Display a redacted address. An expired audit, external target or unproved mapping yields no comparison or fetch. |
| F-0666 | REQ-0098-PAID | DES-0098-W2-PAID-EVIDENCE, VAR-0098-W2-MATCHED-SESSION-BASE | FX-0098-W2-SEARCH-SESSION-SHARE, UT-0098-PAID | Use 25 Paid Search and 75 Organic Search sessions at one approved session grain. The shares are 25% and 75%. An explicit paid zero gives 0% and 100%. Exclude Shopping. A missing or thresholded cell or a zero total yields unknown. Never sum unlike sources. |
| F-0667 | REQ-0098-PAID | DES-0098-W2-PAID-EVIDENCE, VAR-0098-W2-EXACT-OFFER-SURFACE | FX-0098-W2-SHOPPING-CHANNELS, UT-0098-PAID | Compare exact dated offer-to-variant links for Shopping ads and free listings. A remap or mixed channel holds the row. Divide clicks by positive impressions at the exact grain. No guessed count or average of rates. |

### Staff sharing packs (672)

Staff prepare exact approved public text and permitted media in
the 0096 hand-off pack. Bind the site, native actor and current
role, post and pack revisions, destination profile, permitted
audience, intended use, credit and asset rights. Confirm the
named contributor's separate permission. The reviewer approves
the exact sanitised payload digest per 0096 effect. Recheck current
native account right, product action grant, approval and expiry
before release. Change, withdrawal, erasure and role loss hold
the whole pack at delivery, copy, export and restore. Apply a
conditional revision and separate protected hand-off receipt.
If a clipboard effect is uncertain, hold replay and show the
unknown result. A staff manual-share note is an observation
only, with no provider visibility claim. No external account,
credential, automatic send, public tracking or member data.

#### Wave 2 staff sharing hand-off binding

F-0672 · REQ-0098-COMMUNITY · DES-0098-W2-CURRENT-STAFF-SHARE.
QR-163 still owns exact site, approved public scope and staff and
contributor authority. The 0096 manual hand-off remains separate.

FX-0098-W2-CURRENT-STAFF-SHARE reuses BDD-0098-COMMUNITY and
IT-0096-APPROVAL. Prepare a synthetic approved public pack and change
channel, credit, post revision, staff role or contributor permission
after review. Withhold the exact invalidated pack before manual copy.
Replay an uncertain clipboard receipt without repeating the effect.
In a separate case, queue the approved file hand-off and withdraw
the contributor's exact permission before release. Restore the
earlier pack and pending intent under 0032. Through the existing
IT-0096-APPROVAL boundary, retry the local file hand-off and
separately try clipboard copy with the old approval. Both remain
held before effect. Leave the clipboard unchanged and expose no
partial or completed file. Restore cannot revive permission.
A manual-share note records the staff observation and gives no
provider delivery proof, account access or automatic send.

### Press drafts and supplied archive evidence (692 and 694)

Feature 692 prepares a local press release from approved owned
facts and current exact rights. Bind site, brief and draft revisions,
event and embargo timing, named public spokesperson or public
contact, factual claims, quotes and attachments to source records.
Each quote and media file needs its own credit and exact-use grant.
Flag unverified facts for correction. Do not invent a quote,
price, launch or partner statement. An embargo binds the exact
draft hash and approval to an authoritative UTC instant, with
the entered local time, zone, resolution and evidence retained.
Internal preview is available only in the authorised staff scope.
Block file, clipboard, export and all 0096 hand-off paths until
release time is proved under the current clock. Hold on clock
rollback, uncertain time, changed evidence or restore. The
embargo creates no job or send permission. Staff confirm a new
manual 0096 hand-off with current rights and exact digest after
the embargo. Hold an uncertain copy receipt for review. No mail
distribution, subscriber import, press wire or automatic publish.

Feature 694 archives owner-supplied public coverage through
the bounded 0100 file picker and admitted inert profile. Pin
publisher, scrubbed inert address, source revision and artefact
digest, observation and published dates, language, scope, rights
and expiry. Strip unsafe parameters, credentials, identifiers
and executable references before preview, spill or search.
Separate the permission to retain metadata, cite a link, keep
an excerpt, keep a full file or copy wording for hand-off.
Use exact 0097 asset and excerpt rights. Refuse raw contact
lists, member data and unadmitted fields. Repeating an exact
source is inert. Review corrections as new versions of the same
source. Changes and expiry fence dependent releases and withdraw
managed indexes and copies atomically by generation. Apply
0032/0118 to files, indexes, caches and restore. A cover note
can use only its approved reference or quote. No network search,
scrape, reciprocal publication or impression estimate.

### Social forecast, verification link and print pack (696, 698, 700)

These views pin exact site, source, version and purpose. Recheck
0020 actor, scope and approval, 0032 erasure, 0097 rights and 0118
storage before copy, export, replay or a native hand-off.

- **696, social post forecast.** Use the shared 0094 uncertainty
  and evidence rules. QR-168 and 0066 qualify the method below
  for each source separately. The first profile uses the final
  non-negative count of organic impressions during the first
  168 hours after a post. Bind one authorised supplied account,
  channel, post type and audience definition, and close the
  exact native observation window. Match stable provider post
  identity and revision. Apply verified corrections before
  deduplication. Exclude boosted, incomplete or overlapping
  records and label them. No final measurement beyond the
  source cut-off. Take the latest 80 eligible posts in time
  order with stable identity ties. Train on the oldest 30,
  calibrate on the next 30 and hold out the latest 20.
  Predict the median of the training counts, rounding half
  upwards. Use the 28th smallest absolute calibration error
  as the radius for a nominal 90% interval. Cap the lower
  end at zero. Require at least 18 of 20 held-out values
  inside the same interval and no defined source or audience
  drift. Also hold if the holdout median is outside the
  calibrated interval. This is a validation check, with no
  universal coverage guarantee. Never retune from held-out
  values. Show sample, dates, exclusions, observed holdout
  coverage and signed method version. Missing evidence,
  source change or failed admission gives no forecast.
  No promised engagement, causal uplift, connection or send.
- **698, Mastodon verification link.** Use owner-confirmed proof of
  the exact public profile and site page, with current role and
  approved purpose. The [Mastodon link guide][mastodon-me] describes
  placing the exact own-site HTTPS address in profile metadata
  and saving the profile after the site link is present.
  Require the own-site hostname to stay unchanged after IDN
  normalisation for the Mastodon 4.0+ profile. Propose a direct
  `a` or `link` element with the `me` relation and an `href`
  equal to the approved canonical HTTPS profile address.
  Use admitted current guest HTML evidence, including the
  documented 1 MB response limit profile. Require the link
  directly in the response without script, iframe or redirect.
  Check exact profile identifier and host against lookalikes.
  The owner manages their profile manually. Local markup proof
  cannot assert that Mastodon has verified the link. Treat any
  dated supplied provider result separately. No fetch of the
  profile, account sign-in or instance connection. Native output
  stays a manual proposal unless QR-072 admits the platform's
  exact signed 0021 operation. The effect adds or removes only
  the `me` token on one admitted static own-site surface. Bind
  actor, page, anchor identity, rights, expected revision,
  content and href hashes, staff/owner approval and expiry.
  Use a conditional write with one durable receipt. Revoke
  through the exact native route and prove cache invalidation
  or a safe hidden view when authority is unproved. Verify
  expiry, queue, restore, measured budget and guarded undo.
  Keep unrelated relation tokens and later staff edits.
- **700, print campaign QR code.** Without a current 0034/0096
  admitted campaign route, encode the untagged canonical
  HTTPS landing and show “Not measured”. With a current route,
  encode the exact approved same-site HTTPS landing and optional
  admitted 0096 tag. Bind site, landing, source generation,
  campaign purpose, target bytes and print layout. No shortener,
  redirect, credential, account ID, person ID or external service.
  Use an admitted QR library and inert high-contrast output.
  The print profile uses level M, at least four clear module
  widths on every side, a human-readable approved address and
  at least four printer dots per module at the declared final
  print resolution. See the [QR print guide][qr-margin].
  Before export, check final size, module size, bounds and file
  cap. An independent admitted offline decoder must recover
  exactly the approved bytes from a final-size synthetic test
  export. The owner approves size and URL. Recheck the address,
  tag and generation at each managed release. Revocation
  withdraws the asset and stops attribution for old prints.
  Paper already printed stays outside the app's control.
  Use one shared protected 0034/0096 count of recorded landings.
  A scan, typed or copied URL can reach that address. No
  inference of scan count, people or conversion.

Provider sources checked on 20 November 2026.

[mastodon-me]: https://docs.joinmastodon.org/user/profile/#link-verification
[qr-margin]: https://www.qrcode.com/en/howto/code.html/index.html

### Sponsor engagement and placement safeguards (704 and 706)

Feature 704 uses the exact admitted 0034 eligible engagement
rate for site, sponsor, section, placement-equivalence profile
and campaign. Use the current producer and source for numerator,
base and eligibility. Add no measure or event. Inside the
producer, use exact rational rates for six consecutive final
site-calendar weeks in the pinned zone. The first four form the
baseline and the last two are the tested weeks. At each closed
week advance by one. Require same metric, exposure, scope,
receipt generations and eligible cohort. Missing, incomplete,
overlapping or invalid weeks withhold release.

The proposed alert rule requires positive baseline median,
the mean of the two middle rates, and both tested rates at
most 80% of baseline. QR-169 qualifies floor, false alert
evidence and versioned rule. If the complete eligible check
fails the threshold, close the episode. Unknown suspends it
and cannot start or reset it. Keep episode start and version
under the exact site, sponsor, section, placement, campaign
and metric key. Commit the first eligible closed evaluation
and the 0032/0099 local notice intent atomically. Consecutive
qualifying weeks and exact replay share the same receipt.
A higher correction withdraws invalid evidence and can close
the episode only on a complete eligible check. It cannot
repeat the receipt, add a backdated notice or alter notice
time. Rights loss withdraws queued and current output.

Keep counts, exact rates and baseline inside the producer.
Release only permitted rate bands, scope and a safe local
message. If an alert, band, reason, correction or prior result
would reveal a sparse or inferable population, withhold it
under the common 0034 release history. Never infer causality.
No outbound message, billing or placement effect.

Feature 706 checks proposed sponsor placement and its immediately
visible thread context. Bind site, page and target revision,
exact slot, actual preceding and following visible thread
identities and revisions in every permitted render layout and
pagination or ordering state. Pin exact 0101 native moderation
evidence, approved suitability rule, disclosure and asset rights.
Bound scope and qualify query, time and bytes per render route. Review
guest-visible facts and staff moderation
labels only within the scope needed to protect placement. Keep
labels and reason private to authorised staff. For open, changed,
unsupported, restricted or uncertain context, hold the placement.
A current safe checklist and ordinary 0101 disclosure review
permit a proposal only. Approval cannot override a failed check.

At a native review or separate native publication of a managed
placement under 0021/0028, revalidate current slot, evidence,
revisions, actor, rights, policy, expiry and host parity. Use
a signed conditional effect with a durable receipt. A
host that cannot fence cached/public delivery against a later
safety or visibility change may qualify review only. A qualified
managed public slot checks current site authority and hides
its own sponsor content if eligibility cannot be proved.
Prove warm, cold and restored cache fencing and concurrent
native reclassification before admission. Keep thread visibility and
moderation separate. Require exact release and cache footprint
under QR-072. A standalone manual placement has no app control.
Surface a local urgent review for supported changed evidence.
Do not alter threads, add a global ad network or bypass review.

### Newsletter evidence and partner histories (707–712)

Use the exact site, issuer, source, payload and purpose scopes.
Apply 0020 actor and action checks, 0032 erasure and consent,
0100 safe file intake, 0108 isolation and 0118 storage.
These actions use local material and do not start delivery,
connect a channel, track individual mail or change ranking.

- **707, email accessibility.** Use a bounded admitted parser
  under 0078/0100. Treat every incoming fragment as untrusted.
  Preview the exact newsletter revision using the inert 0096
  view and eligible 0115 static facts. No script, navigation,
  tracking pixel or remote subresource. Apply [WCAG 2.2
  criteria][mail-wcag] 1.1.1 for images, 1.3.1 and 1.3.2 for
  structure and sequence, 1.4.3 for supported contrast, and
  2.4.4 for link purpose. Subject and plain-text fallback
  are separate local checks. Where styling or client behaviour
  is outside scope, mark the relevant finding unknown.
  Bind preview and approved hand-off to the same text digest
  and current asset rights. Show the exact check and coverage.
  No mail-client or legal conformance claim, send or auto-fix.
- **708, email list hygiene.** Read an admitted structured
  owner file with exact site, source account, protected recipient
  reference, purpose, delivery and event identity, revision and
  observation time. Never retain raw mail. Map only a proved final
  recipient-specific mailbox failure to a suppression candidate.
  For the first SMTP profile, a valid final per-recipient 5.1.1
  can qualify. Generic 5xx, temporary mailbox failure or policy,
  network, quota and system failures cannot. See the [enhanced
  mail status definitions][smtp-status]. A staff-authorised local
  suppression compares current revision and commits the reason,
  generation and safe receipt atomically. Deduplicate the exact
  source claim. A verified correction can clear only that bounce
  reason. It cannot restore other suppression, consent or a
  withdrawn subscription. Restore applies current fences first.
  Subscriber inactivity needs a separately admitted exact
  newsletter profile. Prove purpose, eligible complete issued
  exposures, measured outcome, follow-up cutoff and coverage.
  Label only “No measured activity in this period”. The existing
  aggregate own-site quiet measure cannot label a subscriber.
  Missing or unavailable evidence stays unknown. Use a scoped
  local review only. No inbox, identifier export or contact.
- **709, suitable send windows.** The first profile compares
  authoritatively observed send times for the same authorised
  supplied account, audience and issue type. Bind source and
  issue revisions and deduplicate stable issue identity.
  Retain UTC instant, local zone and rule version. Use
  half-open four-hour local clock bins within each weekday.
  An ambiguous supplied local time needs verified UTC offset.
  A nonexistent local timestamp is invalid. Use one fixed
  deduplicated outcome divided by eligible delivered exposures,
  computed after common completed follow-up. Define the
  separate send and delivery clocks. Exclude open pixels and
  unknown or overlapping cohorts. Under 0094/0066 qualify
  minimum support, issue-level clustered inference and
  simultaneous uncertainty across all prespecified bins.
  Name a preferred window only when its lower bound exceeds
  all competing upper bounds and the profile's threshold,
  with complete coverage and at least two supported bins.
  Otherwise show observations or unknown. No causal claim,
  schedule, send or account change.
- **710, newsletter subject line tests.** Prepare a local
  two-arm plan with subject-only difference, fixed eligibility,
  primary outcome, assignment unit, exposure rules, sample and
  time limits, guardrails and analysis. Use 0094 power, error
  budget, interim and final rules. Pin the exact variants,
  signed frozen prospective random allocation and denominator
  plan against the plan digest. Staff choose whether to run
  outside the app. Accept independently verified aggregate
  evidence for the exact plan, arms, dates, assignment,
  exposures and completed follow-up. Reject cross-arm or
  duplicate exposures and unplanned cohort or arm changes.
  Analyse as assigned. Follow the predeclared treatment of
  failed deliveries and missing outcomes. Keep arm support
  private where required. A comparable nonrandom sample
  gives observational evidence only. Otherwise show
  “Inconclusive”. No participant rows, opens, audience export,
  deployment selection, inbox read or send.
- **711, newsletter referrals.** Show a local referral plan
  with consent, purpose, campaign and exact own-site public
  destination. Use the 0096 campaign code only with an
  admitted 0034/0096 landing route. Otherwise keep the link
  untagged and show “Not measured”. Never encode a subscriber
  or member invite ID. A signup count needs a separately
  admitted native consented joint aggregate for the exact
  campaign, path, generation, purpose and complete attribution
  and follow-up window. Count one eligible native outcome ID
  once in the producer. Use the same anti-deduction ledger as
  recorded landings. Never join visits to addresses or infer
  signup from a landing, click or scan. Copy only under a
  current exact public approval and receipt. Revoke past-link
  attribution and hold pending output under the same generation
  fence, including replay. The app cannot recall outside
  copies. No programme activation, reward, per-person code,
  contact send or first-visit profile.
- **712, sponsor and partner timeline.** Admit only CRM records
  under the approved sponsor and partner scope, such as a
  reviewed agreement or renewal, authorised staff contact
  note, and permitted supplied campaign or delivery evidence.
  Pin site, relationship and organisation ID, source namespace
  and account, opaque event ID, type, revision, digest,
  original zone and time precision, occurrence and observation
  time, evidence, rights, purpose and audience. Keep planned,
  staff-noted and source-observed activity distinct. Sort by
  known occurrence time then stable source and ID. Group
  unknown times separately. Revisions increase per exact
  event identity. Reject the same revision with another
  digest. Exact replay is inert and a correction only replaces
  its own event. Never merge on words or date. Apply erasure
  and purpose-specific withholding before filters and exports.
  A withdrawal marker prevents access and old restores.
  Exclude denied material and person-level recipient actions.
  A stale or absent channel stays unknown. No mailbox or
  social connection, next-step send or inferred history.

Primary source sections checked on 20 November 2026.

[mail-wcag]: https://www.w3.org/TR/WCAG22/
[smtp-status]: https://www.rfc-editor.org/rfc/rfc3463.html#section-3.2

#### Named late Wave 2 sharing, evidence and review fixtures

These planned spec 0076 cases use synthetic receipts, rights,
public facts and sampled evidence. QR-167 through QR-170 hold
rights, source and exact profile admission.
A reviewed local hand-off is separate from any staff choice to
share it. A due time, code, receipt or forecast gives no send,
provider, billing or publishing authority.

- `DES-0098-W2-REVIEWED-SHARE` holds exact public approvals
  through 0096 or the existing local print export. Keep
  permitted evidence and source credit at each release.
- `DES-0098-W2-SUPPLIED-PARTNER-EVIDENCE` uses only current
  site-scoped supplied files or admitted local records.
  Preserve rights, generations and source lineage.
- `DES-0098-W2-MEASURE-REVIEW` uses exact approved populations
  and current suppression. Keep forecasts, protected native
  outcomes and separate supplied counts distinct. No new
  collection, causal conclusion or ranking input.
- `DES-0098-W2-NATIVE-REVIEW` can propose only the currently
  eligible slot or profile link. Existing separate 0021/0028
  effect approval and exact host and cache proof still govern.
  A held route stays at local review.
- `DES-0098-W2-LOCAL-NEWSLETTER` and
  `DES-0098-W2-CONSENTED-INTAKE` use the existing inert preview,
  local plans and exact current suppression. No sender or
  outside service gets a grant.

| Feature | Requirement | Design and variant | Fixture and test | Named fixture assertion |
|---|---|---|---|---|
| F-0692 | REQ-0098-COMMUNITY | DES-0098-W2-REVIEWED-SHARE, VAR-0098-W2-EMBARGO-LOCAL-REVIEW | FX-0098-W2-PRESS-DRAFT, BDD-0098-COMMUNITY, IT-0096-APPROVAL | Pin draft hash, authoritative time and exact quote and asset rights. Under an unproved or changed embargo keep authorised staff preview only. After the proved release instant require fresh exact manual approval. No scheduled send. |
| F-0694 | REQ-0098-COMMUNITY | DES-0098-W2-SUPPLIED-PARTNER-EVIDENCE, VAR-0098-W2-BOUNDED-COVERAGE-ARCHIVE | FX-0098-W2-PRESS-ARCHIVE, BDD-0098-COMMUNITY, E2E-0098-ISOLATION | Import an eligible supplied opaque artefact digest and permitted excerpt. Strip identifiers and unsafe reference parts. Replay is inert. A correction versions the source, and withdrawal closes managed indexes, derivatives and copy. |
| F-0696 | REQ-0098-MEASURE | DES-0098-W2-MEASURE-REVIEW, VAR-0098-W2-CALIBRATED-LOCAL-INTERVAL | FX-0098-W2-SOCIAL-FORECAST, UT-0098-MEASURE | At 80 final eligible observations split 30 train, 30 calibrate and 20 holdout in time order. Use the 28th absolute calibration error. Seventeen in range fails. Eighteen can qualify only with the median and drift checks. No retuning the holdout. |
| F-0698 | REQ-0098-SITE | DES-0098-W2-NATIVE-REVIEW, VAR-0098-W2-RECIPROCAL-LINK-PROPOSAL | FX-0098-W2-MASTODON-LINK, IT-0098-SITE | Compare exact own-site and approved HTTPS profile addresses with a lookalike, unproved role and active embed. Show only the eligible direct link proposal with the me relation. A separate native effect needs exact host proof. No account read or asserted verification. |
| F-0700 | REQ-0098-MEASURE | DES-0098-W2-REVIEWED-SHARE, VAR-0098-W2-EXACT-PRINT-TARGET | FX-0098-W2-PRINT-QR, UT-0098-MEASURE | Decode exact final-size local bytes and check print profile bounds. Missing campaign admission gives an untagged address and “Not measured”. A changed tag or rights generation holds managed output. Use 0096 FX-0096-W2-CAMPAIGN-LANDINGS for a plausible restored tag with mismatched proof. Withhold stale managed output. A separate untagged print preview needs exact size and URL approval. Landings prove neither scans nor conversions. |
| F-0704 | REQ-0098-SPONSOR | DES-0098-W2-MEASURE-REVIEW, VAR-0098-W2-PROTECTED-ENGAGEMENT-NOTICE | FX-0098-W2-SPONSOR-EPISODE, UT-0098-SPONSOR | Compare four closed baseline weeks and two complete tests at 80% of a positive median under the proposed profile. Adjacent qualifying windows reuse one notice. Missing week suspends the episode. Withdrawal holds output and gives no sponsor contact. |
| F-0706 | REQ-0098-SAFETY, REQ-0098-SITE | DES-0098-W2-NATIVE-REVIEW, VAR-0098-W2-ADJACENCY-PROPOSAL | FX-0098-W2-SLOT-SAFETY, BDD-0098-SAFETY, IT-0098-SITE | Bind exact slot and visible neighbours across admitted layouts. An unknown context, reclassified thread or stale cache holds the proposal or hides only the managed placement under its admitted gate. Approval cannot override a failed check. |
| F-0707 | REQ-0098-NEWSLETTER | DES-0098-W2-LOCAL-NEWSLETTER, VAR-0098-W2-INERT-ACCESSIBILITY | FX-0098-W2-NEWSLETTER-ACCESS, IT-0098-NEWSLETTER | Use exact newsletter text and admitted inert preview. Compare visible headings, missing alt, unresolved contrast and a plain-text mismatch. A planted reference or pixel never loads. An unknown client style gives no conformance claim. |
| F-0708 | REQ-0098-CONSENT, REQ-0098-W2-INACTIVITY-HOLD | DES-0098-W2-CONSENTED-INTAKE, VAR-0098-W2-PROVED-RECIPIENT-STATUS | FX-0098-W2-LOCAL-SUPPRESSION uses BDD-0098-CONSENT and BDD-0098-W2-INACTIVITY-HOLD | Use a supplied final recipient-specific 5.1.1 and generic or transient failures. Only the exact eligible final status supplies a suppression candidate. A verified correction clears only its reason. Separately offer an aggregate quiet-period count, supplied activity with missing exposure or follow-up evidence, and apparently complete activity evidence with no admitted newsletter profile. Try mismatched purpose and revoked consent on replay. The inactivity branch stays unknown and changes no suppression or preferences. Any independent current bounce suppression stays governed by its own evidence. No restored consent, inbox access, address export or contact. |
| F-0709 | REQ-0098-MEASURE | DES-0098-W2-MEASURE-REVIEW, VAR-0098-W2-MATCHED-DELIVERY-BINS | FX-0098-W2-SEND-WINDOW, UT-0098-MEASURE | Compare admitted half-open four-hour weekday bins after equal complete follow-up. Hold a missing or unresolved clock, sparse groups or overlapping interval bounds. Opens give no denominator and a preferred local window cannot schedule a send. |
| F-0710 | REQ-0098-MEASURE | DES-0098-W2-LOCAL-NEWSLETTER, VAR-0098-W2-FROZEN-TWO-ARM-PLAN | FX-0098-W2-SUBJECT-PLAN, UT-0098-MEASURE | Freeze two subject-only arms and exact prospective allocation. Reject changed body, unproved assignment, unplanned read and incomplete follow-up. An eligible nonrandom comparison remains observational. No deployment or send effect. |
| F-0711 | REQ-0098-COMMUNITY, REQ-0098-MEASURE | DES-0098-W2-MEASURE-REVIEW, VAR-0098-W2-PROTECTED-REFERRAL-POPULATION | FX-0098-W2-NEWSLETTER-REFERRAL, BDD-0098-COMMUNITY, UT-0098-MEASURE | Without a current landing route show an untagged link. Keep landings distinct from separately admitted exact joint signup outcomes. Reuse 0096 FX-0096-W2-CAMPAIGN-LANDINGS to refuse missing or mismatched legacy proof. No attributed count or substituted link follows from an old approval. Dedupe one eligible native outcome, withhold small cells and hold stale generations. No contact, person-level join or reward. |
| F-0712 | REQ-0098-CRM | DES-0098-W2-SUPPLIED-PARTNER-EVIDENCE, VAR-0098-W2-EXACT-EVENT-LINEAGE | FX-0098-W2-PARTNER-TIMELINE, IT-0098-CRM | Keep same-date distinct events, occurrence and observation time, and unknown times separate. Exact replay is inert. Conflicting same-version digest fails. Withdrawal fences restore, and a foreign sponsor or inbox gives no result. |

## Data, settings and limits

Use site-scoped storage under spec 0108 and spec 0032 for audit, withdrawal
and restore. Revoke access to evidence, dependent views, drafts, managed
copies and queues. Retain only lawful suppression evidence. Recheck
eligibility before replay or manual hand-off. External copies cannot be
recalled.

Use specs 0078 and 0100 for jobs and files. Advice and imports add
zero public-page work. QR-072 admits separate measurable query,
time and byte budgets before any public route. Values are proposed.

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `community.record_limit` | active records per site | 1,000 | 1 to 10,000 | next save |
| `community.evidence_days` | days from admission | 90 | 1 to 365 | next retention pass |
| `community.active_plans` | open plans per site | 20 | 1 to 100 | next save |

## Failure codes

DR-091 reserves SUT-2700 to SUT-2799. SUT-2700 means missing
evidence. SUT-2701 means import refused. SUT-2702 means consent,
rights or preferences blocked use. SUT-2703 means site route
unavailable. SUT-2704 means a limit was reached. SUT-2705 means
conflicting versions. Actions use spec 0020's permission response.

## Security and privacy

Enforce the active site and actor grant. Keep personal and contract
data out of logs and unapproved reports. No telemetry, off-PC
drafting fallback or ranking signal comes from these workflows.

## Requirements

| ID | Requirement | Test design and expected result |
|---|---|---|
| REQ-0098-CRM | Records MUST version terms and reconcile imports without outside sync. | IT-0098-CRM: test the admitted file types, duplicate and conflicting keys and atomic rollback. Verify one renewal task and no third-party write. |
| REQ-0098-SPONSOR | Sponsor views MUST show their supporting evidence, limits and unknowns. | UT-0098-SPONSOR: compare pipeline currencies, missing facts, sparse engagement and approved delivery evidence. |
| REQ-0098-SAFETY | Sponsored work MUST be held without current disclosure, rights and placement evidence. | BDD-0098-SAFETY: given uncertain context, show a review reason without publishing. |
| REQ-0098-NEWSLETTER | Issues and sequences MUST stay as reviewed local drafts. | IT-0098-NEWSLETTER: refuse inaccessible content and private archive intake. A manual plan and due date make no SMTP or marketing request. |
| REQ-0098-CONSENT | Consent and current preferences MUST govern local use and withdrawal. | BDD-0098-CONSENT: given an old opt-in after an unsubscribe, keep it suppressed. Revoke a signed reuse note, retry a queued copy and restore. Release no affected draft. |
| REQ-0098-W2-INACTIVITY-HOLD | Subscriber inactivity MUST stay unknown without a separately admitted exact newsletter profile, current purpose and eligible complete issued exposures, measured outcome, follow-up cutoff and coverage. An aggregate quiet measure MUST NOT substitute. A held inactivity check creates no suppression, consent change or delivery effect. QR-170 retains qualification of the exact supplied profile. | BDD-0098-W2-INACTIVITY-HOLD uses the refusal branch of FX-0098-W2-LOCAL-SUPPRESSION. The existing bounce and consent cases retain their authority. |
| REQ-0098-AUTH | Mail authentication checks MUST cite admitted supplied evidence. | UT-0098-AUTH: distinguish valid, conflicting, missing and stale evidence without a send or DNS change. |
| REQ-0098-COMMUNITY | Community, referral and press proposals MUST require eligible sources and approvals. | BDD-0098-COMMUNITY: given a proposed spotlight and a revoked quote, hold the pack and send nothing. |
| REQ-0098-SITE | Public actions MUST use an admitted platform and conditional approval. | IT-0098-SITE: test every platform and public route for wrong scope, stale grants, limits, cache parity, replay, expiry, revoke before execution and conflicting undo. |
| REQ-0098-PAID | Paid comparisons MUST require comparable measurement and MUST NOT alter ads. | UT-0098-PAID: reject unlike windows or denominators and non-WooCommerce shopping evidence. |
| REQ-0098-MEASURE | Campaign links and measures MUST use approved populations and declared assumptions. | UT-0098-MEASURE: round-trip a safe printed code. Reject unsafe addresses, person IDs, missing or revealing counts, zero cost and an unsupported forecast. |
| REQ-0098-ISOLATION | Actions MUST enforce site, grants, erasure and resource limits. | E2E-0098-ISOLATION: exercise all bounds, wrong site, queued withdrawal, restore and public request budgets. |

### Named sponsor, newsletter and intake fixtures

These planned spec 0076 cases use synthetic records, roles,
consent and native receipts. QR-072, QR-140 and QR-141 hold
source, platform, rights, retention and output proof. DR-088
confines provider work to local drafts and supplied evidence.

- `DES-0098-W2-SPONSOR-RECORDS` uses `VAR-0098-W2-ATOMIC-CRM-DIFF`,
  `VAR-0098-W2-VERSIONED-RATE` and `VAR-0098-W2-LOCAL-RENEWAL-NOTICE`.
  CRM consent gives no native role or new contact purpose.
  A due notice neither renews terms nor contacts a sponsor.
- `DES-0098-W2-SPONSOR-VIEWS` uses `VAR-0098-W2-LOCAL-STAGE-SCENARIO`,
  `VAR-0098-W2-PROTECTED-SPONSOR-PROJECTION`,
  `VAR-0098-W2-LOCAL-MEDIA-KIT` and
  `VAR-0098-W2-ORGANISATION-CHECKS`. Show existing figures and
  their limits. Output needs exact grant and purpose. A scenario
  or score triggers no deal, payment, contact or access change.
- `DES-0098-W2-NATIVE-REVIEW` uses
  `VAR-0098-W2-SPONSOR-REVIEW-QUEUE` only through the existing
  conditional 0021 effect and exact native 0101 reviewer grant.
  A held profile stays at local review. Publishing remains a
  separate approved native action.
- `DES-0098-W2-LOCAL-NEWSLETTER` uses `VAR-0098-W2-REVIEWED-ISSUE`,
  `VAR-0098-W2-SOURCE-LINKED-SUGGESTION` and
  `VAR-0098-W2-MANUAL-WELCOME` for current approved local work.
  No due date, consent record or export approval grants sending.
- `DES-0098-W2-CONSENTED-INTAKE` uses
  `VAR-0098-W2-EXACT-SUBSCRIBER-PURPOSE` and
  `VAR-0098-W2-WP-NATIVE-LEAD`. WordPress lead capture needs
  exact native object and host evidence. Its separate pull
  requires the current actor and approved fields and purpose.
  An unadmitted host keeps the lead route unavailable.
- `DES-0098-W2-CAMPAIGN-EVIDENCE` uses
  `VAR-0098-W2-0034-JOINT-LANDINGS`. Spec 0034 owns the
  approved population and disclosure. Spec 0096 shares that
  same measurement route. These views add no collection.

| Feature | Requirement | Design and variant | Fixture and test | Named fixture assertion |
|---|---|---|---|---|
| F-0436 | REQ-0098-CRM | DES-0098-W2-SPONSOR-RECORDS, VAR-0098-W2-ATOMIC-CRM-DIFF | FX-0098-W2-CRM-IMPORT, IT-0098-CRM | Replay identical organisation and agreement bytes. Refuse the conflicting batch atomically. A disputed merge needs current exact owner approval and cannot revive retired aliases or widen a contact purpose after restore. |
| F-0437 | REQ-0098-SPONSOR | DES-0098-W2-SPONSOR-VIEWS, VAR-0098-W2-LOCAL-STAGE-SCENARIO | FX-0098-W2-PIPELINE-FORECAST, UT-0098-SPONSOR | Compare 19 and 20 valid pre-outcome decisions in one same-site, stage and currency cohort. Hold an uncalibrated scenario. Keep entered amounts, commitments and observed receipts distinct and round the eligible sum once. |
| F-0438 | REQ-0098-SPONSOR | DES-0098-W2-SPONSOR-VIEWS, VAR-0098-W2-PROTECTED-SPONSOR-PROJECTION | FX-0098-W2-SPONSOR-REPORT, UT-0098-SPONSOR | Freeze the authorised sponsor, recipient purpose, period and evidence. Separate promised, measured and unknown. Suppress foreign or revealing totals. Revocation before local publication leaves no visible pack. |
| F-0439 | REQ-0098-SPONSOR | DES-0098-W2-SPONSOR-VIEWS, VAR-0098-W2-LOCAL-MEDIA-KIT | FX-0098-W2-MEDIA-KIT, UT-0098-SPONSOR | Show dated public figures and exact operation rights. Preview an omission before approval. Withdraw one asset during finalisation and replay. Withhold affected bytes. Replaying a committed hand-off returns only its permitted receipt without duplicate output. |
| F-0440 | REQ-0098-CRM, REQ-0098-SPONSOR | DES-0098-W2-SPONSOR-RECORDS, VAR-0098-W2-VERSIONED-RATE | FX-0098-W2-RATE-CARD, IT-0098-CRM, UT-0098-SPONSOR | Keep an accepted offer pinned to its prior exact card. At an effective interval endpoint use the current eligible revision. Overlap holds selection. Quote no converted rate or inferred currency basis. |
| F-0441 | REQ-0098-CRM | DES-0098-W2-SPONSOR-RECORDS, VAR-0098-W2-LOCAL-RENEWAL-NOTICE | FX-0098-W2-RENEWAL, IT-0098-CRM | Change one due date before and again after delivery. Recheck current grant after quiet hours and clock changes. Only one receipt belongs to the renewal cycle. Completion or cancellation blocks pending notice. |
| F-0442 | REQ-0098-SAFETY | DES-0098-W2-NATIVE-REVIEW, VAR-0098-W2-SPONSOR-REVIEW-QUEUE | FX-0098-W2-SPONSORED-REVIEW, BDD-0098-SAFETY, IT-0098-SITE | Pin site, native queue, page and revision, exact placement, reviewer, visible disclosure and licence. Changed context, revoked native role or missing host proof holds the proposal. Reconcile a lost reply as the same review effect. |
| F-0443 | REQ-0098-NEWSLETTER | DES-0098-W2-LOCAL-NEWSLETTER, VAR-0098-W2-REVIEWED-ISSUE | FX-0098-W2-ISSUE-FILE, IT-0098-NEWSLETTER | Bind exact reviewed accessible bytes and rights. Break reading order or revoke an asset and hold output. A public label on a subscriber-only derivative gives no archive intake or mail effect. |
| F-0444 | REQ-0098-NEWSLETTER | DES-0098-W2-LOCAL-NEWSLETTER, VAR-0098-W2-SOURCE-LINKED-SUGGESTION | FX-0098-W2-NEWSLETTER-SOURCES, IT-0098-NEWSLETTER | Keep the exact eligible active-site sources for each fact. Reject foreign evidence and unsupported wording. With no local helper, show approved source cards and manual composition without remote wording. |
| F-0445 | REQ-0098-CONSENT | DES-0098-W2-CONSENTED-INTAKE, VAR-0098-W2-EXACT-SUBSCRIBER-PURPOSE | FX-0098-W2-SUBSCRIBER-SCOPE, BDD-0098-CONSENT | Apply a broad opt-out and exact later withdrawal. Replay an older opt-in and restore an old copy. Neither revives use, and a CRM purpose supplies no newsletter grant or contact export. |
| F-0446 | REQ-0098-MEASURE | DES-0098-W2-CAMPAIGN-EVIDENCE, VAR-0098-W2-0034-JOINT-LANDINGS | FX-0098-W2-PIXEL-FREE-LANDINGS, UT-0098-MEASURE | Keep the admitted same-site consented landing count distinct from a supplied provider click count. Suppress a small or revealing cell. No source gives no measure and no pixel, message ID or inferred click. |
| F-0447 | REQ-0098-SITE | DES-0098-W2-CONSENTED-INTAKE, VAR-0098-W2-WP-NATIVE-LEAD | FX-0098-W2-WP-LEAD, IT-0098-SITE | On a synthetic admitted WordPress adapter pin page, form, session and consent revision. Race same-key equal and changed bodies. Only one receipt commits. Refuse stale consent, exceeded limits, unapproved projection and XenForo. |
| F-0448 | REQ-0098-NEWSLETTER | DES-0098-W2-LOCAL-NEWSLETTER, VAR-0098-W2-MANUAL-WELCOME | FX-0098-W2-WELCOME-PLAN, IT-0098-NEWSLETTER | Approve one delayed local plan with exact wording and own-site link. Restart its due step after withdrawal. Hold invalid work. No contact list, file delivery or automatic welcome message follows. |
| F-0449 | REQ-0098-SPONSOR | DES-0098-W2-SPONSOR-VIEWS, VAR-0098-W2-ORGANISATION-CHECKS | FX-0098-W2-ENQUIRY-SCORE, UT-0098-SPONSOR | Use one point each for an eligible category, comparable approved budget and plan within confirmed dates. Explain each documented check. A private or incompatible fact or missing input leaves the total unknown. |

## How it is tested

Designed, not run. Spec 0076 and QR-072 require feature-specific
evidence, permissions, formats, rollback and measured footprint tests.

## Changes from the source

DR-088 replaces excluded CRM sync and marketing automation with
local records, imports and manual plans. The approved notice and
ticket routes retain their existing scope. Site changes need
admitted platform contracts and approval.
