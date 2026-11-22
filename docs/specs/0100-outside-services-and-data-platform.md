# 0100: Outside services and the data platform

This specification owns two things. First, the few outside services Sutradhar
may send alerts, tickets and reports to: Slack, Microsoft Teams, email, Jira,
Linear and Asana. Spec 0094 owns compatible local report files for
manual upload to Google Slides, Sheets and warehouses (DR-087).
Second, the generic data-platform tools on the embedded analytics store: a
read-only SQL console, scheduled exports, the data dictionary, custom
metrics, read-only report snapshots, bulk import with quality checks, and
retention per dataset. It does not own the datasets themselves (spec 0034
for visits, 0040 for search data, 0043 for charts, 0044 for crawl facts) or
the screens that run jobs (spec 0099).

DR-076 is the 20 November 2026 baseline for outside destinations.
Use the exact admitted routes below and the named later decisions.
DR-077 limits site edits to XenForo and WordPress.

This spec owns report delivery, other listed services and data
exchange. Spec 0132 owns the separate DataForSEO keyword exception
under DR-117. Spec 0133 owns the separate DR-118 evidence views
on that connector. These decisions grant no additional report
delivery connector here. [Spec 0136](0136-matomo-timeline-connector.md)
owns the separate read-only Matomo timeline exception under DR-120.

Outside services are optional, use the user's own accounts and are off by
default (ADR-0021). There is no remote bridge and no live progress link, so
nothing here accepts a call from outside; Sutradhar only sends. Progress and
reports are shared as exported snapshot files.

Example: Nia connects Slack. She pastes a webhook address for one channel, and
the screen says "Sutradhar will post short headlines to #seo-alerts. It will
not send page text." She presses Send test message; the exact message is shown
first and then sent. Later an alert fires: "2 new broken links on your forum,
high priority". She has also connected Jira for the Fix list: she presses
"Create ticket" on a finding, sees the exact ticket text, and approves it; the
ticket number is stored beside the finding. On the data side she opens the SQL
console, types `select section, sum(clicks) from search_daily group by
section`, and gets a table. When she tries `drop table`, the console refuses
with "This console can only read".

Values marked "proposed" are design choices to be checked by measurement.

## What it does

### Words used here

- **Service**: an outside program or website the user has an account with.
- **Connector**: the part of Sutradhar that sends to one service.
- **Destination**: one place in a service: a channel, a project or a mailbox.
- **Content level**: how much a message says (headline, standard or full).
- **Never-changeable class**: the settings that no token, script, workflow or
  agent can change (ADR-0021, spec 0020).
- **Alekh**: the table and chart views of spec 0043 (formerly QuarryView).
- **Dataset**: a named table of data in the analytics store.
- **Analytics store**: the embedded DuckDB database (ADR-0021).
- **Snapshot**: a self-contained file of a report as it was at one moment.

### What this spec designs

| Feature | What the user sees |
|---|---|
| 478 Slack integration | Alerts and reports to a Slack channel |
| 480 Microsoft Teams integration | Alerts and reports to a Teams channel |
| 482 Email integration | Alerts and reports to the user's own mailbox through their own account |
| 498 Asana integration | Tickets for findings as Asana tasks |
| 499 Linear integration | Tickets for findings as Linear issues |
| 500 Jira integration | Tickets for findings as Jira issues |
| 594 SQL console | A read-only query box on the analytics store |
| 595 Scheduled data exports | Datasets saved to files on a schedule |
| 596 Data dictionary | A plain-English description of every dataset and column |
| 597 Custom metrics | Named formulas over existing columns |
| 598 Report read access | A read-only snapshot file of a report for another person |
| 599 Historical data bulk import | Older data brought in from files |
| 600 Import data quality checks | Problems found before anything is saved |
| 601 Retention controls per dataset | How long each dataset keeps its rows |

### Outside services: shared rules

1. Direction is outbound only. Sutradhar starts every connection. A connector
   sends a message or creates a ticket and may poll the service for a ticket's
   status (a request Sutradhar itself starts every `service.status_poll_min`
   minutes, so no listener opens). A service never calls Sutradhar: there is
   no inbound webhook, no bot that takes commands and no remote bridge
   (ADR-0021, DR-028). Email is send only: no mailbox is read. Whether a
   workflow may be started by an internet message is a separate open question
   (QR-030), and no rule here depends on it.
2. The user's own account. A service is connected with a token or address that
   the user creates in their own account, with the narrowest permission that
   works. The token or address is a secret kept in the secret store of spec
   0065, is shown once and never again, and is never in a file, log, error
   message or report. How each service issues tokens, and what each can do,
   changes over time; every statement about a service in this spec is
   unverified until the connector's build step checks the official guide
   (QR-051).
3. Each destination has its own switch, off by default, and its own content
   level. A "Send test message" button sends one harmless message and reports
   the result.
4. Content levels: - **Headline** (the default): counts, a severity and the
   name of the Sutradhar site, such as "2 new broken links, high priority". No
   page address, no page text, no name of a person. - **Standard**: adds the
   user's own page addresses and a short plain-English finding. Page addresses
   are not secret, but they leave the PC and can reveal what a site holds, so
   Standard is treated as content that leaves the PC. - **Full**: adds an
   evidence excerpt that may hold site text.
5. Who may change a level. Headline is an ordinary setting. Standard and Full
   are both in the never-changeable class: only the owner, in the app, can
   switch them on, and no token, script, workflow or agent can (DR-079,
   decided by the owner on 20 November 2026; spec 0020 lists them). Full also
   needs the ADR-0021 switch "send site text off this PC".
6. Every message and ticket is built from a template (spec 0111) with that
   spec's escaping rules, goes through the approved send steps of spec 0099
   (JW-19), and is shown in full as a preview before the first send of a
   workflow and in the dry run. Nothing is sent from a dry run.
7. Reliability uses 0110 bounded retries, circuit breakers and limits.
   Persist the effect intent before dispatch. The approved destination
   contract below controls marker reconciliation and unknown outcomes.
8. A token that expires or is refused gives a plain "reconnect" prompt and
   pauses that connector, as spec 0040 does for its connections.
9. Disconnecting removes the stored token, stops all sends and keeps the
   history of what was sent. The stored ticket links stay but are marked
   "service disconnected". An erasure request (spec 0032) removes ticket text
   that Sutradhar stored; text already in the service is the service's, and
   the screen says so.
10. Everything sent is recorded in the audit log (spec 0032) with who, when,
    which destination, the content level and a hash of the content, never the
    token or the webhook address. Error messages and the audit log mask
    addresses.
11. Sending text about Sutradhar's own findings is not a change to a site.
    Nothing here changes a site.

### Content safety

1. Before first use, record third-party notice acknowledgement for
   the exact destination, account, content level and grant generation.
   Record action approval separately with scope and expiry. Changing
   the destination, credentials or content scope needs current authority.
2. Names of people (member names, author names) are never in Headline and are
   removed from Standard text; they appear only in Full.
3. Message text is escaped for the service's own markup. A value can never
   create a mention of everyone (`@channel`, `@here`, `@everyone`), a link
   preview, a heading or a hidden link; link previews are switched off where
   the service lets us. Jira and Linear text is escaped for their markup.
   Email follows the header rules of spec 0111.
4. Ticket flood control: one open or unresolved effect per finding
   and a daily cap per destination (`service.tickets_per_day`).
   At the cap show a local summary. It creates no extra ticket.
5. The service's terms of use are checked when the connector is built; a
   connector that would break them is not shipped (QR-051).

### Service details

| Service | What it sends | Notes (unverified until checked, QR-051) |
|---|---|---|
| Slack | A message to one channel | One admitted app incoming webhook for the exact approved channel |
| Microsoft Teams | A message to one channel | One admitted Teams Workflows endpoint for the approved channel |
| Email | A message to configured recipients | The user's own mail account over TLS (spec 0111); recipients come from the alert centre of spec 0099 |
| Jira | Creates an issue in one project; reads its status | Needs a project and issue type; the cloud and the self-hosted products differ |
| Linear | Creates an issue in one team; reads its status | Needs a team; may use a key or sign-in |
| Asana | Creates a task in one project; reads its status | Needs a project; may use a personal token or sign-in |

