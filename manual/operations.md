# Back up, restore, update or remove Sutradhar

This guide is for the owner of a Sutradhar installation. It shows how
to check a backup, tell when a restore or update has passed, and see
which files and settings remain after removal. Use the graphical
screens for your release. You do not need a terminal.

Use the manual and verified files for your exact qualified release.
Stop at an operation, device or profile marked **Not yet available**.
If the app will not open, use the matching offline recovery guide on
verified release media. Run its independently verified local launcher
only if that release provides it. Resume the exact recorded step
only where the profile offers it.

## Make and check a backup

1. Open **Backups**. Check which installation and sites the backup
   covers. Sutradhar is designed to back up nightly and before updates.
   Choose a local folder or an external drive. For a supported
   cloud bucket, separately approve the site, backup type, region
   and exact bucket in the storage setup. The storage service receives
   only the encrypted copy.
2. Access the recovery key through the protected owner flow. Keep it
   somewhere safe and separate from your backup and receipts. Sutradhar
   encrypts the backup on your PC. Without the key,
   you cannot open that copy. If the key is unavailable, stop and use
   another verified backup whose key you can open.
3. Check what the backup includes. It contains the database and
   managed files. It leaves out credentials, caches, telemetry,
   working snapshots from analysis jobs and model files. Models are downloaded
   and checked again when needed. The search index based on
   embeddings is included when there is room. Check the backup's
   contents before relying on that index.
4. Wait for verification. A file name, completed copy bar or cloud
   listing does not establish that the backup is usable. Sutradhar
   checks the exact stored bytes, including reading back an admitted
   encrypted cloud copy. Check the separately dated test restore.
   An upload that is pending or cannot be verified keeps the
   original in place. Deleting it after a successful move is a
   separate choice.

Backups and exports can contain member data. Keep them private.
Previously downloaded copies stay with their recipient. Sutradhar
cannot recall them.

## Move managed storage

1. Open **Move** in **Storage**. Check the site, the items you
   selected and the exact current and new locations. Review and
   confirm the plan shown. If an item is too large for this move,
   leave it in its current location. The preview shows the item's
   verified size and the move limit. If the size cannot be checked,
   leave the plan on hold.
2. Read the verified count. If a write is still being checked, open
   the original move result and wait for that check before trying
   again. Until the new location is verified and selected, leave
   the source copy where it is. Follow any access hold shown for it.
3. If the move stops, read the reason and last verified step. Resume
   only if the app offers it after checking the original move. If
   the run reached its time limit or approval expired, follow the
   current review shown. Confirm a fresh plan only when the app
   offers it and has checked the earlier write. Restarting the app
   keeps the original run limit.
4. To stop a move, choose **Pause**. Wait for **Move paused**. If you
   plan to unplug a drive, wait until Sutradhar says it has stopped
   using that drive. Use your system's safe removal check and wait
   for it to confirm before unplugging.
5. Read which location is active after verification. Removing the
   source copy is a separate choice after a verified move.
   If the new location becomes unavailable afterwards, open the
   original move result. Follow its check for the exact location.
   If removal of the source is unverified, wait for that recorded
   check before trying again. The old copy stays inactive unless
   you confirm a separate move or restore that passes its checks.

## Restore a backup

1. Open **Restore** in the verified release. Confirm the installation,
   backup date and sites in the preview. Check which data and
   settings will come back. Read any list of credentials you will
   need to enter again.
2. Unlock the backup with your recovery key. Wait while Sutradhar
   checks the copy, the list of sites and current removal and erasure
   records. These checks keep an old backup from bringing back erased
   records, a removed site or an approval that has since expired.
3. Wait for the exact verified backup and manifest check. Review
   the current owner-authorised preview. A successful restore
   verifies that each site's records stay in that site. If Sutradhar cannot
   verify the
   current history, the affected sites and data stay unavailable.
   A site with missing connection keys stays disconnected. If you
   choose to pair again, get a fresh approval from the owner and
   site administrator. Restore cannot reactivate revoked keys,
   pairing or grants. Check the result before resuming work.
   If **Restore paused** says removal or erasure records changed,
   keep the restore paused even if its isolated test passed. Wait
   for the app to check current records against the original plan.
   Review a fresh plan if the scope changed. A support bundle can
   show permitted details from current settings and checks. It
   leaves out settings from the held restore.
4. A passed test checks the restored copy at its test location.
   If your release offers a switch to the restored copy, review
   and confirm that separate plan. Wait for **Restore** to report
   the completed switch before working on the restored version.

   If the destination cannot be verified, leave the restore paused.
   Keep your current work and backup. Reconnect the original drive
   if it was disconnected, and wait for **Restore** to check it.
   A matching drive letter or copies of the same files cannot
   verify that destination. If **Restore** reports that the
   recovery key no longer passes its protection checks, keep the
   backup encrypted. Choose another verified backup only if
   **Restore** offers it as eligible.

   If **Restore** shows SUT-848, the app could not record the
   planned switch in its audit log and did not switch copies.
   The passed test still describes only the test copy. Your
   current work and existing holds stay in place. Use
   [Diagnostics][restore-first-check] for the failed audit check.
   Return to the original restore result before trying again.
   Follow any continuation offered after the current checks pass.
