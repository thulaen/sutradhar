#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use super::review_record::check;
use super::{Entry, REQUIRED, fingerprint, is_record};
use std::collections::HashMap;

fn e(mode: &str, id: &str, path: &[u8]) -> Entry {
    (mode.to_owned(), id.to_owned(), path.to_vec())
}

fn record(skill: &str, change: &str, verdict: &str) -> String {
    format!(
        "skill = \"{skill}\"\nchange = \"{change}\"\nverdict = \"{verdict}\"\n\
         agent = \"claude\"\nsummary = \"Read every changed line twice.\"\n"
    )
}

#[test]
fn the_fingerprint_covers_the_tree_but_not_the_records() {
    let code = [e("100644", "a1", b"src/a.rs")];
    let with_record = [
        e("100644", "a1", b"src/a.rs"),
        e("100644", "ff", b"reviews/red-team.toml"),
    ];
    assert_eq!(fingerprint(&code), fingerprint(&with_record));
    let other_file = [
        e("100644", "a1", b"src/a.rs"),
        e("100644", "ff", b"reviews/payload.rs"),
    ];
    assert_ne!(fingerprint(&code), fingerprint(&other_file));
    // git lists a tree in one fixed order, so order counts: a tree with a
    // name twice cannot pose as the same tree with the two swapped.
    let twice = [e("100644", "b2", b"a.md"), e("100644", "a1", b"a.md")];
    let swapped = [e("100644", "a1", b"a.md"), e("100644", "b2", b"a.md")];
    assert_ne!(fingerprint(&twice), fingerprint(&swapped));
    assert_ne!(
        fingerprint(&[e("100755", "a1", b"x")]),
        fingerprint(&[e("100644", "a1", b"x")])
    );
}

#[test]
fn a_name_holding_a_line_break_cannot_pose_as_two_files() {
    let two = [e("100644", "a1", b"x"), e("100644", "b2", b"y")];
    let posing = [e("100644", "a1", b"x\n100644 b2 1:y")];
    assert_ne!(fingerprint(&two), fingerprint(&posing));
    let odd = [e("100644", "a1", b"x\xff")];
    let other = [e("100644", "a1", b"x\xfe")];
    assert_ne!(fingerprint(&odd), fingerprint(&other));
}

fn judged(records: &HashMap<String, (String, String)>) -> Vec<String> {
    let entries: Vec<Entry> = records
        .iter()
        .map(|(p, (mode, _))| e(mode, "0", p.as_bytes()))
        .collect();
    let read =
        |p: &str| records.get(p).map(|(_, t)| Ok(t.clone().into_bytes()));
    check("sha256:c", &entries, &read, "commit")
        .iter()
        .map(|f| f.message.clone())
        .collect()
}

fn all() -> HashMap<String, (String, String)> {
    REQUIRED
        .iter()
        .map(|s| {
            let text = record(s, "sha256:c", "pass");
            (format!("reviews/{s}.toml"), ("100644".to_owned(), text))
        })
        .collect()
}

#[test]
fn every_way_a_record_can_fail_is_refused() {
    assert_eq!(judged(&all()), Vec::<String>::new());
    let mut r = all();
    let set = |r: &mut HashMap<_, _>, skill: &str, mode: &str, text: String| {
        let _old =
            r.insert(format!("reviews/{skill}.toml"), (mode.to_owned(), text));
    };
    let _gone = r.remove("reviews/red-team.toml");
    set(
        &mut r,
        "defects",
        "100644",
        record("defects", "sha256:old", "pass"),
    );
    set(
        &mut r,
        "security",
        "100644",
        record("security", "sha256:c", "fail"),
    );
    let open = "[[finding]]\nwhat = \"a hole\"\nfixed = false\n";
    set(
        &mut r,
        "unsafe-patterns",
        "100644",
        format!("{}{open}", record("unsafe-patterns", "sha256:c", "pass")),
    );
    let blank = "[[finding]]\nwhat = \" \"\nfixed = false\n";
    set(
        &mut r,
        "backlog",
        "100644",
        format!("{}{blank}", record("backlog", "sha256:c", "pass")),
    );
    set(
        &mut r,
        "style-docs",
        "120000",
        record("style-docs", "sha256:c", "pass"),
    );
    let hidden = record("architecture", "sha256:c", "pass")
        .replace("claude", "\u{200b}");
    set(&mut r, "architecture", "100644", hidden);
    set(&mut r, "fast-performance", "100644", "x".repeat(70_000));
    let got = judged(&r);
    for want in [
        "no `red-team` review",
        "made for other content",
        "did not pass",
        "are not fixed",
        "does not say what was wrong",
        "must be a plain file",
        "must name its agent",
        "too large",
    ] {
        assert!(got.iter().any(|m| m.contains(want)), "{want}: {got:?}");
    }
    assert_eq!(got.len(), 8);
}

#[test]
fn text_from_a_record_cannot_rewrite_the_screen() {
    let mut shouting = all();
    let _old = shouting.insert(
        "reviews/red-team.toml".to_owned(),
        ("100644".to_owned(), record("\u{1b}[2J", "sha256:c", "pass")),
    );
    assert!(judged(&shouting).iter().all(|m| !m.contains('\u{1b}')));
}

#[test]
fn only_the_eight_record_files_are_records() {
    for skill in REQUIRED {
        assert!(is_record(format!("reviews/{skill}.toml").as_bytes()));
    }
    for path in [
        "reviews/defects.toml.bak",
        "reviews/defects",
        "reviews/sub/defects.toml",
        "xreviews/defects.toml",
        "reviews/other.toml",
        "reviews/.toml",
        "defects.toml",
        "",
    ] {
        assert!(!is_record(path.as_bytes()), "{path}");
    }
}
