//! Process output must stay bounded and cannot expose raw failure text.
#![allow(clippy::unwrap_used, reason = "tests stop at their first failure")]

use aegis::git_env::run_with_input;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;

fn tool_fixture(mode: &str) -> Option<aegis::Finding> {
    let exe = std::env::current_exe().unwrap();
    aegis::tool::run(
        Path::new("."),
        "fixture.check",
        "env",
        &[
            &format!("AEGIS_CAPTURE_FIXTURE={mode}"),
            exe.to_str().unwrap(),
            "--exact",
            "capture_child_fixture",
            "--nocapture",
            "--skip",
            "synthetic-private-argument",
        ],
    )
}

#[test]
fn tool_refuses_oversized_stdout_even_when_the_child_succeeds() {
    let found = aegis::tool::run(
        Path::new("."),
        "fixture.check",
        "head",
        &["-c", "33554433", "/dev/zero"],
    );
    assert!(found.is_some(), "oversized successful output must fail");
}

#[test]
fn tool_accepts_stdout_at_the_limit_and_binary_output() {
    assert!(
        aegis::tool::run(
            Path::new("."),
            "fixture.check",
            "head",
            &["-c", "33554432", "/dev/zero"],
        )
        .is_none()
    );
    assert!(tool_fixture("binary").is_none());
}

#[test]
fn tool_refuses_oversized_stderr_even_when_the_child_succeeds() {
    let found = tool_fixture("stderr").unwrap();
    assert!(found.message.contains("error output exceeded its limit"));
    assert!(!found.message.contains(&"e".repeat(64)));
}

#[test]
fn tool_accepts_stderr_at_the_limit() {
    assert!(tool_fixture("stderr-at-cap").is_none());
}

#[test]
fn tool_failure_hides_output_and_arguments() {
    let found = tool_fixture("fail").unwrap();
    assert_eq!(found.check, "fixture.check");
    assert!(found.message.contains("failed"));
    for private in [
        "synthetic-private-stdout",
        "synthetic-private-stderr",
        "synthetic-private-argument",
    ] {
        assert!(
            !found.message.contains(private),
            "private child text leaked"
        );
    }
}

#[test]
fn tool_missing_program_hides_its_private_name() {
    let found = aegis::tool::run(
        Path::new("."),
        "fixture.check",
        "/missing-synthetic-private-program",
        &[],
    )
    .unwrap();
    assert!(found.message.contains("fixture.check"));
    assert!(!found.message.contains("synthetic-private-program"));
}

fn fixture(mode: &str) -> Command {
    let mut command = Command::new(std::env::current_exe().unwrap());
    let _set = command
        .args(["--exact", "capture_child_fixture", "--nocapture"])
        .env("AEGIS_CAPTURE_FIXTURE", mode);
    command
}

#[test]
fn oversized_stdout_is_refused() {
    let mut command = Command::new("head");
    let _args = command.args(["-c", "33554433", "/dev/zero"]);
    assert!(run_with_input(&mut command, &[], "fixture", "").is_err());
}

#[test]
fn stdout_at_the_limit_and_stderr_at_the_limit_pass() {
    let mut command = Command::new("head");
    let _args = command.args(["-c", "33554432", "/dev/zero"]);
    assert!(run_with_input(&mut command, &[], "fixture", "").is_ok());
    assert!(
        run_with_input(&mut fixture("stderr-at-cap"), &[], "fixture", "")
            .is_ok()
    );
}

#[test]
fn oversized_stderr_is_refused_even_when_the_child_succeeds() {
    assert!(
        run_with_input(&mut fixture("stderr"), &[], "fixture", "").is_err()
    );
}

#[test]
fn failed_children_do_not_expose_stderr_or_supplied_arguments() {
    let mut command = fixture("fail");
    let _argument = command.arg("synthetic-private-argument");
    let error = run_with_input(
        &mut command,
        &[],
        "fixture",
        "synthetic-private-argument",
    )
    .unwrap_err();
    assert!(!error.contains("synthetic-private-stderr"));
    assert!(!error.contains("synthetic-private-argument"));
}

#[test]
fn input_and_output_are_serviced_concurrently() {
    let input = vec![b'i'; 262_144];
    let mut command = Command::new("timeout");
    let exe = std::env::current_exe().unwrap();
    let _args = command
        .args(["--kill-after=1", "5"])
        .arg(exe)
        .args(["--exact", "capture_child_fixture", "--nocapture"])
        .env("AEGIS_CAPTURE_FIXTURE", "duplex");
    assert!(run_with_input(&mut command, &input, "fixture", "").is_ok());
}

#[test]
fn missing_program_and_incomplete_input_are_refused_safely() {
    let mut absent = Command::new("/missing-synthetic-capture-program");
    assert_eq!(
        run_with_input(&mut absent, &[], "fixture", ""),
        Err("cannot run fixture".into())
    );
    assert!(
        run_with_input(
            &mut fixture("noinput"),
            &vec![b'i'; 262_144],
            "fixture",
            ""
        )
        .is_err()
    );
}

#[test]
fn invalid_utf8_and_timeout_status_are_refused() {
    assert!(
        run_with_input(&mut fixture("binary"), &[], "fixture", "").is_err()
    );
    let mut command = Command::new("timeout");
    let _args = command.args(["--kill-after=1", "0.1", "sleep", "30"]);
    let error = run_with_input(&mut command, &[], "fixture", "").unwrap_err();
    assert!(error.contains("124"));
}

#[test]
fn capture_child_fixture() {
    let Ok(mode) = std::env::var("AEGIS_CAPTURE_FIXTURE") else {
        return;
    };
    if mode == "stderr" {
        std::io::stderr().write_all(&vec![b'e'; 65_537]).unwrap();
    } else if mode == "stderr-at-cap" {
        std::io::stderr().write_all(&vec![b'e'; 65_536]).unwrap();
    } else if mode == "fail" {
        std::io::stdout()
            .write_all(b"synthetic-private-stdout")
            .unwrap();
        std::io::stderr()
            .write_all(b"synthetic-private-stderr")
            .unwrap();
        assert_eq!(mode, "success");
    } else if mode == "duplex" {
        std::io::stdout().write_all(&vec![b'o'; 262_144]).unwrap();
        std::io::stderr().write_all(&vec![b'e'; 32_768]).unwrap();
        let mut input = Vec::new();
        let _read = std::io::stdin().read_to_end(&mut input).unwrap();
        assert_eq!(input, vec![b'i'; 262_144]);
    } else if mode == "binary" {
        std::io::stdout().write_all(&[255]).unwrap();
    }
}
