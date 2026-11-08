//! Running the outside tools the sync uses: `curl` for Forgejo, `gh` for
//! GitHub, `git` for the code and gitleaks for secrets.

use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Duration;

/// Something that runs a program with arguments and input, and gives
/// back what it printed. Tests use a stand-in.
pub trait Run {
    /// Runs `program`.
    ///
    /// # Errors
    /// When the program cannot start, fails or runs too long.
    fn run(
        &self,
        program: &str,
        args: &[&str],
        input: &[u8],
    ) -> Result<String, String>;

    /// Waits a moment between writes to GitHub, which limits how fast
    /// they may come.
    fn pause(&self) {}
}

/// Runs real programs, each for at most `limit` seconds, and pauses
/// `gap` between GitHub writes.
#[derive(Debug, Clone, Copy)]
pub struct System {
    /// The most seconds one program may run.
    pub limit: u64,
    /// The pause between GitHub writes.
    pub gap: Duration,
}

impl Default for System {
    fn default() -> Self {
        Self {
            limit: 300,
            gap: Duration::from_secs(1),
        }
    }
}

impl Run for System {
    fn run(
        &self,
        program: &str,
        args: &[&str],
        input: &[u8],
    ) -> Result<String, String> {
        let mut child = Command::new("timeout")
            .arg(self.limit.to_string())
            .arg(program)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("cannot run {program}: {e}"))?;
        let sent = child.stdin.take().map(|mut s| s.write_all(input));
        let out = child
            .wait_with_output()
            .map_err(|e| format!("{program} did not finish: {e}"))?;
        if out.status.success() && !matches!(sent, Some(Err(_))) {
            Ok(String::from_utf8_lossy(&out.stdout).into_owned())
        } else {
            Err(format!(
                "{program} {} failed: {}",
                args.first().copied().unwrap_or_default(),
                String::from_utf8_lossy(&out.stderr).trim()
            ))
        }
    }

    fn pause(&self) {
        std::thread::sleep(self.gap);
    }
}

#[cfg(test)]
mod tests {
    use super::{Run, System};
    use std::time::Duration;

    #[test]
    fn runs_real_programs_within_a_time_limit() {
        let quick = System {
            limit: 1,
            gap: Duration::ZERO,
        };
        assert_eq!(quick.run("cat", &[], b"hi").unwrap(), "hi");
        assert!(quick.run("false", &[], b"").is_err());
        assert!(quick.run("no-such-program-here", &[], b"").is_err());
        assert!(quick.run("sleep", &["5"], b"").is_err());
        quick.pause();
        assert_eq!(System::default().limit, 300);
    }
}
