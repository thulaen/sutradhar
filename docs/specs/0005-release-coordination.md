# 0005: Agent and release coordination

This part lets agents work together unattended while the owner's
protected decisions, private data and release authority stay enforced.
It adds only what is not already in
[AGENTS.md section 8](../../AGENTS.md#8-how-a-session-works) (the
session steps and file locks) and
[section 8.1](../../AGENTS.md#81-releases) (the release task), and sets
out the release steps in full. It is roadmap Phase 7 item 3.

## What it does

### Who holds which power

1. Writing code, recording check results, recording release approval and
   publishing a release are separate powers, each held by its own
   identity on Dell.
2. Dell service keys and tokens stay outside candidate checkouts.
   Owner keys stay on the owner's PC and are never imported to Dell.
   Coding agents receive no key or ambient owner credential.
3. The protected services expose no general Forgejo or GitHub access,
   no shell and no free reference update. No coding identity can make a
   tag, write `main` other than by an ordinary push through the door,
   weaken a workflow, or publish through an existing tag.
4. Accept evidence only from qualified runners under current policy.
   DR-025 permits a task-scoped approval from the owner or a release
   agent. Accept only a current signed `release-approval` receipt.
   Code, check and publication roles cannot issue that receipt.
   [Owner waivers](../../AGENTS.md#72-waivers), policy changes and key
   custody keep their own signing powers. A comment gives no authority.

### Joining, leases and handover

1. Agents join explicitly. Mailbox messages are transport, never leave
   to edit another agent's files or approve its changes.
2. A private run ledger on Dell records each agent's identity, its task,
   the approval of the specification it works under, its current lease
   and its activity. It also records runs, attempts, file ownership,
   evidence references, publication state and handovers. It never holds
   a copy of a credential. Only scrubbed summaries leave Dell.
3. One writer per file. A clashing lock request is refused or queued.
4. A handover keeps the draft and its checkpoints, records the reason,
   revokes the old lease and issues a new one. A stale worker is shut
   out of later writes, even if it wakes after its replacement starts.
5. No agent discards another worker's staged or uncommitted work to
   make its own task easier.
6. Writes use durable transactions. Before a half-finished write is
   resumed, a recovery check runs. A retry never makes a second commit,
   tag, release or manual deployment.

### When Dell is not available

1. Work starts, and pushes are admitted, only after Dell is shown to be
   available now. An old green result is not proof.
2. While Dell is unavailable, all ordinary project work pauses. A health
   check may notice the return and report state, but does no project
   work during the pause.
3. Queued work and durable progress are kept.
4. A needed service being down pauses the manual's deadlines and leaves
   release checks unfinished.
5. The availability monitor, lease expiry, job pause and scheduled jobs
   are all tested before the pause rule is claimed to be enforced.

### Reviews

The required reviews of AGENTS.md section 7.1 must each name the current
candidate and finish. A reaction, silence, a stale review, a partial
scope or a skipped job is never a pass. A reviewer never wrote the
change it reviews.

### A broken `main`

If `main` breaks, the first response is a revert commit, pushed through
the door like any other commit, never an unrelated fix forward.

### Dependency updates

1. A weekly dependency task on Dell updates the locked versions, grouped
   as Rust crates, PHP libraries and pinned tools.
2. Each group runs the full tiers, including the speed check's reuse and
   invalidation rules (0007). Any new dependency is admitted through the
   licence rules of AGENTS.md section 5.7.
3. Each group lands as one commit through the normal session steps.
4. A group that fails any check is not pushed, and autoissues records
   its findings. They become Forgejo tasks only when an agent takes one
   on, linked to it (the owner, 10 November 2026: "Keep apart, link
   when taken"; spec 0063, "Where faults go").
5. Security findings from GitHub's dependency alerts, cargo-audit,
   `composer audit` and the container image scanner feed autoissues. A
   finding of the two highest severities is filed in autoissues as P0
   or P1, with a Forgejo task only when an agent takes it on.
6. No dependency step makes a branch or a pull request.

## The release steps

A release is a Forgejo task, as AGENTS.md section 8.1 sets out. Its run
goes as follows.

1. **Scope preflight.** Check that `main` passes and that the
   approved inventory and variant designs can enter qualification.
   Verify the current signed Phase 3 closure and the intervening
   prerequisites under 0076. Read the accepted phase proof through
   0010 and 0139, using the existing owning evidence for the later
   prerequisites. This authorises a candidate attempt. The
   version commit and exact release still need their own checks
   and evidence.
2. **Version commit.** On a clean Dell copy of `main`, the release step
   makes one version commit holding the new version and the changelog,
   both from the commit messages. It passes the commit and push tiers
   like any commit.
3. **Candidate tag.** The release step makes a candidate tag (for
   example `v1.0.0-rc.1`) on that exact commit and starts a durable
   release run. Only the release step may make tags; the door refuses
   any other tag push (0001).
4. **Release run.** The run carries out:
   - the full quality assessment on fixed and fresh data;
   - the resource and environment checks, including the peak memory,
     load and soak tests on the test machine that matches the owner's
     PC;
   - a 72-hour supervised soak;
   - the spec 0134 release load receipt for this exact candidate
     and signed workload, with each scenario verdict and required
     independent p99 review. Staging keeps its own long receipt.
   It resumes across provider usage windows. It records the exact
   content, model, scoring guide and data identities. It never reruns or
   publishes a changed candidate on old evidence.
5. **Quality bars.** The run requires no fall in reconstruction recall
   or anchor agreement, and requires the hard-rule and coverage checks
   to pass. After launch, require `production-review-acceptance` under
   DR-152. [Spec 0057][field-acceptance] owns the decisions, reference,
   comparison and outcome. A required criterion without admitted
   evidence holds release approval.
6. **Approval.** Verify all required checks, independent reviews,
   scope, the spec 0079 floor, spec 0114 long lane, spec 0116
   reproducible build evidence and spec 0118 encryption proof for
   the exact unsigned payload manifest. Accept the scoped signed
   approval only when all requirements pass.
7. **Final release.** Recheck authority and evidence. The release
   step signs the approved manifest under the detached delivery
   profile and verifies the exact artefact bytes against it. Only
   then can it create the final tag and publish. Attach the exact
   manual asset. Verify
   the mirror, then read back the authorised Pages deployment
   under spec 0004. A hold before dispatch leaves `latest` in place.
   A lost reply or read-back mismatch holds verification of the
   new deployment. Keep the last verified receipt and reconcile
   only the original Pages attempt. Do not repeat the release.
8. **Same code.** Any packaging or version change that alters the
   tested artefact needs the same checks again. A changed commit needs a
   new candidate.
9. **Failed candidates.** A failed candidate is kept, with a scrubbed
   summary of why it failed. No final tag, product release or manual
   promotion follows.
10. **Recovery.** If the release step crashes after making an artefact,
    recovery checks what exists and finishes once, never making a second
    release. Before approval, reconcile a completed fresh release check
    only through its original authenticated runner result for the same
    authorised run and exact attempt, check, target and input profile. Apply
    current authority and evidence checks. A signed cache entry cannot
    satisfy a required fresh release check under 0010, even with
    identical inputs. An unresolved required check keeps approval held.

### Phase 7 closure after publication

The [Phase 7 roadmap](../roadmap.md#phase-7-version-1) and AGENTS.md
section 10.5 require a signed check to close the milestone. QR-244
owns the protected closure profile. Match the current accepted Phase 6
lineage, the exact `v1.0.0` candidate, 0064 qualification and full-gate
evidence, scoped approval, signed artefacts, exact Forgejo and mirror
tag and release, and 0004's matching Pages read-back. Verify the
required phase reviews and authoritative milestone and task conditions.

A missing, stale or mismatched closure proof holds the milestone.
Preserve each verified publication receipt. Reconcile a lost reply to
an already performed action only through its exact pending effect and
owning read-back. Do not retry while its outcome remains uncertain.
This closure check follows publication and grants no authority to
sign, tag, publish or redeploy.

### Exact candidate, manifest and record

The candidate tag and durable run identify one release attempt.
Freeze the notice, exact unsigned deliverables and required
members before recording a qualification or encryption receipt
for their digest. The verifier reads the run, tag and resolved
commit from the protected release context.

#### Canonical bytes

| Notation | Bytes |
| --- | --- |
| `U(n)` | One unsigned 64-bit integer, big-endian. |
| `T(s)` | `U(len(b)) \|\| b`, where `b` is the exact UTF-8 byte sequence for `s`. There is no normalisation, case conversion or terminator. |
| `B(b)` | `U(len(b)) \|\| b` for a byte sequence. |
| `D(h)` | Exactly 32 raw bytes, with no length or text encoding. |
| `H(b)` | SHA-256 of the exact bytes `b`. |
| `Rec(type, fields)` | `T(type) \|\| U(1) \|\| fields`, in the order below. |

`||` is concatenation. All lists are `U(count)` followed by that
many complete entries with no extra separators. Check every length
against its remaining input and the protected limits before
allocation or recursion. Refuse unknown format or location versions,
an out-of-limit count, invalid text, unexpected fields and trailing
bytes. Require the exact admitted ID spelling. Reject a malformed
or ambiguous ID. Do not repair it into a different identity.

#### Frozen inventory and candidate

The protected packaging profile supplies the approved role, target,
name and location for every required unsigned part, with extraction
and equality rules for that platform. Record the digest of that
exact profile in the candidate. A submitted record cannot add an
optional part, remove a required part or choose a different profile.

| Record or entry | Fields in order |
| --- | --- |
| Part key | `T(role_id) \|\| T(target_id) \|\| T(part_name)`. |
| Standalone location | `U(0)`. Locate the exact file by its part key in the approved delivery profile. |
| Member location | `U(1) \|\| parent_part_key \|\| T(member_path)`. The parent must identify exactly one standalone container row. Resolve the exact parent and member through the admitted parser and extraction profile. No implicit path rewriting, alternate match or second lookup. |
| Part entry | `part_key \|\| location \|\| U(byte_length) \|\| D(H(part_bytes))`. Hash the stored standalone file or the exact extracted member bytes, respectively. |
| Payload inventory `I` | `Rec("release-payload-inventory", U(part_count) \|\| part_entries)`. |
| Candidate `C` | `Rec("release-candidate", T(repository_id) \|\| U(task_number) \|\| T(run_id) \|\| T(commit_object_id) \|\| D(source_tree_digest) \|\| T(candidate_tag) \|\| T(version) \|\| D(H(I)) \|\| D(release_notice_digest) \|\| D(protected_policy_digest) \|\| D(signer_registry_digest) \|\| D(packaging_profile_digest))`. |

Sort part entries by the unframed unsigned UTF-8 byte tuple
`(role_id, target_id, part_name)`, in ascending order. The reader
requires that exact order and refuses duplicates. Do not sort
length-prefixed bytes. Reject missing, extra or substituted parts,
duplicate member references, unsafe paths and scope or filesystem
collisions. An admitted parser must reject ambiguous archive members
before extraction. Each member row binds its parent's identity and
digest through the complete inventory. The parent is also
independently hashed as a standalone part.

Read repository, task, run, tag, version and commit from the protected
release context. Use the exact 0064 trusted source-tree digest. Read
the policy, signer registry and packaging-profile digests from their
protected sources. The exact 0005 versioned notice and frozen manual
are required inventory parts. Compare `release_notice_digest` with
the verified bytes of the one designated notice entry. No field takes
an independently supplied hint when its trusted source is missing.

`unsigned_payload_manifest_digest = H(I)`. Verify this digest
against the exact stored inventory and actual unsigned bytes before
accepting qualification evidence. The candidate digest is `H(C)`.
Both hashes omit signing-purpose prefixes. The 0064 manifest and
ledger bind the exact candidate digest and its shared release
context. The ledger does not enter the candidate preimage.

#### Frozen inventory requirement identities

Apply the owning clauses directly. Shared fixture evidence retains
its original producer, authority and scope.

| Design contract | Owning clauses | Existing task |
| --- | --- | --- |
| `DES-0005-FROZEN-RELEASE-CANDIDATE` | [Exact attempt and freeze](#exact-candidate-manifest-and-record), protected inputs and qualification admission in [frozen inventory](#frozen-inventory-and-candidate) | TASK-0005-RELEASE-CANDIDATE |
| `DES-0005-CANONICAL-RELEASE-RECORDS` | [Canonical bytes](#canonical-bytes), inventory forms and [caller encoding](#existing-signed-record-boundaries) | TASK-0005-RELEASE-CANDIDATE and TASK-0065-RELEASE-BINDING for their respective consumers |
| `DES-0005-RELEASE-INVENTORY-VERIFICATION` | [Frozen inventory](#frozen-inventory-and-candidate), [delivery profile](#detached-delivery-profile) and [verification before use](#before-release-or-package-use) | TASK-0005-RELEASE-CANDIDATE and TASK-0065-RELEASE-BINDING |

| Owning requirement | Exact clause and existing planned evidence |
| --- | --- |
| `REQ-0005-CANDIDATE-ATTEMPT` | [Tag, resolved commit and durable run](#exact-candidate-manifest-and-record). IT-0005-CANDIDATE-RECOVERY verifies exact binding and refuses changed or incomplete state. |
| `REQ-0005-CANDIDATE-FREEZE` | [Freeze exact parts, notice and manual before evidence credit](#exact-candidate-manifest-and-record). IT-0005-CANDIDATE-RECOVERY checks missing and incomplete freeze read-back. |
| `REQ-0005-RELEASE-RECORD-FORMAT` | [Scalar bytes, versions and exact framing](#canonical-bytes). [Encoding fixtures][release-inventory-fixtures] verify golden bytes, unsupported versions, missing fields and trailing bytes. |
| `REQ-0005-RELEASE-DECODE-BOUNDS` | [Covered length and count checks before allocation or recursion](#canonical-bytes). [Encoding fixtures][release-inventory-fixtures] exercise exact limits, out-of-limit counts and truncated input; IT-0005-RELEASE-APPROVAL independently checks an unavailable protected bound. |
| `REQ-0005-RELEASE-TEXT-IDENTITY` | [Exact UTF-8 and admitted spelling](#canonical-bytes). [Encoding fixtures][release-inventory-fixtures] compare exact multibyte sequences, malformed text and changed ID spelling. |
| `REQ-0005-PACKAGING-PROFILE-MEMBERSHIP` | [Exact selected profile and required parts](#frozen-inventory-and-candidate). IT-0005-CANDIDATE-RECOVERY checks missing and extra parts and a competing admitted profile. |
| `REQ-0005-INVENTORY-LOCATION` | [Standalone and member location forms and exact lookup](#frozen-inventory-and-candidate). [Encoding fixtures][release-inventory-fixtures] cover both forms, a non-standalone parent and the absent exact path despite matching alternate bytes. |
| `REQ-0005-INVENTORY-CANONICAL-KEYS` | [Ordering and unique keys and member references](#frozen-inventory-and-candidate). [Encoding fixtures][release-inventory-fixtures] separate sort order, duplicate part key and repeated member reference. |
| `REQ-0005-RELEASE-PATH-SAFETY` | [Archive structure, containment and collisions](#before-release-or-package-use). IT-0005-DETACHED-MIXUP covers ambiguity and exact profile path refusal. Require the applicable admitted QR-202 relation and per-profile proof. |
| `REQ-0005-INVENTORY-PART-BYTES` | [Parent and member sizes and byte digests](#frozen-inventory-and-candidate). IT-0005-CANDIDATE-RECOVERY varies declared length. BDD-0005-DETACHED-EXACT and IT-0005-DETACHED-MIXUP exercise byte equality. |
| `REQ-0005-CANDIDATE-TRUSTED-INPUTS` | [Protected run context and digests](#frozen-inventory-and-candidate). IT-0005-CANDIDATE-RECOVERY withholds the protected signer registry despite a matching candidate claim. |
| `REQ-0005-FROZEN-NOTICE-AND-MANUAL` | [Exact designated notice digest and required frozen manual](#frozen-inventory-and-candidate). [Encoding fixtures][release-inventory-fixtures] and IT-0005-CANDIDATE-RECOVERY cover notice mismatch and missing manual. |
| `REQ-0005-CANDIDATE-PAYLOAD-DIGESTS` | [Raw H(C), H(I) and separate signature domains](#frozen-inventory-and-candidate). [Encoding fixtures][release-inventory-fixtures] pin the exact raw digests. |
| `REQ-0005-QUALIFICATION-INPUT-BINDING` | [Stored and actual payload bytes before qualification credit and ledger binding to frozen H(C)](#frozen-inventory-and-candidate). IT-0005-CANDIDATE-RECOVERY joins IT-0064-METHOD-FLOOR and IT-0064-REPLAY-CHANGE for their own ledger checks. |

[release-inventory-fixtures]: ../planning/release-encoding-fixtures.md

| Location variant | Exact form and associated proof |
| --- | --- |
| `VAR-0005-PART-STANDALONE` | `U(0)` selects the exact stored file by part key. Verify its own length and digest, including when it is a container. Use the standalone encoding fixture and BDD-0005-DETACHED-EXACT. |
| `VAR-0005-PART-MEMBER` | `U(1)` selects one standalone parent and exact member path. The admitted bounded parser verifies unique resolution, length and digest. Use the member encoding fixture and IT-0005-DETACHED-MIXUP under the applicable profile. |

The location variants use the existing protected bounds and trust
profiles. 0064 retains qualification ledger authority and 0065
retains signature, log and retry authority. 0030 and 0031 retain
their own native use and receipt contracts.

#### Existing signed record boundaries

| Stage | Exact caller bytes or equality |
| --- | --- |
| Package dependency proof before approval | Resolve 0003's packaged dependency evidence from the exact full-gate result. Match `H(C)`, `H(I)`, the packaging profile, target and component slice to the verified packaged bytes. Verify each required 0114 scanner result against its exact resolved component revision and the 0113 SBOM entry. Refuse missing, altered or mismatched proof even when the package digests otherwise match. |
| WordPress Plugin Check before approval | Resolve the fresh 0003 WordPress Plugin Check result in the same signed full release evidence. Match H(C), H(I), the WordPress ZIP digest and exact length, every required WordPress profile, check inventory and tool evidence. Refuse a source-only or earlier tier result even if the ZIP matches. A failed or missing cell stops approval and signing. Publish only the unchanged checked ZIP. |
| 0116 proof before approval | Resolve the report digest through the required build-verifier result in the exact release full-gate evidence. Verify the 0003 `evidence-bundle` for the protected release request, with the 0065 runner supervisor as signer. Match the report's `H(C)`, `H(I)`, packaging profile and all required targets. A missing, altered, stale, wrong-purpose or mismatched report or bundle blocks approval. |
| 0118 proof before approval | The governing signed protection result must bind `H(C)`, `H(I)`, the separate protection-inventory digest, exact covered part and profile evidence, fixture digests, observed outcomes and trusted runner identity. Compare `H(I)` explicitly. An exact signed result for another payload, mere coverage inventory or unverifiable envelope cannot pass. |
| 0005 approval | Keep the existing `candidate-release-approval` caller bytes and field order. Its unsigned payload manifest digest is exactly `H(I)`. Resolve the qualification and full-gate evidence digests through their protected records and verify that both bind `H(C)`. The signing preimage remains `T("release-approval") \|\| approval_caller_bytes`. |
| Release-manifest caller bytes `M` | `Rec("release-manifest", T(version) \|\| D(H(C)) \|\| B(C) \|\| B(I))`. |
| Typed release manifest digest | `H(M)`, over caller bytes only. The signing preimage is `T("release-manifest") \|\| M`. |
| Release-record caller bytes `R` | `Rec("release-record", T(version) \|\| D(H(C)) \|\| D(H(M)))`. The signing preimage is `T("release-record") \|\| R`. |

The 0065 verifier parses and re-encodes the nested records, computes
their exact digests and compares the inner and outer candidate,
inventory and version fields with the protected request and ledger.
Check exact inventory membership for every covered hash. Check the
manifest first. The later record must match its successful signature
entry's release, manifest and qualification ledger. The owner head
must cover `release-pending` and satisfy the 0065 GitHub-counter and
current-authority rules. Reconcile a lost reply by exact logged
identity. Reading an already recorded result grants no authority to
perform a pending action or to sign a release again with a revoked key.

The released signature envelope contains separately verified proof.
It is outside these caller preimages and does not hash itself.
Signatures, log fields, approval principal and key selectors only
have authority from their existing trusted roles and key registry.

#### Operational prerequisites and refusal

- The operational reader needs the exact 0064 and 0065 interfaces.
  Verify the source-tree, protected profile, signer registry,
  qualification and gate record digests through their authoritative
  readers. Refuse a missing trusted reader or identity.
- Recheck the Phase 7 entry prerequisites against the exact
  candidate before approval and again before signing, final tagging
  and publication. Use the protected candidate build and
  dependency evidence to match the Boot artefacts and profiles
  it actually consumes to the accepted 3B result in the current
  Phase 3 chain. A later candidate commit may differ from the
  earlier integration commit. A missing binding, superseded
  closure or changed Boot dependency without current matching
  proof holds the release. Claim each new protected effect under
  the exact current 0010 chain head and matching Phase 3 closure.
  Check that head in the step that authorises dispatch. If the phase
  head changes before that step, the claim cannot dispatch. An exact
  receipt can reconcile an effect already dispatched. It provides no
  current phase proof or authority for a later effect. The existing
  waiver route cannot supply omitted 3B parity or platform proof.
- Apply the separate Phase 6 closure prerequisite under
  [0076](0076-specification-completeness.md) and QR-243. Require an
  admitted protected reader and exact proof for the adopted feature
  and variant inventories. A matching Phase 3 chain and a passing
  0064 release qualification leave this prerequisite in force.
  Bind the exact accepted closure to the checked full-gate record
  used for approval. Recheck the same current closure in the step
  that authorises each new signing, tagging or publication effect.
  If it is withdrawn or replaced, hold that effect. A replacement
  needs an admitted full-gate record and approval for its own
  identity. Exact prior effect receipts remain reconcilable.
  An unavailable profile or missing proof holds this route.
- Verify the signed 0118 proof with both inventory digests before
  approval. At each subsequent signing, final tag or publication
  dispatch, resolve its exact protection result in the approved
  full-gate evidence and recheck the current protection inventory
  and required parts and profiles. Matching signed package bytes
  or an earlier signature or tag cannot supply that proof. A
  missing or substituted result holds the next effect. An exact
  prior effect remains eligible only for its own reconciliation.
  Publishing and installation require an admitted signature envelope
  and independent trust source. Refuse missing, stale, mismatched
  or ambiguous proof, envelopes or key identities.
- Each package profile must define exact unsigned files and
  members, supported and required targets, verified extraction and
  installer checks. Use the approved detached verification profile
  below for the Linux package and add-on ZIPs. Any signing transform
  that rewrites those files needs a separate owner decision. Verify
  the unsigned Windows wrapper with the independent pre-launch
  profile in spec 0030.
- Numerical byte, archive and inventory limits, allowed paths,
  release venues, target sets and packaging modes must come from
  their approved records. Before publication, compare the actual
  venue and target with those resolved for the approved release.
  Matching release text, version or artefact bytes cannot prove a
  different destination. A missing or mismatched venue or target
  holds dispatch. Verify the length and digest of the exact package
  input through dispatch against its frozen manifest row. A reopened
  file needs current proof. Reconcile a lost reply only against that
  original verified destination and effect. The admitted read-back
  must prove the actual published package against the same manifest
  row. Tag, release metadata and a local package receipt each prove
  only their own scope. Missing or ambiguous protected evidence
  blocks. Use the
  [synthetic byte fixtures](../planning/release-encoding-fixtures.md)
  to check encoding independently of operational trust.

#### Release authority identities

`DES-0005-RELEASE-AUTHORITY` joins
[signed evidence](#existing-signed-record-boundaries),
[operational prerequisites](#operational-prerequisites-and-refusal)
and the [approval receipt](#approval-receipt) for
TASK-0005-RELEASE-PUBLISH. Each proof producer keeps its own contract.

| Owning requirement | Exact clause and existing verification |
| --- | --- |
| `REQ-0005-RELEASE-EVIDENCE-ADMISSION` | Admit each exact dependency, WordPress and reproducibility result from the protected full-gate record. IT-0005-RELEASE-APPROVAL joins 0003 IT-0003-WP-PLUGIN-CHECK, 0113 UT-BI-19, the applicable 0114 scanner proof and 0116 IT-LS-06. The 0003 supervisor, exact result producer and required inventory scopes stay distinct. |
| `REQ-0005-PROTECTION-PROOF-RECHECK` | Resolve the current 0118 result for both payload and protection inventory at approval and each new effect. Use 0118 PL-0118 under REQ-0118-RELEASE and IT-0005-RELEASE-APPROVAL for missing or substituted current proof after an earlier signature or tag. |
| `REQ-0005-SCOPED-RELEASE-APPROVAL` | Require the owner-policy approver and exact task lease, candidate, qualification and full-gate evidence. IT-0005-RELEASE-APPROVAL separates a different enrolled signer, another run and expired approval with a current lease. QR-101 still governs exact time-source and expiry-boundary qualification. |
| `REQ-0005-RELEASE-PHASE-PREREQUISITES` | Match the current 0010 Phase 3 chain and the separate 0076 Phase 6 closure before each new effect. IT-0005-RELEASE-APPROVAL independently exercises changed head, missing closure and replacement; QR-243 owns the protected closure question. |
| `REQ-0005-APPROVED-RELEASE-EFFECT-BOUNDARY` | Resolve current trusted readers and approved limits, packaging mode, venue and target before the relevant effect. IT-0005-RELEASE-APPROVAL exercises valid parseable proof without its reader, candidate-supplied bounds, unapproved rewrites and a different venue. |
| `REQ-0005-RELEASE-EFFECT-RECONCILIATION` | Reconcile only exact earlier effects under their original records. IT-0005-RELEASE-APPROVAL exercises lost reply and wrong read-back. Signer repeat and count rules remain in 0065. |

#### Detached delivery profile

Use the existing canonical release-manifest signature envelope for the
Linux .deb and both XenForo and WordPress ZIPs. Keep the standalone
bytes identical from qualified unsigned inventory through publication
and exact installer or uploader input. The signature is separate.
Do not append, repackage or issue a second signing purpose. A signed
package means its exact bytes are bound by the verified manifest.

A protected packaging profile selects one mode per exact part key.
The admitted alternatives are detached Linux package, detached XenForo
ZIP, detached WordPress ZIP and the Windows first-trust mode.
Each row binds the existing role, target and standalone part key,
compatibility predicate, container and member rules, bounded parser,
consumer and handoff, trust and rollback policy, and bounded receipt
and error profiles. Use the digest already bound in C. The consumer
matches its own exact host and action against the admitted profile.
Filenames, extensions and envelope claims confer no authority. Exact
operational profile IDs, bounds and parser admission remain required.

#### Before release or package use

1. Obtain the qualified verifier, public trust roots, role registry
   and accepted policy through independently trusted means. First
   trust requires the admitted owner route. A prior trusted install
   can use its established trust and verified rotation path. No
   candidate media, package, sidecar, download origin or key shipped
   with it can establish its own trust. Consumers never receive a
   release private key. Follow the existing revocation, expiry and
   minimum accepted version or evidence rules. Hold offline use if
   its required freshness or trust cannot be verified.
2. Verify canonical M, C and I and the envelope for the existing
   release-manifest preimage under 0065. Verify R's signature and
   its version, H(C) and H(M). On Dell, publication also checks the
   protected ledger and final authority through their private
   readers. The installer checks the admitted public release proof
   for the same release. It reads no private log or ledger and
   holds if the qualifying public proof is missing or stale. The
   0118 proof binds H(C) and H(I) before approval. Publication and
   installation verify the exact bytes again at their boundaries.
3. Copy into a private, operation-owned stage with bounded reads.
   Reject links, paths outside scope, non-regular inputs and
   untrusted stage writers. Check the whole staged container's size
   and SHA-256 against its standalone row. Only then inspect its
   bounded member stream in the verifier, without writing members
   to disk or running native effects. Compare the admitted parser
   and native parser's exact member type, path and digest rules.
   Reject ambiguous names, duplicate or forbidden entries, path
   escapes, disagreement, structural faults and excess sizes.
4. Bind verification to C, M, target, stage identity, policy epoch,
   selected owner action and current authorisation. Recheck at
   handoff. Consume through the same retained object or a stage
   protected from changes and pathname substitution throughout use.
   Never reopen an ordinary user path based on an old verdict. If
   the native consumer cannot prove same bytes, stop. After restart
   re-establish file and policy binding. Reconcile a lost effect
   receipt before retry. Clear rejected and abandoned staging and
   retain only the safe scope-bound result or refusal.

#### Consumers

- Linux verifies from the independently admitted graphical route and
  delegates the same immutable .deb to the admitted native installer
  on Ubuntu 26.04 or Debian 13. Bind the exact architecture, native
  dependency profile and required control and data members. Preserve
  identity through privilege escalation to native consumption.
  Any dependency resolved by the installer must satisfy its pinned
  native dependency contract before unpack or scripts. Native
  package checks apply as well. A direct package open or a
  repository signing check alone does not supply this route's proof.
- For the two add-on ZIPs, verify on the trusted owner PC for the
  exact selected site and XenForo 2.3 or supported WordPress profile.
  The existing native signed-in administrator separately approves
  the exact action. The consuming native path must use the same
  verified stream or independently match the private staged upload's
  length and digest to the owner-verified release binding before
  extraction, registration or activation. Link the final native
  receipt to this exact upload and actor. An ordinary picker without
  immutable handoff proof and a host that cannot check the claim
  hold. Qualify each native flow on its supported hosting profile.
  An installer stub inside the incoming ZIP runs after extraction and
  cannot establish this proof. WordPress hooks that run after unpack
  and a stock XenForo archive upload give no pre-unpack proof by
  themselves. Verify each native action's existing authority
  independently of the byte check.
  [WordPress unpack order][wp-unpack] and
  [XenForo archive install][xf-install].
  No extra outside recipient handles the package or proof.
- Windows follows spec 0030. The independent graphical check must
  verify the exact unsigned wrapper against the trusted signed
  manifest before launch. Keep the unsigned wrapper warning. Verify
  payload and required native and WebView components before their
  first load or execution, including downloaded components. The same
  rule applies to installs, updates, repair and portable use.
  The unsigned wrapper cannot supply its own trust root or trusted
  verifier. QR-202 first-trust proof remains separate from QR-099
  protected build proof. The Linux and ZIP approval adds no new
  Windows signing mode or authority.

#### Native delivery boundary identities

`DES-0005-NATIVE-DELIVERY-BOUNDARIES` owns the release proof and
recipient boundaries in Consumers under DR-125 and ADR-0023.
0030 owns PC package use. 0031 owns the native ZIP effects.

| Owning requirement | Exact clause and planned evidence | Consumer and task |
| --- | --- | --- |
| `REQ-0005-ZIP-RECIPIENT` | The two add-on ZIPs and their proof use the approved owner-to-host route. An extra outside recipient is refused before disclosure or archive effects in `IT-0031-ZIP-EXACT-UPLOAD`. | 0031 `DES-0031-NATIVE-ZIP-HANDOFF` and `TASK-0031-SITE-FIXTURES` |
| `REQ-0005-WINDOWS-TRUST-SEPARATION` | The protected QR-099 build result cannot fill QR-202 native first trust. `IT-0030-WINDOWS-FIRST-TRUST` holds the affected native use when only the former is present. | 0030 `DES-0030-WINDOWS-FIRST-TRUST` and `TASK-0030-PLATFORM-INSTALL` or `TASK-0030-UPDATE-REPAIR` for the applicable action |

Use the exact variants in the consumer specs. The package evidence
and the separate native effect receipt retain their own scopes.

#### Platform acceptance design

| Case | Fixture | Expected evidence and result |
| --- | --- | --- |
| BDD-0005-DETACHED-EXACT | One qualified synthetic release per Linux host and site add-on profile | Record exact native input and match approved standalone and required member hashes. Detached envelope verification leaves all package bytes intact. |
| IT-0005-DETACHED-MIXUP | Correctly signed other release, architecture, target, role or platform ZIP; altered member; wrong purpose; extra archive entry | Refuse before native consumption. Record exact mismatch without private path or values. |
| IT-0005-DETACHED-TRUST | Bundle-only key or verifier, unavailable policy, revoked or stale proof, old valid manifest below policy floor, candidate-only bundle or wrong R | Refuse before effect. No checksum or cached verdict supplies missing authority, including offline. |
| IT-0030-VERIFIED-STAGE | Swap source or stage after verify, race a link across privilege handoff, interrupt and retry with stale proof. On each admitted Linux install, update or repair route that resolves dependencies, exercise the separate [native dependency cases][native-dependency-cases]. | Use only proven immutable bytes and the current admitted dependency resolution. Check the native package decision independently. Reconcile an uncertain earlier effect before any retry. |
| [IT-0031-ZIP-EXACT-UPLOAD](0031-shared-platform-installation.md#exact-native-zip-acceptance) | Substitute staged upload, swap site or native actor, withhold host proof, use an ambiguous ZIP, or select an additional outside recipient for the archive or proof | Refuse before archive effects and disclose nothing to the added recipient. A valid exact upload yields one native receipt for the authorised action. |
| IT-0030-WINDOWS-FIRST-TRUST | Modified wrapper, verifier trusted only by wrapper, changed payload or unverified WebView. In separate admitted Windows branches, replace the verified inert wrapper before launch or a verified required member before its first load. Keep the selected release B and its plan fixed. Offer complete genuine release A before B verification in one branch. In another, verify B then supply the genuine A wrapper and its proof at handoff. Reuse the [0030 Windows handoff branches][windows-native-cases] for bundled and declared downloaded components, each admitted lifecycle route and QR-099 proof offered without QR-202 proof. | Reject A against the unchanged B plan in both branches, without starting an installer. Bind each native handoff to the exact verified bytes. SUT-832 refuses substituted bytes before the affected launch or load. A wrapper refusal starts no installer. A member refusal grants no later effects or setup success from that step. Keep the unsigned wrapper warning. A verified build result supplies no native first-trust authority. |
| IT-0118-DETACHED-RECHECK | Alter published bytes or use 0118 proof belonging to another C or I | Block delivery or use at the exact proof or byte mismatch. |


**Missing public proof in IT-0005-DETACHED-TRUST.** Use an applicable
admitted consumer proof profile and exact approved package bytes.
Verify M and R under their own test purposes and independently
established trust. Keep every release and payload binding current.
Withhold only the admitted public release proof for that release.
In a separate branch offer an authentic, current fixture record
from the private Dell release ledger for the same release and
payload. Its signature and matching digests do not supply the
installer's proof. The installer holds both cases without reading
a private ledger or log, extracting a member or starting a native
action. An exact eligible public proof clears only the public proof
check. Require the actual QR-202 consumer profile evidence before
claiming that route. Keep Dell's private publication check separate.

**Native path safety in IT-0005-DETACHED-MIXUP.** Use the exact
admitted parser and target profile and its existing native
member-path resolution and collision rules. With a synthetic
stage root, use a forbidden path whose destination would escape
that root under those rules. In a separate case use two different
member paths that would each stay in the root but resolve to
the same native destination under the profile's own relation.
Resolve the exact [QR-202 path qualification][native-path-profile]
for this protected packaging profile. Use its qualified examples,
stage root, parser and native consumer identities and native path
relation. Keep each result scoped to its exact profile.

[native-path-profile]: ../registers/questions.md

Bind each case's complete outer bytes and length to its own
inventory and test-signed manifest and record. Keep other
applicable proof eligible. Require the path rule to reject the
escape or collision before writing any member or starting native
use. Refusal by a profile rule that already excludes a colliding
spelling satisfies the early path check. Observe that rule's
refusal explicitly. The control keeps all required paths within
the root at distinct destinations. Hold any profile with no
admitted relation or no applicable qualification proof. These
cases authorise no change to a profile or target.

**Exact member ambiguity in IT-0005-DETACHED-MIXUP.** Use the
synthetic duplicate-path ZIP from 0031's native upload case under
an applicable admitted ZIP parser profile. In this release-verifier
case, I has one required member row for that exact approved decoded
path and one standalone row for its exact parent. Put two regular
members at that path. One matches the member row's length and
digest. The other has the same length and a different digest.
Build two otherwise equivalent cases, with the matching member
first and last respectively. For each case, recompute the outer
ZIP length and digest, I, C, M and R. Use the existing separate
test-key purposes for M and R and bind other required fixture
proof to the exact new digests. Keep the target and authority
fixed. No inventory row is duplicated.

Require the manifest and parent length and digest checks to pass.
Reject the ambiguous mapping before extracting either member to
disk or handing the ZIP to a native consumer. Neither order may
yield a runnable or verified-for-use result. A separately bound
control with just the matching member passes this check. Record
only the existing safe refusal evidence. The 0031 case separately
proves the native upload, authority and no native effects.

#### Linux native package checks

In `IT-0030-VERIFIED-STAGE`, run the install, update and repair
branches separately under their exact admitted Ubuntu or Debian
profiles. Use the existing authority for each action. A repair of
one managed file without native dependency resolution cannot
qualify a repair route that resolves dependencies.

For each applicable resolver branch keep the signed release,
immutable .deb, verified stage and authorised action unchanged.
Offer an otherwise valid dependency outside the pinned contract.
In a separate case keep the visible plan identical to the earlier
eligible one and withhold independent current resolution proof.
In a third case verify the exact eligible resolution, then offer
unverified resolution output at native use. Each gives the
existing 0030 SUT-832 handoff refusal before unpack, dependency
installation, scripts or activation. The compatible control
continues only under the existing route and checks.

Independently offer proof of a direct package open alone and a
repository signature alone for the managed handoff. Hold each
without the independently verified graphical-to-native binding.
Use stage and action tripwires to verify no native effect. In a
separate synthetic branch keep complete package, resolution and
handoff proof, and give the admitted native package preflight a
refused result before unpack. Retain its own reason and hold the
operation with no first native write, unpack, script or activation.
An earlier verification cannot bypass the native decision.

For the resolver-backed update, pass native preflight and keep the
exact owner-approved plan, current lease, immutable package and
independently verified resolution fixed. Before native delegation,
withdraw only the admitted native permission for that update.
The operation holds with its own safe reason. No privileged package
effect, unpack, dependency install, script, activation or success
receipt occurs. Keep any exact prior checkpoint. A separate control
with current native permission proceeds only through the existing
update checks. Use a synthetic resolver and installer with no live
repository, package manager or native action.

[native-dependency-cases]: #linux-native-package-checks

These synthetic cases describe required evidence. An observed outcome
must bind the fixture, current protected profile, verifier, trust
record, candidate, selected exact bytes and native result. Add
tripwires for unpacked files, scripts, elevated actions and activation.
On a refused path each remains untouched. Exercise each admitted
platform, parser, hosting and transport profile independently.

[windows-native-cases]:
  0030-install-upgrade-removal.md#windows-first-trust-check

### Production review acceptance evidence

The protected release check matrix binds the criterion plan, including
its authoritative launch evidence, 0057 metric version and permitted
producer and verifier profile. Only that plan can establish pre-launch
inapplicability. Unknown launch state or an unadmitted source path
blocks the check. Preflight can admit a candidate attempt only.

Resolve launch from the protected release history for this product
lineage, under the current release lease. Its accepted initial
head identifies the repository and lineage. Its current head and
sequence must cover each release attempt and its exact disposition
since that initial head. An authenticated complete history can prove
pre-launch only if it has no verified final public release and no
unreconciled tag or publication effect. An empty query, restored old
head, incomplete read-back or unresolved publication attempt holds.

After the first final release, bind its release record, manifest
and the verified public read-back of the final tag, release and
applicable manual under step 7. Preserve that first proof in the
protected history. A retired release, replaced current asset or
new candidate cannot reset launch. Verify the accepted head and
the current release scope again at the next effect. A stale plan
cannot treat a now-launched lineage as pre-launch.

Use a one-off, owner-approved offline file for the 0057 projection.
Resolve its exact `candidate_digest` to `H(C)`, its run to the
protected release run and its rule to the admitted 0057 policy.
Before accepting it, the protected Dell receiver verifies the separately
admitted producer, record type and version, signature purpose and key
epoch, exact recipient and complete selection proof. Recheck current
authority and custody under 0065 and 0118. A route with no admitted
producer purpose or recipient remains unavailable. Treat an audit
checkpoint, owner-entered counts, source signature or delivery receipt
as insufficient release evidence.

Use this referenced result record in the full gate evidence already
bound by the approval receipt below. Use the canonical bytes above.

| Order | Exact fields |
| --- | --- |
| 1 | `T("production-review-acceptance-result")\|\|U(1)` |
| 2 | `T("production-review-acceptance")\|\|D(H(C))\|\|D(run_manifest_digest)\|\|D(protected_policy_digest)` |
| 3 | `D(criterion_plan_digest)\|\|T(outcome)` |
| 4 | `U(source_count)\|\|source_refs\|\|U(evaluation_count)\|\|evaluation_refs` |

Each reference is one `D` digest and each list has at most one item.
Use the exact outcomes of spec 0003. PASSED or a completed FAILED
comparison requires one admitted source and one owning 0057 evaluation.
BLOCKED has no evaluation and includes a source reference only if that
source is independently admissible. NOT_APPLICABLE has neither reference
and needs the exact protected pre-launch proof in its plan.

The existing Dell runner-result and full gate purposes authenticate
the protected check and verdict. Bind the exact criterion record by
digest in the required full gate evidence. Keep C and the approval
encoding unchanged. Missing, duplicate, stale or mismatched evidence
holds. Only current proof enters approval, signing, tagging or
publication. A delivered file grants no production-data, gate or
release authority. No automatic transfer is part of this route.

[field-acceptance]:
  0057-quality-judging.md#post-launch-review-acceptance

#### Canonical launch witness

Bind the SHA-256 digest of the witness in the protected criterion
plan. Encode these caller bytes with the canonical fields above.
Read the identities and profile from the protected release context.
Use the existing gate evidence and signer scope.

| Order | Canonical field bytes |
| --- | --- |
| 1 | `T("production-review-launch-witness")\|\|U(1)` |
| 2 | `T(repository_id)\|\|T(lineage_id)\|\|D(publication_profile_digest)` |
| 3 | `D(initial_head_digest)\|\|U(current_head_sequence)\|\|D(current_head_digest)` |
| 4 | `T(launch_state)\|\|U(first_final_count)\|\|first_final_refs` |

The one permitted `first_final_refs` item is
`D(H(R)) || D(H(M)) || D(public_read_back_digest)` for the first
verified final public release. Its final digest is SHA-256 of the
exact protected step 7 public read-back record. Use `PRE_LAUNCH`
with count zero or `LAUNCHED` with count one. In the second case,
the offline field source's `launch_record_digest` must equal the
first item's `H(R)`. Resolve and verify all references against the
same release and lineage. An unrecognised state, mismatched count,
incomplete or contradictory history, or unknown outcome supplies
no admissible witness. Hold with SUT-855.

A recheck may use a later authenticated head only when the verified
history extends the original head and preserves the same launch state
and first release identity, if present. Keep the original witness
digest in the frozen plan, gate result and approval.

For each final-tag or final-publication dispatch, compare the
checked head with the current protected lineage head and record the
exact pending effect in one protected step before dispatch. Bind the
effect to the lineage, `H(C)`, release run, action and exact target.
A competing release must recheck after the head changes and hold
while a prior public effect is unresolved. Reconcile a lost reply
only against the exact pending effect. A listing miss or elapsed
deadline alone does not clear it or permit another dispatch. The
matched terminal read-back closes the exact effect once. A verified
final-tag outcome closes only the tag effect. The admitted mirror
and manual steps keep their own authority and recovery checks.

#### Offline field source bytes

The local producer constructs one
`Rec("production-review-acceptance-source", fields)` from the exact
0057 projection. Append the fields in the order below. Apply the
protected profile's input limits before parsing. The canonical
source record is at most 2 KiB. The separate signing and sealed
file envelopes use their admitted profile limits.

| Order | Canonical field bytes |
| --- | --- |
| 1 | `D(policy_digest)\|\|D(cohort_commitment)\|\|D(launch_record_digest)\|\|D(candidate_digest)\|\|T(release_run_id)` |
| 2 | Reference window, then current window, as defined below |
| 3 | `D(reference_receipt_digest)\|\|T(recipient_key_id)\|\|U(issued_at)\|\|U(expires_at)` |

Each window is `U(decisions) || U(approvals)`, followed by the five
raw digests in order: deployed release, review profile, window
commitment, selection proof and current authority proof. Counts
and comparison use 0057. The source candidate digest is `H(C)`.
Text uses exact canonical protected IDs. Time fields are unsigned
whole UTC seconds, with issue strictly before expiry. Trusted UTC
time must be at or after issue and before expiry. Enforce the earliest
expiry of the producer proof, recipient admission, export approval
and protected run. Refuse trailing bytes, extra fields, wrong profile
or mismatched reference before accepting a window.

The owner handles only this sealed file for the exact approved Dell
recipient. Under 0020 and 0118 verify separate export approval bound
to the canonical source digest, recipient, run and criterion plan.
Verify producer and recipient against protected policy, independently
of the file. Revalidate source permission, erasure status, export
approval and recipient at intake, approval, signing, tagging and
publication.
Use separately authenticated current proof for the same cohort and
windows. An earlier authority digest or unexpired file supplies no
current proof by itself. If the stage lacks qualified revalidation,
hold. The named receiver stages the file for verification. The route
uses no live collector or product network connection. A missing
qualified signing profile, incomplete proof or owner refusal leaves
the criterion blocked. The existing runner signs its verified result, separately
from the source signer and from release approval.

### Approval receipt

Only the owner-signed policy can designate or revoke approval
principals. The protected release task assigns a lease to one of
them. No caller can self-grant that role. Coding, checking and
publication identities cannot hold it on that release.

Tijori prepends the text purpose `release-approval`. Append the
following fields in order. Use spec 0064's binary length and text
encoding, with its raw 32-byte digests. Other integers are unsigned
64-bit big-endian. IDs and tags use the exact canonical identifier
from the protected service. Times are UTC seconds. [QR-101][approval-time]
requires the admitted time source and exact boundary rules for
checking current approval authority. Hold a new effect if that
authority cannot be established. 0065's release-key clock and log
confirmation checks remain separately required.

[approval-time]: ../registers/questions.md

| Order | Field and type |
|---|---|
| 1 | Record type text `candidate-release-approval`, format version integer 1 |
| 2 | Repository ID text, task number integer, lease ID text, run ID text |
| 3 | Commit object ID text, candidate tag text, version text |
| 4 | Qualification, full gate evidence, unsigned payload manifest and protected policy digests, in that order |
| 5 | Approver principal text, signed time integer, expiry integer |

Bind the purpose and exact fields in the signed record. Refuse
wrong purpose, extra bytes, unrecognised versions, noncanonical
IDs, expired authority or candidate changes. The verifier obtains
the required outcomes itself.

Recheck the current lease, role and evidence before signing,
tagging and publishing. Reconcile a lost reply for an already
applied action by exact receipt. An unperformed action requires
current authority. The approval key cannot sign artefacts or tags.

Fixed nightly runs keep reporting reconstruction recall, anchor agreement,
hard rules and coverage without AI judges. They give no release authority.

## Limits

- Release-tier memory, load and soak tests run on test machines that
  match both sizes of owner's PC, within the memory limits in AGENTS.md
  section 5.6.
- The soak lasts 72 hours.
- Dell heavy jobs, including dependency group runs, follow AGENTS.md
  section 11.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Rust integration tests on the coordination and release crates, with
scratch Forgejo repositories:

- Two agents ask for the same file: only one gets a valid lease.
- A stopped agent returns after handover: its stale lease cannot write.
- A worker asks for a `main` update outside the door, a tag, a release
  or a free endpoint: it is refused, and no credential is shown.
- IT-0005-RELEASE-APPROVAL: accept a matching scoped agent or owner
  receipt after all gates pass. Refuse a coding identity, forged
  evidence, wrong purpose, noncanonical fields, stale lease,
  incomplete long lane or missing encryption proof. A changed
  package gets no final tag. Replay only reconciles an exact
  existing receipt. Revocation blocks any unperformed effect.
  Keep the exact candidate, receipt bytes, current policy and lease
  for one enrolled approver. Sign the unchanged bytes with a second
  enrolled principal's fixture key for the release-approval purpose.
  The fixture signature verifies for the second principal, whose
  own scoped approval is valid in its separate control. Refuse the
  substitution because the signature, named approver and assigned
  lease must describe one principal. Send no release-key request
  and create no final tag or publication.
  In a separate case commit M, R and the final tag while the named
  approver is enrolled and the exact approval is current. Before
  package dispatch the current owner-signed policy revokes that
  principal's approval role. Keep the original signed approval,
  lease record, candidate, package and prior results fixed. The
  decision time is still strictly within the receipt's interval.
  Verify the approval in its original policy scope. Neither the
  historical signature nor the unchanged lease grants current
  authority. Hold publication. Reconcile only exact prior results
  under current read authority, with no new signature, count, tag
  or dispatch. The control has the named principal still enrolled
  and exact matching current approval and evidence. Keep the
  release key and any field-source authority fixed in both cases.
  In the expiry case keep the exact current lease, role, candidate
  and evidence. Use a fixture-signed approval with expiry strictly
  before the trusted decision time. Refuse before the first new
  release-key request. Repeat after an otherwise authorised M
  signature has committed and before R is signed. Reconcile only the
  exact original result through its owning read-back. The expired
  approval grants no new signature, count, final tag or publication.
  A separate control uses the same current authority with time
  strictly inside the approval interval and passes this check.
  Resolve the clock and equality rules through QR-101.
  In a separate publication case commit M, R and the final tag
  under current approval. With the same lease, role, candidate and
  package bytes, pass the approval's expiry before dispatch.
  Reconcile the retained tag result only. Send no package.
  Independently dispatch while approval is current and lose its
  reply. After expiry, reconcile the exact pending publication
  through its original destination and effect read-back under
  current read authority. Record one completed historical effect.
  A missing or mismatched read-back leaves it unresolved. Neither
  branch grants a new signature, tag, publication or later effect.
  Exercise three separate prerequisite refusals with all unrelated
  checks passing. First, keep the exact candidate and a canonical
  qualification record whose fixture signature verifies. Withhold
  the authorised 0064 qualification reader. A generic parse and
  the fixture signature give no substitute for the missing reader.
  Hold approval, signing, final tagging and publication. Restore
  the exact admitted reader and current proof as the control.
  Next, keep an eligible WordPress ZIP, its exact inventory,
  the approved detached profile and an otherwise valid approval.
  Request a signing transform that rewrites the ZIP without a
  separate owner decision. Refuse before the transform or any
  release-key call. Preserve the frozen ZIP bytes and create no
  final tag or publication. The same package on its admitted
  detached route is the control.
  Finally, leave the approved-record reader available and withhold
  the required protected part-count limit. Keep a canonical
  inventory within the baseline fixture's limit and supply that
  same numerical limit as a candidate hint. Refuse before any
  allocation or recursion governed by the missing limit. Approval,
  signing, final tagging and publication stay held. Supply the
  limit through its approved record as the control. Each control
  clears only the prerequisite exercised by its case.
  Reuse 0116 IT-LS-06 with all other checks passing. Supply the
  exact current 0003 `evidence-bundle` and report, then independently
  omit or alter the report, substitute an otherwise valid test bundle
  for another `H(I)` or target, and offer a valid test-signed
  `release-manifest` in the evidence-bundle slot. Check the other
  records in their own roles. Each fault leaves approval, artefact
  signing, final tagging and publication blocked for this candidate.
  Join IT-0065-EXACT-RELEASE-BINDING for a canonical M whose outer
  version disagrees with C and for the same-byte manifest replay
  carrying a replacement ledger. A matched caller-byte digest cannot
  cure either fault. In the replay case preserve the original
  signing result and its original ledger provenance. Both faults
  leave any new signing, final tagging and publication blocked.
  Reuse 0113 UT-BI-19 with all unrelated release proof passing.
  Keep the correct package and inventory digests for each offered
  fixture. Test a missing or wrong count and a well-formed count
  with a valid scanner result for another resolved revision. Offer
  a matching observed count and result as the control. Only the
  control passes this proof check. Each fault blocks approval,
  artefact signing, final tagging and publication.
  In two separate cases, keep the candidate's own long lane
  incomplete. Offer in the full-gate evidence slot a valid
  test-signed Phase 2 `phase-gate-run` with its own tier proofs,
  then a passing test-signed nightly campaign on the same
  candidate commit. Verify both within their owning scopes.
  Refuse each as release evidence against the protected release
  task, run, candidate and profile. Preserve the accepted phase
  chain and nightly outcome for their existing uses. Both
  substitutions leave approval, artefact signing and the final
  tag blocked. In a separate pre-approval fixture, withhold one
  required fresh release check. Verify a correctly test-signed
  `cache-entry` for the same candidate, tool pins, fixtures,
  environment, policy and other comparable inputs in its own cache
  scope through its admitted reader. Offer it in the missing
  execution role. Refuse it before approval or any release-key
  call. In the recovery control the owning runner records the
  exact authenticated fresh result for its run, attempt and cell
  durably before the reply is lost. Reopen the same authorised
  release run. Resolve that result and ordered attempts by digest.
  Recheck tier, candidate, case, target, policy and input profile.
  Recover the one result without another execution. In separate
  cases lose the reply before the result is durable or present a
  valid fresh receipt from another release run with the same inputs.
  Each leaves the cell missing. Keep approval held when any required
  evidence remains missing. Use the owning 0003 and 0064 readers.
  A generic trust or replay result cannot supply execution. This
  fixture uses test evidence and makes no release-key request, tag
  or publication.
  Reuse PL-0118 in a separate late-boundary case. Keep the approved
  candidate and payload hashes, exact signed manifest and existing
  approval. Lose the reply for the committed signature. Before
  final tagging, withhold the 0118 protection result named by the
  approved full-gate record. Repeat with a valid test-signed result
  for the same candidate and unsigned payload but a different
  protection inventory. In a separate branch, keep the committed
  final tag and substitute the result before publication.
  Each holds the next effect. Reconcile any exact prior signature
  or tag to its original receipt. Emit no additional signature,
  final tag or publication on refusal. With the correct current
  result and authority, verify that the control admits only its
  own unperformed effect. Use synthetic records and no public effect.
  Keep the exact candidate and consumed Boot bytes in
  three separate cases. Accept a replacement 3B through 0010 after
  release approval and before a signing request, after a committed
  signature with a lost reply and before the final tag, or after a
  committed final tag with a lost reply and before publication.
  The former 3C turns historical. Refuse the next effect and leave any
  earlier committed result intact. Recover each exact lost reply
  once from its receipt. Create no second signature or tag on
  recovery and no release or promotion on the stale chain. Also
  change the phase head between a successful read and dispatch
  claim. The claim holds. Fresh matching 3C evidence clears only
  the phase prerequisite. Under the post-launch criterion, supply all other
  passing results and separately omit its receipt, supply owner
  counts or a delivery receipt in its place, and change its candidate,
  run or admitted recipient. Each keeps approval, signing and the
  final tag blocked. Match the 0057 IT-0057-FIELD-CUSTODY proof only
  under the current exact protected check.
- A protected change stays blocked until it carries the owner's
  signature.
- A cached availability result is stale, or Dell disappears: new work
  and admission pause, and the true state is reported.
- The release step refuses to start while scope is short, and says why.
- The version commit passes the same tiers as any commit.
- Only the release step can make candidate and final tags; an agent's
  tag push is refused.
- The final tag names the same commit as the passing candidate; a
  changed commit needs a new candidate.
- A candidate that lacks scope or fails judging or the soak gets no
  final tag, product release or manual promotion.
- The release step crashes after making an artefact: recovery finishes
  once and makes no duplicate.
- In the publication branch of IT-0005-RELEASE-APPROVAL, hold the
  exact candidate, package bytes, signed manifest, current scoped
  approval and final tag fixed. Offer a different synthetic target
  with the same visible repository name and release version before
  publication. Separately withhold the approved venue proof. Each
  holds with no outbound dispatch. Keep the original signing and
  tag receipts. In the control bind the exact approved venue and
  target, then lose the synthetic publication reply. An authentic
  read-back from the other target cannot reconcile the original
  effect. Keep that effect unresolved with no second dispatch.
  Match the original destination and effect read-back to reconcile
  it once. Use no live venue, credentials or release keys.
- In a separate package publication branch, keep the approved
  candidate, manifest, final tag, pending effect and target fixed.
  Verify the package, then substitute a same-length file at the final
  publisher handoff. A name and earlier local verification give no
  proof of these bytes. Hold before sending the package. Separately
  dispatch the exact approved package to a synthetic admitted target
  and lose its reply. Read back the correct tag and release metadata
  there, but expose a different same-length package with the same
  name. The original effect stays unresolved. Keep its record and
  earlier valid receipts, with no verified public result or second
  dispatch. The exact admitted read-back of the original package
  reconciles that effect once. Count asset effects separately from
  tag or metadata results. Use disposable local fixtures only.
- A dependency group that fails a check is not pushed, and its findings
  are filed in autoissues, with no Forgejo task until an agent takes
  one on.
- No release or dependency step makes a branch or a pull request.

### Field evidence transitions in IT-0005-RELEASE-APPROVAL

Use synthetic protected release tasks and test keys. Extend the publishing
and gating model in spec 0010 with two exact release attempts, an admitted
field source and an independently trusted Dell runner. Make other required
checks pass in these traces. Track whether the existing release step requests
a new signature, creates a final tag or publishes. Bind each transition to
the current protected release context.

- **Verified pre-launch.** The release verifier proves pre-launch from
  the authoritative state named by its protected criterion plan. Resolve
  the exact plan and the runner's current scope. A signed
  `NOT_APPLICABLE` record has no source or evaluation references. The
  required full gate evidence binds the record by digest. With all
  remaining requirements met, the exact approval can proceed.
- **Unknown launch state.** Lose the authoritative read or give it an
  incomplete result in separate traces. Offer a valid signature over a
  candidate's claimed pre-launch state, an otherwise valid result from
  another run, or a staged release record. None proves pre-launch or
  admits a release effect. A miss in the feature qualification check
  matrix supplies no evidence about whether the product has launched.
- **Verified post-launch.** A first final public release is verified.
  On the later attempt, replay pre-launch proof from the earlier
  attempt and separately omit the field result. Both hold. Then give
  the later attempt the exact current, admitted source and one owning
  0057 evaluation with 200 eligible decisions per window and equal,
  positive approval counts. One verified `PASSED` record advances the
  field criterion. A signature scoped to another check or execution
  target, an approval signature or a Phase 2 run cannot substitute.
- **Revoke before signing.** Accept the later candidate's exact release
  approval. Before its first release-key request, revoke the current
  source right or export approval, in separate traces. Hold at the
  following boundary. Produce no new signature, final tag or release.
- **Revoke after signing.** Repeat with the exact signature already
  recorded and revoke the right before creating the final tag. Keep the
  recorded signature and block the tag and publication. In a separate
  trace, revoke after a verified tag and before publication. Keep the
  exact tag and block the pending publication. An exact read-back
  reconciles a completed effect and authorises no further effect.
- **Current proof for another window.** Keep the exact approved
  package, signatures, final tag and current release approval. At
  publication keep the sealed field source, frozen reference and
  cohort unchanged. Withhold revalidation for its current window
  and offer authentic current rights and erasure proof for a
  different window in that cohort with equal aggregate counts.
  Hold with SUT-855 before dispatch. Preserve prior receipts and
  do not replace the frozen evidence. The control uses the exact
  current proof for both original windows through the admitted
  reader. Use synthetic aggregates and no production records.
- **Witness encoding and binding.** Verify both canonical states.
  Separately change the repository, lineage, profile, initial head,
  observed head, state, reference count or one of the first final
  digests. Substitute a verified public read-back for another
  manifest, or a source with a different launch record digest. Each
  refuses with SUT-855 before a new release effect.
- **Competing public effects.** Give two release attempts the same
  checked head. Race their final-tag dispatches and, in a separate
  trace, their publication dispatches. Exactly one protected pending
  effect is recorded and may dispatch. Lose its reply and restart.
  The unchanged competitor and the original retry produce no second
  dispatch. The exact read-back closes the effect once. A tag-only
  read-back supplies no launch proof. A verified final public
  read-back requires field proof for the next release.
- **Current proof proceeds.** In the matching control trace, prove
  current authority separately before each remaining step. Continue the
  exact release once. Replaying a completed step preserves its original
  receipt. In a separate trace keep the field result valid and fail
  one required synthetic check. Hold before the next release effect.

For each trace record the candidate, run, policy, criterion plan, allowed
signer purpose and scope, exact result and full-gate evidence digests and
the next blocked or completed effect. Keep launch and authority checks
as model inputs with their own real integration fixtures. The model
checks how the release step uses them.

## Changes from the source

- GitHub App identities and a publisher broker are replaced by Forgejo,
  its door and the sync; agents push only through the door.
- Pull requests, squash merges, auto-merge and fork checks are gone;
  outside work follows AGENTS.md section 9.2.
- DR-025 admits a separate release approval. Owner-signed waivers
  and the protected artefact signer retain their own authority.
- The GitHub AI review is replaced by the required reviews.
- Releases are made only when the roadmap names one or the owner asks,
  not weekly.
- The release bar is the one in AGENTS.md section 3, in place of the
  older 6,000-method bar.
- Version and changelog tools are replaced by the release step working
  from commit messages.
- Dependency bots are replaced by the weekly Dell task; the old audit
  tool for another language is dropped.
- The speed check is the 5% median rule of AGENTS.md section 6, not the
  old 20x gate.
- High-severity security findings are filed in autoissues as P0 or P1,
  with a Forgejo task only when an agent takes one on.

Source: `docs/specifications/agent-release-coordination.md` and
`docs/specifications/release-main-only.md` in the archive
`archive-2026-11-08-old-repo`.

[wp-unpack]: https://developer.wordpress.org/reference/classes/wp_upgrader/run/
[xf-install]: https://docs.xenforo.com/manual/configuration/add-ons