Each service has its own rate limit, read from its answers (a "slow down" answer
makes the connector wait as told); `service.rate_per_min` is only an upper cap.

Ticket rules for Jira, Linear and Asana:
1. A ticket is made from one Fix list item (spec 0048) after the user approves
   its text. The item and the ticket are linked by the idempotency key and the
   ticket's address.
2. Sutradhar may read the ticket's status and show it beside the item. The Fix
   list item closes only when Sutradhar verifies the fix (spec 0048), never
   because the ticket was closed.
3. Sutradhar never edits or deletes a ticket after creating it, except to add
   one comment when the item is verified fixed, if the user switched that on
   (`service.<destination>.comment_on_fix`).
4. A ticket carries the content level of its destination; the default for
   tickets is Standard once the owner has switched Standard on for that
   destination, and Headline otherwise.

### Approved destinations and file-only evidence (478–500)

Each destination binds site, owner approval, provider and adapter
version, exact resource and account references, credential
generation, allowed operations, content level and site-text
permission. Store the credential in Tijori and resolve it only
inside the approved route. Default off and Headline. Scope, consent,
revocation, resource, quota, current content and approval are
checked at intent, dispatch, receipt and local display. A template,
job, file or MCP request cannot change the restricted settings
class or inherit another destination. Preview the exact allowed
effect in the app. A dry-run or preview never dispatches.

Slack v1 qualifies the official app's [incoming webhook][slack]
for one approved workspace, app and channel with setup evidence.
Use the exact approved provider HTTPS origin. Refuse redirects and
changed or archived channel mapping. HTTP 200 with `ok` is the
documented acknowledgement. Preserve explicit failures separately
from lost replies. No channel override, commands, search or edit. Teams v1
qualifies the owner-controlled [Teams Workflows webhook][teams]
for one approved channel. Microsoft retired its legacy Office 365
connector webhooks in May 2026. Refuse them. A Teams posting endpoint does
not grant a generic Power Automate connector or inbound workflow.
QR-051 qualifies an exact authenticated Workflows profile with
current token claims, tenant, team, channel, fixed posting action,
flow owner and configuration revision. Pause on orphaned or changed
flow, invalid token or stale evidence. No anonymous or legacy
fallback. A 2xx can prove acceptance without channel display.
State the scope of the receipt.

Email sends only approved alerts and reports under 0111. Resolve
one configured account and authorised exact recipients. Require
0111 TLS and downgrade refusal. One SMTP effect per approved
recipient, with frozen envelope and header addresses, body and
attachment hashes. A rejected recipient never reaches DATA.
Final DATA acceptance means accepted for delivery. A lost final
reply may be unknown. Replaying a group never resends accepted or
unknown recipient effects. No response-driven forwarding or new
recipient. No IMAP, outreach or marketing purpose. Dry-run
and test sample modes send nothing. The explicit approved setup
test sends only one synthetic harmless message to the bound route.
It cannot serve as a real report or grant further sends.

One ticket adapter per qualified Jira, Linear or Asana profile
per destination, without substituting one provider's authority
for another's. Bind one exact eligible 0048 finding, scope,
destination, visible projection and approved idempotency marker.
A site and finding have one open or unresolved ticket effect.
Its logical identity survives destination and credential changes.
Rotation or remote closure cannot release an unresolved effect.
Read status only by exact linked ticket and distinct grant. Do not
treat remote closure as proof of a fix. A verified-fix comment
needs its own enabled operation and stable receipt. No arbitrary
edits, deletes, issue search access, or contact export.

Persist the logical effect, exact projected digest and reservation
before any send. Distinguish proved pre-dispatch failure from an
unknown external result. Reconcile by exact receipt or a proved
complete marker query for the original account, effect and approved
project, team or resource. It grants no broad issue search. An absent
result from an eventually consistent search cannot prove failure.
A retry is allowed only if the native profile proves no effect
or exact safe idempotency. Unknown stays held. Rate and retry
limits, destination or credential changes, revocation and erasure
cannot create new approval. Keep an audit-safe receipt and never
show a webhook address or secret in logs, exports or errors.

Spec 0040 owns GA4 connector permissions and daily copies.
The 0029 GA4, Matomo, Plausible and Fathom file route requires an admitted
manifest, own-site scope and safe aggregates. File intake grants
no connection or private API token. DR-120's Matomo read exception
is only spec 0136's protected timeline route. Do not share its
credentials, data or capability across routes.

Excluded integration intents, including provider access attempted
via template, script, MCP, file, UI or 0099 trigger, are refused
before credentials, quota, job creation or network activity.
An existing 0100 local file export remains a separate action.
No disabled integration inherits the rights of a named exception.

[slack]: https://docs.slack.dev/messaging/sending-messages-using-incoming-webhooks/
[teams]: https://learn.microsoft.com/microsoftteams/platform/webhooks-and-connectors/how-to/add-incoming-webhook

#### Named destination and refusal fixtures

These planned spec 0076 cases use synthetic destinations,
grants, credentials and receipts. QR-051 and QR-143 hold exact
provider profile and receipt proof. The cases make no live
request, create no new scope and use no real credential.

- `DES-0100-W2-OUTBOUND-DESTINATION` uses
  `VAR-0100-W2-SLACK-CHANNEL`, `VAR-0100-W2-TEAMS-WORKFLOWS`
  and `VAR-0100-W2-ALERT-REPORT-SMTP`. These are existing
  outward delivery routes only. The owner-approved destination,
  current grant, content level, exact template and 0118
  protection checks must pass before any live use. Paused or
  revoked routes give no dispatch. Erasure closes managed
  content access. Mask secrets in logs, receipts and files.
- `DES-0100-W2-CONNECTOR-REFUSAL` binds a distinct negative
  fixture for each excluded intent under SD-21 and DR-076.
  It admits no provider variant. Restored references, credentials,
  templates and jobs cannot create the excluded effect.
- F-0483 belongs to 0040. F-0484 and F-0485 belong to 0029.
  Their authorised intake and the separate 0136 Matomo timeline
  exception give these destinations no data or credential access.
  Local imports, compatible files and observed native evidence
  keep their own approval, scope, erasure and restore checks.

