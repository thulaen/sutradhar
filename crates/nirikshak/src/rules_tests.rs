use super::{ALL, about, run};

use super::tests_common::{facts, lines};

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

#[test]
fn r05_finds_a_setting_no_page_explains() {
    let code = "fn f() {\n    let _q = std::env::var(\"AEGIS_QUEUE\");\n    \
                let _h = std::env::var(\"HIDDEN\");\n    \
                let _c = env!(\"CARGO_PKG_NAME\");\n    \
                let _o = option_env!(\"SECRET_PLACE\");\n}\n";
    let docs = "Set `AEGIS_QUEUE` to the queue.\n";
    let files = [("src/a.rs", code), ("docs/a.md", docs)];
    assert_eq!(lines("R05", &files), [3, 5]);
    let tests = [("crates/a/tests/t.rs", code), ("docs/a.md", docs)];
    assert_eq!(lines("R05", &tests), Vec::<usize>::new());
}

#[test]
fn r18_finds_a_queue_with_no_limit() {
    let code = "use std::sync::mpsc;\nfn f() {\n    \
                let _c = mpsc::channel::<u8>();\n    \
                let _b = mpsc::sync_channel::<u8>(8);\n    \
                let _u = tokio::sync::mpsc::unbounded_channel::<u8>();\n}\n";
    assert_eq!(lines("R18", &[("src/a.rs", code)]), [3, 5]);
}

#[test]
fn r19_finds_sql_built_from_strings() {
    let code = "fn f() {\n    \
                let _q = format!(\"select * from t where a = {a}\");\n    \
                let _n = format!(\"selected {n}\");\n}\n";
    assert_eq!(lines("R19", &[("src/a.rs", code)]), [2]);
}

#[test]
fn s01_finds_a_command_run_through_a_shell() {
    let code = "use std::process::Command;\nfn f() {\n    \
                let _s = Command::new(\"sh\");\n    \
                let _g = Command::new(\"git\");\n}\n";
    assert_eq!(lines("S01", &[("src/a.rs", code)]), [3]);
}

#[test]
fn s02_and_s03_find_switched_off_checks_and_weak_hashes() {
    let code = "fn f() {\n    c.danger_accept_invalid_certs(true);\n    \
                let _h = md5::compute(b\"x\");\n    \
                let _s = sha2::Sha256::digest(b\"x\");\n}\n";
    assert_eq!(lines("S02", &[("tests/a.rs", code)]), [2]);
    assert_eq!(lines("S03", &[("tests/a.rs", code)]), [3]);
}

#[test]
fn r05_also_finds_a_yaml_setting_no_page_explains() {
    let compose = "services:\n  a:\n    environment:\n      SHOWN: x\n      \
                   UNSEEN: y\n";
    let docs = "`SHOWN` turns it on.\n";
    let files = [("docker/a/compose.yaml", compose), ("docs/a.md", docs)];
    assert_eq!(lines("R05", &files), [5]);
}
