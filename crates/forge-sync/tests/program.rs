//! The `forge-sync` program itself.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use std::process::Command;

fn sync(args: &[&str], envs: &[(&str, &str)]) -> Option<i32> {
    let mut command = Command::new(env!("CARGO_BIN_EXE_forge-sync"));
    let _ = command
        .args(args)
        .env_remove("FORGE_SYNC_FORGEJO")
        .env_remove("FORGE_SYNC_HEADER")
        .env_remove("FORGE_SYNC_GITHUB")
        .env_remove("FORGE_SYNC_MIRROR")
        .env_remove("FORGE_SYNC_RECORD")
        .env_remove("FORGE_SYNC_GITHUB_USER")
        .env_remove("FORGE_SYNC_GITLEAKS")
        .env_remove("FORGE_SYNC_PRIVATE");
    for (k, v) in envs {
        let _ = command.env(k, v);
    }
    command.output().unwrap().status.code()
}

#[test]
fn bad_use_and_missing_settings_stop_with_code_2() {
    assert_eq!(sync(&[], &[]), Some(2));
    assert_eq!(sync(&["once"], &[]), Some(2));
    let some = [("FORGE_SYNC_PRIVATE", "/no/such/file")];
    assert_eq!(sync(&["once"], &some), Some(2));
}

#[test]
fn a_run_with_no_tools_fails_and_says_why() {
    let words = std::env::temp_dir()
        .join(format!("forge-sync-words-{}", std::process::id()));
    std::fs::write(&words, "secret\n").unwrap();
    let envs = [
        ("FORGE_SYNC_FORGEJO", "http://127.0.0.1:1/x"),
        ("FORGE_SYNC_HEADER", "/no/header"),
        ("FORGE_SYNC_GITHUB", "o/r"),
        ("FORGE_SYNC_MIRROR", "/no/mirror"),
        ("FORGE_SYNC_RECORD", "/no/record"),
        ("FORGE_SYNC_GITHUB_USER", "owner"),
        ("FORGE_SYNC_GITLEAKS", "gitleaks"),
        ("FORGE_SYNC_PRIVATE", &words.to_string_lossy()),
        ("PATH", "/nonexistent"),
    ];
    assert_eq!(sync(&["once"], &envs), Some(1));
}
