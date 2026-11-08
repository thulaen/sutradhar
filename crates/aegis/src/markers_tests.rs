use super::check;
use crate::file_kinds::Kind;

/// The marker words, built so this file holds none of them whole.
fn word(n: usize) -> String {
    super::markers().get(n).cloned().unwrap_or_default()
}

#[test]
fn a_bare_marker_is_refused_with_its_line() {
    let text =
        format!("fn f() {{}}\n// {} speed up\n// {}\n", word(0), word(1));
    let found = check("a.rs", Kind::Rust, &text);
    assert_eq!(found.len(), 2);
    assert_eq!(found.first().map(|f| f.line), Some(2));
    assert_eq!(found.get(1).map(|f| f.line), Some(3));
}

#[test]
fn every_marker_word_counts() {
    for n in 0..4 {
        let text = format!("# {} later\n", word(n));
        assert_eq!(check("x.sh", Kind::Shell, &text).len(), 1, "{n}");
    }
}

#[test]
fn a_marker_that_names_a_forgejo_task_is_fine() {
    let text = format!("// {}(#16): use the faster sort\n", word(0));
    assert_eq!(check("a.rs", Kind::Rust, &text), Vec::new());
}

#[test]
fn a_malformed_task_number_does_not_count() {
    for tail in ["(#)", "(16)", "(#16", " #16", "(#x)", "(AI-12)"] {
        let text = format!("// {}{tail}: x\n", word(0));
        assert_eq!(check("a.rs", Kind::Rust, &text).len(), 1, "{tail}");
    }
}

#[test]
fn words_that_only_contain_a_marker_are_ignored() {
    let text = format!(
        "{0}S and {1} and MY{0} and {2}s and todo\n",
        word(0),
        word(0).to_lowercase(),
        word(1)
    );
    assert_eq!(check("a.rs", Kind::Rust, &text), Vec::new());
}

#[test]
fn markdown_licences_and_lock_files_are_left_to_their_own_checks() {
    let text = format!("{} list\n", word(0));
    assert_eq!(check("a.md", Kind::Markdown, &text), Vec::new());
    assert_eq!(check("LICENSE", Kind::Licence, &text), Vec::new());
    assert_eq!(check("Cargo.lock", Kind::Lock, &text), Vec::new());
    assert_eq!(check("a.toml", Kind::Config, &text).len(), 1);
}

#[test]
fn two_markers_on_one_line_are_two_findings() {
    let text = format!("// {} and {}\n", word(0), word(1));
    assert_eq!(check("a.rs", Kind::Rust, &text).len(), 2);
}

#[test]
fn words_joined_by_an_underscore_or_a_letter_are_ignored() {
    let text =
        format!("{0}_list and my_{0} and \u{e9}{0} and {0}\u{e9}\n", word(0));
    assert_eq!(check("a.rs", Kind::Rust, &text), Vec::new());
}

#[test]
fn the_task_number_must_follow_at_once() {
    let spaced = format!("// {} (#16): x\n", word(0));
    assert_eq!(check("a.rs", Kind::Rust, &spaced).len(), 1);
    let padded = format!("// {}(#016): x\n", word(0));
    assert_eq!(check("a.rs", Kind::Rust, &padded), Vec::new());
}
