# Source limits inventory

Owner: continuing coordinator. Measured: 30 October 2026, on Dell, from
the preserved history of commit `691559a` (tree `3ce87786`), counting
physical lines (blank and comment lines included) and Unicode code points
per line, the rule later specified in SPEC-007.
Status: decisions recorded and applied; see "Owner decisions" below.

## Owner decisions

On 30 October 2026 the owner accepted all six recommended options below
("Accept all six (Recommended)"). They were applied in the compliance
commit that follows the foundation: long single links kept on their own
lines (at most 200 characters), quoted tabs expanded, data converted to
split YAML with a round-trip proof, the SPEC-001 to SPEC-003 reflow
recorded in the approval record, and transcripts split by question range
with their quoted lines unchanged.

## Result

The foundation has 55 tracked files. Three already comply (`LICENSE`,
`catalogue/README.md`, `docs/operations.md`). The others break at least one
of the new limits (R089). The foundation stays an identified historical
commit; compliance is a separate milestone after it.

### Files over 300 lines

| File | Lines | Kind |
| --- | --- | --- |
| `catalogue/candidates.json` | 2,183 | first-party data |
| `catalogue/schema.json` | 698 | first-party data |
| `catalogue/sources.json` | 440 | first-party data |
| `docs/requirements/source-items.csv` | 1,080 | first-party data |
| `sources/interview-001-030.md` | 935 | transcript |
| `sources/interview-031-060.md` | 963 | transcript |
| `sources/interview-061-090.md` | 812 | transcript |
| `sources/interview-091-120.md` | 1,302 | transcript |
| `sources/interview-121-150.md` | 1,128 | transcript |
| `sources/interview-151-180.md` | 822 | transcript |
| `sources/interview-181-209.md` | 903 | transcript |

The `sources/` paths are under `docs/requirements/`.

### Lines over 80 characters

Fifty-two files have such lines: 3,254 lines in total. The widest are
interview transcript lines (up to 1,947 characters), wide Markdown tables
(up to 735) and long JSON strings (up to 365). Tab characters appear in
eleven lines of `source-items.csv` and eleven of `interview-091-120.md`.
Function length does not apply yet: there is no executable code.

## Compliance work that needs no decision

In progress since 30 October 2026, with a locked "before" copy of every
file saved on Dell for word-by-word comparison:

- Rewrite wide Markdown tables as lists or narrower tables, keeping every
  cell's content.
- Reflow prose paragraphs. Markdown joins wrapped lines, so the rendered
  text is unchanged.
- Split long prose documents into coherent linked parts, keeping the
  original path as the entry page.

Not touched until decided: code-block quotations (decision 6), long
single links (1), tabs (2), data files (3) and the approved SPEC-001 to
SPEC-003 files (5).

## Decisions requested

1. Unbreakable tokens. Some lines are one indivisible item longer than 80
   characters, mainly web addresses. Options: (a) allow a line containing
   only one unbreakable link or reference definition, nothing else, with a
   hard upper limit of 200 characters (recommended); (b) no exception,
   remove or replace such links with shorter official addresses where they
   exist.
2. Tabs in quoted owner text. Options: (a) replace with spaces where the
   rendered text is identical, recording the change (recommended);
   (b) keep the tabs and allow them in quotations only.
3. Data files (the catalogue JSON files and the source-item CSV). JSON
   strings cannot be wrapped. Options: (a) convert to YAML with folded
   strings, split into files of at most 300 lines, with a round-trip test
   proving the parsed content is identical (recommended); (b) classify as
   first-party data, exempt from the line limits but schema-checked and
   size-capped; (c) keep JSON and split long strings into arrays of
   fragments (not recommended: it changes the data's shape only to pass a
   check).
4. Lockfiles (future `uv.lock`, `composer.lock`). These are written by
   their tools and cannot be hand-wrapped. Options: (a) classify as
   lockfiles, exempt from line limits, verified by the tool's own integrity
   checks (recommended); (b) no lockfiles, which would weaken reproducible
   installs.
5. Approved specification identities. SPEC-001, SPEC-002 and SPEC-003 have
   lines of 81 or 82 characters. Reflowing them changes their SHA-256 but
   not their words. Options: (a) record an administrative reformat with old
   and new hashes and a word-for-word comparison, keeping their approval
   (recommended); (b) leave them unchanged and non-compliant until their
   next approved revision.

6. Verbatim interview transcripts. The seven interview files, the initial
   request and two quotations in the follow-up decisions show the owner's
   questions and answers word for word inside code blocks. Code blocks keep
   line breaks, so wrapping their long lines would change how the quotation
   looks, and the source-item register cites lines inside those answers.
   Options: (a) keep the exact quoted lines in the code blocks, allow long
   lines only inside those historical quotation blocks, and split each
   transcript into parts of at most 300 lines by question range, updating
   the register's file paths (recommended); (b) wrap the quotations with a
   visible continuation mark, recording the change and renumbering the
   register's line references; (c) convert quotations to ordinary block
   quotes, which can wrap but lose exact formatting.

## Already settled by the rule

- Third-party material is never edited to meet the limits. It is
  inventoried and kept out of first-party paths.
- Binary files (images, fonts, database fixtures) are not text; they are
  inventoried with their own size limits.
- Generated first-party source is measured like hand-written source.

## Foundation sequencing

Publishing `691559a` unchanged means `main` would briefly contain a commit
that predates the limits. The proposed sequence publishes, in one
fast-forward: the historical foundation commit with its original evidence,
then the compliance commit, then the 30 October requirements commit. The
push checks require the final tree to comply and each new commit to carry
its own passing evidence. This needs the owner's approval because the
foundation commit itself will never satisfy the new limits.
