# id: M02
# title: Number with no name
# why: A bare number such as 86400 in code does not say what it means, and the same value written twice drifts apart.
# fix: Give it a named const that says what it is, and use the name.
package nirikshak.m02

import rego.v1

plain := {"0", "1", "2"}

findings contains {"path": n.path, "line": n.line, "what": n.text} if {
	some n in input.number
	n.kind == "code"
	not n.text in plain
}
