//! Exact digest occurrences, without losing any surrounding bytes.

use std::collections::BTreeMap;

const HEX_DIGEST_LEN: usize = 64;

pub(super) fn mask(text: &str, hashes: &[String]) -> Option<String> {
    let positions = locate(text, hashes, || {})?;
    let mut masked = String::with_capacity(text.len());
    let mut kept = 0;
    for at in positions {
        masked.push_str(text.get(kept..at)?);
        masked.push_str("<image digest>");
        kept = at.checked_add(HEX_DIGEST_LEN)?;
    }
    masked.push_str(text.get(kept..)?);
    Some(masked)
}

pub(super) fn unique(text: &str, hashes: &[String]) -> bool {
    locate(text, hashes, || {}).is_some()
}

fn locate(
    text: &str,
    hashes: &[String],
    mut visited: impl FnMut(),
) -> Option<Vec<usize>> {
    let mut positions: BTreeMap<&[u8], Option<usize>> = BTreeMap::new();
    for hex in hashes {
        if hex.len() != HEX_DIGEST_LEN
            || positions.insert(hex.as_bytes(), None).is_some()
        {
            return None;
        }
    }
    for (at, window) in text
        .as_bytes()
        .windows(HEX_DIGEST_LEN)
        .enumerate()
        .inspect(|_| visited())
    {
        let Some(found) = positions.get_mut(window) else {
            continue;
        };
        // str::matches counts non-overlapping occurrences of each hash.
        if found.is_some_and(|last| at < last.saturating_add(HEX_DIGEST_LEN)) {
            continue;
        }
        if found.replace(at).is_some() {
            return None;
        }
    }
    let mut positions = positions.into_values().collect::<Option<Vec<_>>>()?;
    positions.sort_unstable();
    Some(positions)
}

#[cfg(test)]
mod tests {
    use super::{locate, mask};

    #[test]
    fn each_input_window_is_examined_at_most_once() {
        let hashes: Vec<_> = (0..128).map(|n| format!("{n:064x}")).collect();
        let text = hashes.join("\n");
        let mut visits = 0;
        assert!(locate(&text, &hashes, || visits += 1).is_some());
        assert!(
            visits <= text.len(),
            "{visits} windows for {} bytes",
            text.len()
        );
    }

    #[test]
    fn masking_keeps_the_original_substring_and_duplicate_rules() {
        let hash = "a".repeat(64);
        for text in [
            hash.clone(),
            format!("é{hash}尾"),
            "a".repeat(65),
            "a".repeat(127),
            "a".repeat(128),
            format!("{hash} {hash}"),
            "short".into(),
        ] {
            let expected = (text.matches(&hash).count() == 1)
                .then(|| text.replace(&hash, "<image digest>"));
            assert_eq!(mask(&text, std::slice::from_ref(&hash)), expected);
        }
        assert_eq!(mask(&hash, &[hash.clone(), hash.clone()]), None);
    }
}
