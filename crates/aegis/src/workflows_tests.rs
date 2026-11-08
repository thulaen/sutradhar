use super::{check, is_workflow};

const GOOD: &str = "on:\n  push:\n    branches: [main]\njobs:\n  a:\n    \
runs-on: dell\n    steps:\n      - name: x\n        run: echo \"a b\"\n";

fn refused(text: &str) -> bool {
    !check(".forgejo/workflows/a.yml", text).is_empty()
}

#[test]
fn passes_plain_shell_steps_on_dell() {
    assert_eq!(check(".forgejo/workflows/a.yml", GOOD), Vec::new());
    let ours = include_str!("../../../.forgejo/workflows/runner-check.yml");
    assert_eq!(
        check(".forgejo/workflows/runner-check.yml", ours),
        Vec::new()
    );
}

#[test]
fn knows_the_workflow_folders() {
    assert!(is_workflow(".forgejo/workflows/a.yml"));
    assert!(is_workflow(".gitea/workflows/a.yml"));
    assert!(is_workflow(".github/workflows/b.yaml"));
    assert!(!is_workflow("deploy/runner/config.yml"));
}

#[test]
fn refuses_containers_services_actions_and_options() {
    for bad in [
        "    container: ubuntu\n",
        "    services:\n      db: {}\n",
        "      - uses: actions/checkout@v4\n",
        "      - options: --privileged\n",
        "    \"container\": x\n",
        "    a: {container: x}\n",
        "    a: {b: 1, 'services': 2}\n",
    ] {
        assert!(refused(&format!("{GOOD}{bad}")), "{bad}");
    }
}

#[test]
fn refuses_every_other_way_of_writing_a_key() {
    for bad in [
        "    ? container\n    : ubuntu\n",
        "    \"cont\\x61iner\": x\n",
        "      - [uses: actions/checkout@v4]\n",
        "    !!str container: x\n",
        "    &a services: x\n",
        "    a: {b: \"x #\", container: y}\n",
        "    b: &x {container: 1}\n    d: *x\n",
        "    <<: {container: x}\n",
    ] {
        assert!(refused(&format!("{GOOD}{bad}")), "{bad}");
    }
}

#[test]
fn refuses_odd_characters_and_line_breaks() {
    for bad in [
        "    a: b\rcontainer: ubuntu\n",
        "    a: b\u{85}container: ubuntu\n",
        "    a: b\u{2028}container: ubuntu\n",
        "    a:\tb\n",
        "    c\u{43f}ntainer: x\n",
    ] {
        assert!(refused(&format!("{GOOD}{bad}")), "{bad:?}");
    }
}

#[test]
fn refuses_broken_or_many_documents() {
    assert!(refused("a: [\n"));
    assert!(refused(&format!("{GOOD}---\n{GOOD}")));
}

#[test]
fn a_forgejo_workflow_runs_only_on_dell() {
    let other = GOOD.replace("runs-on: dell", "runs-on: ubuntu-latest");
    assert_eq!(check(".forgejo/workflows/a.yml", &other).len(), 1);
    assert_eq!(check(".gitea/workflows/a.yml", &other).len(), 1);
    assert_eq!(check(".github/workflows/a.yml", &other), Vec::new());
    let list = GOOD.replace("runs-on: dell", "runs-on: [dell, other]");
    assert!(refused(&list));
}

#[test]
fn comments_and_values_are_not_keys() {
    let text = format!("{GOOD}# container: x\n");
    assert_eq!(check(".forgejo/workflows/a.yml", &text), Vec::new());
    let block = GOOD.replace(
        "run: echo \"a b\"",
        "run: |\n          echo container: x\n          echo uses: y",
    );
    assert_eq!(check(".forgejo/workflows/a.yml", &block), Vec::new());
}

#[test]
fn a_key_given_twice_is_checked_both_times() {
    let twice = GOOD
        .replace("runs-on: dell", "runs-on: dell\n    runs-on: ubuntu-latest");
    assert!(refused(&twice));
    let first = GOOD
        .replace("runs-on: dell", "runs-on: ubuntu-latest\n    runs-on: dell");
    assert!(refused(&first));
}

#[test]
fn refuses_tags_that_change_what_a_key_means() {
    for bad in [
        "    !!binary Y29udGFpbmVy: {image: x}\n",
        "    !!binary cnVucy1vbg==: other\n",
        "    a: !!str dell\n",
        "    a: !custom [1]\n",
        "    a: !!map {b: 1}\n",
        "    a: {!!binary Y29udGFpbmVy: x}\n",
        "    ? !!binary Y29udGFpbmVy\n    : x\n",
        "    a: !!str\n",
        "    a: &x !!binary Y29udGFpbmVy\n",
    ] {
        assert!(refused(&format!("{GOOD}{bad}")), "{bad}");
    }
    let head = "%TAG !e! tag:yaml.org,2002:\n---\n";
    let directive = format!("{head}{GOOD}    !e!binary Y29udGFpbmVy: x\n");
    assert!(refused(&directive));
}
