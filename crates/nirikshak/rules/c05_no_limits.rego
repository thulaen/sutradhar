# id: C05
# title: Container with no memory or process limit
# why: With no limit one container can take all the memory or processes and stop everything else.
# fix: Set mem_limit and pids_limit.
#
# Task #112: mistakes in setup files.
package nirikshak.c05

import rego.v1

findings contains {"path": c.path, "line": c.line, "what": c.name} if {
	some c in input.container
	not c.mem_limit
}

findings contains {"path": c.path, "line": c.line, "what": c.name} if {
	some c in input.container
	not c.pids_limit
}
