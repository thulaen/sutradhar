# 0025: Paid embeddings

This part lets the owner use a paid embedding model through AWS Bedrock,
with the owner's own key, in place of the local embedder for one site's
whole index. It is optional and off by default; local models always work
without it (AGENTS.md 5.5). It covers setup and safeguards, spending
caps and the fallback to the local model, the index and the rebuild,
running, the one network exception, failures, erasure and removal. It
is roadmap Phase 4 item 3, and builds on 0022 (reading and indexing),
0023 (matching and anchors) and 0024 (models).

## What it does

### Terms

- Provider: AWS Bedrock, reached through its paid programming interface
  (API). Route: one way of reaching it, for example one region's API
  host.
- Route record: the project's record of one checked route, with its
  checked facts and the date each was checked. It names no model and
  holds no price.
- Typed-in model: the name of a Bedrock embedding model the owner
  enters. Paid embedder: a typed-in model in the `embedder` role. Local
  embedder: Qwen3-Embedding-0.6B or a community model, run on the
  owner's PC in pure Rust at its published precision (0024).
- Key: the owner's own AWS credential that lets programs call Bedrock
  and be billed. Called "key" here, never "token", which means a unit
  of text. A Hub access token (0024) is a different thing.
- Provider client: the one process that may call Bedrock.
- Input: one text sent to be turned into a vector: a piece, an extras
  input, a title or an anchor phrase, exactly as
  0022 and 0023 define them.
- Spending caps: the rebuild budget (most allowed for one rebuild), the
  monthly cap (everything else in a calendar month) and the daily
  ceiling (a share of the monthly cap one day may use).
- Forecast: Sutradhar's estimate of tokens, money and time from the
  site's own counted text and the price in force.
- Hard stop: no request starts that could take spending, by
  Sutradhar's own count, past a cap.
- Fallback: at a cap, the local model that served before the paid one
  (Qwen3-Embedding-0.6B, or a community model that served before)
  takes over automatically from its saved index, and the paid index
  waits to catch up. "The local model" below means whichever it is;
  screens call its kept generation the "saved local index", or the
  saved index of the named community model.
- Send log: what was sent and when, by ids and versions, never text.
- Retention: how long Bedrock keeps inputs it received, under its terms.
- Pause after repeated failures: calls stop for a short time after many
  fail, then one test call decides whether to start again.

### What changes and what does not

1. Unchanged:
   - one embedder per index generation; vectors from different models
     never share an index, so a paid model replaces the local model for the
     whole index and cannot shorten the local model's first run;
   - pieces of the piece size in the active embedder's tokenizer
     (1,024 by default, never fewer), titles prefixed, the overlap
     setting (64 by default), nothing truncated; quotes,
     code, spoilers and signatures skipped;
   - changing the embedder is a full rebuild with new suggestions paused
     and a one-step switch; suggestions already made stay approvable;
     the thread-level index comes first;
   - the compatibility rule, the key check on every search, exact
     search, and a moderator's approval of every link.
2. New: guest-visible text leaves the owner's PC, so consent,
   disclosure, terms and erasure rules apply; it costs money per token,
   so a forecast, caps and the fallback apply; it needs the network, so
   one process gets a narrow exception to "reading makes no network
   calls"; the provider can be slow, rate-limited, down, changed or
   withdrawn, so failure handling and drift checks apply; the local model is not
   loaded while a paid embedder serves, until a fallback loads it.
3. Only the `embedder` role can be paid. The reranker, the anchor phrase
   finder and potion stay local. Anchor phrase vectors come from the
   active embedder. Paid rerankers, summaries and other paid model
   services are out of scope.

### Checked routes and typed-in models

1. A route is offered only once its terms (training use, retention and
   service locations) have been checked and dated on Dell from AWS's
   current official pages, in its admission record. Resellers and other
   hosts are not offered, nor a route needing a contract the owner
   cannot sign online.
2. Route records are `[[provider]]` tables in `models.lock`, with no
   files to download. Fields: `id`, `role` (`embedder`), `purpose`,
   `provider`, `route`, `api_hosts`, `key_auth`, `size_request`,
   `token_count`, `tokenizer`, `tokenizer_sha256`, `excluded_models`,
   `terms_checked`, `retention_days` (never unlimited),
   `training_on_inputs = false`, `locations`, `region_choice` and
   `admission`. Examples hold placeholders only, never facts.
3. Only `sutra models pin-provider <record file>`, run on Dell, writes a
   record, from the admission record; it calls no provider and needs no
   key. It refuses a record whose terms allow training on inputs or set
   no limit on how long inputs are kept. A published local tokenizer is
   pinned with SHA-256 like any model file and must be data-only. A
   change to a record needs the owner's approval.
4. The owner types in any Bedrock embedding model name. The project
   keeps no model list and reviews no model. Sutradhar checks only that
   the model answers as the index needs (key check and self-test); its
   price and limits are read from the provider when used, or entered by
   the owner and confirmed every 180 days; the trial gives the evidence
   of how well it links.
5. Tier and badge: a tier "paid", badge "paid service: sends
   guest-visible text to AWS Bedrock", with the lines "model chosen by
   the owner, not reviewed by the Sutradhar project" and "not part of
   Sutradhar's release quality evidence", because the project never
   sends site content to a provider to test it.

### Who may set it up

- A Cedar permission, "Manage paid embeddings", separate from "Manage
  models", because this choice spends money and sends text out. Viewing
  the status needs "View models".
- `models.paid.enabled` is false by default. While false, the paid
  section shows only an explanation, no provider client runs and Bedrock
  is never contacted. Turning it on sends nothing by itself.
- Every screen action and its `sutra models paid` command run the same
  permission-checked action.

### Setup steps

Each step must finish before the next is offered. No site content
leaves the PC before step 8, and then only the trial's sample.

1. Choose a checked route and type in the model name. Only routes with
   a record are listed. The screen shows each checked fact with its
   date (retention, training use, service locations, region choice, API
   hosts) and says the model is not reviewed. A fact checked over 180
   days ago shows "not re-checked since <date>".
2. Terms check: the owner confirms the account settings only they can
   see (below). If any cannot be confirmed, setup stops.
3. Disclosure: the screen states what leaves the PC and what Bedrock may
   keep, offers a paragraph for each site's privacy notice, and asks the
   owner to confirm the notice covers it.
4. Key: entered and checked with one fixed neutral sentence sent to the
   typed-in model, which must return one vector.
