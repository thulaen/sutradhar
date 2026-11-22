# 0111: Behemoth Boot batch, cloud patterns, AI, MCP and mail

This is group PB-G of Behemoth Boot (spec 0081). It owns six families: the
batch engine for large jobs, the cloud patterns (a config server, service
discovery, an API gateway and secret-vault bridges), the AI orchestration
layer, the macros that build MCP servers and clients, and mail with template
rendering. Spec 0081 owns the ledger and clean-room rules, spec 0106 the core,
0107 the web layer, 0108 data access, 0109 security, 0110 messaging,
scheduling and resilience, and group PB-H will own health and metrics.

Sutradhar uses little of this. It runs no config server, no discovery and no
gateway: it is one program on one PC with an owner-only socket (ADR-0021). Its
secrets live in its own secret store (spec 0065). Its MCP server is the local
one of spec 0020. Its local models belong to the Model Hub (spec 0080) and its
memory scheduler (spec 0078). It does use batch work for large crawls and
reports, and mail to send alerts and reports through the owner's own accounts.
Everything else is framework parity: built, tested and kept in step with
Spring, and off in the Sutradhar profile.

Spring facts here come from the owner's pillar list (19 November 2026) and are
"unverified" until the scout confirms each against official notes (spec 0081).
That includes the version names "Spring Batch 6", "Spring AI 2" and "MCP 2.0".
The behaviour description, not the Spring wording, is the contract. The pillar
list asked for clean-room engines; mature crates sit behind our starters
(ADR-0022, AGENTS.md section 6 rule 14).

Example: a report job reads 3 million page rows in chunks of 500, scores them,
and writes results. At row 1.2 million the PC sleeps. After waking, the job
resumes from its last checkpoint, not from the start. Eleven rows fail to parse;
the skip policy skips them, lists them, and the job ends "completed with 11
skipped". An alert email built from a template goes to the owner's own address
through the owner's own mail account, with every value escaped. A tool written
with an MCP attribute appears on the local MCP server with its schema, and is
refused unless the agent's token allows it.

Values marked "proposed" are design choices to be checked by measurement.

## What it does

### Words used here

- **Chunk**: a fixed number of items read, processed and written together.
- **Checkpoint**: a saved position from which a job can restart.
- **Skip policy**: which errors an item may have without stopping the job.
- **Partition**: a slice of a job's data processed on its own.
- **Config server**: a program that serves settings to other programs.
- **Gateway**: a reverse proxy that routes and filters requests.
- **Vault**: a store for secrets.
- **Advisor, guard**: a step that adds memory to, or checks, an AI prompt.
- **Template**: text with holes that are filled from data.
- **Starter, composition root**: defined in specs 0081 and 0106.

### What this spec designs

| Feature | What the user or program sees |
|---|---|
| N-1847 Full clean-room batch processing engine (Spring Batch 6, unverified) | Chunked jobs with readers, processors, writers, checkpoints, skip policies and partitions |
| N-1778 Centralised config server with live refresh | Versioned settings served from Git, vaults or folders, pushed to running programs |
| N-1779 Service discovery and smart client load balancing | Services found through DNS, Consul, etcd or Kubernetes, with zone-aware balancing |
| N-1780 Clean-room API gateway engine | A reverse proxy with route rules, header rewrites, token relay, breakers and rate limits |
| N-1781 Distributed configuration and secret-vault bridges | Secrets pulled from outside vaults without plain-text files |
| N-1782 Clean-room AI orchestration layer (Spring AI 2, unverified) | One interface for chat, embeddings, structured output, tools, vector stores, memory and prompt guards |
| N-1848 First-class MCP 2.0 (unverified name) server and client macros | Attributes that turn functions into MCP tools, resources and prompts |
| N-1783 Clean-room email and template rendering engines | SMTP and IMAP clients and safe text and HTML templates |

### Batch engine

1. A job is a list of steps. A chunk step has a reader, an optional processor
   and a writer, a chunk size (`batch.chunk_size`) and a byte cap
   (`batch.chunk_max_mib`): a chunk ends when either is reached. A single item
   larger than the byte cap fails with SUT-1115. A step can also be a plain
   task.
2. Memory is bounded by bytes, not only by item count. Each worker draws a byte
   grant from the budget of spec 0078 before it reads, so a job's memory is the
   number of workers times the chunk byte cap. Readers stream; none loads a
   whole table.
