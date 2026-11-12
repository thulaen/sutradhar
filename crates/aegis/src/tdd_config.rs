//! A narrow proof of data-only image digest changes for the TDD check.

use serde_json::Value;
use std::collections::BTreeSet;
use yaml_rust2::{Event, Yaml, parser::Parser};

use crate::file_kinds::{Kind, TypeScript, kind_of};
use crate::tools_list::is_hex64;

#[path = "tdd_digest.rs"]
mod digest;

#[path = "tdd_image_data.rs"]
mod image_data;

const DIGEST_SEPARATOR: &str = "@sha256:";

#[derive(PartialEq)]
enum Data {
    Tools(toml::Value, String),
    Sbom(Value, String),
    Compose(Yaml, String),
}

pub(super) fn data_only(
    path: &str,
    read: &impl Fn(&str) -> Option<(String, String)>,
) -> bool {
    if kind_of(path, TypeScript::Off) != Some(Kind::Config) {
        return false;
    }
    if path == crate::images::allowed::PATH {
        return read(path).is_some_and(|(before, after)| {
            image_data::removed_holes(&before, &after)
        });
    }
    let normalise: fn(&str) -> Option<Data> = match path {
        crate::tools_list::PATH => tools,
        crate::sbom::PATH => sbom,
        "deploy/runner/config.yml" => image_data::runner,
        _ if (path.starts_with("docker/") || path.starts_with("deploy/"))
            && (path.ends_with("/compose.yaml")
                || path.ends_with("/compose.yml")) =>
        {
            compose
        }
        _ => return false,
    };
    read(path).is_some_and(|(before, after)| {
        before != after
            && normalise(&before)
                .zip(normalise(&after))
                .is_some_and(|(a, b)| a == b)
    })
}

fn tools(text: &str) -> Option<Data> {
    let images = crate::tools_list::images(text).ok()?;
    let hashes: Vec<String> = images
        .iter()
        .filter_map(|(_, image)| image.rsplit_once(DIGEST_SEPARATOR))
        .map(|(_, hex)| hex.to_owned())
        .collect();
    let mut doc: toml::Value = toml::from_str(text).ok()?;
    for tool in doc.get_mut("tool")?.as_array_mut()? {
        if let Some(digest) = tool.get_mut("digest") {
            *digest = toml::Value::String(String::new());
        }
    }
    Some(Data::Tools(doc, digest_bytes(text, &hashes)?))
}

fn sbom(text: &str) -> Option<Data> {
    let mut doc: Value = serde_json::from_str(text).ok()?;
    if text != format!("{}\n", serde_json::to_string_pretty(&doc).ok()?) {
        return None;
    }
    if doc.get("bomFormat")?.as_str()? != "CycloneDX" {
        return None;
    }
    let mut sums = Vec::new();
    for component in doc.get_mut("components")?.as_array_mut()? {
        if component.get("type").and_then(Value::as_str) != Some("container") {
            continue;
        }
        let Some(hashes) = component.get_mut("hashes") else {
            continue;
        };
        for hash in hashes.as_array_mut()? {
            if hash.get("alg")?.as_str()? != "SHA-256" {
                continue;
            }
            let content = hash.get_mut("content")?;
            if !is_hex64(content.as_str()?) {
                return None;
            }
            sums.push(content.as_str()?.to_owned());
            *content = Value::String(String::new());
        }
    }
    Some(Data::Sbom(doc, digest_bytes(text, &sums)?))
}

fn compose(text: &str) -> Option<Data> {
    let mut docs = crate::yaml_safe::load(text).ok()?;
    if docs.len() != 1 || !unique_keys(text) {
        return None;
    }
    let mut doc = docs.pop()?;
    let services = doc
        .as_mut_hash()?
        .get_mut(&Yaml::String("services".into()))?
        .as_mut_hash()?;
    let mut images = BTreeSet::new();
    let mut hashes = Vec::new();
    for service in services.values_mut() {
        let map = service.as_mut_hash()?;
        if let Some(image) = map.get_mut(&Yaml::String("image".into())) {
            let value = image.as_str()?.to_owned();
            let (_, hex) = value.rsplit_once(DIGEST_SEPARATOR)?;
            if !is_hex64(hex) {
                return None;
            }
            hashes.push(hex.to_owned());
            let _new = images.insert(value);
            *image = Yaml::String(String::new());
        }
    }
    if images.is_empty() || !digest::unique(text, &hashes) {
        return None;
    }
    Some(Data::Compose(doc, image_bytes(text, &images)?))
}

