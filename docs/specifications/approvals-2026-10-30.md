# Approval record: 30 October 2026 designs

Owner decision recorded: 30 October 2026, through the Claude session's
question form. Status: the designs listed below are approved for their
bounded implementation, tests and qualification.

## Questions and answers

The owner was asked four questions and chose these answers, quoted:

1. Designs: "Approve all (Recommended)". The question named the trust
   boundaries, SPEC-004 to SPEC-007, the mapped rulebook, the SPEC-002 and
   SPEC-003 revisions and live-data storage, with these choices: Claude
   Code as the required AI reviewer, Commitizen instead of Release Please,
   a weekly Dell dependency task instead of Dependabot pull requests, and
   a private non-branch candidate reference only if GitHub test Q1 fails.
2. Source limits: "Accept all six (Recommended)": one-link lines up to 200
   characters; tabs in quotes become spaces; catalogue and CSV data become
   YAML files of at most 300 lines; tool-written lockfiles exempt; the
   SPEC-001 to SPEC-003 reflow recorded as administrative; verbatim
   interview quotations keep exact lines in code blocks and transcripts
   are split by question range.
3. Administrator access and approval: "Keep sudo + comment". Passwordless
   administrator access on the shared Dell operator account stays. Owner
   approval of protected changes is a GitHub comment from the owner's
   account, with the editing computer's GitHub access reduced to
   read-only.
4. Quota amount: "10 (Recommended)".

## Consequences of answer 3

This was the weakest offered option, chosen knowingly. The separations in
the trust boundaries then protect against mistakes and ordinary faults,
not against a deliberately hostile agent using administrator access or
the owner's signed-in browser. Agents must never post, edit or delete an
approval comment, and must never use a browser session signed into the
owner's GitHub account for that purpose. The hardware-key text in the
approved trust-boundaries document is replaced by this choice; the rest of
that document stands.

## Approved revisions

SHA-256 of each file exactly as presented for this decision:

- `docs/specifications/proposal-2026-10-30.md`:
  `2a62368f0a955a7877631ce5371a9da7a485bd8e7a005c366a92e518e931765c`
- `docs/specifications/approval-delta-2026-10-30.md`:
  `e552761797dbe168f638920efc095d4698a0fdd6de6cba40f111ac7b26d6ef08`
- `docs/specifications/trust-boundaries.md`:
  `ad63408445829d9fccd24bca7661630c40ee67339e6119605151984c954fd340`
- `docs/specifications/main-only-publication.md`:
  `0278cf75f597981410d00d695956aa2a77af84c6290fcd67ba816981ea08848d`
- `docs/specifications/main-only-github-settings.md`:
  `bf8217d7148f59a92f7fd880865816dc4e4e34477ed99410d7a1d9305f001bb7`
- `docs/specifications/autoissues/README.md`:
  `9564a36751cd8568b04b14a447c24deffd0062c71d90f7063522a3a0978a1b63`
- `docs/specifications/autoissues/storage.md`:
  `c62d0b9986009194a55db2b1dba3d01bbfc99a70b52679adc40eea20b2881509`
- `docs/specifications/autoissues/lifecycle.md`:
  `252543323c0c705ea57fe07bd3974daa2b26c4155884194912da2ff27c7b7c3d`
- `docs/specifications/autoissues/interfaces.md`:
  `efdb7d5eb35011a302557e2bf741358d99f305bce730bcbf5f9a4038f61e5130`
- `docs/specifications/autoissues/adapters.md`:
  `aa4661f461c64590393e385aa85080eca97683339dda2b7bc0c15fac10027fd0`
- `docs/specifications/autoissues/qa-stack.md`:
  `20bf23c9480a7807acbf44915bcc0ff90586cdae522e688b42053e6648a53001`
- `docs/specifications/autoissues/quota.md`:
  `1d7689914d78c4ef643e9f6201a1fbce31d8c6db5822f42f773666699a799e31`
- `docs/specifications/autoissues/quota-recovery.md`:
  `782be50fcc0c008397a2d3188d13ebdbd5ba3b247a33a3165cd4614330543794`
- `docs/specifications/autoissues/acceptance.md`:
  `70039f4a0d70211d9ada70a6b64466e868b5c15e341f55ef292de46ebfc2e994`
- `docs/specifications/autoissues/acceptance-2.md`:
  `9b8892c43ae4d3cdaab0d3e1a97291a40139ffc6a08faf0e6e7a005ab5793973`
