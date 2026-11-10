# 0036: Audit proof

Audit proof adds Merkle proofs and forward-secure signed checkpoints on
top of the audit log's hash chain (spec 0032). A Merkle proof shows one
event without showing the others. A signed checkpoint, kept outside the
database, catches a full rewrite of history. It is roadmap Phase 5
item 4, as part of the rest of tijori.

Example: a helper asks, "Did a moderator approve that link change on 3 March?"
With the chain alone, the owner can only hand over the whole log. With
this part, the owner hands over one small proof file, and the helper
sees that event and no other.

## What it does

### What a hash chain cannot do

1. Show one event alone. To trust one event you must check the whole
   chain, so you must be shown every event.
2. Stop a full rewrite. Someone who can write the database can change an
   old event and recompute every hash after it. The chain still checks,
   unless a head hash was kept out of their reach.

### What each addition gives

- Merkle tree: event hashes are paired, then the pairs paired, up to one
  top hash, the root. A proof for one event is the few hashes on its
  path (about 24 for 10 million events). It ties the event to the root
  without showing the others. This fixes point 1.
- Consistency proof: shows a newer checkpoint extends an older one, so
  nothing before it was rewritten, with no replay of the log.
- Signed checkpoint: a root carrying Sutradhar's signature, kept outside
  the database. This fixes point 2.
- Forward-secure key: the key changes each period and the old key is
  destroyed. A thief with today's key can sign new checkpoints but
  cannot forge ones that look older than the theft. Past signatures stay
  checkable with the public key. This stops backdating.

### Who the proof is for

- The site owner, to learn whether anyone with database access has
  rewritten history. This is the main use.
- One helper the owner chooses, shown one proof file and the public key,
  never the log.
- Not guests, members, moderators or other installs. A proof never shows
  more than the person may already see. Installs share nothing, so none
  vouches for another.

### What it reuses

- The audit log's BLAKE3 event hashes are the leaves. No new hash
  function and no change to event shape.
- `sutra audit verify` and the nightly chain check gain a checkpoint
  step. The Audit screen gains one button, "Prove this event".
- tijori's key rules: the key sealed to the machine, a local socket with
  the peer checked, no network, an append-only signature log, a counter
  against replay, and no silent key rotation. Sutradhar owns every key;
  no outside service holds one.
- The owner downloads a proof; nothing is sent anywhere.

### What is new

1. A checkpoint every so often: the Merkle root over a block of events,
   with the previous root, signed. One new append-only table.
2. A proof builder and a small offline verifier: proof file and public
   key in, "valid" or "not valid" out. No database, no network.
3. An audit signing key, separate from the release key, held in a
   second instance of tijori's design with its own key, so release and
   audit never mix. A release key signing a checkpoint, or the audit key
   signing a release, is refused.
4. A home for the latest signed checkpoint outside the database: a
   protected folder on the owner's PC, and a file the owner downloads
   regularly. No outside time-stamping service in version 1.

### Erasure: proofs survive, personal data does not

Erasing a person deletes their audit details and their actor mapping;
the events stay, with a neutral stand-in line ("removed member"). This
part must not weaken that.

- A leaf is the event hash only. An event holds identifiers and
  outcomes, never names, addresses or text. So erasure changes no leaf,
  root or signature, and every earlier proof still checks.
- A proof shows the event as kept, with the stand-in line. The proof
  file holds no details and no subject mapping.
- Nothing personal goes into a leaf, root or key. No hash of an email
  or member id, since a hash of something guessable is still personal.
  The actor reference must be random, not derived from a member id;
  research confirms the audit log does this.
- No key per person ("crypto-shredding"). Details are deleted, and that
  stays. Forward-secure keys destroy keys, never people's data.
- Erasure leaves a receipt event in the chain. Its proof shows an
  erasure happened and when, without saying whose.
- A restore applies the erasure list first, then checks the restored log
  reaches the last signed checkpoint. If not, it says so, and the audit
  log's gap event marks it.

### When the key is missing

- No fallback: no unsigned stand-in checkpoint is made.
- The chain keeps working and actions are not refused.
- A plain alert says checkpoints are late, with a code and a manual
  page.
- The key is not backed up, since a copy defeats forward security.
  After loss the owner chooses a recovery plan; a new period starts and
  a marker event records it.

### Settings

Every setting has a safe bottom and top, set by research. Shipped on.

| Setting | Meaning | Value |
| --- | --- | --- |
| Block size | Events per checkpoint | by research |
| Longest gap | Time before a checkpoint is forced | by research |
| Key period | How long one forward-secure key lasts | by research |
| Overdue alert | Checkpoint late, raise an alert | by research |

No setting turns signing off while the audit is on. No hard-coded value.

### What the audit log gains (spec 0032)

The checkpoint table, the "Prove this event" button, a checkpoint-age
gauge, a late-checkpoint alert and one new error code.

### Research first

Before any build, research answers:

1. Does the threat justify it? The advice of 4 November 2026, which
   the owner told agents to follow, was not to build this for
   version 1. "The chain plus an outside head is enough" is a valid
   result; if research finds it, this part is not built for version 1,
   and the roadmap change this needs is put to the owner.
2. Which forward-secure scheme: a fresh key per period signed by a
   parent, or a key hashed forward each period; its size and speed.
3. Block and proof size at 10 million events, and check speed; one
   growing tree or one tree per block.
4. Where the latest checkpoint can live out of a database intruder's
   reach.
5. How the audit key is sealed on the owner's PC, including one with no
   TPM.

### Not taken

Block-chain anchors, notary chains and sharing roots between installs
(installs share nothing); agent "attestation" that a check passed;
outside time-stamping services in version 1. No earlier code comes
across.

## Limits

- Checkpoints stream inside the service's 16 MiB audit budget, off the
  write path, so the audit log's speed targets hold. This sits inside
  the service slice, within the memory limits in AGENTS.md section 5.6.
- A proof is about 24 hashes for 10 million events.
- The verifier needs only the proof file and the public key.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Rust tests:

1. With 10 million events, a proof built for any one event is accepted
   by the offline verifier from the proof and public key alone.
2. A changed event fails its old proof.
3. A log rewritten and re-hashed from some event fails the nightly check
   against the signed checkpoint.
4. A consistency proof for a newer checkpoint passes, and fails if any
   earlier event was altered.
5. After a member is erased, all old proofs pass, and no proof file
   holds a name, address, text or subject mapping.
6. Property test: a leaf built from random inputs holds no field of a
   personal kind.
7. A stolen current key signing a past period is rejected by the
   verifier.
8. With the key missing, no checkpoint is made when due, one alert
   shows, and actions still run.
9. A restore to before the last checkpoint applies erasures first and
   reports the gap.
10. A release key signing a checkpoint, and the audit key signing a
    release, are both refused.

Tree building, proof checking, the key-period rules and the verifier
are proved with Kani, Verus or Creusot, aimed at "a changed event
fails" and "an old period's key cannot sign".

## Changes from the source

- Tree building, proof checking, key-period rules and the verifier are
  rewritten as Rust, proved with Kani, Verus or Creusot.
- The button and alert are in the Dioxus app.
- The checkpoint folder is on the owner's PC, where Sutradhar runs, not
  on a site server.
- tijori holds the key on the owner's PC, not on Dell.

Source: `docs/specifications/operations/audit-proof.md` in the archive
`archive-2026-11-08-old-repo`.
