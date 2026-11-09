---
name: defects
description: >-
  Required review of every change before it is committed. Reads every changed
  line for defects, unclear code and missing tests, then writes the review
  record the gate demands.
---

# Code review

Every change gets this review before it is committed (AGENTS.md section
7.1). It cannot be skipped: the gate checks for its record.

## Who reviews

Run this review as a subagent with a fresh context, given the task and
the diff, never the agent that wrote the change (AGENTS.md section 7.1).

## Steps

1. Read the task, then every changed line, in full. Read the code each
   change calls and the code that calls it.
2. Look for:
   - wrong results, off-by-one, missed cases, panics and unhandled errors;
   - behaviour the task did not ask for, or that the docs do not match;
   - a test that would still pass if the change were wrong;
   - names, comments and docs that mislead;
   - code that repeats code elsewhere, or does two jobs in one place.
3. Report each problem with its place. The agent that wrote the change
   fixes it, or says why it is not one, and this review runs again on
   the new content.
4. Pass only when nothing is left open.

## Writing the record

The gate refuses the commit, and the door the push, without a passing
record from this skill for this exact change.

1. Stage the whole change first: `git add` every file.
2. Run `cargo run --quiet --package aegis -- fingerprint`. It prints the
   fingerprint of the staged content, such as `sha256:3f...`.
3. Write `reviews/defects.toml`:

   ```toml
   skill = "defects"
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
