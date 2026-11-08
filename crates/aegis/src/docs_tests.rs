use super::{Lookup, check, resolve, slug};

fn lookup(path: &str) -> Lookup {
    match path {
        "docs/a.md" => Lookup::File("# A title\n## Section 4.1 Data\n".into()),
        "docs" | "docs/adr" => Lookup::Folder,
        "README.md" => Lookup::File("# Read me\n".into()),
        _ => Lookup::Missing,
    }
}

fn found(text: &str) -> Vec<&'static str> {
    check("README.md", text, &lookup)
        .into_iter()
        .map(|f| f.check)
        .collect()
}

#[test]
fn clean_text_passes() {
    let text = "# Title\n\nSee [a](docs/a.md#section-41-data), \
        [b](docs/adr/),\n\
        [c](#read-me), [d](https://example.com) and `status/ready`.\n";
    assert_eq!(found(text).len(), 0);
}

#[test]
fn refuses_a_long_line() {
    assert_eq!(found(&"x".repeat(81)), ["docs.line-length"]);
}

#[test]
fn long_tables_urls_and_code_are_exempt() {
    let text = format!(
        "| {} |\nhttps://{}\n```\n{}\n```\n",
        "x".repeat(90),
        "x".repeat(90),
        "x".repeat(90)
    );
    assert_eq!(found(&text).len(), 0);
}

#[test]
fn refuses_status_words() {
    let marker = ["TO", "DO: x"].concat();
    for w in ["This is in progress.", "Work in progress", &marker, "wip"] {
        assert_eq!(found(w), ["docs.status-words"], "{w}");
    }
}

#[test]
fn status_words_in_code_spans_are_allowed() {
    assert_eq!(found("the `todo` label").len(), 0);
}

#[test]
fn refuses_missing_files_and_anchors() {
    assert_eq!(found("[x](nope.md)"), ["docs.links"]);
    assert_eq!(found("[x](docs/a.md#nope)"), ["docs.links"]);
    assert_eq!(found("[x](#nope)"), ["docs.links"]);
    assert_eq!(found("[x](../../up.md)"), ["docs.links"]);
}

#[test]
fn folders_and_mail_links_pass() {
    assert_eq!(found("[x](docs) [y](mailto:a@example.com)").len(), 0);
}

#[test]
fn resolves_relative_paths() {
    assert_eq!(
        resolve("docs/adr/x.md", "../../AGENTS.md").as_deref(),
        Some("AGENTS.md")
    );
    assert_eq!(
        resolve("docs/x.md", "./a/../b.md").as_deref(),
        Some("docs/b.md")
    );
    assert_eq!(resolve("x.md", "../a.md"), None);
}

#[test]
fn makes_github_slugs() {
    assert_eq!(slug("5.1 Sutradhar Boot"), "51-sutradhar-boot");
    assert_eq!(
        slug("11. The shared computer (Dell)"),
        "11-the-shared-computer-dell"
    );
}

#[test]
fn an_unpaired_backtick_is_plain_text() {
    assert_eq!(found("Use `x and [y](nope.md)"), ["docs.links"]);
    assert_eq!(found("a `b` c `d [y](nope.md)"), ["docs.links"]);
}

#[test]
fn anchors_follow_github() {
    let text = "# my_func\n## Setup\n## Setup\n## Title ##\n#!/bin/sh\n\
        ####### seven\n\
        #tag\n```\n# hidden\n```\n~~~\n# also hidden\n~~~\n";
    assert_eq!(
        super::anchors(text),
        ["my_func", "setup", "setup-1", "title"]
    );
}

#[test]
fn tilde_fences_hide_long_lines() {
    let text = format!("~~~\n{}\n~~~\n", "x".repeat(90));
    assert_eq!(found(&text).len(), 0);
}

#[test]
fn a_link_title_is_not_part_of_the_target() {
    assert_eq!(found(r#"[x](README.md "Read me")"#).len(), 0);
}
