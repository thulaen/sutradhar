//! IT-0155-LOG: the log is append-only and each line holds the hash of
//! the line before, so a cut or a changed line shows (spec 0155, "The
//! log"), on plain text.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use agent_harness::agent::AgentName;
use agent_harness::log_chain::{
    Chain, ChainError, Done, Kind, Summary, entry, verify,
};
use agent_harness::request::Refusal;

fn me() -> AgentName {
    AgentName::new("codex").unwrap()
}

/// A log of `n` lines, each saying how many requests were refused.
fn log(n: u64) -> (Chain, Vec<String>) {
    let mut chain = Chain::new();
    let lines = (0..n)
        .map(|i| chain.append(&entry(&me(), Kind::Summary, &Summary(i))))
        .collect();
    (chain, lines)
}

#[test]
fn a_fresh_chain_verifies_and_every_line_links_to_the_one_before() {
    let (chain, lines) = log(5);
    let head = verify(&lines).unwrap();
    assert_eq!(head, chain.head());
    assert_eq!(head.count, 5);
    assert_eq!(verify(&[]).unwrap().count, 0);
}

#[test]
fn a_changed_line_is_found() {
    let (_, mut lines) = log(4);
    lines[1] = lines[1].replace("refused 1 ", "refused 9 ");
    assert_eq!(verify(&lines), Err(ChainError::BadHash(2)));
}

#[test]
fn a_cut_in_the_middle_a_swap_and_a_gap_are_found() {
    let (_, lines) = log(5);
    let mut cut = lines.clone();
    let _gone = cut.remove(2);
    assert!(verify(&cut).is_err());
    let mut swapped = lines.clone();
    swapped.swap(1, 2);
    assert!(verify(&swapped).is_err());
    assert!(verify(&lines[1..]).is_err());
}

#[test]
fn a_line_number_written_another_way_is_not_the_same_line() {
    let (_, lines) = log(3);
    for other in ["01 ", "+1 ", "001 "] {
        let mut changed = lines.clone();
        changed[0] = changed[0].replacen("1 ", other, 1);
        assert_eq!(
            verify(&changed),
            Err(ChainError::BadSequence(1)),
            "{other}"
        );
    }
}

#[test]
fn a_cut_tail_shows_against_the_owners_copy_of_the_newest_line() {
    let (chain, lines) = log(5);
    let copy = chain.head();
    assert_eq!(copy.matches_copy(&lines), Ok(()));
    assert_eq!(copy.matches_copy(&lines[..3]), Err(ChainError::TailCut));
    // A log that has grown since the copy still matches it.
    let mut longer = lines.clone();
    let more = entry(&me(), Kind::Push, &Done::Landed);
    longer.push(chain.clone().append(&more));
    assert_eq!(copy.matches_copy(&longer), Ok(()));
}

#[test]
fn a_rewritten_history_of_the_same_length_shows_against_the_copy() {
    let (chain, _) = log(3);
    let copy = chain.head();
    let mut other = Chain::new();
    let forged: Vec<String> = (0..3)
        .map(|_| other.append(&entry(&me(), Kind::Push, &Done::Answered)))
        .collect();
    assert_eq!(copy.matches_copy(&forged), Err(ChainError::HistoryChanged));
}

#[test]
fn an_entry_holds_only_an_agent_a_kind_and_a_fixed_outcome() {
    let line = entry(&me(), Kind::Push, &Done::Landed);
    assert_eq!(line.as_str(), "codex push landed");
    let refused = entry(&me(), Kind::Task, &Refusal::TooBig);
    assert_eq!(
        refused.as_str(),
        "codex task the request is over the size limit"
    );
    assert!(AgentName::new("Codex").is_none());
    assert!(AgentName::new("two words").is_none());
    assert!(AgentName::new("").is_none());
}

#[test]
fn a_line_that_is_not_in_the_format_is_refused() {
    for bad in ["", "1", "1 a b", "x y z w", "1 0 0 entry extra\nline"] {
        assert!(verify(&[bad.to_owned()]).is_err(), "{bad:?}");
    }
}
