# Development storage recovery handoff

Owner: authorised host operator and continuing coordinator.
Recorded: 30 October 2026. Status: failed-drive incident retained; active storage
replacement authorised. Do not write to or reformat the failed drive.

The owner subsequently selected a dedicated protected directory on the existing
data SSD. Follow [storage preservation](storage-preservation.md). The earlier
physical-reconnection request no longer blocks preparation of this authorised
replacement. Normal runtime execution still requires its paths, state and
environment to be correctly migrated and verified.

## Observed state

The authorised development project drive unexpectedly disconnected and reappeared
while final approval metadata was being transferred. The owner confirmed that
they had not moved or unplugged it. The host recorded failed writes, an aborted
filesystem journal and filesystem shutdown. The drive is visible again, but its
project filesystem is not mounted. The underlying mount directory is deliberately
inaccessible to ordinary users, preventing fallback writes onto the system disk.

A read-only filesystem check was run against the verified project partition.
It skipped journal recovery and reported directory/bitmap/count inconsistencies.
It made no repairs. These observations do not establish permanent disk damage,
the cause of the connection failure or which files will survive journal recovery.
During that initial diagnosis, no automatic reformat, repair or writable remount
was performed, and no drive-health utility was installed. The later owner-approved
replacement directory is a separate destination, not a repair of the old drive.

A later read-only inspection found filesystem entries for the earlier focused
check receipt, task database and specification-approval record. No process mount
namespace referenced the project partition at that inspection. Entry presence
does not prove complete or correct file contents, database integrity or retained
journal transactions. Those checks still belong after safe recovery.

The owner was initially asked to reconnect the drive. Their later replacement
decision superseded that prerequisite for new work. Do not write to the failed
drive; qualify the replacement environment before its normal runtime use.

## Preserve and recover

1. Preserve the complete local source working tree, including
   [the three-specification approval](../specifications/approvals.md). Do not reset
   or clean it, run tests there, or create local build/evidence/backup directories.
2. Confirm the physical connection and exact authorised device using the private
   host records and current device identity. Do not rely on a changeable device
   letter. Never operate on the system disk or unrelated project storage.
3. Confirm the partition is not mounted or used elsewhere before any filesystem
   operation. Assess the connection and read-only findings before choosing a safe
   recovery action. Retain observations; do not treat format as recovery.
4. Verify recovered data, the private SQLite ledger and retained check receipts.
   The last attempted finalisation failed before its evidence rename; its final
   receipt, private operator handoff and task-freeze event were not completed.
5. Copy final public source from the preserved local checkout to the authorised
   replacement only after its mount is verified. Verify the actual mount before creating
   directories, caches, dependencies or evidence, including when running as root.
6. Rerun the applicable source checks on the final files and verify the approved
   specification bodies against the recorded hashes. Record the recovered state
   and a safe task handover before implementation resumes.

The App credential/publisher blocker is separate and still open. A recovered
drive does not make unpublished source visible to cloud Claude or qualify the
publisher, checks, product or release. Continue with [the main handoff](handoff.md)
using the qualified replacement environment; recovery of old private state is a
separate task and must not be assumed complete merely because source was recopied.
