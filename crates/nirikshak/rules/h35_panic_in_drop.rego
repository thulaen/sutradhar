# id: H35
# title: Panic inside drop
# why: A panic while dropping during another panic aborts the whole program.
# fix: Report the failure (log it) and carry on; never unwrap in drop.
package nirikshak.h35

import rego.v1

stops(path, name) := {c.line | some c in input.call; c.path == path; c.function == name; c.callee in {".unwrap", ".expect"}} | {m.line | some m in input.macro; m.path == path; m.function == name; panics(m.name)}

panics(n) if n in {"panic", "unreachable", "todo", "unimplemented"}

panics(n) if startswith(n, "assert")

in_code(path, name) if {
	some f in input.function
	f.path == path
	f.name == name
	f.kind == "code"
}

findings contains {"path": d.path, "line": line, "what": d.function} if {
	some d in input.impl_fn
	d.trait == "Drop"
	in_code(d.path, d.function)
	some f in input.function
	f.path == d.path
	f.first == d.line
	some line in stops(d.path, d.function)
	line >= f.first
	line <= f.last
}
