# Manage Sutradhar on your site

Use this guide to check who can manage a site, review a link
suggestion and tell whether a save or undo took effect. Use the
manual for your verified release. If it marks a site, screen or
action **Not yet available**, leave that action paused.

## Set up access and check the connection

1. Follow [Getting started](getting-started.md) to verify the release
   and matching add-on. Open Sutradhar in the site's own admin pages
   at the approved HTTPS address.
2. The site administrator checks the site, PC installation and
   requested permissions there. The owner of the PC confirms the
   same details in the desktop app. Separately compare the signing
   and encryption fingerprints on both trusted screens. These short
   values identify the keys the site and PC expect. A different key
   requires an explicit trust decision. If the approval expires,
   start a fresh check through the trusted screens.
3. Sign in through XenForo's sign-in and permission prompt or
   WordPress's own sign-in. Review access applies to the named
   account on that site. The account needs the current review
   permission and access to the specific item. Apply and undo also
   check the current native permissions for that item. Sign in again
   if the session expires. If the site's grant was withdrawn, wait
   for its administrator to review and restore the grant.

### Reconnect a restored site

If a restored connection stays paused, use its connection check.
Matching saved fingerprints can identify the keys. Wait for the app
to verify that the PC and site still have their current records of
accepted requests. Leave the site paused if it cannot check them.

If the app offers a fresh pairing, get the owner and site
administrator approvals and compare the current keys again as
described above. Review any still-needed changes again. For a
request sent before the restore, check its original result before
deciding what to do next. The old approval stays closed.

### Check a XenForo add-on reporting warning

The site can show recent signed contact while the XenForo add-on
still has the reporting warning **SUT-340**. **Last contact** is
the most recent verified reply. The reporting check also needs a
new heartbeat, which is the add-on’s own regular check-in.

If a new heartbeat arrives, the warning clears and Sutradhar
checks for changes it might have missed. Read that recovery
result separately. Coverage stays incomplete where those changes
are still unverified. Check the current source and preview before
using a suggestion whose evidence may be out of date.

## Decide on a link suggestion

1. Open the site's review page. Check the source text, highlighted
   words and chosen destination. Read **Why** and compare the
   preview with what you want the post to say. If the check is
   still running, the preview is unavailable or the source or
   destination changed, wait for a current review.
2. Choose **Approve** for that exact destination, or **Reject** if
   you do not want it. You can give a reason when rejecting.
   Automatic link insertion is not available in version 1.
3. Read the result for that action. **Approved** means your
   decision was recorded. Wait for the app to confirm the save
   before treating the link as applied. The site checks current
   permissions, pause state, visibility, source and destination
   revisions and the destination's current address. If that address
   changes or the revisions differ, review a fresh preview.

### Recover a group of approvals

If the app loses the result after you approve several suggestions,
return to the original group in its recorded order. Read each
available result separately. Some saves may already have taken
effect. If your access has been withdrawn, leave the results
closed and ask the site administrator to check your permissions.
Submitting the group again cannot recover those results.

With current access, follow the original result check. It can
confirm an earlier save without performing it again. Leave an
unconfirmed item under review. An item proved unperformed can
continue only through its own current approval and safety checks.

### Bring a rejected suggestion back for review

In the site's trash bin, use **Restore** if it is offered for that
suggestion. Wait for the checks. An eligible suggestion returns to
review and still needs approval before any link can be placed.

If the restore result is unclear, check the original action before
trying again. A completed restore applies to that earlier
rejection. If someone rejects the suggestion again later, review
the current rejection before making another decision.

## Review a XenForo hub proposal

If a hub preview says the topic changed, open the current
proposal when it is available. Review its links again before
approving it. If current topic results are unavailable, leave
the proposal on hold. A topic change leaves an existing
published hub in place. Its links and extracts still go
through their usual safety checks.

## Restore a rejected draft

A rejected draft can stay in the draft bin after your review
access or its source access expires. Restore needs current
access to both. If the app reports expired source access,
leave the draft in the bin. When a restore succeeds, the draft
returns to review. Check the current draft before approving
it again.

## Read AI traffic for this site

Open Sutradhar in the WordPress or XenForo admin pages and choose
**AI traffic**. Pick a date range and check coverage. **Verified AI
fetches** counts eligible requests from identified AI crawlers.
**Human visits from AI services** counts eligible visits with a
supported referral. Some figures may be withheld for privacy or
unknown because the site lacks evidence.

The ranking panel labels the configured fetch weight. Where your
view permits it, the panel shows the evidence and expiry. A
verified fetch can change the order of links that already meet
relevance and safety checks. The 28-day window starts at the
UTC date of the first qualifying fetch for that version. Later
fetches can still count in the report. The window keeps its original end
date. A new version needs its own qualifying fetch.
The default weight is 0, so no boost applies until the owner sets a
value from 0 to 500 basis points (0 to 5%; DR-160). Review or adjust it in the
owner app. Human visits are reported separately. Link suggestions
still go through the normal moderator decision.

## Pause work or resolve an uncertain result

- A site administrator can pause Sutradhar from that site's admin
  pages. Pause stays in force after a restart and protects that
  site's saves and undo actions. Other sites are separate.
  Resume only after checking current permissions and the reason
  for the pause. An expired approval needs fresh review.
- If the site connection is revoked, saves and undo requests
  that have yet to be sent stop. If a request was already sent,
  the job can stop while the site result is still unknown. Keep
  the original action under review until its result can be
  verified. A verified final refusal ends that request.
- If a reply was lost or the result is unclear, use the recorded
  action's result or recovery option. Sutradhar checks the
  original save without writing to the site. Retry the exact request
  only if the record proves a retryable refusal before the save,
  nothing is in flight and the approval is current. A timeout,
  missing result or expired approval keeps it held. A verified
  receipt records the earlier save or undo. If the post changed
  afterwards and you can still open it, review its current version
  before deciding what to do next. Recovery leaves later edits in
  place.
- You need current review access even when a review page is open.
  If the page says it cannot check your access, leave that item
  closed and ask the site administrator to check your permissions.
  Open a fresh review only when the site confirms access.
  A person with audit access can read its retained audit entry. If
  the site’s recovery record expires before the outcome is verified,
  the audit entry alone cannot confirm the save or undo. Keep the
  action under review.
- For an applied link, review **Undo** against the current post.
  It removes only the link from that action when it still
  matches, keeping later text and unrelated links. If the words
  or link changed, matches overlap or several links match, ask for
  review. XenForo quote blocks stay outside the edit.
- If someone adds the same link again later, the earlier **Undo**
  result still describes the original post revision. Recovery
  preserves the later link. Check the current post if you still
  have access. If the app cannot check it, leave the current link
  status unconfirmed.

If the PC is unreachable, the site pages show the last known
connection state and keep site pause available. PC updates,
installation repair and engine settings belong in the desktop
app. For WordPress-specific refusals, see [Review and undo links
in WordPress](wordpress-safety.md).
