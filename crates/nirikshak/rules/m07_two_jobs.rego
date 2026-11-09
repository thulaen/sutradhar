# id: M07
# title: Function that does two jobs
# why: A name such as read_and_send says the function does two things, so it is harder to test and to reuse.
# fix: Split it into two functions, one for each job, and call both.
package nirikshak.m07

import rego.v1

findings contains {"path": s.path, "line": s.line, "what": s.function} if {
	some s in input.signature
	s.kind == "code"
	contains(s.function, "_and_")
}
