//! The short list of known image holes allowed for now, in
//! `gate/image-holes.toml`; AGENTS.md section 5.7 holds the rule and why.
//! The code keeps it: an entry must be one of the [`KNOWN`] holes found
//! on 9 November 2026, in a pinned image, for task [`TASK`], once, and
//! ending by [`LAST_DAY`]; an entry that has ended, or whose hole is
//! gone, is refused. So the list can only shrink.

use serde::Deserialize;

/// Where the list lives.
pub const PATH: &str = "gate/image-holes.toml";

/// The last day any entry may run to.
pub const LAST_DAY: &str = "2026-12-09";

/// The task that removes every listed hole.
pub const TASK: u32 = 130;

/// The only holes the list may ever name: (image, id), as Grype 0.121.0
/// found them on 9 November 2026.
pub const KNOWN: [(&str, &str); 10] = [
    ("forgejo", "CVE-2026-113351"),
    ("forgejo", "CVE-2026-115506"),
    ("forgejo", "CVE-2026-117610"),
    ("forgejo", "CVE-2026-118370"),
    ("forgejo", "CVE-2026-123158"),
    ("forgejo", "CVE-2026-4738"),
    ("forgejo", "GO-2026-6860"),
    ("forgejo", "GO-2026-6865"),
    ("forgejo", "GO-2026-6888"),
    ("ubuntu-image", "CVE-2026-109518"),
];

/// The first day the clock may read; an earlier one is set back.
const FIRST_DAY: &str = "2026-11-09";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct List {
    #[serde(default)]
    hole: Vec<Entry>,
}

/// One allowed hole: in which pinned tool's image, its id, the task that
/// removes it, and the last day it is allowed.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    /// The tool name in `gate/tools.toml`.
    pub image: String,
    /// The hole's id, such as `CVE-2026-4738`.
    pub id: String,
    /// The task that removes it.
    pub task: u32,
    /// The last day it is allowed, `YYYY-MM-DD`.
    pub until: String,
}

/// The day number of a `YYYY-MM-DD` date (days since 1970-01-01), by
/// Howard Hinnant's days-from-civil: years run from March, so the leap
/// day comes last, and 400 years always hold the same days.
#[must_use]
pub fn day_of(date: &str) -> Option<i64> {
    const FORM: &str = "YYYY-MM-DD";
    const PARTS: usize = 3;
    const MONTHS: i64 = 12;
    const LONGEST_MONTH: i64 = 31;
    const ERA_YEARS: i64 = 400;
    const ERA_DAYS: i64 = 146_097;
    const YEAR_DAYS: i64 = 365;
    const LEAP_EVERY: i64 = 4;
    const NO_LEAP_EVERY: i64 = 100;
    const MARCH: i64 = 3;
    const TO_MARCH: i64 = 9;
    // Days in the months from March, as (153 * month + 2) / 5.
    const MONTH_STEP: i64 = 153;
    const MONTH_ROUND: i64 = 2;
    const MONTH_SPLIT: i64 = 5;
    // Days from 0000-03-01 to 1970-01-01.
    const EPOCH: i64 = 719_468;
    let mut parts = date.splitn(PARTS, '-').map(str::parse::<i64>);
    let (Some(Ok(y)), Some(Ok(m)), Some(Ok(d))) =
        (parts.next(), parts.next(), parts.next())
    else {
        return None;
    };
    let whole = date.len() == FORM.len()
        && (1..=MONTHS).contains(&m)
        && (1..=LONGEST_MONTH).contains(&d);
    if !whole {
        return None;
    }
    let y = if m < MARCH { y.checked_sub(1)? } else { y };
    let era = y.div_euclid(ERA_YEARS);
    let yoe = y.checked_sub(era.checked_mul(ERA_YEARS)?)?;
    let mp = if m >= MARCH {
        m.checked_sub(MARCH)?
    } else {
        m.checked_add(TO_MARCH)?
    };
    let doy = mp
        .checked_mul(MONTH_STEP)?
        .checked_add(MONTH_ROUND)?
        .checked_div(MONTH_SPLIT)?;
    let doy = doy.checked_add(d)?.checked_sub(1)?;
    let doe = yoe.checked_mul(YEAR_DAYS)?.checked_add(yoe / LEAP_EVERY)?;
    let doe = doe.checked_sub(yoe / NO_LEAP_EVERY)?.checked_add(doy)?;
    era.checked_mul(ERA_DAYS)?
        .checked_add(doe)?
        .checked_sub(EPOCH)
}

