# id: H23
# title: Atomic value read, then set apart
# why: Another thread can change the value between the read and the write.
# fix: Use one atomic step: compare_exchange, fetch_update or swap.
package nirikshak.h23

import rego.v1

in_code(path, name) if {
	some f in input.function
	f.path == path
	f.name == name
	f.kind == "code"
}

findings contains {"path": s.path, "line": s.line, "what": s.receiver} if {
	some s in input.method_receiver
	s.method == ".store"
	some l in input.method_receiver
	l.method == ".load"
	l.path == s.path
	l.function == s.function
	l.receiver == s.receiver
	in_code(s.path, s.function)
}
