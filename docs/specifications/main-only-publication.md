# SPEC-004: main-only protected publication

Status: APPROVED, revision 2 after independent review. Protected workflow
revision awaiting the owner's approval. Replaces SPEC-003 "Repository
admission" (see the [approval delta](approval-delta-2026-10-30.md)).
Approved by the owner on 30 October 2026 (see the approval record in
`docs/specifications/approvals-2026-10-30.md`); it supersedes any
"awaiting approval" wording below.

Requirements: R083–R086, R094, R057, R068–R071 as amended. Accounts,
signing and owner approval: [trust boundaries](trust-boundaries.md).
GitHub settings and permissions: [settings part](main-only-github-settings.md).

## Outcome

`main` is the only branch. Exactly one protected publisher on Dell can
advance it, only by fast-forward, only to a candidate whose fresh, signed
checks, independent reviews, AI review and any owner approval are valid at
the moment of the update, with a complete audit trail and safe recovery.

## Identities

Three GitHub Apps with separate private keys, each held by its own Dell
service account:

- Publisher (the existing App, reduced): Contents write. Uploads objects
  and moves `main`.
- Attester (new): Commit statuses write and Checks write. Reports checks it
  has itself recomputed from signed evidence.
- Approval verifier (new): Commit statuses write and Issues write. Opens
  notification issues and reports owner approval after checking the
  owner's approval comment (the owner's chosen route; see below).

All three also have Metadata read. GitHub rules bind each required check
to the App allowed to report it. Release publication keeps its own future
identity (SPEC-003).

A Contents-write key can also create releases and dispatch events, so the
Publisher key never leaves its account, the Publisher software exposes only
the operations below, and every GitHub write it makes is logged.

## Publication request

The single Git coordinator commits locally after the runner has produced a
signed passing commit-profile bundle for the exact tree. It places a
request in the Publisher's inbox: a Git bundle of the new commits, the
expected remote `main` commit (the base) and the milestone record.
Requests are append-only; they name evidence but cannot contain or create
it.

## Publisher steps

1. Verify the Git bundle; check its commits form a straight line from the
   base; verify the runner's signed commit-profile bundle for every commit
   (AT-M01).
2. Read live `main`. If it is not the base, refuse as stale or unexpected
   (AT-M02, AT-M03).
3. Ask the runner to execute the push profile on the candidate tip in a
   sandbox and sign the result; ask the gate for a signed queue and quota
   verdict.
4. Collect the signed review records for the exact candidate (code,
   performance, security, self-review labelled) and the AI review.
5. Upload the commits through the Git Database API in order and confirm
   each returned identifier equals the local one (AT-M04).
6. The approval verifier classifies the change from the base policy. If
   protected, it opens a notification issue linking the uploaded commit
   and waits for the owner's approval comment.
7. Final check, repeated on every attempt including recovery: within ten
   minutes before the update, re-read `main`, re-run the gate verdict,
   re-check that the approval comment still exists unedited and re-read
   the active rules. Any
   refusal stops here and the Attester overwrites the statuses with
   failure (AT-M16).
8. The Attester recomputes every required check from the signed bundles
   and reports it; the approval verifier reports its result.
9. Move `main` with a non-forced reference update. GitHub refuses it unless
   it is a fast-forward and every required check from the expected App
   passed (AT-M05).
10. Read `main` and the tree back, write an immutable publication receipt,
    and append the gate's latest journal head to the publisher-owned
    anchor log, which the gate can append to but not rewrite (AT-M06).

The Publisher never creates branches, tags or releases, never forces an
update and exposes no general GitHub access (AT-M07).

## Owner approval comment

The owner chose approval by GitHub comment (approval record). The
verifier accepts a comment only if: its author's immutable user number is
the owner's, recorded in the base commit's protected policy; it is on the
notification issue for this candidate; its text is exactly the statement
the issue gives, naming the candidate tree, the commit identifiers and a
SHA-256 digest of the protected files' hashes; and it has never been
edited. Editing or deleting it withdraws the approval, which the final
check in step 7 detects (AT-M10, AT-M11, AT-M16). Agents never post,
edit or delete approval comments.

## Bootstrap publication

The first publication carries commits made before the runner existed:
the historical foundation and the bootstrap milestones. For those
commits only, the runner re-runs the documentation profile (or the
interim check its activation step names) on each commit before upload and
signs the result, and the publisher also verifies the immutable Dell
evidence recorded when each was committed. A required GitHub check is added
to the ruleset only when its gate activates; until then it is not
required and the Attester reports nothing under its name (AT-M17).

## AI review without pull requests

GitHub's hosted Codex and Copilot reviews only review pull requests. The
required AI review therefore runs on Dell under the reviewer account, in
the reviewer sandbox defined in the trust boundaries, containing only
public repository content: the base commit with the candidate's change
applied as uncommitted edits, so no branch is needed. OpenAI documents
Codex's review of uncommitted changes in its desktop app but not an
unattended command-line route; Claude Code's non-interactive mode is
documented. The owner chooses the provider; its unattended route is
qualified before use.

The result records the tree, model, prompt version, findings and verdict
and is signed by the runner. Blocking findings must be fixed or answered
with evidence. Silence, a timeout, a quota error, a stale result or a
review of another tree is BLOCKED (AT-M08, AT-M09). The reviewer is a
different session from the author.

## Recovery

A durable journal records each step. After a crash the Publisher reads
GitHub: if `main` already equals the candidate, it verifies and writes the
receipt; if `main` equals the base, it resumes at step 7, never later;
anything else stops for investigation. Retries never move `main` twice
(AT-M12). A broken `main` gets a revert commit through this same route
(AT-M13).

## If GitHub refuses unreferenced candidates

Whether checks reported on an uploaded but unreferenced commit satisfy the
rules is undocumented; qualification test Q1 settles it. If it fails, the
only option within R084 is a non-branch candidate reference in a private
namespace (for example `refs/sutradhar/candidate`), which the owner must
approve as compatible with main-only development. Dropping GitHub's own
check requirement is not offered: it would remove the independent
enforcement R084 requires.

## Acceptance cases

- AT-M01: commits not in a line from the base, or lacking a valid runner
  signature, are refused.
- AT-M02: a base older than live `main` is refused as stale.
- AT-M03: a remote change between checking and updating stops the update.
- AT-M04: an uploaded commit with a different identifier stops publication.
- AT-M05: a missing, failed or wrong-App check makes GitHub refuse the
  update (qualification evidence).
- AT-M06: the receipt matches GitHub's `main` and tree.
- AT-M07: requests for branches, tags, releases, forced updates or other
  endpoints are refused without exposing credentials.
- AT-M08: an AI review of another tree, a timeout or silence blocks.
- AT-M09: an unanswered blocking AI finding blocks.
- AT-M10: an approval comment from another account, edited, deleted or
  naming a different digest fails.
- AT-M11: approval of candidate X does not approve candidate Y.
- AT-M12: a crash after each journal step recovers without double updates
  and always repeats the final check.
- AT-M13: a revert is published through the same route and checks.
- AT-M14: the agent account cannot read keys, use `sudo`, use Docker or
  switch to a service account (with AT-TB07).
- AT-M15: the owner and agents cannot create branches or push to `main`
  directly on the configured repository (refusal evidence).
- AT-M16: a P0 filed, an approval invalidated or a rule changed after step
  3 stops the update at step 7.
- AT-M17: a bootstrap commit without its recorded Dell evidence, or whose
  runner re-check fails, is refused; a gate's check context is not
  required before that gate activates.
