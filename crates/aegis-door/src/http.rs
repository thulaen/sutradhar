//! The smallest HTTP the door needs: one request in, one answer out,
//! over a local socket.

use std::io::{BufRead, BufReader, Read, Write};
use std::time::{Duration, Instant};

/// The most bytes a request body may have: 512 MiB.
pub const MAX_BODY: usize = 512 * 1024 * 1024;
/// The most header lines a request may have.
const MAX_HEADER_LINES: usize = 64;
/// The most bytes one request line may have.
pub const MAX_LINE: usize = 8192;

/// One request: its path with the query, and its body.
#[derive(Debug, PartialEq, Eq)]
pub struct Request {
    /// The path, such as `/check?old=a&new=b`.
    pub target: String,
    /// The body.
    pub body: Vec<u8>,
}

impl Request {
    /// The value of one query parameter. A name given more than once has
    /// no value, so a crafted ref name cannot smuggle in a second one.
    #[must_use]
    pub fn param(&self, name: &str) -> Option<&str> {
        let (_, query) = self.target.split_once('?')?;
        let mut values = query
            .split('&')
            .filter_map(|pair| pair.split_once('='))
            .filter(|(key, _)| *key == name)
            .map(|(_, value)| value);
        let first = values.next()?;
        values.next().is_none().then_some(first)
    }
}

/// A reader that gives up once a deadline has passed, so a caller cannot
/// hold the door by sending one byte at a time.
#[derive(Debug)]
pub struct Deadline<R> {
    inner: R,
    until: Instant,
}

impl<R> Deadline<R> {
    /// Wraps `inner`; reads fail once `allowed` has passed.
    pub fn new(inner: R, allowed: Duration) -> Self {
        Self {
            inner,
            until: Instant::now()
                .checked_add(allowed)
                .unwrap_or_else(Instant::now),
        }
    }
}

impl<R: Read> Read for Deadline<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if Instant::now() >= self.until {
            return Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "the request took too long",
            ));
        }
        self.inner.read(buf)
    }
}
/// Reads one request.
///
/// # Errors
/// When the request is not a well-formed POST, or is too big.
pub fn read_request(stream: impl Read) -> Result<Request, String> {
    let mut reader = BufReader::new(stream);
    let first = read_line(&mut reader)?;
    let mut parts = first.split(' ');
    let (Some("POST"), Some(target)) = (parts.next(), parts.next()) else {
        return Err(format!("not a POST request: {first}"));
    };
    let target = target.to_owned();
    let mut length = 0usize;
    for _ in 0..MAX_HEADER_LINES {
        let line = read_line(&mut reader)?;
        if line.is_empty() {
            return read_body(reader, length)
                .map(|body| Request { target, body });
        }
        if let Some((name, value)) = line.split_once(':')
            && name.eq_ignore_ascii_case("content-length")
        {
            length = value
                .trim()
                .parse()
                .map_err(|_| format!("bad length: {value}"))?;
        }
    }
    Err("too many header lines".to_owned())
}

/// Reads one line of at most [`MAX_LINE`] bytes.
fn read_line(reader: &mut impl BufRead) -> Result<String, String> {
    let mut line = String::new();
    let limit = u64::try_from(MAX_LINE).map_err(|e| e.to_string())?;
    let _read = reader
        .take(limit)
        .read_line(&mut line)
        .map_err(|e| format!("cannot read the request: {e}"))?;
    if !line.ends_with('\n') && line.len() >= MAX_LINE {
        return Err("a request line is too long".to_owned());
    }
    Ok(line.trim_end_matches(['\r', '\n']).to_owned())
}

fn read_body(reader: impl Read, length: usize) -> Result<Vec<u8>, String> {
    if length > MAX_BODY {
        return Err(format!("the body is over {MAX_BODY} bytes"));
    }
    let wanted = u64::try_from(length).map_err(|e| e.to_string())?;
    let mut body = Vec::new();
    let _read = reader
        .take(wanted)
        .read_to_end(&mut body)
        .map_err(|e| format!("cannot read the body: {e}"))?;
    if body.len() == length {
        Ok(body)
    } else {
        Err("the body ended early".to_owned())
    }
}

