"""A run's private inputs: the host file and the evidence records.

The host file (never in Git) names the command prefix used to start
sandboxes and where each pinned tool is installed. The evidence file
names records that stand in for checks not yet active and the review
records. Each record is a small JSON document that names the exact tree;
an interim record also names its check and shows it passed, so a log
covering other trees, another check or a failure is refused.
"""
import json
import os

from tools.runner import candidate

_FORMATS = {"interim": ("sutradhar-interim/1", "by"),
            "review": ("sutradhar-review/1", "reviewer")}


class Blocked(Exception):
    """The run cannot produce trustworthy evidence."""


def read(path):
    """A file's bytes."""
    with open(path, "rb") as handle:
        return handle.read()


def host(path):
    """(command prefix, installed tool paths) from the private host file."""
    try:
        found = json.loads(read(path))
        prefix, tools = found["prefix"], found["tools"]
        if (isinstance(prefix, list) and isinstance(tools, dict)
                and all(isinstance(part, str) for part in prefix)
                and all(isinstance(place, str) and place.startswith("/")
                        for place in tools.values())):
            return prefix, tools
    except (OSError, ValueError, KeyError, TypeError):
        pass
    raise Blocked("bad host configuration")


def _record(path, tree, check):
    """The bundle entry for one record; check is None for a review."""
    name = os.path.basename(path)
    form, author = _FORMATS["review" if check is None else "interim"]
    try:
        found = json.loads(read(path))
    except ValueError:
        raise Blocked(f"bad evidence record: {name}") from None
    if not (isinstance(found, dict) and found.get("format") == form
            and isinstance(found.get(author), str) and found[author].strip()):
        raise Blocked(f"bad evidence record: {name}")
    if found.get("tree") != tree:
        raise Blocked(f"evidence not for this tree: {name}")
    if check is not None and found.get("check") != check:
        raise Blocked(f"evidence for another check: {name}")
    if check is not None and found.get("outcome") != "PASSED":
        raise Blocked(f"evidence does not show a pass: {name}")
    return {"file": name, "sha256": candidate.path_digest(path)}


def _entries(paths, tree, check):
    if not (isinstance(paths, list)
            and all(isinstance(p, str) and p.startswith("/")
                    for p in paths)):
        raise Blocked("bad evidence file")
    return [_record(path, tree, check) for path in paths]


def evidence(path, tree):
    """(interim records by check, review records) named in the file."""
    if path is None:
        return {}, []
    try:
        found = json.loads(read(path))
    except (OSError, ValueError):
        raise Blocked("bad evidence file") from None
    if not (isinstance(found, dict) and set(found) <= {"interim", "reviews"}
            and isinstance(found.get("interim", {}), dict)):
        raise Blocked("bad evidence file")
    interim = {check: _entries(paths, tree, check)
               for check, paths in found.get("interim", {}).items()}
    return interim, _entries(found.get("reviews", []), tree, None)
