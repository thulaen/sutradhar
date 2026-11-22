//! IT-0155-TASKS: which task calls an agent may make (spec 0155,
//! "Tasks"; AGENTS.md sections 8 and 10), on plain data.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use agent_harness::request::Task;
use agent_harness::task_rules::{
    Backlog, Milestone, Placement, READY_MAX, TaskFacts, TaskRefusal,
    URGENT_MAX, check_task,
};

const ME: &str = "codex";

fn facts() -> TaskFacts {
    TaskFacts {
        is_alarm: false,
        taken_by: Some(ME.to_owned()),
        is_open: true,
        milestone: Milestone::CurrentPhase,
    }
}

fn free() -> TaskFacts {
    TaskFacts {
        taken_by: None,
        ..facts()
    }
}

fn backlog() -> Backlog {
    Backlog {
        ready_count: 0,
        own_open_tasks: 0,
        urgent_count: 0,
    }
}

fn call(task: &Task, facts: &TaskFacts) -> Result<(), TaskRefusal> {
    check_task(ME, task, Some(facts), backlog()).map(|_| ())
}

fn label(word: &str) -> Task {
    Task::Label(7, word.to_owned())
}

fn make(label: &str, reason: Option<&str>) -> Task {
    Task::Make {
        title: "A task".to_owned(),
        body: "Why".to_owned(),
        label: label.to_owned(),
        reason: reason.map(str::to_owned),
    }
}

#[test]
fn reading_and_commenting_are_allowed_even_on_an_alarm_task() {
    let alarm = TaskFacts {
        is_alarm: true,
        taken_by: None,
        ..facts()
    };
    assert_eq!(call(&Task::Read(7), &alarm), Ok(()));
    assert_eq!(call(&Task::Comment(7, "x".to_owned()), &alarm), Ok(()));
}

#[test]
fn an_alarm_task_is_never_taken_closed_or_relabelled() {
    let alarm = TaskFacts {
        is_alarm: true,
        ..facts()
    };
    let calls = [
        Task::Take(7),
        Task::Close(7),
        label("status/working"),
        label("status/ready"),
    ];
    for task in calls {
        assert_eq!(call(&task, &alarm), Err(TaskRefusal::AlarmTask));
    }
}

#[test]
fn only_the_agent_that_took_a_task_closes_or_relabels_it() {
    let other = TaskFacts {
        taken_by: Some("claude".to_owned()),
        ..facts()
    };
    for who in [&other, &free()] {
        let close = call(&Task::Close(7), who);
        assert_eq!(close, Err(TaskRefusal::NotYourTask));
        let work = call(&label("status/working"), who);
        assert_eq!(work, Err(TaskRefusal::NotYourTask));
    }
    assert_eq!(call(&Task::Close(7), &facts()), Ok(()));
    assert_eq!(call(&label("status/blocked"), &facts()), Ok(()));
}

#[test]
fn a_closed_task_is_not_taken_closed_again_or_relabelled() {
    let closed = TaskFacts {
        is_open: false,
        ..facts()
    };
    let calls = [Task::Take(7), Task::Close(7), label("status/working")];
    for task in calls {
        assert_eq!(call(&task, &closed), Err(TaskRefusal::TaskClosed));
    }
    let ready = call(
        &label("status/ready"),
        &TaskFacts {
            is_open: false,
            ..free()
        },
    );
    assert_eq!(ready, Err(TaskRefusal::TaskClosed));
    assert_eq!(call(&Task::Comment(7, "proof".to_owned()), &closed), Ok(()));
}

#[test]
fn the_first_agent_to_take_a_task_wins_and_a_take_is_repeatable() {
    let other = TaskFacts {
        taken_by: Some("claude".to_owned()),
        ..facts()
    };
    assert_eq!(call(&Task::Take(7), &other), Err(TaskRefusal::AlreadyTaken));
    assert_eq!(call(&Task::Take(7), &free()), Ok(()));
    assert_eq!(call(&Task::Take(7), &facts()), Ok(()));
}

