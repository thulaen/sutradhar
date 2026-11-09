//! Reads one Markdown page and records each inline code span, such as
//! `AEGIS_QUEUE`, as a `doc_code` fact. Fenced code blocks are skipped:
//! they show commands, not names the page explains.

use crate::facts::{Facts, number, text};

/// Adds the facts of one Markdown page at `path` to `facts`.
pub fn read(path: &str, source: &str, facts: &mut Facts) {
    facts.add(
        "file",
        &[("path", text(path)), ("language", text("markdown"))],
    );
    let mut fenced = false;
    for (index, line) in source.lines().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            fenced = !fenced;
            continue;
        }
        if fenced {
            continue;
        }
        let at = index.saturating_add(1);
        // Pieces between backticks; the last is not a span unless closed.
        let pieces: Vec<&str> = line.split('`').collect();
        let closed = pieces.len().saturating_sub(1);
        let spans = pieces.iter().take(closed).skip(1).step_by(2);
        for span in spans {
            if !span.trim().is_empty() {
                facts.add(
                    "doc_code",
                    &[
                        ("path", text(path)),
                        ("text", text(span.trim())),
                        ("line", number(at)),
                    ],
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::read;
    use crate::facts::Facts;

    #[test]
    fn code_spans_are_read_and_fenced_blocks_are_not() {
        let mut f = Facts::default();
        let page = "Set `AEGIS_QUEUE` and ` X `.\n```\n`HIDDEN`\n```\n\
                    Then `half";
        read("a.md", page, &mut f);
        let found: Vec<(String, u64)> = f
            .of("doc_code")
            .iter()
            .filter_map(|v| {
                Some((
                    v.get("text")?.as_str()?.to_owned(),
                    v.get("line")?.as_u64()?,
                ))
            })
            .collect();
        assert_eq!(found, [("AEGIS_QUEUE".to_owned(), 1), ("X".to_owned(), 1)]);
    }
}
