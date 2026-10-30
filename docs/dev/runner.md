# Check runner

Owner: the Git coordinator. Status: implemented and checked on Dell;
activation step 2 of the approved order (SPEC-001 runner, the
trust-boundary sandbox and signing). Interim until the owner's account
and host actions are done (below). Standard library only; Dell only.

## What it does

`python3 -E -s -B -m tools.runner.cli run --repo R --tree REV [--base REV]
--profile P --policy FILE --host FILE --key KEY --out DIR
[--evidence FILE]` checks one exact Git tree and writes a signed bundle.
`verify --bundle B --signature S --signers F --identity I --policy FILE
--tree T --profile P --base REV|none` checks a bundle. Exit 0 means
PASSED, 1 FAILED, 2 BLOCKED.

- Candidate: the tree is exported blob by blob, so `.gitattributes`
  export rules, replace references, hooks and the caller's Git environment
  cannot change what is tested. Each blob's identifier is recomputed.
  Links, submodules, `.git` entries, unsafe, option-like or non-UTF-8
  names, missing or corrupt objects and exports over the size or
  file-count limits are refused. Git runs only on a repository the runner
  owns, with a time limit and no fetching.
- Policy: `tools/runner/policy.json` lists checks, profiles, pinned tools,
  the runner's own pin, the bootstrap scope, documentation patterns and the
  maximum evidence age. It is read from the protected copy the caller
  names, never from the candidate, and loaded strictly.
- Profile choice: documentation-only is accepted only when something
  changed and every changed path is documentation with no executable bit
  before or after. NOT_YET_ACTIVE counts only when every changed path is
  inside the bootstrap scope, and checks listed as interim need evidence
  files that name the exact tree.
- Tools: each pinned tool is checked, copied into the run folder, checked
  again, bound read-only from the copy and checked once more after the
  checks; the export is also checked after the checks.
- Sandbox: each check runs in its own systemd transient service under a
  throwaway dynamic user, with no network and no internet sockets, no
  devices or capabilities, a system-call filter, private `/tmp` and
  `/var/tmp` in memory, hidden host folders (`/run`, `/var`, `/srv`,
  `/mnt`, `/media`), a file-size limit and memory, process and time limits.
  The candidate sits alone in a scan folder, so the secret scan never reads
  settings or ignore files the candidate supplies. A small shell step
  reports the check's own status as 10 or 11, so a failure of `systemd-run`
  itself is BLOCKED, never FAILED.
- Unit tests: run by the pinned harness (`tools/runner/harness.py`), which
  takes a fresh run code on standard input and writes its summary to a
  private copy of standard output. The check passes only when every
  collected test ran with nothing failed, skipped or left uncollected.
- Outcomes and verification: each result must have the kind its profile's
  plan names; anything missing, unknown, duplicated or unexplained blocks.
  Bundles are canonical JSON signed with OpenSSH (`ssh-keygen -Y`,
  namespace `sutradhar-evidence`). `verify` checks the signature, form,
  expected tree, base, profile, plan, policy and runner pins and the
  policy's maximum age, and recomputes the profile choice, the bootstrap
  scope and the status.
- Probe: `python3 -m tools.runner.probe` runs inside the sandbox and passes
  only when every forbidden attempt meets a genuine operating-system
  refusal (AT-TB01).

## Independent review

Three passes by an independent reviewer (code, security, performance).

- Pass 1, 16 findings: five high (bootstrap scope, secret-scan settings
  from the candidate, an unrecorded base, unit tests judged by exit status
  alone, a probe network check that could pass with networking open; the
  last was based on an earlier copy). All fixed.
- Pass 2: most closed; open were the base in verify, the candidate's
  `.gitleaksignore`, harness forgery, evidence from another tree, folder
  swaps above the output folder, documentation patterns, `/tmp` on disk and
  smaller points. All fixed, measured on Dell where behaviour depended on
  the pinned tools (gitleaks honours a candidate's ignore file unless the
  candidate sits one level below the scanned folder).
