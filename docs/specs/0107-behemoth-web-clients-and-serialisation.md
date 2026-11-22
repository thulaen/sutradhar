# 0107: Behemoth Boot web, clients and serialisation

This is group PB-C of Behemoth Boot (spec 0081). It owns how a Behemoth Boot
program talks over the network and over local sockets: web servers, routing,
API versioning, declarative HTTP clients, protection against requests that
reach private networks, gRPC, real-time streams, GraphQL, hypermedia
responses, data formats and SOAP. Spec 0081 owns the ledger and clean-room
rules and spec 0106 the core (wiring, settings, lifecycle). Security of the
web layer (filter chain, login, certificates) is group PB-E, and data
access is PB-D.

Sutradhar itself opens no network port (ADR-0021): the app and its tools talk
over a Unix socket on Linux and a user-locked named pipe on Windows. The
servers in this spec are part of the framework's parity and are used by
Sutradhar only on those local doors. A network listener exists only when a
starter for it is added and the owner approves a setting; the Sutradhar
product profile has none.

Spring facts here come from the owner's pillar list (19 November 2026). The
scout confirms each against official release notes before its row leaves
`scheduled` (spec 0081). Until then each is "unverified", and the behaviour
description is the contract. Two items the research dossier could not confirm
are the native API-versioning feature and the name and scope of the outbound
address filter (RQ-015).

Example: a client interface says "get a user by id". The generated client
calls the server, and the server's reply redirects to `http://169.254.169.254`
(a cloud metadata address). The client refuses before connecting, because the
address is private, and reports: "Blocked: the redirect led to a private
address. Nothing was sent." Another client call, to a public site, follows one
redirect and returns typed data. A server route is served as version 2 when
the request carries the header `X-Api-Version: 2`, and version 1 answers carry
a "sunset" header with the retirement date.

Values marked "proposed" are design choices to be checked by measurement.

## What it does

### Words used here

- **Route**: a path and method that a server answers.
- **API version**: a number or name that picks one of several shapes of an
  endpoint.
- **Declarative client**: a client written as an interface; the framework
  generates the calls.
- **Private address**: an address that is not on the public internet:
  loopback, link-local (including cloud metadata addresses), private
  ranges, unique-local IPv6 and similar.
- **Problem details**: the standard JSON error object (RFC 9457).
- **Backpressure**: a receiver telling a sender to slow down.
- **Starter, scout, parity row, relaxed names**: defined in specs 0081 and
  0106. **Engine Room**: the settings view with every deep control (ADR-0021).
  **Planted fault**: a deliberately bad input that a test must see refused.

### Standards named here

HTTP/1.1, HTTP/2 (RFC 9113), HTTP/3 (RFC 9114), WebSocket (RFC 6455), problem
details (RFC 9457), Sunset (RFC 8594) and Deprecation (RFC 9745) are cited
from memory and are checked by the scout. HAL (an Internet draft), HAL-FORMS
(a community specification), STOMP 1.2, RSocket, GraphQL federation and the
Jackson settings are unverified until the scout's rows confirm them.

### What this spec designs

| Feature | What the user or program sees |
|---|---|
| N-1840 Clean-room blocking and reactive HTTP servers | HTTP/1.1, HTTP/2 and HTTP/3 servers in two styles: one task per request (blocking) and streaming (reactive) |
| N-1745 First-class HTTP API versioning | Versions picked by header, path segment, query parameter or media type, with sunset headers |
| N-1746 Declarative HTTP service clients | A client interface with attributes that generates typed calls |
| N-1747 Built-in outbound SSRF protection | Every client blocks requests and redirects that reach private addresses |
| N-1748 Clean-room gRPC server and client starter | Protobuf code generation, interceptors, streaming, health checks and reflection |
| N-1749 WebSockets, STOMP, RSocket and Server-Sent Events | Real-time streams with backpressure, heartbeats and integrity guards |
| N-1750 Full GraphQL server and client engine | Schema, typed resolvers, batched loading, subscriptions and federation |
| N-1751 Hypermedia and HATEOAS link engine | Link builders and HAL, HAL-FORMS and problem details responses |
| N-1752 Clean-room serialisation engine | JSON, XML, YAML, CBOR and Smile with views, strict mode and polymorphic types |
| N-1753 Enterprise SOAP and Web Services layer | A SOAP server, WSDL generation and contract-first routing |

### Servers and routing

