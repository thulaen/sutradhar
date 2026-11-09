//! The names the door accepts: git object ids and release tags.

/// True for 40 lowercase hexadecimal characters.
#[must_use]
pub fn is_object_id(id: &str) -> bool {
    const OBJECT_ID_LEN: usize = 40;
    id.len() == OBJECT_ID_LEN
        && id.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f'))
}

/// True for `refs/tags/v` followed by a digit and then only digits,
/// letters, `.` or `-`, such as `refs/tags/v1.0.0-rc.1`.
#[must_use]
pub fn is_release_tag(name: &str) -> bool {
    name.strip_prefix("refs/tags/v").is_some_and(|rest| {
        rest.starts_with(|c: char| c.is_ascii_digit())
            && rest
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
    })
}

#[cfg(test)]
mod tests {
    use super::{is_object_id, is_release_tag};

    #[test]
    fn knows_an_object_id() {
        assert!(is_object_id(&"a1".repeat(20)));
        assert!(!is_object_id("a1"));
        assert!(!is_object_id(&"A1".repeat(20)));
        assert!(!is_object_id(&"g1".repeat(20)));
    }

    #[test]
    fn knows_a_release_tag() {
        assert!(is_release_tag("refs/tags/v1.0.0"));
        assert!(is_release_tag("refs/tags/v1.0.0-rc.1"));
        for name in [
            "refs/tags/vanity",
            "refs/tags/v",
            "refs/tags/v1/x",
            "refs/heads/v1",
            "refs/tags/1.0",
        ] {
            assert!(!is_release_tag(name), "{name}");
        }
    }
}
