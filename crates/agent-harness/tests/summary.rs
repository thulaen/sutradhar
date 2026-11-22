//! A flood of refused requests is written as one line a minute for each
//! agent, so it cannot push earlier lines out of the log (spec 0155,
//! "The log").
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use agent_harness::agent::AgentName;
use agent_harness::summary::{MINUTE_MS, Summariser};

fn who(name: &str) -> AgentName {
    AgentName::new(name).unwrap()
}

fn nothing() -> Vec<(AgentName, u64)> {
    Vec::new()
}

#[test]
fn refusals_in_one_minute_come_out_as_one_count_for_each_agent() {
    let mut s = Summariser::new();
    for i in 0..5 {
        assert_eq!(s.note(&who("codex"), i * 1000), nothing());
    }
    assert_eq!(s.note(&who("claude"), 7_000), nothing());
    let out = s.note(&who("codex"), MINUTE_MS + 1);
    assert_eq!(out, vec![(who("claude"), 1), (who("codex"), 5)]);
}

#[test]
fn the_new_minute_starts_counting_again() {
    let mut s = Summariser::new();
    let _ = s.note(&who("codex"), 0);
    let first = s.note(&who("codex"), MINUTE_MS);
    assert_eq!(first, vec![(who("codex"), 1)]);
    let second = s.note(&who("codex"), 2 * MINUTE_MS);
    assert_eq!(second, vec![(who("codex"), 1)]);
}

#[test]
fn a_flush_gives_what_is_left_and_empties() {
    let mut s = Summariser::new();
    let _ = s.note(&who("codex"), 0);
    let _ = s.note(&who("codex"), 1);
    assert_eq!(s.flush(), vec![(who("codex"), 2)]);
    assert_eq!(s.flush(), nothing());
}

#[test]
fn a_clock_that_goes_backwards_keeps_counting_in_the_same_minute() {
    let mut s = Summariser::new();
    let _ = s.note(&who("codex"), 5 * MINUTE_MS);
    assert_eq!(s.note(&who("codex"), 0), nothing());
    assert_eq!(s.flush(), vec![(who("codex"), 2)]);
}