5. Self-test, fixed neutral text only: 0024's four embedder sentences
   (vector size, finite values, paraphrase ordering), and one fixed
   neutral text of the piece size (1,024 tokens by default) by the
   record's counting method, which must be accepted whole. Reported
   token counts must agree with that method. The vector size requested
   is always 1,024. A model that cannot give 1,024 numbers, fails, or
   that the checked terms exclude is refused with its reason (PE-E17).
   The outputs become the reference for the daily drift check. The
   model is "added (inactive)".
6. Price and limits: read from the provider, or, where not published to
   programs, entered by the owner, dated.
7. Forecast and caps.
8. Trial on a sample of the site's own links.
9. Activation: the rebuild, with new suggestions paused.

Every step writes one audit event: time, actor, action, route record,
model name, region chosen (or "none") and result code; identifying
details kept apart from the minimal event.

### The key

1. Entry: in a masked field in the app, sent over the owner-only socket,
   or `sutra models paid key set` from a hidden prompt or `--key-stdin`;
   never an argument and never an environment variable. The owner is
   advised to make a key for Sutradhar alone, limited to embeddings and
   with its own spending limit where AWS allows, so it can be revoked
   alone.
2. Check: one request with one fixed neutral sentence and no site
   content. A refused key is shown plainly (PE-E01) and discarded, not
   stored; a model Bedrock does not know gives PE-E17.
3. Storage: held by tijori in one file, `secrets/paid-embedder.key`, in
   the service's data directory, folder `0700`, file `0400`, owned by
   the service account. It is outside the settings system, so never in
   settings history, and excluded from every export, diagnostics
   bundle, backup and support bundle. Only the provider client reads
   it, at start and after a change, and holds it in memory. It is sent
   only in the authentication header, over HTTPS, to the record's API
   hosts.
4. Never shown, never logged: after entry the screen shows only "key set
   on <date>, fingerprint <8 characters>", the first 8 hexadecimal
   characters of the key's SHA-256. Every log, error, metric, trace,
   progress message, audit event, diagnostics bundle and crash report
   is scrubbed of the stored key's value and of anything shaped like an
   AWS key. Audit events record that a key was set, replaced, removed or
   refused, with its fingerprint only.
5. Replacing the key for the same model needs only the check, no
   rebuild; the file is overwritten and the old value dropped from
   memory. Removing it stops every call at once and follows the
   switching-off rule. Pausing a site keeps it; removal always deletes
   it, with no export.

### The terms check

1. Checked by the project per route, with dates, in the admission
   record: the terms; whether inputs are used for training or to
   improve products, and how that is turned off; retention and any
   deletion route; service locations and region choice; API hosts; key
   authentication; and any models the terms treat differently (such as
   preview models), with how to recognise them. A route is offered only
   if its terms say inputs are not used for training for the account
   type used, and are kept only for a stated, limited time. "Keep no
   copy of what was sent" applies: the send log holds ids, never text.
2. Confirmed by the owner, each confirmation recorded:
   - the account is a paid account of the checked type, not a free tier
     whose inputs may improve products;
   - inputs are not used for training on this account, with the setting
     that ensures it, where one exists;
   - the retention shown applies, or a shorter one is agreed;
   - the region chosen, where a choice exists, otherwise processing is
     wherever the checked terms say. It is applied as the record says
     (that region's API host in `api_hosts`, or a region field in each
     request) and recorded in the setup audit event, the consent record
     and `models.paid.region`;
   - any data processing agreement is accepted, where needed;
   - a spending limit is also set at AWS, where offered.
3. When terms change: the project learns of changes only when it
   re-checks for a release; watching the account is the owner's
   responsibility. When a release records a changed term that matters
   (training, retention, locations), calls pause until the owner
   confirms; meanwhile new and edited posts wait and new suggestions
   pause. A route whose re-checked terms no longer pass loses its
   record, and every call stops, as for a withdrawn model (PE-E02).

### Consent and disclosure

1. Consent is the owner's, per site, audited with the route record,
   model name, region, terms date and a SHA-256 of the disclosure text.
2. Members are told through the site's privacy notice. Sutradhar never
   edits it; it offers a paragraph and records the owner's confirmation.
   There is no per-member choice: only text any visitor can read is
   sent.
3. The disclosure states: route and model; that the model is the owner's
   choice and not reviewed; what is sent and never sent; that only
   guest-visible text is sent; the redaction, and that member names,
   mentions included, are sent as guests read them; retention and
   training terms; service locations and the region chosen; that AWS
   also sees the PC's network address and the account; that vectors
   come back and are stored on the PC; that at a cap the local model
   that served before takes over and changed posts are sent later to
   catch up; and that switching back or removing cannot recall what was
   sent. It says post text can hold personal data, never calling it
   anonymous.

### What leaves the PC

1. Sent:
   - inputs built from guest-visible content at the moment of sending,
     exactly as a local embedder would get them: title-prefixed pieces
     of readable body text; extras inputs (alt text, attachment names,
     stored link-preview and media titles, each read in full);
     destination titles; anchor phrases. Quotes,
     code, spoilers and signatures never; censored words already masked;
   - fixed neutral text for the key check, self-test and probes;
   - with each request: the key in the authentication header, the model
     name, the requested vector size, the region field only where the
     record says so, and any field the API requires;
   - what any connection reveals: the PC's public address, the time and
     a neutral user agent naming only the client library.
2. Never sent: item, thread, forum section or member ids; dates;
   reaction or view counts; the site's address or settings; member data
   beyond the text itself; any optional end-user id, label or metadata
   field; anything from staff-only, members-only, private, draft or
   unapproved content.
3. Checks just before sending: the item is still guest-visible, its
   content version unchanged, no erasure waiting, readable text only,
   redaction applied, and it fits the window. An item that changed is
   dropped and read again under its new version. The same checks apply
   to the catch-up. A visibility change after an input has left cannot
   recall it; the send log shows it and a metric counts it.
4. Redaction: email addresses and phone numbers, and only these, are
   replaced in the copy sent by `[email]` and `[phone]`. Names are sent
   as guests read them; member ids never are. Stored text, positions and
   anchors are unchanged; the paid vectors come from the redacted copy.
   The patterns are first-party, reviewed and time-limited; they reduce,
   and cannot remove, personal data, and the disclosure says so.
5. Send log: per request, the item id and content version of each
   input, input kind, token count, time, outcome and the provider's
   request id. No text and no digest of text. Rows are kept for the
   recorded retention plus 30 days, then reduced to daily totals by
   kind. It serves erasure receipts and audits.

