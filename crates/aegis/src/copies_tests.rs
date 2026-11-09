#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use super::{Copied, MOST_TOKENS, check, find};
use crate::file_kinds::Kind;

fn find_ok(files: &[(String, Kind, String)]) -> Vec<Copied> {
    find(files).unwrap()
}

/// A Rust function of about 15 lines, with its names made from `n`.
fn function(n: &str) -> String {
    format!(
        "fn sum_{n}(items: &[u32], limit_{n}: u32) -> u32 {{\n\
         \x20   let mut total_{n} = 0;\n\
         \x20   for item in items {{\n\
         \x20       if *item > limit_{n} {{\n\
         \x20           total_{n} += item * 2;\n\
         \x20       }} else if *item == limit_{n} {{\n\
         \x20           total_{n} += item;\n\
         \x20       }} else {{\n\
         \x20           total_{n} -= 1;\n\
         \x20       }}\n\
         \x20   }}\n\
         \x20   let spare_{n} = total_{n} % 7;\n\
         \x20   let more_{n} = spare_{n} * 3 + total_{n} / 2;\n\
         \x20   let extra_{n} = more_{n} - spare_{n} * 4 + limit_{n};\n\
         \x20   match extra_{n} {{ 0 => 1, x => x + spare_{n} }}\n\
         }}\n"
    )
}

fn rust(path: &str, text: String) -> (String, Kind, String) {
    (path.to_owned(), Kind::Rust, text)
}

#[test]
fn a_copy_with_new_names_is_found() {
    let one = format!("{}{}", function("a"), function("a"));
    let found = find_ok(&[rust("a.rs", one)]);
    assert_eq!(found.len(), 1, "{found:?}");
    let two = find_ok(&[
        rust("b.rs", format!("// a note\n{}", function("b"))),
        rust("a.rs", function("a")),
    ]);
    assert_eq!(
        two,
        [Copied {
            first: ("a.rs".to_owned(), 1, 16),
            second: ("b.rs".to_owned(), 2, 17),
            tokens: two.first().unwrap().tokens,
        }]
    );
    let messages =
        check(&[rust("a.rs", function("a")), rust("b.rs", function("b"))]);
    assert!(messages.first().unwrap().message.contains("copy a.rs"));
}

#[test]
fn different_code_and_short_copies_pass() {
    let short = "fn a() -> u32 { let x = 1; x + 2 }\n";
    let files = [
        rust("a.rs", format!("{short}{short}{short}")),
        rust("b.rs", function("b").replace("* 2", "* 2 + 9 - 4 * 1")),
        rust(
            "c.rs",
            function("c").replace("for item in", "while let Some(item) ="),
        ),
    ];
    assert_eq!(find_ok(&files[..1]), Vec::new());
    let found = find_ok(&files[1..]);
    assert!(found.iter().all(|c| c.tokens < 100), "{found:?}");
}

#[test]
fn copies_in_other_languages_are_found_with_comments_and_text_dropped() {
    let script = script();
    let files = [
        ("a.sh".to_owned(), Kind::Shell, script.clone()),
        (
            "b.sh".to_owned(),
            Kind::Shell,
            script.replace("step", "other"),
        ),
        ("c.md".to_owned(), Kind::Markdown, script),
    ];
    let found = find_ok(&files);
    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!(found.first().unwrap().second.0, "b.sh");
}

#[test]
fn a_table_of_like_entries_is_not_a_copy() {
    let entry = "    (\"a.rego\", include_str!(\"../rules/a.rego\")),\n";
    let table = format!("pub const ALL: [S; 40] = [\n{}];\n", entry.repeat(40));
    assert_eq!(find_ok(&[rust("a.rs", table)]), Vec::new());
}

/// A shell script of 40 unlike lines.
fn script() -> String {
    let mut script = String::new();
    for i in 0..40 {
        let line = format!("cp{i} -r \"$src\" /tmp/x{i} && echo # step {i}\n");
        script.push_str(&line);
    }
    script
}

#[test]
fn a_file_pattern_in_a_script_hides_nothing() {
    let glob = format!("rm -f build/*.o\n{}", script());
    let files = [
        ("a.sh".to_owned(), Kind::Shell, glob),
        ("b.sh".to_owned(), Kind::Shell, script()),
    ];
    assert_eq!(find_ok(&files).len(), 1);
}

fn shell(path: &str, text: String) -> (String, Kind, String) {
    (path.to_owned(), Kind::Shell, text)
}

/// The one copy expected: (path, first line, last line) of each place.
fn one_copy(
    files: &[(String, Kind, String)],
    first: (&str, usize, usize),
    second: (&str, usize, usize),
) {
    let found = find_ok(files);
    let places: Vec<_> = found.iter().map(|c| (&c.first, &c.second)).collect();
    let own = |p: (&str, usize, usize)| (p.0.to_owned(), p.1, p.2);
    assert_eq!(places, [(&own(first), &own(second))], "{found:?}");
}

const TABLE: &str = "set_opt a 1\n";
const OTHER_TABLE: &str = "opt_two b 2 c\n";

