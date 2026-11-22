# Documentation completeness audit

Apply [spec 0076](../specs/0076-specification-completeness.md) to a fixed
candidate, its effective decisions and its protected evidence. Use
the [trace admission procedure](test-and-traceability.md) for the five
feature waves. Record current work, findings and dispositions in
Forgejo.

## Close the independent inventories

1. Compare the original feature lists, protected pasted item keys,
   feature register and acceptance rows by exact key in both
   directions. Inspect every deferred, unsupported, held or alias
   row. Require a governing refusal, limit or canonical owner.
   Record the source and title of each mismatch and the affected
   requirement and evidence owner. Recompute the total and each
   per-owner count from the exact registered rows. Check every
   recorded owner against the numbered specification index. Use
   the source basis and governing decisions to resolve heuristic
   owner assignments before claiming their semantic trace.
2. Independently audit spec 0064's expanded scope, admitted methods,
   interfaces, variants and operational duties. The feature waves
   cover their source feature IDs. Each other inventory needs its
   own exact set and requirement mapping.
3. Every named task in the
   [implementation catalogue](implementation-catalogue.md) needs one
   authoritative trace owner with its case obligations.
   A task may own several cases, or share explicit proof with a
   different task. Incidental mentions in a dependency or narrative
   do not establish the trace. Inspect the owning row or section,
   its acceptance cases and the destination of each result.
4. Check whether every in-scope behaviour can yield its exact
   derived task under 0076. Start with the owning contract and
   preserve roadmap order and the precise dependency slice. Record
   the owner, scoped requirement and design IDs, variants, allowed
   files, exact cases and evidence maker. A proposed decomposition
   can expose missing edges. Report those edges before requesting
   admission. Bind live issues through the catalogue's crosswalk.
5. Review consequential overlaps and constraints independently.
   Compare typed interfaces, authority, data scope, protection,
   budgets and recovery in both directions. Each owner uses the
   same approved boundary. Case or link presence alone cannot
   establish semantic equivalence.

## Audit each owning specification

1. Freeze the indexed specification paths and bytes at the exact
   candidate. Compare the [specification index][spec-index] against
   the candidate's numbered files both ways. Use the current scope
   decision for each applicable, conditional, excluded or separately
   held design. A linked title is an inventory entry only.
2. Read each specification as a source of obligations before looking
   at tasks or test names. Derive the exact observable conditions,
   effects, refusals, limits and transitions from its governing
   sections, including prose and decision tables. Find each clause's
   owning requirement and design IDs under 0076. A legacy clause
   without an owning ID remains an unmatched obligation. Record its
   exact section, source location and reviewed content digest in the
   existing audit evidence. A lexical count of `REQ` labels does not
   define the expected set. Keep reused, replaced and excluded
   clauses tied to their one current owner and governing decision.
3. Independently read the owning test and task records in the other
   direction. Join each case to its exact owning requirement,
   effect, variant and refusal. Verify both directions against the
   existing [trace format][requirement-trace]. Check each shared
   case's actual assertions for each claimed caller and platform.
   Check unit, integration, end-to-end, security, performance and
   formal cases where applicable. Record the reason for each case
   class that does not apply. Case names and links alone cannot
   prove an omitted edge. Compare the two sets and identify an
   unmatched requirement, orphan case, duplicate owner or scope
   mismatch with its exact source and case. Group dispositions by
   owning specification and keep case design, implementation and
   execution claims separate under the existing evidence rules.
4. Record the reviewed evidence, exact unresolved links and owner in
   Forgejo. A missing stable identity, applicable variant or case
   holds the affected acceptance claim under 0076. This audit does
   not create a task, change phase order or supply a gate result.

| Governing obligation and owner | Exact consumer and planned case | Reverse check |
| --- | --- | --- |
| [0005 Linux Consumers][linux-package-consumer] requires the admitted native dependency contract before unpack or scripts. The review starts at that clause, independently of the checklist. | [0030 Linux package handoff][linux-package-handoff] owns native use and the SUT-832 refusal. Match the Linux update branch in `IT-0030-VERIFIED-STAGE` to [the update task trace][update-task-trace] on each admitted Ubuntu and Debian profile. | Compare an otherwise valid out-of-profile dependency, an identical visible plan without independent current proof and a substituted resolution after verification with unchanged signed bytes. Each holds before unpack, dependency installation, scripts or activation. The compatible control enters the remaining checks. A shared case name supplies no substitute for either owning requirement ID. This update binding gives no install, repair, Windows or ZIP evidence. |

