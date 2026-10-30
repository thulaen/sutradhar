# First documentation checkpoint

Audience: agents preparing the repository. Outcome: create the first reviewable
documentation change without claiming that unwritten software passed tests.

Status: approved documentation-only approach from Q190. This is an applicability
rule, not a waiver of checks or evidence that a particular candidate has passed.

## Required order

1. Capture the interview, source request lists and all later replacements in
   privacy-scrubbed requirements. Keep private originals outside the public repo.
2. Write the agent rules, development contracts, feature specification drafts,
   manual specification and complete-scope acceptance ledger.
3. Prepare and verify the authorised Dell environment and dedicated storage.
4. Run the applicable documentation, privacy, rights and consistency checks there.
5. Obtain code, performance and security review of the documentation candidate,
   interpreting each review in terms of the documents' claims and proposed risks.
6. Create a documentation-only commit whose contents match the reviewed candidate.
7. Push its branch under the dedicated App identity and open a pull request.
8. Pass current-change GitHub checks and the owner's protected-rule approval before
   that foundational governance change can merge.

The method research catalogue is the first research milestone. Infrastructure and
checking tools needed to qualify it are dependencies, not permission to skip that
milestone or substitute a small linking product for the full requested release.

## Documentation-only profile

The candidate must contain no executable implementation, test program, build
script, automation workflow, executable notebook or executable configuration that
changes runtime behaviour. Licensed third-party implementation code does not
belong in this public checkpoint either. Ordinary prose, diagrams and clearly
labelled illustrative code fences are documentation, but embedding production
scripts in prose is not a loophole.

| Check | Required result |
| --- | --- |
| Candidate inventory | Exact paths and content identity; no executable implementation |
| Requirements | All interview decisions and long source lists accounted for |
| Supersession | Current decisions linked to the earlier choices they replace |
| Privacy | No forum identity, private metadata, member content or identifying artifacts |
| Secrets | No credentials, tokens, private keys or secret-bearing examples |
| Rights | Source provenance, short original summaries, notices and no unsupported clearance claim |
| Documentation | Valid structure, resolvable local links, consistent names and truthful status |
| Review | Code/process, performance-feasibility and security/privacy reviews recorded |
| Code-only tests | Explicitly not applicable because no executable implementation exists |
| Installed behaviour | Not implemented and not verified; never labelled passed |

Each non-applicable result includes the rule and inventory evidence. A checker
being absent is not a non-applicability reason. If any executable implementation
is added, use the full relevant code profile before committing it. Do not keep
using the bootstrap profile because the normal pipeline is inconvenient.

## Bootstrapping enforcement

The planned normal entry point is `just check`. Do not create a fake justfile whose
only effect is to print successful results. Before the command exists, the
documentation receipt must name the actual trusted tools and review actions used
on Dell. Their outputs and limitations remain explicit.

Repository protection may be configured to block merges before all checks exist.
Keep it blocked while missing components are implemented; do not add a temporary
bypass. A trusted bootstrap verifier may report the narrowly defined docs profile
only after checking the exact candidate itself. That verifier is executable work
and requires its own approved specification and qualification; it cannot certify
itself merely by existing in the candidate it is checking.

## Specification approval

Interview answers approve requirements. A newly drafted detailed feature
specification still needs the owner's approval before its implementation begins.
Prepare the concrete document, alternatives already resolved, acceptance cases,
risks and interfaces first. Then request approval of that revision, while making
progress on independent authorised documentation and setup work.

The mapped NASA/Meta rulebook, repository enforcement implementation, AI-review
verifier, release workflow and product features each need an explicit approved
specification. The owner can approve a named, reviewable group of specifications;
agents may not infer approval of unwritten details from a general request to work.

## Exit evidence

The checkpoint is complete only when the documentation inventory, privacy and
rights review, consistency review, scoped checks, commit, pull request and required
approval are inspectable. Keep later runtime, installation, UI, performance,
method-count and complete-release conditions open in the acceptance ledger.
