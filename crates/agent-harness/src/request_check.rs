//! The patterns every value in a request must match (spec 0155,
//! "Requests"). Nothing here echoes the value it refuses.

/// The longest request id.
const ID_MAX: usize = 64;
/// The shortest request id, long enough to be random.
const ID_MIN: usize = 16;
/// The longest path, in bytes.
const PATH_MAX: usize = 512;
/// The most paths in one reservation.
pub const PATHS_MAX: usize = 64;
/// The longest title.
pub const TITLE_MAX: usize = 200;
/// The longest comment or task body.
pub const BODY_MAX: usize = 32 * 1024;
/// The largest task number.
pub const NUMBER_MAX: u64 = 999_999;
/// The longest lease, one hour in milliseconds.
pub const LEASE_MAX_MS: u64 = 3_600_000;
/// The number of dot-separated numbers in a release tag, `1.2.3`.
const TAG_NUMBERS: usize = 3;
/// The most digits in one number of a release tag.
const TAG_DIGITS_MAX: usize = 9;
/// The longest release tag suffix.
const TAG_SUFFIX_MAX: usize = 32;
/// The longest word in a label.
const LABEL_WORD_MAX: usize = 32;
/// The longest mailbox name.
const NAME_MAX: usize = 32;
/// The label of an urgent task.
pub const URGENT_LABEL: &str = "status/urgent";
/// The only reasons an agent may give for an urgent task (AGENTS.md
/// section 8).
pub const URGENT_REASONS: [&str; 4] =
    ["broken-rule", "failed-sync", "failed-release", "security"];
/// The titles of the alarms the harness keeps out of agents' hands.
const ALARM_TITLES: [&str; 3] = [
    "chaukidar: something that guards the project changed",
    "chaukidar: GitHub's main has stopped following Forgejo's",
    "forge-sync: the copy to GitHub had problems",
];

/// A request id: 16 to 64 letters, digits, `-` or `_`.
pub fn is_id(text: &str) -> bool {
    (ID_MIN..=ID_MAX).contains(&text.len())
        && text
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}

/// A full commit id: 40 or 64 lower-case hex digits.
pub fn is_commit(text: &str) -> bool {
    matches!(text.len(), 40 | 64)
        && text.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

/// A release tag name: `v1.2.3`, with an optional `-suffix`.
pub fn is_release_tag(text: &str) -> bool {
    let Some(rest) = text.strip_prefix('v') else {
        return false;
    };
    let (core, suffix) = match rest.split_once('-') {
        Some((core, suffix)) => (core, Some(suffix)),
        None => (rest, None),
    };
    let parts: Vec<&str> = core.split('.').collect();
    let numbers = parts.len() == TAG_NUMBERS
        && parts.iter().all(|p| {
            !p.is_empty()
                && p.len() <= TAG_DIGITS_MAX
                && p.bytes().all(|b| b.is_ascii_digit())
        });
    numbers
        && !std::path::Path::new(text)
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("lock"))
        && suffix.is_none_or(|s| {
            s.len() <= TAG_SUFFIX_MAX
                && s.split('.').all(|part| {
                    !part.is_empty()
                        && part.bytes().all(|b| b.is_ascii_alphanumeric())
                })
        })
}

/// A relative path with no `..`, no empty part, no leading `-` and no
/// control character.
pub fn is_path(text: &str) -> bool {
    !text.is_empty()
        && text.len() <= PATH_MAX
        && !text.starts_with('/')
        && !text.starts_with('-')
        && !text.chars().any(is_invisible)
        && text
            .split('/')
            .all(|part| !part.is_empty() && part != "." && part != "..")
}

/// A label an agent may set: `status/` and lower-case words, never the
/// owner's `alarm` label.
pub fn is_agent_label(text: &str) -> bool {
    text.strip_prefix("status/").is_some_and(|word| {
        !word.is_empty()
            && word.len() <= LABEL_WORD_MAX
            && word.bytes().all(|b| b.is_ascii_lowercase() || b == b'-')
    })
}

/// A mailbox name: lower-case letters, digits, `-` and `_`.
pub fn is_agent_name(text: &str) -> bool {
    !text.is_empty()
        && text.len() <= NAME_MAX
        && text.bytes().all(|b| {
            b.is_ascii_lowercase()
                || b.is_ascii_digit()
                || b == b'-'
                || b == b'_'
        })
}

/// A character that is invisible or changes how text is drawn, such as a
/// zero-width space or a right-to-left override. None may appear in any
/// request text.
fn is_invisible(c: char) -> bool {
    c.is_control()
        || matches!(
            c,
            '\u{00AD}'
                | '\u{034F}'
                | '\u{061C}'
                | '\u{115F}'
                | '\u{1160}'
                | '\u{17B4}'
                | '\u{17B5}'
                | '\u{180B}'..='\u{180E}'
                | '\u{200B}'..='\u{200F}'
                | '\u{2028}'..='\u{202E}'
                | '\u{2060}'..='\u{206F}'
                | '\u{3164}'
                | '\u{FE00}'..='\u{FE0F}'
                | '\u{FEFF}'
                | '\u{FFA0}'
                | '\u{FFF9}'..='\u{FFFB}'
                | '\u{E0000}'..='\u{E0FFF}'
        )
}

/// Free text: no invisible character, except newline and tab.
pub fn is_text(text: &str, max: usize) -> bool {
    text.len() <= max
        && !text
            .chars()
            .any(|c| is_invisible(c) && c != '\n' && c != '\t')
}

/// One line of text that is not blank: no invisible character at all.
pub fn is_line(text: &str, max: usize) -> bool {
    !text.trim().is_empty()
        && text.len() <= max
        && !text.chars().any(is_invisible)
}

/// Whether a body has a line that imitates the harness's own mark, the
/// first line `[agent: NAME]` that tells agents apart.
pub fn forges_agent_mark(text: &str) -> bool {
    text.lines()
        .any(|line| line.trim_start().to_lowercase().starts_with("[agent"))
}

/// Whether a title is, or starts like, one an alarm uses. Case and
/// spacing do not matter, and no title may start with the name of an
/// alarm producer.
pub fn is_alarm_title(text: &str) -> bool {
    let plain = text
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    plain.starts_with("chaukidar:")
        || plain.starts_with("forge-sync:")
        || ALARM_TITLES
            .iter()
            .any(|title| plain == title.to_lowercase())
}
