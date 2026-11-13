//! Tests for the signing core's sign step (spec 0065 "Signing").
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use std::collections::BTreeMap;

use ed25519_compact::{KeyPair, Seed};
use sutradhar_tijori::check::{KeyEntry, KeyState, Signed, check};
use sutradhar_tijori::purpose::Purpose;
use tijori::core::{Core, SignError};
use tijori::log::{self, Kind};

const POLICY: [u8; 32] = [8; 32];

fn pair(n: u8) -> KeyPair {
    KeyPair::from_seed(Seed::new([n; 32]))
}

fn entry(principal: &str, purpose: &str, n: u8, state: KeyState) -> KeyEntry {
    let mut purposes = BTreeMap::new();
    let _ = purposes.insert(purpose.to_owned(), vec!["scope".to_owned()]);
    KeyEntry {
        public_key: *pair(n).pk,
        principal: principal.to_owned(),
        purposes,
        state,
    }
}

/// Runner "k1" (runner-result); agent "k2" (review-record), "k3" (release).
fn core() -> Core {
    let mut core = Core::new(POLICY, 64);
    core.bind_account("runner-acct", "runner");
    core.bind_account("agent-acct", "agent");
    core.add_key(
        "k1",
        entry("runner", "runner-result", 1, KeyState::Active),
        pair(1),
    );
    core.add_key(
        "k2",
        entry("agent", "review-record", 2, KeyState::Active),
        pair(2),
    );
    core.add_key(
        "k3",
        entry("agent", "release-record", 3, KeyState::Active),
        pair(3),
    );
    core
}

fn purpose(name: &str) -> Purpose {
    Purpose::new(name, 64).unwrap()
}

#[test]
fn a_signature_checks_under_the_policy_and_is_logged() {
    let mut core = core();
    let signed = core
        .sign("agent-acct", "review-record", b"record", 10)
        .unwrap();
    let bytes = purpose("review-record").prefix(b"record");
    let request = Signed {
        key_id: &signed.key_id,
        purpose: &purpose("review-record"),
        bytes: &bytes,
        signature: &signed.signature,
    };
    assert_eq!(
        check(&core.trust(), &request, &POLICY).unwrap().principal,
        "agent"
    );
    let entries = core.log();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries.first().unwrap().kind, Kind::Signature);
    assert_eq!(log::check(entries).unwrap().map(|h| h.counter), Some(1));
}

#[test]
fn an_unknown_account_is_refused() {
    let mut core = core();
    assert_eq!(
        core.sign("stranger", "review-record", b"x", 1),
        Err(SignError::UnknownCaller)
    );
}

#[test]
fn a_purpose_the_callers_principal_may_not_ask_for_is_refused() {
    let mut core = core();
    assert_eq!(
        core.sign("agent-acct", "runner-result", b"x", 1),
        Err(SignError::NoActiveKey)
    );
}

#[test]
fn purposes_other_steps_sign_are_refused_by_sign() {
    let mut core = core();
    for name in [
        "release-record",
        "release-manifest",
        "log-head",
        "head-confirmation",
        "waiver",
        "policy-change",
        "policy-record-reset",
        "site-key-record-reset",
        "site-key-change",
        "recovery",
        "site-message",
    ] {
        core.add_key(name, entry("agent", name, 9, KeyState::Active), pair(9));
        assert_eq!(
            core.sign("agent-acct", name, b"x", 1),
            Err(SignError::NotThisStep)
        );
    }
    assert_eq!(core.log().len(), 0);
}

#[test]
fn a_purpose_name_that_breaks_the_rule_is_refused() {
    let mut core = core();
    assert_eq!(
        core.sign("agent-acct", "Review-Record", b"x", 1),
        Err(SignError::BadPurpose)
    );
    assert_eq!(core.log().len(), 0);
}

#[test]
fn a_repeat_from_a_caller_that_lost_its_key_is_refused() {
    let mut core = core();
    let _ = core
        .sign("runner-acct", "runner-result", b"run", 1)
        .unwrap();
    core.set_state("k1", KeyState::Revoked);
    assert_eq!(
        core.sign("runner-acct", "runner-result", b"run", 2),
        Err(SignError::NoActiveKey)
    );
}

#[test]
fn a_repeat_returns_the_logged_signature_and_writes_nothing() {
    let mut core = core();
    let first = core
        .sign("agent-acct", "review-record", b"same", 1)
        .unwrap();
    let again = core
        .sign("agent-acct", "review-record", b"same", 2)
        .unwrap();
    assert_eq!(first, again);
    assert_eq!(core.log().len(), 1);
    let other = core.sign("agent-acct", "review-record", b"other", 3);
    assert_ne!(other.unwrap(), first);
    assert_eq!(core.log().len(), 2);
}

