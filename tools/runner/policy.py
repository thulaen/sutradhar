"""The runner policy: checks, profiles and pinned tools (SPEC-001).

The caller names the protected copy (from the base commit or the
installed tools), never the candidate's. Loading is strict: unknown keys,
wrong types, unknown checks or tools, empty reasons and overlapping lists
are refused, so a typing mistake cannot silently drop a check, and a check
that is already defined cannot be listed as not yet active. A
documentation-only profile is accepted only when something changed and
every changed path matches a documentation pattern and has no executable
bit before or after the change. A candidate is inside the bootstrap scope
only when every changed path matches a bootstrap pattern. Placeholders:
@paths (every candidate path), @candidate (the read-only export), @scan
(the folder holding only the export) and @tool:NAME[/path] (a pinned
tool).
"""
import fnmatch
import json
import posixpath
import re

from tools.runner import sandbox

FORMAT = "sutradhar-runner-policy/1"
_ID = re.compile(r"[a-z0-9][a-z0-9-]{0,63}")
_HASH = re.compile(r"[0-9a-f]{64}")
_TOOL = re.compile(r"@tool:([a-z0-9][a-z0-9-]{0,63})((?:/[A-Za-z0-9._-]+)*)")
_WORDS = ("@paths", "@candidate", "@scan")
_REPORTS = ("exit", "unittest")
_TOP = {"format": str, "version": str, "runner": str,
        "bootstrap_scope": list, "max_age": int, "limits": dict,
        "documentation": list, "tools": dict, "checks": dict,
        "profiles": dict}
_LIMITS = {"memory": str, "tasks": int, "seconds": int}
_CHECK = {"cwd": str, "report": str, "argv": list}
_PROFILE = {"documentation_only": bool, "run": list,
            "not_applicable": dict, "not_yet_active": dict, "interim": list}


class PolicyError(Exception):
    """The policy or the requested profile cannot be used."""


def _refuse(name):
    raise ValueError(f"{name} is not allowed")


def _shape(doc, fields, where):
    if not isinstance(doc, dict) or set(doc) != set(fields):
        raise PolicyError(f"{where}: expected keys {sorted(fields)}")
    for key, kind in fields.items():
        if type(doc[key]) is not kind:
            raise PolicyError(f"{where}.{key}: expected {kind.__name__}")


def _ids(values, where):
    for value in values:
        if type(value) is not str or not _ID.fullmatch(value):
            raise PolicyError(f"{where}: bad identifier {value!r}")


def _tool_path(value, tools):
    """The path inside a known pinned tool ("" for the tool), else None."""
    match = _TOOL.fullmatch(value)
    if not match or match.group(1) not in tools:
        return None
    if {".", ".."} & set(match.group(2).split("/")):
        return None
    return match.group(2)


def _checks(checks, tools):
    _ids(checks, "checks")
    for name, check in checks.items():
        where = f"checks.{name}"
        _shape(check, _CHECK, where)
        folder = _tool_path(check["cwd"], tools)
        if check["cwd"] != "@candidate" and folder != "":
            raise PolicyError(f"{where}: cwd must be @candidate or a tool")
        if check["report"] not in _REPORTS:
            raise PolicyError(f"{where}: report must be exit or unittest")
        if not check["argv"]:
            raise PolicyError(f"{where}: empty argv")
        for value in check["argv"]:
            if type(value) is not str:
                raise PolicyError(f"{where}: arguments must be text")
            unknown = (value.startswith("@") and value not in _WORDS
                       and _tool_path(value, tools) is None)
            if unknown:
                raise PolicyError(f"{where}: unknown placeholder {value}")


