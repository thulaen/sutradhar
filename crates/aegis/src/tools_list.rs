//! Reading `gate/tools.toml`, the one list of pinned tools (AGENTS.md
//! section 4). The SBOM and the image check both read it here, so a
//! field the list does not know is refused in one place.

use serde::Deserialize;

/// Where the list lives.
pub const PATH: &str = "gate/tools.toml";

/// The registries an image may come from.
const REGISTRIES: [&str; 4] = [
    "codeberg.org/",
    "code.forgejo.org/",
    "docker.io/",
    "ghcr.io/",
];

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct List {
    tool: Vec<Tool>,
}

/// One pinned tool.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tool {
    /// Its name.
    pub name: String,
    /// Its version.
    pub version: String,
    /// Where it comes from.
    pub source: String,
    /// The SHA-256 of a program file.
    pub sha256: Option<String>,
    /// The digest of an image, `sha256:` and 64 hex characters.
    pub digest: Option<String>,
    /// For an image: its name in its registry.
    pub image: Option<String>,
    /// A note.
    pub note: Option<String>,
}

/// How many hex characters a SHA-256 has.
pub const HEX_LEN: usize = 64;

/// Whether `text` is [`HEX_LEN`] hex characters.
#[must_use]
pub fn is_hex64(text: &str) -> bool {
    text.len() == HEX_LEN && text.bytes().all(|b| b.is_ascii_hexdigit())
}

/// The tools in the list.
///
/// # Errors
/// When the list cannot be read.
pub fn read(text: &str) -> Result<Vec<Tool>, String> {
    toml::from_str::<List>(text)
        .map(|l| l.tool)
        .map_err(|e| format!("{PATH} cannot be read: {}", e.message()))
}

/// The images in the list: (tool name, `image@digest`). An image needs
/// both its name, from a known registry, and its digest.
///
/// # Errors
/// When the list cannot be read, or an image entry is not whole.
pub fn images(text: &str) -> Result<Vec<(String, String)>, String> {
    let mut out = Vec::new();
    for t in read(text)? {
        let name = &t.name;
        match (t.image, t.digest) {
            (None, None) => {}
            (Some(image), Some(digest)) => {
                let plain = image.bytes().all(|b| {
                    b.is_ascii_lowercase()
                        || b.is_ascii_digit()
                        || b"./_-".contains(&b)
                });
                if !plain || !REGISTRIES.iter().any(|r| image.starts_with(r)) {
                    return Err(format!(
                        "`{name}`: the image must be a plain name from {}",
                        REGISTRIES.join(", ")
                    ));
                }
                if !digest.strip_prefix("sha256:").is_some_and(is_hex64) {
                    return Err(format!("`{name}`: the digest is not sha256"));
                }
                out.push((t.name, format!("{image}@{digest}")));
            }
            _ => {
                return Err(format!(
                    "`{name}` needs both an image and its digest, or neither"
                ));
            }
        }
    }
    Ok(out)
}
