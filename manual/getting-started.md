# Set up Sutradhar and connect one site

This guide is for the owner of the PC and the administrator of one
XenForo forum or WordPress site. At the end, you'll know whether the
local installation and the connection to that site have passed their
checks. You can stop after the local checks and connect the site later.

Use this guide with a qualified, versioned release for your exact PC
and the manual supplied with it. If there is no stable release or
your system lacks a qualified package, stop before looking for a
download or opening a launcher. Stop at any step marked
**Not yet available** for your release.

## 1. Check your PC and prepare the offline bundle

Use a release qualified for your exact system on an x86-64 PC.

| System | Package and requirements |
|---|---|
| Ubuntu 26.04 LTS | Exact `.deb` checked against this release's separately signed file list. The desktop needs WebKitGTK 4.1. |
| Debian 13 | Exact `.deb` checked against this release's separately signed file list. The desktop needs WebKitGTK 4.1. |
| 64-bit Windows 11 | Installer and payload for this release. The installer is unsigned, so verify its exact file independently before opening it. The desktop needs WebView2. If it is missing, use only the verified component for this release. |

Check that the PC has at least 8 GiB of memory, that its clock is
correct, and that you have room for the program, your site data, the
starter models and backups. The setup check shows the storage it
expects to need. With fewer than four processor cores or without a
solid-state drive, the 30-second target for new posts does not apply.
Normal setup needs 8 GiB or more. Below 8 GiB, proceed only if
your exact release has a qualified Safe mode offer and shows it
during preflight. Otherwise stop.

Before going offline, have the complete bundle for that release. It
includes the signed file list, matching signed release record, public
proof that the release passed its checks for your system, and the
matching package, required components and local recovery guide. The
starter models are separately verified files. Keep the required
offline model files and any matching WebView2 component on the
verified medium before going offline. If you're preparing the
site add-on, keep its signed ZIP for your approved combination. The local
install and recovery guide
works from verified media even when the app cannot start.

## 2. Verify the release before you open the installer

1. Use the trusted release information from an earlier release you
   already trust, or an owner record you checked separately. Use the
   graphical verification route qualified for your system. The
   downloaded bundle cannot establish its own trust.
2. Use the approved verifier to confirm the signed file list,
   signed release record and current public release proof for your
   exact release, operating system and processor. The verifier also
   checks the exact package and its required files against the list.
   A genuine bundle for another release or target fails
   the match. If a file, signature or proof is missing, stale or
   different, stop and obtain the correct verified bundle. Use the
   independent verifier on Windows or the verified graphical route
   on Ubuntu or Debian.
3. On Windows, the verifier and its trusted release key must be
   independent of the unsigned installer. Confirm the verifier has
   checked that exact installer file against the signed release list
   **before** opening it. Verify the payload and required components,
   including any WebView2 component, before they run. Stop if the
   independent verification option is unavailable.
4. On Ubuntu or Debian, continue from the graphical verifier using
   the exact package copy it checked. If your system asks for
   permission to install, the installer must still prove it is using
   that same copy. If this check fails, setup stops with SUT-832.
   Repeat verification through the approved route for your release.
   On Windows, open the independently checked installer through
   that release's approved graphical route.
5. Review the setup plan and choose the folder that will hold
   Sutradhar's local files. Keep it on an eligible drive accessible
   to your account. Optional modules are off by default. Review
   the monitoring choice before you continue.

Wait for the local health checks and the built-in test with made-up
data. The test checks that the parts can start and work together. It
does not write a real post or start reading your site's older posts.
If setup stops, follow the recovery steps below.

## 3. Connect one site you administer

You can install the app while offline. To check a site connection, the
PC must be able to reach that site's approved HTTPS address. The PC
contacts the site. The site needs no connection back to your PC.

1. Use XenForo 2.3 or one of the two WordPress major versions covered
   by this release, with PHP 8.5 and a compatible Sutradhar add-on.
   On your trusted PC, check the exact ZIP against the signed
   release file list for the selected site. Use the upload route
   qualified for your release and host. The site administrator
   must separately approve the exact action in the site's admin
   pages. Continue only if the route proves before unpacking that
   the site will use the same verified ZIP. Stop if it cannot.
2. Review the setup checks in the site's admin pages. Confirm the
   intended site and its exact trusted HTTPS address. A shared host
   must pass its own checks. If a protected admin view fails its
   connection checks, stop there. Use the hosting path the release
   admits, with HTTPS to the authorised site itself. A proxy that
   ends the browser's encrypted connection cannot serve the protected
   views.
