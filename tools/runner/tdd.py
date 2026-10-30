"""Test-first evidence rules (SPEC-007 part 2: AT-T01, AT-T02, AT-G19).

A commit message declares exactly one of `TDD: test-first`, `TDD:
retrofit` or `TDD: not-applicable (REASON)`, REASON being documentation,
refactor, dependency, secret or configuration; a missing, repeated or
unknown declaration is None, which fails. Only test files that differ
between the parent and the candidate are parsed, never run. Each gives
keys: the module (its own statements and imports), each class (its
header and everything but its test methods, so set-up and helpers are
fixtures) and each test method, by qualified name. A key whose code
fingerprint is new or different is changed; layout and comments never
count. A test counts as new or changed when its module, any class it
inherits from or its own method is changed. For test-first such a test
must fail on the parent by assertion and pass on the candidate; an error
or absence on the parent counts only for tests a reviewed new-unit
record names. The modules to run are the changed test modules and the
test modules naming them (so inherited tests run), or every module when
a changed key lies in a helper file. A refactor may move test
methods between classes and files; every other statement, class header
and fixture (with its class) must stay the same, each part carries the
imports it uses, and no import may be added. A file that does not parse,
or is too large, is compared by its bytes and holds no test.
Fingerprints are built without recursion, so deeply nested code cannot
break the run.
"""
import ast
import hashlib
import posixpath
import re

from tools.runner import policy

LIMIT = 1024 * 1024  # bytes: the largest test file the runner parses
REASONS = ("documentation", "refactor", "dependency", "secret",
           "configuration")
_LINE = re.compile(r"TDD: (test-first|retrofit|not-applicable \(([a-z]+)\))")
_FUNCTIONS = (ast.FunctionDef, ast.AsyncFunctionDef)
_IMPORTS = (ast.Import, ast.ImportFrom)


class _Mark:
    def __init__(self, text):
        self.text = text.encode()


_CLOSE, _END = _Mark(")"), _Mark("]")


def declaration(message):
    """The declared kind, or None unless exactly one valid line exists."""
    if message is None:
        return None
    try:
        lines = message.decode().split("\n")
    except UnicodeDecodeError:
        return None
    found = [line.rstrip() for line in lines if line.startswith("TDD:")]
    match = _LINE.fullmatch(found[0]) if len(found) == 1 else None
    if match is None or match.group(2) not in (None, *REASONS):
        return None
    return match.group(1)


def reason(found):
    """The not-applicable reason of a declaration, or None."""
    return _LINE.fullmatch(f"TDD: {found}").group(2)


def _digest(node):
    """SHA-256 of a syntax tree (or list of them), walked iteratively."""
    digest, stack = hashlib.sha256(), [node]
    while stack:
        item = stack.pop()
        if isinstance(item, _Mark):
            digest.update(item.text)
        elif isinstance(item, ast.AST):
            digest.update(type(item).__name__.encode() + b"(")
            stack.append(_CLOSE)
            for field in reversed(item._fields):
                stack += [getattr(item, field), _Mark(f"{field}=")]
        elif isinstance(item, list):
            digest.update(b"[")
            stack.append(_END)
            stack.extend(reversed(item))
        else:  # hex, as repr() refuses integers over 4,300 digits
            text = hex(item) if type(item) is int else repr(item)
            digest.update(text.encode() + b",")
    return digest.hexdigest()


def _tree(data):
    if len(data) > LIMIT:
        return None
    try:
        return ast.parse(data)
    except (SyntaxError, ValueError, RecursionError, MemoryError):
        return None


def _docstring(node):
    return (isinstance(node, ast.Expr) and isinstance(node.value, ast.Constant)
            and isinstance(node.value.value, str))


def _class_keys(prefix, node, found):
    name = f"{prefix}.{node.name}"
    rest = [node.bases, node.keywords, node.decorator_list]
    for item in node.body:
        if isinstance(item, ast.ClassDef):
            _class_keys(name, item, found)
        elif isinstance(item, _FUNCTIONS) and item.name.startswith("test"):
            found[f"{name}.{item.name}"] = _digest(item)
        elif not _docstring(item):
            rest.append(item)
    found[name] = _digest(rest)


def _keys(path, data):
    """{key: fingerprint} for one test file; {} when it does not parse."""
    tree = _tree(data) if path.endswith(".py") else None
    if tree is None:
        return {}
    module, found, rest = path[:-len(".py")].replace("/", "."), {}, []
    for node in tree.body:
        if isinstance(node, ast.ClassDef):
            _class_keys(module, node, found)
        elif not _docstring(node):
            rest.append(node)
    found[module] = _digest(rest)
    return found


