# 0071: Checked variants

This spec says how checked variants are counted towards the variant
floor in [AGENTS.md section 3](../../AGENTS.md#3-the-goal-and-the-release-bar),
item 5, when their owners are methods or method chains (spec 0067). It
applies spec 0064's rule: a variant is counted once by its canonical
owning behaviour and its admitted input class, and a class is distinct
from its neighbour only when its test fails with the neighbour's input
in its place, because the input takes a different decision or code
path. It extends the catalogue of spec 0055 and the `qualify` verifier
of spec 0064, and is roadmap Phase 7 item 1.

Example: a chain of spec 0069's wide link ranking shape is run on a
batch in which one destination carries a "Closed" prefix and on its
twin, the same batch with the prefix removed. On the batch, the
cautioned destination ranks strictly below its copy row, and the
trusted harness sees the chain reach the caution branch of its fuse
step. On the twin, the destination and its copy row are equal, so the
output property is false, and the caution branch is missing from the
twin's branch record. So that chain owns one variant for the class
"cautioned destinations". A lone text relevance method, such as
lexical match (BM25), writes a feature table, not an ordering, and
holds no caution step, so the class does not apply to it and is never
run for it.

## What it does

### Words used here

- Owner: a counted member of the run that may own variants. Under this
  spec it is a method or a method chain. A capability or a capability
  workflow of spec 0070 owns a variant only once a spec admits a class
  list for its input kind under the rules here; until then it owns
  none. A variant counts only when its owner counts in the same run
  (spec 0064).
- Input class: an entry in the fixed class list of one input kind,
  naming one decision or code path that inputs of the class reach.
- Class list: the frozen, reviewed list of input classes for one input
  kind, in `catalogue/classes/`, one file per input kind.
- Neighbour: the class an entry is told apart from. It is either an
  earlier class of the same list or the ordinary batch.
- Ordinary batch: a batch of the member's own probe set (spec 0055).
  It is never a class and never earns a variant.
- Class set: for one input kind, one batch for each class and one twin
  for each batch, both built by trusted code from the same seed and the
  same seeded forum as the probe set. The twin differs from the batch
  only in what the class entry says the twin changes, so that it holds
  the neighbour's input in place of the class's. A class entry may also
  ask for a control row: a seeded destination with no match to the
  source phrase, the same in batch and twin; or a copy row: a seeded
  destination equal to a named row. Rows a class calls equal are equal
  in every input field but the destination id and the one field the
  class names, and the class set gives them the same seeded decisions
  and the same links in the seeded forum's graph. The class set is
  versioned with the probe set, pinned by digest with it in the
  protected policy (spec 0064), and kept apart from it.
- Branch record: the list of decision points, named by Rust path in
  the member's reviewed entry for its claimed classes, that its code
  reached on one batch. It is part of the evidence that spec 0064's
  signed evidence digest covers, and a replay reads it again under that
  digest. The trusted harness writes it from instrumentation it
  applies itself to the pinned build, which the sealed job cannot write
  to (spec 0064); a value a member reports in its output is never part
  of it.
- Variant: one pair of an owner and an admitted class.

### Class lists

1. Each input kind has at most one class list. Each class has an
   index from 1, in list order, never reused; a name; its neighbour;
   the kind of decision point it reaches; the output kinds it applies
   to; its output property, a check the trusted harness runs on a
   member's output alone; and how the class set builds its batch and
   twin.
2. A class is admitted with an independent review, signed by someone
   who is not an author of the class entry or of the code that holds
   its decision points, and bound to the digest of the class entry it
   reviewed; admission refuses a review whose digest differs. The
   review argues why inputs of the class take a different decision or
   code path from the neighbour's.
3. Admission refuses a class whose entry differs from its neighbour's
   only by a value, such as a bound, a band of one number or a length.
   Where a class names a length, as "long threads" does, it is admitted
   for the branch that the length crosses, never for the length.
4. Admission also refuses a class whose output property, on its own
   twin, holds whatever a member outputs, or fails whatever a member
   outputs. So a twin's failure always comes from what the member
   output, never from the input or the branch record alone.
5. A class list is frozen once admitted, with the catalogue the
   protected policy pins (spec 0064). A new class is appended with the
   next index and admitted from the start; an admitted class is never
   edited. One later found wrong is excluded with its reason and keeps
   its index; an excluded class owns no variant, and an id naming it is
   refused.
6. The class set's batches never enter a member's behaviour, order or
   coarse digests (spec 0067), and the checks of spec 0068, "Output
   kinds and probe sets", items 4 and 6, are judged on the probe set
   alone. So a one-row class batch breaks no probe-set rule, and adding
   a class changes no member's identity.

### Classes for the candidate table of one source phrase

This list serves the input kind of specs 0068 and 0069 named "the
candidate table of one source phrase", for members whose output is an
ordering or a feature table; the table names which. Its batches are
drawn from the probe set's own seeded forum and decisions, so the fits
and ganit outputs a member uses on the probe set are the ones it uses
on every class batch and twin.

| No. | Class | Neighbour | Output kinds | Decision or code path that makes it distinct | Check, and why it fails with the neighbour's input |
|---|---|---|---|---|---|
| 1 | Batches with one tied pair | the ordinary batch | ordering | The order key's tie-break by destination id | Output property: A, with the lower destination id, is listed before B. A and B are equal in every input field but the destination id, and the batch lists B before A in its input order; the twin raises the reranker score B's row carries from matching (spec 0027) by the amount the class entry states, so a member that orders by score lists B before A and the property fails |
| 2 | Misspelt or inflected phrases | the ordinary batch | ordering | A text view that differs from the raw text on the matched row, read by a word step (spec 0069, "Output kinds, views and checks", item 3), or a character n-gram step's partial-overlap branch | Output property: destination D ranks strictly above the control row. D and the control row are equal in every input field but D's one word. In the batch the source holds an inflected form of that word; the twin changes it to a word neither holds, so both have no match and share a rank |
| 3 | Long threads | the ordinary batch | ordering | The branch where a destination's length crosses the proximity window or the length normalisation cut its member's entry names | Output property: the long destination L ranks strictly below the short destination T, each with one match. L and T are equal in every input field but length; the twin shortens L to T's length, keeping L's match in the text it keeps, so the two are equal, share a rank and the property fails |
| 4 | Cautioned destinations | the ordinary batch | ordering | The caution steps of spec 0069's slots 7 to 9, the fuse step of slot 12 and the relevance gate in the note under its slot table | Output property: destination C ranks strictly below its copy row C2, equal to C in every input field but the prefix. In the batch C carries a "Closed" prefix; in the twin it does not, C and C2 are equal and the property fails |
| 5 | Columns with outliers | the ordinary batch | feature table | A score normaliser's branch for a value outside the spread its member's entry names | Output property: row O's view is the outlier value the member's entry states. In the twin O's value lies inside the spread, so its view is an ordinary one |
| 6 | Single-row batches | the ordinary batch | feature table | The no-spread branch of spec 0069's slot 11 normalisers | Output property: every view is the no-spread value the member's entry states. The twin adds a second row with another value, so the views take their spread values |
| 7 | Cold-start destinations | the ordinary batch | feature table | A prior step's empty-history branch, such as pooling to the forum level or the stated value at no decisions | Output property: the row in R's place has the no-history value the member's entry states as its prior column, and is kept. In the batch R is a destination the seeded forum holds no decisions for; the twin puts in R's place a destination with seeded decisions and the same text, so its prior differs |
| 8 | Isolated destination threads | the ordinary batch | feature table | A graph step's branch for no path or no shared neighbour between the source thread and the destination | Output property: the row in G's place has the no-path value the member's entry states as its graph column, and is kept. In the batch G is a destination the seeded forum's graph does not link to the source thread; the twin puts in G's place a destination the seeded forum already links to it, with the same text, so the column differs |
| 9 | Unseen words | class 2 | feature table | The branch for a source word the forum's index and ganit outputs have never seen, which no text view maps to a known word | Output property: row U's column for the source word is the unseen-word value the member's entry states, no row is dropped and every value is finite. With class 2's input the view maps the word to a known one, which gives another value |

- A class applies to a member only when the member's own code, or the
  code of one of its steps, holds a decision point of the class's
  kind; a class never applies to an output kind its entry does not
  name, and is never run where it does not apply.

### Who owns what

- `sutradhar-methods` owns the class list types, their admission rules,
  the variant layout and the code that turns a `V-` id back into its
  owner and class, beside its chain code (spec 0067); every reader uses
  that one reader.
- The class set builder and the branch record instrumentation sit in
  spec 0064's pinned check harness, outside the release candidate's
  code.
- `qualify` owns the fold and the count, using the `sutradhar-methods`
  reader.

### Which classes a member claims

1. A method's catalogue entry, and a shape's claim entry (for every
   chain of the shape), list the classes the member claims, from the
   class list of its input kind. A shape's claim entry is a separate
   reviewed entry in `catalogue/claims/`, beside the frozen shape sheet
   and pinned by digest in the protected policy; it is admitted and
   re-admitted with an independent review, signed by someone who is not
   an author of the claim entry or of the code holding its named points,
   and bound to the claim entry's digest; admission refuses a review
   whose digest differs. Changing it changes no shape, no chain id and
   no `V-` id. For each claimed class it names the decision points in
   the member's code, each by its Rust path, and the values the class's
   output property reads, such as the no-spread value. The claim is part
   of the reviewed entry; the run checks only claimed classes, and only
   for chains to which the class applies. The review shows each named
   point is of its class's kind, and admission refuses an entry that
   names one decision point for two classes.
2. A shape's claim entry names decision points per slot row, lists for
   each claimed class the rows that may claim it, and bars rows for it. A
   chain claims a class only when its own steps include a listed row and
   no barred row; its named points for the class are those in its own
   steps, and points in rows it does not hold are never asked of it. A
   row whose step can break a class's output property on correct
   input, such as a step with random starts that may score two equal
   rows apart (class 1), or a non-linear fuser whose fit may ignore the
   prefix caution (class 4), is barred for that class, and the review
   gives the reason for each bar.
3. A chain is its own owning behaviour (spec 0067), so a chain and
   each of its steps may own the same class, each by its own checks on
   its own output.

### When a variant counts

A variant of owner m and class k counts in a release run when all of
these hold:

1. m counts in the run, after spec 0067's removals ("Proving chains
   differ", items 3 to 5). A chain removed there owns no variant, and
   its variants never pass to the member it matched.
2. k is admitted, not excluded, claimed by m, and applies to m.
3. The class check: m ran on class k's batch in the qualified run, in
   the sealed runner jobs of spec 0064; its runner-signed output passed
   k's output property, and every decision point named for m and k (for
   a chain, those in its own steps) is in the branch record.
