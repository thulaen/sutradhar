# ADR-0001: Languages and files

Decided by the owner on 8 November 2026.

## Decision

Rust is the language for everything we write. Beside it, CSS (with
Tailwind), strict TypeScript for small interop modules and PHP for the
two site add-ons are allowed, with a few other kinds of file. The full
list of allowed languages, versions and file types is in
[AGENTS.md section 4](../../AGENTS.md#4-languages-and-files). Cargo is
the only build tool. Ready-made tool programs may run only when pinned
and listed in `gate/tools.toml`.

## Why

- With one main language there is one set of checks, one style and one
  way to test, so every check can really reach 100%.
- Rust is memory-safe and fast without a garbage collector, which helps
  keep within the memory limit.
- PHP is needed only because XenForo and WordPress run PHP.

## Trade-offs

- Rust has fewer ready-made libraries for some tasks, so we write more
  ourselves.
- TypeScript waits for the condition in
  [AGENTS.md section 4](../../AGENTS.md#4-languages-and-files).

## How we check it

The language check refuses any file whose type is not on the allowed
list, at commit, push and release. It is proven on a planted file of each
refused kind.
