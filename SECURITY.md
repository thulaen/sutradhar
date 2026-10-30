# Security and privacy

Sutradhar is not yet a qualified release. Security requirements and scans are
not a guarantee that defects cannot exist.

## Report a vulnerability privately

Use the repository's **Security → Advisories → Report a vulnerability** action
when private vulnerability reporting is enabled. Do not put a vulnerability's
exploit details, credentials or private forum data in a public issue. If that
action is unavailable, ask the maintainer to enable a private reporting route
without posting the sensitive details. This document does not claim the route is
already enabled.

Include the affected version, a minimal synthetic reproduction, expected
behaviour and observed behaviour. Explain the possible impact. Scrub hostnames,
paths, member information, authentication material and real post text.

## Data boundaries

Public source and synthetic fixtures may be used in isolated cloud checks. Real
evaluation corpora and licensed XenForo files remain private on the authorised
development host. No public repository, manual, preview, log, screenshot or
build artifact may reveal the owner's forum identity or member data. Reviewed
anonymous aggregate measurements may be published.

The only approved exception for sending real evaluation text to AI providers is
the bounded redacted judging pack. Its provider, account controls, actual terms,
training setting and redaction tests must be verified first. Do not equate
redaction with guaranteed anonymity or training-off with zero retention. Do not
send entire datasets, private threads, credentials or unrestricted transcripts.

## Execution boundaries

Keep signing credentials, account credentials, private datasets and release
authority out of untrusted code execution. Fork workflows have read-only access
and no secrets. The owner approves an exact outside candidate before its code
runs on the private development host. Recheck trust after every new push.

The GitHub App used for automation must be separate from the owner's personal
approval authority. Its signing keys and write tokens stay in the protected Dell
publisher. GitHub contents-write permission also grants release operations; tag
rules do not turn it into a coding-only credential. Coding workers submit
bounded proposal operations or patches and receive no raw App write token. Their
interface exposes no product-release API. Separate protected identities report
checks, verify owner approvals and publish releases. Coding workers cannot
bypass checks, change rulesets, impersonate approval or invoke release
publication. Trusted publishing and review components verify the exact candidate
themselves.

Forum posts, external pages, model output and dependency metadata are untrusted
data, not instructions. No-tools judging and read-only access are distinct
controls and must both be tested. Models use approved non-executable data
formats; untrusted executable serialisation is prohibited.

## Erasure and audit

Keep minimal append-only event records without names or post text. Store
necessary identifying details separately so they can be erased. Erasure must
remove matching embeddings, caches, snapshots and suggestions, including private
evaluation copies. Record dataset revisions honestly after erasure; do not
retain deleted content merely to reproduce an old score. Keep audit retention
separate from permission to retain personal data.

## Required checks

Security review, dependency admission, secret scanning, permission checks,
input/output validation and fault testing are release and development
obligations defined in [the gate contract](docs/dev/gates.md). An unavailable
required checker blocks its stage. A clean scan cannot prove copyright or patent
clearance, privacy compliance, absence of all vulnerabilities or complete
release readiness.
