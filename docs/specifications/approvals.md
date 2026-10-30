# Specification approval record

Owner decision recorded: 30 October 2026.
Status: the three listed designs are approved for their bounded implementation.

The owner was shown the specification register and asked whether Codex or Claude
may implement the exact drafts for automated checks, the public manual and
protected agent/publishing controls. The question explicitly excluded a product
release and sending real forum data to AI providers. The direct answer was:

> Approve all three foundation designs

## Approved revisions

SHA-256 identifies each complete draft as presented, before its status heading
was updated to record this approval. The design bodies are unchanged by that
administrative update.

| Specification | Approved draft SHA-256 |
| --- | --- |
| [SPEC-001](bootstrap-toolchain.md) | `f2d5981fc053d5ee67640d88f64ceacde99c0e0a554bf1345472d10c446315e5` |
| [SPEC-002](manual-publishing.md) | `7c4694c84563fb48d2698f826e558416f85f0a2976b7c04b2b40305332d2cd1f` |
| [SPEC-003](agent-release-coordination.md) | `9cb83f94494af17802fc4c5eecafe1ed66407b1cbfc8fc45253f731cdace68f5` |

Recording correction: the first approval record accidentally inserted one extra
character into SPEC-003's hash, making it 65 characters long. The value above
matches both the original pre-approval file measurement and reconstruction of
the unchanged approved draft. This corrects the identifier, not the design or
the owner's decision. The earlier saved foundation copy is retained with its
failed metadata check; use the corrected saved version for continuation.

This approves only the scopes stated in those designs, including their tests and
qualification. It does not approve unrelated features, paid provisioning, a
product release, private-data transmission or changes to the design bodies.
The separate mapped NASA/Meta rulebook was not included in this question.

## Evidence boundary

Writing the direct decision and approved identities to the private coordinator
ledger reported success before the storage incident. Their retention must be
verified after recovery; the direct owner decision remains valid. This readable
repository record does not let candidate code
manufacture a trusted owner-approval status. The protected verifier must validate
the actual owner's decision and matching scope/revision. Final admission of a
protected repository change still requires its applicable checks and approval.

Do not ask the owner to approve these same unchanged designs again. Request a new
decision only for a material design/scope change or a separately required action.
