---
name: unsafe-patterns
description: >-
  Required check of every change for unsafe code patterns that tools can miss,
  then writes the review record the gate demands.
---

# Unsafe code patterns

The gate's rules (nirikshak, clippy, cargo-deny and the rest) catch many
unsafe patterns. This review catches the ones they cannot see. Every
change gets it before it is committed (AGENTS.md section 7.1). It cannot
be skipped: the gate checks for its record.

## Who reviews

Run this review as a subagent with a fresh context, given the task and
the diff, never the agent that wrote the change (AGENTS.md section 7.1).

## Steps

1. Run the commit checks first, so known patterns are already fixed.
2. Read the change for patterns that hide from tools:
   - an error turned into a default value, or `unwrap_or` on data that
     must be there;
   - an index, length or count that can overflow or go below zero;
   - a lock held across a wait, or two locks taken in different orders;
   - a check of a value followed later by its use, after it may change;
   - a file, socket or child process not closed on every path;
   - a time, size or count taken from input without a limit;
   - code that trusts a path, name or number from outside;
   - a test or proof that was weakened to make it pass.
3. Report each one with its place. The agent that wrote the change fixes
   it, or says why it is safe, and this review runs again on the new
   content. Pass only when nothing is left open.

## Writing the record

The gate refuses the commit, and the door the push, without a passing
record from this skill for this exact change.

1. Stage the whole change first: `git add` every file.
2. Run `cargo run --quiet --package aegis -- fingerprint`. It prints the
   fingerprint of the staged content, such as `sha256:3f...`.
3. Write `reviews/unsafe-patterns.toml`:

   ```toml
   skill = "unsafe-patterns"
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
