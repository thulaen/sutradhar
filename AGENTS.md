# Agent rules for Sutradhar

These rules express the owner's agreed requirements. They apply to every agent,
including Codex, Claude Code and Gemini CLI, wherever it runs. They do not claim
that enforcement tooling is already installed.

## Speak plainly

Talk to the owner in plain English. Use British spelling. Explain an unfamiliar
technical term when it is necessary. State what changed, what was checked, what
the evidence proves, and what remains unfinished. When a step fails
unexpectedly, tell the owner first, before doing anything else (R119's
notes). Never make the owner learn Git commands to approve work. Never
claim a specification, source file or green focused check proves the
complete product works.

## Start from the current state

1. Read this file and the relevant [developer guide](docs/dev/index.md).
2. Inspect the live checkout, staged changes, active tasks and file
   reservations.
3. Check that the authorised Dell development host is available. If it is not,
   pause all project work, including cloud work, and report the pause.
4. Inspect the AutoIssues queue once it is installed. An open P0 blocks
   unrelated work; work needed to fix it remains allowed. Handle blocking
   P1/P2 issues in the affected component under the approved policy.
5. Identify the approved specification and acceptance criteria for the task.
6. Reserve a non-overlapping writing scope before editing shared files.

Never reset, clean, overwrite or discard another person's work. Use one writer
per file. An agent stopping does not silently transfer ownership. Record a safe
handover, preserve its draft and invalidate its old reservation before another
writer starts. A private SQLite run ledger was created on the failed project
drive; its retained contents and paths are unverified. Reverify or reconstruct
it with explicit provenance before relying on its old assignments. Until then,
record fresh, narrow manual reservations on the approved replacement storage.
Automated reservations, lease enforcement and write fencing are not yet
qualified. Do not claim manual records prevent overlapping edits or stale
writers.

## Specifications before implementation

The owner approves each feature specification before coding begins. Protected
requirements, privacy rules, required checks, release rules and trusted-agent
admission also need the owner's explicit approval before they change. For
changes to the gate itself (its rules, hooks, runner and GitHub settings)
the owner approves by answering a question in the session before the push
(R116). Unattended publication of ordinary work does not remove these
requirements.

Preserve requirements and their history. Mark an earlier decision as superseded
and link its replacement; do not quietly delete a difficult requirement. Keep
private source material out of public history. A public copy must redact private
details without dropping the substantive requirement.

The complete first release requires at least 6,000 independently qualified,
distinct methods and all agreed release scope. Parameter changes, renamed
copies, empty placeholders, untested research entries and rearranged presets
do not count.
Incremental work may be committed, but must remain labelled incomplete until the
full release conditions pass. Do not replace the target with a smaller product.

## Where work runs

The owner has authorised a dedicated `sutradhar` directory on the validation
host's existing data SSD, replacing the failed project drive. The exact private
location is in the host preservation notice. This is project-owned permanent
storage, never a cleanup target for another project or agent. Do not delete,
rename, truncate, overwrite or weaken protection of preserved content without
the owner's explicit instruction identifying the affected material. Keep saved
foundation versions locked; create new versions for authorised work. Neither
age, disk pressure, test cleanup nor a failed run grants deletion permission.
Read [storage preservation](docs/dev/storage-preservation.md) before using it.

All work happens in a Claude session running on Dell, in the Dell working
copy; nothing is edited on the Windows computer or copied between the two
(R119, [ADR-0005](docs/adr/0005-local-publication.md)). The Windows computer
only runs the app that connects to Dell. Do not build, run tests, install
dependencies, create caches, run containers, store test evidence or make
backups there. Dell runs private-data and final
performance tests. Isolated cloud workers may check public source and synthetic
fixtures.
Cloud containers are allowed; starting the development container on the Windows
computer is not.

Do not put the owner's forum name, domain, real posts, member data, private
paths, host addresses, drive identifiers, licensed archive names or secrets in
the repository, public logs, previews, screenshots or published artifacts. Use
neutral examples. Public aggregate results require a privacy review. Full
private datasets remain on Dell. The specifically approved redacted AI judging
packs are a separate exception; provider terms and account controls must be
verified first.

## Trunk-based development on main

Use trunk-based development in its strict form: `main` is the only branch.
Integrate small, complete, checked changes into `main` often. Do not create
feature, fix, release, temporary or personal branches, extra Git worktrees,
pull requests or a second branch named `master`. Keep unfinished user-visible
behaviour unavailable, and labelled as such, instead of holding it on a branch.
Outside contributors may open pull requests from their own forks; agents
review them, and the publisher imports an accepted one as a single commit
through the gate.
This rule and its history are recorded in the [30 October requirements
update](docs/requirements/updates-2026-10-30/README.md) and its
[publication update](docs/requirements/updates-2026-10-30/publication.md).

Commit once per step of the build sequence, at turning points: a verified
gate, a working lifecycle, a tested adapter group, a complete capability with
its documentation, or an approved specification. A small critical fix may be
a milestone. Do not commit single sentences, empty scaffolding, repeated
status notes or unfinished fragments. Push each checked milestone promptly.
Never rewrite published history to tidy it.

With several agents, one Git coordinator alone commits and moves Git state.
Other agents own non-overlapping files and hand their checked work to it.

