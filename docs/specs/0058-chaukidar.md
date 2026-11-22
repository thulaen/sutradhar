# 0058: chaukidar, the watchman

Agents have sudo, so the lock on what guards the project is a rule, not
a wall ([AGENTS.md section 11](../../AGENTS.md#11-the-shared-computer-dell)).
The watchman spots any change to those guards made outside an install,
and raises the alarm. It is roadmap Phase 1 item 5.

## What it watches

A private watch list on Dell names each thing, with a short public
label. Each line is `file LABEL PATH` or `command LABEL SHELL-COMMAND`.
It watches Forgejo's door hook, Forgejo's settings, the branch
rules on `main`, and the programs and service settings of the door, the
copier and the watchman itself. A command's output is sorted where its
order can vary, so the same state always gives the same fingerprint.

Two other guards are covered in other ways:

- The protected gate files live in the repository, so they change only
  through a commit, which the door checks before Forgejo takes it.
- GitHub's copy, including any CI files, can only be changed through
  the sync. A direct edit on GitHub makes GitHub's `main` differ from
  Forgejo's, so the sync's next push is refused and raises its own
  urgent task. Once GitHub's edited head stays still for the grace, the
  sync watch raises its alarm too.

## What it does

Every five minutes, a system timer runs `chaukidar check`. It takes a
SHA-256 fingerprint of each watched thing and compares it with the
fingerprints recorded at the last install. When anything differs, can
no longer be read, or is new or missing from the list, it raises one
alarm in two places at once: an urgent task in Forgejo's current phase
and an issue on GitHub. It raises the alarm again only when what it
finds changes. The sync never copies alarms either way.

After the fingerprints, each check also watches the sync (AGENTS.md
section 8, step 11). It reads the head of `main` on Forgejo and on
GitHub, each read limited to 20 seconds (proposed). When they differ,
it keeps a clock beside its state file, in `STATE.sync`: GitHub's head,
when that head last moved, the last check and whether the alarm was
raised. When GitHub's head has not moved for 10 minutes (proposed)
while it differs from Forgejo's, the watch raises its own alarm once,
in the same two places, under its own title, which the sync never
copies. The alarm names both short commit ids, and says so when
GitHub could not be read. A sync that keeps
copying behind steady pushes never raises it. A GitHub head that cannot
be read counts as not moving. When Forgejo's head cannot be read,
nothing is judged and the clock is kept. Each reason goes to the log.
The clock clears when the heads match and starts over when GitHub's
head moves. A clock from the future, or one whose last check is more
than 15 minutes (proposed) old, restarts its waiting time but keeps
whether the alarm was raised for the same GitHub head. The sync watch
runs after the tamper alarm, so it cannot drop or change that alarm,
and its own errors only go to the log.

Limits of the sync watch:

- It needs the alarm's settings. Without them it names the missing
  setting in the log and does nothing, since there is nowhere to raise
  the alarm.
- A sync paused for its campaign (AGENTS.md section 9.1) is not a
  failure. The watch does not read the pause, so its alarm says this,
  and the reader closes it.
- When the sync refuses a push, it raises its own urgent task, so the
  same outage can show up twice. The sync alarm points to that task.
- It compares heads, not history, so it cannot tell whether Forgejo is
  ahead of GitHub or the two have parted.

Other guards can raise the same alarm with `chaukidar raise TEXT`, where
TEXT is one public line. The nightly noisy-neighbour test (spec 0059)
uses it when a night fails.

## Installing and updating

On Dell, from the repository's top folder, on the commit just accepted:

```
deploy/chaukidar/install.sh WATCH_DIR HEADER_FILE
```

`WATCH_DIR` is a private folder holding the watch list. `HEADER_FILE`
holds Forgejo's sign-in header. The bootstrap script records
fingerprints from the watch list it reads and clears local alarm
state. That step supplies a local observation.

### Accepting a replacement baseline

For protected acceptance, require the reviewed watch inventory and
its scope under the [protected drift case][protected-drift-case].
Verify any protected tool installation under
[0008](0008-trust-boundaries.md#protected-policy-and-installed-tools).
[0062](0062-forgejo-server.md#settings) supplies the Forgejo settings.
Compare the exact watch definition for each label, including its
kind and private target or command, as well as its fingerprint.
Require independent read-back of the same accepted installation,
policy and watch inventory. Repeat the identity check at acceptance.
An old approved result or a definition changed since review leaves
the replacement unqualified, even when every label and target
fingerprint matches.

Install, recovery and timer checks can overlap. Accept a clean or
changed result for the current baseline only when its checked
inventory and installation still match before each external alarm
dispatch and before changing local alarm state. Check both sinks
independently. A delayed result cannot use one accepted dispatch as
authority for another. Preserve prior accepted baselines, unresolved
drift and exact verified sink receipts on a hold.
Apply the existing reconciliation to the matching drift episode.
The clock, timer state and a successful local fingerprint write do
not prove that the watched source is back to its expected state.
An alert that goes quiet remains unresolved until independent
read-back confirms the exact watched source under the current
accepted definition.

Use existing admitted evidence and authority. If the available
fingerprint proof cannot establish an exact definition or current
installation, hold protected acceptance. Private watch definitions
stay on Dell. External alarms use only their existing public labels
and safe findings.

[protected-drift-case]:
  ../planning/test-and-traceability.md#task-0062-protected-checksum

## Settings

The alarm needs `CHAUKIDAR_FORGEJO` (Forgejo's API address for the
repository), `CHAUKIDAR_HEADER` (a private file with Forgejo's sign-in
header) and `CHAUKIDAR_GITHUB` (the GitHub repository, as
`OWNER/NAME`). The sync watch reads both heads with the same settings.

## How it is tested

`crates/chaukidar` tests reading the watch list, fingerprints of files
and commands, every kind of change, the alarm in both places (and when
one place fails), the command itself, and that `raise` refuses an empty
or many-line text. Its sync watch tests cover matching heads, a sync
that follows steady pushes, a stuck sync raising once, a GitHub read
that comes and goes, an unreadable Forgejo, clocks from the future or a
stopped timer, a restart that keeps a raised alarm, the clock's own
format, SHA-256 commit ids, the alarm's own title, and the command
logging an unreadable Forgejo while an unchanged list still passes.

The [protected drift case][protected-drift-case] compares a reviewed
replacement with a stale accepted installation, a retargeted label
whose fingerprint stays equal, quiet drift with the source still
changed, and a delayed clean or changed result from the preceding
installation. In a separate synthetic case commit a result on the
first sink. Replace the approved installation before dispatch to the
second. Retain the first sink receipt and refuse a new dispatch or
alarm state write for the old result. Only the exact current
observation can clear its own finding. Keep the fixture synthetic
and the private watch definitions
out of the expected alarm output.
