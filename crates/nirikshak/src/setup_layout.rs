//! Where things are in a compose file: the `services:` line, each
//! service's own block of lines and its settings, worked out in one pass
//! (see `setup_facts`).

/// Where things are in a compose file, worked out in one pass: the
/// `services:` line, and each service's own block of lines (by its
/// indent), so a name used inside another service is not mistaken for
/// it, and a search never runs over the whole file again.
pub(super) struct Layout {
    top: usize,
    blocks: std::collections::HashMap<String, (usize, usize)>,
    /// Each service's settings, from its own `environment:` block only,
    /// with their lines: worked out once, not searched for each setting.
    settings: std::collections::HashMap<(String, String), usize>,
}

fn key_name(line: &str) -> Option<String> {
    let (key, _) = line.trim_start().split_once(':')?;
    let key = key.trim().trim_matches(['"', '\'']);
    (!key.is_empty() && !key.starts_with('-')).then(|| key.to_owned())
}

fn indent(line: &str) -> usize {
    line.len().saturating_sub(line.trim_start().len())
}

/// The name a line of an `environment:` block sets: `NAME: value` or
/// `- NAME=value`, quoted or not.
fn setting_name(line: &str) -> Option<String> {
    let t = line.trim_start();
    match t.strip_prefix("- ") {
        Some(item) => {
            let item = item.trim_start_matches(['"', '\'']);
            let name = item.split('=').next().unwrap_or(item);
            (!name.is_empty()).then(|| name.to_owned())
        }
        None => key_name(t),
    }
}

/// The settings of the service whose block is `lines[start..end]`, with
/// their lines (from 1), read from its `environment:` block only.
fn settings_of(
    lines: &[&str],
    start: usize,
    end: usize,
) -> Vec<(String, usize)> {
    let mut out = Vec::new();
    let mut env: Option<usize> = None;
    for (i, line) in lines
        .iter()
        .enumerate()
        .take(end)
        .skip(start.saturating_add(1))
    {
        let t = line.trim_start();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let here = indent(line);
        match env {
            Some(level) if here > level => {
                out.extend(
                    setting_name(line).map(|n| (n, i.saturating_add(1))),
                );
            }
            _ => {
                env = (key_name(line).as_deref() == Some("environment"))
                    .then_some(here);
            }
        }
    }
    out
}

impl Layout {
    pub(super) fn new(source: &str) -> Self {
        let lines: Vec<&str> = source.lines().collect();
        let top = lines.iter().position(|l| is_services(l)).unwrap_or(0);
        let mut blocks = std::collections::HashMap::new();
        let mut open: Option<(String, usize)> = None;
        let mut level = None;
        for (i, line) in lines.iter().enumerate().skip(top.saturating_add(1)) {
            let t = line.trim_start();
            if t.is_empty() || t.starts_with('#') {
                continue;
            }
            let here = indent(line);
            if here == 0 {
                break;
            }
            if *level.get_or_insert(here) != here {
                continue;
            }
            if let Some((name, start)) = open.take() {
                let _old = blocks.insert(name, (start, i));
            }
            open = key_name(line).map(|n| (n, i));
        }
        if let Some((name, start)) = open {
            let _old = blocks.insert(name, (start, lines.len()));
        }
        let mut settings = std::collections::HashMap::new();
        for (name, (start, end)) in &blocks {
            for (key, line) in settings_of(&lines, *start, *end) {
                let _first =
                    settings.entry((name.clone(), key)).or_insert(line);
            }
        }
        Self {
            top,
            blocks,
            settings,
        }
    }

    /// The line (from 1) of a service.
    pub(super) fn service(&self, name: &str) -> usize {
        self.blocks
            .get(name)
            .map_or(self.top, |(s, _)| *s)
            .saturating_add(1)
    }

    /// The line (from 1) of a setting in a service's `environment:`
    /// block, or the service's own line when it is not found there.
    pub(super) fn setting(&self, service: &str, key: &str) -> usize {
        self.settings
            .get(&(service.to_owned(), key.to_owned()))
            .copied()
            .unwrap_or_else(|| self.service(service))
    }
}

/// Whether a line opens the top-level `services` map, however it is
/// quoted, and after a byte-order mark.
pub(super) fn is_services(line: &str) -> bool {
    let line = line.trim_start_matches('\u{feff}');
    indent(line) == 0 && key_name(line).as_deref() == Some("services")
}
