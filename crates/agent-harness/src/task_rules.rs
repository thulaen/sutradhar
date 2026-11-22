//! Which task calls an agent may make (spec 0155, "Tasks"; AGENTS.md
//! sections 8 and 10), on plain data. The harness fills [`TaskFacts`]
//! from Forgejo and its own record of who took what. A refusal names the
//! rule, never the task text.

use crate::request::Task;
use crate::request_check::URGENT_REASONS;

/// The most tasks that may carry `status/ready` at once (AGENTS.md
/// section 10, item 3).
pub const READY_MAX: usize = 10;

/// The most open urgent tasks at once (proposed).
pub const URGENT_MAX: usize = 3;

/// The labels an agent may put on a task it took.
const OWN_LABELS: [&str; 2] = ["status/working", "status/blocked"];

/// What the harness knows about the task a call names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskFacts {
    /// Whether it is an alarm task: the `alarm` label or an alarm title.
    pub is_alarm: bool,
    /// The mailbox name of the agent that took it, from the harness's own
    /// record.
    pub taken_by: Option<String>,
    /// Whether it is open.
    pub is_open: bool,
    /// The milestone it is in.
    pub milestone: Milestone,
}

/// The milestone of a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Milestone {
    /// The "Later" milestone; only the owner moves a task out of it.
    Later,
    /// The current phase's milestone.
    CurrentPhase,
    /// Any other milestone, such as a future phase.
    Elsewhere,
}

/// What the harness knows about the backlog.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Backlog {
    /// How many tasks carry `status/ready` now.
    pub ready_count: usize,
    /// How many open tasks the agent has taken.
    pub own_open_tasks: usize,
    /// How many open tasks carry `status/urgent` now.
    pub urgent_count: usize,
}

/// Where a new task is put.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Placement {
    /// The "Later" milestone; only the owner moves a task out of it.
    Later,
    /// The current phase's milestone, for an urgent task.
    CurrentPhase,
}

refusals! {
    /// Why a task call was refused.
    TaskRefusal {
        /// The call names a task the harness cannot find.
        UnknownTask => "the harness cannot find that task",
        /// An alarm task may be read and commented on, nothing more.
        AlarmTask => "an alarm task may only be read and commented on",
        /// The task was taken by another agent, or by none.
        NotYourTask => "the task is not one this agent took",
        /// Another agent took the task first.
        AlreadyTaken => "another agent took the task first",
        /// Only the owner moves a task out of "Later".
        TaskInLater => "only the owner moves a task out of Later",
        /// Ready is only for tasks of the current phase.
        OutsidePhase => "ready is only for tasks of the current phase",
        /// Ten tasks already carry `status/ready`.
        ReadyFull => "ten tasks already carry the ready label",
        /// `status/urgent` is only for a new task.
        UrgentOnlyForNew => "urgent is only for a new task",
        /// A new task goes to "Later" and is not ready.
        NewTaskNotReady => "a new task is not ready",
        /// The task is closed.
        TaskClosed => "the task is closed",
        /// An agent works on one task at a time.
        OneTaskAtATime => "the agent already holds an open task",
        /// A task someone took cannot be marked ready.
        TaskInProgress => "a task that was taken cannot be marked ready",
        /// The label is not one an agent may set here.
        LabelNotAllowed => "the agent may not set that label",
        /// An urgent task needs one of the four reasons.
        UrgentNeedsReason => "an urgent task needs a valid reason",
        /// Three urgent tasks are already open.
        UrgentFull => "three urgent tasks are already open",
    }
}

/// Checks one task call. `facts` is `None` for a call that makes a task.
/// A call that makes a task gives where it goes.
///
/// # Errors
/// The rule the call breaks.
pub fn check_task(
    agent: &str,
    task: &Task,
    facts: Option<&TaskFacts>,
    backlog: Backlog,
) -> Result<Option<Placement>, TaskRefusal> {
    if let Task::Make { label, reason, .. } = task {
        return place_new(label, reason.as_deref(), backlog).map(Some);
    }
    let facts = facts.ok_or(TaskRefusal::UnknownTask)?;
    match task {
        Task::Read(_) | Task::Comment(..) | Task::Make { .. } => {}
        Task::Take(_) => check_take(agent, facts, backlog)?,
        Task::Close(_) => check_own(agent, facts)?,
        Task::Label(_, label) => check_label(agent, label, facts, backlog)?,
    }
    Ok(None)
}

fn place_new(
    label: &str,
    reason: Option<&str>,
    backlog: Backlog,
) -> Result<Placement, TaskRefusal> {
    match label {
        "status/ready" => Err(TaskRefusal::NewTaskNotReady),
        "status/urgent" => {
            if !reason.is_some_and(|r| URGENT_REASONS.contains(&r)) {
                Err(TaskRefusal::UrgentNeedsReason)
            } else if backlog.urgent_count >= URGENT_MAX {
                Err(TaskRefusal::UrgentFull)
            } else {
                Ok(Placement::CurrentPhase)
            }
        }
        "status/blocked" => Ok(Placement::Later),
        _ => Err(TaskRefusal::LabelNotAllowed),
    }
}

fn check_own(agent: &str, facts: &TaskFacts) -> Result<(), TaskRefusal> {
    if facts.is_alarm {
        return Err(TaskRefusal::AlarmTask);
    }
    if !facts.is_open {
        return Err(TaskRefusal::TaskClosed);
    }
    if facts.taken_by.as_deref() == Some(agent) {
        Ok(())
    } else {
        Err(TaskRefusal::NotYourTask)
    }
}

fn check_take(
    agent: &str,
    facts: &TaskFacts,
    backlog: Backlog,
) -> Result<(), TaskRefusal> {
    if facts.is_alarm {
        return Err(TaskRefusal::AlarmTask);
    }
    if !facts.is_open {
        return Err(TaskRefusal::TaskClosed);
    }
    if facts.milestone == Milestone::Later {
        return Err(TaskRefusal::TaskInLater);
    }
    if facts.milestone != Milestone::CurrentPhase {
        return Err(TaskRefusal::OutsidePhase);
    }
    match facts.taken_by.as_deref() {
        Some(who) if who != agent => Err(TaskRefusal::AlreadyTaken),
        None if backlog.own_open_tasks > 0 => Err(TaskRefusal::OneTaskAtATime),
        _ => Ok(()),
    }
}

fn check_label(
    agent: &str,
    label: &str,
    facts: &TaskFacts,
    backlog: Backlog,
) -> Result<(), TaskRefusal> {
    if facts.is_alarm {
        return Err(TaskRefusal::AlarmTask);
    }
    match label {
        "status/urgent" => Err(TaskRefusal::UrgentOnlyForNew),
        "status/ready" => check_ready(facts, backlog),
        _ if OWN_LABELS.contains(&label) => check_own(agent, facts),
        _ => Err(TaskRefusal::LabelNotAllowed),
    }
}

/// Marking a task ready is backlog grooming of a task nobody took; it
/// needs an open current-phase task and a free place.
fn check_ready(facts: &TaskFacts, backlog: Backlog) -> Result<(), TaskRefusal> {
    if !facts.is_open {
        return Err(TaskRefusal::TaskClosed);
    }
    if facts.milestone == Milestone::Later {
        return Err(TaskRefusal::TaskInLater);
    }
    if facts.milestone != Milestone::CurrentPhase {
        return Err(TaskRefusal::OutsidePhase);
    }
    if facts.taken_by.is_some() {
        return Err(TaskRefusal::TaskInProgress);
    }
    if backlog.ready_count >= READY_MAX {
        return Err(TaskRefusal::ReadyFull);
    }
    Ok(())
}
