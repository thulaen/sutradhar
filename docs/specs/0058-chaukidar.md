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
  urgent task.

## What it does

Every five minutes, a system timer runs `chaukidar check`. It takes a
SHA-256 fingerprint of each watched thing and compares it with the
fingerprints recorded at the last install. When anything differs, can
no longer be read, or is new or missing from the list, it raises one
alarm in two places at once: an urgent task in Forgejo's current phase
and an issue on GitHub. It raises the alarm again only when what it
finds changes. The sync never copies alarms either way.

Other guards can raise the same alarm with `chaukidar raise TEXT`, where
TEXT is one public line. The nightly noisy-neighbour test (spec 0059)
uses it when a night fails.

## Installing and updating

On Dell, from the repository's top folder, on the commit just accepted:

```
deploy/chaukidar/install.sh WATCH_DIR HEADER_FILE
```

`WATCH_DIR` is a private folder holding the watch list. `HEADER_FILE`
holds Forgejo's sign-in header. The script records fresh fingerprints,
so run it again after every other install, or the watchman will flag
that install.

## Settings

The alarm needs `CHAUKIDAR_FORGEJO` (Forgejo's API address for the
repository), `CHAUKIDAR_HEADER` (a private file with Forgejo's sign-in
header) and `CHAUKIDAR_GITHUB` (the GitHub repository, as
`OWNER/NAME`).

## How it is tested

`crates/chaukidar` tests reading the watch list, fingerprints of files
and commands, every kind of change, the alarm in both places (and when
one place fails), the command itself, and that `raise` refuses an empty
or many-line text.