| Feature | Requirement | Design and variant | Fixture and test | Named fixture assertion |
|---|---|---|---|---|
| F-0478 | SD-01, SD-05, SD-27, SD-37 | DES-0100-W2-OUTBOUND-DESTINATION, VAR-0100-W2-SLACK-CHANNEL | FX-0100-W2-SLACK, IT-SD-01, UT-SD-23, BD-SD-01, CT-SD-01 | Pin one approved synthetic workspace, app and channel. Refuse a redirect, changed or archived channel and wrong workspace. Distinguish an exact acceptance receipt from an unknown reply. Keep endpoint details masked and do not repeat an uncertain effect. |
| F-0479 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-DISCORD-REFUSAL, SD-SCOPE, AU-SD-02 | Try bot enrolment, an inbound command and a workflow send. Refuse before storing any credential, reserving quota, creating a job or dispatching. An independent 0096 manual pack grants no bot. |
| F-0480 | SD-01, SD-05, SD-07, SD-37 | DES-0100-W2-OUTBOUND-DESTINATION, VAR-0100-W2-TEAMS-WORKFLOWS | FX-0100-W2-TEAMS, IT-SD-01, UT-SD-06, BD-SD-01, CT-SD-01 | Pin an authenticated synthetic flow with exact tenant, team, channel, owner and version. Refuse legacy profiles. Pause an orphaned or changed flow and an invalid token. Acceptance alone proves no channel display. |
| F-0481 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-MATRIX-REFUSAL, SD-SCOPE, AU-SD-02 | Try a room, homeserver and bot command through the action chooser and a restored template. All resolve to refusal before account, credential, job or message activity. |
| F-0482 | SD-01, SD-05, SD-23, SD-37 | DES-0100-W2-OUTBOUND-DESTINATION, VAR-0100-W2-ALERT-REPORT-SMTP | FX-0100-W2-SMTP, IT-SD-01, UT-SD-19, BD-SD-01, CT-SD-01 | Simulate TLS downgrade, a rejected recipient, final acceptance and a lost final DATA reply. Keep one original effect per exact approved recipient, and do not resend the accepted or unknown ones. IMAP, new purposes and a dry-run send are refused. |
| F-0486 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-CLARITY-REFUSAL, SD-SCOPE, AU-SD-02 | Try a Clarity enrolment, session replay and a file named as direct sync. Refuse the route before import of a session recording, credential, job or transfer. |
| F-0487 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-TAG-MANAGER-REFUSAL, SD-SCOPE, AU-SD-02 | Try a Tag Manager account and audit action. Refuse before a provider lookup or credential. Independently eligible existing own-site script observations keep their exact scope. |
| F-0488 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-CLOUDFLARE-REFUSAL, SD-SCOPE, AU-SD-02 | Request Cloudflare analytics and purge through a restored workflow. Both refuse before effect or credential. Existing observed edge evidence supplies no provider call or cache mutation. |
| F-0489 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-MERCHANT-REFUSAL, SD-SCOPE, AU-SD-02 | Try a merchant feed upload and account refresh. Refuse remote reads and writes before dispatch. Separate admitted own-site product evidence provides no merchant account or destination. |
| F-0490 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-DRIVE-REFUSAL, SD-SCOPE, AU-SD-02 | Choose Drive as a report destination. Refuse storage before any job or credential. An eligible compatible local file still uses its protected local route with no upload effect. |
| F-0491 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-DROPBOX-REFUSAL, SD-SCOPE, AU-SD-02 | Try Dropbox account enrolment and a queued report sync. No account, job, quota or dispatch is created, including on restore. A local file gives no remote storage rights. |
| F-0492 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-ONEDRIVE-REFUSAL, SD-SCOPE, AU-SD-02 | Import a synthetic template naming OneDrive output. Hold the unsupported reference as a safe local error. No credential resolution, account read or background file transfer. |
| F-0493 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-GOOGLE-CALENDAR-REFUSAL, SD-SCOPE, AU-SD-02 | Try account read and remote appointment creation from a workflow. Both refuse before setup or dispatch. A permitted local due date retains its own clock and grant. |
| F-0494 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-OUTLOOK-CALENDAR-REFUSAL, SD-SCOPE, AU-SD-02 | Try an Outlook enrolment reference and a relayed event from another workflow. Neither starts calendar lookup, a credential effect or an event change. |
| F-0495 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-NOTION-REFUSAL, SD-SCOPE, AU-SD-02 | Select Notion as input, output and action. The three intents refuse before connector, credential, job or request. A permitted unrelated local input supplies no workspace scope. |

### The read-only SQL console

1. The console runs only on the analytics store, never on PostgreSQL, the
   system of record. DuckDB has no roles, so safety comes from how the
   connection is made, not from a permission: - The console runs in its own
   worker process, which opens the store with `access_mode` read only. -
   Before any user text runs, external access is turned off and the
   configuration is locked, so no file, web address, extension or setting can
   be reached or changed. - It sees only views of allowed datasets, built
   without the sensitive columns. - Memory, threads and the spill folder are
   capped by the settings below.
2. Parsing is a second layer. Only a single SELECT (or a WITH query ending in
   a SELECT, or EXPLAIN) is accepted; a change, a creation, an attach, a copy,
   a pragma, a setting, an install or load, an export, a file-reading function
   or a quoted file name used as a table is refused with "This console can
   only read" (SUT-1700). Each of these routes has its own planted-fault test.
3. Limits protect the PC. DuckDB has no statement timeout, so a watchdog calls
   the interrupt of the connection after `sql.timeout_s` and ends the worker
   if it does not stop. Memory is the store's memory limit (`sql.memory_mib`),
   threads are `sql.threads`, and the rows shown are capped by `sql.max_rows`
   with a limit applied to the result. A query over a limit is stopped and the
   limit is named (SUT-1701). Targets: the console starts a query within a
   second, and a stop takes effect within `sql.stop_grace_s` seconds (proposed
   2).
4. A sensitive column is physically absent from the console's views, so
   `select *`, aliases, joins and subqueries cannot reach it; a query that
   names one is told "that column is not available" (SUT-1702). A column is
   sensitive when the data dictionary marks it so; only the owning spec's
   dataset definition sets the flag and it is not editable in the app. A test
   runs `select *` on every dataset and checks that no sensitive column
   appears.
5. The user can save a query with a name (at most `sql.saved_max` saved
   queries of at most 20 KiB) and sees a history of their last
   `sql.history_max` queries. History and results are local, kept only on the
   PC, never cached beyond the screen, removed by the erase-all button, and
   left out of support bundles. A screen note warns not to type secrets into
   queries.
6. Results can be shown as a table, exported (below) or sent to a chart of spec
   0043 (Alekh). They are never sent to a service.
7. Python scripts (spec 0104) and workflows (spec 0099) may run the same read-
   only queries through the same worker and checks; the console is not a way
   around them.
8. The console and the dictionary are actions of spec 0020 and resources over
   local MCP. An agent token may use them only with the read scope, through
   the same checks, and sees only non-sensitive columns.

### The data dictionary

1. Every dataset and column has a record: name, type, unit, a plain-English
   description, where it comes from, whether it is sensitive, its retention
   and its owning spec. Built-in descriptions are in the translation files.
2. The dictionary is generated from dataset definitions, so it cannot drift
   from the tables (a check fails the build if a column has no record). It is
   shown as a searchable screen and exposed as a read-only resource over local
   MCP.
3. Users may add notes to a dataset or a custom metric; notes are marked
   "added by you".

### Custom metrics

1. A custom metric is a named, versioned formula over existing columns of one
   dataset, chosen from a builder or written as one SQL expression. It is
   parsed into an expression tree that allows only an allow-list of functions
   and the columns of its own dataset, with no subquery, no file or network
   function and no statement. It is substituted into queries as a parsed node,
   never as joined text.
2. The formula is checked when saved (division by zero, types) and checked
   again every time it is used, because a column can later become sensitive or
   be removed; a metric that now uses a sensitive column stops working with
   SUT-1702.
3. A metric appears in tables and charts (spec 0043, Alekh) beside built-in
   columns, marked "custom", with its formula in a tooltip. A calculated
   column written as a Python view script (spec 0104) is separate: custom
   metrics are formulas that work in the console, charts and exports; scripts
   are for anything more.
4. Saving a new version keeps the old one; a chart records the version it used.
5. A custom metric cannot change ranking. It is display and export only. A
   metric that a script computes for ranking is a script signal under spec 0104.

### Scheduled exports

1. A dataset, a saved query or a report can be exported on a schedule to a
   file in the exports folder of the home folder (spec 0030) as CSV, JSON or
   Parquet (CSV is the default). The schedule is a workflow trigger of spec
   0099 in plain words.
2. Exports go only to the local exports folder, which the user may place on an
   external drive through the storage wizard (spec 0038). Sutradhar does not
   push exports to a cloud service in version 1. Files are created readable by
   the owner only.
3. A spreadsheet runs a cell that starts with `=`, `+`, `-`, `@`, a tab or a
   carriage return as a formula, so in a CSV export every such value in a
   text-typed column gets a leading single quote. Number-typed columns are
   left alone, so real negative numbers are not altered. JSON and Parquet are
   not altered. The manifest records “neutralised”. A paired import
   removes the quote only when the verified manifest and typed mapping
   prove it was added by this export. Reapply the rule on later output.
