# Sutradhar requirements

Status: requirements captured; implementation and acceptance remain unproven.

This directory records what the owner asked for and the decisions made during
209 questions. It is developer material, not the public user manual.

Read these documents in order:

1. [Current decisions](current-decisions.md): the rules that apply now.
2. [Overrides](overrides.md): which earlier statements have changed.
3. [Acceptance ledger](acceptance-ledger.md): evidence needed before completion.
4. [Milestones](milestones.md): the delivery order and feature-request process.
5. [Specification index](../specifications/README.md): approved foundation designs
   and the remaining features needing specifications and approval.
6. [Interview sources](sources/README.md): all questions, offered options and
   answers, with private identifiers removed.

The [source-item inventory](source-items.csv) records 1,079 non-empty,
non-separator answer lines. The appendices retain all 1,106 non-empty lines,
including 27 decorative separators. This is a source capture count, not features, algorithms or
tests. A long line may contain several requirements. A heading may contain none.
The long library, SEO-signal, edge-case, toolchain and manual requests remain in
the source appendices; summaries do not replace them.

## Authority and evidence

Explicit later decisions replace conflicting earlier passages, as recorded in
the override table. Unselected options are not instructions. Historical labels
and examples are retained as history rather than silently rewritten.

An answered product question is not approval of an unwritten implementation
specification. Each feature specification and every change to protected rules
requires the owner's explicit approval. Ordinary implementation changes inside
approved specifications may follow the unattended review-and-merge process.

Use these evidence states consistently:

| State | Meaning |
| --- | --- |
| PLANNED | Required behaviour has been recorded; there is no completion claim. |
| UNKNOWN | Compatibility, rights, feasibility or another fact has not been verified. |
| SPECIFIED | A concrete specification exists; its approval state is recorded separately. |
| IMPLEMENTED | Source exists, but this does not establish passing tests or installation. |
| VERIFIED | The named requirement has current, appropriately scoped evidence. |
| SUPERSEDED | A later owner decision replaces this historical requirement. |

No default status of VERIFIED is permitted. Missing, skipped, stale or indirect
evidence must not be presented as a pass. No library name, paper citation,
notebook, registry entry or source file counts as a qualified method by itself.

## Privacy of the record

The public record excludes the owner's forum name, domain, member data, private
paths, machine addresses, licensed archive names, disk identifiers and secrets.
Explicit redaction markers identify removals in source quotations. The original
private conversation remains the source; it is not copied into this repository.

Use synthetic examples for public instructions and screenshots. Reviewed,
anonymous summary statistics may be published, but a private test corpus,
excerpts, reversible identifiers or identifying metadata may not be published.
The same rule applies to branches, Git history, previews, logs and artifacts.

## Source capture limitations

The source appendices preserve all numbered prompts, options and answer text,
apart from explicit privacy redactions and line-ending/trailing-space
normalisation. Their wording, spelling and answer-line numbering are retained.
The source-item inventory points to answer line numbers within each question.
Physical numbering includes blank and decorative lines. Labels trim surrounding
whitespace and are previews: a label longer than 120 UTF-16 code units is shortened
to the first 117 units plus `...`. Read the referenced full answer for its content.
Empty lines and lines consisting only of whitespace or `-`, `_`, `=`, `★`, `◆`,
`○` and `|` are not indexed; they remain in the source appendices. The current
27 non-empty exclusions are 26 hyphen-only separators and one table separator.
It is not an executable acceptance suite or a substitute for a feature-by-feature
test design. Any future decomposition must retain those source links.
