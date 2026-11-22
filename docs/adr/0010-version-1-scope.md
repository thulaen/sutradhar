# ADR-0010: Version 1 scope and release bar

Decided by the owner on 8 November 2026. The release-bar measurement was
revised by [ADR-0020](0020-version-1-documentation-and-architecture-baseline.md)
on 18 November 2026.

## Decision

Version 1 holds every approved feature in the feature register and the
expanded scope in spec 0064. It ships only when every feature and declared
supported variant has complete design and acceptance evidence, every
required check and review passes, the mandatory ranking and suitability
gates pass, release artefacts are signed and the owner approves.

There is no standalone numerical quota for methods, capabilities or
variants. Ranking methods ship on held-out product evidence. The mandatory
nDCG@10 floor is 0.95 and the independent per-forum Precision@6 floor is
at least 1.20 times the locked BM25 baseline.

## Why

The owner wants a complete product, not a partial release or a large count
of aliases and wrappers. Outcome evidence ties release scope to usable
behaviour and makes difficult ranking and suitability requirements
explicit.

## Trade-offs

Full upfront specification and acceptance require substantial research,
review and computing time. Removing quantity quotas does not reduce the
feature scope or any quality, safety or performance gate. The 0.95 floor
may prevent release until the product genuinely meets it.

## How we check it

Spec 0076 checks complete feature ownership and bidirectional
requirement/design/task/test traceability. Spec 0064 checks release scope.
Spec 0057 checks ranking and suitability evidence. Spec 0077 checks that
optimisation and champion control cannot waive a mandatory gate.
