use super::read;
use crate::facts::Facts;
use serde_json::{Value, json};

fn facts(path: &str, source: &str) -> Facts {
    let mut f = Facts::default();
    read(path, source, &mut f);
    f
}

fn has(f: &Facts, relation: &str, want: &Value) -> bool {
    f.of(relation).iter().any(|v| {
        want.as_object()
            .is_some_and(|w| w.iter().all(|(k, x)| v.get(k) == Some(x)))
    })
}

#[test]
fn functions_carry_their_kind_and_lines() {
    let f = facts(
        "src/a.rs",
        "fn main() {\n}\n#[cfg(test)]\nmod tests {\n    fn helper() {}\n    \
         #[test]\n    fn t() {}\n    #[test]\n    #[should_panic]\n    \
         fn p() {}\n}\n",
    );
    for (name, kind) in [
        ("main", "code"),
        ("helper", "test_support"),
        ("t", "test"),
        ("p", "test_panics"),
    ] {
        assert!(has(&f, "function", &json!({"name": name, "kind": kind})));
    }
    assert!(has(
        &f,
        "function",
        &json!({"name": "main", "first": 1, "last": 2})
    ));
    let t = facts("crates/a/tests/x.rs", "fn h() {}\n");
    assert!(has(
        &t,
        "function",
        &json!({"name": "h", "kind": "test_support"})
    ));
}

#[test]
fn calls_are_named_through_imports() {
    let f = facts(
        "a.rs",
        "use std::net::TcpStream;\nuse std::sync::mpsc as m;\nfn f() {\n    \
         let _s = TcpStream::connect(\"x\");\n    m::channel::<u8>();\n    \
         x.unwrap();\n}\n",
    );
    for (callee, line) in [
        ("std::net::TcpStream::connect", 4),
        ("std::sync::mpsc::channel", 5),
        (".unwrap", 6),
    ] {
        let want = json!({"function": "f", "callee": callee, "line": line});
        assert!(has(&f, "call", &want), "{callee}");
    }
}

#[test]
fn thrown_away_results_and_empty_handlers_are_recorded() {
    let f = facts(
        "a.rs",
        "fn f() {\n    let _ = g();\n    h().ok();\n    match r {\n        \
         Ok(v) => v,\n        Err(_) => {}\n    }\n    let _ = 5;\n    \
         let _x = g();\n}\n",
    );
    assert!(has(&f, "discard", &json!({"line": 2, "kind": "let _ ="})));
    assert!(has(&f, "discard", &json!({"line": 3, "kind": ".ok();"})));
    assert_eq!(f.of("discard").len(), 2);
    assert!(has(
        &f,
        "empty_handler",
        &json!({"function": "f", "line": 6})
    ));
}

#[test]
fn macros_and_question_marks_are_recorded() {
    let f = facts(
        "a.rs",
        "fn f() -> R {\n    assert_eq!(1, 1);\n    g()?;\n}\n",
    );
    assert!(has(&f, "macro", &json!({"name": "assert_eq", "line": 2})));
    assert!(has(&f, "uses_try", &json!({"function": "f", "line": 3})));
}

#[test]
fn a_file_that_cannot_be_read_is_a_fact_not_silence() {
    let f = facts("a.rs", "fn f( {\n");
    assert_eq!(f.of("parse_failure").len(), 1);
    assert!(has(
        &f,
        "file",
        &json!({"path": "a.rs", "language": "rust"})
    ));
}
