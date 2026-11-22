//! What one log line says: an agent, a kind of request and a fixed
//! outcome (spec 0155, "The log"). Nothing else can be put in a line.

use std::fmt;

use crate::agent::AgentName;

/// What a log line says happened. Only the types of this crate are
/// outcomes: the fixed texts of the refusals, [`Done`] and [`Summary`],
/// so no token, commit text or other free text can enter the log.
pub trait Outcome: fmt::Display + crate::refusal::Sealed {}

/// An outcome that is not a refusal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Done {
    /// The request was answered.
    Answered,
    /// The harness is about to push.
    Intent,
    /// The push is on `main`.
    Landed,
    /// The push is not on `main`.
    NotLanded,
    /// The harness cannot tell whether the push is on `main`.
    Unknown,
}

impl crate::refusal::Sealed for Done {}
impl Outcome for Done {}

impl fmt::Display for Done {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Answered => "answered",
            Self::Intent => "about to push",
            Self::Landed => "landed",
            Self::NotLanded => "not landed",
            Self::Unknown => "unknown",
        })
    }
}

/// The count of refused requests in the minute before.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Summary(pub u64);

impl crate::refusal::Sealed for Summary {}
impl Outcome for Summary {}

impl fmt::Display for Summary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "refused {} in the last minute", self.0)
    }
}

/// What kind of request a line is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// A push.
    Push,
    /// A task call.
    Task,
    /// A mailbox call.
    Mailbox,
    /// A status request.
    Status,
    /// A summary of refused requests.
    Summary,
}

impl Kind {
    const fn word(self) -> &'static str {
        match self {
            Self::Push => "push",
            Self::Task => "task",
            Self::Mailbox => "mailbox",
            Self::Status => "status",
            Self::Summary => "summary",
        }
    }
}

/// One line's text, built only from an agent, a kind and an outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry(String);

impl Entry {
    /// The text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Builds the entry for one request: who, what kind and what happened.
/// Nothing else can be put in a log line.
#[must_use]
pub fn entry(agent: &AgentName, kind: Kind, outcome: &dyn Outcome) -> Entry {
    Entry(format!("{} {} {outcome}", agent.as_str(), kind.word()))
}