3. The native site administrator confirms the named installation and
   scopes in the admin pages. The owner confirms the same installation,
   site and scopes in the app. Each approval is short-lived and can
   only be used once. This is how the PC and site recognise each other.
4. Check the site identity, supported connection version and the
   signing and encryption key fingerprints shown for both sides.
   A fingerprint is a short identifier for a key. Compare the values
   shown on the two trusted screens as instructed, independently of
   the messages being checked. Stop if they differ or a key has
   changed without your approval. Let the connection check that
   both ends still control those signing and encryption keys before
   pairing completes. If an approval expires, repeat both approvals.
   Keep credentials and protected site content out of this exchange.

## 4. Check what the connection permits

Read the connection result for that exact site. Check that the app
verified the site's reply, that the versions match and that the
permissions it reports match what you approved. An unsupported,
refused or unverified operation stays unavailable. A healthy reply
does not grant an extra capability or mean an edit was made.

After pairing, the site setup checks the actual connection and the
named site's account permissions, write limits, pause and recovery.
It uses made-up content. A failed or uncertain check holds the affected
operation. Keep the first read of older content paused until you've
reviewed and approved it.

Sutradhar suggests links. A moderator uses their own native account on
the exact site and target to accept an eligible suggestion. Check
current permission for that action. App approval alone gives no right
to apply links, and a missing, revoked or unproved native right holds
the edit. Automatic link
insertion is not available in version 1.

## If a check fails

Read the specific failed check and its code. Use the matching local
help from the verified bundle if the app will not open.

| What the check reports | What to do |
|---|---|
| Preflight failed (SUT-830) | Read which item failed. Check the named system or resource, the clock, and the chosen drive as applicable. After correcting that item, repeat the preflight. Review any warning. A failed check stops setup. |
| Package signature, file, handoff or release-proof check failed (SUT-832) | Leave setup stopped. Obtain the bundle for the selected release and target if the one you have differs or its proof expired. Verify through the approved independent graphical verifier on Windows or the existing verified route on Ubuntu or Debian. Continue only with the exact checked copy. |
| Update source unavailable (SUT-4102) | Keep using the installed app if it is healthy. Retry the chosen online source when it is available, or select a complete verified offline bundle. If the plan changes, review and confirm the new plan as the installation owner. |
| A startup health check failed (SUT-834) | Use the same release's local recovery instructions. Resume from the recorded setup step if the installer offers it. If the outcome is unclear, keep setup paused. |
| Setup could not verify that it created the chosen home folder or recorded its creation | Keep setup paused. Use the verified recovery help to check the original setup step, folder and drive. Resume that step only after the installer verifies the original result and offers to continue. |
| The update self-test failed and the previous version was restored (SUT-835) | Check the recorded recovery result and follow the available action. See [Update or roll back](operations.md#update-or-roll-back). |
| Update paused because a compatibility check failed or could not be verified | Leave work paused and use the verified recovery help for this release. Keep the current files and records. See [Update or roll back](operations.md#update-or-roll-back). |
| Add-on and app versions do not match (SUT-838) | Check which side the setup result tells you to update. Use the correct verified package and repeat the site check. |
| The home folder is missing or cannot be written (SUT-4101), or the desktop's web view is missing (SUT-4103) | Check the original approved drive and folder, or the required verified desktop component. Resume only when the named preflight check passes. |
| Protection is unverified (SUT-3700), the key store or protected connection is unavailable (SUT-3701), or protected data cannot be opened (SUT-3702) | Leave the affected part paused. Use the named local unlock or verified connection recovery step if offered. Retry the check after the cause is corrected. Keep unchanged trust records when the cause is uncertain. |

If the pairing reply is lost, use the check for the same site and
request to reconcile its receipt. Leave that site paused if the result
is unclear. Repairing trust in a release is a separate guide from
pairing recovery. Neither can bring back revoked keys or permissions.
A new pairing needs fresh approval from the owner and the native site
administrator.

Setup needs no cloud account. Keep recovery and outside services in
their own approved scopes. Never use unprotected HTTP or a plain
export as a workaround. Encrypted backups and exports need the
intended recipient and the applicable recovery key. When you ask for
help, give the safe error code and failed step. Keep private paths,
keys, credentials and site content out of the message.
