# 0109: Behemoth Boot security and identity

This is group PB-E of Behemoth Boot (spec 0081). It owns how a Behemoth Boot
program decides who someone is and what they may do: the security filter chain,
OAuth 2.0 in all three roles, OpenID Connect and tokens, SAML, LDAP, passkeys
and other passwordless login, method and data security, protection against web
attacks, shared certificate bundles and sessions. Spec 0081 owns the ledger and
clean-room rules, spec 0106 the core, spec 0107 the web layer (this spec plugs
into its servers and clients), and spec 0108 the data layer (sessions use it).

Sutradhar's own use is small. Its owner screen and tools reach the engine only
over an owner-only Unix socket or named pipe (ADR-0021), and every request is
checked against Cedar permission rules (AGENTS.md section 5, ADR-0021). It has
no passwords and no user accounts of its own. It uses an OAuth 2.0 client,
with the user's own accounts, to connect services such as Google Search
Console (spec 0040). Everything else here is framework parity: built, tested
and kept in step with Spring, but off in the Sutradhar product profile.

Spring facts here come from the owner's pillar list (19 November 2026) and are
"unverified" until the scout confirms each against official notes (spec 0081).
The behaviour description, not the Spring wording, is the contract. Standards
are cited by number from memory and the scout checks them. One name in the list
is known to be unconfirmed: the IP and subnet matcher class (QR-004).

Example: a request reaches the engine's socket from a process that is not the
owner's account and carries no owner-made token. The filter chain, which
denies by default, answers 401 with problem details (spec 0107) and a plain
title, "Sign-in needed", and nothing about why. A request from the owner's own
account passes the chain, and the method check for `delete_site` asks Cedar,
which says "allow only with the live confirm". The function does not run until
the owner confirms that one plan in the app (spec 0020). When the certificate
for the outbound client is renewed on disk, the running client picks it up
with no restart and no dropped call.

Values marked "proposed" are design choices to be checked by measurement.

## What it does

### Words used here

- **Principal**: the user or program making a request.
- **Authentication**: proving who the principal is.
- **Authorisation**: deciding what the principal may do.
- **Filter chain**: an ordered list of checks every request passes through.
- **Request matcher**: a rule that picks which requests a chain applies to.
- **Token**: a proof of identity or permission, such as a JWT (a signed token)
  or an opaque token (an unreadable reference checked with its issuer).
- **JWKS**: the published list of public keys that sign tokens.
- **SSL bundle**: one named set of keys, certificates and trust anchors.
- **Session**: the server-side record of a signed-in principal.
- **Cedar**: the permission-rule language Sutradhar uses (AGENTS.md section 5).
- **Problem details**: the standard error object of spec 0107. **Row-level
  security**: the database rule of spec 0108. **Composition root** and
  **hot reload**: defined in spec 0106.
- **Token family**: a refresh token and every token issued from it.
- **Introspection**: asking a token's issuer whether a token is still valid.
- **Signature wrapping**: an attack that moves signed content so a signature
  still checks but the program reads other, unsigned content.

### What this spec designs

| Feature | What the user or program sees |
|---|---|
| N-1842 Composable security filter chain | Ordered, deny-by-default checks on URL patterns, methods and custom rules |
| N-1843 Full OAuth 2.0 client, resource server and authorisation server | All three roles, with PKCE, client credentials, token rotation, dynamic registration and consent screens |
| N-1763 OpenID Connect, JWT and opaque token engine | Signature checking, key rotation and introspection |
| N-1844 Enterprise SAML 2.0 single sign-on engine | Signed assertions, provider metadata, single logout |
| N-1764 Clean-room LDAP client and embedded LDAP server | Directory lookups and a test directory |
| N-1765 Modern passwordless login | Passkeys, one-time magic links and multi-factor flows |
| N-1766 Fine-grained method and data security | Role hierarchies, checks before and after a function, filtered results |
| N-1767 Web attack shields and IP matchers | CSRF, CORS, security headers and address allow-lists |
| N-1768 Live-reloading SSL certificate bundles | One definition used everywhere, rotated with no downtime |
| N-1769 Distributed session management engine | Revocable sessions in admitted framework stores under DR-071 |

### Who is the principal on the local door

1. Sutradhar has no passwords. On the Unix socket the principal is the operating
   system user of the connecting process, read from the peer credentials. On
   the Windows named pipe it is the identity of the pipe's client. A principal
   is "the owner" when it is the account that owns the home folder (spec 0030).
2. An agent or script that is not the owner's own session presents a scoped
   token made by the owner (spec 0020). The token is checked by this chain.
3. A different operating system user is refused at the socket or pipe itself.

### Threat model

