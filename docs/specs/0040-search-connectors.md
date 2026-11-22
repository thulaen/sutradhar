# 0040: Search Console, Bing and GA4 connectors

The connectors let the owner link a site to Google Search Console, Bing
Webmaster Tools and Google Analytics 4 (GA4) with their own accounts.
Each night Sutradhar copies the site's search numbers, read-only, to
feed SEO reports ([0044](0044-seo-signals-crawler.md)), darshak's
analytics ([0034](0034-analytics.md)) and link ranking. It is part of
roadmap Phase 6; "Which spec designs each feature" in
[features.md](features.md) lists the features it designs in whole or in
part. In code it is the `connectors` module of sanket ("signal", the SEO
signals crate).

[Spec 0136](0136-matomo-timeline-connector.md) owns the separate
Matomo timeline read under DR-120. It shares no source metric
with these connectors.

## What it does

### In short

- The owner opens "Search connectors" in the desktop app and presses
  "Connect Google Search Console". A one-screen page says what will be
  read and that nothing can be changed, then the owner's browser opens
  Google's own sign-in page.
- After the owner agrees, Google returns a one-time code, which the
  service swaps for a refresh token: a long-lived key used to get fresh
  short-lived access keys each night. It is encrypted before it is
  saved. On screen these are called "access keys", since "token"
  elsewhere in Sutradhar means a unit of text.
- Each night it copies clicks, impressions, positions and search terms
  per page (Search Console), organic visits and engagement per landing
  page (GA4), and the same kind of numbers from Bing.
- "Disconnect" asks the provider to cancel the access, deletes the key
  and asks whether to keep or delete the copied numbers.
- If access is removed at the provider, the next pull fails, the screen
  says "Search Console needs reconnecting", the copied numbers stay, and
  the gap in dates shows on every chart.
- Bing Webmaster Tools has its own sign-in page, separate from
  Microsoft's main sign-in. The owner signs in there with a Microsoft
  account (Google or Facebook accounts also work there).

### Scope

- In scope: connecting, using and disconnecting the three services per
  site; the OAuth client setup guide; storing, refreshing and revoking
  keys; nightly pulls, backfill and the URL inspection budget; quotas
  and backoff; erasure; health, alerts, and tests with fakes.
- The 0040 scope admits no Yandex Webmaster or Naver Search Advisor
  connection, credential, job, write or provider-specific file intake.
  Features 668 and 669 need the owner decision recorded in QR-162 before
  technical admission for either route. Existing admitted local intake
  continues under spec 0100.
- Every scope asked for is read-only. Nothing is ever written to these
  services (no URL or sitemap submission). Notices of new pages are a
  plain list owned by the crawler ([0044](0044-seo-signals-crawler.md)).
- Owned elsewhere: SEO reports, merging GA4 with darshak, ranking and
  tuning, cloud storage of exports, the settings store and audit.

#### Named search provider scope refusals (668 and 669)

`REQ-0040-W2-PROVIDER-HOLD` and
`DES-0040-W2-SEARCH-PROVIDER-REFUSAL` bind refusal only.
QR-162 still requires the owner's explicit scope decision.
Any affirmative decision needs a separately qualified contract
before technical admission. No positive variant, credentials,
traffic data, job, importer or provider effect exists here.
The named AT cases retain their existing independent scope.

| Feature | Requirement | Design and variant | Fixture and existing test boundary | Named fixture assertion |
|---|---|---|---|---|
| F-0668 | REQ-0040-W2-PROVIDER-HOLD | DES-0040-W2-SEARCH-PROVIDER-REFUSAL | FX-0040-W2-YANDEX-REFUSAL, AT-CN09, AT-CN13, AT-CN32 | Try authorisation, pull, submission and provider-specific import with an unrelated synthetic grant. Refuse before credential, job, quota, file intake, traffic or effect. No capability follows from other OAuth. |
| F-0669 | REQ-0040-W2-PROVIDER-HOLD | DES-0040-W2-SEARCH-PROVIDER-REFUSAL | FX-0040-W2-NAVER-REFUSAL, AT-CN09, AT-CN13, AT-CN32 | Try setup, credential check, provider-specific import and market alteration in a restored plan. Refuse before credentials, network or job. No imported traffic, market or technical route changes. |

### Terms

- OAuth 2.0 authorisation code flow: the person signs in at the service,
  which sends back a one-time code; the program swaps it for keys over a
  direct call.
- PKCE: the program makes a random secret (the verifier), sends only its
  fingerprint (the challenge) with the sign-in, and must show the
  verifier when swapping the code. A stolen code is useless without it.
- OAuth client: the program's identity at Google or Bing (client id and
  secret), registered by the site's owner.
- Property: one site as Google or Bing knows it, such as
  `sc-domain:example.org` or a GA4 property number.
- Connection: one site's link to one service, through one sign-in, with
  one chosen property. Pull: one run copying one day's numbers for one
  connection. Backfill: copying older days on first connection.
- Vault: the part of tijori that seals and opens stored secrets.

### Where it runs

- Pulls run as durable jobs in the service; the management API answers
  the desktop app over the owner-only socket. No new process.
- The desktop app shows the screens and never sees a refresh token,
  access token, client secret or PKCE verifier.
- Network calls go only to the hosts in the allow-list below.

### Modules

- `connectors::oauth`: trait `OAuthProvider` (`authorize_url`,
  `exchange_code`, `refresh`, `revoke`), implemented once over the
  `oauth2` crate (PKCE S256, `state`, code swap, refresh, RFC 7009
  revocation) with reqwest. No OAuth or PKCE code of our own. Google and
  Bing are only configurations. Fake: `FakeProvider`.
- `connectors::vault`: trait `SecretVault` (`seal`, `open`, `rewrap`);
  `AeadVault`; fake `PlainVault`, which builds only under `#[cfg(test)]`.
- `connectors::source`: trait `ConnectorSource` with
  `pull(window, budget) -> Stream<Rows>`; `Gsc`, `Ga4`, `BingWmt`; fake
  `ScriptedSource`.
- `connectors::quota`: trait `QuotaBudget` over `governor` limiters and
  daily counters; fake `ManualQuota`.
- `connectors::state`: the connection state machine, pure.
- `connectors::store`: trait `ConnectorStore` (sqlx); fake
  `MemoryStore`. Every method takes a `SiteScope`.
- `Clock` and `Rng` traits are injected so expiry and randomness are
  testable.
- One OAuth crate, one HTTP client, one retry policy (backon), one
  limiter type (governor), one vault (AES-256-GCM from `aws-lc-rs`), one
  state machine for all three. Only the refresh token and client secret
  are stored; access tokens live in memory. Provider JSON is read with
  serde types, with no generated client.

### The OAuth client and its setup guide

Each site's owner registers their own OAuth clients, since a shared
client secret cannot ship in public source. The "Search connectors"
screen carries a guide with the exact redirect address to paste.

- Google: create a Google Cloud project; enable the Search Console API,
  the Google Analytics Data API and the Google Analytics Admin API; set
  up the consent screen (user type External, or Internal for a Google
  Workspace organisation) with publishing status "In production" (a
  project left in "Testing" gets refresh tokens that expire after 7
  days); add the two scopes; create an OAuth client with the redirect
  address. Google may show an "unverified app" warning (to verify).
- Bing: in Bing Webmaster Tools, Settings, API Access, accept the terms,
  choose OAuth Client, enter a name and the redirect address, copy the
  client id and secret.
- The owner pastes the client id and secret into the app. The secret is
  sealed at once, never shown again, and can only be replaced.

### The sign-in flow

