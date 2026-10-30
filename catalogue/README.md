# Research catalogue

Status: data-only research seed. No production implementation is present.

- `candidates/` contains proposed method identities and test ideas.
- `sources/` records the primary papers and official documentation
  actually consulted, including how much was read.
- `schema/` describes the candidate data format as a JSON Schema.

Each directory's `index.yaml` explains how its part files join into one
record. The data was converted on 30 October 2026 from the earlier
`candidates.json`, `sources.json` and `schema.json` under the owner's
source-limit decision, so that no file exceeds 300 lines or 80 characters
per line. A round-trip check on Dell proved the joined YAML equals the
original JSON exactly; the earlier files remain in Git history.

All initial entries are `candidate`, with `counts_toward_release: false`.
The qualified method count is **zero**. The required count for the complete
first release remains at least **6,000**. Source access, research progress
and qualified progress must be displayed separately.

The schema is a proposed data contract. A schema-valid record is not proof
of truth, distinctness, rights admission or test completion. Referential
integrity, duplicate identities, evidence contents and stage transitions
need the future protected catalogue checks. No executable validator has been
added to the repository. A focused schema check was run separately on Dell;
see the [foundation review](../docs/dev/foundation-review.md) for its limited
scope. That check does not qualify the catalogue's proposed admission workflow.

For the admission process and definitions, see
[method admission](../docs/research/method-admission.md). For rights handling,
see [rights review](../docs/research/rights-review.md).

## Editing the catalogue

Keep stable identifiers. Add aliases to an existing entry when the
procedure is the same. Do not create entries by enumerating parameter
combinations, command names, language packs or renamed presets. Describe
the difference from the nearest existing method and give a test that can
show it. Keep excluded candidates with a reason and a link to the surviving
identity when appropriate.

An excluded entry uses the `exclusion` record for its category, plain-English
reason and decision receipt. A duplicate also names the surviving method in
`distinctness.duplicate_of` and has the distinctness status `duplicate`.
The `exclusion` field describes a current exclusion: every other stage requires
`exclusion: null`. Preserve superseded decisions in the version history with
their decision receipts. Null does not mean a rights or usefulness review has
passed. The future
reference check must reject a duplicate that points to itself or to an
unknown method.

Descriptions are short discovery notes. When approved production functions
and notebooks exist, the notebooks own the mathematical explanations.
This catalogue links to those explanations and the versioned evidence;
it does not duplicate them.

All measured fields are `null` until a qualified run supplies evidence.
Null means unknown or unmeasured, not zero resource use or a passing result.