Threats considered: another process running as the same operating system user
(trusted as that user, except that no token and no script can give the live
confirm, spec 0020); a malicious or confused agent with a scoped token; a
stolen token or refresh token; a malicious or compromised identity provider or
metadata source; a hostile remote client of a framework server; and a
certificate or key mistake. Planted-fault and behaviour tests name the
threat they answer. The threats the owner accepts are recorded in the risk
register.

### The filter chain

1. A chain is a list of filters in a fixed, documented order: connection and
   address checks, header and attack shields, authentication, authorisation,
   then the route. A program may have several chains, each chosen by a request
   matcher (path, method, header, address, or a combination). The first chain
   whose matcher fits handles the request; a request no chain matches is
   refused, never passed through.
2. Deny by default. A route with no rule is refused (401 or 403). Opening a
   route is written explicitly, and the Engine Room lists every open route.
3. A filter failure ends the request. No filter swallows an error (AGENTS.md
   5.1). Errors are problem details (spec 0107) with a stable type, a plain
   title and no hint about which check failed; the detail goes to the audit log
   only.
4. The order of filters is checked when the program is built, and an
   authentication filter placed after an authorisation filter is a build error.
5. Custom filters are ordinary Rust types registered at the composition root
   (spec 0106), not discovered at run time.
6. Every decision is logged once with the principal, the route, the rule and
   the result, and never with a credential (spec 0032).

### OAuth 2.0

1. **Client:** authorisation code with PKCE (RFC 7636), client credentials, and
   refresh with token rotation. Desktop redirects follow RFC 8252: a loopback
   address with a literal `127.0.0.1` or `[::1]` (never the word "localhost"),
   a port chosen at run time, matched exactly except for the port, and a
   `state` value checked on return. Tokens and refresh tokens are kept in the
   secret store (spec 0065), never in a file or a log. This is the role
   Sutradhar uses.
2. **Resource server:** validates the access token on every request, checks
   scopes and audience, and maps scopes to permissions.
3. **Authorisation server:** issues authorisation codes with PKCE, client
   credentials grants and refresh tokens with rotation. A reused refresh token
   revokes its whole family; a short grace window (`auth.refresh_grace_s`)
   covers a legitimate retry after a lost response, and only for the same
   client. A consent screen names the client and the exact scopes.
4. PKCE accepts only the S256 method; `plain` is refused, and a code verifier
   sent when no challenge was registered is refused (downgrade).
5. Dynamic client registration (RFC 7591) needs an initial access token or the
   owner's approval, validates every redirect address (exact, no wildcards, no
   fragments) and is off unless the owner switches it on. The setting that
   allows open registration does not exist.
6. The implicit and password grants are not offered. The current security
   guidance is the OAuth 2.0 security best current practice (RFC 9700, number
   unverified until the scout confirms it).
7. Lifetimes are settings: code (60 s), access token (10 min), refresh token
   (30 days), consent (90 days).
8. The authorisation server is the most risky role and is built last. Its
   release needs an independent security review and a protocol conformance run.

### OpenID Connect, JWT and opaque tokens

1. JWT checks are strict. Permitted algorithms are a named list (default ES256,
   EdDSA, RS256 and PS256); "none" is never accepted; an HMAC algorithm is
   refused unless a symmetric key is configured for that issuer, and a public
   key is never used as an HMAC secret (algorithm confusion). The algorithm in
   the token must match the key. The headers `jku`, `x5u`, an embedded `jwk`
   and any unknown `crit` entry are refused. Issuer, audience, expiry and
   not-before are required, with a small clock tolerance (`auth.clock_skew_s`).
2. For OpenID Connect ID tokens, `nonce`, `azp` and `at_hash` are checked where
   present, and the issuer in a discovery document must equal the configured
   issuer (RFC 9207 mix-up defence).
3. Keys come from a JWKS address, are cached, and are refreshed on an unknown
   key id. Refreshes are limited in rate, and unknown key ids are remembered
   for a short time so a flood of random ids cannot force fetches. A key that
   rotates out is dropped after its overlap time.
4. JWKS, discovery and introspection fetches all go through the declarative
   client of spec 0107, so the outbound address protection applies to every
   one of them. Opaque tokens are checked at the issuer's introspection
   endpoint (RFC 7662). Results are cached for `auth.introspect_cache_s`,
   bounded by the token's own expiry. A revoked opaque token therefore stays
   valid for up to that cache time, and a JWT has no revocation list, so JWTs
   are kept short-lived. This is stated to users and tested.
5. A token never appears in a log, a report or an error message.

### SAML 2.0

1. Assertions and responses must be signed. The signature reference must point
   at the element actually consumed; identifiers must be unique;
   canonicalisation is exclusive with no stylesheet transforms; SHA-1 is
   refused. Destination, Recipient, Audience, InResponseTo and NotOnOrAfter
   are checked, and assertion identifiers are remembered for
   `auth.saml_replay_cache_min` to refuse replays. The XML limits and the no-
   external-entity rule of spec 0107 apply.
