//! The parts of a request, one kind at a time (spec 0155, "Requests").

use serde_json::{Map, Value};

use crate::request::{Mailbox, Push, Refusal, Task};
use crate::request_check as check;

/// The fields of one request that are still to be read.
pub(crate) type Fields = Map<String, Value>;

pub(crate) fn take(fields: &mut Fields, key: &str) -> Result<Value, Refusal> {
    fields.remove(key).ok_or(Refusal::MissingField)
}

pub(crate) fn text(fields: &mut Fields, key: &str) -> Result<String, Refusal> {
    match take(fields, key)? {
        Value::String(s) => Ok(s),
        _ => Err(Refusal::BadValue),
    }
}

fn number(fields: &mut Fields, key: &str) -> Result<u64, Refusal> {
    take(fields, key)?.as_u64().ok_or(Refusal::BadValue)
}

fn task_number(fields: &mut Fields) -> Result<u64, Refusal> {
    let n = number(fields, "number")?;
    if (1..=check::NUMBER_MAX).contains(&n) {
        Ok(n)
    } else {
        Err(Refusal::BadValue)
    }
}

fn commit(fields: &mut Fields, key: &str) -> Result<String, Refusal> {
    let c = text(fields, key)?;
    if check::is_commit(&c) {
        Ok(c)
    } else {
        Err(Refusal::BadValue)
    }
}

pub(crate) fn push(fields: &mut Fields) -> Result<Push, Refusal> {
    let commit_id = commit(fields, "commit")?;
    if fields.contains_key("tag") {
        let name = text(fields, "tag")?;
        if !check::is_release_tag(&name) {
            return Err(Refusal::BadValue);
        }
        Ok(Push::Tag {
            name,
            commit: commit_id,
        })
    } else {
        let expected_main = commit(fields, "expected_main")?;
        Ok(Push::Main {
            commit: commit_id,
            expected_main,
        })
    }
}

fn body(fields: &mut Fields) -> Result<String, Refusal> {
    let b = text(fields, "body")?;
    if check::is_text(&b, check::BODY_MAX) && !check::forges_agent_mark(&b) {
        Ok(b)
    } else {
        Err(Refusal::BadValue)
    }
}

fn label(fields: &mut Fields) -> Result<String, Refusal> {
    let l = text(fields, "label")?;
    if check::is_agent_label(&l) {
        Ok(l)
    } else {
        Err(Refusal::BadValue)
    }
}

/// The reason an urgent task gives: one of the four of AGENTS.md section
/// 8, and required with `status/urgent` and with nothing else.
fn urgent_reason(
    fields: &mut Fields,
    label: &str,
) -> Result<Option<String>, Refusal> {
    let urgent = label == check::URGENT_LABEL;
    if !fields.contains_key("reason") {
        return if urgent {
            Err(Refusal::MissingField)
        } else {
            Ok(None)
        };
    }
    let reason = text(fields, "reason")?;
    if urgent && check::URGENT_REASONS.contains(&reason.as_str()) {
        Ok(Some(reason))
    } else {
        Err(Refusal::BadValue)
    }
}

pub(crate) fn task(fields: &mut Fields) -> Result<Task, Refusal> {
    let call = text(fields, "call")?;
    Ok(match call.as_str() {
        "read" => Task::Read(task_number(fields)?),
        "take" => Task::Take(task_number(fields)?),
        "close" => Task::Close(task_number(fields)?),
        "comment" => {
            let n = task_number(fields)?;
            Task::Comment(n, body(fields)?)
        }
        "label" => {
            let n = task_number(fields)?;
            Task::Label(n, label(fields)?)
        }
        "make" => {
            let title = text(fields, "title")?;
            if check::is_alarm_title(&title) {
                return Err(Refusal::AlarmTitle);
            }
            if !check::is_line(&title, check::TITLE_MAX) {
                return Err(Refusal::BadValue);
            }
            let body = body(fields)?;
            let label = label(fields)?;
            let reason = urgent_reason(fields, &label)?;
            Task::Make {
                title,
                body,
                label,
                reason,
            }
        }
        _ => return Err(Refusal::BadKind),
    })
}

fn paths(fields: &mut Fields) -> Result<Vec<String>, Refusal> {
    let Value::Array(items) = take(fields, "paths")? else {
        return Err(Refusal::BadValue);
    };
    if items.is_empty() || items.len() > check::PATHS_MAX {
        return Err(Refusal::BadValue);
    }
    items
        .into_iter()
        .map(|item| match item {
            Value::String(p) if check::is_path(&p) => Ok(p),
            _ => Err(Refusal::BadValue),
        })
        .collect()
}

fn lease(fields: &mut Fields) -> Result<u64, Refusal> {
    let ms = number(fields, "lease_ms")?;
    if (1..=check::LEASE_MAX_MS).contains(&ms) {
        Ok(ms)
    } else {
        Err(Refusal::BadValue)
    }
}

fn numbers(fields: &mut Fields) -> Result<Vec<u64>, Refusal> {
    let Value::Array(items) = take(fields, "ids")? else {
        return Err(Refusal::BadValue);
    };
    if items.is_empty() || items.len() > check::PATHS_MAX {
        return Err(Refusal::BadValue);
    }
    items
        .iter()
        .map(|item| item.as_u64().ok_or(Refusal::BadValue))
        .collect()
}

pub(crate) fn mailbox(fields: &mut Fields) -> Result<Mailbox, Refusal> {
    let call = text(fields, "call")?;
    Ok(match call.as_str() {
        "reserve" => {
            let paths = paths(fields)?;
            let lease_ms = lease(fields)?;
            Mailbox::Reserve { paths, lease_ms }
        }
        "release" => Mailbox::Release(number(fields, "reservation")?),
        "renew" => {
            let reservation = number(fields, "reservation")?;
            let lease_ms = lease(fields)?;
            Mailbox::Renew {
                reservation,
                lease_ms,
            }
        }
        "check_write" => Mailbox::CheckWrite(paths(fields)?),
        "list_reservations" => Mailbox::ListReservations,
        "list_participants" => Mailbox::ListParticipants,
        "ack" => Mailbox::Ack(numbers(fields)?),
        "get_task" => Mailbox::GetTask(number(fields, "task")?),
        "check" => Mailbox::Check,
        "send" => {
            let to = text(fields, "to")?;
            let message = text(fields, "text")?;
            if !check::is_agent_name(&to)
                || message.trim().is_empty()
                || !check::is_text(&message, check::BODY_MAX)
                || check::forges_agent_mark(&message)
            {
                return Err(Refusal::BadValue);
            }
            Mailbox::Send { to, text: message }
        }
        _ => return Err(Refusal::BadKind),
    })
}