### Spending

1. Price and limits: no project record holds a price or a model's
   limits. The provider client reads the model's price per million input
   tokens, its input window, per-request limits and the account's rate
   limits at setup, before every forecast, trial and activation, and
   daily while serving, wherever they are published to programs; each
   is shown with the time read.
2. An entered price (where none can be read) is dated and confirmed
   every 180 days; an older one blocks activation (PE-E12). While
   serving, alerts start 30 days before; then an unconfirmed price acts
   as a cap and the local model takes over until it is confirmed. Where both
   exist, forecasts and caps use the higher price. Prices stay in the
   billing currency, never converted. A rise while serving alerts and
   applies from then on.
3. Counting: Sutradhar counts every input's tokens by the record's
   method before sending. Where a response reports tokens used, that is
   recorded too and the ledger uses the higher; a daily difference over
   2% warns. Every request that may have been billed counts, timed-out
   ones included, until it is known whether failures are billed. A
   request refused before sending counts nothing.
4. Forecast:
   - rebuild cost: after 0022's first pass gives every eligible item's
     readable text, Sutradhar cuts pieces by the counting method and
     counts pieces, extras and titles; anchor
     phrases are estimated from a seeded sample of 1,000 posts. Money
     is tokens times price over a million;
   - shown as a range: the count, and the count plus 10% for retries,
     crash repeats (at most one batch per worker) and posts written
     during the rebuild at the last 30 days' rate;
   - rebuild time, which is also the pause: tokens over the lower of
     the account's token rate limit and the trial's throughput;
   - monthly cost: the last 30 days' new and edited posts, title edits
     (which re-read a whole thread) and anchor phrases, beside the
     monthly cap; summary vectors need no paid call, as 0022 builds
     them from stored piece vectors;
   - catch-up cost: every item changed since a fallback;
   - for scale only: about 165 million local-tokenizer tokens for a million
     posts' pieces, so about 165 times the price per million;
   - every forecast is labelled an estimate; measured figures replace
     it hourly during a rebuild.
