# 0155: the agent harness

Agents have sudo on Dell and can read any token there, so a token an
agent holds is a token it can leak or misuse. A managed agent runs
under a locked account with no token
([AGENTS.md section 11](../../AGENTS.md#11-the-shared-computer-dell),
task #166). The harness is the program that holds the tokens and acts
for it. This spec designs it. It follows DR-161 and DR-162 in
[ADR-0008][accounts].

[accounts]:
  ../adr/0008-forgejo-and-github.md#agent-accounts-and-credentials

Language. The harness is an operations tool, written in Rust
(ADR-0019, AGENTS.md section 5.1).

The harness adds a second wall in front of the door, not a
replacement. The door (spec 0001) still checks every push and every
tag, and the harness never skips it. The owner records the door as
installed before any token is issued to the harness (QR-254).

## Accounts and identity

- Each agent has its own locked account, with no sudo, no Docker or
  LXD group and no token. The owner makes the accounts. The harness
  reads one owner-written file, outside every agent's reach, that maps
  each account to one mailbox name, one author name, one committer name
  and one clone path (up to ten at once, AGENTS.md section 8.2). The map
  is `/etc/sutradhar/harness/accounts.toml`, written by root and read
  only by root and the harness account, and DR-167 names the first
  accounts. The harness can read that clone path and nothing else of the
  agent's.
  Helpers at any depth share their parent's account, so they share its
  names and its locks.
- The socket lives in a folder that only those accounts and the harness
  can reach. The harness learns the agent from the operating system's
  record of who is on the other end of the socket, never from a name
  in the request. A name in the request is ignored.
- The harness runs under its own account. Only that account and the
  owner can read its folder, its token store and its log.

## Requests

An agent sends one small typed record. The harness parses it with a
size limit and refuses an unknown field, an unknown request or a value
that does not match its pattern. A ref, path, task number or author is
checked against a strict pattern (for example a ref may not start with
`-` and a path may not hold `..`) and is passed to git and to Forgejo
only as a separate argument or a built address, never through a shell.

| Request | What the harness does |
|---|---|
| `push` | Moves `main` forward by one named commit, or adds a release tag |
| `task` | Reads, makes, labels or comments on a Forgejo task |
| `mailbox` | Passes one allowed mailbox call as the agent's own name |
| `status` | Says whether the harness is paused and its limits |

There is no request that runs a command, reads a file or returns a
token. Every request carries a random id. The harness keeps the ids it
has seen for 24 hours (proposed), in its own store so that a restart
does not forget them, and refuses a repeat, so a replay does nothing.
Ids are random, so helpers sharing an account cannot collide.

## Push

The harness never pushes the agent's clone as it stands. It takes the
one commit the agent names.

1. The harness fetches that commit and its missing history from the
   clone path the owner's file names, never one the request gives,
   into its own repository, with every setting the clone could hold
   turned off: no hooks, no filter or fsmonitor program, no alternates,
   and its own `HOME` and git configuration. Nothing from the clone is
   executed. It checks every object in full, and refuses a pack over
   64 MiB, an object over 8 MiB, a `.gitmodules` file or a tree entry
   that is not a plain file or folder (all proposed). The commit must
   descend from `main`, so the new commits are the ones after it.
2. It checks that exact commit, in its own copy, so the agent cannot
   change it afterwards:
   - **Authorship.** Every new commit has the agent's own author name,
     and a committer the agent's account maps to. A merge, a missing
     name or another agent's name is refused.
   - **Locks.** Every path any new commit changes, including renames,
     mode changes and symlinks, is under a live mailbox lock the agent
     holds, checked again at the moment of the push.
   - **Task.** The commit message has a `Task: #N` line, and the harness's
     own record shows the agent took task N. Mentions elsewhere in the
     message do not count. When two agents ask to take one task, the
     first request wins.
3. It pushes that commit id to `main` with the address and credential in its
   own configuration, never the clone's remote, push address, credential
   helper or `insteadOf` setting. The push names the commit id and the old
   value of `main` it expects, with no force, so it fails if `main` has moved.
   Just before the push it asks the mailbox again whether each lock's fencing
   token is still live, and refuses one with less time left than the push
   limit. It never retries by itself.
4. The harness writes a line to its log before the push and another
   after it, so a crash between them leaves a push that is known to be
   unfinished. If the push ends without a clear answer, such as a time
   limit, it reads `main` from Forgejo and answers `landed`, `not
   landed` or `unknown`; it never says failed for a push the door may
   still finish.
5. The harness does not judge the content of the change. It adds no
   list of protected paths: a change to the gate, the door, chaukidar,
   the CI files or the hooks is the door's and the gate's to refuse.
   The commit tier, the reviews and the clean-copy check of AGENTS.md
   section 8.2 stay the agent's duty, and the door runs the push tier
   again.

A release tag is a second push, after `main` already holds its commit. The
harness makes the tag itself, with a fixed message, so an agent cannot supply
the tag object. It is refused unless the signed `release-approval` receipt of
spec 0005 exists for that exact commit and tag name, verified with the owner's
public key and read from the evidence store that the harness account can read
and agents cannot write (QR-256 decides whether a scoped release agent's
signature also counts), the tag is new and the commit is on `main`. A force, a
delete, another branch and a changed tag are refused.

One push runs at a time, in the order asked. An agent may have one push
running or waiting; a second request is refused until that one ends, so
one agent cannot hold the queue.

## Tasks

The harness uses only the Forgejo calls in this list, each as a fixed
path and method. It never calls a path for settings, users, hooks,
tokens or repository administration.

| Call | Allowed |
|---|---|
| Read a task, its comments and labels | Yes |
| Make a task, with a title, body and `status/blocked` | Yes; it goes to "Later", and is never ready |
| Make an urgent task | Only with one of four reasons (broken rule, failed sync, failed release, security) and while fewer than 3 are open (proposed); it goes to the current phase (AGENTS.md section 8) |
| Comment on a task | Yes, on any task, a closed or alarm task included |
| Set `status/working` or `status/blocked` | Only on an open task the agent took |
| Set any other label, or `status/urgent`, on an existing task | No |
| Mark a task `status/ready` | Only an open current-phase task nobody took, and only while fewer than 10 are ready (AGENTS.md section 10) |
| Take a task | Only an open current-phase task, not taken by another agent, and only while the agent holds no other open task; the harness writes the agent's mailbox name in a comment |
| Close a task | Only an open one the agent took, and never an alarm task |
| Assign any task | No |
| Edit or delete a comment, delete anything | No |

A comment the harness writes starts with the agent's mailbox name, so
a reader can tell agents apart although they share one Forgejo user.
An alarm task is one with the label `alarm`, which the harness never lets an
agent add or remove, or with the title of chaukidar (spec 0058) or the sync
(spec 0002). Chaukidar and the sync give every alarm that label, and the
harness refuses a task an agent makes with an alarm title. An agent may read
an alarm task but never close or relabel it. The labels `status/urgent` and
`status/ready` follow AGENTS.md sections 8 and 10: an agent may set them only
where those sections allow.

## Mailbox

An agent touches only its own reservations and messages: a release,
renew or acknowledge that names another agent's id is refused. The
harness passes only these calls, each as the agent's own name:
reserve, renew and release the agent's own locks; check its write
rights, list reservations and list participants; check and acknowledge
its own messages; send a message; and read a mailbox task. It refuses
posting, claiming or completing a mailbox task for another agent, and
releasing another agent's lock.

## Limits (all proposed)

| Limit | Value |
|---|---|
| Request size | 64 KiB |
| `push` per agent | 12 an hour |
| `task` per agent | 240 an hour |
| `mailbox` per agent | 600 an hour |
| Open connections | 16 in all, 2 per agent |
| Time to read one request | 10 seconds |
| Refused requests per agent | 120 an hour |
| Time for one Forgejo call | 60 seconds |
| Time for one push | the door's own time (spec 0001) and 60 seconds |
| Harness memory | 256 MiB, counting the git programs it starts |
| Harness repository | 1 GiB, cleaned up daily |

A request over a limit is refused with the limit named. A hung call
cannot hold up the others: each runs under its time limit.

## The log

Every request, its agent, its answer and the reason for any refusal is
written to a log that agents cannot change. The log is append-only and
each line holds the hash of the line before, so a cut or a changed
line shows. Its newest line is copied after every request to a second
place only the owner can read. Forgejo is reached over the loopback
address. The log never holds a token, a request body that carried
one, or the text of a commit. The log has a size limit (64 MiB for
each of 10 files, proposed) and its own space that no agent account
can write to, so an agent cannot fill it. Refused requests are
summarised once a minute for each agent, so a flood cannot push earlier
lines out. A failed log write stops the harness.

The watchman (spec 0058) fingerprints the harness program, its
service settings and the owner and mode of its token store. It cannot
fingerprint the log, whose head changes with every request. The
harness has a command, `agent-harness verify-log`, that checks the
chain and compares its newest line with the owner-only copy; the
watchman's timer runs it and raises an alarm on a break.

## Tokens

- The harness holds one token for Forgejo (`write:repository` and
  `write:issue` only) and the mailbox token for each agent. It never
  holds the GitHub tokens of forge-sync (spec 0002), or the release
  key.
- The store is encrypted. A key sealed to this host unlocks it when the
  service starts, so a restart needs no one to type a secret (QR-257).
  Only the harness account and the owner can read it.
- A token is kept in memory that is locked and cleared after use, is
  never in the process environment or on a command line, and the
  harness refuses to dump core. It is given to git over a private pipe
  and never written to a file, a log, an error or an answer.
- The owner makes and rotates tokens. Forgejo gives a token no expiry,
  so rotation is a recurring task in Forgejo (every 90 days, proposed).

## Pause and failure

- The owner pauses all agents with a file only the owner and the
  harness account can write. While it exists, or when the log cannot be
  written, the harness refuses every request and says why.
- The harness fails closed: if it cannot check a request, it refuses
  it. A crash leaves no half-made push, because a push is Forgejo's own
  all-or-nothing update, checked again by the door.
- `status` tells an agent the limits and whether the harness is paused.
  An agent can probe the rules this way; that is accepted, since the
  rules are public in this spec.

Sessions the owner starts under the shared account that has sudo are
unmanaged and never described as contained (AGENTS.md section 11).

## Requirements and module

The module is the crate `agent-harness` (proposed), an operations tool.

| Requirement | Acceptance |
|---|---|
| REQ-0155-IDENTITY | IT-0155-IDENTITY and IT-0155-MAILBOX MUST show the agent comes from the socket and no other agent's locks or mailbox tasks are touched |
| REQ-0155-PUSH | IT-0155-PUSH, IT-0155-CLONE, IT-0155-AUTHOR and IT-0155-LOCKS MUST show only a checked forward push of one named commit reaches Forgejo, with the harness's own address and credential, and that a tag needs a genuine approval receipt |
| REQ-0155-TASKS | IT-0155-TASKS MUST show alarm tasks and other agents' tasks are untouched |
| REQ-0155-SECRETS | IT-0155-LEAK MUST show no token appears anywhere an agent can read |
| REQ-0155-LOG | IT-0155-LOG and IT-0155-PAUSE MUST show the log shows tampering, an agent cannot fill it or pause the harness, and a failed write stops it |
| REQ-0155-LIMITS | UT-0155-REQUEST and PF-0155-RATE MUST show every limit at, below and above it |

## How it is tested

Designed, not run. Every test uses a fake Forgejo, a fake mailbox and
fictional agents.

- **UT-0155-REQUEST:** every request type; an unknown field, an
  oversized request and a bad pattern are refused; no request runs a
  command, reads a file or returns a token; a repeated number is
  refused.
- **IT-0155-IDENTITY:** the agent comes from the socket, and a name in
  the request, or another agent's locks, are ignored or refused.
- **IT-0155-PUSH:** a forward push passes; a force, a delete, another
  branch, a moved `main`, a merge, a changed tag and a tag off `main`
  are refused; a tag before its approval, with a forged receipt, with a
  receipt for another candidate or with a receipt an agent wrote is
  refused.
- **IT-0155-CLONE:** a clone with hostile hooks, a `core.hooksPath`,
  an fsmonitor or filter program, alternates, symlinks and a changed
  remote or credential helper runs nothing and sends no token anywhere
  but the configured address; refs changed between the check and the
  push do not change what is pushed.
- **IT-0155-AUTHOR, IT-0155-LOCKS:** another agent's name, no name, an
  expired lock and a rename or mode change on an unlocked path are
  each refused.
- **IT-0155-TASKS:** an alarm task cannot be closed, relabelled or
  assigned; a task made with an alarm title is refused; another agent's
  task cannot be closed; no comment is edited; comments carry the
  mailbox name.
- **IT-0155-MAILBOX:** another agent's locks and mailbox tasks are
  refused.
- **IT-0155-LEAK:** with a planted token in the store, no log line,
  error, answer, environment, command line or core file shows it, and an
  agent that reads the store, the socket folder or the log is refused.
- **IT-0155-LOG:** `verify-log` finds a cut or changed log line; an agent cannot
  fill the log's space; a failed write stops the harness.
- **IT-0155-PAUSE:** the pause file and a failed log write each stop
  every request, and an agent cannot set or clear the file.
- **PF-0155-RATE:** each limit at, below and above it, and a hung Forgejo
  call under its time limit.
- **Planted faults:** an agent that asks for a settings path, a hook, a
  token and a delete.

## Owner items (design on, product off until answered)

| Item | Question | Why it matters | Default until answered |
|---|---|---|---|
| QR-255 | Should the harness also hold GitHub or release credentials, or only Forgejo and mailbox tokens? | More held tokens widen what one flaw can reach | Forgejo and mailbox tokens only |
| QR-256 | Who may approve the exact candidate a release-tag push needs: only your signed approval record (spec 0005), or also a scoped release agent (AGENTS.md section 8.1 step 3)? | Decides who can release | Only the owner's signed record |
| QR-257 | May the harness's token store be sealed to this host so it restarts unattended? | A typed secret blocks an unattended restart; a sealed key is readable by root | Sealed to the host |
