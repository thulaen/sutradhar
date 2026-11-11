//! Bounded draining keeps the scanner's existing timeout supervisor alive.

use std::io::{self, Read, Write};
use std::process::{Child, ChildStderr, ChildStdout, Command, Stdio};
use std::sync::mpsc::{SyncSender, sync_channel};
use std::thread::{Builder, Scope, ScopedJoinHandle};

const STDOUT_CAP: usize = 32 * 1024 * 1024;
const STDERR_CAP: usize = 64 * 1024;

struct Bounded {
    bytes: Vec<u8>,
    cap: usize,
    overflow: bool,
}

impl Write for Bounded {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let remaining = self.cap.saturating_sub(self.bytes.len());
        self.bytes.extend(bytes.iter().take(remaining));
        self.overflow |= bytes.len() > remaining;
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn drain(mut pipe: impl Read, cap: usize) -> io::Result<Vec<u8>> {
    let mut output = Bounded {
        bytes: Vec::new(),
        cap,
        overflow: false,
    };
    let _read = io::copy(&mut pipe, &mut output)?;
    if output.overflow {
        Err(io::Error::other("program output exceeded its byte limit"))
    } else {
        Ok(output.bytes)
    }
}

type Reader<'scope, T> = (
    SyncSender<Option<T>>,
    ScopedJoinHandle<'scope, io::Result<Vec<u8>>>,
);

fn reader<'scope, 'env, T: Read + Send + 'scope>(
    scope: &'scope Scope<'scope, 'env>,
    cap: usize,
) -> Result<Reader<'scope, T>, String> {
    let (send, receive) = sync_channel::<Option<T>>(0);
    let job = Builder::new()
        .spawn_scoped(scope, move || {
            let pipe =
                receive.recv().ok().flatten().ok_or_else(|| {
                    io::Error::other("program pipe unavailable")
                })?;
            drain(pipe, cap)
        })
        .map_err(|_| "cannot start program output reader".to_owned())?;
    Ok((send, job))
}

fn finish(
    mut child: Child,
    input: &[u8],
    readers: (Reader<'_, ChildStdout>, Reader<'_, ChildStderr>),
    what: &str,
) -> Result<String, String> {
    let ((out_send, output), (err_send, errors)) = readers;
    let stdout_sent = out_send.send(child.stdout.take()).is_ok();
    let stderr_sent = err_send.send(child.stderr.take()).is_ok();
    let sent = child.stdin.take().map(|mut pipe| pipe.write_all(input));
    let ended = child.wait();
    let captured = output.join();
    let diagnostics = errors.join();
    if !stdout_sent || !stderr_sent {
        return Err(format!("{what} output reader failed"));
    }
    if !matches!(sent, Some(Ok(()))) {
        return Err(format!("{what} did not accept its complete input"));
    }
    let status = ended.map_err(|_| format!("{what} did not finish"))?;
    if !status.success() {
        return Err(format!("{what} failed ({status})"));
    }
    if !matches!(diagnostics, Ok(Ok(_))) {
        return Err(format!(
            "{what} error output exceeded its limit or could not be read"
        ));
    }
    let Ok(Ok(bytes)) = captured else {
        return Err(format!(
            "{what} output exceeded its limit or could not be read"
        ));
    };
    String::from_utf8(bytes).map_err(|_| format!("{what} output is not UTF-8"))
}

/// Drains both output pipes while supplying input. Overflow is refused
/// after draining; it does not kill an outer timeout and orphan its child.
pub(super) fn run(
    command: &mut Command,
    input: &[u8],
    what: &str,
) -> Result<String, String> {
    std::thread::scope(|scope| {
        // Start readers before the child, so a failed thread start cannot
        // abandon a running timeout supervisor or its process group.
        let stdout = reader(scope, STDOUT_CAP)?;
        let stderr = reader(scope, STDERR_CAP)?;
        let child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|_| format!("cannot run {what}"))?;
        finish(child, input, (stdout, stderr), what)
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, reason = "tests stop at their first failure")]

    use super::{Bounded, drain, reader};
    use std::io::{self, Read, Write};

    struct ReadError;

    impl Read for ReadError {
        fn read(&mut self, _buffer: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::other("synthetic read failure"))
        }
    }

    #[test]
    fn the_exact_limit_passes_and_one_more_byte_fails() {
        assert_eq!(drain(&b"abcd"[..], 4).ok(), Some(b"abcd".to_vec()));
        assert!(drain(&b"abcde"[..], 4).is_err());
        assert_eq!(drain(&b""[..], 0).ok(), Some(Vec::new()));
        assert!(drain(&b"a"[..], 0).is_err());
    }

    #[test]
    fn read_failures_and_missing_pipes_are_refused() {
        assert!(drain(ReadError, 4).is_err());
        missing_pipe(true);
        missing_pipe(false);
    }

    fn missing_pipe(send_none: bool) {
        std::thread::scope(|scope| {
            let (send, job) = reader::<&[u8]>(scope, 4).unwrap();
            if send_none {
                assert!(send.send(None).is_ok());
            }
            drop(send);
            assert!(matches!(job.join(), Ok(Err(_))));
        });
    }

    #[test]
    fn bounded_writes_keep_only_the_prefix_and_flush_safely() {
        let mut output = Bounded {
            bytes: Vec::new(),
            cap: 4,
            overflow: false,
        };
        assert!(output.write_all(b"abcdefgh").is_ok());
        assert_eq!(output.bytes, b"abcd");
        assert!(output.overflow);
        assert!(output.flush().is_ok());
    }
}