4. Exports are listed in the file ledger, are limited in size
   (`export.max_mib`), and old exports are removed after `export.keep_days`.
   Sensitive columns are never exported; the export lists the columns it left
   out.
5. An export file carries a manifest with the dataset, time, row count, format
   and a hash. For a scheduled Parquet export, apply the final file cap
   to the complete encoded Parquet bytes, including its footer.
   Convert the admitted `export.max_mib` to bytes at 1,048,576 bytes
   per MiB. At equality the size check passes. A byte over gives
   SUT-1711 before
   publication. Failed or oversized staging yields no output file or
   completion receipt. Discard staged material through the existing
   0030 and 0118 tracked cleanup. Keep cleanup held while its proof
   is missing. The schedule slot keeps its own recorded failure.
   Apply existing sink limits separately. QR-150 owns the unbound
   relationship to the query export cap.
6. An agent with read scope may prepare an export under 0020.
   The actual file effect uses its exact approved scope and path
   from the storage wizard.

### Read-only report snapshots

1. "Report read access" is a snapshot: the user exports a report as one self-
   contained HTML file (a PDF only after QR-039) that another person can open
   in a browser with no account and no connection to Sutradhar. Spec 0094
   owns the report snapshot master and schedules. This spec owns the shared
   snapshot renderer and export safety checks. Other local formats derive
   from the same approved master projection under DR-087.
2. The file holds the numbers and charts as they were. All text is HTML-
   escaped, charts are inline vector pictures, the page carries a content rule
   that allows no outside loading (`default-src 'none'`), and it loads no
   outside font or image. It holds no secret, no outside call and no internal
   address (a file path, a local network address or a user name). The export
   is refused if it would (SUT-1703); a test plants a `<script>` tag in a
   finding and checks it appears as text only.
3. The snapshot can be limited to chosen sections and, for any dataset with
   sensitive columns, shows only aggregated values.
4. Snapshots carry time, hash and a file-ledger record.
   Managed copies follow current scope and erasure. A manual copy
   already taken outside the app cannot be recalled.

### Bulk import and its checks

1. The user may bring older data into an analytics dataset from CSV, JSON or
   Parquet files chosen with a file picker of the app, for example a search
   export from before Sutradhar was installed. The path comes only from the
   user's pick, never from a workflow or a typed path. A mapping screen
   matches file columns to dataset columns, with a preview of the first rows.
2. Hard errors refuse the import. Only admitted warnings can be
   accepted by the owner. Check types, ranges and required values,
   duplicate keys, date gaps and overlaps with existing data, row counts
   against the file, and suspicious values (a count over
   `import.outlier_factor` times the median, default 1,000). Results are a
   plain list with counts and examples (SUT-1704).
3. The import loads into a staging table and is committed in one database
   transaction. Before that, an undo snapshot of the affected rows' dataset is
   taken as a file copy, after a check that there is enough disk space for it;
   the undo is held for `import.undo_days`. A failed import leaves the dataset
   unchanged. A committed import with accepted problems is SUT-1705.
4. Files are read as data only. Nothing in a file is run, and a file larger
   than `import.max_mib` or with more than `import.max_rows` rows is refused.
   An import records its file name, hash, row count and the user who ran it.
5. Imported rows are labelled with their source, so a later question "where did
   this number come from?" has an answer.

### Retention per dataset

1. Each dataset has a retention period set in the data dictionary. The visits
   dataset keeps raw events for 13 months, defined as 395 days, and then only
   aggregates (ADR-0021); other datasets have their own defaults and minimums,
   set by their specs and listed in the dictionary. Only the owner changes a
   period, in the app.
2. A nightly job (default 03:00, `retention.run_at`) removes expired
   data. Preview the rows and bytes. Shortening requires live owner
   confirmation. No undo copy can reopen data outside current retention.
3. Deleted rows in the analytics store are not physically gone until the file is
   compacted. The job therefore ends with a checkpoint and, weekly, a rewrite of
   the affected tables, so removed data is really removed.
4. Rollup tables that hold only aggregates are kept after raw rows go, if their
   dataset allows it. An erasure request (spec 0032) also reaches rollups where
   they could identify a person.
5. Retention cannot be set below the minimum a dataset's spec states
   (SUT-1712), and it cannot disable erasure requests, which are separate and
   always work.
6. Every retention change and every run is audited.

### Protected analytics contracts (594–601)

The console accepts a bounded single typed read statement under
DR-078. Allow SELECT, a read-only WITH ending in SELECT, and
unexecuted EXPLAIN of those forms. Validate the full parsed tree
against the admitted statement, function and relation catalogue.
Refuse external access, extension operations, dynamic names,
arbitrary catalogue inspection and hidden execution. Open only
a separate protected site-scoped analytics projection under the
admitted DuckDB worker, read-only. The worker cannot open the
operational database, unfiltered source or raw visitor events.
Sales and referral paths under 0103 and 0120 stay absent. Allow
only declared relation, function and metadata shapes. EXPLAIN
cannot expose hidden plans or execute input. Evaluate joins,
results and exports under the common disclosure history.
Recheck snapshot, actor, grant and erasure at dispatch and release.
Abort with a safe reason on time, memory or output breach. Release
no partial rows. Scrub query literals and protected fields in
history, explain and errors. Clear temporary work on stop.

The dictionary is the generated definition of projected dataset,
column, type, unit, source, owner, sensitivity and retention.
Local notes stay distinct. A custom metric binds one dataset,
approved expression and function versions, result type, unit,
precision and null rules. Unknown, division by zero, overflow
or incompatible units cannot yield a numeric value. No unknown
column, effect, external function or cross-scope data. Recheck
each expression at run and export. Keep every used revision
linked to its result. Metrics are display and local export only.

A read grant can prepare an export. A separate current 0020
action approval activates its write or recurring schedule.
Bind schedule ID, version and 0019 slot, exact dataset or 0094
master, projection, grant, format, file mapping and mount.
Protect content and manifest as one ledger effect. An exact
receipt replays after acknowledgement loss. Recheck scope,
disclosure and destination before atomic publication and copy.
Fence replaced schedules and clean orphaned staging. No
caller-chosen path or upload. Apply existing CSV text
neutralisation. Keep typed signed numbers intact. Managed
files follow erasure. Manual copies cannot be recalled.

The app and local 0020 MCP may read a protected report snapshot's
own typed approved projection. Take exact snapshot ID and
bounded cursor. Require current actor, site, grant, authority and erasure.
A cursor binds immutable master hash, projection, section,
sort, filter and ordinal. Use the existing 0020 authenticated
cursor binding for the exact principal, session and grant.
Recheck the original binding immediately before each response.
If the session or grant changed after a page was prepared,
withhold its rows, counts and next cursor. A replacement session
or grant with identical permissions cannot adopt the old cursor.
If an otherwise authorised request presents that cursor, give the
existing safe restart notice. A fresh request uses its current grant
and rechecks the exact master, source and erasure proof. Source
withdrawal still closes the affected projection.
Return at most 100 safe rows within declared output limits.
Apply common suppression. No comments, extra file metadata,
raw paths, unrestricted SQL or remote link. A later source
update does not rewrite an approved frozen report. Revocation
or erasure invalidates it. Imported HTML confers no access. A manual
HTML copy uses the separate escaped file export.

A historical import freezes owner-picked file, digest, exact
dataset and mapping revision, covered interval, zone, stable
row keys, expected dataset generation and rights. Derive the
hash from bytes copied from the owner-selected open handle.
Refuse path substitutions, links and over-limit input. Store
a safe display basename only. Hard structure, missing fields,
scope, sensitive data and conflicting keys at one revision
refuse the full import. Date gaps and outliers are warnings only
where admitted. A compatible update needs its own revision
and approval. Protect sample values, counts and examples.
Atomically publish protected staging through one conditional
data, manifest and receipt transition. An incomplete pointer
is unusable after restart. Exact replay rechecks rights before
returning its existing receipt. Bounded import undo needs
the matching post-import generation, current retention and rights.
Withdrawn, expired or edited evidence cannot be restored. Preserve later edits.