2. Identity provider metadata is synced on a schedule. A new signing key is
   accepted only when the metadata that carries it is itself signed by a
   pinned key, or the owner approves the change; otherwise the sync keeps the
   old key and raises an alert (SUT-990).
3. Single logout is supported both ways.
4. No mature, audited Rust SAML library is known to us. A thin layer of our own
   over mature XML and signature crates is a build-or-reuse record (QR-035).
   It is built after the other groups and needs an independent security review.

### LDAP

1. The client does bind, search and authenticate. A lookup that expects one
   entry fails if it finds none or more than one (the "mandatory single result"
   rule, unverified wording).
2. Filters are escaped under RFC 4515 and distinguished names under RFC 4514; no
   text is joined into either. A bind with an empty password is refused, because
   many servers treat it as an unauthenticated bind that succeeds.
3. LDAPS or StartTLS with certificate validation is required for any bind
   that carries a password. Referrals are not followed unless the owner allows
   them. Timeouts and a maximum entry count apply (`auth.ldap_timeout_s`,
   `auth.ldap_max_entries`).
4. An embedded directory server exists for tests only, with LDAPS and
   hot-reloaded certificates, and is never part of a product build.

### Passwordless and multi-factor

1. Passkeys use WebAuthn through a mature crate. The relying-party id and
   the allowed origins are settings (`auth.rp_id`, `auth.allowed_origins`). A
   challenge is single use and expires (`auth.challenge_ttl_s`); user
   verification is required. A signature counter that goes backwards raises an
   alert, and a counter that is always zero (common for synced passkeys) is
   tolerated. Credentials can be listed and removed.
2. Magic links are one-time, expire quickly (`auth.magic_link_ttl_s`), are bound
   to the request that asked for them by a cookie nonce, and are stored only as
   a hash. The link opens a page that needs a confirming click (a POST), so a
   mail scanner that fetches the link cannot use it up. Any redirect after
   sign-in is checked against an allow-list (no open redirect).
3. Multi-factor flows combine a first factor with a time-based code (RFC 6238,
   window set by `auth.totp_window_steps`, a used code refused) or a passkey.
   Recovery codes are shown once and stored as hashes made with a slow,
   salted password hash.
4. Sign-in attempts are rate limited per principal and per address, and after
   `auth.lockout_threshold` failures a principal is locked for
   `auth.lockout_minutes`. The response for a wrong name and for a wrong secret
   has the same content, and the wrong-name path does the same work (a dummy
   hash or bind) so timing does not tell them apart.

### Method and data security

1. A service function may carry a rule: required role or permission before it
   runs, and a check or a filter on what it returns. The rules are explicit
   attributes on the function (no hidden reflection), and the check runs in the
   service layer even when the web layer would have allowed it.
2. Roles can form a hierarchy ("admin includes editor"). The hierarchy is
   declared once and checked for loops at build time.
3. The decision source is a trait. Sutradhar's profile plugs in Cedar, whose
   answer may be "allow", "deny" or "needs confirm" (ADR-0021). Behemoth Boot's
   own default is a role and scope check.
4. "Needs confirm" is only a hand-over. The adapter never answers "allow" for
   it. It passes the plan to the `ConfirmationCheck` of spec 0014, which spec
   0020 implements: the confirm is its own action that no token and no script
   can call, it is tied to one plan, used once, expires after
   `actions.confirm_wait` and must come from the owner's own account. This
   spec owns only the hand-over; 0020 owns the confirm and its tests. A call
   waiting for a confirm answers 428 with SUT-998.
5. A result that contains rows the principal may not see is filtered, or the
   call fails; it is never returned whole with a warning. This sits on top of
   row-level security (spec 0108) and does not replace it.
6. Deny wins when two rules disagree.

### Web attack shields and address matchers

1. **CSRF:** token checks and same-site cookies for browser-style requests that
   use cookies. Requests over the local socket carry no cookies and need no
   token. The real threat there is another process of the same operating system
   user, and what stops it is the owner-made token scope and the confirm rule
   above, not a CSRF token.
2. **CORS:** this spec owns `web.cors.*`. An explicit allow-list of origins,
   methods and headers, with a maximum age and a credentials switch. A wildcard
   with credentials is refused at start.
3. **Security headers:** strict transport security, content type protection,
   framing control, a content security policy and a referrer policy, on by
   default and changeable only by explicit settings.
4. **Address matchers:** allow-lists and deny-lists of addresses and subnets
   for any chain, in both IP families. The name of the matching feature in
   Spring is unverified (QR-004).
