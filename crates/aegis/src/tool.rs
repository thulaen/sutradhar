//! Running an outside tool as a check. A missing tool fails the check;
//! it never skips it (AGENTS.md section 7).

use std::path::Path;
use std::process::Command;

use crate::Finding;

/// Runs `program` with `args` in `folder`. Returns a finding when the
/// tool fails, cannot start or exceeds the shared output limits.
/// Findings contain the trusted check label, an exit status when
/// available, and a fixed complexity lint label when needed by the score.
#[must_use]
pub fn run(
    folder: &Path,
    check: &'static str,
    program: &str,
    args: &[&str],
) -> Option<Finding> {
    let mut command = Command::new(program);
    let _args = command.current_dir(folder).args(args);
    crate::process_capture::run(&mut command, &[], check)
        .and_then(|output| {
            if output.status.success() {
                return Ok(());
            }
            let lint = complexity_lint(check, &output);
            Err(format!("{check} failed ({}){lint}", output.status))
        })
        .err()
        .map(|message| Finding::new(check, "", 0, message))
}

fn complexity_lint(check: &str, output: &std::process::Output) -> String {
    if check != "rust.lints" {
        return String::new();
    }
    crate::score::COMPLEX
        .iter()
        .find(|lint| {
            [&output.stdout, &output.stderr].iter().any(|bytes| {
                bytes
                    .windows(lint.len())
                    .any(|window| window == lint.as_bytes())
            })
        })
        .map_or_else(String::new, |lint| format!(": {lint}"))
}

#[cfg(test)]
mod tests {
    use super::run;
    use std::path::Path;

    #[test]
    fn a_passing_tool_gives_nothing() {
        assert!(run(Path::new("."), "t.x", "true", &[]).is_none());
    }

    #[test]
    fn a_failing_tool_gives_a_finding() {
        let found = run(Path::new("."), "t.x", "false", &["a"]).unwrap();
        assert!(found.message.starts_with("t.x failed (exit status: 1)"));
    }

    #[test]
    fn a_missing_tool_fails_the_check() {
        let found =
            run(Path::new("."), "t.x", "no-such-tool-here", &[]).unwrap();
        assert_eq!(found.message, "cannot run t.x");
    }

    #[test]
    fn complexity_lint_categories_survive_without_private_diagnostics() {
        for lint in [
            "too_many_lines",
            "cognitive_complexity",
            "excessive_nesting",
            "too_many_arguments",
            "fn_params_excessive_bools",
        ] {
            let private = format!("/missing-synthetic-private-{lint}");
            let found =
                run(Path::new("."), "rust.lints", "head", &[&private]).unwrap();
            assert_eq!(
                crate::score::part(&found),
                crate::score::Part::Complexity
            );
            assert!(found.message.contains(lint));
            assert!(!found.message.contains("synthetic-private"));
            let value = format!("AEGIS_TOOL_LINT=synthetic-private-{lint}");
            let stdout = run(
                Path::new("."),
                "rust.lints",
                "env",
                &[&value, "printenv", "AEGIS_TOOL_LINT", ""],
            )
            .unwrap();
            assert_eq!(
                crate::score::part(&stdout),
                crate::score::Part::Complexity
            );
            assert!(stdout.message.contains(lint));
            assert!(!stdout.message.contains("synthetic-private"));
        }
        let plain = run(Path::new("."), "rust.lints", "false", &[]).unwrap();
        assert_eq!(crate::score::part(&plain), crate::score::Part::StyleDocs);
    }
}
