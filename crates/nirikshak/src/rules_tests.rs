use super::{ALL, Source, about, run};
use crate::facts::Facts;
use crate::rust_facts;

fn facts(files: &[(&str, &str)]) -> Facts {
    let mut f = Facts::default();
    for (path, text) in files {
        rust_facts::read(path, text, &mut f);
    }
    f
}

fn rule(id: &str) -> Source {
    let found = ALL
        .iter()
        .find(|(_, text)| about("x", text).is_ok_and(|a| a.id == id));
    match found {
        Some(source) => *source,
        None => ("missing.rego", ""),
    }
}

/// The lines a rule finds in the files.
fn lines(id: &str, files: &[(&str, &str)]) -> Vec<usize> {
    let found = run(rule(id), &facts(files));
    found.map_or_else(
        |e| vec![usize::MAX; usize::from(!e.is_empty())],
        |f| f.iter().map(|x| x.line).collect(),
    )
}

#[test]
fn every_rule_describes_itself() {
    for (name, text) in ALL {
        let a = about(name, text);
        assert!(a.is_ok(), "{name}: {a:?}");
    }
    assert!(about("x", "# id: R1\npackage p\n").is_err());
}

#[test]
fn r08_finds_a_network_call_with_no_time_limit() {
    let code = "use std::net::TcpStream;\nfn f() {\n    \
                let _s = TcpStream::connect(\"a:1\");\n    \
                let _t = TcpStream::connect_timeout(&a, d);\n}\n";
    assert_eq!(lines("R08", &[("src/a.rs", code)]), [3]);
    assert_eq!(lines("R08", &[("tests/a.rs", code)]), Vec::<usize>::new());
}

#[test]
fn r09_finds_thrown_away_errors_in_code_only() {
    let code = "fn f() {\n    let _ = g();\n    h().ok();\n    \
                match r { Ok(_) => {}, Err(_) => {} }\n    \
                let _kept = g();\n}\n";
    assert_eq!(lines("R09", &[("src/a.rs", code)]), [2, 3, 4]);
    assert_eq!(
        lines("R09", &[("src/a_tests.rs", code)]),
        Vec::<usize>::new()
    );
}

#[test]
fn r14_finds_a_test_that_checks_nothing() {
    let tests = "#[test]\nfn empty() {\n    let _x = 1;\n}\n\
                 #[test]\nfn direct() {\n    assert!(true);\n}\n\
                 #[test]\nfn through() {\n    helper();\n}\n\
                 #[test]\nfn tried() -> R {\n    g()?;\n    Ok(())\n}\n";
    let helper = "pub fn helper() {\n    deeper();\n}\n\
                  fn deeper() {\n    assert_eq!(1, 1);\n}\n";
    let files = [("tests/a.rs", tests), ("tests/common/mod.rs", helper)];
    assert_eq!(lines("R14", &files), [2]);
}

#[test]
fn a_rule_that_cannot_run_blocks() {
    let f = facts(&[("a.rs", "fn f() {}\n")]);
    let broken = ("b.rego", "# id: B\n# title: t\n# why: w\n# fix: f\npackage");
    assert!(run(broken, &f).is_err());
    let shape = (
        "s.rego",
        "# id: S\n# title: t\n# why: w\n# fix: f\npackage nirikshak.s\n\
         import rego.v1\nfindings contains {\"path\": \"a\"} if { true }\n",
    );
    assert!(run(shape, &f).is_err());
}
