# 0061: nirikshak, the code analyser

nirikshak ("inspector") reads facts about the code and applies rules
that no single language's own tools can see. It is roadmap Phase 2
item 2 (task #68). The gate runs it as the check `analysis.nirikshak`
at commit and push, over every Rust file and Markdown page in the
project. A run over the whole project takes about a second.

## How it works

1. **Facts.** `rust_facts` reads each Rust file with `syn`, a pure Rust
   parser, and records facts about it (below). `markdown_facts`
   records each inline code span in a page, outside fenced blocks.
   `yaml_facts` records each setting a YAML file declares under
   `environment:` or `env:`, or reads as `${NAME}`. A
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
| `call_text` | `path`, `function`, `callee`, `text`, `line` (a call whose first argument is a string or a whole number) |
| `macro` | `path`, `function`, `name`, `line` |
| `macro_text` | `path`, `function`, `name`, `text`, `line` (a macro whose first argument is a string) |
| `doc_code` | `path`, `text`, `line` (an inline code span in a page) |
| `literal` | `path`, `function`, `text`, `line` (a string, macros included) |
| `method_receiver` | `path`, `function`, `method`, `receiver`, `line` (what a method is called on, such as `self.cache`) |
| `range_cut` | `path`, `function`, `receiver`, `line` (a slice or `get` from the start, such as `body[..200]`) |
| `impl_fn` | `path`, `function`, `trait`, `line` (a method of a trait impl) |
| `discard` | `path`, `function`, `line`, `kind` (`let _ =` or `.ok();`) |
| `empty_handler` | `path`, `function`, `line` (an `Err(_) => {}` arm) |
| `uses_try` | `path`, `function`, `line` (a `?`) |
| `env_declared` | `path`, `name`, `line` (a setting a YAML file declares or reads) |
| `container` | `path`, `name`, `line`, `image`, `user`, `privileged`, `network_mode`, `read_only`, `mem_limit`, `pids_limit`, `no_new_privileges`, `cap_drop_all` (one service of a compose file) |
| `container_env` | `path`, `service`, `name`, `written`, `line` (a setting a compose service sets; `written` when its value is in the file) |
| `docker_stage` | `path`, `line`, `image`, `user`, `alias`, `last` (one `FROM` stage of a Dockerfile, its `AS` name and its last `USER`) |
| `signature` | `path`, `function`, `kind`, `line`, `inputs`, `bools` (one function's inputs, and how many are `bool`; trait methods with no body included) |
| `number` | `path`, `function`, `kind`, `text`, `line` (a number written in a function, outside a `const`, `static`, type or attribute) |
| `chain` | `path`, `function`, `kind`, `length`, `line` (the outer call of a chain of method calls, counted through `?`, `.await` and brackets, and how many calls it holds) |
| `shape` | `path`, `function`, `kind`, `name`, `what`, `size`, `line` (a struct and its fields, an enum and its variants, or a trait and its items) |
| `comment` | `path`, `kind`, `text`, `line` (a comment on a line of its own that is not a doc comment; `test` in a test file, or inside a test module or test function) |
| `fn_ref` | `path`, `function`, `kind`, `name`, `line` (the last name of a path used as a value inside a function, such as `.any(is_test_attr)`, or any name in a macro's input; a name bound in the same function, such as a local variable, is left out) |
| `repeated` | `path`, `kind`, `text`, `copies`, `line` (each copy of text written three or more times in a file's code) |
| `parse_failure` | `path`, `line`, `reason` (also a compose file that cannot be read, or uses YAML aliases, which a few lines can grow into a huge document) |

A function's `kind` is `test` (a `#[test]`), `test_panics` (a test that
expects a panic), `test_support` (other code in a test module, a test
folder or a `_tests.rs` file) or `code`. A method call's `callee` is
`.name`, such as `.unwrap`.

## Rules

Each rule file starts with comment lines naming its `id`, `title`,
`why` and `fix`. Its package is `nirikshak.` and the id in lower case.

| Rule | Finds | Gate items |
|---|---|---|
| R05 | A setting read in code (`std::env::var`, `env!` and the like), or declared in a YAML file, that no page names in a code span; Cargo's own and test code are left out | GI-48 |
| R08 | A network call with no time limit, in code | GI-15 |
| R09 | An error thrown away, in code: `let _ =` on a call, `.ok();`, or an empty `Err(_)` arm | GI-47 |
| R14 | A test that checks nothing, following its helpers across files | GI-32 |
| R18 | A queue or channel with no limit, in code | GI-36 |
| R19 | SQL built from strings with `format!` and the like | GI-37 |
| S01 | A command run through a shell, in code | GI-37 |
| S02 | Certificate checks switched off, anywhere | GI-18 |
| S03 | MD5 or SHA-1, anywhere | GI-18 |
| T01 | Text cut short, in code: `truncate`, `split_off`, `take` or `drain`, or a slice or `get` from the start, on anything named like content (text, body, post, page, message, title, chunk, token, word and the like). The owner's rule is no truncation anywhere (task #77). Refusing a whole message with an error is allowed. | GI-06 |
| C01 | A container image not pinned by digest (compose or `FROM`; a `FROM` naming an earlier stage's `AS` name is not an image) | GI-48 |
| C02 | A container that runs as root (an explicit root user, or a Dockerfile whose last stage sets no `USER`) | GI-18 |
| C03 | A privileged container | GI-18 |
| C04 | A container on the host network | GI-18 |
| C05 | A container with no memory or process limit | GI-20 |
| C06 | A container that can write its own files (not `read_only`) | GI-18 |
| C07 | A container without `no-new-privileges` | GI-18 |
| C08 | A container that keeps its rights (no `cap_drop: [ALL]`) | GI-18 |
| C09 | A password or token written into a container's settings, as a map or a `- NAME=value` list, including a `${NAME:-default}` that writes its default | GI-46 |
| H06 | A file mode that lets anyone write, in code | GI-18 |
| H11 | A file written more than once with no buffer | GI-11 |
| H22 | SQL that drops, empties or deletes every row, in code | GI-06 |
| H23 | An atomic value read, then set in a separate step | GI-23 |
| H25 | A long-lived cache filled where nothing ever clears it | GI-25 |
| H35 | A panic inside `drop` | GI-35 |
| H39 | Two or more SQL writes with no transaction around them | GI-39 |
| H42 | A property test with fewer than ten cases | GI-57 |
| H44 | Local clock time in code | GI-44 |
| H53 | Memory leaked on purpose, in code | GI-20 |
| H56 | A drive letter or a `/Users/` folder written into code | GI-56 |
| M01 | A function with a `bool` input, in code: it does two things, and `f(true)` does not say which | GI-28 |
| M02 | A number other than 0, 1 or 2 written in code, outside a `const`, a `static`, a type or an attribute | GI-13 |
| M03 | A chain of more than eight method calls, in code | GI-29 |
| M04 | A struct with more than 12 fields, or an enum with more than 30 variants, in code | GI-51 |
| M05 | Code left in a line comment in code: it ends in `;`, `{` or `}` and holds `(`, `=`, `::`, `->`, `!`, `[` or `&`. Comments are read from the raw lines, so a `//` line inside a string counts | GI-24 |
| M06 | The same text of eight or more characters written three or more times in one file's code, two copies on one line counting as two | GI-28 |
| M07 | A function in code whose name holds `_and_`: it does two jobs | GI-16 |
| M08 | A function in code that no test reaches: no test calls it, names it as a value (in code, or anywhere in a macro's input), or reaches a function that does. Methods of an impl of a trait from outside the project, and `main`, count as reached, as they are called through the trait or by running the program. Names are matched alone, as in R14, so a same-name function, or a name used in a test, counts | GI-58 |

Each has tests that a planted case is found and a clean one passes.

Some smells from task #118 are blocked by the workspace lints, not by
rules here: more than 5 inputs to a function (`too_many_arguments`),
nesting deeper than 4 (`excessive_nesting`), a function over 60 lines
(`too_many_lines`), many `bool` inputs (`fn_params_excessive_bools`),
code that is never used (`dead_code`, under deny warnings), a match
with one real arm and an empty `_` arm (`single_match`), a match with
only `_` (`match_single_binding`), and an empty `Err(_)` arm (R09). An
empty arm for cases that need nothing is allowed: it says so plainly.
A catch-all `_ =>` arm that holds code is not checked:
`wildcard_enum_match_arm` was tried, but it fires on matches over
`syn`'s enums, which are open to new variants and cannot be listed in
full (task #133).

## Library choice

`regorus` 0.10.1 is pinned exactly. Later releases need a Verus support
library that brings a second copy of `syn`, which `deny.toml` refuses.
All rules load into one engine, so the facts are handed over once. The
workspace builds `regorus` and `syn` optimised even in debug builds,
since the gate runs a debug build.

## Limits

- Rust, Markdown, YAML and Dockerfiles are read so far. Workflow files
  are not given container rules: `workflows.sealed` already refuses
  every outside action and container option in them. Systemd unit
  files are written by the install scripts, not kept as files, so none
  are read; a unit kept as a file would need its own rules.
- The other allowed languages get facts when their code arrives and a
  rule needs them.
- R05 sees `std::env::var` and its kin, and `env!`. A setting read
  through a helper of our own, such as `setting("NAME")`, is not seen
  yet, so every program's page lists its settings by hand as well.
- T01 goes by names. A cut on content held under another name is not
  seen, and a variable named like content that is not post text is
  refused; such code is renamed to say what it holds. The reading and
  text-engine crates add their own test, when they are built, that
  every character of a post reaches a piece or a counted skip.
- H39 matches callers by name alone, as R14 matches helpers.
- R14 matches helpers by name alone, so a helper of the same name
  elsewhere that asserts can hide a finding.
- The earlier work's other rules come over one by one, each with its
  tests, when it can block honestly. Each rule still needs its own
  measured false-alarm rate before release.
