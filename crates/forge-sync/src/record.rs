//! The sync's own record: what it has copied, brought back and already
//! reported. GitHub's lists can lag behind a change, so the record is
//! what stops a second copy.

use serde_json::{Value, json};

/// What the sync remembers between runs.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Record {
    /// Forgejo tasks copied to GitHub.
    pub copied: Vec<u64>,
    /// GitHub issues brought into Forgejo.
    pub imported: Vec<u64>,
    /// GitHub comments brought into Forgejo.
    pub comments: Vec<u64>,
    /// Problems already reported in the urgent task.
    pub reported: Vec<String>,
}

fn numbers(v: &Value, key: &str) -> Vec<u64> {
    v.get(key)
        .and_then(Value::as_array)
        .map(|a| a.iter().filter_map(Value::as_u64).collect())
        .unwrap_or_default()
}

impl Record {
    /// Reads the record. A missing record is empty.
    ///
    /// # Errors
    /// When the file exists but is not a record.
    pub fn load(path: &str) -> Result<Self, String> {
        let text = match std::fs::read_to_string(path) {
            Ok(text) => text,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Self::default());
            }
            Err(e) => {
                return Err(format!("cannot read the record {path}: {e}"));
            }
        };
        let v: Value = serde_json::from_str(&text)
            .map_err(|e| format!("the record {path} is damaged: {e}"))?;
        Ok(Self {
            copied: numbers(&v, "copied"),
            imported: numbers(&v, "imported"),
            comments: numbers(&v, "comments"),
            reported: v
                .get("reported")
                .and_then(Value::as_array)
                .map(|a| {
                    a.iter()
                        .filter_map(Value::as_str)
                        .map(str::to_owned)
                        .collect()
                })
                .unwrap_or_default(),
        })
    }

    /// Writes the record, replacing the old one in one step.
    ///
    /// # Errors
    /// When the file cannot be written.
    pub fn save(&self, path: &str) -> Result<(), String> {
        let text = json!({
            "copied": self.copied,
            "imported": self.imported,
            "comments": self.comments,
            "reported": self.reported,
        })
        .to_string();
        let part = format!("{path}.part");
        std::fs::write(&part, text)
            .and_then(|()| std::fs::rename(&part, path))
            .map_err(|e| format!("cannot write the record {path}: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::Record;

    fn path(name: &str) -> String {
        std::env::temp_dir()
            .join(format!("forge-sync-rec-{name}-{}", std::process::id()))
            .display()
            .to_string()
    }

    #[test]
    fn saves_and_loads() {
        let p = path("round");
        let record = Record {
            copied: vec![1, 2],
            imported: vec![8],
            comments: vec![99],
            reported: vec!["x".to_owned()],
        };
        record.save(&p).unwrap();
        assert_eq!(Record::load(&p).unwrap(), record);
    }

    #[test]
    fn a_missing_record_is_empty_and_a_damaged_one_is_an_error() {
        assert_eq!(Record::load("/no/such/record").unwrap(), Record::default());
        let p = path("bad");
        std::fs::write(&p, "nope").unwrap();
        assert!(Record::load(&p).is_err());
        assert!(Record::default().save("/no/such/folder/r").is_err());
    }
}
