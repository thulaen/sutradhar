//! The `aegis` command: `aegis msg FILE`, `aegis commit`,
//! `aegis push BASE HEAD`, and `aegis catalogue`, which lists the gate
//! items no check covers yet.

use std::io::{self, Write};
use std::process::ExitCode;

use aegis::repo::Repo;
use aegis::{Finding, commit_msg, tiers};

const USAGE: &str = "usage: aegis msg FILE | aegis commit \
     | aegis push BASE HEAD | aegis catalogue\n";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let refs: Vec<&str> = args.iter().map(String::as_str).collect();
    if refs == ["catalogue"] {
        return catalogue();
    }
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
            let mut out = commit_msg::check(&text);
            out.extend(aegis::private::check_message(&text));
            let staged = aegis::history::staged_paths(&repo)?;
            out.extend(aegis::tdd::check_paths(&text, &staged));
            Ok(out)
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

/// Says which gate items no check covers yet. It is a progress report,
/// not a check: the release tier needs every item covered.
fn catalogue() -> ExitCode {
    let read = |p: &str| std::fs::read_to_string(p).unwrap_or_default();
    let items = read(aegis::catalogue::ITEMS);
    match aegis::catalogue::uncovered(&read(aegis::catalogue::PATH), &items) {
        Ok((none, partly)) if none.is_empty() && partly.is_empty() => {
            say("aegis: every gate item has a check that covers it in full\n");
            ExitCode::SUCCESS
        }
        Ok((none, partly)) => {
            say(&format!(
                "aegis: {} gate items have no check yet: {}\n\
                 aegis: {} are covered only in part: {}\n",
                none.len(),
                none.join(", "),
                partly.len(),
                partly.join(", ")
            ));
            ExitCode::SUCCESS
        }
        Err(problem) => {
            say(&format!("aegis: {problem}\n"));
            ExitCode::from(2)
        }
    }
}
