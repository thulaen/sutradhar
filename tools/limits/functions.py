"""Function, method and closure length (SPEC-007: at most 60 lines).

A span runs from the first decorator (or the definition) to the last
line of the body, inclusive; nested functions count inside their parent
and on their own. Both walks are iterative, so deep nesting cannot
exhaust Python's recursion limit. Functions and classes can only be
statements, so a file without lambdas is walked through statement lists
alone, skipping the expression trees that dominate the cost; a file with
lambdas uses the standard library's full walk. Qualified names come from
one sweep over the scopes in source order. Parser resource failures are
reported as parse_limit, which blocks. PHP is refused as unsupported
until a PHP parser is admitted, so it never passes unmeasured.
"""
import ast

from tools.limits import counting

MAX_FUNCTION_LINES = 60
_DEFS = (ast.FunctionDef, ast.AsyncFunctionDef)
_SCOPES = _DEFS + (ast.ClassDef, ast.Lambda)
_BLOCKS = (ast.stmt, ast.excepthandler, ast.match_case)


def _statements(tree):
    """Every statement-level node, descending only through block lists."""
    stack = [tree]
    while stack:
        node = stack.pop()
        yield node
        for _field, value in ast.iter_fields(node):
            if isinstance(value, list):
                stack.extend(v for v in value if isinstance(v, _BLOCKS))


def _scopes(tree, source):
    nodes = ast.walk(tree) if "lambda" in source else _statements(tree)
    found = [n for n in nodes if isinstance(n, _SCOPES)]
    return sorted(found, key=lambda n: (n.lineno, n.col_offset))


def python_spans(source):
    """(qualified name, first line, last line) for every function."""
    out, open_scopes = [], []  # open_scopes: (end position, child prefix)
    for node in _scopes(ast.parse(source), source):
        start = (node.lineno, node.col_offset)
        while open_scopes and open_scopes[-1][0] < start:
            open_scopes.pop()
        prefix = open_scopes[-1][1] if open_scopes else ""
        end = (node.end_lineno, node.end_col_offset)
        if isinstance(node, ast.ClassDef):
            open_scopes.append((end, prefix + node.name + "."))
            continue
        name = prefix + getattr(node, "name", "<lambda>")
        first = min([d.lineno for d in getattr(node, "decorator_list", [])]
                    + [node.lineno])
        out.append((name, first, node.end_lineno))
        open_scopes.append((end, name + "." if isinstance(node, _DEFS)
                            else prefix))
    return out


def check_python(path, source):
    """Violations for Python source, including parse failures."""
    try:
        spans = python_spans(source)
    except SyntaxError as error:
        return [counting.Violation(path, error.lineno, "parse_error",
                                   error.msg)]
    except (RecursionError, MemoryError):
        return [counting.Violation(path, None, "parse_limit",
                                   "parser resource limit")]
    found = []
    for _name, start, end in spans:
        length = end - start + 1
        if length > MAX_FUNCTION_LINES:
            found.append(counting.Violation(path, start, "function_lines",
                                            length))
    return found


def check_source(path, source):
    """Dispatch by file type; non-code files have no function check."""
    if path.endswith(".py"):
        return check_python(path, source)
    if path.endswith(".php"):
        return [counting.Violation(path, None, "unsupported_language",
                                   "no admitted PHP parser")]
    return []