Retention pins authoritative scope, allowed min/max, policy
version, effective UTC and erasure generation. The owner approves
the exact shortening plan. Expire when event UTC plus the
admitted retention is at or before trusted current UTC. A
clock rollback cannot reopen an expired interval. Deny stale
cursors, derived reads and queued commits at policy activation,
before erasure work begins. Apply 0032 cleanup to active stores,
WAL, scratch, snapshots and object versions. Verify absence.
Admitted immutable backups stay inaccessible under expiry or
cryptographic deletion, and restore reapplies current fences.
No retention undo copy. A lengthening can apply only to future
eligibility; never revive expired or erased data. DR-029's
395-day raw default in 0034 keeps its separate 90-day journey
cap and two-year eligible aggregate allowance.

### Wave 2 local import and dataset retention bindings

Use synthetic selected files, mappings, dates, permissions and
managed-copy outcomes. QR-150 retains projection, atomicity and
erasure proof. No new input, storage profile or remote reader
follows from these fixtures.

| Feature | Owning requirement and design | Planned fixture and expected evidence |
|---|---|---|
| F-0599 | SD-17, SD-18 and SD-34 · DES-0100-W2-OWNER-SELECTED-IMPORT | FX-0100-W2-OWNER-SELECTED-IMPORT · UT-SD-15, UT-SD-16, UT-SD-27 and FI-SD-03. Stage bytes from the exact owner-picked opened handle under the approved mapping and generation. Cancel, substitute a path, change the mapping, fail publication and replay a proved receipt. Reject conflicting keys and expose no partial dataset or unproved undo. Publish data, manifest and receipt together only once under current rights. |
| F-0600 | SD-17 and SD-18 · DES-0100-W2-SAFE-IMPORT-QUALITY | FX-0100-W2-SAFE-IMPORT-QUALITY · UT-SD-15, UT-SD-16 and FI-SD-03. Compare synthetic type failure, missing key, same-version conflict, admitted outlier and bounded range warning. Hard structure, sensitive columns and scope failures refuse the full load. Show only approved warning classes and safe preview coverage. A private example or revealing count cannot be shown or used to waive a refusal. |
| F-0601 | SD-19, SD-20 and SD-35 · DES-0100-W2-DATASET-RETENTION-FENCE | FX-0100-W2-DATASET-RETENTION-FENCE · UT-SD-17, UT-SD-18 and IT-SD-03. Confirm an eligible owner shortening for one dataset and check DR-029's 395-day raw default and distinct 90-day linked journey cap where applicable. Race old cursors and queued commits with policy activation. Interrupted cleanup, trusted-clock rollback, lengthening or restore cannot revive expired data. Apply existing 0032 fences across managed copies and verify actual cleanup or continuing protected hold. No recoverable retention undo. |

### Not built

By the owner's decision of 20 November 2026 (DR-076), these are not built, and
the register marks them "not-built": 479 Discord bot, 481 Matrix, 486
Microsoft Clarity import, 487 Google Tag Manager audit, 488 Cloudflare
analytics and purge, 489 Google Merchant Center feeds, 490 Google Drive report
storage, 491 Dropbox, 492 OneDrive, 493 Google Calendar, 494 Outlook Calendar,
495 Notion, 496 Airtable, 497 Trello, 501 GitHub Issues, 502 Mailchimp, 503
Brevo, 504 Stripe metrics, 505 Patreon, 509 Figma assets, 510 Canva, 511
Unsplash, 512 Pexels, 513 Wikimedia Commons, 514 DeepL translation, 515 AI
provider keys, 517 Bitly, 518 Zapier and n8n,
519 WhatsApp Business, 520 Twilio SMS, 521 Microsoft Advertising keyword data,
522 RSS output feeds and 523 editorial calendar export. Out of scope as
platforms (DR-077): 506 Shopify, 507 Ghost and 508 Discourse. Other specs that
name an item in this list (for example a spec that mentions image sources or
translation) must treat it as not built. The product offers no
credentials, action, job type or destination for these integrations.
Specs 0040 and 0029 own their approved connector or file data.
DR-120 owns its separate 0136 Matomo read. DR-117 and DR-118
admit exact DataForSEO routes in 0132 and 0133. The latter owns
feature 516 within that scope. DR-134 and
[spec 0143](0143-audit-grid-findings-and-local-preview.md) own the
separate Local Search Preview with the same spend authority.
General providers remain excluded.
Spec 0115's feed reads and the approved Bedrock and owner MCP
routes retain their exact authority.
The generic provider key and visibility routes remain excluded.

Acceptance cases 501–523 use one refusal contract. Every excluded
intent fails before creating a capability, credentials, job,
reservation or network request from any supported entry point.
Older templates, restores and renamed or indirect aliases do not
create a permitted route. 0133's exact admitted feature 516
lookup remains separate, with 0132's shared budget and approval.
Local manual work retains its owning rules. Provider exclusions
do not suppress an unrelated licensed and admitted local file.
A file or template can report a safe unsupported-reference error.

**Scope fixture SD-SCOPE.** With old credentials and queued work in
a test restore, verify that an excluded action cannot be registered
or called and sends nothing. Check the compatible local report file,
local calendar, supplied feeds and rights-checked assets still work.
Admitted own-site sources and each 0132, 0133 and 0136 exception
use their own scope, settings and evidence. Restore cannot turn
an admitted exception into a generic connector.

#### Named provider and output refusal fixtures

These planned spec 0076 cases reuse SD-SCOPE through
`DES-0100-W2-CONNECTOR-REFUSAL`. QR-144 holds exact entry
point, alias and restore proof. Each excluded intent fails
before credential, quota, job and provider access. No positive
provider variant follows from an approved local control.

Use QR-controlled bounded local inputs with current rights. An
authorised compatible report file stays local for manual sharing.
A licensed asset file gives no remote source permission. An
approved audience, calendar, feed input or credential grants no
new destination, output profile or transfer.

F-0516 stays with 0133 and its exact DR-118 and 0132 authority.
DR-117 remains a distinct exact 0132 lookup. Existing approved
Bedrock and owner MCP routes keep their individual purpose and
approval. No exception grants generic provider access, paid
spending, a public bridge, RSS output or iCalendar export.

| Feature | Requirement | Design | Fixture and test | Named fixture assertion |
|---|---|---|---|---|
| F-0511 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-UNSPLASH-REFUSAL, SD-SCOPE, AU-SD-02 | Ask the composer for an Unsplash image. Refuse search and account access before creating work. A licensed local control passes only its own bounded reader and current asset rights. |
| F-0512 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-PEXELS-REFUSAL, SD-SCOPE, AU-SD-02 | Try a Pexels search, stale source URL and a supplied owner-picked file. The provider intents refuse before a read. The URL proves neither provenance nor rights for the separate file. |
| F-0513 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-COMMONS-REFUSAL, SD-SCOPE, AU-SD-02 | Try a Wikimedia Commons lookup and an independent local file. Reject the provider lookup. Hold local intake if declared rights or source evidence fail. No provider name establishes permission. |
| F-0514 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-DEEPL-REFUSAL, SD-SCOPE, AU-SD-02 | Withdraw the local translation helper and request a DeepL fallback or credential through preview and replay. Refuse both. An independently approved local language route retains its own controls. |
| F-0515 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-GENERIC-AI-KEY-REFUSAL, SD-SCOPE, AU-SD-02 | Submit a synthetic generic AI credential intent and attempt to reuse an existing local MCP or Bedrock grant. Refuse before import into the vault or outside dispatch. Exact approved routes remain separately scoped. |
| F-0517 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-BITLY-REFUSAL, SD-SCOPE, AU-SD-02 | Ask to shorten an approved campaign link and replay from a saved draft. Refuse shortening, lookup and credential resolution. The eligible own-site link remains available without provider wrapping. |
| F-0518 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-AUTOMATION-BRIDGE-REFUSAL, SD-SCOPE, AU-SD-02 | Restore synthetic Zapier and n8n references and try an inbound bridge from a template. Refuse listener, credential, schedule and effect. No remote trigger inherits the local door. |
| F-0519 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-WHATSAPP-REFUSAL, SD-SCOPE, AU-SD-02 | Select WhatsApp Business from an approved local plan. Refuse at destination validation and release no contact or message content. The prior approval grants no new recipient. |
| F-0520 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-TWILIO-REFUSAL, SD-SCOPE, AU-SD-02 | Replay an approved local alert as a Twilio SMS intent. Refuse before credential, quota reservation, job or dispatch. A local alert supplies no SMS authority. |
| F-0521 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-MICROSOFT-ADS-KEYWORD-REFUSAL, SD-SCOPE, AU-SD-02 | Present a Microsoft Advertising keyword action and an unrelated DataForSEO approval. Refuse before a paid reservation, credential or request. Existing independently admitted research evidence is unchanged. |
| F-0522 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-RSS-OUTPUT-REFUSAL, SD-SCOPE, AU-SD-02 | Request a report RSS output using an approved feed-read reference. Refuse a feed writer, endpoint and syndication job. The source-bound reader keeps its separate rights and bounded input. |
| F-0523 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-CALENDAR-OUTPUT-REFUSAL, SD-SCOPE, AU-SD-02 | Try iCalendar export, a calendar feed and a disguised sync alias. Refuse before file, job or provider effects. An authorised local due view and independent compatible report file retain their own approvals. |

