# ADR-0019: Rust for operations and developer tools; no Go, no shell

Decided by the owner on 17 November 2026, word for word: "go has got
to go, ban it." (task #166, comment 5043). It replaces the Go part of
[ADR-0017](0017-go-for-tools.md). The ban on hand-written shell from
ADR-0017 stays.

## Decision

- Operations and developer tools are written in Rust, with their
  tests, like the product.
  [AGENTS.md section 4](../../AGENTS.md#4-languages-and-files) lists
  them.
- Go is banned for our code. The one Go tool of ours that AGENTS.md
  section 4 keeps on Dell may keep running, mended only for defects,
  until its Rust replacement passes every check and review; then it
  is removed.
- Third-party programs written in Go that we only run, such as
  Forgejo, grype and gitleaks, are not our code and are not affected.
- No hand-written shell, as ADR-0017 set out. The kept shell files and
  shell steps are replaced by small Rust tools, one at a time.

AGENTS.md section 4 states the rule.

## Why

- When this was decided, `main` held no Go, the Rust gate on `main`
  worked, and Rust builds one small program for each tool, as Go
  would.
- One language for the product and the tools means one toolchain to
  pin, one set of checks and one kind of review.
- The Go gate had grown to about 100,000 lines in a side copy. Under
  the owner's rule that a commit changes at most 300 lines (task #166,
  comment 5041), it would take about 350 commits to land.

## Trade-offs

- The ideas the Go gate's reviews found must be written again as
  small Rust tasks. The side copy is kept on Dell, unchanged, as a
  reference only.
- The kept Go tool stays until its Rust replacement passes, so for a
  while there are two kinds of tool to watch.

## Alternatives not chosen

- Keep Go for tools, as ADR-0017 chose: ended by the owner.
- Land the Go gate first and port it later: twice the work, and about
  350 commits of Go that would then be removed.