/// Writes one answer: `200 OK` when `ok`, else `403 Forbidden`.
///
/// # Errors
/// When the answer cannot be written.
pub fn write_answer(
    mut stream: impl Write,
    ok: bool,
    text: &str,
) -> std::io::Result<()> {
    let status = if ok { "200 OK" } else { "403 Forbidden" };
    let head = format!(
        "HTTP/1.1 {status}\r\nContent-Type: text/plain; charset=utf-8\r\n\
         Content-Length: {}\r\nConnection: close\r\n\r\n",
        text.len()
    );
    stream.write_all(head.as_bytes())?;
    stream.write_all(text.as_bytes())?;
    stream.flush()
}

#[cfg(test)]
mod tests {
    use super::{Deadline, MAX_BODY, MAX_LINE, read_request, write_answer};
    use std::time::Duration;

    #[test]
    fn reads_a_post_with_a_body_and_params() {
        let raw = b"POST /check?old=a&new=b HTTP/1.1\r\nHost: door\r\n\
            content-length: 3\r\n\r\nabc";
        let request = read_request(&raw[..]).unwrap();
        assert_eq!(request.body, b"abc");
        assert_eq!(request.param("old"), Some("a"));
        assert_eq!(request.param("new"), Some("b"));
        assert_eq!(request.param("ref"), None);
    }

    #[test]
    fn a_request_without_a_query_has_no_params() {
        let raw = b"POST /check HTTP/1.1\r\n\r\n";
        assert_eq!(read_request(&raw[..]).unwrap().param("old"), None);
    }

    #[test]
    fn refuses_bad_requests() {
        let too_big = format!(
            "POST / HTTP/1.1\r\nContent-Length: {}\r\n\r\n",
            MAX_BODY + 1
        );
        let many = format!("POST / HTTP/1.1\r\n{}", "X: y\r\n".repeat(70));
        let cases: [&[u8]; 5] = [
            b"GET / HTTP/1.1\r\n\r\n",
            b"POST / HTTP/1.1\r\nContent-Length: x\r\n\r\n",
            b"POST / HTTP/1.1\r\nContent-Length: 9\r\n\r\nabc",
            too_big.as_bytes(),
            many.as_bytes(),
        ];
        for raw in cases {
            assert!(read_request(raw).is_err());
        }
    }

    #[test]
    fn a_name_given_twice_has_no_value() {
        let raw = b"POST /c?ref=a&ref=b&old=x HTTP/1.1\r\n\r\n";
        let request = read_request(&raw[..]).unwrap();
        assert_eq!(request.param("ref"), None);
        assert_eq!(request.param("old"), Some("x"));
    }

    #[test]
    fn refuses_a_line_that_is_too_long() {
        let raw = format!("POST /{} HTTP/1.1\r\n\r\n", "x".repeat(MAX_LINE));
        assert!(read_request(raw.as_bytes()).is_err());
    }

    #[test]
    fn a_deadline_stops_slow_requests() {
        let raw = b"POST /c HTTP/1.1\r\n\r\n";
        let slow = Deadline::new(&raw[..], Duration::ZERO);
        assert!(read_request(slow).is_err());
        let fast = Deadline::new(&raw[..], Duration::from_secs(60));
        assert!(read_request(fast).is_ok());
    }

    #[test]
    fn writes_both_kinds_of_answer() {
        let mut out = Vec::new();
        write_answer(&mut out, true, "fine").unwrap();
        let text = String::from_utf8(out).unwrap();
        assert!(text.starts_with("HTTP/1.1 200 OK\r\n"));
        assert!(text.ends_with("\r\n\r\nfine"));
        let mut out = Vec::new();
        write_answer(&mut out, false, "no").unwrap();
        assert!(String::from_utf8(out).unwrap().contains("403 Forbidden"));
    }
}
