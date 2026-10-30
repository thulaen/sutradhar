# Feature specifications and approvals

Status: SPEC-001, SPEC-002 and SPEC-003 were explicitly approved on 30 October
2026. See the [approval record](approvals.md) for the exact reviewed revisions
and bounded scope. Other features still require their own approved designs.

The [current requirements](../requirements/current-decisions.md) remain the
authority. These designs turn the next bounded features into reviewable work.
Approval records identify the specification version/content, owner decision and
scope. Approval of one feature does not approve every later implementation.

## Approved foundation designs

- SPEC-001
  - Draft: [Toolchain and check runner](bootstrap-toolchain.md)
  - Decision requested: Check profiles, evidence contract, execution
    boundaries and failure behaviour.
  - Status: APPROVED FOR IMPLEMENTATION
- SPEC-002
  - Draft: [Manual publishing](manual-publishing.md)
  - Decision requested: User/manual separation, generation, preview/version
    workflow and honest unavailable-feature treatment.
  - Status: APPROVED FOR IMPLEMENTATION
- SPEC-003
  - Draft: [Agent and release coordination](agent-release-coordination.md)
  - Decision requested: Restricted GitHub broker, separate identities,
    protected decisions, ownership ledger and release sequence.
  - Status: APPROVED FOR IMPLEMENTATION

These approvals permit only the respective bounded implementations and their
tests. They do not approve production linking, analytics collection, paid cloud
provisioning, sending real content to AI providers, or a product release. Those
actions have their own requirements and verified preconditions.

## Full feature register

Every feature below remains in the agreed release scope unless an explicit later
owner decision changes it. Optional features may be disabled by default, but
their promised implementation and acceptance evidence are still required.

- F001
  - Capability: Configuration/profiles and atomic settings history
  - Governing requirements: R025, R041–R042
  - Specification state: NOT YET DRAFTED
- F002
  - Capability: Module registry, lifecycle and dependency coordination
  - Governing requirements: R041–R043
  - Specification state: NOT YET DRAFTED
- F003
  - Capability: Events, scheduled/batch work and durable recovery
  - Governing requirements: R008, R019–R023
  - Specification state: NOT YET DRAFTED
- F004
  - Capability: Signed PHP/Python contracts and guest-visible ingestion
  - Governing requirements: R001, R013, R043
  - Specification state: NOT YET DRAFTED
- F005
  - Capability: BBCode extraction and safe anchor placement
  - Governing requirements: R010–R013, R053
  - Specification state: NOT YET DRAFTED
- F006
  - Capability: Method research/admission catalogue and distinctness
  - Governing requirements: R004–R006, R027–R028, R067, R082
  - Specification state: RESEARCH FIRST; FEATURE APPROVAL STILL REQUIRED
- F007
  - Capability: Live method notebooks and lab execution
  - Governing requirements: R026, R058–R066
  - Specification state: NOT YET DRAFTED
- F008
  - Capability: Candidate generation, relevance, confidence and ranking
  - Governing requirements: R003–R006, R015, R019, R036
  - Specification state: NOT YET DRAFTED
- F009
  - Capability: Staff suggestions, permanent insertion and safe undo
  - Governing requirements: R009–R018
  - Specification state: NOT YET DRAFTED
- F010
  - Capability: Feedback learning and bounded autotuning
  - Governing requirements: R018, R024–R025
  - Specification state: NOT YET DRAFTED
- F011
  - Capability: First-party analytics and Adobe capability coverage
  - Governing requirements: R029–R032
  - Specification state: NOT YET DRAFTED
- F012
  - Capability: GSC, Bing and GA4 connectors
  - Governing requirements: R029, R033
  - Specification state: NOT YET DRAFTED
- F013
  - Capability: SEO signal collection, crawler and health reports
  - Governing requirements: R034, R051
  - Specification state: NOT YET DRAFTED
- F014
  - Capability: Knowledge graph and optional tag descriptions
  - Governing requirements: R035
  - Specification state: NOT YET DRAFTED
- F015
  - Capability: Sorting, usefulness feedback and karma
  - Governing requirements: R036–R038
  - Specification state: NOT YET DRAFTED
- F016
  - Capability: Moderation hints and optional expert routing
  - Governing requirements: R038
  - Specification state: NOT YET DRAFTED
- F017
  - Capability: Optional hubs, long-thread summaries and content drafts
  - Governing requirements: R039–R040
  - Specification state: NOT YET DRAFTED
- F018
  - Capability: GUI/action layer, sutra command discovery and MCP
  - Governing requirements: R044, R047–R048
  - Specification state: NOT YET DRAFTED
- F019
  - Capability: Third-party extension isolation
  - Governing requirements: R021, R028, R043
  - Specification state: NOT YET DRAFTED
- F020
  - Capability: Search services and optional cloud storage/provisioning
  - Governing requirements: R016, R032–R033, R049
  - Specification state: NOT YET DRAFTED
- F021
  - Capability: Point-and-click installation, upgrade and removal
  - Governing requirements: R049–R050
  - Specification state: NOT YET DRAFTED
- F022
  - Capability: Audit, erasure, diagnostics and safe exports
  - Governing requirements: R051–R052, R075
  - Specification state: NOT YET DRAFTED
- F023
  - Capability: Quality judging and release assessment
  - Governing requirements: R053–R066
  - Specification state: NOT YET DRAFTED
- F024
  - Capability: Public manual and user-facing references
  - Governing requirements: R078–R080
  - Specification state: SPEC-002 APPROVED
- F025
  - Capability: Developer checks, repository control and agent coordination
  - Governing requirements: R045–R046, R068–R077, R081
  - Specification state: SPEC-001/SPEC-003 APPROVED

This register is an ownership map, not the final exhaustive feature count. Each
large source request and every relevant source-inventory line must map to the
approved feature's detailed acceptance cases. New decomposition must preserve
the full requirement rather than replace it with a convenient subset.
