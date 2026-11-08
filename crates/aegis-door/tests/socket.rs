//! The door's socket and program, tested end to end.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

mod common;

use std::io::{Cursor, Read, Write};
use std::process::{Command, Stdio};

use aegis_door::gate::Door;
use aegis_door::serve;
use common::{GOOD, MAIN, commit, door_work, folder, pack, setup};

/// A stream that reads a fixed request and keeps what is written.
struct Pipe {
    input: Cursor<Vec<u8>>,
    output: Vec<u8>,
}

impl Read for Pipe {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.input.read(buf)
    }
}

impl Write for Pipe {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.output.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn ask(door: &Door, target: &str, body: &[u8]) -> String {
    let mut raw = format!(
        "POST {target} HTTP/1.1\r\nContent-Length: {}\r\n\r\n",
        body.len()
    )
    .into_bytes();
    raw.extend_from_slice(body);
    let mut pipe = Pipe {
        input: Cursor::new(raw),
        output: Vec::new(),
    };
    serve::answer(door, &mut pipe).unwrap();
    String::from_utf8(pipe.output).unwrap()
}

#[test]
fn the_socket_answers_yes_and_no() {
    let (source, door, first) = setup("serve");
    let next = commit(&source, "b.md", "# B\n", GOOD);
    let body = pack(&source, &first, &next);
    let url = |old: &str, new: &str| {
        format!("/check?ref={MAIN}&old={old}&new={new}&main={old}")
    };
    let yes = ask(&door, &url(&first, &next), &body);
    assert!(yes.starts_with("HTTP/1.1 200 OK"), "{yes}");
    let bad = commit(&source, "c.md", "# C\n", "stuff");
    let body = pack(&source, &next, &bad);
    let no = ask(&door, &url(&next, &bad), &body);
    assert!(
        no.contains("403") && no.contains("the push is refused"),
        "{no}"
    );
    let twice = format!("{}&ref=refs/heads/x", url(&next, &bad));
    for target in ["/other", "/check?old=a", &twice] {
        assert!(ask(&door, target, &[]).contains("403"), "{target}");
    }
    let mut broken = Pipe {
        input: Cursor::new(b"GET / HTTP/1.1\r\n\r\n".to_vec()),
        output: Vec::new(),
    };
    serve::answer(&door, &mut broken).unwrap();
    assert!(String::from_utf8(broken.output).unwrap().contains("403"));
}

/// A stream that cannot be written to.
struct Closed;

impl Read for Closed {
    fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
        Ok(0)
    }
}

impl Write for Closed {
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
        Err(std::io::Error::other("closed"))
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
fn serve_all_logs_bad_connections_and_carries_on() {
    let (_source, door, _first) = setup("all");
    let connections: Vec<std::io::Result<Closed>> =
        vec![Err(std::io::Error::other("lost")), Ok(Closed)];
    let mut log = Vec::new();
    serve::serve_all(&door, connections.into_iter(), &mut log);
    let log = String::from_utf8(log).unwrap();
    assert!(log.contains("bad connection: lost"), "{log}");
    assert!(log.contains("cannot answer: closed"), "{log}");
}

fn door_program(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_aegis-door"))
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn the_program_checks_bad_use() {
    for args in [&[][..], &["a"], &["a", "b", "x"]] {
        assert_eq!(door_program(args).status.code(), Some(2), "{args:?}");
    }
    let out = door_program(&["/no/such/folder/door.sock", "w"]);
    assert_eq!(out.status.code(), Some(1));
}

#[test]
fn the_program_answers_on_its_socket() {
    let (source, _door, first) = setup("program");
    let next = commit(&source, "b.md", "# B\n", GOOD);
    let body = pack(&source, &first, &next);
    let socket = folder("program-sock").join("door.sock");
    let mut child = Command::new(env!("CARGO_BIN_EXE_aegis-door"))
        .arg(&socket)
        .arg(door_work("program"))
        .arg("1")
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stream = loop {
        if let Ok(s) = std::os::unix::net::UnixStream::connect(&socket) {
            break s;
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    };
    let head = format!(
        "POST /check?ref={MAIN}&old={first}&new={next}&main={first} \
         HTTP/1.1\r\nContent-Length: {}\r\n\r\n",
        body.len()
    );
    stream.write_all(head.as_bytes()).unwrap();
    stream.write_all(&body).unwrap();
    let mut answer = String::new();
    let _ = stream.read_to_string(&mut answer).unwrap();
    assert!(answer.starts_with("HTTP/1.1 200 OK"), "{answer}");
    assert!(child.wait().unwrap().success());
}
