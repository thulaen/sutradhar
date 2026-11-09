# id: H39
# title: Several writes outside a transaction
# why: A failure between two writes leaves half a change in the database.
# fix: Run the writes in one transaction, committed once at the end.
#
# A function with two or more SQL writes, where neither it nor any
# function that calls it (matched by name) opens a transaction.
package nirikshak.h39

import rego.v1

write_sql(t) if startswith(upper(t), "INSERT INTO")

write_sql(t) if startswith(upper(t), "DELETE FROM")

write_sql(t) if {
	startswith(upper(t), "UPDATE ")
	contains(upper(t), " SET ")
}

writes(path, name) := {l.line |
	some l in input.literal
	l.path == path
	l.function == name
	write_sql(l.text)
}

opens_transaction contains c.function if {
	some c in input.call
	c.callee in {".begin", ".transaction", ".build_transaction", ".commit"}
}

last_part(callee) := name if {
	parts := split(trim_prefix(callee, "."), "::")
	name := parts[count(parts) - 1]
}

callers[name] := from if {
	some f in input.function
	name := f.name
	from := {c.function | some c in input.call; last_part(c.callee) == name}
}

in_code(path, name) if {
	some f in input.function
	f.path == path
	f.name == name
	f.kind == "code"
}

findings contains {"path": f.path, "line": f.first, "what": f.name} if {
	some f in input.function
	count(writes(f.path, f.name)) > 1
	in_code(f.path, f.name)
	count(graph.reachable(callers, {f.name}) & opens_transaction) == 0
}
