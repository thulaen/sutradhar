use std::path::Path;

use super::{ITEMS, PATH, REPORTED, check};

const ITEM_LIST: &str = "| ID | Item |\n|---|---|\n| GI-01 | Bugs |\n";

fn files() -> Vec<(String, String)> {
    vec![(
        "crates/aegis/src/a.rs".to_owned(),
        "#[test]\nfn proves_it() {}\n".to_owned(),
    )]
}

fn entry(id: &str) -> String {
    format!(
        "[[check]]\nid = \"{id}\"\nitems = [\"GI-01\"]\n\
         tiers = [\"commit\"]\nproof = \"aegis::a::tests::proves_it\"\n"
    )
}

fn full() -> String {
    REPORTED.iter().map(|id| entry(id)).collect()
}

fn run(catalogue: &str) -> Vec<String> {
    let read = |p: &str| match p {
        PATH => Some(catalogue.to_owned()),
        ITEMS => Some(ITEM_LIST.to_owned()),
        _ => None,
    };
    check(&read, &files())
        .into_iter()
        .map(|f| f.message)
        .collect()
}

#[test]
fn a_full_and_true_catalogue_passes() {
    assert_eq!(run(&full()), Vec::<String>::new());
}

#[test]
fn refuses_a_check_missing_from_the_catalogue() {
    let without = full().replacen(&entry("text.clean"), "", 1);
    let found = run(&without);
    assert_eq!(found.len(), 1);
    assert!(found.iter().all(|m| m.contains("`text.clean`")));
}

#[test]
fn refuses_extra_repeated_and_wrong_entries() {
    let extra = format!("{}{}", full(), entry("made.up"));
    assert!(run(&extra).iter().any(|m| m.contains("never reports")));
    let twice = format!("{}{}", full(), entry("text.clean"));
    assert!(run(&twice).iter().any(|m| m.contains("listed twice")));
    let item = full().replacen("GI-01", "GI-99", 1);
    assert!(run(&item).iter().any(|m| m.contains("not a gate item")));
    let tier = full().replacen("\"commit\"", "\"someday\"", 1);
    assert!(
        run(&tier)
            .iter()
            .any(|m| m.contains("commit, push or release"))
    );
    let proof = full().replacen("proves_it", "no_such_test", 1);
    assert!(run(&proof).iter().any(|m| m.contains("not a test")));
    assert!(run("not toml [").iter().any(|m| m.contains("cannot read")));
    let odd = full().replacen("proof =", "note = \"x\"\nproof =", 1);
    assert!(run(&odd).iter().any(|m| m.contains("cannot read")));
}

#[test]
fn a_missing_catalogue_is_refused() {
    let found = check(&|_: &str| None, &[]);
    assert_eq!(found.len(), 1);
}

/// The check names in `code`, read where the gate reports them: the first
/// argument of `Finding::new`, a `CHECK` constant, the second argument of
/// `tool::run`, and the name in a `("name", &args)` step. A finding made
/// any other way fails, so no check can be reported without being seen.
/// `Finding {` after `-> ` or `impl ` names the type; anywhere else
/// outside its own file it builds one, which the scan cannot see.
fn refuse_built_findings(file: &str, code: &str) {
    let built = code.match_indices("Finding {").any(|(i, _)| {
        let before = code.get(..i).unwrap_or("");
        !before.ends_with("-> ") && !before.ends_with("impl ")
    });
    assert!(
        file == "finding.rs" || !built,
        "{file} builds a Finding directly"
    );
}

fn reported_in(file: &str, code: &str) -> Vec<String> {
    refuse_built_findings(file, code);
    // Only the tool runner passes a check's name through as `check`.
    let from_name = |args: &str| {
        args.starts_with("CHECK")
            || (file == "tool.rs" && args.starts_with("check"))
    };
    let literal = |rest: &str| -> Option<String> {
        let rest = rest.trim_start().strip_prefix('"')?;
        Some(rest.split('"').next()?.to_owned())
    };
    let mut out = Vec::new();
    for (_, rest) in code
        .match_indices("Finding::new(")
        .map(|(i, _)| code.split_at(i))
    {
        let args = rest.trim_start_matches("Finding::new(").trim_start();
        match literal(args) {
            Some(name) => out.push(name),
            None => assert!(
                from_name(args),
                "a finding is made from `{}`",
                args.lines().next().unwrap_or("")
            ),
        }
    }
    for (_, rest) in code
        .match_indices("const CHECK: &str = ")
        .map(|(i, _)| code.split_at(i))
    {
        out.extend(literal(rest.trim_start_matches("const CHECK: &str = ")));
    }
    for (_, rest) in code
        .match_indices("tool::run(")
        .map(|(i, _)| code.split_at(i))
    {
        let second = rest.split_once(',').map_or("", |(_, r)| r);
        out.extend(literal(second));
    }
    for (_, rest) in code.match_indices("(\"").map(|(i, _)| code.split_at(i)) {
        let Some(name) = literal(rest.trim_start_matches('(')) else {
            continue;
        };
        let id_like = name.contains('.')
            && name
                .chars()
                .all(|c| c.is_ascii_lowercase() || c == '.' || c == '-');
        if !id_like {
            continue;
        }
        let after = rest.get(name.len().saturating_add(3)..).unwrap_or("");
        let after = after.trim_start().strip_prefix(',').unwrap_or("");
        if after.trim_start().starts_with('&') {
            out.push(name);
        }
    }
    out.sort();
    out.dedup();
    out
}