5. Rate limiting and circuit breakers belong to group PB-F.

### SSL bundles and certificate rotation

1. A bundle is named once in settings: private key, certificate chain and
   trust anchors, from files or a secret reference. Servers, clients, gRPC,
   telemetry export and LDAP refer to it by name. Minimum version and client
   authentication are set per bundle.
2. TLS 1.3 is used where both sides support it, with TLS 1.2 as the default
   floor (a setting; the scout checks the current guidance). Only AEAD cipher
   suites with forward secrecy are offered; the list is fixed in code.
3. Hot reload: the app watches the files or receives a reload call from the
   owner's account only, checks the new bundle (key matches certificate, chain
   valid, not expired), and swaps it for new connections with no downtime.
   A bad replacement is refused with SUT-990. Keep the old bundle only
   while it is still eligible. An open connection may finish admitted
   work on its original bundle while current trust and scope permit it.
   A revoked protected connection stops at the 0118 request and reply
   boundaries. A passed handshake cannot carry current authority by itself.
4. Expiry is warned about at `tls.expiry_warn_days` and shown in the Engine
   Room. A bundle that has expired stops new connections of that bundle
   (SUT-991).
5. Private keys are never logged and never shown. They are stored with
   owner-only permission and, as spec 0065 does for its keys, encrypted at
   rest in files only the instance can read; no operating-system key store is
   assumed (QR-036 asks whether to add one).
6. Mutual TLS is an option per bundle: `tls.client_auth` (none, optional or
   required), a client trust set, and revocation checking where the issuer
   publishes it.

### Sessions

1. An admitted framework session uses PostgreSQL with spec 0108 site
   scope and row-level security, Valkey under DR-071, or another
   separately admitted store through the typed interface. These
   framework profiles supply no product login or session store.
2. Only a hash of the session id is stored. The id is random and long, replaced
   at sign-in and after a privilege change. The cookie uses the `__Host-`
   prefix, is secure, same-site and not readable by scripts.
3. Sign-out ends the session in the store at once and clears the cookie.
4. Idle and absolute timeouts apply. A concurrent-login limit per principal
   either refuses the new sign-in or ends the oldest session
   (`session.on_limit`).
5. Revocation is immediate because the store is read on every request; session
   lookups are never cached. A store outage refuses requests (fail closed).
6. A session holds no secret beyond its id; the principal's data is read from
   the store.

### Reuse

Our own code is the filter chain, the decision interface and the Cedar adapter,
the token rules, the session rules and the bundle reload logic. Mature crates
sit behind them: TLS, JWT and key handling, WebAuthn, TOTP, XML signature
parts, LDAP clients, and cookie handling. Each choice is a build-or-reuse record
in the research dossier (AGENTS.md section 6 rule 14).

#### Identity and method evidence (N-1763–N-1766)

Keep framework profiles distinct from the Sutradhar product's
operating-system principal, local owner-only transport and scoped
0020 tokens. Use the admitted 0107 protected client, 0065 custody
and 0118 transport. The fixtures in this slice use synthetic
principals and keys and run no real identity service.

