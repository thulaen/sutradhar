//! The secrets and private data check (AGENTS.md section 11, gate item
//! GI-46). The patterns are built from parts, so this file does not
//! match itself.

use crate::Finding;

const CHECK: &str = "secrets.private-data";

/// How a pattern is matched.
enum Matcher {
    /// The line holds this text.
    Contains(String),
    /// The line starts a private key of any kind.
    PrivateKey,
    /// The line holds an AWS access key: the prefix and 16 more capital
    /// letters or digits.
    AwsKey,
}

fn patterns() -> Vec<(Matcher, &'static str)> {
    let text = |parts: [&str; 2]| Matcher::Contains(parts.concat());
    vec![
        (text(["gh", "p_"]), "a GitHub token"),
        (text(["gh", "o_"]), "a GitHub token"),
        (text(["github", "_pat_"]), "a GitHub token"),
        (Matcher::PrivateKey, "a private key"),
        (Matcher::AwsKey, "an AWS key"),
        (text(["/srv", "/data"]), "a private path on Dell"),
        (text(["/ho", "me/"]), "a private home folder"),
        (text(["dell-", "ubuntu"]), "the build computer's account"),
    ]
}

/// Checks one file's text, or any other text such as a commit message.
/// Licence texts are skipped.
#[must_use]
pub fn check(path: &str, text: &str) -> Vec<Finding> {
    if path.starts_with("LICENSES/") || path == "LICENSE" {
        return Vec::new();
    }
    let patterns = patterns();
    let mut out = Vec::new();
    for (index, line) in text.lines().enumerate() {
        for (matcher, what) in &patterns {
            if hit(line, matcher) {
                out.push(Finding::new(
                    CHECK,
                    path,
                    index.saturating_add(1),
                    format!("this line looks like it holds {what}"),
                ));
            }
        }
    }
    out
}

fn hit(line: &str, matcher: &Matcher) -> bool {
    match matcher {
        Matcher::Contains(text) => line.contains(text.as_str()),
        Matcher::PrivateKey => line
            .split_once(&["-----BEGIN", " "].concat())
            .is_some_and(|(_, rest)| rest.contains("PRIVATE KEY")),
        Matcher::AwsKey => {
            line.match_indices(&["AK", "IA"].concat()).any(|(at, _)| {
                let rest = line.get(at.saturating_add(4)..).unwrap_or("");
                rest.chars()
                    .take(16)
                    .filter(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
                    .count()
                    == 16
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::check;

    #[test]
    fn finds_planted_secrets() {
        let begin = ["-----BEGIN", " "].concat();
        let planted = [
            ["token = gh", "p_abc"].concat(),
            ["gh", "o_abc"].concat(),
            ["github", "_pat_abc"].concat(),
            format!("{begin}PRIVATE KEY-----"),
            format!("{begin}OPENSSH PRIVATE KEY-----"),
            format!("{begin}RSA PRIVATE KEY-----"),
            format!("{begin}EC PRIVATE KEY-----"),
            format!("{begin}ENCRYPTED PRIVATE KEY-----"),
            format!("{begin}PGP PRIVATE KEY BLOCK-----"),
            ["key AK", "IAABCDEFGHIJKLMNOP"].concat(),
            ["see /srv", "/data/x"].concat(),
            ["/ho", "me/someone"].concat(),
            ["dell-", "ubuntu-01"].concat(),
        ];
        for line in planted {
            assert_eq!(check("a.md", &line).len(), 1, "{line}");
        }
    }

    #[test]
    fn passes_clean_text() {
        let begin = ["-----BEGIN", " "].concat();
        let text = format!(
            "AKIA is a prefix\nhttp://127.0.0.1:3000\nhome/page\n\
             {begin}PUBLIC KEY-----"
        );
        assert_eq!(check("a.md", &text).len(), 0);
    }

    #[test]
    fn skips_licence_texts() {
        let line = ["/ho", "me/x"].concat();
        assert_eq!(check("LICENSES/A.txt", &line).len(), 0);
        assert_eq!(check("LICENSE", &line).len(), 0);
    }

    #[test]
    fn reports_the_line_number() {
        let text = ["ok\n/srv", "/data"].concat();
        assert_eq!(check("a.md", &text)[0].line, 2);
    }
}
