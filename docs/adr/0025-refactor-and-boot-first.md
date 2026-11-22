# ADR-0025: Full refactor and Behemoth Boot before product work

The owner approved this delivery order on 21 November 2026.
This decision changes the roadmap and design documents.

## Decision

After the existing gate prerequisites, complete the full refactor of
the existing foundation. After it passes its acceptance checks, build
and qualify all of Behemoth Boot. Once the exact qualified framework
is admitted, start the first Sutradhar product slice and then the
remaining product phases. The roadmap enforces these three stages
in sequence.

Full Behemoth Boot means every applicable feature of the verified
release-train scope in spec 0081. A partial starter set or a sample
application cannot serve as its completion evidence. Use the owning
framework specifications and prove the complete platform matrix.

## Reason and consequences

This order puts the shared architecture, operating contracts and full
framework proof in place before downstream code depends on them.
The refactor needs an exact inventory, compatibility checks and a
tested recovery route. The framework needs its own synthetic end-to-end
fixtures and signed results before the first product integration.

This decision supplements ADR-0022 and supersedes the earlier Phase 3
sequence that built a partial framework alongside the product slice.
Phase 2 supplies the signing bootstrap and gate. The product use of
Tijori moves onto qualified Boot in the first product slice. Aegis and
forge-sync follow ADR-0019. The remaining product phases follow the
same order after the product slice. The release gates still apply.

The [refactor specification](../specs/0139-full-foundation-refactor.md)
defines the evidence for both handoffs.