#[test]
fn a_repeat_after_the_key_is_retired_still_returns_the_logged_signature() {
    let mut core = core();
    let first = core
        .sign("agent-acct", "review-record", b"same", 1)
        .unwrap();
    core.set_state("k2", KeyState::Retired);
    core.add_key(
        "k4",
        entry("agent", "review-record", 4, KeyState::Active),
        pair(4),
    );
    assert_eq!(
        core.sign("agent-acct", "review-record", b"same", 2)
            .unwrap(),
        first
    );
}

#[test]
fn a_repeat_after_the_key_is_revoked_is_signed_afresh_with_the_active_key() {
    let mut core = core();
    let first = core
        .sign("agent-acct", "review-record", b"same", 1)
        .unwrap();
    core.set_state("k2", KeyState::Revoked);
    core.add_key(
        "k4",
        entry("agent", "review-record", 4, KeyState::Active),
        pair(4),
    );
    let again = core
        .sign("agent-acct", "review-record", b"same", 2)
        .unwrap();
    assert_ne!(again, first);
    assert_eq!(again.key_id, "k4");
    assert_eq!(core.log().len(), 2);
    let third = core
        .sign("agent-acct", "review-record", b"same", 3)
        .unwrap();
    assert_eq!(third, again);
    assert_eq!(core.log().len(), 2);
}

#[test]
fn a_runner_repeat_after_revocation_is_refused() {
    for name in ["runner-result", "evidence-bundle", "review-result"] {
        let mut core = Core::new(POLICY, 64);
        core.bind_account("runner-acct", "runner");
        core.add_key("k1", entry("runner", name, 1, KeyState::Active), pair(1));
        let _ = core.sign("runner-acct", name, b"run", 1).unwrap();
        core.set_state("k1", KeyState::Revoked);
        core.add_key("k5", entry("runner", name, 5, KeyState::Active), pair(5));
        assert_eq!(
            core.sign("runner-acct", name, b"run", 2),
            Err(SignError::RevokedRepeat)
        );
        assert_eq!(core.log().len(), 1);
    }
}

/// A policy change moves "k2" off the logged request and "k4" onto it.
#[test]
fn a_repeat_whose_key_no_longer_fits_is_signed_afresh() {
    for (principal, purpose) in
        [("agent", "other-record"), ("someone-else", "review-record")]
    {
        let mut core = core();
        let first = core
            .sign("agent-acct", "review-record", b"same", 1)
            .unwrap();
        core.add_key(
            "k2",
            entry(principal, purpose, 2, KeyState::Active),
            pair(2),
        );
        core.add_key(
            "k4",
            entry("agent", "review-record", 4, KeyState::Active),
            pair(4),
        );
        let again = core
            .sign("agent-acct", "review-record", b"same", 2)
            .unwrap();
        assert_ne!(again, first);
        assert_eq!(again.key_id, "k4");
    }
}

#[test]
fn the_same_bytes_from_another_principal_are_not_a_repeat() {
    let mut core = core();
    core.bind_account("agent2-acct", "agent2");
    core.add_key(
        "k6",
        entry("agent2", "review-record", 6, KeyState::Active),
        pair(6),
    );
    let a = core
        .sign("agent-acct", "review-record", b"same", 1)
        .unwrap();
    let b = core
        .sign("agent2-acct", "review-record", b"same", 2)
        .unwrap();
    assert_ne!(a.key_id, b.key_id);
    assert_eq!(core.log().len(), 2);
}

#[test]
fn bytes_over_the_size_limit_are_refused_before_signing() {
    let mut core = Core::new(POLICY, 4);
    core.bind_account("agent-acct", "agent");
    core.add_key(
        "k2",
        entry("agent", "review-record", 2, KeyState::Active),
        pair(2),
    );
    assert_eq!(
        core.sign("agent-acct", "review-record", b"12345", 1),
        Err(SignError::TooLarge { limit: 4 })
    );
    assert_eq!(core.log().len(), 0);
    assert!(core.sign("agent-acct", "review-record", b"1234", 2).is_ok());
}

#[test]
fn every_error_has_a_message() {
    for error in [
        SignError::UnknownCaller,
        SignError::NoActiveKey,
        SignError::NotThisStep,
        SignError::RevokedRepeat,
        SignError::TooLarge { limit: 4 },
        SignError::BadPurpose,
        SignError::LogFull,
        SignError::OutOfStep,
    ] {
        assert_ne!(error.to_string(), "");
    }
}
