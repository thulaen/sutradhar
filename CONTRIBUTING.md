# Contributing to Sutradhar

Sutradhar is being specified and built. A documented feature is not necessarily
available. Read [the current development status](docs/dev/index.md) before starting.

## Propose a change

1. Describe the problem and the behaviour you want.
2. Link the relevant approved feature specification, or propose a specification.
3. Wait for owner approval before implementing a new feature specification.
4. Fork the repository and work on a branch. Do not push to `main`.
5. Keep the change focused and update the user manual when behaviour changes.
6. Open a pull request with the problem, final behaviour, checks and limitations.

The owner does not need to use Git commands. GitHub pull requests provide the
review and approval interface. Use plain English in discussion and British
spelling in documentation.

## Required checks

The planned single entry point is `just check`. Its full requirements are defined
in [the gate contract](docs/dev/gates.md). During the documentation-only bootstrap,
follow [the bootstrap profile](docs/dev/bootstrap.md); do not invent successful
results for tools that have not been installed.

Ordinary agent commits require the full applicable pre-commit checks. Before a
push, verify that their evidence still applies and rerun the fast suite. GitHub
then performs the additional current-change review and merge checks. Every real
failure blocks progress at the stage where the check is required.

Code, security and performance reviews are mandatory. State whether each reviewer
is independent or is the author reviewing their own work. A reviewer must explain
what they inspected and what their evidence cannot prove.

## Outside contributions

Fork pull requests run only public-code and synthetic-data checks in isolated
workers without write credentials. Their code must not run on Dell until the
owner approves that exact candidate for execution. Private forum data, licensed
XenForo files and production credentials are never provided to a fork workflow.

Outside pull requests wait for the owner's merge decision. Read-only workflow
credentials alone do not enforce this policy; the protected contributor gate does.
Admission to the optional trusted-contributor list is itself an owner-approved
governance change. A new push invalidates approvals tied to older content.

## Trusted automation

Trusted agents may work locally or in approved cloud environments, submitting
bounded proposals through the dedicated GitHub App publisher on Dell. The
publisher keeps App keys and write tokens outside coding sessions; a token with
contents-write permission is not restricted to source branches. Eligible pull
requests use automatic squash merging only
after all checks pass. Changes to specifications, privacy, required checks,
release rules or contributor trust still require the owner's explicit approval.

Dependabot and Release Please may create unverified proposal commits. They receive
no exemption from merge checks. If a merged change breaks `main`, immediately open
a revert pull request instead of a fix-forward pull request.

## Rights and privacy

Write first-party logic independently from approved specifications and admitted
sources. Record source provenance and applicable licences. Do not copy third-party
implementation code into an allegedly independent implementation. A publicly
readable paper, algorithm or patent is not automatically cleared for every use.
Unresolved rights block admission; ask before replacing a specifically requested
library. Keep required notices for admitted dependencies.

Never submit real forum content, member data, forum identity, private addresses,
licensed application archives or credentials. Use synthetic fixtures and neutral
examples. Review generated files, screenshots and logs as carefully as source.
See [security and privacy reporting](SECURITY.md).

## Working safely together

Use one writer per file and a recorded handover. Preserve changes made by others.
Do not reset or clean a shared checkout to make a test pass. Development pauses
when the authorised Dell host is unavailable, even if a cloud worker can continue.
The Windows editing machine must not run builds, tests, dependency installations,
containers or artifact storage.
