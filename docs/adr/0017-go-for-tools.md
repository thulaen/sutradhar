# ADR-0017: Go for operations and developer tools; no shell

Decided by the owner on 10 November 2026 ("ban shell, use golang" and
"make golang for all Go for operations and developer tools"), with
later answers that set its edges: on 10 November 2026, "Allow it only
for sealed Go race tests (Recommended)"; on 11 November 2026, "Yes,
agents may run programs (Recommended)", "Yes, move them first
(Recommended)" and "Finish Phase 1 in Rust (Recommended)".

## Decision

- Operations and developer tools are written in Go, with their tests.
  [AGENTS.md section 4](../../AGENTS.md#4-languages-and-files) lists
  them and names the one exception that finishes in Rust first.
- The product stays Rust, as AGENTS.md section 4 lists it. Where a tool
  and the product must agree, they share one schema or fixture set, not
  two hand-written copies of a rule.
- No hand-written shell: no shell files, no shell steps in hooks or
  CI, and no `sh -c` or `bash -c` in our programs. Go programs start
  other programs with an explicit list of arguments.
- Agents may still run programs, such as the compilers, the tests and
  the gate, with their own command tool. The ban covers code we write
  and keep, not how an agent runs a program.
- Go builds outside sealed race-test jobs keep cgo off. Only sealed Go
  race-test jobs may use a C compiler, pinned by version and hash,
  because Go's race detector needs one.
- Go source lands only once the Go toolchain and every Go analyser are
  pinned by version and hash and their checks catch planted faults.
  Which Go source may land first, and when Go receipts begin, is set
  in AGENTS.md section 4. No check is weaker for Go.
- The shell files, shell steps in hooks and CI, and Rust tools that
  AGENTS.md section 4 keeps go on working until each Go replacement
  passes every check and review. Then the old one is removed. A live
  guard is never switched off for a replacement that has not passed.

AGENTS.md section 4 states the rule.

## Why

- Shell is easy to get wrong: quoting, word splitting and silent
  errors. Go has types, real error values and a standard library that
  starts programs without a shell.
- Go builds one static program for each tool, quickly, which suits
  hooks and tools that run many times a day.
- Keeping the product in Rust keeps its speed, safety proofs and shared
  types where they matter most.

## Trade-offs

- Two languages to pin, check and review instead of one.
- The existing Rust gate, hooks and tools have to be ported, with their
  tests kept as the reference for what each one must do.
- Rules that both a tool and the product use need one shared source,
  so the two languages cannot drift apart.

## Alternatives not chosen

- Keep small shell scripts: the old rule, ended by the owner.
- Python for tools: needs a runtime on every machine, which section 4
  avoids.
- Rust for every tool: the owner chose Go for tools.
