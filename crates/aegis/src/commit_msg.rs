//! The commit message check (AGENTS.md section 9).

use crate::Finding;

const CHECK: &str = "commit.message";
const MAX_SUBJECT: usize = 72;
const TYPES: [&str; 11] = [
    "feat", "fix", "docs", "style", "refactor", "perf", "test", "build", "ci",
    "chore", "revert",
];
const TRAILERS: [&str; 4] = ["TDD:", "Defects:", "Task:", "Signed-off-by:"];

/// Checks one commit message. Lines that start with `#` are ignored, as
/// git ignores them.
#[must_use]
pub fn check(message: &str) -> Vec<Finding> {
    let lines = lines_of(message);
    let mut out = Vec::new();
    match lines.first() {
        Some(subject) => out.extend(check_subject(subject)),
        None => out.push(problem("the message is empty")),
    }
    let block = trailer_block(&lines);
    for trailer in TRAILERS {
        out.extend(check_trailer(block, trailer));
    }
    out
}

/// The lines git keeps: up to the cut line, without `#` comments.
fn lines_of(message: &str) -> Vec<&str> {
    message
        .lines()
        .take_while(|l| !l.starts_with("# ------------------------ >8"))
        .filter(|l| !l.starts_with('#'))
        .map(str::trim_end)
        .collect()
}

/// The value of a trailer such as `TDD:`, read the one way every check
/// reads it: from the last paragraph only.
#[must_use]
pub fn trailer(message: &str, name: &str) -> Option<String> {
    let lines = lines_of(message);
    trailer_block(&lines)
        .iter()
        .find_map(|l| l.strip_prefix(name))
        .map(|v| v.trim().to_owned())
}

