//! A small reader that splits Rust source into code words and symbols,
//! each with its line, dropping comments (line, block and nested) and
//! literals (strings, raw strings with any number of `#`, byte strings
//! and character literals). A lifetime such as `'a` stays a symbol and
//! a word. It is enough for checks that look for words in code; it is
//! not a parser.

/// One code word or symbol, and its line.
pub type Token = (String, usize);

/// Reads through `text` one character at a time, keeping the line.
struct Reader {
    chars: Vec<char>,
    i: usize,
    line: usize,
}

impl Reader {
    fn at(&self, n: usize) -> Option<char> {
        self.chars.get(self.i.saturating_add(n)).copied()
    }

    fn step(&mut self) {
        if self.at(0) == Some('\n') {
            self.line = self.line.saturating_add(1);
        }
        self.i = self.i.saturating_add(1);
    }

    /// Skips a quoted string; the reader is just after the opening quote.
    fn quoted(&mut self) {
        while let Some(c) = self.at(0) {
            self.step();
            match c {
                '\\' => self.step(),
                '"' => return,
                _ => {}
            }
        }
    }

    /// Skips a raw string; the reader is on its first `#` or `"`.
    fn raw(&mut self) {
        let mut hashes = 0_usize;
        while self.at(0) == Some('#') {
            hashes = hashes.saturating_add(1);
            self.step();
        }
        self.step();
        while self.at(0).is_some() && !self.raw_end(hashes) {
            self.step();
        }
        for _ in 0..=hashes {
            self.step();
        }
    }

    /// Whether a raw string with `hashes` closes here.
    fn raw_end(&self, hashes: usize) -> bool {
        self.at(0) == Some('"') && (1..=hashes).all(|n| self.at(n) == Some('#'))
    }

    /// Skips a comment; the reader is on its `/`.
    fn comment(&mut self) {
        if self.at(1) == Some('/') {
            while self.at(0).is_some_and(|c| c != '\n') {
                self.step();
            }
            return;
        }
        let mut depth = 0_usize;
        while let Some(c) = self.at(0) {
            let next = self.at(1);
            if c == '/' && next == Some('*') {
                depth = depth.saturating_add(1);
                self.step();
            } else if c == '*' && next == Some('/') {
                depth = depth.saturating_sub(1);
                self.step();
            }
            self.step();
            if depth == 0 {
                return;
            }
        }
    }

    /// Skips a character literal if one starts here, on its `'`.
    fn char_literal(&mut self) -> bool {
        let escaped = self.at(1) == Some('\\');
        let plain = !escaped && self.at(2) == Some('\'');
        if !escaped && !plain {
            return false;
        }
        self.step();
        if escaped {
            self.step();
        }
        self.step();
        while let Some(c) = self.at(0) {
            self.step();
            if c == '\'' {
                break;
            }
        }
        true
    }
}

impl Reader {
    /// Reads a word; a raw or byte string's prefix starts that literal.
    fn word(&mut self, out: &mut Vec<Token>) {
        let (start, line) = (self.i, self.line);
        while self.at(0).is_some_and(is_word) {
            self.step();
        }
        let word: String = self
            .chars
            .get(start..self.i)
            .unwrap_or_default()
            .iter()
            .collect();
        let next = self.at(0);
        let raw_prefix = matches!(word.as_str(), "r" | "br" | "cr");
        if word == "r" && next == Some('#') && self.at(1).is_some_and(is_word) {
            // A raw name such as `r#type` is the plain name `type`.
            self.step();
            self.word(out);
        } else if raw_prefix && matches!(next, Some('#' | '"')) {
            self.raw();
        } else if !(matches!(word.as_str(), "b" | "c")
            && matches!(next, Some('"' | '\'')))
        {
            out.push((word, line));
        }
    }
}

fn is_word(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// The code words and symbols of `text`.
#[must_use]
pub fn tokens(text: &str) -> Vec<Token> {
    let mut r = Reader {
        chars: text.chars().collect(),
        i: 0,
        line: 1,
    };
    let mut out = Vec::new();
    while let Some(c) = r.at(0) {
        if c == '/' && matches!(r.at(1), Some('/' | '*')) {
            r.comment();
        } else if c == '"' {
            r.step();
            r.quoted();
        } else if c == '\'' {
            if !r.char_literal() {
                out.push(("'".to_owned(), r.line));
                r.step();
            }
        } else if is_word(c) {
            r.word(&mut out);
        } else {
            if !c.is_whitespace() {
                out.push((c.to_string(), r.line));
            }
            r.step();
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::tokens;

    fn words(text: &str) -> Vec<String> {
        tokens(text).into_iter().map(|(w, _)| w).collect()
    }

    #[test]
    fn drops_comments_and_every_kind_of_literal() {
        let text = "a // b\n/* c /* d */ e */ f \"g\" r\"h\\\" i r#\"j\"k\"# \
                    l b\"m\" n '\"' o b'\"' p '\\'' q \
                    cr\"s\\\" r#type c\"t\" u";
        assert_eq!(
            words(text),
            ["a", "f", "i", "l", "n", "o", "p", "q", "type", "u"]
                .map(str::to_owned)
        );
    }

    #[test]
    fn keeps_lifetimes_and_lines() {
        let found = tokens("fn f<'a>(x: &'a u8)\n{ y }");
        assert!(found.iter().any(|(w, _)| w == "a"));
        assert_eq!(found.last().map(|(_, l)| *l), Some(2));
    }
}
