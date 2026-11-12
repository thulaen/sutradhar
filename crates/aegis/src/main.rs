//! The `aegis` command: `aegis msg FILE`, `aegis commit`,
//! `aegis push BASE HEAD`, `aegis catalogue`, which lists the gate
//! items no check covers yet, and `aegis fingerprint`, which prints the
//! staged change's fingerprint for review records, and `aegis sbom`,
//! which writes the list of every part we build with and ship.

use std::io::{self, Write};
use std::process::ExitCode;

use aegis::repo::Repo;
use aegis::{Finding, commit_msg, tiers};

const USAGE: &str = "usage: aegis msg FILE | aegis commit \
     | aegis push BASE HEAD | aegis catalogue | aegis fingerprint \
     | aegis sbom\n";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let refs: Vec<&str> = args.iter().map(String::as_str).collect();
    if refs == ["catalogue"] {
        return catalogue();
    }
    if refs == ["sbom"] {
        return sbom();
    }
    if refs == ["fingerprint"] {
        return match aegis::reviews::staged(&Repo::for_hook(".")) {
            Ok(change) => {
                let _ignored = writeln!(io::stdout(), "{change}");
                ExitCode::SUCCESS
            }
            Err(problem) => stop(&problem),
        };
    }
    match run(&refs) {
        Ok(findings) => {
            // A whole change, not a message alone, gets its score.
            if refs.first() != Some(&"msg") {
                let score = aegis::score::of(&findings);
                say(&format!("{}\n", aegis::score::line(&score)));
            }
            report(&findings)
        }
        Err(problem) => stop(&problem),
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
            out.extend(aegis::tdd::check(&repo, &text, &staged, "HEAD", ""));
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

/// Says why the command could not run, and gives its exit code, 2.
fn stop(problem: &str) -> ExitCode {
    say(&format!("aegis: {problem}\n"));
    ExitCode::from(2)
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
        Err(problem) => stop(&problem),
    }
}

/// Writes the list of every part we build with and ship from the files
/// in this folder, read as the gate reads them: never through a link,
/// and capped in size. The list is written to `gate/sbom.cdx.json.new`
/// and then renamed into place, so it never writes through a link to
/// the list itself; a failed run removes the new file.
fn sbom() -> ExitCode {
    let repo = Repo::for_hook(".");
    let read = |p: &str| {
        repo.read(p).ok_or_else(|| {
            format!("cannot read {p}: missing, a link or too large")
        })
    };
    let path = aegis::sbom::PATH;
    let written = read("Cargo.lock")
        .and_then(|lock| {
            let tools = read("gate/tools.toml")?;
            aegis::sbom::build(&lock, &tools, &read("Cargo.toml")?)
        })
        .and_then(|text| {
            if repo.is_link(path) {
                return Err(format!("{path} or its folder is a link"));
            }
            replace(&repo.content().join(path), &text)
        });
    match written {
        Ok(()) => {
            say(&format!("aegis: wrote {path}\n"));
            ExitCode::SUCCESS
        }
        Err(problem) => stop(&problem),
    }
}

/// Writes `text` to a new file beside `at`, saved to disk, then renames
/// it over `at`. On any failure the new file is removed.
fn replace(at: &std::path::Path, text: &str) -> Result<(), String> {
    let new = at.with_extension("json.new");
    let shown = new.display();
    if let Ok(m) = std::fs::symlink_metadata(&new) {
        if m.is_dir() {
            return Err(format!("{shown} is a folder; remove it"));
        }
        std::fs::remove_file(&new)
            .map_err(|e| format!("cannot remove {shown}: {e}"))?;
    }
    let done = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&new)
        .and_then(|mut f| {
            f.write_all(text.as_bytes())?;
            f.sync_all()
        })
        .and_then(|()| std::fs::rename(&new, at));
    done.map_err(|e| {
        let _gone = std::fs::remove_file(&new);
        format!("cannot write {}: {e}", at.display())
    })
}
