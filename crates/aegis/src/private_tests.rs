use std::fs;
use std::path::PathBuf;

use super::{Private, check};

fn folder(name: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join(format!("aegis-private-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// A private folder like Dell's: two word lists, the licensed-file index
/// and the list of files that hold real secrets.
fn lists(name: &str) -> PathBuf {
    let dir = folder(name);
    let secret = dir.join("token");
    fs::write(&secret, "s3cr3t-value-123\n").unwrap();
    let licensed =
        format!("sha2 {} library/a.php\n", super::sha256(b"licensed"));
    let files = [
        ("deny-list.txt", "# words\nmyforum\n\n"),
        ("xenforo-phrases.txt", "secret phrase\n"),
        ("xenforo-index.txt", licensed.as_str()),
    ];
    for (file, text) in files {
        fs::write(dir.join(file), text).unwrap();
    }
    fs::write(
        dir.join("credential-files.txt"),
        format!("{}\n", secret.display()),
    )
    .unwrap();
    dir
}

#[test]
fn finds_private_words_licensed_files_and_real_secrets() {
    let private = Private::load(&lists("finds")).unwrap();
    let words = check("a.md", b"Visit MyForum today\n", &private);
    assert_eq!(words.len(), 1);
    assert_eq!(words.first().map(|f| f.line), Some(1));
    assert_eq!(check("b.md", b"a SECRET   phrase\n", &private).len(), 0);
    assert_eq!(check("b.md", b"a secret phrase\n", &private).len(), 1);
    assert_eq!(check("c.php", b"licensed", &private).len(), 1);
    assert_eq!(
        check("d.rs", b"let t = \"s3cr3t-value-123\";\n", &private).len(),
        1
    );
    assert_eq!(
        check("e.rs", b"nothing private here\n", &private),
        Vec::new()
    );
}

#[test]
fn a_finding_never_shows_what_it_matched() {
    let private = Private::load(&lists("hidden")).unwrap();
    for text in [&b"myforum"[..], b"s3cr3t-value-123", b"licensed"] {
        for found in check("a.rs", text, &private) {
            let shown = found.to_string().to_lowercase();
            assert!(!shown.contains("myforum"), "{shown}");
            assert!(!shown.contains("s3cr3t"), "{shown}");
        }
    }
}

#[test]
fn a_missing_list_is_an_error_not_an_empty_list() {
    let dir = folder("missing");
    assert!(Private::load(&dir).is_err());
    let dir = lists("one-missing");
    fs::remove_file(dir.join("xenforo-index.txt")).unwrap();
    assert!(Private::load(&dir).is_err());
}

#[test]
fn an_unreadable_secret_file_is_an_error() {
    let dir = lists("bad-secret");
    fs::write(dir.join("credential-files.txt"), "/no/such/file\n").unwrap();
    assert!(Private::load(&dir).is_err());
}

#[test]
fn the_lists_never_show_in_debug_output() {
    let private = Private::load(&lists("debug")).unwrap();
    let shown = format!("{private:?}");
    assert!(
        !shown.contains("myforum") && !shown.contains("s3cr3t"),
        "{shown}"
    );
    assert!(shown.contains("1 secrets"), "{shown}");
}

#[test]
fn every_line_and_value_of_a_secret_file_counts() {
    let dir = lists("many-lines");
    let env = dir.join("env");
    // The made-up value is built from parts, so no leak scan mistakes
    // this file for one holding a real key.
    let value = ["abc", "def123456"].concat();
    fs::write(
        &env,
        format!("API_KEY={value}\nuser:host:pw987654321\nshort\n"),
    )
    .unwrap();
    fs::write(
        dir.join("credential-files.txt"),
        format!("{}\n", env.display()),
    )
    .unwrap();
    let private = Private::load(&dir).unwrap();
    for text in [format!("key: {value}"), "pass pw987654321 here".to_owned()] {
        assert_eq!(check("a.rs", text.as_bytes(), &private).len(), 1, "{text}");
    }
    assert_eq!(check("a.rs", b"short and sweet", &private), Vec::new());
    fs::write(&env, "\n").unwrap();
    assert!(Private::load(&dir).is_err(), "an empty secret file passed");
}
