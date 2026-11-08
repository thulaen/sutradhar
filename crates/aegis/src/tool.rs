//! Running an outside tool as a check. A missing tool fails the check;
//! it never skips it (AGENTS.md section 7).

use std::path::Path;
use std::process::Command;

use crate::Finding;

/// Runs `program` with `args` in `folder`. Returns a finding when the
/// tool fails or cannot start.
#[must_use]
pub fn run(
    folder: &Path,
    check: &'static str,
    program: &str,
    args: &[&str],
) -> Option<Finding> {
    let result = Command::new(program)
        .current_dir(folder)
        .args(args)
        .output();
    match result {
        Ok(out) if out.status.success() => None,
        Ok(out) => Some(Finding::new(
            check,
            "",
            0,
            format!(
                "`{program} {}` failed:\n{}{}",
                args.join(" "),
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            ),
        )),
        Err(e) => Some(Finding::new(
            check,
            "",
            0,
            format!("`{program}` is missing, so the check fails: {e}"),
        )),
    }
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
        assert!(found.message.starts_with("`false a` failed"));
    }

    #[test]
    fn a_missing_tool_fails_the_check() {
        let found =
            run(Path::new("."), "t.x", "no-such-tool-here", &[]).unwrap();
        assert!(found.message.contains("is missing"));
    }
}
