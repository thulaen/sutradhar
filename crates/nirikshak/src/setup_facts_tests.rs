use crate::rules::tests_common::lines;

const NONE: [usize; 0] = [];

const BAD: &str = "services:\n  web:\n    image: nginx:latest\n    \
                   user: root\n    privileged: true\n    \
                   network_mode: host\n    environment:\n      \
                   DB_PASSWORD: hunter2\n      PLAIN: x\n";

const GOOD: &str = "services:\n  web:\n    \
                    image: \"nginx:1@sha256:abc\"\n    user: \"1000\"\n    \
                    mem_limit: 1g\n    pids_limit: 64\n    \
                    read_only: true\n    \
                    cap_drop:\n      - ALL\n    security_opt:\n      \
                    - no-new-privileges:true\n    environment:\n      \
                    DB_PASSWORD: ${DB_PASSWORD}\n";

#[test]
fn every_setup_mistake_in_a_compose_file_is_found() {
    let bad = [("docker/a/compose.yaml", BAD)];
    for id in ["C01", "C02", "C03", "C04", "C05", "C06", "C07", "C08"] {
        assert_eq!(lines(id, &bad), [2], "{id}");
    }
    assert_eq!(lines("C09", &bad), [8]);
    let good = [("docker/a/compose.yaml", GOOD)];
    for id in [
        "C01", "C02", "C03", "C04", "C05", "C06", "C07", "C08", "C09",
    ] {
        assert_eq!(lines(id, &good), NONE, "{id}");
    }
}

#[test]
fn a_dockerfile_must_pin_its_images_and_end_as_another_user() {
    let bad = "FROM rust:1 AS build\nRUN make\nFROM debian:13\n\
               COPY --from=build /a /a\n";
    let file = [("docker/a/Dockerfile", bad)];
    assert_eq!(lines("C01", &file), [1, 3]);
    assert_eq!(lines("C02", &file), [3]);
    let good = "FROM rust@sha256:1 AS build\nUSER 0\nFROM build\n\
                FROM debian@sha256:2\nUSER app\n";
    let file = [("docker/a/Dockerfile", good)];
    assert_eq!(lines("C01", &file), NONE);
    assert_eq!(lines("C02", &file), NONE);
}

#[test]
fn a_yaml_file_that_is_not_compose_has_no_containers() {
    let other = [("deploy/a/config.yml", "log:\n  level: info\n")];
    assert_eq!(lines("C05", &other), NONE);
}

#[test]
fn a_bare_image_after_a_stage_is_still_an_image() {
    let file = "FROM rust@sha256:1 AS build\nFROM ubuntu\nFROM Build\n";
    let files = [("docker/a/Dockerfile", file)];
    assert_eq!(lines("C01", &files), [2]);
}

#[test]
fn root_in_any_spelling_and_privileged_in_any_spelling_are_found() {
    for user in ["root:root", "0:1000", "ROOT"] {
        let file = format!("services:\n  a:\n    user: \"{user}\"\n");
        assert_eq!(lines("C02", &[("c/compose.yaml", &file)]), [2], "{user}");
    }
    for value in ["yes", "True", "\"true\""] {
        let file = format!("services:\n  a:\n    privileged: {value}\n");
        assert_eq!(lines("C03", &[("c/compose.yaml", &file)]), [2], "{value}");
    }
}

#[test]
fn secrets_in_lists_and_in_defaults_are_found_and_plain_reads_pass() {
    let file = "services:\n  a:\n    environment:\n      \
                - DB_PASSWORD=hunter2\n      - API_TOKEN=${TOKEN:-abc}\n      \
                - SECRET_KEY=${KEY}\n      - GOOD_TOKEN=${T:?set it}\n";
    assert_eq!(lines("C09", &[("c/compose.yaml", file)]), [4, 5]);
}

#[test]
fn limits_under_deploy_count_and_a_zero_limit_does_not() {
    let deploy = "services:\n  a:\n    deploy:\n      resources:\n        \
                  limits:\n          memory: 1g\n          pids: 64\n";
    assert_eq!(lines("C05", &[("c/compose.yaml", deploy)]), NONE);
    let zero = "services:\n  a:\n    mem_limit: 0\n    pids_limit: 64\n";
    assert_eq!(lines("C05", &[("c/compose.yaml", zero)]), [2]);
}