fn digest_bytes(text: &str, hashes: &[String]) -> Option<String> {
    digest::mask(text, hashes)
}

enum Level {
    Mapping(BTreeSet<String>, bool),
    Sequence,
}

/// Compose keys must be unique scalar names. Complex keys or a duplicate
/// give no proof, even when a YAML reader would silently keep one value.
fn unique_keys(text: &str) -> bool {
    let mut parser = Parser::new_from_str(text);
    let mut levels = Vec::new();
    loop {
        let Ok((event, _)) = parser.next_token() else {
            return false;
        };
        match event {
            Event::Scalar(name, _, _, _) => {
                if !scalar_key(&mut levels, name) {
                    return false;
                }
            }
            Event::MappingStart(_, _) | Event::SequenceStart(_, _) => {
                if matches!(levels.last(), Some(Level::Mapping(_, true))) {
                    return false;
                }
                levels.push(if matches!(event, Event::MappingStart(_, _)) {
                    Level::Mapping(BTreeSet::new(), true)
                } else {
                    Level::Sequence
                });
            }
            Event::MappingEnd | Event::SequenceEnd => {
                let _ended = levels.pop();
                if let Some(Level::Mapping(_, key)) = levels.last_mut() {
                    *key = true;
                }
            }
            Event::StreamEnd => return levels.is_empty(),
            _ => {}
        }
    }
}

fn scalar_key(levels: &mut [Level], name: String) -> bool {
    let Some(Level::Mapping(keys, key)) = levels.last_mut() else {
        return true;
    };
    let unique = !*key || keys.insert(name);
    *key = !*key;
    unique
}

/// Only scalar spans holding service image values may lose their hash
/// bytes. The remaining bytes must match exactly, and the parsed tree
/// above separately preserves every non-image value.
fn image_bytes(text: &str, images: &BTreeSet<String>) -> Option<String> {
    let mut parser = Parser::new_from_str(text);
    // This pinned parser counts Unicode scalar values in Marker.index().
    // ASCII needs no map; other input gets one bounded UTF-8 offset map.
    let offsets = (!text.is_ascii()).then(|| {
        text.char_indices()
            .map(|(byte, _)| byte)
            .chain(std::iter::once(text.len()))
            .collect::<Vec<_>>()
    });
    let mut pending: Option<(usize, String)> = None;
    let mut output = String::new();
    let mut kept = 0;
    loop {
        let (event, mark) = parser.next_token().ok()?;
        let byte = offsets
            .as_ref()
            .map_or(Some(mark.index()), |map| map.get(mark.index()).copied())?;
        if let Some((start, hex)) = pending.take() {
            let scalar = text.get(start..byte)?;
            if scalar.matches(&hex).count() != 1 {
                return None;
            }
            let at = start.checked_add(scalar.find(&hex)?)?;
            output.push_str(text.get(kept..at)?);
            kept = at.checked_add(hex.len())?;
        }
        if event == Event::StreamEnd {
            break;
        }
        if let Event::Scalar(value, _, _, _) = event
            && images.contains(&value)
        {
            let (_, hex) = value.rsplit_once(DIGEST_SEPARATOR)?;
            pending = Some((byte, hex.to_owned()));
        }
    }
    output.push_str(text.get(kept..)?);
    Some(output)
}

#[cfg(test)]
mod tests {
    use super::data_only;

    fn configuration(kind: &str, hex: &str) -> String {
        let mut component = serde_json::json!({"hashes": [{
            "alg": "SHA-256", "content": hex,
        }]});
        if !kind.is_empty() {
            component["type"] = kind.into();
        }
        let doc = serde_json::json!({
            "bomFormat": "CycloneDX", "components": [component],
        });
        format!("{}\n", serde_json::to_string_pretty(&doc).unwrap())
    }

    #[test]
    fn non_container_sbom_hashes_still_need_a_named_test() {
        for kind in ["container", "library", "application", ""] {
            let before = configuration(kind, &"a".repeat(64));
            let after = configuration(kind, &"b".repeat(64));
            assert_eq!(
                data_only("gate/sbom.cdx.json", &|_| Some((
                    before.clone(),
                    after.clone()
                ))),
                kind == "container",
                "component kind {kind}",
            );
        }
    }
}
