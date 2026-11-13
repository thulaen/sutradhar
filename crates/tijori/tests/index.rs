//! Tests for the log's indexes (spec 0065 "The signature log" items 3
//! and 4).
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use tijori::index::Index;

fn key(text: &str) -> Vec<u8> {
    text.as_bytes().to_vec()
}

#[test]
fn finds_a_counter_by_its_key() {
    let mut index = Index::new();
    index.insert(key("agent|review-record|h1"), 4).unwrap();
    assert_eq!(index.get(&key("agent|review-record|h1")), Some(4));
    assert_eq!(index.get(&key("agent|review-record|h2")), None);
}

#[test]
fn a_counter_that_goes_back_or_repeats_is_refused() {
    use tijori::index::CounterWentBack;
    let mut index = Index::new();
    index.insert(key("a"), 10).unwrap();
    let before = index.running();
    assert_eq!(
        index.insert(key("b"), 3),
        Err(CounterWentBack {
            latest: 10,
            counter: 3
        })
    );
    assert!(index.insert(key("b"), 10).is_err());
    assert_eq!((index.latest(), index.running()), (10, before));
    assert_ne!(index.insert(key("b"), 3).unwrap_err().to_string(), "");
}

#[test]
fn a_later_entry_for_the_same_key_replaces_the_earlier() {
    let mut index = Index::new();
    index.insert(key("k"), 4).unwrap();
    index.insert(key("k"), 9).unwrap();
    assert_eq!(index.get(&key("k")), Some(9));
}

#[test]
fn a_rebuild_in_the_same_order_reaches_the_same_running_hash() {
    let mut a = Index::new();
    let mut b = Index::new();
    for (k, c) in [("x", 1), ("y", 2), ("x", 3)] {
        a.insert(key(k), c).unwrap();
        b.insert(key(k), c).unwrap();
    }
    assert!(b.matches(a.latest(), &a.running()));
}

#[test]
fn a_missing_or_reordered_entry_gives_another_running_hash() {
    let mut full = Index::new();
    full.insert(key("x"), 1).unwrap();
    full.insert(key("y"), 2).unwrap();
    let mut missing = Index::new();
    missing.insert(key("y"), 2).unwrap();
    let mut reordered = Index::new();
    reordered.insert(key("y"), 1).unwrap();
    reordered.insert(key("x"), 2).unwrap();
    assert!(!missing.matches(full.latest(), &full.running()));
    assert!(!reordered.matches(full.latest(), &full.running()));
}

#[test]
fn keys_that_join_the_same_way_still_differ() {
    // "ab" + counter differs from "a" + "b..." because the key's length
    // is part of the running hash.
    let mut a = Index::new();
    a.insert(key("ab"), 1).unwrap();
    let mut b = Index::new();
    b.insert(key("a"), 1).unwrap();
    assert_ne!(a.running(), b.running());
}

#[test]
fn an_index_behind_the_log_does_not_match() {
    let mut index = Index::new();
    index.insert(key("x"), 1).unwrap();
    let at_one = index.running();
    index.insert(key("y"), 2).unwrap();
    assert!(!index.matches(1, &at_one));
}

#[test]
fn next_changes_nothing_and_refuses_a_counter_that_goes_back() {
    let mut index = Index::new();
    index.insert(key("a"), 3).unwrap();
    let before = (index.latest(), index.running());
    assert!(index.next(&key("b"), 3).is_err());
    let _checked = index.next(&key("b"), 4).unwrap();
    assert_eq!((index.latest(), index.running()), before);
    assert_eq!(index.get(&key("b")), None);
}

#[test]
fn next_then_apply_matches_insert() {
    let mut one = Index::new();
    let mut two = Index::new();
    one.insert(key("a"), 1).unwrap();
    let next = two.next(&key("a"), 1).unwrap();
    assert_eq!(next.running(), one.running());
    two.apply(key("a"), next);
    assert_eq!((two.latest(), two.running()), (one.latest(), one.running()));
    assert_eq!(two.get(&key("a")), Some(1));
}