#[test]
fn a_table_in_front_cannot_hide_a_copy() {
    let t = |n: usize| TABLE.repeat(n);
    // Tables of different lengths in front of the same script.
    let files = [
        shell("a.sh", format!("{}{}", t(40), script())),
        shell("b.sh", format!("{}{}", t(80), script())),
    ];
    one_copy(&files, ("a.sh", 1, 80), ("b.sh", 1, 120));
    // A table file sorted first, which would fill the places kept for
    // a run, and a short table that is not squeezed.
    let files = [
        shell("0.sh", t(700)),
        shell("b.sh", format!("{}{}", t(3), script())),
        shell("c.sh", format!("{}{}", t(60), script())),
    ];
    one_copy(&files, ("b.sh", 3, 43), ("c.sh", 1, 100));
    // Two tables in a row.
    let two = |n: usize, m: usize| {
        format!("{}{}{}", t(n), OTHER_TABLE.repeat(m), script())
    };
    let files = [shell("a.sh", two(50, 50)), shell("b.sh", two(80, 70))];
    one_copy(&files, ("a.sh", 1, 140), ("b.sh", 1, 190));
    // The same in Rust, with `use` lines, which are left out, in front.
    let files = [
        rust("a.rs", format!("{}{}", uses(25), function("a"))),
        rust("b.rs", format!("{}{}", uses(30), function("b"))),
    ];
    one_copy(&files, ("a.rs", 26, 41), ("b.rs", 31, 46));
}

/// `n` lines of `use std::mN;`.
fn uses(n: usize) -> String {
    let mut text = String::new();
    for i in 0..n {
        let line = format!("use std::m{i};\n");
        text.push_str(&line);
    }
    text
}

#[test]
fn tables_of_different_lengths_alone_are_not_a_copy() {
    let files = [
        rust("a.rs", format!("{}fn main() {{}}\n", uses(30))),
        rust("b.rs", format!("{}fn main() {{}}\n", uses(40))),
        rust("c.rs", format!("{}fn main() {{}}\n", array(40))),
        rust("d.rs", format!("{}fn main() {{}}\n", array(50))),
        shell("a.sh", format!("{}echo done; exit 0\n", TABLE.repeat(40))),
        shell("b.sh", format!("{}echo done; exit 0\n", TABLE.repeat(50))),
    ];
    assert_eq!(find_ok(&files), Vec::new());
}

#[test]
fn too_much_source_is_refused_not_skipped() {
    let huge = "a b c d e f g h i j\n".repeat(MOST_TOKENS / 10 + 1);
    let files = [("a.sh".to_owned(), Kind::Shell, huge)];
    assert!(find(&files).is_err());
    assert!(check(&files)[0].message.contains("refused"));
}

/// A chain of `n` calls in Rust, with names made from `v`.
fn chain(v: &str, n: usize) -> String {
    let mut text = format!("fn run_{v}({v}0: Body) -> Result<Body, E> {{\n");
    for i in 1..=n {
        let line = format!(
            "    let {v}{i} = parse_{v}({v}{})?;\n",
            i.saturating_sub(1)
        );
        text.push_str(&line);
    }
    let end = format!("    Ok({v}{n})\n}}\n");
    text.push_str(&end);
    text
}

/// Guard checks in Rust, one for each field, with names from `v`.
fn guards(v: &str) -> String {
    let mut text = format!("fn check_{v}(r: &R, l: &L) -> Result<(), E> {{\n");
    for f in ["a", "b", "c", "d", "e", "f", "g"] {
        let up = f.to_uppercase();
        let line = format!(
            "    if r.{v}{f} > l.{v}{f} {{ return Err(E::{v}{up}); }}\n"
        );
        text.push_str(&line);
    }
    text.push_str("    Ok(())\n}\n");
    text
}

#[test]
fn lines_of_one_shape_with_their_own_names_are_code_not_a_table() {
    let files = [rust("a.rs", chain("d", 13)), rust("b.rs", chain("x", 13))];
    assert_eq!(find_ok(&files).len(), 1, "{:?}", find_ok(&files));
    let files = [rust("a.rs", guards("p")), rust("b.rs", guards("q"))];
    assert_eq!(find_ok(&files).len(), 1, "{:?}", find_ok(&files));
}

/// A Rust table: an array of `n` like lines.
fn array(n: usize) -> String {
    format!(
        "const T: [u8; {}] = [\n{}];\n",
        n.saturating_mul(3),
        "    1, 2, 3,\n".repeat(n)
    )
}

#[test]
fn the_same_line_many_times_inside_a_copy_cannot_hide_it() {
    let body = |n: &str| {
        let shadow = "    let d = parse_body(d)?;\n".repeat(13);
        function(n).replacen("{\n", &format!("{{\n{shadow}"), 1)
    };
    let files = [rust("a.rs", body("a")), rust("b.rs", body("b"))];
    assert_eq!(find_ok(&files).len(), 1, "{:?}", find_ok(&files));
}

#[test]
fn only_a_use_item_is_left_out() {
    let fronts = [
        "fn g<'a>(x: &'a u8) -> impl Sized + use<'a> { x }\n",
        "fn r#use() {}\n",
        "macro_rules! m { (use $x:ident) => {}; }\n",
        "pub(crate) use std::fmt;\n#[cfg(test)]\nuse std::io;\n",
    ];
    for front in fronts {
        let files = [
            rust("a.rs", format!("{front}{}", function("a"))),
            rust("b.rs", format!("{front}{}", function("b"))),
        ];
        assert_eq!(find_ok(&files).len(), 1, "{front}: {:?}", find_ok(&files));
    }
}
