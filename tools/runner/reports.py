"""Judging a check by its structured report (SPEC-001: no partial pass).

An exit status alone cannot show that every test ran: code under test
can end the interpreter early with status 0. The pinned harness prints a
summary as its last line, carrying a value fresh for each run; the check
passes only when that summary exists, belongs to this run and shows every
collected test ran with no failure, error, skip or uncollected test file.
A clean summary with a failing exit status is contradictory and blocks.
"""
import json

from tools.runner import outcomes

FORMAT = "sutradhar-unittest/1"
TAIL = 64 * 1024  # bytes read from the end of the output
_COUNTS = ("collected", "run", "failures", "errors", "skipped")
_DETAIL = ("ids", "failed_ids", "errored_ids")


def exit_judge(path, outcome, reason):
    """Checks reported by exit status alone keep their outcome."""
    return outcome, reason


def _last(data):
    lines = [line for line in data.decode(errors="replace").splitlines()
             if line.strip()]
    if not lines:
        return None
    try:
        found = json.loads(lines[-1])
    except (ValueError, RecursionError):
        return None
    valid = (isinstance(found, dict) and found.get("harness") == FORMAT
             and all(type(found.get(key)) is int for key in _COUNTS)
             and isinstance(found.get("uncollected"), list))
    return found if valid else None


def _read(path):
    # Output is capped at sandbox.FILE_LIMIT, so reading it whole is safe.
    with open(path, "rb") as handle:
        return handle.read()


def _summary(path):
    return _last(_read(path)[-TAIL:])


def _texts(value):
    return isinstance(value, list) and all(type(v) is str for v in value)


def detail(path):
    """The harness summary with each test's outcome and keys, or None.

    The whole output is read: with many tests the summary line is long.
    """
    found = _last(_read(path))
    keys = found.get("keys") if found else None
    if not (isinstance(keys, dict) and all(map(_texts, keys.values()))
            and all(_texts(found.get(key)) for key in _DETAIL)):
        return None
    return found


def _verdict(found, nonce):
    if found is None:
        return outcomes.BLOCKED, "no test report"
    if found.get("nonce") != nonce:
        return outcomes.BLOCKED, "test report not for this run"
    if found["collected"] == 0:
        return outcomes.BLOCKED, "no tests ran"
    if found["run"] != found["collected"]:
        return (outcomes.BLOCKED,
                f"{found['run']} of {found['collected']} tests ran")
    if found["uncollected"]:
        names = ", ".join(map(str, found["uncollected"]))
        return outcomes.FAILED, f"test files not collected: {names}"
    if found["failures"] or found["errors"]:
        return (outcomes.FAILED,
                f"{found['failures']} failed, {found['errors']} errors")
    if found["skipped"]:
        return outcomes.FAILED, f"{found['skipped']} skipped"
    return outcomes.PASSED, ""


def unittest_judge(nonce):
    """A judge for checks run through the pinned unit-test harness."""
    def judge(path, outcome, reason):
        if outcome == outcomes.BLOCKED:
            return outcome, reason
        verdict = _verdict(_summary(path), nonce)
        if verdict[0] == outcomes.PASSED and outcome == outcomes.FAILED:
            return outcomes.BLOCKED, "exit status and test report disagree"
        return verdict
    return judge