#[test]
fn an_agent_takes_only_current_phase_tasks_and_one_at_a_time() {
    let later = TaskFacts {
        milestone: Milestone::Later,
        ..free()
    };
    let take = call(&Task::Take(7), &later);
    assert_eq!(take, Err(TaskRefusal::TaskInLater));
    let future = TaskFacts {
        milestone: Milestone::Elsewhere,
        ..free()
    };
    let take = call(&Task::Take(7), &future);
    assert_eq!(take, Err(TaskRefusal::OutsidePhase));
    let busy = Backlog {
        own_open_tasks: 1,
        ..backlog()
    };
    let r = check_task(ME, &Task::Take(7), Some(&free()), busy);
    assert_eq!(r, Err(TaskRefusal::OneTaskAtATime));
    let again = check_task(ME, &Task::Take(7), Some(&facts()), busy);
    assert!(again.is_ok(), "taking the task already held is fine");
}

#[test]
fn ready_is_for_free_open_current_phase_tasks_and_at_most_ten() {
    let ready = |facts: &TaskFacts, count: usize| {
        let backlog = Backlog {
            ready_count: count,
            ..backlog()
        };
        check_task(ME, &label("status/ready"), Some(facts), backlog).map(|_| ())
    };
    assert_eq!(ready(&free(), READY_MAX - 1), Ok(()));
    assert_eq!(ready(&free(), READY_MAX), Err(TaskRefusal::ReadyFull));
    let elsewhere = TaskFacts {
        milestone: Milestone::Elsewhere,
        ..free()
    };
    assert_eq!(ready(&elsewhere, 0), Err(TaskRefusal::OutsidePhase));
    let later = TaskFacts {
        milestone: Milestone::Later,
        ..free()
    };
    assert_eq!(ready(&later, 0), Err(TaskRefusal::TaskInLater));
    // Neither another agent's task nor the agent's own is flipped to ready.
    let other = TaskFacts {
        taken_by: Some("claude".to_owned()),
        ..free()
    };
    assert_eq!(ready(&other, 0), Err(TaskRefusal::TaskInProgress));
    assert_eq!(ready(&facts(), 0), Err(TaskRefusal::TaskInProgress));
}

#[test]
fn an_agent_sets_only_working_and_blocked_on_its_own_task() {
    for word in ["status/done", "status/in-later", "status/anything"] {
        let r = call(&label(word), &facts());
        assert_eq!(r, Err(TaskRefusal::LabelNotAllowed), "{word}");
    }
    assert_eq!(call(&label("status/working"), &facts()), Ok(()));
}

#[test]
fn urgent_is_never_put_on_an_existing_task() {
    let r = call(&label("status/urgent"), &facts());
    assert_eq!(r, Err(TaskRefusal::UrgentOnlyForNew));
}

#[test]
fn a_new_task_goes_to_later_and_is_never_ready() {
    let run = |label: &str| check_task(ME, &make(label, None), None, backlog());
    assert_eq!(run("status/blocked"), Ok(Some(Placement::Later)));
    assert_eq!(run("status/ready"), Err(TaskRefusal::NewTaskNotReady));
    assert_eq!(run("status/working"), Err(TaskRefusal::LabelNotAllowed));
}

#[test]
fn a_new_urgent_task_needs_a_reason_and_a_free_place() {
    let run = |reason: Option<&str>, urgent: usize| {
        let backlog = Backlog {
            urgent_count: urgent,
            ..backlog()
        };
        check_task(ME, &make("status/urgent", reason), None, backlog)
    };
    for reason in ["broken-rule", "failed-sync", "failed-release", "security"] {
        let placed = run(Some(reason), URGENT_MAX - 1);
        assert_eq!(placed, Ok(Some(Placement::CurrentPhase)), "{reason}");
    }
    assert_eq!(run(None, 0), Err(TaskRefusal::UrgentNeedsReason));
    assert_eq!(
        run(Some("new scope"), 0),
        Err(TaskRefusal::UrgentNeedsReason)
    );
    assert_eq!(
        run(Some("security"), URGENT_MAX),
        Err(TaskRefusal::UrgentFull)
    );
}

#[test]
fn a_call_on_a_task_the_harness_cannot_find_is_refused() {
    let r = check_task(ME, &Task::Close(7), None, backlog());
    assert_eq!(r, Err(TaskRefusal::UnknownTask));
}