5. Caps, each a hard stop: the rebuild budget, set from the forecast,
   which also pays for posts read into the new generation (activation
   is refused while the forecast's upper end exceeds it); the monthly
   cap, trials and catch-ups included; the daily ceiling,
   `models.paid.cap.daily_share` of the monthly cap (10%). A rebuild
   that is not the owner's own choice, such as new reading rules in an
   upgrade or a changed piece size or overlap, also needs its forecast
   approved against a new rebuild budget before it starts.
6. The hard stop: before each request its cost is reserved in the
   ledger in one atomic step; if spent plus outstanding reservations
   plus this request would pass any cap, it does not start. After the
   response the reservation is settled with the counted figure. AWS's
   bill may differ; the screen shows both, and AWS's own spending limit
   is the final guard.
7. At a cap, the fallback: when the monthly cap or daily ceiling holds a
   request back while a paid embedder serves, Sutradhar switches to
   the local model automatically from its saved index in one step, as 0024's
   rollback does:
   - no further paid request starts; the local model is loaded ahead of optional
     work such as 0017's map projection, which pauses;
   - posts written or edited since the local model's generation was last current
     are read again by the local model, live lane first; only those wait, as
     "waiting: spending cap reached"; other suggestions continue and
     earlier ones stay approvable;
   - new and edited posts are read by the local model and count toward the
     30-second figure as usual;
   - the paid generation is kept and falls behind; deletions, erasures
     and losses of guest visibility still apply to it at once.
8. Returning: when the period resets, the cap is raised or an
   unconfirmed price is confirmed, the catch-up is forecast and shown.
   It starts on its own (`models.paid.return.automatic`) if its upper
   end fits what is left of the monthly cap; otherwise the screen offers
   raising the cap. While the local model serves, the provider client sends
   every item changed since the fallback, and during the catch-up, in the
   backlog lane, stopping each day at the daily ceiling and going on the
   next. When the paid generation is whole again, one atomic switch returns
   to it and nothing waits. If the monthly cap is reached first it stops
   until next month. The owner may instead roll back, which ends the
   return. A return does not pause suggestions.
9. During an activation's rebuild, a cap cancels the activation: the
   embedder that served before is loaded again and suggestions resume,
   as a cancelled activation does. The partial paid generation is kept;
   resuming needs a raised budget and costs the rest plus posts changed
   since, forecast and shown. A rebuild that pauses nothing (such as
   one for new reading rules) or a trial just stops at a cap while the
   serving generation continues, and resumes when the cap allows.
10. With no saved local index able to serve (deleted to free disk or at
    a second paid activation, or never built because the paid
    generation was the first), paid calls stop at a cap: new and edited
    posts wait as "waiting: spending cap reached" until the period
    resets or the cap is raised, then are read oldest first, live lane
    before backlog, each checked again just before sending. The screen
    offers raising the cap or a full local rebuild, and warns before
    anyone deletes the saved index that the fallback will then be
    impossible. An unconfirmed entered price is handled the same way.
11. Alerts at 50%, 80%, 95% and 100% of each cap, each once per period,
    grouped in the app with a code and manual link. The 100% alert says
    the local model has taken over, or, with no saved index, that calls have
    stopped; a return also alerts. A day over three times the average of
    the last seven, outside a rebuild, alerts too. Nothing stops
    silently at a budget.

### Tokens, pieces and the compatibility key

1. For a paid embedder, pieces hold at most the piece size by the
   record's counting method, title included. The piece size is never
   below 1,024, so a model whose window is under it is not offered.
2. Where a tokenizer for Bedrock's embedding models is published for
   local use, its files are pinned with SHA-256, data-only, load offline
   and never fetch files at run time; its fingerprint enters the key.
   Otherwise the provider's token-counting service is used: pieces are
   cut by a cautious local estimate, then confirmed before embedding,
   and those counting calls are disclosed and logged as content sent;
   forecasts use the cautious estimate, so no text is sent to forecast.
   If checking shows the counting service is needed, that is put to the
   owner before the record is admitted. A route with neither method is
   not offered.
3. Before every request each input is counted again; one over the window
   (as read, or else the piece size the self-test proved), or a piece
   over the piece size, is refused with an error code and reported as
   a defect.
   Nothing is cut silently.
4. The paid key, in 0024's terms: entry id is the record's; repository
   becomes route plus typed-in model name; revision becomes the model
   version the API reports, otherwise the setup self-test date; vector
   size is the size requested and checked on every response; tokenizer
   fingerprint is the pinned tokenizer's, or the counting service's
   identity; plus a fingerprint of the setup self-test outputs, because
   a model can change behind the same name.
5. 0022 stores the key in the generation record; every search carries
   it and the index refuses another key. A fallback or return switches
   the key with the generation. Anchor phrase vectors share the key.

### The trial

1. Activation needs a finished trial; it is the evidence the owner
   decides on. It is 0024's link reconstruction on the site's own
   member-written links, except:
   - it runs only after setup steps 1 to 7;
   - sample 200 links and pool 2,000 threads by default
     (`models.paid.trial.sample_size`, `.pool_size`), to limit cost and
     text sent; its forecast is shown and counted against the monthly
     cap;
   - the paid side embeds pool and sources through Bedrock; the local
     embedder reuses its stored vectors; suggestions continue;
   - results: how often the real link came first, in the top 3 and top
     5; anchor match rate; usual and slowest-1-in-20 response times;
     projected first-suggestion time; rebuild forecast (money and time);
     each with sample size and margin of error. Results store numbers
     and ids only; the trial's paid vectors are deleted when it ends.

### Activation, with suggestions paused

1. Before starting, the screen states: new suggestions pause for the
   whole rebuild; forecast time and cost; the rebuild budget; the
   projected first-suggestion time and, if it misses 30 seconds where
   the local model meets it, a warning; and that what is sent cannot be
   recalled. The owner confirms; the confirmation is audited. A slower
   model is allowed after that warning, and the health screen then says
   the site runs outside the qualified target while the paid embedder
   serves.
2. 0022 builds a whole new generation, thread-level index first,
   then per-post; posts written or edited meanwhile are read into it.
   New suggestions pause from both queues; earlier ones stay approvable;
   readiness says "not ready for suggestions: model update running".
   the local model is unloaded for the rebuild.
3. On completion one atomic change switches embedder and index; new
   suggestions resume, posts saved during the pause first, reported
   separately. The local model stays unloaded; its files and saved
   index are kept for rollback and fallback, with disk use shown.
4. 0024 keeps one previous model per role, so activating a second paid
   model deletes them: before that the screen warns, naming the local
   model, that its saved index will be deleted and that a later cap will
   then stop calls with posts waiting; the confirmation is audited.
5. First generation: a paid embedder chosen before any generation exists makes
   the first one, serving once its thread-level index is built. Chosen during
   the local model's first run, the local model's partial generation is kept,
   stopped, for resuming after a rollback. Either way the screen says there is
   no fallback until a local generation can serve.
6. Before the switch, the owner or a cap can cancel: the local model is loaded
   again, suggestions resume, posts written meanwhile are read by it;
   the partial paid generation is kept or deleted on request. Money
   spent stays spent, and is shown.

### Rollback and switching off

1. Rollback with the saved index (`sutra models rollback --role
   embedder`, or "Roll back"): one step back to the previous local
   model; posts read since the switch are read again; only those wait.
   The provider client stops. During a fallback, rollback makes that
   model the chosen embedder and ends the return.
2. Without it: switching back is a full local rebuild, one to several
   weeks per million posts by 0022's estimate, with new suggestions
   paused. The screen states this before the saved index is deleted and
   before the rebuild starts.
3. After a rollback the paid generation is kept as the previous one
   until the next activation; re-activating costs only posts changed
   since, if key, terms and drift check are still in order.
4. Switching off (`models.paid.enabled` false), removing the key, or a
   terms change awaiting confirmation stops every call within 5
   seconds. New and edited posts wait. The screen offers the rollback at
   once, without starting it. During a fallback, switching off ends the
   return. Data stays until the owner rolls back, removes the paid
   generation or removes the site.

### The provider client

1. One separate process of the service, in the service slice, started
   only while a paid embedder is set up, on trial, serving or catching
   up. It alone holds the key and alone reaches Bedrock.
2. `ModelRegistry::get(Role::Embedder)` returns a `PaidEmbedder`, a Rust
   type implementing 0024's `Embedder` trait, so 0022 and 0023 call it
   as they call the local model. It passes inputs to the provider client over a
   local connection and receives vectors back.
3. Its error type adds variants callers treat as "the embedder is
   missing or failed": `ProviderUnavailable`, `SpendingStopped`,
   `KeyRefused`, `ModelWithdrawn`, `ModelUnsuitable`, `InputRefused`
   and `OutputChanged`. Items wait visibly; nothing is invented.
4. Requests use one admitted plain Rust HTTP library and first-party
   request code, not AWS's own client libraries, to keep dependencies
   few and control retries, logging and telemetry.

### Lanes, batching and retries

1. 0022's two lanes stay: live (new and edited posts) and backlog,
   which also carries the catch-up. A live post's pieces go in one
   request and its anchor phrases in one more; backlog requests fill to
   the per-request limits and 0022's 30 seconds of work.
2. The live lane keeps 20% of the account's rate limit
   (`models.paid.rate.live_share`).
3. Results are written idempotently, keyed by item id, content version
   and model key, with a checkpoint after each committed batch. A crash
   after a response but before its commit repeats at most that batch,
   paid again; the forecast margin allows for it.
4. Responses are not cached by text; no copy of sent text is kept apart
   from 0022's stored readable text. Cheaper delayed batch services are
   not used in the first version, because they keep uploaded files at
   the provider and need their own terms check.
5. A local rate limiter keeps requests and tokens below the limits read
   from the provider; until read, it starts at
   `models.paid.rate.start` (60 requests a minute) and adjusts to what
   the provider reports and refuses.
6. A rate-limit refusal is waited out for the time given, or with pauses
   from 1 second doubling to 5 minutes, with random spread.
7. Timeouts: 10 seconds live, 60 seconds backlog. Live: at most 2
   retries within 20 seconds, then the post waits in the live queue as
   "waiting: provider slow" or "waiting: provider unavailable" and is
   retried first when calls succeed. Backlog: an item failing 5 times is
   marked failed with its code and retried later with growing delays;
   the rest continue.
8. Never retried: a refused key (PE-E01), a withdrawn model (PE-E02),
   a refused input (PE-E07), a request held at a cap (PE-E08) and an
   unsuitable model (PE-E17).
9. Pause after failures: when at least half of the last 20 calls failed
   with retryable errors, or 5 in a row timed out, calls pause for 1
   minute; one probe with a fixed neutral sentence then resumes them or
   doubles the pause, up to 15 minutes. While paused the row is
   "degraded", readiness says "not ready for suggestions: provider
   unavailable", and new posts are still accepted and wait in order.
   The fallback is for the spending caps and an unconfirmed price only;
   outages, a refused key, a withdrawn model, changed terms and
   switching off keep posts waiting and offer the rollback.
10. Every response must hold one vector per input, in order, each of the
    requested size, with finite values. Sutradhar normalises vectors
    itself. A failing response is retried once; a second failure marks
    those items failed (PE-E06). A bad vector is never stored.

### Drift and withdrawal

1. Daily, and after any provider fault lasting over an hour, the
   self-test sends its four sentences. Each vector must have cosine
   similarity of at least 0.999 with the setup reference, and pairwise
   similarities must match within 0.002. How repeatable Bedrock's
   vectors are is to be checked and may change these figures.
2. A failure means the model changed behind its name: "index mismatch",
   no new suggestions, earlier ones approvable, rebuild or rollback
   offered (PE-E09).
3. A withdrawn model, or a route that lost its record, gives PE-E02:
   every call stops, the row is "failed" and rollback is offered at
   once. A model's retirement date, where reported, is shown 90 days
   ahead.

### Service levels

1. Availability: in a provider outage the health check passes and new
   posts are accepted into the live queue, so no availability is lost.
   If the live queue reaches its bound (`models.paid.live_queue_limit`,
   100,000 posts), the service stops taking new posts from the site
   (PE-E16); the add-on keeps them for the next call, and that time
   counts as unavailable.
2. The 30-second level: posts delayed by provider faults or slowness
   count, with the cause shown. Posts waiting because of a cap are
   reported separately (those re-read by the local model after a fallback, or
   waiting with no saved index). Posts read by the local model during a fallback
   or a catch-up count as usual.
3. The live path: the local model's step (about 0.6 to 1.2 seconds per
   typical post) is replaced by one request for pieces and one for
   anchor phrases, measured. Exact search time grows with vector size,
   hence always 1,024 numbers.