3. A batch job is a job class of spec 0019 and runs under the scheduler and
   locks of spec 0110 (the same job with the same identifying parameters
   cannot run twice at once, SUT-1100). The `batch_*` tables are detail rows
   under that job: definition, step plan, checkpoints and skips. The Jobs
   screen (spec 0099) reads run history from `schedule_runs` and the detail
   from these tables through the job listeners.
4. Writers declare whether they are idempotent. A PostgreSQL writer writes the
   chunk and its checkpoint in one transaction, so its effects happen exactly
   once. Any other writer (a file, an outside service, the vector index) gives
   at-least-once effects: it must be idempotent, through a key per item or
   chunk, or by writing a temporary file and renaming it. A non-idempotent
   writer outside PostgreSQL is refused at build time. If the checkpoint cannot
   be written the job stops and never continues (SUT-1103).
5. Restart: a failed or stopped job restarts from its last checkpoint.
6. Retry then skip. An item error is retried first, through the resilience layer
   of spec 0110, then skipped if it is on the step's skip list. A chunk whose
   transaction rolls back is processed again one item at a time so only the
   failing item is skipped. The skip limit (`batch.skip_limit`) counts per step
   per run. A skipped item is recorded with its error. Passing the limit fails
   the step (SUT-1102).
7. Partitioning: a step may split its data into partitions by key range, run in
   parallel up to `batch.max_workers` and the admission rules of spec 0078. The
   partition plan is stored in `batch_steps` when the step first starts and is
   reused on restart, so a changed table cannot change the plan mid-job. Each
   partition has its own checkpoint. When one partition fails, the others
   finish their current chunk and checkpoint, then the step ends as failed.
8. Job listeners report start, chunk, skip, retry and end, with counts.
9. A job's writes are proposals or ordinary data writes under the same rules as
   any other code; a batch job cannot change a site without a human accept.

### Config server and live refresh

1. A server serves settings by application, profile and label (a version). The
   sources are a Git repository, a vault or a folder. The server returns the
   settings of spec 0106 with their metadata. Every config response
   masks or omits secrets, and refresh payloads carry no raw secrets.
   Callers must authenticate (mutual TLS or a
   token under spec 0109); an unauthenticated request is refused.
2. A client loads remote settings as an admitted layer under spec 0106.
   That spec owns the refreshable flag, source authority and validation.
3. Use spec 0106's complete candidate check, atomic generation swap
   and guarded rollback. Invalid or stale candidates are refused
   (SUT-1101). Conflicting attempts leave the active generation intact.
4. Push only prompts a fetch and validation. Clients also poll (`config.poll_s`,
   which
   takes effect at the next poll). Transport is TLS; the outbound address
   protection of spec 0107 applies to every fetch.
5. Off in Sutradhar, which uses local files and history (spec 0014).

