# id: C02
# title: Container runs as root
# why: A process running as root in a container is one step from root on the computer.
# fix: Set a user that is not root: user: in compose, USER in a Dockerfile.
#
# Task #112: mistakes in setup files.
package nirikshak.c02

import rego.v1

# The user part, before any `:group`, in any case.
root(user) if {
	name := lower(split(user, ":")[0])
	name in {"root", "0"}
}

findings contains {"path": c.path, "line": c.line, "what": c.user} if {
	some c in input.container
	root(c.user)
}

findings contains {"path": d.path, "line": d.line, "what": "no USER, so root"} if {
	some d in input.docker_stage
	d.last
	d.user == ""
}

findings contains {"path": d.path, "line": d.line, "what": d.user} if {
	some d in input.docker_stage
	d.last
	root(d.user)
}