4. The neighbour check: m ran on the twin in the same run and ended
   with a runner-signed result and output; k's output property, judged
   on that output alone, is false; and at least one decision point
   named for m and k that the batch reached is not in the twin's branch
   record. A twin run that ends without a signed output, a
   resource limit included, is a test failure in the cell of m, k and
   that target (spec 0064), never a passing neighbour check.
5. Both results are filed under the owner's identity, with k named
   through the check identity, and carry the run, member, check,
   target, outcome and evidence digest that spec 0064 signs for every
   result. Both checks pass on every required target.

- If k's output property holds on any twin result, m treats k as its
  neighbour and k is not a variant for m. The result is recorded as
  "same as neighbour", is not a test failure and counts nothing. A
  twin whose property is false but whose branch record holds every
  point the batch reached is recorded as "same path on twin", is not
  a test failure and counts nothing. The neighbour
  keeps its own rule, so a class folding into the ordinary batch or
  into an earlier class never takes that class with it.
- If the class check fails on a claimed class, it is a test failure in
  the cell of m, k and that target, as spec 0064 sets, and a later
  pass cannot hide it. As spec 0064 sets, a release run with a test
  failure is refused until the claim is withdrawn from the reviewed
  entry or the code is fixed and the run repeated.
- Reruns of one check, two check ids for one owner and class, and
  changed settings, weights or seeds earn nothing more (spec 0064): the
  variant is one pair, whatever the number of results. `qualify` writes
  one row per result to scratch space and sorts the rows on disk by
  owner, class, target and record order, the order of results in the
  run's signed record (spec 0064); one pass over the sorted rows folds
  them into one record per owner and class, as the rules above decide,
  keeping the first passing class result and the first twin result for
  each target.