/// Every check name the gate reports is in `REPORTED`, and every name in
/// `REPORTED` is reported, so a new check cannot be left out.
#[test]
fn the_reported_list_matches_the_code() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut found = Vec::new();
    for entry in std::fs::read_dir(&src).unwrap().flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        let rust = Path::new(&name)
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("rs"));
        if rust && !name.ends_with("_tests.rs") {
            let text = std::fs::read_to_string(entry.path()).unwrap();
            // Test code at the end of a file is not part of the gate.
            let code = text.split("#[cfg(test)]").next().unwrap_or("");
            found.extend(reported_in(&name, code));
        }
    }
    found.sort();
    found.dedup();
    let mut listed: Vec<String> =
        REPORTED.iter().map(|s| (*s).to_owned()).collect();
    listed.sort();
    assert_eq!(found, listed);
}

#[test]
fn the_scan_finds_every_way_a_check_is_named() {
    let code = "Finding::new(\n \"a.b\", p)\nconst CHECK: &str = \"c.d\";\n\
                tool::run(f, \"e.f\", x)\nlet s = [(\"g.h\", &[\"x\"])];\n\
                Finding::new(CHECK, p)\nlet not = (\"i.j\", 1);\n";
    assert_eq!(reported_in("tiers.rs", code), ["a.b", "c.d", "e.f", "g.h"]);
}

#[test]
#[should_panic(expected = "a finding is made from")]
fn a_finding_made_some_other_way_fails_the_scan() {
    let _ = reported_in("x.rs", "Finding::new(format!(\"x\"), p)");
}

/// The real catalogue in this repository passes its own check.
#[test]
fn the_catalogue_of_this_repository_passes() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let read = |p: &str| std::fs::read_to_string(root.join(p)).ok();
    let folders: Vec<std::path::PathBuf> =
        std::fs::read_dir(root.join("crates"))
            .unwrap()
            .flatten()
            .flat_map(|c| ["src", "tests"].map(|s| c.path().join(s)))
            .collect();
    let rust: Vec<(String, String)> = folders
        .iter()
        .filter_map(|f| std::fs::read_dir(f).ok())
        .flat_map(|d| d.flatten().map(|e| e.path()))
        .filter_map(|p| {
            let rel = p.strip_prefix(&root).ok()?.display().to_string();
            Some((rel, std::fs::read_to_string(&p).ok()?))
        })
        .collect();
    assert_eq!(check(&read, &rust), Vec::new());
}

#[test]
fn lists_the_gate_items_no_check_covers_yet() {
    let items = "| GI-01 | Bugs |\n| GI-02 | Defects |\n| GI-03 | Paths |\n";
    let part =
        entry("c.d").replace("items = [\"GI-01\"]", "partly = [\"GI-02\"]");
    let (none, partly) =
        super::uncovered(&format!("{}{part}", entry("a.b")), items).unwrap();
    assert_eq!(none, ["GI-03"]);
    assert_eq!(partly, ["GI-02"]);
    assert!(super::uncovered("[", items).is_err());
}

#[test]
fn a_proof_must_be_a_test_in_the_file_its_path_names() {
    let read = |p: &str| match p {
        PATH => Some(full()),
        ITEMS => Some(ITEM_LIST.to_owned()),
        _ => None,
    };
    let plain_fn = vec![(
        "crates/aegis/src/a.rs".to_owned(),
        "fn proves_it() {}\n".to_owned(),
    )];
    assert!(!check(&read, &plain_fn).is_empty(), "a plain fn counted");
    let wrong_file = vec![(
        "crates/aegis/src/b.rs".to_owned(),
        "#[test]\nfn proves_it() {}\n".to_owned(),
    )];
    assert!(
        !check(&read, &wrong_file).is_empty(),
        "the wrong file counted"
    );
    let own_tests_file = vec![(
        "crates/aegis/src/a_tests.rs".to_owned(),
        "#[test]\n#[allow(x)]\nfn proves_it() {}\n".to_owned(),
    )];
    assert_eq!(check(&read, &own_tests_file), Vec::new());
}

#[test]
#[should_panic(expected = "a finding is made from")]
fn only_the_tool_runner_may_pass_a_name_through() {
    let _ = reported_in("tiers.rs", "Finding::new(check, p)");
}

#[test]
#[should_panic(expected = "builds a Finding directly")]
fn a_finding_built_directly_fails_the_scan() {
    let _ = reported_in("x.rs", "let f = Finding { check: \"x.y\" };");
}

#[test]
fn a_missing_item_list_is_an_error_not_full_cover() {
    assert!(super::uncovered(&entry("a.b"), "").is_err());
}
