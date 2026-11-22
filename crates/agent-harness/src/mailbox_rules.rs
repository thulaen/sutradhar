//! Which mailbox calls an agent may make (spec 0155, "Mailbox"): an
//! agent touches only its own reservations and messages. Plain data; the
//! harness fills [`Owned`] from the mailbox for the agent the socket
//! named.

use crate::request::Mailbox;

/// What the mailbox says the agent owns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Owned {
    /// The ids of the agent's own reservations.
    pub reservations: Vec<u64>,
    /// The ids of the messages sent to the agent.
    pub messages: Vec<u64>,
}

refusals! {
    /// Why a mailbox call was refused.
    MailboxRefusal {
        /// The call names a reservation or message of another agent.
        NotYours => "the call names something that is not this agent's",
    }
}

/// Checks one mailbox call.
///
/// # Errors
/// [`MailboxRefusal::NotYours`] when the call names another agent's
/// reservation or message.
pub fn check_mailbox(
    call: &Mailbox,
    owned: &Owned,
) -> Result<(), MailboxRefusal> {
    let mine = match call {
        Mailbox::Release(id)
        | Mailbox::Renew {
            reservation: id, ..
        } => owned.reservations.contains(id),
        Mailbox::Ack(ids) => ids.iter().all(|id| owned.messages.contains(id)),
        Mailbox::Reserve { .. }
        | Mailbox::CheckWrite(_)
        | Mailbox::Check
        | Mailbox::ListReservations
        | Mailbox::ListParticipants
        | Mailbox::GetTask(_)
        | Mailbox::Send { .. } => true,
    };
    if mine {
        Ok(())
    } else {
        Err(MailboxRefusal::NotYours)
    }
}
