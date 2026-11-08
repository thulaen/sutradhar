#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use super::{Size, launch, take};

fn dir(name: &str) -> String {
    let path = std::env::temp_dir()
        .join(format!("pankti-{name}-{}", std::process::id()));
    let _ignored = std::fs::remove_dir_all(&path);
    std::fs::create_dir_all(&path).unwrap();
    for name in super::FILES {
        std::fs::write(path.join(name), "").unwrap();
    }
    path.display().to_string()
}

const GAP: Duration = Duration::from_millis(10);
const SHORT: Duration = Duration::from_millis(200);

/// Waits until a job holds the line, so the test does not race it.
fn wait_for_the_line(dir: &str) {
    let line = std::fs::File::open(format!("{dir}/line")).unwrap();
    while line.try_lock().is_ok() {
        line.unlock().unwrap();
        thread::sleep(GAP);
    }
}

/// Starts a job in a thread; the channel says when it got its place.
fn start(dir: &str, size: Size) -> mpsc::Receiver<super::Held> {
    let (send, got) = mpsc::channel();
    let dir = dir.to_owned();
    let _job = thread::spawn(move || {
        let held = take(&dir, size, GAP).unwrap();
        let _ignored = send.send(held);
    });
    got
}

#[test]
fn a_third_job_waits_for_a_free_place() {
    let d = dir("third");
    let one = take(&d, Size::One, GAP).unwrap();
    let _two = take(&d, Size::One, GAP).unwrap();
    let third = start(&d, Size::One);
    assert!(third.recv_timeout(SHORT).is_err(), "the third job ran");
    drop(one);
    assert!(third.recv_timeout(Duration::from_secs(5)).is_ok());
}

#[test]
fn a_benchmark_takes_both_places() {
    let d = dir("both");
    let bench = take(&d, Size::Both, GAP).unwrap();
    let other = start(&d, Size::One);
    assert!(other.recv_timeout(SHORT).is_err(), "a job ran beside it");
    drop(bench);
    assert!(other.recv_timeout(Duration::from_secs(5)).is_ok());
}

#[test]
fn a_benchmark_waits_for_both_and_goes_first() {
    let d = dir("order");
    let one = take(&d, Size::One, GAP).unwrap();
    let bench = start(&d, Size::Both);
    wait_for_the_line(&d);
    let later = start(&d, Size::One);
    assert!(bench.recv_timeout(SHORT).is_err(), "it ran with a job on");
    assert!(
        later.recv_timeout(SHORT).is_err(),
        "a later job jumped ahead"
    );
    drop(one);
    let held = bench.recv_timeout(Duration::from_secs(5)).unwrap();
    assert!(later.recv_timeout(SHORT).is_err());
    drop(held);
    assert!(later.recv_timeout(Duration::from_secs(5)).is_ok());
}

#[test]
fn places_are_free_again_after_a_job_ends() {
    let d = dir("again");
    for _ in 0..3 {
        let a = take(&d, Size::One, GAP).unwrap();
        let b = take(&d, Size::One, GAP).unwrap();
        drop((a, b));
    }
    assert!(take(&d, Size::Both, GAP).is_ok());
}

#[test]
fn a_missing_folder_is_an_error() {
    assert!(take("/no/such/pankti", Size::One, GAP).is_err());
}

#[test]
fn jobs_run_capped_in_the_heavy_slice() {
    let cmd = ["cargo".to_owned(), "test".to_owned()];
    let kept = [("CARGO_HOME".to_owned(), "/c".to_owned())];
    let one = launch(Size::One, 7, 8, &kept, &cmd);
    assert_eq!(one.first().map(String::as_str), Some("sudo"));
    for part in [
        "--slice=sutradhar-heavy.slice",
        "MemoryMax=4G",
        "MemorySwapMax=0",
        "CPUQuota=800%",
        "--reuid=7",
        "--regid=8",
        "--init-groups",
    ] {
        assert!(one.iter().any(|a| a == part), "{part}");
    }
    let tail =
        ["--", "env", "CARGO_HOME=/c", "cargo", "test"].map(str::to_owned);
    assert!(one.ends_with(&tail));
    let both = launch(Size::Both, 7, 8, &[], &cmd);
    assert!(both.iter().any(|a| a == "MemoryMax=8G"));
    assert!(both.iter().any(|a| a == "CPUQuota=1600%"));
}

#[test]
fn a_sealed_job_has_no_network_no_new_rights_and_little_to_write() {
    let cmd = ["cargo", "test", "--target-dir", "/w/target"].map(str::to_owned);
    let kept = [
        ("CARGO_HOME".to_owned(), "/c".to_owned()),
        ("PATH".to_owned(), "/bin".to_owned()),
    ];
    let seal = super::Seal {
        cwd: "/w".to_owned(),
        writable: super::writable(&kept, &cmd),
    };
    assert_eq!(seal.writable, ["/w/target"]);
    let line = super::sealed_launch(Size::One, (7, 8), &seal, &kept, &cmd);
    for part in [
        "--wait",
        "--pipe",
        "User=7",
        "Group=8",
        "NoNewPrivileges=yes",
        "PrivateNetwork=yes",
        "ProtectSystem=strict",
        "ProtectHome=read-only",
        "ReadWritePaths=-/w/target",
        "PrivateTmp=yes",
        "PrivateIPC=yes",
        "InaccessiblePaths=-/run/user -/run/dbus",
        "RuntimeMaxSec=3600",
        "TasksMax=4096",
        "--working-directory=/w",
        "CARGO_HOME=/c",
        "CARGO_NET_OFFLINE=true",
        "MemoryMax=4G",
    ] {
        assert!(line.iter().any(|a| a == part), "{part}");
    }
    assert!(!line.iter().any(|a| a == "--scope" || a == "setpriv"));
    assert!(line.ends_with(&cmd));
}

#[test]
fn the_target_folder_is_found_in_every_form() {
    let kept = [("CARGO_TARGET_DIR".to_owned(), "/t".to_owned())];
    let cmd = ["cargo", "build", "--target-dir=/u"].map(str::to_owned);
    assert_eq!(super::writable(&kept, &cmd), ["/t", "/u"]);
}

#[test]
fn a_writable_path_with_a_space_is_refused() {
    let kept = [("CARGO_TARGET_DIR".to_owned(), "/a b".to_owned())];
    assert!(super::check_writable(&super::writable(&kept, &[])).is_err());
    assert!(super::check_writable(&["/w/target".to_owned()]).is_ok());
    for bad in ["/", "target", "//", ""] {
        assert!(super::check_writable(&[bad.to_owned()]).is_err(), "{bad}");
    }
}
