"""Running one sandboxed check and keeping its private output.

Output goes straight to private files, never into memory, and only its
SHA-256 enters the evidence (raw output stays private, SPEC-001). A check
that cannot start, reaches its time limit (the sandbox's own limit, or the
backstop here, which also stops the sandbox) or writes more than
MAX_OUTPUT bytes on either stream is BLOCKED: it decided nothing. A judge
then reads the output when the check reports through it (for example the
unit-test harness summary) and has the last word.
"""
import os
import re
import subprocess
import time

from tools.runner import candidate, outcomes, reports, sandbox

MARGIN = 30  # seconds the backstop allows beyond the sandbox's limit
MAX_OUTPUT = sandbox.FILE_LIMIT
_ID = re.compile(r"[a-z0-9][a-z0-9-]{0,63}")


def _invoke(argv, out, err, seconds, run, stop, stdin_data):
    """(exit code or None, why the check decided nothing or None)."""
    given = ({"stdin": subprocess.DEVNULL} if stdin_data is None
             else {"input": stdin_data})
    try:
        done = run(argv, stdout=out, stderr=err, timeout=seconds + MARGIN,
                   **given)
    except subprocess.TimeoutExpired:
        stop()
        return None, "time limit"
    except OSError as error:
        return None, f"cannot start: {type(error).__name__}"
    return done.returncode, None


def _same(code):
    return code


def _nothing():
    return None


def _reason(code, status):
    if status == 0:
        return ""
    return f"exit {status}" if status is not None else f"sandbox exit {code}"


def run_check(check_id, argv, private_dir, seconds, *, run=subprocess.run,
              clock=time.monotonic, decode=_same, stop=_nothing,
              judge=reports.exit_judge, stdin_data=None):
    """Run argv and return the check's result record.

    decode turns the raw exit code into the check's own status, or None
    when the code says the check decided nothing (see sandbox.WRAPPER);
    it must map None (no exit code) to None. stop ends the sandbox after
    the backstop fires; judge(stdout path, outcome, reason) decides last;
    stdin_data, when given, is the check's whole standard input.
    """
    if not _ID.fullmatch(check_id):
        raise ValueError(f"unsafe check identifier: {check_id!r}")
    paths = [os.path.join(private_dir, f"{check_id}.{stream}")
             for stream in ("stdout", "stderr")]
    start = clock()
    with open(paths[0], "wb") as out, open(paths[1], "wb") as err:
        code, why = _invoke(argv, out, err, seconds, run, stop, stdin_data)
    elapsed = clock() - start
    if why is None and elapsed >= seconds:
        why = "time limit"
    if why is None and max(map(os.path.getsize, paths)) > MAX_OUTPUT:
        why = "output too large"
    status = decode(code)
    outcome, reason = judge(paths[0],
                            outcomes.from_exit(status, why is not None),
                            why or _reason(code, status))
    return {"id": check_id, "outcome": outcome, "reason": reason,
            "exit": code, "seconds": round(elapsed, 3),
            "stdout_sha256": candidate.file_sha256(paths[0]),
            "stderr_sha256": candidate.file_sha256(paths[1])}
