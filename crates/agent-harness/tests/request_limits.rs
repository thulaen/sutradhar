//! REQ-0155-LIMITS: every request limit, at, below and above it.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

mod common;

use agent_harness::request::{MAX_REQUEST_BYTES, Refusal, parse};
use common::{ID, check_write, comment, make, raw, tag};
use serde_json::json;

const KIB: usize = 1024;

#[test]
fn the_title_limit_is_200_bytes() {
    let title = |n: usize| raw(&make(&"t".repeat(n)));
    assert!(title(199).is_ok() && title(200).is_ok());
    assert!(title(201).is_err());
}

#[test]
fn the_body_limit_is_32_kib() {
    let body = |n: usize| raw(&comment(&"b".repeat(n)));
    assert!(body(32 * KIB - 1).is_ok() && body(32 * KIB).is_ok());
    assert!(body(32 * KIB + 1).is_err());
}

#[test]
fn the_path_limits_are_512_bytes_and_64_paths() {
    let long = |n: usize| raw(&check_write(&["p".repeat(n)]));
    assert!(long(511).is_ok() && long(512).is_ok() && long(513).is_err());
    let many = |n: usize| {
        let list: Vec<String> = (0..n).map(|i| format!("f{i}")).collect();
        raw(&check_write(&list))
    };
    assert!(many(63).is_ok() && many(64).is_ok() && many(65).is_err());
}

#[test]
fn the_lease_limit_is_one_hour() {
    let lease = |ms: u64| {
        raw(&json!({
            "kind": "mailbox", "call": "reserve",
            "paths": ["a"], "lease_ms": ms,
        }))
    };
    assert!(lease(1).is_ok() && lease(3_600_000).is_ok());
    assert!(lease(0).is_err() && lease(3_600_001).is_err());
}

#[test]
fn the_tag_suffix_limit_is_32_bytes() {
    let suffix = |n: usize| raw(&tag(&format!("v1.0.0-{}", "a".repeat(n))));
    assert!(suffix(31).is_ok() && suffix(32).is_ok());
    assert!(suffix(33).is_err());
}

#[test]
fn the_id_is_16_to_64_characters() {
    let id = |n: usize| {
        let body = json!({"id": "i".repeat(n), "kind": "status"});
        parse(body.to_string().as_bytes())
    };
    assert!(id(15).is_err() && id(16).is_ok());
    assert!(id(64).is_ok() && id(65).is_err());
}

#[test]
fn the_request_limit_is_64_kib() {
    let base = json!({"id": ID, "kind": "status"}).to_string();
    let pad = |extra: usize| {
        let spaces = " ".repeat(MAX_REQUEST_BYTES - base.len() + extra);
        format!("{base}{spaces}")
    };
    assert_eq!(pad(0).len(), MAX_REQUEST_BYTES);
    assert!(parse(pad(0).as_bytes()).is_ok());
    assert_eq!(parse(pad(1).as_bytes()).unwrap_err(), Refusal::TooBig);
}
