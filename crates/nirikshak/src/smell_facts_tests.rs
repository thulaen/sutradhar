#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use crate::facts::Facts;

fn facts(path: &str, code: &str) -> Facts {
    let mut facts = Facts::default();
    crate::rust_facts::read(path, code, &mut facts);
    facts
}

fn field(facts: &Facts, relation: &str, key: &str) -> Vec<String> {
    facts
        .of(relation)
        .iter()
        .map(|f| f[key].to_string().trim_matches('"').to_owned())
        .collect()
}

#[test]
fn smells_are_read_with_their_kind() {
    let code = "const MAX: usize = 300;\nstruct S { a: u8, b: [u8; 32] }\n\
                enum E { A, B, C }\n\
                fn f(a: bool, b: u8, c: bool) -> usize {\n    \
                // let x = 1;\n    \
                /// not this\n    \
                let v = [1, 2].iter().map(|x| x * 7).sum::<usize>();\n    \
                v + 42\n}\n\
                #[cfg(test)]\nmod tests {\n    #[test]\n    \
                fn t() { let _ = 9; }\n}\n";
    let f = facts("src/a.rs", code);
    assert_eq!(field(&f, "signature", "bools"), ["2", "0"]);
    assert_eq!(field(&f, "signature", "inputs"), ["3", "0"]);
    assert_eq!(field(&f, "number", "text"), ["1", "2", "7", "42", "9"]);
    assert_eq!(
        field(&f, "number", "kind"),
        ["code", "code", "code", "code", "test"]
    );
    assert_eq!(field(&f, "chain", "length"), ["3"]);
    assert_eq!(field(&f, "shape", "size"), ["2", "3"]);
    assert_eq!(field(&f, "comment", "text"), ["let x = 1;"]);
    assert_eq!(
        field(&facts("tests/a.rs", code), "comment", "kind"),
        ["test"]
    );
}
