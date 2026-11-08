//! The `aegis` command: `aegis msg FILE`, `aegis commit` and
//! `aegis push BASE HEAD`.

use std::io::{self, Write};
use std::process::ExitCode;

use aegis::repo::Repo;
use aegis::{Finding, commit_msg, tiers};

const USAGE: &str =
    "usage: aegis msg FILE | aegis commit | aegis push BASE HEAD\n";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let refs: Vec<&str> = args.iter().map(String::as_str).collect();
    match run(&refs) {
        Ok(findings) => report(&findings),
        Err(problem) => {
            say(&format!("aegis: {problem}\n"));
            ExitCode::from(2)
        }
    }
}

fn run(args: &[&str]) -> Result<Vec<Finding>, String> {
    let repo = Repo::for_hook(".");
    match args {
        ["msg", file] => {
            let text = std::fs::read_to_string(file)
                .map_err(|e| format!("cannot read {file}: {e}"))?;
            Ok(commit_msg::check(&text))
        }
        ["commit"] => tiers::commit(&repo),
        ["push", base, head] => tiers::push(&repo, base, head),
        _ => Err(USAGE.trim_end().to_owned()),
    }
}

fn report(findings: &[Finding]) -> ExitCode {
    if findings.is_empty() {
        say("aegis: every check passed\n");
        return ExitCode::SUCCESS;
    }
    let mut lines: Vec<String> =
        findings.iter().map(ToString::to_string).collect();
    lines.push(format!(
        "aegis: {} problem(s); nothing may go in until they are fixed",
        findings.len()
    ));
    say(&format!("{}\n", lines.join("\n")));
    ExitCode::FAILURE
}

fn say(text: &str) {
    let _ignored = io::stderr().write_all(text.as_bytes());
}
