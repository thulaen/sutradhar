use super::{DOCS_ONLY, Tdd, check_paths, parse, value};

#[test]
fn reads_a_test_path_and_docs_only() {
    assert_eq!(
        parse("shodhan::score::tests::overlap_ranks_exact_phrase_first"),
        Some(Tdd::Test {
            krate: "shodhan".to_owned(),
            path: "score::tests::overlap_ranks_exact_phrase_first".to_owned(),
        })
    );
    assert_eq!(
        parse("chaukidar::watchman::a_test"),
        Some(Tdd::Test {
            krate: "chaukidar".to_owned(),
            path: "watchman::a_test".to_owned(),
        })
    );
    assert_eq!(parse(DOCS_ONLY), Some(Tdd::DocsOnly));
}

#[test]
fn refuses_every_other_form() {
    for bad in [
        "aegis test refuses_x",
        "none",
        "none (a probe in a script)",
        "shodhan",
        "shodhan::",
        "::x",
        "Shodhan::x",
        "shodhan::x y",
        "shodhan::x::",
        "shodhan::9x",
        "forge-sync::x",
    ] {
        assert_eq!(parse(bad), None, "{bad}");
    }
}

#[test]
fn finds_the_last_tdd_line() {
    let m = "feat: x\n\nTDD: a::b\nDefects: none\n";
    assert_eq!(value(m).as_deref(), Some("a::b"));
    let body = "feat: x\n\nTDD: none (docs only)\n\nTDD: a::b\n";
    assert_eq!(value(body).as_deref(), Some("a::b"));
    assert_eq!(value("feat: x\n"), None);
}

#[test]
fn docs_only_commits_may_touch_only_markdown() {
    let m = format!("docs: x\n\nTDD: {DOCS_ONLY}\n");
    let docs = ["README.md".to_owned(), "docs/specs/a.MD".to_owned()];
    assert_eq!(check_paths(&m, &docs), Vec::new());
    let code = ["README.md".to_owned(), "crates/a/src/lib.rs".to_owned()];
    let found = check_paths(&m, &code);
    assert_eq!(found.len(), 1);
    assert_eq!(
        found.first().map(|f| f.path.as_str()),
        Some(code[1].as_str())
    );
}

#[test]
fn a_named_test_may_touch_any_path() {
    let m = "feat: x\n\nTDD: a::b\n";
    assert_eq!(check_paths(m, &["src/lib.rs".to_owned()]), Vec::new());
}

#[test]
fn a_second_tdd_line_cannot_hide_code_in_a_docs_only_commit() {
    let m = format!(
        "docs: x\n\nTDD: {DOCS_ONLY}\nTDD: a::b\nDefects: none\nTask: #1\n\
         Signed-off-by: A <a@example.com>\n"
    );
    assert_ne!(crate::commit_msg::check(&m), Vec::new());
    assert_eq!(
        check_paths(&m, &["src/lib.rs".to_owned()]).len(),
        1,
        "the first TDD line rules, as in the message check"
    );
}
