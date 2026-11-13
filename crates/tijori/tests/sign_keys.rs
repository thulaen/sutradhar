//! Tests for which key the sign step picks (spec 0065 "Signing" item 2
//! and "Release guards" item 1).
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use std::collections::BTreeMap;

use ed25519_compact::{KeyPair, Seed};
use sutradhar_tijori::check::{KeyEntry, KeyState};
use tijori::core::{Core, SignError};

/// A core where "agent-acct" holds key "k1" for every one of `purposes`.
fn holding(purposes: &[&str]) -> Core {
    let pair = KeyPair::from_seed(Seed::new([1; 32]));
    let listed: BTreeMap<String, Vec<String>> = purposes
        .iter()
        .map(|p| ((*p).to_owned(), vec!["scope".to_owned()]))
        .collect();
    let mut core = Core::new([8; 32], 64);
    core.bind_account("agent-acct", "agent");
    core.add_key(
        "k1",
        KeyEntry {
            public_key: *pair.pk,
            principal: "agent".to_owned(),
            purposes: listed,
            state: KeyState::Active,
        },
        pair,
    );
    core
}

#[test]
fn the_release_key_never_signs_through_sign() {
    for release in ["release-record", "release-manifest"] {
        let mut core = holding(&[release, "review-record"]);
        assert_eq!(
            core.sign("agent-acct", "review-record", b"x", 1),
            Err(SignError::NoActiveKey)
        );
        assert_eq!(core.log().len(), 0);
    }
}

#[test]
fn the_same_bytes_for_another_purpose_are_not_a_repeat() {
    let mut core = holding(&["review-record", "gate-run"]);
    let first = core
        .sign("agent-acct", "review-record", b"same", 1)
        .unwrap();
    let other = core.sign("agent-acct", "gate-run", b"same", 2).unwrap();
    assert_ne!(first, other);
    assert_eq!(core.log().len(), 2);
}

#[test]
fn a_retired_key_never_signs_new_bytes() {
    let mut core = holding(&["review-record"]);
    core.set_state("k1", KeyState::Retired);
    assert_eq!(
        core.sign("agent-acct", "review-record", b"new", 1),
        Err(SignError::NoActiveKey)
    );
    assert_eq!(core.log().len(), 0);
    core.set_state("k1", KeyState::Active);
    assert!(core.sign("agent-acct", "review-record", b"new", 2).is_ok());
}
