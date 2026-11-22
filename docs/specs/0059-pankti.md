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
one place with a job that fills all 8 threads and about 3.8 GB.
The job may first wait in line. Start 20 rounds only after current
pressure is proved. In each round check Forgejo's health page and
then its public list of repositories. Each probe must answer within
two seconds and needs no sign-in. Wait two seconds after a round
finishes before starting the next.

The legacy script uses one minute of pressure and a fixed delay
after its start marker. Those values do not prove a covered
sequence. At the per-probe deadline the two probes per round and
nineteen round gaps alone can take 118 seconds. Qualification needs
a profile covering the complete bounded sequence and runner
overhead within the existing one-place resource limits. Keep an
unproved duration or admission held.

For each probe, record its start and finish on the same monotonic
clock as the pressure observation. Bind that observation to the same
run, Dell boot, queue place and exact owned scope throughout. Accept a
sample only when its whole interval is covered by verified continuous
pressure from that job's eight workers and memory workload while it
holds its place. A start marker and final exit code alone leave
coverage unproved. Missing evidence, an interrupted workload or an
out-of-scope sample fails the nightly check. The availability result
belongs only to this nightly check. Timing and queue admission use
their own evidence.
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
### Legacy nightly sweep and current permission

The repository's legacy `deploy/pankti/sweep.sh` uses a size cap for
listed build folders and separate age rules for `aegis-work` copies
and temporary scratch. Its size and age tests do not establish the
current cleanup scope in [AGENTS.md section 11][disk-cleanup-rule].
That rule permits automatic cleanup of eligible legacy Cargo and Go
caches only after its existing no-write interval and with no active
build, push or running program using them. Work copies, evidence,
journals and protected folders stay outside the cache permission.
A temporary name or age alone cannot prove ownership or an
approved cleanup scope.

Verify the exact path and cache class, last build write, current use
and authority independently under QR-246. Hold an unreadable or
ambiguous target. A cache marker, free queue locks, a size limit or an
old directory timestamp cannot supply this proof. An explicit owner
instruction naming protected material follows its own rule. Keep the
free-space admission floor and report unavailable space through its
existing route. The cleanup proof supplies no authority to stop a job,
reap owned work or make an occupied queue place available. The
legacy installer supplies no current route attestation or grant.

Resolve the expected timer and service from the current admitted
installation receipt and policy. An older valid receipt cannot
set current expectations after restore. If that identity is unproved,
report individual observations without qualifying the route. Compare
the expected units with those loaded by the service manager. Bind
the unit names, timer target, main files and ordered drop-in
overrides, effective command and
arguments, account and owned service scope. Bind the current executable
path, byte fingerprint, file type, mode and owner. Check custody of the
path and its parent directories under the expected identity. If files
on disk differ from the loaded units, record the difference.

An enabled or active timer and a matching executable fingerprint
identify observations of the route. They give no evidence of a scan,
cleanup effect or permission. Leave unproved effective schedule fields
unknown. A separately observed next time is a dated timer forecast.
After recovery, recheck the effective units and executable. Resolve any
prior attempt and its owned scope from current evidence. A changed or
unreadable unit, path, executable or owned scope leaves that part of the
observation unproved. The legacy profile's size, age and lock checks
cannot qualify as the required per-target write and use evidence. This
check supplies no authority to trigger, change or stop a timer or
service.

[disk-cleanup-rule]: ../../AGENTS.md#11-the-shared-computer-dell

## Limits

- The bootstrap queue can free a place when its holder is lost while
  the child job continues in its capped scope. A freed lock and the
  slice cap supply no proof of an idle place or exclusive timing.
  For protected acceptance, verify the exact prior scope and all
  its owned work through the admitted queue observation. Hold proof
  for the affected place while owned work remains or its disposition
  is unknown. A timed benchmark needs current evidence that both
  places and their prior owned work are clear. Once that evidence is
  verified, recheck admission for the next job under the same limits.
  This proof rule gives no new process control authority.
