//! Refused requests, counted once a minute for each agent (spec 0155,
//! "The log"), so a flood cannot push earlier lines out of the log. Plain
//! data on a given monotonic clock. The last minute comes out when the
//! next refusal arrives or when the service calls `flush` on its timer.

use std::collections::BTreeMap;

use crate::agent::AgentName;

/// One minute, in milliseconds.
pub const MINUTE_MS: u64 = 60_000;

/// Counts refusals for each agent in the current minute.
#[derive(Debug, Default)]
pub struct Summariser {
    minute: Option<u64>,
    counts: BTreeMap<AgentName, u64>,
}

impl Summariser {
    /// An empty summariser.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Notes one refusal at `now_ms`. When a new minute has begun, gives
    /// the counts of the minute before, agent by agent; otherwise none. A
    /// clock that goes backwards counts in the current minute.
    pub fn note(
        &mut self,
        agent: &AgentName,
        now_ms: u64,
    ) -> Vec<(AgentName, u64)> {
        let now_minute = now_ms / MINUTE_MS;
        let done = match self.minute {
            Some(m) if now_minute > m => self.flush(),
            _ => Vec::new(),
        };
        self.minute =
            Some(self.minute.map_or(now_minute, |m| m.max(now_minute)));
        let count = self.counts.entry(agent.clone()).or_insert(0);
        *count = count.saturating_add(1);
        done
    }

    /// Gives what has been counted and starts again.
    pub fn flush(&mut self) -> Vec<(AgentName, u64)> {
        std::mem::take(&mut self.counts).into_iter().collect()
    }
}
