//! Planted and clean cases for the code-smell rules (task #118).

use super::tests_common::lines;

fn only(id: &str, code: &str) -> Vec<usize> {
    lines(id, &[("src/a.rs", code)])
}

fn none() -> Vec<usize> {
    Vec::new()
}

#[test]
fn m01_finds_a_true_or_false_input() {
    assert_eq!(only("M01", "fn f(a: u8, b: bool) {}\n"), [1]);
    assert_eq!(only("M01", "fn f(a: u8, b: Mode) {}\n"), none());
    assert_eq!(
        lines("M01", &[("tests/a.rs", "fn f(b: bool) {}\n")]),
        none()
    );
}

#[test]
fn m02_finds_a_number_with_no_name() {
    let code = "const MAX: usize = 300;\nfn f(v: &[u8; 32]) -> usize {\n    \
                v.len() * 2 + 86_400\n}\n";
    assert_eq!(only("M02", code), [3]);
    let named = "fn f() -> u64 {\n    const DAY: u64 = 86_400;\n    DAY\n}\n";
    assert_eq!(only("M02", named), none());
}

#[test]
fn m03_finds_a_very_long_chain() {
    let long = "fn f(v: V) {\n    v.a().b().c().d().e().f().g().h().i();\n}\n";
    assert_eq!(only("M03", long), [2]);
    let short = "fn f(v: V) {\n    v.a().b().c();\n}\n";
    assert_eq!(only("M03", short), none());
    // Through `?`, and inside a call's input.
    let tried =
        "fn f(v: V) -> R {\n    v.a()?.b()?.c().d().e().f().g().h().i();\n}\n";
    assert_eq!(only("M03", tried), [2]);
    let inside =
        "fn f(v: V) {\n    g(v.a().b().c().d().e().f().g().h().i());\n}\n";
    assert_eq!(only("M03", inside), [2]);
}

#[test]
fn m04_finds_a_very_large_type() {
    let mut fields = String::new();
    for i in 0..13 {
        let one = format!("f{i}: u8, ");
        fields.push_str(&one);
    }
    let big = format!("struct S {{ {fields}}}\n");
    assert_eq!(only("M04", &big), [1]);
    assert_eq!(only("M04", "struct S { a: u8 }\nenum E { A, B }\n"), none());
}

#[test]
fn m05_finds_code_left_in_a_comment() {
    let code =
        "fn f() {\n    // let x = g(1);\n    // and so on, and so forth;\n}\n";
    assert_eq!(only("M05", code), [2]);
    // Plain sentences, doc comments and comments in tests pass.
    let clean = "fn f() {\n    \
                 // Read the file (if there is one), then parse it,\n    \
                 /// let y = h(2);\n}\n#[cfg(test)]\nmod tests {\n    \
                 // let z = k(3);\n}\n";
    assert_eq!(only("M05", clean), none());
    assert_eq!(only("M05", "//// let x = g(1);\n"), [1]);
}

#[test]
fn m06_finds_text_written_three_times() {
    let code = "fn f() {\n    g(\"refs/heads/main\");\n    \
                g(\"refs/heads/main\");\n    g(\"refs/heads/main\");\n}\n";
    assert_eq!(only("M06", code), [2, 3, 4]);
    let twice = "fn f() {\n    g(\"refs/heads/main\");\n    \
                 g(\"refs/heads/main\");\n}\n";
    assert_eq!(only("M06", twice), none());
    let one_line = "fn f() {\n    g(\"refs/heads/main\", \"refs/heads/main\", \
                    \"refs/heads/main\");\n}\n";
    assert_eq!(only("M06", one_line), [2]);
}

#[test]
fn m07_finds_a_function_that_does_two_jobs() {
    assert_eq!(only("M07", "fn read_and_send() {}\n"), [1]);
    assert_eq!(only("M07", "fn read() {}\nfn send() {}\n"), none());
}

#[test]
fn trait_methods_with_a_body_are_read() {
    let code = "trait T {\n    fn f(&self, on: bool) -> u32 {\n        \
                86_400\n    }\n}\n";
    assert_eq!(only("M01", code), [2]);
    assert_eq!(only("M02", code), [3]);
}

#[test]
fn trait_methods_with_no_body_are_read() {
    let code = "trait T {\n    fn f(&self, on: bool);\n    \
                fn read_and_send(&self);\n}\n";
    assert_eq!(only("M01", code), [2]);
    assert_eq!(only("M07", code), [3]);
}

#[test]
fn m08_finds_a_function_no_test_reaches() {
    let code = "fn used() {}\nfn by_name() {}\nfn in_macro() -> u8 { 1 }\n\
                fn alone() {}\nfn helper() { used(); }\n\
                fn main() { started(); }\nfn started() {}\n\
                impl Drop for S {\n    fn drop(&mut self) { cleanup(); }\n}\n\
                fn cleanup() {}\n";
    let tests = "#[test]\nfn t() {\n    helper();\n    \
                 let _f = [1].iter().map(|_| by_name);\n    \
                 assert_eq!(in_macro(), 1);\n}\n";
    let files = [("src/a.rs", code), ("tests/a.rs", tests)];
    assert_eq!(lines("M08", &files), [4]);
}

#[test]
fn m08_is_not_fooled_and_does_not_misreport() {
    // A local of the same name, a helper in test support, an impl of
    // the project's own trait, and code under `cfg(not(test))` do not
    // count as reached.
    let code = "fn compute() {}\nfn helped() {}\ntrait Own {\n    \
                fn go(&self);\n}\nimpl Own for S {\n    \
                fn go(&self) { hidden(); }\n}\nfn hidden() {}\n\
                #[cfg(not(test))]\nfn live() {}\n\
                fn shown(&self) {}\nfn inner() -> u8 { 1 }\n";
    let support = "pub fn setup() {\n    helped();\n}\n";
    let tests = "#[test]\nfn t() {\n    let compute = 1;\n    \
                 assert_eq!(compute, 1);\n    \
                 assert_eq!(S.shown(), vec![format!(\"{}\", inner())]);\n}\n";
    let files = [
        ("src/a.rs", code),
        ("tests/common/mod.rs", support),
        ("tests/a.rs", tests),
    ];
    assert_eq!(lines("M08", &files), [1, 2, 7, 9, 11]);
}

#[test]
fn m08_reads_cfg_conditions_and_an_empty_test_reaches_nothing() {
    let code = "#[cfg(not(test))]\nmod live {\n    fn shipped() {}\n}\n\
                #[cfg(all(test, not(miri)))]\nmod checks {\n    \
                fn helper() {}\n}\n\
                #[cfg(any(test, unix))]\nmod both {\n    fn either() {}\n}\n\
                fn risky() {}\nfn y() {}\nfn main() {}\n";
    let tests = "#[test]\nfn risky() {}\n#[test]\nfn u() {\n    y();\n}\n";
    let files = [("src/a.rs", code), ("tests/a.rs", tests)];
    assert_eq!(lines("M08", &files), [3, 11, 13]);
}
