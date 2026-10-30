# Feature specifications and approvals

Status: SPEC-001, SPEC-002 and SPEC-003 were explicitly approved on 30 October
2026. See the [approval record](approvals.md) for the exact reviewed revisions
and bounded scope. Other features still require their own approved designs.

The [current requirements](../requirements/current-decisions.md) remain the
authority. These designs turn the next bounded features into reviewable work.
Approval records identify the specification version/content, owner decision and
scope. Approval of one feature does not approve every later implementation.

## Approved foundation designs

| Feature | Draft | Decision requested | Status |
| --- | --- | --- | --- |
| SPEC-001 | [Toolchain and check runner](bootstrap-toolchain.md) | Check profiles, evidence contract, execution boundaries and failure behaviour. | APPROVED FOR IMPLEMENTATION |
| SPEC-002 | [Manual publishing](manual-publishing.md) | User/manual separation, generation, preview/version workflow and honest unavailable-feature treatment. | APPROVED FOR IMPLEMENTATION |
| SPEC-003 | [Agent and release coordination](agent-release-coordination.md) | Restricted GitHub broker, separate identities, protected decisions, ownership ledger and release sequence. | APPROVED FOR IMPLEMENTATION |

These approvals permit only the respective bounded implementations and their
tests. They do not approve production linking, analytics collection, paid cloud
provisioning, sending real content to AI providers, or a product release. Those
actions have their own requirements and verified preconditions.

## Full feature register

Every feature below remains in the agreed release scope unless an explicit later
owner decision changes it. Optional features may be disabled by default, but
their promised implementation and acceptance evidence are still required.

| Feature ID | Capability | Governing requirements | Specification state |
| --- | --- | --- | --- |
| F001 | Configuration/profiles and atomic settings history | R025, R041–R042 | NOT YET DRAFTED |
| F002 | Module registry, lifecycle and dependency coordination | R041–R043 | NOT YET DRAFTED |
| F003 | Events, scheduled/batch work and durable recovery | R008, R019–R023 | NOT YET DRAFTED |
| F004 | Signed PHP/Python contracts and guest-visible ingestion | R001, R013, R043 | NOT YET DRAFTED |
| F005 | BBCode extraction and safe anchor placement | R010–R013, R053 | NOT YET DRAFTED |
| F006 | Method research/admission catalogue and distinctness | R004–R006, R027–R028, R067, R082 | RESEARCH FIRST; FEATURE APPROVAL STILL REQUIRED |
| F007 | Live method notebooks and lab execution | R026, R058–R066 | NOT YET DRAFTED |
| F008 | Candidate generation, relevance, confidence and ranking | R003–R006, R015, R019, R036 | NOT YET DRAFTED |
| F009 | Staff suggestions, permanent insertion and safe undo | R009–R018 | NOT YET DRAFTED |
| F010 | Feedback learning and bounded autotuning | R018, R024–R025 | NOT YET DRAFTED |
| F011 | First-party analytics and Adobe capability coverage | R029–R032 | NOT YET DRAFTED |
| F012 | GSC, Bing and GA4 connectors | R029, R033 | NOT YET DRAFTED |
| F013 | SEO signal collection, crawler and health reports | R034, R051 | NOT YET DRAFTED |
| F014 | Knowledge graph and optional tag descriptions | R035 | NOT YET DRAFTED |
| F015 | Sorting, usefulness feedback and karma | R036–R038 | NOT YET DRAFTED |
| F016 | Moderation hints and optional expert routing | R038 | NOT YET DRAFTED |
| F017 | Optional hubs, long-thread summaries and content drafts | R039–R040 | NOT YET DRAFTED |
| F018 | GUI/action layer, sutra command discovery and MCP | R044, R047–R048 | NOT YET DRAFTED |
| F019 | Third-party extension isolation | R021, R028, R043 | NOT YET DRAFTED |
| F020 | Search services and optional cloud storage/provisioning | R016, R032–R033, R049 | NOT YET DRAFTED |
| F021 | Point-and-click installation, upgrade and removal | R049–R050 | NOT YET DRAFTED |
| F022 | Audit, erasure, diagnostics and safe exports | R051–R052, R075 | NOT YET DRAFTED |
| F023 | Quality judging and release assessment | R053–R066 | NOT YET DRAFTED |
| F024 | Public manual and user-facing references | R078–R080 | SPEC-002 APPROVED |
| F025 | Developer checks, repository control and agent coordination | R045–R046, R068–R077, R081 | SPEC-001/SPEC-003 APPROVED |

This register is an ownership map, not the final exhaustive feature count. Each
large source request and every relevant source-inventory line must map to the
approved feature's detailed acceptance cases. New decomposition must preserve
the full requirement rather than replace it with a convenient subset.
