//! Tests for accepting a policy (spec 0065 "Checking signatures" item 3).
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use std::collections::BTreeMap;

use ed25519_compact::{KeyPair, Seed};
use sha2::{Digest, Sha256};
use sutradhar_tijori::check::{CheckError, KeyEntry, KeyState, Signed, check};
use sutradhar_tijori::policy::{
    Policy, PolicyError, PolicyTrust, Record, accept,
};
use sutradhar_tijori::purpose::{Purpose, PurposeError};

const DELL: &str = "dell";

fn owner() -> KeyPair {
    KeyPair::from_seed(Seed::new([3; 32]))
}

/// A policy body for `version`, signed by `pair` for policy-change.
fn signed(version: u64, instance: &str, pair: &KeyPair) -> (Vec<u8>, [u8; 64]) {
    let body = format!("{instance}|{version}");
    let bytes = Purpose::new("policy-change", 32)
        .unwrap()
        .prefix(body.as_bytes());
    let signature = *pair.sk.sign(&bytes, None);
    (bytes, signature)
}

fn policy<'a>(
    version: u64,
    instance: &'a str,
    bytes: &'a [u8],
    sig: &'a [u8; 64],
) -> Policy<'a> {
    Policy {
        instance,
        version,
        signed: bytes,
        signature: sig,
    }
}

fn owner_key() -> [u8; 32] {
    *owner().pk
}

#[test]
fn a_higher_version_signed_by_the_owner_is_accepted() {
    let (bytes, sig) = signed(2, DELL, &owner());
    let record = Record::new(1, [9; 32]);
    let accepted = accept(
        DELL,
        Some(record),
        &policy(2, DELL, &bytes, &sig),
        &owner_key(),
    );
    let accepted = accepted.unwrap();
    assert_eq!(accepted.version(), 2);
    let exact: [u8; 32] = Sha256::digest(&bytes).into();
    assert_eq!(
        accepted.digest(),
        exact,
        "the digest is not of the exact signed bytes"
    );
}

#[test]
fn the_same_version_is_accepted_only_with_the_same_bytes() {
    let (bytes, sig) = signed(2, DELL, &owner());
    let first = accept(
        DELL,
        Some(Record::new(1, [0; 32])),
        &policy(2, DELL, &bytes, &sig),
        &owner_key(),
    )
    .unwrap();
    assert_eq!(
        accept(
            DELL,
            Some(first),
            &policy(2, DELL, &bytes, &sig),
            &owner_key()
        ),
        Ok(first)
    );
    // Any other bytes at the same version will do; a changed instance
    // name is the simplest way to make them.
    let (other, other_sig) = signed(2, "dell ", &owner());
    assert_eq!(
        accept(
            DELL,
            Some(first),
            &policy(2, DELL, &other, &other_sig),
            &owner_key()
        ),
        Err(PolicyError::SameVersionOtherBytes)
    );
}

#[test]
fn a_lower_version_is_refused_even_when_signed() {
    let (bytes, sig) = signed(1, DELL, &owner());
    assert_eq!(
        accept(
            DELL,
            Some(Record::new(2, [0; 32])),
            &policy(1, DELL, &bytes, &sig),
            &owner_key()
        ),
        Err(PolicyError::Older { in_force: 2 })
    );
}

#[test]
fn a_policy_for_another_instance_is_refused() {
    let (bytes, sig) = signed(2, "owner-pc", &owner());
    assert_eq!(
        accept(
            DELL,
            Some(Record::new(1, [0; 32])),
            &policy(2, "owner-pc", &bytes, &sig),
            &owner_key()
        ),
        Err(PolicyError::OtherInstance)
    );
}

#[test]
fn a_policy_not_signed_by_the_owner_is_refused() {
    let stranger = KeyPair::from_seed(Seed::new([4; 32]));
    let (bytes, sig) = signed(2, DELL, &stranger);
    assert!(matches!(
        accept(
            DELL,
            Some(Record::new(1, [0; 32])),
            &policy(2, DELL, &bytes, &sig),
            &owner_key()
        ),
        Err(PolicyError::NotOwnerSigned(_))
    ));
}

#[test]
fn a_policy_signed_for_another_purpose_is_refused() {
    let pair = owner();
    let bytes = Purpose::new("waiver", 32).unwrap().prefix(b"dell|2");
    let sig = *pair.sk.sign(&bytes, None);
    assert_eq!(
        accept(
            DELL,
            Some(Record::new(1, [0; 32])),
            &policy(2, DELL, &bytes, &sig),
            &owner_key()
        ),
        Err(PolicyError::NotPolicyChange(PurposeError::WrongPurpose {
            expected: "policy-change".to_owned()
        }))
    );
}

#[test]
fn a_missing_version_record_refuses_every_policy() {
    let (bytes, sig) = signed(2, DELL, &owner());
    assert_eq!(
        accept(DELL, None, &policy(2, DELL, &bytes, &sig), &owner_key()),
        Err(PolicyError::NoRecord)
    );
}

#[test]
fn the_policy_trust_source_answers_checks_from_one_policy() {
    let signer = KeyPair::from_seed(Seed::new([5; 32]));
    let mut purposes = BTreeMap::new();
    let _ = purposes.insert("waiver".to_owned(), vec!["all".to_owned()]);
    let trust = PolicyTrust::new(
        [8; 32],
        [(
            "k1".to_owned(),
            KeyEntry {
                public_key: *signer.pk,
                principal: "owner-tool".to_owned(),
                purposes,
                state: KeyState::Active,
            },
        )]
        .into_iter()
        .collect(),
    );
    let wanted = Purpose::new("waiver", 32).unwrap();
    let bytes = wanted.prefix(b"waive it");
    let sig = *signer.sk.sign(&bytes, None);
    let mut request = Signed {
        key_id: "k1",
        purpose: &wanted,
        bytes: &bytes,
        signature: &sig,
    };
    assert_eq!(
        check(&trust, &request, &[8; 32]).unwrap().principal,
        "owner-tool"
    );
    assert_eq!(
        check(&trust, &request, &[1; 32]),
        Err(CheckError::PolicyNotInForce)
    );
    request.key_id = "k2";
    assert_eq!(
        check(&trust, &request, &[8; 32]),
        Err(CheckError::UnknownKey)
    );
}

#[test]
fn every_error_has_a_message() {
    for error in [
        PolicyError::NoRecord,
        PolicyError::OtherInstance,
        PolicyError::Older { in_force: 2 },
        PolicyError::SameVersionOtherBytes,
        PolicyError::NotPolicyChange(PurposeError::Truncated),
        PolicyError::NotOwnerSigned(
            sutradhar_tijori::verify::VerifyError::BadSignature,
        ),
    ] {
        assert_ne!(error.to_string(), "");
    }
}

#[test]
fn a_mislabelled_policy_shows_why_the_caller_must_decode_from_the_bytes() {
    // The owner signed version 1 for another instance. If a caller passed
    // a made-up instance and version instead of decoding them from the
    // signed bytes, accept could not tell. This pins that contract: the
    // decoding has to come from the signed bytes until accept does it.
    let (bytes, sig) = signed(1, "owner-pc", &owner());
    let relabelled = policy(9, DELL, &bytes, &sig);
    let record = Record::new(5, [0; 32]);
    assert_eq!(
        accept(DELL, Some(record), &relabelled, &owner_key())
            .map(|r| r.version()),
        Ok(9)
    );
}
