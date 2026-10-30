"""Running a profile for one candidate and signing the evidence.

Order: load the protected policy and the private host file, check the
runner against its pin, check every folder above the output folder is
owned by root or the runner and not writable by others (sticky folders
owned by root excepted),
resolve the tree and base, choose the profile (documentation-only is
refused for code or for no change at all), check each pinned tool, then
create the run folder, copy the tools into it and check the copies,
export the candidate, run each check in its own sandbox and re-check the
export and tool copies. Only then is a canonical bundle written and
signed. Interim evidence and review records named in the evidence file
are hashed into the bundle; NOT_YET_ACTIVE counts only when every
changed path is inside the policy's bootstrap scope.
"""
import collections
import hashlib
import os
import platform
import shutil
import stat
import subprocess

from tools.runner import (bundle, candidate, execute, inputs, outcomes,
                          policy, reports, sandbox, signing)

ENV = {"LANG": "C.UTF-8", "HOME": "/tmp", "TMPDIR": "/tmp",
       "PATH": "/usr/bin:/bin", "PYTHONDONTWRITEBYTECODE": "1"}
Blocked, read = inputs.Blocked, inputs.read
_BINARIES = ("git", "python3", "sh", "ssh-keygen")
_HERE = os.path.dirname(os.path.abspath(__file__))
Job = collections.namedtuple("Job", "loaded paths binds out prefix run token")
Plan = collections.namedtuple(
    "Plan", "raw loaded prefix installed interim reviews tree base changed "
    "executable profile")


def runner_files():
    """(name, path) for each runner source file, tests excluded."""
    found = [("tools/__init__.py",
              os.path.join(os.path.dirname(_HERE), "__init__.py"))]
    return found + [(f"tools/runner/{name}", os.path.join(_HERE, name))
                    for name in sorted(os.listdir(_HERE))
                    if name.endswith(".py")]


def runner_digest():
    """The digest that the policy's runner pin must equal."""
    return candidate.digest([
        {"path": name, "executable": False,
         "sha256": candidate.file_sha256(path)}
        for name, path in runner_files()])


def _safe_folder(path):
    info = os.lstat(path)
    if stat.S_ISLNK(info.st_mode) or info.st_uid not in (0, os.getuid()):
        return False
    shared = info.st_mode & 0o022
    return not shared or (info.st_mode & stat.S_ISVTX and info.st_uid == 0)


def _safe_out(out):
    """Refuse an output folder that another account could swap."""
    try:
        sandbox.safe_path(out)
    except ValueError:
        raise Blocked("unsafe output folder") from None
    folder = os.path.dirname(out)
    while True:
        if not _safe_folder(folder):
            raise Blocked("unsafe output folder")
        if folder == "/":
            return
        folder = os.path.dirname(folder)


def _check_tools(loaded, installed):
    for name, pin in sorted(loaded["tools"].items()):
        if name not in installed:
            raise Blocked(f"tool not installed: {name}")
        try:
            found = candidate.path_digest(installed[name])
        except candidate.CandidateError:
            raise Blocked(f"tool not installed: {name}") from None
        if found != pin:
            raise Blocked(f"tool changed: {name}")


def _copy_tools(loaded, installed, folder):
    """Copy each tool into the run folder; bind the checked copy."""
    os.mkdir(folder)
    binds = []
    for name, pin in sorted(loaded["tools"].items()):
        copy = os.path.join(folder, name)
        if os.path.isdir(installed[name]):
            # Links are copied as links, so the digest below refuses them.
            shutil.copytree(installed[name], copy, symlinks=True)
        else:
            shutil.copy2(installed[name], copy)
        if candidate.path_digest(copy) != pin:
            raise Blocked(f"tool changed: {name}")
        binds.append((copy, f"{sandbox.TOOLS}/{name}"))
    return binds


def prepare(opts):
    """Everything decided before anything is written."""
    raw = read(opts["policy"])
    loaded = policy.load(raw)
    if runner_digest() != loaded["runner"]:
        raise Blocked("runner differs from its pin")
    prefix, installed = inputs.host(opts["host"])
    _safe_out(opts["out"])
    repo = opts["repo"]
    tree = candidate.resolve_tree(repo, opts["tree"])
    interim, reviews = inputs.evidence(opts.get("evidence"), tree)
    base = None
    marked = candidate.executables(repo, tree)
    if "base" in opts:
        base = candidate.resolve_tree(repo, opts["base"])
        marked |= candidate.executables(repo, base)
    changed = candidate.changed(repo, base, tree)
    executable = [path for path in changed if path in marked]
    policy.select(loaded, opts["profile"], changed, executable)
    _check_tools(loaded, installed)
    return Plan(raw, loaded, prefix, installed, interim, reviews, tree, base,
                changed, executable, opts["profile"])


