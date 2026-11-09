//! The images the set-up files run, for `images`. AGENTS.md section 5.7
//! lists what is read and what is refused; this file does the reading.
//! YAML is read through `yaml_safe`; a Dockerfile is refused until
//! task #132; and every file is also searched for `docker://` labels
//! and `sha256:` digests.

use yaml_rust2::Yaml;

use super::CHECK;
use crate::Finding;
use crate::tools_list::is_hex64;

/// Compose keys that bring in images, models, programs or files from
/// elsewhere.
const ELSEWHERE: [&str; 5] =
    ["build", "extends", "include", "provider", "models"];

/// Every image a YAML document names (`image:` values, and the part of
/// any text after `docker://`), and every reason to refuse it.
fn walk(node: &Yaml, out: &mut Vec<String>, refused: &mut Vec<String>) {
    match node {
        Yaml::Hash(map) => {
            for (key, value) in map {
                let Some(name) = key.as_str() else {
                    refused.push("every key must be a plain name".to_owned());
                    continue;
                };
                if ELSEWHERE.contains(&name) {
                    refused.push(format!(
                        "`{name}` brings in an image or file from elsewhere, \
                         which is never scanned"
                    ));
                }
                let kind = value.as_str().unwrap_or_default();
                if name == "type"
                    && (kind.eq_ignore_ascii_case("image")
                        || kind.contains('$'))
                {
                    refused.push(
                        "a `type` of `image`, or set by a variable, can mount \
                         an image that is never scanned"
                            .to_owned(),
                    );
                }
                if name == "image" {
                    match value.as_str() {
                        Some(text) => out.push(text.to_owned()),
                        None => refused.push("`image` must be text".to_owned()),
                    }
                }
                walk(value, out, refused);
            }
        }
        Yaml::Array(items) => {
            for item in items {
                walk(item, out, refused);
            }
        }
        Yaml::String(text) => out.extend(labels(text)),
        Yaml::BadValue => refused.push("a value cannot be read".to_owned()),
        _ => {}
    }
}

/// The images `docker://` labels in `text` name.
fn labels(text: &str) -> Vec<String> {
    text.split("docker://")
        .skip(1)
        .map(|rest| {
            let end = rest
                .find(|c: char| c.is_whitespace() || "\"'`,;)]}".contains(c))
                .unwrap_or(rest.len());
            rest.get(..end).unwrap_or_default().to_owned()
        })
        .collect()
}

/// Whether a file name is a build file's, in any case: any name holding
/// `dockerfile` or `containerfile` (`Dockerfile`, `Dockerfile.yml`,
/// `app.containerfile`), or starting with `docker-bake` (bake files,
/// which can name images and Dockerfiles of their own).
fn is_build_file(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.contains("dockerfile")
        || lower.contains("containerfile")
        || lower.starts_with("docker-bake")
}

/// The images a set-up file names.
///
/// # Errors
/// Always for a build file (see [`is_build_file`]), until task #132; and
/// when a YAML or JSON file cannot be read safely, or brings in an image
/// or file from elsewhere.
pub fn named(path: &str, text: &str) -> Result<Vec<String>, String> {
    let name = path.rsplit('/').next().unwrap_or(path);
    if is_build_file(name) {
        return Err("a Dockerfile or bake file is not allowed here until \
                    task #132 gives the gate a reader that matches Docker's"
            .to_owned());
    }
    // JSON is YAML too, so a `compose.json` meets the compose rules.
    let structured = std::path::Path::new(path).extension().is_some_and(|e| {
        ["yaml", "yml", "json"]
            .iter()
            .any(|x| e.eq_ignore_ascii_case(x))
    });
    if structured {
        let docs = crate::yaml_safe::load(text)?;
        let (mut out, mut refused) = (Vec::new(), Vec::new());
        for doc in &docs {
            walk(doc, &mut out, &mut refused);
        }
        return if refused.is_empty() {
            Ok(out)
        } else {
            Err(refused.join("; "))
        };
    }
    Ok(labels(text))
}

/// An image name in full: `ubuntu` is `docker.io/library/ubuntu`, and
/// `a/b` is `docker.io/a/b`.
fn full_name(name: &str) -> String {
    match name.split_once('/') {
        None => format!("docker.io/library/{name}"),
        Some((first, _)) if !first.contains('.') => format!("docker.io/{name}"),
        Some(_) => name.to_owned(),
    }
}

/// Why `image` is not one of the `pinned` (`name@digest`) images, if it
/// is not: it must give a digest, and the name and digest must match.
#[must_use]
pub fn problem(image: &str, pinned: &[(String, String)]) -> Option<String> {
    let Some((name, digest)) = image.split_once('@') else {
        return Some(format!("`{image}` is named without a digest"));
    };
    if !digest.strip_prefix("sha256:").is_some_and(is_hex64) {
        return Some(format!("`{image}` does not give a sha256 digest"));
    }
    // A tag after the last `/` may stay: the digest decides.
    let name = match name.rsplit_once('/') {
        Some((head, last)) => {
            format!("{head}/{}", last.split(':').next().unwrap_or(last))
        }
        None => name.split(':').next().unwrap_or(name).to_owned(),
    };
    let want = format!("{}@{digest}", full_name(&name));
    (!pinned.iter().any(|(_, p)| *p == want)).then(|| {
        format!("`{image}` is not an image gate/tools.toml pins and scans")
    })
}

/// The `sha256:` digests in any text that are not among `pinned`.
#[must_use]
pub fn loose_digests(text: &str, pinned: &[(String, String)]) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = text;
    while let Some(at) = rest.find("sha256:") {
        let tail = rest.get(at.saturating_add(7)..).unwrap_or_default();
        let hex = tail.get(..64).unwrap_or_default();
        if is_hex64(hex) && !pinned.iter().any(|(_, p)| p.ends_with(hex)) {
            out.push(format!("sha256:{hex}"));
        }
        rest = tail;
    }
    out
}

/// The images the set-up files name that the list does not pin.
pub fn unpinned(
    images: &[(String, String)],
    used: &[(String, String)],
) -> Vec<Finding> {
    let mut out = Vec::new();
    for (path, text) in used {
        let mut say = |why: String| out.push(Finding::new(CHECK, path, 0, why));
        match named(path, text) {
            Ok(named) => {
                named
                    .iter()
                    .filter_map(|i| problem(i, images))
                    .for_each(&mut say);
            }
            Err(why) => say(why),
        }
        for digest in loose_digests(text, images) {
            say(format!(
                "the image {digest} is not pinned in gate/tools.toml, so it \
                 is never scanned"
            ));
        }
    }
    out
}