### Health, readiness and metrics

1. The paid row uses only 0024's statuses, with plain labels: "not
   installed" ("not set up", or no key); "verifying" ("checking the key
   and model"); "loaded" ("ready: connected to AWS Bedrock");
   "degraded" (error rate over 1%, the slowest 1 in 20 calls over twice
   the trial's figure in the last hour, or calls paused); "failed"
   ("stopped: <reason>": key refused, model withdrawn or unsuitable,
   record removed, secret unreadable, or a cap with no saved index);
   "index mismatch" ("the model changed: rebuild or roll back").
   "downloading" and "update available" never show.
2. Fields: revision shows model and version; "checksum last verified"
   shows the last passed drift check; memory is the provider client's;
   device shows "outside: AWS Bedrock". Added: badge, spending this
   period against each cap, forecast, price and limits with when each
   was read or entered, key fingerprint and date, terms date and
   retention.
3. Lifecycle labels: "<local model> serving at spending cap", "<local
   model> serving: price unconfirmed", "catching up to return",
   "stopped at spending cap" and "paused: terms changed".
4. Readiness reasons, each "not ready for suggestions" with a code:
   "provider unavailable", "spending cap reached" (only with no saved
   index), "key refused", "model withdrawn", "terms awaiting
   confirmation". During a fallback readiness is ready.
5. Metrics, labels holding only record id, lane, input kind, outcome
   and status code, never content ids, text or the key:
   `sutradhar_paid_embedding_requests_total`,
   `sutradhar_paid_embedding_request_seconds` (histogram),
   `sutradhar_paid_embedding_tokens_total`,
   `sutradhar_paid_embedding_spend_total` (millionths of the billing
   currency), `sutradhar_paid_embedding_cap_ratio`,
   `sutradhar_paid_embedding_breaker_open`,
   `sutradhar_paid_embedding_fallback_active` and
   `sutradhar_paid_embedding_sent_after_hidden_total`. Spans carry the
   same attributes. Each is listed on the "what we collect" screen.

### The network exception

- Who: the provider client alone, besides 0024's fetcher. The model host
  and every other process keep no outbound network beyond local links
  and the calls to the sites.
- When: only while `models.paid.enabled` is true and a key is set: key
  check, self-test, reading price and limits, trial, rebuild, serving,
  catch-up and probes. Never to look for terms or models.
- Where: HTTPS only, with certificate checks no setting can turn off, to
  the record's API hosts. Redirects elsewhere are refused. An HTTPS
  proxy may be set (`models.paid.proxy`); the allowlist still applies.
- What: "What leaves the PC" above, and nothing else.
- Enforced by systemd network rules on the processes, shared with the
  fetcher, and proven by test.

### Updates to a route record

A release may re-check a route. A changed term follows the terms rule; a
changed API host is shown and applied after the owner confirms; a
changed counting method changes the key, so only a new activation adopts
it. Models are never updated by a release: the owner may type in another
model, adopted only by a new activation with its own self-test, trial,
forecast, budget and pause, which deletes the saved index after a
warning.

### Interfaces

- `PaidEmbedder` and the provider client, above.
- App to service over the owner-only socket, Cedar-checked: `PaidKeySet`
  (the key, once); `PaidSetupStep` (each step's confirmations and the
  model name); `PaidStatus` back (status, spending, forecast, price and
  limits, lifecycle label, reasons, never the key).
- Events: `models.paid.cap_warning`, `models.paid.cap_reached`,
  `models.paid.fallback`, `models.paid.returned`,
  `models.paid.provider_fault`, `models.paid.key_refused`,
  `models.paid.output_changed`, `models.paid.terms_changed`, with
  0024's `models.switched` and `models.rolled_back`.
- The paid row in the models health check; setup checks, trials,
  forecasts, rebuilds and catch-ups report through 0016.

### Settings

Applied atomically between jobs, with history; the key is never a
setting.

- `models.paid.enabled`: false. `models.paid.provider`: none.
  `models.paid.model`: none, typed in.
- `models.paid.price_per_million`: read, or entered.
- `models.paid.cap.rebuild`, `models.paid.cap.monthly`: none, required
  before activation. `models.paid.cap.daily_share`: 10%.
- `models.paid.alert_percents`: 50, 80, 95, 100.
- `models.paid.return.automatic`: true.
- `models.paid.vector_size`: 1,024, shown, not changeable.
- `models.paid.region`: the route's default.
- `models.paid.timeout.live_seconds`: 10;
  `models.paid.timeout.backlog_seconds`: 60.
- `models.paid.retries.live`: 2; `models.paid.retries.backlog`: 5.
- `models.paid.rate.live_share`: 20%; `models.paid.rate.start`: 60 a
  minute.
- `models.paid.live_queue_limit`: 100,000 posts.
- `models.paid.trial.sample_size`: 200; `.pool_size`: 2,000.
- `models.paid.proxy`: none.

Fixed, not settings: guest-visible only; the redaction; the hard stop;
the fallback at a cap; the trial before activation; the pause during a
rebuild; certificate checks; the allowlist.

### Commands

Each uses its screen's action, needs "Manage paid embeddings" (status
needs "View models"), shows progress past two seconds, offers `--json`
and exits non-zero on failure.

