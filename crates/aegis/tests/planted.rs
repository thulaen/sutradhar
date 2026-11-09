//! Planted faults for the Rust and shell checks: each plants one fault in
//! a tiny real project and sees exactly its own check refuse it.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use std::fs;
use std::path::{Path, PathBuf};

use aegis::repo::Repo;
use aegis::{shell, tiers};

const LOCK: &str =
    "version = 4\n\n[[package]]\nname = \"demo\"\nversion = \"0.1.0\"\n";

fn project(name: &str, lib: &str, test: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join(format!("aegis-planted-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    let files = [
        ("Cargo.toml", "[workspace]\nmembers = [\"demo\"]\n"),
        ("Cargo.lock", LOCK),
        ("deny.toml", "[licenses]\nallow = [\"MIT\"]\n"),
        (
            "demo/Cargo.toml",
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n\
             edition = \"2024\"\nlicense = \"MIT\"\n",
        ),
        ("demo/src/lib.rs", lib),
        ("demo/tests/t.rs", test),
    ];
    for (path, text) in files {
        let path = dir.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }
    dir
}

const GOOD_LIB: &str =
    "/// Adds.\npub fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n";
const GOOD_TEST: &str =
    "#[test]\nfn adds() {\n    assert_eq!(demo::add(1, 2), 3);\n}\n";

fn checks(dir: &Path) -> Vec<&'static str> {
    let found = tiers::rust_checks_using(&Repo::at(dir), &["cargo"]);
    found.iter().map(|f| f.check).collect()
}

#[test]
fn a_clean_project_passes_every_rust_check() {
    let dir = project("clean", GOOD_LIB, GOOD_TEST);
    assert_eq!(checks(&dir), Vec::<&str>::new());
}

#[test]
fn badly_laid_out_code_fails_only_the_format_check() {
    let lib = "/// Adds.\npub fn add( a: i32, b: i32 ) -> i32 { a+b }\n";
    let dir = project("format", lib, GOOD_TEST);
    assert_eq!(checks(&dir), ["rust.format"]);
}

#[test]
fn a_lint_breach_fails_only_the_lint_check() {
    let lib = "/// Adds.\npub fn add(a: i32, b: i32) -> i32 {\n    \
               let unused = 1;\n    a + b\n}\n";
    let dir = project("lints", lib, GOOD_TEST);
    assert_eq!(checks(&dir), ["rust.lints"]);
}

#[test]
fn a_failing_test_fails_only_the_test_check() {
    let test = "#[test]\nfn adds() {\n    assert_eq!(demo::add(1, 2), 4);\n}\n";
    let dir = project("tests", GOOD_LIB, test);
    assert_eq!(checks(&dir), ["rust.tests"]);
}

#[test]
fn a_stale_lock_file_fails_the_fetch() {
    let dir = project("fetch", GOOD_LIB, GOOD_TEST);
    assert!(tiers::fetch(&dir).is_none());
    fs::write(dir.join("Cargo.lock"), "version = 4\n").unwrap();
    let found = tiers::fetch(&dir);
    assert_eq!(found.map(|f| f.check), Some("rust.fetch"));
}

fn script(name: &str, text: &str) -> Vec<&'static str> {
    let dir = std::env::temp_dir()
        .join(format!("aegis-planted-sh-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("a.sh"), text).unwrap();
    let found = shell::check(&Repo::at(&dir), &["a.sh".to_owned()]);
    found.iter().map(|f| f.check).collect()
}

#[test]
fn a_badly_laid_out_script_fails_only_the_shell_format_check() {
    let text = "#!/bin/sh\nif true; then\necho x\nfi\n";
    assert_eq!(script("format", text), ["shell.format"]);
}

#[test]
fn a_shell_lint_breach_fails_only_the_shell_lint_check() {
    let text = "#!/bin/sh\necho $1\n";
    assert_eq!(script("lint", text), ["shell.lint"]);
    assert_eq!(
        script("clean", "#!/bin/sh\necho \"$1\"\n"),
        Vec::<&str>::new()
    );
}

#[test]
fn a_library_under_a_licence_not_allowed_fails_only_the_deny_check() {
    let dir = project("deny", GOOD_LIB, GOOD_TEST);
    let manifest = dir.join("demo/Cargo.toml");
    let text = fs::read_to_string(&manifest).unwrap();
    fs::write(&manifest, text.replace("\"MIT\"", "\"SSPL-1.0\"")).unwrap();
    assert_eq!(checks(&dir), ["rust.deny"]);
}

#[test]
fn an_unused_library_fails_only_the_unused_check() {
    let dir = project("unused", GOOD_LIB, GOOD_TEST);
    fs::create_dir_all(dir.join("helper/src")).unwrap();
    fs::write(
        dir.join("helper/Cargo.toml"),
        "[package]\nname = \"helper\"\nversion = \"0.1.0\"\n\
         edition = \"2024\"\nlicense = \"MIT\"\n",
    )
    .unwrap();
    fs::write(dir.join("helper/src/lib.rs"), "//! Helps.\n").unwrap();
    let manifest = dir.join("demo/Cargo.toml");
    let text = fs::read_to_string(&manifest).unwrap();
    let with = format!(
        "{text}\n[dependencies]\nhelper = {{ path = \"../helper\" }}\n"
    );
    fs::write(&manifest, with).unwrap();
    fs::write(
        dir.join("Cargo.toml"),
        "[workspace]\nmembers = [\"demo\", \"helper\"]\n",
    )
    .unwrap();
    let lock = format!(
        "{LOCK}dependencies = [\"helper\"]\n\n[[package]]\n\
         name = \"helper\"\nversion = \"0.1.0\"\n"
    );
    fs::write(dir.join("Cargo.lock"), lock).unwrap();
    assert_eq!(checks(&dir), ["rust.unused-deps"]);
}

#[test]
fn a_misspelt_word_fails_the_spelling_check() {
    let dir = std::env::temp_dir()
        .join(format!("aegis-planted-typos-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("a.md"), "The colour is fine.\n").unwrap();
    let all = ["a.md".to_owned()];
    let run = |d: &Path| {
        let found = tiers::whole_project_docs(&Repo::at(d), &all);
        found.into_iter().find(|f| f.check == "text.typos")
    };
    assert!(run(&dir).is_none());
    // Built from parts, so this file passes its own spelling check.
    let misspelt = format!("{}{} colour is fine.\n", "T", "eh");
    fs::write(dir.join("a.md"), misspelt).unwrap();
    assert_eq!(run(&dir).map(|f| f.check), Some("text.typos"));
}