### Wave 2 protected analytics and report hand-off bindings

Use synthetic site projections, grants, destination records and
receipts. QR-150 retains isolation, disclosure and erasure proof.
A read or preview grants no write, live route or new storage
profile. Hold a missing source, unproved disclosure or stale grant.

| Feature | Owning requirement and design | Planned fixture and expected evidence |
|---|---|---|
| F-0594 | SD-10, SD-11, SD-12, SD-28 and SD-29 · DES-0100-W2-SCOPED-ANALYTICS-WORKER | FX-0100-W2-SCOPED-ANALYTICS-WORKER · UT-SD-08, UT-SD-09, UT-SD-10, PL-SD-02 and FI-SD-02. Use synthetic approved one-site views and refuse undeclared relations or expressions, hidden columns, operational store or raw sales/referral access, wrong site and revealing combined results. On a worker interruption release no partial rows. Preserve the read-only cap, disclosure fence and safe diagnostics. No file or network access. |
| F-0595 | SD-15, SD-32, SD-33 and SD-36 · DES-0100-W2-EXACT-SCHEDULED-LOCAL-FILE | FX-0100-W2-EXACT-SCHEDULED-LOCAL-FILE · UT-SD-13, PL-SD-03, UT-SD-26 and UT-SD-28. Bind the existing separately approved schedule, one slot and destination mapping, then replace a schedule or mount, revoke after staging and lose the committed receipt. Release one complete eligible owner-only file and manifest, or none. Use the original ledger outcome on replay. CSV formula-shaped text is neutralised and typed signed numbers stay intact. For Parquet check below, equal and one byte over after final framing. An overrun returns SUT-1711. Failed staging releases no file or completion receipt. Track cleanup of the affected stage under the existing ledger. No cloud hand-off or widened file scope. |
| F-0596 | SD-12 and SD-13 · DES-0100-W2-DECLARED-DICTIONARY | FX-0100-W2-DECLARED-DICTIONARY · UT-SD-10 and UT-SD-11. Read the generated eligible dataset definition in the app and the exact local MCP read scope. Add a clearly marked local note, then remove a column definition. Refuse the unbound field. Return versioned definition metadata only and no sensitive values. |
| F-0597 | SD-14 and SD-30 · DES-0100-W2-TYPED-CUSTOM-METRIC | FX-0100-W2-TYPED-CUSTOM-METRIC · UT-SD-12, UT-SD-24 and PL-SD-05. Save an admitted typed formula for one dataset. Compare missing, stale, incompatible or protected inputs, zero denominator, overflow and a revoked field. A calculation with unavailable evidence stays unknown or is refused with a safe reason. Retain its exact revision and units for eligible results. No source mutation or rank input. |
| F-0598 | SD-12 and SD-16 · DES-0100-W2-AUTHORISED-FROZEN-REPORT | FX-0100-W2-AUTHORISED-FROZEN-REPORT · UT-SD-14 and the existing app and local MCP read contract. Use the exact approved 0094 snapshot projection with a bounded current 0020 cursor. Try a stale grant, new underlying source version, private report, external URL and altered or imported file. Read page one under G1 and prepare page two. Before release revoke G1 and admit a separately authorised session and G2 for the same actor and site with equal permissions. Hold the original response and refuse the G1 cursor under G2 with the safe restart notice. A fresh G2 request may read only with current eligible master, source and erasure proof. An ordinary source update leaves the frozen snapshot intact. Refuse invalid authority and erasure-held output. No remote opener, automatic file hand-off or broad dataset access. |

### Data

- Connector settings and destinations: `services`, `service_destinations`
  (tokens are in the secret store, not here).
- `service_sends`: destination, time, kind, content level, content hash, result.
- `ticket_links`: finding, service, ticket address, idempotency key, last
  status.
- Dataset definitions, `dictionary`, `custom_metrics`, `metric_versions`,
  `saved_queries`, `query_history`, `exports`, `imports`, `retention_runs`:
  tables with site column and forced row-level security (spec 0108); the
  analytics store holds only the datasets.

### Settings

| Setting | Type and unit | Default | Allowed range | Takes effect |
|---|---|---|---|---|
| `service.<name>.enabled` | on or off, per service; `route_out`, `owner_only` | off | two values | at once |
| `service.<destination>.level` | headline, standard, full; owner only for standard and full | headline | three values; full needs the site-text switch | at once |
| `service.<destination>.standard_allowed` | on or off | off | two values, owner only | at once |
| `service.<destination>.comment_on_fix` | on or off; `route_out`, `owner_only` | off | two values | at once |
| `service.rate_per_min` | whole number per service (upper cap) | 20 | 1 to 600 | next send |
| `service.status_poll_min` | minutes | 30 | 5 to 1,440 | next poll |
| `service.tickets_per_day` | whole number per destination | 25 | 1 to 1,000 | next send |
| `service.timeout_s` | seconds | 15 | 1 to 120 | next send |
| `sql.timeout_s` | seconds | 30 | 1 to 600 | next query |
| `sql.memory_mib` | MiB | 512 | 64 to 4,096 | next query |
| `sql.threads` | whole number | 2 | 1 to 8 | next query |
| `sql.stop_grace_s` | seconds | 2 | 1 to 30 | next query |
| `sql.saved_max` | whole number | 200 | 0 to 10,000 | next save |
| `sql.history_max` | whole number | 100 | 0 to 10,000 | next query |
| `sql.max_rows` | whole number | 10,000 | 100 to 1,000,000 | next query |
| `sql.max_export_mib` | MiB | 100 | 1 to 2,048 | next query |
| `export.max_mib` | MiB | 500 | 1 to 10,240 | next export |
| `export.keep_days` | days | 90 | 1 to 3,650 | next night |
| `import.max_mib` | MiB | 500 | 1 to 10,240 | next import |
| `import.max_rows` | whole number | 20,000,000 | 1,000 to 1,000,000,000 | next import |
| `import.outlier_factor` | ratio | 1,000 | 2 to 1,000,000 | next import |
| `import.undo_days` | days | 14 | 1 to 365 | next night |
| `retention.run_at` | time of day | 03:00 | any time | next night |
| `retention.<dataset>_days` | days | per dataset (visits: 395) | at least the dataset's minimum | next night |

The CLI, MCP and agents cannot change `service.<name>.enabled`,
`comment_on_fix` or the Standard and Full levels.

