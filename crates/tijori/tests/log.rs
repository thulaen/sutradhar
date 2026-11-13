//! Tests for the signature log's hash chain (spec 0065 "The signature
//! log").
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use tijori::log::{Chain, Entry, Head, Kind, LogError, check};

fn three() -> Vec<Entry> {
    let mut chain = Chain::new();
    vec![
        chain
            .append(Kind::PolicyApplied, 100, b"v1".to_vec())
            .unwrap(),
        chain.append(Kind::Signature, 105, b"sig".to_vec()).unwrap(),
        chain
            .append(Kind::Signature, 110, b"sig2".to_vec())
            .unwrap(),
    ]
}

#[test]
fn counters_start_at_one_and_go_up_by_one() {
    let entries = three();
    let counters: Vec<u64> = entries.iter().map(|e| e.counter).collect();
    assert_eq!(counters, [1, 2, 3]);
}

#[test]
fn each_entry_names_the_hash_of_the_one_before() {
    let entries = three();
    assert_eq!(entries.first().unwrap().previous, [0; 32]);
    for pair in entries.windows(2) {
        let [a, b] = pair else { unreachable!() };
        assert_eq!(b.previous, a.hash());
    }
}

#[test]
fn a_whole_log_checks_and_reaches_the_chain_head() {
    let mut chain = Chain::new();
    let entries: Vec<Entry> = (0..5)
        .map(|i| chain.append(Kind::Signature, i, vec![]).unwrap())
        .collect();
    assert_eq!(check(&entries), Ok(chain.head()));
}

#[test]
fn changing_an_earlier_entry_breaks_every_later_link() {
    let mut entries = three();
    entries.first_mut().unwrap().body = b"v2".to_vec();
    assert_eq!(check(&entries), Err(LogError::BrokenChain { counter: 2 }));
}

#[test]
fn a_missing_entry_is_refused() {
    let mut entries = three();
    let _ = entries.remove(1);
    assert_eq!(check(&entries), Err(LogError::CounterGap { expected: 2 }));
}

#[test]
fn log_times_never_go_back_even_when_the_clock_does() {
    let mut chain = Chain::new();
    let a = chain.append(Kind::Signature, 500, vec![]).unwrap();
    let b = chain.append(Kind::Signature, 400, vec![]).unwrap();
    assert_eq!((a.time, b.time), (500, 500));
}

#[test]
fn a_time_that_goes_back_is_refused_by_the_check() {
    let mut entries = three();
    entries.get_mut(2).unwrap().time = 1;
    assert_eq!(check(&entries), Err(LogError::TimeWentBack { counter: 3 }));
}

#[test]
fn the_counter_never_wraps() {
    let mut chain = Chain::resume(
        Head {
            counter: u64::MAX,
            hash: [7; 32],
        },
        0,
    );
    assert_eq!(
        chain.append(Kind::Signature, 0, vec![]),
        Err(LogError::CounterExhausted)
    );
    assert_eq!(chain.head().map(|h| h.counter), Some(u64::MAX));
}

#[test]
fn the_kind_is_part_of_the_hash() {
    let mut a = Chain::new();
    let mut b = Chain::new();
    let x = a.append(Kind::Signature, 1, vec![]).unwrap();
    let y = b.append(Kind::Recovery, 1, vec![]).unwrap();
    assert_ne!(x.hash(), y.hash());
}

#[test]
fn every_error_explains_itself_in_plain_english() {
    for error in [
        LogError::CounterExhausted,
        LogError::CounterGap { expected: 2 },
        LogError::BrokenChain { counter: 2 },
        LogError::TimeWentBack { counter: 2 },
    ] {
        assert_ne!(error.to_string(), "");
    }
}