- `sutra models paid list`, `paid status`, `paid spend`.
- `sutra models paid setup <route> --model <name>`; `paid key set`
  (hidden prompt or `--key-stdin`), `paid key remove`.
- `sutra models paid price set <amount>`; `paid forecast`;
  `paid cap set --rebuild <amount> --monthly <amount>
  [--daily-share <percent>]`.
- `sutra models paid trial`, `paid activate`, `paid cancel`; rollback
  is `sutra models rollback --role embedder`.
- Dell only: `sutra models pin-provider <record file>`.

### Failures

Each has a plain-English message, an error code and a manual link. None
touches the sites' pages. Missing vectors show as waiting or failed,
never invented or treated as zero.

- PE-E01 key refused: every call stops; "failed"; alert; a new key is
  needed. One probe every 10 minutes while the setting stays on.
- PE-E02 model withdrawn or not found after activation, or record
  removed by a release: every call stops; "failed"; alert; rollback
  offered at once.
- PE-E03 rate-limited: waited out; "degraded" if it lasts; live keeps
  its share.
- PE-E04 provider error or down: growing pauses; pause after repeated
  failures; posts wait in order. PE-E05 timeout: as PE-E04; a live post
  waits in the live queue.
- PE-E06 invalid response: retried once, then those items fail.
- PE-E07 input refused: that item fails and is reported as a defect,
  never shortened or reworded to pass.
- PE-E08 a cap reached: fallback, or during a rebuild the activation is
  cancelled; with no saved index, calls stop.
- PE-E09 output changed: "index mismatch"; no new suggestions; earlier
  ones approvable; rebuild or rollback offered.
- PE-E10 host unreachable, refused redirect or certificate failure: as
  PE-E04, with the reason; checks never relaxed.
- PE-E11 secret file missing, unreadable or with changed permissions:
  calls stop; "failed"; the key must be entered again.
- PE-E12 price missing, or entered over 180 days ago, at activation:
  refused until read or confirmed; while serving, an unconfirmed price
  acts as a cap.
- PE-E13 terms changed and awaiting confirmation: calls pause.
- PE-E14 rebuild or activation failure: the local model is loaded again and
  suggestions resume on the old index; the partial generation is kept.
- PE-E15 rollback impossible, saved index missing: the full rebuild is
  offered with its estimate and pause.
- PE-E16 live queue full: new posts are not taken; counts as
  unavailable.
- PE-E17 typed-in model unsuitable at setup: unknown, not an embedding
  model, refuses the 1,024-token text, cannot give 1,024 numbers, gives
  unusable vectors, reports counts that disagree, or is excluded by the
  terms. Setup stops with the reason; nothing is stored and no site text
  has been sent.

An item made private, deleted or erased before its request leaves is
dropped without an error, and counted.

### Privacy, erasure and removal

1. Logs, metrics, traces, progress and audit events carry ids, counts,
   durations, codes and the key's fingerprint, never post text,
   anchors, names or the key. Vectors from Bedrock are content, under
   0022's deletion, erasure and removal rules.
2. No site's text is sent by the project, its agents or its tests. The
   Dell lab's private datasets are never sent to a paid embedder; any
   such comparison needs the owner's separate approval. Repository
   files, fixtures and logs never name the owner's sites, hosts, paths
   or keys. Legal advice is out of scope: Sutradhar states facts and a
   suggested notice paragraph; the legal basis is the site owner's.
3. Erasure on the PC: an erased or deleted item's paid vectors, pieces
   and summary contributions are purged at once from every kept
   generation, the local model's included, also during a fallback; the catch-up
   never sends an erased item.
4. At the provider, Sutradhar cannot delete what was sent; no deletion
   route is assumed; Bedrock keeps inputs for the recorded retention.
   The erasure receipt says whether the item's text was sent, when, to
   which route and model, and the retention then recorded, from the send
   log; ids and times only. The manual tells site owners they may ask
   AWS under its own process, using request ids from the send log.
5. Removal rows, added to 0024's store table:
   - always deleted, no export: the paid generation's vectors, segment
     files, row tables and generation records; the trial's paid
     vectors; and the key's file;
   - deleted without export, as temporary state: the provider client's
     queue and reservation state, the failure-pause state, and the
     fallback and catch-up state;
   - export offered, then kept or deleted: the send log, the spend
     ledger, prices read or entered, terms confirmations, consent and
     disclosure records, forecasts, trial results, caps and their
     history, and their audit events.
   The confirmation screen says text already sent stays with AWS under
   its terms and advises revoking the key. Pausing a site stops the
   provider client's work for it and keeps everything, the key included.

### Facts to check per route before a record is admitted

Checked on Dell from AWS's current official pages, dated, in the
admission record; any that changes this design is put to the owner.
Figures that vary by model are read or tested at use time.

1. How embedding models are named; whether the API reports a model's
   status (generally available or preview); whether terms treat some
   models differently, and how to recognise them.
2. Whether and how a vector size can be requested; how a model that
   cannot give 1,024 numbers answers.
3. Whether the API reports a model's window, inputs and tokens per
   request; how a too-long input is refused.
4. Whether the price can be read by a program; the billing currency;
   whether failed or timed-out requests are billed; any delayed batch
   service, its discount, turnaround and file retention.
5. Whether rate limits (requests and tokens, per minute and day) are
   reported.
6. Whether responses report tokens used.
7. Whether a tokenizer is published for local use, with a Rust library,
   licence and files, and whether it fetches files at run time; if not,
   whether a counting service exists, its cost and terms.
8. Whether inputs are used for training or to improve products, per
   account type, free tiers included, and how to turn that off.
9. How long inputs are kept and why; whether keeping nothing is
   possible; any deletion route.
