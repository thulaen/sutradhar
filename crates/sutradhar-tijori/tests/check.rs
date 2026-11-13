//! Tests for the signature check (spec 0065 "Checking signatures").
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use std::collections::{BTreeMap, HashMap};

use sutradhar_tijori::check::{
    CheckError, Checked, KeyEntry, KeyState, Signed, Trust, check,
};
use sutradhar_tijori::purpose::{Purpose, PurposeError};
use sutradhar_tijori::verify::VerifyError;

const POLICY: [u8; 32] = [9; 32];

// RFC 8032 section 7.1, test 1: an empty message.
const KEY: [u8; 32] = [
    0xd7, 0x5a, 0x98, 0x01, 0x82, 0xb1, 0x0a, 0xb7, 0xd5, 0x4b, 0xfe, 0xd3,
    0xc9, 0x64, 0x07, 0x3a, 0x0e, 0xe1, 0x72, 0xf3, 0xda, 0xa6, 0x23, 0x25,
    0xaf, 0x02, 0x1a, 0x68, 0xf7, 0x07, 0x51, 0x1a,
];
const SIG: [u8; 64] = [
    0xe5, 0x56, 0x43, 0x00, 0xc3, 0x60, 0xac, 0x72, 0x90, 0x86, 0xe2, 0xcc,
    0x80, 0x6e, 0x82, 0x8a, 0x84, 0x87, 0x7f, 0x1e, 0xb8, 0xe5, 0xd9, 0x74,
    0xd8, 0x73, 0xe0, 0x65, 0x22, 0x49, 0x01, 0x55, 0x5f, 0xb8, 0x82, 0x15,
    0x90, 0xa3, 0x3b, 0xac, 0xc6, 0x1e, 0x39, 0x70, 0x1c, 0xf9, 0xb4, 0x6b,
    0xd2, 0x5b, 0xf5, 0xf0, 0x59, 0x5b, 0xbe, 0x24, 0x65, 0x51, 0x41, 0x43,
    0x8e, 0x7a, 0x10, 0x0b,
];

struct Fake {
    digest: Option<[u8; 32]>,
    keys: HashMap<String, KeyEntry>,
}

impl Trust for Fake {
    fn lookup(&self, key_id: &str) -> Option<([u8; 32], Option<KeyEntry>)> {
        self.digest.map(|d| (d, self.keys.get(key_id).cloned()))
    }
}

/// A trust source whose policy changes on every call, as a reload in
/// the middle of a check would make it.
struct Reloading {
    calls: std::cell::Cell<u32>,
    newer: Fake,
}

impl Trust for Reloading {
    fn lookup(&self, key_id: &str) -> Option<([u8; 32], Option<KeyEntry>)> {
        let n = self.calls.get();
        self.calls.set(n.saturating_add(1));
        if n == 0 {
            // The first answer is the newer policy, whose digest the
            // caller does not expect; it carries that policy's key.
            self.newer.lookup(key_id).map(|(_, key)| ([7; 32], key))
        } else {
            Some((POLICY, None))
        }
    }
}

#[test]
fn a_reload_during_a_check_cannot_mix_two_policies() {
    let reloading = Reloading {
        calls: std::cell::Cell::new(0),
        newer: fake(KeyState::Active, "waiver"),
    };
    let wanted = purpose("waiver");
    let bytes = wanted.prefix(b"body");
    let sig = sign(&bytes);
    let signed = Signed {
        key_id: "k1",
        purpose: &wanted,
        bytes: &bytes,
        signature: &sig,
    };
    assert_eq!(
        check(&reloading, &signed, &POLICY),
        Err(CheckError::PolicyNotInForce)
    );
    assert_eq!(reloading.calls.get(), 1, "the check asked twice");
}

/// A fake whose key "k1" is allowed `purpose`. The RFC vector signs an
/// empty message, so the purpose check is driven with a purpose whose
/// field the test builds by hand below.
fn fake(state: KeyState, purpose: &str) -> Fake {
    let mut purposes = BTreeMap::new();
    let _ = purposes.insert(purpose.to_owned(), vec!["scope-a".to_owned()]);
    let mut keys = HashMap::new();
    let _ = keys.insert(
        "k1".to_owned(),
        KeyEntry {
            public_key: KEY,
            principal: "runner".to_owned(),
            purposes,
            state,
        },
    );
    Fake {
        digest: Some(POLICY),
        keys,
    }
}

fn purpose(name: &str) -> Purpose {
    Purpose::new(name, 32).unwrap()
}

/// Checks `bytes` signed by `sig` as key "k1" for `name`, against the
/// policy the fake has in force.
fn run(
    trust: &Fake,
    name: &str,
    bytes: &[u8],
    sig: &[u8; 64],
) -> Result<Checked, CheckError> {
    let wanted = purpose(name);
    let signed = Signed {
        key_id: "k1",
        purpose: &wanted,
        bytes,
        signature: sig,
    };
    check(trust, &signed, &POLICY)
}

#[test]
fn refuses_every_check_when_the_policy_cannot_be_read() {
    let mut trust = fake(KeyState::Active, "waiver");
    trust.digest = None;
    assert_eq!(run(&trust, "waiver", b"", &SIG), Err(CheckError::NoPolicy));
}

