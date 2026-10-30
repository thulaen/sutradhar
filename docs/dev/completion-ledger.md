# Completion ledger

Owner: the Git coordinator. Updated: 30 October 2026, Claude session.
Rule: R097. Evidence lives on Dell; this page is the readable summary.
The full product is not complete. Zero methods are qualified.

## Finished and verified

- Requirements interview (Q001–Q209), current decisions R001–R082 and the
  documentation foundation: commit `691559a`, tree `3ce87786`, 17 passing
  documentation-profile checks retained on Dell.
- Source-limit compliance of the foundation: local commit `ecbd232`, tree
  `857c0391`. The documentation profile passed on Dell, including data
  round trip, transcript preservation, word preservation, links, limits
  and a gitleaks scan with no findings. Not yet on GitHub.
- Source-limit checker (activation step 1, interim): test-first, 163
  tests, 100% line and branch coverage, 769 of 770 mutants killed (one
  reviewed equivalent), whole repository passing, two independent review
  passes, performance result B; see the
  [milestone record](limits-checker.md). Local commit `f74e466`, tree
  `b621a8e5`; unsigned until the runner exists.
- 30 October requirements record R083–R097, supersession table, the
  81-item AutoIssues source register, the approved designs, the approval
  record and the agent rules for trunk-based development: local commit
  `8114c3c`, tree `9578ac1f`, after the Dell documentation profile.
- Model, full-reading and platform decisions R098–R111 with ADR-0001 to
  ADR-0004 and the visual layer register, recorded from the owner's
  answers with three independent fidelity reviews: local commit
  `d960dd7`, after the Dell documentation profile.
- Check runner, sandbox and signing (activation step 2, interim):
  test-first, three independent review passes, real-sandbox
  qualification on Dell; see the [milestone record](runner.md). Signed
  with a qualification key and started as root until the owner creates the
  runner account.
- Owner approval, 30 October 2026, of the trust boundaries, SPEC-004 to
  SPEC-007, the mapped rulebook, the SPEC-002 and SPEC-003 revisions and
  live-data storage, with recorded choices (approval record).
- SPEC-001, SPEC-002 and SPEC-003 approved for their unchanged scopes.
- Authorised Dell project storage, re-checked this session.

## Implemented, awaiting checks

- Earlier publisher modules (policy, candidate reader, transport): 130
  focused test methods and full statement and branch coverage for three
  modules. Their branch-and-pull-request policy is retired; the reader and
  transport are inputs to the main-only publisher. The launcher and
  deliberate-fault testing remain untested.

## Awaiting design approval

- The linking feature specification implementing R098–R108 (models,
  reading every post in full, matching, anchors, retraining, model files)
  and the specifications for R109–R111 (community models, progress, the
  visual layer), with ADR-0001 to ADR-0004.
- Features F001–F023 specifications (not yet drafted, except research).
- The `just` command interface details beyond SPEC-001.
- The four clarifications listed in the approval record, and the
  checker's plain-text class list (the approved text names YAML).

## Blocked by a specific external dependency

- Any push to GitHub: needs the owner's GitHub changes from SPEC-004
  (rulesets, Apps, pull requests off), the test repository decision, the
  read-only editing credential, then publisher implementation and
  qualification. GitHub's current rules accept only pull requests, so no
  compliant push route exists yet.
- Dell accounts and live-data directories: owner action or explicit
  step-by-step instruction (they change host security settings).
- Required AI review: Claude Code sign-in on Dell under a reviewer account.
- Older registered App key retirement: owner action on GitHub.

## Not started

- The test-first gate; the performance harness; the publisher rebuild.
- AutoIssues milestones 1–8 and the QA stack.
- Method research completion toward 6,000 qualified methods, the PHP
  add-on, the Python service and every product feature.
- Manual site, installation, upgrade, recovery and release work.
