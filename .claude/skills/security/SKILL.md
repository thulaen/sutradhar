---
name: security
description: >-
  Required security review of every change. Traces untrusted input to where it
  is used, checks secrets, access and crypto, then writes the review record
  the gate demands.
---

# Security review

Every change gets this review before it is committed (AGENTS.md section
7.1). It cannot be skipped: the gate checks for its record.

## Who reviews

Run this review as a subagent with a fresh context, given the task and
the diff, never the agent that wrote the change (AGENTS.md section 7.1).

## Steps

1. List every input the change reads that someone else controls: site
   posts and pages, HTTP requests, files, settings, other programs.
2. Follow each one to where it is used. Look for:
   - SQL, shell commands, paths or HTML built from it;
   - decoding it into anything but a fixed type;
   - outgoing calls to an address it chose;
   - logging it without cleaning;
   - missing checks of who may do this, and row-level security;
   - secrets in code, logs, tests or errors;
   - weak or home-made crypto, and random numbers that must not be
     guessed;
   - libraries added without a need, or from outside crates.io.
3. Report each problem with its place. The agent that wrote the change
   fixes it, or says why it is not one, and this review runs again on
   the new content. Pass only when nothing is left open.

## Writing the record

The gate refuses the commit, and the door the push, without a passing
record from this skill for this exact change.

1. Stage the whole change first: `git add` every file.
2. Run `cargo run --quiet --package aegis -- fingerprint`. It prints the
   fingerprint of the staged content, such as `sha256:3f...`.
3. Write `reviews/security.toml`:

   ```toml
   skill = "security"
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