### Variant identity

1. A variant's member identity is `V-` and an ordinal, written as spec
   0064 writes member ordinals, worked out from the owner's identity
   and the class index. Ids are never stored one by one; every reader
   works the same id out and turns it back into the same owner and
   class.
2. The variant layout is frozen once admitted, with bounds chosen large
   enough for every later member and class: the class stride K, the
   most classes any input kind's list may ever hold; the method bound
   Nm; the shape bound S; the chain index bound I, the most chains any
   shape may ever have; the capability bound Nc; the workflow shape
   bound W; and the workflow index bound J, the most workflows any
   workflow shape may ever have (spec 0070). The capability and
   workflow ranges are reserved in the layout, so a later class list
   for them changes no id. A catalogue whose members or classes would
   pass a bound is refused at admission, so the layout never changes
   and a `V-` id always names the same owner and class. The protected
   policy pins the layout with the catalogue.
3. The owner's base, from 0, is n − 1 for `M-n`;
   Nm + (s − 1) × I + (i − 1) for `MC-s-i`; Nm + S × I + (n − 1) for
   `CAP-n`; and Nm + S × I + Nc + (w − 1) × J + (j − 1) for `CW-w-j`.
   The ordinal is base × K + k.
   All of it uses checked unsigned 128-bit arithmetic, and a layout
   whose largest ordinal would overflow is refused when it is admitted.
