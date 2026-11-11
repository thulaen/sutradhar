#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use super::{LAST_DAY, TASK, day_of, not_allowed, read, stale};

const HOLE: &str = "CVE-2026-4738";
const GONE: &str = "GO-2026-6888";

fn entry(image: &str, id: &str, until: &str, task: u32) -> String {
    format!(
        "[[hole]]\nimage = \"{image}\"\nid = \"{id}\"\ntask = {task}\n\
         until = \"{until}\"\n"
    )
}

fn good(id: &str) -> String {
    entry("forgejo", id, LAST_DAY, TASK)
}

fn pinned() -> Vec<String> {
    vec!["forgejo".to_owned(), "ubuntu-image".to_owned()]
}

fn hole(id: &str, level: &str, part: &str) -> super::super::Hole {
    super::super::Hole {
        id: id.into(),
        level: level.into(),
        part: part.into(),
    }
}

#[test]
fn days_are_counted_from_1970() {
    assert_eq!(day_of("1970-01-01"), Some(0));
    assert_eq!(day_of("2026-11-09"), Some(20_766));
    assert_eq!(day_of("2026-13-01"), None);
    assert_eq!(day_of("26-11-09"), None);
}

#[test]
fn only_known_holes_inside_their_days_are_allowed() {
    let today = day_of("2026-11-09").unwrap();
    assert_eq!(read(None, today, &pinned()).unwrap().len(), 0);
    assert_eq!(read(Some(&good(HOLE)), today, &pinned()).unwrap().len(), 1);
    for bad in [
        good("CVE-2099-NEW"),
        good(""),
        entry("ubuntu-image", HOLE, LAST_DAY, TASK),
        entry("forgejo", HOLE, LAST_DAY, 4242),
        entry("forgejo", HOLE, "2026-12-10", TASK),
        entry("forgejo", HOLE, "2026-11-08", TASK),
        entry("forgejo", HOLE, "soon", TASK),
        format!("{}{}", good(HOLE), good(HOLE)),
        format!("{}extra = 1\n", good(HOLE)),
    ] {
        assert!(read(Some(&bad), today, &pinned()).is_err(), "{bad}");
    }
    // A hole of an image no longer pinned is refused.
    assert!(read(Some(&good(HOLE)), today, &[]).is_err());
    // A day after its end, the same entry is refused.
    let later = day_of("2026-12-10").unwrap();
    assert!(read(Some(&good(HOLE)), later, &pinned()).is_err());
}

#[test]
fn a_listed_hole_passes_a_new_one_blocks_and_a_gone_one_must_go() {
    let today = day_of("2026-11-09").unwrap();
    let list = format!("{}{}", good(HOLE), good(GONE));
    let allowed = read(Some(&list), today, &pinned()).unwrap();
    let mut seen = Vec::new();
    let holes = vec![
        hole(HOLE, "high", "pkg:apk/alpine/zlib@1"),
        hole("CVE-2026-1", "critical", "pkg:apk/alpine/zlib@1"),
    ];
    let kept = not_allowed("forgejo", holes, &allowed, &mut seen);
    assert_eq!(kept, ["CVE-2026-1 (critical) in pkg:apk/alpine/zlib@1"]);
    let other = not_allowed(
        "ubuntu-image",
        vec![hole(HOLE, "high", "x")],
        &allowed,
        &mut seen,
    );
    assert_eq!(other.len(), 1);
    let gone = stale(&allowed, &seen, &["forgejo".to_owned()]);
    assert_eq!(gone.len(), 1);
    assert!(gone[0].contains(GONE), "{gone:?}");
    assert_eq!(stale(&allowed, &seen, &[]), Vec::<String>::new());
}
