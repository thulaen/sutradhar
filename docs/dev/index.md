# Developer documentation

Audience: contributors and agents. Outcome: find the rules, setup plan and
evidence needed before changing Sutradhar.

Status: documentation foundation. Product implementation and automated
enforcement must be verified separately. This index does not assert that any
proposed command, workflow, GitHub setting, test suite or release pipeline
already exists.

## Start here

- Document: [Agent rules](../../AGENTS.md)
  - Purpose: Rules that every coding agent follows
- Document: [Contribution guide](../../CONTRIBUTING.md)
  - Purpose: How contributors propose and submit work
- Document: [Governance](governance.md)
  - Purpose: Decisions, approvals, ownership and evidence
- Document: [Documentation bootstrap](bootstrap.md)
  - Purpose: The first checked documentation checkpoint
- Document: [Gate contract](gates.md)
  - Purpose: What blocks a commit, push, merge and release
- Document: [Repository setup](repository-setup.md)
  - Purpose: GitHub setup and acceptance evidence
- Document: [Toolchain](toolchain.md)
  - Purpose: Requested tools, responsibilities and qualification
- Document: [Release process](releasing.md)
  - Purpose: Candidate tests, publication and manual versions
- Document: [Mapped coding rulebook](rulebook.md)
  - Purpose: Proposed PHP/Python adaptations for owner review
- Document: [Governance source review](governance-source-review.md)
  - Purpose: Author review with explicit evidence limits
- Document: [Security](../../SECURITY.md)
  - Purpose: Reporting and privacy boundaries

The requirements directory records the 209-question interview and superseded
decisions. Feature specifications live in the specifications directory. Those
documents must link current requirements to acceptance evidence; they must not
replace unknown results with promises.

## Documentation ownership

Developer documents explain architecture, rules and implementation contracts.
Method notebooks explain individual methods and import production functions.
The public manual explains installation and day-to-day operation to forum
owners, administrators and moderators. Do not publish this developer tree as
the manual.

Each explanation has one authoritative home. Use short summaries and links
rather than duplicate copies. Follow the public
[Google documentation guide](https://google.github.io/styleguide/docguide/)
for G3Doc-style Markdown, organisation and maintenance. This is a project
documentation convention, not an external certification.

## Evidence vocabulary

| Status | Meaning |
| --- | --- |
| Proposed | Written design awaiting the required approval |
| Approved | Owner decision recorded for a specific specification revision |
| Implemented | Source exists; this alone does not prove behaviour |
| Verified | Named checks passed for identified content and environment |
| Qualified | Every acceptance condition for the stated scope has evidence |
| Blocked | A required condition is missing or failed |
| Not applicable | A documented scope rule excludes a check for this candidate |

Never use a passing focused test to label a whole feature or release qualified.
