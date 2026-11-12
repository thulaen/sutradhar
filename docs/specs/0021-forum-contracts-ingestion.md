# 0021: Signed contracts and guest-visible ingestion

Sutradhar must read a forum's posts, and only ever use what a visitor
who is not logged in could see. Rarely, when a moderator accepts a
link, it must change a post. This part is the signed message contract
between the service and the XenForo add-on, and the guest-visible
reading built on it: setu, the site link, through the modules sampark
("connection") and dwarpal ("gatekeeper"). WordPress has its own
contract on the same signing rules (see "WordPress" below). It is
roadmap Phase 4 item 1.

Example: a moderator moves a thread from a public forum into a
staff-only forum. XenForo saves the move, and the add-on's note is
saved with it. Within a second of its next poll Sutradhar reads the
note, sees that visitors can no longer see the thread, removes it from
its index, and has suggestions to and from it recalculated and retired,
with the reason in the audit log. No staff-only text was ever read, and
the thread's earlier text is purged.

## What it does

### Words used here

- **Guest-visible**: what a visitor who is not logged in may see under
  XenForo's permissions and content states.
- **Section**: one entry in XenForo's tree of forums and categories
  (a row of `xf_node`).
- **Allow-list**: the fixed list of forum tables and columns that may
  be read at all.
- **Outbox**: the add-on's table of change notes, written in the same
  database transaction as the change.
- **Signed message**: an HTTPS request or reply carrying an Ed25519
  signature, which only the holder of the private key can make and
  anyone with the public key can check.
- **Nonce**: a random single-use number, remembered for five minutes,
  so a copied message is refused the second time.
- **Request ID**: an id on every request. A request that changes
  something is applied once however often it arrives.
- **Contract version**: the version of the messages both sides speak,
  such as 1.0. The first number changes only when old messages stop
  working.
- **Oracle**: XenForo itself, asked by the add-on whether a guest can
  see an item, used to check Sutradhar's own answer.

### Scope

In scope: the signed contract in both directions (keys, signing,
replay protection, rotation, request IDs, versions and message
schemas); reading threads, posts, sections, attachment names, resources
and the forum profile, in full and by id cursor; guest visibility, the
type that carries it and its checks against XenForo; noticing changes
(outbox, permission watch, heartbeat, reconciliation) and turning them
into sanchar events; the add-on's thin parts this contract needs.

Out of scope: what reading does with the content and the text engine
(shodhan); accepting, inserting and undoing links, for which this part
gives only the messages; viewer tokens and the live channel (the
progress part); installation; visitor statistics.

### Shape

```text
  XenForo + add-on (PHP)                 Sutradhar service (Rust)
  saves a post -> outbox note  <-- signed poll    sampark reads notes
  read routes (allow-list)     <-- signed reads   decides guest view,
                                                  publishes events
  contract routes              <-- signed edits   dwarpal signs
  every reply signed           --> verified by dwarpal
```

1. The service always calls the site over HTTPS. The site never calls
   the owner's PC, so nothing is opened on the owner's home network.
   Anything the add-on has to report waits in its own tables until the
   service's next call.
2. The engine never runs on the site's server.
3. `dwarpal`'s signed-message guard: signed-message construction and
   checking, nonces and rotation. It knows nothing of XenForo and
   uses tijori for key custody and signing. Its signature checks are
   separate from the Cedar authorisation module in spec 0020:
   proving the sender never grants permission for an action.
4. `sampark`: the readers, the visibility rules, the outbox reader,
   reconciliation and the client for the add-on's routes. It depends
   on dwarpal, sanchar and the module registry. It is the shared site
   interface; the WordPress adapter has its own.
