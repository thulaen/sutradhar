//! REQ-0155-LIMITS: the rates and connection counts (spec 0155,
//! "Limits"), at, below and above each, on plain data and a given clock.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use agent_harness::agent::AgentName;
use agent_harness::limits::{
    Class, HOUR_MS, LimitRefusal, Limiter, PER_AGENT_CONNECTIONS,
    TOTAL_CONNECTIONS,
};

fn who(name: &str) -> AgentName {
    AgentName::new(name).unwrap()
}

fn burst(limiter: &mut Limiter, agent: &str, class: Class, n: usize) {
    for i in 0..n {
        let now = u64::try_from(i).unwrap();
        limiter.allow(&who(agent), class, now).unwrap();
    }
}

#[test]
fn each_class_allows_its_rate_and_refuses_the_next_request() {
    let cases = [
        (Class::Push, 12, LimitRefusal::PushRate),
        (Class::Task, 240, LimitRefusal::TaskRate),
        (Class::Mailbox, 600, LimitRefusal::MailboxRate),
        (Class::Refused, 120, LimitRefusal::RefusedRate),
    ];
    for (class, rate, refusal) in cases {
        let mut limiter = Limiter::new();
        burst(&mut limiter, "codex", class, rate);
        let over = limiter.allow(&who("codex"), class, 1_000);
        assert_eq!(over, Err(refusal), "{class:?}");
    }
}

#[test]
fn the_window_slides_and_one_agent_does_not_use_up_another() {
    let mut limiter = Limiter::new();
    burst(&mut limiter, "codex", Class::Push, 12);
    assert!(limiter.allow(&who("claude"), Class::Push, 0).is_ok());
    assert!(
        limiter
            .allow(&who("codex"), Class::Push, HOUR_MS - 1)
            .is_err()
    );
    // The first requests were at 0 and 1 ms; an hour later they are out.
    assert!(limiter.allow(&who("codex"), Class::Push, HOUR_MS).is_ok());
    assert!(limiter.allow(&who("codex"), Class::Push, HOUR_MS).is_err());
    assert!(
        limiter
            .allow(&who("codex"), Class::Push, HOUR_MS + 1)
            .is_ok()
    );
    assert!(
        limiter
            .allow(&who("codex"), Class::Push, HOUR_MS + 1)
            .is_err()
    );
}

#[test]
fn the_classes_are_counted_apart() {
    let mut limiter = Limiter::new();
    burst(&mut limiter, "codex", Class::Push, 12);
    assert!(limiter.allow(&who("codex"), Class::Task, 5).is_ok());
    assert!(limiter.allow(&who("codex"), Class::Mailbox, 5).is_ok());
}

#[test]
fn a_clock_that_goes_backwards_never_lets_extra_requests_through() {
    let mut limiter = Limiter::new();
    for _ in 0..12 {
        limiter
            .allow(&who("codex"), Class::Push, 10 * HOUR_MS)
            .unwrap();
    }
    assert!(limiter.allow(&who("codex"), Class::Push, 0).is_err());
}

#[test]
fn connections_are_capped_for_each_agent_and_in_all() {
    let mut limiter = Limiter::new();
    for _ in 0..PER_AGENT_CONNECTIONS {
        limiter.open(&who("codex")).unwrap();
    }
    assert_eq!(
        limiter.open(&who("codex")),
        Err(LimitRefusal::AgentConnections)
    );
    limiter.close(&who("codex"));
    assert!(limiter.open(&who("codex")).is_ok());
    let mut all = Limiter::new();
    for i in 0..TOTAL_CONNECTIONS {
        let name = format!("agent{}", i / PER_AGENT_CONNECTIONS);
        all.open(&who(&name)).unwrap();
    }
    assert_eq!(all.open(&who("other")), Err(LimitRefusal::TotalConnections));
}

#[test]
fn closing_more_than_was_opened_does_not_go_below_zero() {
    let mut limiter = Limiter::new();
    limiter.close(&who("codex"));
    for _ in 0..PER_AGENT_CONNECTIONS {
        limiter.open(&who("codex")).unwrap();
    }
    assert!(limiter.open(&who("codex")).is_err());
}
