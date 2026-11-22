//! Pinned Forgejo image: a daily image from `forgejo-experimental` is
//! pruned from Codeberg within weeks, after which the image scan cannot
//! find it and every push is refused. The pin must be a release image.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use std::fs;
use std::path::Path;

#[test]
fn the_forgejo_image_is_pinned_to_a_release_not_a_daily_build() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for file in ["gate/tools.toml", "docker/forgejo/compose.yaml"] {
        // Only the lines that set an image count; comments may tell the
        // history.
        let text: String = fs::read_to_string(root.join(file))
            .unwrap()
            .lines()
            .map(str::trim_start)
            .filter(|l| l.starts_with("image") || l.starts_with("sha256:"))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            text.contains("codeberg.org/forgejo/forgejo"),
            "{file} does not pin the Forgejo release image"
        );
        assert!(
            !text.contains("forgejo-experimental"),
            "{file} pins a daily Forgejo image that Codeberg prunes"
        );
    }
}