#[test]
fn a_compose_file_that_cannot_be_read_or_uses_aliases_blocks() {
    for bad in ["services:\n  a: [\n", "x: &x {}\nservices:\n  a: *x\n"] {
        let files = vec![("c/compose.yaml".to_owned(), bad.to_owned())];
        let report = crate::analyse(&files);
        assert!(!report.blocked.is_empty(), "{bad}");
    }
}

#[test]
fn a_service_named_like_a_setting_is_found_on_its_own_line() {
    let file = "x:\n  web: 1\nservices:\n  web:\n    privileged: true\n";
    assert_eq!(lines("C03", &[("c/compose.yaml", file)]), [4]);
}

#[test]
fn a_name_nested_in_an_earlier_service_does_not_take_its_line() {
    let file = "services:\n  web:\n    depends_on:\n      db:\n        \
                condition: x\n  db:\n    privileged: true\n";
    assert_eq!(lines("C03", &[("c/compose.yaml", file)]), [6]);
}

#[test]
fn plain_reads_are_not_written_secrets_and_numbers_are_read() {
    let file = "services:\n  a:\n    environment:\n      \
                A_TOKEN: $TOKEN\n      B_TOKEN: ${A}${B}\n      \
                C_TOKEN: ${C:-}\n      D_PASSWORD: 1.5\n";
    assert_eq!(lines("C09", &[("c/compose.yaml", file)]), [7]);
}

#[test]
fn quoted_and_marked_compose_files_are_still_read() {
    let quoted = "\"services\":\n  a:\n    privileged: true\n";
    assert_eq!(lines("C03", &[("c/compose.yaml", quoted)]), [2]);
    let marked = "\u{feff}services:\n  a:\n    privileged: true\n";
    assert_eq!(lines("C03", &[("c/compose.yaml", marked)]), [2]);
}

#[test]
fn dockerfiles_by_any_usual_name_are_read() {
    for name in [
        "Dockerfile.prod",
        "dockerfile",
        "web.Dockerfile",
        "Containerfile",
    ] {
        let path = format!("docker/{name}");
        assert_eq!(
            lines("C01", &[(&path, "FROM debian:13\nUSER a\n")]),
            [1],
            "{name}"
        );
    }
}

#[test]
fn a_one_line_file_with_many_services_is_read_quickly() {
    let many: Vec<String> =
        (0..20_000).map(|n| format!("s{n}: {{}}")).collect();
    let file = format!("services: {{{}}}\n", many.join(", "));
    let started = std::time::Instant::now();
    let found = lines("C05", &[("c/compose.yaml", &file)]);
    assert_eq!(found.len(), 20_000);
    assert!(started.elapsed().as_secs() < 30, "{:?}", started.elapsed());
}

#[test]
fn files_of_their_own_kind_named_like_dockerfiles_keep_their_kind() {
    for name in ["Dockerfile.md", "dockerfile.rs", "Containerfile.yaml"] {
        assert!(
            !crate::reads_as_dockerfile(&format!("docker/{name}")),
            "{name}"
        );
    }
}

#[test]
fn a_setting_is_found_in_its_environment_block_only() {
    let file = "services:\n  a:\n    labels:\n      API_TOKEN: shown\n    \
                environment:\n      API_TOKEN: hunter2\n";
    assert_eq!(lines("C09", &[("c/compose.yaml", file)]), [6]);
}

#[test]
fn a_service_with_very_many_settings_is_read_quickly() {
    let many = (0..100_000).fold(String::new(), |mut all, n| {
        use std::fmt::Write as _;
        let _ok = writeln!(all, "      - A{n}=1");
        all
    });
    let file = format!("services:\n  a:\n    environment:\n{many}");
    let started = std::time::Instant::now();
    let _found = lines("C09", &[("c/compose.yaml", &file)]);
    assert!(started.elapsed().as_secs() < 30, "{:?}", started.elapsed());
}
