# 0059: pankti, the heavy-job queue

Dell is shared. Heavy jobs, such as builds, full test runs and runner
jobs, must not starve Forgejo, PostgreSQL or the test sites
([AGENTS.md section 11](../../AGENTS.md#11-the-shared-computer-dell)).
pankti makes every heavy job wait in one line for a place. It is roadmap
Phase 1 item 7.

## What it does

- There are two places. A job takes one place, with 4 GB of memory and
  8 CPU threads. A benchmark takes both places, with 8 GB and 16
  threads, so it runs alone.
- A third job waits until a place is free. A benchmark waits until both
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

## How to use it

```sh
pankti run -- cargo test --workspace
pankti run --both -- cargo bench
```

The places are lock files in one folder on Dell: `line`, `place-1` and
`place-2`. A job holds `line` while it waits, then holds its places
until it ends. The system frees a lock when its holder ends, even after
a crash or a restart, so no place is ever lost and nothing needs
cleaning up.

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
every 2 seconds that Forgejo's health page and a `git ls-remote` of the
repository each answer within 2 seconds. If any probe is late, or the
job fails or is stopped, the night fails and the watchman raises its
alarm (`chaukidar raise`, spec 0058). Each failed night raises a new
alarm, so a problem that stays is seen every day. PostgreSQL and the
test sites get their own probes once they run on Dell.

## Limits

- If pankti itself is killed while its job runs, the job goes on in its
  capped scope but its place is freed. A later job may then start
  beside it. Both stay inside the 8 GB slice cap, so Dell is still safe.
- pankti does not decide which jobs are heavy. The rule file and the
  runner set-up name them.

## Tests

- A third job waits for a free place, and starts when one is freed.
- A benchmark takes both places, so no other job runs beside it.
- A benchmark waits for both places, and a later job cannot jump ahead
  of it.
- Places are free again after jobs end.
- A missing folder is an error.
- Live: the noisy-neighbour test passes on Dell, with every probe on
  time while the heavy job holds about 3.9 GB and 8 threads, and fails
  when a probe fails.
- The job command carries the slice, the memory cap, no swap, the CPU
  cap and the user's own user and group.
