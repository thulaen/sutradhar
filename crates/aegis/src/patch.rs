//! Reading git's patch output and NUL-separated lists.

/// Every added line in the output of `git log -p`, with the file it was
/// added to. Headers run from `diff --git` to the first `@@`; only lines
/// after that are content, so no content line can pass for a header.
#[must_use]
pub fn added_lines(patch: &str) -> Vec<(String, String)> {
    let mut file = String::new();
    let mut in_header = false;
    let mut out = Vec::new();
    for line in patch.lines() {
        if line.starts_with("diff --git ") {
            in_header = true;
            file.clear();
        } else if in_header {
            if let Some(name) = line.strip_prefix("+++ ") {
                // git ends a name holding a space with a tab.
                let name = name.strip_suffix('\t').unwrap_or(name);
                name.strip_prefix("b/")
                    .unwrap_or(name)
                    .clone_into(&mut file);
            } else if line.starts_with("@@") {
                in_header = false;
            }
        } else if let Some(added) = line.strip_prefix('+') {
            out.push((file.clone(), added.to_owned()));
        }
    }
    out
}

/// The items of a NUL-separated list, as git prints with `-z`.
#[must_use]
pub fn nul_list(text: &str) -> Vec<String> {
    text.split('\0')
        .filter(|l| !l.is_empty())
        .map(str::to_owned)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{added_lines, nul_list};

    #[test]
    fn reads_added_lines_and_ignores_look_alike_headers() {
        let patch = "diff --git a/x b/x\nindex 1..2 100644\n--- a/x\n\
            +++ b/x\n@@ -1,2 +1,2 @@\n--- removed\n+++ added\n+new\n\
            diff --git a/y b/y\n--- a/y\n+++ b/y\n@@ -0,0 +1 @@\n+more\n";
        let pairs = |items: &[(&str, &str)]| -> Vec<(String, String)> {
            items
                .iter()
                .map(|(a, b)| ((*a).to_owned(), (*b).to_owned()))
                .collect()
        };
        assert_eq!(
            added_lines(patch),
            pairs(&[("x", "++ added"), ("x", "new"), ("y", "more")])
        );
    }

    #[test]
    fn a_name_holding_a_space_loses_the_tab_git_adds() {
        let patch = "diff --git a/my x b/my x\n--- a/my x\t\n+++ b/my x\t\n\
            @@ -0,0 +1 @@\n+line\n";
        assert_eq!(
            added_lines(patch),
            [("my x".to_owned(), "line".to_owned())]
        );
    }

    #[test]
    fn splits_nul_lists() {
        assert_eq!(nul_list("a\0b c\0\0"), ["a", "b c"]);
    }
}
