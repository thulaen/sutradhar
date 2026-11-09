//! Fuzz tests: random text, Rust-like and not, must never stop the
//! readers, and what they record must keep its promised shape. The
//! default is 256 cases each; the release tier sets `PROPTEST_CASES`
//! higher.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use nirikshak::facts::Facts;
use proptest::prelude::*;

fn rust_like() -> impl Strategy<Value = String> {
    let piece = prop_oneof![
        Just("fn f() {".to_owned()),
        Just("}".to_owned()),
        Just("let _ = g();".to_owned()),
        Just("x.unwrap();".to_owned()),
        Just("assert!(a);".to_owned()),
        Just("\"text\"".to_owned()),
        Just("#[test]".to_owned()),
        Just("std::env::var(\"A\");".to_owned()),
        "\\PC{0,12}",
    ];
    proptest::collection::vec(piece, 0..40).prop_map(|p| p.join("\n"))
}

proptest! {
    #[test]
    fn any_text_is_read_or_reported_never_a_crash(text in rust_like()) {
        let files = vec![
            ("src/a.rs".to_owned(), text.clone()),
            ("docs/a.md".to_owned(), text.clone()),
            ("a.yaml".to_owned(), text),
        ];
        let report = nirikshak::analyse(&files);
        for found in &report.found {
            prop_assert!(found.line >= 1);
            prop_assert!(files.iter().any(|(p, _)| *p == found.path));
        }
    }

    #[test]
    fn code_spans_never_hold_a_backtick(text in "\\PC{0,300}") {
        let mut facts = Facts::default();
        nirikshak::markdown_facts::read("a.md", &text, &mut facts);
        for span in facts.of("doc_code") {
            let t = span.get("text").and_then(|v| v.as_str()).unwrap();
            prop_assert!(!t.contains('`') && !t.is_empty());
        }
    }

    #[test]
    fn declared_settings_are_always_plain_names(
        text in "[ a-zA-Z_:=${}\\-\n#]{0,300}"
    ) {
        let mut facts = Facts::default();
        nirikshak::yaml_facts::read("a.yaml", &text, &mut facts);
        for setting in facts.of("env_declared") {
            let name = setting.get("name").and_then(|v| v.as_str()).unwrap();
            prop_assert!(!name.is_empty());
            let plain = |c: char| c.is_ascii_alphanumeric() || c == '_';
            prop_assert!(name.chars().all(plain));
        }
    }
}
