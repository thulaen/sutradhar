# id: R09
# title: Error thrown away
# why: A result thrown away hides a failure from the people who run Sutradhar.
# fix: Handle the error, return it to the caller, or log it with context.
#
# Rule 9 of the approved list. Test code is left to rule 14.
package nirikshak.r09

import rego.v1

in_code(path, name) if {
	some f in input.function
	f.path == path
	f.name == name
	f.kind == "code"
}

findings contains {"path": d.path, "line": d.line, "what": d.kind} if {
	some d in input.discard
	in_code(d.path, d.function)
}

findings contains {"path": h.path, "line": h.line, "what": "Err(_) => {}"} if {
	some h in input.empty_handler
	in_code(h.path, h.function)
}