| Owner catalogue entry | Exact security contract |
| --- | --- |
| N-1763 OpenID Connect, JWT, and opaque token engine | Check exact issuer, audience, token kind, time, key, algorithm and current role. Fetch discovery, keys or introspection only through the already admitted scoped client route. Keep key refresh bounded and reject unproved state. Pin approved issuer and key source. Preserve declared opaque revocation and JWT lifetime limits. Unresolved introspection holds. Verified identity or claims cannot supply current service or Cedar action authority. No credential in audit, foreign key source or product identity substitution. [FX-0109-W5-TOKEN-EXACT-ISSUER-ROLE-AND-KEY-SCOPE](../planning/feature-wave-5.md#acceptance-design-for-n-1743-to-n-1766). |
| N-1764 Clean-room LDAP client and embedded LDAPS server | Qualify the exact framework directory and separate synthetic test fixture. Check verified TLS bundle, single-result expectation, admitted filter boundary and exact directory scope. Absent, ambiguous or invalid results hold without secrets. Test certificate replacement through its owning validated bundle route. No real directory listener, connection or OS-principal access from a fixture. [FX-0109-W5-SEALED-DIRECTORY-EXACT-RESULT](../planning/feature-wave-5.md#acceptance-design-for-n-1743-to-n-1766). |
| N-1765 Modern passwordless login built in | Use independently admitted framework enrolment and sign-in profiles for passkeys, one-use links and additional verification. Bind participant, relying party, intended effect, current challenge, expiry and consume-once state. Keep safe denied responses and revocation checks. No outbound link send, default product sign-in or 0020 confirmation from a framework login. [FX-0109-W5-PASSWORDLESS-EXACT-CONSUME-ONCE](../planning/feature-wave-5.md#acceptance-design-for-n-1743-to-n-1766). |
| N-1766 Fine-grained method and data security | Enforce current service method policy and result projection for the exact actor and site. Deny wins, role graphs are acyclic and a protected mutation is authorised before any effect. Keep forced RLS and check current grant and erasure at release of results. Pass an exact needs-confirm result under BE-23, 428 and SUT-998 to the existing 0020 action. Sign-in cannot grant allow. Recheck changed role graph and current action scope across nested calls. No cache, nested call or route can skip the governing method check. [FX-0109-W5-METHOD-SCOPE-EXACT-CONFIRMATION](../planning/feature-wave-5.md#acceptance-design-for-n-1743-to-n-1766). |

#### Web, certificate and session profiles (N-1767–N-1769)

Use the exact selected PB-E starter, route, policy and trust
generation. A framework test or profile gives no Sutradhar
listener, account, permission or fallback transport. The
product keeps its owner-only local door and enrolled peer checks
under 0020 and 0118, and existing site-account and connector
authority. The stricter product transport and security floor
wins. Use synthetic identities and no live traffic for these
acceptance fixtures. DR-070 allows documentation-based parity
only. QR-215 binds the exact profiles.

| Feature | Exact framework behaviour and product boundary | Existing requirements |
|---|---|---|
| N-1767 Web attack shields and IP matchers | Bind the admitted web route to a current CSRF, CORS, header and allow/deny policy. Validate effective peer identity through the owning transport and trusted proxy contract if admitted. Deny on an unmatched or uncertain rule. Headers and address matches confer no identity or Cedar permission. Browser protections do not add browser auth or cookies to the local product door. [FX-0109-W5-WEB-POLICY-NO-OWNER-IMPLICATION](../planning/feature-wave-5.md#acceptance-design-for-n-1767-to-n-1790). | BE-01, BE-15, BE-22, BE-32 |
| N-1768 Live-reloading SSL certificate bundles | Pin bundle purpose, consumer, key, trust anchors, client-auth policy, expiry and current generation. Validate the whole proposed bundle before an atomic swap for new connections. A rejected candidate leaves the previous still-valid bundle and eligible connections under BE-16. Revoked or expired active trust cannot open a new connection. Keep owner-only private key custody under BE-17. Only eligible existing connections can drain; no downgrade or grace can override protected-route revocation. Never log key values. [FX-0109-W5-BUNDLE-ROTATION-VALID-TRUST-ONLY](../planning/feature-wave-5.md#acceptance-design-for-n-1767-to-n-1790). | BE-16, BE-17, BE-33, BE-35; 0118 |
| N-1769 Distributed session management engine | In an admitted framework profile bind the session hash, principal, site, backend, limits and revocation generation. Persist only hashed session ID. Under BE-34 sign-out takes effect immediately. Check the owning store uncached before each protected request, including on a second instance. Deny when the store cannot verify it. Replace IDs on sign-in or privilege change, honour idle, absolute and concurrent limits and fail closed on unproved revocation or outage. No product login or distributed session store is admitted. Only DR-071 eligible backend fixtures may be used. [FX-0109-W5-CURRENT-UNCACHED-SESSION-REVOCATION](../planning/feature-wave-5.md#acceptance-design-for-n-1767-to-n-1790). | BE-18, BE-19, BE-34, BE-35 |

#### Framework security interfaces (N-1842–N-1844)

Use the declared listener, filter and method policy, protected client
and key custody. The local product keeps OS peer identity and 0020
token and confirmation controls. Framework identity fixtures use
synthetic keys and principals.

| Owner catalogue entry | Exact framework contract |
| --- | --- |
| N-1842 Composable security filter chain | Choose the first matching admitted chain in the fixed order. A missing rule or chain denies. Check authentication before authorisation and enforce service policy and current scope on any call, including inherited routes. Report only safe reasons outside the audit boundary. No bypass from a new transport or a route's declared filter. [FX-0109-W5-FIRST-ADMITTED-FILTER-RECHECK](../planning/feature-wave-5.md#acceptance-design-for-n-1839-to-n-1862). |
| N-1843 Full OAuth 2.0 client, resource server, and authorisation server | Qualify the client, resource server and authorisation server roles independently. Pin intended issuer and audience, client, redirect, protected grant class, expiry and consent. Require S256 challenge and verifier; refuse plain, missing challenge, implicit or password flow. Match exact redirect and state. Native loopback literal 127.0.0.1 or [::1] may vary runtime port only. No wildcard or fragment. Dynamic registration defaults off and when allowed requires initial access token or owner approval. Apply exact rotated-token lost-reply grace, replay detection and family revocation under 0109. Keep roles and secrets separate. The authorisation server needs its independent review and conformance proof. The 0040 product Google OAuth client and 0065 custody retain their separate route. Resource and server fixtures are synthetic here, without added product login, identity route or 0020 confirmation. [FX-0109-W5-INDEPENDENT-OAUTH-ROLES-AND-REPLAY](../planning/feature-wave-5.md#acceptance-design-for-n-1839-to-n-1862). |
| N-1844 Enterprise SAML 2.0 single sign-on engine | Validate exact signed assertion, issuer, destination, audience, request, subject timing and replay state under the qualified framework profile. Consume only verified fields. Metadata, signing key changes and logout use the separate declared authority. Unknown or mismatched state refuses. No external discovery, product identity switch or disclosure from a fixture. [FX-0109-W5-SAML-EXACT-ASSERTION-BOUNDARY](../planning/feature-wave-5.md#acceptance-design-for-n-1839-to-n-1862). |

### Data

- Ledger and behaviour descriptions: repository files (spec 0081).
- Sessions, credentials of the authorisation server, registered clients and
  consents: database tables with row-level security (spec 0108).
- Secrets: the secret store (spec 0065), never settings files.

### Settings

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `auth.clock_skew_s` | seconds | 30 | 0 to 300 | next start |
| `auth.code_lifetime_s` | seconds | 60 | 10 to 600 | next start |
| `auth.token_lifetime_s` | seconds | 600 | 60 to 86,400 | next start |
| `auth.refresh_lifetime_days` | days | 30 | 1 to 365 | next start |
| `auth.refresh_grace_s` | seconds | 10 | 0 to 60 | next start |
| `auth.consent_days` | days | 90 | 1 to 365 | next start |
| `auth.key_rotation_overlap_h` | hours | 24 | 1 to 720 | next start |
| `auth.dcr_enabled` | on or off | off | two values | next start |
| `auth.rp_id`, `auth.allowed_origins` | text, list | none | exact values | next start |
| `auth.challenge_ttl_s` | seconds | 120 | 10 to 600 | next start |
| `auth.totp_window_steps` | whole number | 1 | 0 to 3 | next start |
| `auth.lockout_threshold` | whole number | 10 | 3 to 100 | next start |
| `auth.lockout_minutes` | minutes | 15 | 1 to 1,440 | next start |
| `auth.ldap_timeout_s` | seconds | 10 | 1 to 120 | next start |
| `auth.ldap_max_entries` | whole number | 1,000 | 1 to 100,000 | next start |
| `auth.saml_replay_cache_min` | minutes | 60 | 5 to 1,440 | next start |
| `auth.allowed_algs` | list | `ES256`, `EdDSA`, `RS256`, `PS256` | subset of those four plus `ES384`, `ES512`; never `none`, never HMAC without a configured symmetric key | next start |
| `auth.jwks_refresh_min_s` | seconds | 60 | 10 to 3,600 | next start |
| `auth.introspect_cache_s` | seconds | 30 | 0 to 300 | next start |
| `auth.magic_link_ttl_s` | seconds | 600 | 60 to 3,600 | next start |
| `auth.max_attempts_per_min` | whole number | 5 | 1 to 100 | next start |
| `session.idle_timeout_s` | seconds | 1,800 | 60 to 86,400 | next start |
| `session.absolute_timeout_s` | seconds | 43,200 | 300 to 2,592,000 | next start |
| `session.max_per_principal` | whole number | 5 | 1 to 100 | next start |
| `tls.min_version` | `1.2` or `1.3`, per bundle | `1.2` | two values | next start |
| `tls.client_auth` | none, optional, required, per bundle | none | three values | next start |
| `tls.expiry_warn_days` | whole number | 30 | 1 to 365 | next start |
| `web.cors.allow_origins` | list | empty | exact origins only | next start |
| `web.cors.allow_methods` | list | `GET`, `HEAD` | HTTP methods | next start |
| `web.cors.allow_headers` | list | empty | header names | next start |
| `web.cors.max_age_s` | seconds | 600 | 0 to 86,400 | next start |
| `web.cors.allow_credentials` | on or off | off | two values; never with a wildcard origin | next start |
| `session.on_limit` | refuse or end oldest | refuse | two values | next start |

There is no setting that turns off deny-by-default, the JWT signature check,
token rotation, certificate validation or secret masking.

### Failure codes

A refusal never says which part of a check failed; the reason goes to the audit
log. Where one code covers several reasons (for example SUT-986), the audit
record carries the reason.

| Code | Meaning | HTTP | gRPC | WebSocket close |
|---|---|---|---|---|
| SUT-985 | Request refused by the chain (including refused registration or consent) | 401 or 403 | UNAUTHENTICATED or PERMISSION_DENIED | 1008 |
| SUT-986 | Token invalid (signature, issuer, audience, expiry, algorithm, key id) | 401 | UNAUTHENTICATED | 1008 |
| SUT-987 | Token or code reuse detected | 401 | UNAUTHENTICATED | 1008 |
| SUT-988 | Sign-in rate limit or lockout | 429 | RESOURCE_EXHAUSTED | 1008 |
| SUT-989 | Permission denied by the decision source | 403 | PERMISSION_DENIED | 1008 |
| SUT-990 | Certificate, key or metadata change refused (old one kept) | not a request error | not a request error | not a request error |
| SUT-991 | Certificate expired | 502 | UNAVAILABLE | 1008 |
| SUT-992 | Session revoked or expired | 401 | UNAUTHENTICATED | 1008 |
| SUT-993 | SAML signature or assertion refused | 401 | UNAUTHENTICATED | 1008 |
| SUT-994 | Directory lookup or bind failed (not exactly one result, empty password, bind refused) | 401 | UNAUTHENTICATED | 1008 |
| SUT-995 | Second-factor check failed (passkey, code, magic link) | 401 | UNAUTHENTICATED | 1008 |
| SUT-996 | CORS or CSRF refused | 403 | PERMISSION_DENIED | 1008 |
| SUT-997 | Address not allowed | 403 | PERMISSION_DENIED | 1008 |
| SUT-998 | The live confirm is needed | 428 | FAILED_PRECONDITION | 1008 |
| SUT-999 | Credential store unavailable (fails closed) | 503 | UNAVAILABLE | 1013 |

### Security and privacy

- No credential, token, key or recovery code appears in a log, a report, an
  error or a screen.
- Responses to a wrong name and a wrong secret are identical in content and
  timing within a small tolerance.
- Failures close, never open: an unreachable store, directory, key set or
  decision source refuses the request.
- Every sign-in, token issue, revocation and permission denial is audited
  (spec 0032).

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| BE-01 | A request that no chain matches, or that has no rule, MUST be refused. | UT-BE-01, PL-BE-01 |
| BE-02 | A filter order with authentication after authorisation MUST fail the build. | UT-BE-02 |
| BE-03 | An error MUST be problem details that names no failed check, and the detail MUST go only to the audit log. | UT-BE-03, FZ-BE-01 |
| BE-04 | A JWT MUST be refused unless its algorithm is allowed and matches the key and its issuer, audience, expiry and not-before are valid. | UT-BE-04, PL-BE-02 |
| BE-05 | JWKS refresh on an unknown key id MUST be rate limited. | UT-BE-05 |
| BE-06 | An OAuth client MUST use PKCE where a code is exchanged and keep tokens only in the secret store. | UT-BE-06 |
| BE-07 | A reused refresh token MUST revoke its token family. | UT-BE-07, FI-BE-01 |
| BE-08 | The implicit and password grants MUST NOT be offered, and redirect addresses MUST match exactly. | UT-BE-08 |
| BE-09 | A SAML assertion MUST be refused unless signed, and signature wrapping and duplicate-id cases MUST be refused. | UT-BE-09, PL-BE-03 |
| BE-10 | An LDAP lookup expecting one entry MUST fail on none or many, and a filter MUST be escaped. | UT-BE-10, FZ-BE-02 |
| BE-11 | A magic link MUST be single use, expire, and be stored as a hash. | UT-BE-11 |
| BE-12 | A wrong-name and a wrong-secret sign-in MUST give the same content and do the same work, attempts MUST be rate limited, and a principal MUST be locked after the threshold. | UT-BE-12, MS-BE-01 |
| BE-13 | Method rules MUST run in the service layer, deny MUST win, and role hierarchies MUST be loop-free. | UT-BE-13, IT-BE-01 |
| BE-14 | A decision of "allow only with the live confirm" MUST stop the call until the confirm. | BD-BE-01 |
| BE-15 | CORS with a wildcard and credentials MUST be refused at start. | UT-BE-14 |
| BE-16 | A new SSL bundle MUST be validated, swapped without dropping still-eligible connections. A bad replacement MUST be refused and the previous bundle kept only while still eligible. Protected traffic follows 0118 on revocation. | IT-BE-02, FI-BE-02 |
| BE-17 | A private key MUST never be logged and MUST have owner-only permission. | UT-BE-15 |
| BE-18 | A session id MUST be replaced at sign-in, revocation MUST be immediate, and a store outage MUST refuse requests. | IT-BE-03, FI-BE-03 |
| BE-19 | Idle, absolute and concurrent-login limits MUST apply. | UT-BE-16 |
| BE-20 | The authorisation server MUST NOT ship without an independent security review and a conformance run. | AU-BE-01 |
| BE-21 | Each feature MUST have parity rows, behaviour descriptions and independent contract tests. | CT-BE-01 |
| BE-22 | On the local door the principal MUST come from the operating system peer identity, and another user MUST be refused. | UT-BE-17, PL-BE-04 |
| BE-23 | A "needs confirm" answer MUST never become "allow" in this spec and MUST hand over to the confirm check of spec 0020, answering 428 with SUT-998. | UT-BE-18, BD-BE-01 |
| BE-24 | JWT headers `jku`, `x5u`, embedded `jwk` and unknown `crit`, algorithm confusion and HMAC without a configured key MUST be refused. | UT-BE-19, PL-BE-02 |
| BE-25 | JWKS, discovery and introspection fetches MUST go through the protected client of spec 0107. | IT-BE-04 |
| BE-26 | PKCE MUST accept only S256 and refuse a verifier with no registered challenge; redirects MUST match exactly. | UT-BE-20 |
| BE-27 | Dynamic registration MUST be off by default and, when on, need an initial token or owner approval. | UT-BE-21 |
| BE-28 | SAML MUST check destination, recipient, audience, InResponseTo and NotOnOrAfter, refuse SHA-1 and replayed assertion ids, and apply the metadata key-rotation rule. | UT-BE-22, PL-BE-03 |
| BE-29 | LDAP MUST escape filters and distinguished names and refuse an empty-password bind. | UT-BE-23, FZ-BE-02 |
| BE-30 | A passkey MUST use a single-use challenge, required user verification and alert on a counter that goes backwards. | UT-BE-24 |
| BE-31 | A magic link MUST need a confirming POST, and a TOTP code MUST not be accepted twice. | UT-BE-25 |
| BE-32 | CSRF tokens, security headers and address matchers MUST apply as stated, and an address not allowed MUST be refused with SUT-997. | UT-BE-26, UT-BE-27, UT-BE-28 |
| BE-33 | Expiry MUST be warned about, an expired bundle MUST stop new connections, and mutual TLS MUST follow `tls.client_auth`. | IT-BE-05, UT-BE-29 |
| BE-34 | Sign-out MUST end the session at once, only a hash of the session id MUST be stored, and session lookups MUST NOT be cached. | UT-BE-30, IT-BE-03 |
| BE-35 | Sign-in, token issue and revocation, logout, key and bundle changes, consent, registration and permission denials MUST be audited without secrets. | IT-BE-06 |
| BE-36 | The revocation limits of opaque tokens and JWTs MUST be stated to the user and tested. | UT-BE-31 |

## How it is tested

Designed, not run.

- **Unit (UT-BE):** each chain rule, each token check with one value just
  inside and one just outside (expiry, clock tolerance, audience), each
  session timeout, the filter order check, the loop-free role check.
- **Planted faults (PL-BE):** a route with no rule; a JWT signed with "none"; a
  SAML response with a wrapped signature; each must be refused with its SUT
  code (the public answer names no failed check) while an honest request
  passes.
- **Integration (IT-BE):** a full authorisation code flow with PKCE against the
  test authorisation server; a certificate swap while connections are open;
  revocation seen by two processes.
- **Fault injection (FI-BE):** a reused refresh token, a bad certificate, a
  session store that goes down.
- **IT-BE-02 and FI-BE-02:** rotate a valid bundle on a synthetic
  framework route. Let admitted work finish on the eligible open
  connection and verify new connections use the replacement. In the
  applicable 0118 protected fixture, separately revoke current trust
  on an open connection before a new request and before release of
  a staged reply. The first produces no new protected effect. The
  second releases no protected body. Reconcile any proved committed
  effect only through an eligible connection under its current authority.
- **Fuzz (FZ-BE):** malformed tokens, SAML documents, LDAP filters and headers
  never crash a parser or reveal a failed check.
- **Behaviour (BD-BE):** the example above, including the live confirm.
- **Contract (CT-BE):** one test per parity item, written from the behaviour
  description by someone other than the implementer.
- **Audit (AU-BE):** the independent review record of the authorisation
  server and the SAML layer.
- **Measurement (MS-BE-01):** at least 10,000 samples of a wrong-name and of a
  wrong-secret sign-in, with a statistical test; the median difference must
  stay under 1 ms or 5 percent of the median, whichever is larger (proposed).

## Changes from the source

The pillar list asked for clean-room implementations of every protocol.
Behemoth Boot uses mature crates for TLS, tokens, WebAuthn and LDAP, and writes
only the decision, chain, session and rotation logic itself (AGENTS.md section
6 rule 14). The Cedar adapter and the "allow only with the live confirm"
answer are Sutradhar additions, switched on by its profile. SAML and the
authorisation server are the riskiest parts and are built last with an
independent review.
