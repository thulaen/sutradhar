# 0065: tijori's signing core

tijori ("strongbox") holds keys and signs. This spec covers its signing
core: the keys that sign review records, waivers, signed runs, release
records, the site link's messages and the other records in the table
below. Callers ask the core to sign and get back a signature or a
refusal in plain English; no caller reads a key the core holds, except
that the service runs under the same account as its instance (AGENTS.md
section 5.4), so it can read the site-link key. The
forum add-on's own key is not the core's: spec 0021 keeps it in the
site's database.

Roadmap Phase 2 item 8 builds the signing core before Sutradhar Boot
exists, and Phase 3 item 8 moves it onto Boot. It is the one piece of
product code allowed before the gate is complete
([AGENTS.md section 7.3](../../AGENTS.md#73-bootstrap)). It is Rust,
like the rest of the product.

Which roadmap item builds each part:

- Phase 2 item 8: signing and checking, the log, the signer registry,
  the owner's instance, the relay's log and policy carrying, and the
  owner tool, for review records, waivers, signed runs, receipts, cache
  entries and the site link.
- Phase 3 item 8: the move onto Sutradhar Boot.
- Phase 5 item 1: `sign_release` for the `release-manifest` purpose
  (spec 0030), built and tested with test release keys only. Until
  Phase 7 builds the ledger guard and the GitHub copy, `sign_release`
  refuses every request under the real release key.
- Phase 5 item 5: the owner tool's commands become `sutra keys`.
- Phase 7 items 1 and 3: the `release-record` purpose, the ledger guard,
  the relay's GitHub copy, the fetcher, and the `QualifyTrust` source of
  the trust trait (specs 0055 and 0064).

Other keys stay with the specs that own them until the rest of tijori
(Phase 5 item 4) brings them under these rules: hisaab's sealing key
(spec 0035), stored secrets (specs 0025, 0038 and 0040) and the audit
key (spec 0036).

## What it does

### Who uses it

| Used for | What the core signs | Spec |
|---|---|---|
| Review records and waivers | A review record's fingerprint, the required review it is for and its verdict; an owner waiver | AGENTS.md 7.1 and 7.2 |
| The qualification verifier | Runner results, group records, run manifests, ledgers | 0064 |
| Release coordination | Release records | 0005 |
| GanitEvolve | Receipts | 0012, 0013 |
| Agent products | Research-card acceptances | 0013 |
| The gate | Cache entries; the signed full run and phase check | 0010 |
| The runner supervisor (spec 0008's runner) | Evidence bundles; review results | 0003, 0008 |
| Install and upgrade | Release manifests | 0030 |
| The site link | The service's side of messages | 0021 |

### Instances and crates

1. **Dell's instance** holds the keys of the principals on Dell (see
   "Callers and their keys"), including the release key, which belongs
   to the release step: only the release step's account may call
   `sign_release`.
2. **The owner's instance**, on the owner's PC, holds the owner's key
   and a separate head-confirmation key. It signs with the owner's key
   (waivers, policy changes, owner approvals) only after it shows the
   owner, on that PC, what is being signed in plain English: every field
   of every owner-signed purpose, and for a policy change each field
   that differs from the last policy it signed for that instance (such
   as keys, account bindings, purposes, scopes and limits) is marked.
   The screen is built from the exact bytes being signed; control and
   direction-changing characters are shown escaped, any character
   outside ASCII is flagged, key ids are shown in full, and a field over
   its length is refused. Site-key changes use the same screen. The
   owner approves there; a socket request alone never makes an owner
   signature. The head-confirmation key signs only `head-confirmation`,
   never through `sign()`, and only over the instance's own checked copy
   of Dell's log (see "The relay"). Dell checks owner signatures but
   never holds the owner's key, whose public key is pinned in
   `gate/owner-key.pub` ([AGENTS.md section
   7.2](../../AGENTS.md#72-waivers)).
3. **The service's instance**, beside the product on the owner's PC,
   runs under the service account and holds the service's site-link
   key in files only that account can read
   ([AGENTS.md section 5.4](../../AGENTS.md#54-the-site-link)).
4. Each instance is the `tijori` program, with its own keys, signature
   log and copy of the policy. The `sutradhar-tijori` crate (a core
   crate in spec 0015) holds the client, the check library and the one
   policy trust source, with no private-key code, so product crates
   never link key-handling code.

### Signing purposes

1. Every signature is made for one named purpose, such as
   `review-record`, `waiver`, `policy-change`, `head-confirmation`,
   `runner-result`, `evidence-bundle`, `review-result`, `group-record`,
   `run-manifest`, `release-ledger`, `pilot-ledger`, `fixture-ledger`,
   `release-record`, `release-manifest`, `cache-entry`, `gate-run`,
   `receipt`, `log-head`, `policy-record-reset`,
   `site-key-record-reset`, `recovery`, `card-acceptance`,
   `site-key-change` or `site-message`. Each instance's protected policy
   lists its purposes. A purpose name uses only lower-case ASCII
   letters, digits and hyphens, within a length the policy sets.
   Principal and account names follow the same rule, so no look-alike
   letter from another script can pass as a known name.
2. For every purpose but the site link's, the core writes the purpose
   field that spec 0064 puts first in every signed encoding, exactly as
   spec 0064 encodes it. The caller's bytes start at the next field, the
   record type. Anything else a caller needs to tell apart, such as
   which site or which run, is a field of its own encoding.
3. `site-message` uses spec 0021's own signed text instead, so the
   add-on keeps checking with its own library. The core adds no prefix
   to it and refuses `site-message` bytes whose first line is not exactly
   spec 0021's `SUTRADHAR-ED25519` followed by a line feed.
4. A signature made for one purpose is never accepted for another, and a
   key is refused for any purpose its registry entry does not list.
5. Each purpose's encoding is written in the spec of the caller that
   owns it. The core signs exactly those bytes; it never re-encodes
   them.
6. The policy sets the largest whole request and, for each purpose, the
   largest bytes it accepts. The request's length is read first; a
   larger request is refused before the rest of it is read.

### Records this spec owns

The signed encodings below are this spec's; every other purpose's
encoding belongs to the caller's spec, and spec 0010 owns the
`review-record`, `waiver` and `gate-run` encodings. Each starts with
spec 0064's purpose field. A `review-record` names the change's
fingerprint, the required review it is for (AGENTS.md section 7.1) and
its verdict, and the gate refuses one whose review is not the one being
counted.

- The policy and a `policy-change`: the instance it is for, its
  version, the previous policy's digest, every principal with its
  accounts, keys, purposes, scopes and limits, for each key it revokes
  the last confirmed Dell head (counter and hash), and every
  policy-wide setting: the purpose-name, principal-name, request and
  per-purpose size limits, the longest wait and the waiting count, the
  `log_since` page size, the clock bound, the head-confirmation wait,
  the release count and window, and the GitHub repository and path for
  release-key entries.
- A `log-head`: the instance, the counter, the head hash and the time.
  Dell's instance signs it with its own instance key, which signs
  nothing else; that public key and the head-confirmation public key are
  entries in the owner-signed registry, so neither changes without a
  `policy-change`. The `log-head` signature stops a dishonest relay, not
  someone with root on Dell (see "Callers and their keys" item 3);
  `sign()` refuses it, and a `log-head` signature is not itself a log
  entry.
- A `recovery`: the step (clock-floor reset, rebuild and resume, GitHub
  re-copy or index rebuild), the instance, and the named counters or
  the agreed head. A GitHub re-copy names the altered entry's counter
  and hash; the other three steps name the head counter and hash they
  were made against and apply only against that head. Each step applies
  only while the stop it answers is in force, so an old one cannot be
  replayed. On Dell the relay delivers it through
  `receive_recovery(step)`; on the owner's PC the owner tool's account
  does.
- A `head-confirmation`: the instance whose log it covers, the counter,
  the head hash, the time, and the highest counter up to which every
  release-key entry was found on GitHub (none before the first release).
- A `site-key-change`: the service's instance, the version, the
  previous change's digest, and each site-link key added, retired or
  revoked.
- A `site-key-record-reset`: the service's instance, the last site-key
  change's version and digest, and the service log counter it was made
  against.
- A `policy-record-reset`: the instance, the highest policy version and
  digest to record, and the target instance's head counter it was made
  against.

### Callers and their keys

1. Each principal that signs runs under its own operating system
   account, on every instance. The policy names every principal. On Dell
   they include each agent product (its review records and the
   research-card acceptances it makes), the runner supervisor (spec
   0008's runner, which also signs `evidence-bundle` and `review-result`
   records), `qualify` (ledgers, run manifests and group records, spec
   0064), GanitEvolve (receipts only), the gate (cache entries, and
   `gate-run` records for its spec 0008 verdicts and for a signed full
   run or phase check, whose encoding spec 0010 owns), the release step
   and the relay, whose own Dell account is used only by its connection.
   Signed review checklists are `review-record`s. On the owner's PC they
   include the service, which alone may ask for `site-message`, the
   relay, the fetcher and the owner tool. One account belongs to one
   principal, and one principal has one account. Candidates under test
   have no access to the socket, and only the runner supervisor's
   account may ask for the runner purposes: `runner-result` (spec 0064),
   `evidence-bundle` (spec 0003) and `review-result` (spec 0008). The
   core identifies a caller only by the account the operating system
   reports for the socket's peer; a name or field the caller supplies
   never identifies it.
2. The policy binds each account to one principal and lists the purposes
   it may ask for. A principal has one active key for each purpose, and
   a request is signed only with that key. An agent product can ask for
   `review-record` signatures only under its own key, so the gate's rule
   that a reviewer never wrote the change ([AGENTS.md section
   7.1](../../AGENTS.md#71-the-required-reviews)) can check which
   product signed. A `review-result` the runner supervisor signs names
   the reviewing product in a field, under an encoding spec 0008 owns,
   and the gate checks it as spec 0008 item 8 says. An agent product is
   a principal the policy in force allows `review-record`; the gate
   learns this through the socket's `principal` call. The runner's
   principal never counts as a reviewer. A `review-result` never stands
   in for a spec 0064 member review, which still needs the reviewer's
   own signature.
3. Threat model: the account binding stops mistakes and ordinary
   misuse. Anyone with root on Dell, which agents have, can read Dell's
   keys and sign outside the core. That is why the owner's key and the
   copy of Dell's log live on the owner's PC, so a forged waiver or a
   rewritten log on Dell is caught there, and why every release-key
   signature is copied to GitHub, where a release with no copied entry
   stands out. The service can read the site-link key under its own
   account, and so can sign site messages outside the core; spec
   0021's changeover and revocation rules are the defence there.

### The signer registry

1. Each instance's registry maps each public key to one principal, the
   purposes it may sign for and, for each purpose, the scopes its owning
   spec defines (for `runner-result`, the checks and targets the key may
   sign for, spec 0064). The core never reads scopes; a check returns
   them for the caller to compare with its own fields. It is part of the
   protected policy, whose trust root is the owner's key, set outside
   any candidate or run (spec 0064, "Freeze the run").
2. Adding, revoking or changing a key needs a `policy-change` signed by
   the owner. At first start the instance generates and seals its keys,
   each marked in its sealed file as not yet in use, and the owner signs
   a version-1 policy naming their public halves; on the owner's PC this
   signature is made with the owner's key after in-person approval, one
   of the owner signatures, one per instance, made before any policy is
   in force. An instance with no log, no policy-version record and no
   sealed key already marked in use accepts a version-1 policy whose
   previous digest is empty. Only the link to a previous policy and the
   version record are waived: that policy must still carry a valid owner
   signature, checked with `gate/owner-key.pub`, and name this instance.
   Every policy names the instance it is for and has a version that only
   goes up. Applying a policy writes a log entry with its own counter,
   and applying the first one marks its keys in use.
3. Site-link keys are kept apart from the owner-signed registry, in the
   service's instance's own site-key list, which the registry names only
   by purpose. They are rotated and revoked from the app with spec
   0021's changeover period: the owner approves each change in person on
   the owner's PC, and the owner's instance signs the change as a
   `site-key-change` naming the service's instance, with a version that
   only goes up and the previous change's digest. The service's instance
   applies only such a signed change whose previous digest is the last
   change it applied. While the service's log holds no `site-key-change`
   entry and no site-key record exists, its list is empty and its record
   holds version 0 with an empty digest, so the first change names an
   empty previous digest. Changes are delivered by the owner tool's
   account through `receive_site_key_change(change)`, and the service's
   instance writes each one as a log entry. A change adds only keys the
   service's instance generated. It keeps, durably, the version and
   digest of the last change applied; when that record is missing or
   unreadable it refuses `site-message` and site-key changes until the
   owner sets it again with an owner-signed `site-key-record-reset`,
   built from the last change the service's own log shows applied. The
   owner tool's account alone delivers it, through
   `receive_site_key_record(reset)`, and it applies only while the
   record is missing and never below the version in the service's log. A
   socket request alone never changes the list. The list holds only the
   service's own site-link keys, each with its key id, its state
   (active, retired or revoked) and when its changeover ends. dwarpal
   keeps each add-on's public key and checks incoming messages under
   spec 0021; the core only signs the service's outgoing messages.
4. A key replaced by rotation is retired: its private half is destroyed
   and the destruction is logged. The check library still accepts its
   signatures; only the socket's `verify`, using the log index, can
   say whether one was made while the key was active. For site-link
   keys, spec 0021's changeover rules decide instead when an old key is
   refused.
5. A revoked key is refused by the check library in every case. An
   owner-signed revocation names the last Dell head the owner's instance
   confirmed. Only the instance that holds the log can answer that a
   signature was made before revocation: the signature's entry is at or
   below that named head, and the purpose's owning spec allows the
   answer. The runner purposes and `site-message` never allow it.

### Checking signatures

1. Checking needs no private key. Every Rust caller checks in its own
   process with the check library. Go tools (AGENTS.md section 4), such
   as the gate, GanitEvolve, the relay and the fetcher, cannot link it,
   so they check through the socket's `verify` call. Where there is no
   socket, as in GitHub CI (roadmap Phase 2 item 11), they run
   `tijori-verify`, a small keyless command built from the same check
   library. It checks only with Dell's instance's owner-signed policy on
   the protected main branch (spec 0008, "Protected policy and installed
   tools", item 1), never a copy in the change being checked. It refuses
   a policy for another instance or one that fails the check with
   `gate/owner-key.pub`, which it also reads from protected main, never
   from the change. It also refuses when that policy's version is lower
   than the highest version in main's history, so pushing an older
   signed policy does not roll it back, and it refuses when it cannot
   read that full history, main's policy or main's `gate/owner-key.pub`.
   `tijori-verify` is a protected path (spec 0008), and CI runs only a
   copy built from protected main. It also answers `principal(name,
   policy_digest)` from that same main-branch policy, with the same
   refusals. It refuses anything only the log could answer, such as
   "made before revocation". No second check library is written.
2. The library never reads files itself. It gets the registry and the
   policy version through a trait the caller passes in, with an
   in-memory fake for tests. The real one is the policy trust source in
   `sutradhar-tijori`: it reads the protected policy file and the
   record of the highest version accepted, and never writes either.
   Every Rust caller uses it; `qualify` gets it through `QualifyTrust`
   (spec 0055).
3. The trait reads the policy file once per load and checks those exact
   bytes, then keeps that copy in memory for every check. It loads again
   only when the file's version or digest changes. The record of the
   highest version and digest accepted has one writer on each instance,
   the `tijori` program, which raises it when it applies a newer policy;
   every caller's trait only reads it. Each instance keeps its own
   record. It accepts the same version only when the digest matches, and
   a higher version only with a valid owner signature checked with
   `gate/owner-key.pub`. A lower version, a same version with other
   bytes, a bad signature or an unreadable file is refused, and the
   library then refuses checks rather than answer from an older copy. A
   missing or unreadable record of the highest version accepted is
   treated like an unreadable policy file: checks are refused until the
   owner sets the record again with the owner tool, from the last policy
   version that instance's own log shows applied, as an owner-signed
   `policy-record-reset`. A reset applies only on first start or when
   the record is missing, and never records a version lower than one
   already in that instance's log. On Dell every caller's trust trait
   reads the record Dell's instance keeps, and the relay delivers the
   reset to it; on the owner's PC the owner tool's account delivers it.
   The trait refuses a policy that names another instance. Each instance
   applies its own policy the same way.
4. A check takes the key id, purpose, bytes, signature and the policy
   digest the caller expects. It returns the principal that signed and
   the key's scopes for that purpose, or a refusal with its reason:
   unknown key, revoked, wrong purpose, bad signature, policy older than
   the one in force, or a policy digest that is not the one in force.
   Callers compare the principal with any author they need to rule out;
   a check never reads fields inside the bytes.
5. Checks are strict: a non-canonical signature or a small-order public
   key is refused.
6. The socket's `verify` call uses the same library and adds the one
   answer only the log can give: made before revocation.

### Signing

1. Signatures are Ed25519, from one pinned library listed in
   `gate/packages.toml` for that one job. The library is chosen by a
   research card (spec 0013) and must pass the test vectors of RFC 8032
   section 7.1.
2. `sign(purpose, bytes)` checks the caller, the purpose and the
   caller's active key under the policy in force, signs and writes the
   log entry in one step, and only then returns the signature. It
   refuses any purpose bound to the release key.
3. A repeat of a request with the same principal, purpose and bytes
   first checks that the caller may still ask for that purpose and that
   the logged key is not revoked, and that the logged entry's caller,
   purpose and bytes hash match the request, then returns the signature
   already in the log, even if that key has since been retired, so a
   crash between the log write and the reply never blocks a retry. When
   the logged key has been revoked, the request is treated as a new
   request under every rule, signed with the active key, and the index
   points to the new entry. For the runner purposes and `site-message`
   it is refused instead, until the owner's recovery plan has run, so
   bytes signed under a revoked key never come back clean. For
   `release-record` and `release-manifest` it is always refused: a
   release whose key was revoked is never signed again, and after
   recovery the owner makes a new release with a new version.

### Release guards

These are the core's own rules for the release key, kept from the
archive design. The core reads fields inside the bytes in three places
only: these release guards, the `site-message` first-line check, and
the owner's approval screen. Everywhere else it signs bytes.

1. The release key signs only through `sign_release(purpose, bytes,
   version, candidate, catalogue_digest, manifest_digest,
   release_ledger)`, for the `release-record` and `release-manifest`
   purposes. The bytes are the caller's encoding (specs 0005 and 0030).
   For `release-manifest` the core reads the version and candidate from
   their fields and takes the manifest digest as its own hash of the
   bytes, refusing unless that hash equals the typed `manifest_digest`.
   For `release-record` it reads the version, candidate and manifest
   digest from their fields. Each must match the typed fields. The
   manifest is signed first, then the record: a `release-record` is
   refused unless the log holds a release-key `signature` entry for this
   release's `release-manifest`, and its version, candidate, manifest
   digest and ledger digest must equal that entry's. The core finds that
   entry through the bytes-hash index, since the manifest digest is its
   own hash of the manifest bytes, never by scanning the log. A release
   is the pair of its version and its release-manifest digest. Both
   purposes use the same manifest digest and ledger, and the count and
   the version rules apply per release, not per signature (item 3).
2. A release request has typed fields: the version, the candidate, the
   catalogue digest, the manifest digest and the `release-ledger` that
   `qualify` published (spec 0064). The core accepts the ledger only
   through the reader check in `sutradhar-methods`, which compares the
   ledger's candidate and catalogue digests with the typed fields and
   its policy digest with the current protected policy digest. The
   `tijori` program gives that check a `QualifyTrust` that reads Dell's
   protected policy and run ledger read-only (spec 0055), using the
   `tijori` program's own record of the highest policy version accepted,
   and refusing when that record is missing. The core refuses unless the
   ledger shows a complete pass and its candidate digest equals the
   typed candidate. For `release-manifest`, the core also refuses unless
   every artefact hash in the manifest is covered by the ledger's
   candidate digest, under the candidate encoding spec 0005 owns. Until
   spec 0005 defines that encoding, the real release key signs no
   manifest. The ledger's own signature must also be in this core's log,
   at or below a confirmed head, so a ledger signed outside the core is
   refused. For `release-manifest` only, the ledger's signature entry
   must be later in the log than the ledger of the latest
   `release-manifest` signature entry of any other release, so an older
   candidate's passing ledger cannot be released under a new version; a
   `release-record` instead matches its own manifest's entry (item 1).
   The `tijori` program uses `sutradhar-methods`, which uses the check
   library, so the crates still depend in one direction.
3. Versions only go up. A version is never used by two releases, and
   each purpose is signed once per release, except as the repeat in
   "Signing" item 3.
4. Dell's protected policy sets the owner's release limit as a count and
   a window. A release request first writes its own `release-pending`
   entry with the current time, and a retry within the window reuses
   that entry; a pending entry older than the window lapses, and a retry
   then writes a new one. The window ends at the time of the latest head
   the owner's instance has confirmed, which must be at or above this
   request's `release-pending` entry, and every release whose
   `release-manifest` signature entry is later than the window's start
   counts, however recent, so neither a clock set forward on Dell nor an
   older head can widen it. A request that reaches the count is signed;
   the next one inside the window is refused. A repeat is not counted
   again.
5. The release key signs only when the head at the time of the request
   has been confirmed by the owner's instance; a confirmed head at or
   above the release's `release-pending` entry meets this. That
   confirmation's GitHub counter is at or above the last release-key
   entry before this release's `release-pending` entry ([AGENTS.md
   section 8.1](../../AGENTS.md#81-releases), item 4). Later entries
   made while the release waits do not hold it up.
6. Release requests are handled one at a time. After any wait, every
   release guard and the policy check run again, in the same step as
   the log write.

### Keys at rest

1. Private keys are kept encrypted in files that only the instance's
   own account can read, sealed to the machine they belong to. They
   never appear in a setting, a log, an error, a crash report or an
   export.
2. Agents, CI jobs, candidates under test and the release step never
   get a private key. They reach the core only through its local
   socket.
3. Each instance has no network access, gains no new rights after it
   starts and sees a read-only view of the system. Its memory use is
   measured and reported.
4. Keys are never rotated silently. Rotation, revocation and recovery
   are owner-signed policy changes, except site-link keys (see "The
   signer registry" item 3).

### The signature log

1. Every signature goes in an append-only log entry. The head is the
   latest entry's hash with its counter, so changing any earlier entry
   changes every head after it. Each entry has a kind: `signature`,
   `release-pending`, `policy-applied`, `policy-record-reset`,
   `site-key-change`, `site-key-record-reset`, `key-destroyed` or
   `recovery`. Every entry has the time, kind, counter and previous
   entry's hash. A `signature` entry also has the caller, purpose, key
   id, bytes hash, signature and both indexes' running hashes, and for a
   release-key signature also the version, candidate, manifest digest
   and ledger digest; a `release-pending` entry the version and manifest
   digest; a `policy-applied`, `policy-record-reset`, `site-key-change`
   or `site-key-record-reset` entry the version, digest and, for a
   reset, the counter it was made against (the service's instance writes
   the site-key kinds); a `key-destroyed` entry the key id; and a
   `recovery` entry the step and the owner's signature. Only `signature`
   entries go in the indexes and the repeat rule, and only signed
   releases count towards the release limit. The release key's
   `release-pending` and `signature` entries are the ones copied to
   GitHub.
2. The counter is an unsigned 64-bit number that only goes up. When it
   reaches its largest value, signing stops and the owner is told; it
   never wraps round. An entry's time is the later of the clock and the
   previous entry's time, so log times never go backwards, except
   through the owner-signed reset step below. When the
   clock is behind the latest log time by more than a bound the policy
   sets, signing stops and the owner is shown the recovery plan. A quiet
   spell, with the clock ahead of the last entry, never stops signing;
   a clock set forward is caught by the owner's instance (see "The
   relay" item 3).
3. Two indexes are written in the same durable write as the entry: one
   from principal, purpose and bytes hash to counter, and one from key id
   and signature to counter. The repeat rule and the
   before-revocation answer use the index, never a scan of the log.
4. Each index carries a running hash over all its entries in order,
   kept in the same durable write, and each `signature` entry records
   both indexes' running hashes after it is added. A revoked-key repeat
   adds a new index entry that supersedes the old one. On start, the
   core checks that each index's latest counter and running hash match
   those recorded in the latest `signature` entry at or below the log
   head. An index rebuild replays every `signature` entry in order and
   must reach the same running hashes. A missing or
   mismatched index is treated like a missing log: signing stops until
   the owner-signed index-rebuild recovery step (item 6) rebuilds the
   index from the log.
5. The entry and the index are written durably before the signature is
   returned. Requests that arrive together may share one durable write;
   identical requests (the same account, principal, purpose and bytes)
   within one write are merged into one entry.
6. Four recovery steps, each a logged change signed by the owner:
   - index rebuild: rebuilds the indexes from the log after item 4
     finds them missing or mismatched;
   - clock-floor reset: resets the log-time floor after a clock that
     jumped forward has been corrected, naming the counters written
     while the clock was ahead so the owner's instance accepts those
     entries' times;
   - rebuild and resume: rebuilds the owner's copy of Dell's log from
     the last head both sides agree on, after a bad entry was confirmed,
     and lets Dell resume once a new confirmed head matches its own log;
   - GitHub re-copy, bound to the altered entry's counter and hash
     rather than the moving head: has the relay copy a deleted or
     altered GitHub entry again, after which the owner's instance checks
     it afresh.
7. Loss of a sealed key, a reset counter, a missing log or a changed
   trust root stops all signing. The core keeps the evidence and shows
   the owner a recovery plan (new key, revocation, new trust root,
   checking the history). It never picks a new trust decision by itself.
8. The owner tool's log view is paged.
9. When the owner's instance is itself stopped by item 2 or item 4, it
   may still sign the one recovery step that answers that stop, after
   the owner approves it in person in the owner tool, and nothing else
   until that step is applied.

### The relay

1. The relay is an operations tool, written in Go ([AGENTS.md section
   4](../../AGENTS.md#4-languages-and-files)), with no keys and no
   rights beyond its own account. It runs on the owner's PC and reaches
   Dell's instance's socket over the existing connection to Dell; the
   instances themselves open no network connection. Dell's instance
   signs each head it hands out under a `log-head` purpose, and the
   owner's instance accepts entries only when a Dell-signed head covers
   them, so the relay cannot invent an entry. The relay reads Dell's new
   log entries through Dell's socket and carries them to the owner's
   instance; it carries owner-signed policies and confirmed heads back
   to Dell; and it copies each release-key log entry to GitHub.
2. Signing never waits for the relay or for the owner's PC, except the
   release key (see "Release guards" item 5).
3. The owner's instance keeps its own copy of Dell's log. It checks that
   each new entry links to the one before, and tells the owner when the
   log, counter or policy looks older or the chain is broken. It
   returns the head it has checked, signed with its head-confirmation
   key. It refuses to confirm a head whose entry times are ahead of its
   own clock by more than the policy's bound, and tells the owner; since
   the release key needs a confirmed head, a clock set forward on Dell
   cannot reset the release window.
4. Dell's instance checks that signature and that its own entry at that
   counter has the same hash. When it does not, or when its own log is
   older than the confirmed head, Dell stops signing and tells the
   owner.
5. A fetcher on the owner's PC, under its own account and separate from
   the relay and from the owner's instance, reads the GitHub copy of
   each release-key entry from the repository and path the policy names.
   The owner's instance compares those exact bytes with its own copy of
   Dell's log, stays off the network itself, and says so in its signed
   confirmation.
6. When no confirmed head has arrived for longer than the policy allows,
   the owner's instance tells the owner. It also tells the owner when a
   policy it signed for Dell's instance has not appeared in Dell's log
   within the policy's wait (a policy counts as arrived once Dell's log
   shows that version or a higher one for Dell's instance), and while
   such a policy is still waiting it confirms no Dell head at all.
   Policies for the owner's or the service's instance never hold Dell
   heads back.
7. The policy sets the longest wait (a longer caller time limit is cut
   to it), how many requests may wait at once, and the most entries one
   `log_since` reply carries. A waiting request gets a plain refusal
   when its wait runs out; one past the waiting count is refused at
   once. The relay asks `log_since` page by page until it catches up.

### Interfaces

1. The socket API on every instance: `sign(purpose, bytes)`,
   `verify(key_id, purpose, bytes, signature, policy_digest)`,
   `principal(name, policy_digest)`, which returns the purposes a named
   principal may sign for under that policy, or a refusal when the
   digest is not the one in force or the name is unknown;
   `log_since(counter)` and `log_head()`. On Dell only the relay's
   account may call `log_since` and `log_head`; on the owner's PC, the
   relay's account and the owner tool's account.
2. Dell's instance also has `sign_release(purpose, bytes, version,
   candidate, catalogue_digest, manifest_digest, release_ledger)`, which
   the other instances refuse, and four calls only the relay's account
   may make: `receive_recovery(step)`, which applies an owner-signed
   recovery step under the rules above; `receive_policy(policy)`, which
   applies a policy only after checking its owner signature, its
   instance, that its version only goes up and that its previous-policy
   digest is the policy in force, so a policy built on a policy Dell has
   not applied is refused; `receive_head(head)`, which checks the
   head-confirmation signature before Dell uses it; and
   `receive_policy_record(reset)`, which applies an owner-signed
   policy-record reset. Only the core writes Dell's protected policy
   file, after those checks. The owner tool reads Dell's log only
   through the owner's instance's copy. Every reply is a typed result or
   a refusal with a reason.
3. The owner's instance also takes, each from one account only:
   `receive_log(entries)` from the relay's account,
   `receive_github_copy(entry)` from the fetcher's account, and
   `next_outgoing()` from the relay's account, which hands over
   owner-signed policies and confirmed heads for Dell. From the owner
   tool's account it takes `receive_policy(policy)` for its own policy,
   `receive_policy_record(reset)` and `receive_recovery(step)`. Every
   other account is refused.
4. The service's instance takes, from the owner tool's account only,
   `receive_policy(policy)` for its own policy,
   `receive_site_key_change(change)`, `receive_site_key_record(reset)`,
   `receive_policy_record(reset)` and `receive_recovery(step)`, each
   checked as above. Every other
   account is refused.
5. The check library and its trust trait, for every caller's own
   process. `tijori-verify`, the keyless command for checking where
   there is no socket.
6. A Rust trait, `KeySource`, the client for the socket: it asks for
   signatures and never holds a private key. Its test fake uses fixed
   test keys and never touches real ones (spec 0021).
7. The owner tool, on the owner's PC: list keys and purposes, page
   through the signature log, add or revoke a key (owner-signed), set
   the record of the highest accepted policy again, and run the
   recovery plan. Phase 5 item 5 brings these commands under `sutra
   keys`.

## Limits

- Apart from the three places named under "Release guards", the core
  signs bytes; it never decides whether a change, review or
  release is good. Each caller makes that decision under its own spec.
- No outside service holds a key of the core.
- The core holds no product data and no member data.
- Before the signing core exists, records are signed as AGENTS.md
  section 7.3 says.

## Footprint budget

None. The core never runs on a site's server.

## How it is tested

Rust tests in the tijori crates, with test keys only:

- The test vectors of RFC 8032 section 7.1 sign and check exactly.
- A signature made for one purpose is refused for every other purpose,
  even with the same key and bytes; the purpose field and the caller's
  bytes after it match spec 0064's layout byte for byte.
- `site-message` bytes are signed with no added prefix, and bytes that
  do not begin with `SUTRADHAR-ED25519` and a line feed, such as
  `SUTRADHAR-ED25519X`, are refused.
- A key used for a purpose its entry does not list is refused, with the
  reason.
- An account not bound to a principal is refused. Principal A's account
  cannot get a signature under principal B's key. An account not
  allowed a purpose is refused it.
- An agent product's account cannot get a `review-record` signature
  under another product's key.
- Dell's instance cannot make an owner waiver or `policy-change`
  signature, and the owner's instance makes none from a socket request
  without the owner's approval on that PC. The head-confirmation key is
  refused for `waiver` and `policy-change`, and `sign()` refuses
  `head-confirmation`.
- The approval screen shows every field of every owner-signed purpose
  and of every site-key change, marks each field a policy change alters,
  and is built from the signed bytes; a principal name with control,
  line-feed or direction-changing characters is shown escaped; an
  over-long field is refused.
- A candidate under test cannot reach the socket, and only the runner
  supervisor's account gets a `runner-result`, `evidence-bundle` or
  `review-result` signature.
- A purpose name with other characters, or over its length, is refused.
  A request whose stated length is over the whole-request limit, or
  over its purpose's byte limit, is refused before the rest is read.
- Applying a policy writes a log entry.
- `sign` refuses `release-record` and `release-manifest`.
- A check returns a runner key's checks and targets, and a caller test
  shows 0064's verifier refusing a runner key used outside them.
- A socket request alone cannot change the service's site-key list; a
  change approved in person on the owner's PC is logged; an old
  site-link key is refused after spec 0021's changeover.
- A retired key's private half is gone; the socket says whether a
  retired key's signature was made while it was active.
- After a restart the trust trait accepts the policy already in force,
  refuses a same-version policy with other bytes, and checks the owner
  signature once per load however many checks follow.
- A key revoked while a release waits makes that release refused when
  it resumes; a queued request for the same release and purpose is
  refused once the first is signed, and a request for another release
  with that version is refused.
- A release whose ledger signature is not in the core's log is refused;
  a release whose bytes carry another version or manifest digest than
  its typed fields is refused.
- With the service's record of the last site-key change deleted, an
  older owner-signed change is refused and `site-message` stops until
  the owner resets the record.
- A deleted or rolled-back index stops signing at start; no repeat is
  signed afresh and no release gets a second signature until the index
  is rebuilt from the log.
- Every account but the release step's is refused `sign_release`, and
  until Phase 7 the real release key signs nothing.
- A site-key change that is unsigned, wrongly signed, for another
  instance or older than the last one applied is refused.
- A repeat of a runner-purpose or `site-message` request whose key was
  revoked is refused until the recovery plan has run; one of
  `release-record` or `release-manifest` is always refused; any other
  such repeat passes every rule afresh.
- After a forward clock jump is corrected, the owner-signed reset step
  lets Dell sign again; after a relay adds a made-up entry, the
  owner-signed rebuild step restores the owner's copy from the last
  agreed head.
- A policy the owner signed for Dell's instance that the relay holds
  back is reported to the owner.
- The gate's account gets `gate-run` signatures; other accounts are
  refused them.
- A check with a policy digest that is not the one in force, newer or
  unknown, is refused.
- Policy N+2 delivered before N+1 is refused until N+1 applies; once
  both apply, neither is reported as held back.
- After the clock reset step, the owner's instance accepts the named
  entries and confirms heads again.
- An entry the relay invents without a Dell-signed head is refused by
  the owner's instance; after the rebuild step Dell resumes; after the
  re-copy step a release is no longer held back.
- The approval screen flags a look-alike or invisible character in any
  field.
- Two requests from different principals with the same bytes are not
  merged.
- A release whose key was revoked is never signed again; a new release
  with a new version is.
- On Dell, an owner-signed `policy-record-reset` delivered by the relay
  sets a missing record every caller reads; an unsigned one is refused,
  and a replayed older reset is refused once the record exists or
  Dell's log holds a higher version.
- `sign()` refuses `log-head`; a `log-head` signature adds no log
  entry; each recovery step is applied only with its owner signature
  and writes a `recovery` entry.
- A replayed older `policy-record-reset` is refused on the owner's and
  the service's instances as on Dell, checked against that instance's
  own log.
- The first release needs no previous GitHub copy.
- GanitEvolve's account gets `receipt` signatures only, and is refused
  `gate-run` and `cache-entry`.
- A `release-manifest` whose own hash differs from the typed digest is
  refused. A `release-record` whose manifest-digest field differs from
  the release's `release-manifest` signature entry is refused, and so is
  one whose manifest was refused or never asked for, or that names a
  different version from that entry. A record asked for after the
  manifest's `release-pending` entry has lapsed is checked against that
  signature entry and signed.
- A release retry reuses its `release-pending` entry and signs once any
  confirmed head covers it.
- A repeat whose index points at an entry for other bytes is refused.
- A `release-manifest` with one artefact hash changed from the
  qualified candidate's is refused.
- A policy the owner signed for the service's instance does not hold
  Dell heads back.
- A GitHub re-copy step bound to the altered entry applies while
  ordinary signing goes on.
- Restarting after a `policy-applied`, `release-pending` or `recovery`
  entry does not stop signing.
- A site-key change whose previous digest is not the last one applied
  is refused, so a held-back revocation cannot be skipped.
- While a signed policy is held back, no head is confirmed, so no
  release is signed.
- A replayed or out-of-date recovery step is refused; one applies only
  while its stop is in force.
- Every policy-wide setting is part of the signed policy and shown on
  the approval screen.
- With an older owner-signed policy in the change, `tijori-verify`
  refuses a record signed by a key the newer policy on main revoked.
- In a run with no socket, `tijori-verify` refuses a review record with
  a bad signature or a revoked key, and refuses a "made before
  revocation" question.
- Another account's `receive_policy`, `receive_recovery`, `receive_head` or
  `receive_policy_record` call is refused;
  a policy or head with a bad signature, or for another instance, is
  refused.
- A relay that lies about the GitHub copy does not get a confirmation;
  a missing or altered GitHub copy holds the release back.
- With the record of the highest accepted policy deleted, an older
  signed policy is refused and checks stop until the owner resets it.
- A clock set back behind the log by more than the bound stops signing
  and shows the recovery plan; a quiet spell longer than the bound
  still signs; the owner's instance refuses to confirm a head whose
  times are ahead of its own clock.
- A principal or account name with a look-alike letter from another
  script is refused.
- A policy for the owner's PC replayed to Dell is refused. A policy whose
  previous-policy digest is not the one in force is refused, so N+2
  built on a held-back N+1 cannot bring back a key N+1 revoked.
- A `site-key-record-reset` applies only while the record is missing,
  only from the owner tool's account, and never below the version in
  the service's log; a replayed one is refused.
- The two purposes of one release must carry the same manifest digest
  and ledger.
- `principal` with a policy digest that is not the one in force, newer
  or unknown, is refused, through the socket and through
  `tijori-verify`.
- `principal` returns a named principal's purposes under the policy in
  force and refuses an unknown name; the runner supervisor, `qualify`,
  the gate and the release step are not agent products.
- A Go tool's check through the socket's `verify` refuses a small-order
  key and a non-canonical signature, as the check library does. A
  caller's trait cannot write the policy-version record.
- Two identical requests in one durable write give one entry and one
  signature.
- `qualify`'s ledger signatures are made under its own principal and
  are found in the log by the release guard.
- The service's account alone gets `site-message` on the owner's PC.
- A wait longer than the policy's limit is cut to it; a request past
  the waiting count is refused; `log_since(0)` on a large log returns
  pages no larger than the policy allows. Accounts other than the
  relay's and the owner tool's cannot read the log.
- Unknown and revoked keys are refused by the check library; a retired
  key's signatures still pass the library. Through the socket, a
  signature at or below the head a revocation names is reported as made
  before revocation for purposes that allow it; one after it is refused;
  the runner purposes and `site-message` are refused after revocation in
  every case.
- A release signed after the latest confirmed head still counts
  against the window, so a retry cannot leave it out.
- A `release-manifest` for a new version whose ledger is not later
  than the ledger of the latest other release's manifest is refused,
  also when that release's record was never signed; the
  `release-record` of the same release, with its own ledger, is signed.
- A `release-record` naming another candidate or ledger than its
  manifest's signature entry is refused.
- The owner's instance stopped by a clock behind its log signs only the
  clock-floor reset, after in-person approval, and then resumes.
- The service's instance at first start applies a first
  `site-key-change` with an empty previous digest, and refuses one with
  any other previous digest.
- First start is accepted only when the instance has no log, no
  policy-version record and no sealed key marked in use, with a
  version-1 policy whose previous digest is empty; deleting the log and
  the record is not treated as first start, because its keys are marked
  in use. An unsigned or wrongly signed version-1 policy, or one for
  another instance, is refused at first start.
- A release retried with a pending entry older than the window counts
  in the window of its new entry's confirmed head, not of the old entry.
- `tijori-verify` answers `principal` from main's policy as the socket
  does, and refuses an unknown name.
- `tijori-verify` ignores a policy copy and an owner-key copy in the
  change, and refuses when main's policy version is lower than the
  highest in main's history, including after two pushes; it refuses on a
  shallow checkout that lacks main's full policy history.
- The trust trait refuses a policy file with a bad owner signature, an
  older version signed again (also after a restore), and an unreadable
  file; the library then refuses checks. A newer signed policy is
  loaded, and a key it revokes is then refused.
- A `review-record` copied from one required review into another's
  record is refused by the gate.
- A check returns the principal that signed; a caller test shows a
  record whose author is that principal is refused as its own reviewer;
  a `review-result` signed by the runner supervisor that names the
  change's own author as the reviewing product is refused by the gate,
  and so is one naming an unknown or look-alike product, one carrying
  another change's fingerprint, and one for another required review
  (spec 0008, AGENTS.md section 7.1); spec 0064 owns the rule that a
  record is refused as its own reviewer.
- Altered bytes and a truncated or malformed signature are refused; a
  non-canonical signature and a small-order public key are refused.
- A release whose ledger fails the reader check or is for another
  candidate is refused. A release version signed twice, or lower than
  the last, is refused; the request that reaches the release count is
  signed and the next one inside the window is refused; the release
  window counts only up to a confirmed head, so a clock set forward on
  Dell does not widen it. The release key does not sign until the head
  at request time is confirmed with a GitHub counter at or above the
  last release-key entry before its `release-pending` entry, and a busy
  log does not hold a release up for ever.
- Crash at each step of signing (before the log write, after it, before
  the reply): a retry returns the same signature, a release can still
  be signed and is not counted twice towards the limit, and no counter
  gives out two different signatures. A repeat after the key was
  retired returns the logged signature; after it was revoked, it follows
  the revoked-key repeat rule.
- A repeat and a before-revocation answer read the index, never the
  whole log (checked with a large test log).
- A counter at its largest value stops signing instead of wrapping.
- A restored older log, counter or policy, and a rewritten entry with a
  higher counter, are reported by the owner's instance; Dell stops
  signing when a confirmed head does not match its own entry or is
  newer than its log. A confirmed head with a bad signature is refused.
  A clock set back never makes a log time go backwards.
- Signing goes on, and is not slowed, while the relay or the owner's PC
  is off; the owner is told when no confirmed head has arrived for too
  long; a waiting request is refused when its caller's time limit runs
  out.
- A missing sealed key, a reset counter and a changed trust root stop
  signing, and the recovery plan is shown.
- No private key appears in any log, error or crash output (checked
  with planted marker keys).
- On Dell: the instance has no network, no new rights and a read-only
  system view, and its memory use is measured and reported.

## Changes from the source

- The archive design (ADR-0050, "Release signing (tijori)") made tijori
  a SPARK service for release signing only. AGENTS.md section 4 now
  makes the product Rust, so the core is Rust, and it signs for every
  purpose in the table above, not only releases.
- The archive's minisign-compatible format is not required; each
  caller's spec sets the signed encoding after spec 0064's purpose
  field.
- The archive's fixed daily release limit becomes a count and a window
  in Dell's protected policy.
- Key sealing names no particular chip or tool; the machine binding is
  chosen and tested when the core is built.
- The owner's key and a copy of Dell's log live on the owner's PC, so
  root on Dell cannot forge a waiver or hide a rewritten log.

Source: `docs/adr/0050-tijori-signing.md` in the archive
`archive-2026-11-08-old-repo`, with the key rules of specs 0005, 0021
and 0064.
