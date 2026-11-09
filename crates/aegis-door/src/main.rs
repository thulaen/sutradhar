//! The `aegis-door` service: `aegis-door SOCKET WORK [COUNT]`. It listens
//! on the local socket `SOCKET` and checks pushes in the repository
//! `WORK`, one at a time. With `COUNT`, it stops after that many
//! requests.

use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::UnixListener;
use std::process::ExitCode;
use std::time::Duration;

use aegis_door::gate::Door;
use aegis_door::serve;

const USAGE: &str = "usage: aegis-door SOCKET WORK [COUNT]\n";
/// How long the door waits for a caller that has gone quiet.
const WAIT: Duration = Duration::from_secs(60);

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (socket, work, count) = match args.as_slice() {
        [s, w] => (s, w, usize::MAX),
        [s, w, n] => match n.parse() {
            Ok(n) => (s, w, n),
            Err(_) => return usage(),
        },
        _ => return usage(),
    };
    match listen(socket, work, count) {
        Ok(()) => ExitCode::SUCCESS,
        Err(problem) => {
            say(&format!("aegis-door: {problem}\n"));
            ExitCode::FAILURE
        }
    }
}

fn usage() -> ExitCode {
    say(USAGE);
    ExitCode::from(2)
}

fn listen(socket: &str, work: &str, count: usize) -> Result<(), String> {
    const SOCKET_MODE: u32 = 0o660;
    let _old = fs::remove_file(socket);
    let listener = UnixListener::bind(socket)
        .map_err(|e| format!("cannot listen on {socket}: {e}"))?;
    // Only the door's own account and Forgejo's may connect. The socket
    // folder gives Forgejo's account a default access entry (see
    // deploy/door/install.sh), which the socket inherits; 0o660 keeps
    // that entry in force and shuts everyone else out.
    fs::set_permissions(socket, fs::Permissions::from_mode(SOCKET_MODE))
        .map_err(|e| format!("cannot open up {socket}: {e}"))?;
    say(&format!("aegis-door: listening on {socket}\n"));
    let door = Door::new(work);
    let connections = listener.incoming().take(count).map(|c| {
        c.and_then(|s| {
            s.set_read_timeout(Some(WAIT))?;
            s.set_write_timeout(Some(WAIT))?;
            Ok(s)
        })
    });
    serve::serve_all(&door, connections, &mut io::stderr());
    Ok(())
}

fn say(text: &str) {
    let _ignored = io::stderr().write_all(text.as_bytes());
}
