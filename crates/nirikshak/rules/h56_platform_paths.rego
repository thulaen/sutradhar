# id: H56
# title: Path for one computer only
# why: A drive letter or a person's home folder exists on one machine only.
# fix: Take the folder from configuration or the platform's own lookup.
package nirikshak.h56

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

fixed(t) if startswith(t, "/Users/")

fixed(t) if {
	contains("ABCDEFGHIJKLMNOPQRSTUVWXYZ", substring(t, 0, 1))
	substring(t, 1, 1) == ":"
	substring(t, 2, 1) in {"/", "\\"}
}

findings contains {"path": l.path, "line": l.line, "what": l.text} if {
	some l in input.literal
	fixed(l.text)
	in_code(l.path, l.function)
}
