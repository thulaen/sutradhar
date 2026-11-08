use super::check;
use crate::file_kinds::Kind;

fn problems(bytes: &[u8]) -> usize {
    check("a.rs", Kind::Rust, bytes).len()
}

#[test]
fn clean_text_passes() {
    assert_eq!(problems(b"fn f() {}\n\tlet x = 1;\n"), 0);
    assert_eq!(problems(b""), 0);
}

#[test]
fn trailing_space_tab_and_carriage_returns_fail_with_their_line() {
    let found = check("a.rs", Kind::Rust, b"a \nb\t\nc\r\nd\n");
    let lines: Vec<usize> = found.iter().map(|f| f.line).collect();
    assert_eq!(lines, [1, 2, 3]);
}

#[test]
fn a_missing_last_line_break_fails() {
    assert_eq!(problems(b"fn f() {}"), 1);
}

#[test]
fn text_that_is_not_utf8_fails() {
    assert_eq!(problems(b"caf\xe9\n"), 1);
}

#[test]
fn hidden_and_control_characters_fail() {
    for bad in [
        "a\u{202e}b\n",
        "a\u{2066}b\n",
        "a\u{200b}b\n",
        "a\u{2028}b\n",
        "a\u{85}b\n",
        "a\u{feff}b\n",
        "a\u{0c}b\n",
        "a\u{1b}b\n",
    ] {
        assert_eq!(problems(bad.as_bytes()), 1, "{bad:?}");
    }
}

#[test]
fn ordinary_letters_from_any_alphabet_pass() {
    assert_eq!(problems("सूत्रधार café naïve\n".as_bytes()), 0);
}

#[test]
fn licence_texts_are_left_as_written() {
    assert_eq!(check("LICENSE", Kind::Licence, b"a \r\n").len(), 0);
}

#[test]
fn at_most_twenty_problems_are_listed_per_file() {
    let many = "x \n".repeat(50);
    let found = check("a.rs", Kind::Rust, many.as_bytes());
    assert_eq!(found.len(), 21);
    assert!(found.last().is_some_and(|f| f.message.contains("more")));
}

#[test]
fn every_kind_of_hidden_character_fails() {
    for bad in [
        "a\u{ad}b\n",
        "a\u{200e}b\n",
        "a\u{61c}b\n",
        "a\u{2029}b\n",
        "a\u{2069}b\n",
        "a\u{e0041}b\n",
        "a\u{2062}b\n",
        "a\u{180e}b\n",
        "a\u{3164}b\n",
        "a\u{fffa}b\n",
        "a\u{34f}b\n",
        "a\u{17b4}b\n",
        "a\u{180b}b\n",
        "a\u{fe0f}b\n",
        "a\u{e0100}b\n",
    ] {
        assert_eq!(problems(bad.as_bytes()), 1, "{bad:?}");
    }
}

#[test]
fn spaces_of_every_kind_at_the_end_of_a_line_fail() {
    for bad in ["a\u{a0}\n", "a\u{3000}\n", "a\u{2003}\n", "a\u{1f}\n"] {
        assert_eq!(problems(bad.as_bytes()), 1, "{bad:?}");
    }
}

#[test]
fn joiners_are_allowed_only_inside_indian_script_words() {
    assert_eq!(problems("क\u{94d}\u{200d}ष\n".as_bytes()), 0);
    assert_eq!(problems("र\u{200c}क\n".as_bytes()), 0);
    assert_eq!(problems("a\u{200d}b\n".as_bytes()), 1);
    assert_eq!(problems("\u{200c}क\n".as_bytes()), 1);
}
