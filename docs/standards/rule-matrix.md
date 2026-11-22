# Engineering rule matrix

The rules are in [AGENTS.md](../../AGENTS.md#6-code-standards).
[ADR-0007](../adr/0007-standards.md) and
[spec 0007](../specs/0007-performance-and-engineering-gates.md)
require an applicable rule, its enforcement and a planted violation.
This matrix relates the project contracts to the gate and reviews.

## Classify the scope

Use section 4 of AGENTS.md and the
[gate catalogue](../../gate/catalogue.toml) for language and path
admission. Record evidence for admitted source, policy and build
languages and development-only models under their exact roles.
Apply file-kind, input and custody rules separately to configuration,
licences, Markdown and generated files.

Classify the changed operation and its reachable control and evidence
boundaries under
[ADR-0020](../adr/0020-version-1-documentation-and-architecture-baseline.md)
and
[spec 0076](../specs/0076-specification-completeness.md#risk-based-assurance).
A strict-core operation keeps its required coverage and independent
reviews. Other work follows the assurance required by its owner.

For each applicable MISRA, NASA Power of 10, JPL or DO-178C Level A
objective, record the source edition and exact objective, the project
clause, language and assurance scope, check or review identity and
blocking tier, and clean and planted refusal cases. Record the reason
for any inapplicability. Use only the checks and tiers supplied by the
protected catalogue and owning rules. Use independent review where
automated evidence cannot prove the objective.

## Project control anchors

| Project clause | Applicable scope | Owning enforcement and evidence |
|---|---|---|
| AGENTS.md section 6 items 1 and 13 | Each admitted source operation and its test boundary | Use the applicable test-first rule in the gate catalogue. Rule M08 in spec 0061 is a name-based reachability check with stated limits. Keep the exact baseline failure and feature-specific tests distinct. |
| AGENTS.md section 6 items 6, 10 and 11 | Rust safety and the applicable source size, state and complexity limits | Workspace Rust lint policy and the limits and analysis checks selected by the gate. The benchmark exception is owned by section 6. Review at the applicable assurance scope. |
| AGENTS.md section 6 item 4 | Strict-core operations and reachable control flow | Apply the strict-core coverage and independent review requirements. Keep proofs for equivalent mutants and named planted cases. Ordinary work uses its owning risk-based checks. |
| AGENTS.md section 5.2 and spec 0108 | Product SQL, migrations and database calls | Inspect forced RLS, role, migration and site transaction evidence. Spec 0061 R19 and H39 check specific source patterns. Require the owning database tests and review for actual isolation. |
| Specs 0021, 0031 and 0118 | PHP site adapters and their protected Rust callers | Use the owning paired communication, native grant, write, undo and protection cases. Use PHPStan and applicable taint evidence under the owning contract. |
| AGENTS.md sections 4 and 7 | Bounded POSIX shell and protected policy or build files | Use the exact language admission, gate and review rules. Shell format and lint entries apply within their recorded coverage. Cedar and Rego use their owning permission and policy tests. |
| AGENTS.md sections 4 and 6 and ADR-0021 | TypeScript, Angular templates, CSS and admitted Python roles | Use the referenced Google guides, the declared platform contract and each language's admitted checks and planted cases. Python scope stays with the admitted development and sidecar boundaries. |
| ADR-0021 and spec 0076 | Starlark and development-only proof models | Bind to the admitted build or proof obligation and applicable tool and review. A model proves its stated property under recorded assumptions. |

Match the planted case to the exact detector. For example,
`rust.format` names its format refusal in the gate catalogue, and
R09 in spec 0061 names the discarded-error finding used by
`analysis.nirikshak`. The catalogue owns the registered tiers and
partial gate coverage. Spec 0011 GG-04 owns the common finding shape.

A source mapping describes which part of the objective the project
contract covers. A missing pairing, check, review or planted case
remains a gap under spec 0076.

## Power of Ten mapping

The author published ten rules for safety-critical C. The rows below
apply the project contracts to the related engineering concern.
A code review may find a partial match or a missing contract. The
original C mechanism alone cannot define a restriction in another
language. [Holzmann, 2006](https://spinroot.com/gerard/pdf/P10.pdf)

| Rule | Project mapping and evidence boundary |
|---|---|
| 1 | AGENTS.md sections 5.1 and 6 specify acyclic module boundaries and local complexity limits. Inspect the protected dependency graph and review changed control flow. A module cycle test alone cannot prove absence of call recursion. |
| 2 | AGENTS.md section 5.6 and specs 0019 and 0078 bound jobs and resources. Exercise the owning limit and its adjacent oversize case. Job admission alone cannot establish a static bound for every loop. |
| 3 | AGENTS.md section 6 item 8 and spec 0007 bound allocation in measured hot paths. The runner's planted allocation covers that loop. The app also has the measured budget in spec 0078. The allocation rule has no equivalent general ban after start-up. |
| 4 | AGENTS.md section 6 item 11 sets function and file bounds with spec 0007. Check the clean boundary and a planted over-limit routine in each applicable detector's scope. |
| 5 | AGENTS.md sections 5.1 and 6 require typed errors and relevant tests and proofs. Inspect invariant rejection and failure recovery. The project has no assertion-density quota. |
| 6 | AGENTS.md sections 5.1 and 6 require explicit module responsibility and restrict global state. Inspect the applicable state check and review the smallest required data scope. |
| 7 | AGENTS.md section 5.1 bars swallowed errors. Exercise R09 under spec 0061 and the owning boundary's invalid input and dependency failures. These checks exercise their named cases. |
| 8 | AGENTS.md section 4 governs admitted first-party inputs and generated files. The C preprocessor clause is outside the first-party product language. Review equivalent obscured build branches under the admitted graph rules. |
| 9 | AGENTS.md section 6 item 6 and the workspace lints govern safe Rust, with the defined benchmark exception. Raw C pointer syntax is outside admitted first-party product sources. Review callable dependency wiring and third-party ABI scope separately. |
| 10 | Spec 0003 and the catalogue own compiler and analyser checks and tiers. Bind each supported language's pinned check, warning selection and planted failure. A registered entry gives evidence only after its applicable run passes. |

## Other source families

- The MISRA Consortium distinguishes its C and C++ rules from the
  evidence needed for a compliance claim. Identify the applicable
  licensed edition and exact clause before mapping it to another
  language. The local anchors give evidence for project rules only.
  [MISRA Compliance](https://misra.org.uk/compliance/)
- NASA's engineering handbook identifies the JPL C standard as
  D-60411, version 1.0, 3 March 2009. A complete mapping needs the
  admitted reference text and each translated or inapplicable
  clause. Keep JPL institutional clauses distinct from the
  Power of Ten paper.
  [NASA coding standards references][nasa-refs]
- The FAA recognises the 2011 RTCA DO-178C edition. The FAA reference
  identifies the standard. Map the Level A objectives from the
  admitted RTCA text to the applicable strict-core records in
  spec 0076. A project coverage requirement alone supplies no
  objective-by-objective qualification.
  [FAA AC 20-115D][faa-ac]

[nasa-refs]: https://swehb.nasa.gov/pages/viewpage.action?pageId=85426394
[faa-ac]: https://www.faa.gov/documentLibrary/media/Advisory_Circular/AC_20-115D.pdf