4. Reading an id: k is ((ordinal − 1) mod K) + 1 and the base is
   (ordinal − 1) div K. An id whose base falls outside every range,
   whose owner names no member of the pinned catalogue, or whose k names
   no admitted, unexcluded class claimed by that owner is refused as a
   bad value.
5. Variants sort by ordinal as a number, so all of one owner's variants
   sit together, in class order. In a group, variant records sit after
   capability workflow records (spec 0064). The fixture catalogue has
   its own layout; with K = 1, `FX-V-n`'s owner is `FX-M-n`, as spec
   0064's fixture table has it.

### Counting by sorting

1. `qualify` writes one record for each variant whose class check and
   neighbour check passed, holding its ordinal and the digests of its
   class result and its twin result for every required target, to the
   run's scratch space (spec 0064). It sorts them
   on disk by ordinal and reads the list once, refusing two records with
   one ordinal.
2. It writes the bases of the run's counted members, after spec 0067's
   removals, sorted by base. Because the variant list is in ordinal
   order, it is also in base order, so one merged pass over the two
   lists keeps exactly the variants whose owner counts.
3. The variant total is the number of records kept, added with checked
   unsigned 128-bit arithmetic. A replay rebuilds both lists from the
   runner-signed results, sorts them again and refuses on any
   difference (spec 0067, "Proving chains differ", item 6).
4. No product of member counts and class counts, estimate, sample or
   formula supplies a count. A class list's length bounds the ids an
   owner may have; it never says how many it has.

### The paper bound for spec 0069

With every row of spec 0069 admitted, its shape has 5,805,676,800 chain
ids. Every chain writes an ordering, so at most classes 1 to 4 apply.
With those four claimed by every chain, as plain arithmetic,
5,805,676,800 × 4 = 23,222,707,200 variant ids, above the floor in
AGENTS.md section 3, item 5. That is a bound on ids, never a count:

- spec 0069 expects many of its chains to be removed as behaving the
  same, and a removed chain owns no variant;
- many classes fold into their neighbour for many chains: a chain
  whose order key reads no reranker score takes the neighbour's path on
  class 1, a chain whose word steps read no text view takes it on class
  2, and a chain that holds a barred row never claims that class;
- spec 0069's own Performance section expects a run over the whole
  shape to be refused before it starts, so its variants count only
  from runs that fit.

Only `qualify` supplies the variant total, from signed results.

## Limits

- The branch record shows which named decision points were reached; a
  decision point the entry does not name is not seen. The output
  property, the twin's branch record and the independent review stand
  against a member that reaches a named point without taking its
  decision.
- A class list covers one input kind. Class lists for other input
  kinds, and for capabilities and capability workflows, come in later
  specs under these rules.
- A variant never counts towards the method or capability totals, and
  a member never counts towards the variant total by itself.

## Performance

- Class checks share the chain runs of spec 0067. The class set's
  batches run in the same sealed job as each chain's probe batches, in
  index order, so chains that share a prefix share its outputs on class
  batches too (spec 0067, "Performance"). Class batches use the probe
  set's fits and ganit outputs, so no class trains anything.
- Each claimed class adds two batch runs per owner: its batch and its
  twin. The class check itself reads only the result and branch record.
- `qualify` refuses a run before it starts when the measured cost of
  the class batches and twins, times the claimed classes of every
  owner, plus the sort of the result rows, the fold, the sort of the
  variant records, the merged pass and the replay, would pass spec
  0064's execution time limit, or when the result rows, the variant
  records and the sorted member bases would pass its scratch-space
  limit. Both use the counts known from the catalogue and
  checked arithmetic, and refuse on overflow.
- The protected policy bounds each class list's length, by K, and the
  rows of each class batch. The run waits in Dell's one heavy-job queue
  (AGENTS.md section 11).

## How it is tested

Rust tests in `sutradhar-methods` and `qualify`, with made-up class
lists and forums:

