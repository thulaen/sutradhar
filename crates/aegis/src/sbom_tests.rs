#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use super::{PATH, build, check, safe};

const SUM: &str =
    "330a5ed07fa54e4702c9d6c4174f74427fc0ef6e214bbd677ae50a5099946470";

fn lock(extra: &str) -> String {
    format!(
        "version = 4\n\n[[package]]\nname = \"a\"\nversion = \"0.1.0\"\n\
         dependencies = [\"b\", \"c 2.0.0\"]\n\n\
         [[package]]\nname = \"b\"\nversion = \"1.0.0\"\n\
         source = \"registry+https://github.com/rust-lang/crates.io-index\"\n\
         checksum = \"{SUM}\"\n\n\
         [[package]]\nname = \"c\"\nversion = \"2.0.0\"\n\
         source = \"registry+https://github.com/rust-lang/crates.io-index\"\n\
         checksum = \"{SUM}\"\n{extra}"
    )
}

fn tools() -> String {
    format!(
        "[[tool]]\nname = \"gitleaks\"\nversion = \"8\"\nsource = \"web\"\n\
         sha256 = \"{SUM}\"\n\n[[tool]]\nname = \"forgejo\"\n\
         version = \"15\"\nsource = \"codeberg\"\ndigest = \"sha256:{SUM}\"\n"
    )
}

const ROOT: &str = "[workspace.package]\nlicense = \"AGPL-3.0-or-later\"\n";

#[test]
fn the_list_names_every_part_with_its_hash() {
    let text = build(&lock(""), &tools(), ROOT).unwrap();
    let bom: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(bom["bomFormat"], "CycloneDX");
    assert_eq!(bom["specVersion"], "1.6");
    let parts = bom["components"].as_array().unwrap();
    let names: Vec<&str> = parts
        .iter()
        .map(|c| c["bom-ref"].as_str().unwrap())
        .collect();
    assert_eq!(
        names,
        [
            "cargo:a@0.1.0",
            "cargo:b@1.0.0",
            "cargo:c@2.0.0",
            "tool:forgejo@15",
            "tool:gitleaks@8"
        ]
    );
    let b = &parts[1];
    assert_eq!(b["purl"], "pkg:cargo/b@1.0.0");
    assert_eq!(b["hashes"][0]["content"], SUM);
    assert_eq!(parts[0]["licenses"][0]["expression"], "AGPL-3.0-or-later");
    assert_eq!(parts[3]["type"], "container");
    assert_eq!(parts[4]["type"], "application");
    let deps = bom["dependencies"].as_array().unwrap();
    let a = deps.iter().find(|d| d["ref"] == "cargo:a@0.1.0").unwrap();
    assert_eq!(
        a["dependsOn"],
        serde_json::json!(["cargo:b@1.0.0", "cargo:c@2.0.0"])
    );
    // The same files always give the same text.
    assert_eq!(text, build(&lock(""), &tools(), ROOT).unwrap());
}

#[test]
fn a_part_from_elsewhere_or_unhashed_is_refused() {
    let git = "\n[[package]]\nname = \"d\"\nversion = \"1.0.0\"\n\
               source = \"git+https://example.org/d\"\n";
    assert!(
        build(&lock(git), &tools(), ROOT)
            .unwrap_err()
            .contains("`d`")
    );
    let bad = "[[tool]]\nname = \"x\"\nversion = \"1\"\nsource = \"s\"\n";
    assert!(build(&lock(""), bad, ROOT).unwrap_err().contains("`x`"));
    let missing = lock("").replace("\"c 2.0.0\"", "\"c 9.0.0\"");
    assert!(
        build(&missing, &tools(), ROOT)
            .unwrap_err()
            .contains("c 9.0.0")
    );
}

#[test]
fn a_missing_or_stale_list_is_refused() {
    let want = build(&lock(""), &tools(), ROOT).unwrap();
    let run = |kept: Option<&str>| {
        check(Some(&lock("")), Some(&tools()), Some(ROOT), kept)
    };
    assert_eq!(run(Some(&want)), Vec::new());
    let stale = want.replace("1.0.0", "1.0.1");
    let found = run(Some(&stale));
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].path, PATH);
    assert!(found[0].message.contains("out of date"));
    assert!(run(None)[0].message.contains("no SBOM"));
    assert_eq!(check(None, None, None, None), Vec::new());
    // A missing input is refused, never read as empty.
    let without = check(Some(&lock("")), None, Some(ROOT), Some(&want));
    assert!(without[0].message.contains("tools.toml is needed"));
    let without = check(Some(&lock("")), Some(&tools()), None, Some(&want));
    assert!(without[0].message.contains("Cargo.toml is needed"));
    for odd in ["license = 5", "license = \" \""] {
        let odd = format!("[workspace.package]\n{odd}\n");
        let bad =
            check(Some(&lock("")), Some(&tools()), Some(&odd), Some(&want));
        assert!(bad[0].message.contains("must name the licence"));
    }
}

#[test]
fn a_version_is_made_safe_in_its_purl() {
    let plus = lock("").replace("\"1.0.0\"", "\"1.0.0+x\"");
    let plus = plus.replace("\"b\", ", "\"b 1.0.0+x\", ");
    let text = build(&plus, &tools(), ROOT).unwrap();
    assert!(text.contains("pkg:cargo/b@1.0.0%2Bx"), "{text}");
    assert_eq!(safe("a b/c"), "a%20b%2Fc");
    let both = format!("{}sha256 = \"{SUM}\"\n", tools());
    assert!(
        build(&lock(""), &both, ROOT)
            .unwrap_err()
            .contains("one hash")
    );
}
