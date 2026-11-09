//! The allowed languages and file types check (AGENTS.md section 4).

use crate::Finding;

const CHECK: &str = "files.allowed-kind";

/// What kind of file a path is, as far as the rules care.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// Rust source.
    Rust,
    /// CSS.
    Css,
    /// Strict TypeScript.
    TypeScript,
    /// PHP inside a site add-on.
    Php,
    /// SQL.
    Sql,
    /// A shell script.
    Shell,
    /// A Cedar or Rego policy file.
    Policy,
    /// TOML, YAML or JSON.
    Config,
    /// A Dockerfile.
    Docker,
    /// Markdown.
    Markdown,
    /// A licence text.
    Licence,
    /// `.gitignore`, `.gitattributes`, or `.gitleaksignore` (the leak
    /// scan's reviewed false alarms).
    Git,
    /// A lock file written by a tool.
    Lock,
}

/// Whether a folder tree holds cargo settings (a `.cargo` folder), which
/// could make cargo start a program from the tree. When it does, no
/// cargo step may run on it at all, not even a fetch. The build and git
/// folders are skipped.
#[must_use]
pub fn has_cargo_settings(folder: &std::path::Path) -> bool {
    let Ok(entries) = std::fs::read_dir(folder) else {
        return false;
    };
    entries.flatten().any(|entry| {
        let name = entry.file_name();
        let is_dir = entry.file_type().is_ok_and(|t| t.is_dir());
        if name == ".cargo" {
            return true;
        }
        is_dir
            && name != "target"
            && name != ".git"
            && has_cargo_settings(&entry.path())
    })
}

/// Works out the kind of a path, or `None` when it is not allowed at
/// all. `typescript_on` says whether the TypeScript compiler is pinned.
#[must_use]
pub fn kind_of(path: &str, typescript_on: bool) -> Option<Kind> {
    // Cargo settings in the tree could change how checks run, so they
    // are never allowed.
    if path.starts_with(".cargo/") || path.contains("/.cargo/") {
        return None;
    }
    let name = path.rsplit('/').next().unwrap_or(path);
    if let Some(kind) = by_name(path, name) {
        return Some(kind);
    }
    let ext = name.rsplit_once('.').map(|(_, e)| e)?;
    match ext {
        "rs" => Some(Kind::Rust),
        "css" if path.starts_with("ui/") || path.starts_with("crates/") => {
            Some(Kind::Css)
        }
        "ts" if typescript_on && path.starts_with("ui/ts/") => {
            Some(Kind::TypeScript)
        }
        "php" if is_addon(path) => Some(Kind::Php),
        "sql" if is_sql_place(path) => Some(Kind::Sql),
        "sh" => Some(Kind::Shell),
        "cedar" | "rego" => Some(Kind::Policy),
        "toml" | "yaml" | "yml" | "json" => Some(Kind::Config),
        "md" => Some(Kind::Markdown),
        "txt" if path.starts_with("LICENSES/") => Some(Kind::Licence),
        _ => None,
    }
}

fn by_name(path: &str, name: &str) -> Option<Kind> {
    match name {
        "LICENSE" if path == "LICENSE" => Some(Kind::Licence),
        ".gitignore" | ".gitattributes" | ".gitleaksignore" => Some(Kind::Git),
        "Cargo.lock" if path == "Cargo.lock" => Some(Kind::Lock),
        "composer.lock" if is_addon(path) => Some(Kind::Lock),
        "Dockerfile" => Some(Kind::Docker),
        _ if path.starts_with(".githooks/") && !name.contains('.') => {
            Some(Kind::Shell)
        }
        _ => None,
    }
}

fn is_sql_place(path: &str) -> bool {
    path.starts_with("migrations/") || path.contains("/tests/")
}

fn is_addon(path: &str) -> bool {
    path.starts_with("addons/xenforo/") || path.starts_with("addons/wordpress/")
}

/// Checks that every path is an allowed kind of file.
#[must_use]
pub fn check<'a>(
    paths: impl IntoIterator<Item = &'a str>,
    typescript_on: bool,
) -> Vec<Finding> {
    paths
        .into_iter()
        .filter(|p| kind_of(p, typescript_on).is_none())
        .map(|p| {
            Finding::new(
                CHECK,
                p,
                0,
                "this kind of file is not allowed (AGENTS.md section 4)",
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{Kind, check, kind_of};

    #[test]
    fn knows_each_allowed_kind() {
        let cases = [
            ("crates/a/src/lib.rs", Kind::Rust),
            ("ui/styles/app.css", Kind::Css),
            ("addons/xenforo/src/A.php", Kind::Php),
            ("addons/wordpress/a.php", Kind::Php),
            ("migrations/0001_init.sql", Kind::Sql),
            ("crates/db/tests/rls.sql", Kind::Sql),
            ("tools/a.sh", Kind::Shell),
            (".githooks/pre-commit", Kind::Shell),
            ("policy/a.cedar", Kind::Policy),
            ("policy/a.rego", Kind::Policy),
            ("Cargo.toml", Kind::Config),
            ("docker/a/compose.yaml", Kind::Config),
            ("a.yml", Kind::Config),
            ("a.json", Kind::Config),
            ("docker/a/Dockerfile", Kind::Docker),
            ("README.md", Kind::Markdown),
            ("LICENSE", Kind::Licence),
            ("LICENSES/GPL-2.0-or-later.txt", Kind::Licence),
            (".gitignore", Kind::Git),
            (".gitattributes", Kind::Git),
            (".gitleaksignore", Kind::Git),
            ("Cargo.lock", Kind::Lock),
            ("addons/xenforo/composer.lock", Kind::Lock),
        ];
        for (path, kind) in cases {
            assert_eq!(kind_of(path, false), Some(kind), "{path}");
        }
    }

    #[test]
    fn typescript_needs_the_compiler_and_its_folder() {
        assert_eq!(kind_of("ui/ts/a.ts", true), Some(Kind::TypeScript));
        assert_eq!(kind_of("ui/ts/a.ts", false), None);
        assert_eq!(kind_of("src/a.ts", true), None);
    }

    #[test]
    fn refuses_planted_files() {
        let planted = [
            "src/a.js",
            "src/a.html",
            "src/a.py",
            "src/a.java",
            "src/a.go",
            "src/a.c",
            "src/a.php",
            "notes.txt",
            "a.css",
            "Makefile",
            "src/noext",
            ".githooks/a.py",
            "src/a.sql",
            "docs/LICENSE",
            "composer.lock",
            "crates/a/Cargo.lock",
            ".cargo/config.toml",
            "crates/a/.cargo/config.toml",
        ];
        assert_eq!(check(planted, true).len(), planted.len());
    }

    #[test]
    fn passes_clean_files() {
        assert_eq!(check(["a.rs", "b.md"], false).len(), 0);
    }
}
