//! Read decision fields once, with unambiguous IDs and safe diagnostics.

use serde::{Deserialize, Deserializer, de::Error};

#[path = "images_report_object.rs"]
mod object;

use object::Object;

// Bound expansion as well as scanner output: IDs repeat for affected parts.
const DIAGNOSTIC_BYTES: usize = 1024 * 1024;
const DIAGNOSTIC_COUNT: usize = 4096;

/// One serious finding, with identity separate from displayed package text.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Hole {
    /// The complete vulnerability identifier used by the allowance policy.
    pub(super) id: String,
    /// The validated serious severity.
    pub(super) level: String,
    /// The affected reference without its query parameters.
    pub(super) part: String,
}

impl Hole {
    /// Format a diagnostic without recovering or changing the finding's ID.
    pub(super) fn message(self) -> String {
        format!("{} ({}) in {}", self.id, self.level, self.part)
    }
}

#[derive(Deserialize)]
struct Report {
    #[serde(rename = "bomFormat")]
    format: String,
    #[serde(default)]
    vulnerabilities: Vec<Object<Vulnerability>>,
}

#[derive(Deserialize)]
struct Vulnerability {
    #[serde(default, deserialize_with = "identifier")]
    id: String,
    #[serde(default)]
    ratings: Vec<Object<Rating>>,
    #[serde(default)]
    affects: Vec<Object<Affected>>,
}

#[derive(Deserialize)]
struct Rating {
    #[serde(default, deserialize_with = "severity")]
    severity: String,
}

#[derive(Deserialize)]
struct Affected {
    #[serde(rename = "ref", default, deserialize_with = "diagnostic_text")]
    reference: String,
}

fn diagnostic_text<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<String, D::Error> {
    let text = String::deserialize(deserializer)?;
    // Direction overrides and line separators can also disguise a diagnostic.
    if text.chars().any(|c| {
        c.is_control()
            || matches!(c, '\u{61c}' | '\u{200e}'..='\u{200f}'
                | '\u{2028}'..='\u{202e}' | '\u{2066}'..='\u{2069}')
    }) {
        return Err(D::Error::custom("control character in scanner text"));
    }
    Ok(text)
}

fn identifier<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<String, D::Error> {
    let text = diagnostic_text(deserializer)?;
    // Keep diagnostic IDs distinct from the surrounding display separators.
    if text
        .chars()
        .any(|c| c.is_whitespace() || matches!(c, '(' | ')'))
    {
        return Err(D::Error::custom("ambiguous vulnerability identifier"));
    }
    Ok(text)
}

fn severity<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<String, D::Error> {
    let text = String::deserialize(deserializer)?.to_ascii_lowercase();
    match text.as_str() {
        "none" | "info" | "low" | "medium" | "high" | "critical"
        | "unknown" => Ok(text),
        _ => Err(D::Error::custom("unrecognised vulnerability severity")),
    }
}

fn record(
    out: &mut Vec<Hole>,
    remaining: &mut usize,
    id: &str,
    level: &str,
    part: &str,
) -> Result<(), String> {
    let size = id
        .len()
        .checked_add(level.len())
        .and_then(|n| n.checked_add(part.len()))
        .and_then(|n| n.checked_add(" () in ".len()));
    let available = size.and_then(|n| remaining.checked_sub(n));
    if out.len() >= DIAGNOSTIC_COUNT || available.is_none() {
        return Err(
            "the scanner's findings exceed their diagnostic limit".into()
        );
    }
    *remaining = available.unwrap_or_default();
    out.push(Hole {
        id: id.to_owned(),
        level: level.to_owned(),
        part: part.to_owned(),
    });
    Ok(())
}

/// Read serious findings with strict decision fields and bounded diagnostics.
///
/// # Errors
/// Refuses malformed or ambiguous input, unsafe text and excessive output.
pub(super) fn findings(report: &str) -> Result<Vec<Hole>, String> {
    // Typed fields reject duplicate decision keys, including escaped spellings.
    // Unknown metadata is ignored rather than copied into a generic JSON tree.
    let Object(report): Object<Report> = serde_json::from_str(report)
        .map_err(|_| "the scanner's report cannot be read".to_owned())?;
    if report.format != "CycloneDX" {
        return Err("the scanner's report is not a CycloneDX report".into());
    }
    let mut out = Vec::new();
    let mut remaining = DIAGNOSTIC_BYTES;
    for Object(hole) in report.vulnerabilities {
        let level = hole.ratings.iter().find_map(|Object(r)| {
            matches!(r.severity.as_str(), "high" | "critical")
                .then_some(&r.severity)
        });
        let Some(level) = level else { continue };
        let named = hole
            .affects
            .iter()
            .map(|Object(p)| p.reference.split('?').next().unwrap_or_default())
            .chain(
                hole.affects
                    .is_empty()
                    .then_some("a part the report does not name"),
            );
        for part in named {
            record(&mut out, &mut remaining, &hole.id, level, part)?;
        }
    }
    out.sort();
    out.dedup();
    Ok(out)
}

#[cfg(test)]
#[path = "images_report_tests.rs"]
mod tests;
