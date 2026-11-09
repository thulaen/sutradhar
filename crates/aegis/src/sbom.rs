//! The list of every part we build with and ship (task #113): a
//! `CycloneDX` 1.6 SBOM, written by our own code from `Cargo.lock`
//! (every Rust library, with its checksum and what it depends on) and
//! `gate/tools.toml` (every pinned tool and container image, with its
//! hash). The list is kept in the repository at [`PATH`]; the gate
//! refuses a change when it is not exactly what `aegis sbom` would
//! write from the same files, so it can never go out of date. It holds
//! no time or random number, so the same files always give the same
//! list.

use std::collections::BTreeMap;

use serde::Deserialize;
use serde_json::{Map, Value, json};

use crate::Finding;

/// Where the list lives.
pub const PATH: &str = "gate/sbom.cdx.json";

const CHECK: &str = "supply.sbom";

const CRATES_IO: &str = "registry+https://github.com/rust-lang/crates.io-index";

#[derive(Deserialize)]
struct Lock {
    package: Vec<Locked>,
}

#[derive(Deserialize)]
struct Locked {
    name: String,
    version: String,
    source: Option<String>,
    checksum: Option<String>,
    #[serde(default)]
    dependencies: Vec<String>,
}

#[derive(Deserialize)]
struct Tools {
    tool: Vec<Tool>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Tool {
    name: String,
    version: String,
    source: String,
    sha256: Option<String>,
    digest: Option<String>,
    note: Option<String>,
}

fn crate_ref(name: &str, version: &str) -> String {
    format!("cargo:{name}@{version}")
}

fn sha256(hex: &str) -> Value {
    json!([{ "alg": "SHA-256", "content": hex }])
}

/// Text made safe for a purl: every byte but a letter, digit or `.-_~`
/// is written as `%XX`, so `1.0+x` becomes `1.0%2Bx`.
fn safe(text: &str) -> String {
    text.bytes()
        .map(|b| {
            if b.is_ascii_alphanumeric() || b".-_~".contains(&b) {
                char::from(b).to_string()
            } else {
                format!("%{b:02X}")
            }
        })
        .collect()
}

/// A component with these fields, and the licence when there is one.
fn component(fields: Value, license: Option<&str>) -> Value {
    let mut c = match fields {
        Value::Object(m) => m,
        _ => Map::new(),
    };
    if let Some(id) = license {
        let _old = c.insert("licenses".into(), json!([{ "expression": id }]));
    }
    Value::Object(c)
}

/// The library a `Cargo.lock` dependency line names: `name`, or `name
/// version`, or `name version (source)`.
fn resolve(line: &str, all: &[Locked]) -> Result<String, String> {
    let mut parts = line.split_whitespace();
    let name = parts.next().unwrap_or_default();
    let version = parts.next();
    let mut found = all
        .iter()
        .filter(|p| p.name == name && version.is_none_or(|v| p.version == v));
    match (found.next(), found.next()) {
        (Some(p), None) => Ok(crate_ref(&p.name, &p.version)),
        _ => Err(format!("Cargo.lock names `{line}`, which it does not hold")),
    }
}

/// One library from `Cargo.lock`, as a component.
fn library(p: &Locked, license: Option<&str>) -> Result<Value, String> {
    let fields = json!({
        "type": "library",
        "bom-ref": crate_ref(&p.name, &p.version),
        "name": p.name,
        "version": p.version,
    });
    match (p.source.as_deref(), p.checksum.as_deref()) {
        // Our own crates: built here, under the project's licence.
        (None, _) => Ok(component(fields, license)),
        (Some(CRATES_IO), Some(sum)) => {
            let mut c = component(fields, None);
            if let Value::Object(m) = &mut c {
                let purl =
                    format!("pkg:cargo/{}@{}", safe(&p.name), safe(&p.version));
                let _p = m.insert("purl".into(), json!(purl));
                let _h = m.insert("hashes".into(), sha256(sum));
            }
            Ok(c)
        }
        (Some(other), _) => Err(format!(
            "`{}` comes from `{other}` or has no checksum",
            p.name
        )),
    }
}

/// One pinned tool, as a component: a container image when it is
/// pinned by digest, a program when it is pinned by file hash.
fn tool(t: &Tool) -> Result<(String, Value), String> {
    let (kind, hex) = match (t.digest.as_deref(), t.sha256.as_deref()) {
        (Some(d), None) => ("container", d.strip_prefix("sha256:")),
        (None, Some(h)) => ("application", Some(h)),
        _ => return Err(format!("tool `{}` needs one hash", t.name)),
    };
    let hex = hex
        .filter(|h| h.len() == 64 && h.bytes().all(|b| b.is_ascii_hexdigit()))
        .ok_or_else(|| {
            format!("tool `{}` needs one SHA-256 hash or digest", t.name)
        })?;
    let what = t
        .note
        .as_ref()
        .map_or_else(|| t.source.clone(), |n| format!("{}; {n}", t.source));
    let id = format!("tool:{}@{}", t.name, t.version);
    Ok((
        id.clone(),
        json!({
            "type": kind,
            "bom-ref": id,
            "name": t.name,
            "version": t.version,
            "description": what,
            "hashes": sha256(hex),
        }),
    ))
}

/// The parts and what each needs, by bom-ref, with our own parts
/// listed under `ours`.
#[derive(Default)]
struct Parts {
    each: BTreeMap<String, Value>,
    needs: BTreeMap<String, Vec<String>>,
    ours: Vec<String>,
}

impl Parts {
    fn add(&mut self, id: String, part: Value, on: Vec<String>) -> bool {
        let _old = self.needs.insert(id.clone(), on);
        self.each.insert(id, part).is_none()
    }
}

fn gather(
    lock: &Lock,
    tools: &Tools,
    license: Option<&str>,
) -> Result<Parts, String> {
    let mut all = Parts::default();
    for p in &lock.package {
        let id = crate_ref(&p.name, &p.version);
        if p.source.is_none() {
            all.ours.push(id.clone());
        }
        let mut on = p
            .dependencies
            .iter()
            .map(|d| resolve(d, &lock.package))
            .collect::<Result<Vec<_>, _>>()?;
        on.sort();
        if !all.add(id, library(p, license)?, on) {
            return Err(format!("Cargo.lock holds `{}` twice", p.name));
        }
    }
    for t in &tools.tool {
        let (id, c) = tool(t)?;
        all.ours.push(id.clone());
        if !all.add(id, c, Vec::new()) {
            return Err(format!("gate/tools.toml holds `{}` twice", t.name));
        }
    }
    all.ours.sort();
    Ok(all)
}

/// The list for these files, as the exact text kept at [`PATH`].
///
/// # Errors
/// When the text cannot be read, or names a part it does not hold.
pub fn build(lock: &str, tools: &str, root: &str) -> Result<String, String> {
    let lock: Lock = toml::from_str(lock)
        .map_err(|e| format!("Cargo.lock cannot be read: {}", e.message()))?;
    let tools: Tools = toml::from_str(tools).map_err(|e| {
        format!("gate/tools.toml cannot be read: {}", e.message())
    })?;
    let root: toml::Table = toml::from_str(root)
        .map_err(|e| format!("Cargo.toml cannot be read: {}", e.message()))?;
    let license = root
        .get("workspace")
        .and_then(|w| w.get("package"))
        .and_then(|p| p.get("license"))
        .and_then(toml::Value::as_str)
        .filter(|id| !id.trim().is_empty())
        .map(str::to_owned)
        .ok_or("Cargo.toml must name the licence in [workspace.package]")?;
    let mut all = gather(&lock, &tools, Some(&license))?;
    let _top = all.needs.insert("sutradhar".to_owned(), all.ours);
    let dependencies: Vec<Value> = all
        .needs
        .into_iter()
        .map(|(id, on)| json!({ "ref": id, "dependsOn": on }))
        .collect();
    let top = json!({
        "type": "application",
        "bom-ref": "sutradhar",
        "name": "sutradhar",
    });
    let bom = json!({
        "$schema": "http://cyclonedx.org/schema/bom-1.6.schema.json",
        "bomFormat": "CycloneDX",
        "specVersion": "1.6",
        "version": 1,
        "metadata": { "component": component(top, Some(&license)) },
        "components": all.each.into_values().collect::<Vec<_>>(),
        "dependencies": dependencies,
    });
    serde_json::to_string_pretty(&bom)
        .map(|text| format!("{text}\n"))
        .map_err(|e| format!("the list cannot be written: {e}"))
}

/// Checks that the kept list is exactly what [`build`] gives. A
/// repository with no `Cargo.lock` needs none.
#[must_use]
pub fn check(
    lock: Option<&str>,
    tools: Option<&str>,
    root: Option<&str>,
    kept: Option<&str>,
) -> Vec<Finding> {
    let Some(lock) = lock else {
        return Vec::new();
    };
    let problem = |why: String| vec![Finding::new(CHECK, PATH, 0, why)];
    let (Some(tools), Some(root)) = (tools, root) else {
        let gone = if tools.is_none() {
            "gate/tools.toml"
        } else {
            "Cargo.toml"
        };
        return problem(format!("{gone} is needed to check the SBOM"));
    };
    match build(lock, tools, root) {
        Err(why) => problem(why),
        Ok(want) if kept == Some(want.as_str()) => Vec::new(),
        Ok(_) if kept.is_none() => {
            problem("there is no SBOM; run `aegis sbom` to write it".into())
        }
        Ok(_) => problem(
            "the SBOM is out of date; run `aegis sbom` to write it again"
                .into(),
        ),
    }
}

#[cfg(test)]
#[path = "sbom_tests.rs"]
mod tests;
