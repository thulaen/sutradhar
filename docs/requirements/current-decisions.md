# Current decisions

Status: owner decisions captured from Q001–Q209. Product acceptance is PLANNED;
technical qualification remains UNKNOWN until supported by evidence.

The [source index](sources/README.md) contains each cited question, options and
answer. The [override table](overrides.md) explains historical changes. The
statements below are the current requirements, not claims that features exist.

The requirements are kept in three linked parts:

- [R001–R028](current-decisions-product.md): Product and delivery; Internal
  linking; Analysis, tuning and resource limits.
- [R029–R052](current-decisions-features.md): Analytics, SEO and related
  features; Framework, interfaces and experience.
- [R053–R082](current-decisions-operations.md): Quality evaluation and
  release evidence; Repository, privacy and operations; Toolchain and manual.

## 30 October 2026 update

R083–R097 were added in the
[requirements update](updates-2026-10-30/README.md). They replace parts of
R055–R057, R068–R071, R076 and R080, as listed in the
[supersession table](updates-2026-10-30/supersession.md). The rows above
are kept as written; where they conflict, the update applies.

R098–R108, on semantic models and reading every post in full, were added
in the [models update](updates-2026-10-30/models.md), which raises the
service memory cap in R021 and R092 from 2 GB to 6 GB, raises the lab
ceiling from 8 GB to 9 GB, and makes uninstall
delete embeddings, the index and model files (R050 as amended). R109–R111
(community models, one progress system, the visual layer) are in the
[platform update](updates-2026-10-30/platform.md).

## Qualification still required

The interview settled intent. It did not prove PHP/XenForo compatibility,
licence/patent clearance, provider account controls, CLI no-tools enforcement,
coverage instrumentation, browser access, the resource envelope, the
6,000-method admission count or the release quality targets. These remain
explicit evidence requirements in the
[acceptance ledger](acceptance-ledger.md), not exceptions.
