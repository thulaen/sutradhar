//! Reading the values of a compose file's settings (see `setup_facts`):
//! flags, limits, plain text, and whether a setting's value is written in
//! the file.

use yaml_rust2::Yaml;

pub(super) fn has(list: &Yaml, wanted: &str) -> bool {
    list.as_vec().is_some_and(|items| {
        items.iter().any(|i| {
            i.as_str().is_some_and(|s| {
                s.replace(' ', "").eq_ignore_ascii_case(wanted)
            })
        })
    })
}

pub(super) fn flag(value: &Yaml) -> bool {
    value.as_bool() == Some(true)
        || value.as_str().is_some_and(|s| {
            matches!(s.to_ascii_lowercase().as_str(), "true" | "yes" | "on")
        })
}

/// Whether a limit is set and is not zero.
pub(super) fn limit(value: &Yaml) -> bool {
    !value.is_badvalue() && !matches!(plain(value).trim(), "" | "0" | "-1")
}

/// Whether a setting's value is written in the file, rather than only
/// read from outside: `$NAME`, `${NAME}`, `${NAME:?why}` and `${NAME:-}`
/// (empty default), alone or side by side, write nothing; any other
/// text, or a default such as `${NAME:-x}`, writes `x` in the file.
pub(super) fn written(value: &str) -> bool {
    let mut rest = value;
    while !rest.is_empty() {
        let Some(after) = rest.strip_prefix('$') else {
            return true;
        };
        if let Some(inner) = after.strip_prefix('{') {
            let Some(close) = inner.find('}') else {
                return true;
            };
            let body = inner.get(..close).unwrap_or_default();
            let name_end = body
                .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                .unwrap_or(body.len());
            let tail = body.get(name_end..).unwrap_or_default();
            // `${N}`, `${N:?why}` and `${N:-}` (empty default) write
            // nothing; any other default writes its text in the file.
            let plain_read = tail.is_empty()
                || tail.starts_with(":?")
                || tail.starts_with('?')
                || tail == ":-"
                || tail == "-";
            if name_end == 0 || !plain_read {
                return true;
            }
            rest = inner.get(close.saturating_add(1)..).unwrap_or_default();
        } else {
            let end = after
                .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                .unwrap_or(after.len());
            if end == 0 {
                return true;
            }
            rest = after.get(end..).unwrap_or_default();
        }
    }
    false
}

pub(super) fn plain(value: &Yaml) -> String {
    match value {
        Yaml::String(s) => s.clone(),
        Yaml::Integer(n) => n.to_string(),
        Yaml::Real(r) => r.clone(),
        Yaml::Boolean(b) => b.to_string(),
        _ => String::new(),
    }
}
