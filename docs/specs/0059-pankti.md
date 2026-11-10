# 0059: pankti, the heavy-job queue

Dell is shared. Heavy jobs, such as builds, full test runs and runner
jobs, must not starve Forgejo, PostgreSQL or the test sites
([AGENTS.md section 11](../../AGENTS.md#11-the-shared-computer-dell)).
pankti makes every heavy job wait in one line for a place. It is roadmap
Phase 1 item 7.

## What it does

- There are two places. A job takes one place, with 4 GB of memory and
  8 CPU threads. A timed benchmark takes both places, with 8 GB and 16
  threads, so it runs alone; a counted one takes one place.
- A third job waits until a place is free. A timed benchmark waits until both
  are free, and jobs that come after it wait behind it, so it is never
  starved.
- Each job runs in its own systemd scope inside `sutradhar-heavy.slice`,
  with a hard memory cap, no swap and a CPU cap. The slice itself is
  capped at 8 GB and 16 threads, so the two places together can never
  take more.
- pankti passes on the job's exit code. A job stopped by a signal, such
  as the memory cap, ends as 128 plus the signal's number.
- sudo clears the job's settings, so pankti passes on a short fixed
  list: `PATH`, `HOME`, `LANG`, `RUSTUP_HOME`, `RUSTUP_TOOLCHAIN`,
  `CARGO_HOME` and `CARGO_TARGET_DIR`. `AEGIS_QUEUE` is left out, so a
  job inside a job never waits for a place held by its parent. The job
  keeps its working folder. It runs as the same user, with all the
  user's groups.

## Sealed jobs

`pankti run --sealed -- COMMAND` is for code under check, which is not
yet trusted. The job runs as a short-lived systemd service, not a
scope, with:

- no network (`PrivateNetwork`), and cargo told to stay offline;
- no way to gain rights (`NoNewPrivileges`), so no sudo;
- its own `/tmp` and shared memory, and no sight of `/run/user` or the
  system message bus, so it cannot reach the user's own services;
- the whole system read-only (`ProtectSystem=strict`, home folders
  read-only), cargo's home included, but for the build folders named in
  its settings or its `--target-dir`; a build folder may not exist yet;
- the project folder read-only; the checks' own working copies sit in
  the build folder, so a job can write those;
- no devices, kernel settings, kernel modules or control groups;
- at most an hour and 4,096 processes.

It keeps the same caps, user and groups as any job. A writable folder
must be a full path below the top folder, with no white space, which
systemd's list could split. Since the job is offline and cargo's home
read-only, the checks first run `cargo fetch --locked` outside the
seal: it downloads only the exact libraries in the lock file and runs
no code from the project. Cargo settings in a tree (a `.cargo` folder)
could make even a fetch start a program, so when a tree holds them, no
cargo step runs on it at all, in the pushed tree or an old-code tree.
The checks keep their working copies in the build folder
(`target/aegis-work`), where a sealed job can read them. The commit and
push checks and the door run their Rust steps sealed. The runner's loop
does not, since it must reach Forgejo and the container service; its
jobs are sealed by their containers instead (spec 0060).

## How to use it

```sh
pankti run -- cargo test --workspace
pankti run --sealed -- cargo test --workspace
pankti run --both -- cargo bench
```

The places are lock files in one folder on Dell: `line`, `place-1` and
`place-2`. A job holds `line` while it waits, then holds its places
until it ends. The system frees a lock when its holder ends, even after
a crash or a restart, so no place is ever lost and nothing needs
cleaning up.

`PANKTI_DIR` names the folder that holds the three lock files. When it
is unset, pankti uses the folder above its own program's folder.

## Install

`deploy/pankti/install.sh QUEUE_DIR GROUP` builds pankti, installs it in
`QUEUE_DIR/bin`, makes the three lock files, and installs the slice.
Every user in `GROUP` may queue jobs. It needs sudo, as do the jobs,
since a system scope is made for each one.

## Who uses it

- The commit and push hooks, and the door, run aegis's format, lint
  and test steps through pankti. They name it in `AEGIS_QUEUE`; with
  no queue named, as on a contributor's own computer, the steps run
  directly.
- The Forgejo runner's jobs run through it too (Phase 1 item 2).

## The nightly noisy-neighbour test

`deploy/pankti/install-nightly.sh QUEUE_DIR WATCH_DIR HEADER_FILE` sets up
a timer that runs `noisy-neighbour.sh` every night at 03:30. It takes
one place with a job that fills all 8 threads and about 3.8 GB for a
minute. The job may first wait in line; probing starts only once it
has begun, so a night can never pass without the load. Then it checks
every 2 seconds that Forgejo's health page and its public list of
repositories each answer within 2 seconds. The probes need no sign-in.
If any probe is late, or the job fails or is stopped, the night fails
and the watchman raises its alarm (`chaukidar raise`, spec 0058). Each
failed night raises a new alarm, so a problem that stays is seen every
day. PostgreSQL and the test sites get their own probes once they run
on Dell.

## Keeping the disk from filling

Builds and caches only grow. Two guards, set by the owner on 9 November
2026, keep the disk from filling:

- Before a heavy job takes its place, pankti checks the free space on
  the disk of its working folder. Below 50 GB free it refuses, and says
  so.
- Every night at 03:00, `deploy/pankti/sweep.sh` empties any listed
  build folder over 10 GB, since it holds only output the next build
  makes again. It also removes the checks' leftover working copies in
  `aegis-work` older than a day, and the scratch folders our tests
  leave in the temp folder (`aegis-*` and `forge-sync-*`, its own
  account's, over an hour old), since on Dell that folder lives in
  memory. It touches a folder only when it is a
  full path, not `/` or a home folder, and holds cargo's own files: a
  real `CACHEDIR.TAG` (not a link) with the standard mark, and
  `.rustc_info.json`. Removal never crosses into another disk. It holds
  both of the queue's places while it works, so no queued job runs
  meanwhile, and leaves for the next night any folder whose cargo build
  lock another build holds. If the disk still has under 50 GB free, or
  anything goes wrong, the night fails and the watchman raises its
  alarm.

`deploy/pankti/install-nightly.sh` installs both nights; its last
arguments are the build folders to sweep.

## Limits

- If pankti itself is killed while its job runs, the job goes on in its
  capped scope but its place is freed. A later job may then start
  beside it. Both stay inside the 8 GB slice cap, so Dell is still safe.
- pankti does not decide which jobs are heavy. The rule file and the
  runner set-up name them.

## Tests

- A third job waits for a free place, and starts when one is freed.
- A timed benchmark takes both places, so no other job runs beside it.
- A timed benchmark waits for both places, and a later job cannot jump ahead
  of it.
- Places are free again after jobs end.
- A missing folder is an error.
- Live: the noisy-neighbour test passes on Dell, with every probe on
  time while the heavy job holds about 3.9 GB and 8 threads, and fails
  when a probe fails.
- The job command carries the slice, the memory cap, no swap, the CPU
  cap and the user's own user and group.
- A sealed job cannot reach the network, use sudo, see `/run/user`,
  start a user service, or write its working folder or cargo's home, and
  the full test suite passes in one (proved live on Dell, and by the
  `a_sealed_job_has_no_network_no_new_rights_and_little_to_write` test
  of its command line).
- Free space: `check_space` refuses below 50 GB and passes at it, and
  `free_bytes` reads `df` (tests); the sweep empties a marked folder
  over its cap, leaves an unmarked folder alone, and fails below the
  floor (proved live on throwaway folders and on Dell's build folders).
