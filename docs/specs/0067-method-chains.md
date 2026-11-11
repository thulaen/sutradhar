# 0067: Method chains

A method chain runs separately qualified methods one after another,
each step reading the output of the step before. The owner decided on
11 November 2026 that a chain counts once towards the method floor in
[AGENTS.md section 3](../../AGENTS.md#3-the-goal-and-the-release-bar)
when it behaves differently from every other counted method, and that a
changed setting never makes a new method. This spec says how chains are
written down, when one counts and how the `qualify` verifier (spec
0064) proves that no two counted methods behave the same. It extends
the catalogue of spec 0055 and is roadmap Phase 7 item 1.

Example: a ranking shape has three slots. Slot 1 scores a candidate
(BM25 or BM25F), slot 2 adds a second signal (link history or topic
overlap) and slot 3 blends them (a weighted sum or a learned blend).
That shape has 2 × 2 × 2 = 8 chains. Each counts only if all its steps
are qualified, a shipped pipeline step can run it, it passes its own
checks and the order of its outputs on the probe set differs from that
of every other counted method.

## What it does

### Words used here

- Step: a method from spec 0055's catalogue used inside a chain.
- Shape: a named chain plan: an ordered list of slots, each with the
  output kind it writes and the list of steps allowed in it. Spec
  0055's "family" groups methods for review; a shape is not a family.
- Chain: one choice of step for each slot of a shape.
- Probe set: spec 0055's probe set for one input kind and output kind
  (scores and orderings are one output kind there). A chain runs on
  the set for its shape's input kind and the output kind of the shape's
  last slot, the same set a single method of those kinds uses. Every
  comparison below is between members run on the same probe set, and
  admission refuses a shape that has no such set.
- Behaviour digest: as spec 0055 defines it, worked out for a chain on
  its probe set.
- Order digest: the BLAKE3 digest of the rank order of a member's
  outputs within each probe batch, for a score or an ordering, so a
  rescale that keeps the order gives the same order digest. Scores equal
  as numbers take equal rank, the lowest rank of their tied group, with
  -0.0 equal to +0.0; a score that is not a number fails the member's
  checks. Scores and orderings are one output kind, as spec 0055's
  probe sets take them. For any other output kind the order digest
  equals the behaviour digest.
- Coarse digest: the order digest with each rank replaced by its band.
  The protected policy sets the number of bands, b, at least 1. In a
  probe batch of n items, with c the smaller of b and n, rank r (from 1)
  falls in band 1 + floor((r - 1) × c / n), worked out with checked
  arithmetic; where b is n or more, the coarse digest of that batch
  equals its order digest. A change that keeps every item in its band
  gives the same coarse digest; a change that moves an item across a
  band edge is left to item 5 of "Proving chains differ" (SUT-503). For
  any other output kind it equals the behaviour digest.
- The order and coarse digests are used only to find members that
  behave the same; spec 0055's behaviour digest keeps every other use.

### Shapes

1. Each shape is an entry in `catalogue/shapes/`, in TOML, with the
   same file and line limits as spec 0055's catalogue. It has a stable
   id, `S-` followed by an ordinal written as spec 0064 writes member
   ordinals, never reused; a name; the shipped pipeline step that runs
   its chains; its input kind; and its slots in order. Each slot names
   its output kind and its steps by method id. Shape sheets are part of
   the catalogue the protected policy pins by digest (spec 0064).
2. Every step in a slot reads the output kind of the slot before it (or
   the shape's input kind, for the first slot) and writes the slot's
   output kind. Admission refuses a shape where any step does not.
3. A shape has at least two slots, and each slot lists at least one
   step. A shape whose slots are another shape's slots in another order
   is refused: reordered steps earn nothing by themselves (spec 0064).
4. A shape is frozen once admitted. Any change, such as a step added to
   a slot, makes a new shape with a new ordinal, admitted from the
   start. The old shape keeps its chains; where the two share a chain's
   behaviour, "Proving chains differ" keeps one.
5. A shape's steps keep their own settings, read from each step's
   active settings. A shape never lists a setting as a step, and spec
   0055's check for parameter variants (SUT-501) runs on every slot.
   One method may appear in only one slot of a shape: a method repeated
   across slots acts as a count of rounds or a changed weight, a
   setting, so admission refuses it (SUT-501).
6. `sutradhar-methods` owns the shape types, the id rules below and the
   code that turns a chain id back into its steps.

### Shape stages

A shape moves through spec 0055's stages, only through
`sutra methods advance`, which takes shape ids as well as method ids:

- `spec_approved`: the owner approves its sheet, in batches, each batch
  the shapes of one scope area of spec 0064, with a plain-English
  summary of the shape and each slot.
- `implemented`: the shipped pipeline step it names is registered and
  can run any of its chains.
- `verified`: spec 0055's admission areas pass for the shape as a
  whole: sources, distinctness, specification, rights, correctness on
  the probe set, usefulness, resources, integration and reviews. Every
  signature on a shape, on its reviews and on its `S-` scope row, comes
  from someone who is neither one of its authors nor an author of any
  of its steps. Its distinctness review argues why chains that differ
  in two or more slots cannot be near copies.
- `release_qualified`: as spec 0055 sets for a method.

### Chain identity

1. A chain's member identity is `MC-`, its shape's ordinal, `-`, and
   its index, each written as spec 0064 writes member ordinals. The
   index counts from 1 in the order of the shape's chains, with the
   first slot's choice changing slowest and each slot's steps taken in
   the order its catalogue entry lists them. Because a shape never
   changes, an index always names the same steps.
2. A shape's chain count is the product of its slot sizes, worked out
   with checked unsigned 128-bit arithmetic; a shape whose count would
   overflow is refused. An index of 0 or above the count, or a shape
   ordinal that names no shape in the pinned catalogue, is refused when
   an id is read.
3. Ids are worked out, never stored one by one, and every reader turns
   an id back into the same steps. A chain is counted only from its own
   runner-signed result, never from a shape's size.
4. Each step in a chain takes its seed as spec 0055 draws a single
   method's seed: from the seed list of the step's own probe set, for
   that step's input and output kinds, by its method id alone, so it is
   the same seed it uses when run as a single method. A chain's evidence
   records the version of every probe set it draws seeds from. Since a
   method sits in only one slot of a shape, a step's seed never depends
   on where it sits. So a chain gives the same outputs on every run of
   the same inputs, and moving a step to another slot never changes its
   noise.
5. Chains sort by shape ordinal, then by index, each as a number. In a
   group, chain records sit after method records and before capability
   records (spec 0064).

### When a chain counts

A chain counts towards the method total of a release run when all of
these hold:

1. Every step is at spec 0055's `release_qualified` stage, with evidence
   that matches the release candidate.
2. Its shape is at `release_qualified`, with evidence that matches the
   release candidate.
3. The shipped pipeline step its shape names is in the release
   candidate and can run the chain.
4. The chain ran on its probe set in the qualified run, in the sealed
   runner jobs of spec 0064; its own result, signed by the runner,
   passed spec 0055's correctness checks for its output kind on the
   probe set and each step's own checks on the input that step was
   given, and carries its behaviour digest and order digest. A shape
   cannot add, remove or loosen a check.
5. No other counted method or chain behaves the same ("Proving chains
   differ").

This is how a chain is independently qualified (spec 0055): it has its
own run and runner-signed result, its shape has a signed independent
review, bound to the digest of the shape's catalogue entry, that stands
as the review of each of its chains, and each of its steps has its own
distinguishing test. A chain counts in the ranking subtotal when its
shape's signed scope row in spec 0064's scope register marks the shape
as ranking; that row covers each of its chains.

### Proving chains differ

1. Every method and chain is compared by its order digest, so a chain
   that only rescales another member's scores matches it. In a
   qualified run the runner-signed result of every method, like that of
   every chain, carries its rank order on the probe set as well as its
   behaviour digest, so both sides of each comparison are signed.
2. `qualify` works out every order digest and coarse digest again from
   the rank order in the runner-signed results, refusing as a bad value
   a rank order whose length differs from its probe batch, whose ranks
   fall outside 1 to the batch size, or whose form is not that of
   lowest-rank ties: its smallest rank must be 1, and a rank v held by
   k items must be the largest rank or be followed by v + k. It then
   refuses a result whose stated digest differs. This covers the run's
   methods that meet spec 0055's counting rule and its chains that meet
   conditions 1 to 4, across all its evidence groups. It writes the
   digests to the run's scratch space (spec 0064), sorts them on disk
   by probe set and then digest, and reads the sorted list once.
3. Where a chain shares an order digest with another member run on its
   probe set, only the lowest of them counts: methods before chains,
   methods by ordinal, and chains by shape ordinal and then index. Only
   chains lose here; a method is never removed by this item. Each chain
   that loses is recorded as "behaves the same as" that member and
   does not count in this run. Two single methods with equal digests
   keep spec 0055's own rule: the newer counts only once its
   distinguishing test passes and a reviewer accepts it.
4. `qualify` then sorts the coarse digests of the members left after
   item 3, by probe set and then digest, and removes every chain that
   has a lower member with the same coarse digest, judged on the set
   left after item 3.
5. `qualify` then runs spec 0055's rank-correlation check (SUT-503),
   through the one function `sutradhar-methods` holds for it, which
   ranks tied scores as the order digest does, on every
   pair of near members run on the same probe set, from any shapes, where
   at least one of the two is a chain; two single methods keep spec
   0055's own rule (item 3). A member's step list is its steps in slot
   order; a method's is the method alone. Two members are near when
   their step lists have the same length and differ in exactly one
   position, or when one list is the other with one or more steps left
   out, the rest kept in order. So a chain is compared with each method
   among its steps that shares its probe set, with every chain one step
   away in any shape, and with every shorter chain whose steps it
   contains; a shape that adds one or more near-identity or small-noise
   slots to another shape's chains gains nothing.
   - `qualify` finds the pairs by sorting, never by comparing every
     pair. Before the run it works out, from the pinned catalogue, each
     placing of a shorter admitted shape's slots, or of one method, in
     order inside a longer shape's slots, where every placed slot shares
     a step with the slot it sits in. Each chain then writes one key per
     position with that position blanked, tagged as a blanked key with
     its length and position, and one key, tagged as a placing key, for
     each placing in its own shape that its steps fill; methods write
     only their own step list. `qualify` sorts the keys and the
     members' own step lists on disk, within one probe set at a
     time. Two chains that share a blanked key are a pair. A placing
     key pairs the chain that wrote it only with the member whose own
     step list equals it; two chains are never paired just because
     they wrote the same placing key.
   - So the keys per chain are its slot count plus the placings in its
     shape, both known from the catalogue before the run; the key count
     goes into the scratch-space refusal (item 7) and the time
     refusal ("Performance").
   - Each member's runner-signed result carries its rank order on the
     probe set, worked out by the sealed runner from the outputs, never
     from a value a step reports; `qualify` writes these to scratch
     space once, sorted by member, and reads each pair's two rank
     orders from that one copy.
   - Pairs are judged on the set left after item 4, and from each
     flagged pair the higher member is removed, so the result does not
     depend on the order of the pairs. This check can only remove a
     member from the count, never add one.
6. The sorted lists are referenced objects of the run, under spec 0064's
   limits on referenced objects and scratch space; the evidence group
   and the ledger hold only their digests and sizes. A replay rebuilds
   the lists from the runner-signed results, sorts them again and refuses
   on any difference. No estimate, sample or formula over shape sizes
   supplies a count.
7. A run whose digest lists, its key lists, its one sorted copy of rank
   orders and the replay would pass spec 0064's scratch-space limit is
   refused before it starts, with the size it needed.

## Limits

- The live service runs a chain only when its shipped pipeline step
  selects it; qualified runs and lab runs may run any chain of an
  admitted shape.
- A shape's slot count and step lists are bounded by the protected
  policy, so its size is known before a run starts.
- Capabilities and checked variants keep their own definitions in
  their owning specs; this spec covers methods only.

## Performance

- A qualified run works through each shape's chains in index order. It
  works out the output of each shared start of a chain once and keeps
  at most one saved output per slot, so chains that share their first
  slots share that work.
- `qualify` refuses a run before it starts when the measured cost of
  each slot's steps, times the number of times each slot runs, plus the
  sorts, the keys, the pair check and the replay, passes spec 0064's
  execution time limit. Both refusals work out the key count, the list
  sizes and the cost products with checked arithmetic, and refuse on
  overflow.
- The run waits in Dell's one heavy-job queue and stays within the
  heavy-job limits of AGENTS.md section 11.

## How it is tested

Rust tests in `sutradhar-methods` and `qualify`, with made-up shapes:

- A shape whose step does not read the previous slot's output kind is
  refused at admission, and so is a shape that reorders another's
  slots.
- A shape listing two settings of one method in a slot is refused as a
  parameter variant (SUT-501), and so is a shape that lists one method
  in two slots.
- A shape whose input kind and output kind have no probe set is refused
  at admission, and a chain is never compared with a member run on
  another probe set.
- Every chain id turns back into the same steps; an index of 0, an
  index above the count, a padded ordinal and a shape whose count
  overflows are refused. Changing a shape makes a new shape and leaves
  every old id naming the same steps.
- A chain with a step below `release_qualified`, a shape below it, or
  a pipeline step missing from the release candidate or unable to run
  the chain does not count.
- A chain whose order digest equals another member's counts only when it
  is the lower one; a chain whose digest equals a method's does not
  count; a chain that only rescales another's scores without changing
  their order gets the same digest and does not count.
- A pair of chains one step apart, in one shape or in two, with a rank
  correlation at or above spec 0055's threshold keeps only the lower
  member. With chains A, B and C in that order, where A and B are
  flagged and B and C are flagged, only A counts.
- A replay with one digest changed on disk is refused.
- A chain whose last step only swaps two ranks inside one band matches
  the other member by coarse digest and does not count; a swap across a
  band edge is not caught by the coarse digest.
- A stochastic step moved one slot later by a near-identity step put
  in front of it keeps the same seed, and the longer chain is flagged
  by SUT-503 against the shorter one.
- A chain ending in a step that sorts scores into an ordering runs on
  the same probe set as the chain without that step, and matches it.
- A chain whose last step adds seeded noise is flagged by SUT-503
  against its neighbour and does not count, and so is a chain of a
  shape that adds one, or two, near-identity or noise slots to another
  admitted shape's chains.
- Two single methods flagged by SUT-503 keep spec 0055's rule and are
  not removed by the chain pass. Chains [a, b, c] and [a, b, d, e], which
  share the placing [a, b], are not paired with each other.
- A chain whose result fails spec 0055's correctness checks for its
  output kind, or one of its steps' own checks, does not count.
- A chain matched by SUT-503 with one of its own steps' methods does not
  count.
- A probe batch with tied scores gives the same order digest on every
  replay, and a score that is not a number fails the member.
- A run whose lists would pass the scratch-space limit, or whose
  estimated time would pass spec 0064's execution time limit, is refused
  before it starts.
- The ranking subtotal counts a chain only when its shape's scope row
  marks it as ranking.

## Changes from the source

Written fresh from the owner's decisions of 11 November 2026.