Agents commit in Dell clones and push `main` only to the gated bare repository
on Dell. Its pre-receive hook runs the complete gate on every new commit and
refuses anything else; only its publisher account holds the GitHub deploy key
and pushes `main` to GitHub
([ADR-0005](docs/adr/0005-local-publication.md)). Coding sessions never receive
the deploy key. The Windows computer's GitHub sign-in is used once, for the
setup the owner authorised, and then reduced to read-only (R113); it is never
used to publish. Never push to GitHub yourself, force-push, delete `main`,
bypass checks or create release tags from a coding-agent session. Until the
route works nothing is published: do not fall back to raw write credentials,
the owner's personal login or temporarily disabled protection.

Before an ordinary agent commit, run the full applicable pre-commit profile
through the planned single entry point, `just check`. This command is a required
future interface, not permission to claim a pass before the justfile exists.
The clone's pre-commit hook enforces only the fast part; the gated
repository's pre-receive hook enforces everything.
Until then follow the documentation profile in the
[documentation bootstrap](docs/dev/bootstrap.md); its branch and pull
request steps are replaced by the gated Dell repository. Missing, stale,
skipped or incomplete evidence is not a pass.

Dependabot and Release Please normally work through branches and pull
requests, which main-only development forbids. Their earlier pre-check
exception is suspended until the owner approves a main-only replacement.
No bot, including AutoIssues, gets an unchecked commit exception.

Every commit and push also requires: test-driven development evidence for new
behaviour and fixes; the source limits of 300 physical lines per first-party
text file, 60 lines per function and 80 characters per line; the 20×
performance gate result; and the AutoIssues queue and quota checks. These are
agreed requirements; their enforcement is not yet installed or qualified.

Code, performance and security reviews are mandatory. Self-review is allowed
only when honestly identified, supported by evidence and reviewed for quality.
The additional required AI review must be complete for the exact current
candidate; silence, an old review or a reaction is not approval. See
[gates](docs/dev/gates.md); its merge stage is replaced by the gated Dell
repository (ADR-0005).

If a published change breaks `main`, publish a revert commit through the
gated Dell repository as the first response, then fix forward.

## Implementation boundaries

- Use PHP 8.5 for the XenForo integration and the approved Python analysis
  service.
- XenForo integration requirements win at its boundary; framework design may
  take inspiration from Spring Boot without pretending to be Spring.
- Python receives approved data through PHP. It does not access the XenForo
  database directly. Changes go through XenForo's own application mechanisms.
- V1 linking is moderator-approved only. Wrap existing words; never rewrite,
  insert or remove post text to make an anchor. Recheck content and permissions
  at application time. Preserve subsequent edits during undo.
- Keep the production Python service and every worker belonging to it within the
  agreed combined 6 GB limit (R021 as amended in the
  [models update](docs/requirements/updates-2026-10-30/models.md)). Separately
  admitted extension services require explicit declared allowances and
  inclusion in published resource accounting; moving a service worker
  elsewhere cannot evade its shared cap. The separate Dell lab profile has a
  9 GB combined limit. PHP contributes at most 8 MB additional peak memory per
  request; this is not a 6 GB limit for an entire forum deployment.
- Use bounded work, durable checkpoints, safe retries and real recovery tests.
  Never treat a process memory limit as proof of graceful recovery.
- Use only admitted dependencies, models, datasets and independently implemented
  methods. Unresolved rights or patent concerns block admission.
- First-party text files, including documentation, tests, tooling and
  generated source, have at most 300 physical lines and 80 characters per
  line; functions, methods and closures have at most 60 lines. Keep code
  simple, avoid duplication and follow the approved rulebook.
- Do not introduce Fossil, Sapling, Buck2, Bazel, Pants or Nix.

## Names and documentation

Use only approved glossary names for system parts. Keep internal methods,
variables, domain classes, configuration keys, log fields, metric names,
database names and standard interfaces in English. New system names need
approval; use an English placeholder until then. Do not invent religious or
political names.

The approved names are `sutradhar`, `sutra`, `vinyas`, `mandali`, `sanyojan`,
`sanchar`, `karya`, `bhandar`, `sujhav`, `lekha`, `drishti`, `dwarpal`,
`pariksha`, `sampark`, `shodhan`, `namaste`, `setu`, `sanket`, `panchayat`,
`shikhar`, `karma`, `uttar` and `kavach` (spam protection; "armor"). A system
module class may use its approved name, such as `Setu(Module)`; this does not
rename internal domain classes.
User interfaces lead with English and show the glossary name alongside.
Package, module and command help explains the name's meaning at first use.

Use G3Doc-style Markdown with clear ownership, status, purpose and links. Keep
one home for each explanation. Method notebooks own method-level explanations
and import production functions; developer documents own architecture and rules.
The public manual is for forum owners, administrators and moderators, not for
framework internals. A user-visible change must update the affected manual page.

## Code Review Rules

### Protect forum content

Reject paths that expose non-public forum content, use stale visibility
decisions, alter words during linking, bypass XenForo writes, lose later edits
during undo, or hide incomplete method runs behind a successful suggestion.

### Protect evidence and authority

Reject review or test evidence that does not identify the current content and
configuration. Look for ways proposed code could forge checks, access private
corpora or credentials, weaken rules, impersonate owner approval, or publish an
unqualified release. Put routine mechanical checks in the test pipeline.

### Protect resource and recovery limits

Examine worker totals, native-library allocations, queues, retries, model
loading, checkpoint commits and worker replacement. A small passing example does
not prove the million-post workload or the release memory and recovery
requirements.
