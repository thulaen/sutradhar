//! Validate the report fields used to decide which image holes block.

use serde_json::{Map, Value};

type Object = Map<String, Value>;

fn object(value: &Value) -> Result<&Object, String> {
    value
        .as_object()
        .ok_or_else(|| "malformed vulnerability entry".into())
}

fn list<'a>(value: &'a Object, key: &str) -> Result<&'a [Value], String> {
    match value.get(key) {
        None => Ok(&[]),
        Some(Value::Array(items)) => Ok(items),
        Some(_) => Err("malformed vulnerability list".into()),
    }
}

fn text_field(value: &Object, key: &str) -> Result<(), String> {
    match value.get(key) {
        None | Some(Value::String(_)) => Ok(()),
        Some(_) => Err("malformed vulnerability text".into()),
    }
}

fn validate(report: &Value) -> Result<(), String> {
    for entry in list(object(report)?, "vulnerabilities")? {
        let entry = object(entry)?;
        text_field(entry, "id")?;
        for (collection, field) in [("ratings", "severity"), ("affects", "ref")]
        {
            for item in list(entry, collection)? {
                text_field(object(item)?, field)?;
            }
        }
    }
    Ok(())
}

pub(super) fn holes(report: &str) -> Result<Vec<String>, String> {
    let report: Value = serde_json::from_str(report)
        .map_err(|e| format!("the scanner's report cannot be read: {e}"))?;
    if report.get("bomFormat").and_then(Value::as_str) != Some("CycloneDX") {
        return Err("the scanner's report is not a CycloneDX report".into());
    }
    validate(&report)?;
    let text = |v: Option<&Value>| {
        v.and_then(Value::as_str).unwrap_or_default().to_owned()
    };
    let mut out = Vec::new();
    for hole in list(object(&report)?, "vulnerabilities")? {
        let ratings = hole.get("ratings").and_then(Value::as_array);
        let serious = ratings.into_iter().flatten().find_map(|r| {
            let level = text(r.get("severity")).to_ascii_lowercase();
            matches!(level.as_str(), "high" | "critical").then_some(level)
        });
        let Some(level) = serious else { continue };
        let id = text(hole.get("id"));
        let parts = hole.get("affects").and_then(Value::as_array);
        let mut named: Vec<String> = parts
            .into_iter()
            .flatten()
            .map(|p| text(p.get("ref")))
            .map(|r| r.split('?').next().unwrap_or_default().to_owned())
            .collect();
        if named.is_empty() {
            named.push("a part the report does not name".to_owned());
        }
        out.extend(
            named.iter().map(|what| format!("{id} ({level}) in {what}")),
        );
    }
    out.sort();
    out.dedup();
    Ok(out)
}
