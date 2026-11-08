//! The one-rule-file check (AGENTS.md section 12): `CLAUDE.md` and
//! `GEMINI.md` only point to `AGENTS.md`.

use crate::Finding;

const CHECK: &str = "docs.one-rule-file";

/// The exact text each pointer file must hold.
pub const POINTER: &str =
    "Read [AGENTS.md](AGENTS.md). It is the only rule file.\n";

/// The files that must hold only the pointer.
pub const POINTER_FILES: [&str; 2] = ["CLAUDE.md", "GEMINI.md"];

/// Checks one pointer file. `text` is `None` when the file is missing.
#[must_use]
pub fn check(path: &str, text: Option<&str>) -> Option<Finding> {
    match text {
        None => Some(Finding::new(CHECK, path, 0, "the file is missing")),
        Some(t) if t != POINTER => Some(Finding::new(
            CHECK,
            path,
            0,
            "this file may only point to AGENTS.md",
        )),
        Some(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{POINTER, check};

    #[test]
    fn passes_the_pointer() {
        assert!(check("CLAUDE.md", Some(POINTER)).is_none());
    }

    #[test]
    fn refuses_extra_rules_and_missing_files() {
        let extra = format!("{POINTER}Also, never do X.\n");
        assert!(check("CLAUDE.md", Some(&extra)).is_some());
        assert!(check("GEMINI.md", None).is_some());
    }
}
