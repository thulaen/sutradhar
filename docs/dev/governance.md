# Governance and evidence

Audience: contributors and agents. Outcome: know who may decide, write, approve,
merge and publish, and what evidence each action needs.

Status: contract derived from the interview. Automated enforcement remains
pending until the implementation and adversarial checks prove it works.

## Authority

The owner's direct instructions and recorded decisions govern this project.
Signal Controller is a source of selected design and governance ideas, not a
second authority over Sutradhar. Its infrastructure, language restrictions,
repository authority and unresolved specifications do not transfer
automatically.

The requirements register preserves every interview answer and names the current
replacement when answers change. A later specific approved answer wins over the
earlier conflicting answer. An agent must report a newly discovered
contradiction instead of silently choosing a convenient rule.

- Action: Research and draft a specification
  - Authority: Assigned agent or contributor
- Action: Approve a new feature specification
  - Authority: Owner, explicitly, for that revision
- Action: Implement an approved specification
  - Authority: Assigned agent or contributor
- Action: Change requirements, privacy, gates or release rules
  - Authority: Owner approval required
- Action: Admit a trusted contributor or alter trust policy
  - Authority: Owner approval required
- Action: Commit ordinary agent work
  - Authority: Agent, after full applicable checks
- Action: Create Dependabot or Release Please proposals
  - Authority: Recognised bot; narrow pre-commit exception
- Action: Merge ordinary trusted work
  - Authority: Protected automation after all merge conditions pass
- Action: Merge outside work
  - Authority: Owner decision after all merge conditions pass
- Action: Create release tags and publish
  - Authority: Restricted release automation after the appropriate gates
- Action: Insert a link into a forum post
  - Authority: Authorised moderator approval through XenForo

Owner approval of a specification is distinct from merge permission and from a
test pass. Agent self-review may satisfy an engineering review when honestly
labelled; it cannot stand in for the owner's protected decision.

## Identity separation

Use dedicated GitHub App identities for automation. The owner's personal account
must not be impersonated by an agent to approve the agent's own policy changes.
Keep App signing keys and all App write tokens in a trusted publisher on Dell.
Coding sessions submit bounded branch/PR operations or patches through its
proposal interface; they do not receive the underlying credentials.

GitHub's contents-write permission also authorises release operations. A token
with that permission is therefore not a branch-only capability, even when tag
rules prevent creating tags. Do not rely on tag protection to stop release API
operations. [GitHub release API
permissions](https://docs.github.com/en/rest/releases/releases#create-a-release)

The proposal interface exposes no product-release API and cannot accept an
arbitrary GitHub endpoint, shell command or credential request. Separate
protected identities report checks, verify owner approvals and perform release
publication. They verify the exact candidate themselves. Never expose their
credentials to a pull request's executable code, a coding session or an
untrusted agent adapter.

This publisher boundary is covered by approved SPEC-003 but still requires
qualification; it is not an installed capability. Record the exact installed
identities and permitted operations privately and test rejection of disallowed
operations. No agent gets repository-ruleset bypass as a convenience.

## Shared work

A private SQLite ledger was created on the failed project drive for manual
assignments. Before the incident, the coordinator reported four task records,
WAL mode, full synchronous writes and a successful integrity check. Those are
historical results: retained contents and paths require re-verification or
reconstruction before the old records can be used. Fresh setup reservations are
recorded separately on approved replacement storage. None of these manual
records proves automated reservations, leases, fencing or multi-agent recovery.

The complete ledger contract records task identity, assigned writer, reserved
files, specification revision, checkpoints, content identity, review and test
evidence, status and handovers. Do not commit the database or private evidence.
Commit only safe summaries that have passed privacy
checks. Fields or behaviours in the contract that the initial manual ledger does
not yet implement remain unqualified.

An expired reservation does not authorise overwriting a draft. Preserve the
draft, confirm the previous worker can no longer write, record the handover,
then grant the next reservation. Reject writes from an old reservation after
reassignment.
Checkpoint a completed unit only after its durable effects have committed.

Agents join explicitly through supported interfaces. Approved scheduled jobs,
such as release judging and the manual gardener, use their own narrowly defined
authorisation. The A2A protocol does not itself prevent overlapping writes or
prove that every desktop client supports the required interface.

## Availability and storage

All development pauses when Dell is unavailable, including cloud tasks.
Deadlines that depend on unavailable infrastructure pause and must be reported
honestly.
This is a development policy; it must not add a dependency on the private Dell
host to installed forums.

The approved dedicated development storage holds project runtimes, dependencies,
caches, tests and private evidence. On 30 October 2026, the owner replaced the
failed drive with a dedicated project directory on the existing data SSD. Only
that directory is newly authorised; neighbouring projects and the
operating-system disk remain outside scope. Follow
[storage preservation](storage-preservation.md).
Public documents use logical storage roles, not identifying
hardware details. Before destructive storage work, recheck the authorised
device, current mounts, dependencies and the user's preservation decision.

The Windows computer only runs the app that connects to Dell; all work happens
in a Claude session on Dell (R119). The development container runs on Dell or
an approved isolated cloud worker, not on the Windows computer. Private test
corpora can be rebuilt from an authorised forum export after storage loss;
rebuilding creates a new dataset version and requires qualification again.

## Evidence

An evidence record identifies the source content, proposed change, tool and rule
versions, configuration, admitted dependencies, dataset or synthetic-fixture
version, execution environment, timestamps, result and retained artifact hashes.
Review evidence also identifies the reviewer, scope, findings, disposition and
whether the review was independent or a self-review.

Before a commit exists, pre-commit evidence binds to the proposed content tree
and its full execution inputs. After the commit is created, verify the
committed tree is identical and bind the receipt to the resulting commit.
GitHub merge checks must cover the current proposed merge with the up-to-date
target branch. A change to source, base, rules, dependencies or relevant
configuration invalidates affected evidence; a timestamp alone cannot make it
reusable.

Keep source correctness, rights admission, test execution, runtime installation,
release qualification and actual publication as separate verdicts. Neither a
catalogue entry nor a test proves mathematical uniqueness, usefulness, legal
clearance or complete feature coverage by itself.

## Reporting progress

Use the acceptance ledger to report the entire requested scope, including
missing work. Do not report a smaller internal milestone as the complete
release. Reports to the owner must lead with the outcome in plain English and
state material gaps.
Public reports must not identify the private forum or disclose raw test data.
