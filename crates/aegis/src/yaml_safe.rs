//! Reading YAML that a push brings, safely. The text is first read as a
//! stream of events, with no tree built, and refused when it uses a tag
//! (such as `!!binary`, which can make a key mean something other than
//! its text), an anchor or an alias (which can make a small file grow
//! huge), nests deeper than [`DEEPEST`], or holds more than [`MOST`]
//! nodes. Only then is the tree built.

use yaml_rust2::parser::{MarkedEventReceiver, Parser};
use yaml_rust2::scanner::Marker;
use yaml_rust2::{Event, Yaml, YamlLoader};

/// The deepest nesting allowed.
pub const DEEPEST: usize = 64;

/// The most nodes allowed in one file.
pub const MOST: usize = 100_000;

/// Watches the events and notes the first reason to refuse the text.
#[derive(Default)]
struct Watch {
    depth: usize,
    nodes: usize,
    refused: Option<String>,
}

impl MarkedEventReceiver for Watch {
    fn on_event(&mut self, event: Event, mark: Marker) {
        if self.refused.is_some() {
            return;
        }
        let line = mark.line();
        let why = match event {
            Event::Alias(_) => Some("an alias"),
            Event::Scalar(_, _, anchor, tag) => {
                self.nodes = self.nodes.saturating_add(1);
                if tag.is_some() {
                    Some("a tag such as `!!str`")
                } else {
                    (anchor > 0).then_some("an anchor")
                }
            }
            Event::SequenceStart(anchor, tag)
            | Event::MappingStart(anchor, tag) => {
                self.nodes = self.nodes.saturating_add(1);
                self.depth = self.depth.saturating_add(1);
                if tag.is_some() {
                    Some("a tag such as `!!map`")
                } else if anchor > 0 {
                    Some("an anchor")
                } else {
                    (self.depth > DEEPEST).then_some("nesting this deep")
                }
            }
            Event::SequenceEnd | Event::MappingEnd => {
                self.depth = self.depth.saturating_sub(1);
                None
            }
            _ => None,
        };
        let why =
            why.or_else(|| (self.nodes > MOST).then_some("this many nodes"));
        if let Some(why) = why {
            self.refused = Some(format!("line {line}: {why} is not allowed"));
        }
    }
}

/// The documents in `text`.
///
/// # Errors
/// When the text is not YAML, or uses a tag, an anchor or an alias, or
/// nests too deep, or holds too many nodes.
pub fn load(text: &str) -> Result<Vec<Yaml>, String> {
    let mut watch = Watch::default();
    let mut parser = Parser::new_from_str(text);
    // One event at a time, stopping at the first refusal, so deep or
    // huge text is turned away before any tree is built.
    loop {
        let (event, mark) = parser
            .next_token()
            .map_err(|e| format!("this is not valid YAML: {e}"))?;
        let end = event == Event::StreamEnd;
        watch.on_event(event, mark);
        if let Some(why) = watch.refused {
            return Err(why);
        }
        if end {
            break;
        }
    }
    YamlLoader::load_from_str(text)
        .map_err(|e| format!("this is not valid YAML: {e}"))
}

#[cfg(test)]
mod tests {
    use super::load;

    #[test]
    fn plain_yaml_is_read_and_tricks_are_refused() {
        assert_eq!(load("a: 1\nb: [x, y]\n").unwrap_or_default().len(), 1);
        for bad in [
            "s:\n  !!binary aW1hZ2U=: alpine\n",
            "a: &x [1]\nb: *x\n",
            "a: !!str 1\n",
            "a: [\n",
        ] {
            assert!(load(bad).is_err(), "{bad}");
        }
        let too_deep = format!("{}1{}", "[".repeat(65), "]".repeat(65));
        for one in ["a: &x 1\n", "a: *x\n", &too_deep] {
            assert!(load(one).is_err(), "{one}");
        }
        let fine = format!("{}1{}", "[".repeat(64), "]".repeat(64));
        assert!(load(&fine).is_ok());
        let deep = "- ".repeat(100_000);
        assert!(load(&deep).is_err());
        let many = "- 1\n".repeat(100_001);
        assert!(load(&many).is_err());
    }
}
