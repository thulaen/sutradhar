# Rust authoring guide

This guide applies the existing Rust contracts. The rules remain in
[AGENTS.md](../../AGENTS.md#6-code-standards).

## Layout and interfaces

- Use the workspace's [format settings](../../rustfmt.toml) and
  [lint settings](../../Cargo.toml). Apply the line-width exceptions in
  [spec 0007](../specs/0007-performance-and-engineering-gates.md).
- Use the declared module API. Keep ownership and responsibilities
  clear, and wire product dependencies explicitly through the
  composition root in
  [spec 0106](../specs/0106-behemoth-core-container-and-config.md).
- Document public items where the workspace documentation lint
  requires it. Describe the caller's inputs, result and failure through
  the owning interface contract. Keep module examples consistent with
  the runnable documentation tests required by
  [ADR-0021](../adr/0021-version-1-product-stack-and-scope.md).
- Use explicit, typed error outcomes at boundaries and carry them through
  to the owning error shape. See R09 in
  [spec 0061](../specs/0061-nirikshak.md) for discarded errors.

## Review and checks

Use safe Rust within AGENTS.md section 6, including its narrowly
scoped benchmark exception. Keep functions and files within the
limits there. Use the owning requirement and its test-first case.
The [gate catalogue](../../gate/catalogue.toml) records the selected
format, lint and test checks and their planted cases. Apply the
assurance class of the operation under
[ADR-0020](../adr/0020-version-1-documentation-and-architecture-baseline.md)
and
[spec 0076](../specs/0076-specification-completeness.md#risk-based-assurance).