- pankti does not decide which jobs are heavy. The rule file and the
  runner set-up name them.

## Tests

- A third job waits for a free place, and starts when one is freed.
- A timed benchmark takes both places, so no other job runs beside it.
- A timed benchmark waits for both places, and a later job cannot jump ahead
  of it.
- Places are free again after jobs end. With a synthetic queue state,
  lose the holder while a child of the exact owned unit remains live. Offer
  the free lock to a waiting one-place job for the affected place and
  a timed benchmark. Neither can use it as protected admission proof.
  In separate cases make the scope unreadable or offer an observation
  from another scope. Each withholds acceptance. With verified current
  termination for the exact scope, recheck the original admission
  conditions. A timed case also requires the other place clear. Keep
  prior proofs and perform no real process changes in this fixture.
- A missing folder is an error.
- `IT-0059-NOISY-PRESSURE`: on the admitted Dell route, require all
  20 rounds on time with both probes covered by the exact live
  one-place pressure profile. In a synthetic run, let a probe start
  under full pressure and complete after the workload ends, with
  every probe on time and the queue wrapper exiting zero. Repeat
  with a start marker before full pressure, a memory worker that
  produces only a partial load, and an observation from another
  scope under the same limits. Each case fails the nightly check
  with its existing alarm outcome and yields no availability pass.
  In the near-deadline control keep both probes per round in time.
  End pressure at the legacy minute while later rounds continue.
  The whole run fails on missing pressure even with fast replies.
  Keep proof of the queue and any independent timing run separate.
- The job command carries the slice, the memory cap, no swap, the CPU
  cap and the user's own user and group.
- A sealed job cannot reach the network, use sudo, see `/run/user`,
  start a user service, or write its working folder or cargo's home, and
  the full test suite passes in one (proved live on Dell, and by the
  `a_sealed_job_has_no_network_no_new_rights_and_little_to_write` test
  of its command line).
- Check the effective limits of the current job unit and its parent
  against the ordinary or timed profile. Hold on stale or unreadable
  boot, unit, parent or limit evidence. In the synthetic fixture keep
  the arguments and admitted job fixed. Offer another unit, omit CPU
  or memory read-back, and show effective swap enabled in separate
  cases. Refuse each resource proof. Exact current memory, CPU and
  no-swap evidence clears only this control. Occupancy and timing
  still need their own proof.
- Free space: `check_space` refuses below 50 GB and passes at it, and
  `free_bytes` reads `df` (tests). The historical size-and-marker sweep
  fixture supplies no current cleanup authority.
- In a synthetic cleanup acceptance case, keep the low-space pressure
  and free queue locks fixed. Offer a large recently written legacy
  cache, an older `aegis-work` copy and protected evidence nested under
  a cache marker. Separately give an old cache in use by a build, push
  or running program, and one whose use cannot be verified.
  Refuse each without any delete, move, chmod or process action. Use a
  separately identified legacy cache with proved inactivity, no use
  and matching authority to exercise only the eligibility decision.
  In a separate run let that cache become active after a positive
  preflight and before the removal boundary. Hold it with no delete.
  Require the same target and current no-use proof at that boundary.
  Hold one clock tick before the required no-write interval and
  allow the eligibility check at equality only with verified current
  no-use and object class proof.
- In a synthetic installed-route observation, match the expected and
  effective timer and service from the current approved installation.
  Verify command, executable fingerprint, mode and custody. Show
  the observed identity and timer state only.
  Keep the executable bytes unchanged while a drop-in changes the
  effective command. On separate recovery runs keep the effective
  units unchanged and change the executable mode or leave the prior
  owned service scope unproved. In another case restore an older
  valid installation and its matching expected receipt after the
  policy advanced. Hold qualification. Withhold proof for the
  affected part of each observation. No fixture runs the sweep, changes a unit,
  deletes files, controls a process or proves a queue result.
