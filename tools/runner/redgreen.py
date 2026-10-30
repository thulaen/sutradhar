"""The test-first check: red on the parent, green on the candidate.

SPEC-007 part 2. The declaration is read from the commit message given to
the runner (see tools.runner.tdd). Documentation, dependency, secret
and configuration declarations are judged by the changed paths alone. A
refactor must leave every test unedited; a retrofit may change only tests
and documentation and must add or change a test (its proof is coverage or
mutation, separate checks); the unit-tests check proves both green. For
test-first the runner builds the red tree itself, the parent's files with
the candidate's test files, and the pinned harness runs the modules
holding changed tests on the candidate and then on the red tree, each in
its own sandbox with a fresh value proving the summary belongs to that
run (AT-T01). A missing or contradicted declaration fails (AT-G19); a run
that decides nothing blocks. A change that makes the parent hang shows as
a blocked red run, not a failure.
"""
import functools
import os

from tools.runner import (candidate, harness, outcomes, policy, reports,
                          tdd)

PASSED, FAILED, BLOCKED = outcomes.PASSED, outcomes.FAILED, outcomes.BLOCKED


class _Undecided(Exception):
    """A harness run decided nothing, so the check blocks."""


def _detail_judge(nonce, store):
    """Keep the run's detailed summary; block when it cannot be trusted."""
    def judge(path, outcome, reason):
        if outcome == BLOCKED:
            return outcome, reason
        found = reports.detail(path)
        if found is None:
            return BLOCKED, "no test report"
        if found.get("nonce") != nonce:
            return BLOCKED, "test report not for this run"
        if harness.clean(found) != (outcome == PASSED):
            return BLOCKED, "exit status and test report disagree"
        store.append(found)
        return outcome, reason
    return judge


def _harness(job, launch, record, kind, folder, modules):
    """Run the harness on folder's chosen modules; return its detail."""
    nonce, store = job.token(), []
    stdin = "".join(f"{line}\n" for line in [nonce, *modules]).encode()
    run = launch(f"test-first-{kind}", folder, _detail_judge(nonce, store),
                 stdin)
    record["runs"].append(run)
    if not store:
        raise _Undecided(f"{kind} run: {run['reason']}")
    return store[0]


def _red_green(job, plan, launch, record, changed):
    keys, modules, is_test = changed
    record["modules"] = modules
    green = _harness(job, launch, record, "green",
                     os.path.join(job.out, "candidate"), modules)
    if green["collected"] == 0:
        raise _Undecided("green run: no tests ran")
    if not harness.clean(green):
        return FAILED, "tests fail on the candidate"
    folder = os.path.join(job.out, "red")
    manifest = candidate.export_mixed(plan.repo, plan.base, plan.tree,
                                      is_test, folder)
    record["red_manifest_sha256"] = candidate.digest(manifest)
    red = _harness(job, launch, record, "red", folder, modules)
    noted = [name for unit in plan.new_units for name in unit["tests"]]
    record["qualifying"] = tdd.qualifying(keys, red, green, noted)
    if not record["qualifying"]:
        return FAILED, "no new or changed test fails on the parent by " \
            "assertion"
    return PASSED, ""


def _decide(job, plan, launch, record):
    found = record["declaration"]
    if found is None:
        return FAILED, "no valid TDD declaration"
    reason = tdd.reason(found)
    if reason not in (None, "refactor"):
        problem = tdd.paths_problem(reason, plan.changed,
                                          plan.executable, job.loaded)
        return (FAILED, problem) if problem else (PASSED, "")
    problem = found == "retrofit" and tdd.retrofit_problem(
        plan.changed, plan.executable, job.loaded)
    if problem:
        return FAILED, problem
    is_test = functools.partial(policy.matches,
                                job.loaded["test_first"]["tests"])
    before = candidate.contents(plan.repo, plan.base, is_test)
    after = candidate.contents(plan.repo, plan.tree, is_test)
    if reason == "refactor":
        edited = tdd.tests_edited(before, after)
        return (FAILED, "refactor edits tests") if edited else (PASSED, "")
    keys, modules = tdd.changed_keys(before, after)
    if not keys:
        return FAILED, f"{found}: no new or changed test"
    if found == "retrofit":
        return PASSED, ""
    return _red_green(job, plan, launch, record, (keys, modules, is_test))


def check(job, plan, check_id, launch):
    """The test-first check's result record.

    launch(name, folder, judge, stdin_data) runs the check's command on
    folder in a fresh sandbox and returns that run's record.
    """
    record = {"id": check_id, "runs": [],
              "declaration": tdd.declaration(plan.message)}
    try:
        outcome, reason = _decide(job, plan, launch, record)
    except (_Undecided, candidate.CandidateError) as error:
        outcome, reason = BLOCKED, str(error)
    record.update(outcome=outcome, reason=reason)
    return record