#[test]
fn refuses_a_caller_that_expects_another_policy() {
    let trust = fake(KeyState::Active, "waiver");
    assert_eq!(
        check(
            &trust,
            &Signed {
                key_id: "k1",
                purpose: &purpose("waiver"),
                bytes: b"",
                signature: &SIG,
            },
            &[1; 32],
        ),
        Err(CheckError::PolicyNotInForce)
    );
}

#[test]
fn refuses_an_unknown_key() {
    let trust = fake(KeyState::Active, "waiver");
    assert_eq!(
        check(
            &trust,
            &Signed {
                key_id: "k2",
                purpose: &purpose("waiver"),
                bytes: b"",
                signature: &SIG,
            },
            &POLICY,
        ),
        Err(CheckError::UnknownKey)
    );
}

#[test]
fn refuses_a_revoked_key_before_anything_else_about_it() {
    let trust = fake(KeyState::Revoked, "waiver");
    assert_eq!(run(&trust, "waiver", b"", &SIG), Err(CheckError::Revoked));
}

#[test]
fn refuses_a_purpose_the_key_may_not_sign_for() {
    // The bytes and signature are good for runner-result, so only the
    // registry's purpose list can refuse them.
    let trust = fake(KeyState::Active, "waiver");
    let bytes = purpose("runner-result").prefix(b"body");
    assert_eq!(
        run(&trust, "runner-result", &bytes, &sign(&bytes)),
        Err(CheckError::WrongPurpose)
    );
}

#[test]
fn refuses_bytes_whose_purpose_field_is_another_purpose() {
    let trust = fake(KeyState::Active, "waiver");
    let signed = purpose("gate-run").prefix(b"");
    assert_eq!(
        run(&trust, "waiver", &signed, &SIG),
        Err(CheckError::BadPurposeField(PurposeError::WrongPurpose {
            expected: "waiver".to_owned()
        }))
    );
}

#[test]
fn refuses_cut_short_bytes_with_the_real_reason() {
    let trust = fake(KeyState::Active, "waiver");
    assert_eq!(
        run(&trust, "waiver", &[0, 0, 0], &SIG),
        Err(CheckError::BadPurposeField(PurposeError::Truncated))
    );
}

#[test]
fn refuses_a_bad_signature_with_its_reason() {
    let trust = fake(KeyState::Active, "waiver");
    let signed = purpose("waiver").prefix(b"");
    assert_eq!(
        run(&trust, "waiver", &signed, &SIG),
        Err(CheckError::BadSignature(VerifyError::BadSignature))
    );
}

// RFC 8032 section 7.1, test 1: the secret key whose public half is KEY.
const SECRET: [u8; 32] = [
    0x9d, 0x61, 0xb1, 0x9d, 0xef, 0xfd, 0x5a, 0x60, 0xba, 0x84, 0x4a, 0xf4,
    0x92, 0xec, 0x2c, 0xc4, 0x44, 0x49, 0xc5, 0x69, 0x7b, 0x32, 0x69, 0x19,
    0x70, 0x3b, 0xac, 0x03, 0x1c, 0xae, 0x7f, 0x60,
];

fn sign(bytes: &[u8]) -> [u8; 64] {
    use ed25519_compact::{KeyPair, Seed};
    let pair = KeyPair::from_seed(Seed::new(SECRET));
    assert_eq!(*pair.pk, KEY);
    *pair.sk.sign(bytes, None)
}

#[test]
fn a_passing_check_returns_the_principal_and_its_scopes() {
    let trust = fake(KeyState::Active, "waiver");
    let signed = purpose("waiver").prefix(b"waiver body");
    assert_eq!(
        run(&trust, "waiver", &signed, &sign(&signed)),
        Ok(Checked {
            principal: "runner".to_owned(),
            scopes: vec!["scope-a".to_owned()],
        })
    );
}

#[test]
fn a_retired_key_still_passes() {
    let trust = fake(KeyState::Retired, "waiver");
    let signed = purpose("waiver").prefix(b"old body");
    assert!(run(&trust, "waiver", &signed, &sign(&signed)).is_ok());
}

#[test]
fn a_good_signature_under_a_revoked_key_is_refused() {
    let trust = fake(KeyState::Revoked, "waiver");
    let signed = purpose("waiver").prefix(b"body");
    assert_eq!(
        run(&trust, "waiver", &signed, &sign(&signed)),
        Err(CheckError::Revoked)
    );
}

#[test]
fn a_signature_moved_to_other_bytes_is_refused() {
    let trust = fake(KeyState::Active, "waiver");
    let signed = purpose("waiver").prefix(b"body");
    let other = purpose("waiver").prefix(b"bodY");
    assert_eq!(
        run(&trust, "waiver", &other, &sign(&signed)),
        Err(CheckError::BadSignature(VerifyError::BadSignature))
    );
}

#[test]
fn every_error_explains_itself_in_plain_english() {
    for error in [
        CheckError::NoPolicy,
        CheckError::PolicyNotInForce,
        CheckError::UnknownKey,
        CheckError::Revoked,
        CheckError::WrongPurpose,
        CheckError::BadPurposeField(PurposeError::Truncated),
        CheckError::BadSignature(VerifyError::WeakKey),
    ] {
        assert_ne!(error.to_string(), "");
    }
}