/// The last paragraph, after the subject: the only place trailers count.
fn trailer_block<'a>(lines: &'a [&'a str]) -> &'a [&'a str] {
    let after_subject = lines.get(1..).unwrap_or(&[]);
    let end = after_subject
        .iter()
        .rposition(|l| !l.is_empty())
        .map_or(0, |i| i.saturating_add(1));
    let paragraphs = after_subject.get(..end).unwrap_or(&[]);
    let start = paragraphs
        .iter()
        .rposition(|l| l.is_empty())
        .map_or(0, |i| i.saturating_add(1));
    paragraphs.get(start..).unwrap_or(&[])
}

fn problem(message: impl Into<String>) -> Finding {
    Finding::new(CHECK, "", 0, message)
}

fn check_subject(subject: &str) -> Vec<Finding> {
    let mut out = Vec::new();
    if subject.chars().count() > MAX_SUBJECT {
        out.push(problem(format!(
            "the first line is longer than {MAX_SUBJECT} characters"
        )));
    }
    if !is_conventional(subject) {
        out.push(problem(
            "the first line must look like `type(scope): what changed`",
        ));
    }
    out
}

/// True when the line follows Conventional Commits: a known type, an
/// optional scope in brackets, an optional `!`, then `: ` and a summary.
fn is_conventional(subject: &str) -> bool {
    let Some((head, _summary)) = subject.split_once(": ") else {
        return false;
    };
    let head = head.strip_suffix('!').unwrap_or(head);
    let kind = match head.split_once('(') {
        None => head,
        Some((kind, rest)) => match rest.strip_suffix(')') {
            Some(scope) if is_scope(scope) => kind,
            _ => return false,
        },
    };
    TYPES.contains(&kind)
}

fn is_scope(scope: &str) -> bool {
    !scope.is_empty()
        && scope
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

fn check_trailer(lines: &[&str], trailer: &str) -> Option<Finding> {
    let Some(value) = lines.iter().find_map(|l| l.strip_prefix(trailer)) else {
        return Some(problem(format!("the `{trailer}` line is missing")));
    };
    if lines.iter().filter(|l| l.starts_with(trailer)).count() > 1 {
        return Some(problem(format!("the `{trailer}` line is given twice")));
    }
    let value = value.trim();
    if value.is_empty() {
        return Some(problem(format!("the `{trailer}` line is empty")));
    }
    if trailer == "TDD:" && crate::tdd::parse(value).is_none() {
        return Some(problem(
            "`TDD:` must name the test that failed before, as \
             `crate::path::test`, or say `none (docs only)`",
        ));
    }
    if trailer == "Task:" && !is_task(value) {
        return Some(problem(
            "`Task:` must be a Forgejo task such as `#42`, or `bootstrap`",
        ));
    }
    if trailer == "Signed-off-by:" && !is_person(value) {
        return Some(problem(
            "`Signed-off-by:` needs a name and email: `Ann <a@example.com>`",
        ));
    }
    None
}

/// True for `Name <email>`.
fn is_person(value: &str) -> bool {
    let Some((name, rest)) = value.split_once(" <") else {
        return false;
    };
    let Some(email) = rest.strip_suffix('>') else {
        return false;
    };
    !name.trim().is_empty()
        && email
            .split_once('@')
            .is_some_and(|(a, b)| !a.is_empty() && b.contains('.'))
}

fn is_task(value: &str) -> bool {
    value == "bootstrap"
        || value.strip_prefix('#').is_some_and(|n| {
            !n.is_empty() && n.chars().all(|c| c.is_ascii_digit())
        })
}

#[cfg(test)]
mod tests {
    use super::check;

    const GOOD: &str = "feat(shodhan): score links\n\n\
        TDD: shodhan::tests::scores\nDefects: none\nTask: #42\n\
        Signed-off-by: A <a@example.com>\n";

    fn with(subject: &str) -> String {
        GOOD.replacen("feat(shodhan): score links", subject, 1)
    }

    #[test]
    fn accepts_a_good_message() {
        assert_eq!(check(GOOD).len(), 0);
    }

    #[test]
    fn accepts_bootstrap_and_bang_and_no_scope() {
        let m = with("docs!: start").replace("#42", "bootstrap");
        assert_eq!(check(&m).len(), 0);
    }

    #[test]
    fn ignores_comment_lines() {
        let m = format!("# a comment\n{GOOD}");
        assert_eq!(check(&m).len(), 0);
    }

    #[test]
    fn refuses_an_empty_message() {
        assert_eq!(check("").len(), 5);
    }

    #[test]
    fn refuses_a_long_subject() {
        let m = with(&format!("fix: {}", "x".repeat(80)));
        assert_eq!(check(&m).len(), 1);
    }

    #[test]
    fn refuses_bad_subjects() {
        for s in [
            "Fix stuff",
            "wip: thing",
            "feat(): thing",
            "feat(Big): thing",
            "feat(x: thing",
            "feat(x)x: thing",
            "feat:  ",
            "feat:thing",
        ] {
            assert_eq!(check(&with(s)).len(), 1, "{s}");
        }
    }

    #[test]
    fn refuses_missing_and_empty_trailers() {
        let m = GOOD.replace("Defects: none\n", "");
        assert_eq!(check(&m).len(), 1);
        let m = GOOD.replace("TDD: shodhan::tests::scores", "TDD:");
        assert_eq!(check(&m).len(), 1);
    }

    #[test]
    fn stops_at_the_cut_off_line() {
        let m = format!(
            "{GOOD}# ------------------------ >8 ------------------------\n\
             diff --git a/x b/x\n+more\n"
        );
        assert_eq!(check(&m).len(), 0);
    }

    #[test]
    fn trailers_count_only_in_the_last_block() {
        let m = "fix: x\n\nTDD: a\nDefects: none\nTask: #1\n\nmore words\n\
            Signed-off-by: A <a@example.com>\n";
        assert_eq!(check(m).len(), 3);
    }

    #[test]
    fn refuses_a_bad_sign_off() {
        for who in ["x", "A <a>", "<a@b.c>", "A <a@b.c", "A <@b.c>"] {
            let m = GOOD.replace("A <a@example.com>", who);
            assert_eq!(check(&m).len(), 1, "{who}");
        }
    }

    #[test]
    fn refuses_bad_task_values() {
        for t in ["42", "#", "#4x", "later"] {
            let m = GOOD.replace("#42", t);
            assert_eq!(check(&m).len(), 1, "{t}");
        }
    }
}