1. A server accepts HTTP/1.1, HTTP/2 and HTTP/3. Transport, TLS and
   protocol work come from mature crates behind our starter (AGENTS.md section
   6 rule 14); the route table, the handler model, the error shape and the
   limits are our own code.
2. Two handler styles share one route table: a blocking style (each request
   runs on a worker from a bounded pool) and a streaming style (async, with
   request and response bodies as streams). A route chooses its style; the
   pool size and the async runtime obey spec 0078.
3. Routes are declared in the composition root (spec 0106). Path variables,
   query parameters, headers and bodies are typed arguments; a type that
   fails to parse gives a problem-details error with status 400, never a
   crash.
4. The one error shape in an HTTP response is problem details (RFC 9457,
   which replaces RFC 7807). Every error from a route or a server limit is
   turned into it with a stable `type` of the form
   `urn:sutradhar:error:SUT-950`, a plain title, the SUT code and no sensitive
   data (AGENTS.md section 5). Other protocols use their own status: a gRPC
   status code, a WebSocket close code, or a typed error value for a client
   call. The table in "Failure codes" maps each code to all of them.
5. Transports: a Unix socket, a Windows named pipe and, only in a test or
   framework build, a network listener. `web.listen` picks one: `none` (the
   default), `socket`, `pipe` or `network`. The network listener lives in a
   Cargo feature (`net-listener`) that the Sutradhar build leaves out. The
   socket is created with owner-only permission (mode 0600 in an owner-only
   folder) and the server checks the peer's user id. The pipe carries an
   access list for the owner only. Every transport carries the same routes.
   Cookies, sessions and cross-origin rules belong to group PB-E; static file
   serving is out of scope for Sutradhar and is decided by the scout's rows.
6. Limits apply to every server and are not optional (see "Limits").

### API versioning

1. A route may exist in several versions. The version is read from one
   strategy chosen per route group: a header, a path segment, a query
   parameter or a media type. Unverified until the scout confirms the
   official feature; the behaviour here is the contract.
2. A request with no version gets the default version, named in the route
   group. A request with an unknown version gets problem details with the
   list of valid versions.
3. A retired or retiring version answers with a `Sunset` header (RFC 8594)
   carrying the retirement date, and a `Deprecation` header (RFC 9745,
   unverified until the scout confirms the number and wording). After the date
   the version answers 410 with problem details.
4. Version rules are in metadata so the Engine Room and an API listing can show
   them.

### Declarative clients

1. A client is an interface with attributes for method, path, headers and
   body. A macro generates a typed client. The generated code uses one shared
   client core with timeouts, connection reuse and the SSRF protection.
2. Retries, circuit breakers and timeouts policies belong to group PB-F
   (resilience); this spec provides the hook where a policy plugs in and the
   plain default: one connect timeout and one total timeout, no retry.
3. Request and response bodies use the serialisation layer below. A body that
   does not fit its type gives a typed error with the field name, never a
   partial object.
4. Every client call is logged with the target host, status, size and time,
   never with headers that carry credentials or with bodies (AGENTS.md
   section 5).

### Outbound address protection

1. Every client has the protection on, and no setting turns it off.
2. The shared piece is one pure function, `is_blocked(address)`, and one
   connect hook that calls it. Clients use both. Leviathan (spec 0082) calls
   the same function from its own connector and keeps its own resolver,
   redirect rules and request methods (spec 0082, LV-R05). Spec 0082 refers
   to this function so the range table exists once.
3. Pinning: a name is resolved once. Every returned address is classified
   before any attempt, and all blocked addresses are dropped. If none is left,
   the call fails and never falls back to another name. The socket connects to
   the checked address itself, with the original name kept for the Host header
   and for TLS name checking, so a second lookup cannot return a different
   address. The check is inside the connector, at the socket, not before the
   request.
4. The same check runs for each redirect hop, for each candidate in a
   connection race (several addresses tried in turn or at once), and for
   IPv4 addresses embedded in IPv6 forms.
5. The checker looks at the address actually used, never at the text of the
   URL. A URL whose host is a non-canonical number (`2130706433`, `0177.1`,
   `0x7f.1`) is refused (SUT-963). One URL parser, following the WHATWG URL
   standard, is used by every client; tests cover user-info tricks
   (`http://good@bad`), a trailing dot, internationalised names and an IPv6
   zone identifier (`fe80::1%eth0`), which is always refused.
