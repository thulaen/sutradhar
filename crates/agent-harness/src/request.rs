//! One request from an agent, parsed and checked (spec 0155,
//! "Requests"). A request is a small typed record. An unknown field, a
//! repeated field, an unknown kind or a value that does not match its
//! pattern is refused, and a refusal never repeats what it was sent.

use std::collections::BTreeSet;
use std::fmt;

use serde::de::{Deserialize, Deserializer, MapAccess, Visitor};
use serde_json::Value;

use crate::request_check as check;
use crate::request_parts::{mailbox, push, task, text};

/// The largest request, in bytes (proposed in spec 0155).
pub const MAX_REQUEST_BYTES: usize = 64 * 1024;

/// A parsed request and its id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Envelope {
    /// The random id the harness keeps to refuse a replay.
    pub id: String,
    /// What is asked.
    pub request: Request,
}

/// What an agent may ask.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Request {
    /// Move `main` forward, or add a release tag.
    Push(Push),
    /// A Forgejo task call.
    Task(Task),
    /// A mailbox call, made as the agent's own name.
    Mailbox(Mailbox),
    /// Ask whether the harness is paused, and its limits.
    Status,
}

/// A push request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Push {
    /// Move `main` forward by one commit.
    Main {
        /// The commit to push.
        commit: String,
        /// The head of `main` the push expects to find.
        expected_main: String,
    },
    /// Add a release tag naming a commit already on `main`.
    Tag {
        /// The tag name.
        name: String,
        /// The commit it names.
        commit: String,
    },
}

/// A task call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Task {
    /// Read a task, its comments and labels.
    Read(u64),
    /// Take a task.
    Take(u64),
    /// Close a task the agent took.
    Close(u64),
    /// Comment on a task.
    Comment(u64, String),
    /// Set one `status/` label on a task the agent took.
    Label(u64, String),
    /// Make a task.
    Make {
        /// The title.
        title: String,
        /// The body.
        body: String,
        /// The one `status/` label.
        label: String,
        /// Why the task is urgent, one of the four reasons of AGENTS.md
        /// section 8; present only with `status/urgent`.
        reason: Option<String>,
    },
}

/// A mailbox call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mailbox {
    /// Renew one of the agent's own reservations.
    Renew {
        /// The reservation.
        reservation: u64,
        /// The new lease, in milliseconds.
        lease_ms: u64,
    },
    /// Ask whether the agent may write these paths.
    CheckWrite(Vec<String>),
    /// List the agent's own reservations.
    ListReservations,
    /// List who is using the mailbox.
    ListParticipants,
    /// Acknowledge messages the agent has read.
    Ack(Vec<u64>),
    /// Read one mailbox task.
    GetTask(u64),
    /// Reserve paths for a while.
    Reserve {
        /// The paths.
        paths: Vec<String>,
        /// The lease, in milliseconds.
        lease_ms: u64,
    },
    /// Release one of the agent's own reservations.
    Release(u64),
    /// Check the agent's own messages.
    Check,
    /// Send a message.
    Send {
        /// The receiving agent's mailbox name.
        to: String,
        /// The text.
        text: String,
    },
}

refusals! {
    /// Why a request was refused. It names the rule, never the input.
    Refusal {
        /// Over the size limit.
        TooBig => "the request is over the size limit",
        /// Not JSON.
        NotJson => "the request is not JSON",
        /// Not one JSON object.
        NotAnObject => "the request is not one JSON object",
        /// A field the request does not allow, or one given twice.
        UnknownField => "the request has a field it may not have",
        /// A field the request needs is missing.
        MissingField => "the request lacks a field it needs",
        /// The id does not match its pattern.
        BadId => "the request id does not match its pattern",
        /// The kind or call is unknown.
        BadKind => "the request kind or call is unknown",
        /// A value does not match its pattern or limit.
        BadValue => "a value does not match its pattern or limit",
        /// A task title an agent may not use.
        AlarmTitle => "an agent may not use that task title",
    }
}

/// An object whose keys are kept in order, so a repeated key shows.
struct Pairs(Vec<(String, Value)>);

/// Collects the entries of one object in order.
struct Walk;

impl<'de> Visitor<'de> for Walk {
    type Value = Pairs;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("one JSON object")
    }

    fn visit_map<A: MapAccess<'de>>(
        self,
        mut map: A,
    ) -> Result<Pairs, A::Error> {
        let mut pairs = Vec::new();
        while let Some(pair) = map.next_entry::<String, Value>()? {
            pairs.push(pair);
        }
        Ok(Pairs(pairs))
    }
}

impl<'de> Deserialize<'de> for Pairs {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_map(Walk)
    }
}

/// Parses and checks one request.
///
/// # Errors
/// The [`Refusal`] that names the rule it broke.
pub fn parse(bytes: &[u8]) -> Result<Envelope, Refusal> {
    if bytes.len() > MAX_REQUEST_BYTES {
        return Err(Refusal::TooBig);
    }
    let value: Value =
        serde_json::from_slice(bytes).map_err(|_| Refusal::NotJson)?;
    if !value.is_object() {
        return Err(Refusal::NotAnObject);
    }
    let Pairs(pairs) =
        serde_json::from_slice(bytes).map_err(|_| Refusal::NotAnObject)?;
    let mut seen = BTreeSet::new();
    for (key, _) in &pairs {
        if !seen.insert(key.as_str()) {
            return Err(Refusal::UnknownField);
        }
    }
    let Value::Object(mut fields) = value else {
        return Err(Refusal::NotAnObject);
    };
    let id = text(&mut fields, "id")?;
    if !check::is_id(&id) {
        return Err(Refusal::BadId);
    }
    let kind = text(&mut fields, "kind")?;
    let request = match kind.as_str() {
        "push" => Request::Push(push(&mut fields)?),
        "task" => Request::Task(task(&mut fields)?),
        "mailbox" => Request::Mailbox(mailbox(&mut fields)?),
        "status" => Request::Status,
        _ => return Err(Refusal::BadKind),
    };
    if fields.is_empty() {
        Ok(Envelope { id, request })
    } else {
        Err(Refusal::UnknownField)
    }
}