10. Service locations, regional hosts, and whether a region can be fixed.
11. API host names for the allowlist; how keys authenticate; whether a
    key can be limited to embeddings; key shapes, for scrubbing.
12. Spending limits and alerts AWS offers.
13. Whether responses name the model version; retirement dates and
    notice periods.
14. Whether the same input always gives the same vector.
15. Terms on storing returned vectors and using them for search and
    ranking, and any attribution.
16. Whether returned vectors are already normalised.
17. Optional end-user id, label or metadata fields, which are never
    filled.
18. Whether a data processing agreement is offered and how to accept it.
19. Any age rules that matter for sites with young members.
20. Whether the service may refuse some content, and how it says so.

## Limits

1. Memory: the provider client, with at most 8 MB of text and vectors in flight,
   has a budget of about 64 MB, a new row in 0024's memory table inside the
   service slice, within the memory limits in AGENTS.md section 5.6. While a
   paid embedder serves, the local model (about 1,300 MB on a PC under 16 GB) is
   not loaded, leaving that room; a fallback loads it again within the same row,
   ahead of optional work such as the map projection, and the provider client
   keeps its own row, so a catch-up fits beside it. All figures are estimates,
   to be measured on Dell.
2. Disk: the paid generation's size follows 0022's formula; at 1,024
   numbers it matches the local model's and must fit 0022's index
   budget. The saved local index needs room beside it until a second paid
   activation deletes it.
3. Time: rebuild time is set by the account's rate limits and Bedrock's
   speed, forecast from the trial and measured. Live time: measured
   provider latency replaces the local model's step. After a fallback,
   the local model's re-reading takes local time, shown with progress.
4. Speed targets set from what users need: a new post's suggestion
   within 30 seconds for 99% of posts, hence the 10-second live timeout
   and 2 retries within 20 seconds; switching off or removing the key
   stops all sending and spending within 5 seconds; progress on any
   action longer than 2 seconds.
5. Money: forecast and capped by the three hard stops. No figure here is
   a price.
6. Queues and rates: live queue at most 100,000 posts; live lane share
   20% of the rate limit; pauses 1 second to 5 minutes; failure pause 1
   to 15 minutes.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

### Fakes only

- No automated test sends anything to a real provider or holds a real
  key. The runner sandbox has no network, and a guard fails any test
  that connects beyond the loopback address.
- A fake Bedrock server on loopback, with a test certificate authority,
  imitates the checked route's request and response shapes, including
  any price and limit figures, taken from AWS's public documentation
  with page and date and labelled synthetic; an unrecorded request
  fails the test. It returns deterministic vectors from a hash of each
  input, accepts any model name a test gives, and records every request.
- Its faults are programmable: rate limits with and without a wait time,
  server errors, timeouts, slow answers, malformed bodies, wrong vector
  count or size, non-finite values, refused keys, unknown and withdrawn
  models, refused inputs, missing usage or price figures, changed
  outputs, and redirects.
- Fixtures build on the committed 300-thread synthetic fixture, with
  planted email addresses, phone numbers, mentions, staff-only posts,
  censored words, quotes, code and spoilers.
- Before a record is admitted, the coordinator may, with the owner's
  approval each time, check the real API's shapes with fixed neutral
  sentences and a key the owner gives; never in the gate, never with
  site content, recording only status codes, shapes and timings.

### Unit tests (Rust, test first)

