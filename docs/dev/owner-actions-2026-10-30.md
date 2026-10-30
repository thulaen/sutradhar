# Owner actions for main-only publication

Audience: the project owner. Outcome: GitHub and Dell are ready for the
approved main-only publisher (SPEC-004), without anyone using Git
commands. Status: superseded in part later on 30 October 2026. The owner
chose publication through a gated repository on Dell
([ADR-0005](../adr/0005-local-publication.md)) and asked agents to carry
out the setup (R113). Sections 1 to 6 are kept as history and are not
asked of the owner: agents do the work, adapted to the ADR (there are no
attester or approval-verifier Apps or accounts, and no test repository).
Section 7, the reviewer sign-in, can only be completed by the owner and
stays open.

Do the steps in order. Each step says what to click and why. Nothing
here asks you to type a Git command. Settings wording on GitHub may
differ slightly; if a step does not match what you see, stop and ask.

## 1. Read-only access on the editing computer

Why: agents work on this computer, and the approved design relies on
your GitHub login there being unable to post comments or change
settings.

1. GitHub, profile picture, Settings, Developer settings.
2. Note which token or app the editing computer uses for Git (often
   "Git Credential Manager"); replace it with a fine-grained token
   limited to this repository with Contents: read-only and Metadata:
   read-only, or ask for help doing this safely.
3. Do not stay signed in to GitHub in browsers that agents can control.

## 2. Temporary test repository

Why: GitHub cannot dry-run rules on this plan, so refusals are tested
on a throwaway public repository first.

1. GitHub, New repository, name it
   `sutradhar-publisher-qualification`, public, with a README.
2. Leave it empty otherwise; agents will describe each test before it
   runs. You may delete it after qualification.

## 3. Apps

Why: separate identities keep publishing, check reporting and approval
apart.

1. Settings, Developer settings, GitHub Apps, the existing proposal App,
   Permissions: set Contents to Read and write, Metadata to Read-only,
   and every other permission (including Pull requests) to No access.
   Save; accept the change on the installation page if asked.
2. In the same App, Private keys: delete the older unused key (the one
   whose fingerprint is not the working key; agents can tell you which).
3. New GitHub App "Sutradhar attester": no webhook; permissions Commit
   statuses Read and write, Checks Read and write, Metadata Read-only.
   Install it on `thulaen/sutradhar` and the test repository only.
4. New GitHub App "Sutradhar approval verifier": no webhook; permissions
   Commit statuses Read and write, Issues Read and write, Metadata
   Read-only. Install it on the same two repositories only.
5. For each new App, generate one private key. The download lands on
   this computer: tell the coordinator immediately so it is moved to its
   protected Dell location and removed from this computer.

## 4. Rules on the test repository first

Why: proves the design before touching Sutradhar.

1. Test repository, Settings, Rules, Rulesets: create the three rulesets
   described in the SPEC-004 settings document ("protected main",
   "main publisher", "no other branches").
2. Settings, General, Features: turn off Pull requests.
3. Tell the coordinator; tests Q1 to Q6 then run and are reported to you.

## 5. Rules on Sutradhar after the tests pass

1. Settings, Rules, Rulesets, "Sutradhar protected main": remove
   "Require a pull request before merging"; bind each required check to
   its App; add the `AutoIssues gate` and `performance gate` checks only
   when the coordinator reports that those gates have activated.
2. Create "Sutradhar main publisher" and "Sutradhar no other branches"
   exactly as on the test repository.
3. Settings, General, Features: turn off Pull requests.
4. Settings, Code security: keep Dependabot alerts on; turn Dependabot
   security updates and version updates off.

## 6. Dell accounts and directories

Why: the approved trust boundaries need separate service accounts and a
live-data area. These change the host's security setup, so you either
perform them or tell the coordinator explicitly to do each one.

1. Create the coding-agent, runner, gate, QA, publisher, attester and
   approval-verifier accounts.
2. Create `live/` and `snapshots/` inside the Sutradhar project directory
   with the owners described in the live-data storage design.
3. For the check runner (added 30 October 2026): allow only the runner
   account to start sandboxed checks (a narrow rule for `systemd-run`
   with the runner's fixed properties), then have that account create
   its own signing key. Until then the runner is started as root and
   signs with a qualification key, which is recorded as interim.

## 7. AI reviewer

Sign in Claude Code on Dell under the reviewer account when the
coordinator asks; the sign-in screen is yours to complete.
