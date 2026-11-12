//! Exact runner digest deltas and deletion-only image exception changes.

use std::collections::BTreeSet;
use yaml_rust2::Yaml;

use super::{DIGEST_SEPARATOR, Data, digest, image_bytes, unique_keys};
use crate::images::allowed;
use crate::tools_list::is_hex64;

pub(super) fn runner(text: &str) -> Option<Data> {
    let mut docs = crate::yaml_safe::load(text).ok()?;
    if docs.len() != 1 || !unique_keys(text) {
        return None;
    }
    let mut doc = docs.pop()?;
    let labels = doc
        .as_mut_hash()?
        .get_mut(&Yaml::String("runner".into()))?
        .as_mut_hash()?
        .get_mut(&Yaml::String("labels".into()))?
        .as_mut_vec()?;
    let mut images = BTreeSet::new();
    let mut hashes = Vec::new();
    for label in labels {
        let value = label.as_str()?.to_owned();
        let Some((name, image)) = value.split_once(":docker://") else {
            continue;
        };
        let (reference, hex) = image.rsplit_once(DIGEST_SEPARATOR)?;
        if name.is_empty() || reference.is_empty() || !is_hex64(hex) {
            return None;
        }
        hashes.push(hex.to_owned());
        let _new = images.insert(value);
        *label = Yaml::String(String::new());
    }
    if images.is_empty() || !digest::unique(text, &hashes) {
        return None;
    }
    Some(Data::Compose(doc, image_bytes(text, &images)?))
}

/// Each surviving record keeps its original bytes and position. Removing
/// an exception tightens the scan; the scan still proves the image is fixed.
pub(super) fn removed_holes(before: &str, after: &str) -> bool {
    let Some((old_prefix, old)) = hole_blocks(before) else {
        return false;
    };
    let Some((new_prefix, new)) = hole_blocks(after) else {
        return false;
    };
    if old_prefix != new_prefix || new.len() >= old.len() {
        return false;
    }
    let old_count = old.len();
    let new_count = new.len();
    let mut remaining = old.into_iter().enumerate();
    new.into_iter().enumerate().all(|(index, block)| {
        remaining.any(|(old_index, old)| {
            old == block
                || (index.checked_add(1) == Some(new_count)
                    && old_index.checked_add(1).is_some_and(|n| n < old_count)
                    && old.strip_suffix('\n') == Some(block)
                    && block.ends_with('\n'))
        })
    })
}

fn hole_blocks(text: &str) -> Option<(&str, Vec<&str>)> {
    let pinned: Vec<String> = allowed::KNOWN
        .iter()
        .map(|(name, _)| (*name).to_owned())
        .collect();
    let day = allowed::day_of(allowed::LAST_DAY)?;
    let entries = allowed::read(Some(text), day, &pinned).ok()?;
    let mut parts = text.split("[[hole]]\n");
    let prefix = parts.next()?;
    if !prefix.lines().all(|line| {
        line.trim().is_empty() || line.trim_start().starts_with('#')
    }) {
        return None;
    }
    let blocks: Vec<_> = parts.collect();
    if blocks.len() != entries.len() {
        return None;
    }
    for block in &blocks {
        let record = format!("[[hole]]\n{block}");
        if allowed::read(Some(&record), day, &pinned).ok()?.len() != 1 {
            return None;
        }
    }
    Some((prefix, blocks))
}
