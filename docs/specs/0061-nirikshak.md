# 0061: nirikshak, the code analyser

nirikshak ("inspector") reads facts about the code and applies rules
that no single language's own tools can see. It is roadmap Phase 2
item 2 (task #68). The gate runs it as the check `analysis.nirikshak`
at commit and push, over every Rust file and Markdown page in the
project. A run over the whole project takes about a second.

## How it works

1. **Facts.** `rust_facts` reads each Rust file with `syn`, a pure Rust
   parser, and records facts about it (below). `markdown_facts`
   records each inline code span in a page, outside fenced blocks. A
   call is named through the file's `use` lines, so `mpsc::channel`
   reads as
   `std::sync::mpsc::channel`.
2. **Rules.** Each rule is a Rego file in `crates/nirikshak/rules`,
   built into the program. Rego is one of the allowed policy languages
   (ADR-0001). The rules run in `regorus`, a pure Rust Rego engine.
3. **Report.** Each rule's set `findings` holds a path, a line and what
   was found. The gate shows each with the rule's title, why it matters
   and how to fix it.

Every rule that runs blocks. A rule not ready to block is left out, not
run as a warning. A file `syn` cannot read, a rule that cannot run, or a
finding of the wrong shape blocks the run: it is never taken as a pass.

## Facts

Each fact is a JSON object. Paths are relative to the project's top
folder; lines start at 1. Code outside any function is in the function
`<top>`.

| Relation | Fields |
|---|---|
| `file` | `path`, `language` |
| `function` | `path`, `name`, `first`, `last`, `kind` |
| `call` | `path`, `function`, `callee`, `line` |
| `call_text` | `path`, `function`, `callee`, `text`, `line` (a call whose first argument is a string) |
| `macro` | `path`, `function`, `name`, `line` |
| `macro_text` | `path`, `function`, `name`, `text`, `line` (a macro whose first argument is a string) |
| `doc_code` | `path`, `text`, `line` (an inline code span in a page) |
| `discard` | `path`, `function`, `line`, `kind` (`let _ =` or `.ok();`) |
| `empty_handler` | `path`, `function`, `line` (an `Err(_) => {}` arm) |
| `uses_try` | `path`, `function`, `line` (a `?`) |
| `parse_failure` | `path`, `line`, `reason` |

A function's `kind` is `test` (a `#[test]`), `test_panics` (a test that
expects a panic), `test_support` (other code in a test module, a test
folder or a `_tests.rs` file) or `code`. A method call's `callee` is
`.name`, such as `.unwrap`.

## Rules

Each rule file starts with comment lines naming its `id`, `title`,
`why` and `fix`. Its package is `nirikshak.` and the id in lower case.

| Rule | Finds | Gate items |
|---|---|---|
| R05 | A setting read in code (`std::env::var`, `env!` and the like) that no page names in a code span; Cargo's own and test code are left out | GI-48 |
| R08 | A network call with no time limit, in code | GI-15 |
| R09 | An error thrown away, in code: `let _ =` on a call, `.ok();`, or an empty `Err(_)` arm | GI-47 |
| R14 | A test that checks nothing, following its helpers across files | GI-32 |
| R18 | A queue or channel with no limit, in code | GI-36 |
| R19 | SQL built from strings with `format!` and the like | GI-37 |
| S01 | A command run through a shell, in code | GI-37 |
| S02 | Certificate checks switched off, anywhere | GI-18 |
| S03 | MD5 or SHA-1, anywhere | GI-18 |

Each has tests that a planted case is found and a clean one passes.

## Library choice

`regorus` 0.10.1 is pinned exactly. Later releases need a Verus support
library that brings a second copy of `syn`, which `deny.toml` refuses.
All rules load into one engine, so the facts are handed over once. The
workspace builds `regorus` and `syn` optimised even in debug builds,
since the gate runs a debug build.

## Limits

- Only Rust is read so far. Other languages get facts when a rule needs
  them.
- R05 sees `std::env::var` and its kin, and `env!`. A setting read
  through a helper of our own, such as `setting("NAME")`, is not seen
  yet, so every program's page lists its settings by hand as well.
- R14 matches helpers by name alone, so a helper of the same name
  elsewhere that asserts can hide a finding.
- The earlier work's other rules come over one by one, each with its
  tests, when it can block honestly. Each rule still needs its own
  measured false-alarm rate before release.