5. Traits, each with a test fake: `ForumReader` (typed reads only; fake:
   an in-memory forum built from fixtures), `AddonClient` (signed calls;
   fake: a mock HTTP server serving the add-on's routes), `NonceStore`
   (PostgreSQL; fake: a map) and `KeySource` (tijori; fake: fixed test
   keys).
6. Only sampark talks to the add-on's read routes. Other modules get
   typed readers through declared services, so no module can make its
   own forum query.

### Keys

1. Each side has its own Ed25519 key pair for signing, so a message
   signed for one direction can never be accepted in the other. Viewer
   tokens use a separate key and can never pass as a message.
2. The service's private key is held by tijori, encrypted at rest, in
   files only the service account can read.
3. The add-on's private key is encrypted with the site's own secret and
   stored in the site's database. It never appears in a setting, log
   or export.
4. Each side holds the other's public key, exchanged when the site is
   paired at installation.
5. Every key has a key id. Keys are rotated and revoked from the app.
   During a changeover (default 7 days) both sides accept the old and
   new key ids, then only the new one. A revoked key is refused at once.

### Signing a message

The signature covers these lines, each ending in a line feed:

```text
SUTRADHAR-ED25519
<contract version, such as 1.0>
<site id, made at installation>
<key id>
<method>
<path>
<query, its pairs sorted and percent-encoded>
<Unix time in seconds>
<nonce: 32 random hexadecimal characters>
<request ID>
<SHA-256 of the exact body bytes, lower-case hexadecimal>
```

They travel as headers: `X-Sutradhar-Contract`, `X-Sutradhar-Site`,
`X-Sutradhar-Key`, `X-Sutradhar-Time`, `X-Sutradhar-Nonce`,
`X-Sutradhar-Request-Id`, `X-Sutradhar-Body-SHA256` and
`X-Sutradhar-Signature`. Outbox notes carry the site id too. A reply is
signed the same way with the replier's key and also covers the request
ID it answers, so a forged answer ("the link was applied") is refused.

The receiver, before any other work:

1. refuses a message missing any header (SUT-301);
2. refuses an unknown, retired or revoked key id (SUT-302), or another
   site's id (SUT-310);
3. refuses a time more than 300 seconds old or later than its own clock
   (SUT-303); both hosts keep their clocks synchronised and the measured
   difference is shown on the health screen;
4. hashes the body exactly as received, before any decompression, and
   refuses a mismatch with its own code (SUT-304), so a proxy that
   changed the body is told apart from a forgery;
5. checks the signature with the signing library's own verify call,
   never a plain comparison, and refuses a mismatch (SUT-305);
6. records the nonce for five minutes; a nonce seen in that time is a
   replay and is refused (SUT-306). Nonces are kept in a database table
   on each side, so a restart does not forget them;
7. applies a request ID that changes something only once: the add-on
   stores each request ID with its answer for 30 days, and a repeat
   gets the stored answer and changes nothing.

Signed routes refuse compressed request bodies, have a body limit (1
MiB unless a message says otherwise), are rate-limited and are never
cached (`Cache-Control: no-store`). Keys, signatures and nonces never
appear in addresses or logs.

### Messages

Contract version 1.0. Each message is a Rust type that gives its JSON
Schema in the API description. Shared golden examples in
`contracts/forum/` are checked by the Rust tests and the add-on's PHP
tests, so neither side drifts; the Rust type is the one source. Fields
carry ids, versions, hashes, counts and codes.

Service to add-on (every one signed, every reply signed):

- **Read messages**: read threads, posts, sections, attachment names,
  resources, guests' permission rows and the forum profile, each by id
  cursor (see "Reading content").
- **`ReadOutbox`**: new notes after a cursor, with the add-on's waiting
  progress reports and its heartbeat.
- **`CheckGuestView`**: up to 200 content references; the answer is
  XenForo's own guest decision for each.
- **`ApplyLinkEdit`**: a typed instruction: post id, exact character
  range, target, and the result fingerprint Sutradhar approved, bound
  to the source version, site, actor and operation, with the source and
  target references for the guest check and the suggestion id. It
  carries no markup patch, script or replacement body. The add-on
  checks, applies it through XenForo's own post editor, and answers
  applied, refused with its code, or rolled back (SUT-416). A result
  whose hash differs is not success. Posts over 2 MB get no links and
  show "skipped: too long to edit safely" (SUT-346), so the 8 MB promise
  holds.
- **`UndoLinkEdit`**: the same shape, removing the recorded link.
- **`StaffAlert`**: an alert code, a count and a link to the admin page,
  for XenForo's staff alerts; never content.
- **`AcknowledgeEvents`**: the outbox id up to which Sutradhar has
  published events, sent every minute.
- **`ProgressSnapshot`**: progress figures for the add-on's admin pages,
  pushed by the service.

### Versions and upgrades

1. Both sides send their contract version on every message and every
   note. A different first number is refused (SUT-307); a higher second
   number is accepted and unknown fields are ignored.
2. Notes with a newer first number than the service knows are left in
   place (not acknowledged), and readiness says "the add-on is newer
   than the service: upgrade Sutradhar". Upgrades update the service
   first.

### The allow-list (XenForo 2.3)

Each table and column is confirmed on the test forum.

- Content: `xf_node`, `xf_forum`, `xf_thread`, `xf_post`,
  `xf_thread_redirect`, and the columns of `xf_attachment` and
  `xf_attachment_data` that give an attachment's content link and file
  name (not its file path or hash).
- Permissions: `xf_permission_cache_content` and
  `xf_permission_combination` (its id and user-group list), to find
  the guests' compiled permissions.
- Forum profile: `xf_bb_code` (custom tags), `xf_smilie`, and the
  add-on's `xf_sutradhar_board_profile` mirror table.
- The add-on's `xf_sutradhar_outbox`.
- `xf_addon` (id and version only), to confirm XenForo's version.
- Resource Manager's resource, update and category tables, only when it
  is installed.

Never on the list, so never readable: `xf_option` (it holds other
services' passwords and keys, so the add-on mirrors only censored words
and character, board address, friendly-address setting and route
prefixes), `xf_user`, and every user, authentication, conversation,
draft, profile post, IP, session and log table. Private conversations
cannot be read under any settings.

The add-on's read routes run only constant queries that are `SELECT`s
on the allow-list, in a read-only transaction, with no locking reads
(`FOR UPDATE`, `LOCK IN SHARE MODE`) and no table locks, so XenForo's
own writes are never blocked. Each query has a time limit (default 5
seconds). Reads use the main forum database only, never a replica.

### Checks at start-up

Before reading anything, sampark checks, and stops with a plain message
and its code if any fails:

1. Profile safety: under `test` and `dev` the site must be a test site
   on Dell's internal network (SUT-345).
2. The add-on's read routes answer only allow-listed tables (SUT-331).
3. The connection and every allow-listed text column are `utf8mb4`
   (SUT-334).
4. `xf_addon` says XenForo 2.3; another version gives SUT-335.
5. Every column the queries name exists, so an add-on that changed a
   table is caught at once (SUT-337).
6. The pairing keys load and have the right length (SUT-309).

### Calls, time limits and the breaker

1. At most `sampark.pool.max` (default 4) calls to a site at once; each
   waits at most 2 seconds for a free slot.
2. A breaker opens after 5 failures in a row, or when the median answer
   time passes 2 seconds for a minute. While open, backfill pauses, live
   reads retry every 5 seconds, and health says "forum slow or
   unreachable" (SUT-344).
3. The add-on reports its own query times in each answer; they feed the
   jobs part's `YieldSignal`, so the backlog steps aside when the forum
   is busy.

### Reading content

All reads page by id cursor, never by offset, in batches of at most 500
rows or 8 MiB of message text, whichever comes first
(`sampark.batch.rows`, `sampark.batch.bytes`).

- Threads: id, section, title, type, state, first post id, reply count,
  view count, post and last post dates, prefix, a redirect's target, and
  the thread's change counter (outbox item 6), read in the same
  statement (item 6 defines a thread's content state).
- Posts: id, thread, position, user id (an id only), state, post and
  last edit dates, edit count, the stored `message` in full, its
  SHA-256 computed by the database (`SHA2(message, 256)`), and its
  change counter (outbox item 6), all read in one statement. Every
  slice of a long message must report the same change counter; if one
  differs, the read starts again, with the same restart limit as the
  WordPress ranged fetch (SUT-347).
- Sections: the tree (id, parent, type, display order).
- Attachment names, when guests may see attachments.
- Resources, updates and categories when Resource Manager is present,
  each with its change counter read in the same statement.
- The forum profile: custom tags with their options, smilie codes and
  the mirrored options, versioned by a BLAKE3 hash, so a change makes a
  new profile version. shodhan masks censored words itself.

XenForo times are Unix seconds, read as integers. Text is compared in
Rust exactly; the database's case-blind collation is never relied on.

**Long messages, never cut.** A message may be up to 16,777,215 bytes.
One over 4 MiB is read in 1 MiB slices by character position. Every
slice answer carries the stored message's SHA-256; the joined text must
match it, or the read starts again. Nothing is ever dropped or
shortened. A message that still cannot be read whole is marked failed
(SUT-338), never read in part.

**What sampark hands on.** Readers return content only inside
`GuestVisible<T>`: a thread, a post with its message and hash, a
resource, or a list of attachment names. The forum profile is returned
as `ForumProfile` for shodhan. Deleted, hidden and private items are
returned only as `Gone { kind, id, reason }`, with no text, so callers
can purge what they hold.

### The guest rules

An item is guest-visible only if every one of these holds, using the
guests' compiled permissions from `xf_permission_cache_content`, whose
values already include what each section inherits:

- Section: guests have `view`, and the section is a forum (or, for
  resources, a resource category).
- Thread: its section passes; guests have `viewOthers` and `viewContent`
  there; `discussion_state` is `visible`; it is not a redirect (a
  redirect is followed to its target, which must pass on its own).
- Post: its thread passes and `message_state` is `visible`.
- Attachment names: the post passes and guests may view attachments in
  that section; otherwise names are not read.
- Resource, update and category: Resource Manager's equivalents; not
  read when it is absent.
- Everything else is not on the allow-list and cannot be read.

Guests' permissions for every section are held in one in-memory map per
role, refreshed when they change, so a decision is a lookup.

`GuestVisible<T>` wraps the item with the permission-map version and
the moment of the decision. Its constructor is private to
`sampark::visibility`; indexing and suggesting accept content only as
`GuestVisible<T>`, so code that tried to use anything else would not
compile. It is evidence about a moment, not a licence: before any change
to the forum, the decision is taken again.

### Checking against XenForo itself

1. **On Dell, blocking release**: the add-on's developer-only command
   (refused outside the test forum) builds fixture forums covering every
   combination of section permission, inheritance, thread and post state,
   redirect, move and merge, asks XenForo's own `canView()` as a guest
   for every item, and compares with sampark item by item. Any
   difference fails the gate.
2. **On each forum, every hour**: a signed `CheckGuestView` of 200
   random items plus one thread in every readable section. A difference
   quarantines that section (nothing from it or to it is suggested; its
   suggestions are held from review until cleared, then recalculated),
   tells the administrator in plain English and writes a fault record
   (SUT-341).
3. **At application time**: before any link is inserted, the add-on
   checks source and target with `canView()` as a guest inside the same
   transaction as the edit, besides sampark's own check (SUT-343).

### Noticing changes: the outbox

1. The add-on listens to XenForo's entity saves and deletions and,
   inside the same transaction, inserts one note in
   `xf_sutradhar_outbox` for each change that can matter:
   - posts created, edited, soft-deleted, hard-deleted, restored,
     approved, unapproved, moved or merged; threads created, edited,
     renamed, moved, merged, split, redirected, approved, unapproved,
     deleted or restored; sections created, moved or deleted;
   - permission entries changed on a section or user group, and permission
     rebuilds finishing;
   - custom tags, smilies and mirrored options changed;
   - attachments added or removed; resources and resource updates
     created, edited, approved, unapproved, moved, deleted or restored,
     and any other save of them;
   - its own heartbeat, once a minute, from a XenForo cron.
2. A note holds the outbox id, kind, content type, content id, a few
   related ids (such as a moved post's old thread), the content's change
   counter after the save (item 6), the acting member's id, whether the
   save was made through XenForo's public post, reply, edit or thread
   forms, or Resource Manager's add, update or edit forms, by the
   content's own author, the time and the contract version. Never text.
   A rolled-back save rolls its note back.
3. sampark polls for new notes every 250 ms while busy (1 second when
   idle). For each it publishes a sanchar event and saves its cursor in
   the same transaction as the events, so each note becomes events
   exactly once.
4. A save that took an id earlier may commit later. sampark remembers
   each missing id below its newest one for 120 seconds and asks again;
   after that the id is taken as a rolled-back save. Reconciliation
   catches anything missed.
5. The add-on deletes notes at or below the acknowledged cursor once
   they are an hour old. If Sutradhar is away, notes are kept up to
   `sampark.outbox.max_rows` (default 5 million, about 1 GB); beyond
   that the add-on stops adding notes and records a gap (SUT-339), and
   Sutradhar runs a full reconciliation when it returns.
6. The add-on keeps a change counter per content item in
   `xf_sutradhar_change`, raised by one in the save transaction on every
   change to its body, title, links, status or visibility, whatever
   XenForo's own edit count does. The same row holds the shown-before
   flag: set, and never cleared, in the save transaction whenever the
   content's own XenForo state is visible before or after a save,
   visible meaning approved and not deleted, to any reader, whatever the
   section permissions. A row is written clear when an item is first
   created after install; at install the flag is set for every existing
   item, and an existing item with no row counts as set. sampark keeps a
   per-site delivery sequence counter and its epoch in
   `forum_delivery_counter`. The cursor transaction of item 3 gives each
   note the next number as it publishes the note's sanchar event. These
   transactions run one at a time per site, so the number is strictly
   increasing in commit order. A content state is a change counter
   together with the SHA-256 of the content's text: the message for a
   XenForo post or resource update, the title and body for a WordPress
   post (the SHA-256 of the title's UTF-8 byte length as an 8-byte
   big-endian integer, then the title's UTF-8 bytes, then the body's,
   computed the same way by the plugin and by sampark across parts), the
   description for a resource, and the title for a thread. The number
   belongs to the first content state recorded with the change counter
   the note holds (item 2). A delivery carries the number only when all
   of these hold: its handler reads that state; the note's kind is a
   post, resource or resource update created, edited or approved, or a
   thread created (item 1), a creation or approval counting only while
   the shown-before flag was clear before the save, never a move, merge,
   split, restore or any other kind; and its outbox id is above every id
   already numbered. So a note found only on asking again (item 4)
   carries no number. A delivery that coalesces notes of different kinds
   carries a number only when every one of them may. Any other delivery,
   and every delivery with no note (backfill, reconciliation or the
   permission watch), carries no number. The number, the site's counter
   epoch, the note's kind (item 1) and the content's author id are
   carried in the delivery event: the XenForo member id only when the
   note's save was made through XenForo's public post, reply, edit or
   thread forms, or Resource Manager's add, update or edit forms, by
   that member, and 0 for a guest, the feed reader, an import, the admin
   control panel, cron or a save by another member, as the note records.
   The author id is the acting member's id that the note recorded at
   save time, never the content's current user id. A delivery carries a
   non-zero author id only when its handler reads exactly the content
   state recorded with its note and that note records an own-author
   save, and a delivery that coalesces notes carries it only when every
   one of them records an own-author save by that same member; every
   other delivery, and every delivery with no note, carries 0. The
   public API read carries the same author id, and the public API
   returns the current counter value with its epoch, so readers can
   order deliveries without relying on outbox ids, event ids or handling
   order (spec 0072). The number is never taken from a host clock or a
   date the author sets. `forum_delivery_counter` is never reset with
   the index; if it is lost, a new row is created with a fresh random
   epoch, and sampark publishes the change in a typed event,
   `sampark.epoch_started`, in the same transaction.

### Noticing changes: permissions, reconciliation, heartbeat

1. Every 60 seconds sampark recomputes a BLAKE3 digest of the guests'
   permission rows. A section whose digest changed raises
   `visibility.nodes_changed`, even with no note (for example after a
   database restore). Sections that lost guest access leave serving at
   once and are purged; sections that gained it are queued for reading.
2. Every 5 minutes, threads, posts, resources and resource updates made
   or edited in the last two hours are compared with what Sutradhar
   recorded (ids, states, section, thread, position, edit count, change
   counter, last edit date).
3. Every 24 hours (`reading.reconcile.interval_hours`) the same runs
   over every one of them, in the backfill lane; a weekly pass also
   compares each message's SHA-256. Differences become ordinary events.
4. No heartbeat for 5 minutes: health says "the forum add-on is not
   reporting; changes may be missed" (SUT-340), an alert is raised, and
   a reconciliation runs when it returns.

### Handling each event

The handler reads the current truth through sampark, then acts. If the
item is `Gone`, it is purged everywhere (stored text, vectors, spans
and lab copies, by their owners' jobs), and suggestions to and from it
are recalculated at once and retired with the reason in the audit log;
a suggestion is never just left to expire. If it is guest-visible, it
is passed on with its current hash; an unchanged hash means nothing to
do. Deletions and visibility losses run first (priority 0).

### The add-on's part (PHP 8.5)

Kept thin, with no runtime packages beyond XenForo, in XenForo's add-on
structure, and able to run on shared hosting:

- entity listeners that insert outbox notes;
- the read and contract routes, registered in XenForo's own API and
  called with a XenForo API key for a system user holding only the
  rights they need, each checking Sutradhar's signature first;
- a signer and checker using PHP's built-in sodium functions, with the
  shared test vectors;
- crons: heartbeat, pruning acknowledged notes, refreshing mirrored
  options, pruning nonces and old request IDs;
- the developer-only conformance command;
- its tables, made and removed by XenForo's own installer.

Every edit goes through XenForo's post editor with hash checks. By
default the add-on only wraps existing text in a link and never inserts
anyone's text; the member-text option, on from the start, lets a
moderator approve each such change, with undo and recheck as usual.

### Data

In the forum's database, owned by the add-on (times as Unix integers):

- `xf_sutradhar_outbox`: `outbox_id`, `kind`, `content_type`,
  `content_id`, `related` (JSON of ids), `change_counter`,
  `acting_user_id`, `own_author_save`, `created_date`, `contract`.
- `xf_sutradhar_change`: `content_type`, `content_id`,
  `change_counter` and `shown_before` (outbox item 6).
- `xf_sutradhar_board_profile`: `option_key`, `option_value`,
  `updated_date`, for the mirrored options only.
- `xf_sutradhar_nonce`: `key_id`, `nonce`, `expires_date`.
- `xf_sutradhar_applied`: `request_id`, `post_id`, `result` (JSON),
  `applied_date`.
- `xf_sutradhar_ack`: one row, the acknowledged outbox id.
- `xf_sutradhar_key`: the add-on's encrypted private key, its key id,
  and the service's public keys.

In the WordPress database, owned by the plugin, under the site's table
prefix, written only by its save hook; its event rows are pruned as
the "WordPress" section says:

- `sutradhar_change`: `post_id`, `change_counter`, `shown_before`.
- `sutradhar_event`: `event_id`, `kind`, `post_id`, `change_counter`,
  `acting_user_id`, `own_author_save`, `saved_sha256`, `created_date`.

In Sutradhar's PostgreSQL, schemas `sampark` and `dwarpal`, every row
carrying its site:

- `forum_cursor`: the outbox cursor, the missing ids with first-seen
  times, the last heartbeat seen.
- `forum_delivery_counter`: per site, the counter epoch and the delivery
  sequence counter (outbox item 6). The epoch is a fresh random value
  written when the row is first created, so a missing row means a new
  epoch.
- `forum_item_state`: per thread, post, resource and resource update:
  ids, states, section, thread, position, edit count, change counter,
  last edit date, text SHA-256 (outbox item 6) and whether it was
  guest-visible, as last seen. Never text.
- `forum_permission_digest`: per section, the digest and when it changed.
- `forum_quarantine`: quarantined sections with code and time, cleared by
  an administrator after the cause is fixed.
- `signed_nonce`: `key_id`, `nonce`, `expires_at`.

RLS is on and forced on each: `forum_cursor`, `forum_item_state`,
`forum_delivery_counter` and `forum_permission_digest` are written by
the worker role only and read by the serve role for health;
`forum_quarantine` is inserted by the worker and cleared only by serve
after the action layer's check; `signed_nonce` rows are inserted by the
receiving role, never updated, and deleted after expiry.

Uninstall: the add-on's tables go with it; Sutradhar's forum tables are
derived and go with the index, except `forum_delivery_counter` (outbox
item 6); published links stay.

### Failure handling

dwarpal uses SUT-300 to SUT-329, sampark SUT-330 to SUT-399:

| Code | Meaning |
| --- | --- |
| SUT-301 to SUT-306 | The signature checks above, in order |
| SUT-307 | Contract versions differ in their first number |
| SUT-308 | A signed reply failed its checks |
| SUT-309 | A key is missing, unreadable or the wrong length |
| SUT-310 | The site id is not this site's |
| SUT-331 | A read route reaches beyond the allow-list |
| SUT-332 | The site is unreachable |
| SUT-333 | A forum query passed its time limit |
| SUT-334 | A table or the connection is not `utf8mb4` |
| SUT-335 | The forum is not XenForo 2.3 |
| SUT-337 | A column the queries need is missing |
| SUT-338 | A long message could not be read whole |
| SUT-339 | The outbox has a gap; a full reconciliation runs |
| SUT-340 | No heartbeat from the add-on for 5 minutes |
| SUT-341 | Guest check disagreed with XenForo; section quarantined |
| SUT-342 | Edit refused: the post changed since the suggestion |
| SUT-343 | Guest check refused source or target at apply time |
| SUT-344 | The breaker on the site is open |
| SUT-345 | A test profile pointed at a site that is not a test site |
| SUT-346 | A post over 2 MB is skipped for link insertion |
| SUT-347 | A WordPress post's part or whole checksum failed, or a WordPress part or XenForo slice reported a different change counter; read restarted |

- Site slow or unreachable: the breaker opens; backfill pauses; live
  reads retry with backoff; notes wait in the outbox; suggestions made
  stay as they are; nothing is guessed.
- Edits wait and are retried with the same request ID; the review page
  says "waiting for the forum"; nothing is applied twice.
- Edit refused because the post changed: the suggestion is recalculated
  (at 30 days, or at once when content or visibility makes it stale)
  and kept, replaced or retired with the reason; never forced.
- Event storm (the spam cleaner deleting thousands of posts): notes read
  in batches of 1,000, coalesced per item, deletions first.
- Clock difference over 300 seconds: signed calls fail with SUT-303,
  health shows the difference and an alert asks for clock sync.

### Edge cases

- Posts moved, threads split, a first post that becomes a reply after a
  merge: handled from the note by reading the current thread, position
  and first-post id; affected suggestions are recalculated.
- Thread undeleted, section deleted with threads moved, thread moved to a
  section with other guest access: the note, then the current truth,
  decides.
- A banned member's threads stay guest-visible if XenForo shows them.
- A member deleted, renamed or merged: posts keep only the user id.
- Prefixes such as "Closed" are not a visibility rule.
- Censored words are masked by shodhan; never stored in clear.
- A XenForo 2.2 forum is refused (SUT-335).
- Another add-on adding columns is harmless; one removing a needed
  column gives SUT-337.
- A forum on MySQL rather than MariaDB is allowed only once queries and
  time limits are shown to behave the same; until then the installer
  says it is not supported.
- A note naming a missing post: `Gone`, purged.
- A clock jump on either host: signatures fail until clocks agree;
  ordering never uses either host's clock.
- A forum with no guest access: health says so in plain English rather
  than reporting an empty index as healthy.

### Security and privacy

- Only guest-visible content enters; nothing off the allow-list can be
  read. Members appear as ids only. Mentions and quote attributions are
  never-anchor text for shodhan.
- No live credentials in dev, gate or test containers; tests refuse
  live settings before connecting.
- Visibility loss, deletion and erasure purge at once; the state row
  keeps ids, states and the hash only.
- Notes, events, logs, metrics and spans carry ids, counts, codes and
  durations only.
- Public material uses made-up forums only.
- The add-on's routes accept only signed requests, refuse browsers'
  signature headers and are rate-limited.

### WordPress

The WordPress plugin uses its own versioned contract on the same signing
rules and pairing. It has its own visibility facts, native capabilities
and content validator; neither adapter inherits the other's visibility
or save rules. Its events, pause and conditional writes are tested on
their own. Its signed plugin facts include one boolean, whether Akismet
is installed and active, read once at the start of each kavach run (spec
0072). The plugin keeps the same per-content change counter as outbox
item 6 in its own table, `sutradhar_change` (see Data), which no role,
REST route or custom field can write. Only the plugin's save hook raises
it, in the plugin's own database transaction together with the change
event row, on every change to a post's body, title, links, status or
visibility. Change events are rows of `sutradhar_event` (see Data),
which no role, REST route or custom field can write either. The plugin
prunes event rows, stops at the same limit and records a gap exactly as
outbox item 5 does for notes. The plugin's reads return the change
counter in the same statement as the content, with the same counter for
every part, as the Posts read under "Reading content" requires. sampark
numbers those events from the same per-site delivery sequence counter,
in its one-at-a-time transaction that publishes them, exactly as item 6
describes. Its change events carry kinds like a XenForo note's (item 1):
created, published and edited, which are numbered as item 6 describes.
The plugin keeps the same shown-before flag in the same table, set by
its save hook, and never cleared, whenever a post is published, private
or password-protected before or after a save, and otherwise written and
set as item 6 describes. Only a post's publication while the flag was
clear before the save, such as a new post published in its first save or
a draft, scheduled or pending post going live for the first time, is the
numbered kind published. Every other change that makes a post
guest-visible, such as removing a password, making a private post public
or publishing again, is a kind of its own, made public, even when the
same save changes other things, and such a delivery carries no number.
Each change event also carries whether the save came through WordPress's
own editing paths for that post (the classic editor's post.php, the
block editor's REST request for that post under a logged-in session, or
quick edit) by a user who is the post's author and holds the publish
capability, read in the save hook; it is false for every other save,
whoever the acting user is, such as one by another user, by cron, by a
feed importer, through a form, or through REST or XML-RPC with an
application password. sampark carries the content's author id in the
delivery event and its public API read: the acting user's id that the
change event row recorded in the save hook, never the post's current
author, when the event records that fact as true, under the same rules
as item 6 for reading the event's own state and for coalesced
deliveries, and only when sampark's read has the SHA-256 that the hook
recorded of the text the acting request itself saved; 0 otherwise.

**`ReadRange`, the WordPress ranged fetch.** A WordPress post may be
longer than 16 MiB. The service reads it in parts of at most 16 MiB,
each fetched in slices of at most 1 MiB (spec 0022). Nothing is cut.

- It is a signed message under "Signing a message": Ed25519, the Unix
  time, a single-use nonce and a request ID, all checked in the same
  order with the same codes. The reply is signed with the plugin's key
  and covers the request ID it answers. A read changes nothing, so a
  repeated request ID simply gets a fresh, signed answer.
- Request: post id, content version, part number (from 0), and the
  slice's byte start and length within the post. A part is a fixed
  byte range: part n starts at n × 16 MiB, moved back to a character
  boundary.
- Reply: post id, content version, part number, the slice's actual start
  and length (the plugin ends a slice only between characters, so it may
  be a few bytes short), the slice bytes in base64, and three SHA-256
  checksums: of the slice, of the whole part, and of the whole post,
  plus the post's length in bytes and its change counter. Every slice of
  a version carries the same part and whole checksums. The plugin works
  out the part and whole checksums once per content version, reading the
  post from the database in 1 MiB chunks, and keeps them with that
  version until the post changes. It never holds the whole post in
  memory, so PHP's 8 MB limit holds for any size of post.
- Checks in the service: each slice's checksum on arrival; the part's
  checksum once its slices are joined, before the part is parsed; the
  whole post's checksum over all parts, before any piece is served; and
  that every part reports the same change counter. A failed check, or a
  content version that changes, restarts the read from part 0 (SUT-347).
  After three restarts the post is marked failed (SUT-338), never read
  in part.
- Limits: a slice at most 1 MiB of text; the reply body at most
  1.5 MiB, the stated exception to the 1 MiB body limit, to hold the
  base64 slice; a part at most 16 MiB; one post read in parts at a
  time per role; the plugin's extra memory at most 8 MB per request;
  the same rate limit as other signed routes; never cached.

### Observability

Labels: `query` (one per constant query, about 25), `direction`,
`outcome`, `reason`; never ids. Metrics:
`sutradhar_forum_query_seconds{query}`,
`sutradhar_forum_rows_read_total{query}`,
`sutradhar_forum_outbox_lag_seconds`,
`sutradhar_forum_outbox_missing_ids`, `sutradhar_forum_breaker_open`,
`sutradhar_forum_calls_in_use`,
`sutradhar_forum_guest_check_mismatches_total`,
`sutradhar_forum_quarantined_nodes`,
`sutradhar_forum_addon_heartbeat_age_seconds`,
`sutradhar_forum_clock_offset_seconds`,
`sutradhar_signed_requests_total{direction,outcome}`,
`sutradhar_signature_rejections_total{reason}` (CAP-072).

Traces: `sampark.read_batch`, `sampark.outbox_poll`,
`sampark.reconcile`, `dwarpal.sign`, `dwarpal.verify`; a `traceparent`
header goes to the add-on, so one trace follows a link from approval to
XenForo's save. Logs: `forum_connected`, `forum_grants_refused`,
`outbox_gap`, `guest_check_mismatch`, `signature_rejected`,
`addon_silent`. Alerts: outbox lag over 30 s for 5 minutes; any
quarantined section; over 20 rejected signatures in 5 minutes; no
heartbeat; the breaker open over 5 minutes.

Capabilities: CAP-072 and CAP-073, plus new ids for the outbox and the
visibility check.

## Limits

- Speed targets on Dell, each also under the 5% median rule:
  - XenForo committing a post to its sanchar event: at most 1 s (p99);
  - at least 2,000 guest-visible posts a second read from the test
    forum at the default batch, while XenForo's own page time rises by
    no more than 10%;
  - one guest decision at most 2 µs median; rebuilding the map for
    10,000 sections at most 200 ms;
  - signing or checking a 64 KiB body at most 100 µs median, plus at
    most 5 ms (p99) for the nonce write;
  - the application-time guest check at most 50 ms (p99);
  - full reconciliation of a million posts within 15 minutes of
    database time, in the backfill lane.
- Service memory: permission map for 10,000 sections at most 4 MiB; one
  batch of text at most 8 MiB; one long message being joined at most
  32 MiB, one at a time per role; outbox reader and cursors 2 MiB;
  nonce front cache 4 MiB. In total at most 48 MiB in the worker and
  inference roles and 8 MiB in serve, inside the service slice, within the
  memory limits in AGENTS.md section 5.6.
- On the site: at most 8 MB extra memory per request; posts over 2 MB
  get no links; outbox at most 5 million rows (about 1 GB).
- Messages: 1 MiB body limit unless a message says otherwise; 300-second
  window; nonces kept 5 minutes; request IDs kept 30 days.

## Footprint budget

On public pages the add-on adds only the outbox note on a save:

| Measure | Budget |
| --- | --- |
| Extra queries per post save | one outbox insert per change |
| Extra time per post save | at most 1 ms (p99) |
| Extra bytes written per post save | at most 64 KB |
| Extra memory per request | at most 8 MB |
| Extra queries, time or bytes on page views | none |

Forum pages load as they do without the add-on, within 8 MB, when the
service is unreachable, slow or wrong.

## How it is tested

Rust tests in the dwarpal and sampark crates, failure and load tests in
the workspace tests, PHP tests in the add-on's tests folder, and SQL
tests for policies. "Test forum" means Dell's XenForo 2.3 test site with
made-up content. Coverage is 100% of lines, branches and MC/DC in Rust;
the guest rules, signature checks and allow-list check need every mutant
killed. The PHP checker and outbox code are covered by PHP tests, with
PHPStan at its strictest and a taint check.

Signing:

1. Shared vectors in `contracts/forum/signing-vectors.json`: Rust and
   PHP both produce and accept exactly the stored signatures.
2. Property: changing any byte of method, path, query, a signed header,
   the request ID or the body after signing makes checking fail with
   the matching code.
3. A message accepted once is refused with SUT-306 when the same bytes
   arrive within 300 s, even after a restart.
4. Messages 299 and 301 s old: the first passes, the second gets
   SUT-303; one dated in the future gets SUT-303.
5. A body re-compressed by a proxy gives SUT-304, not SUT-305.
6. A source scan shows signatures checked only through the library's
   verify call, in Rust and PHP.
7. Key ids k1 and k2 both pass during a changeover; after it, k1 gets
   SUT-302; a revoked key is refused at once.
8. A message signed with the add-on's key, or a viewer token, sent as a
   service message is refused.
9. A reply whose request ID does not match the request is refused
   (SUT-308).
10. A change sent twice with one request ID edits once; the second
    answer equals the first (PHP).

Reading:

11. Every constant query is a `SELECT` naming only allow-listed tables
    (Rust and PHP scans).
12. A read route that reaches `xf_user` or a write stops start-up with
    SUT-331.
13. A forced `INSERT` through a read route is refused by the read-only
    transaction.
14. Release-blocking conformance on the test forum: sampark matches
    XenForo's `canView()` as a guest for every fixture item.
15. The fake add-on answering "not visible" for one item quarantines
    that section with SUT-341, holds its suggestions, raises an alert and
    leaves other sections alone.
16. A `compile_fail` test: code outside `sampark::visibility` cannot
    build a `GuestVisible<Post>`.
17. Property: an item is visible only if section `view`, `viewOthers`,
    `viewContent`, thread state and post state all allow it.
18. Guests who can see a forum but not its threads get no thread.
19. A 16,777,000-byte post with 4-byte characters across slice edges
    is read byte for byte with a matching SHA-256.
20. A `latin1` post table stops start-up with SUT-334.
21. A moved thread's redirect and a merged thread resolve to the real
    thread, or `Gone` if it is not guest-visible.

Noticing changes:

22. A rolled-back post save leaves no note; a committed one exactly one
    (PHP, test forum).
23. Note 102 committing 30 s after 103 is published when it appears;
    104 never appearing is taken as rolled back after 120 s.
24. Model check with loom: no note is acknowledged before its events
    are committed.
25. Guests losing `view` by a direct change with no note raise
    `visibility.nodes_changed` within 60 s; threads leave serving and
    text is purged.
26. A thread moved to a staff-only section is `Gone`; its suggestions are
    recalculated and retired with the reason.
27. A hard-deleted post leaves the index at once; its suggestions are
    recalculated at once and retired with the reason.
28. An edit whose note was lost is found by the 5-minute pass.
29. No heartbeat for 5 minutes: health shows SUT-340 and an alert.
30. Notes acknowledged up to 500, then Sutradhar away for days: nothing
    above 500 is pruned, and the outbox survives a PHP restart. The
    same holds for WordPress change events in `sutradhar_event`.

Edits, versions and limits:

31. A target made staff-only after the suggestion: the add-on's check
    refuses with SUT-343; the post is unchanged (test forum).
32. A note with contract 2.0 is not acknowledged and readiness asks for
    an upgrade; 1.3 is handled with unknown fields ignored.
33. No free text or member name in any message schema, note, event,
    log, metric or span of a captured run.
34. Each route and a post save, add-on on and off: extra peak memory at
    most 8 MB and extra save time at most 1 ms (p99) (footprint test).
35. Failure test (nightly): the test site stopped for two minutes; the
    breaker opens and closes, no note is lost, reading resumes.
36. Load (at most 2 minutes at the push tier): 50 saves a second; each
    event exists within 1 s (p99).
37. Kani proofs: the guest rule answers visible only if every condition
    holds; the signing text never gives the same bytes for two different
    messages.
38. SQL test: the serve role cannot change `forum_item_state` or the
    cursor; every table has RLS on and forced.
39. An accepted suggestion in a 2.5 MB post sends nothing to the add-on
    and shows SUT-346; the post is unchanged.
40. Fuzzing of the Rust and PHP checkers with malformed headers and
    bodies.
41. Benchmarks `bench_guest_decision`, `bench_permission_map` and
    `bench_sign_verify`; load tests for backlog reading, the apply
    check and nightly reconciliation hold their targets.
42. `ReadRange` on the WordPress test site: a 40 MiB post comes in
    three parts of 1 MiB slices; each reply is signed and covers the
    request ID; a reply whose request ID differs is refused. A slice,
    part or whole checksum made wrong in the test gives SUT-347 and a
    restart from part 0, with nothing served; three restarts give
    SUT-338. A stale time, a reused nonce and a bad signature are
    refused with their usual codes. No reply body passes 1.5 MiB. A
    part that reports a different change counter from part 0 gives
    SUT-347 and a restart from part 0.
43. Change counter and order (PHP, test forum): a save raises the
    item's change counter by one in its transaction and a rolled-back
    save leaves it unchanged; a XenForo read whose slices report
    different counters restarts with SUT-347; deliveries carry
    strictly increasing numbers in commit order; a note found only on
    asking again, a move, and a coalesced edit and move carry none.
44. Shown-before flag: a new post's first creation or approval is
    numbered; after it is unapproved and approved again, or for any
    item that existed at install or has no row, the approval carries
    no number; on WordPress, removing a password, making a private
    post public and publishing again after a draft are the unnumbered
    kind made public.
45. `forum_delivery_counter` survives an index reset and an
    uninstall; when its row is lost, a new row with a fresh random
    epoch and `sampark.epoch_started` are written in one transaction.
46. SQL and PHP tests: no role, REST route or custom field can write the
    WordPress `sutradhar_change` and `sutradhar_event` tables, and only
    the worker role can write `forum_delivery_counter`.
47. A WordPress post saved by its publish-capable author in the
    editor reaches kavach with its user id; one saved by another user,
    by cron, by a feed importer (including a "fetch now" from wp-admin),
    through a form, or through REST with the admin's application
    password reaches it with author id 0; when the author's save is
    followed by another user's save before it is delivered, the
    delivery carries author id 0, including when the other save writes
    its body before the author's hook runs; a post with a non-ASCII
    title read in several parts gives the same hash in PHP and Rust
    (PHP and Rust).
48. On XenForo, a post made by the feed reader, by an import run from
    the admin control panel or by cron, and a revision saved by a
    member other than the post's author, reach kavach with author id 0;
    a post its author saves through the public post form, and a resource
    update its author adds through Resource Manager's form, reach it
    with the member id; a delivery that coalesces the author's note with
    another member's edit, or whose handler reads a later save by
    another member, carries id 0; a user merge or a staff reassignment
    of the author between the save and its delivery leaves the recorded
    id, never the new one (PHP, test forum).

Release tier for every add-on release: fault injection, defect and
security checks, resource, memory and storage checks, profiling, load
and failure tests, including tests 10, 22, 30, 31, 34 and 39; forum
pages loading as without the add-on, within 8 MB, when the service is
unreachable, slow or wrong; and the add-on upgraded before or after the
service with pages working throughout and mismatches shown. A release
that fails any of them is not released.

Facts to check on the XenForo 2.3 test forum: the exact allow-list
columns; the guests' permission combination and how to find it; the
format of `cache_value` and that it includes inherited permissions;
the attachment permission for guests; `view`, `viewOthers` and
`viewContent` in 2.3; how moves and merges leave redirects; Resource
Manager's tables and permissions; the `xf_` prefix; listeners running
inside the save's transaction, and the event that ends a permission
rebuild; add-on routes in XenForo's API with a narrow key; query time
limits in read-only transactions on MariaDB and MySQL; that ids are
never reused after a rolled-back insert and commit order may differ;
the sodium functions behaving the same on PHP 8.5; Kani proving the
guest rule and signing text within the gate's time.

## Changes from the source

- Signatures are Ed25519 with a key pair per side, not a shared secret
  with derived keys, as AGENTS.md 5.4 requires.
- Every message carries a request ID; a reply also signs the request ID
  it answers; a changing request ID is applied once.
- A message dated in the future is refused, not only one 300 s away.
- The service's keys are held by tijori; the add-on's key is encrypted
  with the site's secret in the site's database; keys rotate and are
  revoked from the app.
- The add-on hands over posts through signed read routes over HTTPS,
  instead of a direct read-only database login from the service; the
  grant and pool checks become route and call-limit checks.
- Add-on-to-service messages become replies to the service's polls or
  messages pushed by the service, since the site never calls the
  owner's PC.
- Long messages are checked by hash across slices instead of one
  database snapshot.
- Edits carry the typed instruction of the plugin coexistence decision,
  not tag text.
- The add-on is GPL-2.0-or-later, as AGENTS.md 5.7 sets.
- Disagreements and overdue checks write fault records instead of
  filing issues by themselves.
- Speed follows the 5% median rule and the 20x goal is kept (0007); old per
  container memory estimates are replaced by the service slice.

Source: `docs/specifications/platform/forum.md`, `platform/forum-2.md`,
`platform/forum-3.md`, `platform/forum-4.md`, `platform/forum-5.md`,
`platform/forum-6.md` and `platform/forum-7.md` in the archive
`archive-2026-11-08-old-repo`.
