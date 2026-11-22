# 0003: Toolchain and check runner

The check runner takes one exact version of the code, runs every check
that applies to it in a locked-down box, and signs the result. It is
the one trustworthy way to say whether a change may be committed or
pushed, and why. It is part of aegis, the gate, and is roadmap Phase 2
items 1 and 3. The tiers it serves are in
[AGENTS.md section 7](../../AGENTS.md#7-the-gate).

Language. aegis is a developer tool, written in Rust (ADR-0019).

## What it does

### Entry points and profiles

1. The current protected entry points stay in use until the
   recorded gate cutover. Bazel then owns build, test, lint, type,
   evaluation and developer targets under ADR-0021. A language
   declaration does not admit a tool or change a check. The trusted
   gate chooses the full tier profile.
2. The runner reports the profile it chose and every check that the
   profile requires, with a plain reason for any check that does not
   apply.
3. The documentation-only profile is chosen only when something changed
   and every changed path is documentation with no executable bit,
   before or after. It still runs source capture, links, consistency,
   privacy, secrets, rights and review checks. It also runs WordPress
   Plugin Check when required under the WordPress Plugin Check section.
   Any executable source or workflow change ends the documentation-only
   profile, and the runner picks the code profile.
4. Quoted historical source requests are data. The runner never runs
   their contents.

### Cutover and dependency evidence

1. EX-03 and QR-099 must prove the pinned Bazel graph and Windows
   Rust support with the intended toolchains. Inventory every check
   and target, including Rust, Angular, Python, formal checks,
   package builds and PHP adapter tests. Record its old and new
   entry point, evidence and independent review.
2. Prove the affected target closure on changed shared inputs.
   Include generated artefacts, build tools, policy, migrations and
   dependencies across languages. A missing mapping or private input
   blocks. Run the current protected gate until the authorised
   cutover. Run candidate verification without issuing release
   authority from it. Each planted fault must still be refused.
3. Bind the exact tested candidate, graph, lock digests, trusted
   runner, tools, platform and sealed profiles in the cutover record.
   Verify dependency resolution, licences, vulnerabilities and
   provenance on the full resolved set. The SBOM enumerates the
   same set with hashes, build or runtime scope and target platform.
   Verify it again against the packaged payload and prebuilt bytes.
4. The current trusted Dell gate verifies the exact cutover
   candidate, current protected policy and required reviews.
   Its signed cutover record binds both policy generations, runner,
   entry points, graph, checks, pins, locks and SBOM digests. Any
   reduced or removed required check must have the owner's signed
   waiver under AGENTS.md section 7.2. Switch on that exact record
   as one authority change. Verify that the active runner, policy and
   entry points match its installed generation before accepting new
   route evidence. Keep the last accepted generation recoverable.
   A partial switch can leave the earlier verified route authoritative
   only while its policy remains current under 0065. If the trusted
   policy floor has advanced and activation is incomplete, block
   until the route and policy are verified together. Remove legacy
   inputs only after the accepted route covers them.

### Packaged dependency evidence

Reconcile the 0113 package's component slice with the exact resolved
build, target, package profile and verified packaged bytes. Match the
component hashes and scope to the protected dependency evidence, and
resolve each required 0114 scanner result to that exact component
revision and admitted scan profile. Keep the package slice distinct
from the complete resolved dependency set. The trusted runner binds
the observed package and scanner evidence through the existing signed
result references. On release, 0005 checks those references against
its exact candidate and unsigned payload before approval.

### Evidence and timing

1. Checks run before a commit exists. The evidence binds the tested
   file and content list, the settings, the dependencies and the
   environment. The commit must match that list; any difference voids
   the result, whatever the file names.
2. The push tier runs again at Forgejo's door and checks that the
   evidence still matches.
3. Release judging and the 72-hour soak belong to the release tier
   only. Ordinary checks never pretend to have run them.

### Results

1. A structured result holds: the candidate and content identity, the
   profile and policy version, the environment, the dependency lock,
   each check's name, outcome and reason, timing, review provenance and
   evidence references.
2. Outcomes are PASSED, FAILED, BLOCKED or NOT_APPLICABLE. An absent or
   skipped result never becomes PASSED. A worker crash or partial
   report never counts as success.
3. Exit codes: 0 PASSED, 1 FAILED, 2 BLOCKED.
4. Bundles are canonical JSON, signed through tijori (spec 0065) for the
   `evidence-bundle` purpose. The signed bytes follow spec 0064's
   layout: the purpose, record type and format version fields first,
   then the canonical JSON as one length-prefixed field. This spec owns
   that record type. `verify` rechecks the signature, tree, base,
   profile, plan, pins and maximum age, and works out the status again.
5. Private raw output stays on Dell. Only scrubbed summaries leave it.

For Phase 2 acceptance under 0010, compare the observed results to
its protected required-check inventory. A passing `gate.catalogue`
structure check gives evidence for that check. An `aegis catalogue`
coverage report may exit zero while listing missing or partial items.
Neither gives an observed result for another required check.
Keep `partly` distinct from `items`. Changing the declaration or
referencing a named fixture provides no new language or tier proof.

### Who may decide

1. One set of check definitions is used on Dell and in GitHub CI. No
   recipe holds its own copy of the policy. AGENTS.md holds the rules;
   this part holds the runner's behaviour.
2. Candidate code and ordinary workers cannot mint a trusted pass or an
   owner approval.
3. The policy is read from a protected copy, never from the candidate,
   and is loaded strictly.
4. The trusted runner is a pinned Rust build whose hash is kept in the
   protected policy. A candidate that changes the runner still cannot
   judge itself.
5. A change to the runner, the gate policy or the workflow files is a
   protected change under
   [AGENTS.md section 7.2](../../AGENTS.md#72-waivers), judged by the
   runner already accepted.

### The locked-down box

1. The candidate is exported blob by blob. Links, submodules, `.git`
   entries, unsafe or non-UTF-8 names, corrupt objects and oversize
   exports are refused.
2. The parent tree goes only to checks that name it, read-only, and is
   checked again afterwards.
3. Each pinned tool is checked, copied, checked, bound read-only and
   checked once more after the checks.
4. Each Dell check runs sealed with no network, devices or extra
   capabilities, isolated scratch and bounded resources. For Phase 2,
   prove the required hosted Windows refusal fence and authenticated
   evidence for the current protected gate profile. Verify that evidence
   on Dell against the exact candidate, workflow, checks and policy.
   An unproved Windows check blocks its required result. The replacement
   Windows profile and full Bazel switch have their own EX-03 and QR-099
   proof under Cutover and dependency evidence and spec 0139. A result for
   either route must match the protected profile of the check it serves.
5. The secret scan never reads settings or ignore files that the
   candidate supplies.
6. A failure of the box itself is BLOCKED, never FAILED.
7. The refusal probe passes only when every forbidden attempt meets a
   real operating-system refusal.

### Tools

1. Tests use random order, bounded parallelism and timeouts.
   Tool targets preserve mutation, benchmarks, snapshots, unused
   dependency checks and the owner's lint set. Select pinned tools
   through the protected check catalogue. Commit message, changelog
   and secret checks keep the same admission and coverage.
2. Contract shapes are Rust types with serde and garde. Generated
   shapes are checked against their source and still meet every quality
   and size limit.
3. Module walls are checked by nirikshak's module checks: product code
   never imports lab code, core never imports feature modules, and a
   feature module uses core contracts, never another module's insides.
4. The management API is fuzzed against its published contract.
5. Every tool and its dependencies are pinned by version and hash in
   `gate/tools.toml`, with a licence check. PHP 8.5 support and the
   coverage asked for must be proven for the PHP adapters.

### WordPress Plugin Check

`php.wordpress-plugin-check` runs the official WordPress Plugin
Check (PCP) and blocks commit, push and release under AGENTS.md
section 7 and DR-157. The protected gate chooses applicability and
the expected input. No caller, candidate or workflow can opt out.

| Tier | Exact WordPress input | Refusal or effect |
| --- | --- | --- |
| Commit | Whole installed plugin projection from the exact staged candidate | Before commit. Includes a documentation-only candidate once the product plugin exists. |
| Push | Whole projection from the exact clean candidate at the Forgejo door | Run independently before accepting the ref. A commit result gives no push pass. |
| Release | Fresh verified extraction of the exact unsigned WordPress ZIP bound into 0005's release payload | Before release approval or signing. Verify the unchanged ZIP length and digest at publication. Push results give no release pass. |

On commit and push, derive the projection from the exact protected
build graph and package manifest, including generated code, assets,
pinned dependencies, plugin headers, versions and readme. Use the
same content selection as the release package. Under 0031 the first
WordPress source candidate must have this complete projection and
admitted check. Launch no candidate-controlled tool to decide scope.
The expected plugin remains required if a candidate deletes, renames,
omits or empties the source, projection or ZIP without an approved
change of supported scope. Compare the base and candidate as well
as the current protected applicability record. A candidate before
the first WordPress source and with no product requirement for the
selected tier may report NOT_APPLICABLE with that protected reason.
Phase 2 proves the check on synthetic clean and planted candidates
before the product exists, without a product host or package. A
fixture pass gives no product result.

#### Pinned execution and isolation

Use the official stable PCP distribution and the pinned WP-CLI
route. Record the version and digest for PCP including its required
libraries, WP-CLI, PHP 8.5, the test database and WordPress core. Pin
their input bytes, licences and build or fixture scopes under
`gate/tools.toml`, the dependency evidence, and the SBOM. Use the
two supported WordPress majors and exact host/runtime profiles
owned by 0031. QR-247 qualifies the exact versions, backend, test
fixture, check inventory and output parser before use. Requalify
a changed tool, check set, parser, or runtime profile. Do not use
an unpinned download, automatic update or ad hoc CI recipe.

On Dell, run in the existing sealed worker with read-only pinned
tools and candidate inputs. For each WordPress profile start a
fresh disposable synthetic WordPress instance with its own database
and scratch space, no production credentials, plugins or data, no
external network, no host mounts from the candidate and no release
key. Only the sealed test database and required local fixture
endpoints are reachable. Fixture writes stay inside that instance.
Validate the installed content manifest against the projection or
verified extraction before use and after the checks. Bind the real
plugin root to that manifest and keep the installed target read-only
throughout the scan and runtime loads. An unproved effective path
or writable overlay over any target member blocks. Reuse 0005's
archive bounds and path verification before extracting the release
ZIP. On refusal or success, stop the fixture and remove its owned
scratch state before accepting another run. Cleanup failure holds
the check. PCP, WP-CLI and the fixture database are development
inputs and add zero files, queries, bytes or page time to shipped
sites. Keep 0031's native upload, compatibility and footprint
evidence, PHPStan and dependency checks separate.

The protected launcher owns the WP-CLI working directory and every
ancestor it searches for configuration. Supply the approved
configuration and a fixed environment without inherited host or
candidate values. Select the fixture root with `--path`. Confirm
the effective configuration and arguments before WordPress loads.
Unexpected configuration, an alias, or a different fixture target
blocks. Bind the approved launch inputs to the check evidence.

Run the official `wp plugin check` against the exact installed target
using `--require` to load the pinned PCP `cli.php` before WordPress.
Use the pinned `--mode=new` review profile on all tiers. Request
`--format=strict-json` and
`--fields=file,line,column,type,code,message,docs,severity`.
Require these fields for every finding. Leave severity thresholds
unset. In this profile, the official CLI emits `ERROR` or
`WARNING` at all numeric severities. Require all applicable stable
static and runtime checks in
that profile. The protected inventory binds the exact check IDs,
types and admitted applicability per WordPress profile. The runner
must prove each expected check completed. The CLI listing alone
proves selection only. During runtime, the candidate runs in the
WordPress process. Require the qualified protected supervisor to
verify each check's completion and emitted findings independently
of any output or hooks that candidate PHP can influence. Match the
CLI's normalised report to those findings under the pinned
export mapping. Check each row and preserve duplicate counts.
Unproved or mismatched results are
BLOCKED. Experimental and AI checks are outside this
profile. Do not read candidate filters, select a smaller check set,
enable `.pcpignore` or pass options that suppress files, diagnostics
or severities. Use `--include-low-severity-errors` and
`--include-low-severity-warnings` with the verified parser. Verify
the built-in file scope. The separate dependency and source checks
own their coverage. If the official profile cannot complete offline
or the runner cannot prove coverage, the check is BLOCKED.

Use the AGENTS.md Dell queue and its per-job resource cap. Bound
each profile's fixture and scan to 300 elapsed seconds and the
complete attempt from worker start through teardown to 900 seconds.
Capture at most 1 MiB each of stdout and stderr and 10,000
normalised diagnostics per profile. Equality is allowed for the
byte and row limits. At the time deadline stop the work and hold.
Excess output, crash or a missing or failed cleanup also holds.
No automatic retry changes the scope or deadline. A new protected
attempt uses the same verified input if still current and starts
fresh fixtures on all required profiles. Trace completion, findings
and teardown to that attempt using the protected runner's own
evidence. An earlier attempt's observations cannot fill a current
cell. If the runner cannot prove that attribution, block. Record
a slow gate under the existing timing rules.

#### Outcomes and evidence

The trusted adapter accepts one complete non-empty array of
findings or the exact clean success line for the pinned CLI and
fixed test locale. The line yields an empty diagnostic list only
when the protected observer confirms every check completed and
emitted no findings. Require a zero exit for either form. Empty
stdout, an empty array, extra stdout, unapproved stderr or an
unrecognised success line is invalid. Use the
existing structured 0003 result. Evaluate missing or invalid proof
first. Evaluate findings once evidence is complete.

The admitted types are `ERROR`, `WARNING`, `ERROR_LOW_SEVERITY`
and `WARNING_LOW_SEVERITY`. Keep the exact type and source fields
in private evidence and the original multiplicity in the match.
All four types fail the check when the run evidence is valid.
Missing fields or unknown types block the check. Retain only
the scrubbed diagnostic fields described below in the summary.

| Outcome | Exact condition | Summary text |
| --- | --- | --- |
| PASSED | Every required fixture and check completed with no error or warning and all input and execution evidence current | “WordPress Plugin Check passed.” |
| FAILED | Complete valid evidence reports at least one error or warning, including low severity | “WordPress Plugin Check found issues. Fix them and run the gate again.” |
| BLOCKED | Required input, tool, fixture, check coverage, timeout, report, trust or cleanup proof is missing or invalid | “WordPress Plugin Check could not complete.” Show the safe reason. |
| NOT_APPLICABLE | Protected pre-introduction applicability proof as described above | “The candidate has no WordPress plugin to check.” |

Bind the signed bundle to candidate, tier, current policy, check
inventory, protected runner, fixture and all pinned input digests,
plugin projection and, at release, exact ZIP digest and length.
For release, bind the exact protected request and full run, H(C),
H(I), packaging profile and standalone WordPress part under 0005.
Bind each profile's static and runtime completion, the observer's
per-check finding counts and digests, the normalised outcome and
the matching CLI diagnostics digest. Keep raw output on Dell. Show only
scrubbed relative plugin paths, line, upstream check code, severity
and safe message. Treat output as data. The mirror uses the same
definition under the existing CI scope and does not decide the
Forgejo gate. The 0005 release check consumes the fresh matching
release result. A missing or substituted cell blocks the tier.
Owner waivers follow AGENTS.md section 7.2.

See the [official package][pcp-package], [CLI contract][pcp-cli]
[runtime design][pcp-runtime] and [WP-CLI configuration][wp-cli-config].
The [tagged CLI source][pcp-output]
shows both output forms and low-severity types. Passing PCP supplies
this check's evidence only. WordPress.org listing approval follows
its own process.

[pcp-package]: https://wordpress.org/plugins/plugin-check/
[pcp-cli]: https://github.com/WordPress/plugin-check/blob/trunk/docs/CLI.md
[pcp-runtime]:
  https://github.com/WordPress/plugin-check/blob/trunk/docs/technical-overview.md
[pcp-output]:
  https://github.com/WordPress/plugin-check/blob/2.2.0/includes/CLI/Plugin_Check_Command.php
[wp-cli-config]: https://make.wordpress.org/cli/handbook/references/config/

### Where work runs

1. Dell owns the trusted gate, build queue, private checks,
   licensed forum code and final speed and reference measurements.
   Keep its sealing and shared-host limits under AGENTS.md.
2. ADR-0021 admits hosted Windows builds and synthetic checks
   through GitHub Actions. It gives the hosted runner no Dell gate
   authority. Use only approved public input from the exact
   Forgejo candidate. Pull request triggers, secrets, release
   keys, private fixtures and unverified inputs have no route.
3. Bind each result to clean candidate bytes, pinned workflow,
   actions, tool and input digests, hosted runner and image
   identity, event, permission profile and run identity. A trusted
   collector outside candidate control attests the result. Dell
   verifies its authority, freshness and exact check coverage.
   A different tree, untrusted rerun or missing identity blocks.
4. The owner's editing computer runs no development jobs. Required
   evidence from an unavailable host remains blocked.

## Verification configuration

Added 20 November 2026 (QR-046). The verification rules of Behemoth Boot (spec
0114) and Leviathan (spec 0116) use these files. They are part of the gate. Each
is under `gate/`, is changed only by a reviewed change, and none may be set
below "block" without an owner waiver (AGENTS.md section 7.2).

| File | What it holds | Owning spec |
|---|---|---|
| `gate/verify.toml` | Fuzz, benchmark, soak, chaos, lab and reproducible-build settings, and the compatibility matrix's server versions | 0114, 0116 |
| `gate/bench-baselines.toml` | Stored benchmark results, protected like `gate/speed.toml`; changed only by a reviewed change that gives the reason | 0114 |
| `gate/unsafe-core.toml` | The list of core crates that forbid unsafe code | 0114 |
| `gate/waivers/` | Owner waivers: each names its check and expires within 30 days | AGENTS.md 7.2 |
| `gate/tools.toml` | Every pinned tool, with its hash and licence (existing) | this spec |
| `gate/known-failures.toml` | Standards tests that fail today, each with a reason; a new failure fails the gate | 0116 |
| `gate/allowed-differences.toml` | Checks allowed to differ between Linux and Windows, each with a reason | 0116 |

1. The tools these rules need are pinned in `gate/tools.toml` like every other
   tool: the Quint checker with its Node and Java runtimes, the Kani, Verus and
   Creusot provers, the fuzz engine (once chosen, QR-042), the unsafe-usage
   scanner, the load tool for benchmarks, and the prebuilt servers of the
   compatibility matrix. Each has a hash, a licence check and a supply-chain
   scan.
2. A check named by these files whose tool is missing gives FAILED or BLOCKED,
   never a smaller passing profile (see Limits).
3. The long lane (full fuzz budgets, the soak, the full matrix and the
   benchmark wall) is part of the release tier. By the owner's decision of 20
   November 2026 (DR-069) it is exempt from the push-tier time targets below,
   and an agent may approve a release only after it passes. Commit and push
   still run every check and are never cut short.
4. A tier that exceeds its time target raises a speed task and is never
   shortened by skipping a check.

### Documentation profile evidence

`REQ-0003-DOC-PROFILE`, `DES-0003-DOC-MODES` and
`VAR-0003-DOC-MODE-CHANGE` link the existing entry-point rule to
`TASK-0003-PROJECT-GUARDS` and `IT-0003-PROJECT-GUARDS-REFUSAL`.
See the [task catalogue](../planning/implementation-catalogue.md)
and the
[cases](../planning/test-and-traceability.md#remaining-phase-2-trace-designs).

Check the changed paths and their old and new file modes in the
protected profile decision. Bind those modes through the captured
base and candidate tree identities. A mode change can invalidate
documentation-only evidence even when the file bytes and path match.
A changed path with the executable bit set on either side needs the
code profile. On a valid documentation-only candidate, check the
complete required documentation results under the chosen tier.

## Limits

- A missing tool, an unknown check, a bad report, expired evidence,
  changed content, a missing private input or a failed privacy scan
  gives FAILED or BLOCKED, never a smaller passing profile. The report
  names the failing check in plain English, its code and the next step.
- A timeout stops the work safely and keeps enough private evidence to
  find the cause.
- Parallel test workers share the declared resource budget: at most two
  heavy jobs at once on Dell, each held to 4 GB and 8 CPU threads.
- Tests may not change tracked source in the candidate under review.
- Targets, measured on the real workload and never claimed from an
  empty repository: fast push checks under 5 minutes; the private
  development evaluation under 10 minutes.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Rust integration tests in the aegis crates, run on scratch repositories:

- A documentation-only candidate may report an inapplicable code
  check as NOT_APPLICABLE with proof. Secrets, privacy, consistency
  and reviews still run. WordPress Plugin Check runs when required
  regardless of the selected documentation profile.
- In IT-0003-PROJECT-GUARDS-REFUSAL, add an executable file or
  change only a Markdown path's executable mode. A changed path
  executable on either side needs the code profile. Documentation-only
  evidence from before the mode change cannot verify the new candidate.
  Separately, use a documentation-only push bundle signed with test
  keys as the matching control for the exact protected push request.
  Keep that request's base and candidate commit and tree identities
  fixed. Substitute a correctly signed commit-tier bundle from its
  own test plan for the same base and candidate tree and same policy.
  Take the expected push tier and profile from the protected plan.
  Refuse the substituted bundle even if its checks pass. Keep the
  signature and tier checks distinct. Use only test keys. The door
  still runs its own checks under 0001.
  In a separate scratch history, base A marks an inert Markdown
  path executable. Its child B clears the bit. B's child C edits
  the Markdown and leaves it non-executable. Use the existing
  runner role and a test key to sign the documentation-only push
  bundle for B to C. Keep C's exact commit and tree, push tier and
  policy fixed. Offer that bundle against the protected push
  request from A to C. A's mode makes that request need the code
  profile. The test signature verifies. The candidate verifier
  refuses the bundle for its mismatched base and profile. The
  door still runs its own checks under 0001.
- Source changed after the checks makes admission refuse the stale
  evidence, even with the same file names.
- A forged, absent, truncated or skipped check result never yields a
  passing status.
- A product-to-lab import, a core-to-module import or a module-to-module
  import of insides fails the module check.
- Bad API data, or a shape that no longer matches its source, fails the
  contract check without sending private data anywhere.
- A secret or an identifying forum detail in a source file, artefact or
  log fails the public-output check and the output is held back.
- With parallel workers, random order, timeouts and mutation failures,
  the result stays reproducible and names the exact failing evidence.
- A development subset passing while a required check fails leaves the
  overall result failed.
- The runner's own failure paths are covered, at the coverage AGENTS.md
  section 6 sets.
- A deliberately broken candidate is refused by every check it should
  fail.
- Refusal probes pass on the admitted Dell and Windows profiles.
- Try a missing reverse dependency, omitted package binary, stale
  lock and unauthorised Windows result at cutover. Each is refused.
- Drop a check, change its scope or build from a legacy input
  after cutover. Each fails. Interrupt each switch boundary.
  Check that no mixed policy can issue evidence. Interrupt once before
  the trusted policy floor advances and once after it advances while
  activation is incomplete. Use the earlier verified route only in
  the former case if it stays current. The latter case must hold until
  recovery reconciles the current policy and route. Use 0139
  IT-0139-BOOT-COVERAGE to check both interrupted states at 3B exit.

### WordPress Plugin Check acceptance

`REQ-0003-WP-PLUGIN-CHECK` and `DES-0003-WP-PLUGIN-CHECK` bind the
three tier inputs to `TASK-0003-CHECK-MAP`, the pinned inputs task and
`IT-0003-WP-PLUGIN-CHECK`. Use synthetic sites, test keys and the
approved fixture profiles. Exercise both supported WordPress majors.

| Case | Input and expected result |
| --- | --- |
| Clean and planted | The clean fixture has a qualified static and runtime inventory. Pass both cells. Plant one stable static error and separately one safe runtime-only finding in a synthetic plugin. Refuse each at commit, push and release. Prove the runtime finding is absent in a static-only execution, then hold that execution for missing runtime evidence. |
| Warning and output | In separate synthetic parser cases, a complete report with any one of the four admitted types fails, even with a zero exit. A sealed official run with one low numeric severity warning and thresholds unset fails with its `WARNING` finding. A valid clean success line and independently observed empty result pass. Empty output, an empty array, a malformed report, missing fields, an unknown type, a missing check or a duplicate completion receipt block. |
| Independent findings | Complete every required check. Retain one independently observed low-severity warning. Offer the clean success form, then a valid report with the row absent or its type altered. Block each attempt. Return the full matching report and expect a failed check with the warning. Keep two otherwise identical findings from separate checks and prove that a report with only one copy blocks. |
| Launch inputs | Put a candidate-owned WP-CLI configuration inside the read-only plugin. It stays inert and the full inventory runs. Separately offer an unapproved config in the launcher search path, or a different effective fixture root. Refuse before WordPress loads. |
| Always run | Run commit and push on a source-only change whose dependency closure excludes the plugin. Require the full check. Change documentation-only staged candidate A to staged candidate B with identical plugin bytes. An otherwise valid result for A cannot satisfy B. |
| Profile and bounds | Omit the second WordPress major, change a tool hash, withhold a database pin, add an attempted exclusion, stop at the time limit, emit one byte or row beyond its cap, or lose teardown proof. Each holds. Exercise cap equality with the bounded parser and no authority from a partial report. |
| Candidate and tiers | An eligible pre-plugin documentation fixture may report not applicable. A documentation change with the plugin present must run the check. Attempt deletion or a rename without approved scope change, substitute different staged or generated bytes, and offer a matching commit pass at push. Refuse each substitute. |
| Release and isolation | Keep the release manifest fixed and replace the ZIP, or offer an earlier run, a valid result from another authorised run or a signed different unsigned payload inventory with the same WordPress ZIP. Hold before approval, signing, final tagging or publication. Preserve the candidate tag and the failed-candidate record under 0005. An exact fresh result can continue to the other release checks. A planted runtime attempt to reach an outside endpoint or write beyond scratch is refused by the sandbox. Separately produce a clean-looking exit and output after omitting an official check through candidate hooks or terminating the runtime early. The protected supervisor refuses the result because completion is unproved. |

In the isolation case, retain both manifest measurements and make
the effective installed target unproved at load time. Refuse before
accepting the result despite the matching before and after values.

In a separate release retry case, finish and tear down the first
WordPress profile. Interrupt the checker during the second profile
before its teardown is proved. The attempt is BLOCKED. Keep the
same unsigned inventory, immutable ZIP, policy and pins. Once the
existing recovery route admits a fresh attempt, offer the earlier
first-profile records alongside a current passing second profile
with fresh teardown.
Block the mixed evidence before release approval or signing. Keep
the earlier records eligible only as observations of their own
attempt. The separate fresh control uses that same verified ZIP
and finishes and tears down both profiles in one new attempt.
This passes only Plugin Check and supplies no native handoff proof.

The Rust adapter's unit cases cover output bounds, categories,
severity and failure classification. The sealed fixture and tier
refusals cover integration, end-to-end and security. Verify deadline
and queue bounds under the existing gate resource checks. Use 0003's
protected result and tier replay cases for evidence integrity. There
is no new product protocol state to model formally.

## Changes from the source

- Rust provides the trusted gate. The accepted build uses the
  Bazel target graph and exact tool inputs under ADR-0021.
- Generated data models are replaced by Rust types with serde and garde.
- The notebook lab is replaced by Rust lab runs with recorded inputs
  and outputs.
- The import-contract tool is replaced by nirikshak's module checks, and
  the API fuzz tool by the management API fuzz harness.
- Dell uses sealed jobs. Windows evidence follows the approved
  hosted runner and verification route.
- The trusted runner is a pinned Rust build, not source copied from the
  parent commit.
- GitHub statuses, proposal bots and pull-request reviews are gone:
  Forgejo's door runs the push tier, and the required reviews of AGENTS.md
  section 7.1 block like every other check.
- Release and dependency proposal bots are dropped; their work happens
  on Dell, as 0005 describes.
- The heavy-job limit follows AGENTS.md section 11.

Source: `docs/specifications/bootstrap-toolchain.md` and
`docs/specifications/bootstrap-toolchain-rust.md` in the archive
`archive-2026-11-08-old-repo`.
