# Security

## Report a problem privately

Use GitHub's **Security → Advisories → Report a vulnerability**. Do not
put exploit details, passwords or private site data in a public issue.

Include the version, a small made-up example that shows the problem,
what you expected and what happened. Remove host names, member details
and real post text.

How quickly we reply, and which releases get fixes, is in
[AGENTS.md section 8.1](AGENTS.md#81-releases).

## What we protect

How Sutradhar protects site data, keys and messages is set out in
[AGENTS.md sections 5.4, 5.5 and 11](AGENTS.md#54-the-site-link).

The encryption gate checks protected traffic and managed storage
across the PC and both site add-ons. A failed required check holds
the affected protected operation. For approved external copies,
the sending endpoint encrypts for the named recipient.
[Encryption gate design](docs/specs/0118-end-to-end-encryption-gate.md).

The desktop owner reviews the maximum charge and exact data for an
optional paid search lookup. User data sent on this route is limited to
the approved terms, targeting and eligible public addresses.
[Keyword research](docs/specs/0132-dataforseo-keyword-research.md) and
[live search evidence](docs/specs/0133-dataforseo-live-coach-evidence.md).

The local Empirical Update Lab reads only eligible settled Search
Console comparisons for the active authorised site under
[ADR-0024](docs/adr/0024-own-site-update-lab-evidence.md). The Lab
starts no provider request and sends no study data to a search
provider. Its advisory results stay separate from live ranking and
spam checks. Untrusted records carry no instructions or permissions.
