# id: R08
# title: Network call with no time limit
# why: A call that can wait for ever stalls its job and every job queued behind it.
# fix: Use a call with a time limit, such as TcpStream::connect_timeout.
#
# Rule 8 of the approved list. Calls are named through the file's imports.
# Test code is left out.
package nirikshak.r08

import rego.v1

waits contains "std::net::TcpStream::connect"

waits contains "tokio::net::TcpStream::connect"

waits contains "reqwest::get"

waits contains "reqwest::blocking::get"

in_code(path, name) if {
	some f in input.function
	f.path == path
	f.name == name
	f.kind == "code"
}

findings contains {"path": c.path, "line": c.line, "what": c.callee} if {
	some c in input.call
	c.callee in waits
	in_code(c.path, c.function)
}
