---
name: backlog
description: >-
  Required backlog review of every change. Checks that it does what its task
  asks and no more, fits the roadmap, and that new ideas went to Later, then
  writes the review record the gate demands.
---

# Backlog review

Every change gets this review before it is committed (AGENTS.md section
7.1). It cannot be skipped: the gate checks for its record.

## Who reviews

Run this review as a subagent with a fresh context, given the task and
the diff, never the agent that wrote the change (AGENTS.md section 7.1).

## Steps

1. Read the commit's task in Forgejo and the roadmap item it belongs to.
2. Check:
   - the change does what the task asks, and nothing it does not ask,
     such as tidying unrelated code in passing; anything the gate needs
     before it takes this change counts as asked, such as its review
     records, the `Cargo.lock` and SBOM entries its own manifest edits
     need, and the fixes to old code the gate demands (AGENTS.md section
     6 rule 13);
   - the task is in the current phase, or the owner moved it there;
   - new ideas found on the way are tasks in Later, not in this change;
   - work that belongs to a later roadmap item is a task on that item;
   - the task's proof will be posted on the task at the end of the work;
   - at most 10 tasks are marked next (AGENTS.md section 10).
3. Report each problem with its place. The agent that wrote the change
   fixes it, or says why it is not one, and this review runs again on
   the new content. Pass only when nothing is left open.

## Writing the record

The gate refuses the commit, and the door the push, without a passing
record from this skill for this exact change.

1. Stage the whole change first: `git add` every file.
2. Run `cargo run --quiet --package aegis -- fingerprint`. It prints the
   fingerprint of the staged content, such as `sha256:3f...`.
3. Write `reviews/backlog.toml`:

   ```toml
   skill = "backlog"
   change = "sha256:..."   # exactly what step 2 printed
   verdict = "pass"        # or "fail"
   agent = "claude"        # who reviewed: claude, codex, gemini
   summary = "What you read and what you checked, in a sentence or two."

   [[finding]]             # one per problem found; repeat as needed
   what = "What was wrong, and where."
   fixed = true            # false blocks the commit
   ```

4. Stage the record. If any file changes after this, the fingerprint
   changes and the review must be done again. An amend or a merge
   keeps its review as long as the content is what was reviewed.

Never write `pass` for a review you did not do. A record is a statement
in your name that you did the steps above.
