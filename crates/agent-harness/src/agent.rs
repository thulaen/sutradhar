//! An agent's name, the one key the harness counts and logs by.

use crate::request_check::is_agent_name;

/// A mailbox name that matches its pattern. The service makes one only
/// for an account in the owner's map, so a name an agent invents is
/// never counted, logged or kept in memory.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AgentName(String);

impl AgentName {
    /// The name, if it matches the pattern of a mailbox name.
    #[must_use]
    pub fn new(text: &str) -> Option<Self> {
        is_agent_name(text).then(|| Self(text.to_owned()))
    }

    /// The name as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
