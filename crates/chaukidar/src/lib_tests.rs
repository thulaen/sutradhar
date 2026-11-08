use super::{Watched, changes, fingerprint, parse, record};
use forge_sync::run::System;

fn temp(name: &str, text: &str) -> String {
    let path = std::env::temp_dir()
        .join(format!("chaukidar-{name}-{}", std::process::id()));
    std::fs::write(&path, text).unwrap();
    path.display().to_string()
}

#[test]
fn reads_a_watch_list() {
    let list = "# comment\n\nfile door-bin /a/b\ncommand hooks cat /x | sort\n";
    let got = parse(list).unwrap();
    assert_eq!(
        got,
        [
            Watched::File("door-bin".to_owned(), "/a/b".to_owned()),
            Watched::Command("hooks".to_owned(), "cat /x | sort".to_owned()),
        ]
    );
}

#[test]
fn refuses_bad_watch_lists() {
    for list in [
        "file /srv/x /srv/x",
        "file Big x",
        "nonsense",
        "file onlylabel",
        "other a b",
        "file a x\nfile a y",
    ] {
        assert!(parse(list).is_err(), "{list}");
    }
}

#[test]
fn fingerprints_files_and_commands() {
    let run = System::default();
    let path = temp("fp", "hello\n");
    let file = Watched::File("f".to_owned(), path.clone());
    let command = Watched::Command("c".to_owned(), format!("cat {path}"));
    let a = fingerprint(&run, &file).unwrap();
    assert_eq!(a.len(), 64);
    assert_eq!(fingerprint(&run, &command).unwrap(), a);
    let missing = Watched::File("m".to_owned(), "/no/such/file".to_owned());
    assert!(fingerprint(&run, &missing).is_err());
}

#[test]
fn spots_every_kind_of_change() {
    let run = System::default();
    let one = temp("one", "1\n");
    let two = temp("two", "2\n");
    let watched = vec![
        Watched::File("one".to_owned(), one.clone()),
        Watched::File("two".to_owned(), two.clone()),
    ];
    let sums = record(&run, &watched).unwrap();
    assert_eq!(changes(&run, &watched, &sums).len(), 0);
    std::fs::write(&one, "changed\n").unwrap();
    std::fs::remove_file(&two).unwrap();
    let mut more = watched.clone();
    more.push(Watched::File("new".to_owned(), one));
    let fewer = format!("{sums}old abc\n");
    assert_eq!(
        changes(&run, &more, &fewer),
        [
            "`new` has no recorded fingerprint",
            "`old` is no longer watched",
            "`one` has changed",
            "`two` cannot be read",
        ]
    );
}

#[test]
fn recording_fails_when_something_cannot_be_read() {
    let watched = vec![Watched::File("m".to_owned(), "/no/such".to_owned())];
    assert!(record(&System::default(), &watched).is_err());
}

#[test]
fn a_failing_part_or_empty_output_cannot_be_read() {
    let run = System::default();
    let failing =
        Watched::Command("c".to_owned(), "cat /no/such | sort".to_owned());
    assert!(fingerprint(&run, &failing).is_err());
    let empty = Watched::Command("e".to_owned(), "true".to_owned());
    assert!(fingerprint(&run, &empty).is_err());
}
