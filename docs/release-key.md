# The release signing key

The owner told agents on 22 November 2026 to make the release signing
key (DR-166). This page holds only what is public: its fingerprint,
its public key, where the private part lives and the rules for using it.

## The key

| Item | Value |
|---|---|
| Kind | OpenPGP, ed25519, signing only |
| Name | Sutradhar release signing key |
| Fingerprint | `504D 86DE 92CA 57CA BF5C  1B75 0FC1 E5A0 1058 01D9` |
| Made | 22 November 2026 |
| Expires | 21 November 2028; a new key is made before that |

The public key, to check a signature:

```
-----BEGIN PGP PUBLIC KEY BLOCK-----

mDMEawKXNxYJKwYBBAHaRw8BAQdADcc06D/V6KvO6cz95lndMURbvrPnMZ2Q+tjE
dSK9Tw+0HVN1dHJhZGhhciByZWxlYXNlIHNpZ25pbmcga2V5iJYEExYKAD4WIQRQ
TYbekspXyr9cG3UPweWgEFgB2QUCawKXNwIbAwUJA8JnAAULCQgHAgYVCgkICwIE
FgIDAQIeAQIXgAAKCRAPweWgEFgB2eLlAP9SYbimhMDEpQVLT56p2zJv2beQJhSh
eLtVEDVtZf+ukQEAsAWmTAyqFtQwfSi7ANydtMz6vsOXwgD0j00iDEK7PgY=
=bqXC
-----END PGP PUBLIC KEY BLOCK-----
```

## Where the private part lives

- The private key is in a folder on Dell that only the owner's account
  can read, and it is encrypted with a passphrase. The passphrase is in
  a separate file only the owner's account can read. Neither is in the
  repository, a log, a backup that leaves Dell or any message.
- Agents have sudo on Dell, so a session under the shared account can
  read both. That is a known limit (AGENTS.md section 11). The key moves
  to the agent harness's store, or a locked account, when those exist
  (spec 0155).

## Rules for using it

These are required of the release step (AGENTS.md section 8.1 step 4):

1. Only the release step signs with the key. It signs the artefacts of
   one exact approved candidate and nothing else.
2. Every use is logged and the log is copied to GitHub. A use with no
   log line is a broken rule.
3. It signs nothing without the owner's signed approval of that exact
   candidate (spec 0005).

Also:

4. The chaukidar watch list holds the key's fingerprint, so a changed
   or replaced key raises the alarm.
5. A revocation certificate was made with the key and is kept beside
   the private key. If the key may have leaked, the owner revokes it
   with that certificate and a new key is made.
6. Spec 0065 describes a release-key authority that signs through a
   core caller. It is not reconciled with this key; the catalogue's
   known gap row for the release key holds that work.
