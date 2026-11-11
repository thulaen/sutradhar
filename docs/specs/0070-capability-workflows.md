# 0070: Capability workflows

A capability workflow runs separately shipped actions of spec 0020 one
after another, each action reading the artefact the action before it
wrote. Capabilities count "workflows and actions", by the owner's
decision, towards the capability floor in [AGENTS.md
section 3](../../AGENTS.md#3-the-goal-and-the-release-bar). A workflow
counts once when it does something different from every other counted
capability and passes its own end-to-end tests; a changed setting never
makes a new capability. This spec says how workflows are written down,
when one counts, how the `qualify` verifier (spec 0064) proves that no
two counted capabilities behave the same, and how the live product runs
a workflow when a person or an agent asks. It mirrors spec 0067, which
does the same for methods, and is roadmap Phase 7 item 1.

Example: an audit shape has four slots. Slot 1 crawls the site (a full
crawl or a signed-in crawl), slot 2 runs one audit check (redirect chains or
missing titles), slot 3 orders the findings (by traffic or by impact)
and slot 4 turns them into Fix list tasks. That shape has 2 × 2 × 2 × 1
= 8 workflows. Each counts only if all its actions are shipped with
their own evidence, it ran end to end on the test sites in a qualified
run and its output differs from that of every other counted capability.

## What it does

### Words used here

- Action: one named operation of spec 0020's registry, with its own
  `CAP-` id, integration tests and end-to-end tests.
- Artefact kind: the kind of thing an action reads or writes, such as a
  crawl, an audit report, an ordered fix list, a link set, a source
  pack, an outline, a draft or a change request. Each kind has one
  entry in `catalogue/artefacts/`, in TOML, giving its canonical byte
  encoding and its item key (below). These entries are part of the
  catalogue the protected policy pins by digest (spec 0064), and
  `qualify` refuses an entry whose digest differs. An action's catalogue
  entry names the kinds it reads and writes. An artefact keeps the
  strictest visibility and scope of everything it was made from.
- Workflow shape: a named, ordered list of slots, each with the
  artefact kind it writes and the list of actions allowed in
  it. It is not a feature and not a screen.
- Workflow: one choice of action for each slot of a shape.
- Test sites: the two Dell test sites of spec 0064, XenForo and
  WordPress, each loaded with a seeded made-up site whose version the
  protected policy pins. Each shape names its targets, one platform or
  both; every action it lists applies to each target, and every
  workflow of the shape runs on each target, with the results joined
  under the member's identity. Single capabilities keep spec 0064's own
  targets.
- Item key: for each artefact kind, the stable key of one item in it,
  which the kind's entry fixes:
  - a finding: its page id and check id;
  - a link: its source and destination id;
  - a passage of a draft: the sorted set of source posts it cites,
    empty when it cites none, and the outline section it sits in,
    never its wording, so rewording keeps it;
  - a tip a check adds to a reviewed draft: that check's action id and
    the item key of the passage it is about; a verdict adds no key;
  - a row of a table, such as an analysis, flagged or report table: its
    metric and segment, and for a flagged row also the flag;
  - an item of an ordered kind, such as an ordered fix list or the task
    list made from it: its own key together with its position;
  - a kind that carries another, such as a change request carrying a
    draft or an explained link set carrying a link set: the keys of what
    it carries.

  Every other kind's entry fixes its own item key, such as a page id
  for a crawl, a source post for a source pack and a section's
  position for an outline. An item key never holds wording, run ids or
  action ids, except a finding's or a tip's check. Every kind a slot
  writes has an item key; admission refuses a kind without one.
- Output digest: the BLAKE3 digest of the canonical encoding of a
  member's last artefact on one test site, with run ids, times and
  other fields the kind's entry marks as run-specific left out.
- Key digest: the BLAKE3 digest of the sorted list of item keys of that
  artefact. A workflow that only rewords or reformats another member's
  items gets the same key digest; for an ordered kind, a new order
  changes it. A check that adds no tip on the test sites adds no key.

### Workflow shapes

1. Each shape is an entry in `catalogue/workflows/`, in TOML, with the
   same file and line limits as spec 0055's catalogue. It has a stable
   id, `W-` followed by an ordinal written as spec 0064 writes member
   ordinals, never reused; a name; its input kind; and its slots in
   order. Each slot names its artefact kind and its actions by action
   id. Workflow sheets are part of the catalogue the protected policy
   pins by digest (spec 0064).
2. Every action in a slot reads the artefact kind of the slot before it
   (or the shape's input kind, for the first slot) and writes the
   slot's kind. Admission refuses a shape where any action does not.
3. A shape has at least two slots, and each slot lists at least one
   action. A shape whose slots are another shape's slots, in the same
   order or another, is refused whatever its targets: reordered steps
   earn nothing by themselves (spec 0064), and one shape never counts
   once per platform.
4. A shape is frozen once admitted. Any change, such as an action added
   to a slot, makes a new shape with a new ordinal, admitted from the
   start. The old shape keeps its workflows; where the two share a
   workflow's behaviour, "Proving workflows differ" keeps one.
5. An action runs with its own active settings. A shape never lists a
   setting, a preset, a profile, a template value or a playbook (spec
   0027's `rank.playbook.choose` and spec 0048's team playbooks
   included) as an action, nor a view or a default sort, and never
   lists one action twice. Each action is named by its action id in
   spec 0020's registry. A slot may not list two doors of one
   behaviour: the desktop, `sutra`, MCP and platform wrappers of an
   action share its identity (spec 0064).
6. A slot may not list an action that accepts, applies, publishes or
   moderates, or one that sends data out of the product, such as an
   export to an outside account or a message to a person. A shape
   whose last slot changes a site writes a change request: a plan of
   spec 0020 or a draft on the review page that a person accepts
   through their own action, outside the workflow.
7. `sutradhar-methods` owns the workflow shape types, the artefact
   kind entries, the id rules below, the code that turns a workflow id
   back into its actions and the shapes' stage records, beside its
   method chain code (spec 0067), so `qualify` needs nothing more than
   spec 0064 gives it. `sutradhar-actions` owns only the recipe runner
   and `workflow.recipe.run` ("Running a workflow"), and depends on
   `sutradhar-methods`, never the other way. The recipe runner calls
   spec 0020's action runner through its typed interface, wired at the
   composition root, once per action. A recipe is never an action of a
   slot, so recipes never nest.

### Shape stages

A shape moves through spec 0055's stages, only through
`sutra methods advance`, which takes `W-` ids as well:

- `spec_approved`: the owner approves its sheet, in batches, each batch
  the shapes of one scope area of spec 0064, with a plain-English
  summary of the shape and each slot.
- `implemented`: every action it lists is registered in spec 0020's
  registry, and the recipe runner ("Running a workflow") can run any
  of its workflows.
- `verified`: the shape passes review of sources, distinctness,
  rights, correctness on the test sites, resources and integration.
  Every signature on a shape, on its reviews and on its `W-` scope row,
  comes from someone who is neither one of its authors nor an author
  of any of its actions. Its distinctness review argues why workflows
  that differ in two or more slots cannot be near copies.
- `release_qualified`: as spec 0055 sets for a method.

### Workflow identity

1. A workflow's member identity is `CW-`, its shape's ordinal, `-` and
   its index, each written as spec 0064 writes member ordinals. The
   index counts from 1 in the order of the shape's workflows, with the
   first slot's choice changing slowest and each slot's actions taken
   in the order its catalogue entry lists them. Because a shape never
   changes, an index always names the same actions.
2. A shape's workflow count is the product of its slot sizes, worked
   out with checked unsigned 128-bit arithmetic; a shape whose count
   would overflow is refused. An index of 0 or above the count, or a
   shape ordinal that names no shape in the pinned catalogue, is
   refused when an id is read.
3. Ids are worked out, never stored one by one, and every reader turns
   an id back into the same actions. A workflow is counted only from
   its own runner-signed result, never from a shape's size.
4. Workflows sort by shape ordinal, then by index, each as a number. In
   a group, workflow records sit after single capability records and
   before variant records (spec 0064).
5. An action draws any seed it uses from the test site's seed list by
   its action id alone, so it uses the same seed alone or in any slot.
   An action that calls a model uses the model pinned by digest in the
   protected policy (spec 0024) with its fixed decoding settings. An
   action whose output differs between two runs of the same input on
   the same test site fails its checks; that repeat run is part of the
   action's own evidence as a single capability, never run again inside
   each workflow.

### When a workflow counts

A workflow counts towards the capability total of a release run when
all of these hold:

1. Every action is shipped in the release candidate, registered in spec
   0020's registry, and has its own integration and end-to-end evidence
   that matches the candidate, as spec 0064 resolves a single
   capability.
2. Its shape is at `release_qualified`, with evidence that matches the
   release candidate.
3. The workflow ran end to end on each target of its shape in the
   qualified run, in the sealed runner jobs of spec 0064, through the
   recipe runner and spec 0020's action runner, with every permission
   and agent rule checked. Its own result, signed by the runner, passed
   each action's own checks on the artefact that action was given, and
   the workflow's own end-to-end check: the last artefact is valid for
   its kind, and every change request it wrote is an unexpired plan or
   review-page draft whose preview matches the test site's current
   version, with nothing applied. A shape cannot add, remove or loosen
   a check.
4. The result carries the output digest and key digest of the last
   artefact on each target.
5. No other counted capability behaves the same ("Proving workflows
   differ").

Each single capability's runner-signed result whose action writes an
artefact kind of `catalogue/artefacts/` carries the same two digests,
worked out the same way, so both sides of every comparison are signed
(spec 0064); a single capability whose action writes no such kind is
never compared with a workflow. A workflow's review is as spec 0064
sets for a capability workflow. A workflow takes its primary scope from
its shape's signed `W-` row in spec 0064's scope register, and counts
in that area's share. Workflows count in the capability total and the
capability in-scope total; they never count in the method total or the
ranking subtotal.

### Proving workflows differ

1. `qualify` works out every output digest and key digest again from
   the artefacts the runner-signed results reference, refusing as a bad
   value an artefact that does not decode as its kind, and refusing a
   result whose stated digest differs. This covers every single
   capability that meets spec 0064's counting rule and writes a
   catalogued kind, and every workflow that meets conditions 1 to 4.
   It writes one row per member to the run's scratch space (spec
   0064), holding its digest on each target it ran; the targets are
   taken in a fixed order, XenForo first and WordPress second. Two
   members match when they share at least one target and match on
   every target both ran. Rows of members that ran both targets are
   sorted on disk by kind, then first-target digest, then
   second-target digest, so equal rows sit together; a second copy of
   them is sorted by kind, then second-target digest. Rows of members
   that ran one target are sorted by kind, target and digest. The
   first-target rows are merged with the first sorted copy and the
   second-target rows with the second, each list read once per merge.
2. Where a workflow matches another member by output digest, only the
   lowest counts: single capabilities before workflows, single
   capabilities by ordinal, workflows by shape ordinal and then index.
   A member that loses still counts as a lower member for every
   higher one it matches.
   Members whose actions are the same set, in any order and in any
   shape, are one behaviour, and only the lowest counts; they are
   found by sorting the members on disk by their action sets, each set
   sorted by action id. Only workflows lose here; a single capability is never
   removed by this item. Each workflow that loses is recorded as
   "behaves the same as" that member and does not count in this run.
   Two single capabilities with equal digests keep spec 0064's own
   rule for one behaviour under two ids.
3. `qualify` then removes every workflow left after item 2 that
   matches a lower member by key digest, using item 1's rows, sorts
   and merges with key digests in place of output digests.
4. `qualify` then compares near members, where at least one is a
   workflow. A member's actions are taken as a set, whatever their
   order, so a new order of the same actions is no escape; a single
   capability's set is the action alone. Two members are near when
   their sets have the same size and differ in exactly one action, or
   when one set holds every action of the other and more. A workflow
   that does not count because a lower near member did not run
   ("Performance") still takes part here as a lower member. Two near
   members whose item key sets on each target both ran share at least
   the share the protected policy sets (their overlap over their
   union) behave the same, and the higher is removed. Two empty sets
   on a target count as the same on that target. Nothing shows that
   two near members with no target in common differ, so the higher is
   removed.
   - Each member's sorted item key sets are written to scratch space
     once, sorted by member, and both sets of a pair are read from
     that one copy, never from the artefacts.
   - Pairs are found by sorting keys, never by comparing every pair.
     For same-size pairs, each workflow writes one removal key per
     action: its set less that action, sorted by action id, tagged
     with the set's size and no position; a single capability writes
     none. Two members sharing a removal key are a pair.
   - For pairs where one set holds the other, placings are worked out
     from the pinned catalogue. A placing puts each slot of a shorter
     shape into a distinct slot of a longer shape that lists an action
     the shorter slot lists, in any order; a single capability's
     action is placed in one slot that lists it. A placing is known by
     the set of longer slots it uses, so each is written once. For
     each placing, each workflow of the longer shape writes one placing
     key: its own actions in the placed slots, sorted by action id.
     Every member also writes its own sorted set, and a placing key
     pairs only with a member whose own sorted set equals it. The number of
     removal and placing keys is known from the catalogue before a
     run and counts in both refusals ("Performance").
   - Pairs are judged on the set left after item 3, so the result does
     not depend on the order of the pairs. This check can only remove a
     member from the count, never add one.
5. The sorted lists are referenced objects of the run, under spec
   0064's limits on referenced objects and scratch space; the evidence
   group and the ledger hold only their digests and sizes. A replay
   rebuilds the lists from the runner-signed results, sorts them again
   and refuses on any difference. No estimate, sample or formula over
   shape sizes supplies a count.

### Running a workflow

Workflows are features people use, not only counted ids. Features 458
to 477 (workflow automation) own the workflow builder, run history and
templates; this spec adds the saved recipe and its runner, which
`sutradhar-actions` owns ("Workflow shapes", item 7).

1. A workflow that counted in the running version's qualified run is
   offered as a saved recipe, named by its `CW-` id with a title made
   from its actions' summaries. Recipes are never indexed or listed one
   by one: `sutra find` and the palette match words against each
   shape's name and its slots' action summaries, which spec 0020's
   index already holds, and return shapes with the matching actions in
   each slot. A `CW-` id is worked out only when the caller picks one
   action per slot, and is offered only if it counted. The qualified
   run's counted set is one of its referenced objects: for each shape,
   the sorted ranges of indexes that counted, signed with the run's
   evidence, its size capped by the protected policy; `qualify`
   refuses a run whose set would pass the cap and reports the size
   needed. `sutradhar-methods` owns its encoding and its reader. The
   running version checks its digest and signature once, when it
   installs the set into a read-only store, and find and the recipe
   runner then read it by binary search on disk, never whole.
   `sutra describe <CW-id>` prints its actions in order, each one's
   permission and agent rule, and what it changes.
2. A recipe runs through the action `workflow.recipe.run`, which takes a
   `CW-` id and the shape's input. Its MCP tool takes the same input,
   with the JSON Schema of the shape's input kind. A `CW-` id that is
   not in the counted set is refused with SUT-802, naming the id, and
   nothing runs. The recipe run's own
   `action_runs` row records its `CW-` id and has no parent run id (spec
   0020, "Data"); the runner, its doors and its routes earn no
   capability of their own.
3. Before any action starts, the runner checks the caller against every
   action in the recipe: the agent rule, the Cedar permission through
   dwarpal, and the token's scopes and action allow list (spec 0020).
   The action runner checks the allow list again as each action
   starts, whatever tools the token was shown. If any
   would refuse, the run is refused before it starts with that
   action's own code (SUT-801 or SUT-805, say), naming the action, and
   nothing runs.
4. Each action then runs through spec 0020's action runner in slot
   order, as the same caller, with the same token and door, never as a
   service identity. The action runner checks it again when it starts,
   against what the earlier slots wrote, so a token revoked or a right
   lost during the run stops it there with that check's own code. Each
   action has its own plan, audit entry, change-journal entry and
   event. The run writes one `action_runs` row for itself and one for
   each action; only the recipe runner sets an action row's parent run
   id, to the recipe run's id. Artefact content is only data: it never
   chooses the next action, its input or the caller's rights.
5. Each artefact a recipe passes on keeps the strictest visibility and
   scope of what it was made from. A later action, and the recipe's
   output, show only what the caller may see; an agent gets private
   text only with spec 0020's private scope.
6. An action that changes a site stops at its plan or draft: the
   recipe's output is a change request on the review page (spec 0028)
   or a plan waiting for confirmation (spec 0020). No recipe accepts,
   applies or confirms anything in a person's place, and an agent's
   recipe that reaches a settings change waits for the owner's live
   confirmation like any other.
7. A failure in an action stops the run there. Artefacts already
   written stay as drafts or plans with their own expiry; nothing is
   applied. The run reports which slot failed and why.
8. Long runs report through the one progress system (spec 0016) and are
   cancelled only through `karya.cancel`.

## First shapes

Five shapes, one batch per scope area. The "Listed" column is the most
actions the sheet lists for that slot. Admission refuses an action
whose kinds do not match its slot, and a slot holds only actions whose
features are designed and shipped, so a shape is admitted with what
remains, under its own ordinal. Each action is cited by its feature
number in [features.md](features.md), whose "Which spec designs each
feature" names its owning design, or by the spec that names it.

### W-000001: Audit to Fix list

Scope row: on-page and technical search. Input kind: a test site.
Targets: both.

| Slot | Writes | Actions | Listed |
|---|---|---|---|
| 1. Crawl | crawl | 113 built-in crawl (spec 0044); 1318 signed-in test crawl | 2 |
| 2. Check | audit report | 117 to 186, less 121, 161, 164 and 181 to 185; 635 to 643; 1314, 1316, 1320, 1322 (specs 0044 and 0047 and the range 2 and 4 designs) | 75 |
| 3. Order | ordered fix list | 189 traffic-based priority; 549 impact-ranked backlog (spec 0048) | 2 |
| 4. Hand off | task list | 540 automatic tasks from issues (spec 0048) | 1 |

- Feature 114 (scheduled crawls) is a schedule, a setting; 116 (list
  mode) reads a pasted list, not a site; 185 (custom rules) runs rules
  a person writes; 644, 645 and 1315 apply to WordPress only; spec
  0048's default order is a view. None is listed.
- The signed-in crawl's artefact keeps the visibility of the pages it
  read ("Words used here").
- Workflows: 2 × 75 × 2 × 1 = 300.

### W-000002: Sourced draft to review

Scope row: editorial coaching. Input kind: a writing brief from spec
0041's `prepare_writing_brief`. Targets: both.

| Slot | Writes | Actions | Listed |
|---|---|---|---|
| 1. Sources | source pack | 748 to 753, 755, 757, 758, 762, 763, 765, 767, 768, 771, 773, 774, 777 | 18 |
| 2. Outline | outline | 779, 783 to 786, 791, 797, 801 | 8 |
| 3. Draft | draft | 803 to 832 and 834 to 847, one article type each | 44 |
| 4. Restructure | draft | 1121, 1129, 1130, 1141, 1142, 1144 to 1146 | 8 |
| 5. Reword | draft | 1118 to 1120, 1122 to 1128, 1133 to 1138, 1147 to 1151 | 21 |
| 6. Version | draft | 1192 skimmable, 1195 easy-read, 1196 printable, 1200 expert sidebars | 4 |
| 7. Coach | reviewed draft | spec 0042's seven Narada checks and AI readability advice; 236, 237, 240, 241, 252, 255, 256 (specs 0041 and 0042) | 15 |
| 8. Gate | reviewed draft | 1153 to 1185, less 1158 and 1159 | 31 |
| 9. Submit | change request | `sutra content submit` (spec 0052) | 1 |

- A reviewed draft is the draft unchanged, with each check's tips and
  verdicts added, tagged with the action id that wrote them.
- Narada's "observed, not judged" is a wording rule, not a check. Gate
  1158 and 1159 need an expert and 1186 a person's sign-off, so none
  is listed. 756 and 766 read a draft, not a brief; 787 reads an
  existing page; 780 and 790 write several options. Each Version
  action reads one draft and writes one version of it. 1131 reads two
  drafts; 1132, 1188 to 1191, 1193, 1194, 1197, 1199, 1201 and
  1206 write two or more drafts, and choosing one of them would be a
  setting; 1207 writes summaries, not a draft; 1205, 1208 and 1209 are
  standing language rules; 1198 makes variants for a split test, which
  belongs to that test; 833, 1140 and 1143 apply to WordPress only,
  and 1139 to XenForo only.
- An article type, a rewording or a version whose action differs from
  another only by a prompt or template is a preset; the key digest
  finds it and it does not count.
- Every draft cites the posts it came from and never invents facts
  (features.md, "Rules that hold for every feature").
- Workflows: 18 × 8 × 44 × 8 × 21 × 4 × 15 × 31 × 1 =
  1,979,873,280.

### W-000003: Visits to report

Scope row: first-party and enterprise analytics. Input kind: darshak's
`human` day aggregates for the test site (spec 0034). Targets: both.

| Slot | Writes | Actions | Listed |
|---|---|---|---|
| 1. Analyse | analysis table | 1336, 1338 to 1343, 1345, 1351 to 1353, 1355; 524, 525, 527, 528, 531, 532, 533, 536, 537 (spec 0034 and the range 2 and 4 designs) | 21 |
| 2. Flag | flagged table | 313 anomaly detection (spec 0043) | 1 |
| 3. Report | report | 307 | 1 |

- Features 1337 and 1344 build or run segment steps a person writes,
  1346 to 1349 set rules and views, and 314 needs targets a person sets;
  all are settings, so none is listed. 530 is a fixed set of 1344's
  steps (features.md, "Shared parts"). Spec 0034's before-and-after view
  is a view.
- 309, 310, 534, 555 and 1350 send data out of the product, or keep
  it refreshing outside it (shape rule 6).
- Provenance, sample size and uncertainty travel with every table.
- Workflows: 21 × 1 × 1 = 21.

### W-000004: Agent content request

Scope row: agent operations. Input kind: an agent's request in a scoped
MCP session (spec 0020). Targets: both.

| Slot | Writes | Actions | Listed |
|---|---|---|---|
| 1. Request | content request | `sutra content request` (spec 0052) | 1 |
| 2. Pack | source pack | `sutra content pack` (spec 0052) | 1 |
| 3. Draft | draft | 714 create draft tool | 1 |
| 4. Check | reviewed draft | 742 exact site preview, 1185 second-assistant check | 2 |
| 5. Submit | change request | `sutra content submit` (spec 0052) | 1 |

- Every action here keeps its own agent rule: drafting is allowed, and
  the submitted draft waits for a person's decision. 744 (template
  filler) reads a data table, not a source pack, so it is not listed.
- Workflows: 1 × 1 × 1 × 2 × 1 = 2.

### W-000005: Link finding to review

Scope row: link ranking and protection. Input kind: the test site's
posts after spec 0022's reading. Targets: both.

| Slot | Writes | Actions | Listed |
|---|---|---|---|
| 1. Find | link set | 2 (spec 0023), 3 (spec 0047), 4, 6, 7 | 5 |
| 2. Explain | explained link set | `sutra rank explain` (spec 0027) | 1 |
| 3. Preview | change request | `setu.suggestion.preview` (spec 0028) | 1 |

- Every link set passes spec 0027's relevance gate inside
  `shodhan.rank`; a slot never loosens it.
- Features 5 and 61 feed caps inside `shodhan.rank`, 59 checks the
  site, 9 drafts pages, 58 prefers anchor wording, 63 places menus, 10
  needs a paired forum and blog, and 67 applies to XenForo only, so
  none is listed.
- Approval stays with `setu.suggestion.approve`, a moderator's action.
- Workflows: 5 × 1 × 1 = 5.

### Sizes on paper

| Shape | Scope area | Workflows |
|---|---|---|
| W-000001 | on-page and technical search | 300 |
| W-000002 | editorial coaching | 1,979,873,280 |
| W-000003 | first-party and enterprise analytics | 21 |
| W-000004 | agent operations | 2 |
| W-000005 | link ranking and protection | 5 |
| Total | | 1,979,873,608 |

These are ids, not capabilities. Only a qualified run supplies a count,
and nearly all of the total sits in one shape. A draft passage keeps
its key when reworded, so workflows of W-000002 that differ only in the
Reword or Version slot count only where the cited posts, the sections
or a later check's tips differ, and a Coach or Gate check adds keys
only through the tips it finds. How many of W-000002's ids survive is
known only from a qualified run and may be far fewer than its ids; a
count below the floor is reported as below it. A shape for another
scope area, or a wider shape for one of these, is a new shape with a
new ordinal.

## Limits

- The live product runs only workflows of shapes at `release_qualified`
  in the running version that are in its counted set. Qualified runs
  and lab runs take their workflows from the run manifest and may run
  any workflow of an admitted shape on the test sites, through the
  same recipe runner, without the counted-set check.
- A shape's slot count and action lists are bounded by the protected
  policy, so its size is known before a run starts.
- A recipe run holds one saved artefact per slot and runs one action at
  a time; it stays within the service slice and the heavy-job limits of
  AGENTS.md section 11.
- Single capabilities, methods, chains and variants keep their own
  definitions in their owning specs; this spec covers workflows only.

## Performance

- A qualified run works through each shape's workflows in index order.
  It works out each shared start of a workflow once and keeps at most
  one saved artefact per slot, so workflows that share their first
  slots share that work. On each target, W-000002's slots run 18; 144;
  6,336; 50,688; 1,064,448; 4,257,792; 63,866,880; and, for the last
  two, once per workflow. Those last slots set the cost, so a run over
  the whole shape is refused on time unless its manifest names index
  ranges.
- A run manifest may name index ranges of a shape, each including both
  its ends; workflows outside them do not run and do not count. A
  range that starts at 0, starts above its end, reaches past the
  shape's count, or overlaps another range of the same shape, is
  refused. A workflow counts only when every lower member near it
  under "Proving workflows differ", item 4, in any shape, also ran in
  the same run, so a manifest cannot skip the near-member check. That
  set is worked out from the pinned catalogue and the manifest before
  the run starts.
- `qualify` refuses a run before it starts when the measured cost of
  each slot's actions, times the number of times each slot runs, plus
  the sorts, the second copy of the two-target rows, the keys, item 2's
  sort of action sets, the removal and placing keys, the item key sets,
  the pair check, the near-member set of the manifest and the replay,
  passes spec 0064's execution time limit, or when its digest lists and
  their second copy, key lists, action sets, removal and placing keys,
  item key sets, artefacts and replay would pass the scratch-space
  limit. Both refusals work out the sizes with checked arithmetic,
  refuse on overflow and report the size needed.
- A recipe run in the live product is refused before it starts when
  the sum of its actions' declared limits, worked out with checked
  arithmetic, passes the caller's rate limit (spec 0020).

## How it is tested

Rust tests in `sutradhar-methods` (admission, ids, kinds and keys),
`sutradhar-actions` (the recipe runner and find) and `qualify`, with
made-up shapes and actions, and end-to-end tests on the Dell test
sites:

- A shape whose action does not read the previous slot's kind is
  refused at admission, and so is a shape that reorders another's
  slots, lists one action twice, lists a setting, preset, playbook,
  view or second door of one action, lists a recipe, or lists an
  action that does not apply to one of its targets.
- A shape listing an accept, apply, publish or moderation action, or
  one that exports or sends data out of the product, is refused.
- An artefact entry whose digest differs from the pinned one is
  refused.
- Every workflow id turns back into the same actions; an index of 0,
  an index above the count, a padded ordinal and a shape whose count
  overflows are refused. Changing a shape makes a new shape and leaves
  every old id naming the same actions.
- A workflow with an action missing from the candidate, lacking its
  own integration or end-to-end evidence, with its shape below
  `release_qualified`, or run on only one target of a two-target
  shape, does not count.
- A workflow whose output digests equal a single capability's does not
  count, and of two equal workflows only the lower counts.
- A workflow whose last action only rewords another member's draft
  matches by key digest and does not count; one that puts an ordered
  fix list in a new order does not match.
- A Coach or Gate check that finds no tip on the test sites leaves the
  key digest unchanged, and a verdict adds no key.
- A kind with no item key is refused at admission, and a change
  request carrying a draft has that draft's keys. A passage citing no
  post, or several, gets the key its rule gives.
- Two shapes with the same slots for different targets: the second is
  refused.
- Two members with empty item key sets on a target count as the same
  on it.
- Two adjacent ranges that share an end index are refused as
  overlapping; two that meet without sharing one are accepted.
- A manifest that leaves out a lower near member in another shape:
  the higher workflows near it do not count.
- `workflow.recipe.run` refuses a `CW-` id that did not count with
  SUT-802, and the counted set is read without loading it whole.
- A single capability with one target and a two-target workflow are
  compared on the target both ran.
- A workflow with a slot that changes nothing reaching the last
  artefact matches the shorter set and does not count. With workflows
  A, B and C in that order, where A and B are near and behave the same
  and B and C are near and behave the same, only A counts.
- Two same-size sets that differ in one action, where the new action
  sorts to another rank, such as {1, 2, 5} and {1, 5, 7}, are paired.
- A shorter shape whose slots sit inside a longer shape in reversed
  order is paired with the longer shape's workflows.
- Two single capabilities are never paired by a removal key, and two
  members with no target in common never match by digest.
- A two-target workflow that matches a lower two-target member on one
  target and differs on the other is not removed by item 2.
- A one-target member on the second target is found against a
  two-target workflow with the same second-target digest.
- A single capability whose action writes no catalogued kind is never
  compared with a workflow, and its result is not refused for lacking
  digests.
- An action whose output differs between two runs of one input fails.
- A replay with one digest changed on disk is refused.
- A recipe whose third action the caller may not run is refused before
  the first starts, naming that action, and no audit entry claims a
  success. A token revoked after slot 1 stops the run at slot 2 with
  that check's own code.
- Every action of a recipe runs as the caller, and each action's
  `action_runs` row carries the recipe run's id as its parent run id;
  the recipe run's own row has none.
- Find returns shapes and matching actions, never a list of recipes,
  and stays within spec 0020's find targets with W-000002 qualified;
  a picked `CW-` id that did not count is not offered.
- A post on a test site whose text asks for another action changes no
  action, input or right of the recipe that reads it.
- A recipe whose slot 1 read member-only pages never shows their text
  to a caller or agent who may not read them.
- An agent's recipe ending in a submit leaves a draft for a person, and
  one reaching a settings change waits for the owner's live
  confirmation; `--yes` and MCP never confirm it.
- A failure in slot 2 leaves slot 1's artefact as a draft and applies
  nothing.
- The capability share counts a workflow in its shape's scope row, and
  no workflow reaches the method total or the ranking subtotal.
- A run whose lists would pass the scratch-space limit, or whose cost
  would pass the execution time limit, is refused before it starts;
  so is a manifest range that starts at 0, starts above its end, runs
  past a shape's count or overlaps another. A workflow whose lower near
  members ("Proving workflows differ", item 4, in any shape) did not
  all run in the same run does not count.
- A run whose counted set would pass the policy's cap is refused,
  reporting the size needed, and no set is cut short.
- Two shapes for different targets whose workflows hold the same
  actions count once; a workflow with its slots in another order
  counts once with the original.
- A token shown only `workflow.recipe.run` cannot run an action
  missing from its allow list.

## Changes from the source

Written fresh from specs 0020, 0055, 0064 and 0067 and the owner's
capability decision.
