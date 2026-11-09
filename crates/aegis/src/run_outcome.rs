//! What one `cargo test` run of a named test showed, for the test-first
//! check (`red_green`). A run that could not start is never proof.

/// What one run of the named test showed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// It ran and passed.
    Passed,
    /// It ran and failed, or the code did not compile against it.
    Failed,
    /// No test of that name exists.
    Absent,
    /// Cargo could not run at all, for a reason that is not the test's,
    /// such as a folder it may not write. This is never proof.
    Broken(String),
}

/// Which compile errors count as the test failing: any (`None`), as
/// after the change, or only those in the given files, as before the
/// change, where only the commit's own test files may fail to compile.
pub type Counted<'a> = Option<&'a [String]>;

/// Whether `errors` holds a compile error that counts.
fn compile_error(errors: &str, counted: Counted<'_>) -> bool {
    let mut lines = errors.lines();
    while let Some(line) = lines.next() {
        if !line.starts_with("error[E") {
            continue;
        }
        let Some(files) = counted else {
            return true;
        };
        // rustc names the place on the next line: ` --> path:line:col`.
        let place = lines
            .next()
            .and_then(|l| l.trim_start().strip_prefix("--> "))
            .and_then(|p| p.split(':').next());
        if place.is_some_and(|p| files.iter().any(|f| f == p)) {
            return true;
        }
    }
    false
}

/// Reads one `cargo test` run: whether it succeeded, what it wrote to
/// its output and to its error stream, and which compile errors count.
#[must_use]
pub fn outcome(
    succeeded: bool,
    output: &str,
    errors: &str,
    counted: Counted<'_>,
) -> Outcome {
    if !succeeded {
        let ran_and_failed = output
            .lines()
            .any(|l| l.trim().starts_with("test result: FAILED"));
        let did_not_compile = compile_error(errors, counted);
        if ran_and_failed || did_not_compile {
            return Outcome::Failed;
        }
        let why = errors
            .lines()
            .filter(|l| l.trim_start().starts_with("error"))
            .take(3)
            .collect::<Vec<_>>()
            .join("; ");
        return Outcome::Broken(why);
    }
    let passed: usize = output
        .lines()
        .filter_map(|l| l.trim().strip_prefix("test result: ok. "))
        .filter_map(|r| r.split(' ').next()?.parse::<usize>().ok())
        .sum();
    if passed > 0 {
        Outcome::Passed
    } else {
        Outcome::Absent
    }
}
