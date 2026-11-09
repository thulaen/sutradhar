# id: M06
# title: The same text written three times or more
# why: Text copied by hand drifts apart when one copy is changed and the others are not.
# fix: Give it one named const and use the name.
package nirikshak.m06

import rego.v1

findings contains {"path": r.path, "line": r.line, "what": r.text} if {
	some r in input.repeated
	r.kind == "code"
}
