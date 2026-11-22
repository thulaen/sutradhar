# Synthetic release-encoding golden fixtures

Use these vectors for candidate
[C](../specs/0005-release-coordination.md#exact-candidate-manifest-and-record),
its manifest and release record in spec 0005.
Whitespace in hex blocks is for display. The strings below are ASCII
and `LF` is one byte `0a`. No real credentials, keys, signatures,
archive parser qualification, release proof or owner decision is
represented. `manual.zip` and `installer.exe` are invented identity
labels. The short byte strings represent outputs of an abstract
standalone reader and member reader, not usable archive or executable
files. A production verifier cannot accept these fixture IDs or
synthetic authority records.

## Inputs

The input enumeration is payload, notice, manual, app. Canonical output
sorts by part-key tuple, yielding app, manual, notice, payload.

| Role | Target | Part name | Location | Exact bytes |
| --- | --- | --- | --- | --- |
| `app` | `FX-WIN` | `installer.exe` | Standalone | `outer` + LF |
| `manual` | `FX-ALL` | `manual.zip` | Standalone | `manual` + LF |
| `notice` | `FX-ALL` | `notice.md` | Standalone | `notice` + LF |
| `payload` | `FX-WIN` | `service.exe` | Member of `(app, FX-WIN, installer.exe)`, path `engine/service.exe` | `inner` + LF |

Repository is `FX-REPO`, task number is 42, run is `FX-RUN`,
commit text is `0123456789abcdef0123456789abcdef01234567`, candidate tag is
`v1.2.3-rc.1` and version text is `1.2.3`. Compute
the source-tree, protected-policy, registry and package-profile test
digests as SHA-256 of `FX-TREE`, `FX-POLICY`, `FX-SIGNERS` and
`FX-PROFILE`, each followed by LF. The notice digest hashes the
notice part bytes. These conventionally derived fixture digests do
not define the corresponding operational records.

Approval uses lease `FX-LEASE` and approver `FX-APPROVER`. Its
qualification and full-gate fixture digests hash `FX-QUALIFICATION`
and `FX-GATE`, respectively, each followed by LF. Its signed and
expiry UTC integers are 1700000000 and 1700000060. These times and
the synthetic identities convey no authority.

## Golden bytes and SHA-256

| Record or preimage | Byte length | SHA-256 |
| --- | ---: | --- |
| Inventory I | 498 | `d03d717ef8f74d80489c1d383417a582e8192ab5e08cde2cff020d6265154e0e` |
| Candidate C | 342 | `9c0176575b495a119f24966a414ec47632ce7bb043dd4bf085f9560104ce3545` |
| Manifest caller M | 933 | `55e773f6e993086915f8a48981f0bb23b809dfd9b4f2e946e241a38e25ed6a56` |
| Release-record caller R | 107 | `d2f23170c6cfda4a31a47d96f1036b45d24da9dbd03badf2ef2429adb7cd6136` |
| Approval caller A | 338 | `e59958c6a1c7b06ef21c23e88f490c89eac454ef1b5ee046cce4f8aaf4affa96` |
| Manifest signing preimage | 957 | `15db29d19588d314042cdb6eac529625fb64143dfb201d3978f4dbfa0fbcdaa5` |
| Release-record signing preimage | 129 | `6a2da2bc8956413038a5418edf9af034d77b01e79aa1ff51f174553db988d9f8` |
| Approval signing preimage | 362 | `a0812245b895bed0d8a9f2792d2a75050e96e6c8e0da465717115ed39c3d67c4` |

### Inventory I

```text
000000000000001972656c656173652d7061796c6f61642d696e76656e746f72
7900000000000000010000000000000004000000000000000361707000000000
0000000646582d57494e000000000000000d696e7374616c6c65722e65786500
0000000000000000000000000000068d1926b8a8ddca82688efe7f910affc6c5
adb84f8b16d30136169bf4abd0b4fb00000000000000066d616e75616c000000
000000000646582d414c4c000000000000000a6d616e75616c2e7a6970000000
00000000000000000000000007cd60ed2fa8c947cb05cb57b23884dea9390e4b
6aec0add666e69bf842fc68da000000000000000066e6f746963650000000000
00000646582d414c4c00000000000000096e6f746963652e6d64000000000000
000000000000000000075de37b4c620461f2b695f1cf6b8ee36e7d025da8334a
3693c4ed9aa48a5c7d6a00000000000000077061796c6f616400000000000000
0646582d57494e000000000000000b736572766963652e657865000000000000
00010000000000000003617070000000000000000646582d57494e0000000000
00000d696e7374616c6c65722e6578650000000000000012656e67696e652f73
6572766963652e6578650000000000000006940a68104d3b690442453f4be394
b0a14721a174127d84c1c2f834b7ad05d684
```

### Candidate C

```text
000000000000001172656c656173652d63616e64696461746500000000000000
01000000000000000746582d5245504f000000000000002a0000000000000006
46582d52554e0000000000000028303132333435363738396162636465663031
32333435363738396162636465663031323334353637b8863efa5bd376d5ab19
c4cfd9211c9b7c099911d82b311cb44a63b3835e4212000000000000000b7631
2e322e332d72632e310000000000000005312e322e33d03d717ef8f74d80489c
1d383417a582e8192ab5e08cde2cff020d6265154e0e5de37b4c620461f2b695
f1cf6b8ee36e7d025da8334a3693c4ed9aa48a5c7d6a54007f23e12a08537012
9cae2b1eaef4ffba81bd59249fd1a8a016b1984ef49cdc74d3161adaabd866ee
9e67b9c3140c2f042a1fa45d16b8e00c6590d4948df4864a9a369f2daa30aa3d
f87afdc8cdd1a2d0fb1e049763f24bedaf77ba738654
```

### Manifest caller M

Concatenate the following prefix, `U(342)`, exact C,
`U(498)` and exact I. The prefix includes the candidate digest
and ends immediately before the first nested length field.

```text
000000000000001072656c656173652d6d616e69666573740000000000000001
0000000000000005312e322e339c0176575b495a119f24966a414ec47632ce7b
b043dd4bf085f9560104ce3545
```

The two length fields are `0000000000000156` and
`00000000000001f2`.

### Release-record caller R

```text
000000000000000e72656c656173652d7265636f726400000000000000010000
000000000005312e322e339c0176575b495a119f24966a414ec47632ce7bb043
dd4bf085f9560104ce354555e773f6e993086915f8a48981f0bb23b809dfd9b4
f2e946e241a38e25ed6a56
```

### Approval caller A

```text
000000000000001a63616e6469646174652d72656c656173652d617070726f76
616c0000000000000001000000000000000746582d5245504f00000000000000
2a000000000000000846582d4c45415345000000000000000646582d52554e00
0000000000002830313233343536373839616263646566303132333435363738
396162636465663031323334353637000000000000000b76312e322e332d7263
2e310000000000000005312e322e33a2a9948774a37b8aa4578b6ce8e2457094
be562fdec6b2f8f99976f94e4a9dbda8c5c43c57f3939f616e26fdc621714c68
f740d52e3e79455d50d0144aabacf7d03d717ef8f74d80489c1d383417a582e8
192ab5e08cde2cff020d6265154e0e54007f23e12a085370129cae2b1eaef4ff
ba81bd59249fd1a8a016b1984ef49c000000000000000b46582d415050524f56
4552000000006553f100000000006553f13c
```

### Purpose fields

Prepend only the respective field below to the matching caller bytes.
The digest of the prefixed bytes is listed separately. Do not use it
as the typed manifest digest.

| Purpose | Exact encoded field |
| --- | --- |
| `release-manifest` | `000000000000001072656c656173652d6d616e6966657374` |
| `release-record` | `000000000000000e72656c656173652d7265636f7264` |
| `release-approval` | `000000000000001072656c656173652d617070726f76616c` |

## Inventory sort discriminator

Use a separate synthetic inventory codec fixture with two standalone
rows. It does not represent a complete release inventory.

| Role | Target | Part name | Exact standalone bytes |
| --- | --- | --- | --- |
| `manual` | `FX-Z` | `z.zip` | `z` + LF |
| `manual` | `FX-AA` | `aa.zip` | `aa` + LF |

Supply the rows in both orders. The encoder must emit `FX-AA` then
`FX-Z`. Its inventory is 225 bytes and hashes to
`47293f37b579a2352db1f882229843a9d965ad7cf85fb80cc13c2fb249644a18`.
Both targets are exact fixture IDs. Sorting length-prefixed fields
would instead emit `FX-Z` then `FX-AA`. Supply that order on the
wire with the same valid entries and byte hashes. Its inventory
hash is
`80983b69ff5a2407606e03faaaa5217c911461c77e55d94866efc6a5d7de04a3`.
The inventory reader refuses the latter order. No packaging profile
or release effect is authorised by this codec check.

## Planned negative checks

- Keep the original candidate record type and version. Remove the
  entire 32-byte notice digest from C. Leave the later policy,
  signer registry and packaging digest bytes intact and ordered.
  The candidate reader refuses the missing field. Valid fixture
  hashes in those later fields cannot complete C.

- Reordering input enumeration must still produce I. If the writer
  sends the first two canonical entries swapped on the wire, the
  reader refuses even when each entry and its byte digest match.
- Use the four-row inventory with a test-only count limit of four.
  It passes that bound. Supply five complete entries in canonical
  order by adding the standalone key `(manual, FX-Z, z.zip)` with
  bytes `z` + LF. All five keys are available to the codec fixture.
  Refuse the list with count five before allocating or recursing.
  In a separate isolated scalar case allow five text bytes,
  declare five and supply only the four bytes `FX-A`. Refuse
  before allocating for the text. Repeat with all five bytes
  `FX-AA` as the control. The test-only limits set no release
  profile limits.
- Keep the inventory record type and complete fields as shown.
  Change only its format version from `U(1)` to `U(2)`. The
  record reader refuses the version without decoding its entries.
- For the text codec, use `T` fields with body bytes `c3 a9` and
  `65 cc 81`. The decoder preserves each exact UTF-8 byte
  sequence and the writer emits the different lengths `U(2)` and
  `U(3)`. A field with body bytes `c3 28` and length `U(2)` is
  refused without replacement. Separately use the original
  fixture profile and change the target `FX-ALL` to `fx-all` in
  one complete row. Recompute the record digest and keep the exact
  actual bytes. No case conversion can admit the changed ID.
- Replace the member's `U(1)` with an unknown tag, replace its parent
  with the manual key, duplicate its member path, or alter the
  parent's raw bytes. Each mismatch refuses under the synthetic
  profile. The payload's direct byte hash alone cannot pass.
- Change one byte in I inside M, then separately change only M's
  outer candidate digest or C's notice digest. Refuse the mismatch.
  A new digest alone cannot repair stale proof or approval.
- Compare `H(M)` with the manifest preimage digest. The second
  hash must fail when supplied in the typed manifest digest field.
  Swap the purpose fields for M and R, or append a trailing byte.
  Reject at the exact purpose or framing boundary.
- Present a signed fixture result that binds a protection-inventory
  digest but omits the exact payload digest, or replace the approval
  payload digest with the protection-inventory digest. Hold before
  approval. Missing profile, owner decisions or envelope rules
  remain outside these byte fixtures and block the real route.


### Exact member parent and inventory identity

Use the existing synthetic codec fixtures, with a test-only bound
that admits the number of complete entries in each case. Reset to
the original inventory before each variation.

- Change only the `app` row's location to a member of the standalone
  `manual` row, at `nested/installer.exe`. Keep its key, length and
  digest. The `payload` row still refers to the same `app` key as
  its parent. The references have no cycle. Refuse because that key
  now resolves to a member row. Observe no recursive parent lookup
  or archive inspection on this structural fault.
- Keep I unchanged and provide its matching parent bytes through
  the abstract fixture reader. The reader has no member at
  `engine/service.exe`. The same bytes are available at
  `engine/service-copy.exe`. The lookup of the exact approved path
  fails. Refuse without trying the alternate path or granting an
  effect. Supply the same bytes at the exact approved path in the
  control. These abstract reads give no native archive proof.
- Duplicate the complete standalone `manual` row and update the
  count. Its byte length and digest still match. Refuse its
  duplicate part key even in otherwise sorted input.
- In a separate branch, add `(payload, FX-WIN, service-copy.exe)`
  as a second part key pointing to the existing parent and exact
  `engine/service.exe` member path, with the same byte length and
  digest. Set the correct count and order. The part keys differ.
  Refuse the repeated parent and path reference before archive use.
