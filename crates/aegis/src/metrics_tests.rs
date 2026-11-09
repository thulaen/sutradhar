use super::{Names, check, names, parts};
use crate::Finding;
use crate::file_kinds::Kind;

const LIST: &str = r#"
sources = ["search-console", "ga4"]
vanity_names = ["total_visits", "page_views"]

[[metric]]
id = "clicks_change_after_update"
source = "search-console"
decision = "Which pages to review after an update."
compared_with = "the same pages over the 28 days before"
"#;

fn loaded(text: &str) -> (Names, Vec<Finding>) {
    let mut out = Vec::new();
    let n = names(Some(text), &mut out);
    (n, out)
}

fn lines(path: &str, kind: Kind, text: &str) -> Vec<usize> {
    let (n, _) = loaded(LIST);
    check(path, kind, text, &n).iter().map(|f| f.line).collect()
}

#[test]
fn names_split_the_same_in_every_spelling() {
    for name in ["pageViews", "page_views", "page-views", "PAGE_VIEWS"] {
        assert_eq!(parts(name), ["page", "views"], "{name}");
    }
    assert_eq!(parts("HTTPServer2go"), ["httpserver2go"]);
}

#[test]
fn a_vanity_name_in_any_spelling_is_refused_with_its_line() {
    let text = "fn ok() {}\nlet totalVisits = 1;\nlet x = 2;\n\
                const MONTHLY_PAGE_VIEWS: u8 = 3;\n.page-views { }\n";
    assert_eq!(lines("crates/a/src/lib.rs", Kind::Rust, text), [2, 4, 5]);
}

#[test]
fn honest_names_tests_docs_and_the_list_itself_pass() {
    let honest = "let clicks_change_after_update = 1;\nlet pages = 2;\n\
                  let total = visits_left;\n";
    assert_eq!(
        lines("crates/a/src/lib.rs", Kind::Rust, honest),
        Vec::<usize>::new()
    );
    let vain = "let total_visits = 1;\n";
    for path in [
        "crates/a/tests/t.rs",
        "crates/a/src/a_tests.rs",
        "tests/x.rs",
    ] {
        assert_eq!(
            lines(path, Kind::Rust, vain),
            Vec::<usize>::new(),
            "{path}"
        );
    }
    assert_eq!(
        lines("docs/a.md", Kind::Markdown, vain),
        Vec::<usize>::new()
    );
    assert_eq!(lines(super::PATH, Kind::Config, vain), Vec::<usize>::new());
    assert_eq!(lines("a.sql", Kind::Sql, vain), [1]);
}

#[test]
fn a_bad_or_missing_list_blocks() {
    let mut out = Vec::new();
    let _n = names(None, &mut out);
    assert_eq!(out.len(), 1);
    let (_n, out) = loaded("sources = [\n");
    assert_eq!(out.len(), 1);
    let (_n, out) = loaded("sources = []\nvanity_names = []\n");
    assert_eq!(out.len(), 1, "{out:?}");
}

#[test]
fn every_listed_number_needs_a_source_a_decision_and_a_comparison() {
    let bad = r#"
sources = ["ga4"]
vanity_names = ["page_views"]

[[metric]]
id = "pageViews"
source = "bing"
decision = "Look at it."
compared_with = " "

[[metric]]
id = "pageViews"
source = "ga4"
decision = "Whether a section still does its job."
compared_with = "last month"
"#;
    let (_n, out) = loaded(bad);
    let messages: Vec<&str> = out.iter().map(|f| f.message.as_str()).collect();
    for want in [
        "is a vanity name",
        "no single approved source",
        "no real decision",
        "nothing to compare",
        "listed twice",
    ] {
        assert!(messages.iter().any(|m| m.contains(want)), "{want}");
    }
}
