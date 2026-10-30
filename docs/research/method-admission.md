# Method admission and counting

Status: specification draft for owner review. No implementation is claimed.

The catalogue answers two different questions: what should we investigate,
and what has earned a place in a release? Only the second count contributes
to the required 6,000 methods.

## One procedure, one identity

Give each candidate one stable identifier. Record other names as aliases.
Using the same procedure for linking, spam analysis and confidence does not
create three methods. A library function, a notebook, a command and a
wrapper around the same procedure are not separate methods either.

A proposed research variant needs all of the following:

1. A source-backed difference in the objective, assumptions, representation
   or computation.
2. A small example on which that difference changes the intended behaviour.
3. A useful Sutradhar role that can be measured independently.
4. A review explaining why it is more than a setting or an alias.

Changing a weight, threshold, seed, temperature, population size, language
pack, model size or preset does not earn another identity. Renaming a known
procedure does not make it original. Rearranging existing components does
not earn a new identity without a distinct, justified procedure and tests.

Related methods remain separate candidates while the difference is being
investigated. That is permission to research them, not permission to count
them as qualified. Any duplicate is linked to the surviving identity;
history is retained without counting it twice.

## Admission record

Every method must have:

| Area | Evidence needed before release qualification |
| --- | --- |
| Source | Primary paper or official documentation, version, sections read and access date. |
| Distinctness | Comparison with the nearest existing entries and a distinguishing example. |
| Specification | Inputs, outputs, mathematics, assumptions, missing-data behaviour and owner approval. |
| Rights | Source-expression, implementation, dependency, model, dataset and patent review for the intended distribution. |
| Correctness | Independent expected results, boundary cases, property tests and the applicable coverage and mutation evidence. |
| Usefulness | A predeclared comparison showing a useful contribution on held-out examples or an approved diagnostic purpose. |
| Resources | Measured peak memory, work size, time, cancellation and restart behaviour in the qualified environment. |
| Integration | Public-content permissions, stale-data handling, source offsets, output provenance and failure isolation. |
| Reviews | Recorded code, performance and security review of the relevant version. |

The record must identify the exact code, input, model, settings and test
versions. A receipt from an older version is reusable only when its declared
inputs and scope are unchanged and the governing reuse rule permits it.

## Stages

`candidate` means a research lead. `research_ready` means the source and
distinctness work is ready for specification review. `spec_approved` records
the owner's approval. `implemented` means the approved behaviour exists in
production code. `verified` means the required evidence is present.
`release_qualified` requires all applicable admission and release gates.
`excluded` retains the reason a proposal cannot proceed.

An exclusion has a category, plain-English reason and decision receipt in
the catalogue's `exclusion` field. Duplicate exclusions also identify the
surviving method. Keep the excluded record so the same rejected proposal
cannot quietly return under another name.

Passing one stage does not silently pass any other stage. In particular,
rights admission does not establish correctness, and correctness does not
establish usefulness or resource compliance. Unresolved rights concerns
block admission. Unrun checks stay unrun.

## Execution and resource boundaries

Each admitted method declares when its required evidence is available.
Every method applicable to the available evidence must run. Missing evidence
produces a visible pending state, never a fabricated value or a hidden zero
weight. A failed method is isolated, its settings are preserved and dependent
outputs are withheld according to the approved feature specification.

Methods work in bounded chunks with durable checkpoints. The production
Python service and all its workers share the 2 GB cap; a module cannot avoid
the cap by creating another worker. Graphs, indexes, numerical buffers and
model loading must be included in the relevant resource measurement. A
small published benchmark or a memory-mapped file is not proof of bounded
resident memory at the required scale.

Candidate optimisation and feature-selection methods are initially lab
research. Early stopping of a trial must not silently skip an applicable
production method. A diagnostic feature subset must not silently set a
required method's ranking weight to zero. Only approved settings within
their permitted bounds may be promoted automatically; code and model
changes follow their separate approval and release process.

## Tests that distinguish methods

A distinguishing test must not be a restatement of the implementation.
Use independently derived small examples, published mathematical properties
where applicable, and deliberate comparisons against a neighbouring method.
Record why the expected result follows from the specification.

For a stochastic method, record its random generator and state. Test a
reproducible path as well as the claimed statistical property over enough
independent runs. Do not accept a lucky seed as evidence of improvement.
Compare candidates under the same data split and resource allowance.

Usefulness tests declare their outcome, comparator and acceptance threshold
before looking at final assessment results. Report uncertainty and important
subgroups. The fixed benchmark and fresh release assessment are separate
from tuning data. Judge agreement is evidence about the specified judging
process; it does not certify universal relevance or factual truth.

## Explanations and notebooks

Once production functions exist, method-level explanations and mathematical
experiments have one home in Marimo notebooks that import those functions.
The catalogue remains an index of identities, sources, status and evidence.
Architecture and feature specifications link to the notebooks instead of
copying their explanations. A notebook must not contain an alternative
production implementation or become a public web execution endpoint.

Until those functions exist, candidate descriptions and test ideas are
research notes. They are not executable examples or completed notebooks.