- Pass 3, 7 findings, all fixed: gitleaks' own built-in path allowlist
  skipped secrets in folders such as `venv/lib`, `*.dist-info` and
  `node_modules` (confirmed on Dell: the old setup missed a planted token
  in four such places), interim evidence that was not a structured passing
  record, a hang on paths starting with `//`, an untested part of the
  repository policy, documentation judged by folder rather than file type,
  unreported `*_test.py` files and tool copies following links. The scan
  now uses a settings file derived from gitleaks 8.30.1's built-in rules
  with only the global path list (25 entries) removed, pinned as a tool.

## Accepted limits

- Code under test runs in the harness's process and could forge a summary
  deliberately; that is a review matter, not something a harness prevents.
- The runner is started as root with a qualification key until the owner
  creates the runner account and its narrow sandbox rule
  ([owner actions](owner-actions-2026-10-30.md), section 6).
- Candidate file names are passed on the command line; long or mangled
  names fail closed.
- The manifest digest is recorded but `verify` relies on the Git tree id
  for identity; the publisher re-exports when it needs the manifest.

## Evidence (Dell, 30 October 2026)

Code tree: `tools` subtree `e7ea753b`. Private logs, evidence records and
the qualification run are in locked Dell directories named in the private
operator note.

- Test-first: each change's tests failed on behaviour before the code
  existed or changed: first 77 of 81 against stubs (4 happy-path tests
  passed by coincidence), then the probe (11 of 11), the sandbox-start
  fix, the network probe, and the fixes for review passes 1, 2 and 3 (97
  of 168, 64 of 196 and 7 of 200 failing). Tests written after code to
  close coverage or mutation gaps are labelled as retrofits.
- Tests: 367 across `tools` (204 for the runner), all passing; 100% line
  and branch coverage of the runner (909 statements, 320 branches).
- Mutation (mutmut 3.8.1): 2,473 mutants; 2,471 killed, 1 timed out and 1
  equivalent survivor removed by rewriting the line; a rerun of that
  module killed all 132 of its mutants.
- Source limits: the whole repository passed (246 files).
- Real sandbox (systemd 259, runner started as root, qualification key):
  - commit profile PASSED: source limits, all 367 tests through the
    harness and the secret scan, with interim records for the checks not
    yet active; `verify` PASSED and a tampered bundle was refused;
  - refusal probe: all 16 attempts refused by the operating system
    (reading the signing key, `/etc/shadow`, `/root` and Dell storage,
    writing to the candidate, tools, storage, `/etc`, `/home` and `/root`,
    connecting to the D-Bus and journal sockets, opening IPv4 or IPv6
    sockets, `sudo`); hidden paths were first confirmed to exist on the
    host;
  - `/tmp` and `/var/tmp` are in memory inside a check;
  - hostile secret scan: with a planted token, a candidate settings file
    that allows every path and an ignore entry for the token, gitleaks run
    directly missed it, while the runner found it and the check FAILED;
  - the documentation-only profile was refused for a code change.

## Performance gate (SPEC-007), result B

Unit: `candidate.export`. Reference, fixed before measurement: the
simplest correct way to write a tree's exact blobs, listing the tree and
running one `git cat-file blob` per file. Dell, one pinned core, 60
interleaved pairs on the repository's own 246 files, identical outputs
checked on every pair:

- reference 183.0 ms, export 22.0 ms: 8.36 times faster (95% interval
  8.28–8.43); spread 5.2% and 5.5%, inside the 10% rule;
- floor: Git alone takes 12.1 ms to read the files and 2.0 ms to list them
  with sizes, so even free hashing and writing would allow about 13 times;
  20 times is not reachable without replacing Git;
- peak memory 3.4 MB against 0.45 MB for the reference (the whole tree is
  held once); far inside the budgets;
- for context only, `git archive` takes 13.1 ms but does not give exact
  blobs (attributes can change content).

The export became a few milliseconds slower after the review fixes added
the size listing and Git time limits; that is their cost.