- `class_list_admits_reviewed_class` and
  `class_reviewed_by_its_author_is_refused`.
- `class_differing_only_by_value_is_refused`: two bands of one length
  are refused, and a long-thread class tied to a branch is admitted.
- `class_whose_property_is_decided_by_its_twin_input_is_refused`: an
  output property that holds, or fails, on its twin whatever the output
  is refused.
- `class_review_for_another_entry_digest_is_refused`.
- `twin_judged_on_output_alone`: a member whose output is the same on
  batch and twin owns no variant, whatever its branch record shows.
- `class_with_later_neighbour_is_refused` and
  `admitted_class_is_never_edited`.
- `excluded_class_owns_no_variant`, and an id naming it is refused.
- `class_set_leaves_digests_unchanged`: adding a class changes no
  behaviour, order or coarse digest.
- `variant_counts_when_check_passes_and_twin_fails`.
- `twin_exit_is_test_failure_not_variant`: a twin run with no signed
  output earns nothing and is a failure in its cell.
- `class_same_as_neighbour_counts_nothing`: a member whose output
  passes the property on the twin owns no variant, and the ordinary
  batch is not affected.
- `class_does_not_apply_without_its_decision_point`: a chain with no
  text step is never run on class 2 and fails nothing.
- `failed_claimed_class_is_a_test_failure`, which refuses the release
  run until the claim is withdrawn or the code fixed.
- `removed_chain_owns_no_variant`, and its variants do not pass to the
  member it matched.
- `unclaimed_class_is_not_run` and
  `class_for_another_output_kind_is_not_run`.
- `branch_record_from_member_output_is_ignored`: a member that writes
  a decision point into its own output gains nothing, and the harness
  instruments only the pinned build.
- `tie_class_twin_breaks_the_tie`: the twin of class 1 has no tie, and
  a member that keeps input order without the tie-break by id fails
  class 1's batch, since the batch lists B first.
- `long_class_twin_makes_rows_equal`: class 3's twin makes the two rows
  equal.
- `claim_review_for_another_entry_digest_is_refused`, and a claim
  review signed by an author of the entry is refused.
- `withdrawn_chain_claim_keeps_chain_ids`: re-admitting a shape's
  claim entry without a class changes no shape, chain or `V-` id.
- `chain_with_barred_row_never_claims_class`: a chain holding a listed
  row and a barred row for class 4 is never run on it.
- `chain_checked_only_on_points_in_its_steps`: a chain is never asked
  for points in slot rows it does not hold, and a chain whose steps
  hold no row the claim entry lists for a class does not claim it.
- `equal_rows_share_decisions_and_graph_links`: rows a class calls
  equal get the same seeded decisions and graph links.
- `twin_reaching_every_named_point_is_same_path`: recorded as "same
  path on twin", not a failure, counting nothing.
- `fold_is_per_target_and_any_passing_twin_means_same_as_neighbour`.
- `rerun_and_second_check_id_give_one_variant`, and the record keeps
  the first passing results in record order.
- `variant_id_round_trips` for each owner kind, with an ordinal of 0, a
  base outside every range, an unclaimed class, an excluded class, a
  padded ordinal and a layout that overflows all refused.
- `catalogue_past_a_layout_bound_is_refused`, so a `V-` id names the
  same owner and class in every catalogue.
- `fixture_layout_matches_0064_table`: with K = 1, `FX-V-000001`'s
  owner is `FX-M-000001`.
- `duplicate_variant_ordinal_is_refused` and
  `replay_with_one_record_changed_is_refused`.
- `count_comes_only_from_signed_results`: a shape with four claimed
  classes that apply to it and no results counts no variant.
- `one_decision_point_for_two_classes_is_refused`.
- `edited_branch_record_is_refused`: a replay that reads a twin's
  branch record with one point removed refuses the run.
- `run_refused_when_class_batches_pass_time_limit` and
  `run_refused_when_variant_records_pass_scratch_limit`, before any
  step runs.
- `class_batches_use_probe_set_fits` and
  `single_row_class_batch_breaks_no_probe_rule`.

## Changes from the source

Written fresh from specs 0055, 0064, 0067, 0068, 0069 and 0070. Spec
0064 said a variant's owner is always a method; this spec lets the
owner also be a method chain, and leaves capabilities and capability
workflows to later class lists, and spec 0064 is amended to match. Spec
0067's Limits leave checked variants to their owning specs; this spec
is that home for chains.
