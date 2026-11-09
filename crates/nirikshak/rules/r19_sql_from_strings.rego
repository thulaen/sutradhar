# id: R19
# title: SQL built from strings
# why: SQL pasted together from text can let outside input change the query.
# fix: Use a prepared query with bound parameters, checked when it is built.
#
# Rule 19 of the approved list: a macro that builds text, such as
# format!, whose text starts a query.
package nirikshak.r19

import rego.v1

builds_text contains "format"

builds_text contains "concat"

builds_text contains "write"

builds_text contains "writeln"

starts_query(t) if {
	some start in ["SELECT ", "INSERT INTO", "UPDATE ", "DELETE FROM"]
	startswith(upper(trim_left(t, " \n\t")), start)
}

findings contains {"path": m.path, "line": m.line, "what": m.text} if {
	some m in input.macro_text
	m.name in builds_text
	starts_query(m.text)
}
