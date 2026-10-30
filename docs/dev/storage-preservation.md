# Development storage preservation

Owner: project owner and authorised operator. Decision: 30 October 2026.
Status: approved dedicated-directory replacement; operational migration pending.

## Authorised location and scope

The owner chose a dedicated directory named `sutradhar` on the existing data SSD
after the earlier project drive disconnected. The exact host path and verified
device identity belong in the private operator notice. The operating-system disk
and neighbouring project directories are excluded. Do not format any disk.

This directory is permanent project storage. The owner explicitly requires that
its contents not be deleted without their instruction. Related projects and
agents must exclude it from cleanup, cache eviction, pruning, retention expiry,
recovery disposal, recursive moves and broad filesystem maintenance.

## Protection model

The project root is owned by the administrator and has the filesystem's
append-only directory protection. This permits new entries while preventing
ordinary removal or renaming of entries and the protected directory. Saved
foundation versions and their files are locked with the filesystem's immutable
protection, preventing deletion, renaming, overwriting and truncation while the
flags remain set. Access permissions are restricted to the project role.

These are operating-system safeguards, not a guarantee against an administrator
who deliberately removes them. Agents must never clear these flags or use
administrator access to defeat preservation without an explicit owner
instruction identifying the affected material. Do not claim hardware failure is
prevented.

Future authorised work must use new versions, leaving preserved versions intact.
Do not pretend an immutable source copy is a functioning writable checkout,
SQLite service, build cache or runtime. Before creating a mutable working area,
specify and verify how its saved work remains preserved and how the owner's
deletion rule applies. No automatic garbage collection is authorised by this
storage switch, including for failed output or supposedly temporary files.

## Verification and transition

The private host receipt must identify the actual data volume, owner,
permissions, filesystem flags, source hashes and observed refusal of
representative delete, rename and overwrite attempts. Use disposable probes for
refusal testing rather than risking project content. Keep those probes and the
receipt preserved too.

The local source is copied into a new protected foundation version. Earlier
private evidence, the task database, service-home configuration and paths on the
failed drive remain historical until recovered or reconstructed with explicit
provenance. Do not write to the old empty mountpoint or silently redirect
private state to the operating-system disk. Reverify the actual mount before
every setup operation that could create storage, including privileged
operations.

The requested related Codex session receives the exact private path and an
explicit exclusion notice. That notice supplements filesystem protection; it
does not authorise the other session to modify this project. See the
[handoff](handoff.md) for the remaining publisher, checks and implementation
work.