5. If restore was interrupted, return to the recorded restore plan.
   Wait for the app to check which steps finished. It checks the
   original owner's approval for this backup, site and settings and
   confirms that the owner still has authority. Resume if the app
   verifies those checks and offers to continue. If the approval or
   scope changed, leave the restore paused. Review and confirm a
   fresh plan if the app offers one.

[restore-first-check]: troubleshooting.md#find-the-first-failed-check

Erasure rules apply before restored data can be used. Older backups
stay encrypted and expire on their retention schedule. Use the
current erasure receipt. It says erased, nothing found, waiting
with a reason, or failed for each target. An unreachable device or
provider version needs proof before it can count as erased.

## Update or roll back

1. Open **Update**. Read the verified release notice. Check the
   affected parts, expected pause, database changes, memory change and
   the order shown for the app and site add-on. For a legacy conversion,
   update the service first and leave the protected connection paused
   until both ends pass. If an enabled extension is incompatible, the
   update waits. For a held WordPress plugin update, see
   [the site compatibility check][wp-update-check].
2. Check which sites and parts this update affects. Only the
   installation owner can confirm its exact scope. A downloaded bundle
   follows the same rule. If the plan changed, read and confirm the new
   plan before continuing.
   Sutradhar verifies the signed release manifest and compares the
   installer or payload bytes with the recorded checksums. On Ubuntu
   or Debian, an update or repair using the release package must pass
   the same verified handoff as [first setup][linux]. Keep
   using the exact copy checked against the separately signed file
   list. If the installer cannot verify that copy at handoff, it
   stops with SUT-832. Follow the first setup guide to verify again.
   A Linux update that resolves dependencies also checks the packages
   the installer will use. If the dependency check fails or cannot be
   proved, SUT-832 can hold the update even with the same verified
   Sutradhar package. Follow the reason in the recorded update.
   Matching dependency names and versions alone cannot clear the
   check. If the approved plan changes, the installation owner must
   review and confirm the new plan before continuing.
   On Windows, independently verify the manifest-listed unsigned
   wrapper before opening it. Verify the payload and required
   components before running them. Apply first trust at portable
   first launch and on different hardware where your release
   requires it. A key supplied only inside the download cannot
   establish trust. If verification fails, stop.
   If the chosen online source is unavailable (SUT-4102), retry
   that source when it returns or preview a complete verified
   offline bundle. An expired proof or a bundle for another release
   or target fails the update check (SUT-832), even when the other
   bundle is genuine. Use the approved verifier above to check the
   exact bundle for the selected release and target. If the update
   is already paused, keep it paused and follow its recorded action.
   Changing the route or release needs the installation owner's
   confirmation of the new plan. Leave an uncertain earlier attempt
   paused for recovery before starting another.
3. Wait for work to pause safely and for the backup check. If there
   is no verified backup with a passed test restore, the app tries
   to make one. When that cannot be done, only the owner can approve
   proceeding without one and enter a reason. That choice cannot
   bypass signature or other required checks.
4. Read the result before resuming work. The new version takes over
   after its self-test passes. If it fails, Sutradhar can start the
   previous version only after verifying that it can safely use the
   current installation and sites. The release and protection checks
   still apply. If the screen says **Previous version restored**, read
   the recorded result and follow the available action.
5. If you see **Update paused**, read which check failed or could not
   be verified. Sutradhar keeps the current data and recorded work.
   Leave work paused. Use the release and failed step in the original
   update record to choose verified recovery help. If the app will
   not open, read that matching guide on the verified offline media
   without starting its installer. If you cannot verify the record
   or the match, keep the update paused and preserve its files and
   data. An earlier guide, installer or backup cannot resolve the
   update’s compatibility check. If the app offers an isolated
   backup test, open **Restore** and review its separate plan and
   target before you confirm. A passed test checks the backup in
   that separate location. The live update stays paused and the
   previous version stays stopped.
   If the app says current owner approval is needed for the remaining
   steps, keep the update paused. Check the last verified step in the
   original update record. A verified package or earlier migration
   does not let the update continue on its own. If the app offers a
   new confirmation, the current installation owner reviews the
   exact remaining work against that record before approving it.

[linux]: getting-started.md#2-verify-the-release-before-you-open-the-installer
[wp-update-check]: wordpress-safety.md#if-a-plugin-update-stays-held

## Repair a missing program file

