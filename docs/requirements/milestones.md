# Milestones and feature requests

Status: PLANNED. Each stage needs its own evidence; none is a complete release
until the full scope and release conditions are met.

## Delivery order

| Stage | Outcome | Exit evidence |
| --- | --- | --- |
| M00: requirements | Capture all interview decisions, large source requests, privacy redactions and explicit overrides. | A reviewed source register with no missing question numbers and no identifying material; each active requirement has an acceptance owner. |
| M01: protected foundation | Prepare the authorised validation environment; establish documented repository protections, dedicated automation identities and the documentation checkpoint. | Host/storage records held privately; checked documentation; actual protection settings; no claims that unimplemented checks pass. |
| M02: approved checking and publishing | Build the approved check runner, manual workflow and protected coordination/release services. | Owner-approved specifications, tests of failure paths, trustworthy status reporting and a deployed current-state manual. |
| M03: research catalogue | Research and distinguish the full method catalogue before concentrating on the complete linking flow. | Source/rights records, duplicate analysis, mathematical specifications and a defensible path to at least 6,000 qualified methods; researched entries are not yet counted as implementations. |
| M04: framework and methods | Implement approved core contracts and methods with live notebooks, bounded workers, checkpoints and feedback handling. | Per-feature approvals, correctness and usefulness evidence, resource measurements and registry counts derived from qualified entries. |
| M05: forum linking and review | Complete public-content ingestion, suggestions, staff review, permanent insertion, suppression, undo and recovery. | End-to-end XenForo evidence for hard rules, stale/permission cases, million-post workload and resource budgets. |
| M06: complete feature scope | Complete analytics, SEO, knowledge graph, optional hubs/tag descriptions, ranking/karma, summaries, moderation hints, drafts, integrations and optional cloud/search/storage features. | A completed capability matrix and feature acceptance packets, without quietly dropping optional-but-required-to-ship modules. |
| M07: full release qualification | Qualify the complete candidate, at least 6,000 distinct methods and more than 10,000 meaningful playbook cases. | All ordinary gates, fixed/fresh judging, coverage, accessibility, browser evidence, 72-hour soak and clean installation using only the manual. |
| M08: release and operation | Publish the qualified code and matching frozen manual, monitor operation and run the agreed ongoing jobs. | Final artifact hashes match the candidate; release/manual publication evidence; verified recovery, alerts and privacy handling. |

Stages may overlap only where their dependencies and ownership allow it.
Documentation, research and internal milestones do not authorise an incomplete
public product release. Necessary approved check infrastructure supports the
catalogue milestone; it does not replace that milestone with a smaller product.

## Features must not disappear between stages

Maintain a feature record for each capability in the
[specification index](../specifications/README.md), every active requirement in
[current decisions](current-decisions.md) and every applicable item from the
[source inventory](source-items.csv). Distinguish:

- requirement recorded;
- specification drafted and separately approved;
- implementation present;
- checks passed for the current candidate;
- installed behaviour proved;
- release admitted.

Never infer a later stage from an earlier one. A completed feature has links to
the approved specification, changes, test evidence and manual page, plus any
remaining limitations. An optional module may ship disabled but still needs its
agreed implementation, documentation and tests.

## Feature-request records

New requests receive a stable FR number and live in the feature-request folder
under this directory. Record the original privacy-safe request, affected current
requirements, proposed outcome, overlap search, dependencies, specification,
owner decision and acceptance evidence. Keep rejected and superseded requests
with reasons rather than deleting them.

Before adding a feature or method, search the existing register for equivalent
behaviour. Reuse the existing entry or document the material difference. A new
name is not evidence that the behaviour is new. Reprioritisation and changes to
approved requirements need the owner's approval; agents must not silently shrink
a milestone to make it appear complete.

The existing interview defines the initial backlog; it is not 209 automatically
independent feature requests. Its large requests may decompose into many feature
records, but every record must link back to its exact source.

## Progress and interruption

The private run ledger tracks task ownership and recoverable progress. It is not
committed. Publish only scrubbed summaries and approved aggregate statistics.
When a run stops, inspect the live process and retained evidence before resuming
or replacing it. A missing response is not proof that a process has stopped.

All project work pauses when the validation host is unavailable. Resume from
recorded checkpoints when it returns; never turn an outage, usage limit or
missing permission into a passed check.
