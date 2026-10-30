# Live data and preserved snapshots on Dell

Status: APPROVED revision of [storage preservation](storage-preservation.md),
awaiting the owner's approval (R096). Nothing has been created.
Audience: the owner, the coordinator and service implementers.
Approved by the owner on 30 October 2026 (see the approval record in
`docs/specifications/approvals-2026-10-30.md`); it supersedes any
"awaiting approval" wording below.

## Problem

The project root on Dell is append-only and its saved versions are
immutable. That protects history, but a database, a run ledger, a QA
service volume or a test workspace must change as it runs. R096 requires
live mutable storage to stay separate from preserved immutable snapshots,
and forbids unlocking preserved work to make anything run.

Measured on 30 October 2026 (exact figures are in the private operator
record): the shared data volume is mostly full, with free space measured
in tens of gigabytes rather than hundreds; the project directory is small.
Python 3.14 and PHP 8.5 are installed; containers use the systemd cgroup
driver on cgroup version 2. A `sutradhar` service account and group
already exist.

## Proposed layout

Inside the project root, one new directory per purpose, created once by
the owner-approved setup step:

- `live/`: mutable working state. Not append-only, not immutable. Each
  subdirectory belongs to exactly one service account:
  - `live/autoissues/`: the AutoIssues database and its write-ahead log;
  - `live/ledger/`: the private run ledger (R046);
  - `live/qa/`: QA service volumes (Grafana stack, GlitchTip, SonarQube);
  - `live/work/`: disposable test and build workspaces, one per run.
- `snapshots/`: preserved copies. Each snapshot is a new dated directory,
  written once, verified, given a SHA-256 manifest and then made immutable.
- Existing preserved versions stay exactly as they are.

`live/` is the only place where files are rewritten. Nothing under
`snapshots/` or the existing versions is ever unlocked, overwritten or
deleted to make a run succeed.

## Deletion inside live data

The owner's preservation rule forbids deletion of preserved content without
an explicit instruction. Live working state is different: a database
rewrites its own pages, and a test workspace is meant to be thrown away.
Proposed rule, for the owner to approve:

- databases and the ledger are never deleted; they change only through
  their own software, and are snapshotted nightly before any migration;
- a test workspace under `live/work/` may be removed by the run that
  created it, after its evidence has been copied into a snapshot; nothing
  else may remove it;
- QA service volumes may be reset only by a recorded `just qa` stage after
  that stage's exported evidence is snapshotted.

If the owner prefers that nothing is ever deleted, `live/work/` grows
without limit and disk capacity becomes the binding constraint; see below.

## Capacity

Snapshots are never pruned without the owner's instruction, so space
only grows. A capacity check runs before each heavy stage and refuses to
start when free space would fall below 50 GB, reporting the shortfall in
plain English. It never frees space by itself; the owner decides.

## Accounts

The account model is defined in the
[trust boundaries](../specifications/trust-boundaries.md): throwaway
sandbox identities for candidate code, and the runner, gate, QA,
publisher, attester, approval-verifier and coding-agent accounts. The
existing `sutradhar` account and group keep their current files and are
not reused for a new role without a recorded decision. The operator
account with administrator rights is reserved for owner-approved setup.

Creating accounts and directories changes the host's security setup, so the
owner either performs it or explicitly instructs it, step by step.

## Acceptance cases

- AT-ST01: a write into `snapshots/` or a preserved version is refused by
  the filesystem.
- AT-ST02: each live subdirectory is writable only by its owning account.
- AT-ST03: a nightly snapshot of the AutoIssues database passes its
  integrity check and matches its manifest.
- AT-ST04: the capacity check refuses a heavy stage below the threshold
  and deletes nothing.
- AT-ST05: the coding-agent account cannot read any App key or write
  outside its granted paths.