/// Why one entry is refused, if it is: not a known hole of a pinned
/// image, not task [`TASK`], not a day, past [`LAST_DAY`], or ended
/// before `today`.
fn refused(e: &Entry, today: i64, pinned: &[String]) -> Option<String> {
    let id = &e.id;
    if !KNOWN.iter().any(|(i, h)| *i == e.image && *h == e.id) {
        return Some(format!("`{id}` in `{}` is not a known hole", e.image));
    }
    if !pinned.contains(&e.image) {
        return Some(format!("`{}` is not a pinned image", e.image));
    }
    if e.task != TASK {
        return Some(format!("`{id}` must name task #{TASK}"));
    }
    let final_day = day_of(LAST_DAY).unwrap_or(0);
    match day_of(&e.until) {
        None => Some(format!("`{id}`: `{}` is not a day", e.until)),
        Some(day) if day > final_day => Some(format!(
            "`{id}` may be allowed until {LAST_DAY} at the latest"
        )),
        Some(day) if day < today => Some(format!(
            "`{id}` was allowed until {}; pin a fixed image",
            e.until
        )),
        Some(_) => None,
    }
}

/// The entries of the list, refusing any that is not a known hole of a
/// pinned image (`pinned`), names another task, is listed twice, is past
/// [`LAST_DAY`], or has ended before `today`.
///
/// # Errors
/// With every reason the list is refused.
pub fn read(
    text: Option<&str>,
    today: i64,
    pinned: &[String],
) -> Result<Vec<Entry>, Vec<String>> {
    let Some(text) = text else {
        return Ok(Vec::new());
    };
    let list: List = toml::from_str(text)
        .map_err(|e| vec![format!("{PATH} cannot be read: {}", e.message())])?;
    let mut why: Vec<String> = Vec::new();
    for (n, e) in list.hole.iter().enumerate() {
        why.extend(refused(e, today, pinned));
        let twice = list
            .hole
            .iter()
            .take(n)
            .any(|o| o.image == e.image && o.id == e.id);
        if twice {
            why.push(format!("`{}` in `{}` is listed twice", e.id, e.image));
        }
    }
    if why.is_empty() {
        Ok(list.hole)
    } else {
        Err(why)
    }
}

/// Today's day number.
///
/// # Errors
/// When the clock reads before the list was made, so it is set back.
pub fn today() -> Result<i64, String> {
    const DAY_SECONDS: u64 = 86_400;
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    let day = i64::try_from(secs / DAY_SECONDS).unwrap_or(i64::MAX);
    if day < day_of(FIRST_DAY).unwrap_or(0) {
        return Err(format!(
            "the clock reads before {FIRST_DAY}; set it right"
        ));
    }
    Ok(day)
}

/// The list as the scan reads it: today's entries, for the images
/// `tools` pins.
///
/// # Errors
/// With every reason the list, or the clock, is refused.
pub fn for_scan(
    tools: Option<&str>,
    text: Option<&str>,
) -> Result<Vec<Entry>, Vec<String>> {
    let pinned: Vec<String> = tools
        .and_then(|t| crate::tools_list::images(t).ok())
        .unwrap_or_default()
        .into_iter()
        .map(|(name, _)| name)
        .collect();
    let day = today().map_err(|e| vec![e])?;
    read(text, day, &pinned)
}

/// The holes of tool `name` that the list does not allow; each hole
/// found, allowed or not, is noted in `seen`.
#[must_use]
pub fn not_allowed(
    name: &str,
    holes: Vec<super::Hole>,
    allowed: &[Entry],
    seen: &mut Vec<(String, String)>,
) -> Vec<String> {
    seen.extend(holes.iter().map(|h| (name.to_owned(), h.id.clone())));
    holes
        .into_iter()
        .filter(|h| !allowed.iter().any(|e| e.image == name && e.id == h.id))
        .map(super::Hole::message)
        .collect()
}

/// The entries for holes no longer found in a tool that was scanned in
/// full (`scanned`), which must be removed.
#[must_use]
pub fn stale(
    allowed: &[Entry],
    seen: &[(String, String)],
    scanned: &[String],
) -> Vec<String> {
    allowed
        .iter()
        .filter(|e| scanned.contains(&e.image))
        .filter(|e| !seen.iter().any(|(n, id)| *n == e.image && *id == e.id))
        .map(|e| {
            format!(
                "`{}` in `{}` is gone; remove it from {PATH}",
                e.id, e.image
            )
        })
        .collect()
}

#[cfg(test)]
#[path = "images_allowed_tests.rs"]
mod tests;
