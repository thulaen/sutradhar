use super::check;

const LIST: &str = "[[package]]\nname = \"serde\"\njob = \"typed data\"\n\n\
                    [[package]]\nname = \"toml\"\njob = \"reading TOML\"\n";

fn manifest(deps: &str) -> Vec<(String, String)> {
    vec![(
        "crates/a/Cargo.toml".to_owned(),
        format!("[package]\nname = \"a\"\n\n[dependencies]\n{deps}"),
    )]
}

fn messages(list: &str, deps: &str) -> Vec<String> {
    check(Some(list), None, &manifest(deps))
        .into_iter()
        .map(|f| f.message)
        .collect()
}

#[test]
fn listed_libraries_and_own_crates_pass() {
    let deps =
        "serde = \"1\"\ntoml = { version = \"1\" }\nb = { path = \"../b\" }\n";
    assert_eq!(messages(LIST, deps), Vec::<String>::new());
}

#[test]
fn refuses_a_library_not_on_the_list() {
    let deps = "serde = \"1\"\ntoml = \"1\"\nserde_yaml = \"0.9\"\n";
    let found = messages(LIST, deps);
    assert_eq!(found.len(), 1);
    assert!(found.iter().all(|m| m.contains("`serde_yaml`")));
}

#[test]
fn dev_and_build_libraries_count_too() {
    let text = "[package]\nname = \"a\"\n[dependencies]\nserde = \"1\"\n\
                toml = \"1\"\n[dev-dependencies]\nproptest = \"1\"\n\
                [build-dependencies]\ncc = \"1\"\n";
    let found = check(
        Some(LIST),
        None,
        &[("a/Cargo.toml".to_owned(), text.to_owned())],
    );
    assert_eq!(found.len(), 2);
}

#[test]
fn refuses_two_libraries_for_one_job_and_unused_ones() {
    let two = format!(
        "{LIST}\n[[package]]\nname = \"basic-toml\"\njob = \"Reading TOML\"\n"
    );
    let found =
        messages(&two, "serde = \"1\"\ntoml = \"1\"\nbasic-toml = \"1\"\n");
    assert!(
        found.iter().any(|m| m.contains("two libraries")),
        "{found:?}"
    );
    let unused = messages(LIST, "serde = \"1\"\n");
    assert!(
        unused.iter().any(|m| m.contains("nothing uses")),
        "{unused:?}"
    );
}

#[test]
fn a_missing_or_broken_list_is_refused() {
    assert_eq!(check(None, None, &[]).len(), 1);
    assert_eq!(check(Some("not toml ["), None, &[]).len(), 1);
    let odd = format!("{LIST}note = 1\n");
    assert_eq!(check(Some(&odd), None, &[]).len(), 1);
}

fn one(deps: &str, root: Option<&str>) -> Vec<String> {
    let manifests = vec![(
        "crates/a/Cargo.toml".to_owned(),
        format!("[package]\nname = \"a\"\n{deps}"),
    )];
    check(Some(LIST), root, &manifests)
        .into_iter()
        .map(|f| f.message)
        .collect()
}

const BOTH: &str = "[dependencies]\nserde = \"1\"\ntoml = \"1\"\n";

#[test]
fn every_way_around_the_list_is_refused() {
    for (deps, says) in [
        (
            "[dependencies]\nfoo = { package = \"serde_yaml\" }\n",
            "serde_yaml",
        ),
        (
            "[target.'cfg(unix)'.dependencies]\nlibc = \"0.2\"\n",
            "libc",
        ),
        (
            "[dependencies]\nx = { path = \"/opt/x\" }\n",
            "outside our own",
        ),
        (
            "[dependencies]\nx = { path = \"../../../x\" }\n",
            "outside our own",
        ),
        (
            "[dependencies]\nx = { path = \"../b\", version = \"1\" }\n",
            "outside our own",
        ),
        (
            "[dependencies]\nx = { git = \"https://example.com/x\" }\n",
            "outside crates.io",
        ),
        (
            "[dependencies]\nx = { version = \"1\", registry = \"other\" }\n",
            "outside crates.io",
        ),
        (
            "[patch.crates-io]\nserde = { path = \"../serde\" }\n",
            "patched",
        ),
        (
            "[replace]\n\"serde:1.0.0\" = { path = \"../serde\" }\n",
            "patched",
        ),
        (
            "[dependencies]\nx = { path = \"../../vendor/serde_yaml\" }\n",
            "outside our own",
        ),
        (
            "[dependencies]\nx = { path = \"../b/inner\" }\n",
            "outside our own",
        ),
        (
            "[dependencies]\nx = { registry-index = \"https://x\" }\n",
            "outside crates.io",
        ),
    ] {
        // Cases under [dependencies] join the shared section.
        let deps = deps.strip_prefix("[dependencies]\n").map_or_else(
            || format!("{BOTH}{deps}"),
            |rest| format!("{BOTH}{rest}"),
        );
        let found = one(&deps, None);
        assert!(found.iter().any(|m| m.contains(says)), "{deps}: {found:?}");
    }
}

#[test]
fn shared_workspace_libraries_count_by_their_real_names() {
    let root = "[workspace]\n[workspace.dependencies]\n\
                yaml = { package = \"serde_yaml\", version = \"0.9\" }\n";
    let deps = format!("{BOTH}yaml = {{ workspace = true }}\n");
    let found = one(&deps, Some(root));
    assert!(
        found.iter().any(|m| m.contains("`serde_yaml`")),
        "{found:?}"
    );
    let missing = one(&format!("{BOTH}z = {{ workspace = true }}\n"), None);
    assert!(
        missing.iter().any(|m| m.contains("root lacks")),
        "{missing:?}"
    );
    let patched = "[workspace]\n[patch.crates-io]\nserde = { path = \"x\" }\n";
    assert!(
        one(BOTH, Some(patched))
            .iter()
            .any(|m| m.contains("patched"))
    );
}