6. The blocked ranges are a data table built from the IANA special-purpose
   address registries and checked into the repository with its source and date.
   It covers at least: unspecified and "this network" (0.0.0.0/8, ::), loopback
   (127.0.0.0/8, ::1), private (10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16),
   carrier-grade NAT (100.64.0.0/10), link-local (169.254.0.0/16, fe80::/10,
   which includes cloud metadata addresses), unique-local (fc00::/7), the
   protocol and documentation ranges (192.0.0.0/24, 192.0.2.0/24,
   198.51.100.0/24, 203.0.113.0/24, 198.18.0.0/15), reserved and broadcast
   (240.0.0.0/4), multicast (224.0.0.0/4, ff00::/8) and the translation ranges
   that embed IPv4 (64:ff9b::/96, 2002::/16, Teredo), judged by the embedded
   address.
7. Proxies are off by default (`client.proxy` is `none`). Environment proxy
   variables are ignored. If a proxy is configured, the check applies to the
   proxy's address, and the client refuses a proxy that can reach private
   targets unless the target is also checked by the client before the call.
8. A list of allowed addresses (`client.allow_ranges`) is empty by default. It
   holds IP ranges or literals only, never names or wildcards, is set per
   client and never globally, and each entry needs the owner's recorded
   approval: a decision record id kept in the setting's metadata. The start
   fails if an entry has no approval (spec 0106 settings checks).
9. A refused request fails with SUT-951 (Leviathan reports the same refusal as
   its own SUT-652, spec 0044) before any byte is sent. The message names the
   host and the reason, and omits a query string.
10. The name of this feature in Spring Boot, and its first release, are
    unverified (QR-004); the behaviour above is the contract.

### gRPC

1. A starter provides a gRPC server and client with code generated from
   protocol files at build time. The generator and runtime are mature crates.
2. Features: unary and streaming calls (client, server and both),
   interceptors for logging, authentication and metrics, deadlines and
   cancellation, a standard health service, and server reflection that is off in
   the product profile.
3. Messages above `grpc.max_message_bytes` are refused with a status code and
   problem details when carried over HTTP gateways.

### Real-time streams

1. Server-Sent Events, WebSockets, STOMP (a simple messaging protocol over
   WebSockets) and RSocket are offered as starters. Each stream has
   backpressure: a slow receiver slows the sender or is disconnected after a
   stated time, never causes unbounded memory.
2. Heartbeats: a connection with no traffic is pinged, and one that does not
   answer within `web.heartbeat_timeout_s` is closed.
3. Integrity guards: a maximum frame size, a maximum message size, a maximum
   number of open streams per connection and per client, checks that a stream
   obeys its protocol, and closing on the first violation.
4. A stream counts against the connection limits and the memory budget.

### GraphQL

1. Schema text is parsed and checked. Resolvers are typed functions.
2. A batched loader collects the loads of one request and runs them together,
   which removes the one-query-per-item loop.
3. Subscriptions use the real-time layer above.
4. Federation is supported as far as the scout's rows say (unverified).
5. Limits: query depth, query cost, number of aliases and request size, each
   with a setting, so a hostile query cannot exhaust memory or time.
6. Introspection is off in the product profile.

### Hypermedia

1. Link builders create links from routes, so a link never goes stale when a
   route moves. A relation cache avoids rebuilding the same link.
2. Responses may use HAL, HAL-FORMS or plain problem details. The media type
   chosen follows the request's `Accept` header.

### Serialisation

1. JSON, XML, YAML, CBOR and Smile are supported through mature crates behind
   our layer. Smile has no mature Rust crate known to us; whether to write a
   small adapter is a build-or-reuse record (QR-033).
2. Features: named views of one type (for example "public" and "admin"),
   a strict mode that fails on unknown properties, polymorphic types with an
   explicit tag, per-property settings (rename, skip, default) and date and
   number formats.
3. Limits on every format: maximum depth, size and number of entries, so a
   hostile document cannot cause a crash or a memory blow-up. XML refuses
   external entities and entity expansion beyond a small limit.
4. The settings names and the observable behaviour of Spring Boot's JSON
   settings are the parity target. The library behind them is not copied.

### SOAP and Web Services

1. A SOAP server, a WSDL generator and a contract-first router: the WSDL is the
   source, and handlers are bound to it.
