# 0064: Version 1 scope and qualification

Version 1 includes the four feature ranges and the expanded scope in
this specification. Phase 6 builds the features. Phase 7 qualifies the
release against [AGENTS.md section
3](../../AGENTS.md#3-the-goal-and-the-release-bar). The method catalogue
in spec 0055 and release assessment in spec 0057 share one count
verifier, the `qualify` crate, which this specification owns. It lives
in `tools/qualify`, runs only on Dell and is never shipped. It depends
on `sutradhar-methods` (spec 0055) and the signature checks of tijori,
and nothing depends on it. It owns the verified ledger and publishes it
as a file signed under its own ledger signing purpose. The ledger holds
the six qualified counts (the method, capability and variant totals, the
method and capability in-scope totals, and the ranking subtotal) and no
member rows. It also carries the run id the protected policy issued, the
run kind, which is one of release, pilot or fixture, the run purpose,
the run manifest digest, the candidate, catalogue and current protected
policy digests, the head of the Dell run ledger it was built from, and a
sequence number for that candidate and run kind that only goes up.

The ledger type, its encoding and its one reader check live in
`sutradhar-methods`, using tijori's signature checks; `qualify` writes
with that code, and `sutra methods count` (spec 0055) and release
criterion 7 (spec 0057) read with it. They take the qualified counts
only from the ledger and never count for themselves. The reader check
reads the ledger once into memory under a size limit and takes the
counts from the bytes it checked. It reads the signer registry and the
Dell run ledger's state through the `QualifyTrust` trait of spec 0055,
and refuses when any answer `QualifyTrust` gives cannot be read. It
checks the signature and purpose against that signer registry, and
refuses any ledger whose run kind is not release (a pilot or fixture
ledger, or any other kind), a ledger whose candidate, catalogue or
policy digest differs from what it judges, a ledger that is not the
latest for its candidate and run kind, a ledger older than any attempt
or refused run of the same candidate and run kind recorded since in the
Dell run ledger (not counting passing attempts of gate, nightly or soak
runs, as the policy records them), and a ledger older than any test
failure on the same candidate in a release or pilot run, recorded since.
It takes the current protected policy digest from `QualifyTrust` too.
When it refuses, `sutra methods count` shows the qualified counts as
unverified rather than printing them. `sutra methods count`, which
judges no release, compares against the frozen candidate the ledger
names and the catalogue it reads.

The reader check uses one shared ledger validator for the bounded-byte,
signature, binding and freshness checks above. The shared validator
checks the run kind and ledger signing purpose against a trusted run
context. For replay, the caller names the run only by the run id signed
in the previous ledger. `qualify` looks that id up in the record the
protected policy issued for it ("Evidence records and groups"), which
fixes the run's kind and run purpose, and in the approved run inputs
that "Freeze the run" loads, before any submitted evidence is opened.
Replay refuses if no trusted record matches, or if the record's run id,
kind or purpose differs from the ledger's. Protected policy fixes the
expected ledger signing purpose for that kind. A run purpose is distinct
from a signing purpose. Neither submitted data nor a caller option may
change this context. The reader check keeps its fixed release-only
restriction for both of its callers, with no option to admit a pilot or
fixture ledger. The reader check and verifier replay both use the same
checked bytes and trust checks; they never reopen an unchecked copy to
obtain counts.

## What the product covers

| Area | Work and owning designs |
|---|---|
| On-page and technical search | Specs 0044, 0045, 0047 and 0048 own crawling, page and site audits, speed diagnosis, remedies and evidence that a remedy helped. Leviathan names the shared crawler and its block-page handling in specs 0044 and 0045. The feature catalogue owns enterprise search workflows across content, visibility, crawl, links, competitors, experiments, local presence and reporting. |
| Editorial coaching | Specs 0041 and 0042 own the SEO Editorial Coach and Narada checks. Factledger is the contradiction tip in spec 0042: it compares passages and stores no list of facts. Each suggestion binds the exact passage, page version, evidence and moderator decision. Shared text checks keep their owning computation and identity. |
| Text and meaning | Specs 0024 and 0026 own text mining, linguistic analysis and embedding quality. The clean-room language pipeline covers segmentation, morphology, parts of speech, syntax, entities, entity linking, coreference, sentiment and relations. Test every public operation and the quality of the annotations used downstream. |
| Knowledge and graph analysis | Spec 0049 owns entity and site graphs. ganit owns the pure-Rust graph algorithms, including centrality, community detection, graph structure and link prediction. Use NetworkX public graph operations as clean-room research references for the Rust implementations. Pixie-style bounded random walks enter only as candidates that the method lab admits (spec 0056), compared with named baselines on held-out sites. |
| Link ranking and protection | Specs 0022, 0027, 0053, 0055 and 0056 own full-body analysis of source and destination, learning to rank, blending, calibration, reranking and bounded autotuning. Choosing destination threads or pages for a phrase or passage is matching and link ranking (specs 0023 and 0027), the one recommendation engine; no other spec owns one (spec 0027, "Words used here"). RankMedulla, the ranking model that `shodhan.rank` serves (spec 0027), and SpamGuard, the spam model of kavach, own the ranking and spam evidence. Keep a separately qualified ranking-method subtotal. |
| First-party and enterprise analytics | Specs 0034 and 0040 own events, attribution, segmentation, funnels, cohorts, journeys, retention and source connectors. Join analysis to the page or content version it describes. Provenance, sample size and uncertainty travel with any ranking signal. |
| Research and bounded optimisation | Specs 0012, 0013, 0055 and 0056 own King Solomon research cards, evaluator qualification, research memory, admitted methods and controlled optimisation. Qualify shipped public operations separately from the evidence they produce. |
| Agent operations | Spec 0014 owns the settings registry and its discovery; spec 0020 owns scoped MCP, the `sutra` command, change requests, permission checks and live confirmation. Every action and setting has an owner, reference documentation and machine-readable discovery. |

The enterprise research inventory names Adobe Analytics, Mixpanel,
Semrush Enterprise, Conductor, Piano, Optimizely, Botify, Ahrefs,
Lumar and BrightLocal. Research records link to public descriptions,
map each agreed workflow to one of our designs and state how we test
it. Public papers and documentation give research references for
[graph walks](https://arxiv.org/abs/1711.07601),
[graph analysis](https://networkx.org/documentation/stable/reference/)
and [linguistic analysis](https://stanfordnlp.github.io/CoreNLP/).
Use the clean-room rules in AGENTS.md section 3 for all implementations.

Read both pages' full guest-visible bodies when qualifying link
relevance. Preserve the relevance gate in spec 0027 before any
reordering. Record a typed signal with provenance, freshness and
confidence for every feature that learns about a page. Apply all site
changes through a moderator, with undo and a measured footprint.

## Names used for these areas

Older and working names point to one owning design. A name never
creates a second home.

| Name | Owning design |
|---|---|
| Leviathan | The shared crawler and its block-page handling (specs 0044 and 0045) |
| Narada | The Coach's editorial checks (spec 0042) |
| Factledger | The contradiction tip (spec 0042); no stored fact list |
| King Solomon | Research cards and independent checking (spec 0013) |
| King David | Research memory and recall (spec 0013) |
| Shraddar, Shraddha | The research memory store (spec 0013) |
| MachineMirror | The test room (spec 0039) |
| Subramanian | The web link study (spec 0046) |
| QuarryView | Search charts (spec 0043) |
| RankMedulla | The ranking model that `shodhan.rank` serves (spec 0027) |
| SpamGuard | kavach's spam model, used as a ranking signal (spec 0027) |
| Pixie | A graph-walk method candidate for the lab (specs 0049 and 0056) |
| NetworkX | A public reference for graph operations; no code is copied |

## One home for each behaviour

The feature register maps each number, alternate name, action, surface
and platform to one owning design and one build task. A retired
number remains an alias and points to its owning design. A release
refuses orphaned features, conflicting rules and separate entries
that implement the same behaviour. A shared implementation can serve
many features through its one declared contract. Desktop, CLI, MCP
and platform wrappers for one behaviour share its identity.

A method is counted once by its computation and behaviour. A capability
is counted once by its independently usable, shipped operation. A
variant is counted once by its canonical owning behaviour and its
admitted input class, as spec 0071 sets out. An input class is
admitted in the frozen catalogue with an independent review. It is
distinct from a neighbouring class only when its test fails with the
neighbour's input in its place,
because the input takes a different decision or code path. Classes that
differ only by value, such as adjacent bands of one number, are one
class. Values within one class count once. Check identities record
evidence. Every required check must pass for that variant. Weights,
seeds, names, reordered steps and configuration choices earn no extra
method, capability or variant by themselves; a chain of methods counts
only under spec 0067's rules, and a workflow of shipped actions only
under spec 0070's. A learned weighting model
can count as a ranking method when it meets the distinctness and
admission checks in spec 0055. Treat the ranking subtotal as a subset of
the qualified methods.

Each method and capability has one primary scope classification.
The target share in AGENTS.md section 3 uses the on-page and technical
search, editorial coaching, text and meaning, knowledge and graph,
link ranking and protection, first-party and enterprise analytics,
research and bounded optimisation, and agent operations areas above.
Use the area that owns the computation. Agent operations covers MCP
discovery, scoped agent access and agent requests. An underlying
installation or backup action keeps its own scope when exposed
through MCP. Count a member once, even if its outputs feed several
areas or pipelines. Research cards and run records provide evidence;
only independently usable shipped operations earn capability credit.
A capability workflow earns one capability only as spec 0070 sets out:
each of its actions is shipped with its own evidence, it ran end to end
on each target of its shape, and it behaves differently from every
other counted capability. Presets, playbooks, templates and wrappers earn none.

Implementation quality follows the code standards and required
reviews in AGENTS.md sections 6 and 7. Review concise, readable,
maintainable and extensible code through its module boundaries,
public APIs and the tests of existing callers. Release credit follows
qualified behaviour and its signed evidence. Code volume earns no
release credit.

## How evidence is counted

For each release, freeze the candidate, catalogue, scope classification,
test protocol and approved execution targets. The protected policy pins
the catalogue and scope register by digest. Each member's primary scope
and ranking flag carries its own signature by a reviewer who is not one
of its authors, or the owner's approval of that row, and the verifier
checks it row by row. A method chain takes its shape's row, which
covers each of its chains (spec 0067), and a capability workflow takes
its workflow shape's row, which covers each of its workflows (spec
0070). The trusted verifier
reads signed groups with disjoint member identities. Each group
records the proof that its admitted members actually passed, with
their membership and evidence tied to the frozen inputs. A group
certificate can attest pointwise checks with a replayable proof.
The verifier counts the proven members. An interval, sample or
calculation of all possible combinations supplies no pass count.

The verifier checks exact-candidate binding, independent method
qualification, shipped capability registration, variant checks and group
disjointness before adding counts. It checks duplicate computations and
behaviours across groups. A group with missing, stale, forged,
overlapping or uncheckable evidence blocks the release. Store the six
qualified counts as unsigned 128-bit integers, and parse counts into
that type. An empty ledger fails each share. Machine-readable JSON, MCP
and CLI results carry counts as canonical unsigned decimal strings.
Accept `0` or digits beginning with a non-zero digit, with no sign,
fraction, exponent or leading zero. Parse into that type directly; never
pass a count through floating-point arithmetic. Screens may abbreviate a
count, but its exact value remains available from the same stored
result. Use checked unsigned 128-bit arithmetic for accumulations, ratio
cross-products and capacity calculations. A bad value or overflow blocks
release and names the evidence group.

The release report keeps the counts separate and links to the
signed evidence and owning designs. Integration and end-to-end tests
follow AGENTS.md section 7. The full release checks ranking quality,
resources, recovery and site footprint on both test sites.

## Qualification pilot protocol

### Freeze the run

The pilot uses the shared verifier. The run manifest names
the run kind, exact commit, source tree, dependency locks, catalogue
and scope register. It names the protocol, each execution target,
the toolchain, fixture inputs, the gate policy and the trusted signer
set. Use the digests of the exact bytes for these bindings. Target
identity includes the runtime, resource profile and test site version.
A pilot has a separate signing purpose and cannot add release credit.

The qualified verifier starts from the protected release policy on
Dell (spec 0005). The owner authorises its trust root outside the
candidate, the run and the supplied evidence. From that policy, the
verifier loads the current authorised signer registry and the approved
run inputs, including the exact candidate, required check and target
matrix, and trusted protocol. It compares their digests to the run
manifest before opening any evidence. A submitted manifest or group
cannot change the trust root, select an older policy or grant access.
Refuse unknown, revoked or out-of-scope keys and policy rollback.
The protected policy pins the verifier, the runner supervisor and the
check harness by digest. Load them outside the candidate, and give the
sealed candidate job no access to replace them or use signing keys.
The supervisor observes the approved invocation, exit and evidence
through the trusted harness. The signing service accepts the result
from that supervisor only. A pass printed or written by the candidate
cannot serve as a result. Keep private answers and the scoring
oracle in the trusted harness, as set out in spec 0056. Candidate
code receives only bounded exercise inputs in its separate seal.
The verifier, and the second verifier in "Measure the full route",
each check the runner and harness bindings.

Record the input digest before the run. Use the qualified runner and
signing service in spec 0005. A later candidate or target needs its
own result. Evidence reused under spec 0055 supports admission only;
every counted member needs a runner-signed result bound to this run and
this exact candidate. Commit and push gates apply
before admission to the pilot. A release needs the same verifier on
the frozen version candidate and all other release checks, including
the resource profiles and soak. Tagging follows spec 0005.

### Evidence records and groups

A canonical member identity is ASCII only: `M-`, `CAP-` or `V-`, then a
decimal ordinal, or `MC-`, a shape ordinal, `-` and a chain index for a
method chain (spec 0067), or `CW-`, a workflow shape ordinal, `-` and a
workflow index for a capability workflow (spec 0070), each number from
1 upwards that fits an
unsigned 128-bit integer, written with exactly six digits with leading
zeros below 1,000,000 and no leading zero from there up; ordinal 0 is
not used, and a longer ordinal is refused as a bad value. The fixture's
own ids add `FX-` in front, and a release run refuses any `FX-` id. The
verifier refuses any other byte form, such as a padded ordinal, another
case or a look-alike hyphen, before it compares anything. A shape's
scope-register row is keyed by `S-` and the shape ordinal, written the
same way; `S-` is not a member identity and never counts. An `S-` row
carries the same signature as a member's row. The verifier finds an
`MC-` member's scope row by its shape ordinal. A workflow shape's row
is keyed by `W-` and the shape ordinal, written the same way; `W-` is
not a member identity and never counts, and it carries the same
signature as a member's row. The verifier finds a `CW-` member's scope
row by its shape ordinal.

Each row has a ledger kind, canonical member identity, exact version,
owning spec and task, primary scope and applicable target. Bind the
run manifest digest, check identity, result and evidence digest to the
row. Accept the primary scope and ranking subtotal flag only from
the frozen catalogue. The signer registry maps keys to principals,
allowed purposes and scopes. Get all authors from the authenticated
admission and source provenance for the exact version. A reviewer
must differ from every author by stable principal, even when one
principal has several keys. A display name or an author field supplied
with the result cannot establish identity.

For each required result, verify the qualified runner's signature over
its run, member, check, target, outcome and evidence digest. The runner
key must be authorised for that purpose, check and target. Verify the
result against the check and target matrix from the protected run
policy. A group signature authenticates the collection and cannot
replace the runner's authority for each result. In a run that compares
capability workflows, every capability result whose action writes a
catalogued artefact kind also carries its last artefact's reference,
output digest and key digest (spec 0070); such a result without them
is refused.

The protected policy issues each run a unique run id before it executes,
and fixes the run's kind and its purpose (qualification, gate, nightly
or soak) with it; the verifier refuses a manifest whose run kind or
purpose differs from the policy's, and ignores any label in the
manifest. Gate, nightly and soak runs of a release candidate are release
runs whose test failures count. Their passing attempts do not make a
release ledger stale, and the replay's history check skips them; a
refused or interrupted run always counts, whatever its purpose. Read the
complete attempt history from the protected Dell run ledger (spec 0005).
For each run, member, check and target, require a single final verdict
bound to its ordered attempts and the frozen retry policy. Only that
policy, and only the trusted supervisor's record, can class an attempt
as an infrastructure interruption; an exit the candidate causes, a
resource limit included, is a test failure. A test failure in any run of
the same candidate with the same member, check and target inputs blocks
the cell. A later pass cannot hide it. The verifier reconciles every
submitted result with this ledger, refuses missing or conflicting
verdicts and uses only the approved run. Evidence suppliers cannot
select the attempts that count.

For a method, resolve the admission record and each evidence area in
spec 0055. Check its behaviour digest, independent review and
distinguishing test, check that its catalogue stage is
`release_qualified`, and resolve the shipped pipeline step that runs it;
a method nothing shipped runs is not counted. A method chain resolves
its shape and steps through spec 0067 and counts only when a shipped
pipeline step can run it. For a capability, resolve the shipped action
and its integration and end-to-end evidence. A capability workflow
resolves its shape and actions through spec 0070 and counts only when
every action resolves as a capability does and the workflow's own
runner-signed end-to-end result passed on each target of its shape. For a
variant, resolve its owner, its admitted input class and its
completed check. The owner is a counted method or method chain, or a
capability or capability workflow once a spec admits a class list for
its input kind (spec 0071). A variant counts only when its owner is among this
run's verified members. Results for several required targets join under
the same member identity. The member counts once after every required
result passes.

Within each group, store records in strictly ascending order: by kind
(method, method chain, capability, capability workflow, variant), then
by ordinal as a number (for a chain or a workflow, its shape ordinal
and then its index), then by check
identity, then by target. All of one member's results sit in one group.
Two `V-` ids for the same owner and input class are refused. Give each
group a manifest with its run digest, exact member commitment, evidence
commitment, encoding and verifier version. Use SHA-256 for the
commitments. Sign a canonical byte encoding of the manifest under its
own signing purpose. Every signed encoding begins with its signing
purpose, record type and format version as fields; the verifier refuses
a signature whose purpose is not the one expected at that step. In the
pilot, prefix each field with its byte length as an unsigned 64-bit
big-endian integer. Encode text as UTF-8 and digests as 32 raw bytes.
Encode a sequence with its record count and each length-prefixed record
in order. The format version fixes the field order. Refuse unknown
versions, ambiguous encodings, trailing bytes and evidence whose fetched
bytes fail their digests. The verifier fetches evidence only by digest
from the protected Dell evidence store of spec 0055, and refuses any
reference that names a host, path or address. Retain the input manifest
and evidence under the access controls in spec 0055.

Shared values may be stored once by digest. A group may compress its
rows and proof references, provided the verifier can recover every
member, its admission record and its own signed review. A review
signature covers the method, version, evidence, run and outcome. A group
signature authenticates the manifest. It cannot stand in for the
individual method reviews. A method chain's review is its shape's signed
review, bound to the digest of the shape's catalogue entry, together
with its own runner-signed result (spec 0067). A capability workflow's
review is its workflow shape's signed review, bound to the digest of
that shape's catalogue entry, together with its own runner-signed
result (spec 0070). Use explicit signed per-member records in the
pilot.

### Input and resource limits

The protected run policy sets maximums for the number of groups,
encoded bytes, decoded bytes, member and result rows, referenced
objects, bytes fetched and scratch space for the whole run. It names
an execution time limit and the runner's memory limit. An unset or
unbounded limit refuses the run. Evidence cannot raise a limit.

For the pilot, cap each manifest at 64 KiB, each encoded group at
16 MiB, each decoded group at 64 MiB and each row at 1 MiB. A group
holds at most 16,384 rows. The exact pilot policy records the whole
run's limits before work starts. The verifier runs under the Dell
queue's limits. A larger run needs approved limits justified by
measured resource use and cannot exceed its worker's allocation.

Bound reads before authenticating a manifest. Only after checking
its signature and run binding, stream the group under the approved
limits. Check each declared length and count against bytes remaining
and the policy before allocating or expanding. Count decoded bytes,
rows and referenced bytes during streaming. Stop at the first excess
or incomplete field, even when all signatures are valid. Charge
retries and scratch writes against the run's limits. A refused run
leaves the last verified ledger intact and clears its partial work.

### Replay and refusal

Replay by reopening the frozen catalogue and signed groups with a fresh
verifier. Expand or check membership against the committed records.
Compare canonical identities across all groups and run the distinctness
checks in spec 0055 on the required catalogue domain. Any candidate code
those checks run goes through sealed jobs under the trusted harness and
supervisor; the verifier only reads their supervisor-signed results.
Check implementation and behaviour as well as identity. A range of ids,
a sampled check or a reported group total proves neither disjointness
nor individual results. Treat a digest collision as a refusal when the
bytes differ.

Build the verified ledger in isolation. Publish its totals only after
the whole input passes. A retry or replay of the same verified run first
runs the shared ledger validator on the previous ledger against the
approved original run's trusted context and refuses if that validation
refuses. This permits replay of pilot and fixture runs without admitting
their ledgers to the reader check. Replay preserves the original run
kind, run purpose and ledger signing purpose; it grants no release
credit to a pilot or fixture. It checks the policy digest against the
current protected policy, as the reader check does. The new ledger
carries the original run id. After the replay's own attempts and final
verdict are recorded, replay reads the run-ledger head once and checks
every entry for that candidate from the head the previous ledger names
up to that exact head, whatever its run kind. In both this history check
and the initial shared validation, the trusted supervisor supplies the
exact current replay invocation and its ordered attempt ids. Exclude
only those ids from newer-attempt freshness; never exclude their test
failures or failed replay verdicts. All other attempts remain subject to
the existing history and frozen retry rules, including refused and
interrupted runs and the permitted passing gate, nightly and soak
exclusion. Submitted data cannot name exclusions. This
current-invocation exclusion is available only inside verifier replay,
never to the reader check. Replay refuses if that history for its own
run kind has otherwise changed or if any test failure on the candidate
in a release or pilot run has been recorded in that range, refuses if
any of its own attempts fails, and otherwise returns the same counts in
a new ledger that names exactly that head and carries the next sequence
number. A changed manifest, missing member, overlapping group or
duplicate behaviour refuses the input. The previous ledger then stays
unchanged on disk, and the reader check refuses it until a new verified
run publishes. Compute the method and capability scope shares separately
with checked integer arithmetic. Check the ranking subtotal within the
method ledger.

### Fixture and pilot cases

A fixture run must name the fixture catalogue whose digest the policy
pins, and accepts only `FX-` ids; it refuses any candidate or catalogue
the policy names for a release or pilot run. Its attempts are recorded
only against that catalogue and its `FX-` ids, never as test failures on
a release candidate. Tests that need a release ledger build one from a
separate test catalogue with the in-memory `QualifyTrust` fake. Use a
fixture with three groups. For each row, use its listed scope for both
the method and capability. Each variant uses the method on its row as
its owner. Use the literal UTF-8 scenario key as the fixture's canonical
input. Stub results may serve the arithmetic and membership tests. The
end-to-end pilot uses actual execution evidence. Use separate author,
reviewer and runner fixture principals and signatures scoped to this
fixture run.

| Group | Method | Capability | Variant | Scope | Ranking method | Scenario key |
|---|---|---|---|---|---|---|
| A | FX-M-000001 | FX-CAP-000001 | FX-V-000001 | On-page and technical search | No | redirect-chain |
| A | FX-M-000002 | FX-CAP-000002 | FX-V-000002 | Text and meaning | No | sentence-boundary |
| A | FX-M-000003 | FX-CAP-000003 | FX-V-000003 | Knowledge and graph analysis | No | isolated-node |
| B | FX-M-000004 | FX-CAP-000004 | FX-V-000004 | Link ranking and protection | Yes | off-topic-signal |
| B | FX-M-000005 | FX-CAP-000005 | FX-V-000005 | First-party and enterprise analytics | No | late-event |
| B | FX-M-000006 | FX-CAP-000006 | FX-V-000006 | Outside the scope share | No | package-uninstall |
| C | FX-M-000007 | FX-CAP-000007 | FX-V-000007 | Editorial coaching | No | answer-early |
| C | FX-M-000008 | FX-CAP-000008 | FX-V-000008 | Research and bounded optimisation | No | research-card-mismatch |
| C | FX-M-000009 | FX-CAP-000009 | FX-V-000009 | Agent operations | No | scoped-agent-request |

The independently enumerated answer is nine methods, nine capabilities,
eight in scope in each ledger, one ranking method and nine variants.
Reorder the group inputs and replay. The totals and verified member
set must agree. Replay a previously accepted run to prove idempotence.
The fixture uses its own `FX-` identities in a separate fixture
catalogue and carries no release credit.

Replay accepted fixture and pilot ledgers through the verifier with
their own protected run contexts and require the same counts. Offer
those same correctly signed ledgers to the reader check through both of
its callers, `sutra methods count` and release criterion 7, and require
refusal. Name a run id that differs from the one signed in the ledger
and require refusal. Offer a ledger to replay with another approved
run's context and require refusal, even when its signature is otherwise
valid. Check a request to select the expected run kind, run purpose,
ledger signing purpose or excluded attempt ids from submitted data or a
caller option: it must not change the trusted context. Record the
current replay attempt before its initial validation and require a valid
replay to pass. Add a newer attempt from another invocation and require
refusal under the existing history rules. Record a real failure in the
current replay, then a pass, and require refusal. After the replay's own
attempts begin and before its final head read, record another run's
qualification attempt, or a refused or interrupted run, of the same run
kind, or a test failure on the candidate in a release or pilot run, and
require refusal; record a passing gate, nightly or soak attempt, or a
refused or interrupted run of another kind, there instead and require
the replay to pass. Change the protected policy between the run and the
replay and require refusal. These cases retain signature, policy,
freshness, retry and input-binding checks; run-kind isolation grants no
exception to them.

Plant one fault at a time. Check an identity repeated across groups, the
same behaviour under a new id, an alias or setting counted as new, a
wrapper counted as a capability, a check rerun counted as a variant, one
numeric setting swept to make many variants, near-identical scenario
keys counted as separate variants, a workflow whose output matches one
of its own actions run alone, a workflow that lists a preset or a
second door of one action, a workflow that only rewords another's
draft, a shape that lists an export or a message sent out of the
product, a `CW-` id whose shape has no `W-` row, a
padded ordinal, a look-alike hyphen
in an id, a self-edited scope register supplied in the run manifest,
evidence from an older version counted without a result for this
candidate, a runner signature reused as a review or manifest signature,
a reference that names a location, adjacent value bands of one input
counted as separate classes, a second run of the same cell submitted
after a test failure in the first, a scope or ranking flag signed only
by its author, an admitted method that no shipped pipeline runs, a
variant whose owner was refused, one input class counted under both a
method and a capability, a method that misbehaves only when probed
during replay, a valid ledger from the previous candidate offered to the
release gate, an older valid ledger for the same candidate, a test
failure recorded after the ledger was published, an empty ledger with
each share reporting its own failure, a pilot ledger and a fixture
ledger offered to both readers, a ledger signed under another purpose, a
pair of ids either side of ordinal 1,000,000, an ordinal too long for an
unsigned 128-bit integer, the run ledger or the signer registry
unreachable for both readers, a newer pilot attempt on the release
candidate that still leaves the release ledger accepted, a pilot test
failure on the release candidate after publication that makes both
readers refuse the release ledger, the same failure followed by a
release replay, which must be refused, the fixture suite run after
publication, which leaves the release ledger accepted, a ledger of an
unknown run kind, an `FX-` id in a release run, a fixture-kind run on
the release candidate with real ids, a manifest whose run kind differs
from the policy's, a passing nightly, gate or soak run after publication
that leaves the release ledger accepted, a refused or interrupted
nightly run that makes it refused, a qualification run whose manifest
calls it a nightly run, two `V-` ids for one owner and input class, a
method whose catalogue stage is not `release_qualified`, a member's
results split across two groups, and an id whose ordinal is zero. Check
an orphaned feature, an incorrect scope or ranking flag, altered
evidence bytes, a stale commit or target, a missing target, a forged or
unauthorised signature, and a reviewer who is also the author. Replace
the registry and run manifest, then sign all records correctly with the
replacement keys. Refuse this run under the real trust root. Refuse a
formerly trusted policy and a revoked key. Replace a passing check with
forged evidence rehashed and signed by an authorised group signer who
has no runner scope for that check. Refuse it before counting. Have the
candidate report a pass while the trusted check fails, then try a
candidate-supplied verifier with the same name. Neither attempt may earn
a signed pass. Run the hidden-answer theft case in spec 0056. Try a
second key belonging to an author, change an author's display name, then
name a co-author as reviewer. Refuse all three reviews. Supply a later
signed pass while leaving out an earlier test failure, then supply
conflicting final receipts. Refuse both. Allow an approved
infrastructure retry only when its interrupted attempt is recorded with
the trusted classification. Check a partial group, unchecked range,
ambiguous input and arithmetic overflow. Test a truncated field, a
declared length of 2^64 minus one, an unsigned oversize envelope, a
correctly signed manifest over its limit, and a small signed group that
expands past its limit. Exhaust the whole run's fetch and scratch
budgets using individually valid groups. Require refusal and the
previous ledger unchanged in each case. Separately, two different check
ids for the same owner and input must give one variant count. At each
independent release boundary, test the floor and one below it. At the
method and capability floors, prove that each percentage cross-product
is represented exactly and passes at the required share. With the total
held at the floor, moving one in-scope member out of scope must fail the
share while every other condition passes. Round-trip each boundary, one
below it and one above it through JSON, MCP and CLI results without
changing a digit. Refuse numeric JSON counts, leading zeroes, signs,
fractions, exponent forms and values outside the checked integer range;
the prior ledger stays unchanged. The verifier reports each condition's
own result. Exercise both scope shares independently with four in scope
out of five, then three out of five, and assert the share condition's
own result while the other share stays above its boundary. Change the
target and signer bindings in a previously accepted fixture to prove
replay refuses stale evidence. Arithmetic fixtures supply no evidence
credit.

### Measure the full route

The full pilot exercises real methods, capabilities and variants from
representative owning designs across every scope area above and includes
out-of-scope controls. Run the tests against the actual approved
pilot targets. Demonstrate the individual evidence, independent
review, signing, verification and replay. Retain the signed manifest,
verifier report and gate records. Record the proof references in
Forgejo.

Report unique verified counts and elapsed wall time through the
whole route, including queues, execution, review, signing, transfer,
verification and retries. Report each stage separately to locate
the bottleneck. Measure retained evidence bytes, peak storage, CPU,
memory and replay time on the named hardware. Give accepted and
refused counts by kind and scope area, including the ranking
subtotal and the measured review rate. Exclude duplicates and
retries from the numerator. Include their work in the elapsed time.
Run a second verifier against the frozen inputs and record any
difference in the accepted set.

Compute the remaining counts from verified release evidence only.
Use remaining time and measured sustained rates to report the
capacity needed for each floor and both scope shares. State hardware,
review staffing, storage and error-rate assumptions. Pilot evidence
covers the exercised members. Release-scale membership, cross-group
distinctness, and per-member review and verification capacity need
separate measured proof before use of a compact format. If a
prerequisite is missing, report the blocked step and any fixture
rehearsal separately. Keep the release blocked until all required
evidence exists.

## Agent configuration

Settings discovery is owned by spec 0014, and scoped change requests,
permission checks and live confirmation by spec 0020. Their members
count in the agent operations area above.

## How it is tested

- A release below any floor, below either required share, or below
  the ranking subtotal is refused. A fixture at each boundary passes
  only when every other condition passes.
- A reused identity, alias, setting, second surface, overlapping
  proof group, stale target or unsigned result cannot increase a
  count. Check the verifier against an explicit small catalogue.
- A feature missing a build task or a design, two owners for one
  behaviour, or a conflicting feature contract blocks release.
- Plant an analytics signal with a high score on an off-topic link.
  The relevance gate still refuses it.

This specification adds no work to public pages. Each owning feature
sets its own footprint budget under AGENTS.md section 5.4.