1. A user with the permission "Manage search connectors" presses
   Connect. The app calls `POST /api/connectors/{service}/start`.
2. The service makes a flow waiting for its callback: random `state` and
   a PKCE verifier (43 to 128 characters) with its S256 challenge,
   stored with the site, the app user, the service and a 10-minute
   expiry. Only a hash of `state` is the lookup key; the verifier is
   sealed. One such flow per user and service; a new start replaces it.
3. The service returns the provider's sign-in address. The app shows
   the consent page (below), then opens the owner's browser.
4. Google: `https://accounts.google.com/o/oauth2/v2/auth` with
   `response_type=code`, `client_id`, `redirect_uri`, one `scope`,
   `state`, `code_challenge`, `code_challenge_method=S256`,
   `access_type=offline`, `prompt=consent`,
   `include_granted_scopes=false`.
   Bing: `https://www.bing.com/webmasters/oauth/authorize` with
   `response_type=code`, `client_id`, `redirect_uri`,
   `scope=webmaster.read` and `state`.
5. The provider sends the browser back to the redirect address with
   `code` and `state`, or `error`. The redirect is a loopback address on
   the owner's PC, opened by the service only for the flow, so nothing
   is opened to the network. Both go to the connector code
   (`POST /api/connectors/callback` within the service).
6. The service finds the flow by the hash of `state`, checks it is
   unexpired, unused, for the same user and site, marks it used in the
   same transaction, then swaps the code: Google at
   `https://oauth2.googleapis.com/token` with the verifier; Bing at
   `https://www.bing.com/webmasters/oauth/token` with the client
   secret. Any mismatch ends the flow with SUT-621, nothing stored.
7. The granted scope must equal the one asked for (Google lets a person
   untick a scope), and a refresh token must be present. The refresh
   token is sealed, the access token kept in memory, the readable
   properties listed and the owner picks one. The connection becomes
   `connected` and a backfill is queued.