2. XML limits and entity rules of the serialisation layer apply.
3. This is the lowest-priority group of this spec; it is built last.

### Limits

Limits apply to every server and every client and are not optional. They
defend against stalled senders, floods of reset streams, oversized frames,
compressed bodies that expand beyond the limit, and expansion tricks in data
formats. Defaults are proposed and are checked by measurement.

Memory bound (proposed): the largest body held in memory at one time is
capped across all connections by `web.max_inflight_body_bytes` (default 64
MiB), so the worst case is that cap plus `web.max_connections` times the header
limit and per-stream buffers. The server's share of the budget of spec 0078 is
set after the first measurement; the pass test is that resident memory stays
under the formula in a 10-minute load test at the default limits.

#### Typed protocol and client contracts (N-1745–N-1753)

Keep the explicit composition root, bounded typed inputs, 0109
authorisation and 0118 protection. Route selection, schema discovery,
transport support and framework parity grant no listener or outbound
credential. Use approved components and framework-only or synthetic
fixtures for separate protocol profiles. Sutradhar retains its
owner-authenticated local transports and existing admitted clients.

| Owner catalogue entry | Exact framework contract |
| --- | --- |
| N-1745 First-class HTTP API versioning | Resolve a declared route's version through its exact admitted path, header, query or media selector. Hold malformed, conflicting or unknown versions under the existing error mapping. Apply the approved deprecation and sunset profile and return the owning gone response for retired versions. No substitute route, hidden permission change or release effect. [FX-0107-W5-EXACT-API-VERSION-SELECTION](../planning/feature-wave-5.md#acceptance-design-for-n-1743-to-n-1766) |
| N-1746 Declarative HTTP service clients | Generate checked typed interfaces from declared methods, fields and paths at build time. Pin the exact authority, endpoints, limits and response types. Use the existing protected client, bound retries and safe typed errors. A declaration or response cannot choose a new provider, disclose a credential or widen a route. [FX-0107-W5-TYPED-CLIENT-CHECKED-SCOPE](../planning/feature-wave-5.md#acceptance-design-for-n-1743-to-n-1766) |
| N-1747 Built-in outbound SSRF protection | Reuse the shared socket-bound address decision and range table. Classify each candidate once per attempt and pin the checked address at the actual socket. Recheck every redirect, retry and candidate under current scope. Require intended-name TLS verification and no ambient proxy under BC-21, and hold on unknown or disallowed addresses. Only already approved per-client exceptions in the owning framework route apply. No extension to Leviathan's stricter guest wall, no global override, and no live probing in qualification. [FX-0107-W5-SOCKET-BOUND-DESTINATION-CHECK](../planning/feature-wave-5.md#acceptance-design-for-n-1743-to-n-1766) |
| N-1748 Clean-room gRPC server and client starter | Use the admitted protocol generator and local or synthetic profile for unary and all supported streams. Apply exact interceptor order, data and message caps, deadlines, cancellation and health. Generated code follows the owning marker and provenance. Product reflection and listeners stay disabled. [FX-0107-W5-ADMITTED-GRPC-CONTRACT](../planning/feature-wave-5.md#acceptance-design-for-n-1743-to-n-1766) |
| N-1749 WebSockets, STOMP, RSocket, and Server-Sent Events | Model each admitted protocol under the existing transport, framing, backpressure, heartbeat and close rules. Check exact actor, site, policy and lease at connection, protocol upgrade, subscription, cursor, resumption and each protected delivery. Expiry or protocol error closes the scoped flow. A stream grants no network listener, cross-site replay or action authority. [FX-0107-W5-SCOPED-STREAM-CLOSE](../planning/feature-wave-5.md#acceptance-design-for-n-1743-to-n-1766) |
| N-1750 Full GraphQL server and client engine | Validate a pinned schema and use typed resolvers, limits and request-scoped batching. Bind batching to exact request actor, site, field and current policy. Check each resolver, field and batched item, with no shared protected result across request scope. Subscriptions and optional federation require independent protected client profiles. Reconnection and emitted links confer no authority. No product introspection, arbitrary endpoint or authority through a relation. [FX-0107-W5-PROTECTED-GRAPHQL-BATCH-SCOPE](../planning/feature-wave-5.md#acceptance-design-for-n-1743-to-n-1766) |
| N-1751 Hypermedia and HATEOAS link engine | Build eligible relations and supported responses from the owning route and media profile. Derive fields and links from the current actor's authority. An absent or revoked relation is withheld. A displayed link or form grants no capability, write or confirmation. Errors contain only the existing safe problem fields. [FX-0107-W5-CURRENT-HYPERMEDIA-RELATION](../planning/feature-wave-5.md#acceptance-design-for-n-1743-to-n-1766) |
| N-1752 Clean-room serialisation engine matching Jackson 3 | Qualify each admitted JSON, XML, YAML, CBOR or Smile profile against independently specified fixtures. Bound depth, size, entries and expansion. Check typed fields, selected views, strict handling and the declared variants. Raw input grants no file or network access, arbitrary type load or protected field disclosure. Missing format support reports unavailable. [FX-0107-W5-BOUNDED-INDEPENDENT-FORMAT-PROFILES](../planning/feature-wave-5.md#acceptance-design-for-n-1743-to-n-1766) |
| N-1753 Enterprise SOAP and Web Services layer | Use a pinned reviewed contract for typed routing and generated metadata, and apply the owning XML limits and external-resource refusal. Require authorised action and exact operation for every request. Unsupported, conflicting or unadmitted contracts hold. No fetched contract, external lookup, listener or native effect from a fixture. [FX-0107-W5-PINNED-SOAP-OPERATION](../planning/feature-wave-5.md#acceptance-design-for-n-1743-to-n-1766) |

#### N-1840 Clean-room blocking and reactive HTTP servers

Keep one explicit typed route table, the bounded worker and stream
profiles, and the same authorisation and error contract across
admitted HTTP versions. Use mature qualified transport components.
Cancel or drain scoped work under the owning lifecycle. The
framework network listener needs the separate build and test
profile. Sutradhar retains the owner-only local socket or pipe
and current peer checks. No server profile grants a product port.
[N-1840](../planning/feature-wave-5.md#acceptance-design-for-n-1839-to-n-1862)
uses fixture `FX-0107-W5-BOUNDED-HTTP-PROFILES-NO-PRODUCT-PORT`.

### Data

- Parity ledger and behaviour descriptions: repository files (spec 0081).
- Route metadata and API listing: generated with the build.
- Generated client and gRPC code: generated at build time and marked
  generated (AGENTS.md section 8.5).

### Settings

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `web.listen` | none, socket, pipe, network | none | four values | next start |
| `web.max_body_bytes` | bytes | 2,097,152 | 1,024 to 1,073,741,824 | next start |
| `web.max_inflight_body_bytes` | bytes | 67,108,864 | 1 MiB to 4 GiB | next start |
| `web.max_header_bytes` | bytes | 16,384 | 1,024 to 65,536 | next start |
| `web.max_headers` | whole number | 100 | 10 to 500 | next start |
| `web.max_uri_bytes` | bytes | 8,192 | 256 to 65,536 | next start |
| `web.header_read_timeout_s` | seconds | 10 | 1 to 300 | next start |
| `web.body_read_timeout_s` | seconds | 30 | 1 to 3,600 | next start |
| `web.request_timeout_s` | seconds | 30 | 1 to 3,600 | next start |
| `web.idle_timeout_s` | seconds | 60 | 5 to 3,600 | next start |
| `web.max_connections` | whole number | 256 | 1 to 65,536 | next start |
| `web.h2_max_streams` | whole number | 100 | 1 to 1,000 | next start |
| `web.h2_max_resets_per_s` | whole number | 50 | 1 to 1,000 | next start |
| `web.max_expansion_ratio` | ratio | 20 | 1 to 1,000 | next start |
| `web.http3` | on or off | off | two values | next start |
| `web.heartbeat_timeout_s` | seconds | 30 | 5 to 600 | next start |
| `stream.max_message_bytes` | bytes | 1,048,576 | 1,024 to 67,108,864 | next start |
| `stream.max_per_connection` | whole number | 100 | 1 to 10,000 | next start |
| `stream.max_per_client` | whole number | 20 | 1 to 1,000 | next start |
| `client.connect_timeout_s` | seconds | 10 | 1 to 300 | next call |
| `client.timeout_s` | seconds (total) | 30 | 1 to 3,600 | next call |
| `client.max_redirects` | whole number | 5 | 0 to 20 | next call |
| `client.proxy` | none or a proxy address | none | any | next call |
| `client.allow_ranges` | list of IP ranges | empty | any, each with owner approval | next call |
| `grpc.max_message_bytes` | bytes | 4,194,304 | 1,024 to 268,435,456 | next start |
| `graphql.max_depth` | whole number | 12 | 1 to 100 | next start |
| `graphql.max_cost` | whole number | 1,000 | 1 to 100,000 | next start |
| `graphql.max_aliases` | whole number | 15 | 1 to 200 | next start |
| `graphql.max_batch` | whole number | 10 | 1 to 100 | next start |
| `graphql.introspection` | on or off | off | two values | next start |
| `serial.strict` | on or off | on | two values | next start |
| `serial.max_depth` | whole number | 64 | 4 to 1,000 | next start |
| `serial.max_bytes` | bytes | same as `web.max_body_bytes` | 1,024 to 1 GiB | next start |
| `serial.max_entries` | whole number | 100,000 | 100 to 10,000,000 | next start |
| `serial.max_alias_expansion` | whole number | 1,000 | 1 to 100,000 | next start |
| `serial.xml_max_entity_expansions` | whole number | 100 | 0 to 10,000 | next start |

There is no setting that turns off the outbound address protection, the body
limit or the rule that refuses external XML entities. The Unix socket path and
the pipe name come from the home folder layout of spec 0030, not a setting.

### Failure codes

Each code is returned as problem details in an HTTP response, as a status in
another protocol, and as a typed error for a client call.

| Code | Meaning | HTTP | gRPC | WebSocket close |
|---|---|---|---|---|
| SUT-950 | Request over a limit | 413 or 431 | RESOURCE_EXHAUSTED | 1009 |
| SUT-951 | Outbound address refused | 502 | UNAVAILABLE | 1008 |
| SUT-952 | Version unknown or retired | 400 or 410 | UNIMPLEMENTED | 1008 |
| SUT-953 | Body does not match its type | 400 or 422 | INVALID_ARGUMENT | 1007 |
| SUT-954 | Stream protocol violation | 400 | INTERNAL | 1002 |
| SUT-955 | Slow receiver disconnected | 408 | DEADLINE_EXCEEDED | 1008 |
| SUT-956 | Query over its depth, cost, alias or batch limit | 400 | INVALID_ARGUMENT | 1008 |
| SUT-957 | Document over a format limit | 413 or 400 | INVALID_ARGUMENT | 1009 |
| SUT-958 | XML entity refused | 400 | INVALID_ARGUMENT | 1007 |
| SUT-959 | Message over its size limit | 413 | RESOURCE_EXHAUSTED | 1009 |
| SUT-960 | Stalled request (slow headers or body) | 408 | DEADLINE_EXCEEDED | 1008 |
| SUT-961 | Reset-stream flood | none (connection closed) | UNAVAILABLE | 1008 |
| SUT-962 | Expansion ratio exceeded | 413 | RESOURCE_EXHAUSTED | 1009 |
| SUT-963 | Address literal not canonical or has a zone | 400 | INVALID_ARGUMENT | 1008 |

### Security and privacy

- Untrusted input from a network is parsed only by the limited parsers above.
- Logs hold hosts, statuses and sizes, never bodies or credential headers.
- Reflection and introspection are off by default in the product profile.
- Errors never carry stack traces, file paths or secrets.

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| BC-01 | A server MUST serve the same routes over a Unix socket and a named pipe, and, in a test build only, a network listener. | IT-BC-01 |
| BC-02 | Every error in an HTTP response MUST be problem details with a SUT code and no sensitive data; other protocols MUST use the mapped status. | UT-BC-01, FZ-BC-01 |
| BC-03 | Every server MUST enforce the limits in the settings table, including stalled senders, reset floods and expansion ratio, and refuse early. | UT-BC-02, FI-BC-01 |
| BC-04 | A route MUST pick its version by the chosen strategy (header, path, query, media type), and a retired version MUST answer 410 after its sunset date. | UT-BC-03, BD-BC-01 |
| BC-05 | A client MUST resolve a name once, classify every address, connect only to a checked address and repeat the check at each redirect and candidate. | UT-BC-04, PL-BC-01, PL-BC-04 |
| BC-06 | The outbound protection MUST NOT be switchable off; an allowed range MUST be per client, an IP range, and carry an approval, or the start MUST fail. | UT-BC-05 |
| BC-07 | Leviathan and every client MUST use the one `is_blocked` function and range table. | IT-BC-02 |
| BC-08 | A body that does not fit its type MUST fail with the field name and no partial object. | UT-BC-06 |
| BC-09 | A stream receiver that is slow for more than its time limit MUST be closed and memory MUST stay under the stream limits. | FI-BC-02 |
| BC-10 | A stream MUST be closed on its first protocol violation. | FZ-BC-02 |
| BC-11 | A GraphQL query over its depth, cost, alias or batch limit MUST be refused. | UT-BC-07 |
| BC-12 | A batched loader MUST turn N loads in one request, including nested fields, into one batch per type. | UT-BC-08 |
| BC-13 | Every serialisation format MUST enforce depth, size, entry and alias-expansion limits, and XML MUST refuse external entities. | UT-BC-09, FZ-BC-03 |
| BC-14 | Strict mode MUST fail on unknown properties and named views MUST show only their fields. | UT-BC-10 |
| BC-15 | Reflection, introspection and any network listener MUST be off in the product profile. | UT-BC-11 |
| BC-16 | A client call MUST log host, status, size and time and MUST NOT log credentials or bodies. | UT-BC-12 |
| BC-17 | Each feature MUST have parity rows, behaviour descriptions and independent contract tests. | CT-BC-01 |
| BC-18 | The socket and pipe MUST be owner-only, and a peer of another user MUST be refused. | UT-BC-13, PL-BC-05 |
| BC-19 | A non-canonical address literal, a zone identifier and a user-info trick MUST be refused. | UT-BC-14 |
| BC-20 | The blocked range table MUST be data, cite its source and date, and match the IANA registries in a test. | UT-BC-15 |
| BC-21 | Proxies MUST be off by default, and environment proxy variables MUST be ignored. | UT-BC-16 |
| BC-22 | A server MUST stop accepting and finish in-flight requests within `boot.shutdown_timeout_s` (spec 0106). | IT-BC-03 |
| BC-23 | Hypermedia responses MUST follow the request's media type and build links from routes. | UT-BC-17 |
| BC-24 | A gRPC server MUST support unary and all streaming kinds, deadlines, interceptors and health, with reflection off in the product profile. | IT-BC-04 |
| BC-25 | SOAP and WSDL handling MUST apply the XML limits and entity rule. | UT-BC-18 |
| BC-26 | Generated client and gRPC code MUST carry the generated marker of AGENTS.md section 8.5. | UT-BC-19 |
| BC-27 | Resident memory MUST stay under the stated bound in the load test. | MS-BC-01 |
| BC-28 | The HTTP/3 switch MUST enable and disable HTTP/3 and MUST fall back to HTTP/2. | IT-BC-05 |

## How it is tested

Designed, not run.

- **Unit (UT-BC):** every limit with one value just inside and one just
  outside; the address filter with a table of public and private addresses in
  every notation; version selection by each strategy; strict mode and views;
  loader batching; format limits; log content.
- **Planted faults (PL-BC):** a redirect to a metadata address, a name that
  resolves to a private address after lookup, and an IPv4-mapped form; each
  must be refused with SUT-951, and an honest public call must pass.
- **Behaviour (BD-BC):** the scenario above: a refused redirect, a followed
  redirect, version 2 by header and the sunset on version 1.
- **Integration (IT-BC):** the same route on all three transports; the shared
  filter in Leviathan and in a client.
- **Fault injection (FI-BC):** stalled headers and bodies, a reset-stream
  flood, a compressed body that expands past the limit, a slow stream receiver.
- **Fuzz (FZ-BC):** malformed HTTP, frames, protocol messages and documents in
  every format never crash a parser or leak data.
- **Added planted faults:** a DNS answer that changes between two lookups
  (PL-BC-04); a peer of another user on the socket (PL-BC-05).
- **Contract (CT-BC):** one test per parity item, written from the behaviour
  description by someone other than the implementer.
- **Measurement (MS-BC-01):** a 10-minute load test of one route on the 8 GiB
  reference with the default limits, checking resident memory against the
  stated bound; client connection reuse.

## Changes from the source

The pillar list asked for a clean-room serialisation engine "matching Jackson
3". Behemoth Boot instead matches the observable behaviour of Spring Boot's
JSON settings and uses mature format crates (AGENTS.md section 6 rule 14).
Spec 0082's private-address rule now points to the shared `is_blocked`
function here and keeps its own resolver and redirect rules (LV-R05). SOAP
stays in scope, built last.
