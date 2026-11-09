//! Answering one request on the door's socket.

use std::io::{Read, Write};

use crate::gate::{Door, Push};
use crate::http;

/// The longest a caller may take to send its whole request.
pub const REQUEST_TIME: std::time::Duration =
    std::time::Duration::from_secs(600);

/// Reads one request from `stream`, checks the push, and writes the
/// answer. Every problem, of any kind, refuses the push.
///
/// # Errors
/// When the answer cannot be written.
pub fn answer(
    door: &Door,
    mut stream: impl Read + Write,
) -> std::io::Result<()> {
    let (ok, text) = decide(door, &mut stream);
    let answer = if ok {
        http::Answer::Allow
    } else {
        http::Answer::Refuse
    };
    http::write_answer(stream, answer, &text)
}

fn decide(door: &Door, stream: impl Read) -> (bool, String) {
    let stream = http::Deadline::new(stream, REQUEST_TIME);
    let request = match http::read_request(stream) {
        Ok(r) => r,
        Err(problem) => return (false, format!("door: {problem}\n")),
    };
    if !request.target.starts_with("/check?") {
        return (false, "door: unknown request\n".to_owned());
    }
    let (Some(name), Some(old), Some(new), Some(main)) = (
        request.param("ref"),
        request.param("old"),
        request.param("new"),
        request.param("main"),
    ) else {
        return (
            false,
            "door: `ref`, `old`, `new` and `main` are needed\n".to_owned(),
        );
    };
    let push = Push {
        name,
        old,
        new,
        main,
    };
    match door.check(push, &request.body) {
        Ok(findings) if findings.is_empty() => {
            (true, "aegis: every check passed\n".to_owned())
        }
        Ok(findings) => {
            let mut lines: Vec<String> =
                findings.iter().map(ToString::to_string).collect();
            lines.push(format!(
                "aegis: {} problem(s); the push is refused",
                findings.len()
            ));
            (false, format!("{}\n", lines.join("\n")))
        }
        Err(problem) => (false, format!("door: {problem}\n")),
    }
}

/// Answers every connection in turn. A connection or answer that fails
/// is written to `log`, and the door carries on with the next.
pub fn serve_all<S: Read + Write>(
    door: &Door,
    connections: impl Iterator<Item = std::io::Result<S>>,
    log: &mut impl Write,
) {
    for connection in connections {
        let problem = match connection {
            Ok(stream) => answer(door, stream)
                .err()
                .map(|e| format!("aegis-door: cannot answer: {e}\n")),
            Err(e) => Some(format!("aegis-door: bad connection: {e}\n")),
        };
        if let Some(text) = problem {
            let _ignored = log.write_all(text.as_bytes());
        }
    }
}
