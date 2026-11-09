# id: C09
# title: Secret written into a setting
# why: A password or token written in a setup file is in every copy of the repository.
# fix: Read it from a private file or a ${NAME} setting kept out of git; a ${NAME:-x} default writes x in the file.
#
# Task #112: mistakes in setup files.
package nirikshak.c09

import rego.v1

secret_name(n) if {
	some word in ["PASSWORD", "SECRET", "TOKEN", "API_KEY", "PRIVATE_KEY"]
	contains(upper(n), word)
}

findings contains {"path": e.path, "line": e.line, "what": e.name} if {
	some e in input.container_env
	secret_name(e.name)
	e.written
}
