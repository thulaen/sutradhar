# id: M01
# title: Function with a true-or-false input
# why: A bool input means the function does two things, and a call such as f(true) does not say which.
# fix: Split the function in two, or take a small enum that names each choice.
package nirikshak.m01

import rego.v1

findings contains {"path": s.path, "line": s.line, "what": s.function} if {
	some s in input.signature
	s.kind == "code"
	s.bools > 0
}
