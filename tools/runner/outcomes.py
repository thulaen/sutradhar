"""Check outcomes and how they combine (SPEC-001; SPEC-006 activation).

A plan names the checks a profile requires: those that must run, those
not applicable and those not yet active (some of which need interim
evidence). A missing, unknown, duplicated, unexplained or wrongly
classified result never becomes a pass: the whole set is then BLOCKED,
whatever else it shows. NOT_YET_ACTIVE counts only inside the bootstrap
scope. Otherwise any failure makes the set FAILED, any block makes it
BLOCKED, and only then is it PASSED.
"""
PASSED, FAILED, BLOCKED = "PASSED", "FAILED", "BLOCKED"
NOT_APPLICABLE, NOT_YET_ACTIVE = "NOT_APPLICABLE", "NOT_YET_ACTIVE"
OUTCOMES = (PASSED, FAILED, BLOCKED, NOT_APPLICABLE, NOT_YET_ACTIVE)
PLAN_KEYS = ("run", "not_applicable", "not_yet_active", "interim")
_EXEMPT = (NOT_APPLICABLE, NOT_YET_ACTIVE)
# Exit 0 passes and 1 fails; anything else (usage errors, "no tests",
# signals, crashes of the sandbox itself) means the check did not decide.
_BY_EXIT = {0: PASSED, 1: FAILED}


def from_exit(code, undecided):
    """The outcome of one sandboxed check from its exit status.

    undecided is true when the check was cut short (time limit, failure
    to start, oversized output): its exit status then means nothing.
    """
    if undecided:
        return BLOCKED
    return _BY_EXIT.get(code, BLOCKED)


def checks(plan):
    """Every check the plan requires, each once, in order."""
    return [*plan["run"], *plan["not_applicable"], *plan["not_yet_active"]]


def _expected(plan, check):
    if check in plan["not_applicable"]:
        return NOT_APPLICABLE
    if check in plan["not_yet_active"]:
        return NOT_YET_ACTIVE
    return None


def _kind_problem(result, plan, bootstrap):
    check, outcome = result["id"], result["outcome"]
    expected = _expected(plan, check)
    if expected is None:
        if outcome in _EXEMPT:
            return f"{check}: must run, reported {outcome}"
        return None
    if outcome != expected:
        return f"{check}: expected {expected}, reported {outcome}"
    if not result.get("reason"):
        return f"{check}: {outcome} without a reason"
    if outcome == NOT_YET_ACTIVE and not bootstrap:
        return f"{check}: NOT_YET_ACTIVE outside bootstrap"
    if check in plan["interim"] and not result.get("evidence"):
        return f"{check}: NOT_YET_ACTIVE without interim evidence"
    return None


def _problem(result, seen, plan, bootstrap):
    check, outcome = result["id"], result.get("outcome")
    if check in seen:
        return f"{check}: duplicate result"
    if check not in checks(plan):
        return f"{check}: not in the profile"
    if outcome not in OUTCOMES:
        return f"{check}: unknown outcome {outcome}"
    return _kind_problem(result, plan, bootstrap)


def problems(plan, results, bootstrap):
    """Reasons why this set of results cannot be trusted as a whole."""
    found, seen = [], set()
    for result in results:
        if not (isinstance(result, dict)
                and isinstance(result.get("id"), str)):
            found.append("malformed result")
            continue
        problem = _problem(result, seen, plan, bootstrap)
        if problem:
            found.append(problem)
        seen.add(result["id"])
    return found + [f"{c}: missing result" for c in checks(plan)
                    if c not in seen]


def overall(plan, results, bootstrap):
    """(status, reasons) for the results of one profile's plan."""
    if not checks(plan):
        return BLOCKED, ["profile has no checks"]
    found = problems(plan, results, bootstrap)
    if found:
        return BLOCKED, found
    seen = {result["outcome"] for result in results}
    if FAILED in seen:
        return FAILED, []
    return (BLOCKED if BLOCKED in seen else PASSED), []