Values proposed and to be measured are the limits and defaults above. There is
no setting that lets the SQL console write, lets a connector receive a call,
sends Standard or Full content without the owner-only switches, or disables
erasure.

### Failure codes

SUT-1700 console refused a non-read statement; SUT-1701 query over a limit;
SUT-1702 sensitive column refused; SUT-1703 snapshot refused (secret);
SUT-1704 import checks failed; SUT-1705 import committed with accepted
problems (information); SUT-1706 service token expired or refused; SUT-1707
service unreachable, retrying; SUT-1708 service circuit open; SUT-1709
duplicate send prevented by the idempotency key (information); SUT-1710 send
refused, content level not allowed; SUT-1711 export file over its size limit;
SUT-1712 retention below the dataset's minimum. Codes SUT-1700 to SUT-1799 are
reserved for this spec.

### Security and privacy

- Tokens and addresses are secrets in the secret store; they are masked
  everywhere.
- Content leaving the PC follows the three levels, and full needs the owner's
  never-changeable switch.
- The console cannot read sensitive columns, cannot write and cannot reach
  files or the network.
- Imports and exports treat file content as data, never as an instruction.
- Every send, export, import and retention run is audited without secrets.

## Requirements

| ID | Requirement | Tests |
|---|---|---|
| SD-01 | A connector MUST send only outward, MUST NOT accept a call from a service and MUST be off until the user switches it on. | UT-SD-01, AU-SD-01 |
| SD-02 | A token or address MUST be stored in the secret store, shown once, and masked in every log, report and message. | UT-SD-02, FZ-SD-01 |
| SD-03 | A destination MUST have its own switch and content level, default headline; a level not allowed MUST be refused (SUT-1710). | UT-SD-03, BD-SD-01 |
| SD-04 | Standard and full MUST each need an owner-only switch in the never-changeable class, full MUST also need the site-text switch, and no token, script, workflow or agent MUST be able to turn any on. | UT-SD-04, PL-SD-01 |
| SD-05 | Every send MUST be previewed before the first send of a workflow, shown in the dry run, sent only through an approved send step, and audited. | IT-SD-01 |
| SD-06 | Record the exact ticket marker before dispatch. A retry MUST have proved no effect or exact provider idempotency. Hold unknown effects, alert and use only approved exact marker reconciliation. | IT-SD-05, SUT-1709 |
| SD-23 | A failing service MUST open the breaker, raise an in-app alert (SUT-1707, SUT-1708) and obey the service's own rate limit. | UT-SD-19, FI-SD-01 |
| SD-07 | An expired or refused token MUST pause the connector with a reconnect prompt. | UT-SD-06 |
| SD-08 | Disconnecting MUST remove the token, stop sends and keep history, and tickets MUST be marked disconnected. | IT-SD-02 |
| SD-09 | A Fix list item MUST close only when verified, never because its ticket closed, and a ticket MUST NOT be edited or deleted after creation except the one allowed comment. | UT-SD-07 |
| SD-10 | The SQL console MUST run only on the analytics store with a read-only connection and MUST refuse any statement that is not a single read (SUT-1700). | UT-SD-08, PL-SD-02 |
| SD-11 | The console MUST enforce its time, memory and row limits (SUT-1701) and MUST NOT call a function that reads or writes a file or the network. | UT-SD-09, FI-SD-02 |
| SD-12 | A sensitive column MUST NOT be selectable, exportable or usable in a metric (SUT-1702). | UT-SD-10 |
| SD-13 | Every dataset column MUST have a dictionary record, and the build MUST fail otherwise. | UT-SD-11 |
| SD-14 | A custom metric MUST be versioned, read-only, display and export only, and a chart MUST record the version used. | UT-SD-12 |
| SD-15 | A scheduled export MUST go only to the local exports folder, neutralise formula-starting values, leave out sensitive columns and respect its size and keep-days limits. | UT-SD-13, PL-SD-03 |
| SD-16 | A snapshot MUST be self-contained, hold no secret, script call or internal address, and be refused otherwise (SUT-1703). | UT-SD-14 |
| SD-17 | An import MUST run all its checks before saving, load into staging, commit in one step after an undo snapshot, and leave the dataset unchanged on failure. | UT-SD-15, FI-SD-03 |
| SD-18 | Import files MUST be read as data only and a file over its limit MUST be refused. | UT-SD-16, FZ-SD-02 |
| SD-19 | Preview exact retention purge and require live owner confirmation to shorten. Expired and erased rows MUST become ineligible before physical cleanup. No recoverable retention undo, and erasure remains enabled. | UT-SD-17, IT-SD-03 |
| SD-20 | The raw visit events MUST keep 13 months, defined as 395 days, by default and then only aggregates. | UT-SD-18 |
| SD-21 | Excluded intents MUST create no connector, credential, quota reservation, job or request from UI, script, MCP, alias, template, workflow or restore. Exact admitted exceptions and lawful local routes retain separate authority. | SD-SCOPE, AU-SD-02 |
| SD-22 | Each service MUST be tested against a recorded fixture of its own answers, and its connector MUST record the official guide and date it was built from. | CT-SD-01 |
| SD-24 | Polling for a ticket's status MUST be started by Sutradhar at `service.status_poll_min`, and no connector MUST open a listener. | UT-SD-20, AU-SD-01 |
| SD-25 | Message and ticket text MUST be escaped for the service's markup, MUST NOT create a mention of everyone or a link preview, and MUST NOT include people's names below the full level. | UT-SD-21, PL-SD-04 |
| SD-26 | One open or unresolved effect per site finding and a daily cap per destination MUST apply, with a local summary at the cap. | UT-SD-22 |
| SD-27 | A webhook address or token MUST NOT appear in an error message or the audit log. | UT-SD-23, FZ-SD-01 |
| SD-28 | The console MUST open the store read only in its own worker with external access off and the configuration locked, and MUST expose only views without sensitive columns. | UT-SD-08, PL-SD-02 |
| SD-29 | A query over `sql.timeout_s` MUST be interrupted by the watchdog, and the worker ended after the grace time. | FI-SD-02 |
| SD-30 | A custom metric MUST be parsed into an allow-listed expression tree, substituted as a node and checked again at every use. | UT-SD-24, PL-SD-05 |
| SD-31 | Query history, saved queries and results MUST stay local, be removed by erase-all and be left out of support bundles. | UT-SD-25 |
| SD-32 | A CSV export MUST prefix formula-starting values in text columns, leave number columns alone and record "neutralised". | UT-SD-13 |
| SD-33 | Export files MUST be owner-only, and an agent MUST NOT choose a folder outside the exports folder. | UT-SD-26 |
| SD-34 | An import MUST check disk space for its undo snapshot, take it before the commit, and keep it `import.undo_days`; a path MUST come only from the user's pick. | UT-SD-27, FI-SD-03 |
| SD-35 | Retention MUST compact deleted data so it is physically removed, and a period below the dataset's minimum MUST be refused (SUT-1712). | IT-SD-03, UT-SD-17 |
| SD-36 | An export over its size limit MUST be refused (SUT-1711), and a token refused by a service MUST pause its connector (SUT-1706). | UT-SD-28 |
| SD-37 | The third-party notice MUST bind the exact destination and current scope. A separate current approval MUST authorise every protected effect. | BD-SD-01 |

### Named ticket and connector refusal fixtures

These planned spec 0076 cases use synthetic destinations,
capability proofs, credential references and receipts. QR-051,
QR-143 and QR-144 hold exact profile and refusal admission.

- `DES-0100-W2-TICKET-EFFECT` uses `VAR-0100-W2-ASANA-PROJECT`,
  `VAR-0100-W2-LINEAR-TEAM` and `VAR-0100-W2-JIRA-PROJECT-TYPE`.
  Create on the exact destination needs current outward approval,
  one eligible 0048 finding, approved projection and a durable
  effect marker. One site and finding keeps one open or unresolved
  effect across destination or token changes. Unknown results
  stay held until exact owning-route reconciliation.
