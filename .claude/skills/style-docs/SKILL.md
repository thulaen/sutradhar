---
name: style-docs
description: >-
  Required code-style and docs review of every change. Checks the style guide,
  plain English, names and that every doc the change touches is still true,
  then writes the review record the gate demands.
---

# Code style and docs review

Every change gets this review before it is committed (AGENTS.md section
7.1). It cannot be skipped: the gate checks for its record.

## Who reviews

Run this review as a subagent with a fresh context, given the task and
the diff, never the agent that wrote the change (AGENTS.md section 7.1).

## Steps

1. Read every changed line against AGENTS.md section 6 and the guides
   in `docs/standards/`.
2. Check:
   - names say what a thing is; module names follow the glossary and
     `docs/standards/vocabulary.md`;
   - comments say why, and every public item has a doc comment;
   - screens and docs for the owner are in plain English;
   - every doc, spec and setting the change touches is still true, and
     a new setting is explained in a page;
   - no status words in docs (status lives in Forgejo);
   - nothing is said twice in two places that could drift apart.
3. Report each problem with its place. The agent that wrote the change
   fixes it, or says why it is not one, and this review runs again on
   the new content. Pass only when nothing is left open.

## Writing the record

The gate refuses the commit, and the door the push, without a passing
record from this skill for this exact change.

1. Stage the whole change first: `git add` every file.
2. Run `cargo run --quiet --package aegis -- fingerprint`. It prints the
   fingerprint of the staged content, such as `sha256:3f...`.
3. Write `reviews/style-docs.toml`:

   ```toml
   skill = "style-docs"
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
