# Review and undo links in WordPress

Use this guide to decide whether a suggestion is safe to approve
and to understand why WordPress might hold it. If your verified
release marks linking or undo **Not yet available** for your site
or setup, stop at that action.

## Disconnect or remove the integration

Follow [Remove a site or the installation][wp-remove]. A closed
connection tells you the old pairing no longer works. Check the
removal result for any WordPress plugin files or data in the
approved cleanup. A held or unverified cleanup gives no removal
proof. Links already placed in posts stay under WordPress moderation.

[wp-remove]: operations.md#remove-a-site-or-the-installation

## Check the post and the proposed link

Sign in through WordPress and open the site's Sutradhar review
page. Your account needs permission for the exact post or page.
On multisite, check the intended blog and its own connection.
Access to one blog does not grant access to another.

Both the source and destination must be available to a signed-out
visitor. Sutradhar holds draft, scheduled, private and
password-protected content. An ordinary published label alone
does not establish public access. An unfamiliar item type or
unsupported editor or plugin setup also stays held.

Read the proposed change and check the exact words and destination.
For ordinary WordPress link suggestions, Sutradhar wraps existing
visible words in a link. It leaves the words, other links and
protected parts of the post alone. If the post contains an unknown
or unsupported block, Sutradhar holds the whole post. A normal
paragraph next to that block cannot be approved on its own.

## Read the save result

Choose **Approve** only for the exact preview you checked. Before
WordPress saves, it checks your current permission, the post
version, the destination and the site's pause state. A separate
content check must agree with the exact approved change. WordPress
must be able to save only if the post still matches the preview.
If that check is unavailable, writing stays off.

A human edit or plugin that changes the approved result can stop
the save. Read the reason and use a fresh review if one is offered.
A lost reply or **Reconciliation needed** means the result is
uncertain. Follow the original action's result. Sutradhar looks
for proof without writing to the site. Retry only the exact
request when the record proves a retryable refusal before the
save, nothing is in flight and approval is current. An unknown
result stays held. A terminal refusal ends that request. A verified
receipt records the earlier save or undo. If the post changed
afterwards and you can still open it, review its current version
before deciding what to do next. Recovery keeps the newer edits.

## If Wordfence or a cache blocks the action

Read the compatibility or connection result in the site's admin
pages. A detected plugin, cached page or challenge screen does not
prove that the action is supported. A plugin update, changed
settings or an unverified combination can make an earlier result
unusable. Keep the affected action held and follow its stated
next check. Leave security controls, other plugin settings and
site-wide caches in place.

The protected admin view must reach the verified origin over
HTTPS and bypass the page cache. Keep it held if Sutradhar
cannot verify that path or the hosting, editor and plugin
combination. When the verified route calls for a cache refresh,
it applies only to the affected item.

## If a plugin update stays held

Read the result for the exact WordPress site and the update you
requested. The release checker can verify the plugin ZIP while
the site check still needs current compatibility proof. WordPress,
PHP, hosting and its site database have to match a profile covered
by that release. An earlier result cannot cover a database change.

Keep the update held and read the site compatibility reason.
Leave the current plugin files and records in place. Use the
approved update route for that exact site when its checks pass.
The ordinary WordPress upload screen cannot complete a held
update on its own.

## Undo one applied link

Open the applied action and review **Undo**. The action record must
still identify that exact link in the current post. Sutradhar checks
current permissions and saves through the same protected path.
If the link still matches, undo removes just that link and keeps
later words and unrelated links. If the link is already gone,
there is nothing to remove. Changed words, overlapping or
duplicate matches, an expired action record or an unverified
save path need review. The undo preview identifies the single
link to remove and shows the remaining current post.
