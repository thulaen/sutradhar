# id: C06
# title: Container can write its own files
# why: A container that can change its own files keeps whatever an attacker writes there.
# fix: Set read_only: true and give it a tmpfs or volume where it must write.
#
# Task #112: mistakes in setup files.
package nirikshak.c06

import rego.v1

findings contains {"path": c.path, "line": c.line, "what": c.name} if {
	some c in input.container
	not c.read_only
}
