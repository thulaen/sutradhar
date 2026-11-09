//! Facts about the code, kept by relation, as the rules' input. Each fact
//! is a JSON object with named fields, as listed in
//! docs/specs/0061-nirikshak.md. A path is relative to the analysed root,
//! with `/` between folders; lines start at 1.

use std::collections::BTreeMap;

use serde_json::{Map, Value};

/// Every fact found so far, by relation.
#[derive(Debug, Default)]
pub struct Facts {
    by_relation: BTreeMap<&'static str, Vec<Value>>,
}

impl Facts {
    /// Adds one fact to `relation`, from (field, value) pairs.
    pub fn add(&mut self, relation: &'static str, fields: &[(&str, Value)]) {
        let object: Map<String, Value> = fields
            .iter()
            .map(|(k, v)| ((*k).to_owned(), v.clone()))
            .collect();
        self.by_relation
            .entry(relation)
            .or_default()
            .push(Value::Object(object));
    }

    /// The facts of one relation.
    #[must_use]
    pub fn of(&self, relation: &str) -> &[Value] {
        self.by_relation.get(relation).map_or(&[], Vec::as_slice)
    }

    /// All the facts as one JSON object, a list per relation. Every
    /// relation the rules may read is present, empty when nothing was
    /// found, so a rule never reads a missing list as a pass.
    #[must_use]
    pub fn to_json(&self) -> Value {
        let mut all = Map::new();
        for relation in RELATIONS {
            let _old = all.insert(
                relation.to_owned(),
                Value::Array(self.of(relation).to_vec()),
            );
        }
        Value::Object(all)
    }
}

/// The relations, as listed in the specification.
pub const RELATIONS: [&str; 25] = [
    "file",
    "function",
    "call",
    "call_text",
    "macro",
    "macro_text",
    "doc_code",
    "env_declared",
    "container",
    "container_env",
    "docker_stage",
    "literal",
    "method_receiver",
    "range_cut",
    "impl_fn",
    "discard",
    "empty_handler",
    "uses_try",
    "parse_failure",
    "signature",
    "number",
    "chain",
    "shape",
    "comment",
    "repeated",
];

/// A text value.
#[must_use]
pub fn text(s: impl Into<String>) -> Value {
    Value::String(s.into())
}

/// A whole-number value, such as a line.
#[must_use]
pub fn number(n: usize) -> Value {
    Value::from(n)
}