Ledger and reservation under concurrency (property test: spending never
passes a cap by Sutradhar's count); forecast arithmetic; redaction
patterns and their time limits; key scrubbing; refusal of a key as an
argument; rate limiter and pauses with a fake clock; response checks;
drift check; compatibility key; the guest-visible gate; send log and its
reduction; fallback and return decisions. Mutation tests cover the hard
rules: guest-visible only, the hard stop, the allowlist and the key never
logged.

### Acceptance tests (Rust, against the fake)

- Off by default: on a fresh install the setting is false, no provider
  client exists, no process connects out; turning it on sends nothing.
- Setup offers each step only after the one before; no site text reaches
  the fake before the trial, nor before the terms and disclosure
  confirmations, key check, self-test, price and caps are recorded; each
  step writes one audit event.
- Only checked routes can be chosen and any model name typed in: two
  names the fake accepts both pass with no list consulted; a route
  without a record cannot be chosen; a fact over 180 days old shows "not
  re-checked since <date>". Without "Manage paid embeddings", the screen
  and every command refuse alike.
- A model the fake does not know, one refusing the 1,024-token text,
  one whose counts disagree, and one in `excluded_models` each give
  PE-E17 before any site text is sent. The vector size requested is
  always 1,024; a model that cannot give it is refused; a projection over
  0022's index budget refuses activation.
- Key: the key check and self-test send only fixed neutral text; a
  refused key gives PE-E01 and is not stored. After setup, a forced error
  on the key route and a full diagnostics bundle, the key appears only in
  its secret file: in no log, audit row, error, metric, trace, progress
  message, settings history, export, process argument or environment
  variable. The secret file is `0400` in a `0700` folder, owned by the
  service account; the screen shows only date and fingerprint.
- `pin-provider` refuses a record allowing training on inputs or with
  unlimited retention; setup lists no route without a record.
- What leaves: only the provider client connects out, only to the
  allowlisted host over verified HTTPS; a redirect and a bad certificate
  are refused; the model host has no outbound network. In a rebuild of
  the fixture, no text from staff-only, members-only, private, draft or
  unapproved content, quotes, code, spoilers or signatures arrives;
  censored words arrive masked; planted emails and phones arrive as
  `[email]` and `[phone]`; mention names arrive as guests read them,
  with no member id; stored text and anchor offsets are unchanged. Every
  request holds only the allowed fields; a region field appears only
  where the record says so, and the chosen region is recorded in the
  audit event, consent record and setting, and used. An item made
  private while waiting is not sent; one made private just after its
  request left shows in the send log and the after-hidden metric.
- The disclosure names route and model, says it is not reviewed, lists
  what is and is never sent, the redaction, retention, training terms,
  locations, the fallback and catch-up, and that nothing sent can be
  recalled; its SHA-256 is stored with the consent.
- Spending: the rebuild forecast's tokens lie within 5% of tokens sent,
  and money equals tokens times the price in force (the higher of read
  and entered). With no published price, activation waits for an
  entered one; one over 180 days old gives PE-E12; a rise alerts and
  applies. While serving, an entered price alerts from 30 days before
  turning 180 days old and, unconfirmed, hands over to the local model with no
  further request; confirming starts the return; with no saved index it
  stops calls and posts wait until confirmed, each checked again. A
  rebuild budget below the forecast's upper end refuses activation.
- Property test: over 1,000 random interleavings of live, backlog,
  catch-up and trial requests, spending never passes any cap and the
  request that would pass one never starts.
- At the monthly cap with the saved index: one-step switch to the local model,
  no further request, only posts changed since the local model was last current
  wait as "waiting: spending cap reached", others continue, earlier ones
  approvable. When the month turns, the catch-up forecast shows and
  starts on its own if it fits, sends only items changed since the
  fallback while the local model serves, stops daily at the ceiling, and ends in
  one switch with no post waiting. Raising the cap starts it the same
  day if it fits; otherwise nothing starts and raising is offered. With
  no saved index, calls stop, posts wait with the reason in readiness,
  and when the month turns are read oldest first, live before backlog,
  never sending a post erased or hidden meanwhile, reported apart from
  the 30-second figure; the warning appeared before the index was
  deleted. A synthetic flood stops at the daily ceiling, falls back and
  raises the unusual-day alert; each threshold alert fires once per
  period per cap, grouped. When the fake reports more tokens, the ledger
  uses its figure and a daily gap over 2% warns. A rebuild stopped by
  its budget cancels the activation, reloads the local model, keeps the partial
  generation and shows the money spent.
- Index: pieces never pass the piece size; an input one over the
  window is refused before sending; nothing is cut. The generation record holds
  the full paid key; a search with another key is refused, and after a
  fallback or return every search carries the serving key.
- Trial: runs only after steps 1 to 7, sends only its sample, stays in
  forecast and cap, keeps suggestions running, stores numbers and ids
  and deletes its vectors; activation is refused without it.
- Activation states pause, time and cost first; no new suggestion from
  either queue during the rebuild, earlier ones approvable, "model
  update running" shown; one-step switch, paused posts matched first
  and reported apart; the local model then not resident. Chosen on a fresh
  install, the paid generation serves once its thread-level index is
  built, and the screen says no fallback exists yet. A slower
  projected model needs the warning confirmed and audited, and the
  health screen says the site runs outside the target.
- Rollback with the saved index is one step, only posts since the switch
  wait, and the fake gets no further request; during a fallback it ends
  the return. With the index deleted, the full rebuild's estimate and
  pause are shown before deletion and before the rebuild. Switching off
  or removing the key stops every call within 5 seconds and offers the
  rollback. An upgrade changing reading rules starts no paid rebuild
  until its forecast is approved. A second paid activation warns that
  the saved index will go, audits the confirmation, and later caps stop
  calls. With a community model before the paid one, a cap falls back to
  it, rollback returns to it, and the warning names it.
- A release with a changed term pauses calls until confirmed, showing
  "paused: terms changed"; a removed record stops calls with PE-E02 and
  offers rollback.
- Running: a rate-limit wait time is honoured exactly; without one,
  pauses grow from 1 second to at most 5 minutes; live posts get their
  share while the backlog is throttled; reported limits replace the
  starting rate. A burst of errors pauses calls; the probe holds one
  neutral sentence; calls resume; no item is lost; an item failing 5
  times is marked failed. A live request timing out twice leaves its
  post as "waiting: provider slow", counted in the 30-second figure with
  the cause. During a two-hour outage the health check passes, posts are
  accepted, no fallback happens; with the live queue full, PE-E16 and
  unavailable time. Bad responses are retried once and never stored. A
  refused key probes only every 10 minutes; a withdrawn model offers
  rollback. Changed outputs give "index mismatch" at the next drift
  check. The health row shows only the allowed statuses and fields, with
  "local model serving at spending cap" and ready during a fallback. Metrics
  count calls by outcome and code and tokens by lane and kind, with no
  content id, text or key in any label or span.
- Speed: the gate runs against the fake with a fixed 1-second answer
  delay and fails when 99% of new posts do not get a first suggestion
  within 30 seconds. Hot paths (request builder, response checker,
  redactor, token counting, ledger and reservation, forecast, rate
  limiter, failure pause, fallback and return, send log, switch-off)
  are benchmarked on loopback under AGENTS.md 6.8, with provider waiting
  and model time reported apart. A real site's figure is shown on its
  health screen, never gated.
- Erasure and removal: erasing a fixture post purges its paid and
  local vectors at once, and the receipt says whether, when and to
  which model its text went, with the retention and no text; a post
  erased or hidden during a fallback is purged from the paid generation
  and never sent by the catch-up. Send-log rows past retention plus 30
  days become daily totals and receipts still answer. Removal deletes
  paid vectors, trial vectors and the key file with no export, offers
  export for the send log, ledger, prices, terms, consent and trial
  records, states that sent text stays with AWS and advises revoking the
  key. Pausing a site stops the provider client's work for it and keeps
  the key; resuming restarts calls. Races between a fallback, a return
  and an erasure are tested.
- Resources: with a paid embedder serving, and after a fallback, every
  process's peak stays within its row, the provider client's within its
  64 MB; the local model is absent while the paid one serves and present after.
- A log scanner finds no post text, names or key in logs, metrics,
  traces, progress or audit events. No committed fixture holds real site
  content or a real key.

## Changes from the source

- The paid providers are replaced by AWS Bedrock with the owner's own
  key (AGENTS.md 5.5); the per-provider route split is dropped.
- The site administrator's role is taken by the owner, per site, in the
  Dioxus app, with Cedar permissions instead of site admin permissions.
- The key is entered in the app over the owner-only socket, not through
  the site add-on, so the site log checks are dropped; it is held by
  tijori.
- The provider client is a separate process of the service in the
  service's systemd slice, not its own container; network confinement uses
  systemd network rules.
- With the live queue full, the service stops taking posts from the site
  instead of refusing calls from it, since the site never calls the PC.
- Alerts appear in the app, not in site staff alerts.
- The 20x speed goal and the 5% best-result ratchet are replaced by the
  5% median rule.
- Rewritten as Rust; the local embedder runs in pure Rust.

Source: `docs/specifications/linking/paid-embeddings.md`,
`paid-embeddings-2.md`, `paid-embeddings-3.md`, `paid-embeddings-4.md`,
`paid-embeddings-5.md`, `paid-embeddings-6.md`, `paid-embeddings-7.md`
and `paid-embeddings-8.md` in the archive `archive-2026-11-08-old-repo`.
