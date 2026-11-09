# id: M05
# title: Code left in a comment
# why: Code kept in a comment is never built or tested, goes stale, and confuses readers; git keeps the history.
# fix: Delete it. If it is needed later, it is in git.
package nirikshak.m05

import rego.v1

ends := [";", "{", "}"]

marks := ["(", "=", "::", "->", "!", "[", "&"]

code_like(t) if {
	some e in ends
	endswith(t, e)
	some m in marks
	contains(t, m)
}

findings contains {"path": c.path, "line": c.line, "what": c.text} if {
	some c in input.comment
	c.kind == "code"
	code_like(c.text)
}
