---
name: red-team
description: >-
  Required attack on every change by red agents, run as subagents that each
  try to break it a different way, then writes the review record the gate
  demands.
---

# Red team

Every change is attacked before it is committed, by red agents: helpers
whose only job is to break it (AGENTS.md section 7.1). It cannot be
skipped: the gate checks for its record.

## Who reviews

Run this review as a subagent with a fresh context, given the task and
the diff, never the agent that wrote the change (AGENTS.md section 7.1).

## Steps

1. Start at least three subagents at once (Claude's Agent tool, or your
   own tool's way of starting a helper with a fresh context; with none,
   run each goal as its own fresh session), each with the change's diff
   and one goal, and nothing of your own view of it:
   - **Break it.** Find inputs that give a wrong result, a crash, a hang
     or runaway memory: empty, huge, odd Unicode, malformed, repeated.
   - **Get round it.** For a check or a rule: write code that does the
     forbidden thing and still passes. For other code: get past its
     limits and permissions.
   - **Abuse it.** Act as an outsider who controls a post, a page, a
     request or a file, and make the change leak, run or change
     something it should not.
2. Each red agent reports concrete inputs and what happened, under 300
   words. Ask it to try what it claims where it can.
3. Check each report yourself. Report each real hole to the agent that
   wrote the change, which fixes it and adds a test that would have
   caught it; then the red team runs again on the new content.
4. Pass only when every real hole is fixed. Name the red agents' goals
   in the summary.

## Writing the record

The gate refuses the commit, and the door the push, without a passing
record from this skill for this exact change.

1. Stage the whole change first: `git add` every file.
2. Run `cargo run --quiet --package aegis -- fingerprint`. It prints the
   fingerprint of the staged content, such as `sha256:3f...`.
3. Write `reviews/red-team.toml`:

   ```toml
   skill = "red-team"
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