def _differing(before, after):
    return sorted(p for p in set(before) | set(after)
                  if before.get(p) != after.get(p))


def _test_module(path):
    name = posixpath.basename(path)
    return name.startswith("test") or name.endswith("_test.py")


def _users(modules, after):
    """Test modules in after whose text names one of modules, so that
    tests inheriting a changed test module's classes also run."""
    names = [re.escape(name.split(".")[-1].encode()) for name in modules]
    pattern = re.compile(rb"\b(?:" + b"|".join(names) + rb")\b")
    return {path[:-len(".py")].replace("/", ".") for path, data in
            after.items() if _test_module(path) and path.endswith(".py")
            and pattern.search(data)}


def changed_keys(before, after):
    """(changed keys, modules to run), both sorted.

    before and after map test file paths to bytes. The modules are the
    changed test modules and those naming them, or none (meaning every
    module) when a changed key lies in a helper file.
    """
    changed, modules, every = set(), set(), False
    for path in _differing(before, after):
        old = _keys(path, before[path]) if path in before else {}
        new = _keys(path, after[path]) if path in after else {}
        found = {key for key, value in new.items() if old.get(key) != value}
        changed |= found
        if _test_module(path):
            if found:
                modules.add(path[:-len(".py")].replace("/", "."))
        elif found:
            every = True  # tests in other modules may inherit it
    if every or not modules:
        return sorted(changed), []
    return sorted(changed), sorted(modules | _users(modules, after))


def _bindings(tree):
    """{name: fingerprint of the module-level import that binds it}."""
    found = {}
    for node in tree.body:
        if isinstance(node, _IMPORTS):
            for alias in node.names:
                name = (alias.asname or alias.name).split(".")[0]
                found[name] = _digest(node)
    return found


def _used(nodes, bound):
    """(name, import) pairs for imported names that nodes use."""
    names = {item.id for node in nodes for item in ast.walk(node)
             if isinstance(item, ast.Name)}
    return sorted((name, bound[name]) for name in names & set(bound))


def _parts(path, data, parts, imports):
    tree = _tree(data) if path.endswith(".py") else None
    if tree is None:
        parts.append(hashlib.sha256(data).hexdigest())
        return
    bound, nodes = _bindings(tree), [((), node) for node in tree.body]
    while nodes:
        owner, node = nodes.pop()
        if isinstance(node, ast.ClassDef):
            name, header = (*owner, node.name), [
                *node.bases, *node.keywords, *node.decorator_list]
            parts.append(_digest([name, header, _used(header, bound)]))
            nodes += [(name, item) for item in node.body]
        elif isinstance(node, _IMPORTS):
            imports.add(_digest(node))
        elif not _docstring(node):
            test = isinstance(node, _FUNCTIONS) and node.name.startswith(
                "test")
            parts.append(_digest([None if test else owner, node,
                                  _used([node], bound)]))


def _content(files, paths):
    parts, imports = [], set()
    for path in paths:
        if path in files:
            _parts(path, files[path], parts, imports)
    return sorted(parts), imports


def tests_edited(before, after):
    """Whether any test code, test data or fixture changed (AT-T02).

    Only differing files are compared: every statement, class header and
    data file counts, wherever it sits, so tests may move between files
    and classes and an import may move, but nothing else may change.
    """
    paths = _differing(before, after)
    return _content(before, paths) != _content(after, paths)


def qualifying(changed, red, green, noted=()):
    """Changed tests that fail on the parent and pass on the candidate.

    red and green are harness details: ids that ran, failed and errored
    ids and, for green, each id's keys; green is known to be clean.
    """
    changed, noted, ran = set(changed), set(noted), set(red["ids"])
    failed = set(red["failed_ids"]) & ran
    passed = ran - failed - set(red["errored_ids"])
    return sorted(name for name in green["ids"]
                  if changed & set(green["keys"].get(name, ()))
                  and (name in failed
                       or (name in noted and name not in passed)))


def retrofit_problem(changed, executable, loaded):
    """Why a retrofit changes more than tests and documentation, or None."""
    marked = set(executable)
    for path in changed:
        test = policy.matches(loaded["test_first"]["tests"], path)
        document = (path not in marked
                    and policy.matches(loaded["documentation"], path))
        if not (test or document):
            return f"retrofit changes code: {path}"
    return None


def paths_problem(reason, changed, executable, loaded):
    """Why the change does not fit a not-applicable reason, or None."""
    if not changed:
        return "no change"
    if reason == "documentation":
        patterns, marked = loaded["documentation"], set(executable)
    else:
        patterns, marked = loaded["test_first"]["paths"][reason], set()
    for path in changed:
        if path in marked or not policy.matches(patterns, path):
            return f"not a {reason} change: {path}"
    return None
