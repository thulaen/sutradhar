//! Helpers shared by the request tests.
#![allow(dead_code, reason = "each test file uses some of these")]
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use agent_harness::request::{Refusal, Request, parse};
use serde_json::{Value, json};

/// A valid request id.
pub const ID: &str = "req-0123456789abcdef";
/// A valid commit id.
pub const SHA: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

/// Parses `{"id": ID, ...fields}`.
pub fn raw(fields: &Value) -> Result<Request, Refusal> {
    let mut value = json!({ "id": ID });
    value
        .as_object_mut()
        .unwrap()
        .extend(fields.as_object().unwrap().clone());
    parse(value.to_string().as_bytes()).map(|e| e.request)
}

/// A request to make a task with this title.
pub fn make(title: &str) -> Value {
    json!({
        "kind": "task", "call": "make", "title": title,
        "body": "why", "label": "status/ready",
    })
}

/// A request to comment on a task with this body.
pub fn comment(body: &str) -> Value {
    json!({
        "kind": "task", "call": "comment", "number": 1, "body": body,
    })
}

/// A request to send a message with this text.
pub fn send(text: &str) -> Value {
    json!({
        "kind": "mailbox", "call": "send", "to": "codex", "text": text,
    })
}

/// A request to check write rights on these paths.
pub fn check_write(paths: &[String]) -> Value {
    json!({
        "kind": "mailbox", "call": "check_write", "paths": paths,
    })
}

/// A request to push this release tag.
pub fn tag(name: &str) -> Value {
    json!({ "kind": "push", "tag": name, "commit": SHA })
}

/// A request to set this label on task 3.
pub fn label(name: &str) -> Value {
    json!({
        "kind": "task", "call": "label", "number": 3, "label": name,
    })
}
