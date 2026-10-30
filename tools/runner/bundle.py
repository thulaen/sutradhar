"""Evidence bundles: what was checked, how, and with what result.

A bundle is canonical JSON (sorted keys, no spaces, ASCII), so the signed
bytes can be re-derived and compared exactly. Verification never trusts
the bundle's own summary: it checks the signature, the canonical form,
the expected candidate, profile and policy, the age, and recomputes the
status from the individual results (trust boundaries AT-TB02, AT-TB03).
"""
import datetime
import json

from tools.runner import outcomes

FORMAT = "sutradhar-evidence/2"
FUTURE_SKEW = 300  # seconds another clock may run ahead
_TIME = "%Y-%m-%dT%H:%M:%SZ"
_CANONICAL = {"sort_keys": True, "separators": (",", ":"), "allow_nan": False}
_SHAPE = {"plan": dict, "checks": list, "bootstrap": bool, "status": str,
          "finished": str, "changed": list, "changed_executable": list}


def canonical(doc):
    """The exact bytes that are signed and verified (ASCII by default)."""
    return json.dumps(doc, **_CANONICAL).encode()


def timestamp(moment):
    """A UTC time in the bundle's format."""
    return moment.strftime(_TIME)


def build(facts, plan, results, bootstrap):
    """The bundle for these facts, with the status recomputed."""
    status, reasons = outcomes.overall(plan, results, bootstrap)
    return dict(facts, format=FORMAT, plan=dict(plan),
                checks=list(results), bootstrap=bootstrap, status=status,
                reasons=reasons)


def _parse(data):
    try:
        doc = json.loads(data)
        return doc if canonical(doc) == data else None
    except ValueError:
        return None


def _finished(doc):
    """The finishing time, or None when the bundle is malformed."""
    if any(type(doc.get(key)) is not kind for key, kind in _SHAPE.items()):
        return None
    plan = doc["plan"]
    if set(plan) != set(outcomes.PLAN_KEYS) or not all(
            type(plan[key]) is list for key in outcomes.PLAN_KEYS):
        return None
    try:
        moment = datetime.datetime.strptime(doc["finished"], _TIME)
    except ValueError:
        return None
    return moment.replace(tzinfo=datetime.timezone.utc)


def _age(finished, now, max_age):
    seconds = (now - finished).total_seconds()
    if seconds > max_age:
        return ["evidence expired"]
    if -seconds > FUTURE_SKEW:
        return ["finished in the future"]
    return []


def _nothing_more(doc):
    return []


def verify(data, signature, trust, expect, now, max_age,
           extra=_nothing_more):
    """(status, reasons) for signed bundle bytes, recomputed, never trusted.

    trust(data, signature) checks the signature; expect maps bundle keys
    to the values the verifier's own protected inputs require; extra(doc)
    returns reasons from any further recomputation the verifier makes.
    """
    if not trust(data, signature):
        return outcomes.BLOCKED, ["signature not valid"]
    doc = _parse(data)
    if not isinstance(doc, dict):
        return outcomes.BLOCKED, ["not canonical JSON"]
    if doc.get("format") != FORMAT:
        return outcomes.BLOCKED, ["unsupported format"]
    finished = _finished(doc)
    if finished is None:
        return outcomes.BLOCKED, ["malformed bundle"]
    reasons = [f"{key} differs from the expected value"
               for key, value in expect.items() if doc.get(key) != value]
    reasons += _age(finished, now, max_age) + extra(doc)
    status, problems = outcomes.overall(doc["plan"], doc["checks"],
                                        doc["bootstrap"])
    if status != doc["status"]:
        reasons.append(f"status claims {doc['status']}; checks give {status}")
    return (outcomes.BLOCKED, reasons) if reasons else (status, problems)