Open **Check files**. If the app offers **Restore program files** for
a missing managed file, review its location and the release it will
use. Confirm the exact plan. Repair uses the checked file from the
verified release.

If **Check files** reports a different or newly occupied item at
that location (SUT-4100), leave it where it is and read the finding.
If the signed replacement cannot be verified (SUT-832), follow
[the release verification steps][linux] for the exact package.

If the result is missing after an interruption, open the original
Repair result. Wait for it to confirm what that plan did before
trying again. Finding matching file bytes at the location cannot
confirm the earlier repair. Follow the continuation the app offers.

### If Repair needs a restore point

Some repairs need a protected copy of the settings and scripts in
the plan before they can start. Wait for Repair to verify it saved
that copy and can use it for recovery. If the check fails or the
result is unclear, leave that repair on hold.

If you changed the selected settings or scripts after the copy was
made, leave the old plan paused. Review a fresh plan if Repair
offers one after checking the earlier attempt. An earlier copy
belongs to the repair it was made for. If the app offers a
separate plan to restore that copy, follow
[Restore a backup](#restore-a-backup) and review that plan.

## Remove a site or the installation

1. Open **Remove** and check the exact choice and site.
   **Disconnect this site** closes its connection and revokes its
   keys and grants. That result gives no proof that the site plugin
   or data was removed. **Remove this integration** removes the
   managed parts in the approved site plan. It keeps the app and
   resources still used by other sites. **Remove the whole
   installation** needs installation-owner approval and the
   confirmation required for affected sites. Review the managed
   locations. The operating system's uninstall action leaves the
   home folder.
2. Review each managed group. Items described as always removed apply
   only within the selected, verified removal plan. Optional groups
   start on **Keep**. Choose **Delete** only for optional groups you
   want to remove. Kept records stay within their sites. Approved links
   already in site posts remain under the site's own moderation.
3. If you choose **Export**, verify the completed receipt for that
   exact copy before allowing deletion. Its link expires after
   24 hours. Keep the copy private and its recovery key separate.
   Cloud objects need an explicit delete choice. Ask to retain a
   final receipt before confirming if you want one. Confirm the exact
   plan after reviewing changed groups, scope or drive.
4. Wait for the removal plan to stop its processes and bring any
   backup in scope to a safe stop. If you need to stop a normal app
   outside the removal flow, use **Quit Sutradhar completely** and
   check the result. Portable close drains its own work. Before
   pulling a portable drive wait for its verified safe-removal
   result and the native safe-removal check where available.
5. If removal stops, read the reason and follow the help below.
   Keep the previous result open if the app is still checking what
   was removed.
6. Read the removal result. It lists managed items removed, kept or
   still present or unverified. An unreachable or unconfirmed cloud
   version stays unresolved. The receipt proves the named managed
   scope and verified outcomes. **None remain** applies only to a
   verified empty managed group selected for deletion. Previously
   downloaded copies stay with their recipients. See
   [Make and check a backup](#make-and-check-a-backup).
   Physical overwriting would require separate evidence. Retain the
   optional final receipt if requested.

   Check cleanup on the site separately if the removal plan includes
   it. An unverified site cleanup stays open even after the connection
   is closed. Follow the original removal result before doing it again.
   A backup from before the removal may still hold the old site key.
   Restoring that copy cannot reopen the revoked connection.

   If a removal receipt lists an encrypted copy as **Kept**, Restore
   still checks its current access, expiry and required removal and
   erasure records. If access or retention expires,
   or the required history cannot be checked, that restore stays
   unavailable. Choose a different backup only if Restore offers it
   as eligible. See [Restore a backup](#restore-a-backup).

### If removal is held

- If an unstarted plan is out of date (SUT-803), open **Remove**
  again. This can happen after an update, a site removal or a
  change to the managed files. Review the current sites, groups
  and Keep or Delete choices, then confirm the fresh plan.
- If the request is limited to another site or the site in the
  request differs from your window (SUT-1901 or SUT-1904), open
  the site you mean to remove in the desktop app. Check its name
  in the window and open its removal preview.
- If the item on disk does not match its removal record
  (SUT-4100), leave removal paused and use **Check files**.
  Leave the item alone until Sutradhar confirms its identity.
- If a drive goes missing, wait for the original drive. An empty
  folder at its old location leaves those deletions unverified.
  Earlier verified deletions stay listed. Use **Relink** only if
  Sutradhar verifies the same drive. A matching drive letter alone
  cannot prove its identity. Review and confirm any changed plan
  before continuing.
- After an interruption, return to the original removal result.
  Read the verified steps you still have permission to see. If an
  earlier step is uncertain, wait for Remove to check it. Continue
  only if Remove confirms current permission and offers the exact
  remaining step. If permission for that step has changed, leave it
  paused. Review the exact plan again only if Remove offers a new
  confirmation. Earlier verified results keep their original scope.