- A status read is a separately granted, locally initiated read
  of the exact linked ticket under the existing 0100 interval.
  It gives no create, comment, search or unrelated read permission.
  Reconciliation uses the original resource and exact marker
  under its own approval.
  A verified-fix comment needs separate approval. No remote
  callback, generic polling or provider reply closes a Fix
  list finding.
- `DES-0100-W2-CONNECTOR-REFUSAL` uses refusal cases under
  SD-21 and DR-076 or DR-077. No positive provider variant is
  implied by a separate permitted local file or site adapter.
  Such local inputs keep their own source, destination, rights,
  retention, masking and erasure checks.

| Feature | Requirement | Design and variant | Fixture and test | Named fixture assertion |
|---|---|---|---|---|
| F-0496 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-AIRTABLE-REFUSAL, SD-SCOPE, AU-SD-02 | Import an inert template naming Airtable read and edit intents. Neither intent creates a destination, credential, quota reservation, job or request. |
| F-0497 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-TRELLO-REFUSAL, SD-SCOPE, AU-SD-02 | Choose Trello for a Fix list finding and replay a previously saved plan. Refuse before credentials or dispatch. A local finding grants no Trello ticket. |
| F-0498 | SD-05, SD-06, SD-09, SD-24, SD-26, SD-37 | DES-0100-W2-TICKET-EFFECT, VAR-0100-W2-ASANA-PROJECT | FX-0100-W2-ASANA-TICKET, IT-SD-01, IT-SD-05, UT-SD-07, UT-SD-20, UT-SD-22, BD-SD-01, CT-SD-01 | Simulate one approved create in the exact project. Lose the reply, rotate the credential and revoke the grant. An incomplete marker result cannot clear uncertainty. Only a distinct current grant can read the exact linked task. |
| F-0499 | SD-05, SD-06, SD-09, SD-24, SD-26, SD-37 | DES-0100-W2-TICKET-EFFECT, VAR-0100-W2-LINEAR-TEAM | FX-0100-W2-LINEAR-TICKET, IT-SD-01, IT-SD-05, UT-SD-07, UT-SD-20, UT-SD-22, BD-SD-01, CT-SD-01 | Race synthetic create intents for the same finding, try another destination and exceed the cap. Preserve one unresolved logical effect. Reject a status reply for a foreign issue. Remote closure leaves the local finding open until verified. |
| F-0500 | SD-05, SD-06, SD-09, SD-24, SD-26, SD-37 | DES-0100-W2-TICKET-EFFECT, VAR-0100-W2-JIRA-PROJECT-TYPE | FX-0100-W2-JIRA-TICKET, IT-SD-01, IT-SD-05, UT-SD-07, UT-SD-20, UT-SD-22, BD-SD-01, CT-SD-01 | Pin exact deployment profile, project and issue type. Another deployment or arbitrary edit is refused. Create, exact linked status and one verified-fix comment each require their own current operation approval. |
| F-0501 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-GITHUB-ISSUES-REFUSAL, SD-SCOPE, AU-SD-02 | Try GitHub Issues as a product destination and as a finding-ticket action. Refuse both before resource or effect creation. Project work-list permissions do not transfer into the product. |
| F-0502 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-MAILCHIMP-REFUSAL, SD-SCOPE, AU-SD-02 | Try enrolment, subscriber sync and newsletter dispatch. None starts or releases recipients. An independently permitted local issue stays a manual draft. |
| F-0503 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-BREVO-REFUSAL, SD-SCOPE, AU-SD-02 | Restore a synthetic legacy credential reference and queued audience action. No usable token, recipient sync, job or dispatch returns. |
| F-0504 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-STRIPE-REFUSAL, SD-SCOPE, AU-SD-02 | Try a Stripe account read and an analytics alias that replays a payment dataset. Refuse provider access or payment effects. No shadow store opens. |
| F-0505 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-PATREON-REFUSAL, SD-SCOPE, AU-SD-02 | Try account setup and live membership intake. Both refuse before account scope, credential or provider read. Independently permitted local data retains its exact purpose only. |
| F-0506 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-SHOPIFY-REFUSAL, SD-SCOPE, AU-SD-02 | Try Shopify pairing and a proposed read-only connector. Both refuse. Admitted own-site WordPress shop evidence is a separate route and adds no Shopify scope. |
| F-0507 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-GHOST-REFUSAL, SD-SCOPE, AU-SD-02 | Try to pair Ghost and target a native editorial action. Refuse pairing, credentials and action. No unsupported platform becomes a local adapter. |
| F-0508 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-DISCOURSE-REFUSAL, SD-SCOPE, AU-SD-02 | Try Discourse pairing and live import under a saved forum grant. Refuse the native adapter and the remote read before resolving credentials or starting jobs. |
| F-0509 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-FIGMA-REFUSAL, SD-SCOPE, AU-SD-02 | Refuse live Figma asset fetch. An unrelated licensed local file is eligible only through its own bounded 0097 reader and exact operation rights. No design account or silent background import. |
| F-0510 | SD-21 | DES-0100-W2-CONNECTOR-REFUSAL | FX-0100-W2-CANVA-REFUSAL, SD-SCOPE, AU-SD-02 | Try Canva authentication and a remote template reference. Refuse login and remote rendering. An independently licensed local image can use only the existing protected asset route. |

### Portfolio summary read

Return the count of enabled, admitted destinations for this site
whose current connector state is failing or needs renewal. Count
each once, including a connector automatically paused for expired
credentials. Exclude destinations the owner disabled or disconnected
and services outside version 1. The read probes no service and carries
no address, ticket text, key or connector token.

Expose one bounded, site-scoped summary read under spec 0102. Its
source and failure cases run in IT-SS-06 with the owning module.

## How it is tested

Designed, not run.

- **Unit (UT-SD):** content levels with a table of messages and the data each
  may hold; the statement filter with a table of allowed and refused
  statements (including hidden writes, file and network functions, attach and
  extension loading); limit arithmetic with one value just inside and one just
  outside; formula neutralising; import checks on fixture files; retention
  previews.
- **UT-SD-28, scheduled Parquet size:** with a synthetic admitted
  encoder and eligible schedule, produce separate complete files
  one byte below and exactly at the configured final file cap. Each
  admitted slot publishes one complete owner-only output and its
  manifest. In another slot use a footer that takes the final file
  one byte over. Require SUT-1711 and zero published bytes, manifest
  hand-offs or success receipts. Separately fail before the footer
  completes. Verify the recorded failure and no completed artefact.
  Follow the tracked staging cleanup in both cases. Neither case
  releases a truncated file or selects another format or path.
- **Planted faults (PL-SD):** a workflow that tries to switch on the full
  level; a statement that calls a file function; an export with a value that
  starts with `=`; each must be refused with its named message while an honest
  case passes.
- **Integration (IT-SD):** a recorded fixture of each service: a message, a
  ticket and a status read; a disconnect; a retention run with a snapshot.
- **Fault injection (FI-SD):** a service that times out, a console query that
  runs too long, an import killed halfway.
- **Fuzz (FZ-SD):** hostile import files and hostile SQL text never crash the
  console or leak a secret.
- **Behaviour (BD-SD):** the Nia scenario above.
- **Audit (AU-SD):** no connector accepts inbound calls. Run
  SD-SCOPE for every excluded alias and entry point and assert zero
  credential, quota, job and dispatch effects. Exercise the exact
  approved local, file and named provider routes independently.
- **Contract (CT-SD):** one test per service from its official guide.

## Changes from the source

The old integration range 478–523 has 46 rows. DR-076 names six
delivery and ticket connectors. Three analytic rows use 0040 or
0029. The other 37 rows are excluded in their original scope,
including the three unsupported platforms under DR-077.
Later decisions admit the exact Matomo and DataForSEO routes
documented above. Data-platform features 594–601 are a separate
range. DR-078 limits the SQL console to analytics reads.
