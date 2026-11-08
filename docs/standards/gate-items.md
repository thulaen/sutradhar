# Gate items

The owner's list of what the gate must block, set on 8 November 2026.
Each item maps to one or more checks in `gate/catalogue.toml`. How items
block is in [AGENTS.md section 7](../../AGENTS.md#7-the-gate). Terms are
explained in the [glossary](../glossary.md).

| ID | Item |
|---|---|
| GI-01 | Bugs |
| GI-02 | Defects |
| GI-03 | Bad paths |
| GI-04 | Long files |
| GI-05 | Messy code |
| GI-06 | Data safety |
| GI-07 | Code smells |
| GI-08 | Missing tests |
| GI-09 | Spaghetti code |
| GI-10 | Long functions |
| GI-11 | Inefficient code |
| GI-12 | Broken contracts |
| GI-13 | Hardcoded things |
| GI-14 | Undefined behaviour |
| GI-15 | Network call timeouts |
| GI-16 | Separation of concerns |
| GI-17 | Unclear handoff state |
| GI-18 | Security vulnerabilities |
| GI-19 | Performance regressions |
| GI-20 | Memory issues and out-of-memory risks |
| GI-21 | Backward compatibility breaks |
| GI-22 | Schema or data integrity risks |
| GI-23 | Concurrency and race conditions |
| GI-24 | Dead code and cruft accumulation |
| GI-25 | Caching correctness and stale data risks |
| GI-26 | Localisation and accessibility (i18n and a11y) |
| GI-27 | Admin, operator and support workflow gaps |
| GI-28 | Code that breaks DRY and KISS |
| GI-29 | Code duplication and maintainability smells |
| GI-30 | Modified condition/decision coverage (MC/DC) below 100% |
| GI-31 | API versioning and backward compatibility breaks |
| GI-32 | Tests that run code without proving behaviour |
| GI-33 | Generated or vendored code edited by hand |
| GI-34 | Database tables without row-level security (RLS) |
| GI-35 | The system not staying consistent when things go wrong |
| GI-36 | Unbounded queues, buffer overflows and task starvation |
| GI-37 | XSS, CSRF, CORS, SSRF, SQL injection and command injection |
| GI-38 | Backup, restore, rollback and disaster recovery assumptions |
| GI-39 | Transaction safety, partial writes and idempotency failures |
| GI-40 | Loading, empty, error, retry, offline and unsaved-change states |
| GI-41 | Cache invalidation, stale data and permission-aware caching failures |
| GI-42 | Test data that is unrealistic, stale or unable to catch production bugs |
| GI-43 | Queue, worker, webhook, retry, timeout and dead-letter handling problems |
| GI-44 | Timezone, DST, date, currency, number format, Unicode, casing and locale bugs |
| GI-45 | Missing indexes, N+1 queries, bad or slow queries and pagination problems |
| GI-46 | Secrets in source, logs, tests, docs, build output and front-end bundles |
| GI-47 | Error swallowing: silent failures, errors caught without handling or logging |
| GI-48 | Deployment dependencies: missing environment variables, unapplied migrations |
| GI-49 | Inconsistent naming, misleading comments, stale docs, unclear ownership |
| GI-50 | Observability gaps: missing logs, fault tolerance, error tracking or telemetry |
| GI-51 | Poor architecture, design patterns or modular monolith principles |
| GI-52 | Sensitive data in logs, missing audit trails, weak alerts, alert fatigue, missing SLO coverage |
| GI-53 | Front-end state bugs, memory leaks, dangling listeners, subscriptions, timers and resource leaks |
| GI-54 | Hidden coupling: globals, settings, singletons, monkey patches, circular imports, shared mutable state |
| GI-55 | Responsive layout, browser compatibility, keyboard use, focus order, ARIA, contrast, screen readers |
| GI-56 | Platform or environment incompatibilities: unsupported APIs, OS-specific paths |
| GI-57 | Flaky tests, weak assertions, missing mutation, contract, integration or end-to-end tests |
| GI-58 | Coverage below 100%: lines, branches, mutations, contracts, meaningful assertions |