def _profile(profile, checks, where):
    _shape(profile, _PROFILE, where)
    run, skipped = profile["run"], profile["not_applicable"]
    waiting = profile["not_yet_active"]
    names = [*run, *skipped, *waiting]
    _ids(names, where)
    if not run or len(set(names)) != len(names):
        raise PolicyError(f"{where}: empty or overlapping check lists")
    if not set(run) | set(skipped) <= set(checks):
        raise PolicyError(f"{where}: unknown check")
    for reason in [*skipped.values(), *waiting.values()]:
        if type(reason) is not str or not reason.strip():
            raise PolicyError(f"{where}: an exemption needs a reason")
    if not set(profile["interim"]) <= set(waiting):
        raise PolicyError(f"{where}: interim checks must be not yet active")
    active = sorted(set(waiting) & set(checks))
    if active:
        raise PolicyError(f"{where}: an active check cannot be not yet "
                          f"active: {', '.join(active)}")


def _texts(values, message):
    if not all(type(value) is str for value in values):
        raise PolicyError(message)


def _top(doc):
    if doc["format"] != FORMAT or not doc["version"]:
        raise PolicyError("policy: unsupported format or empty version")
    if not _HASH.fullmatch(doc["runner"]):
        raise PolicyError("runner: must be a SHA-256 digest")
    _texts(doc["bootstrap_scope"], "bootstrap_scope: patterns must be text")
    if doc["max_age"] < 1:
        raise PolicyError("max_age: must be a positive whole number")


def load(data):
    """The validated policy from its file's bytes."""
    try:
        doc = json.loads(data, parse_constant=_refuse)
    except ValueError as error:
        raise PolicyError(f"not JSON: {error}") from None
    _shape(doc, _TOP, "policy")
    _top(doc)
    _shape(doc["limits"], _LIMITS, "limits")
    _ids(doc["tools"], "tools")
    if not all(type(pin) is str and _HASH.fullmatch(pin)
               for pin in doc["tools"].values()):
        raise PolicyError("tools: each pin must be a SHA-256 digest")
    _texts(doc["documentation"], "documentation: patterns must be text")
    _checks(doc["checks"], doc["tools"])
    if not doc["profiles"]:
        raise PolicyError("profiles: none defined")
    _ids(doc["profiles"], "profiles")
    for name, profile in doc["profiles"].items():
        _profile(profile, doc["checks"], f"profiles.{name}")
    return doc


def _matches(patterns, path):
    base = posixpath.basename(path)
    return any(fnmatch.fnmatchcase(path if "/" in pattern else base, pattern)
               for pattern in patterns)


def select(loaded, name, changed, executable=()):
    """The named profile, refusing documentation-only for other changes.

    executable lists the changed paths whose executable bit is set in the
    base or in the candidate.
    """
    profile = loaded["profiles"].get(name)
    if profile is None:
        raise PolicyError(f"unknown profile: {name}")
    if profile["documentation_only"]:
        if not changed:
            raise PolicyError("documentation-only profile refused: "
                              "no change")
        marked = set(executable)
        for path in changed:
            if path in marked or not _matches(loaded["documentation"], path):
                raise PolicyError("documentation-only profile refused: "
                                  f"not a documentation change: {path}")
    return profile


def in_scope(loaded, changed):
    """Whether every changed path is inside the bootstrap scope."""
    patterns = loaded["bootstrap_scope"]
    return bool(changed) and all(_matches(patterns, p) for p in changed)


def plan(loaded, name):
    """The checks the named profile requires, by kind."""
    profile = loaded["profiles"][name]
    return {"run": list(profile["run"]),
            "not_applicable": list(profile["not_applicable"]),
            "not_yet_active": list(profile["not_yet_active"]),
            "interim": list(profile["interim"])}


def _place(value):
    """A single value with @candidate, @scan or @tool:NAME expanded."""
    if value == "@candidate":
        return sandbox.CANDIDATE
    if value == "@scan":
        return sandbox.SCAN
    match = _TOOL.fullmatch(value)
    if match:
        return f"{sandbox.TOOLS}/{match.group(1)}{match.group(2)}"
    return value


def argv(loaded, check_id, paths):
    """The check's command with placeholders expanded."""
    found = []
    for value in loaded["checks"][check_id]["argv"]:
        found += paths if value == "@paths" else [_place(value)]
    return found


def cwd(loaded, check_id):
    """The check's working folder inside the sandbox."""
    return _place(loaded["checks"][check_id]["cwd"])
