# 0009: Source class inputs

The source limits check (0007) reads a class list that says which
tracked paths are first-party text, data, lock files, binary or third
party. This part bounds how much of that class list the check will read,
so a huge or growing file cannot exhaust memory. It is roadmap Phase 2
item 1, as part of the gate code brought over from the archive.

## What it does

### The setting

1. The class list names text, data, lock-file, binary and external
   paths.
2. The check reads at most a set number of bytes of it. The setting is
   positive and defaults to 262,144 bytes (256 KiB).
3. `--class-max-bytes BYTES` sets the allowance. It sits in the option
   prefix after ROOT and before the listed source paths, and may come
   before or after `--classes FILE`.
4. Existing documented commands and public library calls keep working
   unchanged.

### Refused options

1. A repeated `--classes` or `--class-max-bytes` option is refused.
2. A missing value is refused. A known option name never stands in for
   another option's missing value.
3. A byte allowance that is not decimal, is zero or overflows is
   refused.
4. There is no unlimited mode.
5. Source paths after the prefix keep their earlier meaning.

### Reading

1. The real bytes of the class list are read through the shared bounded
   reader, at most the allowance plus one byte. The extra byte only
   detects excess.
2. A file of exactly the allowance is accepted. One extra byte, a short
   read or a malformed rule blocks the command before any source file is
   checked.
3. Accepted rule order and matching, class identity and the report
   shape stay as they are.
4. Only the whole bounded bytes are hashed, never a cut-off prefix.
   A fully read but malformed class list may keep its full byte hash
   while the command stays blocked. A hash never makes a file valid.
5. Transport failures still return the existing output-failure status.

### What does not change

1. The listed-source maximum stays at 4,000,000 bytes.
2. The source limits, file classes and language rules are unchanged.
3. A larger real class list needs a recorded setting change and a
   resource check first.
4. Command-line input alone gives no publishing power. Protected input
   identities must bind the real settings and tools.

## Limits

- Class list: 262,144 bytes by default; any larger value is set on
  purpose and checked.
- Listed sources: 4,000,000 bytes.
- The byte cap alone does not prove a deadline, a stable namespace,
  bounded parser growth, caller isolation or memory recovery. Those
  come from the runner's box limits (0003) and the heavy-job limits of
  AGENTS.md section 11.
- These are protection values, not measured workload coverage.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Rust tests in aegis, written before the code and run against the old
behaviour first:

- With the default, a 262,144-byte class list is read; a 262,145-byte
  one is refused before any listed source is checked.
- The same at exact and one-over sizes for explicit limits, and for
  larger explicit settings.
- Both option orders work.
- Bad, missing and repeated options are refused, including an option
  name given where a value belongs.
- The hash covers the whole bytes read.
- Malformed rules block, and no source checking follows a refusal.
- Reader cases cover growing and trickling files, partial errors and
  the system's own error messages kept intact.
- Each new test runs 20 times fresh, and every check of the tiers runs
  on the change.

## Changes from the source

- The checker is the Rust source limits check in aegis.
- The class list's place is the protected classification file of 0007.
- The old 200-fold speed goal is dropped; speed follows the 5% median
  rule of AGENTS.md section 6.

Source: `docs/specifications/source-class-inputs.md` in the archive
`archive-2026-11-08-old-repo`.