def _binary(name):
    found = shutil.which(name, path=ENV["PATH"])
    return candidate.file_sha256(os.path.realpath(found)) if found else None


def environment():
    """The host facts and the programs the checks rely on."""
    return {"python": platform.python_version(),
            "system": platform.system(), "machine": platform.machine(),
            "binaries": {name: _binary(name) for name in _BINARIES}}


def _check(job, check_id):
    unit, nonce = f"sutradhar-{check_id}-{job.token()}", job.token()
    binds = [(os.path.join(job.out, "candidate"), sandbox.CANDIDATE),
             *job.binds]
    argv = sandbox.command(
        job.prefix, unit, job.loaded["limits"], binds,
        policy.cwd(job.loaded, check_id), ENV,
        policy.argv(job.loaded, check_id, job.paths))
    judge, stdin_data = reports.exit_judge, None
    if job.loaded["checks"][check_id]["report"] == "unittest":
        judge = reports.unittest_judge(nonce)
        stdin_data = f"{nonce}\n".encode()

    def stop():
        try:
            job.run([*job.prefix, "systemctl", "stop", unit],
                    capture_output=True, timeout=60)
        except (OSError, subprocess.SubprocessError):
            pass
    return execute.run_check(
        check_id, argv, os.path.join(job.out, "private"),
        job.loaded["limits"]["seconds"], run=job.run,
        decode=sandbox.check_status, stop=stop, judge=judge,
        stdin_data=stdin_data)


def _results(job, profile, interim):
    results = [_check(job, check_id) for check_id in profile["run"]]
    results += [{"id": check_id, "outcome": outcomes.NOT_APPLICABLE,
                 "reason": reason}
                for check_id, reason in profile["not_applicable"].items()]
    for check_id, reason in profile["not_yet_active"].items():
        entry = {"id": check_id, "outcome": outcomes.NOT_YET_ACTIVE,
                 "reason": reason}
        if check_id in interim:
            entry["evidence"] = interim[check_id]
        results.append(entry)
    return results


def _recheck(loaded, out, manifest):
    exported = candidate.path_digest(os.path.join(out, "candidate"))
    if exported != candidate.digest(manifest):
        raise Blocked("candidate changed during checks")
    for name, pin in sorted(loaded["tools"].items()):
        if candidate.path_digest(os.path.join(out, "tools", name)) != pin:
            raise Blocked(f"tool changed during checks: {name}")


def _write(path, data):
    with open(path, "xb") as handle:
        handle.write(data)


def run_candidate(opts, run, now, token):
    """Run the profile and return the signed bundle's document."""
    plan, out = prepare(opts), opts["out"]
    started = bundle.timestamp(now())
    for folder, mode in ((out, 0o750), (os.path.join(out, "private"), 0o700)):
        os.mkdir(folder)
        os.chmod(folder, mode)
    binds = _copy_tools(plan.loaded, plan.installed,
                        os.path.join(out, "tools"))
    manifest = candidate.export(opts["repo"], plan.tree,
                                os.path.join(out, "candidate"))
    job = Job(plan.loaded, [entry["path"] for entry in manifest], binds, out,
              plan.prefix, run, token)
    profile = plan.loaded["profiles"][plan.profile]
    results = _results(job, profile, plan.interim)
    _recheck(plan.loaded, out, manifest)
    facts = {"tree": plan.tree, "base": plan.base,
             "manifest_sha256": candidate.digest(manifest),
             "profile": plan.profile, "changed": plan.changed,
             "changed_executable": plan.executable,
             "policy_version": plan.loaded["version"],
             "policy_sha256": hashlib.sha256(plan.raw).hexdigest(),
             "runner_sha256": plan.loaded["runner"],
             "environment": environment(), "tools": plan.loaded["tools"],
             "reviews": plan.reviews, "started": started,
             "finished": bundle.timestamp(now())}
    doc = bundle.build(facts, policy.plan(plan.loaded, plan.profile),
                       results, policy.in_scope(plan.loaded, plan.changed))
    data = bundle.canonical(doc)
    signature = signing.sign(opts["key"], data)
    _write(os.path.join(out, "bundle.json"), data)
    _write(os.path.join(out, "bundle.json.sig"), signature)
    return doc
