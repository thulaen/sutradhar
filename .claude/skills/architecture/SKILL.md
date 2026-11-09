---
name: architecture
description: >-
  Required architecture review of every change. Checks module walls, the
  composition root, events and data flow against AGENTS.md section 5, then
  writes the review record the gate demands.
---

# Architecture review

Every change gets this review before it is committed (AGENTS.md section
7.1). It cannot be skipped: the gate checks for its record.

Sutradhar Boot is our own Rust framework in the spirit of Spring Boot,
and it is opinionated: there is one right way to do each common thing
(AGENTS.md section 5.1). This review holds every change to those
conventions. A change that does it another way fails, even if it works.

## Who reviews

Run this review as a subagent with a fresh context, given the task and
the diff, never the agent that wrote the change (AGENTS.md section 7.1).

## Steps

1. Find which modules the change touches, and what it adds to each.
2. Hold it to Boot's conventions, as Spring Boot would:
   - **Convention over configuration.** Use Boot's defaults; set a
     setting only to change a default, never to repeat it.
   - **One composition root.** Every part is wired there, with typed,
     explicit injection through constructors. No globals, singletons,
     service locators, `lazy_static`-style state or hidden wiring.
   - **Starters, not hand-made plumbing.** Web, database, jobs, events,
     health, security and observability come from Boot's starters.
     Rolling your own of any of these fails.
   - **Typed settings bound once.** Each module reads one typed settings
     struct with safe defaults and the `dev`, `test` and `prod`
     profiles. No reading environment variables or files elsewhere.
   - **Layers kept apart.** A web handler only parses, calls one service
     and shapes the reply. Rules live in services. Only the module's own
     store talks to the database, through sqlx with checked queries.
   - **Package by feature.** A module holds everything for one purpose
     and shows one public API. Nothing reaches into another module's
     insides; there are no loops between modules (the Modulith rule).
   - **Events, not calls, for reactions.** A change other modules must
     respond to is a typed event saved in the outbox in the same
     transaction. Consumers handle repeats safely.
   - **One error shape.** Every error is Boot's error type, with a
     plain-English message; nothing is swallowed.
   - **Health, readiness, metrics and audit** come from Boot for every
     part, as Spring Boot's Actuator gives them.
   - **Lifecycle.** Start-up is in a fixed order through Boot's hooks;
     every part shuts down gracefully.
   - **One library per job** (`gate/packages.toml`); Axum, Tokio,
     Tower, Tracing, Serde and sqlx where they fit.
3. Code made before Boot exists (the gate, forge-sync, tijori's signing
   core) is held to the same rules where they apply, and noted for its
   move onto Boot in Phase 3.
4. Look for a simpler design that does the same job with less code.
5. Report each problem with its place. The agent that wrote the change
   fixes it, or says why it is not one, and this review runs again on
   the new content. Pass only when nothing is left open.

## Writing the record

The gate refuses the commit, and the door the push, without a passing
record from this skill for this exact change.

1. Stage the whole change first: `git add` every file.
2. Run `cargo run --quiet --package aegis -- fingerprint`. It prints the
   fingerprint of the staged content, such as `sha256:3f...`.
3. Write `reviews/architecture.toml`:

   ```toml
   skill = "architecture"
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
