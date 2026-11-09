# id: H22
# title: Destructive SQL in product code
# why: Dropping, emptying or deleting every row cannot be undone by the code.
# fix: Use a reviewed migration that keeps the data, or add a WHERE clause.
package nirikshak.h22

import rego.v1

in_code(path, name) if {
	some f in input.function
	f.path == path
	f.name == name
	f.kind == "code"
}

in_code(path, "<top>") if {
	some f in input.file
	f.path == path
	f.language == "rust"
	not contains(path, "/tests/")
	not contains(path, "/benches/")
	not endswith(path, "_tests.rs")
}

destroys(t) if {
	some shape in ["DROP TABLE", "DROP COLUMN", "DROP SCHEMA", "DROP DATABASE", "TRUNCATE "]
	contains(upper(t), shape)
}

destroys(t) if {
	startswith(upper(t), "DELETE FROM")
	not contains(upper(t), "WHERE")
}

findings contains {"path": l.path, "line": l.line, "what": l.text} if {
	some l in input.literal
	destroys(l.text)
	in_code(l.path, l.function)
}
