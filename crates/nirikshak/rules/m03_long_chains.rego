# id: M03
# title: Very long chain of calls
# why: A chain of many calls in one expression is hard to read and to debug, and hides where a value goes wrong.
# fix: Break it with named steps, or move part of it into a helper with a clear name.
package nirikshak.m03

import rego.v1

findings contains {"path": c.path, "line": c.line, "what": sprintf("%d calls", [c.length])} if {
	some c in input.chain
	c.kind == "code"
	c.length > 8
}
