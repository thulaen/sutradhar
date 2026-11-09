//! Tests of how `images` reads the set-up files: compose and other YAML
//! or JSON, build files, and scripts.

use super::scan_with;
use super::tests::{HEX, good};

#[test]
fn compose_may_not_bring_in_images_from_elsewhere() {
    for bad in [
        "services:\n  a:\n    build: .\n",
        "services:\n  a:\n    build:\n      dockerfile_inline: FROM x\n",
        "services:\n  a:\n    extends:\n      file: ../b.yaml\n",
        "include:\n  - ../b.yaml\n",
        "s:\n  !!binary aW1hZ2U=: alpine:latest\n",
        "s:\n  image: [1]\n",
        "s: [\n",
        "{\"services\": {\"a\": {\"build\": \".\"}}}",
        "services:\n  a:\n    volumes:\n      - type: image\n        \
         source: alpine\n",
        "services:\n  a:\n    provider:\n      type: model\n",
        "models:\n  m:\n    model: ai/x\n",
        "s:\n  v:\n    - type: ${T:-image}\n",
        "s:\n  v:\n    - type: ima${UNSET}ge\n",
    ] {
        let path = if bad.starts_with('{') {
            "docker/x/compose.json"
        } else {
            "docker/x/compose.yaml"
        };
        let used = [(path.to_owned(), bad.to_owned())];
        let found = scan_with("/no/grype", Err("no db".into()), &good(), &used);
        assert!(found.iter().any(|f| f.path == path), "{bad}");
    }
}

#[test]
fn dockerfiles_are_refused_and_scripts_name_their_images() {
    let pinned = format!("codeberg.org/forgejo/forgejo@sha256:{HEX}");
    let count = |path: &str, text: String| {
        let used = [(path.to_owned(), text)];
        scan_with("/no/grype", Err("no db".into()), &good(), &used)
            .into_iter()
            .filter(|f| f.path == path)
            .count()
    };
    // Even a Dockerfile naming only the pinned image, until task #132.
    for name in [
        "docker/x/Dockerfile",
        "docker/x/Containerfile",
        "deploy/app.dockerfile",
        "deploy/DOCKERFILE.dev",
        "deploy/app.containerfile",
        "docker/Dockerfile.yml",
        "docker/Containerfile.YAML",
        "docker/docker-bake.json",
        "docker/docker-bake.override.hcl",
    ] {
        assert_eq!(count(name, format!("FROM {pinned}\n")), 1, "{name}");
    }
    let script = "deploy/x.sh";
    assert_eq!(
        count(script, "add --labels dell:docker://ubuntu:26.04\n".into()),
        1
    );
    assert_eq!(count(script, format!("run docker://{pinned}\n")), 0);
}