Spring Cloud 2026.0.0 places Bus under Stream 5.1.0 and keeps its
existing artefact coordinates. Record that provenance on the affected
0081 rows and preserve earlier release records. For any source-proved
configuration refresh, 0111 owns the refresh and validation boundary.
Spec 0110 owns any independently evidenced messaging delivery on that
path. Confirm the exact observable behaviour and profile before
deriving a task.
[RQ-015](../research/dossier.md#spring-cloud-202600-ga-component-list)

### Service discovery and load balancing

1. Instances register with a name, address, zone and health; clients discover
   them through DNS, Consul, etcd or Kubernetes. Each source is a starter behind
   one trait.
2. A client balances across healthy instances (round robin, least in flight, and
   zone-aware preference through `discovery.zone`). An instance is healthy after
   `discovery.healthy_after` consecutive passed probes and unhealthy after
   `discovery.unhealthy_after` consecutive failures, probed every
   `discovery.probe_s` seconds. No healthy instance fails with SUT-1104.
3. Every address found is resolved once, checked by the outbound address
   protection of spec 0107 and connected to as checked, unless it is on the
   allowed ranges of that client.
4. Off in Sutradhar.

### API gateway

1. A gateway is a reverse proxy built on the server and client of spec 0107. A
   route has predicates (path, method, header, host, time), filters (rewrite
   path, add or remove headers, strip a prefix) and a target.
2. Filters include token relay (pass the caller's token on, only to the hosts in
   `gateway.relay_hosts`), circuit breakers and per-client rate limits through
   spec 0110, and size limits through spec 0107.
3. Hop-by-hop headers are removed: Connection, Keep-Alive, TE, Trailer,
   Transfer-Encoding, Upgrade, Proxy-Authenticate, Proxy-Authorization and any
   header named in Connection. A request with ambiguous framing (both
   Content-Length and Transfer-Encoding, or repeated conflicting lengths) is
   refused to prevent request smuggling. Forwarded-for headers are set by one
   rule, and a request that would loop back to the gateway is refused
   (SUT-1105).
4. The target address is resolved once and the check of spec 0107 applies after
   resolution and at connect time.
5. The Sutradhar product has neither a remote config client nor server.

### Secret-vault bridges

1. A bridge reads a secret from HashiCorp Vault, AWS Secrets Manager, Azure
   Key Vault or a local encrypted key store (`vault.provider`) and exposes it
   as a secret setting (spec 0106) using the secret type of spec 0065: a value
   that is wiped when dropped, never printed by `Debug` or logging, and never
   written to a file or a report. It is cached only for `vault.cache_ttl_s`.
2. The vault bridge renews an admitted cached secret without a config
   refresh or changed binding. The old and new values may overlap for
   `vault.rotation_overlap_s`. Never replay a revoked or expired secret.
   An unreachable vault
   or a missing secret fails with SUT-1106 and retries with backoff through
   spec 0110; the app keeps the cached value until it expires and then fails
   closed.
3. The credentials that reach a vault are an explicit list: the secret store of
   spec 0065 in Sutradhar. Environment variables, instance roles and profile
   files are not used by default, because they keep credentials outside the
   secret store.
4. Each bridge uses the user's own account; the owner switches each on
   (ADR-0021: outside services are optional and off by default).
5. The local key store option follows QR-036.

### AI orchestration

1. One interface covers chat models, embeddings, structured output (the model's
   answer must fit a typed schema or fail with SUT-1107), tool calling, vector
   stores, memory advisors that add chat history and prompt guards.
2. Providers are starters. In Sutradhar the local ONNX and GGUF models of the
   Model Hub (spec 0080) are the default providers, admitted and scheduled by
   spec 0078; an AI call never preempts live work. `ai.timeout_s` may not exceed
   the admission timeout of specs 0078 and 0080, and a call runs off the
   request path and can always be cancelled. A provider that uses an outside
   service uses the owner's own account and is off until the owner switches it
   on, and it never receives site text unless that switch is on (ADR-0021).
3. Tool calling: a model acts under the token of the caller that invoked it and
   never gains rights of its own. It may call only tools that are registered and
   allowed for that token (spec 0020). A tool that changes anything creates a
   proposal. Tool arguments are checked against the tool's schema first
   (SUT-1108).
4. Prompt guards are defence in depth, not a boundary. Input and retrieved text
   are untrusted data and never instructions (AGENTS.md section 5). A guard
   marks or strips instruction-like text, caps the prompt size and refuses a
   request that contains a value held in the secret store or matching a secret
   pattern (SUT-1109). The real boundary is that tools need a token and produce
   proposals only. The guard is measured on a fixed corpus: it must mark at
   least the stated share of the injection cases and flag no more than the
   stated share of a clean set (thresholds set at the first measurement).
5. Vector stores use the embedded Rust vector index of ADR-0021 behind a trait;
   other stores are optional starters, off in Sutradhar.
6. Local language models never serve live ranking and never build a reply to a
   site (ADR-0021).
7. Limits: a timeout, input tokens (`ai.max_prompt_tokens`), output tokens
   (`ai.max_output_tokens`) and, for outside providers only, a daily token cap
   (`ai.daily_tokens`) with a cost meter. Token use is recorded per call.
8. Input and output are logged only masked and only when the owner switches
   that on (`ai.log_io`), kept `ai.log_retention_days` and removed by erasure
   requests (spec 0032).
9. Evaluation: each AI starter ships a small fixed test set of cases so a
   change in behaviour is noticed. It runs in the test gate; the quality
   judging of spec 0057 is for ranking and is not used here.

### MCP macros

1. Attributes `mcp_tool`, `mcp_resource` and `mcp_prompt` (names are our own
   choice, not Spring's) turn a function into an MCP tool, resource or prompt.
   The macro derives the input schema from the function's types and the
   description from its documentation. The build fails on a name clash, an
   unsupported parameter type, a missing description or a tool with no declared
   token scope.
2. Transports: standard input and output, and a streamable HTTP transport (both
   names unverified). Both are in the framework. In Sutradhar only the owner's
   own `sutra mcp` standard-input program and the owner-only socket are allowed;
   there is no network port and no remote bridge (ADR-0021, spec 0020). The HTTP
   transport exists in test builds and for other programs.
3. On standard input there is no peer to authenticate. The trusted caller is
   the owner's own operating system user who started the program, and the
   session still presents the token configured for it (spec 0020). A macro-
   built server never skips the token or the security chain and method rules
   of spec 0109. For the HTTP transport, OAuth 2.0 is spec 0109's resource
   server.
4. An MCP client macro generates a typed client for another MCP server, with the
   outbound protection of spec 0107.
5. Tracing: each call carries the trace context of spec 0110 and is audited
   (spec 0032) without secrets. A refusal by the chain is SUT-1112.
6. Spec 0020 owns which actions Sutradhar exposes, their tokens and their
   confirm rules. This spec owns only the framework macros.

### Mail and templates

1. Mail is a service of spec 0099: it is off until `mail.enabled` is switched
   on by the owner, the recipient list belongs to spec 0099's alert centre,
   and the approved send steps of spec 0099 (JW-19) apply, so a workflow's
   mail needs the owner's approval for that workflow.
2. SMTP sending uses TLS with certificate checking (`mail.tls_mode`: implicit
   TLS or required STARTTLS; a downgrade to plain text is refused) and the
   owner's own account, with credentials from the secret store. IMAP reading
   is framework-only and absent from the Sutradhar build; a build test proves
   no IMAP client is linked into Sutradhar (BG-40).
3. A message is built from a template and a data object. Header values are
   checked for line breaks (no header injection), addresses are validated, and
   attachments have a size limit and an allowed type list; file names are
   cleaned of paths. Mail bodies pass through the masking rules, so no secret or
   needless personal data is sent. Sending is rate limited
   (`mail.rate_per_min`, `mail.daily_max`) and goes through the resilience layer
   of spec 0110. A failed send raises an alert and does not loop.
4. Sutradhar sends mail only to the configured recipients, for alerts and
   reports, and only from the owner's account. There is no bulk or marketing
   mail. SPF and DKIM belong to the owner's mail provider and are out of scope.
5. Template engines: a text and HTML engine behind one interface, chosen by a
   build-or-reuse record. Escaping is automatic and depends on the place: HTML
   text, HTML attribute, URL and script contexts each have their own rule, and a
   value placed in a context with no safe rule is refused. Text templates
   (subjects, plain text) strip line breaks. A raw (unescaped) value needs an
   explicit mark, and the gate flags every use.
6. Templates cannot run code, read files or reach the network. Includes come
   only from the fixed template folder, which only the owner can write, and an
   include cycle is refused. Data given to a template cannot hold a secret
   type. A template has a maximum size, nesting depth, output size and render
   time (`template.max_ms`), and exceeding any is SUT-1111.
7. PDF output from templates needs a rendering engine. We compile no C or C++
   (ADR-0021); it is blocked until a mature, licence-clean, prebuilt engine is
   chosen (QR-039, SUT-1113).

### Reuse

Our own code is the job repository and checkpoint logic, the refresh rules,
the gateway filters, the secret handling, the AI interface and its guards, the
MCP macros, and the mail safety rules. Mature crates sit behind them: Git,
vault and cloud clients, DNS and service registries, HTTP, SMTP, IMAP,
template syntaxes, schema derivation and PDF output where it exists. Each
choice is a build-or-reuse record in the research dossier.

#### Optional framework starters and product effects (N-1778–N-1783)

Use existing framework profile and parity admission under 0081.
Test with isolated synthetic scopes and the currently admitted
test harness. Keep the four BG-22 starters out of the product
binary, with no hidden config, restore, CLI or MCP path. The
product uses local approved settings, tijori custody and the
protected local door. An available framework feature or transport
shape gives no product networking, credential, provider or spend
authority. Apply current routes and approval under QR-215.

| Feature | Exact framework behaviour and product boundary | Existing requirements |
|---|---|---|
| N-1778 Centralised config server with live refresh | An admitted framework server authenticates the exact application, profile and version request before releasing allowed metadata. Refresh notices only propose a fetch or candidate. Apply the 0106 whole-candidate validation, atomic generation swap and guarded rollback. No raw secret or remote config server or client in Sutradhar. | BG-07, BG-22, BG-27, BG-28 · [FX-0111-W5-WHOLE-CONFIG-GENERATION-SWAP](../planning/feature-wave-5.md#acceptance-design-for-n-1767-to-n-1790) |
| N-1779 Service discovery and smart client load balancing | Use an admitted framework source and exact eligible endpoint, health and zone profile. Check the actual resolved address through 0107 and refuse stale, unhealthy or unavailable evidence. A registry record or health response cannot grant the client access to a different endpoint. No product discovery starter. | BG-08, BG-22, BG-29 · [FX-0111-W5-CURRENT-CHECKED-DISCOVERY-ENDPOINT](../planning/feature-wave-5.md#acceptance-design-for-n-1767-to-n-1790) |
| N-1780 Clean-room API gateway engine | In an admitted framework route bind method, target, caller, size, scope and any expressly allowed token relay. Refuse ambiguous framing, loops, unapproved address, header or credential forwarding and over-budget input. Route matching and redirects confer no authority. Sutradhar has no gateway or proxy listener. | BG-09, BG-22, BG-30 · [FX-0111-W5-SCOPED-FRAMEWORK-GATEWAY](../planning/feature-wave-5.md#acceptance-design-for-n-1767-to-n-1790) |
| N-1781 Distributed configuration and secret-vault bridges | Use only an admitted framework provider, exact purpose, credential handle and current consent and cache generation. Apply bounded expiry and rotation with no plaintext persistence or unproved fallback. A cached value cannot outlive revocation or drift to a different route. Product keeps 0065 custody without an outside vault bridge. | BG-10, BG-22, BG-31 · [FX-0111-W5-BOUNDED-FRAMEWORK-VAULT-CUSTODY](../planning/feature-wave-5.md#acceptance-design-for-n-1767-to-n-1790) |
| N-1782 Clean-room AI orchestration layer | Bind exact model, role, caller grant, input and output scopes, route and resource caps. Treat retrieved text and model output as inert. Validate typed results and proposals separately under 0020 and 0091. Bind each tool to the current invoking token and actual eligible route. No borrowed orchestration authority. Hold narrowed or expired grant, overdue admission, exceeded release cap or spend cap. Local Model Hub evidence stays default. A paid or off-PC route requires its own current admission, owner approval, egress permission and spend bounds. Naming a provider or tool adds no call, publish, ranking or fallback authority. | BG-11 to BG-15, BG-32, BG-33, BG-39 · [FX-0111-W5-CALLER-SCOPED-AI-PROPOSAL](../planning/feature-wave-5.md#acceptance-design-for-n-1767-to-n-1790) |
| N-1783 Clean-room email and template rendering engines | Use bounded inert templates, context escaping and safe metadata and attachments. The product admits only its separately approved 0099/0100 alerts and reports through a configured owner account and permitted recipients. Require the enabled owner route and exact send approval, current destination, TLS and rate limits. One effect per approved recipient binds frozen envelope and header addresses and body and attachment digests. Loss after final DATA may be unknown. Skip accepted and unknown recipients on replay. No reroute. An approved setup test is synthetic and single-recipient and fixtures make no real send. Framework IMAP and any unqualified PDF path stay unavailable in the product. DR-116 admits no outreach. | BG-17 to BG-20, BG-35 to BG-38 · [FX-0111-W5-APPROVED-MAIL-RECIPIENT-REPLAY](../planning/feature-wave-5.md#acceptance-design-for-n-1767-to-n-1790) |

#### Batch and local MCP contracts (N-1847–N-1848)

| Owner catalogue entry | Exact framework contract |
| --- | --- |
| N-1847 Full clean-room batch processing engine | Pin reader, processor and writer contracts, scoped input, exact chunks, byte grant, partition plan, failure policy and occurrence. An optional processor transforms admitted input into bounded typed data only. Ordinary commits follow the declared writer authority. Native site effects need their separate human acceptance. Preserve BG-02 atomic PostgreSQL chunk and checkpoint and BG-23 cap. Refuse a non-idempotent non-PostgreSQL writer. Missing checkpoints, uncertain effects and stale grants hold before retry, skip or resume. No native write without the owning human decision. [FX-0111-W5-BATCH-BOUNDED-CHUNK-AND-CHECKPOINT](../planning/feature-wave-5.md#acceptance-design-for-n-1839-to-n-1862). |
| N-1848 First-class MCP 2.0 server and client macros | Generate a typed schema from the declared function and registered scope at build time. Deny clashes and unsupported types or transports. Treat the upstream protocol label as subject to 0081 verification. The product's existing local launch and owner-only socket or pipe, enrolled peer, token and 0020 policy govern every call. Framework-only HTTP grants no product listener or bridge. [FX-0111-W5-TYPED-LOCAL-MCP-SCOPE-CHECK](../planning/feature-wave-5.md#acceptance-design-for-n-1839-to-n-1862). |

### Data

- Batch tables (with site column and forced row-level security): `batch_jobs`,
  `batch_steps`, `batch_checkpoints`, `batch_skips`.
- Ledger and behaviour descriptions: repository files (spec 0081).
- Templates: files in the home folder, listed in the file ledger (spec 0030).

### Settings

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `batch.chunk_size` | whole number | 500 | 1 to 100,000 | next job |
| `batch.chunk_max_mib` | MiB | 16 | 1 to 1,024 | next job |
| `batch.skip_limit` | whole number | 100 | 0 to 1,000,000 | next job |
| `batch.max_workers` | whole number | 2 | 1 to 16 | next job |
| `config.poll_s` | seconds | 60 | 5 to 86,400 | next poll |
| `config.refresh_enabled` | on or off | off | two values | next start |
| `discovery.enabled` | on or off | off | two values | next start |
| `discovery.zone` | text | none | any | next start |
| `discovery.probe_s` | seconds | 10 | 1 to 600 | next start |
| `discovery.healthy_after` | whole number | 2 | 1 to 20 | next start |
| `discovery.unhealthy_after` | whole number | 3 | 1 to 20 | next start |
| `gateway.enabled` | on or off | off | two values | next start |
| `gateway.relay_hosts` | list of hosts | empty | exact hosts | next start |
| `gateway.max_body_mib` | MiB | 2 | 1 to 1,024 | next start |
| `gateway.rate_per_min` | whole number | 600 | 1 to 1,000,000 | next start |
| `vault.enabled` | on or off | off | two values | next start |
| `vault.provider` | one of the four providers | none | four values | next start |
| `vault.cache_ttl_s` | seconds | 300 | 0 to 86,400 | next refresh |
| `vault.rotation_overlap_s` | seconds | 60 | 0 to 3,600 | next refresh |
| `ai.timeout_s` | seconds | 120 | 1 to 600 (proposed) | next call |
| `ai.max_prompt_tokens` | whole number | 8,192 | 1 to 1,000,000 | next call |
| `ai.max_output_tokens` | whole number | 2,048 | 1 to 1,000,000 | next call |
| `ai.daily_tokens` | whole number (outside providers only) | 100,000 | 0 to 100,000,000 | next call |
| `ai.log_io` | on or off | off | two values | next call |
| `ai.log_retention_days` | days | 7 | 1 to 365 | next night |
| `mcp.http_enabled` | on or off | off | two values | next start |
| `mail.enabled` | on or off | off | two values | next start |
| `mail.tls_mode` | implicit or starttls | implicit | two values | next start |
| `mail.rate_per_min` | whole number | 10 | 1 to 600 | next start |
| `mail.daily_max` | whole number | 200 | 1 to 100,000 | next start |
| `mail.max_attachment_mib` | MiB | 10 | 0 to 100 | next start |
| `template.max_output_kib` | KiB | 1,024 | 1 to 65,536 | next start |
| `template.max_depth` | whole number | 32 | 1 to 256 | next start |
| `template.max_ms` | milliseconds | 2,000 | 10 to 60,000 | next start |

There is no setting that turns off secret masking, output escaping by default,
the tool schema check, or the rule that AI output is a proposal.

### Failure codes

SUT-1100 batch job already running; SUT-1101 remote settings refused (invalid,
old kept); SUT-1102 skip limit passed; SUT-1103 checkpoint write failed;
SUT-1104 no healthy instance; SUT-1105 gateway route refused (loop or size);
SUT-1106 vault not reachable or secret missing; SUT-1107 model answer did not
fit its schema; SUT-1108 tool call refused; SUT-1109 prompt guard stopped a
request; SUT-1110 mail refused (address, header or size); SUT-1111 template
over its limits; SUT-1112 MCP tool refused by the chain; SUT-1113 PDF engine
blocked; SUT-1115 one item larger than the chunk byte cap. Codes SUT-1100 to
SUT-1199 are reserved for this spec.

### Security and privacy

- No secret, token, mail credential or prompt secret appears in a log, a job
  record, a template output or an error.
- Text from a model, a mail, a template data field or a retrieved page is
  data, not an instruction.
- Nothing here sends site content off the PC unless the owner's switch for that
  destination is on (ADR-0021).
- A batch job, a template, a tool or a model can only create proposals for site
  changes; a person accepts every site edit.

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| BG-01 | A batch step MUST hold at most one chunk per worker in memory and MUST stream its input. | UT-BG-01, MS-BG-01 |
| BG-02 | A checkpoint MUST be written in the same transaction as its chunk's writes. | IT-BG-01, FI-BG-01 |
| BG-03 | A stopped or failed job MUST restart from its last checkpoint and not repeat or lose a chunk's effects. | FI-BG-02 |
| BG-04 | The same job with the same parameters MUST NOT run twice at once. | UT-BG-02 |
| BG-05 | Skipped items MUST be recorded and the skip limit MUST fail the step. | UT-BG-03 |
| BG-06 | Partitions MUST run in parallel within `batch.max_workers` and the admission rules of spec 0078. | IT-BG-02 |
| BG-07 | A remote settings refresh MUST pass the checks of spec 0106 and an invalid set MUST be refused with the old values kept. | UT-BG-04 |
| BG-08 | A discovered address MUST pass the outbound protection of spec 0107. | UT-BG-05 |
| BG-09 | A gateway MUST remove hop-by-hop headers, refuse loops and apply size and rate limits. | UT-BG-06 |
| BG-10 | A vault secret MUST stay in memory only and be dropped when not needed. | UT-BG-07, FZ-BG-01 |
| BG-11 | A model answer MUST be checked against its schema, and a tool call MUST be checked against the tool's schema and the caller's token. | UT-BG-08 |
| BG-12 | A prompt guard MUST cap prompt size, refuse a request containing a secret (SUT-1109) and mark or strip instruction-like retrieved text as defence in depth. | UT-BG-09, PL-BG-01 |
| BG-13 | AI output that would change a site MUST become a proposal. | BD-BG-01 |
| BG-14 | An outside AI provider MUST be off until switched on and MUST NOT receive site text without that switch. | UT-BG-10 |
| BG-15 | An MCP tool MUST be reachable only through the chain and method rules of spec 0109, and only on the local doors in Sutradhar. | IT-BG-03, PL-BG-02 |
| BG-16 | The MCP macros MUST derive schemas from types and fail the build on a mismatch. | UT-BG-11 |
| BG-17 | A mail header MUST reject line breaks, an address MUST be validated, and Sutradhar MUST mail only configured addresses. | UT-BG-12, FZ-BG-02 |
| BG-18 | A template MUST NOT run code, read files or reach the network. | UT-BG-13, PL-BG-03 |
| BG-19 | Every use of a raw (unescaped) value in a template MUST be flagged by the gate. | UT-BG-14 |
| BG-20 | PDF output MUST be blocked until an engine is chosen, and the block MUST be listed. | AU-BG-01 |
| BG-21 | Each feature MUST have parity rows, behaviour descriptions and independent contract tests. | CT-BG-01 |
| BG-22 | The config server, discovery, gateway and vault starters MUST be absent from the Sutradhar build (a Cargo feature, not only a switch), and `*.enabled` MUST default to off where the starter exists. | UT-BG-15 |
| BG-23 | A chunk MUST end at its item count or byte cap, a job's memory MUST be at most the workers times the byte cap, and an oversize item MUST fail with SUT-1115. | UT-BG-16, MS-BG-01 |
| BG-24 | A writer outside PostgreSQL MUST declare itself idempotent or be refused at build time. | UT-BG-17, FI-BG-03 |
| BG-25 | A failed checkpoint write MUST stop the job (SUT-1103), and a duplicate job MUST be refused (SUT-1100). | UT-BG-18 |
| BG-26 | A partition plan MUST be stored at first start and reused on restart, and a failing partition MUST end the step after the others checkpoint. | IT-BG-04 |
| BG-27 | A config snapshot MUST be validated as a whole and swapped atomically, an invalid one MUST be refused (SUT-1101), and a bad active one MUST roll back only under spec 0106 authority and generation checks, or hold affected work. | UT-BG-19, FI-BG-04 |
| BG-28 | The config server MUST refuse an unauthenticated request. | UT-BG-20 |
| BG-29 | Discovery MUST define healthy and unhealthy by the stated probe counts, and no healthy instance MUST fail with SUT-1104. | UT-BG-21 |
| BG-30 | A gateway MUST refuse ambiguous framing and loops (SUT-1105) and resolve and check a target once. | UT-BG-22, FZ-BG-03 |
| BG-31 | An unreachable vault MUST fail with SUT-1106 after the cache expires, and a rotation MUST honour the overlap. | FI-BG-05 |
| BG-32 | An AI call MUST respect the output token limit, the daily cap for outside providers and the admission timeout of specs 0078 and 0080, and its log MUST follow the retention setting. | UT-BG-23 |
| BG-33 | The prompt guard MUST meet the stated detection and false-positive thresholds on its fixed corpora. | MS-BG-02 |
| BG-34 | A macro-built MCP server MUST fail the build on a name clash, unsupported type, missing description or missing token scope, and MUST never skip the token. | UT-BG-24, PL-BG-02 |
| BG-35 | Mail MUST refuse a TLS downgrade, enforce the rate, daily and attachment limits, apply masking and raise an alert on failure without looping. | UT-BG-25 |
| BG-36 | Mail MUST be off until enabled and MUST follow the recipient list and send-step approval of spec 0099. | IT-BG-05 |
| BG-37 | A template MUST escape per context, refuse a context with no safe rule, strip line breaks in text templates, refuse include cycles and secret data, and stop at its limits with SUT-1111. | UT-BG-26, PL-BG-03 |
| BG-38 | A PDF request MUST fail with SUT-1113 while no engine is chosen. | UT-BG-27 |
| BG-39 | A tool call MUST run under the invoking caller's token and never gain rights. | UT-BG-28 |
| BG-40 | A build test MUST prove that no IMAP client is linked into the Sutradhar build. | UT-BG-29 |

## How it is tested

Designed, not run.

- **Unit (UT-BG):** chunk and skip arithmetic with one value just inside and one
  just outside each limit; config grant and replay cases under spec 0106;
  absence of a product config fetch or listener; vault binding, expiry
  and revocation checks; header handling; template
  escaping with a table of hostile inputs; mail header checks; schema checks.
- **Planted faults (PL-BG):** retrieved text that says "ignore your rules"; an
  MCP tool called without a token; a template that tries to read a file; each
  must be refused with its named message while an honest case passes.
- **Integration (IT-BG):** a job with 3 million generated rows on a real
  PostgreSQL 18 started by the test harness, with no Docker; partitions; a local
  MCP call end to end.
- **Fault injection (FI-BG):** the app killed mid-chunk and mid-checkpoint; a
  vault that goes away; a provider that times out.
- **Fuzz (FZ-BG):** hostile mail headers and addresses, secret-looking strings
  in logs, malformed templates.
- **Behaviour (BD-BG):** the report job scenario above, step by step.
- **Contract (CT-BG):** one test per parity item, written from the behaviour
  description by someone other than the implementer.
- **Audit (AU-BG):** each blocked item (PDF output, any vault or registry with
  no usable client) is listed with its reason.
- **Measurement (MS-BG-01, MS-BG-02):** peak memory of a 10-million-row job at
  the default chunk size and byte cap on the 8 GiB reference; mail and template
  throughput; the prompt guard's detection and false-positive rates on its
  fixed corpora.

## Changes from the source

The pillar list asked for clean-room engines for batch, gateway, discovery, AI
and templates. Behemoth Boot wraps mature crates and keeps only the safety,
checkpoint, guard and macro logic as our own code (AGENTS.md section 6 rule
14). The streamable HTTP transport of MCP is built but cannot be used by
Sutradhar, which allows only the local doors. PDF output waits for an engine
(QR-039). The config server, discovery, gateway and vault bridges are
framework parity only.
