# 0040: Search Console, Bing and GA4 connectors

The connectors let the owner link a site to Google Search Console, Bing
Webmaster Tools and Google Analytics 4 (GA4) with their own accounts.
Each night Sutradhar copies the site's search numbers, read-only, to feed
SEO reports ([0044](0044-seo-signals-crawler.md)), darshak's analytics
([0034](0034-analytics.md)) and link ranking. It is roadmap Phase 6
item 1. In code it is the `connectors` module of sanket ("signal", the
SEO signals crate).

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
- Every scope asked for is read-only. Nothing is ever written to these
  services (no URL or sitemap submission). Notices of new pages are a
  plain list owned by the crawler ([0044](0044-seo-signals-crawler.md)).
- Owned elsewhere: SEO reports, merging GA4 with darshak, ranking and
  tuning, cloud storage of exports, the settings store and audit.

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

Search Console, per property:

- On connect and daily: `GET .../webmasters/v3/sites` to confirm the
  property and its permission level.
- Daily: `POST https://www.googleapis.com/webmasters/v3/sites/{siteUrl}/searchAnalytics/query`
  for the newest final date (`dataState` final), with `type` web, image,
  video, news and discover, grouped by date, page and query; by date,
  page, country and device; and by search appearance. `rowLimit` 25,000,
  paged with `startRow` until a short page.
- Backfill on connect: every earlier date the API returns, newest first.
- Daily: the sitemaps list.
- Daily URL inspection,
  `POST https://searchconsole.googleapis.com/v1/urlInspection/index:inspect`,
  within the budget (default 1,900 a day; Google allows 2,000 per
  property): new pages, then changed, then important (most clicks or
  sign-ups, ordered by ganit's indexing probability, lowest first), then
  a rotating sample so every indexable page is checked in turn.

GA4, per property:

- On connect:
  `GET https://analyticsadmin.googleapis.com/v1beta/accountSummaries`;
  the owner picks one property.
- Daily: `POST https://analyticsdata.googleapis.com/v1beta/properties/{id}:runReport`
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
terms or addresses. Each result is applied after its shadow test; the
last good one is kept if a job is late.

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
- Row-level security is the second wall: each transaction sets its site
  with `SET LOCAL`, and every forced policy matches on it.
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
- `connector_connection`: id, site_id, service, property, state,
  connected_by, connected_at, last_success_at, version.
- `connector_secret`: connection_id, sealed refresh token, key_id,
  nonce, version.
- `site_data_key`: site_id, sealed key, master_key_id.
- `connector_pull`: id, connection_id, kind, window, rows, quota used,
  outcome, error code, started_at, ended_at.
- `connector_coverage`: connection_id, date, status (complete, partial,
  missing).
- Copied numbers: `gsc_query_day`, `gsc_page_dim_day`,
  `gsc_inspection`, `gsc_sitemap`, `ga4_landing_day`, `bing_query_day`,
  `bing_page_day`, `bing_crawl_day`, keyed by connection, date and
  dimensions, with page addresses stored as thread or post ids where
  they resolve.

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

Outbox events (ids and counts only): `connector.connected`,
`connector.property_chosen`, `connector.pull_completed`,
`connector.pull_failed`, `connector.quota_exhausted`,
`connector.needs_reconnect`, `connector.disconnected`. SEO reports and
darshak refresh on `pull_completed`.

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

- Rows are committed per page with the pull's id. A crashed pull resumes
  from its checkpoint. A re-run replaces that date's rows in one
  transaction, so nothing counts twice.
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

- The owner may also connect DataForSEO, Semrush and Common Crawl, each
  with their own key. Their figures are labelled by source and never
  added to another's. They are designed in
  [0045](0045-backlink-crawl.md) and [0046](0046-web-link-study.md).
- Version 1 imports Search Console, GA4, Matomo, Plausible and Fathom
  files. No raw server logs, since they hold visitor addresses.
- The Coach's related-questions check reads Search Console questions
  ([0042](0042-coach-additions.md)); this scope covers it.

### Facts to check before building

Whether Bing honours `state`, PKCE and a loopback redirect; Bing's
refresh `grant_type` (its table says `authorization_code`, its example
`refresh_token`); how to remove an app's access in Bing; Bing's quotas,
JSON method names and day ranges; whether either Google scope counts as
"sensitive" and needs verification for an External app (unverified apps
show a warning and have a 100-user cap); GA4 dimension and metric names;
Search Console's history depth (believed 16 months) and daily row cap;
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

## Limits

- Within AGENTS.md 5.6: pulls run in the service, one connection at a
  time, streaming one page of rows at a time, at most 48 MiB above the
  service's idle use, measured nightly. Access-key cache and waiting
  flows: under 1 MiB.
- Copied numbers for 1 million pages and 2 years: estimated 2 to 6 GB on
  disk, to be measured, shown on the disk panel and counted by the disk
  watchdog.
- Every request has a 30-second timeout; every pull a 2-hour limit, with
  a checkpoint after each page of rows.
- Waiting flows expire after 10 minutes. Retention is two years.
- Quotas and rate limits as above, always below the providers' own.

## Footprint budget

This part adds nothing to public pages. It calls only Google and Bing,
from the owner's PC.

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
- AT-CN24: 60,001 rows read as three pages, stored once; a re-run leaves
  the same rows.
- AT-CN25 (failure injection): a pull killed after page 2 of 5 resumes
  at page 3 within 60 seconds, with no lost or doubled rows.
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
- AT-CN36: a job 214 result holding NaN is refused and the last good one
  kept; with no result the late days show "not yet available".
- AT-CN37: the retention job removes only rows over two years old, and
  coverage marks those dates as removed by retention, not missing.

Test depth: 100% line, branch and MC/DC coverage. The flow checks, the
vault binding, the host allow-list, the scope check, the inspection
budget and the row-level security policies need zero surviving mutants
and property tests. Kani proves that the state machine never reaches
`connected` without an unused, unexpired, matching flow; that the
inspection plan never exceeds its budget; and that the allow-list
refuses every unlisted host. The app screens are tested with a fake API.

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
- Admin pages become Dioxus app screens built from SDS, tested with a
  fake API.
- Events travel through the outbox in place of the old event bus.
- "Forum" becomes "site", so WordPress sites are covered too.
- Speed follows the 5% median rule; the 20x goal is kept (spec 0007).
- Speed targets run at the push tier on Dell.

Source: `docs/specifications/signals/connectors.md`,
`docs/specifications/signals/connectors-2.md`,
`docs/specifications/signals/connectors-3.md` and
`docs/specifications/signals/connectors-4.md` in the archive
`archive-2026-11-08-old-repo`.
