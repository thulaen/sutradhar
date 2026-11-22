//! IT-0155-MAILBOX: an agent touches only its own reservations and
//! messages (spec 0155, "Mailbox"), on plain data.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use agent_harness::mailbox_rules::{MailboxRefusal, Owned, check_mailbox};
use agent_harness::request::Mailbox;

fn owned() -> Owned {
    Owned {
        reservations: vec![4, 5],
        messages: vec![10, 11],
    }
}

#[test]
fn an_agent_releases_and_renews_only_its_own_reservations() {
    let renew = |n| Mailbox::Renew {
        reservation: n,
        lease_ms: 60_000,
    };
    assert_eq!(check_mailbox(&Mailbox::Release(4), &owned()), Ok(()));
    assert_eq!(check_mailbox(&renew(5), &owned()), Ok(()));
    assert_eq!(
        check_mailbox(&Mailbox::Release(9), &owned()),
        Err(MailboxRefusal::NotYours)
    );
    assert_eq!(
        check_mailbox(&renew(9), &owned()),
        Err(MailboxRefusal::NotYours)
    );
}

#[test]
fn an_agent_acknowledges_only_its_own_messages() {
    let ack = |ids: &[u64]| Mailbox::Ack(ids.to_vec());
    assert_eq!(check_mailbox(&ack(&[10, 11]), &owned()), Ok(()));
    assert_eq!(
        check_mailbox(&ack(&[10, 12]), &owned()),
        Err(MailboxRefusal::NotYours)
    );
}

#[test]
fn the_other_allowed_calls_need_no_ownership() {
    let paths = vec!["docs/a.md".to_owned()];
    let calls = [
        Mailbox::Reserve {
            paths: paths.clone(),
            lease_ms: 1000,
        },
        Mailbox::CheckWrite(paths),
        Mailbox::Check,
        Mailbox::ListReservations,
        Mailbox::ListParticipants,
        Mailbox::GetTask(3),
        Mailbox::Send {
            to: "claude".to_owned(),
            text: "hello".to_owned(),
        },
    ];
    for call in calls {
        assert_eq!(check_mailbox(&call, &owned()), Ok(()), "{call:?}");
    }
}

#[test]
fn an_agent_with_nothing_owns_nothing() {
    let none = Owned {
        reservations: vec![],
        messages: vec![],
    };
    assert_eq!(
        check_mailbox(&Mailbox::Release(4), &none),
        Err(MailboxRefusal::NotYours)
    );
    assert_eq!(
        check_mailbox(&Mailbox::Ack(vec![10]), &none),
        Err(MailboxRefusal::NotYours)
    );
}