Bing session guard: Bing documents neither PKCE nor `state`. If Bing
returns no `state`, the callback is accepted only for the same live app
session with exactly one Bing flow waiting, started under 5 minutes ago
(Bing's code lasts 5 minutes). Otherwise SUT-621.

### Scopes, the narrowest read-only ones

| Service | Scope asked for |
| --- | --- |
| Search Console | `https://www.googleapis.com/auth/webmasters.readonly` |
| GA4 | `https://www.googleapis.com/auth/analytics.readonly` |
| Bing Webmaster | `webmaster.read` |

Never asked for: `webmasters`, `analytics`, `analytics.edit`,
`webmaster.manage`, `openid`, `email` or `profile`. Search Console and
GA4 are separate connections with separate sign-ins, so each may use a
different Google account, and refusing one does not affect the other.

### Consent pages

Plain text, one screen, two buttons ("Continue to Google", "Cancel").
Search Console says:

> Connect Google Search Console
>
> Sutradhar will ask Google for read-only access to Search Console.
> It can see the sites your Google account can see, but it cannot
> change anything there.
>
> Each night it copies, for the site you choose: clicks, impressions,
> average position and search terms for each page, sitemaps, and the
> index status of up to 1,900 pages a day. The copy stays on your
> computer and is used for this site's reports and link suggestions.
>
> You can disconnect at any time. Sutradhar will then ask Google to
> cancel its access and delete its key, and you choose whether to keep
> the copied numbers.

GA4 says "organic visits, engagement and sign-ups for each landing page"
and has no inspection line. Bing says "Continue to Bing" and adds "Bing
Webmaster Tools has its own sign-in page. You can sign in there with
your Microsoft account." The provider's own consent screen follows.

### Storing secrets

- Stored: each connection's refresh token, each OAuth client secret and
  each waiting flow's PKCE verifier (for 10 minutes). Access tokens stay
  in memory, in a moka cache whose entries expire 60 seconds before the
  provider's `expires_in`.
- Envelope encryption: tijori holds a 32-byte master key, encrypted at
  rest in a file only the service account can read, never in an
  environment variable, never in a database backup, never given to dev,
  gate or test runs.
- Each site has a 32-byte data key, stored only sealed by the master key,
  with the master key's id.
- Each secret is sealed with the site's data key by AES-256-GCM from
  `aws-lc-rs`, with a fresh random 96-bit nonce. The associated data
  binds site id, connection id, secret kind and key id, so a sealed
  value copied to another row will not open.
- In Rust every secret is a `secrecy::SecretString` or
  `SecretBox<[u8; 32]>`: never printed, logged or serialised, wiped when
  dropped. Unsealing returns a secret type, never a plain `String`.
- Rotating the master key: a new key with a new id is added; a durable
  job rewraps every site data key; then the old key may go. Both are
  read during the changeover.
- Losing the master key moves every connection to `needs_reconnect`;
  copied numbers stay. The backup guide says to back the key up
  separately.

### Refresh

- Before each request the job asks the cache for an access token. If
  missing or within 60 seconds of expiry, it refreshes (Google `POST
  https://oauth2.googleapis.com/token` with `grant_type=refresh_token`;
  Bing likewise at its token address).
- One refresh at a time per connection (a `tokio::sync::Mutex` per
  connection id): ten parallel requests cause one refresh.
- A new refresh token in the answer replaces the old in one transaction
  with a version check, so a lost race never overwrites a newer token.
- A `401` on a data call triggers one refresh and one retry, never more.
- Network failures and `5xx` during refresh are retried with backoff and
  never mark the connection broken.
- `invalid_grant` (or Bing's equivalent) means the token no longer works:
  revoked, 6 months unused, 7 days in Google's Testing mode, replaced
  after Google's limit of 100 per account per client, or account
  disabled. The connection moves to `needs_reconnect`.

### When a token expires or is refused

- Pulls for that connection stop; nothing else stops.
- Copied numbers stay usable, marked with their end date. Every chart
  and report shows the gap as missing source coverage, never as zero.
- One grouped alert in the app and as a staff alert on the site: "Search
  Console needs reconnecting (SUT-630). Sutradhar can no longer read
  your Search Console data. Press Reconnect and sign in again.", with
  the manual link. If the token died about 7 days after connecting, it
  adds "Your Google project may still be in Testing status."
- Reconnect runs the same flow, keeps the connection id and property,
  queues a backfill of the missing days, and revokes the old refresh
  token at Google.
- Nightly pulls use each token at least daily, so the 6-month idle
  expiry never happens while connected.

### Disconnect and erasure

Disconnect, from the app, the `sutra` command or the API, all through
the action layer:

1. Google: `POST https://oauth2.googleapis.com/revoke` with the refresh
   token. Bing documents no revocation address; the screen says where in
   Bing Webmaster Tools to remove the app.
2. The sealed refresh token is deleted, even if revocation failed; the
   screen says so.
3. The owner chooses, with nothing preselected: keep the copied numbers
   (marked "source disconnected") or delete them.
4. An audit event records who, when, which service and the choice. No
   secret, address or property name enters it.

- Search terms may hold names, so they are treated as possibly personal
  and kept only in `gsc_query_day` and `bing_query_day`. An erasure
  request names a term or pattern; matching rows are deleted and every
  derived report regenerated.
- Removal offers export and the keep-or-delete choice for copied
  numbers, and always deletes every sealed secret and data key.
- Copied numbers are kept two years.

### What is pulled, and how often

Times are in the site's time zone (chrono-tz). The nightly window is the
quiet hours. Backfill runs at the lowest priority and yields to live
work.

Country dimensions describe past activity on an approved own-site
property. Feature 563, comparing search results by country, is after
version 1 (DR-081, spec 0102). Country fields cannot start a search
request, market report or local-language clustering job.

Search Console, per property:

- On connect and daily: `GET .../webmasters/v3/sites` to confirm the
  property and its permission level.
- Daily: `POST`
  `https://www.googleapis.com/webmasters/v3/sites/{siteUrl}/searchAnalytics/query`
  for the newest final date (`dataState` final), with `type` web, image,
  video, news and discover, grouped by date, page and query; by date,
  page, country and device; and by search appearance. Use the
  bounded paging rule below.
- Backfill on connect: use the published 16-month history as the
  upper bound. Discover the property and surface's available
  dates through the existing read route and fetch newest first.
  Leave unavailable dates and dimensions unknown.
- Daily: the sitemaps list.
- Daily URL inspection,
  `POST https://searchconsole.googleapis.com/v1/urlInspection/index:inspect`,
  within the budget (default 1,900 a day; Google allows 2,000 per
  property): new pages, then changed, then important (most clicks or
  sign-ups, ordered by ganit's indexing probability, lowest first), then
  a rotating sample so every indexable page is checked in turn.

Google's published history and row limits are recorded in
[SR-063 and SR-064](../registers/sources.md). The 16-month
history bounds a request. Actual earliest eligible date and
missing days come from the property and admitted report.
For a daily report, use `rowLimit=25000` and `startRow=0`.
If the first page is full, request `startRow=25000` under
the same property, provider day, search type and report shape.
Stop at a short page or 50,000 returned rows. Record reaching
the ceiling and keep the existing partial-evidence gate.
Page termination or a smaller return count alone cannot prove
complete page or query coverage. Preserve missing values as
unknown, including for spec 0150.

The page and query projection and the separate page, country and
device projection do not prove the joint page, query and device
population needed by 0043 F-0073. Keep their existing report uses.
For that comparison, QR-106 needs a separately qualified producer
profile for a stored final joint projection. It must bind the owned
property, current source scope generation, native dates, surface,
filters, page and query identity, device, position measure and
impressions and proved coverage of the selected population. Final
metadata and a current verified connection supply no completeness
proof for that joint report. The [API contract][joint-panel-api]
allows grouping by several dimensions and can omit rows. Require
all exact cells of the approved panel. Treat a needed hidden,
truncated or unproved cell as unknown. State the covered panel
without estimating the rest of the site. Keep screened or incomplete evidence
under the existing release rules. This report grants no inferred
cross-tab, request or connector scope.

[joint-panel-api]:
  https://developers.google.com/webmaster-tools/v1/searchanalytics/query

GA4, per property:

- On connect:
  `GET https://analyticsadmin.googleapis.com/v1beta/accountSummaries`;
  the owner picks one property.
- Daily: `POST`
  `https://analyticsdata.googleapis.com/v1beta/properties/{id}:runReport`
  for the previous day in the property's time zone: landing page,
  session default channel group, device and country, with sessions,
  engaged sessions, engagement duration, page views, new users and key
  events, `returnPropertyQuota` on. Backfill: all past data. Data API
  v1beta.

Bing, per verified site: on connect, the user's sites list; daily, rank
and traffic, query, page and crawl statistics. JSON interfaces only.

### Quotas and backoff

- Published limits: Search Analytics 1,200 queries a minute per site and
  per user, 40,000 a minute and 30 million a day per project; URL
  inspection 2,000 a day and 600 a minute per site; GA4 200,000 tokens a
  day and 40,000 an hour per property, 14,000 an hour per project per
  property, 10 concurrent requests. Bing publishes none.
- Sutradhar's own limits, in settings, as `governor` limiters per
  connection: Search Analytics 300 a minute; inspection 60 a minute plus
  the daily budget; GA4 2 concurrent, stopping when `propertyQuota`
  shows under 10% left for the hour or day; Bing 1 a second.
- Backoff: `429`, `5xx`, timeouts and connection errors retry with
  exponential backoff and full jitter, 2 seconds to 5 minutes, 6
  attempts, honouring `Retry-After`. Then the pull moves to the next
  window, never to the dead-letter list.
- A used-up daily quota (`RESOURCE_EXHAUSTED`, or `429` with a quota
  reason) stops that kind of call for that connection until the next
  day; the remainder goes first next night.
- A manual pull is limited to one an hour per connection.

### ganit jobs used

ganit receives only ids, dates, counts and quota figures, never search
terms or addresses. Each result is inert candidate evidence; spec 0077 alone
evaluates, promotes, activates and rolls it back. A late or failed candidate
leaves the verified champion unchanged.

- Job 214: estimates for the last 2 to 3 days of Search Console data,
  which arrive late, shown as a shaded "estimate" band, never mixed with
  copied numbers. With no result those days show "not yet available".
- Job 211: busy hours by weekday, to place the pull window in the
  quietest hours (default 02:00 to 06:00).
- Job 11: the nightly job schedule under the memory ceiling.
- Job 82: indexing probability per page, to order important pages in
  the inspection budget; without it, by clicks and sign-ups only.

### Isolation per site

- Every row carries `site_id`. Stores take a `SiteScope`, made only by
  the action layer after the Cedar permission check.
- Row-level security is the second wall. Each transaction binds its
  site through spec 0108's transaction-local setter, and every forced
  policy matches on it.
- Secrets have a third wall: `connector_client`, `connector_flow`,
  `connector_secret` and `site_data_key` are granted only to the role
  `sutradhar_connector`. The reports, review and API roles have no grant.
- Each site has its own data key, OAuth clients, limiters and budgets;
  one site's used-up quota never slows another.

### Data model (sqlx migrations, row-level security forced)

- `connector_client`: site_id, provider, client_id, sealed secret,
  key_id, updated_at.
- `connector_flow`: state_hash, site_id, app_user_id, service, sealed
  verifier, expires_at, used_at.
- `connector_connection`: id, site_id, service, verified property
  binding, source scope generation, state, connected_by, connected_at,
  last_success_at, version. The binding identifies the exact verified
  provider-native property within this site and connection. Keep it
  separate from the positive bigint scope generation. Advance the
  generation with each authorised rebind, permission change or
  disconnect, including reselection of the same property. Hold if
  the current generation cannot be proved or advanced without reuse.
- `connector_secret`: connection_id, sealed refresh token, key_id,
  nonce, version.
- `site_data_key`: site_id, sealed key, master_key_id.
- `connector_pull`: id, connection_id, verified property binding,
  source scope generation, kind, window, rows, quota used, outcome,
  error code, started_at, ended_at. Carry the binding and generation in
  the durable checkpoint.
- `connector_coverage`: key by site, connection, verified property
  binding, source scope generation, provider day and exact report
  scope. Keep the originating pull ID and status (complete, partial,
  missing).
- Copied numbers: `gsc_query_day`, `gsc_page_dim_day`,
  `gsc_inspection`, `gsc_sitemap`, `ga4_landing_day`, `bing_query_day`,
  `bing_page_day`, `bing_crawl_day`. Use that same scope key plus the
  native result dimensions that identify each row. Keep its originating
  pull ID. Store page addresses as thread or post IDs where they resolve.

### API, events and commands

Axum routes, documented with utoipa, each permission-checked by the
action layer with a capability id:

- `POST /api/connectors/{service}/start` (`gsc`, `ga4` or `bing`):
  returns the sign-in address.
- `POST /api/connectors/callback`: `state`, and `code` or `error`.
  Returns the connection and its readable properties.
- `PUT /api/connectors/{id}/property`: chooses the property.
- `GET /api/connectors`: each connection's state, property, last
  success, coverage and quota use. Never a secret.
- `POST /api/connectors/{id}/pull`: queues a pull now.
- `DELETE /api/connectors/{id}?data=keep|delete`: disconnect.
- `PUT /api/connectors/clients/{provider}`: sets client id and secret;
  the secret is write-only.

Spec 0019 transactional outbox events (ids and counts only):
`connector.connected`, `connector.property_chosen`,
`connector.pull_completed`, `connector.pull_failed`,
`connector.quota_exhausted`, `connector.needs_reconnect` and
`connector.disconnected`. Commit each event with the connector
transition or committed coverage change under RLS. Order by site
and connection. SEO reports and darshak consume `pull_completed`
idempotently and refresh local evidence. A stale generation or
disconnect fences the consumer. It cannot start a new provider pull.

Commands: `sutra sanket connectors list`, `status <id>`, `pull <id>`,
`disconnect <id> --data keep|delete`, `rotate-key`. A connection is
started only from the app, since the provider must send a browser back.
Backfills and long pulls report through the one progress system, with
ETAs from the quota left.

### Failure codes

Every failure has a plain message, a SUT code and a manual link.

| Code | When | What happens |
| --- | --- | --- |
| SUT-620 | No OAuth client set | Connect is disabled; guide shown |
| SUT-621 | State unknown, used or expired | Flow ends; nothing stored |
| SUT-622 | Person refused consent | "You chose not to connect" |
| SUT-623 | Scope not granted in full | Nothing stored; try again |
| SUT-624 | No refresh token returned | Revoke, ask to try again |
| SUT-625 | Code swap failed | Flow ends; retry offered |
| SUT-626 | Account sees no property | Explained, with Google's help |
| SUT-627 | Redirect address mismatch | Guide shows the exact address |
| SUT-630 | Refresh token refused | `needs_reconnect`, alert |
| SUT-631 | Property access lost | `needs_reconnect`, alert |
| SUT-632 | Daily quota used up | Paused until tomorrow |
| SUT-633 | Provider unavailable after retries | Rescheduled |
| SUT-634 | Answer not understood | Pull stops, AutoIssue filed |
| SUT-635 | Vault cannot open a secret | `needs_reconnect`, alert |
| SUT-636 | Revocation failed at provider | Key deleted anyway; told |
| SUT-637 | Bing returned no `state` | Session guard used |

- Pin the site, connection, verified property binding and source scope
  generation when admitting the pull. Recheck them and the 0019 job
  fence before each data request. Compare any property identifier the
  provider returns. Check the same current scope and job fence in the
  transaction that writes page rows, advances coverage or a checkpoint,
  or records success. A rebind, scope loss or disconnect fences an
  earlier attempt even if its response arrives successfully.
- Rows commit per page with the pull ID. A crashed pull resumes from
  a matching current checkpoint. A rerun replaces rows and updates
  coverage in one transaction for the exact site, connection, property,
  source scope generation, provider day and report scope. Writes for
  one generation leave any retained older generation untouched.
  Bind each committed coverage update to the pull that establishes it.
  Read a row or its coverage as current only if that pull's site,
  connection, verified property, source scope generation, day and
  report scope match the current requested scope. On any binding or
  scope mismatch, the refused page adds no rows, coverage or checkpoint
  advance. The attempt emits no successful completion.
  Record a superseded attempt under its original pull identity.
  The current connection status stays with its owning transition.
  Show “Pull held because the property or access changed.” Verified
  loss of current property access follows SUT-631. A current reply
  that names another property rejects the whole page under SUT-634.
  A legacy page, coverage row or checkpoint without proved original
  binding and generation cannot supply current evidence or resume a
  pull. Earlier eligible copies keep their original property and
  generation, including after reselection of the same property.
  Retained older copies follow current archive permission, retention
  and erasure.
- A new field in an answer is ignored; a missing required field is
  SUT-634 and the page's rows are rejected whole.

### Security and privacy

- Only a user with the permission connects, after the consent page.
  Nothing about members is sent to Google or Bing; this only reads.
- One read-only scope per connection; no profile or email scope; only
  the chosen property is copied.
- Secrets are sealed at rest, secret types in memory, write-only in the
  API, and never in logs, traces, metrics, URLs, errors, diagnostic
  bundles or audit events.
- Provider addresses are fixed in code, never taken from settings.
- Host allow-list: `accounts.google.com`, `oauth2.googleapis.com`,
  `www.googleapis.com`, `searchconsole.googleapis.com`,
  `analyticsdata.googleapis.com`, `analyticsadmin.googleapis.com`,
  `www.bing.com`, `ssl.bing.com`. Any other host is refused.
- TLS through rustls; certificate checks are never turned off.
- The callback is rate-limited and refuses bodies over 4 KiB. The
  `state` hash is compared in constant time.

### Observability

- Metrics (`sutradhar_` prefix; labels service, outcome and code only,
  never a property, address, term or user):
  `sutradhar_connector_pulls_total{service,outcome}`,
  `sutradhar_connector_rows_total{service}`,
  `sutradhar_connector_refresh_total{service,outcome}`,
  `sutradhar_connector_quota_used_ratio{service,bucket}`,
  `sutradhar_connector_state{service,state}` (gauge),
  `sutradhar_connector_last_success_timestamp_seconds{service}`,
  `sutradhar_connector_pull_duration_seconds{service}` (histogram).
- One span per pull and per provider call (connection id, page number,
  row count, status code). Structured logs with codes and ids.
- The health screen shows each connection's state, last success,
  coverage gaps and quota use. Alerts, grouped: `needs_reconnect`, two
  failed nights in a row, coverage gaps over 3 days.

### Edge cases

Domain and URL-prefix properties for one site; a site moved to https or
a new domain; the wrong property chosen; access cut from owner to
restricted; terms Google hides, so totals exceed the rows; days with no
data against missing pulls; GA4 time zone unlike the site's, and 23- or
25-hour days; thresholded or sampled GA4 answers, recorded as partial;
two users pressing Connect at once; the back button after consent; a
callback after the app session ended; the connecting user removed (the
connection belongs to the site; audit keeps the id); a client secret
rotated at Google; 100 reconnects reaching Google's per-client limit;
clock skew; a lost master key or a backup with an older one; a Bing site
removed from Bing Webmaster Tools.

### Link data and imports

- Live backlink and search-result provider feeds, including
  DataForSEO and Semrush, are not built (DR-076 and DR-085). Link study
  uses separately labelled, permitted owner-supplied evidence
  under spec 0046. Spec 0045 may use a public Common Crawl link map
  as a lead list, without a provider account or recurring feed.
  This does not change this spec's own-site search data routes.
- Version 1 imports Search Console, GA4, Matomo, Plausible and Fathom
  files. No raw server logs, since they hold visitor addresses.
- The Coach's related-questions check reads Search Console questions
  ([0042](0042-coach-additions.md)); this scope covers it.

### Wave 2 GA4 read and file evidence (483)

F-0483 · REQ-0040-W2-OWN-GA4-READOUT ·
DES-0040-W2-BOUND-GA4-SOURCES.

FX-0040-W2-BOUND-GA4-SOURCES uses the existing AT-CN19/22/29/30/35
and the separate 0029 visitor-file and 0100 staging checks. Compare
a stand-in daily copy for the exact admitted read-only GA4 property
and a separately approved GA4 totals file from spec 0029. Bind site,
property or approved file mapping, source, native day, coverage and
current scope. Keep the two inputs as separate report sources.
Partial or missing days stay labelled and cannot give a complete
daily result. Refuse mismatched site or property and deny further
reads on a revoked connection. Prior copies follow the owner's
existing keep-or-delete choice and current read scope. An approved
file or report destination grants no connector credential or reuse.
QR-143 still governs the exact input and projection profiles.

### Search history, query coverage and sitemap evidence (686–690)

These views use only the current authorised own-site data scope.
Bind site, actor, connection or source file, provider and verified
property, exact dates and provider clock, surface, aggregation,
filters, source and policy generations, watermark and coverage.
Display separate sources and units. Apply 0020, 0032, 0108 and
0118 before preview, export, replay or restore. These reports open
no provider session, raise no quota and never affect ranking.

- **686 and 687, Search Console and Bing history.** Save eligible
  final copied numbers from the existing connector in its one
  archive. The two views retain distinct properties, date bases,
  search surfaces and data sources. Key a partition by site,
  provider, property binding or verified Bing site, report
  profile version, search surface, dimensions, aggregation, filters,
  clock or calendar epoch and provider day. Keep source generation
  and canonical digest. A changed report grain cannot overwrite
  another series. A verified newer correction replaces the exact partition
  atomically after rights and erasure checks. Exact replay is inert.
  The [Search Console guide][gsc-history-view] and [Bing Webmaster
  announcement][bing-history] describe up to 16 months in the
  Performance dashboards. For Search Analytics use the separate
  published source bound above and the exact observed coverage.
  Use the existing two-year local retention
  on provider dates, subject to earlier rights or erasure.
  Show first and last observed days, source start, gaps, partial
  data, retention removals and disconnected evidence separately.
  Separate transport access from archive authorisation. Preserve
  keep-or-delete on disconnect. A kept old property needs an
  explicit current archive grant and original identity and cannot
  join the new property's charts. Revoke writes and exports before
  purging and apply erasure before restore. These views cause no
  new provider calls. The admitted connector backfill keeps its
  existing scope. No reconstruction or refill of erased data.
- **688, anonymised query share.** Use a qualified owner-supplied
  Search Console aggregate file under 0100 with explicit provider
  anonymised-query flag or an equivalent explicit metric. The
  first profile follows the [Search Console export
  fields][gsc-export-schema] for `searchdata_site_impression`
  and Web Search. It admits a locally supplied aggregate file
  with one unique reduced row per site, provider day, exact
  admitted country/device scopes and explicit
  `is_anonymized_query` boolean. Keep native additive clicks
  and impressions and no query text in the retained partition.
  Require property, source dataset and table, `ExportLog` epoch,
  date, digest, reduction proof and complete coverage in the
  manifest. The supplied reduction must sum all legitimate
  repeated native contributions once before unique-key import.
  Whole-partition replay is inert. A verified higher epoch
  replaces the earlier partition. A flagged input may only
  carry null or empty query. A missing flag, nonempty flagged
  term or ambiguous unflagged input with missing source term is
  refused at the boundary. Unflagged aggregate counts require
  evidence that all input query labels were valid before
  reduction and removal of text. Compute clicks share and impressions
  share separately from same-scope numerator and positive
  denominator. Keep explicit zero distinct from absent input.
  No ratio from partial or conflicting partitions. Do not use
  ordinary [report downloads][gsc-export-limits] as completeness
  proof. The ordinary Search Analytics API can omit rows. Its total minus
  visible
  named rows is an unexplained remainder, with no anonymisation
  share. Keep Discover-only anonymity, Google News, page and
  property groupings separate. No external warehouse connector.
- **689, landing page and query matrix.** The first profile
  uses final Search Analytics Web Search page aggregation.
  Match the full provider contract key, including site property,
  date, search type, provider page, exact named query and
  native dimensions and filter revision. Never merge device,
  country or other slices that overlap the same report grain.
  Keep Google canonical, declared canonical and observed own
  landing route separate. Merge aliases only with proved
  disjoint populations. Show native clicks, impressions,
  clickthrough rate and position. Sum only compatible disjoint
  counts. Recompute the rate from clicks divided by positive
  impressions. For admitted same-basis one-based native position
  use an impression-weighted aggregate. A supplied URL-bulk
  profile, if separately admitted, uses sum of zero-based
  positions divided by impressions plus one. Never average
  rates or fill missing cells. Show row caps, ties, missing
  dates and privacy restrictions as limits.
  A qualified display label must be an exact normalised term
  approved by the search-term screening and above the shared
  0034 floors. Use one cumulative release rule for labels,
  search, filters, autocomplete, tooltips, tables, CSV and AI.
  If disclosure fails, show only an eligible safe aggregate
  or a held result. No raw term, inferred anonymised label
  or identifier may leak through dependent outputs.
- **690, indexing evidence per sitemap.** Use 0044's admitted
  sitemap inventory for observed page entries and timestamp.
  Keep exact sitemap file and index lineage. Count a distinct
  eligible URL once within one sitemap snapshot. Keep membership
  in multiple files and prohibit summing overlapping subtotals.
  Join retained 0040 URL inspection by the exact selected
  property and inspected URL under a current comparison profile.
  No new inspection or crawl. Retain `result_observed_at` and
  provider `lastCrawlTime` independently. The dated mapping uses
  PASS as Valid, FAIL as Error, and NEUTRAL as Excluded. PARTIAL,
  unspecified, stale or unknown verdicts have no known outcome.
  Keep `indexingState` as separate noindex evidence. A PASS
  with Google canonical matching this exact URL may count as
  reported indexed at inspection. An alternate Google canonical
  takes its own category. Missing canonical proof gives unknown
  for exact indexing count. Never borrow another URL's record.
  A provider sitemap reference can be incomplete. Show
  observed membership, out-of-scope counts where safe,
  current inspected categories, uninspected and unknown separately.
  Whole-file counts require every in-scope member from the same
  sitemap version to have a current compatible known outcome. Sitemap
  declaration, submitted
  count, Sitemaps API's deprecated indexed field, warnings,
  noindex and robots outcomes are not substitutes for indexed
  URLs or evidence that a result currently appears in search.
  See the [Sitemaps API][gsc-sitemaps] and [URL Inspection
  result][gsc-inspection-result]. No sitemap submission or change.

QR-166 admits exact versioned provider parameters, file projection,
storage and permission tests. First-party provider evidence may
stay offline as the approved local copy. Purge withdrawn source
and dependent history under the existing two-year outer limit,
site/key separation, report invalidation and restore protections.
The dated provider references were checked on 20 November 2026.

[gsc-history-view]: https://developers.google.com/search/docs/monitor-debug/debugging-search-traffic-drops
[bing-history]: https://blogs.bing.com/webmaster/2025/6/Start-Using-Bing-Webmaster-Tools-to-Improve-Your-Site-Visibility/
[gsc-export-schema]: https://support.google.com/webmasters/answer/12917991
[gsc-export-limits]: https://support.google.com/webmasters/answer/12919797
[gsc-sitemaps]: https://developers.google.com/webmaster-tools/v1/sitemaps
[gsc-inspection-result]: https://developers.google.com/webmaster-tools/v1/urlInspection.index/UrlInspectionResult

### Wave 2 retained evidence bindings

Use the exact source and permission profiles under QR-166.
Each variant uses the existing protected read or supplied
file contract. Test fixtures cannot admit a new source.

| Feature, requirement and included design | Existing scope and exact planned fixture |
|---|---|
| F-0686 · REQ-0040-W2-ARCHIVE-PARTITION · DES-0040-W2-OBSERVED-HISTORY | FX-0040-W2-GSC-HISTORY and IT-0040-W2-GSC-HISTORY extend AT-CN30/31/37. Preserve distinct final provider days, report grain and property binding. Replay the exact partition, replace it by a verified higher source generation and refuse the wrong property. Keep the original gaps and retention label. Erasure and restored old rights cannot trigger border extension or replenish removed dates. |
| F-0688 · REQ-0040-W2-EXPLICIT-ANONYMITY · DES-0040-W2-SUPPLIED-ANONYMITY | FX-0040-W2-ANONYMOUS-SHARE and IT-0040-W2-ANONYMOUS-SHARE use the complete approved synthetic aggregate. Show separate clicks and impressions shares. A missing flag, populated flagged term, partial source or zero denominator withholds the affected result. Prove legitimate repeated contributions were reduced exactly once. An API remainder stays unexplained. |
| F-0689 · REQ-0040-W2-PAGE-QUERY · DES-0040-W2-BOUND-MATRIX | FX-0040-W2-PAGE-QUERY and IT-0040-W2-PAGE-QUERY extend AT-CN31/35. Use distinct observed landing, declared and provider canonical identities. Compare disjoint compatible counts with overlapping slices and row-cap ties. Hold the same screened label in autocomplete, table, CSV and AI under one rule. Missing cells stay unknown. A named query matrix cannot prove complete unfiltered page-day coverage for 0138. |
| F-0690 · REQ-0040-W2-SITEMAP-OBSERVED · DES-0040-W2-RETAINED-INSPECTION | FX-0040-W2-SITEMAP-INDEX and IT-0040-W2-SITEMAP-INDEX use retained exact URLs and property evidence. Deduplicate inside one sitemap snapshot. Hold whole-file totals on partial, stale, uninspected or foreign facts. Keep alternate canonical and noindex separate. Two overlapping files have separate coverage. No extra inspection, fetch or submission follows. |

### F-0011 and F-0012 Search Console advisory bindings

The owner design is 0040 for both features. The shared input under
`DES-0040-ADVISORY-SOURCE` is a current site grant and one compatible
final Search Console projection. Bind the owned property, native
calendar, search surface, projection grain, applicable dimensions,
page identity, exact time window, coverage and input generation.
Require the source profile's comparable, proved rows. Do not assemble
page totals by adding overlapping slices or infer missing rows.
Use only numbers already admitted through the owning connector.
A partial or missing interval cannot establish zero, position, an
absence of eligible pages or an anomaly. The existing dimensional
rows give no admitted unfiltered page-day profile. Hold either
feature's finding until its exact page-grain population and
coverage receipt are admitted. A connection status is distinct
from projection authority. A current archive grant may permit
comparable pre-gap data with proved exact coverage.

| Feature | Requirement and design binding | Advisory outcome and technical hold |
|---|---|---|
| F-0011 · Striking-distance finder | `REQ-0040-STRIKING`, `DES-0040-STRIKING-ADVICE` | Compare the same-scale source position against the inclusive 8–20 band before display rounding. The applicable impression floor, time window and exact page projection need an admitted feature profile. Until then withhold the advisory finding. A supported finding may explain a reviewable title or link proposal under the existing 0041 and 0044 boundaries. Spec 0044 job 324 is a separate existing input to the need score. This finding adds no ranking input or edit authority. |
| F-0012 · Click-rate anomalies | `REQ-0040-CTR-GAP`, `DES-0040-CTR-GAP-ADVICE` | Observed click rate is clicks divided by a positive impression count on the same basis under 0043. A negative gap needs an independently admitted 0066 ganit position-conditioned expected-rate profile with its compatible population, windows, support, below-expectation threshold and validation. Missing or sparse proof withholds the finding. A supported finding may offer title or snippet advice for review. It never changes metadata or claims a cause. |

Reuse AT-CN30 and AT-CN35 for connector site isolation, AT-CN33 for an
invalid source row and IT-0043-OWN for compatible periods, grain,
weights and coverage. These cases retain their owners and do not
qualify either advisory rule by themselves. Do not use job 324 or a
general analytics anomaly test as proof of feature 12.

| Feature-specific case | Synthetic fixture and required result |
|---|---|
| `UT-0040-STRIKING` · REQ-0040-STRIKING | Use positions 7, 8, 20 and 21 at an explicitly supplied synthetic demand floor. Compare floor equality with one impression below. With a complete matching synthetic profile, only the two boundary positions that meet the floor enter advice. Without an admitted production profile, or with a missing final date, show unavailable with the coverage reason and emit no finding. |
| `UT-0040-CTR-GAP` · REQ-0040-CTR-GAP | Use equal positions with different observed rates, and the frozen support and threshold of a synthetic expected-rate profile. Check above, at and below its declared decision boundary. Change the compatible population, remove or zero the impression denominator, or give sparse inputs. Each incompatible case holds. No qualified expectation means no alert even with a low raw rate. |

VAR-0040-STRIKING-PAGE applies to the admitted page-grain demand
and position profile. VAR-0040-CTR-COMPARABLE applies to matching
current observed and expected rate profiles. Both use the exact
cases and source gates above.

A held or accepted advisory result grants no new connector pull,
rank-weight change or native write. Neither synthetic profile supplies
production thresholds, profile identity or proof.

### F-0013 to F-0019 Search Console evidence bindings

This section covers F-0013, F-0014, F-0015 and F-0017 through
F-0019. F-0016 keeps its 0044 owner. Use only current authorised
0040 evidence under DES-0040-ADVISORY-SOURCE. Bind site, verified
property, exact source and surface, current source profile, native
calendar, page or section identity, scope and window, coverage and
generation. Hold mismatched or unsupported dimension and surface
combinations. An omitted row, stale or partial result, gap or
unsupported metric stays unknown. Final dates and completed pulls
do not prove full population coverage. Keep supported source panes,
estimates and observations distinct. No mixed-provider total or
inferred zero.

| Feature and ownership | Requirement and design |
|---|---|
| F-0013 · Ranking-drop forensics | REQ-0040-DROP-FORENSICS and DES-0040-DATED-FORENSICS bind a drop to comparable final same-surface windows and its exact page identity. Present separately supported dated content changes, verified lost links from the existing evidence owners, inspection observations and official 0137 dates. Read only their admitted projections. Study a source gap separately. A coincident event or missing cause supplies no causal verdict. No new crawl, inspection pull, provider or Lab route. |
| F-0014 · Google update timeline | REQ-0040-UPDATE-SOURCE and DES-0040-TIMELINE-EVIDENCE bind distinct authorised source panes and proved historical section or category scope. 0137 owns the sole official Google incident identity and revision, public-feed admission, freshness and phase gates. 0043 owns event-band presentation. Other 0045 notice sources supply no alternate Google incident. A scope or series gap holds the affected comparison. Preserve admitted bands and series; do not substitute a whole-site or other-source result. |
| F-0015 · Indexing watcher | REQ-0040-INSPECTION-WATCH and DES-0040-QUOTA-INSPECTION use only the existing ordered budget, property and authorised public URL in 0040. Validate the exact inspected URL, observed time and compatible status profile. Keep a matching not-indexed result, alternate canonical, an uninspected URL and an unknown or refused result distinct. Show supported readback and an explanation. F-0175 and F-0179 read this dated URL-level result. Retain result and provider crawl times separately as for F-0690. The connector supplies no indexed body or content-revision binding. Joined revision evidence belongs to 0044 and QR-110. Neither a scheduled inspection nor advice requests indexing, submits a sitemap or applies a repair. QR-103 retains the separate effect design. |
| F-0017 · Vocabulary gap | REQ-0040-SCREENED-WORDING and DES-0040-WORDING-EVIDENCE bind only a permitted screened search term under the existing 0040/0034 release rules to an independently admitted comparison with eligible public member wording for the same site. Recheck rights and source versions. Suppressed, personal, unproved or unavailable terms cannot appear in advice, exports or prompts or establish absence. Spec 0041 owns local proposal presentation and approval. Its separate term report does not admit Search Console terms as its vocabulary. Hold wording advice without a qualified feature-specific projection. |
| F-0018 · Image search harvesting | REQ-0040-IMAGE-EVIDENCE and DES-0040-IMAGE-THREAD-JOIN use only the compatible Search Console image surface. A page-level impression alone identifies no image. Require an admitted receipt binding the exact eligible image and current guest-visible thread or page revision and their rights. Preserve dated page evidence where allowed and hold image-level claims and placement suggestions on a stale, ambiguous or partial join. QR-103 retains exact placement, approval and footprint. This binding creates no link or media effect. |
| F-0019 · Google Discover readiness | REQ-0040-DISCOVER-EVIDENCE and DES-0040-DISCOVER-READINESS keep eligible own-site Discover metrics separate from versioned, actually observed readiness facts under the applicable page evidence owner. A missing metric, missing expected field or unsupported source profile stays unavailable. Neither traffic nor a readiness check certifies Discover eligibility. QR-103 retains the exact public preview-markup effect, approved footprint and native grant. This binding grants no markup edit or extra public work. |

The optional 0136 Matomo pane has its own grant and native dates.
It cannot replace Search Console coverage or qualify an 0138 study.
Spec 0138 requires its separate DR-124 approval and admitted,
complete page-day producer. The 0133 paid Coach route, 0134
synthetic gate workloads and 0135 verified-fetch ranking fact supply
no authority or proxy data here. No binding adds a connector,
provider write, crawl, native effect or rank input.

| Fixture and governing planned obligation | Exact boundary |
|---|---|
| FX-0040-W1-DROP · IT-0040-DROP-FORENSICS | Compare a final matching drop, missing pull, revised page, verified link loss and separately dated update. Hold the unsupported drop and any causal inference. Keep evidence and its dates distinct. |
| FX-0040-W1-UPDATE · IT-0137-FEED, IT-0137-TIME, IT-0043-OWN and QV33 | Accept one dated official notice and replay it. Let its 0137 confirmation lapse during a feed outage. The chart and table keep the permitted saved band, its dates, the last confirmation time and the existing hold. No new phase notice or comparison follows. Confirm corrected dates for the same incident and replay the older revision. One current band keeps the corrected dates. Give two sections different coverage, with required Search Console days still arriving in one. Keep that pane’s own freshness state and hold its comparison. The other section still needs complete current evidence to qualify. |
| FX-0040-W1-INSPECTION · IT-0040-INSPECTION-WATCH and shared AT-CN26/28 | Fill the last eligible slot, queue another URL, and separately vary property and inspected URL. Only the compatible observation can support an explanation. Full quota waits; no repair, submission or live fetch follows from replay or view. |
| FX-0040-W1-WORDING · IT-0040-SCREENED-WORDING and shared AT-CN31/35 | Use two screened fixture search terms, a withheld term, a missing eligible member sample and a superseded revision. Hold unsupported advice and keep the protected terms out of outputs. No title or post write. |
| FX-0040-W1-IMAGE · IT-0040-IMAGE-EVIDENCE | Compare eligible image observations, an exact synthetic image-to-thread receipt, page-only impression, stale and ambiguous join. Preserve the supported observation and hold unsupported placement. No link write or fetch. |
| FX-0040-W1-DISCOVER · IT-0040-DISCOVER-EVIDENCE | Supply eligible synthetic readiness observations and Discover traffic separately, a missing metric and an incompatible surface. Reject preview-markup write and unsupported coverage. Neither missing traffic nor ordinary Web figures establish eligibility. |

Existing shared tests retain their owners. These fixture identities
bind planned source and refusal cases under 0076; each affected
operational profile still requires its own exact evidence.

### Facts to check before building

Whether Bing honours `state`, PKCE and a loopback redirect; Bing's
refresh `grant_type` (its table says `authorization_code`, its example
`refresh_token`); how to remove an app's access in Bing; Bing's quotas,
JSON method names and day ranges; whether either Google scope counts as
"sensitive" and needs verification for an External app (unverified apps
show a warning and have a 100-user cap); GA4 dimension and metric names;
when Google's daily quotas reset; that no Entra ID route to Bing data
exists. Google's and Bing's published facts were read on 31 October
2026 from:

- <https://developers.google.com/identity/protocols/oauth2/web-server>
- <https://developers.google.com/identity/protocols/oauth2/native-app>
- <https://developers.google.com/identity/protocols/oauth2>
- <https://developers.google.com/webmaster-tools/v1/how-tos/authorizing>
- <https://developers.google.com/webmaster-tools/v1/searchanalytics/query>
- <https://developers.google.com/webmaster-tools/v1/urlInspection.index/inspect>
- <https://developers.google.com/webmaster-tools/limits>
- <https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/runReport>
- <https://developers.google.com/analytics/devguides/reporting/data/v1/quotas>
- <https://developers.google.com/analytics/devguides/config/admin/v1/rest/v1beta/accountSummaries/list>
- <https://learn.microsoft.com/en-us/bingwebmaster/oauth2>
- <https://learn.microsoft.com/en-us/bingwebmaster/>
- <https://support.google.com/cloud/answer/7454865>
- <https://docs.rs/oauth2/latest/oauth2/>

### Experiment source profiles

[0150](0150-experiment-source-and-sensor-gates.md) defines
separately qualified projections from these read-only Search
Console and GA4 API connectors for 0148. Reuse the current
property, exact scope and credential boundary. Provider finality
and complete pagination alone supply no exact unit-day coverage
proof. In particular, a missing row remains unknown unless the
qualified profile proves a zero. This route adds no connector,
outside write, bulk export or evidence import.

For the Search Console source, TASK-0040-SCIENCE-GSC binds
AT-CN24 and AT-CN25 with 0150 IT-0150-GSC-COVERAGE. Keep exact
page, surface and native day scope with the producer's receipt.
For the distinct GA4 source, TASK-0040-SCIENCE-GA4 uses the
connector's AT-CN29, AT-CN33 and AT-CN35 boundary cases with
IT-0150-GA4-KEY-EVENT. Exact mature event population and coverage
remain 0150 admission obligations. A read-only provider response
or connector test cannot substitute for the required source proof.

## Limits

- Within AGENTS.md 5.6: pulls run in the service, one connection at a
  time, streaming one page of rows at a time, at most 48 MiB above the
  service's idle use, measured nightly. Access-key cache and waiting
  flows: under 1 MiB.
- Copied numbers for 1 million pages and 2 years carry a planning estimate
  of 2 to 6 GB, not a verified bound. The measured disk cap, fixtures and
  enforcement behaviour are readiness blockers owned by the connector storage
  record. Connector imports cannot qualify until admitted; the admitted cap
  is shown on the disk panel and counted by the disk watchdog.
- Every request has a 30-second timeout; every pull a 2-hour limit, with
  a checkpoint after each page of rows.
- Waiting flows expire after 10 minutes. Retention is two years.
- Quotas and rate limits as above, always below the providers' own.

## Footprint budget

This part adds nothing to public pages. It calls only Google and Bing,
from the owner's PC.

### Portfolio summary read

Return click totals for the two latest expected final Search Console
dates for the selected property, date zone and search type. Also
return whether an enabled connection needs renewal. Use only a
property-level total without page or query breakdowns. Until the
existing connector pull admits that source and budget in QR-090, the
trend is unavailable. A property switch clears prior figures. A gap
in the expected dates gives missing coverage even with an older pair.

Expose one bounded, site-scoped summary read under spec 0102. Its
source and failure cases run in IT-SS-06 with the owning module.

## How it is tested

Rust tests in the sanket crate, against fakes: wiremock for Google and
Bing, a test PostgreSQL, a fixed clock and seeded randomness.

- AT-CN01: a Search Console start holds every parameter of step 4 and a
  challenge equal to the S256 of the stored verifier.
- AT-CN02 (property): any seed gives a 64-character unreserved verifier;
  RFC 7636 appendix B's example verifier gives its published challenge.
- AT-CN03: a valid callback stores only the sealed refresh token and the
  connection is `connected`.
- AT-CN04 to AT-CN07: no matching flow, a reused `state`, a flow 10
  minutes and 1 second old, and a callback from another user's session
  each give SUT-621 with nothing stored.
- AT-CN08: `error=access_denied` gives SUT-622.
- AT-CN09: a different granted scope gives SUT-623, the token is
  revoked and nothing stored.
- AT-CN10: no refresh token gives SUT-624 and the access token is
  revoked.
- AT-CN11: a Bing flow with `state` echoed ends `connected`.
- AT-CN12: Bing with no `state` is accepted only for the same session,
  one flow, under 5 minutes; any other case gives SUT-621.
- AT-CN13: site 1's sealed token copied into site 2's row fails to open
  with SUT-635.
- AT-CN14: `rotate-key` killed midway and restarted keeps every secret
  openable throughout and ends on the new key.
- AT-CN15: with unique marker secrets, no log, span, metric, error,
  API answer or audit event from the whole suite holds a marker.
- AT-CN16 (loom and stress): 10 requests on an expired key make exactly
  one refresh.
- AT-CN17 (property): in any interleaving of refreshes, the stored token
  is the newest and the version only grows.
- AT-CN18: one `401` gives one refresh and one retry; a second ends the
  pull with SUT-630.
- AT-CN19: `invalid_grant` moves to `needs_reconnect`, rows stay, the
  coverage gap shows from that date, one grouped SUT-630 alert.
- AT-CN20: a token refused 7 days after connecting adds the Testing
  hint.
- AT-CN21: reconnecting after 5 days keeps id and property, backfills
  the 5 days and revokes the old token.
- AT-CN22: disconnect without a keep-or-delete choice is refused; with
  `delete`, revoke is called, secret and rows go, the audit event holds
  no secret.
- AT-CN23: revoke failing `503` six times still deletes the secret and
  shows SUT-636.
- AT-CN24: the synthetic shared pagination core reads 60,001 rows
  as three pages and stores them once. Replay leaves the same rows.
  The Search Analytics adapter fixture checks 24,999, 25,000,
  25,001 and 50,000 rows, and refuses a third data page. Only the
  exact property, day and type can resume. Keep omitted data
  unknown and mark the ceiling separately from coverage.
- AT-CN25 (failure injection): the synthetic uncapped pagination
  fixture resumes a pull killed after page 2 of 5 at page 3
  within 60 seconds, with no lost or doubled rows. A Search
  Analytics restart after page 1 can request page 2 only for the
  same admitted cursor and cannot dispatch beyond the ceiling.
  In separate cases, keep the site and connection ID the same and
  hold a fully formed response before committing its page. Rebind
  to another verified property with the same report day and grain,
  or withdraw the connection's source scope. Release the old reply.
  It adds no rows, coverage, checkpoint advance or completed event.
  Retry and replay the old pull identity. Neither dispatches another
  data request under the stale binding. A kept prior result appears
  only under its original property and generation with current archive
  permission. A later authorised pull for the new property succeeds
  separately. In a separate branch seed complete coverage and stored
  rows, hold the old page response and reselect the exact same property
  with a new source scope generation. Keep the day, report scope and
  synthetic values the same. Neither the delayed reply nor replay of
  the old pull supplies current rows or coverage, advances a checkpoint
  or emits completion. The retained old complete coverage only belongs
  to its original generation. A later authorised pull in the new
  generation establishes current rows and coverage at a separate key.
  Rerun it in that generation for the exact provider day and report scope.
  Replace that scope's rows and coverage once. The retained old generation,
  neighbouring provider day and different report scope stay unchanged.
  Keep binding and grant current and return a reply that identifies
  another property. Expect SUT-634 with no rows, coverage, cursor
  advance or completed event. Try a legacy checkpoint without
  binding proof. It cannot resume against the active property.
- AT-CN26 (property): any mix of pages and budget never exceeds the
  budget, keeps the group order and, over enough days, covers every
  indexable page.
- AT-CN27: `429` with `Retry-After: 30` waits at least 30 s on the fake
  clock; after 6 failures the pull is rescheduled, not dead-lettered.
- AT-CN28: site 1's used-up inspection quota pauses site 1 with SUT-632
  while site 2 runs at full speed.
- AT-CN29: `propertyQuota` at 9% of the hour stops the GA4 pull for the
  hour.
- AT-CN30: two sites connect, pull, export, erase and disconnect, and no
  row, alert or key crosses between them.
- AT-CN31: an erased search term's rows go and reports regenerate; on
  removal every sealed secret and data key goes and rows follow the
  choice.
- AT-CN32: a provider answer pointing to another host causes no
  connection to it.
- AT-CN33 (snapshot, insta): a missing required field gives SUT-634,
  rejects the page whole and files an AutoIssue.
- AT-CN34 (benchmark): 25,000 fixture rows parsed, resolved and
  committed within 250 ms.
- AT-CN35 (SQL, under each role): a query with no site filter as site 1
  sees none of site 2's rows; the reports role is denied on
  `connector_secret`.
- AT-CN36: a job 214 result holding NaN is refused and the verified champion
  remains unchanged; with no result the late days show "not yet available".
- AT-CN37: the retention job removes only rows over two years old, and
  coverage marks those dates as removed by retention, not missing.

Test depth follows ADR-0020. Only admitted strict-core components require
100% line, branch and MC/DC coverage and zero surviving killable mutants;
equivalent mutants require proof and independent review, and other code
follows the owning risk matrix. The flow checks, vault binding, host
allow-list, scope check, inspection budget and row-level security policies
retain their assigned mutation gates and property tests. Kani proves that
the state machine never reaches `connected` without an unused, unexpired,
matching flow; that the inspection plan never exceeds its budget; and that
the allow-list refuses every unlisted host. The app screens are tested with
a fake API.

Speed, at the push tier on Dell against fakes (provider time excluded):

- P-CN01: back from Google, "Connected" and the property list show
  within 1 second for 95% of attempts.
- P-CN02: one page of 25,000 Search Console rows is parsed, resolved
  and committed within 250 ms.
- P-CN03: the connectors screen lists within 200 ms at the 95th
  percentile with 3 connections and 2 years of coverage.
- P-CN04: the nightly pull for a site of 1 million pages finishes inside
  4 hours whenever quota allows.
- The row parser and address resolver are under the 5% median rule.

## Changes from the source

- Rewritten for the owner's desktop app and service: the app shows the
  screens in place of site admin pages, and the OAuth redirect returns
  to a loopback address on the owner's PC, not to the site's host.
- The master key is held by tijori in a service-only file, not a
  container secret.
- Admin pages become desktop app screens built from SDS, tested with a
  fake API.
- Events and registered pulls use spec 0019. Test a crash after the
  pull commit and a disconnect before its event is consumed.
- "Forum" becomes "site", so WordPress sites are covered too.
- Speed follows the 5% median rule; the 20x goal is kept (spec 0007).
- Speed targets run at the push tier on Dell.

Source: `docs/specifications/signals/connectors.md`,
`docs/specifications/signals/connectors-2.md`,
`docs/specifications/signals/connectors-3.md` and
`docs/specifications/signals/connectors-4.md` in the archive
`archive-2026-11-08-old-repo`.