- `docs/specifications/autoissues/tool-support.md`:
  `2b924e204114f07bd004668ac1a64173b80c96aa47afa0958298a7d38d59bc79`
- `docs/specifications/performance-gate.md`:
  `b362e7afc723bed016bb2b16462a306e978d56a8057cf56a79d58043c1f41c39`
- `docs/specifications/engineering-gates.md`:
  `5b27e968cd19a40c204961f02f07c9436553b8cee26248582362f7b5b6e60f2d`
- `docs/specifications/manual-publishing-main-only.md`:
  `4db96f3cea6eca0ed489e5f26e61330e3194b7520d15ddc64d40da72e8343caa`
- `docs/specifications/release-main-only.md`:
  `6c901ed82dee355dc8a095619a99213dedcf74e1e59882bae82767dcbe71c83c`
- `docs/dev/rulebook-mapping.md`:
  `1915af4d7704aec0bc83ff1232cb470f0e8d7c5eba179300d987b426761e8e2f`
- `docs/dev/storage-live-data.md`:
  `a81bbe0d16331e1776e06deacf658d79ef28e6e5ae897d2be37ea8c1ffb82ed7`
- `docs/dev/source-limits-inventory.md`:
  `54da13055ac6863be21259cb085761e37b061ebd5f252226a84374ffe911d9ce`

## Scope

This approves the bounded implementation of these designs and their
tests. It does not approve a product release, paid provisioning, sending
private data to any provider, or changes to the approved texts. Owner
actions still needed (GitHub settings, Apps, test repository, Dell
accounts, read-only editing credential, reviewer sign-in) are listed in
the decision summary. Do not ask the owner to approve these same
unchanged designs again.
## Administrative changes after approval

- Each approved file received an "Approved by the owner" status note.
- Privacy redaction: exact host capacity figures, patch-level runtime
  versions and processor counts were removed from
  `docs/dev/storage-live-data.md` and
  `docs/specifications/autoissues/qa-stack.md`; the figures stay in the
  private operator record. No design text changed.
- `docs/specifications/trust-boundaries.md` gained a note recording the
  owner's choice in answer 3.
- `docs/dev/source-limits-inventory.md` records the owner's six
  source-limit answers and drops its link to SPEC-007, which the earlier
  compliance commit could not yet contain.
- `docs/specifications/autoissues/README.md`: one link updated to the
  bootstrap section's new home in quota recovery.
## Post-approval consistency corrections

An independent re-review after approval found places where the approved
texts did not yet follow the owner's recorded answers, or contradicted
each other. These corrections align them, without changing intent:

- SPEC-004, its settings, the trust boundaries and AutoIssues storage now
  use the owner's chosen GitHub-comment approval throughout, with its
  rules stated (owner user number, exact statement naming tree, commits
  and protected-file digest, never edited); owner records use the same
  route; the hardware-key and Administration-read audit steps are removed.
- The source-limit rule in the engineering gates lists the owner's three
  exceptions (single long links, historical quotation blocks, lockfiles).
- AT-D13 now matches the rule that a resolved webhook alone never proves
  absence; AT-G25 tests append-only quota tables.
- The activation order no longer asks for runner signatures before the
  runner exists; earlier milestones are local commits published together
  when the publisher activates.
- The transition table lets a fix checked under a then-live lease reach
  fixed after the lease expires; non-protected duplicates follow the gate.
- The journal anchor log is owned by the publisher; the gate can append
  but not rewrite it.

Four corrections involve a small design choice and await the owner's
confirmation; their implementation waits for it:

1. A gate's required GitHub check is added to the ruleset only when that
   gate activates (SPEC-004 bootstrap publication, AT-M17).
2. A reviewer account and reviewer sandbox, with network access only to
   the AI provider, run the required AI review (trust boundaries).
3. While agents share one account, the runner-produced AI review counts
   as the independent reviewer for credit lessons and repair-lane
   attribution (SPEC-006).
4. Agents and the gate query QA services through a read-only,
   credentialed proxy run by the QA account; nobody else can write to or
   administer them (QA stack).
Files changed by these corrections: `main-only-publication.md`,
`main-only-github-settings.md`, `trust-boundaries.md`,
`engineering-gates.md`, and in `autoissues/`: `storage.md`,
`lifecycle.md`, `interfaces.md`, `quota.md`, `quota-recovery.md`,
`qa-stack.md`, `acceptance.md`, `acceptance-2.md` and `tool-support.md`.