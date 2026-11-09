//! Planted and clean cases for the h-series rules.

use super::tests_common::lines;

const NONE: [usize; 0] = [];

#[test]
fn h06_finds_a_file_anyone_may_write() {
    let code = "fn f() {\n    p.set_mode(0o666);\n    p.set_mode(0o600);\n    \
                let _m = Permissions::from_mode(0o777);\n}\n";
    assert_eq!(lines("H06", &[("src/a.rs", code)]), [2, 4]);
}

#[test]
fn h11_finds_a_file_written_without_a_buffer() {
    let bare = "fn f() {\n    let mut o = std::fs::File::create(p)?;\n    \
                writeln!(o, \"a\")?;\n    writeln!(o, \"b\")?;\n}\n";
    assert_eq!(lines("H11", &[("src/a.rs", bare)]), [2]);
    let buffered = "fn f() {\n    let o = std::fs::File::create(p)?;\n    \
                    let mut o = std::io::BufWriter::new(o);\n    \
                    writeln!(o, \"a\")?;\n    writeln!(o, \"b\")?;\n}\n";
    assert_eq!(lines("H11", &[("src/a.rs", buffered)]), NONE);
}

#[test]
fn h22_finds_destructive_sql_in_code() {
    let code = "fn f() {\n    q(\"drop table posts\");\n    \
                q(\"DELETE FROM t\");\n    \
                q(\"DELETE FROM t WHERE id = $1\");\n}\n";
    assert_eq!(lines("H22", &[("src/a.rs", code)]), [2, 3]);
    assert_eq!(lines("H22", &[("tests/a.rs", code)]), NONE);
}

#[test]
fn h23_finds_a_read_then_a_separate_set() {
    let code = "fn f() {\n    let v = self.n.load(O);\n    \
                self.n.store(v + 1, O);\n    self.m.store(1, O);\n}\n";
    assert_eq!(lines("H23", &[("src/a.rs", code)]), [3]);
}

#[test]
fn h25_finds_a_cache_never_cleared() {
    let filled = "fn f(&mut self) {\n    self.cache.insert(k, v);\n}\n";
    assert_eq!(lines("H25", &[("src/a.rs", filled)]), [2]);
    let cleared = "fn f(&mut self) {\n    self.cache.insert(k, v);\n}\n\
                   fn g(&mut self) {\n    self.cache.clear();\n}\n";
    assert_eq!(lines("H25", &[("src/a.rs", cleared)]), NONE);
}

#[test]
fn h35_finds_a_panic_inside_drop() {
    let code = "impl Drop for A {\n    fn drop(&mut self) {\n        \
                self.f.flush().unwrap();\n    }\n}\n\
                impl B {\n    fn drop(&mut self) {\n        \
                x.unwrap();\n    }\n}\n";
    assert_eq!(lines("H35", &[("src/a.rs", code)]), [3]);
}

#[test]
fn h39_finds_writes_outside_a_transaction() {
    let bare = "fn save() {\n    q(\"INSERT INTO a VALUES (1)\");\n    \
                q(\"UPDATE b SET c = 1\");\n}\n";
    assert_eq!(lines("H39", &[("src/a.rs", bare)]), [1]);
    let held = "fn run() {\n    let t = db.begin();\n    save();\n}\n";
    assert_eq!(
        lines("H39", &[("src/a.rs", bare), ("src/b.rs", held)]),
        NONE
    );
}

#[test]
fn h42_finds_a_property_test_with_too_few_cases() {
    let code = "fn f() {\n    let _c = Config::with_cases(5);\n    \
                let _d = Config::with_cases(500);\n}\n";
    assert_eq!(lines("H42", &[("tests/a.rs", code)]), [2]);
}

#[test]
fn h44_h53_and_h56_find_local_time_leaks_and_one_machine_paths() {
    let code = "use chrono::Local;\nfn f() {\n    let _t = Local::now();\n    \
                let _b = Box::leak(b);\n    let _p = \"C:\\\\data\";\n    \
                let _u = \"/Users/a\";\n    let _ok = \"a:b\";\n}\n";
    let files = [("src/a.rs", code)];
    assert_eq!(lines("H44", &files), [3]);
    assert_eq!(lines("H53", &files), [4]);
    assert_eq!(lines("H56", &files), [5, 6]);
}

#[test]
fn t01_finds_text_cut_short_and_lets_whole_refusals_pass() {
    let cut = "fn first(text: &str) -> String {\n    \
               text.chars().take(400).collect()\n}\n\
               fn embed(mut tokens: Vec<u32>) {\n    tokens.truncate(512);\n}\n\
               fn head(post: &Post) -> &str {\n    &post.body[..200]\n}\n\
               fn lead(page: &str) -> Option<&str> {\n    page.get(..80)\n}\n";
    assert_eq!(
        lines("T01", &[("crates/shodhan/src/a.rs", cut)]),
        [2, 5, 8, 11]
    );
    let fine = "fn check(text: &str) -> Result<(), E> {\n    \
                if text.len() > MAX {\n        \
                return Err(E::TooLong);\n    }\n    \
                Ok(())\n}\n\
                fn top(mut found: Vec<F>) {\n    found.truncate(20);\n}\n\
                fn rest(text: &str) -> &str {\n    &text[4..]\n}\n";
    assert_eq!(lines("T01", &[("crates/shodhan/src/a.rs", fine)]), NONE);
    assert_eq!(lines("T01", &[("crates/shodhan/tests/a.rs", cut)]), NONE);
}
