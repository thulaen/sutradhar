//! UT-0155-REQUEST: the request parser accepts what the spec allows.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

mod common;

use agent_harness::request::{Mailbox, Push, Request, Task, parse};
use common::{ID, SHA, raw};
use serde_json::json;

#[test]
fn a_main_push_names_one_commit_and_the_main_it_expects() {
    let body = json!({
        "id": ID, "kind": "push",
        "commit": SHA, "expected_main": SHA,
    })
    .to_string();
    let envelope = parse(body.as_bytes()).unwrap();
    assert_eq!(envelope.id, ID);
    assert_eq!(
        envelope.request,
        Request::Push(Push::Main {
            commit: SHA.to_owned(),
            expected_main: SHA.to_owned(),
        })
    );
}

#[test]
fn a_tag_push_names_a_release_tag_and_its_commit() {
    let got = raw(&common::tag("v1.2.3-rc.1")).unwrap();
    assert_eq!(
        got,
        Request::Push(Push::Tag {
            name: "v1.2.3-rc.1".to_owned(),
            commit: SHA.to_owned(),
        })
    );
}

#[test]
fn every_task_call_parses() {
    let make = json!({
        "kind": "task", "call": "make", "title": "A task",
        "body": "Why", "label": "status/ready",
    });
    let cases = [
        (
            json!({"kind":"task","call":"read","number":640}),
            Task::Read(640),
        ),
        (
            json!({"kind":"task","call":"take","number":7}),
            Task::Take(7),
        ),
        (
            json!({"kind":"task","call":"close","number":7}),
            Task::Close(7),
        ),
        (common::comment("done"), Task::Comment(1, "done".to_owned())),
        (
            common::label("status/working"),
            Task::Label(3, "status/working".to_owned()),
        ),
        (
            make,
            Task::Make {
                title: "A task".to_owned(),
                body: "Why".to_owned(),
                label: "status/ready".to_owned(),
            },
        ),
    ];
    for (body, want) in cases {
        assert_eq!(raw(&body).unwrap(), Request::Task(want), "{body}");
    }
}

#[test]
fn the_allowed_mailbox_calls_parse() {
    let reserve = json!({
        "kind": "mailbox", "call": "reserve",
        "paths": ["docs/a.md", "crates/x/src/lib.rs"], "lease_ms": 60000,
    });
    let cases = [
        (
            reserve,
            Mailbox::Reserve {
                paths: vec![
                    "docs/a.md".to_owned(),
                    "crates/x/src/lib.rs".to_owned(),
                ],
                lease_ms: 60_000,
            },
        ),
        (
            json!({"kind":"mailbox","call":"release","reservation":12}),
            Mailbox::Release(12),
        ),
        (json!({"kind":"mailbox","call":"check"}), Mailbox::Check),
        (
            common::send("hello"),
            Mailbox::Send {
                to: "codex".to_owned(),
                text: "hello".to_owned(),
            },
        ),
    ];
    for (body, want) in cases {
        assert_eq!(raw(&body).unwrap(), Request::Mailbox(want), "{body}");
    }
}

#[test]
fn the_rest_of_the_mailbox_calls_parse() {
    let renew = json!({
        "kind": "mailbox", "call": "renew",
        "reservation": 4, "lease_ms": 3_600_000,
    });
    let cases = [
        (
            renew,
            Mailbox::Renew {
                reservation: 4,
                lease_ms: 3_600_000,
            },
        ),
        (
            common::check_write(&["a/b".to_owned()]),
            Mailbox::CheckWrite(vec!["a/b".to_owned()]),
        ),
        (
            json!({"kind":"mailbox","call":"list_reservations"}),
            Mailbox::ListReservations,
        ),
        (
            json!({"kind":"mailbox","call":"list_participants"}),
            Mailbox::ListParticipants,
        ),
        (
            json!({"kind":"mailbox","call":"ack","ids":[1,2]}),
            Mailbox::Ack(vec![1, 2]),
        ),
        (
            json!({"kind":"mailbox","call":"get_task","task":9}),
            Mailbox::GetTask(9),
        ),
    ];
    for (body, want) in cases {
        assert_eq!(raw(&body).unwrap(), Request::Mailbox(want), "{body}");
    }
}

#[test]
fn status_needs_nothing_else() {
    assert_eq!(raw(&json!({"kind":"status"})).unwrap(), Request::Status);
}