[spec-index]: ../specs/README.md
[requirement-trace]: test-and-traceability.md#trace-format
[linux-package-consumer]: ../specs/0005-release-coordination.md#consumers
[linux-package-handoff]:
  ../specs/0030-install-upgrade-removal.md#linux-package-trust-and-handoff
[update-task-trace]: test-and-traceability.md#task-0030-update-repair

## Working-tree preflight

A preflight can find document defects before a candidate enters the
protected tier. Record the base and the exact reviewed paths and bytes,
including new files. Record each check used and the part of the audit
it covers. Recheck changed bytes before relying on its findings.

When reconciling checkouts, record Forgejo's published `main` commit
and tree separately from the door's accepted base under [0001].
Compare each reviewed view's exact paths, file modes and bytes.
Record the live protected policy digest from the applicable protected
base or admitted installed copy under [0008]. Keep unverified or
unresolved differences as preflight findings in Forgejo and hold the
affected acceptance claim under spec 0076.

[0001]: ../specs/0001-door.md#what-it-does
[0008]: ../specs/0008-trust-boundaries.md#protected-policy-and-installed-tools

- `git -c diff.autoRefreshIndex=false diff --check` gives a whitespace
  diagnostic for unstaged tracked differences. Inspect new files and
  any differences outside that comparison separately.
- Inspect changed Markdown links, anchors and permission statements
  against their actual destinations. Include changed tables and stable
  test references. Review semantics under 0076. A renderer or spelling
  check cannot establish the owning behaviour or permission.
- Use the pinned `typos` version with `_typos.toml` for spelling when
  that tool and its hash are available. Record it as spelling evidence.
  An owner manual preview and its release checks follow spec 0004.

Use `aegis catalogue` to inspect check coverage. `aegis fingerprint`
identifies the staged tree for review. `aegis commit` checks a staged
snapshot and runs the tier's source checks. The ordinary `aegis push`
route requires a clean checkout. These commands do not offer a
standalone Markdown check on working-tree bytes. The Markdown check
covers local inline links and headings within its parser's scope.

Preflight observations, fixture results and signed tier evidence are
separate records. The trusted runner selects the documentation-only
profile under 0003 when applicable. Retain mandatory links, consistency,
privacy, secrets, rights and independent review checks. Before claiming
tier acceptance, bind its signed result to the exact candidate, policy
and profile.

## Verify the dependent documents

| Area | Independent comparison |
|---|---|
| Owner help | Under spec 0004, compare the actions, errors and platform journeys with the same-release accessible manual, generated reference and offline recovery. Verify the matching packaged source and exact released asset as separate records. |
| Version transitions | Compare each applicable upgrade, coexistence and rollback combination and its acceptance case with the owning contract. Match the target, caller or binary, interface and stored format. Verify applicable refusal, timing and preserved effects before relying on proof of interrupted recovery. |
| Authoring and gate | From AGENTS.md and the owning spec, bind each applicable language and tier to its exact rule, standards mapping, protected check or review and planted violation. Distinguish clean and planted check evidence. |
| Supply chain | Apply spec 0003's inventory and the joined supply-chain evidence in the trace catalogue. Match approved sources, locks, licence, tool and image identities and SBOM to resolved inputs and exact artefact bytes. |
| Release | Use the Phase 7 trace owners for exact candidate qualification, the verified unsigned payload, signed approval, final package verification and publication read-back. Verify Windows first trust separately. |

## Separate four conclusions

| Claim | Required evidence |
|---|---|
| Structural coverage | Exact source, owner, case and task identity comparison, with the complete applicable inventory and declared relation type. |
| Design readiness | Complete 0076 contracts and feature-specific verification designs. Inspect each applicable language, platform, authority, rights and research dependency and independently review semantic closure. |
| Authority to begin a build task | Verified design readiness, current roadmap and prerequisite proof, one admitted task with exact scope, and its applicable authority. Prospective task design can precede that authority. |
| Execution and release | Observed results and independent signed evidence for the exact candidate, required tests, profiles and tiers. Apply full release and signing qualification under the owning specs. |

Preserve the exact baseline failure, test design and candidate
results as distinct evidence. Pin the source, candidate, fixture and
policy identities and reviewed inventory digests. Identify each
defect, what would resolve it and its owner in the authorised record.
Rerun the affected audit when the contract, candidate or authority
changes.
