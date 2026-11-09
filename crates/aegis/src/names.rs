//! The names check (task #48; AGENTS.md section 4: code modules may carry
//! Indian names, each listed in the glossary). Every crate's name must be
//! in the glossary's `## Module names` table, so each module's name and
//! meaning are written down in one place.

use crate::Finding;

const CHECK: &str = "docs.names";

/// Where the names live.
pub const GLOSSARY: &str = "docs/glossary.md";

/// The names in the first column of the `## Module names` table, in lower
/// case, without its header row.
fn names(glossary: &str) -> Vec<String> {
    let section = glossary
        .lines()
        .skip_while(|l| l.trim() != "## Module names")
        .skip(1)
        .take_while(|l| !l.starts_with("## "));
    section
        .filter(|l| !l.starts_with("| Name |"))
        .filter_map(|l| l.strip_prefix("| "))
        .filter_map(|rest| rest.split(" |").next())
        .map(|n| n.trim().to_lowercase())
        .collect()
}

/// Checks that every crate in `paths` (folders under `crates/` with a
/// `Cargo.toml`) is named in the glossary.
#[must_use]
pub fn check(glossary: Option<&str>, paths: &[String]) -> Vec<Finding> {
    let Some(text) = glossary else {
        return vec![Finding::new(
            CHECK,
            GLOSSARY,
            0,
            "the glossary is missing",
        )];
    };
    let known = names(text);
    paths
        .iter()
        .filter_map(|p| p.strip_prefix("crates/")?.strip_suffix("/Cargo.toml"))
        .filter(|name| !name.contains('/'))
        .filter(|name| !known.contains(&name.to_lowercase()))
        .map(|name| {
            Finding::new(
                CHECK,
                &format!("crates/{name}/Cargo.toml"),
                0,
                format!("the crate `{name}` is not named in the glossary"),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::check;

    const GLOSSARY: &str = "## Module names\n\n\
        | Name | Meaning | What it is |\n|---|---|---|\n\
        | shodhan | Hindi | Text engine |\n\n## Terms\n\n\
        | Term | Meaning |\n|---|---|\n| Gate | The checks |\n";

    fn paths(names: &[&str]) -> Vec<String> {
        names
            .iter()
            .map(|n| format!("crates/{n}/Cargo.toml"))
            .collect()
    }

    #[test]
    fn named_crates_pass() {
        assert_eq!(check(Some(GLOSSARY), &paths(&["shodhan"])), Vec::new());
    }

    #[test]
    fn a_term_or_a_header_word_is_not_a_module_name() {
        for name in ["gate", "name", "term"] {
            assert_eq!(
                check(Some(GLOSSARY), &paths(&[name])).len(),
                1,
                "{name}"
            );
        }
        assert_eq!(check(Some(GLOSSARY), &paths(&["ShodHan"])), Vec::new());
    }

    #[test]
    fn refuses_a_crate_missing_from_the_glossary() {
        let found = check(Some(GLOSSARY), &paths(&["shodhan", "mystery"]));
        assert_eq!(found.len(), 1);
        assert!(found.iter().all(|f| f.message.contains("`mystery`")));
        assert_eq!(check(None, &[]).len(), 1);
    }
}
