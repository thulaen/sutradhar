//! The rates and connection counts of spec 0155, "Limits" (all
//! proposed), on plain data and a given clock. A request over a limit
//! is refused with the limit named. Times come from a monotonic clock,
//! so a time that goes backwards is held at the newest time seen.

use std::collections::{BTreeMap, VecDeque};

use crate::agent::AgentName;

/// One hour, in milliseconds.
pub const HOUR_MS: u64 = 3_600_000;
/// Pushes one agent may make in an hour.
const PUSH_PER_HOUR: usize = 12;
/// Task calls one agent may make in an hour.
const TASK_PER_HOUR: usize = 240;
/// Mailbox calls one agent may make in an hour.
const MAILBOX_PER_HOUR: usize = 600;
/// Refused requests one agent may make in an hour.
const REFUSED_PER_HOUR: usize = 120;
/// The most open connections for one agent.
pub const PER_AGENT_CONNECTIONS: usize = 2;
/// The most open connections in all.
pub const TOTAL_CONNECTIONS: usize = 16;

/// What a request counts against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Class {
    /// A push.
    Push,
    /// A task call.
    Task,
    /// A mailbox call.
    Mailbox,
    /// A request the harness refused.
    Refused,
}

impl Class {
    /// How many requests of this class one agent may make in an hour.
    const fn rate(self) -> usize {
        match self {
            Self::Push => PUSH_PER_HOUR,
            Self::Task => TASK_PER_HOUR,
            Self::Mailbox => MAILBOX_PER_HOUR,
            Self::Refused => REFUSED_PER_HOUR,
        }
    }

    const fn refusal(self) -> LimitRefusal {
        match self {
            Self::Push => LimitRefusal::PushRate,
            Self::Task => LimitRefusal::TaskRate,
            Self::Mailbox => LimitRefusal::MailboxRate,
            Self::Refused => LimitRefusal::RefusedRate,
        }
    }
}

refusals! {
    /// Which limit a request went over.
    LimitRefusal {
        /// Too many pushes in an hour.
        PushRate => "over the push limit of 12 an hour",
        /// Too many task calls in an hour.
        TaskRate => "over the task limit of 240 an hour",
        /// Too many mailbox calls in an hour.
        MailboxRate => "over the mailbox limit of 600 an hour",
        /// Too many refused requests in an hour.
        RefusedRate => "over the refused-request limit of 120 an hour",
        /// The agent has too many open connections.
        AgentConnections => "over the limit of 2 open connections",
        /// The harness has too many open connections.
        TotalConnections => "over the limit of 16 open connections",
    }
}

/// Counts requests and connections.
#[derive(Debug, Default)]
pub struct Limiter {
    events: BTreeMap<(AgentName, Class), VecDeque<u64>>,
    connections: BTreeMap<AgentName, usize>,
}

impl Limiter {
    /// A limiter that has seen nothing.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Counts one request of `class` by `agent` at `now_ms`.
    ///
    /// # Errors
    /// The limit it goes over; it is then not counted.
    pub fn allow(
        &mut self,
        agent: &AgentName,
        class: Class,
        now_ms: u64,
    ) -> Result<(), LimitRefusal> {
        let times = self.events.entry((agent.clone(), class)).or_default();
        // A clock that goes backwards is held at the newest time seen.
        let now = times.back().map_or(now_ms, |&last| last.max(now_ms));
        while times
            .front()
            .is_some_and(|&t| t.saturating_add(HOUR_MS) <= now)
        {
            let _old = times.pop_front();
        }
        if times.len() >= class.rate() {
            return Err(class.refusal());
        }
        times.push_back(now);
        Ok(())
    }

    /// Counts one open connection for `agent`.
    ///
    /// # Errors
    /// The connection limit it goes over.
    pub fn open(&mut self, agent: &AgentName) -> Result<(), LimitRefusal> {
        let total: usize = self.connections.values().sum();
        if total >= TOTAL_CONNECTIONS {
            return Err(LimitRefusal::TotalConnections);
        }
        let own = self.connections.entry(agent.clone()).or_insert(0);
        if *own >= PER_AGENT_CONNECTIONS {
            return Err(LimitRefusal::AgentConnections);
        }
        *own = own.saturating_add(1);
        Ok(())
    }

    /// Counts one connection of `agent` closed. Closing more than was
    /// opened leaves the count at none.
    pub fn close(&mut self, agent: &AgentName) {
        if let Some(own) = self.connections.get_mut(agent) {
            *own = own.saturating_sub(1);
            if *own == 0 {
                let _gone = self.connections.remove(agent);
            }
        }
    }
}
