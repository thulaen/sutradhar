"""Function, method and closure length (SPEC-007, AT-T06)."""
import unittest

from tools.limits import functions


def body(n):
    return "".join(f"    x{i} = {i}\n" for i in range(n))


class PythonSpanTest(unittest.TestCase):
    def test_span_includes_decorators(self):
        src = "@a\n@b\ndef f():\n    return 1\n"
        self.assertEqual(functions.python_spans(src), [("f", 1, 4)])

    def test_methods_and_nested_functions_are_listed(self):
        src = ("class C:\n    def m(self):\n        def inner():\n"
               "            pass\n        return inner\n")
        names = [s[0] for s in functions.python_spans(src)]
        self.assertEqual(names, ["C.m", "C.m.inner"])

    def test_lambda_is_a_closure(self):
        src = "f = (lambda x:\n     x + 1)\n"
        self.assertEqual(functions.python_spans(src),
                         [("<lambda>", 1, 2)])

    def test_async_function(self):
        src = "async def g():\n    pass\n"
        self.assertEqual(functions.python_spans(src), [("g", 1, 2)])


class PythonLimitTest(unittest.TestCase):
    def test_60_lines_pass(self):
        src = "def f():\n" + body(59)
        self.assertEqual(functions.check_python("a.py", src), [])

    def test_61_lines_with_decorator_fail(self):
        src = "@d\n@e\ndef f():\n" + body(58)
        v = functions.check_python("a.py", src)
        self.assertEqual([(x.kind, x.line, x.value) for x in v],
                         [("function_lines", 1, 61)])

    def test_blank_and_comment_lines_count(self):
        src = "def f():\n" + "    # c\n\n" * 30 + "    pass\n"
        self.assertEqual(len(functions.check_python("a.py", src)), 1)

    def test_syntax_error_is_reported_not_skipped(self):
        v = functions.check_python("a.py", "def (:\n")
        self.assertEqual([x.kind for x in v], ["parse_error"])


class OtherLanguageTest(unittest.TestCase):
    def test_php_is_blocked_until_a_parser_is_admitted(self):
        v = functions.check_source("a.php", "<?php function f() {}\n")
        self.assertEqual([x.kind for x in v], ["unsupported_language"])

    def test_non_code_files_have_no_function_check(self):
        self.assertEqual(functions.check_source("a.md", "# t\n"), [])

    def test_python_files_are_dispatched(self):
        src = "def f():\n" + body(61)
        kinds = [x.kind for x in functions.check_source("t.py", src)]
        self.assertEqual(kinds, ["function_lines"])


if __name__ == "__main__":
    unittest.main()
